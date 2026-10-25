// Content stamp for the desktop's WebAssembly engines (DEL-07-11 native smoke).
//
// `build-wasm-engine.mjs` writes `source-stamp.json` into each engine's output
// folder after a successful build; `native-smoke/run.mjs` recomputes the stamp
// and rebuilds when it differs, so a stale engine is never reused. The stamp is
// a SHA-256 over, sorted by project-relative path, each input's path and bytes:
//
//   - every crate in the engine's closure (the engine crate and its normal and
//     build path dependencies, recursively, as `cargo metadata --no-deps`
//     reports them; dev dependencies are not compiled in): `Cargo.toml`,
//     `build.rs` or the declared build script, and every file under `src/`;
//   - the engine crate's lock file (its workspace root's `Cargo.lock`, and the
//     workspace `Cargo.toml` when that is a different directory);
//   - this module and the build script, which fix the cargo flags, features,
//     target and the pinned wasm-bindgen version;
//   - the compile reads the last build's cargo dep-info listed outside those
//     files (`include_*!` targets, `#[path]` modules and build-script
//     `rerun-if-changed` files such as the schemas and fixtures that
//     product_physics' build script digests). They are recorded in the stamp,
//     because only a build can list them; a change that adds one edits a file
//     already hashed.
//
// and a context: `rustc -vV`, the rustflag and release-profile environment, and
// the cargo configuration files cargo would read. Build output (`target/`,
// `node_modules/`, `.git/`) is never an input.
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, readdirSync, readFileSync, realpathSync, statSync } from "node:fs";
import os from "node:os";
import path from "node:path";

export const STAMP_FORMAT = "openpipestress.engine-source-stamp.v1";
export const STAMP_FILENAME = "source-stamp.json";
const EXCLUDED_NAMES = new Set(["target", "node_modules", ".git", ".DS_Store"]);
const SCRIPT_INPUTS = ["apps/desktop/scripts/build-wasm-engine.mjs", "apps/desktop/scripts/engineSourceStamp.mjs"];
const CONTEXT_ENV = /^(RUSTC|RUSTFLAGS|CARGO_ENCODED_RUSTFLAGS|CARGO_BUILD_RUSTFLAGS|CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS|CARGO_PROFILE_RELEASE_.+)$/;

const toPosix = (value) => value.split(path.sep).join("/");
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
const real = (value) => (existsSync(value) ? realpathSync(value) : path.resolve(value));

function isSafeRelative(value) {
  return typeof value === "string" && value.length > 0 && !path.isAbsolute(value) && !value.split("/").some((part) => part === ".." || part === "" || part === ".");
}

// Project-relative POSIX path of an absolute path, or null outside the project.
function projectRelative(projectRoot, absolute) {
  const relative = path.relative(real(projectRoot), real(absolute));
  if (relative === "" || relative.startsWith("..") || path.isAbsolute(relative)) return null;
  return toPosix(relative);
}

function excluded(relative) {
  return relative.split("/").some((part) => EXCLUDED_NAMES.has(part));
}

// Every file under `relativeDir` (project-relative POSIX paths), build output excluded.
export function listTreeFiles(projectRoot, relativeDir) {
  const out = [];
  const visit = (absolute, relative) => {
    for (const entry of readdirSync(absolute, { withFileTypes: true })) {
      if (EXCLUDED_NAMES.has(entry.name)) continue;
      const childAbsolute = path.join(absolute, entry.name);
      const childRelative = `${relative}/${entry.name}`;
      const stat = entry.isSymbolicLink() ? statSync(childAbsolute, { throwIfNoEntry: false }) : entry;
      if (!stat) continue;
      if (stat.isDirectory()) visit(childAbsolute, childRelative);
      else if (stat.isFile()) out.push(childRelative);
    }
  };
  const root = path.join(projectRoot, relativeDir);
  if (existsSync(root) && statSync(root).isDirectory()) visit(root, relativeDir);
  return out.sort();
}

// The deterministic hash of `relativePaths` (deduplicated, sorted) and `context`
// (an array of [name, value] pairs). A missing input hashes as missing.
export function hashInputs(projectRoot, relativePaths, context = []) {
  const inputs = [...new Set(relativePaths)].sort();
  for (const relative of inputs) {
    if (!isSafeRelative(relative)) throw new Error(`engine stamp: input path is not project-relative: ${relative}`);
  }
  const hash = createHash("sha256");
  hash.update(`${STAMP_FORMAT}\n`);
  for (const [name, value] of context) hash.update(`context ${JSON.stringify(name)} ${JSON.stringify(value)}\n`);
  for (const relative of inputs) {
    let bytes = null;
    try {
      bytes = readFileSync(path.join(projectRoot, relative));
    } catch {
      bytes = null;
    }
    hash.update(bytes === null ? `input ${JSON.stringify(relative)} missing\n` : `input ${JSON.stringify(relative)} ${bytes.length} ${sha256(bytes)}\n`);
  }
  return { hash: `sha256:${hash.digest("hex")}`, input_count: inputs.length };
}

// One crate's manifest facts through `cargo metadata --no-deps` (no network, no lock change).
export function readCrateManifest(manifestPath) {
  const stdout = execFileSync(
    "cargo",
    ["metadata", "--format-version", "1", "--no-deps", "--offline", "--manifest-path", manifestPath],
    { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], env: { ...process.env, CARGO_NET_OFFLINE: "true" } }
  );
  const data = JSON.parse(stdout);
  const wanted = real(manifestPath);
  const pkg = (data.packages ?? []).find((candidate) => real(candidate.manifest_path) === wanted);
  if (!pkg) throw new Error(`engine stamp: cargo metadata did not report the package at ${manifestPath}`);
  return {
    workspaceRoot: data.workspace_root ?? path.dirname(manifestPath),
    // Normal (kind null) and build path dependencies; dev dependencies are not compiled in.
    pathDependencies: (pkg.dependencies ?? []).filter((dep) => typeof dep.path === "string" && dep.kind !== "dev").map((dep) => path.join(dep.path, "Cargo.toml")),
    buildScripts: (pkg.targets ?? []).filter((target) => (target.kind ?? []).includes("custom-build")).map((target) => target.src_path)
  };
}

// The project-relative input files that exist before a build: the closure's
// manifests, build scripts and `src/` trees, the lock file and the scripts.
export function staticEngineInputs(projectRoot, crateDirectory, readManifest = readCrateManifest) {
  const inputs = new Set(SCRIPT_INPUTS);
  const need = (absolute, what) => {
    const relative = projectRelative(projectRoot, absolute);
    if (relative === null) throw new Error(`engine stamp: ${what} is outside the project: ${absolute}`);
    return relative;
  };
  const rootManifest = path.join(crateDirectory, "Cargo.toml");
  const seen = new Set();
  const pending = [rootManifest];
  let rootFacts = null;
  while (pending.length > 0) {
    const manifest = pending.pop();
    const key = real(manifest);
    if (seen.has(key)) continue;
    seen.add(key);
    const facts = readManifest(manifest);
    if (rootFacts === null) rootFacts = facts;
    const crate = need(path.dirname(manifest), "engine crate");
    inputs.add(`${crate}/Cargo.toml`);
    if (existsSync(path.join(projectRoot, crate, "build.rs"))) inputs.add(`${crate}/build.rs`);
    for (const script of facts.buildScripts) inputs.add(need(script, "build script"));
    for (const file of listTreeFiles(projectRoot, `${crate}/src`)) inputs.add(file);
    pending.push(...facts.pathDependencies);
  }
  const workspace = need(rootFacts.workspaceRoot, "engine workspace root");
  inputs.add(`${workspace}/Cargo.lock`);
  if (real(rootFacts.workspaceRoot) !== real(crateDirectory)) inputs.add(`${workspace}/Cargo.toml`);
  return [...inputs].sort();
}

// The build context that changes the output without being a project file.
export function engineStampContext(cargoCwd, env = process.env) {
  const context = [];
  const rustc = execFileSync(env.RUSTC || "rustc", ["-vV"], { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });
  context.push(["rustc -vV", rustc.trim()]);
  for (const name of Object.keys(env).filter((key) => CONTEXT_ENV.test(key)).sort()) context.push([`env ${name}`, env[name]]);
  // Cargo reads `.cargo/config.toml` (or `config`) from its working directory
  // upward, then from CARGO_HOME.
  const configs = [];
  for (let dir = path.resolve(cargoCwd); ; dir = path.dirname(dir)) {
    configs.push(path.join(dir, ".cargo", "config.toml"), path.join(dir, ".cargo", "config"));
    if (path.dirname(dir) === dir) break;
  }
  const cargoHome = env.CARGO_HOME || path.join(os.homedir(), ".cargo");
  configs.push(path.join(cargoHome, "config.toml"), path.join(cargoHome, "config"));
  for (const config of [...new Set(configs)]) {
    if (existsSync(config) && statSync(config).isFile()) context.push([`cargo config ${config}`, sha256(readFileSync(config))]);
  }
  return context;
}

// The prerequisite paths of a Makefile-syntax dep-info file's first rule.
export function parseDepInfo(text) {
  const line = text.replace(/\\\r?\n/g, " ").split(/\r?\n/).find((candidate) => candidate.trim().length > 0) ?? "";
  const tokens = [];
  let current = "";
  for (let index = 0; index < line.length; index += 1) {
    const char = line[index];
    if (char === "\\" && index + 1 < line.length && (line[index + 1] === " " || line[index + 1] === "\\")) {
      current += line[index + 1];
      index += 1;
    } else if (char === " " || char === "\t") {
      if (current) tokens.push(current);
      current = "";
    } else {
      current += char;
    }
  }
  if (current) tokens.push(current);
  const targetEnd = tokens.findIndex((token) => token.endsWith(":"));
  if (targetEnd < 0) throw new Error("engine stamp: dep-info has no rule");
  return tokens.slice(targetEnd + 1);
}

// The dep-info prerequisites inside the project and outside build output.
export function compileReadsFromDepInfo(projectRoot, depInfoText, excludedRoots = []) {
  const reads = new Set();
  const roots = excludedRoots.map(real);
  for (const prerequisite of parseDepInfo(depInfoText)) {
    const absolute = real(prerequisite);
    if (roots.some((root) => absolute === root || absolute.startsWith(root + path.sep))) continue;
    const relative = projectRelative(projectRoot, absolute);
    if (relative !== null && !excluded(relative)) reads.add(relative);
  }
  return [...reads].sort();
}

// The full stamp: the static inputs, the recorded compile reads and the context.
export function computeEngineStamp({ projectRoot, crateDirectory, compileReads = [], context = [], readManifest = readCrateManifest }) {
  const inputs = staticEngineInputs(projectRoot, crateDirectory, readManifest);
  const known = new Set(inputs);
  const recorded = [...new Set(compileReads)].filter((relative) => !known.has(relative)).sort();
  const { hash, input_count } = hashInputs(projectRoot, [...inputs, ...recorded], context);
  return {
    format: STAMP_FORMAT,
    crate: projectRelative(projectRoot, crateDirectory),
    hash,
    input_count,
    compile_reads: recorded
  };
}

// A stamp file's text (null when absent) as a stamp, or the reason it is not one.
export function parseStampText(text) {
  if (text === null || text === undefined) return { stamp: null, problem: "stamp file missing" };
  let value;
  try {
    value = JSON.parse(text);
  } catch {
    return { stamp: null, problem: "stamp file unreadable" };
  }
  if (!value || typeof value !== "object" || Array.isArray(value)) return { stamp: null, problem: "stamp file unreadable" };
  if (value.format !== STAMP_FORMAT) return { stamp: null, problem: `stamp format is not ${STAMP_FORMAT}` };
  if (typeof value.hash !== "string" || !Array.isArray(value.compile_reads) || !value.compile_reads.every(isSafeRelative)) {
    return { stamp: null, problem: "stamp file unreadable" };
  }
  return { stamp: value, problem: null };
}

// Whether an engine folder must be rebuilt. Pure: the folder's presence, its
// stamp file's text (null when absent) and the hash its sources have now.
export function decideEngine({ folderPresent, stampText, currentHash }) {
  if (!folderPresent) return { action: "rebuild", reason: "engine folder missing" };
  const { stamp, problem } = parseStampText(stampText);
  if (problem) return { action: "rebuild", reason: problem };
  if (stamp.hash !== currentHash) return { action: "rebuild", reason: `inputs changed since stamp ${stamp.hash}` };
  return { action: "current", reason: null };
}

export function readStampText(outDir) {
  try {
    return readFileSync(path.join(outDir, STAMP_FILENAME), "utf8");
  } catch {
    return null;
  }
}

// Recompute an engine folder's stamp from its sources and decide.
export function checkEngine({ projectRoot, crateDirectory, outDir, context, readManifest = readCrateManifest }) {
  const folderPresent = existsSync(outDir);
  const stampText = folderPresent ? readStampText(outDir) : null;
  const { stamp } = parseStampText(stampText);
  const current = computeEngineStamp({ projectRoot, crateDirectory, compileReads: stamp?.compile_reads ?? [], context, readManifest });
  return { ...decideEngine({ folderPresent, stampText, currentHash: current.hash }), hash: current.hash, stamp: current };
}

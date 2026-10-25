#!/usr/bin/env node
// Builds the wasm32 browser operation engine (DEC-020 / ADR-0001):
// `core/model_operations/operation_applier` compiled with `--features wasm`,
// then bound with the pinned wasm-bindgen CLI into web-target glue under
// `public/wasm-engine/` (TP-APP-R2-WASMPKG-001: Vite serves `public/` at the
// site root in dev AND copies it verbatim into `dist`, so the same root
// `/wasm-engine/` URL works in dev, `vite preview`, and the packaged tauri://
// asset protocol — the previous `src/.../__generated__/` location was never
// emitted into `dist`). Generated artifacts are not committed. Every missing
// prerequisite fails with the exact remediation command — no silent fallback
// (operation-seam plan §3 T3; ADR-0001).
// Glue is written to a temp directory and renamed into place so concurrent
// readers never see a half-written artifact set (DEC-025 F-4 rider).
// Each engine folder also receives `source-stamp.json`, the content stamp of
// the sources it was built from (`engineSourceStamp.mjs`), written with the
// glue so it exists only after a successful build; the native smoke recomputes
// it and rebuilds a stale engine. The stamp is written only for a build whose
// complete input set held still: the inputs (static ones plus every compile
// read known so far) are digested before cargo runs and again after; a compile
// read the dep-info newly reveals, a source added or removed, or any changed
// digest means cargo builds again, up to MAX_STABLE_BUILD_ATTEMPTS, then the
// build fails naming the inputs.
// Each engine builds in its own target directory, `target/wasm-engines/<engine>`
// under this app (CARGO_TARGET_DIR is set to it; cargo metadata must report it
// back). When the engine's stamp is missing or differs from its sources, and
// before every further attempt after an unstamped build, that directory is
// deleted first, so the build is clean: cargo's freshness check, which relies
// on modification times, cannot keep an old artifact after an edit that kept a
// file's time or a symbolic link retargeted to an older file. A current stamp
// builds incrementally.

import { execFileSync } from "node:child_process";
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  STAMP_FILENAME,
  checkEngine,
  compileReadsFromDepInfo,
  digestInputs,
  engineInputs,
  engineStampContext,
  inputDrift,
  parseStampText,
  readStampText,
  stampFromDigests
} from "./engineSourceStamp.mjs";

const PINNED_WASM_BINDGEN_VERSION = "0.2.123";
export const MAX_STABLE_BUILD_ATTEMPTS = 3;

const desktopRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const projectRoot = path.resolve(desktopRoot, "..", "..");
const crateDir = path.join(projectRoot, "core", "model_operations", "operation_applier");
const crateManifest = path.join(crateDir, "Cargo.toml");
const outDir = path.join(desktopRoot, "public", "wasm-engine");
// Pre-TP-APP-R2-WASMPKG-001 emission location; removed on every build so a
// stale copy can never be mistaken for the live artifact set.
const legacyOutDir = path.join(desktopRoot, "src", "services", "wasmEngine", "__generated__");
// The engines this script builds, in order; `native-smoke/run.mjs` checks the
// same list's stamps.
export const ENGINE_BUILDS = [
  { crateDirectory: crateDir, crateName: "open_pipe_stress_operation_applier", destination: outDir },
  { crateDirectory: path.join(projectRoot, "core", "loads", "self_weight_wasm"), crateName: "open_pipe_stress_self_weight_wasm", destination: path.join(desktopRoot, "public", "self-weight-engine") }
];

function fail(lines) {
  console.error(["", "[build-wasm-engine] FAILED:", ...lines.map((line) => `  ${line}`), ""].join("\n"));
  process.exit(1);
}

function tryRun(command, args, { cwd, env = {} } = {}) {
  try {
    return {
      ok: true,
      stdout: execFileSync(command, args, {
        cwd,
        encoding: "utf8",
        env: { ...process.env, CARGO_NET_OFFLINE: "true", ...env },
        stdio: ["ignore", "pipe", "pipe"]
      })
    };
  } catch (error) {
    return { ok: false, stderr: error?.stderr?.toString?.() ?? String(error) };
  }
}

export function resolveWasmArtifact(crateDirectory = crateDir, crateName = "open_pipe_stress_operation_applier", env = {}) {
  const metadata = tryRun(
    "cargo",
    ["metadata", "--format-version", "1", "--no-deps"],
    { cwd: crateDirectory, env }
  );
  if (!metadata.ok) {
    throw new Error(`cargo metadata failed: ${metadata.stderr}`);
  }

  let targetDirectory;
  try {
    targetDirectory = JSON.parse(metadata.stdout).target_directory;
  } catch (error) {
    throw new Error(`cargo metadata returned invalid JSON: ${error.message}`);
  }
  if (typeof targetDirectory !== "string" || targetDirectory.length === 0) {
    throw new Error("cargo metadata did not return a target_directory");
  }

  return path.join(
    targetDirectory,
    "wasm32-unknown-unknown",
    "release",
    `${crateName}.wasm`
  );
}

// The dedicated target directory of one engine, inside this app.
const engineTargetsRoot = path.join(desktopRoot, "target", "wasm-engines");
export const engineTargetDirectory = (engine) => path.join(engineTargetsRoot, engine);

// Delete an engine's dedicated target directory, after checking that it is one:
// `<app>/target/wasm-engines/<engine>` for an engine this script builds, with no
// symbolic link from the app down to it.
export function removeEngineTargetDirectory(directory, engines = ENGINE_BUILDS.map(({ destination }) => path.basename(destination))) {
  const engine = path.basename(directory);
  if (!engines.includes(engine) || directory !== engineTargetDirectory(engine)) {
    throw new Error(`refusing to delete ${directory}: not an engine's dedicated target directory`);
  }
  for (const step of [path.dirname(engineTargetsRoot), engineTargetsRoot, directory]) {
    if (lstatSync(step, { throwIfNoEntry: false })?.isSymbolicLink()) throw new Error(`refusing to delete ${directory}: ${step} is a symbolic link`);
  }
  if (!existsSync(directory)) return;
  if (realpathSync(path.dirname(directory)) !== path.join(realpathSync(desktopRoot), "target", "wasm-engines")) {
    throw new Error(`refusing to delete ${directory}: it does not resolve inside ${desktopRoot}`);
  }
  rmSync(directory, { recursive: true, force: true });
}

const CARGO_BUILD_ARGS = [
  "build",
  "--offline",
  "--target",
  "wasm32-unknown-unknown",
  "--features",
  "wasm",
  "--release"
];

function main() {

// 1. cargo present.
if (!tryRun("cargo", ["--version"]).ok) {
  fail([
    "`cargo` was not found on PATH.",
    "Install the Rust toolchain (https://rustup.rs) before building the wasm engine."
  ]);
}

// 2. wasm32 target installed.
const targetProbe = tryRun("rustup", ["target", "list", "--installed"]);
if (!targetProbe.ok) {
  fail([
    "`rustup` was not found on PATH, so the wasm32 target cannot be verified.",
    "Install rustup, then run: rustup target add wasm32-unknown-unknown"
  ]);
}
if (!targetProbe.stdout.split("\n").map((line) => line.trim()).includes("wasm32-unknown-unknown")) {
  fail([
    "The `wasm32-unknown-unknown` Rust target is not installed.",
    "Run: rustup target add wasm32-unknown-unknown"
  ]);
}

// 3. wasm-bindgen CLI present at the pinned version (must match the crate's
//    pinned wasm-bindgen dependency exactly).
let wasmBindgen = "wasm-bindgen";
let bindgenProbe = tryRun(wasmBindgen, ["--version"]);
if (!bindgenProbe.ok) {
  const cargoBinFallback = path.join(os.homedir(), ".cargo", "bin", "wasm-bindgen");
  if (existsSync(cargoBinFallback)) {
    wasmBindgen = cargoBinFallback;
    bindgenProbe = tryRun(wasmBindgen, ["--version"]);
  }
}
if (!bindgenProbe.ok) {
  fail([
    "The `wasm-bindgen` CLI was not found on PATH (or in ~/.cargo/bin).",
    `Run: cargo install wasm-bindgen-cli --version ${PINNED_WASM_BINDGEN_VERSION} --locked`
  ]);
}
const bindgenVersion = bindgenProbe.stdout.trim().split(/\s+/)[1];
if (bindgenVersion !== PINNED_WASM_BINDGEN_VERSION) {
  fail([
    `wasm-bindgen CLI version ${bindgenVersion} does not match the pinned crate version ${PINNED_WASM_BINDGEN_VERSION}.`,
    "Mismatched CLI/crate versions produce broken or subtly different glue.",
    `Run: cargo install wasm-bindgen-cli --version ${PINNED_WASM_BINDGEN_VERSION} --locked --force`
  ]);
}

// The stamp context (rustc identity, compiler, wrapper, linker and rustflag
// environment, cargo configuration); cargo runs from this process's working
// directory.
let stampContext;
try {
  stampContext = engineStampContext(process.cwd());
} catch (error) {
  fail(["Could not read the toolchain identity for the engine source stamp:", error.message]);
}

for (const { crateDirectory, crateName, destination } of ENGINE_BUILDS) {
const crateManifest = path.join(crateDirectory, "Cargo.toml");
const outDir = destination;
const crateLabel = path.relative(projectRoot, crateDirectory);
const targetDirectory = engineTargetDirectory(path.basename(outDir));
// 4. Compile the engine to wasm32 with the wasm feature, in its own target
//    directory, until a build's complete input set held still across it (4b).
//    A missing or stale stamp, or inputs that moved, mean a clean build.
let clean = null;
try {
  const current = checkEngine({ projectRoot, crateDirectory, outDir, context: stampContext });
  clean = current.action === "rebuild" ? current.reason : null;
} catch (error) {
  fail(["Could not check the engine's source stamp:", error.message]);
}
let knownReads = parseStampText(readStampText(outDir)).stamp?.compile_reads ?? [];
let wasmArtifact;
let stamp = null;
let drift = [];
for (let attempt = 1; attempt <= MAX_STABLE_BUILD_ATTEMPTS && stamp === null; attempt += 1) {
  if (clean !== null) {
    try {
      removeEngineTargetDirectory(targetDirectory);
    } catch (error) {
      fail([error.message]);
    }
    console.log(`[build-wasm-engine] ${crateLabel}: clean build in ${path.relative(desktopRoot, targetDirectory)} (${clean})`);
  }
  let before;
  let beforeDigests;
  try {
    before = engineInputs({ projectRoot, crateDirectory, compileReads: knownReads });
    beforeDigests = digestInputs(projectRoot, before.all);
  } catch (error) {
    fail(["Could not list the engine's source inputs for its stamp:", error.message]);
  }
  console.log(`[build-wasm-engine] cargo ${CARGO_BUILD_ARGS.join(" ")} (${crateLabel}, attempt ${attempt}/${MAX_STABLE_BUILD_ATTEMPTS})`);
  const cargoEnv = { CARGO_TARGET_DIR: targetDirectory };
  const build = tryRun("cargo", [...CARGO_BUILD_ARGS, "--manifest-path", crateManifest], { env: cargoEnv });
  if (!build.ok) {
    fail(["`cargo build` for the wasm32 engine failed:", build.stderr]);
  }
  try {
    wasmArtifact = resolveWasmArtifact(crateDirectory, crateName, cargoEnv);
  } catch (error) {
    fail(["Could not resolve Cargo's wasm artifact directory:", error.message]);
  }
  if (wasmArtifact !== path.join(targetDirectory, "wasm32-unknown-unknown", "release", `${crateName}.wasm`)) {
    fail([`cargo reports the wasm artifact at ${wasmArtifact}, outside the engine's dedicated target directory ${targetDirectory}`]);
  }
  if (!existsSync(wasmArtifact)) {
    fail([`cargo build succeeded but the expected artifact is missing: ${wasmArtifact}`]);
  }
  // 4b. The content stamp: the static inputs plus the compile reads cargo's
  //     dep-info lists for this artifact (schemas, fixtures, `#[path]` modules,
  //     build-script `rerun-if-changed` files). No dep-info, no stamp.
  const depInfo = wasmArtifact.replace(/\.wasm$/, ".d");
  try {
    const compileReads = compileReadsFromDepInfo(projectRoot, readFileSync(depInfo, "utf8"), [targetDirectory]);
    const after = engineInputs({ projectRoot, crateDirectory, compileReads });
    const afterDigests = digestInputs(projectRoot, after.all);
    drift = inputDrift({ beforeStatic: before.staticInputs, beforeDigests, after, afterDigests });
    if (drift.length === 0) {
      stamp = { engine: path.basename(outDir), ...stampFromDigests({ projectRoot, crateDirectory, digests: afterDigests, recorded: after.recorded, context: stampContext }) };
    } else {
      console.log(`[build-wasm-engine] ${crateLabel}: not stamped (${drift.join("; ")}); building again`);
      knownReads = [...new Set([...knownReads, ...compileReads])];
      clean = "the last build was not stamped";
    }
  } catch (error) {
    fail([`Could not stamp the engine built from ${crateLabel} (dep-info ${depInfo}):`, error.message]);
  }
}
if (stamp === null) {
  fail([`The inputs of the engine built from ${crateLabel} did not hold still across ${MAX_STABLE_BUILD_ATTEMPTS} builds, so it is not stamped:`, ...drift, "Stop whatever is editing them and run `npm run build:wasm` again."]);
}

// 5. Generate the web-target JS glue into a sibling temp directory, then
//    swap it into place with renames (DEC-025 F-4 rider). Writing directly
//    into `__generated__/` left a window where a concurrent Vitest/dev-server
//    read saw a half-written artifact set; the rename swap replaces the whole
//    directory at once. Stale `.tmp-*`/`.old-*` siblings from interrupted
//    runs are removed first.
const generatedParent = path.dirname(outDir);
const generatedName = path.basename(outDir);
mkdirSync(generatedParent, { recursive: true });
const legacyParent = path.dirname(legacyOutDir);
const legacyName = path.basename(legacyOutDir);
if (existsSync(legacyParent)) {
  for (const entry of readdirSync(legacyParent)) {
    if (entry === legacyName || entry.startsWith(`${legacyName}.tmp-`) || entry.startsWith(`${legacyName}.old-`)) {
      rmSync(path.join(legacyParent, entry), { recursive: true, force: true });
    }
  }
}
for (const entry of readdirSync(generatedParent)) {
  if (entry.startsWith(`${generatedName}.tmp-`) || entry.startsWith(`${generatedName}.old-`)) {
    rmSync(path.join(generatedParent, entry), { recursive: true, force: true });
  }
}
const tmpDir = `${outDir}.tmp-${process.pid}`;
const oldDir = `${outDir}.old-${process.pid}`;

console.log(`[build-wasm-engine] wasm-bindgen --target web --out-dir ${path.relative(desktopRoot, tmpDir)}`);
const bindgen = tryRun(wasmBindgen, ["--target", "web", "--out-dir", tmpDir, wasmArtifact]);
if (!bindgen.ok) {
  rmSync(tmpDir, { recursive: true, force: true });
  fail(["`wasm-bindgen` glue generation failed:", bindgen.stderr]);
}
writeFileSync(path.join(tmpDir, STAMP_FILENAME), `${JSON.stringify(stamp, null, 2)}\n`);

if (existsSync(outDir)) {
  renameSync(outDir, oldDir);
}
try {
  renameSync(tmpDir, outDir);
} catch (error) {
  if (existsSync(oldDir)) renameSync(oldDir, outDir);
  throw error;
}
rmSync(oldDir, { recursive: true, force: true });

console.log(
  `[build-wasm-engine] OK — wasm operation engine generated at ${path.relative(desktopRoot, outDir)} ` +
    `(wasm-bindgen ${bindgenVersion}; stamp ${stamp.hash} over ${stamp.input_count} inputs; artifacts are intentionally not committed)`
);
}

}

const invokedPath = process.argv[1] ? path.resolve(process.argv[1]) : null;
if (invokedPath === fileURLToPath(import.meta.url)) {
  main();
}

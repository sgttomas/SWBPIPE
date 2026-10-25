// Content stamp for the desktop's WebAssembly engines (DEL-07-11 native smoke).
//
// `build-wasm-engine.mjs` writes `source-stamp.json` into each engine's output
// folder after a build whose inputs held still (see there); `native-smoke/run.mjs`
// recomputes the stamp and rebuilds when it differs. The stamp is a SHA-256
// over, sorted by project-relative path, each input's path, every symbolic link
// met while resolving that path (the link's path and its target text) and the
// resolved file's bytes. The inputs are:
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
//     product_physics' build script digests), by the path cargo reported, not
//     its canonical target. They are recorded in the stamp, because only a
//     build can list them; a change that adds one edits a file already hashed.
//
// and a context: `rustc -vV` of the rustc cargo runs (RUSTC, else
// CARGO_BUILD_RUSTC, else `rustc`), the compiler, wrapper, linker, rustflag and
// profile environment (CONTEXT_ENV), and the bytes of the cargo configuration
// files cargo would read. Those files' keys are decoded as TOML (bare, quoted
// with any escape, literal, dotted, in table headers and inline tables) and a
// file is refused by name when a key names a compiler or wrapper (`rustc`,
// `rustc-wrapper`, `rustc-workspace-wrapper`, anywhere), moves cargo's output
// (`build.build-dir`, `build.target-dir`) or includes another file
// (`include`), or when it cannot be tokenized with confidence. A
// `[target.<triple>]` `linker`, `runner` or `rustflags` key is not refused: the
// file's bytes are in the context. Not covered: the contents of a wrapper or
// linker program (only its name) and files outside the project. The stamp sees an edit
// that keeps a file's modification time; `build-wasm-engine.mjs` then builds in
// a clean target directory, so cargo's mtime-based freshness cannot keep the old
// output. Build output (`target/`, `node_modules/`, `.git/`) is never an input.
import { execFileSync } from "node:child_process";
import { refuseHostSlotCommand } from "./worktreeBuildLock.mjs";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, readdirSync, readFileSync, readlinkSync, realpathSync, statSync } from "node:fs";
import os from "node:os";
import path from "node:path";

export const STAMP_FORMAT = "openpipestress.engine-source-stamp.v2";
export const STAMP_FILENAME = "source-stamp.json";
const EXCLUDED_NAMES = new Set(["target", "node_modules", ".git", ".DS_Store"]);
const SCRIPT_INPUTS = ["apps/desktop/scripts/build-wasm-engine.mjs", "apps/desktop/scripts/engineSourceStamp.mjs"];
// The environment that selects or configures the compiler, wrappers and linker:
// RUSTC, RUSTC_WRAPPER, RUSTC_WORKSPACE_WRAPPER, RUSTC_BOOTSTRAP, RUSTFLAGS,
// CARGO_ENCODED_RUSTFLAGS, every CARGO_BUILD_* (rustc, rustc-wrapper,
// rustflags, target, ...), every CARGO_TARGET_<triple>_* (linker, rustflags,
// runner) and CARGO_HOST_*, CARGO_PROFILE_*, CARGO_UNSTABLE_* and
// RUSTUP_TOOLCHAIN. CARGO_TARGET_DIR moves the output, not its bytes.
const CONTEXT_ENV = /^(RUSTC|RUSTC_WRAPPER|RUSTC_WORKSPACE_WRAPPER|RUSTC_BOOTSTRAP|RUSTFLAGS|CARGO_ENCODED_RUSTFLAGS|RUSTUP_TOOLCHAIN|CARGO_BUILD_.+|CARGO_TARGET_.+|CARGO_HOST_.+|CARGO_PROFILE_.+|CARGO_UNSTABLE_.+)$/;
// The output locations: every cargo the smoke tooling runs has them set to its
// dedicated directory (`dedicatedCargoEnv`), so they move nothing.
const CONTEXT_ENV_IGNORED = new Set(["CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR", "CARGO_BUILD_BUILD_DIR"]);
const MAX_LINK_HOPS = 40;
const COMPILER_CONFIG_KEYS = new Set(["rustc", "rustc-wrapper", "rustc-workspace-wrapper"]);
const OUTPUT_CONFIG_KEYS = new Set(["build-dir", "target-dir"]);
// The directories cargo descends into inside a profile directory when it builds
// or cleans (`<dir>/<profile>/` and `<dir>/<triple>/<profile>/`).
const CARGO_PROFILE_SUBDIRECTORIES = ["deps", "build", ".fingerprint", "incremental", "examples"];

const toPosix = (value) => value.split(path.sep).join("/");
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
// The kernel's resolution (realpath(3)): `..` after a symbolic link leaves the
// link's target, unlike Node's lexical realpathSync.
const real = (value) => (existsSync(value) ? realpathSync.native(value) : path.resolve(value));

function isSafeRelative(value) {
  return typeof value === "string" && value.length > 0 && !path.isAbsolute(value) && !value.split("/").some((part) => part === ".." || part === "" || part === ".");
}

// Project-relative POSIX path of an absolute path, or null outside the project.
export function projectRelative(projectRoot, absolute) {
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

const within = (root, candidate) => candidate === root || candidate.startsWith(root + path.sep);

// The identity of one project-relative input: each symbolic link met while
// resolving it component by component (the link's path, project-relative when
// inside the project, and its target text), then the resolved file's size and
// SHA-256, or `missing` when nothing readable is there.
export function inputDigest(projectRoot, relative) {
  const base = real(projectRoot);
  const shown = (absolute) => (within(base, absolute) && absolute !== base ? toPosix(path.relative(base, absolute)) : absolute);
  const links = [];
  const pending = relative.split("/");
  let current = base;
  let hops = 0;
  let missing = false;
  while (pending.length > 0 && !missing) {
    const part = pending.shift();
    if (part === "" || part === ".") continue;
    const next = part === ".." ? path.dirname(current) : path.join(current, part);
    const stat = lstatSync(next, { throwIfNoEntry: false });
    if (!stat) {
      missing = true;
    } else if (stat.isSymbolicLink()) {
      hops += 1;
      if (hops > MAX_LINK_HOPS) throw new Error(`engine stamp: too many symbolic links resolving ${relative}`);
      const target = readlinkSync(next);
      links.push(`link ${JSON.stringify(shown(next))} ${JSON.stringify(target)}`);
      if (path.isAbsolute(target)) current = path.parse(target).root;
      pending.unshift(...target.split("/"));
    } else {
      current = next;
    }
  }
  let bytes = null;
  if (!missing) {
    try {
      bytes = readFileSync(current);
    } catch {
      bytes = null;
    }
  }
  return [...links, bytes === null ? "missing" : `${bytes.length} ${sha256(bytes)}`].join(" ");
}

// Each of `relativePaths` (deduplicated, sorted) with its digest, as a Map.
export function digestInputs(projectRoot, relativePaths) {
  const inputs = [...new Set(relativePaths)].sort();
  for (const relative of inputs) {
    if (!isSafeRelative(relative)) throw new Error(`engine stamp: input path is not project-relative: ${relative}`);
  }
  return new Map(inputs.map((relative) => [relative, inputDigest(projectRoot, relative)]));
}

// The deterministic hash of input digests and `context` (an array of
// [name, value] pairs).
export function hashDigests(digests, context = []) {
  const hash = createHash("sha256");
  hash.update(`${STAMP_FORMAT}\n`);
  for (const [name, value] of context) hash.update(`context ${JSON.stringify(name)} ${JSON.stringify(value)}\n`);
  for (const relative of [...digests.keys()].sort()) hash.update(`input ${JSON.stringify(relative)} ${digests.get(relative)}\n`);
  return { hash: `sha256:${hash.digest("hex")}`, input_count: digests.size };
}

// The deterministic hash of `relativePaths` and `context`. A missing input
// hashes as missing.
export function hashInputs(projectRoot, relativePaths, context = []) {
  return hashDigests(digestInputs(projectRoot, relativePaths), context);
}

// One crate's manifest facts through `cargo metadata --no-deps` (no network, no lock change).
export function readCrateManifest(manifestPath) {
  refuseHostSlotCommand("cargo");
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
    name: pkg.name,
    workspaceRoot: data.workspace_root ?? path.dirname(manifestPath),
    // Normal (kind null) and build path dependencies; dev dependencies are not compiled in.
    pathDependencies: (pkg.dependencies ?? []).filter((dep) => typeof dep.path === "string" && dep.kind !== "dev").map((dep) => path.join(dep.path, "Cargo.toml")),
    buildScripts: (pkg.targets ?? []).filter((target) => (target.kind ?? []).includes("custom-build")).map((target) => target.src_path)
  };
}

// A crate's closure before a build: its project-relative files (the closure's
// manifests, build scripts and `src/` trees and the lock file) and the closure's
// package names.
export function crateClosure(projectRoot, crateDirectory, readManifest = readCrateManifest) {
  const inputs = new Set();
  const packages = new Set();
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
    if (facts.name) packages.add(facts.name);
    const crate = need(path.dirname(manifest), "crate");
    inputs.add(`${crate}/Cargo.toml`);
    if (existsSync(path.join(projectRoot, crate, "build.rs"))) inputs.add(`${crate}/build.rs`);
    for (const script of facts.buildScripts) inputs.add(need(script, "build script"));
    for (const file of listTreeFiles(projectRoot, `${crate}/src`)) inputs.add(file);
    pending.push(...facts.pathDependencies);
  }
  const workspace = need(rootFacts.workspaceRoot, "workspace root");
  inputs.add(`${workspace}/Cargo.lock`);
  if (real(rootFacts.workspaceRoot) !== real(crateDirectory)) inputs.add(`${workspace}/Cargo.toml`);
  return { inputs: [...inputs].sort(), packages: [...packages].sort() };
}

// An engine's static inputs: its crate closure and the build scripts.
export function staticEngineInputs(projectRoot, crateDirectory, readManifest = readCrateManifest) {
  return [...new Set([...SCRIPT_INPUTS, ...crateClosure(projectRoot, crateDirectory, readManifest).inputs])].sort();
}

// The decoded key path of every table header and key/value pair in a TOML
// document (`[a."b"]` then `c.'d' = { e = 1 }` gives [a,b], [a,b,c,d] and
// [a,b,c,d,e]). Throws when the text is not TOML this scanner reads with
// confidence; values are skipped, not interpreted.
export function tomlKeyPaths(text) {
  const src = text.startsWith("﻿") ? text.slice(1) : text;
  const paths = [];
  let i = 0;
  const fail = (why) => {
    const line = src.slice(0, i).split("\n").length;
    throw new Error(`${why} at line ${line}`);
  };
  const at = (literal) => src.startsWith(literal, i);
  const blank = () => {
    while (i < src.length && (src[i] === " " || src[i] === "\t")) i += 1;
  };
  const comment = () => {
    if (src[i] !== "#") return;
    while (i < src.length && src[i] !== "\n") i += 1;
  };
  const newline = () => {
    if (at("\r\n")) i += 2;
    else if (src[i] === "\n") i += 1;
    else return false;
    return true;
  };
  // Whitespace, newlines and comments (inside arrays and inline tables).
  const filler = () => {
    for (;;) {
      blank();
      comment();
      if (!newline()) return;
    }
  };
  const lineEnd = () => {
    blank();
    comment();
    if (i < src.length && !newline()) fail("unexpected text after a value");
  };
  const hex = (count) => {
    const digits = src.slice(i, i + count);
    if (!new RegExp(`^[0-9A-Fa-f]{${count}}$`).test(digits)) fail("bad escape");
    i += count;
    const code = Number.parseInt(digits, 16);
    if (code > 0x10ffff || (code >= 0xd800 && code <= 0xdfff)) fail("escape is not a Unicode scalar value");
    return String.fromCodePoint(code);
  };
  const ESCAPES = { b: "\b", t: "\t", n: "\n", f: "\f", r: "\r", e: "\u001b", '"': '"', "\\": "\\" };
  // A one-line basic string, decoded; `i` is at its opening quote.
  const basicString = () => {
    i += 1;
    let out = "";
    for (;;) {
      if (i >= src.length || src[i] === "\n" || src[i] === "\r") fail("unterminated string");
      const char = src[i];
      i += 1;
      if (char === '"') return out;
      if (char !== "\\") {
        out += char;
        continue;
      }
      const escape = src[i];
      i += 1;
      if (escape in ESCAPES) out += ESCAPES[escape];
      else if (escape === "u") out += hex(4);
      else if (escape === "U") out += hex(8);
      else if (escape === "x") out += hex(2);
      else fail("unknown escape");
    }
  };
  const literalString = () => {
    const end = src.indexOf("'", i + 1);
    const value = end < 0 ? null : src.slice(i + 1, end);
    if (value === null || /[\r\n]/.test(value)) fail("unterminated string");
    i = end + 1;
    return value;
  };
  // A multi-line string; its content is not a key, so only its end matters.
  const multiline = (quote) => {
    i += 3;
    for (;;) {
      if (i >= src.length) fail("unterminated multi-line string");
      if (quote === '"' && src[i] === "\\") {
        i += 2;
        continue;
      }
      if (at(quote.repeat(3))) {
        i += 3;
        // Up to two more quotes belong to the content.
        for (let extra = 0; extra < 2 && src[i] === quote; extra += 1) i += 1;
        return;
      }
      i += 1;
    }
  };
  const simpleKey = () => {
    if (src[i] === '"') return basicString();
    if (src[i] === "'") return literalString();
    const match = /^[A-Za-z0-9_-]+/.exec(src.slice(i));
    if (!match) fail("expected a key");
    i += match[0].length;
    return match[0];
  };
  const dottedKey = () => {
    const key = [];
    for (;;) {
      blank();
      key.push(simpleKey());
      blank();
      if (src[i] !== ".") return key;
      i += 1;
    }
  };
  const SCALAR = /^(true|false|[+-]?(inf|nan)|[+-]?[0-9][0-9_]*(\.[0-9_]+)?([eE][+-]?[0-9_]+)?|0x[0-9A-Fa-f_]+|0o[0-7_]+|0b[01_]+|[0-9]{4}-[0-9]{2}-[0-9]{2}([Tt ][0-9]{2}:[0-9]{2}(:[0-9]{2}(\.[0-9]+)?)?([Zz]|[+-][0-9]{2}:[0-9]{2})?)?|[0-9]{2}:[0-9]{2}(:[0-9]{2}(\.[0-9]+)?)?)$/;
  const value = (prefix) => {
    if (at('"""') || at("'''")) return multiline(src[i]);
    if (src[i] === '"') return void basicString();
    if (src[i] === "'") return void literalString();
    if (src[i] === "[") {
      i += 1;
      for (;;) {
        filler();
        if (src[i] === "]") break;
        value(prefix);
        filler();
        if (src[i] === ",") i += 1;
        else if (src[i] !== "]") fail("expected , or ] in an array");
      }
      i += 1;
      return undefined;
    }
    if (src[i] === "{") {
      i += 1;
      for (;;) {
        filler();
        if (src[i] === "}") break;
        keyValue(prefix);
        filler();
        if (src[i] === ",") i += 1;
        else if (src[i] !== "}") fail("expected , or } in an inline table");
      }
      i += 1;
      return undefined;
    }
    let match = /^[0-9A-Za-z_+.:-]+/.exec(src.slice(i));
    // A date and time separated by a space.
    if (match && /^[0-9]{4}-[0-9]{2}-[0-9]{2}$/.test(match[0])) match = /^[0-9]{4}-[0-9]{2}-[0-9]{2}( [0-9]{2}:[0-9A-Za-z_+.:-]+)?/.exec(src.slice(i));
    if (!match || !SCALAR.test(match[0])) fail("unrecognised value");
    i += match[0].length;
    return undefined;
  };
  const keyValue = (prefix) => {
    const key = [...prefix, ...dottedKey()];
    paths.push(key);
    if (src[i] !== "=") fail("expected =");
    i += 1;
    blank();
    value(key);
  };
  let table = [];
  while (i < src.length) {
    blank();
    comment();
    if (newline() || i >= src.length) continue;
    if (src[i] === "[") {
      const array = at("[[");
      i += array ? 2 : 1;
      table = dottedKey();
      if (!at(array ? "]]" : "]")) fail("unterminated table header");
      i += array ? 2 : 1;
      paths.push(table);
      lineEnd();
      continue;
    }
    keyValue(table);
    lineEnd();
  }
  return paths;
}

// Why a cargo configuration file's text is refused, or null. A key named for a
// compiler or wrapper is refused wherever it appears (conservatively: only
// `build.<key>` selects one); an output directory under `build`; `include`.
export function refusedCargoConfig(text) {
  let paths;
  try {
    paths = tomlKeyPaths(text);
  } catch (error) {
    return `cannot be read as TOML with confidence (${error.message}), so the compiler and output keys it may set are unknown; the stamp refuses it`;
  }
  for (const key of paths) {
    const compiler = key.find((segment) => COMPILER_CONFIG_KEYS.has(segment));
    if (compiler) return `sets ${compiler} (a compiler or wrapper key), which the stamp cannot identify; remove it and set RUSTC or RUSTC_WRAPPER instead`;
    if (key[0] === "build" && OUTPUT_CONFIG_KEYS.has(key[1])) {
      return `sets build.${key[1]} (an output directory key), which could move cargo's build out of the smoke's dedicated directory; remove it`;
    }
    if (key[0] === "include") return "sets include (another configuration file), which the stamp does not read; remove it";
  }
  return null;
}

// The environment that puts cargo's output and intermediate artifacts in
// `directory`: it overrides any inherited CARGO_TARGET_DIR, CARGO_BUILD_TARGET_DIR
// or CARGO_BUILD_BUILD_DIR and any configuration file.
export function dedicatedCargoEnv(directory) {
  return { CARGO_TARGET_DIR: directory, CARGO_BUILD_TARGET_DIR: directory, CARGO_BUILD_BUILD_DIR: directory };
}

// Refuse, by name, a dedicated cargo directory that is, or holds, a symbolic
// link where cargo descends when it builds or cleans (every entry one and two
// levels down: `<profile>`, `<triple>`, `<profile>/deps`, ...; and `deps`,
// `build`, `.fingerprint`, `incremental` and `examples` of `<triple>/<profile>`),
// or that resolves outside `container`. Cargo would follow such a link and write
// or delete another tree's files. Absent: nothing to check.
export function refuseLinkedCargoTree(directory, container) {
  const top = lstatSync(directory, { throwIfNoEntry: false });
  if (!top) return;
  const refuse = (where, why) => {
    throw new Error(`refusing to let cargo build or clean in ${directory}: ${where} ${why}`);
  };
  if (top.isSymbolicLink()) refuse(directory, "is a symbolic link");
  const realDirectory = realpathSync.native(directory);
  if (!within(realpathSync.native(container), realDirectory)) refuse(directory, `resolves outside ${container}`);
  const check = (candidate) => {
    const stat = lstatSync(candidate, { throwIfNoEntry: false });
    if (!stat) return false;
    if (stat.isSymbolicLink()) refuse(candidate, "is a symbolic link");
    if (!stat.isDirectory()) return false;
    if (!within(realDirectory, realpathSync.native(candidate))) refuse(candidate, `resolves outside ${directory}`);
    return true;
  };
  for (const first of readdirSync(directory)) {
    const level1 = path.join(directory, first);
    if (!check(level1)) continue;
    for (const second of readdirSync(level1)) {
      const level2 = path.join(level1, second);
      if (!check(level2)) continue;
      for (const name of CARGO_PROFILE_SUBDIRECTORIES) check(path.join(level2, name));
    }
  }
}

// The build context that changes the output without being a project file.
export function engineStampContext(cargoCwd, env = process.env) {
  const context = [];
  // Cargo prefers RUSTC, then build.rustc (CARGO_BUILD_RUSTC or configuration).
  const rustcProgram = env.RUSTC || env.CARGO_BUILD_RUSTC || "rustc";
  refuseHostSlotCommand(rustcProgram, [], env);
  const rustc = execFileSync(rustcProgram, ["-vV"], { encoding: "utf8", env, stdio: ["ignore", "pipe", "pipe"] });
  context.push(["rustc -vV", rustc.trim()]);
  for (const name of Object.keys(env).filter((key) => CONTEXT_ENV.test(key) && !CONTEXT_ENV_IGNORED.has(key)).sort()) {
    context.push([`env ${name}`, env[name]]);
  }
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
    if (!existsSync(config) || !statSync(config).isFile()) continue;
    const bytes = readFileSync(config);
    // The compiler or wrapper named there would be the one to identify, and an
    // output directory there would move the build out of the smoke's dedicated
    // directory; this stamp does not resolve cargo configuration, so it refuses
    // rather than mislabel.
    const problem = refusedCargoConfig(bytes.toString("utf8"));
    if (problem) throw new Error(`engine stamp: ${config} ${problem}`);
    context.push([`cargo config ${config}`, sha256(bytes)]);
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

// Whether a "..", read component by component as the kernel does, cancels a
// symbolic link: it then leaves the link's target, not the link's directory,
// and the path no longer identifies the file it reaches, even where both agree
// today. A link in an ancestor that no ".." cancels is harmless.
export function linkBeforeParent(reported) {
  let prefix = path.isAbsolute(reported) ? "" : process.cwd();
  const stack = [];
  for (const part of reported.split("/")) {
    if (part === "" || part === ".") continue;
    if (part === "..") {
      if (stack.pop() === true) return true;
    } else {
      stack.push(lstatSync(`${prefix}/${part}`, { throwIfNoEntry: false })?.isSymbolicLink() === true);
    }
    prefix = `${prefix}/${part}`;
  }
  return false;
}

// The dep-info prerequisites inside the project and outside build output, by
// the path cargo reported (lexically normalised), not their canonical target,
// so that a symbolic link on the way stays part of the input's identity. A
// path with a ".." after a symbolic link is refused by name.
export function compileReadsFromDepInfo(projectRoot, depInfoText, excludedRoots = []) {
  const reads = new Set();
  const roots = [...new Set(excludedRoots.flatMap((root) => [path.resolve(root), real(root)]))];
  const projectRoots = [...new Set([path.resolve(projectRoot), real(projectRoot)])];
  for (const prerequisite of parseDepInfo(depInfoText)) {
    const lexical = path.resolve(prerequisite);
    const resolved = real(lexical);
    if (roots.some((root) => within(root, lexical) || within(root, resolved))) continue;
    if (linkBeforeParent(prerequisite)) {
      throw new Error(`engine stamp: compile read ${prerequisite} has a ".." after a symbolic link, so its path does not identify the file read`);
    }
    let relative = null;
    for (const root of projectRoots) {
      const candidate = path.relative(root, lexical);
      if (candidate !== "" && !candidate.startsWith("..") && !path.isAbsolute(candidate)) {
        relative = toPosix(candidate);
        break;
      }
    }
    if (relative === null) relative = projectRelative(projectRoot, resolved);
    if (relative !== null && !excluded(relative)) reads.add(relative);
  }
  return [...reads].sort();
}

// The stamp's input set: the static inputs and the compile reads not among them.
export function engineInputs({ projectRoot, crateDirectory, compileReads = [], readManifest = readCrateManifest }) {
  const inputs = staticEngineInputs(projectRoot, crateDirectory, readManifest);
  const known = new Set(inputs);
  const recorded = [...new Set(compileReads)].filter((relative) => !known.has(relative)).sort();
  return { staticInputs: inputs, recorded, all: [...inputs, ...recorded] };
}

// A stamp from input digests already taken (`digestInputs` over an input set's `all`).
export function stampFromDigests({ projectRoot, crateDirectory, digests, recorded, context = [] }) {
  const { hash, input_count } = hashDigests(digests, context);
  return {
    format: STAMP_FORMAT,
    crate: projectRelative(projectRoot, crateDirectory),
    hash,
    input_count,
    compile_reads: recorded
  };
}

// The full stamp: the static inputs, the recorded compile reads and the context.
export function computeEngineStamp({ projectRoot, crateDirectory, compileReads = [], context = [], readManifest = readCrateManifest }) {
  const { all, recorded } = engineInputs({ projectRoot, crateDirectory, compileReads, readManifest });
  return stampFromDigests({ projectRoot, crateDirectory, digests: digestInputs(projectRoot, all), recorded, context });
}

// Why a build's inputs did not hold still, comparing the digests taken before
// cargo ran (`before`, over the input set then known) with the input set the
// build's dep-info gives (`after`) and its digests now (`afterDigests`): empty
// when the build may be stamped with `afterDigests`.
export function inputDrift({ beforeStatic, beforeDigests, after, afterDigests }) {
  const problems = [];
  const discovered = after.recorded.filter((relative) => !beforeDigests.has(relative));
  if (discovered.length > 0) problems.push(`compile reads not known before the build: ${discovered.join(", ")}`);
  const beforeSet = new Set(beforeStatic);
  const afterSet = new Set(after.staticInputs);
  const moved = [...after.staticInputs.filter((relative) => !beforeSet.has(relative)), ...beforeStatic.filter((relative) => !afterSet.has(relative))].sort();
  if (moved.length > 0) problems.push(`source files added or removed during the build: ${moved.join(", ")}`);
  const changed = after.all.filter((relative) => beforeDigests.has(relative) && beforeDigests.get(relative) !== afterDigests.get(relative));
  if (changed.length > 0) problems.push(`inputs changed during the build: ${changed.join(", ")}`);
  return problems;
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

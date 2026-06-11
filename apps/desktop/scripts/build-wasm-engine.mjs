#!/usr/bin/env node
// Builds the wasm32 browser operation engine (DEC-020 / ADR-0001):
// `core/model_operations/operation_applier` compiled with `--features wasm`,
// then bound with the pinned wasm-bindgen CLI into web-target glue under
// `src/services/wasmEngine/__generated__/`. Generated artifacts are not
// committed. Every missing prerequisite fails with the exact remediation
// command — no silent fallback (operation-seam plan §3 T3; ADR-0001).

import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const PINNED_WASM_BINDGEN_VERSION = "0.2.123";

const desktopRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const projectRoot = path.resolve(desktopRoot, "..", "..");
const crateDir = path.join(projectRoot, "core", "model_operations", "operation_applier");
const crateManifest = path.join(crateDir, "Cargo.toml");
const wasmArtifact = path.join(
  crateDir,
  "target",
  "wasm32-unknown-unknown",
  "release",
  "open_pipe_stress_operation_applier.wasm"
);
const outDir = path.join(desktopRoot, "src", "services", "wasmEngine", "__generated__");

function fail(lines) {
  console.error(["", "[build-wasm-engine] FAILED:", ...lines.map((line) => `  ${line}`), ""].join("\n"));
  process.exit(1);
}

function tryRun(command, args) {
  try {
    return {
      ok: true,
      stdout: execFileSync(command, args, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] })
    };
  } catch (error) {
    return { ok: false, stderr: error?.stderr?.toString?.() ?? String(error) };
  }
}

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

// 4. Compile the engine to wasm32 with the wasm feature.
console.log("[build-wasm-engine] cargo build --target wasm32-unknown-unknown --features wasm --release");
const build = tryRun("cargo", [
  "build",
  "--manifest-path",
  crateManifest,
  "--target",
  "wasm32-unknown-unknown",
  "--features",
  "wasm",
  "--release"
]);
if (!build.ok) {
  fail(["`cargo build` for the wasm32 engine failed:", build.stderr]);
}
if (!existsSync(wasmArtifact)) {
  fail([`cargo build succeeded but the expected artifact is missing: ${wasmArtifact}`]);
}

// 5. Generate the web-target JS glue.
console.log(`[build-wasm-engine] wasm-bindgen --target web --out-dir ${path.relative(desktopRoot, outDir)}`);
const bindgen = tryRun(wasmBindgen, ["--target", "web", "--out-dir", outDir, wasmArtifact]);
if (!bindgen.ok) {
  fail(["`wasm-bindgen` glue generation failed:", bindgen.stderr]);
}

console.log(
  `[build-wasm-engine] OK — wasm operation engine generated at ${path.relative(desktopRoot, outDir)} ` +
    `(wasm-bindgen ${bindgenVersion}; artifacts are intentionally not committed)`
);

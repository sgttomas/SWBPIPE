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

import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, renameSync, rmSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const PINNED_WASM_BINDGEN_VERSION = "0.2.123";

const desktopRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const projectRoot = path.resolve(desktopRoot, "..", "..");
const crateDir = path.join(projectRoot, "core", "model_operations", "operation_applier");
const crateManifest = path.join(crateDir, "Cargo.toml");
const outDir = path.join(desktopRoot, "public", "wasm-engine");
// Pre-TP-APP-R2-WASMPKG-001 emission location; removed on every build so a
// stale copy can never be mistaken for the live artifact set.
const legacyOutDir = path.join(desktopRoot, "src", "services", "wasmEngine", "__generated__");

function fail(lines) {
  console.error(["", "[build-wasm-engine] FAILED:", ...lines.map((line) => `  ${line}`), ""].join("\n"));
  process.exit(1);
}

function tryRun(command, args, { cwd } = {}) {
  try {
    return {
      ok: true,
      stdout: execFileSync(command, args, {
        cwd,
        encoding: "utf8",
        env: { ...process.env, CARGO_NET_OFFLINE: "true" },
        stdio: ["ignore", "pipe", "pipe"]
      })
    };
  } catch (error) {
    return { ok: false, stderr: error?.stderr?.toString?.() ?? String(error) };
  }
}

export function resolveWasmArtifact(crateDirectory = crateDir) {
  const metadata = tryRun(
    "cargo",
    ["metadata", "--format-version", "1", "--no-deps"],
    { cwd: crateDirectory }
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
    "open_pipe_stress_operation_applier.wasm"
  );
}

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

// 4. Compile the engine to wasm32 with the wasm feature.
console.log("[build-wasm-engine] cargo build --offline --target wasm32-unknown-unknown --features wasm --release");
const build = tryRun("cargo", [
  "build",
  "--offline",
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
let wasmArtifact;
try {
  wasmArtifact = resolveWasmArtifact();
} catch (error) {
  fail(["Could not resolve Cargo's wasm artifact directory:", error.message]);
}
if (!existsSync(wasmArtifact)) {
  fail([`cargo build succeeded but the expected artifact is missing: ${wasmArtifact}`]);
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

if (existsSync(outDir)) {
  renameSync(outDir, oldDir);
}
renameSync(tmpDir, outDir);
rmSync(oldDir, { recursive: true, force: true });

console.log(
  `[build-wasm-engine] OK — wasm operation engine generated at ${path.relative(desktopRoot, outDir)} ` +
    `(wasm-bindgen ${bindgenVersion}; artifacts are intentionally not committed)`
);
}

const invokedPath = process.argv[1] ? path.resolve(process.argv[1]) : null;
if (invokedPath === fileURLToPath(import.meta.url)) {
  main();
}

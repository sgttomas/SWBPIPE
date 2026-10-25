// The WebAssembly engines' content stamp and the native smoke's rebuild
// decision (`scripts/engineSourceStamp.mjs`), against a temporary project with
// an injected manifest reader: no cargo, no build.
//
//   node --test apps/desktop/native-smoke/engine-stamp.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, realpathSync, rmSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import {
  STAMP_FILENAME,
  STAMP_FORMAT,
  checkEngine,
  compileReadsFromDepInfo,
  computeEngineStamp,
  decideEngine,
  parseDepInfo,
  staticEngineInputs
} from "../scripts/engineSourceStamp.mjs";

const CONTEXT = [["rustc -vV", "rustc 1.0.0 (fake)"]];

// engine (path dep: lib; dev dep: devonly; build script) under a temp project.
function project() {
  const root = realpathSync(mkdtempSync(path.join(tmpdir(), "engine-stamp-")));
  const write = (relative, text) => {
    mkdirSync(path.dirname(path.join(root, relative)), { recursive: true });
    writeFileSync(path.join(root, relative), text);
  };
  write("apps/desktop/scripts/build-wasm-engine.mjs", "// build\n");
  write("apps/desktop/scripts/engineSourceStamp.mjs", "// stamp\n");
  write("core/engine/Cargo.toml", "[package]\nname = \"engine\"\n");
  write("core/engine/Cargo.lock", "# lock\n");
  write("core/engine/build.rs", "fn main() {}\n");
  write("core/engine/src/lib.rs", "mod sub;\n");
  write("core/engine/src/sub/mod.rs", "// sub\n");
  write("core/engine/src/table.json", "{}\n");
  write("core/engine/target/release/engine.wasm", "output\n");
  write("core/engine/src/node_modules/x.js", "output\n");
  write("core/engine/tests/it.rs", "// test only\n");
  write("core/lib/Cargo.toml", "[package]\nname = \"lib\"\n");
  write("core/lib/src/lib.rs", "pub fn f() {}\n");
  write("core/devonly/Cargo.toml", "[package]\nname = \"devonly\"\n");
  write("core/devonly/src/lib.rs", "pub fn g() {}\n");
  write("fixtures/read.json", "{\"a\":1}\n");
  const manifests = {
    [path.join(root, "core/engine/Cargo.toml")]: {
      workspaceRoot: path.join(root, "core/engine"),
      // readCrateManifest drops dev dependencies; the fake does the same.
      pathDependencies: [path.join(root, "core/lib/Cargo.toml")],
      buildScripts: [path.join(root, "core/engine/build.rs")]
    },
    [path.join(root, "core/lib/Cargo.toml")]: { workspaceRoot: path.join(root, "core/lib"), pathDependencies: [], buildScripts: [] }
  };
  const readManifest = (manifest) => {
    const facts = manifests[manifest];
    if (!facts) throw new Error(`unexpected manifest ${manifest}`);
    return facts;
  };
  const crateDirectory = path.join(root, "core/engine");
  const stamp = (overrides = {}) =>
    computeEngineStamp({ projectRoot: root, crateDirectory, compileReads: ["fixtures/read.json"], context: CONTEXT, readManifest, ...overrides });
  return { root, write, readManifest, crateDirectory, stamp, cleanup: () => rmSync(root, { recursive: true, force: true }) };
}

test("the static inputs are the closure's manifests, build scripts, src trees, the lock and the scripts", () => {
  const p = project();
  try {
    assert.deepEqual(staticEngineInputs(p.root, p.crateDirectory, p.readManifest), [
      "apps/desktop/scripts/build-wasm-engine.mjs",
      "apps/desktop/scripts/engineSourceStamp.mjs",
      "core/engine/Cargo.lock",
      "core/engine/Cargo.toml",
      "core/engine/build.rs",
      "core/engine/src/lib.rs",
      "core/engine/src/sub/mod.rs",
      "core/engine/src/table.json",
      "core/lib/Cargo.toml",
      "core/lib/src/lib.rs"
    ]);
    const stamp = p.stamp();
    assert.equal(stamp.format, STAMP_FORMAT);
    assert.equal(stamp.crate, "core/engine");
    assert.equal(stamp.input_count, 11);
    assert.deepEqual(stamp.compile_reads, ["fixtures/read.json"]);
    assert.match(stamp.hash, /^sha256:[0-9a-f]{64}$/);
  } finally {
    p.cleanup();
  }
});

test("same inputs give the same hash; a changed byte, an added or a removed file give another", () => {
  const p = project();
  try {
    const base = p.stamp().hash;
    assert.equal(p.stamp().hash, base);
    assert.equal(p.stamp({ compileReads: ["fixtures/read.json", "fixtures/read.json"] }).hash, base);

    p.write("core/lib/src/lib.rs", "pub fn f() {}\n ");
    const changed = p.stamp().hash;
    assert.notEqual(changed, base);
    p.write("core/lib/src/lib.rs", "pub fn f() {}\n");
    assert.equal(p.stamp().hash, base);

    p.write("core/engine/src/added.rs", "// new\n");
    assert.notEqual(p.stamp().hash, base);
    unlinkSync(path.join(p.root, "core/engine/src/added.rs"));
    assert.equal(p.stamp().hash, base);

    unlinkSync(path.join(p.root, "core/engine/src/table.json"));
    assert.notEqual(p.stamp().hash, base);
    p.write("core/engine/src/table.json", "{}\n");
    assert.equal(p.stamp().hash, base);

    // A recorded compile read: changed, then missing.
    p.write("fixtures/read.json", "{\"a\":2}\n");
    assert.notEqual(p.stamp().hash, base);
    unlinkSync(path.join(p.root, "fixtures/read.json"));
    assert.notEqual(p.stamp().hash, base);
    p.write("fixtures/read.json", "{\"a\":1}\n");
    assert.equal(p.stamp().hash, base);

    // The build script, the lock file and the toolchain context are inputs too.
    p.write("apps/desktop/scripts/build-wasm-engine.mjs", "// build 2\n");
    assert.notEqual(p.stamp().hash, base);
    p.write("apps/desktop/scripts/build-wasm-engine.mjs", "// build\n");
    p.write("core/engine/Cargo.lock", "# lock 2\n");
    assert.notEqual(p.stamp().hash, base);
    p.write("core/engine/Cargo.lock", "# lock\n");
    assert.notEqual(p.stamp({ context: [["rustc -vV", "rustc 1.0.1 (fake)"]] }).hash, base);
    assert.equal(p.stamp().hash, base);
  } finally {
    p.cleanup();
  }
});

test("build output, tests and dev dependencies are not inputs", () => {
  const p = project();
  try {
    const base = p.stamp().hash;
    p.write("core/engine/target/release/engine.wasm", "rebuilt\n");
    p.write("core/engine/target/new.rs", "// output\n");
    p.write("core/engine/src/node_modules/x.js", "changed\n");
    p.write("core/engine/tests/it.rs", "// changed test\n");
    p.write("core/devonly/src/lib.rs", "pub fn g() { }\n");
    assert.equal(p.stamp().hash, base);
  } finally {
    p.cleanup();
  }
});

test("dep-info prerequisites become compile reads inside the project and outside build output", () => {
  const p = project();
  try {
    const target = path.join(p.root, "core/engine/target");
    p.write("fixtures/with space.json", "{}\n");
    const text =
      `${target}/wasm32-unknown-unknown/release/engine.wasm: ${p.root}/core/engine/src/lib.rs \\\n` +
      ` ${p.root}/core/engine/src/../../../fixtures/read.json ${p.root}/fixtures/with\\ space.json` +
      ` ${target}/release/build/engine-1/out/generated.rs /elsewhere/registry/src/lib.rs\n\n` +
      `${p.root}/core/engine/src/lib.rs:\n`;
    assert.equal(parseDepInfo(text).length, 5);
    assert.deepEqual(compileReadsFromDepInfo(p.root, text, [target]), [
      "core/engine/src/lib.rs",
      "fixtures/read.json",
      "fixtures/with space.json"
    ]);
    // Files already in the static inputs are not recorded twice.
    const stamp = p.stamp({ compileReads: compileReadsFromDepInfo(p.root, text, [target]) });
    assert.deepEqual(stamp.compile_reads, ["fixtures/read.json", "fixtures/with space.json"]);
  } finally {
    p.cleanup();
  }
});

test("decideEngine: missing folder, missing, unreadable, foreign or mismatched stamp rebuild; a match is current", () => {
  const hash = `sha256:${"a".repeat(64)}`;
  const text = (value) => JSON.stringify({ format: STAMP_FORMAT, hash, input_count: 1, compile_reads: [], ...value });
  assert.deepEqual(decideEngine({ folderPresent: false, stampText: text(), currentHash: hash }), { action: "rebuild", reason: "engine folder missing" });
  assert.deepEqual(decideEngine({ folderPresent: true, stampText: null, currentHash: hash }), { action: "rebuild", reason: "stamp file missing" });
  assert.deepEqual(decideEngine({ folderPresent: true, stampText: "{ not json", currentHash: hash }), { action: "rebuild", reason: "stamp file unreadable" });
  assert.deepEqual(decideEngine({ folderPresent: true, stampText: text({ compile_reads: ["../outside"] }), currentHash: hash }), { action: "rebuild", reason: "stamp file unreadable" });
  assert.deepEqual(decideEngine({ folderPresent: true, stampText: text({ format: "other.v0" }), currentHash: hash }), {
    action: "rebuild",
    reason: `stamp format is not ${STAMP_FORMAT}`
  });
  const other = `sha256:${"b".repeat(64)}`;
  assert.deepEqual(decideEngine({ folderPresent: true, stampText: text(), currentHash: other }), { action: "rebuild", reason: `inputs changed since stamp ${hash}` });
  assert.deepEqual(decideEngine({ folderPresent: true, stampText: text(), currentHash: hash }), { action: "current", reason: null });
});

test("checkEngine recomputes from the sources and the stamp's recorded compile reads", () => {
  const p = project();
  try {
    const outDir = path.join(p.root, "apps/desktop/public/engine");
    const check = () => checkEngine({ projectRoot: p.root, crateDirectory: p.crateDirectory, outDir, context: CONTEXT, readManifest: p.readManifest });
    assert.equal(check().reason, "engine folder missing");
    mkdirSync(outDir, { recursive: true });
    assert.equal(check().reason, "stamp file missing");
    const stamp = p.stamp();
    writeFileSync(path.join(outDir, STAMP_FILENAME), JSON.stringify({ engine: "engine", ...stamp }));
    assert.deepEqual([check().action, check().hash], ["current", stamp.hash]);
    // Only the stamp names the compile read, and a change to it is seen.
    p.write("fixtures/read.json", "{\"a\":3}\n");
    assert.deepEqual([check().action, check().reason], ["rebuild", `inputs changed since stamp ${stamp.hash}`]);
  } finally {
    p.cleanup();
  }
});

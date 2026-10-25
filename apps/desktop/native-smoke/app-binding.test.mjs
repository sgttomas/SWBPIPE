// The native smoke's app-binary binding and its --skip-build plan
// (`scripts/appBinaryBinding.mjs`), against a temporary dist/ and binary: no
// cargo, no build.
//
//   node --test apps/desktop/native-smoke/app-binding.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { BINDING_FORMAT, bindingRecord, checkBinding, fileIdentity, planAppBuild, treeDigest } from "../scripts/appBinaryBinding.mjs";

const BUILD = { command: ["cargo", "build", "--features", "native-smoke"], tauri_config: "{}" };
const hash = (letter) => `sha256:${letter.repeat(64)}`;

function workspace() {
  const root = realpathSync(mkdtempSync(path.join(tmpdir(), "app-binding-")));
  const write = (relative, text) => {
    mkdirSync(path.dirname(path.join(root, relative)), { recursive: true });
    writeFileSync(path.join(root, relative), text);
  };
  write("dist/index.html", "<html>1</html>\n");
  write("dist/wasm-engine/source-stamp.json", `{"hash":"${hash("a")}"}\n`);
  write("target/debug/app", "binary 1\n");
  const dist = path.join(root, "dist");
  const binary = path.join(root, "target/debug/app");
  const state = (engines) => ({ engines, dist: treeDigest(dist), build: BUILD, binary: fileIdentity(binary) });
  return { root, write, state, cleanup: () => rmSync(root, { recursive: true, force: true }) };
}

// The run.mjs sequence under --skip-build once engines and dist/ copies are current.
const skipPlan = (bindingText, current) => planAppBuild({ skipBuild: true, staleEngines: [], distStale: [], binding: checkBinding(bindingText, current) });

test("the binding record names the engine stamps, the dist/ digest, the build and the binary", () => {
  const w = workspace();
  try {
    const record = bindingRecord(w.state({ "wasm-engine": hash("a") }));
    assert.equal(record.format, BINDING_FORMAT);
    assert.deepEqual(record.engines, { "wasm-engine": hash("a") });
    assert.match(record.dist, /^sha256:[0-9a-f]{64}$/);
    assert.deepEqual(record.binary, fileIdentity(path.join(w.root, "target/debug/app")));
    assert.equal(record.binary.size, 9);
    const current = skipPlan(JSON.stringify(record), w.state({ "wasm-engine": hash("a") }));
    assert.deepEqual([current.frontend, current.app], [false, false]);
    assert.match(current.log, /--skip-build applies/);
  } finally {
    w.cleanup();
  }
});

test("Codex r1: engines and dist/ rebuilt, cargo failed: a retry with --skip-build rebuilds the app", () => {
  const w = workspace();
  try {
    // A good app build bound to engine stamp a.
    const record = JSON.stringify(bindingRecord(w.state({ "wasm-engine": hash("a") })));
    // build:wasm and npm run build succeed (engine stamp b, a new dist/), then
    // cargo fails: the old binary and its old record remain.
    w.write("dist/wasm-engine/source-stamp.json", `{"hash":"${hash("b")}"}\n`);
    w.write("dist/index.html", "<html>2</html>\n");
    const retry = skipPlan(record, w.state({ "wasm-engine": hash("b") }));
    assert.deepEqual([retry.frontend, retry.app], [false, true]);
    assert.match(retry.log, /--skip-build overridden for the app \(the engine stamps differ from those the app binary embedded\)/);
    // run.mjs removes the record before cargo runs, so the failure leaves none.
    const none = skipPlan(null, w.state({ "wasm-engine": hash("b") }));
    assert.deepEqual([none.frontend, none.app], [false, true]);
    assert.match(none.log, /no app binding record/);
  } finally {
    w.cleanup();
  }
});

test("a frontend rebuilt on its own, a replaced or missing binary or another build command rebuild the app", () => {
  const w = workspace();
  try {
    const engines = { "wasm-engine": hash("a") };
    const record = JSON.stringify(bindingRecord(w.state(engines)));
    w.write("dist/index.html", "<html>rebuilt</html>\n");
    assert.match(skipPlan(record, w.state(engines)).log, /dist\/ differs from the copy the app binary embedded/);
    w.write("dist/index.html", "<html>1</html>\n");
    assert.equal(skipPlan(record, w.state(engines)).app, false);

    w.write("target/debug/app", "binary 2\n");
    const replaced = skipPlan(record, w.state(engines));
    assert.equal(replaced.app, true);
    assert.match(replaced.log, /the app binary changed since its binding was written/);
    rmSync(path.join(w.root, "target/debug/app"));
    assert.match(skipPlan(record, w.state(engines)).log, /app binary missing/);
    w.write("target/debug/app", "binary 1\n");
    assert.equal(skipPlan(record, w.state(engines)).app, false);

    const other = { ...w.state(engines), build: { ...BUILD, tauri_config: "{\"x\":1}" } };
    assert.match(skipPlan(record, other).log, /the app build command differs/);
    assert.match(skipPlan("{ nope", w.state(engines)).log, /unreadable/);
  } finally {
    w.cleanup();
  }
});

test("without --skip-build both are built; a rebuilt engine or a stale dist/ copy rebuilds frontend and app", () => {
  const binding = { match: true, reason: null };
  assert.deepEqual(planAppBuild({ skipBuild: false, staleEngines: [], distStale: [], binding }), { frontend: true, app: true, log: null });
  const rebuilt = planAppBuild({ skipBuild: true, staleEngines: ["wasm-engine"], distStale: [], binding });
  assert.deepEqual([rebuilt.frontend, rebuilt.app], [true, true]);
  assert.match(rebuilt.log, /wasm-engine rebuilt/);
  const dist = planAppBuild({ skipBuild: true, staleEngines: [], distStale: [{ name: "self-weight-engine", reason: "stamp file missing" }], binding });
  assert.deepEqual([dist.frontend, dist.app], [true, true]);
  assert.match(dist.log, /dist\/self-weight-engine: stamp file missing/);
});

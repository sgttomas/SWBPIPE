// The native smoke's frontend and app builds and the app-binary binding
// (`scripts/appBinaryBinding.mjs`), against a temporary project with a fake
// frontend build and a fake cargo: no npm, no cargo.
//
//   node --test apps/desktop/native-smoke/app-binding.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, statSync, symlinkSync, unlinkSync, utimesSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { tmpdir } from "node:os";
import path from "node:path";
import { BINDING_FORMAT, bindingPathFor, fileIdentity, frontendInputs, launchCopiesDirectory, planAppBuild, prepareSmokeApp, privateLaunchCopy } from "../scripts/appBinaryBinding.mjs";
import { STAMP_FORMAT } from "../scripts/engineSourceStamp.mjs";

const BUILD = { command: ["cargo", "build", "--message-format=json-render-diagnostics", "--features", "native-smoke"], tauri_config: "{}" };
const CONTEXT = [["rustc -vV", "rustc 1.0.0 (fake)"]];
const hash = (letter) => `sha256:${letter.repeat(64)}`;
const stampText = (letter) => JSON.stringify({ format: STAMP_FORMAT, hash: hash(letter), input_count: 1, compile_reads: [] });

// A project with a frontend (one source importing a project fixture), an app
// crate with a path dependency, and fakes for the frontend build and cargo.
// The fake cargo writes `<CARGO_TARGET_DIR>/debug/app` from the app's main.rs
// and dist/index.html, keeps it while its dep-info is newer than both (cargo's
// freshness by modification time), and reports it in a compiler-artifact
// message. `cargo.mode`: "elsewhere" builds and reports another directory;
// "silent" reports nothing; "touch-dist" edits dist/ while it builds; "fail" fails.
// Like run.mjs, the fake runs with `cargo.inherited` (the environment the smoke
// inherits) under the env it is given, and keeps an intermediate in the build
// directory (CARGO_BUILD_BUILD_DIR, else the target directory), which clean removes.
function workspace() {
  const root = realpathSync(mkdtempSync(path.join(tmpdir(), "app-binding-")));
  const write = (relative, text) => {
    mkdirSync(path.dirname(path.join(root, relative)), { recursive: true });
    writeFileSync(path.join(root, relative), text);
  };
  const read = (relative) => readFileSync(path.join(root, relative), "utf8");
  write("package.json", "{}\n");
  write("package-lock.json", "{}\n");
  write("fixtures/data.json", "{\"d\":1}\n");
  write("fixtures/schema.json", "{\"s\":1}\n");
  write("apps/desktop/package.json", "{}\n");
  write("apps/desktop/index.html", "<script src=\"/src/main.ts\"></script>\n");
  write("apps/desktop/vite.config.ts", "export default {};\n");
  write("apps/desktop/tsconfig.tsbuildinfo", "output\n");
  write("apps/desktop/src/main.ts", "import data from \"../../../fixtures/data.json?raw\";\nimport { x } from \"./x\";\n");
  write("apps/desktop/src/x.ts", "export const x = 1;\n");
  write("apps/desktop/public/wasm-engine/source-stamp.json", stampText("a"));
  write("apps/desktop/src-tauri/Cargo.toml", "[package]\nname = \"app\"\n");
  write("apps/desktop/src-tauri/Cargo.lock", "# lock\n");
  write("apps/desktop/src-tauri/build.rs", "fn main() {}\n");
  write("apps/desktop/src-tauri/tauri.conf.json", "{}\n");
  write("apps/desktop/src-tauri/icons/icon.png", "png\n");
  write("apps/desktop/src-tauri/src/main.rs", "fn main() {}\n");
  write("core/lib/Cargo.toml", "[package]\nname = \"lib\"\n");
  write("core/lib/src/lib.rs", "pub fn f() {}\n");
  const desktopRoot = path.join(root, "apps/desktop");
  const tauriDir = path.join(desktopRoot, "src-tauri");
  const manifests = {
    [path.join(tauriDir, "Cargo.toml")]: { name: "app", workspaceRoot: tauriDir, pathDependencies: [path.join(root, "core/lib/Cargo.toml")], buildScripts: [] },
    [path.join(root, "core/lib/Cargo.toml")]: { name: "lib", workspaceRoot: path.join(root, "core/lib"), pathDependencies: [], buildScripts: [] }
  };
  const readManifest = (manifest) => {
    if (!manifests[manifest]) throw new Error(`unexpected manifest ${manifest}`);
    return manifests[manifest];
  };
  const calls = [];
  const cargo = { mode: "ok", inherited: {}, envs: [] };
  let engine = "a";
  const runFrontend = () => {
    calls.push("frontend");
    write("apps/desktop/dist/index.html", `built from ${read("apps/desktop/src/main.ts").length} ${read("fixtures/data.json")}`);
    write("apps/desktop/dist/wasm-engine/source-stamp.json", read("apps/desktop/public/wasm-engine/source-stamp.json"));
  };
  const runCargo = (command, env) => {
    calls.push(command.slice(0, 2).join(" ") + (command[1] === "clean" ? ` ${command.filter((arg, index) => command[index - 1] === "-p").join(",")}` : ""));
    cargo.envs.push(env);
    const merged = { ...cargo.inherited, ...env };
    const target = cargo.mode === "elsewhere" ? path.join(root, "elsewhere") : merged.CARGO_TARGET_DIR;
    const exe = path.join(target, "debug", "app");
    const intermediate = path.join(merged.CARGO_BUILD_BUILD_DIR ?? target, "debug", "deps", "app-intermediate");
    if (command[1] === "clean") {
      rmSync(exe, { force: true });
      rmSync(`${exe}.d`, { force: true });
      rmSync(intermediate, { force: true });
      return "";
    }
    if (cargo.mode === "fail") throw new Error("cargo build failed");
    mkdirSync(path.dirname(exe), { recursive: true });
    mkdirSync(path.dirname(intermediate), { recursive: true });
    writeFileSync(intermediate, "intermediate\n");
    const main = path.join(tauriDir, "src/main.rs");
    const dist = path.join(desktopRoot, "dist/index.html");
    const fresh = existsSync(exe) && existsSync(`${exe}.d`) && [main, dist].every((input) => statSync(input).mtimeMs <= statSync(`${exe}.d`).mtimeMs);
    if (!fresh) {
      writeFileSync(exe, `binary ${read("apps/desktop/src-tauri/src/main.rs")} ${read("apps/desktop/dist/index.html")}`);
      writeFileSync(`${exe}.d`, `${exe}: ${main} ${root}/core/lib/src/lib.rs ${root}/fixtures/schema.json ${tauriDir}/../dist/index.html ${target}/debug/build/out.rs\n`);
    }
    if (cargo.mode === "touch-dist") write("apps/desktop/dist/index.html", "rebuilt meanwhile\n");
    const artifact = { reason: "compiler-artifact", manifest_path: path.join(tauriDir, "Cargo.toml"), target: { name: "app", kind: ["bin"] }, executable: exe, fresh };
    return [JSON.stringify({ reason: "build-script-executed" }), cargo.mode === "silent" ? "" : JSON.stringify(artifact), JSON.stringify({ reason: "build-finished", success: true })].join("\n");
  };
  const logs = [];
  const prepare = (skipBuild, extra = {}) =>
    prepareSmokeApp({
      projectRoot: root,
      desktopRoot,
      tauriDir,
      binName: "app",
      skipBuild,
      engineFolders: ["wasm-engine"],
      engineHashes: () => ({ "wasm-engine": hash(engine) }),
      build: BUILD,
      context: CONTEXT,
      runFrontend,
      runCargo,
      log: (line) => logs.push(line),
      readManifest,
      ...extra
    });
  const setEngine = (letter) => {
    engine = letter;
    write("apps/desktop/public/wasm-engine/source-stamp.json", stampText(letter));
  };
  const take = () => {
    const taken = { calls: calls.splice(0), logs: logs.splice(0) };
    return taken;
  };
  const record = () => (existsSync(bindingPathFor(desktopRoot)) ? JSON.parse(readFileSync(bindingPathFor(desktopRoot), "utf8")) : null);
  return { root, write, read, desktopRoot, tauriDir, cargo, runCargo, prepare, take, setEngine, record, cleanup: () => rmSync(root, { recursive: true, force: true }) };
}

test("the binding record names the engines, frontend, dist/, app sources, build and the binary cargo reported", async () => {
  const w = workspace();
  try {
    const app = await w.prepare(false);
    const exe = path.join(w.tauriDir, "target/debug/app");
    assert.deepEqual(app, { binary: exe, identity: fileIdentity(exe) });
    const { calls, logs } = w.take();
    // No record yet: the app's own packages are cleaned before the build. With
    // no earlier dep-info either, the build reveals a compile read, so a second
    // (clean) build is the one bound.
    assert.deepEqual(calls, ["frontend", "cargo clean app,lib", "cargo build", "cargo clean app,lib", "cargo build"]);
    assert.match(logs.join("\n"), /cleaning the app's own packages in apps\/desktop\/src-tauri\/target \(no app binding record/);
    assert.match(logs.join("\n"), /app build not bound \(compile reads not known before the build: fixtures\/schema\.json\); cargo builds it again/);
    const record = w.record();
    assert.equal(record.format, BINDING_FORMAT);
    assert.deepEqual(record.engines, { "wasm-engine": hash("a") });
    assert.match(record.frontend, /^sha256:[0-9a-f]{64}$/);
    assert.match(record.dist, /^sha256:[0-9a-f]{64}$/);
    assert.match(record.app_sources.hash, /^sha256:[0-9a-f]{64}$/);
    // The dep-info read outside the static inputs, dist/ and the target directory.
    assert.deepEqual(record.app_sources.compile_reads, ["fixtures/schema.json"]);
    assert.deepEqual(record.build, BUILD);
    assert.deepEqual(record.binary, { path: exe, ...fileIdentity(exe) });
    // Then --skip-build changes nothing and launches the bound binary.
    assert.deepEqual(await w.prepare(true), app);
    const skipped = w.take();
    assert.deepEqual(skipped.calls, []);
    assert.match(skipped.logs.join("\n"), /--skip-build applies/);
  } finally {
    w.cleanup();
  }
});

test("the frontend inputs: src/, the app's config files, public/ other than engine folders, the workspace lock and fixtures the sources import", () => {
  const w = workspace();
  try {
    w.write("apps/desktop/public/logo.svg", "<svg/>\n");
    assert.deepEqual(frontendInputs(w.root, w.desktopRoot, ["wasm-engine"]), [
      "apps/desktop/index.html",
      "apps/desktop/package.json",
      "apps/desktop/public/logo.svg",
      "apps/desktop/src/main.ts",
      "apps/desktop/src/x.ts",
      "apps/desktop/vite.config.ts",
      "fixtures/data.json",
      "package-lock.json",
      "package.json"
    ]);
    w.write("apps/desktop/src/y.ts", "import z from \"../../../fixtures/missing.json\";\n");
    assert.throws(() => frontendInputs(w.root, w.desktopRoot, ["wasm-engine"]), /src\/y\.ts imports \.\.\/\.\.\/\.\.\/fixtures\/missing\.json, which names no file/);
  } finally {
    w.cleanup();
  }
});

test("Codex r2: an old executable at the fixed path is never bound or launched when cargo built elsewhere or reported nothing", async () => {
  const w = workspace();
  try {
    // The executable an earlier build left at the fixed path.
    w.write("apps/desktop/src-tauri/target/debug/app", "old binary\n");
    w.cargo.mode = "elsewhere";
    await assert.rejects(() => w.prepare(false), /cargo reported the app executable at .*\/elsewhere\/debug\/app, outside .*src-tauri\/target; nothing is bound or launched/);
    assert.equal(w.record(), null);
    w.cargo.mode = "silent";
    await assert.rejects(() => w.prepare(false), /cargo reported no compiler-artifact executable for app .*; nothing is bound or launched/);
    assert.equal(w.record(), null);
    // Under --skip-build too: no record, so cargo builds and its report decides.
    await assert.rejects(() => w.prepare(true), /cargo reported no compiler-artifact executable/);
    assert.equal(w.record(), null);
  } finally {
    w.cleanup();
  }
});

test("Codex r2: dist/ changing while cargo builds the app aborts the run: no binding, nothing launched", async () => {
  const w = workspace();
  try {
    w.cargo.mode = "touch-dist";
    await assert.rejects(() => w.prepare(false), /dist\/ changed while cargo built the app, so what the binary embeds is unknown; nothing is bound or launched/);
    assert.equal(w.record(), null);
  } finally {
    w.cleanup();
  }
});

test("Codex r2: under --skip-build a frontend .ts edit rebuilds frontend and app; an app .rs edit, even one keeping its time, rebuilds the app clean", async () => {
  const w = workspace();
  try {
    await w.prepare(false);
    w.take();
    const binary = () => w.read("apps/desktop/src-tauri/target/debug/app");

    w.write("apps/desktop/src/x.ts", "export const x = 2;\n");
    await w.prepare(true);
    let step = w.take();
    assert.deepEqual(step.calls, ["frontend", "cargo clean app,lib", "cargo build"]);
    assert.match(step.logs[0], /--skip-build overridden \(the frontend sources differ from those dist\/ was built from\); the app embeds dist\/, so the frontend and app are rebuilt/);

    // A fixture the frontend imports from outside src/ is a frontend input too.
    w.write("fixtures/data.json", "{\"d\":2}\n");
    await w.prepare(true);
    assert.deepEqual(w.take().calls, ["frontend", "cargo clean app,lib", "cargo build"]);

    w.write("apps/desktop/src-tauri/src/main.rs", "fn main() { two() }\n");
    await w.prepare(true);
    step = w.take();
    assert.deepEqual(step.calls, ["cargo clean app,lib", "cargo build"]);
    assert.match(step.logs[0], /--skip-build overridden for the app \(the app's Rust sources differ from those the binary was built from\); cargo builds it from the current dist\//);
    assert.match(binary(), /two\(\)/);

    // New bytes, old time: cargo alone would keep the old binary; the clean does not.
    const main = path.join(w.tauriDir, "src/main.rs");
    const old = statSync(main);
    writeFileSync(main, "fn main() { three() }\n");
    utimesSync(main, old.atime, old.mtime);
    await w.prepare(true);
    assert.deepEqual(w.take().calls, ["cargo clean app,lib", "cargo build"]);
    assert.match(binary(), /three\(\)/);

    // A compile read only the dep-info names, and the toolchain context.
    w.write("fixtures/schema.json", "{\"s\":2}\n");
    await w.prepare(true);
    assert.deepEqual(w.take().calls, ["cargo clean app,lib", "cargo build"]);
    await w.prepare(true, { context: [["rustc -vV", "rustc 1.0.1 (fake)"]] });
    assert.deepEqual(w.take().calls, ["cargo clean app,lib", "cargo build"]);
    await w.prepare(true, { context: [["rustc -vV", "rustc 1.0.1 (fake)"]] });
    assert.deepEqual(w.take().calls, []);
  } finally {
    w.cleanup();
  }
});

test("Codex r1: engines and dist/ rebuilt, cargo failed: a retry with --skip-build rebuilds frontend and app", async () => {
  const w = workspace();
  try {
    await w.prepare(false);
    w.take();
    // build:wasm rebuilt the engine (stamp b) and the frontend build ran, then cargo failed.
    w.setEngine("b");
    w.cargo.mode = "fail";
    await assert.rejects(() => w.prepare(false), /cargo build failed/);
    // The record is removed before cargo runs, so the failure leaves none.
    assert.equal(w.record(), null);
    w.cargo.mode = "ok";
    w.take();
    await w.prepare(true);
    const retry = w.take();
    // The clean before the failed build removed the last dep-info, so, with no
    // record either, the retry's first build reveals a compile read and a second
    // (clean) build is the one bound.
    assert.deepEqual(retry.calls, ["frontend", "cargo clean app,lib", "cargo build", "cargo clean app,lib", "cargo build"]);
    assert.match(retry.logs[0], /--skip-build overridden \(no app binding record/);
    assert.deepEqual(w.record().engines, { "wasm-engine": hash("b") });
  } finally {
    w.cleanup();
  }
});

test("a replaced or missing binary, other engine stamps or another build command rebuild the app; a stale dist/ copy the frontend too", async () => {
  const w = workspace();
  try {
    await w.prepare(false);
    w.take();
    const exe = path.join(w.tauriDir, "target/debug/app");
    writeFileSync(exe, "replaced\n");
    await w.prepare(true);
    let step = w.take();
    assert.deepEqual(step.calls, ["cargo clean app,lib", "cargo build"]);
    assert.match(step.logs[0], /the app binary changed since its binding was written/);
    rmSync(exe);
    await w.prepare(true);
    assert.match(w.take().logs[0], /app binary missing/);
    await w.prepare(true, { build: { ...BUILD, tauri_config: "{\"x\":1}" } });
    assert.match(w.take().logs[0], /the app build command differs/);
    await w.prepare(true);
    assert.match(w.take().logs[0], /the app build command differs/);
    // The engine's dist/ copy no longer matches the engine stamp.
    w.setEngine("c");
    await w.prepare(true);
    step = w.take();
    assert.deepEqual(step.calls, ["frontend", "cargo clean app,lib", "cargo build"]);
    assert.match(step.logs[0], /dist\/wasm-engine: inputs changed since stamp/);
    // An unreadable record rebuilds both.
    writeFileSync(bindingPathFor(w.desktopRoot), "{ nope");
    await w.prepare(true);
    assert.match(w.take().logs[0], /app binding record unreadable/);
  } finally {
    w.cleanup();
  }
});

test("the frontend inputs moving while it builds abort the run", async () => {
  const w = workspace();
  try {
    await assert.rejects(
      () => w.prepare(false, { runFrontend: () => w.write("apps/desktop/src/x.ts", "export const x = 9;\n") }),
      /the frontend inputs moved while it built \(inputs changed during the build: apps\/desktop\/src\/x\.ts\); nothing is bound or launched/
    );
  } finally {
    w.cleanup();
  }
});

test("without --skip-build both are built; a rebuilt engine rebuilds frontend and app", () => {
  assert.deepEqual(planAppBuild({ skipBuild: false }), { frontend: true, app: true, log: null });
  const rebuilt = planAppBuild({ skipBuild: true, staleEngines: ["wasm-engine"] });
  assert.deepEqual([rebuilt.frontend, rebuilt.app], [true, true]);
  assert.match(rebuilt.log, /wasm-engine rebuilt/);
});

test("Codex r3: an inherited CARGO_BUILD_BUILD_DIR or CARGO_TARGET_DIR neither moves the app's build nor lets cargo clean another tree", async () => {
  const w = workspace();
  try {
    w.write("other-worktree/build/debug/deps/app-intermediate", "other worktree\n");
    w.cargo.inherited = { CARGO_BUILD_BUILD_DIR: path.join(w.root, "other-worktree/build"), CARGO_TARGET_DIR: path.join(w.root, "other-worktree/target") };
    await w.prepare(false);
    assert.deepEqual(w.take().calls, ["frontend", "cargo clean app,lib", "cargo build", "cargo clean app,lib", "cargo build"]);
    assert.equal(w.read("other-worktree/build/debug/deps/app-intermediate"), "other worktree\n");
    assert.ok(!existsSync(path.join(w.root, "other-worktree/target")));
    assert.equal(w.read("apps/desktop/src-tauri/target/debug/deps/app-intermediate"), "intermediate\n");
    const target = path.join(w.tauriDir, "target");
    for (const env of w.cargo.envs) {
      assert.deepEqual([env.CARGO_TARGET_DIR, env.CARGO_BUILD_TARGET_DIR, env.CARGO_BUILD_BUILD_DIR], [target, target, target]);
    }
  } finally {
    w.cleanup();
  }
});

test("Codex r3: with target/debug, or target/debug/deps, a link to another worktree's output, cargo neither cleans nor builds there", async () => {
  const w = workspace();
  try {
    const target = path.join(w.tauriDir, "target");
    w.write("other-worktree/target/debug/app", "other worktree's binary\n");
    w.write("other-worktree/target/debug/deps/app-intermediate", "other worktree's intermediate\n");
    mkdirSync(target);
    symlinkSync(path.join(w.root, "other-worktree/target/debug"), path.join(target, "debug"));
    const refused = async (where) => {
      await assert.rejects(() => w.prepare(false), new RegExp(`native smoke: refusing to let cargo build or clean in .*src-tauri/target: .*src-tauri/target/${where} is a symbolic link`));
      assert.ok(!w.take().calls.some((call) => call.startsWith("cargo")), "no cargo ran");
      assert.equal(w.record(), null);
      assert.equal(w.read("other-worktree/target/debug/app"), "other worktree's binary\n");
      assert.equal(w.read("other-worktree/target/debug/deps/app-intermediate"), "other worktree's intermediate\n");
    };
    await refused("debug");
    unlinkSync(path.join(target, "debug"));
    mkdirSync(path.join(target, "debug"));
    symlinkSync(path.join(w.root, "other-worktree/target/debug/deps"), path.join(target, "debug/deps"));
    await refused("debug/deps");
  } finally {
    w.cleanup();
  }
});

test("Codex r4: an app binary an ordinary cargo build changed after the binding is never rebound to the restored sources", async () => {
  const w = workspace();
  try {
    const main = path.join(w.tauriDir, "src/main.rs");
    const target = path.join(w.tauriDir, "target");
    const binary = () => w.read("apps/desktop/src-tauri/target/debug/app");
    // Revision A bound.
    await w.prepare(false);
    w.take();
    assert.match(binary(), /^binary fn main\(\) \{\}\n/);
    const a = statSync(main);
    // Revision B built by an ordinary cargo build in the same target.
    writeFileSync(main, "fn main() { b() }\n");
    w.runCargo(["cargo", "build"], { CARGO_TARGET_DIR: target });
    assert.match(binary(), /b\(\)/);
    // A's sources restored with their old times: they match the record, and cargo would take B as fresh.
    writeFileSync(main, "fn main() {}\n");
    utimesSync(main, a.atime, a.mtime);
    w.take();
    const app = await w.prepare(true);
    const step = w.take();
    assert.match(step.logs[0], /the app binary changed since its binding was written/);
    // The executable bound and launched is A's.
    assert.match(binary(), /^binary fn main\(\) \{\}\n/);
    assert.deepEqual(step.calls, ["cargo clean app,lib", "cargo build"]);
    assert.deepEqual(app.identity, fileIdentity(app.binary));
    assert.deepEqual(w.record().binary, { path: app.binary, ...fileIdentity(app.binary) });
  } finally {
    w.cleanup();
  }
});

test("Codex r6: the launch runs a private copy of the bound binary, checked by hash; a copy that is not the bound binary is refused and removed", async () => {
  const w = workspace();
  try {
    const app = await w.prepare(false);
    w.take();
    // The bound executable, made runnable: it reports where it runs from.
    writeFileSync(app.binary, "#!/bin/sh\necho \"running $0\"\n");
    chmodSync(app.binary, 0o755);
    const bound = fileIdentity(app.binary);
    const copy = privateLaunchCopy({ desktopRoot: w.desktopRoot, binary: app.binary, identity: bound, token: "t1" });
    assert.equal(copy.directory, path.join(launchCopiesDirectory(w.desktopRoot), "run-t1"));
    assert.equal(copy.executable, path.join(copy.directory, "app"));
    assert.deepEqual(copy.identity, bound);
    assert.equal(statSync(copy.directory).mode & 0o777, 0o700);
    // The copy runs on its own, and a later rebuild of the bound path does not change it.
    assert.equal(execFileSync(copy.executable, { encoding: "utf8" }).trim(), `running ${copy.executable}`);
    writeFileSync(app.binary, "#!/bin/sh\necho rebuilt\n");
    assert.deepEqual(fileIdentity(copy.executable), bound);
    // The directory is the run's alone: the same token is never reused.
    assert.throws(() => privateLaunchCopy({ desktopRoot: w.desktopRoot, binary: app.binary, identity: fileIdentity(app.binary), token: "t1" }), /EEXIST/);
    copy.remove();
    assert.ok(!existsSync(copy.directory), "the copy is removed when the run ends");
    // The binary replaced after the binding check: the copy's hash is not the bound one.
    assert.throws(
      () => privateLaunchCopy({ desktopRoot: w.desktopRoot, binary: app.binary, identity: bound, token: "t2" }),
      new RegExp(`native smoke: the copy of .*src-tauri/target/debug/app has SHA-256 ${fileIdentity(app.binary).sha256}, not the bound binary's ${bound.sha256}; nothing is launched`)
    );
    assert.deepEqual(readdirSync(launchCopiesDirectory(w.desktopRoot)).filter((name) => name.startsWith("run-")), [], "a refused copy is removed");
    // A link where the copies are kept is refused.
    rmSync(launchCopiesDirectory(w.desktopRoot), { recursive: true, force: true });
    mkdirSync(path.join(w.root, "elsewhere"));
    symlinkSync(path.join(w.root, "elsewhere"), launchCopiesDirectory(w.desktopRoot));
    assert.throws(() => privateLaunchCopy({ desktopRoot: w.desktopRoot, binary: app.binary, identity: fileIdentity(app.binary), token: "t3" }), /refusing to copy the app into .*target\/native-smoke: .*target\/native-smoke is a symbolic link; nothing is launched/);
    assert.deepEqual(readdirSync(path.join(w.root, "elsewhere")), []);
  } finally {
    w.cleanup();
  }
});

// The WebAssembly engines' content stamp and the native smoke's rebuild
// decision (`scripts/engineSourceStamp.mjs`), against a temporary project with
// an injected manifest reader: no cargo, no build.
//
//   node --test apps/desktop/native-smoke/engine-stamp.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import {
  STAMP_FILENAME,
  STAMP_FORMAT,
  checkEngine,
  compileReadsFromDepInfo,
  computeEngineStamp,
  decideEngine,
  digestInputs,
  engineInputs,
  engineStampContext,
  inputDrift,
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

test("a compile read through a symbolic link keeps its link: retargeting it makes the stamp stale", () => {
  const p = project();
  try {
    p.write("assets/a.json", "{\"v\":\"a\"}\n");
    p.write("assets/b.json", "{\"v\":\"b\"}\n");
    p.write("assets/same-as-a.json", "{\"v\":\"a\"}\n");
    symlinkSync("a.json", path.join(p.root, "assets/active.json"));
    const target = path.join(p.root, "core/engine/target");
    const reads = compileReadsFromDepInfo(p.root, `${target}/engine.wasm: ${p.root}/assets/active.json\n`, [target]);
    // The path cargo reported, not its canonical target.
    assert.deepEqual(reads, ["assets/active.json"]);
    const base = p.stamp({ compileReads: reads }).hash;
    const retarget = (to) => {
      unlinkSync(path.join(p.root, "assets/active.json"));
      symlinkSync(to, path.join(p.root, "assets/active.json"));
    };
    retarget("b.json");
    assert.notEqual(p.stamp({ compileReads: reads }).hash, base);
    // Same bytes behind another target: still another input identity.
    retarget("same-as-a.json");
    assert.notEqual(p.stamp({ compileReads: reads }).hash, base);
    retarget("a.json");
    assert.equal(p.stamp({ compileReads: reads }).hash, base);

    // A symbolic link on a directory component on the way.
    p.write("data-v1/x.json", "1\n");
    p.write("data-v2/x.json", "2\n");
    symlinkSync("data-v1", path.join(p.root, "data"));
    const before = p.stamp({ compileReads: ["data/x.json"] }).hash;
    unlinkSync(path.join(p.root, "data"));
    symlinkSync("data-v2", path.join(p.root, "data"));
    assert.notEqual(p.stamp({ compileReads: ["data/x.json"] }).hash, before);
    assert.match(digestInputs(p.root, ["data/x.json"]).get("data/x.json"), /^link "data" "data-v2" 2 [0-9a-f]{64}$/);

    // A ".." after a link, whose lexical normal form is another file, is refused by name.
    mkdirSync(path.join(p.root, "deep/inner"), { recursive: true });
    symlinkSync(path.join(p.root, "deep/inner"), path.join(p.root, "core/engine/src/jump"));
    p.write("deep/read.json", "{}\n");
    assert.throws(
      () => compileReadsFromDepInfo(p.root, `${target}/engine.wasm: ${p.root}/core/engine/src/jump/../read.json\n`, [target]),
      /compile read .*core\/engine\/src\/jump\/\.\.\/read\.json has a "\.\." after a symbolic link/
    );
  } finally {
    p.cleanup();
  }
});

test("Codex r2: a \"..\" after a symbolic link is refused even where kernel and lexical resolution agree", () => {
  const p = project();
  try {
    const target = path.join(p.root, "core/engine/target");
    // alias -> sub: alias/.. is src both lexically and through the link, today.
    symlinkSync("sub", path.join(p.root, "core/engine/src/alias"));
    const depInfo = (read) => `${target}/engine.wasm: ${read}\n`;
    assert.throws(
      () => compileReadsFromDepInfo(p.root, depInfo(`${p.root}/core/engine/src/alias/../lib.rs`), [target]),
      /compile read .*src\/alias\/\.\.\/lib\.rs has a "\.\." after a symbolic link, so its path does not identify the file read/
    );
    // Two levels: the second ".." cancels the link.
    assert.throws(() => compileReadsFromDepInfo(p.root, depInfo(`${p.root}/core/engine/src/alias/x/../../lib.rs`), [target]), /after a symbolic link/);
    // A ".." that cancels an ordinary directory, or a link no ".." cancels, is accepted.
    assert.deepEqual(compileReadsFromDepInfo(p.root, depInfo(`${p.root}/core/engine/src/sub/../lib.rs`), [target]), ["core/engine/src/lib.rs"]);
    assert.deepEqual(compileReadsFromDepInfo(p.root, depInfo(`${p.root}/core/engine/src/alias/mod.rs`), [target]), ["core/engine/src/alias/mod.rs"]);
  } finally {
    p.cleanup();
  }
});

test("the compiler, wrapper and linker environment and the cargo configuration are part of the context", () => {
  const p = project();
  try {
    const rustc = path.join(p.root, "fake-rustc");
    writeFileSync(rustc, "#!/bin/sh\necho 'rustc 9.9.9 (fake)'\n");
    chmodSync(rustc, 0o755);
    const home = path.join(p.root, "cargo-home");
    mkdirSync(home);
    const env = { PATH: process.env.PATH, HOME: process.env.HOME, CARGO_HOME: home, RUSTC: rustc };
    const stampWith = (extra) => p.stamp({ context: engineStampContext(p.root, { ...env, ...extra }) }).hash;
    const base = stampWith({});
    assert.equal(stampWith({ CARGO_TARGET_DIR: "/elsewhere" }), base);
    for (const extra of [
      { RUSTC_WRAPPER: "sccache" },
      { RUSTC_WORKSPACE_WRAPPER: "clippy-driver" },
      { CARGO_BUILD_RUSTC_WRAPPER: "sccache" },
      { CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER: "wasm-ld" },
      { CARGO_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS: "-Ctarget-cpu=native" },
      { CARGO_ENCODED_RUSTFLAGS: "-Copt-level=1" },
      { CARGO_PROFILE_RELEASE_LTO: "true" }
    ]) {
      assert.notEqual(stampWith(extra), base, JSON.stringify(extra));
    }
    // The effective rustc's identity: CARGO_BUILD_RUSTC when RUSTC is unset.
    const other = path.join(p.root, "other-rustc");
    writeFileSync(other, "#!/bin/sh\necho 'rustc 9.9.10 (fake)'\n");
    chmodSync(other, 0o755);
    const { RUSTC: _unused, ...withoutRustc } = env;
    const viaBuild = engineStampContext(p.root, { ...withoutRustc, CARGO_BUILD_RUSTC: other });
    assert.deepEqual(viaBuild[0], ["rustc -vV", "rustc 9.9.10 (fake)"]);
    // A cargo configuration file is hashed.
    writeFileSync(path.join(home, "config.toml"), "[build]\njobs = 4\nrustflags = [\"-Cdebuginfo=0\"]\n");
    assert.notEqual(stampWith({}), base);
  } finally {
    p.cleanup();
  }
});

test("inputDrift: a newly discovered compile read, an added source or a changed input block the stamp", () => {
  const p = project();
  try {
    const input = (compileReads) => engineInputs({ projectRoot: p.root, crateDirectory: p.crateDirectory, compileReads, readManifest: p.readManifest });
    const drift = (knownReads, mutate, foundReads) => {
      const before = input(knownReads);
      const beforeDigests = digestInputs(p.root, before.all);
      mutate();
      const after = input(foundReads);
      return inputDrift({ beforeStatic: before.staticInputs, beforeDigests, after, afterDigests: digestInputs(p.root, after.all) });
    };
    assert.deepEqual(drift(["fixtures/read.json"], () => {}, ["fixtures/read.json"]), []);
    assert.deepEqual(drift([], () => {}, ["fixtures/read.json"]), ["compile reads not known before the build: fixtures/read.json"]);
    assert.deepEqual(drift(["fixtures/read.json"], () => p.write("fixtures/read.json", "{\"a\":9}\n"), ["fixtures/read.json"]), ["inputs changed during the build: fixtures/read.json"]);
    assert.deepEqual(drift(["fixtures/read.json"], () => p.write("core/engine/src/new.rs", "\n"), ["fixtures/read.json"]), ["source files added or removed during the build: core/engine/src/new.rs"]);
  } finally {
    p.cleanup();
  }
});

test("Codex r2: cargo configuration naming a compiler or wrapper is refused by name in every TOML spelling", () => {
  const p = project();
  try {
    const home = path.join(p.root, "cargo-home");
    mkdirSync(home);
    const config = path.join(home, "config.toml");
    const env = { PATH: process.env.PATH, HOME: process.env.HOME, CARGO_HOME: home };
    for (const [text, key] of [
      ["build = { rustc = \"/opt/rustc\" }\n", "rustc"],
      ["build={rustc=\"/opt/rustc\"}\n", "rustc"],
      ["build = { jobs = 2, \"rustc-wrapper\" = \"sccache\" }\n", "rustc-wrapper"],
      ["[build]\nrustc = \"/opt/rustc\"\n", "rustc"],
      ["build.rustc = \"/opt/rustc\"\n", "rustc"],
      ["[build]\n\"rustc\" = \"/opt/rustc\"\n", "rustc"],
      ["[build]\nrustc-wrapper = \"sccache\"\n", "rustc-wrapper"],
      ["[build]\nrustc-workspace-wrapper = \"clippy-driver\"\n", "rustc-workspace-wrapper"]
    ]) {
      writeFileSync(config, text);
      // Also when RUSTC names a compiler: the configuration is not resolved.
      for (const extra of [{}, { RUSTC: "rustc" }]) {
        assert.throws(() => engineStampContext(p.root, { ...env, ...extra }), new RegExp(`config\\.toml sets ${key} \\(a compiler or wrapper key\\)`), text);
      }
    }
    // A cargo configuration in the working directory's ancestry is read too.
    writeFileSync(config, "[build]\njobs = 4\n");
    mkdirSync(path.join(p.root, ".cargo"));
    writeFileSync(path.join(p.root, ".cargo/config.toml"), "[build]\nbuild = 1\nx = { y = 1, rustc = \"r\" }\n");
    assert.throws(() => engineStampContext(path.join(p.root, "core/engine"), env), /\.cargo\/config\.toml sets rustc/);
  } finally {
    p.cleanup();
  }
});

test("Codex r3: cargo configuration keys are decoded as TOML: escaped, literal and quoted-header spellings are refused by name; an unreadable file is refused", () => {
  const p = project();
  try {
    const home = path.join(p.root, "cargo-home");
    mkdirSync(home);
    const config = path.join(home, "config.toml");
    const env = { PATH: process.env.PATH, HOME: process.env.HOME, CARGO_HOME: home, RUSTC: "rustc" };
    const refused = (text, pattern) => {
      writeFileSync(config, text);
      assert.throws(() => engineStampContext(p.root, env), pattern, text);
    };
    const compiler = (key) => new RegExp(`config\\.toml sets ${key} \\(a compiler or wrapper key\\)`);
    const output = (key) => new RegExp(`config\\.toml sets build\\.${key} \\(an output directory key\\)`);
    // Basic-string keys with escapes, as cargo decodes them.
    refused("[build]\n\"rustc\" = \"/opt/rustc\"\n", compiler("rustc"));
    refused("[build]\n\"\\u0072ustc\" = \"/opt/rustc\"\n", compiler("rustc"));
    refused("[build]\n\"\\U00000072ustc-wrapper\" = \"sccache\"\n", compiler("rustc-wrapper"));
    refused("build.\"rustc\\u002dworkspace-wrapper\" = \"clippy-driver\"\n", compiler("rustc-workspace-wrapper"));
    // Literal-string keys and quoted, spaced header segments.
    refused("[build]\n'rustc' = '/opt/rustc'\n", compiler("rustc"));
    refused("[build]\n'target-dir' = '/tmp/elsewhere'\n", output("target-dir"));
    refused("[ \"build\" ]\n\"build-dir\" = \"/tmp/elsewhere\"\n", output("build-dir"));
    refused("[ 'b\\u0075ild' ]\nx = 1\n[\"b\\u0075ild\"]\ntarget-dir = \"x\"\n", output("target-dir"));
    refused("build = { jobs = 2, 'build-dir' = \"x\" }\n", output("build-dir"));
    refused("include = \"other.toml\"\n", /config\.toml sets include/);
    // Not tokenizable with confidence: refused, not guessed.
    refused("[build\nrustc = 1\n", /config\.toml cannot be read as TOML with confidence \(unterminated table header/);
    refused("[build]\n\"rus\\qtc\" = 1\n", /config\.toml cannot be read as TOML with confidence \(unknown escape/);
    refused("[build]\n\"\\uD800\" = 1\n", /cannot be read as TOML with confidence \(escape is not a Unicode scalar value/);
    // Names in comments, string values and other tables are not keys; target
    // linker, runner and rustflags are in the context through the file's bytes.
    writeFileSync(config, "# rustc = \"x\"\n[build]\njobs = 4 # target-dir\nrustflags = [\"-C\", \"rustc = 1\"]\n[target.'cfg(target_arch = \"wasm32\")']\nlinker = \"rust-lld\"\nrunner = '''\nrustc = 1'''\n[env]\nRUSTC_NOTE = { value = \"build-dir\", force = true }\nwhen = 1979-05-27 07:32:00Z\n");
    const before = engineStampContext(p.root, env);
    writeFileSync(config, readFileSync(config, "utf8").replace("rust-lld", "wasm-ld"));
    assert.notDeepEqual(engineStampContext(p.root, env), before);
  } finally {
    p.cleanup();
  }
});

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { realpathSync, mkdtempSync, mkdirSync, writeFileSync, copyFileSync, readFileSync, readdirSync, rmSync, existsSync, statSync, utimesSync, symlinkSync, unlinkSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { execFileSync, spawn } from 'node:child_process';
import { resolveWasmArtifact } from './build-wasm-engine.mjs';
import { STAMP_FORMAT, checkEngine, engineStampContext } from './engineSourceStamp.mjs';

// Fake cargo/rustup/rustc/wasm-bindgen. `cargo build` "reads" fixtures/read.json
// into an intermediate in the build directory (CARGO_BUILD_BUILD_DIR, else the
// target directory), uplifts it to the wasm artifact in CARGO_TARGET_DIR (else
// the fixed target) and writes the dep-info naming it; like cargo, it keeps an
// existing intermediate whose dep-info is newer than every input (freshness by
// modification time). With
// MUTATE_BUILDS=n the first n builds then edit the fixture, as an editor saving
// it after cargo read it would. Each build appends to BUILD_LOG when set.
function fakeToolchain(root, target) {
  return `#!${process.execPath}
import fs from 'node:fs'; import path from 'node:path';
const root=${JSON.stringify(root)}, fixed=${JSON.stringify(target)}, target=process.env.CARGO_TARGET_DIR||fixed;
const command=path.basename(process.argv[1]),args=process.argv.slice(2);
if(command==='cargo') {
  const m=args.indexOf('--manifest-path');
  if(args[0]==='metadata'&&m>=0){const manifest=args[m+1];console.log(JSON.stringify({target_directory:target,workspace_root:path.dirname(manifest),packages:[{manifest_path:manifest,dependencies:[],targets:[]}]}));}
  else if(args[0]==='metadata') console.log(JSON.stringify({target_directory:target}));
  else if(args[0]==='build'&&m>=0){
    const crate=path.dirname(args[m+1]); const name=crate.includes('self_weight')?'self_weight_wasm':'operation_applier';
    const dir=path.join(target,'wasm32-unknown-unknown/release'); fs.mkdirSync(dir,{recursive:true});
    const wasm=path.join(dir,'open_pipe_stress_'+name+'.wasm'), dep=path.join(dir,'open_pipe_stress_'+name+'.d'), read=path.join(root,'fixtures/read.json');
    const deps=path.join(process.env.CARGO_BUILD_BUILD_DIR||target,'wasm32-unknown-unknown/release/deps'); fs.mkdirSync(deps,{recursive:true});
    const cached=path.join(deps,name+'.wasm'), cachedDep=path.join(deps,name+'.d');
    const fresh=fs.existsSync(cached)&&fs.existsSync(cachedDep)&&[read,path.join(crate,'src/lib.rs')].every((input)=>fs.statSync(input).mtimeMs<=fs.statSync(cachedDep).mtimeMs);
    if(!fresh){ fs.writeFileSync(cached,'wasm:'+fs.readFileSync(read,'utf8')); fs.writeFileSync(cachedDep,'deps'); }
    fs.copyFileSync(cached,wasm);
    fs.writeFileSync(dep,target+'/x.wasm: '+crate+'/src/lib.rs '+root+'/fixtures/read.json '+target+'/out/generated.rs\\n');
    if(process.env.BUILD_LOG) fs.appendFileSync(process.env.BUILD_LOG,name+'\\n');
    const counter=path.join(root,'mutations'); const done=fs.existsSync(counter)?Number(fs.readFileSync(counter,'utf8')):0;
    if(done<Number(process.env.MUTATE_BUILDS||0)){fs.writeFileSync(read,'{"edit":'+(done+1)+'}');fs.writeFileSync(counter,String(done+1));}
  } else console.log('cargo fake');
}
if(command==='rustc') console.log('rustc fake');
if(command==='rustup') console.log('wasm32-unknown-unknown');
if(command==='wasm-bindgen') {if(args[0]==='--version') console.log('wasm-bindgen 0.2.123'); else {if(process.env.FAIL_SELF==='1' && args.at(-1).includes('self_weight')) process.exit(3); if(process.env.PAUSE_DIR && args.at(-1).includes('operation_applier')){fs.writeFileSync(path.join(process.env.PAUSE_DIR,'paused'),'');const t=Date.now();while(!fs.existsSync(path.join(process.env.PAUSE_DIR,'go'))&&Date.now()-t<60000)Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,20);} const out=args[args.indexOf('--out-dir')+1];fs.mkdirSync(out,{recursive:true});fs.writeFileSync(path.join(out,'artifact'),'new');fs.copyFileSync(args.at(-1),path.join(out,'engine.wasm'));}}
`;
}

// A temporary project with both engine crates, a fixture read at compile time
// and the fake toolchain first on PATH.
function fakeProject() {
  const root = realpathSync(mkdtempSync(path.join(tmpdir(), 'piping-wasm-build-')));
  const scripts = path.join(root, 'apps/desktop/scripts');
  const bin = path.join(root, 'bin');
  mkdirSync(scripts, {recursive:true}); mkdirSync(bin);
  for (const dir of ["core/model_operations/operation_applier", "core/loads/self_weight_wasm"]) { mkdirSync(path.join(root,dir,'src'),{recursive:true}); writeFileSync(path.join(root,dir,'Cargo.toml'),'[package]\n'); writeFileSync(path.join(root,dir,'src/lib.rs'),'// lib\n'); }
  mkdirSync(path.join(root,'fixtures'),{recursive:true}); writeFileSync(path.join(root,'fixtures/read.json'),'{}');
  for (const name of ['build-wasm-engine.mjs','engineSourceStamp.mjs','worktreeBuildLock.mjs']) copyFileSync(new URL(`./${name}`, import.meta.url), path.join(scripts, name));
  const target = path.join(root,'custom-target');
  for (const name of ['cargo','rustup','rustc','wasm-bindgen']) writeFileSync(path.join(bin,name),fakeToolchain(root, target),{mode:0o755});
  writeFileSync(path.join(bin,'package.json'),'{"type":"module"}');
  const env = { ...process.env, PATH: `${bin}:${process.env.PATH}` };
  const build = (extra = {}) => execFileSync(process.execPath,[path.join(scripts,'build-wasm-engine.mjs')],{env:{...env,...extra},encoding:'utf8',stdio:'pipe'});
  return { root, target, env, build, cleanup: () => rmSync(root,{recursive:true,force:true}) };
}

test('dual artifact build resolves Cargo target paths and preserves old selfweight on failed generation', () => {
  const root = realpathSync(mkdtempSync(path.join(tmpdir(), 'piping-wasm-build-')));
  const previousPath = process.env.PATH;
  try {
    const scripts = path.join(root, 'apps/desktop/scripts');
    const bin = path.join(root, 'bin');
    mkdirSync(scripts, {recursive:true}); mkdirSync(bin);
    const crates = ["core/model_operations/operation_applier", "core/loads/self_weight_wasm"];
    for (const dir of crates) { mkdirSync(path.join(root,dir,'src'),{recursive:true}); writeFileSync(path.join(root,dir,'Cargo.toml'),'[package]\n'); writeFileSync(path.join(root,dir,'src/lib.rs'),'// lib\n'); }
    mkdirSync(path.join(root,'fixtures'),{recursive:true}); writeFileSync(path.join(root,'fixtures/read.json'),'{}');
    for (const name of ['build-wasm-engine.mjs','engineSourceStamp.mjs','worktreeBuildLock.mjs']) copyFileSync(new URL(`./${name}`, import.meta.url), path.join(scripts, name));
    const target = path.join(root,'custom-target');
    const fake = fakeToolchain(root, target);
    for (const name of ['cargo','rustup','rustc','wasm-bindgen']) writeFileSync(path.join(bin,name),fake,{mode:0o755});
    writeFileSync(path.join(bin,'package.json'),'{"type":"module"}');
    process.env.PATH = `${bin}:${previousPath}`;
    assert.equal(resolveWasmArtifact(root),path.join(target,'wasm32-unknown-unknown/release/open_pipe_stress_operation_applier.wasm'));
    assert.equal(resolveWasmArtifact(root, 'x', { CARGO_TARGET_DIR: path.join(root, 'elsewhere') }), path.join(root, 'elsewhere/wasm32-unknown-unknown/release/x.wasm'));
    execFileSync(process.execPath,[path.join(scripts,'build-wasm-engine.mjs')]);
    // Each engine built in its own target directory under the app, not cargo's default.
    for (const name of ['wasm-engine','self-weight-engine']) assert.ok(existsSync(path.join(root,'apps/desktop/target/wasm-engines',name,'wasm32-unknown-unknown/release')), name);
    assert.ok(!existsSync(target));
    for (const name of ['wasm-engine','self-weight-engine']) assert.equal(readFileSync(path.join(root,'apps/desktop/public',name,'artifact'),'utf8'),'new');
    // Each engine folder carries its source stamp, and a recompute matches it.
    const context = engineStampContext(process.cwd());
    for (const [index, name] of ['wasm-engine','self-weight-engine'].entries()) {
      const stamp = JSON.parse(readFileSync(path.join(root,'apps/desktop/public',name,'source-stamp.json'),'utf8'));
      assert.equal(stamp.format, STAMP_FORMAT);
      assert.equal(stamp.engine, name);
      assert.deepEqual(stamp.compile_reads, ['fixtures/read.json']);
      const check = checkEngine({ projectRoot: root, crateDirectory: path.join(root, crates[index]), outDir: path.join(root,'apps/desktop/public',name), context });
      assert.equal(check.action, 'current');
      assert.equal(check.hash, stamp.hash);
    }
    const selfWeightStamp = readFileSync(path.join(root,'apps/desktop/public/self-weight-engine/source-stamp.json'),'utf8');
    writeFileSync(path.join(root,'apps/desktop/public/self-weight-engine/artifact'),'previous');
    // Only a stale engine is built: edit the self-weight crate.
    writeFileSync(path.join(root,crates[1],'src/lib.rs'),'// lib v2\n');
    assert.throws(()=>execFileSync(process.execPath,[path.join(scripts,'build-wasm-engine.mjs')],{env:{...process.env,FAIL_SELF:'1'},stdio:'pipe'}));
    assert.equal(readFileSync(path.join(root,'apps/desktop/public/self-weight-engine/artifact'),'utf8'),'previous');
    // A failed build writes no stamp: the previous one stays with the previous artifacts.
    assert.equal(readFileSync(path.join(root,'apps/desktop/public/self-weight-engine/source-stamp.json'),'utf8'), selfWeightStamp);
  } finally {process.env.PATH=previousPath;rmSync(root,{recursive:true,force:true});}
});

test('Codex r1: a compile read edited after cargo read it is not stamped; cargo builds again until the inputs hold still', () => {
  const p = fakeProject();
  const previousPath = process.env.PATH;
  try {
    // checkEngine below runs the fake cargo too.
    process.env.PATH = p.env.PATH;
    const log = path.join(p.root, 'builds.log');
    p.build({ BUILD_LOG: log });
    const crate = path.join(p.root, 'core/model_operations/operation_applier');
    const outDir = path.join(p.root, 'apps/desktop/public/wasm-engine');
    const context = engineStampContext(process.cwd(), p.env);
    assert.equal(checkEngine({ projectRoot: p.root, crateDirectory: crate, outDir, context }).action, 'current');
    rmSync(log);
    // The engine is edited, and the fixture is saved again after its cargo read it.
    writeFileSync(path.join(crate, 'src/lib.rs'), '// lib v2\n');
    const output = p.build({ BUILD_LOG: log, MUTATE_BUILDS: '1' });
    assert.match(output, /operation_applier: not stamped \(inputs changed during the build: fixtures\/read\.json\); building again/);
    assert.deepEqual(readFileSync(log, 'utf8').trim().split('\n'), ['operation_applier', 'operation_applier', 'self_weight_wasm']);
    // The published engine embeds the fixture its stamp records.
    const fixture = readFileSync(path.join(p.root, 'fixtures/read.json'), 'utf8');
    assert.equal(fixture, '{"edit":1}');
    assert.equal(readFileSync(path.join(outDir, 'engine.wasm'), 'utf8'), `wasm:${fixture}`);
    const check = checkEngine({ projectRoot: p.root, crateDirectory: crate, outDir, context });
    assert.equal(check.action, 'current');
    assert.equal(JSON.parse(readFileSync(path.join(outDir, 'source-stamp.json'), 'utf8')).hash, check.hash);
  } finally { process.env.PATH = previousPath; p.cleanup(); }
});

test('a fresh build verifies its newly discovered compile reads with a second build; inputs that never hold still fail by name', () => {
  const p = fakeProject();
  try {
    const log = path.join(p.root, 'builds.log');
    const output = p.build({ BUILD_LOG: log });
    assert.match(output, /not stamped \(compile reads not known before the build: fixtures\/read\.json\)/);
    assert.deepEqual(readFileSync(log, 'utf8').trim().split('\n'), ['operation_applier', 'operation_applier', 'self_weight_wasm', 'self_weight_wasm']);
    const stampPath = path.join(p.root, 'apps/desktop/public/wasm-engine/source-stamp.json');
    const stamp = readFileSync(stampPath, 'utf8');
    writeFileSync(path.join(p.root, 'core/model_operations/operation_applier/src/lib.rs'), '// lib v2\n');
    let failure = null;
    try { p.build({ MUTATE_BUILDS: '99' }); } catch (error) { failure = error; }
    assert.ok(failure, 'the build must fail');
    assert.match(failure.stderr, /core\/model_operations\/operation_applier did not hold still across 3 builds/);
    assert.match(failure.stderr, /inputs changed during the build: fixtures\/read\.json/);
    assert.equal(readFileSync(stampPath, 'utf8'), stamp);
  } finally { p.cleanup(); }
});

test('Codex r2: an edit that keeps the modification time, or a link retargeted to an older file, is built clean and published', () => {
  const p = fakeProject();
  const previousPath = process.env.PATH;
  try {
    process.env.PATH = p.env.PATH;
    p.build();
    const crate = path.join(p.root, 'core/model_operations/operation_applier');
    const outDir = path.join(p.root, 'apps/desktop/public/wasm-engine');
    const fixture = path.join(p.root, 'fixtures/read.json');
    const context = engineStampContext(process.cwd(), p.env);
    const published = () => readFileSync(path.join(outDir, 'engine.wasm'), 'utf8');
    assert.equal(published(), 'wasm:{}');
    // A current stamp builds nothing.
    const log = path.join(p.root, 'builds.log');
    assert.match(p.build({ BUILD_LOG: log }), /operation_applier: the published engine matches its sources \(stamp sha256:[0-9a-f]{64}\); nothing built/);
    assert.ok(!existsSync(log), 'cargo built nothing');
    // New bytes, old modification time: cargo's freshness check would keep the old artifact.
    const old = statSync(fixture);
    writeFileSync(fixture, '{"v":2}');
    utimesSync(fixture, old.atime, old.mtime);
    const output = p.build();
    assert.equal(published(), 'wasm:{"v":2}');
    assert.match(output, /core\/model_operations\/operation_applier: clean build in target\/wasm-engines\/wasm-engine \(inputs changed since stamp sha256:[0-9a-f]{64}\)/);
    assert.equal(checkEngine({ projectRoot: p.root, crateDirectory: crate, outDir, context }).action, 'current');
    // The fixture becomes a link to a file older than the last build.
    const older = path.join(p.root, 'fixtures/older.json');
    writeFileSync(older, '{"v":"older"}');
    utimesSync(older, new Date(2000, 0, 1), new Date(2000, 0, 1));
    unlinkSync(fixture);
    symlinkSync('older.json', fixture);
    p.build();
    assert.equal(published(), 'wasm:{"v":"older"}');
    assert.equal(checkEngine({ projectRoot: p.root, crateDirectory: crate, outDir, context }).action, 'current');
  } finally { process.env.PATH = previousPath; p.cleanup(); }
});

test('the clean build deletes only an engine\'s dedicated target directory, never through a symbolic link', () => {
  const p = fakeProject();
  try {
    p.build();
    // The app's target directory replaced by a link to a shared one.
    const shared = path.join(p.root, 'shared-target');
    mkdirSync(path.join(shared, 'wasm-engines/wasm-engine'), { recursive: true });
    writeFileSync(path.join(shared, 'wasm-engines/wasm-engine/keep'), 'shared');
    rmSync(path.join(p.root, 'apps/desktop/target'), { recursive: true, force: true });
    symlinkSync(shared, path.join(p.root, 'apps/desktop/target'));
    writeFileSync(path.join(p.root, 'fixtures/read.json'), '{"v":3}');
    let failure = null;
    try { p.build(); } catch (error) { failure = error; }
    assert.ok(failure, 'the build must refuse');
    // Refused before anything is locked, built or deleted there.
    assert.match(failure.stderr, /refusing to take the build lock .*target\/build\.lock: .*apps\/desktop\/target is a symbolic link/);
    assert.equal(readFileSync(path.join(shared, 'wasm-engines/wasm-engine/keep'), 'utf8'), 'shared');
  } finally { p.cleanup(); }
});

test('Codex r3: an inherited CARGO_BUILD_BUILD_DIR cannot keep stale intermediates; cargo configuration moving the output is refused by name', () => {
  const p = fakeProject();
  try {
    const external = path.join(p.root, 'external-build');
    const inherited = { CARGO_BUILD_BUILD_DIR: external };
    const outDir = path.join(p.root, 'apps/desktop/public/wasm-engine');
    const published = () => readFileSync(path.join(outDir, 'engine.wasm'), 'utf8');
    p.build(inherited);
    assert.equal(published(), 'wasm:{}');
    // New bytes, old modification time: an intermediate kept outside the deleted
    // target directory would be fresh by cargo's check and published as current.
    const fixture = path.join(p.root, 'fixtures/read.json');
    const old = statSync(fixture);
    writeFileSync(fixture, '{"v":2}');
    utimesSync(fixture, old.atime, old.mtime);
    assert.match(p.build(inherited), /operation_applier: clean build in target\/wasm-engines\/wasm-engine/);
    assert.equal(published(), 'wasm:{"v":2}');
    assert.ok(!existsSync(external), 'cargo built nothing outside the dedicated directory');
    // A configuration file setting either directory, in any spelling, is refused.
    const home = path.join(p.root, 'cargo-home');
    mkdirSync(home);
    for (const [text, key] of [
      ['[build]\nbuild-dir = "/tmp/elsewhere"\n', 'build-dir'],
      ['build = { "target-dir" = "/tmp/elsewhere" }\n', 'target-dir'],
      ['[ "build" ]\n"\\u0062uild-dir" = "/tmp/elsewhere"\n', 'build-dir']
    ]) {
      writeFileSync(path.join(home, 'config.toml'), text);
      let failure = null;
      try { p.build({ CARGO_HOME: home }); } catch (error) { failure = error; }
      assert.ok(failure, text);
      assert.match(failure.stderr, new RegExp(`cargo-home/config\\.toml sets build\\.${key} \\(an output directory key\\)`), text);
    }
  } finally { p.cleanup(); }
});

test('Codex r3: a symbolic link where cargo descends beneath an engine target directory is refused before cargo builds or the directory is deleted', () => {
  const p = fakeProject();
  try {
    p.build();
    const engineTarget = path.join(p.root, 'apps/desktop/target/wasm-engines/wasm-engine');
    // `wasm32-unknown-unknown/release` a link to another worktree's output.
    const other = path.join(p.root, 'other-worktree/target/release');
    mkdirSync(path.join(other, 'deps'), { recursive: true });
    writeFileSync(path.join(other, 'deps/keep'), 'other');
    rmSync(path.join(engineTarget, 'wasm32-unknown-unknown'), { recursive: true });
    mkdirSync(path.join(engineTarget, 'wasm32-unknown-unknown'));
    symlinkSync(other, path.join(engineTarget, 'wasm32-unknown-unknown/release'));
    const refused = () => {
      let failure = null;
      try { p.build(); } catch (error) { failure = error; }
      assert.ok(failure, 'the build must refuse');
      assert.match(failure.stderr, /refusing to let cargo build or clean in .*wasm-engines\/wasm-engine: .*wasm32-unknown-unknown\/release is a symbolic link/);
    };
    // A current stamp builds nothing, so cargo touches nothing there; a changed source is refused before the clean build.
    assert.match(p.build(), /operation_applier: the published engine matches its sources/);
    writeFileSync(path.join(p.root, 'fixtures/read.json'), '{"v":4}');
    refused();
    assert.deepEqual(readdirSync(other), ['deps']);
    assert.deepEqual(readdirSync(path.join(other, 'deps')), ['keep']);
    assert.equal(readFileSync(path.join(other, 'deps/keep'), 'utf8'), 'other');
  } finally { p.cleanup(); }
});

test('Codex r4: a build that failed after cargo compiled leaves the published engine; restored sources build nothing and keep its bytes', () => {
  const p = fakeProject();
  const previousPath = process.env.PATH;
  try {
    process.env.PATH = p.env.PATH;
    const fixture = path.join(p.root, 'fixtures/read.json');
    const crate = path.join(p.root, 'core/loads/self_weight_wasm');
    const outDir = path.join(p.root, 'apps/desktop/public/self-weight-engine');
    const published = () => readFileSync(path.join(outDir, 'engine.wasm'), 'utf8');
    const context = engineStampContext(process.cwd(), p.env);
    // A built and published.
    p.build();
    assert.equal(published(), 'wasm:{}');
    const stampA = readFileSync(path.join(outDir, 'source-stamp.json'), 'utf8');
    // B: cargo compiles it, then the self-weight glue generation fails; A's stamp stays published.
    const a = statSync(fixture);
    writeFileSync(fixture, '{"v":"B"}');
    assert.throws(() => p.build({ FAIL_SELF: '1' }));
    assert.equal(readFileSync(path.join(outDir, 'source-stamp.json'), 'utf8'), stampA);
    // A restored with its old modification time: the published stamp matches,
    // and cargo's freshness check would reuse B's intermediate.
    writeFileSync(fixture, '{}');
    utimesSync(fixture, a.atime, a.mtime);
    assert.equal(checkEngine({ projectRoot: p.root, crateDirectory: crate, outDir, context }).action, 'current');
    const output = p.build();
    assert.equal(published(), 'wasm:{}');
    assert.equal(readFileSync(path.join(outDir, 'source-stamp.json'), 'utf8'), stampA);
    assert.match(output, /self_weight_wasm: the published engine matches its sources .*; nothing built/);
  } finally { process.env.PATH = previousPath; p.cleanup(); }
});

test('Codex r5: an ordinary cargo build of other sources in the engine target directory is never published under the current stamp', () => {
  const p = fakeProject();
  const previousPath = process.env.PATH;
  try {
    process.env.PATH = p.env.PATH;
    const fixture = path.join(p.root, 'fixtures/read.json');
    const crate = path.join(p.root, 'core/model_operations/operation_applier');
    const outDir = path.join(p.root, 'apps/desktop/public/wasm-engine');
    const engineTarget = path.join(p.root, 'apps/desktop/target/wasm-engines/wasm-engine');
    const published = () => readFileSync(path.join(outDir, 'engine.wasm'), 'utf8');
    const context = engineStampContext(process.cwd(), p.env);
    // A published.
    p.build();
    assert.equal(published(), 'wasm:{}');
    const stampA = readFileSync(path.join(outDir, 'source-stamp.json'), 'utf8');
    // An ordinary cargo build of B in the engine's dedicated target directory.
    const a = statSync(fixture);
    writeFileSync(fixture, '{"v":"B"}');
    execFileSync('cargo', ['build', '--release', '--manifest-path', path.join(crate, 'Cargo.toml')], { env: { ...p.env, CARGO_TARGET_DIR: engineTarget, CARGO_BUILD_BUILD_DIR: engineTarget } });
    assert.equal(readFileSync(path.join(engineTarget, 'wasm32-unknown-unknown/release/open_pipe_stress_operation_applier.wasm'), 'utf8'), 'wasm:{"v":"B"}');
    // A restored with its old modification time: B's intermediate is fresh by cargo's check.
    writeFileSync(fixture, '{}');
    utimesSync(fixture, a.atime, a.mtime);
    assert.equal(checkEngine({ projectRoot: p.root, crateDirectory: crate, outDir, context }).action, 'current');
    const output = p.build();
    assert.match(output, /operation_applier: the published engine matches its sources .*; nothing built/);
    assert.equal(published(), 'wasm:{}');
    assert.equal(readFileSync(path.join(outDir, 'source-stamp.json'), 'utf8'), stampA);
    // Any build is clean: B's intermediate cannot serve a later build of C either.
    const c = statSync(fixture);
    writeFileSync(fixture, '{"v":"C"}');
    execFileSync('cargo', ['build', '--release', '--manifest-path', path.join(crate, 'Cargo.toml')], { env: { ...p.env, CARGO_TARGET_DIR: engineTarget, CARGO_BUILD_BUILD_DIR: engineTarget } });
    writeFileSync(fixture, '{"v":"D"}');
    utimesSync(fixture, c.atime, c.mtime);
    assert.match(p.build(), /operation_applier: clean build in target\/wasm-engines\/wasm-engine \(inputs changed since stamp/);
    assert.equal(published(), 'wasm:{"v":"D"}');
  } finally { process.env.PATH = previousPath; p.cleanup(); }
});

// Run the build script as a child; resolves with its exit code and output.
function startBuild(p, extra) {
  const child = spawn(process.execPath, [path.join(p.root, 'apps/desktop/scripts/build-wasm-engine.mjs')], { env: { ...p.env, ...extra }, stdio: ['ignore', 'pipe', 'pipe'] });
  let output = '';
  child.stdout.on('data', (chunk) => { output += chunk; });
  child.stderr.on('data', (chunk) => { output += chunk; });
  const done = new Promise((resolve) => child.on('exit', (code) => resolve(code)));
  return { child, done, output: () => output };
}

async function until(condition, what, ms = 30000) {
  const start = Date.now();
  while (!condition()) {
    if (Date.now() - start > ms) throw new Error(`timed out waiting for ${what}`);
    await new Promise((resolve) => setTimeout(resolve, 20));
  }
}

test('Codex r5: two builds in one worktree do not interleave; the second waits for the build lock and nothing is misbound', async () => {
  const p = fakeProject();
  const previousPath = process.env.PATH;
  try {
    process.env.PATH = p.env.PATH;
    const fixture = path.join(p.root, 'fixtures/read.json');
    const crate = path.join(p.root, 'core/model_operations/operation_applier');
    const outDir = path.join(p.root, 'apps/desktop/public/wasm-engine');
    const pause = path.join(p.root, 'pause');
    mkdirSync(pause);
    // The first build compiles and stamps A, then pauses before its glue reads the artifact.
    const first = startBuild(p, { PAUSE_DIR: pause });
    await until(() => existsSync(path.join(pause, 'paused')), 'the first build to pause');
    // The second build starts on B while the first is paused.
    writeFileSync(fixture, '{"v":"B"}');
    const second = startBuild(p, { OPENPIPESTRESS_BUILD_LOCK_WAIT_MS: '60000' });
    const waiting = `waiting for the build lock held by pid ${first.child.pid}`;
    let secondCode = null;
    second.done.then((code) => { secondCode = code; });
    await until(() => second.output().includes(waiting) || secondCode !== null, 'the second build to wait or finish');
    writeFileSync(path.join(pause, 'go'), '');
    const [firstCode] = await Promise.all([first.done, second.done]);
    assert.equal(firstCode, 0, first.output());
    assert.equal(secondCode, 0, second.output());
    assert.ok(second.output().includes(waiting), second.output());
    // The published bytes are the ones the published stamp describes.
    const context = engineStampContext(process.cwd(), p.env);
    assert.equal(readFileSync(path.join(outDir, 'engine.wasm'), 'utf8'), 'wasm:{"v":"B"}');
    assert.equal(checkEngine({ projectRoot: p.root, crateDirectory: crate, outDir, context }).action, 'current');
    assert.ok(!existsSync(path.join(p.root, 'apps/desktop/target/build.lock')), 'the lock is released');
  } finally { process.env.PATH = previousPath; p.cleanup(); }
});

test('a build holding the worktree lock refuses by name a cargo that would take a host slot', () => {
  const p = fakeProject();
  try {
    const shim = path.join(p.root, 'shim');
    mkdirSync(shim);
    writeFileSync(path.join(shim, 'cargo'), '#!/bin/sh\nexec /opt/host/tools/t3_cargo.sh "$@"\n', { mode: 0o755 });
    let failure = null;
    try { p.build({ PATH: `${shim}:${p.env.PATH}` }); } catch (error) { failure = error; }
    assert.ok(failure, 'the build must refuse');
    assert.match(failure.stderr, /refusing to run cargo \(.*shim\/cargo\) while holding the worktree build lock .*build\.lock: it would acquire a host slot \(t3_cargo\.sh\)/);
    assert.ok(!existsSync(path.join(p.root, 'apps/desktop/target/build.lock')), 'the lock is released on exit');
  } finally { p.cleanup(); }
});

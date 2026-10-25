import { test } from 'node:test';
import assert from 'node:assert/strict';
import { realpathSync, mkdtempSync, mkdirSync, writeFileSync, copyFileSync, readFileSync, rmSync, existsSync, statSync, utimesSync, symlinkSync, unlinkSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { resolveWasmArtifact } from './build-wasm-engine.mjs';
import { STAMP_FORMAT, checkEngine, engineStampContext } from './engineSourceStamp.mjs';

// Fake cargo/rustup/rustc/wasm-bindgen. `cargo build` "reads" fixtures/read.json
// into the wasm artifact in CARGO_TARGET_DIR (else the fixed target) and writes
// the dep-info naming it; like cargo, it keeps an existing artifact whose
// dep-info is newer than every input (freshness by modification time). With
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
    const fresh=fs.existsSync(wasm)&&fs.existsSync(dep)&&[read,path.join(crate,'src/lib.rs')].every((input)=>fs.statSync(input).mtimeMs<=fs.statSync(dep).mtimeMs);
    if(!fresh){
      fs.writeFileSync(wasm,'wasm:'+fs.readFileSync(read,'utf8'));
      fs.writeFileSync(dep,target+'/x.wasm: '+crate+'/src/lib.rs '+root+'/fixtures/read.json '+target+'/out/generated.rs\\n');
    }
    if(process.env.BUILD_LOG) fs.appendFileSync(process.env.BUILD_LOG,name+'\\n');
    const counter=path.join(root,'mutations'); const done=fs.existsSync(counter)?Number(fs.readFileSync(counter,'utf8')):0;
    if(done<Number(process.env.MUTATE_BUILDS||0)){fs.writeFileSync(read,'{"edit":'+(done+1)+'}');fs.writeFileSync(counter,String(done+1));}
  } else console.log('cargo fake');
}
if(command==='rustc') console.log('rustc fake');
if(command==='rustup') console.log('wasm32-unknown-unknown');
if(command==='wasm-bindgen') {if(args[0]==='--version') console.log('wasm-bindgen 0.2.123'); else {if(process.env.FAIL_SELF==='1' && args.at(-1).includes('self_weight')) process.exit(3); const out=args[args.indexOf('--out-dir')+1];fs.mkdirSync(out,{recursive:true});fs.writeFileSync(path.join(out,'artifact'),'new');fs.copyFileSync(args.at(-1),path.join(out,'engine.wasm'));}}
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
  for (const name of ['build-wasm-engine.mjs','engineSourceStamp.mjs']) copyFileSync(new URL(`./${name}`, import.meta.url), path.join(scripts, name));
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
    for (const name of ['build-wasm-engine.mjs','engineSourceStamp.mjs']) copyFileSync(new URL(`./${name}`, import.meta.url), path.join(scripts, name));
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
    // The fixture is saved again after the first engine's cargo read it.
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
    // A current stamp builds incrementally: no clean build.
    assert.doesNotMatch(p.build(), /clean build/);
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
    assert.match(failure.stderr, /refusing to delete .*wasm-engines\/wasm-engine: .*target is a symbolic link/);
    assert.equal(readFileSync(path.join(shared, 'wasm-engines/wasm-engine/keep'), 'utf8'), 'shared');
  } finally { p.cleanup(); }
});

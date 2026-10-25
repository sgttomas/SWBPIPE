import { test } from 'node:test';
import assert from 'node:assert/strict';
import { realpathSync, mkdtempSync, mkdirSync, writeFileSync, copyFileSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { resolveWasmArtifact } from './build-wasm-engine.mjs';
import { STAMP_FORMAT, checkEngine, engineStampContext } from './engineSourceStamp.mjs';

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
    mkdirSync(path.join(target,'wasm32-unknown-unknown/release'),{recursive:true});
    for (const name of ['operation_applier','self_weight_wasm']) {
      writeFileSync(path.join(target,`wasm32-unknown-unknown/release/open_pipe_stress_${name}.wasm`),'wasm');
      // Cargo's dep-info: a source, a compile read outside src/ and a build output (not an input).
      writeFileSync(path.join(target,`wasm32-unknown-unknown/release/open_pipe_stress_${name}.d`),`${target}/x.wasm: ${root}/${crates[name==='operation_applier'?0:1]}/src/lib.rs ${root}/fixtures/read.json ${target}/out/generated.rs\n`);
    }
    const fake = `#!${process.execPath}\nimport fs from 'node:fs'; import path from 'node:path';\nconst command=path.basename(process.argv[1]),args=process.argv.slice(2);\nif(command==='cargo') { const m=args.indexOf('--manifest-path'); if(args[0]==='metadata'&&m>=0){const manifest=args[m+1];console.log(JSON.stringify({target_directory:${JSON.stringify(target)},workspace_root:path.dirname(manifest),packages:[{manifest_path:manifest,dependencies:[],targets:[]}]}));} else console.log(args[0]==='metadata'?JSON.stringify({target_directory:${JSON.stringify(target)}}):'cargo fake'); }\nif(command==='rustc') console.log('rustc fake');\nif(command==='rustup') console.log('wasm32-unknown-unknown');\nif(command==='wasm-bindgen') {if(args[0]==='--version') console.log('wasm-bindgen 0.2.123'); else {if(process.env.FAIL_SELF==='1' && args.at(-1).includes('self_weight')) process.exit(3); const out=args[args.indexOf('--out-dir')+1];fs.mkdirSync(out,{recursive:true});fs.writeFileSync(path.join(out,'artifact'),'new');}}\n`;
    for (const name of ['cargo','rustup','rustc','wasm-bindgen']) writeFileSync(path.join(bin,name),fake,{mode:0o755});
    writeFileSync(path.join(bin,'package.json'),'{"type":"module"}');
    process.env.PATH = `${bin}:${previousPath}`;
    assert.equal(resolveWasmArtifact(root),path.join(target,'wasm32-unknown-unknown/release/open_pipe_stress_operation_applier.wasm'));
    execFileSync(process.execPath,[path.join(scripts,'build-wasm-engine.mjs')]);
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

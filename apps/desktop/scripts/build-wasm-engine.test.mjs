import { test } from 'node:test';
import assert from 'node:assert/strict';
import { realpathSync, mkdtempSync, mkdirSync, writeFileSync, copyFileSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { resolveWasmArtifact } from './build-wasm-engine.mjs';

test('dual artifact build resolves Cargo target paths and preserves old selfweight on failed generation', () => {
  const root = realpathSync(mkdtempSync(path.join(tmpdir(), 'piping-wasm-build-')));
  const previousPath = process.env.PATH;
  try {
    const scripts = path.join(root, 'apps/desktop/scripts');
    const bin = path.join(root, 'bin');
    mkdirSync(scripts, {recursive:true}); mkdirSync(bin);
    for (const dir of ["core/model_operations/operation_applier", "core/loads/self_weight_wasm"]) mkdirSync(path.join(root,dir),{recursive:true});
    copyFileSync(new URL('./build-wasm-engine.mjs', import.meta.url), path.join(scripts, 'build-wasm-engine.mjs'));
    const target = path.join(root,'custom-target');
    mkdirSync(path.join(target,'wasm32-unknown-unknown/release'),{recursive:true});
    for (const name of ['operation_applier','self_weight_wasm']) writeFileSync(path.join(target,`wasm32-unknown-unknown/release/open_pipe_stress_${name}.wasm`),'wasm');
    const fake = `#!${process.execPath}\nimport fs from 'node:fs'; import path from 'node:path';\nconst command=path.basename(process.argv[1]),args=process.argv.slice(2);\nif(command==='cargo') { console.log(args[0]==='metadata'?JSON.stringify({target_directory:${JSON.stringify(target)}}):'cargo fake'); }\nif(command==='rustup') console.log('wasm32-unknown-unknown');\nif(command==='wasm-bindgen') {if(args[0]==='--version') console.log('wasm-bindgen 0.2.123'); else {if(process.env.FAIL_SELF==='1' && args.at(-1).includes('self_weight')) process.exit(3); const out=args[args.indexOf('--out-dir')+1];fs.mkdirSync(out,{recursive:true});fs.writeFileSync(path.join(out,'artifact'),'new');}}\n`;
    for (const name of ['cargo','rustup','wasm-bindgen']) writeFileSync(path.join(bin,name),fake,{mode:0o755});
    writeFileSync(path.join(bin,'package.json'),'{"type":"module"}');
    process.env.PATH = `${bin}:${previousPath}`;
    assert.equal(resolveWasmArtifact(root),path.join(target,'wasm32-unknown-unknown/release/open_pipe_stress_operation_applier.wasm'));
    execFileSync(process.execPath,[path.join(scripts,'build-wasm-engine.mjs')]);
    for (const name of ['wasm-engine','self-weight-engine']) assert.equal(readFileSync(path.join(root,'apps/desktop/public',name,'artifact'),'utf8'),'new');
    writeFileSync(path.join(root,'apps/desktop/public/self-weight-engine/artifact'),'previous');
    assert.throws(()=>execFileSync(process.execPath,[path.join(scripts,'build-wasm-engine.mjs')],{env:{...process.env,FAIL_SELF:'1'},stdio:'pipe'}));
    assert.equal(readFileSync(path.join(root,'apps/desktop/public/self-weight-engine/artifact'),'utf8'),'previous');
  } finally {process.env.PATH=previousPath;rmSync(root,{recursive:true,force:true});}
});

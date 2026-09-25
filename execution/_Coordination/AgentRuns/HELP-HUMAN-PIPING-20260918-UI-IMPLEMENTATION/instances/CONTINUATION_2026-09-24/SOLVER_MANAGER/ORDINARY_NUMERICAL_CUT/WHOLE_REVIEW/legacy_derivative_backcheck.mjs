// Actual-source repair backcheck; preserves original failing probe artifacts.
import { createRequire } from 'node:module';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
const root=process.cwd(), output=path.dirname(fileURLToPath(import.meta.url));
const require=createRequire(path.join(root,'package.json')), {build}=require('esbuild');
const sourcePath='apps/desktop/src/features/result-export/resultExportAdapter.ts';
const built=await build({stdin:{contents:`export {deriveResultDocument,validateResultDocument,resultDigest} from ${JSON.stringify(path.join(root,sourcePath))};`,resolveDir:root,loader:'ts'},bundle:true,write:false,format:'esm',platform:'node',logLevel:'silent'});
const api=await import(`data:text/javascript;base64,${Buffer.from(built.outputFiles[0].contents).toString('base64')}`);
const oldBytes=readFileSync(path.join(output,'legacy_derivative_probe.json'));
const old=JSON.parse(oldBytes), attempts=[];
for(const original of old.attempts){
 const base=structuredClone(old.base);Object.assign(base.result_envelope,original.mutation);
 const result={name:original.name};
 for(const [operation,run] of [['construct',()=>api.deriveResultDocument(base,old.model,old.source,old.origin)],['validate_prior_hash_valid_document',()=>api.validateResultDocument(original.document,old.source)]]){
  try{await run();result[operation]={accepted:true};}catch(error){result[operation]={accepted:false,error:String(error)};}
 }
 const projected=structuredClone(original.document);delete projected.result_envelope.reproducibility.derivative_hash;
 result.original_hash_still_valid=await api.resultDigest(projected)===original.document.result_envelope.reproducibility.derivative_hash.value;
 attempts.push(result);
}
if(!attempts[0].construct.accepted || !attempts[0].validate_prior_hash_valid_document.accepted || attempts.slice(1).some(x=>x.construct.accepted||x.validate_prior_hash_valid_document.accepted||!x.original_hash_still_valid))throw Error('Repair backcheck failed');
const sha=x=>createHash('sha256').update(x).digest('hex');
const identities=Object.fromEntries([sourcePath,'apps/desktop/src/features/result-export/resultExportAdapter.test.ts','core/reporting/result_export/src/derivative.rs','apps/desktop/public/wasm-engine/open_pipe_stress_operation_applier.js','apps/desktop/public/wasm-engine/open_pipe_stress_operation_applier_bg.wasm'].map(p=>[p,sha(readFileSync(path.join(root,p)))]));
writeFileSync(path.join(output,'legacy_derivative_backcheck.json'),JSON.stringify({node:process.version,esbuild:require('esbuild/package.json').version,original_probe_sha256:sha(oldBytes),identities,attempts,scope:'Actual TS constructor and validator; existing WASM hashing; no Current admission or Rust execution.'},null,2)+'\n');
console.log(JSON.stringify(attempts,null,2));

// ROOT-authorized bounded actual-source constructor/validator probe.
// Run from projects/chirality-piping. No application, Rust build, or suite.
import { createRequire } from 'node:module';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
const root = process.cwd();
const output = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(path.join(root, 'package.json'));
const { build } = require('esbuild');
const sha = bytes => createHash('sha256').update(bytes).digest('hex');
const sourcePath = 'apps/desktop/src/features/result-export/resultExportAdapter.ts';
const compiled = await build({stdin:{contents:`export { deriveResultDocument, validateResultDocument, resultDigest, ref, derivativeProvenance } from ${JSON.stringify(path.join(root,sourcePath))};`,resolveDir:root,loader:'ts'},bundle:true,write:false,format:'esm',platform:'node',logLevel:'silent'});
const compiledBytes = compiled.outputFiles[0].contents;
const { deriveResultDocument, validateResultDocument, resultDigest, ref, derivativeProvenance } = await import(`data:text/javascript;base64,${Buffer.from(compiledBytes).toString('base64')}`);
const model = {project:{id:'project:independent-legacy-probe'}};
const source = {schema_version:'0.1.0',run_id:'independent-legacy-probe',model_ref:model.project.id,status:{mechanics:'MECHANICS_SOLVED'},results:[{id:'result:probe:ux',kind:'global_nodal_displacement_x',value:0.125,unit:'mm',entity_ref:'node:probe',basis_ref:{ref_type:'load_case',ref_id:'load:probe'}}]};
const origin = {origin_id:'independent-probe-unattested',origin_class:'received_current_dimension_absent',qualification_ref:ref('synthetic_fixture','not-Current'),authentic_producer_available:false,received_carrier_checksum:{algorithm:'sha256',canonicalization:'openpipestress_jcs_ijson_v1',payload_scope:'received_current_dimension_absent_carrier',payload_ref:ref('synthetic_received_carrier','probe'),value:await resultDigest(source)},original_producer_checksum:null,origin_limit:'pure constructor/validator probe; no Current authenticity',actual_model_ref:ref('model_payload',model.project.id),mechanics_run_ref:ref('mechanics_run',source.run_id),request_model_ref:null,request_run_ref:null,request_alias_disclosure:null};
const base = {schema_version:'0.2.0',result_envelope:{schema_version:'0.2.0',envelope_id:'probe',run_ref:ref('analysis_run',source.run_id),provenance:derivativeProvenance,unit_system_ref:ref('unit_system','probe'),result_sets:[{values:[]}],reproducibility:{}}};
const headers = {producer:{component_name:'open_pipe_stress_product_physics',component_version:'0.2.0',semantic_contract_id:'openpipestress.result_semantics/0.3.0/precision-1'},numerical_quality:{value_representation:'finite_binary64',publication_quantization:'none',integrity_policy:'M03-INTEGRITY-v1',status:'checks_passed',cases:[]},formulation_basis:{profile_id:'product_preview_mechanics_v1',limitations:['Deliberately contradictory synthetic legacy-header probe.']},semantic_contract_ref:ref('semantic_contract','openpipestress.result_semantics/0.3.0/precision-1')};
const attempts = [];
for (const [name, mutation] of [['baseline',{}], ...Object.entries(headers).map(([key,value])=>[key,{[key]:value}]), ['all_headers',headers]]) {
  const supplied = structuredClone(base);Object.assign(supplied.result_envelope,mutation);
  try {
    const document = await deriveResultDocument(supplied,model,source,origin);
    await validateResultDocument(document,source);
    const projected = structuredClone(document);delete projected.result_envelope.reproducibility.derivative_hash;
    attempts.push({name,mutation,accepted:true,document,independently_recomputed_hash:await resultDigest(projected)});
  } catch (error) {attempts.push({name,mutation,accepted:false,error:String(error)});}
}
const identities = Object.fromEntries([sourcePath,'src/services/hashService.ts','src/services/wasmEngine/loadWasmEngine.ts'].map(p=>p.startsWith('src/')?'apps/desktop/'+p:p).concat(['core/reporting/result_export/src/derivative.rs','apps/desktop/public/wasm-engine/open_pipe_stress_operation_applier.js','apps/desktop/public/wasm-engine/open_pipe_stress_operation_applier_bg.wasm']).map(p=>[p,sha(readFileSync(path.join(root,p)))]));
const receipt = {scope:'Actual TypeScript constructor and validator using existing operation-WASM canonical hash functions; no Rust validator execution and no Current claim.',node:process.version,esbuild:require('esbuild/package.json').version,compiled_in_memory_sha256:sha(compiledBytes),identities,model,source,origin,base,attempts};
writeFileSync(path.join(output,'legacy_derivative_probe.json'),JSON.stringify(receipt,null,2)+'\n');
console.log(JSON.stringify(attempts.map(a=>({name:a.name,accepted:a.accepted,error:a.error,hash_match:a.document?.result_envelope.reproducibility.derivative_hash.value===a.independently_recomputed_hash})),null,2));

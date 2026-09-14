import {mkdirSync,writeFileSync} from "node:fs";
import path from "node:path";
import {describe,it,expect} from 'vitest';
import modelJson from '../../../../../fixtures/product_preview/invented_preview_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_mechanics_result.json';
import metadataFixtures from '../../../../../fixtures/results/invented/result_export_v0_2.json';
import annotationFixtures from '../../../../../fixtures/results/invented/result_export_v0_2_rejections.json';
import type {PreviewModel,MechanicsResult} from '../../types';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,bindSourceResultDimensions} from '../../services/previewService';
import {buildCurrentResultExport,validateResultDocument,guardResultJson,resultDigest,resultSchemaVersion,deriveResultDocument,ref,derivativeProvenance,type JsonObject} from './resultExportAdapter';
import {canonicalJsonString} from '../../services/hashService';
async function current(){
 const model=structuredClone(modelJson) as PreviewModel,result=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'open_pipe_stress_product_physics',solver_version:'0.1.0',solver_build_ref:'open_pipe_stress_product_physics@0.1.0',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});return {model,result,inputManifest,analysisRun};
}
async function projection(row:any){
 const model={project:{id:'project:synthetic'}} as PreviewModel,source={run_id:'synthetic',model_ref:'project:synthetic',status:{mechanics:'MECHANICS_SOLVED'},results:[row]} as MechanicsResult;
 const origin={origin_id:'synthetic-projection-unattested',origin_class:'received_current_dimension_absent',qualification_ref:ref('synthetic_fixture','not-Current'),authentic_producer_available:false,received_carrier_checksum:{algorithm:'sha256',canonicalization:'openpipestress_jcs_ijson_v1',payload_scope:'received_current_dimension_absent_carrier',payload_ref:ref('synthetic_received_carrier','fixture'),value:await resultDigest(source)},original_producer_checksum:null,origin_limit:'pure projection not authenticated Current',actual_model_ref:ref('model_payload','project:synthetic'),mechanics_run_ref:ref('mechanics_run','synthetic'),request_model_ref:null,request_run_ref:null,request_alias_disclosure:null};
 const base={schema_version:'0.2.0',result_envelope:{schema_version:'0.2.0',envelope_id:'synthetic',run_ref:ref('analysis_run','synthetic'),provenance:derivativeProvenance,unit_system_ref:ref('unit_system','fixture'),result_sets:[{values:[]}],reproducibility:{}}};
 return deriveResultDocument(base,model,source,origin);
}
describe('qualified Current derivative export',()=>{
 it('binds current payload/carrier and preserves old hashes and observed declarations',async()=>{
  const args=await current(),before=JSON.stringify(args);const doc=await buildCurrentResultExport(args);expect(doc.schema_version).toBe('0.2.0');if(process.env.RESULTS_CONTRACT_OUTPUT_DIR){const dir=process.env.RESULTS_CONTRACT_OUTPUT_DIR;mkdirSync(dir,{recursive:true});writeFileSync(path.join(dir,'current-enriched.document.json'),JSON.stringify(doc,null,2));writeFileSync(path.join(dir,'current-enriched.received.json'),JSON.stringify(args.result,null,2));writeFileSync(path.join(dir,'current-enriched.model.json'),JSON.stringify(args.model,null,2));writeFileSync(path.join(dir,'current-enriched.analysis-run.json'),JSON.stringify(args.analysisRun,null,2));}expect(doc.result_envelope.row_accounting).toHaveLength(args.result.results.length);expect(JSON.stringify(args)).toBe(before);
  expect(doc.result_envelope.reproducibility.raw_source_hashes).toEqual([]);expect(doc.result_envelope.reproducibility.source_origin_bindings[0].original_producer_checksum).toBeNull();
  const payload=structuredClone(doc);delete payload.result_envelope.reproducibility.derivative_hash;expect(doc.result_envelope.reproducibility.derivative_hash.value).toBe(await resultDigest(payload));
  for(const a of doc.result_envelope.row_accounting){const row=args.result.results[a.source_row_index];expect(a.received_carrier_row_checksum.value).toBe(await resultDigest(row));const ann=doc.result_envelope.source_annotations[a.source_row_index];expect(ann.observed_carrier_dimension).toEqual({present:true,value:row.dimension});}
 });
 it('dimension-absent received carrier remains distinct from authentic producer',async()=>{
  const args=await current();args.result=structuredClone(resultJson) as unknown as MechanicsResult;expect(args.result.results.every(row=>!Object.hasOwn(row,'dimension'))).toBe(true);const doc=await buildCurrentResultExport(args);expect(doc.result_envelope.reproducibility.source_origin_bindings[0].origin_class).toBe('received_current_dimension_absent');expect(doc.result_envelope.reproducibility.source_origin_bindings[0].authentic_producer_available).toBe(false);
  if(process.env.RESULTS_CONTRACT_OUTPUT_DIR){const dir=process.env.RESULTS_CONTRACT_OUTPUT_DIR;mkdirSync(dir,{recursive:true});writeFileSync(path.join(dir,'current-absent.document.json'),JSON.stringify(doc,null,2));writeFileSync(path.join(dir,'current-absent.received.json'),JSON.stringify(args.result,null,2));}
 });
 it('rejects missing proof, same-id changed model, stale run, changed value and mixed/null/empty/wrong dimensions',async()=>{
  const args=await current();await expect(buildCurrentResultExport({...args,inputManifest:null})).rejects.toThrow('UNAVAILABLE');
  const changed=structuredClone(args);(changed.model as any).unknown_authored_metadata='same-id-substitution';await expect(buildCurrentResultExport(changed)).rejects.toThrow('MODEL_PAYLOAD_MISMATCH');
  const stale=structuredClone(args);stale.result.run_id+='stale';await expect(buildCurrentResultExport(stale)).rejects.toThrow('RUN_BINDING');
  const value=structuredClone(args);value.result.results[0].value+=1;await expect(buildCurrentResultExport(value)).rejects.toThrow('RESULT_BINDING');
  for(const dimension of [null,'',7,'wrong']){const bad=structuredClone(args);(bad.result.results[0] as any).dimension=dimension;await expect(buildCurrentResultExport(bad)).rejects.toThrow();}
  const mixed=structuredClone(args);delete mixed.result.results[0].dimension;await expect(buildCurrentResultExport(mixed)).rejects.toThrow('MIXED');
 });
 it('rejects checksum scope/type/algorithm/canonicalization/ref tampering',async()=>{
  const args=await current();const edits:Array<(a:typeof args)=>void>=[a=>{(a.analysisRun.analysis_run.hashes[0] as any).algorithm='TBD'},a=>{a.analysisRun.analysis_run.hashes[0].payload_ref.object_type='Wrong'},a=>{a.analysisRun.analysis_run.hashes[1].canonicalization='wrong'},a=>{a.analysisRun.analysis_run.result_refs[0].hash_refs[0].payload_scope='wrong'},a=>{a.analysisRun.analysis_run.result_refs[0].hash_refs[0].payload_ref.ref='wrong'},a=>{(a.analysisRun.analysis_run.reproducibility.input_manifest_hashes[0] as any).payload_scope='wrong'},a=>{a.analysisRun.analysis_run.reproducibility.input_manifest_refs[0].object_type='Wrong'}];
  for(const edit of edits){const bad=structuredClone(args);edit(bad);await expect(buildCurrentResultExport(bad)).rejects.toThrow();}
 });
});
describe('portable independent projection fixtures and strict serializer',()=>{
 it('all60 semantic expectations and all28 exact annotations agree before any origin claim',async()=>{
  for(const f of metadataFixtures.fixtures){const doc=await projection(f.input_row);expect(doc.result_envelope.row_accounting[0].disposition).toBe(f.expected.disposition);expect(doc.result_envelope.source_annotations[0].metadata).toEqual(f.input_row.metadata??null);}
  for(const f of annotationFixtures.cases){const doc=await projection(f.received_row);expect(doc.result_envelope.row_accounting[0].disposition).toBe(f.expected.row_disposition);expect(doc.result_envelope.source_annotations[0].metadata).toEqual(f.expected.annotation_metadata);}
 });
 it('canonical authority orders numeric-looking/Unicode keys and normalizes negative zero',async()=>{expect(await canonicalJsonString({'2':2,'10':10,'𐀀':1,'\ue000':2,'é':'accent',zero:-0})).toBe('{"10":10,"2":2,"zero":0,"é":"accent","𐀀":1,"":2}');});
 it('rejects unsupported, hidden, sparse, extra, unsafe, malformed and cyclic JSON',()=>{
  const hidden={};Object.defineProperty(hidden,'hidden',{value:1});const extra:any[]=[1];(extra as any).named='omitted';const cycle:any={};cycle.self=cycle;
  for(const input of [undefined,NaN,Infinity,9007199254740992,BigInt(1),()=>1,Symbol(),new Date(),{bad:'\ud800'},hidden,extra,Array(2),cycle,{undefined:undefined}])expect(()=>guardResultJson(input)).toThrow();
 });
 it('rejects unknown/mixed versions preserving0.1 versus0.2 reader dispatch',()=>{expect(resultSchemaVersion({schema_version:'0.1.0',result_envelope:{schema_version:'0.1.0'}})).toBe('0.1.0');expect(()=>resultSchemaVersion({schema_version:'0.3.0',result_envelope:{schema_version:'0.3.0'}})).toThrow();expect(()=>resultSchemaVersion({schema_version:'0.1.0',result_envelope:{schema_version:'0.2.0'}})).toThrow();});
});

it('rejects ghost references, orphan witnesses and semantic tampering despite recomputed derivative digest',async()=>{
 const args=await current(),doc=await buildCurrentResultExport(args);
 for(const path of [['row_accounting',0,'target_ref','ref_id'],['row_accounting',0,'received_carrier_row_checksum','payload_scope'],['source_annotations',0,'source_origin_ref','ref_id'],['unit_preservation_witnesses',0,'source_quantity','unit'],['unit_preservation_witnesses',0,'target_row_checksum','algorithm'],['result_sets',0,'values',0,'dimension']]){
  const bad=structuredClone(doc);let value:any=bad.result_envelope;for(const key of path.slice(0,-1))value=value[key];value[path[path.length-1]]='tampered';delete bad.result_envelope.reproducibility.derivative_hash;const digest=await resultDigest(bad);bad.result_envelope.reproducibility.derivative_hash={algorithm:'sha256',canonicalization:'openpipestress_jcs_ijson_v1',payload_scope:'derivative_document_excludes_own_hash',payload_ref:ref('derivative_document',bad.result_envelope.envelope_id),value:digest};await expect(validateResultDocument(bad,args.result)).rejects.toThrow();
 }
});

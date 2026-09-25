import transport from "../../../../../fixtures/results/precision_transport_v0_3.json";
import { PRECISION_CONTRACT_ID } from "../results/numericalResultQuality";
import {mkdirSync,writeFileSync} from "node:fs";
import path from "node:path";
import {describe,it,expect,afterEach,vi} from 'vitest';
import {createNativeMechanicsReplay,nativeMechanicsReplayPair} from '../../test/nativeMechanicsReplay';
const invokeMock=vi.hoisted(()=>vi.fn());
vi.mock('@tauri-apps/api/core',()=>({invoke:invokeMock}));
afterEach(()=>{invokeMock.mockReset();delete (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__;});
import modelJson from '../../../../../fixtures/product_preview/invented_preview_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_mechanics_result.json';
import metadataFixtures from '../../../../../fixtures/results/invented/result_export_v0_2.json';
import annotationFixtures from '../../../../../fixtures/results/invented/result_export_v0_2_rejections.json';
import type {PreviewModel,MechanicsResult} from '../../types';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,bindSourceResultDimensions,runPreviewMechanics} from '../../services/previewService';
import {buildCurrentResultExport,validateResultDocument,guardResultJson,resultDigest,resultSchemaVersion,deriveResultDocument,ref,derivativeProvenance,type JsonObject} from './resultExportAdapter';
import {canonicalJsonString,canonicalSha256HexCheckedV1} from '../../services/hashService';
import {analysisRecordProjection,verifyAnalysisRunRecord} from '../../services/analysisRunCompatibility';
async function legacyDigest(value:unknown){
 const sort=(item:any):any=>Array.isArray(item)?item.map(sort):item&&typeof item==='object'?Object.fromEntries(Object.entries(item).sort(([a],[b])=>a.localeCompare(b)).map(([key,child])=>[key,sort(child)])):item;
 const digest=await crypto.subtle.digest('SHA-256',new TextEncoder().encode(JSON.stringify(sort(value))));return [...new Uint8Array(digest)].map(byte=>byte.toString(16).padStart(2,'0')).join('');
}
// Historical-format protocol controls only: copied rows and gate labels are
// never replayed as native output and never qualify Current or numerical accuracy.
function precision(result: MechanicsResult, model: PreviewModel): MechanicsResult {
 // Preserve every legacy warning and load reference while giving this mock's
 // diagnostics unique identities. The immutable imported fixture is untouched.
 result.diagnostics=result.diagnostics.map((diagnostic,index)=>({...diagnostic,id:`test:synthetic-diagnostic:${index}:${diagnostic.id ?? 'missing'}`}));
 result.schema_version='0.2.0';
 result.producer={component_name:'open_pipe_stress_product_physics',component_version:'0.2.0',semantic_contract_id:PRECISION_CONTRACT_ID};
 result.formulation_basis={profile_id:'product_preview_mechanics_v1',limitations:['bounded pressure/component/stress/support preview']};
 result.diagnostics.push({id:'test:gate',code:'TEST_GATE',severity:'info',message:'Synthetic consumer gate evidence; not native proof'});
 result.numerical_quality={value_representation:'finite_binary64',publication_quantization:'none',integrity_policy:'M03-INTEGRITY-v1',status:'checks_passed',cases:model.load_cases.map(c=>({basis_ref:{ref_type:'load_case',ref_id:c.id},structural_status:'passive_model_basis',solve_quality:'checks_passed',model_matrix_fidelity:'represented_equations_retained',accuracy_evidence:'not_claimed',evidence_refs:['test:gate']}))};
 return result;
}
async function referenceSession(){
 const model=structuredClone(modelJson) as PreviewModel,result=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'open_pipe_stress_product_physics',solver_version:'0.2.0',solver_build_ref:'open_pipe_stress_product_physics@0.2.0',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 precision(result,model);
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});return {model,result,inputManifest,analysisRun};
}
// Actual captured input/output pair through explicitly simulated IPC; this is a
// unit premise for Current gates, never an actual native UI qualification.
async function current(){
 const {model}=nativeMechanicsReplayPair();
 (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__={};
 invokeMock.mockImplementation(createNativeMechanicsReplay().invoke);
 const result=await runPreviewMechanics(model);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:result.producer!.component_name,solver_version:result.producer!.component_version,solver_build_ref:'open_pipe_stress_product_physics@0.2.0',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 return {model,result,inputManifest,analysisRun};
}
// Keep accepted historical carrier enum/scope vocabulary for protocol oracles.
// The fixture qualification ref and origin limit expressly withhold Current;
// this calls only the pure projector, never the qualified export entrypoint.
async function referenceProjection({model,result}:{model:PreviewModel;result:MechanicsResult}){
 const {base,origin}=await projectionBasis(result.results[0]);
 const enriched=result.results.every(row=>Object.hasOwn(row,'dimension'));
 origin.origin_id='reference-projection-unattested';
 origin.origin_class=enriched?'received_current_qualified_legacy_enriched':'received_current_dimension_absent';
 origin.qualification_ref=ref('synthetic_fixture','unqualified-reference-projection-not-Current');
 origin.origin_limit='Pure reference contract projection; no native invocation, Current, or qualified export claim';
 origin.actual_model_ref=ref('model_payload',model.project.id);origin.mechanics_run_ref=ref('mechanics_run',result.run_id);
 origin.received_carrier_checksum={...origin.received_carrier_checksum,payload_scope:enriched?'received_current_legacy_enriched_carrier':'received_current_dimension_absent_carrier',payload_ref:ref('synthetic_received_carrier',result.run_id),value:await resultDigest(result)};
 base.result_envelope.run_ref=ref('analysis_run',result.run_id);
 return deriveResultDocument(base,model,result,origin);
}
async function projectionBasis(row:any){
 const model={project:{id:'project:synthetic'}} as PreviewModel,source={schema_version:'0.1.0',run_id:'synthetic',model_ref:'project:synthetic',status:{mechanics:'MECHANICS_SOLVED'},results:[row]} as MechanicsResult;
 const origin={origin_id:'synthetic-projection-unattested',origin_class:'received_current_dimension_absent',qualification_ref:ref('synthetic_fixture','not-Current'),authentic_producer_available:false,received_carrier_checksum:{algorithm:'sha256',canonicalization:'openpipestress_jcs_ijson_v1',payload_scope:'received_current_dimension_absent_carrier',payload_ref:ref('synthetic_received_carrier','fixture'),value:await resultDigest(source)},original_producer_checksum:null,origin_limit:'pure projection not authenticated Current',actual_model_ref:ref('model_payload','project:synthetic'),mechanics_run_ref:ref('mechanics_run','synthetic'),request_model_ref:null,request_run_ref:null,request_alias_disclosure:null};
 const base={schema_version:'0.2.0',result_envelope:{schema_version:'0.2.0',envelope_id:'synthetic',run_ref:ref('analysis_run','synthetic'),provenance:derivativeProvenance,unit_system_ref:ref('unit_system','fixture'),result_sets:[{values:[]}],reproducibility:{}}};
 return {base,model,source,origin};
}
async function projection(row:any){
 const {base,model,source,origin}=await projectionBasis(row);
 return deriveResultDocument(base,model,source,origin);
}
async function legacyCurrent(){
 const args=await referenceSession(),run=args.analysisRun.analysis_run,legacy=args.result,ruleStatus=run.analysis_status.find(status=>['RULE_INPUTS_INCOMPLETE','USER_RULE_CHECKED','USER_RULE_FAILED'].includes(status))!;
 const recordPayload={run_id:legacy.run_id,model_ref:legacy.model_ref,status:{...legacy.status,rule_check:ruleStatus},load_basis_refs:run.load_basis_refs,result_ids:legacy.results.map(row=>row.id).sort(),diagnostic_ids:legacy.diagnostics.map(item=>item.id??'diagnostic:unknown').sort(),input_manifest_ref:args.inputManifest.manifest_ref,input_manifest_sha256:args.inputManifest.manifest_sha256,result_dimensions:legacy.results.map(row=>({result_id:row.id,dimension:row.dimension})).sort((a,b)=>a.result_id.localeCompare(b.result_id))};
 args.analysisRun.schema_version='0.1.0';
 run.hashes=[{algorithm:'sha256',canonicalization:'rfc8785_jcs',payload_ref:{object_type:'AnalysisRun',ref:run.run_id},payload_scope:'analysis_run_record',value:await legacyDigest(recordPayload)},{algorithm:'sha256',canonicalization:'rfc8785_jcs',payload_ref:{object_type:'ResultEnvelope',ref:`result-envelope:${legacy.run_id}`},payload_scope:'result_envelope',value:await legacyDigest(legacy)}];
 run.result_refs=await Promise.all(legacy.results.map(async row=>({...run.result_refs.find(item=>item.result_ref.ref===row.id)!,hash_refs:[{algorithm:'sha256',canonicalization:'rfc8785_jcs',payload_ref:{object_type:'Result',ref:row.id},payload_scope:'result_value',value:await legacyDigest(row)}]})));
 run.reproducibility.input_manifest_hashes[0].canonicalization='rfc8785_jcs';
 return args;
}
async function rehashV02ResultClaims(args:Awaited<ReturnType<typeof current>>){
 const run=args.analysisRun.analysis_run;
 for(const row of args.result.results){const resultRef=run.result_refs.find(item=>item.result_ref.ref===row.id)!;resultRef.hash_refs[0].value=await canonicalSha256HexCheckedV1(row);}
 run.hashes.find(hash=>hash.payload_scope==='received_result')!.value=await canonicalSha256HexCheckedV1(args.result);
 run.hashes.find(hash=>hash.payload_scope==='analysis_run_record')!.value=await canonicalSha256HexCheckedV1(analysisRecordProjection(args.analysisRun));
}
describe('reference projection contracts and simulated native Current boundaries',()=>{
 it('binds reference payload/carrier and preserves old hashes and observed declarations',async()=>{
  const args=await referenceSession(),before=JSON.stringify(args);const doc=await referenceProjection(args);expect(doc.schema_version).toBe('0.3.0');if(process.env.RESULTS_CONTRACT_OUTPUT_DIR){const dir=process.env.RESULTS_CONTRACT_OUTPUT_DIR;mkdirSync(dir,{recursive:true});writeFileSync(path.join(dir,'reference-enriched.document.json'),JSON.stringify(doc,null,2));writeFileSync(path.join(dir,'reference-enriched.received.json'),JSON.stringify(args.result,null,2));writeFileSync(path.join(dir,'reference-enriched.model.json'),JSON.stringify(args.model,null,2));writeFileSync(path.join(dir,'reference-enriched.analysis-run.json'),JSON.stringify(args.analysisRun,null,2));}expect(doc.result_envelope.row_accounting).toHaveLength(args.result.results.length);expect(JSON.stringify(args)).toBe(before);
  expect(doc.result_envelope.reproducibility.raw_source_hashes).toEqual([]);expect(doc.result_envelope.reproducibility.source_origin_bindings[0].original_producer_checksum).toBeNull();
  const payload=structuredClone(doc);delete payload.result_envelope.reproducibility.derivative_hash;expect(doc.result_envelope.reproducibility.derivative_hash.value).toBe(await resultDigest(payload));
  for(const a of doc.result_envelope.row_accounting){const row=args.result.results[a.source_row_index];expect(a.received_carrier_row_checksum.value).toBe(await resultDigest(row));const ann=doc.result_envelope.source_annotations[a.source_row_index];expect(ann.observed_carrier_dimension).toEqual({present:true,value:row.dimension});}
 });
 it('dimension-absent received carrier remains distinct from authentic producer',async()=>{
  const args=await referenceSession();args.result=precision(structuredClone(resultJson) as unknown as MechanicsResult,args.model);args.analysisRun=await buildAnalysisRunPreview(args.result,{inputManifest:args.inputManifest});expect(args.result.results.every(row=>!Object.hasOwn(row,'dimension'))).toBe(true);const doc=await referenceProjection(args);expect(doc.result_envelope.reproducibility.source_origin_bindings[0].origin_class).toBe('received_current_dimension_absent');expect(doc.result_envelope.reproducibility.source_origin_bindings[0].authentic_producer_available).toBe(false);
  if(process.env.RESULTS_CONTRACT_OUTPUT_DIR){const dir=process.env.RESULTS_CONTRACT_OUTPUT_DIR;mkdirSync(dir,{recursive:true});writeFileSync(path.join(dir,'reference-absent.document.json'),JSON.stringify(doc,null,2));writeFileSync(path.join(dir,'reference-absent.received.json'),JSON.stringify(args.result,null,2));}
 });
 it('dispatches raw 0.2 basis, discrete and diagnostic semantics without legacy enrichment',async()=>{
  const args=await referenceSession(),kinds=['modulus_basis_record','linear_solver_mode_basis','nonlinear_support_free_dof_work_residual'];args.result=precision(structuredClone(resultJson) as unknown as MechanicsResult,args.model);args.result.results=kinds.map(kind=>structuredClone(metadataFixtures.fixtures.find(f=>f.input_row.kind===kind)!.input_row)) as MechanicsResult['results'];args.analysisRun=await buildAnalysisRunPreview(args.result,{inputManifest:args.inputManifest});const before=JSON.stringify(args.result);
  expect(await verifyAnalysisRunRecord(args.analysisRun)).toBe('match');expect(args.result.results.every(row=>!Object.hasOwn(row,'dimension'))).toBe(true);const doc=await referenceProjection(args);expect(JSON.stringify(args.result)).toBe(before);expect(doc.result_envelope.row_accounting).toHaveLength(3);expect(doc.result_envelope.row_accounting.map((row:any)=>row.source_result_id)).toEqual(args.result.results.map(row=>row.id));expect(doc.result_envelope.row_accounting.every((row:any)=>row.disposition==='disclosed')).toBe(true);expect(doc.result_envelope.unit_preservation_witnesses).toHaveLength(0);
  const received=args.analysisRun.analysis_run.hashes.find(hash=>hash.payload_scope==='received_result')!;expect(received.value).toBe(await resultDigest(args.result));for(const [index,row] of args.result.results.entries()){expect(args.analysisRun.analysis_run.result_refs.find(item=>item.result_ref.ref===row.id)!.hash_refs[0].value).toBe(await resultDigest(row));expect(doc.result_envelope.source_annotations[index].observed_carrier_dimension).toEqual({present:false,value:null});}
 });
 it('admits the four accepted explicit legacy dimensions without replacing them with interpreted target dimensions',async()=>{
  const args=await referenceSession(),accepted=[
   {kind:'nonlinear_support_free_dof_work_residual',unit:'N*m',component:'free_dof_work_residual',dimension:'moment'},
   {kind:'nonlinear_support_final_displacement',unit:'rad',component:'nonlinear_support_final_displacement',dimension:'length'},
   {kind:'nonlinear_support_final_reaction',unit:'N*m',component:'nonlinear_support_final_reaction',dimension:'force'},
   {kind:'spring_hanger_user_input_review',unit:'N*m/rad',component:'variable_spring_hanger_stiffness',dimension:'linear_stiffness'},
  ];args.result.results=accepted.map(item=>{const fixture=metadataFixtures.fixtures.find(candidate=>candidate.input_row.kind===item.kind&&candidate.input_row.unit===item.unit&&candidate.input_row.metadata?.component===item.component)!;return {...structuredClone(fixture.input_row),dimension:item.dimension};}) as MechanicsResult['results'];args.analysisRun=await buildAnalysisRunPreview(args.result,{inputManifest:args.inputManifest});const before=JSON.stringify(args.result),doc=await referenceProjection(args);expect(JSON.stringify(args.result)).toBe(before);expect(doc.result_envelope.source_annotations.map((annotation:any)=>annotation.observed_carrier_dimension.value)).toEqual(accepted.map(item=>item.dimension));expect(doc.result_envelope.source_annotations.map((annotation:any)=>annotation.derivative_target_dimension)).toEqual([null,'angle','moment','rotational_stiffness']);
 });
 it('preserves established 0.1 legacy enrichment and hash verification',async()=>{
  const args=await legacyCurrent();args.result.schema_version='0.1.0';delete args.result.producer;delete args.result.numerical_quality;delete args.result.formulation_basis;const before=JSON.stringify(args),claims=JSON.stringify(args.analysisRun.analysis_run.hashes);await expect(buildCurrentResultExport(args)).rejects.toThrow('CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE');expect(JSON.stringify(args)).toBe(before);expect(JSON.stringify(args.analysisRun.analysis_run.hashes)).toBe(claims);
 });
 it('rejects missing proof, same-id changed model, stale run, changed value and mixed/null/empty/wrong dimensions',async()=>{
  const args=await current();await expect(buildCurrentResultExport({...args,inputManifest:null})).rejects.toThrow('UNAVAILABLE');
  const changed=structuredClone(args);(changed.model as any).unknown_authored_metadata='same-id-substitution';await expect(buildCurrentResultExport(changed)).rejects.toThrow('MODEL_PAYLOAD_MISMATCH');
  const stale=structuredClone(args);stale.result.run_id+='stale';await expect(buildCurrentResultExport(stale)).rejects.toThrow('RUN_BINDING');
  const value=structuredClone(args);value.result.results[0].value+=1;await expect(buildCurrentResultExport(value)).rejects.toThrow('RESULT_BINDING');
  for(const dimension of [null,'',7,'wrong']){const bad=structuredClone(args);(bad.result.results[0] as any).dimension=dimension;await expect(buildCurrentResultExport(bad)).rejects.toThrow();}
  const mixed=structuredClone(args);mixed.result.results[0].dimension='length';await expect(buildCurrentResultExport(mixed)).rejects.toThrow('MIXED');
 });
 it('rejects a rehashed 0.2 known explicit carrier dimension contradiction without mutating evidence',async()=>{
  const args=await referenceSession(),row=args.result.results.find(item=>item.kind==='displacement_magnitude')!;(row as any).dimension='stress';await rehashV02ResultClaims(args);expect(await verifyAnalysisRunRecord(args.analysisRun)).toBe('match');const before=JSON.stringify(args.result);await expect(buildCurrentResultExport(args)).rejects.toThrow('CURRENT_CARRIER_DIMENSION_CONTRADICTION');expect(JSON.stringify(args.result)).toBe(before);
 });
 it('rejects checksum scope/type/algorithm/canonicalization/ref tampering',async()=>{
  const args=await current();const edits:Array<(a:typeof args)=>void>=[a=>{(a.analysisRun.analysis_run.hashes[0] as any).algorithm='TBD'},a=>{a.analysisRun.analysis_run.hashes[0].payload_ref.object_type='Wrong'},a=>{a.analysisRun.analysis_run.hashes[1].canonicalization='wrong'},a=>{a.analysisRun.analysis_run.result_refs[0].hash_refs[0].payload_scope='wrong'},a=>{a.analysisRun.analysis_run.result_refs[0].hash_refs[0].payload_ref.ref='wrong'},a=>{(a.analysisRun.analysis_run.reproducibility.input_manifest_hashes[0] as any).payload_scope='wrong'},a=>{a.analysisRun.analysis_run.reproducibility.input_manifest_refs[0].object_type='Wrong'}];
  for(const edit of edits){const bad=structuredClone(args);edit(bad);await expect(buildCurrentResultExport(bad)).rejects.toThrow();}
 });
 it('rejects unknown analysis versions and strict known semantic contradictions',async()=>{
  const args=await current();(args.analysisRun as any).schema_version='0.4.0';await expect(buildCurrentResultExport(args)).rejects.toThrow('CURRENT_ANALYSIS_VERSION_UNSUPPORTED');
  for(const kind of ['modulus_basis_record','linear_solver_mode_basis','nonlinear_support_free_dof_work_residual']){const fixture=metadataFixtures.fixtures.find(f=>f.input_row.kind===kind)!;for(const [field,replacement] of [['unit',fixture.negative_unit.replacement],['component',fixture.negative_component!.replacement]] as const){const row=structuredClone(fixture.input_row) as any;if(field==='unit')row.unit=replacement;else row.metadata.component=replacement;await expect(projection(row)).rejects.toThrow('CONTRADICTION');}}
 });
});
describe('legacy derivative precision metadata exclusion', () => {
 const row = {id:'result:legacy:ux',kind:'global_nodal_displacement_x',value:0.125,unit:'mm',entity_ref:'node:legacy'};
 const fields = ['producer','numerical_quality','formulation_basis','semantic_contract_ref'] as const;
 const claims = [null, false, 0, '', [], {}, {claimed:'precision'}];
 it('preserves an unannotated legacy derivative and source without a Current claim', async () => {
  const basis = await projectionBasis(row), before = JSON.stringify(basis);
  const doc = await deriveResultDocument(basis.base,basis.model,basis.source,basis.origin);
  expect(doc.schema_version).toBe('0.2.0');
  for (const field of fields) expect(Object.hasOwn(doc.result_envelope,field)).toBe(false);
  await expect(validateResultDocument(doc,basis.source)).resolves.toBeUndefined();
  expect(JSON.stringify(basis)).toBe(before);
 });
 it.each(fields)('rejects any present %s in the constructor without deleting evidence', async field => {
  const basis = await projectionBasis(row);
  for (const value of claims) {
   const base: JsonObject = structuredClone(basis.base);base.result_envelope[field] = value;
   const before = JSON.stringify({base,source:basis.source,origin:basis.origin});
   await expect(deriveResultDocument(base,basis.model,basis.source,basis.origin)).rejects.toThrow('LEGACY_DERIVATIVE_METADATA_CONTRADICTION');
   expect(JSON.stringify({base,source:basis.source,origin:basis.origin})).toBe(before);
  }
 });
 it.each(fields)('rejects independently rehashed legacy documents carrying %s', async field => {
  const basis = await projectionBasis(row);
  const valid = await deriveResultDocument(basis.base,basis.model,basis.source,basis.origin);
  const sourceBefore = JSON.stringify(basis.source);
  for (const value of claims) {
   const bad = structuredClone(valid);bad.result_envelope[field] = value;
   const claim = bad.result_envelope.reproducibility.derivative_hash;
   delete bad.result_envelope.reproducibility.derivative_hash;
   // Real checked hash helper, not a fabricated digest or a validator stub.
   const digest = await canonicalSha256HexCheckedV1(bad);
   bad.result_envelope.reproducibility.derivative_hash = {...claim,value:digest};
   const projected = structuredClone(bad);delete projected.result_envelope.reproducibility.derivative_hash;
   expect(await resultDigest(projected)).toBe(digest);
   const before = JSON.stringify(bad);
   await expect(validateResultDocument(bad,basis.source)).rejects.toThrow('LEGACY_DERIVATIVE_METADATA_CONTRADICTION');
   expect(JSON.stringify(bad)).toBe(before);
  }
  expect(JSON.stringify(basis.source)).toBe(sourceBefore);
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
 it('rejects unknown/mixed versions preserving0.1 versus0.2 reader dispatch',()=>{expect(resultSchemaVersion({schema_version:'0.1.0',result_envelope:{schema_version:'0.1.0'}})).toBe('0.1.0');expect(resultSchemaVersion({schema_version:'0.3.0',result_envelope:{schema_version:'0.3.0'}})).toBe('0.3.0');expect(()=>resultSchemaVersion({schema_version:'0.4.0',result_envelope:{schema_version:'0.4.0'}})).toThrow();expect(()=>resultSchemaVersion({schema_version:'0.1.0',result_envelope:{schema_version:'0.2.0'}})).toThrow();});
});

it('rejects ghost references, orphan witnesses and semantic tampering despite recomputed derivative digest',async()=>{
 const args=await current(),doc=await buildCurrentResultExport(args);
 for(const path of [['row_accounting',0,'target_ref','ref_id'],['row_accounting',0,'received_carrier_row_checksum','payload_scope'],['source_annotations',0,'source_origin_ref','ref_id'],['unit_preservation_witnesses',0,'source_quantity','unit'],['unit_preservation_witnesses',0,'target_row_checksum','algorithm'],['result_sets',0,'values',0,'dimension']]){
  const bad=structuredClone(doc);let value:any=bad.result_envelope;for(const key of path.slice(0,-1))value=value[key];value[path[path.length-1]]='tampered';delete bad.result_envelope.reproducibility.derivative_hash;const digest=await resultDigest(bad);bad.result_envelope.reproducibility.derivative_hash={algorithm:'sha256',canonicalization:'openpipestress_jcs_ijson_v1',payload_scope:'derivative_document_excludes_own_hash',payload_ref:ref('derivative_document',bad.result_envelope.envelope_id),value:digest};await expect(validateResultDocument(bad,args.result)).rejects.toThrow();
 }
});

it('preserves every shared nonzero f64 vector through canonical hash and derivative JSON',async()=>{
 const bits=(value:number)=>{const bytes=new DataView(new ArrayBuffer(8));bytes.setFloat64(0,value,false);return bytes.getBigUint64(0,false).toString(16).padStart(16,'0');};
 const args=await referenceSession();args.result.results=transport.vectors.map(v=>({id:v.id,kind:'global_nodal_rotation_x',value:JSON.parse(v.decimal),unit:'rad',entity_ref:'node:transport'}));
 args.analysisRun=await buildAnalysisRunPreview(args.result,{inputManifest:args.inputManifest});
 const before=await resultDigest(args.result),doc=await referenceProjection(args),decoded=JSON.parse(await canonicalJsonString(doc));
 expect(await resultDigest(JSON.parse(await canonicalJsonString(args.result)))).toBe(before);
 for(const vector of transport.vectors){const row=decoded.result_envelope.result_sets[0].values.find((r:any)=>r.result_id===vector.id);expect(bits(row.magnitude)).toBe(vector.bits_hex);expect(row.unit).toBe('rad');}
 expect(decoded.result_envelope.producer).toEqual(args.result.producer);expect(decoded.result_envelope.numerical_quality).toEqual(args.result.numerical_quality);expect(decoded.result_envelope.formulation_basis).toEqual(args.result.formulation_basis);
 await validateResultDocument(decoded,args.result);
 decoded.result_envelope.numerical_quality.status='not_assessed';await expect(validateResultDocument(decoded,args.result)).rejects.toThrow('SOURCE_NUMERICAL_METADATA_MISMATCH');
});

it('rejects rehashed Current analysis diagnostics and claimed basis using independent model evidence',async()=>{
 const args=await current();
 for(const mutate of [
  (run:typeof args.analysisRun.analysis_run)=>{run.diagnostics=[];},
  (run:typeof args.analysisRun.analysis_run)=>{run.load_basis_refs=[{object_type:'LoadCase',ref:'fabricated'}];},
 ]) {
  const bad=structuredClone(args);mutate(bad.analysisRun.analysis_run);
  bad.analysisRun.analysis_run.hashes.find(h=>h.payload_scope==='analysis_run_record')!.value=await canonicalSha256HexCheckedV1(analysisRecordProjection(bad.analysisRun));
  expect(await verifyAnalysisRunRecord(bad.analysisRun)).toBe('match');
  await expect(buildCurrentResultExport(bad)).rejects.toThrow('ANALYSIS_SOURCE_');
 }
});


it('requires the registered exact source object for Current export in a native transport unit simulation',async()=>{
 const args=await current(),before=JSON.stringify(args.result),doc=await buildCurrentResultExport(args);
 expect(doc.schema_version).toBe('0.3.0');
 expect(doc.result_envelope.contract_evidence).toEqual(args.result.contract_evidence);
 expect(doc.result_envelope.producer).toEqual(args.result.producer);
 expect(JSON.stringify(args.result)).toBe(before);
 await expect(buildCurrentResultExport({...args,result:structuredClone(args.result)})).rejects.toThrow('CURRENT_NATIVE_INVOCATION_REQUIRED');
 const reference=await referenceSession();
 await expect(buildCurrentResultExport(reference)).rejects.toThrow('CURRENT_NATIVE_INVOCATION_REQUIRED');
});

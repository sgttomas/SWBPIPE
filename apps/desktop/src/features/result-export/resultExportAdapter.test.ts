import transport from "../../../../../fixtures/canonical_hash/binary64_transport_vectors.json";
import previewModelJson from "../../../../../fixtures/model_operations/precision_connected_ui_model.json";
import previewText from "../../../../../fixtures/results/preview_physics_connected_sparse.json?raw";
import {mkdirSync,writeFileSync} from "node:fs";
import path from "node:path";
import {describe,it,expect,afterEach,vi} from 'vitest';
import {createNativeMechanicsReplay,nativeMechanicsReplayPair} from '../../test/nativeMechanicsReplay';
const invokeMock=vi.hoisted(()=>vi.fn());
vi.mock('@tauri-apps/api/core',()=>({invoke:invokeMock}));
afterEach(()=>{invokeMock.mockReset();delete (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__;});
import resultJson from '../../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json';
import type {PreviewModel,MechanicsResult} from '../../types';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,runPreviewMechanics} from '../../services/previewService';
import {buildCurrentResultExport,validateResultDocument,guardResultJson,resultDigest,resultSchemaVersion,deriveResultDocument,ref,derivativeProvenance,type JsonObject} from './resultExportAdapter';
import {canonicalJsonString,canonicalSha256HexCheckedV1} from '../../services/hashService';
import {analysisRecordProjection,verifyAnalysisRunRecord} from '../../services/analysisRunCompatibility';
import {minimalPreviewStatement} from '../../test-support/retiredPrecision';
// B1 retired precision-1, whose synthetic header gave the demo carrier's legacy rows a
// current record. A reference session is now the connected preview-physics-2 producer
// output and its model, read from its bytes; it is never replayed as native output.
function previewReference(): MechanicsResult { return JSON.parse(previewText) as MechanicsResult; }
async function referenceSession(){
 const model=structuredClone(previewModelJson) as unknown as PreviewModel,result=previewReference();
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'open_pipe_stress_product_physics',solver_version:'0.2.0',solver_build_ref:'open_pipe_stress_product_physics@0.2.0',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});return {model,result,inputManifest,analysisRun};
}
// Actual captured input/output pair through explicitly simulated IPC; this is a
// unit premise for Current gates, never an actual native UI qualification.
// B7b: the Current pair is preview-physics-2's; the exact route's pressure-2 (the v3
// twin of the retired physics-1) is recorded but has no desktop export.
async function current(){
 const {model}=nativeMechanicsReplayPair('sparse_interactive',{profile:'preview'});
 (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__={};
 invokeMock.mockImplementation(createNativeMechanicsReplay({profile:'preview'}).invoke);
 const result=await runPreviewMechanics(model);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:result.producer!.component_name,solver_version:result.producer!.component_version,solver_build_ref:'open_pipe_stress_product_physics@0.2.0',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 return {model,result,inputManifest,analysisRun};
}
// The received-carrier origin vocabulary (b7-plan E2: dimension-absent only).
// The fixture qualification ref and origin limit expressly withhold Current;
// this calls only the pure projector, never the qualified export entrypoint.
async function referenceProjection({model,result}:{model:PreviewModel;result:MechanicsResult}){
 const {base,origin}=await projectionBasis(result.results[0]);
 origin.origin_id='reference-projection-unattested';
 origin.origin_class='received_current_dimension_absent';
 origin.qualification_ref=ref('synthetic_fixture','unqualified-reference-projection-not-Current');
 origin.origin_limit='Pure reference contract projection; no native invocation, Current, or qualified export claim';
 origin.actual_model_ref=ref('model_payload',model.project.id);origin.mechanics_run_ref=ref('mechanics_run',result.run_id);
 origin.received_carrier_checksum={...origin.received_carrier_checksum,payload_scope:'received_current_dimension_absent_carrier',payload_ref:ref('synthetic_received_carrier',result.run_id),value:await resultDigest(result)};
 base.result_envelope.run_ref=ref('analysis_run',result.run_id);
 return deriveResultDocument(base,model,result,origin);
}
async function projectionBasis(row:any){
 // B1 and E2: the projected row's source is a current preview-physics-2 statement
 // (the retired precision-1 header and the 0.1.0 carrier are refused by name).
 const model={project:{id:'project:synthetic'}} as PreviewModel,source=minimalPreviewStatement({run_id:'synthetic',model_ref:'project:synthetic',rows:[row],cases:[]});
 const origin={origin_id:'synthetic-projection-unattested',origin_class:'received_current_dimension_absent',qualification_ref:ref('synthetic_fixture','not-Current'),authentic_producer_available:false,received_carrier_checksum:{algorithm:'sha256',canonicalization:'openpipestress_jcs_ijson_v1',payload_scope:'received_current_dimension_absent_carrier',payload_ref:ref('synthetic_received_carrier','fixture'),value:await resultDigest(source)},original_producer_checksum:null,origin_limit:'pure projection not authenticated Current',actual_model_ref:ref('model_payload','project:synthetic'),mechanics_run_ref:ref('mechanics_run','synthetic'),request_model_ref:null,request_run_ref:null,request_alias_disclosure:null};
 const base={schema_version:'0.3.0',result_envelope:{schema_version:'0.3.0',envelope_id:'synthetic',run_ref:ref('analysis_run','synthetic'),provenance:derivativeProvenance,unit_system_ref:ref('unit_system','fixture'),result_sets:[{values:[]}],reproducibility:{}}};
 return {base,model,source,origin};
}
async function projection(row:any){
 const {base,model,source,origin}=await projectionBasis(row);
 return deriveResultDocument(base,model,source,origin);
}
async function rehashV02ResultClaims(args:Awaited<ReturnType<typeof current>>){
 const run=args.analysisRun.analysis_run;
 for(const row of args.result.results){const resultRef=run.result_refs.find(item=>item.result_ref.ref===row.id)!;resultRef.hash_refs[0].value=await canonicalSha256HexCheckedV1(row);}
 run.hashes.find(hash=>hash.payload_scope==='received_result')!.value=await canonicalSha256HexCheckedV1(args.result);
 run.hashes.find(hash=>hash.payload_scope==='analysis_run_record')!.value=await canonicalSha256HexCheckedV1(analysisRecordProjection(args.analysisRun));
}
describe('reference projection contracts and simulated native Current boundaries',()=>{
 it('a carrier declaring row dimensions is not a dimension-absent carrier (b7-plan E2)',async()=>{
  const args=await referenceSession();args.result.results=args.result.results.map(row=>({...row,dimension:'length'}));const before=JSON.stringify(args.result);
  await expect(referenceProjection(args)).rejects.toThrow('CARRIER_DIMENSION_PRESENT');expect(JSON.stringify(args.result)).toBe(before);
 });
 it('dimension-absent received carrier remains distinct from authentic producer',async()=>{
  const args=await referenceSession();args.result=previewReference();args.analysisRun=await buildAnalysisRunPreview(args.result,{inputManifest:args.inputManifest});expect(args.result.results.every(row=>!Object.hasOwn(row,'dimension'))).toBe(true);const doc=await referenceProjection(args);expect(doc.result_envelope.reproducibility.source_origin_bindings[0].origin_class).toBe('received_current_dimension_absent');expect(doc.result_envelope.reproducibility.source_origin_bindings[0].authentic_producer_available).toBe(false);
  if(process.env.RESULTS_CONTRACT_OUTPUT_DIR){const dir=process.env.RESULTS_CONTRACT_OUTPUT_DIR;mkdirSync(dir,{recursive:true});writeFileSync(path.join(dir,'reference-absent.document.json'),JSON.stringify(doc,null,2));writeFileSync(path.join(dir,'reference-absent.received.json'),JSON.stringify(args.result,null,2));}
 });
 it('dispatches raw basis and discrete semantics without dimension enrichment',async()=>{
  const args=await referenceSession(),kinds=['linear_solver_mode_basis','nonlinear_support_active_set_iteration_count','nonlinear_support_active_set_state_code'];args.result=minimalPreviewStatement({run_id:args.result.run_id,model_ref:args.result.model_ref,cases:[],rows:kinds.map(kind=>{const row=structuredClone((resultJson as unknown as MechanicsResult).results.find(candidate=>candidate.kind===kind)!) as any;if(row.basis_ref)row.basis_ref.ref_id=args.model.load_cases[0].id;return row;}) as MechanicsResult['results']});args.analysisRun=await buildAnalysisRunPreview(args.result,{inputManifest:args.inputManifest});const before=JSON.stringify(args.result);
  expect(await verifyAnalysisRunRecord(args.analysisRun)).toBe('match');expect(args.result.results.every(row=>!Object.hasOwn(row,'dimension'))).toBe(true);const doc=await referenceProjection(args);expect(JSON.stringify(args.result)).toBe(before);expect(doc.result_envelope.row_accounting).toHaveLength(3);expect(doc.result_envelope.row_accounting.map((row:any)=>row.source_result_id)).toEqual(args.result.results.map(row=>row.id));expect(doc.result_envelope.row_accounting.every((row:any)=>row.disposition==='disclosed')).toBe(true);expect(doc.result_envelope.unit_preservation_witnesses).toHaveLength(0);
  const received=args.analysisRun.analysis_run.hashes.find(hash=>hash.payload_scope==='received_result')!;expect(received.value).toBe(await resultDigest(args.result));for(const [index,row] of args.result.results.entries()){expect(args.analysisRun.analysis_run.result_refs.find(item=>item.result_ref.ref===row.id)!.hash_refs[0].value).toBe(await resultDigest(row));expect(doc.result_envelope.source_annotations[index].observed_carrier_dimension).toEqual({present:false,value:null});}
 });
 it('refuses a retired 0.1.0 carrier without changing its evidence (b7-plan E2)',async()=>{
  const args=await referenceSession();args.result.schema_version='0.1.0';delete args.result.producer;delete args.result.numerical_quality;delete args.result.formulation_basis;const before=JSON.stringify(args);await expect(buildCurrentResultExport(args)).rejects.toThrow();expect(JSON.stringify(args)).toBe(before);
  await expect(referenceProjection(args)).rejects.toThrow('SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED');
 });
 it('rejects missing proof, same-id changed model, stale run, changed value and any declared dimension',async()=>{
  const args=await current();await expect(buildCurrentResultExport({...args,inputManifest:null})).rejects.toThrow('UNAVAILABLE');
  const changed=structuredClone(args);(changed.model as any).unknown_authored_metadata='same-id-substitution';await expect(buildCurrentResultExport(changed)).rejects.toThrow('MODEL_PAYLOAD_MISMATCH');
  const stale=structuredClone(args);stale.result.run_id+='stale';await expect(buildCurrentResultExport(stale)).rejects.toThrow('RUN_BINDING');
  const value=structuredClone(args);value.result.results[0].value+=1;await expect(buildCurrentResultExport(value)).rejects.toThrow('RESULT_BINDING');
  for(const dimension of [null,'',7,'wrong']){const bad=structuredClone(args);(bad.result.results[0] as any).dimension=dimension;await expect(buildCurrentResultExport(bad)).rejects.toThrow();}
  const declared=structuredClone(args);declared.result.results[0].dimension='length';await expect(buildCurrentResultExport(declared)).rejects.toThrow('CURRENT_CARRIER_DIMENSION_PRESENT');
 });
 it('rejects a rehashed 0.2 known explicit carrier dimension contradiction without mutating evidence',async()=>{
  const args=await referenceSession(),row=args.result.results.find(item=>item.kind==='displacement_magnitude')!;(row as any).dimension='stress';await rehashV02ResultClaims(args);expect(await verifyAnalysisRunRecord(args.analysisRun)).toBe('match');const before=JSON.stringify(args.result);// A declared dimension on a current statement is refused by name (B1: no longer a precision-1 reference read at its historical standing; E2: the mixed-carrier dimension reading went with the 0.1.0 carrier).
  await expect(buildCurrentResultExport(args)).rejects.toThrow('CURRENT_CARRIER_DIMENSION_PRESENT');expect(JSON.stringify(args.result)).toBe(before);
 });
 it('rejects checksum scope/type/algorithm/canonicalization/ref tampering',async()=>{
  const args=await current();const edits:Array<(a:typeof args)=>void>=[a=>{(a.analysisRun.analysis_run.hashes[0] as any).algorithm='TBD'},a=>{a.analysisRun.analysis_run.hashes[0].payload_ref.object_type='Wrong'},a=>{a.analysisRun.analysis_run.hashes[1].canonicalization='wrong'},a=>{a.analysisRun.analysis_run.result_refs[0].hash_refs[0].payload_scope='wrong'},a=>{a.analysisRun.analysis_run.result_refs[0].hash_refs[0].payload_ref.ref='wrong'},a=>{(a.analysisRun.analysis_run.reproducibility.input_manifest_hashes[0] as any).payload_scope='wrong'},a=>{a.analysisRun.analysis_run.reproducibility.input_manifest_refs[0].object_type='Wrong'}];
  for(const edit of edits){const bad=structuredClone(args);edit(bad);await expect(buildCurrentResultExport(bad)).rejects.toThrow();}
 });
 it('rejects unknown analysis versions and strict known semantic contradictions',async()=>{
  const args=await current();(args.analysisRun as any).schema_version='0.4.0';await expect(buildCurrentResultExport(args)).rejects.toThrow('CURRENT_ANALYSIS_VERSION_UNSUPPORTED');
  for(const kind of ['linear_solver_mode_basis','nonlinear_support_free_dof_work_residual']){const input=(resultJson as unknown as MechanicsResult).results.find(row=>row.kind===kind)!;for(const [field,replacement] of [['unit','Pa'],['component','contradictory_source_component']] as const){const row=structuredClone(input) as any;if(field==='unit')row.unit=replacement;else row.metadata.component=replacement;await expect(projection(row)).rejects.toThrow('CONTRADICTION');}}
 });
});
describe('strict serializer and version dispatch',()=>{
 it('canonical authority orders numeric-looking/Unicode keys and normalizes negative zero',async()=>{expect(await canonicalJsonString({'2':2,'10':10,'𐀀':1,'\ue000':2,'é':'accent',zero:-0})).toBe('{"10":10,"2":2,"zero":0,"é":"accent","𐀀":1,"":2}');});
 it('rejects unsupported, hidden, sparse, extra, unsafe, malformed and cyclic JSON',()=>{
  const hidden={};Object.defineProperty(hidden,'hidden',{value:1});const extra:any[]=[1];(extra as any).named='omitted';const cycle:any={};cycle.self=cycle;
  for(const input of [undefined,NaN,Infinity,9007199254740992,BigInt(1),()=>1,Symbol(),new Date(),{bad:'\ud800'},hidden,extra,Array(2),cycle,{undefined:undefined}])expect(()=>guardResultJson(input)).toThrow();
 });
 it('admits only the one result-export version (b7-plan E2)',()=>{expect(resultSchemaVersion({schema_version:'0.3.0',result_envelope:{schema_version:'0.3.0'}})).toBe('0.3.0');for(const [outer,inner] of [['0.1.0','0.1.0'],['0.2.0','0.2.0'],['0.4.0','0.4.0'],['0.3.0','0.2.0'],['0.2.0','0.3.0']])expect(()=>resultSchemaVersion({schema_version:outer,result_envelope:{schema_version:inner}})).toThrow('RESULT_VERSION_UNSUPPORTED_OR_MIXED');});
});

it('rejects ghost references, orphan witnesses and semantic tampering despite recomputed derivative digest',async()=>{
 const args=await current(),doc=await buildCurrentResultExport(args);
 for(const path of [['row_accounting',0,'target_ref','ref_id'],['row_accounting',0,'received_carrier_row_checksum','payload_scope'],['source_annotations',0,'source_origin_ref','ref_id'],['unit_preservation_witnesses',0,'source_quantity','unit'],['unit_preservation_witnesses',0,'target_row_checksum','algorithm'],['result_sets',0,'values',0,'dimension']]){
  const bad=structuredClone(doc);let value:any=bad.result_envelope;for(const key of path.slice(0,-1))value=value[key];value[path[path.length-1]]='tampered';delete bad.result_envelope.reproducibility.derivative_hash;const digest=await resultDigest(bad);bad.result_envelope.reproducibility.derivative_hash={algorithm:'sha256',canonicalization:'openpipestress_jcs_ijson_v1',payload_scope:'derivative_document_excludes_own_hash',payload_ref:ref('derivative_document',bad.result_envelope.envelope_id),value:digest};await expect(validateResultDocument(bad,args.result)).rejects.toThrow();
 }
});

it('preserves every shared nonzero f64 vector through canonical hash and derivative JSON',async()=>{
 const bits=(value:number)=>{const bytes=new DataView(new ArrayBuffer(8));bytes.setFloat64(0,value,false);return bytes.getBigUint64(0,false).toString(16).padStart(16,'0');};
 const args=await referenceSession();args.result=minimalPreviewStatement({run_id:args.result.run_id,model_ref:args.result.model_ref,cases:[],rows:transport.vectors.map(v=>({id:v.id,kind:'global_nodal_rotation_x',value:JSON.parse(v.decimal),unit:'rad',entity_ref:'node:transport'}))});
 args.analysisRun=await buildAnalysisRunPreview(args.result,{inputManifest:args.inputManifest});
 const before=await resultDigest(args.result),doc=await referenceProjection(args),decoded=JSON.parse(await canonicalJsonString(doc));
 expect(await resultDigest(JSON.parse(await canonicalJsonString(args.result)))).toBe(before);
 for(const vector of transport.vectors){const row=decoded.result_envelope.result_sets[0].values.find((r:any)=>r.result_id===vector.id);expect(bits(row.magnitude)).toBe(vector.bits_hex);expect(row.unit).toBe('rad');}
 expect(decoded.result_envelope.producer).toEqual(args.result.producer);expect(decoded.result_envelope.numerical_quality).toEqual(args.result.numerical_quality);expect(decoded.result_envelope.formulation_basis).toEqual(args.result.formulation_basis);
 await validateResultDocument(decoded,args.result);
 decoded.result_envelope.numerical_quality.status='checks_passed';await expect(validateResultDocument(decoded,args.result)).rejects.toThrow('SOURCE_NUMERICAL_METADATA_MISMATCH');
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
 // A reference session has no native invocation (B1: no longer a precision-1 statement).
 await expect(buildCurrentResultExport(reference)).rejects.toThrow('CURRENT_NATIVE_INVOCATION_REQUIRED');
});

it('exports a registered preview-physics-2 Current result with its contract evidence',async()=>{
 for(const profile of ['preview'] as const){
  const {model}=nativeMechanicsReplayPair('sparse_interactive',{profile});
  (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__={};
  invokeMock.mockImplementation(createNativeMechanicsReplay({profile}).invoke);
  const result=await runPreviewMechanics(model);
  const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:result.producer!.component_name,solver_version:result.producer!.component_version,solver_build_ref:'open_pipe_stress_product_physics@0.2.0',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
  const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
  const doc=await buildCurrentResultExport({model,result,inputManifest,analysisRun});
  expect(doc.schema_version).toBe('0.3.0');
  expect(doc.result_envelope.contract_evidence).toEqual(result.contract_evidence);
  expect(doc.result_envelope.semantic_contract_ref.ref_id).toBe('openpipestress.result_semantics/0.3.0/preview-physics-2');
  // N-A: no notice text is written into the exported document.
  expect(JSON.stringify(doc)).not.toContain('Rule checks cannot bind');
  await validateResultDocument(doc,result);
 }
});

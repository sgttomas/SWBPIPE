import { buildAnalysisRunV02, modelLoadBasisRefs, analysisRecordProjection, verifyAnalysisRunRecord } from "../../services/analysisRunCompatibility";
import transport from "../../../../../fixtures/results/precision_transport_v0_3.json";
import { PRECISION_CONTRACT_ID } from "../results/numericalResultQuality";
import {resultSemantics} from '../results/resultSemantics';
import {afterEach,it,expect,vi} from 'vitest';
import {act,fireEvent,render,screen,waitFor} from '@testing-library/react';
import {mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import path from 'node:path';
import modelJson from '../../../../../fixtures/product_preview/invented_preview_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_mechanics_result.json';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,bindSourceResultDimensions} from '../../services/previewService';
import {buildStressNeutralExportPacket,validateStressNeutralExportPacket,precisionStressRow} from './StressNeutralExportPanel';
import {StressNeutralExportPanel} from './StressNeutralExportPanel';
import {canonicalSha256HexCheckedV1,canonicalJsonCheckedV1} from '../../services/hashService';
import type {PreviewModel,MechanicsResult} from '../../types';
import {isNativeResultSaveRuntime,saveNativeResultJson} from '../result-export/nativeResultSave';
vi.mock('../result-export/nativeResultSave',()=>({isNativeResultSaveRuntime:vi.fn(()=>false),saveNativeResultJson:vi.fn()}));
afterEach(()=>{vi.mocked(isNativeResultSaveRuntime).mockReturnValue(false);vi.mocked(saveNativeResultJson).mockReset();});
it('retains every native-shaped source row and hash while deriving 828 semantic witnesses and two diagnostic-work withholdings',async()=>{
 const model=modelJson as PreviewModel,result=structuredClone(resultJson) as unknown as MechanicsResult;
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunV02(result,inputManifest),before=JSON.stringify({result,analysisRun});const packet=await buildStressNeutralExportPacket({model,result,analysisRun});
 expect(packet.schema_version).toBe('0.2.0');expect(packet.manifest.package_members).toHaveLength(9);expect(packet.manifest.checksums).toHaveLength(9);expect(packet.package_checksum.payload_scope).toBe('complete_package_excluding_self_checksum');
 expect(packet.export_profile).toMatchObject({profile_id:'ops.stress_neutral.v2',profile_version:'0.2.0'});expect(packet.manifest.export_profile_ref).toEqual({object_type:'StressNeutralExportProfile',ref:'ops.stress_neutral.v2'});
 expect(packet.result_rows).toHaveLength(result.results.length);expect(packet.csv_text.trim().split('\n')).toHaveLength(result.results.length+1);
 expect(packet.unit_preservation_witnesses).toHaveLength(828);
 const withheld=packet.diagnostics.filter((d:any)=>d.code==='SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK');expect(withheld).toHaveLength(2);expect(new Set(withheld.map((d:any)=>d.source.ref))).toEqual(new Set(['result:nonlinear-support:free-dof-work-residual','result:loadcase:load-L-200:nonlinear-support:free-dof-work-residual']));expect(packet.validation_ready).toBe(false);
 const aggregate=packet.diagnostics.find((d:any)=>d.code==='SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE');expect(aggregate.message).toContain('2 retained rows');expect(aggregate.message).toContain('828 rows');expect(packet.diagnostics.some((d:any)=>d.code==='SN-UNIT-DIMENSION-MISSING')).toBe(false);expect(packet.validation_report.checks.find((check:any)=>check.check_id==='unit_preservation_witness_per_row')).toMatchObject({check_status:'blocking',blocking_count:2,diagnostic_count:2});
 expect(packet.loss_report.find((entry:any)=>entry.category==='exported').reason).toContain('830 received numerical rows');expect(packet.loss_report.find((entry:any)=>entry.category==='exported').reason).toContain('828 rows have accepted semantic-contract dimension witnesses and 2 rows');
 expect(packet.diagnostics.some((d:any)=>d.code==='SN-DESKTOP-PREVIEW-HASH-TBD')).toBe(false);expect(packet.export_profile.boundary_notes.some((note:string)=>note.includes('does not emit canonical package member hashes'))).toBe(false);
 for(const row of packet.result_rows){const original=result.results.find(r=>r.id===row.result_id)!;expect(row.value).toBe(original.value);expect(row.unit).toBe(original.unit);if(resultSemantics(original)?.family==='rotation' && resultSemantics(original)?.category==='physical_quantity'){expect(row.dimension).toBe('angle');expect(row.result_family).toBe('rotation');}}
 for(const w of packet.unit_preservation_witnesses){const row=result.results.find(r=>r.id===w.result_id)!;expect(w.source_quantity.dimension).toBe(resultSemantics(row)!.derivative_target_dimension);expect(w.target_quantity).toEqual(w.source_quantity);}
 expect(result.results.every(row=>!("dimension" in row))).toBe(true);expect(packet.export_profile.source_basis_refs).toContainEqual({object_type:'ExternalReference',ref:'fixtures/results/semantic_contract_v0_2.json'});expect(packet.unit_system_disclosure.decision_basis_refs).toContainEqual({object_type:'ExternalReference',ref:'fixtures/results/semantic_contract_v0_2.json'});
 expect(packet.export_profile.boundary_notes.some((note:string)=>note.includes('no absent raw dimension is claimed'))).toBe(true);expect(JSON.stringify({result,analysisRun})).toBe(before);
 await expect(validateStressNeutralExportPacket(packet)).resolves.toBeUndefined();const tampered=structuredClone(packet);tampered.result_rows[0].value+=1;await expect(validateStressNeutralExportPacket(tampered)).rejects.toThrow('MEMBER-CHECKSUM-MISMATCH');
 if(process.env.RESULTS_CONTRACT_OUTPUT_DIR){mkdirSync(process.env.RESULTS_CONTRACT_OUTPUT_DIR,{recursive:true});writeFileSync(path.join(process.env.RESULTS_CONTRACT_OUTPUT_DIR,'stress-neutral.packet.json'),JSON.stringify(packet,null,2));}
});

it('categorizes unknown, missing-semantic and contradictory rows separately from diagnostic work',async()=>{
 const model=modelJson as PreviewModel,baseResult=structuredClone(resultJson) as unknown as MechanicsResult;
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const analysisRun=await buildAnalysisRunV02(baseResult,inputManifest);
 const cases:[string,(row:MechanicsResult['results'][number])=>void][]=[
  ['SN-UNIT-WITNESS-WITHHELD-UNKNOWN-SEMANTIC',row=>{row.kind='unknown_native_quantity';}],
  ['SN-UNIT-WITNESS-WITHHELD-MISSING-SEMANTIC',row=>{delete (row as any).metadata.component;}],
  ['SN-UNIT-WITNESS-WITHHELD-CONTRADICTION',row=>{row.unit='Pa';}],
 ];
 for(const [code,mutate] of cases){const result=structuredClone(baseResult);const source=result.results.find(row=>row.kind==='element_local_axial_force')!;mutate(source);const before=structuredClone(result);const packet=await buildStressNeutralExportPacket({model,result,analysisRun});expect(packet.diagnostics.filter((item:any)=>item.source?.ref===source.id && item.code===code)).toHaveLength(1);expect(packet.diagnostics.filter((item:any)=>item.source?.ref===source.id && item.code==='SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK')).toHaveLength(0);expect(packet.unit_preservation_witnesses.some((item:any)=>item.result_id===source.id)).toBe(false);expect(result).toEqual(before);await expect(validateStressNeutralExportPacket(packet)).resolves.toBeUndefined();}
});

it('uses accepted semantics for a known row without rewriting a legacy dimension observation',async()=>{
 const model=modelJson as PreviewModel,result=structuredClone(resultJson) as unknown as MechanicsResult;const source=result.results.find(row=>row.kind==='element_local_axial_force')!;source.dimension='stress';const before=structuredClone(result);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const analysisRun=await buildAnalysisRunV02(structuredClone(resultJson) as unknown as MechanicsResult,inputManifest);const packet=await buildStressNeutralExportPacket({model,result,analysisRun});const row=packet.result_rows.find((item:any)=>item.result_id===source.id),witness=packet.unit_preservation_witnesses.find((item:any)=>item.result_id===source.id);expect(row.dimension).toBe('force');expect(witness.source_quantity.dimension).toBe('force');expect(packet.diagnostics.some((item:any)=>item.source?.ref===source.id&&item.code==='SN-UNIT-WITNESS-WITHHELD-CONTRADICTION')).toBe(false);expect(result).toEqual(before);expect(source.dimension).toBe('stress');
});

it('constructs and materializes a validated packet from an explicitly bound authentic native carrier',async()=>{
 const resultPath=process.env.RESULTS_NATIVE_CARRIER_PATH,modelPath=process.env.RESULTS_NATIVE_MODEL_PATH,analysisPath=process.env.RESULTS_NATIVE_ANALYSIS_PATH;
 if(!resultPath||!modelPath||!analysisPath)return;
 const rawResult=readFileSync(resultPath,'utf8'),rawModel=readFileSync(modelPath,'utf8'),rawAnalysis=readFileSync(analysisPath,'utf8');
 const result=JSON.parse(rawResult) as MechanicsResult,model=JSON.parse(rawModel) as PreviewModel,analysisRun=JSON.parse(rawAnalysis);
 expect(result.results).toHaveLength(830);expect(result.results.every(row=>!('dimension' in row))).toBe(true);
 const before=JSON.stringify({model,result,analysisRun});const packet=await buildStressNeutralExportPacket({model,result,analysisRun});
 expect(packet.result_rows).toHaveLength(830);expect(packet.unit_preservation_witnesses).toHaveLength(828);expect(packet.diagnostics.filter((item:any)=>item.code==='SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK')).toHaveLength(2);expect(JSON.stringify({model,result,analysisRun})).toBe(before);await expect(validateStressNeutralExportPacket(packet)).resolves.toBeUndefined();
 if(process.env.RESULTS_CONTRACT_OUTPUT_DIR){mkdirSync(process.env.RESULTS_CONTRACT_OUTPUT_DIR,{recursive:true});const materialized=JSON.stringify(packet);writeFileSync(path.join(process.env.RESULTS_CONTRACT_OUTPUT_DIR,'authentic-native-stress-neutral.packet.json'),materialized);expect(JSON.parse(materialized)).toEqual(packet);}
});

async function rawSha(text:string){const digest=await crypto.subtle.digest('SHA-256',new TextEncoder().encode(text));return [...new Uint8Array(digest)].map(x=>x.toString(16).padStart(2,'0')).join('');}
async function rehash(packet:any){
 const payloads:any={'stress_neutral_results.csv':packet.csv_text,'result_rows.json':packet.result_rows,'unit_system_disclosure.json':packet.unit_system_disclosure,'unit_preservation_witnesses.json':packet.unit_preservation_witnesses,'stable_id_map.json':packet.stable_id_map,'loss_report.json':packet.loss_report,'validation_report.json':packet.validation_report,'diagnostics.json':packet.diagnostics};
 const non=packet.manifest.checksums.filter((x:any)=>x.payload_ref.ref!=='manifest.json');for(const c of non)c.value=c.payload_ref.ref.endsWith('.csv')?await rawSha(payloads[c.payload_ref.ref]):await canonicalSha256HexCheckedV1(payloads[c.payload_ref.ref]);
 const seed={manifest_id:packet.manifest.manifest_id,source_result_ref:packet.source_result_ref,source_run_ref:packet.source_run_ref,source_model_ref:packet.source_model_ref,received_source_checksums:packet.received_source_checksums,unresolved_assumption_refs:packet.unresolved_assumption_refs,reproducibility_refs:packet.reproducibility_refs,export_profile_ref:packet.manifest.export_profile_ref,boundary_notes:packet.manifest.boundary_notes,member_checksums:non,diagnostics:packet.diagnostics};
 const m=packet.manifest.checksums.find((x:any)=>x.payload_ref.ref==='manifest.json');m.value=await canonicalSha256HexCheckedV1(seed);const by=new Map(packet.manifest.checksums.map((x:any)=>[x.payload_ref.ref,x]));for(const member of packet.manifest.package_members)member.checksum=structuredClone(by.get(member.filename));const projection=structuredClone(packet);delete projection.package_checksum;packet.package_checksum.value=await canonicalSha256HexCheckedV1(projection);
}

it('rejects relational tamper even after all covered hashes are recomputed',async()=>{
 const model=modelJson as PreviewModel,result=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const analysisRun=await buildAnalysisRunV02(result,inputManifest);const base=await buildStressNeutralExportPacket({model,result,analysisRun});
 const cases:[(p:any)=>void,string][]=[[(p)=>p.unit_preservation_witnesses[0].result_id='wrong','UNIT-WITNESS-BINDING'],[(p)=>p.unit_preservation_witnesses.pop(),'WITNESS-CATEGORY-ACCOUNTING'],[(p)=>p.export_profile.source_basis_refs=p.export_profile.source_basis_refs.filter((item:any)=>item.ref!=='fixtures/results/semantic_contract_v0_2.json'),'SEMANTIC-CONTRACT-BINDING'],[(p)=>p.loss_report[0].target_artifact_ref.ref='wrong','LOSS-REPORT-BINDING'],[(p)=>p.source_result_ref.ref='wrong','SOURCE-RESULT-REF-UNBOUND'],[(p)=>{p.result_rows[1].result_id=p.result_rows[0].result_id;p.result_rows[1].canonical_ref=structuredClone(p.result_rows[0].canonical_ref);p.result_rows[1].source_result_ref=structuredClone(p.result_rows[0].source_result_ref);},'RESULT-ROW-IDENTITY'],[(p)=>p.stable_id_map[1].canonical_ref=structuredClone(p.stable_id_map[0].canonical_ref),'STABLE-ID-MAP-BINDING'],[(p)=>p.stable_id_map.pop(),'STABLE-ID-MAP-BINDING'],[(p)=>p.csv_text=p.csv_text.replace(',mm,length,',',cm,length,',1),'CSV-ROW-BINDING'],[(p)=>p.validation_report.validation_status='passed','VALIDATION-STATUS-BINDING'],[(p)=>{p.validation_report.checks[0].check_status='blocking';p.validation_report.checks[0].blocking_count=0;},'VALIDATION-STATUS-BINDING'],[(p)=>p.schema_conformant=false,'SCHEMA-CONFORMANCE-CLAIM'],[(p)=>p.privacy.private_payload_embedded=true,'PRIVACY-BOUNDARY'],[(p)=>p.professional_boundary.software_makes_approval_claim=true,'PROFESSIONAL-BOUNDARY']];
 for(const [mutate,error] of cases){const packet=structuredClone(base);mutate(packet);await rehash(packet);await expect(validateStressNeutralExportPacket(packet)).rejects.toThrow(error);}
});

it('rejects duplicate missing path checksum metadata member divergence and package metadata',async()=>{
 const model=modelJson as PreviewModel,result=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const analysisRun=await buildAnalysisRunV02(result,inputManifest);const base=await buildStressNeutralExportPacket({model,result,analysisRun});
 const cases:[(p:any)=>void,string][]=[[(p)=>p.manifest.checksums[0].algorithm='sha512','CHECKSUM-METADATA'],[(p)=>p.manifest.checksums[1].payload_ref.ref='../escape.csv','BIJECTION'],[(p)=>p.manifest.checksums.push(structuredClone(p.manifest.checksums[0])),'STRICT-INVENTORY'],[(p)=>p.manifest.checksums.pop(),'STRICT-INVENTORY'],[(p)=>p.manifest.package_members[0].checksum=structuredClone(p.manifest.checksums[1]),'MEMBER-CHECKSUM-BINDING'],[(p)=>p.package_checksum.payload_scope='member_payload','PACKAGE-CHECKSUM-METADATA']];
 for(const [mutate,error] of cases){const packet=structuredClone(base);mutate(packet);await expect(validateStressNeutralExportPacket(packet)).rejects.toThrow(error);}
});

it('does not publish a delayed packet after its source result is replaced',async()=>{
 const model=modelJson as PreviewModel,first=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);const second=structuredClone(first);second.run_id='run:replacement-stress-neutral';
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const firstRun=await buildAnalysisRunV02(first,inputManifest),secondRun=await buildAnalysisRunV02(second,inputManifest);
 const original=crypto.subtle.digest.bind(crypto.subtle);let release!:()=>void;const gate=new Promise<void>(resolve=>{release=resolve;});let calls=0;
 vi.spyOn(crypto.subtle,'digest').mockImplementation(async (algorithm:any,data:any)=>{calls+=1;if(calls===1)await gate;return original(algorithm,data);});
 const view=render(<StressNeutralExportPanel model={model} result={first} analysisRun={firstRun}/>);await waitFor(()=>expect(calls).toBe(1));view.rerender(<StressNeutralExportPanel model={model} result={second} analysisRun={secondRun}/>);
 await waitFor(()=>expect(screen.getByTestId('stress-neutral-state-binding').textContent).toContain('result-envelope:run:replacement-stress-neutral'));release();await waitFor(()=>expect(calls).toBeGreaterThan(1));await Promise.resolve();expect(screen.getByTestId('stress-neutral-state-binding').textContent).toContain('result-envelope:run:replacement-stress-neutral');vi.restoreAllMocks();
});

it('saves native stress JSON only after validation and intent and invalidates a pending source generation',async()=>{
 vi.mocked(isNativeResultSaveRuntime).mockReturnValue(true);
 const model=modelJson as PreviewModel,first=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult),second=structuredClone(first);second.run_id='run:replacement-stress-neutral';
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const firstRun=await buildAnalysisRunV02(first,inputManifest),secondRun=await buildAnalysisRunV02(second,inputManifest);
 let finishOld!:(value:any)=>void;vi.mocked(saveNativeResultJson).mockImplementationOnce(()=>new Promise(resolve=>{finishOld=resolve;}));
 const view=render(<StressNeutralExportPanel model={model} result={first} analysisRun={firstRun}/>);
 const firstButton=await screen.findByRole('button',{name:/Package JSON/},{timeout:10_000});expect(firstButton).toBeDisabled();expect(saveNativeResultJson).not.toHaveBeenCalled();
 fireEvent.click(screen.getByTestId('stress-neutral-export-link-local-private-intent'));await waitFor(()=>expect(firstButton).toBeEnabled());fireEvent.click(firstButton);
 expect(saveNativeResultJson).toHaveBeenCalledTimes(1);const firstRequest=vi.mocked(saveNativeResultJson).mock.calls[0][0];expect(firstRequest.screening.route_id).toBe('DOTH-FORMAT-003');expect(firstRequest.file_name).toMatch(/^openpipestress-preview-stress-neutral-[a-z0-9-]+\.json$/);expect(firstRequest.local_first.route_id).toBe('DOTH-FORMAT-003');
 view.rerender(<StressNeutralExportPanel model={model} result={second} analysisRun={secondRun}/>);await waitFor(()=>expect(screen.getByTestId('stress-neutral-state-binding')).toHaveTextContent('result-envelope:run:replacement-stress-neutral'));const secondButton=screen.getByRole('button',{name:/Package JSON/});expect(secondButton).toBeDisabled();
 await act(async()=>{finishOld({outcome:'saved',file_name:firstRequest.file_name,byte_count:10,replaced_existing:false,durability:'not_guaranteed',path_containment:'best_effort_non_adversarial'});});expect(secondButton).toBeDisabled();expect(screen.getByTestId('stress-neutral-export-link-native-save-status')).not.toHaveTextContent('Saved');fireEvent.click(screen.getByTestId('stress-neutral-export-link-local-private-intent'));await waitFor(()=>expect(secondButton).toBeEnabled());
 vi.mocked(saveNativeResultJson).mockResolvedValueOnce({outcome:'saved',file_name:'openpipestress-preview-stress-neutral-result-envelope-run-replacement-stress-neutral.json',byte_count:10,replaced_existing:false,durability:'not_guaranteed',path_containment:'best_effort_non_adversarial'});fireEvent.click(secondButton);await waitFor(()=>expect(saveNativeResultJson).toHaveBeenCalledTimes(2));expect(vi.mocked(saveNativeResultJson).mock.calls[1][0].file_name).toContain('replacement-stress-neutral');
});

it('binds precision v3 metadata and preserves shared f64 bits in CSV and JSON',async()=>{
 const model=structuredClone(modelJson) as PreviewModel,result=structuredClone(resultJson) as unknown as MechanicsResult;
 result.schema_version='0.2.0';result.producer={component_name:'open_pipe_stress_product_physics',component_version:'0.2.0',semantic_contract_id:PRECISION_CONTRACT_ID};
 result.formulation_basis={profile_id:'product_preview_mechanics_v1',limitations:['pressure/component/stress/support preview limitations retained']};
 // Explicit synthetic consumer semantics for transport vectors; no physical producer qualification.
 result.results=transport.vectors.map(v=>({id:v.id,kind:'global_nodal_rotation_x',entity_ref:'node:transport',value:JSON.parse(v.decimal),unit:'rad',metadata:{component:'nodal_rotation_x',coordinate_system:'global',location:'summary',basis:'synthetic consumer transport fixture',sign_convention:'synthetic signed scalar'}}));
 result.diagnostics=[{id:'gate:test',code:'SYNTHETIC_GATE',severity:'info',message:'Consumer fixture only'}];
 result.numerical_quality={value_representation:'finite_binary64',publication_quantization:'none',integrity_policy:'M03-INTEGRITY-v1',status:'checks_passed',cases:model.load_cases.map(c=>({basis_ref:{ref_type:'load_case',ref_id:c.id},structural_status:'passive_model_basis',solve_quality:'checks_passed',model_matrix_fidelity:'represented_equations_retained',accuracy_evidence:'not_claimed',evidence_refs:['gate:test']}))};
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:result.producer.component_name,solver_version:'0.2.0',solver_build_ref:'test',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 const before=JSON.stringify(result),packet=await buildStressNeutralExportPacket({model,result,analysisRun}),decoded=JSON.parse(JSON.stringify(packet));
 expect(packet.schema_version).toBe('0.3.0');expect(packet.export_profile.profile_id).toBe('ops.stress_neutral.v3');
 expect(packet.producer).toEqual(result.producer);expect(packet.numerical_quality).toEqual(result.numerical_quality);expect(packet.formulation_basis).toEqual(result.formulation_basis);
 expect(packet.source_carrier_checksum.value).toBe(await canonicalSha256HexCheckedV1(result));
 const bits=(value:number)=>{const bytes=new DataView(new ArrayBuffer(8));bytes.setFloat64(0,value,false);return bytes.getBigUint64(0,false).toString(16).padStart(16,'0');};
 for(const vector of transport.vectors){const row=decoded.result_rows.find((r:any)=>r.result_id===vector.id);expect(bits(row.value)).toBe(vector.bits_hex);const csv=decoded.csv_text.split('\n').find((r:string)=>r.startsWith(`${vector.id},`));expect(bits(Number(csv.split(',')[7]))).toBe(vector.bits_hex);}
 expect(JSON.stringify(result)).toBe(before);await validateStressNeutralExportPacket(decoded);
 await validateStressNeutralExportPacket(decoded,result,analysisRun,modelLoadBasisRefs(model));
 await expect(validateStressNeutralExportPacket(decoded,result)).resolves.toBeUndefined();
 if(process.env.RESULTS_CONTRACT_OUTPUT_DIR){
   const dir=process.env.RESULTS_CONTRACT_OUTPUT_DIR;mkdirSync(dir,{recursive:true});
   for(const [name,value] of Object.entries({'precision.source.json':result,'precision.analysis.json':analysisRun,'precision.model.json':model}))writeFileSync(path.join(dir,name),JSON.stringify(value,null,2));
   writeFileSync(path.join(dir,'precision.packet.json'),await canonicalJsonCheckedV1(packet));
 }
 const csvSpellingPacket=async(value:number,spelling:string)=>{
   const candidate=structuredClone(packet);candidate.result_rows[0].value=value;
   for(const w of candidate.unit_preservation_witnesses)if(w.result_id===candidate.result_rows[0].result_id){w.source_quantity.value=value;w.target_quantity.value=value;}
   const encode=(v:any)=>{const text=typeof v==='object'?v.ref:String(v);return /[",\n\r]/.test(text)?`"${text.replaceAll('"','""')}"`:text;};
   candidate.csv_text=candidate.export_profile.csv_columns.join(',')+'\n'+candidate.result_rows.map((r:any,i:number)=>candidate.export_profile.csv_columns.map((c:string)=>i===0&&c==='value'?spelling:encode(r[c])).join(',')).join('\n')+'\n';
   await rehash(candidate);return candidate;
 };
 for(const [value,spelling] of [[1e-7,'1e-07'],[1e-7,'1e-7'],[1e-7,'0.0000001'],[-3.308659368829149e-05,'-3.308659368829149e-05'],[-3.308659368829149e-05,'-0.00003308659368829149'],[0,'0e-999'],[0,'-0'],[Number.MIN_VALUE,'5e-324'],[1e-7,'"1e-07"']] as const){
   const candidate=await csvSpellingPacket(value,spelling),before=candidate.csv_text;
   await expect(validateStressNeutralExportPacket(candidate)).resolves.toBeUndefined();expect(candidate.csv_text).toBe(before);
 }
 for(const spelling of ['+0','00','0.',' .0','NaN','Infinity','-Inf','1e-999','1e999','9007199254740992','"0"junk','"0']) {
   const candidate=await csvSpellingPacket(0,spelling);
   await expect(validateStressNeutralExportPacket(candidate)).rejects.toThrow('SN-CSV-ROW-BINDING-MISMATCH');
 }
 for(const mutate of [(p:any)=>{p.csv_text+=p.csv_text.split('\n')[1]+'\n';},(p:any)=>{p.csv_text=p.csv_text.replace('result_id,','wrong_id,');},(p:any)=>{p.csv_text=p.csv_text.replace('\n',',extra\n');},(p:any)=>{p.csv_text=p.csv_text.replaceAll('\n','\r\n');},(p:any)=>{p.csv_text+='\n';},(p:any)=>{p.csv_text=p.csv_text.replace('node:transport','node:wrong');},(p:any)=>{p.csv_text=p.csv_text.replace('node:transport','node:é');}]) {
   const candidate=await csvSpellingPacket(0,'0');mutate(candidate);await rehash(candidate);
   await expect(validateStressNeutralExportPacket(candidate)).rejects.toThrow('SN-CSV-ROW-BINDING-MISMATCH');
 }
 const canonicalDecoded=JSON.parse(await canonicalJsonCheckedV1(packet));
 await expect(validateStressNeutralExportPacket(canonicalDecoded)).resolves.toBeUndefined();
 const wrongQuantity=structuredClone(canonicalDecoded);wrongQuantity.unit_preservation_witnesses[0].target_quantity.unit='wrong';await rehash(wrongQuantity);
 await expect(validateStressNeutralExportPacket(wrongQuantity)).rejects.toThrow('SN-UNIT-WITNESS-BINDING-MISMATCH');
 for(const value of [true,'1',null]) {
   const invalidSource=structuredClone(result);invalidSource.results[0].value=value as never;
   const invalidAnalysis=await buildAnalysisRunPreview(invalidSource,{inputManifest});
   await expect(buildStressNeutralExportPacket({model,result:invalidSource,analysisRun:invalidAnalysis})).rejects.toThrow('SN-PRECISION-ROW-VALUE-INVALID');
   const invalidPacket=structuredClone(packet);invalidPacket.result_rows[0].value=value;
   for(const witness of invalidPacket.unit_preservation_witnesses)if(witness.result_id===invalidPacket.result_rows[0].result_id){witness.source_quantity.value=value;witness.target_quantity.value=value;}
   const encode=(v:any)=>{const text=v!==null&&typeof v==='object'?v.ref:String(v);return /[",\n\r]/.test(text)?`"${text.replaceAll('"','""')}"`:text;};
   invalidPacket.csv_text=invalidPacket.export_profile.csv_columns.join(',')+'\n'+invalidPacket.result_rows.map((r:any)=>invalidPacket.export_profile.csv_columns.map((c:string)=>encode(r[c])).join(',')).join('\n')+'\n';
   await rehash(invalidPacket);
   await expect(validateStressNeutralExportPacket(invalidPacket)).rejects.toThrow('SN-PRECISION-ROW-VALUE-INVALID');
 }

 await expect(validateStressNeutralExportPacket(decoded,undefined,analysisRun)).rejects.toThrow('SN-PRECISION-ANALYSIS-SOURCE-REQUIRED');
 const imported=structuredClone(packet);
 imported.provenance={source_name:'Python style fixture exporter',source_location:'interop-fixture',source_license:'project-governed',review_status:'pending',professional_claim:false};
 imported.result_rows.reverse();
 imported.result_rows.forEach((r:any)=>{r.provenance=structuredClone(imported.provenance);});
 imported.unit_preservation_witnesses=imported.result_rows.map((r:any,index:number)=>({witness_id:`unit-witness:${index}`,source_row_index:index,result_id:r.result_id,source_quantity:{value:r.value,unit:r.unit,dimension:r.dimension},target_quantity:{value:r.value,unit:r.unit,dimension:r.dimension},conversion_performed:false,policy:'preserve_received_value_and_unit'}));
 imported.stable_id_map.reverse();
 imported.stable_id_map.forEach((m:any,index:number)=>{m.export_ref={object_type:"StressNeutralResultRow",ref:`python:export:${index}`};});
 const importedEncode=(v:any)=>{const text=typeof v==='object'?v.ref:String(v);return /[",\n\r]/.test(text)?`"${text.replaceAll('"','""')}"`:text;};
 imported.csv_text=imported.export_profile.csv_columns.join(',')+'\n'+imported.result_rows.map((r:any)=>imported.export_profile.csv_columns.map((c:string)=>importedEncode(r[c])).join(',')).join('\n')+'\n';
 await rehash(imported);
 await expect(validateStressNeutralExportPacket(imported,result)).resolves.toBeUndefined();
 await expect(validateStressNeutralExportPacket(imported,result,analysisRun,modelLoadBasisRefs(model))).resolves.toBeUndefined();
 const typedTargets=structuredClone(imported);
 typedTargets.stable_id_map[0].export_ref={object_type:'ImporterRow',ref:'shared-text'};
 typedTargets.stable_id_map[1].export_ref={object_type:'AnotherImporterRow',ref:'shared-text'};
 await rehash(typedTargets);
 await expect(validateStressNeutralExportPacket(typedTargets)).resolves.toBeUndefined();
 await expect(validateStressNeutralExportPacket(typedTargets,result)).resolves.toBeUndefined();
 const extraClaim=structuredClone(imported);
 extraClaim.received_source_checksums.push({algorithm:'sha256',canonicalization:'openpipestress_jcs_ijson_v1',payload_scope:'external_evidence',payload_ref:{object_type:'ExternalReference',ref:'opaque:retained'},value:'a'.repeat(64)});
 await rehash(extraClaim);
 await expect(validateStressNeutralExportPacket(extraClaim,result)).resolves.toBeUndefined();
 await expect(validateStressNeutralExportPacket(extraClaim,result,analysisRun,modelLoadBasisRefs(model))).resolves.toBeUndefined();
 for(const mutate of [(p:any)=>{p.received_source_checksums.push(structuredClone(p.source_carrier_checksum));},(p:any)=>{p.received_source_checksums.find((h:any)=>h.payload_scope==='received_result').value='b'.repeat(64);}]) {
   const badClaim=structuredClone(imported);mutate(badClaim);await rehash(badClaim);
   await expect(validateStressNeutralExportPacket(badClaim,result)).rejects.toThrow('SN-PRECISION-SOURCE-IDENTITY-MISMATCH');
 }

 for(const mutate of [(p:any)=>{p.unit_preservation_witnesses[0].witness_id='wrong';},(p:any)=>{p.unit_preservation_witnesses[1].witness_id=p.unit_preservation_witnesses[0].witness_id;}]) {
   const badWitness=structuredClone(imported);mutate(badWitness);await rehash(badWitness);
   await expect(validateStressNeutralExportPacket(badWitness)).rejects.toThrow('SN-UNIT-WITNESS-BINDING-MISMATCH');
 }

 for(const mutate of [(p:any)=>{p.stable_id_map[0].canonical_ref={...p.stable_id_map[0].canonical_ref,object_type:'Wrong'};},(p:any)=>{p.stable_id_map[0].export_ref.ref='';},(p:any)=>{p.stable_id_map[0].export_ref.object_type='';},(p:any)=>{p.stable_id_map[0].export_ref.extra=true;},(p:any)=>{delete p.stable_id_map[0].export_ref.ref;},(p:any)=>{p.stable_id_map[0].export_ref=null;},(p:any)=>{delete p.stable_id_map[0].export_ref.object_type;},(p:any)=>{p.stable_id_map[0].export_ref.ref=null;}]) {
   const badMap=structuredClone(imported),rowsBefore=structuredClone(badMap.result_rows);mutate(badMap);
   expect(badMap.result_rows).toEqual(rowsBefore);
   await rehash(badMap);
   await expect(validateStressNeutralExportPacket(badMap,result)).rejects.toThrow('SN-STABLE-ID-MAP-BINDING-MISMATCH');
 }


 for (const mutate of [
   (p:any)=>{p.source_model_ref.ref='wrong';},
   (p:any)=>{p.source_run_ref.ref='wrong';p.reproducibility_refs=[p.source_run_ref];},
   (p:any)=>{p.source_result_ref.ref='wrong';p.source_carrier_checksum.payload_ref.ref='wrong';p.received_source_checksums.find((h:any)=>h.payload_scope==='received_result').payload_ref.ref='wrong';},
   (p:any)=>{p.result_rows[0].load_case_ref={object_type:'LoadCase',ref:'wrong'};},
   (p:any)=>{p.result_rows[0].component_ref={object_type:'Node',ref:'node:wrong'};},
   (p:any)=>{p.result_rows[0].station_ref={object_type:'Station',ref:'wrong'};},
   (p:any)=>{p.result_rows[0].result_family='other';},
   (p:any)=>{p.result_rows[0].row_kind='diagnostic_work';},
   (p:any)=>{delete p.result_rows[0].station_ref;},
   (p:any)=>{p.result_rows[0].station_ref=null;},
   (p:any)=>{p.unit_preservation_witnesses[0].witness_id='wrong';},
 ]) {
   const tampered=structuredClone(packet);mutate(tampered);
   // Regenerate CSV safely even for intentionally malformed JSON references.
   const encode=(v:any)=>{const text=typeof v==='object'?(typeof v?.ref==='string'?v.ref:''):String(v??'');return /[",\n\r]/.test(text)?`"${text.replaceAll('"','""')}"`:text;};
   tampered.csv_text=packet.export_profile.csv_columns.join(',')+'\n'+tampered.result_rows.map((r:any)=>packet.export_profile.csv_columns.map((c:string)=>encode(r[c])).join(',')).join('\n')+'\n';
   await rehash(tampered);
   await expect(validateStressNeutralExportPacket(tampered,result,analysisRun,modelLoadBasisRefs(model))).rejects.toThrow('SN-PRECISION-');
 }
 // Case/punctuation IDs collide under filename sanitizing but never under .3 indexed map/witness IDs.
 const collisions=structuredClone(result);collisions.results=collisions.results.slice(0,4);collisions.results.forEach((row,i)=>row.id=['ROW:A','row:a','row/a','row?a'][i]);
 const collisionAnalysis=await buildAnalysisRunPreview(collisions,{inputManifest});
 const collisionPacket=await buildStressNeutralExportPacket({model,result:collisions,analysisRun:collisionAnalysis});
 expect(new Set(collisionPacket.unit_preservation_witnesses.map((w:any)=>w.witness_id)).size).toBe(4);
 expect(new Set(collisionPacket.stable_id_map.map((m:any)=>m.export_ref.ref)).size).toBe(4);

 const wrongModelRun=structuredClone(analysisRun);wrongModelRun.analysis_run.model_state_ref={object_type:'ModelState',ref:'state:other:preview'};
 wrongModelRun.analysis_run.hashes.find(h=>h.payload_scope==='analysis_run_record')!.value=await canonicalSha256HexCheckedV1(analysisRecordProjection(wrongModelRun));
 expect(await verifyAnalysisRunRecord(wrongModelRun)).toBe('match');
 await expect(buildStressNeutralExportPacket({model,result,analysisRun:wrongModelRun})).rejects.toThrow('ANALYSIS_SOURCE_MODEL_STATE_MISMATCH');
 const bad=structuredClone(packet);bad.producer.semantic_contract_id='unknown';await expect(validateStressNeutralExportPacket(bad)).rejects.toThrow('SN-PRECISION-CONTRACT-MISMATCH');
 const withheldSource=structuredClone(result);
 withheldSource.results=[{id:'unknown:interop',entity_ref:'node:interop',kind:'unknown_quantity',value:1,unit:'mm'}];
 const withheldAnalysis=await buildAnalysisRunPreview(withheldSource,{inputManifest});
 const withheldPacket=await buildStressNeutralExportPacket({model,result:withheldSource,analysisRun:withheldAnalysis});
 withheldPacket.provenance=structuredClone(imported.provenance);
 withheldPacket.result_rows[0].provenance=structuredClone(imported.provenance);
 for(const d of withheldPacket.diagnostics)if(d.code.startsWith('SN-UNIT-WITNESS-WITHHELD')||d.code==='SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE')d.provenance=structuredClone(imported.provenance);
 expect(withheldPacket.diagnostics.find((d:any)=>d.code==='SN-UNIT-WITNESS-WITHHELD-UNKNOWN-SEMANTIC').remediation).toBe('Resolve the source family, unit, dimension and correlation evidence before downstream physical interpretation.');
 await rehash(withheldPacket);
 await expect(validateStressNeutralExportPacket(withheldPacket,withheldSource)).resolves.toBeUndefined();
 const missingAggregate=structuredClone(withheldPacket);
 missingAggregate.diagnostics=missingAggregate.diagnostics.filter((d:any)=>d.code!=='SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE');
 await rehash(missingAggregate);
 await expect(validateStressNeutralExportPacket(missingAggregate,withheldSource)).rejects.toThrow('SN-PRECISION-WITNESS-BINDING-MISMATCH');
 withheldPacket.diagnostics.find((d:any)=>d.code==='SN-UNIT-WITNESS-WITHHELD-UNKNOWN-SEMANTIC').provenance={...imported.provenance,source_name:'unbound'};
 await rehash(withheldPacket);
 await expect(validateStressNeutralExportPacket(withheldPacket,withheldSource)).rejects.toThrow('SN-PRECISION-WITNESS-BINDING-MISMATCH');
 result.numerical_quality.status='not_assessed';await expect(buildStressNeutralExportPacket({model,result,analysisRun})).rejects.toThrow('SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE');
});

it('projects precision location and missing component without guessed semantics',()=>{
 const source=structuredClone(resultJson) as unknown as MechanicsResult;
 source.schema_version='0.2.0';source.producer={component_name:'open_pipe_stress_product_physics',component_version:'0.2.0',semantic_contract_id:PRECISION_CONTRACT_ID};
 source.formulation_basis={profile_id:'product_preview_mechanics_v1',limitations:['Synthetic consumer fixture']};source.numerical_quality={value_representation:'finite_binary64',publication_quantization:'none',integrity_policy:'M03-INTEGRITY-v1',status:'not_assessed',cases:[]};
 const row=structuredClone(source.results.find(r=>r.kind==='element_local_axial_force')!);
 for(const value of [0,-0,Number.MIN_VALUE,-Number.MIN_VALUE,1.0000000000000002])expect(precisionStressRow({...row,value},source).value).toBe(value);
 for(const value of [true,'1',null,NaN,Infinity,-Infinity])expect(()=>precisionStressRow({...row,value} as never,source)).toThrow('SN-PRECISION-ROW-VALUE-INVALID');

 for(const location of [undefined,null,'midspan','  ']) {
   row.metadata={...row.metadata,location} as never;
   expect(precisionStressRow(row,source).station_ref.ref).toBe(location??'summary');
 }
 for(const location of ['',0,false,{},[]]) {
   row.metadata={...row.metadata,location} as never;
   expect(()=>precisionStressRow(row,source)).toThrow('SN-PRECISION-ROW-LOCATION-INVALID');
 }
 row.metadata={...row.metadata,location:'summary'} as never;
 delete (row.metadata as any).component;
 expect(precisionStressRow(row,source)).toMatchObject({result_family:'other',dimension:'TBD',correlation_status:'unit_or_dimension_blocking_review_required'});
 (row.metadata as unknown as {component:string}).component='invalid';expect(()=>precisionStressRow(row,source)).toThrow('SOURCE_COMPONENT_CONTRADICTION');
});

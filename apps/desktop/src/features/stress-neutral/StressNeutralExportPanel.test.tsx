import {resultSemantics} from '../results/resultSemantics';
import {afterEach,it,expect,vi} from 'vitest';
import {act,fireEvent,render,screen,waitFor} from '@testing-library/react';
import {mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import path from 'node:path';
import modelJson from '../../../../../fixtures/product_preview/invented_preview_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_mechanics_result.json';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,bindSourceResultDimensions} from '../../services/previewService';
import {buildStressNeutralExportPacket,validateStressNeutralExportPacket} from './StressNeutralExportPanel';
import {StressNeutralExportPanel} from './StressNeutralExportPanel';
import {canonicalSha256HexCheckedV1} from '../../services/hashService';
import type {PreviewModel,MechanicsResult} from '../../types';
import {isNativeResultSaveRuntime,saveNativeResultJson} from '../result-export/nativeResultSave';
vi.mock('../result-export/nativeResultSave',()=>({isNativeResultSaveRuntime:vi.fn(()=>false),saveNativeResultJson:vi.fn()}));
afterEach(()=>{vi.mocked(isNativeResultSaveRuntime).mockReturnValue(false);vi.mocked(saveNativeResultJson).mockReset();});
it('retains every native-shaped source row and hash while deriving 828 semantic witnesses and two diagnostic-work withholdings',async()=>{
 const model=modelJson as PreviewModel,result=structuredClone(resultJson) as unknown as MechanicsResult;
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest}),before=JSON.stringify({result,analysisRun});const packet=await buildStressNeutralExportPacket({model,result,analysisRun});
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
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const analysisRun=await buildAnalysisRunPreview(baseResult,{inputManifest});
 const cases:[string,(row:MechanicsResult['results'][number])=>void][]=[
  ['SN-UNIT-WITNESS-WITHHELD-UNKNOWN-SEMANTIC',row=>{row.kind='unknown_native_quantity';}],
  ['SN-UNIT-WITNESS-WITHHELD-MISSING-SEMANTIC',row=>{delete (row as any).metadata.component;}],
  ['SN-UNIT-WITNESS-WITHHELD-CONTRADICTION',row=>{row.unit='Pa';}],
 ];
 for(const [code,mutate] of cases){const result=structuredClone(baseResult);const source=result.results.find(row=>row.kind==='element_local_axial_force')!;mutate(source);const before=structuredClone(result);const packet=await buildStressNeutralExportPacket({model,result,analysisRun});expect(packet.diagnostics.filter((item:any)=>item.source?.ref===source.id && item.code===code)).toHaveLength(1);expect(packet.diagnostics.filter((item:any)=>item.source?.ref===source.id && item.code==='SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK')).toHaveLength(0);expect(packet.unit_preservation_witnesses.some((item:any)=>item.result_id===source.id)).toBe(false);expect(result).toEqual(before);await expect(validateStressNeutralExportPacket(packet)).resolves.toBeUndefined();}
});

it('uses accepted semantics for a known row without rewriting a legacy dimension observation',async()=>{
 const model=modelJson as PreviewModel,result=structuredClone(resultJson) as unknown as MechanicsResult;const source=result.results.find(row=>row.kind==='element_local_axial_force')!;source.dimension='stress';const before=structuredClone(result);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const analysisRun=await buildAnalysisRunPreview(structuredClone(resultJson) as unknown as MechanicsResult,{inputManifest});const packet=await buildStressNeutralExportPacket({model,result,analysisRun});const row=packet.result_rows.find((item:any)=>item.result_id===source.id),witness=packet.unit_preservation_witnesses.find((item:any)=>item.result_id===source.id);expect(row.dimension).toBe('force');expect(witness.source_quantity.dimension).toBe('force');expect(packet.diagnostics.some((item:any)=>item.source?.ref===source.id&&item.code==='SN-UNIT-WITNESS-WITHHELD-CONTRADICTION')).toBe(false);expect(result).toEqual(before);expect(source.dimension).toBe('stress');
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
 const model=modelJson as PreviewModel,result=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});const base=await buildStressNeutralExportPacket({model,result,analysisRun});
 const cases:[(p:any)=>void,string][]=[[(p)=>p.unit_preservation_witnesses[0].result_id='wrong','UNIT-WITNESS-BINDING'],[(p)=>p.unit_preservation_witnesses.pop(),'WITNESS-CATEGORY-ACCOUNTING'],[(p)=>p.export_profile.source_basis_refs=p.export_profile.source_basis_refs.filter((item:any)=>item.ref!=='fixtures/results/semantic_contract_v0_2.json'),'SEMANTIC-CONTRACT-BINDING'],[(p)=>p.loss_report[0].target_artifact_ref.ref='wrong','LOSS-REPORT-BINDING'],[(p)=>p.source_result_ref.ref='wrong','SOURCE-RESULT-REF-UNBOUND'],[(p)=>{p.result_rows[1].result_id=p.result_rows[0].result_id;p.result_rows[1].canonical_ref=structuredClone(p.result_rows[0].canonical_ref);p.result_rows[1].source_result_ref=structuredClone(p.result_rows[0].source_result_ref);},'RESULT-ROW-IDENTITY'],[(p)=>p.stable_id_map[1].canonical_ref=structuredClone(p.stable_id_map[0].canonical_ref),'STABLE-ID-MAP-BINDING'],[(p)=>p.stable_id_map.pop(),'STABLE-ID-MAP-BINDING'],[(p)=>p.csv_text=p.csv_text.replace(',mm,length,',',cm,length,',1),'CSV-ROW-BINDING'],[(p)=>p.validation_report.validation_status='passed','VALIDATION-STATUS-BINDING'],[(p)=>{p.validation_report.checks[0].check_status='blocking';p.validation_report.checks[0].blocking_count=0;},'VALIDATION-STATUS-BINDING'],[(p)=>p.schema_conformant=false,'SCHEMA-CONFORMANCE-CLAIM'],[(p)=>p.privacy.private_payload_embedded=true,'PRIVACY-BOUNDARY'],[(p)=>p.professional_boundary.software_makes_approval_claim=true,'PROFESSIONAL-BOUNDARY']];
 for(const [mutate,error] of cases){const packet=structuredClone(base);mutate(packet);await rehash(packet);await expect(validateStressNeutralExportPacket(packet)).rejects.toThrow(error);}
});

it('rejects duplicate missing path checksum metadata member divergence and package metadata',async()=>{
 const model=modelJson as PreviewModel,result=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});const base=await buildStressNeutralExportPacket({model,result,analysisRun});
 const cases:[(p:any)=>void,string][]=[[(p)=>p.manifest.checksums[0].algorithm='sha512','CHECKSUM-METADATA'],[(p)=>p.manifest.checksums[1].payload_ref.ref='../escape.csv','BIJECTION'],[(p)=>p.manifest.checksums.push(structuredClone(p.manifest.checksums[0])),'STRICT-INVENTORY'],[(p)=>p.manifest.checksums.pop(),'STRICT-INVENTORY'],[(p)=>p.manifest.package_members[0].checksum=structuredClone(p.manifest.checksums[1]),'MEMBER-CHECKSUM-BINDING'],[(p)=>p.package_checksum.payload_scope='member_payload','PACKAGE-CHECKSUM-METADATA']];
 for(const [mutate,error] of cases){const packet=structuredClone(base);mutate(packet);await expect(validateStressNeutralExportPacket(packet)).rejects.toThrow(error);}
});

it('does not publish a delayed packet after its source result is replaced',async()=>{
 const model=modelJson as PreviewModel,first=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);const second=structuredClone(first);second.run_id='run:replacement-stress-neutral';
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const firstRun=await buildAnalysisRunPreview(first,{inputManifest}),secondRun=await buildAnalysisRunPreview(second,{inputManifest});
 const original=crypto.subtle.digest.bind(crypto.subtle);let release!:()=>void;const gate=new Promise<void>(resolve=>{release=resolve;});let calls=0;
 vi.spyOn(crypto.subtle,'digest').mockImplementation(async (algorithm:any,data:any)=>{calls+=1;if(calls===1)await gate;return original(algorithm,data);});
 const view=render(<StressNeutralExportPanel model={model} result={first} analysisRun={firstRun}/>);await waitFor(()=>expect(calls).toBe(1));view.rerender(<StressNeutralExportPanel model={model} result={second} analysisRun={secondRun}/>);
 await waitFor(()=>expect(screen.getByTestId('stress-neutral-state-binding').textContent).toContain('result-envelope:run:replacement-stress-neutral'));release();await waitFor(()=>expect(calls).toBeGreaterThan(1));await Promise.resolve();expect(screen.getByTestId('stress-neutral-state-binding').textContent).toContain('result-envelope:run:replacement-stress-neutral');vi.restoreAllMocks();
});

it('saves native stress JSON only after validation and intent and invalidates a pending source generation',async()=>{
 vi.mocked(isNativeResultSaveRuntime).mockReturnValue(true);
 const model=modelJson as PreviewModel,first=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult),second=structuredClone(first);second.run_id='run:replacement-stress-neutral';
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const firstRun=await buildAnalysisRunPreview(first,{inputManifest}),secondRun=await buildAnalysisRunPreview(second,{inputManifest});
 let finishOld!:(value:any)=>void;vi.mocked(saveNativeResultJson).mockImplementationOnce(()=>new Promise(resolve=>{finishOld=resolve;}));
 const view=render(<StressNeutralExportPanel model={model} result={first} analysisRun={firstRun}/>);
 const firstButton=await screen.findByRole('button',{name:/Package JSON/});expect(firstButton).toBeDisabled();expect(saveNativeResultJson).not.toHaveBeenCalled();
 fireEvent.click(screen.getByTestId('stress-neutral-export-link-local-private-intent'));await waitFor(()=>expect(firstButton).toBeEnabled());fireEvent.click(firstButton);
 expect(saveNativeResultJson).toHaveBeenCalledTimes(1);const firstRequest=vi.mocked(saveNativeResultJson).mock.calls[0][0];expect(firstRequest.screening.route_id).toBe('DOTH-FORMAT-003');expect(firstRequest.file_name).toMatch(/^openpipestress-preview-stress-neutral-[a-z0-9-]+\.json$/);expect(firstRequest.local_first.route_id).toBe('DOTH-FORMAT-003');
 view.rerender(<StressNeutralExportPanel model={model} result={second} analysisRun={secondRun}/>);await waitFor(()=>expect(screen.getByTestId('stress-neutral-state-binding')).toHaveTextContent('result-envelope:run:replacement-stress-neutral'));const secondButton=screen.getByRole('button',{name:/Package JSON/});expect(secondButton).toBeDisabled();
 await act(async()=>{finishOld({outcome:'saved',file_name:firstRequest.file_name,byte_count:10,replaced_existing:false,durability:'not_guaranteed',path_containment:'best_effort_non_adversarial'});});expect(secondButton).toBeDisabled();expect(screen.getByTestId('stress-neutral-export-link-native-save-status')).not.toHaveTextContent('Saved');fireEvent.click(screen.getByTestId('stress-neutral-export-link-local-private-intent'));await waitFor(()=>expect(secondButton).toBeEnabled());
 vi.mocked(saveNativeResultJson).mockResolvedValueOnce({outcome:'saved',file_name:'openpipestress-preview-stress-neutral-result-envelope-run-replacement-stress-neutral.json',byte_count:10,replaced_existing:false,durability:'not_guaranteed',path_containment:'best_effort_non_adversarial'});fireEvent.click(secondButton);await waitFor(()=>expect(saveNativeResultJson).toHaveBeenCalledTimes(2));expect(vi.mocked(saveNativeResultJson).mock.calls[1][0].file_name).toContain('replacement-stress-neutral');
});

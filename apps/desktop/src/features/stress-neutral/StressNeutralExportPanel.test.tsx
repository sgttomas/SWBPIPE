/** E3 ports packet/member/CSV/witness/data-boundary controls to the already
 * admitted current native replay. The producerless 0.1→0.2 positive packages
 * (including the optional 513-row legacy carrier) and arbitrary legacy-source
 * family examples no longer describe the current-only builder. Actual current
 * refusal, custody, source/hash and activation controls remain below. */
import { invoke } from "@tauri-apps/api/core";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
vi.mock("@tauri-apps/api/core",()=>({invoke:vi.fn()}));
import { modelLoadBasisRefs, analysisRecordProjection, verifyAnalysisRunRecord } from "../../services/analysisRunCompatibility";
import transport from "../../../../../fixtures/canonical_hash/binary64_transport_vectors.json";
import { minimalPreviewStatement } from "../../test-support/retiredPrecision";
import connectedPreview from "../../../../../fixtures/results/preview_physics_connected_sparse.json";
import {afterEach,it,expect,onTestFinished,vi} from 'vitest';
import {act,fireEvent,getConfig,render,screen,waitFor} from '@testing-library/react';
import {mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import path from 'node:path';
import {readRetainedPrecisionCorpus} from '../../test-support/retainedPrecisionCorpus';
import modelJson from '../../../../../fixtures/product_preview/invented_demo_model.json';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,runPreviewMechanics,loadBundledMechanicsReference} from '../../services/previewService';
import {buildStressNeutralExportPacket,validateStressNeutralExportPacket,validateStressNeutralCsv,precisionStressRow,strictWitnessDisposition} from './StressNeutralExportPanel';
import {StressNeutralExportPanel} from './StressNeutralExportPanel';
import {canonicalSha256HexCheckedV1,canonicalJsonCheckedV1} from '../../services/hashService';
import type {PreviewModel,MechanicsResult} from '../../types';
import {isNativeResultSaveRuntime,saveNativeResultJson} from '../result-export/nativeResultSave';
import {buildAnalysisRunV03,sourceBasisReference} from '../../services/analysisRunCompatibility';
import {checkedJsonText} from '../../services/hashService';
import {registerRetainedPrecision} from '../results/retainedPrecisionStanding';
import { numericalResultStanding as successorStanding } from '../results/resultStanding';
import {validateRetainedPrecision} from '../results/retainedPrecisionReader';
import {retainedClassDisclosure} from '../results/retainedPrecisionDisclosure';
import combinationSuccessorSparse from '../../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json';
import combinationSuccessorDense from '../../../../../fixtures/results/retained_precision_combination_successor_dense_scrutiny.json';
vi.mock('../result-export/nativeResultSave',()=>({isNativeResultSaveRuntime:vi.fn(()=>false),saveNativeResultJson:vi.fn()}));
afterEach(()=>{vi.restoreAllMocks();delete (window as any).__TAURI_INTERNALS__;vi.mocked(invoke).mockReset();vi.mocked(isNativeResultSaveRuntime).mockReturnValue(false);vi.mocked(saveNativeResultJson).mockReset();});
it('retains every current source row, unit and hash with strict semantic witnesses',async()=>{
 const {model,result,analysisRun}=await currentPrecision(),before=JSON.stringify({result,analysisRun});
 const packet=await buildStressNeutralExportPacket({model,result,analysisRun});
 expect(JSON.stringify({result,analysisRun})).toBe(before);expect(packet.schema_version).toBe('0.3.0');
 expect(packet.export_profile).toMatchObject({profile_id:'ops.stress_neutral.v3',profile_version:'0.3.0'});
 expect(packet.manifest.export_profile_ref).toEqual({object_type:'StressNeutralExportProfile',ref:'ops.stress_neutral.v3'});
 expect(packet.result_rows).toHaveLength(result.results.length);
 for(const source of result.results){const row=packet.result_rows.find((r:any)=>r.result_id===source.id);expect(row.value).toBe(source.value);expect(row.unit).toBe(source.unit);}
 expect(packet.unit_preservation_witnesses.length+packet.diagnostics.filter((d:any)=>d.class==='unit_preservation_witness').length).toBe(result.results.length);
 expect(packet.manifest.package_members).toHaveLength(9);expect(packet.manifest.checksums).toHaveLength(9);
 expect(packet.unit_system_disclosure.conversion_performed).toBe(false);
 expect(packet.source_carrier_checksum.value).toBe(await canonicalSha256HexCheckedV1(result));
 await expect(validateStressNeutralExportPacket(packet,result,analysisRun,modelLoadBasisRefs(model))).resolves.toBeUndefined();
 const tampered=structuredClone(packet);tampered.result_rows[0].value+=1;
 await expect(validateStressNeutralExportPacket(tampered)).rejects.toThrow('SN-SOURCE-ANNOTATION-ROW-BINDING');
 const changedMember=structuredClone(packet);changedMember.loss_report[1].reason+=' changed without checksum';
 await expect(validateStressNeutralExportPacket(changedMember)).rejects.toThrow('MEMBER-CHECKSUM-MISMATCH');
});

it('withholds unsupported and contradictory current row statements through the real reader',async()=>{
 const {result}=await currentPrecision();const original=result.results.find(row=>row.kind==='element_local_axial_force')!;
 const unknown=structuredClone(original);unknown.kind='unknown_native_quantity';
 expect(strictWitnessDisposition(unknown,result).disposition).toBe('unknown_semantic');
 const contradiction=structuredClone(original);contradiction.unit='Pa';
 expect(strictWitnessDisposition(contradiction,result).disposition).toBe('contradiction');
 expect(result.results.find(row=>row.id===original.id)).toEqual(original);
});

it('current row projection refuses contradictory dimension declarations and preserves the statement',async()=>{
 const {result}=await currentPrecision();const source=structuredClone(result.results.find(row=>row.kind==='element_local_axial_force')!);
 source.dimension='stress';const before=structuredClone(source);
 expect(()=>precisionStressRow(source,result)).toThrow('ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH');
 expect(strictWitnessDisposition(source,result).disposition).toBe('contradiction');
 expect(source).toEqual(before);
});

async function rawSha(text:string){const digest=await crypto.subtle.digest('SHA-256',new TextEncoder().encode(text));return [...new Uint8Array(digest)].map(x=>x.toString(16).padStart(2,'0')).join('');}
async function rehash(packet:any){
 const payloads:any={'stress_neutral_results.csv':packet.csv_text,'result_rows.json':packet.result_rows,'unit_system_disclosure.json':packet.unit_system_disclosure,'unit_preservation_witnesses.json':packet.unit_preservation_witnesses,'stable_id_map.json':packet.stable_id_map,'loss_report.json':packet.loss_report,'validation_report.json':packet.validation_report,'diagnostics.json':packet.diagnostics};
 const non=packet.manifest.checksums.filter((x:any)=>x.payload_ref.ref!=='manifest.json');for(const c of non)c.value=c.payload_ref.ref.endsWith('.csv')?await rawSha(payloads[c.payload_ref.ref]):await canonicalSha256HexCheckedV1(payloads[c.payload_ref.ref]);
 const seed={manifest_id:packet.manifest.manifest_id,source_result_ref:packet.source_result_ref,source_run_ref:packet.source_run_ref,source_model_ref:packet.source_model_ref,received_source_checksums:packet.received_source_checksums,unresolved_assumption_refs:packet.unresolved_assumption_refs,reproducibility_refs:packet.reproducibility_refs,export_profile_ref:packet.manifest.export_profile_ref,boundary_notes:packet.manifest.boundary_notes,member_checksums:non,diagnostics:packet.diagnostics};
 const m=packet.manifest.checksums.find((x:any)=>x.payload_ref.ref==='manifest.json');m.value=await canonicalSha256HexCheckedV1(seed);const by=new Map(packet.manifest.checksums.map((x:any)=>[x.payload_ref.ref,x]));for(const member of packet.manifest.package_members)member.checksum=structuredClone(by.get(member.filename));const projection=structuredClone(packet);delete projection.package_checksum;packet.package_checksum.value=await canonicalSha256HexCheckedV1(projection);
}

it('rejects relational tamper even after all covered hashes are recomputed',async()=>{
 const {model,result,analysisRun}=await currentPrecision();const base=await buildStressNeutralExportPacket({model,result,analysisRun});
 const cases:[(p:any)=>void,string][]=[[(p)=>p.unit_preservation_witnesses[0].result_id='wrong','UNIT-WITNESS-BINDING'],[(p)=>p.unit_preservation_witnesses.pop(),'WITNESS-CATEGORY-ACCOUNTING'],[(p)=>p.export_profile.source_basis_refs=p.export_profile.source_basis_refs.filter((item:any)=>item.ref!=='fixtures/results/semantic_contract_v0_3_preview_physics_2.json'),'SEMANTIC-CONTRACT-BINDING'],[(p)=>p.loss_report[0].target_artifact_ref.ref='wrong','LOSS-REPORT-BINDING'],[(p)=>p.source_result_ref.ref='wrong','SOURCE-CARRIER-CHECKSUM-UNBOUND'],[(p)=>{p.result_rows[1].result_id=p.result_rows[0].result_id;p.result_rows[1].canonical_ref=structuredClone(p.result_rows[0].canonical_ref);p.result_rows[1].source_result_ref=structuredClone(p.result_rows[0].source_result_ref);},'SOURCE-ANNOTATION-COVERAGE'],[(p)=>p.stable_id_map[1].canonical_ref=structuredClone(p.stable_id_map[0].canonical_ref),'STABLE-ID-MAP-BINDING'],[(p)=>p.stable_id_map.pop(),'STABLE-ID-MAP-BINDING'],[(p)=>{const row=p.result_rows[0];p.csv_text=p.csv_text.replace(`,${row.unit},${row.dimension},`,`,changed-unit,${row.dimension},`);},'CSV-ROW-BINDING'],[(p)=>p.validation_report.validation_status=p.validation_report.validation_status==='passed'?'blocked':'passed','VALIDATION-STATUS-BINDING'],[(p)=>{p.validation_report.checks[0].check_status='blocking';p.validation_report.checks[0].blocking_count=0;},'VALIDATION-STATUS-BINDING'],[(p)=>p.schema_conformant=false,'SCHEMA-CONFORMANCE-CLAIM'],[(p)=>p.privacy.private_payload_embedded=true,'PRIVACY-BOUNDARY'],[(p)=>p.professional_boundary.software_makes_approval_claim=true,'PROFESSIONAL-BOUNDARY']];
 for(const [mutate,error] of cases){const packet=structuredClone(base);mutate(packet);await rehash(packet);await expect(validateStressNeutralExportPacket(packet)).rejects.toThrow(error);}
});

it('rejects duplicate missing path checksum metadata member divergence and package metadata',async()=>{
 const {model,result,analysisRun}=await currentPrecision();const base=await buildStressNeutralExportPacket({model,result,analysisRun});
 const cases:[(p:any)=>void,string][]=[[(p)=>p.manifest.checksums[0].algorithm='sha512','CHECKSUM-METADATA'],[(p)=>p.manifest.checksums[1].payload_ref.ref='../escape.csv','BIJECTION'],[(p)=>p.manifest.checksums.push(structuredClone(p.manifest.checksums[0])),'STRICT-INVENTORY'],[(p)=>p.manifest.checksums.pop(),'STRICT-INVENTORY'],[(p)=>p.manifest.package_members[0].checksum=structuredClone(p.manifest.checksums[1]),'MEMBER-CHECKSUM-BINDING'],[(p)=>p.package_checksum.payload_scope='member_payload','PACKAGE-CHECKSUM-METADATA']];
 for(const [mutate,error] of cases){const packet=structuredClone(base);mutate(packet);await expect(validateStressNeutralExportPacket(packet)).rejects.toThrow(error);}
});

// Native IPC simulation only; these exact two modes were emitted from the
// maintained ordinary input. Neither old legacy rows nor run IDs are rewritten.
// T0R: the fresh ordinary route is preview-physics-2.
async function currentPrecision(mode: "sparse_interactive" | "dense_scrutiny" = "sparse_interactive") {
 const {model}=nativeMechanicsReplayPair(mode,{profile:"preview"});
 (window as any).__TAURI_INTERNALS__={};
 vi.mocked(invoke).mockImplementation(createNativeMechanicsReplay({profile:"preview"}).invoke);
 const result=await runPreviewMechanics(model,mode);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:result.producer!.component_name,solver_version:result.producer!.component_version,solver_build_ref:'unit-transport-replay',solver_mode:mode,settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 return {model,result,analysisRun};
}
/** Waits until `read` returns a value, rechecking on each DOM mutation, with no wall-clock
 * deadline: it ends when the panel renders the state, however long the work before it takes
 * under load. The test timeout bounds a failure; when the test ends first, the observer is
 * disconnected and the wait settles, so nothing outlives the test. It runs in Testing Library's
 * async wrapper, as `waitFor` does. */
function untilRendered<T>(read:()=>T|null|undefined):Promise<T>{
 return getConfig().asyncWrapper(()=>new Promise<T>((resolve,reject)=>{
  const observer=new MutationObserver(check);
  onTestFinished(()=>{observer.disconnect();reject(new Error('untilRendered: the test ended before the state rendered'));});
  function check(){const found=read();if(found!==null&&found!==undefined){observer.disconnect();resolve(found);}}
  observer.observe(document.body,{subtree:true,childList:true,attributes:true,characterData:true});check();
 }));
}
it('does not publish a delayed packet after its source result is replaced',async()=>{
 const first=await currentPrecision(),second=await currentPrecision("dense_scrutiny");
 const expected=await buildStressNeutralExportPacket(second);
 const original=crypto.subtle.digest.bind(crypto.subtle);let release!:()=>void;const gate=new Promise<void>(resolve=>{release=resolve;});let calls=0;
 let firstHeld!:()=>void;const held=new Promise<void>((resolve,reject)=>{firstHeld=resolve;onTestFinished(()=>reject(new Error('the first build never reached its first digest')));});
 vi.spyOn(crypto.subtle,'digest').mockImplementation(async (algorithm:any,data:any)=>{calls+=1;if(calls===1){firstHeld();await gate;}return original(algorithm,data);});
 // The first build reaches its first digest (and is held there) after its own work, so this
 // wait, too, ends on the event rather than a 1 s deadline.
 const view=render(<StressNeutralExportPanel {...first}/>);await getConfig().asyncWrapper(()=>held);expect(calls).toBe(1);view.rerender(<StressNeutralExportPanel {...second}/>);
 // The second build, then the link's validation, can each outlast the default 1 s wait under
 // load (hosted run 39779582833), so these waits end on the rendered state instead.
 await untilRendered(()=>screen.queryByTestId('stress-neutral-state-binding'));fireEvent.click(screen.getByTestId('stress-neutral-export-link-local-private-intent'));
 await untilRendered(()=>/^data:application\/json/.test(screen.queryByTestId('stress-neutral-export-link')?.getAttribute('href')??'')||null);
 const href=screen.getByTestId('stress-neutral-export-link').getAttribute('href')!;
 expect(JSON.parse(decodeURIComponent(href.slice(href.indexOf(',')+1))).package_checksum.value).toBe(expected.package_checksum.value);
 release();await waitFor(()=>expect(calls).toBeGreaterThan(1));await Promise.resolve();
 expect(screen.getByTestId('stress-neutral-export-link').getAttribute('href')).toBe(href);vi.restoreAllMocks();
});
it('saves native stress JSON after validation and intent and invalidates a pending source generation',async()=>{
 vi.mocked(isNativeResultSaveRuntime).mockReturnValue(true);
 const first=await currentPrecision(),second=await currentPrecision("dense_scrutiny");
 let finishOld!:(value:any)=>void;vi.mocked(saveNativeResultJson).mockImplementationOnce(()=>new Promise(resolve=>{finishOld=resolve;}));
 const view=render(<StressNeutralExportPanel {...first}/>);
 const firstButton=await screen.findByRole('button',{name:/Package JSON/},{timeout:10_000});expect(firstButton).toBeDisabled();expect(saveNativeResultJson).not.toHaveBeenCalled();
 fireEvent.click(screen.getByTestId('stress-neutral-export-link-local-private-intent'));await waitFor(()=>expect(firstButton).toBeEnabled(),{timeout:10_000});fireEvent.click(firstButton);
 expect(saveNativeResultJson).toHaveBeenCalledTimes(1);const firstRequest=vi.mocked(saveNativeResultJson).mock.calls[0][0];expect(firstRequest.screening.route_id).toBe('DOTH-FORMAT-003');expect(firstRequest.file_name).toMatch(/^openpipestress-preview-stress-neutral-[a-z0-9-]+\.json$/);expect(firstRequest.local_first.route_id).toBe('DOTH-FORMAT-003');
 view.rerender(<StressNeutralExportPanel {...second}/>);await waitFor(()=>expect(screen.getByTestId('stress-neutral-summary')).toHaveTextContent(`rows=${second.result.results.length}`),{timeout:10_000});
 const secondButton=screen.getByRole('button',{name:/Package JSON/});expect(secondButton).toBeDisabled();
 await act(async()=>{finishOld({outcome:'saved',file_name:firstRequest.file_name,byte_count:10,replaced_existing:false,durability:'not_guaranteed',path_containment:'best_effort_non_adversarial'});});expect(screen.getByTestId('stress-neutral-export-link-native-save-status')).toBeEmptyDOMElement();
 expect(saveNativeResultJson).toHaveBeenCalledTimes(1);expect(secondButton).not.toHaveAttribute('href');
});
it('keeps preserved references available only to the pure packet projector',async()=>{
 const {model,source:result}=await loadBundledMechanicsReference();
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:result.producer!.component_name,solver_version:result.producer!.component_version,solver_build_ref:'reference-only',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 // The demo's nonlinear cases are confirmed and published CHECKS_PASSED (DEL-04-01 kd5-nonlinear-active-set.md §5).
 expect((await buildStressNeutralExportPacket({model,result,analysisRun})).result_rows).toHaveLength(result.results.length);
 render(<StressNeutralExportPanel model={model} result={result} analysisRun={analysisRun}/>);
 expect(screen.getByTestId('stress-neutral-empty')).toHaveTextContent('Bundled references');
 expect(screen.queryByTestId('stress-neutral-export-link')).toBeNull();
}, 120_000); // heavy build; the hosted full-suite runner is several times slower than local

it('binds preview v3 metadata and preserves shared f64 bits in CSV and JSON',async()=>{
 // B1: the minimal preview-physics-2 transport statement replaces the retired precision-1 header.
 // Explicit synthetic consumer semantics for transport vectors; no physical producer qualification.
 // W-15 slice d: the synthetic statement forms no member, so its test-local model does not consume valve
 // C-130's scale (the model-bound reader would otherwise expect pipe:P-130's body records).
 const model=structuredClone(modelJson) as PreviewModel;
 (model.components.find(c=>c.id==='component:C-130') as any).mechanics_interface.solver_consumption='not_solver_consumed'; // eslint-disable-line @typescript-eslint/no-explicit-any
 const result=minimalPreviewStatement({run_id:'run:transport-vectors',model_ref:model.project.id,
  rows:transport.vectors.map(v=>({id:v.id,kind:'global_nodal_rotation_x',entity_ref:'node:transport',value:JSON.parse(v.decimal),unit:'rad',metadata:{component:'nodal_rotation_x',coordinate_system:'global',location:'summary',basis:'synthetic consumer transport fixture',sign_convention:'synthetic signed scalar'}})),
  diagnostics:[{id:'gate:test',code:'SYNTHETIC_GATE',severity:'info',message:'Consumer fixture only'}],
  cases:model.load_cases.map(c=>({basis_ref:{ref_type:'load_case',ref_id:c.id},structural_status:'passive_model_basis',solve_quality:'checks_passed',model_matrix_fidelity:'represented_equations_retained',accuracy_evidence:'not_claimed',evidence_refs:['gate:test']}))});
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:result.producer!.component_name,solver_version:'0.2.0',solver_build_ref:'test',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
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
   for(const [name,value] of Object.entries({'transport.source.json':result,'transport.analysis.json':analysisRun,'transport.model.json':model}))writeFileSync(path.join(dir,name),JSON.stringify(value,null,2));
   writeFileSync(path.join(dir,'transport.packet.json'),await canonicalJsonCheckedV1(packet));
 }
 const csvSpellingPacket=async(value:number,spelling:string)=>{
   const candidate=structuredClone(packet);candidate.result_rows[0].value=value;
   for(const w of candidate.unit_preservation_witnesses)if(w.result_id===candidate.result_rows[0].result_id){w.source_quantity.value=value;w.target_quantity.value=value;}
   // Every current route carries the source-row annotations; the edited row's goes with it.
   const annotation=candidate.source_annotations.find((x:any)=>x.source_result_id===candidate.result_rows[0].result_id);
   annotation.source_row.value=value;annotation.source_row_sha256=await canonicalSha256HexCheckedV1(annotation.source_row);annotation.source_value_bits=bits(value);
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
   // The source reader refuses a non-number row value before any record or packet exists.
   const invalidSource=structuredClone(result);invalidSource.results[0].value=value as never;
   await expect(buildAnalysisRunPreview(invalidSource,{inputManifest})).rejects.toThrow('SOURCE_PREVIEW_PHYSICS_NUMBER_INVALID');
   const invalidPacket=structuredClone(packet);invalidPacket.result_rows[0].value=value;
   for(const witness of invalidPacket.unit_preservation_witnesses)if(witness.result_id===invalidPacket.result_rows[0].result_id){witness.source_quantity.value=value;witness.target_quantity.value=value;}
   const encode=(v:any)=>{const text=v!==null&&typeof v==='object'?v.ref:String(v);return /[",\n\r]/.test(text)?`"${text.replaceAll('"','""')}"`:text;};
   invalidPacket.csv_text=invalidPacket.export_profile.csv_columns.join(',')+'\n'+invalidPacket.result_rows.map((r:any)=>invalidPacket.export_profile.csv_columns.map((c:string)=>encode(r[c])).join(',')).join('\n')+'\n';
   await rehash(invalidPacket);
   // The source-row annotation (the received value) binds the row first.
   await expect(validateStressNeutralExportPacket(invalidPacket)).rejects.toThrow('SN-SOURCE-ANNOTATION-ROW-BINDING');
 }

 await expect(validateStressNeutralExportPacket(decoded,undefined,analysisRun)).rejects.toThrow('SN-PRECISION-ANALYSIS-SOURCE-REQUIRED');
 const imported=structuredClone(packet);
 imported.provenance={source_name:'Python style fixture exporter',source_location:'interop-fixture',source_license:'project-governed',review_status:'pending',professional_claim:false};
 // Rows keep the UTF-8 CSV's scalar result-id order (every current route); the exporter differs in provenance, witness and export ids.
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
 // A row the reader admits without a resolvable signature (no component): an unknown semantic, withheld.
 withheldSource.results=[{id:'unknown:interop',entity_ref:'nonlinear_supports',kind:'nonlinear_support_free_dof_work_residual',value:1,unit:'N*m',metadata:{coordinate_system:'solver_iteration',location:'final_iteration',basis:'invented transport fixture',sign_convention:'nonnegative max absolute free-DOF residual work product in the final linearized solve'}} as never];
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
 result.numerical_quality!.status='not_assessed';await expect(buildStressNeutralExportPacket({model,result,analysisRun})).rejects.toThrow('SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE');
});

it('projects row location and missing component without guessed semantics',()=>{
 const axial=structuredClone((connectedPreview as unknown as MechanicsResult).results.find(r=>r.kind==='element_local_axial_force')!);
 const source=minimalPreviewStatement({run_id:'run:location',model_ref:'model:location',rows:[axial],cases:[]});
 const row=structuredClone(axial);
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


it('refuses retired stress-neutral versions before reading their metadata',async()=>{
 const base=await buildStressNeutralExportPacket(await currentPrecision()),original=JSON.stringify(base);
 for(const version of ['0.1.0','0.2.0'])for(const key of ['producer','numerical_quality','formulation_basis','semantic_contract_ref','semantic_contract','source_carrier_checksum'])for(const value of [null,false,0,'',{},[]]){
   const candidate=structuredClone(base);candidate.schema_version=version;candidate[key]=value;
   const before=JSON.stringify(candidate);
   await expect(validateStressNeutralExportPacket(candidate)).rejects.toThrow(version==='0.2.0'?'SN-LEGACY-PRECISION-METADATA-FORBIDDEN':'SN-VERSION-UNSUPPORTED');
   expect(JSON.stringify(candidate)).toBe(before);
 }
 for(const version of ['0.1.0','0.2.0'])await expect(validateStressNeutralExportPacket({schema_version:version})).rejects.toThrow('SN-VERSION-UNSUPPORTED');
 expect(JSON.stringify(base)).toBe(original);await expect(validateStressNeutralExportPacket(base)).resolves.toBeUndefined();
});

// Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §3, §4.3): a received pressure-2
// statement (the v3 twin of the retired physics-1) packages at the Rust reader's standing,
// as Python's builder packages it. pressure-2 has no transport reader, so the package is
// read through its source; without it Rust's refusal is kept by name. Bytes that were not
// natively captured are not Current, so the panel offers no package.
it('packages a received exact-route result (pressure-2) at Rust\'s standing, reads it through its source, and offers it only while Current',async()=>{
 const {model,source:result}=nativeMechanicsReplayPair('sparse_interactive',{profile:'pressure'});
 expect(result.producer!.semantic_contract_id).toBe('openpipestress.result_semantics/0.3.0/pressure-2');
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:result.producer!.component_name,solver_version:result.producer!.component_version,solver_build_ref:'received-pressure-reference',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const before=JSON.stringify(result);
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 expect(analysisRun.analysis_run.reproducibility.semantic_contract?.id).toBe('openpipestress.result_semantics/0.3.0/pressure-2');
 const packet=await buildStressNeutralExportPacket({model,result,analysisRun});
 expect(packet.schema_version).toBe('0.3.0');
 expect(packet.semantic_contract.id).toBe('openpipestress.result_semantics/0.3.0/pressure-2');
 expect(packet.export_profile.source_basis_refs[1]).toEqual({object_type:'ExternalReference',ref:'fixtures/results/semantic_contract_v0_3_pressure_2.json'});
 expect(packet.contract_evidence).toEqual(result.contract_evidence);
 expect(Object.hasOwn(packet,'source_block_recovery')).toBe(false);
 await expect(validateStressNeutralExportPacket(packet,result,analysisRun,modelLoadBasisRefs(model))).resolves.toBeUndefined();
 // Without its source, the header has no transport reader: Rust's refusal by name.
 await expect(validateStressNeutralExportPacket(structuredClone(packet))).rejects.toThrow('SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED');
 // The fixture bytes were not natively captured (a copy, not Current): the panel offers nothing.
 render(<StressNeutralExportPanel model={model} result={result} analysisRun={analysisRun}/>);
 expect(screen.getByTestId('stress-neutral-empty')).toBeTruthy();
 expect(screen.queryByTestId('stress-neutral-export-link')).toBeNull();
 expect(screen.queryByTestId('stress-neutral-load-reference-disclosure')).toBeNull();
 expect(JSON.stringify(result)).toBe(before);
});

// Moved from the retired physics-source-1 integration suite (B7b), on the registered
// preview-physics-2 package: the CSV profile is route-independent.
it('preserves UTF8 quoted identifier code points and refuses invalid encoding or record separators', async () => {
 const {model,result,analysisRun}=await currentPrecision();
 const packet=await buildStressNeutralExportPacket({model,result,analysisRun});
 const columns=packet.export_profile.csv_columns as string[];
 const special='case:"温,\r\n度":😀';
 // U+E000 (private use) and U+FEFF (BOM) are written as code points so the source holds no invisible characters.
 const ids=[special,String.fromCharCode(0xe000),'😀']; // Scalar order differs from UTF16 order at the last two IDs.
 const rows=ids.map(id=>({...packet.result_rows[0],result_id:id,canonical_ref:{object_type:'Result',ref:id}}));
 const csv=[columns.join(','),...rows.map(row=>columns.map(key=>{
   const value=(row as any)[key],text=typeof value==='object'?value.ref:String(value);
   return /[",\r\n]/.test(text)?`"${text.replaceAll('"','""')}"`:text;
 }).join(','))].join('\n')+'\n';
 expect(csv).toContain('"case:""温,\r\n度"":😀"');
 expect(()=>validateStressNeutralCsv(csv,rows)).not.toThrow();
 for(const bad of [String.fromCharCode(0xfeff)+csv,csv.replace('温','\ud800'),csv.replace(columns.join(',')+'\n',columns.join(',')+'\r\n')]) {
   expect(()=>validateStressNeutralCsv(bad,rows)).toThrow('SN-CSV-ROW-BINDING-MISMATCH');
 }
 expect(()=>validateStressNeutralCsv(csv,[rows[0],rows[2],rows[1]])).toThrow('SN-CSV-ROW-BINDING-MISMATCH');
 for(const mutate of [
   (p:any)=>{p.export_profile.csv_encoding='ascii';},
   (p:any)=>{p.export_profile.csv_row_order='locale';},
   (p:any)=>{p.manifest.checksums.find((c:any)=>c.payload_ref.ref==='stress_neutral_results.csv').canonicalization='normalized_ascii_lf_text';},
 ]) {
   const bad=structuredClone(packet);mutate(bad);
   await expect(validateStressNeutralExportPacket(bad)).rejects.toThrow(/SN-CSV-ENCODING-PROFILE-MISMATCH|SN-CHECKSUM-METADATA-MISMATCH/);
 }
});
it("preserves signed-zero sidecars and rejects a coherently rehashed false sign against original source",async()=>{
 const {model,result,analysisRun}=await currentPrecision();
 const packet=await buildStressNeutralExportPacket({model,result,analysisRun});
 const changed=JSON.parse(await canonicalJsonCheckedV1(packet));
 const annotation=changed.source_annotations.find((a:any)=>a.source_value_bits==="0000000000000000");expect(annotation).toBeDefined();
 annotation.source_value_bits="8000000000000000";
 // Exact original-source comparison must reject before any package self-hash
 // can present the changed sign as authentic. The raw producer is untouched.
 await expect(validateStressNeutralExportPacket(changed,result,analysisRun)).rejects.toThrow("SN-SOURCE-ANNOTATION-BINDING-MISMATCH");
 expect(await canonicalSha256HexCheckedV1(result)).toBe(await canonicalSha256HexCheckedV1(JSON.parse(nativeMechanicsReplayPair('sparse_interactive',{profile:'preview'}).bytes)));
});

it.each(['json','csv'] as const)('rechecks source and analysis contents at %s activation',async route=>{
 for(const change of ['source','analysis'] as const){
  const props=await currentPrecision();
  vi.mocked(isNativeResultSaveRuntime).mockReturnValue(route==='json');vi.mocked(saveNativeResultJson).mockClear();
  const view=render(<StressNeutralExportPanel {...props}/>);
  const id=route==='json'?'stress-neutral-export-link':'stress-neutral-csv-link';
  // The packet build, then the link's validation, can outlast a 1 s wait under load, so
  // these waits end on the rendered state (`untilRendered`, as the delayed-packet test).
  if(route==='json'){await untilRendered(()=>view.queryByTestId(`${id}-local-private-intent`));fireEvent.click(view.getByTestId(`${id}-local-private-intent`));}
  const link=await untilRendered(()=>{const element=view.queryByTestId(id);
   return element&&element.tagName===(route==='json'?'BUTTON':'SPAN')&&(route==='json'?!(element as HTMLButtonElement).disabled:element.getAttribute('aria-disabled')==='true')?element:null;});
  expect(link.tagName).toBe(route==='json'?'BUTTON':'SPAN');if(route==='json')expect(link).toBeEnabled();else expect(link).toHaveAttribute('aria-disabled','true');
  // Downstream privacy policy already withholds CSV; preserve that refusal.
  // The activation guard also retires the withheld control on content change.
  if(change==='source')props.result.results[0].value+=1;else props.analysisRun.analysis_run.run_name+=' changed without rehash';
  expect(fireEvent.click(link)).toBe(false);expect(saveNativeResultJson).not.toHaveBeenCalled();
  expect(view.queryByTestId(id)).toBeNull();view.unmount();
 }
});

it('packages a registered preview-physics-2 result with its closed evidence and no UI notice text (N-A)',async()=>{
 const props=await currentPrecision();
 expect(props.result.producer!.semantic_contract_id).toBe('openpipestress.result_semantics/0.3.0/preview-physics-2');
 const packet=await buildStressNeutralExportPacket(props);
 expect(packet.contract_evidence).toEqual(props.result.contract_evidence);
 expect(packet.semantic_contract.id).toBe('openpipestress.result_semantics/0.3.0/preview-physics-2');
 expect(packet.export_profile.csv_encoding).toBe('utf-8');
 expect(JSON.stringify(packet.manifest)).not.toMatch(/maximum elastic normal stress; nominal|Rule checks cannot bind|Historical precision-1/);
 await expect(validateStressNeutralExportPacket(packet,props.result,props.analysisRun)).resolves.toBeUndefined();
});

// T6S on a B2 combination successor, tests only (B2-C §10.4, N-8): B2-P's committed W-CB3 successors (case A selected, case B
// not_required, A + B retained_selected), registered with their own invocation as the T6S suite registers corpus bases (a
// TEST STAND-IN for the live native capture, D-U7-4). Combination rows carry `Combination` basis references, and their
// absolute_verified classes from the reader withhold their witnesses with D-U6-2's existing codes and messages; no
// disclosure text or code changes. Unit tests over the committed bytes only, not a native witness.
it.each([['sparse_interactive',combinationSuccessorSparse],['dense_scrutiny',combinationSuccessorDense]] as const)('%s: a combination successor package carries Combination basis references and withholds its combination rows\' class witnesses',async(mode,document)=>{
 const source=structuredClone((document as any).source) as MechanicsResult,invocation=structuredClone((document as any).invocation),model=structuredClone(invocation.request.model) as PreviewModel;
 const captured=checkedJsonText(model);
 await registerRetainedPrecision(source,invocation,(m)=>{try{return checkedJsonText(m)===captured;}catch{return false;}});
 expect(successorStanding(source,model)).toMatchObject({eligible:true,findings:[]});
 const manifest={manifest_ref:{object_type:'InputManifest',ref:'manifest:invented-b2-t6s'},manifest_sha256:'1'.repeat(64),manifest:{model_basis:{model_ref:source.model_ref,model_payload:model},solver_basis:{solver_name:source.producer!.component_name,solver_version:source.producer!.component_version,solver_build_ref:'unit-transport-replay-not-native-witness',solver_mode:mode}}};
 const loadBasis=modelLoadBasisRefs(model);
 expect(loadBasis).toContainEqual({object_type:'Combination',ref:'combination:ab'});
 const analysisRun=await buildAnalysisRunV03(source,manifest as any,undefined,loadBasis);
 const before=checkedJsonText(source),packet=await buildStressNeutralExportPacket({model,result:source,analysisRun});
 expect(checkedJsonText(source)).toBe(before);
 const combinationRows=source.results.filter(r=>r.basis_ref?.ref_type==='combination');
 expect(combinationRows.length).toBe(111);
 for(const row of combinationRows){const packed=packet.result_rows.find((x:any)=>x.result_id===row.id);expect(packed.load_case_ref).toEqual({object_type:'Combination',ref:'combination:ab'});expect(packed.value).toBe(row.value);}
 expect(sourceBasisReference(combinationRows[0].basis_ref)).toEqual({object_type:'Combination',ref:'combination:ab'});
 const classes=(await validateRetainedPrecision(source)).classifications,combinationClasses=classes.filter(c=>c.basis_ref.ref_type==='combination');
 expect(combinationClasses.map(c=>c.result_id)).toEqual(combinationRows.map(r=>r.id));
 const absolute=combinationClasses.filter(c=>c.class==='absolute_verified');
 expect(absolute).toHaveLength(75);
 const findings=packet.diagnostics.filter((d:any)=>d.code==='SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED'&&combinationRows.some(r=>r.id===d.source.ref));
 expect(findings.map((d:any)=>d.source.ref).sort()).toEqual(absolute.map(c=>c.result_id).sort());
 for(const finding of findings){const raw=source.results.find(r=>r.id===finding.source.ref)!;expect(finding).toMatchObject({class:'unit_preservation_witness',severity:'info',message:retainedClassDisclosure(raw.kind,raw.unit,absolute.find(c=>c.result_id===raw.id))!.message});expect(packet.unit_preservation_witnesses.some((w:any)=>w.result_id===raw.id)).toBe(false);}
 // Relative-verified and input-derived combination rows keep their witnesses.
 for(const c of combinationClasses.filter(c=>c.class==='relative_verified'||c.class==='input_derived'))expect(packet.unit_preservation_witnesses.some((w:any)=>w.result_id===c.result_id)).toBe(true);
 // No combination row is not_covered; the load cases' T4-U4b derived rows are (never combined).
 expect(packet.diagnostics.some((d:any)=>d.code==='SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-NOT-COVERED'&&combinationRows.some(r=>r.id===d.source.ref))).toBe(false);
 await expect(validateStressNeutralExportPacket(packet,source,analysisRun,loadBasis)).resolves.toBeUndefined();
 await expect(validateStressNeutralExportPacket(structuredClone(packet))).resolves.toBeUndefined();
}, 120_000); // heavy build; the hosted full-suite runner is several times slower than local
// The same, on 07o's R-COMB-1 bases read from the shared corpus by id (lane C of PR-B2): W-CB4a (A - B, ordinary over the
// selected A: quantity rows not_covered, its two records non_quantity) and W-CB2 (A + B retained_unavailable: every row
// not_covered). Their quantity rows' witnesses are withheld with D-U6-2's existing not-covered code and message.
it.each(['w_cb4a_sparse_interactive','w_cb2_sparse_interactive'])('%s: R-COMB-1 rows are disclosed not_covered and withheld; records keep no class finding',async(id)=>{
 const corpus=readRetainedPrecisionCorpus();
 const entry=corpus.cases.find((c:any)=>c.id===id);expect(entry?.id).toBe(id);
 const source=structuredClone(entry.source) as MechanicsResult,invocation=structuredClone(entry.invocation),model=structuredClone(invocation.request.model) as PreviewModel;
 const captured=checkedJsonText(model);
 await registerRetainedPrecision(source,invocation,(m)=>{try{return checkedJsonText(m)===captured;}catch{return false;}});
 expect(successorStanding(source,model)).toMatchObject({eligible:true,findings:[]});
 const manifest={manifest_ref:{object_type:'InputManifest',ref:'manifest:invented-b2-t6s'},manifest_sha256:'1'.repeat(64),manifest:{model_basis:{model_ref:source.model_ref,model_payload:model},solver_basis:{solver_name:source.producer!.component_name,solver_version:source.producer!.component_version,solver_build_ref:'unit-transport-replay-not-native-witness',solver_mode:invocation.solver_mode}}};
 const loadBasis=modelLoadBasisRefs(model),analysisRun=await buildAnalysisRunV03(source,manifest as any,undefined,loadBasis);
 const packet=await buildStressNeutralExportPacket({model,result:source,analysisRun});
 const combinationId=(source as any).retained_precision.body.combinations[0].basis_ref.ref_id,combinationRows=source.results.filter(r=>r.basis_ref?.ref_type==='combination');
 expect(combinationRows.length).toBeGreaterThan(0);
 for(const row of combinationRows)expect(packet.result_rows.find((x:any)=>x.result_id===row.id).load_case_ref).toEqual({object_type:'Combination',ref:combinationId});
 const classes=(await validateRetainedPrecision(source)).classifications.filter(c=>c.basis_ref.ref_type==='combination');
 expect(classes.map(c=>c.result_id)).toEqual(combinationRows.map(r=>r.id));
 expect(classes.every(c=>c.class==='not_covered'||c.class==='non_quantity')).toBe(true);
 const uncovered=classes.filter(c=>c.class==='not_covered');
 expect(uncovered.length).toBeGreaterThan(0);
 // The load cases' T4-U4b derived rows are also not_covered and withheld; these are the combination rows' findings.
 const findings=packet.diagnostics.filter((d:any)=>d.code==='SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-NOT-COVERED'&&combinationRows.some(r=>r.id===d.source.ref));
 expect(findings.map((d:any)=>d.source.ref).sort()).toEqual(uncovered.map(c=>c.result_id).sort());
 for(const finding of findings){const raw=source.results.find(r=>r.id===finding.source.ref)!;expect(finding).toMatchObject({class:'unit_preservation_witness',severity:'info',message:retainedClassDisclosure(raw.kind,raw.unit,uncovered.find(c=>c.result_id===raw.id))!.message});expect(packet.unit_preservation_witnesses.some((w:any)=>w.result_id===raw.id)).toBe(false);}
 for(const c of classes.filter(c=>c.class==='non_quantity'))expect(packet.diagnostics.some((d:any)=>d.source?.ref===c.result_id&&String(d.code).includes('RETAINED-PRECISION'))).toBe(false);
 await expect(validateStressNeutralExportPacket(packet,source,analysisRun,loadBasis)).resolves.toBeUndefined();
}, 180_000); // heavy build; the hosted full-suite runner is several times slower than local

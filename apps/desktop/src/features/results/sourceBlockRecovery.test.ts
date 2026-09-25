/// <reference types="vite/client" />
import rejectedRaw0Text from '../../../../../fixtures/product_preview/source_blocks/rejected_stress_range/dense_scrutiny.raw.json?raw';
import rejectedRequest0Text from '../../../../../fixtures/product_preview/source_blocks/rejected_stress_range/dense_scrutiny.request.json?raw';
import rejectedRaw1Text from '../../../../../fixtures/product_preview/source_blocks/rejected_stress_range/sparse_interactive.raw.json?raw';
import rejectedRequest1Text from '../../../../../fixtures/product_preview/source_blocks/rejected_stress_range/sparse_interactive.request.json?raw';
import uiRaw0Text from '../../../../../fixtures/product_preview/source_blocks/ui/n05-dense_scrutiny.raw.json?raw';
import uiRequest0Text from '../../../../../fixtures/product_preview/source_blocks/ui/n05-dense_scrutiny.request.json?raw';
import uiRaw1Text from '../../../../../fixtures/product_preview/source_blocks/ui/n05-sparse_interactive.raw.json?raw';
import uiRequest1Text from '../../../../../fixtures/product_preview/source_blocks/ui/n05-sparse_interactive.request.json?raw';
import uiRaw2Text from '../../../../../fixtures/product_preview/source_blocks/ui/n06-dense_scrutiny.raw.json?raw';
import uiRequest2Text from '../../../../../fixtures/product_preview/source_blocks/ui/n06-dense_scrutiny.request.json?raw';
import uiRaw3Text from '../../../../../fixtures/product_preview/source_blocks/ui/n06-sparse_interactive.raw.json?raw';
import uiRequest3Text from '../../../../../fixtures/product_preview/source_blocks/ui/n06-sparse_interactive.request.json?raw';
import uiRaw4Text from '../../../../../fixtures/product_preview/source_blocks/ui/multicase-dense_scrutiny.raw.json?raw';
import uiRequest4Text from '../../../../../fixtures/product_preview/source_blocks/ui/multicase-dense_scrutiny.request.json?raw';
import uiRaw5Text from '../../../../../fixtures/product_preview/source_blocks/ui/multicase-sparse_interactive.raw.json?raw';
import uiRequest5Text from '../../../../../fixtures/product_preview/source_blocks/ui/multicase-sparse_interactive.request.json?raw';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview} from '../../services/previewService';
import {validateAnalysisRunV03,modelLoadBasisRefs} from '../../services/analysisRunCompatibility';
import {buildCurrentResultExport,validateResultDocument} from '../result-export/resultExportAdapter';
import {buildStressNeutralExportPacket,validateStressNeutralExportPacket} from '../stress-neutral/StressNeutralExportPanel';
import {buildHistoricalRunContext} from './HistoricalRunContext';
import {useResultsSessionState} from '../workspace/resultsSessionState';
import {act,renderHook,render} from '@testing-library/react';
import {createElement} from 'react';
import {ResultsPanel} from './ResultsPanel';
import {runRuleChecks} from '../../services/ruleCheckService';
import type {LocalProjectEnvelope} from '../../types';
import raw0Text from '../../../../../fixtures/product_preview/source_blocks/n05-dense_scrutiny.raw.json?raw';
const raw0 = JSON.parse(raw0Text);
import request0Text from '../../../../../fixtures/product_preview/source_blocks/n05-dense_scrutiny.request.json?raw';
const request0 = JSON.parse(request0Text);
import raw1Text from '../../../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.raw.json?raw';
const raw1 = JSON.parse(raw1Text);
import request1Text from '../../../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.request.json?raw';
const request1 = JSON.parse(request1Text);
import raw2Text from '../../../../../fixtures/product_preview/source_blocks/n06-dense_scrutiny.raw.json?raw';
const raw2 = JSON.parse(raw2Text);
import request2Text from '../../../../../fixtures/product_preview/source_blocks/n06-dense_scrutiny.request.json?raw';
const request2 = JSON.parse(request2Text);
import raw3Text from '../../../../../fixtures/product_preview/source_blocks/n06-sparse_interactive.raw.json?raw';
const raw3 = JSON.parse(raw3Text);
import request3Text from '../../../../../fixtures/product_preview/source_blocks/n06-sparse_interactive.request.json?raw';
const request3 = JSON.parse(request3Text);
import raw4Text from '../../../../../fixtures/product_preview/source_blocks/multicase-dense_scrutiny.raw.json?raw';
const raw4 = JSON.parse(raw4Text);
import request4Text from '../../../../../fixtures/product_preview/source_blocks/multicase-dense_scrutiny.request.json?raw';
const request4 = JSON.parse(request4Text);
import raw5Text from '../../../../../fixtures/product_preview/source_blocks/multicase-sparse_interactive.raw.json?raw';
const raw5 = JSON.parse(raw5Text);
import request5Text from '../../../../../fixtures/product_preview/source_blocks/multicase-sparse_interactive.request.json?raw';
const request5 = JSON.parse(request5Text);
import { describe, it, expect, vi, afterEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { runPreviewMechanics, startPreviewMechanicsJob, pollPreviewMechanicsJob, cancelPreviewMechanicsJob, hasNativeMechanicsInvocation, retainedNativeMechanicsInvocation, loadBundledMechanicsReference } from '../../services/previewService';
vi.mock('@tauri-apps/api/core', () => ({invoke:vi.fn()}));
afterEach(() => { delete (window as any).__TAURI_INTERNALS__; vi.resetAllMocks(); });
import precisionFixture from '../../../../../fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json';
import modelFixture from '../../../../../fixtures/product_preview/invented_preview_model.json';
import { canonicalSha256HexCheckedV1 } from '../../services/hashService';
import { sourceContract, numericalResultStanding, PRECISION_CONTRACT_ID } from './numericalResultQuality';
import { SOURCE_BLOCKS_CONTRACT_ID, sourceBlockReceiptShape, validateSourceBlockRecovery, sourceBlockStanding, retainedSourceBlockInvocation } from './sourceBlockRecovery';
import type { MechanicsResult, PreviewModel } from '../../types';

// Synthetic validator control, not a claimed product/source-recovery execution.
// The underlying ordinary producer rows are genuine; the wrapper is deliberately
// built here to probe structural/header/binding rejection independently of solve.
async function ordinaryControl() {
  const source = structuredClone(precisionFixture) as MechanicsResult;
  const model = structuredClone(modelFixture) as PreviewModel;
  model.combinations = [];
  source.results = source.results.filter(r => r.basis_ref?.ref_type !== 'combination');
  const invocation = {request:{model,materials:[]},solver_mode:'sparse_interactive' as const};
  source.producer!.semantic_contract_id = SOURCE_BLOCKS_CONTRACT_ID;
  const body = {
    receipt_version:'1.0.0',policy:'SOURCE-BLOCKS-1',status:'qualified',invocation_work:{limit:64_000_000,charged:0,publication_charged:0},
    invocation:{algorithm:'sha256',canonicalization:'openpipestress_jcs_ijson_v1',payload_scope:'source_blocks_invocation_v1',value:await canonicalSha256HexCheckedV1({domain:'source_blocks_invocation_v1',payload:invocation})},
    publication_sha256:await canonicalSha256HexCheckedV1({domain:'source_blocks_publication_v1',payload:source}),
    cases:model.load_cases.map((load,index) => ({
      basis_ref:{ref_type:'load_case',ref_id:load.id},outcome:'qualified',requested_mode:'sparse_interactive',selected_method:'ordinary_sparse_structural_v1',
      ordinary_attempt:{requested_mode:'sparse_interactive',outcome:'checks_passed',structural_report_diagnostic_ref:source.numerical_quality!.cases[index].evidence_refs.find(id=>source.diagnostics.some(d=>d.id===id)),failure:null,quality_case_index:index},
      source:null,projections:[],rows:source.results.filter(r=>r.basis_ref?.ref_id===load.id).map(r=>({result_id:r.id,treatment:'ordinary_checked',projection_id:null,recipe_id:null,input_result_ids:[]})),supports:[],failure:null,
      work:{limit:1000,charged:0,reserved_unobserved_failure:0,rejected_reservation:{kind:'finite',amount:0}}
    })),envelope_observation_result_ids:source.results.filter(r=>!r.basis_ref).map(r=>r.id)
  };
  source.source_block_recovery = {body,receipt_sha256:await canonicalSha256HexCheckedV1({domain:'source_blocks_receipt_v1',payload:body})};
  return {source,model,invocation};
}
async function rehash(source: MechanicsResult) {
  const receipt = source.source_block_recovery as any, pub = {...source};delete pub.source_block_recovery;
  receipt.body.publication_sha256 = await canonicalSha256HexCheckedV1({domain:'source_blocks_publication_v1',payload:pub});
  receipt.receipt_sha256 = await canonicalSha256HexCheckedV1({domain:'source_blocks_receipt_v1',payload:receipt.body});
}
describe('source-blocks receipt boundary', () => {
  it('rejects receipt presence on ordinary, legacy and unknown headers, including falsy values', () => {
    for (const value of [null,false,0,{},'']) {
      const source = structuredClone(precisionFixture) as MechanicsResult;source.source_block_recovery=value;
      expect(sourceContract(source)).toBe('unsupported');
      source.schema_version='0.1.0';delete source.producer;delete source.numerical_quality;delete source.formulation_basis;
      expect(sourceContract(source)).toBe('unsupported');
    }
    const unknown=structuredClone(precisionFixture) as MechanicsResult;unknown.producer!.semantic_contract_id=SOURCE_BLOCKS_CONTRACT_ID+'/future';expect(sourceContract(unknown)).toBe('unsupported');
  });
  it('does not relabel an ordinary-only wrapper as a successful new method', async () => {
    const {source,invocation}=await ordinaryControl();
    await expect(validateSourceBlockRecovery(source,invocation)).rejects.toThrow('SOURCE_BLOCKS_NO_SOURCE_METHOD');
  });
  it('never grants standing from a receipt shape, public flag or cloned source', async () => {
    const {source,model}=await ordinaryControl();expect(sourceContract(source)).toBe('source_blocks');
    expect(sourceBlockReceiptShape(source.source_block_recovery)).toBe(true);
    expect(numericalResultStanding(source,model).eligible).toBe(false);
    (source as any).source_block_validated=true;
    expect(sourceBlockStanding(source,model).eligible).toBe(false);
    expect(retainedSourceBlockInvocation(source,model)).toBeNull();
  });
  it('binds the entire actual request, including unknown fields, absent materials and mode', async () => {
    const {source,invocation}=await ordinaryControl();
    for(const altered of [ {...invocation,solver_mode:'dense_scrutiny' as const}, {...invocation,request:{model:invocation.request.model}}, {...invocation,request:{...invocation.request,additional:'actual raw field'}} ]) {
      await expect(validateSourceBlockRecovery(source,altered)).rejects.toThrow('SOURCE_BLOCKS_INVOCATION_HASH');
    }
  });
  it('rejects rehashed case deletion, case-mode mismatch and unaccounted result deletion', async () => {
    for(const [mutate,code] of [
      [(s:any)=>s.source_block_recovery.body.cases.pop(),'CASE_COVERAGE'],
      [(s:any)=>s.source_block_recovery.body.cases[0].requested_mode='dense_scrutiny','MODE_QUALITY_BINDING'],
      [(s:any)=>s.source_block_recovery.body.cases[0].rows.pop(),'ROW_COVERAGE']
    ] as const) {
      const {source,invocation}=await ordinaryControl();mutate(source);await rehash(source);
      await expect(validateSourceBlockRecovery(source,invocation)).rejects.toThrow(`SOURCE_BLOCKS_${code}`);
    }
  });
  it('rejects changed publication, malformed nested fields and downgrade with recomputed hashes', async () => {
    const {source,invocation}=await ordinaryControl();source.results[0].value+=1;
    await expect(validateSourceBlockRecovery(source,invocation)).rejects.toThrow('SOURCE_BLOCKS_PUBLICATION_HASH');
    const receipt=source.source_block_recovery as any;receipt.body.cases[0].work.extra='not in schema';
    expect(sourceBlockReceiptShape(receipt)).toBe(false);
    delete receipt.body.cases[0].work.extra;source.producer!.semantic_contract_id=PRECISION_CONTRACT_ID;await rehash(source);
    expect(sourceContract(source)).toBe('unsupported');
    await expect(validateSourceBlockRecovery(source,invocation)).rejects.toThrow('SOURCE_BLOCKS_CONTRACT');
  });
});

const genuinePairs = [
  {name:'n05-dense_scrutiny',raw:raw0,request:request0,mode:'dense_scrutiny' as const},
  {name:'n05-sparse_interactive',raw:raw1,request:request1,mode:'sparse_interactive' as const},
  {name:'n06-dense_scrutiny',raw:raw2,request:request2,mode:'dense_scrutiny' as const},
  {name:'n06-sparse_interactive',raw:raw3,request:request3,mode:'sparse_interactive' as const},
  {name:'multicase-dense_scrutiny',raw:raw4,request:request4,mode:'dense_scrutiny' as const},
  {name:'multicase-sparse_interactive',raw:raw5,request:request5,mode:'sparse_interactive' as const}
];

// Captured from the actual Rust Value-aware producer; these checks consume
// received artifacts. They are not new Rust solves or native UI witnesses.
describe('received genuine source-block producer fixtures', () => {
  it.each(genuinePairs)('$name validates its exact request and preserves ordinary attempt evidence', async ({raw,request,mode}) => {
    const source=structuredClone(raw) as MechanicsResult;
    const invocation={request:structuredClone(request),solver_mode:mode};
    const before=structuredClone(source.numerical_quality);
    expect(await validateSourceBlockRecovery(source,invocation)).toEqual({eligible:true,findings:[]});
    expect(numericalResultStanding(source,invocation.request.model as PreviewModel).eligible).toBe(true);
    expect(source.numerical_quality).toEqual(before);
    const receipt=source.source_block_recovery as any;
    expect(()=>{receipt.body.cases[0].projections[0].value=42;}).toThrow(TypeError);
    expect(sourceBlockReceiptShape(receipt)).toBe(true);
    expect(sourceBlockStanding(structuredClone(source),invocation.request.model as PreviewModel).eligible).toBe(false);
  });
});

function nativeHost() { Object.defineProperty(window,'__TAURI_INTERNALS__',{value:{},configurable:true}); }
function ieeeBits(value:number) {const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value);return view.getBigUint64(0).toString(16).padStart(16,'0');}
describe('actual invocation boundary using mocked IPC and genuine received fixtures', () => {
  it.each(genuinePairs)('$name registers only the actual native response path',async ({raw,request,mode})=>{
    nativeHost();const model=structuredClone(request.model) as PreviewModel;
    vi.mocked(invoke).mockResolvedValueOnce(structuredClone(raw));
    const source=await runPreviewMechanics(model,mode);
    expect(hasNativeMechanicsInvocation(source,model,mode)).toBe(true);
    expect(retainedNativeMechanicsInvocation(source,model)).toEqual({request,solver_mode:mode});
    expect(numericalResultStanding(source,model).eligible).toBe(true);
    expect(hasNativeMechanicsInvocation(structuredClone(source),model,mode)).toBe(false);
    expect(hasNativeMechanicsInvocation(source,model,mode==='dense_scrutiny'?'sparse_interactive':'dense_scrutiny')).toBe(false);
    source.status.rule_check='USER_RULE_CHECKED';
    expect(hasNativeMechanicsInvocation(source,model,mode)).toBe(false);
    expect(sourceBlockStanding(source,model).eligible).toBe(false);
  });
  it('known completed jobs may register; unknown, mismatched and failed jobs cannot',async ()=>{
    const {raw,request,mode}=genuinePairs[0];nativeHost();const model=structuredClone(request.model) as PreviewModel;
    vi.mocked(invoke).mockResolvedValueOnce({job_id:'known',backend_cancellation_token:'token',state:'queued',cancellation_scope:'job'});
    await startPreviewMechanicsJob(model,mode);
    vi.mocked(invoke).mockResolvedValueOnce({job_id:'known',state:'completed',result:structuredClone(raw)});
    const done=await pollPreviewMechanicsJob('known');expect(hasNativeMechanicsInvocation(done.result,model,mode)).toBe(true);
    vi.mocked(invoke).mockResolvedValueOnce({job_id:'unknown',state:'completed',result:structuredClone(raw)});
    const unknown=await pollPreviewMechanicsJob('unknown');expect(hasNativeMechanicsInvocation(unknown.result,model)).toBe(false);
    vi.mocked(invoke).mockResolvedValueOnce({job_id:'other',state:'completed',result:structuredClone(raw)});
    await expect(pollPreviewMechanicsJob('expected')).rejects.toThrow('SOLVE-JOB-IDENTITY-MISMATCH');
    vi.mocked(invoke).mockResolvedValueOnce({job_id:'failed',backend_cancellation_token:'token',state:'queued',cancellation_scope:'job'});
    await startPreviewMechanicsJob(model,mode);vi.mocked(invoke).mockResolvedValueOnce({job_id:'failed',state:'failed',result:structuredClone(raw)});
    const failed=await pollPreviewMechanicsJob('failed');expect(hasNativeMechanicsInvocation(failed.result,model)).toBe(false);
  });
  it('browser references never establish fresh Current provenance',async ()=>{
    const reference=await loadBundledMechanicsReference();expect(reference.standing).toBe('reference_only');
    expect(hasNativeMechanicsInvocation(reference.source,modelFixture as PreviewModel)).toBe(false);
    await expect(runPreviewMechanics(modelFixture as PreviewModel)).rejects.toThrow('BROWSER_SOLVE_BACKEND_REQUIRED_REFERENCE_ONLY');
    await expect(startPreviewMechanicsJob(modelFixture as PreviewModel)).rejects.toThrow('BROWSER_SOLVE_BACKEND_REQUIRED_REFERENCE_ONLY');
  });
  it('invalidates method registration after actual context mutation or signed-zero mutation',async ()=>{
    const {raw,request,mode}=genuinePairs[0];const source=structuredClone(raw) as MechanicsResult;const invocation={request:structuredClone(request),solver_mode:mode};
    const model=structuredClone(invocation.request.model) as PreviewModel;
    await validateSourceBlockRecovery(source,invocation);expect(sourceBlockStanding(source,model).eligible).toBe(true);
    (invocation.request as any).changed_raw_field=true;expect(sourceBlockStanding(source,model).eligible).toBe(false);
    delete (invocation.request as any).changed_raw_field;await validateSourceBlockRecovery(source,invocation);
    const zero=source.results.find(r=>Object.is(r.value,-0))!;expect(zero).toBeDefined();zero.value=0;
    expect(sourceBlockStanding(source,model).eligible).toBe(false);
  });
});

describe('source-block work and derived range policy mutations',()=>{
  it.each(['case_limit','case_sum','invocation_limit','balanced_invocation_overflow','unbalanced_publication'] as const)('rejects rehashed %s claims',async kind=>{
    const {raw,request,mode}=genuinePairs[4];const source=structuredClone(raw) as MechanicsResult;const body=(source.source_block_recovery as any).body;
    if(kind==='case_limit')body.cases[0].work.limit=4_000_001;
    if(kind==='case_sum')body.cases[0].work.reserved_unobserved_failure=body.cases[0].work.limit;
    if(kind==='invocation_limit')body.invocation_work.limit=64_000_001;
    if(kind==='balanced_invocation_overflow'){body.invocation_work.charged=64_000_001;body.invocation_work.publication_charged=64_000_001-body.cases.reduce((n:number,c:any)=>n+c.work.charged+c.work.reserved_unobserved_failure,0);}
    if(kind==='unbalanced_publication')body.invocation_work.publication_charged++;
    await rehash(source);await expect(validateSourceBlockRecovery(source,{request,solver_mode:mode})).rejects.toThrow(['case_limit','invocation_limit','balanced_invocation_overflow'].includes(kind)?'SOURCE_BLOCKS_RECEIPT_SHAPE':kind==='case_sum'?'SOURCE_BLOCKS_WORK':'SOURCE_BLOCKS_INVOCATION_WORK');
  });
  it('counts reserved unobserved failure exactly once rather than requiring it below observed work',async()=>{
    const {raw,request,mode}=genuinePairs[0];const source=structuredClone(raw) as MechanicsResult;const work=(source.source_block_recovery as any).body.cases[0].work;
    work.reserved_unobserved_failure=work.charged;work.charged=0;await rehash(source);
    expect((await validateSourceBlockRecovery(source,{request,solver_mode:mode})).eligible).toBe(true);
    // This rehashed policy control is not a new producer execution/current source.
    expect(hasNativeMechanicsInvocation(source,request.model as PreviewModel)).toBe(false);
  });
  it('rejects a nonzero subnormal output of the ordered scaled norm',async()=>{
    const {raw,request,mode}=genuinePairs[0];const source=structuredClone(raw) as MechanicsResult;const receipt=(source.source_block_recovery as any).body.cases[0];
    const treatment=receipt.rows.find((t:any)=>t.recipe_id==='translation_norm_scaled_v1');const row=source.results.find(r=>r.id===treatment.input_result_ids[0])!;row.value=2**-1030;
    const p=receipt.projections.find((p:any)=>p.result_id===row.id);p.value=row.value;p.value_bits=ieeeBits(row.value);p.interval=[row.value,row.value];p.basis='exact_identity';
    source.results.find(r=>r.id===treatment.result_id)!.value=Math.abs(row.value);
    await rehash(source);await expect(validateSourceBlockRecovery(source,{request,solver_mode:mode})).rejects.toThrow('SOURCE_BLOCKS_NORM_VALUE');
  });
});

const uiPairs=[{name:'n05-dense_scrutiny',raw:JSON.parse(uiRaw0Text),request:JSON.parse(uiRequest0Text),mode:'dense_scrutiny' as const},
{name:'n05-sparse_interactive',raw:JSON.parse(uiRaw1Text),request:JSON.parse(uiRequest1Text),mode:'sparse_interactive' as const},
{name:'n06-dense_scrutiny',raw:JSON.parse(uiRaw2Text),request:JSON.parse(uiRequest2Text),mode:'dense_scrutiny' as const},
{name:'n06-sparse_interactive',raw:JSON.parse(uiRaw3Text),request:JSON.parse(uiRequest3Text),mode:'sparse_interactive' as const},
{name:'multicase-dense_scrutiny',raw:JSON.parse(uiRaw4Text),request:JSON.parse(uiRequest4Text),mode:'dense_scrutiny' as const},
{name:'multicase-sparse_interactive',raw:JSON.parse(uiRaw5Text),request:JSON.parse(uiRequest5Text),mode:'sparse_interactive' as const}];

// These real producer outputs are replayed through mocked transport. This
// exercises the actual frontend registrar and consumers, not a native UI run.
describe('genuine full-UI source-block lifecycle through simulated native IPC',()=>{
 it.each(uiPairs)('$name connects Current, exports, rule revision and saved history',async({raw,request,mode})=>{
  nativeHost();const model=structuredClone(request.model) as PreviewModel;
  vi.mocked(invoke).mockResolvedValueOnce(structuredClone(raw));const result=await runPreviewMechanics(model,mode);
  expect(hasNativeMechanicsInvocation(result,model,mode)).toBe(true);
  const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:result.producer!.component_name,solver_version:result.producer!.component_version,solver_build_ref:'genuine-fixture-mocked-native-transport',solver_mode:mode,settings:{}},active_rule_packs:[],external_assets:[]});
  const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
  await validateAnalysisRunV03(analysisRun,result,modelLoadBasisRefs(model));
  const hook=renderHook(()=>useResultsSessionState());
  act(()=>{hook.result.current.setResult(result);hook.result.current.setInputManifest(inputManifest);});expect(hook.result.current.currentSolvedResult).toBeNull();
  act(()=>hook.result.current.setAnalysisRun(analysisRun));expect(hook.result.current.currentSolvedResult).toBe(result);
  const panel=render(createElement(ResultsPanel,{result,knowledge:null,analysisRun,selectedResultId:null,onSelectResult:()=>{}}));
  expect(panel.getByTestId('numerical-result-standing').textContent).toContain(`Ordinary solve: ${result.numerical_quality!.status}`);
  expect(panel.getByTestId('numerical-result-standing').textContent).toContain('Producer recovery receipt: qualified');panel.unmount();
  const before=structuredClone(result),beforeHash=await canonicalSha256HexCheckedV1(result);
  const canonical=await buildCurrentResultExport({model,result,analysisRun,inputManifest});
  expect(canonical.result_envelope.source_block_recovery).toEqual(result.source_block_recovery);
  await validateResultDocument(canonical,result);
  const stressNeutral=await buildStressNeutralExportPacket({model,result,analysisRun});
  expect(stressNeutral.source_block_recovery).toEqual(result.source_block_recovery);
  await validateStressNeutralExportPacket(stressNeutral,result,analysisRun,modelLoadBasisRefs(model));
  const revision=await buildAnalysisRunPreview(result,{inputManifest,ruleCheckAggregate:'USER_RULE_CHECKED'});
  expect(revision.analysis_run.analysis_status).toContain('USER_RULE_CHECKED');
  expect(result.status.rule_check).toBe('RULE_INPUTS_INCOMPLETE');expect(result).toEqual(before);
  expect(await canonicalSha256HexCheckedV1(result)).toBe(beforeHash);
  expect(revision.analysis_run.hashes.find(h=>h.payload_scope==='received_result')?.value).toBe(analysisRun.analysis_run.hashes.find(h=>h.payload_scope==='received_result')?.value);
  act(()=>hook.result.current.setAnalysisRun(revision));expect(hook.result.current.currentSolvedResult).toBe(result);
  vi.mocked(invoke).mockResolvedValueOnce({aggregate_status:'RULE_INPUTS_INCOMPLETE',checks:[]});
  await runRuleChecks({model,solvedEnvelope:result,rulePackDocument:{metadata:{rule_pack_id:'invented-test-only'}} as never});
  const ruleCall=vi.mocked(invoke).mock.calls.find(call=>call[0]==='run_rule_checks')![1] as any;
  expect(ruleCall.sourceBlockInvocation).toEqual({request,solver_mode:mode});
  expect(result).toEqual(before);
  // Simulate persisted transport with a fresh object; no private token survives.
  const reopened: LocalProjectEnvelope=structuredClone({
   model,mechanics_result:result,analysis_run:revision,model_hash:null,project_envelope_hash:null,editor_intents:[],proposal:null,selected_review_target:null,model_document_migration:null,model_migration_ledger:[],
   summary:{project_id:model.project.id,project_name:model.project.name,database_path:'simulated-received-payload',storage_mode:'test_memory_replay',migration_status:'not_executed',migration_framework:'not_executed',store_schema_version:0,store_schema_target_version:0,migrations_applied_on_open:[],fts_indexed:false,copied_external_files:false,editor_intent_count:0,proposal_count:0,selected_review_target_count:0,selected_review_target_ref:'',persisted_mechanics_result_count:1,persisted_analysis_run_count:1,persisted_analysis_run_ref:revision.analysis_run.run_id,persisted_model_hash_count:0,persisted_model_hash_ref:'',persisted_project_envelope_hash_count:0,persisted_project_envelope_hash_ref:'',unit_round_trip_status:'not_executed',unit_round_trip_checked_ref_count:0,unit_round_trip_signature:'',message:'Simulated persisted transport; no storage operation performed.'}
  });
  const history=await buildHistoricalRunContext(reopened);
  expect(history?.designation).toBe('historical_saved_run');expect(history?.rawMechanicsResult).toEqual(before);
  expect(history?.findings).not.toContain('HISTORICAL_RESULT_HASH_MISMATCH');expect(history?.findings).toContain('SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED');
  expect(hasNativeMechanicsInvocation(reopened.mechanics_result,model)).toBe(false);
  await expect(buildCurrentResultExport({model,result:reopened.mechanics_result!,analysisRun:revision,inputManifest})).rejects.toThrow('CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE');
  hook.unmount();
 },30000);
});

it('accepted cancellation during async receipt validation prevents native registration',async()=>{
 const {raw,request,mode}=genuinePairs[0];nativeHost();const model=structuredClone(request.model) as PreviewModel;
 vi.mocked(invoke).mockResolvedValueOnce({job_id:'cancel-race',backend_cancellation_token:'token',state:'queued',cancellation_scope:'job'});
 await startPreviewMechanicsJob(model,mode);
 vi.mocked(invoke).mockResolvedValueOnce({job_id:'cancel-race',state:'completed',result:structuredClone(raw)}).mockResolvedValueOnce({job_id:'cancel-race',accepted:true});
 const pending=pollPreviewMechanicsJob('cancel-race');await Promise.resolve();await cancelPreviewMechanicsJob('cancel-race','token');
 const done=await pending;expect(hasNativeMechanicsInvocation(done.result,model,mode)).toBe(false);
});

function tinyNormControl() {
 const pair=genuinePairs[0],source=structuredClone(pair.raw) as MechanicsResult,c=(source.source_block_recovery as any).body.cases[0];
 const treatment=c.rows.find((t:any)=>t.recipe_id==='translation_norm_scaled_v1');
 const row=source.results.find(r=>r.id===treatment.input_result_ids[0])!,p=c.projections.find((p:any)=>p.result_id===row.id);
 row.value=2**-520;p.value=row.value;p.value_bits=ieeeBits(row.value);p.interval=[row.value,row.value];p.basis='exact_identity';
 p.absolute_error_bound=0;p.relative_error_bound=0;
 const norm=source.results.find(r=>r.id===treatment.result_id)!;norm.value=row.value;
 source.summary.max_displacement={value:norm.value,unit:norm.unit,result_ref:norm.id,location_ref:norm.entity_ref};
 return {pair,source,p,row};
}
it('ordered scaled norm retains a normal tiny result whose direct squares are subnormal',async()=>{
 const {pair,source,row}=tinyNormControl();expect(row.value*row.value).toBeLessThan(2**-1022);await rehash(source);
 expect((await validateSourceBlockRecovery(source,{request:pair.request,solver_mode:pair.mode})).eligible).toBe(true);
 expect(hasNativeMechanicsInvocation(source,pair.request.model as PreviewModel)).toBe(false);
});
it('reserves the scaled recipe error allowance beyond its affine input bounds',async()=>{
 const {pair,source,p,row}=tinyNormControl();p.basis='outward_interval';p.relative_error_bound=1e-9;p.absolute_error_bound=row.value*1e-9;p.interval=[row.value*(1-1e-9),row.value*(1+1e-9)];
 // Isolate the norm budget in an explicitly inspection-only stress carrier;
 // full stress qualification uses the separately stricter128ε case bound.
 for(const t of (source.source_block_recovery as any).body.cases[0].rows) if(['straight_open_stress_v1','reviewed_stress_summary_v1'].includes(t.recipe_id)){t.treatment='inspection_only';t.recipe_id=null;}
 await rehash(source);
 await expect(validateSourceBlockRecovery(source,{request:pair.request,solver_mode:pair.mode})).rejects.toThrow('SOURCE_BLOCKS_NORM_ERROR_BOUND');
});

describe('conservative normal-range stress carrier admission',()=>{
 it.each([['axial_force',1e-306,'STRESS_MPA_RANGE'],['torsional_moment',1e-307,'STRESS_TORSION_PRODUCT_RANGE']] as const)('withholds %s range loss after rehash',async(component,value,code)=>{
  const pair=genuinePairs[0],source=structuredClone(pair.raw) as MechanicsResult,c=(source.source_block_recovery as any).body.cases[0];
  const row=source.results.find(r=>r.metadata?.component===component&&r.metadata.location==='end_i')!;
  row.value=value;const p=c.projections.find((p:any)=>p.result_id===row.id);Object.assign(p,{value,value_bits:ieeeBits(value),interval:[value,value],basis:'exact_identity',absolute_error_bound:0,relative_error_bound:0});
  await rehash(source);await expect(validateSourceBlockRecovery(source,{request:pair.request,solver_mode:pair.mode})).rejects.toThrow(`SOURCE_BLOCKS_${code}`);
 });
 it('reserves128ε beyond every case projection before checking stress arithmetic',async()=>{
  const pair=genuinePairs[0],source=structuredClone(pair.raw) as MechanicsResult,c=(source.source_block_recovery as any).body.cases[0],p=c.projections.find((p:any)=>p.value!==0);
  const delta=Math.abs(p.value)*1e-9;Object.assign(p,{basis:'outward_interval',relative_error_bound:1e-9,absolute_error_bound:delta,interval:[p.value-delta,p.value+delta]});
  await rehash(source);await expect(validateSourceBlockRecovery(source,{request:pair.request,solver_mode:pair.mode})).rejects.toThrow('SOURCE_BLOCKS_STRESS_ERROR_BOUND');
 });
});

const rejectedStressPairs=[{mode:'dense_scrutiny' as const,raw:JSON.parse(rejectedRaw0Text),request:JSON.parse(rejectedRequest0Text)},
{mode:'sparse_interactive' as const,raw:JSON.parse(rejectedRaw1Text),request:JSON.parse(rejectedRequest1Text)}];

it.each(rejectedStressPairs)('refuses the genuine known-bad $mode MPa publication despite its qualified producer receipt',async({mode,raw,request})=>{
 const source=structuredClone(raw) as MechanicsResult;expect((source.source_block_recovery as any).body.status).toBe('qualified');
 await expect(validateSourceBlockRecovery(source,{request,solver_mode:mode})).rejects.toThrow('SOURCE_BLOCKS_STRESS_MPA_RANGE');
 expect(numericalResultStanding(source,request.model as PreviewModel).eligible).toBe(false);
 nativeHost();vi.mocked(invoke).mockResolvedValueOnce(structuredClone(raw));const received=await runPreviewMechanics(request.model as PreviewModel,mode);
 expect(received).toEqual(raw);expect(hasNativeMechanicsInvocation(received,request.model as PreviewModel)).toBe(false);
});

describe('FE02 caller representation and serialized invocation binding',()=>{
 it.each(['direct','job'] as const)('accepts initial negative zero through the %s native boundary without changing dispatch or source',async route=>{
  const {raw,request,mode}=uiPairs[0];nativeHost();const model=structuredClone(request.model) as PreviewModel;
  model.nodes[0].position.x=-0;
  let source: MechanicsResult;
  if(route==='direct'){
   vi.mocked(invoke).mockResolvedValueOnce(structuredClone(raw));source=await runPreviewMechanics(model,mode);
  }else{
   vi.mocked(invoke).mockResolvedValueOnce({job_id:'fe02',backend_cancellation_token:'token',state:'queued',cancellation_scope:'job'});
   await startPreviewMechanicsJob(model,mode);vi.mocked(invoke).mockResolvedValueOnce({job_id:'fe02',state:'completed',result:structuredClone(raw)});
   source=(await pollPreviewMechanicsJob('fe02')).result!;
  }
  const sent=(vi.mocked(invoke).mock.calls[0][1] as {model:PreviewModel}).model;
  expect(Object.is(model.nodes[0].position.x,-0)).toBe(true);
  expect(Object.is(sent.nodes[0].position.x,0)).toBe(true);
  expect({model:sent,materials:[]}).toEqual(request);expect(source).toEqual(raw);
  expect(hasNativeMechanicsInvocation(source,model,mode)).toBe(true);
  expect(numericalResultStanding(source,model).eligible).toBe(true);
  expect(sourceBlockStanding(source,sent).eligible).toBe(true);
  const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:source.producer!.component_name,solver_version:source.producer!.component_version,solver_build_ref:'FE02-genuine-pair-mocked-IPC',solver_mode:mode,settings:{}},active_rule_packs:[],external_assets:[]});
  expect(Object.is(inputManifest.manifest.model_basis.model_payload.nodes[0].position.x,-0)).toBe(true);
  const analysisRun=await buildAnalysisRunPreview(source,{inputManifest});const hook=renderHook(()=>useResultsSessionState());
  act(()=>{hook.result.current.setResult(source);hook.result.current.setInputManifest(inputManifest);hook.result.current.setAnalysisRun(analysisRun);});
  expect(hook.result.current.currentSolvedResult).toBe(source);
  await expect(buildCurrentResultExport({model,result:source,analysisRun,inputManifest})).resolves.toMatchObject({schema_version:'0.3.0'});
  vi.mocked(invoke).mockResolvedValueOnce({aggregate_status:'RULE_INPUTS_INCOMPLETE',checks:[]});
  await runRuleChecks({model,solvedEnvelope:source,rulePackDocument:{metadata:{rule_pack_id:'invented-test'}} as never});
  const ruleArgs=vi.mocked(invoke).mock.calls.find(call=>call[0]==='run_rule_checks')![1] as any;
  expect(ruleArgs.sourceBlockInvocation).toEqual({request,solver_mode:mode});hook.unmount();
 });
 it.each(['caller_zero','caller_value','dispatch_zero','source_zero'] as const)('rejects %s mutation after successful caller normalization',async change=>{
  const {raw,request,mode}=uiPairs[0];nativeHost();const model=structuredClone(request.model) as PreviewModel;model.nodes[0].position.x=-0;
  vi.mocked(invoke).mockResolvedValueOnce(structuredClone(raw));const source=await runPreviewMechanics(model,mode);
  expect(numericalResultStanding(source,model).eligible).toBe(true);const sent=(vi.mocked(invoke).mock.calls[0][1] as {model:PreviewModel}).model;
  if(change==='caller_zero')model.nodes[0].position.x=0;
  if(change==='caller_value')model.nodes[0].position.x=0.125;
  if(change==='dispatch_zero')sent.nodes[0].position.x=-0;
  if(change==='source_zero'){const row=source.results.find(r=>Object.is(r.value,-0))!;expect(row).toBeDefined();row.value=0;}
  expect(hasNativeMechanicsInvocation(source,model,mode)).toBe(false);
  expect(sourceBlockStanding(source,model).eligible).toBe(false);
  expect(retainedSourceBlockInvocation(source,model)).toBeNull();
 });
 it('refuses a caller edited while IPC is pending and an unobserved caller alias',async()=>{
  const {raw,request,mode}=uiPairs[0];nativeHost();const model=structuredClone(request.model) as PreviewModel;model.nodes[0].position.x=-0;
  let resolve!: (value:MechanicsResult)=>void;vi.mocked(invoke).mockReturnValueOnce(new Promise<MechanicsResult>(r=>{resolve=r;}));
  const pending=runPreviewMechanics(model,mode);model.nodes[0].position.x=0;resolve(structuredClone(raw));const source=await pending;
  expect(hasNativeMechanicsInvocation(source,model)).toBe(false);expect(sourceBlockStanding(source,model).eligible).toBe(false);
  const unsupported=structuredClone(request.model) as PreviewModel;unsupported.nodes[0].position.x=1;
  await expect(validateSourceBlockRecovery(structuredClone(raw),{request,solver_mode:mode},unsupported)).rejects.toThrow('CALLER_DISPATCH_NORMALIZATION_MISMATCH');
 });
});

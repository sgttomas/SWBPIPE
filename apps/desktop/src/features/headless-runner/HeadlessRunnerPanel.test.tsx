import {it,expect} from 'vitest';
import modelJson from '../../../../../fixtures/product_preview/invented_preview_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_mechanics_result.json';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,bindSourceResultDimensions} from '../../services/previewService';
import {PRECISION_CONTRACT_ID,sourceContract} from '../results/numericalResultQuality';
import {buildHeadlessRunnerPacket} from './HeadlessRunnerPanel';
import type {PreviewModel,MechanicsResult,SolveJobAuditState} from '../../types';
// Historical-format protocol control only, as `precision()` in
// resultExportAdapter.test.ts: the legacy rows, with or without their received
// dimension declarations, carry a precision-1 header so that the supported
// analysis record can be composed. They are never replayed as native output and
// never qualify Current or numerical accuracy. The imported fixture is untouched.
function precisionCarrier(result:MechanicsResult,model:PreviewModel):MechanicsResult{
 result.diagnostics=result.diagnostics.map((diagnostic,index)=>({...diagnostic,id:`test:synthetic-diagnostic:${index}:${diagnostic.id ?? 'missing'}`}));
 result.schema_version='0.2.0';
 result.producer={component_name:'open_pipe_stress_product_physics',component_version:'0.2.0',semantic_contract_id:PRECISION_CONTRACT_ID};
 result.formulation_basis={profile_id:'product_preview_mechanics_v1',limitations:['bounded pressure/component/stress/support preview']};
 result.diagnostics.push({id:'test:gate',code:'TEST_GATE',severity:'info',message:'Synthetic consumer gate evidence; not native proof'});
 result.numerical_quality={value_representation:'finite_binary64',publication_quantization:'none',integrity_policy:'M03-INTEGRITY-v1',status:'checks_passed',cases:model.load_cases.map(c=>({basis_ref:{ref_type:'load_case',ref_id:c.id},structural_status:'passive_model_basis',solve_quality:'checks_passed',model_matrix_fidelity:'represented_equations_retained',accuracy_evidence:'not_claimed',evidence_refs:['test:gate']}))};
 return result;
}
// A supported analysis record binds the producer identity the carrier declares.
const manifestFor=(model:PreviewModel)=>buildCurrentSessionInputManifest({model,solver:{solver_name:'open_pipe_stress_product_physics',solver_version:'0.2.0',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
it('keeps received declarations as reference evidence and reports unavailable witnesses',async()=>{
 const model=modelJson as PreviewModel,result=precisionCarrier(bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult),model),solveJob={job_id:'fixture',state:'completed',events:[],cancellation_requested:false,cancellation_status:'not-requested'} as unknown as SolveJobAuditState;
 expect(sourceContract(result)).toBe('precision');
 const inputManifest=await manifestFor(model);const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 const packet=buildHeadlessRunnerPacket({model,result,analysisRun,solveJob});
 // Each received declaration is witnessed, so the preservation loop cannot pass vacuously.
 const declared=result.results.filter(row=>typeof row.dimension==='string'&&row.dimension);expect(declared.length).toBeGreaterThan(0);expect(packet.result.unit_preservation_witnesses).toHaveLength(declared.length);
 for(const w of packet.result.unit_preservation_witnesses){const row=result.results.find(r=>r.id===w.source_result_ref.ref_id)!;expect(w.source_quantity.dimension).toBe(row.dimension);expect(w.target_quantity).toEqual(w.source_quantity);}
 // The same rows without received declarations: nothing is witnessed and the gap is disclosed.
 const absent=precisionCarrier(structuredClone(resultJson) as unknown as MechanicsResult,model);expect(absent.results.some(row=>Object.hasOwn(row,'dimension'))).toBe(false);const unavailable=buildHeadlessRunnerPacket({model,result:absent,analysisRun,solveJob});expect(unavailable.result.unit_preservation_witnesses).toHaveLength(0);expect(unavailable.result.diagnostics.some(d=>d.code==='RUNNER_RECEIVED_DIMENSION_WITNESS_UNAVAILABLE')).toBe(true);
});
it('refuses the legacy carrier as the runner analysis basis without changing it',async()=>{
 const model=modelJson as PreviewModel,legacy=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult),before=JSON.stringify(legacy);
 expect(sourceContract(legacy)).toBe('legacy');
 await expect(buildAnalysisRunPreview(legacy,{inputManifest:await manifestFor(model)})).rejects.toThrow('SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED');
 expect(JSON.stringify(legacy)).toBe(before);
});

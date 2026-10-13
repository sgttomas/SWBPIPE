import {it,expect} from 'vitest';
import modelJson from '../../../../../fixtures/product_preview/invented_demo_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_demo_result_legacy_0_1.json';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,bindSourceResultDimensions} from '../../services/previewService';
import previewText from '../../../../../fixtures/results/preview_physics_connected_sparse.json?raw';
import previewModelJson from '../../../../../fixtures/model_operations/precision_connected_ui_model.json';
import { sourceContract } from '../results/resultReader';
import {buildHeadlessRunnerPacket} from './HeadlessRunnerPanel';
import type {PreviewModel,MechanicsResult,SolveJobAuditState} from '../../types';
// A supported analysis record binds the producer identity the carrier declares.
const manifestFor=(model:PreviewModel)=>buildCurrentSessionInputManifest({model,solver:{solver_name:'open_pipe_stress_product_physics',solver_version:'0.2.0',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
// B1 retired precision-1, whose synthetic carrier gave the legacy rows' received
// dimension declarations a current record. A current preview-physics-1 result
// declares none: nothing is witnessed and the gap is disclosed.
it('reports unavailable witnesses for a current result without received declarations',async()=>{
 const model=previewModelJson as unknown as PreviewModel,result=JSON.parse(previewText) as MechanicsResult,solveJob={job_id:'fixture',state:'completed',events:[],cancellation_requested:false,cancellation_status:'not-requested'} as unknown as SolveJobAuditState;
 expect(sourceContract(result)).toBe('preview_physics');
 expect(result.results.some(row=>Object.hasOwn(row,'dimension'))).toBe(false);
 const inputManifest=await manifestFor(model);const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 const packet=buildHeadlessRunnerPacket({model,result,analysisRun,solveJob});
 expect(packet.result.unit_preservation_witnesses).toHaveLength(0);expect(packet.result.diagnostics.some(d=>d.code==='RUNNER_RECEIVED_DIMENSION_WITNESS_UNAVAILABLE')).toBe(true);
});
it('refuses the legacy carrier as the runner analysis basis without changing it',async()=>{
 const model=modelJson as PreviewModel,legacy=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult),before=JSON.stringify(legacy);
 expect(sourceContract(legacy)).toBe('legacy');
 await expect(buildAnalysisRunPreview(legacy,{inputManifest:await manifestFor(model)})).rejects.toThrow('SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED');
 expect(JSON.stringify(legacy)).toBe(before);
});

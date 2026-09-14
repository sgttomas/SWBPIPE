import {it,expect} from 'vitest';
import modelJson from '../../../../../fixtures/product_preview/invented_preview_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_mechanics_result.json';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,bindSourceResultDimensions} from '../../services/previewService';
import {buildHeadlessRunnerPacket} from './HeadlessRunnerPanel';
import type {PreviewModel,MechanicsResult,SolveJobAuditState} from '../../types';
it('keeps received declarations as reference evidence and reports unavailable witnesses',async()=>{
 const model=modelJson as PreviewModel,result=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult),solveJob={job_id:'fixture',state:'completed',events:[],cancellation_requested:false,cancellation_status:'not-requested'} as unknown as SolveJobAuditState;
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 const packet=buildHeadlessRunnerPacket({model,result,analysisRun,solveJob});for(const w of packet.result.unit_preservation_witnesses){const row=result.results.find(r=>r.id===w.source_result_ref.ref_id)!;expect(w.source_quantity.dimension).toBe(row.dimension);expect(w.target_quantity).toEqual(w.source_quantity);}
 const absent=structuredClone(resultJson) as unknown as MechanicsResult;const unavailable=buildHeadlessRunnerPacket({model,result:absent,analysisRun,solveJob});expect(unavailable.result.unit_preservation_witnesses).toHaveLength(0);expect(unavailable.result.diagnostics.some(d=>d.code==='RUNNER_RECEIVED_DIMENSION_WITNESS_UNAVAILABLE')).toBe(true);
});

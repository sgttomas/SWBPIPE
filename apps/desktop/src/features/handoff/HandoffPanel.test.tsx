import {it,expect} from 'vitest';
import modelJson from '../../../../../fixtures/product_preview/invented_demo_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_demo_result_legacy_0_1.json';
import {buildHandoffPackage} from './HandoffPanel';
import {buildAnalysisRunPreview,bindSourceResultDimensions} from '../../services/previewService';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import previewText from '../../../../../fixtures/results/preview_physics_connected_sparse.json?raw';
import previewModelJson from '../../../../../fixtures/model_operations/precision_connected_ui_model.json';
import { sourceContract } from '../results/resultReader';
import type {PreviewModel,MechanicsResult} from '../../types';
// A supported analysis record binds the producer identity the carrier declares.
const manifestFor=(model:PreviewModel)=>buildCurrentSessionInputManifest({model,solver:{solver_name:"open_pipe_stress_product_physics",solver_version:"0.2.0",solver_build_ref:"fixture",solver_mode:"sparse_interactive",settings:{}},active_rule_packs:[],external_assets:[]});
// B1 retired precision-1, whose synthetic carrier gave the legacy rows' received
// dimension declarations a current record. A current preview-physics-1 result
// declares none: it is recorded, and nothing is witnessed from absent declarations.
it('a current preview-physics-1 result has no received declarations, so nothing is witnessed',async()=>{
 const model=previewModelJson as unknown as PreviewModel,result=JSON.parse(previewText) as MechanicsResult;
 expect(sourceContract(result)).toBe('preview_physics');
 expect(result.results.some(row=>Object.hasOwn(row,'dimension'))).toBe(false);
 const inputManifest=await manifestFor(model);const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 const packet=buildHandoffPackage({model,result,analysisRun,knowledge:null,comparison:null,editorIntents:[],proposal:null,selectedReviewTarget:null});
 expect(packet.unit_preservation_witnesses).toHaveLength(0);
});
it('refuses the legacy carrier as the handoff analysis basis without changing it',async()=>{
 const model=modelJson as PreviewModel,legacy=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult),before=JSON.stringify(legacy);
 expect(sourceContract(legacy)).toBe('legacy');
 await expect(buildAnalysisRunPreview(legacy,{inputManifest:await manifestFor(model)})).rejects.toThrow('SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED');
 expect(JSON.stringify(legacy)).toBe(before);
});

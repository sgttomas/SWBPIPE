import {it,expect} from 'vitest';
import modelJson from '../../../../../fixtures/product_preview/invented_demo_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json';
import {buildHandoffPackage} from './HandoffPanel';
import {buildAnalysisRunPreview} from '../../services/previewService';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import previewText from '../../../../../fixtures/results/preview_physics_connected_sparse.json?raw';
import previewModelJson from '../../../../../fixtures/model_operations/precision_connected_ui_model.json';
import { sourceContract } from '../results/resultReader';
import type {PreviewModel,MechanicsResult} from '../../types';
// A supported analysis record binds the producer identity the carrier declares.
const manifestFor=(model:PreviewModel)=>buildCurrentSessionInputManifest({model,solver:{solver_name:"open_pipe_stress_product_physics",solver_version:"0.2.0",solver_build_ref:"fixture",solver_mode:"sparse_interactive",settings:{}},active_rule_packs:[],external_assets:[]});
// B1 retired precision-1, whose synthetic carrier gave the legacy rows' received
// dimension declarations a current record. A current preview-physics-2 result
// declares none: it is recorded, and nothing is witnessed from absent declarations.
it('a current preview-physics-2 result has no received declarations, so nothing is witnessed',async()=>{
 const model=previewModelJson as unknown as PreviewModel,result=JSON.parse(previewText) as MechanicsResult;
 expect(sourceContract(result)).toBe('preview_physics');
 expect(result.results.some(row=>Object.hasOwn(row,'dimension'))).toBe(false);
 const inputManifest=await manifestFor(model);const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 const packet=buildHandoffPackage({model,result,analysisRun,knowledge:null,comparison:null,editorIntents:[],proposal:null,selectedReviewTarget:null});
 expect(packet.unit_preservation_witnesses).toHaveLength(0);
});
it('refuses the retired 0.1.0 carrier as the handoff analysis basis without changing it',async()=>{
 const model=modelJson as PreviewModel,retired=structuredClone(resultJson) as unknown as MechanicsResult;
 retired.schema_version='0.1.0';for(const key of ['producer','numerical_quality','formulation_basis','contract_evidence'])delete (retired as unknown as Record<string,unknown>)[key];
 const before=JSON.stringify(retired);
 expect(sourceContract(retired)).toBe('unsupported');
 await expect(buildAnalysisRunPreview(retired,{inputManifest:await manifestFor(model)})).rejects.toThrow('SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED');
 expect(JSON.stringify(retired)).toBe(before);
});
// DEL-15-01 W-6b: the panel stays a preview; its download says so and points to
// File › Save Handoff…, and its rule-pack references are the document's.
it('is labelled a preview summary, points to Save Handoff, and carries the document rule-pack references',async()=>{
 const {render,screen}=await import('@testing-library/react');
 const {HandoffPanel,HANDOFF_PREVIEW_DOWNLOAD_LABEL,HANDOFF_SAVE_POINTER,handoffRulePackRefs}=await import('./HandoffPanel');
 const everySlot=(await import('../../../../../fixtures/model_document/invented_every_slot_model.json')).default as unknown as PreviewModel;
 expect(HANDOFF_PREVIEW_DOWNLOAD_LABEL).toBe('Preview handoff summary (not a handoff manifest)');
 expect(handoffRulePackRefs(everySlot)).toEqual(everySlot.rule_pack_refs);
 expect(handoffRulePackRefs(everySlot)[0]).not.toBe(everySlot.rule_pack_refs![0]);
 const model={...(previewModelJson as unknown as PreviewModel),rule_pack_refs:everySlot.rule_pack_refs},result=JSON.parse(previewText) as MechanicsResult;
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest:await manifestFor(model)});
 const packet=buildHandoffPackage({model,result,analysisRun,knowledge:null,comparison:null,editorIntents:[],proposal:null,selectedReviewTarget:null});
 expect(packet.library_refs.rule_pack_refs).toEqual(everySlot.rule_pack_refs);
 render(<HandoffPanel model={model} knowledge={null} result={result} analysisRun={analysisRun} comparison={null} editorIntents={[]} proposal={null} selectedReviewTarget={null}/>);
 expect(screen.getByTestId('handoff-export-link')).toHaveTextContent(HANDOFF_PREVIEW_DOWNLOAD_LABEL);
 expect(screen.getByTestId('handoff-save-pointer')).toHaveTextContent(HANDOFF_SAVE_POINTER);
 expect(HANDOFF_SAVE_POINTER).toContain('File › Save Handoff…');
});

import {it,expect} from 'vitest';
import modelJson from '../../../../../fixtures/product_preview/invented_preview_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_mechanics_result.json';
import {buildHandoffPackage} from './HandoffPanel';
import {buildAnalysisRunPreview,bindSourceResultDimensions} from '../../services/previewService';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import type {PreviewModel,MechanicsResult} from '../../types';
it('reference-only handoff preserves received declarations and withholds absent dimension witnesses',async()=>{
 const model=modelJson as PreviewModel,result=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:"fixture",solver_version:"1",solver_build_ref:"fixture",solver_mode:"sparse_interactive",settings:{}},active_rule_packs:[],external_assets:[]});const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});const packet=buildHandoffPackage({model,result,analysisRun,knowledge:null,comparison:null,editorIntents:[],proposal:null,selectedReviewTarget:null});
 for(const witness of packet.unit_preservation_witnesses){const row=result.results.find(row=>row.id===witness.source_ref.ref)!;expect(witness.source_quantity.dimension).toBe(row.dimension);expect(witness.source_quantity.value).toBe(row.value);expect(witness.target_quantity).toEqual(witness.source_quantity);}
 const absent=structuredClone(resultJson) as unknown as MechanicsResult;const unavailable=buildHandoffPackage({model,result:absent,analysisRun,knowledge:null,comparison:null,editorIntents:[],proposal:null,selectedReviewTarget:null});expect(unavailable.unit_preservation_witnesses).toHaveLength(0);
});

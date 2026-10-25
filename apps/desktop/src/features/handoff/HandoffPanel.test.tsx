import {afterEach,describe,it,expect} from 'vitest';
import {cleanup} from '@testing-library/react';
import modelJson from '../../../../../fixtures/product_preview/invented_demo_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json';
import {buildHandoffPackage,HandoffPanel} from './HandoffPanel';
import {buildAnalysisRunPreview} from '../../services/previewService';
import {RULE_PACK_REF_CHECKSUM_MISMATCH,RULE_PACK_REF_UNRESOLVED} from '../../services/rulePackRefDiagnostics';
import {resolveModelRulePacks} from '../../services/modelRulePackResolution';
import {hex,inventedEnvelope,inventedReference} from '../../test/inventedRulePacks';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import previewText from '../../../../../fixtures/results/preview_physics_connected_sparse.json?raw';
import previewModelJson from '../../../../../fixtures/model_operations/precision_connected_ui_model.json';
import { sourceContract } from '../results/resultReader';
import type {DesignKnowledge,PreviewModel,MechanicsResult,RulePackReference} from '../../types';
afterEach(cleanup);
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
// File › Save Handoff…, and its rule-pack references are the document's the run resolved.
it('is labelled a preview summary, points to Save Handoff, and names the document rule-pack references the run excluded',async()=>{
 const {render,screen}=await import('@testing-library/react');
 const {HandoffPanel,HANDOFF_PREVIEW_DOWNLOAD_LABEL,HANDOFF_SAVE_POINTER,handoffRulePackRefs}=await import('./HandoffPanel');
 const everySlot=(await import('../../../../../fixtures/model_document/invented_every_slot_model.json')).default as unknown as PreviewModel;
 expect(HANDOFF_PREVIEW_DOWNLOAD_LABEL).toBe('Preview handoff summary (not a handoff manifest)');
 expect(handoffRulePackRefs(everySlot)).toEqual(everySlot.rule_pack_refs);
 expect(handoffRulePackRefs(everySlot)[0]).not.toBe(everySlot.rule_pack_refs![0]);
 const model={...(previewModelJson as unknown as PreviewModel),rule_pack_refs:everySlot.rule_pack_refs},result=JSON.parse(previewText) as MechanicsResult;
 // #1602: a run records the model's rule-pack references the manifest leaves out (none is stored here).
 const rulePackExclusions=everySlot.rule_pack_refs!.map((ref)=>({rule_pack_id:ref.rule_pack_id,rule_pack_version:ref.rule_pack_version,code:RULE_PACK_REF_UNRESOLVED}) as const);
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest:await manifestFor(model),rulePackExclusions});
 const packet=buildHandoffPackage({model,result,analysisRun,knowledge:null,comparison:null,editorIntents:[],proposal:null,selectedReviewTarget:null});
 // Every reference is excluded: none is listed, and each is named among the diagnostics.
 expect(packet.library_refs.rule_pack_refs).toEqual([]);
 expect(packet.diagnostic_refs.slice(-1)).toEqual(['diagnostic:rule-pack-ref:invented_demo_rule_pack@0.2.0']);
 render(<HandoffPanel model={model} knowledge={null} result={result} analysisRun={analysisRun} comparison={null} editorIntents={[]} proposal={null} selectedReviewTarget={null}/>);
 expect(screen.getByTestId('handoff-export-link')).toHaveTextContent(HANDOFF_PREVIEW_DOWNLOAD_LABEL);
 expect(screen.getByTestId('handoff-save-pointer')).toHaveTextContent(HANDOFF_SAVE_POINTER);
 expect(HANDOFF_SAVE_POINTER).toContain('File › Save Handoff…');
});

// #1602's follow-up: the preview's rule-pack list is the model's references the solve resolved
// (the run record's exclusions omitted), and each exclusion is named once among its diagnostics.
describe('the preview handoff summary follows the solve-time rule-pack resolution',()=>{
 const HELD=inventedReference('invented-pack-held',hex('1')),MISSING=inventedReference('invented-pack-missing',hex('2')),CHANGED=inventedReference('invented-pack-changed',hex('3'));
 async function solvedWith(references:RulePackReference[]|null){
  const model=structuredClone(previewModelJson) as unknown as PreviewModel,result=JSON.parse(previewText) as MechanicsResult;
  if(references)model.rule_pack_refs=references;
  const store=new Map([[HELD.rule_pack_id,inventedEnvelope(model.project.id,HELD.rule_pack_id,hex('1'))],[CHANGED.rule_pack_id,inventedEnvelope(model.project.id,CHANGED.rule_pack_id,hex('4'))]]);
  const resolution=await resolveModelRulePacks(model,async(_projectId,id)=>({route:'tauri_backend',envelope:store.get(id)??null}));
  const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'open_pipe_stress_product_physics',solver_version:'0.2.0',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:resolution.active_rule_packs,external_assets:[]});
  const analysisRun=await buildAnalysisRunPreview(result,{inputManifest,rulePackExclusions:resolution.excluded});
  return {model,result,analysisRun,resolution};
 }
 const packetOf=(session:Awaited<ReturnType<typeof solvedWith>>,knowledge:DesignKnowledge|null=null)=>buildHandoffPackage({...session,knowledge,comparison:null,editorIntents:[],proposal:null,selectedReviewTarget:null});
 const MISSING_ID='diagnostic:rule-pack-ref:invented-pack-missing@1.0.0',CHANGED_ID='diagnostic:rule-pack-ref:invented-pack-changed@1.0.0';

 it('lists the resolved references, omits the excluded ones and names each exclusion once',async()=>{
  const session=await solvedWith([HELD,MISSING,CHANGED]);
  expect(session.resolution.excluded.map(item=>item.code)).toEqual([RULE_PACK_REF_UNRESOLVED,RULE_PACK_REF_CHECKSUM_MISMATCH]);
  const packet=packetOf(session);
  expect(packet.library_refs.rule_pack_refs).toEqual([HELD]);
  expect(packet.library_refs.rule_pack_refs[0]).not.toBe(session.model.rule_pack_refs![0]);
  const carried=[...session.model.diagnostics,...session.result.diagnostics].map(item=>item.id??item.code);
  expect(packet.diagnostic_refs).toEqual([...carried,MISSING_ID,CHANGED_ID]);
  expect(session.analysisRun.analysis_run.diagnostics!.map(item=>item.source_annotation.id)).toEqual(expect.arrayContaining([MISSING_ID,CHANGED_ID]));
  const {render,screen}=await import('@testing-library/react');
  render(<HandoffPanel model={session.model} knowledge={null} result={session.result} analysisRun={session.analysisRun} comparison={null} editorIntents={[]} proposal={null} selectedReviewTarget={null}/>);
  expect(screen.getByTestId('handoff-export-summary')).toHaveTextContent(`${carried.length+2} diagnostics`);
 });

 it('adds no second reference for an exclusion a carried diagnostic already names',async()=>{
  const session=await solvedWith([HELD,MISSING,CHANGED]);
  const knowledge={schema_version:'0.1.0',document_kind:'invented',knowledge_set_id:'invented',model_ref:session.model.project.id,records:[],
   diagnostics:[{id:'invented-knowledge-diagnostic',code:RULE_PACK_REF_UNRESOLVED,severity:'warning',message:'Invented diagnostic naming the reference.',affected_refs:['invented-pack-missing@1.0.0']}]} as DesignKnowledge;
  const packet=packetOf(session,knowledge);
  expect(packet.diagnostic_refs.filter(ref=>ref==='invented-knowledge-diagnostic')).toHaveLength(1);
  expect(packet.diagnostic_refs).not.toContain(MISSING_ID);
  expect(packet.diagnostic_refs.filter(ref=>ref===CHANGED_ID)).toHaveLength(1);
 });

 it('without exclusions lists every model reference and adds no diagnostic',async()=>{
  for(const references of [null,[HELD]]){
   const session=await solvedWith(references);
   expect(session.resolution.excluded).toEqual([]);
   const packet=packetOf(session);
   expect(packet.library_refs.rule_pack_refs).toEqual(references??[]);
   expect(packet.diagnostic_refs).toEqual([...session.model.diagnostics,...session.result.diagnostics].map(item=>item.id??item.code));
  }
 });
});

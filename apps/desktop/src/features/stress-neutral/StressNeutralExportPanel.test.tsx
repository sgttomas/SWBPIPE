import {resultSemantics} from '../results/resultSemantics';
import {it,expect} from 'vitest';
import {mkdirSync,writeFileSync} from 'node:fs';
import path from 'node:path';
import modelJson from '../../../../../fixtures/product_preview/invented_preview_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_mechanics_result.json';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,bindSourceResultDimensions} from '../../services/previewService';
import {buildStressNeutralExportPacket} from './StressNeutralExportPanel';
import type {PreviewModel,MechanicsResult} from '../../types';
it('retains every source numerical row and CSV unit while withholding incompatible declaration witnesses',async()=>{
 const model=modelJson as PreviewModel,result=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'fixture',solver_version:'1',solver_build_ref:'fixture',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest}),before=JSON.stringify({result,analysisRun});const packet=buildStressNeutralExportPacket({model,result,analysisRun});
 expect(packet.result_rows).toHaveLength(result.results.length);expect(packet.csv_text.trim().split('\n')).toHaveLength(result.results.length+1);
 for(const row of packet.result_rows){const original=result.results.find(r=>r.id===row.result_id)!;expect(row.value).toBe(original.value);expect(row.unit).toBe(original.unit);if(resultSemantics(original)?.family==='rotation' && resultSemantics(original)?.category==='physical_quantity'){expect(row.dimension).toBe('angle');expect(row.result_family).toBe('rotation');}}
 for(const w of packet.unit_preservation_witnesses){const row=result.results.find(r=>r.id===w.source_result_ref.ref)!;expect(w.source_quantity.dimension).toBe(row.dimension);expect(w.target_quantity).toEqual(w.source_quantity);}
 expect(packet.diagnostics.some(d=>d.code==='SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE')).toBe(true);expect(JSON.stringify({result,analysisRun})).toBe(before);
 if(process.env.RESULTS_CONTRACT_OUTPUT_DIR){mkdirSync(process.env.RESULTS_CONTRACT_OUTPUT_DIR,{recursive:true});writeFileSync(path.join(process.env.RESULTS_CONTRACT_OUTPUT_DIR,'stress-neutral.packet.json'),JSON.stringify(packet,null,2));}
});

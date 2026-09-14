import {fireEvent,render,screen,waitFor} from '@testing-library/react';
import {it,expect} from 'vitest';
import modelJson from '../../../../../fixtures/product_preview/invented_preview_model.json';
import resultJson from '../../../../../fixtures/product_preview/invented_mechanics_result.json';
import type {PreviewModel,MechanicsResult} from '../../types';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,bindSourceResultDimensions} from '../../services/previewService';
import {resultDigest} from './resultExportAdapter';
import {ResultExportPanel} from './ResultExportPanel';
it('only makes Current proof-bound JSON downloadable and invalidates it on same-id edit',async()=>{
 const model=structuredClone(modelJson) as PreviewModel,result=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'open_pipe_stress_product_physics',solver_version:'0.1.0',solver_build_ref:'local_preview',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest}),props={model,result,analysisRun,inputManifest};const view=render(<ResultExportPanel {...props}/>);
 const link=await screen.findByTestId('result-export-link',{}, {timeout:20000});expect(link.getAttribute('href')).toBeNull();fireEvent.click(screen.getByTestId('result-export-link-local-private-intent'));const packet=JSON.parse(decodeURIComponent(screen.getByTestId('result-export-link').getAttribute('href')!.split(',')[1]));expect(packet.schema_version).toBe('0.2.0');expect(packet.export_format_status.additional_formats).toBe('TBD');const payload=structuredClone(packet);delete payload.result_envelope.reproducibility.derivative_hash;expect(packet.result_envelope.reproducibility.derivative_hash.value).toBe(await resultDigest(payload));
 const changed=structuredClone(model);(changed as any).unknown_metadata='edited';view.rerender(<ResultExportPanel {...props} model={changed}/>);expect(screen.queryByTestId('result-export-link')).toBeNull();await waitFor(()=>expect(screen.getByTestId('result-export-empty').textContent).toContain('MISMATCH'));
});

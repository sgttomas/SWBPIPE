import {act,fireEvent,render,screen,waitFor} from '@testing-library/react';
import {afterEach,it,expect,vi} from 'vitest';
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


import {invoke} from '@tauri-apps/api/core';
vi.mock('@tauri-apps/api/core',()=>({invoke:vi.fn()}));
afterEach(()=>{delete (window as any).__TAURI_INTERNALS__;vi.mocked(invoke).mockReset();});
async function currentProps(){
 const model=structuredClone(modelJson) as PreviewModel,result=bindSourceResultDimensions(structuredClone(resultJson) as unknown as MechanicsResult);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:'open_pipe_stress_product_physics',solver_version:'0.1.0',solver_build_ref:'local_preview',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});return {model,result,analysisRun,inputManifest};
}
it('native caller retains actual Current proof and exact browser serialized href, then suppresses stale edit completion',async()=>{
 const props=await currentProps(),view=render(<ResultExportPanel {...props}/>);
 await screen.findByTestId('result-export-link',{}, {timeout:20000});fireEvent.click(screen.getByTestId('result-export-link-local-private-intent'));
 const originalHref=screen.getByTestId('result-export-link').getAttribute('href')!,originalName=screen.getByTestId('result-export-link').getAttribute('download')!;
 (window as any).__TAURI_INTERNALS__={};view.rerender(<ResultExportPanel {...props}/>);
 const button=screen.getByTestId('result-export-link');expect(button.tagName).toBe('BUTTON');expect(button).not.toHaveAttribute('href');expect(button).not.toHaveAttribute('download');
 let finish!:(value:unknown)=>void;vi.mocked(invoke).mockImplementation((()=>new Promise<unknown>(resolve=>{finish=resolve;})) as typeof invoke);fireEvent.click(button);
 expect(invoke).toHaveBeenCalledTimes(1);const request=(vi.mocked(invoke).mock.calls[0][1] as any).request;expect(request.href).toBe(originalHref);expect(request.file_name).toBe(originalName);expect(request.screening.exact_payload_match).toBe(true);
 const doc=JSON.parse(decodeURIComponent(request.href.slice(request.href.indexOf(',')+1)));expect(doc.schema_version).toBe('0.2.0');expect(doc.result_envelope.model_ref.ref_id).toBe(props.model.project.id);expect(doc.result_envelope.run_ref.ref_id).toBe(props.result.run_id);expect(doc.result_envelope.row_accounting.map((row:any)=>row.source_result_id)).toEqual(props.result.results.map(row=>row.id));
 const payload=structuredClone(doc);delete payload.result_envelope.reproducibility.derivative_hash;expect(doc.result_envelope.reproducibility.derivative_hash.value).toBe(await resultDigest(payload));
 const changed=structuredClone(props.model);(changed as any).unknown_metadata='same-id edited';view.rerender(<ResultExportPanel {...props} model={changed}/>);expect(screen.queryByTestId('result-export-link')).toBeNull();
 await act(async()=>{finish({outcome:'saved',file_name:originalName,byte_count:123,replaced_existing:false,durability:'not_guaranteed',path_containment:'best_effort_non_adversarial'});});expect(screen.queryByTestId('result-export-link-native-save-status')).toBeNull();await waitFor(()=>expect(screen.getByTestId('result-export-empty')).toHaveTextContent('MISMATCH'));
});
it('Historical or absent Current manifest cannot expose native caller and invalidates an admitted pending completion',async()=>{
 (window as any).__TAURI_INTERNALS__={};const props=await currentProps();const view=render(<ResultExportPanel {...props}/>);await screen.findByTestId('result-export-link',{}, {timeout:20000});fireEvent.click(screen.getByTestId('result-export-link-local-private-intent'));
 let finish!:(value:unknown)=>void;vi.mocked(invoke).mockImplementation((()=>new Promise<unknown>(resolve=>{finish=resolve;})) as typeof invoke);fireEvent.click(screen.getByTestId('result-export-link'));const request=(vi.mocked(invoke).mock.calls[0][1] as any).request;
 // Historical restored records have no Current input-manifest proof. Do not
 // manufacture it from the stored result or analysis-run record.
 view.rerender(<ResultExportPanel {...props} inputManifest={null}/>);expect(screen.queryByTestId('result-export-link')).toBeNull();await act(async()=>{finish({outcome:'saved',file_name:request.file_name,byte_count:123,replaced_existing:false,durability:'not_guaranteed',path_containment:'best_effort_non_adversarial'});});await waitFor(()=>expect(screen.getByTestId('result-export-empty')).toHaveTextContent('CURRENT_INPUT_MANIFEST_UNAVAILABLE'));expect(screen.queryByRole('button',{name:'Local result JSON'})).toBeNull();expect(screen.queryByTestId('result-export-link-native-save-status')).toBeNull();
});

import {act,fireEvent,render,screen,waitFor} from '@testing-library/react';
import {afterEach,it,expect,vi} from 'vitest';
import {invoke} from '@tauri-apps/api/core';
import {buildCurrentSessionInputManifest} from '../../services/inputManifestService';
import {buildAnalysisRunPreview,loadBundledMechanicsReference,runPreviewMechanics} from '../../services/previewService';
import {buildCurrentResultExport,resultDigest} from './resultExportAdapter';
import {ResultExportPanel} from './ResultExportPanel';
import {createNativeMechanicsReplay,nativeMechanicsReplayPair} from '../../test/nativeMechanicsReplay';
vi.mock('@tauri-apps/api/core',()=>({invoke:vi.fn()}));
afterEach(()=>{delete (window as any).__TAURI_INTERNALS__;vi.mocked(invoke).mockReset();});

// Scoped IPC simulation of a genuine supported input/output pair, not a native
// product witness. Registration goes through the production invocation path.
async function currentProps(){
 const {model}=nativeMechanicsReplayPair();
 (window as any).__TAURI_INTERNALS__={};
 vi.mocked(invoke).mockImplementation(createNativeMechanicsReplay().invoke);
 const result=await runPreviewMechanics(model);
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:result.producer!.component_name,solver_version:result.producer!.component_version,solver_build_ref:'open_pipe_stress_product_physics@0.2.0',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 vi.mocked(invoke).mockReset();
 return {model,result,analysisRun,inputManifest};
}

it('only exposes invocation-bound JSON and invalidates it on same-id edit',async()=>{
 const props=await currentProps(),view=render(<ResultExportPanel {...props}/>);
 const button=await screen.findByTestId('result-export-link');expect(button).toBeDisabled();
 fireEvent.click(screen.getByTestId('result-export-link-local-private-intent'));expect(button).toBeEnabled();
 expect(button).not.toHaveAttribute('href');
 const packet=await buildCurrentResultExport(props);expect(packet.schema_version).toBe('0.3.0');
 expect(packet.export_format_status.additional_formats).toBe('TBD');
 const payload=structuredClone(packet);delete payload.result_envelope.reproducibility.derivative_hash;
 expect(packet.result_envelope.reproducibility.derivative_hash.value).toBe(await resultDigest(payload));
 const changed=structuredClone(props.model);(changed as any).unknown_metadata='edited';
 view.rerender(<ResultExportPanel {...props} model={changed}/>);expect(screen.queryByTestId('result-export-link')).toBeNull();
 await waitFor(()=>expect(screen.getByTestId('result-export-empty').textContent).toContain('MISMATCH'));
});

it('native save sends exact serialized bytes and suppresses a stale edit completion',async()=>{
 const props=await currentProps();const originalHref=`data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(await buildCurrentResultExport(props)))}`;
 const view=render(<ResultExportPanel {...props}/>);
 await screen.findByTestId('result-export-link');fireEvent.click(screen.getByTestId('result-export-link-local-private-intent'));
 const button=screen.getByTestId('result-export-link');expect(button.tagName).toBe('BUTTON');expect(button).not.toHaveAttribute('href');expect(button).not.toHaveAttribute('download');
 let finish!:(value:unknown)=>void;vi.mocked(invoke).mockImplementation((()=>new Promise<unknown>(resolve=>{finish=resolve;})) as typeof invoke);fireEvent.click(button);
 expect(invoke).toHaveBeenCalledTimes(1);const request=(vi.mocked(invoke).mock.calls[0][1] as any).request;
 expect(request.href).toBe(originalHref);expect(request.file_name).toMatch(/^openpipestress-preview-results-.*\.json$/);expect(request.screening.exact_payload_match).toBe(true);
 const doc=JSON.parse(decodeURIComponent(request.href.slice(request.href.indexOf(',')+1)));expect(doc.schema_version).toBe('0.3.0');expect(doc.result_envelope.model_ref.ref_id).toBe(props.model.project.id);expect(doc.result_envelope.run_ref.ref_id).toBe(props.result.run_id);expect(doc.result_envelope.row_accounting.map((row:any)=>row.source_result_id)).toEqual(props.result.results.map(row=>row.id));
 const payload=structuredClone(doc);delete payload.result_envelope.reproducibility.derivative_hash;expect(doc.result_envelope.reproducibility.derivative_hash.value).toBe(await resultDigest(payload));
 const changed=structuredClone(props.model);(changed as any).unknown_metadata='same-id edited';view.rerender(<ResultExportPanel {...props} model={changed}/>);expect(screen.queryByTestId('result-export-link')).toBeNull();
 await act(async()=>{finish({outcome:'saved',file_name:request.file_name,byte_count:123,replaced_existing:false,durability:'not_guaranteed',path_containment:'best_effort_non_adversarial'});});expect(screen.queryByTestId('result-export-link-native-save-status')).toBeNull();await waitFor(()=>expect(screen.getByTestId('result-export-empty')).toHaveTextContent('MISMATCH'));
});
it('Historical or absent Current manifest cannot expose native save or publish an earlier completion',async()=>{
 const props=await currentProps();const view=render(<ResultExportPanel {...props}/>);await screen.findByTestId('result-export-link');fireEvent.click(screen.getByTestId('result-export-link-local-private-intent'));
 let finish!:(value:unknown)=>void;vi.mocked(invoke).mockImplementation((()=>new Promise<unknown>(resolve=>{finish=resolve;})) as typeof invoke);fireEvent.click(screen.getByTestId('result-export-link'));const request=(vi.mocked(invoke).mock.calls[0][1] as any).request;
 view.rerender(<ResultExportPanel {...props} inputManifest={null}/>);expect(screen.queryByTestId('result-export-link')).toBeNull();await act(async()=>{finish({outcome:'saved',file_name:request.file_name,byte_count:123,replaced_existing:false,durability:'not_guaranteed',path_containment:'best_effort_non_adversarial'});});await waitFor(()=>expect(screen.getByTestId('result-export-empty')).toHaveTextContent('CURRENT_INPUT_MANIFEST_UNAVAILABLE'));expect(screen.queryByRole('button',{name:'Local result JSON'})).toBeNull();expect(screen.queryByTestId('result-export-link-native-save-status')).toBeNull();
});
it('a bundled record stays unavailable even with a correctly bound reference analysis',async()=>{
 const {model,source:result}=await loadBundledMechanicsReference();
 const inputManifest=await buildCurrentSessionInputManifest({model,solver:{solver_name:result.producer!.component_name,solver_version:result.producer!.component_version,solver_build_ref:'reference-only',solver_mode:'sparse_interactive',settings:{}},active_rule_packs:[],external_assets:[]});
 const analysisRun=await buildAnalysisRunPreview(result,{inputManifest});
 render(<ResultExportPanel model={model} result={result} analysisRun={analysisRun} inputManifest={inputManifest}/>);
 await waitFor(()=>expect(screen.getByTestId('result-export-empty')).toHaveTextContent('CURRENT_NATIVE_INVOCATION_REQUIRED'));
 expect(screen.queryByTestId('result-export-link')).toBeNull();
});

it.each(['source','model','analysis','manifest','mode'] as const)('rejects an in-place %s edit at native save activation',async change=>{
 const props=await currentProps(),view=render(<ResultExportPanel {...props}/>);
 await screen.findByTestId('result-export-link');fireEvent.click(screen.getByTestId('result-export-link-local-private-intent'));
 const button=screen.getByTestId('result-export-link');await waitFor(()=>expect(button).toBeEnabled());
 if(change==='source')props.result.results[0].value+=1;
 else if(change==='model')props.model.project.name+=' changed';
 else if(change==='analysis')props.analysisRun.analysis_run.run_name+=' changed';
 else if(change==='manifest')props.inputManifest.manifest.model_basis.model_payload.project.name+=' changed';
 else props.inputManifest.manifest.solver_basis.solver_mode='dense_scrutiny';
 expect(fireEvent.click(button)).toBe(false);expect(invoke).not.toHaveBeenCalled();
 expect(view.queryByTestId('result-export-link')).toBeNull();
});

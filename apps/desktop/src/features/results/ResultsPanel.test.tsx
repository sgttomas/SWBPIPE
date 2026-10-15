import {cleanup,render,screen} from '@testing-library/react';
import {it,expect} from 'vitest';
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {ResultsPanel} from './ResultsPanel';
import type {MechanicsResult} from '../../types';
import {currentCarrierHeader} from '../../test-support/currentCarrierHeader';
import unicodeSparse from '../../../../../fixtures/results/preview_physics_unicode_ids_sparse.json';
it('keeps rotations separate and nonphysical work/count/review visible without governing ratio',()=>{
 const result={...currentCarrierHeader(),run_id:'fixture',model_ref:'fixture',status:{mechanics:'MECHANICS_SOLVED',rule_check:'RULE_INPUTS_INCOMPLETE',professional_acceptance:'NOT_PROVIDED'},summary:{},diagnostics:[],results:[{id:'rotation',kind:'global_nodal_rotation_x',value:0.1,unit:'rad',entity_ref:'node',metadata:{component:'nodal_rotation_x',coordinate_system:'global',location:'node',basis:'solved_from_global_linear_system',sign_convention:'right hand'}},{id:'work',kind:'nonlinear_support_free_dof_work_residual',value:2,unit:'N*m',dimension:'moment',entity_ref:'fixture',metadata:{component:'free_dof_work_residual',coordinate_system:'solver_iteration',location:'load_case',basis:'fixture',sign_convention:'diagnostic'}}]} as unknown as MechanicsResult;
 render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={()=>{}}/>);expect(screen.getByTestId('result-row-rotation')).toBeTruthy();expect(screen.getByTestId('result-row-work').textContent).toContain('diagnostic_work');expect(screen.getByTestId('governing-ratio-status').textContent).toContain('unavailable');
});

it('shows exact signed tiny scientific values and historical recompute qualification',()=>{
 const result={schema_version:'0.1.0',document_kind:'openpipestress.mechanics_result',run_id:'tiny',model_ref:'fixture',status:{mechanics:'MECHANICS_SOLVED',rule_check:'RULE_INPUTS_INCOMPLETE',professional_acceptance:'NOT_PROVIDED'},summary:{},diagnostics:[],results:[1e-12,-1e-12,1.0000000000000002].map((value,i)=>({id:`tiny:${i}`,kind:'global_nodal_rotation_x',value,unit:'rad',entity_ref:'node'}))} as MechanicsResult;
 const before=JSON.stringify(result);
 render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={'tiny:1'} onSelectResult={()=>{}}/>);
 for(let i=0;i<3;i++) expect(screen.getByTestId(`result-row-tiny:${i}`).textContent).toContain(String(result.results[i].value));
 expect(screen.getAllByTestId('numerical-result-standing').at(-1)?.textContent).toContain('Needs recompute');
 expect(JSON.stringify(result)).toBe(before);
});

// DEL-07-04: a mechanics display shows its diagnostics and limitations. The published
// limitations of every route that shows rows with them (pressure-1 and pressure-source-1,
// each with and without the load/reference-state group, preview-physics-1, and both
// retained successors, whose statements carry the fixture's `source` member) appear in
// the standing line once, joined in published order.
it.each([
 ['pressure-1','fixtures/results/pressure_connected_mechanics_sparse.json',null],
 ['pressure-1 with load/reference states','fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json',null],
 ['pressure-source-1','fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json',null],
 ['pressure-source-1 with load/reference states','fixtures/product_preview/pressure_source/load_reference_source/mixed-sparse_interactive.raw.json',null],
 ['preview-physics-1','fixtures/results/preview_physics_unicode_ids_sparse.json',null],
 ['physics-retained-1','fixtures/results/retained_precision_exact_successor_dense_scrutiny.json','source'],
 ['preview-physics-retained-1','fixtures/results/retained_precision_milestone_successor_sparse_interactive.json','source'],
] as const)('shows every formulation limitation of a %s result, once and in order',(_,path,member)=>{
 const read=JSON.parse(readFileSync(resolve(__dirname,'../../../../../',path),'utf8'));
 const result=(member?read[member]:read) as MechanicsResult;
 const limitations=result.formulation_basis!.limitations;
 expect(limitations.length).toBeGreaterThan(0);
 render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={()=>{}}/>);
 const standing=screen.getByTestId('numerical-result-standing').textContent??'';
 expect(standing.split(limitations.join(' ')).length-1).toBe(1);
 cleanup();
});

it('details a preview-physics-1 combination row with its recovery basis and every source result',()=>{
 // The product's actual preview-physics-1 output for the unicode-ids vector, whose
 // linear combination combination:Σ is published (the demo's is withheld).
 const result=structuredClone(unicodeSparse) as unknown as MechanicsResult;
 const before=JSON.stringify(result);
 const row=result.results.find(r=>r.kind==='element_local_axial_force'&&r.basis_ref?.ref_type==='combination'&&(r.source_result_refs?.length??0)>0)!;
 expect(row.id).toBe('result:combination:combination-Σ:force:pipe-α-β:axial');
 expect(row.source_result_refs).toEqual(['result:force:pipe-α-β:axial','result:loadcase:load-𝔫:force:pipe-α-β:axial']);
 render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={row.id} onSelectResult={()=>{}}/>);
 expect(screen.getByTestId('selected-result-id').textContent).toContain(row.id);
 expect(screen.getByTestId('selected-result-recovery-basis').textContent).toContain('explicit_user_linear_combination; combination:combination:Σ');
 const sources=screen.getByTestId('selected-result-source-refs').textContent;
 expect(sources).not.toContain('not a combined result');
 for(const ref of row.source_result_refs!) expect(sources).toContain(ref);
 expect(JSON.stringify(result)).toBe(before);
});

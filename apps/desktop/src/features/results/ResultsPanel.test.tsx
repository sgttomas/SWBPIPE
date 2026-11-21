import {cleanup,render,screen} from '@testing-library/react';
import {it,expect} from 'vitest';
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {ResultsPanel} from './ResultsPanel';
import {OUTPUT_SURFACES,PRESSURE_2_OUTPUT_REFUSAL,surfaceRouteRefusal} from './outputPolicy';
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
// limitations of every route that shows rows with them (pressure-2 and pressure-source-1,
// each with and without the load/reference-state group, preview-physics-2, and both
// retained successors, whose statements carry the fixture's `source` member) appear in
// the standing line once, joined in published order.
it.each([
 ['pressure-2','fixtures/results/pressure_connected_mechanics_sparse.json',null],
 ['pressure-2 with load/reference states','fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json',null],
 ['pressure-source-1','fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json',null],
 ['pressure-source-1 with load/reference states','fixtures/product_preview/pressure_source/load_reference_source/mixed-sparse_interactive.raw.json',null],
 ['preview-physics-2','fixtures/results/preview_physics_unicode_ids_sparse.json',null],
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

it('details a preview-physics-2 combination row with its recovery basis and every source result',()=>{
 // The product's actual preview-physics-2 output for the unicode-ids vector, whose
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

// T4-U6 (D-5, design section 2.2): an exact-contract result carries one stress headline per
// section basis and a null max_open_formula_stress. The session results view shows both,
// labelled by basis, and never picks a governing one. It is a view: since epoch 8 the
// identity's outputs are the three opened surfaces' (DEL-04-07 Design
// `desktop-exact-outputs.md` §5.1), and every other surface refuses it. The committed pressure-2 envelope is the producer's
// D-5 output (coincident bases); the reduced headline's value is changed here (an invented
// value) so the two shown headlines differ.
it('shows both per-basis stress headlines of an exact result, labelled by basis, and chooses no governing basis',()=>{
 const read=JSON.parse(readFileSync(resolve(__dirname,'../../../../../fixtures/results/pressure_connected_mechanics_sparse.json'),'utf8')) as MechanicsResult;
 expect(read.summary.max_open_formula_stress).toBeNull();
 const nominalHeadline=read.summary.max_open_formula_stress_nominal_section!;
 const reducedHeadline=read.summary.max_open_formula_stress_reduced_section!;
 expect(nominalHeadline.result_ref.endsWith(':nominal_section')).toBe(true);
 expect(reducedHeadline.result_ref.endsWith(':reduced_section')).toBe(true);
 const result=structuredClone(read);
 result.summary.max_open_formula_stress_reduced_section={...reducedHeadline,value:reducedHeadline.value*1.25};
 const before=JSON.stringify(result);
 render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={()=>{}}/>);
 const nominal=screen.getByTestId('section-basis-headline-nominal_section').textContent??'';
 const reduced=screen.getByTestId('section-basis-headline-reduced_section').textContent??'';
 expect(nominal).toContain('Nominal section (nominal wall)');
 expect(nominal).toContain(nominalHeadline.result_ref);
 expect(nominal).toContain(nominalHeadline.location_ref);
 expect(reduced).toContain('Reduced section (wall minus mill tolerance and corrosion allowance)');
 expect(reduced).toContain(reducedHeadline.result_ref);
 const block=screen.getByTestId('section-basis-headlines');
 expect(block.textContent).toContain('no governing basis is chosen');
 expect(block.textContent).not.toMatch(/\bgoverning (basis|headline):/i);
 expect(JSON.stringify(result)).toBe(before);
 for(const surface of OUTPUT_SURFACES) expect(surfaceRouteRefusal(result,surface)).toBe(['result-export','stress-neutral','rule-check','rendered-report','report-package'].includes(surface)?null:PRESSURE_2_OUTPUT_REFUSAL);
 cleanup();
 // A basis whose coverage is incomplete is shown withheld; the other keeps its headline.
 result.summary.max_open_formula_stress_reduced_section=null;
 render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={()=>{}}/>);
 expect(screen.getByTestId('section-basis-headline-reduced_section').textContent).toContain('withheld: a stress maximum on this basis is not available for every member in every load case');
 expect(screen.getByTestId('section-basis-headline-nominal_section').textContent).toContain(nominalHeadline.result_ref);
 cleanup();
 // Without the per-basis keys nothing is shown: a pressure-free result (the producer omits
 // them), and the exact envelope with both keys removed.
 const withoutKeys=structuredClone(read);
 delete withoutKeys.summary.max_open_formula_stress_nominal_section;
 delete withoutKeys.summary.max_open_formula_stress_reduced_section;
 render(<ResultsPanel result={withoutKeys} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={()=>{}}/>);
 expect(screen.queryByTestId('section-basis-headlines')).toBeNull();
 cleanup();
 const pressureFree=structuredClone(unicodeSparse) as unknown as MechanicsResult;
 expect(pressureFree.summary.max_open_formula_stress_nominal_section).toBeUndefined();
 render(<ResultsPanel result={pressureFree} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={()=>{}}/>);
 expect(screen.queryByTestId('section-basis-headlines')).toBeNull();
});

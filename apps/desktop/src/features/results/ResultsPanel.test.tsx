import {render,screen} from '@testing-library/react';
import {it,expect} from 'vitest';
import {ResultsPanel} from './ResultsPanel';
import type {MechanicsResult} from '../../types';
it('keeps rotations separate and nonphysical work/count/review visible without governing ratio',()=>{
 const result={schema_version:'0.1.0',document_kind:'openpipestress.mechanics_result',run_id:'fixture',model_ref:'fixture',status:{mechanics:'MECHANICS_SOLVED',rule_check:'RULE_INPUTS_INCOMPLETE',professional_acceptance:'NOT_PROVIDED'},summary:{},diagnostics:[],results:[{id:'rotation',kind:'global_nodal_rotation_x',value:0.1,unit:'rad',entity_ref:'node',metadata:{component:'nodal_rotation_x',coordinate_system:'global',location:'node',basis:'solved_from_global_linear_system',sign_convention:'right hand'}},{id:'work',kind:'nonlinear_support_free_dof_work_residual',value:2,unit:'N*m',dimension:'moment',entity_ref:'fixture',metadata:{component:'free_dof_work_residual',coordinate_system:'solver_iteration',location:'load_case',basis:'fixture',sign_convention:'diagnostic'}}]} as unknown as MechanicsResult;
 render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={()=>{}}/>);expect(screen.getByTestId('result-row-rotation')).toBeTruthy();expect(screen.getByTestId('result-row-work').textContent).toContain('diagnostic_work');expect(screen.getByTestId('governing-ratio-status').textContent).toContain('unavailable');
});

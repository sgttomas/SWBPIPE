from pathlib import Path
from decimal import Decimal as D, localcontext
import json,math,hashlib,sys
out=Path(__file__).parent;label=sys.argv[1];mode=sys.argv[2];assert mode in ('sparse_interactive','dense_scrutiny')
p=out/'private-rows'/(label+'.json');capture=json.loads(p.read_text());record=capture['row'];model=json.loads(record['model_json']);raw=json.loads(record['mechanics_result_json']);analysis=json.loads(record['analysis_run_json'])
assert model['schema_version']=='0.3.0' and model['pressure_contract']=={'mode':'exact_straight_pressure_v2','version':'2.0.0'}
assert model['materials'][0]['elastic_modulus']=={'unit':'Pa','value':200000000000} and model['materials'][0]['shear_modulus']=={'unit':'Pa','value':77000000000}
assert model['materials'][0]['poisson_ratio']=={'unit':'1','value':0.3} and model['materials'][0]['constitutive_basis']=='homogeneous_isotropic_E_nu_v1'
assert model['pipe_segments'][0]['section']=={'outside_diameter':{'unit':'m','value':0.12},'wall_thickness':{'unit':'m','value':0.01},'material_density':{'unit':'kg/m^3','value':1234}}
assert [n['position'] for n in model['nodes']]==[{'x':0,'y':0,'z':0},{'x':1,'y':0,'z':0}]
cases={c['id']:c for c in model['load_cases']};assert set(cases)=={'case:closed-pressure','case:six-component-load'}
region=cases['case:closed-pressure']['pressure_regions'][0];assert region['pressure']=={'value':2000,'unit':'kPa'} and region['member_pipe_ids']==['pipe:fixture-span']
assert [(t['node_ref'],t['closure_transfer']) for t in region['terminals']]==[('node:fixture-root','transfers_to_wall'),('node:fixture-tip','transfers_to_wall')]
assert cases['case:six-component-load']['pressure_regions']==[]
assert [(l['direction'],l['magnitude']['value'],l['magnitude']['unit']) for l in cases['case:six-component-load']['primitive_loads']]==[('global_x',1000,'N'),('global_y',2000,'N'),('global_z',-3000,'N'),('rotation_x',400,'N*m'),('rotation_y',-500,'N*m'),('rotation_z',600,'N*m')]
assert raw['schema_version']=='0.2.0' and analysis['schema_version']=='0.3.0' and raw['producer']['semantic_contract_id']=='openpipestress.result_semantics/0.3.0/physics-1'
assert raw['numerical_quality']['status']=='checks_passed' and raw['status']['mechanics']=='MECHANICS_SOLVED'
assert len({r['id'] for r in raw['results']})==len(raw['results'])
mode_rows=[r for r in raw['results'] if r['kind']=='linear_solver_mode_basis'];assert len(mode_rows)==2
assert {r['basis_ref']['ref_id'] for r in mode_rows}==set(cases)
assert all(r['value']==(1 if mode=='sparse_interactive' else 2) and 'solver_mode='+mode in r['metadata']['basis'] for r in mode_rows)
parity_rows=[r for r in raw['results'] if r['kind']=='sparse_live_path_dense_parity_relative_delta'];assert len(parity_rows)==(0 if mode=='sparse_interactive' else 2)
with localcontext() as c:
 c.prec=100;a=D(1);b=D(1)/D(2).sqrt();t=D(1)/4;power=D(1)
 for _ in range(9):
  avg=(a+b)/2;b=(a*b).sqrt();t-=power*(a-avg)**2;a=avg;power*=2
 pi=(a+b)**2/(4*t);ro=D('0.06');ri=D('0.05');E=D('200e9');nu=D('0.3');L=D(1);pressure=D('2e6');area=pi*(ro*ro-ri*ri);I=pi*(ro**4-ri**4)/4;J=2*I;G=E/(2*(1+nu));cap=pressure*pi*ri*ri
 extension=(1-2*nu)*cap/(E*area)*L;theta=D(400)*L/(G*J);wrong_theta=D(400)*L/(D('77e9')*J)
 maximum=D(1000)/area+(D(2500)**2+D(2600)**2).sqrt()*ro/I
 expected={'extension_mm':float(extension*1000),'wall_N':float(cap),'membrane_Pa':float(cap/area),'inner_hoop_Pa':float(pressure*(ro*ro+ri*ri)/(ro*ro-ri*ri)),'outer_hoop_Pa':float(2*pressure*ri*ri/(ro*ro-ri*ri)),'theta_x_rad':float(theta),'wrong_authored_G_theta_rad':float(wrong_theta),'maximum_Pa':float(maximum)}
checks=[]
def row(case,kind,entity,component=None,location=None):
 found=[r for r in raw['results'] if r['kind']==kind and r['entity_ref']==entity and r.get('basis_ref')=={'ref_id':case,'ref_type':'load_case'} and (component is None or r['metadata']['component']==component) and (location is None or r['metadata']['location']==location)]
 assert len(found)==1,(case,kind,entity,component,location,len(found));return found[0]
def check(r,expected_value,unit,zero_scale=0):
 value=r['value'];assert type(value) in (int,float) and math.isfinite(value) and r['unit']==unit
 # Existing pressure_runtime.rs policy: no floor for nonzero references;
 # exactly-zero references use the same-unit declared load scale.
 scale=abs(expected_value) if expected_value!=0 else abs(zero_scale)
 error=abs(value-expected_value);passed=error<=1e-9*scale
 checks.append({'result_id':r['id'],'case_ref':r['basis_ref']['ref_id'],'observed':value,'expected':expected_value,'unit':unit,'relative_error':error/scale if scale else 0,'zero_scale':zero_scale if expected_value==0 else None,'passed':passed});assert passed,checks[-1]
pc='case:closed-pressure';mc='case:six-component-load';pipe='pipe:fixture-span';support='support:fixture-root'
check(row(pc,'global_nodal_displacement_x','node:fixture-tip'),expected['extension_mm'],'mm')
for component in ['Fx','Fy','Fz','Mx','My','Mz']:
 r=row(pc,'support_reaction_component_v2',support,component);assert r['metadata']['coordinate_system']=='global'
 check(r,0,'N' if component.startswith('F') else 'N*m',expected['wall_N'])
for location in ['end_i','end_j','quarter_1','midspan','quarter_3']:
 check(row(pc,'pipe_wall_axial_force_v2',pipe,None,location),expected['wall_N'],'N')
 check(row(pc,'pipe_effective_axial_force_v2',pipe,None,location),0,'N',expected['wall_N'])
 check(row(pc,'pipe_axial_membrane_stress_v2',pipe,None,location),expected['membrane_Pa'],'Pa')
 for kind,component,value in [('pipe_lame_radial_stress_v2','lame_inner_radial_stress',-2e6),('pipe_lame_radial_stress_v2','lame_outer_radial_stress',0),('pipe_lame_hoop_stress_v2','lame_inner_hoop_stress',expected['inner_hoop_Pa']),('pipe_lame_hoop_stress_v2','lame_outer_hoop_stress',expected['outer_hoop_Pa'])]:check(row(pc,kind,pipe,component,location),value,'Pa',2e6)
for component,value in zip(['Fx','Fy','Fz','Mx','My','Mz'],[-1000,-2000,3000,-400,-2500,-2600]):
 r=row(mc,'support_reaction_component_v2',support,component);assert r['metadata']['coordinate_system']=='global';check(r,value,'N' if component.startswith('F') else 'N*m')
check(row(mc,'global_nodal_rotation_x','node:fixture-tip'),expected['theta_x_rad'],'rad')
check(row(mc,'pipe_elastic_normal_stress_maximum_v2',pipe),expected['maximum_Pa'],'Pa')
check(row(pc,'pipe_elastic_normal_stress_maximum_v2',pipe),expected['membrane_Pa'],'Pa')
assert abs(expected['theta_x_rad']-expected['wrong_authored_G_theta_rad'])/abs(expected['theta_x_rad'])>1e-6
summary=raw['summary']['max_open_formula_stress'];mr=row(mc,'pipe_elastic_normal_stress_maximum_v2',pipe);assert summary['result_ref']==mr['id'] and summary['value']==mr['value'] and summary['unit']=='Pa'
case_evidence={x['load_case_id']:x for x in raw['contract_evidence']['exact_cases']};maximum_evidence=case_evidence[mc]['pipe_stress_extrema'][0];assert maximum_evidence['station_fraction']==0
result={'checker_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'private_capture_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'native_candidate':'8b982aa7ce64afe37e6067d1038d92608f4aaf3f','case_model_bound':True,'actual_mode':mode,'observed_total_rows':len(raw['results']),'mode_specific_parity_rows':len(parity_rows),'independent_method':'Decimal100 AGM pi; force/moment equilibrium, circular-section integrals, Lamé fields and declared homogeneous isotropic axial/torsional constitutive equations; no production imports','criterion_basis':'Existing core/product_physics/tests/pressure_runtime.rs close/support: nonzero relative1e-9; zero uses declared same-unit cap force, cap force times L=1, or pressure scale','expected':expected,'checks':checks,'count':len(checks),'all_passed':all(x['passed'] for x in checks),'governing_global_case':'case:six-component-load','governing_station_fraction':0,'retained_G77_is_not_consumed':True,'native_call_observed_by':'Root CUA Run action; raw core run_id remains its declared fixed identifier and is not claimed as unique invocation authentication'}
(out/'row-audits'/(label+'-analytical.json')).write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'checks':result['count'],'all_passed':result['all_passed'],'largest_scaled_error':max(x['relative_error'] for x in checks),'expected':expected}))

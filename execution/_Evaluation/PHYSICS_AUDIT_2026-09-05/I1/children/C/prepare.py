from pathlib import Path
import json, subprocess, shutil, math, hashlib, datetime
ROOT=Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip())
W=ROOT/'projects/chirality-piping'; P=Path(__file__).resolve().parent
T=Path('/tmp/piping-audit-I1-C-driver'); (T/'src').mkdir(parents=True,exist_ok=True)
(P/'fixtures').mkdir(exist_ok=True); (P/'results').mkdir(exist_ok=True)
old=W/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/A2/fixtures'
base=json.loads((old/'invented_linear_m.json').read_text())
def clone(x=base): return json.loads(json.dumps(x))
def save(name,x): (P/'fixtures'/f'{name}.json').write_text(json.dumps(x,indent=2)+'\n')
for u in ['m','mm','in']: shutil.copyfile(old/f'invented_linear_{u}.json',P/'fixtures'/f'original_{u}.json')
for name,offset in [('translated',(7,-11,19)),('large_origin',(1e9,-1e9,1e9))]:
 x=clone()
 for n in x['nodes']:
  for a,v in zip(['x','y','z'],offset): n['position'][a]+=v
 save(name,x)
x=clone()
for n in x['nodes']: n['position']['x'],n['position']['y']=-n['position']['y'],n['position']['x']
x['load_cases'][0]['primitive_loads'][0]['direction']='global_x'; x['load_cases'][0]['primitive_loads'][0]['magnitude']['value']=-350
save('rotated_z90',x)
x=clone(); x['pipe_segments'][0]['section']['outside_diameter']={'value':168,'unit':'mm'}; x['pipe_segments'][0]['section']['wall_thickness']={'value':7,'unit':'mm'}
x['materials'][0]['elastic_modulus']={'value':200,'unit':'GPa'}; x['materials'][0]['shear_modulus']={'value':77,'unit':'GPa'}; x['load_cases'][0]['primitive_loads'][0]['magnitude']={'value':0.35,'unit':'kN'}
save('mixed_units',x)
x=clone(); l=x['load_cases'][0]['primitive_loads'][0]; l['direction']='global_rz'; l['dimension']='moment'; l['magnitude']={'value':100,'unit':'N*m'}; save('pure_moment',x)
def spring(dof,node,stiffness=1000000):
 return {'id':'support:spring-'+dof,'node':node,'family':'spring','restraints':[], 'stiffness':{'dof':dof,'value':{'value':stiffness,'unit':'N/m' if dof.startswith('U') else 'N*m/rad'}},'provenance':'invented_audit_analytical'}
def gap(node):
 return {'id':'support:audit-gap','node':node,'family':'nonlinear','restraints':[], 'nonlinear':{'behavior':'gap','dof':'UY','initial_state':'inactive','closes_when':'positive','gap':{'value':1,'unit':'m'}},'provenance':'invented_audit_analytical'}
for name,withspring,withgap in [('linear_spring',True,False),('nonlinear_spring',True,True),('nonlinear_nospring',False,True)]:
 x=clone()
 if withspring:x['supports'].append(spring('UY',x['nodes'][1]['id']))
 if withgap:x['supports'].append(gap(x['nodes'][1]['id']))
 save(name,x)
x=clone(); x['supports']=[spring(d, x['nodes'][0]['id']) for d in ['UX','UY','UZ','RX','RY','RZ']]; save('spring_stabilized',x)
x=clone(); x['supports'][0]['restraints'].remove('UY'); x['supports'].append(spring('UY',x['nodes'][0]['id'])); save('five_rigid_plus_spring',x)
x=clone(); x['supports'][0]['restraints'].remove('UY'); s=gap(x['nodes'][0]['id']); s['nonlinear']['gap']['value']=0; s['nonlinear']['initial_state']='active'; x['supports'].append(s); save('five_rigid_plus_contact',x)
for name,unit in [('missing_length',None),('wrong_length','N')]:
 x=clone()
 if unit is None:del x['project']['units']['length']
 else:x['project']['units']['length']=unit
 save(name,x)
x=clone(); x['nodes'][1]['id']=x['nodes'][0]['id']; save('duplicate_node',x)
x=clone(); x['nodes'][1]['position']['x']=1e308; save('huge_finite_coordinate',x)
x=clone(); x['pipe_segments'][0]['material']='missing'; save('missing_material',x)
x=clone(); x['load_cases'][0]['primitive_loads'][0]['target']['node']='missing'; save('missing_target',x)
for name,dim,unit,value in [('thermal_C','temperature_change','delta_degC',100),('thermal_F','temperature_change','delta_degF',180),('pressure_Pa','pressure','Pa',1000000),('pressure_MPa','pressure','MPa',1),('distributed','force_per_length','N/m',350)]:
 x=clone(); l=x['load_cases'][0]['primitive_loads'][0]; l.update(target={'type':'element','pipe':x['pipe_segments'][0]['id']},dimension=dim,magnitude={'value':value,'unit':unit}); l['category']='thermal' if dim=='temperature_change' else ('pressure' if dim=='pressure' else 'weight'); save(name,x)
D=.168;t=.007;L=2.;E=2e11;F=350.;A=math.pi*(D*D-(D-2*t)**2)/4;I=math.pi*(D**4-(D-2*t)**4)/64;k=3*E*I/L**3
oracle={'frozen_before_actual_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'basis':'independent Euler-Bernoulli closed-form derivation using elementary equilibrium and curvature EI v_second=M; circular annulus geometric integrals; not extracted corpus or production output','definitions':{'D_m':D,'t_m':t,'L_m':L,'E_Pa':E,'F_N':F,'A_m2':A,'I_m4':I,'cantilever_tip_stiffness_N_per_m':k},'expected':{'tip_force_disp_mm':F/k*1000,'tip_force_anchor_force_N':350,'tip_force_anchor_moment_Nm':700,'pure_moment_anchor_force_N':0,'pure_moment_anchor_moment_Nm':100,'spring_tip_displacement_mm':F/(k+1e6)*1000,'nonlinear_open_gap_with_spring_mm':F/(k+1e6)*1000,'thermal_free_extension_mm':1.2e-5*100*L*1000,'pressure_free_extension_mm':1e6*math.pi*(D-2*t)**2/4*L/(E*A)*1000,'uniform_load_tip_disp_mm':350*L**4/(8*E*I)*1000},'criteria':'Published quantities are round6; compare analytical observations with display quantization +/-0.5e-6 and floating error. This is numerical comparison, not adopted engineering acceptance. Equivalent untouched m/mm/in envelopes expected exact identical; transformed components map under rotation. Stable positive-definite spring systems should reach solve, not rigid-count rejection. Source preservation uses input-file SHA equality. Inactive 1m gap must not alter submillimetre spring response.'}
(P/'EXPECTED_BEFORE_RUN.json').write_text(json.dumps(oracle,indent=2)+'\n')
main='''use open_pipe_stress_product_physics::{LinearStaticPreviewRequest,PreviewModel,PreviewSolverMode,run_linear_static_preview_with_mode};
use serde_json::{Value,json};
fn main(){let a:Vec<String>=std::env::args().collect();let text=std::fs::read_to_string(&a[1]).unwrap();let payload:Value=serde_json::from_str(&text).unwrap();let model=match serde_json::from_value::<PreviewModel>(payload){Ok(x)=>x,Err(e)=>{println!("{}",json!({"deserialize_error":e.to_string()}));return;}};let mode=if a.get(2).map(String::as_str)==Some("dense"){PreviewSolverMode::DenseScrutiny}else{PreviewSolverMode::default()};let out=std::panic::catch_unwind(||run_linear_static_preview_with_mode(LinearStaticPreviewRequest{model,materials:vec![]},mode));match out{Ok(x)=>println!("{}",serde_json::to_string_pretty(&x).unwrap()),Err(_)=>println!("{}",json!({"panic":true}))}}
'''
(P/'driver.rs').write_text(main);(T/'src/main.rs').write_text(main)
(T/'Cargo.toml').write_text('[package]\nname="piping-audit-i1-c-driver"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nopen_pipe_stress_product_physics={path="'+str(W/'core/product_physics')+'"}\nserde_json="1"\n')
files={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in (W/'core').rglob('*.rs')}
(P/'SOURCE_BEFORE.json').write_text(json.dumps({'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'sources':files,'fixtures':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (P/'fixtures').glob('*.json')},'oracle_sha256':hashlib.sha256((P/'EXPECTED_BEFORE_RUN.json').read_bytes()).hexdigest()},indent=2)+'\n')
print(T)

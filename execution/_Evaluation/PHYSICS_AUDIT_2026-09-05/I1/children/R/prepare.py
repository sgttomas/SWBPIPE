from pathlib import Path
import json,hashlib,subprocess,math
p=Path(__file__).resolve().parent
root=Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip()); w=root/'projects/chirality-piping'
prior=w/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/A2'
base=json.loads((prior/'fixtures/invented_linear_m.json').read_text()); base['pipe_segments'][0]['y_reference']={'x':0,'y':1,'z':0}
fixtures={}
for name,direction,dimension,unit,value in [('axial','global_x','force','N',350),('shear_y','global_y','force','N',350),('shear_z','global_z','force','N',350),('torsion','global_rx','moment','N*m',350)]:
 x=json.loads(json.dumps(base)); a=x['load_cases'][0]['primitive_loads'][0]; a.update(direction=direction,dimension=dimension,magnitude={'value':value,'unit':unit}); fixtures[name]=x
for name,extent in [('uniform',None),('partial',{'start':0.25,'end':0.75})]:
 x=json.loads(json.dumps(base)); a=x['load_cases'][0]['primitive_loads'][0]; a.update(target={'type':'element_uniform','element':'pipe:P-100'},direction='global_y',dimension='force_per_length',magnitude={'value':100,'unit':'N/m'})
 if extent: a['extent']=extent
 fixtures[name]=x
fixtures['identity_base']=base
x=json.loads(json.dumps(base));x['nodes'][1]['position']['x']=3;fixtures['identity_geometry']=x
x=json.loads(json.dumps(base));x['materials'][0]['elastic_modulus']['value']*=0.5;fixtures['identity_material']=x
for name in ['invented_nonlinear','invented_nonlinear_removed']:
 fixtures[name]=json.loads((prior/'fixtures'/f'{name}.json').read_text())
# Opposite loads: signed physical components cancel in the sum, while magnitude summaries require recomputation.
x=json.loads(json.dumps(base)); second=json.loads(json.dumps(x['load_cases'][0]));second['id']='load:R-negative';second['primitive_loads'][0]['id']='load:R-negative-input';second['primitive_loads'][0]['magnitude']['value']=-350;x['load_cases'].append(second)
x['combinations']=[{'id':'combination:R-sum','label':'Opposite equal loads sum','basis':'mechanics','terms':[{'load_case':'load:L-100','factor':1},{'load_case':'load:R-negative','factor':1}]},{'id':'combination:R-sub','label':'Result difference','basis':'result_state_subtraction','terms':[],'minuend_id':'load:L-100','subtrahend_id':'load:R-negative'}]
for mode in ['min','max','min_abs','max_abs']:
 x['combinations'].append({'id':'combination:R-'+mode,'label':mode,'basis':'range_envelope','terms':[],'operand_ids':['load:L-100','load:R-negative'],'mode':mode})
fixtures['opposite_combinations']=x
# Simple fixed axial member plus gap: physically final UX must be the same in support and node views.
x=json.loads(json.dumps(fixtures['axial']));x['supports'].append({'id':'support:R-gap','node':'node:N-110','family':'gap','restraints':[],'nonlinear':{'dof':'UX','gap':{'value':0,'unit':'m'},'direction':'positive','initial_state':'inactive'}});fixtures['axial_gap']=x
for name,x in fixtures.items(): (p/'fixtures'/f'{name}.json').write_text(json.dumps(x,indent=2)+'\n')
D=.168;t=.007;A=math.pi*(D*D-(D-2*t)**2)/4;I=math.pi*(D**4-(D-2*t)**4)/64;J=2*I;E=2e11;G=77e9;L=2;P=350;q=100
oracle={'authored_before_execution':True,'source_commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'independence':'Closed-form section integrals and static free-body equilibrium authored independently of product outputs. Existing fixture input values only; no old outputs used.','criteria':'Exact mechanical identities compared with 1e-6 published scalar quantization floor; analytical errors reported, not engineering acceptance thresholds.','section':{'A':A,'I':I,'J':J},'axial':{'end_i_N':-P,'end_j_N':P,'interior_absolute_N':[P,P,P],'tip_mm':P*L/(E*A)*1000,'stress_absolute_MPa':P/A/1e6},'torsion':{'end_i_Nm':-P,'end_j_Nm':P,'interior_absolute_Nm':[P,P,P],'tip_rad':P*L/(G*J),'stress_absolute_MPa':P*(D/2)/J/1e6},'shear_y':{'end_i_N':-P,'end_j_N':P,'interior_absolute_N':[P,P,P],'moment_absolute_Nm':[P*L*.75,P*L*.5,P*L*.25],'tip_mm':P*L**3/(3*E*I)*1000},'uniform':{'root_force_N':-q*L,'root_moment_Nm':-q*L**2/2,'tip_force_N':0,'tip_moment_Nm':0,'interior_absolute_shear_N':[q*L*.75,q*L*.5,q*L*.25],'interior_absolute_moment_Nm':[q*(L*.75)**2/2,q*(L*.5)**2/2,q*(L*.25)**2/2],'tip_mm_Euler_Bernoulli':q*L**4/(8*E*I)*1000},'opposite_combinations':{'summed_displacement_components':0,'summed_displacement_magnitude':0,'summed_reaction_resultant':0},'nonlinear':'Within the same successful solution/basis, ordinary node component must equal nonlinear support component at same node/DOF; compare published quantization floor, no correctness claim for either numerical state.'}
(p/'EXPECTED_BEFORE_RUN.json').write_text(json.dumps(oracle,indent=2)+'\n')
sourcepaths=['core/product_physics/src/lib.rs','core/loads/stress_recovery/src/lib.rs','core/loads/load_case_algebra/src/lib.rs','core/runner/headless/src/lib.rs','apps/desktop/src-tauri/src/lib.rs','apps/desktop/src/services/previewService.ts','core/analysis_runs/records.py']
(p/'_run_records/SOURCE_BINDING.json').write_text(json.dumps({s:hashlib.sha256((w/s).read_bytes()).hexdigest() for s in sourcepaths},indent=2)+'\n')
(p/'_run_records/PREEXEC_MANIFEST.json').write_text(json.dumps({str(f.relative_to(p)):hashlib.sha256(f.read_bytes()).hexdigest() for f in sorted(p.glob('fixtures/*.json'))}|{'EXPECTED_BEFORE_RUN.json':hashlib.sha256((p/'EXPECTED_BEFORE_RUN.json').read_bytes()).hexdigest()},indent=2)+'\n')
s=Path('/tmp/piping-audit-I1-R-driver');(s/'src').mkdir(parents=True,exist_ok=True)
(s/'Cargo.toml').write_text('[package]\nname="piping-audit-i1-r"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nopen_pipe_stress_product_physics={path="'+str(w/'core/product_physics')+'"}\nserde_json="1"\n')
main='''use open_pipe_stress_product_physics::{LinearStaticPreviewRequest,PreviewModel,PreviewSolverMode,run_linear_static_preview_with_mode};
fn main(){let args:Vec<String>=std::env::args().collect();let input=std::fs::read_to_string(&args[1]).unwrap();let model:PreviewModel=serde_json::from_str(&input).unwrap();let mode=if args.get(2).map(String::as_str)==Some("dense"){PreviewSolverMode::DenseScrutiny}else{PreviewSolverMode::default()};let result=run_linear_static_preview_with_mode(LinearStaticPreviewRequest{model,materials:vec![]},mode);println!("{}",serde_json::to_string_pretty(&result).unwrap());}
'''
(s/'src/main.rs').write_text(main);(p/'driver.rs').write_text(main)
print(json.dumps({'fixtures':len(fixtures),'expected_sha256':hashlib.sha256((p/'EXPECTED_BEFORE_RUN.json').read_bytes()).hexdigest(),'scratch':str(s)}))

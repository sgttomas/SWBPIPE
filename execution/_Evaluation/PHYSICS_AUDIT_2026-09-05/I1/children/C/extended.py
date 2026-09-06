from pathlib import Path
import json,math,datetime,hashlib,subprocess
P=Path(__file__).resolve().parent; B=json.loads((P/'fixtures/original_m.json').read_text());Q=P/'extended';(Q/'fixtures').mkdir(parents=True,exist_ok=True);(Q/'results').mkdir(exist_ok=True)
def clone(x=B):return json.loads(json.dumps(x))
def save(n,x):(Q/'fixtures'/f'{n}.json').write_text(json.dumps(x,indent=2)+'\n')
def q(v,u):return {'value':v,'unit':u}
x=clone();x['materials'][0]['elastic_modulus']=q(200000,'MPa');x['materials'][0]['shear_modulus']=q(77000,'MPa');x['pipe_segments'][0]['section']['outside_diameter']=q(168,'mm');x['pipe_segments'][0]['section']['wall_thickness']=q(7,'mm');save('supported_mixed_units',x)
x=clone();x['nodes'].reverse();save('node_permutation',x)
x=clone();x['pipe_segments'][0]['from'],x['pipe_segments'][0]['to']=x['pipe_segments'][0]['to'],x['pipe_segments'][0]['from'];save('element_reversed',x)
x=clone();x['load_cases'][0]['primitive_loads'][0]['magnitude']['value']*=2;save('double_force',x)
x=clone();x['sections']=[{'id':'section:audit','section_type':'pipe','properties':x['pipe_segments'][0]['section'].copy(),'provenance':'invented_example'}];x['pipe_segments'][0]['section_ref']='section:audit';save('shared_section',x)
x=clone(x);x['sections'][0]['properties']['outside_diameter']=q(.2,'m');save('shared_cache_stale',x)
x=clone();x['pipe_segments'][0]['section']['mill_tolerance']=q(1,'mm');save('mill_reduction',x)
for name,basis in [('exact','cold'),('interpolated',None),('extrapolated',None)]:
 x=clone();m=x['materials'][0];m['temperature_points']=[{'id':n,'temperature':q(T,'degC'),'elastic_modulus':q(E,'Pa'),'shear_modulus':q(G,'Pa'),'thermal_expansion_coefficient':q(a,'1/degC'),'provenance':'invented_example'} for n,T,E,G,a in [('cold',0,2e11,77e9,1.2e-5),('hot',100,1e11,40e9,2e-5)]]
 if basis:x['load_cases'][0]['modulus_basis_ref']=basis
 else:x['load_cases'][0]['modulus_basis_temperature']=q(50 if name=='interpolated' else 150,'degC')
 save('modulus_'+name,x)
for name,kind in [('wind','whole'),('wind_partial','partial'),('wind_duplicate','duplicate'),('wind_overlap','overlap'),('seismic','seismic')]:
 x=clone();lc=x['load_cases'][0];lc['primitive_loads']=[]
 if kind=='seismic':
  x['pipe_segments'][0]['section']['material_density']=q(7850,'kg/m^3');lc['equivalent_static']={'seismic':{'gravity_acceleration':q(10,'m/s^2'),'g_factor_y':q(.5,'1')},'provenance':'invented_example'}
 else:
  wind={'pressure':q(1000,'Pa'),'shape_factor':q(1,'1'),'direction':'global_y','exposed_pipe_refs':['pipe:P-100'] if kind in ['whole','duplicate'] else []}
  if kind=='duplicate':wind['exposed_pipe_refs']*=2
  if kind in ['partial','overlap']:wind['exposed_spans']=[{'pipe_ref':'pipe:P-100','start_fraction':q(.25,'1'),'end_fraction':q(.75,'1')}]*(2 if kind=='overlap' else 1)
  lc['equivalent_static']={'wind':wind,'provenance':'invented_example'}
 save(name,x)
# User entered imposed displacement has no represented field in PreviewSupport: test incoming unknown key as interface finding, not supported mechanics claim.
x=clone();x['supports'][0]['imposed_displacement']={'dof':'UX','value':q(.01,'m')};save('unknown_imposed_field',x)
D=.168;t=.007;L=2;E=2e11;I=lambda t:math.pi*(D**4-(D-2*t)**4)/64;A=math.pi*(D*D-(D-2*t)**2)/4
expected={'frozen_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'basis':'Independent formulas established EXPECTED_BEFORE_RUN; source catalog and schema inspected before executing these fixtures. Duplicate wind must reject or de-duplicate rather than double-count. Unknown imposed field is interface unsupported input observation, not proof implemented displacement capability.','values':{'supported_mixed_units_mm':350*L**3/(3*E*I(t))*1000,'mill_reduction_mm':350*L**3/(3*E*I(.006))*1000,'modulus_interpolated_mm':350*L**3/(3*1.5e11*I(t))*1000,'wind_intensity_N_per_m':168,'wind_whole_resultant_N':336,'wind_partial_resultant_N':168,'seismic_intensity_N_per_m':A*7850*10*.5},'comparison':'round6 quantization only, no engineering acceptance; generated load equivalence corroborates implemented lumped mapping only, not continuum distributed response.'}
(Q/'EXPECTED_BEFORE_RUN.json').write_text(json.dumps(expected,indent=2)+'\n')
(Q/'PRE_RUN_MANIFEST.json').write_text(json.dumps({str(f.relative_to(Q)):hashlib.sha256(f.read_bytes()).hexdigest() for f in Q.rglob('*.json')},indent=2)+'\n')
for f in sorted((Q/'fixtures').glob('*.json')):
 for mode in ['sparse','dense']:
  r=subprocess.run(['/tmp/piping-audit-I1-C-target/debug/piping-audit-i1-c-driver',str(f),mode],capture_output=True,text=True);(Q/'results'/f'{f.stem}_{mode}.json').write_text(r.stdout);(Q/'results'/f'{f.stem}_{mode}.stderr.json').write_text(json.dumps({'stderr':r.stderr,'exit':r.returncode})+'\n')
  x=json.loads(r.stdout);print(f.stem,mode,x.get('analysis_status',{}),x.get('summary',{}).get('max_displacement'),[(d['code'],d['message']) for d in x.get('diagnostics',[]) if d['severity']=='blocking'],x.get('deserialize_error'))

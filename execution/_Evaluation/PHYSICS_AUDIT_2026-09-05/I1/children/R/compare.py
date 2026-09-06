from pathlib import Path
import json,hashlib,sys,subprocess
p=Path(__file__).resolve().parent;w=Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip())/'projects/chirality-piping';sys.dont_write_bytecode=True;sys.path.insert(0,str(w))
from core.analysis_runs.records import build_preview_analysis_run_envelope

def load(name):return json.loads((p/'results'/f'{name}_sparse.json').read_text())
def rows(name):return {r['id']:r for r in load(name)['results']}
def value(name,key):return rows(name).get(key,{}).get('value')
expected=json.loads((p/'EXPECTED_BEFORE_RUN.json').read_text());out={}
for name,kind,component in [('axial','force','axial'),('torsion','moment','torsion'),('shear_y','force','shear-y'),('shear_z','force','shear-z')]:
 out[name]={'status':load(name)['status'],'station_values':[value(name,f'result:{kind}:pipe-P-100:{s}:{component}') for s in ['quarter-1','midspan','quarter-3']],'expected_absolute_constant':350,'tip':{r['id']:r['value'] for r in load(name)['results'] if 'result:disp:node-N-110:' in r['id']}}
out['uniform']={'station_shear':[value('uniform',f'result:force:pipe-P-100:{s}:shear-y') for s in ['quarter-1','midspan','quarter-3']],'station_moment':[value('uniform',f'result:moment:pipe-P-100:{s}:bending-z') for s in ['quarter-1','midspan','quarter-3']],'oracle':expected['uniform'],'tip_mm':value('uniform','result:disp:node-N-110:uy')}
a=rows('invented_nonlinear');b=rows('invented_nonlinear_removed');common=[k for k in a.keys()&b.keys() if 'nonlinear' not in k and 'solver-mode' not in k];out['nonlinear_existing']={'comparable_rows':len(common),'equal_values':sum(a[k]['value']==b[k]['value'] for k in common),'pairs':[['UY at N140',a['result:disp:node-N-140:uy']['value'],a['result:nonlinear-support:support-NL-140:uy-displacement']['value']],['UZ at N130',a['result:disp:node-N-130:uz']['value'],a['result:nonlinear-support:support-NL-130-FRIC:uz-displacement']['value']]]}
out['gap']={k:r['value'] for k,r in rows('axial_gap_v2').items() if 'support-R-gap' in k or ':node-N-110:ux' in k or ':pipe-P-100:axial' in k or 'converged-flag' in k}
out['combinations']={k:r['value'] for k,r in rows('opposite_combinations_v2').items() if 'combination-R-sum' in k and (':disp:node-N-110' in k or ':reaction:' in k)}
identity=[]
for name in ['identity_base','identity_geometry','identity_material']:
 result=load(name);fixture=json.loads((p/'fixtures'/f'{name}.json').read_text());h=hashlib.sha256(json.dumps(fixture,sort_keys=True,separators=(',',':')).encode()).hexdigest();token=result['model_ref'].replace(':','-');record=build_preview_analysis_run_envelope(result,input_manifest_hash=h,input_manifest_ref={'object_type':'InputManifest','ref':f'input-manifest:{token}:{h}'})
 (p/'results'/f'analysis_run_{name}.json').write_text(json.dumps(record,indent=2)+'\n');identity.append({'name':name,'run_id':result['run_id'],'model_ref':result['model_ref'],'max_displacement':result['summary']['max_displacement'],'input_digest':h,'record_hashes':record['analysis_run']['hashes'],'qualification':'Audit-supplied input manifest digest from fixture; tests wrapper reference/hash separation, not authentic frontend manifest generation or native command invocation.'})
out['identity']=identity
out['pressure']={'axial_end_N':value('pressure','result:force:pipe-P-100:axial:end-j'),'axial_end_MPa':value('pressure','result:stress:pipe-P-100:end-j:axial-normal'),'pressure_longitudinal_rows':[r['id'] for r in load('pressure')['results'] if 'pressure-longitudinal' in r['id']],'tip_ux_mm':value('pressure','result:disp:node-N-110:ux')}
out['overflow']={'null_rows':[r['id'] for r in load('overflow')['results'] if r['value'] is None],'status':load('overflow')['status'],'blocking':[d for d in load('overflow')['diagnostics'] if d['severity']=='blocking']}
headless=[]
for f in sorted((p/'results').glob('headless_explicit_*.json')):
 name=f.stem.replace('headless_explicit_','');x=json.loads(f.read_text())['payload'];headless.append({'name':name,'embedded_equals_product':x['mechanics_envelope']==load(name),'runner_run_id':x['runner_result']['run_id'],'runner_diagnostics':x['runner_result']['diagnostics']})
out['headless']=headless
(p/'COMPARISON.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({'nonlinear':out['nonlinear_existing'],'pressure':out['pressure'],'nullrows':len(out['overflow']['null_rows']),'headless':headless},indent=2))

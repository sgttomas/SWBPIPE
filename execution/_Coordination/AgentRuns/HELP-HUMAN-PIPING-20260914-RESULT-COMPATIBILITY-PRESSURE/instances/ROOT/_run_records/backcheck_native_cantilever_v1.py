from pathlib import Path
import json,hashlib,math,copy
base=Path.cwd()
run=base/'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE'
life=run/'instances/NATIVE/FINAL_NATIVE_V4/_run_records/lifecycle'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
refs=[]
def read(rel):
 p=life/rel;refs.append({'path':str(p.relative_to(base)),'sha256':sha(p)});return json.loads(p.read_text())
allowance=0.5e-6+1e-10
length=3.2;od=0.168;wall=0.007;young=200e9
inertia=math.pi*(od**4-(od-2*wall)**4)/64
checks=[]
models={}
for tag,force in [('350_sparse',350),('500_current_before_quit',500),('500_current_after_recovery',500)]:
 model=read(tag+'/model_json.json');raw=read(tag+'/mechanics_result_json.json');models[tag]=model
 nodes={n['id']:n for n in model['nodes']};pipe=model['pipe_segments'][0];material=model['materials'][0];support=model['supports'][0];load=model['load_cases'][0]['primitive_loads'][0]
 assert len(nodes)==2 and len(model['pipe_segments'])==len(model['materials'])==len(model['supports'])==len(model['load_cases'])==1
 assert nodes['node:ROOT']['position']=={'x':0,'y':2.4,'z':0} and nodes['node:TIP']['position']=={'x':length,'y':2.4,'z':0}
 assert pipe['from']=='node:ROOT' and pipe['to']=='node:TIP' and pipe['y_reference']=={'x':0,'y':0,'z':1}
 assert pipe['material']==material['id'] and pipe['section']['outside_diameter']=={'value':od,'unit':'m'} and pipe['section']['wall_thickness']=={'value':wall,'unit':'m'}
 assert material['elastic_modulus']=={'value':young,'unit':'Pa'} and material['shear_modulus']=={'value':77e9,'unit':'Pa'}
 assert support['node']=='node:ROOT' and set(support['restraints'])=={'UX','UY','UZ','RX','RY','RZ'}
 assert load['target']=={'node':'node:TIP','type':'node'} and load['direction']=='global_y' and load['magnitude']=={'value':force,'unit':'N'}
 rows={v['id']:v for v in raw['results']}; assert len(rows)==67
 expectations=[('result:disp:node-TIP',force*length**3/(3*young*inertia)*1000,'mm'),('result:disp:node-TIP:rz',force*length**2/(2*young*inertia),'rad'),('result:reaction:support-ROOT',force,'N'),('result:force:pipe-CANTILEVER:shear-z',force,'N'),('result:moment:pipe-CANTILEVER:bending-y',-force*length,'N*m')]
 for key,expected,unit in expectations:
  got=rows[key];delta=abs(got['value']-expected);passed=got['unit']==unit and delta<=allowance
  checks.append({'run':tag,'row_id':key,'observed':got['value'],'expected_unrounded':expected,'unit':unit,'absolute_error':delta,'absolute_allowance':allowance,'pass':passed})
  assert passed,(tag,key,got,expected,delta)
h350='578ed0da4a4a1aa59eb20dfc2403a3b1fa912305a07ddeb39539c87833fdf028';h500='db1705b6b2f774d51dbc4e042f29283d147140e6669e3e0c154c4ff59054105f'
for tag,expect in [('350_sparse',h350),('undo350',h350),('redo500',h500),('agent500',h500),('500_current_before_quit',h500),('500_current_after_recovery',h500)]:
 p=life/tag/'model_json.json'; assert sha(p)==expect;refs.append({'path':str(p.relative_to(base)),'sha256':sha(p)})
left=copy.deepcopy(models['500_current_before_quit']);left['load_cases'][0]['primitive_loads'][0]['magnitude']['value']=350
assert left==models['350_sparse']
exact=[]
for name in ['model_json.json','mechanics_result_json.json','analysis_run_json.json','model_hash_json.json']:
 a=life/'500_current_before_quit'/name;b=life/'500_current_after_recovery'/name
 assert a.read_bytes()==b.read_bytes();exact.append({'file':name,'sha256':sha(a),'byte_exact':True})
result={'schema':'root-native-cantilever-backcheck-v1','candidate':'8ad37207cf088025623aa1e777a97a6fcb802f48','status':'PASS','method':'Agent 0 independently reads actual saved models and raw results; closed-form annular inertia and Euler-Bernoulli cantilever equations, without importing product implementation. Expectations are unrounded, with the user-approved publication allowance.','inertia_m4':inertia,'checks':checks,'exact_history_and_agent_hashes':{'H350':h350,'H500':h500,'pass':True},'references_and_entities_preserved_across_load_edit':True,'only_model_delta':'load:WF-FY magnitude 350 N to 500 N','recovery_byte_parity':exact,'evidence_refs':refs,'calibration':'Native analytical_comparison files used rounded expectations and a looser 1e-6 numeric allowance. Those observations remain unchanged. This independent record owns acceptance at the exact 0.5e-6 + 1e-10 allowance. Root support reaction row is magnitude; signed global reactions follow the authored frame/equilibrium and are not misrepresented as separate published component rows.'}
out=run/'instances/ROOT/_run_records/NATIVE_CANTILEVER_BACKCHECK_V1.json';assert not out.exists();out.write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'status':'PASS','output':str(out.relative_to(base)),'sha256':sha(out),'checks':len(checks)}))

"""Independent stdlib-only review; never runs build/UI/store/checker tools.

Reads only packet records, explicitly named candidate build inputs/artifacts and
CUSTODY-listed owned private artifacts. Writes aggregate review facts here only.
"""
from pathlib import Path
from decimal import Decimal as D, localcontext
import hashlib,json,math,sys

HERE=Path(__file__).resolve().parent
PACKET=HERE.parent
RUN=PACKET/'_run_records'
ROOT=Path('/private/tmp/piping-engine-integration-20260925')
PRIVATE=Path('/private/tmp/piping-native-engine-records-20260925')
sha=lambda b:hashlib.sha256(b).hexdigest()
file_sha=lambda p:sha(p.read_bytes())
def read(p):return json.loads(p.read_text())
checks=0
def check(value,label):
 global checks
 checks+=1
 assert value,label
custody=read(PACKET/'CUSTODY.json')
public={}
for item in custody['public_files']:
 p=PACKET/item['path'];data=p.read_bytes()
 check(len(data)==item['bytes'] and sha(data)==item['sha256'],'public custody '+item['path']);public[item['path']]=item['sha256']
private={}
for item in custody['private_evidence']:
 p=PRIVATE/item['private_filename']
 if not p.is_file():p=PRIVATE/'private-rows'/item['private_filename']
 data=p.read_bytes();check(len(data)==item['bytes'] and sha(data)==item['sha256'],'private custody '+item['private_filename'])
 private[item['private_filename']]=p
pre=read(RUN/'inputs-pre.json');post=read(RUN/'inputs-post.json')
check(pre==post and len(pre)==1092,'pre/post input inventory')
for item in pre:check(file_sha(ROOT/item['path'])==item['sha256'],'current named build input '+item['path'])
artifact=read(RUN/'artifact-identity.json');app=Path(artifact['app'])
check(file_sha(Path(artifact['executable']))==artifact['executable_sha256']=='f68dc4de906eb5e8c96bb72c8736b7a5a9afec09ba533ada7ac676223fd36dcc','native executable identity')
for item in artifact['bundle_files']:check(file_sha(app/item['path'])==item['sha256'],'bundle component')
desktop=ROOT/'projects/chirality-piping/apps/desktop'
for item in artifact['dist_files']:check(file_sha(desktop/'dist'/item['path'])==item['sha256'],'captured dist component')
for directory,items in artifact['generated_public'].items():
 for item in items:check(file_sha(desktop/'public'/directory/item['path'])==item['sha256'],'captured WASM component')
check((ROOT/'projects/chirality-piping/node_modules/@openpipestress/desktop').resolve()==desktop,'current recorded dependency alias')
build=read(RUN/'build-command.json');check(build['exit_code']==0 and build['environment']['CARGO_NET_OFFLINE']=='true' and build['environment']['npm_config_offline']=='true' and build['environment']['SWBPIPE_LIVE_CONTROL'] is None,'recorded offline/non-live build')
check(read(RUN/'basis.json')['head']==read(RUN/'post-check.json')['head_post']==custody['candidate']=='8b982aa7ce64afe37e6067d1038d92608f4aaf3f','candidate records')

# Independent pi via Machin arctangents, distinct from the packet AGM generator.
with localcontext() as c:
 c.prec=100
 def atan_inverse(n):
  x=D(1)/n;s=D(0);power=x;i=0
  while True:
   term=power/(2*i+1);next_s=s+term if i%2==0 else s-term
   if next_s==s:return s
   s=next_s;power*=x*x;i+=1
 PI=+(16*atan_inverse(5)-4*atan_inverse(239))

saved={}
for name,p in private.items():
 if name.endswith('-saved.json'):
  v=read(p);saved[name]=v
  audit=read(RUN/'row-audits'/name)
  check(audit['private_capture_sha256']==file_sha(p),'selected audit capture hash')
  check(audit['model_json_sha256']==sha(v['row']['model_json'].encode()),'saved model bytes')
  model=read_model=json.loads(v['row']['model_json'])
  check(model['project']['id']==v['project_id']==v['row']['project_id']==audit['project_id'],'owned project binding')
  claim=json.loads(v['row']['model_hash_json'])
  check(claim['value']=='sha256:'+sha(v['row']['model_json'].encode()),'stored model hash and canonical serialized preimage')

def expected_for(model,case_id,row):
 """Closed named fixture equilibrium/compatibility oracle, not generic solver."""
 with localcontext() as c:
  c.prec=80
  check(len(model['pipe_segments'])==1 and len(model['nodes'])==2,'named one-span domain')
  pipe=model['pipe_segments'][0];nodes={n['id']:n['position'] for n in model['nodes']}
  i,j=nodes[pipe['from']],nodes[pipe['to']]
  check(i['y']==j['y']==i['z']==j['z']==0 and i['x']==0,'straight +x domain')
  check(pipe['y_reference']=={'x':0,'y':1,'z':0},'named local axes')
  length=D(str(j['x']))-D(str(i['x']));ro=D(str(pipe['section']['outside_diameter']['value']))/2;t=D(str(pipe['section']['wall_thickness']['value']));ri=ro-t
  check(pipe['section']['outside_diameter']['unit']==pipe['section']['wall_thickness']['unit']=='m','named SI geometry')
  area=PI*(ro*ro-ri*ri);inertia=PI*(ro**4-ri**4)/4;polar=2*inertia
  case=next(x for x in model['load_cases'] if x['id']==case_id)
  material=next(x for x in model['materials'] if x['id']==pipe['material'])
  selected=next(x for x in material['temperature_points'] if x['id']==case['modulus_basis_ref']) if 'modulus_basis_ref' in case else material
  check(selected['elastic_modulus']['unit']=='Pa' and selected['poisson_ratio']['unit']=='1','named E/nu basis')
  E=D(str(selected['elastic_modulus']['value']));nu=D(str(selected['poisson_ratio']['value']));G=E/(2*(1+nu))
  forces={k:D(0) for k in ['global_x','global_y','global_z','rotation_x','rotation_y','rotation_z']}
  for load in case['primitive_loads']:
   check(load['target']=={'type':'node','node':pipe['to']} and load['direction'] in forces,'only authored tip actions')
   check(load['magnitude']['unit']==('N*m' if load['direction'].startswith('rotation') else 'N'),'named load units')
   forces[load['direction']]+=D(str(load['magnitude']['value']))
  fx,fy,fz=[forces['global_'+a] for a in 'xyz'];tx,my,mz=[forces['rotation_'+a] for a in 'xyz']
  regions=case['pressure_regions'];pressure=D(0);cap=D(0)
  if regions:
   check(len(regions)==1 and not any(forces.values()),'separate pure pressure case')
   region=regions[0];check(region['member_pipe_ids']==[pipe['id']] and region['pressure_basis']=='internal_differential_zero_external_v1','explicit region source')
   check([(x['node_ref'],x['closure_transfer']) for x in region['terminals']]==[(pipe['from'],'transfers_to_wall'),(pipe['to'],'transfers_to_wall')],'transferred physical terminals')
   pressure=D(str(region['pressure']['value']))*{'Pa':1,'kPa':1000}[region['pressure']['unit']];cap=pressure*PI*ri*ri
  spring=next((s for s in model['supports'] if s.get('family')=='spring'),None)
  root_rotation=tx/D(str(spring['stiffness']['value']['value'])) if spring else D(0)
  kind=row['kind'];component=row.get('metadata',{}).get('component');entity=row['entity_ref']
  if kind.startswith('global_nodal_'):
   axis=kind[-1]
   if entity==pipe['from']:return root_rotation if kind=='global_nodal_rotation_x' else D(0)
   check(entity==pipe['to'],'named displacement owner')
   if 'displacement' in kind:
    return {'x':(fx+(1-2*nu)*cap)*length/(E*area)*1000,'y':(fy*length**3/3+mz*length**2/2)/(E*inertia)*1000,'z':(fz*length**3/3-my*length**2/2)/(E*inertia)*1000}[axis]
   return {'x':root_rotation+tx*length/(G*polar),'y':(-fz*length**2/2+my*length)/(E*inertia),'z':(fy*length**2/2+mz*length)/(E*inertia)}[axis]
  if kind=='support_reaction_component_v2':
   if spring and entity==spring['id']:return -tx if component=='Mx' else D(0)
   return dict(zip(['Fx','Fy','Fz','Mx','My','Mz'],[-fx,-fy,-fz,D(0) if spring else -tx,-(my-length*fz),-(mz+length*fy)]))[component]
  if kind=='pipe_wall_axial_force_v2':return cap
  if kind=='pipe_effective_axial_force_v2':return D(0)
  if kind=='pipe_axial_membrane_stress_v2':return cap/area
  if kind=='pipe_lame_radial_stress_v2':return -pressure if component=='lame_inner_radial_stress' else D(0)
  if kind=='pipe_lame_hoop_stress_v2':return pressure*(ro*ro+ri*ri)/(ro*ro-ri*ri) if component=='lame_inner_hoop_stress' else 2*pressure*ri*ri/(ro*ro-ri*ri)
  if kind=='element_local_torsional_shear_stress':return tx*ro/polar/D(1000000)
  if kind=='pipe_elastic_normal_stress_maximum_v2':
   end_norm=max(((my-length*fz)**2+(mz+length*fy)**2).sqrt(),(my*my+mz*mz).sqrt())
   return abs((fx+cap)/area)+end_norm*ro/inertia
  raise AssertionError(kind)

comparisons=[];primary_count=repeat_count=0;largest_scaled=D(0)
for p in sorted((RUN/'row-audits').glob('*-analytical.json')):
 report=read(p);name=p.name.replace('-analytical.json','.json');capture=saved[name];record=capture['row'];model=json.loads(record['model_json']);raw=json.loads(record['mechanics_result_json']);analysis=json.loads(record['analysis_run_json'])
 check(report['private_capture_sha256']==file_sha(private[name]),'analytical capture custody')
 checker_name='check_ordinary_native.py' if name.startswith('ordinary') else 'check_fields_native.py' if name.startswith('fields') else 'check_mixed_native.py' if name.startswith('mixed') else 'check_source_torsion.py'
 check(report['checker_sha256']==file_sha(RUN/checker_name),'checker custody')
 if 'reference_sha256' in report:check(report['reference_sha256']==file_sha(RUN/'connected-native-references.json'),'frozen reference custody')
 check(report['count']==len(report['checks']) and report['all_passed'],'count/pass records')
 rows={r['id']:r for r in raw['results']};check(len(rows)==len(raw['results']),'unique native rows')
 check(raw['schema_version']=='0.2.0' and analysis['schema_version']=='0.3.0','raw/derived versions distinct')
 check(analysis['analysis_run']['run_id']==raw['run_id'],'stored analysis/raw run association')
 check(raw['status']['mechanics']=='MECHANICS_SOLVED' and raw['status']['rule_check']=='RULE_INPUTS_INCOMPLETE','no invented rule qualification')
 reprod=analysis['analysis_run']['reproducibility'];check(len(reprod['input_manifest_refs'])==len(reprod['input_manifest_hashes'])==1,'preserved manifest ref/hash only')
 check('input_manifest' not in analysis and 'input_manifest' not in record,'no claimed persisted manifest body')
 mode=report.get('mode',report.get('actual_mode'));source=raw.get('source_block_recovery')
 if source:
  check(source['body']['status']=='qualified','retained selected source status')
  check(all(c['requested_mode']==mode for c in source['body']['cases']),'mode-bound source')
  ids=[c['id'] for c in model['load_cases']];check([c['basis_ref']['ref_id'] for c in source['body']['cases']]==ids,'source requested cases')
  for c,physical in zip(source['body']['cases'],raw['contract_evidence']['exact_cases']):check(c['basis_ref']['ref_id']==physical['load_case_id'] and c['selected_method']==physical['recovery_method'],'physical/source case-method binding')
  if name.startswith('n05'):check(source['body']['cases'][0]['ordinary_attempt']['outcome']=='sensitive','actual N05 ordinary Sensitive')
  if name.startswith('n06'):check(source['body']['cases'][0]['ordinary_attempt']['outcome']=='rejected','actual N06 ordinary rejection')
 else:
  modes=[r for r in raw['results'] if r['kind']=='linear_solver_mode_basis'];check(len(modes)==2 and all('solver_mode='+mode in r['metadata']['basis'] for r in modes),'ordinary actual mode rows')
 for item in report['checks']:
  row=rows[item['result_id']];check(row['value']==item['observed'] and row['unit']==item['unit'],'recorded comparison actual row')
  case=row['basis_ref']['ref_id'];correct=expected_for(model,case,row)
  check(abs(D(str(item['expected']))-correct)<=D('1e-14')*abs(correct) if correct else item['expected']==0,'independently derived expected value')
  # Recorded policy: exact source zeros; ordinary zeros use declared same-unit scale.
  scale=abs(item['expected']) if item['expected'] else item.get('zero_scale',item.get('scale',0)) or 0
  error=abs(row['value']-item['expected']);check(error<=1e-9*scale,'unchanged numerical criterion')
  check(item['passed'],'retained named comparison pass')
  if scale:largest_scaled=max(largest_scaled,D(str(error/scale)))
 count=len(report['checks']);repeated=any(x in name for x in ['reopened','restored'])
 if repeated:repeat_count+=count
 else:primary_count+=count
 comparisons.append({'label':name,'mode':mode,'count':count,'repeated':repeated,'capture_sha256':file_sha(private[name]),'model_sha256':sha(record['model_json'].encode()),'raw_sha256':sha(record['mechanics_result_json'].encode())})
check(primary_count==348 and repeat_count==129 and len(comparisons)==13,'honest comparison and invocation counts')
check(largest_scaled<D('1e-15'),'reported largest scaled error')
preservation=[]
for a,b,keys in [
 ('ordinary-sparse-saved.json','ordinary-reopened-fresh-saved.json',['model_json','mechanics_result_json']),
 ('ordinary-reopened-fresh-saved.json','atomic-rejected-saved.json',['model_json','mechanics_result_json','analysis_run_json','model_hash_json']),
 ('atomic-rejected-saved.json','dependency-refusals-saved.json',['model_json','mechanics_result_json','analysis_run_json','model_hash_json']),
 ('n05-dense-saved.json','n05-reopened-fresh-saved.json',['model_json','mechanics_result_json']),
 ('mixed-sparse-saved.json','mixed-restored-saved.json',['model_json','mechanics_result_json'])]:
 for key in keys:check(saved[a]['row'][key]==saved[b]['row'][key],'before/after exact bytes '+key)
 preservation.append({'before':a,'after':b,'identical_fields':keys})
initial=read(private['atomic-late-invalid-batch.json']);corrected=read(private['atomic-late-invalid-batch-r2.json'])
check(len(initial['operations'])==len(corrected['operations'])==3,'atomic three steps')
check([x['operation_kind'] for x in initial['operations']]==['edit']*3 and [x['operation_kind'] for x in corrected['operations']]==['modify']*3,'setup-token failure distinguished')
check(json.loads(corrected['operations'][-1]['change']['after'])['pressure_contract']['version']=='2.0.99','late-step invalid profile control')

result={'actor':'/root/physics_resume/joined_producer_review','assignment_requester':'/root','role':'TASK Type2','execution':'Independent stdlib hash/record/equilibrium arithmetic only; no original checker/build/native/store execution','public_custody_files':len(public),'private_custody_files':len(private),'maintained_build_inputs_verified':len(pre),'executable_sha256':artifact['executable_sha256'],'checks':checks,'primary_comparisons':primary_count,'repeated_comparisons':repeat_count,'largest_recorded_scaled_error':str(largest_scaled),'analytical_method':'Independent Decimal Machin pi, integral circular section properties, vector equilibrium and axial/Euler-Bernoulli/Saint-Venant/declared uniform-pressure equations for the named cases; no production imports','comparison_records':comparisons,'preservation':preservation,'UI_observation_limit':'UI actions, Current/Historical labels, rejection display, actual Quit and app inventory are ROOT-observed; reviewer did not witness UI','private_payloads_copied':False}
(HERE/'VERIFICATION.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k not in ['comparison_records','preservation']}))

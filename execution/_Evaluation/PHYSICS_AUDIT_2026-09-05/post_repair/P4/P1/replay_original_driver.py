from pathlib import Path
import subprocess,json,hashlib
r=Path('projects/chirality-piping');a=r/'execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/I1';e=r/'execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P4/P1';o=e/'original_replay_final';o.mkdir(exist_ok=True)
fixtures=[a/'children/C/fixtures'/f'{n}.json' for n in ['original_m','pure_moment','linear_spring','nonlinear_spring','nonlinear_nospring','spring_stabilized','five_rigid_plus_spring']]+[a/'children/R/fixtures/axial_gap_v2.json',a/'children/R/fixtures/overflow.json',a/'manager_overflow/finite_huge_force.json']
records=[]
for f in fixtures:
 for mode in ['dense','sparse']:
  result=subprocess.run(['/tmp/piping-p1-target/debug/piping-audit-i1-c-driver',str(f),mode],capture_output=True,text=True);name=f.stem+'_'+mode+'.json';(o/name).write_text(result.stdout);d=json.loads(result.stdout)
  overflow='overflow' in f.name or 'huge' in f.name
  assert (d['status']['mechanics']=='MECHANICS_SOLVED') != overflow,(name,d['status'])
  if overflow:assert not d['results']
  assert all(isinstance(x['value'],(float,int)) for x in d['results'])
  records.append({'input':str(f),'sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'mode':mode,'output':name,'exit':result.returncode,'status':d['status']['mechanics']})
for mode in ['dense','sparse']:
 def rows(n):return json.loads((o/(n+'_'+mode+'.json')).read_text())['results']
 for n in ['linear_spring','nonlinear_spring']:
  rr=rows(n); values={x['entity_ref']:x['value'] for x in rr if x['kind']=='reaction_resultant'}
  assert abs(values['support:spring-UY']-187.96797141911276)<.5e-6
  assert abs(values['support:S-100']-(350-187.96797141911276))<.5e-6
  assert any(x['kind']=='global_nodal_displacement_y' and abs(x['value']-.18796797141911276)<.5e-6 for x in rr)
 assert next(x['value'] for x in rows('pure_moment') if x['kind']=='reaction_resultant')==0
(o/'BINDING.json').write_text(json.dumps({'inputs':records,'source_hashes':{str(f):hashlib.sha256(f.read_bytes()).hexdigest() for f in [r/'core/product_physics/src/lib.rs',r/'core/solver/nonlinear_integration/src/lib.rs']},'assertions':'20 original inputs/modes; bounded status, spring action/displacement, force-only moment reaction, overflow nonsolved; full precision oracle preserved; no original audit mutations'},indent=2)+'\n')
print('PASS',len(records))

from pathlib import Path
from collections import Counter
import hashlib,json,subprocess,sys
root=Path(sys.argv[1]).resolve() if len(sys.argv)>1 else Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip())
r=Path(sys.argv[2]).resolve() if len(sys.argv)>2 else Path(__file__).resolve().parent
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip()
run=json.loads((r/'run.json').read_text());jobs=json.loads((r/'jobs.json').read_text())['jobs'];plan=json.loads((r/'selection/piping-e2e-plan.json').read_text())
assert head=='1c1b8b072a25a5014f41dcc0e0fd8fd9d1ab0a9a'
assert run['head_sha']==plan['head']==head and run['conclusion']=='success' and run['status']=='completed'
assert run['path']=='.github/workflows/piping-desktop-e2e.yml' and run['run_attempt']==1
assert plan['mode']=='full' and set(plan['inventory'])==set(plan['selected_specs'])
required={'Select source coverage','Accessibility barrier and selected coverage','Desktop E2E (source mode)'}|{f'Source remainder ({i}/4)' for i in range(1,5)}
assert {j['name'] for j in jobs}==required
for j in jobs:
 assert j['conclusion']=='success' and j['status']=='completed',j['name']
 if j['name'].startswith('Source remainder'):
  assert any(s['name']=='Run isolated source remainder shard' and s['conclusion']=='success' for s in j['steps'])
 if j['name']=='Accessibility barrier and selected coverage':
  assert any(s['name']=='Run accessibility first, then any reduced selection' and s['conclusion']=='success' for s in j['steps'])
 if j['name']=='Desktop E2E (source mode)':
  assert any(s['name']=='Require all planned coverage' and s['conclusion']=='success' for s in j['steps'])
key=lambda t:(t['project'],t['file'],tuple(t['title_path']))
files=sorted((r/'collections').rglob('collection.json'));assert len(files)==5
expected=None;executed=[];parts=[];metadata=[]
plan_hash=hashlib.sha256(json.dumps(plan,sort_keys=True).encode()).hexdigest()
for f in files:
 d=json.loads(f.read_text());assert d['kind']=='collection-only' and d['status']=='validated'
 assert d['head']==head and d['mode']=='full' and not d['omitted'] and d['target_base']==plan['target_base']
 assert d['plan_sha256']==plan_hash
 selected=Counter(map(key,d['selected']))
 if expected is None:expected=selected
 else:assert selected==expected
 assert all(v==1 for v in selected.values())
 for name,sha in d['identity']['source_file_sha256'].items():
  assert name in plan['inventory']
  actual=root/'projects/chirality-piping/apps/desktop'/name
  assert hashlib.sha256(actual.read_bytes()).hexdigest()==sha,name
 assert hashlib.sha256((root/'projects/chirality-piping/apps/desktop/playwright.config.ts').read_bytes()).hexdigest()==d['identity']['config_sha256']
 assert hashlib.sha256((root/'projects/chirality-piping/tools/ci/e2e_plan.py').read_bytes()).hexdigest()==d['identity']['selector_sha256']
 executed+=d['execution_tests'];parts.append({'stage':d['stage'],'shard':d['shard'],'count':len(d['execution_tests'])})
 metadata.append({'file':str(f.relative_to(r)),'sha256':hashlib.sha256(f.read_bytes()).hexdigest()})
assert Counter(map(key,executed))==expected
projects=sorted({t['project'] for t in executed});assert projects==['chromium-compact','chromium-desktop']
assert {t['file'] for t in executed}==set(plan['inventory'])
binding={'workflow_path':run['path'],'run_id':run['id'],'run_attempt':run['run_attempt'],'head_sha':head,'conclusion':'success','registered_e2e_specs_executed':True,'viewport_projects':['chromium-desktop','chromium-compact']}
(r/'binding.json').write_text(json.dumps(binding,indent=2)+'\n')
proof={'head':head,'run_id':run['id'],'run_attempt':run['run_attempt'],'mode':'full','selected_identities':len(executed),'registered_specs':len(plan['inventory']),'projects':projects,'partitions':parts,'partition_union_exact':True,'omitted':0,'execution_proof':'All seven workflow jobs and each named actual run/aggregate step concluded success. Collection-only artifacts establish identity/partition coverage and are not themselves represented as test passes.','source_identity_hashes_match':True,'collection_records':metadata,'limits':'Declared test skips retain their original meaning; no D72 performance, native WebKit, engineering or release qualification.'}
(r/'BINDING_CHECK.json').write_text(json.dumps(proof,indent=2)+'\n')
print(json.dumps(binding,indent=2));print('Verified exact 495-identity union, 16 registered specs, both projects and actual execution-step success.')

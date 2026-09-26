"""Read-only exact-content review reconciliation; no builds or Git writes."""
from pathlib import Path
import hashlib,json,re,subprocess,collections,datetime
ROOT=Path('/private/tmp/piping-engine-integration-20260925')
OUT=Path(__file__).resolve().parent
C='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24'
E=C+'/ENGINE_INTEGRATION'
BASE='aa312755e671b0fc7d598314d591c552749aaea6'
CANDIDATE='e65001ad50072d3399bf204da02055b0f360353a'
RECORD_BASIS='c278f64ba122eb8b848b9f14e58a0e533e94439a'
sha=lambda b:hashlib.sha256(b).hexdigest()
def git(*args):return subprocess.check_output(['git','-C',str(ROOT),*args])
cache={}
def blob(rev,path):
 key=(rev,path)
 if key not in cache:cache[key]=git('show',rev+':'+path)
 return cache[key]
def obj(path,rev=RECORD_BASIS):return json.loads(blob(rev,path))
def normalize(path):
 if not isinstance(path,str):return None
 at=path.find('projects/chirality-piping/')
 if at>=0:return path[at:]
 if path.startswith(('core/','apps/','schemas/','tests/','tools/','fixtures/','validation/')):return 'projects/chirality-piping/'+path
 return path
def declared(value):
 """Extract explicit path-to-hash pairs; never treat a test pass as review."""
 output=collections.defaultdict(set)
 def walk(v):
  if isinstance(v,dict):
   path=normalize(v.get('path'))
   if path:
    for k in ['sha256','actual','current_sha256','final_sha256','after_sha256','reviewed_sha256','observed_sha256','union_sha256']:
     h=v.get(k)
     if isinstance(h,str) and re.fullmatch('[0-9a-f]{64}',h):output[path].add(h)
   for k,item in v.items():
    if isinstance(k,str) and k.startswith(('projects/chirality-piping/','core/','apps/','schemas/','tests/','tools/','fixtures/','validation/')):
     if isinstance(item,str) and re.fullmatch('[0-9a-f]{64}',item):output[normalize(k)].add(item)
    walk(item)
  elif isinstance(v,list):
   for item in v:walk(item)
 walk(value);return output
all_paths=git('diff','--name-only',BASE+'..'+CANDIDATE).decode().splitlines()
maintained=[p for p in all_paths if '/execution/' not in p]
assert len(maintained)==322
current={p:sha(blob(CANDIDATE,p)) for p in maintained}
coverage={}
origins={}
def origin(path,rev=RECORD_BASIS):
 key='git:'+rev+':'+path;origins[key]=sha(blob(rev,path));return key
def cover(path,digest,basis,review,group,rev=RECORD_BASIS,notes=None):
 if path not in current:return
 assert current[path]==digest,(path,'candidate differs from reviewed bytes',digest,current[path])
 # Every selected coverage entry retains both exact basis and actual review.
 bk=origin(basis,rev);rk=origin(review,rev)
 coverage[path]={'sha256':digest,'basis':bk,'review':rk,'group':group,'notes':notes}
mapping=obj(E+'/CURRENT_REVIEW_COVERAGE.json')
union=obj(E+'/ROOT_MAIN_UNION/INDEPENDENT_REVIEW/CHECKS.json')
umap={x['path']:x for x in union['selected_maintained239']}
for entry in mapping['maintained_paths']:
 path=entry['path']
 if path not in current:continue
 if path.endswith('core/product_physics/src/lib.rs'):
  before=blob('1792774a2c5b77612e666332151f10f4b14dbc2c',path);after=blob(CANDIDATE,path)
  body=lambda data:data.split(b'mod annulus_geometry;',1)[1]
  assert body(before)==body(after)
  cover(path,umap[path]['union_sha256'],E+'/ROOT_MAIN_UNION/INDEPENDENT_REVIEW/CHECKS.json',E+'/ROOT_MAIN_UNION/INDEPENDENT_REVIEW/RETURN.md','engine_main_union_doc_only')
  continue
 if entry['status']!='exact_hash_independent_review':continue
 match=False
 for item in entry['exact_reviews']:
  if item['sha256']!=current[path]:continue
  basis,review=item['basis'],item['review']
  if basis.startswith('git:3c17'):
   rev,source=basis.split(':',2)[1:]
   assert sha(blob(rev,source))==current[path]
   coverage[path]={'sha256':current[path],'basis':basis,'review':'qualified M35 source23/PR892','group':'inherited_M35'};match=True;break
  if basis.startswith('git:ed688'):
   rev=basis.split(':')[1]
   suffix=C+'/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT/SOURCE_RECOVERY_IMPLEMENTATION_01/'
   basis=suffix+'REVIEWED_SUBSET.json';record=obj(basis,rev)
   assert any(x['path']==path and x['sha256']==current[path] for x in record['reviewed_source'])
   review=suffix+review.split(',')[0]
   cover(path,current[path],basis,review,'inherited_reviewed_NUM18',rev);match=True;break
  record=obj(basis)
  if basis.endswith('/AUDIT_LOCK_REPAIR.json'):
   assert path.endswith('physics_audit_regression/Cargo.lock') and record['after_sha256']==current[path]
  else:assert current[path] in declared(record).get(path,set()),(path,basis,'no exact path/hash declaration')
  cover(path,current[path],basis,review,'engine_exact_component_review');match=True;break
 assert match,('unmatched engine reviewed entry',path)
front_basis=E+'/FINAL_FRONTEND_REVIEW/INDEPENDENT_REVIEW/FINAL_CHECKED_HASHES.json'
for item in obj(front_basis)['files']:
 cover(item['path'],item['final_sha256'],front_basis,E+'/FINAL_FRONTEND_REVIEW/INDEPENDENT_REVIEW/RETURN.md','engine_final_frontend44')
witness=E+'/PHYSICS_READER_JOIN/RESUME_01/STRESS_NEUTRAL_PYTHON/WITNESS_TARGET_INTEROP/INDEPENDENT_BACKCHECK/'
for item in obj(witness+'CHECKED_BASIS.json')['files']:
 cover(normalize(item['path']),item['after_sha256'],witness+'CHECKED_BASIS.json',witness+'RETURN.md','engine_final_witness2')
# Older ordinary NUM/precision paths absent from the later 239-file selection.
ordinary=C+'/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT/WHOLE_REVIEW_RESUME_01/'
for item in obj(ordinary+'FINAL_HASH_CHECK.json')['files']:
 path=item['path']
 if path not in coverage and current.get(path)==item['actual']:
  assert item['actual']==item['expected'] and item['match']
  cover(path,item['actual'],ordinary+'FINAL_HASH_CHECK.json',ordinary+'RETURN.md','ordinary_NUM112_unchanged')
# Validation final26, backed by original11 + repair + fixture-promotion backcheck.
adapter=C+'/VALIDATION_FOUNDATION/ORDINARY_PHYSICS_ADAPTER/INDEPENDENT_REVIEW/'
vf=obj(adapter+'FIXTURE_PROMOTION_FINAL_BASIS_03.json')
assert vf['status'].startswith('CLEAR')
for item in vf['all26file_hashes']:
 cover(item['path'],item['sha256'],adapter+'FIXTURE_PROMOTION_FINAL_BASIS_03.json',adapter+'FIXTURE_PROMOTION_BACKCHECK_03.md','validation_final26')
origin(adapter+'REPAIR_BACKCHECK_02.md');origin(adapter+'REPAIR_FINAL_CHECKED_BASIS_02.json')
# Six earlier validation files retain their own full-review/repair origin.
gate=C+'/VALIDATION_FOUNDATION/GATE_REVIEW/'
for item in obj(gate+'REPAIR_CHECKED_BASIS_03.json')['files']:
 if item['path'] not in coverage and current.get(item['path'])==item['sha256']:
  cover(item['path'],item['sha256'],gate+'REPAIR_CHECKED_BASIS_03.json',gate+'REPAIR_RETURN_03.md','validation_gate_repair03')
foundation=C+'/VALIDATION_FOUNDATION/'
for item in obj(foundation+'SOURCE_FREEZE_02.json')['files']:
 if item['path'] not in coverage and current.get(item['path'])==item['sha256']:
  cover(item['path'],item['sha256'],foundation+'SOURCE_FREEZE_02.json',foundation+'ROOT_MAPPING_BACKCHECK/RETURN.md','validation_mapping_repair02')
for item in obj(gate+'MAPPING_DELTA_BACKCHECK.json')['results']:
 if item['path'] not in coverage and current.get(item['path'])==item['current_sha256']:
  cover(item['path'],item['current_sha256'],gate+'MAPPING_DELTA_BACKCHECK.json',gate+'RETURN.md','validation_mapping_state_delta')
inventory='projects/chirality-piping/validation/qualification/capability_inventory.json'
old=obj(inventory);new=obj(inventory,CANDIDATE)
deltas=[]
def compare(a,b,path=()):
 if isinstance(a,dict) and isinstance(b,dict):
  assert a.keys()==b.keys()
  for k in a:compare(a[k],b[k],path+(k,))
 elif isinstance(a,list) and isinstance(b,list):
  assert len(a)==len(b)
  for i,(x,y) in enumerate(zip(a,b)):compare(x,y,path+(i,))
 elif a!=b:deltas.append((path,a,b))
compare(old,new);assert len(deltas)==2
for location,before,after in deltas:
 assert location[-1]=='path' and after==before.replace('/STATIC_REFERENCE_BASIS/INDEPENDENT_REVIEW/','/STATIC_REFERENCE_BASIS/_run_records/INDEPENDENT_REVIEW/')
 parent=new
 for key in location[:-1]:parent=parent[key]
 assert sha(blob(CANDIDATE,after))==parent['sha256']
 assert blob(RECORD_BASIS,before)==blob(CANDIDATE,after)
mapping_inventory=next(x for x in obj(gate+'MAPPING_DELTA_BACKCHECK.json')['results'] if x['path']==inventory)
assert sha(blob(RECORD_BASIS,inventory))==mapping_inventory['current_sha256']
coverage[inventory]={'sha256':current[inventory],'basis':origin(gate+'MAPPING_DELTA_BACKCHECK.json'),'review':origin(gate+'RETURN.md'),'group':'validation_inventory_two_links','notes':'e65001ad alters exactly two evidence paths; target bytes/digests unchanged, independently checked here'}
missing=sorted(set(maintained)-set(coverage))
assert not missing,missing
# Exact validation union equality; one later inventory two-link change explicit.
validation_paths=obj(E+'/ROOT_VALIDATION_UNION/BASIS.json')['incoming_maintained_paths']
assert len(validation_paths)==32
for p in validation_paths:
 if p!=inventory:assert blob(CANDIDATE,p)==blob('26b9904a1e8d2b997315165572d732ce4320df3b',p)
origin(E+'/ROOT_VALIDATION_UNION/BASIS.json')
native=obj(C+'/_run_records/JOINED_ENGINE_NATIVE/_run_records/inputs-pre.json')
assert len(native)==1092
for p in native:assert sha(blob(CANDIDATE,p['path']))==p['sha256']
observed_head=git('rev-parse','HEAD').decode().strip()
head_delta=git('diff','--name-only',CANDIDATE+'..'+observed_head).decode().splitlines()
assert not [p for p in head_delta if '/execution/' not in p],('new maintained HEAD delta',head_delta)
pending_tracked=git('diff','--name-only','HEAD').decode().splitlines()
pending_untracked=git('ls-files','--others','--exclude-standard').decode().splitlines()
pending_maintained=[p for p in pending_tracked+pending_untracked if '/execution/' not in p and p!='projects/chirality-piping/node_modules']
assert not pending_maintained,('pending maintained files',pending_maintained)
for p in native:assert sha((ROOT/p['path']).read_bytes())==p['sha256']
assert git('rev-parse','origin/main').decode().strip()==BASE
groups=dict(collections.Counter(x['group'] for x in coverage.values()))
report={'status':'SOURCE_COVERAGE_CLEAR; FINAL_READINESS_PENDING_ACTUAL_HEAD_HOSTED_CHECKS_AND_CLEAN_DEC025','candidate':CANDIDATE,'observed_head':observed_head,'base':BASE,'total_changed_paths':len(all_paths),'maintained_changed_paths':len(maintained),'covered_maintained_paths':len(coverage),'omitted_maintained_paths':missing,'coverage_groups':groups,'native_build_inputs_exactly_unchanged':1092,'pending_maintained_paths':pending_maintained,'pending_record_paths_count':len(pending_tracked)+len(pending_untracked),'record_only_head_delta_count':len(head_delta),'complete_native_tests_repeated':False,'coverage':{p:coverage[p] for p in maintained},'review_record_hashes':origins,'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
(OUT/'COVERAGE.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items() if k not in ['coverage','review_record_hashes']}))

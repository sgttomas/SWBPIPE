"""Independent bounded repair backcheck: source/preservation, exact count prefixes and semantic equality."""
from pathlib import Path
import hashlib,json,os,re,subprocess
NUM=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/numerics')
CODE=NUM.parent/'f2a';R=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
OUT=Path(__file__).resolve().parent;BULK=NUM.parent/'scratch/rv65_named_support/repair04'
CAND='c79a1c293dbf5581468e839c8545b5a815a32df7';BASE='8104a4fedd0fd575f20b3d723cc0cdd722d73ba1';RP='4f9799a9631895e1bf1bbfe9bb9755728b8ffe2c'
PACKET=CODE/R/'I50/support_accounting_repair_03';OLD_PACKET=CODE/R/'I50/first_publishing_component_02'
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0');sha=lambda b:hashlib.sha256(b).hexdigest()
def git(root,*args):return subprocess.check_output(['git',*args],cwd=str(root),env=env)
def item(p,scope,revision=None):
 b=p.read_bytes();return dict(location=str(p),bytes=len(b),sha256=sha(b),scope_read=scope,revision=revision)
def verify_manifest(j,root=None):
 for e in j['files']:
  p=root/e['path'] if root else Path(e['location']);b=p.read_bytes();assert len(b)==e['bytes'] and sha(b)==e['sha256'],str(p)
assert git(CODE,'rev-parse','HEAD').decode().strip()==CAND
assert not git(CODE,'status','--porcelain').strip()
for p in [PACKET,OLD_PACKET]:
 verify_manifest(json.loads((p/'SEAL.json').read_text()),p)
 verify_manifest(json.loads((p/'BULK_MANIFEST.json').read_text()))
prior=NUM/R/'REVIEW_RV65/named_support_implementation_03'
verify_manifest(json.loads((prior/'SEAL.json').read_text()),prior)
verify_manifest(json.loads((prior/'BULK_MANIFEST.json').read_text()))
assert sha((prior/'SEAL.json').read_bytes())=='76dd57e8822d7ff633f6f96ed5a0a4c054040b669034c2bbc3ffc6e7d3225440'
fm=json.loads((PACKET/'FINAL_SOURCE.json').read_text());changed={e['path'] for e in fm['files']}
assert set(git(CODE,'diff','--name-only',BASE,CAND,'--','projects/chirality-piping/core','projects/chirality-piping/fixtures').decode().splitlines())==changed
for e in fm['files']:
 b=(CODE/e['path']).read_bytes();assert b==git(CODE,'show',CAND+':'+e['path']) and sha(b)==e['sha256'] and len(b)==e['bytes']
baseline=json.loads((NUM.parent/'scratch/i50_first_publishing/repair03/BASELINE.json').read_text())
unchanged=[p for p in baseline if p not in changed]
assert len(unchanged)==955 and all(sha((CODE/p).read_bytes())==baseline[p] for p in unchanged)
source=(CODE/'projects/chirality-piping/core/product_physics/src/retained_product.rs').read_text()
assert source.count('self.fill_support_bitmap(')==3
assert all(x not in source for x in ['built_used.resize','spring_used.resize','rigid_owned.resize','let mut hits = 0'])

log=BULK/'pp_debug.log';text=log.read_text();run=json.loads((BULK/'pp_debug.json').read_text())
assert run['exit']==0 and run['source_stable'] and run['reaped'] and run['candidate']==CAND
assert 'test result: ok. 19 passed; 0 failed' in text
M=(1<<64)-1;prefixes=[]
matches=re.findall(r'I50_BITMAP_SUCCESS n=(\d+) counts=(\[[^\]]*\]) capacities=(\[[^\]]*\])',text)
assert len(matches)==5
for n,counts,caps in matches:
 n=int(n);actual=json.loads(counts);capacity=json.loads(caps)
 # One successful reserve/capacity record, one shape guard and n owned writes.
 expected=[0,0,1+n,1,0,0,1,1,0,capacity[0]]
 assert actual==expected and capacity==[n,0,0,0,0,0,0]
 prefixes.append(dict(kind='success',n=n,expected=expected,observed=actual,capacity=capacity))
after=re.search(r'I50_BITMAP_AFTER_RESERVE prefix=(\[[^\]]*\]) first_capacity=4 second_allocations=0 committed_maps=0',text)
mid=re.search(r'I50_BITMAP_MID_FILL prefix=(\[[^\]]*\]) written=2 reserved=4',text)
assert after and mid
for kind,found,expected in [('after_reserve',after,[0,0,M,5,0,0,1,1,0,4]),('mid_fill',mid,[0,0,M,1,0,0,1,1,0,4])]:
 observed=json.loads(found[1]);assert observed==expected
 prefixes.append(dict(kind=kind,expected=expected,observed=observed))
assert 'I50_TWO_REPEAT_BOUNDARY Association("rigid boundary identity")' in text

common=['REQUEST','CASE','INPUT','ROWS','VERDICTS','PASS','FULL_CASE','G5A','OPERATIONAL','G5A_WORK','TERM_MAP','G5A_DATA','NATIVE_ROWS','NATIVE_IDENTITY']
tags=['I45_'+x for x in common]+['I47_'+x for x in common+['BOUNDARY','SELECTION','BASIS_CAPTURE','ANCILLARY','CAPTURE']]
def read(p):
 d={k:[] for k in tags+['I45_ADAPTER','I47_ADAPTER','I50_RECORD']}
 for line in p.read_text().splitlines():
  k,_,v=line.partition(' ')
  if k not in d:continue
  try:v=json.loads(v)
  except json.JSONDecodeError:pass
  d[k].append(v)
 return d
def counts(s):
 assert 'fault: Cell { value: None }' in s
 return json.loads(re.search(r'counts: Cell \{ value: (\[[^\]]*\])',s)[1])
old=read(NUM.parent/'scratch/rv65_named_support/implementation03/pp_debug.log')
checks=[]
for label,path in [('fresh_debug',log),('author_debug',NUM.parent/'scratch/i50_first_publishing/repair03/pp_debug.log'),('author_optimized',NUM.parent/'scratch/i50_first_publishing/repair03/pp_optimized.log')]:
 new=read(path);count=0;deltas=[]
 for tag in tags:
  assert len(old[tag])==(2 if tag.startswith('I45_') else 4) and new[tag]==old[tag],(label,tag)
  count+=len(old[tag])
 assert count==104
 for tag in ['I45_ADAPTER','I47_ADAPTER']:
  assert len(old[tag])==len(new[tag])==(2 if tag.startswith('I45_') else 4)
  for i,(a,b) in enumerate(zip(old[tag],new[tag])):
   delta=[y-x for x,y in zip(counts(a),counts(b))];assert delta==[0,0,13,3,0,0,0,0,0,0]
   deltas.append(dict(tag=tag,index=i,delta=delta))
 assert len(new['I50_RECORD'])==len(old['I50_RECORD'])==2
 for a,b in zip(old['I50_RECORD'],new['I50_RECORD']):
  assert set(a)==set(b)
  assert {k:v for k,v in a.items() if k!='adapter'}=={k:v for k,v in b.items() if k!='adapter'}
  delta=[y-x for x,y in zip(counts(a['adapter']),counts(b['adapter']))];assert delta==[0,0,19,3,0,0,0,0,0,0]
  deltas.append(dict(tag='I50_RECORD',mode=a['mode'],delta=delta))
 checks.append(dict(label=label,input=item(path,'semantic/count comparison input'),old_semantic_records=count,named_all_other_fields_equal=True,exact_deltas=deltas))
expected_state=dict(baseline);expected_state.update({e['path']:e['sha256'] for e in fm['files']})
digest=sha(json.dumps(expected_state,sort_keys=True,separators=(',',':')).encode())
author_runtime=[]
for name in ['pp_debug','pp_optimized','bitmap_first']:
 p=NUM.parent/'scratch/i50_first_publishing/repair03'/(name+'.json');j=json.loads(p.read_text());assert j['exit']==0 and j['reaped'] and j['source_stable']
 final=j['before_digest']==j['after_digest']==digest and j['delta_from_baseline']=={e['path']:e['sha256'] for e in fm['files']}
 if name!='bitmap_first':assert final
 author_runtime.append(dict(name=name,input=item(p,'author command source/exit/custody record'),matches_exact_final_source=final,
  disposition='final-candidate evidence' if final else 'historical first-focused command; earlier test bytes, no final-candidate credit'))

origins=[]
for e in fm['files']:origins.append(item(CODE/e['path'],'entire repair diff and local surrounding count/consumer context',CAND))
for name in ['RETURN.md','FINAL_SOURCE.json','SEAL.json','BULK_MANIFEST.json','compare.py','run_check.py']:
 origins.append(item(PACKET/name,'repair evidence and exact count rule',CAND))
for rel,scope in [('AGENTS.md','continued Root instructions'),('agents/AGENT_TASK.md','continued TASK role'),('projects/chirality-piping/AGENTS.md','continued project instructions'),('.agents/skills/software-code-review/SKILL.md','continued selected review skill'),(str(R/'BRIEFS/RV65_BITMAP_REPAIR_BACKCHECK.md'),'full new brief'),(str(R/'REVIEW_RV65/named_support_implementation_03/REVIEW.md'),'prior finding/functional/numerical limits continued'),(str(R/'REVIEW_RV65/named_support_implementation_03/SEAL.json'),'prior review seal verified unchanged')]:
 p=Path(rel);assert git(NUM,'show',RP+':'+str(p))==(NUM/p).read_bytes();origins.append(item(NUM/p,scope,RP))
answer=dict(candidate=CAND,base=BASE,records_pin=RP,scope_paths=2,source_clean=True,three_consumers_use_owned_loop=True,
 expected_prefix_derivation='Reservation: AllocationRequest1, capacity MapWrite1, actual reserve LibraryBoundary1, RustCapacityBytes=observed capacity. Fill: one shape ValidationEntry, one checked MapWrite before each push, no resize/bulk call. Prior fault returns before any event.',
 prefixes=prefixes,fresh_debug_tests=19,author_optimized_tests=19,optimized_rerun_by_reviewer=False,
 semantic_checks=checks,author_runtime=author_runtime,prior194_rows_and135_PASS_reference_transfers_by_exact_semantic_equality=True,
 preservation=dict(repair_packet_files=8,new_author_bulk_files=15,unchanged_baseline_files=955,original_packet_files=10,original_author_bulk_files=73,prior_reviewer_seal_unchanged=True),origins=origins)
(OUT/'CHECKS.json').write_text(json.dumps(answer,indent=2)+'\n')
print(json.dumps(dict(prefixes_verified=len(prefixes),fresh_debug=19,comparisons=[x['label'] for x in checks],old_records_each=104,named_fields_unchanged=True,all_preservation_verified=True)))

from pathlib import Path
import hashlib,json,os,re,subprocess
NUM=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/numerics')
CODE=NUM.parent/'f2a';R=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
OUT=Path(__file__).resolve().parent;BULK=NUM.parent/'scratch/rv65_named_support/implementation03'
PACKET=CODE/R/'I50/first_publishing_component_02';CAND='8104a4fedd0fd575f20b3d723cc0cdd722d73ba1';BASE='d0daa18717f8243a7232e898c9ef9b4f4d18d9e4';RP='59a85f6165b7187d743745bd4ddaed0100cf0a60'
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
sha=lambda b:hashlib.sha256(b).hexdigest()
def git(root,*args):return subprocess.check_output(['git',*args],cwd=str(root),env=env)
def manifest_entry(p,scope,revision=None):
 b=p.read_bytes();return dict(location=str(p),bytes=len(b),sha256=sha(b),scope_read=scope,revision=revision)
assert git(CODE,'rev-parse','HEAD').decode().strip()==CAND
assert not git(CODE,'status','--porcelain').strip()
seal=json.loads((PACKET/'SEAL.json').read_text())
for e in seal['files']:
 b=(PACKET/e['path']).read_bytes();assert len(b)==e['bytes'] and sha(b)==e['sha256']
bm=json.loads((PACKET/'BULK_MANIFEST.json').read_text())
for e in bm['files']:
 b=Path(e['location']).read_bytes();assert len(b)==e['bytes'] and sha(b)==e['sha256']
fm=json.loads((PACKET/'FINAL_SOURCE.json').read_text())
changed={e['path'] for e in fm['files']}
diff=git(CODE,'diff','--name-only',BASE,CAND,'--','projects/chirality-piping/core','projects/chirality-piping/fixtures').decode().splitlines()
assert set(diff)==changed and len(changed)==7
for e in fm['files']:
 b=(CODE/e['path']).read_bytes();assert len(b)==e['bytes'] and sha(b)==e['sha256'] and b==git(CODE,'show',CAND+':'+e['path'])
base_inventory_path=CODE.parent/'scratch/i50_first_publishing/runtime02/BASELINE.json'
baseline=json.loads(base_inventory_path.read_text());unchanged=[p for p in baseline if p not in changed]
assert len(unchanged)==950 and all(sha((CODE/p).read_bytes())==baseline[p] for p in unchanged)
lib='projects/chirality-piping/core/product_physics/src/lib.rs'
hook=b'    if let Some(observer) = product.as_deref_mut() {\n        observer.solver_observations(load_case, solver_mode, &results);\n    }\n'
current=(CODE/lib).read_bytes();assert current.count(hook)==1 and current.replace(hook,b'')==git(CODE,'show',BASE+':'+lib)
test='projects/chirality-piping/core/product_physics/tests/formation_check_runtime.rs'
old=git(CODE,'show',BASE+':'+test)
pattern=rb'const RF_SKEW_T_CANT_OFF_122_R1E_04: &str = r#"(.*?)"#;'
match=re.search(pattern,old);assert match
fixture=CODE/'projects/chirality-piping/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json'
assert match[1]==fixture.read_bytes() and len(match[1])==2747
replacement=b'const RF_SKEW_T_CANT_OFF_122_R1E_04: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");'
assert old[:match.start()]+replacement+old[match.end():]==(CODE/test).read_bytes()

# Independently compare six old semantic capture cases; new adapter/capacity work is separate.
old_log=CODE/R/'I47/selected_material_02/_run_records/pp_debug.log'
common=['REQUEST','CASE','INPUT','ROWS','VERDICTS','PASS','FULL_CASE','G5A','OPERATIONAL','G5A_WORK','TERM_MAP','G5A_DATA','NATIVE_ROWS','NATIVE_IDENTITY']
tags=['I45_'+s for s in common]+['I47_'+s for s in common+['BOUNDARY','SELECTION','BASIS_CAPTURE','ANCILLARY','CAPTURE']]
def parse(p):
 data={k:[] for k in tags}
 for line in p.read_text().splitlines():
  tag,_,v=line.partition(' ')
  if tag not in data:continue
  try:v=json.loads(v)
  except json.JSONDecodeError:pass
  data[tag].append(v)
 return data
old_data=parse(old_log);compat=[]
for profile in ['pp_debug','pp_release']:
 data=parse(BULK/(profile+'.log'));count=0
 for tag in tags:
  assert len(old_data[tag])==(2 if tag.startswith('I45_') else 4)
  if tag=='I47_CAPTURE':
   extension=', ("supports", 1), ("spring_map", 0), ("support_fixed", 1)'
   assert all(s.count(extension)==1 for s in data[tag]);data[tag]=[s.replace(extension,'') for s in data[tag]]
  assert data[tag]==old_data[tag],(profile,tag)
  count+=len(data[tag])
 assert count==104;compat.append(dict(profile=profile,unchanged_semantic_records=count,cases=6))

origins=[]
for e in fm['files']:origins.append(manifest_entry(CODE/e['path'],'entire seven-path frozen diff and relevant context',CAND))
for name in ['RETURN.md','FINAL_SOURCE.json','SEAL.json','BULK_MANIFEST.json','run_check.py','compatibility_check.py','named_oracle.py','EXECUTION.json']:
 origins.append(manifest_entry(PACKET/name,'author evidence/source reviewed; numerical oracle read after own statics/checker was implemented',CAND))
for rel,scope in [
 ('AGENTS.md','continued active Root instructions'),('agents/AGENT_TASK.md','continued active TASK role'),('projects/chirality-piping/AGENTS.md','continued project instructions'),
 ('.agents/skills/software-code-review/SKILL.md','continued selected skill'),
 (str(R/'BRIEFS/RV65_NAMED_SUPPORT_CODE_REVIEW.md'),'full active review brief'),
 (str(R/'REVIEW_RV65/named_support_component_01/REVIEW.md'),'prior sealed RV65-1/2 conditions, continued context'),
 (str(R/'REVIEW_RV65/dense_observation_02/REVIEW.md'),'prior sealed exact dense custody contract, continued context'),
 (str(R/'REVIEW_RV66/named_torsion_01/RETURN.md'),'full accepted bounded mathematical return'),
 (str(R/'REVIEW_RV66/named_torsion_01/REVIEW.md'),'basis/statics/fixed-scale scope, lines1-100')]:
  p=Path(rel);b=git(NUM,'show',RP+':'+str(p));assert b==(NUM/p).read_bytes()
  origins.append(manifest_entry(NUM/p,scope,RP))
for rel,scope in [
 ('solver/linear_supports/src/lib.rs','prepare_rigid_support/add_restrained_dof unique producer warrant'),
 ('solver/frame_kernel/src/structural/retained/source.rs','canonical source encoding/count and primitive contracts'),
 ('solver/frame_kernel/src/structural/retained/ledger.rs','canonical exact-net encoding'),
 ('solver/frame_kernel/src/structural/retained/adaptive.rs','named floor/allowance constants')]:
 p=Path('projects/chirality-piping/core')/rel;b=git(CODE,'show',CAND+':'+str(p));assert b==(CODE/p).read_bytes()
 origins.append(manifest_entry(CODE/p,scope,CAND))
origins.append(manifest_entry(old_log,'old accepted six-case semantic comparison basis',BASE))
results=[]
for p in sorted(BULK.glob('*.json')):
 if p.stem not in ['pp_debug','fk_certificate','formation_named','pp_s11','fk_s11','pp_release']:continue
 j=json.loads(p.read_text());assert j['exit']==0 and j['source_stable'] and j['reaped']
 log=p.with_suffix('.log');summary=[s for s in log.read_text().splitlines() if s.startswith('test result:')]
 results.append(dict(name=p.stem,result=summary[-1],record=str(p),log=str(log)))
assert len(results)==6
check=dict(candidate=CAND,base=BASE,records_pin=RP,source_paths=7,packet_files=10,external_author_files=len(bm['files']),unchanged_baseline_files=950,
 scope_verified=True,exact_three_line_hook=True,exact_fixture_and_only_literal_replacement=True,fixture_sha256=sha(fixture.read_bytes()),
 code_clean=True,compatibility=compat,runtime=results,origins=origins,
 author_bulk_manifest=manifest_entry(PACKET/'BULK_MANIFEST.json','all73 hashes/sizes verified'),
 author_seal=manifest_entry(PACKET/'SEAL.json','all9 sealed payloads verified'))
(OUT/'CHECKS.json').write_text(json.dumps(check,indent=2)+'\n')
print(json.dumps({k:check[k] for k in ['source_paths','packet_files','external_author_files','unchanged_baseline_files','scope_verified','exact_three_line_hook','exact_fixture_and_only_literal_replacement','code_clean','compatibility']},indent=2))

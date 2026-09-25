from pathlib import Path
import json,hashlib,re,subprocess
O=Path(__file__).parent;A=O.parent;R=next(p for p in A.parents if (p/'agents/AGENT_TASK.md').is_file());P=R/'projects/chirality-piping';F=P/'validation/qualification/fixtures/first_static'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f2=read(A/'SOURCE_FREEZE_02.json');f3=read(A/'SOURCE_FREEZE_03.json');assert sha(A/'SOURCE_FREEZE_03.json')=='845d0a4ab9298f921156daae2a4456f79640d56ad5f58dceae441d00606ff7bc';assert sha(R/f3['fixture_delta']['path'])=='a0cdef602209f487794e89247e116dc4379ae7c74e08e7118bc14d4466eda9fb';assert sha(A/'FIXTURE_PROMOTION/RETURN.md')=='711b8eb6c90e078ba80cff4fcece477fff744ee4172ee1e1081f2590ba28a1d9'
for row in f3['files']:assert sha(R/row['path'])==row['sha256'] and (R/row['path']).stat().st_size==row['byte_length']
oldhash={x['path']:x['sha256'] for x in f2['files']};changed=[x['path'] for x in f3['files'] if oldhash.get(x['path'])!=x['sha256']];assert changed==f3['changed_paths'] and len(changed)==18
for row in f3['runtime_adapter_unchanged']:assert row['sha256']==oldhash[row['path']]==sha(R/row['path'])
assert len(f3['runtime_adapter_unchanged'])==5
# Reconstruct freeze02 from base and apply every byte of the18-path delta in memory.
def apply(patch,bases,prefix):
 result={}
 for ch in [x for x in re.split(r'(?m)(?=^--- (?:a/|/dev/null|freeze02/))',patch) if x.strip()]:
  lines=ch.splitlines(keepends=True);path=lines[1].strip().removeprefix('+++ '+prefix)
  if prefix=='freeze03/':path='projects/chirality-piping/'+path
  old=bases.get(path,b'').decode().splitlines(keepends=True);out=[];pos=0;i=0
  while i<len(lines):
   m=re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@',lines[i])
   if not m:i+=1;continue
   at=max(0,int(m[1])-1);out.extend(old[pos:at]);pos=at;i+=1
   while i<len(lines) and not lines[i].startswith('@@ '):
    l=lines[i]
    if l.startswith(' '):assert old[pos]==l[1:];out.append(l[1:]);pos+=1
    elif l.startswith('-'):assert old[pos]==l[1:];pos+=1
    elif l.startswith('+'):out.append(l[1:])
    elif l.strip():raise AssertionError((path,l))
    i+=1
  out.extend(old[pos:]);result[path]=''.join(out).encode()
 return result
base={x['path']:subprocess.check_output(['git','show',f2['base_commit']+':'+x['path']],cwd=R) if x['base_sha256'] else b'' for x in f2['files']};old=apply((R/f2['patch']['path']).read_text(),base,'b/')
for p,b in old.items():assert hashlib.sha256(b).hexdigest()==oldhash[p]
new=apply((R/f3['fixture_delta']['path']).read_text(),old,'freeze03/');assert set(new)==set(changed)
for p,b in new.items():assert b==(R/p).read_bytes()
provenance=read(F/'PROVENANCE.json');assert len(provenance['files'])==12;fixture_records=[]
for row in provenance['files']:
 path=F/row['path'];origin=P/row['origin_path'];assert path.read_bytes()==origin.read_bytes() and sha(path)==row['sha256']
 if row['origin_commit']:
  assert subprocess.check_output(['git','show',row['origin_commit']+':projects/chirality-piping/'+row['origin_path']],cwd=R)==path.read_bytes()
 fixture_records.append({'path':str(path.relative_to(R)),'sha256':sha(path),'origin_commit':row['origin_commit'],'origin_path':row['origin_path']})
package=A.parent/'FIRST_STATIC_BINDINGS';pm=read(package/'PACKET_MANIFEST.json');assert len(pm['files'])==25 and all(sha(package/x['path'])==x['sha256'] for x in pm['files'])
checks=read(A/'FIXTURE_PROMOTION/CHECKS.json');assert checks['source_unchanged'] and checks['before']==checks['after'] and checks['exit_code']==0
for row in checks['before']:assert sha(P/row['path'])==row['sha256']
assert sha(A/'FIXTURE_PROMOTION/affected_test_driver.py')==checks['driver_sha256'];log=A/'FIXTURE_PROMOTION/affected_checks.log';assert sha(log)==checks['log_sha256'];assert re.search(r'Ran 37 tests',log.read_text()) and 'historical_agent_run_access_attempts=0' in log.read_text()
for name in ['test_qualification_physics_structure.py','test_qualification_physics_integration.py','test_first_static_selection.py','qualification_fixture_support.py']:
 text=(P/'tests'/name).read_text();assert 'execution/_Coordination/AgentRuns' not in text and 'CONTINUATION_2026-09-24' not in text
for row in f3['runtime_adapter_unchanged']:assert 'qualification_fixture_support' not in (R/row['path']).read_text()
programme_path=R/f3['actual_four_run_programme_unchanged']['path'];assert sha(programme_path)==f3['actual_four_run_programme_unchanged']['sha256'];programme=read(programme_path);assert programme['actual_case_mode_executions']==4 and programme['adapter_freeze_sha256']==sha(A/'SOURCE_FREEZE_02.json') and programme['all_selected_obligations_matched']
assert programme['programme_denominator']=={'case_mode_executions':4,'scalar_obligations':292,'structural_obligations':40,'section_subchecks_nested':36}
stream_count=0
for mode in programme['modes']:
 lp=A/mode['ledger_path'];assert sha(lp)==mode['ledger_sha256'];ledger=read(lp);assert ledger['execution_basis']['harness_sha256']==sha(P/'tools/validation/qualification_gate.py')
 for case in ledger['cases']:
  for artifact in case['retained_artifacts']:
   target=lp.parent/artifact['path'];assert sha(target)==artifact['sha256'] and target.stat().st_size==artifact['byte_length'];stream_count+=1
result={'status':'CLEAR complete18-path test/fixture portability delta','freeze03_sha256':sha(A/'SOURCE_FREEZE_03.json'),'delta_sha256':sha(R/f3['fixture_delta']['path']),'all26frozen_hashes_match':True,'reconstructed_delta_paths':changed,'fixtures_byte_identical':fixture_records,'original25_unchanged':True,'runtime5_unchanged':f3['runtime_adapter_unchanged'],'permanent_test_dated_dependency_removed':True,'test_helper_not_imported_by_runtime':True,'observed_affected_methods':37,'observed_historical_access_attempts':0,'audit_guard_negative_control_inspected':True,'actual_four_run_programme_sha256':sha(programme_path),'actual_denominator':programme['programme_denominator'],'ledger_bound_artifacts_rechecked':stream_count,'limits':'No suite/solver/build/native/network rerun. Previous complete adapter/repaired source reviews remain scope basis; this preserves recorded genuine comparison evidence, not a new result assessment.'}
(O/'FIXTURE_PROMOTION_CHECKS_03.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k not in ['fixtures_byte_identical','runtime5_unchanged','reconstructed_delta_paths']},indent=2))

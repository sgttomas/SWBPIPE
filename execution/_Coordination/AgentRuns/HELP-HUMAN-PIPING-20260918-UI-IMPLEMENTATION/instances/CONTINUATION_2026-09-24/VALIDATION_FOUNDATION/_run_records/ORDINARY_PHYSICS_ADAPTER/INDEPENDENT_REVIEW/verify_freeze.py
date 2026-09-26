from pathlib import Path
import json,hashlib,re,subprocess
O=Path(__file__).parent;D=O.parent;ROOT=next(p for p in D.parents if (p/'agents/AGENT_TASK.md').is_file());PROJECT=ROOT/'projects/chirality-piping'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads(p.read_text())
freeze=read(D/'SOURCE_FREEZE_01.json');assert sha(D/'SOURCE_FREEZE_01.json')=='aa3a8e43eeb5a2ccdce909811c71219eabdb850382d6d12ac25c04b35c0dbeb0';patch=ROOT/freeze['patch']['path'];assert sha(patch)==freeze['patch']['sha256']=='71525315db46f1f2a11b8c62004a0865d1ef5c5f3c1bbfedf4870f44eb982645'
coverage=[]
for r in freeze['files']:
 p=ROOT/r['path'];assert sha(p)==r['sha256'] and p.stat().st_size==r['byte_length']
 if r['base_sha256'] is not None:
  b=subprocess.check_output(['git','show',freeze['base_commit']+':'+r['path']],cwd=ROOT);assert hashlib.sha256(b).hexdigest()==r['base_sha256']
 coverage.append(r)
assert len(coverage)==11
for r in freeze['evidence']+freeze['unchanged_dependencies']:assert sha(ROOT/r['path'])==r['sha256'] and (ROOT/r['path']).stat().st_size==r['byte_length']
# Patch reconstruction without applying/staging files.
seen=[]
for chunk in [x for x in re.split(r'(?m)(?=^--- (?:a/|/dev/null))', patch.read_text()) if x.strip()]:
 lines=chunk.splitlines(keepends=True);path=lines[1].strip().removeprefix('+++ b/');r=next(x for x in coverage if x['path']==path);old=[] if r['base_sha256'] is None else subprocess.check_output(['git','show',freeze['base_commit']+':'+path],cwd=ROOT).decode().splitlines(keepends=True);out=[];pos=0;i=0
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
 out.extend(old[pos:]);assert hashlib.sha256(''.join(out).encode()).hexdigest()==r['sha256'];seen.append(path)
assert set(seen)=={x['path'] for x in coverage}
focused=read(D/'FOCUSED_CHECKS_01.json');assert focused['source_unchanged'] and focused['before']==focused['after']
for r in focused['before']:assert sha(PROJECT/r['path'])==r['sha256']
counts=[]
for r in focused['checks']:
 log=D/r['log'];assert sha(log)==r['log_sha256'] and r['exit_code']==0;count=int(re.search(r'Ran (\d+) tests?',log.read_text())[1]);counts.append(count)
assert counts==[70,3]
binding=read(D/'REVIEWED_READER_BINDING.json');source=Path('/private/tmp/piping-engine-integration-20260925/projects/chirality-piping')
for r in binding['files']+binding['review_basis']:assert sha(source/r['path'])==r['sha256']
selections=read(D/'EXECUTION_SELECTIONS/MANIFEST.json');scalar=structural=nested=0;selection_rows=[]
for r in selections['selections']:
 path=D/r['selection'];assert sha(path)==r['sha256'];value=read(path);assert value['runner']['solver_mode']==r['mode'] and len(value['cases'])==2
 for case in value['cases']:
  scalar+=len(case['assertions']);structural+=10;nested+=9
  for role in ['input','reference','criterion']:
   b=case[role];assert sha((path.parent/b['path']).resolve())==b['sha256']
  for b in case['structural'].values():assert sha((path.parent/b['path']).resolve())==b['sha256']
 selection_rows.append({'mode':r['mode'],'sha256':r['sha256'],'scalar':146,'structural':20,'nested':18,'runner':value['runner']})
assert (scalar,structural,nested)==(292,40,36)
build=read(D/'HEADLESS_BUILD/BUILD_RESULT.json');assert build['exit_code']==0 and build['source_unchanged'] and not build['solver_execution_performed'];assert build['candidate_commit']=='8b982aa7ce64afe37e6067d1038d92608f4aaf3f';exe=build['executable'];assert sha(Path(exe['path']))==exe['sha256']=='506866a9beefcb4ec1374d1fd7b8f2f999f17670cb7c65b2b17ef91a92162614'
assert sha(D/'HEADLESS_BUILD/SOURCE_BEFORE.json')==build['source_before_sha256']==sha(D/'HEADLESS_BUILD/SOURCE_AFTER.json');inputs=read(D/'HEADLESS_BUILD/SOURCE_BEFORE.json');assert len(inputs)==1347
for r in inputs:assert sha(Path(r['path']))==r['sha256'] and Path(r['path']).stat().st_size==r['byte_length']
for name,key in [('cargo.stderr.log','stderr_sha256'),('cargo.stdout.jsonl','stdout_sha256'),('metadata.json','metadata_sha256'),('runner.dep-info.txt',None)]:
 assert sha(D/'HEADLESS_BUILD'/name)==(build[key] if key else build['dep_info']['sha256'])
cross=read(D/'HEADLESS_BUILD/DEPENDENCY_CROSSCHECK.json');assert cross['dep_info_dependencies']==74 and cross['all_dep_info_inputs_bound_before_build']
assert len(cross['registry_package_checksums'])==22
for r in cross['registry_package_checksums']:assert r['matches'] and sha(Path(r['archive_path']))==r['package_checksum']
received=read(D/'RECEIVED_READER_CHECK_01/RESULT.json');assert len(received['observations'])==2
for r in received['observations']:assert sha(Path(r['original_stdout_path']))==r['original_stdout_sha256'] and r['observation']['process']['outcome']=='completed' and r['observation']['response']['verdict']=='consistent'
result={'status':'Complete frozen-source/evidence comparison; two code findings retained separately','base':freeze['base_commit'],'files':coverage,'patch_reconstructed_paths':len(seen),'evidence_and_unchanged_dependency_hashes_match':True,'owner_focused_methods':counts,'reviewed_reader_files':binding['files'],'review_basis_hashes_match':True,'selections':selection_rows,'denominator':{'scalar':scalar,'structural':structural,'nested':nested,'actual_first_static_runs':0},'build':{'candidate':build['candidate_commit'],'binary_sha256':exe['sha256'],'before_after_inputs_verified':len(inputs),'dep_info_inputs_recorded':74,'registry_archives_verified':22,'no_solver_run':True},'historical_reader_observations':2,'limits':'Record parsing/hashing only; no repeated owner suites, solver/build/native execution. Supplied dep-info closure and source review are reused; no independent receipt implementation review claimed.'}
(O/'FREEZE_AND_EVIDENCE_CHECK.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k not in ['files','selections','reviewed_reader_files']},indent=2))

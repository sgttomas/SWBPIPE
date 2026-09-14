import pathlib, subprocess, json, hashlib, os, datetime, shutil, tomllib, difflib
P=pathlib.Path('/Users/ryan/.codex/worktrees/8728/chirality-results-integrity-20260913/projects/chirality-piping')
I=pathlib.Path('/Users/ryan/.codex/worktrees/8728/chirality-results-engineering-3d-20260913/projects/chirality-piping')
E=pathlib.Path(__file__).parent
LOCKS=['core/reporting/report_package/Cargo.lock','validation/benchmarks/mechanics/Cargo.lock','validation/benchmarks/stress/Cargo.lock']
def sha(b): return hashlib.sha256(b).hexdigest()
def save(n,x): (E/n).write_text(json.dumps(x,indent=2)+'\n')
def snapshot(paths): return {p:sha((P/p).read_bytes()) if (P/p).is_file() else None for p in paths}
brief=E/'TASK_WRITER_CONTINUATION_V1.md'; origin=pathlib.Path('/tmp/chirality-results-engineering-3d-20260913/LOCKFILE_REPAIR_BRIEF_V1.md')
assert sha(brief.read_bytes())=='84260ae8503337ccdf7aab6aa4f5100871272a18f8849110ab23787af03af258'
assert sha(origin.read_bytes())=='25210ee938801b71152bc5a5a1bb0dcc9226eee0cbcd6c0c132bdebf1d948a67'
(E/'ROOT_ORIGIN_BRIEF_V1.md').write_bytes(origin.read_bytes())
prior=P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260913-RESULTS-ENGINEERING-3D/instances/RESULTS/_run_records/TASK_WRITER_IMPLEMENTATION_V1/MANIFEST.json'
priorbytes=prior.read_bytes(); assert sha(priorbytes)=='0626b60855ee515819646f4ea85ce9df4b5a2232db38f6c29b844318da693d27'
old=json.loads(priorbytes)
# Read-only Git inventory and original tracked bytes; no Git mutation.
r=subprocess.run(['git','ls-files','-z'],cwd=P,capture_output=True,check=True)
(E/'READ_ONLY_GIT_LS_FILES.stdout').write_bytes(r.stdout)
paths=[x for x in r.stdout.decode().split('\0') if x]+[x['path'] for x in old['product_source_files']]
paths=sorted(set(paths)); before=snapshot(paths); save('TRACKED_AND_ACCEPTED_SOURCE_BEFORE_V1.json',before)
for x in old['product_source_files']: assert before[x['path']]==x['sha256'],x['path']
immutable_before={x['path']:sha((prior.parent/x['path']).read_bytes()) for x in old['files']}
assert all(immutable_before[x['path']]==x['sha256'] for x in old['files'])
records=[]
for idx,lock in enumerate(LOCKS,1):
 assert lock in paths
 d=E/('lock_'+str(idx)); d.mkdir(exist_ok=True)
 for tag,data in [('lane_before',(P/lock).read_bytes()),('integration_before',(I/lock).read_bytes())]: (d/(tag+'.Cargo.lock')).write_bytes(data)
 rr=subprocess.run(['git','show','HEAD:./'+lock],cwd=P,capture_output=True)
 save(str(d.relative_to(E))+'/READ_ONLY_GIT_SHOW_COMMAND.json',{'argv':['git','show','HEAD:./'+lock],'cwd':str(P),'exit_code':rr.returncode,'stderr':rr.stderr.decode()})
 assert rr.returncode==0,rr.stderr
 (d/'tracked_head_original.Cargo.lock').write_bytes(rr.stdout)
 records.append({'path':lock,'snapshots':{tag:sha((d/(tag+'.Cargo.lock')).read_bytes()) for tag in ['lane_before','integration_before','tracked_head_original']}})
save('ORIGINAL_LOCK_SNAPSHOTS_V1.json',records)
env=os.environ.copy(); env['CARGO_NET_OFFLINE']='true'; env['CARGO_TARGET_DIR']='/tmp/chirality-results-engineering-3d-20260913/results-lockfile-metadata-target'
ledger=[]
for num,(locked,lock) in enumerate([(False,x) for x in LOCKS]+[(True,x) for x in LOCKS],1):
 d=E/('command_%03d'%num); d.mkdir()
 argv=['cargo','metadata']+(['--locked'] if locked else [])+['--offline','--format-version','1','--manifest-path',str(P/lock.replace('Cargo.lock','Cargo.toml'))]
 cmd={'argv':argv,'cwd':str(P),'env_overrides':{'CARGO_NET_OFFLINE':'true','CARGO_TARGET_DIR':env['CARGO_TARGET_DIR']},'source_hashes_before':snapshot(paths),'driver_sha256':sha(pathlib.Path(__file__).read_bytes()),'prepared_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
 (d/'COMMAND.json').write_text(json.dumps(cmd,indent=2)+'\n')
 with (d/'stdout.json').open('wb') as out,(d/'stderr.txt').open('wb') as err: rr=subprocess.run(argv,cwd=P,env=env,stdout=out,stderr=err)
 result={'exit_code':rr.returncode,'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'stdout_sha256':sha((d/'stdout.json').read_bytes()),'stderr_sha256':sha((d/'stderr.txt').read_bytes())}
 (d/'RESULT.json').write_text(json.dumps(result,indent=2)+'\n'); ledger.append({'command':str(d.relative_to(E))+'/COMMAND.json',**result}); save('COMMAND_LEDGER_V1.json',ledger)
 print('command',num,'exit',rr.returncode,flush=True)
 after=snapshot(paths); changed=[x for x in paths if before[x]!=after[x]]
 assert set(changed)<=set(LOCKS),changed
 if rr.returncode: raise SystemExit(rr.returncode)
 for k in LOCKS:
  od=tomllib.loads((E/('lock_'+str(LOCKS.index(k)+1))/'lane_before.Cargo.lock').read_text())
  nd=tomllib.loads((P/k).read_text())
  oldpk={(x['name'],x['version'],x.get('source')) for x in od['package']}; newpk={(x['name'],x['version'],x.get('source')) for x in nd['package']}
  assert oldpk<=newpk, {'removed_or_upgraded_packages':list(oldpk-newpk)}
after=snapshot(paths); save('TRACKED_AND_ACCEPTED_SOURCE_AFTER_V1.json',after)
summary=[]; patch=''
for idx,k in enumerate(LOCKS,1):
 d=E/('lock_'+str(idx)); data=(P/k).read_bytes(); (d/'lane_after.Cargo.lock').write_bytes(data)
 oldtext=(d/'integration_before.Cargo.lock').read_text(); newtext=data.decode()
 dp=''.join(difflib.unified_diff(oldtext.splitlines(True),newtext.splitlines(True),fromfile='a/'+k,tofile='b/'+k)); (d/'integration_delta.patch').write_text(dp); patch+=dp
 laneold=tomllib.loads((d/'lane_before.Cargo.lock').read_text()); newer=tomllib.loads(newtext)
 op={(x['name'],x['version'],x.get('source')) for x in laneold['package']}; np={(x['name'],x['version'],x.get('source')) for x in newer['package']}
 summary.append({'path':k,'new_sha256':sha(data),'bytes':len(data),'lane_before_sha256':records[idx-1]['snapshots']['lane_before'],'integration_before_sha256':records[idx-1]['snapshots']['integration_before'],'existing_package_versions_preserved':op<=np,'added_packages':sorted(np-op,key=str),'removed_packages':sorted(op-np,key=str)})
(E/'LOCKFILE_TRANSFER_V1.patch').write_text(patch)
immutable_after={x:sha((prior.parent/x).read_bytes()) for x in immutable_before}
assert immutable_after==immutable_before; assert sha(prior.read_bytes())==sha(priorbytes)
save('FINAL_SOURCE_AUDIT_V1.json',{'changed_paths_from_pre_effect':[x for x in paths if before[x]!=after[x]],'tracked_inventory_count':len(paths),'accepted36_unchanged':True,'prior875_evidence_files_unchanged':True,'prior_manifest_unchanged':True,'locks':summary,'tracked_vs_ignored_correction':'These three Cargo.lock files are TRACKED. Historical generated/ignored inventory descriptions must not be generalized to these tracked locks; lane-before/integration-before/HEAD-original bytes are separately preserved.','runtime_complete':True})
print('RUNTIME_COMPLETE',flush=True)

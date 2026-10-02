from pathlib import Path
import sys,os,json,subprocess,hashlib,difflib,re,datetime
wt=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3'); repo=wt/'k6c'; p=Path('projects/chirality-piping'); r=p/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30'; h=p/'core/solver/performance_harness'; raw=repo/r/'I26/kernel_mutants_02/_run_records'; archive=wt/'scratch/i26-kernel-mutants-02/archive'; target=wt/'k6c-i26-kernel-mutants02-target'
sha=lambda b:hashlib.sha256(b).hexdigest(); now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
assert datetime.datetime.now(datetime.timezone.utc)<datetime.datetime.fromisoformat('2026-10-02T03:57:11+00:00')
plan=json.loads((raw/'VARIANT_PLAN.json').read_text()); binding=json.loads((raw/'INPUT_BINDING.json').read_text()); variant=sys.argv[1]; ids=[x['id'] for x in plan['ordered_variants']]; index=ids.index(variant); selected=plan['ordered_variants'][index]
if index:
    previous=json.loads((raw/ids[index-1]/'RESULT.json').read_text())
    assert previous['all_compiled'] and previous['source_restored']
    assert previous['verdict']==('NORMAL_PASS' if index==1 else 'COMPILED_TEST_FAILURE'),previous['verdict']
    control=json.loads((raw/'NONE/RESULT.json').read_text())
out=raw/variant; out.mkdir()
source=Path(plan['source_path']); normal=(archive/source).read_bytes(); assert sha(normal)==plan['normal_source_sha256']
for path,digest in binding['archive_file_hashes'].items():assert sha((archive/path).read_bytes())==digest,path
active=normal if selected['old'] is None else normal.decode().replace(selected['old'],selected['new']).encode()
patch=''.join(difflib.unified_diff(normal.decode().splitlines(keepends=True),active.decode().splitlines(keepends=True),fromfile='a/'+str(source),tofile='b/'+str(source)))
(out/(variant+'.diff')).write_text(patch)
env_changes={'RUSTUP_TOOLCHAIN':'1.97.1','RUSTUP_AUTO_INSTALL':'0','CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','RUST_TEST_THREADS':'2','CARGO_TARGET_DIR':str(target),'GIT_OPTIONAL_LOCKS':'0'}; env={**os.environ,**env_changes}
record={'variant':variant,'candidate':binding['candidate'],'start_utc':now(),'source_path':str(source),'normal_source_sha256':sha(normal),'active_source_sha256':sha(active),'patch_sha256':sha(patch.encode()),'test_hashes':{path:binding['archive_file_hashes'][path] for path in [str(h/'tests/k6c_envelope.rs'),str(h/'tests/k6b_w1.rs')]},'runs':[]}
if variant=='NONE':
    (raw/'ENVIRONMENT.json').write_text(json.dumps({'overrides':env_changes,'uname':subprocess.check_output(['uname','-a'],text=True),'rustc':subprocess.check_output(['/Users/ryan/.cargo/bin/rustup','run','1.97.1','rustc','-Vv'],env=env,text=True),'cargo':subprocess.check_output(['/Users/ryan/.cargo/bin/rustup','run','1.97.1','cargo','-V'],env=env,text=True)},indent=2)+'\n')
(archive/source).write_bytes(active)
try:
    for selection in plan['fixed_tests']:
        guard=subprocess.run(['ps','-p','5387','-o','pid,ppid,command'],text=True,capture_output=True,check=True); assert '/guard/memguard.sh' in guard.stdout
        argv=['/Users/ryan/.cargo/bin/rustup','run','1.97.1','cargo','test','--manifest-path',str(h/'Cargo.toml'),'--release','--locked','--offline','-j','4']+selection['args']
        run={'selection':selection['selection'],'argv':argv,'cwd':str(archive),'environment_overrides':env_changes,'guard':guard.stdout,'start_utc':now()}
        logfile=out/(selection['selection']+'.log')
        with logfile.open('w') as log:process=subprocess.run(argv,cwd=archive,env=env,stdout=log,stderr=subprocess.STDOUT)
        log=logfile.read_text(); match=re.search(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed;',log)
        run.update(returncode=process.returncode,end_utc=now(),log_sha256=sha(log.encode()),compile_succeeded='Finished `release`' in log and match is not None,passed=int(match[1]) if match else None,failed=int(match[2]) if match else None,assertion_excerpt=log[log.index('failures:\n'):log.rindex('test result:')] if 'failures:\n' in log else '',source_unchanged=(archive/source).read_bytes()==active,tests_unchanged=all(sha((archive/path).read_bytes())==digest for path,digest in record['test_hashes'].items()))
        stem='k6c_envelope' if selection['selection']=='kernel_reference_suite' else 'k6b_w1'
        run['test_binaries']=[{'path':str(x),'sha256':sha(x.read_bytes())} for x in sorted((target/'release/deps').glob(stem+'-*')) if x.is_file() and os.access(x,os.X_OK)]
        if index:run['binary_differs_from_NONE']=run['test_binaries']!=control['runs'][len(record['runs'])]['test_binaries']
        record['runs'].append(run)
        if not run['compile_succeeded'] or process.returncode not in (0,101) or (variant=='NONE' and process.returncode!=0):break
    record['all_compiled']=len(record['runs'])==2 and all(x['compile_succeeded'] for x in record['runs'])
    if not record['all_compiled']:record['verdict']='STOP_COMPILE_OR_EXECUTION_FAILURE'
    elif variant=='NONE':record['verdict']='NORMAL_PASS' if all(x['returncode']==0 for x in record['runs']) else 'STOP_NORMAL_FAILURE'
    elif all(x['returncode']==0 for x in record['runs']):record['verdict']='SURVIVED_STOP'
    else:record['verdict']='COMPILED_TEST_FAILURE'
finally:
    (archive/source).write_bytes(normal)
    record['source_restored']=(archive/source).read_bytes()==normal
    record['end_utc']=now()
    (out/'RESULT.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({'variant':variant,'verdict':record.get('verdict'),'source_restored':record['source_restored'],'runs':[{k:x[k] for k in ('selection','returncode','compile_succeeded','passed','failed','assertion_excerpt')} for x in record['runs']]},indent=2))

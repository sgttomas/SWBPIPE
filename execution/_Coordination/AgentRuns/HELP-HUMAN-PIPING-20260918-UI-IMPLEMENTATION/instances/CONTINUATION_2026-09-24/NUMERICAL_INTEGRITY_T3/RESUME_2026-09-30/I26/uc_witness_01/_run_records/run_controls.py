from pathlib import Path
import os,sys,json,subprocess,hashlib,datetime,re,difflib
wt=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3')
repo=wt/'k6c'; p=Path('projects/chirality-piping'); r=p/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30'; h=p/'core/solver/performance_harness'
raw=repo/r/'I26/uc_witness_01/_run_records'; archive=wt/'scratch/i26-uc-witness/archive'; target=wt/'k6c-i26-uc-witness-target'
variant=sys.argv[1]; assert variant in ('NONE','M05_UC_C_OVERLAP')
out=raw/variant; out.mkdir()
sha=lambda b:hashlib.sha256(b).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
env_changes={'RUSTUP_TOOLCHAIN':'1.97.1','RUSTUP_AUTO_INSTALL':'0','CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','RUST_TEST_THREADS':'2','CARGO_TARGET_DIR':str(target),'GIT_OPTIONAL_LOCKS':'0'}
env={**os.environ,**env_changes}
source=h/'src/k6/w1/envelope.rs'; test=h/'tests/k6c_envelope.rs'
normal=(archive/source).read_bytes(); binding=json.loads((raw/'INPUT_BINDING.json').read_text())
assert sha(normal)==binding['archive_file_hashes'][str(source)]
assert (archive/test).read_bytes()==(raw/'TEST.after.rs').read_bytes()==(repo/test).read_bytes()
normal_text=normal.decode(); needle='(4 * f * w + 4 * f).max(2 * f * w + 2 * bc * w)'
assert normal_text.count(needle)==1
mutant=normal_text.replace(needle,'(3 * f * w + 4 * f).max(2 * f * w + 2 * bc * w)').encode()
patch=''.join(difflib.unified_diff(normal_text.splitlines(keepends=True),mutant.decode().splitlines(keepends=True),fromfile='a/'+str(source),tofile='b/'+str(source)))
assert patch.encode()==(raw/'M05_UC_C_OVERLAP.diff').read_bytes()
if variant!='NONE':
    control=json.loads((raw/'NONE/RESULT.json').read_text()); assert control['returncode']==0 and control['tests_passed']==10
    (archive/source).write_bytes(mutant)
active=(archive/source).read_bytes()
guard=subprocess.run(['ps','-p','5387','-o','pid,ppid,command'],text=True,capture_output=True,check=True)
assert '/guard/memguard.sh' in guard.stdout
argv=['/Users/ryan/.cargo/bin/rustup','run','1.97.1','cargo','test','--manifest-path',str(h/'Cargo.toml'),'--release','--locked','--offline','-j','4','--test','k6c_envelope','--','--test-threads=2']
record={'variant':variant,'start_utc':now(),'argv':argv,'cwd':str(archive),'environment_overrides':env_changes,'guard_check':guard.stdout,'candidate':binding['candidate'],'normal_source_sha256':sha(normal),'active_source_sha256':sha(active),'test_sha256':sha((archive/test).read_bytes()),'patch_sha256':sha((raw/(variant+'.diff')).read_bytes())}
(raw/'ENVIRONMENT.json').write_text(json.dumps({'overrides':env_changes,'uname':subprocess.check_output(['uname','-a'],text=True),'rustc':subprocess.check_output(['/Users/ryan/.cargo/bin/rustup','run','1.97.1','rustc','-Vv'],env=env,text=True),'cargo':subprocess.check_output(['/Users/ryan/.cargo/bin/rustup','run','1.97.1','cargo','-V'],env=env,text=True)},indent=2)+'\n') if variant=='NONE' else None
try:
    with (out/'k6c_envelope.log').open('w') as log:
        process=subprocess.run(argv,cwd=archive,env=env,stdout=log,stderr=subprocess.STDOUT)
    log=(out/'k6c_envelope.log').read_text()
    record.update(returncode=process.returncode,end_utc=now(),log_sha256=sha(log.encode()),compile_succeeded='Finished `release`' in log,tests_passed=int(re.search(r'(\d+) passed;',log).group(1)) if re.search(r'(\d+) passed;',log) else None,tests_failed=int(re.search(r'(\d+) failed;',log).group(1)) if re.search(r'(\d+) failed;',log) else None,test_binaries=[{'path':str(x),'sha256':sha(x.read_bytes())} for x in sorted((target/'release/deps').glob('k6c_envelope-*')) if x.is_file() and os.access(x,os.X_OK)],source_unchanged_during_test=(archive/source).read_bytes()==active,test_unchanged_during_test=(archive/test).read_bytes()==(raw/'TEST.after.rs').read_bytes())
    if variant!='NONE':
        record['assertion_excerpt']=log[log.index('---- six_axis_springs'):log.index('failures:',log.index('---- six_axis_springs'))]
        record['binary_differs_from_NONE']=record['test_binaries']!=control['test_binaries']
    print(json.dumps(record,indent=2))
finally:
    (archive/source).write_bytes(normal)
    record['normal_source_restored']=(archive/source).read_bytes()==normal
    (out/'RESULT.json').write_text(json.dumps(record,indent=2)+'\n')

from pathlib import Path
import os, subprocess, hashlib, json, datetime, difflib
wt=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3')
r=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
raw=wt/'k6c'/r/'REVIEW_RV35/phase_witnesses_01/_run_records'; archive=wt/'scratch/rv35-phase-witnesses-01/H'; target=wt/'scratch/rv35-phase-witnesses-01/target-H'
h=Path('projects/chirality-piping/core/solver/performance_harness'); file=archive/h/'src/k6/w1/envelope.rs'; test=archive/h/'tests/k6c_envelope.rs'
sha=lambda b:hashlib.sha256(b).hexdigest(); now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
normal=file.read_bytes(); before={str(p.relative_to(archive)):sha(p.read_bytes()) for p in archive.rglob('*') if p.is_file()}
overrides={'GIT_OPTIONAL_LOCKS':'0','RUSTUP_TOOLCHAIN':'1.97.1','RUSTUP_AUTO_INSTALL':'0','CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','RUST_TEST_THREADS':'2','CARGO_TARGET_DIR':str(target)}; env={**os.environ,**overrides}
versions={name:subprocess.check_output(['/Users/ryan/.cargo/bin/rustup','run','1.97.1',name,'-V'],env=env,text=True) for name in ('rustc','cargo')}
for name in ('NONE','M05_UC_C_OVERLAP'):
    needle=b'(4 * f * w + 4 * f).max(2 * f * w + 2 * bc * w)'; assert normal.count(needle)==1
    active=normal if name=='NONE' else normal.replace(needle,b'(3 * f * w + 4 * f).max(2 * f * w + 2 * bc * w)')
    if name!='NONE':
        diff=''.join(difflib.unified_diff(normal.decode().splitlines(True),active.decode().splitlines(True),fromfile='a/'+str(h/'src/k6/w1/envelope.rs'),tofile='b/'+str(h/'src/k6/w1/envelope.rs')))
        assert diff.encode()==(raw/'M05.original.diff').read_bytes()
    file.write_bytes(active)
    guard=subprocess.run(['ps','-p','5387','-o','pid,ppid,command'],text=True,capture_output=True,check=True); assert '/guard/memguard.sh' in guard.stdout
    argv=['/Users/ryan/.cargo/bin/rustup','run','1.97.1','cargo','test','--manifest-path',str(h/'Cargo.toml'),'--release','--locked','--offline','-j','4','--test','k6c_envelope','--','--test-threads=2']
    rec={'candidate':'2ae028eb275684ac0aa8082e034128ff251027a4','variant':name,'start_utc':now(),'argv':argv,'cwd':str(archive),'environment_overrides':overrides,'versions':versions,'guard':guard.stdout,'source_sha256':sha(active),'test_sha256':sha(test.read_bytes())}
    try:
        with (raw/(name+'.log')).open('w') as log: result=subprocess.run(argv,cwd=archive,env=env,stdout=log,stderr=subprocess.STDOUT)
        rec.update(returncode=result.returncode,finished_utc=now(),log_sha256=sha((raw/(name+'.log')).read_bytes()),source_unchanged=file.read_bytes()==active,binaries=[{'path':str(p),'sha256':sha(p.read_bytes())} for p in (target/'release/deps').glob('k6c_envelope-*') if p.is_file() and os.access(p,os.X_OK)])
    finally:
        file.write_bytes(normal); rec['normal_source_restored']=file.read_bytes()==normal; (raw/(name+'.json')).write_text(json.dumps(rec,indent=2)+'\n')
    print(json.dumps({'variant':name,'returncode':result.returncode,'log':str(raw/(name+'.log'))}),flush=True)
    if name=='NONE': assert result.returncode==0
    else: assert result.returncode==101
assert before=={str(p.relative_to(archive)):sha(p.read_bytes()) for p in archive.rglob('*') if p.is_file()}
(raw/'H.archive_restoration.json').write_text(json.dumps({'verified_files':len(before),'all_candidate_files_identical':True,'last_binaries':'M05 diagnostic only'},indent=2)+'\n')

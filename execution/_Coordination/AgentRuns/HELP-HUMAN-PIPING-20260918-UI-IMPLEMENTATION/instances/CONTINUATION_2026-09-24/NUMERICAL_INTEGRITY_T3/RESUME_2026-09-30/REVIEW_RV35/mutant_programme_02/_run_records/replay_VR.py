from pathlib import Path
import os,subprocess,json,hashlib,datetime
wt=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3'); r=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30'); raw=wt/'k6c'/r/'REVIEW_RV35/mutant_programme_02/_run_records'; archive=wt/'scratch/rv35-mutant-programme-02/archive'; target=wt/'scratch/rv35-mutant-programme-02/target-VR'; vr=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness'); source=archive/vr/'src/envelope.rs'; normal=source.read_bytes(); sha=lambda b:hashlib.sha256(b).hexdigest(); now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
before={str(p.relative_to(archive)):sha(p.read_bytes()) for p in archive.rglob('*') if p.is_file()}
overrides={'GIT_OPTIONAL_LOCKS':'0','RUSTUP_TOOLCHAIN':'1.97.1','RUSTUP_AUTO_INSTALL':'0','CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','RUST_TEST_THREADS':'2','CARGO_TARGET_DIR':str(target)}; env={**os.environ,**overrides}
mutant=(raw/'VR-M01-OLD-PORT.postimage.rs').read_bytes(); assert sha(mutant)=='a30cb3c4835c3cce5778b46909a19e2cff41dbb253c453bf8f284e6249fd3c0e'
try:
 for name,active,expected in [('NONE-pre',normal,0),('VR-M01-OLD-PORT',mutant,101),('NONE-post',normal,0)]:
    source.write_bytes(active); guard=subprocess.check_output(['ps','-p','5387','-o','pid,ppid,command'],text=True); assert '/guard/memguard.sh' in guard
    argv=['/Users/ryan/.cargo/bin/rustup','run','1.97.1','cargo','test','--manifest-path',str(vr/'Cargo.toml'),'--locked','--offline','-j','4','--test','k6c_envelope','all_193_independently_reviewed_reference_values_and_source_drop','--','--exact','--test-threads=2']
    rec={'variant':name,'candidate':'81c03849033f3ce745668f581f446530789397b8','record_candidate':'2529cd365b0a3574c89e41b4ee82c525709483c9','start_utc':now(),'argv':argv,'cwd':str(archive),'environment_overrides':overrides,'guard':guard,'source_sha256':sha(active),'test_sha256':sha((archive/vr/'tests/k6c_envelope.rs').read_bytes())}
    with (raw/('independent-'+name+'.log')).open('w') as log:res=subprocess.run(argv,cwd=archive,env=env,stdout=log,stderr=subprocess.STDOUT)
    rec.update(returncode=res.returncode,end_utc=now(),source_unchanged=source.read_bytes()==active,log_sha256=sha((raw/('independent-'+name+'.log')).read_bytes()),binaries=[{'path':str(p),'sha256':sha(p.read_bytes())} for p in (target/'debug/deps').glob('k6c_envelope-*') if p.is_file() and os.access(p,os.X_OK)])
    (raw/('independent-'+name+'.json')).write_text(json.dumps(rec,indent=2)+'\n'); print(json.dumps({'variant':name,'returncode':res.returncode,'expected':expected}),flush=True); assert res.returncode==expected
finally:
 source.write_bytes(normal); after={str(p.relative_to(archive)):sha(p.read_bytes()) for p in archive.rglob('*') if p.is_file()}; (raw/'VR_REPLAY_RESTORATION.json').write_text(json.dumps({'files':len(before),'full_archive_unchanged':before==after,'last_binaries':'normal if NONE-post completed; consult per-check metadata'},indent=2)+'\n'); assert before==after

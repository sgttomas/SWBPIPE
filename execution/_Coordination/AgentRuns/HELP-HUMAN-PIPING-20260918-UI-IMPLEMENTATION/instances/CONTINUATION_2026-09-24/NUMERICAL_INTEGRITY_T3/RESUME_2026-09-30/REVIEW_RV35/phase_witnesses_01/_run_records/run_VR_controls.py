from pathlib import Path
import os,subprocess,json,hashlib,datetime
wt=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3'); r=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
raw=wt/'k6c'/r/'REVIEW_RV35/phase_witnesses_01/_run_records'; archive=wt/'scratch/rv35-phase-witnesses-01/VR'; target=wt/'scratch/rv35-phase-witnesses-01/target-VR'; vr=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness'); source=archive/vr/'src/envelope.rs'; normal=source.read_bytes()
sha=lambda b:hashlib.sha256(b).hexdigest(); now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
before={str(p.relative_to(archive)):sha(p.read_bytes()) for p in archive.rglob('*') if p.is_file()}
overrides={'GIT_OPTIONAL_LOCKS':'0','RUSTUP_TOOLCHAIN':'1.97.1','RUSTUP_AUTO_INSTALL':'0','CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','RUST_TEST_THREADS':'2','CARGO_TARGET_DIR':str(target)}; env={**os.environ,**overrides}
ref='all_193_independently_reviewed_reference_values_and_source_drop'
variants=[('NONE-pre',normal,[('unit',['--lib','envelope::tests::'],0),('integration',['--test','k6c_envelope'],0)]),('VR-M02-SPARSE-16F',normal.replace(b'    n + 16 * f\n',b'    n\n'),[('local',['--lib','envelope::tests::sparse_factor_validation_retains_caller_ordering_phase'],101),('reference',['--test','k6c_envelope',ref],0)]),('VR-M05-REFUSED-FORMAT-ARG',normal.replace(b'    fmt(89.into()) + fmt(out) + e * out\n',b'    fmt(out) + e * out\n'),[('local',['--lib','envelope::tests::refused_format_argument_survives_outer_format_phase'],101),('reference',['--test','k6c_envelope',ref],101)]),('NONE-post',normal,[('unit',['--lib','envelope::tests::'],0),('reference',['--test','k6c_envelope',ref],0)])]
try:
 for variant,active,checks in variants:
    source.write_bytes(active)
    for label,args,expected in checks:
        name=variant+'-'+label
        guard=subprocess.check_output(['ps','-p','5387','-o','pid,ppid,command'],text=True); assert '/guard/memguard.sh' in guard
        argv=['/Users/ryan/.cargo/bin/rustup','run','1.97.1','cargo','test','--manifest-path',str(vr/'Cargo.toml'),'--release','--locked','--offline','-j','4',*args,'--','--test-threads=2']
        rec={'variant':variant,'check':label,'candidate':'81c03849033f3ce745668f581f446530789397b8','start_utc':now(),'argv':argv,'cwd':str(archive),'environment_overrides':overrides,'guard':guard,'source_sha256':sha(active)}
        with (raw/(name+'.log')).open('w') as log: res=subprocess.run(argv,cwd=archive,env=env,stdout=log,stderr=subprocess.STDOUT)
        rec.update(returncode=res.returncode,finish_utc=now(),source_unchanged=source.read_bytes()==active,log_sha256=sha((raw/(name+'.log')).read_bytes()),binaries=[{'path':str(p),'sha256':sha(p.read_bytes())} for p in (target/'release/deps').glob('*') if p.is_file() and os.access(p,os.X_OK) and (p.name.startswith('piping_numerical_robustness-') or p.name.startswith('k6c_envelope-'))])
        (raw/(name+'.json')).write_text(json.dumps(rec,indent=2)+'\n'); print(json.dumps({'check':name,'returncode':res.returncode,'expected':expected}),flush=True)
        assert res.returncode==expected
finally:
 source.write_bytes(normal)
 after={str(p.relative_to(archive)):sha(p.read_bytes()) for p in archive.rglob('*') if p.is_file()}; (raw/'VR.archive_restoration.json').write_text(json.dumps({'verified_files':len(before),'all_candidate_files_identical':before==after,'last_compiled_binaries':'normal if NONE-post completed; see per-check records'},indent=2)+'\n'); assert before==after

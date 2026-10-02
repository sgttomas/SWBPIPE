from pathlib import Path
import os,subprocess,json,hashlib,datetime,difflib
wt=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3'); r=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30'); raw=wt/'k6c'/r/'REVIEW_RV35/mutant_programme_02/_run_records'; archive=wt/'scratch/rv35-mutant-programme-02/archive'; target=wt/'scratch/rv35-mutant-programme-02/target-H'; h=Path('projects/chirality-piping/core/solver/performance_harness'); rel=h/'src/k6/w1/envelope.rs'; source=archive/rel; normal=source.read_bytes(); sha=lambda b:hashlib.sha256(b).hexdigest(); now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
before={str(p.relative_to(archive)):sha(p.read_bytes()) for p in archive.rglob('*') if p.is_file()}
overrides={'GIT_OPTIONAL_LOCKS':'0','RUSTUP_TOOLCHAIN':'1.97.1','RUSTUP_AUTO_INSTALL':'0','CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','RUST_TEST_THREADS':'2','CARGO_TARGET_DIR':str(target)}; env={**os.environ,**overrides}
variants=[('NONE-pre',None,None,[0,0]),('M04_OPTION_REPORT_PLUS8','let report = 344 + 5 * q * w + (f + c) * w + REPORT_BLOCK[vi] * bc + BODY[vi] * b + res;','let report = 344 + 5 * q * (w + 8) + (f + c) * w + REPORT_BLOCK[vi] * bc + BODY[vi] * b + res;',[101,101]),('M12_REBUILD_OLD_DROP','Pair::new(table + kept, old(40, rows)),','Pair::new(kept, old(40, rows)),',[0,101]),('NONE-post',None,None,[0,0])]
checks=[('kernel',['--test','k6c_envelope']),('h33',['--test','k6b_w1','the_committed_counts_carry_this_codes_estimate'])]
try:
 for variant,old,new,expected in variants:
    active=normal if old is None else normal.decode().replace(old,new).encode()
    if old:
        assert normal.decode().count(old)==1
        patch=''.join(difflib.unified_diff(normal.decode().splitlines(True),active.decode().splitlines(True),fromfile='a/'+str(rel),tofile='b/'+str(rel)))
        assert patch.encode()==(raw/(variant+'.diff')).read_bytes()
    source.write_bytes(active)
    for (label,args),code in zip(checks,expected):
        guard=subprocess.check_output(['ps','-p','5387','-o','pid,ppid,command'],text=True); assert '/guard/memguard.sh' in guard
        name='independent-H-'+variant+'-'+label
        argv=['/Users/ryan/.cargo/bin/rustup','run','1.97.1','cargo','test','--manifest-path',str(h/'Cargo.toml'),'--release','--locked','--offline','-j','4',*args,'--',*(['--exact'] if label=='h33' else []),'--test-threads=2']
        rec={'variant':variant,'check':label,'candidate':'81c03849033f3ce745668f581f446530789397b8','start_utc':now(),'argv':argv,'cwd':str(archive),'environment_overrides':overrides,'guard':guard,'source_sha256':sha(active),'test_hashes':{str(h/'tests'/f):sha((archive/h/'tests'/f).read_bytes()) for f in ['k6c_envelope.rs','k6b_w1.rs']}}
        with (raw/(name+'.log')).open('w') as log:res=subprocess.run(argv,cwd=archive,env=env,stdout=log,stderr=subprocess.STDOUT)
        stem='k6c_envelope' if label=='kernel' else 'k6b_w1'; rec.update(returncode=res.returncode,end_utc=now(),source_unchanged=source.read_bytes()==active,log_sha256=sha((raw/(name+'.log')).read_bytes()),binaries=[{'path':str(p),'sha256':sha(p.read_bytes())} for p in (target/'release/deps').glob(stem+'-*') if p.is_file() and os.access(p,os.X_OK)])
        (raw/(name+'.json')).write_text(json.dumps(rec,indent=2)+'\n'); print(json.dumps({'check':name,'returncode':res.returncode,'expected':code}),flush=True); assert res.returncode==code
finally:
 source.write_bytes(normal); after={str(p.relative_to(archive)):sha(p.read_bytes()) for p in archive.rglob('*') if p.is_file()}; (raw/'H_REPLAY_RESTORATION.json').write_text(json.dumps({'files':len(before),'full_archive_unchanged':before==after,'last_binaries':'normal if NONE-post complete; consult per-check metadata'},indent=2)+'\n'); assert before==after

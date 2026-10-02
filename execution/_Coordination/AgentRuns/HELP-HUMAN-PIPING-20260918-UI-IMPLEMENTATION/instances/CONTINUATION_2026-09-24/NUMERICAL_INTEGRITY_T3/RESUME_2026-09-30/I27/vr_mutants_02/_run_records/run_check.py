from pathlib import Path
import sys, os, subprocess, json, datetime, hashlib
wt=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3')
r=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I27/vr_mutants_02/_run_records')
q=wt/'k6c'/r
label, variant, mode = sys.argv[1:4]
s=wt/'scratch/i27-vr-mutants-02'/variant
vr=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness')
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0',RUSTUP_TOOLCHAIN='1.97.1',RUSTUP_AUTO_INSTALL='0',CARGO_NET_OFFLINE='true',CARGO_INCREMENTAL='0',RUST_TEST_THREADS='2',CARGO_TARGET_DIR=str(wt/'k6c-i27-vr-mutants-02-target'))
guard=subprocess.run(['pgrep','-fl','[m]emguard.sh'],capture_output=True,text=True)
assert guard.returncode==0, 'Missing guard: stop'
(q/(label+'.guard.log')).write_text(guard.stdout)
if mode=='build':
    cmd=['cargo','test','--offline','--locked','-j','4','--manifest-path',str(vr/'Cargo.toml'),'--test','k6c_envelope','--no-run','--message-format=json']
else:
    cmd=sys.argv[4:]
rec={'label':label,'variant':variant,'command':cmd,'cwd':str(s),'environment':{k:env[k] for k in ['GIT_OPTIONAL_LOCKS','RUSTUP_TOOLCHAIN','RUSTUP_AUTO_INSTALL','CARGO_NET_OFFLINE','CARGO_INCREMENTAL','RUST_TEST_THREADS','CARGO_TARGET_DIR']},'start_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_sha256':hashlib.sha256((s/vr/'src/envelope.rs').read_bytes()).hexdigest()}
if mode!='build':
    rec['binary_sha256']=hashlib.sha256(Path(cmd[0]).read_bytes()).hexdigest()
with (q/(label+'.stdout.log')).open('w') as stdout, (q/(label+'.stderr.log')).open('w') as stderr:
    proc=subprocess.run(cmd,cwd=s,env=env,stdout=stdout,stderr=stderr)
rec.update(exit_code=proc.returncode,end_utc=datetime.datetime.now(datetime.timezone.utc).isoformat())
if mode=='build':
    artifacts=[]
    for line in (q/(label+'.stdout.log')).read_text().splitlines():
        try: j=json.loads(line)
        except ValueError: continue
        if j.get('reason')=='compiler-artifact' and j.get('executable'):
            artifacts.append({'path':j['executable'],'target':j['target'],'profile':j['profile'],'features':j['features'],'sha256':hashlib.sha256(Path(j['executable']).read_bytes()).hexdigest()})
    rec['artifacts']=artifacts
(q/(label+'.json')).write_text(json.dumps(rec,indent=2)+'\n')
print(json.dumps(rec))
print((q/(label+'.stderr.log')).read_text()[-3000:])
if mode!='build': print((q/(label+'.stdout.log')).read_text()[-4000:])

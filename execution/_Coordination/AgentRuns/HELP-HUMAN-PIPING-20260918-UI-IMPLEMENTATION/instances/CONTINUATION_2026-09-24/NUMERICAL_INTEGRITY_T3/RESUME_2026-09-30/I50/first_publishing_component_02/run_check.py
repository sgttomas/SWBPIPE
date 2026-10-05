from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, os, signal, subprocess, sys
CODE=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a')
BULK=CODE.parent/'scratch/i50_first_publishing/runtime02'
BASE=json.loads((BULK/'BASELINE.json').read_text())
EXTRA='projects/chirality-piping/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json'
def inventory():
    return {p:hashlib.sha256((CODE/p).read_bytes()).hexdigest() for p in sorted(set(BASE)|{EXTRA}) if (CODE/p).is_file()}
def digest(x):return hashlib.sha256(json.dumps(x,sort_keys=True,separators=(',',':')).encode()).hexdigest()
name,kind,*args=sys.argv[1:]
manifest=CODE/'projects/chirality-piping/core'/({'pp':'product_physics','fk':'solver/frame_kernel'}[kind])/'Cargo.toml'
argv=['/Users/ryan/.cargo/bin/cargo','test','--manifest-path',str(manifest),'--locked','--offline',*args]
env=dict(os.environ);env.update(CARGO_BUILD_JOBS='4',RUST_TEST_THREADS='2',CARGO_TARGET_DIR=str(CODE.parent/'targets/i47-selected-material'/({'pp':'product_physics','fk':'frame_kernel'}[kind])))
before=inventory(); start=datetime.now(timezone.utc).isoformat()
with (BULK/(name+'.log')).open('wb') as log:
    process=subprocess.Popen(argv,cwd=CODE,env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
    try: code=process.wait(timeout=1200)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid,signal.SIGTERM)
        try: process.wait(timeout=5)
        except subprocess.TimeoutExpired: os.killpg(process.pid,signal.SIGKILL);process.wait()
        code=124
after=inventory()
record={'argv':argv,'cwd':str(CODE),'start':start,'finish':datetime.now(timezone.utc).isoformat(),'exit':code,'pid':process.pid,'reaped':True,'wall_limit_seconds':1200,'env':{k:env[k] for k in ['CARGO_BUILD_JOBS','RUST_TEST_THREADS','CARGO_TARGET_DIR']},'baseline':str(BULK/'BASELINE.json'),'before_digest':digest(before),'after_digest':digest(after),'source_stable':before==after,'delta_from_baseline':{p:h for p,h in before.items() if BASE.get(p)!=h}}
(BULK/(name+'.json')).write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps(record));sys.exit(code if code>=0 else 128-code)

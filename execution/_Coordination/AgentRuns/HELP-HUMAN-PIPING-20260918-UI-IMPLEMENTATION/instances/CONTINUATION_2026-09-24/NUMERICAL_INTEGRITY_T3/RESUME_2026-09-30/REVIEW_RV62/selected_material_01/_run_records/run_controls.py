from pathlib import Path
import subprocess,hashlib,os,json,datetime,sys
wt=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3'); code=wt/'scratch/rv62-selected-material'; out=Path(__file__).resolve().parent
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def inventory():
 return {str(p.relative_to(code)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((code/'projects/chirality-piping/core').rglob('*')) if p.is_file() and 'target' not in p.parts}
label=sys.argv[1]; crate=sys.argv[2]; options=sys.argv[3:]
env={**os.environ,'GIT_OPTIONAL_LOCKS':'0','CARGO_BUILD_JOBS':'4','RUST_TEST_THREADS':'2','CARGO_TARGET_DIR':str(wt/'targets/rv62-selected-material'/crate.split('/')[-1])}
cmd=[str(Path.home()/'.cargo/bin/cargo'),'test','--manifest-path',str(code/'projects/chirality-piping/core'/crate/'Cargo.toml'),'--locked','--offline',*options]
rec={'argv':cmd,'cwd':str(code),'environment':{k:env[k] for k in ['GIT_OPTIONAL_LOCKS','CARGO_BUILD_JOBS','RUST_TEST_THREADS','CARGO_TARGET_DIR']},'start':now(),'before':inventory(),'timeout_seconds':1200}
try:
 with (out/(label+'.log')).open('wb') as f: rec['exit']=subprocess.run(cmd,cwd=code,env=env,stdout=f,stderr=subprocess.STDOUT,timeout=1200).returncode
except subprocess.TimeoutExpired:rec['exit']='timeout'
rec['end']=now();rec['after']=inventory();rec['source_unchanged']=rec['before']==rec['after'];(out/(label+'.json')).write_text(json.dumps(rec,indent=2)+'\n');print(json.dumps({k:v for k,v in rec.items() if k not in ['before','after']}))

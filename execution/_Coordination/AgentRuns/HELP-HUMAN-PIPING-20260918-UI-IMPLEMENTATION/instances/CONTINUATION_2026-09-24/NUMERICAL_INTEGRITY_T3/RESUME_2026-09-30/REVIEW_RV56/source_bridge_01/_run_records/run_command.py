import datetime, hashlib, json, os, pathlib, signal, subprocess, sys
packet=pathlib.Path(__file__).resolve().parent
label=sys.argv[1]; cwd=sys.argv[2]; argv=sys.argv[3:]
source=pathlib.Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel')
freeze={str(f):hashlib.sha256(f.read_bytes()).hexdigest() for f in sorted(source.rglob('*.rs')) if 'target' not in f.parts}
freeze[str(source/'Cargo.toml')]=hashlib.sha256((source/'Cargo.toml').read_bytes()).hexdigest()
(packet/(label+'.freeze.json')).write_text(json.dumps(freeze,indent=2)+'\n')
env=os.environ.copy();env.update(CARGO_BUILD_JOBS='4',RUST_TEST_THREADS='2',CARGO_TARGET_DIR=str(packet.parent/'target'/(('imported' if 'imported' in label else 'candidate'))))
record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),argv=argv,cwd=cwd,env={k:env[k] for k in ('CARGO_BUILD_JOBS','RUST_TEST_THREADS','CARGO_TARGET_DIR')},wall_seconds=1200,guard_pid=5387)
with open(packet/(label+'.stdout'),'w') as out,open(packet/(label+'.stderr'),'w') as err:
 p=subprocess.Popen(argv,cwd=cwd,env=env,stdout=out,stderr=err,start_new_session=True);record['pid']=p.pid
 (packet/(label+'.json')).write_text(json.dumps(record,indent=2)+'\n')
 try:record['exit']=p.wait(timeout=1200)
 except subprocess.TimeoutExpired:
  os.killpg(p.pid,signal.SIGTERM)
  try:p.wait(timeout=10)
  except subprocess.TimeoutExpired:os.killpg(p.pid,signal.SIGKILL);p.wait()
  record.update(exit=p.returncode,timed_out=True)
record['end']=datetime.datetime.now(datetime.timezone.utc).isoformat()
(packet/(label+'.json')).write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps(record,indent=2))
print((packet/(label+'.stderr')).read_text()[-3000:])
print((packet/(label+'.stdout')).read_text()[-2000:])
sys.exit(record['exit'])

from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,signal,subprocess
CODE=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a')
BULK=CODE.parent/'scratch/rv65_named_support/repair04'
paths=['projects/chirality-piping/core/product_physics/src/retained_product.rs','projects/chirality-piping/core/product_physics/src/retained_product_tests.rs']
def inventory():return {p:hashlib.sha256((CODE/p).read_bytes()).hexdigest() for p in paths}
argv=['/Users/ryan/.cargo/bin/cargo','test','--manifest-path',str(CODE/'projects/chirality-piping/core/product_physics/Cargo.toml'),'--locked','--offline','--lib','retained_product_tests','--','--nocapture']
env=dict(os.environ,CARGO_BUILD_JOBS='4',RUST_TEST_THREADS='2',CARGO_TARGET_DIR=str(CODE.parent/'targets/rv65-named-support/product_physics'))
before=inventory();start=datetime.now(timezone.utc).isoformat()
with (BULK/'pp_debug.log').open('xb') as log:
 p=subprocess.Popen(argv,cwd=str(CODE),env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
 try:rc=p.wait(timeout=1200)
 except subprocess.TimeoutExpired:
  os.killpg(p.pid,signal.SIGTERM)
  try:p.wait(timeout=5)
  except subprocess.TimeoutExpired:os.killpg(p.pid,signal.SIGKILL);p.wait()
  rc=124
after=inventory();record=dict(candidate='c79a1c293dbf5581468e839c8545b5a815a32df7',argv=argv,cwd=str(CODE),
 start=start,finish=datetime.now(timezone.utc).isoformat(),exit=rc,pid=p.pid,reaped=True,wall_limit_seconds=1200,
 env={k:env[k] for k in ['CARGO_BUILD_JOBS','RUST_TEST_THREADS','CARGO_TARGET_DIR']},before=before,after=after,source_stable=before==after)
(BULK/'pp_debug.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record));raise SystemExit(rc)

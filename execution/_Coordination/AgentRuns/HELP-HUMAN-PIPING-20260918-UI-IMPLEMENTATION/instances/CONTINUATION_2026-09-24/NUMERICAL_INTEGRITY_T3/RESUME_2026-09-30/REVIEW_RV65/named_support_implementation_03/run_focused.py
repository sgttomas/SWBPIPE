from pathlib import Path
from datetime import datetime, timezone
import hashlib,json,os,signal,subprocess,sys,time
CODE=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a')
BULK=CODE.parent/'scratch/rv65_named_support/implementation03'
R=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
files=json.loads((CODE/R/'I50/first_publishing_component_02/FINAL_SOURCE.json').read_text())['files']
def inventory():return {e['path']:hashlib.sha256((CODE/e['path']).read_bytes()).hexdigest() for e in files}
suite=[('pp_debug','pp',['--lib','retained_product_tests','--','--nocapture']),
 ('fk_certificate','fk',['--lib','structural::retained::product_certificate','--','--nocapture']),
 ('formation_named','pp',['--test','formation_check_runtime','kd5_required_true_positive_122_demotes_in_both_modes_on_both_entries','--','--exact','--nocapture']),
 ('pp_s11','pp',['--test','s11f_site_test']),('fk_s11','fk',['--test','s11_site_table']),
 ('pp_release','pp',['--release','--lib','retained_product_tests','--','--nocapture'])]
for name,kind,args in suite:
 if name=='pp_release' and datetime.now(timezone.utc).isoformat()>'2026-10-03T08:05:00':
  print('SKIP optimized fresh check: time budget',flush=True);break
 manifest=CODE/'projects/chirality-piping/core'/({'pp':'product_physics','fk':'solver/frame_kernel'}[kind])/'Cargo.toml'
 target=CODE.parent/'targets/rv65-named-support'/({'pp':'product_physics','fk':'frame_kernel'}[kind])
 env=dict(os.environ,CARGO_BUILD_JOBS='4',RUST_TEST_THREADS='2',CARGO_TARGET_DIR=str(target))
 argv=['/Users/ryan/.cargo/bin/cargo','test','--manifest-path',str(manifest),'--locked','--offline',*args]
 before=inventory();start=datetime.now(timezone.utc).isoformat();print('START '+name+' '+start,flush=True)
 with (BULK/(name+'.log')).open('xb') as log:
  p=subprocess.Popen(argv,cwd=str(CODE),env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
  try:rc=p.wait(timeout=1200)
  except subprocess.TimeoutExpired:
   os.killpg(p.pid,signal.SIGTERM)
   try:p.wait(timeout=5)
   except subprocess.TimeoutExpired:os.killpg(p.pid,signal.SIGKILL);p.wait()
   rc=124
 after=inventory()
 rec=dict(name=name,argv=argv,cwd=str(CODE),start=start,finish=datetime.now(timezone.utc).isoformat(),exit=rc,pid=p.pid,reaped=True,
  wall_limit_seconds=1200,env={k:env[k] for k in ['CARGO_BUILD_JOBS','RUST_TEST_THREADS','CARGO_TARGET_DIR']},
  candidate='8104a4fedd0fd575f20b3d723cc0cdd722d73ba1',before=before,after=after,source_stable=before==after)
 (BULK/(name+'.json')).write_text(json.dumps(rec,indent=2)+'\n');print('END '+name+' exit='+str(rc)+' '+rec['finish'],flush=True)
 if before!=after or rc==124:break

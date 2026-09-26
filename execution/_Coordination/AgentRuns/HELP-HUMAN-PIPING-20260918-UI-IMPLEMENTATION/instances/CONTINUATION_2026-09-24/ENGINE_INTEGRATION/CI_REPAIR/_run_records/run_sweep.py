from pathlib import Path
import subprocess,os,json,datetime,hashlib,time,sys
repo=Path('/private/tmp/piping-engine-integration-20260925'); project=repo/'projects/chirality-piping'; out=Path('/private/tmp/piping-joined-qualification-20260925')
expected=sys.argv[1]; binding=Path(sys.argv[2])
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip(); status=subprocess.check_output(['git','status','--porcelain'],cwd=repo,text=True)
assert head==expected and not status,(head,status)
b=json.loads(binding.read_text());assert b['head_sha']==head and b['conclusion']=='success'
env=os.environ.copy();env['PATH']='/private/tmp/chirality-piping-dec025-venv/bin:/Users/ryan/.local/share/mise/installs/node/24/bin:'+env['PATH'];env['CARGO_BUILD_JOBS']='2';env['CARGO_NET_OFFLINE']='true';env['VITEST_MAX_WORKERS']='2';env.pop('CARGO_TARGET_DIR',None)
for name in ['OPENPIPESTRESS_CHECKED_JSON_BIN','OPENPIPESTRESS_BINARY64_JSON_BIN']:env.pop(name,None)
cmd=['/private/tmp/chirality-piping-dec025-venv/bin/python','tools/release/run_evidence_sweep.py','--execute','--require-capability','host','--surface4-ci-binding',str(binding),'--output-dir',str(out/'sweep')]
record={'candidate':head,'initial_status':status,'cwd':str(project),'command':cmd,'environment':{k:env.get(k) for k in ['PATH','CARGO_BUILD_JOBS','CARGO_NET_OFFLINE','CARGO_TARGET_DIR','VITEST_MAX_WORKERS']},'binding_sha256':hashlib.sha256(binding.read_bytes()).hexdigest(),'started_at':datetime.datetime.now(datetime.timezone.utc).isoformat()};(out/'sweep-invocation.json').write_text(json.dumps(record,indent=2)+'\n')
started=time.monotonic()
with (out/'sweep.log').open('wb') as log: result=subprocess.run(cmd,cwd=project,env=env,stdout=log,stderr=subprocess.STDOUT)
record.update(exit_code=result.returncode,elapsed_seconds=time.monotonic()-started,finished_at=datetime.datetime.now(datetime.timezone.utc).isoformat());(out/'sweep-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({'candidate':head,'exit_code':result.returncode,'elapsed_seconds':record['elapsed_seconds']}),flush=True)
sys.exit(result.returncode)

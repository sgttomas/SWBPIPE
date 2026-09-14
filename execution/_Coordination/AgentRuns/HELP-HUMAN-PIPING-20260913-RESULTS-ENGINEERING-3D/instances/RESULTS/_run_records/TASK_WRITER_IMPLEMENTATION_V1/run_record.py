from pathlib import Path
import os,sys,json,subprocess,time,shutil,hashlib
lane=Path(__file__).resolve().parent
number,claim,*argv=sys.argv[1:]
a=lane/'attempts'/number;a.mkdir(parents=True,exist_ok=False)
inputs=[]
project=Path.cwd()
for prefix in ['core/reporting/result_export/src','core/runner/headless/src','fixtures/results','schemas','apps/desktop/src','tests']:
 for p in sorted((project/prefix).rglob('*')):
  if p.is_file():inputs.append({'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
for arg in argv:
 p=project/arg
 if p.is_file() and not any(x['path']==str(p) for x in inputs):inputs.append({'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
record={'allocation':'ROOT_LEASE_RESULTS_IMPLEMENTATION_01','argv':argv,'cwd':str(project),'executable':shutil.which(argv[0]),'env':{'CARGO_NET_OFFLINE':'true','CARGO_TARGET_DIR':'/tmp/chirality-results-engineering-3d-20260913/results-implementation-target','RESULTS_RUST_CONTRACT_OUTPUT_DIR':str(lane/'captures'/number/'rust'),'RESULTS_CONTRACT_OUTPUT_DIR':str(lane/'captures'/number/'desktop')},'toolchain_identity_ref':'attempts/001-cargo,002-rustc,003-node,004-python','claim':claim,'inputs':inputs,'stdout':str(a/'stdout.txt'),'stderr':str(a/'stderr.txt'),'prepared_before_execution':True}
(a/'COMMAND.json').write_text(json.dumps(record,indent=2)+'\n')
print('PREPARED',str(a/'COMMAND.json'),flush=True)
env=dict(os.environ);env.update(record['env']);start=time.time()
with (a/'stdout.txt').open('w') as out,(a/'stderr.txt').open('w') as err:
 result=subprocess.run(argv,cwd=project,env=env,stdout=out,stderr=err)
(a/'RESULT.json').write_text(json.dumps({'exit_code':result.returncode,'elapsed_seconds':time.time()-start,'stdout_sha256':hashlib.sha256((a/'stdout.txt').read_bytes()).hexdigest(),'stderr_sha256':hashlib.sha256((a/'stderr.txt').read_bytes()).hexdigest()},indent=2)+'\n')
print('EXIT',result.returncode,flush=True);print((a/'stdout.txt').read_text()[-15000:]);print((a/'stderr.txt').read_text()[-15000:]);sys.exit(result.returncode)

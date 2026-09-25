from pathlib import Path
import datetime,hashlib,json,os,subprocess,sys,time
root=Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip());p=root/'projects/chirality-piping'
e=Path(__file__).resolve().parent
label=sys.argv[1];assert label.replace('_','').isalnum()
paths=['tools/validation/qualification_gate.py','tools/validation/qualification_process.py','tests/test_qualification_gate.py','validation/qualification/GATE_USAGE.md']
command=[sys.executable,'-m','unittest','discover','-s','tests','-p','test_qualification_gate.py','-v']
env=os.environ.copy();env['PYTHONDONTWRITEBYTECODE']='1'
start=time.monotonic()
with (e/(label+'.log')).open('w') as log:result=subprocess.run(command,cwd=p,env=env,stdout=log,stderr=subprocess.STDOUT)
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':command,'cwd':'WORKING_ROOT','exit_code':result.returncode,'seconds':time.monotonic()-start,'scope':'Synthetic subprocess and received-packet gate verification only; no Piping solve, Cargo, fullsuite, CI or native run','files':[{'path':'projects/chirality-piping/'+x,'sha256':hashlib.sha256((p/x).read_bytes()).hexdigest()} for x in paths],'log_sha256':hashlib.sha256((e/(label+'.log')).read_bytes()).hexdigest()}
(e/(label+'.json')).write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record));print('\n'.join((e/(label+'.log')).read_text().splitlines()[-8:]));sys.exit(result.returncode)

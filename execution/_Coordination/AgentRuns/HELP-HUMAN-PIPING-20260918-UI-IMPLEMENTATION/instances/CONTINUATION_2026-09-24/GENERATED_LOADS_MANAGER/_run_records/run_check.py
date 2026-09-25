from pathlib import Path
import os,sys,subprocess,json,hashlib,time,datetime
r=Path('/private/tmp/piping-generated-loads-20260924')
e=Path(__file__).resolve().parent.parent
label,cwd,target,*cmd=sys.argv[1:]
env=os.environ.copy(); env['CARGO_BUILD_JOBS']='2'; env['CARGO_NET_OFFLINE']='true'
if target=='-': env.pop('CARGO_TARGET_DIR',None)
else: env['CARGO_TARGET_DIR']=target
freeze=json.loads((e/'CANDIDATE_FILES.json').read_text())
actual=[{'path':v['path'],'sha256':hashlib.sha256((r/v['path']).read_bytes()).hexdigest()} for v in freeze['files']]
assert actual==freeze['files'],'Candidate drift before check'
start=datetime.datetime.now(datetime.timezone.utc).isoformat(); tick=time.time()
p=subprocess.run(cmd,cwd=r/cwd,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
out=e/'_run_records/checks'/label; out.parent.mkdir(exist_ok=True)
out.with_suffix('.log').write_text(p.stdout)
record={'command':cmd,'cwd':str(r/cwd),'environment':{k:env.get(k) for k in ['CARGO_BUILD_JOBS','CARGO_NET_OFFLINE','CARGO_TARGET_DIR']},'started_at':start,'elapsed_seconds':time.time()-tick,'exit_code':p.returncode,'candidate_patch_sha256':freeze['patch_sha256'],'source_files':actual,'log_sha256':hashlib.sha256(p.stdout.encode()).hexdigest()}
record['source_files_after']=[{'path':v['path'],'sha256':hashlib.sha256((r/v['path']).read_bytes()).hexdigest()} for v in freeze['files']]
record['candidate_unchanged']=record['source_files_after']==actual
out.with_suffix('.result.json').write_text(json.dumps(record,indent=2)+'\n')
print(p.stdout[-5000:]); print('EXIT',p.returncode); sys.exit(p.returncode)

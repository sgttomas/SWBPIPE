#!/usr/bin/env python3
import argparse,json,os,subprocess,time
from pathlib import Path
p=argparse.ArgumentParser(); p.add_argument('--record',required=True); p.add_argument('--cwd',required=True); p.add_argument('--stdout',required=True); p.add_argument('--stderr',required=True); p.add_argument('--env',action='append',default=[]); p.add_argument('argv',nargs=argparse.REMAINDER); a=p.parse_args()
argv=a.argv[1:] if a.argv and a.argv[0]=='--' else a.argv
env=os.environ.copy(); shown={}
for item in a.env:
 k,v=item.split('=',1); env[k]=v; shown[k]=v
start=time.time(); cp=subprocess.run(argv,cwd=a.cwd,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE); end=time.time()
Path(a.stdout).write_bytes(cp.stdout); Path(a.stderr).write_bytes(cp.stderr)
rec={'argv':argv,'cwd':a.cwd,'environment':shown,'started_unix':start,'ended_unix':end,'elapsed_seconds':end-start,'exit_code':cp.returncode,'stdout':a.stdout,'stderr':a.stderr}
Path(a.record).write_text(json.dumps(rec,indent=2,sort_keys=True)+'\n')
print(json.dumps(rec,indent=2))
raise SystemExit(cp.returncode)

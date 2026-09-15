#!/usr/bin/env python3
import json,os,subprocess,sys,time
from pathlib import Path
out=Path(sys.argv[1]); tmp='/private/tmp/piping-foundation-native-repaired-20260915'
raw=subprocess.run(['ps','-axo','pid=,ppid=,comm=,args='],text=True,capture_output=True,check=True).stdout.splitlines()
matches=[]
for line in raw:
 parts=line.strip().split(None,3)
 if len(parts)<4: continue
 pid,ppid,comm,args=parts
 if int(pid)==os.getpid() or int(ppid)==os.getpid(): continue
 app=comm.endswith('/openpipestress-desktop') or comm=='openpipestress-desktop'
 browser=('Google Chrome' in comm or comm.endswith('/Google Chrome')) and tmp in args
 server=(comm.endswith('/node') or comm=='node' or comm.endswith('/npm') or comm=='npm') and tmp in args
 build=(comm.endswith('/cargo') or comm=='cargo' or comm.endswith('/rustc') or comm=='rustc') and tmp in args
 if app or browser or server or build: matches.append({'pid':int(pid),'ppid':int(ppid),'comm':comm,'args':args,'class':'app' if app else 'browser' if browser else 'server' if server else 'build'})
rec={'schema':'final-native-v3-owned-process-cleanup-v2','captured_unix':time.time(),'owned_matches':matches,'owned_process_count':len(matches),'app_absent':not any(x['class']=='app' for x in matches),'browser_absent':not any(x['class']=='browser' for x in matches),'server_absent':not any(x['class']=='server' for x in matches),'build_absent':not any(x['class']=='build' for x in matches),'normal_quit_ui_outcome':'Computer Use returned App quit after Command-Q'}
out.write_text(json.dumps(rec,indent=2,sort_keys=True)+'\n');print(json.dumps(rec,indent=2))

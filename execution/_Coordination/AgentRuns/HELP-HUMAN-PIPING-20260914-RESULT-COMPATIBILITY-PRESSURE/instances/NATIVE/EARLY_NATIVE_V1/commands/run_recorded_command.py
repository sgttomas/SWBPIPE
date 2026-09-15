from pathlib import Path
import json,os,subprocess,sys,time
cwd=Path(sys.argv[1]); out=Path(sys.argv[2]); err=Path(sys.argv[3]); record=Path(sys.argv[4]); argv=sys.argv[5:]
env=dict(os.environ); env["CARGO_NET_OFFLINE"]="true"
started=time.time(); completed=subprocess.run(argv,cwd=cwd,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
ended=time.time(); out.write_bytes(completed.stdout); err.write_bytes(completed.stderr)
record.write_text(json.dumps({"argv":argv,"cwd":str(cwd),"environment":{"CARGO_NET_OFFLINE":"true"},"started_unix":started,"ended_unix":ended,"elapsed_seconds":ended-started,"exit_code":completed.returncode},indent=2,sort_keys=True)+"\n")
raise SystemExit(completed.returncode)


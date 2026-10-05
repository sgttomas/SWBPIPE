from pathlib import Path
import subprocess,os,json,datetime,sys,signal,hashlib
r=Path(__file__).resolve().parent.parent
src=Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a")
rec=r/"_run_records"
commands=[("independent_debug",r/"controls/Cargo.toml",["--lib"],"review"),("independent_release",r/"controls/Cargo.toml",["--release","--lib"],"review"),("maintained_origins",src/"projects/chirality-piping/core/solver/frame_kernel/Cargo.toml",["--locked","--lib","origins::tests"],"kernel")]
for name,manifest,flags,target in commands:
 argv=["cargo","test","--offline","--manifest-path",str(manifest),"-j","4",*flags,"--","--nocapture"]
 env=dict(os.environ,CARGO_TARGET_DIR=str(r/"_scratch"/target),RUST_TEST_THREADS="2",GIT_OPTIONAL_LOCKS="0")
 info={"argv":argv,"cwd":str(src),"environment":{k:env[k] for k in ["CARGO_TARGET_DIR","RUST_TEST_THREADS","GIT_OPTIONAL_LOCKS"]},"wall_seconds":1200,"start":datetime.datetime.now(datetime.timezone.utc).isoformat(),"source":"MAINTAINED_HASHES.json","control_sha256":hashlib.sha256((r/"controls/lib.rs").read_bytes()).hexdigest()}
 with (rec/(name+".log")).open("w") as log:
  p=subprocess.Popen(argv,cwd=src,env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True);info["pid"]=p.pid
  (rec/(name+".json")).write_text(json.dumps(info,indent=2)+"\n")
  try: info["exit"]=p.wait(timeout=1200)
  except subprocess.TimeoutExpired:
   os.killpg(p.pid,signal.SIGTERM);p.wait(timeout=30);info["exit"]=124
 info["end"]=datetime.datetime.now(datetime.timezone.utc).isoformat();(rec/(name+".json")).write_text(json.dumps(info,indent=2)+"\n")
 print(name,info["exit"],flush=True)
 print((rec/(name+".log")).read_text()[-6000:],flush=True)
 if info["exit"]:sys.exit(info["exit"])

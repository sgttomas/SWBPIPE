from pathlib import Path
import os,sys,json,hashlib,subprocess,datetime,signal
HERE=Path(__file__).resolve().parent
NUM=next(p for p in HERE.parents if p.name=="numerics")
WT=NUM.parent
CODE=WT/"f2a"
name,crate,*args=sys.argv[1:]
source=CODE if not name.startswith("control_") else WT/"scratch/rv60-product-vertical"
manifest=source/"projects/chirality-piping/core"/crate/"Cargo.toml"
target=WT/"targets/rv60-product-vertical"/crate.split("/")[-1]
cmd=["/Users/ryan/.cargo/bin/cargo","test","--manifest-path",str(manifest),"--locked","--offline",*args]
env={**os.environ,"CARGO_BUILD_JOBS":"4","RUST_TEST_THREADS":"2","CARGO_TARGET_DIR":str(target),"GIT_OPTIONAL_LOCKS":"0"}
inv=json.loads((HERE/"CANDIDATE_SOURCE.json").read_text())
def hashes():return {x["path"]:hashlib.sha256((CODE/x["path"]).read_bytes()).hexdigest() for x in inv}
record={"argv":cmd,"cwd":str(source),"target":str(target),"jobs":4,"threads":2,"timeout_seconds":1200,"start":datetime.datetime.now(datetime.timezone.utc).isoformat(),"before":hashes()}
with (HERE/(name+".log")).open("w") as log:
 p=subprocess.Popen(cmd,cwd=source,env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
 try: record["exit"]=p.wait(timeout=1200)
 except subprocess.TimeoutExpired:
  os.killpg(p.pid,signal.SIGTERM);record["timeout"]=True;record["exit"]=p.wait(timeout=30)
record["after"]=hashes();record["end"]=datetime.datetime.now(datetime.timezone.utc).isoformat()
(HERE/(name+".json")).write_text(json.dumps(record,indent=2)+"\n")
print(json.dumps({k:v for k,v in record.items() if k not in ("before","after")}))
print("\n".join(x for x in (HERE/(name+".log")).read_text().splitlines() if x.startswith("test ") or x.startswith("error")))

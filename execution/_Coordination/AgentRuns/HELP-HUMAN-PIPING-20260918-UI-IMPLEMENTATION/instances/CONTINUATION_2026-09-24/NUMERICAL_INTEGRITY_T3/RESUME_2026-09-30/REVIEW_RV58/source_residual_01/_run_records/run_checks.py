from pathlib import Path
import sys,os,json,hashlib,subprocess,time,datetime
out=Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/REVIEW_RV58/source_residual_01")
src=Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic")
kernel=Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel")
target=Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/targets/rv58-source-residual/frame_kernel")
name=sys.argv[1]
if name == 'reviewer_fixture':
 kernel=out/'_run_records/fixture'
 target=target.parent/'reviewer-fixture'
commands={
"reviewer_fixture":["cargo","test","--manifest-path",str(kernel/"Cargo.toml"),"--locked","--offline","--lib","source_residual_rv58","--","--nocapture"],
"native_debug":["cargo","test","--manifest-path",str(kernel/"Cargo.toml"),"--locked","--offline","--lib","source_residual","--","--nocapture"],
"native_release":["cargo","test","--manifest-path",str(kernel/"Cargo.toml"),"--locked","--offline","--release","--lib","source_residual","--","--nocapture"],
"s11":["cargo","test","--manifest-path",str(kernel/"Cargo.toml"),"--locked","--offline","--test","s11_site_table"],
"member":["cargo","test","--manifest-path",str(kernel/"Cargo.toml"),"--locked","--offline","--lib","product_certificate::tests"],
"bridge":["cargo","test","--manifest-path",str(kernel/"Cargo.toml"),"--locked","--offline","--lib","source_bridge_tests"],
"sqrt":["cargo","test","--manifest-path",str(kernel/"Cargo.toml"),"--locked","--offline","--lib","directed::certificate::tests"]
}
argv=commands[name]
env=dict(os.environ,CARGO_BUILD_JOBS="4",RUST_TEST_THREADS="2",CARGO_TARGET_DIR=str(target),GIT_OPTIONAL_LOCKS="0")
paths=json.loads((out/"_run_records/SOURCE_BEFORE.json").read_text())
def hashes():return {p:hashlib.sha256((src/p).read_bytes()).hexdigest() for p in paths}
start=time.monotonic();before=hashes()
record={"argv":argv,"cwd":str(src),"utc":datetime.datetime.now(datetime.timezone.utc).isoformat(),"source_before":before,"timeout_seconds":1200,"environment":{k:env[k] for k in ["CARGO_BUILD_JOBS","RUST_TEST_THREADS","CARGO_TARGET_DIR","GIT_OPTIONAL_LOCKS"]}}
with (out/("_run_records/"+name+".log")).open("w") as log:
 try:r=subprocess.run(argv,cwd=src,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=1200);record["exit_code"]=r.returncode
 except subprocess.TimeoutExpired:record["timeout"]=True;record["exit_code"]="timeout"
record["elapsed_seconds"]=time.monotonic()-start;record["source_after"]=hashes();record["source_unchanged"]=before==record["source_after"]
(out/("_run_records/"+name+".command.json")).write_text(json.dumps(record,indent=2)+"\n")
print(json.dumps({k:record[k] for k in ["exit_code","elapsed_seconds","source_unchanged"]}))

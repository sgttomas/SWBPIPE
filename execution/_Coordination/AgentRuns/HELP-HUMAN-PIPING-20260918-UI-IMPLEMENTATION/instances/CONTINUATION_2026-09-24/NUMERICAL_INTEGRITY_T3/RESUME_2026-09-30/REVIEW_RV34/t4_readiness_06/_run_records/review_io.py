import pathlib,subprocess,os,json,hashlib,io,tarfile,functools,sys
REC=pathlib.Path(__file__).resolve().parent
ROOT=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c")
R="projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30"
REV="a7f119286185d8d6d62b00f9a51a045ee965313a"
NUM="0deebdf9a248aabe3bbb5ce6109abaca68788885"
SOURCE="81c03849033f3ce745668f581f446530789397b8"
P=R+"/I26/t4_readiness_09"
H="projects/chirality-piping/core/solver/performance_harness"
@functools.lru_cache(None)
def get(path,rev=REV):
 b=subprocess.check_output(["git","show",rev+":"+path],cwd=ROOT,env=dict(os.environ,GIT_OPTIONAL_LOCKS="0"))
 with (REC/"READS.jsonl").open("a") as f:f.write(json.dumps({"path":path,"revision":rev,"sha256":hashlib.sha256(b).hexdigest(),"bytes":len(b)})+"\n")
 return b
def js(path,rev=REV):return json.loads(get(path,rev))
def packet(name):return js(P+"/_run_records/"+name)
def save(name,data):(REC/name).write_text(json.dumps(data,indent=2)+"\n")
@functools.lru_cache(None)
def archive(tier):
 name="records.tar.gz" if tier==1 else "qualification.tar.gz" if tier==4 else "delta_records.tar.gz"
 sub="W1_T4_READINESS" if tier==4 else "W1_T"+str(tier)
 with tarfile.open(fileobj=io.BytesIO(get(R+"/MEASUREMENTS/"+sub+"/_run_records/"+name)),mode="r:gz") as t:return {m.name:t.extractfile(m).read() for m in t.getmembers() if m.isfile()}

import pathlib,subprocess,os,json,hashlib,io,tarfile,functools,sys
REC=pathlib.Path(__file__).resolve().parent
ROOT=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c")
R="projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30"
REV="9db50d42e47990834fcfd2612f9fc81357014de7"
NUM="7008ce5260fe1081abdd932120037384adcc6e42"
SOURCE="81c03849033f3ce745668f581f446530789397b8"
P=R+"/I26/t1_measurements_06"
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
def archive():
    b=get(R+"/MEASUREMENTS/W1_T1/_run_records/records.tar.gz")
    with tarfile.open(fileobj=io.BytesIO(b),mode="r:gz") as t:
        return {m.name:t.extractfile(m).read() for m in t.getmembers() if m.isfile()}

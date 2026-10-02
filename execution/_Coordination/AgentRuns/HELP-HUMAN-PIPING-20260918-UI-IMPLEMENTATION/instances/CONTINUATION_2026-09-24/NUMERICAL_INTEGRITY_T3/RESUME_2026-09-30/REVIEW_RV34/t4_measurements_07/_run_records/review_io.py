import pathlib,subprocess,os,json,hashlib,io,tarfile,functools,sys
REC=pathlib.Path(__file__).resolve().parent
ROOT=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c")
R="projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30"
T3=R.rsplit("/",1)[0]
REV="8cf5271b13af9cb4ddb383491b11ea15bc5e6ea0"
SOURCE="81c03849033f3ce745668f581f446530789397b8"
P=R+"/I26/t4_measurements_10"
H="projects/chirality-piping/core/solver/performance_harness"
def gitraw(path,rev):return subprocess.check_output(["git","show",rev+":"+path],cwd=ROOT,env=dict(os.environ,GIT_OPTIONAL_LOCKS="0"))
LEDGER=json.loads(gitraw(R+"/verification/k6c_record_placement_01/_run_records/RELOCATION.json",REV))
def expand(s):
 if s.startswith("R/"):return R+s[1:]
 if s.startswith("T3/"):return T3+s[2:]
 if s.startswith("P/"):return "projects/chirality-piping"+s[1:]
 return s
@functools.lru_cache(None)
def get(path,rev=REV):
 actual=path
 if rev==REV:
  for u in sorted(LEDGER["units"],key=lambda u:len(u["source"]),reverse=True):
   src=expand(u["source"]);dst=expand(u["destination"])
   if path.startswith(src+"/"):
    tail=path[len(src)+1:]
    if u["method"].startswith("root sealed-unit") and "/" in tail:continue
    actual=dst+"/"+tail;break
 b=gitraw(actual,rev)
 with (REC/"READS.jsonl").open("a") as f:f.write(json.dumps({"requested_path":path,"path":actual,"revision":rev,"sha256":hashlib.sha256(b).hexdigest(),"bytes":len(b)})+"\n")
 return b
def js(path,rev=REV):return json.loads(get(path,rev))
def packet(name):return js(P+"/_run_records/"+name)
def save(name,data):(REC/name).write_text(json.dumps(data,indent=2)+"\n")
@functools.lru_cache(None)
def archive(tier):
 name="records.tar.gz" if tier==1 else "delta_records.tar.gz"
 with tarfile.open(fileobj=io.BytesIO(get(R+"/MEASUREMENTS/W1_T"+str(tier)+"/_run_records/"+name)),mode="r:gz") as t:return {m.name:t.extractfile(m).read() for m in t.getmembers() if m.isfile()}

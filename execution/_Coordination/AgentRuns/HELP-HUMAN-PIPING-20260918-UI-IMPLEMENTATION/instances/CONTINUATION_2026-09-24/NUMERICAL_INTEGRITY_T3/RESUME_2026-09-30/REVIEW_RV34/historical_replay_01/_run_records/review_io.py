import json, pathlib, hashlib, subprocess, os, sys
REC=pathlib.Path(__file__).resolve().parent
ROOT=REC.parents[12] if False else pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c")
R="projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30"
T3=R.rsplit("/",1)[0]
REV="40179f1da5961cfef7edeaf7be4eb675379f79b5"
CACHE=pathlib.Path(os.environ.get("RV34_CACHE", str(ROOT.parent/"scratch/rv34_historical_replay_01")))
PKT=CACHE/"snapshot"/R/"I25/historical_replay_01/_run_records"
def get(path,rev=REV):
    path=str(path); dest=CACHE/"inputs"/rev/path
    if not dest.exists():
        b=subprocess.check_output(["git","show",rev+":"+path],cwd=ROOT,env=dict(os.environ,GIT_OPTIONAL_LOCKS="0"))
        dest.parent.mkdir(parents=True,exist_ok=True); dest.write_bytes(b)
        with (REC/"INPUT_READS.jsonl").open("a") as h: h.write(json.dumps({"revision":rev,"path":path,"sha256":hashlib.sha256(b).hexdigest(),"bytes":len(b)})+"\n")
    return dest.read_bytes()
def jget(path,rev=REV):return json.loads(get(path,rev))
def packet(name):return json.loads((PKT/name).read_bytes())
def save(name,obj): (REC/name).write_text(json.dumps(obj,indent=2)+"\n")
def rows(path):return [json.loads(s) for s in get(path).splitlines() if s.strip()]

# Rehydrate the immutable author packet when repeating in fresh owned scratch.
if not PKT.exists():
    packet_root=R+"/I25/historical_replay_01"
    files=subprocess.check_output(["git","ls-tree","-r","--name-only",REV,packet_root],cwd=ROOT,env=dict(os.environ,GIT_OPTIONAL_LOCKS="0")).decode().splitlines()
    for name in files:
        dest=CACHE/"snapshot"/name
        dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(get(name))

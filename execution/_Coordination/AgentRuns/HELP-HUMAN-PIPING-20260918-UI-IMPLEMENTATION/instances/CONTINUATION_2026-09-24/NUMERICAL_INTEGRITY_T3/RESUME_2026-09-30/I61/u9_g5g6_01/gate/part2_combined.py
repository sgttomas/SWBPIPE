#!/usr/bin/env python3
"""Combine the part-2 executions: per run and execution, wall seconds, timed_out, exit, outcome, blocking codes, full-envelope
sha256 and foreign-job samples (overlap.py's rule). Verdict: every execution <= 1800 s and not timed out; base and candidate
full envelopes identical per run; and the clean executions (0 foreign samples) listed per run."""
import json,hashlib,os,re,sys,datetime as dt
sys.path.insert(0,"scripts")
import compare as C
execs=sys.argv[1:]
def overlaps(d):
    samples=[]
    for line in open(d+"/host_watch.tsv"):
        p=line.rstrip("\n").replace("\\t","\t").split("\t")
        samples.append((dt.datetime.strptime(p[0],"%Y-%m-%dT%H:%M:%SZ"),int(p[1].strip() or 0),p[3] if len(p)>3 else ""))
    out={}
    for line in open(d+"/driver.log"):
        m=re.match(r"(\S+) (base|cand) (\S+) ok ([\d.]+)",line)
        if m:
            end=dt.datetime.strptime(m.group(1),"%Y-%m-%dT%H:%M:%SZ"); st=end-dt.timedelta(seconds=float(m.group(4)))
            ins=[s for s in samples if st<=s[0]<=end]
            # the gate's own exiting probe, shown as "(t3_p1_probe)", is not a foreign job (run 2's detector counted it)
            out[(m.group(2),m.group(3))]=sum(1 for s in ins if s[1]>0 and s[2].strip() not in ("","(t3_p1_probe)") and not re.fullmatch(r"\s*\d+ \(t3_p1_probe\)\s*",s[2]))
    return out
ok=True; clean={}
print("execution\tside\trun\twall_s\ttimed_out\texit\toutcome\tblocking\tfull_sha256\tforeign_samples")
for d in execs:
    ov=overlaps(d); shas={}
    for side in ("base","cand"):
        for line in open(f"{d}/{side}/runs.jsonl"):
            r=json.loads(line); key=f"{r['case']}__dense_scrutiny__{r['entry']}"
            out,_=C.classify(r)
            full=json.load(open(f"{d}/{side}/full/{key}.json"))
            blocking=",".join(x["code"] for x in full.get("diagnostics",[]) if x.get("severity")=="blocking")
            fs=hashlib.sha256(open(f"{d}/{side}/full/{key}.json","rb").read()).hexdigest()
            shas[(side,key)]=fs
            n=ov[(side,key)]
            print(f"{os.path.basename(d)}\t{side}\t{key}\t{r['wall_seconds']}\t{r['timed_out']}\t{r['exit_code']}\t{out}\t{blocking}\t{fs}\t{n}")
            ok&= (not r["timed_out"]) and r["exit_code"]==0 and r["wall_seconds"]<=1800
            if n==0: clean.setdefault((side,key),[]).append((os.path.basename(d),r["wall_seconds"]))
    for key in {k for _,k in shas}:
        ok&= shas[("base",key)]==shas[("cand",key)]
print()
for side in ("base","cand"):
    for case in ("RF-LARGE-CHAIN-n01000-ROT","RF-LARGE-TREE-n01000-AX"):
        for entry in ("captured","typed"):
            key=f"{case}__dense_scrutiny__{entry}"
            print(f"clean executions\t{side}\t{key}\t{clean.get((side,key),[])}")
print("every run has a clean execution:", all(clean.get((s,f"{c}__dense_scrutiny__{e}")) for s in ("base","cand") for c in ("RF-LARGE-CHAIN-n01000-ROT","RF-LARGE-TREE-n01000-AX") for e in ("captured","typed")))
print("RESULT", "PASS" if ok else "FAIL", "(every execution ended within 1800 s with exit 0, and base and candidate full envelopes are identical per run)")

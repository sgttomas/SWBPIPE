"""Replay/check sealed exact source-span operations. No Git or product tests."""
from pathlib import Path
import argparse, hashlib, json, re

HERE=Path(__file__).resolve().parent
ROOT=next(p for p in HERE.parents if (p/"agents/AGENT_TASK.md").exists())
def sha(s):return hashlib.sha256(s.encode()).hexdigest()
p=argparse.ArgumentParser();g=p.add_mutually_exclusive_group(required=True)
g.add_argument("--apply",action="store_true");g.add_argument("--check",action="store_true");g.add_argument("--check-spans",action="store_true")
args=p.parse_args();m=json.loads((HERE/"operations.json").read_text());counts={}
for f in m["files"]:
    assert f["deliverable_id"]!="DEL-01-01" and f["path"].endswith("/_STATUS.md")
    original=(HERE/f["before_snapshot"]).read_text();assert sha(original)==f["before_file_sha256"]
    ops=[o for o in m["operations"] if o["path"]==f["path"]]
    result=original
    for o in reversed(ops):
        assert original[o["source_start"]:o["source_end"]]==o["before_text"]
        assert sha(o["before_text"])==o["before_body_sha256"] and sha(o["after_text"])==o["after_body_sha256"]
        result=result[:o["source_start"]]+o["after_text"]+result[o["source_end"]:]
        counts[o["operation_kind"]]=counts.get(o["operation_kind"],0)+1
    assert sha(result)==f["after_file_sha256"]
    target=ROOT/f["path"];current=target.read_text()
    if args.apply and current!=result:
        assert sha(current)==f["before_file_sha256"],f"Unexpected source drift: {target}"
        target.write_text(result);current=result
    if args.check_spans:
        for o in ops:assert o["after_text"] in current, (o["source_key"],"changed repaired span")
    else:assert sha(current)==f["after_file_sha256"],f"Postimage drift: {target}"
    assert re.search(r"(?m)^\*\*Current State:\*\* .+$",current)[0]==f["lifecycle"]
print(json.dumps({"result":"PASS","mode":"apply" if args.apply else "check-spans" if args.check_spans else "check","files":len(m["files"]),"operations":len(m["operations"]),"by_kind":counts,"lifecycle_changes":0},indent=2))

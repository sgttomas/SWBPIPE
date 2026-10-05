"""RV90: apply every corpus entry with RV90's own edit/rehash code (07e format rule); write JSONL."""
import sys, json, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from helpers import apply
from copy import deepcopy
corpus_path, out = sys.argv[1], sys.argv[2]
c = json.load(open(corpus_path)); cases = {x["id"]: x for x in c["cases"]}
with open(out, "w") as f:
    for x in c["cases"]:
        f.write(json.dumps({"kind": "cases", "id": x["id"], "expected": None, "source": x["source"], "invocation": x["invocation"]}) + "\n")
    for kind in ("mutations", "must_pass"):
        for e in c[kind]:
            s, i = apply(cases[e["base"]], e)
            f.write(json.dumps({"kind": kind, "id": e["id"], "expected": e.get("expected"), "expected_by_reader": e.get("expected_by_reader"), "source": s, "invocation": i}) + "\n")
print("applied", out)

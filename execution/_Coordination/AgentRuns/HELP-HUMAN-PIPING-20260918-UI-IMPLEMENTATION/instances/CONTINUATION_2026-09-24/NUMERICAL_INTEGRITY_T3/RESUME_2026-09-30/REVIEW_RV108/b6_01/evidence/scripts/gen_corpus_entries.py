"""RV108: the shared corpus's entries as concrete (source, invocation) pairs, by the shared format
(edits, invocation_edits with the invocation digest, rehash 'all', after_rehash), implemented here."""
import json, sys
from copy import deepcopy
from pathlib import Path
sys.path.insert(0, sys.argv[3]); import gen_probes_lib as L  # noqa
PROOT = Path(sys.argv[1]); OUT = Path(sys.argv[2])
sys.path.insert(0, str(PROOT))
from core.analysis_runs import retained_precision as rp
corpus = json.loads((PROOT / "fixtures/results/retained_precision_cases.json").read_text())
cases = {c["id"]: c for c in corpus["cases"]}
def edits(v, es):
    for e in es or []:
        (L.delp(v, e["path"]) if e["op"] == "remove" else L.setp(v, e["path"], e["value"]))
out = []
for c in corpus["cases"]:
    out.append({"id": f"case:{c['id']}", "family": "corpus_case", "base": c["id"], "source": c["source"], "invocation": c["invocation"]})
for kind, entries in (("mutation", corpus["mutations"]), ("must_pass", corpus["must_pass"])):
    for i, m in enumerate(entries):
        base = cases[m["base"]]; s, inv = deepcopy(base["source"]), deepcopy(base["invocation"])
        edits(s, m["edits"]); edits(inv, m.get("invocation_edits"))
        if m.get("invocation_edits"):
            s["retained_precision"]["body"]["invocation"]["value"] = rp._hash("source_blocks_invocation_v1", inv)
        assert m["rehash"] == "all"
        L.rehash(s); edits(s, m.get("after_rehash"))
        out.append({"id": f"{kind}:{i}:{m['id']}", "family": f"corpus_{kind}", "base": m["base"], "source": s, "invocation": inv,
                    "expected": m.get("expected"), "expected_by_reader": m.get("expected_by_reader"), "expected_eligibility": m.get("expected_eligibility")})
OUT.write_text(json.dumps(out)); print(len(out))

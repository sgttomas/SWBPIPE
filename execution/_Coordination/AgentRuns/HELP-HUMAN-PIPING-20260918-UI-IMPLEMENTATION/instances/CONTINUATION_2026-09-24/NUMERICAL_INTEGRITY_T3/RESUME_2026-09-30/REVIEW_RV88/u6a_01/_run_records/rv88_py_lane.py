"""RV88 (U6a review): D-U6-1, independent outcome capture for one lane.

Usage: python rv88_py_lane.py <lane P root> <out.json> [<slice dir>]
Records, for each of the 07f corpus's 320 entries, the public-entry and draft
outcomes; the public outcomes again with the eligibility flag forced True
(in-process only, nothing written); and the milestone receipts (and, when a
slice dir is given, the Rust derivative's receipt reattached to the source).
"""
import json
import sys
from copy import deepcopy
from pathlib import Path

root = Path(sys.argv[1]).resolve()
out = Path(sys.argv[2])
slice_dir = Path(sys.argv[3]) if len(sys.argv) > 3 else None
sys.path.insert(0, str(root))
sys.path.insert(0, str(root / "tests"))
from core.analysis_runs import retained_precision as rp  # noqa: E402
import test_retained_precision_contract as t  # noqa: E402  (corpus application helper only)


def outcome(fn, source, invocation):
    try:
        r = fn(deepcopy(source), deepcopy(invocation))
        return {"kind": "pass", "result": r}
    except rp.RetainedPrecisionError as e:
        return {"kind": "refuse", "gate": e.gate, "code": e.code, "detail": e.detail}


def entries():
    data = t.corpus()
    for f in data["cases"]:
        yield f["id"], "case", None, deepcopy(f["source"]), deepcopy(f["invocation"])
        yield f["id"] + ":no-invocation", "case_noinv", None, deepcopy(f["source"]), None
    for kind in ("mutations", "must_pass"):
        for e in data.get(kind, []):
            f = next(x for x in data["cases"] if x["id"] == e["base"])
            s, i = t.apply_entry(f, e)
            yield e["id"], kind, e.get("expected"), s, i


rec = {"flag": rp._IMPLEMENTATION_COMPLETE, "entries": [], "milestone": {}}
for label, kind, expected, s, i in entries():
    pub = outcome(rp.validate_retained_precision, s, i)
    draft = outcome(rp._validate_draft, s, i)
    rec["entries"].append({"id": label, "kind": kind, "expected": expected, "public": pub, "draft": draft})

# The flag gates eligibility only: force it True in-process and recapture.
saved = rp._IMPLEMENTATION_COMPLETE
rp._IMPLEMENTATION_COMPLETE = True
try:
    for n, (label, kind, expected, s, i) in enumerate(entries()):
        rec["entries"][n]["public_flag_true"] = outcome(rp.validate_retained_precision, s, i)
finally:
    rp._IMPLEMENTATION_COMPLETE = saved

for mode in ("sparse_interactive", "dense_scrutiny"):
    p = root / f"fixtures/results/retained_precision_milestone_successor_{mode}.json"
    if not p.exists():
        p = Path(sys.argv[4]) / p.name if len(sys.argv) > 4 else p
    if not p.exists():
        continue
    doc = json.loads(p.read_bytes())
    m = {
        "public_inv": outcome(rp.validate_retained_precision, doc["source"], doc["invocation"]),
        "public_noinv": outcome(rp.validate_retained_precision, doc["source"], None),
        "draft_inv": outcome(rp._validate_draft, doc["source"], doc["invocation"]),
    }
    if slice_dir is not None:
        rp_path = slice_dir / f"cand_{mode}_receipt_from_derivative.json"
        if rp_path.exists():
            back = deepcopy(doc["source"])
            back["retained_precision"] = json.loads(rp_path.read_text())
            m["back_out_inv"] = outcome(rp.validate_retained_precision, back, doc["invocation"])
            m["back_out_equal"] = m["back_out_inv"] == m["public_inv"]
            m["receipt_equal"] = back["retained_precision"] == doc["source"]["retained_precision"]
    rec["milestone"][mode] = m

out.write_text(json.dumps(rec, sort_keys=True))
print(len(rec["entries"]), "entries;", "flag", rec["flag"])

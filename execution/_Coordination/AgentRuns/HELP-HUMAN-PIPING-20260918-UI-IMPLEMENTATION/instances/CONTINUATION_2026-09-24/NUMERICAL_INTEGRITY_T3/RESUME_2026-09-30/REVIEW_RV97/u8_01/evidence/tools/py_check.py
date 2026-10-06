"""RV97: the Python reader (validate_retained_precision) on the saved documents, plus my body
checks for L = 0 and the two-body base. Usage: py_check.py READER_P_ROOT OUT_DIR"""
import json, struct, sys
from collections import Counter
from copy import deepcopy
from pathlib import Path
root, out = sys.argv[1], Path(sys.argv[2])
sys.path.insert(0, root)
from core.analysis_runs import retained_precision as rp
bits = lambda v: struct.unpack(">Q", struct.pack(">d", v))[0]

def read(doc, inv):
    try:
        r = rp.validate_retained_precision(deepcopy(doc["source"]), deepcopy(inv))
        return {"ok": True, "bound": r["invocation_bound"], "eligible": r["numerical_eligible"], "standing": r["standing"],
                "rows": len(r["classifications"]), "classes": dict(sorted(Counter(c["class"] for c in r["classifications"]).items())),
                "publication_sha256": r.get("publication_sha256")}, r
    except rp.RetainedPrecisionError as e:
        return {"ok": False, "gate": e.gate, "code": e.code}, None

files = sorted(out.glob("*.json"))
docs = {f.name: json.loads(f.read_text()) for f in files}
for name, doc in docs.items():
    for label, inv in (("bound", doc["invocation"]), ("unbound", None)):
        print("PY", name, label, json.dumps(read(doc, inv)[0], sort_keys=True))

def bodies(name, milestone_name, extra_refs):
    doc = docs[name]; src = doc["source"]; rows = src["results"]
    _, r = read(doc, doc["invocation"])
    cls = {c["result_id"]: c for c in r["classifications"]}
    body = src["retained_precision"]["body"]
    print("BODY", name, "membership", json.dumps(body["sources"][0]["body_membership"]))
    print("BODY", name, "coverage", json.dumps(body["product_attempts"][0]["proof"]["summary_coverage"]))
    print("BODY", name, "scales", json.dumps(body["cases"][0]["selection"]["body_scales"]))
    print("BODY", name, "stop_rule bodies", [s.get("body") for s in body["cases"][0]["selection"].get("stop_rule", [])])
    b1 = [x for x in rows if x["entity_ref"] in extra_refs]
    tally = Counter()
    bad = []
    for x in b1:
        c = cls[x["id"]]
        zero = bits(x["value"]) == 0
        if c["class"] == "input_derived": tally["input_derived"] += 1
        elif c["class"] == "absolute_verified" and c["normalized_bits"] == "0000000000000000" and c["bound_bits"] == "0000000000000000": tally["exact_zero_absolute"] += 1
        else: tally["other:" + c["class"]] += 1; bad.append(x["id"])
        if not zero: bad.append(x["id"] + " nonzero")
    print("BODY", name, "body1 rows", len(b1), dict(tally), "bad", bad)
    print("BODY", name, "body1 ids", [ (x["id"], x["kind"]) for x in b1])
    parity = [x for x in rows if x["kind"] == "sparse_live_path_dense_parity_relative_delta" or "parity" in x["id"]]
    print("BODY", name, "parity rows", [(x["id"], x["kind"]) for x in parity])
    print("BODY", name, "range_scaling", json.dumps(src.get("numerical_integrity", {}).get("range_scaling")) if isinstance(src.get("numerical_integrity"), dict) else None)
    if milestone_name:
        ms = docs[milestone_name]["source"]["results"]
        mids = {x["id"]: x for x in ms}
        present = [x for x in rows if x["id"] in mids]
        same = sum(1 for x in present if bits(x["value"]) == bits(mids[x["id"]]["value"]))
        print("BODY", name, "body0 milestone rows", len(mids), "present", len(present), "bit-identical", same,
              "extra", len(rows) - len(present), "row order equal", [x["id"] for x in present] == [x["id"] for x in ms])
        _, mr = read(docs[milestone_name], docs[milestone_name]["invocation"])
        mcls = {c["result_id"]: c for c in mr["classifications"]}
        diffcls = [i for i in mids if (cls[i]["class"], cls[i]["bound_bits"], cls[i]["normalized_bits"], cls[i]["scale_bits"]) != (mcls[i]["class"], mcls[i]["bound_bits"], mcls[i]["normalized_bits"], mcls[i]["scale_bits"])]
        print("BODY", name, "body0 class/bound/normalized/scale differences vs milestone", diffcls)

for mode in ("sparse_interactive", "dense_scrutiny"):
    bodies(f"l0_document_{mode}.json", f"milestone_document_{mode}.json", ("N2", "rigid:N2"))
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = docs[f"stages_two_body_a_{mode}.json"]
    refs = ("node:section-a", "node:section-b", "pipe:source-section", "support:section-a")
    b1 = [x for x in doc["source"]["results"] if x["entity_ref"] in refs or any(r in x["id"] for r in refs)]
    print("TWOBODY", mode, "rows", len(doc["source"]["results"]), "body1-named rows", len(b1), "all zero", all(bits(x["value"]) == 0 for x in b1))
    body = doc["source"]["retained_precision"]["body"]
    print("TWOBODY", mode, "coverage", json.dumps(body["product_attempts"][0]["proof"]["summary_coverage"]), "scales b1", json.dumps(body["cases"][0]["selection"]["body_scales"][1]))
    parity = [x["id"] for x in doc["source"]["results"] if x["kind"] == "sparse_live_path_dense_parity_relative_delta"]
    print("TWOBODY", mode, "parity rows", parity)

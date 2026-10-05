"""RV90 probes: the real milestone receipts (unmodified and resealed F5 variants) and reader-parity probes."""
import sys, json, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from helpers import apply
from copy import deepcopy
P = sys.argv[1]; out = sys.argv[2]
rows = []
REFS = ["retained_precision", "body", "ordinary_attempts", 0, "diagnostic_refs"]
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.load(open(f"{P}/fixtures/results/retained_precision_milestone_successor_{mode}.json"))
    base = {"source": doc["source"], "invocation": doc["invocation"]}
    src = doc["source"]; cid = src["retained_precision"]["body"]["cases"][0]["basis_ref"]["ref_id"]
    ds = src["diagnostics"]
    names = lambda d: isinstance(d.get("affected_refs"), list) and cid in d["affected_refs"]
    ret = lambda d: str(d["code"]).startswith("RETAINED_PRECISION_")
    exact = [d["id"] for d in ds if names(d) and not ret(d)]
    assert src["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"] == exact, mode
    rows.append(("milestone", mode + ":unmodified", deepcopy(src), deepcopy(doc["invocation"]), "pass"))
    s, i = apply(base, {"edits": [], "rehash": "all"}); assert s == src, mode + " does not reseal to itself"
    rows.append(("milestone", mode + ":resealed", s, i, "pass"))
    variants = {
        "m09_all_naming": [d["id"] for d in ds if names(d)],
        "m10_all_non_retained": [d["id"] for d in ds if not ret(d)],
        "truncated_last": exact[:-1],
        "truncated_first": exact[1:],
        "swapped_first_two": [exact[1], exact[0]] + exact[2:],
        "duplicate_last": exact + [exact[-1]],
        "plus_invocation_level": exact + [d["id"] for d in ds if d.get("affected_refs") is None],
        "empty": [],
    }
    for k, v in variants.items():
        s, i = apply(base, {"edits": [{"path": REFS, "op": "set", "value": v}], "rehash": "all"})
        rows.append(("milestone", mode + ":" + k, s, i, ["G5", "RETAINED_PRECISION_ATTEMPT_MISMATCH"]))
    s, i = apply(base, {"edits": [{"path": ["results", 0, "recovery_method"], "op": "set", "value": "other"}], "rehash": "all"})
    rows.append(("milestone", mode + ":m20_row_method_other", s, i, ["G6", "RETAINED_PRECISION_ROW_METHOD_MISMATCH"]))
# parity probes on the 07g corpus base ordinary_prepared_synthetic
c = json.load(open(f"{P}/fixtures/results/retained_precision_cases.json")); base = next(x for x in c["cases"] if x["id"] == "ordinary_prepared_synthetic")
ds = base["source"]["diagnostics"]; k = next(j for j, d in enumerate(ds) if d["code"] == "LOAD_CATEGORY_PREVIEW_MAPPED")
cid = "case:six-component-load"; exact = base["source"]["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"]
s, i = apply(base, {"edits": [{"path": ["diagnostics", k, "affected_refs"], "op": "set", "value": cid}], "rehash": "all"})
rows.append(("probe", "affected_refs_string_listed", s, i, None))
s, i = apply(base, {"edits": [{"path": ["diagnostics", k, "affected_refs"], "op": "set", "value": cid}, {"path": REFS, "op": "set", "value": [x for x in exact if x != ds[k]["id"]]}], "rehash": "all"})
rows.append(("probe", "affected_refs_string_unlisted", s, i, None))
s, i = apply(base, {"edits": [{"path": REFS, "op": "set", "value": exact[:-1]}], "rehash": "all"})
rows.append(("probe", "corpus_base_truncated_last", s, i, ["G5", "RETAINED_PRECISION_ATTEMPT_MISMATCH"]))
two = next(x for x in c["cases"] if x["id"] == "two_case_synthetic")
e1 = two["source"]["retained_precision"]["body"]["ordinary_attempts"][1]["diagnostic_refs"]
s, i = apply(two, {"edits": [{"path": ["retained_precision", "body", "ordinary_attempts", 1, "diagnostic_refs"], "op": "set", "value": [e1[1], e1[0]] + e1[2:]}], "rehash": "all"})
rows.append(("probe", "second_case_swapped", s, i, ["G5", "RETAINED_PRECISION_ATTEMPT_MISMATCH"]))
s, i = apply(two, {"edits": [{"path": ["retained_precision", "body", "ordinary_attempts", 1, "diagnostic_refs"], "op": "set", "value": e1[-1:]}], "rehash": "all"})
rows.append(("probe", "second_case_relaxed_form", s, i, ["G5", "RETAINED_PRECISION_ATTEMPT_MISMATCH"]))
with open(out, "w") as f:
    for kind, id_, s, i, exp in rows:
        f.write(json.dumps({"kind": kind, "id": id_, "expected": exp, "source": s, "invocation": i}) + "\n")
print("probes", len(rows))

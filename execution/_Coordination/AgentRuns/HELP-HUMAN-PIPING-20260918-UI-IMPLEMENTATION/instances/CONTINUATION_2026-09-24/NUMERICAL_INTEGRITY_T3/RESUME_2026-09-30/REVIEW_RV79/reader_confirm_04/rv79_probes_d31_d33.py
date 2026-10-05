"""RV79 confirmation 04: own probes for D31 (model schema_version) and D33 (estimate kind)."""
import json, sys, traceback
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import apply_entry, corpus
C = corpus(); BASE = {f["id"]: f for f in C["cases"]}
B = ["retained_precision", "body"]
def run(base, edits, inv_edits=None):
    entry = {"base": base, "edits": edits, "rehash": "all"}
    if inv_edits: entry["invocation_edits"] = inv_edits
    src, inv = apply_entry(BASE[base], entry)
    try:
        r = rp._validate_draft(src, inv); return "PASS" if r["classifications"] == BASE[base]["expected_classifications"] else "PASS (classifications differ)"
    except rp.RetainedPrecisionError as e:
        return f"{e.gate} {e.code}"
out = []
def probe(pid, base, edits, inv, ruled, basis):
    got = run(base, edits, inv); out.append({"id": pid, "python": got, "ruled": ruled, "basis": basis, "agrees": got == ruled})
V = lambda v: [{"path": ["request", "model", "schema_version"], "op": "set", "value": v}]
for v, r in (("0.1.0", "PASS"), ("0.2.0", "PASS"), ("0.3.0", "PASS"), ("0.4.0", "G8 RETAINED_PRECISION_INVOCATION_MISMATCH"), ("0.1", "G8 RETAINED_PRECISION_INVOCATION_MISMATCH")):
    probe(f"D31_model_schema_version_{v}", "ordinary_prepared_synthetic", [], V(v), r, "D31")
LAD = "p512_ladder_synthetic"; lb = BASE[LAD]["source"]["retained_precision"]["body"]; RUN = B + ["cases", 0, "run"]
base_reason = lb["cases"][0]["run"]["records"][0]["outcome"]["reason"]
force_row = next(r for r in lb["sources"][0]["layout"] if r["kind"] == "force")
for kind, row, ruled in (("translation", None, "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH"), ("force", force_row, None)):
    reason = dict(deepcopy(base_reason), tag="verification_estimate")
    if row is not None: reason.update(quantity=row["quantity"], body=row["body"], kind=row["kind"])
    got = run(LAD, [{"path": RUN + ["records", 0, "outcome", "reason"], "op": "set", "value": reason}, {"path": RUN + ["attempts", 0, "outcome", "reason"], "op": "set", "value": reason}])
    out.append({"id": f"D33_verification_estimate_{kind}_row", "python": got, "ruled": ruled or "not G5 ATTEMPT at the D33/D28 checks (a force row is admitted)", "basis": "D33/D28", "agrees": (got == ruled) if ruled else True})
for p in out: print(json.dumps(p))

"""RV79 confirmation 03: new probes for D21 (summary indicator), D28, D23, D25 and D29 on READER a894d9d0ba."""
import json, sys, traceback
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import apply_entry, corpus
C = corpus(); BASE = {f["id"]: f for f in C["cases"]}
B = ["retained_precision", "body"]
def S(path, value): return {"path": B + path, "op": "set", "value": value}
def body(base): return BASE[base]["source"]["retained_precision"]["body"]
def run(base, edits):
    source, invocation = apply_entry(BASE[base], {"base": base, "edits": edits, "rehash": "all"})
    try:
        r = rp._validate_draft(source, invocation)
        same = r["classifications"] == BASE[base]["expected_classifications"]
        return "PASS" + ("" if same else " (classifications differ)"), None
    except rp.RetainedPrecisionError as e:
        return f"{e.gate} {e.code}", [f.lineno for f in traceback.extract_tb(e.__traceback__) if f.filename.endswith("retained_precision.py")]
out = []
def probe(pid, base, edits, ruled, decision):
    got, lines = run(base, edits)
    out.append({"id": pid, "base": base, "python": got, "raise_lines": lines, "ruled": ruled, "decision": decision, "agrees": got == ruled})
O, LAD, V = "ordinary_prepared_synthetic", "p512_ladder_synthetic", "verification_failure_skip_synthetic"
G5A = "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH"; G3C = "G3 RETAINED_PRECISION_COVERAGE_MISMATCH"
RUN = ["cases", 0, "run"]
probe("D21_summary_on_escalating_failed_verification", V, [S(RUN + ["records", 1, "verification"], deepcopy(body(V)["cases"][0]["run"]["records"][3]["verification"]))], G5A, "D21 third indicator")
for tag in ("verification_estimate", "charge"):
    rq = deepcopy(body(LAD)["cases"][0]["run"]["records"][0]["outcome"]); rq["reason"]["tag"] = tag; rq["reason"]["quantity"]["dof"]["node"] = 99
    probe(f"D28_{tag}_foreign_quantity", LAD, [S(RUN + ["records", 0, "outcome"], rq), S(RUN + ["attempts", 0, "outcome"], rq)], G5A, "D28")
rq = deepcopy(body(LAD)["cases"][0]["run"]["records"][0]["outcome"]); rq["reason"]["body"] = 1
probe("D28_stop_rule_other_body", LAD, [S(RUN + ["records", 0, "outcome"], rq), S(RUN + ["attempts", 0, "outcome"], rq)], G5A, "D5d/D28 same body")
rq = deepcopy(body(LAD)["cases"][0]["run"]["records"][0]["outcome"]); rq["reason"]["kind"] = "rotation"
probe("D28_stop_rule_other_kind", LAD, [S(RUN + ["records", 0, "outcome"], rq), S(RUN + ["attempts", 0, "outcome"], rq)], G5A, "D5d/D28 same kind")
mm = deepcopy(body(O)["sources"][0]["id_maps"]["members"]); mm[0]["kernel_member"] = 5
probe("D23_kernel_member_noncanonical", O, [S(["sources", 0, "id_maps", "members"], mm)], G3C, "D23")
probe("D25_integral_float_counter_accepted", O, [S(["work", "charged"], float(body(O)["work"]["charged"]))], "PASS", "D25")
probe("D25_nonintegral_counter_rejected", O, [S(["work", "charged"], body(O)["work"]["charged"] + 0.5)], "G2 RETAINED_PRECISION_ENCODING_MISMATCH", "D25 (integrality stays)")
probe("D29_empty_inventory_only", O, [S(["sources", 0, "body_membership"], [])], G3C, "D29")
for p in out: print(json.dumps(p))

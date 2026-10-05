"""RV79 confirmation 03: D25 (numbers are values) against Python's remaining `type(x) is int` guards.

Under D25, 0.0 and 0 are the same JSON number with the same canonical hash, so the receipt hash
is unchanged when an index is written as an integral float. Each probe edits the parsed base,
re-hashes the receipt itself (the shared harness indexes lists with the reference, so it cannot
express a float index), and validates."""
import json, sys, traceback
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import corpus
C = corpus(); BASE = {f["id"]: f for f in C["cases"]}
def finish(src):
    body = src["retained_precision"]["body"]
    src["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
    return src
def outcome(src, inv):
    try:
        r = rp._validate_draft(src, inv); return "PASS (standing %s)" % r["standing"], None
    except rp.RetainedPrecisionError as e:
        return f"{e.gate} {e.code}", [f.lineno for f in traceback.extract_tb(e.__traceback__) if f.filename.endswith("retained_precision.py")]
out = []
def probe(pid, base, mutate, ruled, basis, invocation=True):
    f = BASE[base]; src = deepcopy(f["source"]); mutate(src["retained_precision"]["body"]); finish(src)
    got, lines = outcome(src, deepcopy(f["invocation"]) if invocation else None)
    out.append({"id": pid, "base": base, "invocation": invocation, "python": got, "raise_lines": lines, "ruled": ruled, "basis": basis, "agrees": got == ruled})
O, TC, F2 = "ordinary_prepared_synthetic", "two_case_synthetic", "two_case_facade_after_certificate_synthetic"
FORGED = "0" * 64
G1R = "G1 RETAINED_PRECISION_RECEIPT_MISMATCH"
# control: integer index, forged source identity -> G1
def m_ctrl(b): b["cases"][0]["source_identity_sha256"] = FORGED
probe("control_forged_source_identity_int_ref", O, m_ctrl, G1R, "G1 source identity (C1 G1 row)")
def m_float(b): b["cases"][0]["source_ref"] = 0.0; b["cases"][0]["source_identity_sha256"] = FORGED
probe("forged_source_identity_float_ref", O, m_float, G1R, "D25 + G1: 0.0 is the index 0, so the identity must be checked")
def m_float_ok(b): b["cases"][0]["source_ref"] = 0.0
probe("float_ref_alone", O, m_float_ok, "PASS (standing needs_recompute)", "D25: same value, same result")
def m_prep(b):
    # forge the preparation hash, then re-derive case 1's source identity (it binds the preparation) so that
    # only the preparation-hash integrity check can catch the forgery
    b["sources"][1]["preparation"]["attempt_ref"] = 1.0; b["sources"][1]["preparation"]["sha256"] = FORGED
    b["cases"][1]["source_identity_sha256"] = rp._source_hash(b["sources"][1])
def m_prep_int(b):
    b["sources"][1]["preparation"]["sha256"] = FORGED
    b["cases"][1]["source_identity_sha256"] = rp._source_hash(b["sources"][1])
probe("control_forged_preparation_hash_int_ref", TC, m_prep_int, G1R, "G1 preparation hash (C3 G1 row)")
probe("forged_preparation_hash_float_ref", TC, m_prep, G1R, "D25 + G1 preparation hash")
probe("forged_preparation_hash_float_ref_no_invocation", TC, m_prep, G1R, "D25 + G1 preparation hash (no G8 backstop)", invocation=False)
def m_d23(b):
    b["product_attempts"][0]["source_ref"] = 0.0; b["sources"][0]["id_maps"]["members"][0]["kernel_member"] = 5
    b["cases"][0]["source_identity_sha256"] = rp._source_hash(b["sources"][0])
probe("d23_noncanonical_kernel_member_float_source_ref", O, m_d23, "G3 RETAINED_PRECISION_COVERAGE_MISMATCH", "D25 + D23 (G3)")
def m_cov(b):
    a = b["product_attempts"][0]; a["source_ref"] = 0.0; cov = a["proof"]["summary_coverage"]; a["proof"]["summary_coverage"] = list(reversed(cov)) if len(cov) > 1 else [dict(cov[0], body=1)]
probe("coverage_roster_foreign_body_float_source_ref", O, m_cov, "G3 RETAINED_PRECISION_COVERAGE_MISMATCH", "D25 + I57 G3 roster")
for p in out: print(json.dumps(p))

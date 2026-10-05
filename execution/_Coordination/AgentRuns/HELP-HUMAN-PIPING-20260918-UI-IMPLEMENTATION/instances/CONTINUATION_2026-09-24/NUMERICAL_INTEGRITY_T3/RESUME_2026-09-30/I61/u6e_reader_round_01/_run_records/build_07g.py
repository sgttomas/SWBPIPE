#!/usr/bin/env python3
"""I61 U6e: stage snapshot 07g from 07f (F5, RV79-N1, RV80-N2).

Usage: build_07g.py P_ROOT OUT_CORPUS
- F5 (D-U6-7; A2): every base's ordinary diagnostic_refs become A2's exact list (the
  diagnostics whose affected_refs name the case, once each, in envelope order, excluding
  RETAINED_PRECISION_*), and the base's receipt hash is recomputed; six new mutations pin
  the exact list (omission, order, a RETAINED_PRECISION_* entry (U1 M09), an invocation-level
  entry (U1 M10), another case's entry, and 07f's relaxed form).
- RV79-N1: the independent D37 table (native source, not reader code) as corpus data.
- RV80-N2: no corpus change (the scope pin is reader-local; see the note below).
Everything else in 07f is copied unchanged (asserted).
"""
import json, sys
from copy import deepcopy
from pathlib import Path

P, OUT = Path(sys.argv[1]), Path(sys.argv[2])
sys.path.insert(0, str(P))
from core.analysis_runs import retained_precision as rp  # the hashes only (C1 s3 domains)

SRC = P / "fixtures/results/retained_precision_cases.json"
raw = SRC.read_bytes()
data = json.loads(raw)
before = deepcopy(data)
assert (len(data["cases"]), len(data["mutations"]), len(data["must_pass"])) == (15, 268, 22), "07f"


def a2_list(source, case_id):
    return [d["id"] for d in source["diagnostics"]
            if case_id in (d.get("affected_refs") or []) and not str(d.get("code")).startswith("RETAINED_PRECISION_")]


# 1. F5: the bases carry A2's exact list; only the receipt hash depends on it.
repaired = []
for fixture in data["cases"]:
    source = fixture["source"]
    body = source["retained_precision"]["body"]
    old = [list(o["diagnostic_refs"]) for o in body["ordinary_attempts"]]
    for i, case in enumerate(body["cases"]):
        body["ordinary_attempts"][i]["diagnostic_refs"] = a2_list(source, case["basis_ref"]["ref_id"])
    assert source["retained_precision"]["body"]["publication_sha256"] == rp._hash(
        "retained_precision_publication_mp_v2", {k: v for k, v in source.items() if k != "retained_precision"}), fixture["id"]
    source["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
    repaired.append({"id": fixture["id"], "before": old, "after": [o["diagnostic_refs"] for o in body["ordinary_attempts"]]})

ATTEMPT = {"gate": "G5", "code": "RETAINED_PRECISION_ATTEMPT_MISMATCH"}
BASE = "ordinary_prepared_synthetic"
TWO = "two_case_synthetic"
REFS0 = ["retained_precision", "body", "ordinary_attempts", 0, "diagnostic_refs"]


def base_source(name):
    return next(f for f in data["cases"] if f["id"] == name)["source"]


one = base_source(BASE)
exact0 = a2_list(one, "case:six-component-load")
assert exact0[-1] == "diagnostic:numerical-integrity:case:six-component-load" and len(exact0) == 7
two = base_source(TWO)
exact_two = a2_list(two, "case:six-component-load")
other_case = "diagnostic:numerical-integrity:case:zero-load"
assert other_case in [d["id"] for d in two["diagnostics"]] and other_case not in exact_two


def mutation(mid, base, refs):
    return {"id": mid, "base": base, "edits": [{"path": REFS0, "op": "set", "value": refs}], "rehash": "all", "expected": ATTEMPT}


f5 = [
    mutation("f5_ordinary_refs_omit_naming_diagnostic", BASE, exact0[1:]),
    mutation("f5_ordinary_refs_out_of_envelope_order", BASE, [exact0[1], exact0[0]] + exact0[2:]),
    # U1 M09: a RETAINED_PRECISION_* diagnostic naming the case, at its envelope position.
    mutation("f5_ordinary_refs_list_retained_precision_m09", BASE, exact0 + ["diagnostic:retained:synthetic"]),
    # U1 M10: an invocation-level diagnostic (no affected_refs), at its envelope position.
    mutation("f5_ordinary_refs_list_invocation_level_m10", BASE, exact0 + ["diagnostic:physics:rule-inputs-missing"]),
    mutation("f5_ordinary_refs_list_other_case", TWO, exact_two + [other_case]),
    # 07f's relaxed D6a form (the integrity diagnostic only): refused under F5.
    mutation("f5_ordinary_refs_relaxed_d6a_form", BASE, [exact0[-1]]),
]
data["mutations"].extend(f5)

# 3. RV80-N2: no shared observable distinguishes the normalization's scope (RV80 found no
# consumer for which a row's 17.0 versus 17 changes an outcome), so the shared entry stays 07d's
# D32 must-pass `integral_float_integers_and_references` and the scope is pinned reader-locally.
# 2. RV79-N1: the independent D37 table.
STAGES = ["preparation", "native", "proof_start", "projection", "maxima", "values", "aliases", "certificate", "observables", "g5a"]
checks = ["--", "CC", "CF", "FC", "FF"]
records = []
for k in range(8):
    records += ["C" * k + "F" + "-" * (9 - k), "C" * k + "-" * (10 - k)]
records += ["C" * 7 + "F" + c for c in checks[1:]] + ["C" * 8 + c for c in checks]
records = sorted(set(records), key=lambda r: (r.replace("-", "~"),))
assert len(records) == 25
kinds = {
    "preparation": ["F---------"],
    "native": ["CF--------"],
    "capture": ["CF--------", "CC--------", "CCCCCCCC--", "CCCCCCCCCC"],
    "proof": ["CCF-------", "CCCF------", "CCCCCCCF--", "CCCCCCCFCC", "CCCCCCCFCF", "CCCCCCCFFC", "CCCCCCCFFF"],
    "values": ["CCCCCF----"],
    "abandoned": ["CCCCF-----", "CCCCCCF---", "CCCCCCC---"],
    "numeric": ["CCCCCCCCCC"],
    "observable": ["CCCCCCCCFC", "CCCCCCCCFF"],
    "g5a": ["CCCCCCCCCF"],
}
assert all(r in records for v in kinds.values() for r in v)
data["d37"] = {
    "basis": ("RV79-N1 (D36): D37's expected table, derived from the native sequence and C3, not from any "
              "reader. PP/core/product_physics/src/retained_product.rs at 844448112f: prepare_owned_case "
              "(enter Preparation :3313, completed :3426, fail_entered :3430); solve_native (enter Native :3453; "
              "a selected outcome completes :3466; a nonselected outcome after the Run is recorded is "
              "NativeUnavailable, the `native` kind; any earlier error is the `capture` kind (a); fail_entered :3467); "
              "freeze_candidate (capture (b) before ProofStart :3655-3660; ProofStart :3662-3664 and Projection "
              ":3666-3668 failures are `proof`; Maxima :3668-3671 `abandoned`; Values :3671-3676 `values`; Aliases "
              ":3676-3681 `abandoned`; bind_rows_view :3684-3685 `abandoned` with the certificate not entered; "
              "Certificate :3686-3709: a failed certificate is `proof`, with Observables and G5a both entered and "
              "checked when the verdict copy covers every row (:3696-3703); after a passed certificate the frozen "
              "owner or verdict copy :3712-3719 is `capture` (c); Observables and G5a :3723-3728, then "
              "`observable` (observables failed), `g5a` (observables passed, G5a failed) or `numeric` (both passed) "
              ":3734-3738; the precharged commit :3740-3743 is `capture` (d), every stage completed); the error "
              "return calls fail_entered :3749. Trace transitions: retained_receipt.rs enter/completed/checked/"
              "fail_entered. Error kinds: C3's PublicError (preparation, native, capture, proof, values, "
              "abandoned, numeric, observable, g5a)."),
    "stage_order": STAGES,
    "marks": {"-": "not_entered", "C": "completed", "F": "failed"},
    "records": records,
    "records_rule": ("every stage-2-consistent record: a completed prefix of the eight pipeline stages, then at most "
                     "one stage failed or not entered, the rest not entered; Observables and G5a entered together, "
                     "only after the certificate was entered, each completed or failed"),
    "kinds": kinds,
    "unknown_kinds": ["storage"],
}

# Unchanged: everything else (asserted).
for key in data:
    if key not in ("cases", "mutations", "must_pass", "d37"):
        assert data[key] == before[key], key
assert data["mutations"][:268] == before["mutations"] and data["must_pass"][:22] == before["must_pass"]
for f_new, f_old in zip(data["cases"], before["cases"]):
    a, b = deepcopy(f_new), deepcopy(f_old)
    for x in (a, b):
        for o in x["source"]["retained_precision"]["body"]["ordinary_attempts"]:
            o["diagnostic_refs"] = None
        x["source"]["retained_precision"]["receipt_sha256"] = None
    assert a == b, f_new["id"]

OUT.write_text(json.dumps(data, indent=2) + "\n")
Path(str(OUT) + ".repairs.json").write_text(json.dumps(repaired, indent=1) + "\n")
print("staged", OUT, len(data["cases"]), len(data["mutations"]), len(data["must_pass"]))

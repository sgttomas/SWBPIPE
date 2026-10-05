#!/usr/bin/env python3
"""I61 07h: the frozen snapshot record (SHARED_SNAPSHOT_07H.json) from the installed corpus and the 07g base.

Usage: snapshot_07h.py WT_P OLD_P OUT
"""
import datetime, hashlib, json, sys
from pathlib import Path
WT, OLD, OUT = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
rel = "fixtures/results/retained_precision_cases.json"
raw, old_raw = (WT / rel).read_bytes(), (OLD / rel).read_bytes()
new, old = json.loads(raw), json.loads(old_raw)
assert json.dumps(new, indent=2).encode() + b"\n" == raw
sha = lambda b: hashlib.sha256(b).hexdigest()
added_m = [m["id"] for m in new["mutations"][len(old["mutations"]):]]
added_p = [m["id"] for m in new["must_pass"][len(old["must_pass"]):]]
rec = {
    "version": "I61_SHARED_SNAPSHOT_07H",
    "freeze_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "status": ("Frozen delta from SHARED_SNAPSHOT_07G (U6e repair round: RV90 S1, N1, N2, N4); completeness flags "
               "unchanged (false); no acceptance, eligibility or reader approval."),
    "basis": {"ruling": "RV90 on U6e PASS (0B/1S/4N); 07h repair round granted (NUM a45a3201f8)",
              "previous_snapshot": "SHARED_SNAPSHOT_07G (corpus " + sha(old_raw)[:10] + ")",
              "reader_head_at_start": "5e1e2625ac"},
    "files": [{"path": "P/" + rel, "sha256": sha(raw), "bytes": len(raw),
               "snapshot": "WT/scratch/i61_u6e_07h/stage/retained_precision_cases_07h.json", "changed_from_snapshot_07g": True}],
    "format_rule": {"unchanged": "07e's rehash indexing, preparation-hash and order rules and 07g's d37 layout apply unchanged"},
    "delta": {
        "mutations_added": added_m,
        "must_pass_added": added_p,
        "d37_basis": "appended: the facade-order premise (PP/core/product_physics/src/lib.rs:2962-2974); every other d37 key unchanged",
        "s1": "f5_affected_refs_string_names_no_case: a case-naming diagnostic's affected_refs is the case id as a string, still listed; a non-array names no case, so F5 refuses (G5 ATTEMPT)",
        "n1": "f5_ordinary_refs_second_case_relaxed_d6a_form: case 1 of two_case_synthetic listing only its integrity diagnostic (07f's relaxed form); F5 refuses on a case after the first",
        "n2": "f5_ordinary_refs_strict_prefix: the first load diagnostic moved to the end of the envelope, the list without its last element (a strict prefix that still lists the integrity diagnostic); the must-pass f5_envelope_reordered_exact_list admits the exact list over the same envelope",
    },
    "unchanged": {
        "mutations_0_274_byte_identical": new["mutations"][:274] == old["mutations"],
        "must_pass_0_22_byte_identical": new["must_pass"][:22] == old["must_pass"],
        "cases_identical": new["cases"] == old["cases"],
        "arithmetic_identical": new["arithmetic"] == old["arithmetic"],
        "provenance_identical": new["provenance"] == old["provenance"],
        "version_identical": new["version"] == old["version"],
        "d37_except_basis_identical": {k: v for k, v in new["d37"].items() if k != "basis"} == {k: v for k, v in old["d37"].items() if k != "basis"},
        "d37_basis_is_a_suffix_extension": new["d37"]["basis"].startswith(old["d37"]["basis"]),
    },
    "counts": {"cases": len(new["cases"]), "mutations": len(new["mutations"]), "must_pass": len(new["must_pass"]),
               "d37_records": len(new["d37"]["records"]), "d37_kinds": len(new["d37"]["kinds"])},
    "python": {"reader_sha256": sha((WT / "core/analysis_runs/retained_precision.py").read_bytes()),
               "outcomes": "_run_records/PYTHON_OUTCOMES_07H.json", "delta": "_run_records/OUTCOME_DELTA_07G_07H.json"},
}
assert all(rec["unchanged"].values()), rec["unchanged"]
OUT.write_text(json.dumps(rec, indent=1) + "\n")
print(json.dumps(rec["counts"]), rec["files"][0]["sha256"])

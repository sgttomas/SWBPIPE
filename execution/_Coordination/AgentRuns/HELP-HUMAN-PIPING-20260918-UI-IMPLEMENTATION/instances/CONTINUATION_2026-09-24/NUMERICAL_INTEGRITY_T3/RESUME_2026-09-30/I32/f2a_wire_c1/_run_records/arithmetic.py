"""I32: exact integer projection of existing records only; no solver imports/runs.

Run from NUM root: python3 <this repository-relative path>. Writes beside itself.
This is run evidence, not a maintained runtime/schema/fixture implementation.
"""
import hashlib
import json
import pathlib
import tarfile

OUT = pathlib.Path(__file__).resolve().parent
NUM = OUT
while NUM.name != "numerics":
    NUM = NUM.parent
R = OUT.parents[2]
origins = []


def read(path):
    data = path.read_bytes()
    origins.append({"path": str(path.relative_to(NUM)), "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)})
    return data


def project(case):
    records = case["attempts"]
    logical = []
    # Each native record is retained exactly once. Charge fragments identify
    # ownership, while the original role/outcome/summary remains immutable.
    for i, a in enumerate(records):
        if a["role"] == "Verification":
            continue
        own = a["own_work"] + a["exact_sum_work"]
        stop = a["stop_rule_work"]
        verification = a["verification_work"]
        assert 0 <= stop + verification <= own
        fragments = [{"record": i, "part": "candidate_stop", "own": stop, "shared": 0, "verification_shared": 0}]
        if a["role"] == "Candidate":
            fragments.insert(0, {"record": i, "part": "solve_and_verification", "own": own-stop, "shared": a["shared_work"], "verification_shared": a["verification_shared_work"]})
        v = i + 1
        if v < len(records) and records[v]["precision"] == 2*a["precision"] and records[v]["role"] in ("Verification", "VerificationThenCandidate"):
            b = records[v]
            fragments.append({"record": v, "part": "solve_and_verification", "own": b["own_work"]+b["exact_sum_work"]-b["stop_rule_work"], "shared": b["shared_work"], "verification_shared": b["verification_shared_work"]})
        else:
            v = None
        total = sum(x["own"]+x["shared"]+x["verification_shared"] for x in fragments)
        invoked = sum(x["own"]+x["shared"]*int(records[x["record"]]["shared_built_here"])+x["verification_shared"]*int(records[x["record"]]["verification_shared_built_here"]) for x in fragments)
        logical.append({"precision": a["precision"], "candidate_record": i, "candidate_origin": "fresh" if a["role"] == "Candidate" else "reused_verification", "verification_record": v, "kernel_outcome": a["outcome"], "fragments": fragments, "case_charge": total, "invocation_increment": invoked})
    native_case = sum(a["own_work"]+a["exact_sum_work"]+a["shared_work"]+a["verification_shared_work"] for a in records)
    native_inv = sum(a["own_work"]+a["exact_sum_work"]+a["shared_work"]*int(a["shared_built_here"])+a["verification_shared_work"]*int(a["verification_shared_built_here"]) for a in records)
    assert native_case == sum(x["case_charge"] for x in logical)
    assert native_inv == sum(x["invocation_increment"] for x in logical) == case["invocation_charged"]
    for a in records:
        assert sum(a["stages"].values()) == a["own_work"]+a["exact_sum_work"]
        assert sum(a["shared_stages"].values()) == a["shared_work"]+a["verification_shared_work"]
    # Both wrong sums must disagree for the examples: verification and stop
    # fields are subsets, never new additive charges.
    double_counted = native_case + sum(a["verification_work"]+a["stop_rule_work"] for a in records)
    filtered = sum(a["own_work"]+a["exact_sum_work"]+a["shared_work"]+a["verification_shared_work"] for a in records if a["role"] != "Verification")
    assert double_counted != native_case and filtered != native_case
    return {"id": case["id"], "outcome": case["outcome"], "native_records": records, "logical_attempts": logical, "native_case_charge": native_case, "native_invocation_increment": native_inv, "wrong_subset_double_count": double_counted, "wrong_verification_filter_charge": filtered, "stage_closure": True}


vrpath = NUM / "projects/chirality-piping/validation/benchmarks/numerical_robustness/observations/kernel_lane/rf_range.json"
cases = {c["id"]: c for c in json.loads(read(vrpath))}
examples = [project(cases[n]) for n in ("RF-RANGE-CHAIN-L-240", "RF-RANGE-THIN-A")]
archive = R / "MEASUREMENTS/W1_T4/_run_records/delta_records.tar.gz"
read(archive)
member = "records/247_RF-LARGE-CHAIN-n10000-AX_w1a.jsonl"
with tarfile.open(archive) as tf:
    data = tf.extractfile(member).read()
origins.append({"archive": str(archive.relative_to(NUM)), "member": member, "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)})
rows = [json.loads(line) for line in data.splitlines() if line.strip()]
prefix = next(x for x in rows if x["kind"] == "prefix" and x["prefix"] == 1)
stopped = {"source_record": prefix, "overshoot_LME": prefix["meter_charged"] - prefix["case_limit"], "scope": "Existing stopped-prefix summary only. Archive does not expose this prefix's native attempt breakdown; do not invent it or claim fragment conservation checked for that prefix."}
assert stopped["overshoot_LME"] == 28740
U64 = (1 << 64)-1
SAFE = (1 << 53)-1
boundary = {"safe_integer_max": SAFE, "u64_max_decimal": str(U64), "safe_plus_one_refuses_checked_receipt": SAFE+1, "u64_saturation_example": {"before_decimal": str(U64-3), "increment": 5, "native_saturating_after_decimal": str(U64), "mathematical_sum_decimal": str(U64+2), "exact_reconstruction_available_from_native_after_alone": False}}
result = {"scope": "Exact integer/JSON arithmetic on existing immutable records, plus integer encoding boundaries; no numerical experiment or new solve.", "examples": examples, "stopped_prefix": stopped, "integer_boundaries": boundary}
(OUT / "ARITHMETIC.json").write_text(json.dumps(result, indent=2)+"\n")
(OUT / "ARITHMETIC_ORIGINS.json").write_text(json.dumps(origins, indent=2)+"\n")
print(json.dumps({"case_totals": [(x["id"], x["native_case_charge"], [(a["precision"], a["case_charge"]) for a in x["logical_attempts"]]) for x in examples], "stopped_prefix_overshoot": stopped["overshoot_LME"]}, indent=2))

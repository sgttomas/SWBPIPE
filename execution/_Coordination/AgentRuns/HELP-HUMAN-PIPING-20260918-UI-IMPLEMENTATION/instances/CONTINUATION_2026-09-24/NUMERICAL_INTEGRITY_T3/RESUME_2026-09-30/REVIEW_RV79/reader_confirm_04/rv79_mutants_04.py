"""RV79 single-edit mutants (confirmation 04: the 22 mutants plus M23-M29 aimed at D32, on READER abcb16fd27; confirmation 03: same 22 mutants on READER a894d9d0ba; confirmation 02: the same 22 mutants re-targeted to READER b36739112a; M04 and M20 follow the renamed R2/R3 lines)

Original: RV79 single-edit mutants of P/core/analysis_runs/retained_precision.py.

Each mutant starts from the pristine archived bytes (sha256 checked), applies exactly one
textual replacement (the old text must occur exactly once), runs the two Python test files
and restores the pristine bytes. Run from WT/rv79/P with the reader's environment.
"""
import hashlib, json, os, subprocess, sys, time
PRISTINE_SHA = "031334e29fb150e3bdb63f4b88816679cf83d4243f91db05a94ee9a8a6afa8f3"
TARGET = "core/analysis_runs/retained_precision.py"
MUTANTS = [
 ("M01_floor_positivity_dropped", "G5a feasibility ignores the p512 floor positivity",
  "            positive[2] = positive[2] or floor_positive[0]\n", "            positive[2] = positive[2]\n"),
 ("M02_p512_charge_is_estimate", "G5a charge roster uses the estimate at p512",
  "charge = [stop[2], stop[3]] if p == 512 else estimate", "charge = estimate"),
 ("M03_failed_verification_skips_one", "N4: escalating failed verification advances one slot",
  "                c += 2; escalated = True", "                c += 1; escalated = True"),
 ("M04_R3_disabled", "R3 cause/status containment disabled",
  "STATUS_FAULTS[o[\"fault\"]] <= _statuses(owner)", "True"),
 ("M05_N17_case_scope_ge", "N17 case-scope overshoot uses >= instead of >",
  "fail(charge > body[\"work\"][\"case_limit\"], \"WORK_MISMATCH\")", "fail(charge >= body[\"work\"][\"case_limit\"], \"WORK_MISMATCH\")"),
 ("M06_coverage_null_on_passed_g5a", "I57 s3: null coverage allowed after a passed G5a",
  "            or stages[\"g5a\"] == \"completed\" or checks[\"g5a\"][\"kind\"] == \"passed\"):", "            ):"),
 ("M07_G5b_phi_check_removed", "G5b floor equality Phi = phi_512(e_hat) removed",
  "_need([bits(_phi_512(hat[0])), bits(_phi_512(hat[1]))] == [floor[\"force\"], floor[\"moment\"]], \"G5b\", \"SCALE_MISMATCH\")", "pass"),
 ("M08_layout_input_derived_false", "canonical layout never marks constrained DOFs input-derived",
  "\"translation\" if j < 3 else \"rotation\", n, (n, j) in fixed)", "\"translation\" if j < 3 else \"rotation\", n, False)"),
 ("M09_separate_failure_unchecked", "P6: a values failure need not be separate_failure",
  "        fail(completion == \"separate_failure\")\n", "        pass\n"),
 ("M10_G3_roster_order_unchecked", "G3 coverage roster need not be 0..n-1",
  "_need(bool(inventory) and [x[\"body\"] for x in coverage]==inventory==list(range(len(inventory))),gate,\"COVERAGE_MISMATCH\")", "_need(bool(inventory) and len(coverage)==len(inventory),gate,\"COVERAGE_MISMATCH\")"),
 ("M11_small_scale_threshold", "absolute_bound small-scale threshold 2^-988 -> 2^-987",
  "if 0 < scale < 2.0 ** -988 else b0", "if 0 < scale < 2.0 ** -987 else b0"),
 ("M12_budget_failure_cached", "C1: budget failures are cached",
  "                if b[\"state\"] != \"budget_failure\":\n                    cache[b[\"slot\"]] = int(bi)", "                if True:\n                    cache[b[\"slot\"]] = int(bi)"),
 ("M13_w2_trigger_rule_removed", "O3: W2 need not follow an initial failure",
  "        fail((w2[\"kind\"] == \"not_triggered\") or initial[\"kind\"] in (\"structural_failure\", \"formation_failure\"))\n", "        pass\n"),
 ("M14_typed_first_failure_removed", "P9/P8: result error vs first failed stage unchecked",
  "        fail(error in expected[first_failed])", "        pass"),
 ("M15_N5_any_nonselected_terminal", "N5: any non-selected terminal accepted after a terminal stop",
  "        fail(terminal == _terminal_of(end_stop))", "        fail(terminal[\"kind\"] != \"selected\")"),
 ("M16_N10_exhaustion_not_required", "N10: idle group-null run need not be exhausted",
  "            fail(exhausted and terminal == {", "            fail(terminal == {"),
 ("M17_L0_feasibility_coupled", "G5a feasibility couples kinds even at L = 0",
  "positive = list(a) if length == 0 else [", "positive = [False] * 4 if False else ["),
 ("M18_strict_bracket_relaxed", "G8 interpolation bracket made non-strict",
  "if a[0] < t < b[0])", "if a[0] <= t <= b[0])"),
 ("M19_implementation_complete", "eligibility hold flipped to true",
  "_IMPLEMENTATION_COMPLETE = False", "_IMPLEMENTATION_COMPLETE = True"),
 ("M20_R2_disabled", "R2 lost-trace rule disabled",
  "    r2 = not any(o.get(\"lost\") is True for _, o in located)", "    r2 = True"),
 ("M21_upper_sanity_factor", "G5a sanity factor 1+2^-40 -> 1+2^-39",
  "float.fromhex(\"0x1.0000000001000p+0\")", "float.fromhex(\"0x1.0000000002000p+0\")"),
 ("M22_extent_order", "body extent summation order changed",
  "return math.sqrt(((d[0] * d[0]) + (d[1] * d[1])) + (d[2] * d[2]))", "return math.sqrt((d[0] * d[0]) + ((d[1] * d[1]) + (d[2] * d[2])))"),
 ("M23_integral_accepts_bool", "D32: _integral accepts bool (isinstance instead of type is int)",
  "    if type(value) is int: return value\n", "    if isinstance(value, int): return value\n"),
 ("M24_integral_allows_negative_zero", "D32: _integral drops its -0 exclusion",
  " and value == int(value) and not (value == 0 and math.copysign(1, value) < 0): return int(value)", " and value == int(value): return int(value)"),
 ("M25_normalize_skips_list_items", "D32: normalization leaves floats inside lists",
  "            if type(v) is float: value[i] = int(v)\n", "            if False: value[i] = int(v)\n"),
 ("M26_normalize_removed", "D32: no normalization after G2",
  'gate="G2";_encoding(receipt,schema);_normalize_integrals(receipt)', 'gate="G2";_encoding(receipt,schema)'),
 ("M27_g1_identity_host_int", "D32: G1 source-identity site back to a host-int test",
  'si=_integral(c.get("source_ref"))', 'si=c.get("source_ref") if type(c.get("source_ref")) is int else None'),
 ("M28_g3_d23_host_int", "D32: G3 D23 site back to a host-int test",
  'si=_integral(a["source_ref"])\n                if si is not None and 0<=si<len(body["sources"]):_need([x["member"]', 'si=a["source_ref"] if type(a["source_ref"]) is int else None\n                if si is not None and 0<=si<len(body["sources"]):_need([x["member"]'),
 ("M29_integral_truncates", "D32: _integral accepts non-integral floats (truncation)",
  "if type(value) is float and math.isfinite(value) and value == int(value) and not", "if type(value) is float and math.isfinite(value) and not"),
]

def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()

def main(log_dir, python):
    pristine = open(TARGET, "rb").read()
    assert hashlib.sha256(pristine).hexdigest() == PRISTINE_SHA, "not the reviewed bytes"
    results = []
    only = set(sys.argv[3:])
    for mid, desc, old, new in MUTANTS:
        if only and mid not in only: continue
        text = pristine.decode()
        count = text.count(old)
        if count != 1:
            results.append({"id": mid, "description": desc, "status": f"NOT_APPLIED (occurrences={count})"}); continue
        open(TARGET, "w").write(text.replace(old, new))
        mutated_sha = sha(TARGET)
        t0 = time.time()
        proc = subprocess.run([python, "-m", "pytest", "-q", "-p", "no:cacheprovider", "tests/test_retained_precision_contract.py", "tests/test_retained_precision_schema.py"], capture_output=True, text=True, timeout=1200)
        open(TARGET, "wb").write(pristine)
        assert sha(TARGET) == PRISTINE_SHA
        out = proc.stdout + proc.stderr
        open(os.path.join(log_dir, f"mutant_{mid}.log"), "w").write(out)
        summary = [l for l in out.splitlines() if " passed" in l or " failed" in l or "error" in l.lower()][-1:] or ["?"]
        failed = sorted({l.split("::", 1)[1].split(" ")[0] for l in out.splitlines() if l.startswith("FAILED ")})
        results.append({"id": mid, "description": desc, "mutated_sha256": mutated_sha, "exit": proc.returncode,
                        "killed": proc.returncode != 0, "summary": summary[0], "killing_tests": failed[:12], "killing_test_count": len(failed),
                        "seconds": round(time.time() - t0, 1)})
        print(json.dumps(results[-1]), flush=True)
    json.dump(results, open(os.path.join(log_dir, "mutants.json"), "w"), indent=1)

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])

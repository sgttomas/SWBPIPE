#!/usr/bin/env python3
"""I89 B1-SA mutants (PLAN_v2 §2.3's list, then extras): one textual edit each in a disposable
copy of the candidate's sources (`mut`: a `git archive` of the candidate commit, P only), then
PP's whole `--lib` suite in the registered build through WT/tools/t3_cargo.sh. Records
killed/survived, the failing tests with their panic messages, and whether the build failed (a
compile error is not a kill). Pristine bytes are restored and compared after every run. After
I85's b1_st_01 mutants.py (itself after I68's).

Usage: mutants.py MUT_P LOG_DIR TARGET_DIR T3_CARGO TMPDIR [ids...]
"""
import hashlib, json, os, re, subprocess, sys, time
M, LOG, TD, CARGO, TMP = sys.argv[1:6]
only = set(sys.argv[6:])
PP = os.path.join(M, "core/product_physics")
RM = os.path.join(PP, "src/retained_memory.rs")
RP = os.path.join(PP, "src/retained_product.rs")
MUTANTS = [
    # ---- PLAN_v2 §2.3's list ----
    # D1.4's bound: ≤ C becomes < C.
    ("MA1_d1_4_lt_c", RM,
     "    if m.load_cases.is_empty() || m.load_cases.len() > caps::LOAD_CASES {",
     "    if m.load_cases.is_empty() || m.load_cases.len() >= caps::LOAD_CASES {"),
    # D1.4's bound: ≤ C becomes ≤ C + 1.
    ("MA2_d1_4_le_c_plus_1", RM,
     "    if m.load_cases.is_empty() || m.load_cases.len() > caps::LOAD_CASES {",
     "    if m.load_cases.is_empty() || m.load_cases.len() > caps::LOAD_CASES + 1 {"),
    # The per-case load rows read case 0 only (the census's pre-B1 shape).
    ("MA3_load_rows_case_0_only", RM,
     "            let loads = &mut self.facts.primitive_loads;\n            loads.length = loads.length.max(case.primitive_loads.len());\n            loads.capacity = loads.capacity.max(case.primitive_loads.capacity());\n",
     "            if std::ptr::eq(case, &m.load_cases[0]) {\n                self.facts.primitive_loads = CapacityFact::vector(&case.primitive_loads);\n            }\n"),
    # G-B's running total dropped.
    ("MA4_g_b_total_dropped", RM,
     "        o(P::CaseLoadsTotal, count(f.capture.late_loads_total)),",
     "        o(P::CaseLoadsTotal, 0),"),
    # G-B's running total reads the current case only.
    ("MA5_g_b_total_current_case_only", RM,
     "        o(P::CaseLoadsTotal, count(f.capture.late_loads_total)),",
     "        o(P::CaseLoadsTotal, count(f.case.primitive_loads.len())),"),
    # T-3 (e): the seeds only (the pre-B1 predicate; `requested` unused).
    ("MA6_t3e_seeds_only", RM,
     "    requested >= 1 && capture.ordinary.len() == requested && capture.ordinary.iter().all(|seed| seed.initial.is_some())",
     "    let _ = requested;\n    !capture.ordinary.is_empty() && capture.ordinary.iter().all(|seed| seed.initial.is_some())"),
    # T-3 (e): G-C ignores the requested count (`CompleteFacts::requested_cases`), passing 1.
    ("MA7a_t3e_requested_ignored_at_g_c", RM,
     "        o(P::OrdinarySolveNotAttempted, u64::from(!ordinary_solve_attempted(f.capture, f.requested_cases))),",
     "        o(P::OrdinarySolveNotAttempted, u64::from(!ordinary_solve_attempted(f.capture, 1))),"),
    # T-3 (e): the predicate ignores the requested count (no seed-per-case comparison).
    ("MA7b_t3e_requested_ignored_in_count", RM,
     "    requested >= 1 && capture.ordinary.len() == requested && capture.ordinary.iter().all(|seed| seed.initial.is_some())",
     "    requested >= 1 && capture.ordinary.iter().all(|seed| seed.initial.is_some())"),
    # T-3 (e): ≥ instead of ==.
    ("MA8_t3e_ge_not_eq", RM,
     "    requested >= 1 && capture.ordinary.len() == requested && capture.ordinary.iter().all(|seed| seed.initial.is_some())",
     "    requested >= 1 && capture.ordinary.len() >= requested && capture.ordinary.iter().all(|seed| seed.initial.is_some())"),
    # The EnvelopeResults bound: P_final instead of C·P_final.
    ("MA9_envelope_results_p_final", RM,
     "            c * P_FINAL,\n            push_capacity(c * P_FINAL),",
     "            P_FINAL,\n            push_capacity(c * P_FINAL),"),
    # Its capacity: PushCap(P_final).
    ("MA9b_envelope_capacity_p_final", RM,
     "            push_capacity(c * P_FINAL),",
     "            push_capacity(P_FINAL),"),
    # Its text: 2·P_final·Text(row).
    ("MA9c_envelope_text_p_final", RM,
     "            2 * c * P_FINAL * text_atoms::ROW,",
     "            2 * P_FINAL * text_atoms::ROW,"),
    # ---- Extras (supporting acceptance items beyond the plan's list) ----
    # D1.5 reads case 0 only.
    ("MX1_d1_5_case_0_only", RM,
     "    for case in &m.load_cases {\n        if case.pressure_regions.is_some() {",
     "    for case in m.load_cases.iter().take(1) {\n        if case.pressure_regions.is_some() {"),
    # D1.7 reads case 0 only.
    ("MX2_d1_7_case_0_only", RM,
     "    for load in m.load_cases.iter().flat_map(|case| &case.primitive_loads) {",
     "    for load in m.load_cases.iter().take(1).flat_map(|case| &case.primitive_loads) {"),
    # The TotalLoads row observes nothing.
    ("MX3_total_loads_row_dropped", RM,
     "        row(K::TotalLoads, n.total_loads as usize, TOTAL_LOADS),",
     "        row(K::TotalLoads, 0, TOTAL_LOADS),"),
    # The census total keeps only the last case.
    ("MX4_census_total_last_case_only", RM,
     "            let total = u32::try_from(case.primitive_loads.len()).ok().and_then(|l| self.facts.total_loads.checked_add(l));",
     "            let total = u32::try_from(case.primitive_loads.len()).ok();"),
    # The seam's add dropped (retained_product.rs, SP's file; scratch copy only).
    ("MX5_seam_add_dropped", RP,
     "                Some(total)=>self.late_loads_total=total,",
     "                Some(_)=>{},"),
    # B-6 not multiplied by C.
    ("MX6_b6_one_notice", RM,
     "    notice_reserve_bytes: NOTICE_RESERVE_BYTES * caps::LOAD_CASES as u64,",
     "    notice_reserve_bytes: NOTICE_RESERVE_BYTES,"),
    # A contract-evidence fact not multiplied by C.
    ("MX7_contract_evidence_one_case", RM,
     "            c * (3 * m + 2 * g),",
     "            3 * m + 2 * g,"),
    # RetainedErrorTextBytes not multiplied by C.
    ("MX8_retained_error_one_case", RM,
     "            c * (3 * m + 1) * text_atoms::ERR,",
     "            (3 * m + 1) * text_atoms::ERR,"),
    # LoadCasesCapacity capped at 1 (the pre-B1 cap).
    ("MX9_load_cases_capacity_one", RM,
     "        row(K::LoadCasesCapacity, t.load_cases.capacity, LOAD_CASES),",
     "        row(K::LoadCasesCapacity, t.load_cases.capacity, 1),"),
]
ENV = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", CARGO_TARGET_DIR=TD, TMPDIR=TMP)
ENV.pop("RUSTFLAGS", None); ENV.pop("CARGO_ENCODED_RUSTFLAGS", None)
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
results = []
for mid, path, old, new in MUTANTS:
    if only and mid not in only: continue
    pristine = open(path, "rb").read()
    text = pristine.decode()
    assert text.count(old) == 1, (mid, text.count(old))
    open(path, "wb").write(text.replace(old, new).encode())
    log = os.path.join(LOG, f"mutant_{mid}.log")
    t0 = time.time()
    with open(log, "w") as fh:
        rc = subprocess.run([CARGO, "test", "--locked", "--offline", "--lib", "--no-fail-fast"], cwd=PP, env=ENV, stdout=fh, stderr=subprocess.STDOUT).returncode
    open(path, "wb").write(pristine)
    assert open(path, "rb").read() == pristine
    out = open(log, encoding="utf-8", errors="replace").read()
    compiled = "Running unittests" in out
    failed = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED$", out, re.M)))
    panics = re.findall(r"^thread '([^']+)' \(\d+\) panicked at ([^\n]+):\n([^\n]*)", out, re.M)
    summary = re.findall(r"^test result: .*$", out, re.M)
    results.append({"id": mid, "file": os.path.relpath(path, M), "rc": rc, "compiled": compiled,
                    "killed": compiled and bool([f for f in failed if "t13_committed_fallback_uz_is_byte_identical" not in f]),
                    "failed_tests": failed,
                    "panics": [{"test": t, "at": a, "message": m} for t, a, m in panics],
                    "summary": summary, "seconds": round(time.time() - t0, 1), "restored_sha256": sha(path)})
    print(json.dumps({k: results[-1][k] for k in ("id", "compiled", "killed", "failed_tests", "seconds")}), flush=True)
json.dump(results, open(os.path.join(LOG, "mutants.json" if not only else "mutants_partial.json"), "w"), indent=1)

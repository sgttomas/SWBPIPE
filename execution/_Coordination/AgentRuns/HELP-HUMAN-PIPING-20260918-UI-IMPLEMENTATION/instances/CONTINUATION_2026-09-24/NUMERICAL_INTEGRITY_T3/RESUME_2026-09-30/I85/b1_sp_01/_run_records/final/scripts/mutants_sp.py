#!/usr/bin/env python3
"""I85 B1-SP mutants: PLAN_v2 §2.2's list with RV109's R17, and ST's seven (M1-M5, R10, R16) with
their anchors moved to SP's code. Each mutant is one or more textual edits in a disposable copy of
the candidate's P (`mut`: a `git archive` of the candidate commit), then PP's whole `--lib` suite in
the registered build through WT/tools/t3_cargo.sh. Records killed/survived, the failing tests with
their panic messages, and whether the build compiled (a compile error is not a kill). The pristine
bytes are restored and compared after every run. After ST's mutants.py.

Usage: mutants_sp.py MUT_P LOG_DIR TARGET_DIR T3_CARGO TMPDIR [ids...]
"""
import hashlib, json, os, re, subprocess, sys, time
M, LOG, TD, CARGO, TMP = sys.argv[1:6]
only = set(sys.argv[6:])
PP = os.path.join(M, "core/product_physics")
LIB = os.path.join(PP, "src/lib.rs")
PROD = os.path.join(PP, "src/retained_product.rs")
WIRE = os.path.join(PP, "src/retained_wire.rs")
T4 = "    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, cases.ids()).any(|trigger| trigger == CaseTrigger::Attempted) {\n        return (ordinary, Err(W1Fallback::NoTriggeredCase));\n    }"
MUTANTS = [
    # ---- ST's (PLAN_v2 §2.1, repair 1), anchored in SP's code.
    ("M1_key_on_initial", [(LIB,
     "        if verdict == Some(NumericalQualityStatus::ChecksPassed) {\n            CaseTrigger::NotRequired",
     "        if seed.is_some_and(|seed| matches!(&seed.initial, Some(retained_product::InitialSeed::Report { code, .. }) if code == \"NUMERICAL_INTEGRITY_CHECKS_PASSED\")) {\n            CaseTrigger::NotRequired")]),
    ("M2_verdict_by_position", [(LIB,
     "    case_ids.iter().map(move |&case_id| {\n        let verdict = only_one(quality.cases.iter().filter(|entry| entry.basis_ref.ref_id == case_id)).map(|entry| entry.solve_quality);",
     "    case_ids.iter().enumerate().map(move |(position, &case_id)| {\n        let verdict = quality.cases.get(position).map(|entry| entry.solve_quality);")]),
    ("M3_drop_decision_21", [(LIB, "        } else if seed.is_some_and(dn_trigger_excluded) {", "        } else if seed.is_some_and(|_| false) {")]),
    ("M4_seedless_excluded", [(LIB, "        } else if seed.is_some_and(dn_trigger_excluded) {", "        } else if seed.is_none_or(dn_trigger_excluded) {")]),
    ("M5_no_triggered_case_notice", [(LIB,
     "        return (ordinary, Err(W1Fallback::NoTriggeredCase));",
     "        return match ReservedNotices::reserve(&mut ordinary, cases.ids()) { Some(notice) => notice.publish(ordinary, W1Fallback::NoTriggeredCase, 0), None => (ordinary, Err(W1Fallback::NoticeReservation)) };")]),
    ("R10_t4_after_reservation", [(LIB, T4,
     "    let Some(early) = ReservedNotices::reserve(&mut ordinary, cases.ids()) else {\n        return (ordinary, Err(W1Fallback::NoticeReservation));\n    };\n"
     "    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, cases.ids()).any(|trigger| trigger == CaseTrigger::Attempted) {\n        drop(early);\n        return (ordinary, Err(W1Fallback::NoTriggeredCase));\n    }\n    drop(early);")]),
    ("R16_not_required_needs_seed", [(LIB, "        if verdict == Some(NumericalQualityStatus::ChecksPassed) {", "        if verdict == Some(NumericalQualityStatus::ChecksPassed) && seed.is_some() {")]),
    # ---- RV109 N-2: `.all` for `.any` in the trigger test (W-C2 distinguishes them).
    ("R17_all_for_any", [(LIB, ".any(|trigger| trigger == CaseTrigger::Attempted) {", ".all(|trigger| trigger == CaseTrigger::Attempted) {")]),
    # ---- PLAN_v2 §2.2's SP list.
    # Staging order: the unavailable cases' diagnostics before the selected cases'.
    ("S1_staging_order", [(WIRE,
     """    for (index, (_, omit)) in legacy.iter().enumerate() {
        if status[index] == CaseStatus::Selected {
            diagnostic_ids[index] = Some(selected_case_envelope(&mut env, case_ids[index], omit.as_deref())?);
        }
    }
    for index in 0..count {
        if status[index] == CaseStatus::Unavailable {
            diagnostic_ids[index] = Some(unavailable_case_envelope(&mut env, case_ids[index])?);
        }
    }""",
     """    for index in 0..count {
        if status[index] == CaseStatus::Unavailable {
            diagnostic_ids[index] = Some(unavailable_case_envelope(&mut env, case_ids[index])?);
        }
    }
    for (index, (_, omit)) in legacy.iter().enumerate() {
        if status[index] == CaseStatus::Selected {
            diagnostic_ids[index] = Some(selected_case_envelope(&mut env, case_ids[index], omit.as_deref())?);
        }
    }""")]),
    # attempt_ref: an unavailable case's source names attempt 0, not its own.
    ("S2_attempt_ref", [(WIRE,
     "                bind_preparation(&mut source_v, &attempt_v, attempt.attempt)?;",
     "                bind_preparation(&mut source_v, &attempt_v, 0)?;")]),
    # The snapshot record point: a nonselected Run's snapshot deferred to the end of T-9.
    ("S3_snapshot_point", [(PROD,
     "                    attempt.trace.native_error = Some(error);\n                    attempt.trace.freeze(capture);",
     "                    attempt.trace.native_error = Some(error);"),
     (PROD, "        capture.native_invocation = invocation;\n    }\n    /// B1 SP (DESIGN_v2 T-11, staging)",
     "        for attempt in attempts.iter_mut() { attempt.trace.freeze(capture); }\n        capture.native_invocation = invocation;\n    }\n    /// B1 SP (DESIGN_v2 T-11, staging)")]),
    # Detail placement: C1:68's detail on every notice.
    ("S4_detail_placement", [(LIB, "            if let Some(detail) = detail.filter(|_| was_selected) {", "            if let Some(detail) = detail {")]),
    # The reservation count: one slot fewer than the cases in A.
    ("S5_reservation_count", [(LIB, "        ordinary.diagnostics.try_reserve_exact(case_ids.len()).ok()?;", "        ordinary.diagnostics.try_reserve_exact(case_ids.len() - 1).ok()?;")]),
    # Decision 5's abandonment set, each member made not to abandon:
    ("S6a_custody", [(PROD,
     "        if let Err(error)=self.prepared_custody(&ordinary,requested,attempted) {",
     "        if let Err(error)=self.prepared_custody(&ordinary,requested,attempted).or_else(|e|match e {CaptureError::Association(_)=>Ok(()),e=>Err(e)}) {")]),
    ("S6b_call_failure", [(PROD, "                Err(error) => Err(error.clone()),", "                Err(_) => Ok(()),")]),
    ("S6c_no_case_selected", [(LIB, "    if !reached(|end| matches!(end, AttemptEnd::Frozen(_))) {", "    if false && !reached(|end| matches!(end, AttemptEnd::Frozen(_))) {")]),
    ("S6d_staging", [(LIB, "        Err(fault) => return (prepared.into_ordinary(), Err((W1Fallback::Staging(fault), selected))),", "        Err(_) => prepared.ordinary.clone(),")]),
    ("S6e_serializer", [(LIB, "        Err(failure) => return (prepared.into_ordinary(), Err((W1Fallback::Serializer(failure), selected))),", "        Err(_) => serde_json::to_value(&prepared.ordinary).unwrap(),")]),
    ("S6f_precommit", [(LIB,
     "    if let Err(error) = open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)) {",
     "    if let Some(error) = open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)).err().filter(|_| false) {")]),
    # The ordinal-to-request mapping: the batch ordinals taken as the attempts' ids.
    ("S7_ordinal_mapping", [(WIRE,
     "    let requests: Vec<usize> = attempts.iter().filter(|attempt| attempt.prepared).map(|attempt| attempt.request).collect();",
     "    let requests: Vec<usize> = attempts.iter().filter(|attempt| attempt.prepared).map(|attempt| attempt.attempt).collect();")]),
    # The domain re-check: c = C + 1 admitted; one combination admitted.
    ("S8a_domain_c_plus_1", [(LIB, "cases.len() > retained_memory::caps::LOAD_CASES || combinations != 0", "cases.len() > retained_memory::caps::LOAD_CASES + 1 || combinations != 0")]),
    ("S8b_domain_combination", [(LIB, "cases.len() > retained_memory::caps::LOAD_CASES || combinations != 0", "cases.len() > retained_memory::caps::LOAD_CASES || combinations > 1")]),
    # c = C + 1 admitted by attempting only the first C cases (S8a alone is equivalent: `CaseSet::push`
    # also refuses past C, as RV109 found for its S16 twin).
    ("S8a2_domain_c_plus_1_truncated", [(LIB, "cases.len() > retained_memory::caps::LOAD_CASES || combinations != 0", "combinations != 0"),
     (LIB, "    for (request, case) in cases.iter().enumerate() {\n        set.push(request, case[\"id\"].as_str()?)?;",
      "    for (request, case) in cases.iter().take(retained_memory::caps::LOAD_CASES).enumerate() {\n        set.push(request, case[\"id\"].as_str()?)?;")]),
    # ---- RV109's survivors at R3′ (R3P-7, R3P-8), kept in SP's list.
    ("RV_S3_per_case_presence", [(PROD,
     "            if counts[index] != [1, usize::from(captured.parity_produced)] {",
     "            if false && counts[index] != [1, usize::from(captured.parity_produced)] {")]),
    ("RV_S4_finish_parked_captures", [(PROD,
     "                || self.parked.iter().any(|slot|slot.prepared_late_calls!=1 || !slot.prepared_source_permit\n                    || !slot.prepared_one_case_seen || slot.source_capture_entries!=1 || slot.source.is_none()) {",
     "                || false {")]),
    ("RV_S7_parked_native", [(PROD, "\n            || self.parked.iter().any(|slot|slot.native.is_some())", "")]),
    ("RV_S13_one_diagnostics_slot", [(LIB, "        ordinary.diagnostics.try_reserve_exact(case_ids.len()).ok()?;", "        ordinary.diagnostics.try_reserve_exact(1).ok()?;")]),
    ("RV_S14_no_cross_notice_id_check", [(LIB, "if ordinary.diagnostics.iter().any(|d| d.id == id) || ids[..k].contains(&id) {", "if ordinary.diagnostics.iter().any(|d| d.id == id) {")]),
    ("RV_S20_capacity_assigned", [(PROD,
     "        self.observation_capacity_bytes[slot] = self.observation_capacity_bytes[slot]\n            .checked_add(value.capacity())\n            .ok_or(CaptureError::CountRange(\"observation capacity total\"))?;",
     "        self.observation_capacity_bytes[slot] = value.capacity();")]),
]
ENV = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", CARGO_TARGET_DIR=TD, TMPDIR=TMP)
ENV.pop("RUSTFLAGS", None); ENV.pop("CARGO_ENCODED_RUSTFLAGS", None)
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
results = []
for mid, edits in MUTANTS:
    if only and mid not in only: continue
    pristine = {path: open(path, "rb").read() for path, _, _ in edits}
    current = {path: data.decode() for path, data in pristine.items()}
    for path, old, new in edits:
        assert current[path].count(old) == 1, (mid, path, current[path].count(old))
        current[path] = current[path].replace(old, new)
    for path, text in current.items():
        open(path, "wb").write(text.encode())
    log = os.path.join(LOG, f"mutant_{mid}.log")
    t0 = time.time()
    with open(log, "w") as fh:
        rc = subprocess.run([CARGO, "test", "--locked", "--offline", "--lib", "--no-fail-fast"], cwd=PP, env=ENV, stdout=fh, stderr=subprocess.STDOUT).returncode
    for path, data in pristine.items():
        open(path, "wb").write(data)
        assert open(path, "rb").read() == data
    out = open(log, encoding="utf-8", errors="replace").read()
    compiled = "Running unittests" in out
    failed = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED$", out, re.M)))
    panics = re.findall(r"^thread '([^']+)' \(\d+\) panicked at ([^\n]+):\n([^\n]*)", out, re.M)
    summary = re.findall(r"^test result: .*$", out, re.M)
    results.append({"id": mid, "files": sorted({os.path.relpath(p, M) for p, _, _ in edits}), "rc": rc, "compiled": compiled,
                    "killed": compiled and bool([f for f in failed if "t13_committed_fallback_uz_is_byte_identical" not in f]),
                    "failed_tests": failed,
                    "panics": [{"test": t, "at": a, "message": m} for t, a, m in panics],
                    "summary": summary, "seconds": round(time.time() - t0, 1),
                    "restored_sha256": {os.path.relpath(p, M): sha(p) for p in pristine}})
    print(json.dumps({k: results[-1][k] for k in ("id", "compiled", "killed", "failed_tests", "seconds")}), flush=True)
json.dump(results, open(os.path.join(LOG, "mutants.json"), "w"), indent=1)

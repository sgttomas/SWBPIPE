#!/usr/bin/env python3
"""RV109 round 2: the reviewer's mutants of SP at 603e238517 (beyond I85's and PLAN_v2 §2.2's list).
Each mutant is one or more exact replacements, each of which must match exactly once in its PP src file.

Usage: sp2_mutants.py <PP src dir> apply <ID>   |   sp2_mutants.py list   |   sp2_mutants.py check <PP src dir>
"""
import sys

P, L, W = "retained_product.rs", "lib.rs", "retained_wire.rs"
M = {}


def m(mid, why, *edits):
    M[mid] = (why, edits)


# ---- R3′'s S13, S14, S20 (now pinned, per I85), and the four whose code changed (S7, S9, S10, S18) ----
m("S7_custody_ignores_parked_native", "T-6: native work in a parked slot is not refused",
  (P, "            || self.parked.iter().any(|slot|slot.native.is_some())\n", ""))
m("S9_no_failure_snapshot", "T-7/T-11: a failed preparation takes no terminal snapshot",
  (P, "Err(e)=>{trace.fail_entered();self.error=Some(e);trace.freeze(self);(false,AttemptEnd::Preparation)}",
      "Err(e)=>{trace.fail_entered();self.error=Some(e);(false,AttemptEnd::Preparation)}"))
m("S10_failure_not_in_slot", "T-7: a failed preparation's error is not kept in its slot",
  (P, "Err(e)=>{trace.fail_entered();self.error=Some(e);trace.freeze(self);(false,AttemptEnd::Preparation)}",
      "Err(e)=>{trace.fail_entered();let _=e;trace.freeze(self);(false,AttemptEnd::Preparation)}"))
m("S13_one_reserved_slot", "T-5: one diagnostics slot reserved, whatever |A|",
  (L, "ordinary.diagnostics.try_reserve_exact(case_ids.len()).ok()?;", "ordinary.diagnostics.try_reserve_exact(1).ok()?;"))
m("S14_no_cross_notice_collision", "T-5: two notices with one id are not refused",
  (L, "if ordinary.diagnostics.iter().any(|d| d.id == id) || ids[..k].contains(&id) {", "if ordinary.diagnostics.iter().any(|d| d.id == id) {"))
m("S18_one_case_binder_at_c_ge_2", "T-6: custody binds observations with the one-case binder at c >= 2",
  (P, "        self.bind_observations_by_case(ordinary)?;\n        self.bind_case_rows(ordinary)", "        self.bind_observations(ordinary)?;\n        self.bind_case_rows(ordinary)"))
m("S20_observation_bytes_assigned", "the observation capacity record is the last case's, not the sum",
  (P, "self.observation_capacity_bytes[slot] = self.observation_capacity_bytes[slot]\n            .checked_add(value.capacity())\n            .ok_or(CaptureError::CountRange(\"observation capacity total\"))?;",
      "self.observation_capacity_bytes[slot] = value.capacity();"))

# ---- T-12: detail placement and notice order (lib.rs) ----
m("M02_detail_on_every_notice", "T-12: C1:68's detail on every notice, not only the selected cases'",
  (L, ".is_some_and(|bits| bits & 1 == 1);", ".is_some_and(|_| true);"))
m("M03_detail_bit_shifted", "T-12: notice k reads bit k + 1",
  (L, "and_then(|k| selected.checked_shr(k))", "and_then(|k| selected.checked_shr(k.wrapping_add(1)))"))
m("M04_selected_bits_zero", "T-12: no case is recorded as selected at abandonment",
  (L, "let selected = prepared.selected_attempts();", "let selected = 0u64;"))
m("M05_selected_bits_by_request", "T-12: bit = request index, not the case's position in A",
  (P, "        self.attempts.iter().enumerate()\n            .filter(|(_, attempt)| matches!(attempt.end, AttemptEnd::Frozen(_)))",
      "        self.attempts.iter().map(|attempt| (attempt.request, attempt))\n            .filter(|(_, attempt)| matches!(attempt.end, AttemptEnd::Frozen(_)))"))
m("M07_notices_reversed", "T-12: the notices in reverse request order",
  (L, "self.0.into_iter().flatten().enumerate()", "self.0.into_iter().flatten().rev().enumerate()"))
# ---- T-10 and decision 5 (lib.rs) ----
m("M06_cause_all_not_any", "T-10: the cause is the stage every case reached, not the furthest any reached",
  (L, "prepared.attempts.iter().any(|attempt| stage(&attempt.end));", "prepared.attempts.iter().all(|attempt| stage(&attempt.end));"))
m("M08_w1_attempts_every_case", "T-4/T-7: W1 attempts every requested case, not A",
  (L, "match w1_transaction(observer, ordinary, capture, cases.ids().len(), attempted.requests()) {",
      "match w1_transaction(observer, ordinary, capture, cases.ids().len(), cases.requests()) {"))
m("M46_domain_c_plus_one", "domain re-check: c = C + 1 is admitted",
  (L, "if cases.is_empty() || cases.len() > retained_memory::caps::LOAD_CASES || combinations != 0 {",
      "if cases.is_empty() || cases.len() > retained_memory::caps::LOAD_CASES + 1 || combinations != 0 {"))
m("M47_custody_cause_native", "decision 5: a custody failure is named Native",
  (L, "Err(failure) => return (failure.ordinary, Err((W1Fallback::Preparation, 0))),", "Err(failure) => return (failure.ordinary, Err((W1Fallback::Native, 0))),"))
m("M48_no_t10_check", "T-10: no check that a case is selected before T-11",
  (L, "if !reached(|end| matches!(end, AttemptEnd::Frozen(_))) {", "if false && !reached(|end| matches!(end, AttemptEnd::Frozen(_))) {"))
m("M49_staging_fault_ignored", "decision 5: a staging fault does not abandon (the ordinary copy is serialized)",
  (L, "Err(fault) => return (prepared.into_ordinary(), Err((W1Fallback::Staging(fault), selected))),", "Err(_fault) => prepared.ordinary.clone(),"))
m("M51_precommit_never_refuses", "decision 5: precommit's refusal does not abandon",
  (L, "if let Err(error) = open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)) {",
      "if let Some(error) = open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)).err().filter(|_| false) {"))
m("M52_r_b_prime_dropped", "N-5: R-b′'s recovery_demoted no longer refuses (the known limit)",
  (W, "    if seed.recovery_demoted {\n        e.untranslated(\"ordinary_attempts[].formation.recovery_finding\");\n    }\n", ""))
# ---- T-8 (retained_product.rs) ----
m("M09_runs_to_wrong_cases", "T-8: the call's Runs are assigned to the submitted cases in reverse",
  (P, "for (request,case) in submitted().zip(cases) {", "for (request,case) in submitted().zip(cases.into_iter().rev()) {"))
m("M10_any_run_selected", "T-8: a Run of any terminal counts as selected",
  (P, "                    Some(k::ExecutionOutcome::Selected(_)) => Ok(()),\n                    _ => Err(CaptureError::NativeUnavailable),",
      "                    Some(_) => Ok(()),\n                    _ => Err(CaptureError::NativeUnavailable),"))
m("M11_sources_back_reversed", "T-8: the moved sources go back to the wrong cases",
  (P, "let mut back=sources.into_iter();", "let mut back=sources.into_iter().rev();"))
m("M41_call_failure_selects", "T-8: a call failure before any Run leaves the cases selectable",
  (P, "                Err(error) => Err(error.clone()),\n                Ok(()) => match capture.case_native",
      "                Err(_) => Ok(()),\n                Ok(()) => match capture.case_native"))
# ---- T-11 snapshots (retained_product.rs) ----
m("M12_native_snapshot_late", "T-11: a non-selected Run's snapshot is taken after T-9, not at its Run",
  (P, "                    attempt.trace.native_error = Some(error);\n                    attempt.trace.freeze(capture);\n",
      "                    attempt.trace.native_error = Some(error);\n"),
  (P, "        capture.native_invocation = invocation;\n    }\n    /// B1 SP (DESIGN_v2 T-11, staging)",
      "        for a in attempts.iter_mut().filter(|a| matches!(a.end, AttemptEnd::Native)) { a.trace.freeze(capture); }\n        capture.native_invocation = invocation;\n    }\n    /// B1 SP (DESIGN_v2 T-11, staging)"))
m("M13_frozen_snapshot_early", "T-11: a selected case's snapshot is taken before its proof",
  (P, "            let scope = capture.case_scope(attempt.request);\n            let frozen = capture.with_case(",
      "            attempt.trace.freeze(capture);\n            let scope = capture.case_scope(attempt.request);\n            let frozen = capture.with_case("))
# ---- T-9 scope (retained_product.rs) ----
m("M33_scope_unqualified", "T-9: a later case's row ids are bound unqualified",
  (P, "CaseScope { rows: Some(rows), evidence: index, cases, qualified: index > 0 }", "CaseScope { rows: Some(rows), evidence: index, cases, qualified: false }"))
m("M34_scope_evidence_zero", "T-9: every case reads evidence case 0",
  (P, "CaseScope { rows: Some(rows), evidence: index, cases, qualified: index > 0 }", "CaseScope { rows: Some(rows), evidence: 0, cases, qualified: index > 0 }"))
m("M36_custody_attempted_unchecked", "T-6: A's indices are not validated",
  (P, "if attempted.iter().any(|&request|request>=requested) || attempted.windows(2).any(|pair|pair[0]>=pair[1]) {", "if false {"))
m("M37_prior_error_own_first", "T-6: the last case's prior error before the parked cases'",
  (P, "        if let Some(slot)=self.parked.iter_mut().find(|slot|slot.error.is_some()) {\n            return Err(slot.error.take().expect(\"observed prior case cause\"));\n        }\n        if self.error.is_some() {\n            return Err(self.error.take().expect(\"observed prior capture cause\"));\n        }",
      "        if self.error.is_some() {\n            return Err(self.error.take().expect(\"observed prior capture cause\"));\n        }\n        if let Some(slot)=self.parked.iter_mut().find(|slot|slot.error.is_some()) {\n            return Err(slot.error.take().expect(\"observed prior case cause\"));\n        }"))
# ---- T-11 headline rule (retained_product.rs) ----
m("M27_headline_tie_reversed", "T-11 headline: a tie goes to the larger (case, location)",
  (P, "(case,row.entity_ref.as_str())<(c,b.entity_ref.as_str())", "(case,row.entity_ref.as_str())>(c,b.entity_ref.as_str())"))
m("M28_headline_tie_location_only", "T-11 headline: a tie ignores the case id",
  (P, "(case,row.entity_ref.as_str())<(c,b.entity_ref.as_str())", "row.entity_ref.as_str()<b.entity_ref.as_str() && !c.is_empty() && !case.is_empty()"))
m("M29_no_stage_headlines", "T-11 headline: the ordinary headlines are kept at c >= 2",
  (P, "            stage_headlines(&mut staged, &self.ordinary)?;", "            let _ = stage_headlines;"))
m("M31_headline_ignores_presence", "T-11 headline: a headline is staged where the ordinary has none",
  (P, "        if !present {continue;}\n", "        let _ = present;\n"))
m("M32_headline_stress_only", "T-11 headline: the displacement headline is not restaged",
  (P, "for (kind,stress) in [(\"displacement_magnitude\",false),(\"pipe_elastic_normal_stress_maximum_v2\",true)] {",
      "for (kind,stress) in [(\"pipe_elastic_normal_stress_maximum_v2\",true)] {"))
# ---- T-11 serializer (retained_wire.rs) ----
m("M14_unavailable_diagnostics_first", "T-11 staging: the unavailable diagnostics before the selected ones",
  (W, "    for (index, (_, omit)) in legacy.iter().enumerate() {\n        if status[index] == CaseStatus::Selected {\n            diagnostic_ids[index] = Some(selected_case_envelope(&mut env, case_ids[index], omit.as_deref())?);\n        }\n    }\n    for index in 0..count {\n        if status[index] == CaseStatus::Unavailable {\n            diagnostic_ids[index] = Some(unavailable_case_envelope(&mut env, case_ids[index])?);\n        }\n    }",
      "    for index in 0..count {\n        if status[index] == CaseStatus::Unavailable {\n            diagnostic_ids[index] = Some(unavailable_case_envelope(&mut env, case_ids[index])?);\n        }\n    }\n    for (index, (_, omit)) in legacy.iter().enumerate() {\n        if status[index] == CaseStatus::Selected {\n            diagnostic_ids[index] = Some(selected_case_envelope(&mut env, case_ids[index], omit.as_deref())?);\n        }\n    }"))
m("M15_selected_diagnostics_reversed", "T-11 staging: the selected cases' diagnostics in reverse request order",
  (W, "    for (index, (_, omit)) in legacy.iter().enumerate() {\n        if status[index] == CaseStatus::Selected {",
      "    for (index, (_, omit)) in legacy.iter().enumerate().rev() {\n        if status[index] == CaseStatus::Selected {"))
m("M38_not_required_gets_notice", "T-11 staging: a not_required case gets the unavailable diagnostic",
  (W, "        if status[index] == CaseStatus::Unavailable {\n            diagnostic_ids[index] = Some(unavailable_case_envelope(",
      "        if status[index] != CaseStatus::Selected {\n            diagnostic_ids[index] = Some(unavailable_case_envelope("))
m("M16_frozen_attempt_ref_zero", "T-7: a selected case's source names attempt 0",
  (W, "bind_preparation(&mut source, &attempt_v, attempt.attempt)?;", "bind_preparation(&mut source, &attempt_v, 0)?;"))
m("M17_frozen_attempt_id_zero", "T-11: a selected case's product attempt has id 0",
  (W, "let attempt_v = product_attempt(e, &view, attempt.attempt, request, Some(run.source), Some(case.run), json!({\"kind\":\"ready\"}), pc);",
      "let attempt_v = product_attempt(e, &view, 0, request, Some(run.source), Some(case.run), json!({\"kind\":\"ready\"}), pc);"))
m("M18_unavailable_attempt_ref_zero", "T-7: an unavailable case's source names attempt 0",
  (W, "bind_preparation(&mut source_v, &attempt_v, attempt.attempt)?;", "bind_preparation(&mut source_v, &attempt_v, 0)?;"))
m("M19_frozen_run_owner_ordinal", "N-2: a selected case's Run owner is its batch ordinal",
  (W, "        let run_v = run_value(e, run, records, terminal, request)?;\n        let mut source = case_source(",
      "        let run_v = run_value(e, run, records, terminal, ordinal.unwrap_or(request))?;\n        let mut source = case_source("))
m("M20_unavailable_run_owner_ordinal", "N-2: an unavailable case's Run owner is its batch ordinal",
  (W, "            let run_v = run_value(e, run, records, terminal, request)?;\n            let source = match selected_owner",
      "            let run_v = run_value(e, run, records, terminal, ordinal.unwrap_or(request))?;\n            let source = match selected_owner"))
m("M21_call_owner_refs_ordinal", "N-2: calls[].owner_refs name batch ordinals",
  (W, "k::NativeOwner::Case(i) => requests.get(*i).map(|request| json!({\"kind\":\"case\",\"index\":request}))",
      "k::NativeOwner::Case(i) => requests.get(*i).map(|_| json!({\"kind\":\"case\",\"index\":i}))"))
m("M22_execution_order_ordinal", "N-2: work.execution_order names batch ordinals",
  (W, "k::NativeOwner::Case(ordinal) => requests.get(ordinal).map(|request| json!({\"kind\":\"case\",\"index\":request})),",
      "k::NativeOwner::Case(ordinal) => requests.get(ordinal).map(|_| json!({\"kind\":\"case\",\"index\":ordinal})),"))
m("M23_source_index_zero", "T-11: every source's index is 0",
  (W, "Ok(json!({\"index\":run.source,", "Ok(json!({\"index\":0,"))
m("M24_ordinals_over_all_attempts", "N-2: batch ordinals over every attempt, not the prepared ones",
  (W, "let requests: Vec<usize> = attempts.iter().filter(|attempt| attempt.prepared).map(|attempt| attempt.request).collect();",
      "let requests: Vec<usize> = attempts.iter().map(|attempt| attempt.request).collect();"))
m("M25_material_first_case_only", "D1.5: the material basis names case 0 only",
  (W, "\"case_indices\":(0..cases).collect::<Vec<_>>()", "\"case_indices\":[0]"))
m("M26_diag_refs_first_case_only", "D6a: a diagnostic naming several cases is referenced by the first only",
  (W, "            if affected.iter().any(|r| r == *case_id) {\n                refs.push(d[\"id\"].clone());\n            }",
      "            if affected.iter().any(|r| r == *case_id) {\n                refs.push(d[\"id\"].clone());\n                break;\n            }"))
m("M43_facade_code_kernel", "D4d: a refusal after a selected Run is coded as the kernel's",
  (W, "                (_, \"selected\") => (\"facade_certificate\".to_owned(), \"facade\"),\n                (_, other) => (format!(\"kernel_{other}\"), \"kernel\"),\n            };\n            let attempt_v",
      "                (_, \"selected\") => (\"kernel_selected\".to_owned(), \"kernel\"),\n                (_, other) => (format!(\"kernel_{other}\"), \"kernel\"),\n            };\n            let attempt_v"))

# ---- batch B (run after batch A) ----
m("B_S3_no_per_case_presence", "decision 19: the per-case final mode/parity presence check is dropped (R3′ S3)",
  (P, "if counts[index] != [1, usize::from(captured.parity_produced)] {", "if false && counts[index] != [1, usize::from(captured.parity_produced)] {"))
m("B_S4_finish_ignores_parked_late_hooks", "T-2/T-6: the final hook no longer checks the parked cases' late captures (R3′ S4)",
  (P, "|| self.parked.iter().any(|slot|slot.prepared_late_calls!=1", "|| false && self.parked.iter().any(|slot|slot.prepared_late_calls!=1"))
m("B_R3P4_every_slot_submitted", "R3P-4: the call submits every requested case's slot source, not the prepared attempts",
  (P, "        let called = {\n            let attempts = &*attempts;\n            capture.native_call(|| attempts.iter().filter(|attempt| matches!(attempt.end, AttemptEnd::Prepared)).map(|attempt| attempt.request))\n        };",
      "        let called = {\n            let n = capture.cases_seen();\n            capture.native_call(|| 0..n)\n        };"))
m("B_M53_notices_for_every_case", "T-5: a notice is reserved (and published) for every requested case, not A",
  (L, "let Some(notice) = ReservedNotices::reserve(&mut ordinary, attempted.ids()) else {", "let Some(notice) = ReservedNotices::reserve(&mut ordinary, cases.ids()) else {"))
m("B_M54_cause_preparation_before_native", "T-10: with no case selected, Preparation outranks Native",
  (L, "        } else if reached(|end| matches!(end, AttemptEnd::Native)) {\n            W1Fallback::Native\n        } else {\n            W1Fallback::Preparation\n        };",
      "        } else if reached(|end| matches!(end, AttemptEnd::Preparation)) {\n            W1Fallback::Preparation\n        } else {\n            W1Fallback::Native\n        };"))


def edits_ok(src, mid):
    why, edits = M[mid]
    texts = {}
    for f, old, new in edits:
        t = texts.get(f) or open(f"{src}/{f}", encoding="utf-8").read()
        if t.count(old) != 1:
            return False, f"{mid}: {t.count(old)} hits in {f}"
        texts[f] = t.replace(old, new)
    return True, texts


def main():
    if sys.argv[1] == "list":
        for k, (why, edits) in M.items():
            print(f"{k}\t{why}")
        return
    if sys.argv[1] == "check":
        bad = 0
        for k in M:
            ok, info = edits_ok(sys.argv[2], k)
            if not ok:
                bad += 1
                print(info)
        print(f"checked {len(M)}, bad {bad}")
        return
    src, action, mid = sys.argv[1], sys.argv[2], sys.argv[3]
    assert action == "apply"
    ok, texts = edits_ok(src, mid)
    if not ok:
        sys.exit(texts)
    for f, t in texts.items():
        open(f"{src}/{f}", "w", encoding="utf-8").write(t)
    print(f"applied {mid}")


if __name__ == "__main__":
    main()

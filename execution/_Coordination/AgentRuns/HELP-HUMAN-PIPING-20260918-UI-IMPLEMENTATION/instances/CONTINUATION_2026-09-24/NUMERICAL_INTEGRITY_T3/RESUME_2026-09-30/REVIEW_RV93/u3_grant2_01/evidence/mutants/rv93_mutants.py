#!/usr/bin/env python3
"""RV93 mutants on U3 grant 2: one textual edit each in RV93's disposable `mut` copy of the
candidate (664f8df7b7), PP `--lib` in the registered build, killed/survived, the failing tests,
and whether the kill was a compile error. The pristine file is restored and its sha256 checked
after every run. I61's eleven (re-expressed verbatim from its mutants.py) plus RV93's own.

Usage: rv93_mutants.py <mut core dir> <log dir> <target dir> [ids...]
"""
import hashlib, json, os, subprocess, sys, time
M, LOG, TD = sys.argv[1], sys.argv[2], sys.argv[3]
only = set(sys.argv[4:])
PP = os.path.join(M, "product_physics")
LIB = os.path.join(PP, "src/lib.rs")
PROD = os.path.join(PP, "src/retained_product.rs")
MEM = os.path.join(PP, "src/retained_memory.rs")
G2 = os.path.join(PP, "src/retained_tests_hooks/grant2.rs")

I61 = [
    ("I61_SV18_late_check_removed", LIB,
     "    } else if let Some(refusal) = observer.late_refusal().cloned() {\n        (ordinary, Err(W1Fallback::LateGate(refusal)))\n    } else {",
     "    } else {"),
    ("I61_SV18b_late_check_never_taken", LIB,
     "    } else if let Some(refusal) = observer.late_refusal().cloned() {",
     "    } else if let Some(refusal) = observer.late_refusal().cloned().filter(|_| false) {"),
    ("I61_B1_permitted_second_run", LIB,
     "    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));",
     "    let _extra = run_linear_static_preview_observed(request.to_owned(), solver_mode, Some(capture), &mut SourceRecoveryBudget::default(), None);\n    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));"),
    ("I61_Bp_no_permit_second_run", LIB,
     "    let result = run_linear_static_preview_captured(request, solver_mode, Some(capture), &mut budget);",
     "    let _extra = run_linear_static_preview_captured(request.to_owned(), solver_mode, Some(capture), &mut SourceRecoveryBudget::default());\n    let result = run_linear_static_preview_captured(request, solver_mode, Some(capture), &mut budget);"),
    ("I61_Bp_no_permit_custody_copy", LIB,
     "    let mut budget = SourceRecoveryBudget::default();\n    if pressure_runtime::is_exact(&request.model) {\n        budget.per_case_limit = PHYSICS_SOURCE_WORK_LIMIT;\n    }\n    #[cfg(test)]\n    retained_memory::tests::ordinary_dispatch_entered();",
     "    let mut budget = SourceRecoveryBudget::default();\n    if pressure_runtime::is_exact(&request.model) {\n        budget.per_case_limit = PHYSICS_SOURCE_WORK_LIMIT;\n    }\n    let _copy = capture.borrowed_raw().to_owned();\n    #[cfg(test)]\n    retained_memory::tests::ordinary_dispatch_entered();"),
    ("I61_Bp_permitted_owner_altered", LIB,
     "    (frozen.into_ordinary(), Ok(RetainedSuccessor(successor)))",
     "    ({ let mut o = frozen.into_ordinary(); o.diagnostics.pop(); o }, Ok(RetainedSuccessor(successor)))"),
    ("I61_F_preparation_without_notice", LIB,
     "        Err(failure) => return notice.publish(failure.ordinary, W1Fallback::Preparation),",
     "        Err(failure) => return (failure.ordinary, Err(W1Fallback::Preparation)),"),
    ("I61_GC_refusal_ignored", LIB,
     "            Some(Err(refusal)) => (ordinary, Err(W1Fallback::CompleteGate(refusal))),",
     "            Some(Err(_refusal)) => retained_w1(observer, ordinary, capture),"),
    ("I61_STACK_cause_lost", LIB,
     "Some(Err(W1Fallback::StackReservation))),",
     "None),"),
    ("I61_COEX_not_before_g_c", LIB,
     "    let (envelope, retained) = if ordinary.source_block_recovery.is_some() {",
     "    let (envelope, retained) = if ordinary.source_block_recovery.is_some() && false {"),
    ("I61_GB_refusal_not_recorded", PROD,
     "            if let Err(refusal)=permit.check_late(&facts) {self.late_refusal=Some(refusal);return;}",
     "            let _ = permit.check_late(&facts);"),
]
RV93 = [
    # (a) the successor-or-ordinary choice: the one publication is always the ordinary owner.
    ("RV93_A1_publication_always_ordinary", LIB,
     "            Some(Ok(successor)) => RetainedPublication::Successor(successor.0),",
     "            Some(Ok(_successor)) => RetainedPublication::Ordinary(self.envelope),"),
    # (a') the choice inverted: a fallback's ordinary bytes published as if a successor existed is
    # not typable; instead `successor()` hides a real successor while into_publication keeps it.
    ("RV93_A2_successor_accessor_hidden", LIB,
     "            Some(Ok(successor)) => Some(&successor.0),\n            _ => None,",
     "            Some(Ok(_successor)) => None,\n            _ => None,"),
    # (b') two notices with no allocation in the diagnostics vector (the last ordinary
    # diagnostic is overwritten by a copy of the notice, then the notice is appended), so the
    # product's capacity assertions hold: only a notice-count or byte assertion can kill it.
    ("RV93_B1b_notice_twice_no_alloc", LIB,
     "        ordinary.diagnostics.push(notice);\n",
     "        if let Some(last) = ordinary.diagnostics.last_mut() { *last = notice.clone(); }\n        ordinary.diagnostics.push(notice);\n"),
    # (b) the N1 notice count: two notices (the slot is reserved for one), and none at all on a
    # receipt-encoding serializer fallback only.
    ("RV93_B1_notice_twice", LIB,
     "        ordinary.diagnostics.push(notice);\n",
     "        ordinary.diagnostics.push(notice.clone());\n        ordinary.diagnostics.push(notice);\n"),
    ("RV93_B2_no_notice_on_serializer", LIB,
     "        Err(failure) => return notice.publish(frozen.into_ordinary(), W1Fallback::Serializer(failure)),",
     "        Err(failure) => return (frozen.into_ordinary(), Err(W1Fallback::Serializer(failure))),"),
    # (c) the fallback that returns the untouched ordinary envelope: the native fallback's owner
    # loses its last diagnostic before the notice (capacity unchanged, so no product assert fires).
    ("RV93_C1_native_fallback_owner_touched", LIB,
     "        return notice.publish(prepared.into_ordinary(), W1Fallback::Native);",
     "        return notice.publish({ let mut o = prepared.into_ordinary(); o.diagnostics.truncate(o.diagnostics.len().saturating_sub(1)); o }, W1Fallback::Native);"),
    ("RV93_C2_late_gate_fallback_gets_notice_slot", LIB,
     "        (ordinary, Err(W1Fallback::LateGate(refusal)))\n    } else {",
     "        ({ let mut o = ordinary; o.diagnostics.reserve(1); o.diagnostics.push(Diagnostic { id: \"x\".into(), code: \"RETAINED_PRECISION_UNAVAILABLE\".into(), severity: \"info\".into(), message: String::new(), source: None, affected_refs: vec![] }); o }, Err(W1Fallback::LateGate(refusal)))\n    } else {"),
    # (d) the dispatch count: an extra ordinary run on the no-permit path, spelled without
    # `.clone()`, `to_owned()` or `to_vec()` (the text guard's words).
    ("RV93_D1_no_permit_extra_run_unspelled", LIB,
     "    let result = run_linear_static_preview_captured(request, solver_mode, Some(capture), &mut budget);",
     "    let _extra = run_linear_static_preview_captured(serde_json::from_value::<LinearStaticPreviewRequest>(serde_json::from_str(&capture.borrowed_raw().to_string()).unwrap()).unwrap(), solver_mode, Some(capture), &mut SourceRecoveryBudget::default());\n    let result = run_linear_static_preview_captured(request, solver_mode, Some(capture), &mut budget);"),
    # (d') an extra ordinary run after the permitted work, on the caller thread.
    ("RV93_D2_permitted_extra_run_after", LIB,
     "        (Some(result), _) => result,",
     "        (Some(result), _) => { let _ = run_linear_static_preview_value_with_mode(capture.borrowed_raw().to_string().parse::<serde_json::Value>().unwrap(), solver_mode); result }"),
    # (e) the G-C unattempted-solve decline: the fact never observed (U4's file).
    ("RV93_E1_unattempted_fact_zero", MEM,
     "        o(P::OrdinarySolveNotAttempted, u64::from(!ordinary_solve_attempted(f.capture))),",
     "        o(P::OrdinarySolveNotAttempted, 0),"),
    # (e') the decline published with a notice (as if W1 work ran).
    ("RV93_E2_unattempted_decline_with_notice", LIB,
     "            Some(Err(refusal)) => (ordinary, Err(W1Fallback::CompleteGate(refusal))),",
     "            Some(Err(refusal)) => match w1_case_id(capture).map(|c| c.to_string()) { Some(case) => { let mut o = ordinary; match ReservedNotice::reserve(&mut o, &case) { Some(n) => n.publish(o, W1Fallback::CompleteGate(refusal)), None => (o, Err(W1Fallback::CompleteGate(refusal))) } } None => (ordinary, Err(W1Fallback::CompleteGate(refusal))) },"),
    # RV85 item 5: hook propagation broken (the work is not wrapped; faults stay on the caller).
    ("RV93_H1_hooks_not_carried", LIB,
     "    let carried = retained_tests_hooks::Carried::take();\n    move || {\n        let caller = carried.install();\n        let value = work();\n        retained_tests_hooks::hand_back(caller);\n        value\n    }",
     "    move || work()"),
    # The tally alone not carried (the counts would read 0 on the permitted path).
    ("RV93_H2_tally_not_installed", G2,
     "    pub(super) fn install(&mut self) { TALLY.with(|t| *t.borrow_mut() = self.0.take()); }",
     "    pub(super) fn install(&mut self) { let _ = self.0.take(); }"),
    # The G-B fault seam inert (would make the SV18 test vacuous if nothing else noticed).
    ("RV93_H3_late_fault_inert", G2,
     "    if consume(|a| std::mem::take(&mut a.late_gate)) {\n        exceed_every_bound(capture);\n    }",
     "    if consume(|a| std::mem::take(&mut a.late_gate)) {\n        let _ = capture;\n    }"),
    # The stack fallback publishes through the permitted cause list (cause changed to Coexistence).
    ("RV93_S1_stack_cause_wrong", LIB,
     "Some(Err(W1Fallback::StackReservation))),",
     "Some(Err(W1Fallback::Coexistence))),"),
    # Receipt-encoding detail lost from the notice text.
    ("RV93_R1_detail_dropped", LIB,
     "        token @ (\"work_counter_range\" | \"work_counter_inconsistent\" | \"saturation_not_excluded\" | \"publication_hash_range\") => Some(token),",
     "        _token @ (\"work_counter_range\" | \"work_counter_inconsistent\" | \"saturation_not_excluded\" | \"publication_hash_range\") => None,"),
    # Notice at the front of the diagnostics instead of after the ordinary prefix.
    ("RV93_N1_notice_first", LIB,
     "        ordinary.diagnostics.push(notice);\n",
     "        ordinary.diagnostics.insert(0, notice);\n"),
]
ENV = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
ENV.pop("RUSTFLAGS", None)
results = []
for mid, target, old, new in I61 + RV93:
    if only and mid not in only:
        continue
    pristine = open(target, "rb").read(); psha = hashlib.sha256(pristine).hexdigest()
    text = pristine.decode(); assert text.count(old) == 1, (mid, text.count(old))
    open(target, "w").write(text.replace(old, new))
    t0 = time.time()
    try:
        if subprocess.run(["ps", "-p", "5387"], capture_output=True).returncode != 0:
            sys.exit("MEMGUARD NOT RUNNING")
        proc = subprocess.run(["perl", "-e", "alarm shift; exec @ARGV", "1500", "cargo", "test", "--locked", "--offline", "--no-fail-fast",
                               "--lib", "--target-dir", TD], cwd=PP, env=ENV, capture_output=True, text=True)
    finally:
        open(target, "wb").write(pristine)
        assert hashlib.sha256(open(target, "rb").read()).hexdigest() == psha
    out = proc.stdout + proc.stderr
    open(os.path.join(LOG, f"mutant_{mid}.log"), "w").write(out)
    failing = sorted({l.split()[1] for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")})
    failing = [f for f in failing if f != "s11g_tests::t13_committed_fallback_uz_is_byte_identical"]
    compile_error = "error[" in out or "could not compile" in out
    summary = [l for l in out.splitlines() if l.startswith("test result:")]
    r = {"id": mid, "killed": bool(failing) or compile_error, "compile_error": compile_error,
         "killed_by_u3g2": [f for f in failing if "u3g2_" in f], "failing_count": len(failing),
         "failing": failing[:14], "result": summary[:1], "seconds": round(time.time() - t0, 1)}
    results.append(r); print(json.dumps(r), flush=True)
json.dump(results, open(os.path.join(LOG, "rv93_mutants.json" if not only else "rv93_mutants_" + "_".join(sorted(only)) + ".json"), "w"), indent=1)

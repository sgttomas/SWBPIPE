#!/usr/bin/env python3
"""I61 U3 grant 2 mutants: one textual edit each in the disposable `mut` tree (a copy of the
candidate), PP's whole `--lib` suite in the registered build, killed/survived, the failing tests,
and whether a compile error; pristine bytes restored and compared after every run.

Usage: mutants.py MUT_P_CORE LOG_DIR TARGET_DIR [ids...]
"""
import hashlib, json, os, subprocess, sys, time
M, LOG, TD = sys.argv[1], sys.argv[2], sys.argv[3]
only = set(sys.argv[4:])
PP = os.path.join(M, "product_physics")
LIB = os.path.join(PP, "src/lib.rs")
PROD = os.path.join(PP, "src/retained_product.rs")

MUTANTS = [
    # RV85 U4/N2: SV18 verbatim in intent (permitted_run's G-B outcome check removed), and a
    # variant that keeps the checked text but never takes the branch (structural guards pass).
    ("SV18_late_check_removed", LIB,
     "    } else if let Some(refusal) = observer.late_refusal().cloned() {\n        (ordinary, Err(W1Fallback::LateGate(refusal)))\n    } else {",
     "    } else {"),
    ("SV18b_late_check_never_taken", LIB,
     "    } else if let Some(refusal) = observer.late_refusal().cloned() {",
     "    } else if let Some(refusal) = observer.late_refusal().cloned().filter(|_| false) {"),
    # B-1/S-7: a second ordinary run on the permitted path (no `clone()` spelling, so N9's text
    # guard does not see it).
    ("B1_permitted_second_run", LIB,
     "    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));",
     "    let _extra = run_linear_static_preview_observed(request.to_owned(), solver_mode, Some(capture), &mut SourceRecoveryBudget::default(), None);\n    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));"),
    # B′ on the no-permit path: a second ordinary run, and a copy of the custody.
    ("Bp_no_permit_second_run", LIB,
     "    let result = run_linear_static_preview_captured(request, solver_mode, Some(capture), &mut budget);",
     "    let _extra = run_linear_static_preview_captured(request.to_owned(), solver_mode, Some(capture), &mut SourceRecoveryBudget::default());\n    let result = run_linear_static_preview_captured(request, solver_mode, Some(capture), &mut budget);"),
    ("Bp_no_permit_custody_copy", LIB,
     "    let mut budget = SourceRecoveryBudget::default();\n    if pressure_runtime::is_exact(&request.model) {\n        budget.per_case_limit = PHYSICS_SOURCE_WORK_LIMIT;\n    }\n    #[cfg(test)]\n    retained_memory::tests::ordinary_dispatch_entered();",
     "    let mut budget = SourceRecoveryBudget::default();\n    if pressure_runtime::is_exact(&request.model) {\n        budget.per_case_limit = PHYSICS_SOURCE_WORK_LIMIT;\n    }\n    let _copy = capture.borrowed_raw().to_owned();\n    #[cfg(test)]\n    retained_memory::tests::ordinary_dispatch_entered();"),
    # B′ on the permit path: the ordinary owner beside the successor is altered.
    ("Bp_permitted_owner_altered", LIB,
     "    (frozen.into_ordinary(), Ok(RetainedSuccessor(successor)))",
     "    ({ let mut o = frozen.into_ordinary(); o.diagnostics.pop(); o }, Ok(RetainedSuccessor(successor)))"),
    # Deliverable 1: a W1 fallback without its notice; a G-C refusal ignored; the stack cause lost;
    # coexistence not checked before G-C; G-B's refusal not recorded.
    ("F_preparation_without_notice", LIB,
     "        Err(failure) => return notice.publish(failure.ordinary, W1Fallback::Preparation),",
     "        Err(failure) => return (failure.ordinary, Err(W1Fallback::Preparation)),"),
    ("GC_refusal_ignored", LIB,
     "            Some(Err(refusal)) => (ordinary, Err(W1Fallback::CompleteGate(refusal))),",
     "            Some(Err(_refusal)) => retained_w1(observer, ordinary, capture),"),
    ("STACK_cause_lost", LIB,
     "Some(Err(W1Fallback::StackReservation))),",
     "None),"),
    ("COEX_not_before_g_c", LIB,
     "    let (envelope, retained) = if ordinary.source_block_recovery.is_some() {",
     "    let (envelope, retained) = if ordinary.source_block_recovery.is_some() && false {"),
    ("GB_refusal_not_recorded", PROD,
     "            if let Err(refusal)=permit.check_late(&facts) {self.late_refusal=Some(refusal);return;}",
     "            let _ = permit.check_late(&facts);"),
]
ENV = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
ENV.pop("RUSTFLAGS", None)
results = []
for mid, target, old, new in MUTANTS:
    if only and mid not in only:
        continue
    pristine = open(target, "rb").read(); psha = hashlib.sha256(pristine).hexdigest()
    text = pristine.decode(); assert text.count(old) == 1, (mid, text.count(old))
    open(target, "w").write(text.replace(old, new))
    t0 = time.time()
    try:
        if subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode != 0:
            sys.exit("MEMGUARD NOT RUNNING")
        proc = subprocess.run(["perl", "-e", "alarm shift; exec @ARGV", "1200", "cargo", "test", "--locked", "--offline", "--no-fail-fast",
                               "--lib", "--target-dir", TD], cwd=PP, env=ENV, capture_output=True, text=True)
    finally:
        open(target, "wb").write(pristine)
        assert hashlib.sha256(open(target, "rb").read()).hexdigest() == psha
    out = proc.stdout + proc.stderr
    open(os.path.join(LOG, f"mutant_{mid}.log"), "w").write(out)
    failing = sorted({l.split()[1] for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")})
    failing_new = [f for f in failing if "u3g2_" in f]
    # The Mac t13 (s11g_tests::t13_committed_fallback_uz_is_byte_identical) fails at base too; it
    # neither kills nor saves a mutant.
    failing = [f for f in failing if f != "s11g_tests::t13_committed_fallback_uz_is_byte_identical"]
    compile_error = "error[" in out or "could not compile" in out
    r = {"id": mid, "killed": bool(failing) or compile_error, "compile_error": compile_error,
         "killed_by_new_tests": failing_new, "failing_count": len(failing),
         "failing": failing[:12], "seconds": round(time.time() - t0, 1)}
    results.append(r); print(json.dumps(r), flush=True)
json.dump(results, open(os.path.join(LOG, "mutants.json" if not only else "mutants_" + "_".join(sorted(only)) + ".json"), "w"), indent=1)

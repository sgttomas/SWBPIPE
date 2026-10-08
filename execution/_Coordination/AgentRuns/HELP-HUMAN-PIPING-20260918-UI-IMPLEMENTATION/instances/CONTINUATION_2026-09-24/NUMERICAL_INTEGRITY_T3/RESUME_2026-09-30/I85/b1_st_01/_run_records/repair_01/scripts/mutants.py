#!/usr/bin/env python3
"""I85 B1-ST mutants (PLAN_v2 §2.1's five, and RV109's R10 and R16 from repair 1): one textual edit each in a disposable copy of the
candidate's PP crate sources (`mut`: a `git archive` of the candidate commit, P only), then PP's
whole `--lib` suite in the registered build through WT/tools/t3_cargo.sh. Records killed/survived,
the failing tests with their panic messages, and whether the build failed (a compile error is not
a kill). Pristine bytes are restored and compared after every run. After I68's u8_witnesses_01.

Usage: mutants.py MUT_P LOG_DIR TARGET_DIR T3_CARGO TMPDIR [ids...]
"""
import hashlib, json, os, re, subprocess, sys, time
M, LOG, TD, CARGO, TMP = sys.argv[1:6]
only = set(sys.argv[6:])
PP = os.path.join(M, "core/product_physics")
LIB = os.path.join(PP, "src/lib.rs")
MUTANTS = [
    # Keying T-4 on the seed's `initial` (a report Passed) instead of the published verdict.
    ("M1_key_on_initial", LIB,
     "        if verdict == Some(NumericalQualityStatus::ChecksPassed) {\n            CaseTrigger::NotRequired",
     "        if seed.is_some_and(|seed| matches!(&seed.initial, Some(retained_product::InitialSeed::Report { code, .. }) if code == \"NUMERICAL_INTEGRITY_CHECKS_PASSED\")) {\n            CaseTrigger::NotRequired"),
    # Looking the published verdict up by the case's request position instead of its id.
    ("M2_verdict_by_position", LIB,
     "    case_ids.iter().map(move |&case_id| {\n        let verdict = only_one(quality.cases.iter().filter(|entry| entry.basis_ref.ref_id == case_id)).map(|entry| entry.solve_quality);",
     "    case_ids.iter().enumerate().map(move |(position, &case_id)| {\n        let verdict = quality.cases.get(position).map(|entry| entry.solve_quality);"),
    # Dropping decision 21 (DN §4.3's exclusion).
    ("M3_drop_decision_21", LIB,
     "        } else if seed.is_some_and(dn_trigger_excluded) {",
     "        } else if seed.is_some_and(|_| false) {"),
    # A seedless case excluded (ruling 2 keeps it in A).
    ("M4_seedless_excluded", LIB,
     "        } else if seed.is_some_and(dn_trigger_excluded) {",
     "        } else if seed.is_none_or(dn_trigger_excluded) {"),
    # `NoTriggeredCase` appending a notice (reserved, then published).
    ("M5_no_triggered_case_notice", LIB,
     "        return (ordinary, Err(W1Fallback::NoTriggeredCase));",
     "        return match ReservedNotice::reserve(&mut ordinary, case_id) { Some(notice) => notice.publish(ordinary, W1Fallback::NoTriggeredCase), None => (ordinary, Err(W1Fallback::NoticeReservation)) };"),
    # Repair 1 (RV109 SF-1): RV109's R10, T-4 after R-2's reservation (the slot reserved, then dropped).
    ("R10_t4_after_reservation", LIB,
     """    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, &[case_id]).any(|trigger| trigger == CaseTrigger::Attempted) {
        return (ordinary, Err(W1Fallback::NoTriggeredCase));
    }
    // R-2: the notice's space is reserved before any W1 work starts.
    let Some(notice) = ReservedNotice::reserve(&mut ordinary, case_id) else {
        return (ordinary, Err(W1Fallback::NoticeReservation));
    };""",
     """    // R-2: the notice's space is reserved before any W1 work starts.
    let Some(notice) = ReservedNotice::reserve(&mut ordinary, case_id) else {
        return (ordinary, Err(W1Fallback::NoticeReservation));
    };
    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, &[case_id]).any(|trigger| trigger == CaseTrigger::Attempted) {
        drop(notice);
        return (ordinary, Err(W1Fallback::NoTriggeredCase));
    }"""),
    # Repair 1 (RV109 N-1): RV109's R16, not_required only when a seed also exists.
    ("R16_not_required_needs_seed", LIB,
     "        if verdict == Some(NumericalQualityStatus::ChecksPassed) {",
     "        if verdict == Some(NumericalQualityStatus::ChecksPassed) && seed.is_some() {"),
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
    print(json.dumps(results[-1]), flush=True)
json.dump(results, open(os.path.join(LOG, "mutants.json"), "w"), indent=1)

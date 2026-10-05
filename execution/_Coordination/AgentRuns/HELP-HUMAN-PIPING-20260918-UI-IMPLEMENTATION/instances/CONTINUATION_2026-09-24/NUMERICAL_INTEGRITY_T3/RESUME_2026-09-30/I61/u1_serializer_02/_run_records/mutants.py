#!/usr/bin/env python3
"""U1 grant 1 mutants: each applies one source change in the disposable mutant tree,
runs the U1/U2 tests, records killed/survived, and restores the candidate bytes."""
import subprocess, sys, os, hashlib, shutil
W = 'WT'
S = W + '/scratch/i61_u1_serializer_02'
M = S + '/mut/projects/chirality-piping/core/product_physics'
SRC = M + '/src/'
FK = S + '/mut/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/product_certificate/'
MUTANTS = [
 # G-i translation tables (U1 grant 2): one substitution per table.
 ('T01_budget_scope', 'retained_wire.rs', 'k::BudgetScope::Case => "case",', 'k::BudgetScope::Case => "invocation",'),
 ('T02_certificate_issue', 'retained_wire.rs', 'k::CertificateIssue::PairIdentity => "pair_identity",', 'k::CertificateIssue::PairIdentity => "shape",'),
 ('T03_wide_error', 'retained_wire.rs', 'k::WideError::NegativeSqrt => tag("negative_sqrt"),', 'k::WideError::NegativeSqrt => tag("non_finite"),'),
 ('T04_wide_unencodable', 'retained_wire.rs', 'k::WideError::CountRange(_) => e.unencodable("WideError.count_range"),', 'k::WideError::CountRange(_) => tag("non_finite"),'),
 ('T05_sum_error', 'retained_wire.rs', 'S::AccumulatorOverflow=>"accumulator_overflow"', 'S::AccumulatorOverflow=>"non_finite"'),
 ('T06_stop', 'retained_wire.rs', 'k::AttemptStop::Condition => tag("condition"),', 'k::AttemptStop::Condition => tag("structure"),'),
 ('T07_attempt_reason', 'retained_wire.rs', 'k::AttemptReason::Uc { body } => json!({"space":"attempt","tag":"uc","body":body}),', 'k::AttemptReason::Uc { body } => json!({"space":"attempt","tag":"theta","body":body}),'),
 ('T08_unresolved', 'retained_wire.rs', 'k::UnresolvedReason::Ceiling => tag("ceiling"),', 'k::UnresolvedReason::Ceiling => tag("exact_sum_span"),'),
 ('T09_refusal', 'retained_wire.rs', 'k::Refusal::Structure => json!({"space":"refusal","tag":"structure"}),', 'k::Refusal::Structure => json!({"space":"stop","tag":"structure"}),'),
 ('T10_block_step_sentinel', 'retained_wire.rs', 'if b.refusal.row == usize::MAX {', 'if b.refusal.row == 0 {'),
 ('T11_numeric_error', 'retained_wire.rs', 'k::NumericError::AxisBits => kind("axis_bits"),', 'k::NumericError::AxisBits => kind("binary64_range"),'),
 ('T12_view_issue', 'retained_wire.rs', 'k::ViewFailure::Ordering => kind("ordering"),', 'k::ViewFailure::Ordering => kind("body_bound"),'),
 ('T13_bridge_error', 'retained_wire.rs', 'k::BridgeFailure::MissingRadius(row) => json!({"kind":"missing_radius","row":row}),', 'k::BridgeFailure::MissingRadius(row) => json!({"kind":"row_identity","row":row}),'),
 ('T14_product_predicate', 'retained_wire.rs', 'k::ProductPredicate::DecimalSi=>"decimal_si"', 'k::ProductPredicate::DecimalSi=>"decimal_raw"'),
 ('T15_helper_error', 'retained_wire.rs', 'k::HelperFailure::Invariant=>json!({"kind":"invariant"})', 'k::HelperFailure::Invariant=>json!({"kind":"binary64_range"})'),
 ('T16_source_directional', 'retained_wire.rs', '            e.fail(ReceiptCheck::Association, "SourceError.directional_spring");\n', ''),
 ('T17_member_property', 'retained_wire.rs', 'k::MemberProperty::SecondMomentY => "second_moment_y",', 'k::MemberProperty::SecondMomentY => "second_moment_z",'),
 ('T18_origin_error', 'retained_wire.rs', 'k::OriginError::Capacity => json!({"kind":"capacity"}),', 'k::OriginError::Capacity => json!({"kind":"allocation"}),'),
 ('T19_capture_error', 'retained_wire.rs', 'rp::CaptureError::NativeUnavailable => json!({"kind":"native_unavailable"}),', 'rp::CaptureError::NativeUnavailable => json!({"kind":"prepared_attempt_consumed"}),'),
 ('T20_g5a_quantity_kind', 'retained_wire.rs', 'if *k <= 1 { json!(k) }', 'if *k <= 2 { json!(k) }'),
 ('T21_g5a_sanity', 'retained_wire.rs', 'json!({"kind":"sanity","body":body,', 'json!({"kind":"lower","body":body,'),
 ('T22_public_d38_capture', 'retained_wire.rs', 'None => json!({"kind":"capture","cause":capture_error(e,c)}),', 'None => json!({"kind":"native","run_ref":0}),'),
 ('T23_public_numeric', 'retained_wire.rs', 'rp::PreparedCandidateError::Numeric => json!({"kind":"numeric",', 'rp::PreparedCandidateError::Numeric => json!({"kind":"proof",'),
 ('T24_outcome_rejected', 'retained_wire.rs', 'k::AttemptOutcome::Rejected(r) => json!({"kind":"rejected"', 'k::AttemptOutcome::Rejected(r) => json!({"kind":"failed"'),
 ('T25_verification_reason', 'retained_wire.rs', '(_, k::AttemptOutcome::Failed(reason)) => ("failed", attempt_reason(e, reason)),', '(_, k::AttemptOutcome::Failed(_reason)) => ("failed", Value::Null),'),
 ('T26_kernel_terminal', 'retained_wire.rs', '(attempts, json!({"kind":"unresolved","reason":unresolved(e,reason)}))', '(attempts, json!({"kind":"refused","reason":unresolved(e,reason)}))'),
 ('T27_section_arithmetic', 'retained_wire.rs', 'k::SectionPreparationError::Arithmetic(c) => json!({"kind":"arithmetic","cause":numeric_error(e,c.cause())}),', 'k::SectionPreparationError::Arithmetic(_) => json!({"kind":"accounting"}),'),
 ('T28_structural_error', 'retained_wire.rs', 'StructuralError::Range(detail) => json!({"tag":"range","detail":detail}),', 'StructuralError::Range(detail) => json!({"tag":"invalid_input","detail":detail}),'),
 # U2 on the failure path.
 ('U01_seam_owner_check', 'retained_receipt.rs', '        if !work.owner_matches(owner) {return Err(TraceProjectionError::WorkAssociation);}\n', ''),
 ('U02_fk_anchor_not_kept', 'FK:final_case.rs', '        work.anchor=Some(std::sync::Arc::clone(&anchor));\n', ''),
 ('U03_serializer_failure_binding', 'retained_wire.rs', 'if r.proof_failure().is_some_and(|f| !bound(&|o| f.owner_matches(o))) {', 'if false && r.proof_failure().is_some_and(|f| !bound(&|o| f.owner_matches(o))) {'),
 ('U04_serializer_refusal_certificate_binding', 'retained_wire.rs', 'if r.certificate().is_some_and(|c| !bound(&|o| c.owner_matches(o))) {', 'if false && r.certificate().is_some_and(|c| !bound(&|o| c.owner_matches(o))) {'),
 # The unavailable representation and D38.
 ('V01_unselected_disclosure', 'retained_wire.rs', 'let disclosed = |d: &String| if selected {', 'let disclosed = |d: &String| if true {'),
 ('V02_facade_code', 'retained_wire.rs', '(_, "selected") => ("facade_certificate".to_owned(), "facade"),', '(_, "selected") => ("kernel_selected".to_owned(), "kernel"),'),
 ('V03_d38_code', 'retained_wire.rs', 'let code = ("source_unavailable", "preparation");', 'let code = ("kernel_refused", "kernel");'),
 ('V04_unavailable_diagnostic', 'retained_wire.rs', '"code":UNAVAILABLE_CODE,"severity":"info"', '"code":SELECTED_CODE,"severity":"info"'),
 ('V05_source_decline_fail_closed', 'retained_wire.rs', 'if matches!(failure, rr::FailureRef::Preparation { capture: rp::CaptureError::Source(_), .. }) {', 'if false {'),
 ('V06_prepared_source_fail_closed', 'retained_wire.rs', '            if view.prepared_source.is_some() {', '            if false {'),
 # RV82's survivors (its patches, verbatim where the text still exists) and S2.
 ('R06_f1_d5_ref_always_set', 'lib.rs', '                    linear.formation_check.is_some(),\n                );', '                    true,\n                );'),
 ('R08_run_after_check_removed_reexpressed', 'retained_wire.rs', '    e.sum(&[before, increment], "cases[].run.invocation_after") == after\n', '    let _ = (before, increment, after);\n    true\n'),
 ('R09_body_charged_check_removed', 'retained_wire.rs', 'if charged != e.exact(run.work.invocation_after(), "work.charged") {', 'if false && charged != e.exact(run.work.invocation_after(), "work.charged") {'),
 ('R10_g4_guard_removed', 'retained_wire.rs', 'if diags.iter().any(|d| d["code"] == json!(LEGACY_CODE) && names(d)) {', 'if false && diags.iter().any(|d| d["code"] == json!(LEGACY_CODE) && names(d)) {'),
 ('R13_initial_quality_crosscheck_removed', 'retained_wire.rs', 'if initial["kind"] == "report" && env', 'if false && initial["kind"] == "report" && env'),
 ('R14_omitted_code_check_removed', 'retained_wire.rs', 'if diags[i]["code"] != json!(LEGACY_CODE) || !names(&diags[i]) {', 'if !names(&diags[i]) {'),
 ('R22_safe_range_boundary', 'retained_wire.rs', 'let beyond = |amount: usize| u64::try_from(amount).map_or(true, |a| a > MAX_SAFE);', 'let beyond = |amount: usize| u64::try_from(amount).map_or(true, |a| a >= MAX_SAFE);'),
 ('S2_invocation_digest_check_removed', 'retained_wire.rs', 'if pc.invocation_digest.as_deref() != Some(invocation.borrowed_digest().as_str()) {', 'if false && pc.invocation_digest.as_deref() != Some(invocation.borrowed_digest().as_str()) {'),
 ('S2b_invocation_digest_not_recorded', 'retained_product.rs', '        self.invocation_digest = capture.map(|c| c.borrowed_digest().clone());\n', ''),
 ('V07_unavailable_no_method_token', 'retained_wire.rs', '    env["formulation_basis"]["profile_id"] = json!(RETAINED_PROFILE_ID);\n    let diags = env["diagnostics"].as_array_mut().ok_or(assoc("diagnostics"))?;\n    let id = format!("diagnostic:retained-precision:{case_id}:unavailable");', '    env["formulation_basis"]["profile_id"] = json!(RETAINED_PROFILE_ID);\n    for row in env["results"].as_array_mut().into_iter().flatten() { row["recovery_method"] = json!(METHOD); }\n    let diags = env["diagnostics"].as_array_mut().ok_or(assoc("diagnostics"))?;\n    let id = format!("diagnostic:retained-precision:{case_id}:unavailable");'),
]
def sha(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()
only = sys.argv[1:]
results = []
for name, f, old, new in MUTANTS:
    if only and name not in only: continue
    path = (FK + f[3:]) if f.startswith('FK:') else SRC + f; orig = open(path).read(); before = sha(path)
    assert orig.count(old) == 1, (name, orig.count(old))
    open(path, 'w').write(orig.replace(old, new))
    try:
        log = f'{S}/logs/mutant_{name}.log'
        cmd = ['perl', '-e', 'alarm shift; exec @ARGV', '1200', 'cargo', 'test', '--locked', '--offline', '--target-dir', W + '/targets/i61-u1/mut', '--lib', '--', 'u1_', 'u2_', 'u1g2', 'i61_certificate_prefixes']
        env = dict(os.environ, CARGO_BUILD_JOBS='4', RUST_TEST_THREADS='2')
        if subprocess.run(['pgrep', '-f', 'memguard.sh'], capture_output=True).returncode != 0: sys.exit('MEMGUARD NOT RUNNING')
        with open(log, 'w') as out:
            rc = subprocess.run(cmd, cwd=M, env=env, stdout=out, stderr=subprocess.STDOUT).returncode
        text = open(log).read()
        failed = sorted({l.split()[1] for l in text.splitlines() if l.startswith('test ') and l.endswith('FAILED')})
        compile_error = 'error[' in text or 'error: could not compile' in text
        verdict = 'KILLED' if rc != 0 else 'SURVIVED'
        line = f'{name} {verdict} rc={rc} compile_error={compile_error} failed={failed}'
    finally:
        open(path, 'w').write(orig)
        assert sha(path) == before
    print(line, flush=True); results.append(line)
open(f'{S}/logs/mutants_summary.txt', 'a').write('\n'.join(results) + '\n')

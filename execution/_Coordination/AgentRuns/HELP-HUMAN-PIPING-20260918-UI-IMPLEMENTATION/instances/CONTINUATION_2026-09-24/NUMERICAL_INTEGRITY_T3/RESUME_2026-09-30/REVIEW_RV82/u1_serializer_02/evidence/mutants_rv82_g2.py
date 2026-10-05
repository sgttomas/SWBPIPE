#!/usr/bin/env python3
"""RV82 grant-2 mutants on a clean `git archive` of b54caba7ab (WT/rv82/mut):
I61's grant-2 list verbatim (its filter), NONE controls, RV82's grant-1 survivors
re-run, and RV82's own grant-2 mutants (whole PP --lib, t13 excluded)."""
import subprocess, sys, os, hashlib
W = os.environ["WT"]; S = W + "/scratch/rv82_u1_serializer_02"
CORE = W + "/rv82/mut/projects/chirality-piping/core"
M = CORE + "/product_physics"
SRC = M + "/src/"
FK = CORE + "/solver/frame_kernel/src/structural/retained/product_certificate/"
KNOWN = {"s11g_tests::t13_committed_fallback_uz_is_byte_identical"}
I61_FILTER = ["u1_", "u2_", "u1g2", "i61_certificate_prefixes"]
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

GRANT1_SURVIVORS = [
 ("R06_f1_d5_ref_always_set", "lib.rs", "                    linear.formation_check.is_some(),\n                );", "                    true,\n                );"),
 ("R08_run_after_check_helper", "retained_wire.rs", '    e.sum(&[before, increment], "cases[].run.invocation_after") == after\n', '    let _ = (before, increment, after);\n    true\n'),
 ("R08b_run_after_check_call_site", "retained_wire.rs", '(after_conserved(e, before, increment, after), "cases[].run.invocation_after"),', '(true, "cases[].run.invocation_after"),'),
 ("R09_body_charged_check_removed", "retained_wire.rs", 'if charged != e.exact(run.work.invocation_after(), "work.charged") {', 'if false && charged != e.exact(run.work.invocation_after(), "work.charged") {'),
 ("R10_g4_guard_removed", "retained_wire.rs", 'if diags.iter().any(|d| d["code"] == json!(LEGACY_CODE) && names(d)) {', 'if false && diags.iter().any(|d| d["code"] == json!(LEGACY_CODE) && names(d)) {'),
 ("R13_initial_quality_crosscheck_removed", "retained_wire.rs", 'if initial["kind"] == "report" && env', 'if false && initial["kind"] == "report" && env'),
 ("R14_omitted_code_check_removed", "retained_wire.rs", 'if diags[i]["code"] != json!(LEGACY_CODE) || !names(&diags[i]) {', 'if !names(&diags[i]) {'),
 ("R22_safe_range_boundary", "retained_wire.rs", "let beyond = |amount: usize| u64::try_from(amount).map_or(true, |a| a > MAX_SAFE);", "let beyond = |amount: usize| u64::try_from(amount).map_or(true, |a| a >= MAX_SAFE);"),
 ("R24_w2_published_report_overwrites_initial", "retained_product.rs", "            _ if seed.initial.is_none() => {", "            _ => {"),
]
RV82_G2 = [
 ("G01_unresolved_work_accounting_carries_prior", "retained_wire.rs", 'json!({"space":"unresolved","tag":"work_accounting","fault":fault_tag(*fault)})', 'json!({"space":"unresolved","tag":"work_accounting","fault":fault_tag(*fault),"prior":null})'),
 ("G02_block_bound_swapped", "retained_wire.rs", 'k::CertifiedBound::Uc=>"uc", k::CertifiedBound::S=>"s"', 'k::CertifiedBound::Uc=>"s", k::CertifiedBound::S=>"uc"'),
 ("G03_source_spring_id_as_member_id", "retained_wire.rs", 'S::DuplicateSpringId { id: i } => id("duplicate_spring_id", i),', 'S::DuplicateSpringId { id: i } => id("duplicate_member_id", i),'),
 ("G04_native_with_run_as_capture", "retained_wire.rs", 'Some((_, case)) => json!({"kind":"native","run_ref":case.run}),', 'Some((_, _case)) => json!({"kind":"capture","cause":capture_error(e,c)}),'),
 ("G05_pre_anchor_work_accepted", "FK:final_case.rs", "self.anchor.as_ref().is_some_and(|anchor|anchor.matches_owner(owner))", "self.anchor.as_ref().is_none_or(|anchor|anchor.matches_owner(owner))"),
 ("G06_proof_failure_owner_always", "FK:final_case.rs", "pub fn owner_matches(&self,owner:&adaptive::RetainedSolve)->bool { self.work.owner_matches(owner) }", "pub fn owner_matches(&self,owner:&adaptive::RetainedSolve)->bool { let _=owner; true }"),
 ("G07_kernel_code_fixed", "retained_wire.rs", '(_, other) => (format!("kernel_{other}"), "kernel"),', '(_, _other) => ("kernel_unresolved".to_owned(), "kernel"),'),
 ("G08_g5a_quantity_kind_two", "retained_wire.rs", "if *k <= 1 { json!(k) }", "if *k <= 2 { json!(k) }"),
 ("G09_seam_non_selected_owner_passes", "retained_receipt.rs", "Some(k::ExecutionOutcome::Selected(owner))=>owner,_=>return Err(TraceProjectionError::WorkAssociation)};", "Some(k::ExecutionOutcome::Selected(owner))=>owner,_=>return Err(TraceProjectionError::MissingFailure)};"),
 ("G10_s2_digest_compares_mode_only", "retained_wire.rs", "if pc.invocation_digest.as_deref() != Some(invocation.borrowed_digest().as_str()) {", "if pc.invocation_digest.is_none() {"),
]
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
def path_of(f): return (FK + f[3:]) if f.startswith("FK:") else SRC + f
def run(name, f, old, new, full):
    path = path_of(f); orig = open(path).read(); before = sha(path)
    if old:
        assert orig.count(old) == 1, (name, orig.count(old))
        open(path, "w").write(orig.replace(old, new))
    try:
        log = f"{S}/logs/mutant_{name}.log"
        cmd = ["perl", "-e", "alarm shift; exec @ARGV", "1500", "cargo", "test", "--locked", "--offline", "--target-dir", W + "/targets/rv82/mut", "--lib"] + ([] if full else ["--"] + I61_FILTER)
        env = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
        if subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode != 0: sys.exit("MEMGUARD NOT RUNNING")
        with open(log, "w") as out:
            rc = subprocess.run(cmd, cwd=M, env=env, stdout=out, stderr=subprocess.STDOUT).returncode
        text = open(log).read()
        failed = sorted({l.split()[1] for l in text.splitlines() if l.startswith("test ") and l.endswith("FAILED")})
        compile_error = "error[" in text or "error: could not compile" in text
        result_line = [l for l in text.splitlines() if l.startswith("test result:")]
        real = [t for t in failed if t not in KNOWN]
        killed = compile_error or bool(real) or (rc != 0 and not failed)
        line = f"{name} {'KILLED' if killed else 'SURVIVED'} rc={rc} compile_error={compile_error} failed={real} {result_line[-1] if result_line else ''}"
    finally:
        open(path, "w").write(orig)
        assert sha(path) == before
    print(line, flush=True); return line
only = sys.argv[1:]
lines = []
plan = [("NONE", True, [("NONE_control", "retained_wire.rs", "", "")]), ("NONE i61-filter", False, [("NONE_control_filtered", "retained_wire.rs", "", "")]),
        ("I61", False, MUTANTS), ("RV82-g1", True, GRANT1_SURVIVORS), ("RV82-g2", True, RV82_G2)]
for group, full, items in plan:
    for name, f, old, new in items:
        if only and name not in only and group not in only: continue
        lines.append(f"[{group}] " + run(name, f, old, new, full))
open(f"{S}/mutants_rv82_g2_summary.txt", "a").write("\n".join(lines) + "\n")

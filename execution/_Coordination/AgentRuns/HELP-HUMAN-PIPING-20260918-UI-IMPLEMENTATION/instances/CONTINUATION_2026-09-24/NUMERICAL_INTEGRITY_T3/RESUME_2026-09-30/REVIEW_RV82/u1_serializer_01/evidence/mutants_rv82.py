#!/usr/bin/env python3
"""RV82 item 8: I61's 39 mutants verbatim (its list, copied from I61/u1_serializer_01/_run_records/mutants.py),
a NONE control, and RV82's own mutants, on a clean `git archive` of 59a5de2032 (WT/rv82/mut).
I61's mutants use I61's kill criterion (`--lib -- u1_ u2_`). RV82's own mutants and NONE run the whole
PP `--lib`; the known Mac platform failure t13 is excluded from the kill set."""
import subprocess, sys, os, hashlib
W = os.environ["WT"]; S = W + "/scratch/rv82_u1_serializer_01"
CORE = W + "/rv82/mut/projects/chirality-piping/core"
M = CORE + "/product_physics"
FILES = {"retained_wire.rs": M + "/src/retained_wire.rs", "retained_product.rs": M + "/src/retained_product.rs", "lib.rs": M + "/src/lib.rs",
         "final_case.rs": CORE + "/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs"}
KNOWN = {"s11g_tests::t13_committed_fallback_uz_is_byte_identical"}
MUTANTS = [
 ('M01_t1a_keep_disclosure', 'retained_wire.rs', 'successor_envelope(&mut env, &case_id, omit.as_deref())?;', 'successor_envelope(&mut env, &case_id, None)?;'),
 ('M02_t1a_disclosure_ref', 'retained_wire.rs', '(json!({"disposition":"unavailable","diagnostic_ref":null,"work_ref":work.len()-1}), Some(diagnostic_ref.clone()))', '(json!({"disposition":"unavailable","diagnostic_ref":diagnostic_ref,"work_ref":work.len()-1}), Some(diagnostic_ref.clone()))'),
 ('M03_gl_charged', 'retained_product.rs', 'charged: failure.work.charged,', 'charged: failure.work.rejected,'),
 ('M04_gl_rejected', 'retained_product.rs', 'rejected: failure.work.rejected, limit: failure.work.limit };', 'rejected: failure.work.charged, limit: failure.work.limit };'),
 ('M05_gl_limit', 'retained_product.rs', 'rejected: failure.work.rejected, limit: failure.work.limit };', 'rejected: failure.work.rejected, limit: failure.work.charged };'),
 ('M06_gl_stage', 'retained_product.rs', 'let work = LegacyWork { stage: failure.stage,', 'let work = LegacyWork { stage: "source closure",'),
 ('M07_gl_attempted', 'lib.rs', '            legacy_attempted = true;\n', ''),
 ('M08_helper_stage', 'retained_wire.rs', 'AttemptStage::SourceClosure => "source_closure",', 'AttemptStage::SourceClosure => "preparation",'),
 ('M09_d6a_retained_filter', 'retained_wire.rs', '.filter(|d| !d["code"].as_str().is_some_and(|c| c.starts_with("RETAINED_PRECISION_")))', '.filter(|_| true)'),
 ('M10_d6a_case_filter', 'retained_wire.rs', '.filter(|d| d["affected_refs"].as_array().is_some_and(|a| a.iter().any(|r| r == case_id.as_str())))', '.filter(|_| true)'),
 ('M11_d39_eligible_required', 'retained_product.rs', '            seed.legacy = Some(LegacySeed::NotEligible);', '            seed.legacy = Some(LegacySeed::NotRequired);'),
 ('M12_d39_decline_label', 'retained_wire.rs', '"disposition":"declined_without_attempt"', '"disposition":"unavailable"'),
 ('M13_d39_exact_selected', 'retained_wire.rs', 'Some(rp::LegacySeed::ExactSelected) => return Err(fail(ReceiptCheck::Scope, "ordinary_attempts[].legacy_source")),', 'Some(rp::LegacySeed::ExactSelected) => (json!({"disposition":"not_required","diagnostic_ref":null,"work_ref":null}), None),'),
 ('M14_u2_binding', 'retained_wire.rs', 'if !certificate.owner_matches(owner) {', 'if false && !certificate.owner_matches(owner) {'),
 ('M15_gb_d5_ref', 'lib.rs', '                    linear.formation_check.is_some(),\n                );', '                    false,\n                );'),
 ('M16_ge_not_covered', 'retained_wire.rs', 'None if matches!(recipe, k::ProductRecipe::NonQuantity | k::ProductRecipe::ModulusBasisRecord | k::ProductRecipe::DenseParityObservation) => {}', 'None if false => {}'),
 ('M17_gd_support_indices', 'retained_wire.rs', '"support_indices":owners}', '"support_indices":Vec::<usize>::new()}'),
 ('M18_gb_initial_outcome', 'retained_wire.rs', '"NUMERICAL_INTEGRITY_SENSITIVE" => json!("sensitive"),', '"NUMERICAL_INTEGRITY_SENSITIVE" => json!("checks_passed"),'),
 ('M19_gb_finding_disclosure', 'lib.rs', 'load_row_finding.is_some() && linear.structural_report.quality != SolveQuality::Sensitive,', 'load_row_finding.is_some(),'),
 ('M20_ga_method_token', 'retained_wire.rs', '            row["recovery_method"] = json!(METHOD);', '            row["recovery_method"] = json!("other");'),
 # ROOT correction (I65 D-4 §3): legacy-field substitutions and the stage-status gate.
 ('C01_legacy_shared', 'retained_wire.rs', 'shared: e.exact(r.checked_shared_work(), "run.records[].work.shared_lme"),', 'shared: r.shared_work,'),
 ('C02_legacy_stop_rule', 'retained_wire.rs', 'stop_rule: e.exact(r.checked_stop_rule_work(), "run.records[].work.stop_rule_lme"),', 'stop_rule: r.stop_rule_work,'),
 ('C03_legacy_verification', 'retained_wire.rs', 'verification: e.exact(r.checked_verification_work(), "run.records[].work.verification_lme"),', 'verification: r.verification_work,'),
 ('C04_legacy_verification_shared', 'retained_wire.rs', 'verification_shared: e.exact(r.checked_verification_shared_work(), "run.records[].work.verification_shared_lme"),', 'verification_shared: r.verification_shared_work,'),
 ('C05_unlatched_own', 'retained_wire.rs', 'own: e.exact(r.checked_own_work(), "run.records[].work.own_lme"),', 'own: e.sum(&[e.exact(r.work.checked_lme(), "w"), e.exact(r.k4_work.checked_lme(), "k")], "o"),'),
 ('C06_legacy_case_charge', 'retained_wire.rs', 'case_charge: e.exact(r.checked_case_charge(), "run.attempts[].case_charge"),', 'case_charge: e.sum(&[e.exact(r.work.checked_lme(), "w"), e.exact(r.k4_work.checked_lme(), "k"), r.shared_work, r.verification_shared_work], "c"),'),
 ('C07_legacy_invocation_increment', 'retained_wire.rs', 'invocation_increment: e.exact(r.checked_invocation_increment(), "run.attempts[].invocation_increment"),', 'invocation_increment: e.sum(&[e.exact(r.work.checked_lme(), "w"), e.exact(r.k4_work.checked_lme(), "k"), if r.shared_built_here { r.shared_work } else { 0 }, if r.verification_shared_built_here { r.verification_shared_work } else { 0 }], "i"),'),
 ('C08_stage_status_gate', 'retained_wire.rs', 'if let Err(fault) = s.checked_total().exact() {', 'if let Err(fault) = Ok::<u64, k::WorkFault>(0) {'),
 ('C09_legacy_meter', 'retained_wire.rs', 'let charged = e.exact(inv.meter().checked_charged(), "work.charged");', 'let charged = inv.meter().charged();'),
 # D-4 (ROOT, NUM fe38ea55bc): the wire vocabulary, exact-or-abandon counts, legacy rejected, conservation.
 ('D01_wire_overflow_token', 'retained_wire.rs', 'Self::WorkCounter(k::WorkFault::Overflow) => "work_counter_range",', 'Self::WorkCounter(k::WorkFault::Overflow) => "work_counter_overflow",'),
 ('D02_wire_both_token', 'retained_wire.rs', 'Self::WorkCounter(k::WorkFault::Inconsistent | k::WorkFault::Both) => "work_counter_inconsistent",', 'Self::WorkCounter(k::WorkFault::Inconsistent) => "work_counter_inconsistent", Self::WorkCounter(k::WorkFault::Both) => "work_counter_range",'),
 ('D03_wire_saturation_token', 'retained_wire.rs', 'Self::SaturationNotExcluded => "saturation_not_excluded",', 'Self::SaturationNotExcluded => "work_counter_range",'),
 ('D04_rejected_cause', 'retained_wire.rs', 'e.fail(ReceiptCheck::SaturationNotExcluded, "legacy_source_work[].rejected");', 'e.fail(ReceiptCheck::WorkCounterRange, "legacy_source_work[].rejected");'),
 ('D05_count_not_strict', 'retained_wire.rs', '                self.fail(ReceiptCheck::WorkCounter(f), "Count.value");\n', ''),
 ('D06_own_stages_check', 'retained_wire.rs', 'inconsistent(own_stages == work.own, "run.records[].work.own_stages");', 'inconsistent(true, "run.records[].work.own_stages");'),
 ('D07_shared_stages_check', 'retained_wire.rs', 'inconsistent(shared_stages == e.sum(&[work.shared, work.verification_shared], "run.records[].work.shared_stages"), "run.records[].work.shared_stages");', 'inconsistent(true, "run.records[].work.shared_stages");'),
 ('D08_d_plus_q_check', 'retained_wire.rs', 'inconsistent(e.sum(&[work.stop_rule, work.verification], "run.records[].work.stop_rule_lme") <= work.own, "run.records[].work.stop_rule_lme");', 'inconsistent(true, "run.records[].work.stop_rule_lme");'),
 ('D09_run_case_check', 'retained_wire.rs', '(total("case_charge", "cases[].run.case_charge") == case, "cases[].run.case_charge"),', '(true, "cases[].run.case_charge"),'),
 ('D10_run_increment_check', 'retained_wire.rs', '(total("invocation_increment", "cases[].run.invocation_increment") == increment, "cases[].run.invocation_increment"),', '(true, "cases[].run.invocation_increment"),'),
]

RV82 = [
 ("NONE_control", "retained_wire.rs", "", ""),
 ("R01_gb_report_from_wrong_diagnostic", "lib.rs", "if let (Some(observer), Some(record)) = (product.as_deref_mut(), diagnostics.last()) {\n                observer.ordinary_report(", "if let (Some(observer), Some(record)) = (product.as_deref_mut(), diagnostics.first()) {\n                observer.ordinary_report("),
 ("R02a_d39_not_required_folded_at_capture", "retained_product.rs", "            seed.legacy = Some(LegacySeed::NotRequired);", "            seed.legacy = Some(LegacySeed::NotEligible);"),
 ("R02b_d39_not_required_folded_on_wire", "retained_wire.rs", 'Some(rp::LegacySeed::NotRequired) => (json!({"disposition":"not_required"', 'Some(rp::LegacySeed::NotRequired) => (json!({"disposition":"not_eligible"'),
 ("R03_d6a_admits_retained_selected", "retained_wire.rs", 'c.starts_with("RETAINED_PRECISION_")', 'c.starts_with("RETAINED_PRECISION_UNAVAILABLE")'),
 ("R04_u2_owner_by_public_facts", "final_case.rs", "{self.anchor.matches_owner(owner)}", "{self.anchor.owner.prep.identity==owner.prep.identity}"),
 ("R05_hash_over_noncanonical_text", "retained_wire.rs", "canonical_json_checked_v1_text(&text).ok().map(|canon| sha_hex(canon.as_bytes()))", "{ let _ = canonical_json_checked_v1_text; Some(sha_hex(text.as_bytes())) }"),
 ("R06_f1_d5_ref_always_set", "lib.rs", "                    linear.formation_check.is_some(),\n                );", "                    true,\n                );"),
 ("R07_a1_hash_of_wrong_bytes", "retained_wire.rs", '"retained_state_sha256":sha_hex(&ev.retained_state_encoding),', '"retained_state_sha256":sha_hex(&ev.ledger_encoding),'),
 ("R08_run_after_check_removed", "retained_wire.rs", '(e.sum(&[before, increment], "cases[].run.invocation_after") == after, "cases[].run.invocation_after"),', '(true, "cases[].run.invocation_after"),'),
 ("R09_body_charged_check_removed", "retained_wire.rs", 'if charged != e.exact(run.work.invocation_after(), "work.charged") {', 'if false && charged != e.exact(run.work.invocation_after(), "work.charged") {'),
 ("R10_g4_guard_removed", "retained_wire.rs", "if diags.iter().any(|d| d[\"code\"] == json!(LEGACY_CODE) && names(d)) {", "if false && diags.iter().any(|d| d[\"code\"] == json!(LEGACY_CODE) && names(d)) {"),
 ("R13_initial_quality_crosscheck_removed", "retained_wire.rs", 'if initial["kind"] == "report" && env', 'if false && initial["kind"] == "report" && env'),
 ("R14_omitted_code_check_removed", "retained_wire.rs", "if diags[i][\"code\"] != json!(LEGACY_CODE) || !names(&diags[i]) {", "if !names(&diags[i]) {"),
 ("R17_legacy_attempted_always_true", "lib.rs", "let mut legacy_attempted = false;", "let mut legacy_attempted = true;"),
 ("R18_gd_any_support_at_node", "retained_wire.rs", "*node == c.dof.node as usize && fixed[c.dof.component.index()]", "*node == c.dof.node as usize && fixed.len() == 6"),
 ("R22_safe_range_boundary", "retained_wire.rs", "let beyond = |amount: usize| u64::try_from(amount).map_or(true, |a| a > MAX_SAFE);", "let beyond = |amount: usize| u64::try_from(amount).map_or(true, |a| a >= MAX_SAFE);"),
 ("R23_ga_selected_severity", "retained_wire.rs", '"code":SELECTED_CODE,"severity":"info"', '"code":SELECTED_CODE,"severity":"warning"'),
 ("R24_w2_published_report_overwrites_initial", "retained_product.rs", "            _ if seed.initial.is_none() => {", "            _ => {"),
]
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
def run(name, f, old, new, full):
    path = FILES[f]; orig = open(path).read(); before = sha(path)
    if old:
        assert orig.count(old) == 1, (name, orig.count(old))
        open(path, "w").write(orig.replace(old, new))
    try:
        log = f"{S}/logs/mutant_{name}.log"
        filt = [] if full else ["--", "u1_", "u2_"]
        cmd = ["perl", "-e", "alarm shift; exec @ARGV", "1500", "cargo", "test", "--locked", "--offline", "--target-dir", W + "/targets/rv82/mut", "--lib"] + filt
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
        verdict = "KILLED" if killed else "SURVIVED"
        line = f"{name} {verdict} rc={rc} compile_error={compile_error} failed={real} {result_line[-1] if result_line else ''}"
    finally:
        open(path, "w").write(orig)
        assert sha(path) == before
    print(line, flush=True); return line
only = sys.argv[1:]
lines = []
for group, full in (("NONE", True), ("NONE", False), ("I61", False), ("RV82", True)):
    items = [m for m in (RV82[:1] if group == "NONE" else MUTANTS if group == "I61" else RV82[1:])]
    for name, f, old, new in items:
        if only and name not in only and group not in only: continue
        lines.append(f"[{group}{'' if full else ' u1_/u2_'}] " + run(name if full or group != "NONE" else name + "_filtered", f, old, new, full))
        if group == "I61":
            # NONE with I61's own filter as well, once.
            pass
open(f"{S}/mutants_rv82_summary.txt", "a").write("\n".join(lines) + "\n")

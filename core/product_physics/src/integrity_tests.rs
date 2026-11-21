//! Carrier revision 4 §3 and §7: the pair owner, and one test per bypass kind.
//! Each bypass reaches a checkpoint; the test asserts the typed withdrawal, no
//! projected record, and unchanged diagnostic bytes. These tests show the
//! guard's behaviour; they do not prove that every future write uses the owner.

use super::*;
use open_pipe_stress_frame_kernel::structural::{synthetic, BoundStage, StageKey};
use std::cell::Cell;

thread_local! {
    /// Set by `AllowWithdrawal` while a test deliberately causes a disagreement.
    pub(crate) static ALLOW_WITHDRAWAL: Cell<bool> = const { Cell::new(false) };
}

/// While alive, a checkpoint's withdrawal does not panic.
pub(crate) struct AllowWithdrawal;
impl AllowWithdrawal {
    pub(crate) fn new() -> Self {
        ALLOW_WITHDRAWAL.with(|allow| allow.set(true));
        Self
    }
}
impl Drop for AllowWithdrawal {
    fn drop(&mut self) {
        ALLOW_WITHDRAWAL.with(|allow| allow.set(false));
    }
}

/// A test's hand-built diagnostics, with each integrity diagnostic written
/// through the owner in place: the same vector, order and bytes, and the sink
/// a product path would have left beside it.
pub(crate) fn through_owner(built: Vec<Diagnostic>) -> (Vec<Diagnostic>, InverseNormSink) {
    let mut diagnostics = Vec::new();
    let mut sink = InverseNormSink::default();
    for diagnostic in built {
        match case_of(&diagnostic).map(str::to_string) {
            Some(case_id) => publish(&mut diagnostics, &mut sink, &case_id, diagnostic, EntryUpdate::None),
            None => diagnostics.push(diagnostic),
        }
    }
    (diagnostics, sink)
}

fn integrity(case_id: &str, code: &str) -> Diagnostic {
    crate::diag(&crate::integrity_diagnostic_id(case_id), code, "info", format!("Load case {case_id}: report"), vec![case_id.to_string()])
}

fn record() -> InverseNormRecord {
    synthetic::record(StageKey::new(BoundStage::FormationCheck, None), None, synthetic::verified(3, 1), 7, 1.0, None)
}

/// Two cases written through the owner, with an unrelated diagnostic between.
fn published() -> (Vec<Diagnostic>, InverseNormSink) {
    let mut diagnostics = Vec::new();
    let mut sink = InverseNormSink::default();
    publish(&mut diagnostics, &mut sink, "load:A", integrity("load:A", "NUMERICAL_INTEGRITY_CHECKS_PASSED"), EntryUpdate::Linear(Some(record())));
    diagnostics.push(crate::diag("diagnostic:other", "OTHER", "info", "unrelated", Vec::new()));
    publish(&mut diagnostics, &mut sink, "load:B", integrity("load:B", "NUMERICAL_INTEGRITY_SENSITIVE"), EntryUpdate::Linear(None));
    (diagnostics, sink)
}

const AB: &[&str] = &["load:A", "load:B"];

fn case<'e>(evidence: &'e InverseNormEvidence, id: &str) -> &'e CaseEvidence {
    evidence.cases.iter().find(|c| c.case_id == id).unwrap()
}

fn bytes(diagnostics: &[Diagnostic]) -> String {
    serde_json::to_string(diagnostics).unwrap()
}

#[test]
fn owner_writes_keep_the_pair_in_agreement() {
    let (mut diagnostics, mut sink) = published();
    update_entry(&mut sink, "load:A", EntryUpdate::Basis(PublishedBasis::Linear));
    assert!(amend(&mut diagnostics, &mut sink, "load:A", |d| {
        d.message.push_str(" amended");
        true
    }, EntryUpdate::None));
    reconcile(&diagnostics, &mut sink);
    let evidence = sink.projection(&diagnostics, AB);
    assert_eq!(evidence.cases.len(), 2);
    let a = case(&evidence, "load:A");
    assert_eq!(a.withdrawn, None);
    assert_eq!(a.verdict, VerdictMirror::ChecksPassed);
    assert!(a.linear.is_some(), "the record is projected");
    assert_eq!(a.required, vec![inverse_norm_evidence::linear_requirement()]);
    assert_eq!(case(&evidence, "load:B").verdict, VerdictMirror::Sensitive);
}

#[test]
fn a_no_op_amendment_changes_neither_side() {
    let (mut diagnostics, mut sink) = published();
    let before = bytes(&diagnostics);
    let mirror = sink.entries[0].mirror;
    assert!(!amend(&mut diagnostics, &mut sink, "load:A", |_| false, EntryUpdate::Basis(PublishedBasis::Linear)));
    assert_eq!(bytes(&diagnostics), before);
    assert_eq!(sink.entries[0].mirror, mirror);
    assert_eq!(sink.entries[0].published_basis, PublishedBasis::None, "no entry update on a no-op");
    assert!(!amend(&mut diagnostics, &mut sink, "load:none", |_| true, EntryUpdate::None), "no diagnostic, nothing changes");
    reconcile(&diagnostics, &mut sink);
}

#[test]
fn clear_drops_both_sides() {
    let (mut diagnostics, mut sink) = published();
    clear(&mut diagnostics, &mut sink);
    assert!(diagnostics.is_empty());
    reconcile(&diagnostics, &mut sink);
    assert!(sink.projection(&diagnostics, AB).cases.iter().all(|c| c.verdict == VerdictMirror::NotAssessed && c.linear.is_none()));
}

/// Runs `bypass`, reaches the checkpoint, and returns the projection.
fn after_bypass(bypass: impl FnOnce(&mut Vec<Diagnostic>)) -> (InverseNormEvidence, String, String) {
    after_bypass_in(AB, bypass)
}

fn after_bypass_in(cases: &[&str], bypass: impl FnOnce(&mut Vec<Diagnostic>)) -> (InverseNormEvidence, String, String) {
    let _allow = AllowWithdrawal::new();
    let (mut diagnostics, mut sink) = published();
    bypass(&mut diagnostics);
    let published_bytes = bytes(&diagnostics);
    reconcile(&diagnostics, &mut sink);
    assert_eq!(bytes(&diagnostics), published_bytes, "reconcile never changes a published byte");
    (sink.projection(&diagnostics, cases), published_bytes, bytes(&diagnostics))
}

#[test]
fn an_id_alias_edit_is_withdrawn() {
    let (evidence, ..) = after_bypass(|diagnostics| {
        let alias = crate::integrity_diagnostic_id("load:A");
        if let Some(d) = diagnostics.iter_mut().find(|d| d.id == alias) {
            d.message = format!("{} bypass", d.message);
        }
    });
    let a = case(&evidence, "load:A");
    assert_eq!(a.withdrawn, Some(WithdrawalView::MirrorMismatch));
    assert!(a.linear.is_none(), "a withdrawn case projects no record");
    assert_eq!(case(&evidence, "load:B").withdrawn, None, "the other case is untouched");
}

#[test]
fn a_last_mut_severity_edit_is_withdrawn() {
    let (evidence, ..) = after_bypass(|diagnostics| {
        let last = diagnostics.last_mut().unwrap();
        last.severity = "info".into();
        last.message = format!("Rejected ordinary attempt. {}", last.message);
    });
    assert_eq!(case(&evidence, "load:B").withdrawn, Some(WithdrawalView::MirrorMismatch));
}

#[test]
fn a_retain_removal_is_withdrawn() {
    let (evidence, ..) = after_bypass(|diagnostics| {
        let alias = crate::integrity_diagnostic_id("load:A");
        diagnostics.retain(|d| d.id != alias);
    });
    assert_eq!(case(&evidence, "load:A").withdrawn, Some(WithdrawalView::OrphanEntry));
}

#[test]
fn an_extra_push_under_an_integrity_id_is_withdrawn() {
    let (evidence, ..) = after_bypass_in(&["load:A", "load:B", "load:C"], |diagnostics| {
        diagnostics.push(integrity("load:A", "NUMERICAL_INTEGRITY_CHECKS_PASSED"));
        diagnostics.push(integrity("load:C", "NUMERICAL_INTEGRITY_CHECKS_PASSED"));
    });
    assert_eq!(case(&evidence, "load:A").withdrawn, Some(WithdrawalView::DuplicateDiagnostic));
    let c = case(&evidence, "load:C");
    assert_eq!(c.withdrawn, Some(WithdrawalView::MissingEntry), "a diagnostic with no entry never publishes evidence");
    assert!(c.linear.is_none() && c.confirmation.is_none());
}

#[test]
fn a_reconstruction_with_an_empty_sink_is_withdrawn() {
    let _allow = AllowWithdrawal::new();
    let (diagnostics, _) = published();
    let copied = diagnostics.clone();
    let mut empty = InverseNormSink::default();
    reconcile(&copied, &mut empty);
    let evidence = empty.projection(&copied, AB);
    assert!(evidence.cases.iter().all(|c| c.withdrawn == Some(WithdrawalView::MissingEntry)));
    assert_eq!(evidence.cases.len(), 2);
}

#[test]
#[should_panic(expected = "integrity pair disagreement")]
fn a_disagreement_panics_in_tests_unless_allowed() {
    let (mut diagnostics, mut sink) = published();
    diagnostics[0].code = "NUMERICAL_INTEGRITY_SENSITIVE".into();
    reconcile(&diagnostics, &mut sink);
}

#[test]
fn a_cloned_pair_stays_in_agreement() {
    let (diagnostics, sink) = published();
    let (copied, mut copied_sink) = (diagnostics.clone(), sink.clone());
    reconcile(&copied, &mut copied_sink);
    assert!(copied_sink.projection(&copied, AB).cases.iter().all(|c| c.withdrawn.is_none()));
}

#[test]
fn every_published_case_has_exactly_one_entry_in_case_order() {
    let (diagnostics, mut sink) = published();
    reconcile(&diagnostics, &mut sink);
    let evidence = sink.projection(&diagnostics, &["load:B", "load:Z", "load:A"]);
    let ids: Vec<&str> = evidence.cases.iter().map(|c| c.case_id.as_str()).collect();
    assert_eq!(ids, ["load:B", "load:Z", "load:A"], "the document's case order, one each");
    let z = case(&evidence, "load:Z");
    assert_eq!(z.verdict, VerdictMirror::NotAssessed, "no integrity diagnostic");
    assert!(z.linear.is_none() && z.confirmation.is_none() && z.withdrawn.is_none() && z.required.is_empty());
}

#[test]
fn an_unknown_integrity_code_is_withdrawn_and_failure_codes_are_closed() {
    let _allow = AllowWithdrawal::new();
    let mut diagnostics = Vec::new();
    let mut sink = InverseNormSink::default();
    publish(&mut diagnostics, &mut sink, "load:A", integrity("load:A", "NUMERICAL_INTEGRITY_SOMETHING_NEW"), EntryUpdate::Linear(Some(record())));
    publish(&mut diagnostics, &mut sink, "load:B", integrity("load:B", "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY"), EntryUpdate::None);
    reconcile(&diagnostics, &mut sink);
    let evidence = sink.projection(&diagnostics, AB);
    let a = case(&evidence, "load:A");
    assert_eq!(a.withdrawn, Some(WithdrawalView::UnknownCode));
    assert!(a.linear.is_none());
    assert_eq!(case(&evidence, "load:B").verdict, VerdictMirror::Failed { code: IntegrityFailureCode::NegativeEnergy });
    assert_eq!(case(&evidence, "load:B").withdrawn, None);
}

#[test]
fn an_inspection_only_diagnostic_carries_its_prior_outcome_and_a_malformed_one_is_withdrawn() {
    let _allow = AllowWithdrawal::new();
    let mut diagnostics = Vec::new();
    let mut sink = InverseNormSink::default();
    let mut good = integrity("load:A", "NUMERICAL_INTEGRITY_INSPECTION_ONLY");
    good.message = format!("Inspection only (prior_outcome=NUMERICAL_INTEGRITY_SENSITIVE): this document is blocked (MODEL_INCOMPLETE). {}", good.message);
    let mut bad = integrity("load:B", "NUMERICAL_INTEGRITY_INSPECTION_ONLY");
    bad.message = format!("Inspection only (prior_outcome=NUMERICAL_INTEGRITY_FAILED): {}", bad.message);
    publish(&mut diagnostics, &mut sink, "load:A", good, EntryUpdate::Linear(Some(record())));
    publish(&mut diagnostics, &mut sink, "load:B", bad, EntryUpdate::Linear(Some(record())));
    reconcile(&diagnostics, &mut sink);
    let evidence = sink.projection(&diagnostics, AB);
    let a = case(&evidence, "load:A");
    assert_eq!(a.verdict, VerdictMirror::InspectionOnly { prior: PriorVerdict::Sensitive });
    assert_eq!(a.withdrawn, None);
    assert!(a.linear.is_some(), "the retained record is kept, not discarded");
    let b = case(&evidence, "load:B");
    assert_eq!(b.withdrawn, Some(WithdrawalView::UnknownCode), "a prior outside the two values is never guessed");
    assert!(b.linear.is_none());
}

#[test]
fn the_inspection_only_grammar_accepts_exactly_two_priors_at_the_start_and_nothing_near_them() {
    let ok = |m: &str| inspection_prior(m);
    assert_eq!(ok("Inspection only (prior_outcome=NUMERICAL_INTEGRITY_CHECKS_PASSED): blocked. x"), Some(PriorVerdict::ChecksPassed));
    assert_eq!(ok("Inspection only (prior_outcome=NUMERICAL_INTEGRITY_SENSITIVE): blocked. x"), Some(PriorVerdict::Sensitive));
    for near in [
        "inspection only (prior_outcome=NUMERICAL_INTEGRITY_CHECKS_PASSED): x", // case
        "Inspection only(prior_outcome=NUMERICAL_INTEGRITY_CHECKS_PASSED): x",  // missing space
        "Inspection only (prior_outcome= NUMERICAL_INTEGRITY_CHECKS_PASSED): x", // space in token
        "Inspection only (prior_outcome=NUMERICAL_INTEGRITY_CHECKS_PASSED: x",   // no closing parenthesis
        "Inspection only (prior_outcome=NUMERICAL_INTEGRITY_FAILED): x",         // another code
        "Inspection only (prior_outcome=NUMERICAL_INTEGRITY_INSPECTION_ONLY): x",
        "Inspection only (prior_outcome=CHECKS_PASSED): x",
        "Inspection only (prior_outcome=): x",
        " Inspection only (prior_outcome=NUMERICAL_INTEGRITY_SENSITIVE): x",    // not at the start
        "Load case c: Inspection only (prior_outcome=NUMERICAL_INTEGRITY_SENSITIVE): x",
        "",
    ] {
        assert_eq!(ok(near), None, "{near:?}");
    }
}

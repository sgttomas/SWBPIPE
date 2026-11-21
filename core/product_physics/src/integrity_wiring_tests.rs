//! Carrier revision 5 §3, §4 and §6, wired: what the document's evidence says
//! on the product's own paths, against DEL-04-07 *Published displacement*'s
//! single publication-basis decision. This build has no verifier, so no record
//! exists and every attempted stage reports `NoVerifierInThisBuild`: not
//! attempted, neither failed nor verified. These tests read the projection; the
//! published bytes are the existing tests' concern, and the projection never
//! enters them (`#[serde(skip)]`).

use super::tests::{through_owner, AllowWithdrawal};
use super::*;
use crate::inverse_norm_evidence::{
    AttemptState, DiscardTag, EstablishmentView, HeldOutcomeView, PlanView, ProgressView, PublicationView, StageTag,
};
use crate::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use open_pipe_stress_frame_kernel::structural::DisplacementFailureKind;
use serde_json::Value;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive];

fn demo() -> Value {
    serde_json::json!({
        "model": serde_json::from_str::<Value>(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap(),
        "materials": [],
    })
}

fn run(request: &Value, mode: PreviewSolverMode, forced: Option<(usize, DisplacementFailureKind)>) -> crate::MechanicsEnvelope {
    crate::corrected_displacement_tests::FORCED_CORRECTION_FAILURE.with(|f| f.set(forced));
    let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode);
    crate::corrected_displacement_tests::FORCED_CORRECTION_FAILURE.with(|f| f.set(None));
    envelope.unwrap()
}

/// Every attempted stage of a confirmation that ran: no record, and the
/// not-run-verifier progress, never an attempt or a failure.
fn assert_no_verifier(view: &crate::inverse_norm_evidence::ConfirmationView, label: &str) {
    assert_eq!(view.attempt, AttemptState::Attempted, "{label}");
    let PlanView::Known(stages) = &view.plan else { panic!("{label}: plan not determined") };
    assert!(!stages.is_empty(), "{label}");
    assert_eq!(stages.last().unwrap().stage.stage, StageTag::ConfirmationStageB, "{label}");
    for stage in stages {
        assert_eq!(stage.progress, ProgressView::NoVerifierInThisBuild, "{label}: {stage:?}");
        assert!(stage.record.is_none(), "{label}: a record exists without a verifier");
    }
    assert!(view.association.is_empty(), "{label}");
}

/// A confirmed nonlinear case: used for its published basis, Stage B bound,
/// the confirmation's own plan required. PR 374's correction failure on the
/// same model: the confirmation is kept as it reported itself (established),
/// discarded with `DisplacementCorrectionUnavailable`, and no basis or
/// requirement remains.
#[test]
fn a_confirmed_case_is_used_and_a_correction_failure_discards_it() {
    let request = demo();
    for mode in MODES {
        let normal = run(&request, mode, None);
        let evidence = normal.inverse_norm_evidence();
        assert_eq!(evidence.cases.len(), normal.numerical_quality.cases.len());
        assert!(!evidence.cases.is_empty());
        for case in &evidence.cases {
            let label = format!("{mode:?} {}", case.case_id);
            assert_eq!(case.withdrawn, None, "{label}");
            assert_eq!(case.verdict, crate::inverse_norm_evidence::VerdictMirror::ChecksPassed, "{label}");
            assert_eq!(case.published_basis, crate::inverse_norm_evidence::PublishedBasisView::ConfirmedStageB, "{label}");
            let view = case.confirmation.as_ref().unwrap_or_else(|| panic!("{label}: no confirmation"));
            assert_eq!(view.established, EstablishmentView::Established, "{label}");
            assert_eq!(view.publication, PublicationView::UsedForPublishedBasis, "{label}");
            assert_no_verifier(view, &label);
            let PlanView::Known(stages) = &view.plan else { unreachable!() };
            assert_eq!(case.required.len(), 1, "{label}");
            assert_eq!(case.required[0].stages, stages.iter().map(|s| s.stage.clone()).collect::<Vec<_>>(), "{label}");
            assert!(case.linear.is_none(), "{label}");
        }
        for kind in [DisplacementFailureKind::Certify, DisplacementFailureKind::Represent] {
            let failed = run(&request, mode, Some((crate::DOF_PER_NODE, kind)));
            for case in &failed.inverse_norm_evidence().cases {
                let label = format!("{mode:?} {kind:?} {}", case.case_id);
                assert_eq!(case.withdrawn, None, "{label}");
                assert_eq!(case.verdict, crate::inverse_norm_evidence::VerdictMirror::Sensitive, "{label}");
                assert_eq!(case.published_basis, crate::inverse_norm_evidence::PublishedBasisView::None, "{label}");
                assert!(case.required.is_empty(), "{label}");
                let view = case.confirmation.as_ref().unwrap();
                assert_eq!(view.established, EstablishmentView::Established, "{label}: the confirmation was relabelled");
                assert_eq!(view.publication, PublicationView::Discarded { reason: DiscardTag::DisplacementCorrectionUnavailable }, "{label}");
                assert_no_verifier(view, &label);
            }
        }
    }
}

/// A linear case: the published rows rest on the ordinary check's bound, so the
/// basis is linear and the formation check is required; no record exists, which
/// the reader derives as not recorded. A correction failure withdraws the bound
/// and so the basis.
#[test]
fn a_linear_case_requires_the_formation_check_and_records_nothing() {
    let mut request = demo();
    let model = request["model"].as_object_mut().unwrap();
    let linear: Vec<Value> = model["supports"].as_array().unwrap().iter().filter(|s| s["family"] != "nonlinear").cloned().collect();
    assert!(linear.len() < model["supports"].as_array().unwrap().len(), "the demo model has no nonlinear support to remove");
    model.insert("supports".into(), Value::Array(linear));
    for mode in MODES {
        let normal = run(&request, mode, None);
        assert_eq!(serde_json::to_value(&normal).unwrap()["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:#?}", normal.diagnostics);
        for case in &normal.inverse_norm_evidence().cases {
            let label = format!("{mode:?} {}", case.case_id);
            assert_eq!(case.withdrawn, None, "{label}");
            assert!(case.confirmation.is_none(), "{label}");
            assert!(case.linear.is_none(), "{label}: a record exists without a verifier");
            if case.verdict == crate::inverse_norm_evidence::VerdictMirror::ChecksPassed {
                assert_eq!(case.published_basis, crate::inverse_norm_evidence::PublishedBasisView::Linear, "{label}");
                assert_eq!(case.required.len(), 1, "{label}");
                assert_eq!(case.required[0].stages.iter().map(|s| s.stage).collect::<Vec<_>>(), vec![StageTag::FormationCheck], "{label}");
            }
        }
        let failed = run(&request, mode, Some((0, DisplacementFailureKind::Certify)));
        for case in &failed.inverse_norm_evidence().cases {
            let label = format!("{mode:?} forced {}", case.case_id);
            assert_eq!(case.withdrawn, None, "{label}");
            assert_eq!(case.published_basis, crate::inverse_norm_evidence::PublishedBasisView::None, "{label}");
            assert!(case.required.is_empty(), "{label}");
        }
    }
}

/// DEL-04-07 on the held route (PR 374's withdrawal): the held solve's own
/// evidence is projected under the comparison. In the control it is used for
/// the established held record; when the case's correction fails and
/// `withdraw_established` withdraws that record, the evidence is discarded with
/// the same reason, never shown as used, and the confirmation keeps its own
/// outcome.
#[test]
fn a_withdrawn_held_record_is_never_shown_as_used() {
    let request = crate::corrected_displacement_tests::held_operand_request();
    for mode in MODES {
        let held = |forced: bool| {
            let envelope = run(&request, mode, forced.then_some((0, DisplacementFailureKind::Certify)));
            let evidence = envelope.inverse_norm_evidence();
            assert_eq!(evidence.held.len(), 1, "{mode:?} forced={forced}");
            evidence.held[0].clone()
        };
        let control = held(false);
        assert_eq!((control.comparison_id.as_str(), control.minuend_id.as_str(), control.target_id.as_str()), ("comb:o-s-held", "case:O", "case:S"));
        assert_eq!(control.outcome, HeldOutcomeView::Admitted { kind: "established".into() }, "{mode:?}");
        let view = control.confirmation.as_ref().unwrap();
        assert_eq!(view.publication, PublicationView::UsedForPublishedBasis, "{mode:?}");
        assert_eq!(view.established, EstablishmentView::Established, "{mode:?}");
        assert_no_verifier(view, &format!("{mode:?} control"));
        let withdrawn = held(true);
        assert_eq!(withdrawn.outcome, HeldOutcomeView::Admitted { kind: "not_established".into() }, "{mode:?}");
        let view = withdrawn.confirmation.as_ref().unwrap();
        assert_eq!(view.publication, PublicationView::Discarded { reason: DiscardTag::DisplacementCorrectionUnavailable }, "{mode:?}");
        assert_eq!(view.established, EstablishmentView::Established, "{mode:?}: the confirmation was relabelled");
    }
}

/// `blocked_envelope`'s inspection-only conversion goes through the owner: the
/// bytes are exactly those of the conversion alone, and the pair agrees after
/// it. A diagnostic changed outside the owner before the conversion is still
/// withdrawn, and its bytes are still the conversion's.
#[test]
fn the_blocked_conversion_keeps_its_bytes_and_launders_no_bypass() {
    let model: crate::PreviewModel = serde_json::from_str(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap();
    let integrity = |case: &str, code: &str| crate::diag(&crate::integrity_diagnostic_id(case), code, "info", format!("original evaluation of {case}"), vec![case.to_string()]);
    let blocking = crate::diag("diagnostic:test:blocking", "SOLVER_SYSTEM_BLOCKED", "blocking", "refused".to_string(), vec![]);
    let built = vec![integrity("load:L-100", "NUMERICAL_INTEGRITY_CHECKS_PASSED"), integrity("load:L-200", "NUMERICAL_INTEGRITY_SENSITIVE"), blocking];
    let mut expected = built.clone();
    for d in expected.iter_mut().filter(|d| d.id.starts_with(crate::INTEGRITY_DIAGNOSTIC_PREFIX)) {
        crate::inspection_only(d);
    }
    let (diagnostics, sink) = through_owner(built.clone());
    let envelope = crate::blocked_envelope(model.clone(), diagnostics, sink);
    assert_eq!(serde_json::to_string(&envelope.diagnostics).unwrap(), serde_json::to_string(&expected).unwrap());
    let evidence = envelope.inverse_norm_evidence();
    for (case, prior) in [("load:L-100", PriorVerdict::ChecksPassed), ("load:L-200", PriorVerdict::Sensitive)] {
        let entry = evidence.cases.iter().find(|c| c.case_id == case).unwrap();
        assert_eq!(entry.withdrawn, None, "{case}");
        assert_eq!(entry.verdict, VerdictMirror::InspectionOnly { prior }, "{case}");
    }
    // A bypass before the conversion: same bytes after it, and withdrawn.
    let (mut diagnostics, sink) = through_owner(built);
    diagnostics[0].message.push_str(" (edited outside the owner)");
    expected[0].message.push_str(" (edited outside the owner)");
    let _allow = AllowWithdrawal::new();
    let envelope = crate::blocked_envelope(model, diagnostics, sink);
    assert_eq!(serde_json::to_string(&envelope.diagnostics).unwrap(), serde_json::to_string(&expected).unwrap());
    let evidence = envelope.inverse_norm_evidence();
    assert_eq!(evidence.cases.iter().find(|c| c.case_id == "load:L-100").unwrap().withdrawn, Some(WithdrawalView::MirrorMismatch));
    assert_eq!(evidence.cases.iter().find(|c| c.case_id == "load:L-200").unwrap().withdrawn, None);
}

/// The envelope's clone carries the sink, and a reconstruction that drops it
/// (as the retained reconstruction would with an empty sink) withdraws every
/// case at its checkpoint, publishing no evidence for a diagnostic it does
/// not describe.
#[test]
fn a_clone_keeps_the_pair_and_a_dropped_sink_withdraws() {
    let envelope = run(&demo(), PreviewSolverMode::DenseScrutiny, None);
    let copy = envelope.clone();
    assert_eq!(copy.inverse_norm_evidence(), envelope.inverse_norm_evidence());
    let mut rebuilt = envelope.clone();
    rebuilt.inverse_norm = InverseNormSink::default();
    let _allow = AllowWithdrawal::new();
    reconcile(&rebuilt.diagnostics, &mut rebuilt.inverse_norm);
    for case in &rebuilt.inverse_norm_evidence().cases {
        assert_eq!(case.withdrawn, Some(WithdrawalView::MissingEntry), "{}", case.case_id);
        assert!(case.confirmation.is_none() && case.linear.is_none(), "{}", case.case_id);
    }
}

/// The recovery guard's demotion (R-b') keeps a confirmed basis: the owner
/// moves the mirror with the edit, the verdict follows the diagnostic, and the
/// basis, evidence and publication use are unchanged. A no-op demotion (an
/// already sensitive case) changes nothing.
#[test]
fn a_demotion_that_keeps_its_basis_keeps_its_evidence() {
    let envelope = run(&demo(), PreviewSolverMode::DenseScrutiny, None);
    let mut diagnostics = envelope.diagnostics.clone();
    let mut sink = envelope.inverse_norm.clone();
    let case = envelope.numerical_quality.cases[0].basis_ref.ref_id.clone();
    let before = sink.projection(&diagnostics, &[case.as_str()]).cases.remove(0);
    let finding = crate::formation_guard::FormationFinding { guard: crate::formation_guard::Guard::Recovery, sentence: "demoted by test".into(), fired: Vec::new() };
    assert!(amend(&mut diagnostics, &mut sink, &case, |d| crate::formation_guard::demote(d, &finding), EntryUpdate::None));
    assert!(!amend(&mut diagnostics, &mut sink, &case, |d| crate::formation_guard::demote(d, &finding), EntryUpdate::None));
    reconcile(&diagnostics, &mut sink);
    let after = sink.projection(&diagnostics, &[case.as_str()]).cases.remove(0);
    assert_eq!(after.withdrawn, None);
    assert_eq!(after.verdict, VerdictMirror::Sensitive);
    assert_eq!((after.published_basis, &after.required, &after.confirmation), (before.published_basis, &before.required, &before.confirmation));
    assert_eq!(after.confirmation.unwrap().publication, PublicationView::UsedForPublishedBasis);
}

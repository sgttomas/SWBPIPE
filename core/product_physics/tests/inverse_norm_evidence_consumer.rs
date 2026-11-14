//! An outside-crate consumer of the inverse-norm evidence projection (DEL-04-01;
//! carrier revision 4 §5 and §7, "export boundary"). It reaches the projection
//! only through the public `MechanicsEnvelope::inverse_norm_evidence()` and the
//! named public types of `open_pipe_stress_product_physics::inverse_norm_evidence`,
//! and it mirrors the report reader's steps 0 to 2 (the report lane's page
//! `reader-computation-inverse-norm.md`, provisional for its PR-3):
//! - step 0, admissibility (otherwise inconsistent, never read as absent);
//! - step 1, each required stage's state;
//! - step 2, each output's state, in the order withdrawn, no requirement,
//!   discharged, not verified, not recorded.
//! Agent 0's two cautions are tests here: an empty requirement or a
//! `NotDetermined` plan discharges nothing, and withdrawn is never folded into
//! not recorded. In this build no verifier exists, so no real document carries
//! a record; constructed projections exercise every state.

use open_pipe_stress_product_physics::inverse_norm_evidence::*;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode};

/// What the reader reads from the published document besides the projection:
/// each published case and its integrity diagnostic's code, if any.
struct Document {
    cases: Vec<(String, Option<String>)>,
}

impl Document {
    fn of(envelope: &MechanicsEnvelope) -> Self {
        let cases = envelope
            .numerical_quality
            .cases
            .iter()
            .map(|c| {
                let id = format!("diagnostic:numerical-integrity:{}", c.basis_ref.ref_id);
                let code = envelope.diagnostics.iter().find(|d| d.id == id).map(|d| d.code.clone());
                (c.basis_ref.ref_id.clone(), code)
            })
            .collect();
        Self { cases }
    }
}

#[derive(Debug, PartialEq)]
enum StageState {
    Verified,
    NotVerified,
    NotRecorded,
}

#[derive(Debug, PartialEq)]
enum OutputState {
    Withdrawn,
    InspectionOnly,
    NoRequirement,
    Discharged,
    NotVerified,
    NotRecorded,
}

fn verdict_matches(verdict: &VerdictMirror, code: Option<&str>) -> bool {
    match (verdict, code) {
        (VerdictMirror::NotAssessed, None) => true,
        (VerdictMirror::ChecksPassed, Some("NUMERICAL_INTEGRITY_CHECKS_PASSED")) => true,
        (VerdictMirror::Sensitive, Some("NUMERICAL_INTEGRITY_SENSITIVE")) => true,
        (VerdictMirror::RecoveryBasisUnqualified, Some("NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED")) => true,
        (VerdictMirror::Failed { code: closed }, Some(code)) => IntegrityFailureCode::of(code) == Some(*closed),
        (VerdictMirror::InspectionOnly { .. }, Some("NUMERICAL_INTEGRITY_INSPECTION_ONLY")) => true,
        _ => false,
    }
}

fn plan_refs(confirmation: Option<&ConfirmationView>) -> Option<Vec<StageRef>> {
    match confirmation.map(|c| &c.plan) {
        Some(PlanView::Known(stages)) => Some(stages.iter().map(|s| s.stage.clone()).collect()),
        _ => None,
    }
}

/// Step 0: the reasons the member is inconsistent; empty when admissible.
fn inconsistencies(document: &Document, evidence: &InverseNormEvidence) -> Vec<String> {
    let mut out = Vec::new();
    if evidence.format != INVERSE_NORM_EVIDENCE_FORMAT {
        out.push("format".to_string());
    }
    for (case_id, code) in &document.cases {
        let entries: Vec<&CaseEvidence> = evidence.cases.iter().filter(|c| &c.case_id == case_id).collect();
        if entries.len() != 1 {
            out.push(format!("{case_id}: {} entries", entries.len()));
            continue;
        }
        let case = entries[0];
        if !verdict_matches(&case.verdict, code.as_deref()) {
            out.push(format!("{case_id}: verdict"));
        }
        let required: Vec<StageRef> = case.required.iter().flat_map(|r| r.stages.clone()).collect();
        let expected = match case.published_basis {
            PublishedBasisView::None => Some(Vec::new()),
            PublishedBasisView::Linear => Some(vec![StageRef { stage: StageTag::FormationCheck, subject: None }]),
            PublishedBasisView::ConfirmedStageB => plan_refs(case.confirmation.as_ref()),
        };
        if expected.as_ref() != Some(&required) {
            out.push(format!("{case_id}: requirement and published basis"));
        }
        let used = case.confirmation.as_ref().is_some_and(|c| c.publication == PublicationView::UsedForPublishedBasis);
        if used != (case.published_basis == PublishedBasisView::ConfirmedStageB) {
            out.push(format!("{case_id}: publication use and published basis"));
        }
        let mut records: Vec<&RecordView> = case.linear.iter().collect();
        if let Some(PlanView::Known(stages)) = case.confirmation.as_ref().map(|c| &c.plan) {
            records.extend(stages.iter().filter_map(|s| s.record.as_ref()));
        }
        for record in records {
            let shaped = match record.status {
                RecordStatus::Verified => record.verified.is_some() && record.not_verified.is_none(),
                RecordStatus::NotVerified => record.verified.is_none() && record.not_verified.is_some(),
            };
            if !shaped {
                out.push(format!("{case_id}: record shape"));
            }
        }
    }
    for case in &evidence.cases {
        if !document.cases.iter().any(|(id, _)| id == &case.case_id) {
            out.push(format!("{}: not published", case.case_id));
        }
    }
    out
}

/// Step 1: one required stage's state.
fn stage_state(case: &CaseEvidence, stage: &StageRef) -> StageState {
    let record = if stage.stage == StageTag::FormationCheck {
        case.linear.as_ref().filter(|r| r.stage == *stage)
    } else {
        match case.confirmation.as_ref().map(|c| &c.plan) {
            Some(PlanView::Known(stages)) => stages.iter().find(|s| s.stage == *stage).and_then(|s| s.record.as_ref()),
            _ => None,
        }
    };
    match record.map(|r| r.status) {
        Some(RecordStatus::Verified) => StageState::Verified,
        Some(RecordStatus::NotVerified) => StageState::NotVerified,
        None => StageState::NotRecorded,
    }
}

/// Step 2: the case's one output (VerdictAndFigures).
fn output_state(case: &CaseEvidence) -> OutputState {
    let required: Vec<StageRef> = case.required.iter().flat_map(|r| r.stages.clone()).collect();
    let rests_on_confirmation = required.iter().any(|s| s.stage != StageTag::FormationCheck);
    let association = case.confirmation.as_ref().is_some_and(|c| !c.association.is_empty());
    if case.withdrawn.is_some() || (association && rests_on_confirmation) {
        return OutputState::Withdrawn;
    }
    if matches!(case.verdict, VerdictMirror::InspectionOnly { .. }) {
        return OutputState::InspectionOnly;
    }
    if required.is_empty() {
        return OutputState::NoRequirement;
    }
    let states: Vec<StageState> = required.iter().map(|s| stage_state(case, s)).collect();
    if states.iter().all(|s| *s == StageState::Verified) {
        OutputState::Discharged
    } else if states.contains(&StageState::NotVerified) {
        OutputState::NotVerified
    } else {
        OutputState::NotRecorded
    }
}

// ---- constructed projections

fn stage(tag: StageTag, subject: Option<&str>) -> StageRef {
    StageRef { stage: tag, subject: subject.map(str::to_string) }
}

fn record(at: StageRef, status: RecordStatus) -> RecordView {
    RecordView {
        stage: at,
        status,
        estimate_limit_bits: 1024.0f64.to_bits(),
        est_bits: 1.0f64.to_bits(),
        matrix_digest: 7,
        inputs_digest: (status == RecordStatus::Verified).then_some(9),
        verified: (status == RecordStatus::Verified).then(|| VerifiedView {
            bound_bits: 256.0f64.to_bits(),
            block_count: 1,
            largest_block: 6,
            blocks_by_method: vec![MethodCount { method: MethodTag::LemmaEShiftedFactorization, shift: Some(ShiftTag::Sigma1), blocks: 1 }],
        }),
        not_verified: (status == RecordStatus::NotVerified).then(|| NotVerifiedView {
            block_count: 1,
            largest_block: 6,
            blocks_verified: 0,
            attempts: vec![AttemptView {
                method: MethodTag::LemmaDComparisonMatrix,
                reasons: vec![ReasonCount { reason: ReasonTag::LemmaDContractionNotBelowOne, blocks: 1 }],
            }],
        }),
    }
}

fn linear_case(id: &str, linear: Option<RecordStatus>) -> CaseEvidence {
    CaseEvidence {
        case_id: id.into(),
        verdict: VerdictMirror::ChecksPassed,
        published_basis: PublishedBasisView::Linear,
        required: vec![OutputRequirement { output: OutputKind::VerdictAndFigures, stages: vec![stage(StageTag::FormationCheck, None)] }],
        linear: linear.map(|s| record(stage(StageTag::FormationCheck, None), s)),
        confirmation: None,
        withdrawn: None,
    }
}

/// A confirmed coupled case: [A, influence(S1), B], with the given record per stage.
fn coupled_case(id: &str, records: [Option<RecordStatus>; 3]) -> CaseEvidence {
    let refs = [stage(StageTag::ConfirmationStageA, None), stage(StageTag::ConfirmationInfluence, Some("S1")), stage(StageTag::ConfirmationStageB, None)];
    let stages = refs
        .iter()
        .zip(records)
        .map(|(r, s)| StageView { stage: r.clone(), progress: ProgressView::VerifierReached, record: s.map(|s| record(r.clone(), s)) })
        .collect();
    CaseEvidence {
        case_id: id.into(),
        verdict: VerdictMirror::ChecksPassed,
        published_basis: PublishedBasisView::ConfirmedStageB,
        required: vec![OutputRequirement { output: OutputKind::VerdictAndFigures, stages: refs.to_vec() }],
        linear: None,
        confirmation: Some(ConfirmationView {
            attempt: AttemptState::Attempted,
            plan: PlanView::Known(stages),
            established: EstablishmentView::Established,
            publication: PublicationView::UsedForPublishedBasis,
            association: Vec::new(),
        }),
        withdrawn: None,
    }
}

fn evidence(cases: Vec<CaseEvidence>) -> InverseNormEvidence {
    InverseNormEvidence { format: INVERSE_NORM_EVIDENCE_FORMAT, cases, held: Vec::new() }
}

fn document(cases: &[(&str, Option<&str>)]) -> Document {
    Document { cases: cases.iter().map(|(id, code)| (id.to_string(), code.map(str::to_string))).collect() }
}

const PASSED: Option<&str> = Some("NUMERICAL_INTEGRITY_CHECKS_PASSED");

#[test]
fn a_real_documents_projection_is_admissible_with_one_entry_per_case_and_nothing_discharged() {
    let request: serde_json::Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"
    ))
    .unwrap();
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
        let projection = envelope.inverse_norm_evidence();
        let doc = Document::of(&envelope);
        assert!(!doc.cases.is_empty(), "{mode:?}");
        assert_eq!(inconsistencies(&doc, &projection), Vec::<String>::new(), "{mode:?}");
        assert_eq!(projection.cases.len(), doc.cases.len(), "{mode:?}");
        // No verifier exists in this build: nothing is discharged, and no case is withdrawn.
        for case in &projection.cases {
            assert_ne!(output_state(case), OutputState::Discharged, "{mode:?} {}", case.case_id);
            assert_ne!(output_state(case), OutputState::Withdrawn, "{mode:?} {}", case.case_id);
        }
        // The projection is identity-free and deterministic across solves.
        let again = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap().inverse_norm_evidence();
        assert_eq!(serde_json::to_string(&projection).unwrap(), serde_json::to_string(&again).unwrap(), "{mode:?}");
    }
}

#[test]
fn every_output_state_is_read_in_the_settled_order() {
    use RecordStatus::{NotVerified, Verified};
    let cases = vec![
        linear_case("load:discharged", Some(Verified)),
        linear_case("load:not-recorded", None),
        linear_case("load:not-verified", Some(NotVerified)),
        coupled_case("load:coupled-discharged", [Some(Verified), Some(Verified), Some(Verified)]),
        // Stage B alone verified never discharges a coupled case's figures.
        coupled_case("load:coupled-b-only", [None, None, Some(Verified)]),
        coupled_case("load:coupled-not-verified", [Some(Verified), Some(NotVerified), None]),
    ];
    let doc = document(&cases.iter().map(|c| (c.case_id.as_str(), PASSED)).collect::<Vec<_>>());
    let projection = evidence(cases);
    assert_eq!(inconsistencies(&doc, &projection), Vec::<String>::new());
    let state = |id: &str| output_state(projection.cases.iter().find(|c| c.case_id == id).unwrap());
    assert_eq!(state("load:discharged"), OutputState::Discharged);
    assert_eq!(state("load:not-recorded"), OutputState::NotRecorded);
    assert_eq!(state("load:not-verified"), OutputState::NotVerified);
    assert_eq!(state("load:coupled-discharged"), OutputState::Discharged);
    assert_eq!(state("load:coupled-b-only"), OutputState::NotRecorded);
    assert_eq!(state("load:coupled-not-verified"), OutputState::NotVerified);
}

#[test]
fn an_empty_requirement_or_an_undetermined_plan_discharges_nothing() {
    let mut unconfirmed = linear_case("load:nonlinear", None);
    unconfirmed.verdict = VerdictMirror::Sensitive;
    unconfirmed.published_basis = PublishedBasisView::None;
    unconfirmed.required = Vec::new();
    unconfirmed.confirmation = Some(ConfirmationView {
        attempt: AttemptState::Attempted,
        plan: PlanView::NotDetermined,
        established: EstablishmentView::NotEstablished { reason: "confirmation attempt: the converged state is not qualified".into() },
        publication: PublicationView::NotConsulted,
        association: Vec::new(),
    });
    let mut not_run = unconfirmed.clone();
    not_run.case_id = "load:not-run".into();
    not_run.confirmation.as_mut().unwrap().attempt = AttemptState::NotRun;
    let doc = document(&[("load:nonlinear", Some("NUMERICAL_INTEGRITY_SENSITIVE")), ("load:not-run", Some("NUMERICAL_INTEGRITY_SENSITIVE"))]);
    let projection = evidence(vec![unconfirmed, not_run]);
    assert_eq!(inconsistencies(&doc, &projection), Vec::<String>::new());
    for case in &projection.cases {
        assert_eq!(output_state(case), OutputState::NoRequirement, "{}", case.case_id);
        assert_ne!(output_state(case), OutputState::Discharged);
    }
    // A confirmed basis whose plan is NotDetermined has no requirement to match: inconsistent.
    let mut bad = coupled_case("load:bad", [None, None, None]);
    bad.confirmation.as_mut().unwrap().plan = PlanView::NotDetermined;
    let doc = document(&[("load:bad", PASSED)]);
    assert!(inconsistencies(&doc, &evidence(vec![bad])).iter().any(|r| r.contains("requirement")));
}

#[test]
fn withdrawn_is_its_own_state_and_is_never_folded_into_not_recorded() {
    let mut withdrawn = linear_case("load:withdrawn", None);
    withdrawn.withdrawn = Some(WithdrawalView::MirrorMismatch);
    let mut associated = coupled_case("load:associated", [Some(RecordStatus::Verified), Some(RecordStatus::Verified), Some(RecordStatus::Verified)]);
    associated.confirmation.as_mut().unwrap().association =
        vec![AssociationView { slot: Some(stage(StageTag::ConfirmationStageB, None)), kind: AssociationTag::ForeignRecord }];
    let doc = document(&[("load:withdrawn", PASSED), ("load:associated", PASSED)]);
    let projection = evidence(vec![withdrawn, associated]);
    assert_eq!(inconsistencies(&doc, &projection), Vec::<String>::new());
    for case in &projection.cases {
        assert_eq!(output_state(case), OutputState::Withdrawn, "{}", case.case_id);
        assert_ne!(output_state(case), OutputState::NotRecorded);
    }
}

#[test]
fn a_missing_duplicate_or_mismatched_entry_is_inconsistent_not_absent() {
    let one = linear_case("load:a", None);
    // Missing case.
    let doc = document(&[("load:a", PASSED), ("load:b", PASSED)]);
    assert!(inconsistencies(&doc, &evidence(vec![one.clone()])).iter().any(|r| r.contains("load:b: 0 entries")));
    // Duplicate case.
    let doc = document(&[("load:a", PASSED)]);
    assert!(inconsistencies(&doc, &evidence(vec![one.clone(), one.clone()])).iter().any(|r| r.contains("2 entries")));
    // A case the document does not publish.
    let mut extra = one.clone();
    extra.case_id = "load:z".into();
    assert!(inconsistencies(&doc, &evidence(vec![one.clone(), extra])).iter().any(|r| r.contains("not published")));
    // Verdict against the closed codes: a failure code must be the diagnostic's own.
    let mut failed = one.clone();
    failed.verdict = VerdictMirror::Failed { code: IntegrityFailureCode::NegativeEnergy };
    let doc_failed = document(&[("load:a", Some("NUMERICAL_INTEGRITY_UNRESOLVED"))]);
    assert!(inconsistencies(&doc_failed, &evidence(vec![failed])).iter().any(|r| r.contains("verdict")));
    // NotAssessed pairs only with no integrity diagnostic.
    let mut unassessed = one.clone();
    unassessed.verdict = VerdictMirror::NotAssessed;
    unassessed.published_basis = PublishedBasisView::None;
    unassessed.required = Vec::new();
    assert_eq!(inconsistencies(&document(&[("load:a", None)]), &evidence(vec![unassessed.clone()])), Vec::<String>::new());
    assert!(inconsistencies(&doc, &evidence(vec![unassessed])).iter().any(|r| r.contains("verdict")));
    // A record whose status and payload disagree.
    let mut misshapen = linear_case("load:a", Some(RecordStatus::Verified));
    misshapen.linear.as_mut().unwrap().verified = None;
    assert!(inconsistencies(&doc, &evidence(vec![misshapen])).iter().any(|r| r.contains("record shape")));
}

#[test]
fn the_progress_value_of_this_build_never_reads_as_an_attempt() {
    // NoVerifierInThisBuild is progress, never a record: a stage that shows it is not recorded.
    let mut case = coupled_case("load:c", [None, None, None]);
    if let Some(ConfirmationView { plan: PlanView::Known(stages), .. }) = case.confirmation.as_mut() {
        for s in stages {
            s.progress = ProgressView::NoVerifierInThisBuild;
        }
    }
    assert_eq!(stage_state(&case, &stage(StageTag::ConfirmationStageB, None)), StageState::NotRecorded);
    assert_eq!(output_state(&case), OutputState::NotRecorded);
    assert_eq!(serde_json::to_value(ProgressView::NoVerifierInThisBuild).unwrap(), "no_verifier_in_this_build");
}

#[test]
fn an_inspection_only_case_discharges_nothing_published_and_pairs_only_with_its_code() {
    let mut retained = coupled_case("load:retained", [Some(RecordStatus::Verified), Some(RecordStatus::Verified), Some(RecordStatus::Verified)]);
    retained.verdict = VerdictMirror::InspectionOnly { prior: PriorVerdict::ChecksPassed };
    let doc = document(&[("load:retained", Some("NUMERICAL_INTEGRITY_INSPECTION_ONLY"))]);
    let projection = evidence(vec![retained.clone()]);
    assert_eq!(inconsistencies(&doc, &projection), Vec::<String>::new());
    assert_eq!(output_state(&projection.cases[0]), OutputState::InspectionOnly, "even with every stage verified");
    // Its stage states are still read, as retained evidence.
    assert_eq!(stage_state(&projection.cases[0], &stage(StageTag::ConfirmationStageB, None)), StageState::Verified);
    // Withdrawn comes first.
    let mut both = retained.clone();
    both.withdrawn = Some(WithdrawalView::MirrorMismatch);
    assert_eq!(output_state(&both), OutputState::Withdrawn);
    // Inspection-only pairs only with the inspection-only code.
    let doc_passed = document(&[("load:retained", PASSED)]);
    assert!(inconsistencies(&doc_passed, &projection).iter().any(|r| r.contains("verdict")));
    assert_eq!(serde_json::to_value(&retained.verdict).unwrap(), serde_json::json!({"inspection_only": {"prior": "checks_passed"}}));
}

#[test]
fn a_discard_carries_a_closed_reason() {
    for (tag, spelled) in [
        (DiscardTag::LoadRowFinding, "load_row_finding"),
        (DiscardTag::PreGuardNotPassed, "pre_guard_not_passed"),
        (DiscardTag::DisplacementCorrectionUnavailable, "displacement_correction_unavailable"),
    ] {
        let view = PublicationView::Discarded { reason: tag };
        assert_eq!(serde_json::to_value(&view).unwrap(), serde_json::json!({"discarded": {"reason": spelled}}));
    }
}

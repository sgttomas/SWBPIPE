//! I2b-2 unit tests (DEL-04-09): nonlinear supports at the named state case.
//! The decision-B seam on the static case solve (`LoadCaseSolve::
//! nonlinear_state`), its admission (D2) and mode (D3), the linearization of
//! §1.3's table against hand-built linear twins, every refusal, and the
//! structural checks that no blocked construction carries evidence and that
//! the state module reads no diagnostic message. The K-D5 nonlinear reference
//! requests (`validation/references/kd5_nonlinear/`) supply the physics; this
//! test adds only a pipe density, which no static solve reads. All models are
//! invented.
use super::outcome::{ModalOutcome, MODAL_DENSE_PATH_MODE};
use super::prepare::{prepare_modal, Prepared};
use super::state::{admit_modal_state_evidence, linearize_modal_supports, solve_modal_state_case, ModalSupportTreatment, StateCaseRefusal};
use super::tests::{arc_request, codes, modal, pf_request, q, section, with, PROV};
use super::tests_eigen::test_local_parameters;
use super::MODAL_STATE_CASE_UNAVAILABLE;
use crate::{
    active_set_state_code, solve_load_case_observed, form_basis_stiffness, ActiveSetState, CaseMotionEvidence, Diagnostic, LoadCaseSolve, NonlinearQualification,
    NonlinearStateEvidence, NonlinearStateStanding, NonlinearSupport, PreviewSolverMode, SourceRecoveryBudget, SupportStateRecord,
};
use open_pipe_stress_linear_supports::FrameDof;
use open_pipe_stress_nonlinear_supports::{ActivationSense, GapDirection};
use serde_json::{json, Value};

const U1: &str = include_str!("../../../../validation/references/kd5_nonlinear/u1_request.json");
const F1: &str = include_str!("../../../../validation/references/kd5_nonlinear/f1_request.json");
const F1_EXPLICIT: &str = include_str!("../../../../validation/references/kd5_nonlinear/f1_explicit_request.json");
const LOAD_STATE_REQUEST: &str = include_str!("../../../../fixtures/product_preview/pressure/load_reference/connected.request.json");

// ---------------------------------------------------------------------------
// Builders
// ---------------------------------------------------------------------------

/// A K-D5 reference request on the current document version, with a pipe
/// density so that it has a mass (no static solve reads it).
pub(super) fn kd5(text: &str) -> Value {
    let mut request: Value = serde_json::from_str(text).unwrap();
    request["model"]["schema_version"] = json!(crate::MODEL_SCHEMA_VERSION);
    for pipe in request["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["material_density"] = q(7850.0, "kg/m^3");
    }
    request
}

fn support_mut<'a>(request: &'a mut Value, id: &str) -> &'a mut Value {
    request["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == id).unwrap()
}

/// `request` with nonlinear support `id` replaced by its hand-built linear
/// twin: a rigid restraint of the same node and DOF when `rigid`, nothing
/// otherwise.
fn linear_twin(request: &Value, id: &str, rigid: bool) -> Value {
    let mut twin = request.clone();
    let supports = twin["model"]["supports"].as_array_mut().unwrap();
    let index = supports.iter().position(|s| s["id"] == id).unwrap();
    let removed = supports.remove(index);
    if rigid {
        supports.push(json!({"id": "support:twin", "node": removed["node"], "family": "guide",
            "restraints": [removed["nonlinear"]["dof"]], "provenance": "invented DEL-04-09 I2b-2 linear twin"}));
    }
    twin
}

fn state_request(request: &Value, case: &str) -> Value {
    with(request.clone(), json!({"mode_count": 3, "state_case": case}))
}

fn state_modal_run(request: Value) -> Result<ModalOutcome, Vec<Diagnostic>> {
    super::outcome::run_modal_dense(&modal(request), &test_local_parameters())
}

fn state_modal_solved(request: Value) -> ModalOutcome {
    state_modal_run(request).unwrap_or_else(|found| panic!("refused: {found:#?}"))
}

fn state_mode_bits(outcome: &ModalOutcome) -> Vec<u64> {
    outcome.modes.iter().map(|m| m.eigenvalue.to_bits()).collect()
}

/// The state case at `case` against its linear twin: the same eigenvalues
/// bit for bit (the same K_ff and M_ff on the same partition), and the
/// outcome names the case, the mode, each support's state and treatment.
fn matches_twin(request: &Value, case: &str, id: &str, state: ActiveSetState, treatment: ModalSupportTreatment) -> ModalOutcome {
    let outcome = state_modal_solved(state_request(request, case));
    let rigid = matches!(treatment, ModalSupportTreatment::Restrained | ModalSupportTreatment::AlreadyRestrained);
    let twin = state_modal_solved(with(linear_twin(request, id, rigid), json!({"mode_count": 3})));
    assert!(!outcome.modes.is_empty());
    assert_eq!(state_mode_bits(&outcome), state_mode_bits(&twin), "{case}: the linearized model equals its hand-built linear twin");
    let linearized = outcome.nonlinear_state.as_ref().expect("a nonlinear model records its state case");
    assert_eq!(linearized.case_id, case);
    assert_eq!(linearized.solver_mode, PreviewSolverMode::DenseScrutiny);
    let support = linearized.supports.iter().find(|s| s.support_id == id).unwrap();
    assert_eq!((support.state, support.treatment), (state, treatment), "{case}");
    assert!(twin.nonlinear_state.is_none());
    assert!(outcome.disclosures.iter().any(|d| d.contains(&format!("state case `{case}`")) && d.contains("dense_scrutiny")));
    outcome
}

/// The refusal of `request` at `case`: the last diagnostic is the named code
/// and states `refusal`'s reason.
fn refused_with(request: Value, case: &str, refusal: &StateCaseRefusal) -> Vec<Diagnostic> {
    let found = state_modal_run(request).err().unwrap_or_else(|| panic!("{case}: expected a refusal"));
    let last = found.last().unwrap();
    assert_eq!(last.code, MODAL_STATE_CASE_UNAVAILABLE, "{case}: {found:#?}");
    assert!(last.message.contains(&refusal.modal_refusal_reason()), "{case}: {}", last.message);
    assert_eq!(last.affected_refs, vec!["state_case".to_string(), case.to_string()]);
    found
}

/// The static case solve exactly as the modal state solve runs it, returned
/// for inspection.
fn static_state_solve(prepared: &Prepared, mode: PreviewSolverMode) -> (LoadCaseSolve, Vec<Diagnostic>) {
    let bound = prepared.bound_case.as_ref().unwrap();
    // The static entry's `linear` flag for this invocation.
    let stiffness = form_basis_stiffness(&prepared.built, &prepared.boundary.springs, prepared.built.nonlinear_supports.is_empty()).unwrap();
    let mut diagnostics = Vec::new();
    let solve = solve_load_case_observed(
        &prepared.model,
        &prepared.built,
        &prepared.materials,
        &stiffness,
        &prepared.boundary.restrained_dofs,
        &prepared.boundary.springs,
        &prepared.model.load_cases[bound.case_index],
        prepared.state_basis_record.as_deref(),
        mode,
        None,
        &mut SourceRecoveryBudget::default(),
        prepared.state_resolution.as_ref(),
        &mut diagnostics,
        None,
        crate::held_status::CaseRole::Own(None),
    )
    .unwrap();
    (solve, diagnostics)
}

// ---------------------------------------------------------------------------
// Admitted: each mapping against its linear twin
// ---------------------------------------------------------------------------

#[test]
fn one_way_active_and_inactive_match_their_linear_twins() {
    let u1 = kd5(U1);
    let pressed = matches_twin(&u1, "load:PRESSED", "support:NL-U", ActiveSetState::Active, ModalSupportTreatment::Restrained);
    let lifted = matches_twin(&u1, "load:LIFTED", "support:NL-U", ActiveSetState::Inactive, ModalSupportTreatment::Free);
    // The active state restrains node C's UZ (node 2): one fewer free DOF.
    assert_eq!(pressed.guard.free_dimension + 1, lifted.guard.free_dimension);
    let state = pressed.nonlinear_state.unwrap();
    assert_eq!(state.supports[0].global_dof, 2 * 6 + 2);
    assert!(state.restrained_dofs.contains(&14) && state.restrained_dofs.windows(2).all(|w| w[0] < w[1]));
    assert!(!lifted.nonlinear_state.unwrap().restrained_dofs.contains(&14));
}

#[test]
fn lift_off_matches_its_linear_twin() {
    let mut u1 = kd5(U1);
    support_mut(&mut u1, "support:NL-U")["nonlinear"] =
        json!({"behavior": "lift_off", "dof": "UZ", "initial_state": "active", "contact_when": "positive_reaction"});
    matches_twin(&u1, "load:PRESSED", "support:NL-U", ActiveSetState::Active, ModalSupportTreatment::Restrained);
    matches_twin(&u1, "load:LIFTED", "support:NL-U", ActiveSetState::Inactive, ModalSupportTreatment::Free);
}

#[test]
fn friction_sticking_and_sliding_follow_their_own_states_with_their_normal() {
    // F-1: the normal is the linear guide S-N (UY at B), which stays restrained
    // in both states; the friction tangent (UX at B) follows its own state.
    let f1 = kd5(F1);
    let stick = matches_twin(&f1, "load:STICK", "support:NL-F", ActiveSetState::Sticking, ModalSupportTreatment::Restrained);
    let slide = matches_twin(&f1, "load:SLIDE-POS", "support:NL-F", ActiveSetState::Sliding, ModalSupportTreatment::FreeFrozenFriction);
    let guide_uy = 6 + 1;
    for outcome in [&stick, &slide] {
        assert!(outcome.nonlinear_state.as_ref().unwrap().restrained_dofs.contains(&guide_uy));
        assert!(outcome.disclosures.iter().any(|d| d.contains("its normal restraint is its own support")));
    }
    assert!(slide.disclosures.iter().any(|d| d.contains("`support:NL-F` slides") && d.contains("frozen")));
    assert!(!stick.disclosures.iter().any(|d| d.contains("frozen")));
    // F-1 explicit: the normal is an explicit value.
    let explicit = kd5(F1_EXPLICIT);
    matches_twin(&explicit, "load:STICK", "support:NL-F", ActiveSetState::Sticking, ModalSupportTreatment::Restrained);
    matches_twin(&explicit, "load:SLIDE", "support:NL-F", ActiveSetState::Sliding, ModalSupportTreatment::FreeFrozenFriction);
}

#[test]
fn a_dof_already_restrained_stays_restrained_and_is_recorded() {
    // The static loop refuses an active support on a base-restrained DOF
    // ("active boundary DOF repeated"), so the solver cannot produce this
    // state: the mapping is reached with constructed final states.
    let support = NonlinearSupport::one_way("support:nl", 1, FrameDof::Uz, ActivationSense::PositiveReaction);
    let restrained = [0, 1, 2, 3, 4, 5, 8];
    let (states, set) = linearize_modal_supports(&[support.clone()], &restrained, &[SupportStateRecord::new("support:nl", ActiveSetState::Active)]).unwrap();
    assert_eq!(states[0].global_dof, 8);
    assert_eq!(states[0].treatment, ModalSupportTreatment::AlreadyRestrained);
    assert_eq!(set, restrained.to_vec());
    let (states, set) = linearize_modal_supports(&[support], &restrained, &[SupportStateRecord::new("support:nl", ActiveSetState::Inactive)]).unwrap();
    assert_eq!(states[0].treatment, ModalSupportTreatment::Free);
    assert_eq!(set, restrained.to_vec(), "a free state never removes a linear restraint");
}

#[test]
fn every_table_row_maps_as_stated() {
    let restrained = [0, 1, 2, 3, 4, 5];
    let gap = NonlinearSupport::gap("g", 1, FrameDof::Uy, 0.001, GapDirection::PositiveDisplacement).unwrap();
    let lift = NonlinearSupport::lift_off("l", 1, FrameDof::Ux, ActivationSense::NegativeReaction);
    let friction = NonlinearSupport::friction("f", 1, FrameDof::Uz, 0.3).unwrap();
    let supports = [gap, lift, friction];
    let record = |states: [ActiveSetState; 3]| -> Vec<SupportStateRecord> {
        ["g", "l", "f"].iter().zip(states).map(|(id, s)| SupportStateRecord::new(*id, s)).collect()
    };
    let (states, set) =
        linearize_modal_supports(&supports, &restrained, &record([ActiveSetState::Active, ActiveSetState::Active, ActiveSetState::Sticking])).unwrap();
    assert!(states.iter().all(|s| s.treatment == ModalSupportTreatment::Restrained));
    assert_eq!(set, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    let (states, set) =
        linearize_modal_supports(&supports, &restrained, &record([ActiveSetState::Inactive, ActiveSetState::Inactive, ActiveSetState::Sliding])).unwrap();
    assert_eq!(
        states.iter().map(|s| s.treatment).collect::<Vec<_>>(),
        vec![ModalSupportTreatment::Free, ModalSupportTreatment::Free, ModalSupportTreatment::FreeFrozenFriction]
    );
    assert_eq!(set, restrained.to_vec());
    assert_eq!(states.iter().map(|s| s.behavior).collect::<Vec<_>>(), vec!["gap", "lift_off", "friction"]);
    // Friction inactive (its normal does not bear): free, with no force frozen
    // (O-14 (b), released by Agent 0).
    let (states, set) =
        linearize_modal_supports(&supports, &restrained, &record([ActiveSetState::Active, ActiveSetState::Active, ActiveSetState::Inactive])).unwrap();
    assert_eq!(
        states.iter().map(|s| s.treatment).collect::<Vec<_>>(),
        vec![ModalSupportTreatment::Restrained, ModalSupportTreatment::Restrained, ModalSupportTreatment::Free]
    );
    assert_eq!(set, vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

/// `request` with friction support `support:NL-F`'s normal replaced by an
/// explicit value (a derived source, if any, is removed).
fn with_explicit_friction_normal(request: &Value, newtons: f64) -> Value {
    let mut out = request.clone();
    let nonlinear = &mut support_mut(&mut out, "support:NL-F")["nonlinear"];
    nonlinear.as_object_mut().unwrap().remove("normal_reaction_source");
    nonlinear["normal_reaction"] = q(newtons, "N");
    out
}

#[test]
fn friction_whose_normal_does_not_bear_is_free_confirmed_and_matches_its_linear_twin() {
    // O-14 (b): a genuinely solved friction-inactive state. The classifier
    // returns inactive when the normal is not positive, and the static
    // confirmation establishes an explicit-normal inactive state exactly when
    // its normal is not positive, so a zero or tensile explicit normal comes
    // out converged, qualified and confirmed, through the real modal path.
    let f1_explicit = kd5(F1_EXPLICIT);
    // F-1 with an explicit normal beside its linear guide S-N (UY at B): the
    // guide is its own support, restrained in every state.
    let f1_guided = kd5(F1);
    let guide_uy = 6 + 1;
    for (request, case, guided) in [
        (with_explicit_friction_normal(&f1_explicit, 0.0), "load:SLIDE", false),
        (with_explicit_friction_normal(&f1_explicit, 0.0), "load:STICK", false),
        (with_explicit_friction_normal(&f1_explicit, -70.0), "load:SLIDE", false),
        (with_explicit_friction_normal(&f1_guided, 0.0), "load:SLIDE-POS", true),
        (with_explicit_friction_normal(&f1_guided, 0.0), "load:STICK", true),
    ] {
        // The static standing, from the same solve the modal path runs.
        let modal_request = modal(state_request(&request, case));
        let (solve, diagnostics) = static_state_solve(&prepare_modal(&modal_request).unwrap(), PreviewSolverMode::DenseScrutiny);
        let evidence = solve.nonlinear_state.as_deref().expect("a converged, qualified case carries its evidence");
        assert_eq!((evidence.converged, evidence.qualification), (true, NonlinearQualification::Qualified), "{case}");
        assert!(matches!(evidence.standing, NonlinearStateStanding::Confirmed), "{case}: {:?}", evidence.standing);
        assert_eq!(evidence.final_states, vec![SupportStateRecord::new("support:NL-F", ActiveSetState::Inactive)], "{case}");
        assert!(diagnostics.iter().all(|d| d.severity != "blocking"), "{case}: {diagnostics:#?}");
        // The modal model equals its hand-built twin: the friction tangent free.
        let outcome = matches_twin(&request, case, "support:NL-F", ActiveSetState::Inactive, ModalSupportTreatment::Free);
        let state = outcome.nonlinear_state.as_ref().unwrap();
        assert!(!state.restrained_dofs.contains(&6), "{case}: the tangent (UX at B) is free");
        assert_eq!(state.restrained_dofs.contains(&guide_uy), guided, "{case}: the normal guide keeps its own state");
        assert!(outcome.disclosures.iter().any(|d| d.contains("`support:NL-F` is inactive") && d.contains("tangential DOF is free")), "{case}");
        assert!(!outcome.disclosures.iter().any(|d| d.contains("frozen")), "{case}: no friction force is frozen");
    }
}

#[test]
fn friction_whose_derived_normal_does_not_bear_stays_refused() {
    // A derived normal is |R| of its linear source (classifier: inactive at
    // zero), and the static confirmation never establishes a derived-normal
    // inactive friction ("friction normal not established"), so this state is
    // NotEstablished and refused through the unchanged admission.
    let mut f1 = kd5(F1);
    f1["model"]["load_cases"].as_array_mut().unwrap().push(json!({"id": "load:TANGENT-ONLY", "provenance": PROV,
        "primitive_loads": [{"id": "load:TANGENT-ONLY-0", "category": "occasional", "target": {"type": "node", "node": "node:B"},
            "direction": "global_x", "magnitude": q(30.0, "N"), "dimension": "force", "provenance": PROV}]}));
    let modal_request = modal(state_request(&f1, "load:TANGENT-ONLY"));
    let (solve, _) = static_state_solve(&prepare_modal(&modal_request).unwrap(), PreviewSolverMode::DenseScrutiny);
    let evidence = solve.nonlinear_state.as_deref().unwrap();
    assert_eq!(evidence.final_states, vec![SupportStateRecord::new("support:NL-F", ActiveSetState::Inactive)]);
    assert_eq!(evidence.qualification, NonlinearQualification::Qualified);
    assert!(matches!(&evidence.standing, NonlinearStateStanding::NotEstablished { reason } if reason.contains("friction normal not established")));
    let found = state_modal_run(state_request(&f1, "load:TANGENT-ONLY")).err().unwrap();
    let last = found.last().unwrap();
    assert_eq!(last.code, MODAL_STATE_CASE_UNAVAILABLE, "{found:#?}");
    assert!(last.message.contains("its converged support state is not confirmed") && last.message.contains("friction normal not established"), "{}", last.message);
}

// ---------------------------------------------------------------------------
// Refused: the seam's own values (D2) and the linearization's identities
// ---------------------------------------------------------------------------

fn constructed_state_evidence(case: &str) -> NonlinearStateEvidence {
    NonlinearStateEvidence {
        case_id: case.to_string(),
        solver_mode: PreviewSolverMode::DenseScrutiny,
        converged: true,
        qualification: NonlinearQualification::Qualified,
        standing: NonlinearStateStanding::Confirmed,
        final_states: vec![SupportStateRecord::new("support:nl", ActiveSetState::Active)],
        motion: CaseMotionEvidence::default(),
    }
}

fn state_blocking_diag(code: &str) -> Diagnostic {
    crate::diag("diagnostic:test", code, "blocking", "test", vec![])
}

#[test]
fn admission_refuses_every_combination_but_the_confirmed_dense_one() {
    let dense = PreviewSolverMode::DenseScrutiny;
    let good = constructed_state_evidence("case:s");
    assert!(admit_modal_state_evidence(Some(&good), "case:s", &CaseMotionEvidence::default(), dense, &[]).is_ok());
    let warning = crate::diag("diagnostic:test", "NONLINEAR_SUPPORT_LOOP_CONVERGED", "info", "test", vec![]);
    assert!(admit_modal_state_evidence(Some(&good), "case:s", &CaseMotionEvidence::default(), dense, &[warning]).is_ok());
    let refused = |e: Option<&NonlinearStateEvidence>, mode: PreviewSolverMode, found: &[Diagnostic]| {
        admit_modal_state_evidence(e, "case:s", &CaseMotionEvidence::default(), mode, found).err().unwrap()
    };
    // Missing evidence: named as not converged when the loop says so by code.
    let not_converged = crate::diag("diagnostic:test", "NONLINEAR_SUPPORT_LOOP_NOT_CONVERGED", "warning", "test", vec![]);
    assert_eq!(refused(None, dense, &[not_converged, state_blocking_diag("SOLVER_SYSTEM_BLOCKED")]), StateCaseRefusal::NotConverged);
    assert_eq!(refused(None, dense, &[state_blocking_diag("SOLVER_SYSTEM_BLOCKED")]), StateCaseRefusal::MissingEvidence(vec!["SOLVER_SYSTEM_BLOCKED".into()]));
    assert_eq!(refused(None, dense, &[]), StateCaseRefusal::MissingEvidence(vec![]));
    // A mismatched case id.
    assert_eq!(
        refused(Some(&constructed_state_evidence("case:other")), dense, &[]),
        StateCaseRefusal::CaseMismatch { expected: "case:s".into(), found: "case:other".into() }
    );
    // R-4 §3.7 S6: evidence whose consumed motion is not the bound case's
    // (a replayed solve of another motion under the same id) is refused,
    // and so is the bound motion against evidence of no motion.
    let moved = CaseMotionEvidence { ground_motion: vec![("support:nl".into(), 0.25f64.to_bits())], ..CaseMotionEvidence::default() };
    let replayed = NonlinearStateEvidence { motion: moved.clone(), ..constructed_state_evidence("case:s") };
    assert_eq!(refused(Some(&replayed), dense, &[]), StateCaseRefusal::MotionMismatch);
    assert_eq!(admit_modal_state_evidence(Some(&good), "case:s", &moved, dense, &[]).err().unwrap(), StateCaseRefusal::MotionMismatch);
    assert!(admit_modal_state_evidence(Some(&replayed), "case:s", &moved, dense, &[]).is_ok());
    // A sign of zero is part of the bound motion.
    let negative_zero = CaseMotionEvidence { ground_motion: vec![("support:nl".into(), (-0.0f64).to_bits())], ..CaseMotionEvidence::default() };
    let positive_zero = CaseMotionEvidence { ground_motion: vec![("support:nl".into(), 0.0f64.to_bits())], ..CaseMotionEvidence::default() };
    let zero = NonlinearStateEvidence { motion: positive_zero, ..constructed_state_evidence("case:s") };
    assert_eq!(admit_modal_state_evidence(Some(&zero), "case:s", &negative_zero, dense, &[]).err().unwrap(), StateCaseRefusal::MotionMismatch);
    // A mismatched mode, and a request mode other than the dense path's.
    let sparse = NonlinearStateEvidence { solver_mode: PreviewSolverMode::SparseInteractive, ..constructed_state_evidence("case:s") };
    assert_eq!(refused(Some(&sparse), dense, &[]), StateCaseRefusal::ModeMismatch { requested: dense, solved: PreviewSolverMode::SparseInteractive });
    assert_eq!(refused(Some(&sparse), PreviewSolverMode::SparseInteractive, &[]), StateCaseRefusal::ModeUnsupported(PreviewSolverMode::SparseInteractive));
    // Not converged.
    let unconverged = NonlinearStateEvidence { converged: false, ..constructed_state_evidence("case:s") };
    assert_eq!(refused(Some(&unconverged), dense, &[]), StateCaseRefusal::NotConverged);
    // The converged-but-unqualified inspectable gap state, even "confirmed".
    let inspectable = NonlinearStateEvidence { qualification: NonlinearQualification::InspectableUnqualifiedGap, ..constructed_state_evidence("case:s") };
    assert_eq!(refused(Some(&inspectable), dense, &[]), StateCaseRefusal::InspectableUnqualifiedGap);
    // NotEstablished, with its own sentence.
    let absent = NonlinearStateEvidence {
        standing: NonlinearStateStanding::NotEstablished { reason: "confirmation attempt: not run".into() },
        ..constructed_state_evidence("case:s")
    };
    assert_eq!(refused(Some(&absent), dense, &[]), StateCaseRefusal::NotEstablished("confirmation attempt: not run".into()));
    // A confirmed state beside a blocking static diagnostic.
    assert_eq!(
        refused(Some(&good), dense, &[state_blocking_diag("SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED")]),
        StateCaseRefusal::Blocking(vec!["SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED".into()])
    );
}

#[test]
fn linearization_refuses_missing_duplicate_unexpected_ids_and_invalid_pairs() {
    let one_way = NonlinearSupport::one_way("support:nl", 1, FrameDof::Uz, ActivationSense::PositiveReaction);
    let friction = NonlinearSupport::friction("support:f", 1, FrameDof::Ux, 0.3).unwrap();
    let gap = NonlinearSupport::gap("support:g", 1, FrameDof::Uy, 0.0, GapDirection::NegativeDisplacement).unwrap();
    let r = |id: &str, s: ActiveSetState| SupportStateRecord::new(id, s);
    let refused = |supports: &[NonlinearSupport], states: &[SupportStateRecord]| linearize_modal_supports(supports, &[0], states).err().unwrap();
    assert_eq!(refused(&[one_way.clone()], &[]), StateCaseRefusal::SupportMissing("support:nl".into()));
    assert_eq!(
        refused(&[one_way.clone()], &[r("support:nl", ActiveSetState::Active), r("support:nl", ActiveSetState::Active)]),
        StateCaseRefusal::SupportDuplicate("support:nl".into())
    );
    assert_eq!(
        refused(&[one_way.clone()], &[r("support:nl", ActiveSetState::Active), r("support:x", ActiveSetState::Active)]),
        StateCaseRefusal::SupportUnexpected("support:x".into())
    );
    for (support, state) in [
        (one_way.clone(), ActiveSetState::Sticking),
        (one_way.clone(), ActiveSetState::Sliding),
        (gap.clone(), ActiveSetState::Sliding),
        (friction.clone(), ActiveSetState::Active),
    ] {
        let id = support.support_id.clone();
        let found = refused(&[support], &[r(&id, state)]);
        assert!(matches!(&found, StateCaseRefusal::StatePairInvalid { support, state: s, .. } if *support == id && *s == state), "{found:?}");
    }
}

// ---------------------------------------------------------------------------
// Refused through the product path: NotEstablished, gap models, upstream
// ---------------------------------------------------------------------------

#[test]
fn a_not_established_state_is_refused_with_its_reason() {
    // Weakly active (R3): zero reaction and zero separation, or exactly at
    // the stick capacity; the static path's own sentence is carried.
    for (request, case) in [(kd5(U1), "load:IN-PLANE"), (kd5(F1), "load:BOUNDARY")] {
        let found = state_modal_run(state_request(&request, case)).err().unwrap();
        let last = found.last().unwrap();
        assert_eq!(last.code, MODAL_STATE_CASE_UNAVAILABLE, "{case}: {found:#?}");
        assert!(last.message.contains("its converged support state is not confirmed"), "{case}: {}", last.message);
        assert!(last.message.contains("not established"), "{case}: {}", last.message);
        // The static diagnostics of the solve are carried before it.
        assert!(codes(&found).contains(&"NONLINEAR_SUPPORT_LOOP_CONVERGED"), "{case}: {found:#?}");
    }
}

#[test]
fn every_gap_model_is_refused_for_now() {
    // The static path never confirms a gap state (`confirm && passed && !gap`).
    // On these models its qualification takes the inspectable-unqualified
    // branch, which is refused (D2); a qualified gap state would be
    // NotEstablished ("confirmation attempt: not run") and refused too.
    let mut u1 = kd5(U1);
    support_mut(&mut u1, "support:NL-U")["nonlinear"] =
        json!({"behavior": "gap", "dof": "UZ", "initial_state": "inactive", "closes_when": "negative_displacement", "gap": q(1e-5, "m")});
    let cases = [(u1.clone(), "load:PRESSED"), (u1, "load:LIFTED"), (arc_request(true), "case:z")];
    for (request, case) in cases {
        let found = state_modal_run(state_request(&request, case)).err().unwrap_or_else(|| panic!("{case}: a gap model is refused"));
        let last = found.last().unwrap();
        assert_eq!(last.code, MODAL_STATE_CASE_UNAVAILABLE, "{case}: {found:#?}");
        assert!(last.message.contains(&StateCaseRefusal::InspectableUnqualifiedGap.modal_refusal_reason()), "{case}: {}", last.message);
    }
}

/// A cantilever on n one-way UZ supports, all initially active, with
/// alternating vertical loads: the active-set loop does not settle within
/// its iteration limit (n = 4 is enough).
fn alternating_chain(n: usize) -> Value {
    let nodes: Vec<Value> = (0..=n).map(|i| json!({"id": format!("n{i}"), "position": {"x": i as f64, "y": 0, "z": 0}, "provenance": PROV})).collect();
    let pipes: Vec<Value> = (0..n)
        .map(|i| json!({"id": format!("p{i}"), "from": format!("n{i}"), "to": format!("n{}", i + 1), "section": section(), "material": "m",
            "y_reference": {"x": 0, "y": 1, "z": 0}, "provenance": PROV}))
        .collect();
    let mut supports = vec![json!({"id": "s", "node": "n0", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV})];
    let mut loads = Vec::new();
    for i in 1..=n {
        supports.push(json!({"id": format!("nl{i}"), "node": format!("n{i}"), "family": "nonlinear", "restraints": [],
            "nonlinear": {"behavior": "one_way", "dof": "UZ", "initial_state": "active", "active_when": "positive_reaction"}, "provenance": PROV}));
        loads.push(json!({"id": format!("l{i}"), "category": "concentrated_force", "target": {"type": "node", "node": format!("n{i}")},
            "direction": "global_z", "magnitude": q(if i % 2 == 0 { 1000.0 } else { -1000.0 }, "N"), "dimension": "force", "provenance": PROV}));
    }
    json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:i2b2-chain", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes,
        "materials": [{"id": "m", "elastic_modulus": q(200e9, "Pa"), "shear_modulus": q(80e9, "Pa"), "provenance": PROV}],
        "supports": supports, "components": [],
        "load_cases": [{"id": "case:c", "primitive_loads": loads, "provenance": PROV}], "combinations": []}, "materials": []})
}

#[test]
fn a_non_converged_state_case_is_refused_carrying_the_static_block() {
    let request = alternating_chain(4);
    let entry = crate::run_linear_static_preview_value_with_mode(request.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
    assert!(codes(&entry.diagnostics).contains(&"NONLINEAR_SUPPORT_LOOP_NOT_CONVERGED"));
    let found = refused_with(state_request(&request, "case:c"), "case:c", &StateCaseRefusal::NotConverged);
    assert!(codes(&found).contains(&"SOLVER_SYSTEM_BLOCKED"), "{found:#?}");
    // The blocked construction carries no evidence.
    let modal_request = modal(state_request(&request, "case:c"));
    let (solve, _) = static_state_solve(&prepare_modal(&modal_request).unwrap(), PreviewSolverMode::DenseScrutiny);
    assert!(solve.nonlinear_state.is_none() && solve.results.is_empty());
}

#[test]
fn the_retired_interim_code_is_gone_and_a_linear_model_has_no_state() {
    assert!(!include_str!("../modal.rs").contains("INTERIM_UNSUPPORTED"));
    let outcome = state_modal_solved(with(pf_request(false), json!({"mode_count": 2})));
    assert!(outcome.nonlinear_state.is_none());
}

#[test]
fn a_load_state_document_with_a_nonlinear_support_is_refused_upstream() {
    // A load/reference-state document is on the exact route, which does not
    // admit nonlinear supports yet (`exact_admission`): the static refusal is
    // carried with no modal bypass. Without one, the state case solves.
    let request: Value = serde_json::from_str(LOAD_STATE_REQUEST).unwrap();
    let mut nonlinear = request.clone();
    nonlinear["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "support:nl", "node": "node:middle", "family": "nonlinear",
        "restraints": [], "nonlinear": {"behavior": "one_way", "dof": "UY", "initial_state": "active", "active_when": "positive_reaction"},
        "provenance": "invented DEL-04-09 I2b-2"}));
    let entry = crate::run_linear_static_preview_value_with_mode(nonlinear.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
    let static_blocking: Vec<&str> = entry.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect();
    assert!(!static_blocking.is_empty());
    let found = state_modal_run(with(nonlinear, json!({"mode_count": 1, "state_case": "case:hot"}))).err().unwrap();
    for code in &static_blocking {
        assert!(codes(&found).contains(code), "{code}: {found:#?}");
    }
    let mut with_density = request;
    for pipe in with_density["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["material_density"] = q(7850.0, "kg/m^3");
    }
    match state_modal_run(with(with_density, json!({"mode_count": 1, "state_case": "case:hot"}))) {
        Ok(outcome) => assert!(outcome.nonlinear_state.is_none()),
        Err(found) => assert!(!codes(&found).contains(&MODAL_STATE_CASE_UNAVAILABLE), "{found:#?}"),
    }
}

// ---------------------------------------------------------------------------
// The seam on the static case solve (decision B, D3)
// ---------------------------------------------------------------------------

#[test]
fn the_seam_is_the_same_solves_states_bound_to_its_case_and_mode() {
    let u1 = kd5(U1);
    for (case, mode) in [
        ("load:PRESSED", PreviewSolverMode::DenseScrutiny),
        ("load:PRESSED", PreviewSolverMode::SparseInteractive),
        ("load:LIFTED", PreviewSolverMode::DenseScrutiny),
        ("load:IN-PLANE", PreviewSolverMode::DenseScrutiny),
    ] {
        let request = modal(state_request(&u1, case));
        let prepared = prepare_modal(&request).unwrap();
        let (solve, _) = static_state_solve(&prepared, mode);
        let evidence = solve.nonlinear_state.as_deref().expect("a completed converged case carries its evidence");
        assert_eq!((evidence.case_id.as_str(), evidence.solver_mode, evidence.converged), (case, mode, true));
        assert_eq!(evidence.qualification, NonlinearQualification::Qualified);
        assert_eq!(matches!(evidence.standing, NonlinearStateStanding::Confirmed), case != "load:IN-PLANE", "{case} {mode:?}");
        // The evidence's final states are the ones the case's own rows publish.
        for state in &evidence.final_states {
            let row = solve
                .results
                .iter()
                .find(|r| r.kind == "nonlinear_support_active_set_state_code" && r.entity_ref == state.support_id)
                .unwrap();
            assert_eq!(row.value, active_set_state_code(state.state), "{case} {mode:?}");
        }
        assert_eq!(evidence.final_states.len(), prepared.built.nonlinear_supports.len());
    }
    // The modal path records the dense mode it ran (D3).
    assert_eq!(MODAL_DENSE_PATH_MODE, PreviewSolverMode::DenseScrutiny);
    let state = solve_modal_state_case(&prepare_modal(&modal(state_request(&u1, "load:PRESSED"))).unwrap(), MODAL_DENSE_PATH_MODE)
        .unwrap()
        .unwrap();
    assert_eq!(state.solver_mode, PreviewSolverMode::DenseScrutiny);
    assert!(state.static_diagnostics.iter().all(|d| d.severity != "blocking"));
}

#[test]
fn a_linear_case_carries_no_evidence() {
    let request = modal(with(pf_request(false), json!({"mode_count": 1, "state_case": "case:f"})));
    let prepared = prepare_modal(&request).unwrap();
    let (solve, _) = static_state_solve(&prepared, PreviewSolverMode::DenseScrutiny);
    assert!(solve.nonlinear_state.is_none());
    assert!(!solve.results.is_empty());
}

/// By construction (the error-path proof's structural half): every
/// `LoadCaseSolve` construction in lib.rs but the completed one at the end of
/// the case solve sets `nonlinear_state: None`; the completed one takes the
/// captured value; and only the capture helper forms `Some` evidence, from a
/// converged solve with a recorded qualification.
#[test]
fn only_the_completed_case_construction_can_carry_evidence() {
    let lib = include_str!("../lib.rs");
    // Eleven `LoadCaseSolve {` tokens: the struct definition, the seven
    // blocked constructions, the completed one, and the test builder's
    // signature and construction.
    let pieces: Vec<&str> = lib.split("LoadCaseSolve {").collect();
    assert_eq!(pieces.len() - 1, 11, "every token is counted");
    let mut none = 0;
    let mut captured = 0;
    // Each construction's own field list is read up to its first
    // `nonlinear_state`, which must be the construction's own last field.
    for body in pieces[2..].iter().filter(|piece| !piece.trim().is_empty()) {
        let at = body.find("nonlinear_state").expect("every construction names nonlinear_state");
        assert!(body[..at].contains("withheld_summaries"), "nonlinear_state is set in the construction itself");
        let rest = &body[at + "nonlinear_state".len()..];
        if rest.starts_with(": None,") {
            none += 1;
        } else if rest.starts_with(",\n    })") {
            captured += 1;
        } else {
            panic!("a construction without an explicit nonlinear_state: {}", &body[..at + 40]);
        }
    }
    assert_eq!((none, captured), (8, 1));
    assert_eq!(lib.matches("Some(Box::new(NonlinearStateEvidence {").count(), 1);
    assert_eq!(lib.matches("let nonlinear_state = capture_nonlinear_state_evidence(").count(), 1);
    let helper = &lib[lib.find("fn capture_nonlinear_state_evidence(").unwrap()..];
    let helper = &helper[..helper.find("\n}\n").unwrap()];
    assert!(helper.contains("let solve = solve.filter(|solve| solve.converged)?;"));
    assert!(helper.contains("let qualification = qualification?;"));
    // The capture follows the blocked return of a non-converged or
    // unavailable nonlinear state, so that return carries none.
    let capture = lib.find("let nonlinear_state = capture_nonlinear_state_evidence(").unwrap();
    let blocked = lib.find("\"diagnostic:physics:selected-state-unavailable\"").unwrap();
    assert!(blocked < capture);
}

/// No prose parsing: the state module reads no diagnostic message text.
#[test]
fn the_state_module_reads_no_diagnostic_message() {
    let text = include_str!("state.rs");
    assert!(!text.contains(".message"));
    assert!(!text.contains("message."));
    assert!(!text.contains("NonlinearStanding"), "it reads the typed evidence, not the publication's standing");
}

/// A refusal's blocking codes are stated by their count (D3: their number has
/// no source bound), never repeated; the refusal keeps the codes themselves.
#[test]
fn a_refusals_blocking_codes_are_stated_by_count() {
    let codes = vec![String::from("SOLVER_SYSTEM_BLOCKED"); 1000];
    for refusal in [StateCaseRefusal::MissingEvidence(codes.clone()), StateCaseRefusal::Blocking(codes.clone())] {
        let reason = refusal.modal_refusal_reason();
        assert!(reason.contains("1000"), "{reason}");
        assert!(!reason.contains("SOLVER_SYSTEM_BLOCKED"), "{reason}");
        assert!(reason.len() < 160, "{} B: {reason}", reason.len());
    }
    assert!(StateCaseRefusal::MissingEvidence(Vec::new()).modal_refusal_reason().contains("(0 blocking diagnostic(s)"));
    assert_eq!(StateCaseRefusal::Blocking(codes.clone()), StateCaseRefusal::Blocking(codes));
}

/// R-4 §3.7 S6: a load/reference-state state case with a moved nonlinear
/// ground and a moved spring base (`validation/references/rt_a3_s6`'s C-5
/// twin). The static case solve's evidence carries the motion it consumed;
/// the modal path admits it against the bound case's own resolved motion
/// and linearizes the confirmed state with no second active-set solve.
/// Evidence of another case of the same model (a replay) is refused.
#[test]
fn an_s6_state_case_binds_its_moved_ground_and_spring_base() {
    let mut twin: Value = serde_json::from_str(include_str!("../../../../validation/references/rt_a3_s6/requests/ls1_c5.json")).unwrap();
    for pipe in twin["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["material_density"] = q(7850.0, "kg/m^3");
    }
    let unequal = modal(state_request(&twin, "load:UNEQUAL"));
    let prepared = prepare_modal(&unequal).unwrap();
    let (solve, diagnostics) = static_state_solve(&prepared, PreviewSolverMode::DenseScrutiny);
    let evidence = solve.nonlinear_state.as_deref().expect("a confirmed S6 case carries its evidence");
    assert_eq!(evidence.case_id, "load:UNEQUAL");
    assert!(matches!(evidence.standing, NonlinearStateStanding::Confirmed));
    let motion = CaseMotionEvidence::resolve(
        &prepared.model,
        &prepared.model.load_cases[prepared.bound_case.as_ref().unwrap().case_index],
        prepared.state_resolution.as_ref(),
        &prepared.boundary.restrained_dofs,
        &prepared.boundary.springs,
    );
    assert_eq!(evidence.motion, motion);
    assert_eq!(motion.ground_motion, vec![("support:LIFT".to_string(), (-0.00048828125f64).to_bits())]);
    assert!(motion.spring_ground.contains(&(-0.000244140625f64).to_bits()));
    assert!(admit_modal_state_evidence(Some(evidence), "load:UNEQUAL", &motion, PreviewSolverMode::DenseScrutiny, &diagnostics).is_ok());
    // The same-run modal state case admits it and linearizes the active lift-off.
    let state = solve_modal_state_case(&prepared, MODAL_DENSE_PATH_MODE).unwrap().unwrap();
    assert_eq!(state.case_id, "load:UNEQUAL");
    assert_eq!(state.supports.iter().map(|s| (s.support_id.as_str(), s.state)).collect::<Vec<_>>(), [("support:LIFT", ActiveSetState::Active)]);
    // A replay: the ACTIVE case's evidence (equal grounds) presented for UNEQUAL.
    let active = modal(state_request(&twin, "load:ACTIVE"));
    let active_prepared = prepare_modal(&active).unwrap();
    let (active_solve, _) = static_state_solve(&active_prepared, PreviewSolverMode::DenseScrutiny);
    let mut replayed = active_solve.nonlinear_state.as_deref().unwrap().clone();
    replayed.case_id = "load:UNEQUAL".to_string();
    assert_eq!(
        admit_modal_state_evidence(Some(&replayed), "load:UNEQUAL", &motion, PreviewSolverMode::DenseScrutiny, &[]).err().unwrap(),
        StateCaseRefusal::MotionMismatch
    );
}

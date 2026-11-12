//! Nonlinear supports at the named state case (DEL-04-09 Design §1.2, §1.3,
//! §1.4, D-s; increment I2b-2).
//!
//! The state case is solved **once**, in the same run, through the static
//! path's own private case solve (`solve_load_case_observed`) on the prepared
//! model, build, boundary, materials and load state, with no capture, no
//! product observer and a fresh, scratch source budget, in the modal path's
//! solver mode (`dense_scrutiny`, recorded; D3). The static resource guards
//! the static entry applies to a nonlinear invocation run first, as it runs
//! them. Nothing is stored or reused, and the static entry is not changed.
//!
//! **Admission (Agent 0's decisions B and D2).** The solve's typed evidence
//! (`LoadCaseSolve::nonlinear_state`, filled only on a completed case whose
//! active-set solve converged and passed qualification) admits the state
//! only when it is present and bound to the state case, its mode is the
//! requested `dense_scrutiny`, it converged, its qualification is `Qualified`
//! (not the converged-but-unqualified inspectable gap state), its standing is
//! `Confirmed`, and the solve published no blocking diagnostic. Anything else
//! refuses `MODAL_STATE_CASE_UNAVAILABLE` with the specific reason, carrying
//! every static diagnostic of the solve. The static path never confirms a
//! model with a gap support, so **every model with a gap support is refused
//! for now** (a stated limitation). No diagnostic message is read: admission
//! reads the typed evidence and diagnostic codes and severities only.
//!
//! **Linearization (§1.3's table).** Exactly one final state per built
//! nonlinear support, by support id (a missing, duplicate or unexpected id
//! refuses), and only the table's behaviour and state pairs: one-way,
//! lift-off and gap are active (rigid) or inactive (free); friction is
//! sticking (rigid), sliding (free, the friction force frozen, disclosed) or
//! inactive (free: its normal does not bear, and the static path prescribes
//! no displacement for an inactive state, `prescribed_displacement_for_state`
//! in `nonlinear_integration`; Agent 0's release of O-14 (b)). A friction
//! support is never active. The DOF is the built support's (node index × 6 +
//! DOF index). A rigid state adds its DOF to the restrained set with no
//! imposed or gap value; a DOF a linear support already restrains stays
//! restrained and is recorded. A friction support's normal is its own
//! support (or an explicit value), which keeps its own state.
use super::prepare::Prepared;
use super::MODAL_STATE_CASE_UNAVAILABLE;
use crate::{
    dense_loop_memory_guard, dense_scrutiny_ceiling_bytes, dense_scrutiny_guard, dense_scrutiny_refusal_diagnostic, diag, dof_index,
    form_basis_stiffness, has_blocking, solve_load_case_observed, stable_suffix, ActiveSetState, CaseMotionEvidence, Diagnostic, NonlinearQualification,
    NonlinearStateEvidence, NonlinearStateStanding, NonlinearSupport, NonlinearSupportBehavior, PreviewSolverMode, SourceRecoveryBudget,
    SupportStateRecord, DENSE_LOOP_MEMORY_CEILING_BYTES, DOF_PER_NODE,
};
use std::collections::{BTreeSet, HashMap};

/// The code of the static path's not-converged loop diagnostic, read by code
/// only to name the reason (never its message).
const LOOP_NOT_CONVERGED: &str = "NONLINEAR_SUPPORT_LOOP_NOT_CONVERGED";

/// How one nonlinear support enters the modal model (§1.3's table).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModalSupportTreatment {
    /// Active or sticking: its DOF is added to the restrained set; an imposed
    /// ground or gap value does not enter.
    Restrained,
    /// Active or sticking on a DOF a linear support already restrains: it
    /// stays restrained and nothing is added.
    AlreadyRestrained,
    /// Inactive one-way, lift-off, gap or friction (its normal does not
    /// bear): its DOF is free.
    Free,
    /// Sliding friction: its tangential DOF is free and the friction force is
    /// frozen, so it contributes no stiffness (a disclosed approximation).
    FreeFrozenFriction,
}

/// One nonlinear support's state at the state case and its modal treatment.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ModalSupportState {
    pub(crate) support_id: String,
    pub(crate) behavior: &'static str,
    /// node_index × 6 + DOF index, from the built support.
    pub(crate) global_dof: usize,
    pub(crate) state: ActiveSetState,
    pub(crate) treatment: ModalSupportTreatment,
}

/// The linearized state case the modal model is formed at.
#[derive(Debug, Clone)]
pub(crate) struct ModalStateCase {
    pub(crate) case_id: String,
    /// The solver mode the state solve ran and its evidence recorded.
    pub(crate) solver_mode: PreviewSolverMode,
    /// In the built supports' order.
    pub(crate) supports: Vec<ModalSupportState>,
    /// The linear restrained DOFs with every rigid state's DOF added,
    /// ascending, without repeats.
    pub(crate) restrained_dofs: Vec<usize>,
    pub(crate) disclosures: Vec<String>,
    /// Every diagnostic of the state solve (none blocking).
    pub(crate) static_diagnostics: Vec<Diagnostic>,
}

/// Why the state case defines no modal model.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StateCaseRefusal {
    /// The model has nonlinear supports and no bound state case.
    NoBoundCase,
    /// The static resource guard of a nonlinear or dense invocation refused.
    ResourceGuard,
    /// The static case solve returned an error before completing.
    SolveFailed(String),
    /// No evidence: the active-set loop did not converge.
    NotConverged,
    /// No evidence, and the solve's blocking diagnostic codes.
    MissingEvidence(Vec<String>),
    CaseMismatch { expected: String, found: String },
    /// R-4 §3.7 S6: the evidence's consumed motion is not the bound case's.
    MotionMismatch,
    ModeMismatch { requested: PreviewSolverMode, solved: PreviewSolverMode },
    /// Only the dense path is built; the state solve runs only in its mode.
    ModeUnsupported(PreviewSolverMode),
    InspectableUnqualifiedGap,
    /// The case's own fail-closed sentence.
    NotEstablished(String),
    /// The solve published blocking diagnostics (their codes).
    Blocking(Vec<String>),
    SupportMissing(String),
    SupportDuplicate(String),
    SupportUnexpected(String),
    StatePairInvalid { support: String, behavior: &'static str, state: ActiveSetState },
}

impl StateCaseRefusal {
    /// The refusal's reason text. A list of blocking codes is stated by its
    /// count, not repeated: nothing bounds how many a solve publishes, and the
    /// diagnostics themselves are carried with the refusal.
    pub(crate) fn modal_refusal_reason(&self) -> String {
        match self {
            Self::NoBoundCase => "the model has nonlinear supports and the request binds no state case".into(),
            Self::ResourceGuard => "the static path's resource guard refuses the state solve".into(),
            Self::SolveFailed(error) => format!("its same-run solve failed ({error})"),
            Self::NotConverged => "its same-run active-set solve did not converge".into(),
            Self::MissingEvidence(codes) => format!(
                "its same-run solve completed with no nonlinear-state evidence ({} blocking diagnostic(s), carried with this one)",
                codes.len()
            ),
            Self::CaseMismatch { expected, found } => format!("the solve's evidence is bound to case `{found}`, not `{expected}`"),
            Self::MotionMismatch => "the solve's evidence records boundary motion other than the state case's own resolved motion".into(),
            Self::ModeMismatch { requested, solved } => {
                format!("the solve's evidence records solver mode {}, not the requested {}", solved.as_str(), requested.as_str())
            }
            Self::ModeUnsupported(mode) => format!("the state solve runs only in dense_scrutiny, the modal dense path's mode, not {}", mode.as_str()),
            Self::InspectableUnqualifiedGap => {
                "its converged gap state is unqualified (inspectable only), so no confirmed support state defines the modal model".into()
            }
            Self::NotEstablished(sentence) => format!("its converged support state is not confirmed: {sentence}"),
            Self::Blocking(codes) => {
                format!("its same-run solve published blocking diagnostics ({} of them, carried with this one)", codes.len())
            }
            Self::SupportMissing(id) => format!("the solve's final states omit nonlinear support `{id}`"),
            Self::SupportDuplicate(id) => format!("the solve's final states name nonlinear support `{id}` more than once"),
            Self::SupportUnexpected(id) => format!("the solve's final states name `{id}`, which is not a built nonlinear support"),
            Self::StatePairInvalid { support, behavior, state } => format!(
                "nonlinear support `{support}` ({behavior}) ended {}, which the modal state table does not admit",
                state.as_str()
            ),
        }
    }
}

fn modal_state_blocking_codes(diagnostics: &[Diagnostic]) -> Vec<String> {
    diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect()
}

/// The refusal: every static diagnostic of the solve, then the named code.
fn state_case_refusal(mut diagnostics: Vec<Diagnostic>, case: &str, refusal: &StateCaseRefusal) -> Vec<Diagnostic> {
    diagnostics.push(diag(
        &format!("diagnostic:modal:state-case:{}:state-solve", stable_suffix(case)),
        MODAL_STATE_CASE_UNAVAILABLE,
        "blocking",
        format!(
            "state case `{case}` defines no modal model: {}; the static diagnostics of its same-run solve are carried",
            refusal.modal_refusal_reason()
        ),
        vec!["state_case".to_string(), case.to_string()],
    ));
    diagnostics
}

/// Admission of the solve's evidence (module docs). Pure: seam tests reach
/// every combination with constructed values.
pub(crate) fn admit_modal_state_evidence<'a>(
    evidence: Option<&'a NonlinearStateEvidence>,
    case_id: &str,
    motion: &CaseMotionEvidence,
    requested: PreviewSolverMode,
    diagnostics: &[Diagnostic],
) -> Result<&'a NonlinearStateEvidence, StateCaseRefusal> {
    let Some(evidence) = evidence else {
        if diagnostics.iter().any(|d| d.code == LOOP_NOT_CONVERGED) {
            return Err(StateCaseRefusal::NotConverged);
        }
        return Err(StateCaseRefusal::MissingEvidence(modal_state_blocking_codes(diagnostics)));
    };
    if evidence.case_id != case_id {
        return Err(StateCaseRefusal::CaseMismatch { expected: case_id.to_string(), found: evidence.case_id.clone() });
    }
    if &evidence.motion != motion {
        return Err(StateCaseRefusal::MotionMismatch);
    }
    if requested != PreviewSolverMode::DenseScrutiny {
        return Err(StateCaseRefusal::ModeUnsupported(requested));
    }
    if evidence.solver_mode != requested {
        return Err(StateCaseRefusal::ModeMismatch { requested, solved: evidence.solver_mode });
    }
    if !evidence.converged {
        return Err(StateCaseRefusal::NotConverged);
    }
    if evidence.qualification == NonlinearQualification::InspectableUnqualifiedGap {
        return Err(StateCaseRefusal::InspectableUnqualifiedGap);
    }
    if let NonlinearStateStanding::NotEstablished { reason } = &evidence.standing {
        return Err(StateCaseRefusal::NotEstablished(reason.clone()));
    }
    if has_blocking(diagnostics) {
        return Err(StateCaseRefusal::Blocking(modal_state_blocking_codes(diagnostics)));
    }
    Ok(evidence)
}

fn behavior_name(behavior: NonlinearSupportBehavior) -> &'static str {
    match behavior {
        NonlinearSupportBehavior::OneWay { .. } => "one_way",
        NonlinearSupportBehavior::LiftOff { .. } => "lift_off",
        NonlinearSupportBehavior::Gap { .. } => "gap",
        NonlinearSupportBehavior::Friction => "friction",
    }
}

/// §1.3's table: `Some(true)` rigid, `Some(false)` free, `None` not admitted.
fn state_is_rigid(behavior: NonlinearSupportBehavior, state: ActiveSetState) -> Option<bool> {
    match (behavior, state) {
        (NonlinearSupportBehavior::Friction, ActiveSetState::Sticking) => Some(true),
        (NonlinearSupportBehavior::Friction, ActiveSetState::Sliding | ActiveSetState::Inactive) => Some(false),
        (NonlinearSupportBehavior::Friction, ActiveSetState::Active) => None,
        (_, ActiveSetState::Active) => Some(true),
        (_, ActiveSetState::Inactive) => Some(false),
        (_, ActiveSetState::Sticking | ActiveSetState::Sliding) => None,
    }
}

/// The supports' modal treatment and the augmented restrained set (module
/// docs). Pure: seam tests reach every combination with constructed values.
pub(crate) fn linearize_modal_supports(
    supports: &[NonlinearSupport],
    restrained: &[usize],
    final_states: &[SupportStateRecord],
) -> Result<(Vec<ModalSupportState>, Vec<usize>), StateCaseRefusal> {
    let mut states: HashMap<&str, ActiveSetState> = HashMap::new();
    for record in final_states {
        if !supports.iter().any(|support| support.support_id == record.support_id) {
            return Err(StateCaseRefusal::SupportUnexpected(record.support_id.clone()));
        }
        if states.insert(record.support_id.as_str(), record.state).is_some() {
            return Err(StateCaseRefusal::SupportDuplicate(record.support_id.clone()));
        }
    }
    let linear: BTreeSet<usize> = restrained.iter().copied().collect();
    let mut augmented = linear.clone();
    let mut treated = Vec::with_capacity(supports.len());
    for support in supports {
        let Some(&state) = states.get(support.support_id.as_str()) else {
            return Err(StateCaseRefusal::SupportMissing(support.support_id.clone()));
        };
        let behavior = behavior_name(support.behavior);
        let rigid = state_is_rigid(support.behavior, state).ok_or_else(|| StateCaseRefusal::StatePairInvalid {
            support: support.support_id.clone(),
            behavior,
            state,
        })?;
        let global_dof = support.node_index * DOF_PER_NODE + dof_index(support.dof);
        let treatment = match (rigid, support.behavior) {
            (true, _) if linear.contains(&global_dof) => ModalSupportTreatment::AlreadyRestrained,
            (true, _) => {
                augmented.insert(global_dof);
                ModalSupportTreatment::Restrained
            }
            (false, NonlinearSupportBehavior::Friction) if state == ActiveSetState::Sliding => ModalSupportTreatment::FreeFrozenFriction,
            (false, _) => ModalSupportTreatment::Free,
        };
        treated.push(ModalSupportState { support_id: support.support_id.clone(), behavior, global_dof, state, treatment });
    }
    Ok((treated, augmented.into_iter().collect()))
}

fn modal_state_disclosures(case: &str, mode: PreviewSolverMode, supports: &[ModalSupportState]) -> Vec<String> {
    let mut out = vec![
        format!(
            "nonlinear supports are linearized at state case `{case}`'s confirmed converged support states, from its same-run {} solve; the result holds only at those states, and a different operating state gives different frequencies",
            mode.as_str()
        ),
        "active one-way and lift-off supports, closed gaps and sticking friction restrain their DOF rigidly, with no finite contact stiffness; an imposed ground or gap value does not enter; inactive and open supports leave their DOF free".to_string(),
    ];
    if supports.iter().any(|s| s.behavior == "friction") {
        out.push("a friction support restrains only its tangential DOF; its normal restraint is its own support (or an explicit value) and keeps its own state".into());
    }
    for support in supports {
        match support.treatment {
            ModalSupportTreatment::FreeFrozenFriction => out.push(format!(
                "friction support `{}` slides at the state case: its tangential DOF is free with the friction force frozen, contributing no stiffness; this is a modal approximation (the static solver's sliding tangent is nonsymmetric, which a symmetric-definite eigenproblem cannot represent)",
                support.support_id
            )),
            ModalSupportTreatment::AlreadyRestrained => out.push(format!(
                "nonlinear support `{}` is {} on a DOF a linear support already restrains; it stays restrained",
                support.support_id,
                support.state.as_str()
            )),
            ModalSupportTreatment::Free if support.behavior == "friction" => out.push(format!(
                "friction support `{}` is inactive at the state case (its normal does not bear): its tangential DOF is free and it carries no friction force",
                support.support_id
            )),
            ModalSupportTreatment::Restrained | ModalSupportTreatment::Free => {}
        }
    }
    out
}

/// The state case's same-run solve and linearization (module docs); `None`
/// for a model with no nonlinear support.
pub(crate) fn solve_modal_state_case(prepared: &Prepared, mode: PreviewSolverMode) -> Result<Option<ModalStateCase>, Vec<Diagnostic>> {
    let built = &prepared.built;
    if built.nonlinear_supports.is_empty() {
        return Ok(None);
    }
    let boundary = &prepared.boundary;
    let Some(bound) = &prepared.bound_case else {
        return Err(state_case_refusal(Vec::new(), "", &StateCaseRefusal::NoBoundCase));
    };
    let case_id = bound.case_id.as_str();
    let mut diagnostics = Vec::new();

    // The static entry's guards for a nonlinear invocation, in its order.
    if let Err(refusal) = dense_loop_memory_guard(built.nodes.len(), &boundary.restrained_dofs, DENSE_LOOP_MEMORY_CEILING_BYTES) {
        diagnostics.push(diag(
            "diagnostic:physics:nonlinear-dense-memory-guard",
            "NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED",
            "blocking",
            format!("{refusal}; it applies in sparse_interactive and dense_scrutiny alike"),
            vec!["model".to_string()],
        ));
        return Err(state_case_refusal(diagnostics, case_id, &StateCaseRefusal::ResourceGuard));
    }
    if mode == PreviewSolverMode::DenseScrutiny {
        if let Err(refusal) = dense_scrutiny_guard(built.nodes.len() * DOF_PER_NODE, dense_scrutiny_ceiling_bytes()) {
            diagnostics.push(dense_scrutiny_refusal_diagnostic(&refusal));
            return Err(state_case_refusal(diagnostics, case_id, &StateCaseRefusal::ResourceGuard));
        }
    }
    // The static entry's basis stiffness of a nonlinear invocation (`linear`
    // false), and its one case solve.
    let stiffness = match form_basis_stiffness(built, &boundary.springs, false) {
        Ok(stiffness) => stiffness,
        Err(error) => {
            diagnostics.push(diag("diagnostic:physics:solver", "SOLVER_SYSTEM_BLOCKED", "blocking", error.to_string(), vec!["model".to_string()]));
            return Err(state_case_refusal(diagnostics, case_id, &StateCaseRefusal::SolveFailed(error.to_string())));
        }
    };
    let mut scratch_budget = SourceRecoveryBudget::default();
    let solved = solve_load_case_observed(
        &prepared.model,
        built,
        &prepared.materials,
        &stiffness,
        &boundary.restrained_dofs,
        &boundary.springs,
        &prepared.model.load_cases[bound.case_index],
        prepared.state_basis_record.as_deref(),
        mode,
        None,
        &mut scratch_budget,
        prepared.state_resolution.as_ref(),
        &mut diagnostics,
        None,
        crate::held_status::CaseRole::Own(None),
    );
    let solve = match solved {
        Ok(solve) => solve,
        Err(error) => {
            diagnostics.push(diag("diagnostic:physics:solver", "SOLVER_SYSTEM_BLOCKED", "blocking", error.to_string(), vec!["model".to_string()]));
            return Err(state_case_refusal(diagnostics, case_id, &StateCaseRefusal::SolveFailed(error.to_string())));
        }
    };
    // R-4 §3.7 S6: the state case's own motion, by the static resolvers.
    let motion = CaseMotionEvidence::resolve(
        &prepared.model,
        &prepared.model.load_cases[bound.case_index],
        prepared.state_resolution.as_ref(),
        &boundary.restrained_dofs,
        &boundary.springs,
    );
    let evidence = match admit_modal_state_evidence(solve.nonlinear_state.as_deref(), case_id, &motion, mode, &diagnostics) {
        Ok(evidence) => evidence,
        Err(refusal) => return Err(state_case_refusal(diagnostics, case_id, &refusal)),
    };
    let (supports, restrained_dofs) = match linearize_modal_supports(&built.nonlinear_supports, &boundary.restrained_dofs, &evidence.final_states) {
        Ok(linearized) => linearized,
        Err(refusal) => return Err(state_case_refusal(diagnostics, case_id, &refusal)),
    };
    let disclosures = modal_state_disclosures(case_id, evidence.solver_mode, &supports);
    Ok(Some(ModalStateCase {
        case_id: evidence.case_id.clone(),
        solver_mode: evidence.solver_mode,
        supports,
        restrained_dofs,
        disclosures,
        static_diagnostics: diagnostics,
    }))
}

//! Nonlinear support active-set mechanics.
//!
//! This crate classifies nonlinear support states for iterative mechanics
//! solves. It does not assemble a global solve, encode design-code checks,
//! provide support catalog defaults, or make professional approval claims.

use open_pipe_stress_linear_supports::{
    CanonicalDimension, FrameDof, QuantityUnitMetadata, UnitSystemRef,
};
use open_pipe_stress_solver_diagnostics::{
    convergence_diagnostic, DiagnosticSeverity, DiagnosticSource, SolverDiagnostic,
    SolverDiagnosticCode,
};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationSense {
    PositiveReaction,
    NegativeReaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapDirection {
    PositiveDisplacement,
    NegativeDisplacement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonlinearSupportBehavior {
    OneWay { active_when: ActivationSense },
    Gap { closes_when: GapDirection },
    LiftOff { contact_when: ActivationSense },
    Friction,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearSupport {
    pub support_id: String,
    pub node_index: usize,
    pub dof: FrameDof,
    pub behavior: NonlinearSupportBehavior,
    pub gap: Option<f64>,
    pub friction_coefficient: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearSupportUnitMetadata {
    pub unit_system_ref: UnitSystemRef,
    pub displacement_unit: QuantityUnitMetadata,
    pub reaction_unit: QuantityUnitMetadata,
    pub friction_coefficient_unit: QuantityUnitMetadata,
}

impl NonlinearSupportUnitMetadata {
    pub fn new(
        unit_system_ref: UnitSystemRef,
        displacement_unit: QuantityUnitMetadata,
        reaction_unit: QuantityUnitMetadata,
        friction_coefficient_unit: QuantityUnitMetadata,
    ) -> Option<Self> {
        let metadata = Self {
            unit_system_ref,
            displacement_unit,
            reaction_unit,
            friction_coefficient_unit,
        };
        metadata.has_expected_dimensions().then_some(metadata)
    }

    pub fn has_expected_dimensions(&self) -> bool {
        self.displacement_unit.dimension == CanonicalDimension::Displacement
            && self.reaction_unit.dimension == CanonicalDimension::Force
            && self.friction_coefficient_unit.dimension == CanonicalDimension::Dimensionless
    }
}

impl NonlinearSupport {
    pub fn one_way(
        support_id: impl Into<String>,
        node_index: usize,
        dof: FrameDof,
        active_when: ActivationSense,
    ) -> Self {
        Self {
            support_id: support_id.into(),
            node_index,
            dof,
            behavior: NonlinearSupportBehavior::OneWay { active_when },
            gap: None,
            friction_coefficient: None,
        }
    }

    pub fn gap(
        support_id: impl Into<String>,
        node_index: usize,
        dof: FrameDof,
        clearance: f64,
        closes_when: GapDirection,
    ) -> Result<Self, NonlinearSupportError> {
        validate_nonnegative_finite("gap clearance", clearance)?;
        Ok(Self {
            support_id: support_id.into(),
            node_index,
            dof,
            behavior: NonlinearSupportBehavior::Gap { closes_when },
            gap: Some(clearance),
            friction_coefficient: None,
        })
    }

    pub fn lift_off(
        support_id: impl Into<String>,
        node_index: usize,
        dof: FrameDof,
        contact_when: ActivationSense,
    ) -> Self {
        Self {
            support_id: support_id.into(),
            node_index,
            dof,
            behavior: NonlinearSupportBehavior::LiftOff { contact_when },
            gap: None,
            friction_coefficient: None,
        }
    }

    pub fn friction(
        support_id: impl Into<String>,
        node_index: usize,
        dof: FrameDof,
        friction_coefficient: f64,
    ) -> Result<Self, NonlinearSupportError> {
        validate_nonnegative_finite("friction coefficient", friction_coefficient)?;
        Ok(Self {
            support_id: support_id.into(),
            node_index,
            dof,
            behavior: NonlinearSupportBehavior::Friction,
            gap: None,
            friction_coefficient: Some(friction_coefficient),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveSetState {
    Active,
    Inactive,
    Sticking,
    Sliding,
}

impl ActiveSetState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Sticking => "sticking",
            Self::Sliding => "sliding",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrialSupportState {
    pub support_id: String,
    pub displacement: f64,
    pub reaction: f64,
    pub normal_reaction: Option<f64>,
    pub tangential_reaction: Option<f64>,
}

impl TrialSupportState {
    pub fn new(
        support_id: impl Into<String>,
        displacement: f64,
        reaction: f64,
    ) -> Result<Self, NonlinearSupportError> {
        validate_finite("displacement", displacement)?;
        validate_finite("reaction", reaction)?;
        Ok(Self {
            support_id: support_id.into(),
            displacement,
            reaction,
            normal_reaction: None,
            tangential_reaction: None,
        })
    }

    pub fn with_friction_reactions(
        mut self,
        normal_reaction: f64,
        tangential_reaction: f64,
    ) -> Result<Self, NonlinearSupportError> {
        validate_finite("normal reaction", normal_reaction)?;
        validate_finite("tangential reaction", tangential_reaction)?;
        self.normal_reaction = Some(normal_reaction);
        self.tangential_reaction = Some(tangential_reaction);
        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportStateRecord {
    pub support_id: String,
    pub state: ActiveSetState,
}

impl SupportStateRecord {
    pub fn new(support_id: impl Into<String>, state: ActiveSetState) -> Self {
        Self {
            support_id: support_id.into(),
            state,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveSetIterationInput {
    pub iteration: usize,
    pub max_iterations: usize,
    pub tolerance: f64,
    pub supports: Vec<NonlinearSupport>,
    pub trial_states: Vec<TrialSupportState>,
    pub prior_states: Vec<SupportStateRecord>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveSetIteration {
    pub iteration: usize,
    pub states: Vec<SupportStateRecord>,
    pub changed_supports: Vec<String>,
    pub residual_norm: f64,
    pub converged: bool,
    pub diagnostics: Vec<SolverDiagnostic>,
}

impl ActiveSetIteration {
    pub fn is_blocked(&self) -> bool {
        self.diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic.severity,
                DiagnosticSeverity::Blocking | DiagnosticSeverity::Failure
            )
        })
    }

    pub fn to_report_record(
        &self,
        input: &ActiveSetIterationInput,
    ) -> Result<ActiveSetReportRecord, NonlinearSupportError> {
        validate_nonnegative_finite("tolerance", input.tolerance)?;
        Ok(ActiveSetReportRecord {
            iteration: self.iteration,
            max_iterations: input.max_iterations,
            tolerance: input.tolerance,
            residual_norm: self.residual_norm,
            converged: self.converged,
            support_states: self
                .states
                .iter()
                .map(|state| {
                    ActiveSetSupportReportState::new(
                        state.support_id.clone(),
                        state.state,
                        self.changed_supports.contains(&state.support_id),
                    )
                })
                .collect(),
            changed_supports: self.changed_supports.clone(),
            diagnostics: self.diagnostics.clone(),
            assumptions: active_set_report_assumptions(),
            limitations: active_set_report_limitations(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveSetSupportReportState {
    pub support_id: String,
    pub state: ActiveSetState,
    pub changed: bool,
}

impl ActiveSetSupportReportState {
    pub fn new(support_id: impl Into<String>, state: ActiveSetState, changed: bool) -> Self {
        Self {
            support_id: support_id.into(),
            state,
            changed,
        }
    }

    pub fn state_label(&self) -> &'static str {
        self.state.as_str()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveSetReportRecord {
    pub iteration: usize,
    pub max_iterations: usize,
    pub tolerance: f64,
    pub residual_norm: f64,
    pub converged: bool,
    pub support_states: Vec<ActiveSetSupportReportState>,
    pub changed_supports: Vec<String>,
    pub diagnostics: Vec<SolverDiagnostic>,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
}

impl ActiveSetReportRecord {
    pub fn support_state(&self, support_id: &str) -> Option<&ActiveSetSupportReportState> {
        self.support_states
            .iter()
            .find(|state| state.support_id == support_id)
    }
}

pub fn active_set_report_assumptions() -> Vec<String> {
    vec![
        "Trial displacement and reaction facts are supplied by an external mechanics solve."
            .to_string(),
        "Active-set residual norm is the count of support states changed in this bounded iteration record."
            .to_string(),
        "Support sign conventions are those encoded on each nonlinear support behavior.".to_string(),
        "A friction support that was already sliding remains sliding through a released DOF while nonzero trial displacement persists; this is deterministic anti-chatter state logic, not a derived friction load model."
            .to_string(),
    ]
}

pub fn active_set_report_limitations() -> Vec<String> {
    vec![
        "This crate does not assemble or solve the global nonlinear system.".to_string(),
        "Production tolerance policy, sparse solver selection, final constraint strategy, and result-envelope integration remain outside this bounded crate."
            .to_string(),
        "The record is mechanics-reporting evidence only and does not state rule compliance or professional approval."
            .to_string(),
    ]
}

#[derive(Debug, Clone, PartialEq)]
pub enum NonlinearSupportError {
    NonFiniteInput { name: &'static str, value: f64 },
    NegativeInput { name: &'static str, value: f64 },
    MissingGap { support_id: String },
    MissingFrictionCoefficient { support_id: String },
    MissingTrialState { support_id: String },
    MissingFrictionData { support_id: String },
}

impl fmt::Display for NonlinearSupportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::NegativeInput { name, value } => {
                write!(f, "{name} must be nonnegative, got {value}")
            }
            Self::MissingGap { support_id } => {
                write!(
                    f,
                    "missing gap clearance for nonlinear support {support_id}"
                )
            }
            Self::MissingFrictionCoefficient { support_id } => {
                write!(
                    f,
                    "missing friction coefficient for nonlinear support {support_id}"
                )
            }
            Self::MissingTrialState { support_id } => {
                write!(f, "missing trial state for nonlinear support {support_id}")
            }
            Self::MissingFrictionData { support_id } => {
                write!(
                    f,
                    "missing friction reaction data for nonlinear support {support_id}"
                )
            }
        }
    }
}

impl Error for NonlinearSupportError {}

pub fn diagnostic_from_nonlinear_support_error(error: &NonlinearSupportError) -> SolverDiagnostic {
    match error {
        NonlinearSupportError::MissingGap { support_id } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidRestraint,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("missing gap clearance for nonlinear support {support_id}"),
        )
        .with_affected_ref(support_id.clone())
        .with_remediation("Provide explicit gap clearance before active-set classification."),
        NonlinearSupportError::MissingFrictionCoefficient { support_id } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidRestraint,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("missing friction coefficient for nonlinear support {support_id}"),
        )
        .with_affected_ref(support_id.clone())
        .with_remediation(
            "Provide an explicit friction coefficient before friction support classification.",
        ),
        NonlinearSupportError::MissingTrialState { support_id } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidRestraint,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("missing trial state for nonlinear support {support_id}"),
        )
        .with_affected_ref(support_id.clone())
        .with_remediation(
            "Provide trial displacement and reaction facts before active-set classification.",
        ),
        NonlinearSupportError::MissingFrictionData { support_id } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidRestraint,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("missing friction reaction data for nonlinear support {support_id}"),
        )
        .with_affected_ref(support_id.clone())
        .with_remediation(
            "Provide normal and tangential reaction facts for friction support classification.",
        ),
        NonlinearSupportError::NonFiniteInput { .. }
        | NonlinearSupportError::NegativeInput { .. } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            error.to_string(),
        )
        .with_remediation("Correct nonlinear support numeric inputs before mechanics solve use."),
    }
}

pub fn evaluate_active_set_iteration(
    input: &ActiveSetIterationInput,
) -> Result<ActiveSetIteration, NonlinearSupportError> {
    validate_nonnegative_finite("tolerance", input.tolerance)?;

    let mut states = Vec::with_capacity(input.supports.len());
    let mut changed_supports = Vec::new();

    for support in &input.supports {
        let trial = input
            .trial_states
            .iter()
            .find(|candidate| candidate.support_id == support.support_id)
            .ok_or_else(|| NonlinearSupportError::MissingTrialState {
                support_id: support.support_id.clone(),
            })?;

        let prior_state = input
            .prior_states
            .iter()
            .find(|prior| prior.support_id == support.support_id)
            .map(|prior| prior.state);
        let state = classify_iteration_support_state(support, trial, prior_state)?;
        let changed = prior_state.is_none_or(|prior| prior != state);

        if changed {
            changed_supports.push(support.support_id.clone());
        }

        states.push(SupportStateRecord::new(support.support_id.clone(), state));
    }

    states.sort_by(|left, right| left.support_id.cmp(&right.support_id));
    changed_supports.sort();

    let residual_norm = changed_supports.len() as f64;
    let mut diagnostics = Vec::new();
    if let Some(diagnostic) = convergence_diagnostic(
        input.iteration,
        input.max_iterations,
        residual_norm,
        input.tolerance,
    )
    .map_err(|error| match error {
        open_pipe_stress_solver_diagnostics::DiagnosticsError::NonFiniteInput { name, value } => {
            NonlinearSupportError::NonFiniteInput { name, value }
        }
        open_pipe_stress_solver_diagnostics::DiagnosticsError::NegativeInput { name, value } => {
            NonlinearSupportError::NegativeInput { name, value }
        }
        open_pipe_stress_solver_diagnostics::DiagnosticsError::InvalidThresholds { .. } => {
            NonlinearSupportError::NegativeInput {
                name: "tolerance",
                value: input.tolerance,
            }
        }
    })? {
        diagnostics.push(with_active_set_context(
            diagnostic,
            &changed_supports,
            &states,
        ));
    }

    Ok(ActiveSetIteration {
        iteration: input.iteration,
        states,
        changed_supports,
        residual_norm,
        converged: residual_norm <= input.tolerance,
        diagnostics,
    })
}

pub fn evaluate_active_set_report(
    input: &ActiveSetIterationInput,
) -> Result<ActiveSetReportRecord, NonlinearSupportError> {
    evaluate_active_set_iteration(input)?.to_report_record(input)
}

fn classify_iteration_support_state(
    support: &NonlinearSupport,
    trial: &TrialSupportState,
    prior_state: Option<ActiveSetState>,
) -> Result<ActiveSetState, NonlinearSupportError> {
    let state = classify_support_state(support, trial)?;
    if matches!(support.behavior, NonlinearSupportBehavior::Friction)
        && prior_state == Some(ActiveSetState::Sliding)
        && state == ActiveSetState::Sticking
        && trial.displacement != 0.0
    {
        return Ok(ActiveSetState::Sliding);
    }
    Ok(state)
}

pub fn classify_support_state(
    support: &NonlinearSupport,
    trial: &TrialSupportState,
) -> Result<ActiveSetState, NonlinearSupportError> {
    match support.behavior {
        NonlinearSupportBehavior::OneWay { active_when } => {
            Ok(state_from_sense(trial.reaction, active_when))
        }
        NonlinearSupportBehavior::Gap { closes_when } => {
            let gap = support
                .gap
                .ok_or_else(|| NonlinearSupportError::MissingGap {
                    support_id: support.support_id.clone(),
                })?;
            validate_nonnegative_finite("gap clearance", gap)?;
            let closed = match closes_when {
                GapDirection::PositiveDisplacement => trial.displacement >= gap,
                GapDirection::NegativeDisplacement => trial.displacement <= -gap,
            };
            Ok(if closed {
                ActiveSetState::Active
            } else {
                ActiveSetState::Inactive
            })
        }
        NonlinearSupportBehavior::LiftOff { contact_when } => {
            Ok(state_from_sense(trial.reaction, contact_when))
        }
        NonlinearSupportBehavior::Friction => {
            let normal = trial.normal_reaction.ok_or_else(|| {
                NonlinearSupportError::MissingFrictionData {
                    support_id: support.support_id.clone(),
                }
            })?;
            let tangential = trial.tangential_reaction.ok_or_else(|| {
                NonlinearSupportError::MissingFrictionData {
                    support_id: support.support_id.clone(),
                }
            })?;
            validate_finite("normal reaction", normal)?;
            validate_finite("tangential reaction", tangential)?;

            if normal <= 0.0 {
                return Ok(ActiveSetState::Inactive);
            }

            let coefficient = support.friction_coefficient.ok_or_else(|| {
                NonlinearSupportError::MissingFrictionCoefficient {
                    support_id: support.support_id.clone(),
                }
            })?;
            validate_nonnegative_finite("friction coefficient", coefficient)?;
            let limit = coefficient * normal.abs();
            if tangential.abs() <= limit {
                Ok(ActiveSetState::Sticking)
            } else {
                Ok(ActiveSetState::Sliding)
            }
        }
    }
}

fn with_active_set_context(
    mut diagnostic: SolverDiagnostic,
    changed_supports: &[String],
    states: &[SupportStateRecord],
) -> SolverDiagnostic {
    if diagnostic.code != SolverDiagnosticCode::NonConvergence {
        return diagnostic;
    }

    let changed = if changed_supports.is_empty() {
        "none".to_string()
    } else {
        changed_supports.join(",")
    };
    let state_summary = if states.is_empty() {
        "none".to_string()
    } else {
        states
            .iter()
            .map(|record| format!("{}={}", record.support_id, record.state.as_str()))
            .collect::<Vec<_>>()
            .join(",")
    };

    diagnostic.message = format!(
        "{}; active-set changed supports: {}; active-set states: {}",
        diagnostic.message, changed, state_summary
    );
    if !changed_supports.is_empty() {
        diagnostic.affected_ref = Some(format!("active-set:{changed}"));
    }
    diagnostic
}

pub fn nonconvergence_code(iteration: &ActiveSetIteration) -> Option<SolverDiagnosticCode> {
    iteration
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == SolverDiagnosticCode::NonConvergence)
        .map(|diagnostic| diagnostic.code)
}

fn state_from_sense(reaction: f64, active_when: ActivationSense) -> ActiveSetState {
    let active = match active_when {
        ActivationSense::PositiveReaction => reaction > 0.0,
        ActivationSense::NegativeReaction => reaction < 0.0,
    };
    if active {
        ActiveSetState::Active
    } else {
        ActiveSetState::Inactive
    }
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), NonlinearSupportError> {
    if !value.is_finite() {
        return Err(NonlinearSupportError::NonFiniteInput { name, value });
    }
    Ok(())
}

fn validate_nonnegative_finite(
    name: &'static str,
    value: f64,
) -> Result<(), NonlinearSupportError> {
    validate_finite(name, value)?;
    if value < 0.0 {
        return Err(NonlinearSupportError::NegativeInput { name, value });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_way_support_activates_from_explicit_reaction_sense() {
        let support = NonlinearSupport::one_way(
            "one-way-1",
            0,
            FrameDof::Uz,
            ActivationSense::PositiveReaction,
        );
        let active_trial = TrialSupportState::new("one-way-1", 0.0, 12.0).unwrap();
        let inactive_trial = TrialSupportState::new("one-way-1", 0.0, -1.0).unwrap();

        assert_eq!(
            classify_support_state(&support, &active_trial).unwrap(),
            ActiveSetState::Active
        );
        assert_eq!(
            classify_support_state(&support, &inactive_trial).unwrap(),
            ActiveSetState::Inactive
        );
    }

    #[test]
    fn gap_support_closes_only_after_explicit_clearance() {
        let support = NonlinearSupport::gap(
            "gap-1",
            0,
            FrameDof::Ux,
            0.25,
            GapDirection::PositiveDisplacement,
        )
        .unwrap();

        let open_trial = TrialSupportState::new("gap-1", 0.20, 0.0).unwrap();
        let closed_trial = TrialSupportState::new("gap-1", 0.25, 0.0).unwrap();

        assert_eq!(
            classify_support_state(&support, &open_trial).unwrap(),
            ActiveSetState::Inactive
        );
        assert_eq!(
            classify_support_state(&support, &closed_trial).unwrap(),
            ActiveSetState::Active
        );
    }

    #[test]
    fn gap_support_without_clearance_is_blocking_model_error() {
        let support = NonlinearSupport {
            support_id: "gap-missing".to_string(),
            node_index: 0,
            dof: FrameDof::Ux,
            behavior: NonlinearSupportBehavior::Gap {
                closes_when: GapDirection::PositiveDisplacement,
            },
            gap: None,
            friction_coefficient: None,
        };
        let trial = TrialSupportState::new("gap-missing", 0.20, 0.0).unwrap();

        let error = classify_support_state(&support, &trial).unwrap_err();

        assert_eq!(
            error,
            NonlinearSupportError::MissingGap {
                support_id: "gap-missing".to_string()
            }
        );
        let diagnostic = diagnostic_from_nonlinear_support_error(&error);
        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidRestraint);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostic.affected_ref.as_deref(), Some("gap-missing"));
    }

    #[test]
    fn lift_off_support_deactivates_when_contact_reaction_is_lost() {
        let support = NonlinearSupport::lift_off(
            "lift-1",
            0,
            FrameDof::Uz,
            ActivationSense::NegativeReaction,
        );
        let contact_trial = TrialSupportState::new("lift-1", 0.0, -3.0).unwrap();
        let lifted_trial = TrialSupportState::new("lift-1", 0.0, 0.0).unwrap();

        assert_eq!(
            classify_support_state(&support, &contact_trial).unwrap(),
            ActiveSetState::Active
        );
        assert_eq!(
            classify_support_state(&support, &lifted_trial).unwrap(),
            ActiveSetState::Inactive
        );
    }

    #[test]
    fn friction_support_classifies_sticking_and_sliding() {
        let support = NonlinearSupport::friction("friction-1", 0, FrameDof::Ux, 0.30).unwrap();
        let sticking_trial = TrialSupportState::new("friction-1", 0.0, 0.0)
            .unwrap()
            .with_friction_reactions(10.0, 2.5)
            .unwrap();
        let sliding_trial = TrialSupportState::new("friction-1", 0.0, 0.0)
            .unwrap()
            .with_friction_reactions(10.0, 3.5)
            .unwrap();

        assert_eq!(
            classify_support_state(&support, &sticking_trial).unwrap(),
            ActiveSetState::Sticking
        );
        assert_eq!(
            classify_support_state(&support, &sliding_trial).unwrap(),
            ActiveSetState::Sliding
        );
    }

    #[test]
    fn friction_support_without_contact_is_inactive() {
        let support = NonlinearSupport::friction("friction-1", 0, FrameDof::Ux, 0.30).unwrap();
        let trial = TrialSupportState::new("friction-1", 0.0, 0.0)
            .unwrap()
            .with_friction_reactions(0.0, 1.0)
            .unwrap();

        assert_eq!(
            classify_support_state(&support, &trial).unwrap(),
            ActiveSetState::Inactive
        );
    }

    #[test]
    fn friction_support_negative_contact_reaction_is_inactive() {
        let support = NonlinearSupport::friction("friction-1", 0, FrameDof::Ux, 0.30).unwrap();
        let trial = TrialSupportState::new("friction-1", 0.0, 0.0)
            .unwrap()
            .with_friction_reactions(-10.0, 99.0)
            .unwrap();

        assert_eq!(
            classify_support_state(&support, &trial).unwrap(),
            ActiveSetState::Inactive
        );
    }

    #[test]
    fn friction_support_without_coefficient_is_blocking_model_error() {
        let support = NonlinearSupport {
            support_id: "friction-missing".to_string(),
            node_index: 0,
            dof: FrameDof::Ux,
            behavior: NonlinearSupportBehavior::Friction,
            gap: None,
            friction_coefficient: None,
        };
        let trial = TrialSupportState::new("friction-missing", 0.0, 0.0)
            .unwrap()
            .with_friction_reactions(10.0, 1.0)
            .unwrap();

        let error = classify_support_state(&support, &trial).unwrap_err();

        assert_eq!(
            error,
            NonlinearSupportError::MissingFrictionCoefficient {
                support_id: "friction-missing".to_string()
            }
        );
        let diagnostic = diagnostic_from_nonlinear_support_error(&error);
        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidRestraint);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostic.affected_ref.as_deref(), Some("friction-missing"));
    }

    #[test]
    fn friction_support_rejects_nonfinite_public_coefficient() {
        let support = NonlinearSupport {
            support_id: "friction-nan".to_string(),
            node_index: 0,
            dof: FrameDof::Ux,
            behavior: NonlinearSupportBehavior::Friction,
            gap: None,
            friction_coefficient: Some(f64::NAN),
        };
        let trial = TrialSupportState::new("friction-nan", 0.0, 0.0)
            .unwrap()
            .with_friction_reactions(10.0, 1.0)
            .unwrap();

        let error = classify_support_state(&support, &trial).unwrap_err();

        match error {
            NonlinearSupportError::NonFiniteInput { name, value } => {
                assert_eq!(name, "friction coefficient");
                assert!(value.is_nan());
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn nonlinear_unit_metadata_uses_explicit_canonical_dimensions() {
        let metadata = NonlinearSupportUnitMetadata::new(
            UnitSystemRef::new("fixture-unit-system").unwrap(),
            QuantityUnitMetadata::new("fixture-displacement", CanonicalDimension::Displacement)
                .unwrap(),
            QuantityUnitMetadata::new("fixture-force", CanonicalDimension::Force).unwrap(),
            QuantityUnitMetadata::new(
                "fixture-friction-coefficient",
                CanonicalDimension::Dimensionless,
            )
            .unwrap(),
        )
        .unwrap();

        assert!(metadata.has_expected_dimensions());
        assert_eq!(metadata.displacement_unit.dimension_id(), "displacement");
        assert_eq!(
            metadata.friction_coefficient_unit.dimension_id(),
            "dimensionless"
        );
    }

    #[test]
    fn nonlinear_missing_data_errors_map_to_solver_diagnostics() {
        let diagnostic =
            diagnostic_from_nonlinear_support_error(&NonlinearSupportError::MissingFrictionData {
                support_id: "friction-1".to_string(),
            });

        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidRestraint);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostic.source, DiagnosticSource::ModelValidation);
        assert_eq!(diagnostic.affected_ref.as_deref(), Some("friction-1"));
        assert!(diagnostic.remediation.is_some());
    }

    #[test]
    fn active_set_iteration_converges_when_states_do_not_change() {
        let support = NonlinearSupport::one_way(
            "one-way-1",
            0,
            FrameDof::Uz,
            ActivationSense::PositiveReaction,
        );
        let input = ActiveSetIterationInput {
            iteration: 2,
            max_iterations: 10,
            tolerance: 0.0,
            supports: vec![support],
            trial_states: vec![TrialSupportState::new("one-way-1", 0.0, 5.0).unwrap()],
            prior_states: vec![SupportStateRecord::new("one-way-1", ActiveSetState::Active)],
        };

        let iteration = evaluate_active_set_iteration(&input).unwrap();

        assert!(iteration.converged);
        assert!(iteration.changed_supports.is_empty());
        assert!(iteration.diagnostics.is_empty());
    }

    #[test]
    fn active_set_iteration_persists_sliding_for_released_friction_displacement() {
        let support = NonlinearSupport::friction("friction-slide", 0, FrameDof::Ux, 0.30).unwrap();
        let trial = TrialSupportState::new("friction-slide", 0.12, 0.0)
            .unwrap()
            .with_friction_reactions(10.0, 0.0)
            .unwrap();
        assert_eq!(
            classify_support_state(&support, &trial).unwrap(),
            ActiveSetState::Sticking
        );

        let input = ActiveSetIterationInput {
            iteration: 2,
            max_iterations: 4,
            tolerance: 0.0,
            supports: vec![support],
            trial_states: vec![trial],
            prior_states: vec![SupportStateRecord::new(
                "friction-slide",
                ActiveSetState::Sliding,
            )],
        };

        let iteration = evaluate_active_set_iteration(&input).unwrap();

        assert!(iteration.converged);
        assert_eq!(
            iteration.states,
            vec![SupportStateRecord::new(
                "friction-slide",
                ActiveSetState::Sliding
            )]
        );
        assert!(iteration.changed_supports.is_empty());
    }

    #[test]
    fn active_set_iteration_reports_nonconvergence_at_iteration_limit() {
        let support = NonlinearSupport::one_way(
            "one-way-1",
            0,
            FrameDof::Uz,
            ActivationSense::PositiveReaction,
        );
        let input = ActiveSetIterationInput {
            iteration: 3,
            max_iterations: 3,
            tolerance: 0.0,
            supports: vec![support],
            trial_states: vec![TrialSupportState::new("one-way-1", 0.0, 5.0).unwrap()],
            prior_states: vec![SupportStateRecord::new(
                "one-way-1",
                ActiveSetState::Inactive,
            )],
        };

        let iteration = evaluate_active_set_iteration(&input).unwrap();

        assert!(!iteration.converged);
        assert!(iteration.is_blocked());
        assert_eq!(
            nonconvergence_code(&iteration),
            Some(SolverDiagnosticCode::NonConvergence)
        );
        assert_eq!(
            iteration.diagnostics[0].affected_ref.as_deref(),
            Some("active-set:one-way-1")
        );
        assert!(iteration.diagnostics[0]
            .message
            .contains("active-set changed supports: one-way-1"));
        assert!(iteration.diagnostics[0]
            .message
            .contains("one-way-1=active"));
    }

    #[test]
    fn active_set_iteration_converts_to_report_record_with_explicit_fields() {
        let support = NonlinearSupport::one_way(
            "one-way-1",
            0,
            FrameDof::Uz,
            ActivationSense::PositiveReaction,
        );
        let input = ActiveSetIterationInput {
            iteration: 1,
            max_iterations: 5,
            tolerance: 0.0,
            supports: vec![support],
            trial_states: vec![TrialSupportState::new("one-way-1", 0.0, 5.0).unwrap()],
            prior_states: vec![SupportStateRecord::new(
                "one-way-1",
                ActiveSetState::Inactive,
            )],
        };

        let iteration = evaluate_active_set_iteration(&input).unwrap();
        let report = iteration.to_report_record(&input).unwrap();
        let support_report = report.support_state("one-way-1").unwrap();

        assert_eq!(report.iteration, 1);
        assert_eq!(report.max_iterations, 5);
        assert_eq!(report.tolerance, 0.0);
        assert_eq!(report.residual_norm, 1.0);
        assert!(!report.converged);
        assert_eq!(report.changed_supports, vec!["one-way-1".to_string()]);
        assert_eq!(support_report.state, ActiveSetState::Active);
        assert_eq!(support_report.state_label(), "active");
        assert!(support_report.changed);
        assert_eq!(
            report.diagnostics[0].code,
            SolverDiagnosticCode::NonConvergence
        );
        assert!(report
            .assumptions
            .iter()
            .any(|assumption| assumption.contains("residual norm")));
        assert!(report
            .limitations
            .iter()
            .any(|limitation| limitation.contains("does not assemble")));
    }

    #[test]
    fn active_set_report_preserves_nonconvergence_as_structured_diagnostic() {
        let support = NonlinearSupport::one_way(
            "one-way-1",
            0,
            FrameDof::Uz,
            ActivationSense::PositiveReaction,
        );
        let input = ActiveSetIterationInput {
            iteration: 3,
            max_iterations: 3,
            tolerance: 0.0,
            supports: vec![support],
            trial_states: vec![TrialSupportState::new("one-way-1", 0.0, 5.0).unwrap()],
            prior_states: vec![SupportStateRecord::new(
                "one-way-1",
                ActiveSetState::Inactive,
            )],
        };

        let report = evaluate_active_set_report(&input).unwrap();
        let diagnostic = &report.diagnostics[0];
        let support_report = report.support_state("one-way-1").unwrap();

        assert_eq!(report.iteration, 3);
        assert_eq!(report.max_iterations, 3);
        assert_eq!(report.residual_norm, 1.0);
        assert!(!report.converged);
        assert_eq!(support_report.state, ActiveSetState::Active);
        assert!(support_report.changed);
        assert_eq!(diagnostic.code, SolverDiagnosticCode::NonConvergence);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Failure);
        assert_eq!(
            diagnostic.affected_ref.as_deref(),
            Some("active-set:one-way-1")
        );
        assert!(report
            .limitations
            .iter()
            .any(|limitation| limitation.contains("rule compliance")));
    }

    #[test]
    fn invalid_numeric_inputs_are_rejected() {
        let error = NonlinearSupport::gap(
            "gap-1",
            0,
            FrameDof::Ux,
            f64::NAN,
            GapDirection::PositiveDisplacement,
        )
        .unwrap_err();

        match error {
            NonlinearSupportError::NonFiniteInput { name, value } => {
                assert_eq!(name, "gap clearance");
                assert!(value.is_nan());
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}

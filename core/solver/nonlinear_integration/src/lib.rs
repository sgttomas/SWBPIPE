//! Assembled nonlinear active-set integration loop.
//!
//! This crate is the `DEC-044` / D6 integration owner between the linear frame
//! kernel and the nonlinear-support active-set classifier. It computes open
//! mechanics quantities only. It does not encode design-code checks, protected
//! standards content, public support defaults, catalog values, or professional
//! approval claims.

use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness, node_dof_index, reduce_system_with_prescribed_displacements,
    solve_dense, DenseMatrix, DenseVector, FrameElement, FrameKernelError, DOF_PER_NODE,
};
use open_pipe_stress_nonlinear_supports::{
    evaluate_active_set_iteration, ActiveSetIteration, ActiveSetIterationInput, ActiveSetState,
    GapDirection, NonlinearSupport, NonlinearSupportBehavior, NonlinearSupportError,
    SupportStateRecord, TrialSupportState,
};
use open_pipe_stress_solver_diagnostics::{tolerance_policy_tbd_diagnostic, SolverDiagnostic};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvergencePolicyStatus {
    Accepted,
    Tbd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConvergenceControl {
    pub policy_ref: String,
    pub policy_status: ConvergencePolicyStatus,
    pub residual_tolerance: f64,
    pub absolute_residual_floor: f64,
    pub max_iterations: usize,
}

impl ConvergenceControl {
    pub fn new(
        policy_ref: impl Into<String>,
        policy_status: ConvergencePolicyStatus,
        residual_tolerance: f64,
        absolute_residual_floor: f64,
        max_iterations: usize,
    ) -> Result<Self, NonlinearIntegrationError> {
        validate_nonnegative_finite("residual_tolerance", residual_tolerance)?;
        validate_nonnegative_finite("absolute_residual_floor", absolute_residual_floor)?;
        if max_iterations == 0 {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: "max_iterations must be positive".to_string(),
            });
        }
        let policy_ref = normalize_required_id(policy_ref, "policy_ref")?;
        Ok(Self {
            policy_ref,
            policy_status,
            residual_tolerance,
            absolute_residual_floor,
            max_iterations,
        })
    }

    pub fn effective_tolerance(&self) -> f64 {
        self.residual_tolerance.max(self.absolute_residual_floor)
    }

    pub fn emits_tbd_diagnostic(&self) -> bool {
        self.policy_status == ConvergencePolicyStatus::Tbd
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrictionNormalReaction {
    pub support_id: String,
    pub normal_reaction: f64,
}

impl FrictionNormalReaction {
    pub fn new(
        support_id: impl Into<String>,
        normal_reaction: f64,
    ) -> Result<Self, NonlinearIntegrationError> {
        validate_finite("normal_reaction", normal_reaction)?;
        Ok(Self {
            support_id: normalize_required_id(support_id, "support_id")?,
            normal_reaction,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearFrameSolveInput {
    pub node_count: usize,
    pub elements: Vec<FrameElement>,
    pub force: DenseVector,
    pub base_restrained_dofs: Vec<usize>,
    pub nonlinear_supports: Vec<NonlinearSupport>,
    pub initial_states: Vec<SupportStateRecord>,
    pub friction_normal_reactions: Vec<FrictionNormalReaction>,
    pub convergence: ConvergenceControl,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearFrameIteration {
    pub iteration: usize,
    pub active_restrained_dofs: Vec<usize>,
    pub active_prescribed_displacements: DenseVector,
    pub displacements: DenseVector,
    pub reactions: DenseVector,
    pub active_set: ActiveSetIteration,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearFrameSolveResult {
    pub converged: bool,
    pub policy_ref: String,
    pub final_states: Vec<SupportStateRecord>,
    pub displacements: DenseVector,
    pub reactions: DenseVector,
    pub iterations: Vec<NonlinearFrameIteration>,
    pub diagnostics: Vec<SolverDiagnostic>,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
}

impl NonlinearFrameSolveResult {
    pub fn is_blocked(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.is_blocking())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NonlinearIntegrationError {
    FrameKernel(FrameKernelError),
    NonlinearSupport(NonlinearSupportError),
    InvalidInput { detail: String },
}

impl fmt::Display for NonlinearIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FrameKernel(error) => write!(f, "frame-kernel integration failed: {error}"),
            Self::NonlinearSupport(error) => {
                write!(f, "nonlinear support classification failed: {error}")
            }
            Self::InvalidInput { detail } => {
                write!(f, "invalid nonlinear integration input: {detail}")
            }
        }
    }
}

impl Error for NonlinearIntegrationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FrameKernel(error) => Some(error),
            Self::NonlinearSupport(error) => Some(error),
            Self::InvalidInput { .. } => None,
        }
    }
}

impl From<FrameKernelError> for NonlinearIntegrationError {
    fn from(error: FrameKernelError) -> Self {
        Self::FrameKernel(error)
    }
}

impl From<NonlinearSupportError> for NonlinearIntegrationError {
    fn from(error: NonlinearSupportError) -> Self {
        Self::NonlinearSupport(error)
    }
}

pub fn solve_active_set_frame(
    input: &NonlinearFrameSolveInput,
) -> Result<NonlinearFrameSolveResult, NonlinearIntegrationError> {
    validate_input(input)?;

    let stiffness = assemble_global_stiffness(input.node_count, &input.elements)?;
    let mut current_states = normalized_initial_states(input)?;
    let mut iterations = Vec::new();
    let mut final_diagnostics = policy_diagnostics(&input.convergence);
    let friction_normals = friction_normal_map(input)?;

    for iteration_index in 1..=input.convergence.max_iterations {
        let boundary = active_boundary(
            input.node_count,
            &input.base_restrained_dofs,
            &input.nonlinear_supports,
            &current_states,
        )?;
        let linearized = solve_linearized_system(&stiffness, &input.force, &boundary)?;
        let trial_states = build_trial_states(
            &input.nonlinear_supports,
            &linearized.displacements,
            &linearized.reactions,
            &friction_normals,
        )?;
        let active_set_input = ActiveSetIterationInput {
            iteration: iteration_index,
            max_iterations: input.convergence.max_iterations,
            tolerance: input.convergence.effective_tolerance(),
            supports: input.nonlinear_supports.clone(),
            trial_states,
            prior_states: current_states.clone(),
        };
        let active_set = evaluate_active_set_iteration(&active_set_input)?;
        let blocked = active_set.is_blocked();
        let converged = active_set.converged && !blocked;
        current_states = active_set.states.clone();

        let iteration = NonlinearFrameIteration {
            iteration: iteration_index,
            active_restrained_dofs: boundary.dofs.clone(),
            active_prescribed_displacements: boundary.displacements.clone(),
            displacements: linearized.displacements,
            reactions: linearized.reactions,
            active_set,
        };
        iterations.push(iteration);

        if converged || blocked {
            let final_iteration = iterations
                .last()
                .expect("just pushed nonlinear iteration record");
            final_diagnostics.extend(final_iteration.active_set.diagnostics.clone());
            return Ok(NonlinearFrameSolveResult {
                converged,
                policy_ref: input.convergence.policy_ref.clone(),
                final_states: current_states,
                displacements: final_iteration.displacements.clone(),
                reactions: final_iteration.reactions.clone(),
                iterations,
                diagnostics: final_diagnostics,
                assumptions: assembled_loop_assumptions(),
                limitations: assembled_loop_limitations(),
            });
        }
    }

    unreachable!("positive max_iterations loop must return on the capped final iteration")
}

pub fn assembled_loop_assumptions() -> Vec<String> {
    vec![
        "Active nonlinear support states are represented as prescribed frame DOFs in the current linearized iteration.".to_string(),
        "The first assembled-loop residual is the nonlinear-support classifier state-change count; force/displacement residual norms remain future D6/D9 evidence work.".to_string(),
        "Friction support normal reaction facts must be supplied explicitly until a governed normal/tangential support model is integrated.".to_string(),
    ]
}

pub fn assembled_loop_limitations() -> Vec<String> {
    vec![
        "This dense integration slice does not bind the live sparse solver path; D-17 remains the sparse live-path timing gate.".to_string(),
        "DEC-046 class-tiered convergence values remain TBD until assembled-loop evidence seeds them; callers must supply explicit controls and policy references.".to_string(),
        "The result is mechanics evidence only and does not state rule compliance, professional approval, certification, sealing, authentication, or code compliance.".to_string(),
    ]
}

#[derive(Debug, Clone, PartialEq)]
struct BoundaryState {
    dofs: Vec<usize>,
    displacements: DenseVector,
}

#[derive(Debug, Clone, PartialEq)]
struct LinearizedSolve {
    displacements: DenseVector,
    reactions: DenseVector,
}

fn validate_input(input: &NonlinearFrameSolveInput) -> Result<(), NonlinearIntegrationError> {
    if input.node_count == 0 {
        return Err(NonlinearIntegrationError::InvalidInput {
            detail: "node_count must be positive".to_string(),
        });
    }
    let expected_dofs = input.node_count * DOF_PER_NODE;
    if input.force.len() != expected_dofs {
        return Err(NonlinearIntegrationError::InvalidInput {
            detail: format!(
                "force vector length must be {expected_dofs} for {} node(s), got {}",
                input.node_count,
                input.force.len()
            ),
        });
    }
    for &value in &input.force {
        validate_finite("force", value)?;
    }
    for &dof in &input.base_restrained_dofs {
        if dof >= expected_dofs {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "base restrained DOF {dof} is outside total DOF count {expected_dofs}"
                ),
            });
        }
    }
    for support in &input.nonlinear_supports {
        if support.node_index >= input.node_count {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "nonlinear support {} node index {} is outside node count {}",
                    support.support_id, support.node_index, input.node_count
                ),
            });
        }
    }
    Ok(())
}

fn normalized_initial_states(
    input: &NonlinearFrameSolveInput,
) -> Result<Vec<SupportStateRecord>, NonlinearIntegrationError> {
    let support_ids: HashSet<&str> = input
        .nonlinear_supports
        .iter()
        .map(|support| support.support_id.as_str())
        .collect();
    let mut seen = HashSet::new();
    for state in &input.initial_states {
        if !support_ids.contains(state.support_id.as_str()) {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "initial state references unknown nonlinear support {}",
                    state.support_id
                ),
            });
        }
        if !seen.insert(state.support_id.as_str()) {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "initial state for nonlinear support {} is repeated",
                    state.support_id
                ),
            });
        }
    }
    for support in &input.nonlinear_supports {
        if !seen.contains(support.support_id.as_str()) {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "initial active-set state is required for nonlinear support {}",
                    support.support_id
                ),
            });
        }
    }

    let mut states = input.initial_states.clone();
    states.sort_by(|left, right| left.support_id.cmp(&right.support_id));
    Ok(states)
}

fn friction_normal_map(
    input: &NonlinearFrameSolveInput,
) -> Result<HashMap<&str, f64>, NonlinearIntegrationError> {
    let mut normals = HashMap::new();
    for reaction in &input.friction_normal_reactions {
        if normals
            .insert(reaction.support_id.as_str(), reaction.normal_reaction)
            .is_some()
        {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "friction normal reaction for support {} is repeated",
                    reaction.support_id
                ),
            });
        }
    }
    Ok(normals)
}

fn active_boundary(
    node_count: usize,
    base_restrained_dofs: &[usize],
    supports: &[NonlinearSupport],
    states: &[SupportStateRecord],
) -> Result<BoundaryState, NonlinearIntegrationError> {
    let mut boundary_pairs = base_restrained_dofs
        .iter()
        .copied()
        .map(|dof| (dof, 0.0))
        .collect::<Vec<_>>();
    let state_map = states
        .iter()
        .map(|state| (state.support_id.as_str(), state.state))
        .collect::<HashMap<_, _>>();

    for support in supports {
        let state = state_map.get(support.support_id.as_str()).ok_or_else(|| {
            NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "missing active-set state for nonlinear support {}",
                    support.support_id
                ),
            }
        })?;
        if let Some(displacement) = prescribed_displacement_for_state(support, *state)? {
            let global_dof = node_dof_index(support.node_index, support.dof);
            if global_dof >= node_count * DOF_PER_NODE {
                return Err(NonlinearIntegrationError::InvalidInput {
                    detail: format!(
                        "active nonlinear support {} DOF {global_dof} is outside total DOF count {}",
                        support.support_id,
                        node_count * DOF_PER_NODE
                    ),
                });
            }
            boundary_pairs.push((global_dof, displacement));
        }
    }

    boundary_pairs.sort_by_key(|(dof, _)| *dof);
    for pair in boundary_pairs.windows(2) {
        if pair[0].0 == pair[1].0 {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!("active boundary DOF {} is repeated", pair[0].0),
            });
        }
    }

    let (dofs, displacements): (Vec<_>, Vec<_>) = boundary_pairs.into_iter().unzip();
    Ok(BoundaryState {
        dofs,
        displacements,
    })
}

fn prescribed_displacement_for_state(
    support: &NonlinearSupport,
    state: ActiveSetState,
) -> Result<Option<f64>, NonlinearIntegrationError> {
    match state {
        ActiveSetState::Inactive | ActiveSetState::Sliding => Ok(None),
        ActiveSetState::Sticking => Ok(Some(0.0)),
        ActiveSetState::Active => match support.behavior {
            NonlinearSupportBehavior::Gap { closes_when } => {
                let gap = support
                    .gap
                    .ok_or_else(|| NonlinearSupportError::MissingGap {
                        support_id: support.support_id.clone(),
                    })?;
                validate_nonnegative_finite("gap", gap)?;
                Ok(Some(match closes_when {
                    GapDirection::PositiveDisplacement => gap,
                    GapDirection::NegativeDisplacement => -gap,
                }))
            }
            NonlinearSupportBehavior::OneWay { .. }
            | NonlinearSupportBehavior::LiftOff { .. }
            | NonlinearSupportBehavior::Friction => Ok(Some(0.0)),
        },
    }
}

fn solve_linearized_system(
    stiffness: &DenseMatrix,
    force: &DenseVector,
    boundary: &BoundaryState,
) -> Result<LinearizedSolve, NonlinearIntegrationError> {
    let reduced = reduce_system_with_prescribed_displacements(
        stiffness,
        force,
        &boundary.dofs,
        &boundary.displacements,
    )?;
    let reduced_solution = solve_dense(&reduced.stiffness, &reduced.force)?;

    let mut displacements = vec![0.0; force.len()];
    for (&dof, &value) in boundary.dofs.iter().zip(boundary.displacements.iter()) {
        displacements[dof] = value;
    }
    for (&global_dof, &value) in reduced.free_dofs.iter().zip(reduced_solution.iter()) {
        displacements[global_dof] = value;
    }

    let reactions = multiply_matrix_vector(stiffness, &displacements)?
        .into_iter()
        .zip(force.iter())
        .map(|(internal, applied)| internal - applied)
        .collect();

    Ok(LinearizedSolve {
        displacements,
        reactions,
    })
}

fn build_trial_states(
    supports: &[NonlinearSupport],
    displacements: &[f64],
    reactions: &[f64],
    friction_normals: &HashMap<&str, f64>,
) -> Result<Vec<TrialSupportState>, NonlinearIntegrationError> {
    let mut trial_states = Vec::with_capacity(supports.len());
    for support in supports {
        let global_dof = node_dof_index(support.node_index, support.dof);
        let mut trial = TrialSupportState::new(
            support.support_id.clone(),
            displacements[global_dof],
            reactions[global_dof],
        )?;
        if matches!(support.behavior, NonlinearSupportBehavior::Friction) {
            let normal = friction_normals
                .get(support.support_id.as_str())
                .copied()
                .ok_or_else(|| NonlinearSupportError::MissingFrictionData {
                    support_id: support.support_id.clone(),
                })?;
            trial = trial.with_friction_reactions(normal, reactions[global_dof])?;
        }
        trial_states.push(trial);
    }
    Ok(trial_states)
}

fn multiply_matrix_vector(
    matrix: &DenseMatrix,
    vector: &[f64],
) -> Result<DenseVector, NonlinearIntegrationError> {
    if matrix.len() != vector.len() {
        return Err(NonlinearIntegrationError::InvalidInput {
            detail: format!(
                "matrix/vector multiply requires {} vector entries, got {}",
                matrix.len(),
                vector.len()
            ),
        });
    }
    let mut result = Vec::with_capacity(matrix.len());
    for row in matrix {
        if row.len() != vector.len() {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: "matrix/vector multiply requires a square matrix".to_string(),
            });
        }
        result.push(row.iter().zip(vector.iter()).map(|(a, b)| a * b).sum());
    }
    Ok(result)
}

fn policy_diagnostics(convergence: &ConvergenceControl) -> Vec<SolverDiagnostic> {
    if convergence.emits_tbd_diagnostic() {
        vec![tolerance_policy_tbd_diagnostic().with_affected_ref(convergence.policy_ref.clone())]
    } else {
        Vec::new()
    }
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), NonlinearIntegrationError> {
    if !value.is_finite() {
        return Err(NonlinearIntegrationError::InvalidInput {
            detail: format!("{name} must be finite, got {value}"),
        });
    }
    Ok(())
}

fn validate_nonnegative_finite(
    name: &'static str,
    value: f64,
) -> Result<(), NonlinearIntegrationError> {
    validate_finite(name, value)?;
    if value < 0.0 {
        return Err(NonlinearIntegrationError::InvalidInput {
            detail: format!("{name} must be nonnegative, got {value}"),
        });
    }
    Ok(())
}

fn normalize_required_id(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, NonlinearIntegrationError> {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("TBD") {
        return Err(NonlinearIntegrationError::InvalidInput {
            detail: format!("{field} must be explicit and not TBD"),
        });
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_frame_kernel::{
        FrameDof, FrameElement, FrameNode, FrameSection, RX, RY, RZ, UX, UY, UZ,
    };
    use open_pipe_stress_nonlinear_supports::{ActivationSense, GapDirection};

    fn two_node_axial_problem(
        nonlinear_supports: Vec<NonlinearSupport>,
        initial_states: Vec<SupportStateRecord>,
        max_iterations: usize,
    ) -> NonlinearFrameSolveInput {
        let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap();
        let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
        let element = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap();
        let mut force = vec![0.0; 2 * DOF_PER_NODE];
        force[node_dof_index(1, FrameDof::Ux)] = 10.0;

        NonlinearFrameSolveInput {
            node_count: 2,
            elements: vec![element],
            force,
            base_restrained_dofs: vec![
                node_dof_index(0, FrameDof::Ux),
                node_dof_index(0, FrameDof::Uy),
                node_dof_index(0, FrameDof::Uz),
                node_dof_index(0, FrameDof::Rx),
                node_dof_index(0, FrameDof::Ry),
                node_dof_index(0, FrameDof::Rz),
                node_dof_index(1, FrameDof::Uy),
                node_dof_index(1, FrameDof::Uz),
                node_dof_index(1, FrameDof::Rx),
                node_dof_index(1, FrameDof::Ry),
                node_dof_index(1, FrameDof::Rz),
            ],
            nonlinear_supports,
            initial_states,
            friction_normal_reactions: Vec::new(),
            convergence: ConvergenceControl::new(
                "DEC-046-fixture-active-set-count-tightening",
                ConvergencePolicyStatus::Accepted,
                0.0,
                0.0,
                max_iterations,
            )
            .unwrap(),
        }
    }

    #[test]
    fn one_way_support_deactivates_then_converges_through_frame_loop() {
        let support_id = "NL-ONE-WAY-DEACTIVATE";
        let support = NonlinearSupport::one_way(
            support_id,
            1,
            FrameDof::Ux,
            ActivationSense::PositiveReaction,
        );
        let input = two_node_axial_problem(
            vec![support],
            vec![SupportStateRecord::new(support_id, ActiveSetState::Active)],
            4,
        );

        let result = solve_active_set_frame(&input).unwrap();

        assert!(result.converged);
        assert!(!result.is_blocked());
        assert_eq!(result.iterations.len(), 2);
        assert_eq!(
            result.final_states,
            vec![SupportStateRecord::new(
                support_id,
                ActiveSetState::Inactive
            )]
        );
        assert!(result.displacements[node_dof_index(1, FrameDof::Ux)] > 0.0);
        assert!(!result
            .iterations
            .last()
            .unwrap()
            .active_restrained_dofs
            .contains(&node_dof_index(1, FrameDof::Ux)));
    }

    #[test]
    fn one_way_support_remains_active_when_reaction_matches_sense() {
        let support_id = "NL-ONE-WAY-ACTIVE";
        let support = NonlinearSupport::one_way(
            support_id,
            1,
            FrameDof::Ux,
            ActivationSense::NegativeReaction,
        );
        let input = two_node_axial_problem(
            vec![support],
            vec![SupportStateRecord::new(support_id, ActiveSetState::Active)],
            4,
        );

        let result = solve_active_set_frame(&input).unwrap();

        assert!(result.converged);
        assert_eq!(result.iterations.len(), 1);
        assert_eq!(
            result.final_states,
            vec![SupportStateRecord::new(support_id, ActiveSetState::Active)]
        );
        assert_eq!(result.displacements[node_dof_index(1, FrameDof::Ux)], 0.0);
        assert_eq!(result.reactions[node_dof_index(1, FrameDof::Ux)], -10.0);
    }

    #[test]
    fn gap_support_prescribes_clearance_then_converges() {
        let support_id = "NL-GAP-CLOSE";
        let support = NonlinearSupport::gap(
            support_id,
            1,
            FrameDof::Ux,
            0.05,
            GapDirection::PositiveDisplacement,
        )
        .unwrap();
        let input = two_node_axial_problem(
            vec![support],
            vec![SupportStateRecord::new(
                support_id,
                ActiveSetState::Inactive,
            )],
            4,
        );

        let result = solve_active_set_frame(&input).unwrap();

        assert!(result.converged);
        assert_eq!(result.iterations.len(), 2);
        assert_eq!(
            result.final_states,
            vec![SupportStateRecord::new(support_id, ActiveSetState::Active)]
        );
        assert_eq!(result.displacements[node_dof_index(1, FrameDof::Ux)], 0.05);
        assert_eq!(
            result
                .iterations
                .last()
                .unwrap()
                .active_prescribed_displacements,
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.05, 0.0, 0.0, 0.0, 0.0, 0.0]
        );
    }

    #[test]
    fn iteration_cap_returns_nonconvergence_failure_diagnostic() {
        let support_id = "NL-GAP-CAPPED";
        let support = NonlinearSupport::gap(
            support_id,
            1,
            FrameDof::Ux,
            0.05,
            GapDirection::PositiveDisplacement,
        )
        .unwrap();
        let input = two_node_axial_problem(
            vec![support],
            vec![SupportStateRecord::new(
                support_id,
                ActiveSetState::Inactive,
            )],
            1,
        );

        let result = solve_active_set_frame(&input).unwrap();

        assert!(!result.converged);
        assert!(result.is_blocked());
        assert_eq!(result.iterations.len(), 1);
        assert_eq!(
            result.diagnostics[0].code,
            open_pipe_stress_solver_diagnostics::SolverDiagnosticCode::NonConvergence
        );
    }

    #[test]
    fn tbd_policy_emits_visible_tolerance_policy_diagnostic_without_defaults() {
        let support_id = "NL-ONE-WAY-TBD-POLICY";
        let support = NonlinearSupport::one_way(
            support_id,
            1,
            FrameDof::Ux,
            ActivationSense::NegativeReaction,
        );
        let mut input = two_node_axial_problem(
            vec![support],
            vec![SupportStateRecord::new(support_id, ActiveSetState::Active)],
            4,
        );
        input.convergence = ConvergenceControl::new(
            "DEC-046-class-tier-gap-entry-TBD",
            ConvergencePolicyStatus::Tbd,
            0.0,
            0.0,
            4,
        )
        .unwrap();

        let result = solve_active_set_frame(&input).unwrap();

        assert!(result.converged);
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(
            result.diagnostics[0].code,
            open_pipe_stress_solver_diagnostics::SolverDiagnosticCode::TolerancePolicyTbd
        );
        assert_eq!(
            result.diagnostics[0].affected_ref.as_deref(),
            Some("DEC-046-class-tier-gap-entry-TBD")
        );
    }

    #[test]
    fn initial_state_is_required_for_every_nonlinear_support() {
        let support_id = "NL-STATE-REQUIRED";
        let support = NonlinearSupport::one_way(
            support_id,
            1,
            FrameDof::Ux,
            ActivationSense::NegativeReaction,
        );
        let input = two_node_axial_problem(vec![support], Vec::new(), 4);

        let error = solve_active_set_frame(&input).unwrap_err();

        assert!(error
            .to_string()
            .contains("initial active-set state is required"));
    }

    #[test]
    fn base_fixture_dofs_match_six_dof_order() {
        assert_eq!(UX, 0);
        assert_eq!(UY, 1);
        assert_eq!(UZ, 2);
        assert_eq!(RX, 3);
        assert_eq!(RY, 4);
        assert_eq!(RZ, 5);
    }
}

//! Assembled nonlinear active-set integration loop.
//!
//! This crate is the `DEC-044` / D6 integration owner between the linear frame
//! kernel and the nonlinear-support active-set classifier. It computes open
//! mechanics quantities only. It does not encode design-code checks, protected
//! standards content, public support defaults, catalog values, or professional
//! approval claims.

use open_pipe_stress_curved_bend::CurvedBendMacroElement;
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness_with_user_elements, node_dof_index,
    reduce_system_with_prescribed_displacements, solve_dense, DenseMatrix, DenseVector, FrameDof,
    FrameElement, FrameKernelError, Matrix12, UserStiffnessElement, DOF_PER_NODE, ELEMENT_DOF,
};
use open_pipe_stress_nonlinear_supports::{
    evaluate_active_set_iteration, ActiveSetIteration, ActiveSetIterationInput, ActiveSetState,
    GapDirection, NonlinearSupport, NonlinearSupportBehavior, NonlinearSupportError,
    SupportStateRecord, TrialSupportState,
};
use open_pipe_stress_solver_diagnostics::{
    tolerance_policy_tbd_diagnostic, DiagnosticSeverity, DiagnosticSource, SolverDiagnostic,
    SolverDiagnosticCode,
};
use open_pipe_stress_sparse_direct::{solve_symmetric_system_from_entries, SymmetricMatrixEntry};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

const DEC050_DIRECT_REDUCED_PROFILE_ASSEMBLY_BASIS: &str = "direct_reduced_profile_entries";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinearSolveMode {
    SparseInteractive,
    DenseScrutiny,
}

impl LinearSolveMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SparseInteractive => "sparse_interactive",
            Self::DenseScrutiny => "dense_scrutiny",
        }
    }

    fn solution_basis(self, fallback_to_dense: bool) -> &'static str {
        match (self, fallback_to_dense) {
            (Self::SparseInteractive, false) => "sparse_profile_direct_primary",
            (Self::SparseInteractive, true) => "dense_fallback_after_sparse_failure",
            (Self::DenseScrutiny, _) => "dense_scrutiny_primary",
        }
    }
}

impl Default for LinearSolveMode {
    fn default() -> Self {
        Self::SparseInteractive
    }
}

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
pub struct DerivedFrictionNormalReaction {
    pub support_id: String,
    pub source_node_index: usize,
    pub source_dof: FrameDof,
    pub source_ref: String,
}

impl DerivedFrictionNormalReaction {
    pub fn from_support_reaction(
        support_id: impl Into<String>,
        source_node_index: usize,
        source_dof: FrameDof,
        source_ref: impl Into<String>,
    ) -> Result<Self, NonlinearIntegrationError> {
        Ok(Self {
            support_id: normalize_required_id(support_id, "support_id")?,
            source_node_index,
            source_dof,
            source_ref: normalize_required_id(source_ref, "source_ref")?,
        })
    }
}

/// Explicit curved-bend macro-element stiffness slot for the nonlinear loop
/// (DEC-070 residual closure). The caller supplies the validated 12x12 global
/// arc stiffness — either formed once at model build time or through
/// [`CurvedBendStiffnessElement::from_macro_element`] — so every linearized
/// active-set iteration assembles the identical arc stiffness beside the frame
/// and user-stiffness elements. No straight-chord fallback is derived here.
#[derive(Debug, Clone, PartialEq)]
pub struct CurvedBendStiffnessElement {
    pub element_id: String,
    pub node_i: usize,
    pub node_j: usize,
    pub global_stiffness: Matrix12,
}

impl CurvedBendStiffnessElement {
    pub fn new(
        element_id: impl Into<String>,
        node_i: usize,
        node_j: usize,
        global_stiffness: Matrix12,
    ) -> Result<Self, NonlinearIntegrationError> {
        let element_id = normalize_required_id(element_id, "curved-bend element_id")?;
        if node_i == node_j {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "curved-bend macro-element {element_id} repeats node index {node_i}"
                ),
            });
        }
        for row in &global_stiffness {
            for &value in row {
                if !value.is_finite() {
                    return Err(NonlinearIntegrationError::InvalidInput {
                        detail: format!(
                            "curved-bend macro-element {element_id} global stiffness entry must be finite, got {value}"
                        ),
                    });
                }
            }
        }
        Ok(Self {
            element_id,
            node_i,
            node_j,
            global_stiffness,
        })
    }

    /// Form the slot directly from the `open_pipe_stress_curved_bend`
    /// macro-element (the DEC-070 stiffness source).
    pub fn from_macro_element(
        element_id: impl Into<String>,
        element: &CurvedBendMacroElement,
    ) -> Result<Self, NonlinearIntegrationError> {
        let element_id = normalize_required_id(element_id, "curved-bend element_id")?;
        let global_stiffness = element.global_stiffness().map_err(|error| {
            NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "curved-bend macro-element {element_id} stiffness formation failed: {error}"
                ),
            }
        })?;
        Self::new(
            element_id,
            element.node_i.index,
            element.node_j.index,
            global_stiffness,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearFrameSolveInput {
    pub node_count: usize,
    pub elements: Vec<FrameElement>,
    pub user_stiffness_elements: Vec<UserStiffnessElement>,
    pub curved_bend_elements: Vec<CurvedBendStiffnessElement>,
    pub force: DenseVector,
    pub base_restrained_dofs: Vec<usize>,
    pub nonlinear_supports: Vec<NonlinearSupport>,
    pub initial_states: Vec<SupportStateRecord>,
    pub friction_normal_reactions: Vec<FrictionNormalReaction>,
    pub derived_friction_normal_reactions: Vec<DerivedFrictionNormalReaction>,
    pub convergence: ConvergenceControl,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearFrameIteration {
    pub iteration: usize,
    pub active_restrained_dofs: Vec<usize>,
    pub active_prescribed_displacements: DenseVector,
    pub displacements: DenseVector,
    pub reactions: DenseVector,
    pub applied_sliding_friction_forces: Vec<AppliedSlidingFrictionForce>,
    pub residuals: NonlinearResidualObservation,
    pub sparse_evidence: SparseLinearSolveEvidence,
    pub active_set: ActiveSetIteration,
}

/// Bounded Coulomb sliding force applied at a sliding friction support DOF in
/// one linearized iteration (DEC-067). The magnitude is the explicit friction
/// coefficient times the current-iterate normal-reaction evidence, and the
/// sign opposes the observed sliding motion. This is not a load-step or
/// path-history friction model.
#[derive(Debug, Clone, PartialEq)]
pub struct AppliedSlidingFrictionForce {
    pub support_id: String,
    pub global_dof: usize,
    pub force: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearResidualObservation {
    pub active_set_changed_support_count: f64,
    pub max_abs_translation_delta_from_previous: Option<f64>,
    pub max_abs_rotation_delta_from_previous: Option<f64>,
    pub max_abs_force_reaction_delta_from_previous: Option<f64>,
    pub max_abs_moment_reaction_delta_from_previous: Option<f64>,
    pub max_abs_free_dof_force_residual: f64,
    pub max_abs_free_dof_moment_residual: f64,
    pub max_abs_free_dof_work_residual: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SparseLinearSolveEvidence {
    pub status: SparseEvidenceStatus,
    pub policy_ref: String,
    pub solver_mode: LinearSolveMode,
    pub solution_basis: String,
    pub assembly_basis: String,
    pub reduced_dof_count: usize,
    pub original_max_half_bandwidth: Option<usize>,
    pub ordered_max_half_bandwidth: Option<usize>,
    pub original_profile_entry_count: Option<usize>,
    pub ordered_profile_entry_count: Option<usize>,
    pub nonpositive_pivot_count: Option<usize>,
    pub pivot_condition_ratio_estimate: Option<f64>,
    pub max_abs_dense_sparse_solution_delta: Option<f64>,
    pub relative_dense_sparse_solution_delta: Option<f64>,
    pub max_abs_sparse_residual: Option<f64>,
    pub failure_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SparseEvidenceStatus {
    Observed,
    DenseFallbackAfterSparseFailure,
    Unavailable,
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
    solve_active_set_frame_with_mode(input, LinearSolveMode::default())
}

pub fn solve_active_set_frame_with_mode(
    input: &NonlinearFrameSolveInput,
    linear_solve_mode: LinearSolveMode,
) -> Result<NonlinearFrameSolveResult, NonlinearIntegrationError> {
    solve_active_set_frame_with_mode_and_springs(input, linear_solve_mode, &[])
}

/// Compatibility-preserving adapter for validated linear ground springs.
/// Entries are (global DOF, stiffness in canonical force/displacement units).
pub fn solve_active_set_frame_with_mode_and_springs(
    input: &NonlinearFrameSolveInput,
    linear_solve_mode: LinearSolveMode,
    springs: &[(usize, f64)],
) -> Result<NonlinearFrameSolveResult, NonlinearIntegrationError> {
    validate_input(input)?;

    let mut stiffness = assemble_global_stiffness_with_user_elements(
        input.node_count,
        &input.elements,
        &input.user_stiffness_elements,
    )?;
    add_curved_bend_stiffness_contributions(&mut stiffness, &input.curved_bend_elements);
    for &(dof, value) in springs {
        validate_nonnegative_finite("linear spring stiffness", value)?;
        if dof >= stiffness.len() {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: "linear spring DOF is outside the assembled system".to_string(),
            });
        }
        stiffness[dof][dof] += value;
    }
    for value in stiffness.iter().flatten() {
        validate_finite("assembled stiffness", *value)?;
    }

    let mut current_states = normalized_initial_states(input)?;
    let mut iterations = Vec::new();
    let mut final_diagnostics = policy_diagnostics(&input.convergence);
    let friction_normals = friction_normal_map(input)?;
    let derived_friction_normals = derived_friction_normal_map(input)?;

    for iteration_index in 1..=input.convergence.max_iterations {
        let boundary = active_boundary(
            input.node_count,
            &input.base_restrained_dofs,
            &input.nonlinear_supports,
            &current_states,
        )?;
        // A friction support seeded sliding has no solved iterate yet to
        // orient or scale its bounded force. Defer convergence past this
        // first linearized iterate so the bounded +/- mu*N force is applied
        // before the loop can converge and the converged result does not
        // depend on whether sliding was seeded or reached by transition.
        let sliding_force_deferred = iterations.is_empty()
            && sliding_friction_support_present(&input.nonlinear_supports, &current_states);
        let sliding_solve = solve_iteration_with_sliding_friction(
            input,
            &stiffness,
            &boundary,
            &current_states,
            iterations.last(),
            &friction_normals,
            &derived_friction_normals,
            linear_solve_mode,
        )?;
        let linearized = sliding_solve.linearized;
        let reactions = sliding_solve.reactions;
        let applied_sliding_friction_forces = sliding_solve.applied_forces;
        let trial_states = build_trial_states(
            &input.nonlinear_supports,
            &linearized.displacements,
            &reactions,
            &friction_normals,
            &derived_friction_normals,
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
        let converged = active_set.converged
            && !blocked
            && !sliding_force_deferred
            && sliding_solve.derived_normal_branches_admissible;
        current_states = active_set.states.clone();
        let residuals = residual_observation(
            &linearized,
            &reactions,
            iterations.last(),
            active_set.residual_norm,
        );

        let iteration = NonlinearFrameIteration {
            iteration: iteration_index,
            active_restrained_dofs: boundary.dofs.clone(),
            active_prescribed_displacements: boundary.displacements.clone(),
            displacements: linearized.displacements,
            reactions,
            applied_sliding_friction_forces,
            residuals,
            sparse_evidence: linearized.sparse_evidence,
            active_set,
        };
        iterations.push(iteration);

        if converged || blocked || iteration_index == input.convergence.max_iterations {
            let final_iteration = iterations
                .last()
                .expect("just pushed nonlinear iteration record");
            final_diagnostics.extend(final_iteration.active_set.diagnostics.clone());
            // Every non-converged exit must fail loudly. The active-set
            // classifier emits its residual-based nonconvergence diagnostic
            // only when the state-change residual exceeds the tolerance, so a
            // run reaching the iteration cap with a zero residual -- e.g. a
            // sliding-seeded friction support at max_iterations == 1, whose
            // first-iterate convergence is deferred so the bounded sliding
            // force can be applied -- would otherwise return
            // converged == false without any visible diagnostic (recorded
            // corner in WORKING_ITEMS_RUN_2026-07-10_TP-PMM-P2-FRICTION-001).
            if !converged
                && !final_diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.code == SolverDiagnosticCode::NonConvergence)
            {
                final_diagnostics.push(nonconverged_exit_diagnostic(
                    iteration_index,
                    sliding_force_deferred,
                    !sliding_solve.derived_normal_branches_admissible,
                    &current_states,
                ));
            }
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
        "The governed assembled-loop convergence residual is the nonlinear-support classifier state-change count; force/displacement/work residual axes are reported for callers to bind to explicit evidence policies.".to_string(),
        "Friction support normal reactions are either explicit input evidence or derived from the same linearized iterate through a named support-normal DOF supplied by the caller.".to_string(),
        "Released friction supports may persist in sliding state while nonzero displacement remains, preventing active-set chatter without adding hidden friction-load defaults.".to_string(),
        "A support classified sliding applies a bounded +/- mu*N tangential force opposing the observed motion, using the current iterate's normal-reaction evidence; a sliding state seeded before any solved iterate defers convergence one iteration so the bounded force is applied before the loop can converge.".to_string(),
        "Explicit user-stiffness macro-elements are assembled with frame elements when supplied by the caller.".to_string(),
        "Explicit curved-bend macro-element global stiffness slots supplied by the caller are assembled beside frame and user-stiffness elements in every linearized iteration.".to_string(),
    ]
}

pub fn assembled_loop_limitations() -> Vec<String> {
    vec![
        "DEC-053 sparse interactive mode uses direct reduced profile-entry sparse solves as the default linearized active-set path; dense scrutiny remains an explicit parity/review mode.".to_string(),
        "Sparse timing, allocator/RSS memory, hardware normalization, true condition-number, and CI evidence are observational R4 closure evidence, not release-performance thresholds.".to_string(),
        "DEC-046 threshold authority exists only where callers supply explicit controls and policy references; unmeasured classes and broader release/external thresholds remain out of scope.".to_string(),
        "User-stiffness and curved-bend macro-elements consume caller-supplied stiffness values only; pressure-thrust load generation, vendor defaults, and compliance checks are outside this loop.".to_string(),
        "The bounded sliding friction force is a same-iterate affine Coulomb coupling, not a path-dependent or load-step friction history model; its magnitude is not itself a convergence residual axis.".to_string(),
        "The result is mechanics decision-support evidence; acceptance and professional judgment remain with the responsible engineer.".to_string(),
    ]
}

/// Crate-constant component identity of the assembled active-set loop, for
/// producer-path metadata binding (R14 W1 T1). Derived from the crate
/// manifest, never hardcoded by consumers.
pub fn assembled_loop_component_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

/// Crate-constant component version of the assembled active-set loop. See
/// [`assembled_loop_component_name`].
pub fn assembled_loop_component_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
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
    max_abs_free_dof_force_residual: f64,
    max_abs_free_dof_moment_residual: f64,
    max_abs_free_dof_work_residual: f64,
    sparse_evidence: SparseLinearSolveEvidence,
}

fn validate_input(input: &NonlinearFrameSolveInput) -> Result<(), NonlinearIntegrationError> {
    // Reapply constructor checks at the execution boundary: fields are public.
    ConvergenceControl::new(
        input.convergence.policy_ref.clone(),
        input.convergence.policy_status,
        input.convergence.residual_tolerance,
        input.convergence.absolute_residual_floor,
        input.convergence.max_iterations,
    )?;
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
    let mut support_ids = HashSet::new();
    for support in &input.nonlinear_supports {
        if !support_ids.insert(support.support_id.as_str()) {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!("duplicate nonlinear support ID {}", support.support_id),
            });
        }
        if support.node_index >= input.node_count {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "nonlinear support {} node index {} is outside node count {}",
                    support.support_id, support.node_index, input.node_count
                ),
            });
        }
    }
    for element in &input.curved_bend_elements {
        for node_index in [element.node_i, element.node_j] {
            if node_index >= input.node_count {
                return Err(NonlinearIntegrationError::InvalidInput {
                    detail: format!(
                        "curved-bend macro-element {} node index {node_index} is outside node count {}",
                        element.element_id, input.node_count
                    ),
                });
            }
        }
    }
    Ok(())
}

/// Scatter-add the explicit curved-bend macro-element global stiffness beside
/// the frame and user-stiffness assembly for every linearized iteration.
fn add_curved_bend_stiffness_contributions(
    stiffness: &mut DenseMatrix,
    curved_bend_elements: &[CurvedBendStiffnessElement],
) {
    for element in curved_bend_elements {
        let mut dof_map = [0usize; ELEMENT_DOF];
        for local in 0..DOF_PER_NODE {
            dof_map[local] = element.node_i * DOF_PER_NODE + local;
            dof_map[DOF_PER_NODE + local] = element.node_j * DOF_PER_NODE + local;
        }
        for (local_row, &global_row) in dof_map.iter().enumerate() {
            for (local_col, &global_col) in dof_map.iter().enumerate() {
                stiffness[global_row][global_col] += element.global_stiffness[local_row][local_col];
            }
        }
    }
}

fn sliding_friction_support_present(
    supports: &[NonlinearSupport],
    states: &[SupportStateRecord],
) -> bool {
    let state_map = states
        .iter()
        .map(|state| (state.support_id.as_str(), state.state))
        .collect::<HashMap<_, _>>();
    supports.iter().any(|support| {
        matches!(support.behavior, NonlinearSupportBehavior::Friction)
            && state_map.get(support.support_id.as_str()).copied() == Some(ActiveSetState::Sliding)
    })
}

#[derive(Debug, Clone, Copy)]
enum SlidingNormalSource {
    Explicit(f64),
    Derived { source_global_dof: usize },
}

#[derive(Debug, Clone)]
struct SlidingFrictionCandidate {
    support_id: String,
    global_dof: usize,
    direction: f64,
    coefficient: f64,
    normal_source: SlidingNormalSource,
}

struct SlidingIterationSolve {
    linearized: LinearizedSolve,
    reactions: DenseVector,
    applied_forces: Vec<AppliedSlidingFrictionForce>,
    derived_normal_branches_admissible: bool,
}

/// Solve one active-set iteration with bounded +/- mu*N Coulomb forces.
/// Explicit normals remain constant loads. Active derived normals are solved
/// simultaneously from the affine base response and unit-load reaction
/// influences, so every applied force uses its current-iterate normal.
fn solve_iteration_with_sliding_friction(
    input: &NonlinearFrameSolveInput,
    stiffness: &DenseMatrix,
    boundary: &BoundaryState,
    states: &[SupportStateRecord],
    previous: Option<&NonlinearFrameIteration>,
    friction_normals: &HashMap<&str, f64>,
    derived_friction_normals: &HashMap<&str, &DerivedFrictionNormalReaction>,
    linear_solve_mode: LinearSolveMode,
) -> Result<SlidingIterationSolve, NonlinearIntegrationError> {
    let candidates = sliding_friction_candidates(
        input,
        states,
        previous,
        friction_normals,
        derived_friction_normals,
    )?;

    let mut explicit_forces = Vec::new();
    let mut derived_candidates = Vec::new();
    for (candidate_index, candidate) in candidates.iter().enumerate() {
        match candidate.normal_source {
            SlidingNormalSource::Explicit(normal) => {
                let force = -candidate.direction * candidate.coefficient * normal.abs();
                validate_finite("explicit-normal sliding friction force", force)?;
                if force != 0.0 {
                    explicit_forces.push(AppliedSlidingFrictionForce {
                        support_id: candidate.support_id.clone(),
                        global_dof: candidate.global_dof,
                        force,
                    });
                }
            }
            SlidingNormalSource::Derived { .. }
                if candidate.coefficient > 0.0 && candidate.direction != 0.0 =>
            {
                derived_candidates.push(candidate_index);
            }
            SlidingNormalSource::Derived { .. } => {}
        }
    }

    let mut base_force = input.force.clone();
    add_applied_forces(&mut base_force, &explicit_forces);

    let mut solved_derived_forces = HashMap::new();
    let mut assumed_branches = HashMap::new();
    if !derived_candidates.is_empty() {
        let base = solve_linearized_system(
            stiffness,
            &base_force,
            boundary,
            LinearSolveMode::DenseScrutiny,
        )?;
        let base_reactions = reported_reactions(&base.reactions, &explicit_forces)?;

        let mut branches = Vec::with_capacity(derived_candidates.len());
        for &candidate_index in &derived_candidates {
            let candidate = &candidates[candidate_index];
            let SlidingNormalSource::Derived { source_global_dof } = candidate.normal_source else {
                unreachable!("derived candidate index must name a derived normal")
            };
            let previous_reaction = previous
                .expect("derived sliding candidate requires a previous iterate")
                .reactions[source_global_dof];
            let branch = if previous_reaction == 0.0 {
                normalized_reaction_sign(base_reactions[source_global_dof])
            } else {
                normalized_reaction_sign(previous_reaction)
            };
            branches.push(branch);
            assumed_branches.insert(candidate_index, branch);
        }

        let row_count = derived_candidates.len();
        let mut reaction_influence = vec![vec![0.0; row_count]; row_count];
        for (column, &load_candidate_index) in derived_candidates.iter().enumerate() {
            let load_candidate = &candidates[load_candidate_index];
            let mut unit_force = base_force.clone();
            unit_force[load_candidate.global_dof] += 1.0;
            let unit = solve_linearized_system(
                stiffness,
                &unit_force,
                boundary,
                LinearSolveMode::DenseScrutiny,
            )?;
            let mut unit_applied = explicit_forces.clone();
            unit_applied.push(AppliedSlidingFrictionForce {
                support_id: load_candidate.support_id.clone(),
                global_dof: load_candidate.global_dof,
                force: 1.0,
            });
            let unit_reactions = reported_reactions(&unit.reactions, &unit_applied)?;
            for (row, &reaction_candidate_index) in derived_candidates.iter().enumerate() {
                let SlidingNormalSource::Derived { source_global_dof } =
                    candidates[reaction_candidate_index].normal_source
                else {
                    unreachable!("derived candidate index must name a derived normal")
                };
                let influence =
                    unit_reactions[source_global_dof] - base_reactions[source_global_dof];
                validate_finite("derived-normal reaction influence", influence)?;
                reaction_influence[row][column] = influence;
            }
        }

        let mut coupling = vec![vec![0.0; row_count]; row_count];
        let mut right_hand_side = vec![0.0; row_count];
        for (row, &candidate_index) in derived_candidates.iter().enumerate() {
            let candidate = &candidates[candidate_index];
            let SlidingNormalSource::Derived { source_global_dof } = candidate.normal_source else {
                unreachable!("derived candidate index must name a derived normal")
            };
            let c = candidate.direction * candidate.coefficient * branches[row];
            validate_finite("derived-normal Coulomb branch coefficient", c)?;
            right_hand_side[row] = -c * base_reactions[source_global_dof];
            validate_finite(
                "derived-normal Coulomb coupling right-hand side",
                right_hand_side[row],
            )?;
            for column in 0..row_count {
                coupling[row][column] = c * reaction_influence[row][column];
                if row == column {
                    coupling[row][column] += 1.0;
                }
                validate_finite(
                    "derived-normal Coulomb coupling matrix entry",
                    coupling[row][column],
                )?;
            }
        }

        let forces = solve_dense(&coupling, &right_hand_side)?;
        for (&candidate_index, force) in derived_candidates.iter().zip(forces) {
            validate_finite("solved derived-normal sliding friction force", force)?;
            solved_derived_forces.insert(candidate_index, force);
        }
    }

    // Reconstruct public applied-force records in nonlinear-support input order.
    let mut applied_forces = Vec::new();
    for (candidate_index, candidate) in candidates.iter().enumerate() {
        let force = match candidate.normal_source {
            SlidingNormalSource::Explicit(normal) => {
                -candidate.direction * candidate.coefficient * normal.abs()
            }
            SlidingNormalSource::Derived { .. } => solved_derived_forces
                .get(&candidate_index)
                .copied()
                .unwrap_or(0.0),
        };
        if force != 0.0 {
            applied_forces.push(AppliedSlidingFrictionForce {
                support_id: candidate.support_id.clone(),
                global_dof: candidate.global_dof,
                force,
            });
        }
    }

    let mut final_force = input.force.clone();
    add_applied_forces(&mut final_force, &applied_forces);
    let linearized = solve_linearized_system(stiffness, &final_force, boundary, linear_solve_mode)?;
    let reactions = reported_reactions(&linearized.reactions, &applied_forces)?;

    let derived_normal_branches_admissible = derived_candidates.iter().all(|candidate_index| {
        let candidate = &candidates[*candidate_index];
        let SlidingNormalSource::Derived { source_global_dof } = candidate.normal_source else {
            unreachable!("derived candidate index must name a derived normal")
        };
        derived_normal_branch_admissible(
            candidate.coefficient,
            candidate.direction,
            assumed_branches[candidate_index],
            reactions[source_global_dof],
        )
    });

    Ok(SlidingIterationSolve {
        linearized,
        reactions,
        applied_forces,
        derived_normal_branches_admissible,
    })
}

fn sliding_friction_candidates(
    input: &NonlinearFrameSolveInput,
    states: &[SupportStateRecord],
    previous: Option<&NonlinearFrameIteration>,
    friction_normals: &HashMap<&str, f64>,
    derived_friction_normals: &HashMap<&str, &DerivedFrictionNormalReaction>,
) -> Result<Vec<SlidingFrictionCandidate>, NonlinearIntegrationError> {
    let state_map = states
        .iter()
        .map(|state| (state.support_id.as_str(), state.state))
        .collect::<HashMap<_, _>>();
    let mut candidates = Vec::new();
    for support in &input.nonlinear_supports {
        if !matches!(support.behavior, NonlinearSupportBehavior::Friction) {
            continue;
        }
        if state_map.get(support.support_id.as_str()).copied() != Some(ActiveSetState::Sliding) {
            continue;
        }
        let Some(previous) = previous else {
            continue;
        };
        let global_dof = node_dof_index(support.node_index, support.dof);
        let direction = sliding_direction(
            previous.displacements[global_dof],
            previous.reactions[global_dof],
        );
        if direction == 0.0 {
            continue;
        }
        let coefficient = support.friction_coefficient.ok_or_else(|| {
            NonlinearSupportError::MissingFrictionCoefficient {
                support_id: support.support_id.clone(),
            }
        })?;
        validate_nonnegative_finite("friction coefficient", coefficient)?;
        let normal_source =
            if let Some(normal) = friction_normals.get(support.support_id.as_str()).copied() {
                SlidingNormalSource::Explicit(normal)
            } else if let Some(source) = derived_friction_normals
                .get(support.support_id.as_str())
                .copied()
            {
                SlidingNormalSource::Derived {
                    source_global_dof: node_dof_index(source.source_node_index, source.source_dof),
                }
            } else {
                return Err(NonlinearSupportError::MissingFrictionData {
                    support_id: support.support_id.clone(),
                }
                .into());
            };
        candidates.push(SlidingFrictionCandidate {
            support_id: support.support_id.clone(),
            global_dof,
            direction,
            coefficient,
            normal_source,
        });
    }
    Ok(candidates)
}

fn add_applied_forces(force: &mut [f64], applied_forces: &[AppliedSlidingFrictionForce]) {
    for applied in applied_forces {
        force[applied.global_dof] += applied.force;
    }
}

/// Convert a solved reaction vector to the public convention, which reports
/// bounded friction loads as support reactions against the caller's base load.
fn reported_reactions(
    solved_reactions: &[f64],
    applied_forces: &[AppliedSlidingFrictionForce],
) -> Result<DenseVector, NonlinearIntegrationError> {
    let mut reactions = solved_reactions.to_vec();
    add_applied_forces(&mut reactions, applied_forces);
    for value in &reactions {
        validate_finite("selected reaction", *value)?;
    }
    Ok(reactions)
}

fn normalized_reaction_sign(reaction: f64) -> f64 {
    if reaction == 0.0 {
        0.0
    } else {
        reaction.signum()
    }
}

fn derived_normal_branch_admissible(
    coefficient: f64,
    direction: f64,
    assumed_branch: f64,
    current_reaction: f64,
) -> bool {
    if coefficient == 0.0 || direction == 0.0 {
        true
    } else if assumed_branch == 0.0 {
        current_reaction == 0.0
    } else {
        assumed_branch * current_reaction >= 0.0
    }
}

/// Sliding motion sense at the support DOF from the prior iterate: the free
/// trial displacement when nonzero, otherwise the impending-motion sense
/// opposite the restrained tangential reaction. Zero means no deterministic
/// direction evidence exists yet.
fn sliding_direction(previous_displacement: f64, previous_reaction: f64) -> f64 {
    if previous_displacement != 0.0 {
        previous_displacement.signum()
    } else if previous_reaction != 0.0 {
        -previous_reaction.signum()
    } else {
        0.0
    }
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

fn derived_friction_normal_map(
    input: &NonlinearFrameSolveInput,
) -> Result<HashMap<&str, &DerivedFrictionNormalReaction>, NonlinearIntegrationError> {
    let mut normals = HashMap::new();
    let explicit_ids = input
        .friction_normal_reactions
        .iter()
        .map(|reaction| reaction.support_id.as_str())
        .collect::<HashSet<_>>();
    let expected_dofs = input.node_count * DOF_PER_NODE;
    for source in &input.derived_friction_normal_reactions {
        if explicit_ids.contains(source.support_id.as_str()) {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "friction normal reaction source for support {} is both explicit and derived",
                    source.support_id
                ),
            });
        }
        let global_dof = node_dof_index(source.source_node_index, source.source_dof);
        if global_dof >= expected_dofs {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "derived friction normal source {} DOF {global_dof} is outside total DOF count {expected_dofs}",
                    source.source_ref
                ),
            });
        }
        if normals.insert(source.support_id.as_str(), source).is_some() {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "derived friction normal reaction source for support {} is repeated",
                    source.support_id
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
    linear_solve_mode: LinearSolveMode,
) -> Result<LinearizedSolve, NonlinearIntegrationError> {
    let reduced = reduce_system_with_prescribed_displacements(
        stiffness,
        force,
        &boundary.dofs,
        &boundary.displacements,
    )?;
    let (reduced_solution, sparse_evidence) = match linear_solve_mode {
        LinearSolveMode::SparseInteractive => solve_sparse_interactive_linearized_system(
            stiffness,
            &reduced.stiffness,
            &reduced.free_dofs,
            &reduced.force,
        )?,
        LinearSolveMode::DenseScrutiny => {
            let reduced_solution = solve_dense(&reduced.stiffness, &reduced.force)?;
            let sparse_evidence = observe_sparse_linearized_solve(
                stiffness,
                &reduced.free_dofs,
                &reduced.force,
                &reduced_solution,
                LinearSolveMode::DenseScrutiny,
            );
            (reduced_solution, sparse_evidence)
        }
    };

    let mut displacements = vec![0.0; force.len()];
    for (&dof, &value) in boundary.dofs.iter().zip(boundary.displacements.iter()) {
        displacements[dof] = value;
    }
    for (&global_dof, &value) in reduced.free_dofs.iter().zip(reduced_solution.iter()) {
        displacements[global_dof] = value;
    }

    let reactions: DenseVector = multiply_matrix_vector(stiffness, &displacements)?
        .into_iter()
        .zip(force.iter())
        .map(|(internal, applied)| internal - applied)
        .collect();
    for value in displacements.iter().chain(reactions.iter()) {
        validate_finite("computed displacement or reaction", *value)?;
    }
    let max_abs_free_dof_force_residual =
        max_abs_by_dof_group(&reactions, &reduced.free_dofs, true);
    let max_abs_free_dof_moment_residual =
        max_abs_by_dof_group(&reactions, &reduced.free_dofs, false);
    let max_abs_free_dof_work_residual =
        max_abs_free_dof_work_residual(&reactions, &displacements, &reduced.free_dofs);

    validate_finite("free DOF work residual", max_abs_free_dof_work_residual)?;
    Ok(LinearizedSolve {
        displacements,
        reactions,
        max_abs_free_dof_force_residual,
        max_abs_free_dof_moment_residual,
        max_abs_free_dof_work_residual,
        sparse_evidence,
    })
}

fn solve_sparse_interactive_linearized_system(
    global_stiffness: &DenseMatrix,
    reduced_stiffness: &DenseMatrix,
    free_dofs: &[usize],
    reduced_force: &[f64],
) -> Result<(DenseVector, SparseLinearSolveEvidence), NonlinearIntegrationError> {
    let entries = match direct_reduced_profile_entries(global_stiffness, free_dofs) {
        Ok(entries) => entries,
        Err(error) => {
            let dense_solution = solve_dense(reduced_stiffness, reduced_force)?;
            return Ok((
                dense_solution,
                sparse_evidence_dense_fallback(reduced_force.len(), error),
            ));
        }
    };

    match solve_symmetric_system_from_entries(reduced_force.len(), &entries, reduced_force) {
        Ok(sparse) => {
            let evidence = sparse_linear_solve_evidence(
                reduced_force.len(),
                &entries,
                reduced_force,
                &sparse.solution,
                &sparse,
                None,
                LinearSolveMode::SparseInteractive,
            );
            Ok((sparse.solution, evidence))
        }
        Err(error) => {
            let reduced_dense = dense_from_entries(reduced_force.len(), &entries)?;
            let dense_solution = solve_dense(&reduced_dense, reduced_force)?;
            Ok((
                dense_solution,
                sparse_evidence_dense_fallback(reduced_force.len(), error.to_string()),
            ))
        }
    }
}

fn observe_sparse_linearized_solve(
    global_stiffness: &DenseMatrix,
    free_dofs: &[usize],
    reduced_force: &[f64],
    dense_solution: &[f64],
    linear_solve_mode: LinearSolveMode,
) -> SparseLinearSolveEvidence {
    let entries = match direct_reduced_profile_entries(global_stiffness, free_dofs) {
        Ok(entries) => entries,
        Err(error) => {
            return sparse_evidence_unavailable(reduced_force.len(), error, linear_solve_mode);
        }
    };

    match solve_symmetric_system_from_entries(reduced_force.len(), &entries, reduced_force) {
        Ok(sparse) => sparse_linear_solve_evidence(
            reduced_force.len(),
            &entries,
            reduced_force,
            &sparse.solution,
            &sparse,
            Some(dense_solution),
            linear_solve_mode,
        ),
        Err(error) => {
            sparse_evidence_unavailable(reduced_force.len(), error.to_string(), linear_solve_mode)
        }
    }
}

fn sparse_linear_solve_evidence(
    reduced_dof_count: usize,
    entries: &[SymmetricMatrixEntry],
    reduced_force: &[f64],
    sparse_solution: &[f64],
    sparse: &open_pipe_stress_sparse_direct::SparseSolveResult,
    dense_solution: Option<&[f64]>,
    linear_solve_mode: LinearSolveMode,
) -> SparseLinearSolveEvidence {
    let max_abs_dense_sparse_solution_delta =
        dense_solution.map(|dense| max_abs_delta(dense, sparse_solution));
    let relative_dense_sparse_solution_delta =
        match (dense_solution, max_abs_dense_sparse_solution_delta) {
            (Some(dense), Some(delta)) => {
                let dense_scale = max_abs_value(dense);
                Some(if dense_scale > 0.0 {
                    delta / dense_scale
                } else {
                    delta
                })
            }
            _ => None,
        };
    let residual =
        max_abs_entry_residual(reduced_dof_count, entries, sparse_solution, reduced_force);
    if !residual.is_finite()
        || max_abs_dense_sparse_solution_delta.is_some_and(|v| !v.is_finite())
        || relative_dense_sparse_solution_delta.is_some_and(|v| !v.is_finite())
        || sparse
            .factorization
            .pivot_condition_ratio_estimate
            .is_some_and(|v| !v.is_finite())
    {
        return sparse_evidence_unavailable(
            reduced_dof_count,
            "nonfinite computed sparse scrutiny evidence".to_string(),
            linear_solve_mode,
        );
    }
    SparseLinearSolveEvidence {
        status: SparseEvidenceStatus::Observed,
        policy_ref: "DEC-053".to_string(),
        solver_mode: linear_solve_mode,
        solution_basis: linear_solve_mode.solution_basis(false).to_string(),
        assembly_basis: DEC050_DIRECT_REDUCED_PROFILE_ASSEMBLY_BASIS.to_string(),
        reduced_dof_count,
        original_max_half_bandwidth: Some(sparse.original_max_half_bandwidth),
        ordered_max_half_bandwidth: Some(sparse.ordered_max_half_bandwidth),
        original_profile_entry_count: Some(sparse.original_profile_entry_count),
        ordered_profile_entry_count: Some(sparse.ordered_profile_entry_count),
        nonpositive_pivot_count: Some(sparse.factorization.nonpositive_pivot_count),
        pivot_condition_ratio_estimate: sparse.factorization.pivot_condition_ratio_estimate,
        max_abs_dense_sparse_solution_delta,
        relative_dense_sparse_solution_delta,
        max_abs_sparse_residual: Some(max_abs_entry_residual(
            reduced_dof_count,
            entries,
            sparse_solution,
            reduced_force,
        )),
        failure_message: None,
    }
}

fn direct_reduced_profile_entries(
    global_stiffness: &DenseMatrix,
    free_dofs: &[usize],
) -> Result<Vec<SymmetricMatrixEntry>, String> {
    let mut entries = Vec::new();
    for (reduced_row, &global_row) in free_dofs.iter().enumerate() {
        for (reduced_col, &global_col) in free_dofs.iter().take(reduced_row + 1).enumerate() {
            let value = *global_stiffness
                .get(global_row)
                .and_then(|row| row.get(global_col))
                .ok_or_else(|| {
                    format!(
                        "free DOF map references global stiffness entry ({global_row}, {global_col}) outside matrix dimensions"
                    )
                })?;
            if !value.is_finite() {
                return Err(format!(
                    "direct reduced sparse entry ({reduced_row}, {reduced_col}) is non-finite"
                ));
            }
            if value == 0.0 {
                continue;
            }
            entries.push(SymmetricMatrixEntry {
                row: reduced_row,
                col: reduced_col,
                value,
            });
        }
    }
    Ok(entries)
}

fn dense_from_entries(
    dimension: usize,
    entries: &[SymmetricMatrixEntry],
) -> Result<DenseMatrix, NonlinearIntegrationError> {
    let mut dense = vec![vec![0.0; dimension]; dimension];
    for entry in entries {
        if entry.row >= dimension || entry.col >= dimension {
            return Err(NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "sparse fallback entry ({}, {}) is outside reduced dimension {dimension}",
                    entry.row, entry.col
                ),
            });
        }
        dense[entry.row][entry.col] += entry.value;
        if entry.row != entry.col {
            dense[entry.col][entry.row] += entry.value;
        }
    }
    Ok(dense)
}

fn sparse_evidence_unavailable(
    reduced_dof_count: usize,
    failure_message: String,
    linear_solve_mode: LinearSolveMode,
) -> SparseLinearSolveEvidence {
    SparseLinearSolveEvidence {
        status: SparseEvidenceStatus::Unavailable,
        policy_ref: "DEC-053".to_string(),
        solver_mode: linear_solve_mode,
        solution_basis: linear_solve_mode.solution_basis(false).to_string(),
        assembly_basis: DEC050_DIRECT_REDUCED_PROFILE_ASSEMBLY_BASIS.to_string(),
        reduced_dof_count,
        original_max_half_bandwidth: None,
        ordered_max_half_bandwidth: None,
        original_profile_entry_count: None,
        ordered_profile_entry_count: None,
        nonpositive_pivot_count: None,
        pivot_condition_ratio_estimate: None,
        max_abs_dense_sparse_solution_delta: None,
        relative_dense_sparse_solution_delta: None,
        max_abs_sparse_residual: None,
        failure_message: Some(failure_message),
    }
}

fn sparse_evidence_dense_fallback(
    reduced_dof_count: usize,
    failure_message: String,
) -> SparseLinearSolveEvidence {
    SparseLinearSolveEvidence {
        status: SparseEvidenceStatus::DenseFallbackAfterSparseFailure,
        policy_ref: "DEC-053".to_string(),
        solver_mode: LinearSolveMode::SparseInteractive,
        solution_basis: LinearSolveMode::SparseInteractive
            .solution_basis(true)
            .to_string(),
        assembly_basis: DEC050_DIRECT_REDUCED_PROFILE_ASSEMBLY_BASIS.to_string(),
        reduced_dof_count,
        original_max_half_bandwidth: None,
        ordered_max_half_bandwidth: None,
        original_profile_entry_count: None,
        ordered_profile_entry_count: None,
        nonpositive_pivot_count: None,
        pivot_condition_ratio_estimate: None,
        max_abs_dense_sparse_solution_delta: None,
        relative_dense_sparse_solution_delta: None,
        max_abs_sparse_residual: None,
        failure_message: Some(failure_message),
    }
}

fn residual_observation(
    linearized: &LinearizedSolve,
    reactions: &[f64],
    previous: Option<&NonlinearFrameIteration>,
    active_set_changed_support_count: f64,
) -> NonlinearResidualObservation {
    NonlinearResidualObservation {
        active_set_changed_support_count,
        max_abs_translation_delta_from_previous: previous
            .map(|previous| {
                max_abs_delta_by_dof_group(&linearized.displacements, &previous.displacements, true)
            })
            .filter(|value| value.is_finite()),
        max_abs_rotation_delta_from_previous: previous
            .map(|previous| {
                max_abs_delta_by_dof_group(
                    &linearized.displacements,
                    &previous.displacements,
                    false,
                )
            })
            .filter(|value| value.is_finite()),
        max_abs_force_reaction_delta_from_previous: previous
            .map(|previous| max_abs_delta_by_dof_group(reactions, &previous.reactions, true))
            .filter(|value| value.is_finite()),
        max_abs_moment_reaction_delta_from_previous: previous
            .map(|previous| max_abs_delta_by_dof_group(reactions, &previous.reactions, false))
            .filter(|value| value.is_finite()),
        max_abs_free_dof_force_residual: linearized.max_abs_free_dof_force_residual,
        max_abs_free_dof_moment_residual: linearized.max_abs_free_dof_moment_residual,
        max_abs_free_dof_work_residual: linearized.max_abs_free_dof_work_residual,
    }
}

fn finite_max(left: f64, right: f64) -> f64 {
    if left.is_finite() && right.is_finite() {
        left.max(right)
    } else {
        f64::NAN
    }
}

fn max_abs_free_dof_work_residual(
    reactions: &[f64],
    displacements: &[f64],
    free_dofs: &[usize],
) -> f64 {
    free_dofs
        .iter()
        .map(|&dof| (reactions[dof] * displacements[dof]).abs())
        .fold(0.0, finite_max)
}

fn max_abs_delta_by_dof_group(current: &[f64], previous: &[f64], translations: bool) -> f64 {
    current
        .iter()
        .zip(previous.iter())
        .enumerate()
        .filter(|(index, _)| is_translation_dof_index(*index) == translations)
        .map(|(_, (current, previous))| (current - previous).abs())
        .fold(0.0, finite_max)
}

fn max_abs_delta(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .fold(0.0, finite_max)
}

fn max_abs_value(values: &[f64]) -> f64 {
    values.iter().copied().map(f64::abs).fold(0.0, finite_max)
}

fn max_abs_entry_residual(
    dimension: usize,
    entries: &[SymmetricMatrixEntry],
    solution: &[f64],
    force: &[f64],
) -> f64 {
    let mut internal = vec![0.0; dimension];
    for entry in entries {
        internal[entry.row] += entry.value * solution[entry.col];
        if entry.row != entry.col {
            internal[entry.col] += entry.value * solution[entry.row];
        }
    }
    internal
        .iter()
        .zip(force.iter())
        .map(|(internal, applied)| (internal - applied).abs())
        .fold(0.0, finite_max)
}

fn max_abs_by_dof_group(values: &[f64], dofs: &[usize], translations: bool) -> f64 {
    dofs.iter()
        .copied()
        .filter(|dof| is_translation_dof_index(*dof) == translations)
        .filter_map(|dof| values.get(dof).copied())
        .map(f64::abs)
        .fold(0.0, finite_max)
}

fn is_translation_dof_index(global_dof: usize) -> bool {
    matches!(global_dof % DOF_PER_NODE, 0 | 1 | 2)
}

fn build_trial_states(
    supports: &[NonlinearSupport],
    displacements: &[f64],
    reactions: &[f64],
    friction_normals: &HashMap<&str, f64>,
    derived_friction_normals: &HashMap<&str, &DerivedFrictionNormalReaction>,
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
            let normal = friction_normal_for_support(
                support,
                reactions,
                friction_normals,
                derived_friction_normals,
            )?;
            trial = trial.with_friction_reactions(normal, reactions[global_dof])?;
        }
        trial_states.push(trial);
    }
    Ok(trial_states)
}

fn friction_normal_for_support(
    support: &NonlinearSupport,
    reactions: &[f64],
    friction_normals: &HashMap<&str, f64>,
    derived_friction_normals: &HashMap<&str, &DerivedFrictionNormalReaction>,
) -> Result<f64, NonlinearIntegrationError> {
    if let Some(normal) = friction_normals.get(support.support_id.as_str()).copied() {
        return Ok(normal);
    }

    if let Some(source) = derived_friction_normals
        .get(support.support_id.as_str())
        .copied()
    {
        let global_dof = node_dof_index(source.source_node_index, source.source_dof);
        let reaction = reactions.get(global_dof).copied().ok_or_else(|| {
            NonlinearIntegrationError::InvalidInput {
                detail: format!(
                    "derived friction normal source {} DOF {global_dof} is outside reaction vector",
                    source.source_ref
                ),
            }
        })?;
        return Ok(reaction.abs());
    }

    Err(NonlinearSupportError::MissingFrictionData {
        support_id: support.support_id.clone(),
    }
    .into())
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

/// Failure diagnostic for a non-converged loop exit that carries no
/// residual-based `NonConvergence` diagnostic from the active-set classifier.
///
/// These exits include a deferred first sliding force or an inadmissible exact
/// signed-normal branch reaching the existing iteration cap while the support
/// state-change residual is zero. The guard is written for every non-converged
/// exit so the `converged == false` -> visible-diagnostic contract does not
/// depend on which path produced the exit. Diagnostics only; no mechanics
/// change.
fn nonconverged_exit_diagnostic(
    iteration_count: usize,
    sliding_force_deferred: bool,
    derived_normal_branch_invalid: bool,
    final_states: &[SupportStateRecord],
) -> SolverDiagnostic {
    let cause = if sliding_force_deferred {
        "the sliding-seeded first iterate defers convergence so the bounded sliding-friction \
         force can be applied, and the iteration cap was reached before a post-deferral \
         iterate could be evaluated"
    } else if derived_normal_branch_invalid {
        "a derived-normal sliding-friction row remained outside its assumed exact signed-normal \
         branch when the iteration cap was reached"
    } else {
        "the loop exited before the active-set convergence check accepted an iterate"
    };
    let remediation = if sliding_force_deferred {
        "Raise max_iterations above 1 so the loop can evaluate an iterate after the deferred \
         sliding-force first iterate."
    } else if derived_normal_branch_invalid {
        "Raise max_iterations or review the coupled derived-normal friction configuration before \
         reuse."
    } else {
        "Raise max_iterations or review the nonlinear support configuration before reuse."
    };
    let state_summary = if final_states.is_empty() {
        "none".to_string()
    } else {
        final_states
            .iter()
            .map(|record| format!("{}={}", record.support_id, record.state.as_str()))
            .collect::<Vec<_>>()
            .join(",")
    };
    SolverDiagnostic::new(
        SolverDiagnosticCode::NonConvergence,
        DiagnosticSeverity::Failure,
        DiagnosticSource::SolverIteration,
        format!(
            "nonlinear active-set solve exited non-converged after {iteration_count} iteration(s) \
             without a residual-based nonconvergence diagnostic: {cause}; active-set states: \
             {state_summary}"
        ),
    )
    .with_remediation(remediation)
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
mod component_identity_tests {
    use super::{assembled_loop_component_name, assembled_loop_component_version};

    #[test]
    fn component_identity_accessors_are_crate_constant_derived() {
        assert_eq!(
            assembled_loop_component_name(),
            "open_pipe_stress_nonlinear_integration"
        );
        assert!(!assembled_loop_component_version().trim().is_empty());
        // Semantic-version shape from the crate manifest, not free text.
        assert_eq!(
            assembled_loop_component_version()
                .split('.')
                .filter(|part| part.chars().all(|ch| ch.is_ascii_digit()) && !part.is_empty())
                .count(),
            3
        );
    }
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
            user_stiffness_elements: Vec::new(),
            curved_bend_elements: Vec::new(),
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
            derived_friction_normal_reactions: Vec::new(),
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

    fn coupled_normal_friction_problem(
        force_x: f64,
        force_y: f64,
        seed: ActiveSetState,
        max_iterations: usize,
    ) -> NonlinearFrameSolveInput {
        let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(1, [1.0, 1.0, 0.0]).unwrap();
        let mut input = two_node_axial_problem(Vec::new(), Vec::new(), max_iterations);
        input.elements.clear();
        input.user_stiffness_elements = vec![UserStiffnessElement::new(
            node_i,
            node_j,
            [-1.0, 1.0, 0.0],
            100.0,
            200.0,
            200.0,
            200.0,
        )
        .unwrap()];
        input.force[node_dof_index(1, FrameDof::Ux)] = force_x;
        input.force[node_dof_index(1, FrameDof::Uy)] = force_y;
        input.nonlinear_supports =
            vec![NonlinearSupport::friction("F", 1, FrameDof::Ux, 0.30).unwrap()];
        input.initial_states = vec![SupportStateRecord::new("F", seed)];
        input.derived_friction_normal_reactions =
            vec![DerivedFrictionNormalReaction::from_support_reaction(
                "F",
                1,
                FrameDof::Uy,
                "fixed_y",
            )
            .unwrap()];
        input
    }

    fn two_row_current_normal_problem(
        seed: ActiveSetState,
        first_source_force: f64,
        second_source_force: f64,
        tangential_forces: [f64; 2],
        support_order: [&str; 2],
        max_iterations: usize,
    ) -> NonlinearFrameSolveInput {
        let mut stiffness = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
        for (index, row) in stiffness.iter_mut().enumerate() {
            row[index] = 1_000.0;
        }
        let ux = DOF_PER_NODE + UX;
        let uy = DOF_PER_NODE + UY;
        let uz = DOF_PER_NODE + UZ;
        let rx = DOF_PER_NODE + RX;
        stiffness[ux][ux] = 100.0;
        stiffness[uy][uy] = 100.0;
        stiffness[uz][ux] = 1.0;
        stiffness[ux][uz] = 1.0;
        stiffness[uz][uy] = -1.0;
        stiffness[uy][uz] = -1.0;
        stiffness[rx][uy] = 10.0;
        stiffness[uy][rx] = 10.0;

        let slot =
            CurvedBendStiffnessElement::new("test-coupled-stiffness", 0, 1, stiffness).unwrap();
        let mut force = vec![0.0; 2 * DOF_PER_NODE];
        force[node_dof_index(1, FrameDof::Ux)] = tangential_forces[0];
        force[node_dof_index(1, FrameDof::Uy)] = tangential_forces[1];
        force[node_dof_index(1, FrameDof::Uz)] = first_source_force;
        force[node_dof_index(1, FrameDof::Rx)] = second_source_force;

        let support_for = |id: &str| match id {
            "F-X" => NonlinearSupport::friction("F-X", 1, FrameDof::Ux, 0.20).unwrap(),
            "F-Y" => NonlinearSupport::friction("F-Y", 1, FrameDof::Uy, 0.25).unwrap(),
            _ => unreachable!(),
        };
        let normal_for = |id: &str| match id {
            "F-X" => DerivedFrictionNormalReaction::from_support_reaction(
                "F-X",
                1,
                FrameDof::Uz,
                "source-x",
            )
            .unwrap(),
            "F-Y" => DerivedFrictionNormalReaction::from_support_reaction(
                "F-Y",
                1,
                FrameDof::Rx,
                "source-y",
            )
            .unwrap(),
            _ => unreachable!(),
        };

        NonlinearFrameSolveInput {
            node_count: 2,
            elements: Vec::new(),
            user_stiffness_elements: Vec::new(),
            curved_bend_elements: vec![slot],
            force,
            base_restrained_dofs: (0..2 * DOF_PER_NODE)
                .filter(|dof| {
                    *dof != node_dof_index(1, FrameDof::Ux)
                        && *dof != node_dof_index(1, FrameDof::Uy)
                })
                .collect(),
            nonlinear_supports: support_order.iter().map(|id| support_for(id)).collect(),
            initial_states: support_order
                .iter()
                .map(|id| SupportStateRecord::new(*id, seed))
                .collect(),
            friction_normal_reactions: Vec::new(),
            derived_friction_normal_reactions: support_order
                .iter()
                .map(|id| normal_for(id))
                .collect(),
            convergence: ConvergenceControl::new(
                "DEC-046-current-normal-affine-fixture",
                ConvergencePolicyStatus::Accepted,
                0.0,
                0.0,
                max_iterations,
            )
            .unwrap(),
        }
    }

    fn exact_zero_current_normal_problem(
        load_sign: f64,
        max_iterations: usize,
    ) -> NonlinearFrameSolveInput {
        let mut stiffness = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
        for (index, row) in stiffness.iter_mut().enumerate() {
            row[index] = 10.0;
        }
        let ux = DOF_PER_NODE + UX;
        let uy = DOF_PER_NODE + UY;
        stiffness[ux][ux] = 1.0;
        stiffness[uy][uy] = 2.0;
        stiffness[ux][uy] = -1.0;
        stiffness[uy][ux] = -1.0;
        let slot =
            CurvedBendStiffnessElement::new("test-zero-normal-stiffness", 0, 1, stiffness).unwrap();
        let mut force = vec![0.0; 2 * DOF_PER_NODE];
        force[node_dof_index(1, FrameDof::Ux)] = load_sign;
        force[node_dof_index(1, FrameDof::Uy)] = -load_sign;
        NonlinearFrameSolveInput {
            node_count: 2,
            elements: Vec::new(),
            user_stiffness_elements: Vec::new(),
            curved_bend_elements: vec![slot],
            force,
            base_restrained_dofs: (0..2 * DOF_PER_NODE)
                .filter(|dof| *dof != node_dof_index(1, FrameDof::Ux))
                .collect(),
            nonlinear_supports: vec![
                NonlinearSupport::friction("F-ZERO", 1, FrameDof::Ux, 0.5).unwrap()
            ],
            initial_states: vec![SupportStateRecord::new("F-ZERO", ActiveSetState::Sticking)],
            friction_normal_reactions: Vec::new(),
            derived_friction_normal_reactions: vec![
                DerivedFrictionNormalReaction::from_support_reaction(
                    "F-ZERO",
                    1,
                    FrameDof::Uy,
                    "source-zero",
                )
                .unwrap(),
            ],
            convergence: ConvergenceControl::new(
                "DEC-046-current-normal-zero-fixture",
                ConvergencePolicyStatus::Accepted,
                0.0,
                0.0,
                max_iterations,
            )
            .unwrap(),
        }
    }

    #[test]
    fn current_normal_affine_fixture_matches_both_signed_oracles_seeds_and_modes() {
        for (force_x, force_y, expected_u, expected_normal, expected_friction) in [
            (10.0, -10.0, 7.0 / 135.0, 200.0 / 27.0, -20.0 / 9.0),
            (-10.0, -10.0, -7.0 / 165.0, 400.0 / 33.0, 40.0 / 11.0),
        ] {
            let mut physical_results = Vec::new();
            for seed in [ActiveSetState::Sticking, ActiveSetState::Sliding] {
                for mode in [
                    LinearSolveMode::SparseInteractive,
                    LinearSolveMode::DenseScrutiny,
                ] {
                    let input = coupled_normal_friction_problem(force_x, force_y, seed, 4);
                    let result = solve_active_set_frame_with_mode(&input, mode).unwrap();
                    assert!(result.converged, "seed={seed:?} mode={mode:?}");
                    assert_eq!(result.iterations.len(), 2);
                    assert_eq!(result.final_states[0].state, ActiveSetState::Sliding);
                    let final_iteration = result.iterations.last().unwrap();
                    assert_eq!(final_iteration.sparse_evidence.solver_mode, mode);
                    let applied = &final_iteration.applied_sliding_friction_forces;
                    assert_eq!(applied.len(), 1);
                    let displacement = result.displacements[node_dof_index(1, FrameDof::Ux)];
                    let normal = result.reactions[node_dof_index(1, FrameDof::Uy)];
                    let friction = result.reactions[node_dof_index(1, FrameDof::Ux)];
                    assert!((displacement - expected_u).abs() <= 1.0e-12);
                    assert!((normal - expected_normal).abs() <= 1.0e-12);
                    assert!((friction - expected_friction).abs() <= 1.0e-12);
                    assert!((applied[0].force - expected_friction).abs() <= 1.0e-12);
                    assert!((friction.abs() - 0.30 * normal.abs()).abs() <= 1.0e-12);
                    physical_results.push((displacement, normal, friction));
                }
            }
            for result in &physical_results[1..] {
                assert_eq!(*result, physical_results[0]);
            }
        }
    }

    #[test]
    fn exact_derived_normal_branch_domains_include_closed_zero_and_irrelevant_rows() {
        assert_eq!(normalized_reaction_sign(0.0), 0.0);
        assert_eq!(normalized_reaction_sign(-0.0), 0.0);
        assert!(derived_normal_branch_admissible(0.3, 1.0, 1.0, 0.0));
        assert!(derived_normal_branch_admissible(0.3, 1.0, -1.0, -0.0));
        assert!(derived_normal_branch_admissible(0.3, 1.0, 0.0, 0.0));
        assert!(!derived_normal_branch_admissible(0.3, 1.0, 0.0, 1.0));
        assert!(!derived_normal_branch_admissible(0.3, 1.0, 0.0, -1.0));
        assert!(!derived_normal_branch_admissible(0.3, 1.0, 1.0, -1.0));
        assert!(!derived_normal_branch_admissible(0.3, 1.0, -1.0, 1.0));
        assert!(derived_normal_branch_admissible(0.0, 1.0, -1.0, 12.0));
        assert!(derived_normal_branch_admissible(0.3, 0.0, -1.0, 12.0));
    }

    #[test]
    fn both_nonzero_assumed_branches_accept_exact_zero_current_normal() {
        for load_sign in [1.0, -1.0] {
            for mode in [
                LinearSolveMode::SparseInteractive,
                LinearSolveMode::DenseScrutiny,
            ] {
                let input = exact_zero_current_normal_problem(load_sign, 1);
                let first = solve_active_set_frame_with_mode(&input, mode).unwrap();
                let previous = &first.iterations[0];
                let tangential_dof = node_dof_index(1, FrameDof::Ux);
                let source_dof = node_dof_index(1, FrameDof::Uy);
                assert_eq!(
                    sliding_direction(0.0, previous.reactions[tangential_dof]),
                    load_sign
                );
                assert_eq!(previous.reactions[source_dof], load_sign);
                assert_eq!(previous.active_set.states[0].state, ActiveSetState::Sliding);

                let mut stiffness = assemble_global_stiffness_with_user_elements(
                    input.node_count,
                    &input.elements,
                    &input.user_stiffness_elements,
                )
                .unwrap();
                add_curved_bend_stiffness_contributions(
                    &mut stiffness,
                    &input.curved_bend_elements,
                );
                let boundary = active_boundary(
                    input.node_count,
                    &input.base_restrained_dofs,
                    &input.nonlinear_supports,
                    &previous.active_set.states,
                )
                .unwrap();
                let explicit = friction_normal_map(&input).unwrap();
                let derived = derived_friction_normal_map(&input).unwrap();
                let second = solve_iteration_with_sliding_friction(
                    &input,
                    &stiffness,
                    &boundary,
                    &previous.active_set.states,
                    Some(previous),
                    &explicit,
                    &derived,
                    mode,
                )
                .unwrap();
                assert!(second.derived_normal_branches_admissible);
                assert_eq!(second.reactions[source_dof], 0.0);
                assert!(second.applied_forces.is_empty());
            }
        }
    }

    #[test]
    fn derived_normal_sign_flips_retry_before_convergence_and_fail_honestly_at_cap() {
        for (force_x, force_y) in [(10.0, -1.0), (-10.0, 1.0)] {
            for mode in [
                LinearSolveMode::SparseInteractive,
                LinearSolveMode::DenseScrutiny,
            ] {
                let input =
                    coupled_normal_friction_problem(force_x, force_y, ActiveSetState::Sticking, 4);
                let result = solve_active_set_frame_with_mode(&input, mode).unwrap();
                assert!(result.converged, "force_x={force_x} mode={mode:?}");
                assert_eq!(result.iterations.len(), 3);
                assert_eq!(result.final_states[0].state, ActiveSetState::Sliding);
                let final_iteration = result.iterations.last().unwrap();
                let normal = final_iteration.reactions[node_dof_index(1, FrameDof::Uy)];
                let friction = final_iteration.reactions[node_dof_index(1, FrameDof::Ux)];
                assert!((friction.abs() - 0.30 * normal.abs()).abs() <= 1.0e-12);
            }

            let capped =
                coupled_normal_friction_problem(force_x, force_y, ActiveSetState::Sticking, 2);
            let result = solve_active_set_frame(&capped).unwrap();
            assert!(!result.converged);
            assert!(result.is_blocked());
            assert_eq!(result.iterations.len(), 2);
            assert!(result.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == SolverDiagnosticCode::NonConvergence
                    && diagnostic.message.contains("derived-normal")
            }));
        }
    }

    #[test]
    fn two_coupled_derived_normal_rows_are_order_independent() {
        for seed in [ActiveSetState::Sticking, ActiveSetState::Sliding] {
            for mode in [
                LinearSolveMode::SparseInteractive,
                LinearSolveMode::DenseScrutiny,
            ] {
                let mut ordered_results = Vec::new();
                for order in [["F-X", "F-Y"], ["F-Y", "F-X"]] {
                    let input = two_row_current_normal_problem(
                        seed,
                        -12.0,
                        -100.0,
                        [100.0, 100.0],
                        order,
                        6,
                    );
                    let result = solve_active_set_frame_with_mode(&input, mode).unwrap();
                    assert!(
                        result.converged,
                        "seed={seed:?} mode={mode:?} order={order:?}"
                    );
                    let final_iteration = result.iterations.last().unwrap();
                    assert_eq!(final_iteration.applied_sliding_friction_forces.len(), 2);
                    assert_eq!(
                        final_iteration
                            .applied_sliding_friction_forces
                            .iter()
                            .map(|force| force.support_id.as_str())
                            .collect::<Vec<_>>(),
                        order
                    );
                    let mut forces = final_iteration
                        .applied_sliding_friction_forces
                        .iter()
                        .map(|force| (force.support_id.clone(), force.force))
                        .collect::<Vec<_>>();
                    forces.sort_by(|left, right| left.0.cmp(&right.0));
                    ordered_results.push((
                        result.displacements.clone(),
                        result.reactions.clone(),
                        forces,
                    ));
                }
                let (left_displacements, left_reactions, left_forces) = &ordered_results[0];
                let (right_displacements, right_reactions, right_forces) = &ordered_results[1];
                assert!(left_displacements
                    .iter()
                    .zip(right_displacements)
                    .all(|(left, right)| (left - right).abs() <= 1.0e-12));
                assert!(left_reactions
                    .iter()
                    .zip(right_reactions)
                    .all(|(left, right)| (left - right).abs() <= 1.0e-12));
                assert_eq!(
                    left_forces.iter().map(|force| &force.0).collect::<Vec<_>>(),
                    right_forces
                        .iter()
                        .map(|force| &force.0)
                        .collect::<Vec<_>>()
                );
                assert!(left_forces
                    .iter()
                    .zip(right_forces)
                    .all(|(left, right)| (left.1 - right.1).abs() <= 1.0e-12));
            }
        }
    }

    #[test]
    fn active_signed_zero_branch_rejects_cross_coupled_nonzero_current_normal() {
        for signed_zero in [0.0, -0.0] {
            for mode in [
                LinearSolveMode::SparseInteractive,
                LinearSolveMode::DenseScrutiny,
            ] {
                let input = two_row_current_normal_problem(
                    ActiveSetState::Sliding,
                    0.0,
                    -100.0,
                    [100.0, 100.0],
                    ["F-X", "F-Y"],
                    6,
                );
                let solved = solve_active_set_frame_with_mode(&input, mode).unwrap();
                let mut previous = solved.iterations[0].clone();
                let tangential_dof = node_dof_index(1, FrameDof::Ux);
                let source_dof = node_dof_index(1, FrameDof::Uz);
                previous.reactions[source_dof] = signed_zero;
                assert_eq!(
                    sliding_direction(previous.displacements[tangential_dof], 0.0),
                    1.0
                );
                assert_eq!(previous.reactions[source_dof], 0.0);
                assert_eq!(
                    previous.reactions[source_dof].is_sign_negative(),
                    signed_zero.is_sign_negative()
                );

                let states = vec![
                    SupportStateRecord::new("F-X", ActiveSetState::Sliding),
                    SupportStateRecord::new("F-Y", ActiveSetState::Sliding),
                ];
                let mut stiffness = assemble_global_stiffness_with_user_elements(
                    input.node_count,
                    &input.elements,
                    &input.user_stiffness_elements,
                )
                .unwrap();
                add_curved_bend_stiffness_contributions(
                    &mut stiffness,
                    &input.curved_bend_elements,
                );
                let boundary = active_boundary(
                    input.node_count,
                    &input.base_restrained_dofs,
                    &input.nonlinear_supports,
                    &states,
                )
                .unwrap();
                let base = solve_linearized_system(
                    &stiffness,
                    &input.force,
                    &boundary,
                    LinearSolveMode::DenseScrutiny,
                )
                .unwrap();
                let base_reactions = reported_reactions(&base.reactions, &[]).unwrap();
                assert_eq!(base_reactions[source_dof], 0.0);

                let explicit = friction_normal_map(&input).unwrap();
                let derived = derived_friction_normal_map(&input).unwrap();
                let current = solve_iteration_with_sliding_friction(
                    &input,
                    &stiffness,
                    &boundary,
                    &states,
                    Some(&previous),
                    &explicit,
                    &derived,
                    mode,
                )
                .unwrap();
                assert!(!current
                    .applied_forces
                    .iter()
                    .any(|force| force.support_id == "F-X"));
                let coupled_force = current
                    .applied_forces
                    .iter()
                    .find(|force| force.support_id == "F-Y")
                    .unwrap();
                assert!((coupled_force.force + 1100.0 / 41.0).abs() <= 1.0e-12);
                assert_eq!(current.reactions[tangential_dof], 0.0);
                assert!((current.reactions[source_dof] - 11.0 / 41.0).abs() <= 1.0e-12);
                assert!(!current.derived_normal_branches_admissible);

                let trial_states = build_trial_states(
                    &input.nonlinear_supports,
                    &current.linearized.displacements,
                    &current.reactions,
                    &explicit,
                    &derived,
                )
                .unwrap();
                let active_set = evaluate_active_set_iteration(&ActiveSetIterationInput {
                    iteration: 2,
                    max_iterations: input.convergence.max_iterations,
                    tolerance: input.convergence.effective_tolerance(),
                    supports: input.nonlinear_supports.clone(),
                    trial_states,
                    prior_states: states,
                })
                .unwrap();
                assert!(active_set.converged);
                assert!(!active_set.is_blocked());
                let caller_converged = active_set.converged
                    && !active_set.is_blocked()
                    && current.derived_normal_branches_admissible;
                assert!(!caller_converged);
            }
        }
    }

    #[test]
    fn zero_coefficient_derived_normal_row_applies_no_force() {
        for mode in [
            LinearSolveMode::SparseInteractive,
            LinearSolveMode::DenseScrutiny,
        ] {
            let mut input =
                coupled_normal_friction_problem(10.0, -10.0, ActiveSetState::Sliding, 4);
            input.nonlinear_supports =
                vec![NonlinearSupport::friction("F", 1, FrameDof::Ux, 0.0).unwrap()];
            let result = solve_active_set_frame_with_mode(&input, mode).unwrap();
            assert!(result.converged);
            assert!(result
                .iterations
                .iter()
                .all(|iteration| iteration.applied_sliding_friction_forces.is_empty()));
        }
    }

    #[test]
    fn audit_gap_equilibrium_both_signs_seeds_and_modes() {
        for (sign, direction) in [
            (1.0, GapDirection::PositiveDisplacement),
            (-1.0, GapDirection::NegativeDisplacement),
        ] {
            for seed in [ActiveSetState::Active, ActiveSetState::Inactive] {
                for mode in [
                    LinearSolveMode::DenseScrutiny,
                    LinearSolveMode::SparseInteractive,
                ] {
                    for magnitude in [4.0, 5.0, 6.0] {
                        let support =
                            NonlinearSupport::gap("gap", 1, FrameDof::Ux, 0.05, direction).unwrap();
                        let mut input = two_node_axial_problem(
                            vec![support],
                            vec![SupportStateRecord::new("gap", seed)],
                            8,
                        );
                        input.force[6] = sign * magnitude;
                        let result = solve_active_set_frame_with_mode(&input, mode).unwrap();
                        assert!(
                            result.converged,
                            "sign={sign} seed={seed:?} mode={mode:?} F={magnitude}"
                        );
                        let expected_u = sign * if magnitude < 5.0 { 0.04 } else { 0.05 };
                        let expected_r = if magnitude > 5.0 { -sign } else { 0.0 };
                        assert!((result.displacements[6] - expected_u).abs() < 1e-14);
                        assert!((result.reactions[6] - expected_r).abs() < 1e-14);
                        assert_eq!(
                            result.final_states[0].state,
                            if magnitude < 5.0 {
                                ActiveSetState::Inactive
                            } else {
                                ActiveSetState::Active
                            }
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn audit_coupled_exact_contacts_converge_from_all_seeds() {
        for sign in [1.0, -1.0] {
            for seed1 in [ActiveSetState::Active, ActiveSetState::Inactive] {
                for seed2 in [ActiveSetState::Active, ActiveSetState::Inactive] {
                    for mode in [
                        LinearSolveMode::DenseScrutiny,
                        LinearSolveMode::SparseInteractive,
                    ] {
                        let mut input = two_node_axial_problem(vec![], vec![], 8);
                        input.node_count = 3;
                        input.elements.push(
                            FrameElement::new(
                                FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap(),
                                FrameNode::new(2, [2.0, 0.0, 0.0]).unwrap(),
                                FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap(),
                                [0.0, 1.0, 0.0],
                            )
                            .unwrap(),
                        );
                        input.force = vec![0.0; 18];
                        input.force[12] = sign * 12.5;
                        input.base_restrained_dofs.extend(13..18);
                        let direction = if sign > 0.0 {
                            GapDirection::PositiveDisplacement
                        } else {
                            GapDirection::NegativeDisplacement
                        };
                        input.nonlinear_supports = vec![
                            NonlinearSupport::gap("g1", 1, FrameDof::Ux, 0.125, direction).unwrap(),
                            NonlinearSupport::gap("g2", 2, FrameDof::Ux, 0.25, direction).unwrap(),
                        ];
                        input.initial_states = vec![
                            SupportStateRecord::new("g1", seed1),
                            SupportStateRecord::new("g2", seed2),
                        ];
                        let result = solve_active_set_frame_with_mode(&input, mode).unwrap();
                        assert!(result.converged);
                        assert_eq!(result.displacements[6], sign * 0.125);
                        assert_eq!(result.displacements[12], sign * 0.25);
                        assert_eq!(result.reactions[6], 0.0);
                        assert_eq!(result.reactions[12], 0.0);
                        assert!(result
                            .final_states
                            .iter()
                            .all(|state| state.state == ActiveSetState::Active));
                    }
                }
            }
        }
    }

    #[test]
    fn audit_literal_zero_iteration_cap_returns_error() {
        let mut input = two_node_axial_problem(vec![], vec![], 4);
        input.convergence.max_iterations = 0;
        assert!(matches!(
            solve_active_set_frame(&input),
            Err(NonlinearIntegrationError::InvalidInput { .. })
        ));
    }

    #[test]
    fn audit_duplicate_support_ids_return_error() {
        let support = NonlinearSupport::gap(
            "duplicate",
            1,
            FrameDof::Ux,
            1.0,
            GapDirection::PositiveDisplacement,
        )
        .unwrap();
        let input = two_node_axial_problem(
            vec![support.clone(), support],
            vec![SupportStateRecord::new(
                "duplicate",
                ActiveSetState::Inactive,
            )],
            4,
        );
        assert!(matches!(
            solve_active_set_frame(&input),
            Err(NonlinearIntegrationError::InvalidInput { .. })
        ));
    }

    #[test]
    fn direct_reduced_profile_entries_follow_free_dof_map() {
        let global_stiffness = vec![
            vec![4.0, 0.0, 0.0, 0.0],
            vec![0.0, 5.0, 0.0, 0.0],
            vec![2.0, 0.0, 6.0, 0.0],
            vec![0.0, 0.0, 7.0, 9.0],
        ];
        let entries = direct_reduced_profile_entries(&global_stiffness, &[0, 2, 3]).unwrap();

        assert_eq!(
            entries,
            vec![
                SymmetricMatrixEntry {
                    row: 0,
                    col: 0,
                    value: 4.0
                },
                SymmetricMatrixEntry {
                    row: 1,
                    col: 0,
                    value: 2.0
                },
                SymmetricMatrixEntry {
                    row: 1,
                    col: 1,
                    value: 6.0
                },
                SymmetricMatrixEntry {
                    row: 2,
                    col: 1,
                    value: 7.0
                },
                SymmetricMatrixEntry {
                    row: 2,
                    col: 2,
                    value: 9.0
                },
            ]
        );
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
        assert_eq!(
            result.iterations[0]
                .residuals
                .active_set_changed_support_count,
            1.0
        );
        assert_eq!(
            result.iterations[0]
                .residuals
                .max_abs_translation_delta_from_previous,
            None
        );
        assert_eq!(
            result
                .iterations
                .last()
                .unwrap()
                .residuals
                .active_set_changed_support_count,
            0.0
        );
        assert!(
            result
                .iterations
                .last()
                .unwrap()
                .residuals
                .max_abs_translation_delta_from_previous
                .unwrap()
                > 0.0
        );
        assert_eq!(
            result
                .iterations
                .last()
                .unwrap()
                .residuals
                .max_abs_free_dof_force_residual,
            0.0
        );
        assert_eq!(
            result
                .iterations
                .last()
                .unwrap()
                .residuals
                .max_abs_free_dof_work_residual,
            0.0
        );
        let sparse_evidence = &result.iterations.last().unwrap().sparse_evidence;
        assert_eq!(sparse_evidence.status, SparseEvidenceStatus::Observed);
        assert_eq!(sparse_evidence.policy_ref, "DEC-053");
        assert_eq!(
            sparse_evidence.solver_mode,
            LinearSolveMode::SparseInteractive
        );
        assert_eq!(
            sparse_evidence.solution_basis,
            "sparse_profile_direct_primary"
        );
        assert_eq!(
            sparse_evidence.assembly_basis,
            DEC050_DIRECT_REDUCED_PROFILE_ASSEMBLY_BASIS
        );
        assert_eq!(sparse_evidence.reduced_dof_count, 1);
        assert_eq!(sparse_evidence.relative_dense_sparse_solution_delta, None);
        assert_eq!(sparse_evidence.nonpositive_pivot_count, Some(0));
        assert!(result
            .limitations
            .iter()
            .any(|limitation| limitation.contains("DEC-053 sparse interactive mode")));
        assert!(!result
            .limitations
            .iter()
            .any(|limitation| limitation.contains("D-17 remains")));
    }

    #[test]
    fn dense_scrutiny_mode_keeps_sparse_parity_evidence() {
        let support_id = "NL-ONE-WAY-DENSE-SCRUTINY";
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

        let result = solve_active_set_frame_with_mode(&input, LinearSolveMode::DenseScrutiny)
            .expect("dense scrutiny solve succeeds");
        let sparse_evidence = &result.iterations.last().unwrap().sparse_evidence;

        assert_eq!(sparse_evidence.status, SparseEvidenceStatus::Observed);
        assert_eq!(sparse_evidence.policy_ref, "DEC-053");
        assert_eq!(sparse_evidence.solver_mode, LinearSolveMode::DenseScrutiny);
        assert_eq!(sparse_evidence.solution_basis, "dense_scrutiny_primary");
        assert!(
            sparse_evidence
                .relative_dense_sparse_solution_delta
                .expect("dense scrutiny records sparse parity")
                <= 1.0e-9
        );
        assert_eq!(sparse_evidence.nonpositive_pivot_count, Some(0));
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
        assert_eq!(result.iterations.len(), 1);
        assert_eq!(
            result.iterations[0]
                .residuals
                .max_abs_translation_delta_from_previous,
            None
        );
        assert_eq!(
            result.iterations[0]
                .residuals
                .max_abs_free_dof_force_residual,
            0.0
        );
        assert_eq!(
            result.iterations[0]
                .residuals
                .max_abs_free_dof_work_residual,
            0.0
        );
    }

    #[test]
    fn friction_support_slides_then_converges_through_released_frame_loop() {
        let support_id = "NL-FRICTION-SLIDE";
        let support = NonlinearSupport::friction(support_id, 1, FrameDof::Ux, 0.30).unwrap();
        let mut input = two_node_axial_problem(
            vec![support],
            vec![SupportStateRecord::new(
                support_id,
                ActiveSetState::Sticking,
            )],
            4,
        );
        input.friction_normal_reactions =
            vec![FrictionNormalReaction::new(support_id, 10.0).unwrap()];

        let result = solve_active_set_frame(&input).unwrap();

        assert!(result.converged);
        assert!(!result.is_blocked());
        assert_eq!(result.iterations.len(), 2);
        assert_eq!(
            result.iterations[0].active_set.states,
            vec![SupportStateRecord::new(support_id, ActiveSetState::Sliding)]
        );
        assert_eq!(
            result.final_states,
            vec![SupportStateRecord::new(support_id, ActiveSetState::Sliding)]
        );
        assert!(result.displacements[node_dof_index(1, FrameDof::Ux)] > 0.0);
        assert!(!result
            .iterations
            .last()
            .unwrap()
            .active_restrained_dofs
            .contains(&node_dof_index(1, FrameDof::Ux)));
    }

    fn invented_curved_bend_macro_element() -> CurvedBendMacroElement {
        let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap();
        let radius = 0.75_f64;
        let sagitta_offset = (radius * radius - 0.25).sqrt();
        let center = [0.5, -sagitta_offset, 0.0];
        CurvedBendMacroElement::new(node_i, node_j, center, 100.0, 40.0, 1.0, 1.0, 1.0, 1.5, 1.5)
            .unwrap()
    }

    #[test]
    fn curved_bend_slot_matches_linear_arc_path_when_support_stays_released() {
        // DEC-070 residual closure (TP-PMM-P1-CURVEDBEND-003): a curved-bend
        // macro-element plus a nonlinear support solves through the loop, and
        // with the one-way support staying released the result equals the
        // direct linear curved-bend solve.
        let macro_element = invented_curved_bend_macro_element();
        let slot = CurvedBendStiffnessElement::from_macro_element("curved-bend-1", &macro_element)
            .unwrap();
        let support_id = "NL-ONE-WAY-ARC-RELEASED";
        let support = NonlinearSupport::one_way(
            support_id,
            1,
            FrameDof::Uy,
            ActivationSense::PositiveReaction,
        );
        let mut force = vec![0.0; 2 * DOF_PER_NODE];
        force[node_dof_index(1, FrameDof::Uy)] = 2.0;
        let base_restrained_dofs: Vec<usize> = (0..DOF_PER_NODE).collect();
        let input = NonlinearFrameSolveInput {
            node_count: 2,
            elements: Vec::new(),
            user_stiffness_elements: Vec::new(),
            curved_bend_elements: vec![slot.clone()],
            force: force.clone(),
            base_restrained_dofs: base_restrained_dofs.clone(),
            nonlinear_supports: vec![support],
            initial_states: vec![SupportStateRecord::new(
                support_id,
                ActiveSetState::Inactive,
            )],
            friction_normal_reactions: Vec::new(),
            derived_friction_normal_reactions: Vec::new(),
            convergence: ConvergenceControl::new(
                "DEC-046-fixture-active-set-count-tightening",
                ConvergencePolicyStatus::Accepted,
                0.0,
                0.0,
                4,
            )
            .unwrap(),
        };

        let result = solve_active_set_frame(&input).unwrap();

        assert!(result.converged);
        assert!(!result.is_blocked());
        assert_eq!(
            result.final_states,
            vec![SupportStateRecord::new(
                support_id,
                ActiveSetState::Inactive
            )]
        );

        // Independent linear reference: the same 12x12 arc stiffness reduced
        // and solved directly through the frame kernel.
        let dense: DenseMatrix = slot
            .global_stiffness
            .iter()
            .map(|row| row.to_vec())
            .collect();
        let reduced =
            open_pipe_stress_frame_kernel::reduce_system(&dense, &force, &base_restrained_dofs)
                .unwrap();
        let reference = solve_dense(&reduced.stiffness, &reduced.force).unwrap();
        for (&global_dof, &expected) in reduced.free_dofs.iter().zip(reference.iter()) {
            let delta = (result.displacements[global_dof] - expected).abs();
            assert!(
                delta <= 1.0e-12,
                "nonlinear-loop arc displacement at DOF {global_dof} must match the linear curved-bend path (delta {delta})"
            );
        }
        assert!(result.displacements[node_dof_index(1, FrameDof::Uy)] > 0.0);
    }

    #[test]
    fn sliding_friction_support_applies_bounded_force_independent_of_seed() {
        // DEC-067: a sliding support applies the bounded +/- mu*N tangential
        // force opposing motion instead of a full DOF release, and the
        // converged result is the same whether sliding is seeded directly or
        // reached from a sticking transition.
        let support_id = "NL-FRICTION-BOUNDED-SLIDE";
        let mut results = Vec::new();
        for seed in [ActiveSetState::Sticking, ActiveSetState::Sliding] {
            let support = NonlinearSupport::friction(support_id, 1, FrameDof::Ux, 0.30).unwrap();
            let mut input = two_node_axial_problem(
                vec![support],
                vec![SupportStateRecord::new(support_id, seed)],
                4,
            );
            input.friction_normal_reactions =
                vec![FrictionNormalReaction::new(support_id, 10.0).unwrap()];

            let result = solve_active_set_frame(&input).unwrap();

            assert!(result.converged);
            assert_eq!(result.iterations.len(), 2);
            assert_eq!(
                result.final_states,
                vec![SupportStateRecord::new(support_id, ActiveSetState::Sliding)]
            );
            // First iterate carries no sliding force (no solved iterate to
            // orient it); the second applies the bounded -mu*N force.
            assert!(result.iterations[0]
                .applied_sliding_friction_forces
                .is_empty());
            let applied = &result.iterations[1].applied_sliding_friction_forces;
            assert_eq!(applied.len(), 1);
            assert_eq!(applied[0].support_id, support_id);
            assert_eq!(applied[0].global_dof, node_dof_index(1, FrameDof::Ux));
            assert!((applied[0].force + 3.0).abs() <= 1.0e-12);
            // Reported support reaction is the bounded tangential force, and
            // the displacement is bounded by the net (10 - 3) N drive.
            assert!((result.reactions[node_dof_index(1, FrameDof::Ux)] + 3.0).abs() <= 1.0e-12);
            let displacement = result.displacements[node_dof_index(1, FrameDof::Ux)];
            assert!((displacement - 0.07).abs() <= 1.0e-12);
            results.push((displacement, result.reactions.clone()));
        }
        assert_eq!(results[0], results[1]);
    }

    #[test]
    fn sliding_seed_at_single_iteration_cap_emits_nonconvergence_diagnostic() {
        // TP-PMM-P2-NONCONVDIAG-001: recorded TP-PMM-P2-FRICTION-001 corner.
        // A sliding seed defers first-iterate convergence so the bounded
        // sliding force can be applied, so at max_iterations == 1 the loop
        // exits converged == false while the classifier residual is zero.
        // That exit must still carry a visible NonConvergence failure
        // diagnostic instead of passing silently.
        let support_id = "NL-FRICTION-SLIDE-CAP-ONE";
        let support = NonlinearSupport::friction(support_id, 1, FrameDof::Ux, 0.30).unwrap();
        let mut input = two_node_axial_problem(
            vec![support],
            vec![SupportStateRecord::new(support_id, ActiveSetState::Sliding)],
            1,
        );
        input.friction_normal_reactions =
            vec![FrictionNormalReaction::new(support_id, 10.0).unwrap()];

        let result = solve_active_set_frame(&input).unwrap();

        assert!(!result.converged);
        assert!(result.is_blocked());
        assert_eq!(result.iterations.len(), 1);
        // The classifier itself reports a converged unchanged active set at
        // zero residual, so the diagnostic must come from the integration
        // loop's non-converged exit guard.
        assert_eq!(result.iterations[0].active_set.residual_norm, 0.0);
        assert!(result.iterations[0].active_set.converged);
        let nonconvergence: Vec<_> = result
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.code
                    == open_pipe_stress_solver_diagnostics::SolverDiagnosticCode::NonConvergence
            })
            .collect();
        assert_eq!(nonconvergence.len(), 1);
        assert_eq!(
            nonconvergence[0].severity,
            open_pipe_stress_solver_diagnostics::DiagnosticSeverity::Failure
        );
        assert!(nonconvergence[0]
            .message
            .contains("defers convergence so the bounded sliding-friction force"));
        assert!(nonconvergence[0]
            .message
            .contains(&format!("{support_id}=sliding")));
        assert!(nonconvergence[0].remediation.is_some());
    }

    #[test]
    fn sticking_friction_converged_at_single_iteration_cap_has_no_false_positive() {
        // TP-PMM-P2-NONCONVDIAG-001: a solve that genuinely converges on its
        // single allowed iteration must not gain a NonConvergence diagnostic
        // from the non-converged exit guard.
        let support_id = "NL-FRICTION-STICK-CAP-ONE";
        let support = NonlinearSupport::friction(support_id, 1, FrameDof::Ux, 0.30).unwrap();
        let mut input = two_node_axial_problem(
            vec![support],
            vec![SupportStateRecord::new(
                support_id,
                ActiveSetState::Sticking,
            )],
            1,
        );
        // mu * N = 0.30 * 40 = 12 N bounds the 10 N tangential reaction, so
        // the sticking state is confirmed unchanged on the first iterate.
        input.friction_normal_reactions =
            vec![FrictionNormalReaction::new(support_id, 40.0).unwrap()];

        let result = solve_active_set_frame(&input).unwrap();

        assert!(result.converged);
        assert!(!result.is_blocked());
        assert_eq!(result.iterations.len(), 1);
        assert!(result.diagnostics.is_empty());
        assert_eq!(
            result.final_states,
            vec![SupportStateRecord::new(
                support_id,
                ActiveSetState::Sticking
            )]
        );
    }

    #[test]
    fn friction_support_derives_normal_from_named_support_reaction() {
        let support_id = "NL-FRICTION-DERIVED-NORMAL";
        let support = NonlinearSupport::friction(support_id, 1, FrameDof::Ux, 0.30).unwrap();
        let mut input = two_node_axial_problem(
            vec![support],
            vec![SupportStateRecord::new(
                support_id,
                ActiveSetState::Sticking,
            )],
            4,
        );
        input.force[node_dof_index(1, FrameDof::Uy)] = -100.0;
        input.derived_friction_normal_reactions =
            vec![DerivedFrictionNormalReaction::from_support_reaction(
                support_id,
                1,
                FrameDof::Uy,
                "support:NORMAL-110:UY",
            )
            .unwrap()];

        let result = solve_active_set_frame(&input).unwrap();

        assert!(result.converged);
        assert_eq!(result.iterations.len(), 1);
        assert_eq!(
            result.final_states,
            vec![SupportStateRecord::new(
                support_id,
                ActiveSetState::Sticking
            )]
        );
        assert_eq!(result.reactions[node_dof_index(1, FrameDof::Uy)], 100.0);
    }

    #[test]
    fn duplicate_explicit_and_derived_friction_normal_source_is_invalid() {
        let support_id = "NL-FRICTION-DUPLICATE-NORMAL";
        let support = NonlinearSupport::friction(support_id, 1, FrameDof::Ux, 0.30).unwrap();
        let mut input = two_node_axial_problem(
            vec![support],
            vec![SupportStateRecord::new(
                support_id,
                ActiveSetState::Sticking,
            )],
            4,
        );
        input.friction_normal_reactions =
            vec![FrictionNormalReaction::new(support_id, 10.0).unwrap()];
        input.derived_friction_normal_reactions =
            vec![DerivedFrictionNormalReaction::from_support_reaction(
                support_id,
                1,
                FrameDof::Uy,
                "support:NORMAL-110:UY",
            )
            .unwrap()];

        let error = solve_active_set_frame(&input).unwrap_err();

        assert!(error.to_string().contains("is both explicit and derived"));
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
        assert_eq!(
            result
                .iterations
                .last()
                .unwrap()
                .residuals
                .max_abs_translation_delta_from_previous,
            Some(0.05)
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
        // Exactly one NonConvergence diagnostic: the state-switching support
        // at max_iterations == 1 keeps the classifier's residual-based
        // diagnostic, and the non-converged exit guard must not add a second.
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(
            result.diagnostics[0].code,
            open_pipe_stress_solver_diagnostics::SolverDiagnosticCode::NonConvergence
        );
        assert_eq!(
            result.diagnostics[0].severity,
            open_pipe_stress_solver_diagnostics::DiagnosticSeverity::Failure
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

#[cfg(test)]
mod computed_finite_audit_tests {
    use super::*;

    #[test]
    fn finite_solution_with_overflowing_reaction_or_work_is_rejected() {
        for mode in [
            LinearSolveMode::DenseScrutiny,
            LinearSolveMode::SparseInteractive,
        ] {
            // Exact prescribed u=2 gives unrepresentable reaction 2e308.
            let boundary = BoundaryState {
                dofs: vec![0],
                displacements: vec![2.0],
            };
            assert!(
                solve_linearized_system(&vec![vec![1e308]], &vec![0.0], &boundary, mode).is_err()
            );
        }
        assert!(!max_abs_free_dof_work_residual(&[1e308], &[2.0], &[0]).is_finite());
        assert!(!max_abs_delta_by_dof_group(&[1e308], &[-1e308], true).is_finite());
        // NaN accumulation used to disappear into f64::max's zero seed.
        assert!(!max_abs_entry_residual(
            1,
            &[
                SymmetricMatrixEntry {
                    row: 0,
                    col: 0,
                    value: 1e308
                },
                SymmetricMatrixEntry {
                    row: 0,
                    col: 0,
                    value: -1e308
                },
            ],
            &[2.0],
            &[0.0]
        )
        .is_finite());
    }
}

//! Deterministic solver performance and conditioning harness.
//!
//! This crate observes frame-kernel solve behavior for invented/public fixtures.
//! It does not change solver logic, select a production sparse solver, encode
//! protected standards data, or make engineering approval/compliance claims.

use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness, reduce_system, solve_dense, DenseMatrix, FrameElement,
    FrameKernelError, FrameKernelUnitBasis, FrameNode, FrameSection, UnitSystemRef, DOF_PER_NODE,
};
use open_pipe_stress_solver_diagnostics::{
    classify_condition_ratio, diagnostic_from_frame_error, diagnostic_from_sparse_error,
    diagnostics_from_factorization_report, sparse_solver_tbd_diagnostic,
    tolerance_policy_tbd_diagnostic, DiagnosticsError, SolverDiagnostic,
};
use open_pipe_stress_sparse_direct::{solve_symmetric_system, SparseDirectError};
use std::error::Error;
use std::fmt;
use std::time::Instant;

/// Identifier of the deterministic ordering algorithm used by the sparse
/// path (see `core/solver/sparse_direct` README for its determinism
/// properties).
pub const SPARSE_ORDERING_ALGORITHM_ID: &str = "reverse-cuthill-mckee-deterministic";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureProvenanceStatus {
    InventedPublic,
    PublicPermissive,
    UnknownSource,
    ProtectedSuspected,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureProvenance {
    pub source_name: String,
    pub source_location: String,
    pub redistribution_status: FixtureProvenanceStatus,
    pub review_note: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkFixture {
    pub fixture_id: String,
    pub name: String,
    pub provenance: FixtureProvenance,
    pub unit_basis: FrameKernelUnitBasis,
    pub node_count: usize,
    pub elements: Vec<FrameElement>,
    pub force: Vec<f64>,
    pub restrained_dofs: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HarnessSettings {
    pub solver_version: String,
    pub repeat_count: usize,
    pub condition_warning_threshold: Option<f64>,
    pub condition_failure_threshold: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatRunObservation {
    pub repeat_index: usize,
    pub max_abs_solution_delta_from_first: f64,
    pub max_abs_residual: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditioningObservation {
    pub matrix_dimension: usize,
    pub finite_diagonal_count: usize,
    pub zero_diagonal_count: usize,
    pub min_abs_diagonal: Option<f64>,
    pub max_abs_diagonal: Option<f64>,
    pub diagonal_condition_ratio_estimate: Option<f64>,
}

/// Sparse-path (DEC-023 in-repo skyline LDL^T solver) observations recorded
/// alongside the dense path for the same fixture. All fields are
/// measurements; no thresholds are asserted (thresholds remain governed by
/// D-04).
#[derive(Debug, Clone, PartialEq)]
pub struct SparseSolveObservation {
    pub ordering_algorithm: String,
    pub reduced_dofs: usize,
    pub original_max_half_bandwidth: usize,
    pub ordered_max_half_bandwidth: usize,
    pub original_profile_entry_count: usize,
    pub ordered_profile_entry_count: usize,
    pub min_abs_pivot: Option<f64>,
    pub max_abs_pivot: Option<f64>,
    /// Pivot-ratio conditioning proxy from the factorization report
    /// (`max |d| / min |d|`); documented proxy, not a true condition number.
    pub pivot_condition_ratio_estimate: Option<f64>,
    pub nonpositive_pivot_count: usize,
    /// Max-abs delta between the sparse solution and the dense first
    /// solution on the same reduced system; `None` when the dense solve
    /// failed and no parity oracle exists for the run.
    pub max_abs_sparse_dense_solution_delta: Option<f64>,
    pub max_abs_residual: f64,
    pub repeat_count: usize,
    pub max_abs_repeat_solution_delta: f64,
    /// Wall-clock measurement of the first sparse order+factor+solve;
    /// environment-dependent, recorded for measurement only.
    pub first_solve_elapsed_nanos: u128,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HarnessRunRecord {
    pub fixture_id: String,
    pub fixture_name: String,
    pub solver_version: String,
    pub unit_system_ref: UnitSystemRef,
    pub node_count: usize,
    pub element_count: usize,
    pub total_dofs: usize,
    pub reduced_dofs: usize,
    pub stiffness_nonzero_count: usize,
    pub reduced_stiffness_nonzero_count: usize,
    pub force_nonzero_count: usize,
    pub repeat_count: usize,
    pub max_abs_solution_delta: f64,
    pub max_abs_residual: f64,
    pub repeat_observations: Vec<RepeatRunObservation>,
    pub conditioning_observation: ConditioningObservation,
    pub condition_ratio_estimate: Option<f64>,
    /// Wall-clock measurement of the first dense solve attempt;
    /// environment-dependent, recorded for measurement only.
    pub dense_first_solve_elapsed_nanos: u128,
    pub sparse_observation: Option<SparseSolveObservation>,
    pub diagnostics: Vec<SolverDiagnostic>,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
    pub provenance_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HarnessSuiteRunRecord {
    pub suite_id: String,
    pub suite_name: String,
    pub solver_version: String,
    pub fixture_element_counts: Vec<usize>,
    pub fixture_records: Vec<HarnessRunRecord>,
    pub summary: HarnessSuiteSummary,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
    pub provenance_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HarnessSuiteSummary {
    pub requested_fixture_count: usize,
    pub completed_fixture_count: usize,
    pub total_node_count: usize,
    pub total_element_count: usize,
    pub total_dof_count: usize,
    pub total_reduced_dof_count: usize,
    pub total_stiffness_nonzero_count: usize,
    pub total_reduced_stiffness_nonzero_count: usize,
    pub total_force_nonzero_count: usize,
    pub total_repeat_observation_count: usize,
    pub records_with_condition_ratio_count: usize,
    pub records_with_diagnostics_count: usize,
    pub diagnostic_count: usize,
    pub max_reduced_dofs: usize,
    pub max_abs_solution_delta: f64,
    pub max_abs_residual: f64,
    pub records_with_sparse_observation_count: usize,
    pub total_original_profile_entry_count: usize,
    pub total_ordered_profile_entry_count: usize,
    pub max_abs_sparse_dense_solution_delta: f64,
    pub max_abs_sparse_residual: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HarnessError {
    InvalidSetting {
        name: &'static str,
        detail: &'static str,
    },
    InvalidFixtureProvenance {
        fixture_id: String,
        status: FixtureProvenanceStatus,
    },
    FrameKernel(FrameKernelError),
    Diagnostics(DiagnosticsError),
    SparseDirect(SparseDirectError),
}

impl fmt::Display for HarnessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSetting { name, detail } => write!(f, "invalid {name}: {detail}"),
            Self::InvalidFixtureProvenance { fixture_id, status } => {
                write!(
                    f,
                    "fixture {fixture_id} has unacceptable provenance status {status:?}"
                )
            }
            Self::FrameKernel(error) => write!(f, "{error}"),
            Self::Diagnostics(error) => write!(f, "{error}"),
            Self::SparseDirect(error) => write!(f, "{error}"),
        }
    }
}

impl Error for HarnessError {}

impl From<FrameKernelError> for HarnessError {
    fn from(error: FrameKernelError) -> Self {
        Self::FrameKernel(error)
    }
}

impl From<DiagnosticsError> for HarnessError {
    fn from(error: DiagnosticsError) -> Self {
        Self::Diagnostics(error)
    }
}

impl From<SparseDirectError> for HarnessError {
    fn from(error: SparseDirectError) -> Self {
        Self::SparseDirect(error)
    }
}

impl Default for HarnessSettings {
    fn default() -> Self {
        Self {
            solver_version: "TBD".to_string(),
            repeat_count: 2,
            condition_warning_threshold: None,
            condition_failure_threshold: None,
        }
    }
}

pub const DEFAULT_INVENTED_SUITE_ELEMENT_COUNTS: [usize; 4] = [1, 2, 4, 8];

pub fn invented_cantilever_chain_fixture(
    element_count: usize,
) -> Result<BenchmarkFixture, HarnessError> {
    if element_count == 0 {
        return Err(HarnessError::InvalidSetting {
            name: "element_count",
            detail: "must be at least one",
        });
    }

    let section = FrameSection::new(2.0e11, 7.7e10, 0.01, 8.0e-6, 8.0e-6, 1.6e-5)?;
    let nodes: Vec<FrameNode> = (0..=element_count)
        .map(|index| FrameNode::new(index, [index as f64, 0.0, 0.0]))
        .collect::<Result<Vec<_>, _>>()?;

    let mut elements = Vec::with_capacity(element_count);
    for index in 0..element_count {
        elements.push(FrameElement::new(
            nodes[index],
            nodes[index + 1],
            section,
            [0.0, 1.0, 0.0],
        )?);
    }

    let node_count = element_count + 1;
    let mut force = vec![0.0; node_count * DOF_PER_NODE];
    force[element_count * DOF_PER_NODE + 1] = -1.0e3;

    Ok(BenchmarkFixture {
        fixture_id: format!("invented-cantilever-chain-{element_count}"),
        name: format!("Invented cantilever chain with {element_count} frame elements"),
        provenance: FixtureProvenance {
            source_name: "OpenPipeStress invented mechanics fixture".to_string(),
            source_location: "core/solver/performance_harness".to_string(),
            redistribution_status: FixtureProvenanceStatus::InventedPublic,
            review_note: "Original synthetic fixture; no protected standards or vendor data."
                .to_string(),
        },
        unit_basis: invented_frame_unit_basis().ok_or(HarnessError::InvalidSetting {
            name: "fixture.unit_basis",
            detail: "must declare explicit unit-system and unit identifiers",
        })?,
        node_count,
        elements,
        force,
        restrained_dofs: (0..DOF_PER_NODE).collect(),
    })
}

/// Invented planar grid frame fixture (synthetic geometry only). Nodes are
/// numbered row-major; horizontal and vertical frame elements connect grid
/// neighbors; the bottom row is fully restrained and an invented lateral
/// load acts at the last top-row node. Grid fixtures give the sparse path a
/// banded-but-not-chain structure so ordering and profile observations are
/// meaningful.
pub fn invented_grid_frame_fixture(
    x_count: usize,
    y_count: usize,
) -> Result<BenchmarkFixture, HarnessError> {
    if x_count < 2 || y_count < 2 {
        return Err(HarnessError::InvalidSetting {
            name: "grid_dimensions",
            detail: "x_count and y_count must each be at least two",
        });
    }

    let section = FrameSection::new(2.0e11, 7.7e10, 0.01, 8.0e-6, 8.0e-6, 1.6e-5)?;
    let node_count = x_count * y_count;
    let mut nodes = Vec::with_capacity(node_count);
    for y_index in 0..y_count {
        for x_index in 0..x_count {
            nodes.push(FrameNode::new(
                y_index * x_count + x_index,
                [x_index as f64, y_index as f64, 0.0],
            )?);
        }
    }

    let mut elements = Vec::new();
    for y_index in 0..y_count {
        for x_index in 0..x_count {
            let index = y_index * x_count + x_index;
            if x_index + 1 < x_count {
                elements.push(FrameElement::new(
                    nodes[index],
                    nodes[index + 1],
                    section,
                    [0.0, 1.0, 0.0],
                )?);
            }
            if y_index + 1 < y_count {
                elements.push(FrameElement::new(
                    nodes[index],
                    nodes[index + x_count],
                    section,
                    [0.0, 0.0, 1.0],
                )?);
            }
        }
    }

    let mut force = vec![0.0; node_count * DOF_PER_NODE];
    force[(node_count - 1) * DOF_PER_NODE] = 1.0e3;
    let restrained_dofs: Vec<usize> = (0..x_count * DOF_PER_NODE).collect();

    Ok(BenchmarkFixture {
        fixture_id: format!("invented-grid-frame-{x_count}x{y_count}"),
        name: format!("Invented planar grid frame with {x_count} by {y_count} nodes"),
        provenance: FixtureProvenance {
            source_name: "OpenPipeStress invented mechanics fixture".to_string(),
            source_location: "core/solver/performance_harness".to_string(),
            redistribution_status: FixtureProvenanceStatus::InventedPublic,
            review_note: "Original synthetic fixture; no protected standards or vendor data."
                .to_string(),
        },
        unit_basis: invented_frame_unit_basis().ok_or(HarnessError::InvalidSetting {
            name: "fixture.unit_basis",
            detail: "must declare explicit unit-system and unit identifiers",
        })?,
        node_count,
        elements,
        force,
        restrained_dofs,
    })
}

pub fn run_default_invented_fixture_suite(
    settings: &HarnessSettings,
) -> Result<HarnessSuiteRunRecord, HarnessError> {
    run_invented_fixture_suite(&DEFAULT_INVENTED_SUITE_ELEMENT_COUNTS, settings)
}

pub fn run_invented_fixture_suite(
    fixture_element_counts: &[usize],
    settings: &HarnessSettings,
) -> Result<HarnessSuiteRunRecord, HarnessError> {
    validate_suite_fixture_sizes(fixture_element_counts)?;
    validate_settings(settings)?;

    let mut fixture_records = Vec::with_capacity(fixture_element_counts.len());
    for element_count in fixture_element_counts {
        let fixture = invented_cantilever_chain_fixture(*element_count)?;
        fixture_records.push(run_fixture_repeat(&fixture, settings)?);
    }

    Ok(suite_record(
        fixture_element_counts,
        settings,
        fixture_records,
    ))
}

/// Runs the dense path and the DEC-023 sparse skyline path on the same
/// reduced system and records both. Diagnostic ordering in the record is
/// fixed and deterministic:
/// 1. sparse-solver adoption-status diagnostic (`SparseSolverTbd` code),
/// 2. tolerance-policy `TBD` diagnostic,
/// 3. dense diagonal conditioning classification (when thresholds are set),
/// 4. dense solve failure diagnostic (when the dense solve fails),
/// 5. sparse factorization-report diagnostics (nonpositive pivots),
/// 6. sparse pivot-ratio conditioning classification (when thresholds are
///    set),
/// 7. sparse solve failure diagnostic (when the sparse path fails).
pub fn run_fixture_repeat(
    fixture: &BenchmarkFixture,
    settings: &HarnessSettings,
) -> Result<HarnessRunRecord, HarnessError> {
    validate_fixture(fixture)?;
    validate_settings(settings)?;

    let stiffness = assemble_global_stiffness(fixture.node_count, &fixture.elements)?;
    let reduced = reduce_system(&stiffness, &fixture.force, &fixture.restrained_dofs)?;

    let mut diagnostics = vec![
        sparse_solver_tbd_diagnostic(),
        tolerance_policy_tbd_diagnostic(),
    ];
    let conditioning_observation = observe_diagonal_conditioning(&reduced.stiffness);
    let condition_ratio_estimate = conditioning_observation.diagonal_condition_ratio_estimate;
    if let (Some(warning), Some(failure), Some(ratio)) = (
        settings.condition_warning_threshold,
        settings.condition_failure_threshold,
        condition_ratio_estimate,
    ) {
        if let Some(diagnostic) = classify_condition_ratio(ratio, warning, failure)? {
            diagnostics.push(diagnostic);
        }
    }

    let dense_solve_started = Instant::now();
    let dense_first_result = solve_dense(&reduced.stiffness, &reduced.force);
    let dense_first_solve_elapsed_nanos = dense_solve_started.elapsed().as_nanos();

    let first_solution = match dense_first_result {
        Ok(solution) => solution,
        Err(error) => {
            diagnostics.push(diagnostic_from_frame_error(&error));
            let (sparse_observation, sparse_diagnostics) =
                observe_sparse_path(&reduced.stiffness, &reduced.force, settings, None)?;
            diagnostics.extend(sparse_diagnostics);
            let force_nonzero_count = reduced
                .force
                .iter()
                .filter(|value| value.abs() > 0.0)
                .count();
            return Ok(run_record(
                fixture,
                settings,
                &stiffness,
                &reduced.stiffness,
                force_nonzero_count,
                0.0,
                0.0,
                Vec::new(),
                conditioning_observation,
                condition_ratio_estimate,
                dense_first_solve_elapsed_nanos,
                sparse_observation,
                diagnostics,
            ));
        }
    };

    let mut max_abs_solution_delta: f64 = 0.0;
    let mut repeat_observations = vec![RepeatRunObservation {
        repeat_index: 0,
        max_abs_solution_delta_from_first: 0.0,
        max_abs_residual: max_abs_residual(&reduced.stiffness, &first_solution, &reduced.force),
    }];
    for _ in 1..settings.repeat_count {
        let repeat_index = repeat_observations.len();
        let solution = solve_dense(&reduced.stiffness, &reduced.force)?;
        let repeat_delta = max_abs_delta(&first_solution, &solution);
        max_abs_solution_delta = max_abs_solution_delta.max(repeat_delta);
        repeat_observations.push(RepeatRunObservation {
            repeat_index,
            max_abs_solution_delta_from_first: repeat_delta,
            max_abs_residual: max_abs_residual(&reduced.stiffness, &solution, &reduced.force),
        });
    }

    let max_abs_residual = repeat_observations
        .iter()
        .map(|observation| observation.max_abs_residual)
        .fold(0.0, f64::max);

    let (sparse_observation, sparse_diagnostics) = observe_sparse_path(
        &reduced.stiffness,
        &reduced.force,
        settings,
        Some(&first_solution),
    )?;
    diagnostics.extend(sparse_diagnostics);

    Ok(run_record(
        fixture,
        settings,
        &stiffness,
        &reduced.stiffness,
        reduced
            .force
            .iter()
            .filter(|value| value.abs() > 0.0)
            .count(),
        max_abs_solution_delta,
        max_abs_residual,
        repeat_observations,
        conditioning_observation,
        condition_ratio_estimate,
        dense_first_solve_elapsed_nanos,
        sparse_observation,
        diagnostics,
    ))
}

/// Measures the DEC-023 sparse skyline path on the reduced system. Returns
/// the observation (when the sparse solve completes) and the sparse-path
/// diagnostics in deterministic order: factorization-report diagnostics,
/// then pivot-ratio conditioning classification (when thresholds are set),
/// then a failure diagnostic when the path fails.
fn observe_sparse_path(
    reduced_stiffness: &DenseMatrix,
    reduced_force: &[f64],
    settings: &HarnessSettings,
    dense_first_solution: Option<&[f64]>,
) -> Result<(Option<SparseSolveObservation>, Vec<SolverDiagnostic>), HarnessError> {
    let started = Instant::now();
    let first = match solve_symmetric_system(reduced_stiffness, reduced_force) {
        Ok(result) => result,
        Err(error) => {
            return Ok((None, vec![diagnostic_from_sparse_error(&error)]));
        }
    };
    let first_solve_elapsed_nanos = started.elapsed().as_nanos();

    let mut diagnostics = diagnostics_from_factorization_report(&first.factorization);
    if let (Some(warning), Some(failure), Some(ratio)) = (
        settings.condition_warning_threshold,
        settings.condition_failure_threshold,
        first.factorization.pivot_condition_ratio_estimate,
    ) {
        if let Some(diagnostic) = classify_condition_ratio(ratio, warning, failure)? {
            diagnostics.push(diagnostic);
        }
    }

    let mut max_abs_repeat_solution_delta: f64 = 0.0;
    for _ in 1..settings.repeat_count {
        let repeat = solve_symmetric_system(reduced_stiffness, reduced_force)?;
        max_abs_repeat_solution_delta =
            max_abs_repeat_solution_delta.max(max_abs_delta(&first.solution, &repeat.solution));
    }

    let observation = SparseSolveObservation {
        ordering_algorithm: SPARSE_ORDERING_ALGORITHM_ID.to_string(),
        reduced_dofs: first.solution.len(),
        original_max_half_bandwidth: first.original_max_half_bandwidth,
        ordered_max_half_bandwidth: first.ordered_max_half_bandwidth,
        original_profile_entry_count: first.original_profile_entry_count,
        ordered_profile_entry_count: first.ordered_profile_entry_count,
        min_abs_pivot: first.factorization.min_abs_pivot,
        max_abs_pivot: first.factorization.max_abs_pivot,
        pivot_condition_ratio_estimate: first.factorization.pivot_condition_ratio_estimate,
        nonpositive_pivot_count: first.factorization.nonpositive_pivot_count,
        max_abs_sparse_dense_solution_delta: dense_first_solution
            .map(|dense| max_abs_delta(dense, &first.solution)),
        max_abs_residual: max_abs_residual(reduced_stiffness, &first.solution, reduced_force),
        repeat_count: settings.repeat_count,
        max_abs_repeat_solution_delta,
        first_solve_elapsed_nanos,
    };

    Ok((Some(observation), diagnostics))
}

fn suite_record(
    fixture_element_counts: &[usize],
    settings: &HarnessSettings,
    fixture_records: Vec<HarnessRunRecord>,
) -> HarnessSuiteRunRecord {
    let summary = suite_summary(fixture_element_counts.len(), &fixture_records);
    let suite_size_label = fixture_element_counts
        .iter()
        .map(|count| count.to_string())
        .collect::<Vec<_>>()
        .join("-");
    let provenance_notes = fixture_records
        .iter()
        .flat_map(|record| record.provenance_notes.iter().cloned())
        .collect();

    HarnessSuiteRunRecord {
        suite_id: format!("invented-cantilever-chain-suite-{suite_size_label}"),
        suite_name: "Invented cantilever-chain deterministic suite".to_string(),
        solver_version: settings.solver_version.clone(),
        fixture_element_counts: fixture_element_counts.to_vec(),
        fixture_records,
        summary,
        assumptions: vec![
            "suite uses only invented cantilever-chain fixtures".to_string(),
            "suite summary counts aggregate deterministic fixture records without timing thresholds"
                .to_string(),
            "single-fixture records remain the authority for per-fixture matrix and diagnostic observations"
                .to_string(),
        ],
        limitations: vec![
            "fixture sizes are explicit invented coverage points, not approved practical-size bands"
                .to_string(),
            "DEC-023 selects the sparse skyline solver and DEC-050 binds it as a live evidence lane; profile-direct assembly and default sparse promotion remain follow-on work".to_string(),
            "release timing, memory, conditioning, and CI thresholds remain TBD".to_string(),
        ],
        provenance_notes,
    }
}

fn suite_summary(
    requested_fixture_count: usize,
    fixture_records: &[HarnessRunRecord],
) -> HarnessSuiteSummary {
    HarnessSuiteSummary {
        requested_fixture_count,
        completed_fixture_count: fixture_records.len(),
        total_node_count: fixture_records.iter().map(|record| record.node_count).sum(),
        total_element_count: fixture_records
            .iter()
            .map(|record| record.element_count)
            .sum(),
        total_dof_count: fixture_records.iter().map(|record| record.total_dofs).sum(),
        total_reduced_dof_count: fixture_records
            .iter()
            .map(|record| record.reduced_dofs)
            .sum(),
        total_stiffness_nonzero_count: fixture_records
            .iter()
            .map(|record| record.stiffness_nonzero_count)
            .sum(),
        total_reduced_stiffness_nonzero_count: fixture_records
            .iter()
            .map(|record| record.reduced_stiffness_nonzero_count)
            .sum(),
        total_force_nonzero_count: fixture_records
            .iter()
            .map(|record| record.force_nonzero_count)
            .sum(),
        total_repeat_observation_count: fixture_records
            .iter()
            .map(|record| record.repeat_observations.len())
            .sum(),
        records_with_condition_ratio_count: fixture_records
            .iter()
            .filter(|record| record.condition_ratio_estimate.is_some())
            .count(),
        records_with_diagnostics_count: fixture_records
            .iter()
            .filter(|record| !record.diagnostics.is_empty())
            .count(),
        diagnostic_count: fixture_records
            .iter()
            .map(|record| record.diagnostics.len())
            .sum(),
        max_reduced_dofs: fixture_records
            .iter()
            .map(|record| record.reduced_dofs)
            .max()
            .unwrap_or_default(),
        max_abs_solution_delta: fixture_records
            .iter()
            .map(|record| record.max_abs_solution_delta)
            .fold(0.0, f64::max),
        max_abs_residual: fixture_records
            .iter()
            .map(|record| record.max_abs_residual)
            .fold(0.0, f64::max),
        records_with_sparse_observation_count: fixture_records
            .iter()
            .filter(|record| record.sparse_observation.is_some())
            .count(),
        total_original_profile_entry_count: fixture_records
            .iter()
            .filter_map(|record| record.sparse_observation.as_ref())
            .map(|observation| observation.original_profile_entry_count)
            .sum(),
        total_ordered_profile_entry_count: fixture_records
            .iter()
            .filter_map(|record| record.sparse_observation.as_ref())
            .map(|observation| observation.ordered_profile_entry_count)
            .sum(),
        max_abs_sparse_dense_solution_delta: fixture_records
            .iter()
            .filter_map(|record| record.sparse_observation.as_ref())
            .filter_map(|observation| observation.max_abs_sparse_dense_solution_delta)
            .fold(0.0, f64::max),
        max_abs_sparse_residual: fixture_records
            .iter()
            .filter_map(|record| record.sparse_observation.as_ref())
            .map(|observation| observation.max_abs_residual)
            .fold(0.0, f64::max),
    }
}

fn run_record(
    fixture: &BenchmarkFixture,
    settings: &HarnessSettings,
    stiffness: &DenseMatrix,
    reduced_stiffness: &DenseMatrix,
    force_nonzero_count: usize,
    max_abs_solution_delta: f64,
    max_abs_residual: f64,
    repeat_observations: Vec<RepeatRunObservation>,
    conditioning_observation: ConditioningObservation,
    condition_ratio_estimate: Option<f64>,
    dense_first_solve_elapsed_nanos: u128,
    sparse_observation: Option<SparseSolveObservation>,
    diagnostics: Vec<SolverDiagnostic>,
) -> HarnessRunRecord {
    HarnessRunRecord {
        fixture_id: fixture.fixture_id.clone(),
        fixture_name: fixture.name.clone(),
        solver_version: settings.solver_version.clone(),
        unit_system_ref: fixture.unit_basis.unit_system_ref.clone(),
        node_count: fixture.node_count,
        element_count: fixture.elements.len(),
        total_dofs: fixture.node_count * DOF_PER_NODE,
        reduced_dofs: reduced_stiffness.len(),
        stiffness_nonzero_count: nonzero_count(stiffness),
        reduced_stiffness_nonzero_count: nonzero_count(reduced_stiffness),
        force_nonzero_count,
        repeat_count: settings.repeat_count,
        max_abs_solution_delta,
        max_abs_residual,
        repeat_observations,
        conditioning_observation,
        condition_ratio_estimate,
        dense_first_solve_elapsed_nanos,
        sparse_observation,
        diagnostics,
        assumptions: vec![
            "fixture uses invented mechanics quantities".to_string(),
            "fixture unit identifiers declare the calculation basis without conversion constants"
                .to_string(),
            "frame-kernel dense solve path serves as the parity oracle for the DEC-023 sparse skyline path"
                .to_string(),
            "sparse-path observations use the in-repo skyline LDLT solver (core/solver/sparse_direct) selected by DEC-023"
                .to_string(),
        ],
        limitations: vec![
            "DEC-023 selects the sparse skyline solver and DEC-050 binds it as a live evidence lane; profile-direct assembly and default sparse promotion remain follow-on work".to_string(),
            "release timing, memory, and conditioning thresholds remain TBD".to_string(),
            "elapsed-time observations are environment-dependent measurements, not asserted thresholds"
                .to_string(),
            "hardware-normalized performance gates are not claimed".to_string(),
        ],
        provenance_notes: vec![fixture.provenance.review_note.clone()],
    }
}

fn validate_fixture(fixture: &BenchmarkFixture) -> Result<(), HarnessError> {
    match fixture.provenance.redistribution_status {
        FixtureProvenanceStatus::InventedPublic | FixtureProvenanceStatus::PublicPermissive => {}
        status => {
            return Err(HarnessError::InvalidFixtureProvenance {
                fixture_id: fixture.fixture_id.clone(),
                status,
            });
        }
    }

    if fixture.force.len() != fixture.node_count * DOF_PER_NODE {
        return Err(HarnessError::InvalidSetting {
            name: "fixture.force",
            detail: "length must match node_count * DOF_PER_NODE",
        });
    }

    if !fixture.unit_basis.has_expected_dimensions() {
        return Err(HarnessError::InvalidSetting {
            name: "fixture.unit_basis",
            detail: "must match frame-kernel canonical dimensions",
        });
    }

    Ok(())
}

fn validate_suite_fixture_sizes(fixture_element_counts: &[usize]) -> Result<(), HarnessError> {
    if fixture_element_counts.is_empty() {
        return Err(HarnessError::InvalidSetting {
            name: "fixture_element_counts",
            detail: "must contain at least one explicit invented size",
        });
    }

    Ok(())
}

fn invented_frame_unit_basis() -> Option<FrameKernelUnitBasis> {
    FrameKernelUnitBasis::from_unit_ids(
        "invented-frame-fixture-units",
        "invented-length",
        "invented-force",
        "invented-moment",
        "invented-stress",
        "invented-area",
        "invented-second-moment-area",
        "invented-displacement",
        "invented-rotation",
    )
}

fn validate_settings(settings: &HarnessSettings) -> Result<(), HarnessError> {
    if settings.repeat_count == 0 {
        return Err(HarnessError::InvalidSetting {
            name: "repeat_count",
            detail: "must be at least one",
        });
    }

    match (
        settings.condition_warning_threshold,
        settings.condition_failure_threshold,
    ) {
        (Some(_), Some(_)) | (None, None) => Ok(()),
        _ => Err(HarnessError::InvalidSetting {
            name: "condition thresholds",
            detail: "warning and failure thresholds must be supplied together",
        }),
    }
}

fn observe_diagonal_conditioning(matrix: &DenseMatrix) -> ConditioningObservation {
    let mut finite_diagonal_count = 0;
    let mut zero_diagonal_count = 0;
    let mut min_diag = f64::INFINITY;
    let mut max_diag: f64 = 0.0;

    for (index, row) in matrix.iter().enumerate() {
        if let Some(value) = row.get(index) {
            let abs_value = value.abs();
            if abs_value.is_finite() {
                finite_diagonal_count += 1;
                if abs_value > 0.0 {
                    min_diag = min_diag.min(abs_value);
                    max_diag = max_diag.max(abs_value);
                } else {
                    zero_diagonal_count += 1;
                }
            }
        }
    }

    let diagonal_condition_ratio_estimate = if min_diag.is_finite() && min_diag > 0.0 {
        Some(max_diag / min_diag)
    } else {
        None
    };

    ConditioningObservation {
        matrix_dimension: matrix.len(),
        finite_diagonal_count,
        zero_diagonal_count,
        min_abs_diagonal: min_diag.is_finite().then_some(min_diag),
        max_abs_diagonal: (max_diag > 0.0).then_some(max_diag),
        diagonal_condition_ratio_estimate,
    }
}

fn nonzero_count(matrix: &DenseMatrix) -> usize {
    matrix
        .iter()
        .flat_map(|row| row.iter())
        .filter(|value| value.abs() > 0.0)
        .count()
}

fn max_abs_delta(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f64::max)
}

fn max_abs_residual(matrix: &DenseMatrix, solution: &[f64], force: &[f64]) -> f64 {
    matrix
        .iter()
        .zip(force.iter())
        .map(|(row, rhs)| {
            let lhs: f64 = row
                .iter()
                .zip(solution.iter())
                .map(|(stiffness, displacement)| stiffness * displacement)
                .sum();
            (lhs - rhs).abs()
        })
        .fold(0.0, f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_solver_diagnostics::SolverDiagnosticCode;

    #[test]
    fn invented_fixture_produces_deterministic_repeat_record() {
        let fixture = invented_cantilever_chain_fixture(4).unwrap();
        let settings = HarnessSettings {
            solver_version: "test-solver".to_string(),
            repeat_count: 3,
            ..HarnessSettings::default()
        };

        let record = run_fixture_repeat(&fixture, &settings).unwrap();

        assert_eq!(record.fixture_id, "invented-cantilever-chain-4");
        assert_eq!(
            record.unit_system_ref.unit_system_id,
            "invented-frame-fixture-units"
        );
        assert_eq!(record.node_count, 5);
        assert_eq!(record.element_count, 4);
        assert_eq!(record.repeat_count, 3);
        assert_eq!(record.repeat_observations.len(), 3);
        assert_eq!(record.repeat_observations[0].repeat_index, 0);
        assert_eq!(record.repeat_observations[1].repeat_index, 1);
        assert_eq!(record.repeat_observations[2].repeat_index, 2);
        assert_eq!(record.max_abs_solution_delta, 0.0);
        assert!(record.max_abs_residual < 1.0e-6);
        assert!(record
            .repeat_observations
            .iter()
            .all(|observation| observation.max_abs_residual < 1.0e-6));
        assert!(record.stiffness_nonzero_count > record.reduced_stiffness_nonzero_count);
        assert!(record
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == SolverDiagnosticCode::SparseSolverTbd));
        assert!(record
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == SolverDiagnosticCode::TolerancePolicyTbd));
        assert!(record.assumptions.iter().any(|assumption| {
            assumption.contains("unit identifiers declare the calculation basis")
        }));
    }

    #[test]
    fn invented_suite_preserves_fixture_records_and_summary_counts() {
        let settings = HarnessSettings {
            solver_version: "suite-test-solver".to_string(),
            repeat_count: 3,
            ..HarnessSettings::default()
        };
        let suite = run_invented_fixture_suite(&[1, 3, 5], &settings).unwrap();

        assert_eq!(suite.suite_id, "invented-cantilever-chain-suite-1-3-5");
        assert_eq!(suite.solver_version, "suite-test-solver");
        assert_eq!(suite.fixture_element_counts, vec![1, 3, 5]);
        assert_eq!(suite.fixture_records.len(), 3);
        assert_eq!(suite.summary.requested_fixture_count, 3);
        assert_eq!(suite.summary.completed_fixture_count, 3);
        assert_eq!(suite.summary.total_element_count, 9);
        assert_eq!(suite.summary.total_node_count, 12);
        assert_eq!(suite.summary.total_repeat_observation_count, 9);
        assert_eq!(suite.summary.records_with_condition_ratio_count, 3);
        assert_eq!(suite.summary.records_with_diagnostics_count, 3);
        assert_eq!(
            suite.summary.diagnostic_count,
            suite
                .fixture_records
                .iter()
                .map(|record| record.diagnostics.len())
                .sum()
        );
        assert_eq!(
            suite.summary.total_stiffness_nonzero_count,
            suite
                .fixture_records
                .iter()
                .map(|record| record.stiffness_nonzero_count)
                .sum()
        );
        assert_eq!(
            suite.summary.total_reduced_stiffness_nonzero_count,
            suite
                .fixture_records
                .iter()
                .map(|record| record.reduced_stiffness_nonzero_count)
                .sum()
        );
        assert_eq!(
            suite.summary.total_force_nonzero_count,
            suite
                .fixture_records
                .iter()
                .map(|record| record.force_nonzero_count)
                .sum()
        );
        assert_eq!(
            suite.summary.max_reduced_dofs,
            suite
                .fixture_records
                .iter()
                .map(|record| record.reduced_dofs)
                .max()
                .unwrap()
        );
        assert_eq!(suite.summary.max_abs_solution_delta, 0.0);
        assert!(suite.summary.max_abs_residual < 1.0e-6);
        assert!(suite.assumptions.iter().any(|assumption| {
            assumption.contains("single-fixture records remain the authority")
        }));
        assert!(suite
            .limitations
            .iter()
            .any(|limitation| limitation.contains("explicit invented coverage points")));
        assert_eq!(suite.provenance_notes.len(), 3);

        for (record, expected_element_count) in suite.fixture_records.iter().zip([1, 3, 5]) {
            assert_eq!(
                record.fixture_id,
                format!("invented-cantilever-chain-{expected_element_count}")
            );
            assert_eq!(record.element_count, expected_element_count);
            assert_eq!(record.repeat_observations.len(), 3);
            assert!(record.reduced_stiffness_nonzero_count > 0);
            assert_eq!(
                record.conditioning_observation.matrix_dimension,
                record.reduced_dofs
            );
            assert!(record.condition_ratio_estimate.is_some());
            assert!(!record.diagnostics.is_empty());
            assert!(!record.assumptions.is_empty());
            assert!(!record.limitations.is_empty());
            assert!(!record.provenance_notes.is_empty());
        }
    }

    #[test]
    fn default_invented_suite_uses_declared_fixture_sizes() {
        let suite = run_default_invented_fixture_suite(&HarnessSettings::default()).unwrap();

        assert_eq!(
            DEFAULT_INVENTED_SUITE_ELEMENT_COUNTS,
            [1usize, 2usize, 4usize, 8usize]
        );
        assert_eq!(
            suite.fixture_element_counts,
            DEFAULT_INVENTED_SUITE_ELEMENT_COUNTS.to_vec()
        );
        assert_eq!(
            suite.summary.requested_fixture_count,
            DEFAULT_INVENTED_SUITE_ELEMENT_COUNTS.len()
        );
        assert_eq!(
            suite.summary.completed_fixture_count,
            DEFAULT_INVENTED_SUITE_ELEMENT_COUNTS.len()
        );
        assert_eq!(
            suite
                .fixture_records
                .iter()
                .map(|record| record.element_count)
                .collect::<Vec<_>>(),
            DEFAULT_INVENTED_SUITE_ELEMENT_COUNTS.to_vec()
        );
    }

    #[test]
    fn suite_rejects_empty_fixture_size_list() {
        let error = run_invented_fixture_suite(&[], &HarnessSettings::default()).unwrap_err();

        assert_eq!(
            error,
            HarnessError::InvalidSetting {
                name: "fixture_element_counts",
                detail: "must contain at least one explicit invented size"
            }
        );
    }

    #[test]
    fn suite_rejects_zero_element_fixture_size() {
        let error = run_invented_fixture_suite(&[1, 0], &HarnessSettings::default()).unwrap_err();

        assert_eq!(
            error,
            HarnessError::InvalidSetting {
                name: "element_count",
                detail: "must be at least one"
            }
        );
    }

    #[test]
    fn conditioning_observation_is_recorded_without_threshold_policy() {
        let fixture = invented_cantilever_chain_fixture(3).unwrap();
        let record = run_fixture_repeat(&fixture, &HarnessSettings::default()).unwrap();

        assert_eq!(
            record.conditioning_observation.matrix_dimension,
            record.reduced_dofs
        );
        assert_eq!(
            record.conditioning_observation.finite_diagonal_count,
            record.reduced_dofs
        );
        assert_eq!(record.conditioning_observation.zero_diagonal_count, 0);
        assert!(record.conditioning_observation.min_abs_diagonal.unwrap() > 0.0);
        assert_eq!(
            record.condition_ratio_estimate,
            record
                .conditioning_observation
                .diagonal_condition_ratio_estimate
        );
        assert!(record.condition_ratio_estimate.unwrap() >= 1.0);
        assert!(!record.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == SolverDiagnosticCode::ConditioningFailure
                || diagnostic.code == SolverDiagnosticCode::IllConditionedSystem
        }));
        assert!(record
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == SolverDiagnosticCode::TolerancePolicyTbd));
    }

    #[test]
    fn condition_thresholds_add_diagnostic_without_release_claim() {
        let fixture = invented_cantilever_chain_fixture(2).unwrap();
        let settings = HarnessSettings {
            solver_version: "test-solver".to_string(),
            repeat_count: 2,
            condition_warning_threshold: Some(1.0),
            condition_failure_threshold: Some(1.0),
        };

        let record = run_fixture_repeat(&fixture, &settings).unwrap();

        assert!(record.condition_ratio_estimate.unwrap() >= 1.0);
        assert!(record.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == SolverDiagnosticCode::ConditioningFailure
                || diagnostic.code == SolverDiagnosticCode::IllConditionedSystem
        }));
        assert!(record
            .limitations
            .iter()
            .any(|limitation| limitation.contains("thresholds remain TBD")));
    }

    #[test]
    fn singular_repeat_attempt_records_solver_diagnostic() {
        let mut fixture = invented_cantilever_chain_fixture(1).unwrap();
        fixture.restrained_dofs = vec![0];

        let record = run_fixture_repeat(&fixture, &HarnessSettings::default()).unwrap();

        assert_eq!(record.repeat_observations.len(), 0);
        assert_eq!(record.max_abs_solution_delta, 0.0);
        assert_eq!(record.max_abs_residual, 0.0);
        assert_eq!(
            record.conditioning_observation.matrix_dimension,
            record.reduced_dofs
        );
        assert!(record
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == SolverDiagnosticCode::SingularSystem));
        assert!(record
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == SolverDiagnosticCode::SparseSolverTbd));
    }

    #[test]
    fn singular_fixture_records_located_sparse_singular_diagnostic() {
        let mut fixture = invented_cantilever_chain_fixture(1).unwrap();
        fixture.restrained_dofs = vec![0];

        let record = run_fixture_repeat(&fixture, &HarnessSettings::default()).unwrap();

        // The sparse path fails before producing an observation, and its
        // singular-pivot diagnostic carries both the ordered location and the
        // original reduced DOF index.
        assert!(record.sparse_observation.is_none());
        let sparse_singular = record
            .diagnostics
            .iter()
            .find(|diagnostic| {
                diagnostic.code == SolverDiagnosticCode::SingularSystem
                    && diagnostic.message.contains("sparse profile factorization")
            })
            .expect("sparse singular diagnostic must be recorded");
        assert!(sparse_singular.message.contains("ordered index 2"));
        assert!(sparse_singular.message.contains("original reduced index 2"));
        assert_eq!(
            sparse_singular.affected_ref.as_deref(),
            Some("reduced-dof:2")
        );
    }

    #[test]
    fn sparse_path_parity_is_observed_for_invented_chain_fixture() {
        let fixture = invented_cantilever_chain_fixture(4).unwrap();
        let settings = HarnessSettings {
            solver_version: "test-solver".to_string(),
            repeat_count: 3,
            ..HarnessSettings::default()
        };

        let record = run_fixture_repeat(&fixture, &settings).unwrap();
        let observation = record.sparse_observation.as_ref().unwrap();

        // Independent dense reference for the parity scale.
        let stiffness = assemble_global_stiffness(fixture.node_count, &fixture.elements).unwrap();
        let reduced = reduce_system(&stiffness, &fixture.force, &fixture.restrained_dofs).unwrap();
        let dense_solution = solve_dense(&reduced.stiffness, &reduced.force).unwrap();
        let scale = dense_solution
            .iter()
            .map(|value| value.abs())
            .fold(0.0, f64::max);

        assert_eq!(observation.ordering_algorithm, SPARSE_ORDERING_ALGORITHM_ID);
        assert_eq!(observation.reduced_dofs, record.reduced_dofs);
        assert_eq!(observation.repeat_count, 3);
        // Repeat sparse solves are deterministic.
        assert_eq!(observation.max_abs_repeat_solution_delta, 0.0);
        // Parity tolerance basis: DEC-026 (SOFTWARE_DECOMP.md §12) seeds the
        // analytic reference-result class at 1.0e-9 relative; the seed is
        // scaled by the dense reference solution magnitude rather than
        // inventing a new tolerance.
        assert!(scale > 0.0);
        assert!(observation.max_abs_sparse_dense_solution_delta.unwrap() <= 1.0e-9 * scale);
        assert!(observation.max_abs_residual < 1.0e-6);
        assert_eq!(observation.nonpositive_pivot_count, 0);
        assert!(observation.pivot_condition_ratio_estimate.unwrap() >= 1.0);
        // Chain numbering is already contiguous, so ordering must not grow
        // the profile.
        assert!(
            observation.ordered_profile_entry_count <= observation.original_profile_entry_count
        );
        assert!(observation.ordered_max_half_bandwidth <= observation.original_max_half_bandwidth);
    }

    #[test]
    fn grid_fixture_supports_sparse_measurement_alongside_dense() {
        let fixture = invented_grid_frame_fixture(4, 3).unwrap();

        assert_eq!(fixture.fixture_id, "invented-grid-frame-4x3");
        assert_eq!(fixture.node_count, 12);
        // Horizontal: (4 - 1) * 3 = 9; vertical: 4 * (3 - 1) = 8.
        assert_eq!(fixture.elements.len(), 17);
        assert_eq!(fixture.restrained_dofs.len(), 4 * DOF_PER_NODE);

        let record = run_fixture_repeat(&fixture, &HarnessSettings::default()).unwrap();
        let observation = record.sparse_observation.as_ref().unwrap();

        assert_eq!(record.reduced_dofs, 8 * DOF_PER_NODE);
        assert_eq!(observation.reduced_dofs, record.reduced_dofs);
        assert!(observation.ordered_profile_entry_count > 0);
        assert!(observation.max_abs_sparse_dense_solution_delta.is_some());
        assert_eq!(observation.nonpositive_pivot_count, 0);
        assert!(record
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != SolverDiagnosticCode::NonPositivePivot));
        assert!(record
            .limitations
            .iter()
            .any(|limitation| limitation.contains("DEC-023")));
    }

    #[test]
    fn larger_generated_banded_grid_parity_stays_within_dec026_seed() {
        // Larger generated banded model (invented/synthetic geometry only):
        // 6 x 8 grid, 48 nodes, 288 total DOFs, 252 reduced DOFs.
        let fixture = invented_grid_frame_fixture(6, 8).unwrap();
        let record = run_fixture_repeat(&fixture, &HarnessSettings::default()).unwrap();
        let observation = record.sparse_observation.as_ref().unwrap();

        assert_eq!(record.total_dofs, 288);
        assert_eq!(record.reduced_dofs, 252);

        let stiffness = assemble_global_stiffness(fixture.node_count, &fixture.elements).unwrap();
        let reduced = reduce_system(&stiffness, &fixture.force, &fixture.restrained_dofs).unwrap();
        let dense_solution = solve_dense(&reduced.stiffness, &reduced.force).unwrap();
        let scale = dense_solution
            .iter()
            .map(|value| value.abs())
            .fold(0.0, f64::max);

        // Parity tolerance basis: DEC-026 analytic-class relative seed
        // (1.0e-9), scaled by the dense reference solution magnitude.
        assert!(scale > 0.0);
        assert!(observation.max_abs_sparse_dense_solution_delta.unwrap() <= 1.0e-9 * scale);
        assert_eq!(observation.max_abs_repeat_solution_delta, 0.0);
        assert_eq!(observation.nonpositive_pivot_count, 0);
    }

    #[test]
    fn grid_fixture_rejects_degenerate_dimensions() {
        let error = invented_grid_frame_fixture(1, 5).unwrap_err();

        assert_eq!(
            error,
            HarnessError::InvalidSetting {
                name: "grid_dimensions",
                detail: "x_count and y_count must each be at least two"
            }
        );
    }

    #[test]
    fn suite_summary_aggregates_sparse_observations() {
        let suite = run_invented_fixture_suite(&[2, 4], &HarnessSettings::default()).unwrap();

        assert_eq!(suite.summary.records_with_sparse_observation_count, 2);
        assert!(suite.summary.total_ordered_profile_entry_count > 0);
        assert!(
            suite.summary.total_ordered_profile_entry_count
                <= suite.summary.total_original_profile_entry_count
        );
        assert!(suite.summary.max_abs_sparse_residual < 1.0e-6);
        for record in &suite.fixture_records {
            let observation = record.sparse_observation.as_ref().unwrap();
            assert_eq!(observation.repeat_count, record.repeat_count);
        }
        assert!(suite
            .limitations
            .iter()
            .any(|limitation| limitation.contains("DEC-023")));
    }

    #[test]
    fn rejects_protected_or_unknown_fixture_provenance() {
        let mut fixture = invented_cantilever_chain_fixture(1).unwrap();
        fixture.provenance.redistribution_status = FixtureProvenanceStatus::ProtectedSuspected;

        let error = run_fixture_repeat(&fixture, &HarnessSettings::default()).unwrap_err();

        assert_eq!(
            error,
            HarnessError::InvalidFixtureProvenance {
                fixture_id: "invented-cantilever-chain-1".to_string(),
                status: FixtureProvenanceStatus::ProtectedSuspected
            }
        );
    }

    #[test]
    fn rejects_invalid_repeat_count() {
        let fixture = invented_cantilever_chain_fixture(1).unwrap();
        let settings = HarnessSettings {
            repeat_count: 0,
            ..HarnessSettings::default()
        };

        let error = run_fixture_repeat(&fixture, &settings).unwrap_err();

        assert_eq!(
            error,
            HarnessError::InvalidSetting {
                name: "repeat_count",
                detail: "must be at least one"
            }
        );
    }

    #[test]
    fn fixture_force_length_must_match_node_dofs() {
        let mut fixture = invented_cantilever_chain_fixture(1).unwrap();
        fixture.force.pop();

        let error = run_fixture_repeat(&fixture, &HarnessSettings::default()).unwrap_err();

        assert_eq!(
            error,
            HarnessError::InvalidSetting {
                name: "fixture.force",
                detail: "length must match node_count * DOF_PER_NODE"
            }
        );
    }

    #[test]
    fn zero_element_fixture_is_rejected() {
        let error = invented_cantilever_chain_fixture(0).unwrap_err();

        assert_eq!(
            error,
            HarnessError::InvalidSetting {
                name: "element_count",
                detail: "must be at least one"
            }
        );
    }
}

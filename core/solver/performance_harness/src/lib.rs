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
    classify_condition_ratio, diagnostic_from_frame_error, sparse_solver_tbd_diagnostic,
    tolerance_policy_tbd_diagnostic, DiagnosticsError, SolverDiagnostic,
};
use std::error::Error;
use std::fmt;

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
    pub diagnostics: Vec<SolverDiagnostic>,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
    pub provenance_notes: Vec<String>,
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

    let first_solution = match solve_dense(&reduced.stiffness, &reduced.force) {
        Ok(solution) => solution,
        Err(error) => {
            diagnostics.push(diagnostic_from_frame_error(&error));
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
        diagnostics,
    ))
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
        diagnostics,
        assumptions: vec![
            "fixture uses invented mechanics quantities".to_string(),
            "fixture unit identifiers declare the calculation basis without conversion constants"
                .to_string(),
            "frame-kernel dense solve path is a verification stand-in for future sparse adapter"
                .to_string(),
        ],
        limitations: vec![
            "sparse numerical library remains TBD".to_string(),
            "release timing, memory, and conditioning thresholds remain TBD".to_string(),
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

//! Original nonlinear support regression benchmarks for OpenPipeStress.
//!
//! The fixtures in this crate use invented values and direct calls into the
//! committed nonlinear-support and solver-diagnostics APIs. They are software
//! regression checks only: no protected standards content, real project values,
//! external commercial outputs, or authority claims are encoded.

use std::path::Path;

use open_pipe_stress_linear_supports::FrameDof;
use open_pipe_stress_nonlinear_supports::{
    evaluate_active_set_iteration, ActivationSense, ActiveSetIteration, ActiveSetIterationInput,
    ActiveSetState, GapDirection, NonlinearSupport, NonlinearSupportError, SupportStateRecord,
    TrialSupportState,
};
use open_pipe_stress_solver_diagnostics::{
    DiagnosticSeverity, SolverDiagnosticCode, SolverDiagnosticReport, SolverStatus,
};

const PKG09_NONLINEAR_FIXTURE_UNIT_SYSTEM_REF: &str =
    "PKG09-NONLINEAR-FIXTURE-UNITS-EXPLICIT-MM-N-NM";
const CANONICAL_DIMENSIONS: &[&str] = &[
    "dimensionless",
    "length",
    "mass",
    "time",
    "temperature",
    "temperature_interval",
    "angle",
    "rotation",
    "force",
    "force_per_length",
    "moment",
    "pressure",
    "stress",
    "area",
    "volume",
    "density",
    "linear_stiffness",
    "rotational_stiffness",
    "displacement",
    "velocity",
    "acceleration",
    "thermal_conductivity",
    "specific_heat",
    "thermal_expansion_coefficient",
    "second_moment_area",
    "section_modulus",
    "mass_per_length",
    "volume_per_length",
    "slope",
    "TBD",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonlinearRegressionFamily {
    ActiveSet,
    Gap,
    LiftOff,
    Friction,
    NonConvergence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedistributionStatus {
    PublicOriginal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewDisposition {
    AcceptedInventedFixture,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkProvenance {
    pub source_name: &'static str,
    pub source_location: &'static str,
    pub source_license: &'static str,
    pub contributor: &'static str,
    pub contributor_assertion: &'static str,
    pub redistribution_status: RedistributionStatus,
    pub review_disposition: ReviewDisposition,
}

impl BenchmarkProvenance {
    pub fn public_original(source_location: &'static str) -> Self {
        Self {
            source_name: "OpenPipeStress original nonlinear support regression fixture",
            source_location,
            source_license: "project-original-public-content",
            contributor: "OpenPipeStress agentic development workflow",
            contributor_assertion:
                "Generated from invented support states; not copied from protected standards, commercial software examples, proprietary data, private data, or real project records.",
            redistribution_status: RedistributionStatus::PublicOriginal,
            review_disposition: ReviewDisposition::AcceptedInventedFixture,
        }
    }

    pub fn is_publicly_usable(&self) -> bool {
        self.redistribution_status == RedistributionStatus::PublicOriginal
            && self.review_disposition == ReviewDisposition::AcceptedInventedFixture
            && !self.source_name.is_empty()
            && !self.source_location.is_empty()
            && !self.source_license.is_empty()
            && self
                .contributor_assertion
                .contains("not copied from protected standards")
            && self
                .contributor_assertion
                .contains("invented support states")
    }

    pub fn source_artifact_exists(&self, repo_root: &Path) -> bool {
        repo_root.join(self.source_location).is_file()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureUnitBasis {
    pub unit_system_ref: &'static str,
    pub unit_system_status: &'static str,
    pub support_displacement_unit: &'static str,
    pub translational_reaction_unit: &'static str,
    pub rotational_reaction_unit: &'static str,
    pub friction_coefficient_unit: &'static str,
    pub residual_tolerance_unit: &'static str,
    pub residual_tolerance_dimension: &'static str,
    pub note: &'static str,
}

impl FixtureUnitBasis {
    pub fn is_explicit_fixture_basis(&self) -> bool {
        self.unit_system_ref == PKG09_NONLINEAR_FIXTURE_UNIT_SYSTEM_REF
            && self.unit_system_status == "fixture-local-explicit-units-no-conversions"
            && !self.support_displacement_unit.is_empty()
            && !self.translational_reaction_unit.is_empty()
            && !self.rotational_reaction_unit.is_empty()
            && !self.friction_coefficient_unit.is_empty()
            && self.residual_tolerance_dimension == "dimensionless"
            && self.note.contains("unit catalog remains TBD")
    }
}

pub const NONLINEAR_FIXTURE_UNIT_BASIS: FixtureUnitBasis = FixtureUnitBasis {
    unit_system_ref: PKG09_NONLINEAR_FIXTURE_UNIT_SYSTEM_REF,
    unit_system_status: "fixture-local-explicit-units-no-conversions",
    support_displacement_unit: "mm",
    translational_reaction_unit: "N",
    rotational_reaction_unit: "N-m",
    friction_coefficient_unit: "ratio",
    residual_tolerance_unit: "count",
    residual_tolerance_dimension: "dimensionless",
    note: "Explicit fixture-local unit identifiers for raw support input values and active-set tolerance only; project unit catalog remains TBD and no conversion constants are encoded.",
};

#[derive(Debug, Clone, PartialEq)]
pub struct DimensionedObservation {
    pub name: &'static str,
    pub value: f64,
    pub unit: &'static str,
    pub dimension: &'static str,
    pub tolerance_policy: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpectedState {
    pub support_id: &'static str,
    pub state: ActiveSetState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpectedDiagnostic {
    pub code: SolverDiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub status: SolverStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearRegressionCase {
    pub fixture_id: &'static str,
    pub family: NonlinearRegressionFamily,
    pub description: &'static str,
    pub assumptions: &'static [&'static str],
    pub provenance: BenchmarkProvenance,
    pub unit_basis: FixtureUnitBasis,
    pub input: ActiveSetIterationInput,
    pub expected_states: Vec<ExpectedState>,
    pub expected_changed_supports: Vec<&'static str>,
    pub expected_residual_norm: f64,
    pub expected_converged: bool,
    pub expected_diagnostic: Option<ExpectedDiagnostic>,
    pub observations: Vec<DimensionedObservation>,
}

impl NonlinearRegressionCase {
    pub fn run(&self) -> Result<ActiveSetIteration, NonlinearSupportError> {
        evaluate_active_set_iteration(&self.input)
    }

    pub fn expected_report(&self) -> SolverDiagnosticReport {
        let status = self
            .expected_diagnostic
            .as_ref()
            .map(|expected| expected.status)
            .unwrap_or(SolverStatus::MechanicsSolved);

        let diagnostics = self
            .run()
            .expect("fixture construction must remain valid")
            .diagnostics;

        SolverDiagnosticReport::new(status, diagnostics)
    }

    pub fn tolerance_policy_is_unresolved(&self) -> bool {
        self.observations
            .iter()
            .all(|observation| observation.tolerance_policy.is_none())
    }

    pub fn has_dimensioned_observations(&self) -> bool {
        self.observations.iter().all(|observation| {
            observation.value.is_finite()
                && !observation.unit.is_empty()
                && CANONICAL_DIMENSIONS.contains(&observation.dimension)
        }) && self.unit_basis.is_explicit_fixture_basis()
    }

    pub fn matches_expected_outcome(&self) -> bool {
        let iteration = match self.run() {
            Ok(iteration) => iteration,
            Err(_) => return false,
        };

        let expected_states: Vec<SupportStateRecord> = self
            .expected_states
            .iter()
            .map(|expected| SupportStateRecord::new(expected.support_id, expected.state))
            .collect();
        let expected_changed: Vec<String> = self
            .expected_changed_supports
            .iter()
            .map(|support| (*support).to_string())
            .collect();

        iteration.states == expected_states
            && iteration.changed_supports == expected_changed
            && iteration.residual_norm == self.expected_residual_norm
            && iteration.converged == self.expected_converged
            && diagnostic_matches(self.expected_diagnostic.as_ref(), &iteration)
    }
}

pub fn fixture_inventory() -> Vec<NonlinearRegressionCase> {
    vec![
        active_set_one_way_fixture(),
        gap_closure_fixture(),
        lift_off_fixture(),
        friction_transition_fixture(),
        unresolved_nonconvergence_fixture(),
    ]
}

pub fn missing_required_families(
    fixtures: &[NonlinearRegressionCase],
) -> Vec<NonlinearRegressionFamily> {
    let required = [
        NonlinearRegressionFamily::ActiveSet,
        NonlinearRegressionFamily::Gap,
        NonlinearRegressionFamily::LiftOff,
        NonlinearRegressionFamily::Friction,
        NonlinearRegressionFamily::NonConvergence,
    ];

    required
        .into_iter()
        .filter(|family| !fixtures.iter().any(|fixture| fixture.family == *family))
        .collect()
}

pub fn active_set_one_way_fixture() -> NonlinearRegressionCase {
    let support_id = "NL-ACTIVE-ONE-WAY-A";
    let support = NonlinearSupport::one_way(
        support_id,
        0,
        FrameDof::Uz,
        ActivationSense::PositiveReaction,
    );
    let input = ActiveSetIterationInput {
        iteration: 1,
        max_iterations: 6,
        tolerance: 0.0,
        supports: vec![support],
        trial_states: vec![TrialSupportState::new(support_id, 0.0, 4.0).unwrap()],
        prior_states: vec![SupportStateRecord::new(
            support_id,
            ActiveSetState::Inactive,
        )],
    };

    NonlinearRegressionCase {
        fixture_id: "NL-ACTIVE-ONE-WAY-ORIGINAL",
        family: NonlinearRegressionFamily::ActiveSet,
        description: "Invented one-way support activates from a positive trial reaction.",
        assumptions: &[
            "Reaction sign is supplied by the committed nonlinear-support API.",
            "The case exercises active-set change tracking without a global frame solve.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/active_set_one_way.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_states: vec![ExpectedState {
            support_id,
            state: ActiveSetState::Active,
        }],
        expected_changed_supports: vec![support_id],
        expected_residual_norm: 1.0,
        expected_converged: false,
        expected_diagnostic: Some(ExpectedDiagnostic {
            code: SolverDiagnosticCode::NonConvergence,
            severity: DiagnosticSeverity::Warning,
            status: SolverStatus::SolvedWithWarnings,
        }),
        observations: vec![
            DimensionedObservation {
                name: "trial_reaction",
                value: 4.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "state_change_count",
                value: 1.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn gap_closure_fixture() -> NonlinearRegressionCase {
    let support_id = "NL-GAP-POSITIVE-A";
    let support = NonlinearSupport::gap(
        support_id,
        1,
        FrameDof::Ux,
        0.25,
        GapDirection::PositiveDisplacement,
    )
    .unwrap();
    let input = ActiveSetIterationInput {
        iteration: 2,
        max_iterations: 6,
        tolerance: 0.0,
        supports: vec![support],
        trial_states: vec![TrialSupportState::new(support_id, 0.25, 0.0).unwrap()],
        prior_states: vec![SupportStateRecord::new(support_id, ActiveSetState::Active)],
    };

    NonlinearRegressionCase {
        fixture_id: "NL-GAP-CLOSURE-ORIGINAL",
        family: NonlinearRegressionFamily::Gap,
        description: "Invented positive-clearance gap remains closed at its explicit clearance.",
        assumptions: &[
            "Gap closure is checked at the committed clearance comparison boundary.",
            "The prior active state is repeated to verify a converged unchanged active set.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/gap_closure.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_states: vec![ExpectedState {
            support_id,
            state: ActiveSetState::Active,
        }],
        expected_changed_supports: vec![],
        expected_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic: None,
        observations: vec![
            DimensionedObservation {
                name: "clearance",
                value: 0.25,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "trial_displacement",
                value: 0.25,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn lift_off_fixture() -> NonlinearRegressionCase {
    let support_id = "NL-LIFT-OFF-A";
    let support = NonlinearSupport::lift_off(
        support_id,
        2,
        FrameDof::Uy,
        ActivationSense::NegativeReaction,
    );
    let input = ActiveSetIterationInput {
        iteration: 2,
        max_iterations: 6,
        tolerance: 0.0,
        supports: vec![support],
        trial_states: vec![TrialSupportState::new(support_id, 0.04, 0.0).unwrap()],
        prior_states: vec![SupportStateRecord::new(support_id, ActiveSetState::Active)],
    };

    NonlinearRegressionCase {
        fixture_id: "NL-LIFT-OFF-ORIGINAL",
        family: NonlinearRegressionFamily::LiftOff,
        description:
            "Invented lift-off support loses contact when the trial reaction reaches zero.",
        assumptions: &[
            "Contact is represented by the negative-reaction sense in this invented case.",
            "Loss of contact is reported as an active-set change before the iteration limit.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/lift_off.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_states: vec![ExpectedState {
            support_id,
            state: ActiveSetState::Inactive,
        }],
        expected_changed_supports: vec![support_id],
        expected_residual_norm: 1.0,
        expected_converged: false,
        expected_diagnostic: Some(ExpectedDiagnostic {
            code: SolverDiagnosticCode::NonConvergence,
            severity: DiagnosticSeverity::Warning,
            status: SolverStatus::SolvedWithWarnings,
        }),
        observations: vec![
            DimensionedObservation {
                name: "trial_reaction",
                value: 0.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "trial_displacement",
                value: 0.04,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn friction_transition_fixture() -> NonlinearRegressionCase {
    let stick_id = "NL-FRICTION-STICK-A";
    let slide_id = "NL-FRICTION-SLIDE-A";
    let stick_support = NonlinearSupport::friction(stick_id, 3, FrameDof::Ux, 0.30).unwrap();
    let slide_support = NonlinearSupport::friction(slide_id, 3, FrameDof::Uz, 0.30).unwrap();
    let input = ActiveSetIterationInput {
        iteration: 3,
        max_iterations: 6,
        tolerance: 0.0,
        supports: vec![stick_support, slide_support],
        trial_states: vec![
            TrialSupportState::new(stick_id, 0.0, 0.0)
                .unwrap()
                .with_friction_reactions(10.0, 2.0)
                .unwrap(),
            TrialSupportState::new(slide_id, 0.0, 0.0)
                .unwrap()
                .with_friction_reactions(10.0, 3.5)
                .unwrap(),
        ],
        prior_states: vec![
            SupportStateRecord::new(stick_id, ActiveSetState::Sticking),
            SupportStateRecord::new(slide_id, ActiveSetState::Sliding),
        ],
    };

    NonlinearRegressionCase {
        fixture_id: "NL-FRICTION-STICK-SLIDE-ORIGINAL",
        family: NonlinearRegressionFamily::Friction,
        description: "Invented friction pair classifies one support as sticking and one as sliding.",
        assumptions: &[
            "The friction limit is coefficient times positive normal reaction.",
            "The case uses invented normal and tangential reactions with no component catalog data.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/friction_transition.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_states: vec![
            ExpectedState {
                support_id: slide_id,
                state: ActiveSetState::Sliding,
            },
            ExpectedState {
                support_id: stick_id,
                state: ActiveSetState::Sticking,
            },
        ],
        expected_changed_supports: vec![],
        expected_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic: None,
        observations: vec![
            DimensionedObservation {
                name: "friction_coefficient",
                value: 0.30,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "stick_tangential_reaction",
                value: 2.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "slide_tangential_reaction",
                value: 3.5,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn unresolved_nonconvergence_fixture() -> NonlinearRegressionCase {
    let support_id = "NL-NONCONVERGENCE-A";
    let support = NonlinearSupport::one_way(
        support_id,
        4,
        FrameDof::Ry,
        ActivationSense::NegativeReaction,
    );
    let input = ActiveSetIterationInput {
        iteration: 4,
        max_iterations: 4,
        tolerance: 0.0,
        supports: vec![support],
        trial_states: vec![TrialSupportState::new(support_id, 0.0, -1.5).unwrap()],
        prior_states: vec![SupportStateRecord::new(
            support_id,
            ActiveSetState::Inactive,
        )],
    };

    NonlinearRegressionCase {
        fixture_id: "NL-NONCONVERGENCE-LIMIT-ORIGINAL",
        family: NonlinearRegressionFamily::NonConvergence,
        description: "Invented support remains changed at the configured iteration limit.",
        assumptions: &[
            "The residual is the committed active-set changed-support count.",
            "The expected diagnostic is unresolved non-convergence, not an engineering acceptance result.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/unresolved_nonconvergence.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_states: vec![ExpectedState {
            support_id,
            state: ActiveSetState::Active,
        }],
        expected_changed_supports: vec![support_id],
        expected_residual_norm: 1.0,
        expected_converged: false,
        expected_diagnostic: Some(ExpectedDiagnostic {
            code: SolverDiagnosticCode::NonConvergence,
            severity: DiagnosticSeverity::Failure,
            status: SolverStatus::SolveFailed,
        }),
        observations: vec![
            DimensionedObservation {
                name: "trial_rotational_reaction",
                value: -1.5,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 4.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "active_set_residual",
                value: 1.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
        ],
    }
}

fn diagnostic_matches(
    expected: Option<&ExpectedDiagnostic>,
    iteration: &ActiveSetIteration,
) -> bool {
    match expected {
        None => iteration.diagnostics.is_empty(),
        Some(expected) => {
            iteration.diagnostics.len() == 1
                && iteration.diagnostics[0].code == expected.code
                && iteration.diagnostics[0].severity == expected.severity
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_covers_required_nonlinear_families() {
        let fixtures = fixture_inventory();

        assert!(missing_required_families(&fixtures).is_empty());
        assert_eq!(fixtures.len(), 5);
    }

    #[test]
    fn fixtures_are_public_original_and_unit_aware() {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(3)
            .expect("benchmark crate remains under validation/benchmarks/nonlinear");

        for fixture in fixture_inventory() {
            assert!(fixture.provenance.is_publicly_usable());
            assert!(
                fixture.provenance.source_artifact_exists(repo_root),
                "{} provenance source is missing: {}",
                fixture.fixture_id,
                fixture.provenance.source_location
            );
            assert!(fixture.has_dimensioned_observations());
            assert!(fixture.tolerance_policy_is_unresolved());
        }
    }

    #[test]
    fn active_set_gap_lift_off_and_friction_outcomes_are_deterministic() {
        for fixture in fixture_inventory() {
            assert!(
                fixture.matches_expected_outcome(),
                "{:?}",
                fixture.fixture_id
            );
        }
    }

    #[test]
    fn nonconvergence_fixture_reports_failure_diagnostic() {
        let fixture = unresolved_nonconvergence_fixture();
        let iteration = fixture.run().unwrap();

        assert!(!iteration.converged);
        assert!(iteration.is_blocked());
        assert_eq!(iteration.diagnostics.len(), 1);
        assert_eq!(
            iteration.diagnostics[0].code,
            SolverDiagnosticCode::NonConvergence
        );
        assert_eq!(
            iteration.diagnostics[0].severity,
            DiagnosticSeverity::Failure
        );
        assert!(iteration.diagnostics[0]
            .message
            .contains("did not converge after 4 iterations"));
    }

    #[test]
    fn expected_reports_preserve_warning_and_failure_statuses() {
        let warning_report = active_set_one_way_fixture().expected_report();
        let failure_report = unresolved_nonconvergence_fixture().expected_report();

        assert_eq!(warning_report.status, SolverStatus::SolvedWithWarnings);
        assert!(!warning_report.is_blocked());
        assert_eq!(failure_report.status, SolverStatus::SolveFailed);
        assert!(failure_report.is_blocked());
    }
}

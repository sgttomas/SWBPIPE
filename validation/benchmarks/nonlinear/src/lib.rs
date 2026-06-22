//! Original nonlinear support regression benchmarks for OpenPipeStress.
//!
//! The fixtures in this crate use invented values and direct calls into the
//! committed nonlinear-support and solver-diagnostics APIs. They are software
//! regression checks only: no protected standards content, real project values,
//! external commercial outputs, or authority claims are encoded.

use std::path::Path;

use open_pipe_stress_frame_kernel::{
    node_dof_index, DenseVector, FrameDof, FrameElement, FrameNode, FrameSection, DOF_PER_NODE,
};
use open_pipe_stress_nonlinear_integration::{
    solve_active_set_frame, ConvergenceControl, ConvergencePolicyStatus,
    DerivedFrictionNormalReaction, FrictionNormalReaction, NonlinearFrameSolveInput,
    NonlinearFrameSolveResult,
};
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
pub const DEC_046_ACTIVE_SET_COUNT_POLICY_REF: &str = "DEC-046-CV-B-active-set-count-validation-v1";
const DEC_046_ACTIVE_SET_COUNT_RESIDUAL_BASIS: &str = "active_set_changed_support_count";
const DEC_046_ACTIVE_SET_COUNT_RESIDUAL_UNIT: &str = "count";
const DEC_046_ACTIVE_SET_COUNT_RESIDUAL_DIMENSION: &str = "dimensionless";
const DEC_046_ACTIVE_SET_COUNT_RELATIVE_TOLERANCE: f64 = 0.0;
const DEC_046_ACTIVE_SET_COUNT_ABSOLUTE_FLOOR: f64 = 0.0;
const DEC_046_ACTIVE_SET_COUNT_MAX_ITERATIONS: usize = 4;
const DEC_046_ACTIVE_SET_COUNT_LIMITATIONS: &[&str] = &[
    "Applies only to the current public-original assembled validation seed and its active-set changed-support-count residual.",
    "Does not define free-DOF force/moment, displacement, free-DOF work/energy, sparse live-path, product-preview, or external validation convergence thresholds.",
];
pub const DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF: &str =
    "DEC-046-CV-B-free-dof-force-moment-residual-validation-v1";
const DEC_046_FREE_DOF_FORCE_RESIDUAL_BASIS: &str = "free_dof_force_residual";
const DEC_046_FREE_DOF_FORCE_RESIDUAL_UNIT: &str = "N";
const DEC_046_FREE_DOF_FORCE_RESIDUAL_DIMENSION: &str = "force";
const DEC_046_FREE_DOF_FORCE_ABSOLUTE_LIMIT: f64 = 0.0;
const DEC_046_FREE_DOF_MOMENT_RESIDUAL_BASIS: &str = "free_dof_moment_residual";
const DEC_046_FREE_DOF_MOMENT_RESIDUAL_UNIT: &str = "N-m";
const DEC_046_FREE_DOF_MOMENT_RESIDUAL_DIMENSION: &str = "moment";
const DEC_046_FREE_DOF_MOMENT_ABSOLUTE_LIMIT: f64 = 0.0;
const DEC_046_FREE_DOF_FORCE_MOMENT_LIMITATIONS: &[&str] = &[
    "Applies only to current public-original assembled validation-seed final-iteration free-DOF force and moment equilibrium residuals.",
    "Does not define displacement-delta, reaction-delta, free-DOF work/energy, sparse live-path, product-preview, release, or external validation thresholds.",
];
pub const DEC_046_FREE_DOF_WORK_POLICY_REF: &str =
    "DEC-046-CV-B-free-dof-work-residual-validation-v1";
const DEC_046_FREE_DOF_WORK_RESIDUAL_BASIS: &str = "free_dof_work_residual";
const DEC_046_FREE_DOF_WORK_RESIDUAL_UNIT: &str = "N-m";
const DEC_046_FREE_DOF_WORK_RESIDUAL_DIMENSION: &str = "moment";
const DEC_046_FREE_DOF_WORK_ABSOLUTE_LIMIT: f64 = 0.0;
const DEC_046_FREE_DOF_WORK_LIMITATIONS: &[&str] = &[
    "Applies only to current public-original assembled validation-seed final-iteration free-DOF residual work products.",
    "Does not define displacement-delta, reaction-delta, general energy convergence, sparse live-path, product-preview, release, or external validation thresholds.",
];
pub const DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF: &str =
    "DEC-046-CV-B-displacement-reaction-delta-observation-v1";
pub const DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_REF: &str =
    "DEC-046-CV-B-displacement-reaction-delta-threshold-validation-v1";
const DEC_046_TRANSLATION_DELTA_BASIS: &str = "max_abs_translation_delta_from_previous";
const DEC_046_TRANSLATION_DELTA_UNIT: &str = "mm";
const DEC_046_TRANSLATION_DELTA_DIMENSION: &str = "displacement";
const DEC_046_ROTATION_DELTA_BASIS: &str = "max_abs_rotation_delta_from_previous";
const DEC_046_ROTATION_DELTA_UNIT: &str = "rad";
const DEC_046_ROTATION_DELTA_DIMENSION: &str = "rotation";
const DEC_046_FORCE_REACTION_DELTA_BASIS: &str = "max_abs_force_reaction_delta_from_previous";
const DEC_046_FORCE_REACTION_DELTA_UNIT: &str = "N";
const DEC_046_FORCE_REACTION_DELTA_DIMENSION: &str = "force";
const DEC_046_MOMENT_REACTION_DELTA_BASIS: &str = "max_abs_moment_reaction_delta_from_previous";
const DEC_046_MOMENT_REACTION_DELTA_UNIT: &str = "N-m";
const DEC_046_MOMENT_REACTION_DELTA_DIMENSION: &str = "moment";
const DEC_046_SEED_TRANSLATION_DELTA_ABSOLUTE_LIMIT: f64 = 100.0;
const DEC_046_SEED_GAP_TRANSLATION_DELTA_ABSOLUTE_LIMIT: f64 = 50.0;
const DEC_046_SEED_ROTATION_DELTA_ABSOLUTE_LIMIT: f64 = 0.0;
const DEC_046_SEED_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT: f64 = 10.0;
const DEC_046_SEED_GAP_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT: f64 = 5.0;
const DEC_046_SEED_MOMENT_REACTION_DELTA_ABSOLUTE_LIMIT: f64 = 0.0;
const DEC_046_DISPLACEMENT_REACTION_DELTA_LIMITATIONS: &[&str] = &[
    "Observation-only record for current public-original assembled validation-seed final-iteration deltas from the previous active-set solve.",
    "The observed deltas are transition magnitudes between consecutive linearized active-set solves; nonzero final values can coexist with active-set convergence.",
    "Does not define displacement-delta, reaction-delta, general energy, sparse live-path, product-preview, release, or external validation thresholds.",
];
const DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_LIMITATIONS: &[&str] = &[
    "Applies only to the current public-original assembled validation seed final-iteration displacement and reaction deltas from the previous active-set solve.",
    "Limits are fixture-evidence envelopes for the accepted seed classes; fixture-local overrides may only tighten them.",
    "Does not define general energy, sparse live-path, product-preview, release, external validation, or CI thresholds.",
];
const MULTI_SUPPORT_DEPTH_POLICY_REF: &str = "TP-R4-D9-MULTISUPPORT-OBS-TBD";
pub const DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF: &str =
    "DEC-046-CV-B-multisupport-active-set-count-validation-v1";
pub const DEC_046_MULTISUPPORT_FREE_DOF_FORCE_MOMENT_POLICY_REF: &str =
    "DEC-046-CV-B-multisupport-free-dof-force-moment-residual-validation-v1";
pub const DEC_046_MULTISUPPORT_FREE_DOF_WORK_POLICY_REF: &str =
    "DEC-046-CV-B-multisupport-free-dof-work-residual-validation-v1";
pub const DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF: &str =
    "DEC-046-CV-B-multisupport-displacement-reaction-delta-observation-v1";
pub const DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_POLICY_REF: &str =
    "DEC-046-CV-B-multisupport-displacement-reaction-delta-threshold-validation-v1";
const DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_LIMITATIONS: &[&str] = &[
    "Applies only to the public-original multi-DOF / multi-support validation fixture set and its active-set changed-support-count residual.",
    "Does not define displacement-delta, reaction-delta, free-DOF work/energy, sparse live-path, product-preview, release, or external validation thresholds.",
];
const DEC_046_MULTISUPPORT_FREE_DOF_FORCE_MOMENT_LIMITATIONS: &[&str] = &[
    "Applies only to the public-original multi-DOF / multi-support validation fixture set final-iteration free-DOF force and moment equilibrium residuals.",
    "Does not define displacement-delta, reaction-delta, free-DOF work/energy, sparse live-path, product-preview, release, or external validation thresholds.",
];
const DEC_046_MULTISUPPORT_FREE_DOF_WORK_LIMITATIONS: &[&str] = &[
    "Applies only to the public-original multi-DOF / multi-support validation fixture set final-iteration free-DOF residual work products.",
    "Does not define displacement-delta, reaction-delta, general energy convergence, sparse live-path, product-preview, release, or external validation thresholds.",
];
const DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_LIMITATIONS: &[&str] = &[
    "Observation-only record for public-original multi-DOF / multi-support validation fixture-set final-iteration deltas from the previous active-set solve.",
    "The observed deltas are transition magnitudes between consecutive linearized active-set solves; nonzero final values can coexist with active-set convergence.",
    "Does not define displacement-delta, reaction-delta, general energy, sparse live-path, product-preview, release, or external validation thresholds.",
];
const DEC_046_MULTISUPPORT_TRANSLATION_DELTA_ABSOLUTE_LIMIT: f64 = 100.0;
const DEC_046_MULTISUPPORT_ROTATION_DELTA_ABSOLUTE_LIMIT: f64 = 0.005;
const DEC_046_MULTISUPPORT_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT: f64 = 10.0;
const DEC_046_MULTISUPPORT_MOMENT_REACTION_DELTA_ABSOLUTE_LIMIT: f64 = 3.0;
const DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_POLICY_LIMITATIONS: &[&str] = &[
    "Applies only to the public-original multi-DOF / multi-support validation fixture set final-iteration displacement and reaction deltas from the previous active-set solve.",
    "Limits are fixture-evidence envelopes for the accepted nine-fixture multi-support set; fixture-local overrides may only tighten them.",
    "Does not define general energy, sparse default, product-preview, release, external validation, or CI thresholds.",
];
const DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS: &[&str] = &[
    "NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-ACCEPTED-ORIGINAL",
    "NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL",
    "NL-ASSEMBLED-MULTI-DOF-FRICTION-GAP-ACCEPTED-ORIGINAL",
    "NL-ASSEMBLED-MULTI-DOF-THREE-SUPPORT-ACCEPTED-ORIGINAL",
    "NL-ASSEMBLED-MULTI-DOF-ROTATIONAL-ACCEPTED-ORIGINAL",
    "NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-GAP-ACCEPTED-ORIGINAL",
    "NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-ROTATIONAL-ACCEPTED-ORIGINAL",
    "NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL",
    "NL-ASSEMBLED-MULTI-DOF-NEGATIVE-GAP-ACCEPTED-ORIGINAL",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonlinearRegressionFamily {
    ActiveSet,
    Gap,
    LiftOff,
    Friction,
    NonConvergence,
    MixedSupport,
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

#[derive(Debug, Clone, PartialEq)]
pub struct AssembledNonlinearRegressionCase {
    pub fixture_id: &'static str,
    pub family: NonlinearRegressionFamily,
    pub description: &'static str,
    pub assumptions: &'static [&'static str],
    pub provenance: BenchmarkProvenance,
    pub unit_basis: FixtureUnitBasis,
    pub input: NonlinearFrameSolveInput,
    pub expected_final_states: Vec<ExpectedState>,
    pub expected_iteration_count: usize,
    pub expected_final_residual_norm: f64,
    pub expected_converged: bool,
    pub expected_diagnostic_codes: Vec<SolverDiagnosticCode>,
    pub observations: Vec<DimensionedObservation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConvergenceObservation {
    pub fixture_id: &'static str,
    pub family: NonlinearRegressionFamily,
    pub nonlinear_class: &'static str,
    pub policy_ref: String,
    pub policy_status: ConvergencePolicyStatus,
    pub max_iterations: usize,
    pub observed_iteration_count: usize,
    pub final_residual_norm: f64,
    pub residual_unit: &'static str,
    pub residual_dimension: &'static str,
    pub observed_converged: bool,
    pub diagnostic_codes: Vec<SolverDiagnosticCode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForceDisplacementResidualObservation {
    pub fixture_id: &'static str,
    pub family: NonlinearRegressionFamily,
    pub nonlinear_class: &'static str,
    pub policy_ref: String,
    pub observed_iteration_count: usize,
    pub max_abs_translation_delta_from_previous: Option<f64>,
    pub translation_delta_unit: &'static str,
    pub max_abs_rotation_delta_from_previous: Option<f64>,
    pub rotation_delta_unit: &'static str,
    pub max_abs_force_reaction_delta_from_previous: Option<f64>,
    pub force_reaction_delta_unit: &'static str,
    pub max_abs_moment_reaction_delta_from_previous: Option<f64>,
    pub moment_reaction_delta_unit: &'static str,
    pub max_abs_free_dof_force_residual: f64,
    pub free_dof_force_residual_unit: &'static str,
    pub max_abs_free_dof_moment_residual: f64,
    pub free_dof_moment_residual_unit: &'static str,
    pub max_abs_free_dof_work_residual: f64,
    pub free_dof_work_residual_unit: &'static str,
    pub displacement_reaction_delta_observation_ref: Option<&'static str>,
    pub displacement_reaction_delta_threshold_policy: Option<&'static str>,
    pub free_dof_force_moment_threshold_policy: Option<&'static str>,
    pub free_dof_work_threshold_policy: Option<&'static str>,
}

impl ForceDisplacementResidualObservation {
    pub fn uses_accepted_free_dof_force_moment_threshold_policy(&self) -> bool {
        self.policy_ref == DEC_046_ACTIVE_SET_COUNT_POLICY_REF
            && self.free_dof_force_moment_threshold_policy
                == Some(DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF)
            && self.translation_delta_unit == "mm"
            && self.rotation_delta_unit == "rad"
            && self.force_reaction_delta_unit == "N"
            && self.moment_reaction_delta_unit == "N-m"
            && self.free_dof_force_residual_unit == DEC_046_FREE_DOF_FORCE_RESIDUAL_UNIT
            && self.free_dof_moment_residual_unit == DEC_046_FREE_DOF_MOMENT_RESIDUAL_UNIT
            && self.free_dof_work_residual_unit == DEC_046_FREE_DOF_WORK_RESIDUAL_UNIT
            && self.free_dof_work_threshold_policy == Some(DEC_046_FREE_DOF_WORK_POLICY_REF)
            && self.max_abs_free_dof_force_residual.is_finite()
            && self.max_abs_free_dof_moment_residual.is_finite()
            && self.max_abs_free_dof_work_residual.is_finite()
            && self.max_abs_free_dof_force_residual <= DEC_046_FREE_DOF_FORCE_ABSOLUTE_LIMIT
            && self.max_abs_free_dof_moment_residual <= DEC_046_FREE_DOF_MOMENT_ABSOLUTE_LIMIT
            && self.max_abs_free_dof_work_residual <= DEC_046_FREE_DOF_WORK_ABSOLUTE_LIMIT
    }

    pub fn records_tbd_displacement_reaction_delta_observation(&self) -> bool {
        self.policy_ref == DEC_046_ACTIVE_SET_COUNT_POLICY_REF
            && self.displacement_reaction_delta_observation_ref
                == Some(DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF)
            && self.displacement_reaction_delta_threshold_policy.is_none()
            && self.has_dimensioned_displacement_reaction_delta_axes()
    }

    pub fn uses_accepted_displacement_reaction_delta_threshold_policy(&self) -> bool {
        let Some((translation_limit, rotation_limit, force_reaction_limit, moment_reaction_limit)) =
            seed_displacement_reaction_delta_limits(self.nonlinear_class)
        else {
            return false;
        };

        self.policy_ref == DEC_046_ACTIVE_SET_COUNT_POLICY_REF
            && self.displacement_reaction_delta_observation_ref
                == Some(DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF)
            && self.displacement_reaction_delta_threshold_policy
                == Some(DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_REF)
            && self.has_dimensioned_displacement_reaction_delta_axes()
            && optional_delta_within_limit(
                self.max_abs_translation_delta_from_previous,
                translation_limit,
            )
            && optional_delta_within_limit(
                self.max_abs_rotation_delta_from_previous,
                rotation_limit,
            )
            && optional_delta_within_limit(
                self.max_abs_force_reaction_delta_from_previous,
                force_reaction_limit,
            )
            && optional_delta_within_limit(
                self.max_abs_moment_reaction_delta_from_previous,
                moment_reaction_limit,
            )
    }

    pub fn uses_accepted_multisupport_free_dof_force_moment_threshold_policy(&self) -> bool {
        self.policy_ref == DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF
            && self.free_dof_force_moment_threshold_policy
                == Some(DEC_046_MULTISUPPORT_FREE_DOF_FORCE_MOMENT_POLICY_REF)
            && self.translation_delta_unit == "mm"
            && self.rotation_delta_unit == "rad"
            && self.force_reaction_delta_unit == "N"
            && self.moment_reaction_delta_unit == "N-m"
            && self.free_dof_force_residual_unit == DEC_046_FREE_DOF_FORCE_RESIDUAL_UNIT
            && self.free_dof_moment_residual_unit == DEC_046_FREE_DOF_MOMENT_RESIDUAL_UNIT
            && self.free_dof_work_residual_unit == DEC_046_FREE_DOF_WORK_RESIDUAL_UNIT
            && self.free_dof_work_threshold_policy
                == Some(DEC_046_MULTISUPPORT_FREE_DOF_WORK_POLICY_REF)
            && self.max_abs_free_dof_force_residual.is_finite()
            && self.max_abs_free_dof_moment_residual.is_finite()
            && self.max_abs_free_dof_work_residual.is_finite()
            && self.max_abs_free_dof_force_residual <= DEC_046_FREE_DOF_FORCE_ABSOLUTE_LIMIT
            && self.max_abs_free_dof_moment_residual <= DEC_046_FREE_DOF_MOMENT_ABSOLUTE_LIMIT
            && self.max_abs_free_dof_work_residual <= DEC_046_FREE_DOF_WORK_ABSOLUTE_LIMIT
    }

    pub fn records_tbd_multisupport_displacement_reaction_delta_observation(&self) -> bool {
        self.policy_ref == DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF
            && self.displacement_reaction_delta_observation_ref
                == Some(DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF)
            && self.displacement_reaction_delta_threshold_policy.is_none()
            && self.has_dimensioned_displacement_reaction_delta_axes()
    }

    pub fn uses_accepted_multisupport_displacement_reaction_delta_threshold_policy(&self) -> bool {
        self.policy_ref == DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF
            && self.displacement_reaction_delta_observation_ref
                == Some(DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF)
            && self.displacement_reaction_delta_threshold_policy
                == Some(DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_POLICY_REF)
            && self.has_dimensioned_displacement_reaction_delta_axes()
            && optional_delta_within_limit(
                self.max_abs_translation_delta_from_previous,
                DEC_046_MULTISUPPORT_TRANSLATION_DELTA_ABSOLUTE_LIMIT,
            )
            && optional_delta_within_limit(
                self.max_abs_rotation_delta_from_previous,
                DEC_046_MULTISUPPORT_ROTATION_DELTA_ABSOLUTE_LIMIT,
            )
            && optional_delta_within_limit(
                self.max_abs_force_reaction_delta_from_previous,
                DEC_046_MULTISUPPORT_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT,
            )
            && optional_delta_within_limit(
                self.max_abs_moment_reaction_delta_from_previous,
                DEC_046_MULTISUPPORT_MOMENT_REACTION_DELTA_ABSOLUTE_LIMIT,
            )
    }

    fn has_dimensioned_displacement_reaction_delta_axes(&self) -> bool {
        self.translation_delta_unit == DEC_046_TRANSLATION_DELTA_UNIT
            && self.rotation_delta_unit == DEC_046_ROTATION_DELTA_UNIT
            && self.force_reaction_delta_unit == DEC_046_FORCE_REACTION_DELTA_UNIT
            && self.moment_reaction_delta_unit == DEC_046_MOMENT_REACTION_DELTA_UNIT
            && optional_finite(self.max_abs_translation_delta_from_previous)
            && optional_finite(self.max_abs_rotation_delta_from_previous)
            && optional_finite(self.max_abs_force_reaction_delta_from_previous)
            && optional_finite(self.max_abs_moment_reaction_delta_from_previous)
    }
}

impl ConvergenceObservation {
    pub fn uses_accepted_dec_046_active_set_policy(&self) -> bool {
        self.policy_ref == DEC_046_ACTIVE_SET_COUNT_POLICY_REF
            && self.policy_status == ConvergencePolicyStatus::Accepted
            && !self
                .diagnostic_codes
                .contains(&SolverDiagnosticCode::TolerancePolicyTbd)
    }

    pub fn uses_accepted_multisupport_dec_046_active_set_policy(&self) -> bool {
        self.policy_ref == DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF
            && self.policy_status == ConvergencePolicyStatus::Accepted
            && !self
                .diagnostic_codes
                .contains(&SolverDiagnosticCode::TolerancePolicyTbd)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConvergencePolicyEntry {
    pub policy_ref: &'static str,
    pub nonlinear_class: &'static str,
    pub residual_basis: &'static str,
    pub residual_unit: &'static str,
    pub residual_dimension: &'static str,
    pub relative_residual_tolerance: f64,
    pub absolute_residual_floor: f64,
    pub max_iterations: usize,
    pub status: ConvergencePolicyStatus,
    pub evidence_fixture_ids: &'static [&'static str],
    pub limitations: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForceMomentResidualPolicyEntry {
    pub policy_ref: &'static str,
    pub nonlinear_class: &'static str,
    pub force_residual_basis: &'static str,
    pub force_residual_unit: &'static str,
    pub force_residual_dimension: &'static str,
    pub force_absolute_limit: f64,
    pub moment_residual_basis: &'static str,
    pub moment_residual_unit: &'static str,
    pub moment_residual_dimension: &'static str,
    pub moment_absolute_limit: f64,
    pub status: ConvergencePolicyStatus,
    pub evidence_fixture_ids: &'static [&'static str],
    pub limitations: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkResidualPolicyEntry {
    pub policy_ref: &'static str,
    pub nonlinear_class: &'static str,
    pub work_residual_basis: &'static str,
    pub work_residual_unit: &'static str,
    pub work_residual_dimension: &'static str,
    pub work_absolute_limit: f64,
    pub status: ConvergencePolicyStatus,
    pub evidence_fixture_ids: &'static [&'static str],
    pub limitations: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisplacementReactionDeltaObservationEntry {
    pub observation_ref: &'static str,
    pub nonlinear_class: &'static str,
    pub threshold_status: ConvergencePolicyStatus,
    pub translation_delta_basis: &'static str,
    pub translation_delta_unit: &'static str,
    pub translation_delta_dimension: &'static str,
    pub rotation_delta_basis: &'static str,
    pub rotation_delta_unit: &'static str,
    pub rotation_delta_dimension: &'static str,
    pub force_reaction_delta_basis: &'static str,
    pub force_reaction_delta_unit: &'static str,
    pub force_reaction_delta_dimension: &'static str,
    pub moment_reaction_delta_basis: &'static str,
    pub moment_reaction_delta_unit: &'static str,
    pub moment_reaction_delta_dimension: &'static str,
    pub evidence_fixture_ids: &'static [&'static str],
    pub limitations: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisplacementReactionDeltaPolicyEntry {
    pub policy_ref: &'static str,
    pub observation_ref: &'static str,
    pub nonlinear_class: &'static str,
    pub translation_delta_basis: &'static str,
    pub translation_delta_unit: &'static str,
    pub translation_delta_dimension: &'static str,
    pub translation_delta_absolute_limit: f64,
    pub rotation_delta_basis: &'static str,
    pub rotation_delta_unit: &'static str,
    pub rotation_delta_dimension: &'static str,
    pub rotation_delta_absolute_limit: f64,
    pub force_reaction_delta_basis: &'static str,
    pub force_reaction_delta_unit: &'static str,
    pub force_reaction_delta_dimension: &'static str,
    pub force_reaction_delta_absolute_limit: f64,
    pub moment_reaction_delta_basis: &'static str,
    pub moment_reaction_delta_unit: &'static str,
    pub moment_reaction_delta_dimension: &'static str,
    pub moment_reaction_delta_absolute_limit: f64,
    pub status: ConvergencePolicyStatus,
    pub evidence_fixture_ids: &'static [&'static str],
    pub limitations: &'static [&'static str],
}

impl ConvergencePolicyEntry {
    pub fn is_accepted_active_set_count_policy(&self) -> bool {
        self.policy_ref == DEC_046_ACTIVE_SET_COUNT_POLICY_REF
            && self.residual_basis == DEC_046_ACTIVE_SET_COUNT_RESIDUAL_BASIS
            && self.residual_unit == DEC_046_ACTIVE_SET_COUNT_RESIDUAL_UNIT
            && self.residual_dimension == DEC_046_ACTIVE_SET_COUNT_RESIDUAL_DIMENSION
            && self.relative_residual_tolerance == DEC_046_ACTIVE_SET_COUNT_RELATIVE_TOLERANCE
            && self.absolute_residual_floor == DEC_046_ACTIVE_SET_COUNT_ABSOLUTE_FLOOR
            && self.max_iterations == DEC_046_ACTIVE_SET_COUNT_MAX_ITERATIONS
            && self.status == ConvergencePolicyStatus::Accepted
            && !self.evidence_fixture_ids.is_empty()
            && self.limitations == DEC_046_ACTIVE_SET_COUNT_LIMITATIONS
    }

    pub fn is_accepted_multisupport_active_set_count_policy(&self) -> bool {
        self.policy_ref == DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF
            && self.nonlinear_class == "multi_support_multi_dof"
            && self.residual_basis == DEC_046_ACTIVE_SET_COUNT_RESIDUAL_BASIS
            && self.residual_unit == DEC_046_ACTIVE_SET_COUNT_RESIDUAL_UNIT
            && self.residual_dimension == DEC_046_ACTIVE_SET_COUNT_RESIDUAL_DIMENSION
            && self.relative_residual_tolerance == DEC_046_ACTIVE_SET_COUNT_RELATIVE_TOLERANCE
            && self.absolute_residual_floor == DEC_046_ACTIVE_SET_COUNT_ABSOLUTE_FLOOR
            && self.max_iterations == DEC_046_ACTIVE_SET_COUNT_MAX_ITERATIONS
            && self.status == ConvergencePolicyStatus::Accepted
            && !self.evidence_fixture_ids.is_empty()
            && self.limitations == DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_LIMITATIONS
    }
}

impl ForceMomentResidualPolicyEntry {
    pub fn is_accepted_free_dof_force_moment_policy(&self) -> bool {
        self.policy_ref == DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF
            && self.force_residual_basis == DEC_046_FREE_DOF_FORCE_RESIDUAL_BASIS
            && self.force_residual_unit == DEC_046_FREE_DOF_FORCE_RESIDUAL_UNIT
            && self.force_residual_dimension == DEC_046_FREE_DOF_FORCE_RESIDUAL_DIMENSION
            && self.force_absolute_limit == DEC_046_FREE_DOF_FORCE_ABSOLUTE_LIMIT
            && self.moment_residual_basis == DEC_046_FREE_DOF_MOMENT_RESIDUAL_BASIS
            && self.moment_residual_unit == DEC_046_FREE_DOF_MOMENT_RESIDUAL_UNIT
            && self.moment_residual_dimension == DEC_046_FREE_DOF_MOMENT_RESIDUAL_DIMENSION
            && self.moment_absolute_limit == DEC_046_FREE_DOF_MOMENT_ABSOLUTE_LIMIT
            && self.status == ConvergencePolicyStatus::Accepted
            && !self.evidence_fixture_ids.is_empty()
            && self.limitations == DEC_046_FREE_DOF_FORCE_MOMENT_LIMITATIONS
    }

    pub fn is_accepted_multisupport_free_dof_force_moment_policy(&self) -> bool {
        self.policy_ref == DEC_046_MULTISUPPORT_FREE_DOF_FORCE_MOMENT_POLICY_REF
            && self.nonlinear_class == "multi_support_multi_dof"
            && self.force_residual_basis == DEC_046_FREE_DOF_FORCE_RESIDUAL_BASIS
            && self.force_residual_unit == DEC_046_FREE_DOF_FORCE_RESIDUAL_UNIT
            && self.force_residual_dimension == DEC_046_FREE_DOF_FORCE_RESIDUAL_DIMENSION
            && self.force_absolute_limit == DEC_046_FREE_DOF_FORCE_ABSOLUTE_LIMIT
            && self.moment_residual_basis == DEC_046_FREE_DOF_MOMENT_RESIDUAL_BASIS
            && self.moment_residual_unit == DEC_046_FREE_DOF_MOMENT_RESIDUAL_UNIT
            && self.moment_residual_dimension == DEC_046_FREE_DOF_MOMENT_RESIDUAL_DIMENSION
            && self.moment_absolute_limit == DEC_046_FREE_DOF_MOMENT_ABSOLUTE_LIMIT
            && self.status == ConvergencePolicyStatus::Accepted
            && !self.evidence_fixture_ids.is_empty()
            && self.limitations == DEC_046_MULTISUPPORT_FREE_DOF_FORCE_MOMENT_LIMITATIONS
    }
}

impl WorkResidualPolicyEntry {
    pub fn is_accepted_free_dof_work_policy(&self) -> bool {
        self.policy_ref == DEC_046_FREE_DOF_WORK_POLICY_REF
            && self.work_residual_basis == DEC_046_FREE_DOF_WORK_RESIDUAL_BASIS
            && self.work_residual_unit == DEC_046_FREE_DOF_WORK_RESIDUAL_UNIT
            && self.work_residual_dimension == DEC_046_FREE_DOF_WORK_RESIDUAL_DIMENSION
            && self.work_absolute_limit == DEC_046_FREE_DOF_WORK_ABSOLUTE_LIMIT
            && self.status == ConvergencePolicyStatus::Accepted
            && !self.evidence_fixture_ids.is_empty()
            && self.limitations == DEC_046_FREE_DOF_WORK_LIMITATIONS
    }

    pub fn is_accepted_multisupport_free_dof_work_policy(&self) -> bool {
        self.policy_ref == DEC_046_MULTISUPPORT_FREE_DOF_WORK_POLICY_REF
            && self.nonlinear_class == "multi_support_multi_dof"
            && self.work_residual_basis == DEC_046_FREE_DOF_WORK_RESIDUAL_BASIS
            && self.work_residual_unit == DEC_046_FREE_DOF_WORK_RESIDUAL_UNIT
            && self.work_residual_dimension == DEC_046_FREE_DOF_WORK_RESIDUAL_DIMENSION
            && self.work_absolute_limit == DEC_046_FREE_DOF_WORK_ABSOLUTE_LIMIT
            && self.status == ConvergencePolicyStatus::Accepted
            && !self.evidence_fixture_ids.is_empty()
            && self.limitations == DEC_046_MULTISUPPORT_FREE_DOF_WORK_LIMITATIONS
    }
}

impl DisplacementReactionDeltaObservationEntry {
    pub fn is_tbd_seed_displacement_reaction_delta_observation(&self) -> bool {
        self.observation_ref == DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF
            && self.threshold_status == ConvergencePolicyStatus::Tbd
            && self.has_expected_axes()
            && !self.evidence_fixture_ids.is_empty()
            && self.limitations == DEC_046_DISPLACEMENT_REACTION_DELTA_LIMITATIONS
    }

    pub fn is_tbd_multisupport_displacement_reaction_delta_observation(&self) -> bool {
        self.observation_ref == DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF
            && self.nonlinear_class == "multi_support_multi_dof"
            && self.threshold_status == ConvergencePolicyStatus::Tbd
            && self.has_expected_axes()
            && !self.evidence_fixture_ids.is_empty()
            && self.limitations == DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_LIMITATIONS
    }

    fn has_expected_axes(&self) -> bool {
        self.translation_delta_basis == DEC_046_TRANSLATION_DELTA_BASIS
            && self.translation_delta_unit == DEC_046_TRANSLATION_DELTA_UNIT
            && self.translation_delta_dimension == DEC_046_TRANSLATION_DELTA_DIMENSION
            && self.rotation_delta_basis == DEC_046_ROTATION_DELTA_BASIS
            && self.rotation_delta_unit == DEC_046_ROTATION_DELTA_UNIT
            && self.rotation_delta_dimension == DEC_046_ROTATION_DELTA_DIMENSION
            && self.force_reaction_delta_basis == DEC_046_FORCE_REACTION_DELTA_BASIS
            && self.force_reaction_delta_unit == DEC_046_FORCE_REACTION_DELTA_UNIT
            && self.force_reaction_delta_dimension == DEC_046_FORCE_REACTION_DELTA_DIMENSION
            && self.moment_reaction_delta_basis == DEC_046_MOMENT_REACTION_DELTA_BASIS
            && self.moment_reaction_delta_unit == DEC_046_MOMENT_REACTION_DELTA_UNIT
            && self.moment_reaction_delta_dimension == DEC_046_MOMENT_REACTION_DELTA_DIMENSION
    }
}

impl DisplacementReactionDeltaPolicyEntry {
    pub fn is_accepted_seed_displacement_reaction_delta_policy(&self) -> bool {
        let Some((translation_limit, rotation_limit, force_reaction_limit, moment_reaction_limit)) =
            seed_displacement_reaction_delta_limits(self.nonlinear_class)
        else {
            return false;
        };

        self.policy_ref == DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_REF
            && self.observation_ref == DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF
            && self.status == ConvergencePolicyStatus::Accepted
            && self.has_expected_axes()
            && self.translation_delta_absolute_limit == translation_limit
            && self.rotation_delta_absolute_limit == rotation_limit
            && self.force_reaction_delta_absolute_limit == force_reaction_limit
            && self.moment_reaction_delta_absolute_limit == moment_reaction_limit
            && !self.evidence_fixture_ids.is_empty()
            && self.limitations == DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_LIMITATIONS
    }

    pub fn is_accepted_multisupport_displacement_reaction_delta_policy(&self) -> bool {
        self.policy_ref == DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_POLICY_REF
            && self.observation_ref
                == DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF
            && self.nonlinear_class == "multi_support_multi_dof"
            && self.status == ConvergencePolicyStatus::Accepted
            && self.has_expected_axes()
            && self.translation_delta_absolute_limit
                == DEC_046_MULTISUPPORT_TRANSLATION_DELTA_ABSOLUTE_LIMIT
            && self.rotation_delta_absolute_limit
                == DEC_046_MULTISUPPORT_ROTATION_DELTA_ABSOLUTE_LIMIT
            && self.force_reaction_delta_absolute_limit
                == DEC_046_MULTISUPPORT_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT
            && self.moment_reaction_delta_absolute_limit
                == DEC_046_MULTISUPPORT_MOMENT_REACTION_DELTA_ABSOLUTE_LIMIT
            && !self.evidence_fixture_ids.is_empty()
            && self.limitations
                == DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_POLICY_LIMITATIONS
    }

    fn has_expected_axes(&self) -> bool {
        self.translation_delta_basis == DEC_046_TRANSLATION_DELTA_BASIS
            && self.translation_delta_unit == DEC_046_TRANSLATION_DELTA_UNIT
            && self.translation_delta_dimension == DEC_046_TRANSLATION_DELTA_DIMENSION
            && self.rotation_delta_basis == DEC_046_ROTATION_DELTA_BASIS
            && self.rotation_delta_unit == DEC_046_ROTATION_DELTA_UNIT
            && self.rotation_delta_dimension == DEC_046_ROTATION_DELTA_DIMENSION
            && self.force_reaction_delta_basis == DEC_046_FORCE_REACTION_DELTA_BASIS
            && self.force_reaction_delta_unit == DEC_046_FORCE_REACTION_DELTA_UNIT
            && self.force_reaction_delta_dimension == DEC_046_FORCE_REACTION_DELTA_DIMENSION
            && self.moment_reaction_delta_basis == DEC_046_MOMENT_REACTION_DELTA_BASIS
            && self.moment_reaction_delta_unit == DEC_046_MOMENT_REACTION_DELTA_UNIT
            && self.moment_reaction_delta_dimension == DEC_046_MOMENT_REACTION_DELTA_DIMENSION
    }
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

impl AssembledNonlinearRegressionCase {
    pub fn run(
        &self,
    ) -> Result<
        NonlinearFrameSolveResult,
        open_pipe_stress_nonlinear_integration::NonlinearIntegrationError,
    > {
        solve_active_set_frame(&self.input)
    }

    pub fn tolerance_policy_is_unresolved(&self) -> bool {
        self.observations
            .iter()
            .all(|observation| observation.tolerance_policy.is_none())
    }

    pub fn uses_governed_convergence_policy(&self) -> bool {
        self.input.convergence.policy_ref == DEC_046_ACTIVE_SET_COUNT_POLICY_REF
            && self.input.convergence.policy_status == ConvergencePolicyStatus::Accepted
            && self.input.convergence.residual_tolerance
                == DEC_046_ACTIVE_SET_COUNT_RELATIVE_TOLERANCE
            && self.input.convergence.absolute_residual_floor
                == DEC_046_ACTIVE_SET_COUNT_ABSOLUTE_FLOOR
            && self.input.convergence.max_iterations == DEC_046_ACTIVE_SET_COUNT_MAX_ITERATIONS
            && self.observations.iter().any(|observation| {
                observation.name == "final_residual"
                    && observation.tolerance_policy == Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF)
            })
    }

    pub fn uses_governed_multisupport_convergence_policy(&self) -> bool {
        self.input.convergence.policy_ref == DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF
            && self.input.convergence.policy_status == ConvergencePolicyStatus::Accepted
            && self.input.convergence.residual_tolerance
                == DEC_046_ACTIVE_SET_COUNT_RELATIVE_TOLERANCE
            && self.input.convergence.absolute_residual_floor
                == DEC_046_ACTIVE_SET_COUNT_ABSOLUTE_FLOOR
            && self.input.convergence.max_iterations == DEC_046_ACTIVE_SET_COUNT_MAX_ITERATIONS
            && self.observations.iter().any(|observation| {
                observation.name == "final_residual"
                    && observation.tolerance_policy
                        == Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF)
            })
    }

    pub fn has_dimensioned_observations(&self) -> bool {
        self.observations.iter().all(|observation| {
            observation.value.is_finite()
                && !observation.unit.is_empty()
                && CANONICAL_DIMENSIONS.contains(&observation.dimension)
        }) && self.unit_basis.is_explicit_fixture_basis()
    }

    pub fn matches_expected_outcome(&self) -> bool {
        let solve = match self.run() {
            Ok(solve) => solve,
            Err(_) => return false,
        };
        let expected_states: Vec<SupportStateRecord> = self
            .expected_final_states
            .iter()
            .map(|expected| SupportStateRecord::new(expected.support_id, expected.state))
            .collect();
        let final_residual = solve
            .iterations
            .last()
            .map(|iteration| iteration.active_set.residual_norm)
            .unwrap_or(f64::NAN);
        let diagnostic_codes = solve
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>();

        solve.final_states == expected_states
            && solve.iterations.len() == self.expected_iteration_count
            && final_residual == self.expected_final_residual_norm
            && solve.converged == self.expected_converged
            && diagnostic_codes == self.expected_diagnostic_codes
    }

    pub fn convergence_observation(
        &self,
    ) -> Result<
        ConvergenceObservation,
        open_pipe_stress_nonlinear_integration::NonlinearIntegrationError,
    > {
        let solve = self.run()?;
        let final_residual_norm = solve
            .iterations
            .last()
            .map(|iteration| iteration.active_set.residual_norm)
            .unwrap_or(f64::NAN);

        Ok(ConvergenceObservation {
            fixture_id: self.fixture_id,
            family: self.family,
            nonlinear_class: convergence_class_label(self.fixture_id, self.family),
            policy_ref: solve.policy_ref,
            policy_status: self.input.convergence.policy_status,
            max_iterations: self.input.convergence.max_iterations,
            observed_iteration_count: solve.iterations.len(),
            final_residual_norm,
            residual_unit: "count",
            residual_dimension: "dimensionless",
            observed_converged: solve.converged,
            diagnostic_codes: solve
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code)
                .collect(),
        })
    }

    pub fn force_displacement_residual_observation(
        &self,
    ) -> Result<
        ForceDisplacementResidualObservation,
        open_pipe_stress_nonlinear_integration::NonlinearIntegrationError,
    > {
        self.force_displacement_residual_observation_with_policy(
            Some(DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF),
            Some(DEC_046_FREE_DOF_WORK_POLICY_REF),
            Some(DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF),
            Some(DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_REF),
        )
    }

    pub fn observation_only_force_displacement_residual(
        &self,
    ) -> Result<
        ForceDisplacementResidualObservation,
        open_pipe_stress_nonlinear_integration::NonlinearIntegrationError,
    > {
        self.force_displacement_residual_observation_with_policy(None, None, None, None)
    }

    pub fn multisupport_force_displacement_residual_observation(
        &self,
    ) -> Result<
        ForceDisplacementResidualObservation,
        open_pipe_stress_nonlinear_integration::NonlinearIntegrationError,
    > {
        self.force_displacement_residual_observation_with_policy(
            Some(DEC_046_MULTISUPPORT_FREE_DOF_FORCE_MOMENT_POLICY_REF),
            Some(DEC_046_MULTISUPPORT_FREE_DOF_WORK_POLICY_REF),
            Some(DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF),
            Some(DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_POLICY_REF),
        )
    }

    fn force_displacement_residual_observation_with_policy(
        &self,
        force_moment_threshold_policy: Option<&'static str>,
        work_threshold_policy: Option<&'static str>,
        displacement_reaction_delta_observation_ref: Option<&'static str>,
        displacement_reaction_delta_threshold_policy: Option<&'static str>,
    ) -> Result<
        ForceDisplacementResidualObservation,
        open_pipe_stress_nonlinear_integration::NonlinearIntegrationError,
    > {
        let solve = self.run()?;
        let final_residuals = solve
            .iterations
            .last()
            .expect("assembled solve always records at least one iteration")
            .residuals
            .clone();

        Ok(ForceDisplacementResidualObservation {
            fixture_id: self.fixture_id,
            family: self.family,
            nonlinear_class: convergence_class_label(self.fixture_id, self.family),
            policy_ref: solve.policy_ref,
            observed_iteration_count: solve.iterations.len(),
            max_abs_translation_delta_from_previous: final_residuals
                .max_abs_translation_delta_from_previous
                .map(|value| value * 1000.0),
            translation_delta_unit: "mm",
            max_abs_rotation_delta_from_previous: final_residuals
                .max_abs_rotation_delta_from_previous,
            rotation_delta_unit: "rad",
            max_abs_force_reaction_delta_from_previous: final_residuals
                .max_abs_force_reaction_delta_from_previous,
            force_reaction_delta_unit: "N",
            max_abs_moment_reaction_delta_from_previous: final_residuals
                .max_abs_moment_reaction_delta_from_previous,
            moment_reaction_delta_unit: "N-m",
            max_abs_free_dof_force_residual: final_residuals.max_abs_free_dof_force_residual,
            free_dof_force_residual_unit: "N",
            max_abs_free_dof_moment_residual: final_residuals.max_abs_free_dof_moment_residual,
            free_dof_moment_residual_unit: "N-m",
            max_abs_free_dof_work_residual: final_residuals.max_abs_free_dof_work_residual,
            free_dof_work_residual_unit: "N-m",
            displacement_reaction_delta_observation_ref,
            displacement_reaction_delta_threshold_policy,
            free_dof_force_moment_threshold_policy: force_moment_threshold_policy,
            free_dof_work_threshold_policy: work_threshold_policy,
        })
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

pub fn assembled_fixture_inventory() -> Vec<AssembledNonlinearRegressionCase> {
    vec![
        assembled_one_way_deactivation_fixture(),
        assembled_gap_closure_fixture(),
        assembled_lift_off_loss_fixture(),
        assembled_friction_sticking_fixture(),
        assembled_friction_sliding_fixture(),
        assembled_friction_derived_normal_fixture(),
    ]
}

pub fn assembled_convergence_observations() -> Vec<ConvergenceObservation> {
    assembled_fixture_inventory()
        .iter()
        .map(|fixture| {
            fixture
                .convergence_observation()
                .expect("assembled convergence observation fixtures remain valid")
        })
        .collect()
}

pub fn assembled_force_displacement_residual_observations(
) -> Vec<ForceDisplacementResidualObservation> {
    assembled_fixture_inventory()
        .iter()
        .map(|fixture| {
            fixture
                .force_displacement_residual_observation()
                .expect("assembled force/displacement residual observation fixtures remain valid")
        })
        .collect()
}

pub fn assembled_multisupport_depth_inventory() -> Vec<AssembledNonlinearRegressionCase> {
    vec![assembled_multi_dof_multi_support_observation_fixture()]
}

pub fn assembled_multisupport_acceptance_inventory() -> Vec<AssembledNonlinearRegressionCase> {
    vec![
        assembled_multi_dof_multi_support_acceptance_fixture(),
        assembled_multi_dof_gap_lift_off_acceptance_fixture(),
        assembled_multi_dof_friction_gap_acceptance_fixture(),
        assembled_multi_dof_three_support_acceptance_fixture(),
        assembled_multi_dof_rotational_acceptance_fixture(),
        assembled_multi_dof_derived_normal_gap_acceptance_fixture(),
        assembled_multi_dof_derived_normal_rotational_acceptance_fixture(),
        assembled_multi_dof_cascade_gap_lift_off_acceptance_fixture(),
        assembled_multi_dof_negative_gap_acceptance_fixture(),
    ]
}

pub fn assembled_multisupport_depth_convergence_observations() -> Vec<ConvergenceObservation> {
    assembled_multisupport_depth_inventory()
        .iter()
        .map(|fixture| {
            fixture
                .convergence_observation()
                .expect("multi-support depth observation fixtures remain valid")
        })
        .collect()
}

pub fn assembled_multisupport_depth_residual_observations(
) -> Vec<ForceDisplacementResidualObservation> {
    assembled_multisupport_depth_inventory()
        .iter()
        .map(|fixture| {
            fixture
                .observation_only_force_displacement_residual()
                .expect("multi-support depth residual observation fixtures remain valid")
        })
        .collect()
}

pub fn assembled_multisupport_acceptance_convergence_observations() -> Vec<ConvergenceObservation> {
    assembled_multisupport_acceptance_inventory()
        .iter()
        .map(|fixture| {
            fixture
                .convergence_observation()
                .expect("multi-support acceptance fixtures remain valid")
        })
        .collect()
}

pub fn assembled_multisupport_acceptance_residual_observations(
) -> Vec<ForceDisplacementResidualObservation> {
    assembled_multisupport_acceptance_inventory()
        .iter()
        .map(|fixture| {
            fixture
                .multisupport_force_displacement_residual_observation()
                .expect("multi-support acceptance residual observation fixtures remain valid")
        })
        .collect()
}

pub fn governed_convergence_policy_entries() -> Vec<ConvergencePolicyEntry> {
    vec![
        ConvergencePolicyEntry {
            policy_ref: DEC_046_ACTIVE_SET_COUNT_POLICY_REF,
            nonlinear_class: "one_way",
            residual_basis: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_BASIS,
            residual_unit: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_UNIT,
            residual_dimension: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_DIMENSION,
            relative_residual_tolerance: DEC_046_ACTIVE_SET_COUNT_RELATIVE_TOLERANCE,
            absolute_residual_floor: DEC_046_ACTIVE_SET_COUNT_ABSOLUTE_FLOOR,
            max_iterations: DEC_046_ACTIVE_SET_COUNT_MAX_ITERATIONS,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &["NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL"],
            limitations: DEC_046_ACTIVE_SET_COUNT_LIMITATIONS,
        },
        ConvergencePolicyEntry {
            policy_ref: DEC_046_ACTIVE_SET_COUNT_POLICY_REF,
            nonlinear_class: "gap",
            residual_basis: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_BASIS,
            residual_unit: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_UNIT,
            residual_dimension: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_DIMENSION,
            relative_residual_tolerance: DEC_046_ACTIVE_SET_COUNT_RELATIVE_TOLERANCE,
            absolute_residual_floor: DEC_046_ACTIVE_SET_COUNT_ABSOLUTE_FLOOR,
            max_iterations: DEC_046_ACTIVE_SET_COUNT_MAX_ITERATIONS,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &["NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL"],
            limitations: DEC_046_ACTIVE_SET_COUNT_LIMITATIONS,
        },
        ConvergencePolicyEntry {
            policy_ref: DEC_046_ACTIVE_SET_COUNT_POLICY_REF,
            nonlinear_class: "lift_off",
            residual_basis: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_BASIS,
            residual_unit: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_UNIT,
            residual_dimension: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_DIMENSION,
            relative_residual_tolerance: DEC_046_ACTIVE_SET_COUNT_RELATIVE_TOLERANCE,
            absolute_residual_floor: DEC_046_ACTIVE_SET_COUNT_ABSOLUTE_FLOOR,
            max_iterations: DEC_046_ACTIVE_SET_COUNT_MAX_ITERATIONS,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &["NL-ASSEMBLED-LIFT-OFF-ORIGINAL"],
            limitations: DEC_046_ACTIVE_SET_COUNT_LIMITATIONS,
        },
        ConvergencePolicyEntry {
            policy_ref: DEC_046_ACTIVE_SET_COUNT_POLICY_REF,
            nonlinear_class: "friction",
            residual_basis: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_BASIS,
            residual_unit: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_UNIT,
            residual_dimension: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_DIMENSION,
            relative_residual_tolerance: DEC_046_ACTIVE_SET_COUNT_RELATIVE_TOLERANCE,
            absolute_residual_floor: DEC_046_ACTIVE_SET_COUNT_ABSOLUTE_FLOOR,
            max_iterations: DEC_046_ACTIVE_SET_COUNT_MAX_ITERATIONS,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &[
                "NL-ASSEMBLED-FRICTION-STICK-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL",
            ],
            limitations: DEC_046_ACTIVE_SET_COUNT_LIMITATIONS,
        },
    ]
}

pub fn governed_free_dof_force_moment_policy_entries() -> Vec<ForceMomentResidualPolicyEntry> {
    vec![
        ForceMomentResidualPolicyEntry {
            policy_ref: DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF,
            nonlinear_class: "one_way",
            force_residual_basis: DEC_046_FREE_DOF_FORCE_RESIDUAL_BASIS,
            force_residual_unit: DEC_046_FREE_DOF_FORCE_RESIDUAL_UNIT,
            force_residual_dimension: DEC_046_FREE_DOF_FORCE_RESIDUAL_DIMENSION,
            force_absolute_limit: DEC_046_FREE_DOF_FORCE_ABSOLUTE_LIMIT,
            moment_residual_basis: DEC_046_FREE_DOF_MOMENT_RESIDUAL_BASIS,
            moment_residual_unit: DEC_046_FREE_DOF_MOMENT_RESIDUAL_UNIT,
            moment_residual_dimension: DEC_046_FREE_DOF_MOMENT_RESIDUAL_DIMENSION,
            moment_absolute_limit: DEC_046_FREE_DOF_MOMENT_ABSOLUTE_LIMIT,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &["NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL"],
            limitations: DEC_046_FREE_DOF_FORCE_MOMENT_LIMITATIONS,
        },
        ForceMomentResidualPolicyEntry {
            policy_ref: DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF,
            nonlinear_class: "gap",
            force_residual_basis: DEC_046_FREE_DOF_FORCE_RESIDUAL_BASIS,
            force_residual_unit: DEC_046_FREE_DOF_FORCE_RESIDUAL_UNIT,
            force_residual_dimension: DEC_046_FREE_DOF_FORCE_RESIDUAL_DIMENSION,
            force_absolute_limit: DEC_046_FREE_DOF_FORCE_ABSOLUTE_LIMIT,
            moment_residual_basis: DEC_046_FREE_DOF_MOMENT_RESIDUAL_BASIS,
            moment_residual_unit: DEC_046_FREE_DOF_MOMENT_RESIDUAL_UNIT,
            moment_residual_dimension: DEC_046_FREE_DOF_MOMENT_RESIDUAL_DIMENSION,
            moment_absolute_limit: DEC_046_FREE_DOF_MOMENT_ABSOLUTE_LIMIT,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &["NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL"],
            limitations: DEC_046_FREE_DOF_FORCE_MOMENT_LIMITATIONS,
        },
        ForceMomentResidualPolicyEntry {
            policy_ref: DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF,
            nonlinear_class: "lift_off",
            force_residual_basis: DEC_046_FREE_DOF_FORCE_RESIDUAL_BASIS,
            force_residual_unit: DEC_046_FREE_DOF_FORCE_RESIDUAL_UNIT,
            force_residual_dimension: DEC_046_FREE_DOF_FORCE_RESIDUAL_DIMENSION,
            force_absolute_limit: DEC_046_FREE_DOF_FORCE_ABSOLUTE_LIMIT,
            moment_residual_basis: DEC_046_FREE_DOF_MOMENT_RESIDUAL_BASIS,
            moment_residual_unit: DEC_046_FREE_DOF_MOMENT_RESIDUAL_UNIT,
            moment_residual_dimension: DEC_046_FREE_DOF_MOMENT_RESIDUAL_DIMENSION,
            moment_absolute_limit: DEC_046_FREE_DOF_MOMENT_ABSOLUTE_LIMIT,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &["NL-ASSEMBLED-LIFT-OFF-ORIGINAL"],
            limitations: DEC_046_FREE_DOF_FORCE_MOMENT_LIMITATIONS,
        },
        ForceMomentResidualPolicyEntry {
            policy_ref: DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF,
            nonlinear_class: "friction",
            force_residual_basis: DEC_046_FREE_DOF_FORCE_RESIDUAL_BASIS,
            force_residual_unit: DEC_046_FREE_DOF_FORCE_RESIDUAL_UNIT,
            force_residual_dimension: DEC_046_FREE_DOF_FORCE_RESIDUAL_DIMENSION,
            force_absolute_limit: DEC_046_FREE_DOF_FORCE_ABSOLUTE_LIMIT,
            moment_residual_basis: DEC_046_FREE_DOF_MOMENT_RESIDUAL_BASIS,
            moment_residual_unit: DEC_046_FREE_DOF_MOMENT_RESIDUAL_UNIT,
            moment_residual_dimension: DEC_046_FREE_DOF_MOMENT_RESIDUAL_DIMENSION,
            moment_absolute_limit: DEC_046_FREE_DOF_MOMENT_ABSOLUTE_LIMIT,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &[
                "NL-ASSEMBLED-FRICTION-STICK-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL",
            ],
            limitations: DEC_046_FREE_DOF_FORCE_MOMENT_LIMITATIONS,
        },
    ]
}

pub fn governed_free_dof_work_policy_entries() -> Vec<WorkResidualPolicyEntry> {
    vec![
        WorkResidualPolicyEntry {
            policy_ref: DEC_046_FREE_DOF_WORK_POLICY_REF,
            nonlinear_class: "one_way",
            work_residual_basis: DEC_046_FREE_DOF_WORK_RESIDUAL_BASIS,
            work_residual_unit: DEC_046_FREE_DOF_WORK_RESIDUAL_UNIT,
            work_residual_dimension: DEC_046_FREE_DOF_WORK_RESIDUAL_DIMENSION,
            work_absolute_limit: DEC_046_FREE_DOF_WORK_ABSOLUTE_LIMIT,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &["NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL"],
            limitations: DEC_046_FREE_DOF_WORK_LIMITATIONS,
        },
        WorkResidualPolicyEntry {
            policy_ref: DEC_046_FREE_DOF_WORK_POLICY_REF,
            nonlinear_class: "gap",
            work_residual_basis: DEC_046_FREE_DOF_WORK_RESIDUAL_BASIS,
            work_residual_unit: DEC_046_FREE_DOF_WORK_RESIDUAL_UNIT,
            work_residual_dimension: DEC_046_FREE_DOF_WORK_RESIDUAL_DIMENSION,
            work_absolute_limit: DEC_046_FREE_DOF_WORK_ABSOLUTE_LIMIT,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &["NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL"],
            limitations: DEC_046_FREE_DOF_WORK_LIMITATIONS,
        },
        WorkResidualPolicyEntry {
            policy_ref: DEC_046_FREE_DOF_WORK_POLICY_REF,
            nonlinear_class: "lift_off",
            work_residual_basis: DEC_046_FREE_DOF_WORK_RESIDUAL_BASIS,
            work_residual_unit: DEC_046_FREE_DOF_WORK_RESIDUAL_UNIT,
            work_residual_dimension: DEC_046_FREE_DOF_WORK_RESIDUAL_DIMENSION,
            work_absolute_limit: DEC_046_FREE_DOF_WORK_ABSOLUTE_LIMIT,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &["NL-ASSEMBLED-LIFT-OFF-ORIGINAL"],
            limitations: DEC_046_FREE_DOF_WORK_LIMITATIONS,
        },
        WorkResidualPolicyEntry {
            policy_ref: DEC_046_FREE_DOF_WORK_POLICY_REF,
            nonlinear_class: "friction",
            work_residual_basis: DEC_046_FREE_DOF_WORK_RESIDUAL_BASIS,
            work_residual_unit: DEC_046_FREE_DOF_WORK_RESIDUAL_UNIT,
            work_residual_dimension: DEC_046_FREE_DOF_WORK_RESIDUAL_DIMENSION,
            work_absolute_limit: DEC_046_FREE_DOF_WORK_ABSOLUTE_LIMIT,
            status: ConvergencePolicyStatus::Accepted,
            evidence_fixture_ids: &[
                "NL-ASSEMBLED-FRICTION-STICK-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL",
            ],
            limitations: DEC_046_FREE_DOF_WORK_LIMITATIONS,
        },
    ]
}

pub fn governed_multisupport_convergence_policy_entries() -> Vec<ConvergencePolicyEntry> {
    vec![ConvergencePolicyEntry {
        policy_ref: DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF,
        nonlinear_class: "multi_support_multi_dof",
        residual_basis: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_BASIS,
        residual_unit: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_UNIT,
        residual_dimension: DEC_046_ACTIVE_SET_COUNT_RESIDUAL_DIMENSION,
        relative_residual_tolerance: DEC_046_ACTIVE_SET_COUNT_RELATIVE_TOLERANCE,
        absolute_residual_floor: DEC_046_ACTIVE_SET_COUNT_ABSOLUTE_FLOOR,
        max_iterations: DEC_046_ACTIVE_SET_COUNT_MAX_ITERATIONS,
        status: ConvergencePolicyStatus::Accepted,
        evidence_fixture_ids: DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS,
        limitations: DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_LIMITATIONS,
    }]
}

pub fn governed_multisupport_free_dof_force_moment_policy_entries(
) -> Vec<ForceMomentResidualPolicyEntry> {
    vec![ForceMomentResidualPolicyEntry {
        policy_ref: DEC_046_MULTISUPPORT_FREE_DOF_FORCE_MOMENT_POLICY_REF,
        nonlinear_class: "multi_support_multi_dof",
        force_residual_basis: DEC_046_FREE_DOF_FORCE_RESIDUAL_BASIS,
        force_residual_unit: DEC_046_FREE_DOF_FORCE_RESIDUAL_UNIT,
        force_residual_dimension: DEC_046_FREE_DOF_FORCE_RESIDUAL_DIMENSION,
        force_absolute_limit: DEC_046_FREE_DOF_FORCE_ABSOLUTE_LIMIT,
        moment_residual_basis: DEC_046_FREE_DOF_MOMENT_RESIDUAL_BASIS,
        moment_residual_unit: DEC_046_FREE_DOF_MOMENT_RESIDUAL_UNIT,
        moment_residual_dimension: DEC_046_FREE_DOF_MOMENT_RESIDUAL_DIMENSION,
        moment_absolute_limit: DEC_046_FREE_DOF_MOMENT_ABSOLUTE_LIMIT,
        status: ConvergencePolicyStatus::Accepted,
        evidence_fixture_ids: DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS,
        limitations: DEC_046_MULTISUPPORT_FREE_DOF_FORCE_MOMENT_LIMITATIONS,
    }]
}

pub fn governed_multisupport_free_dof_work_policy_entries() -> Vec<WorkResidualPolicyEntry> {
    vec![WorkResidualPolicyEntry {
        policy_ref: DEC_046_MULTISUPPORT_FREE_DOF_WORK_POLICY_REF,
        nonlinear_class: "multi_support_multi_dof",
        work_residual_basis: DEC_046_FREE_DOF_WORK_RESIDUAL_BASIS,
        work_residual_unit: DEC_046_FREE_DOF_WORK_RESIDUAL_UNIT,
        work_residual_dimension: DEC_046_FREE_DOF_WORK_RESIDUAL_DIMENSION,
        work_absolute_limit: DEC_046_FREE_DOF_WORK_ABSOLUTE_LIMIT,
        status: ConvergencePolicyStatus::Accepted,
        evidence_fixture_ids: DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS,
        limitations: DEC_046_MULTISUPPORT_FREE_DOF_WORK_LIMITATIONS,
    }]
}

pub fn displacement_reaction_delta_observation_entries(
) -> Vec<DisplacementReactionDeltaObservationEntry> {
    vec![
        displacement_reaction_delta_observation_entry(
            DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF,
            "one_way",
            &["NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL"],
            DEC_046_DISPLACEMENT_REACTION_DELTA_LIMITATIONS,
        ),
        displacement_reaction_delta_observation_entry(
            DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF,
            "gap",
            &["NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL"],
            DEC_046_DISPLACEMENT_REACTION_DELTA_LIMITATIONS,
        ),
        displacement_reaction_delta_observation_entry(
            DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF,
            "lift_off",
            &["NL-ASSEMBLED-LIFT-OFF-ORIGINAL"],
            DEC_046_DISPLACEMENT_REACTION_DELTA_LIMITATIONS,
        ),
        displacement_reaction_delta_observation_entry(
            DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF,
            "friction",
            &[
                "NL-ASSEMBLED-FRICTION-STICK-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL",
            ],
            DEC_046_DISPLACEMENT_REACTION_DELTA_LIMITATIONS,
        ),
    ]
}

pub fn multisupport_displacement_reaction_delta_observation_entries(
) -> Vec<DisplacementReactionDeltaObservationEntry> {
    vec![displacement_reaction_delta_observation_entry(
        DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF,
        "multi_support_multi_dof",
        DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS,
        DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_LIMITATIONS,
    )]
}

pub fn governed_displacement_reaction_delta_policy_entries(
) -> Vec<DisplacementReactionDeltaPolicyEntry> {
    vec![
        displacement_reaction_delta_policy_entry(
            DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_REF,
            DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF,
            "one_way",
            seed_displacement_reaction_delta_limits("one_way")
                .expect("one_way seed displacement/reaction delta limits exist"),
            &["NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL"],
            DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_LIMITATIONS,
        ),
        displacement_reaction_delta_policy_entry(
            DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_REF,
            DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF,
            "gap",
            seed_displacement_reaction_delta_limits("gap")
                .expect("gap seed displacement/reaction delta limits exist"),
            &["NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL"],
            DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_LIMITATIONS,
        ),
        displacement_reaction_delta_policy_entry(
            DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_REF,
            DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF,
            "lift_off",
            seed_displacement_reaction_delta_limits("lift_off")
                .expect("lift-off seed displacement/reaction delta limits exist"),
            &["NL-ASSEMBLED-LIFT-OFF-ORIGINAL"],
            DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_LIMITATIONS,
        ),
        displacement_reaction_delta_policy_entry(
            DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_REF,
            DEC_046_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF,
            "friction",
            seed_displacement_reaction_delta_limits("friction")
                .expect("friction seed displacement/reaction delta limits exist"),
            &[
                "NL-ASSEMBLED-FRICTION-STICK-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL",
            ],
            DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_LIMITATIONS,
        ),
    ]
}

pub fn governed_multisupport_displacement_reaction_delta_policy_entries(
) -> Vec<DisplacementReactionDeltaPolicyEntry> {
    vec![displacement_reaction_delta_policy_entry(
        DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_POLICY_REF,
        DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF,
        "multi_support_multi_dof",
        (
            DEC_046_MULTISUPPORT_TRANSLATION_DELTA_ABSOLUTE_LIMIT,
            DEC_046_MULTISUPPORT_ROTATION_DELTA_ABSOLUTE_LIMIT,
            DEC_046_MULTISUPPORT_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT,
            DEC_046_MULTISUPPORT_MOMENT_REACTION_DELTA_ABSOLUTE_LIMIT,
        ),
        DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS,
        DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_POLICY_LIMITATIONS,
    )]
}

fn displacement_reaction_delta_observation_entry(
    observation_ref: &'static str,
    nonlinear_class: &'static str,
    evidence_fixture_ids: &'static [&'static str],
    limitations: &'static [&'static str],
) -> DisplacementReactionDeltaObservationEntry {
    DisplacementReactionDeltaObservationEntry {
        observation_ref,
        nonlinear_class,
        threshold_status: ConvergencePolicyStatus::Tbd,
        translation_delta_basis: DEC_046_TRANSLATION_DELTA_BASIS,
        translation_delta_unit: DEC_046_TRANSLATION_DELTA_UNIT,
        translation_delta_dimension: DEC_046_TRANSLATION_DELTA_DIMENSION,
        rotation_delta_basis: DEC_046_ROTATION_DELTA_BASIS,
        rotation_delta_unit: DEC_046_ROTATION_DELTA_UNIT,
        rotation_delta_dimension: DEC_046_ROTATION_DELTA_DIMENSION,
        force_reaction_delta_basis: DEC_046_FORCE_REACTION_DELTA_BASIS,
        force_reaction_delta_unit: DEC_046_FORCE_REACTION_DELTA_UNIT,
        force_reaction_delta_dimension: DEC_046_FORCE_REACTION_DELTA_DIMENSION,
        moment_reaction_delta_basis: DEC_046_MOMENT_REACTION_DELTA_BASIS,
        moment_reaction_delta_unit: DEC_046_MOMENT_REACTION_DELTA_UNIT,
        moment_reaction_delta_dimension: DEC_046_MOMENT_REACTION_DELTA_DIMENSION,
        evidence_fixture_ids,
        limitations,
    }
}

fn displacement_reaction_delta_policy_entry(
    policy_ref: &'static str,
    observation_ref: &'static str,
    nonlinear_class: &'static str,
    (
        translation_delta_absolute_limit,
        rotation_delta_absolute_limit,
        force_reaction_delta_absolute_limit,
        moment_reaction_delta_absolute_limit,
    ): (f64, f64, f64, f64),
    evidence_fixture_ids: &'static [&'static str],
    limitations: &'static [&'static str],
) -> DisplacementReactionDeltaPolicyEntry {
    DisplacementReactionDeltaPolicyEntry {
        policy_ref,
        observation_ref,
        nonlinear_class,
        translation_delta_basis: DEC_046_TRANSLATION_DELTA_BASIS,
        translation_delta_unit: DEC_046_TRANSLATION_DELTA_UNIT,
        translation_delta_dimension: DEC_046_TRANSLATION_DELTA_DIMENSION,
        translation_delta_absolute_limit,
        rotation_delta_basis: DEC_046_ROTATION_DELTA_BASIS,
        rotation_delta_unit: DEC_046_ROTATION_DELTA_UNIT,
        rotation_delta_dimension: DEC_046_ROTATION_DELTA_DIMENSION,
        rotation_delta_absolute_limit,
        force_reaction_delta_basis: DEC_046_FORCE_REACTION_DELTA_BASIS,
        force_reaction_delta_unit: DEC_046_FORCE_REACTION_DELTA_UNIT,
        force_reaction_delta_dimension: DEC_046_FORCE_REACTION_DELTA_DIMENSION,
        force_reaction_delta_absolute_limit,
        moment_reaction_delta_basis: DEC_046_MOMENT_REACTION_DELTA_BASIS,
        moment_reaction_delta_unit: DEC_046_MOMENT_REACTION_DELTA_UNIT,
        moment_reaction_delta_dimension: DEC_046_MOMENT_REACTION_DELTA_DIMENSION,
        moment_reaction_delta_absolute_limit,
        status: ConvergencePolicyStatus::Accepted,
        evidence_fixture_ids,
        limitations,
    }
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

pub fn missing_required_assembled_families(
    fixtures: &[AssembledNonlinearRegressionCase],
) -> Vec<NonlinearRegressionFamily> {
    let required = [
        NonlinearRegressionFamily::ActiveSet,
        NonlinearRegressionFamily::Gap,
        NonlinearRegressionFamily::LiftOff,
        NonlinearRegressionFamily::Friction,
    ];

    required
        .into_iter()
        .filter(|family| !fixtures.iter().any(|fixture| fixture.family == *family))
        .collect()
}

fn convergence_class_label(
    fixture_id: &'static str,
    family: NonlinearRegressionFamily,
) -> &'static str {
    match fixture_id {
        "NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL" => "one_way",
        "NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL" => "gap",
        "NL-ASSEMBLED-LIFT-OFF-ORIGINAL" => "lift_off",
        "NL-ASSEMBLED-FRICTION-STICK-ORIGINAL"
        | "NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL"
        | "NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL" => "friction",
        "NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-OBS-ORIGINAL"
        | "NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-ACCEPTED-ORIGINAL"
        | "NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL"
        | "NL-ASSEMBLED-MULTI-DOF-FRICTION-GAP-ACCEPTED-ORIGINAL"
        | "NL-ASSEMBLED-MULTI-DOF-THREE-SUPPORT-ACCEPTED-ORIGINAL"
        | "NL-ASSEMBLED-MULTI-DOF-ROTATIONAL-ACCEPTED-ORIGINAL"
        | "NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-GAP-ACCEPTED-ORIGINAL"
        | "NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-ROTATIONAL-ACCEPTED-ORIGINAL"
        | "NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL"
        | "NL-ASSEMBLED-MULTI-DOF-NEGATIVE-GAP-ACCEPTED-ORIGINAL" => "multi_support_multi_dof",
        _ => match family {
            NonlinearRegressionFamily::ActiveSet => "active_set",
            NonlinearRegressionFamily::Gap => "gap",
            NonlinearRegressionFamily::LiftOff => "lift_off",
            NonlinearRegressionFamily::Friction => "friction",
            NonlinearRegressionFamily::NonConvergence => "nonconvergence",
            NonlinearRegressionFamily::MixedSupport => "mixed_support",
        },
    }
}

fn assembled_axial_input(
    nonlinear_supports: Vec<NonlinearSupport>,
    initial_states: Vec<SupportStateRecord>,
    convergence: ConvergenceControl,
) -> NonlinearFrameSolveInput {
    let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
    let node_j = FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap();
    let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let element = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap();
    let mut force: DenseVector = vec![0.0; 2 * DOF_PER_NODE];
    force[node_dof_index(1, FrameDof::Ux)] = 10.0;

    NonlinearFrameSolveInput {
        node_count: 2,
        elements: vec![element],
        user_stiffness_elements: Vec::new(),
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
        convergence,
    }
}

fn assembled_xy_tip_input(
    nonlinear_supports: Vec<NonlinearSupport>,
    initial_states: Vec<SupportStateRecord>,
    convergence: ConvergenceControl,
) -> NonlinearFrameSolveInput {
    let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
    let node_j = FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap();
    let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let element = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap();
    let mut force: DenseVector = vec![0.0; 2 * DOF_PER_NODE];
    force[node_dof_index(1, FrameDof::Ux)] = 10.0;
    force[node_dof_index(1, FrameDof::Uy)] = 1.0;

    NonlinearFrameSolveInput {
        node_count: 2,
        elements: vec![element],
        user_stiffness_elements: Vec::new(),
        force,
        base_restrained_dofs: vec![
            node_dof_index(0, FrameDof::Ux),
            node_dof_index(0, FrameDof::Uy),
            node_dof_index(0, FrameDof::Uz),
            node_dof_index(0, FrameDof::Rx),
            node_dof_index(0, FrameDof::Ry),
            node_dof_index(0, FrameDof::Rz),
            node_dof_index(1, FrameDof::Uz),
            node_dof_index(1, FrameDof::Rx),
            node_dof_index(1, FrameDof::Ry),
            node_dof_index(1, FrameDof::Rz),
        ],
        nonlinear_supports,
        initial_states,
        friction_normal_reactions: Vec::new(),
        derived_friction_normal_reactions: Vec::new(),
        convergence,
    }
}

fn assembled_xyz_tip_input(
    nonlinear_supports: Vec<NonlinearSupport>,
    initial_states: Vec<SupportStateRecord>,
    convergence: ConvergenceControl,
) -> NonlinearFrameSolveInput {
    let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
    let node_j = FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap();
    let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let element = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap();
    let mut force: DenseVector = vec![0.0; 2 * DOF_PER_NODE];
    force[node_dof_index(1, FrameDof::Ux)] = 10.0;
    force[node_dof_index(1, FrameDof::Uy)] = 1.0;
    force[node_dof_index(1, FrameDof::Uz)] = 6.0;

    NonlinearFrameSolveInput {
        node_count: 2,
        elements: vec![element],
        user_stiffness_elements: Vec::new(),
        force,
        base_restrained_dofs: vec![
            node_dof_index(0, FrameDof::Ux),
            node_dof_index(0, FrameDof::Uy),
            node_dof_index(0, FrameDof::Uz),
            node_dof_index(0, FrameDof::Rx),
            node_dof_index(0, FrameDof::Ry),
            node_dof_index(0, FrameDof::Rz),
            node_dof_index(1, FrameDof::Rx),
            node_dof_index(1, FrameDof::Ry),
            node_dof_index(1, FrameDof::Rz),
        ],
        nonlinear_supports,
        initial_states,
        friction_normal_reactions: Vec::new(),
        derived_friction_normal_reactions: Vec::new(),
        convergence,
    }
}

fn assembled_ux_rz_tip_input(
    nonlinear_supports: Vec<NonlinearSupport>,
    initial_states: Vec<SupportStateRecord>,
    convergence: ConvergenceControl,
) -> NonlinearFrameSolveInput {
    let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
    let node_j = FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap();
    let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let element = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap();
    let mut force: DenseVector = vec![0.0; 2 * DOF_PER_NODE];
    force[node_dof_index(1, FrameDof::Ux)] = 10.0;
    force[node_dof_index(1, FrameDof::Rz)] = 2.0;

    NonlinearFrameSolveInput {
        node_count: 2,
        elements: vec![element],
        user_stiffness_elements: Vec::new(),
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
        ],
        nonlinear_supports,
        initial_states,
        friction_normal_reactions: Vec::new(),
        derived_friction_normal_reactions: Vec::new(),
        convergence,
    }
}

fn assembled_uy_rz_tip_input(
    nonlinear_supports: Vec<NonlinearSupport>,
    initial_states: Vec<SupportStateRecord>,
    convergence: ConvergenceControl,
) -> NonlinearFrameSolveInput {
    let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
    let node_j = FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap();
    let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let element = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap();
    let mut force: DenseVector = vec![0.0; 2 * DOF_PER_NODE];
    force[node_dof_index(1, FrameDof::Uy)] = 1.0;

    NonlinearFrameSolveInput {
        node_count: 2,
        elements: vec![element],
        user_stiffness_elements: Vec::new(),
        force,
        base_restrained_dofs: vec![
            node_dof_index(0, FrameDof::Ux),
            node_dof_index(0, FrameDof::Uy),
            node_dof_index(0, FrameDof::Uz),
            node_dof_index(0, FrameDof::Rx),
            node_dof_index(0, FrameDof::Ry),
            node_dof_index(0, FrameDof::Rz),
            node_dof_index(1, FrameDof::Ux),
            node_dof_index(1, FrameDof::Uz),
            node_dof_index(1, FrameDof::Rx),
            node_dof_index(1, FrameDof::Ry),
        ],
        nonlinear_supports,
        initial_states,
        friction_normal_reactions: Vec::new(),
        derived_friction_normal_reactions: Vec::new(),
        convergence,
    }
}

fn accepted_convergence_control(
    nonlinear_class: &'static str,
) -> Result<ConvergenceControl, open_pipe_stress_nonlinear_integration::NonlinearIntegrationError> {
    let entry = governed_convergence_policy_entries()
        .into_iter()
        .find(|entry| entry.nonlinear_class == nonlinear_class)
        .expect("assembled validation policy entry exists for class");

    ConvergenceControl::new(
        entry.policy_ref,
        entry.status,
        entry.relative_residual_tolerance,
        entry.absolute_residual_floor,
        entry.max_iterations,
    )
}

fn tbd_multi_support_observation_control(
) -> Result<ConvergenceControl, open_pipe_stress_nonlinear_integration::NonlinearIntegrationError> {
    ConvergenceControl::new(
        MULTI_SUPPORT_DEPTH_POLICY_REF,
        ConvergencePolicyStatus::Tbd,
        DEC_046_ACTIVE_SET_COUNT_RELATIVE_TOLERANCE,
        DEC_046_ACTIVE_SET_COUNT_ABSOLUTE_FLOOR,
        DEC_046_ACTIVE_SET_COUNT_MAX_ITERATIONS,
    )
}

fn accepted_multisupport_convergence_control(
) -> Result<ConvergenceControl, open_pipe_stress_nonlinear_integration::NonlinearIntegrationError> {
    let entry = governed_multisupport_convergence_policy_entries()
        .into_iter()
        .next()
        .expect("multi-support validation policy entry exists");

    ConvergenceControl::new(
        entry.policy_ref,
        entry.status,
        entry.relative_residual_tolerance,
        entry.absolute_residual_floor,
        entry.max_iterations,
    )
}

pub fn assembled_one_way_deactivation_fixture() -> AssembledNonlinearRegressionCase {
    let support_id = "NL-ASSEMBLED-ONE-WAY-DEACTIVATE-A";
    let support = NonlinearSupport::one_way(
        support_id,
        1,
        FrameDof::Ux,
        ActivationSense::PositiveReaction,
    );
    let input = assembled_axial_input(
        vec![support],
        vec![SupportStateRecord::new(support_id, ActiveSetState::Active)],
        accepted_convergence_control("one_way").unwrap(),
    );

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL",
        family: NonlinearRegressionFamily::ActiveSet,
        description:
            "Invented assembled frame solve deactivates a one-way support and then converges.",
        assumptions: &[
            "The frame fixture is a two-node axial member with invented stiffness values.",
            "The residual is the assembled loop's active-set changed-support count.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_one_way_deactivation.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![ExpectedState {
            support_id,
            state: ActiveSetState::Inactive,
        }],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "applied_force",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_gap_closure_fixture() -> AssembledNonlinearRegressionCase {
    let support_id = "NL-ASSEMBLED-GAP-CLOSE-A";
    let support = NonlinearSupport::gap(
        support_id,
        1,
        FrameDof::Ux,
        0.05,
        GapDirection::PositiveDisplacement,
    )
    .unwrap();
    let input = assembled_axial_input(
        vec![support],
        vec![SupportStateRecord::new(
            support_id,
            ActiveSetState::Inactive,
        )],
        accepted_convergence_control("gap").unwrap(),
    );

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL",
        family: NonlinearRegressionFamily::Gap,
        description: "Invented assembled frame solve closes a positive gap at explicit clearance.",
        assumptions: &[
            "The free axial displacement is larger than the explicit invented clearance.",
            "The closed gap is represented as a prescribed support displacement in the linearized iteration.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_gap_closure.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![ExpectedState {
            support_id,
            state: ActiveSetState::Active,
        }],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "gap_clearance",
                value: 0.05,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_lift_off_loss_fixture() -> AssembledNonlinearRegressionCase {
    let support_id = "NL-ASSEMBLED-LIFT-OFF-A";
    let support = NonlinearSupport::lift_off(
        support_id,
        1,
        FrameDof::Ux,
        ActivationSense::PositiveReaction,
    );
    let input = assembled_axial_input(
        vec![support],
        vec![SupportStateRecord::new(support_id, ActiveSetState::Active)],
        accepted_convergence_control("lift_off").unwrap(),
    );

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-LIFT-OFF-ORIGINAL",
        family: NonlinearRegressionFamily::LiftOff,
        description:
            "Invented assembled frame solve releases a lift-off support and then converges.",
        assumptions: &[
            "Contact requires a positive reaction in this invented fixture.",
            "The final free-state reaction remains zero, so contact stays released.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_lift_off.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![ExpectedState {
            support_id,
            state: ActiveSetState::Inactive,
        }],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "applied_force",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_friction_sticking_fixture() -> AssembledNonlinearRegressionCase {
    let support_id = "NL-ASSEMBLED-FRICTION-STICK-A";
    let support = NonlinearSupport::friction(support_id, 1, FrameDof::Ux, 0.30).unwrap();
    let mut input = assembled_axial_input(
        vec![support],
        vec![SupportStateRecord::new(
            support_id,
            ActiveSetState::Sticking,
        )],
        accepted_convergence_control("friction").unwrap(),
    );
    input.friction_normal_reactions = vec![FrictionNormalReaction::new(support_id, 100.0).unwrap()];

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-FRICTION-STICK-ORIGINAL",
        family: NonlinearRegressionFamily::Friction,
        description:
            "Invented assembled frame solve keeps a friction support sticking with explicit normal evidence.",
        assumptions: &[
            "The normal reaction is explicit invented input evidence, not a derived normal-force model.",
            "The tangential trial reaction comes from the assembled frame reaction at the friction DOF.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_friction_sticking.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![ExpectedState {
            support_id,
            state: ActiveSetState::Sticking,
        }],
        expected_iteration_count: 1,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "friction_coefficient",
                value: 0.30,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "explicit_normal_reaction",
                value: 100.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "tangential_reaction",
                value: -10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 1.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_friction_sliding_fixture() -> AssembledNonlinearRegressionCase {
    let support_id = "NL-ASSEMBLED-FRICTION-SLIDE-A";
    let support = NonlinearSupport::friction(support_id, 1, FrameDof::Ux, 0.30).unwrap();
    let mut input = assembled_axial_input(
        vec![support],
        vec![SupportStateRecord::new(
            support_id,
            ActiveSetState::Sticking,
        )],
        accepted_convergence_control("friction").unwrap(),
    );
    input.friction_normal_reactions = vec![FrictionNormalReaction::new(support_id, 10.0).unwrap()];

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL",
        family: NonlinearRegressionFamily::Friction,
        description:
            "Invented assembled frame solve releases a friction support to sliding with explicit normal evidence.",
        assumptions: &[
            "The normal reaction is explicit invented input evidence, not a derived normal-force model.",
            "The first sticking trial reaction exceeds the invented friction limit, then the released DOF keeps sliding while displacement persists.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_friction_sliding.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![ExpectedState {
            support_id,
            state: ActiveSetState::Sliding,
        }],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "friction_coefficient",
                value: 0.30,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "explicit_normal_reaction",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "first_trial_tangential_reaction",
                value: -10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "friction_limit",
                value: 3.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_friction_derived_normal_fixture() -> AssembledNonlinearRegressionCase {
    let support_id = "NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-A";
    let support = NonlinearSupport::friction(support_id, 1, FrameDof::Ux, 0.30).unwrap();
    let mut input = assembled_axial_input(
        vec![support],
        vec![SupportStateRecord::new(
            support_id,
            ActiveSetState::Sticking,
        )],
        accepted_convergence_control("friction").unwrap(),
    );
    input.force[node_dof_index(1, FrameDof::Uy)] = -100.0;
    input.derived_friction_normal_reactions =
        vec![DerivedFrictionNormalReaction::from_support_reaction(
            support_id,
            1,
            FrameDof::Uy,
            "fixture-normal-support:node-1:uy",
        )
        .unwrap()];

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL",
        family: NonlinearRegressionFamily::Friction,
        description:
            "Invented assembled frame solve derives friction normal evidence from a named support reaction.",
        assumptions: &[
            "The normal reaction magnitude is the absolute frame reaction at a named restrained support-normal DOF.",
            "The source support DOF is explicit fixture input; no catalog, default, or protected normal-force value is supplied.",
            "The tangential trial reaction comes from the assembled frame reaction at the friction DOF.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_friction_derived_normal.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![ExpectedState {
            support_id,
            state: ActiveSetState::Sticking,
        }],
        expected_iteration_count: 1,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "friction_coefficient",
                value: 0.30,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "derived_normal_reaction",
                value: 100.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "tangential_reaction",
                value: -10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 1.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_multi_dof_multi_support_observation_fixture() -> AssembledNonlinearRegressionCase {
    let one_way_id = "NL-ASSEMBLED-MULTI-ONE-WAY-UX-A";
    let gap_id = "NL-ASSEMBLED-MULTI-GAP-UY-A";
    let one_way = NonlinearSupport::one_way(
        one_way_id,
        1,
        FrameDof::Ux,
        ActivationSense::PositiveReaction,
    );
    let gap = NonlinearSupport::gap(
        gap_id,
        1,
        FrameDof::Uy,
        0.0002,
        GapDirection::PositiveDisplacement,
    )
    .unwrap();
    let input = assembled_xy_tip_input(
        vec![one_way, gap],
        vec![
            SupportStateRecord::new(one_way_id, ActiveSetState::Active),
            SupportStateRecord::new(gap_id, ActiveSetState::Inactive),
        ],
        tbd_multi_support_observation_control().unwrap(),
    );

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-OBS-ORIGINAL",
        family: NonlinearRegressionFamily::MixedSupport,
        description:
            "Invented assembled frame solve observes simultaneous Ux one-way release and Uy gap closure without promoting a non-seed threshold.",
        assumptions: &[
            "The frame fixture is a two-node member with two free translational tip DOFs.",
            "The first linearized iteration can change two nonlinear supports in different DOFs.",
            "This is a depth observation outside the accepted current assembled validation seed.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![
            ExpectedState {
                support_id: gap_id,
                state: ActiveSetState::Active,
            },
            ExpectedState {
                support_id: one_way_id,
                state: ActiveSetState::Inactive,
            },
        ],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![SolverDiagnosticCode::TolerancePolicyTbd],
        observations: vec![
            DimensionedObservation {
                name: "applied_ux_force",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_uy_force",
                value: 1.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "gap_clearance",
                value: 0.0002,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn assembled_multi_dof_multi_support_acceptance_fixture() -> AssembledNonlinearRegressionCase {
    let one_way_id = "NL-ASSEMBLED-MULTI-ONE-WAY-UX-B";
    let gap_id = "NL-ASSEMBLED-MULTI-GAP-UY-B";
    let one_way = NonlinearSupport::one_way(
        one_way_id,
        1,
        FrameDof::Ux,
        ActivationSense::PositiveReaction,
    );
    let gap = NonlinearSupport::gap(
        gap_id,
        1,
        FrameDof::Uy,
        0.0002,
        GapDirection::PositiveDisplacement,
    )
    .unwrap();
    let input = assembled_xy_tip_input(
        vec![one_way, gap],
        vec![
            SupportStateRecord::new(one_way_id, ActiveSetState::Active),
            SupportStateRecord::new(gap_id, ActiveSetState::Inactive),
        ],
        accepted_multisupport_convergence_control().unwrap(),
    );

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-ACCEPTED-ORIGINAL",
        family: NonlinearRegressionFamily::MixedSupport,
        description:
            "Invented assembled frame solve accepts simultaneous Ux one-way release and Uy gap closure under a narrow multi-support DEC-046 policy.",
        assumptions: &[
            "The frame fixture is a two-node member with two free translational tip DOFs.",
            "The first linearized iteration can change two nonlinear supports in different DOFs.",
            "This is non-seed multi-support acceptance evidence only; displacement, reaction-delta, and work/energy thresholds remain TBD.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof_acceptance.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![
            ExpectedState {
                support_id: gap_id,
                state: ActiveSetState::Active,
            },
            ExpectedState {
                support_id: one_way_id,
                state: ActiveSetState::Inactive,
            },
        ],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "applied_ux_force",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_uy_force",
                value: 1.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "gap_clearance",
                value: 0.0002,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_multi_dof_gap_lift_off_acceptance_fixture() -> AssembledNonlinearRegressionCase {
    let lift_off_id = "NL-ASSEMBLED-MULTI-LIFT-OFF-UX-C";
    let gap_id = "NL-ASSEMBLED-MULTI-GAP-UY-C";
    let lift_off = NonlinearSupport::lift_off(
        lift_off_id,
        1,
        FrameDof::Ux,
        ActivationSense::PositiveReaction,
    );
    let gap = NonlinearSupport::gap(
        gap_id,
        1,
        FrameDof::Uy,
        0.0002,
        GapDirection::PositiveDisplacement,
    )
    .unwrap();
    let input = assembled_xy_tip_input(
        vec![lift_off, gap],
        vec![
            SupportStateRecord::new(lift_off_id, ActiveSetState::Active),
            SupportStateRecord::new(gap_id, ActiveSetState::Inactive),
        ],
        accepted_multisupport_convergence_control().unwrap(),
    );

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL",
        family: NonlinearRegressionFamily::MixedSupport,
        description:
            "Invented assembled frame solve accepts simultaneous Ux lift-off release and Uy gap closure under a narrow multi-support DEC-046 policy.",
        assumptions: &[
            "The frame fixture is a two-node member with two free translational tip DOFs.",
            "This acceptance companion uses a different support-behavior pair than the one-way/gap multi-support companion.",
            "This is non-seed multi-support acceptance evidence only; displacement, reaction-delta, and general energy thresholds remain TBD.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_multi_support_gap_lift_off_acceptance.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![
            ExpectedState {
                support_id: gap_id,
                state: ActiveSetState::Active,
            },
            ExpectedState {
                support_id: lift_off_id,
                state: ActiveSetState::Inactive,
            },
        ],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "applied_ux_force",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_uy_force",
                value: 1.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "gap_clearance",
                value: 0.0002,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_multi_dof_friction_gap_acceptance_fixture() -> AssembledNonlinearRegressionCase {
    let friction_id = "NL-ASSEMBLED-MULTI-FRICTION-UX-D";
    let gap_id = "NL-ASSEMBLED-MULTI-GAP-UY-D";
    let friction = NonlinearSupport::friction(friction_id, 1, FrameDof::Ux, 0.30).unwrap();
    let gap = NonlinearSupport::gap(
        gap_id,
        1,
        FrameDof::Uy,
        0.0002,
        GapDirection::PositiveDisplacement,
    )
    .unwrap();
    let mut input = assembled_xy_tip_input(
        vec![friction, gap],
        vec![
            SupportStateRecord::new(friction_id, ActiveSetState::Sticking),
            SupportStateRecord::new(gap_id, ActiveSetState::Inactive),
        ],
        accepted_multisupport_convergence_control().unwrap(),
    );
    input.friction_normal_reactions = vec![FrictionNormalReaction::new(friction_id, 10.0).unwrap()];

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-MULTI-DOF-FRICTION-GAP-ACCEPTED-ORIGINAL",
        family: NonlinearRegressionFamily::MixedSupport,
        description:
            "Invented assembled frame solve accepts simultaneous Ux friction sliding and Uy gap closure under a narrow multi-support DEC-046 policy.",
        assumptions: &[
            "The frame fixture is a two-node member with two free translational tip DOFs.",
            "The friction normal reaction is explicit invented input evidence.",
            "This acceptance companion adds a friction/gap behavior pair to the accepted multi-support fixture set.",
            "This is non-seed multi-support acceptance evidence only; displacement, reaction-delta, and general energy thresholds remain TBD.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_multi_support_friction_gap_acceptance.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![
            ExpectedState {
                support_id: friction_id,
                state: ActiveSetState::Sliding,
            },
            ExpectedState {
                support_id: gap_id,
                state: ActiveSetState::Active,
            },
        ],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "applied_ux_force",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_uy_force",
                value: 1.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "friction_coefficient",
                value: 0.30,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "explicit_normal_reaction",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "friction_limit",
                value: 3.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "gap_clearance",
                value: 0.0002,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_multi_dof_three_support_acceptance_fixture() -> AssembledNonlinearRegressionCase {
    let one_way_id = "NL-ASSEMBLED-MULTI-ONE-WAY-UX-E";
    let gap_id = "NL-ASSEMBLED-MULTI-GAP-UY-E";
    let friction_id = "NL-ASSEMBLED-MULTI-FRICTION-UZ-E";
    let one_way = NonlinearSupport::one_way(
        one_way_id,
        1,
        FrameDof::Ux,
        ActivationSense::PositiveReaction,
    );
    let gap = NonlinearSupport::gap(
        gap_id,
        1,
        FrameDof::Uy,
        0.0002,
        GapDirection::PositiveDisplacement,
    )
    .unwrap();
    let friction = NonlinearSupport::friction(friction_id, 1, FrameDof::Uz, 0.30).unwrap();
    let mut input = assembled_xyz_tip_input(
        vec![one_way, gap, friction],
        vec![
            SupportStateRecord::new(one_way_id, ActiveSetState::Active),
            SupportStateRecord::new(gap_id, ActiveSetState::Inactive),
            SupportStateRecord::new(friction_id, ActiveSetState::Sticking),
        ],
        accepted_multisupport_convergence_control().unwrap(),
    );
    input.friction_normal_reactions = vec![FrictionNormalReaction::new(friction_id, 10.0).unwrap()];

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-MULTI-DOF-THREE-SUPPORT-ACCEPTED-ORIGINAL",
        family: NonlinearRegressionFamily::MixedSupport,
        description:
            "Invented assembled frame solve accepts simultaneous Ux one-way release, Uy gap closure, and Uz friction sliding under a narrow multi-support DEC-046 policy.",
        assumptions: &[
            "The frame fixture is a two-node member with three free translational tip DOFs.",
            "The first linearized iteration can change three nonlinear supports across three different DOFs.",
            "The friction normal reaction is explicit invented input evidence.",
            "This acceptance companion broadens fixture depth only; displacement, reaction-delta, and general energy thresholds remain TBD.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_multi_support_three_dof_acceptance.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![
            ExpectedState {
                support_id: friction_id,
                state: ActiveSetState::Sliding,
            },
            ExpectedState {
                support_id: gap_id,
                state: ActiveSetState::Active,
            },
            ExpectedState {
                support_id: one_way_id,
                state: ActiveSetState::Inactive,
            },
        ],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "applied_ux_force",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_uy_force",
                value: 1.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_uz_force",
                value: 6.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "friction_coefficient",
                value: 0.30,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "explicit_normal_reaction",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "friction_limit",
                value: 3.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "gap_clearance",
                value: 0.0002,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_multi_dof_rotational_acceptance_fixture() -> AssembledNonlinearRegressionCase {
    let one_way_id = "NL-ASSEMBLED-MULTI-ONE-WAY-UX-F";
    let lift_off_id = "NL-ASSEMBLED-MULTI-LIFT-OFF-RZ-F";
    let one_way = NonlinearSupport::one_way(
        one_way_id,
        1,
        FrameDof::Ux,
        ActivationSense::PositiveReaction,
    );
    let lift_off = NonlinearSupport::lift_off(
        lift_off_id,
        1,
        FrameDof::Rz,
        ActivationSense::PositiveReaction,
    );
    let input = assembled_ux_rz_tip_input(
        vec![one_way, lift_off],
        vec![
            SupportStateRecord::new(one_way_id, ActiveSetState::Active),
            SupportStateRecord::new(lift_off_id, ActiveSetState::Active),
        ],
        accepted_multisupport_convergence_control().unwrap(),
    );

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-MULTI-DOF-ROTATIONAL-ACCEPTED-ORIGINAL",
        family: NonlinearRegressionFamily::MixedSupport,
        description:
            "Invented assembled frame solve accepts simultaneous Ux one-way release and Rz lift-off release under a narrow multi-support DEC-046 policy.",
        assumptions: &[
            "The frame fixture is a two-node member with one free translational and one free rotational tip DOF.",
            "The first linearized iteration can change two nonlinear supports across translation and rotation groups.",
            "This acceptance companion broadens fixture breadth to include the rotational moment residual axis; displacement, reaction-delta, and general energy thresholds remain TBD.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_multi_support_rotational_acceptance.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![
            ExpectedState {
                support_id: lift_off_id,
                state: ActiveSetState::Inactive,
            },
            ExpectedState {
                support_id: one_way_id,
                state: ActiveSetState::Inactive,
            },
        ],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "applied_ux_force",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_rz_moment",
                value: 2.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_multi_dof_derived_normal_gap_acceptance_fixture(
) -> AssembledNonlinearRegressionCase {
    let friction_id = "NL-ASSEMBLED-MULTI-FRICTION-DERIVED-NORMAL-G";
    let gap_id = "NL-ASSEMBLED-MULTI-GAP-UZ-G";
    let friction = NonlinearSupport::friction(friction_id, 1, FrameDof::Ux, 0.03).unwrap();
    let gap = NonlinearSupport::gap(
        gap_id,
        1,
        FrameDof::Uz,
        0.0002,
        GapDirection::PositiveDisplacement,
    )
    .unwrap();
    let mut input = assembled_xyz_tip_input(
        vec![friction, gap],
        vec![
            SupportStateRecord::new(friction_id, ActiveSetState::Sticking),
            SupportStateRecord::new(gap_id, ActiveSetState::Inactive),
        ],
        accepted_multisupport_convergence_control().unwrap(),
    );
    input.force[node_dof_index(1, FrameDof::Uy)] = -100.0;
    input
        .base_restrained_dofs
        .push(node_dof_index(1, FrameDof::Uy));
    input.derived_friction_normal_reactions =
        vec![DerivedFrictionNormalReaction::from_support_reaction(
            friction_id,
            1,
            FrameDof::Uy,
            "fixture-normal-support:node-1:uy",
        )
        .unwrap()];

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-GAP-ACCEPTED-ORIGINAL",
        family: NonlinearRegressionFamily::MixedSupport,
        description:
            "Invented assembled frame solve accepts derived-normal friction sliding on Ux and gap closure on Uz under a narrow multi-support DEC-046 policy.",
        assumptions: &[
            "The frame fixture is a two-node member with two free translational tip DOFs plus a separate restrained normal-source DOF.",
            "The friction normal reaction is derived from the absolute reaction at a named restrained support-normal DOF supplied by the fixture.",
            "This acceptance companion broadens multi-support evidence to derived friction normal-source behavior; displacement, reaction-delta, and general energy thresholds remain TBD.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_multi_support_derived_normal_gap_acceptance.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![
            ExpectedState {
                support_id: friction_id,
                state: ActiveSetState::Sliding,
            },
            ExpectedState {
                support_id: gap_id,
                state: ActiveSetState::Active,
            },
        ],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "applied_ux_force",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_normal_source_force",
                value: -100.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_uz_force",
                value: 6.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "friction_coefficient",
                value: 0.03,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "derived_normal_reaction",
                value: 100.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "friction_limit",
                value: 3.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "gap_clearance",
                value: 0.0002,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_multi_dof_derived_normal_rotational_acceptance_fixture(
) -> AssembledNonlinearRegressionCase {
    let friction_id = "NL-ASSEMBLED-MULTI-FRICTION-DERIVED-NORMAL-H";
    let lift_off_id = "NL-ASSEMBLED-MULTI-LIFT-OFF-RZ-H";
    let friction = NonlinearSupport::friction(friction_id, 1, FrameDof::Ux, 0.03).unwrap();
    let lift_off = NonlinearSupport::lift_off(
        lift_off_id,
        1,
        FrameDof::Rz,
        ActivationSense::PositiveReaction,
    );
    let mut input = assembled_ux_rz_tip_input(
        vec![friction, lift_off],
        vec![
            SupportStateRecord::new(friction_id, ActiveSetState::Sticking),
            SupportStateRecord::new(lift_off_id, ActiveSetState::Active),
        ],
        accepted_multisupport_convergence_control().unwrap(),
    );
    input.force[node_dof_index(1, FrameDof::Uy)] = -100.0;
    input.derived_friction_normal_reactions =
        vec![DerivedFrictionNormalReaction::from_support_reaction(
            friction_id,
            1,
            FrameDof::Uy,
            "fixture-normal-support:node-1:uy",
        )
        .unwrap()];

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-ROTATIONAL-ACCEPTED-ORIGINAL",
        family: NonlinearRegressionFamily::MixedSupport,
        description:
            "Invented assembled frame solve accepts derived-normal friction sliding on Ux and rotational lift-off release on Rz under a narrow multi-support DEC-046 policy.",
        assumptions: &[
            "The frame fixture is a two-node member with one free translational and one free rotational tip DOF plus a separate restrained normal-source DOF.",
            "The friction normal reaction is derived from the absolute reaction at a named restrained support-normal DOF supplied by the fixture.",
            "This acceptance companion broadens multi-support evidence to a combined derived-normal friction and rotational lift-off transition; displacement, reaction-delta, and general energy thresholds remain TBD.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_multi_support_derived_normal_rotational_acceptance.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![
            ExpectedState {
                support_id: friction_id,
                state: ActiveSetState::Sliding,
            },
            ExpectedState {
                support_id: lift_off_id,
                state: ActiveSetState::Inactive,
            },
        ],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "applied_ux_force",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_normal_source_force",
                value: -100.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_rz_moment",
                value: 2.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "friction_coefficient",
                value: 0.03,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "derived_normal_reaction",
                value: 100.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "friction_limit",
                value: 3.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_multi_dof_cascade_gap_lift_off_acceptance_fixture(
) -> AssembledNonlinearRegressionCase {
    let lift_off_id = "NL-ASSEMBLED-MULTI-LIFT-OFF-RZ-I";
    let gap_id = "NL-ASSEMBLED-MULTI-GAP-UY-I";
    let lift_off = NonlinearSupport::lift_off(
        lift_off_id,
        1,
        FrameDof::Rz,
        ActivationSense::PositiveReaction,
    );
    let gap = NonlinearSupport::gap(
        gap_id,
        1,
        FrameDof::Uy,
        0.002,
        GapDirection::PositiveDisplacement,
    )
    .unwrap();
    let input = assembled_uy_rz_tip_input(
        vec![lift_off, gap],
        vec![
            SupportStateRecord::new(lift_off_id, ActiveSetState::Active),
            SupportStateRecord::new(gap_id, ActiveSetState::Inactive),
        ],
        accepted_multisupport_convergence_control().unwrap(),
    );

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL",
        family: NonlinearRegressionFamily::MixedSupport,
        description:
            "Invented assembled frame solve accepts a measured active-set cascade: Rz lift-off release changes the next Uy gap classification before final convergence.",
        assumptions: &[
            "The frame fixture is a two-node member with coupled free transverse and rotational tip DOFs.",
            "With Rz initially restrained, the first Uy displacement remains below the explicit gap clearance while the Rz reaction releases lift-off contact.",
            "After Rz is released, the next linearized solve increases Uy displacement enough to close the gap; the third iteration has no state changes.",
            "This acceptance companion broadens multi-support evidence to a sequential active-set cascade within the existing max-iteration cap; displacement, reaction-delta, and general energy thresholds remain TBD.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_multi_support_cascade_gap_lift_off_acceptance.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![
            ExpectedState {
                support_id: gap_id,
                state: ActiveSetState::Active,
            },
            ExpectedState {
                support_id: lift_off_id,
                state: ActiveSetState::Inactive,
            },
        ],
        expected_iteration_count: 3,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "applied_uy_force",
                value: 1.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "gap_clearance",
                value: 0.002,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "first_iteration_changed_support_count",
                value: 1.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "second_iteration_changed_support_count",
                value: 1.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 3.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
}

pub fn assembled_multi_dof_negative_gap_acceptance_fixture() -> AssembledNonlinearRegressionCase {
    let one_way_id = "NL-ASSEMBLED-MULTI-ONE-WAY-UX-J";
    let gap_id = "NL-ASSEMBLED-MULTI-GAP-NEG-UY-J";
    let one_way = NonlinearSupport::one_way(
        one_way_id,
        1,
        FrameDof::Ux,
        ActivationSense::PositiveReaction,
    );
    let gap = NonlinearSupport::gap(
        gap_id,
        1,
        FrameDof::Uy,
        0.0002,
        GapDirection::NegativeDisplacement,
    )
    .unwrap();
    let mut input = assembled_xy_tip_input(
        vec![one_way, gap],
        vec![
            SupportStateRecord::new(one_way_id, ActiveSetState::Active),
            SupportStateRecord::new(gap_id, ActiveSetState::Inactive),
        ],
        accepted_multisupport_convergence_control().unwrap(),
    );
    input.force[node_dof_index(1, FrameDof::Uy)] = -1.0;

    AssembledNonlinearRegressionCase {
        fixture_id: "NL-ASSEMBLED-MULTI-DOF-NEGATIVE-GAP-ACCEPTED-ORIGINAL",
        family: NonlinearRegressionFamily::MixedSupport,
        description:
            "Invented assembled frame solve accepts simultaneous Ux one-way release and negative-direction Uy gap closure under a narrow multi-support DEC-046 policy.",
        assumptions: &[
            "The frame fixture is a two-node member with two free translational tip DOFs.",
            "The Uy gap closes only in the negative displacement direction under an explicit negative invented load.",
            "This acceptance companion broadens the multi-support fixture set to the negative gap-direction branch; general energy, release, and external thresholds remain TBD.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/nonlinear/assembled_multi_support_negative_gap_acceptance.md",
        ),
        unit_basis: NONLINEAR_FIXTURE_UNIT_BASIS,
        input,
        expected_final_states: vec![
            ExpectedState {
                support_id: gap_id,
                state: ActiveSetState::Active,
            },
            ExpectedState {
                support_id: one_way_id,
                state: ActiveSetState::Inactive,
            },
        ],
        expected_iteration_count: 2,
        expected_final_residual_norm: 0.0,
        expected_converged: true,
        expected_diagnostic_codes: vec![],
        observations: vec![
            DimensionedObservation {
                name: "applied_ux_force",
                value: 10.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "applied_uy_force",
                value: -1.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "gap_clearance",
                value: 0.0002,
                unit: "mm",
                dimension: "length",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "gap_direction_negative",
                value: 1.0,
                unit: "flag",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            DimensionedObservation {
                name: "iteration_count",
                value: 2.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
            DimensionedObservation {
                name: "final_residual",
                value: 0.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: Some(DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF),
            },
        ],
    }
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

fn optional_finite(value: Option<f64>) -> bool {
    value.map(|value| value.is_finite()).unwrap_or(true)
}

fn optional_delta_within_limit(value: Option<f64>, absolute_limit: f64) -> bool {
    value
        .map(|value| value.is_finite() && value >= 0.0 && value <= absolute_limit)
        .unwrap_or(true)
}

fn seed_displacement_reaction_delta_limits(nonlinear_class: &str) -> Option<(f64, f64, f64, f64)> {
    match nonlinear_class {
        "one_way" | "lift_off" | "friction" => Some((
            DEC_046_SEED_TRANSLATION_DELTA_ABSOLUTE_LIMIT,
            DEC_046_SEED_ROTATION_DELTA_ABSOLUTE_LIMIT,
            DEC_046_SEED_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT,
            DEC_046_SEED_MOMENT_REACTION_DELTA_ABSOLUTE_LIMIT,
        )),
        "gap" => Some((
            DEC_046_SEED_GAP_TRANSLATION_DELTA_ABSOLUTE_LIMIT,
            DEC_046_SEED_ROTATION_DELTA_ABSOLUTE_LIMIT,
            DEC_046_SEED_GAP_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT,
            DEC_046_SEED_MOMENT_REACTION_DELTA_ABSOLUTE_LIMIT,
        )),
        _ => None,
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
    fn assembled_inventory_covers_global_loop_seed_families() {
        let fixtures = assembled_fixture_inventory();

        assert!(missing_required_assembled_families(&fixtures).is_empty());
        assert_eq!(fixtures.len(), 6);
    }

    #[test]
    fn multisupport_depth_inventory_is_observation_only_and_separate_from_seed() {
        let fixtures = assembled_multisupport_depth_inventory();

        assert_eq!(fixtures.len(), 1);
        assert_eq!(
            fixtures[0].fixture_id,
            "NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-OBS-ORIGINAL"
        );
        assert!(!assembled_fixture_inventory()
            .iter()
            .any(|fixture| fixture.fixture_id == fixtures[0].fixture_id));
        assert!(fixtures[0].tolerance_policy_is_unresolved());
        assert!(!fixtures[0].uses_governed_convergence_policy());
        assert!(fixtures[0].matches_expected_outcome());

        let solve = fixtures[0].run().unwrap();
        assert!(solve.converged);
        assert_eq!(solve.iterations.len(), 2);
        assert_eq!(solve.iterations[0].active_set.residual_norm, 2.0);
        assert_eq!(
            solve.iterations.last().unwrap().active_set.residual_norm,
            0.0
        );
        assert!(solve
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == SolverDiagnosticCode::TolerancePolicyTbd));

        let observations = assembled_multisupport_depth_convergence_observations();
        assert_eq!(observations.len(), 1);
        assert_eq!(observations[0].policy_ref, MULTI_SUPPORT_DEPTH_POLICY_REF);
        assert_eq!(observations[0].policy_status, ConvergencePolicyStatus::Tbd);
        assert_eq!(observations[0].nonlinear_class, "multi_support_multi_dof");
        assert!(observations[0].observed_converged);
        assert!(!observations[0].uses_accepted_dec_046_active_set_policy());

        let residuals = assembled_multisupport_depth_residual_observations();
        assert_eq!(residuals.len(), 1);
        assert_eq!(
            residuals[0].displacement_reaction_delta_observation_ref,
            None
        );
        assert_eq!(
            residuals[0].displacement_reaction_delta_threshold_policy,
            None
        );
        assert_eq!(residuals[0].free_dof_force_moment_threshold_policy, None);
        assert_eq!(residuals[0].free_dof_work_threshold_policy, None);
        assert!(!residuals[0].uses_accepted_free_dof_force_moment_threshold_policy());
        assert_eq!(residuals[0].max_abs_free_dof_work_residual, 0.0);
        assert!(
            residuals[0]
                .max_abs_translation_delta_from_previous
                .unwrap()
                > 0.0
        );
        assert!(
            residuals[0]
                .max_abs_force_reaction_delta_from_previous
                .unwrap()
                > 0.0
        );
    }

    #[test]
    fn multisupport_acceptance_inventory_uses_narrow_dec_046_policy() {
        let fixtures = assembled_multisupport_acceptance_inventory();

        assert_eq!(fixtures.len(), 9);
        assert_eq!(
            fixtures
                .iter()
                .map(|fixture| fixture.fixture_id)
                .collect::<Vec<_>>(),
            DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS
        );
        for fixture in &fixtures {
            assert!(!assembled_fixture_inventory()
                .iter()
                .any(|seed| seed.fixture_id == fixture.fixture_id));
            assert!(!assembled_multisupport_depth_inventory()
                .iter()
                .any(|depth| depth.fixture_id == fixture.fixture_id));
            assert!(!fixture.tolerance_policy_is_unresolved());
            assert!(fixture.uses_governed_multisupport_convergence_policy());
            assert!(fixture.matches_expected_outcome());

            let solve = fixture.run().unwrap();
            assert!(solve.converged);
            assert_eq!(solve.iterations.len(), fixture.expected_iteration_count);
            let expected_initial_residual =
                if fixture.fixture_id == "NL-ASSEMBLED-MULTI-DOF-THREE-SUPPORT-ACCEPTED-ORIGINAL" {
                    3.0
                } else if fixture.fixture_id
                    == "NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL"
                {
                    1.0
                } else {
                    2.0
                };
            assert_eq!(
                solve.iterations[0].active_set.residual_norm,
                expected_initial_residual
            );
            assert_eq!(
                solve.iterations.last().unwrap().active_set.residual_norm,
                0.0
            );
            assert!(!solve
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == SolverDiagnosticCode::TolerancePolicyTbd));
        }

        let rotational_fixture = fixtures
            .iter()
            .find(|fixture| {
                fixture.fixture_id == "NL-ASSEMBLED-MULTI-DOF-ROTATIONAL-ACCEPTED-ORIGINAL"
            })
            .expect("rotational acceptance companion remains in inventory");
        let rotational_solve = rotational_fixture.run().unwrap();
        let final_residuals = &rotational_solve.iterations.last().unwrap().residuals;
        assert!(
            final_residuals
                .max_abs_rotation_delta_from_previous
                .unwrap()
                > 0.0
        );
        assert!(
            final_residuals
                .max_abs_moment_reaction_delta_from_previous
                .unwrap()
                > 0.0
        );

        let cascade_fixture = fixtures
            .iter()
            .find(|fixture| {
                fixture.fixture_id
                    == "NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL"
            })
            .expect("cascade acceptance companion remains in inventory");
        let cascade_solve = cascade_fixture.run().unwrap();
        assert_eq!(
            cascade_solve
                .iterations
                .iter()
                .map(|iteration| iteration.active_set.residual_norm)
                .collect::<Vec<_>>(),
            vec![1.0, 1.0, 0.0]
        );
        assert_eq!(
            cascade_solve.iterations[0].active_set.changed_supports,
            vec!["NL-ASSEMBLED-MULTI-LIFT-OFF-RZ-I".to_string()]
        );
        assert_eq!(
            cascade_solve.iterations[1].active_set.changed_supports,
            vec!["NL-ASSEMBLED-MULTI-GAP-UY-I".to_string()]
        );

        let observations = assembled_multisupport_acceptance_convergence_observations();
        assert_eq!(observations.len(), 9);
        for observation in &observations {
            assert_eq!(
                observation.policy_ref,
                DEC_046_MULTISUPPORT_ACTIVE_SET_COUNT_POLICY_REF
            );
            assert_eq!(observation.policy_status, ConvergencePolicyStatus::Accepted);
            assert_eq!(observation.nonlinear_class, "multi_support_multi_dof");
            assert!(observation.observed_converged);
            assert!(observation.uses_accepted_multisupport_dec_046_active_set_policy());
        }

        let residuals = assembled_multisupport_acceptance_residual_observations();
        assert_eq!(residuals.len(), 9);
        for residual in &residuals {
            assert_eq!(
                residual.free_dof_force_moment_threshold_policy,
                Some(DEC_046_MULTISUPPORT_FREE_DOF_FORCE_MOMENT_POLICY_REF)
            );
            assert_eq!(
                residual.free_dof_work_threshold_policy,
                Some(DEC_046_MULTISUPPORT_FREE_DOF_WORK_POLICY_REF)
            );
            assert!(residual.uses_accepted_multisupport_free_dof_force_moment_threshold_policy());
            assert!(
                residual.uses_accepted_multisupport_displacement_reaction_delta_threshold_policy()
            );
            assert_eq!(
                residual.displacement_reaction_delta_threshold_policy,
                Some(DEC_046_MULTISUPPORT_DISPLACEMENT_REACTION_DELTA_POLICY_REF)
            );
            assert_eq!(residual.max_abs_free_dof_force_residual, 0.0);
            assert_eq!(residual.max_abs_free_dof_moment_residual, 0.0);
            assert_eq!(residual.max_abs_free_dof_work_residual, 0.0);
        }
    }

    #[test]
    fn governed_convergence_policy_covers_dec_046_classes() {
        let entries = governed_convergence_policy_entries();

        assert_eq!(entries.len(), 4);
        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.nonlinear_class)
                .collect::<Vec<_>>(),
            vec!["one_way", "gap", "lift_off", "friction"]
        );
        for entry in &entries {
            assert!(
                entry.is_accepted_active_set_count_policy(),
                "{}",
                entry.nonlinear_class
            );
        }
    }

    #[test]
    fn governed_free_dof_force_moment_policy_covers_dec_046_classes() {
        let entries = governed_free_dof_force_moment_policy_entries();

        assert_eq!(entries.len(), 4);
        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.nonlinear_class)
                .collect::<Vec<_>>(),
            vec!["one_way", "gap", "lift_off", "friction"]
        );
        for entry in &entries {
            assert!(
                entry.is_accepted_free_dof_force_moment_policy(),
                "{}",
                entry.nonlinear_class
            );
        }
    }

    #[test]
    fn governed_free_dof_work_policy_covers_dec_046_classes() {
        let entries = governed_free_dof_work_policy_entries();

        assert_eq!(entries.len(), 4);
        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.nonlinear_class)
                .collect::<Vec<_>>(),
            vec!["one_way", "gap", "lift_off", "friction"]
        );
        for entry in &entries {
            assert!(
                entry.is_accepted_free_dof_work_policy(),
                "{}",
                entry.nonlinear_class
            );
        }
    }

    #[test]
    fn governed_multisupport_policy_covers_dec_046_non_seed_fixture() {
        let convergence_entries = governed_multisupport_convergence_policy_entries();
        let force_moment_entries = governed_multisupport_free_dof_force_moment_policy_entries();
        let work_entries = governed_multisupport_free_dof_work_policy_entries();
        let displacement_reaction_entries =
            governed_multisupport_displacement_reaction_delta_policy_entries();

        assert_eq!(convergence_entries.len(), 1);
        assert_eq!(force_moment_entries.len(), 1);
        assert_eq!(work_entries.len(), 1);
        assert_eq!(displacement_reaction_entries.len(), 1);
        assert!(convergence_entries[0].is_accepted_multisupport_active_set_count_policy());
        assert!(force_moment_entries[0].is_accepted_multisupport_free_dof_force_moment_policy());
        assert!(work_entries[0].is_accepted_multisupport_free_dof_work_policy());
        assert!(displacement_reaction_entries[0]
            .is_accepted_multisupport_displacement_reaction_delta_policy());
        assert_eq!(
            convergence_entries[0].evidence_fixture_ids,
            DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS
        );
        assert_eq!(
            force_moment_entries[0].evidence_fixture_ids,
            DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS
        );
        assert_eq!(
            work_entries[0].evidence_fixture_ids,
            DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS
        );
        assert_eq!(
            displacement_reaction_entries[0].evidence_fixture_ids,
            DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS
        );
    }

    #[test]
    fn displacement_reaction_delta_observation_entries_keep_thresholds_tbd() {
        let seed_entries = displacement_reaction_delta_observation_entries();
        let multisupport_entries = multisupport_displacement_reaction_delta_observation_entries();

        assert_eq!(seed_entries.len(), 4);
        assert_eq!(
            seed_entries
                .iter()
                .map(|entry| entry.nonlinear_class)
                .collect::<Vec<_>>(),
            vec!["one_way", "gap", "lift_off", "friction"]
        );
        for entry in &seed_entries {
            assert!(
                entry.is_tbd_seed_displacement_reaction_delta_observation(),
                "{}",
                entry.nonlinear_class
            );
        }

        assert_eq!(multisupport_entries.len(), 1);
        assert!(
            multisupport_entries[0].is_tbd_multisupport_displacement_reaction_delta_observation()
        );
        assert_eq!(
            multisupport_entries[0].evidence_fixture_ids,
            DEC_046_MULTISUPPORT_EVIDENCE_FIXTURE_IDS
        );
    }

    #[test]
    fn governed_displacement_reaction_delta_policy_covers_accepted_fixture_sets() {
        let seed_entries = governed_displacement_reaction_delta_policy_entries();
        let multisupport_entries =
            governed_multisupport_displacement_reaction_delta_policy_entries();

        assert_eq!(seed_entries.len(), 4);
        assert_eq!(
            seed_entries
                .iter()
                .map(|entry| entry.nonlinear_class)
                .collect::<Vec<_>>(),
            vec!["one_way", "gap", "lift_off", "friction"]
        );
        for entry in &seed_entries {
            assert!(
                entry.is_accepted_seed_displacement_reaction_delta_policy(),
                "{}",
                entry.nonlinear_class
            );
        }
        let gap_entry = seed_entries
            .iter()
            .find(|entry| entry.nonlinear_class == "gap")
            .expect("gap displacement/reaction delta policy entry exists");
        assert_eq!(
            gap_entry.translation_delta_absolute_limit,
            DEC_046_SEED_GAP_TRANSLATION_DELTA_ABSOLUTE_LIMIT
        );
        assert_eq!(
            gap_entry.force_reaction_delta_absolute_limit,
            DEC_046_SEED_GAP_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT
        );

        assert_eq!(multisupport_entries.len(), 1);
        assert!(
            multisupport_entries[0].is_accepted_multisupport_displacement_reaction_delta_policy()
        );
        assert_eq!(
            multisupport_entries[0].translation_delta_absolute_limit,
            DEC_046_MULTISUPPORT_TRANSLATION_DELTA_ABSOLUTE_LIMIT
        );
        assert_eq!(
            multisupport_entries[0].rotation_delta_absolute_limit,
            DEC_046_MULTISUPPORT_ROTATION_DELTA_ABSOLUTE_LIMIT
        );
        assert_eq!(
            multisupport_entries[0].force_reaction_delta_absolute_limit,
            DEC_046_MULTISUPPORT_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT
        );
        assert_eq!(
            multisupport_entries[0].moment_reaction_delta_absolute_limit,
            DEC_046_MULTISUPPORT_MOMENT_REACTION_DELTA_ABSOLUTE_LIMIT
        );
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

        for fixture in assembled_fixture_inventory() {
            assert!(fixture.provenance.is_publicly_usable());
            assert!(
                fixture.provenance.source_artifact_exists(repo_root),
                "{} provenance source is missing: {}",
                fixture.fixture_id,
                fixture.provenance.source_location
            );
            assert!(fixture.has_dimensioned_observations());
            assert!(!fixture.tolerance_policy_is_unresolved());
            assert!(fixture.uses_governed_convergence_policy());
        }

        for fixture in assembled_multisupport_depth_inventory() {
            assert!(fixture.provenance.is_publicly_usable());
            assert!(
                fixture.provenance.source_artifact_exists(repo_root),
                "{} provenance source is missing: {}",
                fixture.fixture_id,
                fixture.provenance.source_location
            );
            assert!(fixture.has_dimensioned_observations());
            assert!(fixture.tolerance_policy_is_unresolved());
            assert!(!fixture.uses_governed_convergence_policy());
        }

        for fixture in assembled_multisupport_acceptance_inventory() {
            assert!(fixture.provenance.is_publicly_usable());
            assert!(
                fixture.provenance.source_artifact_exists(repo_root),
                "{} provenance source is missing: {}",
                fixture.fixture_id,
                fixture.provenance.source_location
            );
            assert!(fixture.has_dimensioned_observations());
            assert!(!fixture.tolerance_policy_is_unresolved());
            assert!(fixture.uses_governed_multisupport_convergence_policy());
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
    fn assembled_global_loop_seed_cases_converge_under_governed_policy() {
        for fixture in assembled_fixture_inventory() {
            let solve = fixture.run().unwrap();

            assert!(fixture.matches_expected_outcome(), "{}", fixture.fixture_id);
            assert!(solve.converged, "{}", fixture.fixture_id);
            assert_eq!(
                solve.iterations.last().unwrap().active_set.residual_norm,
                0.0
            );
            assert!(!solve
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == SolverDiagnosticCode::TolerancePolicyTbd));
        }
    }

    #[test]
    fn assembled_convergence_observations_record_measured_fixture_values_only() {
        let observations = assembled_convergence_observations();

        assert_eq!(observations.len(), assembled_fixture_inventory().len());
        assert_eq!(
            observations
                .iter()
                .map(|observation| observation.fixture_id)
                .collect::<Vec<_>>(),
            vec![
                "NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL",
                "NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL",
                "NL-ASSEMBLED-LIFT-OFF-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-STICK-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL",
                "NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL",
            ]
        );

        for observation in &observations {
            assert!(observation.observed_converged, "{}", observation.fixture_id);
            assert_eq!(observation.final_residual_norm, 0.0);
            assert_eq!(observation.residual_unit, "count");
            assert_eq!(observation.residual_dimension, "dimensionless");
            assert!(
                observation.observed_iteration_count <= observation.max_iterations,
                "{}",
                observation.fixture_id
            );
            assert!(
                observation.uses_accepted_dec_046_active_set_policy(),
                "{}",
                observation.fixture_id
            );
        }

        assert_eq!(observations[0].observed_iteration_count, 2);
        assert_eq!(observations[1].observed_iteration_count, 2);
        assert_eq!(observations[2].observed_iteration_count, 2);
        assert_eq!(observations[3].observed_iteration_count, 1);
        assert_eq!(observations[4].observed_iteration_count, 2);
        assert_eq!(observations[5].observed_iteration_count, 1);
    }

    #[test]
    fn assembled_force_displacement_residual_observations_use_free_dof_force_moment_thresholds() {
        let observations = assembled_force_displacement_residual_observations();

        assert_eq!(observations.len(), assembled_fixture_inventory().len());
        for observation in &observations {
            assert!(
                observation.uses_accepted_free_dof_force_moment_threshold_policy(),
                "{}",
                observation.fixture_id
            );
            assert_eq!(observation.max_abs_free_dof_force_residual, 0.0);
            assert_eq!(observation.max_abs_free_dof_moment_residual, 0.0);
            assert_eq!(observation.max_abs_free_dof_work_residual, 0.0);
            assert_eq!(
                observation.free_dof_work_threshold_policy,
                Some(DEC_046_FREE_DOF_WORK_POLICY_REF)
            );
            assert!(observation.uses_accepted_displacement_reaction_delta_threshold_policy());
            assert_eq!(
                observation.displacement_reaction_delta_threshold_policy,
                Some(DEC_046_DISPLACEMENT_REACTION_DELTA_POLICY_REF)
            );
        }

        assert!(
            observations[0]
                .max_abs_translation_delta_from_previous
                .unwrap()
                > 0.0
        );
        assert!(
            observations[1]
                .max_abs_translation_delta_from_previous
                .unwrap()
                > 0.0
        );
        assert!(
            observations[2]
                .max_abs_translation_delta_from_previous
                .unwrap()
                > 0.0
        );
        assert_eq!(
            observations[3].max_abs_translation_delta_from_previous,
            None
        );
        assert!(
            observations[4]
                .max_abs_translation_delta_from_previous
                .unwrap()
                > 0.0
        );
        assert_eq!(
            observations[5].max_abs_translation_delta_from_previous,
            None
        );
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

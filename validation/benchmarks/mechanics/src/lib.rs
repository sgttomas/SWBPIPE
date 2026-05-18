//! Original mechanics verification benchmarks for OpenPipeStress.
//!
//! The fixtures in this crate use elementary open mechanics with invented
//! numeric values. They are verification aids only: no code-specific
//! acceptance criteria, protected standards content, or professional approval
//! claims are encoded here.

use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness, reduce_system, solve_dense, FrameElement, FrameKernelError,
    FrameNode, FrameSection, DOF_PER_NODE, RX, RY, RZ, UX, UY, UZ,
};
use open_pipe_stress_linear_supports::{
    apply_linear_supports, prepare_boundary, FrameDof, LinearSupport, QuantityDimension,
    SupportQuantity,
};
use open_pipe_stress_primitive_loads::{
    assemble_solver_load_vector, prepare_loads, prepare_lumped_nodal_loads,
    prepare_straight_pipe_axial_effects, ElementAxialEffectProperties, ElementLoadSpan,
    LoadDimension, LoadDirection, LoadQuantity, PrimitiveAxialEffectContribution, PrimitiveLoad,
    PrimitiveLoadCategory, SolverNodalLoadContribution,
};
use open_pipe_stress_result_export::{
    sorted_result_values, validate_result_envelope, AnalysisStatus, ChecksumRef,
    Diagnostic as ExportDiagnostic, DiagnosticClass as ExportDiagnosticClass,
    DiagnosticSeverity as ExportDiagnosticSeverity, DimensionId as ExportDimensionId,
    ProfessionalBoundary, Provenance as ExportProvenance, QuantityResult,
    RedistributionStatus as ExportRedistributionStatus, Reference, ReproducibilityRefs,
    ResultEnvelope, ResultFamily, ResultMetadata, ResultSet, ResultSetType, ResultTraceLink,
};
use open_pipe_stress_solver_diagnostics::{
    diagnostic_from_primitive_load_finding, diagnostics_from_support_application_error,
    DiagnosticSeverity, SolverDiagnosticCode,
};
use open_pipe_stress_straight_pipe::{
    LocalLoadDirection, PipeEnd, PointLocalForce, SpannedUniformLocalLoad, StraightPipeAxialEffect,
    StraightPipeElement, StraightPipeSectionProperties, UniformLoadSpan, UniformLocalLoad,
};
use open_pipe_stress_user_loads::{
    apply_straight_pipe_equivalent_user_loads,
    apply_straight_pipe_equivalent_user_loads_with_axial_effects,
    ElementLoadSpan as UserElementLoadSpan, RecoveryHookKind, UserLoad, UserLoadDirection,
    UserLoadQuantity,
};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const INTERNAL_ASSERTION_EPSILON: f64 = 1.0e-9;
const PKG09_FIXTURE_UNIT_SYSTEM_REF: &str = "PKG09-FIXTURE-UNITS-EXPLICIT-N-M-RAD-K";
const TP_PHYS_014_ANALYTICAL_PAYLOAD: &str =
    include_str!("../fixtures/tp_phys_014_canonical_analytical_payload.json");
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
pub enum BenchmarkFamily {
    Cantilever,
    Frame,
    StraightPipe,
    SupportBoundary,
    PrimitiveLoad,
    IntegratedLinearStatic,
    LoadToResultantIntegration,
    ThermalGrowth,
    ImposedDisplacement,
    StiffnessTransform,
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
    pub contributor_certification: &'static str,
    pub redistribution_status: RedistributionStatus,
    pub review_disposition: ReviewDisposition,
}

impl BenchmarkProvenance {
    pub fn public_original(source_location: &'static str) -> Self {
        Self {
            source_name: "OpenPipeStress original mechanics benchmark",
            source_location,
            source_license: "project-original-public-content",
            contributor: "OpenPipeStress agentic development workflow",
            contributor_certification:
                "Generated from elementary open mechanics; not copied from protected standards, commercial software examples, or proprietary data.",
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
                .contributor_certification
                .contains("not copied from protected standards")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureUnitBasis {
    pub unit_system_ref: &'static str,
    pub unit_system_status: &'static str,
    pub length_unit: &'static str,
    pub force_unit: &'static str,
    pub moment_unit: &'static str,
    pub stress_unit: &'static str,
    pub area_unit: &'static str,
    pub second_moment_area_unit: &'static str,
    pub section_modulus_unit: &'static str,
    pub linear_stiffness_unit: &'static str,
    pub mass_per_length_unit: &'static str,
    pub acceleration_unit: &'static str,
    pub rotation_unit: &'static str,
    pub temperature_interval_unit: &'static str,
    pub dimensionless_unit: &'static str,
    pub distributed_load_unit: &'static str,
    pub note: &'static str,
}

impl FixtureUnitBasis {
    pub fn is_explicit_fixture_basis(&self) -> bool {
        self.unit_system_ref == PKG09_FIXTURE_UNIT_SYSTEM_REF
            && self.unit_system_status == "fixture-local-explicit-units-no-conversions"
            && !self.length_unit.is_empty()
            && !self.force_unit.is_empty()
            && !self.moment_unit.is_empty()
            && !self.stress_unit.is_empty()
            && !self.distributed_load_unit.is_empty()
            && self.note.contains("unit catalog remains TBD")
    }
}

pub const FIXTURE_UNIT_BASIS: FixtureUnitBasis = FixtureUnitBasis {
    unit_system_ref: PKG09_FIXTURE_UNIT_SYSTEM_REF,
    unit_system_status: "fixture-local-explicit-units-no-conversions",
    length_unit: "m",
    force_unit: "N",
    moment_unit: "N-m",
    stress_unit: "Pa",
    area_unit: "m^2",
    second_moment_area_unit: "m^4",
    section_modulus_unit: "m^3",
    linear_stiffness_unit: "N/m",
    mass_per_length_unit: "kg/m",
    acceleration_unit: "m/s^2",
    rotation_unit: "rad",
    temperature_interval_unit: "K",
    dimensionless_unit: "ratio",
    distributed_load_unit: "N/m",
    note: "Explicit fixture-local unit identifiers only; project unit catalog remains TBD and no conversion constants are encoded.",
};

#[derive(Debug, Clone, PartialEq)]
pub struct ExpectedValue {
    pub name: &'static str,
    pub value: f64,
    pub unit: &'static str,
    pub dimension: &'static str,
    pub tolerance_policy: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MechanicsBenchmark {
    pub fixture_id: &'static str,
    pub family: BenchmarkFamily,
    pub description: &'static str,
    pub assumptions: &'static [&'static str],
    pub provenance: BenchmarkProvenance,
    pub unit_basis: FixtureUnitBasis,
    pub expected_values: Vec<ExpectedValue>,
}

impl MechanicsBenchmark {
    pub fn tolerance_policy_is_unresolved(&self) -> bool {
        self.expected_values
            .iter()
            .all(|value| value.tolerance_policy.is_none())
    }

    pub fn has_dimensioned_expected_values(&self) -> bool {
        self.expected_values.iter().all(|value| {
            value.value.is_finite()
                && !value.unit.is_empty()
                && CANONICAL_DIMENSIONS.contains(&value.dimension)
        }) && self.unit_basis.is_explicit_fixture_basis()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinearStaticIntegrationResult {
    pub node_1_ux_displacement: f64,
    pub node_1_uy_displacement: f64,
    pub node_1_uz_displacement: f64,
    pub lumped_node_0_uy_force: f64,
    pub lumped_node_1_uy_force: f64,
    pub recovered_local_axial_force_j: f64,
    pub recovered_local_shear_y_j: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadToResultantIntegrationResult {
    pub node_1_uy_displacement: f64,
    pub node_1_rz_rotation: f64,
    pub assembled_node_0_uy_force: f64,
    pub assembled_node_0_rz_moment: f64,
    pub midspan_shear_y: f64,
    pub midspan_bending_z: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrientedLoadToResultantIntegrationResult {
    pub node_1_ux_displacement: f64,
    pub node_1_rz_rotation: f64,
    pub assembled_node_0_ux_force: f64,
    pub assembled_node_0_rz_moment: f64,
    pub midspan_shear_y: f64,
    pub midspan_bending_z: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartialSpanLoadToResultantIntegrationResult {
    pub node_1_uy_displacement: f64,
    pub node_1_rz_rotation: f64,
    pub assembled_node_0_uy_force: f64,
    pub assembled_node_0_rz_moment: f64,
    pub assembled_node_1_uy_force: f64,
    pub assembled_node_1_rz_moment: f64,
    pub midspan_shear_y: f64,
    pub midspan_bending_z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StationSweepResultant {
    pub station_fraction: f64,
    pub shear_y: f64,
    pub bending_z: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StationSweepResultantIntegrationResult {
    pub station_resultants: Vec<StationSweepResultant>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AxialEffectStationResultant {
    pub station_fraction: f64,
    pub axial_force: f64,
    pub shear_y: f64,
    pub bending_z: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThermalPressureAxialEffectsResult {
    pub thermal_axial_force: f64,
    pub pressure_thrust_force: f64,
    pub total_axial_effect_force: f64,
    pub equivalent_node_i_axial_load: f64,
    pub equivalent_node_j_axial_load: f64,
    pub recovered_local_i_axial_force: f64,
    pub recovered_local_j_axial_force: f64,
    pub end_i_axial_force: f64,
    pub end_j_axial_force: f64,
    pub midspan_axial_force: f64,
    pub station_sweep: Vec<AxialEffectStationResultant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CombinedLoadAxialEffectsResult {
    pub thermal_axial_force: f64,
    pub pressure_thrust_force: f64,
    pub total_axial_effect_force: f64,
    pub assembled_node_0_ux_force: f64,
    pub assembled_node_0_uy_force: f64,
    pub assembled_node_0_rz_moment: f64,
    pub assembled_node_1_ux_force: f64,
    pub assembled_node_1_uy_force: f64,
    pub assembled_node_1_rz_moment: f64,
    pub node_1_ux_displacement: f64,
    pub node_1_uy_displacement: f64,
    pub node_1_rz_rotation: f64,
    pub end_i_axial_force: f64,
    pub end_i_shear_y: f64,
    pub end_i_bending_z: f64,
    pub midspan_axial_force: f64,
    pub midspan_shear_y: f64,
    pub midspan_bending_z: f64,
    pub station_sweep: Vec<AxialEffectStationResultant>,
    pub distributed_recovery_hook_count: usize,
    pub axial_effect_recovery_hook_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalAnalyticalPayloadSolverResult {
    pub distributed_load_count: usize,
    pub point_force_count: usize,
    pub y_reference: [f64; 3],
    pub assembled_node_0_uy_force: f64,
    pub assembled_node_0_rz_moment: f64,
    pub assembled_node_1_uy_force: f64,
    pub assembled_node_1_rz_moment: f64,
    pub support_reaction_node_0_uy_force: f64,
    pub support_reaction_node_0_rz_moment: f64,
    pub node_1_uy_displacement: f64,
    pub node_1_rz_rotation: f64,
    pub midspan_shear_y: f64,
    pub midspan_bending_z: f64,
    pub distributed_recovery_hook_count: usize,
    pub point_force_recovery_hook_count: usize,
    pub load_vector_trace_chains: Vec<CanonicalResultTraceChain>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalResultTraceChain {
    pub result_id: String,
    pub trace_chain: Vec<ResultTraceLink>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalSolveResultEnvelopeEvidence {
    pub envelope: ResultEnvelope,
    pub export_validation_diagnostic_count: usize,
    pub quantity_result_count: usize,
    pub envelope_diagnostic_count: usize,
}

pub fn fixture_inventory() -> Vec<MechanicsBenchmark> {
    vec![
        cantilever_tip_force_fixture(),
        portal_frame_sway_fixture(),
        straight_pipe_weight_recovery_fixture(),
        support_boundary_fixture(),
        primitive_load_preparation_fixture(),
        tp_phys_002_linear_static_integration_fixture(),
        tp_phys_004_load_to_resultant_fixture(),
        tp_phys_005_oriented_load_to_resultant_fixture(),
        tp_phys_006_partial_span_load_to_resultant_fixture(),
        tp_phys_007_station_sweep_resultants_fixture(),
        fixed_fixed_thermal_fixture(),
        tp_phys_008_thermal_pressure_axial_effects_fixture(),
        tp_phys_009_combined_load_axial_effects_fixture(),
        tp_phys_014_canonical_analytical_payload_fixture(),
        tp_phys_015a_canonical_solve_result_envelope_fixture(),
        imposed_displacement_spring_fixture(),
        inclined_member_transform_fixture(),
    ]
}

pub fn missing_required_families(fixtures: &[MechanicsBenchmark]) -> Vec<BenchmarkFamily> {
    let required = [
        BenchmarkFamily::Cantilever,
        BenchmarkFamily::Frame,
        BenchmarkFamily::StraightPipe,
        BenchmarkFamily::SupportBoundary,
        BenchmarkFamily::PrimitiveLoad,
        BenchmarkFamily::IntegratedLinearStatic,
        BenchmarkFamily::LoadToResultantIntegration,
        BenchmarkFamily::ThermalGrowth,
        BenchmarkFamily::ImposedDisplacement,
        BenchmarkFamily::StiffnessTransform,
    ];

    required
        .into_iter()
        .filter(|family| !fixtures.iter().any(|fixture| fixture.family == *family))
        .collect()
}

pub fn cantilever_tip_force_fixture() -> MechanicsBenchmark {
    let length: f64 = 10.0;
    let elastic_modulus: f64 = 1200.0;
    let second_moment_z: f64 = 4.0;
    let tip_force: f64 = 6.0;
    let tip_displacement_y = tip_force * length.powi(3) / (3.0 * elastic_modulus * second_moment_z);
    let fixed_end_moment_z = tip_force * length;

    MechanicsBenchmark {
        fixture_id: "MECH-CANTILEVER-TIP-FORCE",
        family: BenchmarkFamily::Cantilever,
        description:
            "Two-node cantilever with invented lateral tip force in the local y/global Y direction.",
        assumptions: &[
            "Euler-Bernoulli frame stiffness as implemented by the frame kernel.",
            "Node 0 restrained in all six degrees of freedom.",
            "Node 1 free in all six degrees of freedom.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/cantilever_tip_force.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "tip_displacement_y",
                value: tip_displacement_y,
                unit: "m",
                dimension: "length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "fixed_end_moment_z",
                value: fixed_end_moment_z,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn portal_frame_sway_fixture() -> MechanicsBenchmark {
    MechanicsBenchmark {
        fixture_id: "MECH-PORTAL-SWAY-ORIGINAL",
        family: BenchmarkFamily::Frame,
        description: "Original two-column portal-frame assembly used as a deterministic frame assembly smoke benchmark.",
        assumptions: &[
            "Three frame elements are assembled into one global stiffness matrix.",
            "Base nodes are restrained in all six degrees of freedom.",
            "A lateral nodal force is applied at the upper-right joint.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/portal_frame_sway.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![ExpectedValue {
            name: "top_right_sway_x",
            value: solve_portal_frame_sway().expect("fixture construction must remain valid"),
            unit: "m",
            dimension: "length",
            tolerance_policy: None,
        }],
    }
}

pub fn fixed_fixed_thermal_fixture() -> MechanicsBenchmark {
    let elastic_modulus = 2000.0;
    let area = 3.0;
    let alpha = 1.2e-5;
    let delta_temperature = 75.0;
    let restrained_force = elastic_modulus * area * alpha * delta_temperature;

    MechanicsBenchmark {
        fixture_id: "MECH-FIXED-FIXED-THERMAL-AXIAL",
        family: BenchmarkFamily::ThermalGrowth,
        description: "Fixed-fixed axial member with invented thermal strain and fully restrained free expansion.",
        assumptions: &[
            "Uniform temperature change over a prismatic member.",
            "Both axial ends are restrained.",
            "Expected axial force magnitude follows open mechanics EA alpha delta_T.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/fixed_fixed_thermal_axial.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "free_thermal_strain",
                value: alpha * delta_temperature,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "restrained_axial_force_magnitude",
                value: restrained_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_008_thermal_pressure_axial_effects_fixture() -> MechanicsBenchmark {
    let result = solve_tp_phys_008_thermal_pressure_axial_effects()
        .expect("TP-PHYS-008 fixture construction must remain valid");

    MechanicsBenchmark {
        fixture_id: "MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS",
        family: BenchmarkFamily::ThermalGrowth,
        description: "Invented fixed-fixed straight-pipe axial-effect fixture combining thermal restraint and closed-end pressure thrust.",
        assumptions: &[
            "single straight two-node pipe aligned to the global X axis",
            "both end translations are fixed for the axial-effect recovery check",
            "uniform thermal effect uses F = E A alpha DeltaT",
            "closed-end pressure thrust uses F = p A_internal",
            "axial-effect station resultants are mechanics quantities only, with no rule checks or allowables",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/tp_phys_008_thermal_pressure_axial_effects.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "thermal_axial_force",
                value: result.thermal_axial_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "pressure_thrust_force",
                value: result.pressure_thrust_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "total_axial_effect_force",
                value: result.total_axial_effect_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "equivalent_node_i_axial_load",
                value: result.equivalent_node_i_axial_load,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "equivalent_node_j_axial_load",
                value: result.equivalent_node_j_axial_load,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "recovered_local_i_axial_force",
                value: result.recovered_local_i_axial_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "recovered_local_j_axial_force",
                value: result.recovered_local_j_axial_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_axial_force",
                value: result.midspan_axial_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_009_combined_load_axial_effects_fixture() -> MechanicsBenchmark {
    let result = solve_tp_phys_009_combined_load_axial_effects()
        .expect("TP-PHYS-009 fixture construction must remain valid");

    MechanicsBenchmark {
        fixture_id: "MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS",
        family: BenchmarkFamily::LoadToResultantIntegration,
        description: "Invented straight-pipe fixture combining explicit distributed user-load assembly with prepared thermal and pressure axial effects.",
        assumptions: &[
            "single straight two-node pipe aligned to the global X axis",
            "primitive thermal and pressure loads are prepared as straight-pipe axial effects before user-load assembly",
            "one local/global Y distributed user load acts over span fractions 0.25 to 0.75",
            "node 0 is fixed and node 1 axial translation is restrained so axial effects remain visible in resultants",
            "combined resultants are mechanics quantities only, with no rule checks or allowables",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/tp_phys_009_combined_load_axial_effects.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "thermal_axial_force",
                value: result.thermal_axial_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "pressure_thrust_force",
                value: result.pressure_thrust_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "total_axial_effect_force",
                value: result.total_axial_effect_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_0_ux_equivalent_load",
                value: result.assembled_node_0_ux_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_0_uy_equivalent_load",
                value: result.assembled_node_0_uy_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_0_rz_equivalent_moment",
                value: result.assembled_node_0_rz_moment,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_ux_equivalent_load",
                value: result.assembled_node_1_ux_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_uy_equivalent_load",
                value: result.assembled_node_1_uy_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_rz_equivalent_moment",
                value: result.assembled_node_1_rz_moment,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_ux_displacement",
                value: result.node_1_ux_displacement,
                unit: "m",
                dimension: "displacement",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_uy_displacement",
                value: result.node_1_uy_displacement,
                unit: "m",
                dimension: "displacement",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_rz_rotation",
                value: result.node_1_rz_rotation,
                unit: "rad",
                dimension: "rotation",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "end_i_axial_force",
                value: result.end_i_axial_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "end_i_shear_y",
                value: result.end_i_shear_y,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "end_i_bending_z",
                value: result.end_i_bending_z,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_axial_force",
                value: result.midspan_axial_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_shear_y",
                value: result.midspan_shear_y,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_bending_z",
                value: result.midspan_bending_z,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_014_canonical_analytical_payload_fixture() -> MechanicsBenchmark {
    let result = solve_tp_phys_014_canonical_analytical_payload()
        .expect("TP-PHYS-014 fixture construction must remain valid");

    MechanicsBenchmark {
        fixture_id: "MECH-TP-PHYS-014-CANONICAL-ANALYTICAL-PAYLOAD",
        family: BenchmarkFamily::LoadToResultantIntegration,
        description: "Invented canonical analytical payload fixture that extracts governed straight-pipe orientation, material/section properties, typed distributed force-per-length and element point-force records, then consumes them through current straight-pipe and user-load solver APIs.",
        assumptions: &[
            "payload is an analytical_solver_model with no preview-only solver defaults",
            "straight pipe is aligned to global X with governed y_reference aligned to global Y",
            "one element_uniform_distributed_force record supplies force_per_length over the full span",
            "one element_point_force record supplies station_fraction and force",
            "node N-1 anchor support is read from the payload before solving",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/tp_phys_014_canonical_analytical_payload.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "distributed_load_count",
                value: result.distributed_load_count as f64,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "point_force_count",
                value: result.point_force_count as f64,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_0_uy_equivalent_load",
                value: result.assembled_node_0_uy_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_0_rz_equivalent_moment",
                value: result.assembled_node_0_rz_moment,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_uy_equivalent_load",
                value: result.assembled_node_1_uy_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_rz_equivalent_moment",
                value: result.assembled_node_1_rz_moment,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_uy_displacement",
                value: result.node_1_uy_displacement,
                unit: "m",
                dimension: "displacement",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_rz_rotation",
                value: result.node_1_rz_rotation,
                unit: "rad",
                dimension: "rotation",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_shear_y",
                value: result.midspan_shear_y,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_bending_z",
                value: result.midspan_bending_z,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_015a_canonical_solve_result_envelope_fixture() -> MechanicsBenchmark {
    let evidence = tp_phys_015a_canonical_solve_result_envelope()
        .expect("TP-PHYS-015A result-envelope evidence must remain valid");
    let result = solve_tp_phys_014_canonical_analytical_payload()
        .expect("TP-PHYS-014 fixture construction must remain valid");

    MechanicsBenchmark {
        fixture_id: "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE",
        family: BenchmarkFamily::LoadToResultantIntegration,
        description: "Invented canonical analytical payload result-boundary fixture that represents solved displacement, reaction/load-vector evidence, station resultants, diagnostics, provenance, and source references using existing result-export vocabulary.",
        assumptions: &[
            "payload source remains the TP-PHYS-014 analytical_solver_model JSON fixture",
            "result envelope is validation-local in-memory evidence and does not add public export, CLI, GUI, report, or persistence behavior",
            "result quantities use existing result-export Reference, QuantityResult, ResultSet, Diagnostic, Provenance, and ProfessionalBoundary vocabulary",
            "export/headless integration remains TBD for adjacent deliverables",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/tp_phys_015a_canonical_solve_result_envelope.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "quantity_result_count",
                value: evidence.quantity_result_count as f64,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "envelope_diagnostic_count",
                value: evidence.envelope_diagnostic_count as f64,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "export_validation_diagnostic_count",
                value: evidence.export_validation_diagnostic_count as f64,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "support_reaction_node_0_uy_force",
                value: result.support_reaction_node_0_uy_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "support_reaction_node_0_rz_moment",
                value: result.support_reaction_node_0_rz_moment,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_shear_y",
                value: result.midspan_shear_y,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_bending_z",
                value: result.midspan_bending_z,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn straight_pipe_weight_recovery_fixture() -> MechanicsBenchmark {
    let mass_per_length = 2.5;
    let gravity = 9.0;
    let weight_force_per_length = mass_per_length * gravity;
    let axial_force = 1200.0 * 3.0 * 0.01 / 2.0;

    MechanicsBenchmark {
        fixture_id: "MECH-STRAIGHT-PIPE-WEIGHT-RECOVERY",
        family: BenchmarkFamily::StraightPipe,
        description:
            "Straight pipe boundary fixture for explicit weight hook and local axial recovery.",
        assumptions: &[
            "Pipe section values are invented and passed through the straight-pipe adapter.",
            "Weight requires explicit mass per length and explicit gravity.",
            "Local axial recovery follows EA delta / L for a two-node straight member.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/straight_pipe_weight_recovery.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "weight_force_per_length",
                value: weight_force_per_length,
                unit: "N/m",
                dimension: "TBD",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "recovered_local_axial_force_j",
                value: axial_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn support_boundary_fixture() -> MechanicsBenchmark {
    MechanicsBenchmark {
        fixture_id: "MECH-SUPPORT-BOUNDARY-MIXED",
        family: BenchmarkFamily::SupportBoundary,
        description:
            "Mixed support-boundary fixture with anchor, translational spring, and imposed rotation.",
        assumptions: &[
            "Node 0 anchor contributes all six restrained frame DOFs.",
            "Node 1 spring contributes an explicit translational stiffness entry.",
            "Node 2 imposed rotation contributes one restrained rotational DOF and one imposed value.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/support_boundary_mixed.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "restrained_dof_count",
                value: 7.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "spring_stiffness",
                value: 250.0,
                unit: "N/m",
                dimension: "linear_stiffness",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "imposed_rotation",
                value: 0.015,
                unit: "rad",
                dimension: "rotation",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn primitive_load_preparation_fixture() -> MechanicsBenchmark {
    MechanicsBenchmark {
        fixture_id: "MECH-PRIMITIVE-LOAD-PREP",
        family: BenchmarkFamily::PrimitiveLoad,
        description:
            "Primitive-load fixture with accumulated nodal force, uniform weight, and imposed displacement contributions.",
        assumptions: &[
            "Nodal mechanics loads accumulate into the global load vector.",
            "Uniform element weight remains an element-boundary contribution.",
            "Primitive imposed displacement is routed to support-boundary style data.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/primitive_load_preparation.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "node_1_global_y_force",
                value: 5.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "uniform_weight_force_per_length",
                value: 1.25,
                unit: "N/m",
                dimension: "TBD",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "imposed_uz_displacement",
                value: -0.02,
                unit: "m",
                dimension: "length",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_002_linear_static_integration_fixture() -> MechanicsBenchmark {
    let result = solve_tp_phys_002_linear_static_integration()
        .expect("TP-PHYS-002 fixture construction must remain valid");

    MechanicsBenchmark {
        fixture_id: "MECH-TP-PHYS-002-LINEAR-STATIC-INTEGRATION",
        family: BenchmarkFamily::IntegratedLinearStatic,
        description: "Integrated invented linear static case covering load preparation, support application, frame solve, straight-pipe force recovery, and diagnostics.",
        assumptions: &[
            "Two-node straight pipe element along global X with invented section values.",
            "A nodal axial force is combined with a lumped uniform global Y element load.",
            "Node 0 is anchored; node 1 has a translational spring and an imposed displacement.",
            "The reduced frame solve is dense and deterministic for this small verification fixture.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/tp_phys_002_linear_static_integration.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "lumped_node_0_uy_force",
                value: result.lumped_node_0_uy_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "lumped_node_1_uy_force",
                value: result.lumped_node_1_uy_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_ux_displacement",
                value: result.node_1_ux_displacement,
                unit: "m",
                dimension: "length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_uy_displacement",
                value: result.node_1_uy_displacement,
                unit: "m",
                dimension: "length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_uz_prescribed_displacement",
                value: result.node_1_uz_displacement,
                unit: "m",
                dimension: "length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "recovered_local_axial_force_j",
                value: result.recovered_local_axial_force_j,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "recovered_local_shear_y_j",
                value: result.recovered_local_shear_y_j,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_004_load_to_resultant_fixture() -> MechanicsBenchmark {
    MechanicsBenchmark {
        fixture_id: "MECH-TP-PHYS-004-LOAD-TO-RESULTANT",
        family: BenchmarkFamily::LoadToResultantIntegration,
        description: "Invented straight-pipe load assembly to displacement and station-resultant integration fixture.",
        assumptions: &[
            "single straight two-node pipe aligned to the global X axis",
            "node 0 is fixed; node 1 is free",
            "full-span local/global Y distributed load and one interior point force are explicit user loads",
            "station resultants are mechanics quantities only, with no rule/code acceptance logic",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/tp_phys_004_load_to_resultant.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "node_1_uy_displacement",
                value: -0.04533333333333334,
                unit: "m",
                dimension: "displacement",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_rz_rotation",
                value: -0.014666666666666668,
                unit: "rad",
                dimension: "rotation",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "assembled_node_0_uy_force",
                value: -6.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "assembled_node_0_rz_moment",
                value: -4.666666666666667,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_shear_y",
                value: 4.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_bending_z",
                value: 4.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_005_oriented_load_to_resultant_fixture() -> MechanicsBenchmark {
    MechanicsBenchmark {
        fixture_id: "MECH-TP-PHYS-005-ORIENTED-LOAD-TO-RESULTANT",
        family: BenchmarkFamily::LoadToResultantIntegration,
        description: "Invented orientation-aware straight-pipe global load transformation to displacement and station-resultant fixture.",
        assumptions: &[
            "single straight two-node pipe aligned to the global Y axis",
            "local y reference is global X, so global X user loads transform into local Y load effects",
            "node 0 is fixed; node 1 is free",
            "station resultants are local mechanics quantities only, with no rule/code acceptance logic",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/tp_phys_005_oriented_load_to_resultant.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "node_1_ux_displacement",
                value: -0.04533333333333334,
                unit: "m",
                dimension: "displacement",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_rz_rotation",
                value: 0.014666666666666668,
                unit: "rad",
                dimension: "rotation",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "assembled_node_0_ux_force",
                value: -6.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "assembled_node_0_rz_moment",
                value: 4.666666666666667,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_shear_y",
                value: 4.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_bending_z",
                value: 4.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_006_partial_span_load_to_resultant_fixture() -> MechanicsBenchmark {
    MechanicsBenchmark {
        fixture_id: "MECH-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-RESULTANT",
        family: BenchmarkFamily::LoadToResultantIntegration,
        description: "Invented partial-span straight-pipe distributed-load fixture covering equivalent nodal loads, displacement solve, and station-resultant recovery.",
        assumptions: &[
            "single straight two-node pipe aligned to the global X axis",
            "node 0 is fixed; node 1 is free",
            "one local/global Y uniform distributed user load acts only over span fractions 0.25 to 0.75",
            "station resultants are mechanics quantities only, with no rule/code acceptance logic",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/tp_phys_006_partial_span_load_to_resultant.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "node_0_uy_equivalent_load",
                value: -2.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_0_rz_equivalent_moment",
                value: -11.0 / 6.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_uy_equivalent_load",
                value: -2.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_rz_equivalent_moment",
                value: 11.0 / 6.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_uy_displacement",
                value: -7.0 / 500.0,
                unit: "m",
                dimension: "displacement",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "node_1_rz_rotation",
                value: -13.0 / 3000.0,
                unit: "rad",
                dimension: "rotation",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_shear_y",
                value: 2.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "midspan_bending_z",
                value: 1.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_007_station_sweep_resultants_fixture() -> MechanicsBenchmark {
    MechanicsBenchmark {
        fixture_id: "MECH-TP-PHYS-007-STATION-SWEEP-RESULTANTS",
        family: BenchmarkFamily::LoadToResultantIntegration,
        description: "Invented ordered station-resultant sweep fixture for a fixed-free straight pipe with a partial-span distributed load.",
        assumptions: &[
            "single straight two-node pipe aligned to the global X axis",
            "node 0 is fixed; node 1 is free",
            "one local/global Y uniform distributed user load acts only over span fractions 0.25 to 0.75",
            "station fractions are intentionally requested out of geometric order",
            "station resultants are mechanics quantities only, with no rule/code acceptance logic",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/tp_phys_007_station_sweep_resultants.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "requested_station_fraction_0",
                value: 0.75,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_0_shear_y",
                value: 0.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_0_bending_z",
                value: 0.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "requested_station_fraction_1",
                value: 0.25,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_1_shear_y",
                value: 4.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_1_bending_z",
                value: 4.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "requested_station_fraction_2",
                value: 0.5,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_2_shear_y",
                value: 2.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_2_bending_z",
                value: 1.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "requested_station_fraction_3",
                value: 1.0,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_3_shear_y",
                value: 0.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_3_bending_z",
                value: 0.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn imposed_displacement_spring_fixture() -> MechanicsBenchmark {
    let stiffness = 150.0;
    let imposed_displacement = 0.04;
    let reaction = stiffness * imposed_displacement;

    MechanicsBenchmark {
        fixture_id: "MECH-IMPOSED-DISPLACEMENT-SPRING",
        family: BenchmarkFamily::ImposedDisplacement,
        description: "Single translational spring with an invented imposed displacement.",
        assumptions: &[
            "Linear spring reaction follows k times imposed displacement.",
            "The support model records the imposed displacement without solving a full frame system.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/imposed_displacement_spring.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![ExpectedValue {
            name: "spring_reaction_force",
            value: reaction,
            unit: "N",
            dimension: "force",
            tolerance_policy: None,
        }],
    }
}

pub fn inclined_member_transform_fixture() -> MechanicsBenchmark {
    let direction_cosine = 1.0 / 2.0_f64.sqrt();

    MechanicsBenchmark {
        fixture_id: "MECH-INCLINED-MEMBER-TRANSFORM",
        family: BenchmarkFamily::StiffnessTransform,
        description: "Inclined two-node frame member with 45-degree global XY projection.",
        assumptions: &[
            "Local x axis is the normalized node-to-node vector.",
            "Global stiffness transform preserves matrix symmetry.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/inclined_member_transform.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "local_x_global_x_component",
                value: direction_cosine,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "local_x_global_y_component",
                value: direction_cosine,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn solve_cantilever_tip_force() -> Result<f64, FrameKernelError> {
    let section = benchmark_section()?;
    let element = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0])?,
        FrameNode::new(1, [10.0, 0.0, 0.0])?,
        section,
        [0.0, 1.0, 0.0],
    )?;
    let stiffness = assemble_global_stiffness(2, &[element])?;
    let mut force = vec![0.0; 2 * DOF_PER_NODE];
    force[DOF_PER_NODE + UY] = 6.0;
    let reduced = reduce_system(&stiffness, &force, &[UX, UY, UZ, RX, RY, RZ])?;
    let displacement = solve_dense(&reduced.stiffness, &reduced.force)?;
    let reduced_index = reduced
        .free_dofs
        .iter()
        .position(|&dof| dof == DOF_PER_NODE + UY)
        .expect("tip UY is free in this fixture");
    Ok(displacement[reduced_index])
}

pub fn solve_portal_frame_sway() -> Result<f64, FrameKernelError> {
    let section = FrameSection::new(1800.0, 700.0, 2.5, 2.0, 2.0, 1.0)?;
    let nodes = [
        FrameNode::new(0, [0.0, 0.0, 0.0])?,
        FrameNode::new(1, [0.0, 4.0, 0.0])?,
        FrameNode::new(2, [6.0, 4.0, 0.0])?,
        FrameNode::new(3, [6.0, 0.0, 0.0])?,
    ];
    let elements = [
        FrameElement::new(nodes[0], nodes[1], section, [1.0, 0.0, 0.0])?,
        FrameElement::new(nodes[1], nodes[2], section, [0.0, 1.0, 0.0])?,
        FrameElement::new(nodes[3], nodes[2], section, [1.0, 0.0, 0.0])?,
    ];
    let stiffness = assemble_global_stiffness(nodes.len(), &elements)?;
    let load = PrimitiveLoad::nodal_force(
        "portal-lateral-load",
        PrimitiveLoadCategory::Occasional,
        2,
        LoadDirection::GlobalX,
        LoadQuantity::new(3.0, LoadDimension::Force).expect("fixture load is finite"),
    );
    let load_application = prepare_loads(nodes.len(), elements.len(), &[load]);
    assert!(
        !load_application.is_blocked(),
        "portal frame fixture load preparation should not have findings"
    );
    let force = load_application.global_load_vector(nodes.len());
    let base_restraints = all_node_dofs(0)
        .into_iter()
        .chain(all_node_dofs(3))
        .collect::<Vec<_>>();
    let reduced = reduce_system(&stiffness, &force, &base_restraints)?;
    let displacement = solve_dense(&reduced.stiffness, &reduced.force)?;
    let reduced_index = reduced
        .free_dofs
        .iter()
        .position(|&dof| dof == 2 * DOF_PER_NODE + UX)
        .expect("top-right UX is free in this fixture");
    Ok(displacement[reduced_index])
}

pub fn validate_imposed_displacement_fixture() -> bool {
    let stiffness = SupportQuantity::positive(150.0, QuantityDimension::TranslationalStiffness)
        .expect("fixture stiffness is positive");
    let imposed = SupportQuantity::new(0.04, QuantityDimension::Displacement)
        .expect("fixture displacement is finite");
    let supports = [
        LinearSupport::spring("fixture-spring", 0, FrameDof::Ux, Some(stiffness.clone())),
        LinearSupport::imposed_displacement(
            "fixture-imposed",
            0,
            FrameDof::Ux,
            Some(imposed.clone()),
        ),
    ];
    let preparation = prepare_boundary(1, &supports);
    !preparation.is_blocked()
        && preparation.springs.len() == 1
        && preparation.imposed_displacements.len() == 1
        && (stiffness.value * imposed.value
            - imposed_displacement_spring_fixture().expected_values[0].value)
            .abs()
            <= INTERNAL_ASSERTION_EPSILON
}

pub fn validate_straight_pipe_boundary() -> bool {
    let section = StraightPipeSectionProperties::new(1200.0, 500.0, 3.0, 3.0, 4.0, 1.5, Some(2.5))
        .expect("fixture section is valid");
    let pipe = StraightPipeElement::new(
        "straight-pipe-cantilever",
        FrameNode::new(1, [0.0, 0.0, 0.0]).expect("fixture node is valid"),
        FrameNode::new(3, [2.0, 0.0, 0.0]).expect("fixture node is valid"),
        section,
        [0.0, 1.0, 0.0],
    )
    .expect("fixture pipe is valid");
    let mut model_displacements = vec![0.0; 4 * DOF_PER_NODE];
    model_displacements[3 * DOF_PER_NODE + UX] = 0.01;
    let Ok(weight) = pipe.weight_hook(9.0) else {
        return false;
    };
    let Ok(recovered) = pipe.recover_local_forces_from_global_model(&model_displacements) else {
        return false;
    };
    let fixture = straight_pipe_weight_recovery_fixture();
    pipe.local_stiffness().is_ok()
        && pipe.global_stiffness().is_ok()
        && (pipe.length().expect("fixture length is valid") - 2.0).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (weight.weight_force_per_length - fixture.expected_values[0].value).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (recovered.local_forces[DOF_PER_NODE + UX] - fixture.expected_values[1].value).abs()
            <= INTERNAL_ASSERTION_EPSILON
}

pub fn validate_transform_fixture() -> Result<bool, FrameKernelError> {
    let section = benchmark_section()?;
    let element = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0])?,
        FrameNode::new(1, [1.0, 1.0, 0.0])?,
        section,
        [0.0, 0.0, 1.0],
    )?;
    let orientation = element.orientation()?;
    let global = element.global_stiffness()?;
    let expected = 1.0 / 2.0_f64.sqrt();
    Ok(
        (orientation.local_axes[0][0] - expected).abs() <= INTERNAL_ASSERTION_EPSILON
            && (orientation.local_axes[0][1] - expected).abs() <= INTERNAL_ASSERTION_EPSILON
            && matrix_is_symmetric(&global),
    )
}

pub fn validate_support_boundary_fixture() -> bool {
    let spring = SupportQuantity::positive(250.0, QuantityDimension::TranslationalStiffness)
        .expect("fixture spring stiffness is positive");
    let imposed_rotation = SupportQuantity::new(0.015, QuantityDimension::Rotation)
        .expect("fixture rotation is finite");
    let supports = [
        LinearSupport::anchor("fixture-anchor", 0),
        LinearSupport::spring("fixture-spring", 1, FrameDof::Uy, Some(spring)),
        LinearSupport::imposed_displacement(
            "fixture-imposed-rotation",
            2,
            FrameDof::Rz,
            Some(imposed_rotation),
        ),
    ];
    let preparation = prepare_boundary(3, &supports);
    let fixture = support_boundary_fixture();
    !preparation.is_blocked()
        && preparation.restrained_dofs.len() as f64 == fixture.expected_values[0].value
        && preparation.springs.len() == 1
        && preparation.imposed_displacements.len() == 1
        && (preparation.springs[0].stiffness.value - fixture.expected_values[1].value).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (preparation.imposed_displacements[0].displacement.value
            - fixture.expected_values[2].value)
            .abs()
            <= INTERNAL_ASSERTION_EPSILON
}

pub fn validate_primitive_load_fixture() -> bool {
    let loads = [
        PrimitiveLoad::nodal_force(
            "fixture-node-y-positive",
            PrimitiveLoadCategory::Occasional,
            1,
            LoadDirection::GlobalY,
            LoadQuantity::new(8.0, LoadDimension::Force).expect("fixture force is finite"),
        ),
        PrimitiveLoad::nodal_force(
            "fixture-node-y-negative",
            PrimitiveLoadCategory::Occasional,
            1,
            LoadDirection::GlobalY,
            LoadQuantity::new(-3.0, LoadDimension::Force).expect("fixture force is finite"),
        ),
        PrimitiveLoad::uniform_element_load(
            "fixture-weight-uniform",
            PrimitiveLoadCategory::Weight,
            0,
            LoadDirection::GlobalZ,
            LoadQuantity::new(1.25, LoadDimension::ForcePerLength).expect("fixture load is finite"),
        ),
        PrimitiveLoad::imposed_displacement(
            "fixture-imposed-uz",
            2,
            FrameDof::Uz,
            LoadQuantity::new(-0.02, LoadDimension::Displacement)
                .expect("fixture displacement is finite"),
        ),
    ];
    let application = prepare_loads(3, 1, &loads);
    let vector = application.global_load_vector(3);
    let fixture = primitive_load_preparation_fixture();
    !application.is_blocked()
        && application.nodal_loads.len() == 2
        && application.element_uniform_loads.len() == 1
        && application.imposed_displacements.len() == 1
        && (vector[DOF_PER_NODE + UY] - fixture.expected_values[0].value).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (application.element_uniform_loads[0].magnitude.value - fixture.expected_values[1].value)
            .abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (application.imposed_displacements[0].value.value - fixture.expected_values[2].value)
            .abs()
            <= INTERNAL_ASSERTION_EPSILON
}

pub fn solve_tp_phys_002_linear_static_integration() -> Result<LinearStaticIntegrationResult, String>
{
    let (pipe, stiffness, force, lumped_force) = tp_phys_002_model()?;
    let supports = tp_phys_002_supports();
    let support_application =
        apply_linear_supports(&stiffness, &force, &supports).map_err(|error| error.to_string())?;
    let reduced_displacements = solve_dense(
        &support_application.reduced_system.stiffness,
        &support_application.reduced_system.force,
    )
    .map_err(|error| error.to_string())?;

    let mut global_displacements = vec![0.0; 2 * DOF_PER_NODE];
    for (&dof, &value) in support_application
        .prescribed_dofs
        .iter()
        .zip(support_application.prescribed_displacements.iter())
    {
        global_displacements[dof] = value;
    }
    for (&dof, &value) in support_application
        .reduced_system
        .free_dofs
        .iter()
        .zip(reduced_displacements.iter())
    {
        global_displacements[dof] = value;
    }

    let recovered = pipe
        .recover_local_forces_from_global_model(&global_displacements)
        .map_err(|error| error.to_string())?;

    Ok(LinearStaticIntegrationResult {
        node_1_ux_displacement: global_displacements[DOF_PER_NODE + UX],
        node_1_uy_displacement: global_displacements[DOF_PER_NODE + UY],
        node_1_uz_displacement: global_displacements[DOF_PER_NODE + UZ],
        lumped_node_0_uy_force: lumped_force[UY],
        lumped_node_1_uy_force: lumped_force[DOF_PER_NODE + UY],
        recovered_local_axial_force_j: recovered.local_forces[DOF_PER_NODE + UX],
        recovered_local_shear_y_j: recovered.local_forces[DOF_PER_NODE + UY],
    })
}

pub fn validate_tp_phys_002_linear_static_integration() -> bool {
    let Ok(result) = solve_tp_phys_002_linear_static_integration() else {
        return false;
    };

    [
        result.node_1_ux_displacement,
        result.node_1_uy_displacement,
        result.node_1_uz_displacement,
        result.lumped_node_0_uy_force,
        result.lumped_node_1_uy_force,
        result.recovered_local_axial_force_j,
        result.recovered_local_shear_y_j,
    ]
    .into_iter()
    .all(f64::is_finite)
        && (result.lumped_node_0_uy_force + 4.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.lumped_node_1_uy_force + 4.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.node_1_ux_displacement - 0.016).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.node_1_uz_displacement + 0.01).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.recovered_local_axial_force_j - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && result.recovered_local_shear_y_j.abs() > INTERNAL_ASSERTION_EPSILON
}

pub fn validate_tp_phys_002_diagnostic_mapping() -> bool {
    let Ok((_pipe, stiffness, force, _lumped_force)) = tp_phys_002_model() else {
        return false;
    };

    let bad_support = LinearSupport::spring("tp-phys-002-missing-spring", 1, FrameDof::Uy, None);
    let Err(support_error) = apply_linear_supports(&stiffness, &force, &[bad_support]) else {
        return false;
    };
    let support_diagnostics = diagnostics_from_support_application_error(&support_error);
    let support_diagnostic_ok = support_diagnostics.iter().any(|diagnostic| {
        diagnostic.code == SolverDiagnosticCode::InvalidRestraint
            && diagnostic.severity == DiagnosticSeverity::Blocking
            && diagnostic.affected_ref.as_deref() == Some("tp-phys-002-missing-spring")
    });

    let missing_target_load = PrimitiveLoad::missing_target(
        "tp-phys-002-missing-load-target",
        PrimitiveLoadCategory::Weight,
        LoadDirection::GlobalY,
        Some(LoadQuantity::new(-2.0, LoadDimension::ForcePerLength).expect("finite load")),
    );
    let span = ElementLoadSpan::new(0, 0, 1, 4.0).expect("fixture span is positive");
    let load_application = prepare_lumped_nodal_loads(2, 1, &[span], &[missing_target_load]);
    let Some(load_finding) = load_application.findings.first() else {
        return false;
    };
    let load_diagnostic = diagnostic_from_primitive_load_finding(load_finding);
    let load_diagnostic_ok = load_diagnostic.code == SolverDiagnosticCode::InvalidModelTopology
        && load_diagnostic.severity == DiagnosticSeverity::Blocking
        && load_diagnostic.affected_ref.as_deref() == Some("tp-phys-002-missing-load-target");

    support_diagnostic_ok && load_diagnostic_ok
}

pub fn solve_tp_phys_004_load_to_resultant_integration(
) -> Result<LoadToResultantIntegrationResult, String> {
    let pipe = tp_phys_004_pipe()?;
    let frame_element = pipe.frame_element().map_err(|error| error.to_string())?;
    let stiffness =
        assemble_global_stiffness(2, &[frame_element]).map_err(|error| error.to_string())?;
    let loads = vec![
        UserLoad::uniform_distributed(
            "tp-phys-004-line-y",
            0,
            UserLoadDirection::GlobalY,
            UserLoadQuantity::new(-2.0, LoadDimension::ForcePerLength)
                .map_err(|error| error.to_string())?,
            UserElementLoadSpan::full(),
            Some(4.0),
        ),
        UserLoad::element_concentrated_force(
            "tp-phys-004-point-y",
            0,
            0.5,
            UserLoadDirection::GlobalY,
            UserLoadQuantity::new(-4.0, LoadDimension::Force).map_err(|error| error.to_string())?,
        ),
    ];
    let user_application = apply_straight_pipe_equivalent_user_loads(0, &pipe, &loads);
    if user_application.is_blocked() {
        return Err("TP-PHYS-004 user-load recovery produced findings".to_string());
    }
    let solver_contributions = user_application
        .nodal_loads
        .iter()
        .map(|load| {
            SolverNodalLoadContribution::new(
                load.trace.load_id.clone(),
                load.node_index,
                load.global_dof,
                load.value,
            )
        })
        .collect::<Vec<_>>();
    let assembly = assemble_solver_load_vector(2, &solver_contributions);
    if assembly.is_blocked() {
        return Err("TP-PHYS-004 load-case assembly produced findings".to_string());
    }

    let support_application = apply_linear_supports(
        &stiffness,
        &assembly.global_load_vector,
        &[LinearSupport::anchor("tp-phys-004-node-0-anchor", 0)],
    )
    .map_err(|error| error.to_string())?;
    let reduced_displacements = solve_dense(
        &support_application.reduced_system.stiffness,
        &support_application.reduced_system.force,
    )
    .map_err(|error| error.to_string())?;

    let mut global_displacements = vec![0.0; 2 * DOF_PER_NODE];
    for (&dof, &value) in support_application
        .prescribed_dofs
        .iter()
        .zip(support_application.prescribed_displacements.iter())
    {
        global_displacements[dof] = value;
    }
    for (&dof, &value) in support_application
        .reduced_system
        .free_dofs
        .iter()
        .zip(reduced_displacements.iter())
    {
        global_displacements[dof] = value;
    }

    let station = pipe
        .recover_station_resultants_from_global_model(
            &global_displacements,
            0.5,
            &[UniformLocalLoad::new(LocalLoadDirection::Y, -2.0)
                .map_err(|error| error.to_string())?],
            &[PointLocalForce::new(0.5, LocalLoadDirection::Y, -4.0)
                .map_err(|error| error.to_string())?],
        )
        .map_err(|error| error.to_string())?;

    Ok(LoadToResultantIntegrationResult {
        node_1_uy_displacement: global_displacements[DOF_PER_NODE + UY],
        node_1_rz_rotation: global_displacements[DOF_PER_NODE + RZ],
        assembled_node_0_uy_force: assembly.global_load_vector[UY],
        assembled_node_0_rz_moment: assembly.global_load_vector[RZ],
        midspan_shear_y: station.shear_force_y,
        midspan_bending_z: station.bending_moment_z,
    })
}

pub fn validate_tp_phys_004_load_to_resultant_integration() -> bool {
    let Ok(result) = solve_tp_phys_004_load_to_resultant_integration() else {
        return false;
    };

    [
        result.node_1_uy_displacement,
        result.node_1_rz_rotation,
        result.assembled_node_0_uy_force,
        result.assembled_node_0_rz_moment,
        result.midspan_shear_y,
        result.midspan_bending_z,
    ]
    .into_iter()
    .all(f64::is_finite)
        && (result.node_1_uy_displacement + 0.04533333333333334).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.node_1_rz_rotation + 0.014666666666666668).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_0_uy_force + 6.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_0_rz_moment + 4.666666666666667).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_shear_y - 4.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_bending_z - 4.0).abs() <= INTERNAL_ASSERTION_EPSILON
}

pub fn solve_tp_phys_005_oriented_load_to_resultant_integration(
) -> Result<OrientedLoadToResultantIntegrationResult, String> {
    let pipe = tp_phys_005_pipe()?;
    let frame_element = pipe.frame_element().map_err(|error| error.to_string())?;
    let stiffness =
        assemble_global_stiffness(2, &[frame_element]).map_err(|error| error.to_string())?;
    let loads = vec![
        UserLoad::uniform_distributed(
            "tp-phys-005-line-global-x",
            0,
            UserLoadDirection::GlobalX,
            UserLoadQuantity::new(-2.0, LoadDimension::ForcePerLength)
                .map_err(|error| error.to_string())?,
            UserElementLoadSpan::full(),
            Some(4.0),
        ),
        UserLoad::element_concentrated_force(
            "tp-phys-005-point-global-x",
            0,
            0.5,
            UserLoadDirection::GlobalX,
            UserLoadQuantity::new(-4.0, LoadDimension::Force).map_err(|error| error.to_string())?,
        ),
    ];
    let user_application = apply_straight_pipe_equivalent_user_loads(0, &pipe, &loads);
    if user_application.is_blocked() {
        return Err("TP-PHYS-005 user-load recovery produced findings".to_string());
    }
    let solver_contributions = user_application
        .nodal_loads
        .iter()
        .map(|load| {
            SolverNodalLoadContribution::new(
                load.trace.load_id.clone(),
                load.node_index,
                load.global_dof,
                load.value,
            )
        })
        .collect::<Vec<_>>();
    let assembly = assemble_solver_load_vector(2, &solver_contributions);
    if assembly.is_blocked() {
        return Err("TP-PHYS-005 load-case assembly produced findings".to_string());
    }

    let support_application = apply_linear_supports(
        &stiffness,
        &assembly.global_load_vector,
        &[LinearSupport::anchor("tp-phys-005-node-0-anchor", 0)],
    )
    .map_err(|error| error.to_string())?;
    let reduced_displacements = solve_dense(
        &support_application.reduced_system.stiffness,
        &support_application.reduced_system.force,
    )
    .map_err(|error| error.to_string())?;

    let mut global_displacements = vec![0.0; 2 * DOF_PER_NODE];
    for (&dof, &value) in support_application
        .prescribed_dofs
        .iter()
        .zip(support_application.prescribed_displacements.iter())
    {
        global_displacements[dof] = value;
    }
    for (&dof, &value) in support_application
        .reduced_system
        .free_dofs
        .iter()
        .zip(reduced_displacements.iter())
    {
        global_displacements[dof] = value;
    }

    let station = pipe
        .recover_station_resultants_from_global_model(
            &global_displacements,
            0.5,
            &[UniformLocalLoad::new(LocalLoadDirection::Y, -2.0)
                .map_err(|error| error.to_string())?],
            &[PointLocalForce::new(0.5, LocalLoadDirection::Y, -4.0)
                .map_err(|error| error.to_string())?],
        )
        .map_err(|error| error.to_string())?;

    Ok(OrientedLoadToResultantIntegrationResult {
        node_1_ux_displacement: global_displacements[DOF_PER_NODE + UX],
        node_1_rz_rotation: global_displacements[DOF_PER_NODE + RZ],
        assembled_node_0_ux_force: assembly.global_load_vector[UX],
        assembled_node_0_rz_moment: assembly.global_load_vector[RZ],
        midspan_shear_y: station.shear_force_y,
        midspan_bending_z: station.bending_moment_z,
    })
}

pub fn validate_tp_phys_005_oriented_load_to_resultant_integration() -> bool {
    let Ok(result) = solve_tp_phys_005_oriented_load_to_resultant_integration() else {
        return false;
    };

    [
        result.node_1_ux_displacement,
        result.node_1_rz_rotation,
        result.assembled_node_0_ux_force,
        result.assembled_node_0_rz_moment,
        result.midspan_shear_y,
        result.midspan_bending_z,
    ]
    .into_iter()
    .all(f64::is_finite)
        && (result.node_1_ux_displacement + 0.04533333333333334).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.node_1_rz_rotation - 0.014666666666666668).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_0_ux_force + 6.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_0_rz_moment - 4.666666666666667).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_shear_y - 4.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_bending_z - 4.0).abs() <= INTERNAL_ASSERTION_EPSILON
}

pub fn solve_tp_phys_006_partial_span_load_to_resultant_integration(
) -> Result<PartialSpanLoadToResultantIntegrationResult, String> {
    let pipe = tp_phys_006_pipe()?;
    let frame_element = pipe.frame_element().map_err(|error| error.to_string())?;
    let stiffness =
        assemble_global_stiffness(2, &[frame_element]).map_err(|error| error.to_string())?;
    let loads = vec![UserLoad::uniform_distributed(
        "tp-phys-006-partial-line-y",
        0,
        UserLoadDirection::GlobalY,
        UserLoadQuantity::new(-2.0, LoadDimension::ForcePerLength)
            .map_err(|error| error.to_string())?,
        UserElementLoadSpan::new(0.25, 0.75).map_err(|error| error.to_string())?,
        Some(4.0),
    )];
    let user_application = apply_straight_pipe_equivalent_user_loads(0, &pipe, &loads);
    if user_application.is_blocked() {
        return Err("TP-PHYS-006 user-load recovery produced findings".to_string());
    }
    let solver_contributions = user_application
        .nodal_loads
        .iter()
        .map(|load| {
            SolverNodalLoadContribution::new(
                load.trace.load_id.clone(),
                load.node_index,
                load.global_dof,
                load.value,
            )
        })
        .collect::<Vec<_>>();
    let assembly = assemble_solver_load_vector(2, &solver_contributions);
    if assembly.is_blocked() {
        return Err("TP-PHYS-006 load-case assembly produced findings".to_string());
    }

    let support_application = apply_linear_supports(
        &stiffness,
        &assembly.global_load_vector,
        &[LinearSupport::anchor("tp-phys-006-node-0-anchor", 0)],
    )
    .map_err(|error| error.to_string())?;
    let reduced_displacements = solve_dense(
        &support_application.reduced_system.stiffness,
        &support_application.reduced_system.force,
    )
    .map_err(|error| error.to_string())?;

    let mut global_displacements = vec![0.0; 2 * DOF_PER_NODE];
    for (&dof, &value) in support_application
        .prescribed_dofs
        .iter()
        .zip(support_application.prescribed_displacements.iter())
    {
        global_displacements[dof] = value;
    }
    for (&dof, &value) in support_application
        .reduced_system
        .free_dofs
        .iter()
        .zip(reduced_displacements.iter())
    {
        global_displacements[dof] = value;
    }

    let spanned_uniform = SpannedUniformLocalLoad::new(
        LocalLoadDirection::Y,
        -2.0,
        UniformLoadSpan::new(0.25, 0.75).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let station = pipe
        .recover_station_resultants_from_global_model_with_spans(
            &global_displacements,
            0.5,
            &[spanned_uniform],
            &[],
        )
        .map_err(|error| error.to_string())?;

    Ok(PartialSpanLoadToResultantIntegrationResult {
        node_1_uy_displacement: global_displacements[DOF_PER_NODE + UY],
        node_1_rz_rotation: global_displacements[DOF_PER_NODE + RZ],
        assembled_node_0_uy_force: assembly.global_load_vector[UY],
        assembled_node_0_rz_moment: assembly.global_load_vector[RZ],
        assembled_node_1_uy_force: assembly.global_load_vector[DOF_PER_NODE + UY],
        assembled_node_1_rz_moment: assembly.global_load_vector[DOF_PER_NODE + RZ],
        midspan_shear_y: station.shear_force_y,
        midspan_bending_z: station.bending_moment_z,
    })
}

pub fn validate_tp_phys_006_partial_span_load_to_resultant_integration() -> bool {
    let Ok(result) = solve_tp_phys_006_partial_span_load_to_resultant_integration() else {
        return false;
    };

    [
        result.node_1_uy_displacement,
        result.node_1_rz_rotation,
        result.assembled_node_0_uy_force,
        result.assembled_node_0_rz_moment,
        result.assembled_node_1_uy_force,
        result.assembled_node_1_rz_moment,
        result.midspan_shear_y,
        result.midspan_bending_z,
    ]
    .into_iter()
    .all(f64::is_finite)
        && (result.assembled_node_0_uy_force + 2.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_0_rz_moment + 11.0 / 6.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_1_uy_force + 2.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_1_rz_moment - 11.0 / 6.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.node_1_uy_displacement + 7.0 / 500.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.node_1_rz_rotation + 13.0 / 3000.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_shear_y - 2.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_bending_z - 1.0).abs() <= INTERNAL_ASSERTION_EPSILON
}

pub fn solve_tp_phys_007_station_sweep_resultants_integration(
) -> Result<StationSweepResultantIntegrationResult, String> {
    let pipe = tp_phys_007_pipe()?;
    let frame_element = pipe.frame_element().map_err(|error| error.to_string())?;
    let stiffness =
        assemble_global_stiffness(2, &[frame_element]).map_err(|error| error.to_string())?;
    let loads = vec![UserLoad::uniform_distributed(
        "tp-phys-007-partial-line-y",
        0,
        UserLoadDirection::GlobalY,
        UserLoadQuantity::new(-2.0, LoadDimension::ForcePerLength)
            .map_err(|error| error.to_string())?,
        UserElementLoadSpan::new(0.25, 0.75).map_err(|error| error.to_string())?,
        Some(4.0),
    )];
    let user_application = apply_straight_pipe_equivalent_user_loads(0, &pipe, &loads);
    if user_application.is_blocked() {
        return Err("TP-PHYS-007 user-load recovery produced findings".to_string());
    }
    let solver_contributions = user_application
        .nodal_loads
        .iter()
        .map(|load| {
            SolverNodalLoadContribution::new(
                load.trace.load_id.clone(),
                load.node_index,
                load.global_dof,
                load.value,
            )
        })
        .collect::<Vec<_>>();
    let assembly = assemble_solver_load_vector(2, &solver_contributions);
    if assembly.is_blocked() {
        return Err("TP-PHYS-007 load-case assembly produced findings".to_string());
    }

    let support_application = apply_linear_supports(
        &stiffness,
        &assembly.global_load_vector,
        &[LinearSupport::anchor("tp-phys-007-node-0-anchor", 0)],
    )
    .map_err(|error| error.to_string())?;
    let reduced_displacements = solve_dense(
        &support_application.reduced_system.stiffness,
        &support_application.reduced_system.force,
    )
    .map_err(|error| error.to_string())?;

    let mut global_displacements = vec![0.0; 2 * DOF_PER_NODE];
    for (&dof, &value) in support_application
        .prescribed_dofs
        .iter()
        .zip(support_application.prescribed_displacements.iter())
    {
        global_displacements[dof] = value;
    }
    for (&dof, &value) in support_application
        .reduced_system
        .free_dofs
        .iter()
        .zip(reduced_displacements.iter())
    {
        global_displacements[dof] = value;
    }

    let spanned_uniform = SpannedUniformLocalLoad::new(
        LocalLoadDirection::Y,
        -2.0,
        UniformLoadSpan::new(0.25, 0.75).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let sweep = pipe
        .recover_station_resultant_sweep_from_global_model_with_spans(
            &global_displacements,
            &[0.75, 0.25, 0.5, 1.0],
            &[spanned_uniform],
            &[],
        )
        .map_err(|error| error.to_string())?;

    Ok(StationSweepResultantIntegrationResult {
        station_resultants: sweep
            .into_iter()
            .map(|station| StationSweepResultant {
                station_fraction: station.station_fraction,
                shear_y: station.shear_force_y,
                bending_z: station.bending_moment_z,
            })
            .collect(),
    })
}

pub fn validate_tp_phys_007_station_sweep_resultants_integration() -> bool {
    let Ok(result) = solve_tp_phys_007_station_sweep_resultants_integration() else {
        return false;
    };
    let expected = expected_tp_phys_007_station_sweep();

    result.station_resultants.len() == expected.len()
        && result
            .station_resultants
            .iter()
            .zip(expected.iter())
            .all(|(actual, expected)| {
                actual.station_fraction.is_finite()
                    && actual.shear_y.is_finite()
                    && actual.bending_z.is_finite()
                    && (actual.station_fraction - expected.station_fraction).abs()
                        <= INTERNAL_ASSERTION_EPSILON
                    && (actual.shear_y - expected.shear_y).abs() <= INTERNAL_ASSERTION_EPSILON
                    && (actual.bending_z - expected.bending_z).abs() <= INTERNAL_ASSERTION_EPSILON
            })
}

pub fn solve_tp_phys_008_thermal_pressure_axial_effects(
) -> Result<ThermalPressureAxialEffectsResult, String> {
    let pipe = tp_phys_008_pipe()?;
    let loads = tp_phys_008_axial_effect_loads();
    let properties = [tp_phys_008_axial_effect_properties()];
    let prepared = prepare_straight_pipe_axial_effects(1, &loads, &properties);
    if prepared.is_blocked() {
        return Err("TP-PHYS-008 axial-effect preparation produced findings".to_string());
    }

    let thermal_axial_force =
        axial_force_by_load_id(&prepared.axial_effects, "tp-phys-008-thermal-restraint")?;
    let pressure_thrust_force =
        axial_force_by_load_id(&prepared.axial_effects, "tp-phys-008-pressure-thrust")?;
    let total_axial_effect_force = thermal_axial_force + pressure_thrust_force;
    let axial_effects = prepared
        .axial_effects
        .iter()
        .map(|contribution| {
            StraightPipeAxialEffect::new(contribution.axial_force)
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let equivalent_global = pipe
        .equivalent_global_axial_effect_loads(&axial_effects)
        .map_err(|error| error.to_string())?;

    let zero_element_displacements = [0.0; 2 * DOF_PER_NODE];
    let recovered = pipe
        .recover_local_forces_with_axial_effects(&zero_element_displacements, &axial_effects)
        .map_err(|error| error.to_string())?;
    let end_i = pipe
        .recover_end_resultants_with_axial_effects(
            &zero_element_displacements,
            PipeEnd::I,
            &axial_effects,
        )
        .map_err(|error| error.to_string())?;
    let end_j = pipe
        .recover_end_resultants_with_axial_effects(
            &zero_element_displacements,
            PipeEnd::J,
            &axial_effects,
        )
        .map_err(|error| error.to_string())?;

    let zero_global_model_displacements = vec![0.0; 2 * DOF_PER_NODE];
    let midspan = pipe
        .recover_station_resultants_from_global_model_with_axial_effects(
            &zero_global_model_displacements,
            0.5,
            &axial_effects,
        )
        .map_err(|error| error.to_string())?;
    let sweep = pipe
        .recover_station_resultant_sweep_from_global_model_with_axial_effects(
            &zero_global_model_displacements,
            &[1.0, 0.0, 0.5],
            &axial_effects,
        )
        .map_err(|error| error.to_string())?;

    Ok(ThermalPressureAxialEffectsResult {
        thermal_axial_force,
        pressure_thrust_force,
        total_axial_effect_force,
        equivalent_node_i_axial_load: equivalent_global[UX],
        equivalent_node_j_axial_load: equivalent_global[DOF_PER_NODE + UX],
        recovered_local_i_axial_force: recovered.local_forces[UX],
        recovered_local_j_axial_force: recovered.local_forces[DOF_PER_NODE + UX],
        end_i_axial_force: end_i.axial_force,
        end_j_axial_force: end_j.axial_force,
        midspan_axial_force: midspan.axial_force,
        station_sweep: sweep
            .into_iter()
            .map(|station| AxialEffectStationResultant {
                station_fraction: station.station_fraction,
                axial_force: station.axial_force,
                shear_y: station.shear_force_y,
                bending_z: station.bending_moment_z,
            })
            .collect(),
    })
}

pub fn solve_tp_phys_009_combined_load_axial_effects(
) -> Result<CombinedLoadAxialEffectsResult, String> {
    let pipe = tp_phys_009_pipe()?;
    let frame_element = pipe.frame_element().map_err(|error| error.to_string())?;
    let stiffness =
        assemble_global_stiffness(2, &[frame_element]).map_err(|error| error.to_string())?;

    let primitive_loads = tp_phys_009_axial_effect_loads();
    let properties = [tp_phys_009_axial_effect_properties()];
    let prepared_axial = prepare_straight_pipe_axial_effects(1, &primitive_loads, &properties);
    if prepared_axial.is_blocked() {
        return Err("TP-PHYS-009 axial-effect preparation produced findings".to_string());
    }

    let thermal_axial_force = axial_force_by_load_id(
        &prepared_axial.axial_effects,
        "tp-phys-009-thermal-restraint",
    )?;
    let pressure_thrust_force =
        axial_force_by_load_id(&prepared_axial.axial_effects, "tp-phys-009-pressure-thrust")?;
    let total_axial_effect_force = thermal_axial_force + pressure_thrust_force;
    let axial_effects = prepared_axial
        .axial_effects
        .iter()
        .map(|contribution| {
            StraightPipeAxialEffect::new(contribution.axial_force)
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let user_loads = vec![UserLoad::uniform_distributed(
        "tp-phys-009-partial-line-y",
        0,
        UserLoadDirection::GlobalY,
        UserLoadQuantity::new(-2.0, LoadDimension::ForcePerLength)
            .map_err(|error| error.to_string())?,
        UserElementLoadSpan::new(0.25, 0.75).map_err(|error| error.to_string())?,
        Some(6.0),
    )];
    let user_application = apply_straight_pipe_equivalent_user_loads_with_axial_effects(
        0,
        &pipe,
        &user_loads,
        &prepared_axial.axial_effects,
    );
    if user_application.is_blocked() {
        return Err("TP-PHYS-009 combined user-load application produced findings".to_string());
    }

    let distributed_recovery_hook_count = user_application
        .recovery_hooks
        .iter()
        .filter(|hook| hook.hook_kind == RecoveryHookKind::ElementDistributedLoad)
        .count();
    let axial_effect_recovery_hook_count = user_application
        .recovery_hooks
        .iter()
        .filter(|hook| hook.hook_kind == RecoveryHookKind::ElementAxialEffect)
        .count();

    let solver_contributions = user_application
        .nodal_loads
        .iter()
        .map(|load| {
            SolverNodalLoadContribution::new(
                load.trace.load_id.clone(),
                load.node_index,
                load.global_dof,
                load.value,
            )
        })
        .collect::<Vec<_>>();
    let assembly = assemble_solver_load_vector(2, &solver_contributions);
    if assembly.is_blocked() {
        return Err("TP-PHYS-009 load-case assembly produced findings".to_string());
    }

    let support_application = apply_linear_supports(
        &stiffness,
        &assembly.global_load_vector,
        &[
            LinearSupport::anchor("tp-phys-009-node-0-anchor", 0),
            LinearSupport::line_stop("tp-phys-009-node-1-axial-stop", 1, FrameDof::Ux),
        ],
    )
    .map_err(|error| error.to_string())?;
    let reduced_displacements = solve_dense(
        &support_application.reduced_system.stiffness,
        &support_application.reduced_system.force,
    )
    .map_err(|error| error.to_string())?;

    let mut global_displacements = vec![0.0; 2 * DOF_PER_NODE];
    for (&dof, &value) in support_application
        .prescribed_dofs
        .iter()
        .zip(support_application.prescribed_displacements.iter())
    {
        global_displacements[dof] = value;
    }
    for (&dof, &value) in support_application
        .reduced_system
        .free_dofs
        .iter()
        .zip(reduced_displacements.iter())
    {
        global_displacements[dof] = value;
    }

    let spanned_uniform = SpannedUniformLocalLoad::new(
        LocalLoadDirection::Y,
        -2.0,
        UniformLoadSpan::new(0.25, 0.75).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let end_i = pipe
        .recover_end_resultants_from_global_model_with_spans_and_axial_effects(
            &global_displacements,
            PipeEnd::I,
            &[spanned_uniform],
            &[],
            &axial_effects,
        )
        .map_err(|error| error.to_string())?;
    let midspan = pipe
        .recover_station_resultants_from_global_model_with_spans_and_axial_effects(
            &global_displacements,
            0.5,
            &[spanned_uniform],
            &[],
            &axial_effects,
        )
        .map_err(|error| error.to_string())?;
    let sweep = pipe
        .recover_station_resultant_sweep_from_global_model_with_spans_and_axial_effects(
            &global_displacements,
            &[0.25, 0.5, 0.75, 1.0],
            &[spanned_uniform],
            &[],
            &axial_effects,
        )
        .map_err(|error| error.to_string())?;

    Ok(CombinedLoadAxialEffectsResult {
        thermal_axial_force,
        pressure_thrust_force,
        total_axial_effect_force,
        assembled_node_0_ux_force: assembly.global_load_vector[UX],
        assembled_node_0_uy_force: assembly.global_load_vector[UY],
        assembled_node_0_rz_moment: assembly.global_load_vector[RZ],
        assembled_node_1_ux_force: assembly.global_load_vector[DOF_PER_NODE + UX],
        assembled_node_1_uy_force: assembly.global_load_vector[DOF_PER_NODE + UY],
        assembled_node_1_rz_moment: assembly.global_load_vector[DOF_PER_NODE + RZ],
        node_1_ux_displacement: global_displacements[DOF_PER_NODE + UX],
        node_1_uy_displacement: global_displacements[DOF_PER_NODE + UY],
        node_1_rz_rotation: global_displacements[DOF_PER_NODE + RZ],
        end_i_axial_force: end_i.axial_force,
        end_i_shear_y: end_i.shear_force_y,
        end_i_bending_z: end_i.bending_moment_z,
        midspan_axial_force: midspan.axial_force,
        midspan_shear_y: midspan.shear_force_y,
        midspan_bending_z: midspan.bending_moment_z,
        station_sweep: sweep
            .into_iter()
            .map(|station| AxialEffectStationResultant {
                station_fraction: station.station_fraction,
                axial_force: station.axial_force,
                shear_y: station.shear_force_y,
                bending_z: station.bending_moment_z,
            })
            .collect(),
        distributed_recovery_hook_count,
        axial_effect_recovery_hook_count,
    })
}

pub fn solve_tp_phys_014_canonical_analytical_payload(
) -> Result<CanonicalAnalyticalPayloadSolverResult, String> {
    let root: Value = serde_json::from_str(TP_PHYS_014_ANALYTICAL_PAYLOAD)
        .map_err(|error| format!("TP-PHYS-014 payload JSON parse failed: {error}"))?;
    let model = object_field(object_value(&root, "root")?, "model", "root")?;
    require_string_field(model, "model_role", "root.model").and_then(|role| {
        (role == "analytical_solver_model")
            .then_some(role)
            .ok_or_else(|| {
                "TP-PHYS-014 payload model_role must be analytical_solver_model".to_string()
            })
    })?;

    let (nodes, node_index_by_id) = tp_phys_014_nodes(model)?;
    let element = only_record(
        array_field(model, "elements", "payload.model")?,
        "payload.model.elements",
    )?;
    let element_id = require_string_field(element, "id", "payload.model.elements[0]")?;
    require_string_field(element, "element_type", "payload.model.elements[0]").and_then(
        |kind| {
            (kind == "straight_pipe")
                .then_some(kind)
                .ok_or_else(|| "TP-PHYS-014 element_type must be straight_pipe".to_string())
        },
    )?;
    let start_node_id = reference_id(
        object_field(element, "start_node_ref", "payload.model.elements[0]")?,
        "Node",
        "payload.model.elements[0].start_node_ref",
    )?;
    let end_node_id = reference_id(
        object_field(element, "end_node_ref", "payload.model.elements[0]")?,
        "Node",
        "payload.model.elements[0].end_node_ref",
    )?;
    let material_id = reference_id(
        object_field(element, "material_ref", "payload.model.elements[0]")?,
        "Material",
        "payload.model.elements[0].material_ref",
    )?;
    let section_id = reference_id(
        object_field(element, "section_ref", "payload.model.elements[0]")?,
        "Section",
        "payload.model.elements[0].section_ref",
    )?;
    let local_coordinate_system = object_field(
        element,
        "local_coordinate_system",
        "payload.model.elements[0]",
    )?;
    let y_reference = direction_vector_field(
        local_coordinate_system,
        "y_reference",
        "payload.model.elements[0].local_coordinate_system",
    )?;

    let materials = array_field(model, "materials", "payload.model")?;
    let material = record_by_id(materials, material_id, "payload.model.materials")?;
    let sections = array_field(model, "sections", "payload.model")?;
    let section = record_by_id(sections, section_id, "payload.model.sections")?;
    let section_properties = tp_phys_014_section_properties(material, section)?;
    let pipe = StraightPipeElement::new(
        element_id,
        nodes
            .get(start_node_id)
            .ok_or_else(|| format!("missing start node {start_node_id}"))?
            .clone(),
        nodes
            .get(end_node_id)
            .ok_or_else(|| format!("missing end node {end_node_id}"))?
            .clone(),
        section_properties,
        y_reference,
    )
    .map_err(|error| error.to_string())?;
    let pipe_length = pipe.length().map_err(|error| error.to_string())?;

    let support_node_index = tp_phys_014_anchor_node_index(model, &node_index_by_id)?;
    let (
        user_loads,
        local_uniforms,
        local_points,
        distributed_load_count,
        point_force_count,
        trace_anchors,
    ) = tp_phys_014_loads(model, element_id, pipe_length)?;
    if distributed_load_count != 1 || point_force_count != 1 {
        return Err(format!(
            "TP-PHYS-014 requires one distributed force and one point force, got {distributed_load_count} and {point_force_count}"
        ));
    }

    let frame_element = pipe.frame_element().map_err(|error| error.to_string())?;
    let stiffness = assemble_global_stiffness(nodes.len(), &[frame_element])
        .map_err(|error| error.to_string())?;
    let user_application = apply_straight_pipe_equivalent_user_loads(0, &pipe, &user_loads);
    if user_application.is_blocked() {
        return Err(format!(
            "TP-PHYS-014 user-load application findings: {:?}",
            user_application.findings
        ));
    }

    let distributed_recovery_hook_count = user_application
        .recovery_hooks
        .iter()
        .filter(|hook| hook.hook_kind == RecoveryHookKind::ElementDistributedLoad)
        .count();
    let point_force_recovery_hook_count = user_application
        .recovery_hooks
        .iter()
        .filter(|hook| hook.hook_kind == RecoveryHookKind::NodalForce)
        .count();
    let solver_contributions = user_application
        .nodal_loads
        .iter()
        .map(|load| {
            SolverNodalLoadContribution::new(
                load.trace.load_id.clone(),
                load.node_index,
                load.global_dof,
                load.value,
            )
        })
        .collect::<Vec<_>>();
    let assembly = assemble_solver_load_vector(nodes.len(), &solver_contributions);
    if assembly.is_blocked() {
        return Err("TP-PHYS-014 load-vector assembly produced findings".to_string());
    }
    let node_id_by_index = tp_phys_014_node_id_by_index(&node_index_by_id, nodes.len())?;
    let load_vector_trace_chains = tp_phys_014_load_vector_trace_chains(
        &user_application.nodal_loads,
        &trace_anchors,
        &node_id_by_index,
    )?;

    let support_application = apply_linear_supports(
        &stiffness,
        &assembly.global_load_vector,
        &[LinearSupport::anchor(
            "tp-phys-014-payload-anchor",
            support_node_index,
        )],
    )
    .map_err(|error| error.to_string())?;
    let reduced_displacements = solve_dense(
        &support_application.reduced_system.stiffness,
        &support_application.reduced_system.force,
    )
    .map_err(|error| error.to_string())?;

    let mut global_displacements = vec![0.0; nodes.len() * DOF_PER_NODE];
    for (&dof, &value) in support_application
        .prescribed_dofs
        .iter()
        .zip(support_application.prescribed_displacements.iter())
    {
        global_displacements[dof] = value;
    }
    for (&dof, &value) in support_application
        .reduced_system
        .free_dofs
        .iter()
        .zip(reduced_displacements.iter())
    {
        global_displacements[dof] = value;
    }
    let residual_loads = stiffness
        .iter()
        .map(|row| {
            row.iter()
                .zip(global_displacements.iter())
                .map(|(stiffness_value, displacement)| stiffness_value * displacement)
                .sum::<f64>()
        })
        .zip(assembly.global_load_vector.iter())
        .map(|(internal_force, applied_force)| internal_force - applied_force)
        .collect::<Vec<_>>();

    let station = pipe
        .recover_station_resultants_from_global_model_with_spans(
            &global_displacements,
            0.5,
            &local_uniforms,
            &local_points,
        )
        .map_err(|error| error.to_string())?;

    Ok(CanonicalAnalyticalPayloadSolverResult {
        distributed_load_count,
        point_force_count,
        y_reference,
        assembled_node_0_uy_force: assembly.global_load_vector[UY],
        assembled_node_0_rz_moment: assembly.global_load_vector[RZ],
        assembled_node_1_uy_force: assembly.global_load_vector[DOF_PER_NODE + UY],
        assembled_node_1_rz_moment: assembly.global_load_vector[DOF_PER_NODE + RZ],
        support_reaction_node_0_uy_force: residual_loads[UY],
        support_reaction_node_0_rz_moment: residual_loads[RZ],
        node_1_uy_displacement: global_displacements[DOF_PER_NODE + UY],
        node_1_rz_rotation: global_displacements[DOF_PER_NODE + RZ],
        midspan_shear_y: station.shear_force_y,
        midspan_bending_z: station.bending_moment_z,
        distributed_recovery_hook_count,
        point_force_recovery_hook_count,
        load_vector_trace_chains,
    })
}

pub fn tp_phys_015a_canonical_solve_result_envelope(
) -> Result<CanonicalSolveResultEnvelopeEvidence, String> {
    let result = solve_tp_phys_014_canonical_analytical_payload()?;
    let provenance = tp_phys_015a_result_provenance();
    let load_case_ref = Reference::new("load_case", "LC-TP-PHYS-014");
    let model_ref = Reference::new("analytical_solver_model", "ANALYTICAL-TP-PHYS-014");
    let run_ref = Reference::new(
        "analysis_run",
        "RUN-TP-PHYS-015A-CANONICAL-SOLVE-RESULT-ENVELOPE",
    );
    let envelope_id = "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE";

    let values = vec![
        tp_phys_015a_quantity(
            "result:disp:node-N-2:uy",
            ResultFamily::Displacement,
            Reference::new("node", "N-2"),
            &load_case_ref,
            None,
            result.node_1_uy_displacement,
            "m",
            ExportDimensionId::Length,
            None,
        ),
        tp_phys_015a_quantity(
            "result:rotation:node-N-2:rz",
            ResultFamily::Rotation,
            Reference::new("node", "N-2"),
            &load_case_ref,
            None,
            result.node_1_rz_rotation,
            "rad",
            ExportDimensionId::Angle,
            None,
        ),
        tp_phys_015a_with_trace_chain(
            tp_phys_015a_quantity(
                "result:load-vector:node-N-1:uy",
                ResultFamily::Force,
                Reference::new("solver_load_vector", "node-N-1"),
                &load_case_ref,
                None,
                result.assembled_node_0_uy_force,
                "N",
                ExportDimensionId::Force,
                Some(tp_phys_015a_result_metadata(
                    "shear_force_y",
                    "global",
                    "end_i",
                    "explicit_user_linear_combination",
                    "positive follows the global UY solver degree of freedom",
                )),
            ),
            tp_phys_015a_runtime_trace_chain(&result, "result:load-vector:node-N-1:uy"),
        ),
        tp_phys_015a_with_trace_chain(
            tp_phys_015a_quantity(
                "result:load-vector:node-N-1:rz",
                ResultFamily::Moment,
                Reference::new("solver_load_vector", "node-N-1"),
                &load_case_ref,
                None,
                result.assembled_node_0_rz_moment,
                "N-m",
                ExportDimensionId::Moment,
                Some(tp_phys_015a_result_metadata(
                    "bending_moment_z",
                    "global",
                    "end_i",
                    "explicit_user_linear_combination",
                    "positive follows the global RZ solver degree of freedom",
                )),
            ),
            tp_phys_015a_runtime_trace_chain(&result, "result:load-vector:node-N-1:rz"),
        ),
        tp_phys_015a_with_trace_chain(
            tp_phys_015a_quantity(
                "result:load-vector:node-N-2:uy",
                ResultFamily::Force,
                Reference::new("solver_load_vector", "node-N-2"),
                &load_case_ref,
                None,
                result.assembled_node_1_uy_force,
                "N",
                ExportDimensionId::Force,
                Some(tp_phys_015a_result_metadata(
                    "shear_force_y",
                    "global",
                    "end_j",
                    "explicit_user_linear_combination",
                    "positive follows the global UY solver degree of freedom",
                )),
            ),
            tp_phys_015a_runtime_trace_chain(&result, "result:load-vector:node-N-2:uy"),
        ),
        tp_phys_015a_with_trace_chain(
            tp_phys_015a_quantity(
                "result:load-vector:node-N-2:rz",
                ResultFamily::Moment,
                Reference::new("solver_load_vector", "node-N-2"),
                &load_case_ref,
                None,
                result.assembled_node_1_rz_moment,
                "N-m",
                ExportDimensionId::Moment,
                Some(tp_phys_015a_result_metadata(
                    "bending_moment_z",
                    "global",
                    "end_j",
                    "explicit_user_linear_combination",
                    "positive follows the global RZ solver degree of freedom",
                )),
            ),
            tp_phys_015a_runtime_trace_chain(&result, "result:load-vector:node-N-2:rz"),
        ),
        tp_phys_015a_quantity(
            "result:reaction:support-N-1:uy",
            ResultFamily::Reaction,
            Reference::new("support", "N-1-anchor"),
            &load_case_ref,
            None,
            result.support_reaction_node_0_uy_force,
            "N",
            ExportDimensionId::Force,
            None,
        ),
        tp_phys_015a_quantity(
            "result:reaction:support-N-1:rz",
            ResultFamily::Reaction,
            Reference::new("support", "N-1-anchor"),
            &load_case_ref,
            None,
            result.support_reaction_node_0_rz_moment,
            "N-m",
            ExportDimensionId::Moment,
            None,
        ),
        tp_phys_015a_quantity(
            "result:force:element-E-1:midspan:shear-y",
            ResultFamily::Force,
            Reference::new("element", "E-1"),
            &load_case_ref,
            Some(Reference::new("result_station", "E-1:midspan")),
            result.midspan_shear_y,
            "N",
            ExportDimensionId::Force,
            Some(tp_phys_015a_result_metadata(
                "shear_force_y",
                "element_local",
                "midspan",
                "interpolated_from_endpoint_resultants",
                "positive follows the element-local y shear resultant convention",
            )),
        ),
        tp_phys_015a_quantity(
            "result:moment:element-E-1:midspan:bending-z",
            ResultFamily::Moment,
            Reference::new("element", "E-1"),
            &load_case_ref,
            Some(Reference::new("result_station", "E-1:midspan")),
            result.midspan_bending_z,
            "N-m",
            ExportDimensionId::Moment,
            Some(tp_phys_015a_result_metadata(
                "bending_moment_z",
                "element_local",
                "midspan",
                "interpolated_from_endpoint_resultants",
                "positive follows the element-local z bending resultant convention",
            )),
        ),
    ];
    let quantity_result_count = values.len();

    let envelope = ResultEnvelope {
        envelope_id: envelope_id.to_string(),
        schema_version: "0.1.0".to_string(),
        model_ref,
        run_ref,
        solver_name: "open_pipe_stress_validation_mechanics_benchmark".to_string(),
        solver_version: "0.1.0".to_string(),
        solver_build_ref: "validation-local-cargo-test".to_string(),
        unit_system_ref: Reference::new("unit_system", PKG09_FIXTURE_UNIT_SYSTEM_REF),
        load_basis_refs: vec![load_case_ref.clone()],
        result_sets: vec![ResultSet {
            set_id: "result-set:tp-phys-015a:canonical-mechanics".to_string(),
            set_type: ResultSetType::Mechanics.as_str().to_string(),
            basis_ref: load_case_ref,
            values,
        }],
        diagnostics: vec![ExportDiagnostic {
            code: "TP_PHYS_015A_RESULT_BOUNDARY_EVIDENCE".to_string(),
            class: ExportDiagnosticClass::AssumptionWarning,
            severity: ExportDiagnosticSeverity::Info,
            source: Reference::new("mechanics_benchmark", "DEL-09-01"),
            affected_object: Reference::new("result_envelope", envelope_id),
            message: "Validation-local result envelope records the canonical payload solve result without export/headless runtime expansion.".to_string(),
            remediation: "Treat result-export and headless-runner fit as adjacent deliverable checks; release tolerance policy remains TBD.".to_string(),
            provenance: provenance.clone(),
        }],
        provenance: provenance.clone(),
        reproducibility: ReproducibilityRefs {
            model_hash: tp_phys_015a_checksum(
                "analytical_solver_model:ANALYTICAL-TP-PHYS-014",
                TP_PHYS_014_ANALYTICAL_PAYLOAD.as_bytes(),
            ),
            run_hashes: vec![tp_phys_015a_checksum(
                "analysis_run:RUN-TP-PHYS-015A-CANONICAL-SOLVE-RESULT-ENVELOPE",
                format!("{result:?}").as_bytes(),
            )],
            audit_manifest_ref: Reference::new(
                "task_run_record",
                "TASK_RUN_2026-05-17_TP-PHYS-015A",
            ),
            deterministic_ordering: true,
        },
        analysis_status: vec![
            AnalysisStatus::MechanicsSolved,
            AnalysisStatus::HumanReviewRequired,
        ],
        rule_pack_refs: Vec::new(),
        professional_boundary: ProfessionalBoundary::project_default(),
    };

    let validation = validate_result_envelope(&envelope);
    Ok(CanonicalSolveResultEnvelopeEvidence {
        quantity_result_count,
        envelope_diagnostic_count: envelope.diagnostics.len(),
        export_validation_diagnostic_count: validation.diagnostics.len(),
        envelope,
    })
}

pub fn validate_tp_phys_015a_canonical_solve_result_envelope() -> bool {
    let Ok(evidence) = tp_phys_015a_canonical_solve_result_envelope() else {
        return false;
    };
    let result = match solve_tp_phys_014_canonical_analytical_payload() {
        Ok(result) => result,
        Err(_) => return false,
    };
    let sorted_result_ids = sorted_tp_phys_015a_result_ids(&evidence.envelope);

    evidence.export_validation_diagnostic_count == 0
        && evidence.envelope_diagnostic_count == 1
        && evidence.quantity_result_count == 10
        && evidence
            .envelope
            .analysis_status
            .contains(&AnalysisStatus::HumanReviewRequired)
        && evidence.envelope.professional_boundary == ProfessionalBoundary::project_default()
        && result.load_vector_trace_chains.len() == 4
        && tp_phys_015a_load_vector_trace_chains_are_runtime_evidence(&evidence.envelope)
        && sorted_result_ids
            == vec![
                "result:disp:node-N-2:uy",
                "result:rotation:node-N-2:rz",
                "result:force:element-E-1:midspan:shear-y",
                "result:load-vector:node-N-1:uy",
                "result:load-vector:node-N-2:uy",
                "result:moment:element-E-1:midspan:bending-z",
                "result:load-vector:node-N-1:rz",
                "result:load-vector:node-N-2:rz",
                "result:reaction:support-N-1:rz",
                "result:reaction:support-N-1:uy",
            ]
        && result.support_reaction_node_0_uy_force.is_finite()
        && result.support_reaction_node_0_rz_moment.is_finite()
        && (result.support_reaction_node_0_uy_force - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.support_reaction_node_0_rz_moment - 24.0).abs() <= INTERNAL_ASSERTION_EPSILON
}

pub fn validate_tp_phys_008_thermal_pressure_axial_effects() -> bool {
    let Ok(result) = solve_tp_phys_008_thermal_pressure_axial_effects() else {
        return false;
    };

    let expected_sweep = expected_tp_phys_008_axial_station_sweep();
    [
        result.thermal_axial_force,
        result.pressure_thrust_force,
        result.total_axial_effect_force,
        result.equivalent_node_i_axial_load,
        result.equivalent_node_j_axial_load,
        result.recovered_local_i_axial_force,
        result.recovered_local_j_axial_force,
        result.end_i_axial_force,
        result.end_j_axial_force,
        result.midspan_axial_force,
    ]
    .into_iter()
    .all(f64::is_finite)
        && (result.thermal_axial_force - 3.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.pressure_thrust_force - 9.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.total_axial_effect_force - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.equivalent_node_i_axial_load + 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.equivalent_node_j_axial_load - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.recovered_local_i_axial_force - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.recovered_local_j_axial_force + 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.end_i_axial_force - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.end_j_axial_force + 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_axial_force - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && result.station_sweep.len() == expected_sweep.len()
        && result
            .station_sweep
            .iter()
            .zip(expected_sweep.iter())
            .all(|(actual, expected)| {
                actual.station_fraction.is_finite()
                    && actual.axial_force.is_finite()
                    && actual.shear_y.is_finite()
                    && actual.bending_z.is_finite()
                    && (actual.station_fraction - expected.station_fraction).abs()
                        <= INTERNAL_ASSERTION_EPSILON
                    && (actual.axial_force - expected.axial_force).abs()
                        <= INTERNAL_ASSERTION_EPSILON
                    && (actual.shear_y - expected.shear_y).abs() <= INTERNAL_ASSERTION_EPSILON
                    && (actual.bending_z - expected.bending_z).abs() <= INTERNAL_ASSERTION_EPSILON
            })
}

pub fn validate_tp_phys_009_combined_load_axial_effects() -> bool {
    let Ok(result) = solve_tp_phys_009_combined_load_axial_effects() else {
        return false;
    };

    let expected_sweep = expected_tp_phys_009_combined_station_sweep();
    [
        result.thermal_axial_force,
        result.pressure_thrust_force,
        result.total_axial_effect_force,
        result.assembled_node_0_ux_force,
        result.assembled_node_0_uy_force,
        result.assembled_node_0_rz_moment,
        result.assembled_node_1_ux_force,
        result.assembled_node_1_uy_force,
        result.assembled_node_1_rz_moment,
        result.node_1_ux_displacement,
        result.node_1_uy_displacement,
        result.node_1_rz_rotation,
        result.end_i_axial_force,
        result.end_i_shear_y,
        result.end_i_bending_z,
        result.midspan_axial_force,
        result.midspan_shear_y,
        result.midspan_bending_z,
    ]
    .into_iter()
    .all(f64::is_finite)
        && (result.thermal_axial_force - 3.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.pressure_thrust_force - 9.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.total_axial_effect_force - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_0_ux_force + 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_0_uy_force + 3.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_0_rz_moment + 4.125).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_1_ux_force - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_1_uy_force + 3.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_1_rz_moment - 4.125).abs() <= INTERNAL_ASSERTION_EPSILON
        && result.node_1_ux_displacement.abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.node_1_uy_displacement + 0.070875).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.node_1_rz_rotation + 0.014625).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.end_i_axial_force - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.end_i_shear_y - 6.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.end_i_bending_z - 18.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_axial_force - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_shear_y - 3.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_bending_z - 2.25).abs() <= INTERNAL_ASSERTION_EPSILON
        && result.distributed_recovery_hook_count == 1
        && result.axial_effect_recovery_hook_count == 2
        && result.station_sweep.len() == expected_sweep.len()
        && result
            .station_sweep
            .iter()
            .zip(expected_sweep.iter())
            .all(|(actual, expected)| {
                actual.station_fraction.is_finite()
                    && actual.axial_force.is_finite()
                    && actual.shear_y.is_finite()
                    && actual.bending_z.is_finite()
                    && (actual.station_fraction - expected.station_fraction).abs()
                        <= INTERNAL_ASSERTION_EPSILON
                    && (actual.axial_force - expected.axial_force).abs()
                        <= INTERNAL_ASSERTION_EPSILON
                    && (actual.shear_y - expected.shear_y).abs() <= INTERNAL_ASSERTION_EPSILON
                    && (actual.bending_z - expected.bending_z).abs() <= INTERNAL_ASSERTION_EPSILON
            })
}

pub fn validate_tp_phys_014_canonical_analytical_payload() -> bool {
    let Ok(result) = solve_tp_phys_014_canonical_analytical_payload() else {
        return false;
    };

    [
        result.assembled_node_0_uy_force,
        result.assembled_node_0_rz_moment,
        result.assembled_node_1_uy_force,
        result.assembled_node_1_rz_moment,
        result.support_reaction_node_0_uy_force,
        result.support_reaction_node_0_rz_moment,
        result.node_1_uy_displacement,
        result.node_1_rz_rotation,
        result.midspan_shear_y,
        result.midspan_bending_z,
    ]
    .into_iter()
    .all(f64::is_finite)
        && result.distributed_load_count == 1
        && result.point_force_count == 1
        && result.y_reference == [0.0, 1.0, 0.0]
        && (result.assembled_node_0_uy_force + 6.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_0_rz_moment + 4.666666666666667).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_1_uy_force + 6.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.assembled_node_1_rz_moment - 4.666666666666667).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (result.support_reaction_node_0_uy_force - 12.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.support_reaction_node_0_rz_moment - 24.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.node_1_uy_displacement + 0.04533333333333334).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.node_1_rz_rotation + 0.014666666666666668).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_shear_y - 4.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && (result.midspan_bending_z - 4.0).abs() <= INTERNAL_ASSERTION_EPSILON
        && result.distributed_recovery_hook_count == 1
        && result.point_force_recovery_hook_count == 1
}

fn tp_phys_004_pipe() -> Result<StraightPipeElement, String> {
    let section = StraightPipeSectionProperties::new(1000.0, 400.0, 3.0, 1.5, 2.0, 1.0, None)
        .map_err(|error| error.to_string())?;
    StraightPipeElement::new(
        "tp-phys-004-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        FrameNode::new(1, [4.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        section,
        [0.0, 1.0, 0.0],
    )
    .map_err(|error| error.to_string())
}

fn tp_phys_005_pipe() -> Result<StraightPipeElement, String> {
    let section = StraightPipeSectionProperties::new(1000.0, 400.0, 3.0, 1.5, 2.0, 1.0, None)
        .map_err(|error| error.to_string())?;
    StraightPipeElement::new(
        "tp-phys-005-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        FrameNode::new(1, [0.0, 4.0, 0.0]).map_err(|error| error.to_string())?,
        section,
        [1.0, 0.0, 0.0],
    )
    .map_err(|error| error.to_string())
}

fn tp_phys_006_pipe() -> Result<StraightPipeElement, String> {
    let section = StraightPipeSectionProperties::new(1000.0, 400.0, 3.0, 1.5, 2.0, 1.0, None)
        .map_err(|error| error.to_string())?;
    StraightPipeElement::new(
        "tp-phys-006-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        FrameNode::new(1, [4.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        section,
        [0.0, 1.0, 0.0],
    )
    .map_err(|error| error.to_string())
}

fn tp_phys_007_pipe() -> Result<StraightPipeElement, String> {
    let section = StraightPipeSectionProperties::new(1000.0, 400.0, 3.0, 1.5, 2.0, 1.0, None)
        .map_err(|error| error.to_string())?;
    StraightPipeElement::new(
        "tp-phys-007-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        FrameNode::new(1, [4.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        section,
        [0.0, 1.0, 0.0],
    )
    .map_err(|error| error.to_string())
}

fn tp_phys_008_pipe() -> Result<StraightPipeElement, String> {
    let section = StraightPipeSectionProperties::new(1000.0, 400.0, 4.0, 1.5, 2.0, 1.0, None)
        .map_err(|error| error.to_string())?;
    StraightPipeElement::new(
        "tp-phys-008-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        FrameNode::new(1, [6.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        section,
        [0.0, 1.0, 0.0],
    )
    .map_err(|error| error.to_string())
}

fn tp_phys_009_pipe() -> Result<StraightPipeElement, String> {
    let section = StraightPipeSectionProperties::new(1000.0, 400.0, 4.0, 1.5, 2.0, 1.0, None)
        .map_err(|error| error.to_string())?;
    StraightPipeElement::new(
        "tp-phys-009-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        FrameNode::new(1, [6.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        section,
        [0.0, 1.0, 0.0],
    )
    .map_err(|error| error.to_string())
}

fn tp_phys_008_axial_effect_properties() -> ElementAxialEffectProperties {
    ElementAxialEffectProperties::new(0, Some(1000.0), Some(4.0), Some(1.0e-5), Some(0.1))
}

fn tp_phys_009_axial_effect_properties() -> ElementAxialEffectProperties {
    ElementAxialEffectProperties::new(0, Some(1000.0), Some(4.0), Some(1.0e-5), Some(0.1))
}

fn tp_phys_008_axial_effect_loads() -> Vec<PrimitiveLoad> {
    vec![
        PrimitiveLoad::uniform_element_load(
            "tp-phys-008-thermal-restraint",
            PrimitiveLoadCategory::Thermal,
            0,
            LoadDirection::GlobalX,
            LoadQuantity::new(75.0, LoadDimension::TemperatureChange)
                .expect("fixture temperature change is finite"),
        ),
        PrimitiveLoad::uniform_element_load(
            "tp-phys-008-pressure-thrust",
            PrimitiveLoadCategory::Pressure,
            0,
            LoadDirection::GlobalX,
            LoadQuantity::new(90.0, LoadDimension::Pressure).expect("fixture pressure is finite"),
        ),
    ]
}

fn tp_phys_009_axial_effect_loads() -> Vec<PrimitiveLoad> {
    vec![
        PrimitiveLoad::uniform_element_load(
            "tp-phys-009-thermal-restraint",
            PrimitiveLoadCategory::Thermal,
            0,
            LoadDirection::GlobalX,
            LoadQuantity::new(75.0, LoadDimension::TemperatureChange)
                .expect("fixture temperature change is finite"),
        ),
        PrimitiveLoad::uniform_element_load(
            "tp-phys-009-pressure-thrust",
            PrimitiveLoadCategory::Pressure,
            0,
            LoadDirection::GlobalX,
            LoadQuantity::new(90.0, LoadDimension::Pressure).expect("fixture pressure is finite"),
        ),
    ]
}

fn tp_phys_014_nodes(
    model: &Map<String, Value>,
) -> Result<(BTreeMap<String, FrameNode>, BTreeMap<String, usize>), String> {
    let mut nodes = BTreeMap::new();
    let mut node_index_by_id = BTreeMap::new();
    for (index, node) in array_field(model, "nodes", "payload.model")?
        .iter()
        .enumerate()
    {
        let node = object_value(node, "payload.model.nodes[]")?;
        let node_id = require_string_field(node, "id", "payload.model.nodes[]")?;
        let coordinates = object_field(node, "coordinates", "payload.model.nodes[]")?;
        let frame_node = FrameNode::new(
            index,
            [
                quantity_field(
                    object_field(coordinates, "x", "payload.model.nodes[].coordinates")?,
                    "length",
                    "payload.model.nodes[].coordinates.x",
                )?,
                quantity_field(
                    object_field(coordinates, "y", "payload.model.nodes[].coordinates")?,
                    "length",
                    "payload.model.nodes[].coordinates.y",
                )?,
                quantity_field(
                    object_field(coordinates, "z", "payload.model.nodes[].coordinates")?,
                    "length",
                    "payload.model.nodes[].coordinates.z",
                )?,
            ],
        )
        .map_err(|error| error.to_string())?;
        node_index_by_id.insert(node_id.to_string(), index);
        nodes.insert(node_id.to_string(), frame_node);
    }
    Ok((nodes, node_index_by_id))
}

fn tp_phys_014_section_properties(
    material: &Map<String, Value>,
    section: &Map<String, Value>,
) -> Result<StraightPipeSectionProperties, String> {
    let material_properties = object_field(material, "properties", "payload.model.materials[]")?;
    let section_properties = object_field(section, "properties", "payload.model.sections[]")?;
    StraightPipeSectionProperties::new(
        quantity_field(
            object_field(
                material_properties,
                "elastic_modulus",
                "payload.model.materials[].properties",
            )?,
            "stress",
            "payload.model.materials[].properties.elastic_modulus",
        )?,
        quantity_field(
            object_field(
                material_properties,
                "shear_modulus",
                "payload.model.materials[].properties",
            )?,
            "stress",
            "payload.model.materials[].properties.shear_modulus",
        )?,
        quantity_field(
            object_field(
                section_properties,
                "area",
                "payload.model.sections[].properties",
            )?,
            "area",
            "payload.model.sections[].properties.area",
        )?,
        quantity_field(
            object_field(
                section_properties,
                "second_moment_y",
                "payload.model.sections[].properties",
            )?,
            "second_moment_area",
            "payload.model.sections[].properties.second_moment_y",
        )?,
        quantity_field(
            object_field(
                section_properties,
                "second_moment_z",
                "payload.model.sections[].properties",
            )?,
            "second_moment_area",
            "payload.model.sections[].properties.second_moment_z",
        )?,
        quantity_field(
            object_field(
                section_properties,
                "torsion_constant",
                "payload.model.sections[].properties",
            )?,
            "second_moment_area",
            "payload.model.sections[].properties.torsion_constant",
        )?,
        None,
    )
    .map_err(|error| error.to_string())
}

fn tp_phys_014_anchor_node_index(
    model: &Map<String, Value>,
    node_index_by_id: &BTreeMap<String, usize>,
) -> Result<usize, String> {
    for support in array_field(model, "supports", "payload.model")? {
        let support = object_value(support, "payload.model.supports[]")?;
        if require_string_field(support, "support_type", "payload.model.supports[]")? != "anchor" {
            continue;
        }
        let node_id = reference_id(
            object_field(support, "target_ref", "payload.model.supports[]")?,
            "Node",
            "payload.model.supports[].target_ref",
        )?;
        let directions = array_field(support, "directions", "payload.model.supports[]")?;
        let direction_values = directions
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .ok_or_else(|| "TP-PHYS-014 support direction must be a string".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        if direction_values != ["UX", "UY", "UZ", "RX", "RY", "RZ"] {
            return Err(
                "TP-PHYS-014 anchor support must carry explicit UX, UY, UZ, RX, RY, RZ directions"
                    .to_string(),
            );
        }
        return node_index_by_id
            .get(node_id)
            .copied()
            .ok_or_else(|| format!("TP-PHYS-014 support target node {node_id} is unresolved"));
    }
    Err("TP-PHYS-014 payload must contain an anchor support".to_string())
}

#[derive(Debug, Clone, PartialEq)]
struct RuntimeLoadTraceAnchor {
    load_id: String,
    source_ref: Reference,
    adapter_dto_ref: Reference,
}

fn tp_phys_014_loads(
    model: &Map<String, Value>,
    element_id: &str,
    pipe_length: f64,
) -> Result<
    (
        Vec<UserLoad>,
        Vec<SpannedUniformLocalLoad>,
        Vec<PointLocalForce>,
        usize,
        usize,
        Vec<RuntimeLoadTraceAnchor>,
    ),
    String,
> {
    let load_cases = array_field(model, "load_cases", "payload.model")?;
    let load_case = only_record(load_cases, "payload.model.load_cases")?;
    let load_case_id = require_string_field(load_case, "id", "payload.model.load_cases[0]")?;
    let loads = array_field(load_case, "loads", "payload.model.load_cases[0]")?;
    let mut user_loads = Vec::new();
    let mut local_uniforms = Vec::new();
    let mut local_points = Vec::new();
    let mut distributed_load_count = 0;
    let mut point_force_count = 0;
    let mut trace_anchors = Vec::new();

    for (index, load) in loads.iter().enumerate() {
        let path = format!("payload.model.load_cases[0].loads[{index}]");
        let load = object_value(load, &path)?;
        let record_type = require_string_field(load, "load_record_type", &path)?;
        reference_id(
            object_field(load, "target_ref", &path)?,
            "Element",
            &format!("{path}.target_ref"),
        )
        .and_then(|target_id| {
            (target_id == element_id)
                .then_some(target_id)
                .ok_or_else(|| format!("{path}.target_ref must target element {element_id}"))
        })?;

        match record_type {
            "element_uniform_distributed_force" => {
                distributed_load_count += 1;
                let load_id = format!("tp-phys-014-load-{index}-element-uniform-distributed-force");
                let direction = user_direction(require_string_field(load, "direction", &path)?)?;
                let span = object_field(load, "span", &path)?;
                let start_fraction = quantity_field(
                    object_field(span, "start_fraction", &format!("{path}.span"))?,
                    "dimensionless",
                    &format!("{path}.span.start_fraction"),
                )?;
                let end_fraction = quantity_field(
                    object_field(span, "end_fraction", &format!("{path}.span"))?,
                    "dimensionless",
                    &format!("{path}.span.end_fraction"),
                )?;
                let force_per_length = quantity_field(
                    object_field(load, "quantity", &path)?,
                    "force_per_length",
                    &format!("{path}.quantity"),
                )?;
                let user_span = UserElementLoadSpan::new(start_fraction, end_fraction)
                    .map_err(|error| error.to_string())?;
                user_loads.push(UserLoad::uniform_distributed(
                    load_id.clone(),
                    0,
                    direction,
                    UserLoadQuantity::new(force_per_length, LoadDimension::ForcePerLength)
                        .map_err(|error| error.to_string())?,
                    user_span,
                    Some(pipe_length),
                ));
                local_uniforms.push(
                    SpannedUniformLocalLoad::new(
                        local_direction(direction)?,
                        force_per_length,
                        UniformLoadSpan::new(start_fraction, end_fraction)
                            .map_err(|error| error.to_string())?,
                    )
                    .map_err(|error| error.to_string())?,
                );
                trace_anchors.push(RuntimeLoadTraceAnchor {
                    load_id,
                    source_ref: Reference::new("load_case", format!("{load_case_id}:load:{index}")),
                    adapter_dto_ref: Reference::new(
                        "adapter_dto",
                        format!("dto:load_application:{load_case_id}:{index}"),
                    ),
                });
            }
            "element_point_force" => {
                point_force_count += 1;
                let load_id = format!("tp-phys-014-load-{index}-element-point-force");
                let direction = user_direction(require_string_field(load, "direction", &path)?)?;
                let station_fraction = quantity_field(
                    object_field(load, "station_fraction", &path)?,
                    "dimensionless",
                    &format!("{path}.station_fraction"),
                )?;
                let force = quantity_field(
                    object_field(load, "quantity", &path)?,
                    "force",
                    &format!("{path}.quantity"),
                )?;
                user_loads.push(UserLoad::element_concentrated_force(
                    load_id.clone(),
                    0,
                    station_fraction,
                    direction,
                    UserLoadQuantity::new(force, LoadDimension::Force)
                        .map_err(|error| error.to_string())?,
                ));
                local_points.push(
                    PointLocalForce::new(station_fraction, local_direction(direction)?, force)
                        .map_err(|error| error.to_string())?,
                );
                trace_anchors.push(RuntimeLoadTraceAnchor {
                    load_id,
                    source_ref: Reference::new("load_case", format!("{load_case_id}:load:{index}")),
                    adapter_dto_ref: Reference::new(
                        "adapter_dto",
                        format!("dto:load_application:{load_case_id}:{index}"),
                    ),
                });
            }
            other => {
                return Err(format!(
                    "TP-PHYS-014 payload contains unsupported load_record_type {other}"
                ))
            }
        }
    }

    Ok((
        user_loads,
        local_uniforms,
        local_points,
        distributed_load_count,
        point_force_count,
        trace_anchors,
    ))
}

fn tp_phys_015a_result_provenance() -> ExportProvenance {
    ExportProvenance {
        source_name: "OpenPipeStress original mechanics benchmark".to_string(),
        source_location:
            "validation/benchmarks/mechanics/fixtures/tp_phys_014_canonical_analytical_payload.json"
                .to_string(),
        source_license: "project-original-public-content".to_string(),
        contributor: "OpenPipeStress agentic development workflow".to_string(),
        contributor_certification: "Generated from elementary open mechanics; not copied from protected standards, commercial software examples, or proprietary data.".to_string(),
        redistribution_status: ExportRedistributionStatus::InventedNonEngineeringExample,
        review_status: "accepted".to_string(),
    }
}

fn tp_phys_014_node_id_by_index(
    node_index_by_id: &BTreeMap<String, usize>,
    node_count: usize,
) -> Result<Vec<String>, String> {
    let mut node_id_by_index = vec![String::new(); node_count];
    for (node_id, &node_index) in node_index_by_id {
        let slot = node_id_by_index
            .get_mut(node_index)
            .ok_or_else(|| format!("TP-PHYS-014 node index {node_index} is outside node count"))?;
        *slot = node_id.clone();
    }
    if let Some((node_index, _)) = node_id_by_index
        .iter()
        .enumerate()
        .find(|(_, node_id)| node_id.is_empty())
    {
        return Err(format!(
            "TP-PHYS-014 node index {node_index} has no stable node id"
        ));
    }
    Ok(node_id_by_index)
}

fn tp_phys_014_load_vector_result_id(node_id: &str, dof_index: usize) -> Option<String> {
    match dof_index {
        UY => Some(format!("result:load-vector:node-{node_id}:uy")),
        RZ => Some(format!("result:load-vector:node-{node_id}:rz")),
        _ => None,
    }
}

fn tp_phys_014_dof_label(dof_index: usize) -> Option<&'static str> {
    match dof_index {
        UX => Some("ux"),
        UY => Some("uy"),
        UZ => Some("uz"),
        RX => Some("rx"),
        RY => Some("ry"),
        RZ => Some("rz"),
        _ => None,
    }
}

fn tp_phys_014_load_vector_trace_chains(
    nodal_loads: &[open_pipe_stress_user_loads::NodalLoadContribution],
    trace_anchors: &[RuntimeLoadTraceAnchor],
    node_id_by_index: &[String],
) -> Result<Vec<CanonicalResultTraceChain>, String> {
    let provenance = tp_phys_015a_result_provenance();
    let anchors_by_load_id = trace_anchors
        .iter()
        .map(|anchor| (anchor.load_id.as_str(), anchor))
        .collect::<BTreeMap<_, _>>();
    let mut chains_by_result_id: BTreeMap<String, Vec<ResultTraceLink>> = BTreeMap::new();

    for load in nodal_loads {
        let node_id = node_id_by_index.get(load.node_index).ok_or_else(|| {
            format!(
                "TP-PHYS-014 load references missing node {}",
                load.node_index
            )
        })?;
        let dof_index = load.global_dof % DOF_PER_NODE;
        let Some(result_id) = tp_phys_014_load_vector_result_id(node_id, dof_index) else {
            continue;
        };
        let dof_label = tp_phys_014_dof_label(dof_index)
            .ok_or_else(|| format!("TP-PHYS-014 unsupported DOF index {dof_index}"))?;
        let anchor = anchors_by_load_id
            .get(load.trace.load_id.as_str())
            .ok_or_else(|| {
                format!(
                    "TP-PHYS-014 missing trace anchor for {}",
                    load.trace.load_id
                )
            })?;
        let solver_input_ref = Reference::new(
            "solver_nodal_load_contribution",
            format!("{}:node-{node_id}:{dof_label}", load.trace.load_id),
        );
        let result_value_ref = Reference::new("result_value", result_id.clone());
        let trace_chain = chains_by_result_id.entry(result_id.clone()).or_default();
        trace_chain.push(ResultTraceLink {
            trace_id: format!(
                "trace:tp-phys-015:{}:source-to-adapter-dto",
                load.trace.load_id
            ),
            trace_type: "analytical_model_to_adapter_dto".to_string(),
            source_ref: anchor.source_ref.clone(),
            target_ref: anchor.adapter_dto_ref.clone(),
            provenance: provenance.clone(),
            diagnostics: Vec::new(),
        });
        trace_chain.push(ResultTraceLink {
            trace_id: format!(
                "trace:tp-phys-015:{}:adapter-dto-to-solver-input:node-{node_id}:{dof_label}",
                load.trace.load_id
            ),
            trace_type: "adapter_dto_to_solver_input".to_string(),
            source_ref: anchor.adapter_dto_ref.clone(),
            target_ref: solver_input_ref.clone(),
            provenance: provenance.clone(),
            diagnostics: Vec::new(),
        });
        trace_chain.push(ResultTraceLink {
            trace_id: format!(
                "trace:tp-phys-015:{}:solver-input-to-result:{}",
                load.trace.load_id, result_id
            ),
            trace_type: "solver_input_to_result_value".to_string(),
            source_ref: solver_input_ref,
            target_ref: result_value_ref,
            provenance: provenance.clone(),
            diagnostics: Vec::new(),
        });
    }

    Ok(chains_by_result_id
        .into_iter()
        .map(|(result_id, trace_chain)| CanonicalResultTraceChain {
            result_id,
            trace_chain,
        })
        .collect())
}

fn tp_phys_015a_result_metadata(
    component: &str,
    coordinate_system: &str,
    location: &str,
    basis: &str,
    sign_convention: &str,
) -> ResultMetadata {
    ResultMetadata {
        component: component.to_string(),
        coordinate_system: coordinate_system.to_string(),
        location: location.to_string(),
        basis: basis.to_string(),
        sign_convention: sign_convention.to_string(),
    }
}

fn tp_phys_015a_runtime_trace_chain(
    result: &CanonicalAnalyticalPayloadSolverResult,
    result_id: &str,
) -> Vec<ResultTraceLink> {
    result
        .load_vector_trace_chains
        .iter()
        .find(|chain| chain.result_id == result_id)
        .map(|chain| chain.trace_chain.clone())
        .unwrap_or_default()
}

fn tp_phys_015a_with_trace_chain(
    mut value: QuantityResult,
    trace_chain: Vec<ResultTraceLink>,
) -> QuantityResult {
    value.trace_chain = trace_chain;
    value
}

fn tp_phys_015a_quantity(
    result_id: &str,
    family: ResultFamily,
    object_ref: Reference,
    basis_ref: &Reference,
    station_ref: Option<Reference>,
    magnitude: f64,
    unit: &str,
    dimension: ExportDimensionId,
    metadata: Option<ResultMetadata>,
) -> QuantityResult {
    QuantityResult {
        result_id: result_id.to_string(),
        family,
        object_ref,
        basis_ref: basis_ref.clone(),
        station_ref,
        magnitude,
        unit: unit.to_string(),
        dimension,
        metadata,
        diagnostics: Vec::new(),
        trace_chain: Vec::new(),
        provenance: tp_phys_015a_result_provenance(),
    }
}

fn tp_phys_015a_checksum(payload_id: &str, payload: &[u8]) -> ChecksumRef {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    ChecksumRef {
        algorithm: "sha256".to_string(),
        canonicalization: "JCS".to_string(),
        payload_ref: Reference::new("payload", payload_id),
        value: format!("{:x}", hasher.finalize()),
    }
}

fn sorted_tp_phys_015a_result_ids(envelope: &ResultEnvelope) -> Vec<&str> {
    sorted_result_values(envelope)
        .into_iter()
        .map(|value| value.result_id.as_str())
        .collect()
}

fn tp_phys_015a_load_vector_trace_chains_are_runtime_evidence(envelope: &ResultEnvelope) -> bool {
    let load_vector_values = sorted_result_values(envelope)
        .into_iter()
        .filter(|value| value.result_id.starts_with("result:load-vector:"))
        .collect::<Vec<_>>();
    load_vector_values.len() == 4
        && load_vector_values.iter().all(|value| {
            value.trace_chain.len() == 6
                && value.trace_chain.iter().all(|link| {
                    link.target_ref.ref_type == "result_value"
                        || link.target_ref.ref_type == "adapter_dto"
                        || link.target_ref.ref_type == "solver_nodal_load_contribution"
                })
                && value.trace_chain.iter().any(|link| {
                    link.trace_type == "analytical_model_to_adapter_dto"
                        && link.source_ref.ref_type == "load_case"
                        && link.source_ref.ref_id.starts_with("LC-TP-PHYS-014:load:")
                        && link.target_ref.ref_type == "adapter_dto"
                        && link
                            .target_ref
                            .ref_id
                            .starts_with("dto:load_application:LC-TP-PHYS-014:")
                })
                && value.trace_chain.iter().any(|link| {
                    link.trace_type == "adapter_dto_to_solver_input"
                        && link.source_ref.ref_type == "adapter_dto"
                        && link.target_ref.ref_id.starts_with("tp-phys-014-load-")
                })
                && value.trace_chain.iter().any(|link| {
                    link.trace_type == "solver_input_to_result_value"
                        && link.source_ref.ref_type == "solver_nodal_load_contribution"
                        && link.target_ref
                            == Reference::new("result_value", value.result_id.as_str())
                })
        })
}

fn object_value<'a>(value: &'a Value, path: &str) -> Result<&'a Map<String, Value>, String> {
    value
        .as_object()
        .ok_or_else(|| format!("{path} must be a JSON object"))
}

fn object_field<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    path: &str,
) -> Result<&'a Map<String, Value>, String> {
    object
        .get(key)
        .ok_or_else(|| format!("{path}.{key} is required"))
        .and_then(|value| object_value(value, &format!("{path}.{key}")))
}

fn array_field<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    path: &str,
) -> Result<&'a Vec<Value>, String> {
    object
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{path}.{key} must be an array"))
}

fn require_string_field<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    path: &str,
) -> Result<&'a str, String> {
    object
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{path}.{key} must be a string"))
}

fn only_record<'a>(records: &'a [Value], path: &str) -> Result<&'a Map<String, Value>, String> {
    if records.len() != 1 {
        return Err(format!("{path} must contain exactly one record"));
    }
    object_value(&records[0], &format!("{path}[0]"))
}

fn record_by_id<'a>(
    records: &'a [Value],
    id: &str,
    path: &str,
) -> Result<&'a Map<String, Value>, String> {
    for record in records {
        let record = object_value(record, &format!("{path}[]"))?;
        if require_string_field(record, "id", &format!("{path}[]"))? == id {
            return Ok(record);
        }
    }
    Err(format!("{path} is missing record {id}"))
}

fn reference_id<'a>(
    object: &'a Map<String, Value>,
    expected_object_type: &str,
    path: &str,
) -> Result<&'a str, String> {
    let object_type = require_string_field(object, "object_type", path)?;
    if object_type != expected_object_type {
        return Err(format!(
            "{path}.object_type must be {expected_object_type}, got {object_type}"
        ));
    }
    require_string_field(object, "id", path)
}

fn quantity_field(
    object: &Map<String, Value>,
    expected_dimension: &str,
    path: &str,
) -> Result<f64, String> {
    let unit = require_string_field(object, "unit", path)?;
    if unit.is_empty() {
        return Err(format!("{path}.unit must not be empty"));
    }
    let dimension = require_string_field(object, "dimension", path)?;
    if dimension != expected_dimension {
        return Err(format!(
            "{path}.dimension must be {expected_dimension}, got {dimension}"
        ));
    }
    let value = object
        .get("value")
        .and_then(Value::as_f64)
        .ok_or_else(|| format!("{path}.value must be numeric"))?;
    value
        .is_finite()
        .then_some(value)
        .ok_or_else(|| format!("{path}.value must be finite"))
}

fn direction_vector_field(
    object: &Map<String, Value>,
    key: &str,
    path: &str,
) -> Result<[f64; 3], String> {
    let values = object
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{path}.{key} must be a direction vector"))?;
    if values.len() != 3 {
        return Err(format!("{path}.{key} must contain three values"));
    }
    let mut vector = [0.0; 3];
    for (index, value) in values.iter().enumerate() {
        vector[index] = value
            .as_f64()
            .ok_or_else(|| format!("{path}.{key}[{index}] must be numeric"))?;
        if !vector[index].is_finite() {
            return Err(format!("{path}.{key}[{index}] must be finite"));
        }
    }
    Ok(vector)
}

fn user_direction(value: &str) -> Result<UserLoadDirection, String> {
    match value {
        "X" => Ok(UserLoadDirection::GlobalX),
        "Y" => Ok(UserLoadDirection::GlobalY),
        "Z" => Ok(UserLoadDirection::GlobalZ),
        other => Err(format!("unsupported translational load direction {other}")),
    }
}

fn local_direction(value: UserLoadDirection) -> Result<LocalLoadDirection, String> {
    match value {
        UserLoadDirection::GlobalX => Ok(LocalLoadDirection::X),
        UserLoadDirection::GlobalY => Ok(LocalLoadDirection::Y),
        UserLoadDirection::GlobalZ => Ok(LocalLoadDirection::Z),
        UserLoadDirection::RotationX
        | UserLoadDirection::RotationY
        | UserLoadDirection::RotationZ => {
            Err("rotational direction is not valid for force recovery".to_string())
        }
    }
}

fn expected_tp_phys_007_station_sweep() -> [StationSweepResultant; 4] {
    [
        StationSweepResultant {
            station_fraction: 0.75,
            shear_y: 0.0,
            bending_z: 0.0,
        },
        StationSweepResultant {
            station_fraction: 0.25,
            shear_y: 4.0,
            bending_z: 4.0,
        },
        StationSweepResultant {
            station_fraction: 0.5,
            shear_y: 2.0,
            bending_z: 1.0,
        },
        StationSweepResultant {
            station_fraction: 1.0,
            shear_y: 0.0,
            bending_z: 0.0,
        },
    ]
}

fn expected_tp_phys_008_axial_station_sweep() -> [AxialEffectStationResultant; 3] {
    [
        AxialEffectStationResultant {
            station_fraction: 1.0,
            axial_force: 12.0,
            shear_y: 0.0,
            bending_z: 0.0,
        },
        AxialEffectStationResultant {
            station_fraction: 0.0,
            axial_force: 12.0,
            shear_y: 0.0,
            bending_z: 0.0,
        },
        AxialEffectStationResultant {
            station_fraction: 0.5,
            axial_force: 12.0,
            shear_y: 0.0,
            bending_z: 0.0,
        },
    ]
}

fn expected_tp_phys_009_combined_station_sweep() -> [AxialEffectStationResultant; 4] {
    [
        AxialEffectStationResultant {
            station_fraction: 0.25,
            axial_force: 12.0,
            shear_y: 6.0,
            bending_z: 9.0,
        },
        AxialEffectStationResultant {
            station_fraction: 0.5,
            axial_force: 12.0,
            shear_y: 3.0,
            bending_z: 2.25,
        },
        AxialEffectStationResultant {
            station_fraction: 0.75,
            axial_force: 12.0,
            shear_y: 0.0,
            bending_z: 0.0,
        },
        AxialEffectStationResultant {
            station_fraction: 1.0,
            axial_force: 12.0,
            shear_y: 0.0,
            bending_z: 0.0,
        },
    ]
}

fn axial_force_by_load_id(
    axial_effects: &[PrimitiveAxialEffectContribution],
    load_id: &str,
) -> Result<f64, String> {
    axial_effects
        .iter()
        .find(|effect| effect.load_id == load_id)
        .map(|effect| effect.axial_force)
        .ok_or_else(|| format!("missing axial effect contribution {load_id}"))
}

fn benchmark_section() -> Result<FrameSection, FrameKernelError> {
    FrameSection::new(1200.0, 500.0, 2.0, 3.0, 4.0, 1.5)
}

fn tp_phys_002_model() -> Result<(StraightPipeElement, Vec<Vec<f64>>, Vec<f64>, Vec<f64>), String> {
    let section = StraightPipeSectionProperties::new(1500.0, 600.0, 2.0, 1.8, 2.2, 0.9, None)
        .map_err(|error| error.to_string())?;
    let pipe = StraightPipeElement::new(
        "tp-phys-002-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        FrameNode::new(1, [4.0, 0.0, 0.0]).map_err(|error| error.to_string())?,
        section,
        [0.0, 1.0, 0.0],
    )
    .map_err(|error| error.to_string())?;
    let frame_element = pipe.frame_element().map_err(|error| error.to_string())?;
    let stiffness =
        assemble_global_stiffness(2, &[frame_element]).map_err(|error| error.to_string())?;

    let nodal_load = PrimitiveLoad::nodal_force(
        "tp-phys-002-node-1-axial",
        PrimitiveLoadCategory::Occasional,
        1,
        LoadDirection::GlobalX,
        LoadQuantity::new(12.0, LoadDimension::Force).expect("finite nodal load"),
    );
    let nodal_application = prepare_loads(2, 1, &[nodal_load]);
    if nodal_application.is_blocked() {
        return Err("TP-PHYS-002 nodal load preparation produced findings".to_string());
    }

    let span = ElementLoadSpan::new(0, 0, 1, 4.0).map_err(|error| error.to_string())?;
    let uniform_load = PrimitiveLoad::uniform_element_load(
        "tp-phys-002-uniform-y",
        PrimitiveLoadCategory::Weight,
        0,
        LoadDirection::GlobalY,
        LoadQuantity::new(-2.0, LoadDimension::ForcePerLength).expect("finite uniform load"),
    );
    let lumped_application = prepare_lumped_nodal_loads(2, 1, &[span], &[uniform_load]);
    if lumped_application.is_blocked() {
        return Err("TP-PHYS-002 lumped load preparation produced findings".to_string());
    }

    let mut force = nodal_application.global_load_vector(2);
    let lumped_force = lumped_application.global_load_vector(2);
    for (target, value) in force.iter_mut().zip(lumped_force.iter()) {
        *target += value;
    }

    Ok((pipe, stiffness, force, lumped_force))
}

fn tp_phys_002_supports() -> [LinearSupport; 3] {
    let spring = SupportQuantity::positive(40.0, QuantityDimension::TranslationalStiffness)
        .expect("fixture spring stiffness is positive");
    let imposed = SupportQuantity::new(-0.01, QuantityDimension::Displacement)
        .expect("fixture imposed displacement is finite");

    [
        LinearSupport::anchor("tp-phys-002-node-0-anchor", 0),
        LinearSupport::spring(
            "tp-phys-002-node-1-uy-spring",
            1,
            FrameDof::Uy,
            Some(spring),
        ),
        LinearSupport::imposed_displacement(
            "tp-phys-002-node-1-uz-imposed",
            1,
            FrameDof::Uz,
            Some(imposed),
        ),
    ]
}

fn all_node_dofs(node_index: usize) -> Vec<usize> {
    let base = node_index * DOF_PER_NODE;
    vec![
        base + UX,
        base + UY,
        base + UZ,
        base + RX,
        base + RY,
        base + RZ,
    ]
}

fn matrix_is_symmetric(matrix: &[[f64; 12]; 12]) -> bool {
    for row in 0..12 {
        for col in 0..12 {
            if (matrix[row][col] - matrix[col][row]).abs() > INTERNAL_ASSERTION_EPSILON {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_covers_required_mechanics_families() {
        let fixtures = fixture_inventory();
        assert!(missing_required_families(&fixtures).is_empty());
        assert_eq!(fixtures.len(), 17);
        assert!(fixtures
            .iter()
            .any(|fixture| fixture.fixture_id == "MECH-TP-PHYS-007-STATION-SWEEP-RESULTANTS"));
        assert!(
            fixtures
                .iter()
                .any(|fixture| fixture.fixture_id
                    == "MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS")
        );
        assert!(fixtures
            .iter()
            .any(|fixture| fixture.fixture_id == "MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS"));
        assert!(fixtures
            .iter()
            .any(|fixture| fixture.fixture_id == "MECH-TP-PHYS-014-CANONICAL-ANALYTICAL-PAYLOAD"));
        assert!(fixtures.iter().any(
            |fixture| fixture.fixture_id == "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE"
        ));
    }

    #[test]
    fn fixture_provenance_is_public_original_and_reviewed() {
        for fixture in fixture_inventory() {
            assert!(
                fixture.provenance.is_publicly_usable(),
                "{} lacks accepted public-original provenance",
                fixture.fixture_id
            );
            assert!(fixture.tolerance_policy_is_unresolved());
            assert!(fixture.has_dimensioned_expected_values());
        }
    }

    #[test]
    fn cantilever_tip_force_matches_open_mechanics_formula() {
        let solved_tip_displacement = solve_cantilever_tip_force().unwrap();
        let fixture = cantilever_tip_force_fixture();
        let expected = fixture.expected_values[0].value;
        assert!((solved_tip_displacement - expected).abs() <= INTERNAL_ASSERTION_EPSILON);
    }

    #[test]
    fn straight_pipe_fixture_exercises_weight_and_recovery_path() {
        let fixture = straight_pipe_weight_recovery_fixture();
        assert_eq!(fixture.expected_values[0].value, 22.5);
        assert_eq!(fixture.expected_values[1].value, 18.0);
        assert!(validate_straight_pipe_boundary());
    }

    #[test]
    fn portal_frame_fixture_solves_repeatably() {
        let fixture = portal_frame_sway_fixture();
        let solved = solve_portal_frame_sway().unwrap();
        assert!((solved - fixture.expected_values[0].value).abs() <= INTERNAL_ASSERTION_EPSILON);
        assert!(solved.is_finite());
    }

    #[test]
    fn thermal_growth_fixture_records_open_axial_restraint_formula() {
        let fixture = fixed_fixed_thermal_fixture();
        let strain = fixture.expected_values[0].value;
        let force = fixture.expected_values[1].value;
        assert!((strain - 0.0009).abs() <= INTERNAL_ASSERTION_EPSILON);
        assert!((force - 5.4).abs() <= INTERNAL_ASSERTION_EPSILON);
    }

    #[test]
    fn support_boundary_fixture_prepares_mixed_supports() {
        assert!(validate_support_boundary_fixture());
    }

    #[test]
    fn primitive_load_fixture_prepares_all_contribution_types() {
        assert!(validate_primitive_load_fixture());
    }

    #[test]
    fn tp_phys_002_integrated_fixture_solves_recovers_and_maps_diagnostics() {
        let fixture = tp_phys_002_linear_static_integration_fixture();
        assert_eq!(
            fixture.fixture_id,
            "MECH-TP-PHYS-002-LINEAR-STATIC-INTEGRATION"
        );
        assert!(validate_tp_phys_002_linear_static_integration());
        assert!(validate_tp_phys_002_diagnostic_mapping());
    }

    #[test]
    fn tp_phys_004_load_to_resultant_fixture_assembles_solves_and_recovers() {
        let fixture = tp_phys_004_load_to_resultant_fixture();
        assert_eq!(fixture.fixture_id, "MECH-TP-PHYS-004-LOAD-TO-RESULTANT");
        let result = solve_tp_phys_004_load_to_resultant_integration().unwrap();
        assert!(
            validate_tp_phys_004_load_to_resultant_integration(),
            "unexpected TP-PHYS-004 mechanics result: {result:?}"
        );
    }

    #[test]
    fn tp_phys_005_oriented_load_to_resultant_fixture_transforms_global_loads() {
        let fixture = tp_phys_005_oriented_load_to_resultant_fixture();
        assert_eq!(
            fixture.fixture_id,
            "MECH-TP-PHYS-005-ORIENTED-LOAD-TO-RESULTANT"
        );
        let result = solve_tp_phys_005_oriented_load_to_resultant_integration().unwrap();
        assert!(
            validate_tp_phys_005_oriented_load_to_resultant_integration(),
            "unexpected TP-PHYS-005 mechanics result: {result:?}"
        );
    }

    #[test]
    fn tp_phys_006_partial_span_load_to_resultant_fixture_routes_spanned_loads() {
        let fixture = tp_phys_006_partial_span_load_to_resultant_fixture();
        assert_eq!(
            fixture.fixture_id,
            "MECH-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-RESULTANT"
        );
        let result = solve_tp_phys_006_partial_span_load_to_resultant_integration().unwrap();
        assert!(
            validate_tp_phys_006_partial_span_load_to_resultant_integration(),
            "unexpected TP-PHYS-006 mechanics result: {result:?}"
        );
    }

    #[test]
    fn tp_phys_007_station_sweep_fixture_preserves_requested_order_and_values() {
        let fixture = tp_phys_007_station_sweep_resultants_fixture();
        assert_eq!(
            fixture.fixture_id,
            "MECH-TP-PHYS-007-STATION-SWEEP-RESULTANTS"
        );
        let result = solve_tp_phys_007_station_sweep_resultants_integration().unwrap();
        assert!(
            validate_tp_phys_007_station_sweep_resultants_integration(),
            "unexpected TP-PHYS-007 mechanics result: {result:?}"
        );
        assert_eq!(
            result
                .station_resultants
                .iter()
                .map(|station| station.station_fraction)
                .collect::<Vec<_>>(),
            vec![0.75, 0.25, 0.5, 1.0]
        );
    }

    #[test]
    fn tp_phys_008_thermal_pressure_fixture_prepares_and_recovers_axial_effects() {
        let fixture = tp_phys_008_thermal_pressure_axial_effects_fixture();
        assert_eq!(
            fixture.fixture_id,
            "MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS"
        );
        let result = solve_tp_phys_008_thermal_pressure_axial_effects().unwrap();
        assert!(
            validate_tp_phys_008_thermal_pressure_axial_effects(),
            "unexpected TP-PHYS-008 mechanics result: {result:?}"
        );
        assert_eq!(
            result
                .station_sweep
                .iter()
                .map(|station| station.station_fraction)
                .collect::<Vec<_>>(),
            vec![1.0, 0.0, 0.5]
        );
    }

    #[test]
    fn tp_phys_009_combined_fixture_assembles_solves_and_recovers_axial_effects() {
        let fixture = tp_phys_009_combined_load_axial_effects_fixture();
        assert_eq!(
            fixture.fixture_id,
            "MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS"
        );
        let result = solve_tp_phys_009_combined_load_axial_effects().unwrap();
        assert!(
            validate_tp_phys_009_combined_load_axial_effects(),
            "unexpected TP-PHYS-009-C mechanics result: {result:?}"
        );
        assert_eq!(
            result
                .station_sweep
                .iter()
                .map(|station| station.station_fraction)
                .collect::<Vec<_>>(),
            vec![0.25, 0.5, 0.75, 1.0]
        );
    }

    #[test]
    fn tp_phys_014_canonical_analytical_payload_drives_solver_consumption() {
        let fixture = tp_phys_014_canonical_analytical_payload_fixture();
        assert_eq!(
            fixture.fixture_id,
            "MECH-TP-PHYS-014-CANONICAL-ANALYTICAL-PAYLOAD"
        );
        let result = solve_tp_phys_014_canonical_analytical_payload().unwrap();
        assert!(
            validate_tp_phys_014_canonical_analytical_payload(),
            "unexpected TP-PHYS-014-D mechanics result: {result:?}"
        );
        assert_eq!(result.y_reference, [0.0, 1.0, 0.0]);
        assert_eq!(result.distributed_load_count, 1);
        assert_eq!(result.point_force_count, 1);
        assert_eq!(result.distributed_recovery_hook_count, 1);
        assert_eq!(result.point_force_recovery_hook_count, 1);
    }

    #[test]
    fn tp_phys_015a_canonical_payload_builds_result_boundary_evidence() {
        let fixture = tp_phys_015a_canonical_solve_result_envelope_fixture();
        assert_eq!(
            fixture.fixture_id,
            "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE"
        );
        let evidence = tp_phys_015a_canonical_solve_result_envelope().unwrap();
        assert!(
            validate_tp_phys_015a_canonical_solve_result_envelope(),
            "unexpected TP-PHYS-015A result-envelope evidence: {evidence:?}"
        );
        assert_eq!(evidence.quantity_result_count, 10);
        assert_eq!(evidence.envelope_diagnostic_count, 1);
        assert_eq!(evidence.export_validation_diagnostic_count, 0);
        assert_eq!(evidence.envelope.model_ref.ref_id, "ANALYTICAL-TP-PHYS-014");
        assert_eq!(
            evidence.envelope.load_basis_refs[0].ref_id,
            "LC-TP-PHYS-014"
        );
        let load_vector_value = sorted_result_values(&evidence.envelope)
            .into_iter()
            .find(|value| value.result_id == "result:load-vector:node-N-1:uy")
            .expect("load-vector UY result must exist");
        assert_eq!(load_vector_value.trace_chain.len(), 6);
        assert!(load_vector_value.trace_chain.iter().any(|link| {
            link.trace_type == "analytical_model_to_adapter_dto"
                && link.source_ref == Reference::new("load_case", "LC-TP-PHYS-014:load:0")
                && link.target_ref
                    == Reference::new("adapter_dto", "dto:load_application:LC-TP-PHYS-014:0")
        }));
        assert!(load_vector_value.trace_chain.iter().any(|link| {
            link.trace_type == "solver_input_to_result_value"
                && link.source_ref.ref_type == "solver_nodal_load_contribution"
                && link.target_ref
                    == Reference::new("result_value", "result:load-vector:node-N-1:uy")
        }));
    }

    #[test]
    fn imposed_displacement_fixture_prepares_support_boundary() {
        assert!(validate_imposed_displacement_fixture());
    }

    #[test]
    fn inclined_member_transform_fixture_checks_direction_cosines_and_symmetry() {
        let fixture = inclined_member_transform_fixture();
        assert!(validate_transform_fixture().unwrap());
        assert!(
            (fixture.expected_values[0].value - fixture.expected_values[1].value).abs()
                <= INTERNAL_ASSERTION_EPSILON
        );
    }
}

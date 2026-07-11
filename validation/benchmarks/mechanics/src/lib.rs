//! Original mechanics verification benchmarks for OpenPipeStress.
//!
//! The fixtures in this crate use elementary open mechanics with invented
//! numeric values. They are verification aids only: no code-specific
//! acceptance criteria, protected standards content, or professional approval
//! claims are encoded here.

use open_pipe_stress_curved_bend::CurvedBendMacroElement;
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness, element_dof_map, reduce_system, solve_dense, FrameElement,
    FrameKernelError, FrameNode, FrameSection, DOF_PER_NODE, ELEMENT_DOF, RX, RY, RZ, UX, UY, UZ,
};
use open_pipe_stress_linear_supports::{
    apply_linear_supports, prepare_boundary, FrameDof, LinearSupport, QuantityDimension,
    SupportQuantity,
};
use open_pipe_stress_primitive_loads::{
    assemble_solver_load_vector, generate_seismic_equivalent_static_loads,
    generate_wind_equivalent_static_loads, prepare_equivalent_static_loads, prepare_loads,
    prepare_lumped_nodal_loads, prepare_straight_pipe_axial_effects, ElementAxialEffectProperties,
    ElementExposedDiameter, ElementLoadSpan, ElementMassPerLength, EquivalentStaticAxisFactor,
    EquivalentStaticMechanicsBasis, LoadDimension, LoadDirection, LoadQuantity,
    PrimitiveAxialEffectContribution, PrimitiveLoad, PrimitiveLoadCategory,
    SeismicEquivalentStaticBasis, SolverNodalLoadContribution, WindEquivalentStaticBasis,
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
const BENCHMARK_README: &str = include_str!("../README.md");
const HAND_CALC_README: &str = include_str!("../../../hand_calcs/mechanics/README.md");
const TP_PHYS_014_ANALYTICAL_PAYLOAD: &str =
    include_str!("../fixtures/tp_phys_014_canonical_analytical_payload.json");
const REQUIRED_READINESS_TBD_MARKERS: &[&str] = &[
    "final tolerance policy",
    "release thresholds",
    "CI gate policy",
    "benchmark publication scope",
    "external validation claims",
    "professional reliance",
];
const FORBIDDEN_RELIANCE_CLAIM_MARKERS: &[&str] = &[
    "code compliant",
    "code-compliant",
    "complies with code",
    "approved for professional reliance",
    "certified for professional reliance",
    "sealed engineering",
    "is a protected standards example",
    "is a commercial benchmark file",
    "uses proprietary engineering values",
];
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
    BranchAssembly,
    StraightPipe,
    SupportBoundary,
    PrimitiveLoad,
    IntegratedLinearStatic,
    LoadToResultantIntegration,
    ThermalGrowth,
    ImposedDisplacement,
    StiffnessTransform,
    CurvedBendExpansionLoop,
    CurvedBendDistributedLoad,
    EquivalentStaticGeneration,
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
pub struct BranchAssemblyBenchmarkResult {
    pub branch_axial_stiffness: f64,
    pub header_lateral_stiffness: f64,
    pub junction_uy_displacement: f64,
    pub branch_tip_uy_displacement: f64,
    pub branch_axial_extension: f64,
    pub header_left_uy_reaction: f64,
    pub header_right_uy_reaction: f64,
    pub header_reaction_sum: f64,
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

// --- Static-equivalent occasional-load generation fixture
// (TP-PMM-P3-OCCLOADGEN-001, DEC-068 item 2) ---

const OCCLOADGEN_OUTSIDE_DIAMETER: f64 = 0.2;
const OCCLOADGEN_NOMINAL_WALL: f64 = 0.01;
const OCCLOADGEN_MILL_TOLERANCE: f64 = 0.00125;
const OCCLOADGEN_MATERIAL_DENSITY: f64 = 7850.0;
const OCCLOADGEN_CONTENTS_DENSITY: f64 = 800.0;
const OCCLOADGEN_INSULATION_THICKNESS: f64 = 0.025;
const OCCLOADGEN_INSULATION_DENSITY: f64 = 120.0;
/// User-entered gravity acceleration for the fixture; an explicit input,
/// not an embedded physical-constant default.
const OCCLOADGEN_USER_GRAVITY: f64 = 9.80665;
const OCCLOADGEN_G_FACTOR_X: f64 = 0.3;
const OCCLOADGEN_G_FACTOR_Z: f64 = -0.2;
const OCCLOADGEN_WIND_PRESSURE: f64 = 480.0;
const OCCLOADGEN_WIND_SHAPE_FACTOR: f64 = 0.7;
const OCCLOADGEN_SPAN_LENGTH: f64 = 3.0;

/// Mass per unit length from the fixture's own user-entered section
/// values: metal + contents + insulation over the mill-tolerance
/// effective wall (`core/section_properties/calculator.py` closed forms).
pub fn occloadgen_mass_per_length() -> f64 {
    let effective_wall = OCCLOADGEN_NOMINAL_WALL - OCCLOADGEN_MILL_TOLERANCE;
    let od = OCCLOADGEN_OUTSIDE_DIAMETER;
    let id = od - 2.0 * effective_wall;
    let metal_area = std::f64::consts::PI / 4.0 * (od.powi(2) - id.powi(2));
    let contents_area = std::f64::consts::PI / 4.0 * id.powi(2);
    let insulation_od = od + 2.0 * OCCLOADGEN_INSULATION_THICKNESS;
    let insulation_area = std::f64::consts::PI / 4.0 * (insulation_od.powi(2) - od.powi(2));
    metal_area * OCCLOADGEN_MATERIAL_DENSITY
        + contents_area * OCCLOADGEN_CONTENTS_DENSITY
        + insulation_area * OCCLOADGEN_INSULATION_DENSITY
}

pub fn occloadgen_exposed_diameter() -> f64 {
    OCCLOADGEN_OUTSIDE_DIAMETER + 2.0 * OCCLOADGEN_INSULATION_THICKNESS
}

pub fn occloadgen_seismic_basis() -> SeismicEquivalentStaticBasis {
    SeismicEquivalentStaticBasis {
        load_case_ref: "load_case:LC-OCCLOADGEN".to_string(),
        gravity_acceleration: OCCLOADGEN_USER_GRAVITY,
        axis_factors: vec![
            EquivalentStaticAxisFactor {
                direction: LoadDirection::GlobalX,
                g_factor: OCCLOADGEN_G_FACTOR_X,
            },
            EquivalentStaticAxisFactor {
                direction: LoadDirection::GlobalZ,
                g_factor: OCCLOADGEN_G_FACTOR_Z,
            },
        ],
    }
}

pub fn occloadgen_wind_basis() -> WindEquivalentStaticBasis {
    WindEquivalentStaticBasis {
        load_case_ref: "load_case:LC-OCCLOADGEN".to_string(),
        pressure: OCCLOADGEN_WIND_PRESSURE,
        shape_factor: OCCLOADGEN_WIND_SHAPE_FACTOR,
        direction: LoadDirection::GlobalY,
    }
}

pub fn tp_pmm_p3_occloadgen_equivalent_static_fixture() -> MechanicsBenchmark {
    let mass_per_length = occloadgen_mass_per_length();
    let seismic_x = mass_per_length * OCCLOADGEN_G_FACTOR_X * OCCLOADGEN_USER_GRAVITY;
    let seismic_z = mass_per_length * OCCLOADGEN_G_FACTOR_Z * OCCLOADGEN_USER_GRAVITY;
    let wind_y =
        OCCLOADGEN_WIND_PRESSURE * OCCLOADGEN_WIND_SHAPE_FACTOR * occloadgen_exposed_diameter();
    MechanicsBenchmark {
        fixture_id: "MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC",
        family: BenchmarkFamily::EquivalentStaticGeneration,
        description: "Seismic g-factor and wind pressure/shape static-equivalent generation from the fixture's own computed mass distribution and exposed diameter over user-marked spans.",
        assumptions: &[
            "All generation values are user-entered, including the gravity acceleration; no code coefficient, exposure category, catalog value, or default is encoded.",
            "The mass distribution uses metal + contents + insulation areas over the mill-tolerance effective wall.",
            "Wind projects onto the exposed diameter (outside diameter plus twice the insulation thickness) of user-marked spans only.",
            "No dynamics content is encoded; the disposition for dynamics remains with D-12.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/tp_pmm_p3_occloadgen_equivalent_static.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "mass_per_length",
                value: mass_per_length,
                unit: "kg/m",
                dimension: "mass_per_length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "seismic_intensity_global_x",
                value: seismic_x,
                unit: "N/m",
                dimension: "force_per_length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "seismic_intensity_global_z",
                value: seismic_z,
                unit: "N/m",
                dimension: "force_per_length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "wind_intensity_global_y",
                value: wind_y,
                unit: "N/m",
                dimension: "force_per_length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "seismic_lumped_end_force_global_x",
                value: seismic_x * OCCLOADGEN_SPAN_LENGTH / 2.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "wind_lumped_end_force_global_y",
                value: wind_y * OCCLOADGEN_SPAN_LENGTH / 2.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn fixture_inventory() -> Vec<MechanicsBenchmark> {
    vec![
        cantilever_tip_force_fixture(),
        portal_frame_sway_fixture(),
        branch_assembly_fixture(),
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
        expansion_loop_curved_bend_thermal_fixture(),
        curved_bend_distributed_fixed_end_fixture(),
        tp_pmm_p3_occloadgen_equivalent_static_fixture(),
    ]
}

pub fn fixture_inventory_ids() -> Vec<&'static str> {
    fixture_inventory()
        .into_iter()
        .map(|fixture| fixture.fixture_id)
        .collect()
}

pub fn readiness_boundaries_are_documented() -> bool {
    REQUIRED_READINESS_TBD_MARKERS.iter().all(|marker| {
        contains_normalized_marker(BENCHMARK_README, marker)
            && contains_normalized_marker(HAND_CALC_README, marker)
    }) && !contains_forbidden_reliance_claim(BENCHMARK_README)
        && !contains_forbidden_reliance_claim(HAND_CALC_README)
}

pub fn contains_forbidden_reliance_claim(text: &str) -> bool {
    let normalized = text.to_ascii_lowercase();
    FORBIDDEN_RELIANCE_CLAIM_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
}

fn contains_normalized_marker(text: &str, marker: &str) -> bool {
    normalize_whitespace(text).contains(&normalize_whitespace(marker))
}

fn normalize_whitespace(text: &str) -> String {
    text.to_ascii_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn missing_required_families(fixtures: &[MechanicsBenchmark]) -> Vec<BenchmarkFamily> {
    let required = [
        BenchmarkFamily::Cantilever,
        BenchmarkFamily::Frame,
        BenchmarkFamily::BranchAssembly,
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

pub fn branch_assembly_fixture() -> MechanicsBenchmark {
    let result = expected_branch_assembly_result()
        .expect("branch assembly fixture construction must remain valid");

    MechanicsBenchmark {
        fixture_id: "MECH-BRANCH-ASSEMBLY-THREE-MEMBER",
        family: BenchmarkFamily::BranchAssembly,
        description: "Invented three-member branch assembly with two header legs, one branch leg, a shared junction node, and an elementary closed-form stiffness check.",
        assumptions: &[
            "Two collinear header frame members and one perpendicular branch member share the junction node.",
            "Remote header ends are anchored; the branch tip receives a positive global Y force.",
            "The junction and branch tip leave only global UY free, making the closed-form check a two-degree stiffness network.",
            "The fixture verifies assembled topology and elementary member stiffness only; it does not encode code-derived branch factors.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/branch_assembly.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "branch_axial_stiffness",
                value: result.branch_axial_stiffness,
                unit: "N/m",
                dimension: "linear_stiffness",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "header_lateral_stiffness",
                value: result.header_lateral_stiffness,
                unit: "N/m",
                dimension: "linear_stiffness",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "junction_uy_displacement",
                value: result.junction_uy_displacement,
                unit: "m",
                dimension: "length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "branch_tip_uy_displacement",
                value: result.branch_tip_uy_displacement,
                unit: "m",
                dimension: "length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "branch_axial_extension",
                value: result.branch_axial_extension,
                unit: "m",
                dimension: "length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "header_left_uy_reaction",
                value: result.header_left_uy_reaction,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "header_right_uy_reaction",
                value: result.header_right_uy_reaction,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "header_reaction_sum",
                value: result.header_reaction_sum,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
        ],
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
                dimension: "force_per_length",
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
                dimension: "force_per_length",
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

pub fn solve_branch_assembly_benchmark() -> Result<BranchAssemblyBenchmarkResult, FrameKernelError>
{
    let section = benchmark_section()?;
    let nodes = branch_assembly_nodes()?;
    let elements = [
        FrameElement::new(nodes[0], nodes[1], section, [0.0, 1.0, 0.0])?,
        FrameElement::new(nodes[1], nodes[2], section, [0.0, 1.0, 0.0])?,
        FrameElement::new(nodes[1], nodes[3], section, [1.0, 0.0, 0.0])?,
    ];
    let stiffness = assemble_global_stiffness(nodes.len(), &elements)?;
    let mut force = vec![0.0; nodes.len() * DOF_PER_NODE];
    force[3 * DOF_PER_NODE + UY] = branch_assembly_tip_load();
    let restraints = branch_assembly_restrained_dofs();
    let reduced = reduce_system(&stiffness, &force, &restraints)?;
    let reduced_displacements = solve_dense(&reduced.stiffness, &reduced.force)?;

    let mut global_displacements = vec![0.0; nodes.len() * DOF_PER_NODE];
    for (&dof, &value) in reduced.free_dofs.iter().zip(reduced_displacements.iter()) {
        global_displacements[dof] = value;
    }

    let reactions = stiffness
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .zip(global_displacements.iter())
                .map(|(stiffness, displacement)| stiffness * displacement)
                .sum::<f64>()
                - force[row_index]
        })
        .collect::<Vec<_>>();

    let junction_uy_displacement = global_displacements[DOF_PER_NODE + UY];
    let branch_tip_uy_displacement = global_displacements[3 * DOF_PER_NODE + UY];
    let expected = expected_branch_assembly_result()?;

    Ok(BranchAssemblyBenchmarkResult {
        branch_axial_stiffness: expected.branch_axial_stiffness,
        header_lateral_stiffness: expected.header_lateral_stiffness,
        junction_uy_displacement,
        branch_tip_uy_displacement,
        branch_axial_extension: branch_tip_uy_displacement - junction_uy_displacement,
        header_left_uy_reaction: reactions[UY],
        header_right_uy_reaction: reactions[2 * DOF_PER_NODE + UY],
        header_reaction_sum: reactions[UY] + reactions[2 * DOF_PER_NODE + UY],
    })
}

// ---------------------------------------------------------------------------
// MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL
//
// Expansion-loop (L-bend) thermal-bending benchmark against the independent
// hand-calculated known-flexibility reference
// `validation/hand_calcs/mechanics/expansion_loop_curved_bend_thermal.md`
// (D-34 / DEC-070 exit evidence). All numeric inputs below are invented
// fixture values transcribed from that witness note; the in-plane bend
// flexibility factor `k` is an opaque user-entered number throughout.
// ---------------------------------------------------------------------------

// Invented witness inputs (units per PKG09-FIXTURE-UNITS-EXPLICIT-N-M-RAD-K).
const EXPANSION_LOOP_OUTER_DIAMETER: f64 = 0.2191;
const EXPANSION_LOOP_WALL_THICKNESS: f64 = 0.00818;
const EXPANSION_LOOP_ELASTIC_MODULUS: f64 = 200.0e9;
const EXPANSION_LOOP_THERMAL_EXPANSION_COEFFICIENT: f64 = 12.0e-6;
const EXPANSION_LOOP_TEMPERATURE_RISE: f64 = 150.0;
const EXPANSION_LOOP_LEG1_LENGTH: f64 = 3.0;
const EXPANSION_LOOP_BEND_RADIUS: f64 = 0.5;
const EXPANSION_LOOP_LEG2_LENGTH: f64 = 4.0;
// Invented out-of-plane-only values: the loop is loaded strictly in-plane and
// the out-of-plane DOFs are restrained, so the shear modulus and the torsion
// constant (set to 2 I in `expansion_loop_global_stiffness`) never enter the
// compared in-plane quantities.
const EXPANSION_LOOP_SHEAR_MODULUS: f64 = 80.0e9;
// The witness flexibility coefficients are bending-only. The comparison model
// replicates that assumption by scaling the cross-section area (axial
// rigidity) of all three members by this factor, per the witness verification
// appendix items 5 and 6. Boost study on this exact 4-node model (residual
// axial-flexibility error scales as ~5.5e-3 / boost; round-off grows with the
// axial-to-bending stiffness ratio):
//   boost 1.0e3 -> max reaction deviation vs witness 5.8e-6 relative
//   boost 1.0e5 -> max reaction deviation vs witness 5.8e-8 relative
//   boost 1.0e7 -> max reaction deviation vs witness 3.5e-7 relative,
//                  degraded by penalty round-off (conditioning-limited)
// 1.0e5 is chosen: best measured agreement before the extreme-penalty
// round-off regime the witness warns about sets in.
const EXPANSION_LOOP_AXIAL_RIGIDITY_BOOST: f64 = 1.0e5;
// User-entered in-plane bend flexibility sweep from the witness table.
pub const EXPANSION_LOOP_FLEXIBILITY_FACTORS: [f64; 4] = [1.0, 5.0, 10.0, 20.0];
// Reaction comparison tolerance (relative). Limiting factor: residual axial
// flexibility of the boosted bending-only comparison model (~5.5e-3 / boost
// relative, witness appendix item 6); measured max deviation 5.8e-8 relative
// at boost 1.0e5.
const EXPANSION_LOOP_REACTION_RELATIVE_TOLERANCE: f64 = 5.0e-7;
// u_y(T2) comparison tolerance (relative). Same limiting factor as the
// reactions; measured max deviation 4.5e-8 relative at boost 1.0e5.
const EXPANSION_LOOP_DISPLACEMENT_RELATIVE_TOLERANCE: f64 = 5.0e-7;
// u_x(T2) = -alpha DeltaT L2 is exact and k-independent only under the
// bending-only assumption; the finite boost leaves the residual elastic
// axial shortening H_B L2 / (E A boost) of leg 2, ~6e-8 relative at boost
// 1.0e5 (measured max deviation 6.0e-8 relative).
const EXPANSION_LOOP_UX_T2_RELATIVE_TOLERANCE: f64 = 5.0e-7;
// Whole-body equilibrium residual floors (absolute, N and N-m). Limiting
// factor: dense-solve round-off against ~2e11 N internal boosted-axial force
// scale; measured max residuals 2.9e-5 N and 7.1e-5 N-m.
pub const EXPANSION_LOOP_EQUILIBRIUM_FORCE_TOLERANCE: f64 = 1.0e-3;
pub const EXPANSION_LOOP_EQUILIBRIUM_MOMENT_TOLERANCE: f64 = 1.0e-3;
// Free-expansion identity self-check floors: with only rigid-body-blocking
// restraints the displacements must reproduce the similarity-scaling mode
// and the reactions must vanish at round-off. Limiting factor: conditioning
// of the boosted-axial system; measured max deviations 1.5e-10 m (against
// 8.1e-3 m free growth) and 1.1e-4 N (against the ~2e11 N internal force
// scale).
pub const EXPANSION_LOOP_SELF_CHECK_DISPLACEMENT_TOLERANCE: f64 = 1.0e-8;
pub const EXPANSION_LOOP_SELF_CHECK_REACTION_TOLERANCE: f64 = 1.0e-2;

// Witness expected values (anchor-on-pipe, counterclockwise +Z moments) from
// the `Expected Values` tables of the witness note.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExpansionLoopWitnessRow {
    pub in_plane_flexibility_factor: f64,
    pub h_b: f64,
    pub v_b: f64,
    pub m_b: f64,
    pub m_a: f64,
    pub t2_uy_displacement: f64,
}

pub const EXPANSION_LOOP_WITNESS_ROWS: [ExpansionLoopWitnessRow; 4] = [
    ExpansionLoopWitnessRow {
        in_plane_flexibility_factor: 1.0,
        h_b: -1.163612454e4,
        v_b: -7.635238153e3,
        m_b: 1.523056675e4,
        m_a: -2.159843094e4,
        t2_uy_displacement: 6.691964071e-3,
    },
    ExpansionLoopWitnessRow {
        in_plane_flexibility_factor: 5.0,
        h_b: -7.461555977e3,
        v_b: -4.329778531e3,
        m_b: 1.065447422e4,
        m_a: -1.728591674e4,
        t2_uy_displacement: 6.468273963e-3,
    },
    ExpansionLoopWitnessRow {
        in_plane_flexibility_factor: 10.0,
        h_b: -6.144267267e3,
        v_b: -3.301677847e3,
        m_b: 9.245475309e3,
        m_a: -1.589286043e4,
        t2_uy_displacement: 6.417658716e-3,
    },
    ExpansionLoopWitnessRow {
        in_plane_flexibility_factor: 20.0,
        h_b: -5.282037255e3,
        v_b: -2.654674339e3,
        m_b: 8.383839402e3,
        m_a: -1.492493527e4,
        t2_uy_displacement: 6.419030749e-3,
    },
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExpansionLoopCurvedBendThermalResult {
    pub in_plane_flexibility_factor: f64,
    pub h_a: f64,
    pub v_a: f64,
    pub m_a: f64,
    pub h_b: f64,
    pub v_b: f64,
    pub m_b: f64,
    pub t2_ux_displacement: f64,
    pub t2_uy_displacement: f64,
}

pub fn expansion_loop_curved_bend_thermal_fixture() -> MechanicsBenchmark {
    let mut expected_values = vec![ExpectedValue {
        // Exact under the bending-only assumption and k-independent:
        // u_x(T2) = -alpha DeltaT L2.
        name: "t2_ux_displacement",
        value: -expansion_loop_thermal_strain() * EXPANSION_LOOP_LEG2_LENGTH,
        unit: "m",
        dimension: "displacement",
        tolerance_policy: None,
    }];
    for row in EXPANSION_LOOP_WITNESS_ROWS {
        let k_label: &'static str = match row.in_plane_flexibility_factor as u32 {
            1 => "k1",
            5 => "k5",
            10 => "k10",
            _ => "k20",
        };
        let named = |name: &'static str, value: f64, unit: &'static str, dimension| ExpectedValue {
            name,
            value,
            unit,
            dimension,
            tolerance_policy: None,
        };
        let (h_b_name, v_b_name, m_b_name, m_a_name, uy_name): (
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        ) = match k_label {
            "k1" => (
                "h_b_k1",
                "v_b_k1",
                "m_b_k1",
                "m_a_k1",
                "t2_uy_displacement_k1",
            ),
            "k5" => (
                "h_b_k5",
                "v_b_k5",
                "m_b_k5",
                "m_a_k5",
                "t2_uy_displacement_k5",
            ),
            "k10" => (
                "h_b_k10",
                "v_b_k10",
                "m_b_k10",
                "m_a_k10",
                "t2_uy_displacement_k10",
            ),
            _ => (
                "h_b_k20",
                "v_b_k20",
                "m_b_k20",
                "m_a_k20",
                "t2_uy_displacement_k20",
            ),
        };
        expected_values.push(named(h_b_name, row.h_b, "N", "force"));
        expected_values.push(named(v_b_name, row.v_b, "N", "force"));
        expected_values.push(named(m_b_name, row.m_b, "N-m", "moment"));
        expected_values.push(named(m_a_name, row.m_a, "N-m", "moment"));
        expected_values.push(named(uy_name, row.t2_uy_displacement, "m", "displacement"));
    }

    MechanicsBenchmark {
        fixture_id: "MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL",
        family: BenchmarkFamily::CurvedBendExpansionLoop,
        description: "Invented plane expansion-loop (L-bend) thermal benchmark: two straight legs plus one curved-bend macro element, compared for k in {1, 5, 10, 20} against the independent force-method witness MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL with axial rigidity boosted by 1.0e5 to replicate the witness bending-only flexibility assumption.",
        assumptions: &[
            "Anchors A(0, 0) and B(4.5, 3.5) are fully fixed; the loop plane is the global X-Y plane and out-of-plane DOFs are restrained at the tangent points.",
            "Straight legs A->T1 and T2->B are single Euler-Bernoulli frame elements, exact for nodal loading.",
            "The quarter-circle elbow is one curved-bend macro element with center (0.5, 3.0) and user-entered in-plane flexibility factor k (opaque number; no code content).",
            "Uniform-temperature thermal driving uses the exact identity f = K u_free with u_free the similarity-scaling mode alpha DeltaT (p - p_A) and zero rotations.",
            "The witness bending-only flexibility assumption is replicated by scaling all member areas by EXPANSION_LOOP_AXIAL_RIGIDITY_BOOST = 1.0e5 (recorded boost study in the constant comment).",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/expansion_loop_curved_bend_thermal.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values,
    }
}

fn expansion_loop_thermal_strain() -> f64 {
    EXPANSION_LOOP_THERMAL_EXPANSION_COEFFICIENT * EXPANSION_LOOP_TEMPERATURE_RISE
}

// Annulus section properties derived longhand in the witness note:
// A = 5.420270230e-3 m^2, I = 3.018694757e-5 m^4.
fn expansion_loop_section_area_and_second_moment() -> (f64, f64) {
    let outer = EXPANSION_LOOP_OUTER_DIAMETER;
    let inner = outer - 2.0 * EXPANSION_LOOP_WALL_THICKNESS;
    let area = std::f64::consts::PI / 4.0 * (outer * outer - inner * inner);
    let second_moment = std::f64::consts::PI / 64.0 * (outer.powi(4) - inner.powi(4));
    (area, second_moment)
}

fn expansion_loop_nodes() -> Result<[FrameNode; 4], FrameKernelError> {
    Ok([
        FrameNode::new(0, [0.0, 0.0, 0.0])?,
        FrameNode::new(1, [0.0, EXPANSION_LOOP_LEG1_LENGTH, 0.0])?,
        FrameNode::new(
            2,
            [
                EXPANSION_LOOP_BEND_RADIUS,
                EXPANSION_LOOP_LEG1_LENGTH + EXPANSION_LOOP_BEND_RADIUS,
                0.0,
            ],
        )?,
        FrameNode::new(
            3,
            [
                EXPANSION_LOOP_BEND_RADIUS + EXPANSION_LOOP_LEG2_LENGTH,
                EXPANSION_LOOP_LEG1_LENGTH + EXPANSION_LOOP_BEND_RADIUS,
                0.0,
            ],
        )?,
    ])
}

// Global stiffness of the two straight legs plus the curved-bend macro
// element, with all member areas scaled by `axial_rigidity_boost`.
fn expansion_loop_global_stiffness(
    in_plane_flexibility_factor: f64,
    axial_rigidity_boost: f64,
) -> Result<Vec<Vec<f64>>, String> {
    let nodes = expansion_loop_nodes().map_err(|error| error.to_string())?;
    let (area, second_moment) = expansion_loop_section_area_and_second_moment();
    let boosted_area = area * axial_rigidity_boost;
    let torsion_constant = 2.0 * second_moment;
    let section = FrameSection::new(
        EXPANSION_LOOP_ELASTIC_MODULUS,
        EXPANSION_LOOP_SHEAR_MODULUS,
        boosted_area,
        second_moment,
        second_moment,
        torsion_constant,
    )
    .map_err(|error| error.to_string())?;
    let legs = [
        FrameElement::new(nodes[0], nodes[1], section, [1.0, 0.0, 0.0])
            .map_err(|error| error.to_string())?,
        FrameElement::new(nodes[2], nodes[3], section, [0.0, 1.0, 0.0])
            .map_err(|error| error.to_string())?,
    ];
    let mut stiffness =
        assemble_global_stiffness(nodes.len(), &legs).map_err(|error| error.to_string())?;

    let bend = CurvedBendMacroElement::new(
        nodes[1],
        nodes[2],
        [EXPANSION_LOOP_BEND_RADIUS, EXPANSION_LOOP_LEG1_LENGTH, 0.0],
        EXPANSION_LOOP_ELASTIC_MODULUS,
        EXPANSION_LOOP_SHEAR_MODULUS,
        boosted_area,
        second_moment,
        torsion_constant,
        in_plane_flexibility_factor,
        1.0,
    )
    .map_err(|error| error.to_string())?;
    let bend_stiffness = bend.global_stiffness().map_err(|error| error.to_string())?;
    let dof_map = element_dof_map(nodes[1].index, nodes[2].index);
    for row in 0..ELEMENT_DOF {
        for col in 0..ELEMENT_DOF {
            stiffness[dof_map[row]][dof_map[col]] += bend_stiffness[row][col];
        }
    }
    Ok(stiffness)
}

// Free thermal displacement mode of the loop under uniform DeltaT: similarity
// scaling about anchor A with strain alpha DeltaT, zero rotations everywhere.
fn expansion_loop_free_thermal_displacements(nodes: &[FrameNode]) -> Vec<f64> {
    let strain = expansion_loop_thermal_strain();
    let anchor = nodes[0].coordinates;
    let mut displacements = vec![0.0; nodes.len() * DOF_PER_NODE];
    for node in nodes {
        let base = node.index * DOF_PER_NODE;
        displacements[base + UX] = strain * (node.coordinates[0] - anchor[0]);
        displacements[base + UY] = strain * (node.coordinates[1] - anchor[1]);
        displacements[base + UZ] = strain * (node.coordinates[2] - anchor[2]);
    }
    displacements
}

fn matrix_vector_product(matrix: &[Vec<f64>], vector: &[f64]) -> Vec<f64> {
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .zip(vector.iter())
                .map(|(entry, value)| entry * value)
                .sum()
        })
        .collect()
}

fn expansion_loop_solve_with_boost(
    in_plane_flexibility_factor: f64,
    axial_rigidity_boost: f64,
) -> Result<ExpansionLoopCurvedBendThermalResult, String> {
    let nodes = expansion_loop_nodes().map_err(|error| error.to_string())?;
    let stiffness =
        expansion_loop_global_stiffness(in_plane_flexibility_factor, axial_rigidity_boost)?;
    let free_displacements = expansion_loop_free_thermal_displacements(&nodes);
    // Exact uniform-DeltaT equivalent nodal load: f = K u_free.
    let force = matrix_vector_product(&stiffness, &free_displacements);

    // Anchors fully fixed; tangent points restrained out of plane.
    let restrained_dofs = all_node_dofs(0)
        .into_iter()
        .chain(all_node_dofs(3))
        .chain([
            DOF_PER_NODE + UZ,
            DOF_PER_NODE + RX,
            DOF_PER_NODE + RY,
            2 * DOF_PER_NODE + UZ,
            2 * DOF_PER_NODE + RX,
            2 * DOF_PER_NODE + RY,
        ])
        .collect::<Vec<_>>();
    let reduced =
        reduce_system(&stiffness, &force, &restrained_dofs).map_err(|error| error.to_string())?;
    let reduced_displacements =
        solve_dense(&reduced.stiffness, &reduced.force).map_err(|error| error.to_string())?;
    let mut displacements = vec![0.0; nodes.len() * DOF_PER_NODE];
    for (&dof, &value) in reduced.free_dofs.iter().zip(reduced_displacements.iter()) {
        displacements[dof] = value;
    }
    // Anchor-on-pipe reactions at restrained DOFs: r = K u - f.
    let elastic_forces = matrix_vector_product(&stiffness, &displacements);
    let reactions = elastic_forces
        .iter()
        .zip(force.iter())
        .map(|(elastic, applied)| elastic - applied)
        .collect::<Vec<_>>();

    Ok(ExpansionLoopCurvedBendThermalResult {
        in_plane_flexibility_factor,
        h_a: reactions[UX],
        v_a: reactions[UY],
        m_a: reactions[RZ],
        h_b: reactions[3 * DOF_PER_NODE + UX],
        v_b: reactions[3 * DOF_PER_NODE + UY],
        m_b: reactions[3 * DOF_PER_NODE + RZ],
        t2_ux_displacement: displacements[2 * DOF_PER_NODE + UX],
        t2_uy_displacement: displacements[2 * DOF_PER_NODE + UY],
    })
}

pub fn solve_expansion_loop_curved_bend_thermal(
    in_plane_flexibility_factor: f64,
) -> Result<ExpansionLoopCurvedBendThermalResult, String> {
    expansion_loop_solve_with_boost(
        in_plane_flexibility_factor,
        EXPANSION_LOOP_AXIAL_RIGIDITY_BOOST,
    )
}

/// Self-check of the thermal-driving identity f = K u_free: with only
/// rigid-body-blocking restraints (anchor A fixed, anchor B released) the
/// solved displacements must reproduce the free similarity-scaling mode and
/// every reaction must vanish at round-off. Returns the maximum displacement
/// deviation (m) and maximum reaction magnitude (N or N-m) over the sweep.
pub fn expansion_loop_free_expansion_self_check() -> Result<(f64, f64), String> {
    let nodes = expansion_loop_nodes().map_err(|error| error.to_string())?;
    let mut max_displacement_deviation = 0.0_f64;
    let mut max_reaction_magnitude = 0.0_f64;
    for in_plane_flexibility_factor in EXPANSION_LOOP_FLEXIBILITY_FACTORS {
        let stiffness = expansion_loop_global_stiffness(
            in_plane_flexibility_factor,
            EXPANSION_LOOP_AXIAL_RIGIDITY_BOOST,
        )?;
        let free_displacements = expansion_loop_free_thermal_displacements(&nodes);
        let force = matrix_vector_product(&stiffness, &free_displacements);
        let restrained_dofs = all_node_dofs(0);
        let reduced = reduce_system(&stiffness, &force, &restrained_dofs)
            .map_err(|error| error.to_string())?;
        let reduced_displacements =
            solve_dense(&reduced.stiffness, &reduced.force).map_err(|error| error.to_string())?;
        let mut displacements = vec![0.0; nodes.len() * DOF_PER_NODE];
        for (&dof, &value) in reduced.free_dofs.iter().zip(reduced_displacements.iter()) {
            displacements[dof] = value;
        }
        for (solved, free) in displacements.iter().zip(free_displacements.iter()) {
            max_displacement_deviation = max_displacement_deviation.max((solved - free).abs());
        }
        let elastic_forces = matrix_vector_product(&stiffness, &displacements);
        for (elastic, applied) in elastic_forces.iter().zip(force.iter()) {
            max_reaction_magnitude = max_reaction_magnitude.max((elastic - applied).abs());
        }
    }
    Ok((max_displacement_deviation, max_reaction_magnitude))
}

fn expansion_loop_close(actual: f64, expected: f64, relative_tolerance: f64) -> bool {
    (actual - expected).abs() <= relative_tolerance * expected.abs().max(1.0e-5)
}

pub fn validate_expansion_loop_curved_bend_thermal() -> bool {
    EXPANSION_LOOP_WITNESS_ROWS.iter().all(|row| {
        let Ok(result) = solve_expansion_loop_curved_bend_thermal(row.in_plane_flexibility_factor)
        else {
            return false;
        };
        expansion_loop_close(
            result.h_b,
            row.h_b,
            EXPANSION_LOOP_REACTION_RELATIVE_TOLERANCE,
        ) && expansion_loop_close(
            result.v_b,
            row.v_b,
            EXPANSION_LOOP_REACTION_RELATIVE_TOLERANCE,
        ) && expansion_loop_close(
            result.m_b,
            row.m_b,
            EXPANSION_LOOP_REACTION_RELATIVE_TOLERANCE,
        ) && expansion_loop_close(
            result.m_a,
            row.m_a,
            EXPANSION_LOOP_REACTION_RELATIVE_TOLERANCE,
        ) && expansion_loop_close(
            result.h_a,
            -row.h_b,
            EXPANSION_LOOP_REACTION_RELATIVE_TOLERANCE,
        ) && expansion_loop_close(
            result.v_a,
            -row.v_b,
            EXPANSION_LOOP_REACTION_RELATIVE_TOLERANCE,
        ) && expansion_loop_close(
            result.t2_uy_displacement,
            row.t2_uy_displacement,
            EXPANSION_LOOP_DISPLACEMENT_RELATIVE_TOLERANCE,
        ) && expansion_loop_close(
            result.t2_ux_displacement,
            -expansion_loop_thermal_strain() * EXPANSION_LOOP_LEG2_LENGTH,
            EXPANSION_LOOP_UX_T2_RELATIVE_TOLERANCE,
        )
    })
}

// ---------------------------------------------------------------------------
// MECH-CURVED-BEND-DISTRIBUTED-FIXED-END
//
// Arc-consistent distributed-load and interior-station benchmark against the
// independent hand-calculated force-method reference
// `validation/hand_calcs/mechanics/curved_bend_distributed_load_fixed_end.md`
// (D-34 / DEC-070 arc-residual closure evidence). A quarter-circle arc,
// clamped at both ends, carries a uniform in-plane or out-of-plane load; the
// comparison side forms the consistent equivalent nodal loads and section
// resultants directly on the curved-bend macro-element crate. All numeric
// inputs below are invented fixture values transcribed from the witness note;
// the bending flexibility factor `k` is an opaque user-entered number.
// ---------------------------------------------------------------------------

// Invented witness inputs (units per PKG09-FIXTURE-UNITS-EXPLICIT-N-M-RAD-K).
const CBDFE_BEND_RADIUS: f64 = 1.2;
const CBDFE_OUTER_DIAMETER: f64 = 0.1683;
const CBDFE_WALL_THICKNESS: f64 = 0.0071;
const CBDFE_ELASTIC_MODULUS: f64 = 200.0e9;
const CBDFE_SHEAR_MODULUS: f64 = 80.0e9;
const CBDFE_IN_PLANE_INTENSITY: f64 = -1500.0; // local -y, N per m of arc
const CBDFE_OUT_OF_PLANE_INTENSITY: f64 = -800.0; // local -z, N per m of arc
pub const CBDFE_FLEXIBILITY_FACTORS: [f64; 2] = [1.0, 2.0];
pub const CBDFE_STATION_FRACTIONS: [f64; 3] = [0.25, 0.5, 0.75];
// Comparison tolerance: both sides are closed-form (force method on the same
// strain-energy model), so the DEC-026 analytic-class 1.0e-9 relative tier
// applies with a near-zero absolute scale floor of 1.0e-3 (N / N-m; the
// exact-zero midspan torsion row). Measured max deviation is recorded by
// `curved_bend_distributed_fixture_matches_witness_reference_table`.
const CBDFE_RELATIVE_TOLERANCE: f64 = 1.0e-9;
const CBDFE_NEAR_ZERO_SCALE_FLOOR: f64 = 1.0e-3;

// Witness expected values (support-on-element reactions in the arc local
// frame; interior stations in the arc section frame) from the witness note
// `Results` tables.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurvedBendDistributedWitnessRow {
    pub flexibility_factor: f64,
    pub in_plane_tip: [f64; 3], // u_x [m], u_y [m], theta_z [rad]
    pub in_plane_b: [f64; 3],   // F_x, F_y [N], M_z [N m] at B
    pub in_plane_a: [f64; 3],   // F_x, F_y [N], M_z [N m] at A
    pub in_plane_station_mz: [f64; 3],
    pub out_of_plane_tip: [f64; 3], // u_z [m], theta_x, theta_y [rad]
    pub out_of_plane_b: [f64; 3],   // F_z [N], M_x, M_y [N m] at B
    pub out_of_plane_a: [f64; 3],   // F_z [N], M_x, M_y [N m] at A
    pub out_of_plane_station_torsion: [f64; 3],
    pub out_of_plane_station_bending_y: [f64; 3],
}

pub const CBDFE_WITNESS_ROWS: [CurvedBendDistributedWitnessRow; 2] = [
    CurvedBendDistributedWitnessRow {
        flexibility_factor: 1.0,
        in_plane_tip: [-3.325456995161e-4, -4.901547357913e-4, 4.753498913120e-4],
        in_plane_b: [518.6887093849, 675.8314673820, 152.2772588244],
        in_plane_a: [-518.6887093849, 2151.6019208490, 48.2268874190],
        in_plane_station_mz: [36.3621332981, -36.5602035248, -45.2741760268],
        out_of_plane_tip: [-4.987403008669e-4, -2.430655343790e-4, -2.852099347872e-4],
        out_of_plane_b: [753.9822368616, -15.0729196640, -262.2942354301],
        out_of_plane_a: [753.9822368616, 262.2942354301, 15.0729196640],
        out_of_plane_station_torsion: [29.1159479459, 0.0, -29.1159479459],
        out_of_plane_station_bending_y: [-10.4566016240, -106.2339587762, -10.4566016240],
    },
    CurvedBendDistributedWitnessRow {
        flexibility_factor: 2.0,
        in_plane_tip: [-6.662709328105e-4, -9.777057497307e-4, 9.506997826241e-4],
        in_plane_b: [603.4037708891, 589.0892325630, 122.9516066821],
        in_plane_a: [-603.4037708891, 2238.3441556680, 75.1199315834],
        in_plane_station_mz: [40.4485183865, -22.0575891752, -42.5043088892],
        out_of_plane_tip: [-8.531453880460e-4, -5.384031070283e-4, -4.119699058038e-4],
        out_of_plane_b: [753.9822368616, -12.2125547407, -259.4338705068],
        out_of_plane_a: [753.9822368616, 259.4338705068, 12.2125547407],
        out_of_plane_station_torsion: [30.6639662874, 0.0, -30.6639662874],
        out_of_plane_station_bending_y: [-14.1938484988, -110.2791256441, -14.1938484988],
    },
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurvedBendDistributedLoadCase {
    InPlane,
    OutOfPlane,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurvedBendDistributedFixedEndResult {
    pub flexibility_factor: f64,
    /// Free-tip displacements of the anchored arc solved under the consistent
    /// load vector (all six local DOFs at node B).
    pub tip_displacements: [f64; 6],
    /// Clamped-end support-on-element reactions at A and B (local frame;
    /// with both ends clamped these are the negated equivalent nodal loads).
    pub reactions_a: [f64; 6],
    pub reactions_b: [f64; 6],
    /// Interior-station section-frame resultants at the witness fractions,
    /// clamped-clamped state: [axial, shear_y, shear_z, torsion, bending_y,
    /// bending_z] per station.
    pub stations: [[f64; 6]; 3],
}

fn cbdfe_section_area_and_second_moment() -> (f64, f64) {
    let outer = CBDFE_OUTER_DIAMETER;
    let inner = outer - 2.0 * CBDFE_WALL_THICKNESS;
    let area = std::f64::consts::PI / 4.0 * (outer * outer - inner * inner);
    let second_moment = std::f64::consts::PI / 64.0 * (outer.powi(4) - inner.powi(4));
    (area, second_moment)
}

fn cbdfe_element(flexibility_factor: f64) -> Result<CurvedBendMacroElement, String> {
    let radius = CBDFE_BEND_RADIUS;
    let node_a = FrameNode::new(0, [radius, 0.0, 0.0]).map_err(|error| error.to_string())?;
    let node_b = FrameNode::new(1, [0.0, radius, 0.0]).map_err(|error| error.to_string())?;
    let (area, second_moment) = cbdfe_section_area_and_second_moment();
    CurvedBendMacroElement::new(
        node_a,
        node_b,
        [0.0, 0.0, 0.0],
        CBDFE_ELASTIC_MODULUS,
        CBDFE_SHEAR_MODULUS,
        area,
        second_moment,
        2.0 * second_moment,
        flexibility_factor,
        flexibility_factor,
    )
    .map_err(|error| error.to_string())
}

fn cbdfe_intensity(case: CurvedBendDistributedLoadCase) -> [f64; 3] {
    // The arc local frame coincides with the global frame in this fixture
    // (center at the origin, node A on +x, bend plane x-y).
    match case {
        CurvedBendDistributedLoadCase::InPlane => [0.0, CBDFE_IN_PLANE_INTENSITY, 0.0],
        CurvedBendDistributedLoadCase::OutOfPlane => [0.0, 0.0, CBDFE_OUT_OF_PLANE_INTENSITY],
    }
}

pub fn solve_curved_bend_distributed_fixed_end(
    flexibility_factor: f64,
    case: CurvedBendDistributedLoadCase,
) -> Result<CurvedBendDistributedFixedEndResult, String> {
    let element = cbdfe_element(flexibility_factor)?;
    let intensity = cbdfe_intensity(case);
    let equivalent = element
        .consistent_uniform_nodal_loads(intensity)
        .map_err(|error| error.to_string())?;

    // Clamped-clamped state: zero displacements, so the support-on-element
    // reactions are exactly the negated consistent equivalent nodal loads.
    let mut reactions_a = [0.0; 6];
    let mut reactions_b = [0.0; 6];
    for dof in 0..DOF_PER_NODE {
        reactions_a[dof] = -equivalent[dof];
        reactions_b[dof] = -equivalent[DOF_PER_NODE + dof];
    }

    // Anchored-at-A free-tip solve under the same consistent load vector.
    let stiffness = element
        .global_stiffness()
        .map_err(|error| error.to_string())?;
    let dense: Vec<Vec<f64>> = stiffness.iter().map(|row| row.to_vec()).collect();
    let restrained: Vec<usize> = (0..DOF_PER_NODE).collect();
    let reduced =
        reduce_system(&dense, &equivalent, &restrained).map_err(|error| error.to_string())?;
    let solution =
        solve_dense(&reduced.stiffness, &reduced.force).map_err(|error| error.to_string())?;
    let mut tip_displacements = [0.0; DOF_PER_NODE];
    tip_displacements.copy_from_slice(&solution);

    // Interior stations of the clamped-clamped arc: the node-B end force is
    // the clamped redundant, and the section resultants follow from segment
    // equilibrium along the arc.
    let mut stations = [[0.0; 6]; 3];
    for (station, fraction) in stations.iter_mut().zip(CBDFE_STATION_FRACTIONS) {
        *station = element
            .arc_section_resultants(fraction, reactions_b, intensity)
            .map_err(|error| error.to_string())?;
    }

    Ok(CurvedBendDistributedFixedEndResult {
        flexibility_factor,
        tip_displacements,
        reactions_a,
        reactions_b,
        stations,
    })
}

fn cbdfe_close(actual: f64, expected: f64) -> bool {
    (actual - expected).abs()
        <= CBDFE_RELATIVE_TOLERANCE * expected.abs().max(CBDFE_NEAR_ZERO_SCALE_FLOOR)
}

/// Maximum deviation of the solved fixture against the witness table,
/// normalized per compared quantity by the DEC-026 relative+floor pair.
pub fn curved_bend_distributed_fixed_end_max_normalized_deviation() -> Result<f64, String> {
    let mut max_normalized = 0.0_f64;
    let mut check = |actual: f64, expected: f64| {
        let scale = expected.abs().max(CBDFE_NEAR_ZERO_SCALE_FLOOR);
        max_normalized = max_normalized.max((actual - expected).abs() / scale);
    };
    for row in CBDFE_WITNESS_ROWS {
        let in_plane = solve_curved_bend_distributed_fixed_end(
            row.flexibility_factor,
            CurvedBendDistributedLoadCase::InPlane,
        )?;
        check(in_plane.tip_displacements[UX], row.in_plane_tip[0]);
        check(in_plane.tip_displacements[UY], row.in_plane_tip[1]);
        check(in_plane.tip_displacements[RZ], row.in_plane_tip[2]);
        check(in_plane.reactions_b[UX], row.in_plane_b[0]);
        check(in_plane.reactions_b[UY], row.in_plane_b[1]);
        check(in_plane.reactions_b[RZ], row.in_plane_b[2]);
        check(in_plane.reactions_a[UX], row.in_plane_a[0]);
        check(in_plane.reactions_a[UY], row.in_plane_a[1]);
        check(in_plane.reactions_a[RZ], row.in_plane_a[2]);
        for (station, expected) in in_plane.stations.iter().zip(row.in_plane_station_mz) {
            check(station[5], expected);
        }
        let out_of_plane = solve_curved_bend_distributed_fixed_end(
            row.flexibility_factor,
            CurvedBendDistributedLoadCase::OutOfPlane,
        )?;
        check(out_of_plane.tip_displacements[UZ], row.out_of_plane_tip[0]);
        check(out_of_plane.tip_displacements[RX], row.out_of_plane_tip[1]);
        check(out_of_plane.tip_displacements[RY], row.out_of_plane_tip[2]);
        check(out_of_plane.reactions_b[UZ], row.out_of_plane_b[0]);
        check(out_of_plane.reactions_b[RX], row.out_of_plane_b[1]);
        check(out_of_plane.reactions_b[RY], row.out_of_plane_b[2]);
        check(out_of_plane.reactions_a[UZ], row.out_of_plane_a[0]);
        check(out_of_plane.reactions_a[RX], row.out_of_plane_a[1]);
        check(out_of_plane.reactions_a[RY], row.out_of_plane_a[2]);
        for (station, expected) in out_of_plane
            .stations
            .iter()
            .zip(row.out_of_plane_station_torsion)
        {
            check(station[3], expected);
        }
        for (station, expected) in out_of_plane
            .stations
            .iter()
            .zip(row.out_of_plane_station_bending_y)
        {
            check(station[4], expected);
        }
    }
    Ok(max_normalized)
}

pub fn validate_curved_bend_distributed_fixed_end() -> bool {
    matches!(
        curved_bend_distributed_fixed_end_max_normalized_deviation(),
        Ok(deviation) if deviation <= CBDFE_RELATIVE_TOLERANCE
    )
}

pub fn curved_bend_distributed_fixed_end_fixture() -> MechanicsBenchmark {
    let mut expected_values = Vec::new();
    for row in CBDFE_WITNESS_ROWS {
        let (
            labels_in_b,
            labels_in_a,
            labels_in_st,
            labels_out_b,
            labels_out_a,
            labels_out_t,
            labels_out_by,
        ): (
            [&'static str; 3],
            [&'static str; 3],
            [&'static str; 3],
            [&'static str; 3],
            [&'static str; 3],
            [&'static str; 3],
            [&'static str; 3],
        ) = if row.flexibility_factor as u32 == 1 {
            (
                ["in_plane_b_fx_k1", "in_plane_b_fy_k1", "in_plane_b_mz_k1"],
                ["in_plane_a_fx_k1", "in_plane_a_fy_k1", "in_plane_a_mz_k1"],
                [
                    "in_plane_station_mz_q1_k1",
                    "in_plane_station_mz_mid_k1",
                    "in_plane_station_mz_q3_k1",
                ],
                [
                    "out_of_plane_b_fz_k1",
                    "out_of_plane_b_mx_k1",
                    "out_of_plane_b_my_k1",
                ],
                [
                    "out_of_plane_a_fz_k1",
                    "out_of_plane_a_mx_k1",
                    "out_of_plane_a_my_k1",
                ],
                [
                    "out_of_plane_station_torsion_q1_k1",
                    "out_of_plane_station_torsion_mid_k1",
                    "out_of_plane_station_torsion_q3_k1",
                ],
                [
                    "out_of_plane_station_bending_y_q1_k1",
                    "out_of_plane_station_bending_y_mid_k1",
                    "out_of_plane_station_bending_y_q3_k1",
                ],
            )
        } else {
            (
                ["in_plane_b_fx_k2", "in_plane_b_fy_k2", "in_plane_b_mz_k2"],
                ["in_plane_a_fx_k2", "in_plane_a_fy_k2", "in_plane_a_mz_k2"],
                [
                    "in_plane_station_mz_q1_k2",
                    "in_plane_station_mz_mid_k2",
                    "in_plane_station_mz_q3_k2",
                ],
                [
                    "out_of_plane_b_fz_k2",
                    "out_of_plane_b_mx_k2",
                    "out_of_plane_b_my_k2",
                ],
                [
                    "out_of_plane_a_fz_k2",
                    "out_of_plane_a_mx_k2",
                    "out_of_plane_a_my_k2",
                ],
                [
                    "out_of_plane_station_torsion_q1_k2",
                    "out_of_plane_station_torsion_mid_k2",
                    "out_of_plane_station_torsion_q3_k2",
                ],
                [
                    "out_of_plane_station_bending_y_q1_k2",
                    "out_of_plane_station_bending_y_mid_k2",
                    "out_of_plane_station_bending_y_q3_k2",
                ],
            )
        };
        let force_dims: [&'static str; 3] = ["force", "force", "moment"];
        let force_units: [&'static str; 3] = ["N", "N", "N-m"];
        let out_dims: [&'static str; 3] = ["force", "moment", "moment"];
        let out_units: [&'static str; 3] = ["N", "N-m", "N-m"];
        for slot in 0..3 {
            expected_values.push(ExpectedValue {
                name: labels_in_b[slot],
                value: row.in_plane_b[slot],
                unit: force_units[slot],
                dimension: force_dims[slot],
                tolerance_policy: None,
            });
            expected_values.push(ExpectedValue {
                name: labels_in_a[slot],
                value: row.in_plane_a[slot],
                unit: force_units[slot],
                dimension: force_dims[slot],
                tolerance_policy: None,
            });
            expected_values.push(ExpectedValue {
                name: labels_in_st[slot],
                value: row.in_plane_station_mz[slot],
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            });
            expected_values.push(ExpectedValue {
                name: labels_out_b[slot],
                value: row.out_of_plane_b[slot],
                unit: out_units[slot],
                dimension: out_dims[slot],
                tolerance_policy: None,
            });
            expected_values.push(ExpectedValue {
                name: labels_out_a[slot],
                value: row.out_of_plane_a[slot],
                unit: out_units[slot],
                dimension: out_dims[slot],
                tolerance_policy: None,
            });
            expected_values.push(ExpectedValue {
                name: labels_out_t[slot],
                value: row.out_of_plane_station_torsion[slot],
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            });
            expected_values.push(ExpectedValue {
                name: labels_out_by[slot],
                value: row.out_of_plane_station_bending_y[slot],
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            });
        }
    }
    MechanicsBenchmark {
        fixture_id: "MECH-CURVED-BEND-DISTRIBUTED-FIXED-END",
        family: BenchmarkFamily::CurvedBendDistributedLoad,
        description: "Invented clamped-clamped quarter-circle arc under uniform in-plane and out-of-plane distributed loads, compared for k in {1, 2} against the independent force-method witness MECH-CURVED-BEND-DISTRIBUTED-FIXED-END: arc-consistent fixed-end forces/moments and interior-station section resultants from the curved-bend macro-element closed forms.",
        assumptions: &[
            "The arc center sits at the origin with node A at (R, 0, 0) and node B at (0, R, 0); the arc local frame coincides with the global frame.",
            "Both ends are fully clamped for the reaction and station tables; the free-tip deflection rows anchor A only and solve under the same consistent load vector.",
            "The user-entered bending flexibility factor k (opaque number; no code content) applies to both bending planes, matching the product single-factor mapping.",
            "The strain-energy model is Euler-Bernoulli bending + St. Venant torsion + axial stretch, identical on both comparison sides; shear deformation is excluded.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/curved_bend_distributed_load_fixed_end.md",
        ),
        unit_basis: FIXTURE_UNIT_BASIS,
        expected_values,
    }
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

pub fn validate_branch_assembly_benchmark() -> bool {
    let Ok(result) = solve_branch_assembly_benchmark() else {
        return false;
    };
    let Ok(expected) = expected_branch_assembly_result() else {
        return false;
    };

    [
        result.branch_axial_stiffness,
        result.header_lateral_stiffness,
        result.junction_uy_displacement,
        result.branch_tip_uy_displacement,
        result.branch_axial_extension,
        result.header_left_uy_reaction,
        result.header_right_uy_reaction,
        result.header_reaction_sum,
    ]
    .into_iter()
    .all(f64::is_finite)
        && (result.branch_axial_stiffness - expected.branch_axial_stiffness).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (result.header_lateral_stiffness - expected.header_lateral_stiffness).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (result.junction_uy_displacement - expected.junction_uy_displacement).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (result.branch_tip_uy_displacement - expected.branch_tip_uy_displacement).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (result.branch_axial_extension - expected.branch_axial_extension).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (result.header_left_uy_reaction - expected.header_left_uy_reaction).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (result.header_right_uy_reaction - expected.header_right_uy_reaction).abs()
            <= INTERNAL_ASSERTION_EPSILON
        && (result.header_reaction_sum - expected.header_reaction_sum).abs()
            <= INTERNAL_ASSERTION_EPSILON
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

fn branch_assembly_nodes() -> Result<[FrameNode; 4], FrameKernelError> {
    Ok([
        FrameNode::new(0, [-2.0, 0.0, 0.0])?,
        FrameNode::new(1, [0.0, 0.0, 0.0])?,
        FrameNode::new(2, [3.0, 0.0, 0.0])?,
        FrameNode::new(3, [0.0, 4.0, 0.0])?,
    ])
}

fn branch_assembly_tip_load() -> f64 {
    90.0
}

fn expected_branch_assembly_result() -> Result<BranchAssemblyBenchmarkResult, FrameKernelError> {
    let section = benchmark_section()?;
    let left_header_length: f64 = 2.0;
    let right_header_length: f64 = 3.0;
    let branch_length: f64 = 4.0;
    let left_header_stiffness =
        12.0 * section.elastic_modulus * section.second_moment_z / left_header_length.powi(3);
    let right_header_stiffness =
        12.0 * section.elastic_modulus * section.second_moment_z / right_header_length.powi(3);
    let header_lateral_stiffness = left_header_stiffness + right_header_stiffness;
    let branch_axial_stiffness = section.elastic_modulus * section.area / branch_length;
    let tip_load = branch_assembly_tip_load();
    let junction_uy_displacement = tip_load / header_lateral_stiffness;
    let branch_axial_extension = tip_load / branch_axial_stiffness;
    let branch_tip_uy_displacement = junction_uy_displacement + branch_axial_extension;
    let header_left_uy_reaction = -left_header_stiffness * junction_uy_displacement;
    let header_right_uy_reaction = -right_header_stiffness * junction_uy_displacement;

    Ok(BranchAssemblyBenchmarkResult {
        branch_axial_stiffness,
        header_lateral_stiffness,
        junction_uy_displacement,
        branch_tip_uy_displacement,
        branch_axial_extension,
        header_left_uy_reaction,
        header_right_uy_reaction,
        header_reaction_sum: header_left_uy_reaction + header_right_uy_reaction,
    })
}

fn branch_assembly_restrained_dofs() -> Vec<usize> {
    let mut restrained = all_node_dofs(0);
    restrained.extend(all_node_dofs(2));
    restrained.extend([
        DOF_PER_NODE + UX,
        DOF_PER_NODE + UZ,
        DOF_PER_NODE + RX,
        DOF_PER_NODE + RY,
        DOF_PER_NODE + RZ,
    ]);
    restrained.extend([
        3 * DOF_PER_NODE + UX,
        3 * DOF_PER_NODE + UZ,
        3 * DOF_PER_NODE + RX,
        3 * DOF_PER_NODE + RY,
        3 * DOF_PER_NODE + RZ,
    ]);
    restrained
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
        assert_eq!(fixtures.len(), 21);
        assert!(fixtures
            .iter()
            .any(|fixture| fixture.fixture_id == "MECH-BRANCH-ASSEMBLY-THREE-MEMBER"));
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
    fn occloadgen_generation_matches_witness_intensities() {
        let fixture = tp_pmm_p3_occloadgen_equivalent_static_fixture();
        let expected = |name: &str| {
            fixture
                .expected_values
                .iter()
                .find(|value| value.name == name)
                .unwrap_or_else(|| panic!("missing expected value {name}"))
                .value
        };

        let masses = [ElementMassPerLength {
            element_index: 0,
            mass_per_length: occloadgen_mass_per_length(),
        }];
        let (seismic_loads, seismic_findings) =
            generate_seismic_equivalent_static_loads(&occloadgen_seismic_basis(), &masses);
        assert!(seismic_findings.is_empty());
        assert_eq!(seismic_loads.len(), 2);
        let seismic_x = seismic_loads
            .iter()
            .find(|load| load.direction == LoadDirection::GlobalX)
            .unwrap();
        let seismic_z = seismic_loads
            .iter()
            .find(|load| load.direction == LoadDirection::GlobalZ)
            .unwrap();
        assert!(
            (seismic_x.magnitude.unwrap().value - expected("seismic_intensity_global_x")).abs()
                <= 1.0e-9
        );
        assert!(
            (seismic_z.magnitude.unwrap().value - expected("seismic_intensity_global_z")).abs()
                <= 1.0e-9
        );
        assert!(seismic_loads
            .iter()
            .all(|load| load.category == PrimitiveLoadCategory::Seismic));

        let exposed = [ElementExposedDiameter {
            element_index: 0,
            exposed_diameter: occloadgen_exposed_diameter(),
        }];
        let (wind_loads, wind_findings) =
            generate_wind_equivalent_static_loads(&occloadgen_wind_basis(), &exposed);
        assert!(wind_findings.is_empty());
        assert_eq!(wind_loads.len(), 1);
        assert_eq!(wind_loads[0].category, PrimitiveLoadCategory::Wind);
        assert!(
            (wind_loads[0].magnitude.unwrap().value - expected("wind_intensity_global_y")).abs()
                <= 1.0e-9
        );
    }

    #[test]
    fn occloadgen_generated_loads_pass_boundary_and_lump_to_end_nodes() {
        let fixture = tp_pmm_p3_occloadgen_equivalent_static_fixture();
        let expected = |name: &str| {
            fixture
                .expected_values
                .iter()
                .find(|value| value.name == name)
                .unwrap_or_else(|| panic!("missing expected value {name}"))
                .value
        };

        let masses = [ElementMassPerLength {
            element_index: 0,
            mass_per_length: occloadgen_mass_per_length(),
        }];
        let (mut loads, findings) =
            generate_seismic_equivalent_static_loads(&occloadgen_seismic_basis(), &masses);
        assert!(findings.is_empty());
        let exposed = [ElementExposedDiameter {
            element_index: 0,
            exposed_diameter: occloadgen_exposed_diameter(),
        }];
        let (wind_loads, wind_findings) =
            generate_wind_equivalent_static_loads(&occloadgen_wind_basis(), &exposed);
        assert!(wind_findings.is_empty());
        loads.extend(wind_loads);

        let basis = EquivalentStaticMechanicsBasis::new(
            "basis:occloadgen-equivalent-static",
            "provenance:user-input",
        )
        .unwrap();
        let application = prepare_equivalent_static_loads(2, 1, &basis, &loads).unwrap();
        assert!(application.findings.is_empty());
        assert_eq!(application.element_uniform_loads.len(), 3);

        let span = ElementLoadSpan::new(0, 0, 1, OCCLOADGEN_SPAN_LENGTH).unwrap();
        let lumped = prepare_lumped_nodal_loads(2, 1, &[span], &loads);
        assert!(lumped.findings.is_empty());
        let sum_for = |node_index: usize, dof: usize| -> f64 {
            lumped
                .nodal_loads
                .iter()
                .filter(|load| load.node_index == node_index && load.global_dof % 6 == dof)
                .map(|load| load.value)
                .sum()
        };
        let expected_x = expected("seismic_lumped_end_force_global_x");
        let expected_y = expected("wind_lumped_end_force_global_y");
        assert!((sum_for(0, UX) - expected_x).abs() <= 1.0e-9);
        assert!((sum_for(1, UX) - expected_x).abs() <= 1.0e-9);
        assert!((sum_for(0, UY) - expected_y).abs() <= 1.0e-9);
        assert!((sum_for(1, UY) - expected_y).abs() <= 1.0e-9);
    }

    #[test]
    fn occloadgen_generation_blocks_without_user_inputs_or_marked_spans() {
        let (loads, findings) =
            generate_seismic_equivalent_static_loads(&occloadgen_seismic_basis(), &[]);
        assert!(loads.is_empty());
        assert!(!findings.is_empty());

        let (loads, findings) =
            generate_wind_equivalent_static_loads(&occloadgen_wind_basis(), &[]);
        assert!(loads.is_empty());
        assert!(!findings.is_empty());
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
    fn readiness_metadata_matches_documented_boundaries() {
        let fixtures = fixture_inventory();
        let mut ids = fixture_inventory_ids();
        ids.sort_unstable();
        ids.dedup();

        assert_eq!(ids.len(), fixtures.len(), "fixture IDs must be unique");
        assert!(readiness_boundaries_are_documented());

        for fixture in &fixtures {
            assert!(
                BENCHMARK_README.contains(fixture.fixture_id),
                "{} missing from benchmark README fixture inventory",
                fixture.fixture_id
            );

            let hand_calc_note = fixture
                .provenance
                .source_location
                .rsplit('/')
                .next()
                .expect("fixture provenance source must include a file name");
            assert!(
                HAND_CALC_README.contains(fixture.fixture_id)
                    && HAND_CALC_README.contains(hand_calc_note),
                "{} missing hand-calculation README inventory entry for {}",
                fixture.fixture_id,
                hand_calc_note
            );
            assert_eq!(fixture.unit_basis, FIXTURE_UNIT_BASIS);
            assert!(fixture.tolerance_policy_is_unresolved());
            assert!(fixture.provenance.is_publicly_usable());
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
    fn branch_assembly_fixture_matches_open_stiffness_network() {
        let fixture = branch_assembly_fixture();
        assert_eq!(fixture.fixture_id, "MECH-BRANCH-ASSEMBLY-THREE-MEMBER");
        let result = solve_branch_assembly_benchmark().unwrap();
        assert!(
            validate_branch_assembly_benchmark(),
            "unexpected branch assembly mechanics result: {result:?}"
        );
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
    fn expansion_loop_fixture_matches_witness_reference_table() {
        let fixture = expansion_loop_curved_bend_thermal_fixture();
        assert_eq!(
            fixture.fixture_id,
            "MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL"
        );
        assert!(fixture.provenance.is_publicly_usable());
        assert!(fixture.tolerance_policy_is_unresolved());
        assert!(fixture.has_dimensioned_expected_values());
        assert_eq!(fixture.expected_values.len(), 21);
        assert!(BENCHMARK_README.contains(fixture.fixture_id));

        for row in EXPANSION_LOOP_WITNESS_ROWS {
            let result =
                solve_expansion_loop_curved_bend_thermal(row.in_plane_flexibility_factor).unwrap();
            for (name, actual, expected) in [
                ("H_B", result.h_b, row.h_b),
                ("V_B", result.v_b, row.v_b),
                ("M_B", result.m_b, row.m_b),
                ("M_A", result.m_a, row.m_a),
                ("H_A", result.h_a, -row.h_b),
                ("V_A", result.v_a, -row.v_b),
            ] {
                assert!(
                    expansion_loop_close(
                        actual,
                        expected,
                        EXPANSION_LOOP_REACTION_RELATIVE_TOLERANCE
                    ),
                    "{name} at k = {}: solver {actual} vs witness {expected}",
                    row.in_plane_flexibility_factor
                );
            }
            assert!(
                expansion_loop_close(
                    result.t2_uy_displacement,
                    row.t2_uy_displacement,
                    EXPANSION_LOOP_DISPLACEMENT_RELATIVE_TOLERANCE
                ),
                "u_y(T2) at k = {}: solver {} vs witness {}",
                row.in_plane_flexibility_factor,
                result.t2_uy_displacement,
                row.t2_uy_displacement
            );
        }
        assert!(validate_expansion_loop_curved_bend_thermal());
    }

    #[test]
    fn expansion_loop_free_expansion_identity_is_stress_free() {
        let (max_displacement_deviation, max_reaction_magnitude) =
            expansion_loop_free_expansion_self_check().unwrap();
        assert!(
            max_displacement_deviation <= EXPANSION_LOOP_SELF_CHECK_DISPLACEMENT_TOLERANCE,
            "free-expansion displacement deviation {max_displacement_deviation} exceeds tolerance"
        );
        assert!(
            max_reaction_magnitude <= EXPANSION_LOOP_SELF_CHECK_REACTION_TOLERANCE,
            "free-expansion reaction magnitude {max_reaction_magnitude} exceeds tolerance"
        );
    }

    #[test]
    fn expansion_loop_t2_x_displacement_is_k_independent_free_shortening() {
        let expected = -expansion_loop_thermal_strain() * EXPANSION_LOOP_LEG2_LENGTH;
        for in_plane_flexibility_factor in EXPANSION_LOOP_FLEXIBILITY_FACTORS {
            let result =
                solve_expansion_loop_curved_bend_thermal(in_plane_flexibility_factor).unwrap();
            assert!(
                expansion_loop_close(
                    result.t2_ux_displacement,
                    expected,
                    EXPANSION_LOOP_UX_T2_RELATIVE_TOLERANCE
                ),
                "u_x(T2) at k = {in_plane_flexibility_factor}: solver {} vs exact {expected}",
                result.t2_ux_displacement
            );
        }
    }

    #[test]
    fn expansion_loop_k_sweep_reactions_decrease_strictly_monotonically() {
        let results = EXPANSION_LOOP_FLEXIBILITY_FACTORS
            .map(|factor| solve_expansion_loop_curved_bend_thermal(factor).unwrap());
        for pair in results.windows(2) {
            assert!(
                pair[1].m_a.abs() < pair[0].m_a.abs(),
                "|M_A| must strictly decrease from k = {} to k = {}",
                pair[0].in_plane_flexibility_factor,
                pair[1].in_plane_flexibility_factor
            );
            assert!(
                pair[1].m_b.abs() < pair[0].m_b.abs(),
                "|M_B| must strictly decrease from k = {} to k = {}",
                pair[0].in_plane_flexibility_factor,
                pair[1].in_plane_flexibility_factor
            );
            assert!(
                pair[1].h_b.abs() < pair[0].h_b.abs(),
                "|H_B| must strictly decrease from k = {} to k = {}",
                pair[0].in_plane_flexibility_factor,
                pair[1].in_plane_flexibility_factor
            );
            assert!(
                pair[1].v_b.abs() < pair[0].v_b.abs(),
                "|V_B| must strictly decrease from k = {} to k = {}",
                pair[0].in_plane_flexibility_factor,
                pair[1].in_plane_flexibility_factor
            );
        }
    }

    #[test]
    fn expansion_loop_reactions_satisfy_whole_body_equilibrium() {
        let x_b = EXPANSION_LOOP_BEND_RADIUS + EXPANSION_LOOP_LEG2_LENGTH;
        let y_b = EXPANSION_LOOP_LEG1_LENGTH + EXPANSION_LOOP_BEND_RADIUS;
        for in_plane_flexibility_factor in EXPANSION_LOOP_FLEXIBILITY_FACTORS {
            let result =
                solve_expansion_loop_curved_bend_thermal(in_plane_flexibility_factor).unwrap();
            let sum_fx = result.h_a + result.h_b;
            let sum_fy = result.v_a + result.v_b;
            let sum_m_about_a = result.m_a + result.m_b + x_b * result.v_b - y_b * result.h_b;
            assert!(
                sum_fx.abs() <= EXPANSION_LOOP_EQUILIBRIUM_FORCE_TOLERANCE,
                "sum Fx residual {sum_fx} at k = {in_plane_flexibility_factor}"
            );
            assert!(
                sum_fy.abs() <= EXPANSION_LOOP_EQUILIBRIUM_FORCE_TOLERANCE,
                "sum Fy residual {sum_fy} at k = {in_plane_flexibility_factor}"
            );
            assert!(
                sum_m_about_a.abs() <= EXPANSION_LOOP_EQUILIBRIUM_MOMENT_TOLERANCE,
                "sum M about A residual {sum_m_about_a} at k = {in_plane_flexibility_factor}"
            );
        }
    }

    #[test]
    fn expansion_loop_stiffer_elbow_forces_larger_departure_from_free_growth() {
        // Free thermal growth of T2 in Y is alpha DeltaT y_T2 = 6.3e-3 m. The
        // witness table shows every restrained u_y(T2) above that value, with
        // the k = 1 (stiffest elbow) departure strictly larger than the
        // k = 20 (most flexible elbow) departure: increased elbow flexibility
        // lets the tangent point relax toward its free-growth position.
        let free_uy = expansion_loop_thermal_strain()
            * (EXPANSION_LOOP_LEG1_LENGTH + EXPANSION_LOOP_BEND_RADIUS);
        let stiff = solve_expansion_loop_curved_bend_thermal(1.0).unwrap();
        let flexible = solve_expansion_loop_curved_bend_thermal(20.0).unwrap();
        for result in [&stiff, &flexible] {
            assert!(
                result.t2_uy_displacement > free_uy,
                "restrained u_y(T2) {} must exceed free growth {free_uy} at k = {}",
                result.t2_uy_displacement,
                result.in_plane_flexibility_factor
            );
        }
        assert!(
            (stiff.t2_uy_displacement - free_uy).abs()
                > (flexible.t2_uy_displacement - free_uy).abs(),
            "k = 1 departure {} must exceed k = 20 departure {}",
            (stiff.t2_uy_displacement - free_uy).abs(),
            (flexible.t2_uy_displacement - free_uy).abs()
        );
    }

    #[test]
    fn curved_bend_distributed_fixture_matches_witness_reference_table() {
        let fixture = curved_bend_distributed_fixed_end_fixture();
        assert_eq!(fixture.fixture_id, "MECH-CURVED-BEND-DISTRIBUTED-FIXED-END");
        assert!(fixture.has_dimensioned_expected_values());
        assert!(fixture.provenance.is_publicly_usable());
        // Both sides are closed-form; the measured normalized deviation must
        // sit inside the DEC-026 analytic-class 1.0e-9 relative tier.
        let deviation = curved_bend_distributed_fixed_end_max_normalized_deviation().unwrap();
        eprintln!(
            "MECH-CURVED-BEND-DISTRIBUTED-FIXED-END measured normalized deviation: {deviation:e}"
        );
        assert!(
            deviation <= CBDFE_RELATIVE_TOLERANCE,
            "measured normalized deviation {deviation} exceeds the analytic-class tolerance"
        );
        assert!(validate_curved_bend_distributed_fixed_end());
    }

    #[test]
    fn curved_bend_distributed_reactions_carry_the_exact_load_resultant() {
        for flexibility_factor in CBDFE_FLEXIBILITY_FACTORS {
            let radius = CBDFE_BEND_RADIUS;
            let arc_length = radius * std::f64::consts::PI / 2.0;
            let in_plane = solve_curved_bend_distributed_fixed_end(
                flexibility_factor,
                CurvedBendDistributedLoadCase::InPlane,
            )
            .unwrap();
            // Force balance in the load direction.
            let total = CBDFE_IN_PLANE_INTENSITY * arc_length;
            assert!(
                (in_plane.reactions_a[UY] + in_plane.reactions_b[UY] + total).abs()
                    <= 1.0e-9 * total.abs()
            );
            let out_of_plane = solve_curved_bend_distributed_fixed_end(
                flexibility_factor,
                CurvedBendDistributedLoadCase::OutOfPlane,
            )
            .unwrap();
            // Mirror symmetry of the quarter arc: each clamp carries exactly
            // half the out-of-plane load, independent of k.
            let half = -CBDFE_OUT_OF_PLANE_INTENSITY * arc_length / 2.0;
            assert!((out_of_plane.reactions_a[UZ] - half).abs() <= 1.0e-9 * half);
            assert!((out_of_plane.reactions_b[UZ] - half).abs() <= 1.0e-9 * half);
            // Antisymmetric torsion with an exact midspan zero; symmetric
            // bending about the inward radial.
            let stations = out_of_plane.stations;
            assert!((stations[0][3] + stations[2][3]).abs() <= 1.0e-9 * stations[0][3].abs());
            assert!(stations[1][3].abs() <= 1.0e-6);
            assert!((stations[0][4] - stations[2][4]).abs() <= 1.0e-9 * stations[0][4].abs());
        }
    }

    #[test]
    fn curved_bend_distributed_free_tip_matches_station_end_identity() {
        // At fraction 1 the station resultants reduce to the node-B end
        // force in the section frame; at the clamped state that end force is
        // the redundant itself, so the fraction-1 station reproduces the
        // tabulated B reactions rotated into the section frame at B
        // (tangent (-1, 0, 0), inward radial (0, -1, 0), normal (0, 0, 1)).
        let element = cbdfe_element(2.0).unwrap();
        let intensity = cbdfe_intensity(CurvedBendDistributedLoadCase::InPlane);
        let result =
            solve_curved_bend_distributed_fixed_end(2.0, CurvedBendDistributedLoadCase::InPlane)
                .unwrap();
        let at_b = element
            .arc_section_resultants(1.0, result.reactions_b, intensity)
            .unwrap();
        assert!((at_b[0] - -result.reactions_b[UX]).abs() <= 1.0e-9 * result.reactions_b[UX].abs());
        assert!((at_b[1] - -result.reactions_b[UY]).abs() <= 1.0e-9 * result.reactions_b[UY].abs());
        assert!((at_b[5] - result.reactions_b[RZ]).abs() <= 1.0e-9 * result.reactions_b[RZ].abs());
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

//! Product-preview physics adapter.
//!
//! This crate maps invented public preview-model data into the code-neutral
//! mechanics crates. It emits mechanics quantities and diagnostics only; it
//! does not encode standards criteria, allowables, SIF tables, private data, or
//! professional acceptance.

use open_pipe_stress_curved_bend::CurvedBendMacroElement;
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness_with_user_elements, element_dof_map, reduce_system, solve_dense,
    FrameElement, FrameKernelError, FrameNode, Matrix12, UserStiffnessElement, DOF_PER_NODE,
    ELEMENT_DOF, RX, RY, RZ, UX, UY, UZ,
};
use open_pipe_stress_linear_supports::{
    prepare_boundary, FrameDof, LinearSupport, QuantityDimension, SpringEntry, SupportFamily,
    SupportQuantity,
};
use open_pipe_stress_load_case_algebra::{
    evaluate_linear_combination, evaluate_range_envelope, evaluate_result_state_subtraction,
    AlgebraOperand, AlgebraQuantity, AlgebraResult, AnalysisStatus as AlgebraAnalysisStatus,
    CombinationTerm, FindingCode, RangeMode,
};
use open_pipe_stress_nonlinear_integration::{
    solve_active_set_frame_with_mode, ConvergenceControl, ConvergencePolicyStatus,
    CurvedBendStiffnessElement, DerivedFrictionNormalReaction, FrictionNormalReaction,
    LinearSolveMode, NonlinearFrameSolveInput, NonlinearIntegrationError,
    NonlinearResidualObservation,
};
use open_pipe_stress_nonlinear_supports::{
    ActivationSense, ActiveSetState, GapDirection, NonlinearSupport, NonlinearSupportBehavior,
    SupportStateRecord,
};
use open_pipe_stress_primitive_loads::{
    generate_seismic_equivalent_static_loads, generate_wind_equivalent_static_loads, prepare_loads,
    ElementExposedDiameter, ElementMassPerLength, EquivalentStaticAxisFactor, LoadDimension,
    LoadDirection, LoadQuantity, PrimitiveLoad, PrimitiveLoadCategory,
    SeismicEquivalentStaticBasis, WindEquivalentStaticBasis,
};
use open_pipe_stress_solver_diagnostics::{
    DiagnosticSeverity as SolverDiagnosticSeverity, SolverDiagnostic, SolverDiagnosticCode,
};
use open_pipe_stress_sparse_direct::{solve_symmetric_system_from_entries, SymmetricMatrixEntry};
use open_pipe_stress_straight_pipe::{StraightPipeElement, StraightPipeSectionProperties};
use open_pipe_stress_stress_recovery::{
    recover_stresses, AnalysisStatus, ForceResultants, PressureBasis, StressComponents,
    StressRecoveryInput, StressSectionProperties,
};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::f64::consts::PI;

mod validation;
use validation::validate_model_inputs;

const DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_POLICY_REF: &str =
    "DEC-046-CV-B-product-preview-active-set-count-v1";
const DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_MAX_ITERATIONS: usize = 4;
const DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_RESIDUAL_TOLERANCE: f64 = 0.0;
const DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_ABSOLUTE_FLOOR: f64 = 0.0;
const DEC_046_PRODUCT_PREVIEW_FREE_DOF_FORCE_MOMENT_POLICY_REF: &str =
    "DEC-046-CV-B-product-preview-free-dof-force-moment-residual-v1";
const DEC_046_PRODUCT_PREVIEW_FREE_DOF_FORCE_ABSOLUTE_LIMIT: f64 = 0.0;
const DEC_046_PRODUCT_PREVIEW_FREE_DOF_MOMENT_ABSOLUTE_LIMIT: f64 = 0.0;
const DEC_046_PRODUCT_PREVIEW_FREE_DOF_WORK_POLICY_REF: &str =
    "DEC-046-CV-B-product-preview-free-dof-work-residual-v1";
const DEC_046_PRODUCT_PREVIEW_FREE_DOF_WORK_ABSOLUTE_LIMIT: f64 = 0.0;
const DEC_046_PRODUCT_PREVIEW_GENERAL_ENERGY_POLICY_REF: &str =
    "DEC-046-CV-B-product-preview-general-energy-residual-v1";
const DEC_046_PRODUCT_PREVIEW_GENERAL_ENERGY_ABSOLUTE_LIMIT: f64 = 0.0;
const DEC_046_PRODUCT_PREVIEW_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF: &str =
    "DEC-046-CV-B-product-preview-displacement-reaction-delta-observation-v1";
const DEC_046_PRODUCT_PREVIEW_DISPLACEMENT_REACTION_DELTA_POLICY_REF: &str =
    "DEC-046-CV-B-product-preview-displacement-reaction-delta-threshold-v1";
const DEC_046_PRODUCT_PREVIEW_TRANSLATION_DELTA_ABSOLUTE_LIMIT_MM: f64 = 50.0;
const DEC_046_PRODUCT_PREVIEW_ROTATION_DELTA_ABSOLUTE_LIMIT_RAD: f64 = 0.05;
const DEC_046_PRODUCT_PREVIEW_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT_N: f64 = 110_000.0;
const DEC_046_PRODUCT_PREVIEW_MOMENT_REACTION_DELTA_ABSOLUTE_LIMIT_N_M: f64 = 110_000.0;

// DEC-070 curved-bend macro-element realization (D-34 Option O-B): a bend
// component with this solver-consumption mode is assembled as an
// arc-consistent macro-element carrying the user-entered flexibility factor;
// the legacy `mechanics_geometry_only` mode keeps the DEC-045 multiplier-only
// behavior byte-identically.
const DEC_070_CURVED_BEND_SOLVER_CONSUMPTION: &str = "curved_bend_macro_element";
// Relative agreement demanded between the user-entered bend angle and the
// included angle implied by the user chord and user bend radius.
const DEC_070_CURVED_BEND_ANGLE_MATCH_TOLERANCE: f64 = 1.0e-6;
// Minimum in-plane component of the pipe y_reference perpendicular to the
// chord before the user bend plane is treated as undefined.
const DEC_070_CURVED_BEND_PLANE_TOLERANCE: f64 = 1.0e-9;

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewModel {
    pub schema_version: String,
    pub document_kind: String,
    pub project: Project,
    pub analysis_status: StatusEnvelope,
    pub nodes: Vec<PreviewNode>,
    pub pipe_segments: Vec<PreviewPipe>,
    pub supports: Vec<PreviewSupport>,
    #[serde(default)]
    pub components: Vec<PreviewComponent>,
    #[serde(default)]
    pub materials: Vec<MaterialInput>,
    #[serde(default)]
    pub load_cases: Vec<PreviewLoadCase>,
    #[serde(default)]
    pub combinations: Vec<PreviewCombination>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct StatusEnvelope {
    pub mechanics: String,
    pub rule_check: String,
    pub professional_acceptance: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewNode {
    pub id: String,
    pub position: Vec3,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewPipe {
    pub id: String,
    pub from: String,
    pub to: String,
    pub section: PipeSectionInput,
    pub material: String,
    #[serde(default)]
    pub y_reference: Option<Vec3>,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PipeSectionInput {
    pub outside_diameter: Quantity,
    pub wall_thickness: Quantity,
    /// User-entered absolute mill-tolerance thickness reduction (length).
    /// Absence means no reduction; absence is not a default value of zero.
    #[serde(default)]
    pub mill_tolerance: Option<Quantity>,
    /// User-entered pipe material density for the model's own computed mass
    /// distribution. Optional; equivalent-static generation blocks when it
    /// needs mass and this is absent (absence is not a default of zero).
    #[serde(default)]
    pub material_density: Option<Quantity>,
    /// User-entered contents density over the effective inside area.
    #[serde(default)]
    pub contents_density: Option<Quantity>,
    /// User-entered insulation annulus thickness outside the pipe.
    #[serde(default)]
    pub insulation_thickness: Option<Quantity>,
    /// User-entered insulation density over the insulation annulus.
    #[serde(default)]
    pub insulation_density: Option<Quantity>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Quantity {
    pub value: f64,
    #[allow(dead_code)]
    pub unit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VectorQuantity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    #[allow(dead_code)]
    pub unit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewComponent {
    pub id: String,
    #[serde(default)]
    pub label: Option<String>,
    pub kind: String,
    pub node: String,
    #[serde(default)]
    pub geometry: Option<ComponentGeometryInput>,
    #[serde(default)]
    pub modifiers: Option<ComponentModifierInput>,
    #[serde(default)]
    pub mechanics_interface: Option<ComponentMechanicsInterfaceInput>,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ComponentGeometryInput {
    #[serde(default)]
    pub bend_pipe_ref: Option<String>,
    #[serde(default)]
    pub bend_radius: Option<Quantity>,
    #[serde(default)]
    pub bend_angle: Option<Quantity>,
    #[serde(default)]
    pub bend_plane_orientation: Option<String>,
    #[serde(default)]
    pub bend_geometry_source_reference: Option<String>,
    #[serde(default)]
    pub branch_header_pipe_ref: Option<String>,
    #[serde(default)]
    pub branch_branch_pipe_ref: Option<String>,
    #[serde(default)]
    pub branch_run_size: Option<Quantity>,
    #[serde(default)]
    pub branch_header_size: Option<Quantity>,
    #[serde(default)]
    pub branch_connection_angle: Option<Quantity>,
    #[serde(default)]
    pub branch_connection_type: Option<String>,
    #[serde(default)]
    pub branch_reinforcement_area: Option<Quantity>,
    #[serde(default)]
    pub branch_reinforcement_reference: Option<String>,
    #[serde(default)]
    pub branch_geometry_source_reference: Option<String>,
    #[serde(default)]
    pub rigid_pipe_ref: Option<String>,
    #[serde(default)]
    pub rigid_body_length: Option<Quantity>,
    #[serde(default)]
    pub end_a_size: Option<Quantity>,
    #[serde(default)]
    pub end_b_size: Option<Quantity>,
    #[serde(default)]
    pub weight: Option<Quantity>,
    #[serde(default)]
    pub center_of_gravity: Option<VectorQuantity>,
    #[serde(default)]
    pub connection_end_a_reference: Option<String>,
    #[serde(default)]
    pub connection_end_b_reference: Option<String>,
    #[serde(default)]
    pub stiffness_behavior_reference: Option<String>,
    #[serde(default)]
    pub rigid_component_source_reference: Option<String>,
    #[serde(default)]
    pub expansion_joint_pipe_ref: Option<String>,
    #[serde(default)]
    pub effective_area: Option<Quantity>,
    #[serde(default)]
    pub movement_limit: Option<Quantity>,
    #[serde(default)]
    pub hardware_reference: Option<String>,
    #[serde(default)]
    pub manufacturer_reference: Option<String>,
    #[serde(default)]
    pub pressure_thrust_reference: Option<String>,
    #[serde(default)]
    pub expansion_joint_source_reference: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ComponentModifierInput {
    #[serde(default)]
    pub sif_user_value: Option<Quantity>,
    #[serde(default)]
    pub branch_header_sif_user_value: Option<Quantity>,
    #[serde(default)]
    pub branch_branch_sif_user_value: Option<Quantity>,
    #[serde(default)]
    pub flexibility_factor_user_value: Option<Quantity>,
    #[serde(default)]
    pub stiffness_scaling_user_value: Option<Quantity>,
    #[serde(default)]
    pub linear_stiffness_user_value: Option<Quantity>,
    #[serde(default)]
    pub rotational_stiffness_user_value: Option<Quantity>,
    #[serde(default)]
    pub axial_stiffness_user_value: Option<Quantity>,
    #[serde(default)]
    pub lateral_stiffness_user_value: Option<Quantity>,
    #[serde(default)]
    pub angular_stiffness_user_value: Option<Quantity>,
    #[serde(default)]
    pub torsional_stiffness_user_value: Option<Quantity>,
    #[serde(default)]
    pub source_reference: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ComponentMechanicsInterfaceInput {
    #[serde(default)]
    pub solver_consumption: Option<String>,
    #[serde(default)]
    pub rule_check_consumption: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewSupport {
    pub id: String,
    pub node: String,
    pub restraints: Vec<String>,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub stiffness: Option<SupportStiffnessInput>,
    #[serde(default)]
    pub hanger: Option<SpringHangerInput>,
    #[serde(default)]
    pub nonlinear: Option<NonlinearSupportInput>,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SupportStiffnessInput {
    pub dof: String,
    pub value: Quantity,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpringHangerInput {
    #[serde(default)]
    pub hanger_type: Option<String>,
    #[serde(default)]
    pub stiffness: Option<SupportStiffnessInput>,
    #[serde(default)]
    pub installed_load: Option<Quantity>,
    #[serde(default)]
    pub cold_load: Option<Quantity>,
    #[serde(default)]
    pub hot_load: Option<Quantity>,
    #[serde(default)]
    pub constant_load: Option<Quantity>,
    #[serde(default)]
    pub travel_range: Option<Quantity>,
    #[serde(default)]
    pub movement_limit: Option<Quantity>,
    #[serde(default)]
    pub manufacturer_reference: Option<String>,
    #[serde(default)]
    pub source_reference: Option<String>,
    #[serde(default)]
    pub load_side_review_reference: Option<String>,
    #[serde(default)]
    pub mechanics_consumption: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NonlinearSupportInput {
    pub behavior: String,
    pub dof: String,
    #[serde(default)]
    pub initial_state: Option<String>,
    #[serde(default)]
    pub active_when: Option<String>,
    #[serde(default)]
    pub contact_when: Option<String>,
    #[serde(default)]
    pub closes_when: Option<String>,
    #[serde(default)]
    pub gap: Option<Quantity>,
    #[serde(default)]
    pub friction_coefficient: Option<Quantity>,
    #[serde(default)]
    pub normal_reaction: Option<Quantity>,
    #[serde(default)]
    pub normal_reaction_source: Option<FrictionNormalReactionSourceInput>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FrictionNormalReactionSourceInput {
    pub support_ref: String,
    pub dof: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewLoadCase {
    pub id: String,
    #[serde(default)]
    pub primitive_loads: Vec<PreviewPrimitiveLoad>,
    /// Optional user-entered static-equivalent generation inputs (DEC-068
    /// item 2). When present, the preview synthesizes seismic/wind
    /// distributed loads from these explicit inputs and the model's own
    /// computed mass distribution; missing inputs are blocking.
    #[serde(default)]
    pub equivalent_static: Option<EquivalentStaticGenerationInput>,
    /// Optional modulus basis (DEC-068 item 1): names the user-entered
    /// material temperature-point id whose E (and alpha, when supplied on
    /// the point) this load case solves with. Exact-id selection remains
    /// available under DEC-077; an unresolved reference is blocking.
    #[serde(default)]
    pub modulus_basis_ref: Option<String>,
    /// Optional user-entered solve temperature (DEC-077). When supplied
    /// instead of `modulus_basis_ref`, E and alpha are linearly interpolated
    /// between the two adjacent user-entered temperature points. Requests at
    /// or outside the stored range edges are blocking; extrapolation is never
    /// performed.
    #[serde(default)]
    pub modulus_basis_temperature: Option<Quantity>,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EquivalentStaticGenerationInput {
    #[serde(default)]
    pub seismic: Option<SeismicGenerationInput>,
    #[serde(default)]
    pub wind: Option<WindGenerationInput>,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SeismicGenerationInput {
    /// User-entered gravity acceleration; an explicit input, not an
    /// embedded physical-constant default.
    #[serde(default)]
    pub gravity_acceleration: Option<Quantity>,
    #[serde(default)]
    pub g_factor_x: Option<Quantity>,
    #[serde(default)]
    pub g_factor_y: Option<Quantity>,
    #[serde(default)]
    pub g_factor_z: Option<Quantity>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WindGenerationInput {
    #[serde(default)]
    pub pressure: Option<Quantity>,
    #[serde(default)]
    pub shape_factor: Option<Quantity>,
    /// Global axis the projected wind intensity acts along
    /// (`global_x` | `global_y` | `global_z`).
    #[serde(default)]
    pub direction: Option<String>,
    /// User-marked exposed spans by pipe id; wind is generated on these
    /// spans only.
    #[serde(default)]
    pub exposed_pipe_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewCombination {
    pub id: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub label: Option<String>,
    /// Closed basis set: `mechanics` (linear terms),
    /// `result_state_subtraction` (`minuend_id` − `subtrahend_id`), and
    /// `range_envelope` (`mode` over `operand_ids`); vocabulary mirrors
    /// `open_pipe_stress_load_case_algebra::AlgebraExpression`.
    pub basis: String,
    #[serde(default)]
    pub terms: Vec<PreviewCombinationTerm>,
    #[serde(default)]
    pub minuend_id: Option<String>,
    #[serde(default)]
    pub subtrahend_id: Option<String>,
    #[serde(default)]
    pub operand_ids: Option<Vec<String>>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewCombinationTerm {
    pub load_case: String,
    pub factor: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewPrimitiveLoad {
    pub id: String,
    pub category: String,
    pub target: LoadTargetInput,
    pub direction: String,
    pub magnitude: Quantity,
    pub dimension: String,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LoadTargetInput {
    Node { node: String },
    Element { pipe: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct MaterialInput {
    pub id: String,
    pub elastic_modulus: Quantity,
    pub shear_modulus: Quantity,
    #[serde(default)]
    pub thermal_expansion_coefficient: Option<Quantity>,
    /// User-entered temperature-indexed property points (DEC-068 item 1).
    /// A load case may name one point id as its exact modulus basis or supply
    /// a solve temperature for DEC-077 linear interpolation between adjacent
    /// points. All values remain user-entered; extrapolation is blocked.
    #[serde(default)]
    pub temperature_points: Vec<MaterialTemperaturePointInput>,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MaterialTemperaturePointInput {
    /// Stable user-assigned basis id (e.g. a temperature-case label).
    pub id: String,
    #[serde(default)]
    pub temperature: Option<Quantity>,
    #[serde(default)]
    pub elastic_modulus: Option<Quantity>,
    #[serde(default)]
    pub thermal_expansion_coefficient: Option<Quantity>,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LinearStaticPreviewRequest {
    pub model: PreviewModel,
    #[serde(default)]
    pub materials: Vec<MaterialInput>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewSolverMode {
    SparseInteractive,
    DenseScrutiny,
}

impl PreviewSolverMode {
    pub fn from_wire(value: &str) -> Option<Self> {
        match value {
            "sparse_interactive" => Some(Self::SparseInteractive),
            "dense_scrutiny" => Some(Self::DenseScrutiny),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::SparseInteractive => "sparse_interactive",
            Self::DenseScrutiny => "dense_scrutiny",
        }
    }

    fn nonlinear_mode(self) -> LinearSolveMode {
        match self {
            Self::SparseInteractive => LinearSolveMode::SparseInteractive,
            Self::DenseScrutiny => LinearSolveMode::DenseScrutiny,
        }
    }

    fn mode_code(self) -> f64 {
        match self {
            Self::SparseInteractive => 1.0,
            Self::DenseScrutiny => 2.0,
        }
    }
}

impl Default for PreviewSolverMode {
    fn default() -> Self {
        Self::SparseInteractive
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MechanicsEnvelope {
    pub schema_version: String,
    pub document_kind: String,
    pub run_id: String,
    pub model_ref: String,
    pub status: StatusEnvelope,
    pub summary: Summary,
    pub results: Vec<ResultItem>,
    pub diagnostics: Vec<Diagnostic>,
    pub professional_boundary: ProfessionalBoundary,
    pub accepted_model_state_mutated: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub node_count: usize,
    pub segment_count: usize,
    pub support_count: usize,
    pub load_case_count: usize,
    pub component_stress_modifier_count: usize,
    pub component_user_stiffness_macro_element_count: usize,
    pub component_pressure_thrust_load_count: usize,
    pub spring_hanger_user_input_count: usize,
    pub max_displacement: Option<LocatedQuantity>,
    pub max_open_formula_stress: Option<LocatedQuantity>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LocatedQuantity {
    pub value: f64,
    pub unit: String,
    pub location_ref: String,
    pub result_ref: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResultItem {
    pub id: String,
    pub kind: String,
    pub value: f64,
    pub unit: String,
    pub entity_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_ref: Option<ResultBasisRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_result_refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResultMetadata>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ResultBasisRef {
    pub ref_type: String,
    pub ref_id: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ResultMetadata {
    pub component: String,
    pub coordinate_system: String,
    pub location: String,
    pub basis: String,
    pub sign_convention: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub id: String,
    pub code: String,
    pub severity: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub affected_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfessionalBoundary {
    pub human_review_required: bool,
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub software_makes_approval_claim: bool,
}

#[derive(Debug)]
struct BuiltModel {
    nodes: Vec<FrameNode>,
    pipes: Vec<StraightPipeElement>,
    frame_elements: Vec<FrameElement>,
    user_stiffness_elements: Vec<UserStiffnessElement>,
    curved_bend_elements: Vec<CurvedBendMacroBuild>,
    supports: Vec<LinearSupport>,
    nonlinear_supports: Vec<NonlinearSupport>,
    nonlinear_initial_states: Vec<SupportStateRecord>,
    nonlinear_friction_normal_reactions: Vec<FrictionNormalReaction>,
    nonlinear_derived_friction_normal_reactions: Vec<DerivedFrictionNormalReaction>,
    sections: HashMap<String, DerivedSection>,
}

#[derive(Debug, Clone)]
struct LoadCaseSolve {
    load_case_id: String,
    results: Vec<ResultItem>,
    max_displacement: Option<LocatedQuantity>,
    max_stress: Option<LocatedQuantity>,
    component_stress_modifier_count: usize,
    component_pressure_thrust_load_count: usize,
}

#[derive(Debug, Clone, Copy)]
struct ThermalElementLoad {
    element_index: usize,
    axial_load: f64,
    /// Free thermal strain `alpha * delta_T`; the curved-bend macro span uses
    /// this with the exact free-expansion identity instead of `axial_load`.
    thermal_strain: f64,
}

/// Curved-bend macro-element realization of one bend component over one pipe
/// span (DEC-070). The 12x12 global stiffness is validated and formed once at
/// build time so every assembly path consumes the identical matrix.
#[derive(Debug, Clone)]
struct CurvedBendMacroBuild {
    component_id: String,
    pipe_id: String,
    pipe_index: usize,
    node_i: usize,
    node_j: usize,
    /// Chord vector `x_j - x_i` in global coordinates.
    chord: [f64; 3],
    global_stiffness: Matrix12,
    arc_length: f64,
    included_angle: f64,
    bend_radius: f64,
    flexibility_factor: f64,
    source_reference: String,
    /// The validated macro-element itself, kept for the arc-consistent
    /// distributed-load and interior-station closed forms so every consumer
    /// shares the build-time-validated geometry.
    macro_element: CurvedBendMacroElement,
}

#[derive(Debug, Clone)]
struct PressureThrustLoad {
    element_index: usize,
    axial_load: f64,
    source_load_id: String,
    source: PressureThrustSource,
}

#[derive(Debug, Clone)]
enum PressureThrustSource {
    PipeInternalArea,
    ExpansionJointEffectiveArea(ExpansionJointPressureThrustInput),
}

#[derive(Debug, Clone)]
struct ExpansionJointPressureThrustInput {
    component_id: String,
    pipe_id: String,
    effective_area: f64,
    pressure_thrust_reference: String,
    source_reference: String,
    solver_consumption: String,
}

#[derive(Debug, Clone, Copy)]
struct DerivedSection {
    area: f64,
    internal_area: f64,
    second_moment: f64,
    torsion_constant: f64,
    section_modulus: f64,
    torsion_radius: f64,
    membrane_radius: f64,
    wall_thickness: f64,
}

pub fn run_linear_static_preview(request: LinearStaticPreviewRequest) -> MechanicsEnvelope {
    run_linear_static_preview_with_mode(request, PreviewSolverMode::default())
}

pub fn run_linear_static_preview_with_mode(
    request: LinearStaticPreviewRequest,
    solver_mode: PreviewSolverMode,
) -> MechanicsEnvelope {
    let mut model = request.model;
    let mut materials = if request.materials.is_empty() {
        model.materials.clone()
    } else {
        request.materials
    };
    let mut diagnostics = Vec::new();

    validate_model_inputs(&model, &materials, &mut diagnostics);
    if model.document_kind != "openpipestress.product_preview.model" {
        diagnostics.push(diag(
            "diagnostic:physics:document-kind",
            "PREVIEW_DOCUMENT_KIND_INVALID",
            "blocking",
            "physics adapter requires an openpipestress.product_preview.model document",
            vec!["model".to_string()],
        ));
    }
    if model.schema_version.is_empty() {
        diagnostics.push(diag(
            "diagnostic:physics:schema-version",
            "PREVIEW_SCHEMA_VERSION_MISSING",
            "blocking",
            "preview model requires an explicit schema version",
            vec!["model".to_string()],
        ));
    }
    if has_blocking(&diagnostics) {
        return blocked_envelope(model, diagnostics);
    }

    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return blocked_envelope(model, diagnostics);
    }

    let built = build_model(&model, &materials, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return blocked_envelope(model, diagnostics);
    }
    let built = built.expect("build_model returns Some when no blocking diagnostics were added");

    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    if boundary.restrained_dofs.is_empty() && boundary.springs.is_empty() {
        diagnostics.push(diag(
            "diagnostic:physics:no-restraints",
            "SUPPORT_INPUT_MISSING",
            "blocking",
            "linear-static preview requires explicit support restraints or springs; no automatic boundary conditions are applied",
            vec!["supports".to_string()],
        ));
    } else if boundary.restrained_dofs.len() < DOF_PER_NODE {
        let (restrained, missing) = support_restraint_summary(&boundary.restrained_dofs);
        let support_map = support_contribution_summary(&model);
        diagnostics.push(diag(
            "diagnostic:physics:under-restrained",
            "SOLVER_SYSTEM_BLOCKED",
            "blocking",
            format!(
                "linear-static preview has fewer than six rigid restraints; restrained global DOF classes: {restrained}; missing global rigid-body DOF classes: {missing}; support contributions: {support_map}"
            ),
            vec![
                "supports".to_string(),
                format!("restrained:{restrained}"),
                format!("missing:{missing}"),
                format!("support_contributions:{support_map}"),
            ],
        ));
    }
    for finding in &boundary.findings {
        diagnostics.push(diag(
            &format!("diagnostic:support:{}", finding.support_id),
            "SUPPORT_INPUT_INVALID",
            "blocking",
            finding.message.clone(),
            vec![finding.support_id.clone()],
        ));
    }
    if has_blocking(&diagnostics) {
        return blocked_envelope(model, diagnostics);
    }

    let mut stiffness = match assemble_global_stiffness_with_user_elements(
        built.nodes.len(),
        &built.frame_elements,
        &built.user_stiffness_elements,
    ) {
        Ok(stiffness) => stiffness,
        Err(error) => return solver_blocked(model, diagnostics, error),
    };
    add_curved_bend_stiffness_contributions(&mut stiffness, &built.curved_bend_elements);
    for spring in &boundary.springs {
        stiffness[spring.node_dof.global_index()][spring.node_dof.global_index()] +=
            spring.stiffness.value;
    }

    // DEC-068 item 1 + DEC-077: a load case may name an exact user-entered
    // temperature-point id or an explicit solve temperature. Each distinct
    // basis gets its own built model and assembled stiffness from the
    // basis-resolved E; temperature bases interpolate E and alpha only
    // between adjacent user points and never extrapolate.
    let mut basis_solve_states: Vec<(
        Option<String>,
        Vec<MaterialInput>,
        BuiltModel,
        Vec<Vec<f64>>,
        Option<String>,
    )> = vec![(None, materials.clone(), built, stiffness, None)];
    let mut load_case_solves = Vec::new();
    for load_case in &model.load_cases {
        let Some(basis_key) = modulus_basis_key(load_case, &mut diagnostics) else {
            if has_blocking(&diagnostics) {
                return blocked_envelope(model, diagnostics);
            }
            let (_, basis_materials, basis_built, basis_stiffness, basis_record) =
                &basis_solve_states[0];
            match solve_load_case(
                &model,
                basis_built,
                basis_materials,
                basis_stiffness,
                &boundary.restrained_dofs,
                &boundary.springs,
                load_case,
                basis_record.as_deref(),
                solver_mode,
                &mut diagnostics,
            ) {
                Ok(solve) => load_case_solves.push(solve),
                Err(error) => return solver_blocked(model, diagnostics, error),
            }
            if has_blocking(&diagnostics) {
                return blocked_envelope(model, diagnostics);
            }
            continue;
        };
        let state_index = match basis_solve_states
            .iter()
            .position(|(key, _, _, _, _)| key.as_ref() == Some(&basis_key))
        {
            Some(index) => index,
            None => {
                let Some((basis_materials, basis_record)) =
                    materials_for_modulus_basis(&model, &materials, load_case, &mut diagnostics)
                else {
                    return blocked_envelope(model, diagnostics);
                };
                let basis_built = build_model(&model, &basis_materials, &mut diagnostics);
                if has_blocking(&diagnostics) {
                    return blocked_envelope(model, diagnostics);
                }
                let basis_built = basis_built
                    .expect("build_model returns Some when no blocking diagnostics were added");
                let mut basis_stiffness = match assemble_global_stiffness_with_user_elements(
                    basis_built.nodes.len(),
                    &basis_built.frame_elements,
                    &basis_built.user_stiffness_elements,
                ) {
                    Ok(stiffness) => stiffness,
                    Err(error) => return solver_blocked(model, diagnostics, error),
                };
                add_curved_bend_stiffness_contributions(
                    &mut basis_stiffness,
                    &basis_built.curved_bend_elements,
                );
                for spring in &boundary.springs {
                    basis_stiffness[spring.node_dof.global_index()]
                        [spring.node_dof.global_index()] += spring.stiffness.value;
                }
                basis_solve_states.push((
                    Some(basis_key.clone()),
                    basis_materials,
                    basis_built,
                    basis_stiffness,
                    Some(basis_record),
                ));
                basis_solve_states.len() - 1
            }
        };
        let (_, basis_materials, basis_built, basis_stiffness, basis_record) =
            &basis_solve_states[state_index];
        match solve_load_case(
            &model,
            basis_built,
            basis_materials,
            basis_stiffness,
            &boundary.restrained_dofs,
            &boundary.springs,
            load_case,
            basis_record.as_deref(),
            solver_mode,
            &mut diagnostics,
        ) {
            Ok(solve) => load_case_solves.push(solve),
            Err(error) => return solver_blocked(model, diagnostics, error),
        }
        if has_blocking(&diagnostics) {
            return blocked_envelope(model, diagnostics);
        }
    }
    let built = &basis_solve_states[0].2;

    let max_displacement = load_case_solves
        .first()
        .and_then(|solve| solve.max_displacement.clone());
    let max_stress = load_case_solves
        .first()
        .and_then(|solve| solve.max_stress.clone());
    let component_stress_modifier_count = load_case_solves
        .iter()
        .map(|solve| solve.component_stress_modifier_count)
        .sum();
    let component_pressure_thrust_load_count = load_case_solves
        .iter()
        .map(|solve| solve.component_pressure_thrust_load_count)
        .sum();
    let mut results = Vec::new();
    let mut rows_by_base_id: HashMap<String, HashMap<String, ResultItem>> = HashMap::new();
    for (index, solve) in load_case_solves.into_iter().enumerate() {
        let is_default = index == 0;
        let load_case_id = solve.load_case_id.clone();
        for mut result in solve.results {
            let base_id = result.id.clone();
            result.basis_ref = Some(ResultBasisRef {
                ref_type: "load_case".to_string(),
                ref_id: load_case_id.clone(),
            });
            if !is_default {
                result.id = qualified_load_case_result_id(&load_case_id, &base_id);
            }
            rows_by_base_id
                .entry(base_id)
                .or_default()
                .insert(load_case_id.clone(), result.clone());
            results.push(result);
        }
    }
    append_combination_results(&model, &rows_by_base_id, &mut results, &mut diagnostics);
    append_combination_modulus_basis_records(&model, &mut results);
    let component_user_stiffness_macro_element_count =
        append_expansion_joint_user_stiffness_results(&model, &mut results)
            + append_curved_bend_macro_element_results(&built.curved_bend_elements, &mut results);
    let spring_hanger_user_input_count =
        append_spring_hanger_user_input_results(&model, &mut results);

    if let Some(maximum) = &max_displacement {
        if maximum.value > 5.0 {
            diagnostics.push(diag(
                "diagnostic:physics:high-displacement-review",
                "HIGH_DISPLACEMENT_REVIEW",
                "warning",
                "computed preview displacement exceeds the invented review threshold; inspect support/load inputs before relying on trends",
                vec![maximum.result_ref.clone(), maximum.location_ref.clone()],
            ));
        }
    }
    diagnostics.push(diag(
        "diagnostic:physics:rule-inputs-missing",
        "RULE_CHECK_INPUTS_MISSING",
        "warning",
        "rule-check inputs and protected criteria are absent; no compliance result is produced",
        vec![],
    ));

    MechanicsEnvelope {
        schema_version: "0.1.0".to_string(),
        document_kind: "openpipestress.product_preview.mechanics_result".to_string(),
        run_id: "run:preview-linear-static-001".to_string(),
        model_ref: model.project.id,
        status: StatusEnvelope {
            mechanics: "MECHANICS_SOLVED".to_string(),
            rule_check: "RULE_INPUTS_INCOMPLETE".to_string(),
            professional_acceptance: "NOT_PROVIDED".to_string(),
        },
        summary: Summary {
            node_count: model.nodes.len(),
            segment_count: model.pipe_segments.len(),
            support_count: model.supports.len(),
            load_case_count: model.load_cases.len(),
            component_stress_modifier_count,
            component_user_stiffness_macro_element_count,
            component_pressure_thrust_load_count,
            spring_hanger_user_input_count,
            max_displacement,
            max_open_formula_stress: max_stress,
        },
        results,
        diagnostics,
        professional_boundary: professional_boundary(),
        accepted_model_state_mutated: false,
    }
}

fn solve_load_case(
    model: &PreviewModel,
    built: &BuiltModel,
    materials: &[MaterialInput],
    stiffness: &[Vec<f64>],
    restrained_dofs: &[usize],
    spring_entries: &[SpringEntry],
    load_case: &PreviewLoadCase,
    modulus_basis_record: Option<&str>,
    solver_mode: PreviewSolverMode,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<LoadCaseSolve, FrameKernelError> {
    let loads = build_load_case_primitive_loads(model, load_case, diagnostics);
    let load_application = prepare_loads(built.nodes.len(), built.pipes.len(), &loads);
    for finding in &load_application.findings {
        diagnostics.push(diag(
            &format!("diagnostic:load:{}", finding.load_id),
            "LOAD_INPUT_INVALID",
            "blocking",
            finding.message.clone(),
            vec![finding.load_id.clone(), load_case.id.clone()],
        ));
    }
    if has_blocking(diagnostics) {
        return Ok(LoadCaseSolve {
            load_case_id: load_case.id.clone(),
            results: Vec::new(),
            max_displacement: None,
            max_stress: None,
            component_stress_modifier_count: 0,
            component_pressure_thrust_load_count: 0,
        });
    }

    let pipe_map = model
        .pipe_segments
        .iter()
        .enumerate()
        .map(|(i, p)| (p.id.as_str(), i))
        .collect::<HashMap<_, _>>();
    let material_map = materials
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect::<HashMap<_, _>>();
    let thermal_loads =
        build_thermal_element_loads(model, load_case, &material_map, &pipe_map, &built.sections);
    let pressure_thrust_loads =
        build_pressure_thrust_loads(model, load_case, &pipe_map, &built.sections);

    let curved_bends_by_pipe = built
        .curved_bend_elements
        .iter()
        .map(|element| (element.pipe_index, element))
        .collect::<HashMap<_, _>>();
    let curved_bend_uniform_intensity = curved_bend_uniform_intensity_by_pipe(
        &load_application.element_uniform_loads,
        &curved_bends_by_pipe,
    );
    let mut force = load_application.global_load_vector(built.nodes.len());
    add_uniform_element_loads(
        &mut force,
        model,
        &load_application.element_uniform_loads,
        &built.pipes,
        &curved_bends_by_pipe,
        &load_case.id,
        diagnostics,
    );
    // Pressure thrust keeps the straight-element treatment on macro spans:
    // equal/opposite axial end forces along the chord direction (decision
    // recorded in the curved-bend review-row basis).
    add_pressure_thrust_loads(&mut force, &pressure_thrust_loads, &built.pipes);
    add_thermal_equivalent_loads(
        &mut force,
        &thermal_loads,
        &built.pipes,
        &curved_bends_by_pipe,
    );

    let reduced = reduce_system(stiffness, &force, restrained_dofs)?;
    let linear_solve = solve_preview_reduced_system(
        solver_mode,
        &reduced.stiffness,
        &reduced.force,
        built,
        spring_entries,
        &force,
        restrained_dofs,
        load_case,
        diagnostics,
    )?;
    let reduced_displacements = linear_solve.solution.clone();

    let mut displacements = vec![0.0; built.nodes.len() * DOF_PER_NODE];
    for (index, dof) in reduced.free_dofs.iter().enumerate() {
        displacements[*dof] = reduced_displacements[index];
    }

    let mut results = Vec::new();
    append_linear_solver_mode_evidence(&mut results, &load_case.id, solver_mode, &linear_solve);
    append_modulus_basis_record(&mut results, load_case, modulus_basis_record);
    if solver_mode == PreviewSolverMode::DenseScrutiny {
        append_sparse_live_path_evidence(
            &mut results,
            diagnostics,
            &load_case.id,
            built,
            spring_entries,
            &force,
            restrained_dofs,
            &reduced_displacements,
        );
    }
    let component_pressure_thrust_load_count = append_expansion_joint_pressure_thrust_results(
        &mut results,
        diagnostics,
        load_case,
        &pressure_thrust_loads,
    );
    let mut max_displacement = None;
    for node in &model.nodes {
        let node_index = node_index(&model, &node.id).unwrap();
        let magnitude = displacement_magnitude(&displacements, node_index);
        let result_id = format!("result:disp:{}", stable_suffix(&node.id));
        if max_displacement
            .as_ref()
            .map(|q: &LocatedQuantity| magnitude * 1000.0 > q.value)
            .unwrap_or(true)
        {
            max_displacement = Some(LocatedQuantity {
                value: round6(magnitude * 1000.0),
                unit: "mm".to_string(),
                location_ref: node.id.clone(),
                result_ref: result_id.clone(),
            });
        }
        results.push(ResultItem {
            id: result_id,
            kind: "displacement_magnitude".to_string(),
            value: round6(magnitude * 1000.0),
            unit: "mm".to_string(),
            entity_ref: node.id.clone(),
            basis_ref: None,
            source_result_refs: Vec::new(),
            metadata: None,
        });
    }
    for node in &model.nodes {
        let node_index = node_index(&model, &node.id).unwrap();
        append_node_displacement_component_results(
            &mut results,
            &node.id,
            &displacements,
            node_index,
        );
    }

    let reactions = multiply_matrix_vector(&stiffness, &displacements)
        .into_iter()
        .zip(force.iter())
        .map(|(internal, applied)| internal - applied)
        .collect::<Vec<_>>();
    for support in &model.supports {
        if let Some(index) = node_index(&model, &support.node) {
            let magnitude = support
                .restraints
                .iter()
                .filter_map(|dof| parse_dof(dof).ok())
                .map(|dof| {
                    let global = index * DOF_PER_NODE + dof_index(dof);
                    reactions[global] * reactions[global]
                })
                .sum::<f64>()
                .sqrt();
            results.push(ResultItem {
                id: format!("result:reaction:{}", stable_suffix(&support.id)),
                kind: "reaction_resultant".to_string(),
                value: round6(magnitude),
                unit: "N".to_string(),
                entity_ref: support.id.clone(),
                basis_ref: None,
                source_result_refs: Vec::new(),
                metadata: None,
            });
        }
    }

    append_nonlinear_support_loop_results(
        &mut results,
        diagnostics,
        built,
        restrained_dofs,
        &force,
        load_case,
        solver_mode,
    );

    let mut max_stress = None;
    let mut component_stress_modifier_count = 0;
    for (pipe_index, pipe) in built.pipes.iter().enumerate() {
        let macro_bend = curved_bends_by_pipe.get(&pipe_index).copied();
        let uniform_intensity = curved_bend_uniform_intensity
            .get(&pipe_index)
            .copied()
            .unwrap_or([0.0; 3]);
        let corrected_local_forces = if let Some(bend) = macro_bend {
            // Macro-span end forces come from the assembled arc stiffness
            // (K_macro * d minus the exact free-expansion thermal part and
            // minus the arc-consistent distributed equivalent loads),
            // expressed in the chord frame so the existing result-envelope
            // rows keep their coordinate convention.
            match recover_curved_bend_local_forces(
                bend,
                pipe,
                &displacements,
                &thermal_loads,
                &pressure_thrust_loads,
                uniform_intensity,
            ) {
                Ok(local_forces) => local_forces,
                Err(message) => {
                    diagnostics.push(diag(
                        &format!("diagnostic:stress:{}", stable_suffix(&pipe.element_id)),
                        "ELEMENT_FORCE_RECOVERY_FAILED",
                        "warning",
                        message,
                        vec![pipe.element_id.clone()],
                    ));
                    continue;
                }
            }
        } else {
            let local = match pipe.recover_local_forces_from_global_model(&displacements) {
                Ok(local) => local,
                Err(error) => {
                    diagnostics.push(diag(
                        &format!("diagnostic:stress:{}", stable_suffix(&pipe.element_id)),
                        "ELEMENT_FORCE_RECOVERY_FAILED",
                        "warning",
                        error.to_string(),
                        vec![pipe.element_id.clone()],
                    ));
                    continue;
                }
            };
            corrected_local_forces_for_axial_effects(
                &local.local_forces,
                pipe_index,
                &thermal_loads,
                &pressure_thrust_loads,
            )
        };
        append_element_force_results(&mut results, &pipe.element_id, &corrected_local_forces);
        // Interior stations: straight spans keep the endpoint-linear
        // interpolation; curved-bend macro spans evaluate true section
        // resultants along the arc by segment equilibrium from the recovered
        // assembled end forces (closed form in the curved-bend crate).
        let station_resultants = if let Some(bend) = macro_bend {
            match curved_bend_station_resultants(
                bend,
                pipe,
                &corrected_local_forces,
                uniform_intensity,
            ) {
                Ok(stations) => stations.to_vec(),
                Err(message) => {
                    diagnostics.push(diag(
                        &format!(
                            "diagnostic:curved-bend:{}:{}:interior-stations",
                            stable_suffix(&load_case.id),
                            stable_suffix(&pipe.element_id)
                        ),
                        "ELEMENT_FORCE_RECOVERY_FAILED",
                        "warning",
                        format!(
                            "curved-bend span {} could not evaluate arc interior station resultants: {message}",
                            pipe.element_id
                        ),
                        vec![
                            pipe.element_id.clone(),
                            bend.component_id.clone(),
                            load_case.id.clone(),
                        ],
                    ));
                    Vec::new()
                }
            }
        } else {
            station_grid_resultants_from_endpoints(&corrected_local_forces).to_vec()
        };
        let (station_basis, station_sign_convention, station_coordinate_system) = if macro_bend
            .is_some()
        {
            (
                    CURVED_BEND_STATION_BASIS,
                    "positive value follows the j-side arc segment action on the section in the arc section frame (x tangent toward end j, z bend-plane normal, y toward the arc center)",
                    "arc_section_frame",
                )
        } else {
            (
                "interpolated_from_endpoint_resultants",
                "positive value is linearly interpolated from endpoint element-local resultants",
                "element_local",
            )
        };
        for station in &station_resultants {
            append_station_force_results(
                &mut results,
                &pipe.element_id,
                station.location,
                &station.resultants,
                station_basis,
                station_sign_convention,
                station_coordinate_system,
            );
        }
        let section = built
            .sections
            .get(&pipe.element_id)
            .expect("section exists for pipe");
        let pressure = pressure_for_pipe(model, load_case, pipe_index, &pipe.element_id);
        let pressure_thrust_active =
            pressure_thrust_for_pipe(pipe_index, &pressure_thrust_loads) != 0.0;
        let include_pressure_longitudinal = !pressure_thrust_active;
        let end_i_stress = recover_endpoint_stress(&corrected_local_forces, 0, section, pressure);
        let end_j_stress =
            recover_endpoint_stress(&corrected_local_forces, DOF_PER_NODE, section, pressure);
        let station_stresses = station_resultants
            .iter()
            .map(|station| {
                (
                    station.location,
                    recover_station_stress(&station.resultants, section, pressure),
                )
            })
            .collect::<Vec<_>>();
        for (location, stress) in [("end_i", &end_i_stress), ("end_j", &end_j_stress)] {
            for finding in &stress.findings {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:stress:{}:{}:{:?}",
                        stable_suffix(&pipe.element_id),
                        location.replace('_', "-"),
                        finding.code
                    ),
                    "STRESS_RECOVERY_LIMITED",
                    "warning",
                    finding.message.clone(),
                    vec![pipe.element_id.clone()],
                ));
            }
        }
        for (location, stress) in &station_stresses {
            for finding in &stress.findings {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:stress:{}:{}:{:?}",
                        stable_suffix(&pipe.element_id),
                        location.replace('_', "-"),
                        finding.code
                    ),
                    "STRESS_RECOVERY_LIMITED",
                    "warning",
                    finding.message.clone(),
                    vec![pipe.element_id.clone()],
                ));
            }
        }
        if end_i_stress.findings.is_empty() {
            append_endpoint_stress_results(
                &mut results,
                &pipe.element_id,
                "end_i",
                &end_i_stress.components,
                pressure.is_some(),
                include_pressure_longitudinal,
            );
        }
        if end_j_stress.findings.is_empty() {
            append_endpoint_stress_results(
                &mut results,
                &pipe.element_id,
                "end_j",
                &end_j_stress.components,
                pressure.is_some(),
                include_pressure_longitudinal,
            );
        }
        for (location, stress) in &station_stresses {
            if !stress.findings.is_empty() {
                continue;
            }
            append_station_stress_results(
                &mut results,
                &pipe.element_id,
                location,
                &stress.components,
                pressure.is_some(),
                include_pressure_longitudinal,
                station_basis,
            );
        }
        let mut summary_values = [
            open_formula_summary_mpa(&end_i_stress, include_pressure_longitudinal),
            open_formula_summary_mpa(&end_j_stress, include_pressure_longitudinal),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        for (_, stress) in &station_stresses {
            if let Some(value) = open_formula_summary_mpa(stress, include_pressure_longitudinal) {
                summary_values.push(value);
            }
        }
        let summary_value = summary_values.into_iter().reduce(f64::max);
        if let Some(value) = summary_value {
            let result_id = format!("result:stress:{}", stable_suffix(&pipe.element_id));
            if max_stress
                .as_ref()
                .map(|q: &LocatedQuantity| value > q.value)
                .unwrap_or(true)
            {
                max_stress = Some(LocatedQuantity {
                    value: round6(value),
                    unit: "MPa".to_string(),
                    location_ref: pipe.element_id.clone(),
                    result_ref: result_id.clone(),
                });
            }
            results.push(ResultItem {
                id: result_id,
                kind: "open_formula_stress_summary".to_string(),
                value: round6(value),
                unit: "MPa".to_string(),
                entity_ref: pipe.element_id.clone(),
                basis_ref: None,
                source_result_refs: Vec::new(),
                metadata: None,
            });
        }
        component_stress_modifier_count += append_component_stress_multiplier_results(
            &mut results,
            diagnostics,
            model,
            &pipe.element_id,
            &end_i_stress,
            &end_j_stress,
            include_pressure_longitudinal,
        );
    }

    Ok(LoadCaseSolve {
        load_case_id: load_case.id.clone(),
        results,
        max_displacement,
        max_stress,
        component_stress_modifier_count,
        component_pressure_thrust_load_count,
    })
}

#[derive(Debug, Default)]
struct NonlinearSupportBuild {
    supports: Vec<NonlinearSupport>,
    initial_states: Vec<SupportStateRecord>,
    friction_normal_reactions: Vec<FrictionNormalReaction>,
    derived_friction_normal_reactions: Vec<DerivedFrictionNormalReaction>,
}

fn append_nonlinear_support_loop_results(
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
    built: &BuiltModel,
    restrained_dofs: &[usize],
    force: &[f64],
    load_case: &PreviewLoadCase,
    solver_mode: PreviewSolverMode,
) {
    if built.nonlinear_supports.is_empty() {
        return;
    }

    // The assembled active-set loop consumes the same build-time validated
    // curved-bend macro-element global stiffness as the linear preview paths
    // (DEC-070): each realized bend is passed as an explicit-stiffness slot so
    // every linearized iteration assembles the arc stiffness. No straight-chord
    // fallback exists on this path.
    let mut curved_bend_stiffness_elements = Vec::with_capacity(built.curved_bend_elements.len());
    for element in &built.curved_bend_elements {
        match CurvedBendStiffnessElement::new(
            element.component_id.clone(),
            element.node_i,
            element.node_j,
            element.global_stiffness,
        ) {
            Ok(slot) => curved_bend_stiffness_elements.push(slot),
            Err(error) => {
                diagnostics.push(nonlinear_loop_blocked_diag(&load_case.id, error));
                return;
            }
        }
    }

    let support_classes = product_preview_policy_support_classes(&built.nonlinear_supports);
    let support_classes_basis = support_classes.join(",");
    let convergence = match product_preview_convergence_control() {
        Ok(convergence) => convergence,
        Err(error) => {
            diagnostics.push(nonlinear_loop_blocked_diag(&load_case.id, error));
            return;
        }
    };

    let input = NonlinearFrameSolveInput {
        node_count: built.nodes.len(),
        elements: built.frame_elements.clone(),
        user_stiffness_elements: built.user_stiffness_elements.clone(),
        curved_bend_elements: curved_bend_stiffness_elements,
        force: force.to_vec(),
        base_restrained_dofs: restrained_dofs.to_vec(),
        nonlinear_supports: built.nonlinear_supports.clone(),
        initial_states: built.nonlinear_initial_states.clone(),
        friction_normal_reactions: built.nonlinear_friction_normal_reactions.clone(),
        derived_friction_normal_reactions: built
            .nonlinear_derived_friction_normal_reactions
            .clone(),
        convergence,
    };

    match solve_active_set_frame_with_mode(&input, solver_mode.nonlinear_mode()) {
        Ok(solve) => {
            for (index, solver_diagnostic) in solve.diagnostics.iter().enumerate() {
                diagnostics.push(product_diag_from_solver_diag(
                    solver_diagnostic,
                    &load_case.id,
                    index,
                ));
            }
            let final_residual = solve
                .iterations
                .last()
                .map(|iteration| iteration.active_set.residual_norm)
                .unwrap_or(0.0);
            append_nonlinear_scalar_result(
                results,
                "result:nonlinear-support:iteration-count",
                "nonlinear_support_active_set_iteration_count",
                solve.iterations.len() as f64,
                "count",
                "nonlinear_supports",
                "active_set_iteration_count",
                "load_case",
                &format!(
                    "{}_active_set_loop; policy_ref={}; policy_status=accepted; support_count={}; support_classes={}",
                    solver_mode.as_str(),
                    solve.policy_ref,
                    built.nonlinear_supports.len(),
                    support_classes_basis
                ),
                "counts completed dense active-set linearization iterations",
            );
            append_nonlinear_scalar_result(
                results,
                "result:nonlinear-support:final-residual-count",
                "nonlinear_support_active_set_final_residual_count",
                final_residual,
                "count",
                "nonlinear_supports",
                "active_set_final_residual_count",
                "load_case",
                &format!(
                    "{}_active_set_loop; policy_ref={}; residual_is_changed_support_count",
                    solver_mode.as_str(),
                    solve.policy_ref
                ),
                "zero means no nonlinear support state changed in the final iteration",
            );
            append_nonlinear_scalar_result(
                results,
                "result:nonlinear-support:converged-flag",
                "nonlinear_support_active_set_converged_flag",
                if solve.converged { 1.0 } else { 0.0 },
                "boolean",
                "nonlinear_supports",
                "active_set_converged_flag",
                "load_case",
                &format!(
                    "{}_active_set_loop; policy_ref={}; policy_status=accepted; residual_is_changed_support_count",
                    solver_mode.as_str(),
                    solve.policy_ref
                ),
                "1 means the active-set state-change residual satisfied the supplied preview tolerance",
            );
            if let Some(final_iteration) = solve.iterations.last() {
                append_nonlinear_residual_observation_results(
                    results,
                    &final_iteration.residuals,
                    &solve.policy_ref,
                    solver_mode,
                );
            }
            append_nonlinear_friction_normal_evidence(
                results,
                &built.nonlinear_friction_normal_reactions,
                &built.nonlinear_derived_friction_normal_reactions,
                &solve.reactions,
                &solve.policy_ref,
                solver_mode,
            );

            for state in &solve.final_states {
                let Some(support) = built
                    .nonlinear_supports
                    .iter()
                    .find(|candidate| candidate.support_id == state.support_id)
                else {
                    continue;
                };
                let global = support.node_index * DOF_PER_NODE + dof_index(support.dof);
                let suffix = stable_suffix(&support.support_id);
                let dof = support.dof.as_str();
                append_nonlinear_scalar_result(
                    results,
                    &format!("result:nonlinear-support:{suffix}:state-code"),
                    "nonlinear_support_active_set_state_code",
                    active_set_state_code(state.state),
                    "state_code",
                    &support.support_id,
                    "active_set_state_code",
                    dof,
                    &format!(
                        "{}_active_set_loop; policy_ref={}; final_state={}",
                        solver_mode.as_str(),
                        solve.policy_ref,
                        state.state.as_str()
                    ),
                    "0=inactive, 1=active, 2=sticking, 3=sliding",
                );
                let displacement_scale = if support.dof.is_translational() {
                    1000.0
                } else {
                    1.0
                };
                let displacement_unit = if support.dof.is_translational() {
                    "mm"
                } else {
                    "rad"
                };
                append_nonlinear_scalar_result(
                    results,
                    &format!("result:nonlinear-support:{suffix}:{dof}-displacement"),
                    "nonlinear_support_final_displacement",
                    solve.displacements[global] * displacement_scale,
                    displacement_unit,
                    &support.support_id,
                    "nonlinear_support_final_displacement",
                    dof,
                    &format!(
                        "{}_active_set_loop; policy_ref={}; final_state={}",
                        solver_mode.as_str(),
                        solve.policy_ref,
                        state.state.as_str()
                    ),
                    "positive value follows the global frame DOF sign convention",
                );
                let reaction_unit = if support.dof.is_translational() {
                    "N"
                } else {
                    "N*m"
                };
                append_nonlinear_scalar_result(
                    results,
                    &format!("result:nonlinear-support:{suffix}:{dof}-reaction"),
                    "nonlinear_support_final_reaction",
                    solve.reactions[global],
                    reaction_unit,
                    &support.support_id,
                    "nonlinear_support_final_reaction",
                    dof,
                    &format!(
                        "{}_active_set_loop; policy_ref={}; final_state={}",
                        solver_mode.as_str(),
                        solve.policy_ref,
                        state.state.as_str()
                    ),
                    "positive value follows the global frame DOF reaction sign convention",
                );
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:nonlinear:{}:{}:state",
                        stable_suffix(&load_case.id),
                        suffix
                    ),
                    "NONLINEAR_SUPPORT_STATE_REVIEW",
                    "info",
                    format!(
                        "nonlinear support {} ended in {} state for load case {}; {} preview loop evidence only",
                        support.support_id,
                        state.state.as_str(),
                        load_case.id,
                        solver_mode.as_str()
                    ),
                    vec![support.support_id.clone(), load_case.id.clone()],
                ));
            }

            diagnostics.push(diag(
                &format!(
                    "diagnostic:nonlinear:{}:loop",
                    stable_suffix(&load_case.id)
                ),
                if solve.converged {
                    "NONLINEAR_SUPPORT_LOOP_CONVERGED"
                } else {
                    "NONLINEAR_SUPPORT_LOOP_NOT_CONVERGED"
                },
                if solve.converged { "info" } else { "warning" },
                format!(
                    "{} nonlinear support active-set preview completed {} iteration(s); final residual count {}; accepted active-set-count policy_ref={}; accepted free-DOF force/moment residual policy_ref={}; accepted free-DOF work residual policy_ref={}; accepted general-energy residual policy_ref={}; accepted displacement/reaction-delta policy_ref={} for emitted product-preview delta rows; timing/RSS/hardware sparse evidence remains observational, not thresholded",
                    solver_mode.as_str(),
                    solve.iterations.len(),
                    final_residual,
                    solve.policy_ref,
                    DEC_046_PRODUCT_PREVIEW_FREE_DOF_FORCE_MOMENT_POLICY_REF,
                    DEC_046_PRODUCT_PREVIEW_FREE_DOF_WORK_POLICY_REF,
                    DEC_046_PRODUCT_PREVIEW_GENERAL_ENERGY_POLICY_REF,
                    DEC_046_PRODUCT_PREVIEW_DISPLACEMENT_REACTION_DELTA_POLICY_REF
                ),
                vec![load_case.id.clone(), "DEC-046".to_string()],
            ));
        }
        Err(error) => diagnostics.push(nonlinear_loop_blocked_diag(&load_case.id, error)),
    }
}

fn product_preview_convergence_control() -> Result<ConvergenceControl, NonlinearIntegrationError> {
    ConvergenceControl::new(
        DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_POLICY_REF,
        ConvergencePolicyStatus::Accepted,
        DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_RESIDUAL_TOLERANCE,
        DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_ABSOLUTE_FLOOR,
        DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_MAX_ITERATIONS,
    )
}

fn product_preview_policy_support_classes(supports: &[NonlinearSupport]) -> Vec<&'static str> {
    let mut classes = supports
        .iter()
        .map(|support| match support.behavior {
            NonlinearSupportBehavior::OneWay { .. } => "one_way",
            NonlinearSupportBehavior::Gap { .. } => "gap",
            NonlinearSupportBehavior::LiftOff { .. } => "lift_off",
            NonlinearSupportBehavior::Friction => "friction",
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    classes.sort_unstable();
    classes
}

#[derive(Debug, Clone)]
struct PreviewLinearSolve {
    solution: Vec<f64>,
    solution_basis: &'static str,
    sparse_entry_count: Option<usize>,
    original_profile_entry_count: Option<usize>,
    ordered_profile_entry_count: Option<usize>,
    original_max_half_bandwidth: Option<usize>,
    ordered_max_half_bandwidth: Option<usize>,
    nonpositive_pivot_count: Option<usize>,
    pivot_condition_ratio_estimate: Option<f64>,
    sparse_residual: Option<f64>,
    dense_fallback_message: Option<String>,
}

fn solve_preview_reduced_system(
    solver_mode: PreviewSolverMode,
    reduced_stiffness: &[Vec<f64>],
    reduced_force: &[f64],
    built: &BuiltModel,
    spring_entries: &[SpringEntry],
    global_force: &[f64],
    restrained_dofs: &[usize],
    load_case: &PreviewLoadCase,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<PreviewLinearSolve, FrameKernelError> {
    match solver_mode {
        PreviewSolverMode::SparseInteractive => {
            let direct_system = match assemble_reduced_sparse_entry_system(
                built.nodes.len(),
                &built.frame_elements,
                &built.user_stiffness_elements,
                &built.curved_bend_elements,
                spring_entries,
                global_force,
                restrained_dofs,
            ) {
                Ok(system) => system,
                Err(error) => {
                    let message = error.to_string();
                    diagnostics.push(sparse_interactive_fallback_diag(&load_case.id, &message));
                    let solution = solve_dense(reduced_stiffness, reduced_force)?;
                    return Ok(PreviewLinearSolve {
                        solution,
                        solution_basis: "dense_fallback_after_sparse_failure",
                        sparse_entry_count: None,
                        original_profile_entry_count: None,
                        ordered_profile_entry_count: None,
                        original_max_half_bandwidth: None,
                        ordered_max_half_bandwidth: None,
                        nonpositive_pivot_count: None,
                        pivot_condition_ratio_estimate: None,
                        sparse_residual: None,
                        dense_fallback_message: Some(message),
                    });
                }
            };
            match solve_symmetric_system_from_entries(
                direct_system.dimension,
                &direct_system.entries,
                &direct_system.force,
            ) {
                Ok(sparse) => {
                    let sparse_residual = max_abs_entry_residual(
                        direct_system.dimension,
                        &direct_system.entries,
                        &sparse.solution,
                        &direct_system.force,
                    );
                    Ok(PreviewLinearSolve {
                        solution: sparse.solution,
                        solution_basis: "sparse_profile_direct_primary",
                        sparse_entry_count: Some(direct_system.entries.len()),
                        original_profile_entry_count: Some(sparse.original_profile_entry_count),
                        ordered_profile_entry_count: Some(sparse.ordered_profile_entry_count),
                        original_max_half_bandwidth: Some(sparse.original_max_half_bandwidth),
                        ordered_max_half_bandwidth: Some(sparse.ordered_max_half_bandwidth),
                        nonpositive_pivot_count: Some(sparse.factorization.nonpositive_pivot_count),
                        pivot_condition_ratio_estimate: sparse
                            .factorization
                            .pivot_condition_ratio_estimate,
                        sparse_residual: Some(sparse_residual),
                        dense_fallback_message: None,
                    })
                }
                Err(error) => {
                    let message = error.to_string();
                    diagnostics.push(sparse_interactive_fallback_diag(&load_case.id, &message));
                    Ok(PreviewLinearSolve {
                        solution: solve_dense(reduced_stiffness, reduced_force)?,
                        solution_basis: "dense_fallback_after_sparse_failure",
                        sparse_entry_count: Some(direct_system.entries.len()),
                        original_profile_entry_count: None,
                        ordered_profile_entry_count: None,
                        original_max_half_bandwidth: None,
                        ordered_max_half_bandwidth: None,
                        nonpositive_pivot_count: None,
                        pivot_condition_ratio_estimate: None,
                        sparse_residual: None,
                        dense_fallback_message: Some(message),
                    })
                }
            }
        }
        PreviewSolverMode::DenseScrutiny => Ok(PreviewLinearSolve {
            solution: solve_dense(reduced_stiffness, reduced_force)?,
            solution_basis: "dense_scrutiny_primary",
            sparse_entry_count: None,
            original_profile_entry_count: None,
            ordered_profile_entry_count: None,
            original_max_half_bandwidth: None,
            ordered_max_half_bandwidth: None,
            nonpositive_pivot_count: None,
            pivot_condition_ratio_estimate: None,
            sparse_residual: None,
            dense_fallback_message: None,
        }),
    }
}

fn append_linear_solver_mode_evidence(
    results: &mut Vec<ResultItem>,
    load_case_id: &str,
    solver_mode: PreviewSolverMode,
    linear_solve: &PreviewLinearSolve,
) {
    let fallback = linear_solve.dense_fallback_message.is_some();
    let basis = format!(
        "DEC-053 sparse_default_promotion; solver_mode={}; solution_basis={}; default_sparse_promotion=interactive_default; dense_scrutiny_available=true; sparse_entry_count={}; original_profile_entries={}; ordered_profile_entries={}; original_half_bandwidth={}; ordered_half_bandwidth={}; nonpositive_pivots={}; pivot_condition_ratio_proxy={}; max_abs_sparse_residual={}; dense_fallback={}; dense_fallback_message={}",
        solver_mode.as_str(),
        linear_solve.solution_basis,
        optional_usize(linear_solve.sparse_entry_count),
        optional_usize(linear_solve.original_profile_entry_count),
        optional_usize(linear_solve.ordered_profile_entry_count),
        optional_usize(linear_solve.original_max_half_bandwidth),
        optional_usize(linear_solve.ordered_max_half_bandwidth),
        optional_usize(linear_solve.nonpositive_pivot_count),
        optional_f64(linear_solve.pivot_condition_ratio_estimate),
        optional_f64(linear_solve.sparse_residual),
        fallback,
        linear_solve
            .dense_fallback_message
            .as_deref()
            .unwrap_or("none")
    );

    results.push(ResultItem {
        id: "result:solver-mode:linear-solve-basis".to_string(),
        kind: "linear_solver_mode_basis".to_string(),
        value: if fallback { 3.0 } else { solver_mode.mode_code() },
        unit: "mode_code".to_string(),
        entity_ref: "solver:linear_static_preview".to_string(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            component: "linear_solver_mode".to_string(),
            coordinate_system: "reduced_system".to_string(),
            location: load_case_id.to_string(),
            basis,
            sign_convention:
                "mode_code 1=sparse_interactive, 2=dense_scrutiny, 3=dense_fallback_after_sparse_failure"
                    .to_string(),
        }),
    });
}

fn sparse_interactive_fallback_diag(load_case_id: &str, message: &str) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:sparse-interactive:{}:dense-fallback",
            stable_suffix(load_case_id)
        ),
        "SPARSE_INTERACTIVE_DENSE_FALLBACK",
        "warning",
        format!(
            "DEC-053 sparse interactive solve could not complete for load case {load_case_id}; dense fallback was used and the result basis is explicit: {message}"
        ),
        vec![load_case_id.to_string(), "DEC-053".to_string()],
    )
}

fn optional_usize(value: Option<usize>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "not_observed".to_string())
}

fn optional_f64(value: Option<f64>) -> String {
    value
        .map(|value| round6(value).to_string())
        .unwrap_or_else(|| "not_observed".to_string())
}

fn append_sparse_live_path_evidence(
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
    load_case_id: &str,
    built: &BuiltModel,
    spring_entries: &[SpringEntry],
    force: &[f64],
    restrained_dofs: &[usize],
    dense_solution: &[f64],
) {
    let direct_system = match assemble_reduced_sparse_entry_system(
        built.nodes.len(),
        &built.frame_elements,
        &built.user_stiffness_elements,
        &built.curved_bend_elements,
        spring_entries,
        force,
        restrained_dofs,
    ) {
        Ok(system) => system,
        Err(error) => {
            diagnostics.push(diag(
                &format!("diagnostic:sparse-live:{}:unavailable", stable_suffix(load_case_id)),
                "SPARSE_LIVE_PATH_EVIDENCE_UNAVAILABLE",
                "warning",
                format!(
                    "DEC-050 direct profile sparse evidence lane could not assemble load case {load_case_id}; dense solve remains the default product path and parity oracle: {error}"
                ),
                vec![load_case_id.to_string(), "DEC-050".to_string()],
            ));
            return;
        }
    };
    if direct_system.dimension != dense_solution.len() {
        diagnostics.push(diag(
            &format!("diagnostic:sparse-live:{}:unavailable", stable_suffix(load_case_id)),
            "SPARSE_LIVE_PATH_EVIDENCE_UNAVAILABLE",
            "warning",
            format!(
                "DEC-050 direct profile sparse evidence lane dimension {} did not match dense solution length {}; dense solve remains the default product path and parity oracle",
                direct_system.dimension,
                dense_solution.len()
            ),
            vec![load_case_id.to_string(), "DEC-050".to_string()],
        ));
        return;
    }

    let sparse = match solve_symmetric_system_from_entries(
        direct_system.dimension,
        &direct_system.entries,
        &direct_system.force,
    ) {
        Ok(sparse) => sparse,
        Err(error) => {
            diagnostics.push(diag(
                &format!("diagnostic:sparse-live:{}:unavailable", stable_suffix(load_case_id)),
                "SPARSE_LIVE_PATH_EVIDENCE_UNAVAILABLE",
                "warning",
                format!(
                    "DEC-050 direct profile sparse evidence lane could not observe load case {load_case_id}; dense solve remains the default product path and parity oracle: {error}"
                ),
                vec![load_case_id.to_string(), "DEC-050".to_string()],
            ));
            return;
        }
    };
    let max_abs_dense_sparse_solution_delta = max_abs_delta(dense_solution, &sparse.solution);
    let dense_scale = max_abs_value(dense_solution);
    let relative_dense_sparse_solution_delta = if dense_scale > 0.0 {
        max_abs_dense_sparse_solution_delta / dense_scale
    } else {
        max_abs_dense_sparse_solution_delta
    };
    let sparse_residual = max_abs_entry_residual(
        direct_system.dimension,
        &direct_system.entries,
        &sparse.solution,
        &direct_system.force,
    );

    results.push(ResultItem {
        id: "result:sparse-live:dense-parity-relative-delta".to_string(),
        kind: "sparse_live_path_dense_parity_relative_delta".to_string(),
        value: round6(relative_dense_sparse_solution_delta),
        unit: "unitless".to_string(),
        entity_ref: "solver:sparse_direct".to_string(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            component: "sparse_live_path".to_string(),
            coordinate_system: "reduced_system".to_string(),
            location: "load_case".to_string(),
            basis: format!(
                "DEC-053 dense_scrutiny_sparse_parity; solver_mode=dense_scrutiny; sparse_interactive_default=true; dense_parity_oracle=true; solver=core/solver/sparse_direct; assembly=direct_reduced_profile_entries; ordering=reverse_cuthill_mckee; reduced_dofs={}; sparse_entry_count={}; original_profile_entries={}; ordered_profile_entries={}; original_half_bandwidth={}; ordered_half_bandwidth={}; max_abs_dense_sparse_delta={}; max_abs_sparse_residual={}; nonpositive_pivots={}; profile_direct_assembly=observed; default_sparse_promotion=interactive_default",
                sparse.solution.len(),
                direct_system.entries.len(),
                sparse.original_profile_entry_count,
                sparse.ordered_profile_entry_count,
                sparse.original_max_half_bandwidth,
                sparse.ordered_max_half_bandwidth,
                round6(max_abs_dense_sparse_solution_delta),
                round6(sparse_residual),
                sparse.factorization.nonpositive_pivot_count
            ),
            sign_convention:
                "unitless max absolute dense-sparse solution delta divided by max dense solution magnitude; no release threshold asserted"
                    .to_string(),
        }),
    });
}

#[derive(Debug, Clone)]
struct ReducedSparseEntrySystem {
    dimension: usize,
    entries: Vec<SymmetricMatrixEntry>,
    force: Vec<f64>,
}

fn assemble_reduced_sparse_entry_system(
    node_count: usize,
    frame_elements: &[FrameElement],
    user_stiffness_elements: &[UserStiffnessElement],
    curved_bend_elements: &[CurvedBendMacroBuild],
    spring_entries: &[SpringEntry],
    force: &[f64],
    restrained_dofs: &[usize],
) -> Result<ReducedSparseEntrySystem, FrameKernelError> {
    let total_dofs = node_count * DOF_PER_NODE;
    if force.len() != total_dofs {
        return Err(FrameKernelError::InvalidVectorLength {
            expected: total_dofs,
            actual: force.len(),
        });
    }
    for &value in force {
        if !value.is_finite() {
            return Err(FrameKernelError::NonFiniteInput {
                name: "force entry",
                value,
            });
        }
    }

    let mut constrained = vec![false; total_dofs];
    for &dof in restrained_dofs {
        if dof >= total_dofs {
            return Err(FrameKernelError::RestrainedDofOutOfRange { dof, total_dofs });
        }
        if constrained[dof] {
            return Err(FrameKernelError::RepeatedRestrainedDof { dof });
        }
        constrained[dof] = true;
    }

    let mut global_to_reduced = vec![None; total_dofs];
    let mut reduced_force = Vec::new();
    for global_dof in 0..total_dofs {
        if constrained[global_dof] {
            continue;
        }
        global_to_reduced[global_dof] = Some(reduced_force.len());
        reduced_force.push(force[global_dof]);
    }

    let mut entries = Vec::new();
    for element in frame_elements {
        validate_element_nodes(element.node_i.index, element.node_j.index, node_count)?;
        let element_stiffness = element.global_stiffness()?;
        append_reduced_element_entries(
            &mut entries,
            &global_to_reduced,
            element.node_i.index,
            element.node_j.index,
            &element_stiffness,
        )?;
    }
    for element in user_stiffness_elements {
        validate_element_nodes(element.node_i.index, element.node_j.index, node_count)?;
        let element_stiffness = element.global_stiffness()?;
        append_reduced_element_entries(
            &mut entries,
            &global_to_reduced,
            element.node_i.index,
            element.node_j.index,
            &element_stiffness,
        )?;
    }
    for element in curved_bend_elements {
        validate_element_nodes(element.node_i, element.node_j, node_count)?;
        append_reduced_element_entries(
            &mut entries,
            &global_to_reduced,
            element.node_i,
            element.node_j,
            &element.global_stiffness,
        )?;
    }
    for spring in spring_entries {
        if spring.node_dof.node_index >= node_count {
            return Err(FrameKernelError::InvalidNodeIndex {
                node_index: spring.node_dof.node_index,
                node_count,
            });
        }
        let global_dof = spring.node_dof.global_index();
        if let Some(reduced_dof) = global_to_reduced[global_dof] {
            entries.push(SymmetricMatrixEntry {
                row: reduced_dof,
                col: reduced_dof,
                value: spring.stiffness.value,
            });
        }
    }

    Ok(ReducedSparseEntrySystem {
        dimension: reduced_force.len(),
        entries,
        force: reduced_force,
    })
}

// Dense-path assembly of the curved-bend macro-element stiffness beside the
// frame and user-stiffness elements (the replaced chord element is excluded in
// `build_model`, so the arc contribution is never double-counted).
fn add_curved_bend_stiffness_contributions(
    stiffness: &mut [Vec<f64>],
    curved_bend_elements: &[CurvedBendMacroBuild],
) {
    for element in curved_bend_elements {
        let dof_map = element_dof_map(element.node_i, element.node_j);
        for (local_row, &global_row) in dof_map.iter().enumerate() {
            for (local_col, &global_col) in dof_map.iter().enumerate() {
                stiffness[global_row][global_col] += element.global_stiffness[local_row][local_col];
            }
        }
    }
}

fn validate_element_nodes(
    node_i: usize,
    node_j: usize,
    node_count: usize,
) -> Result<(), FrameKernelError> {
    if node_i >= node_count {
        return Err(FrameKernelError::InvalidNodeIndex {
            node_index: node_i,
            node_count,
        });
    }
    if node_j >= node_count {
        return Err(FrameKernelError::InvalidNodeIndex {
            node_index: node_j,
            node_count,
        });
    }
    Ok(())
}

fn append_reduced_element_entries(
    entries: &mut Vec<SymmetricMatrixEntry>,
    global_to_reduced: &[Option<usize>],
    node_i: usize,
    node_j: usize,
    element_stiffness: &Matrix12,
) -> Result<(), FrameKernelError> {
    let dof_map = element_dof_map(node_i, node_j);
    for local_row in 0..ELEMENT_DOF {
        let global_row = dof_map[local_row];
        let Some(reduced_row) = global_to_reduced[global_row] else {
            continue;
        };
        for local_col in 0..=local_row {
            let global_col = dof_map[local_col];
            let Some(reduced_col) = global_to_reduced[global_col] else {
                continue;
            };
            let value = element_stiffness[local_row][local_col];
            if !value.is_finite() {
                return Err(FrameKernelError::NonFiniteInput {
                    name: "matrix entry",
                    value,
                });
            }
            if value == 0.0 {
                continue;
            }
            let (row, col) = if reduced_row >= reduced_col {
                (reduced_row, reduced_col)
            } else {
                (reduced_col, reduced_row)
            };
            entries.push(SymmetricMatrixEntry { row, col, value });
        }
    }
    Ok(())
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
        .fold(0.0, f64::max)
}

fn append_nonlinear_scalar_result(
    results: &mut Vec<ResultItem>,
    id: &str,
    kind: &str,
    value: f64,
    unit: &str,
    entity_ref: &str,
    component: &str,
    location: &str,
    basis: &str,
    sign_convention: &str,
) {
    results.push(ResultItem {
        id: id.to_string(),
        kind: kind.to_string(),
        value: round6(value),
        unit: unit.to_string(),
        entity_ref: entity_ref.to_string(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            component: component.to_string(),
            coordinate_system: "solver_iteration".to_string(),
            location: location.to_string(),
            basis: basis.to_string(),
            sign_convention: sign_convention.to_string(),
        }),
    });
}

fn append_nonlinear_residual_observation_results(
    results: &mut Vec<ResultItem>,
    residuals: &NonlinearResidualObservation,
    policy_ref: &str,
    solver_mode: PreviewSolverMode,
) {
    let displacement_reaction_delta_threshold_basis = format!(
        "{}_active_set_loop; policy_ref={policy_ref}; observation_ref={}; threshold_policy_ref={}; threshold_policy_status=accepted; residual_basis=displacement_reaction_delta_from_previous_iteration; threshold_axes=displacement_and_reaction_delta; translation_delta_threshold={} mm; rotation_delta_threshold={} rad; force_reaction_delta_threshold={} N; moment_reaction_delta_threshold={} N*m; product_preview_only",
        solver_mode.as_str(),
        DEC_046_PRODUCT_PREVIEW_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF,
        DEC_046_PRODUCT_PREVIEW_DISPLACEMENT_REACTION_DELTA_POLICY_REF,
        DEC_046_PRODUCT_PREVIEW_TRANSLATION_DELTA_ABSOLUTE_LIMIT_MM,
        DEC_046_PRODUCT_PREVIEW_ROTATION_DELTA_ABSOLUTE_LIMIT_RAD,
        DEC_046_PRODUCT_PREVIEW_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT_N,
        DEC_046_PRODUCT_PREVIEW_MOMENT_REACTION_DELTA_ABSOLUTE_LIMIT_N_M
    );
    let free_dof_work_threshold_basis = format!(
        "{}_active_set_loop; policy_ref={policy_ref}; threshold_policy_ref={}; threshold_policy_status=accepted; residual_basis=free_dof_work_residual; work_threshold={} N*m; general_energy_threshold_policy_ref={}; general_energy_threshold_policy_status=accepted; general_energy_threshold={} N*m; product_preview_only",
        solver_mode.as_str(),
        DEC_046_PRODUCT_PREVIEW_FREE_DOF_WORK_POLICY_REF,
        DEC_046_PRODUCT_PREVIEW_FREE_DOF_WORK_ABSOLUTE_LIMIT,
        DEC_046_PRODUCT_PREVIEW_GENERAL_ENERGY_POLICY_REF,
        DEC_046_PRODUCT_PREVIEW_GENERAL_ENERGY_ABSOLUTE_LIMIT
    );
    let force_moment_threshold_basis = format!(
        "{}_active_set_loop; policy_ref={policy_ref}; threshold_policy_ref={}; threshold_policy_status=accepted; residual_basis=free_dof_force_moment_equilibrium; force_threshold={} N; moment_threshold={} N*m; product_preview_only",
        solver_mode.as_str(),
        DEC_046_PRODUCT_PREVIEW_FREE_DOF_FORCE_MOMENT_POLICY_REF,
        DEC_046_PRODUCT_PREVIEW_FREE_DOF_FORCE_ABSOLUTE_LIMIT,
        DEC_046_PRODUCT_PREVIEW_FREE_DOF_MOMENT_ABSOLUTE_LIMIT
    );
    if let Some(value) = residuals.max_abs_translation_delta_from_previous {
        append_nonlinear_scalar_result(
            results,
            "result:nonlinear-support:max-translation-delta",
            "nonlinear_support_observed_max_translation_delta",
            value * 1000.0,
            "mm",
            "nonlinear_supports",
            "observed_max_translation_delta",
            "final_iteration",
            &displacement_reaction_delta_threshold_basis,
            "nonnegative max absolute translational displacement change from the previous iteration",
        );
    }
    if let Some(value) = residuals.max_abs_rotation_delta_from_previous {
        append_nonlinear_scalar_result(
            results,
            "result:nonlinear-support:max-rotation-delta",
            "nonlinear_support_observed_max_rotation_delta",
            value,
            "rad",
            "nonlinear_supports",
            "observed_max_rotation_delta",
            "final_iteration",
            &displacement_reaction_delta_threshold_basis,
            "nonnegative max absolute rotational displacement change from the previous iteration",
        );
    }
    if let Some(value) = residuals.max_abs_force_reaction_delta_from_previous {
        append_nonlinear_scalar_result(
            results,
            "result:nonlinear-support:max-force-reaction-delta",
            "nonlinear_support_observed_max_force_reaction_delta",
            value,
            "N",
            "nonlinear_supports",
            "observed_max_force_reaction_delta",
            "final_iteration",
            &displacement_reaction_delta_threshold_basis,
            "nonnegative max absolute translational reaction change from the previous iteration",
        );
    }
    if let Some(value) = residuals.max_abs_moment_reaction_delta_from_previous {
        append_nonlinear_scalar_result(
            results,
            "result:nonlinear-support:max-moment-reaction-delta",
            "nonlinear_support_observed_max_moment_reaction_delta",
            value,
            "N*m",
            "nonlinear_supports",
            "observed_max_moment_reaction_delta",
            "final_iteration",
            &displacement_reaction_delta_threshold_basis,
            "nonnegative max absolute rotational reaction change from the previous iteration",
        );
    }
    append_nonlinear_scalar_result(
        results,
        "result:nonlinear-support:free-dof-force-residual",
        "nonlinear_support_observed_free_dof_force_residual",
        residuals.max_abs_free_dof_force_residual,
        "N",
        "nonlinear_supports",
        "observed_free_dof_force_residual",
        "final_iteration",
        &force_moment_threshold_basis,
        "nonnegative max absolute translational free-DOF equilibrium residual in the final linearized solve",
    );
    append_nonlinear_scalar_result(
        results,
        "result:nonlinear-support:free-dof-moment-residual",
        "nonlinear_support_observed_free_dof_moment_residual",
        residuals.max_abs_free_dof_moment_residual,
        "N*m",
        "nonlinear_supports",
        "observed_free_dof_moment_residual",
        "final_iteration",
        &force_moment_threshold_basis,
        "nonnegative max absolute rotational free-DOF equilibrium residual in the final linearized solve",
    );
    append_nonlinear_scalar_result(
        results,
        "result:nonlinear-support:free-dof-work-residual",
        "nonlinear_support_free_dof_work_residual",
        residuals.max_abs_free_dof_work_residual,
        "N*m",
        "nonlinear_supports",
        "free_dof_work_residual",
        "final_iteration",
        &free_dof_work_threshold_basis,
        "nonnegative max absolute free-DOF residual work product in the final linearized solve",
    );
}

fn nonlinear_loop_blocked_diag(load_case_id: &str, error: NonlinearIntegrationError) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:nonlinear:{}:blocked",
            stable_suffix(load_case_id)
        ),
        "NONLINEAR_SUPPORT_LOOP_BLOCKED",
        "blocking",
        format!("nonlinear support dense preview loop could not run: {error}"),
        vec![load_case_id.to_string()],
    )
}

fn product_diag_from_solver_diag(
    diagnostic: &SolverDiagnostic,
    load_case_id: &str,
    index: usize,
) -> Diagnostic {
    let code = solver_diag_code(diagnostic.code);
    let mut affected_refs = vec![load_case_id.to_string()];
    if let Some(affected_ref) = &diagnostic.affected_ref {
        affected_refs.push(affected_ref.clone());
    }
    diag(
        &format!(
            "diagnostic:nonlinear:{}:{}:{}",
            stable_suffix(load_case_id),
            index + 1,
            stable_suffix(code)
        ),
        code,
        solver_diag_severity(diagnostic.severity),
        diagnostic.message.clone(),
        affected_refs,
    )
}

fn solver_diag_code(code: SolverDiagnosticCode) -> &'static str {
    match code {
        SolverDiagnosticCode::SingularSystem => "SOLVER_SINGULAR_SYSTEM",
        SolverDiagnosticCode::IllConditionedSystem => "SOLVER_ILL_CONDITIONED_SYSTEM",
        SolverDiagnosticCode::ConditioningFailure => "SOLVER_CONDITIONING_FAILURE",
        SolverDiagnosticCode::InvalidRestraint => "SOLVER_INVALID_RESTRAINT",
        SolverDiagnosticCode::InvalidModelTopology => "SOLVER_INVALID_MODEL_TOPOLOGY",
        SolverDiagnosticCode::InvalidNumericInput => "SOLVER_INVALID_NUMERIC_INPUT",
        SolverDiagnosticCode::NonConvergence => "NONLINEAR_SUPPORT_NONCONVERGENCE",
        SolverDiagnosticCode::NonPositivePivot => "SOLVER_NONPOSITIVE_PIVOT",
        SolverDiagnosticCode::SparseSolverTbd => "SPARSE_SOLVER_TBD",
        SolverDiagnosticCode::TolerancePolicyTbd => "TOLERANCE_POLICY_TBD",
    }
}

fn solver_diag_severity(severity: SolverDiagnosticSeverity) -> &'static str {
    match severity {
        SolverDiagnosticSeverity::Info => "info",
        SolverDiagnosticSeverity::Warning => "warning",
        SolverDiagnosticSeverity::Blocking | SolverDiagnosticSeverity::Failure => "blocking",
    }
}

fn active_set_state_code(state: ActiveSetState) -> f64 {
    match state {
        ActiveSetState::Inactive => 0.0,
        ActiveSetState::Active => 1.0,
        ActiveSetState::Sticking => 2.0,
        ActiveSetState::Sliding => 3.0,
    }
}

fn derived_friction_normal_source(
    friction_support_id: &str,
    source: &FrictionNormalReactionSourceInput,
    support_by_id: &HashMap<&str, &PreviewSupport>,
    node_map: &HashMap<&str, usize>,
) -> Result<DerivedFrictionNormalReaction, Diagnostic> {
    let Some(source_support) = support_by_id.get(source.support_ref.as_str()) else {
        return Err(diag(
            &format!(
                "diagnostic:nonlinear-support:{}:normal-source",
                stable_suffix(friction_support_id)
            ),
            "NONLINEAR_FRICTION_NORMAL_SOURCE_UNKNOWN",
            "blocking",
            "friction normal_reaction_source support_ref is not present in preview model",
            vec![friction_support_id.to_string(), source.support_ref.clone()],
        ));
    };
    if source_support.nonlinear.is_some() {
        return Err(diag(
            &format!(
                "diagnostic:nonlinear-support:{}:normal-source",
                stable_suffix(friction_support_id)
            ),
            "NONLINEAR_FRICTION_NORMAL_SOURCE_INVALID",
            "blocking",
            "friction normal_reaction_source must reference a linear support restraint",
            vec![friction_support_id.to_string(), source.support_ref.clone()],
        ));
    }
    let source_dof = parse_dof(&source.dof).map_err(|message| {
        diag(
            &format!(
                "diagnostic:nonlinear-support:{}:normal-source-dof",
                stable_suffix(friction_support_id)
            ),
            "NONLINEAR_FRICTION_NORMAL_SOURCE_DOF_INVALID",
            "blocking",
            message,
            vec![friction_support_id.to_string(), source.dof.clone()],
        )
    })?;
    if !source_dof.is_translational() {
        return Err(diag(
            &format!(
                "diagnostic:nonlinear-support:{}:normal-source-dof",
                stable_suffix(friction_support_id)
            ),
            "NONLINEAR_FRICTION_NORMAL_SOURCE_DOF_INVALID",
            "blocking",
            "friction normal_reaction_source must reference a translational support DOF",
            vec![friction_support_id.to_string(), source.dof.clone()],
        ));
    }
    let source_has_restraint = source_support
        .restraints
        .iter()
        .filter_map(|dof| parse_dof(dof).ok())
        .any(|dof| dof == source_dof);
    if !source_has_restraint {
        return Err(diag(
            &format!(
                "diagnostic:nonlinear-support:{}:normal-source-dof",
                stable_suffix(friction_support_id)
            ),
            "NONLINEAR_FRICTION_NORMAL_SOURCE_DOF_UNRESTRAINED",
            "blocking",
            "friction normal_reaction_source must reference a restrained support DOF",
            vec![
                friction_support_id.to_string(),
                source.support_ref.clone(),
                source.dof.clone(),
            ],
        ));
    }
    let Some(&source_node_index) = node_map.get(source_support.node.as_str()) else {
        return Err(diag(
            &format!(
                "diagnostic:nonlinear-support:{}:normal-source-node",
                stable_suffix(friction_support_id)
            ),
            "NONLINEAR_FRICTION_NORMAL_SOURCE_NODE_UNKNOWN",
            "blocking",
            "friction normal_reaction_source support node is not present in preview model",
            vec![
                friction_support_id.to_string(),
                source.support_ref.clone(),
                source_support.node.clone(),
            ],
        ));
    };
    DerivedFrictionNormalReaction::from_support_reaction(
        friction_support_id,
        source_node_index,
        source_dof,
        &source.support_ref,
    )
    .map_err(|error| {
        diag(
            &format!(
                "diagnostic:nonlinear-support:{}:normal-source",
                stable_suffix(friction_support_id)
            ),
            "NONLINEAR_FRICTION_NORMAL_SOURCE_INVALID",
            "blocking",
            error.to_string(),
            vec![friction_support_id.to_string(), source.support_ref.clone()],
        )
    })
}

fn build_nonlinear_supports(
    model: &PreviewModel,
    node_map: &HashMap<&str, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) -> NonlinearSupportBuild {
    let mut build = NonlinearSupportBuild::default();
    let support_by_id = model
        .supports
        .iter()
        .map(|support| (support.id.as_str(), support))
        .collect::<HashMap<_, _>>();
    for support in &model.supports {
        let Some(input) = &support.nonlinear else {
            continue;
        };
        let Some(&node_index) = node_map.get(support.node.as_str()) else {
            diagnostics.push(diag(
                &format!("diagnostic:support:{}:node", stable_suffix(&support.id)),
                "SUPPORT_NODE_UNKNOWN",
                "blocking",
                "nonlinear support node is not present in preview model",
                vec![support.id.clone(), support.node.clone()],
            ));
            continue;
        };
        let dof = match parse_dof(&input.dof) {
            Ok(dof) => dof,
            Err(message) => {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:nonlinear-support:{}:dof",
                        stable_suffix(&support.id)
                    ),
                    "NONLINEAR_SUPPORT_DOF_INVALID",
                    "blocking",
                    message,
                    vec![support.id.clone(), input.dof.clone()],
                ));
                continue;
            }
        };
        let Some(initial_state_value) = input.initial_state.as_deref() else {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:nonlinear-support:{}:initial-state",
                    stable_suffix(&support.id)
                ),
                "NONLINEAR_SUPPORT_INITIAL_STATE_MISSING",
                "blocking",
                "nonlinear support requires an explicit initial active-set state",
                vec![support.id.clone()],
            ));
            continue;
        };
        let initial_state = match parse_active_set_state(initial_state_value) {
            Ok(state) => state,
            Err(message) => {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:nonlinear-support:{}:initial-state",
                        stable_suffix(&support.id)
                    ),
                    "NONLINEAR_SUPPORT_INITIAL_STATE_INVALID",
                    "blocking",
                    message,
                    vec![support.id.clone(), initial_state_value.to_string()],
                ));
                continue;
            }
        };

        let nonlinear_support = match input.behavior.as_str() {
            "one_way" | "one-way" | "oneway" => {
                let Some(active_when) = input.active_when.as_deref() else {
                    diagnostics.push(missing_nonlinear_field_diag(&support.id, "active_when"));
                    continue;
                };
                match parse_activation_sense(active_when) {
                    Ok(sense) => NonlinearSupport::one_way(&support.id, node_index, dof, sense),
                    Err(message) => {
                        diagnostics.push(invalid_nonlinear_field_diag(
                            &support.id,
                            "active_when",
                            active_when,
                            message,
                        ));
                        continue;
                    }
                }
            }
            "gap" => {
                let Some(closes_when) = input.closes_when.as_deref() else {
                    diagnostics.push(missing_nonlinear_field_diag(&support.id, "closes_when"));
                    continue;
                };
                let closes_when = match parse_gap_direction(closes_when) {
                    Ok(direction) => direction,
                    Err(message) => {
                        diagnostics.push(invalid_nonlinear_field_diag(
                            &support.id,
                            "closes_when",
                            closes_when,
                            message,
                        ));
                        continue;
                    }
                };
                let Some(gap) = input.gap.as_ref() else {
                    diagnostics.push(missing_nonlinear_field_diag(&support.id, "gap"));
                    continue;
                };
                match NonlinearSupport::gap(&support.id, node_index, dof, gap.value, closes_when) {
                    Ok(support) => support,
                    Err(error) => {
                        diagnostics.push(diag(
                            &format!(
                                "diagnostic:nonlinear-support:{}:gap",
                                stable_suffix(&support.id)
                            ),
                            "NONLINEAR_SUPPORT_INPUT_INVALID",
                            "blocking",
                            error.to_string(),
                            vec![support.id.clone()],
                        ));
                        continue;
                    }
                }
            }
            "lift_off" | "lift-off" | "liftoff" => {
                let Some(contact_when) = input.contact_when.as_deref() else {
                    diagnostics.push(missing_nonlinear_field_diag(&support.id, "contact_when"));
                    continue;
                };
                match parse_activation_sense(contact_when) {
                    Ok(sense) => NonlinearSupport::lift_off(&support.id, node_index, dof, sense),
                    Err(message) => {
                        diagnostics.push(invalid_nonlinear_field_diag(
                            &support.id,
                            "contact_when",
                            contact_when,
                            message,
                        ));
                        continue;
                    }
                }
            }
            "friction" => {
                let Some(coefficient) = input.friction_coefficient.as_ref() else {
                    diagnostics.push(missing_nonlinear_field_diag(
                        &support.id,
                        "friction_coefficient",
                    ));
                    continue;
                };
                if input.normal_reaction.is_some() && input.normal_reaction_source.is_some() {
                    diagnostics.push(diag(
                        &format!(
                            "diagnostic:nonlinear-support:{}:normal-reaction",
                            stable_suffix(&support.id)
                        ),
                        "NONLINEAR_FRICTION_NORMAL_REACTION_AMBIGUOUS",
                        "blocking",
                        "friction nonlinear support must use either explicit normal_reaction or normal_reaction_source, not both",
                        vec![support.id.clone()],
                    ));
                    continue;
                }
                if let Some(normal_reaction) = input.normal_reaction.as_ref() {
                    match FrictionNormalReaction::new(&support.id, normal_reaction.value) {
                        Ok(reaction) => build.friction_normal_reactions.push(reaction),
                        Err(error) => {
                            diagnostics.push(diag(
                                &format!(
                                    "diagnostic:nonlinear-support:{}:normal-reaction",
                                    stable_suffix(&support.id)
                                ),
                                "NONLINEAR_FRICTION_NORMAL_REACTION_INVALID",
                                "blocking",
                                error.to_string(),
                                vec![support.id.clone()],
                            ));
                            continue;
                        }
                    }
                } else if let Some(source) = input.normal_reaction_source.as_ref() {
                    match derived_friction_normal_source(
                        &support.id,
                        source,
                        &support_by_id,
                        node_map,
                    ) {
                        Ok(source) => build.derived_friction_normal_reactions.push(source),
                        Err(diagnostic) => {
                            diagnostics.push(diagnostic);
                            continue;
                        }
                    }
                } else {
                    diagnostics.push(diag(
                        &format!(
                            "diagnostic:nonlinear-support:{}:normal-reaction",
                            stable_suffix(&support.id)
                        ),
                        "NONLINEAR_FRICTION_NORMAL_REACTION_MISSING",
                        "blocking",
                        "friction nonlinear support requires explicit normal_reaction or a normal_reaction_source support DOF",
                        vec![support.id.clone()],
                    ));
                    continue;
                }
                match NonlinearSupport::friction(&support.id, node_index, dof, coefficient.value) {
                    Ok(support) => support,
                    Err(error) => {
                        diagnostics.push(diag(
                            &format!(
                                "diagnostic:nonlinear-support:{}:friction",
                                stable_suffix(&support.id)
                            ),
                            "NONLINEAR_SUPPORT_INPUT_INVALID",
                            "blocking",
                            error.to_string(),
                            vec![support.id.clone()],
                        ));
                        continue;
                    }
                }
            }
            _ => {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:nonlinear-support:{}:behavior",
                        stable_suffix(&support.id)
                    ),
                    "NONLINEAR_SUPPORT_BEHAVIOR_INVALID",
                    "blocking",
                    format!("unsupported nonlinear support behavior {}", input.behavior),
                    vec![support.id.clone(), input.behavior.clone()],
                ));
                continue;
            }
        };

        build
            .initial_states
            .push(SupportStateRecord::new(&support.id, initial_state));
        build.supports.push(nonlinear_support);
    }
    build
}

fn build_model(
    model: &PreviewModel,
    materials: &[MaterialInput],
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<BuiltModel> {
    let material_map = materials
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect::<HashMap<_, _>>();
    let mut node_map = HashMap::new();
    let mut nodes = Vec::new();
    for (index, node) in model.nodes.iter().enumerate() {
        node_map.insert(node.id.as_str(), index);
        match FrameNode::new(index, [node.position.x, node.position.y, node.position.z]) {
            Ok(frame_node) => nodes.push(frame_node),
            Err(error) => diagnostics.push(diag(
                &format!("diagnostic:node:{}", stable_suffix(&node.id)),
                "NODE_INPUT_INVALID",
                "blocking",
                error.to_string(),
                vec![node.id.clone()],
            )),
        }
    }

    // Pipe spans realized as curved-bend macro-elements must not also carry
    // their straight chord element (no double stiffness); the macro-element
    // build below emits blocking diagnostics for every insufficiency, so an
    // excluded span never silently loses stiffness.
    let curved_bend_pipe_ids = curved_bend_realized_pipe_ids(model);
    let mut pipes = Vec::new();
    let mut frame_elements = Vec::new();
    let mut sections = HashMap::new();
    for pipe in &model.pipe_segments {
        let Some(&from) = node_map.get(pipe.from.as_str()) else {
            diagnostics.push(diag(
                &format!("diagnostic:pipe:{}:from", stable_suffix(&pipe.id)),
                "PIPE_ENDPOINT_UNKNOWN",
                "blocking",
                "pipe from-node is not present in preview model",
                vec![pipe.id.clone(), pipe.from.clone()],
            ));
            continue;
        };
        let Some(&to) = node_map.get(pipe.to.as_str()) else {
            diagnostics.push(diag(
                &format!("diagnostic:pipe:{}:to", stable_suffix(&pipe.id)),
                "PIPE_ENDPOINT_UNKNOWN",
                "blocking",
                "pipe to-node is not present in preview model",
                vec![pipe.id.clone(), pipe.to.clone()],
            ));
            continue;
        };
        let Some(material) = material_map.get(pipe.material.as_str()) else {
            diagnostics.push(diag(&format!("diagnostic:material:{}", stable_suffix(&pipe.material)), "MATERIAL_INPUT_MISSING", "blocking", "pipe material requires explicit elastic and shear modulus inputs; no defaults are applied", vec![pipe.id.clone(), pipe.material.clone()]));
            continue;
        };
        let Some(derived) = derive_pipe_section(&pipe.section, &pipe.id, diagnostics) else {
            continue;
        };
        let Some(y_reference) = pipe.y_reference else {
            diagnostics.push(diag(
                &format!("diagnostic:pipe-orientation:{}", stable_suffix(&pipe.id)),
                "PIPE_ORIENTATION_INPUT_MISSING",
                "blocking",
                "pipe orientation requires an explicit y_reference vector; no default orientation is applied",
                vec![pipe.id.clone()],
            ));
            continue;
        };
        let section = match StraightPipeSectionProperties::new(
            material.elastic_modulus.value,
            material.shear_modulus.value,
            derived.area,
            derived.second_moment,
            derived.second_moment,
            derived.torsion_constant,
            None,
        ) {
            Ok(section) => section,
            Err(error) => {
                diagnostics.push(diag(
                    &format!("diagnostic:pipe-section:{}", stable_suffix(&pipe.id)),
                    "PIPE_SECTION_INPUT_INVALID",
                    "blocking",
                    error.to_string(),
                    vec![pipe.id.clone()],
                ));
                continue;
            }
        };
        let y_ref = [y_reference.x, y_reference.y, y_reference.z];
        let element =
            match StraightPipeElement::new(&pipe.id, nodes[from], nodes[to], section, y_ref) {
                Ok(element) => element,
                Err(error) => {
                    diagnostics.push(diag(
                        &format!("diagnostic:pipe-element:{}", stable_suffix(&pipe.id)),
                        "PIPE_ELEMENT_INPUT_INVALID",
                        "blocking",
                        error.to_string(),
                        vec![pipe.id.clone()],
                    ));
                    continue;
                }
            };
        if !curved_bend_pipe_ids.contains(pipe.id.as_str()) {
            frame_elements.push(
                element
                    .frame_element()
                    .expect("validated straight pipe frame element"),
            );
        }
        sections.insert(pipe.id.clone(), derived);
        pipes.push(element);
    }

    let user_stiffness_elements =
        build_expansion_joint_user_stiffness_elements(model, &nodes, &node_map, diagnostics);
    let curved_bend_elements = build_curved_bend_macro_elements(
        model,
        materials,
        &nodes,
        &node_map,
        &sections,
        diagnostics,
    );
    let nonlinear = build_nonlinear_supports(model, &node_map, diagnostics);
    let supports = model
        .supports
        .iter()
        .filter_map(|support| {
            if support.nonlinear.is_some() {
                return None;
            }
            if is_constant_effort_support(support) {
                return None;
            }
            let Some(&node) = node_map.get(support.node.as_str()) else {
                diagnostics.push(diag(
                    &format!("diagnostic:support:{}:node", stable_suffix(&support.id)),
                    "SUPPORT_NODE_UNKNOWN",
                    "blocking",
                    "support node is not present in preview model",
                    vec![support.id.clone(), support.node.clone()],
                ));
                return None;
            };
            let dofs = support
                .restraints
                .iter()
                .filter_map(|dof| match parse_dof(dof) {
                    Ok(dof) => Some(dof),
                    Err(message) => {
                        diagnostics.push(diag(
                            &format!("diagnostic:support:{}:dof", stable_suffix(&support.id)),
                            "SUPPORT_DOF_INVALID",
                            "blocking",
                            message,
                            vec![support.id.clone()],
                        ));
                        None
                    }
                })
                .collect::<Vec<_>>();
            if support.family.as_deref() == Some("spring") || is_variable_spring_hanger(support) {
                let stiffness = support_stiffness_input(support).and_then(|input| {
                    let dimension = if parse_dof(&input.dof).ok()?.is_translational() {
                        QuantityDimension::TranslationalStiffness
                    } else {
                        QuantityDimension::RotationalStiffness
                    };
                    SupportQuantity::positive(input.value.value, dimension).ok()
                });
                Some(LinearSupport::spring(
                    &support.id,
                    node,
                    support_stiffness_input(support)
                        .and_then(|input| parse_dof(&input.dof).ok())
                        .or_else(|| dofs.first().copied())
                        .unwrap_or(FrameDof::Uz),
                    stiffness,
                ))
            } else if dofs.len() == 6 {
                Some(LinearSupport::anchor(&support.id, node))
            } else {
                Some(LinearSupport {
                    support_id: support.id.clone(),
                    family: SupportFamily::Guide,
                    node_index: node,
                    restrained_dofs: dofs,
                    stiffness: None,
                    imposed_displacement: None,
                })
            }
        })
        .collect::<Vec<_>>();

    Some(BuiltModel {
        nodes,
        pipes,
        frame_elements,
        user_stiffness_elements,
        curved_bend_elements,
        supports,
        nonlinear_supports: nonlinear.supports,
        nonlinear_initial_states: nonlinear.initial_states,
        nonlinear_friction_normal_reactions: nonlinear.friction_normal_reactions,
        nonlinear_derived_friction_normal_reactions: nonlinear.derived_friction_normal_reactions,
        sections,
    })
}

fn build_expansion_joint_user_stiffness_elements(
    model: &PreviewModel,
    nodes: &[FrameNode],
    node_map: &HashMap<&str, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<UserStiffnessElement> {
    let pipe_map = model
        .pipe_segments
        .iter()
        .map(|pipe| (pipe.id.as_str(), pipe))
        .collect::<HashMap<_, _>>();
    let mut elements = Vec::new();

    for component in model
        .components
        .iter()
        .filter(|component| is_expansion_joint_component(component))
    {
        let solver_consumption = component
            .mechanics_interface
            .as_ref()
            .and_then(|interface| interface.solver_consumption.as_deref())
            .unwrap_or("not_provided");
        if solver_consumption != "mechanics_geometry_and_user_flexibility" {
            continue;
        }

        let Some(geometry) = component.geometry.as_ref() else {
            continue;
        };
        let Some(modifiers) = component.modifiers.as_ref() else {
            continue;
        };
        let Some(pipe_ref) = geometry
            .expansion_joint_pipe_ref
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        else {
            continue;
        };
        let Some(pipe) = pipe_map.get(pipe_ref) else {
            continue;
        };
        let Some(&component_node_index) = node_map.get(component.node.as_str()) else {
            continue;
        };
        let Some(&from_index) = node_map.get(pipe.from.as_str()) else {
            continue;
        };
        let Some(&to_index) = node_map.get(pipe.to.as_str()) else {
            continue;
        };
        let other_node_index = if component_node_index == from_index {
            to_index
        } else if component_node_index == to_index {
            from_index
        } else {
            continue;
        };
        let Some(y_reference) = pipe.y_reference else {
            continue;
        };
        let Some(axial) = modifiers.axial_stiffness_user_value.as_ref() else {
            continue;
        };
        let Some(lateral) = modifiers.lateral_stiffness_user_value.as_ref() else {
            continue;
        };
        let Some(angular) = modifiers.angular_stiffness_user_value.as_ref() else {
            continue;
        };
        let Some(torsional) = modifiers.torsional_stiffness_user_value.as_ref() else {
            continue;
        };

        match UserStiffnessElement::new(
            nodes[other_node_index],
            nodes[component_node_index],
            [y_reference.x, y_reference.y, y_reference.z],
            axial.value,
            lateral.value,
            angular.value,
            torsional.value,
        ) {
            Ok(element) => elements.push(element),
            Err(error) => diagnostics.push(expansion_joint_macro_element_diag(
                component,
                pipe_ref,
                &error.to_string(),
            )),
        }
    }

    elements
}

fn expansion_joint_macro_element_diag(
    component: &PreviewComponent,
    pipe_ref: &str,
    message: &str,
) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:component:{}:macro-element",
            stable_suffix(&component.id)
        ),
        "EXPANSION_JOINT_MACRO_ELEMENT_INPUT_INVALID",
        "blocking",
        message,
        vec![component.id.clone(), pipe_ref.to_string()],
    )
}

fn component_solver_consumption(component: &PreviewComponent, default: &'static str) -> String {
    component
        .mechanics_interface
        .as_ref()
        .and_then(|interface| interface.solver_consumption.as_deref())
        .unwrap_or(default)
        .to_string()
}

fn is_curved_bend_macro_component(component: &PreviewComponent) -> bool {
    is_bend_component(component)
        && component_solver_consumption(component, "mechanics_geometry_only")
            == DEC_070_CURVED_BEND_SOLVER_CONSUMPTION
}

// Pipe spans whose straight chord element is replaced by a curved-bend
// macro-element. Membership requires only the mode and a resolvable pipe
// reference; every other insufficiency blocks in
// `build_curved_bend_macro_elements`, so exclusion never silently drops
// stiffness from a model that solves.
fn curved_bend_realized_pipe_ids(model: &PreviewModel) -> HashSet<&str> {
    model
        .components
        .iter()
        .filter(|component| is_curved_bend_macro_component(component))
        .filter_map(|component| {
            component
                .geometry
                .as_ref()
                .and_then(|geometry| geometry.bend_pipe_ref.as_deref())
                .filter(|value| !value.trim().is_empty())
        })
        .collect()
}

fn build_curved_bend_macro_elements(
    model: &PreviewModel,
    materials: &[MaterialInput],
    nodes: &[FrameNode],
    node_map: &HashMap<&str, usize>,
    sections: &HashMap<String, DerivedSection>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<CurvedBendMacroBuild> {
    let pipe_map = model
        .pipe_segments
        .iter()
        .enumerate()
        .map(|(index, pipe)| (pipe.id.as_str(), (index, pipe)))
        .collect::<HashMap<_, _>>();
    let material_map = materials
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect::<HashMap<_, _>>();
    let mut builds = Vec::new();

    for component in model
        .components
        .iter()
        .filter(|component| is_curved_bend_macro_component(component))
    {
        // The mode explicitly requests the assembled arc realization, so every
        // insufficiency below is a blocking diagnostic; there is no silent
        // straight-chord or multiplier-only fallback (PRD 6.2).
        let Some(geometry) = component.geometry.as_ref() else {
            diagnostics.push(curved_bend_geometry_missing_diag(
                component,
                "component geometry block with bend_pipe_ref and bend_radius",
            ));
            continue;
        };
        let Some(pipe_ref) = geometry
            .bend_pipe_ref
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        else {
            diagnostics.push(curved_bend_geometry_missing_diag(
                component,
                "geometry.bend_pipe_ref naming the realized bend span",
            ));
            continue;
        };
        let Some(&(pipe_index, pipe)) = pipe_map.get(pipe_ref) else {
            diagnostics.push(curved_bend_mapping_diag(
                component,
                pipe_ref,
                "bend_pipe_ref is not present in preview model pipe segments",
            ));
            continue;
        };
        let (Some(&from_index), Some(&to_index)) = (
            node_map.get(pipe.from.as_str()),
            node_map.get(pipe.to.as_str()),
        ) else {
            // The pipe loop already emitted the blocking endpoint diagnostic.
            continue;
        };
        let Some(&component_node_index) = node_map.get(component.node.as_str()) else {
            diagnostics.push(curved_bend_mapping_diag(
                component,
                pipe_ref,
                "component node is not present in preview model",
            ));
            continue;
        };
        if component_node_index != from_index && component_node_index != to_index {
            diagnostics.push(curved_bend_mapping_diag(
                component,
                pipe_ref,
                "component node must be an endpoint of the referenced bend span",
            ));
            continue;
        }
        let Some(y_reference) = pipe.y_reference else {
            // The pipe loop already emitted the blocking orientation diagnostic.
            continue;
        };
        let Some(modifiers) = component.modifiers.as_ref() else {
            diagnostics.push(curved_bend_geometry_missing_diag(
                component,
                "component modifiers block with flexibility_factor_user_value",
            ));
            continue;
        };
        let Some(flexibility) = modifiers
            .flexibility_factor_user_value
            .as_ref()
            .map(|quantity| quantity.value)
        else {
            diagnostics.push(curved_bend_geometry_missing_diag(
                component,
                "modifiers.flexibility_factor_user_value",
            ));
            continue;
        };
        if !positive_finite(flexibility) {
            diagnostics.push(curved_bend_macro_element_diag(
                component,
                pipe_ref,
                "user-entered flexibility factor must be a finite positive dimensionless value",
            ));
            continue;
        }
        let Some(bend_radius) = geometry.bend_radius.as_ref().map(|quantity| quantity.value) else {
            diagnostics.push(curved_bend_geometry_missing_diag(
                component,
                "geometry.bend_radius",
            ));
            continue;
        };
        if !positive_finite(bend_radius) {
            diagnostics.push(curved_bend_macro_element_diag(
                component,
                pipe_ref,
                "user-entered bend radius must be a finite positive length",
            ));
            continue;
        }
        let (Some(section), Some(material)) = (
            sections.get(pipe_ref),
            material_map.get(pipe.material.as_str()),
        ) else {
            // The pipe loop already emitted the blocking section/material diagnostic.
            continue;
        };

        // Arc geometry from user fields: the chord is the span's two nodes,
        // the bend plane is spanned by the chord and the pipe y_reference,
        // and the user bend radius fixes the sagitta. The arc center sits on
        // the negative-y_reference side of the chord midpoint, so the arc
        // bows toward the positive pipe y_reference side (recorded in the
        // review-row basis).
        let from_position = nodes[from_index].coordinates;
        let to_position = nodes[to_index].coordinates;
        let chord = [
            to_position[0] - from_position[0],
            to_position[1] - from_position[1],
            to_position[2] - from_position[2],
        ];
        let chord_length = (chord[0] * chord[0] + chord[1] * chord[1] + chord[2] * chord[2]).sqrt();
        if !(chord_length.is_finite() && chord_length > 0.0) {
            diagnostics.push(curved_bend_macro_element_diag(
                component,
                pipe_ref,
                "bend span chord length must be a finite positive value",
            ));
            continue;
        }
        let half_chord = 0.5 * chord_length;
        if bend_radius <= half_chord {
            diagnostics.push(curved_bend_inconsistent_diag(
                component,
                pipe_ref,
                &format!(
                    "user-entered bend radius {} m cannot span the {} m chord; the arc included angle would reach or exceed pi",
                    rounded_scalar(bend_radius),
                    rounded_scalar(chord_length)
                ),
            ));
            continue;
        }
        let implied_included_angle = 2.0 * (half_chord / bend_radius).asin();
        if let Some(user_angle) = geometry.bend_angle.as_ref().map(|quantity| quantity.value) {
            let angle_scale = user_angle.abs().max(implied_included_angle.abs()).max(1.0);
            if (user_angle - implied_included_angle).abs()
                > DEC_070_CURVED_BEND_ANGLE_MATCH_TOLERANCE * angle_scale
            {
                diagnostics.push(curved_bend_inconsistent_diag(
                    component,
                    pipe_ref,
                    &format!(
                        "user-entered bend angle {} rad disagrees with the included angle {} rad implied by the user chord and bend radius; make the chord, radius, and angle arc-consistent",
                        rounded_scalar(user_angle),
                        rounded_scalar(implied_included_angle)
                    ),
                ));
                continue;
            }
        }
        let chord_unit = [
            chord[0] / chord_length,
            chord[1] / chord_length,
            chord[2] / chord_length,
        ];
        let y_raw = [y_reference.x, y_reference.y, y_reference.z];
        let axial_component =
            y_raw[0] * chord_unit[0] + y_raw[1] * chord_unit[1] + y_raw[2] * chord_unit[2];
        let plane_normal = [
            y_raw[0] - axial_component * chord_unit[0],
            y_raw[1] - axial_component * chord_unit[1],
            y_raw[2] - axial_component * chord_unit[2],
        ];
        let plane_magnitude = (plane_normal[0] * plane_normal[0]
            + plane_normal[1] * plane_normal[1]
            + plane_normal[2] * plane_normal[2])
            .sqrt();
        if plane_magnitude <= DEC_070_CURVED_BEND_PLANE_TOLERANCE {
            diagnostics.push(curved_bend_inconsistent_diag(
                component,
                pipe_ref,
                "pipe y_reference is parallel to the bend span chord, so the user bend plane is undefined",
            ));
            continue;
        }
        let sagitta_offset = (bend_radius * bend_radius - half_chord * half_chord).sqrt();
        let center = [
            0.5 * (from_position[0] + to_position[0])
                - sagitta_offset * plane_normal[0] / plane_magnitude,
            0.5 * (from_position[1] + to_position[1])
                - sagitta_offset * plane_normal[1] / plane_magnitude,
            0.5 * (from_position[2] + to_position[2])
                - sagitta_offset * plane_normal[2] / plane_magnitude,
        ];

        // The single user flexibility factor applies to both the in-plane and
        // out-of-plane bending strain-energy terms (mapping recorded in the
        // review-row basis); no code-content value is derived or defaulted.
        let element = match CurvedBendMacroElement::new(
            nodes[from_index],
            nodes[to_index],
            center,
            material.elastic_modulus.value,
            material.shear_modulus.value,
            section.area,
            section.second_moment,
            section.torsion_constant,
            flexibility,
            flexibility,
        ) {
            Ok(element) => element,
            Err(error) => {
                diagnostics.push(curved_bend_macro_element_diag(
                    component,
                    pipe_ref,
                    &error.to_string(),
                ));
                continue;
            }
        };
        let (global_stiffness, arc_length, included_angle) = match (
            element.global_stiffness(),
            element.arc_length(),
            element.included_angle(),
        ) {
            (Ok(stiffness), Ok(arc_length), Ok(included_angle)) => {
                (stiffness, arc_length, included_angle)
            }
            (Err(error), _, _) | (_, Err(error), _) | (_, _, Err(error)) => {
                diagnostics.push(curved_bend_macro_element_diag(
                    component,
                    pipe_ref,
                    &error.to_string(),
                ));
                continue;
            }
        };

        builds.push(CurvedBendMacroBuild {
            component_id: component.id.clone(),
            pipe_id: pipe.id.clone(),
            pipe_index,
            node_i: from_index,
            node_j: to_index,
            chord,
            global_stiffness,
            arc_length,
            included_angle,
            bend_radius,
            flexibility_factor: flexibility,
            source_reference: modifiers
                .source_reference
                .as_deref()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("source_reference_missing")
                .to_string(),
            macro_element: element,
        });
    }

    builds
}

fn curved_bend_geometry_missing_diag(component: &PreviewComponent, field: &str) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:component:{}:curved-bend-geometry",
            stable_suffix(&component.id)
        ),
        "CURVED_BEND_GEOMETRY_INPUT_MISSING",
        "blocking",
        format!(
            "bend component {} requests solver_consumption={} but is missing {}; the curved-bend macro-element is never realized from defaults",
            component.id, DEC_070_CURVED_BEND_SOLVER_CONSUMPTION, field
        ),
        vec![component.id.clone()],
    )
}

fn curved_bend_mapping_diag(
    component: &PreviewComponent,
    pipe_ref: &str,
    message: &str,
) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:component:{}:curved-bend-mapping",
            stable_suffix(&component.id)
        ),
        "CURVED_BEND_MAPPING_INPUT_INVALID",
        "blocking",
        message,
        vec![component.id.clone(), pipe_ref.to_string()],
    )
}

fn curved_bend_macro_element_diag(
    component: &PreviewComponent,
    pipe_ref: &str,
    message: &str,
) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:component:{}:curved-bend-macro-element",
            stable_suffix(&component.id)
        ),
        "CURVED_BEND_MACRO_ELEMENT_INPUT_INVALID",
        "blocking",
        message,
        vec![component.id.clone(), pipe_ref.to_string()],
    )
}

fn curved_bend_inconsistent_diag(
    component: &PreviewComponent,
    pipe_ref: &str,
    message: &str,
) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:component:{}:curved-bend-geometry-inconsistent",
            stable_suffix(&component.id)
        ),
        "CURVED_BEND_GEOMETRY_INCONSISTENT",
        "blocking",
        message,
        vec![component.id.clone(), pipe_ref.to_string()],
    )
}

fn normalize_model_units(
    model: &mut PreviewModel,
    materials: &mut [MaterialInput],
    diagnostics: &mut Vec<Diagnostic>,
) {
    for material in materials {
        normalize_quantity(
            &mut material.elastic_modulus,
            Dimension::Stress,
            &format!(
                "diagnostic:unit-conversion:material:{}:elastic-modulus",
                stable_suffix(&material.id)
            ),
            vec![material.id.clone(), "elastic_modulus".to_string()],
            diagnostics,
        );
        normalize_quantity(
            &mut material.shear_modulus,
            Dimension::Stress,
            &format!(
                "diagnostic:unit-conversion:material:{}:shear-modulus",
                stable_suffix(&material.id)
            ),
            vec![material.id.clone(), "shear_modulus".to_string()],
            diagnostics,
        );
        if let Some(coefficient) = &mut material.thermal_expansion_coefficient {
            normalize_quantity(
                coefficient,
                Dimension::ThermalExpansionCoefficient,
                &format!(
                    "diagnostic:unit-conversion:material:{}:thermal-expansion",
                    stable_suffix(&material.id)
                ),
                vec![
                    material.id.clone(),
                    "thermal_expansion_coefficient".to_string(),
                ],
                diagnostics,
            );
        }
        for point in &mut material.temperature_points {
            let point_id = point.id.clone();
            if let Some(temperature) = &mut point.temperature {
                normalize_quantity(
                    temperature,
                    Dimension::Temperature,
                    &format!(
                        "diagnostic:unit-conversion:material:{}:temperature-point:{}:temperature",
                        stable_suffix(&material.id),
                        stable_suffix(&point_id)
                    ),
                    vec![material.id.clone(), point_id.clone()],
                    diagnostics,
                );
            }
            if let Some(elastic_modulus) = &mut point.elastic_modulus {
                normalize_quantity(
                    elastic_modulus,
                    Dimension::Stress,
                    &format!(
                        "diagnostic:unit-conversion:material:{}:temperature-point:{}:elastic-modulus",
                        stable_suffix(&material.id),
                        stable_suffix(&point_id)
                    ),
                    vec![material.id.clone(), point_id.clone()],
                    diagnostics,
                );
            }
            if let Some(coefficient) = &mut point.thermal_expansion_coefficient {
                normalize_quantity(
                    coefficient,
                    Dimension::ThermalExpansionCoefficient,
                    &format!(
                        "diagnostic:unit-conversion:material:{}:temperature-point:{}:thermal-expansion",
                        stable_suffix(&material.id),
                        stable_suffix(&point_id)
                    ),
                    vec![material.id.clone(), point_id.clone()],
                    diagnostics,
                );
            }
        }
    }

    for pipe in &mut model.pipe_segments {
        normalize_quantity(
            &mut pipe.section.outside_diameter,
            Dimension::Length,
            &format!(
                "diagnostic:unit-conversion:pipe:{}:outside-diameter",
                stable_suffix(&pipe.id)
            ),
            vec![pipe.id.clone(), "outside_diameter".to_string()],
            diagnostics,
        );
        normalize_quantity(
            &mut pipe.section.wall_thickness,
            Dimension::Length,
            &format!(
                "diagnostic:unit-conversion:pipe:{}:wall-thickness",
                stable_suffix(&pipe.id)
            ),
            vec![pipe.id.clone(), "wall_thickness".to_string()],
            diagnostics,
        );
        if let Some(mill_tolerance) = &mut pipe.section.mill_tolerance {
            normalize_quantity(
                mill_tolerance,
                Dimension::Length,
                &format!(
                    "diagnostic:unit-conversion:pipe:{}:mill-tolerance",
                    stable_suffix(&pipe.id)
                ),
                vec![pipe.id.clone(), "mill_tolerance".to_string()],
                diagnostics,
            );
        }
        if let Some(density) = &mut pipe.section.material_density {
            normalize_quantity(
                density,
                Dimension::Density,
                &format!(
                    "diagnostic:unit-conversion:pipe:{}:material-density",
                    stable_suffix(&pipe.id)
                ),
                vec![pipe.id.clone(), "material_density".to_string()],
                diagnostics,
            );
        }
        if let Some(density) = &mut pipe.section.contents_density {
            normalize_quantity(
                density,
                Dimension::Density,
                &format!(
                    "diagnostic:unit-conversion:pipe:{}:contents-density",
                    stable_suffix(&pipe.id)
                ),
                vec![pipe.id.clone(), "contents_density".to_string()],
                diagnostics,
            );
        }
        if let Some(thickness) = &mut pipe.section.insulation_thickness {
            normalize_quantity(
                thickness,
                Dimension::Length,
                &format!(
                    "diagnostic:unit-conversion:pipe:{}:insulation-thickness",
                    stable_suffix(&pipe.id)
                ),
                vec![pipe.id.clone(), "insulation_thickness".to_string()],
                diagnostics,
            );
        }
        if let Some(density) = &mut pipe.section.insulation_density {
            normalize_quantity(
                density,
                Dimension::Density,
                &format!(
                    "diagnostic:unit-conversion:pipe:{}:insulation-density",
                    stable_suffix(&pipe.id)
                ),
                vec![pipe.id.clone(), "insulation_density".to_string()],
                diagnostics,
            );
        }
    }

    for support in &mut model.supports {
        if let Some(stiffness) = &mut support.stiffness {
            let Some(dimension) = parse_dof(&stiffness.dof).ok().map(|dof| {
                if dof.is_translational() {
                    Dimension::LinearStiffness
                } else {
                    Dimension::RotationalStiffness
                }
            }) else {
                continue;
            };
            normalize_quantity(
                &mut stiffness.value,
                dimension,
                &format!(
                    "diagnostic:unit-conversion:support:{}:stiffness",
                    stable_suffix(&support.id)
                ),
                vec![support.id.clone(), "stiffness".to_string()],
                diagnostics,
            );
        }
        if let Some(hanger) = &mut support.hanger {
            if let Some(stiffness) = &mut hanger.stiffness {
                let Some(dimension) = parse_dof(&stiffness.dof).ok().map(|dof| {
                    if dof.is_translational() {
                        Dimension::LinearStiffness
                    } else {
                        Dimension::RotationalStiffness
                    }
                }) else {
                    continue;
                };
                normalize_quantity(
                    &mut stiffness.value,
                    dimension,
                    &format!(
                        "diagnostic:unit-conversion:support:{}:hanger-stiffness",
                        stable_suffix(&support.id)
                    ),
                    vec![support.id.clone(), "hanger.stiffness".to_string()],
                    diagnostics,
                );
            }
            for (field, quantity) in [
                ("hanger.installed_load", &mut hanger.installed_load),
                ("hanger.cold_load", &mut hanger.cold_load),
                ("hanger.hot_load", &mut hanger.hot_load),
                ("hanger.constant_load", &mut hanger.constant_load),
            ] {
                if let Some(quantity) = quantity {
                    normalize_quantity(
                        quantity,
                        Dimension::Force,
                        &format!(
                            "diagnostic:unit-conversion:support:{}:{}",
                            stable_suffix(&support.id),
                            stable_suffix(field)
                        ),
                        vec![support.id.clone(), field.to_string()],
                        diagnostics,
                    );
                }
            }
            for (field, quantity) in [
                ("hanger.travel_range", &mut hanger.travel_range),
                ("hanger.movement_limit", &mut hanger.movement_limit),
            ] {
                if let Some(quantity) = quantity {
                    normalize_quantity(
                        quantity,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit-conversion:support:{}:{}",
                            stable_suffix(&support.id),
                            stable_suffix(field)
                        ),
                        vec![support.id.clone(), field.to_string()],
                        diagnostics,
                    );
                }
            }
        }
        if let Some(nonlinear) = &mut support.nonlinear {
            if let Some(gap) = &mut nonlinear.gap {
                normalize_quantity(
                    gap,
                    Dimension::Length,
                    &format!(
                        "diagnostic:unit-conversion:support:{}:nonlinear-gap",
                        stable_suffix(&support.id)
                    ),
                    vec![support.id.clone(), "nonlinear.gap".to_string()],
                    diagnostics,
                );
            }
            if let Some(coefficient) = &mut nonlinear.friction_coefficient {
                normalize_dimensionless_quantity(
                    coefficient,
                    &format!(
                        "diagnostic:unit-conversion:support:{}:friction-coefficient",
                        stable_suffix(&support.id)
                    ),
                    vec![
                        support.id.clone(),
                        "nonlinear.friction_coefficient".to_string(),
                    ],
                    diagnostics,
                );
            }
            if let Some(normal) = &mut nonlinear.normal_reaction {
                normalize_quantity(
                    normal,
                    Dimension::Force,
                    &format!(
                        "diagnostic:unit-conversion:support:{}:normal-reaction",
                        stable_suffix(&support.id)
                    ),
                    vec![support.id.clone(), "nonlinear.normal_reaction".to_string()],
                    diagnostics,
                );
            }
        }
    }

    for component in &mut model.components {
        let is_bend = matches!(component.kind.as_str(), "bend" | "elbow");
        let is_branch = matches!(
            component.kind.as_str(),
            "branch" | "tee" | "branch_connection"
        );
        let is_rigid = matches!(
            component.kind.as_str(),
            "valve" | "flange" | "reducer" | "rigid" | "specialty"
        );
        let is_expansion_joint = component.kind == "expansion_joint";
        if let Some(geometry) = &mut component.geometry {
            if is_bend {
                if let Some(radius) = &mut geometry.bend_radius {
                    normalize_quantity(
                        radius,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:bend-radius",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.bend_radius".to_string()],
                        diagnostics,
                    );
                }
                if let Some(angle) = &mut geometry.bend_angle {
                    normalize_quantity(
                        angle,
                        Dimension::Angle,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:bend-angle",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.bend_angle".to_string()],
                        diagnostics,
                    );
                }
            }
            if is_branch {
                if let Some(size) = &mut geometry.branch_run_size {
                    normalize_quantity(
                        size,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:branch-run-size",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.branch_run_size".to_string()],
                        diagnostics,
                    );
                }
                if let Some(size) = &mut geometry.branch_header_size {
                    normalize_quantity(
                        size,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:branch-header-size",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "geometry.branch_header_size".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(angle) = &mut geometry.branch_connection_angle {
                    normalize_quantity(
                        angle,
                        Dimension::Angle,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:branch-connection-angle",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "geometry.branch_connection_angle".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(area) = &mut geometry.branch_reinforcement_area {
                    normalize_quantity(
                        area,
                        Dimension::Area,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:branch-reinforcement-area",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "geometry.branch_reinforcement_area".to_string(),
                        ],
                        diagnostics,
                    );
                }
            }
            if is_rigid {
                if let Some(length) = &mut geometry.rigid_body_length {
                    normalize_quantity(
                        length,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:rigid-body-length",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "geometry.rigid_body_length".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(size) = &mut geometry.end_a_size {
                    normalize_quantity(
                        size,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:end-a-size",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.end_a_size".to_string()],
                        diagnostics,
                    );
                }
                if let Some(size) = &mut geometry.end_b_size {
                    normalize_quantity(
                        size,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:end-b-size",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.end_b_size".to_string()],
                        diagnostics,
                    );
                }
                if let Some(weight) = &mut geometry.weight {
                    normalize_quantity(
                        weight,
                        Dimension::Force,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:weight",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.weight".to_string()],
                        diagnostics,
                    );
                }
                if let Some(cog) = &mut geometry.center_of_gravity {
                    normalize_vector_quantity(
                        cog,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:center-of-gravity",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "geometry.center_of_gravity".to_string(),
                        ],
                        diagnostics,
                    );
                }
            }
            if is_expansion_joint {
                if let Some(area) = &mut geometry.effective_area {
                    normalize_quantity(
                        area,
                        Dimension::Area,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:effective-area",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.effective_area".to_string()],
                        diagnostics,
                    );
                }
                if let Some(limit) = &mut geometry.movement_limit {
                    normalize_quantity(
                        limit,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:movement-limit",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.movement_limit".to_string()],
                        diagnostics,
                    );
                }
            }
        }
        if let Some(modifiers) = &mut component.modifiers {
            if is_bend {
                if let Some(sif) = &mut modifiers.sif_user_value {
                    normalize_dimensionless_quantity(
                        sif,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:sif-user-value",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "modifiers.sif_user_value".to_string()],
                        diagnostics,
                    );
                }
            }
            if is_branch {
                if let Some(sif) = &mut modifiers.branch_header_sif_user_value {
                    normalize_dimensionless_quantity(
                        sif,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:branch-header-sif",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.branch_header_sif_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(sif) = &mut modifiers.branch_branch_sif_user_value {
                    normalize_dimensionless_quantity(
                        sif,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:branch-branch-sif",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.branch_branch_sif_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
            }
            if is_bend || is_branch {
                if let Some(flexibility) = &mut modifiers.flexibility_factor_user_value {
                    normalize_dimensionless_quantity(
                        flexibility,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:flexibility-factor",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.flexibility_factor_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
            } else if let Some(sif) = &mut modifiers.sif_user_value {
                normalize_dimensionless_quantity(
                    sif,
                    &format!(
                        "diagnostic:unit-conversion:component:{}:sif-user-value",
                        stable_suffix(&component.id)
                    ),
                    vec![component.id.clone(), "modifiers.sif_user_value".to_string()],
                    diagnostics,
                );
            } else if let Some(flexibility) = &mut modifiers.flexibility_factor_user_value {
                normalize_dimensionless_quantity(
                    flexibility,
                    &format!(
                        "diagnostic:unit-conversion:component:{}:flexibility-factor",
                        stable_suffix(&component.id)
                    ),
                    vec![
                        component.id.clone(),
                        "modifiers.flexibility_factor_user_value".to_string(),
                    ],
                    diagnostics,
                );
            }
            if is_rigid {
                if let Some(scale) = &mut modifiers.stiffness_scaling_user_value {
                    normalize_dimensionless_quantity(
                        scale,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:stiffness-scaling",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.stiffness_scaling_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(stiffness) = &mut modifiers.linear_stiffness_user_value {
                    normalize_quantity(
                        stiffness,
                        Dimension::LinearStiffness,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:linear-stiffness",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.linear_stiffness_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(stiffness) = &mut modifiers.rotational_stiffness_user_value {
                    normalize_quantity(
                        stiffness,
                        Dimension::RotationalStiffness,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:rotational-stiffness",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.rotational_stiffness_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
            }
            if is_expansion_joint {
                if let Some(stiffness) = &mut modifiers.axial_stiffness_user_value {
                    normalize_quantity(
                        stiffness,
                        Dimension::LinearStiffness,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:axial-stiffness",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.axial_stiffness_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(stiffness) = &mut modifiers.lateral_stiffness_user_value {
                    normalize_quantity(
                        stiffness,
                        Dimension::LinearStiffness,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:lateral-stiffness",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.lateral_stiffness_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(stiffness) = &mut modifiers.angular_stiffness_user_value {
                    normalize_quantity(
                        stiffness,
                        Dimension::RotationalStiffness,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:angular-stiffness",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.angular_stiffness_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(stiffness) = &mut modifiers.torsional_stiffness_user_value {
                    normalize_quantity(
                        stiffness,
                        Dimension::RotationalStiffness,
                        &format!(
                            "diagnostic:unit-conversion:component:{}:torsional-stiffness",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.torsional_stiffness_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
            }
        }
    }

    for load in model
        .load_cases
        .iter_mut()
        .flat_map(|case| case.primitive_loads.iter_mut())
    {
        if let Some(dimension) = expected_load_dimension(&load.dimension) {
            normalize_quantity(
                &mut load.magnitude,
                dimension,
                &format!(
                    "diagnostic:unit-conversion:load:{}:magnitude",
                    stable_suffix(&load.id)
                ),
                vec![load.id.clone(), "magnitude".to_string()],
                diagnostics,
            );
        }
    }
    for load_case in &mut model.load_cases {
        if let Some(temperature) = &mut load_case.modulus_basis_temperature {
            normalize_quantity(
                temperature,
                Dimension::Temperature,
                &format!(
                    "diagnostic:unit-conversion:load-case:{}:modulus-basis-temperature",
                    stable_suffix(&load_case.id)
                ),
                vec![
                    load_case.id.clone(),
                    "modulus_basis_temperature".to_string(),
                ],
                diagnostics,
            );
        }
        let Some(equivalent_static) = &mut load_case.equivalent_static else {
            continue;
        };
        let case_id = load_case.id.clone();
        if let Some(seismic) = &mut equivalent_static.seismic {
            if let Some(gravity) = &mut seismic.gravity_acceleration {
                normalize_quantity(
                    gravity,
                    Dimension::Acceleration,
                    &format!(
                        "diagnostic:unit-conversion:load-case:{}:gravity-acceleration",
                        stable_suffix(&case_id)
                    ),
                    vec![case_id.clone(), "gravity_acceleration".to_string()],
                    diagnostics,
                );
            }
            for (label, factor) in [
                ("g_factor_x", &mut seismic.g_factor_x),
                ("g_factor_y", &mut seismic.g_factor_y),
                ("g_factor_z", &mut seismic.g_factor_z),
            ] {
                if let Some(factor) = factor {
                    normalize_dimensionless_quantity(
                        factor,
                        &format!(
                            "diagnostic:unit-conversion:load-case:{}:{label}",
                            stable_suffix(&case_id)
                        ),
                        vec![case_id.clone(), label.to_string()],
                        diagnostics,
                    );
                }
            }
        }
        if let Some(wind) = &mut equivalent_static.wind {
            if let Some(pressure) = &mut wind.pressure {
                normalize_quantity(
                    pressure,
                    Dimension::Pressure,
                    &format!(
                        "diagnostic:unit-conversion:load-case:{}:wind-pressure",
                        stable_suffix(&case_id)
                    ),
                    vec![case_id.clone(), "wind.pressure".to_string()],
                    diagnostics,
                );
            }
            if let Some(shape_factor) = &mut wind.shape_factor {
                normalize_dimensionless_quantity(
                    shape_factor,
                    &format!(
                        "diagnostic:unit-conversion:load-case:{}:wind-shape-factor",
                        stable_suffix(&case_id)
                    ),
                    vec![case_id.clone(), "wind.shape_factor".to_string()],
                    diagnostics,
                );
            }
        }
    }
}

fn normalize_dimensionless_quantity(
    quantity: &mut Quantity,
    diagnostic_id: &str,
    affected_refs: Vec<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if quantity.unit == "none" {
        quantity.unit = "1".to_string();
        return;
    }
    normalize_quantity(
        quantity,
        Dimension::Dimensionless,
        diagnostic_id,
        affected_refs,
        diagnostics,
    );
}

fn normalize_quantity(
    quantity: &mut Quantity,
    dimension: Dimension,
    diagnostic_id: &str,
    affected_refs: Vec<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !quantity.value.is_finite() {
        return;
    }
    let from = match unit_by_symbol(&quantity.unit, dimension) {
        Ok(unit) => unit,
        Err(error) => {
            diagnostics.push(unit_conversion_diag(
                diagnostic_id,
                error.to_string(),
                affected_refs,
            ));
            return;
        }
    };
    let Some(to) = canonical_unit(dimension) else {
        diagnostics.push(unit_conversion_diag(
            diagnostic_id,
            format!("no canonical unit is accepted for {}", dimension.as_str()),
            affected_refs,
        ));
        return;
    };
    match convert_for_dimension(quantity.value, dimension, from, to) {
        Ok(converted) => {
            quantity.value = converted;
            quantity.unit = to.definition().symbol.to_string();
        }
        Err(error) => diagnostics.push(unit_conversion_diag(
            diagnostic_id,
            error.to_string(),
            affected_refs,
        )),
    }
}

fn normalize_vector_quantity(
    quantity: &mut VectorQuantity,
    dimension: Dimension,
    diagnostic_id: &str,
    affected_refs: Vec<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let original_unit = quantity.unit.clone();
    let mut x = Quantity {
        value: quantity.x,
        unit: original_unit.clone(),
    };
    let mut y = Quantity {
        value: quantity.y,
        unit: original_unit.clone(),
    };
    let mut z = Quantity {
        value: quantity.z,
        unit: original_unit,
    };
    normalize_quantity(
        &mut x,
        dimension,
        diagnostic_id,
        affected_refs.clone(),
        diagnostics,
    );
    normalize_quantity(
        &mut y,
        dimension,
        diagnostic_id,
        affected_refs.clone(),
        diagnostics,
    );
    normalize_quantity(&mut z, dimension, diagnostic_id, affected_refs, diagnostics);
    quantity.x = x.value;
    quantity.y = y.value;
    quantity.z = z.value;
    quantity.unit = x.unit;
}

fn unit_conversion_diag(
    diagnostic_id: &str,
    message: impl Into<String>,
    affected_refs: Vec<String>,
) -> Diagnostic {
    diag(
        diagnostic_id,
        "UNIT_CONVERSION_UNAVAILABLE",
        "blocking",
        format!(
            "preview mechanics could not normalize unit-bearing input to the DEC-018 SI-canonical solver boundary: {}",
            message.into()
        ),
        affected_refs,
    )
}

fn build_load_case_primitive_loads(
    model: &PreviewModel,
    load_case: &PreviewLoadCase,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<PrimitiveLoad> {
    let node_map = model
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.id.as_str(), i))
        .collect::<HashMap<_, _>>();
    let pipe_map = model
        .pipe_segments
        .iter()
        .enumerate()
        .map(|(i, p)| (p.id.as_str(), i))
        .collect::<HashMap<_, _>>();

    let mut loads = load_case
        .primitive_loads
        .iter()
        .filter_map(|load| {
            let category = parse_category(&load.category);
            let direction = parse_direction(&load.direction);
            let dimension = parse_load_dimension(&load.dimension);
            match (category, direction, dimension) {
                (Ok(category), Ok(direction), Ok(dimension)) => {
                    if let Some(preview) = authored_category_preview_mapping(&load.category) {
                        diagnostics.push(diag(
                            &format!(
                                "diagnostic:load:{}:category-mapping",
                                stable_suffix(&load.id)
                            ),
                            "LOAD_CATEGORY_PREVIEW_MAPPED",
                            "warning",
                            format!(
                                "authored load category {} is applied under the equivalent-static preview category {preview}; the preview classification is not a user-selected engineering classification",
                                load.category
                            ),
                            vec![load.id.clone(), load_case.id.clone()],
                        ));
                    }
                    let Ok(quantity) = LoadQuantity::new(load.magnitude.value, dimension) else {
                        diagnostics.push(diag(
                            &format!("diagnostic:load:{}:quantity", stable_suffix(&load.id)),
                            "LOAD_MAGNITUDE_INVALID",
                            "blocking",
                            "load magnitude must be finite",
                            vec![load.id.clone(), load_case.id.clone()],
                        ));
                        return None;
                    };
                    match &load.target {
                        LoadTargetInput::Node { node } => {
                            let Some(&node_index) = node_map.get(node.as_str()) else {
                                diagnostics.push(diag(
                                    &format!("diagnostic:load:{}:node", stable_suffix(&load.id)),
                                    "LOAD_NODE_UNKNOWN",
                                    "blocking",
                                    "load target node is not present in preview model",
                                    vec![load.id.clone(), node.clone(), load_case.id.clone()],
                                ));
                                return None;
                            };
                            Some(PrimitiveLoad::nodal_force(
                                &load.id, category, node_index, direction, quantity,
                            ))
                        }
                        LoadTargetInput::Element { pipe } => {
                            let Some(&pipe_index) = pipe_map.get(pipe.as_str()) else {
                                diagnostics.push(diag(
                                    &format!("diagnostic:load:{}:pipe", stable_suffix(&load.id)),
                                    "LOAD_PIPE_UNKNOWN",
                                    "blocking",
                                    "load target pipe is not present in preview model",
                                    vec![load.id.clone(), pipe.clone(), load_case.id.clone()],
                                ));
                                return None;
                            };
                            Some(PrimitiveLoad::uniform_element_load(
                                &load.id, category, pipe_index, direction, quantity,
                            ))
                        }
                    }
                }
                _ => {
                    diagnostics.push(diag(
                        &format!("diagnostic:load:{}:kind", stable_suffix(&load.id)),
                        "LOAD_INPUT_INVALID",
                        "blocking",
                        "load category, direction, and dimension must use supported preview values",
                        vec![load.id.clone(), load_case.id.clone()],
                    ));
                    None
                }
            }
        })
        .collect::<Vec<_>>();
    append_equivalent_static_generated_loads(model, load_case, &mut loads, diagnostics);
    loads
}

/// Compute one pipe's mass per unit length from its own user-entered
/// section inputs, mirroring `core/section_properties/calculator.py`
/// (metal + contents + insulation over the mill-tolerance-reduced
/// effective wall). Absent optional inputs contribute nothing; the
/// required inputs for the requesting generation path are diagnosed by
/// the caller.
fn compute_pipe_mass_per_length(
    pipe: &PreviewPipe,
    load_case_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<f64> {
    let section = &pipe.section;
    let od = section.outside_diameter.value;
    let wall = section.wall_thickness.value;
    let mill_tolerance = section.mill_tolerance.as_ref().map_or(0.0, |q| q.value);
    let effective_wall = wall - mill_tolerance;
    if !od.is_finite() || !effective_wall.is_finite() || od <= 0.0 || effective_wall <= 0.0 {
        diagnostics.push(diag(
            &format!(
                "diagnostic:equivalent-static:{}:{}:section",
                stable_suffix(load_case_id),
                stable_suffix(&pipe.id)
            ),
            "EQUIVALENT_STATIC_INPUT_INVALID",
            "blocking",
            "equivalent-static mass distribution requires a valid pipe section (positive outside diameter and effective wall)",
            vec![pipe.id.clone(), load_case_id.to_string()],
        ));
        return None;
    }
    let Some(material_density) = section.material_density.as_ref().map(|q| q.value) else {
        diagnostics.push(diag(
            &format!(
                "diagnostic:equivalent-static:{}:{}:material-density",
                stable_suffix(load_case_id),
                stable_suffix(&pipe.id)
            ),
            "EQUIVALENT_STATIC_INPUT_MISSING",
            "blocking",
            "seismic equivalent-static generation requires user-entered material_density on every pipe segment; no density is defaulted",
            vec![pipe.id.clone(), "material_density".to_string(), load_case_id.to_string()],
        ));
        return None;
    };
    if !material_density.is_finite() || material_density <= 0.0 {
        diagnostics.push(diag(
            &format!(
                "diagnostic:equivalent-static:{}:{}:material-density-invalid",
                stable_suffix(load_case_id),
                stable_suffix(&pipe.id)
            ),
            "EQUIVALENT_STATIC_INPUT_INVALID",
            "blocking",
            "material_density must be a finite positive user-entered value",
            vec![
                pipe.id.clone(),
                "material_density".to_string(),
                load_case_id.to_string(),
            ],
        ));
        return None;
    }
    let id = od - 2.0 * effective_wall;
    if id < 0.0 {
        diagnostics.push(diag(
            &format!(
                "diagnostic:equivalent-static:{}:{}:inside-diameter",
                stable_suffix(load_case_id),
                stable_suffix(&pipe.id)
            ),
            "EQUIVALENT_STATIC_INPUT_INVALID",
            "blocking",
            "equivalent-static mass distribution requires a non-negative inside diameter",
            vec![pipe.id.clone(), load_case_id.to_string()],
        ));
        return None;
    }
    let metal_area = PI / 4.0 * (od.powi(2) - id.powi(2));
    let mut mass_per_length = metal_area * material_density;
    if let Some(contents_density) = section.contents_density.as_ref().map(|q| q.value) {
        if !contents_density.is_finite() || contents_density < 0.0 {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:equivalent-static:{}:{}:contents-density",
                    stable_suffix(load_case_id),
                    stable_suffix(&pipe.id)
                ),
                "EQUIVALENT_STATIC_INPUT_INVALID",
                "blocking",
                "contents_density must be a finite non-negative user-entered value",
                vec![
                    pipe.id.clone(),
                    "contents_density".to_string(),
                    load_case_id.to_string(),
                ],
            ));
            return None;
        }
        mass_per_length += PI / 4.0 * id.powi(2) * contents_density;
    }
    match (
        section.insulation_thickness.as_ref().map(|q| q.value),
        section.insulation_density.as_ref().map(|q| q.value),
    ) {
        (None, None) => {}
        (Some(thickness), Some(density)) => {
            if !thickness.is_finite() || thickness < 0.0 || !density.is_finite() || density < 0.0 {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:equivalent-static:{}:{}:insulation",
                        stable_suffix(load_case_id),
                        stable_suffix(&pipe.id)
                    ),
                    "EQUIVALENT_STATIC_INPUT_INVALID",
                    "blocking",
                    "insulation_thickness and insulation_density must be finite non-negative user-entered values",
                    vec![pipe.id.clone(), load_case_id.to_string()],
                ));
                return None;
            }
            let insulation_od = od + 2.0 * thickness;
            mass_per_length += PI / 4.0 * (insulation_od.powi(2) - od.powi(2)) * density;
        }
        _ => {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:equivalent-static:{}:{}:insulation-pair",
                    stable_suffix(load_case_id),
                    stable_suffix(&pipe.id)
                ),
                "EQUIVALENT_STATIC_INPUT_MISSING",
                "blocking",
                "insulation mass requires both insulation_thickness and insulation_density; supply both or neither (no silent skip)",
                vec![pipe.id.clone(), load_case_id.to_string()],
            ));
            return None;
        }
    }
    Some(mass_per_length)
}

fn equivalent_static_diag_id(load_case_id: &str, suffix: &str) -> String {
    format!(
        "diagnostic:equivalent-static:{}:{suffix}",
        stable_suffix(load_case_id)
    )
}

/// Synthesize seismic/wind static-equivalent primitive loads for a load
/// case carrying `equivalent_static` user inputs (DEC-068 item 2). Pure
/// mechanics from user inputs and the model's own computed mass
/// distribution: no coefficients, no catalogs, no defaults; missing inputs
/// or marked spans are blocking (PRD section 6.2).
fn append_equivalent_static_generated_loads(
    model: &PreviewModel,
    load_case: &PreviewLoadCase,
    loads: &mut Vec<PrimitiveLoad>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(equivalent_static) = &load_case.equivalent_static else {
        return;
    };
    let case_id = load_case.id.as_str();
    if equivalent_static.seismic.is_none() && equivalent_static.wind.is_none() {
        diagnostics.push(diag(
            &equivalent_static_diag_id(case_id, "inputs"),
            "EQUIVALENT_STATIC_INPUT_MISSING",
            "blocking",
            "equivalent_static generation requires user-entered seismic or wind inputs",
            vec![case_id.to_string()],
        ));
        return;
    }

    if let Some(seismic) = &equivalent_static.seismic {
        let mut axis_factors = Vec::new();
        for (direction, factor) in [
            (LoadDirection::GlobalX, &seismic.g_factor_x),
            (LoadDirection::GlobalY, &seismic.g_factor_y),
            (LoadDirection::GlobalZ, &seismic.g_factor_z),
        ] {
            if let Some(factor) = factor {
                axis_factors.push(EquivalentStaticAxisFactor {
                    direction,
                    g_factor: factor.value,
                });
            }
        }
        let mut inputs_missing = false;
        if axis_factors.is_empty() {
            diagnostics.push(diag(
                &equivalent_static_diag_id(case_id, "seismic:g-factors"),
                "EQUIVALENT_STATIC_INPUT_MISSING",
                "blocking",
                "seismic equivalent-static generation requires at least one user-entered per-axis g-factor",
                vec![case_id.to_string()],
            ));
            inputs_missing = true;
        }
        let Some(gravity) = seismic.gravity_acceleration.as_ref().map(|q| q.value) else {
            diagnostics.push(diag(
                &equivalent_static_diag_id(case_id, "seismic:gravity"),
                "EQUIVALENT_STATIC_INPUT_MISSING",
                "blocking",
                "seismic equivalent-static generation requires a user-entered gravity_acceleration; no physical-constant default is applied",
                vec![case_id.to_string(), "gravity_acceleration".to_string()],
            ));
            return;
        };
        if inputs_missing {
            return;
        }
        let mut masses = Vec::new();
        let mut mass_blocked = false;
        for (pipe_index, pipe) in model.pipe_segments.iter().enumerate() {
            match compute_pipe_mass_per_length(pipe, case_id, diagnostics) {
                Some(mass_per_length) => masses.push(ElementMassPerLength {
                    element_index: pipe_index,
                    mass_per_length,
                }),
                None => mass_blocked = true,
            }
        }
        if mass_blocked {
            return;
        }
        let basis = SeismicEquivalentStaticBasis {
            load_case_ref: case_id.to_string(),
            gravity_acceleration: gravity,
            axis_factors,
        };
        let (generated, findings) = generate_seismic_equivalent_static_loads(&basis, &masses);
        for finding in &findings {
            diagnostics.push(diag(
                &equivalent_static_diag_id(
                    case_id,
                    &format!("seismic:{}", stable_suffix(&finding.load_id)),
                ),
                "EQUIVALENT_STATIC_INPUT_INVALID",
                "blocking",
                finding.message.clone(),
                vec![case_id.to_string()],
            ));
        }
        if findings.is_empty() {
            loads.extend(generated);
        }
    }

    if let Some(wind) = &equivalent_static.wind {
        let mut missing = Vec::new();
        if wind.pressure.is_none() {
            missing.push("pressure");
        }
        if wind.shape_factor.is_none() {
            missing.push("shape_factor");
        }
        if wind.direction.is_none() {
            missing.push("direction");
        }
        if wind.exposed_pipe_refs.is_empty() {
            missing.push("exposed_pipe_refs");
        }
        if !missing.is_empty() {
            diagnostics.push(diag(
                &equivalent_static_diag_id(case_id, "wind:inputs"),
                "EQUIVALENT_STATIC_INPUT_MISSING",
                "blocking",
                format!(
                    "wind equivalent-static generation requires user-entered {}; no value or marked span is defaulted",
                    missing.join(", ")
                ),
                vec![case_id.to_string()],
            ));
            return;
        }
        let direction_value = wind.direction.as_deref().expect("checked above");
        let direction = match parse_direction(direction_value) {
            Ok(
                direction @ (LoadDirection::GlobalX
                | LoadDirection::GlobalY
                | LoadDirection::GlobalZ),
            ) => direction,
            _ => {
                diagnostics.push(diag(
                    &equivalent_static_diag_id(case_id, "wind:direction"),
                    "EQUIVALENT_STATIC_INPUT_INVALID",
                    "blocking",
                    "wind equivalent-static direction must be a global axis (global_x, global_y, or global_z)",
                    vec![case_id.to_string(), "wind.direction".to_string()],
                ));
                return;
            }
        };
        let pipe_map = model
            .pipe_segments
            .iter()
            .enumerate()
            .map(|(i, p)| (p.id.as_str(), i))
            .collect::<HashMap<_, _>>();
        let mut exposed = Vec::new();
        let mut refs_blocked = false;
        for pipe_ref in &wind.exposed_pipe_refs {
            let Some(&pipe_index) = pipe_map.get(pipe_ref.as_str()) else {
                diagnostics.push(diag(
                    &equivalent_static_diag_id(
                        case_id,
                        &format!("wind:span:{}", stable_suffix(pipe_ref)),
                    ),
                    "EQUIVALENT_STATIC_INPUT_INVALID",
                    "blocking",
                    "wind equivalent-static marked span references a pipe that is not present in the preview model",
                    vec![case_id.to_string(), pipe_ref.clone()],
                ));
                refs_blocked = true;
                continue;
            };
            let section = &model.pipe_segments[pipe_index].section;
            let insulation = section
                .insulation_thickness
                .as_ref()
                .map_or(0.0, |q| q.value);
            exposed.push(ElementExposedDiameter {
                element_index: pipe_index,
                exposed_diameter: section.outside_diameter.value + 2.0 * insulation,
            });
        }
        if refs_blocked {
            return;
        }
        let basis = WindEquivalentStaticBasis {
            load_case_ref: case_id.to_string(),
            pressure: wind.pressure.as_ref().expect("checked above").value,
            shape_factor: wind.shape_factor.as_ref().expect("checked above").value,
            direction,
        };
        let (generated, findings) = generate_wind_equivalent_static_loads(&basis, &exposed);
        for finding in &findings {
            diagnostics.push(diag(
                &equivalent_static_diag_id(
                    case_id,
                    &format!("wind:{}", stable_suffix(&finding.load_id)),
                ),
                "EQUIVALENT_STATIC_INPUT_INVALID",
                "blocking",
                finding.message.clone(),
                vec![case_id.to_string()],
            ));
        }
        if findings.is_empty() {
            loads.extend(generated);
        }
    }
}

/// Resolve the material set a load case solves with under a named modulus
fn modulus_basis_key(
    load_case: &PreviewLoadCase,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<String> {
    match (
        load_case.modulus_basis_ref.as_deref(),
        load_case.modulus_basis_temperature.as_ref(),
    ) {
        (Some(_), Some(_)) => {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:modulus-basis:{}:selection-conflict",
                    stable_suffix(&load_case.id)
                ),
                "MODULUS_BASIS_SELECTION_CONFLICT",
                "blocking",
                "load case supplies both modulus_basis_ref and modulus_basis_temperature; select one exact-id or interpolated-temperature basis",
                vec![load_case.id.clone()],
            ));
            None
        }
        (Some(basis_ref), None) => Some(format!("exact:{basis_ref}")),
        (None, Some(temperature)) if temperature.value.is_finite() => Some(format!(
            "temperature_kelvin_bits:{:016x}",
            temperature.value.to_bits()
        )),
        (None, Some(_)) => {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:modulus-basis:{}:temperature-invalid",
                    stable_suffix(&load_case.id)
                ),
                "MODULUS_BASIS_INPUT_INVALID",
                "blocking",
                "modulus_basis_temperature must be a finite user-entered absolute temperature",
                vec![
                    load_case.id.clone(),
                    "modulus_basis_temperature".to_string(),
                ],
            ));
            None
        }
        (None, None) => None,
    }
}

/// Resolve an exact temperature-point basis (DEC-068 item 1) or a declared
/// linear interpolation basis (DEC-077). For every material used by a pipe
/// segment, exact selection requires the named point and its E; interpolation
/// requires two adjacent points that strictly bracket the solve temperature
/// and carry both E and alpha. No base-value mixing or extrapolation occurs.
fn materials_for_modulus_basis(
    model: &PreviewModel,
    materials: &[MaterialInput],
    load_case: &PreviewLoadCase,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<(Vec<MaterialInput>, String)> {
    let load_case_id = &load_case.id;
    let used_material_ids = model
        .pipe_segments
        .iter()
        .map(|pipe| pipe.material.as_str())
        .collect::<HashSet<_>>();
    let mut resolved = Vec::with_capacity(materials.len());
    let mut provenance_records = Vec::new();
    let mut blocked = false;
    for material in materials {
        if !used_material_ids.contains(material.id.as_str()) {
            resolved.push(material.clone());
            continue;
        }
        let (elastic_modulus, thermal_expansion_coefficient, provenance) = if let Some(basis_ref) =
            load_case.modulus_basis_ref.as_deref()
        {
            let Some(point) = material
                .temperature_points
                .iter()
                .find(|point| point.id == basis_ref)
            else {
                diagnostics.push(diag(
                        &format!(
                            "diagnostic:modulus-basis:{}:{}:unresolved",
                            stable_suffix(load_case_id),
                            stable_suffix(&material.id)
                        ),
                        "MODULUS_BASIS_UNRESOLVED",
                        "blocking",
                        format!(
                            "load case names modulus basis {basis_ref}, but material {} stores no user-entered temperature point with that id; exact-id selection remains available and no value is defaulted",
                            material.id
                        ),
                        vec![load_case_id.to_string(), material.id.clone(), basis_ref.to_string()],
                    ));
                blocked = true;
                continue;
            };
            let Some(elastic_modulus) = point.elastic_modulus.clone() else {
                diagnostics.push(diag(
                        &format!(
                            "diagnostic:modulus-basis:{}:{}:elastic-modulus",
                            stable_suffix(load_case_id),
                            stable_suffix(&material.id)
                        ),
                        "MODULUS_BASIS_INPUT_MISSING",
                        "blocking",
                        format!(
                            "modulus basis {basis_ref} on material {} carries no user-entered elastic modulus; no value is defaulted",
                            material.id
                        ),
                        vec![load_case_id.to_string(), material.id.clone(), basis_ref.to_string()],
                    ));
                blocked = true;
                continue;
            };
            (
                    elastic_modulus,
                    point.thermal_expansion_coefficient.clone(),
                    format!(
                        "material={}; selection=exact_user_entered_point; source_point={basis_ref}; interpolation=not_performed",
                        material.id
                    ),
                )
        } else {
            let solve_temperature = load_case
                .modulus_basis_temperature
                .as_ref()
                .expect("basis resolver is called only for a selected basis")
                .value;
            let mut points = material
                .temperature_points
                .iter()
                .filter_map(|point| point.temperature.as_ref().map(|t| (t.value, point)))
                .collect::<Vec<_>>();
            points.sort_by(|left, right| left.0.total_cmp(&right.0));
            if points.windows(2).any(|pair| pair[0].0 == pair[1].0) {
                diagnostics.push(diag(
                        &format!(
                            "diagnostic:modulus-basis:{}:{}:duplicate-temperature",
                            stable_suffix(load_case_id),
                            stable_suffix(&material.id)
                        ),
                        "MODULUS_BASIS_INPUT_INVALID",
                        "blocking",
                        format!(
                            "material {} stores duplicate user-entered temperature points; an unambiguous adjacent interpolation bracket is required",
                            material.id
                        ),
                        vec![load_case_id.to_string(), material.id.clone()],
                    ));
                blocked = true;
                continue;
            }
            let bracket = points
                .windows(2)
                .find(|pair| pair[0].0 < solve_temperature && solve_temperature < pair[1].0);
            let Some(bracket) = bracket else {
                diagnostics.push(diag(
                        &format!(
                            "diagnostic:modulus-basis:{}:{}:interpolation-range",
                            stable_suffix(load_case_id),
                            stable_suffix(&material.id)
                        ),
                        "MODULUS_BASIS_UNRESOLVED",
                        "blocking",
                        format!(
                            "solve temperature {solve_temperature} K is not strictly bracketed by two adjacent user-entered points on material {}; interpolation blocks at and beyond stored range edges and never extrapolates",
                            material.id
                        ),
                        vec![load_case_id.to_string(), material.id.clone()],
                    ));
                blocked = true;
                continue;
            };
            let (lower_temperature, lower) = bracket[0];
            let (upper_temperature, upper) = bracket[1];
            let (Some(lower_e), Some(upper_e), Some(lower_alpha), Some(upper_alpha)) = (
                lower.elastic_modulus.as_ref(),
                upper.elastic_modulus.as_ref(),
                lower.thermal_expansion_coefficient.as_ref(),
                upper.thermal_expansion_coefficient.as_ref(),
            ) else {
                diagnostics.push(diag(
                        &format!(
                            "diagnostic:modulus-basis:{}:{}:interpolation-input",
                            stable_suffix(load_case_id),
                            stable_suffix(&material.id)
                        ),
                        "MODULUS_BASIS_INPUT_MISSING",
                        "blocking",
                        format!(
                            "material {} interpolation bracket {}..{} must carry user-entered elastic modulus and thermal expansion coefficient on both source points",
                            material.id, lower.id, upper.id
                        ),
                        vec![
                            load_case_id.to_string(),
                            material.id.clone(),
                            lower.id.clone(),
                            upper.id.clone(),
                        ],
                    ));
                blocked = true;
                continue;
            };
            let fraction =
                (solve_temperature - lower_temperature) / (upper_temperature - lower_temperature);
            let interpolated_e = lower_e.value + fraction * (upper_e.value - lower_e.value);
            let interpolated_alpha =
                lower_alpha.value + fraction * (upper_alpha.value - lower_alpha.value);
            (
                    Quantity {
                        value: interpolated_e,
                        unit: lower_e.unit.clone(),
                    },
                    Some(Quantity {
                        value: interpolated_alpha,
                        unit: lower_alpha.unit.clone(),
                    }),
                    format!(
                        "material={}; elastic_modulus_sources={},{}; thermal_expansion_coefficient_sources={},{}; method=linear_temperature_interpolation; solve_temperature_kelvin={solve_temperature}",
                        material.id, lower.id, upper.id, lower.id, upper.id
                    ),
                )
        };
        if !elastic_modulus.value.is_finite() || elastic_modulus.value <= 0.0 {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:modulus-basis:{}:{}:elastic-modulus-invalid",
                    stable_suffix(load_case_id),
                    stable_suffix(&material.id)
                ),
                "MODULUS_BASIS_INPUT_INVALID",
                "blocking",
                format!(
                    "resolved elastic modulus on material {} must be finite and positive",
                    material.id
                ),
                vec![load_case_id.to_string(), material.id.clone()],
            ));
            blocked = true;
            continue;
        }
        if thermal_expansion_coefficient
            .as_ref()
            .is_some_and(|alpha| !alpha.value.is_finite())
        {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:modulus-basis:{}:{}:thermal-expansion-invalid",
                    stable_suffix(load_case_id),
                    stable_suffix(&material.id)
                ),
                "MODULUS_BASIS_INPUT_INVALID",
                "blocking",
                format!(
                    "resolved thermal expansion coefficient on material {} must be finite",
                    material.id
                ),
                vec![load_case_id.to_string(), material.id.clone()],
            ));
            blocked = true;
            continue;
        }
        provenance_records.push(provenance);
        resolved.push(MaterialInput {
            id: material.id.clone(),
            elastic_modulus,
            // The shear modulus stays the user-entered base value; a
            // temperature-indexed shear modulus slot is a recorded residual
            // outside the DEC-068 item 1 (E, alpha) grant.
            shear_modulus: material.shear_modulus.clone(),
            thermal_expansion_coefficient,
            temperature_points: material.temperature_points.clone(),
            provenance: material.provenance.clone(),
        });
    }
    if blocked {
        None
    } else {
        let basis_record = if let Some(basis_ref) = load_case.modulus_basis_ref.as_deref() {
            format!(
                "temperature_point:{basis_ref}; selection=exact_user_entered_point; interpolation=not_performed; {}",
                provenance_records.join(" | ")
            )
        } else {
            format!(
                "selection=declared_solve_temperature; {}; {}",
                load_case
                    .modulus_basis_temperature
                    .as_ref()
                    .map(|temperature| format!("solve_temperature_kelvin={}", temperature.value))
                    .unwrap_or_default(),
                provenance_records.join(" | ")
            )
        };
        Some((resolved, basis_record))
    }
}

/// Emit the explicit modulus-basis record row for a load case that names
/// one (DEC-068 item 1 evidence shape).
fn append_modulus_basis_record(
    results: &mut Vec<ResultItem>,
    load_case: &PreviewLoadCase,
    basis_record: Option<&str>,
) {
    let Some(basis_record) = basis_record else {
        return;
    };
    results.push(ResultItem {
        id: format!(
            "result:modulus-basis:{}",
            stable_suffix(&load_case.id)
        ),
        kind: "modulus_basis_record".to_string(),
        value: 1.0,
        unit: "record".to_string(),
        entity_ref: load_case.id.clone(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            component: "material_modulus_basis".to_string(),
            coordinate_system: "not_applicable".to_string(),
            location: load_case.id.clone(),
            basis: basis_record.to_string(),
            sign_convention: "presence record; value 1.0 means the load case solved with the recorded user-entered property basis".to_string(),
        }),
    });
}

/// Record the modulus basis of every operand of expansion-range
/// combinations (result-state subtraction and range envelopes) explicitly
/// (DEC-068 item 1).
fn append_combination_modulus_basis_records(model: &PreviewModel, results: &mut Vec<ResultItem>) {
    let recorded_basis_by_case = results
        .iter()
        .filter(|item| item.kind == "modulus_basis_record")
        .filter_map(|item| {
            item.metadata
                .as_ref()
                .map(|metadata| (item.entity_ref.clone(), metadata.basis.clone()))
        })
        .collect::<HashMap<_, _>>();
    for combination in &model.combinations {
        let operand_ids: Vec<&str> = match combination.basis.as_str() {
            "result_state_subtraction" => combination
                .minuend_id
                .iter()
                .chain(combination.subtrahend_id.iter())
                .map(|id| id.as_str())
                .collect(),
            "range_envelope" => combination
                .operand_ids
                .iter()
                .flatten()
                .map(|id| id.as_str())
                .collect(),
            _ => continue,
        };
        for operand_id in operand_ids {
            let Some(load_case) = model.load_cases.iter().find(|case| case.id == operand_id) else {
                continue;
            };
            let basis_label = recorded_basis_by_case
                .get(operand_id)
                .cloned()
                .unwrap_or_else(|| {
                    debug_assert!(
                        load_case.modulus_basis_ref.is_none()
                            && load_case.modulus_basis_temperature.is_none()
                    );
                    "material_base_values".to_string()
                });
            results.push(ResultItem {
                id: format!(
                    "result:combination:{}:modulus-basis:{}",
                    stable_suffix(&combination.id),
                    stable_suffix(operand_id)
                ),
                kind: "combination_modulus_basis_record".to_string(),
                value: 1.0,
                unit: "record".to_string(),
                entity_ref: operand_id.to_string(),
                basis_ref: Some(ResultBasisRef {
                    ref_type: "combination".to_string(),
                    ref_id: combination.id.clone(),
                }),
                source_result_refs: Vec::new(),
                metadata: Some(ResultMetadata {
                    component: "material_modulus_basis".to_string(),
                    coordinate_system: "not_applicable".to_string(),
                    location: combination.id.clone(),
                    basis: basis_label,
                    sign_convention:
                        "presence record; each range operand's solve basis is recorded explicitly"
                            .to_string(),
                }),
            });
        }
    }
}

fn derive_pipe_section(
    input: &PipeSectionInput,
    pipe_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<DerivedSection> {
    let od = input.outside_diameter.value;
    let nominal_thickness = input.wall_thickness.value;
    if !od.is_finite()
        || !nominal_thickness.is_finite()
        || od <= 0.0
        || nominal_thickness <= 0.0
        || 2.0 * nominal_thickness >= od
    {
        diagnostics.push(diag(&format!("diagnostic:section:{}", stable_suffix(pipe_id)), "PIPE_DIMENSION_INVALID", "blocking", "outside diameter and wall thickness must be explicit positive values with wall thickness less than radius", vec![pipe_id.to_string()]));
        return None;
    }
    // Mill tolerance is an optional user-entered absolute thickness reduction
    // consumed alongside the nominal wall (DEC-068 item 3). Absence means no
    // reduction; a present-but-invalid slot is blocking (PRD section 6.2).
    let thickness = match input.mill_tolerance.as_ref().map(|q| q.value) {
        None => nominal_thickness,
        Some(mill_tolerance) => {
            let effective = nominal_thickness - mill_tolerance;
            if !mill_tolerance.is_finite() || mill_tolerance < 0.0 || effective <= 0.0 {
                diagnostics.push(diag(
                    &format!("diagnostic:section:{}:mill-tolerance", stable_suffix(pipe_id)),
                    "PIPE_DIMENSION_INVALID",
                    "blocking",
                    "mill tolerance must be a finite non-negative thickness that leaves a positive effective wall",
                    vec![pipe_id.to_string(), "mill_tolerance".to_string()],
                ));
                return None;
            }
            effective
        }
    };
    let id = od - 2.0 * thickness;
    let area = PI * (od.powi(2) - id.powi(2)) / 4.0;
    let internal_area = PI * id.powi(2) / 4.0;
    let second_moment = PI * (od.powi(4) - id.powi(4)) / 64.0;
    let torsion_constant = 2.0 * second_moment;
    Some(DerivedSection {
        area,
        internal_area,
        second_moment,
        torsion_constant,
        section_modulus: second_moment / (od / 2.0),
        torsion_radius: od / 2.0,
        membrane_radius: (od - thickness) / 2.0,
        wall_thickness: thickness,
    })
}

fn add_uniform_element_loads(
    force: &mut [f64],
    model: &PreviewModel,
    loads: &[open_pipe_stress_primitive_loads::ElementUniformLoadContribution],
    pipes: &[StraightPipeElement],
    curved_bends_by_pipe: &HashMap<usize, &CurvedBendMacroBuild>,
    load_case_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for load in loads {
        if matches!(
            load.magnitude.dimension,
            LoadDimension::Pressure | LoadDimension::TemperatureChange
        ) {
            continue;
        }
        // Curved-bend macro spans consume arc-consistent equivalent nodal
        // loads: fixed-end forces and moments from exact closed-form
        // integration of the uniform intensity along the arc, consistent with
        // the assembled macro-element stiffness. Element uniform loads are
        // validated translational upstream, so the intensity is a global
        // force per unit arc length along one global axis.
        if let Some(bend) = curved_bends_by_pipe.get(&load.element_index) {
            let dof = load.direction.dof_index();
            let equivalent = if dof < 3 {
                let mut intensity = [0.0; 3];
                intensity[dof] = load.magnitude.value;
                bend.macro_element
                    .consistent_uniform_nodal_loads(intensity)
                    .map_err(|error| error.to_string())
            } else {
                Err("uniform element loads on curved-bend macro spans require a translational direction".to_string())
            };
            match equivalent {
                Ok(equivalent) => {
                    let dof_map = element_dof_map(bend.node_i, bend.node_j);
                    for (local_dof, &global_dof) in dof_map.iter().enumerate() {
                        force[global_dof] += equivalent[local_dof];
                    }
                }
                Err(message) => {
                    // No silent drop or lumped fallback: an inapplicable
                    // uniform load on a realized arc blocks the solve.
                    diagnostics.push(diag(
                        &format!(
                            "diagnostic:curved-bend:{}:{}:distributed-load",
                            stable_suffix(load_case_id),
                            stable_suffix(&load.load_id)
                        ),
                        "LOAD_INPUT_INVALID",
                        "blocking",
                        format!(
                            "uniform load {} on curved-bend span {} could not be converted to arc-consistent equivalent nodal loads: {message}",
                            load.load_id, bend.pipe_id
                        ),
                        vec![
                            load.load_id.clone(),
                            bend.pipe_id.clone(),
                            bend.component_id.clone(),
                            load_case_id.to_string(),
                        ],
                    ));
                }
            }
            continue;
        }
        let Ok(length) = pipes[load.element_index].length() else {
            continue;
        };
        let pipe = &model.pipe_segments[load.element_index];
        let Some(i) = node_index(model, &pipe.from) else {
            continue;
        };
        let Some(j) = node_index(model, &pipe.to) else {
            continue;
        };
        let dof = load.direction.dof_index();
        let share = load.magnitude.value * length / 2.0;
        force[i * DOF_PER_NODE + dof] += share;
        force[j * DOF_PER_NODE + dof] += share;
    }
}

fn build_thermal_element_loads(
    model: &PreviewModel,
    load_case: &PreviewLoadCase,
    material_map: &HashMap<&str, &MaterialInput>,
    pipe_map: &HashMap<&str, usize>,
    sections: &HashMap<String, DerivedSection>,
) -> Vec<ThermalElementLoad> {
    let mut loads = Vec::new();
    for load in &load_case.primitive_loads {
        if load.category != "thermal" || !is_temperature_change_dimension(&load.dimension) {
            continue;
        }
        let LoadTargetInput::Element { pipe } = &load.target else {
            continue;
        };
        let Some(&element_index) = pipe_map.get(pipe.as_str()) else {
            continue;
        };
        let Some(pipe_input) = model.pipe_segments.get(element_index) else {
            continue;
        };
        let Some(material) = material_map.get(pipe_input.material.as_str()) else {
            continue;
        };
        let Some(alpha) = material
            .thermal_expansion_coefficient
            .as_ref()
            .map(|quantity| quantity.value)
        else {
            continue;
        };
        let Some(section) = sections.get(&pipe_input.id) else {
            continue;
        };
        let epsilon_th = alpha * load.magnitude.value;
        loads.push(ThermalElementLoad {
            element_index,
            axial_load: material.elastic_modulus.value * section.area * epsilon_th,
            thermal_strain: epsilon_th,
        });
    }
    loads
}

fn build_pressure_thrust_loads(
    model: &PreviewModel,
    load_case: &PreviewLoadCase,
    pipe_map: &HashMap<&str, usize>,
    sections: &HashMap<String, DerivedSection>,
) -> Vec<PressureThrustLoad> {
    let mut loads = Vec::new();
    let expansion_joint_inputs = expansion_joint_pressure_thrust_inputs_by_pipe(model);
    for load in &load_case.primitive_loads {
        if load.category != "pressure" || load.dimension != "pressure" {
            continue;
        }
        let LoadTargetInput::Element { pipe } = &load.target else {
            continue;
        };
        let Some(&element_index) = pipe_map.get(pipe.as_str()) else {
            continue;
        };
        if let Some(inputs) = expansion_joint_inputs.get(pipe.as_str()) {
            for input in inputs {
                loads.push(PressureThrustLoad {
                    element_index,
                    axial_load: load.magnitude.value * input.effective_area,
                    source_load_id: load.id.clone(),
                    source: PressureThrustSource::ExpansionJointEffectiveArea(input.clone()),
                });
            }
            continue;
        }
        let Some(section) = sections.get(pipe) else {
            continue;
        };
        loads.push(PressureThrustLoad {
            element_index,
            axial_load: load.magnitude.value * section.internal_area,
            source_load_id: load.id.clone(),
            source: PressureThrustSource::PipeInternalArea,
        });
    }
    loads
}

fn expansion_joint_pressure_thrust_inputs_by_pipe(
    model: &PreviewModel,
) -> HashMap<String, Vec<ExpansionJointPressureThrustInput>> {
    let mut inputs_by_pipe: HashMap<String, Vec<ExpansionJointPressureThrustInput>> =
        HashMap::new();
    for component in model
        .components
        .iter()
        .filter(|component| is_expansion_joint_component(component))
    {
        let solver_consumption = component
            .mechanics_interface
            .as_ref()
            .and_then(|interface| interface.solver_consumption.as_deref())
            .unwrap_or("not_provided");
        if solver_consumption != "mechanics_geometry_and_user_flexibility" {
            continue;
        }
        let Some(geometry) = component.geometry.as_ref() else {
            continue;
        };
        let Some(pipe_id) = geometry
            .expansion_joint_pipe_ref
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        else {
            continue;
        };
        let Some(effective_area) = geometry
            .effective_area
            .as_ref()
            .map(|quantity| quantity.value)
            .filter(|value| positive_finite(*value))
        else {
            continue;
        };
        let pressure_thrust_reference = geometry
            .pressure_thrust_reference
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("load_side_pressure_thrust_reference_missing")
            .to_string();
        let source_reference = geometry
            .expansion_joint_source_reference
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("expansion_joint_source_reference_missing")
            .to_string();
        inputs_by_pipe.entry(pipe_id.to_string()).or_default().push(
            ExpansionJointPressureThrustInput {
                component_id: component.id.clone(),
                pipe_id: pipe_id.to_string(),
                effective_area,
                pressure_thrust_reference,
                source_reference,
                solver_consumption: solver_consumption.to_string(),
            },
        );
    }
    inputs_by_pipe
}

fn add_pressure_thrust_loads(
    force: &mut [f64],
    pressure_loads: &[PressureThrustLoad],
    pipes: &[StraightPipeElement],
) {
    for load in pressure_loads {
        let Some(pipe) = pipes.get(load.element_index) else {
            continue;
        };
        let Ok(frame_element) = pipe.frame_element() else {
            continue;
        };
        let Ok(orientation) = frame_element.orientation() else {
            continue;
        };
        let local_x = orientation.local_axes[0];
        let i_base = pipe.node_i.index * DOF_PER_NODE;
        let j_base = pipe.node_j.index * DOF_PER_NODE;
        for axis in 0..3 {
            force[i_base + axis] -= load.axial_load * local_x[axis];
            force[j_base + axis] += load.axial_load * local_x[axis];
        }
    }
}

fn add_thermal_equivalent_loads(
    force: &mut [f64],
    thermal_loads: &[ThermalElementLoad],
    pipes: &[StraightPipeElement],
    curved_bends_by_pipe: &HashMap<usize, &CurvedBendMacroBuild>,
) {
    for load in thermal_loads {
        if let Some(bend) = curved_bends_by_pipe.get(&load.element_index) {
            add_curved_bend_thermal_equivalent_load(force, bend, load.thermal_strain);
            continue;
        }
        let Some(pipe) = pipes.get(load.element_index) else {
            continue;
        };
        let Ok(frame_element) = pipe.frame_element() else {
            continue;
        };
        let Ok(orientation) = frame_element.orientation() else {
            continue;
        };
        let local_x = orientation.local_axes[0];
        let i_base = pipe.node_i.index * DOF_PER_NODE;
        let j_base = pipe.node_j.index * DOF_PER_NODE;
        for axis in 0..3 {
            force[i_base + axis] -= load.axial_load * local_x[axis];
            force[j_base + axis] += load.axial_load * local_x[axis];
        }
    }
}

// Exact free-expansion identity for the arc span: uniform thermal expansion
// is stress-free, so the equivalent nodal load is K_macro * u_free with
// u_free the pure translation field alpha*deltaT*(p - p_i) (zero rotations)
// evaluated at the two end nodes. No approximation is introduced for the arc.
fn add_curved_bend_thermal_equivalent_load(
    force: &mut [f64],
    bend: &CurvedBendMacroBuild,
    thermal_strain: f64,
) {
    let free_expansion = curved_bend_free_expansion_displacements(bend, thermal_strain);
    let dof_map = element_dof_map(bend.node_i, bend.node_j);
    for (local_row, &global_row) in dof_map.iter().enumerate() {
        let mut value = 0.0;
        for (local_col, free_value) in free_expansion.iter().enumerate() {
            value += bend.global_stiffness[local_row][local_col] * free_value;
        }
        force[global_row] += value;
    }
}

// Free thermal expansion of the arc about node i: node i stays put and node j
// translates by thermal_strain * chord with zero end rotations (uniform
// scaling is rotation-free; the rigid part of the field is in the stiffness
// nullspace, so anchoring the field at node i is exact).
fn curved_bend_free_expansion_displacements(
    bend: &CurvedBendMacroBuild,
    thermal_strain: f64,
) -> [f64; ELEMENT_DOF] {
    let mut free_expansion = [0.0; ELEMENT_DOF];
    for axis in 0..3 {
        free_expansion[DOF_PER_NODE + axis] = thermal_strain * bend.chord[axis];
    }
    free_expansion
}

fn corrected_local_forces_for_axial_effects(
    local_forces: &[f64],
    element_index: usize,
    thermal_loads: &[ThermalElementLoad],
    pressure_loads: &[PressureThrustLoad],
) -> Vec<f64> {
    let mut corrected = local_forces.to_vec();
    let thermal_axial_load = thermal_loads
        .iter()
        .filter(|load| load.element_index == element_index)
        .map(|load| load.axial_load)
        .sum::<f64>();
    let pressure_axial_load = pressure_thrust_for_pipe(element_index, pressure_loads);
    let axial_load = thermal_axial_load + pressure_axial_load;
    if axial_load != 0.0 && corrected.len() >= DOF_PER_NODE + UX + 1 {
        corrected[UX] += axial_load;
        corrected[DOF_PER_NODE + UX] -= axial_load;
    }
    corrected
}

// Macro-span recovery: end forces are K_macro * (d - u_free) minus the
// arc-consistent distributed equivalent loads, in global coordinates — the
// exact free-expansion correction mirrors
// `corrected_local_forces_for_axial_effects` so recovered forces exclude the
// self-equilibrated thermal part, and the equivalent-load subtraction turns
// the nodal solve response into the true node-on-element end forces of the
// continuously loaded arc — then rotated to the chord frame of the replaced
// straight span so the existing result rows keep their convention. Pressure
// thrust keeps the straight-element equal/opposite chord-axial correction.
fn recover_curved_bend_local_forces(
    bend: &CurvedBendMacroBuild,
    pipe: &StraightPipeElement,
    displacements: &[f64],
    thermal_loads: &[ThermalElementLoad],
    pressure_loads: &[PressureThrustLoad],
    uniform_intensity: [f64; 3],
) -> Result<Vec<f64>, String> {
    let required = (bend.node_i.max(bend.node_j) + 1) * DOF_PER_NODE;
    if displacements.len() < required {
        return Err(format!(
            "curved-bend recovery requires {} global displacement entries, got {}",
            required,
            displacements.len()
        ));
    }
    let mut element_displacements = [0.0; ELEMENT_DOF];
    for (node_slot, node_index) in [bend.node_i, bend.node_j].into_iter().enumerate() {
        for dof in 0..DOF_PER_NODE {
            element_displacements[node_slot * DOF_PER_NODE + dof] =
                displacements[node_index * DOF_PER_NODE + dof];
        }
    }
    let thermal_strain = thermal_loads
        .iter()
        .filter(|load| load.element_index == bend.pipe_index)
        .map(|load| load.thermal_strain)
        .sum::<f64>();
    if thermal_strain != 0.0 {
        let free_expansion = curved_bend_free_expansion_displacements(bend, thermal_strain);
        for (entry, free_value) in element_displacements.iter_mut().zip(free_expansion.iter()) {
            *entry -= free_value;
        }
    }

    let mut global_forces = [0.0; ELEMENT_DOF];
    for (row, force) in global_forces.iter_mut().enumerate() {
        for (col, displacement) in element_displacements.iter().enumerate() {
            *force += bend.global_stiffness[row][col] * displacement;
        }
    }
    if uniform_intensity != [0.0; 3] {
        let equivalent = bend
            .macro_element
            .consistent_uniform_nodal_loads(uniform_intensity)
            .map_err(|error| error.to_string())?;
        for (force, load) in global_forces.iter_mut().zip(equivalent.iter()) {
            *force -= load;
        }
    }

    let frame_element = pipe.frame_element().map_err(|error| error.to_string())?;
    let orientation = frame_element
        .orientation()
        .map_err(|error| error.to_string())?;
    let transform = orientation.transformation_matrix();
    let mut local_forces = vec![0.0; ELEMENT_DOF];
    for (row, local_force) in local_forces.iter_mut().enumerate() {
        for (col, global_force) in global_forces.iter().enumerate() {
            *local_force += transform[row][col] * global_force;
        }
    }

    let pressure_axial_load = pressure_thrust_for_pipe(bend.pipe_index, pressure_loads);
    if pressure_axial_load != 0.0 {
        local_forces[UX] += pressure_axial_load;
        local_forces[DOF_PER_NODE + UX] -= pressure_axial_load;
    }
    Ok(local_forces)
}

// Result-metadata basis for curved-bend interior stations: true section
// resultants from segment equilibrium of the arc between the station and the
// recovered assembled end force at node j (closed form in the curved-bend
// crate), not an endpoint interpolation.
const CURVED_BEND_STATION_BASIS: &str = "arc_section_equilibrium_from_assembled_end_forces";

// Summed global uniform intensity (force per unit arc length) per realized
// curved-bend span. The consistent equivalent loads are linear in the
// intensity, so the sum carries every uniform load on the span; pressure and
// temperature-change dimensioned loads follow their own dedicated paths.
fn curved_bend_uniform_intensity_by_pipe(
    loads: &[open_pipe_stress_primitive_loads::ElementUniformLoadContribution],
    curved_bends_by_pipe: &HashMap<usize, &CurvedBendMacroBuild>,
) -> HashMap<usize, [f64; 3]> {
    let mut intensity_by_pipe: HashMap<usize, [f64; 3]> = HashMap::new();
    for load in loads {
        if matches!(
            load.magnitude.dimension,
            LoadDimension::Pressure | LoadDimension::TemperatureChange
        ) {
            continue;
        }
        if !curved_bends_by_pipe.contains_key(&load.element_index) {
            continue;
        }
        let dof = load.direction.dof_index();
        if dof >= 3 {
            continue;
        }
        intensity_by_pipe
            .entry(load.element_index)
            .or_insert([0.0; 3])[dof] += load.magnitude.value;
    }
    intensity_by_pipe
}

// Arc interior stations from the assembled macro-element: rotate the
// recovered chord-frame end-j force back to global and evaluate section
// resultants along the arc by segment equilibrium (closed form in the
// curved-bend crate). The recovered end forces already exclude the
// self-equilibrated thermal free-expansion part and carry the recorded
// chord-axial pressure-thrust treatment, so the stations inherit both
// recovery decisions; the station grid mirrors the straight-span fractions.
fn curved_bend_station_resultants(
    bend: &CurvedBendMacroBuild,
    pipe: &StraightPipeElement,
    corrected_local_forces: &[f64],
    uniform_intensity: [f64; 3],
) -> Result<[StationResultants; 3], String> {
    if corrected_local_forces.len() < ELEMENT_DOF {
        return Err(format!(
            "curved-bend station evaluation requires {} recovered end-force entries, got {}",
            ELEMENT_DOF,
            corrected_local_forces.len()
        ));
    }
    let frame_element = pipe.frame_element().map_err(|error| error.to_string())?;
    let orientation = frame_element
        .orientation()
        .map_err(|error| error.to_string())?;
    let axes = orientation.local_axes;
    // Chord-frame end-j force back to global: transpose of the chord rotation
    // applied to the force and moment blocks.
    let mut node_j_force = [0.0; DOF_PER_NODE];
    for block in 0..2 {
        for component in 0..3 {
            let mut value = 0.0;
            for (axis, axis_row) in axes.iter().enumerate() {
                value +=
                    axis_row[component] * corrected_local_forces[DOF_PER_NODE + 3 * block + axis];
            }
            node_j_force[3 * block + component] = value;
        }
    }
    let locations: [(&'static str, f64); 3] =
        [("quarter_1", 0.25), ("midspan", 0.5), ("quarter_3", 0.75)];
    let mut stations = [
        StationResultants {
            location: "quarter_1",
            resultants: [0.0; 6],
        },
        StationResultants {
            location: "midspan",
            resultants: [0.0; 6],
        },
        StationResultants {
            location: "quarter_3",
            resultants: [0.0; 6],
        },
    ];
    for (station, (location, fraction)) in stations.iter_mut().zip(locations.into_iter()) {
        station.location = location;
        station.resultants = bend
            .macro_element
            .arc_section_resultants(fraction, node_j_force, uniform_intensity)
            .map_err(|error| error.to_string())?;
    }
    Ok(stations)
}

fn pressure_thrust_for_pipe(element_index: usize, pressure_loads: &[PressureThrustLoad]) -> f64 {
    pressure_loads
        .iter()
        .filter(|load| load.element_index == element_index)
        .map(|load| load.axial_load)
        .sum::<f64>()
}

#[derive(Debug, Clone)]
struct ExpansionJointPressureThrustAggregate {
    input: ExpansionJointPressureThrustInput,
    axial_load: f64,
    source_load_ids: Vec<String>,
}

fn append_expansion_joint_pressure_thrust_results(
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
    load_case: &PreviewLoadCase,
    pressure_loads: &[PressureThrustLoad],
) -> usize {
    let mut aggregates: BTreeMap<String, ExpansionJointPressureThrustAggregate> = BTreeMap::new();
    for load in pressure_loads {
        let PressureThrustSource::ExpansionJointEffectiveArea(input) = &load.source else {
            continue;
        };
        let entry = aggregates
            .entry(input.component_id.clone())
            .or_insert_with(|| ExpansionJointPressureThrustAggregate {
                input: input.clone(),
                axial_load: 0.0,
                source_load_ids: Vec::new(),
            });
        entry.axial_load += load.axial_load;
        entry.source_load_ids.push(load.source_load_id.clone());
    }

    let mut appended = 0;
    for (_, mut aggregate) in aggregates {
        if aggregate.axial_load == 0.0 {
            continue;
        }
        aggregate.source_load_ids.sort();
        aggregate.source_load_ids.dedup();
        let component_suffix = stable_suffix(&aggregate.input.component_id);
        let result_id = format!("result:pressure-thrust:{component_suffix}");
        results.push(ResultItem {
            id: result_id.clone(),
            kind: "expansion_joint_pressure_thrust_load_review".to_string(),
            value: round6(aggregate.axial_load),
            unit: "N".to_string(),
            entity_ref: aggregate.input.component_id.clone(),
            basis_ref: None,
            source_result_refs: aggregate.source_load_ids.clone(),
            metadata: Some(ResultMetadata {
                component: "expansion_joint_pressure_thrust".to_string(),
                coordinate_system: "element_local".to_string(),
                location: aggregate.input.pipe_id.clone(),
                basis: format!(
                    "component_family=expansion_joint;pressure_thrust_generation=load_side_user_effective_area;effective_area={};source={};pressure_thrust={};solver_consumption={}",
                    rounded_scalar(aggregate.input.effective_area),
                    aggregate.input.source_reference,
                    aggregate.input.pressure_thrust_reference,
                    aggregate.input.solver_consumption
                ),
                sign_convention:
                    "positive value is explicit pressure multiplied by user-entered effective pressure area and applied as equal/opposite axial load along the mapped pipe; no compliance claim is made"
                        .to_string(),
            }),
        });
        let mut affected_refs = vec![
            aggregate.input.component_id.clone(),
            aggregate.input.pipe_id.clone(),
            load_case.id.clone(),
            result_id,
            aggregate.input.pressure_thrust_reference.clone(),
        ];
        affected_refs.extend(aggregate.source_load_ids.clone());
        diagnostics.push(diag(
            &format!(
                "diagnostic:pressure-thrust:{}:{}",
                stable_suffix(&load_case.id),
                component_suffix
            ),
            "EXPANSION_JOINT_PRESSURE_THRUST_APPLIED",
            "info",
            format!(
                "expansion joint {} pressure thrust uses explicit effective area {} m^2 and pressure primitive(s) from load case {}; applied on load side along {}; no protected/default manufacturer value is supplied",
                aggregate.input.component_id,
                rounded_scalar(aggregate.input.effective_area),
                load_case.id,
                aggregate.input.pipe_id
            ),
            affected_refs,
        ));
        appended += 1;
    }
    appended
}

#[derive(Debug, Clone, Copy)]
struct StationResultants {
    location: &'static str,
    resultants: [f64; 6],
}

fn append_element_force_results(
    results: &mut Vec<ResultItem>,
    pipe_id: &str,
    local_forces: &[f64],
) {
    let suffix = stable_suffix(pipe_id);
    let components = [
        (
            format!("result:force:{suffix}:axial"),
            format!("result:force:{suffix}:axial:end-j"),
            "element_local_axial_force",
            "axial_force",
            UX,
            "N",
        ),
        (
            format!("result:force:{suffix}:shear-y"),
            format!("result:force:{suffix}:shear-y:end-j"),
            "element_local_shear_force_y",
            "shear_force_y",
            UY,
            "N",
        ),
        (
            format!("result:force:{suffix}:shear-z"),
            format!("result:force:{suffix}:shear-z:end-j"),
            "element_local_shear_force_z",
            "shear_force_z",
            UZ,
            "N",
        ),
        (
            format!("result:moment:{suffix}:torsion"),
            format!("result:moment:{suffix}:torsion:end-j"),
            "element_local_torsional_moment",
            "torsional_moment",
            RX,
            "N*m",
        ),
        (
            format!("result:moment:{suffix}:bending-y"),
            format!("result:moment:{suffix}:bending-y:end-j"),
            "element_local_bending_moment_y",
            "bending_moment_y",
            RY,
            "N*m",
        ),
        (
            format!("result:moment:{suffix}:bending-z"),
            format!("result:moment:{suffix}:bending-z:end-j"),
            "element_local_bending_moment_z",
            "bending_moment_z",
            RZ,
            "N*m",
        ),
    ];
    for (end_i_id, end_j_id, kind, component, dof, unit) in components {
        append_endpoint_force_result(
            results,
            pipe_id,
            &end_i_id,
            kind,
            component,
            local_forces[dof],
            unit,
            "end_i",
            "positive value follows the element-local DOF at the i-end force vector",
        );
        append_endpoint_force_result(
            results,
            pipe_id,
            &end_j_id,
            kind,
            component,
            local_forces[DOF_PER_NODE + dof],
            unit,
            "end_j",
            "positive value follows the element-local DOF at the j-end force vector",
        );
    }
}

fn append_node_displacement_component_results(
    results: &mut Vec<ResultItem>,
    node_id: &str,
    displacements: &[f64],
    node_index: usize,
) {
    let suffix = stable_suffix(node_id);
    let base = node_index * DOF_PER_NODE;
    let components = [
        (
            "ux",
            "global_nodal_displacement_x",
            "nodal_displacement_x",
            UX,
            1000.0,
            "mm",
            "positive value follows the global cartesian X axis displacement of the node",
        ),
        (
            "uy",
            "global_nodal_displacement_y",
            "nodal_displacement_y",
            UY,
            1000.0,
            "mm",
            "positive value follows the global cartesian Y axis displacement of the node",
        ),
        (
            "uz",
            "global_nodal_displacement_z",
            "nodal_displacement_z",
            UZ,
            1000.0,
            "mm",
            "positive value follows the global cartesian Z axis displacement of the node",
        ),
        (
            "rx",
            "global_nodal_rotation_x",
            "nodal_rotation_x",
            RX,
            1.0,
            "rad",
            "positive value follows the right-hand-rule rotation about the global cartesian X axis",
        ),
        (
            "ry",
            "global_nodal_rotation_y",
            "nodal_rotation_y",
            RY,
            1.0,
            "rad",
            "positive value follows the right-hand-rule rotation about the global cartesian Y axis",
        ),
        (
            "rz",
            "global_nodal_rotation_z",
            "nodal_rotation_z",
            RZ,
            1.0,
            "rad",
            "positive value follows the right-hand-rule rotation about the global cartesian Z axis",
        ),
    ];
    for (id_tail, kind, component, dof, scale, unit, sign_convention) in components {
        results.push(ResultItem {
            id: format!("result:disp:{suffix}:{id_tail}"),
            kind: kind.to_string(),
            value: round6(displacements[base + dof] * scale),
            unit: unit.to_string(),
            entity_ref: node_id.to_string(),
            basis_ref: None,
            source_result_refs: Vec::new(),
            metadata: Some(ResultMetadata {
                component: component.to_string(),
                coordinate_system: "global".to_string(),
                location: "node".to_string(),
                basis: "solved_from_global_linear_system".to_string(),
                sign_convention: sign_convention.to_string(),
            }),
        });
    }
}

fn station_grid_resultants_from_endpoints(local_forces: &[f64]) -> [StationResultants; 3] {
    [
        StationResultants {
            location: "quarter_1",
            resultants: station_resultants_from_endpoints(local_forces, 0.25),
        },
        StationResultants {
            location: "midspan",
            resultants: station_resultants_from_endpoints(local_forces, 0.5),
        },
        StationResultants {
            location: "quarter_3",
            resultants: station_resultants_from_endpoints(local_forces, 0.75),
        },
    ]
}

fn station_resultants_from_endpoints(local_forces: &[f64], fraction: f64) -> [f64; 6] {
    [
        interpolate_endpoint_resultant(local_forces[UX], local_forces[DOF_PER_NODE + UX], fraction),
        interpolate_endpoint_resultant(local_forces[UY], local_forces[DOF_PER_NODE + UY], fraction),
        interpolate_endpoint_resultant(local_forces[UZ], local_forces[DOF_PER_NODE + UZ], fraction),
        interpolate_endpoint_resultant(local_forces[RX], local_forces[DOF_PER_NODE + RX], fraction),
        interpolate_endpoint_resultant(local_forces[RY], local_forces[DOF_PER_NODE + RY], fraction),
        interpolate_endpoint_resultant(local_forces[RZ], local_forces[DOF_PER_NODE + RZ], fraction),
    ]
}

fn interpolate_endpoint_resultant(end_i: f64, end_j: f64, fraction: f64) -> f64 {
    end_i + (end_j - end_i) * fraction
}

fn append_station_force_results(
    results: &mut Vec<ResultItem>,
    pipe_id: &str,
    location: &str,
    resultants: &[f64; 6],
    basis: &str,
    sign_convention: &str,
    coordinate_system: &str,
) {
    let suffix = stable_suffix(pipe_id);
    let station = station_id_location(location);
    let components = [
        (
            format!("result:force:{suffix}:{station}:axial"),
            "element_local_axial_force",
            "axial_force",
            resultants[0],
            "N",
        ),
        (
            format!("result:force:{suffix}:{station}:shear-y"),
            "element_local_shear_force_y",
            "shear_force_y",
            resultants[1],
            "N",
        ),
        (
            format!("result:force:{suffix}:{station}:shear-z"),
            "element_local_shear_force_z",
            "shear_force_z",
            resultants[2],
            "N",
        ),
        (
            format!("result:moment:{suffix}:{station}:torsion"),
            "element_local_torsional_moment",
            "torsional_moment",
            resultants[3],
            "N*m",
        ),
        (
            format!("result:moment:{suffix}:{station}:bending-y"),
            "element_local_bending_moment_y",
            "bending_moment_y",
            resultants[4],
            "N*m",
        ),
        (
            format!("result:moment:{suffix}:{station}:bending-z"),
            "element_local_bending_moment_z",
            "bending_moment_z",
            resultants[5],
            "N*m",
        ),
    ];
    for (id, kind, component, value, unit) in components {
        append_station_force_result(
            results,
            pipe_id,
            &id,
            kind,
            component,
            value,
            unit,
            location,
            basis,
            sign_convention,
            coordinate_system,
        );
    }
}

fn append_endpoint_force_result(
    results: &mut Vec<ResultItem>,
    pipe_id: &str,
    id: &str,
    kind: &str,
    component: &str,
    value: f64,
    unit: &str,
    location: &str,
    sign_convention: &str,
) {
    results.push(ResultItem {
        id: id.to_string(),
        kind: kind.to_string(),
        value: round6(value),
        unit: unit.to_string(),
        entity_ref: pipe_id.to_string(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            component: component.to_string(),
            coordinate_system: "element_local".to_string(),
            location: location.to_string(),
            basis: "recovered_from_local_element_stiffness".to_string(),
            sign_convention: sign_convention.to_string(),
        }),
    });
}

#[allow(clippy::too_many_arguments)]
fn append_station_force_result(
    results: &mut Vec<ResultItem>,
    pipe_id: &str,
    id: &str,
    kind: &str,
    component: &str,
    value: f64,
    unit: &str,
    location: &str,
    basis: &str,
    sign_convention: &str,
    coordinate_system: &str,
) {
    results.push(ResultItem {
        id: id.to_string(),
        kind: kind.to_string(),
        value: round6(value),
        unit: unit.to_string(),
        entity_ref: pipe_id.to_string(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            component: component.to_string(),
            coordinate_system: coordinate_system.to_string(),
            location: location.to_string(),
            basis: basis.to_string(),
            sign_convention: sign_convention.to_string(),
        }),
    });
}

fn recover_endpoint_stress(
    local_forces: &[f64],
    offset: usize,
    section: &DerivedSection,
    pressure: Option<f64>,
) -> open_pipe_stress_stress_recovery::StressRecoveryResult {
    recover_stresses(&StressRecoveryInput {
        resultants: ForceResultants::new(
            Some(local_forces[offset + UX]),
            Some(local_forces[offset + RY]),
            Some(local_forces[offset + RZ]),
            Some(local_forces[offset + RX]),
        ),
        section: StressSectionProperties::new(
            Some(section.area),
            Some(section.section_modulus),
            Some(section.section_modulus),
            Some(section.torsion_constant),
            Some(section.torsion_radius),
        ),
        pressure: pressure.map(|p| {
            PressureBasis::new(
                Some(p),
                Some(section.membrane_radius),
                Some(section.wall_thickness),
            )
        }),
        statuses: vec![AnalysisStatus::MechanicsSolved],
    })
}

fn recover_station_stress(
    resultants: &[f64; 6],
    section: &DerivedSection,
    pressure: Option<f64>,
) -> open_pipe_stress_stress_recovery::StressRecoveryResult {
    recover_stresses(&StressRecoveryInput {
        resultants: ForceResultants::new(
            Some(resultants[0]),
            Some(resultants[4]),
            Some(resultants[5]),
            Some(resultants[3]),
        ),
        section: StressSectionProperties::new(
            Some(section.area),
            Some(section.section_modulus),
            Some(section.section_modulus),
            Some(section.torsion_constant),
            Some(section.torsion_radius),
        ),
        pressure: pressure.map(|p| {
            PressureBasis::new(
                Some(p),
                Some(section.membrane_radius),
                Some(section.wall_thickness),
            )
        }),
        statuses: vec![AnalysisStatus::MechanicsSolved],
    })
}

fn open_formula_summary_mpa(
    stress: &open_pipe_stress_stress_recovery::StressRecoveryResult,
    include_pressure_longitudinal: bool,
) -> Option<f64> {
    if !stress.findings.is_empty() {
        return None;
    }
    let components = &stress.components;
    let axial = components.axial_normal.unwrap_or(0.0);
    let pressure_longitudinal = if include_pressure_longitudinal {
        components.pressure_longitudinal.unwrap_or(0.0)
    } else {
        0.0
    };
    let bending_y = components.bending_normal_y.unwrap_or(0.0).abs();
    let bending_z = components.bending_normal_z.unwrap_or(0.0).abs();
    let base_normal = axial + pressure_longitudinal;
    let bending_total = bending_y + bending_z;
    Some(
        (base_normal + bending_total)
            .abs()
            .max((base_normal - bending_total).abs())
            / 1_000_000.0,
    )
}

fn append_component_stress_multiplier_results(
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
    model: &PreviewModel,
    pipe_id: &str,
    end_i_stress: &open_pipe_stress_stress_recovery::StressRecoveryResult,
    end_j_stress: &open_pipe_stress_stress_recovery::StressRecoveryResult,
    include_pressure_longitudinal: bool,
) -> usize {
    let Some(pipe) = model
        .pipe_segments
        .iter()
        .find(|candidate| candidate.id == pipe_id)
    else {
        return 0;
    };
    let endpoint_stresses = [
        ("end_i", pipe.from.as_str(), end_i_stress),
        ("end_j", pipe.to.as_str(), end_j_stress),
    ];
    let mut appended = 0;
    for (location, node_id, stress) in endpoint_stresses {
        let Some(base_value_mpa) = open_formula_summary_mpa(stress, include_pressure_longitudinal)
        else {
            continue;
        };
        for component in model
            .components
            .iter()
            .filter(|component| component.node == node_id)
        {
            let Some(modifier) = component_stress_modifier_for_pipe(component, pipe_id) else {
                continue;
            };
            append_component_stress_multiplier_result(
                results,
                diagnostics,
                component,
                pipe_id,
                location,
                base_value_mpa,
                modifier,
            );
            appended += 1;
        }
    }
    appended
}

#[derive(Debug, Clone, Copy)]
struct ComponentStressModifier<'a> {
    family: &'a str,
    side: &'a str,
    sif: f64,
    flexibility: f64,
    /// DEC-070: when the flexibility factor is realized in the assembled
    /// curved-bend macro-element stiffness, the stress-review multiplier
    /// applies the user-entered SIF only (no double-counting of k).
    flexibility_in_assembled_stiffness: bool,
    source_reference: &'a str,
    solver_consumption: &'a str,
}

fn component_stress_modifier_for_pipe<'a>(
    component: &'a PreviewComponent,
    pipe_id: &str,
) -> Option<ComponentStressModifier<'a>> {
    if is_bend_component(component) {
        return bend_stress_modifier(component);
    }
    if is_branch_component(component) {
        return branch_stress_modifier_for_pipe(component, pipe_id);
    }
    None
}

fn bend_stress_modifier(component: &PreviewComponent) -> Option<ComponentStressModifier<'_>> {
    let solver_consumption = component
        .mechanics_interface
        .as_ref()
        .and_then(|interface| interface.solver_consumption.as_deref())
        .unwrap_or("mechanics_geometry_only");
    let flexibility_in_assembled_stiffness =
        solver_consumption == DEC_070_CURVED_BEND_SOLVER_CONSUMPTION;
    if solver_consumption != "mechanics_geometry_only" && !flexibility_in_assembled_stiffness {
        return None;
    }
    let modifiers = component.modifiers.as_ref()?;
    let sif = modifiers.sif_user_value.as_ref()?.value;
    let flexibility = modifiers.flexibility_factor_user_value.as_ref()?.value;
    if !positive_finite(sif) || !positive_finite(flexibility) {
        return None;
    }
    let source_reference = modifiers
        .source_reference
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("source_reference_missing");
    Some(ComponentStressModifier {
        family: "bend",
        side: "through",
        sif,
        flexibility,
        flexibility_in_assembled_stiffness,
        source_reference,
        solver_consumption,
    })
}

fn branch_stress_modifier_for_pipe<'a>(
    component: &'a PreviewComponent,
    pipe_id: &str,
) -> Option<ComponentStressModifier<'a>> {
    let solver_consumption = component
        .mechanics_interface
        .as_ref()
        .and_then(|interface| interface.solver_consumption.as_deref())
        .unwrap_or("mechanics_geometry_only");
    if solver_consumption != "mechanics_geometry_only" {
        return None;
    }
    let geometry = component.geometry.as_ref()?;
    let modifiers = component.modifiers.as_ref()?;
    let (side, sif) = if geometry
        .branch_header_pipe_ref
        .as_deref()
        .filter(|value| *value == pipe_id)
        .is_some()
    {
        (
            "header",
            modifiers.branch_header_sif_user_value.as_ref()?.value,
        )
    } else if geometry
        .branch_branch_pipe_ref
        .as_deref()
        .filter(|value| *value == pipe_id)
        .is_some()
    {
        (
            "branch",
            modifiers.branch_branch_sif_user_value.as_ref()?.value,
        )
    } else {
        return None;
    };
    let flexibility = modifiers.flexibility_factor_user_value.as_ref()?.value;
    if !positive_finite(sif) || !positive_finite(flexibility) {
        return None;
    }
    let source_reference = modifiers
        .source_reference
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("source_reference_missing");
    Some(ComponentStressModifier {
        family: "branch",
        side,
        sif,
        flexibility,
        flexibility_in_assembled_stiffness: false,
        source_reference,
        solver_consumption,
    })
}

fn append_component_stress_multiplier_result(
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
    component: &PreviewComponent,
    pipe_id: &str,
    location: &str,
    base_value_mpa: f64,
    modifier: ComponentStressModifier<'_>,
) {
    let component_suffix = stable_suffix(&component.id);
    let pipe_suffix = stable_suffix(pipe_id);
    let endpoint = endpoint_id_location(location);
    let result_id =
        format!("result:stress:{component_suffix}:{pipe_suffix}:{endpoint}:user-multiplier");
    // DEC-070 no-double-counting rule: when the flexibility factor is realized
    // in the assembled curved-bend macro-element stiffness, the review
    // multiplier applies the user-entered SIF only; the legacy
    // mechanics_geometry_only mode keeps sif * flexibility byte-identically.
    let multiplier = if modifier.flexibility_in_assembled_stiffness {
        modifier.sif
    } else {
        modifier.sif * modifier.flexibility
    };
    let value = round6(base_value_mpa * multiplier);
    let mut basis = format!(
        "component_family={};component_side={};user_entered_sif={};user_entered_flexibility={};source={};solver_consumption={}",
        modifier.family,
        modifier.side,
        rounded_scalar(modifier.sif),
        rounded_scalar(modifier.flexibility),
        modifier.source_reference,
        modifier.solver_consumption
    );
    if modifier.flexibility_in_assembled_stiffness {
        basis.push_str(";flexibility_realization=assembled_curved_bend_macro_element_stiffness");
    }
    let sign_convention = if modifier.flexibility_in_assembled_stiffness {
        "positive value is base open-mechanics stress summary multiplied by the user-entered SIF only; the user-entered flexibility factor enters the assembled curved-bend macro-element stiffness"
    } else {
        "positive value is base open-mechanics stress summary multiplied by user-entered component modifiers; base frame stiffness unchanged"
    };
    results.push(ResultItem {
        id: result_id.clone(),
        kind: "component_user_stress_multiplier_review".to_string(),
        value,
        unit: "MPa".to_string(),
        entity_ref: component.id.clone(),
        basis_ref: None,
        source_result_refs: endpoint_stress_source_refs(pipe_id, location),
        metadata: Some(ResultMetadata {
            component: "user_entered_component_stress_multiplier".to_string(),
            coordinate_system: "component_review".to_string(),
            location: format!("{pipe_id}:{location}"),
            basis,
            sign_convention: sign_convention.to_string(),
        }),
    });
    let message = if modifier.flexibility_in_assembled_stiffness {
        format!(
            "{} component {} applies user-entered {} SIF {} to {} {location} stress-recovery review; user-entered flexibility factor {} enters the assembled curved-bend macro-element stiffness; solver_consumption is {}; no protected or default component factor is supplied",
            modifier.family,
            component.id,
            modifier.side,
            rounded_scalar(modifier.sif),
            pipe_id,
            rounded_scalar(modifier.flexibility),
            modifier.solver_consumption
        )
    } else {
        format!(
            "{} component {} applies user-entered {} SIF {} and flexibility factor {} to {} {location} stress-recovery review; solver_consumption remains {}; no protected or default component factor is supplied",
            modifier.family,
            component.id,
            modifier.side,
            rounded_scalar(modifier.sif),
            rounded_scalar(modifier.flexibility),
            pipe_id,
            modifier.solver_consumption
        )
    };
    diagnostics.push(diag(
        &format!(
            "diagnostic:component-stress-multiplier:{}:{}:{}",
            component_suffix, pipe_suffix, endpoint
        ),
        "COMPONENT_STRESS_MULTIPLIER_APPLIED",
        "info",
        message,
        vec![
            component.id.clone(),
            pipe_id.to_string(),
            result_id,
            modifier.source_reference.to_string(),
        ],
    ));
}

fn append_expansion_joint_user_stiffness_results(
    model: &PreviewModel,
    results: &mut Vec<ResultItem>,
) -> usize {
    let mut appended = 0;
    for component in model
        .components
        .iter()
        .filter(|component| is_expansion_joint_component(component))
    {
        let solver_consumption = component
            .mechanics_interface
            .as_ref()
            .and_then(|interface| interface.solver_consumption.as_deref())
            .unwrap_or("not_provided");
        if solver_consumption != "mechanics_geometry_and_user_flexibility" {
            continue;
        }
        let Some(geometry) = component.geometry.as_ref() else {
            continue;
        };
        let Some(modifiers) = component.modifiers.as_ref() else {
            continue;
        };
        let Some(pipe_ref) = geometry
            .expansion_joint_pipe_ref
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        else {
            continue;
        };
        let source_reference = modifiers
            .source_reference
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("source_reference_missing");
        let entries = [
            (
                "axial",
                "axial_user_stiffness",
                "N/m",
                modifiers.axial_stiffness_user_value.as_ref(),
            ),
            (
                "lateral",
                "lateral_user_stiffness",
                "N/m",
                modifiers.lateral_stiffness_user_value.as_ref(),
            ),
            (
                "angular",
                "angular_user_stiffness",
                "N*m/rad",
                modifiers.angular_stiffness_user_value.as_ref(),
            ),
            (
                "torsional",
                "torsional_user_stiffness",
                "N*m/rad",
                modifiers.torsional_stiffness_user_value.as_ref(),
            ),
        ];
        for (axis, metadata_component, unit, quantity) in entries {
            let Some(quantity) = quantity else {
                continue;
            };
            if !positive_finite(quantity.value) {
                continue;
            }
            let component_suffix = stable_suffix(&component.id);
            let result_id = format!("result:component-stiffness:{component_suffix}:{axis}");
            results.push(ResultItem {
                id: result_id,
                kind: "component_user_stiffness_macro_element_review".to_string(),
                value: round6(quantity.value),
                unit: unit.to_string(),
                entity_ref: component.id.clone(),
                basis_ref: None,
                source_result_refs: Vec::new(),
                metadata: Some(ResultMetadata {
                    component: metadata_component.to_string(),
                    coordinate_system: "component_local_preview".to_string(),
                    location: pipe_ref.to_string(),
                    basis: format!(
                        "component_family=expansion_joint;user_entered_axis={axis};source={source_reference};solver_consumption={solver_consumption};macro_element_solve=assembled_user_stiffness;pressure_thrust_generation=load_side_user_effective_area;pressure_thrust={}",
                        geometry
                            .pressure_thrust_reference
                            .as_deref()
                            .unwrap_or("load_side_pressure_thrust_reference_missing")
                    ),
                    sign_convention:
                        "positive value is user-entered expansion-joint stiffness consumed by the assembled user-stiffness macro-element; pressure-thrust generation is load-side effective-area evidence and no compliance claim is made"
                            .to_string(),
                }),
            });
            appended += 1;
        }
    }
    appended
}

// DEC-070 review rows: state that the user-entered bend flexibility factor is
// consumed by the assembled curved-bend macro-element (EJ precedent wording
// style), and record the arc-geometry conventions and load/recovery decisions.
fn append_curved_bend_macro_element_results(
    curved_bend_elements: &[CurvedBendMacroBuild],
    results: &mut Vec<ResultItem>,
) -> usize {
    let mut appended = 0;
    for element in curved_bend_elements {
        let component_suffix = stable_suffix(&element.component_id);
        results.push(ResultItem {
            id: format!("result:component-stiffness:{component_suffix}:curved-bend-flexibility"),
            kind: "curved_bend_macro_element_review".to_string(),
            value: round6(element.flexibility_factor),
            unit: "unitless".to_string(),
            entity_ref: element.component_id.clone(),
            basis_ref: None,
            source_result_refs: Vec::new(),
            metadata: Some(ResultMetadata {
                component: "curved_bend_flexibility".to_string(),
                coordinate_system: "component_local_preview".to_string(),
                location: element.pipe_id.clone(),
                basis: format!(
                    "component_family=bend;user_entered_flexibility={};flexibility_axis_mapping=single_user_factor_applied_to_in_plane_and_out_of_plane_bending;bend_radius_m={};arc_included_angle_rad={};arc_length_m={};arc_plane=chord_and_pipe_y_reference;arc_side=bows_toward_positive_pipe_y_reference;source={};solver_consumption={};macro_element_solve=assembled_curved_bend_stiffness;thermal_load_treatment=exact_free_expansion_identity;distributed_load_treatment=arc_consistent_fixed_end_integration;pressure_thrust_treatment=straight_chord_axial_end_forces;recovery=end_forces_from_assembled_stiffness_in_chord_frame;interior_stations=arc_section_equilibrium_stations",
                    rounded_scalar(element.flexibility_factor),
                    rounded_scalar(element.bend_radius),
                    rounded_scalar(element.included_angle),
                    rounded_scalar(element.arc_length),
                    element.source_reference,
                    DEC_070_CURVED_BEND_SOLVER_CONSUMPTION
                ),
                sign_convention:
                    "positive value is the user-entered bend flexibility factor consumed by the assembled curved-bend macro-element stiffness; the stress-review multiplier applies the user-entered SIF only and no protected or default component factor is supplied"
                        .to_string(),
            }),
        });
        appended += 1;
    }
    appended
}

fn append_spring_hanger_user_input_results(
    model: &PreviewModel,
    results: &mut Vec<ResultItem>,
) -> usize {
    let mut appended = 0;
    for support in model
        .supports
        .iter()
        .filter(|support| is_variable_spring_hanger(support) || is_constant_effort_support(support))
    {
        let Some(hanger) = support.hanger.as_ref() else {
            continue;
        };
        let source_reference = hanger
            .source_reference
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("source_reference_missing");
        let manufacturer_reference = hanger
            .manufacturer_reference
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("manufacturer_reference_missing");
        let load_side_review = hanger
            .load_side_review_reference
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("load_side_review_reference_missing");
        let mechanics_consumption = hanger
            .mechanics_consumption
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("review_only");
        let suffix = stable_suffix(&support.id);

        if is_variable_spring_hanger(support) {
            if let Some(stiffness) = support_stiffness_input(support) {
                append_hanger_quantity_result(
                    results,
                    &format!("result:spring-hanger:{suffix}:stiffness"),
                    "spring_hanger_user_input_review",
                    support,
                    "variable_spring_hanger_stiffness",
                    &format!("{} stiffness", stiffness.dof),
                    &stiffness.value,
                    &format!(
                        "support_family=variable_spring_hanger;user_entered_dof={};source={source_reference};manufacturer={manufacturer_reference};mechanics_consumption={mechanics_consumption};dec_ref=DEC-049",
                        stiffness.dof
                    ),
                    "positive value is user-entered variable spring hanger stiffness consumed by the preview linear spring primitive; no catalog/default value is supplied",
                );
                appended += 1;
            }
            for (field, component, quantity) in [
                (
                    "installed-load",
                    "variable_spring_hanger_installed_load",
                    hanger.installed_load.as_ref(),
                ),
                (
                    "cold-load",
                    "variable_spring_hanger_cold_load",
                    hanger.cold_load.as_ref(),
                ),
                (
                    "hot-load",
                    "variable_spring_hanger_hot_load",
                    hanger.hot_load.as_ref(),
                ),
                (
                    "travel-range",
                    "variable_spring_hanger_travel_range",
                    hanger.travel_range.as_ref(),
                ),
                (
                    "movement-limit",
                    "variable_spring_hanger_movement_limit",
                    hanger.movement_limit.as_ref(),
                ),
            ] {
                let Some(quantity) = quantity else {
                    continue;
                };
                append_hanger_quantity_result(
                    results,
                    &format!("result:spring-hanger:{suffix}:{field}"),
                    "spring_hanger_user_input_review",
                    support,
                    component,
                    field,
                    quantity,
                    &format!(
                        "support_family=variable_spring_hanger;field={field};source={source_reference};manufacturer={manufacturer_reference};load_side_review={load_side_review};mechanics_consumption={mechanics_consumption};dec_ref=DEC-049"
                    ),
                    "positive value is user-entered variable spring hanger input evidence; load-side and travel review remain human-reviewed preview metadata",
                );
                appended += 1;
            }
        }

        if is_constant_effort_support(support) {
            for (field, component, quantity) in [
                (
                    "constant-load",
                    "constant_effort_support_constant_load",
                    hanger.constant_load.as_ref(),
                ),
                (
                    "travel-range",
                    "constant_effort_support_travel_range",
                    hanger.travel_range.as_ref(),
                ),
                (
                    "movement-limit",
                    "constant_effort_support_movement_limit",
                    hanger.movement_limit.as_ref(),
                ),
            ] {
                let Some(quantity) = quantity else {
                    continue;
                };
                append_hanger_quantity_result(
                    results,
                    &format!("result:constant-effort-support:{suffix}:{field}"),
                    "constant_effort_user_input_review",
                    support,
                    component,
                    field,
                    quantity,
                    &format!(
                        "support_family=constant_effort_support;field={field};source={source_reference};manufacturer={manufacturer_reference};load_side_review={load_side_review};mechanics_consumption={mechanics_consumption};dec_ref=DEC-049"
                    ),
                    "positive value is user-entered constant-effort support input evidence; no global constant-effort load or nonlinear behavior is claimed by this preview row",
                );
                appended += 1;
            }
        }
    }
    appended
}

fn append_hanger_quantity_result(
    results: &mut Vec<ResultItem>,
    id: &str,
    kind: &str,
    support: &PreviewSupport,
    metadata_component: &str,
    location: &str,
    quantity: &Quantity,
    basis: &str,
    sign_convention: &str,
) {
    if !positive_finite(quantity.value) {
        return;
    }
    results.push(ResultItem {
        id: id.to_string(),
        kind: kind.to_string(),
        value: round6(quantity.value),
        unit: quantity.unit.clone(),
        entity_ref: support.id.clone(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            component: metadata_component.to_string(),
            coordinate_system: "support_local_preview".to_string(),
            location: format!("{}:{location}", support.node),
            basis: basis.to_string(),
            sign_convention: sign_convention.to_string(),
        }),
    });
}

fn endpoint_stress_source_refs(pipe_id: &str, location: &str) -> Vec<String> {
    let suffix = stable_suffix(pipe_id);
    let endpoint = endpoint_id_location(location);
    [
        format!("result:stress:{suffix}:{endpoint}:axial-normal"),
        format!("result:stress:{suffix}:{endpoint}:bending-normal-y"),
        format!("result:stress:{suffix}:{endpoint}:bending-normal-z"),
        format!("result:stress:{suffix}:{endpoint}:torsional-shear"),
        format!("result:stress:{suffix}"),
    ]
    .into_iter()
    .collect()
}

fn is_bend_component(component: &PreviewComponent) -> bool {
    matches!(component.kind.as_str(), "bend" | "elbow")
}

fn is_branch_component(component: &PreviewComponent) -> bool {
    matches!(
        component.kind.as_str(),
        "branch" | "tee" | "branch_connection"
    )
}

fn is_expansion_joint_component(component: &PreviewComponent) -> bool {
    component.kind == "expansion_joint"
}

pub(crate) fn support_hanger_type(support: &PreviewSupport) -> Option<&str> {
    support
        .hanger
        .as_ref()
        .and_then(|hanger| hanger.hanger_type.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| support.family.as_deref().map(str::trim))
}

pub(crate) fn is_variable_spring_hanger(support: &PreviewSupport) -> bool {
    matches!(
        support_hanger_type(support),
        Some("variable_spring_hanger" | "spring_hanger")
    )
}

pub(crate) fn is_constant_effort_support(support: &PreviewSupport) -> bool {
    matches!(
        support_hanger_type(support),
        Some("constant_effort_support")
    )
}

pub(crate) fn support_stiffness_input(support: &PreviewSupport) -> Option<&SupportStiffnessInput> {
    support.stiffness.as_ref().or_else(|| {
        support
            .hanger
            .as_ref()
            .and_then(|hanger| hanger.stiffness.as_ref())
    })
}

fn positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn rounded_scalar(value: f64) -> String {
    let rounded = round6(value);
    if (rounded.fract()).abs() < 1.0e-9 {
        format!("{rounded:.0}")
    } else {
        format!("{rounded}")
    }
}

fn append_endpoint_stress_results(
    results: &mut Vec<ResultItem>,
    pipe_id: &str,
    location: &str,
    components: &StressComponents,
    include_pressure: bool,
    include_pressure_longitudinal: bool,
) {
    let suffix = stable_suffix(pipe_id);
    let id_location = endpoint_id_location(location);
    let local_components = [
        (
            "axial-normal",
            "element_local_axial_normal_stress",
            "axial_normal_stress",
            components.axial_normal,
            "positive normal stress follows the element-local axial resultant at this endpoint",
        ),
        (
            "bending-normal-y",
            "element_local_bending_normal_stress_y",
            "bending_normal_stress_y",
            components.bending_normal_y,
            "positive bending normal stress follows the element-local y bending resultant at this endpoint",
        ),
        (
            "bending-normal-z",
            "element_local_bending_normal_stress_z",
            "bending_normal_stress_z",
            components.bending_normal_z,
            "positive bending normal stress follows the element-local z bending resultant at this endpoint",
        ),
        (
            "torsional-shear",
            "element_local_torsional_shear_stress",
            "torsional_shear_stress",
            components.torsional_shear,
            "positive torsional shear stress follows the element-local torsional resultant at this endpoint",
        ),
    ];
    for (id_tail, kind, component, value, sign_convention) in local_components {
        if let Some(value) = value {
            append_endpoint_stress_result(
                results,
                pipe_id,
                &format!("result:stress:{suffix}:{id_location}:{id_tail}"),
                kind,
                component,
                value,
                "element_local",
                location,
                sign_convention,
            );
        }
    }

    if include_pressure {
        if let Some(value) = components.pressure_hoop {
            append_endpoint_stress_result(
                results,
                pipe_id,
                &format!("result:stress:{suffix}:{id_location}:pressure-hoop"),
                "pipe_section_pressure_hoop_stress",
                "pressure_hoop_stress",
                value,
                "pipe_section",
                location,
                "positive pressure membrane hoop stress follows the explicit pipe pressure basis",
            );
        }
        if include_pressure_longitudinal {
            if let Some(value) = components.pressure_longitudinal {
                append_endpoint_stress_result(
                    results,
                    pipe_id,
                    &format!("result:stress:{suffix}:{id_location}:pressure-longitudinal"),
                    "pipe_section_pressure_longitudinal_stress",
                    "pressure_longitudinal_stress",
                    value,
                    "pipe_section",
                    location,
                    "positive pressure membrane longitudinal stress follows the explicit pipe pressure basis",
                );
            }
        }
    }
}

fn append_station_stress_results(
    results: &mut Vec<ResultItem>,
    pipe_id: &str,
    location: &str,
    components: &StressComponents,
    include_pressure: bool,
    include_pressure_longitudinal: bool,
    basis: &str,
) {
    let suffix = stable_suffix(pipe_id);
    let station = station_id_location(location);
    let local_components = [
        (
            "axial-normal",
            "element_local_axial_normal_stress",
            "axial_normal_stress",
            components.axial_normal,
            "positive normal stress follows the interpolated element-local axial resultant at this station",
        ),
        (
            "bending-normal-y",
            "element_local_bending_normal_stress_y",
            "bending_normal_stress_y",
            components.bending_normal_y,
            "positive bending normal stress follows the interpolated element-local y bending resultant at this station",
        ),
        (
            "bending-normal-z",
            "element_local_bending_normal_stress_z",
            "bending_normal_stress_z",
            components.bending_normal_z,
            "positive bending normal stress follows the interpolated element-local z bending resultant at this station",
        ),
        (
            "torsional-shear",
            "element_local_torsional_shear_stress",
            "torsional_shear_stress",
            components.torsional_shear,
            "positive torsional shear stress follows the interpolated element-local torsional resultant at this station",
        ),
    ];
    for (id_tail, kind, component, value, sign_convention) in local_components {
        if let Some(value) = value {
            append_station_stress_result(
                results,
                pipe_id,
                &format!("result:stress:{suffix}:{station}:{id_tail}"),
                kind,
                component,
                value,
                "element_local",
                location,
                basis,
                sign_convention,
            );
        }
    }

    if include_pressure {
        if let Some(value) = components.pressure_hoop {
            append_station_stress_result(
                results,
                pipe_id,
                &format!("result:stress:{suffix}:{station}:pressure-hoop"),
                "pipe_section_pressure_hoop_stress",
                "pressure_hoop_stress",
                value,
                "pipe_section",
                location,
                basis,
                "positive pressure membrane hoop stress follows the explicit pipe pressure basis at this station",
            );
        }
        if include_pressure_longitudinal {
            if let Some(value) = components.pressure_longitudinal {
                append_station_stress_result(
                    results,
                    pipe_id,
                    &format!("result:stress:{suffix}:{station}:pressure-longitudinal"),
                    "pipe_section_pressure_longitudinal_stress",
                    "pressure_longitudinal_stress",
                    value,
                    "pipe_section",
                    location,
                    basis,
                    "positive pressure membrane longitudinal stress follows the explicit pipe pressure basis at this station",
                );
            }
        }
    }
}

fn append_endpoint_stress_result(
    results: &mut Vec<ResultItem>,
    pipe_id: &str,
    id: &str,
    kind: &str,
    component: &str,
    value_pa: f64,
    coordinate_system: &str,
    location: &str,
    sign_convention: &str,
) {
    results.push(ResultItem {
        id: id.to_string(),
        kind: kind.to_string(),
        value: round6(value_pa / 1_000_000.0),
        unit: "MPa".to_string(),
        entity_ref: pipe_id.to_string(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            component: component.to_string(),
            coordinate_system: coordinate_system.to_string(),
            location: location.to_string(),
            basis: "recovered_from_open_mechanics_stress_components".to_string(),
            sign_convention: sign_convention.to_string(),
        }),
    });
}

fn append_station_stress_result(
    results: &mut Vec<ResultItem>,
    pipe_id: &str,
    id: &str,
    kind: &str,
    component: &str,
    value_pa: f64,
    coordinate_system: &str,
    location: &str,
    basis: &str,
    sign_convention: &str,
) {
    results.push(ResultItem {
        id: id.to_string(),
        kind: kind.to_string(),
        value: round6(value_pa / 1_000_000.0),
        unit: "MPa".to_string(),
        entity_ref: pipe_id.to_string(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            component: component.to_string(),
            coordinate_system: coordinate_system.to_string(),
            location: location.to_string(),
            basis: basis.to_string(),
            sign_convention: sign_convention.to_string(),
        }),
    });
}

fn endpoint_id_location(location: &str) -> &str {
    match location {
        "end_i" => "end-i",
        "end_j" => "end-j",
        other => other,
    }
}

fn station_id_location(location: &str) -> &str {
    match location {
        "quarter_1" => "quarter-1",
        "quarter_3" => "quarter-3",
        other => other,
    }
}

/// Per-basis combination expression over solved load-case result rows;
/// the variants mirror `open_pipe_stress_load_case_algebra::AlgebraExpression`.
enum CombinationExpression<'a> {
    Mechanics {
        terms: &'a [PreviewCombinationTerm],
    },
    ResultStateSubtraction {
        minuend_id: &'a str,
        subtrahend_id: &'a str,
    },
    RangeEnvelope {
        ordered_operand_ids: Vec<String>,
        mode: RangeMode,
    },
}

impl<'a> CombinationExpression<'a> {
    /// Structural shape problems are blocked pre-solve by
    /// `validate_combinations`; an unresolvable shape here is unreachable for
    /// solved models and is skipped exactly like the previous empty-terms
    /// guard (the named blocking diagnostic already exists).
    fn resolve(combination: &'a PreviewCombination) -> Option<Self> {
        match combination.basis.as_str() {
            "mechanics" => {
                if combination.terms.is_empty() {
                    return None;
                }
                Some(Self::Mechanics {
                    terms: &combination.terms,
                })
            }
            "result_state_subtraction" => Some(Self::ResultStateSubtraction {
                minuend_id: combination.minuend_id.as_deref()?,
                subtrahend_id: combination.subtrahend_id.as_deref()?,
            }),
            "range_envelope" => {
                let mut ordered_operand_ids = combination.operand_ids.clone()?;
                if ordered_operand_ids.is_empty() {
                    return None;
                }
                ordered_operand_ids.sort_unstable();
                Some(Self::RangeEnvelope {
                    ordered_operand_ids,
                    mode: RangeMode::parse_token(combination.mode.as_deref()?)?,
                })
            }
            _ => None,
        }
    }

    /// Operand load-case ids in the deterministic order used for source
    /// lookup and `source_result_refs` (mechanics: authored term order;
    /// subtraction: minuend then subtrahend; range: sorted operand ids,
    /// matching the algebra crate's evaluation order).
    fn operand_ids(&self) -> Vec<&str> {
        match self {
            Self::Mechanics { terms } => terms.iter().map(|term| term.load_case.as_str()).collect(),
            Self::ResultStateSubtraction {
                minuend_id,
                subtrahend_id,
            } => vec![minuend_id, subtrahend_id],
            Self::RangeEnvelope {
                ordered_operand_ids,
                ..
            } => ordered_operand_ids.iter().map(String::as_str).collect(),
        }
    }

    /// The operand whose row supplies result identity (kind, unit, entity,
    /// metadata) for the combined row.
    fn reference_operand_id(&self) -> &str {
        match self {
            Self::Mechanics { terms } => terms[0].load_case.as_str(),
            Self::ResultStateSubtraction { minuend_id, .. } => minuend_id,
            Self::RangeEnvelope {
                ordered_operand_ids,
                ..
            } => ordered_operand_ids[0].as_str(),
        }
    }

    fn evaluate(&self, operand_by_id: &HashMap<&str, &AlgebraOperand>) -> AlgebraResult {
        match self {
            Self::Mechanics { terms } => {
                let algebra_terms = terms
                    .iter()
                    .filter_map(|term| {
                        CombinationTerm::new(term.load_case.clone(), term.factor).ok()
                    })
                    .collect::<Vec<_>>();
                evaluate_linear_combination(operand_by_id, &algebra_terms)
            }
            Self::ResultStateSubtraction {
                minuend_id,
                subtrahend_id,
            } => evaluate_result_state_subtraction(operand_by_id, minuend_id, subtrahend_id),
            Self::RangeEnvelope {
                ordered_operand_ids,
                mode,
            } => evaluate_range_envelope(operand_by_id, ordered_operand_ids, *mode),
        }
    }

    fn result_metadata_basis(&self) -> &'static str {
        match self {
            Self::Mechanics { .. } => "explicit_user_linear_combination",
            Self::ResultStateSubtraction { .. } => "explicit_user_result_state_subtraction",
            Self::RangeEnvelope { .. } => "explicit_user_range_envelope",
        }
    }

    fn result_sign_convention(&self) -> String {
        match self {
            Self::Mechanics { .. } => {
                "positive value follows explicit user linear combination of matching source result sign conventions"
                    .to_string()
            }
            Self::ResultStateSubtraction { .. } => {
                "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions"
                    .to_string()
            }
            Self::RangeEnvelope { mode, .. } => format!(
                "value is the {} mode-selected source result across explicit user range-envelope operands and keeps that source result sign convention",
                mode.token()
            ),
        }
    }
}

fn append_combination_results(
    model: &PreviewModel,
    rows_by_base_id: &HashMap<String, HashMap<String, ResultItem>>,
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut base_ids = rows_by_base_id.keys().cloned().collect::<Vec<_>>();
    base_ids.sort();

    for combination in &model.combinations {
        let Some(expression) = CombinationExpression::resolve(combination) else {
            continue;
        };
        for base_id in &base_ids {
            let Some(source_rows) = rows_by_base_id.get(base_id) else {
                continue;
            };
            let reference_operand_id = expression.reference_operand_id();
            let Some(reference_row) = source_rows.get(reference_operand_id) else {
                diagnostics.push(combination_diag(
                    combination,
                    base_id,
                    "LOAD_COMBINATION_SOURCE_RESULT_MISSING",
                    "warning",
                    format!(
                        "combination source result {base_id} is not available for load case {reference_operand_id}"
                    ),
                    vec![combination.id.clone(), reference_operand_id.to_string()],
                ));
                continue;
            };
            if reference_row.kind == "open_formula_stress_summary" {
                diagnostics.push(combination_diag(
                    combination,
                    base_id,
                    "COMBINATION_STRESS_SUMMARY_SKIPPED",
                    "warning",
                    "open-formula stress summary rows are not linearly combined in TP-MAC-08; combine inspectable stress component rows instead",
                    vec![combination.id.clone(), reference_row.id.clone()],
                ));
                continue;
            }
            if is_combination_excluded_result_kind(&reference_row.kind) {
                continue;
            }
            let Some(dimension) = algebra_dimension(reference_row) else {
                diagnostics.push(combination_diag(
                    combination,
                    base_id,
                    "LOAD_COMBINATION_RESULT_DIMENSION_UNSUPPORTED",
                    "warning",
                    "result row unit is not supported for TP-MAC-08 scalar load-combination algebra",
                    vec![combination.id.clone(), reference_row.id.clone()],
                ));
                continue;
            };

            let mut operands = Vec::new();
            let mut source_result_refs = Vec::new();
            let mut source_identity_mismatch = false;
            for operand_id in expression.operand_ids() {
                let Some(source) = source_rows.get(operand_id) else {
                    continue;
                };
                if !combination_source_identity_matches(reference_row, source) {
                    source_identity_mismatch = true;
                    diagnostics.push(combination_diag(
                        combination,
                        base_id,
                        "LOAD_COMBINATION_SOURCE_RESULT_MISMATCH",
                        "warning",
                        "combination source rows must match kind, unit, entity, and result metadata",
                        vec![
                            combination.id.clone(),
                            reference_row.id.clone(),
                            source.id.clone(),
                        ],
                    ));
                    continue;
                }
                operands.push(AlgebraOperand::new(
                    operand_id.to_string(),
                    operand_id.to_string(),
                    AlgebraQuantity::new(source.value, dimension)
                        .expect("result values are finite preview mechanics outputs"),
                    vec![
                        AlgebraAnalysisStatus::MechanicsSolved,
                        AlgebraAnalysisStatus::HumanReviewRequired,
                    ],
                ));
                source_result_refs.push(source.id.clone());
            }
            if source_identity_mismatch {
                continue;
            }
            let operand_by_id = operands
                .iter()
                .map(|operand| (operand.operand_id.as_str(), operand))
                .collect::<HashMap<_, _>>();
            let algebra = expression.evaluate(&operand_by_id);
            if algebra.is_blocked() {
                for finding in algebra.findings {
                    diagnostics.push(combination_diag(
                        combination,
                        base_id,
                        algebra_finding_diagnostic_code(finding.code),
                        "warning",
                        finding.message,
                        vec![combination.id.clone(), finding.subject_id],
                    ));
                }
                continue;
            }
            let Some(quantity) = algebra.quantity else {
                continue;
            };
            let mut combined = reference_row.clone();
            combined.id = qualified_combination_result_id(&combination.id, base_id);
            combined.value = round6(quantity.value);
            combined.basis_ref = Some(ResultBasisRef {
                ref_type: "combination".to_string(),
                ref_id: combination.id.clone(),
            });
            combined.source_result_refs = source_result_refs;
            if let Some(metadata) = combined.metadata.as_mut() {
                metadata.basis = expression.result_metadata_basis().to_string();
                metadata.sign_convention = expression.result_sign_convention();
            }
            results.push(combined);
        }
    }
}

fn append_nonlinear_friction_normal_evidence(
    results: &mut Vec<ResultItem>,
    friction_normal_reactions: &[FrictionNormalReaction],
    derived_friction_normal_reactions: &[DerivedFrictionNormalReaction],
    reactions: &[f64],
    policy_ref: &str,
    solver_mode: PreviewSolverMode,
) {
    for reaction in friction_normal_reactions {
        let suffix = stable_suffix(&reaction.support_id);
        append_nonlinear_scalar_result(
            results,
            &format!("result:nonlinear-support:{suffix}:friction-normal-reaction"),
            "nonlinear_support_friction_normal_reaction_input",
            reaction.normal_reaction,
            "N",
            &reaction.support_id,
            "friction_normal_reaction_input",
            "normal",
            &format!(
                "{}_active_set_loop; policy_ref={policy_ref}; explicit_user_entered_normal_reaction; no_catalog_or_default_normal_force",
                solver_mode.as_str()
            ),
            "positive value is an explicit contact normal reaction supplied by the user or caller",
        );
    }
    for source in derived_friction_normal_reactions {
        let suffix = stable_suffix(&source.support_id);
        let global = source.source_node_index * DOF_PER_NODE + dof_index(source.source_dof);
        let Some(reaction) = reactions.get(global) else {
            continue;
        };
        append_nonlinear_scalar_result(
            results,
            &format!("result:nonlinear-support:{suffix}:friction-normal-reaction"),
            "nonlinear_support_friction_normal_reaction_derived",
            reaction.abs(),
            "N",
            &source.support_id,
            "friction_normal_reaction_derived",
            source.source_dof.as_str(),
            &format!(
                "{}_active_set_loop; policy_ref={policy_ref}; derived_support_reaction; source_ref={}; source_dof={}",
                solver_mode.as_str(),
                source.source_ref,
                source.source_dof.as_str()
            ),
            "positive value is the absolute support reaction at the named normal-source DOF",
        );
    }
}

fn is_combination_excluded_result_kind(kind: &str) -> bool {
    matches!(
        kind,
        "nonlinear_support_friction_normal_reaction_input"
            | "nonlinear_support_friction_normal_reaction_derived"
            | "nonlinear_support_observed_max_translation_delta"
            | "nonlinear_support_observed_max_rotation_delta"
            | "nonlinear_support_observed_max_force_reaction_delta"
            | "nonlinear_support_observed_max_moment_reaction_delta"
            | "nonlinear_support_observed_free_dof_force_residual"
            | "nonlinear_support_observed_free_dof_moment_residual"
            | "nonlinear_support_free_dof_work_residual"
            | "linear_solver_mode_basis"
            | "sparse_live_path_dense_parity_relative_delta"
            | "modulus_basis_record"
            | "combination_modulus_basis_record"
    )
}

fn combination_source_identity_matches(reference: &ResultItem, candidate: &ResultItem) -> bool {
    reference.kind == candidate.kind
        && reference.unit == candidate.unit
        && reference.entity_ref == candidate.entity_ref
        && reference.metadata == candidate.metadata
}

fn algebra_dimension(result: &ResultItem) -> Option<LoadDimension> {
    match result.unit.as_str() {
        "mm" => Some(LoadDimension::Displacement),
        "rad" => Some(LoadDimension::Rotation),
        "N" => Some(LoadDimension::Force),
        "N*m" => Some(LoadDimension::Moment),
        "MPa" => Some(LoadDimension::Pressure),
        _ => None,
    }
}

fn algebra_finding_diagnostic_code(code: FindingCode) -> &'static str {
    match code {
        FindingCode::MissingOperand => "LOAD_COMBINATION_SOURCE_RESULT_MISSING",
        FindingCode::EmptyExpression => "LOAD_COMBINATION_TERMS_EMPTY",
        FindingCode::NonFiniteFactor => "LOAD_COMBINATION_FACTOR_INVALID",
        FindingCode::DimensionMismatch => "LOAD_COMBINATION_DIMENSION_MISMATCH",
        FindingCode::DuplicateOperand => "LOAD_COMBINATION_DUPLICATE_TERM",
        FindingCode::UnsupportedExpressionShape => "LOAD_COMBINATION_UNSUPPORTED_EXPRESSION",
        FindingCode::MissingResultState => "LOAD_COMBINATION_SOURCE_RESULT_MISSING",
        FindingCode::StatusBoundaryViolation => "LOAD_COMBINATION_STATUS_BOUNDARY_VIOLATION",
    }
}

fn combination_diag(
    combination: &PreviewCombination,
    base_id: &str,
    code: &str,
    severity: &str,
    message: impl Into<String>,
    mut affected_refs: Vec<String>,
) -> Diagnostic {
    affected_refs.push(base_id.to_string());
    diag(
        &format!(
            "diagnostic:combination:{}:{}:{}",
            stable_suffix(&combination.id),
            stable_suffix(base_id),
            stable_suffix(code)
        ),
        code,
        severity,
        message,
        affected_refs,
    )
}

fn qualified_load_case_result_id(load_case_id: &str, base_id: &str) -> String {
    format!(
        "result:loadcase:{}:{}",
        stable_suffix(load_case_id),
        result_tail(base_id)
    )
}

fn qualified_combination_result_id(combination_id: &str, base_id: &str) -> String {
    format!(
        "result:combination:{}:{}",
        stable_suffix(combination_id),
        result_tail(base_id)
    )
}

fn result_tail(base_id: &str) -> &str {
    base_id.strip_prefix("result:").unwrap_or(base_id)
}

fn blocked_envelope(model: PreviewModel, diagnostics: Vec<Diagnostic>) -> MechanicsEnvelope {
    MechanicsEnvelope {
        schema_version: "0.1.0".to_string(),
        document_kind: "openpipestress.product_preview.mechanics_result".to_string(),
        run_id: "run:preview-linear-static-blocked".to_string(),
        model_ref: model.project.id,
        status: StatusEnvelope {
            mechanics: "MODEL_INCOMPLETE".to_string(),
            rule_check: "RULE_INPUTS_INCOMPLETE".to_string(),
            professional_acceptance: "NOT_PROVIDED".to_string(),
        },
        summary: Summary {
            node_count: model.nodes.len(),
            segment_count: model.pipe_segments.len(),
            support_count: model.supports.len(),
            load_case_count: model.load_cases.len(),
            component_stress_modifier_count: 0,
            component_user_stiffness_macro_element_count: 0,
            component_pressure_thrust_load_count: 0,
            spring_hanger_user_input_count: 0,
            max_displacement: None,
            max_open_formula_stress: None,
        },
        results: vec![],
        diagnostics,
        professional_boundary: professional_boundary(),
        accepted_model_state_mutated: false,
    }
}

fn solver_blocked(
    model: PreviewModel,
    mut diagnostics: Vec<Diagnostic>,
    error: FrameKernelError,
) -> MechanicsEnvelope {
    diagnostics.push(diag(
        "diagnostic:physics:solver",
        "SOLVER_SYSTEM_BLOCKED",
        "blocking",
        error.to_string(),
        vec!["model".to_string()],
    ));
    blocked_envelope(model, diagnostics)
}

fn pressure_for_pipe(
    model: &PreviewModel,
    load_case: &PreviewLoadCase,
    pipe_index: usize,
    pipe_id: &str,
) -> Option<f64> {
    let mut pressure = 0.0;
    let mut found = false;
    for load in load_case.primitive_loads.iter() {
        match &load.target {
            LoadTargetInput::Element { pipe }
                if pipe == pipe_id && load.dimension == "pressure" =>
            {
                pressure += load.magnitude.value;
                found = true;
            }
            LoadTargetInput::Element { pipe }
                if model.pipe_segments.get(pipe_index).map(|p| &p.id) == Some(pipe)
                    && load.dimension == "pressure" =>
            {
                pressure += load.magnitude.value;
                found = true;
            }
            _ => {}
        }
    }
    found.then_some(pressure)
}

fn displacement_magnitude(displacements: &[f64], node_index: usize) -> f64 {
    let base = node_index * DOF_PER_NODE;
    (displacements[base + UX].powi(2)
        + displacements[base + UY].powi(2)
        + displacements[base + UZ].powi(2))
    .sqrt()
}

fn support_restraint_summary(restrained_dofs: &[usize]) -> (String, String) {
    let restrained = restrained_dofs
        .iter()
        .map(|global| global % DOF_PER_NODE)
        .collect::<HashSet<_>>();
    let all = [
        (UX, "UX"),
        (UY, "UY"),
        (UZ, "UZ"),
        (RX, "RX"),
        (RY, "RY"),
        (RZ, "RZ"),
    ];
    let present = all
        .iter()
        .filter_map(|(index, name)| restrained.contains(index).then_some(*name))
        .collect::<Vec<_>>();
    let missing = all
        .iter()
        .filter_map(|(index, name)| (!restrained.contains(index)).then_some(*name))
        .collect::<Vec<_>>();
    (join_dofs(&present), join_dofs(&missing))
}

fn support_contribution_summary(model: &PreviewModel) -> String {
    let contributions = model
        .supports
        .iter()
        .map(|support| {
            let mut dofs = support
                .restraints
                .iter()
                .filter_map(|value| parse_dof(value).ok())
                .map(dof_name)
                .collect::<Vec<_>>();
            dofs.sort_unstable();
            dofs.dedup();
            format!("{}@{}={}", support.id, support.node, join_dofs(&dofs))
        })
        .collect::<Vec<_>>();
    if contributions.is_empty() {
        "none".to_string()
    } else {
        contributions.join(";")
    }
}

fn dof_name(dof: FrameDof) -> &'static str {
    match dof {
        FrameDof::Ux => "UX",
        FrameDof::Uy => "UY",
        FrameDof::Uz => "UZ",
        FrameDof::Rx => "RX",
        FrameDof::Ry => "RY",
        FrameDof::Rz => "RZ",
    }
}

fn join_dofs(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn multiply_matrix_vector(matrix: &[Vec<f64>], vector: &[f64]) -> Vec<f64> {
    matrix
        .iter()
        .map(|row| row.iter().zip(vector).map(|(a, b)| a * b).sum())
        .collect()
}

fn max_abs_delta(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .fold(0.0, f64::max)
}

fn max_abs_value(values: &[f64]) -> f64 {
    values.iter().copied().map(f64::abs).fold(0.0, f64::max)
}

fn node_index(model: &PreviewModel, id: &str) -> Option<usize> {
    model.nodes.iter().position(|node| node.id == id)
}

fn parse_dof(value: &str) -> Result<FrameDof, String> {
    match value {
        "UX" | "Ux" | "ux" => Ok(FrameDof::Ux),
        "UY" | "Uy" | "uy" => Ok(FrameDof::Uy),
        "UZ" | "Uz" | "uz" => Ok(FrameDof::Uz),
        "RX" | "Rx" | "rx" => Ok(FrameDof::Rx),
        "RY" | "Ry" | "ry" => Ok(FrameDof::Ry),
        "RZ" | "Rz" | "rz" => Ok(FrameDof::Rz),
        _ => Err(format!("unsupported frame DOF {value}")),
    }
}

fn parse_active_set_state(value: &str) -> Result<ActiveSetState, String> {
    match value {
        "active" | "ACTIVE" => Ok(ActiveSetState::Active),
        "inactive" | "INACTIVE" => Ok(ActiveSetState::Inactive),
        "sticking" | "STICKING" => Ok(ActiveSetState::Sticking),
        "sliding" | "SLIDING" => Ok(ActiveSetState::Sliding),
        _ => Err(format!(
            "unsupported nonlinear support initial state {value}"
        )),
    }
}

fn parse_activation_sense(value: &str) -> Result<ActivationSense, String> {
    match value {
        "positive_reaction" | "positive" | "POSITIVE_REACTION" | "POSITIVE" => {
            Ok(ActivationSense::PositiveReaction)
        }
        "negative_reaction" | "negative" | "NEGATIVE_REACTION" | "NEGATIVE" => {
            Ok(ActivationSense::NegativeReaction)
        }
        _ => Err(format!(
            "unsupported nonlinear support activation sense {value}"
        )),
    }
}

fn parse_gap_direction(value: &str) -> Result<GapDirection, String> {
    match value {
        "positive_displacement" | "positive" | "POSITIVE_DISPLACEMENT" | "POSITIVE" => {
            Ok(GapDirection::PositiveDisplacement)
        }
        "negative_displacement" | "negative" | "NEGATIVE_DISPLACEMENT" | "NEGATIVE" => {
            Ok(GapDirection::NegativeDisplacement)
        }
        _ => Err(format!("unsupported nonlinear gap direction {value}")),
    }
}

fn parse_direction(value: &str) -> Result<LoadDirection, String> {
    match value {
        "global_x" | "GLOBAL_X" => Ok(LoadDirection::GlobalX),
        "global_y" | "GLOBAL_Y" => Ok(LoadDirection::GlobalY),
        "global_z" | "GLOBAL_Z" => Ok(LoadDirection::GlobalZ),
        _ => parse_dof(value).map(LoadDirection::Dof),
    }
}

fn parse_category(value: &str) -> Result<PrimitiveLoadCategory, String> {
    match value {
        "weight" => Ok(PrimitiveLoadCategory::Weight),
        "distributed_force" => Ok(PrimitiveLoadCategory::Weight),
        "pressure" => Ok(PrimitiveLoadCategory::Pressure),
        "thermal" => Ok(PrimitiveLoadCategory::Thermal),
        "hydrotest" => Ok(PrimitiveLoadCategory::Hydrotest),
        "wind" => Ok(PrimitiveLoadCategory::Wind),
        "seismic" => Ok(PrimitiveLoadCategory::Seismic),
        "occasional" => Ok(PrimitiveLoadCategory::Occasional),
        "concentrated_force" | "concentrated_moment" => Ok(PrimitiveLoadCategory::Occasional),
        _ => Err(format!("unsupported load category {value}")),
    }
}

fn authored_category_preview_mapping(value: &str) -> Option<&'static str> {
    match value {
        "concentrated_force" | "concentrated_moment" => Some("occasional"),
        "distributed_force" => Some("weight"),
        _ => None,
    }
}

fn parse_load_dimension(value: &str) -> Result<LoadDimension, String> {
    match value {
        "force" => Ok(LoadDimension::Force),
        "moment" => Ok(LoadDimension::Moment),
        "force_per_length" => Ok(LoadDimension::ForcePerLength),
        "pressure" => Ok(LoadDimension::Pressure),
        "temperature_change" | "temperature_interval" => Ok(LoadDimension::TemperatureChange),
        "acceleration" => Ok(LoadDimension::Acceleration),
        "displacement" => Ok(LoadDimension::Displacement),
        "rotation" => Ok(LoadDimension::Rotation),
        _ => Err(format!("unsupported load dimension {value}")),
    }
}

pub(crate) fn expected_load_dimension(value: &str) -> Option<Dimension> {
    match value {
        "force" => Some(Dimension::Force),
        "moment" => Some(Dimension::Moment),
        "force_per_length" => Some(Dimension::ForcePerLength),
        "pressure" => Some(Dimension::Pressure),
        "temperature_change" | "temperature_interval" => Some(Dimension::TemperatureInterval),
        "acceleration" => Some(Dimension::Acceleration),
        "displacement" => Some(Dimension::Displacement),
        "rotation" => Some(Dimension::Rotation),
        _ => None,
    }
}

pub(crate) fn unit_symbol_matches_dimension(
    unit_symbol: &str,
    dimension: Dimension,
) -> Result<(), String> {
    unit_by_symbol(unit_symbol, dimension)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

pub(crate) fn canonical_unit_symbol(dimension: Dimension) -> Option<&'static str> {
    canonical_unit(dimension).map(|unit| unit.definition().symbol)
}

fn is_temperature_change_dimension(value: &str) -> bool {
    matches!(value, "temperature_change" | "temperature_interval")
}

fn dof_index(dof: FrameDof) -> usize {
    match dof {
        FrameDof::Ux => UX,
        FrameDof::Uy => UY,
        FrameDof::Uz => UZ,
        FrameDof::Rx => RX,
        FrameDof::Ry => RY,
        FrameDof::Rz => RZ,
    }
}

fn has_blocking(diagnostics: &[Diagnostic]) -> bool {
    diagnostics.iter().any(|d| d.severity == "blocking")
}

fn professional_boundary() -> ProfessionalBoundary {
    ProfessionalBoundary {
        human_review_required: true,
        software_makes_compliance_claim: false,
        software_makes_certification_claim: false,
        software_makes_sealing_claim: false,
        software_makes_approval_claim: false,
    }
}

fn diag(
    id: &str,
    code: &str,
    severity: &str,
    message: impl Into<String>,
    affected_refs: Vec<String>,
) -> Diagnostic {
    Diagnostic {
        id: id.to_string(),
        code: code.to_string(),
        severity: severity.to_string(),
        message: message.into(),
        source: Some("core/product_physics".to_string()),
        affected_refs,
    }
}

fn missing_nonlinear_field_diag(support_id: &str, field: &str) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:nonlinear-support:{}:{}",
            stable_suffix(support_id),
            stable_suffix(field)
        ),
        "NONLINEAR_SUPPORT_INPUT_MISSING",
        "blocking",
        format!("nonlinear support requires explicit {field}"),
        vec![support_id.to_string(), field.to_string()],
    )
}

fn invalid_nonlinear_field_diag(
    support_id: &str,
    field: &str,
    value: &str,
    message: String,
) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:nonlinear-support:{}:{}",
            stable_suffix(support_id),
            stable_suffix(field)
        ),
        "NONLINEAR_SUPPORT_INPUT_INVALID",
        "blocking",
        message,
        vec![support_id.to_string(), field.to_string(), value.to_string()],
    )
}

fn stable_suffix(id: &str) -> String {
    id.replace(':', "-")
}

fn round6(value: f64) -> f64 {
    let rounded = (value * 1_000_000.0).round() / 1_000_000.0;
    if rounded == 0.0 {
        0.0
    } else {
        rounded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> LinearStaticPreviewRequest {
        serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/invented_preview_model.json"
        ))
        .map(|model| LinearStaticPreviewRequest {
            model,
            materials: invented_materials(),
        })
        .unwrap()
    }

    fn invented_materials() -> Vec<MaterialInput> {
        vec![MaterialInput {
            id: "material:invented-carbon-steel".to_string(),
            elastic_modulus: Quantity {
                value: 200_000_000_000.0,
                unit: "Pa".to_string(),
            },
            shear_modulus: Quantity {
                value: 77_000_000_000.0,
                unit: "Pa".to_string(),
            },
            thermal_expansion_coefficient: Some(Quantity {
                value: 0.000012,
                unit: "1/degC".to_string(),
            }),
            temperature_points: Vec::new(),
            provenance: Some("invented_example_no_material_standard".to_string()),
        }]
    }

    fn find_result<'a>(envelope: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
        envelope["results"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["id"] == id)
            .unwrap_or_else(|| panic!("missing result {id}"))
    }

    fn result_value(envelope: &MechanicsEnvelope, id: &str) -> f64 {
        envelope
            .results
            .iter()
            .find(|item| item.id == id)
            .unwrap_or_else(|| panic!("missing result {id}"))
            .value
    }

    fn fixed_fixed_thermal_request(direction: &str) -> LinearStaticPreviewRequest {
        let mut request = request();
        request.model.nodes.truncate(2);
        request.model.nodes[0].id = "node:N-100".to_string();
        request.model.nodes[0].position = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        request.model.nodes[1].id = "node:N-110".to_string();
        request.model.nodes[1].position = Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };
        request.model.pipe_segments.truncate(1);
        request.model.pipe_segments[0].id = "pipe:P-100".to_string();
        request.model.pipe_segments[0].from = "node:N-100".to_string();
        request.model.pipe_segments[0].to = "node:N-110".to_string();
        request.model.pipe_segments[0].y_reference = Some(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });
        request.model.supports.truncate(2);
        request.model.supports[0].id = "support:S-100".to_string();
        request.model.supports[0].node = "node:N-100".to_string();
        request.model.supports[0].restraints = vec![
            "UX".to_string(),
            "UY".to_string(),
            "UZ".to_string(),
            "RX".to_string(),
            "RY".to_string(),
            "RZ".to_string(),
        ];
        request.model.supports[1].id = "support:S-110".to_string();
        request.model.supports[1].node = "node:N-110".to_string();
        request.model.supports[1].restraints = vec!["UX".to_string()];
        request.model.load_cases.truncate(1);
        request.model.combinations.clear();
        request.model.load_cases[0].primitive_loads = vec![PreviewPrimitiveLoad {
            id: "load:L-THERMAL".to_string(),
            category: "thermal".to_string(),
            target: LoadTargetInput::Element {
                pipe: "pipe:P-100".to_string(),
            },
            direction: direction.to_string(),
            magnitude: Quantity {
                value: 10.0,
                unit: "degC".to_string(),
            },
            dimension: "temperature_change".to_string(),
            provenance: Some("invented_example_user_input".to_string()),
        }];
        request
    }

    fn fixed_fixed_pressure_request(direction: &str) -> LinearStaticPreviewRequest {
        let mut request = request();
        request.model.nodes.truncate(2);
        request.model.nodes[0].id = "node:N-100".to_string();
        request.model.nodes[0].position = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        request.model.nodes[1].id = "node:N-110".to_string();
        request.model.nodes[1].position = Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };
        request.model.pipe_segments.truncate(1);
        request.model.pipe_segments[0].id = "pipe:P-100".to_string();
        request.model.pipe_segments[0].from = "node:N-100".to_string();
        request.model.pipe_segments[0].to = "node:N-110".to_string();
        request.model.pipe_segments[0].y_reference = Some(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });
        request.model.supports.truncate(2);
        request.model.supports[0].id = "support:S-100".to_string();
        request.model.supports[0].node = "node:N-100".to_string();
        request.model.supports[0].restraints = vec![
            "UX".to_string(),
            "UY".to_string(),
            "UZ".to_string(),
            "RX".to_string(),
            "RY".to_string(),
            "RZ".to_string(),
        ];
        request.model.supports[1].id = "support:S-110".to_string();
        request.model.supports[1].node = "node:N-110".to_string();
        request.model.supports[1].restraints = vec!["UX".to_string()];
        request.model.load_cases.truncate(1);
        request.model.combinations.clear();
        request.model.load_cases[0].primitive_loads = vec![PreviewPrimitiveLoad {
            id: "load:L-PRESSURE".to_string(),
            category: "pressure".to_string(),
            target: LoadTargetInput::Element {
                pipe: "pipe:P-100".to_string(),
            },
            direction: direction.to_string(),
            magnitude: Quantity {
                value: 1_000_000.0,
                unit: "Pa".to_string(),
            },
            dimension: "pressure".to_string(),
            provenance: Some("invented_example_user_input".to_string()),
        }];
        request
    }

    #[test]
    fn valid_invented_model_solves_deterministically() {
        let result = run_linear_static_preview(request());
        let result_ids = result
            .results
            .iter()
            .map(|item| item.id.as_str())
            .collect::<HashSet<_>>();

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(!result.results.is_empty());
        assert!(result.summary.max_displacement.as_ref().unwrap().value > 0.0);
        assert!(result
            .results
            .iter()
            .any(|item| item.id == "result:disp:node-N-140"));
        assert!(result_ids.contains("result:solver-mode:linear-solve-basis"));
        assert!(result_ids.contains("result:loadcase:load-L-200:solver-mode:linear-solve-basis"));
        assert!(!result_ids
            .contains("result:combination:combination-C-OPER-ALT:solver-mode:linear-solve-basis"));
        let solver_mode_evidence = result
            .results
            .iter()
            .find(|item| item.id == "result:solver-mode:linear-solve-basis")
            .expect("default load case solver-mode evidence row is present");
        assert_eq!(solver_mode_evidence.kind, "linear_solver_mode_basis");
        assert_eq!(solver_mode_evidence.value, 1.0);
        let metadata = solver_mode_evidence.metadata.as_ref().unwrap();
        assert_eq!(metadata.component, "linear_solver_mode");
        assert_eq!(metadata.coordinate_system, "reduced_system");
        assert!(metadata.basis.contains("DEC-053 sparse_default_promotion"));
        assert!(metadata.basis.contains("solver_mode=sparse_interactive"));
        assert!(metadata
            .basis
            .contains("solution_basis=sparse_profile_direct_primary"));
        assert!(metadata
            .basis
            .contains("default_sparse_promotion=interactive_default"));
        assert!(metadata.basis.contains("dense_scrutiny_available=true"));
    }

    #[test]
    fn dense_scrutiny_mode_keeps_sparse_parity_row() {
        let result =
            run_linear_static_preview_with_mode(request(), PreviewSolverMode::DenseScrutiny);
        let sparse_evidence = result
            .results
            .iter()
            .find(|item| item.id == "result:sparse-live:dense-parity-relative-delta")
            .expect("dense scrutiny sparse parity row is present");
        let mode_evidence = result
            .results
            .iter()
            .find(|item| item.id == "result:solver-mode:linear-solve-basis")
            .expect("dense scrutiny solver-mode row is present");

        assert_eq!(mode_evidence.value, 2.0);
        assert!(mode_evidence
            .metadata
            .as_ref()
            .unwrap()
            .basis
            .contains("solver_mode=dense_scrutiny"));
        assert_eq!(
            sparse_evidence.kind,
            "sparse_live_path_dense_parity_relative_delta"
        );
        assert!(sparse_evidence.value <= 1.0e-9);
        let metadata = sparse_evidence.metadata.as_ref().unwrap();
        assert_eq!(metadata.component, "sparse_live_path");
        assert!(metadata
            .basis
            .contains("DEC-053 dense_scrutiny_sparse_parity"));
        assert!(metadata.basis.contains("solver_mode=dense_scrutiny"));
        assert!(metadata.basis.contains("sparse_interactive_default=true"));
    }

    #[test]
    fn valid_invented_model_exposes_nonlinear_support_loop_evidence() {
        let result = run_linear_static_preview(request());
        let result_ids = result
            .results
            .iter()
            .map(|item| item.id.as_str())
            .collect::<HashSet<_>>();
        let diagnostic_codes = result
            .diagnostics
            .iter()
            .map(|item| item.code.as_str())
            .collect::<HashSet<_>>();

        assert!(result_ids.contains("result:nonlinear-support:iteration-count"));
        assert!(result_ids.contains("result:nonlinear-support:final-residual-count"));
        assert!(result_ids.contains("result:nonlinear-support:converged-flag"));
        assert!(result_ids.contains("result:nonlinear-support:support-NL-140:state-code"));
        assert!(result_ids.contains("result:nonlinear-support:support-NL-140:uy-displacement"));
        assert!(result_ids.contains("result:nonlinear-support:support-NL-140:uy-reaction"));
        assert!(result_ids.contains("result:nonlinear-support:support-NL-130-FRIC:state-code"));
        assert!(result_ids.contains("result:nonlinear-support:support-NL-130-FRIC:uz-displacement"));
        assert!(result_ids.contains("result:nonlinear-support:support-NL-130-FRIC:uz-reaction"));
        assert!(result_ids
            .contains("result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction"));
        assert!(result_ids
            .contains("result:loadcase:load-L-200:nonlinear-support:support-NL-140:uy-reaction"));
        assert!(result_ids.contains(
            "result:combination:combination-C-OPER-ALT:nonlinear-support:support-NL-140:uy-reaction"
        ));
        // DEC-067: the sliding-seeded friction support defers convergence one
        // iteration so the bounded sliding force is applied before the loop
        // converges.
        assert_eq!(
            result_value(&result, "result:nonlinear-support:iteration-count"),
            2.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:final-residual-count"),
            0.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:converged-flag"),
            1.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-140:state-code"
            ),
            1.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-140:uy-displacement"
            ),
            0.0
        );
        assert!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-140:uy-reaction"
            ) < 0.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-130-FRIC:state-code"
            ),
            3.0
        );
        assert_ne!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-130-FRIC:uz-displacement"
            ),
            0.0
        );
        // DEC-067: the sliding support carries the bounded +mu*N tangential
        // reaction opposing its negative-Z motion instead of a released zero
        // reaction.
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-130-FRIC:uz-reaction"
            ),
            0.490101
        );
        let normal_evidence = result
            .results
            .iter()
            .find(|item| {
                item.id == "result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction"
            })
            .expect("derived normal evidence row is present");
        assert_eq!(
            normal_evidence.kind,
            "nonlinear_support_friction_normal_reaction_derived"
        );
        assert_eq!(normal_evidence.value, 48.952652);
        let normal_metadata = normal_evidence.metadata.as_ref().unwrap();
        assert!(normal_metadata.basis.contains("derived_support_reaction"));
        assert!(normal_metadata.basis.contains("source_ref=support:S-130"));
        assert!(normal_metadata.basis.contains("source_dof=uy"));
        assert!(!normal_metadata
            .basis
            .contains("derived_normal_force_model=TBD"));
        assert!(!diagnostic_codes.contains("TOLERANCE_POLICY_TBD"));
        assert!(diagnostic_codes.contains("NONLINEAR_SUPPORT_STATE_REVIEW"));
        assert!(diagnostic_codes.contains("NONLINEAR_SUPPORT_LOOP_CONVERGED"));
        assert!(!diagnostic_codes.contains("NONLINEAR_SUPPORT_LOOP_BLOCKED"));
    }

    fn two_node_nonlinear_preview_request(
        support_id: &str,
        nonlinear: NonlinearSupportInput,
        load_id: &str,
        load_value: f64,
        combination_id: &str,
    ) -> LinearStaticPreviewRequest {
        let mut request = request();
        request.model.nodes.truncate(2);
        request.model.nodes[0].id = "node:N-100".to_string();
        request.model.nodes[0].position = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        request.model.nodes[1].id = "node:N-110".to_string();
        request.model.nodes[1].position = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        request.model.pipe_segments.truncate(1);
        request.model.pipe_segments[0].id = "pipe:P-100".to_string();
        request.model.pipe_segments[0].from = "node:N-100".to_string();
        request.model.pipe_segments[0].to = "node:N-110".to_string();
        request.model.pipe_segments[0].y_reference = Some(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });
        request.model.supports = vec![
            PreviewSupport {
                id: "support:S-100".to_string(),
                node: "node:N-100".to_string(),
                restraints: vec![
                    "UX".to_string(),
                    "UY".to_string(),
                    "UZ".to_string(),
                    "RX".to_string(),
                    "RY".to_string(),
                    "RZ".to_string(),
                ],
                family: Some("anchor".to_string()),
                stiffness: None,
                hanger: None,
                nonlinear: None,
                provenance: Some("invented_example".to_string()),
            },
            PreviewSupport {
                id: support_id.to_string(),
                node: "node:N-110".to_string(),
                restraints: Vec::new(),
                family: Some("nonlinear".to_string()),
                stiffness: None,
                hanger: None,
                nonlinear: Some(nonlinear),
                provenance: Some(
                    "invented_example_user_entered_nonlinear_support_no_catalog".to_string(),
                ),
            },
        ];
        request.model.components.clear();
        request.model.load_cases.truncate(1);
        request.model.load_cases[0].id = load_id.to_string();
        request.model.load_cases[0].primitive_loads = vec![PreviewPrimitiveLoad {
            id: format!("{load_id}-X"),
            category: "occasional".to_string(),
            target: LoadTargetInput::Node {
                node: "node:N-110".to_string(),
            },
            direction: "global_x".to_string(),
            magnitude: Quantity {
                value: load_value,
                unit: "N".to_string(),
            },
            dimension: "force".to_string(),
            provenance: Some("invented_example_user_input".to_string()),
        }];
        request.model.combinations = vec![PreviewCombination {
            id: combination_id.to_string(),
            label: None,
            basis: "mechanics".to_string(),
            terms: vec![PreviewCombinationTerm {
                load_case: load_id.to_string(),
                factor: 1.0,
            }],
            minuend_id: None,
            subtrahend_id: None,
            operand_ids: None,
            mode: None,
            provenance: Some("invented_example_no_code_combination".to_string()),
        }];
        request
    }

    fn friction_preview_request() -> LinearStaticPreviewRequest {
        two_node_nonlinear_preview_request(
            "support:NL-FRIC-110",
            NonlinearSupportInput {
                behavior: "friction".to_string(),
                dof: "UX".to_string(),
                initial_state: Some("sticking".to_string()),
                active_when: None,
                contact_when: None,
                closes_when: None,
                gap: None,
                friction_coefficient: Some(Quantity {
                    value: 0.50,
                    unit: "none".to_string(),
                }),
                normal_reaction: Some(Quantity {
                    value: 1000.0,
                    unit: "N".to_string(),
                }),
                normal_reaction_source: None,
            },
            "load:L-FRICTION",
            100.0,
            "combination:C-FRICTION",
        )
    }

    fn mixed_nonlinear_preview_request() -> LinearStaticPreviewRequest {
        let mut request = two_node_nonlinear_preview_request(
            "support:NL-MIX-ONEWAY-110",
            NonlinearSupportInput {
                behavior: "one_way".to_string(),
                dof: "UX".to_string(),
                initial_state: Some("active".to_string()),
                active_when: Some("positive_reaction".to_string()),
                contact_when: None,
                closes_when: None,
                gap: None,
                friction_coefficient: None,
                normal_reaction: None,
                normal_reaction_source: None,
            },
            "load:L-MIXED-NONLINEAR",
            100.0,
            "combination:C-MIXED-NONLINEAR",
        );
        request.model.supports.push(PreviewSupport {
            id: "support:NL-MIX-GAP-110".to_string(),
            node: "node:N-110".to_string(),
            restraints: Vec::new(),
            family: Some("nonlinear".to_string()),
            stiffness: None,
            hanger: None,
            nonlinear: Some(NonlinearSupportInput {
                behavior: "gap".to_string(),
                dof: "UY".to_string(),
                initial_state: Some("inactive".to_string()),
                active_when: None,
                contact_when: None,
                closes_when: Some("positive_displacement".to_string()),
                gap: Some(Quantity {
                    value: 0.05,
                    unit: "mm".to_string(),
                }),
                friction_coefficient: None,
                normal_reaction: None,
                normal_reaction_source: None,
            }),
            provenance: Some("invented_example_user_entered_nonlinear_gap_no_catalog".to_string()),
        });
        request.model.supports.push(PreviewSupport {
            id: "support:NL-MIX-FRIC-110".to_string(),
            node: "node:N-110".to_string(),
            restraints: Vec::new(),
            family: Some("nonlinear".to_string()),
            stiffness: None,
            hanger: None,
            nonlinear: Some(NonlinearSupportInput {
                behavior: "friction".to_string(),
                dof: "UZ".to_string(),
                initial_state: Some("sticking".to_string()),
                active_when: None,
                contact_when: None,
                closes_when: None,
                gap: None,
                friction_coefficient: Some(Quantity {
                    value: 0.30,
                    unit: "none".to_string(),
                }),
                normal_reaction: Some(Quantity {
                    value: 10.0,
                    unit: "N".to_string(),
                }),
                normal_reaction_source: None,
            }),
            provenance: Some(
                "invented_example_user_entered_nonlinear_friction_no_catalog".to_string(),
            ),
        });
        request.model.load_cases[0]
            .primitive_loads
            .push(PreviewPrimitiveLoad {
                id: "load:L-MIXED-NONLINEAR-Y".to_string(),
                category: "occasional".to_string(),
                target: LoadTargetInput::Node {
                    node: "node:N-110".to_string(),
                },
                direction: "global_y".to_string(),
                magnitude: Quantity {
                    value: 100_000.0,
                    unit: "N".to_string(),
                },
                dimension: "force".to_string(),
                provenance: Some("invented_example_user_input".to_string()),
            });
        request.model.load_cases[0]
            .primitive_loads
            .push(PreviewPrimitiveLoad {
                id: "load:L-MIXED-NONLINEAR-Z".to_string(),
                category: "occasional".to_string(),
                target: LoadTargetInput::Node {
                    node: "node:N-110".to_string(),
                },
                direction: "global_z".to_string(),
                magnitude: Quantity {
                    value: 100.0,
                    unit: "N".to_string(),
                },
                dimension: "force".to_string(),
                provenance: Some("invented_example_user_input".to_string()),
            });
        request
    }

    #[test]
    fn mixed_nonlinear_preview_bundle_converges_and_emits_each_support_state() {
        let result = run_linear_static_preview(mixed_nonlinear_preview_request());
        let result_ids = result
            .results
            .iter()
            .map(|item| item.id.as_str())
            .collect::<HashSet<_>>();
        let diagnostic_codes = result
            .diagnostics
            .iter()
            .map(|item| item.code.as_str())
            .collect::<HashSet<_>>();

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(
            result_ids.contains("result:nonlinear-support:support-NL-MIX-ONEWAY-110:state-code")
        );
        assert!(result_ids.contains("result:nonlinear-support:support-NL-MIX-GAP-110:state-code"));
        assert!(result_ids.contains("result:nonlinear-support:support-NL-MIX-FRIC-110:state-code"));
        assert!(result_ids
            .contains("result:nonlinear-support:support-NL-MIX-FRIC-110:friction-normal-reaction"));
        assert!(result_ids.contains("result:nonlinear-support:max-translation-delta"));
        assert!(result_ids.contains("result:nonlinear-support:max-rotation-delta"));
        assert!(result_ids.contains("result:nonlinear-support:max-force-reaction-delta"));
        assert!(result_ids.contains("result:nonlinear-support:max-moment-reaction-delta"));
        assert!(result_ids.contains("result:nonlinear-support:free-dof-force-residual"));
        assert!(result_ids.contains("result:nonlinear-support:free-dof-moment-residual"));
        assert!(!result_ids.contains(
            "result:combination:combination-C-MIXED-NONLINEAR:nonlinear-support:support-NL-MIX-FRIC-110:friction-normal-reaction"
        ));
        assert!(!result_ids.contains(
            "result:combination:combination-C-MIXED-NONLINEAR:nonlinear-support:max-translation-delta"
        ));
        assert_eq!(
            result_value(&result, "result:nonlinear-support:iteration-count"),
            2.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:final-residual-count"),
            0.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:converged-flag"),
            1.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-MIX-ONEWAY-110:state-code"
            ),
            0.0
        );
        assert!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-MIX-ONEWAY-110:ux-displacement"
            ) > 0.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-MIX-ONEWAY-110:ux-reaction"
            ),
            0.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-MIX-GAP-110:state-code"
            ),
            1.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-MIX-GAP-110:uy-displacement"
            ),
            0.05
        );
        assert!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-MIX-GAP-110:uy-reaction"
            ) < 0.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-MIX-FRIC-110:state-code"
            ),
            3.0
        );
        assert!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-MIX-FRIC-110:uz-displacement"
            ) > 0.0
        );
        // DEC-067: the sliding support carries the bounded -mu*N tangential
        // reaction opposing motion (0.3 * 10 N normal evidence), not a full
        // DOF release with zero reaction.
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-MIX-FRIC-110:uz-reaction"
            ),
            -3.0
        );
        let max_translation_delta =
            result_value(&result, "result:nonlinear-support:max-translation-delta");
        let max_rotation_delta =
            result_value(&result, "result:nonlinear-support:max-rotation-delta");
        let max_force_reaction_delta =
            result_value(&result, "result:nonlinear-support:max-force-reaction-delta");
        let max_moment_reaction_delta = result_value(
            &result,
            "result:nonlinear-support:max-moment-reaction-delta",
        );
        assert!(max_translation_delta > 0.0);
        assert!(
            max_translation_delta <= DEC_046_PRODUCT_PREVIEW_TRANSLATION_DELTA_ABSOLUTE_LIMIT_MM,
            "max translation delta {max_translation_delta}"
        );
        assert!(max_rotation_delta >= 0.0);
        assert!(
            max_rotation_delta <= DEC_046_PRODUCT_PREVIEW_ROTATION_DELTA_ABSOLUTE_LIMIT_RAD,
            "max rotation delta {max_rotation_delta}"
        );
        assert!(max_force_reaction_delta > 0.0);
        assert!(
            max_force_reaction_delta
                <= DEC_046_PRODUCT_PREVIEW_FORCE_REACTION_DELTA_ABSOLUTE_LIMIT_N,
            "max force reaction delta {max_force_reaction_delta}"
        );
        assert!(max_moment_reaction_delta >= 0.0);
        assert!(
            max_moment_reaction_delta
                <= DEC_046_PRODUCT_PREVIEW_MOMENT_REACTION_DELTA_ABSOLUTE_LIMIT_N_M,
            "max moment reaction delta {max_moment_reaction_delta}"
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:free-dof-force-residual"),
            0.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:free-dof-moment-residual"),
            0.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:free-dof-work-residual"),
            0.0
        );
        let translation_delta = result
            .results
            .iter()
            .find(|item| item.id == "result:nonlinear-support:max-translation-delta")
            .expect("translation delta row exists");
        let translation_delta_basis = &translation_delta.metadata.as_ref().unwrap().basis;
        assert!(translation_delta_basis
            .contains(DEC_046_PRODUCT_PREVIEW_DISPLACEMENT_REACTION_DELTA_OBSERVATION_REF));
        assert!(translation_delta_basis
            .contains(DEC_046_PRODUCT_PREVIEW_DISPLACEMENT_REACTION_DELTA_POLICY_REF));
        assert!(translation_delta_basis.contains("threshold_policy_status=accepted"));
        assert!(translation_delta_basis.contains("residual_basis=displacement_reaction_delta"));
        assert!(translation_delta_basis.contains("translation_delta_threshold=50 mm"));
        assert!(translation_delta_basis.contains("rotation_delta_threshold=0.05 rad"));
        assert!(translation_delta_basis.contains("force_reaction_delta_threshold=110000 N"));
        assert!(translation_delta_basis.contains("moment_reaction_delta_threshold=110000 N*m"));
        assert!(!translation_delta_basis.contains("threshold_policy_status=tbd"));
        assert!(!translation_delta_basis.contains("threshold_policy_ref=TBD"));
        assert!(!translation_delta_basis.contains("observed_delta_only"));
        let force_residual = result
            .results
            .iter()
            .find(|item| item.id == "result:nonlinear-support:free-dof-force-residual")
            .expect("force residual row exists");
        let moment_residual = result
            .results
            .iter()
            .find(|item| item.id == "result:nonlinear-support:free-dof-moment-residual")
            .expect("moment residual row exists");
        let work_residual = result
            .results
            .iter()
            .find(|item| item.id == "result:nonlinear-support:free-dof-work-residual")
            .expect("work residual row exists");
        for residual in [force_residual, moment_residual] {
            let basis = &residual.metadata.as_ref().unwrap().basis;
            assert!(basis.contains(DEC_046_PRODUCT_PREVIEW_FREE_DOF_FORCE_MOMENT_POLICY_REF));
            assert!(basis.contains("threshold_policy_status=accepted"));
            assert!(basis.contains("residual_basis=free_dof_force_moment_equilibrium"));
            assert!(!basis.contains("threshold=TBD"));
        }
        let work_basis = &work_residual.metadata.as_ref().unwrap().basis;
        assert!(work_basis.contains(DEC_046_PRODUCT_PREVIEW_FREE_DOF_WORK_POLICY_REF));
        assert!(work_basis.contains(DEC_046_PRODUCT_PREVIEW_GENERAL_ENERGY_POLICY_REF));
        assert!(work_basis.contains("threshold_policy_status=accepted"));
        assert!(work_basis.contains("residual_basis=free_dof_work_residual"));
        assert!(work_basis.contains("general_energy_threshold_policy_status=accepted"));
        assert!(work_basis.contains("general_energy_threshold=0 N*m"));
        assert!(!work_basis.contains("observed_residual_only"));
        let iteration_count = result
            .results
            .iter()
            .find(|item| item.id == "result:nonlinear-support:iteration-count")
            .expect("iteration-count row exists");
        assert!(iteration_count
            .metadata
            .as_ref()
            .unwrap()
            .basis
            .contains("support_count=3"));
        assert!(iteration_count
            .metadata
            .as_ref()
            .unwrap()
            .basis
            .contains(DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_POLICY_REF));
        assert!(iteration_count
            .metadata
            .as_ref()
            .unwrap()
            .basis
            .contains("policy_status=accepted"));
        assert!(iteration_count
            .metadata
            .as_ref()
            .unwrap()
            .basis
            .contains("support_classes=friction,gap,one_way"));
        let normal_evidence = result
            .results
            .iter()
            .find(|item| {
                item.id
                    == "result:nonlinear-support:support-NL-MIX-FRIC-110:friction-normal-reaction"
            })
            .expect("normal evidence row is present");
        assert_eq!(
            normal_evidence.kind,
            "nonlinear_support_friction_normal_reaction_input"
        );
        assert_eq!(normal_evidence.value, 10.0);
        assert!(normal_evidence
            .metadata
            .as_ref()
            .unwrap()
            .basis
            .contains("explicit_user_entered_normal_reaction"));
        assert_eq!(
            result
                .diagnostics
                .iter()
                .filter(|item| item.code == "NONLINEAR_SUPPORT_STATE_REVIEW")
                .count(),
            3
        );
        assert!(!diagnostic_codes.contains("TOLERANCE_POLICY_TBD"));
        assert!(diagnostic_codes.contains("NONLINEAR_SUPPORT_LOOP_CONVERGED"));
        assert!(!diagnostic_codes.contains("NONLINEAR_SUPPORT_LOOP_BLOCKED"));
    }

    #[test]
    fn friction_preview_surfaces_explicit_normal_evidence_without_combining_it() {
        let result = run_linear_static_preview(friction_preview_request());
        let result_ids = result
            .results
            .iter()
            .map(|item| item.id.as_str())
            .collect::<HashSet<_>>();

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(result_ids
            .contains("result:nonlinear-support:support-NL-FRIC-110:friction-normal-reaction"));
        assert!(result_ids.contains("result:nonlinear-support:support-NL-FRIC-110:state-code"));
        assert!(result_ids.contains("result:nonlinear-support:support-NL-FRIC-110:ux-reaction"));
        assert!(!result_ids.contains(
            "result:combination:combination-C-FRICTION:nonlinear-support:support-NL-FRIC-110:friction-normal-reaction"
        ));
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-FRIC-110:state-code"
            ),
            2.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-FRIC-110:friction-normal-reaction"
            ),
            1000.0
        );
        let normal_evidence = result
            .results
            .iter()
            .find(|item| {
                item.id == "result:nonlinear-support:support-NL-FRIC-110:friction-normal-reaction"
            })
            .expect("normal evidence row is present");
        assert_eq!(
            normal_evidence.kind,
            "nonlinear_support_friction_normal_reaction_input"
        );
        assert!(normal_evidence
            .metadata
            .as_ref()
            .unwrap()
            .basis
            .contains("explicit_user_entered_normal_reaction"));
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "NONLINEAR_SUPPORT_LOOP_CONVERGED"));
    }

    fn friction_derived_normal_preview_request() -> LinearStaticPreviewRequest {
        let mut request = two_node_nonlinear_preview_request(
            "support:NL-FRIC-DERIVED-110",
            NonlinearSupportInput {
                behavior: "friction".to_string(),
                dof: "UX".to_string(),
                initial_state: Some("sticking".to_string()),
                active_when: None,
                contact_when: None,
                closes_when: None,
                gap: None,
                friction_coefficient: Some(Quantity {
                    value: 0.30,
                    unit: "none".to_string(),
                }),
                normal_reaction: None,
                normal_reaction_source: Some(FrictionNormalReactionSourceInput {
                    support_ref: "support:S-NORMAL-110".to_string(),
                    dof: "UY".to_string(),
                }),
            },
            "load:L-FRICTION-DERIVED",
            10.0,
            "combination:C-FRICTION-DERIVED",
        );
        request.model.supports.push(PreviewSupport {
            id: "support:S-NORMAL-110".to_string(),
            node: "node:N-110".to_string(),
            restraints: vec!["UY".to_string()],
            family: Some("guide".to_string()),
            stiffness: None,
            hanger: None,
            nonlinear: None,
            provenance: Some("invented_example_normal_reaction_source".to_string()),
        });
        request.model.load_cases[0]
            .primitive_loads
            .push(PreviewPrimitiveLoad {
                id: "load:L-FRICTION-DERIVED-Y".to_string(),
                category: "occasional".to_string(),
                target: LoadTargetInput::Node {
                    node: "node:N-110".to_string(),
                },
                direction: "global_y".to_string(),
                magnitude: Quantity {
                    value: -100.0,
                    unit: "N".to_string(),
                },
                dimension: "force".to_string(),
                provenance: Some("invented_example_user_input".to_string()),
            });
        request
    }

    #[test]
    fn friction_preview_derives_normal_from_named_support_reaction() {
        let result = run_linear_static_preview(friction_derived_normal_preview_request());
        let result_ids = result
            .results
            .iter()
            .map(|item| item.id.as_str())
            .collect::<HashSet<_>>();

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(result_ids.contains(
            "result:nonlinear-support:support-NL-FRIC-DERIVED-110:friction-normal-reaction"
        ));
        assert!(!result_ids.contains(
            "result:combination:combination-C-FRICTION-DERIVED:nonlinear-support:support-NL-FRIC-DERIVED-110:friction-normal-reaction"
        ));
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-FRIC-DERIVED-110:state-code"
            ),
            2.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-FRIC-DERIVED-110:friction-normal-reaction"
            ),
            100.0
        );
        let normal_evidence = result
            .results
            .iter()
            .find(|item| {
                item.id
                    == "result:nonlinear-support:support-NL-FRIC-DERIVED-110:friction-normal-reaction"
            })
            .expect("derived normal evidence row is present");
        assert_eq!(
            normal_evidence.kind,
            "nonlinear_support_friction_normal_reaction_derived"
        );
        let metadata = normal_evidence.metadata.as_ref().unwrap();
        assert!(metadata.basis.contains("derived_support_reaction"));
        assert!(metadata.basis.contains("source_ref=support:S-NORMAL-110"));
        assert!(metadata.basis.contains("source_dof=uy"));
        assert!(!metadata.basis.contains("derived_normal_force_model=TBD"));
    }

    fn friction_sliding_preview_request() -> LinearStaticPreviewRequest {
        two_node_nonlinear_preview_request(
            "support:NL-FRIC-SLIDE-110",
            NonlinearSupportInput {
                behavior: "friction".to_string(),
                dof: "UX".to_string(),
                initial_state: Some("sticking".to_string()),
                active_when: None,
                contact_when: None,
                closes_when: None,
                gap: None,
                friction_coefficient: Some(Quantity {
                    value: 0.30,
                    unit: "none".to_string(),
                }),
                normal_reaction: Some(Quantity {
                    value: 10.0,
                    unit: "N".to_string(),
                }),
                normal_reaction_source: None,
            },
            "load:L-FRICTION-SLIDE",
            10.0,
            "combination:C-FRICTION-SLIDE",
        )
    }

    #[test]
    fn friction_preview_slides_and_converges_with_explicit_normal_evidence() {
        let result = run_linear_static_preview(friction_sliding_preview_request());
        let diagnostic_codes = result
            .diagnostics
            .iter()
            .map(|item| item.code.as_str())
            .collect::<HashSet<_>>();

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(
            result_value(&result, "result:nonlinear-support:iteration-count"),
            2.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:final-residual-count"),
            0.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:converged-flag"),
            1.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-FRIC-SLIDE-110:state-code"
            ),
            3.0
        );
        assert!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-FRIC-SLIDE-110:ux-displacement"
            ) > 0.0
        );
        // DEC-067: the sliding support reports the bounded -mu*N tangential
        // reaction opposing motion (0.3 * 10 N explicit normal evidence)
        // instead of a fully released zero reaction.
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-FRIC-SLIDE-110:ux-reaction"
            ),
            -3.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-FRIC-SLIDE-110:friction-normal-reaction"
            ),
            10.0
        );
        assert!(!diagnostic_codes.contains("TOLERANCE_POLICY_TBD"));
        assert!(diagnostic_codes.contains("NONLINEAR_SUPPORT_STATE_REVIEW"));
        assert!(diagnostic_codes.contains("NONLINEAR_SUPPORT_LOOP_CONVERGED"));
        assert!(!diagnostic_codes.contains("NONLINEAR_SUPPORT_LOOP_BLOCKED"));
    }

    fn gap_closure_preview_request() -> LinearStaticPreviewRequest {
        two_node_nonlinear_preview_request(
            "support:NL-GAP-110",
            NonlinearSupportInput {
                behavior: "gap".to_string(),
                dof: "UX".to_string(),
                initial_state: Some("inactive".to_string()),
                active_when: None,
                contact_when: None,
                closes_when: Some("positive_displacement".to_string()),
                gap: Some(Quantity {
                    value: 0.05,
                    unit: "mm".to_string(),
                }),
                friction_coefficient: None,
                normal_reaction: None,
                normal_reaction_source: None,
            },
            "load:L-GAP",
            100_000.0,
            "combination:C-GAP",
        )
    }

    #[test]
    fn gap_preview_closes_to_explicit_clearance_through_dense_loop() {
        let result = run_linear_static_preview(gap_closure_preview_request());
        let diagnostic_codes = result
            .diagnostics
            .iter()
            .map(|item| item.code.as_str())
            .collect::<HashSet<_>>();

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(
            result_value(&result, "result:nonlinear-support:iteration-count"),
            2.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:final-residual-count"),
            0.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:converged-flag"),
            1.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-GAP-110:state-code"
            ),
            1.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-GAP-110:ux-displacement"
            ),
            0.05
        );
        assert!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-GAP-110:ux-reaction"
            ) < 0.0
        );
        assert!(!diagnostic_codes.contains("TOLERANCE_POLICY_TBD"));
        assert!(diagnostic_codes.contains("NONLINEAR_SUPPORT_STATE_REVIEW"));
        assert!(diagnostic_codes.contains("NONLINEAR_SUPPORT_LOOP_CONVERGED"));
        assert!(!diagnostic_codes.contains("NONLINEAR_SUPPORT_LOOP_BLOCKED"));
    }

    fn lift_off_release_preview_request() -> LinearStaticPreviewRequest {
        two_node_nonlinear_preview_request(
            "support:NL-LIFT-110",
            NonlinearSupportInput {
                behavior: "lift_off".to_string(),
                dof: "UX".to_string(),
                initial_state: Some("active".to_string()),
                active_when: None,
                contact_when: Some("positive_reaction".to_string()),
                closes_when: None,
                gap: None,
                friction_coefficient: None,
                normal_reaction: None,
                normal_reaction_source: None,
            },
            "load:L-LIFT",
            100.0,
            "combination:C-LIFT",
        )
    }

    #[test]
    fn lift_off_preview_releases_contact_through_dense_loop() {
        let result = run_linear_static_preview(lift_off_release_preview_request());
        let diagnostic_codes = result
            .diagnostics
            .iter()
            .map(|item| item.code.as_str())
            .collect::<HashSet<_>>();

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(
            result_value(&result, "result:nonlinear-support:iteration-count"),
            2.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:final-residual-count"),
            0.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:converged-flag"),
            1.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-LIFT-110:state-code"
            ),
            0.0
        );
        assert!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-LIFT-110:ux-displacement"
            ) > 0.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-LIFT-110:ux-reaction"
            ),
            0.0
        );
        assert!(!diagnostic_codes.contains("TOLERANCE_POLICY_TBD"));
        assert!(diagnostic_codes.contains("NONLINEAR_SUPPORT_STATE_REVIEW"));
        assert!(diagnostic_codes.contains("NONLINEAR_SUPPORT_LOOP_CONVERGED"));
        assert!(!diagnostic_codes.contains("NONLINEAR_SUPPORT_LOOP_BLOCKED"));
    }

    #[test]
    fn operation_authored_primitive_categories_map_to_preview_mechanics() {
        assert_eq!(
            parse_category("concentrated_force").unwrap(),
            PrimitiveLoadCategory::Occasional
        );
        assert_eq!(
            parse_category("concentrated_moment").unwrap(),
            PrimitiveLoadCategory::Occasional
        );
        assert_eq!(
            parse_category("distributed_force").unwrap(),
            PrimitiveLoadCategory::Weight
        );

        let mut request = request();
        request.model.load_cases.truncate(1);
        request.model.combinations.clear();
        let primitive = request.model.load_cases[0]
            .primitive_loads
            .iter_mut()
            .find(|load| load.id == "load:L-100-Y")
            .expect("fixture carries a nodal force primitive");
        primitive.category = "concentrated_force".to_string();

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(result
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "LOAD_INPUT_INVALID"));
        assert!(result
            .results
            .iter()
            .any(|item| item.id == "result:disp:node-N-140"));

        let mapping = result
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "LOAD_CATEGORY_PREVIEW_MAPPED")
            .expect("authored category mapping must surface as a named diagnostic");
        assert_eq!(mapping.severity, "warning");
        assert!(mapping.message.contains("concentrated_force"));
        assert!(mapping.message.contains("occasional"));
        assert!(mapping
            .affected_refs
            .iter()
            .any(|reference| reference == "load:L-100-Y"));

        let native = run_linear_static_preview(self::request());
        assert_eq!(native.status.mechanics, "MECHANICS_SOLVED");
        assert!(
            native
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "LOAD_CATEGORY_PREVIEW_MAPPED"),
            "native preview categories must not emit the mapping diagnostic"
        );
    }

    #[test]
    fn valid_invented_model_exposes_element_force_components() {
        let result = run_linear_static_preview(request());
        let result_ids = result
            .results
            .iter()
            .map(|item| item.id.as_str())
            .collect::<HashSet<_>>();

        assert!(result_ids.contains("result:force:pipe-P-120:axial"));
        assert!(result_ids.contains("result:force:pipe-P-120:axial:end-j"));
        assert!(result_ids.contains("result:force:pipe-P-120:shear-y"));
        assert!(result_ids.contains("result:force:pipe-P-120:shear-y:end-j"));
        assert!(result_ids.contains("result:force:pipe-P-120:shear-z"));
        assert!(result_ids.contains("result:force:pipe-P-120:shear-z:end-j"));
        assert!(result_ids.contains("result:moment:pipe-P-120:torsion"));
        assert!(result_ids.contains("result:moment:pipe-P-120:torsion:end-j"));
        assert!(result_ids.contains("result:moment:pipe-P-120:bending-y"));
        assert!(result_ids.contains("result:moment:pipe-P-120:bending-y:end-j"));
        assert!(result_ids.contains("result:moment:pipe-P-120:bending-z"));
        assert!(result_ids.contains("result:moment:pipe-P-120:bending-z:end-j"));
        assert!(result_ids.contains("result:force:pipe-P-120:quarter-1:axial"));
        assert!(result_ids.contains("result:force:pipe-P-120:quarter-1:shear-y"));
        assert!(result_ids.contains("result:force:pipe-P-120:quarter-1:shear-z"));
        assert!(result_ids.contains("result:force:pipe-P-120:midspan:axial"));
        assert!(result_ids.contains("result:force:pipe-P-120:midspan:shear-y"));
        assert!(result_ids.contains("result:force:pipe-P-120:midspan:shear-z"));
        assert!(result_ids.contains("result:force:pipe-P-120:quarter-3:axial"));
        assert!(result_ids.contains("result:force:pipe-P-120:quarter-3:shear-y"));
        assert!(result_ids.contains("result:force:pipe-P-120:quarter-3:shear-z"));
        assert!(result_ids.contains("result:moment:pipe-P-120:quarter-1:torsion"));
        assert!(result_ids.contains("result:moment:pipe-P-120:quarter-1:bending-y"));
        assert!(result_ids.contains("result:moment:pipe-P-120:quarter-1:bending-z"));
        assert!(result_ids.contains("result:moment:pipe-P-120:midspan:torsion"));
        assert!(result_ids.contains("result:moment:pipe-P-120:midspan:bending-y"));
        assert!(result_ids.contains("result:moment:pipe-P-120:midspan:bending-z"));
        assert!(result_ids.contains("result:moment:pipe-P-120:quarter-3:torsion"));
        assert!(result_ids.contains("result:moment:pipe-P-120:quarter-3:bending-y"));
        assert!(result_ids.contains("result:moment:pipe-P-120:quarter-3:bending-z"));
        assert!(!result_ids.contains("result:force:pipe-P-120:end-i:axial"));
        assert!(result.results.iter().any(|item| {
            item.id == "result:force:pipe-P-120:axial"
                && item.kind == "element_local_axial_force"
                && item.unit == "N"
                && item
                    .metadata
                    .as_ref()
                    .map(|metadata| {
                        metadata.component == "axial_force"
                            && metadata.coordinate_system == "element_local"
                            && metadata.location == "end_i"
                    })
                    .unwrap_or(false)
        }));
        assert!(result.results.iter().any(|item| {
            item.id == "result:force:pipe-P-120:shear-y"
                && item.kind == "element_local_shear_force_y"
                && item.unit == "N"
                && item
                    .metadata
                    .as_ref()
                    .map(|metadata| {
                        metadata.component == "shear_force_y"
                            && metadata.coordinate_system == "element_local"
                            && metadata.location == "end_i"
                            && metadata.basis == "recovered_from_local_element_stiffness"
                    })
                    .unwrap_or(false)
        }));
        assert!(result.results.iter().any(|item| {
            item.id == "result:force:pipe-P-120:axial:end-j"
                && item.kind == "element_local_axial_force"
                && item.unit == "N"
                && item
                    .metadata
                    .as_ref()
                    .map(|metadata| {
                        metadata.component == "axial_force"
                            && metadata.coordinate_system == "element_local"
                            && metadata.location == "end_j"
                            && metadata.sign_convention.contains("j-end")
                    })
                    .unwrap_or(false)
        }));
        assert!(result.results.iter().any(|item| {
            item.id == "result:force:pipe-P-120:quarter-1:shear-z"
                && item.kind == "element_local_shear_force_z"
                && item.unit == "N"
                && item
                    .metadata
                    .as_ref()
                    .map(|metadata| {
                        metadata.component == "shear_force_z"
                            && metadata.coordinate_system == "element_local"
                            && metadata.location == "quarter_1"
                            && metadata.basis == "interpolated_from_endpoint_resultants"
                    })
                    .unwrap_or(false)
        }));
        assert!(result.results.iter().any(|item| {
            item.id == "result:force:pipe-P-120:midspan:axial"
                && item.kind == "element_local_axial_force"
                && item.unit == "N"
                && item
                    .metadata
                    .as_ref()
                    .map(|metadata| {
                        metadata.component == "axial_force"
                            && metadata.coordinate_system == "element_local"
                            && metadata.location == "midspan"
                            && metadata.basis == "interpolated_from_endpoint_resultants"
                    })
                    .unwrap_or(false)
        }));
    }

    #[test]
    fn valid_invented_model_exposes_global_displacement_components() {
        let result = run_linear_static_preview(request());
        let result_ids = result
            .results
            .iter()
            .map(|item| item.id.as_str())
            .collect::<HashSet<_>>();

        for node in [
            "node-N-100",
            "node-N-110",
            "node-N-120",
            "node-N-130",
            "node-N-140",
        ] {
            for tail in ["ux", "uy", "uz", "rx", "ry", "rz"] {
                assert!(result_ids.contains(format!("result:disp:{node}:{tail}").as_str()));
                assert!(result_ids
                    .contains(format!("result:loadcase:load-L-200:disp:{node}:{tail}").as_str()));
                assert!(result_ids.contains(
                    format!("result:combination:combination-C-OPER-ALT:disp:{node}:{tail}")
                        .as_str()
                ));
            }
        }

        assert!(result.results.iter().any(|item| {
            item.id == "result:disp:node-N-140:uy"
                && item.kind == "global_nodal_displacement_y"
                && item.unit == "mm"
                && item.entity_ref == "node:N-140"
                && item
                    .metadata
                    .as_ref()
                    .map(|metadata| {
                        metadata.component == "nodal_displacement_y"
                            && metadata.coordinate_system == "global"
                            && metadata.location == "node"
                            && metadata.basis == "solved_from_global_linear_system"
                            && metadata.sign_convention.contains("global cartesian Y axis")
                    })
                    .unwrap_or(false)
        }));
        assert!(result.results.iter().any(|item| {
            item.id == "result:disp:node-N-140:rz"
                && item.kind == "global_nodal_rotation_z"
                && item.unit == "rad"
                && item.entity_ref == "node:N-140"
                && item
                    .metadata
                    .as_ref()
                    .map(|metadata| {
                        metadata.component == "nodal_rotation_z"
                            && metadata.coordinate_system == "global"
                            && metadata.location == "node"
                            && metadata.basis == "solved_from_global_linear_system"
                            && metadata.sign_convention.contains("right-hand-rule")
                    })
                    .unwrap_or(false)
        }));

        // Translation components reassemble the published magnitude row within
        // round6 tolerance.
        let ux = result_value(&result, "result:disp:node-N-140:ux");
        let uy = result_value(&result, "result:disp:node-N-140:uy");
        let uz = result_value(&result, "result:disp:node-N-140:uz");
        let magnitude = result_value(&result, "result:disp:node-N-140");
        assert!(((ux * ux + uy * uy + uz * uz).sqrt() - magnitude).abs() < 5.0e-6);

        // Component rows join the explicit user combination algebra exactly
        // like other supported scalar rows.
        let default_uy = result_value(&result, "result:disp:node-N-140:uy");
        let alternate_uy = result_value(&result, "result:loadcase:load-L-200:disp:node-N-140:uy");
        let combined_uy = result
            .results
            .iter()
            .find(|item| item.id == "result:combination:combination-C-OPER-ALT:disp:node-N-140:uy")
            .expect("combination displacement component row should be emitted");
        assert_eq!(combined_uy.value, round6(default_uy + 0.5 * alternate_uy));
        assert_eq!(
            combined_uy
                .metadata
                .as_ref()
                .map(|metadata| metadata.basis.as_str()),
            Some("explicit_user_linear_combination")
        );

        // Deterministic emission position: all magnitude rows first, then the
        // component block, then reaction rows, per load case.
        let index_of = |id: &str| {
            result
                .results
                .iter()
                .position(|item| item.id == id)
                .unwrap_or_else(|| panic!("missing result {id}"))
        };
        assert!(index_of("result:disp:node-N-140") < index_of("result:disp:node-N-100:ux"));
        assert!(index_of("result:disp:node-N-140:rz") < index_of("result:reaction:support-S-100"));
    }

    #[test]
    fn displacement_component_rows_carry_signed_global_directions() {
        let solve = |magnitude: f64| {
            let mut request = request();
            request.model.nodes.truncate(2);
            request.model.nodes[0].id = "node:N-100".to_string();
            request.model.nodes[0].position = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            request.model.nodes[1].id = "node:N-110".to_string();
            request.model.nodes[1].position = Vec3 {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            };
            request.model.pipe_segments.truncate(1);
            request.model.pipe_segments[0].id = "pipe:P-100".to_string();
            request.model.pipe_segments[0].from = "node:N-100".to_string();
            request.model.pipe_segments[0].to = "node:N-110".to_string();
            request.model.pipe_segments[0].y_reference = Some(Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
            request.model.supports.truncate(1);
            request.model.supports[0].id = "support:S-100".to_string();
            request.model.supports[0].node = "node:N-100".to_string();
            request.model.supports[0].restraints = vec![
                "UX".to_string(),
                "UY".to_string(),
                "UZ".to_string(),
                "RX".to_string(),
                "RY".to_string(),
                "RZ".to_string(),
            ];
            request.model.load_cases.truncate(1);
            request.model.combinations.clear();
            request.model.load_cases[0].primitive_loads = vec![PreviewPrimitiveLoad {
                id: "load:L-TIP-Y".to_string(),
                category: "occasional".to_string(),
                target: LoadTargetInput::Node {
                    node: "node:N-110".to_string(),
                },
                direction: "global_y".to_string(),
                magnitude: Quantity {
                    value: magnitude,
                    unit: "N".to_string(),
                },
                dimension: "force".to_string(),
                provenance: Some("invented_example_user_input".to_string()),
            }];
            run_linear_static_preview(request)
        };

        let upward = solve(350.0);
        assert_eq!(upward.status.mechanics, "MECHANICS_SOLVED");
        let tip_uy = result_value(&upward, "result:disp:node-N-110:uy");
        let tip_rz = result_value(&upward, "result:disp:node-N-110:rz");
        assert!(tip_uy > 0.0, "+Y tip force must displace the tip in +Y");
        assert!(
            tip_rz > 0.0,
            "+Y tip force on a +X member must rotate about +Z"
        );
        assert_eq!(result_value(&upward, "result:disp:node-N-110:ux"), 0.0);
        assert_eq!(result_value(&upward, "result:disp:node-N-110:uz"), 0.0);
        assert_eq!(result_value(&upward, "result:disp:node-N-110:rx"), 0.0);
        assert_eq!(result_value(&upward, "result:disp:node-N-110:ry"), 0.0);
        assert_eq!(result_value(&upward, "result:disp:node-N-100:uy"), 0.0);

        let downward = solve(-350.0);
        assert_eq!(downward.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(
            result_value(&downward, "result:disp:node-N-110:uy"),
            -tip_uy
        );
        assert_eq!(
            result_value(&downward, "result:disp:node-N-110:rz"),
            -tip_rz
        );
        assert_eq!(
            result_value(&downward, "result:disp:node-N-110"),
            result_value(&upward, "result:disp:node-N-110"),
            "magnitude row stays unsigned while component rows carry sign"
        );
    }

    #[test]
    fn displacement_component_rows_are_deterministic_across_runs() {
        let first = serde_json::to_string(&run_linear_static_preview(request())).unwrap();
        let second = serde_json::to_string(&run_linear_static_preview(request())).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn valid_invented_model_exposes_endpoint_stress_components() {
        let result = run_linear_static_preview(request());
        let result_ids = result
            .results
            .iter()
            .map(|item| item.id.as_str())
            .collect::<HashSet<_>>();

        assert!(result_ids.contains("result:stress:pipe-P-120"));
        assert!(result_ids.contains("result:stress:pipe-P-120:end-i:axial-normal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:end-i:torsional-shear"));
        assert!(result_ids.contains("result:stress:pipe-P-120:end-i:pressure-hoop"));
        assert!(result_ids.contains("result:stress:pipe-P-120:end-j:axial-normal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:end-j:torsional-shear"));
        assert!(!result_ids.contains("result:stress:pipe-P-120:end-j:pressure-longitudinal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-1:axial-normal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-1:bending-normal-y"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-1:bending-normal-z"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-1:torsional-shear"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-1:pressure-hoop"));
        assert!(!result_ids.contains("result:stress:pipe-P-120:quarter-1:pressure-longitudinal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:midspan:axial-normal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:midspan:bending-normal-y"));
        assert!(result_ids.contains("result:stress:pipe-P-120:midspan:bending-normal-z"));
        assert!(result_ids.contains("result:stress:pipe-P-120:midspan:torsional-shear"));
        assert!(result_ids.contains("result:stress:pipe-P-120:midspan:pressure-hoop"));
        assert!(!result_ids.contains("result:stress:pipe-P-120:midspan:pressure-longitudinal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-3:axial-normal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-3:bending-normal-y"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-3:bending-normal-z"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-3:torsional-shear"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-3:pressure-hoop"));
        assert!(!result_ids.contains("result:stress:pipe-P-120:quarter-3:pressure-longitudinal"));
        assert!(!result_ids.contains("result:stress:pipe-P-120:midspan:shear-y"));
        assert!(!result_ids.contains("result:stress:pipe-P-120:quarter-1:shear-y"));
        assert!(result.results.iter().any(|item| {
            item.id == "result:stress:pipe-P-120:end-j:torsional-shear"
                && item.kind == "element_local_torsional_shear_stress"
                && item.unit == "MPa"
                && item
                    .metadata
                    .as_ref()
                    .map(|metadata| {
                        metadata.component == "torsional_shear_stress"
                            && metadata.coordinate_system == "element_local"
                            && metadata.location == "end_j"
                            && metadata.basis == "recovered_from_open_mechanics_stress_components"
                    })
                    .unwrap_or(false)
        }));
        assert!(result.results.iter().any(|item| {
            item.id == "result:stress:pipe-P-120:end-i:pressure-hoop"
                && item.kind == "pipe_section_pressure_hoop_stress"
                && item.unit == "MPa"
                && item
                    .metadata
                    .as_ref()
                    .map(|metadata| {
                        metadata.component == "pressure_hoop_stress"
                            && metadata.coordinate_system == "pipe_section"
                            && metadata.location == "end_i"
                    })
                    .unwrap_or(false)
        }));
        assert!(result.results.iter().any(|item| {
            item.id == "result:stress:pipe-P-120:midspan:torsional-shear"
                && item.kind == "element_local_torsional_shear_stress"
                && item.unit == "MPa"
                && item
                    .metadata
                    .as_ref()
                    .map(|metadata| {
                        metadata.component == "torsional_shear_stress"
                            && metadata.coordinate_system == "element_local"
                            && metadata.location == "midspan"
                            && metadata.basis == "interpolated_from_endpoint_resultants"
                    })
                    .unwrap_or(false)
        }));
        assert!(result.results.iter().any(|item| {
            item.id == "result:stress:pipe-P-120:quarter-1:torsional-shear"
                && item.kind == "element_local_torsional_shear_stress"
                && item.unit == "MPa"
                && item
                    .metadata
                    .as_ref()
                    .map(|metadata| {
                        metadata.component == "torsional_shear_stress"
                            && metadata.coordinate_system == "element_local"
                            && metadata.location == "quarter_1"
                            && metadata.basis == "interpolated_from_endpoint_resultants"
                    })
                    .unwrap_or(false)
        }));
        assert!(!result
            .diagnostics
            .iter()
            .any(|item| item.code == "PRESSURE_LOAD_NOT_APPLIED_TO_FRAME_VECTOR"));
    }

    #[test]
    fn bend_component_user_multipliers_emit_stress_review_rows() {
        let result = run_linear_static_preview(request());
        let default_row_id = "result:stress:component-C-110:pipe-P-100:end-j:user-multiplier";
        let combination_row_id =
            "result:combination:combination-C-OPER-ALT:stress:component-C-110:pipe-P-100:end-j:user-multiplier";
        let default_row = result
            .results
            .iter()
            .find(|item| item.id == default_row_id)
            .expect("bend user multiplier row should be emitted for adjacent pipe endpoint");
        let combination_row = result
            .results
            .iter()
            .find(|item| item.id == combination_row_id)
            .expect("bend user multiplier row should participate in explicit combinations");

        assert_eq!(result.summary.component_stress_modifier_count, 8);
        assert_eq!(default_row.kind, "component_user_stress_multiplier_review");
        assert_eq!(default_row.entity_ref, "component:C-110");
        assert!(default_row.value > 0.0);
        assert!(default_row
            .source_result_refs
            .contains(&"result:stress:pipe-P-100:end-j:axial-normal".to_string()));
        assert!(default_row
            .source_result_refs
            .contains(&"result:stress:pipe-P-100:end-j:bending-normal-y".to_string()));
        assert!(default_row
            .source_result_refs
            .contains(&"result:stress:pipe-P-100".to_string()));
        let metadata = default_row
            .metadata
            .as_ref()
            .expect("component multiplier row carries recovery metadata");
        assert_eq!(
            metadata.component,
            "user_entered_component_stress_multiplier"
        );
        assert_eq!(metadata.coordinate_system, "component_review");
        assert_eq!(metadata.location, "pipe:P-100:end_j");
        assert!(metadata.basis.contains("user_entered_sif=1.15"));
        assert!(metadata.basis.contains("user_entered_flexibility=1.08"));
        assert!(metadata
            .basis
            .contains("source=invented_user_entered_preview_no_code_table"));
        assert!(metadata
            .basis
            .contains("solver_consumption=mechanics_geometry_only"));
        assert!(metadata
            .sign_convention
            .contains("base frame stiffness unchanged"));

        assert_eq!(
            combination_row
                .basis_ref
                .as_ref()
                .map(|basis| basis.ref_id.as_str()),
            Some("combination:C-OPER-ALT")
        );
        assert!(
            result
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.code == "COMPONENT_STRESS_MULTIPLIER_APPLIED")
                .count()
                >= 4
        );
    }

    #[test]
    fn branch_component_user_multipliers_emit_side_specific_stress_review_rows() {
        let result = run_linear_static_preview(request());
        let branch_row_id = "result:stress:component-C-120:pipe-P-110:end-j:user-multiplier";
        let header_row_id = "result:stress:component-C-120:pipe-P-120:end-i:user-multiplier";
        let combination_row_id =
            "result:combination:combination-C-OPER-ALT:stress:component-C-120:pipe-P-120:end-i:user-multiplier";
        let branch_row = result
            .results
            .iter()
            .find(|item| item.id == branch_row_id)
            .expect(
                "branch-side user multiplier row should be emitted for the branch pipe endpoint",
            );
        let header_row = result
            .results
            .iter()
            .find(|item| item.id == header_row_id)
            .expect(
                "header-side user multiplier row should be emitted for the header pipe endpoint",
            );
        let combination_row = result
            .results
            .iter()
            .find(|item| item.id == combination_row_id)
            .expect("branch user multiplier row should participate in explicit combinations");

        assert_eq!(branch_row.kind, "component_user_stress_multiplier_review");
        assert_eq!(branch_row.entity_ref, "component:C-120");
        assert_eq!(header_row.entity_ref, "component:C-120");
        assert!(branch_row.value > 0.0);
        assert!(header_row.value > 0.0);

        let branch_metadata = branch_row
            .metadata
            .as_ref()
            .expect("branch-side multiplier row carries recovery metadata");
        assert_eq!(branch_metadata.coordinate_system, "component_review");
        assert_eq!(branch_metadata.location, "pipe:P-110:end_j");
        assert!(branch_metadata.basis.contains("component_family=branch"));
        assert!(branch_metadata.basis.contains("component_side=branch"));
        assert!(branch_metadata.basis.contains("user_entered_sif=1.31"));
        assert!(branch_metadata
            .basis
            .contains("source=invented_user_entered_branch_modifiers_no_code_table"));

        let header_metadata = header_row
            .metadata
            .as_ref()
            .expect("header-side multiplier row carries recovery metadata");
        assert_eq!(header_metadata.location, "pipe:P-120:end_i");
        assert!(header_metadata.basis.contains("component_family=branch"));
        assert!(header_metadata.basis.contains("component_side=header"));
        assert!(header_metadata.basis.contains("user_entered_sif=1.22"));

        assert_eq!(
            combination_row
                .basis_ref
                .as_ref()
                .map(|basis| basis.ref_id.as_str()),
            Some("combination:C-OPER-ALT")
        );
    }

    #[test]
    fn expansion_joint_user_stiffness_emits_macro_element_review_rows() {
        let result = run_linear_static_preview(request());
        let axial = result
            .results
            .iter()
            .find(|item| item.id == "result:component-stiffness:component-C-150:axial")
            .expect("expansion joint axial stiffness review row should be emitted");
        let torsional = result
            .results
            .iter()
            .find(|item| item.id == "result:component-stiffness:component-C-150:torsional")
            .expect("expansion joint torsional stiffness review row should be emitted");

        assert_eq!(
            result.summary.component_user_stiffness_macro_element_count,
            4
        );
        assert_eq!(axial.kind, "component_user_stiffness_macro_element_review");
        assert_eq!(axial.entity_ref, "component:C-150");
        assert_eq!(axial.value, 3_200_000.0);
        assert_eq!(axial.unit, "N/m");
        let axial_metadata = axial
            .metadata
            .as_ref()
            .expect("expansion joint row carries macro-element metadata");
        assert_eq!(axial_metadata.component, "axial_user_stiffness");
        assert_eq!(axial_metadata.coordinate_system, "component_local_preview");
        assert_eq!(axial_metadata.location, "pipe:P-130");
        assert!(axial_metadata
            .basis
            .contains("component_family=expansion_joint"));
        assert!(axial_metadata
            .basis
            .contains("solver_consumption=mechanics_geometry_and_user_flexibility"));
        assert!(axial_metadata
            .basis
            .contains("macro_element_solve=assembled_user_stiffness"));
        assert!(axial_metadata
            .basis
            .contains("pressure_thrust_generation=load_side_user_effective_area"));
        assert!(axial_metadata
            .basis
            .contains("pressure_thrust=load_side_pressure_thrust_user_review_required"));
        assert!(axial_metadata
            .sign_convention
            .contains("consumed by the assembled user-stiffness macro-element"));

        assert_eq!(torsional.value, 620_000.0);
        assert_eq!(torsional.unit, "N*m/rad");
        assert!(result
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "EXPANSION_JOINT_USER_STIFFNESS_REVIEWED"));
        assert!(
            result
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code
                    != "EXPANSION_JOINT_MECHANICS_INTERFACE_UNSUPPORTED")
        );
    }

    #[test]
    fn expansion_joint_pressure_thrust_uses_user_effective_area_as_load_side_evidence() {
        let result = run_linear_static_preview(request());
        let default_row = result
            .results
            .iter()
            .find(|item| item.id == "result:pressure-thrust:component-C-150")
            .expect("default load case expansion joint pressure-thrust row should be emitted");
        let alternate_row = result
            .results
            .iter()
            .find(|item| item.id == "result:loadcase:load-L-200:pressure-thrust:component-C-150")
            .expect("alternate load case expansion joint pressure-thrust row should be emitted");
        let combination_row = result
            .results
            .iter()
            .find(|item| {
                item.id
                    == "result:combination:combination-C-OPER-ALT:pressure-thrust:component-C-150"
            })
            .expect(
                "expansion joint pressure-thrust row should participate in explicit combinations",
            );

        assert_eq!(result.summary.component_pressure_thrust_load_count, 2);
        assert_eq!(
            default_row.kind,
            "expansion_joint_pressure_thrust_load_review"
        );
        assert_eq!(default_row.entity_ref, "component:C-150");
        assert_eq!(default_row.value, 21_600.0);
        assert_eq!(default_row.unit, "N");
        assert_eq!(default_row.source_result_refs, vec!["load:L-100-P-EJ"]);
        let metadata = default_row
            .metadata
            .as_ref()
            .expect("pressure-thrust row carries load-side evidence metadata");
        assert_eq!(metadata.component, "expansion_joint_pressure_thrust");
        assert_eq!(metadata.coordinate_system, "element_local");
        assert_eq!(metadata.location, "pipe:P-130");
        assert!(metadata
            .basis
            .contains("pressure_thrust_generation=load_side_user_effective_area"));
        assert!(metadata.basis.contains("effective_area=0.018"));
        assert!(metadata
            .basis
            .contains("source=invented_user_entered_expansion_joint_preview_geometry"));
        assert!(metadata
            .sign_convention
            .contains("user-entered effective pressure area"));

        assert_eq!(alternate_row.value, 10_800.0);
        assert_eq!(alternate_row.source_result_refs, vec!["load:L-200-P-EJ"]);
        assert_eq!(combination_row.value, 27_000.0);
        assert_eq!(
            combination_row.source_result_refs,
            vec![
                "result:pressure-thrust:component-C-150".to_string(),
                "result:loadcase:load-L-200:pressure-thrust:component-C-150".to_string(),
            ]
        );
        assert!(result
            .results
            .iter()
            .any(|item| item.id == "result:stress:pipe-P-130:end-i:pressure-hoop"));
        assert!(!result
            .results
            .iter()
            .any(|item| item.id == "result:stress:pipe-P-130:end-i:pressure-longitudinal"));
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "EXPANSION_JOINT_PRESSURE_THRUST_APPLIED"
                && diagnostic
                    .affected_refs
                    .contains(&"load:L-100-P-EJ".to_string())
        }));
    }

    #[test]
    fn spring_hanger_user_inputs_emit_review_rows_without_catalog_defaults() {
        let result = run_linear_static_preview(request());
        let variable_stiffness = result
            .results
            .iter()
            .find(|item| item.id == "result:spring-hanger:support-SH-140:stiffness")
            .expect("variable spring hanger stiffness review row should be emitted");
        let constant_load = result
            .results
            .iter()
            .find(|item| item.id == "result:constant-effort-support:support-CE-120:constant-load")
            .expect("constant-effort support load review row should be emitted");

        assert_eq!(result.summary.spring_hanger_user_input_count, 7);
        assert_eq!(variable_stiffness.kind, "spring_hanger_user_input_review");
        assert_eq!(variable_stiffness.entity_ref, "support:SH-140");
        assert_eq!(variable_stiffness.value, 42_000.0);
        assert_eq!(variable_stiffness.unit, "N/m");
        let variable_metadata = variable_stiffness
            .metadata
            .as_ref()
            .expect("spring hanger row carries support metadata");
        assert_eq!(
            variable_metadata.component,
            "variable_spring_hanger_stiffness"
        );
        assert_eq!(variable_metadata.coordinate_system, "support_local_preview");
        assert!(variable_metadata
            .basis
            .contains("mechanics_consumption=linear_spring_primitive_user_stiffness"));
        assert!(variable_metadata.basis.contains("dec_ref=DEC-049"));
        assert!(variable_metadata
            .sign_convention
            .contains("no catalog/default value is supplied"));

        assert_eq!(constant_load.kind, "constant_effort_user_input_review");
        assert_eq!(constant_load.entity_ref, "support:CE-120");
        assert_eq!(constant_load.value, 375.0);
        assert_eq!(constant_load.unit, "N");
        let constant_metadata = constant_load
            .metadata
            .as_ref()
            .expect("constant-effort row carries review metadata");
        assert!(constant_metadata
            .basis
            .contains("mechanics_consumption=load_side_review_only_no_global_solve_consumption"));
        assert!(constant_metadata
            .sign_convention
            .contains("no global constant-effort load"));
        assert!(result
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "SPRING_HANGER_USER_DATA_REVIEWED"));
        assert!(result
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "CONSTANT_EFFORT_USER_DATA_REVIEWED"));
    }

    #[test]
    fn spring_hanger_missing_stiffness_blocks_without_defaulting() {
        let mut request = request();
        let support = request
            .model
            .supports
            .iter_mut()
            .find(|support| support.id == "support:SH-140")
            .expect("fixture carries variable spring hanger");
        support.stiffness = None;
        support
            .hanger
            .as_mut()
            .expect("fixture carries hanger data")
            .stiffness = None;

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert_eq!(result.summary.spring_hanger_user_input_count, 0);
        assert!(result
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "SPRING_HANGER_STIFFNESS_MISSING"));
    }

    #[test]
    fn valid_invented_model_exposes_explicit_load_combination_results() {
        let result = run_linear_static_preview(request());
        let combination_id = "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial";
        let alternate_load_case_id = "result:loadcase:load-L-200:force:pipe-P-120:axial";
        let quarter_combination_id =
            "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y";
        let combination = result
            .results
            .iter()
            .find(|item| item.id == combination_id)
            .expect("combination result should be emitted");
        let alternate = result
            .results
            .iter()
            .find(|item| item.id == alternate_load_case_id)
            .expect("non-default load-case result should be emitted");
        let quarter_combination = result
            .results
            .iter()
            .find(|item| item.id == quarter_combination_id)
            .expect("station-grid combination result should be emitted");

        assert_eq!(result.summary.load_case_count, 2);
        assert_eq!(
            alternate
                .basis_ref
                .as_ref()
                .map(|basis| basis.ref_id.as_str()),
            Some("load:L-200")
        );
        assert_eq!(
            combination
                .basis_ref
                .as_ref()
                .map(|basis| basis.ref_id.as_str()),
            Some("combination:C-OPER-ALT")
        );
        assert_eq!(
            combination.source_result_refs,
            vec![
                "result:force:pipe-P-120:axial".to_string(),
                "result:loadcase:load-L-200:force:pipe-P-120:axial".to_string(),
            ]
        );
        assert_eq!(
            combination
                .metadata
                .as_ref()
                .map(|metadata| metadata.basis.as_str()),
            Some("explicit_user_linear_combination")
        );
        assert_eq!(quarter_combination.unit, "N");
        assert_eq!(
            quarter_combination
                .basis_ref
                .as_ref()
                .map(|basis| basis.ref_id.as_str()),
            Some("combination:C-OPER-ALT")
        );
        assert_eq!(
            quarter_combination.source_result_refs,
            vec![
                "result:force:pipe-P-120:quarter-1:shear-y".to_string(),
                "result:loadcase:load-L-200:force:pipe-P-120:quarter-1:shear-y".to_string(),
            ]
        );
        assert_eq!(
            quarter_combination.metadata.as_ref().map(|metadata| (
                metadata.component.as_str(),
                metadata.location.as_str(),
                metadata.basis.as_str()
            )),
            Some((
                "shear_force_y",
                "quarter_1",
                "explicit_user_linear_combination"
            ))
        );
    }

    #[test]
    fn combination_stress_summary_rows_are_skipped_with_diagnostics() {
        let result = run_linear_static_preview(request());

        assert!(!result
            .results
            .iter()
            .any(|item| item.id == "result:combination:combination-C-OPER-ALT:stress:pipe-P-120"));
        assert!(result.diagnostics.iter().any(|item| item.code
            == "COMBINATION_STRESS_SUMMARY_SKIPPED"
            && item
                .affected_refs
                .contains(&"result:stress:pipe-P-120".to_string())));
    }

    #[test]
    fn fixed_fixed_thermal_load_applies_axial_fixed_end_correction() {
        let request = fixed_fixed_thermal_request("global_z");
        let area = derive_pipe_section(
            &request.model.pipe_segments[0].section,
            "pipe:P-100",
            &mut Vec::new(),
        )
        .unwrap()
        .area;
        let expected_force = 200_000_000_000.0 * area * 0.000012 * 10.0;
        let result = run_linear_static_preview(request);
        let axial_i = result
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:axial")
            .unwrap();
        let axial_j = result
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:axial:end-j")
            .unwrap();
        let stress_i = result
            .results
            .iter()
            .find(|item| item.id == "result:stress:pipe-P-100:end-i:axial-normal")
            .unwrap();

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!((axial_i.value - round6(expected_force)).abs() < 1.0e-6);
        assert!((axial_j.value + round6(expected_force)).abs() < 1.0e-6);
        assert!((stress_i.value - round6(expected_force / area / 1_000_000.0)).abs() < 1.0e-6);
    }

    fn mill_tolerance_section(mill_tolerance: Option<f64>) -> PipeSectionInput {
        PipeSectionInput {
            outside_diameter: Quantity {
                value: 0.2,
                unit: "m".to_string(),
            },
            wall_thickness: Quantity {
                value: 0.01,
                unit: "m".to_string(),
            },
            mill_tolerance: mill_tolerance.map(|value| Quantity {
                value,
                unit: "m".to_string(),
            }),
            material_density: None,
            contents_density: None,
            insulation_thickness: None,
            insulation_density: None,
        }
    }

    #[test]
    fn mill_tolerance_reduces_derived_effective_wall_and_section_modulus() {
        let mut diagnostics = Vec::new();
        let nominal = derive_pipe_section(
            &mill_tolerance_section(None),
            "pipe:P-MILL",
            &mut diagnostics,
        )
        .unwrap();
        let reduced = derive_pipe_section(
            &mill_tolerance_section(Some(0.00125)),
            "pipe:P-MILL",
            &mut diagnostics,
        )
        .unwrap();
        assert!(diagnostics.is_empty());

        let od: f64 = 0.2;
        let effective_wall = 0.01 - 0.00125;
        let id = od - 2.0 * effective_wall;
        let expected_modulus = PI * (od.powi(4) - id.powi(4)) / 64.0 / (od / 2.0);
        assert!((reduced.wall_thickness - effective_wall).abs() < 1.0e-12);
        assert!((reduced.section_modulus - expected_modulus).abs() < 1.0e-12);
        assert!(reduced.section_modulus < nominal.section_modulus);
        assert!(reduced.area < nominal.area);
        assert!(reduced.internal_area > nominal.internal_area);
    }

    #[test]
    fn absent_mill_tolerance_slot_means_no_reduction() {
        let mut diagnostics = Vec::new();
        let absent = derive_pipe_section(
            &mill_tolerance_section(None),
            "pipe:P-MILL",
            &mut diagnostics,
        )
        .unwrap();
        let zero = derive_pipe_section(
            &mill_tolerance_section(Some(0.0)),
            "pipe:P-MILL",
            &mut diagnostics,
        )
        .unwrap();
        assert!(diagnostics.is_empty());
        assert_eq!(absent.wall_thickness, zero.wall_thickness);
        assert_eq!(absent.section_modulus, zero.section_modulus);
    }

    #[test]
    fn present_but_invalid_mill_tolerance_is_blocking() {
        for invalid in [-0.001, 0.01, f64::NAN] {
            let mut diagnostics = Vec::new();
            let derived = derive_pipe_section(
                &mill_tolerance_section(Some(invalid)),
                "pipe:P-MILL",
                &mut diagnostics,
            );
            assert!(derived.is_none());
            assert!(diagnostics.iter().any(|item| {
                item.code == "PIPE_DIMENSION_INVALID"
                    && item.severity == "blocking"
                    && item.affected_refs.contains(&"mill_tolerance".to_string())
            }));
        }
    }

    #[test]
    fn mill_tolerance_units_are_normalized_at_preview_boundary() {
        let mut base = request();
        for pipe in &mut base.model.pipe_segments {
            pipe.section.mill_tolerance = Some(Quantity {
                value: 0.00125,
                unit: "m".to_string(),
            });
        }
        let mut millimeters = request();
        for pipe in &mut millimeters.model.pipe_segments {
            pipe.section.mill_tolerance = Some(Quantity {
                value: 1.25,
                unit: "mm".to_string(),
            });
        }

        let base_result = run_linear_static_preview(base);
        let millimeter_result = run_linear_static_preview(millimeters);
        assert_eq!(base_result.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(millimeter_result.status.mechanics, "MECHANICS_SOLVED");
        for (left, right) in base_result
            .results
            .iter()
            .zip(millimeter_result.results.iter())
        {
            assert_eq!(left.id, right.id);
            assert_eq!(left.value, right.value);
        }
    }

    fn equivalent_static_base_request() -> LinearStaticPreviewRequest {
        let mut request = request();
        request.model.nodes.truncate(2);
        request.model.nodes[0].id = "node:N-100".to_string();
        request.model.nodes[0].position = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        request.model.nodes[1].id = "node:N-110".to_string();
        request.model.nodes[1].position = Vec3 {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        };
        request.model.pipe_segments.truncate(1);
        request.model.pipe_segments[0].id = "pipe:P-100".to_string();
        request.model.pipe_segments[0].from = "node:N-100".to_string();
        request.model.pipe_segments[0].to = "node:N-110".to_string();
        request.model.pipe_segments[0].y_reference = Some(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });
        request.model.pipe_segments[0].section = PipeSectionInput {
            outside_diameter: Quantity {
                value: 0.2,
                unit: "m".to_string(),
            },
            wall_thickness: Quantity {
                value: 0.01,
                unit: "m".to_string(),
            },
            mill_tolerance: None,
            material_density: Some(Quantity {
                value: 7850.0,
                unit: "kg/m^3".to_string(),
            }),
            contents_density: None,
            insulation_thickness: None,
            insulation_density: None,
        };
        request.model.supports.truncate(2);
        request.model.supports[0].id = "support:S-100".to_string();
        request.model.supports[0].node = "node:N-100".to_string();
        request.model.supports[0].restraints = vec![
            "UX".to_string(),
            "UY".to_string(),
            "UZ".to_string(),
            "RX".to_string(),
            "RY".to_string(),
            "RZ".to_string(),
        ];
        request.model.supports[1].id = "support:S-110".to_string();
        request.model.supports[1].node = "node:N-110".to_string();
        request.model.supports[1].restraints = vec!["UY".to_string(), "UZ".to_string()];
        request.model.load_cases.truncate(1);
        request.model.load_cases[0].id = "load_case:LC-EQ".to_string();
        request.model.load_cases[0].primitive_loads = Vec::new();
        request.model.load_cases[0].equivalent_static = None;
        request.model.combinations.clear();
        request
    }

    fn expected_single_pipe_mass_per_length() -> f64 {
        let od: f64 = 0.2;
        let id: f64 = 0.2 - 2.0 * 0.01;
        PI / 4.0 * (od.powi(2) - id.powi(2)) * 7850.0
    }

    #[test]
    fn seismic_equivalent_static_generation_matches_authored_distributed_load() {
        let mut generated = equivalent_static_base_request();
        generated.model.load_cases[0].equivalent_static = Some(EquivalentStaticGenerationInput {
            seismic: Some(SeismicGenerationInput {
                gravity_acceleration: Some(Quantity {
                    value: 9.80665,
                    unit: "m/s^2".to_string(),
                }),
                g_factor_x: None,
                g_factor_y: Some(Quantity {
                    value: 0.3,
                    unit: "1".to_string(),
                }),
                g_factor_z: None,
            }),
            wind: None,
            provenance: Some("invented_example_user_input".to_string()),
        });

        let intensity = expected_single_pipe_mass_per_length() * 0.3 * 9.80665;
        let mut authored = equivalent_static_base_request();
        authored.model.load_cases[0].primitive_loads = vec![PreviewPrimitiveLoad {
            id: "load:L-SEISMIC-AUTHORED".to_string(),
            category: "seismic".to_string(),
            target: LoadTargetInput::Element {
                pipe: "pipe:P-100".to_string(),
            },
            direction: "global_y".to_string(),
            magnitude: Quantity {
                value: intensity,
                unit: "N/m".to_string(),
            },
            dimension: "force_per_length".to_string(),
            provenance: Some("invented_example_user_input".to_string()),
        }];

        let generated_result = run_linear_static_preview(generated);
        let authored_result = run_linear_static_preview(authored);

        assert_eq!(generated_result.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(authored_result.status.mechanics, "MECHANICS_SOLVED");
        for (left, right) in generated_result
            .results
            .iter()
            .zip(authored_result.results.iter())
        {
            assert_eq!(left.id, right.id);
            assert!(
                (left.value - right.value).abs() <= 1.0e-9 * right.value.abs().max(1.0),
                "{}: {} != {}",
                left.id,
                left.value,
                right.value
            );
        }
    }

    #[test]
    fn wind_equivalent_static_generation_matches_authored_distributed_load() {
        let mut generated = equivalent_static_base_request();
        generated.model.pipe_segments[0]
            .section
            .insulation_thickness = Some(Quantity {
            value: 0.025,
            unit: "m".to_string(),
        });
        generated.model.pipe_segments[0].section.insulation_density = Some(Quantity {
            value: 120.0,
            unit: "kg/m^3".to_string(),
        });
        generated.model.load_cases[0].equivalent_static = Some(EquivalentStaticGenerationInput {
            seismic: None,
            wind: Some(WindGenerationInput {
                pressure: Some(Quantity {
                    value: 480.0,
                    unit: "Pa".to_string(),
                }),
                shape_factor: Some(Quantity {
                    value: 0.7,
                    unit: "1".to_string(),
                }),
                direction: Some("global_z".to_string()),
                exposed_pipe_refs: vec!["pipe:P-100".to_string()],
            }),
            provenance: Some("invented_example_user_input".to_string()),
        });

        let exposed_diameter = 0.2 + 2.0 * 0.025;
        let intensity = 480.0 * 0.7 * exposed_diameter;
        let mut authored = equivalent_static_base_request();
        authored.model.pipe_segments[0].section.insulation_thickness = Some(Quantity {
            value: 0.025,
            unit: "m".to_string(),
        });
        authored.model.pipe_segments[0].section.insulation_density = Some(Quantity {
            value: 120.0,
            unit: "kg/m^3".to_string(),
        });
        authored.model.load_cases[0].primitive_loads = vec![PreviewPrimitiveLoad {
            id: "load:L-WIND-AUTHORED".to_string(),
            category: "wind".to_string(),
            target: LoadTargetInput::Element {
                pipe: "pipe:P-100".to_string(),
            },
            direction: "global_z".to_string(),
            magnitude: Quantity {
                value: intensity,
                unit: "N/m".to_string(),
            },
            dimension: "force_per_length".to_string(),
            provenance: Some("invented_example_user_input".to_string()),
        }];

        let generated_result = run_linear_static_preview(generated);
        let authored_result = run_linear_static_preview(authored);

        assert_eq!(generated_result.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(authored_result.status.mechanics, "MECHANICS_SOLVED");
        for (left, right) in generated_result
            .results
            .iter()
            .zip(authored_result.results.iter())
        {
            assert_eq!(left.id, right.id);
            assert!(
                (left.value - right.value).abs() <= 1.0e-9 * right.value.abs().max(1.0),
                "{}: {} != {}",
                left.id,
                left.value,
                right.value
            );
        }
    }

    #[test]
    fn seismic_generation_without_material_density_is_blocking() {
        let mut request = equivalent_static_base_request();
        request.model.pipe_segments[0].section.material_density = None;
        request.model.load_cases[0].equivalent_static = Some(EquivalentStaticGenerationInput {
            seismic: Some(SeismicGenerationInput {
                gravity_acceleration: Some(Quantity {
                    value: 9.80665,
                    unit: "m/s^2".to_string(),
                }),
                g_factor_x: None,
                g_factor_y: Some(Quantity {
                    value: 0.3,
                    unit: "1".to_string(),
                }),
                g_factor_z: None,
            }),
            wind: None,
            provenance: Some("invented_example_user_input".to_string()),
        });

        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "EQUIVALENT_STATIC_INPUT_MISSING"
                && item.severity == "blocking"));
        assert!(result.results.is_empty());
    }

    #[test]
    fn seismic_generation_without_user_gravity_is_blocking_not_defaulted() {
        let mut request = equivalent_static_base_request();
        request.model.load_cases[0].equivalent_static = Some(EquivalentStaticGenerationInput {
            seismic: Some(SeismicGenerationInput {
                gravity_acceleration: None,
                g_factor_x: Some(Quantity {
                    value: 0.2,
                    unit: "1".to_string(),
                }),
                g_factor_y: None,
                g_factor_z: None,
            }),
            wind: None,
            provenance: Some("invented_example_user_input".to_string()),
        });

        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "EQUIVALENT_STATIC_INPUT_MISSING"
                && item.message.contains("gravity_acceleration")));
    }

    #[test]
    fn wind_generation_without_marked_spans_is_blocking() {
        let mut request = equivalent_static_base_request();
        request.model.load_cases[0].equivalent_static = Some(EquivalentStaticGenerationInput {
            seismic: None,
            wind: Some(WindGenerationInput {
                pressure: Some(Quantity {
                    value: 480.0,
                    unit: "Pa".to_string(),
                }),
                shape_factor: Some(Quantity {
                    value: 0.7,
                    unit: "1".to_string(),
                }),
                direction: Some("global_z".to_string()),
                exposed_pipe_refs: Vec::new(),
            }),
            provenance: Some("invented_example_user_input".to_string()),
        });

        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "EQUIVALENT_STATIC_INPUT_MISSING"
                && item.message.contains("exposed_pipe_refs")));
    }

    #[test]
    fn seismic_generation_with_partial_insulation_pair_is_blocking() {
        let mut request = equivalent_static_base_request();
        request.model.pipe_segments[0].section.insulation_thickness = Some(Quantity {
            value: 0.025,
            unit: "m".to_string(),
        });
        request.model.load_cases[0].equivalent_static = Some(EquivalentStaticGenerationInput {
            seismic: Some(SeismicGenerationInput {
                gravity_acceleration: Some(Quantity {
                    value: 9.80665,
                    unit: "m/s^2".to_string(),
                }),
                g_factor_x: None,
                g_factor_y: Some(Quantity {
                    value: 0.3,
                    unit: "1".to_string(),
                }),
                g_factor_z: None,
            }),
            wind: None,
            provenance: Some("invented_example_user_input".to_string()),
        });

        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "EQUIVALENT_STATIC_INPUT_MISSING"
                && item.message.contains("insulation")));
    }

    fn hot_basis_material() -> MaterialInput {
        let mut material = invented_materials().remove(0);
        material.temperature_points = vec![MaterialTemperaturePointInput {
            id: "temperature-point:hot".to_string(),
            temperature: Some(Quantity {
                value: 533.15,
                unit: "K".to_string(),
            }),
            elastic_modulus: Some(Quantity {
                value: 180_000_000_000.0,
                unit: "Pa".to_string(),
            }),
            thermal_expansion_coefficient: Some(Quantity {
                value: 0.000013,
                unit: "1/K".to_string(),
            }),
            provenance: Some("invented_example_user_input".to_string()),
        }];
        material
    }

    fn interpolation_basis_material() -> MaterialInput {
        let mut material = invented_materials().remove(0);
        material.temperature_points = vec![
            MaterialTemperaturePointInput {
                id: "temperature-point:cold".to_string(),
                temperature: Some(Quantity {
                    value: 300.0,
                    unit: "K".to_string(),
                }),
                elastic_modulus: Some(Quantity {
                    value: 200_000_000_000.0,
                    unit: "Pa".to_string(),
                }),
                thermal_expansion_coefficient: Some(Quantity {
                    value: 0.000012,
                    unit: "1/K".to_string(),
                }),
                provenance: Some("invented_cold_user_input".to_string()),
            },
            MaterialTemperaturePointInput {
                id: "temperature-point:hot".to_string(),
                temperature: Some(Quantity {
                    value: 500.0,
                    unit: "K".to_string(),
                }),
                elastic_modulus: Some(Quantity {
                    value: 180_000_000_000.0,
                    unit: "Pa".to_string(),
                }),
                thermal_expansion_coefficient: Some(Quantity {
                    value: 0.000014,
                    unit: "1/K".to_string(),
                }),
                provenance: Some("invented_hot_user_input".to_string()),
            },
        ];
        material
    }

    #[test]
    fn load_case_modulus_basis_selects_exact_user_entered_hot_point() {
        let mut request = fixed_fixed_thermal_request("global_z");
        request.materials = vec![hot_basis_material()];
        request.model.load_cases[0].modulus_basis_ref = Some("temperature-point:hot".to_string());
        let area = derive_pipe_section(
            &request.model.pipe_segments[0].section,
            "pipe:P-100",
            &mut Vec::new(),
        )
        .unwrap()
        .area;

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        let expected_force = 180_000_000_000.0 * area * 0.000013 * 10.0;
        let axial = result
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:axial")
            .unwrap();
        assert!((axial.value - round6(expected_force)).abs() < 1.0e-6);

        let record = result
            .results
            .iter()
            .find(|item| item.kind == "modulus_basis_record")
            .expect("modulus basis record row");
        assert_eq!(record.entity_ref, "load:L-100");
        let metadata = record.metadata.as_ref().unwrap();
        assert!(metadata
            .basis
            .contains("temperature_point:temperature-point:hot"));
        assert!(metadata.basis.contains("interpolation=not_performed"));
    }

    #[test]
    fn declared_solve_temperature_interpolates_e_and_alpha_with_provenance() {
        let mut request = fixed_fixed_thermal_request("global_z");
        request.materials = vec![interpolation_basis_material()];
        request.model.load_cases[0].modulus_basis_temperature = Some(Quantity {
            value: 400.0,
            unit: "K".to_string(),
        });
        let area = derive_pipe_section(
            &request.model.pipe_segments[0].section,
            "pipe:P-100",
            &mut Vec::new(),
        )
        .unwrap()
        .area;

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        let expected_force = 190_000_000_000.0 * area * 0.000013 * 10.0;
        let axial = result
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:axial")
            .unwrap();
        assert!((axial.value - round6(expected_force)).abs() < 1.0e-6);

        let record = result
            .results
            .iter()
            .find(|item| item.kind == "modulus_basis_record")
            .expect("interpolated modulus basis record row");
        let basis = &record.metadata.as_ref().unwrap().basis;
        assert!(basis.contains("method=linear_temperature_interpolation"));
        assert!(
            basis.contains("elastic_modulus_sources=temperature-point:cold,temperature-point:hot")
        );
        assert!(basis.contains(
            "thermal_expansion_coefficient_sources=temperature-point:cold,temperature-point:hot"
        ));
        assert!(basis.contains("solve_temperature_kelvin=400"));
    }

    #[test]
    fn interpolation_blocks_at_and_beyond_stored_range_edges() {
        for solve_temperature in [250.0, 300.0, 500.0, 550.0] {
            let mut request = fixed_fixed_thermal_request("global_z");
            request.materials = vec![interpolation_basis_material()];
            request.model.load_cases[0].modulus_basis_temperature = Some(Quantity {
                value: solve_temperature,
                unit: "K".to_string(),
            });

            let result = run_linear_static_preview(request);

            assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
            assert!(result.diagnostics.iter().any(|item| {
                item.code == "MODULUS_BASIS_UNRESOLVED"
                    && item.severity == "blocking"
                    && item.message.contains("never extrapolates")
            }));
            assert!(result.results.is_empty());
        }
    }

    #[test]
    fn exact_and_interpolated_basis_fields_are_mutually_exclusive() {
        let mut request = fixed_fixed_thermal_request("global_z");
        request.materials = vec![interpolation_basis_material()];
        request.model.load_cases[0].modulus_basis_ref = Some("temperature-point:hot".to_string());
        request.model.load_cases[0].modulus_basis_temperature = Some(Quantity {
            value: 400.0,
            unit: "K".to_string(),
        });

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result.diagnostics.iter().any(|item| {
            item.code == "MODULUS_BASIS_SELECTION_CONFLICT" && item.severity == "blocking"
        }));
        assert!(result.results.is_empty());
    }

    #[test]
    fn base_material_values_are_used_when_no_modulus_basis_is_named() {
        let mut request = fixed_fixed_thermal_request("global_z");
        request.materials = vec![hot_basis_material()];
        let area = derive_pipe_section(
            &request.model.pipe_segments[0].section,
            "pipe:P-100",
            &mut Vec::new(),
        )
        .unwrap()
        .area;

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        let expected_force = 200_000_000_000.0 * area * 0.000012 * 10.0;
        let axial = result
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:axial")
            .unwrap();
        assert!((axial.value - round6(expected_force)).abs() < 1.0e-6);
        assert!(!result
            .results
            .iter()
            .any(|item| item.kind == "modulus_basis_record"));
    }

    #[test]
    fn unresolved_exact_modulus_basis_is_blocking_without_defaulting() {
        let mut request = fixed_fixed_thermal_request("global_z");
        request.materials = vec![hot_basis_material()];
        request.model.load_cases[0].modulus_basis_ref =
            Some("temperature-point:between".to_string());

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result.diagnostics.iter().any(|item| {
            item.code == "MODULUS_BASIS_UNRESOLVED"
                && item.severity == "blocking"
                && item
                    .message
                    .contains("exact-id selection remains available")
                && item.message.contains("no value is defaulted")
        }));
        assert!(result.results.is_empty());
    }

    #[test]
    fn modulus_basis_point_without_elastic_modulus_is_blocking() {
        let mut request = fixed_fixed_thermal_request("global_z");
        let mut material = hot_basis_material();
        material.temperature_points[0].elastic_modulus = None;
        request.materials = vec![material];
        request.model.load_cases[0].modulus_basis_ref = Some("temperature-point:hot".to_string());

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "MODULUS_BASIS_INPUT_MISSING" && item.severity == "blocking"));
    }

    #[test]
    fn range_combination_records_each_operand_modulus_basis() {
        let mut request = request();
        request.materials = vec![hot_basis_material()];
        request.model.load_cases[1].modulus_basis_ref = Some("temperature-point:hot".to_string());
        request.model.combinations = vec![range_combination(
            "combination:C-RANGE-BASIS",
            &["load:L-100", "load:L-200"],
            "max_abs",
        )];

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        let records = result
            .results
            .iter()
            .filter(|item| item.kind == "combination_modulus_basis_record")
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 2);
        let base_record = records
            .iter()
            .find(|item| item.entity_ref == "load:L-100")
            .unwrap();
        assert_eq!(
            base_record.metadata.as_ref().unwrap().basis,
            "material_base_values"
        );
        let hot_record = records
            .iter()
            .find(|item| item.entity_ref == "load:L-200")
            .unwrap();
        assert!(hot_record
            .metadata
            .as_ref()
            .unwrap()
            .basis
            .contains("temperature_point:temperature-point:hot"));
        assert!(records.iter().all(|item| {
            item.basis_ref
                == Some(ResultBasisRef {
                    ref_type: "combination".to_string(),
                    ref_id: "combination:C-RANGE-BASIS".to_string(),
                })
        }));
    }

    #[test]
    fn thermal_load_direction_does_not_change_thermal_magnitude_or_sign() {
        let global = run_linear_static_preview(fixed_fixed_thermal_request("global_z"));
        let legacy = run_linear_static_preview(fixed_fixed_thermal_request("RZ"));
        let global_axial = global
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:axial")
            .unwrap()
            .value;
        let legacy_axial = legacy
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:axial")
            .unwrap()
            .value;

        assert_eq!(global.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(legacy.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(global_axial, legacy_axial);
    }

    #[test]
    fn pressure_thrust_applies_axial_fixed_end_correction_without_longitudinal_rows() {
        let request = fixed_fixed_pressure_request("global_z");
        let section = derive_pipe_section(
            &request.model.pipe_segments[0].section,
            "pipe:P-100",
            &mut Vec::new(),
        )
        .unwrap();
        let expected_force = 1_000_000.0 * section.internal_area;
        let result = run_linear_static_preview(request);
        let result_ids = result
            .results
            .iter()
            .map(|item| item.id.as_str())
            .collect::<HashSet<_>>();
        let axial_i = result
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:axial")
            .unwrap();
        let axial_j = result
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:axial:end-j")
            .unwrap();
        let stress_i = result
            .results
            .iter()
            .find(|item| item.id == "result:stress:pipe-P-100:end-i:axial-normal")
            .unwrap();

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!((axial_i.value - round6(expected_force)).abs() < 1.0e-6);
        assert!((axial_j.value + round6(expected_force)).abs() < 1.0e-6);
        assert!(
            (stress_i.value - round6(expected_force / section.area / 1_000_000.0)).abs() < 1.0e-6
        );
        assert!(result_ids.contains("result:stress:pipe-P-100:end-i:pressure-hoop"));
        assert!(!result_ids.contains("result:stress:pipe-P-100:end-i:pressure-longitudinal"));
        assert!(result_ids.contains("result:stress:pipe-P-100"));
        assert!(!result
            .diagnostics
            .iter()
            .any(|item| item.code == "PRESSURE_LOAD_NOT_APPLIED_TO_FRAME_VECTOR"));
    }

    #[test]
    fn pressure_load_direction_does_not_change_thrust_magnitude_or_sign() {
        let global = run_linear_static_preview(fixed_fixed_pressure_request("global_z"));
        let legacy = run_linear_static_preview(fixed_fixed_pressure_request("RZ"));
        let global_axial = global
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:axial")
            .unwrap()
            .value;
        let legacy_axial = legacy
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:axial")
            .unwrap()
            .value;

        assert_eq!(global.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(legacy.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(global_axial, legacy_axial);
    }

    #[test]
    fn thermal_load_requires_explicit_material_expansion_coefficient() {
        let mut request = request();
        request.materials[0].thermal_expansion_coefficient = None;

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "THERMAL_EXPANSION_INPUT_MISSING"));
        assert!(result.results.is_empty());
    }

    #[test]
    fn generated_result_surface_matches_fallback_fixture_force_metadata() {
        let generated = serde_json::to_value(run_linear_static_preview(request())).unwrap();
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/invented_mechanics_result.json"
        ))
        .unwrap();
        let generated_force = find_result(&generated, "result:force:pipe-P-120:axial");
        let fixture_force = find_result(&fixture, "result:force:pipe-P-120:axial");

        assert_eq!(generated_force["kind"], fixture_force["kind"]);
        assert_eq!(generated_force["unit"], fixture_force["unit"]);
        assert_eq!(generated_force["metadata"], fixture_force["metadata"]);
        let fixture_force_end_j = find_result(&fixture, "result:force:pipe-P-120:axial:end-j");
        assert_eq!(fixture_force_end_j["metadata"]["location"], "end_j");
        let fixture_force_midspan = find_result(&fixture, "result:force:pipe-P-120:midspan:axial");
        assert_eq!(fixture_force_midspan["metadata"]["location"], "midspan");
        assert_eq!(
            fixture_force_midspan["metadata"]["basis"],
            "interpolated_from_endpoint_resultants"
        );
        let fixture_force_quarter =
            find_result(&fixture, "result:force:pipe-P-120:quarter-1:shear-y");
        assert_eq!(fixture_force_quarter["kind"], "element_local_shear_force_y");
        assert_eq!(
            fixture_force_quarter["metadata"]["component"],
            "shear_force_y"
        );
        assert_eq!(fixture_force_quarter["metadata"]["location"], "quarter_1");
        let fixture_stress_quarter = find_result(
            &fixture,
            "result:stress:pipe-P-120:quarter-1:torsional-shear",
        );
        assert_eq!(
            fixture_stress_quarter["metadata"]["basis"],
            "interpolated_from_endpoint_resultants"
        );
        assert!(find_result(&fixture, "result:moment:pipe-P-120:bending-z")
            .get("metadata")
            .is_some());
        let fixture_stress_end_j =
            find_result(&fixture, "result:stress:pipe-P-120:end-j:torsional-shear");
        assert_eq!(fixture_stress_end_j["metadata"]["location"], "end_j");
        assert_eq!(
            fixture_stress_end_j["metadata"]["basis"],
            "recovered_from_open_mechanics_stress_components"
        );
        let generated_disp_uy = find_result(&generated, "result:disp:node-N-140:uy");
        let fixture_disp_uy = find_result(&fixture, "result:disp:node-N-140:uy");
        assert_eq!(generated_disp_uy["kind"], fixture_disp_uy["kind"]);
        assert_eq!(generated_disp_uy["unit"], fixture_disp_uy["unit"]);
        assert_eq!(generated_disp_uy["value"], fixture_disp_uy["value"]);
        assert_eq!(generated_disp_uy["metadata"], fixture_disp_uy["metadata"]);
        assert_eq!(fixture_disp_uy["metadata"]["coordinate_system"], "global");
        let fixture_disp_rz = find_result(&fixture, "result:disp:node-N-140:rz");
        assert_eq!(fixture_disp_rz["kind"], "global_nodal_rotation_z");
        assert_eq!(fixture_disp_rz["unit"], "rad");
    }

    #[test]
    fn missing_material_blocks_with_diagnostic() {
        let mut request = request();
        request.materials.clear();
        request.model.materials.clear();

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "MATERIAL_INPUT_MISSING"));
        assert!(result.results.is_empty());
    }

    #[test]
    fn missing_load_input_blocks_with_diagnostic() {
        let mut request = request();
        request.model.load_cases[0].primitive_loads[0]
            .magnitude
            .value = f64::NAN;

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "LOAD_MAGNITUDE_INVALID"));
    }

    #[test]
    fn missing_all_primitive_loads_blocks_with_diagnostic() {
        let mut request = request();
        for case in &mut request.model.load_cases {
            case.primitive_loads.clear();
        }

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "LOAD_INPUT_MISSING"));
        assert!(result.results.is_empty());
    }

    #[test]
    fn missing_pipe_orientation_blocks_with_diagnostic() {
        let mut request = request();
        request.model.pipe_segments[0].y_reference = None;

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "PIPE_ORIENTATION_INPUT_MISSING"));
        assert!(result.results.is_empty());
    }

    #[test]
    fn duplicate_ids_block_with_diagnostic() {
        let mut request = request();
        request.model.nodes[1].id = request.model.nodes[0].id.clone();

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "DUPLICATE_ID"));
        assert!(result.results.is_empty());
    }

    #[test]
    fn empty_ids_block_with_diagnostic() {
        let mut request = request();
        request.model.pipe_segments[0].id.clear();

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "EMPTY_ID"));
        assert!(result.results.is_empty());
    }

    #[test]
    fn invalid_combination_records_block_with_diagnostics() {
        let mut request = request();
        request.model.combinations[0]
            .terms
            .push(PreviewCombinationTerm {
                load_case: "load:missing".to_string(),
                factor: f64::NAN,
            });
        request.model.combinations.push(PreviewCombination {
            id: "".to_string(),
            label: None,
            basis: "mechanics".to_string(),
            terms: Vec::new(),
            minuend_id: None,
            subtrahend_id: None,
            operand_ids: None,
            mode: None,
            provenance: Some("invented_example_user_defined_combination".to_string()),
        });
        request.model.combinations.push(PreviewCombination {
            id: "combination:C-OWNER".to_string(),
            label: None,
            basis: "owner_design_basis".to_string(),
            terms: Vec::new(),
            minuend_id: None,
            subtrahend_id: None,
            operand_ids: None,
            mode: None,
            provenance: Some("invented_example_user_defined_combination".to_string()),
        });

        let result = run_linear_static_preview(request);
        let codes = result
            .diagnostics
            .iter()
            .map(|item| item.code.as_str())
            .collect::<HashSet<_>>();

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(codes.contains("LOAD_COMBINATION_BASIS_UNSUPPORTED"));
        assert!(codes.contains("LOAD_COMBINATION_LOAD_CASE_UNKNOWN"));
        assert!(codes.contains("LOAD_COMBINATION_FACTOR_INVALID"));
        assert!(codes.contains("LOAD_COMBINATION_TERMS_EMPTY"));
        assert!(codes.contains("EMPTY_ID"));
        assert!(result.results.is_empty());
    }

    fn subtraction_combination(id: &str, minuend: &str, subtrahend: &str) -> PreviewCombination {
        PreviewCombination {
            id: id.to_string(),
            label: Some("Invented subtraction preview".to_string()),
            basis: "result_state_subtraction".to_string(),
            terms: Vec::new(),
            minuend_id: Some(minuend.to_string()),
            subtrahend_id: Some(subtrahend.to_string()),
            operand_ids: None,
            mode: None,
            provenance: Some(
                "invented_example_user_defined_subtraction_no_code_default".to_string(),
            ),
        }
    }

    fn range_combination(id: &str, operand_ids: &[&str], mode: &str) -> PreviewCombination {
        PreviewCombination {
            id: id.to_string(),
            label: Some("Invented range envelope preview".to_string()),
            basis: "range_envelope".to_string(),
            terms: Vec::new(),
            minuend_id: None,
            subtrahend_id: None,
            operand_ids: Some(operand_ids.iter().map(|id| id.to_string()).collect()),
            mode: Some(mode.to_string()),
            provenance: Some(
                "invented_example_user_defined_range_envelope_no_code_default".to_string(),
            ),
        }
    }

    #[test]
    fn subtraction_combination_subtracts_solved_rows_with_signed_determinism() {
        let mut request = request();
        request.model.combinations = vec![
            subtraction_combination("combination:C-SUB", "load:L-100", "load:L-200"),
            subtraction_combination("combination:C-SUB-REV", "load:L-200", "load:L-100"),
        ];

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        let base = result_value(&result, "result:disp:node-N-130:uz");
        let alternate = result_value(&result, "result:loadcase:load-L-200:disp:node-N-130:uz");
        assert_ne!(
            base, alternate,
            "fixture load cases must differ at node N-130 uz"
        );
        let combination = result
            .results
            .iter()
            .find(|item| item.id == "result:combination:combination-C-SUB:disp:node-N-130:uz")
            .expect("subtraction combination row should be emitted");
        assert_eq!(combination.value, round6(base - alternate));
        assert_eq!(
            combination
                .basis_ref
                .as_ref()
                .map(|basis| basis.ref_id.as_str()),
            Some("combination:C-SUB")
        );
        assert_eq!(
            combination.source_result_refs,
            vec![
                "result:disp:node-N-130:uz".to_string(),
                "result:loadcase:load-L-200:disp:node-N-130:uz".to_string(),
            ]
        );
        assert_eq!(
            combination
                .metadata
                .as_ref()
                .map(|metadata| metadata.basis.as_str()),
            Some("explicit_user_result_state_subtraction")
        );
        let reversed = result_value(
            &result,
            "result:combination:combination-C-SUB-REV:disp:node-N-130:uz",
        );
        assert_eq!(reversed, round6(alternate - base));
        assert_eq!(combination.value, -reversed);
    }

    #[test]
    fn range_envelope_combination_selects_each_shipped_mode_deterministically() {
        let mut request = request();
        request.model.combinations = vec![
            range_combination("combination:C-MIN", &["load:L-100", "load:L-200"], "min"),
            range_combination("combination:C-MAX", &["load:L-100", "load:L-200"], "max"),
            range_combination(
                "combination:C-MINABS",
                &["load:L-100", "load:L-200"],
                "min_abs",
            ),
            range_combination(
                "combination:C-MAXABS",
                &["load:L-100", "load:L-200"],
                "max_abs",
            ),
        ];

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        let base = result_value(&result, "result:disp:node-N-130:uz");
        let alternate = result_value(&result, "result:loadcase:load-L-200:disp:node-N-130:uz");
        assert_ne!(
            base.abs(),
            alternate.abs(),
            "fixture load cases must produce distinct-magnitude rows for mode coverage"
        );
        let row_tail = "disp:node-N-130:uz";
        assert_eq!(
            result_value(
                &result,
                &format!("result:combination:combination-C-MIN:{row_tail}")
            ),
            if base <= alternate { base } else { alternate }
        );
        assert_eq!(
            result_value(
                &result,
                &format!("result:combination:combination-C-MAX:{row_tail}")
            ),
            if base >= alternate { base } else { alternate }
        );
        assert_eq!(
            result_value(
                &result,
                &format!("result:combination:combination-C-MINABS:{row_tail}")
            ),
            if base.abs() <= alternate.abs() {
                base
            } else {
                alternate
            }
        );
        let max_abs = result
            .results
            .iter()
            .find(|item| item.id == format!("result:combination:combination-C-MAXABS:{row_tail}"))
            .expect("max_abs combination row should be emitted");
        assert_eq!(
            max_abs.value,
            if base.abs() >= alternate.abs() {
                base
            } else {
                alternate
            }
        );
        assert_eq!(
            max_abs.source_result_refs,
            vec![
                "result:disp:node-N-130:uz".to_string(),
                "result:loadcase:load-L-200:disp:node-N-130:uz".to_string(),
            ]
        );
        assert_eq!(
            max_abs
                .metadata
                .as_ref()
                .map(|metadata| metadata.basis.as_str()),
            Some("explicit_user_range_envelope")
        );
        assert!(max_abs
            .metadata
            .as_ref()
            .is_some_and(|metadata| metadata.sign_convention.contains("max_abs")));
    }

    #[test]
    fn invalid_subtraction_and_range_records_block_with_named_diagnostics() {
        let mut request = request();
        let mut mechanics_with_mode = request.model.combinations[0].clone();
        mechanics_with_mode.id = "combination:C-SHAPE".to_string();
        mechanics_with_mode.mode = Some("max".to_string());
        let mut subtraction_with_terms =
            subtraction_combination("combination:C-SUB-TERMS", "load:L-100", "load:L-200");
        subtraction_with_terms.terms = vec![PreviewCombinationTerm {
            load_case: "load:L-100".to_string(),
            factor: 1.0,
        }];
        request.model.combinations = vec![
            mechanics_with_mode,
            subtraction_with_terms,
            subtraction_combination("combination:C-SUB-SELF", "load:L-100", "load:L-100"),
            subtraction_combination("combination:C-SUB-MISSING", "load:L-100", "load:missing"),
            range_combination(
                "combination:C-RANGE-MODE",
                &["load:L-100", "load:L-200"],
                "largest",
            ),
            range_combination(
                "combination:C-RANGE-DUP",
                &["load:L-100", "load:L-100"],
                "max",
            ),
            range_combination("combination:C-RANGE-EMPTY", &[], "max"),
        ];

        let result = run_linear_static_preview(request);
        let codes = result
            .diagnostics
            .iter()
            .map(|item| item.code.as_str())
            .collect::<HashSet<_>>();

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(codes.contains("LOAD_COMBINATION_SHAPE_INVALID"));
        assert!(codes.contains("LOAD_COMBINATION_DUPLICATE_TERM"));
        assert!(codes.contains("LOAD_COMBINATION_LOAD_CASE_UNKNOWN"));
        assert!(codes.contains("LOAD_COMBINATION_RANGE_MODE_UNKNOWN"));
        assert!(codes.contains("LOAD_COMBINATION_OPERANDS_EMPTY"));
        assert!(result.diagnostics.iter().any(|item| item.code
            == "LOAD_COMBINATION_SHAPE_INVALID"
            && item
                .affected_refs
                .contains(&"combination:C-SHAPE".to_string())));
        assert!(result.diagnostics.iter().any(|item| item.code
            == "LOAD_COMBINATION_SHAPE_INVALID"
            && item
                .affected_refs
                .contains(&"combination:C-SUB-TERMS".to_string())));
        assert!(result.diagnostics.iter().any(|item| item.code
            == "LOAD_COMBINATION_DUPLICATE_TERM"
            && item
                .affected_refs
                .contains(&"combination:C-SUB-SELF".to_string())));
        assert!(result.results.is_empty());
    }

    #[test]
    fn missing_public_preview_provenance_blocks_with_diagnostic() {
        let mut request = request();
        request.model.load_cases[0].primitive_loads[0].provenance = None;

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "PROVENANCE_INPUT_MISSING"));
        assert!(result.results.is_empty());
    }

    #[test]
    fn mixed_units_are_normalized_at_preview_mechanics_boundary() {
        let baseline = run_linear_static_preview(request());
        let mut request = request();
        request.materials[0].elastic_modulus = Quantity {
            value: 200_000.0,
            unit: "MPa".to_string(),
        };
        request.materials[0].shear_modulus = Quantity {
            value: 77_000.0,
            unit: "MPa".to_string(),
        };
        for pipe in &mut request.model.pipe_segments {
            pipe.section.outside_diameter = Quantity {
                value: 168.0,
                unit: "mm".to_string(),
            };
            pipe.section.wall_thickness = Quantity {
                value: 7.0,
                unit: "mm".to_string(),
            };
        }
        for load in request
            .model
            .load_cases
            .iter_mut()
            .flat_map(|case| case.primitive_loads.iter_mut())
            .filter(|load| load.dimension == "pressure")
        {
            load.magnitude.value /= 1_000.0;
            load.magnitude.unit = "kPa".to_string();
        }

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(result
            .diagnostics
            .iter()
            .all(|item| item.code != "UNIT_INPUT_INVALID"));
        assert_eq!(result.results.len(), baseline.results.len());
        assert_eq!(
            result
                .summary
                .max_displacement
                .as_ref()
                .map(|item| item.value),
            baseline
                .summary
                .max_displacement
                .as_ref()
                .map(|item| item.value)
        );
        assert_eq!(
            result
                .summary
                .max_open_formula_stress
                .as_ref()
                .map(|item| item.value),
            baseline
                .summary
                .max_open_formula_stress
                .as_ref()
                .map(|item| item.value)
        );
        assert_eq!(
            result_value(&result, "result:stress:pipe-P-120:end-i:pressure-hoop"),
            result_value(&baseline, "result:stress:pipe-P-120:end-i:pressure-hoop")
        );
    }

    #[test]
    fn incompatible_material_unit_blocks_with_diagnostic() {
        let mut request = request();
        request.materials[0].elastic_modulus.unit = "m".to_string();

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result.diagnostics.iter().any(|item| {
            item.code == "UNIT_INPUT_INVALID"
                && item.affected_refs.contains(&"elastic_modulus".to_string())
        }));
        assert!(result.results.is_empty());
    }

    #[test]
    fn invalid_load_unit_blocks_with_diagnostic() {
        let mut request = request();
        request.model.load_cases[0].primitive_loads[0]
            .magnitude
            .unit = "kg/m".to_string();
        let load_id = request.model.load_cases[0].primitive_loads[0].id.clone();

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result.diagnostics.iter().any(|item| {
            item.code == "UNIT_INPUT_INVALID" && item.affected_refs.contains(&load_id)
        }));
        assert!(result.results.is_empty());
    }

    #[test]
    fn under_restrained_model_reports_solver_diagnostic() {
        let mut request = request();
        request.model.supports.truncate(1);
        request.model.supports[0].restraints = vec!["UZ".to_string()];

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        let diagnostic = result
            .diagnostics
            .iter()
            .find(|item| item.code == "SOLVER_SYSTEM_BLOCKED")
            .expect("under-restraint diagnostic should be present");
        assert!(diagnostic
            .message
            .contains("restrained global DOF classes: UZ"));
        assert!(diagnostic
            .message
            .contains("missing global rigid-body DOF classes: UX,UY,RX,RY,RZ"));
    }

    #[test]
    fn envelope_keeps_status_boundaries_separate() {
        let result = run_linear_static_preview(request());

        assert_eq!(result.status.rule_check, "RULE_INPUTS_INCOMPLETE");
        assert_eq!(result.status.professional_acceptance, "NOT_PROVIDED");
        assert!(result.professional_boundary.human_review_required);
        assert!(!result.professional_boundary.software_makes_compliance_claim);
        assert!(!result.accepted_model_state_mutated);
    }

    // Invented single-span curved-bend model (DEC-070): the quarter-circle arc
    // over a 2 m chord along global x with the pipe y_reference (0, 1, 0), a
    // user bend radius sqrt(2) m, user bend angle pi/2, and an invented user
    // flexibility factor 2. Node N-100 is anchored; loads vary per test.
    const CURVED_BEND_TEST_CHORD_M: f64 = 2.0;
    const CURVED_BEND_TEST_RADIUS_M: f64 = std::f64::consts::SQRT_2;
    const CURVED_BEND_TEST_FLEXIBILITY: f64 = 2.0;
    const CURVED_BEND_TEST_SIF: f64 = 1.15;

    fn curved_bend_span_request() -> LinearStaticPreviewRequest {
        let mut request = request();
        request.model.nodes.truncate(2);
        request.model.nodes[0].id = "node:N-100".to_string();
        request.model.nodes[0].position = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        request.model.nodes[1].id = "node:N-110".to_string();
        request.model.nodes[1].position = Vec3 {
            x: CURVED_BEND_TEST_CHORD_M,
            y: 0.0,
            z: 0.0,
        };
        request.model.pipe_segments.truncate(1);
        request.model.pipe_segments[0].id = "pipe:P-100".to_string();
        request.model.pipe_segments[0].from = "node:N-100".to_string();
        request.model.pipe_segments[0].to = "node:N-110".to_string();
        request.model.pipe_segments[0].y_reference = Some(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });
        request.model.supports.truncate(1);
        request.model.supports[0].id = "support:S-100".to_string();
        request.model.supports[0].node = "node:N-100".to_string();
        request.model.supports[0].restraints = vec![
            "UX".to_string(),
            "UY".to_string(),
            "UZ".to_string(),
            "RX".to_string(),
            "RY".to_string(),
            "RZ".to_string(),
        ];
        request.model.components.truncate(1);
        let component = &mut request.model.components[0];
        component.node = "node:N-110".to_string();
        let geometry = component.geometry.as_mut().unwrap();
        geometry.bend_pipe_ref = Some("pipe:P-100".to_string());
        geometry.bend_radius = Some(Quantity {
            value: CURVED_BEND_TEST_RADIUS_M,
            unit: "m".to_string(),
        });
        geometry.bend_angle = Some(Quantity {
            value: PI / 2.0,
            unit: "rad".to_string(),
        });
        let modifiers = component.modifiers.as_mut().unwrap();
        modifiers.sif_user_value = Some(Quantity {
            value: CURVED_BEND_TEST_SIF,
            unit: "none".to_string(),
        });
        modifiers.flexibility_factor_user_value = Some(Quantity {
            value: CURVED_BEND_TEST_FLEXIBILITY,
            unit: "none".to_string(),
        });
        component
            .mechanics_interface
            .as_mut()
            .unwrap()
            .solver_consumption = Some(DEC_070_CURVED_BEND_SOLVER_CONSUMPTION.to_string());
        request.model.load_cases.truncate(1);
        request.model.load_cases[0].id = "load:L-100".to_string();
        request.model.load_cases[0].primitive_loads = vec![curved_bend_tip_force_load()];
        request.model.combinations.clear();
        request
    }

    fn curved_bend_tip_force_load() -> PreviewPrimitiveLoad {
        PreviewPrimitiveLoad {
            id: "load:L-100-Y".to_string(),
            category: "occasional".to_string(),
            target: LoadTargetInput::Node {
                node: "node:N-110".to_string(),
            },
            direction: "global_y".to_string(),
            magnitude: Quantity {
                value: 1000.0,
                unit: "N".to_string(),
            },
            dimension: "force".to_string(),
            provenance: Some("invented_example_user_input".to_string()),
        }
    }

    // Independent oracle: the same invented arc built directly on the
    // curved-bend crate (no product-physics assembly involved).
    fn curved_bend_direct_element() -> CurvedBendMacroElement {
        let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(1, [CURVED_BEND_TEST_CHORD_M, 0.0, 0.0]).unwrap();
        let half_chord = 0.5 * CURVED_BEND_TEST_CHORD_M;
        let sagitta_offset = (CURVED_BEND_TEST_RADIUS_M * CURVED_BEND_TEST_RADIUS_M
            - half_chord * half_chord)
            .sqrt();
        let center = [half_chord, -sagitta_offset, 0.0];
        let material = &invented_materials()[0];
        let od = 0.168_f64;
        let thickness = 0.007_f64;
        let inner = od - 2.0 * thickness;
        let area = PI * (od.powi(2) - inner.powi(2)) / 4.0;
        let second_moment = PI * (od.powi(4) - inner.powi(4)) / 64.0;
        CurvedBendMacroElement::new(
            node_i,
            node_j,
            center,
            material.elastic_modulus.value,
            material.shear_modulus.value,
            area,
            second_moment,
            2.0 * second_moment,
            CURVED_BEND_TEST_FLEXIBILITY,
            CURVED_BEND_TEST_FLEXIBILITY,
        )
        .unwrap()
    }

    // Anchored-at-i solve of the direct arc under a 12-slot global load
    // vector, returning full element displacements.
    fn curved_bend_direct_solution(force: &[f64; ELEMENT_DOF]) -> [f64; ELEMENT_DOF] {
        let element = curved_bend_direct_element();
        let stiffness = element.global_stiffness().unwrap();
        let dense: Vec<Vec<f64>> = stiffness.iter().map(|row| row.to_vec()).collect();
        let restrained: Vec<usize> = (0..DOF_PER_NODE).collect();
        let reduced = reduce_system(&dense, force.as_slice(), &restrained).unwrap();
        let solution = solve_dense(&reduced.stiffness, &reduced.force).unwrap();
        let mut displacements = [0.0; ELEMENT_DOF];
        displacements[DOF_PER_NODE..].copy_from_slice(&solution);
        displacements
    }

    fn curved_bend_direct_tip_displacements(loaded_dof: usize, magnitude: f64) -> [f64; 6] {
        let mut force = [0.0; ELEMENT_DOF];
        force[DOF_PER_NODE + loaded_dof] = magnitude;
        let displacements = curved_bend_direct_solution(&force);
        let mut tip = [0.0; DOF_PER_NODE];
        tip.copy_from_slice(&displacements[DOF_PER_NODE..]);
        tip
    }

    fn curved_bend_uniform_weight_load() -> PreviewPrimitiveLoad {
        PreviewPrimitiveLoad {
            id: "load:L-100-W".to_string(),
            category: "weight".to_string(),
            target: LoadTargetInput::Element {
                pipe: "pipe:P-100".to_string(),
            },
            direction: "global_z".to_string(),
            magnitude: Quantity {
                value: -190.0,
                unit: "N/m".to_string(),
            },
            dimension: "force_per_length".to_string(),
            provenance: Some("invented_example_user_input".to_string()),
        }
    }

    #[test]
    fn curved_bend_macro_element_assembles_arc_stiffness_without_straight_chord() {
        let result = run_linear_static_preview(curved_bend_span_request());
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");

        let expected_tip = curved_bend_direct_tip_displacements(UY, 1000.0);
        let uy_mm = result_value(&result, "result:disp:node-N-110:uy");
        assert!(
            (uy_mm - round6(expected_tip[UY] * 1000.0)).abs() <= 1.0e-6,
            "assembled arc tip displacement {uy_mm} mm must match the direct macro-element solve {} mm",
            round6(expected_tip[UY] * 1000.0)
        );
        // The straight chord of the same span is much stiffer; if the chord
        // element were (also) assembled the tip displacement would shrink.
        let straight_result = {
            let mut request = curved_bend_span_request();
            request.model.components[0]
                .mechanics_interface
                .as_mut()
                .unwrap()
                .solver_consumption = Some("mechanics_geometry_only".to_string());
            run_linear_static_preview(request)
        };
        let straight_uy_mm = result_value(&straight_result, "result:disp:node-N-110:uy");
        assert!(
            uy_mm > straight_uy_mm,
            "arc realization {uy_mm} mm must be more flexible than the straight chord {straight_uy_mm} mm"
        );

        // Force-balance sanity on the recovered chord-frame end forces: the
        // chord frame coincides with global axes here, so end i carries the
        // reaction to the 1000 N tip load.
        let shear_end_i = result_value(&result, "result:force:pipe-P-100:shear-y");
        assert!(
            (shear_end_i + 1000.0).abs() <= 1.0e-3 * 1000.0
                || (shear_end_i - 1000.0).abs() <= 1.0e-3 * 1000.0
        );
    }

    #[test]
    fn curved_bend_macro_element_keeps_dense_sparse_parity() {
        // The tip force plus the uniform arc weight exercise both the
        // assembled arc stiffness and the arc-consistent distributed-load
        // vector on the two solve lanes.
        let loaded_request = || {
            let mut loaded = curved_bend_span_request();
            loaded.model.load_cases[0]
                .primitive_loads
                .push(curved_bend_uniform_weight_load());
            loaded
        };
        let sparse = run_linear_static_preview(loaded_request());
        let dense =
            run_linear_static_preview_with_mode(loaded_request(), PreviewSolverMode::DenseScrutiny);

        assert_eq!(sparse.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(dense.status.mechanics, "MECHANICS_SOLVED");
        let parity = dense
            .results
            .iter()
            .find(|item| item.id == "result:sparse-live:dense-parity-relative-delta")
            .expect("dense scrutiny keeps the sparse parity row with a curved bend assembled");
        assert!(parity.value <= 1.0e-9);
        for row in [
            "result:disp:node-N-110:uy",
            "result:disp:node-N-110:uz",
            "result:force:pipe-P-100:midspan:shear-z",
        ] {
            assert_eq!(result_value(&sparse, row), result_value(&dense, row));
        }
    }

    #[test]
    fn curved_bend_macro_element_review_row_states_assembled_consumption() {
        let result = run_linear_static_preview(curved_bend_span_request());

        assert_eq!(
            result.summary.component_user_stiffness_macro_element_count,
            1
        );
        let review = result
            .results
            .iter()
            .find(|item| {
                item.id == "result:component-stiffness:component-C-110:curved-bend-flexibility"
            })
            .expect("curved-bend macro-element review row is present");
        assert_eq!(review.kind, "curved_bend_macro_element_review");
        assert_eq!(review.value, CURVED_BEND_TEST_FLEXIBILITY);
        let metadata = review.metadata.as_ref().unwrap();
        assert_eq!(metadata.component, "curved_bend_flexibility");
        assert_eq!(metadata.location, "pipe:P-100");
        assert!(metadata.basis.contains("component_family=bend"));
        assert!(metadata.basis.contains("user_entered_flexibility=2"));
        assert!(metadata
            .basis
            .contains("macro_element_solve=assembled_curved_bend_stiffness"));
        assert!(metadata
            .basis
            .contains("solver_consumption=curved_bend_macro_element"));
        assert!(metadata
            .basis
            .contains("thermal_load_treatment=exact_free_expansion_identity"));
        assert!(metadata
            .basis
            .contains("distributed_load_treatment=arc_consistent_fixed_end_integration"));
        assert!(metadata
            .basis
            .contains("pressure_thrust_treatment=straight_chord_axial_end_forces"));
        assert!(metadata
            .basis
            .contains("interior_stations=arc_section_equilibrium_stations"));
        assert!(metadata
            .sign_convention
            .contains("consumed by the assembled curved-bend macro-element stiffness"));
    }

    #[test]
    fn curved_bend_macro_element_multiplier_applies_sif_only() {
        let result = run_linear_static_preview(curved_bend_span_request());

        let multiplier_row_id = "result:stress:component-C-110:pipe-P-100:end-j:user-multiplier";
        let multiplier_row = result
            .results
            .iter()
            .find(|item| item.id == multiplier_row_id)
            .expect("curved-bend SIF-only multiplier row is present");
        // Reconstruct the end-j open-formula summary from the emitted
        // endpoint stress rows (MPa) and check the multiplier is SIF-only.
        let axial = result_value(&result, "result:stress:pipe-P-100:end-j:axial-normal");
        let bending_y =
            result_value(&result, "result:stress:pipe-P-100:end-j:bending-normal-y").abs();
        let bending_z =
            result_value(&result, "result:stress:pipe-P-100:end-j:bending-normal-z").abs();
        let bending_total = bending_y + bending_z;
        let summary = (axial + bending_total)
            .abs()
            .max((axial - bending_total).abs());
        assert!(
            (multiplier_row.value - summary * CURVED_BEND_TEST_SIF).abs() <= 1.0e-3,
            "multiplier row {} must be the end-j summary {} times the user SIF only",
            multiplier_row.value,
            summary
        );
        let metadata = multiplier_row.metadata.as_ref().unwrap();
        assert!(metadata
            .basis
            .contains("solver_consumption=curved_bend_macro_element"));
        assert!(metadata
            .basis
            .contains("flexibility_realization=assembled_curved_bend_macro_element_stiffness"));
        assert!(!metadata
            .sign_convention
            .contains("base frame stiffness unchanged"));
        assert!(metadata
            .sign_convention
            .contains("multiplied by the user-entered SIF only"));
        assert!(metadata
            .sign_convention
            .contains("enters the assembled curved-bend macro-element stiffness"));
        let diagnostic = result
            .diagnostics
            .iter()
            .find(|item| item.code == "COMPONENT_STRESS_MULTIPLIER_APPLIED")
            .expect("multiplier diagnostic is present");
        assert!(diagnostic
            .message
            .contains("enters the assembled curved-bend macro-element stiffness"));
    }

    #[test]
    fn curved_bend_macro_element_emits_arc_interior_station_results() {
        let mut arc_request = curved_bend_span_request();
        arc_request.model.load_cases[0]
            .primitive_loads
            .push(curved_bend_uniform_weight_load());
        let result = run_linear_static_preview(arc_request);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        // The interior-station residual is retired: no station suppression
        // diagnostic fires and all three station grids are emitted.
        assert!(result
            .diagnostics
            .iter()
            .all(|item| !item.id.contains(":interior-stations")));
        let result_ids = result
            .results
            .iter()
            .map(|item| item.id.as_str())
            .collect::<HashSet<_>>();
        for station in ["quarter-1", "midspan", "quarter-3"] {
            for tail in ["axial", "shear-y", "shear-z"] {
                assert!(
                    result_ids
                        .contains(format!("result:force:pipe-P-100:{station}:{tail}").as_str()),
                    "missing arc station force row {station}:{tail}"
                );
            }
            for tail in ["torsion", "bending-y", "bending-z"] {
                assert!(
                    result_ids
                        .contains(format!("result:moment:pipe-P-100:{station}:{tail}").as_str()),
                    "missing arc station moment row {station}:{tail}"
                );
            }
            assert!(
                result_ids
                    .contains(format!("result:stress:pipe-P-100:{station}:axial-normal").as_str()),
                "missing arc station stress row {station}"
            );
        }

        // Independent oracle: the free loaded tip carries exactly the applied
        // nodal force, so the midspan section resultants follow from segment
        // equilibrium on the direct arc.
        let element = curved_bend_direct_element();
        let intensity = [0.0, 0.0, -190.0];
        let node_j_force = [0.0, 1000.0, 0.0, 0.0, 0.0, 0.0];
        let expected = element
            .arc_section_resultants(0.5, node_j_force, intensity)
            .unwrap();
        let midspan_rows = [
            ("result:force:pipe-P-100:midspan:axial", 0),
            ("result:force:pipe-P-100:midspan:shear-y", 1),
            ("result:force:pipe-P-100:midspan:shear-z", 2),
            ("result:moment:pipe-P-100:midspan:torsion", 3),
            ("result:moment:pipe-P-100:midspan:bending-y", 4),
            ("result:moment:pipe-P-100:midspan:bending-z", 5),
        ];
        for (row_id, slot) in midspan_rows {
            let value = result_value(&result, row_id);
            assert!(
                (value - round6(expected[slot])).abs() <= 1.0e-3,
                "midspan station row {row_id} value {value} must match the direct arc segment equilibrium {}",
                expected[slot]
            );
        }
        let station_row = result
            .results
            .iter()
            .find(|item| item.id == "result:force:pipe-P-100:midspan:shear-z")
            .expect("midspan station row present");
        let metadata = station_row.metadata.as_ref().unwrap();
        assert_eq!(
            metadata.basis,
            "arc_section_equilibrium_from_assembled_end_forces"
        );
        assert_eq!(metadata.coordinate_system, "arc_section_frame");
        assert!(metadata.sign_convention.contains("arc section frame"));
        // Straight spans keep the endpoint-interpolation basis untouched.
        let straight = run_linear_static_preview(request());
        let straight_row = straight
            .results
            .iter()
            .find(|item| {
                item.id.contains(":midspan:") && item.kind == "element_local_shear_force_y"
            })
            .expect("straight midspan station row present");
        let straight_metadata = straight_row.metadata.as_ref().unwrap();
        assert_eq!(
            straight_metadata.basis,
            "interpolated_from_endpoint_resultants"
        );
        assert_eq!(straight_metadata.coordinate_system, "element_local");
    }

    #[test]
    fn curved_bend_macro_element_thermal_free_expansion_is_stress_free() {
        let mut request = curved_bend_span_request();
        request.model.load_cases[0].primitive_loads = vec![PreviewPrimitiveLoad {
            id: "load:L-100-T".to_string(),
            category: "thermal".to_string(),
            target: LoadTargetInput::Element {
                pipe: "pipe:P-100".to_string(),
            },
            direction: "global_x".to_string(),
            magnitude: Quantity {
                value: 10.0,
                unit: "degC".to_string(),
            },
            dimension: "temperature_interval".to_string(),
            provenance: Some("invented_example_user_input".to_string()),
        }];
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");

        // Uniform thermal expansion of the anchored-free arc is stress-free:
        // the tip translates by alpha * deltaT * chord and the anchor carries
        // no reaction; recovered end forces exclude the self-equilibrated
        // free-expansion part exactly.
        let expected_tip_mm = 0.000012 * 10.0 * CURVED_BEND_TEST_CHORD_M * 1000.0;
        let ux_mm = result_value(&result, "result:disp:node-N-110:ux");
        assert!((ux_mm - expected_tip_mm).abs() <= 1.0e-6);
        assert!(result_value(&result, "result:reaction:support-S-100").abs() <= 1.0e-6);
        for row in [
            "result:force:pipe-P-100:axial",
            "result:force:pipe-P-100:axial:end-j",
            "result:force:pipe-P-100:shear-y",
            "result:moment:pipe-P-100:bending-z",
        ] {
            assert!(
                result_value(&result, row).abs() <= 1.0e-6,
                "free thermal expansion must recover zero force for {row}"
            );
        }
    }

    #[test]
    fn curved_bend_macro_element_anchored_thermal_produces_reactions() {
        let mut request = curved_bend_span_request();
        request.model.supports.push({
            let mut anchor = request.model.supports[0].clone();
            anchor.id = "support:S-110".to_string();
            anchor.node = "node:N-110".to_string();
            anchor
        });
        request.model.load_cases[0].primitive_loads = vec![PreviewPrimitiveLoad {
            id: "load:L-100-T".to_string(),
            category: "thermal".to_string(),
            target: LoadTargetInput::Element {
                pipe: "pipe:P-100".to_string(),
            },
            direction: "global_x".to_string(),
            magnitude: Quantity {
                value: 10.0,
                unit: "degC".to_string(),
            },
            dimension: "temperature_interval".to_string(),
            provenance: Some("invented_example_user_input".to_string()),
        }];
        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(result_value(&result, "result:reaction:support-S-100") > 1.0);
        assert!(
            result_value(&result, "result:force:pipe-P-100:axial").abs() > 1.0,
            "restrained thermal expansion must recover a nonzero axial end force"
        );
    }

    #[test]
    fn curved_bend_macro_element_consumes_arc_consistent_uniform_weight() {
        let mut request = curved_bend_span_request();
        request.model.load_cases[0].primitive_loads = vec![curved_bend_uniform_weight_load()];
        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        // The lumping disclosure is retired with the consistent path: no
        // curved-bend distributed-load diagnostic fires on a valid load.
        assert!(result
            .diagnostics
            .iter()
            .all(|item| !item.id.contains(":distributed-load")));

        // Independent oracle: consistent equivalent loads on the direct arc,
        // anchored solve, true end forces K d - p. The chord frame coincides
        // with global axes in this fixture.
        let element = curved_bend_direct_element();
        let intensity = [0.0, 0.0, -190.0];
        let equivalent = element.consistent_uniform_nodal_loads(intensity).unwrap();
        let displacements = curved_bend_direct_solution(&equivalent);
        let stiffness = element.global_stiffness().unwrap();
        let mut expected_forces = [0.0; ELEMENT_DOF];
        for row in 0..ELEMENT_DOF {
            for col in 0..ELEMENT_DOF {
                expected_forces[row] += stiffness[row][col] * displacements[col];
            }
            expected_forces[row] -= equivalent[row];
        }

        // The anchored-end member force carries the full distributed
        // resultant (the tributary end-lumping carried only half) plus the
        // consistent fixed-end moments; the free tip carries no end force.
        let arc_length = CURVED_BEND_TEST_RADIUS_M * PI / 2.0;
        let total_load = 190.0 * arc_length;
        let rows = [
            ("result:force:pipe-P-100:axial", UX),
            ("result:force:pipe-P-100:shear-y", UY),
            ("result:force:pipe-P-100:shear-z", UZ),
            ("result:moment:pipe-P-100:torsion", RX),
            ("result:moment:pipe-P-100:bending-y", RY),
            ("result:moment:pipe-P-100:bending-z", RZ),
        ];
        for (row_id, dof) in rows {
            let end_i = result_value(&result, row_id);
            assert!(
                (end_i - round6(expected_forces[dof])).abs() <= 1.0e-3,
                "end-i row {row_id} value {end_i} must match the direct oracle {}",
                expected_forces[dof]
            );
            let end_j = result_value(&result, &format!("{row_id}:end-j"));
            assert!(
                end_j.abs() <= 1.0e-3,
                "free tip must carry no end force for {row_id}, got {end_j}"
            );
        }
        let shear_z_end_i = result_value(&result, "result:force:pipe-P-100:shear-z");
        assert!(
            (shear_z_end_i - total_load).abs() <= 1.0e-3 * total_load,
            "end-i shear {shear_z_end_i} must carry the full distributed resultant {total_load}"
        );

        // The solved tip displacement matches the direct consistent-load solve.
        let uz_mm = result_value(&result, "result:disp:node-N-110:uz");
        assert!(
            (uz_mm - round6(displacements[DOF_PER_NODE + UZ] * 1000.0)).abs() <= 1.0e-6,
            "tip displacement {uz_mm} mm must match the direct consistent-load solve"
        );
    }

    #[test]
    fn curved_bend_macro_element_blocks_on_missing_or_inconsistent_geometry() {
        // Missing bend_pipe_ref.
        let mut request = curved_bend_span_request();
        request.model.components[0]
            .geometry
            .as_mut()
            .unwrap()
            .bend_pipe_ref = None;
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "CURVED_BEND_GEOMETRY_INPUT_MISSING"
                && item.severity == "blocking"
                && item.message.contains("bend_pipe_ref")));
        assert!(result.results.is_empty());

        // Missing user flexibility factor.
        let mut request = curved_bend_span_request();
        request.model.components[0]
            .modifiers
            .as_mut()
            .unwrap()
            .flexibility_factor_user_value = None;
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "CURVED_BEND_GEOMETRY_INPUT_MISSING"
                && item.message.contains("flexibility_factor_user_value")));

        // Missing bend radius.
        let mut request = curved_bend_span_request();
        request.model.components[0]
            .geometry
            .as_mut()
            .unwrap()
            .bend_radius = None;
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "CURVED_BEND_GEOMETRY_INPUT_MISSING"
                && item.message.contains("bend_radius")));

        // Radius too small for the chord (included angle would reach pi).
        let mut request = curved_bend_span_request();
        request.model.components[0]
            .geometry
            .as_mut()
            .unwrap()
            .bend_radius = Some(Quantity {
            value: 0.9,
            unit: "m".to_string(),
        });
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "CURVED_BEND_GEOMETRY_INCONSISTENT"
                && item.message.contains("reach or exceed pi")));

        // User bend angle inconsistent with the chord and radius.
        let mut request = curved_bend_span_request();
        request.model.components[0]
            .geometry
            .as_mut()
            .unwrap()
            .bend_angle = Some(Quantity {
            value: 1.0,
            unit: "rad".to_string(),
        });
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "CURVED_BEND_GEOMETRY_INCONSISTENT"
                && item.message.contains("arc-consistent")));

        // Unknown pipe reference.
        let mut request = curved_bend_span_request();
        request.model.components[0]
            .geometry
            .as_mut()
            .unwrap()
            .bend_pipe_ref = Some("pipe:P-999".to_string());
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "CURVED_BEND_MAPPING_INPUT_INVALID"));
    }

    #[test]
    fn curved_bend_macro_element_solves_assembled_nonlinear_loop() {
        // DEC-070 residual closure: the nonlinear active-set loop now carries
        // the curved-bend macro-element as an explicit-stiffness slot. With
        // the one-way support staying released, the nonlinear solve must
        // reproduce the linear curved-bend path exactly.
        let mut request = curved_bend_span_request();
        request.model.supports.push({
            let mut support = request.model.supports[0].clone();
            support.id = "support:S-110".to_string();
            support.node = "node:N-110".to_string();
            support.restraints = vec![];
            support.nonlinear = Some(NonlinearSupportInput {
                behavior: "one_way".to_string(),
                dof: "UY".to_string(),
                initial_state: Some("inactive".to_string()),
                active_when: Some("positive".to_string()),
                contact_when: None,
                closes_when: None,
                gap: None,
                friction_coefficient: None,
                normal_reaction: None,
                normal_reaction_source: None,
            });
            support
        });
        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(result
            .diagnostics
            .iter()
            .all(|item| item.code != "CURVED_BEND_NONLINEAR_LOOP_UNSUPPORTED"));
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "NONLINEAR_SUPPORT_LOOP_CONVERGED"));
        assert_eq!(
            result_value(&result, "result:nonlinear-support:converged-flag"),
            1.0
        );
        assert_eq!(
            result_value(&result, "result:nonlinear-support:support-S-110:state-code"),
            0.0
        );

        // The released nonlinear loop's arc-tip displacement matches both the
        // linear curved-bend preview row and the independent direct
        // macro-element oracle.
        let linear_uy_mm = result_value(&result, "result:disp:node-N-110:uy");
        let nonlinear_uy_mm = result_value(
            &result,
            "result:nonlinear-support:support-S-110:uy-displacement",
        );
        assert!(
            (nonlinear_uy_mm - linear_uy_mm).abs() <= 1.0e-6,
            "nonlinear loop tip displacement {nonlinear_uy_mm} mm must match the linear curved-bend path {linear_uy_mm} mm"
        );
        let expected_tip = curved_bend_direct_tip_displacements(UY, 1000.0);
        assert!(
            (nonlinear_uy_mm - round6(expected_tip[UY] * 1000.0)).abs() <= 1.0e-6,
            "nonlinear loop tip displacement {nonlinear_uy_mm} mm must match the direct macro-element solve"
        );
    }

    #[test]
    fn legacy_bend_mode_keeps_multiplier_and_chord_realization_unchanged() {
        // The invented fixture bend stays on mechanics_geometry_only: the
        // straight chord is assembled, the multiplier stays sif * flexibility,
        // and the DEC-045 provenance wording is untouched.
        let result = run_linear_static_preview(request());

        assert!(result
            .results
            .iter()
            .all(|item| item.kind != "curved_bend_macro_element_review"));
        let row = result
            .results
            .iter()
            .find(|item| {
                item.id == "result:stress:component-C-110:pipe-P-100:end-j:user-multiplier"
            })
            .expect("legacy bend multiplier row is present");
        let metadata = row.metadata.as_ref().unwrap();
        assert!(metadata
            .basis
            .contains("solver_consumption=mechanics_geometry_only"));
        assert!(!metadata.basis.contains("flexibility_realization"));
        assert_eq!(
            metadata.sign_convention,
            "positive value is base open-mechanics stress summary multiplied by user-entered component modifiers; base frame stiffness unchanged"
        );
    }
}

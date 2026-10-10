//! Product-preview physics adapter.
//!
//! This crate maps locally supplied model data into the code-neutral mechanics
//! crates. It emits mechanics quantities and diagnostics only. Nonblank source
//! provenance does not establish redistribution clearance or engineering approval;
//! bundled examples remain invented. No standards criteria, allowables, SIF tables,
//! private datasets, or professional acceptance are bundled by this crate.

mod annulus_geometry;
mod formation_guard;
// Resolved load/reference-state case (model 0.4.0); one resolved case per load
// case drives assembly, recovery and published evidence.
mod case_state;
pub use case_state::input::{
    AnalysisStateInput, Authored, ExpansionLawInput, ReferenceConfigurationInput,
};
pub use case_state::{
    LOAD_REFERENCE_SEMANTIC_CONTRACT_ID, LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID,
    LOAD_REFERENCE_STATE_CONTRACT, LOAD_STATE_MODEL_VERSION, LOAD_STATE_PROFILE_ID,
    LOAD_STATE_SOURCE_PROFILE_ID,
};
mod pressure_sum;
pub mod self_weight;
mod source_recovery;
mod source_receipt;
#[cfg(test)]
mod f1a_tests;
#[cfg(test)]
mod f1b_tests;
#[cfg(test)]
mod s11f_tests;
#[cfg(test)]
mod s11g_tests;
#[cfg(test)]
mod source_budget_tests;

use open_pipe_stress_curved_bend::CurvedBendMacroElement;
pub use open_pipe_stress_curved_bend::{arc_geometry, kink, ArcFrame, TANGENCY_TOLERANCE_RAD};
// I109: magnitudes formerly formed with libm `hypot` are correctly rounded norms; `source_receipt::scaled_norm` and `displacement_magnitude` stay deterministic IEEE, not correctly rounded.
use open_pipe_stress_frame_kernel::correct_norm::{norm2, norm3};
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_frame_kernel::load_ledger::{
    gamma, product_upward, AssembledForce, Formation, LoadLedger,
};
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, certify_curved_uniform_load, reduce_assembled_sparse_system,
    CurvedFormation, ForceScaleReason, ForceScaledError,
    ForceScalingRefusal, FormationCheck, FormationCheckReason, LoadFidelityReport, PublishedValue,
    RangeTrigger, RecordOutcome, RecordRepresentability, Representability, SolveQuality,
    SparseAssemblyOptions, SparseStiffness, StiffnessBlock, StructuralError, StructuralReport,
};
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness_with_connectors, element_dof_map, force_scaled_spring_action,
    reduce_assembled_system, reduce_assembled_system_with_prescribed_displacements, solve_dense,
    ForceScale, FrameElement, FrameKernelError, FrameNode, Matrix12,
    DOF_PER_NODE, ELEMENT_DOF, RX, RY, RZ, UX, UY, UZ,
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
use open_pipe_stress_nonlinear_integration::structural_adapter::{
    solve_with_force_scaling, AssemblyEvidence, EvidenceRepresentation, ForceScalingCase,
    ForceScalingOutcome, SparseAssemblyEvidence, StrictGapEvidence,
};
use open_pipe_stress_nonlinear_integration::{
    eligible_contact_dofs, solve_active_set_frame_with_mode_and_springs_assembled,
    ConvergenceControl, ConvergencePolicyStatus, CurvedBendStiffnessElement,
    DerivedFrictionNormalReaction, FrictionNormalReaction, LinearSolveMode,
    NonlinearFrameSolveInput, NonlinearFrameSolveResult, NonlinearIntegrationError,
    NonlinearResidualObservation,
};
use open_pipe_stress_nonlinear_supports::{
    ActivationSense, ActiveSetState, GapDirection, NonlinearSupport, NonlinearSupportBehavior,
    SupportStateRecord,
};
use open_pipe_stress_primitive_loads::{
    generate_seismic_equivalent_static_loads, generate_wind_equivalent_static_loads, prepare_loads,
    ElementExposedDiameter, ElementMassPerLength, EquivalentStaticAxisFactor, LoadApplication,
    LoadDimension, LoadDirection, LoadExtent, LoadQuantity, PrimitiveLoad, PrimitiveLoadCategory,
    SeismicEquivalentStaticBasis, WindEquivalentStaticBasis,
};
use open_pipe_stress_solver_diagnostics::{
    DiagnosticSeverity as SolverDiagnosticSeverity, SolverDiagnostic, SolverDiagnosticCode,
};
use open_pipe_stress_sparse_direct::{solve_symmetric_system_from_entries, SymmetricMatrixEntry};
use open_pipe_stress_straight_pipe::{
    LocalLoadDirection, PipeEnd, PipeEndResultants, SpannedGlobalUniformLoad,
    SpannedUniformLocalLoad, StraightPipeElement, StraightPipeError, StraightPipeSectionProperties,
    UniformLoadSpan,
};
use open_pipe_stress_stress_recovery::{
    recover_stresses, AnalysisStatus, ForceResultants, StressComponents,
    StressRecoveryInput, StressSectionProperties,
};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::f64::consts::PI;

mod validation;
use validation::validate_model_inputs;

#[cfg(test)]
#[cfg(test)]
mod membrane_publication_range;
#[allow(dead_code)]
mod pressure_exact;
mod pressure_material;
mod exact_admission;
mod bend_pressure;
mod joint;
use exact_admission::PRESSURE_SEMANTIC_CONTRACT_ID;
mod pressure_runtime;
mod preview_physics;
mod retained_product;
mod retained_receipt;
mod retained_memory;
// I61 U1: the private serializer (production-unreachable until U3).
mod retained_wire;
pub use retained_memory::{
    borrowed_request_census, borrowed_value_census, AllowanceStatus, BorrowedRequestFacts,
    BorrowedValueFacts, CapacityFact, CensusStatus, HeadlessRootFacts, MissingAdmissionTerm,
    ProfileStatus, RetainedAdmissionReport, RetainedCaller, RetainedHeadlessContext,
};
#[cfg(test)]
mod retained_product_tests;
#[cfg(test)]
mod retained_wire_tests;
#[cfg(test)]
mod retained_facade_tests;
pub use pressure_runtime::{PressureContractInput, PressureRegionInput, PressureTerminalInput};

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

#[derive(Debug, Clone)]
pub struct PreviewModel {
    pub pressure_contract: Option<PressureContractInput>,
    pub schema_version: String,
    pub document_kind: String,
    pub project: Project,
    pub analysis_status: StatusEnvelope,
    pub nodes: Vec<PreviewNode>,
    pub pipe_segments: Vec<PreviewPipe>,
    pub sections: Vec<PreviewSection>,
    pub supports: Vec<PreviewSupport>,
    pub components: Vec<PreviewComponent>,
    pub materials: Vec<MaterialInput>,
    pub load_cases: Vec<PreviewLoadCase>,
    pub combinations: Vec<PreviewCombination>,
    /// Load/reference-state owner (model 0.4.0); presence elsewhere blocks.
    pub reference_configurations: Authored<Vec<ReferenceConfigurationInput>>,
    /// Material-owned `expansion_laws`, parallel to `materials` by index. The
    /// authored key lives on each material record; `MaterialInput` is unchanged.
    pub material_expansion_laws: Vec<Authored<Vec<ExpansionLawInput>>>,
    /// Indices of request-level material records that authored an
    /// `expansion_laws` key (any value, including null). Request materials
    /// never own expansion laws; the resolver blocks rather than ignores them.
    pub(crate) request_material_expansion_laws: Vec<usize>,
}

/// One material record as authored: the unchanged `MaterialInput` fields
/// (typed, so duplicate keys and positions keep their serde errors) and the
/// material-owned `expansion_laws` key.
#[derive(Deserialize)]
struct MaterialRecordWire<L> {
    #[serde(flatten)]
    material: MaterialInput,
    #[serde(default)]
    expansion_laws: Authored<L>,
}

/// Wire form of `PreviewModel`; identical fields and defaults, except that each
/// material record is split into its unchanged `MaterialInput` and its optional
/// `expansion_laws`, so the new key is never silently discarded.
#[derive(Deserialize)]
struct PreviewModelWire {
    #[serde(default)]
    pressure_contract: Option<PressureContractInput>,
    schema_version: String,
    document_kind: String,
    project: Project,
    analysis_status: StatusEnvelope,
    nodes: Vec<PreviewNode>,
    pipe_segments: Vec<PreviewPipe>,
    #[serde(default)]
    sections: Vec<PreviewSection>,
    supports: Vec<PreviewSupport>,
    #[serde(default)]
    components: Vec<PreviewComponent>,
    #[serde(default)]
    materials: Vec<MaterialRecordWire<Vec<ExpansionLawInput>>>,
    #[serde(default)]
    load_cases: Vec<PreviewLoadCase>,
    #[serde(default)]
    combinations: Vec<PreviewCombination>,
    #[serde(default)]
    reference_configurations: Authored<Vec<ReferenceConfigurationInput>>,
}

impl<'de> Deserialize<'de> for PreviewModel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let wire = PreviewModelWire::deserialize(deserializer)?;
        let (materials, material_expansion_laws) = wire
            .materials
            .into_iter()
            .map(|record| (record.material, record.expansion_laws))
            .unzip();
        Ok(Self {
            pressure_contract: wire.pressure_contract,
            schema_version: wire.schema_version,
            document_kind: wire.document_kind,
            project: wire.project,
            analysis_status: wire.analysis_status,
            nodes: wire.nodes,
            pipe_segments: wire.pipe_segments,
            sections: wire.sections,
            supports: wire.supports,
            components: wire.components,
            materials,
            load_cases: wire.load_cases,
            combinations: wire.combinations,
            reference_configurations: wire.reference_configurations,
            material_expansion_laws,
            request_material_expansion_laws: Vec::new(),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub id: String,
    // Retain authored metadata, including malformed/missing values, so the
    // solver boundary can report a structured unit-input diagnostic.
    #[serde(default)]
    pub units: serde_json::Value,
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
    #[serde(default)]
    pub section_ref: Option<String>,
    pub material: String,
    #[serde(default)]
    pub y_reference: Option<Vec3>,
    #[serde(default)]
    pub provenance: Option<String>,
}

/// Shared preview-section metadata. Only explicit pipe OD/wall are bindable.
#[derive(Debug, Clone, Deserialize)]
pub struct PreviewSection {
    pub id: String,
    pub name: String,
    pub section_type: String,
    pub properties: BTreeMap<String, Quantity>,
    pub provenance: serde_json::Value,
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
    #[serde(default)]
    pub objective_connector: Option<serde_json::Value>,
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
    #[serde(default)]
    pub pressure_regions: Option<Vec<PressureRegionInput>>,
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
    /// material temperature-point id whose E, G, and alpha (when supplied on
    /// the point) this load case solves with. Exact-id selection remains
    /// available under DEC-077 and DEC-092; an unresolved reference or a
    /// selected point without explicit E or G is blocking.
    #[serde(default)]
    pub modulus_basis_ref: Option<String>,
    /// Optional user-entered solve temperature (DEC-077). When supplied
    /// instead of `modulus_basis_ref`, E, G, and alpha are linearly
    /// interpolated between the two adjacent user-entered temperature points.
    /// Requests at or outside the stored range edges are blocking;
    /// extrapolation and base-property fallback are never performed.
    #[serde(default)]
    pub modulus_basis_temperature: Option<Quantity>,
    #[serde(default)]
    pub provenance: Option<String>,
    /// Closed `openpipestress.load_reference_state/1.0.0` case state (model
    /// 0.4.0). It is the complete resolved-state and ordinary-source request.
    #[serde(default)]
    pub analysis_state: Authored<AnalysisStateInput>,
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
    /// User-marked partial-extent exposed spans. Each entry generates its
    /// own wind load over the marked fraction range of one pipe span only;
    /// multiple disjoint extents per pipe are allowed, overlapping extents
    /// block, and a pipe named here must not also appear in
    /// `exposed_pipe_refs`.
    #[serde(default)]
    pub exposed_spans: Vec<WindExposedSpanInput>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WindExposedSpanInput {
    /// User-marked pipe id the partial extent applies to.
    #[serde(default)]
    pub pipe_ref: Option<String>,
    /// User-entered dimensionless start fraction of the exposed extent.
    #[serde(default)]
    pub start_fraction: Option<Quantity>,
    /// User-entered dimensionless end fraction of the exposed extent.
    #[serde(default)]
    pub end_fraction: Option<Quantity>,
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
    #[serde(default)]
    pub constitutive_basis: Option<String>,
    #[serde(default)]
    pub poisson_ratio: Option<Quantity>,
    pub id: String,
    pub elastic_modulus: Quantity,
    #[serde(default)]
    pub shear_modulus: Option<Quantity>,
    #[serde(default)]
    pub thermal_expansion_coefficient: Option<Quantity>,
    /// User-entered temperature-indexed property points (DEC-068 item 1).
    /// A load case may name one point id as its exact modulus basis or supply
    /// a solve temperature for DEC-077 linear interpolation between adjacent
    /// points. All values remain user-entered; selected bases require explicit
    /// point G values, and extrapolation or base-G fallback is blocked.
    #[serde(default)]
    pub temperature_points: Vec<MaterialTemperaturePointInput>,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MaterialTemperaturePointInput {
    #[serde(default)]
    pub poisson_ratio: Option<Quantity>,
    /// Stable user-assigned basis id (e.g. a temperature-case label).
    pub id: String,
    #[serde(default)]
    pub temperature: Option<Quantity>,
    #[serde(default)]
    pub elastic_modulus: Option<Quantity>,
    /// Optional explicit user-entered shear modulus at this temperature point
    /// (DEC-092). It is required when this point participates in an exact or
    /// interpolated selected basis; no base-material G is substituted.
    #[serde(default)]
    pub shear_modulus: Option<Quantity>,
    #[serde(default)]
    pub thermal_expansion_coefficient: Option<Quantity>,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LinearStaticPreviewRequest {
    pub model: PreviewModel,
    pub materials: Vec<MaterialInput>,
}

#[derive(Deserialize)]
struct LinearStaticPreviewRequestWire {
    model: PreviewModel,
    #[serde(default)]
    materials: Vec<MaterialRecordWire<serde::de::IgnoredAny>>,
}

impl<'de> Deserialize<'de> for LinearStaticPreviewRequest {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let wire = LinearStaticPreviewRequestWire::deserialize(deserializer)?;
        let mut model = wire.model;
        let mut materials = Vec::with_capacity(wire.materials.len());
        for (index, record) in wire.materials.into_iter().enumerate() {
            if record.expansion_laws.is_authored() {
                model.request_material_expansion_laws.push(index);
            }
            materials.push(record.material);
        }
        Ok(Self { model, materials })
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_evidence: Option<serde_json::Value>,
    pub schema_version: String,
    pub producer: MechanicsProducer,
    pub numerical_quality: NumericalQuality,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_block_recovery: Option<serde_json::Value>,
    pub formulation_basis: FormulationBasis,
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

/// Prospective source semantics; historical raw 0.1 carriers retain their identity.
pub const MECHANICS_SCHEMA_VERSION: &str = "0.2.0";
// Initial source-method work-reservation policy; generic/strict-gap defaults are separate.
const SOURCE_BLOCKS_WORK_LIMIT: usize = 4_000_000;
const PHYSICS_SOURCE_WORK_LIMIT: usize = 8_000_000;
const SOURCE_BLOCKS_INVOCATION_WORK_LIMIT: usize = 64_000_000;
#[derive(Debug)]
struct SourceRecoveryBudget {
    per_case_limit: usize,
    invocation_limit: usize,
    charged: usize,
    failed_charged: usize,
    publication_charged: usize,
    rejected: usize,
    attempts: usize,
    /// 0.4.0 only (ROOT CP3 SF-1): when set, this invocation publishes every
    /// case on its ordinary route; a successful retained-source attempt is
    /// declined with this cause instead of being selected.
    load_state_join_withheld: Option<String>,
    /// 0.4.0 only: the first cause for which a selected join could not
    /// finalize. The captured route then republishes ordinarily.
    load_state_join_failure: Option<String>,
}
impl Default for SourceRecoveryBudget {
    fn default() -> Self { Self { per_case_limit: SOURCE_BLOCKS_WORK_LIMIT, invocation_limit: SOURCE_BLOCKS_INVOCATION_WORK_LIMIT, charged: 0, failed_charged: 0, publication_charged: 0, rejected: 0, attempts: 0, load_state_join_withheld: None, load_state_join_failure: None } }
}
impl SourceRecoveryBudget {
    /// The republication continues this same ledger (CP4 review N-2, ROOT's
    /// preference): every successful, failed, reserved and publication charge
    /// of the first run stays charged, so the whole captured invocation,
    /// including its fallback, stays within the one invocation limit.
    fn withholding_load_state_join(&self, cause: String) -> Self {
        Self {
            per_case_limit: self.per_case_limit,
            invocation_limit: self.invocation_limit,
            charged: self.charged,
            failed_charged: self.failed_charged,
            publication_charged: self.publication_charged,
            rejected: self.rejected,
            attempts: self.attempts,
            load_state_join_withheld: Some(cause),
            load_state_join_failure: self.load_state_join_failure.clone(),
        }
    }
    fn record_load_state_join_failure(&mut self, cause: String) {
        self.load_state_join_failure.get_or_insert(cause);
    }
    fn case_limit(&self) -> usize { self.per_case_limit.min(self.invocation_limit.saturating_sub(self.charged)) }
    fn reserve_publication(&mut self, amount: usize) -> Result<(), open_pipe_stress_frame_kernel::structural::exact_boundary::WorkReport> {
        if amount > self.invocation_limit.saturating_sub(self.charged) {
            self.rejected = self.rejected.saturating_add(amount);
            return Err(open_pipe_stress_frame_kernel::structural::exact_boundary::WorkReport { charged: self.charged, rejected: amount, limit: self.invocation_limit });
        }
        self.charged += amount;
        self.publication_charged += amount;
        Ok(())
    }
    fn debit(&mut self, charged: usize, failed: bool) {
        // Every fresh attempt was allocated at most this remaining amount;
        // failure never resets or refunds executed reservations.
        assert!(charged <= self.invocation_limit.saturating_sub(self.charged));
        self.charged += charged;
        if failed { self.failed_charged += charged; }
    }
}

pub const SOURCE_BLOCKS_SEMANTIC_CONTRACT_ID: &str =
    "openpipestress.result_semantics/0.3.0/source-blocks-1";
pub const PHYSICS_SOURCE_SEMANTIC_CONTRACT_ID: &str =
    "openpipestress.result_semantics/0.3.0/physics-source-1";
pub const PRECISION_SEMANTIC_CONTRACT_ID: &str =
    "openpipestress.result_semantics/0.3.0/precision-1";
pub const PHYSICS_SEMANTIC_CONTRACT_ID: &str = "openpipestress.result_semantics/0.3.0/physics-1";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MechanicsProducer {
    pub component_name: String,
    pub component_version: String,
    pub semantic_contract_id: String,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NumericalQualityStatus {
    NotAssessed,
    ChecksPassed,
    Sensitive,
    Unresolved,
    Failed,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StructuralStatus {
    PassiveModelBasis,
    PhysicalMechanismWitnessed,
    NegativeEnergyWitnessed,
    NumericallyUnresolved,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelMatrixFidelity {
    RepresentedEquationsRetained,
    AssemblyLossDetected,
    AssemblyUncertainty,
    NotAssessed,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccuracyEvidence {
    NotClaimed,
    ReferenceVerified,
    Unresolved,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct NumericalCaseQuality {
    pub basis_ref: ResultBasisRef,
    pub structural_status: StructuralStatus,
    pub solve_quality: NumericalQualityStatus,
    pub model_matrix_fidelity: ModelMatrixFidelity,
    pub accuracy_evidence: AccuracyEvidence,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct NumericalQuality {
    pub value_representation: String,
    pub publication_quantization: String,
    pub integrity_policy: String,
    pub status: NumericalQualityStatus,
    pub cases: Vec<NumericalCaseQuality>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct FormulationBasis {
    pub profile_id: String,
    pub limitations: Vec<String>,
}

pub fn mechanics_producer() -> MechanicsProducer {
    MechanicsProducer {
        component_name: solver_component_name().to_string(),
        component_version: solver_component_version().to_string(),
        semantic_contract_id: PRECISION_SEMANTIC_CONTRACT_ID.to_string(),
    }
}

fn mechanics_producer_for_model(model: &PreviewModel) -> MechanicsProducer {
    let mut producer = mechanics_producer();
    if pressure_runtime::exact_contract(model) == Some(pressure_runtime::ExactContract::PressureV3) {
        producer.semantic_contract_id = PRESSURE_SEMANTIC_CONTRACT_ID.to_string();
    } else if case_state::is_load_state(model) {
        producer.semantic_contract_id = LOAD_REFERENCE_SEMANTIC_CONTRACT_ID.to_string();
    } else if pressure_runtime::is_exact(model) {
        producer.semantic_contract_id = PHYSICS_SEMANTIC_CONTRACT_ID.to_string();
    }
    producer
}

fn formulation_basis_for_model(model: &PreviewModel) -> FormulationBasis {
    if pressure_runtime::exact_contract(model) == Some(pressure_runtime::ExactContract::PressureV3) {
        // T4-U3: a model with an objective connector states its law; a model
        // without one keeps v3's text unchanged.
        let mut basis = exact_admission::pressure_v3_formulation_basis(
            case_state::is_load_state(model),
            model.components.iter().any(|c| c.objective_connector.is_some()),
        );
        // T4-U2: a model with a realized bend states the bend's terms; a
        // straight-only v3 model keeps its text unchanged (SP-1).
        if model.components.iter().any(is_curved_bend_macro_component) {
            basis.limitations.push(bend_pressure::BEND_LIMITATION.to_string());
        }
        return basis;
    }
    // T1 (DESIGN 10.3): 0.4.0 is exact-route only. A 0.4.0 document without
    // the exact contract never solves (pressure_runtime blocks it), and its
    // blocked envelope stays on the load/reference-state identity and profile.
    if case_state::is_load_state(model) {
        return load_state_formulation_basis();
    }
    if !pressure_runtime::is_exact(model) {
        return preview_formulation_basis();
    }
    exact_straight_pressure_formulation_basis()
}

fn load_state_formulation_basis() -> FormulationBasis {
    {
        FormulationBasis {
            profile_id: LOAD_STATE_PROFILE_ID.to_string(),
            limitations: vec![
                "Independently solved linear small-displacement straight circular members on the exact straight-pressure route; one resolved case per load case supplies each member's selected E/nu with derived G, actual/selected/installation temperatures and explicit expansion definition.".to_string(),
                "Thermal and fit reference strain compose as lambda_fit*lambda_thermal-1 and enter once as an axial eigenstrain; uniform member temperature only, no gradients, finite strain, inelasticity or fit-up joints.".to_string(),
                "Global rigid support translations/rotations are prescribed absolute boundary values through the partitioned solve; reactions come from the unreduced equations. Spring base motion, device preload/reference, inactive or locked supports are not provided.".to_string(),
                "Ordinary applied loads are exactly the case's declared source ledger with explicit factors; unreferenced stored primitives are excluded. Hydrostatic head, contents-weight state and per-case mass selection are not provided.".to_string(),
                "History is independent equilibrium only; no installation, contact, friction or predecessor history is represented. Combinations remain unsupported.".to_string(),
                "Retained-source recovery is not joined for these inputs; only ordinary structural recovery is published. No code compliance or professional acceptance is produced.".to_string(),
            ],
        }
    }
}

/// The joined load/reference-state envelope: the ordinary profile's physics
/// with the selected retained-source response published for sensitive or
/// rejected ordinary cases.
fn joined_load_state_formulation_basis() -> FormulationBasis {
    let mut basis = load_state_formulation_basis();
    basis.profile_id = LOAD_STATE_SOURCE_PROFILE_ID.to_string();
    let last = basis.limitations.len() - 1;
    basis.limitations[last] = "A case whose ordinary attempt is sensitive or rejected publishes its selected retained-source exact response when the resolved case closes on the retained source (eigen element loads, member pairs and prescribed motions included) and no pressure region is present; every other case publishes checked ordinary structural recovery. A finalized receipt binds every case. No code compliance or professional acceptance is produced.".to_string();
    basis
}

fn exact_straight_pressure_formulation_basis() -> FormulationBasis {
    FormulationBasis {
        profile_id: "exact_straight_pressure_v2".to_string(),
        limitations: vec![
            "Small-displacement homogeneous-isotropic straight circular members with explicit common E/nu selection; G is derived, and source OD/effective wall define the section.".to_string(),
            "Pressure is internal differential with zero external pressure, explicit case-scoped collinear equal-bore regions and closure-transfer paths; pressure bends, joints and nonlinear support composition remain unimplemented.".to_string(),
            "Mechanical, pressure-eigen and cap-transfer contributions remain distinct; rounded cap/eigen ledgers are observational and do not replace source-grouped pressure assembly.".to_string(),
            "Pressure is uniform within each region. Structural line loads act as entered; contents density does not imply hydrostatic pressure head or a coupled static-fluid pressure/weight state.".to_string(),
            "Signed support actions are attributed only for admitted linear restraints and springs; ambiguous coincident rigid attribution is refused.".to_string(),
            "The circular normal-stress maximum bounds the supplied binary64 section-statical coefficients; solver/coefficient formation error is separate, and incomplete coverage withholds the headline.".to_string(),
            "Exact-profile combinations and equivalent-static generators remain unsupported; no code compliance or professional acceptance is produced, and actual numerical admission remains separate.".to_string(),
        ],
    }
}

/// Precision preservation alone supplies no structural integrity assessment.
fn integrity_dof_map(model: &PreviewModel) -> Vec<String> {
    model
        .nodes
        .iter()
        .flat_map(|node| {
            ["UX", "UY", "UZ", "RX", "RY", "RZ"].map(|dof| format!("{}:{dof}", node.id))
        })
        .collect()
}

/// One entry of `integrity_dof_map`, formed alone.
fn integrity_dof_label(model: &PreviewModel, dof: usize) -> String {
    let name = ["UX", "UY", "UZ", "RX", "RY", "RZ"][dof % DOF_PER_NODE];
    model.nodes.get(dof / DOF_PER_NODE).map_or_else(
        || format!("global_dof={dof}"),
        |node| format!("{}:{name}", node.id),
    )
}

// Byte lengths delimit exact source identities without lossy punctuation folding.
fn exact_source_identity(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect()
}

fn integrity_diagnostic_id(case_id: &str) -> String {
    format!("diagnostic:numerical-integrity:{case_id}")
}

/// S11-G section 3.5: `formation` is the case's load-row finding. When the
/// ordinary code would be `CHECKS_PASSED`, a finding demotes it to
/// `NUMERICAL_INTEGRITY_SENSITIVE` with one reason sentence appended; the
/// `StructuralReport` text stays truthful. The no-op rule: a case already
/// Sensitive is left exactly as today (no sentence, no byte change).
///
/// F1a (T3 D1 §4.3.1 D5C-3, §5 item 5a): `formation_check` is K-D5's
/// `FormationCheck` record, present only when the D-5 check demoted the case.
/// When present it is rendered as one evidence line after the S11-G step
/// (whose no-op rule leaves a K-D5-Sensitive record without a guard
/// sentence); when absent the record is byte-identical to today's.
///
/// F1b (T3 D1 §4.7; ROOT Q7): `range_scaling` is W2's publication, present
/// only when the case was published at b != 0. Its `range_scaling:` line is
/// method evidence, appended after F1a's line (or the S11-G sentence, or the
/// base message), separated by one space, whatever the code. When absent the
/// record is byte-identical to today's.
#[allow(clippy::too_many_arguments)]
fn append_integrity_report(
    diagnostics: &mut Vec<Diagnostic>,
    case_id: &str,
    report: &StructuralReport,
    model: &PreviewModel,
    equilibrium: Option<
        &open_pipe_stress_nonlinear_integration::product_equilibrium::ProductEquilibriumReport,
    >,
    formation: Option<&formation_guard::FormationFinding>,
    formation_check: Option<&FormationCheck>,
    range_scaling: Option<&ForceScaledPublication>,
) {
    let code = if report.quality == SolveQuality::Sensitive {
        "NUMERICAL_INTEGRITY_SENSITIVE"
    } else {
        "NUMERICAL_INTEGRITY_CHECKS_PASSED"
    };
    diagnostics.push(diag(&integrity_diagnostic_id(case_id), code, if report.quality == SolveQuality::Sensitive { "warning" } else { "info" },
        format!("{} represented original-equation structural evidence for load case {}: {:?}; global_dof_map={:?}. {} No certified inertia, guaranteed forward accuracy, or pressure/component/stress engineering qualification is claimed.", report.policy, case_id, report, integrity_dof_map(model),
            equilibrium.map(|e|format!("{} final same-state evaluated equilibrium and derived residual-work evidence: {:?}; residual units are N for global DOF%6<3 and N*m otherwise; work units N*m; observed maximum only, exact represented maximum not claimed; general-energy historical alias is residual work, not total energy balance; separate zero count/cap/contact/sliding checks passed",e.policy,e)).unwrap_or_else(||"The contribution audit distinguishes intended assembly from stored equations; physical formulation limitations remain applicable.".into())),
        vec![case_id.to_string()]));
    if let (Some(finding), Some(record)) = (formation, diagnostics.last_mut()) {
        formation_guard::demote(record, finding);
    }
    if let (Some(check), Some(record)) = (formation_check, diagnostics.last_mut()) {
        record.message = format!(
            "{} {}",
            record.message,
            formation_check_evidence_line(model, check)
        );
    }
    if let (Some(publication), Some(record)) = (range_scaling, diagnostics.last_mut()) {
        record.message = format!(
            "{} {}",
            record.message,
            range_scaling_evidence_line(model, publication)
        );
    }
}

/// F1a: K-D5's `FormationCheck` record as one evidence line of the integrity
/// diagnostic (D5C-3; the record is never part of `StructuralReport`).
fn formation_check_evidence_line(model: &PreviewModel, check: &FormationCheck) -> String {
    match &check.reason {
        FormationCheckReason::Estimate => format!(
            "formation_check: reason=estimate; row={}; doubled_correction={:?}; scale={:?}; trigger_ratio={:?}",
            // `none` is unreachable (F1a N1): K-D5 sets `global_dof` on every
            // `Estimate` record; the fallback keeps the formatter total.
            check
                .global_dof
                .map_or_else(|| "none".to_string(), |dof| integrity_dof_label(model, dof)),
            check.doubled_correction,
            check.scale,
            check.ratio,
        ),
        FormationCheckReason::FormationCheckUnavailable { detail } => format!(
            "formation_check: reason=formation_check_unavailable; detail={detail}"
        ),
    }
}

/// The retained-source eligibility predicate (main's; S11-G revision 2.2
/// G-1 removed 2.1's load-row gate, T10): the captured entry, without
/// nonlinear supports or combinations.
fn source_eligible(captured: bool, nonlinear: bool, combinations: bool) -> bool {
    captured && !nonlinear && !combinations
}

/// S11-G revision 2.2 G-3 (T21): a retained-source response of a case whose
/// load-row guard fired is never selected; it is declined into the existing
/// `failed`/`unsupported` receipt entry, with its executed work charged.
fn decline_for_formation(
    recovery: source_recovery::SelectedSourceRecovery,
    load_row_finding: Option<&formation_guard::FormationFinding>,
) -> Result<source_recovery::SelectedSourceRecovery, source_recovery::RecoveryFailure> {
    if load_row_finding.is_some() {
        Err(recovery.decline_formation())
    } else {
        Ok(recovery)
    }
}

/// Whether a case routes to retained-source recovery (S11-G revision 2.2
/// G-2, T10): an ordinary report that is Sensitive, an ordinary attempt that
/// errs, or a load-row formation finding (which demotes the published
/// verdict to Sensitive).
fn needs_source_recovery(
    report_sensitive: bool,
    attempt_err: bool,
    load_row_finding: Option<&formation_guard::FormationFinding>,
) -> bool {
    report_sensitive || attempt_err || load_row_finding.is_some()
}

/// S11-G: each published row's entity (member, support, node) and its body.
fn formation_entity_bodies(
    model: &PreviewModel,
    built: &BuiltModel,
    bodies: &formation_guard::Bodies,
) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    let replaced = replaced_span_ids_of(built);
    for pipe in &built.pipes {
        // T4-U3 (S21): a replaced span publishes no row and has no body.
        if replaced.contains(pipe.element_id.as_str()) {
            continue;
        }
        if let Some(body) = bodies.body_of_node(pipe.node_i.index) {
            map.insert(pipe.element_id.clone(), body);
        }
    }
    for (index, node) in model.nodes.iter().enumerate() {
        if let Some(body) = bodies.body_of_node(index) {
            map.insert(node.id.clone(), body);
        }
    }
    for support in &model.supports {
        if let Some(body) = node_index(model, &support.node).and_then(|i| bodies.body_of_node(i)) {
            map.insert(support.id.clone(), body);
        }
    }
    map
}

/// S11-G: the case's bodies for the guards' scales (DESIGN section 4.1.6.1
/// item 1: straight members, curved spans and objective connectors).
fn formation_bodies(built: &BuiltModel) -> formation_guard::Bodies {
    let coordinates = built
        .nodes
        .iter()
        .map(|n| n.coordinates)
        .collect::<Vec<_>>();
    // T4-U3 (S12): the connector's edge replaces its span's.
    let replaced = replaced_span_ids_of(built);
    let edges = built
        .pipes
        .iter()
        .filter(|p| !replaced.contains(p.element_id.as_str()))
        .map(|p| (p.node_i.index, p.node_j.index))
        .chain(
            built
                .curved_bend_elements
                .iter()
                .map(|b| (b.node_i, b.node_j)),
        )
        .chain(
            built
                .connectors
                .iter()
                .map(|c| (c.node_i().index, c.node_j().index)),
        )
        .collect::<Vec<_>>();
    formation_guard::Bodies::new(&coordinates, &edges)
}

/// S11 section 6: a detected load-contribution loss keeps the case Sensitive
/// (the structural report's quality, published as
/// `NUMERICAL_INTEGRITY_SENSITIVE`) and adds this warning, whose
/// `affected_refs` are the case and the ledger sources of the flagged rows
/// (ROOT's F-1 amendment). An unaudited row (RV1-N5) is named in the message
/// text only; no envelope field is added (D-S11-4).
fn append_load_contribution_absorbed(
    diagnostics: &mut Vec<Diagnostic>,
    case_id: &str,
    report: &LoadFidelityReport,
) {
    let mut refs = vec![case_id.to_string()];
    let mut sources: Vec<&String> = report.rows.iter().flat_map(|row| &row.sources).collect();
    sources.sort();
    sources.dedup();
    refs.extend(sources.into_iter().cloned());
    let rows = report
        .rows
        .iter()
        .map(|row| match row.unaudited {
            Some(reason) => format!(
                "global_dof={} restrained={} unaudited ({reason}); sources={:?}",
                row.global_dof, row.restrained, row.sources
            ),
            None => format!(
                "global_dof={} restrained={} exact_net_bits={:016x} actual_bits={:016x} guarded_ratio={:?} target={:?} operation_count={} completeness_limit={:?}; sources={:?}",
                row.global_dof, row.restrained, row.exact_net_bits, row.actual_bits,
                row.guarded_ratio, row.target, row.operation_count, row.completeness_limit, row.sources
            ),
        })
        .collect::<Vec<_>>();
    let audit = report
        .audit_error
        .as_ref()
        .map(|error| {
            format!("; the load-fidelity audit could not run ({error}); the case is unaudited")
        })
        .unwrap_or_default();
    diagnostics.push(diag(
        &format!("diagnostic:load-fidelity:{}", stable_suffix(case_id)),
        "LOAD_CONTRIBUTION_ABSORBED",
        "warning",
        format!(
            "Load case {case_id}: the load-fidelity audit flagged force rows whose load term differs from the exact net of their identified contributions, or could not audit them; the case is Sensitive and its rows are kept for inspection. Flagged rows: [{}]{audit}",
            rows.join("; ")
        ),
        refs,
    ));
}

/// The integrity code of a structural refusal (shared by the ordinary route
/// and, per ROOT's F1b OQ4, by a non-range failure at W2's chosen b).
fn integrity_failure_code(error: &StructuralError) -> &'static str {
    match error {
        StructuralError::Mechanism { .. } => "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM",
        StructuralError::NegativeEnergy { .. } => "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY",
        StructuralError::Asymmetric { .. } | StructuralError::InvalidInput(_) => {
            "NUMERICAL_INTEGRITY_FAILED"
        }
        StructuralError::NumericallyUnresolved { reason, .. }
            if reason.contains("contribution") || reason.contains("assembly") =>
        {
            "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED"
        }
        _ => "NUMERICAL_INTEGRITY_UNRESOLVED",
    }
}

fn append_integrity_failure(
    diagnostics: &mut Vec<Diagnostic>,
    case_id: &str,
    error: &StructuralError,
    model: &PreviewModel,
) {
    let code = integrity_failure_code(error);
    diagnostics.push(diag(&integrity_diagnostic_id(case_id), code, "blocking", format!("Load case {case_id}: {error}; global_dof_map={:?}; no structural rejection is bypassed by generic LU or output quantization", integrity_dof_map(model)), vec![case_id.to_string()]));
}

// ------------------------------------------------------------ F1b: W2 (range)

/// F1b: the ordinary attempt's failure: the M03 evaluation's error, or K2a's
/// formation `NumericalRange` deferred from the case's basis (a linear
/// invocation only; `form_basis_stiffness`).
#[derive(Debug)]
enum OrdinaryFailure {
    Structural(StructuralError),
    Formation(FrameKernelError),
}

/// F1b (T3 D1 §4.7 step 1; ROOT Q2): the ordinary attempt's range trigger, as
/// K2b's orchestrator classifies its own evaluation at b = 0 (SA
/// `evaluate_force_scaled`): K2a's `NumericalRange` at formation, or
/// `StructuralError::Range` from the evidence or the solve. Every other
/// failure is not a range trigger. (SA's third arm, a force-scaled load-term
/// refusal, cannot occur at b = 0: nothing is scaled there.)
fn ordinary_range_trigger(failure: &OrdinaryFailure) -> Option<RangeTrigger> {
    match failure {
        OrdinaryFailure::Formation(error @ FrameKernelError::NumericalRange { .. }) => {
            Some(RangeTrigger::Formation(error.clone()))
        }
        OrdinaryFailure::Structural(error @ StructuralError::Range(_)) => {
            Some(RangeTrigger::Evaluation(error.clone()))
        }
        _ => None,
    }
}

/// F1b: a case published at b != 0 (T3 D1 §4.7 step 5): every force-unit
/// value the case publishes directly, through K2b's checked helpers, each
/// unscaled once (RV11-1, RV11D-1), and the report's non-normal records.
#[derive(Debug, Clone)]
struct ForceScaledPublication {
    /// b, never 0.
    force_scale_exponent: i32,
    /// (global DOF, value) of each restrained reaction, DOFs ascending.
    reactions: Vec<(usize, PublishedValue)>,
    /// One per `spring_entries` entry, in order.
    spring_actions: Vec<PublishedValue>,
    /// One per `BuiltModel::pipes` entry (no curved span is admitted at b != 0).
    end_actions: Vec<[PublishedValue; ELEMENT_DOF]>,
    /// The element id of each `end_actions` entry.
    members: Vec<String>,
    spring_dofs: Vec<usize>,
    /// `ForceScaledSolution::records`, as the kernel lists them.
    records: Vec<RecordOutcome>,
}

impl ForceScaledPublication {
    /// A full-length reaction vector: the published value at each restrained
    /// DOF (the only DOFs the product reads), +0.0 elsewhere.
    fn reaction_values(&self, dimension: usize) -> Vec<f64> {
        let mut values = vec![0.0; dimension];
        for &(dof, published) in &self.reactions {
            values[dof] = published.value;
        }
        values
    }
}

/// F1b: why W2 did not publish a case.
#[derive(Debug, Clone)]
enum ForceScalingFailure {
    /// The orchestrator's named refusal (§4.7 steps 2-4), with its trigger.
    Refused(ForceScalingRefusal),
    /// A non-range failure at the chosen b, or of the census (RV11-N2). The
    /// orchestrator carries neither b nor the trigger (ROOT OQ1: c2).
    Failed(ForceScaledError),
    /// ROOT Q3: a family not admitted at b != 0.
    NotAdmitted { family: &'static str, b: i32 },
    /// §4.7 step 5: a published value that would underflow or overflow.
    Publication { quantity: String, b: i32 },
    /// Defensive: the orchestrator's step 1 did not reproduce the trigger.
    NotEngaged,
}

/// F1b (T3 D1 §4.7 steps 2-5; ROOT Q2, Q3, Q4): the W2 attempt of a linear
/// case whose ordinary attempt range-triggered and that exact-block did not
/// recover, through K2b's orchestrator (the only W2 entry). The case is formed
/// exactly as the ordinary route formed it, at b = 0, with its unscaled ledger
/// force (RV11-N4: the product never forms a scaled ledger). The orchestrator's
/// refusals take precedence; then admission (Q3); then step-5 publication,
/// before any row is built.
#[allow(clippy::too_many_arguments)]
fn force_scaling_attempt(
    model: &PreviewModel,
    built: &BuiltModel,
    spring_entries: &[SpringEntry],
    restrained_dofs: &[usize],
    load_case: &PreviewLoadCase,
    load_application: &LoadApplication,
    thermal_loads: &[ThermalElementLoad],
    exact_pressure: Option<&pressure_runtime::ExactPressureCase>,
    force: &AssembledForce,
    prescribed: &[(usize, f64)],
    solver_mode: PreviewSolverMode,
) -> Result<(PreviewLinearSolve, ForceScaledPublication), ForceScalingFailure> {
    let curved = built
        .curved_bend_elements
        .iter()
        .map(|e| {
            CurvedBendStiffnessElement::from_macro_element(e.component_id.clone(), &e.macro_element)
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| {
            ForceScalingFailure::Failed(ForceScaledError::Structural(
                StructuralError::InvalidInput("curved formation evidence"),
            ))
        })?;
    let curved_sources = built
        .curved_bend_elements
        .iter()
        .map(|e| e.macro_element)
        .collect::<Vec<_>>();
    let springs = spring_entries
        .iter()
        .map(|e| (e.node_dof.global_index(), e.stiffness.value))
        .collect::<Vec<_>>();
    let mode = match solver_mode {
        PreviewSolverMode::DenseScrutiny => LinearSolveMode::DenseScrutiny,
        PreviewSolverMode::SparseInteractive => LinearSolveMode::SparseInteractive,
    };
    let case = ForceScalingCase {
        node_count: built.nodes.len(),
        frames: &built.frame_elements,
        connectors: &built.connectors,
        curved: &curved,
        curved_sources: &curved_sources,
        springs: &springs,
        force,
        prescribed,
        mode,
        selected: true,
        representation: EvidenceRepresentation::Pattern,
    };
    let outcome = solve_with_force_scaling(&case).map_err(|error| match error {
        ForceScaledError::Refused(refusal) => ForceScalingFailure::Refused(refusal),
        error => ForceScalingFailure::Failed(error),
    })?;
    if outcome.solution.force_scale.is_unscaled() {
        return Err(ForceScalingFailure::NotEngaged);
    }
    let b = outcome.solution.force_scale.exponent();
    force_scaling_admission(
        model,
        built,
        load_case,
        load_application,
        thermal_loads,
        exact_pressure,
        force,
        b,
    )?;
    let publication = force_scaled_publication(
        model,
        built,
        spring_entries,
        restrained_dofs,
        &outcome,
        force,
    )?;
    let solution = &outcome.solution.solution;
    let free = (0..force.len())
        .filter(|dof| !prescribed.iter().any(|&(boundary, _)| boundary == *dof))
        .collect::<Vec<_>>();
    let solve = PreviewLinearSolve {
        structural_report: solution.report.clone(),
        load_fidelity: solution.load_fidelity.clone(),
        formation_check: solution.formation_check.clone(),
        solution: free
            .iter()
            .map(|&dof| solution.displacements[dof])
            .collect(),
        solution_basis: match solver_mode {
            PreviewSolverMode::DenseScrutiny => "dense_structural_integrity_primary",
            PreviewSolverMode::SparseInteractive => "sparse_structural_integrity_primary",
        },
        // ROOT OQ5: the DEC-050/053 observation lanes do not run at b != 0.
        sparse_entry_count: None,
        original_profile_entry_count: None,
        ordered_profile_entry_count: None,
        original_max_half_bandwidth: None,
        ordered_max_half_bandwidth: None,
        nonpositive_pivot_count: None,
        pivot_condition_ratio_estimate: None,
        sparse_residual: None,
        dense_fallback_message: None,
    };
    Ok((solve, publication))
}

/// F1b (ROOT Q3, OQ7, OQ13 narrowed): at b != 0 W2 admits only straight
/// frames, ground springs, rigid restraints, prescribed support motion and
/// authored nodal loads. The first failing check names the family. Every
/// element-targeted primitive (distributed, weight, thermal, pressure) is an
/// `element_uniform_loads` entry, so the thermal and exact-pressure checks
/// precede that one, which names what remains (A2: a
/// thermal load was named `uniform_element_load` in the plan's order). The
/// authored value of a nodal load is not available here (units are normalized
/// in place; a 0.4.0 case's magnitudes are factored), so every exactly-zero
/// nodal term is refused: it cannot be told from one that underflowed at
/// formation (disclosed). Subnormal formed terms are already refused by the
/// census.
#[allow(clippy::too_many_arguments)]
fn force_scaling_admission(
    model: &PreviewModel,
    built: &BuiltModel,
    load_case: &PreviewLoadCase,
    load_application: &LoadApplication,
    thermal_loads: &[ThermalElementLoad],
    exact_pressure: Option<&pressure_runtime::ExactPressureCase>,
    force: &AssembledForce,
    b: i32,
) -> Result<(), ForceScalingFailure> {
    let authored_nodal: HashSet<&str> = load_case
        .primitive_loads
        .iter()
        .filter(|load| matches!(load.target, LoadTargetInput::Node { .. }))
        .map(|load| load.id.as_str())
        .collect();
    let term_is_authored_nodal = |term: &open_pipe_stress_frame_kernel::load_ledger::ForceTerm| {
        matches!(
            term.kind,
            open_pipe_stress_frame_kernel::load_ledger::ForceTermKind::Term(_)
        ) && authored_nodal.contains(term.source.as_str())
    };
    // T4-U3 (S9): a connector model is outside the declared F1b subset.
    let family = if !built.connectors.is_empty() {
        Some("objective_connector")
    } else if !built.curved_bend_elements.is_empty() {
        Some("curved_bend_macro_element")
    } else if !thermal_loads.is_empty() {
        Some("thermal_or_eigen_load")
    } else if exact_pressure.is_some_and(|exact| !exact.assembled_operands.is_empty()) {
        Some("exact_pressure_operand")
    } else if !load_application.element_uniform_loads.is_empty() {
        Some("uniform_element_load")
    } else if constant_effort_solve_dispositions(model)
        .iter()
        .any(|(_, disposition)| disposition.is_ok())
    {
        Some("constant_effort_support")
    } else if !force.terms().iter().all(term_is_authored_nodal) {
        Some("non_nodal_load_term")
    } else if force.terms().iter().any(|term| {
        matches!(
            term.kind,
            open_pipe_stress_frame_kernel::load_ledger::ForceTermKind::Term(value) if value == 0.0
        )
    }) {
        Some("zero_nodal_load_term")
    } else {
        None
    };
    match family {
        Some(family) => Err(ForceScalingFailure::NotAdmitted { family, b }),
        None => Ok(()),
    }
}

/// F1b (T3 D1 §4.7 step 5; RV11-1, RV11D-1): every force-unit value the case
/// publishes directly, at b != 0, through K2b's checked helpers only. A value
/// that would underflow or overflow refuses the case (never flushed, never a
/// wrong `Normal`); a subnormal one carries its precision.
fn force_scaled_publication(
    model: &PreviewModel,
    built: &BuiltModel,
    spring_entries: &[SpringEntry],
    restrained_dofs: &[usize],
    outcome: &ForceScalingOutcome,
    force: &AssembledForce,
) -> Result<ForceScaledPublication, ForceScalingFailure> {
    let scale = outcome.solution.force_scale;
    let b = scale.exponent();
    let u = &outcome.solution.solution.displacements;
    let refused = |error: ForceScaledError, quantity: String| match error {
        ForceScaledError::Refused(ForceScalingRefusal {
            reason: ForceScaleReason::PublicationOutsideBinary64 { .. },
            ..
        }) => ForceScalingFailure::Publication { quantity, b },
        error => ForceScalingFailure::Failed(error),
    };
    let mut dofs = restrained_dofs.to_vec();
    dofs.sort_unstable();
    let reactions = outcome
        .stiffness
        .force_scaled_reactions(u, force, scale, &dofs)
        .map_err(|error| {
            let dof = match &error {
                ForceScaledError::Refused(ForceScalingRefusal {
                    reason: ForceScaleReason::PublicationOutsideBinary64 { global_dof },
                    ..
                }) => *global_dof,
                _ => None,
            };
            let quantity = dof.map_or_else(
                || "reaction".to_string(),
                |dof| format!("reaction@{}", integrity_dof_label(model, dof)),
            );
            refused(error, quantity)
        })?;
    let reactions = dofs.into_iter().zip(reactions).collect::<Vec<_>>();
    let spring_actions = spring_entries
        .iter()
        .map(|spring| {
            let dof = spring.node_dof.global_index();
            force_scaled_spring_action((dof, spring.stiffness.value), u, scale).map_err(|error| {
                refused(
                    error,
                    format!("spring_action@{}", integrity_dof_label(model, dof)),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let end_actions = built
        .pipes
        .iter()
        .map(|pipe| {
            let frame = pipe.frame_element().map_err(|_| {
                ForceScalingFailure::Failed(ForceScaledError::Structural(
                    StructuralError::InvalidInput("straight member frame element"),
                ))
            })?;
            frame
                .force_scaled_end_actions(u, scale)
                .map_err(|error| refused(error, format!("end_actions@{}", pipe.element_id)))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ForceScaledPublication {
        force_scale_exponent: b,
        reactions,
        spring_actions,
        end_actions,
        members: built
            .pipes
            .iter()
            .map(|pipe| pipe.element_id.clone())
            .collect(),
        spring_dofs: spring_entries
            .iter()
            .map(|spring| spring.node_dof.global_index())
            .collect(),
        records: outcome.solution.records.clone(),
    })
}

/// F1b (ROOT Q7, fixed at checkpoint 0): the `range_scaling:` evidence line of
/// a case published at b != 0. The non-normal outcomes follow in DOF order
/// (records, then reactions, then spring actions at each DOF), then member end
/// actions in member and local order; at most S11-G's `NAMED` are named, then
/// `more=<count>`.
fn range_scaling_evidence_line(
    model: &PreviewModel,
    publication: &ForceScaledPublication,
) -> String {
    let precision = |published: &PublishedValue| match published.representability {
        Representability::Subnormal { relative_precision } => Some(relative_precision),
        Representability::Normal => None,
    };
    // (global DOF, kind rank, order within kind, entry)
    let mut located: Vec<(usize, u8, usize, String)> = Vec::new();
    for (index, record) in publication.records.iter().enumerate() {
        let outcome = match record.representability {
            RecordRepresentability::Subnormal { relative_precision } => {
                format!("subnormal(relative_precision={relative_precision:?})")
            }
            RecordRepresentability::Underflow => "underflow".to_string(),
            RecordRepresentability::Overflow => "overflow".to_string(),
        };
        located.push((
            record.global_dof,
            0,
            index,
            format!(
                "record={}@{}:{outcome}",
                record.record,
                integrity_dof_label(model, record.global_dof)
            ),
        ));
    }
    for (dof, published) in &publication.reactions {
        if let Some(p) = precision(published) {
            located.push((
                *dof,
                1,
                0,
                format!(
                    "subnormal=reaction@{}:relative_precision={p:?}",
                    integrity_dof_label(model, *dof)
                ),
            ));
        }
    }
    for (index, (dof, published)) in publication
        .spring_dofs
        .iter()
        .zip(&publication.spring_actions)
        .enumerate()
    {
        if let Some(p) = precision(published) {
            located.push((
                *dof,
                2,
                index,
                format!(
                    "subnormal=spring_action@{}:relative_precision={p:?}",
                    integrity_dof_label(model, *dof)
                ),
            ));
        }
    }
    located.sort_by_key(|(dof, rank, order, _)| (*dof, *rank, *order));
    let mut entries: Vec<String> = located.into_iter().map(|(_, _, _, entry)| entry).collect();
    const COMPONENTS: [&str; DOF_PER_NODE] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];
    for (member, actions) in publication.members.iter().zip(&publication.end_actions) {
        for (index, published) in actions.iter().enumerate() {
            if let Some(p) = precision(published) {
                let end = if index < DOF_PER_NODE { "i" } else { "j" };
                entries.push(format!(
                    "subnormal=end_action@{member}.{end}:{}:relative_precision={p:?}",
                    COMPONENTS[index % DOF_PER_NODE]
                ));
            }
        }
    }
    let named = entries
        .iter()
        .take(formation_guard::NAMED)
        .map(|entry| format!("; {entry}"))
        .collect::<String>();
    let more = if entries.len() > formation_guard::NAMED {
        format!("; more={}", entries.len() - formation_guard::NAMED)
    } else {
        String::new()
    };
    format!(
        "range_scaling: force_scale_exponent={}; basis=exact power-of-two{named}{more}",
        publication.force_scale_exponent
    )
}

/// F1b (ROOT Q6 and OQ1 c2, OQ4; the template fixed at checkpoint 0): a W2
/// refusal, published per case as the case's integrity diagnostic (blocking),
/// modelled on `append_integrity_failure`. It carries the refusal's reason, the
/// ordinary attempt's step-1 trigger (so K2a's names survive) and b where the
/// outcome carries it: `none` where no b exists (steps 2-3), omitted where the
/// orchestrator does not return it (step 4, a non-range failure).
fn append_force_scaling_refusal(
    diagnostics: &mut Vec<Diagnostic>,
    case_id: &str,
    failure: &ForceScalingFailure,
    trigger: &RangeTrigger,
    model: &PreviewModel,
) {
    let exponent = |b: i32| format!("; force_scale_exponent={b}; basis=exact power-of-two");
    let (code, reason, b) = match failure {
        ForceScalingFailure::Refused(refusal) => (
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            refusal.to_string(),
            match refusal.reason {
                ForceScaleReason::SubnormalAtFormation
                | ForceScaleReason::InfeasibleWindow { .. } => {
                    "; force_scale_exponent=none".to_string()
                }
                _ => String::new(),
            },
        ),
        ForceScalingFailure::Failed(error) => (
            match error {
                ForceScaledError::Structural(error) => integrity_failure_code(error),
                _ => "NUMERICAL_INTEGRITY_UNRESOLVED",
            },
            error.to_string(),
            String::new(),
        ),
        ForceScalingFailure::NotAdmitted { family, b } => (
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            format!("range: family not admitted under force scaling: {family}"),
            exponent(*b),
        ),
        ForceScalingFailure::Publication { quantity, b } => (
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            format!(
                "{}; at={quantity}",
                ForceScaleReason::PublicationOutsideBinary64 { global_dof: None }
            ),
            exponent(*b),
        ),
        ForceScalingFailure::NotEngaged => (
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            "range: force scaling did not engage after a range trigger".to_string(),
            String::new(),
        ),
    };
    diagnostics.push(diag(
        &integrity_diagnostic_id(case_id),
        code,
        "blocking",
        format!(
            "Load case {case_id}: {reason}; range_scaling: attempted; step1_trigger={trigger:?}{b}; global_dof_map={:?}; no structural rejection is bypassed by generic LU or output quantization",
            integrity_dof_map(model)
        ),
        vec![case_id.to_string()],
    ));
}

fn assessed_numerical_quality(
    model: &PreviewModel,
    diagnostics: &[Diagnostic],
) -> NumericalQuality {
    let mut quality = unassessed_numerical_quality();
    for case in &model.load_cases {
        let evidence = diagnostics
            .iter()
            .find(|d| d.id == integrity_diagnostic_id(&case.id));
        let (structural_status, solve_quality, model_matrix_fidelity, accuracy_evidence) =
            match evidence.map(|d| d.code.as_str()) {
                Some("NUMERICAL_INTEGRITY_CHECKS_PASSED") => (
                    StructuralStatus::PassiveModelBasis,
                    NumericalQualityStatus::ChecksPassed,
                    ModelMatrixFidelity::RepresentedEquationsRetained,
                    AccuracyEvidence::NotClaimed,
                ),
                Some("NUMERICAL_INTEGRITY_SENSITIVE") => (
                    StructuralStatus::PassiveModelBasis,
                    NumericalQualityStatus::Sensitive,
                    ModelMatrixFidelity::RepresentedEquationsRetained,
                    AccuracyEvidence::NotClaimed,
                ),
                Some("NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM") => (
                    StructuralStatus::PhysicalMechanismWitnessed,
                    NumericalQualityStatus::Failed,
                    ModelMatrixFidelity::NotAssessed,
                    AccuracyEvidence::NotClaimed,
                ),
                Some("NUMERICAL_INTEGRITY_NEGATIVE_ENERGY") => (
                    StructuralStatus::NegativeEnergyWitnessed,
                    NumericalQualityStatus::Failed,
                    ModelMatrixFidelity::NotAssessed,
                    AccuracyEvidence::NotClaimed,
                ),
                Some("NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED") => (
                    StructuralStatus::NumericallyUnresolved,
                    NumericalQualityStatus::Unresolved,
                    ModelMatrixFidelity::NotAssessed,
                    AccuracyEvidence::Unresolved,
                ),
                Some("NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED") => (
                    StructuralStatus::NumericallyUnresolved,
                    NumericalQualityStatus::Unresolved,
                    ModelMatrixFidelity::AssemblyLossDetected,
                    AccuracyEvidence::Unresolved,
                ),
                Some("NUMERICAL_INTEGRITY_FAILED") => (
                    StructuralStatus::NumericallyUnresolved,
                    NumericalQualityStatus::Failed,
                    ModelMatrixFidelity::AssemblyUncertainty,
                    AccuracyEvidence::Unresolved,
                ),
                Some(_) => (
                    StructuralStatus::NumericallyUnresolved,
                    NumericalQualityStatus::Unresolved,
                    ModelMatrixFidelity::AssemblyUncertainty,
                    AccuracyEvidence::Unresolved,
                ),
                None => (
                    StructuralStatus::NumericallyUnresolved,
                    NumericalQualityStatus::NotAssessed,
                    ModelMatrixFidelity::NotAssessed,
                    AccuracyEvidence::NotClaimed,
                ),
            };
        quality.cases.push(NumericalCaseQuality {
            basis_ref: ResultBasisRef {
                ref_type: "load_case".into(),
                ref_id: case.id.clone(),
            },
            structural_status,
            solve_quality,
            model_matrix_fidelity,
            accuracy_evidence,
            evidence_refs: evidence.map(|d| vec![d.id.clone()]).unwrap_or_default(),
        });
    }
    let rank = |status: NumericalQualityStatus| match status {
        NumericalQualityStatus::ChecksPassed => 0,
        NumericalQualityStatus::Sensitive => 1,
        NumericalQualityStatus::NotAssessed => 2,
        NumericalQualityStatus::Unresolved => 3,
        NumericalQualityStatus::Failed => 4,
    };
    quality.status = quality
        .cases
        .iter()
        .map(|case| case.solve_quality)
        .max_by_key(|&status| rank(status))
        .unwrap_or(NumericalQualityStatus::NotAssessed);
    quality
}

pub fn unassessed_numerical_quality() -> NumericalQuality {
    NumericalQuality {
        value_representation: "finite_binary64".to_string(),
        publication_quantization: "none".to_string(),
        integrity_policy: "M03-INTEGRITY-v1".to_string(),
        status: NumericalQualityStatus::NotAssessed,
        cases: Vec::new(),
    }
}

pub fn preview_formulation_basis() -> FormulationBasis {
    FormulationBasis {
        profile_id: "product_preview_mechanics_v1".to_string(),
        limitations: vec![
            "Small-displacement product-preview mechanics; numerical integrity does not establish physical formulation correctness.".to_string(),
            "Pressure is not solved on this profile: legacy pressure inputs are refused, and pressure is solved only on the exact straight-pressure profile, under that profile's own qualifications.".to_string(),
            "Component stiffness, flexibility and stress modifiers depend on declared user inputs and supported component families; no general component qualification is provided.".to_string(),
            "Open-formula stress recovery retains its existing section, station and load-basis limitations; no code compliance result is produced.".to_string(),
            "Support, spring-hanger and nonlinear active-state behavior retain their existing capability and convergence qualifications; numerical precision does not qualify their constitutive models.".to_string(),
            "Protected rule inputs and professional acceptance are not provided; human review remains required.".to_string(),
        ],
    }
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
    #[serde(serialize_with = "serialize_finite_f64")]
    pub value: f64,
    pub unit: String,
    pub location_ref: String,
    pub result_ref: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResultItem {
    pub id: String,
    pub kind: String,
    #[serde(serialize_with = "serialize_finite_f64")]
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

// serde_json otherwise converts nonfinite f64s to null. Even independently
// constructed public quantity rows must retain the finite scientific-value contract.
fn serialize_finite_f64<S: serde::Serializer>(
    value: &f64,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if !value.is_finite() {
        return Err(serde::ser::Error::custom(
            "mechanics quantity must be finite",
        ));
    }
    serializer.serialize_f64(*value)
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
    exact_sections: HashMap<String, pressure_exact::SourceAnnulus>,
    nodes: Vec<FrameNode>,
    pipes: Vec<StraightPipeElement>,
    frame_elements: Vec<FrameElement>,
    /// T4-U3: the objective connectors, in model order (by reference in the
    /// retained census; unpriced).
    connectors: Vec<open_pipe_stress_frame_kernel::connector::ObjectiveConnector>,
    /// T4-U3: each connector's component and replaced span, in the same order.
    connector_records: Vec<joint::ConnectorRecord>,
    curved_bend_elements: Vec<CurvedBendMacroBuild>,
    supports: Vec<LinearSupport>,
    nonlinear_supports: Vec<NonlinearSupport>,
    nonlinear_initial_states: Vec<SupportStateRecord>,
    nonlinear_friction_normal_reactions: Vec<FrictionNormalReaction>,
    nonlinear_derived_friction_normal_reactions: Vec<DerivedFrictionNormalReaction>,
    sections: HashMap<String, DerivedSection>,
}

#[derive(Debug)]
struct LoadCaseSolve {
    load_state_evidence: Option<serde_json::Value>,
    exact_case_evidence: Option<serde_json::Value>,
    pressure_evidence: Vec<serde_json::Value>,
    source_case: Option<source_receipt::FinalizedSourceBlockCase>,
    source_selected: bool,
    load_case_id: String,
    results: Vec<ResultItem>,
    max_displacement: Option<LocatedQuantity>,
    max_stress: Option<LocatedQuantity>,
    component_stress_modifier_count: usize,
    component_pressure_thrust_load_count: usize,
    support_force_vectors: HashMap<String, [f64; 3]>,
    preview: Option<preview_physics::CaseRecord>,
}

#[derive(Debug, Clone)]
struct ThermalElementLoad {
    element_index: usize,
    /// Ledger source id (S11 section 4.2): the primitive load id, or the
    /// resolved eigen source id on the 0.4.0 route.
    source: String,
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

#[derive(Debug, Clone, Copy)]
struct DerivedSection {
    area: f64,
    internal_area: f64,
    second_moment: f64,
    torsion_constant: f64,
    section_modulus: f64,
    torsion_radius: f64,
    wall_thickness: f64,
}

pub fn run_linear_static_preview(request: LinearStaticPreviewRequest) -> MechanicsEnvelope {
    run_linear_static_preview_with_mode(request, PreviewSolverMode::default())
}

pub fn run_linear_static_preview_with_mode(
    request: LinearStaticPreviewRequest,
    solver_mode: PreviewSolverMode,
) -> MechanicsEnvelope {
    // This historical typed entry has no original request-Value custody.
    run_linear_static_preview_captured(request, solver_mode, None, &mut SourceRecoveryBudget::default())
}

/// Capture the exact actual request before parsing. This is the source-block
/// method's invocation boundary; typed reserialization cannot substitute for it.
pub fn run_linear_static_preview_value_with_mode(
    actual_request: serde_json::Value,
    solver_mode: PreviewSolverMode,
) -> Result<MechanicsEnvelope, String> {
    // Shared/native compatibility never selects a retained caller profile.
    run_linear_static_preview_value_dispatch(actual_request, solver_mode, None)
        .map(|outcome| outcome.envelope)
}

/// Ordinary result plus non-wire admission facts. A production permit exists only for a D1 Direct call in the one registered (dev/test) build; any other build is Stale and keeps the ordinary route.
pub struct RetainedPreviewOutput {
    envelope: MechanicsEnvelope,
    admission: Option<RetainedAdmissionReport>,
    /// U3 (D-a): a permitted invocation's W1 result: the checked successor, or
    /// the private cause of its fallback. `None` on every refused or shared call.
    retained: Option<Result<RetainedSuccessor, W1Fallback>>,
}
/// R-1 (ROOT, NUM efde9ca2d1, Proposal A): the one publication of a retained entry.
/// Without a permit it is always `Ordinary`, as in every Stale (unregistered) build.
#[derive(Debug, Clone)]
pub enum RetainedPublication {
    /// The ordinary base, with R-2's unavailable notice when a permitted
    /// invocation's W1 work ran and fell back.
    Ordinary(MechanicsEnvelope),
    /// The precommit-validated retained-precision successor document.
    Successor(serde_json::Value),
}
/// U3 (D-a): the successor document (the envelope with `retained_precision`),
/// staged, serialized and validated before the transfer moved it here.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RetainedSuccessor(serde_json::Value);
impl RetainedSuccessor {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn value(&self) -> &serde_json::Value { &self.0 }
}
/// U3: why a permitted invocation published the untouched ordinary bytes.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum W1Fallback {
    /// No W1 work: a model on no W1 route (a load state or 0.4.0; B3b-P's `w1_route`), or a
    /// request outside W1's case domain (`w1_case_ids`); the ordinary route.
    Domain,
    /// The reserved-stack thread could not be spawned (STACK_PLAN §1).
    StackReservation,
    /// Exact-block selected on the ordinary route (coexistence, D-15).
    Coexistence,
    /// G-B or G-C refused.
    LateGate(retained_memory::PhaseRefusal),
    CompleteGate(retained_memory::PhaseRefusal),
    /// The prepared attempt refused at a stage (one-case: no successor, RR:8436).
    Preparation,
    Native,
    Candidate,
    /// The staging overlay's invariant did not hold (RV85 N6): the site.
    Staging(retained_product::StagingFault),
    /// The serializer refused typed.
    Serializer(retained_wire::ReceiptFailure),
    /// Precommit validation (decision 5): the accepted Rust reader's first failure.
    Precommit { gate: &'static str, code: String },
    /// T-4 (B0 DESIGN_v2 §1.2; decisions 1 and 21): no requested case is in A. Each is
    /// `not_required` by its published verdict, or excluded by DN §4.3, so W1 does not
    /// start: the exact ordinary bytes, no notice and no reservation. Distinct from
    /// `LegacySeed::NotRequired` and from the receipt's `not_required` disposition.
    NoTriggeredCase,
    /// R-2: the notice's space could not be reserved, so W1 did not start.
    NoticeReservation,
    /// Fail-closed guard: the permitted observer no longer held its permit at G-C
    /// (unreachable: `permitted_probe` moves the permit in and nothing takes it).
    PermitUnbound,
    /// B2-P (B2-C §2.6; REVISION_01 N-1, N-5): T-10b abandoned the whole successor: an
    /// `OriginError` or `OriginRefusal` from a combination's custody, a `PreparedCaseSource`
    /// refusal after a completed operand preparation, or a rebuild refusal. Never a capture
    /// error; the notices of the cases in A are published, plain.
    CombinationCustody,
}
impl RetainedPreviewOutput {
    /// The ordinary base: the publication unless `successor()` is present.
    pub fn envelope(&self) -> &MechanicsEnvelope { &self.envelope }
    /// R-1: the successor document, present only when a permitted invocation's
    /// W1 transfer completed. Always `None` without a permit.
    #[doc = "R-1 (RV92 N-6, C-3): a borrowing view of the same successor that `into_publication()` publishes, not a second publication. A caller that publishes this successor does not also publish `envelope()`."] pub fn successor(&self) -> Option<&serde_json::Value> {
        match &self.retained {
            Some(Ok(successor)) => Some(&successor.0),
            _ => None,
        }
    }
    /// R-1: exactly one publication. The ordinary owner drops when a caller takes
    /// the successor.
    pub fn into_publication(self) -> RetainedPublication {
        match self.retained {
            Some(Ok(successor)) => RetainedPublication::Successor(successor.0),
            _ => RetainedPublication::Ordinary(self.envelope),
        }
    }
    /// U3: the permitted invocation's W1 result (see `retained`).
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn retained(&self) -> Option<&Result<RetainedSuccessor, W1Fallback>> { self.retained.as_ref() }
    /// None means the existing pre-parse refusal returned before census entry.
    pub fn admission(&self) -> Option<&RetainedAdmissionReport> { self.admission.as_ref() }
    #[doc = "R-1 (RV92 N-6, C-1): the ordinary base and the admission report only. A successor that a permitted invocation produced is dropped here, never published; take it with `into_publication()`."] pub fn into_parts(self) -> (MechanicsEnvelope, Option<RetainedAdmissionReport>) {
        (self.envelope, self.admission)
    }
}

pub fn run_linear_static_preview_value_with_retained_direct(
    actual_request: serde_json::Value,
    solver_mode: PreviewSolverMode,
) -> Result<RetainedPreviewOutput, String> {
    run_linear_static_preview_value_dispatch(actual_request, solver_mode, Some(retained_memory::Entry::Direct))
}

pub fn run_linear_static_preview_value_with_retained_headless(
    actual_request: serde_json::Value,
    solver_mode: PreviewSolverMode,
    context: RetainedHeadlessContext<'_>,
) -> Result<RetainedPreviewOutput, String> {
    run_linear_static_preview_value_dispatch(actual_request, solver_mode, Some(retained_memory::Entry::Headless(context)))
}

fn run_linear_static_preview_value_dispatch(
    actual_request: serde_json::Value,
    solver_mode: PreviewSolverMode,
    retained_entry: Option<retained_memory::Entry<'_>>,
) -> Result<RetainedPreviewOutput, String> {
    if let Some(refused) = preview_physics::imposed_displacement_refusal(&actual_request) {
        return refused.map(|envelope| RetainedPreviewOutput { envelope, admission: None, retained: None });
    }
    let (request, capture) = source_receipt::CapturedInvocation::parse(actual_request, solver_mode)
        .map_err(|error| error.0)?;
    // G-A (API.md §2): census, then admission, before any ProductCapture can be
    // installed. A refusal is private evidence only; the once-only ordinary route
    // below is unchanged. Only D1 Direct calls in the registered build get a permit.
    let admission = match retained_entry.map(|entry| retained_memory::admit(&capture, &request, entry)) {
        Some(Ok((permit, report))) => return permitted_dispatch(permit, report, request, capture, solver_mode),
        Some(Err(report)) => Some(report),
        None => None,
    };
    ordinary_dispatch(request, &capture, solver_mode, admission, None)
}

/// The unchanged ordinary route of every refused or shared call (and of a
/// permitted call whose reserved-stack thread could not be spawned).
fn ordinary_dispatch(
    request: LinearStaticPreviewRequest,
    capture: &source_receipt::CapturedInvocation,
    solver_mode: PreviewSolverMode,
    admission: Option<RetainedAdmissionReport>,
    retained: Option<Result<RetainedSuccessor, W1Fallback>>,
) -> Result<RetainedPreviewOutput, String> {
    // The composite profile pays for retained-source and physical evidence.
    // This closed resource policy does not change old source-blocks-1 or the
    // exact helper defaults, and never changes the numerical criterion.
    let mut budget = SourceRecoveryBudget::default();
    if pressure_runtime::is_exact(&request.model) {
        budget.per_case_limit = PHYSICS_SOURCE_WORK_LIMIT;
    }
    #[cfg(test)]
    retained_memory::tests::ordinary_dispatch_entered();
    let result = run_linear_static_preview_captured(request, solver_mode, Some(capture), &mut budget);
    if source_finalization_failed(&result) {
        return Err("SOURCE_BLOCKS_FINALIZATION_FAILED".into());
    }
    Ok(RetainedPreviewOutput { envelope: result, admission, retained })
}

fn source_finalization_failed(envelope: &MechanicsEnvelope) -> bool {
    matches!(envelope.producer.semantic_contract_id.as_str(), SOURCE_BLOCKS_SEMANTIC_CONTRACT_ID | PHYSICS_SOURCE_SEMANTIC_CONTRACT_ID | LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID)
        && envelope.source_block_recovery.is_none()
}

fn run_linear_static_preview_captured(
    request: LinearStaticPreviewRequest,
    solver_mode: PreviewSolverMode,
    capture: Option<&source_receipt::CapturedInvocation>,
    source_budget: &mut SourceRecoveryBudget,
) -> MechanicsEnvelope {
    // ROOT CP3 SF-1 (0.4.0 only): an invocation whose selected join cannot
    // finalize never loses its ordinary results. It is republished with every
    // case on its ordinary route (`load-reference-1`, no receipt); each
    // successful retained-source attempt is declined with the recorded cause.
    // The republication continues the same ledger, so the invocation limit
    // bounds all executed work. Pre-0.4 invocations keep their existing
    // behaviour and bytes.
    let fallback_request = (capture.is_some()
        && case_state::is_load_state(&request.model)
        && source_budget.load_state_join_withheld.is_none())
    .then(|| request.clone());
    let envelope =
        run_linear_static_preview_captured_once(request, solver_mode, capture, source_budget);
    match (
        fallback_request,
        source_budget.load_state_join_failure.clone(),
    ) {
        (Some(request), Some(cause)) => {
            let mut fallback = source_budget.withholding_load_state_join(cause);
            let envelope = run_linear_static_preview_captured_once(
                request,
                solver_mode,
                capture,
                &mut fallback,
            );
            *source_budget = fallback;
            envelope
        }
        _ => envelope,
    }
}

fn run_linear_static_preview_captured_once(
    request: LinearStaticPreviewRequest,
    solver_mode: PreviewSolverMode,
    capture: Option<&source_receipt::CapturedInvocation>,
    source_budget: &mut SourceRecoveryBudget,
) -> MechanicsEnvelope {
    run_linear_static_preview_observed(request, solver_mode, capture, source_budget, None)
}

fn run_linear_static_preview_observed(
    request: LinearStaticPreviewRequest,
    solver_mode: PreviewSolverMode,
    capture: Option<&source_receipt::CapturedInvocation>,
    source_budget: &mut SourceRecoveryBudget,
    mut product: Option<&mut retained_product::ProductCapture>,
) -> MechanicsEnvelope {
    // T4-U2a: a v3 invocation is not joined to retained-source recovery.
    let capture = capture.filter(|_| exact_admission::joins_retained_source(&request.model));
    #[cfg(test)] retained_tests_hooks::ordinary_run_entered(); if let Some(observer)=product.as_deref_mut(){observer.invocation(capture,solver_mode);}
    let mut model = request.model;
    let request_materials_supplied = !request.materials.is_empty();
    let mut materials = if request.materials.is_empty() {
        model.materials.clone()
    } else {
        request.materials
    };
    let mut diagnostics = Vec::new();

    pressure_runtime::validate_profile(&model, &mut diagnostics);
    case_state::resolve::validate_document(&model, request_materials_supplied, &mut diagnostics);
    validate_model_inputs(&model, &materials, &mut diagnostics);
    validate_support_family_tokens(&model, &mut diagnostics);
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

    self_weight::validate_applied_self_weight(&model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return blocked_envelope(model, diagnostics);
    }

    resolve_shared_sections(&mut model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return blocked_envelope(model, diagnostics);
    }

    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return blocked_envelope(model, diagnostics);
    }

    let load_state = case_state::is_load_state(&model);
    // A 0.4.0 document never consumes a case-wide base pair: each member's
    // pair comes from its own resolved selection, so only consumed data counts.
    if !load_state {
        pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    }
    if has_blocking(&diagnostics) {
        return blocked_envelope(model, diagnostics);
    }
    if let Some(observer) = product.as_deref_mut() {
        observer.normalized(&model, &materials, request_materials_supplied);
    }
    let resolved_cases = if load_state {
        let resolved = model
            .load_cases
            .iter()
            .filter_map(|case| {
                case_state::resolve::resolve_case(&model, &materials, case, &mut diagnostics)
            })
            .collect::<Vec<_>>();
        if has_blocking(&diagnostics) || resolved.len() != model.load_cases.len() {
            return blocked_envelope(model, diagnostics);
        }
        case_state::resolve::refuse_resolved_span_strain(&model, &resolved, &mut diagnostics);
        if has_blocking(&diagnostics) {
            return blocked_envelope(model, diagnostics);
        }
        Some(resolved)
    } else {
        None
    };
    let built = build_model_for_members(
        &model,
        &materials,
        resolved_cases.as_ref().map(|cases| &cases[0].pairs),
        &mut diagnostics,
    );
    if has_blocking(&diagnostics) {
        return blocked_envelope(model, diagnostics);
    }
    let built = built.expect("build_model returns Some when no blocking diagnostics were added");
    append_constant_effort_consumption_diagnostics(&model, &mut diagnostics);

    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    let potential_contact_dofs = eligible_contact_dofs(
        built.nodes.len(),
        &boundary.restrained_dofs,
        &built.nonlinear_supports,
        &built.nonlinear_initial_states,
    );
    if boundary.restrained_dofs.is_empty()
        && boundary.springs.is_empty()
        && potential_contact_dofs.is_none()
    {
        diagnostics.push(diag(
            "diagnostic:physics:no-restraints",
            "SUPPORT_INPUT_MISSING",
            "blocking",
            "linear-static preview requires explicit support restraints or springs; no automatic boundary conditions are applied",
            vec!["supports".to_string()],
        ));
    }

    // Necessary lower bound only: fewer than six independent ground DOFs
    // cannot remove six rigid-body modes. Springs count; duplicates do not.
    // Passing this check never establishes stability: the assembled solve does.
    let ground_dofs = boundary
        .restrained_dofs
        .iter()
        .copied()
        .chain(
            boundary
                .springs
                .iter()
                .filter(|spring| spring.stiffness.value > 0.0)
                .map(|spring| spring.node_dof.global_index()),
        )
        .chain(potential_contact_dofs.iter().flatten().copied())
        .collect::<HashSet<_>>();
    if !ground_dofs.is_empty() && ground_dofs.len() < DOF_PER_NODE {
        let (restrained, missing) = support_restraint_summary(&boundary.restrained_dofs);
        diagnostics.push(diag("diagnostic:physics:under-restrained", "SOLVER_SYSTEM_BLOCKED", "blocking",
            format!("fewer than six independent ground constraints including positive springs: the six rigid-body modes of a connected structure cannot all be removed; directly restrained global DOF classes: {restrained}; global DOF classes with no direct restraint: {missing} (not a rigid-body mode analysis; separated restraints can resist rotations); support contributions: {}", support_contribution_summary(&model)),
            vec!["supports".to_string()]));
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

    // F1b (T3 D1 §4.8, W3): the kernel's sparse assembly per basis; no dense
    // matrix is held across cases or bases. F1b (§4.7, ROOT Q2): on a linear
    // invocation K2a's range refusal is deferred to the basis's cases.
    let linear = built.nonlinear_supports.is_empty();
    let stiffness = match form_basis_stiffness(&built, &boundary.springs, linear) {
        Ok(stiffness) => stiffness,
        Err(error) => return solver_blocked(model, diagnostics, error),
    };
    // F1b (§4.8 resource guard; ROOT Q8): dense scrutiny refuses, once per
    // invocation and before any n^2 allocation, a model whose dense-path
    // estimate exceeds the provisional ceiling.
    if solver_mode == PreviewSolverMode::DenseScrutiny {
        if let Err(refusal) = dense_scrutiny_guard(
            built.nodes.len() * DOF_PER_NODE,
            dense_scrutiny_ceiling_bytes(),
        ) {
            diagnostics.push(dense_scrutiny_refusal_diagnostic(&refusal));
            return blocked_envelope(model, diagnostics);
        }
    }

    // DEC-068 item 1 + DEC-077: a load case may name an exact user-entered
    // temperature-point id or an explicit solve temperature. Each distinct
    // basis gets its own built model and assembled stiffness from the
    // basis-resolved E and G; temperature bases interpolate E, G, and alpha
    // between adjacent user points and never extrapolate or fall back to base
    // properties.
    let mut basis_solve_states: Vec<(
        Option<String>,
        Vec<MaterialInput>,
        BuiltModel,
        BasisStiffness,
        Option<String>,
    )> = vec![(None, materials.clone(), built, stiffness, None)];
    let mut load_case_solves = Vec::new();
    for (case_index, load_case) in model.load_cases.iter().enumerate() {
        if let Some(resolved_cases) = &resolved_cases {
            // Each resolved case rebuilds stiffness from its own member pairs;
            // no matrix is reused under an unchanged label when inputs differ.
            let resolved = &resolved_cases[case_index];
            let state_index = if case_index == 0 {
                0
            } else {
                let case_built = build_model_for_members(
                    &model,
                    &materials,
                    Some(&resolved.pairs),
                    &mut diagnostics,
                );
                if has_blocking(&diagnostics) {
                    return blocked_envelope(model, diagnostics);
                }
                let case_built = case_built
                    .expect("build_model returns Some when no blocking diagnostics were added");
                let case_stiffness =
                    match form_basis_stiffness(&case_built, &boundary.springs, linear) {
                        Ok(stiffness) => stiffness,
                        Err(error) => return solver_blocked(model, diagnostics, error),
                    };
                basis_solve_states.push((
                    Some(format!("load_state:{}", load_case.id)),
                    materials.clone(),
                    case_built,
                    case_stiffness,
                    None,
                ));
                basis_solve_states.len() - 1
            };
            let (_, basis_materials, basis_built, basis_stiffness, _) =
                &basis_solve_states[state_index];
            match solve_load_case_observed(
                &model,
                basis_built,
                basis_materials,
                basis_stiffness,
                &boundary.restrained_dofs,
                &boundary.springs,
                load_case,
                None,
                solver_mode,
                capture,
                source_budget,
                Some(resolved),
                &mut diagnostics,
                product.as_deref_mut(),
            ) {
                Ok(solve) => load_case_solves.push(solve),
                Err(error) => return solver_blocked(model, diagnostics, error),
            }
            if has_blocking(&diagnostics) {
                return blocked_envelope(model, diagnostics);
            }
            continue;
        }
        let Some(basis_key) = modulus_basis_key(load_case, &mut diagnostics) else {
            if has_blocking(&diagnostics) {
                return blocked_envelope(model, diagnostics);
            }
            let (_, basis_materials, basis_built, basis_stiffness, basis_record) =
                &basis_solve_states[0];
            match solve_load_case_observed(
                &model,
                basis_built,
                basis_materials,
                basis_stiffness,
                &boundary.restrained_dofs,
                &boundary.springs,
                load_case,
                basis_record.as_deref(),
                solver_mode,
                capture,
                source_budget,
                None,
                &mut diagnostics,
                product.as_deref_mut(),
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
                    materials_for_modulus_basis_observed(&model, &materials, load_case, &mut diagnostics, product.as_deref_mut())
                else {
                    return blocked_envelope(model, diagnostics);
                };
                let basis_built = build_model(&model, &basis_materials, &mut diagnostics);
                if has_blocking(&diagnostics) {
                    return blocked_envelope(model, diagnostics);
                }
                let basis_built = basis_built
                    .expect("build_model returns Some when no blocking diagnostics were added");
                let basis_stiffness =
                    match form_basis_stiffness(&basis_built, &boundary.springs, linear) {
                        Ok(stiffness) => stiffness,
                        Err(error) => return solver_blocked(model, diagnostics, error),
                    };
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
        match solve_load_case_observed(
            &model,
            basis_built,
            basis_materials,
            basis_stiffness,
            &boundary.restrained_dofs,
            &boundary.springs,
            load_case,
            basis_record.as_deref(),
            solver_mode,
            capture,
            source_budget,
            None,
            &mut diagnostics,
            product.as_deref_mut(),
        ) {
            Ok(solve) => load_case_solves.push(solve),
            Err(error) => return solver_blocked(model, diagnostics, error),
        }
        if has_blocking(&diagnostics) {
            return blocked_envelope(model, diagnostics);
        }
    }
    let built = &basis_solve_states[0].2;
    let source_selected = load_case_solves.iter().any(|solve| solve.source_selected);
    let source_cases: Vec<_> = load_case_solves.iter_mut().filter_map(|solve| solve.source_case.take()).collect();
    let preview = (!pressure_runtime::is_exact(&model) && !source_selected).then(|| {
        preview_physics::render(&model, built, &boundary.restrained_dofs, &mut load_case_solves, &mut diagnostics)
    });
    let max_displacement = if pressure_runtime::is_exact(&model) || source_selected || preview.is_some() {
        maximum_across_cases(&load_case_solves, false)
    } else {
        load_case_solves.first().and_then(|solve| solve.max_displacement.clone())
    };
    let max_stress = if pressure_runtime::is_exact(&model) || source_selected || preview.is_some() {
        maximum_across_cases(&load_case_solves, true)
    } else {
        load_case_solves.first().and_then(|solve| solve.max_stress.clone())
    };
    let component_stress_modifier_count = load_case_solves
        .iter()
        .map(|solve| solve.component_stress_modifier_count)
        .sum();
    let component_pressure_thrust_load_count = load_case_solves
        .iter()
        .map(|solve| solve.component_pressure_thrust_load_count)
        .sum();
    let mut results = Vec::new();
    let mut pressure_evidence = Vec::new();
    let mut exact_cases = Vec::new();
    let mut load_reference_states = Vec::new();
    let mut rows_by_base_id: HashMap<String, HashMap<String, ResultItem>> = HashMap::new();
    let mut support_vectors_by_case = HashMap::new();
    for (index, solve) in load_case_solves.into_iter().enumerate() {
        pressure_evidence.extend(solve.pressure_evidence.iter().cloned());
        load_reference_states.extend(solve.load_state_evidence.clone());
        if let Some(mut evidence) = solve.exact_case_evidence.clone() {
            if source_selected {
                evidence["recovery_method"] = serde_json::json!(if solve.source_selected {
                    "retained_source_blocks_exact_v1"
                } else if solver_mode == PreviewSolverMode::DenseScrutiny {
                    "ordinary_dense_structural_v1"
                } else { "ordinary_sparse_structural_v1" });
            }
            exact_cases.push(evidence);
        }
        let is_default = index == 0;
        let load_case_id = solve.load_case_id.clone();
        support_vectors_by_case.insert(load_case_id.clone(), solve.support_force_vectors);
        let id_map = solve
            .results
            .iter()
            .map(|row| {
                (
                    row.id.clone(),
                    if is_default || ((pressure_runtime::is_exact(&model) || preview.is_some()) && row.kind.ends_with("_v2")) {
                        row.id.clone()
                    } else {
                        qualified_load_case_result_id(&load_case_id, &row.id)
                    },
                )
            })
            .collect::<HashMap<_, _>>();
        for mut result in solve.results {
            let base_id = result.id.clone();
            result.basis_ref = Some(ResultBasisRef {
                ref_type: "load_case".to_string(),
                ref_id: load_case_id.clone(),
            });
            if !is_default && !((pressure_runtime::is_exact(&model) || preview.is_some()) && result.kind.ends_with("_v2")) {
                result.id = qualified_load_case_result_id(&load_case_id, &base_id);
            }
            for source_ref in &mut result.source_result_refs {
                if let Some(qualified) = id_map.get(source_ref) {
                    *source_ref = qualified.clone();
                }
            }
            rows_by_base_id
                .entry(base_id)
                .or_default()
                .insert(load_case_id.clone(), result.clone());
            results.push(result);
        }
    }
    let preview_gates = match &preview {
        Some(rendered) => preview_physics::append_combination_results(&model, rendered, &rows_by_base_id, &mut results, &mut diagnostics),
        None => {
            append_combination_results(
                &model,
                &rows_by_base_id,
                &support_vectors_by_case,
                &mut results,
                &mut diagnostics,
            );
            Vec::new()
        }
    };
    append_combination_modulus_basis_records(&model, &mut results);
    // H-4: the key is kept; since T4-U3 it counts the curved macro-element rows
    // only (the joint review rows are no longer produced).
    let component_user_stiffness_macro_element_count =
        append_curved_bend_macro_element_results(
            &built.curved_bend_elements,
            pressure_runtime::exact_contract(&model) == Some(pressure_runtime::ExactContract::PressureV3),
            &mut results,
        );
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

    if let Err(error) = require_finite_mechanics(
        results
            .iter()
            .map(|row| row.value)
            .chain(max_displacement.iter().map(|q| q.value))
            .chain(max_stress.iter().map(|q| q.value)),
    ) {
        return solver_blocked(model, diagnostics, error);
    }
    // Publish finite computed binary64 quantities without an absolute decimal quantum.
    let composite = source_selected && pressure_runtime::is_exact(&model);
    let joined_load_state = composite && case_state::is_load_state(&model);
    let mut envelope = MechanicsEnvelope {
        contract_evidence: pressure_runtime::is_exact(&model).then(|| {
            let mut evidence = serde_json::json!({"pressure": pressure_evidence, "connector": joint::connector_evidence(&model), "exact_cases": exact_cases});
            if case_state::is_load_state(&model) {
                evidence["load_reference_states"] = serde_json::json!(load_reference_states);
            }
            evidence
        }),
        schema_version: MECHANICS_SCHEMA_VERSION.to_string(),
        producer: mechanics_producer_for_model(&model),
        numerical_quality: assessed_numerical_quality(&model, &diagnostics),
        source_block_recovery: None,
        formulation_basis: formulation_basis_for_model(&model),
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
    };
    if let Some(rendered) = &preview {
        envelope.producer.semantic_contract_id = preview_physics::ID.into();
        envelope.formulation_basis = preview_physics::formulation_basis();
        envelope.contract_evidence = Some(rendered.evidence(preview_gates));
    }
    if source_selected {
        envelope.producer.semantic_contract_id = if joined_load_state {
            // The joined envelope has its own semantics and exclusive profile.
            envelope.formulation_basis = joined_load_state_formulation_basis();
            LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID
        } else if composite { PHYSICS_SOURCE_SEMANTIC_CONTRACT_ID } else { SOURCE_BLOCKS_SEMANTIC_CONTRACT_ID }.into();
        if let Some(capture) = capture {
            let finalized = if composite {
                source_receipt::FinalizedSourceBlockReceipt::finalize_composite(capture, &envelope, source_cases, source_budget)
            } else {
                source_receipt::FinalizedSourceBlockReceipt::finalize(capture, &envelope, source_cases, source_budget)
            };
            match finalized {
                Ok(receipt) => envelope.source_block_recovery = Some(receipt.into_wire()),
                Err(error) => {
                    if joined_load_state {
                        source_budget.record_load_state_join_failure(format!(
                            "invocation receipt: {}",
                            error.0
                        ));
                    }
                    envelope.diagnostics.push(diag("diagnostic:source-recovery:publication", "SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED", "blocking", format!("{}; actual_finalization_work={:?}", error.0, error.1), vec![]))
                }
            }
        }
    }
    if let Some(observer) = product.as_deref_mut() { observer.finish(&envelope); }
    envelope
}

/// U3: a permitted retained invocation (API.md §3). Everything after G-A runs on
/// one scoped thread with the permit's reserved stack (STACK_PLAN §1, D-3 = S1):
/// the single observed ordinary run, G-B (inside the capture), G-C and every W1
/// phase. A spawn failure runs the unchanged ordinary route on this thread.
/// The permit is linear (U3 grant 1b): it moves onto the reserved-stack thread and
/// into the observer, and drops with the work, or unrun with it on a spawn failure.
/// Only D1 Direct calls in the registered dev/test build reach it (M = 4,026,531,840 B, D-7).
fn permitted_dispatch(
    permit: retained_memory::CapturePermit,
    report: RetainedAdmissionReport,
    request: LinearStaticPreviewRequest,
    capture: source_receipt::CapturedInvocation,
    solver_mode: PreviewSolverMode,
) -> Result<RetainedPreviewOutput, String> {
    let bytes = permit.reserved_stack_bytes();
    let mut slot = Some(request);
    let (pending, captured) = (&mut slot, &capture);
    let ran = on_reserved_stack(bytes, carry_test_hooks(move || {
        pending.take().map(|request| permitted_run(permit, report, request, captured, solver_mode))
    }));
    #[cfg(test)]
    retained_tests_hooks::reclaim_handed_back();
    match (ran.flatten(), slot) {
        (Some(result), _) => result,
        (None, Some(request)) => ordinary_dispatch(request, &capture, solver_mode, Some(report), Some(Err(W1Fallback::StackReservation))),
        (None, None) => Err("retained dispatch lost its request".into()),
    }
}

/// STACK_PLAN §1: run `work` on one scoped thread with `bytes` of reserved stack,
/// borrowing the caller's owners. `None` when the thread cannot be spawned (the
/// work did not run). A panic is re-raised here with its original payload.
fn on_reserved_stack<T: Send>(bytes: usize, work: impl FnOnce() -> T + Send) -> Option<T> {
    std::thread::scope(|scope| {
        match std::thread::Builder::new().stack_size(bytes).spawn_scoped(scope, work) {
            Ok(handle) => Some(match handle.join() {
                Ok(value) => value,
                Err(payload) => std::panic::resume_unwind(payload),
            }),
            Err(_) => None,
        }
    })
}

/// Production: the work itself. Test builds: the caller thread's armed fault hooks
/// (thread-local) move with the work onto the reserved-stack thread, so a committed
/// fault test cannot pass vacuously there (ROOT's flag on grant 1). Faults that did
/// not fire there are handed back to the caller (RV85 U2): after the work, or as the
/// unrun work drops on a spawn failure; the caller re-arms them with
/// `reclaim_handed_back`.
#[cfg(not(test))]
fn carry_test_hooks<F>(work: F) -> F { work }
#[cfg(test)]
fn carry_test_hooks<T, F: FnOnce() -> T + Send>(work: F) -> impl FnOnce() -> T + Send {
    let carried = retained_tests_hooks::Carried::take();
    move || {
        let caller = carried.install();
        let value = work();
        retained_tests_hooks::hand_back(caller);
        value
    }
}

/// The permitted run on the reserved-stack thread: the single observed ordinary
/// run with capture installed, G-C, then the W1 phases.
fn permitted_run(
    permit: retained_memory::CapturePermit,
    report: RetainedAdmissionReport,
    request: LinearStaticPreviewRequest,
    capture: &source_receipt::CapturedInvocation,
    solver_mode: PreviewSolverMode,
) -> Result<RetainedPreviewOutput, String> {
    // B3b-P (B3-D P-1): the route, decided once from the admitted model's namespace branch.
    // Defensively, a permit for a model on no W1 route (load states, 0.4.0) takes the unchanged
    // ordinary route (with its SF-1 logic).
    let Some(route) = w1_route(&request.model) else {
        return ordinary_dispatch(request, capture, solver_mode, Some(report), Some(Err(W1Fallback::Domain)));
    };
    // B3b-P (P-2): the route's exact-block budget, as the ordinary route's.
    let mut budget = w1_budget(route);
    // B1 seam (PLAN_v2 §2.1; RV107 SF-4): T-3 (e)'s requested count, read before the
    // request moves into the observed run. G-C carries it, and its attempt fact counts the
    // requested cases (B1 SA, T-3 (e)).
    let requested_cases = request.model.load_cases.len();
    // The permit moves into the observer, which checks G-B with it.
    let mut observer = retained_product::ProductCapture::permitted_probe_on(permit, route);
    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));
    if source_finalization_failed(&ordinary) {
        return Err("SOURCE_BLOCKS_FINALIZATION_FAILED".into());
    }
    // RV85 N1 (I51 COMPOSITION §2): G-C follows only a settled ordinary run whose
    // exact-block arbitration left W1 open and whose late capture G-B authorized;
    // otherwise its true cause is recorded and G-C is never consulted.
    let (envelope, retained) = if ordinary.source_block_recovery.is_some() {
        (ordinary, Err(W1Fallback::Coexistence))
    } else if let Some(refusal) = observer.late_refusal().cloned() {
        (ordinary, Err(W1Fallback::LateGate(refusal)))
    } else {
        #[cfg(test)] retained_tests_hooks::at_complete_gate(&mut observer); // G-C, with the observer's own permit.
        match observer.permit().map(|permit| permit.check_complete(&retained_memory::CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases })) {
            Some(Ok(())) => retained_w1(observer, ordinary, capture),
            Some(Err(refusal)) => (ordinary, Err(W1Fallback::CompleteGate(refusal))),
            None => (ordinary, Err(W1Fallback::PermitUnbound)),
        }
    };
    Ok(RetainedPreviewOutput { envelope, admission: Some(report), retained: Some(retained) })
}

/// B3b-P (B3-D P-1; I95's ruling 1): an admitted model's W1 route, decided once, from its D1.3
/// namespace branch: L takes the preview route (preview-physics-1 base), E the exact route
/// (physics-1 base, `physics-retained-1`). `None` (no W1: `Domain`) for a load-state model and
/// any model on no branch (0.4.0 included).
fn w1_route(model: &PreviewModel) -> Option<retained_product::W1Route> {
    use retained_memory::NamespaceBranch as B;
    if case_state::is_load_state(model) {
        return None;
    }
    match retained_memory::namespace_branch(model) {
        Ok(B::Legacy) => Some(retained_product::W1Route::Preview),
        Ok(B::Exact) => Some(retained_product::W1Route::Exact),
        Err(_) => None,
    }
}

/// B3b-P (B3-D P-2; RR "I99's B3-W verified; …", ruling 2): W1's exact-block budget is the
/// ordinary route's (`ordinary_dispatch`): `PHYSICS_SOURCE_WORK_LIMIT` per case on the exact
/// route, the default elsewhere. physics-source-1 then selects on the Direct entry exactly as on
/// the ordinary route, so T-3 (c) publishes the exact ordinary bytes, and the receipt's
/// `legacy_source_work[].limit` is that budget.
fn w1_budget(route: retained_product::W1Route) -> SourceRecoveryBudget {
    let mut budget = SourceRecoveryBudget::default();
    if route == retained_product::W1Route::Exact {
        budget.per_case_limit = PHYSICS_SOURCE_WORK_LIMIT;
    }
    budget
}

/// R-2 (ROOT, NUM efde9ca2d1, N1): the fixed product text of the base publication's
/// unavailable notice. No receipt reference: the base publication carries none.
pub(crate) const RETAINED_UNAVAILABLE_NOTICE: &str = "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics.";
/// C1:68: the reason a receipt-encoding fallback states, before its detail token.
const RECEIPT_ENCODING_REASON: &str = " Reason: receipt_encoding; detail: ";
/// The longest C1:68 detail token (`work_counter_inconsistent`).
const RECEIPT_ENCODING_DETAIL_MAX: usize = 25;

/// C1:68's receipt-encoding detail for a serializer refusal, from the accepted
/// `ReceiptCheck::wire()` token: only the details C1:68 names (the three work-counter
/// tokens, and `publication_hash_range` "similarly"). An association or untranslated
/// refusal is not a receipt-encoding fallback and states no reason.
fn receipt_encoding_detail(check: retained_wire::ReceiptCheck) -> Option<&'static str> {
    match check.wire() {
        token @ ("work_counter_range" | "work_counter_inconsistent" | "saturation_not_excluded" | "publication_hash_range") => Some(token),
        _ => None,
    }
}

/// R-2 (N1): one case's notice, whose space is reserved before any W1 work starts
/// (COMP:66), so that rendering it after a fallback allocates nothing.
struct ReservedNotice(Diagnostic);
impl ReservedNotice {
    /// The longest message and the diagnostic, for an id already checked against the base.
    /// `None` (W1 does not start) if the message's reservation fails.
    fn reserve(id: String, case_id: &str) -> Option<Self> {
        let mut message = String::new();
        message.try_reserve_exact(RETAINED_UNAVAILABLE_NOTICE.len() + RECEIPT_ENCODING_REASON.len() + RECEIPT_ENCODING_DETAIL_MAX + 1).ok()?;
        message.push_str(RETAINED_UNAVAILABLE_NOTICE);
        #[cfg(test)]
        assert!(message.capacity() >= RETAINED_UNAVAILABLE_NOTICE.len() + RECEIPT_ENCODING_REASON.len() + RECEIPT_ENCODING_DETAIL_MAX + 1);
        Some(Self(Diagnostic {
            id,
            code: retained_wire::UNAVAILABLE_CODE.to_owned(),
            severity: "info".to_owned(),
            message,
            source: Some("core/product_physics".to_owned()),
            affected_refs: vec![case_id.to_owned()],
        }))
    }
}
/// B1 SP (DESIGN_v2 T-5; R-2 per case): the notices of the cases in A, in request order,
/// all reserved before any W1 work starts. At c = 1 this is R-2's one notice.
struct ReservedNotices([Option<ReservedNotice>; retained_memory::caps::LOAD_CASES]);
impl ReservedNotices {
    /// One diagnostic slot per case in the ordinary owner, and each case's longest message.
    /// `None` (W1 does not start) if any reservation fails, or any notice's id is already in
    /// the base or is another case's notice id.
    fn reserve(ordinary: &mut MechanicsEnvelope, case_ids: &[&str]) -> Option<Self> {
        let mut ids: [String; retained_memory::caps::LOAD_CASES] = Default::default();
        if case_ids.is_empty() || case_ids.len() > ids.len() {
            return None;
        }
        for (k, case_id) in case_ids.iter().enumerate() {
            let id = format!("diagnostic:retained-precision:{case_id}:unavailable");
            if ordinary.diagnostics.iter().any(|d| d.id == id) || ids[..k].contains(&id) {
                return None;
            }
            ids[k] = id;
        }
        ordinary.diagnostics.try_reserve_exact(case_ids.len()).ok()?;
        let mut notices: [Option<ReservedNotice>; retained_memory::caps::LOAD_CASES] = std::array::from_fn(|_| None);
        for ((slot, id), case_id) in notices.iter_mut().zip(ids).zip(case_ids) {
            *slot = Some(ReservedNotice::reserve(id, case_id)?);
        }
        Some(Self(notices))
    }
    /// T-12: append the notices after the ordinary prefix, in request order, for a fallback
    /// after W1 work ran. Within the reserved capacities: no allocation. C1:68's receipt-encoding
    /// detail (a serializer refusal with one of its details) goes only on the notices of the
    /// cases that were selected when the successor was abandoned: bit `k` of `selected` is the
    /// `k`-th case in A (decision 6). Every other notice is plain.
    fn publish(self, mut ordinary: MechanicsEnvelope, cause: W1Fallback, selected: u64) -> (MechanicsEnvelope, Result<RetainedSuccessor, W1Fallback>) {
        // RV85 T1: rendering and appending allocate nothing (the capacities reserved
        // before W1 are the capacities published).
        #[cfg(test)]
        let reserved = ordinary.diagnostics.capacity();
        let detail = match &cause {
            W1Fallback::Serializer(failure) => receipt_encoding_detail(failure.check),
            _ => None,
        };
        for (k, ReservedNotice(mut notice)) in self.0.into_iter().flatten().enumerate() {
            #[cfg(test)]
            let message = notice.message.capacity();
            let was_selected = u32::try_from(k).ok().and_then(|k| selected.checked_shr(k)).is_some_and(|bits| bits & 1 == 1);
            if let Some(detail) = detail.filter(|_| was_selected) {
                notice.message.push_str(RECEIPT_ENCODING_REASON);
                notice.message.push_str(detail);
                notice.message.push('.');
            }
            #[cfg(test)]
            assert!(ordinary.diagnostics.len() < ordinary.diagnostics.capacity() && notice.message.capacity() == message,
                "the notice's space was reserved before W1: rendering allocated nothing");
            ordinary.diagnostics.push(notice);
        }
        #[cfg(test)]
        assert_eq!(ordinary.diagnostics.capacity(), reserved, "appending the notices allocated nothing");
        (ordinary, Err(cause))
    }
}

/// B1 SP: requested cases (at most `caps::LOAD_CASES`), by request index and id, in request
/// order, with no heap allocation.
struct CaseSet<'a> {
    requests: [usize; retained_memory::caps::LOAD_CASES],
    ids: [&'a str; retained_memory::caps::LOAD_CASES],
    len: usize,
}
impl<'a> CaseSet<'a> {
    fn new() -> Self { Self { requests: [0; retained_memory::caps::LOAD_CASES], ids: [""; retained_memory::caps::LOAD_CASES], len: 0 } }
    /// Append a case; `None` past `caps::LOAD_CASES`.
    fn push(&mut self, request: usize, id: &'a str) -> Option<()> {
        *self.requests.get_mut(self.len)? = request;
        self.ids[self.len] = id;
        self.len = self.len.checked_add(1)?;
        Some(())
    }
    fn requests(&self) -> &[usize] { &self.requests[..self.len] }
    fn ids(&self) -> &[&'a str] { &self.ids[..self.len] }
    /// T-4's A: the cases whose trigger is `Attempted`, in request order.
    fn attempted(&self, triggers: impl Iterator<Item = CaseTrigger>) -> Self {
        let mut attempted = Self::new();
        for ((&request, &id), trigger) in self.requests().iter().zip(self.ids()).zip(triggers) {
            if trigger == CaseTrigger::Attempted {
                attempted.push(request, id).expect("A is a subset of the requested cases");
            }
        }
        attempted
    }
}

/// B1 SP (PLAN_v2 §2.2; RV107 SF-2), widened by B2-P (B2-C §2.1 T-4, §9): the request's
/// cases, in request order, when W1 may run: 1 ≤ c ≤ `caps::LOAD_CASES` (D1.4) and the
/// combinations D1.4 admits (`w1_combinations_admitted`). `None` otherwise, where W1 never
/// starts (`Domain`). With no combination this is B1's re-check, unchanged.
fn w1_case_ids(capture: &source_receipt::CapturedInvocation) -> Option<CaseSet<'_>> {
    let model = &capture.borrowed_raw()["model"];
    let cases = model["load_cases"].as_array()?;
    let combinations = model["combinations"].as_array().map_or(&[][..], Vec::as_slice);
    if cases.is_empty() || cases.len() > retained_memory::caps::LOAD_CASES {
        return None;
    }
    let mut set = CaseSet::new();
    for (request, case) in cases.iter().enumerate() {
        set.push(request, case["id"].as_str()?)?;
    }
    if combinations.iter().any(|combination| combination["id"].as_str().is_none()) {
        return None;
    }
    let shapes = combinations.iter().map(|combination| (
        combination["id"].as_str().unwrap_or_default(),
        combination["terms"].as_array().map_or(0, Vec::len),
        combination["operand_ids"].as_array().map_or(0, Vec::len),
    ));
    w1_combinations_admitted(set.ids().iter().copied(), shapes).then_some(set)
}

/// B2-P (B2-C §9; REVISION_01 S-1): D1.4's combination clauses as W1 reads them, the one
/// predicate shared by T-4's domain re-check (`w1_case_ids`) and T-2's capture hooks. With
/// z ≥ 1 combinations: z ≤ `caps::COMBINATIONS`; C_eq = c + z ≤ `caps::CASE_EQUIVALENTS`; each
/// combination's terms (repeats counted) ≤ `caps::COMBINATION_TERMS` and range operand ids ≤
/// `caps::RANGE_OPERANDS`; and no combination id equal to a load-case id (C-9). Each
/// combination is `(id, terms, range operand ids)`. With no combination it holds: z = 0 is
/// B1's domain, decided by the case clauses alone.
pub(crate) fn w1_combinations_admitted<'a>(
    case_ids: impl Iterator<Item = &'a str> + Clone,
    combinations: impl ExactSizeIterator<Item = (&'a str, usize, usize)>,
) -> bool {
    use retained_memory::caps;
    let z = combinations.len();
    if z == 0 {
        return true;
    }
    let c = case_ids.clone().count();
    if z > caps::COMBINATIONS || c.checked_add(z).is_none_or(|equivalents| equivalents > caps::CASE_EQUIVALENTS) {
        return false;
    }
    let mut combinations = combinations;
    combinations.all(|(id, terms, operands)| {
        terms <= caps::COMBINATION_TERMS && operands <= caps::RANGE_OPERANDS && !case_ids.clone().any(|case| case == id)
    })
}

/// `w1_combinations_admitted` over a typed model (T-2's capture hooks).
pub(crate) fn w1_model_combinations_admitted(model: &PreviewModel) -> bool {
    w1_combinations_admitted(
        model.load_cases.iter().map(|case| case.id.as_str()),
        model.combinations.iter().map(|combination| (
            combination.id.as_str(),
            combination.terms.len(),
            combination.operand_ids.as_ref().map_or(0, Vec::len),
        )),
    )
}

/// T-4 (B0 DESIGN_v2 §1.2; decisions 1 and 21): one requested case's W1 trigger class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CaseTrigger {
    /// Its published verdict is `checks_passed`: no product attempt and no notice.
    NotRequired,
    /// DN §4.3 (decision 21): an ordinary Mechanism, Asymmetric or InvalidInput failure
    /// that W2 did not publish. No product attempt and no notice; it stays an ordinary
    /// failed case.
    Excluded,
    /// In A: W1 attempts the case. Every other verdict, an absent quality entry, and a
    /// case with no seed (RR "I81's B1-0 probe verified…", ruling 2).
    Attempted,
}

/// T-4's classifier over the requested case ids, in request order (n cases; D1.4
/// admits one until B1's SA). A case's published verdict is the one
/// `numerical_quality.cases[]` entry whose `basis_ref.ref_id` is the case id, never an
/// entry found by position (as `ordinary_value` binds it); its seed is the one seed
/// whose `case` is the case id. An entry or a seed that is absent, or not unique, is
/// absent: such a case is in A unless its verdict is `checks_passed`.
fn case_triggers<'a>(
    quality: &'a NumericalQuality,
    seeds: &'a [retained_product::OrdinarySeed],
    case_ids: &'a [&'a str],
) -> impl Iterator<Item = CaseTrigger> + 'a {
    case_ids.iter().map(move |&case_id| {
        let verdict = only_one(quality.cases.iter().filter(|entry| entry.basis_ref.ref_id == case_id)).map(|entry| entry.solve_quality);
        let seed = only_one(seeds.iter().filter(|seed| seed.case == case_id));
        if verdict == Some(NumericalQualityStatus::ChecksPassed) {
            CaseTrigger::NotRequired
        } else if seed.is_some_and(dn_trigger_excluded) {
            CaseTrigger::Excluded
        } else {
            CaseTrigger::Attempted
        }
    })
}

/// The one item, or `None` when there is none or more than one.
fn only_one<T>(mut items: impl Iterator<Item = T>) -> Option<T> {
    match (items.next(), items.next()) {
        (Some(item), None) => Some(item),
        _ => None,
    }
}

/// Decision 21 (DN §4.3): the trigger never fires for an ordinary attempt that failed
/// as a Mechanism, Asymmetric or InvalidInput, unless W2 published the case.
fn dn_trigger_excluded(seed: &retained_product::OrdinarySeed) -> bool {
    matches!(&seed.initial, Some(retained_product::InitialSeed::StructuralFailure {
        error: StructuralError::Mechanism { .. } | StructuralError::Asymmetric { .. } | StructuralError::InvalidInput(_), ..
    })) && !matches!(seed.w2, retained_product::W2Seed::Published { .. })
}

/// U3: the W1 phases over the single actual ordinary run's capture: coexistence,
/// G-B's outcome, the domain re-check and T-4's trigger (B1), the notices' reservation (T-5),
/// custody (T-6), per-case preparation (T-7), native, proof (the frozen
/// candidate), staging, serialization, precommit validation and the transfer. The
/// ordinary envelope's bytes are never changed by W1: it returns on every fallback
/// (D-b), with R-2's notice appended when W1 work ran, and beside the successor on
/// success. No fallible step follows the transfer's first move.
fn retained_w1(
    observer: retained_product::ProductCapture,
    mut ordinary: MechanicsEnvelope,
    capture: &source_receipt::CapturedInvocation,
) -> (MechanicsEnvelope, Result<RetainedSuccessor, W1Fallback>) {
    if ordinary.source_block_recovery.is_some() {
        return (ordinary, Err(W1Fallback::Coexistence));
    }
    if let Some(refusal) = observer.late_refusal() {
        let refusal = refusal.clone();
        return (ordinary, Err(W1Fallback::LateGate(refusal)));
    }
    // B1 SP, B2-P: the request's cases; outside D1.4 (c = 0, c > C, or a combination D1.4
    // does not admit: z > 2, C_eq > 3, h > 3, a range over > 3 ids, a shared id) W1 never starts.
    let Some(cases) = w1_case_ids(capture) else {
        return (ordinary, Err(W1Fallback::Domain));
    };
    // T-4 (decisions 1 and 21): W1 attempts only the cases in A. With A empty, the exact
    // ordinary bytes: no notice, and nothing reserved.
    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, cases.ids()).any(|trigger| trigger == CaseTrigger::Attempted) {
        return (ordinary, Err(W1Fallback::NoTriggeredCase));
    }
    let attempted = cases.attempted(case_triggers(&ordinary.numerical_quality, &observer.ordinary, cases.ids()));
    // T-5 (R-2 per case): every case in A's notice is reserved before any W1 work starts.
    let Some(notice) = ReservedNotices::reserve(&mut ordinary, attempted.ids()) else {
        return (ordinary, Err(W1Fallback::NoticeReservation));
    };
    // T-6 to T-11, then T-12 on any abandonment.
    match w1_transaction(observer, ordinary, capture, cases.ids().len(), attempted.requests()) {
        // The transfer: moves only. The unused notices drop; the reserved slots are not
        // bytes of the ordinary publication.
        (ordinary, Ok(successor)) => {
            drop(notice);
            (ordinary, Ok(RetainedSuccessor(successor)))
        }
        (ordinary, Err((cause, selected))) => notice.publish(ordinary, cause, selected),
    }
}

/// B1 SP (DESIGN_v2 T-6 to T-11): the transaction over the `requested` cases, with A's
/// request indices `attempted`, in request order. Custody once (T-6); one product attempt per
/// case in A (T-7); one `CaseBatchCall` over the prepared cases (T-8); one freeze per selected
/// Run (T-9); with no case selected, T-10's fallback; otherwise staging, the serializer and
/// precommit (T-11). It returns the untouched ordinary owner with the successor, or with the
/// abandonment's cause and which cases in A were selected then (bit `k`: the `k`-th case in A).
/// With no case selected the cause is the furthest stage any case reached: `Candidate` (T-9),
/// `Native` (T-8, also a failure of the call itself) or `Preparation` (T-6, T-7). At c = 1
/// these are the one-case transaction's causes and bytes.
fn w1_transaction(
    observer: retained_product::ProductCapture,
    ordinary: MechanicsEnvelope,
    capture: &source_receipt::CapturedInvocation,
    requested: usize,
    attempted: &[usize],
) -> (MechanicsEnvelope, Result<serde_json::Value, (W1Fallback, u64)>) {
    use retained_product::AttemptEnd;
    // T-6, then T-7: invocation custody once; then one product attempt per case in A.
    let mut prepared = match observer.prepare_cases(ordinary, requested, attempted) {
        Ok(prepared) => prepared,
        Err(failure) => return (failure.ordinary, Err((W1Fallback::Preparation, 0))),
    };
    // T-8: one CaseBatchCall over the prepared cases of A.
    #[cfg(test)]
    retained_tests_hooks::before_native(&mut prepared);
    prepared.native();
    // T-9: one freeze per selected Run.
    prepared.freeze();
    // T-10: a successor needs a selected case.
    let reached = |stage: fn(&AttemptEnd) -> bool| prepared.attempts.iter().any(|attempt| stage(&attempt.end));
    if !reached(|end| matches!(end, AttemptEnd::Frozen(_))) {
        let cause = if reached(|end| matches!(end, AttemptEnd::Candidate(_))) {
            W1Fallback::Candidate
        } else if reached(|end| matches!(end, AttemptEnd::Native)) {
            W1Fallback::Native
        } else {
            W1Fallback::Preparation
        };
        return (prepared.into_ordinary(), Err((cause, 0)));
    }
    let selected = prepared.selected_attempts();
    // B2-P T-10a and T-10b (B2-C §2.3, §2.4): each combination's disposition, then for the
    // retained ones their operand sources, operand preparations, Calls, Runs and freezes. Each
    // combination's outcome is its own; a custody failure abandons the successor (§2.6).
    prepared.dispositions();
    if prepared.combine().is_err() {
        return (prepared.into_ordinary(), Err((W1Fallback::CombinationCustody, selected)));
    }
    // T-11. Staging: the overlays apply to one copy; the ordinary owner stays intact. A
    // broken overlay invariant falls back typed (RV85 N6).
    #[cfg(test)]
    retained_tests_hooks::before_staging(&mut prepared);
    let staged = match prepared.staged_envelope() {
        Ok(staged) => staged,
        Err(fault) => return (prepared.into_ordinary(), Err((W1Fallback::Staging(fault), selected))),
    };
    let serialized = retained_wire::serialize_cases(&mut prepared, &staged, capture);
    drop(staged);
    #[cfg(test)]
    let serialized = retained_tests_hooks::after_serialize(serialized);
    #[cfg_attr(not(test), allow(unused_mut))]
    let mut successor = match serialized {
        Ok(successor) => successor,
        Err(failure) => return (prepared.into_ordinary(), Err((W1Fallback::Serializer(failure), selected))),
    };
    #[cfg(test)]
    retained_tests_hooks::before_precommit(&mut successor);
    // Decision 5: precommit validation by the accepted Rust reader, against the
    // actual invocation (only Ok/Err is used here; eligibility is not read).
    #[cfg_attr(not(test), allow(unused_mut))]
    let mut invocation = serde_json::json!({"request": capture.borrowed_raw(), "solver_mode": capture.mode().as_str()});
    #[cfg(test)]
    retained_tests_hooks::before_precommit_invocation(&mut invocation);
    if let Err(error) = open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)) {
        return (prepared.into_ordinary(), Err((W1Fallback::Precommit { gate: error.gate, code: error.code }, selected)));
    }
    drop(invocation);
    // The transfer moves only: no fallible step follows.
    (prepared.into_ordinary(), Ok(successor))
}

/// Test-only fault seams for the W1 phases the private driver reaches (decision 7
/// permits no test permit). The armed faults are thread-local; `carry_test_hooks`
/// moves them onto the reserved-stack thread.
#[cfg(test)]
pub(crate) mod retained_tests_hooks {
    use super::retained_wire::{ReceiptCheck, ReceiptFailure};
    /// The faults armed on one thread, each consumed by its next stage.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Armed {
        corrupt: bool,
        withdraw: bool,
        rebind: bool,
        serializer: Option<ReceiptCheck>,
        staging: bool, late_gate: bool, complete_gate: bool, preparation: bool, candidate: Option<super::retained_receipt::TraceFault>,
        /// B1 SP (decision 23; RV107 A1-N-6): the request index of the case whose preparation fails.
        preparation_of_case: Option<usize>,
        /// B3b-P (P-12): the exact route's material capture refuses (its Ĝ check sees Ĝ + 1 ulp).
        exact_capture: bool,
        /// B3b-P (P-12): the next frozen exact case's first section patch names an index past
        /// its `pipe_sections`.
        section_staging: bool,
        /// B2-P hooks: the request index of the `not_required` case whose operand preparation
        /// fails; the authored index of the combination whose Call refuses before any source; and
        /// a fault in the next combination freeze (its observables stage).
        operand_preparation_of_case: Option<usize>,
        combination_call: Option<usize>,
        combination_freeze: bool,
        /// RV123 S-2: the request index of the case whose freeze refuses (its maxima stage).
        freeze_of_case: Option<usize>,
        /// B2-P: the serializer's meter-chain check reads the next pair of Calls as unchained.
        meter_chain: bool,
        /// RV123 (B2-P round 2) S-1: the request index of the case whose batch Run reads as
        /// refused `ledger_unavailable`.
        ledger_of_case: Option<usize>,
        /// lib.rs's dense-scrutiny ceiling override (F1b), read by the ordinary run.
        ceiling: Option<u128>,
    }
    thread_local! {
        static ARMED: std::cell::Cell<Armed> = std::cell::Cell::new(Armed::default());
    }
    fn arm(f: impl FnOnce(&mut Armed)) { ARMED.with(|a| { let mut armed = a.get(); f(&mut armed); a.set(armed); }); }
    fn consume<T>(f: impl FnOnce(&mut Armed) -> T) -> T { ARMED.with(|a| { let mut armed = a.get(); let value = f(&mut armed); a.set(armed); value }) }
    /// Take this thread's armed faults (leaving none) and copy its ceiling override.
    pub(crate) fn take_armed() -> Armed {
        let mut armed = ARMED.with(|a| a.take());
        // A setting, not a one-shot fault: copied, so the caller thread keeps it.
        armed.ceiling = super::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|c| c.get());
        armed
    }
    pub(crate) fn install_armed(armed: Armed) {
        super::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|c| c.set(armed.ceiling));
        ARMED.with(|a| a.set(Armed { ceiling: None, ..armed }));
    }
    /// RV85 U2: faults handed back across the hop, by the caller's thread.
    static HANDED_BACK: std::sync::Mutex<Vec<(std::thread::ThreadId, Armed)>> = std::sync::Mutex::new(Vec::new());
    fn record(caller: std::thread::ThreadId, armed: Armed) {
        let faults = Armed { ceiling: None, ..armed };
        if faults != Armed::default() {
            HANDED_BACK.lock().unwrap().push((caller, faults));
        }
    }
    /// The caller's faults in transit: installed on the worker, or handed back as
    /// they drop unrun (a spawn failure drops the work on the caller's thread).
    pub(crate) struct Carried { armed: Option<Armed>, caller: std::thread::ThreadId, tally: grant2::TallyCarry }
    impl Carried {
        pub(crate) fn take() -> Self { Self { armed: Some(take_armed()), caller: std::thread::current().id(), tally: grant2::TallyCarry::take() } }
        /// On the worker: install the carried faults; returns the caller to hand back to.
        pub(crate) fn install(mut self) -> std::thread::ThreadId {
            install_armed(self.armed.take().unwrap_or_default()); self.tally.install();
            self.caller
        }
    }
    impl Drop for Carried {
        fn drop(&mut self) {
            if let Some(armed) = self.armed.take() {
                record(self.caller, armed);
            }
        }
    }
    /// On the worker, after the work: this thread's unfired faults go back to the caller.
    pub(crate) fn hand_back(caller: std::thread::ThreadId) {
        record(caller, ARMED.with(|a| a.take())); grant2::TallyCarry::hand_back();
    }
    /// On the caller, after the hop: re-arm every fault handed back to this thread.
    pub(crate) fn reclaim_handed_back() {
        let me = std::thread::current().id();
        let mut back = HANDED_BACK.lock().unwrap();
        let mut mine = Vec::new();
        back.retain(|(owner, armed)| if *owner == me { mine.push(*armed); false } else { true });
        drop(back);
        for faults in mine {
            arm(|a| {
                a.corrupt |= faults.corrupt;
                a.withdraw |= faults.withdraw;
                a.rebind |= faults.rebind;
                a.serializer = a.serializer.or(faults.serializer);
                a.staging |= faults.staging; grant2::merge(a, &faults);
            });
        }
    }
    /// The names of the faults armed on this thread (test assertions).
    pub(crate) fn armed_names() -> Vec<&'static str> {
        let a = armed();
        [(a.corrupt, "precommit"), (a.withdraw, "native"), (a.rebind, "rebind"), (a.serializer.is_some(), "serializer"), (a.staging, "staging")]
            .into_iter().chain(grant2::names(&a)).filter(|(on, _)| *on).map(|(_, name)| name).collect()
    }
    /// Disarm every fault on this thread (test cleanup).
    pub(crate) fn disarm() { ARMED.with(|a| a.set(Armed { ceiling: None, ..Armed::default() })); }
    /// Precommit binding fault: the next precommit validates against the other mode.
    pub(crate) fn rebind_next_precommit_invocation() { arm(|a| a.rebind = true); }
    pub(crate) fn before_precommit_invocation(invocation: &mut serde_json::Value) {
        if consume(|a| std::mem::take(&mut a.rebind)) {
            let other = if invocation["solver_mode"] == "dense_scrutiny" { "sparse_interactive" } else { "dense_scrutiny" };
            invocation["solver_mode"] = serde_json::json!(other);
        }
    }
    pub(crate) fn corrupt_next_precommit() { arm(|a| a.corrupt = true); }
    /// Native-stage fault: the first prepared case's source is withdrawn before the call, so
    /// the call itself fails before any Run (B1 SP T-8; D38's trigger).
    pub(crate) fn withdraw_next_native_source() { arm(|a| a.withdraw = true); }
    /// Serializer-stage fault: the next serialization refuses with this check.
    pub(crate) fn fail_next_serializer(check: ReceiptCheck) { arm(|a| a.serializer = Some(check)); }
    /// Staging fault: the next frozen overlay names a maxima patch past the evidence.
    pub(crate) fn break_next_staging() { arm(|a| a.staging = true); }
    pub(crate) fn before_staging(prepared: &mut super::retained_product::PreparedCases) {
        if consume(|a| std::mem::take(&mut a.staging)) {
            prepared.test_break_overlay();
        }
        if consume(|a| std::mem::take(&mut a.section_staging)) {
            prepared.test_break_section_overlay();
        }
    }
    /// Reached before the call; it fires only when a case was prepared (as before T-8, where
    /// the one-case hook ran only after a successful preparation).
    pub(crate) fn before_native(prepared: &mut super::retained_product::PreparedCases) {
        let Some(request) = prepared.attempts.iter().find(|attempt| attempt.prepared).map(|attempt| attempt.request) else { return };
        if consume(|a| std::mem::take(&mut a.withdraw)) {
            prepared.capture.with_case(request, |capture| capture.source = None);
        }
    }
    pub(crate) fn after_serialize(serialized: Result<serde_json::Value, ReceiptFailure>) -> Result<serde_json::Value, ReceiptFailure> {
        match consume(|a| a.serializer.take()) {
            Some(check) => Err(ReceiptFailure { check, field_path: "cases[].run.invocation_after" }),
            None => serialized,
        }
    }
    pub(crate) fn before_precommit(successor: &mut serde_json::Value) {
        // RV107 A1-N-1: the serialized successor, as precommit receives it, for the test.
        grant2::capture_successor(successor);
        if consume(|a| std::mem::take(&mut a.corrupt)) {
            successor["retained_precision"]["receipt_sha256"] = serde_json::json!("0".repeat(64));
        }
    }
    /// The faults still armed on this thread (tests assert none is left behind).
    pub(crate) fn armed() -> Armed { ARMED.with(|a| a.get()) } mod grant2; pub(crate) use grant2::*;
}

/// Resolved thermal+fit eigenstrain with each member's own E and the exact
/// annulus wall area; assembled once and removed once in recovery. The
/// product route and captured-source replay share this one construction.
/// A resolved member whose eigenload has no built section is refused, never
/// dropped: the caller blocks the case (or refuses the replay).
fn load_state_eigen_loads(
    state: &case_state::resolve::ResolvedCase,
    built: &BuiltModel,
) -> Result<Vec<ThermalElementLoad>, String> {
    state
        .members
        .iter()
        .filter(|member| member.strain.total_eigenstrain != 0.0)
        .map(|member| {
            let section = built.sections.get(&member.pipe_id).ok_or_else(|| member.pipe_id.clone())?;
            Ok(ThermalElementLoad {
                element_index: member.pipe_index,
                source: source_recovery::eigen_source_id(&member.pipe_id),
                axial_load: member.material.pair.elastic_modulus_pa()
                    * section.area
                    * member.strain.total_eigenstrain,
                thermal_strain: member.strain.total_eigenstrain,
            })
        })
        .collect()
}

/// Per-member material evidence of one resolved case: the same pair that
/// built stiffness and pressure recovery; the case-wide base is not consulted.
fn load_state_pipe_materials(state: &case_state::resolve::ResolvedCase) -> serde_json::Value {
    serde_json::json!(state.members.iter().map(|member| serde_json::json!({
        "pipe_id":member.pipe_id,"material_id":member.material.material_id,
        "E_pa":member.material.pair.elastic_modulus_pa(),"nu":member.material.pair.poisson_ratio(),
        "G_pa":member.material.pair.shear_modulus_pa(),"constitutive_basis":"homogeneous_isotropic_E_nu_v1",
        "material_selection_kind":member.material.selection_kind,
        "thermal_consumed":false,"alpha_per_kelvin":serde_json::Value::Null,
        "resolved_eigenstrain":member.strain.total_eigenstrain,
        "provenance":member.material.material_provenance})).collect::<Vec<_>>())
}

/// The published `load_reference_states` record of one case. Only a selected
/// retained-source response is joined; every other case states the ordinary
/// route it actually published.
fn load_state_case_record(
    state: &case_state::resolve::ResolvedCase,
    solver_mode: PreviewSolverMode,
    source_selected: bool,
) -> serde_json::Value {
    let mut evidence = state.evidence.clone();
    evidence["solve"] = serde_json::json!({
        "requested_mode": solver_mode.as_str(),
        "recovery_method": if source_selected { "retained_source_blocks_exact_v1" } else if solver_mode == PreviewSolverMode::DenseScrutiny { "ordinary_dense_structural_v1" } else { "ordinary_sparse_structural_v1" },
        "boundary": "every restrained DOF prescribed; reduced K_ff u_f = f_f - K_fc g_c; complete u includes g; reactions from unreduced K u - f",
        "eigenload": "axial E_member*A_s*total_eigenstrain assembled once and removed once in recovery",
    });
    evidence["source_recovery"] = if source_selected {
        serde_json::json!({"status":"selected","method":"retained_source_blocks_exact_v1"})
    } else {
        serde_json::json!({"status":"not_joined","code":case_state::SOURCE_RECOVERY_NOT_JOINED})
    };
    evidence
}

/// F1b (T3 D1 §4.8, W3; K1): one basis's global stiffness on the kernel's
/// sparse pattern, for the ordinary route. The contributions accumulate in the
/// dense assembly's order (frames and users, then the realized curved bends as
/// blocks, then the springs), so every stored value is bit-identical to
/// `assemble_case_stiffness`'s dense entry and every absent entry is its +0.0.
/// Each element is formed by the same call in the same order, so a formation
/// error is the same error for the same first element.
fn assemble_basis_stiffness(
    built: &BuiltModel,
    springs: &[SpringEntry],
) -> Result<SparseStiffness, FrameKernelError> {
    let blocks = built
        .curved_bend_elements
        .iter()
        .map(|element| StiffnessBlock {
            node_i: element.node_i,
            node_j: element.node_j,
            stiffness: element.global_stiffness,
        })
        .collect::<Vec<_>>();
    let springs = springs
        .iter()
        .map(|spring| (spring.node_dof.global_index(), spring.stiffness.value))
        .collect::<Vec<_>>();
    assemble_sparse_stiffness(
        built.nodes.len(),
        &built.frame_elements,
        &built.connectors,
        &blocks,
        &springs,
        &SparseAssemblyOptions::new(),
    )
}

/// F1b: one basis's global stiffness (one modulus basis, or one 0.4.0
/// resolved case).
enum BasisStiffness {
    /// The kernel's sparse assembly at b = 0 (`assemble_basis_stiffness`).
    Formed(SparseStiffness),
    /// K2a's `NumericalRange` at this basis's assembly, on a linear
    /// invocation: deferred to each of the basis's cases as its step-1 range
    /// trigger (T3 D1 §4.7 step 1; ROOT Q2). Main blocks the invocation here.
    RangeDeferred(FrameKernelError),
}

/// F1b (ROOT Q2): the basis assembly with K2a's range refusal deferred on a
/// linear invocation. Every other formation error, and a range refusal on an
/// invocation with a nonlinear support, blocks the invocation as today.
fn form_basis_stiffness(
    built: &BuiltModel,
    springs: &[SpringEntry],
    linear: bool,
) -> Result<BasisStiffness, FrameKernelError> {
    match assemble_basis_stiffness(built, springs) {
        Ok(stiffness) => Ok(BasisStiffness::Formed(stiffness)),
        Err(error @ FrameKernelError::NumericalRange { .. }) if linear => {
            Ok(BasisStiffness::RangeDeferred(error))
        }
        Err(error) => Err(error),
    }
}

/// F1b (T3 D1 §4.8 "Resource guard"; ROOT Q8(a), provisional): the dense
/// scrutiny path's estimated peak per n^2 entry. At the peak (inside FK's
/// `prepare_bound` via `audit_contributions`, called from SA's DenseScrutiny
/// branch) six n^2 buffers are alive together: the dense K view (8), the two
/// dense symmetry views (8 + 8), the prepared matrix (8) and the contribution
/// sums and differences (`Expansion`, 32 + 32). A stated formula over the
/// dense entry count, not a measurement.
const DENSE_SCRUTINY_BYTES_PER_ENTRY: u128 = 96;
/// Provisional (ROOT Q8(a), 2026-09-28): the gate's 6 GiB heap cap on the
/// owner's Mac, revisited from K6's and V-P's measurements. It admits at most
/// 8,192 global DOFs (1,365 nodes).
const DENSE_SCRUTINY_CEILING_BYTES: u128 = 6 * 1024 * 1024 * 1024;

/// The dense-scrutiny guard's refusal: the formula's inputs and its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DenseScrutinyRefusal {
    dimension: usize,
    dense_entries: u128,
    estimated_bytes: u128,
    ceiling_bytes: u128,
}

/// The dense path's estimated bytes for `dense_entries` n^2 entries.
fn dense_scrutiny_estimate_bytes(dense_entries: u128) -> u128 {
    DENSE_SCRUTINY_BYTES_PER_ENTRY.saturating_mul(dense_entries)
}

/// The guard's decision for a model of `dimension` global DOFs. Its dense
/// entry count is `dimension^2`, which is `SparseStorageCounts::dense_entries`
/// of the model's assembly. It refuses only when the estimate exceeds the
/// ceiling.
fn dense_scrutiny_guard(dimension: usize, ceiling_bytes: u128) -> Result<(), DenseScrutinyRefusal> {
    let dense_entries = (dimension as u128) * (dimension as u128);
    let estimated_bytes = dense_scrutiny_estimate_bytes(dense_entries);
    if estimated_bytes > ceiling_bytes {
        Err(DenseScrutinyRefusal {
            dimension,
            dense_entries,
            estimated_bytes,
            ceiling_bytes,
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
thread_local! {
    /// F1b tests only: a lowered ceiling for the unit-level product test.
    static DENSE_SCRUTINY_CEILING_OVERRIDE: std::cell::Cell<Option<u128>> =
        const { std::cell::Cell::new(None) };
}

/// The ceiling the product applies (the constant; a unit test may lower it).
fn dense_scrutiny_ceiling_bytes() -> u128 {
    #[cfg(test)]
    if let Some(ceiling) = DENSE_SCRUTINY_CEILING_OVERRIDE.with(|cell| cell.get()) {
        return ceiling;
    }
    DENSE_SCRUTINY_CEILING_BYTES
}

fn dense_scrutiny_refusal_diagnostic(refusal: &DenseScrutinyRefusal) -> Diagnostic {
    diag(
        "diagnostic:physics:dense-scrutiny-resource-guard",
        "SOLVER_SYSTEM_BLOCKED",
        "blocking",
        format!(
            "dense scrutiny resource guard: estimated dense-path peak {} bytes ({} bytes x {} dense entries, {} global DOFs squared) exceeds the provisional ceiling {} bytes; the model is refused before any n^2 allocation. The estimate is a stated formula, not a measurement; sparse_interactive does not use it, and no automatic dense fallback exists",
            refusal.estimated_bytes,
            DENSE_SCRUTINY_BYTES_PER_ENTRY,
            refusal.dense_entries,
            refusal.dimension,
            refusal.ceiling_bytes,
        ),
        vec!["model".to_string()],
    )
}

/// F1b (ROOT's ruling on the gate's heap-cap finding, 2026-09-28): the legacy
/// DEC-050/053 observation lane (`solve_symmetric_system_from_entries`) builds
/// the reduced system's identity-order profile (`SymmetricProfileMatrix::
/// from_entries`) only to count it. Its `values` vector of f64 grows by
/// `resize`, so during its last amortized growth the old and the new capacity
/// are alive together: at most 3 times the final length, 24 bytes per profile
/// entry. The RCM-ordered profile, its factor and the lane's O(n) and O(nnz)
/// vectors are not counted. A stated formula, not a measurement.
const SPARSE_OBSERVATION_BYTES_PER_PROFILE_ENTRY: u128 = 24;

/// The lane's identity-order profile as `SymmetricProfileMatrix::from_entries`
/// forms it from the same entries: each row's first stored column is the
/// smallest column of a nonzero entry in its lower triangle (a zero entry is
/// skipped, as the lane skips it). Returns (profile entries, maximum
/// half-bandwidth). O(nnz), with one first-column index per reduced DOF and
/// no profile storage.
fn observation_lane_profile(system: &ReducedSparseEntrySystem) -> (u128, usize) {
    let mut first_columns: Vec<usize> = (0..system.dimension).collect();
    for entry in &system.entries {
        if entry.value == 0.0 || entry.row >= system.dimension || entry.col >= system.dimension {
            continue;
        }
        let (hi, lo) = if entry.row >= entry.col {
            (entry.row, entry.col)
        } else {
            (entry.col, entry.row)
        };
        if lo < first_columns[hi] {
            first_columns[hi] = lo;
        }
    }
    let entries = first_columns
        .iter()
        .enumerate()
        .map(|(row, &first)| (row - first + 1) as u128)
        .sum();
    let bandwidth = first_columns
        .iter()
        .enumerate()
        .map(|(row, &first)| row - first)
        .max()
        .unwrap_or(0);
    (entries, bandwidth)
}

/// The observation lane guard's refusal: the estimate's inputs and value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ObservationLaneRefusal {
    profile_entries: u128,
    max_half_bandwidth: usize,
    estimated_bytes: u128,
    ceiling_bytes: u128,
}

/// F1b (ROOT, 2026-09-28): the lane runs only when its estimated bytes are
/// within the provisional ceiling, the dense-scrutiny guard's named constant
/// (`dense_scrutiny_ceiling_bytes`, which a unit test may lower).
fn observation_lane_guard(
    system: &ReducedSparseEntrySystem,
    ceiling_bytes: u128,
) -> Result<(), ObservationLaneRefusal> {
    let (profile_entries, max_half_bandwidth) = observation_lane_profile(system);
    let estimated_bytes =
        SPARSE_OBSERVATION_BYTES_PER_PROFILE_ENTRY.saturating_mul(profile_entries);
    if estimated_bytes > ceiling_bytes {
        Err(ObservationLaneRefusal {
            profile_entries,
            max_half_bandwidth,
            estimated_bytes,
            ceiling_bytes,
        })
    } else {
        Ok(())
    }
}

fn observation_lane_refusal_diagnostic(
    load_case_id: &str,
    refusal: &ObservationLaneRefusal,
) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:sparse-observation:{}:resource-guard",
            stable_suffix(load_case_id)
        ),
        "SPARSE_OBSERVATION_LANE_NOT_RUN",
        "info",
        format!(
            "legacy DEC-050/053 observation lane not run for load case {load_case_id}: its identity-order profile of {} entries (maximum half-bandwidth {}) needs an estimated {} bytes ({} bytes x {} profile entries), above the provisional ceiling {} bytes; the mode row's profile, pivot and residual observation fields are published as not_observed. The estimate is a stated formula, not a measurement; the lane never selects a solution, and the structural solve and its publication are unchanged",
            refusal.profile_entries,
            refusal.max_half_bandwidth,
            refusal.estimated_bytes,
            SPARSE_OBSERVATION_BYTES_PER_PROFILE_ENTRY,
            refusal.profile_entries,
            refusal.ceiling_bytes,
        ),
        vec![load_case_id.to_string(), "DEC-053".to_string()],
    )
}

/// Assemble one resolved case's global stiffness, with the same element,
/// curved and spring contributions as the default basis assembly. F1b: the
/// ordinary route uses `assemble_basis_stiffness`; this dense form remains for
/// the n <= 256 captured replay (`source_receipt`) and tests.
fn assemble_case_stiffness(
    built: &BuiltModel,
    springs: &[SpringEntry],
) -> Result<Vec<Vec<f64>>, FrameKernelError> {
    let mut stiffness = assemble_global_stiffness_with_connectors(
        built.nodes.len(),
        &built.frame_elements,
        &built.connectors,
    )?;
    add_curved_bend_stiffness_contributions(&mut stiffness, &built.curved_bend_elements);
    for spring in springs {
        stiffness[spring.node_dof.global_index()][spring.node_dof.global_index()] +=
            spring.stiffness.value;
    }
    Ok(stiffness)
}

fn maximum_across_cases(cases: &[LoadCaseSolve], stress: bool) -> Option<LocatedQuantity> {
    // A headline covers the complete requested domain, never only available members.
    if cases.iter().any(|case| {
        if stress {
            case.max_stress.is_none()
        } else {
            case.max_displacement.is_none()
        }
    }) {
        return None;
    }
    cases
        .iter()
        .enumerate()
        .filter_map(|(index, case)| {
            let mut q = if stress {
                case.max_stress.clone()
            } else {
                case.max_displacement.clone()
            }?;
            let already_qualified = case
                .results
                .iter()
                .find(|r| r.id == q.result_ref)
                .is_some_and(|r| r.kind.ends_with("_v2"));
            if index > 0 && !already_qualified {
                q.result_ref = qualified_load_case_result_id(&case.load_case_id, &q.result_ref);
            }
            Some((&case.load_case_id, q))
        })
        .max_by(|(case_a, a), (case_b, b)| {
            a.value
                .total_cmp(&b.value)
                .then_with(|| case_b.cmp(case_a))
                .then_with(|| b.location_ref.cmp(&a.location_ref))
        })
        .map(|(_, q)| q)
}

fn require_finite_mechanics(values: impl IntoIterator<Item = f64>) -> Result<(), FrameKernelError> {
    for value in values {
        if !value.is_finite() {
            return Err(FrameKernelError::NonFiniteInput {
                name: "computed mechanics",
                value,
            });
        }
    }
    Ok(())
}

/// S11 section 4.2: one term per nodal load, as `global_load_vector` added
/// them (a DOF outside the system is skipped exactly as it did).
fn push_nodal_loads(ledger: &mut LoadLedger, application: &LoadApplication, node_count: usize) {
    let size = node_count * DOF_PER_NODE;
    for load in &application.nodal_loads {
        if load.global_dof < size {
            ledger.push(&load.load_id, load.global_dof, load.value);
        }
    }
}

/// The case's load ledger (S11 sections 4.2 and 4.3): every force producer
/// pushes its contributions, term by term, in this fixed order (nodal loads,
/// uniform element equivalents, thermal and eigen equivalents, exact-pressure
/// group operands, constant effort). The legacy pressure thrust producer is
/// retired with legacy pressure (U3).
#[allow(clippy::too_many_arguments)]
fn case_force_ledger(
    model: &PreviewModel,
    built: &BuiltModel,
    load_application: &LoadApplication,
    curved_bends_by_pipe: &HashMap<usize, &CurvedBendMacroBuild>,
    thermal_loads: &[ThermalElementLoad],
    exact_pressure: Option<&pressure_runtime::ExactPressureCase>,
    load_case_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> LoadLedger {
    let mut ledger = LoadLedger::new();
    push_nodal_loads(&mut ledger, load_application, built.nodes.len());
    add_uniform_element_loads(
        &mut ledger,
        model,
        &load_application.element_uniform_loads,
        &built.pipes,
        curved_bends_by_pipe,
        load_case_id,
        diagnostics,
    );
    add_thermal_equivalent_loads(
        &mut ledger,
        thermal_loads,
        &built.pipes,
        curved_bends_by_pipe,
    );
    if let Some(exact) = exact_pressure {
        push_exact_pressure_operands(&mut ledger, exact);
        // T4-U2 (H-2): each realized arc's K_b·u_free(ε_p).
        add_curved_bend_pressure_equivalent_load(&mut ledger, exact, curved_bends_by_pipe);
    }
    add_connector_reference_loads(&mut ledger, built, load_case_id, diagnostics);
    // DEC-049 constant-effort consumption enters here — the one assembled
    // force-vector seam shared by the dense, sparse, and nonlinear
    // active-set solve paths.
    add_constant_effort_support_loads(&mut ledger, model);
    ledger
}

/// The retained-source replay's case force (S11 section 4.5): the same live
/// producer functions as `solve_load_case` (nodal loads, then T1's eigen
/// equivalents), through the ledger. Retained scope admits no other producer.
pub(crate) fn nodal_and_eigen_case_force(
    application: &LoadApplication,
    eigen: &[ThermalElementLoad],
    built: &BuiltModel,
) -> Result<AssembledForce, open_pipe_stress_frame_kernel::load_ledger::LedgerError> {
    let mut ledger = LoadLedger::new();
    push_nodal_loads(&mut ledger, application, built.nodes.len());
    add_thermal_equivalent_loads(&mut ledger, eigen, &built.pipes, &HashMap::new());
    ledger.finish(built.nodes.len() * DOF_PER_NODE)
}

/// The replaced spans of a built model (S20/S21 lookup set), by pipe ID.
fn replaced_span_ids_of(built: &BuiltModel) -> HashSet<&str> {
    built.connector_records.iter().map(|r| r.span_id.as_str()).collect()
}

/// T4-U3 (S14): each connector's recovered rows for one case, from the solved
/// displacements: q − q_ref, g and the global end actions (node on element),
/// each component one exact sum rounded once. A declared coverage limit:
/// these rows are outside R-b′ and the straight-member recovery bound.
fn append_connector_results(
    built: &BuiltModel,
    displacements: &[f64],
    load_case_id: &str,
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (connector, record) in built.connectors.iter().zip(&built.connector_records) {
        let recovered = connector
            .element_displacements(displacements)
            .ok_or(FrameKernelError::NonFiniteInput { name: "connector displacements", value: f64::NAN })
            .and_then(|d| connector.recover(&d));
        let recovery = match recovered {
            Ok(recovery) => recovery,
            Err(error) => {
                diagnostics.push(diag(
                    &format!("diagnostic:joint:{}:{}:recovery", stable_suffix(load_case_id), stable_suffix(&record.component_id)),
                    "ELEMENT_FORCE_RECOVERY_FAILED",
                    "blocking",
                    format!("objective connector {} cannot be recovered: {error}", record.component_id),
                    vec![record.component_id.clone(), load_case_id.to_string()],
                ));
                continue;
            }
        };
        let rows = joint::connector_rows(&recovery);
        for row in 0..8 {
            let (kind, components, unit, values, location) = rows[row];
            for axis in 0..3 {
                let (component, value) = (components[axis], values[axis]);
                results.push(ResultItem {
                    id: format!(
                        "result:connector:{}:{}:{location}:{}",
                        stable_suffix(&record.component_id),
                        kind.trim_end_matches("_v1").replace('_', "-"),
                        component.to_ascii_lowercase()
                    ),
                    kind: kind.to_string(),
                    value,
                    unit: unit.to_string(),
                    entity_ref: record.component_id.clone(),
                    basis_ref: None,
                    source_result_refs: Vec::new(),
                    metadata: Some(ResultMetadata {
                        component: component.to_string(),
                        coordinate_system: if location == "connector_local" { "connector_axes_q" } else { "global" }.to_string(),
                        location: location.to_string(),
                        // T4-U2: one stable basis string (the export vocabulary);
                        // the replaced span is bound through the connector record.
                        basis: "objective_connector_v1;symmetric_midpoint_small_rotation_v1".to_string(),
                        sign_convention: if location == "connector_local" {
                            "generalized coordinates of the connector frame Q: q - q_ref and g = K(q - q_ref); positive along the connector axes"
                        } else {
                            "global end action on the connector at its node (node on element), f = B^T g; the connector acts on its node with -f"
                        }
                        .to_string(),
                    }),
                });
            }
        }
    }
}

/// T4-U3 (S13): each connector's installed-state assembly load +BᵀK q_ref,
/// one formed term per nonzero DOF, bounded by `connector_reference_load_bound`
/// from the held operands, and self-equilibrated (it is the action of a
/// stress-free-referenced internal element on its own two nodes). A
/// connector with K q_ref = 0 exactly has no term (N-7).
fn add_connector_reference_loads(
    ledger: &mut LoadLedger,
    built: &BuiltModel,
    load_case_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    use open_pipe_stress_frame_kernel::connector::connector_reference_load_bound;
    for (connector, record) in built.connectors.iter().zip(&built.connector_records) {
        let formed = connector.reference_load().and_then(|load| {
            Ok(match load {
                Some(load) => Some((load.values, connector_reference_load_bound(connector)?)),
                None => None,
            })
        });
        let (values, bounds) = match formed {
            Ok(Some(formed)) => formed,
            Ok(None) => continue,
            Err(error) => {
                diagnostics.push(diag(
                    &format!("diagnostic:joint:{}:{}:reference-load", stable_suffix(load_case_id), stable_suffix(&record.component_id)),
                    "OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE",
                    "blocking",
                    format!("objective connector {}'s reference load cannot be formed: {error}", record.component_id),
                    vec![record.component_id.clone(), load_case_id.to_string()],
                ));
                continue;
            }
        };
        let dofs = element_dof_map(connector.node_i().index, connector.node_j().index);
        for index in 0..12 {
            let (dof, value, bound) = (dofs[index], values[index], bounds[index]);
            if value == 0.0 {
                continue;
            }
            ledger.push_formed(
                format!("connector_reference:{}", record.component_id),
                dof,
                value,
                Formation::Bounded { bound },
                0.0,
                true,
            );
        }
    }
}

/// S11 section 4.2: each exact-pressure source group's operand, never the
/// group's pre-summed per-DOF total.
fn push_exact_pressure_operands(
    ledger: &mut LoadLedger,
    exact: &pressure_runtime::ExactPressureCase,
) {
    for (dof, value, source) in &exact.assembled_operands {
        ledger.push_formed(
            source,
            *dof,
            *value,
            Formation::Bounded {
                bound: exact_pressure_operand_bound(*value),
            },
            0.0,
            false,
        );
    }
}

/// S11-G SF-2: the formation bound of one exact-pressure group operand,
/// `pressure_group_value` = to_f64(p * (PI * r_i * r_i) * c * d) in `Scaled`
/// arithmetic. Counted from the source, the chain is products only with k = 10
/// relative roundings: r_i = od*0.5 - wall (1, exact halving; entering twice,
/// 2), PI (1), PI*r_i and *r_i (2), *p (1), the coefficient's correctly
/// rounded sum (1), *c (1), *d (1) and the output to_f64 (1). k exceeds 8, so
/// the design's rule takes gamma_2k = gamma_20 (rounded upward), plus
/// 2^-1074 when the output is subnormal (the output rounding is then
/// absolute).
fn exact_pressure_operand_bound(value: f64) -> f64 {
    let relative = product_upward(gamma(20), value.abs());
    if value != 0.0 && value.abs() < f64::MIN_POSITIVE {
        (relative + f64::from_bits(1)).next_up()
    } else {
        relative
    }
}

/// The case force: each DOF's correctly rounded net (S11 section 4.3). A net
/// outside the binary64 range, or a non-finite term, is the same non-finite
/// mechanics refusal the folded vector met in `require_finite_mechanics`.
fn finish_case_ledger(
    ledger: LoadLedger,
    node_count: usize,
) -> Result<AssembledForce, FrameKernelError> {
    ledger
        .finish(node_count * DOF_PER_NODE)
        .map_err(|_| FrameKernelError::NonFiniteInput {
            name: "computed mechanics",
            value: f64::INFINITY,
        })
}

/// Allow-listed observation lane (S11 section 4.3 limit 2): the legacy
/// DEC050/053 observation rebuilds a reduced system from a global vector; it
/// must observe K_ff u_f = f_f - K_fc g_c, never f_f. It reads the ledger's
/// values and folds in binary64 on purpose; it never reaches a solve seam.
/// F1b: `stiffness.get` is the stored value or +0.0, the dense entry bit for
/// bit, so the observation is unchanged.
fn legacy_observation_force(
    force: &AssembledForce,
    stiffness: &SparseStiffness,
    restrained_dofs: &[usize],
    prescribed: &[(usize, f64)],
    coupled: bool,
) -> Vec<f64> {
    let mut observation = force.values().to_vec();
    if coupled {
        for (row, value) in observation.iter_mut().enumerate() {
            if restrained_dofs.contains(&row) {
                continue;
            }
            for &(column, displacement) in prescribed {
                *value -= stiffness.get(row, column) * displacement;
            }
        }
    }
    observation
}

/// Allow-listed observation lane (S11 section 4.3 limits 1 and 2): the
/// protected DEC050/053 comparison retains the legacy unscaled LU reference on
/// the reduced system. It never selects a published solution.
fn legacy_dense_observation(
    reduced: &open_pipe_stress_frame_kernel::ReducedAssembledSystem,
) -> Result<Vec<f64>, FrameKernelError> {
    solve_dense(&reduced.stiffness, reduced.force.values())
}

/// E12 (S11 section 4.4): each restrained reaction is one exact sum of the
/// formed `K * u` term and minus every ledger term of that DOF, rounded once.
/// F1b (§4.8): from sparse rows, `SparseStiffness::reactions`, bit-identical
/// to the dense form (K1). A non-finite operand or an out-of-range net keeps
/// NaN, which `require_finite_mechanics` refuses, as before; so does a length
/// mismatch, which cannot occur (both are the model's 6 * nodes).
fn restrained_reactions(
    stiffness: &SparseStiffness,
    displacements: &[f64],
    force: &AssembledForce,
) -> Vec<f64> {
    stiffness
        .reactions(displacements, force)
        .unwrap_or_else(|_| vec![f64::NAN; stiffness.dimension()])
}

fn solve_load_case(
    model: &PreviewModel,
    built: &BuiltModel,
    materials: &[MaterialInput],
    stiffness: &BasisStiffness,
    restrained_dofs: &[usize],
    spring_entries: &[SpringEntry],
    load_case: &PreviewLoadCase,
    modulus_basis_record: Option<&str>,
    solver_mode: PreviewSolverMode,
    capture: Option<&source_receipt::CapturedInvocation>,
    source_budget: &mut SourceRecoveryBudget,
    load_state: Option<&case_state::resolve::ResolvedCase>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<LoadCaseSolve, FrameKernelError> {
    solve_load_case_observed(model, built, materials, stiffness, restrained_dofs, spring_entries,
        load_case, modulus_basis_record, solver_mode, capture, source_budget, load_state, diagnostics, None)
}

fn solve_load_case_observed(
    model: &PreviewModel,
    built: &BuiltModel,
    materials: &[MaterialInput],
    stiffness: &BasisStiffness,
    restrained_dofs: &[usize],
    spring_entries: &[SpringEntry],
    load_case: &PreviewLoadCase,
    modulus_basis_record: Option<&str>,
    solver_mode: PreviewSolverMode,
    capture: Option<&source_receipt::CapturedInvocation>,
    source_budget: &mut SourceRecoveryBudget,
    load_state: Option<&case_state::resolve::ResolvedCase>,
    diagnostics: &mut Vec<Diagnostic>,
    mut product: Option<&mut retained_product::ProductCapture>,
) -> Result<LoadCaseSolve, FrameKernelError> {
    // A resolved case supplies the complete ordinary-source ledger: only its
    // declared, factored primitives are applied, exactly once.
    let load_case = load_state.map_or(load_case, |state| &state.effective_case);
    let exact_pressure = pressure_runtime::build_pressure_case_with_members(
        model,
        built,
        materials,
        load_case,
        load_state.map(|state| &state.pairs),
        diagnostics,
    );
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
            load_state_evidence: None,
            exact_case_evidence: None,
            pressure_evidence: Vec::new(),
            source_case: None,
            source_selected: false,
            load_case_id: load_case.id.clone(),
            results: Vec::new(),
            max_displacement: None,
            max_stress: None,
            component_stress_modifier_count: 0,
            component_pressure_thrust_load_count: 0,
            support_force_vectors: HashMap::new(),
            preview: None,
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
    let thermal_loads = match load_state.map(|state| load_state_eigen_loads(state, built)) {
        Some(Ok(loads)) => loads,
        Some(Err(pipe_id)) => {
            diagnostics.push(diag(
                &format!("diagnostic:load-state:{}:{}:eigen-section", stable_suffix(&load_case.id), stable_suffix(&pipe_id)),
                "LOAD_STATE_MEMBER_SECTION_MISSING", "blocking",
                "the resolved member has a nonzero eigenstrain but no built section; its eigenload is not dropped",
                vec![load_case.id.clone(), pipe_id],
            ));
            return Ok(LoadCaseSolve {
                load_state_evidence: None,
                exact_case_evidence: None,
                pressure_evidence: Vec::new(),
                source_case: None,
                source_selected: false,
                load_case_id: load_case.id.clone(),
                results: Vec::new(),
                max_displacement: None,
                max_stress: None,
                component_stress_modifier_count: 0,
                component_pressure_thrust_load_count: 0,
                support_force_vectors: HashMap::new(),
                preview: None,
            });
        }
        None => build_thermal_element_loads(
            model,
            load_case,
            &material_map,
            &pipe_map,
            &built.sections,
            diagnostics,
        ),
    };

    let curved_bends_by_pipe = built
        .curved_bend_elements
        .iter()
        .map(|element| (element.pipe_index, element))
        .collect::<HashMap<_, _>>();
    // E10 removed (S11 section 4.4): recovery uses each uniform load's own
    // intensity, as the force side forms one equivalent per load.
    let curved_bend_uniform_intensities = curved_bend_uniform_intensities_by_pipe(
        &load_application.element_uniform_loads,
        &curved_bends_by_pipe,
    );
    // S11 sections 4.2 and 4.3: every producer pushes its contributions, term
    // by term, into one per-case exact ledger; the solve's force vector is
    // each DOF's correctly rounded net, and it can be built only here.
    let ledger = case_force_ledger(
        model,
        built,
        &load_application,
        &curved_bends_by_pipe,
        &thermal_loads,
        exact_pressure.as_ref(),
        &load_case.id,
        diagnostics,
    );
    if let Some(observer) = product.as_deref_mut() {
        observer.case_source(model, built, materials, load_case, restrained_dofs, spring_entries,
            &load_application, &thermal_loads);
    }
    let force = finish_case_ledger(ledger, built.nodes.len())?;
    // S11-G section 3.5: the load-row guard reads the ledger's formation
    // records once, before the solve; its finding gates retained-source
    // routing and demotes the published verdict (never a refusal).
    let formation_bodies = formation_bodies(built);
    let load_row_finding = formation_guard::load_row_finding(
        &force,
        &formation_bodies,
        restrained_dofs,
        |dof| integrity_dof_label(model, dof),
        &load_case.id,
    );
    // I61 U1 (G-b): the typed finding; its disclosure, if any, is set at the report.
    if let Some(observer) = product.as_deref_mut() {
        observer.ordinary_finding(&load_case.id, load_row_finding.as_ref());
    }

    // F1b: the basis's formed values (none when its range refusal was
    // deferred, ROOT Q2).
    let formed = match stiffness {
        BasisStiffness::Formed(formed) => Some(formed),
        BasisStiffness::RangeDeferred(_) => None,
    };
    // F1b: the stored values are row-major and every absent entry is a finite
    // +0.0, so the first non-finite value is the dense scan's, with its bits.
    require_finite_mechanics(
        formed
            .map_or(&[][..], SparseStiffness::values)
            .iter()
            .copied()
            .chain(force.values().iter().copied()),
    )?;
    // Every restrained DOF is prescribed: explicit zero unless the resolved case
    // supplies an actual support-state motion for that rigid DOF.
    let prescribed: Vec<(usize, f64)> = restrained_dofs
        .iter()
        .map(|&dof| {
            (
                dof,
                load_state
                    .and_then(|state| state.prescribed.get(&dof).copied())
                    .unwrap_or(0.0),
            )
        })
        .collect();
    if let Some(state) = load_state {
        for dof in state.prescribed.keys() {
            if !restrained_dofs.contains(dof) {
                diagnostics.push(diag(&format!("diagnostic:load-state:{}:prescribed-dof:{dof}", stable_suffix(&load_case.id)), "LOAD_STATE_BOUNDARY_MOTION_UNRESTRAINED", "blocking",
                    "a prescribed support-state motion does not address a prepared rigid boundary DOF", vec![load_case.id.clone()]));
            }
        }
        if has_blocking(diagnostics) {
            return Ok(LoadCaseSolve {
                load_state_evidence: None,
                exact_case_evidence: None,
                pressure_evidence: Vec::new(),
                source_case: None,
                source_selected: false,
                load_case_id: load_case.id.clone(),
                results: Vec::new(),
                max_displacement: None,
                max_stress: None,
                component_stress_modifier_count: 0,
                component_pressure_thrust_load_count: 0,
                support_force_vectors: HashMap::new(),
                preview: None,
            });
        }
    }
    let prescribed_values = prescribed
        .iter()
        .map(|&(_, value)| value)
        .collect::<Vec<_>>();
    // T1's 0.4.0 prescribed motion reaches the kernel's exact KS2 through the
    // typed reduction (S11 section 8.2). F1b: the pattern's partition and
    // reduced right-hand side, bit-identical to `reduce_assembled_system*`'s,
    // with their errors in their order (K1); no reduced matrix is formed.
    let reduced = formed
        .map(|formed| {
            reduce_assembled_sparse_system(
                formed,
                &force,
                restrained_dofs,
                load_state.is_some().then_some(prescribed_values.as_slice()),
            )
        })
        .transpose()?;
    // A deferred basis has no reduction: its free DOFs are the ascending
    // complement of the restrained ones, which is the partition
    // `reduce_assembled_sparse_system` forms (F1b D6).
    let deferred_free_dofs: Vec<usize>;
    let free_dofs: &[usize] = match &reduced {
        Some(reduced) => &reduced.free_dofs,
        None => {
            deferred_free_dofs = (0..force.len())
                .filter(|dof| !restrained_dofs.contains(dof))
                .collect();
            &deferred_free_dofs
        }
    };
    let observation_force = formed
        .map(|formed| {
            legacy_observation_force(
                &force,
                formed,
                restrained_dofs,
                &prescribed,
                load_state.is_some(),
            )
        })
        .unwrap_or_default();
    // Preliminary linear evidence belongs only to an actual successful solve.
    let mut preliminary_diagnostics = Vec::new();
    let attempted_linear = match stiffness {
        BasisStiffness::Formed(formed) => solve_preview_reduced_system(
            solver_mode,
            formed,
            reduced
                .as_ref()
                .map_or(&[][..], |reduced| reduced.force.values()),
            built,
            spring_entries,
            &force,
            &observation_force,
            &prescribed,
            load_case,
            &mut preliminary_diagnostics,
        )
        .map_err(OrdinaryFailure::Structural),
        // F1b (ROOT Q2): a deferred basis's ordinary attempt is its formation
        // range refusal; nothing was formed to attempt.
        BasisStiffness::RangeDeferred(error) => Err(OrdinaryFailure::Formation(error.clone())),
    };
    // I61 U1 (G-b; C2 §5): the initial failure, captured before W2 consumes it.
    if let (Some(observer), Err(failure)) = (product.as_deref_mut(), &attempted_linear) {
        observer.ordinary_initial_failure(&load_case.id, failure);
    }
    let report_sensitive = matches!(&attempted_linear, Ok(solve) if solve.structural_report.quality == SolveQuality::Sensitive);
    let attempt_err = attempted_linear.is_err();
    let needs_source_recovery =
        needs_source_recovery(report_sensitive, attempt_err, load_row_finding.as_ref());
    let mut selected_source = None;
    let mut source_failure = None;
    // A resolved case enters retained-source recovery only through its own
    // closed join: the recovery input carries the resolved case, and source
    // closure refuses any eigen load, pair or motion it does not own.
    let source_eligible = source_eligible(
        capture.is_some(),
        !built.nonlinear_supports.is_empty(),
        !model.combinations.is_empty(),
    );
    // F1b (ROOT Q9(a)): retained-source recovery keeps its dense `stiffness`
    // input, built only for an attempt it can run (n <= 256). Above that its
    // budget refusal precedes every read of `stiffness`
    // (`source_recovery::prepare_sources`), so the empty slice changes neither
    // the charge nor the bytes. A selected response exists only at n <= 256.
    let recovery_stiffness = match formed {
        Some(formed)
            if source_eligible
                && needs_source_recovery
                && formed.dimension() <= source_recovery::DENSE_SOURCE_DOF_LIMIT =>
        {
            formed.to_dense()
        }
        _ => Vec::new(),
    };
    let recovery_input = || source_recovery::Input {
        model, built, stiffness: &recovery_stiffness, force: &force, free: free_dofs,
        prescribed: &prescribed, spring_entries, load_case, load_application: &load_application,
        thermal_loads: &thermal_loads,
        load_state,
    };
    if source_eligible && needs_source_recovery {
        // S11-G revision 2.2 with ROOT's D22-1 condition: a case the ordinary
        // route would not attempt (report Passed, no Err) is declined for its
        // load-row finding without running an attempt, so the invocation
        // ledger equals the unguarded one.
        // I61 U1 (G-l; D39): whether an actual `solve_ordinary` attempt ran.
        let mut legacy_attempted = false;
        let attempt = if !crate::needs_source_recovery(report_sensitive, attempt_err, None) {
            Err(source_recovery::formation_decline_without_attempt())
        } else if formed.is_none() {
            // F1b (ROOT OQ2, option B): a basis whose formation left the
            // range has no assembled stiffness, and the method would refuse
            // it at the same frames' formation (F1b D2); it is declined
            // without an attempt and with zero work.
            Err(source_recovery::range_formation_decline_without_attempt())
        } else {
            source_budget.attempts += 1;
            legacy_attempted = true;
            let case_limit = source_budget.case_limit();
            source_recovery::solve_ordinary(
                recovery_input(),
                open_pipe_stress_frame_kernel::structural::exact_boundary::Limits {
                    operations: case_limit,
                    ..Default::default()
                },
                ForceScale::UNSCALED,
            )
            // S11-G revision 2.2 G-3: a guard-fired case is never selected; the
            // selection is declined before the 0.4.0 replay reservation.
            .and_then(|recovery| decline_for_formation(recovery, load_row_finding.as_ref()))
            .and_then(|recovery| match load_state {
                // Pre-0.4 selection is unchanged.
                None => Ok(recovery),
                // ROOT CP3 SF-1 screen: captured replay repeats the live
                // attempt's source closure and exact solve in the same ledger, so
                // selection first reserves an amount equal to the live charge.
                // This screen does not guarantee finalization (other stages are
                // charged before replay, and replay may cost slightly more); a
                // selected join that still cannot finalize is republished on the
                // ordinary route. The case's own refusal is checked first so
                // that a republication still reports it.
                Some(_) => recovery
                    .reserve_captured_replay(case_limit)
                    .and_then(|recovery| {
                        if source_budget.load_state_join_withheld.is_some() {
                            // ROOT CP3 SF-1: the invocation publishes ordinarily.
                            Err(recovery.decline_withheld())
                        } else {
                            Ok(recovery)
                        }
                    }),
            })
        };
        match attempt {
            Ok(recovery) => selected_source = Some(recovery),
            Err(failure) => {
                source_budget.debit(failure.work.charged, true);
                let withheld = match (&load_state, &source_budget.load_state_join_withheld) {
                    (Some(_), Some(cause)) => format!("; the invocation publishes every case on its ordinary route because its selected join could not finalize: {cause}"),
                    _ => String::new(),
                };
                diagnostics.push(diag(
                    &format!("diagnostic:source-recovery:{}", load_case.id),
                    "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", "info",
                    format!("The bounded retained-source method did not produce a selected response: {failure:?}{withheld}"),
                    vec![load_case.id.clone()],
                ));
                // I61 U1 (G-l; C2:160-162): the typed failure and its actual WorkReport.
                if let (Some(observer), Some(record)) = (product.as_deref_mut(), diagnostics.last()) {
                    observer.ordinary_legacy_failure(&load_case.id, legacy_attempted, &failure, &record.id);
                }
                source_failure = Some(failure);
            }
        }
    }
    // I61 U1 (G-l; D39): the route rows that carry no RecoveryFailure.
    if let Some(observer) = product.as_deref_mut() {
        observer.ordinary_legacy_route(&load_case.id, source_eligible, needs_source_recovery, selected_source.is_some());
    }
    if load_state.is_some() && selected_source.is_none() {
        let attempt = if !needs_source_recovery {
            "not_required_ordinary_checks_passed"
        } else if source_failure.is_some() {
            "unavailable"
        } else {
            "not_eligible"
        };
        diagnostics.push(diag(
            &format!("diagnostic:load-state:{}:source-recovery-not-joined", stable_suffix(&load_case.id)),
            case_state::SOURCE_RECOVERY_NOT_JOINED, "info",
            format!("requested_mode={}; the published response for this case is the ordinary structural route; no retained-source response is joined into it; retained_source_attempt={attempt}", solver_mode.as_str()),
            vec![load_case.id.clone()],
        ));
    }
    // The receipt's ordinary attempt (step below) and W2's publication.
    let mut ordinary_error = None;
    let mut w2_publication = None;
    let linear_solve = match attempted_linear {
        Ok(solve) => {
            diagnostics.extend(preliminary_diagnostics);
            Some(solve)
        }
        Err(OrdinaryFailure::Structural(error)) if selected_source.is_some() => {
            // Preserve the rejected ordinary attempt as evidence. It is not the
            // selected solve and is not relabelled as a successful factorization.
            append_integrity_failure(diagnostics, &load_case.id, &error, model);
            if let Some(record) = diagnostics.last_mut() {
                record.severity = "info".into();
                record.message = format!("Rejected ordinary attempt; a separate retained-source response is selected. {}", record.message);
            }
            ordinary_error = Some(error);
            None
        }
        Err(OrdinaryFailure::Structural(error))
            if open_pipe_stress_nonlinear_integration::structural_adapter::permits_contact_seed_trial(&error, built.connectors.is_empty() && built.curved_bend_elements.is_empty()) && eligible_contact_dofs(
                built.nodes.len(),
                restrained_dofs,
                &built.nonlinear_supports,
                &built.nonlinear_initial_states,
            )
            .is_some() =>
        {
            ordinary_error = Some(error);
            None
        }
        Err(failure) => {
            // F1b (T3 D1 §4.7; ROOT Q2, W2): a linear case whose ordinary
            // attempt range-triggered and that exact-block did not recover is
            // evaluated through K2b's orchestrator, then published or refused
            // per case. Every other failure is main's.
            let trigger = ordinary_range_trigger(&failure)
                .filter(|_| built.nonlinear_supports.is_empty());
            match (trigger, failure) {
                (Some(trigger), _) => match force_scaling_attempt(
                    model,
                    built,
                    spring_entries,
                    restrained_dofs,
                    load_case,
                    &load_application,
                    &thermal_loads,
                    exact_pressure.as_ref(),
                    &force,
                    &prescribed,
                    solver_mode,
                ) {
                    Ok((solve, publication)) => {
                        // I61 U1 (G-b): W2 published at b != 0.
                        if let Some(observer) = product.as_deref_mut() {
                            observer.ordinary_w2_published(&load_case.id, &trigger, publication.force_scale_exponent);
                        }
                        w2_publication = Some(publication);
                        Some(solve)
                    }
                    Err(refusal) => {
                        append_force_scaling_refusal(
                            diagnostics,
                            &load_case.id,
                            &refusal,
                            &trigger,
                            model,
                        );
                        // I61 U1 (G-b): W2's actual failure, before the early return.
                        if let (Some(observer), Some(record)) = (product.as_deref_mut(), diagnostics.last()) {
                            observer.ordinary_w2_failed(&load_case.id, &trigger, &refusal, &record.id);
                        }
                        return Ok(LoadCaseSolve {
                load_state_evidence: None,
                pressure_evidence: Vec::new(),
                exact_case_evidence: None,
                source_case: None,
                source_selected: false,
                load_case_id: load_case.id.clone(),
                results: Vec::new(),
                max_displacement: None,
                max_stress: None,
                component_stress_modifier_count: 0,
                component_pressure_thrust_load_count: 0,
                support_force_vectors: HashMap::new(),
                preview: None,
            });
                    }
                },
                // A deferred formation refusal exists only on a linear
                // invocation (`form_basis_stiffness`), so this is main's
                // `solver_blocked` path, unreachable by construction.
                (None, OrdinaryFailure::Formation(error)) => return Err(error),
                // The staged solver diagnostics currently contain only the
                // warning advertising a completed dense fallback. A failed
                // attempt has no solution basis to advertise; propagate its
                // actual error alone.
                (None, OrdinaryFailure::Structural(error)) => {
                    append_integrity_failure(diagnostics, &load_case.id, &error, model);
                    // I61 U1 (G-b): the initial failure's own integrity diagnostic.
                    if let (Some(observer), Some(record)) = (product.as_deref_mut(), diagnostics.last()) {
                        observer.ordinary_failure_diagnostic(&load_case.id, &record.id);
                    }
                    return Ok(LoadCaseSolve {
                load_state_evidence: None,
                pressure_evidence: Vec::new(),
                exact_case_evidence: None,
                source_case: None,
                source_selected: false,
                load_case_id: load_case.id.clone(),
                results: Vec::new(),
                max_displacement: None,
                max_stress: None,
                component_stress_modifier_count: 0,
                component_pressure_thrust_load_count: 0,
                support_force_vectors: HashMap::new(),
                preview: None,
            });
                }
            }
        }
    };

    // S11-G revision 2.2 G-2: the receipt's ordinary outcome follows the
    // published verdict, which the load-row finding demotes to Sensitive.
    // F1b: formed after W2, so a W2-published case records its published
    // report; a deferred formation refusal was never attempted.
    let ordinary_attempt = match (&linear_solve, &ordinary_error) {
        (Some(solve), _) => source_receipt::OrdinaryAttempt::passed(
            solver_mode,
            &solve.structural_report,
            integrity_diagnostic_id(&load_case.id),
            load_row_finding.is_some(),
        ),
        (None, Some(error)) => source_receipt::OrdinaryAttempt::rejected(
            solver_mode,
            error,
            integrity_diagnostic_id(&load_case.id),
        ),
        (None, None) => source_receipt::OrdinaryAttempt::not_attempted(solver_mode),
    };

    let mut displacements = vec![0.0; built.nodes.len() * DOF_PER_NODE];
    let mut results = Vec::new();
    append_modulus_basis_record(&mut results, load_case, modulus_basis_record);
    if let Some(linear_solve) = &linear_solve {
        for (index, dof) in free_dofs.iter().enumerate() {
            displacements[*dof] = linear_solve.solution[index];
        }
        // Complete u includes the actual prescribed boundary values.
        for &(dof, value) in &prescribed {
            displacements[dof] = value;
        }
        if selected_source.is_none() {
            append_linear_solver_mode_evidence(&mut results, &load_case.id, solver_mode, linear_solve);
        }
        // F1b (ROOT OQ5): no DEC-050/053 observation runs at b != 0; both
        // lanes observe the unscaled binary64 system that left the range.
        if let (PreviewSolverMode::DenseScrutiny, Some(formed), None) =
            (solver_mode, formed, &w2_publication)
        {
            // Protected DEC050/053 comparison retains the legacy unscaled LU
            // reference. This observation never selects a published solution.
            // F1b: its dense reduced system is formed from the dense view of
            // the same values only here, after the attempt, in dense scrutiny
            // behind the resource guard. The reduction repeats the sparse
            // one's checks on the same values, so it succeeds when that did.
            let dense_view = formed.to_dense();
            let dense_reduced = if load_state.is_some() {
                reduce_assembled_system_with_prescribed_displacements(
                    &dense_view,
                    &force,
                    restrained_dofs,
                    &prescribed_values,
                )
            } else {
                reduce_assembled_system(&dense_view, &force, restrained_dofs)
            };
            drop(dense_view);
            if let Ok(legacy_dense) = dense_reduced
                .as_ref()
                .map_err(|_| ())
                .and_then(|reduced| legacy_dense_observation(reduced).map_err(|_| ()))
            {
                append_sparse_live_path_evidence(
                    &mut results,
                    diagnostics,
                    &load_case.id,
                    built,
                    spring_entries,
                    &observation_force,
                    restrained_dofs,
                    &legacy_dense,
                );
            }
        }
    }
    if let Some(observer) = product.as_deref_mut() {
        observer.solver_observations(load_case, solver_mode, &results);
    }
    if let Some(recovery) = &selected_source {
        displacements.copy_from_slice(recovery.displacements());
        diagnostics.push(diag(
            &format!("diagnostic:source-recovery:{}:selected", load_case.id),
            "SOURCE_BLOCK_RECOVERY_SELECTED", "info",
            format!("requested_mode={}; selected_method=retained_source_blocks_exact_v1; complete source-bound displacement/member/station/device projections precede binary64 publication; ordinary-attempt evidence remains separate; {:?}", solver_mode.as_str(), recovery.summary()),
            vec![load_case.id.clone()],
        ));
    }
    let selected_nonlinear = append_nonlinear_support_loop_results(
        &mut results,
        diagnostics,
        model,
        built,
        restrained_dofs,
        &force,
        load_case,
        solver_mode,
        spring_entries,
    );
    if !built.nonlinear_supports.is_empty() {
        match &selected_nonlinear {
            Some(solve) if solve.converged => displacements = solve.displacements.clone(),
            _ => {
                diagnostics.push(diag(
                    "diagnostic:physics:selected-state-unavailable",
                    "SOLVER_SYSTEM_BLOCKED",
                    "blocking",
                    "nonlinear selected mechanics state is unavailable or did not converge",
                    vec![load_case.id.clone()],
                ));
                return Ok(LoadCaseSolve {
            load_state_evidence: None,
                    exact_case_evidence: None,
                    pressure_evidence: Vec::new(),
            source_case: None,
            source_selected: false,
                    load_case_id: load_case.id.clone(),
                    results: Vec::new(),
                    max_displacement: None,
                    max_stress: None,
                    component_stress_modifier_count: 0,
                    component_pressure_thrust_load_count: 0,
                    support_force_vectors: HashMap::new(),
                    preview: None,
                });
            }
        }
    }
    if built.nonlinear_supports.is_empty() {
        if let Some(linear) = &linear_solve {
            append_integrity_report(
                diagnostics,
                &load_case.id,
                &linear.structural_report,
                model,
                None,
                load_row_finding.as_ref(),
                linear.formation_check.as_ref(),
                w2_publication.as_ref(),
            );
            // I61 U1 (G-b): the published report; `demote` discloses the finding
            // only on a Passed report, and K-D5's record is one line of it.
            if let (Some(observer), Some(record)) = (product.as_deref_mut(), diagnostics.last()) {
                observer.ordinary_report(
                    &load_case.id,
                    record,
                    load_row_finding.is_some() && linear.structural_report.quality != SolveQuality::Sensitive,
                    linear.formation_check.is_some(),
                );
            }
            if let Some(report) = &linear.load_fidelity {
                append_load_contribution_absorbed(diagnostics, &load_case.id, report);
            }
        }
    } else if let Some(iteration) = selected_nonlinear
        .as_ref()
        .and_then(|solve| solve.iterations.last())
    {
        if matches!(&iteration.strict_gap, StrictGapEvidence::Qualified(_)) {
            // The ordinary factor/solve report precedes exact-ratio selection.
            // It cannot qualify the selected mixed field-recovery basis.
            diagnostics.push(diag(
                &integrity_diagnostic_id(&load_case.id),
                "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED",
                "warning",
                format!(
                    "Load case {} selected represented-exact-ratio global displacement and support reactions, while member fields are recovered from projected binary64 displacement. The ordinary structural report is a preprojection precondition, and selected-public-displacement equilibrium does not prove exact-ratio equilibrium or complete field recovery. Mixed recovery remains numerically unresolved and unavailable for Current reliance; computed loop and quantity rows remain inspection evidence only. Ordinary structural precondition: {:?}; selected-public-displacement equilibrium: {:?}",
                    load_case.id, iteration.structural_report, iteration.product_equilibrium
                ),
                vec![load_case.id.clone()],
            ));
        } else if let StrictGapEvidence::Unsupported { reason, .. } = &iteration.strict_gap {
            diagnostics.push(diag(
                &integrity_diagnostic_id(&load_case.id),
                "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED",
                "warning",
                format!(
                    "Load case {} retains source-bound ordinary binary64 mechanics for inspection only: strict-gap proof is unavailable ({reason}). Ordinary original-system equilibrium, returned-state classifier, separate zero count/cap/contact/sliding checks passed. Delta rows retain their scoped observation/policy metadata; no universal delta acceptance is asserted. Exact-gap and recovery qualification remain unresolved; these rows do not grant Current or downstream qualified use. Ordinary structural report: {:?}; ordinary selected-state equilibrium: {:?}",
                    load_case.id, iteration.structural_report, iteration.product_equilibrium
                ),
                vec![load_case.id.clone()],
            ));
        } else {
            append_integrity_report(
                diagnostics,
                &load_case.id,
                &iteration.structural_report,
                model,
                Some(&iteration.product_equilibrium),
                load_row_finding.as_ref(),
                None,
                None,
            );
        }
    }
    require_finite_mechanics(displacements.iter().copied())?;
    // U3: legacy pressure thrust is retired; the published summary count stays 0.
    let component_pressure_thrust_load_count = 0;
    let mut max_displacement = None;
    for node in &model.nodes {
        let node_index = node_index(&model, &node.id).unwrap();
        let magnitude_mm = selected_source.as_ref().map(|recovery| {
            let values = &recovery.published_nodal_components()[node_index * DOF_PER_NODE..];
            source_receipt::scaled_norm([values[0], values[1], values[2]])
        }).unwrap_or_else(|| displacement_magnitude(&displacements, node_index) * 1000.0);
        let result_id = format!("result:disp:{}", stable_suffix(&node.id));
        if max_displacement
            .as_ref()
            .map(|q: &LocatedQuantity| {
                magnitude_mm > q.value
                    || (pressure_runtime::is_exact(model)
                        && magnitude_mm == q.value
                        && node.id < q.location_ref)
            })
            .unwrap_or(true)
        {
            max_displacement = Some(LocatedQuantity {
                value: magnitude_mm,
                unit: "mm".to_string(),
                location_ref: node.id.clone(),
                result_ref: result_id.clone(),
            });
        }
        results.push(ResultItem {
            id: result_id,
            kind: "displacement_magnitude".to_string(),
            value: magnitude_mm,
            unit: "mm".to_string(),
            entity_ref: node.id.clone(),
            basis_ref: None,
            source_result_refs: Vec::new(),
            metadata: None,
        });
    }
    for node in &model.nodes {
        let node_index = node_index(&model, &node.id).unwrap();
        let first = results.len();
        append_node_displacement_component_results(
            &mut results,
            &node.id,
            &displacements,
            node_index,
        );
        if let Some(recovery) = &selected_source {
            for (component, row) in results[first..].iter_mut().enumerate() {
                row.value = recovery.published_nodal_components()[node_index * DOF_PER_NODE + component];
            }
        }
    }

    let reactions = selected_source
        .as_ref()
        .map(|recovery| recovery.reactions().to_vec())
        .or_else(|| {
            selected_nonlinear
                .as_ref()
                .map(|solve| solve.reactions.clone())
        })
        .unwrap_or_else(|| match (&w2_publication, formed) {
            // F1b (§4.7 step 5): at b != 0 the restrained reactions are K2b's
            // checked `force_scaled_reactions`, each unscaled once.
            (Some(publication), _) => publication.reaction_values(displacements.len()),
            (None, Some(formed)) => restrained_reactions(formed, &displacements, &force),
            // A deferred basis publishes only through W2 (the case returned
            // otherwise), so this is unreachable; NaN is refused below.
            (None, None) => vec![f64::NAN; displacements.len()],
        });
    require_finite_mechanics(reactions.iter().copied())?;
    // preview-physics-1 side record; rendered only when no case is source-selected.
    let mut preview_record = (!pressure_runtime::is_exact(model) && selected_source.is_none())
        .then(preview_physics::CaseRecord::default);
    let mut support_force_vectors = HashMap::new();
    for support in &model.supports {
        if let Some(index) = node_index(model, &support.node) {
            let mut vector = [0.0; 6];
            let source_action = selected_source.as_ref().and_then(|recovery| recovery.support_actions().iter().find(|action| action.support_id == support.id));
            if let Some(action) = source_action {
                vector = action.values;
            } else {
            if let Some(linear) = built
                .supports
                .iter()
                .find(|item| item.support_id == support.id)
            {
                for dof in &linear.restrained_dofs {
                    let slot = dof_index(*dof);
                    if slot < 6 {
                        vector[slot] = reactions[index * DOF_PER_NODE + slot];
                    }
                }
            }
            for (spring_index, spring) in spring_entries
                .iter()
                .enumerate()
                .filter(|(_, item)| item.support_id == support.id)
            {
                let global = spring.node_dof.global_index();
                if global % DOF_PER_NODE < 6 {
                    // F1b (§4.7 step 5): at b != 0, K2b's checked
                    // `force_scaled_spring_action`, unscaled once.
                    vector[global % DOF_PER_NODE] = match &w2_publication {
                        Some(publication) => publication.spring_actions[spring_index].value,
                        None => -spring.stiffness.value * displacements[global],
                    };
                }
            }
            if let Some(nonlinear) = built
                .nonlinear_supports
                .iter()
                .find(|item| item.support_id == support.id)
            {
                let slot = dof_index(nonlinear.dof);
                if slot < 6 {
                    vector[slot] = reactions[index * DOF_PER_NODE + slot];
                }
            }
            }
            if let Some(action) = source_action.filter(|_| !pressure_runtime::is_exact(model)) {
                for (component, name) in ["Fx", "Fy", "Fz", "Mx", "My", "Mz"].iter().enumerate() {
                    results.push(ResultItem {
                        id: format!("result:support-component-v2:{}:{name}", exact_source_identity(&[&load_case.id, &support.id])),
                        kind: "support_reaction_component_v2".into(),
                        value: action.values[component],
                        unit: if component < 3 { "N" } else { "N*m" }.into(),
                        entity_ref: support.id.clone(), basis_ref: None, source_result_refs: Vec::new(),
                        metadata: Some(ResultMetadata {
                            basis: "recovered_from_assembled_support_law".into(), component: (*name).into(),
                            coordinate_system: "global".into(), location: "node".into(),
                            sign_convention: "support_on_pipe_positive_global_force_right_hand_couple_at_attachment_node".into(),
                        }),
                    });
                }
            }
            require_finite_mechanics(vector)?;
            if let Some(record) = preview_record.as_mut() {
                record.support_vectors.push((support.id.clone(), vector));
            }
            let force_vector = [vector[0], vector[1], vector[2]];
            let magnitude = if selected_source.is_some() {
                source_receipt::scaled_norm(force_vector)
            } else {
                norm3(force_vector[0], force_vector[1], force_vector[2])
            };
            support_force_vectors.insert(support.id.clone(), [vector[0], vector[1], vector[2]]);
            if pressure_runtime::is_exact(model) {
                if let Some(selected) = selected_source.as_mut() {
                    match source_receipt::composite_support_norms(&recovery_input(), selected, &support.id) {
                        Ok(norms) => {
                            append_signed_support_results(&mut results, load_case, support, vector);
                            let end = results.len();
                            results[end - 2].value = norms[0];
                            results[end - 1].value = norms[1];
                        }
                        Err(error) => diagnostics.push(diag(
                            &format!("diagnostic:source-recovery:{}:{}:support-norm", load_case.id, support.id),
                            "SOURCE_BLOCK_RECOVERY_DERIVED_UNAVAILABLE", "blocking", error.0,
                            vec![load_case.id.clone(), support.id.clone()],
                        )),
                    }
                } else { append_signed_support_results(&mut results, load_case, support, vector); }
            }
            if !pressure_runtime::is_exact(model) {
                results.push(ResultItem {
                    id: format!("result:reaction:{}", stable_suffix(&support.id)),
                    kind: "reaction_resultant".to_string(),
                    value: magnitude,
                    unit: "N".to_string(),
                    entity_ref: support.id.clone(),
                    basis_ref: None,
                    source_result_refs: Vec::new(),
                    metadata: None,
                });
            }
        }
    }

    append_constant_effort_support_results(
        &mut results,
        diagnostics,
        model,
        &displacements,
        load_case,
    );

    // S11-G R-b' (section 4): (member, end, q, B) of each straight member end
    // published from the formed K_e * u, decided after the loop.
    let mut recovery_records = Vec::new();
    let mut max_stress = None;
    let mut pipe_stress_extrema = Vec::new();
    let mut unavailable_stress_maximum_members = Vec::new();
    let mut component_stress_modifier_count = 0;
    for (pipe_index, pipe) in built.pipes.iter().enumerate() {
        // T4-U3 (S20): a replaced span has no member rows, maxima or
        // recovery record; its connector's rows follow the loop.
        if built.connector_records.iter().any(|r| r.span_index == pipe_index) {
            continue;
        }
        let macro_bend = curved_bends_by_pipe.get(&pipe_index).copied();
        let uniform_intensities = curved_bend_uniform_intensities
            .get(&pipe_index)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let straight_loads = if macro_bend.is_none() {
            match straight_local_uniform_loads(
                pipe,
                pipe_index,
                &load_application.element_uniform_loads,
            ) {
                Ok(loads) => loads,
                Err(error) => {
                    diagnostics.push(diag(
                        "diagnostic:load:straight-recovery",
                        "ELEMENT_FORCE_RECOVERY_FAILED",
                        "blocking",
                        error.to_string(),
                        vec![pipe.element_id.clone()],
                    ));
                    continue;
                }
            }
        } else {
            Vec::new()
        };
        let mut exact_mechanical_local_forces = None;
        let recovered_member = selected_source.as_ref().and_then(|recovery| recovery.members().get(pipe_index));
        let corrected_local_forces = if let Some(member) = recovered_member {
            member.end_forces.to_vec()
        } else if let Some(bend) = macro_bend {
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
                uniform_intensities,
                bend_pressure::recovery_strain(exact_pressure.as_ref(), pipe_index),
            ) {
                Ok(local_forces) => local_forces,
                Err(message) => {
                    diagnostics.push(diag(
                        &format!("diagnostic:stress:{}", stable_suffix(&pipe.element_id)),
                        "ELEMENT_FORCE_RECOVERY_FAILED",
                        "blocking",
                        message,
                        vec![pipe.element_id.clone()],
                    ));
                    continue;
                }
            }
        } else {
            // F1b (§4.7 step 5): at b != 0 the elastic end actions are K2b's
            // checked `FrameElement::force_scaled_end_actions`, unscaled once;
            // admission leaves no load term to add at scale (A2.5).
            let local_forces = match &w2_publication {
                Some(publication) => publication.end_actions[pipe_index].map(|action| action.value),
                None => match pipe.recover_local_forces_from_global_model(&displacements) {
                    Ok(local) => local.local_forces,
                    Err(error) => {
                        diagnostics.push(diag(
                            &format!("diagnostic:stress:{}", stable_suffix(&pipe.element_id)),
                            "ELEMENT_FORCE_RECOVERY_FAILED",
                            "blocking",
                            error.to_string(),
                            vec![pipe.element_id.clone()],
                        ));
                        continue;
                    }
                },
            };
            let equivalent_terms =
                match pipe.equivalent_nodal_load_terms_with_spans(&straight_loads, &[]) {
                    Ok(value) => value,
                    Err(error) => {
                        diagnostics.push(diag(
                            "diagnostic:load:straight-fixed-end",
                            "ELEMENT_FORCE_RECOVERY_FAILED",
                            "blocking",
                            error.to_string(),
                            vec![pipe.element_id.clone()],
                        ));
                        continue;
                    }
                };
            let mut wall = exact_straight_end_forces(
                &local_forces,
                &equivalent_terms,
                pipe_index,
                &thermal_loads,
            );
            if let Some(state) = exact_pressure
                .as_ref()
                .and_then(|case| case.pipe_states.get(&pipe_index))
            {
                exact_mechanical_local_forces = Some(wall.clone());
                let ends = (
                    state.annulus.recover_wall_effective_membrane(
                        -wall[UX],
                        state.material,
                        state.pressure,
                    ),
                    state.annulus.recover_wall_effective_membrane(
                        wall[DOF_PER_NODE + UX],
                        state.material,
                        state.pressure,
                    ),
                );
                match ends {
                    (Ok(i), Ok(j)) => {
                        wall[UX] = -i.0;
                        wall[DOF_PER_NODE + UX] = j.0;
                    }
                    _ => {
                        diagnostics.push(diag(
                            "diagnostic:exact-pressure:wall-action",
                            "EXACT_PRESSURE_RECOVERY_FAILED",
                            "blocking",
                            "source pressure wall action cannot be represented",
                            vec![load_case.id.clone(), pipe.element_id.clone()],
                        ));
                        continue;
                    }
                }
            }
            wall
        };
        require_finite_mechanics(corrected_local_forces.iter().copied())?;
        if recovered_member.is_none() && macro_bend.is_none() {
            let bounds = pipe
                .bending_formation_bound(&displacements)
                .map_err(|error| error.to_string());
            let end = |offset: usize, index: usize| {
                (
                    norm2(corrected_local_forces[offset + RY], corrected_local_forces[offset + RZ]),
                    bounds.clone().map(|b| b[index]),
                )
            };
            recovery_records.push(formation_guard::RecoveryRecord {
                member: pipe.element_id.clone(),
                body: formation_bodies.body_of_node(pipe.node_i.index),
                ends: [end(0, 0), end(DOF_PER_NODE, 1)],
            });
        }
        append_element_force_results(&mut results, &pipe.element_id, &corrected_local_forces);
        // Raw end rows remain node-on-element actions. Stress recovery and
        // station rows consume the common j-side section-cut convention.
        let station_resultants = if let Some(member) = recovered_member {
            [("quarter_1", 1usize), ("midspan", 2), ("quarter_3", 3)].into_iter()
                .map(|(location, index)| StationResultants { location, resultants: member.sections[index] }).collect::<Vec<_>>()
        } else if let Some(bend) = macro_bend {
            match curved_bend_station_resultants(
                bend,
                pipe,
                &corrected_local_forces,
                uniform_intensities,
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
                        "blocking",
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
            let mut stations = Vec::new();
            for (location, fraction) in [("quarter_1", 0.25), ("midspan", 0.5), ("quarter_3", 0.75)]
            {
                match straight_section_resultants(
                    pipe,
                    &corrected_local_forces,
                    &straight_loads,
                    fraction,
                ) {
                    Ok(resultants) => stations.push(StationResultants {
                        location,
                        resultants,
                    }),
                    Err(error) => diagnostics.push(diag(
                        "diagnostic:stress:straight-station",
                        "ELEMENT_FORCE_RECOVERY_FAILED",
                        "blocking",
                        error.to_string(),
                        vec![pipe.element_id.clone()],
                    )),
                }
            }
            stations
        };
        let endpoint_resultants = if let Some(member) = recovered_member {
            if pressure_runtime::is_exact(model) {
                // Composite stresses and endpoint maxima share the actual
                // retained section-cut functionals, including endpoint zeros.
                [member.sections[0], member.sections[4]]
            } else {
                // Retained source-blocks-1 keeps its original publication basis.
                [std::array::from_fn(|i| -member.end_forces[i]), std::array::from_fn(|i| member.end_forces[6 + i])]
            }
        } else if let Some(bend) = macro_bend {
            let evaluate = |fraction| {
                curved_bend_section_resultants(
                    bend,
                    pipe,
                    &corrected_local_forces,
                    uniform_intensities,
                    fraction,
                )
            };
            match (evaluate(0.0), evaluate(1.0)) {
                (Ok(end_i), Ok(end_j)) => [end_i, end_j],
                (Err(message), _) | (_, Err(message)) => {
                    diagnostics.push(diag(
                        &format!(
                            "diagnostic:stress:{}:endpoint-section-cut",
                            stable_suffix(&pipe.element_id)
                        ),
                        "ELEMENT_FORCE_RECOVERY_FAILED",
                        "blocking",
                        format!(
                            "curved-bend span {} could not evaluate endpoint section resultants: {message}",
                            pipe.element_id
                        ),
                        vec![
                            pipe.element_id.clone(),
                            bend.component_id.clone(),
                            load_case.id.clone(),
                        ],
                    ));
                    continue;
                }
            }
        } else {
            let evaluate = |fraction| {
                straight_section_resultants(
                    pipe,
                    &corrected_local_forces,
                    &straight_loads,
                    fraction,
                )
            };
            match (evaluate(0.0), evaluate(1.0)) {
                (Ok(end_i), Ok(end_j)) => [end_i, end_j],
                (Err(error), _) | (_, Err(error)) => {
                    diagnostics.push(diag(
                        "diagnostic:stress:straight-endpoint-section-cut",
                        "ELEMENT_FORCE_RECOVERY_FAILED",
                        "blocking",
                        error.to_string(),
                        vec![pipe.element_id.clone()],
                    ));
                    continue;
                }
            }
        };
        let (station_basis, station_sign_convention, station_coordinate_system) = if macro_bend
            .is_some()
        {
            (
                SECTION_RESULTANT_BASIS,
                CURVED_BEND_SECTION_SIGN_CONVENTION,
                "element_local",
            )
        } else {
            (
                SECTION_RESULTANT_BASIS,
                "positive value follows the j-side section action in the element-local frame (x toward end j); section equilibrium from stiffness-recovered end actions with consistent distributed-load fixed-end correction",
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
        if let Some(record) = preview_record.as_mut() {
            record.members.push(preview_physics::MemberRecord {
                pipe_id: pipe.element_id.clone(),
                arc: macro_bend.is_some(),
                maximum: macro_bend.is_none().then(|| {
                    exact_straight_summary_extrema(pipe, &corrected_local_forces, &straight_loads, section, None)
                }),
                end_resultants: endpoint_resultants,
                section_modulus: section.section_modulus,
            });
        }
        let end_i_stress = recover_section_stress(&endpoint_resultants[0], section);
        let end_j_stress = recover_section_stress(&endpoint_resultants[1], section);
        let station_stresses = station_resultants
            .iter()
            .map(|station| {
                (
                    station.location,
                    recover_section_stress(&station.resultants, section),
                )
            })
            .collect::<Vec<_>>();
        for stress in std::iter::once(&end_i_stress)
            .chain(std::iter::once(&end_j_stress))
            .chain(station_stresses.iter().map(|(_, stress)| stress))
        {
            let c = &stress.components;
            require_finite_mechanics(
                [
                    c.axial_normal,
                    c.bending_normal_y,
                    c.bending_normal_z,
                    c.torsional_shear,
                ]
                .into_iter()
                .flatten(),
            )?;
        }
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
                if macro_bend.is_some() {
                    CURVED_BEND_SECTION_SIGN_CONVENTION
                } else {
                    STRAIGHT_ENDPOINT_SECTION_SIGN_CONVENTION
                },
            );
        }
        if end_j_stress.findings.is_empty() {
            append_endpoint_stress_results(
                &mut results,
                &pipe.element_id,
                "end_j",
                &end_j_stress.components,
                if macro_bend.is_some() {
                    CURVED_BEND_SECTION_SIGN_CONVENTION
                } else {
                    STRAIGHT_ENDPOINT_SECTION_SIGN_CONVENTION
                },
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
                if macro_bend.is_some() {
                    station_basis
                } else {
                    "recovered_from_open_mechanics_stress_components"
                },
                macro_bend
                    .is_some()
                    .then_some(CURVED_BEND_SECTION_SIGN_CONVENTION),
            );
        }
        let mut summary_values = [
            open_formula_summary_mpa(&end_i_stress),
            open_formula_summary_mpa(&end_j_stress),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        for (_, stress) in &station_stresses {
            if let Some(value) = open_formula_summary_mpa(stress) {
                summary_values.push(value);
            }
        }
        if macro_bend.is_none() && !pressure_runtime::is_exact(model) && recovered_member.is_none() {
            match straight_summary_extrema(
                pipe,
                &corrected_local_forces,
                &straight_loads,
                section,
            ) {
                Ok(value) => summary_values.push(value),
                Err(error) => diagnostics.push(diag(
                    "diagnostic:stress:straight-extrema",
                    "ELEMENT_FORCE_RECOVERY_FAILED",
                    "blocking",
                    error.to_string(),
                    vec![pipe.element_id.clone()],
                )),
            }
        }
        let summary_value = if pressure_runtime::is_exact(model) && selected_source.is_some() {
            // T4-U2a (T4-I13 open item): the endpoint recipe holds for an
            // unloaded circular straight span only; the arc policy gates it.
            // The withheld reason is a static text borrowed, not formatted, inside
            // the per-pipe loop (T3 O-10, as T4-U0's); the warning below takes it
            // as `error.0`, like the composite error's own text.
            let maximum = match pressure_runtime::exact_member_maximum_policy(macro_bend.is_some()) {
                pressure_runtime::ExactMemberMaximumPolicy::Withhold(reason) => Err((std::borrow::Cow::Borrowed(reason),)),
                pressure_runtime::ExactMemberMaximumPolicy::Compute => source_receipt::composite_member_maximum(
                    &recovery_input(), selected_source.as_mut().expect("selected source"), &pipe.element_id,
                ).map_err(|error| (std::borrow::Cow::Owned(error.0),)),
            };
            match maximum {
                Ok(maximum) => {
                    let value = maximum.value_pa();
                    let result_id = format!("result:elastic-maximum:{}:{}:{}:{}", load_case.id.len(), load_case.id, pipe.element_id.len(), pipe.element_id);
                    results.push(ResultItem { id: result_id.clone(), kind: "pipe_elastic_normal_stress_maximum_v2".into(), value, unit: "Pa".into(), entity_ref: pipe.element_id.clone(), basis_ref: None, source_result_refs: Vec::new(), metadata: Some(ResultMetadata {
                        component: "maximum_absolute_normal_stress".into(), coordinate_system: "pipe_section".into(), location: "governing_station".into(), basis: "retained_source_endpoint_normal_max_v1".into(),
                        sign_convention: "nonnegative maximum absolute axial-plus-bending normal stress over an unloaded circular straight span; retained endpoint actions with projected-action and arithmetic bounds; endpoint witness does not imply uniqueness; torsional shear separate".into(),
                    }) });
                    if max_stress.as_ref().is_none_or(|q: &LocatedQuantity| value > q.value || (value == q.value && pipe.element_id < q.location_ref)) {
                        max_stress = Some(LocatedQuantity { value, unit: "Pa".into(), location_ref: pipe.element_id.clone(), result_ref: result_id.clone() });
                    }
                    pipe_stress_extrema.push(maximum.evidence(&result_id));
                }
                Err(error) => {
                    unavailable_stress_maximum_members.push(pipe.element_id.clone());
                    diagnostics.push(diag(&format!("diagnostic:source-recovery:{}:{}:maximum",load_case.id,pipe.element_id), "SOURCE_ENDPOINT_MAXIMUM_UNAVAILABLE", "warning", error.0, vec![load_case.id.clone(),pipe.element_id.clone()]));
                }
            }
            None
        } else if pressure_runtime::is_exact(model) {
            let extrema = match pressure_runtime::exact_member_maximum_policy(macro_bend.is_some()) {
                // T4-U0 (O-10): the withheld reason is a static text borrowed, not
                // formatted, inside the per-pipe loop; the warning below formats it.
                pressure_runtime::ExactMemberMaximumPolicy::Withhold(reason) => {
                    Err(std::borrow::Cow::Borrowed(reason))
                }
                pressure_runtime::ExactMemberMaximumPolicy::Compute => exact_straight_summary_extrema(
                    pipe,
                    exact_mechanical_local_forces
                        .as_ref()
                        .map(Vec::as_slice)
                        .unwrap_or(&corrected_local_forces),
                    &straight_loads,
                    section,
                    exact_pressure
                        .as_ref()
                        .and_then(|case| case.pipe_states.get(&pipe_index)),
                )
                .map_err(std::borrow::Cow::Owned),
            };
            match extrema {
                Ok(maximum) => {
                    let value =
                        maximum.value_lower + 0.5 * (maximum.value_upper - maximum.value_lower);
                    let result_id = format!(
                        "result:elastic-maximum:{}:{}:{}:{}",
                        load_case.id.len(),
                        load_case.id,
                        pipe.element_id.len(),
                        pipe.element_id
                    );
                    results.push(ResultItem { id:result_id.clone(),kind:"pipe_elastic_normal_stress_maximum_v2".to_string(),value,unit:"Pa".to_string(),entity_ref:pipe.element_id.clone(),basis_ref:None,source_result_refs:Vec::new(),metadata:Some(ResultMetadata {
                        component:"maximum_absolute_normal_stress".to_string(),coordinate_system:"pipe_section".to_string(),location:"governing_station".to_string(),basis:"recovered_from_open_mechanics_stress_components".to_string(),
                        sign_convention:"nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim".to_string() }) });
                    if max_stress
                        .as_ref()
                        .map(|q: &LocatedQuantity| {
                            value > q.value
                                || (value == q.value && pipe.element_id < q.location_ref)
                        })
                        .unwrap_or(true)
                    {
                        max_stress = Some(LocatedQuantity {
                            value,
                            unit: "Pa".to_string(),
                            location_ref: pipe.element_id.clone(),
                            result_ref: result_id.clone(),
                        });
                    }
                    pipe_stress_extrema.push(serde_json::json!({"pipe_id":pipe.element_id,"result_id":result_id,
                        "station_fraction":maximum.station,"span_index":maximum.span_index,"local_fraction":maximum.local_fraction,
                        "value_lower_pa":maximum.value_lower,"value_upper_pa":maximum.value_upper,"global_upper_bound_pa":maximum.upper_bound,
                        "certified_gap_pa":maximum.certified_gap,"subdivisions":maximum.subdivisions,
                        "approximation":"piecewise_quadratic_straight_section_statics","coefficient_basis":"j_side_section_equilibrium_binary64",
                        "enclosure_scope":"supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate"}));
                }
                Err(error) => {
                    unavailable_stress_maximum_members.push(pipe.element_id.clone());
                    // T4-U2: an arc's withheld maximum recurs in every case, so
                    // its id is case-qualified; a straight member's id keeps
                    // its bytes (SP-1).
                    let id = if macro_bend.is_some() {
                        format!("diagnostic:exact-stress:{}:{}:extrema", stable_suffix(&load_case.id), stable_suffix(&pipe.element_id))
                    } else {
                        format!("diagnostic:exact-stress:{}:extrema", stable_suffix(&pipe.element_id))
                    };
                    diagnostics.push(diag(&id,"EXACT_STRESS_GOVERNING_MAXIMUM_UNAVAILABLE","warning",format!("signed physical rows remain available; governing circular-normal-stress maximum is unavailable: {error}"),vec![load_case.id.clone(),pipe.element_id.clone()]));
                }
            }
            None
        } else {
            summary_values.into_iter().reduce(f64::max)
        };
        if let Some(value) = summary_value {
            let result_id = format!("result:stress:{}", stable_suffix(&pipe.element_id));
            if max_stress
                .as_ref()
                .map(|q: &LocatedQuantity| value > q.value)
                .unwrap_or(true)
            {
                max_stress = Some(LocatedQuantity {
                    value: value,
                    unit: "MPa".to_string(),
                    location_ref: pipe.element_id.clone(),
                    result_ref: result_id.clone(),
                });
            }
            results.push(ResultItem {
                id: result_id,
                kind: "open_formula_stress_summary".to_string(),
                value: value,
                unit: "MPa".to_string(),
                entity_ref: pipe.element_id.clone(),
                basis_ref: None,
                source_result_refs: Vec::new(),
                metadata: None,
            });
        }
        if let (Some(state), true) = (
            exact_pressure.as_ref().and_then(|case| case.pipe_states.get(&pipe_index)),
            macro_bend.is_some(),
        ) {
            // T4-U2 (H-2): a realized arc's pressure rows.
            bend_pressure::append_arc_pressure_results(
                &mut results,
                diagnostics,
                load_case,
                &pipe.element_id,
                state,
                &endpoint_resultants,
                &station_resultants,
            );
        } else if let Some(state) = exact_pressure
            .as_ref()
            .and_then(|case| case.pipe_states.get(&pipe_index))
        {
            append_exact_pressure_results(
                &mut results,
                diagnostics,
                load_case,
                &pipe.element_id,
                state,
                &corrected_local_forces,
                exact_mechanical_local_forces.as_deref(),
                pipe,
                &straight_loads,
            );
        }
        component_stress_modifier_count += append_component_stress_multiplier_results(
            &mut results,
            diagnostics,
            model,
            &load_case.id,
            &pipe.element_id,
            &end_i_stress,
            &end_j_stress,
        );
    }

    if !recovery_records.is_empty() {
        let scales = formation_guard::moment_scales(
            &results,
            &formation_entity_bodies(model, built, &formation_bodies),
            &curved_bends_by_pipe
                .values()
                .map(|bend| bend.pipe_id.clone())
                .collect(),
            &formation_bodies,
        );
        if let Some(finding) =
            formation_guard::recovery_finding(&recovery_records, &scales, &load_case.id)
        {
            let demoted = formation_guard::amend_integrity_report(
                diagnostics,
                &integrity_diagnostic_id(&load_case.id),
                &finding,
            );
            // I61 U1 (G-b): no wire member carries R-b''s finding.
            if let (true, Some(observer)) = (demoted, product.as_deref_mut()) {
                observer.ordinary_recovery_demoted(&load_case.id);
            }
        }
    }
    require_finite_mechanics(results.iter().map(|row| row.value))?;
    let pressure_assembly_evidence = exact_pressure.as_ref().map(|p| p.assembly_evidence.clone());
    let pressure_evidence = exact_pressure
        .map(|mut p| {
            for evidence in &mut p.evidence {
                let region_id = evidence
                    .get("region_id")
                    .and_then(|id| id.as_str())
                    .unwrap_or("");
                let members = p
                    .pipe_states
                    .iter()
                    .filter(|(_, state)| state.region_id == region_id)
                    .map(|(index, _)| built.pipes[*index].element_id.as_str())
                    .collect::<HashSet<_>>();
                evidence["result_ids"] = serde_json::json!(results
                    .iter()
                    .filter(|r| members.contains(r.entity_ref.as_str())
                        && r.kind.starts_with("pipe_")
                        && r.kind.ends_with("_v2"))
                    .map(|r| &r.id)
                    .collect::<Vec<_>>());
            }
            p.evidence
        })
        .unwrap_or_default();
    if pressure_runtime::is_exact(model) && !unavailable_stress_maximum_members.is_empty() {
        max_stress = None;
    }
    append_connector_results(built, &displacements, &load_case.id, &mut results, diagnostics);
    let exact_case_evidence = pressure_runtime::is_exact(model).then(|| {
        let mut evidence = serde_json::json!({
        "load_case_id":load_case.id,"profile_mode":pressure_runtime::exact_contract(model).map(|c| c.mode()),
        "material_basis":modulus_basis_record.unwrap_or("base_material_common_E_nu"),
        "pressure_rhs_assembly":pressure_assembly_evidence,
        "pipe_sections":built.pipes.iter().map(|pipe| exact_section_evidence(&pipe.element_id,*built.exact_sections.get(&pipe.element_id).expect("every built exact member has source geometry"))).collect::<Vec<_>>(),
        "pipe_stress_extrema":pipe_stress_extrema,
        "stress_maximum_coverage":{"complete":unavailable_stress_maximum_members.is_empty(),"unavailable_pipe_ids":unavailable_stress_maximum_members},
        "pipe_materials":if load_state.is_some() { Vec::new() } else { model.pipe_segments.iter().map(|pipe| {
            let material=materials.iter().find(|m| m.id==pipe.material).expect("built exact member material is validated");
            let thermal_consumed=load_case.primitive_loads.iter().any(|load| load.category=="thermal" && is_temperature_change_dimension(&load.dimension) && matches!(&load.target,LoadTargetInput::Element{pipe:target} if target==&pipe.id));
            serde_json::json!({"pipe_id":pipe.id,"material_id":material.id,
                "E_pa":material.elastic_modulus.value,"nu":material.poisson_ratio.as_ref().expect("validated exact nu").value,
                "G_pa":material.shear_modulus.as_ref().expect("validated exact G").value,"constitutive_basis":material.constitutive_basis,
                "thermal_consumed":thermal_consumed,"alpha_per_kelvin":if thermal_consumed {material.thermal_expansion_coefficient.as_ref().map(|a|a.value)} else {None},
                "provenance":material.provenance})
        }).collect::<Vec<_>>() }
        });
        if let Some(state) = load_state {
            // The same resolved per-member pair that built stiffness and
            // pressure recovery; the case-wide base record is not consulted.
            evidence["material_basis"] = serde_json::json!("resolved_per_member_load_reference_state_v1");
            evidence["pipe_materials"] = load_state_pipe_materials(state);
        }
        // T4-U3 (S21): no replaced span in any per-pipe evidence list.
        joint::exclude_replaced_spans(&mut evidence, &replaced_span_ids_of(built));
        evidence
    });
    let source_selected = selected_source.is_some();
    let load_state_evidence = load_state.map(|state| {
        let mut record = load_state_case_record(state, solver_mode, source_selected);
        // T4-U3 (S21, RV6 C-2): the replaced span's resolved member record and
        // eigenstrain contribution are not published (it is still resolved).
        joint::exclude_replaced_spans(&mut record, &replaced_span_ids_of(built));
        record
    });
    // Retained-source custody/finalization re-derives each case through the
    // same pipeline that produced it: the resolver for 0.4.0, the case-wide
    // material methods otherwise.
    let source_case = if let Some(capture) = capture {
        let qualified = qualify_source_case_rows(model, &load_case.id, &results);
        let finalized = if let Some(selected) = selected_source.take() {
            match source_row_bindings(&selected, &qualified) {
                Ok(bindings) => {
                    if pressure_runtime::is_exact(model) {
                        let mut physical = exact_case_evidence.clone().expect("exact physical evidence");
                        physical["recovery_method"] = serde_json::json!("retained_source_blocks_exact_v1");
                        source_receipt::FinalizedSourceBlockCase::composite_exact(capture, recovery_input(), selected, ordinary_attempt, &qualified, &bindings, &physical, load_state_evidence.as_ref())
                    } else {
                        source_receipt::FinalizedSourceBlockCase::exact(capture, recovery_input(), selected, ordinary_attempt, &qualified, &bindings)
                    }
                },
                Err(message) => Err(source_receipt::ReceiptError(message, Some(selected.summary().work))),
            }
        } else if let Some(failure) = &source_failure {
            source_receipt::FinalizedSourceBlockCase::failed(capture, &load_case.id, ordinary_attempt, failure, &format!("diagnostic:source-recovery:{}", load_case.id), &qualified)
        } else if pressure_runtime::is_exact(model) {
            let mut physical = exact_case_evidence.clone().expect("exact physical evidence");
            physical["recovery_method"] = serde_json::json!(if solver_mode == PreviewSolverMode::DenseScrutiny { "ordinary_dense_structural_v1" } else { "ordinary_sparse_structural_v1" });
            source_receipt::FinalizedSourceBlockCase::ordinary_physics(capture, &load_case.id, ordinary_attempt, &qualified, &physical, &pressure_evidence, load_state_evidence.as_ref())
        } else {
            source_receipt::FinalizedSourceBlockCase::ordinary(capture, &load_case.id, ordinary_attempt, &qualified)
        };
        match finalized {
            Ok(case) => {
                if source_selected { source_budget.debit(case.charged_work(), false); }
                if source_selected && !case.is_qualified() {
                    diagnostics.push(diag(&format!("diagnostic:source-recovery:{}:derived-unqualified", load_case.id), "SOURCE_RECOVERY_DERIVED_UNQUALIFIED", "warning", "Primary source projections are retained, but an unsupported derived row withholds whole-envelope numerical use", vec![load_case.id.clone()]));
                }
                Some(case)
            }
            Err(error) => {
                if source_selected {
                    if load_state.is_some() {
                        source_budget.record_load_state_join_failure(format!(
                            "case {}: {}",
                            load_case.id, error.0
                        ));
                    }
                    if let Some(work) = error.1 { source_budget.debit(work.charged, true); }
                    diagnostics.push(diag(&format!("diagnostic:source-recovery:{}:finalization", load_case.id), "SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED", "blocking", format!("{}; actual_finalization_work={:?}", error.0, error.1), vec![load_case.id.clone()]));
                }
                None
            }
        }
    } else { None };
    if let Some(observer) = product.as_deref_mut() {
        observer.prepared_case_source(source_selected, model, built, materials, load_case,
            restrained_dofs, spring_entries, &load_application, &thermal_loads);
    }
    Ok(LoadCaseSolve {
        load_state_evidence,
        exact_case_evidence,
        pressure_evidence,
        source_case,
        source_selected,
        load_case_id: load_case.id.clone(),
        results,
        max_displacement,
        max_stress,
        component_stress_modifier_count,
        component_pressure_thrust_load_count,
        support_force_vectors,
        preview: preview_record,
    })
}

fn qualify_source_case_rows(model: &PreviewModel, case_id: &str, rows: &[ResultItem]) -> Vec<ResultItem> {
    let is_default = model.load_cases.first().is_some_and(|case| case.id == case_id);
    let ids: HashMap<_, _> = rows.iter().map(|row| (row.id.clone(), if is_default || (pressure_runtime::is_exact(model) && row.kind.ends_with("_v2")) { row.id.clone() } else { qualified_load_case_result_id(case_id, &row.id) })).collect();
    rows.iter().cloned().map(|mut row| {
        row.id = ids[&row.id].clone();
        row.basis_ref = Some(ResultBasisRef { ref_type: "load_case".into(), ref_id: case_id.into() });
        for reference in &mut row.source_result_refs {
            if let Some(qualified) = ids.get(reference) { *reference = qualified.clone(); }
        }
        row
    }).collect()
}

fn source_row_bindings(
    selected: &source_recovery::SelectedSourceRecovery,
    rows: &[ResultItem],
) -> Result<Vec<source_receipt::FunctionalRowBinding>, String> {
    use open_pipe_stress_frame_kernel::structural::exact_boundary::functionals::{FunctionalQuantity, MemberEnd};
    let force_kinds = ["element_local_axial_force", "element_local_shear_force_y", "element_local_shear_force_z", "element_local_torsional_moment", "element_local_bending_moment_y", "element_local_bending_moment_z"];
    let node_kinds = ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z", "global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z"];
    let mut bindings = Vec::new();
    for (index, descriptor) in selected.retained().descriptors().iter().enumerate() {
        let (entity, kind, location, component) = match &descriptor.key.quantity {
            FunctionalQuantity::NodeDisplacement { node, dof } => (node.as_str(), node_kinds[dof % 6], "node", None),
            FunctionalQuantity::MemberEnd { member, end, row } => (member.as_str(), force_kinds[*row as usize], if *end == MemberEnd::I { "end_i" } else { "end_j" }, None),
            FunctionalQuantity::MemberSection { member, station_bits, component } => {
                let station = f64::from_bits(*station_bits);
                let location = if station == 0.25 { "quarter_1" } else if station == 0.5 { "midspan" } else if station == 0.75 { "quarter_3" } else { continue };
                (member.as_str(), force_kinds[*component as usize], location, None)
            }
            FunctionalQuantity::SupportAction { support, component, .. } => (support.as_str(), "support_reaction_component_v2", "node", Some(["Fx", "Fy", "Fz", "Mx", "My", "Mz"][*component as usize])),
            _ => continue,
        };
        let matched: Vec<_> = rows.iter().filter(|row| row.entity_ref == entity && row.kind == kind && row.metadata.as_ref().is_some_and(|metadata| metadata.location == location && component.map_or(true, |name| metadata.component == name))).collect();
        if matched.len() != 1 { return Err(format!("source functional row binding is not unique: {entity}/{kind}/{location}")); }
        bindings.push(source_receipt::FunctionalRowBinding { functional_index: index, result_id: matched[0].id.clone() });
    }
    Ok(bindings)
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
    model: &PreviewModel,
    built: &BuiltModel,
    restrained_dofs: &[usize],
    force: &AssembledForce,
    load_case: &PreviewLoadCase,
    solver_mode: PreviewSolverMode,
    spring_entries: &[SpringEntry],
) -> Option<NonlinearFrameSolveResult> {
    if built.nonlinear_supports.is_empty() {
        return None;
    }

    // The assembled active-set loop consumes the same build-time validated
    // curved-bend macro-element global stiffness as the linear preview paths
    // (DEC-070): each realized bend is passed as an explicit-stiffness slot so
    // every linearized iteration assembles the arc stiffness. No straight-chord
    // fallback exists on this path.
    let mut curved_bend_stiffness_elements = Vec::with_capacity(built.curved_bend_elements.len());
    for element in &built.curved_bend_elements {
        match CurvedBendStiffnessElement::from_macro_element(
            element.component_id.clone(),
            &element.macro_element,
        ) {
            Ok(slot) => curved_bend_stiffness_elements.push(slot),
            Err(error) => {
                diagnostics.push(nonlinear_loop_blocked_diag(&load_case.id, error));
                return None;
            }
        }
    }

    let support_classes = product_preview_policy_support_classes(&built.nonlinear_supports);
    let support_classes_basis = support_classes.join(",");
    let convergence = match product_preview_convergence_control() {
        Ok(convergence) => convergence,
        Err(error) => {
            diagnostics.push(nonlinear_loop_blocked_diag(&load_case.id, error));
            return None;
        }
    };

    let input = NonlinearFrameSolveInput {
        node_count: built.nodes.len(),
        elements: built.frame_elements.clone(),
        connectors: built.connectors.clone(),
        curved_bend_elements: curved_bend_stiffness_elements,
        // The loop's base force is the ledger's net (S11 section 8.2); the
        // typed entry below checks this copy against it bit for bit.
        force: force.values().to_vec(),
        base_restrained_dofs: restrained_dofs.to_vec(),
        nonlinear_supports: built.nonlinear_supports.clone(),
        initial_states: built.nonlinear_initial_states.clone(),
        friction_normal_reactions: built.nonlinear_friction_normal_reactions.clone(),
        derived_friction_normal_reactions: built
            .nonlinear_derived_friction_normal_reactions
            .clone(),
        convergence,
    };

    let springs = spring_entries
        .iter()
        .map(|spring| (spring.node_dof.global_index(), spring.stiffness.value))
        .collect::<Vec<_>>();
    match solve_active_set_frame_with_mode_and_springs_assembled(
        &input,
        force,
        solver_mode.nonlinear_mode(),
        &springs,
    ) {
        Ok(solve) => {
            if solve.converged {
                match open_pipe_stress_nonlinear_integration::product_selected_state_is_qualified(
                    &input, &springs, &solve,
                ) {
                    Ok(true) => {}
                    Ok(false)
                        if matches!(
                            open_pipe_stress_nonlinear_integration::product_unsupported_gap_state_is_inspectable(
                                &input, &springs, &solve,
                            ),
                            Ok(true)
                        ) => {}
                    outcome => {
                        let error=StructuralError::NumericallyUnresolved{reason:"M03 product final same-state equilibrium/contact qualification failed",global_dof:None};
                        append_integrity_failure(diagnostics, &load_case.id, &error, model);
                        if let Err(error) = outcome {
                            diagnostics.push(nonlinear_loop_blocked_diag(&load_case.id, error));
                        }
                        return None;
                    }
                }
            }
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
                    &final_iteration.product_equilibrium,
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
                    "{} nonlinear support active-set preview completed {} iteration(s); final residual count {}; accepted active-set-count policy_ref={}; prospective evaluated force/moment equilibrium policy_ref={}; derived per-row residual-work policy_ref={}; historical general-energy alias is residual work, not total energy balance; historical policy_ref={}; accepted displacement/reaction-delta policy_ref={} for emitted product-preview delta rows; timing/RSS/hardware sparse evidence remains observational, not thresholded",
                    solver_mode.as_str(),
                    solve.iterations.len(),
                    final_residual,
                    solve.policy_ref,
                    open_pipe_stress_nonlinear_integration::product_equilibrium::POLICY,
                    open_pipe_stress_nonlinear_integration::product_equilibrium::POLICY,
                    DEC_046_PRODUCT_PREVIEW_GENERAL_ENERGY_POLICY_REF,
                    DEC_046_PRODUCT_PREVIEW_DISPLACEMENT_REACTION_DELTA_POLICY_REF
                ),
                vec![load_case.id.clone(), "DEC-046".to_string()],
            ));
            Some(solve)
        }
        Err(error) => {
            if let NonlinearIntegrationError::Structural(structural) = &error {
                append_integrity_failure(diagnostics, &load_case.id, structural, model);
            }
            diagnostics.push(nonlinear_loop_blocked_diag(&load_case.id, error));
            None
        }
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
    structural_report: StructuralReport,
    /// S11 section 6: the kernel's load-fidelity findings (a detected loss is
    /// Sensitive, never a refusal); `None` when nothing is flagged.
    load_fidelity: Option<LoadFidelityReport>,
    /// F1a: K-D5's formation-check record, `Some` only when the check demoted
    /// the case; rendered only as the integrity diagnostic's evidence line.
    formation_check: Option<FormationCheck>,
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

/// `prescribed` lists every restrained DOF with its actual boundary value
/// (explicit zero unless a resolved case prescribes motion). The structural
/// solve applies K_fc g_c itself against the original `global_force`; the legacy
/// observation lane consumes `observation_force`, which already carries it.
fn solve_preview_reduced_system(
    solver_mode: PreviewSolverMode,
    stiffness: &SparseStiffness,
    _reduced_force: &[f64],
    built: &BuiltModel,
    spring_entries: &[SpringEntry],
    global_force: &AssembledForce,
    observation_force: &[f64],
    prescribed: &[(usize, f64)],
    load_case: &PreviewLoadCase,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<PreviewLinearSolve, StructuralError> {
    let restrained_dofs = prescribed.iter().map(|&(dof, _)| dof).collect::<Vec<_>>();
    let restrained_dofs = restrained_dofs.as_slice();
    let curved = built
        .curved_bend_elements
        .iter()
        .map(|e| {
            CurvedBendStiffnessElement::from_macro_element(e.component_id.clone(), &e.macro_element)
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| StructuralError::InvalidInput("curved formation evidence"))?;
    let springs = spring_entries
        .iter()
        .map(|e| (e.node_dof.global_index(), e.stiffness.value))
        .collect::<Vec<_>>();
    // F1b (§4.8; ROOT Q8(d)): the pattern evidence in both modes. In dense
    // scrutiny SA materializes the dense view of the same values and runs
    // today's dense Cholesky path (K1: the `StructuralSolution` is
    // byte-identical in `Debug` to the dense evidence's).
    let assembly = SparseAssemblyEvidence::new(
        stiffness.pattern(),
        built.nodes.len(),
        &built.frame_elements,
        &built.connectors,
        &curved,
        &springs,
    )?;
    let free = (0..global_force.len())
        .filter(|i| !restrained_dofs.contains(i))
        .collect::<Vec<_>>();
    let mode = match solver_mode {
        PreviewSolverMode::DenseScrutiny => LinearSolveMode::DenseScrutiny,
        PreviewSolverMode::SparseInteractive => LinearSolveMode::SparseInteractive,
    };
    // K-D5 (T3 D1 §4.3.1): the linear route's typed solve with the D-5
    // formation check. An invocation with any nonlinear support is never
    // selected (ROOT): it runs the unchanged `solve_assembled` and keeps its
    // ordinary result and standing.
    let curved_sources = built
        .curved_bend_elements
        .iter()
        .map(|e| e.macro_element)
        .collect::<Vec<_>>();
    let checked = assembly.solve_assembled_with_formation_check(
        stiffness,
        global_force,
        &free,
        prescribed,
        mode,
        &curved_sources,
        built.nonlinear_supports.is_empty(),
    )?;
    // Legacy raw DEC050/053 observations retain their own unscaled algorithm.
    // They neither select the solution nor rescue a rejected structural gate.
    let direct = assemble_reduced_sparse_entry_system(
        built.nodes.len(),
        &built.frame_elements,
        &built.connectors,
        &built.curved_bend_elements,
        spring_entries,
        observation_force,
        restrained_dofs,
    )
    .ok();
    // F1b (ROOT, 2026-09-28): the lane runs only within the provisional
    // ceiling; otherwise its fields are published as not_observed, with the
    // named reason (the gate's CONT n10000 heap-cap finding).
    let lane_refusal = direct
        .as_ref()
        .and_then(|d| observation_lane_guard(d, dense_scrutiny_ceiling_bytes()).err());
    if let Some(refusal) = &lane_refusal {
        diagnostics.push(observation_lane_refusal_diagnostic(&load_case.id, refusal));
    }
    let legacy = direct
        .as_ref()
        .filter(|_| lane_refusal.is_none())
        .and_then(|d| solve_symmetric_system_from_entries(d.dimension, &d.entries, &d.force).ok());
    let residual = direct
        .as_ref()
        .zip(legacy.as_ref())
        .map(|(d, l)| max_abs_entry_residual(d.dimension, &d.entries, &l.solution, &d.force));
    Ok(PreviewLinearSolve {
        solution: free.iter().map(|&i| checked.displacements[i]).collect(),
        structural_report: checked.report,
        load_fidelity: checked.load_fidelity,
        formation_check: checked.formation_check,
        solution_basis: match solver_mode {
            PreviewSolverMode::DenseScrutiny => "dense_structural_integrity_primary",
            PreviewSolverMode::SparseInteractive => "sparse_structural_integrity_primary",
        },
        sparse_entry_count: direct.as_ref().map(|d| d.entries.len()),
        original_profile_entry_count: legacy.as_ref().map(|l| l.original_profile_entry_count),
        ordered_profile_entry_count: legacy.as_ref().map(|l| l.ordered_profile_entry_count),
        original_max_half_bandwidth: legacy.as_ref().map(|l| l.original_max_half_bandwidth),
        ordered_max_half_bandwidth: legacy.as_ref().map(|l| l.ordered_max_half_bandwidth),
        nonpositive_pivot_count: legacy
            .as_ref()
            .map(|l| l.factorization.nonpositive_pivot_count),
        pivot_condition_ratio_estimate: legacy
            .as_ref()
            .and_then(|l| l.factorization.pivot_condition_ratio_estimate),
        sparse_residual: residual,
        dense_fallback_message: None,
    })
}

fn append_linear_solver_mode_evidence(
    results: &mut Vec<ResultItem>,
    load_case_id: &str,
    solver_mode: PreviewSolverMode,
    linear_solve: &PreviewLinearSolve,
) {
    let fallback = linear_solve.dense_fallback_message.is_some();
    let basis = format!(
        "DEC-053 sparse_default_promotion; solver_mode={}; solution_basis={}; structural_policy=M03-INTEGRITY-v1; profile_pivot_residual_observation_basis=legacy_unscaled_DEC050_DEC053;  default_sparse_promotion=interactive_default; dense_scrutiny_available=true; sparse_entry_count={}; original_profile_entries={}; ordered_profile_entries={}; original_half_bandwidth={}; ordered_half_bandwidth={}; nonpositive_pivots={}; pivot_condition_ratio_proxy={}; max_abs_sparse_residual={}; dense_fallback={}; dense_fallback_message={}",
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

fn optional_usize(value: Option<usize>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "not_observed".to_string())
}

fn optional_f64(value: Option<f64>) -> String {
    value
        .filter(|value| value.is_finite())
        .map(|value| value.to_string())
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
        &built.connectors,
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

    if [
        max_abs_dense_sparse_solution_delta,
        relative_dense_sparse_solution_delta,
        sparse_residual,
    ]
    .iter()
    .any(|value| !value.is_finite())
    {
        diagnostics.push(diag(
            "diagnostic:sparse-live:nonfinite",
            "SPARSE_LIVE_PATH_EVIDENCE_UNAVAILABLE",
            "warning",
            "nonfinite computed sparse scrutiny evidence",
            vec![load_case_id.to_string()],
        ));
        return;
    }
    results.push(ResultItem {
        id: "result:sparse-live:dense-parity-relative-delta".to_string(),
        kind: "sparse_live_path_dense_parity_relative_delta".to_string(),
        value: relative_dense_sparse_solution_delta,
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
                max_abs_dense_sparse_solution_delta,
                sparse_residual,
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
    connectors: &[open_pipe_stress_frame_kernel::connector::ObjectiveConnector],
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
    // T4-U3 (S3): each connector's Ke after the frames (observation only).
    for connector in connectors {
        let (node_i, node_j) = (connector.node_i().index, connector.node_j().index);
        validate_element_nodes(node_i, node_j, node_count)?;
        let element_stiffness = connector.global_stiffness()?;
        append_reduced_element_entries(
            &mut entries,
            &global_to_reduced,
            node_i,
            node_j,
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
// frame elements and connectors (the replaced chord element is excluded in
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

fn finite_observation_max(left: f64, right: f64) -> f64 {
    if left.is_finite() && right.is_finite() {
        left.max(right)
    } else {
        f64::NAN
    }
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
        .fold(0.0, finite_observation_max)
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
        value: value,
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
    equilibrium: &open_pipe_stress_nonlinear_integration::product_equilibrium::ProductEquilibriumReport,
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
        "{}_active_set_loop; policy_ref={policy_ref}; threshold_policy_ref={}; threshold_policy_status=accepted; observed_compliance={}; residual_basis=free_dof_work_residual; work_target=per_row_abs_u_times_tau_times_original_denominator_with_evaluation_allowance; observed_governing_global_dof={:?}; general_energy_alias=residual_work_not_total_energy_balance; historical_superseded_policy_refs={},{}, historical_limits={} N*m,{} N*m; product_preview_only",
        solver_mode.as_str(),equilibrium.policy,equilibrium.passed,equilibrium.observed_governing_work_dof,DEC_046_PRODUCT_PREVIEW_FREE_DOF_WORK_POLICY_REF,DEC_046_PRODUCT_PREVIEW_GENERAL_ENERGY_POLICY_REF,DEC_046_PRODUCT_PREVIEW_FREE_DOF_WORK_ABSOLUTE_LIMIT,DEC_046_PRODUCT_PREVIEW_GENERAL_ENERGY_ABSOLUTE_LIMIT);
    let force_moment_threshold_basis = format!(
        "{}_active_set_loop; policy_ref={policy_ref}; threshold_policy_ref={}; threshold_policy_status=accepted; observed_compliance={}; residual_basis=free_dof_force_moment_equilibrium; target=64*gamma(actual_row_operation_count); original_full_equations_with_prescribed_coupling; evaluation_allowance_and_denominator_guard; historical_superseded_policy_ref={}; historical_zero_limits={} N,{} N*m; product_preview_only",
        solver_mode.as_str(),equilibrium.policy,equilibrium.passed,DEC_046_PRODUCT_PREVIEW_FREE_DOF_FORCE_MOMENT_POLICY_REF,DEC_046_PRODUCT_PREVIEW_FREE_DOF_FORCE_ABSOLUTE_LIMIT,DEC_046_PRODUCT_PREVIEW_FREE_DOF_MOMENT_ABSOLUTE_LIMIT);
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
        equilibrium.rows.iter().filter(|r|r.global_dof%6<3).map(|r|r.residual.abs()).fold(0.0,f64::max),
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
        equilibrium.rows.iter().filter(|r|r.global_dof%6>=3).map(|r|r.residual.abs()).fold(0.0,f64::max),
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
        equilibrium
            .work_rows
            .iter()
            .map(|r| r.observed_work)
            .fold(0.0, f64::max),
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
    build_model_for_members(model, materials, None, diagnostics)
}

/// Build with each pipe's constitutive pair from a resolved load/reference-state
/// case when supplied (keyed by pipe ID; G derived from that pair). Without it,
/// the historical material-ID lookup is used unchanged.
fn build_model_for_members(
    model: &PreviewModel,
    materials: &[MaterialInput],
    member_pairs: Option<&HashMap<String, pressure_exact::IsotropicENu>>,
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
    // T4-U3 (S18, replaces_span): a span replaced by an objective connector
    // is excluded from the frame assembly, as a curved span is; the
    // connector carries the stiffness between its end nodes.
    let replaced_span_ids = joint::replaced_span_ids(model);
    let mut pipes = Vec::new();
    let mut frame_elements = Vec::new();
    let mut sections = HashMap::new();
    // Each realized-bend pipe's resolved (E, G): the material's, or the 0.4.0
    // member pair. Straight pipes insert nothing (RV12 S-3).
    let mut member_moduli: HashMap<&str, (f64, f64)> = HashMap::new();
    let mut exact_sections = HashMap::new();
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
        let Some(mut derived) = derive_pipe_section(&pipe.section, &pipe.id, diagnostics) else {
            continue;
        };
        if pressure_runtime::is_exact(model) {
            let geometry = match pressure_exact::SourceAnnulus::from_od_wall(
                pipe.section.outside_diameter.value,
                derived.wall_thickness,
            ) {
                Ok(geometry) => geometry,
                Err(error) => {
                    diagnostics.push(diag(&format!("diagnostic:exact-section:{}",stable_suffix(&pipe.id)),"EXACT_SECTION_GEOMETRY_UNREPRESENTABLE","blocking",format!("source OD/effective-wall annulus requires distinct positive represented radii and finite positive area/I/J/Z: {error:?}"),vec![pipe.id.clone()]));
                    continue;
                }
            };
            derived.area = geometry.wall_area_m2();
            derived.internal_area = geometry.internal_area_m2();
            derived.second_moment = geometry.second_moment_m4();
            derived.torsion_constant = geometry.polar_moment_m4();
            derived.section_modulus = geometry.section_modulus_m3();
            exact_sections.insert(pipe.id.clone(), geometry);
        }
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
        let (elastic_modulus, shear_modulus) = match member_pairs {
            Some(pairs) => {
                let Some(pair) = pairs.get(&pipe.id) else {
                    diagnostics.push(diag(&format!("diagnostic:load-state:{}:member-material", stable_suffix(&pipe.id)), "LOAD_STATE_MEMBER_MATERIAL_MISSING", "blocking",
                        "the resolved case supplies no selected E/nu pair for this member; no material-ID fallback is used", vec![pipe.id.clone()]));
                    continue;
                };
                (pair.elastic_modulus_pa(), pair.shear_modulus_pa())
            }
            None => (
                material.elastic_modulus.value,
                material
                    .shear_modulus
                    .as_ref()
                    .expect("validated material G")
                    .value,
            ),
        };
        let section = match StraightPipeSectionProperties::new(
            elastic_modulus,
            shear_modulus,
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
        if curved_bend_pipe_ids.contains(pipe.id.as_str()) {
            member_moduli.insert(pipe.id.as_str(), (elastic_modulus, shear_modulus));
        } else if !replaced_span_ids.contains(pipe.id.as_str()) {
            frame_elements.push(
                element
                    .frame_element()
                    .expect("validated straight pipe frame element"),
            );
        }
        sections.insert(pipe.id.clone(), derived);
        pipes.push(element);
    }

    // T4-U3 (S1, S18): every admitted v3 connector is formed here or refused
    // by code (its replaced span too); none is skipped.
    let mut connectors = Vec::new();
    let mut connector_records = Vec::new();
    if pressure_runtime::exact_contract(model) == Some(pressure_runtime::ExactContract::PressureV3) {
        for component in &model.components {
            let Some(spec) = joint::connector_spec(component) else {
                continue;
            };
            let span_index = pipes
                .iter()
                .position(|pipe: &StraightPipeElement| pipe.element_id == spec.span_id);
            match joint::build_connector(&spec, &nodes, &node_map, span_index) {
                Ok((connector, record)) => {
                    connectors.push(connector);
                    connector_records.push(record);
                }
                Err(refusal) => diagnostics.push(refusal),
            }
        }
    }
    let curved_bend_elements = build_curved_bend_macro_elements(
        model,
        &nodes,
        &node_map,
        &sections,
        &member_moduli,
        diagnostics,
    );
    let nonlinear = build_nonlinear_supports(model, &node_map, diagnostics);
    let supports = model
        .supports
        .iter()
        .filter_map(|support| {
            if support.nonlinear.is_some()
                && support.restraints.is_empty()
                && support_stiffness_input(support).is_none()
            {
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
            } else {
                Some(rigid_linear_support_from_preview(support, node, dofs))
            }
        })
        .collect::<Vec<_>>();

    Some(BuiltModel {
        exact_sections,
        nodes,
        pipes,
        frame_elements,
        connectors,
        connector_records,
        curved_bend_elements,
        supports,
        nonlinear_supports: nonlinear.supports,
        nonlinear_initial_states: nonlinear.initial_states,
        nonlinear_friction_normal_reactions: nonlinear.friction_normal_reactions,
        nonlinear_derived_friction_normal_reactions: nonlinear.derived_friction_normal_reactions,
        sections,
    })
}

fn validate_support_family_tokens(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    for support in &model.supports {
        let Some(family) = support.family.as_deref() else {
            continue;
        };
        if !matches!(
            family,
            "anchor"
                | "guide"
                | "line_stop"
                | "vertical_support"
                | "spring"
                | "variable_spring_hanger"
                | "spring_hanger"
                | "constant_effort_support"
                | "nonlinear"
        ) {
            diagnostics.push(diag(
                &format!("diagnostic:support:{}:family", stable_suffix(&support.id)),
                "SUPPORT_FAMILY_INVALID",
                "blocking",
                format!(
                    "unsupported explicit support family {family:?}; supported values: anchor, guide, line_stop, vertical_support, spring, variable_spring_hanger, spring_hanger, constant_effort_support, nonlinear"
                ),
                vec![support.id.clone(), format!("{}.family", support.id)],
            ));
        }
    }
}

fn rigid_linear_support_from_preview(
    support: &PreviewSupport,
    node_index: usize,
    restrained_dofs: Vec<FrameDof>,
) -> LinearSupport {
    let family = match support.family.as_deref() {
        Some("anchor") => SupportFamily::Anchor,
        Some("guide") => SupportFamily::Guide,
        Some("line_stop") => SupportFamily::LineStop,
        Some("vertical_support") => SupportFamily::VerticalSupport,
        _ if restrained_dofs.len() == 6 => SupportFamily::Anchor,
        _ => SupportFamily::Guide,
    };

    LinearSupport {
        support_id: support.id.clone(),
        family,
        node_index,
        restrained_dofs,
        stiffness: None,
        imposed_displacement: None,
    }
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
    nodes: &[FrameNode],
    node_map: &HashMap<&str, usize>,
    sections: &HashMap<String, DerivedSection>,
    member_moduli: &HashMap<&str, (f64, f64)>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<CurvedBendMacroBuild> {
    let pipe_map = model
        .pipe_segments
        .iter()
        .enumerate()
        .map(|(index, pipe)| (pipe.id.as_str(), (index, pipe)))
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
        let (Some(section), Some(&(elastic_modulus, shear_modulus))) =
            (sections.get(pipe_ref), member_moduli.get(pipe_ref))
        else {
            // The pipe loop already emitted the blocking section/material diagnostic.
            continue;
        };

        // Arc geometry from user fields (T4-U1): the chord is the span's two
        // nodes, the bend plane is spanned by the chord and the pipe
        // y_reference, and the user bend radius fixes the sagitta. The arc
        // bows toward the positive pipe y_reference side (recorded in the
        // review-row basis). The element is formed from (x_i, x_j, R,
        // y_reference) only; no absolute centre is formed.
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
                    scalar_string(bend_radius),
                    scalar_string(chord_length)
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
                        scalar_string(user_angle),
                        scalar_string(implied_included_angle)
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
        // The single user flexibility factor applies to both the in-plane and
        // out-of-plane bending strain-energy terms (mapping recorded in the
        // review-row basis); no code-content value is derived or defaulted.
        // E and G are the pipe's resolved pair (the material's, or the 0.4.0
        // member pair; T4-U1, I1 §5.3 #12).
        let element = match CurvedBendMacroElement::new(
            nodes[from_index],
            nodes[to_index],
            bend_radius,
            y_raw,
            elastic_modulus,
            shear_modulus,
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

/// Reject contradictory materialized geometry before any solver consumes it.
/// Resolution is in-memory only; local supplemental section inputs are retained.
fn resolve_shared_sections(model: &mut PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    for pipe in &mut model.pipe_segments {
        let Some(reference) = &pipe.section_ref else {
            continue;
        };
        let refs = vec![pipe.id.clone(), reference.clone()];
        let id = format!("diagnostic:section-reference:{}", stable_suffix(&pipe.id));
        if reference.trim().is_empty() {
            diagnostics.push(diag(
                &id,
                "SECTION_REFERENCE_INVALID",
                "blocking",
                "bound section requires an explicit nonblank identity",
                refs,
            ));
            continue;
        }
        let matches: Vec<_> = model
            .sections
            .iter()
            .filter(|s| &s.id == reference)
            .collect();
        if matches.len() != 1 {
            diagnostics.push(diag(
                &id,
                "SECTION_REFERENCE_INVALID",
                "blocking",
                "bound section must exist exactly once",
                refs,
            ));
            continue;
        }
        let shared = matches[0];
        if shared.section_type != "pipe"
            || shared
                .properties
                .keys()
                .any(|key| key != "outside_diameter" && key != "wall_thickness")
        {
            diagnostics.push(diag(&id, "SECTION_BINDING_UNSUPPORTED", "blocking",
                "binding requires a pipe section with only outside_diameter and wall_thickness properties", refs));
            continue;
        }
        let (Some(od), Some(wall)) = (
            shared.properties.get("outside_diameter"),
            shared.properties.get("wall_thickness"),
        ) else {
            diagnostics.push(diag(
                &id,
                "SECTION_REFERENCE_INVALID",
                "blocking",
                "bound section requires explicit outside_diameter and wall_thickness quantities",
                refs,
            ));
            continue;
        };
        let mut resolved = pipe.section.clone();
        resolved.outside_diameter = od.clone();
        resolved.wall_thickness = wall.clone();
        let before = diagnostics.len();
        for quantity in [&mut resolved.outside_diameter, &mut resolved.wall_thickness] {
            normalize_quantity(quantity, Dimension::Length, &id, refs.clone(), diagnostics);
        }
        if let Some(tolerance) = &mut resolved.mill_tolerance {
            normalize_quantity(tolerance, Dimension::Length, &id, refs.clone(), diagnostics);
        }
        if diagnostics.len() != before {
            continue;
        }
        if derive_pipe_section(&resolved, &pipe.id, diagnostics).is_none() {
            continue;
        }
        // Stored cache must preserve the exact source representation used by
        // operation materialization, so display/export and solver agree.
        if pipe.section.outside_diameter.value != od.value
            || pipe.section.outside_diameter.unit != od.unit
            || pipe.section.wall_thickness.value != wall.value
            || pipe.section.wall_thickness.unit != wall.unit
        {
            diagnostics.push(diag(
                &id,
                "SECTION_REFERENCE_CACHE_STALE",
                "blocking",
                "inline OD/wall contradict the shared section; explicitly rematerialize before use",
                refs,
            ));
            continue;
        }
        // Copy source quantities, leaving all supplemental values untouched.
        pipe.section.outside_diameter = od.clone();
        pipe.section.wall_thickness = wall.clone();
    }
}

fn normalize_node_coordinates(model: &mut PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    let diagnostic_id = "diagnostic:unit-input:project:length";
    let affected_refs = vec![model.project.id.clone(), "project.units.length".to_string()];
    let Some(symbol) = model
        .project
        .units
        .get("length")
        .and_then(serde_json::Value::as_str)
    else {
        diagnostics.push(diag(
            diagnostic_id,
            "UNIT_INPUT_INVALID",
            "blocking",
            "preview node coordinates require explicit project.units.length metadata",
            affected_refs,
        ));
        return;
    };
    let from = match unit_by_symbol(symbol, Dimension::Length) {
        Ok(unit) => unit,
        Err(error) => {
            diagnostics.push(diag(
                diagnostic_id,
                "UNIT_INPUT_INVALID",
                "blocking",
                format!("preview project.units.length must be an accepted length unit: {error}"),
                affected_refs,
            ));
            return;
        }
    };
    let Some(to) = canonical_unit(Dimension::Length) else {
        diagnostics.push(unit_conversion_diag(
            diagnostic_id,
            "no canonical length unit is accepted",
            affected_refs,
        ));
        return;
    };
    // Positions are stored in the project length unit. Direction vectors and
    // explicit quantity records have separate semantics and are not scaled here.
    for node in &mut model.nodes {
        for coordinate in [
            &mut node.position.x,
            &mut node.position.y,
            &mut node.position.z,
        ] {
            match convert_for_dimension(*coordinate, Dimension::Length, from, to) {
                Ok(value) => *coordinate = value,
                Err(error) => diagnostics.push(unit_conversion_diag(
                    &format!(
                        "diagnostic:unit-conversion:node:{}:position",
                        stable_suffix(&node.id)
                    ),
                    error.to_string(),
                    vec![node.id.clone(), "position".to_string()],
                )),
            }
        }
    }
    model.project.units["length"] = to.definition().symbol.into();
}

fn normalize_model_units(
    model: &mut PreviewModel,
    materials: &mut [MaterialInput],
    diagnostics: &mut Vec<Diagnostic>,
) {
    normalize_node_coordinates(model, diagnostics);
    pressure_runtime::normalize_region_units(model, diagnostics);
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
        if let Some(shear_modulus) = &mut material.shear_modulus {
            normalize_quantity(
                shear_modulus,
                Dimension::Stress,
                &format!(
                    "diagnostic:unit-conversion:material:{}:shear-modulus",
                    stable_suffix(&material.id)
                ),
                vec![material.id.clone(), "shear_modulus".to_string()],
                diagnostics,
            );
        }
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
            if let Some(shear_modulus) = &mut point.shear_modulus {
                normalize_quantity(
                    shear_modulus,
                    Dimension::Stress,
                    &format!(
                        "diagnostic:unit-conversion:material:{}:temperature-point:{}:shear-modulus",
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
            for (index, span) in wind.exposed_spans.iter_mut().enumerate() {
                for (label, fraction) in [
                    ("start_fraction", &mut span.start_fraction),
                    ("end_fraction", &mut span.end_fraction),
                ] {
                    if let Some(fraction) = fraction {
                        normalize_dimensionless_quantity(
                            fraction,
                            &format!(
                                "diagnostic:unit-conversion:load-case:{}:wind-exposed-span-{index}-{label}",
                                stable_suffix(&case_id)
                            ),
                            vec![
                                case_id.clone(),
                                format!("wind.exposed_spans[{index}].{label}"),
                            ],
                            diagnostics,
                        );
                    }
                }
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
        if wind.exposed_pipe_refs.is_empty() && wind.exposed_spans.is_empty() {
            missing.push("exposed_pipe_refs or exposed_spans");
        }
        let mut span_fields_missing = false;
        for (index, span) in wind.exposed_spans.iter().enumerate() {
            let mut span_missing = Vec::new();
            if span.pipe_ref.is_none() {
                span_missing.push("pipe_ref");
            }
            if span.start_fraction.is_none() {
                span_missing.push("start_fraction");
            }
            if span.end_fraction.is_none() {
                span_missing.push("end_fraction");
            }
            if !span_missing.is_empty() {
                diagnostics.push(diag(
                    &equivalent_static_diag_id(
                        case_id,
                        &format!("wind:exposed-span-{index}:inputs"),
                    ),
                    "EQUIVALENT_STATIC_INPUT_MISSING",
                    "blocking",
                    format!(
                        "wind equivalent-static exposed span requires user-entered {}; no value or marked span is defaulted",
                        span_missing.join(", ")
                    ),
                    vec![case_id.to_string()],
                ));
                span_fields_missing = true;
            }
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
        if span_fields_missing {
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
        let mut whole_span_pipe_indices = HashSet::new();
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
            if !whole_span_pipe_indices.insert(pipe_index) {
                diagnostics.push(diag(
                    &equivalent_static_diag_id(case_id, "wind:duplicate-whole-span"),
                    "EQUIVALENT_STATIC_INPUT_INVALID",
                    "blocking",
                    "wind equivalent-static whole-span pipe references must be unique",
                    vec![case_id.to_string(), pipe_ref.clone()],
                ));
                refs_blocked = true;
                continue;
            }
            let section = &model.pipe_segments[pipe_index].section;
            let insulation = section
                .insulation_thickness
                .as_ref()
                .map_or(0.0, |q| q.value);
            exposed.push(ElementExposedDiameter {
                element_index: pipe_index,
                exposed_diameter: section.outside_diameter.value + 2.0 * insulation,
                exposed_extent: None,
            });
        }
        // Partial-extent marking: same pipe-reference resolution and section
        // basis as whole-span marking; a pipe marked in both forms blocks
        // (no double exposure), invalid fractions block, and overlapping
        // extents on one pipe block. Multiple disjoint extents per pipe each
        // generate their own load.
        let mut extents_by_pipe: BTreeMap<usize, Vec<LoadExtent>> = BTreeMap::new();
        for (index, span) in wind.exposed_spans.iter().enumerate() {
            let pipe_ref = span.pipe_ref.as_ref().expect("checked above");
            let Some(&pipe_index) = pipe_map.get(pipe_ref.as_str()) else {
                diagnostics.push(diag(
                    &equivalent_static_diag_id(
                        case_id,
                        &format!("wind:exposed-span-{index}:{}", stable_suffix(pipe_ref)),
                    ),
                    "EQUIVALENT_STATIC_INPUT_INVALID",
                    "blocking",
                    "wind equivalent-static marked span references a pipe that is not present in the preview model",
                    vec![case_id.to_string(), pipe_ref.clone()],
                ));
                refs_blocked = true;
                continue;
            };
            if whole_span_pipe_indices.contains(&pipe_index) {
                diagnostics.push(diag(
                    &equivalent_static_diag_id(
                        case_id,
                        &format!("wind:exposed-span-{index}:double-exposure"),
                    ),
                    "EQUIVALENT_STATIC_INPUT_INVALID",
                    "blocking",
                    "wind equivalent-static marking names the same pipe in exposed_pipe_refs and exposed_spans; mark each pipe with exactly one form",
                    vec![case_id.to_string(), pipe_ref.clone()],
                ));
                refs_blocked = true;
                continue;
            }
            let start = span.start_fraction.as_ref().expect("checked above").value;
            let end = span.end_fraction.as_ref().expect("checked above").value;
            let extent = match LoadExtent::new(start, end) {
                Ok(extent) => extent,
                Err(error) => {
                    diagnostics.push(diag(
                        &equivalent_static_diag_id(
                            case_id,
                            &format!("wind:exposed-span-{index}:extent"),
                        ),
                        "EQUIVALENT_STATIC_INPUT_INVALID",
                        "blocking",
                        format!(
                            "wind equivalent-static exposed span fractions are invalid: {error}"
                        ),
                        vec![case_id.to_string(), pipe_ref.clone()],
                    ));
                    refs_blocked = true;
                    continue;
                }
            };
            extents_by_pipe.entry(pipe_index).or_default().push(extent);
            let section = &model.pipe_segments[pipe_index].section;
            let insulation = section
                .insulation_thickness
                .as_ref()
                .map_or(0.0, |q| q.value);
            exposed.push(ElementExposedDiameter {
                element_index: pipe_index,
                exposed_diameter: section.outside_diameter.value + 2.0 * insulation,
                exposed_extent: Some(extent),
            });
        }
        for (pipe_index, extents) in &extents_by_pipe {
            let mut sorted = extents.clone();
            sorted.sort_by(|a, b| {
                a.start_fraction
                    .partial_cmp(&b.start_fraction)
                    .expect("validated finite fractions")
            });
            if sorted
                .windows(2)
                .any(|pair| pair[1].start_fraction < pair[0].end_fraction)
            {
                let pipe_id = model.pipe_segments[*pipe_index].id.as_str();
                diagnostics.push(diag(
                    &equivalent_static_diag_id(
                        case_id,
                        &format!("wind:exposed-span-overlap:{}", stable_suffix(pipe_id)),
                    ),
                    "EQUIVALENT_STATIC_INPUT_INVALID",
                    "blocking",
                    "wind equivalent-static exposed spans overlap on one pipe; mark disjoint extents (multiple disjoint extents per pipe are allowed)",
                    vec![case_id.to_string(), pipe_id.to_string()],
                ));
                refs_blocked = true;
            }
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
/// segment, exact selection requires the named point and its E and G;
/// interpolation requires two adjacent points that strictly bracket the solve
/// temperature and carry E, G, and alpha. No base-value mixing or
/// extrapolation occurs.
fn materials_for_modulus_basis(
    model: &PreviewModel,
    materials: &[MaterialInput],
    load_case: &PreviewLoadCase,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<(Vec<MaterialInput>, String)> {
    materials_for_modulus_basis_observed(model, materials, load_case, diagnostics, None)
}
fn materials_for_modulus_basis_observed(
    model: &PreviewModel, materials: &[MaterialInput], load_case: &PreviewLoadCase,
    diagnostics: &mut Vec<Diagnostic>, mut product: Option<&mut retained_product::ProductCapture>,
) -> Option<(Vec<MaterialInput>, String)> {
    if pressure_runtime::is_exact(model) {
        return pressure_material::resolve_case(model, materials, load_case, diagnostics);
    }
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
        let mut selected_ordinals = None;
        let (elastic_modulus, shear_modulus, thermal_expansion_coefficient, provenance) =
            if let Some(basis_ref) = load_case.modulus_basis_ref.as_deref() {
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
                let Some(shear_modulus) = point.shear_modulus.clone() else {
                    diagnostics.push(diag(
                        &format!(
                            "diagnostic:modulus-basis:{}:{}:shear-modulus",
                            stable_suffix(load_case_id),
                            stable_suffix(&material.id)
                        ),
                        "MODULUS_BASIS_INPUT_MISSING",
                        "blocking",
                        format!(
                            "modulus basis {basis_ref} on material {} carries no user-entered shear modulus; selected bases never fall back to base G",
                            material.id
                        ),
                        vec![
                            load_case_id.to_string(),
                            material.id.clone(),
                            basis_ref.to_string(),
                        ],
                    ));
                    blocked = true;
                    continue;
                };
                if product.is_some() {
                    selected_ordinals = material.temperature_points.iter().position(|p| std::ptr::eq(p, point)).map(|i| (i, None));
                }
                (
                    elastic_modulus,
                    shear_modulus,
                    point.thermal_expansion_coefficient.clone(),
                    format!(
                        "material={}; selection=exact_user_entered_point; elastic_modulus_source={basis_ref}; shear_modulus_source={basis_ref}; thermal_expansion_coefficient_source={}; interpolation=not_performed",
                        material.id,
                        point
                            .thermal_expansion_coefficient
                            .as_ref()
                            .map(|_| basis_ref)
                            .unwrap_or("absent_on_selected_point")
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
                let (
                    Some(lower_e),
                    Some(upper_e),
                    Some(lower_g),
                    Some(upper_g),
                    Some(lower_alpha),
                    Some(upper_alpha),
                ) = (
                    lower.elastic_modulus.as_ref(),
                    upper.elastic_modulus.as_ref(),
                    lower.shear_modulus.as_ref(),
                    upper.shear_modulus.as_ref(),
                    lower.thermal_expansion_coefficient.as_ref(),
                    upper.thermal_expansion_coefficient.as_ref(),
                )
                else {
                    diagnostics.push(diag(
                        &format!(
                            "diagnostic:modulus-basis:{}:{}:interpolation-input",
                            stable_suffix(load_case_id),
                            stable_suffix(&material.id)
                        ),
                        "MODULUS_BASIS_INPUT_MISSING",
                        "blocking",
                        format!(
                            "material {} interpolation bracket {}..{} must carry user-entered elastic modulus, shear modulus, and thermal expansion coefficient on both source points; selected bases never fall back to base G",
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
                if [lower_g, upper_g]
                    .iter()
                    .any(|value| !value.value.is_finite() || value.value <= 0.0)
                {
                    diagnostics.push(diag(
                        &format!(
                            "diagnostic:modulus-basis:{}:{}:interpolation-shear-modulus-invalid",
                            stable_suffix(load_case_id),
                            stable_suffix(&material.id)
                        ),
                        "MODULUS_BASIS_INPUT_INVALID",
                        "blocking",
                        format!(
                            "material {} interpolation bracket {}..{} must carry finite positive user-entered shear modulus on both source points; selected bases never fall back to base G",
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
                }
                if product.is_some() {
                    let lo = material.temperature_points.iter().position(|p| std::ptr::eq(p, lower)).expect("selected lower");
                    let hi = material.temperature_points.iter().position(|p| std::ptr::eq(p, upper)).expect("selected upper");
                    selected_ordinals = Some((lo, Some(hi)));
                }
                let fraction = (solve_temperature - lower_temperature)
                    / (upper_temperature - lower_temperature);
                let interpolated_e = lower_e.value + fraction * (upper_e.value - lower_e.value);
                let interpolated_g = lower_g.value + fraction * (upper_g.value - lower_g.value);
                let interpolated_alpha =
                    lower_alpha.value + fraction * (upper_alpha.value - lower_alpha.value);
                (
                    Quantity {
                        value: interpolated_e,
                        unit: lower_e.unit.clone(),
                    },
                    Quantity {
                        value: interpolated_g,
                        unit: lower_g.unit.clone(),
                    },
                    Some(Quantity {
                        value: interpolated_alpha,
                        unit: lower_alpha.unit.clone(),
                    }),
                    format!(
                        "material={}; elastic_modulus_sources={},{}; shear_modulus_sources={},{}; thermal_expansion_coefficient_sources={},{}; method=linear_temperature_interpolation; solve_temperature_kelvin={solve_temperature}",
                        material.id,
                        lower.id,
                        upper.id,
                        lower.id,
                        upper.id,
                        lower.id,
                        upper.id
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
        if !shear_modulus.value.is_finite() || shear_modulus.value <= 0.0 {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:modulus-basis:{}:{}:shear-modulus-invalid",
                    stable_suffix(load_case_id),
                    stable_suffix(&material.id)
                ),
                "MODULUS_BASIS_INPUT_INVALID",
                "blocking",
                format!(
                    "resolved shear modulus on material {} must be finite and positive; selected bases never fall back to base G",
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
        if let Some(observer) = product.as_deref_mut() {
            observer.selection(load_case, materials, material, selected_ordinals,
                elastic_modulus.value, shear_modulus.value, thermal_expansion_coefficient.as_ref().map(|a| a.value));
        }
        provenance_records.push(provenance);
        resolved.push(MaterialInput {
            id: material.id.clone(),
            elastic_modulus,
            shear_modulus: Some(shear_modulus),
            constitutive_basis: material.constitutive_basis.clone(),
            poisson_ratio: material.poisson_ratio.clone(),
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
        if let Some(observer) = product.as_deref_mut() {
            observer.successful_basis_record(load_case, &basis_record);
        }
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
        wall_thickness: thickness,
    })
}

// Mechanical distributed loads use the same spans and frame for assembly,
// fixed-end correction and section equilibrium; thermal/pressure stay separate.
fn straight_global_uniform_load(
    load: &open_pipe_stress_primitive_loads::ElementUniformLoadContribution,
) -> Result<SpannedGlobalUniformLoad, StraightPipeError> {
    let mut intensity = [0.0; 3];
    let dof = load.direction.dof_index();
    if dof >= 3 {
        return Err(StraightPipeError::MissingInput {
            name: "translational distributed direction",
        });
    }
    intensity[dof] = load.magnitude.value;
    let span = match load.extent {
        Some(extent) => UniformLoadSpan::new(extent.start_fraction, extent.end_fraction)?,
        None => UniformLoadSpan::full(),
    };
    SpannedGlobalUniformLoad::new(intensity, span)
}

fn straight_local_uniform_loads(
    pipe: &StraightPipeElement,
    pipe_index: usize,
    loads: &[open_pipe_stress_primitive_loads::ElementUniformLoadContribution],
) -> Result<Vec<SpannedUniformLocalLoad>, StraightPipeError> {
    let orientation = pipe.frame_element()?.orientation()?;
    let mut local_loads = Vec::new();
    for load in loads.iter().filter(|load| {
        load.element_index == pipe_index
            && !matches!(
                load.magnitude.dimension,
                LoadDimension::Pressure | LoadDimension::TemperatureChange
            )
    }) {
        let global = straight_global_uniform_load(load)?;
        for (axis, direction) in [
            LocalLoadDirection::X,
            LocalLoadDirection::Y,
            LocalLoadDirection::Z,
        ]
        .into_iter()
        .enumerate()
        {
            let intensity = (0..3)
                .map(|j| orientation.local_axes[axis][j] * global.force_per_length[j])
                .sum();
            local_loads.push(SpannedUniformLocalLoad::new(
                direction,
                intensity,
                global.span,
            )?);
        }
    }
    Ok(local_loads)
}

fn straight_section_resultants(
    pipe: &StraightPipeElement,
    end_forces: &[f64],
    loads: &[SpannedUniformLocalLoad],
    fraction: f64,
) -> Result<[f64; 6], StraightPipeError> {
    let end: &[f64; ELEMENT_DOF] =
        end_forces
            .try_into()
            .map_err(|_| StraightPipeError::InvalidDisplacementLength {
                expected: ELEMENT_DOF,
                actual: end_forces.len(),
            })?;
    let section = pipe.station_resultants_from_i_end_with_spans(
        PipeEndResultants::from_local_forces(end, PipeEnd::I),
        fraction,
        loads,
        &[],
    )?;
    // The lower helper uses the i-side action. Negate all components together
    // to express the j-side cut action, matching curved station orientation.
    Ok([
        -section.axial_force,
        -section.shear_force_y,
        -section.shear_force_z,
        -section.torsional_moment,
        -section.bending_moment_y,
        -section.bending_moment_z,
    ])
}

fn exact_section_evidence(pipe_id: &str, g: pressure_exact::SourceAnnulus) -> serde_json::Value {
    serde_json::json!({"pipe_id":pipe_id,"geometry_basis":"authored_normalized_od_wall_v1",
        "outside_diameter_m":g.outside_diameter_m(),"effective_wall_thickness_m":g.effective_wall_thickness_m(),
        "ri_m":g.inner_radius_m(),"ro_m":g.outer_radius_m(),"Ai_m2":g.internal_area_m2(),"As_m2":g.wall_area_m2(),
        "I_m4":g.second_moment_m4(),"J_m4":g.polar_moment_m4(),"Z_m3":g.section_modulus_m3()})
}

fn exact_straight_summary_extrema(
    pipe: &StraightPipeElement,
    end_forces: &[f64],
    loads: &[SpannedUniformLocalLoad],
    section: &DerivedSection,
    pressure_state: Option<&pressure_runtime::ExactPressurePipeState>,
) -> Result<open_pipe_stress_stress_recovery::elastic_extrema::CertifiedStressMaximum, String> {
    use open_pipe_stress_stress_recovery::elastic_extrema::{
        bound_piecewise_elastic_maximum, QuadraticStressSpan,
    };
    let length = pipe.length().map_err(|e| e.to_string())?;
    let mut boundaries = vec![0.0, 1.0];
    for load in loads {
        boundaries.extend([load.span.start_fraction, load.span.end_fraction]);
    }
    boundaries.sort_by(f64::total_cmp);
    boundaries.dedup();
    let mut spans = Vec::new();
    for bounds in boundaries.windows(2) {
        let (a, b) = (bounds[0], bounds[1]);
        let h = (b - a) * length;
        let r =
            straight_section_resultants(pipe, end_forces, loads, a).map_err(|e| e.to_string())?;
        // E7 (S11 section 4.4): each axis intensity is one exact sum of the
        // active loads' force_per_length, rounded once.
        let mut intensity = [
            ExactAccumulator::new(),
            ExactAccumulator::new(),
            ExactAccumulator::new(),
        ];
        for load in loads
            .iter()
            .filter(|l| l.span.start_fraction <= a && l.span.end_fraction >= b)
        {
            let axis = match load.direction {
                LocalLoadDirection::X => 0,
                LocalLoadDirection::Y => 1,
                LocalLoadDirection::Z => 2,
            };
            intensity[axis]
                .add(load.force_per_length)
                .map_err(|e| format!("span intensity: {e}"))?;
        }
        let mut w = [0.0; 3];
        for (value, accumulator) in w.iter_mut().zip(&intensity) {
            *value = accumulator
                .round()
                .map_err(|e| format!("span intensity: {e}"))?;
        }
        // Direct j-side statics: N'=-wx, My'=Vz, Mz'=-Vy; My''=-wz, Mz''=wy.
        // Convert power coefficients on u in [0,1] to Bernstein controls.
        let bernstein = |c0: f64, c1: f64, c2: f64| [c0, c0 + 0.5 * c1, c0 + c1 + c2];
        // Pressure members arrive with mechanical/thermal actions. Form the
        // constant membrane stress before rounding the pressure wall force;
        // a subnormal force can still carry a normal representable stress.
        // Uniform pressure is constant on the member, so the axial slope and
        // the unchanged nonaxial section statics retain their original basis.
        let axial = if let Some(state) = pressure_state {
            state
                .annulus
                .recover_wall_effective_membrane(r[0], state.material, state.pressure)
                .map_err(|e| format!("source pressure membrane is unrepresentable: {e:?}"))?
                .2
        } else {
            r[0] / section.area
        };
        spans.push(QuadraticStressSpan {
            start: a,
            end: b,
            axial: bernstein(axial, -w[0] * h / section.area, 0.0),
            bending_y: bernstein(
                r[4] / section.section_modulus,
                r[2] * h / section.section_modulus,
                -0.5 * w[2] * h * h / section.section_modulus,
            ),
            bending_z: bernstein(
                r[5] / section.section_modulus,
                -r[1] * h / section.section_modulus,
                0.5 * w[1] * h * h / section.section_modulus,
            ),
        });
    }
    bound_piecewise_elastic_maximum(&spans).map_err(|e| format!("{e:?}"))
}

fn is_exact_pressure_result_kind(kind: &str) -> bool {
    matches!(
        kind,
        "pipe_wall_endpoint_action_v2"
            | "pipe_wall_axial_force_v2"
            | "pipe_effective_axial_force_v2"
            | "pipe_axial_membrane_stress_v2"
            | "pipe_lame_radial_stress_v2"
            | "pipe_lame_hoop_stress_v2"
    )
}

fn straight_summary_extrema(
    pipe: &StraightPipeElement,
    end_forces: &[f64],
    loads: &[SpannedUniformLocalLoad],
    section: &DerivedSection,
) -> Result<f64, StraightPipeError> {
    let mut boundaries = vec![0.0, 1.0];
    for load in loads {
        boundaries.extend([load.span.start_fraction, load.span.end_fraction]);
    }
    boundaries.sort_by(f64::total_cmp);
    boundaries.dedup();
    let components = |fraction| -> Result<[f64; 3], StraightPipeError> {
        let r = straight_section_resultants(pipe, end_forces, loads, fraction)?;
        let stress = recover_section_stress(&r, section);
        let c = stress.components;
        let values = [
            // H-1 (U3): the retired legacy pressure term was always +0.0 here;
            // the explicit + 0.0 keeps a zero axial stress's published sign.
            c.axial_normal.unwrap_or(0.0) + 0.0,
            c.bending_normal_y.unwrap_or(0.0),
            c.bending_normal_z.unwrap_or(0.0),
        ];
        for value in values {
            if !value.is_finite() {
                return Err(StraightPipeError::NonFiniteInput {
                    name: "straight stress extrema",
                    value,
                });
            }
        }
        if !stress.findings.is_empty() {
            return Err(StraightPipeError::MissingInput {
                name: "straight stress extrema inputs",
            });
        }
        Ok(values)
    };
    let mut maximum: f64 = 0.0;
    for interval in boundaries.windows(2) {
        let (a, b) = (interval[0], interval[1]);
        let start = components(a)?;
        let middle = components(a + (b - a) / 2.0)?;
        let end = components(b)?;
        // Each stress component is quadratic on this uniform-load interval.
        // Interpolate in normalized interval coordinate t to avoid division by
        // a short physical interval. |A|+|B|+|C| is max of eight signed sums.
        for mask in 0..8 {
            let signs: [f64; 3] =
                std::array::from_fn(|i| if mask & (1 << i) == 0 { -1.0 } else { 1.0 });
            let sum = |values: [f64; 3]| (0..3).map(|i| signs[i] * values[i]).sum::<f64>();
            let (v0, vm, v1) = (sum(start), sum(middle), sum(end));
            let quadratic = 2.0 * (v1 + v0 - 2.0 * vm);
            let linear = v1 - v0 - quadratic;
            for value in [v0, vm, v1, quadratic, linear] {
                if !value.is_finite() {
                    return Err(StraightPipeError::NonFiniteInput {
                        name: "straight stress polynomial",
                        value,
                    });
                }
            }
            maximum = maximum.max(v0).max(v1);
            if quadratic != 0.0 {
                let t = -0.5 * (linear / quadratic);
                if !t.is_finite() {
                    return Err(StraightPipeError::NonFiniteInput {
                        name: "straight stress stationary root",
                        value: t,
                    });
                }
                if t > 0.0 && t < 1.0 {
                    maximum = maximum.max(sum(components(a + (b - a) * t)?));
                }
            }
        }
    }
    Ok(maximum / 1_000_000.0)
}

fn add_uniform_element_loads(
    ledger: &mut LoadLedger,
    _model: &PreviewModel,
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
        // S11-G section 3.2: an equivalent-static generated intensity carries
        // gamma_4 of operand formation (a same-sign product chain).
        let generated = load
            .load_id
            .starts_with(&format!("{load_case_id}:generated:"));
        // Curved-bend macro spans consume arc-consistent equivalent nodal
        // loads: fixed-end forces and moments from exact closed-form
        // integration of the uniform intensity along the arc, consistent with
        // the assembled macro-element stiffness. Element uniform loads are
        // validated translational upstream, so the intensity is a global
        // force per unit arc length along one global axis.
        if let Some(bend) = curved_bends_by_pipe.get(&load.element_index) {
            // Partial extents fail closed on macro-realized arcs: the
            // arc-consistent machinery integrates a full uniform intensity
            // only, and no partial-arc treatment is invented here. Never a
            // silent chord approximation or a drop.
            if load.extent.is_some() {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:curved-bend:{}:{}:partial-extent-load",
                        stable_suffix(load_case_id),
                        stable_suffix(&load.load_id)
                    ),
                    "LOAD_INPUT_INVALID",
                    "blocking",
                    format!(
                        "partial-extent uniform load {} targets curved-bend macro-realized span {}; partial extents are not supported on realized arcs — mark the whole span or model separate pipes",
                        load.load_id, bend.pipe_id
                    ),
                    vec![
                        load.load_id.clone(),
                        bend.pipe_id.clone(),
                        bend.component_id.clone(),
                        load_case_id.to_string(),
                    ],
                ));
                continue;
            }
            let dof = load.direction.dof_index();
            let mut intensity = [0.0; 3];
            let equivalent = if dof < 3 {
                intensity[dof] = load.magnitude.value;
                bend.macro_element
                    .consistent_uniform_nodal_loads(intensity)
                    .map_err(|error| error.to_string())
            } else {
                Err("uniform element loads on curved-bend macro spans require a translational direction".to_string())
            };
            match equivalent {
                Ok(equivalent) => {
                    // S11 section 4.2: one term per (load, DOF) from this load's
                    // own consistent equivalent.
                    // T4-U1b (R-1): certified arc terms; any failed precondition
                    // keeps CannotBound (S11-G SF-2), which demotes the case.
                    let dof_map = element_dof_map(bend.node_i, bend.node_j);
                    match certify_curved_uniform_load(&curved_formation_of(bend), intensity) {
                        Ok(certified) => {
                            for (local_dof, &global_dof) in dof_map.iter().enumerate() {
                                let term = &certified.terms[local_dof];
                                ledger.push_formed(
                                    &load.load_id,
                                    global_dof,
                                    equivalent[local_dof],
                                    Formation::Exact {
                                        scale: 1.0,
                                        scaled_intended: term.intended.clone(),
                                    },
                                    term.operand_bound(generated),
                                    false,
                                );
                            }
                        }
                        Err(_) => {
                            for (local_dof, &global_dof) in dof_map.iter().enumerate() {
                                ledger.push_formed(
                                    &load.load_id,
                                    global_dof,
                                    equivalent[local_dof],
                                    Formation::CannotBound,
                                    0.0,
                                    false,
                                );
                            }
                        }
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
        let pipe = &pipes[load.element_index];
        let equivalent = straight_global_uniform_load(load)
            .and_then(|global| pipe.equivalent_global_nodal_loads_with_spans_formed(&[global]));
        match equivalent {
            Ok((equivalent, formations)) => {
                // One load per call: the SP formula of one load is formation
                // (E1 for one load is bit-identical); one term per (load, DOF),
                // each with its exact intended formula (S11-G).
                for ((slot, global), formation) in
                    element_dof_map(pipe.node_i.index, pipe.node_j.index)
                        .iter()
                        .enumerate()
                        .zip(formations)
                {
                    let operand_bound = if generated {
                        product_upward(gamma(4), equivalent[slot].abs())
                    } else {
                        0.0
                    };
                    ledger.push_formed(
                        &load.load_id,
                        *global,
                        equivalent[slot],
                        formation,
                        operand_bound,
                        false,
                    );
                }
            }
            Err(error) => diagnostics.push(diag(
                &format!(
                    "diagnostic:load:{}:consistent",
                    stable_suffix(&load.load_id)
                ),
                "LOAD_INPUT_INVALID",
                "blocking",
                error.to_string(),
                vec![load.load_id.clone(), load_case_id.to_string()],
            )),
        }
    }
}

/// T4-U1b: the held operands of a realized arc for its load certificate
/// (`certify_curved_uniform_load`), copied from the validated macro-element
/// as SA's formation source copies them (no force scaling).
fn curved_formation_of(bend: &CurvedBendMacroBuild) -> CurvedFormation {
    let m = &bend.macro_element;
    CurvedFormation {
        node_i: m.node_i.index,
        node_j: m.node_j.index,
        coordinates_i: m.node_i.coordinates,
        coordinates_j: m.node_j.coordinates,
        radius: m.radius,
        y_reference: m.y_reference,
        elastic_modulus: m.elastic_modulus,
        shear_modulus: m.shear_modulus,
        area: m.area,
        second_moment: m.second_moment,
        torsion_constant: m.torsion_constant,
        in_plane_flexibility_factor: m.in_plane_flexibility_factor,
        out_of_plane_flexibility_factor: m.out_of_plane_flexibility_factor,
    }
}

fn build_thermal_element_loads(
    model: &PreviewModel,
    load_case: &PreviewLoadCase,
    material_map: &HashMap<&str, &MaterialInput>,
    pipe_map: &HashMap<&str, usize>,
    sections: &HashMap<String, DerivedSection>,
    diagnostics: &mut Vec<Diagnostic>,
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
            diagnostics.push(diag(
                &format!("diagnostic:thermal:{}:{}:selected-alpha", stable_suffix(&load_case.id), stable_suffix(&load.id)),
                "THERMAL_EXPANSION_INPUT_MISSING", "blocking",
                "thermal load requires an expansion coefficient in the selected effective material basis",
                vec![load_case.id.clone(), load.id.clone(), material.id.clone()],
            ));
            continue;
        };
        let Some(section) = sections.get(&pipe_input.id) else {
            continue;
        };
        let epsilon_th = alpha * load.magnitude.value;
        loads.push(ThermalElementLoad {
            element_index,
            source: load.id.clone(),
            axial_load: material.elastic_modulus.value * section.area * epsilon_th,
            thermal_strain: epsilon_th,
        });
    }
    loads
}

fn add_thermal_equivalent_loads(
    ledger: &mut LoadLedger,
    thermal_loads: &[ThermalElementLoad],
    pipes: &[StraightPipeElement],
    curved_bends_by_pipe: &HashMap<usize, &CurvedBendMacroBuild>,
) {
    for load in thermal_loads {
        if let Some(bend) = curved_bends_by_pipe.get(&load.element_index) {
            add_curved_bend_thermal_equivalent_load(
                ledger,
                &load.source,
                bend,
                load.thermal_strain,
            );
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
        // S11 section 4.2: fl(P * x_a) per axis, one term at each end (the
        // same terms `source_recovery` pushes for T1's eigen loads); S11-G:
        // each is an exact rounded product of a self-equilibrated pair.
        for axis in 0..3 {
            let (a, b) = (load.axial_load, local_x[axis]);
            let value = a * b;
            let product = |k| Formation::RoundedProduct { k, a, b };
            ledger.push_formed(
                &load.source,
                i_base + axis,
                -value,
                product(-1.0),
                0.0,
                true,
            );
            ledger.push_formed(&load.source, j_base + axis, value, product(1.0), 0.0, true);
        }
    }
}

// Exact free-expansion identity for the arc span: uniform thermal expansion
// is stress-free, so the equivalent nodal load is K_macro * u_free with
// u_free the pure translation field alpha*deltaT*(p - p_i) (zero rotations)
// evaluated at the two end nodes. No approximation is introduced for the arc.
// S11 section 4.2 (V1's S11-V3 example): the equivalent is pushed as the exact
// products K_rc * fl(eps * chord_c), one per nonzero column, never a pre-summed
// row value, so it is the exact K * u_free of the represented u_free.
fn add_curved_bend_thermal_equivalent_load(
    ledger: &mut LoadLedger,
    source: &str,
    bend: &CurvedBendMacroBuild,
    thermal_strain: f64,
) {
    let free_expansion = curved_bend_free_expansion_displacements(bend, thermal_strain);
    let dof_map = element_dof_map(bend.node_i, bend.node_j);
    for (local_row, &global_row) in dof_map.iter().enumerate() {
        for (local_col, &free_value) in free_expansion.iter().enumerate() {
            if free_value != 0.0 {
                // S11-G: K_rc * fl(eps * chord_c), an exact scaled rounded
                // product (self-equilibrated: K * u_free of a rigid-free field).
                let k = bend.global_stiffness[local_row][local_col];
                ledger.push_formed_product(
                    source,
                    global_row,
                    k,
                    free_value,
                    Formation::RoundedProduct {
                        k,
                        a: thermal_strain,
                        b: bend.chord[local_col - DOF_PER_NODE],
                    },
                    0.0,
                    true,
                );
            }
        }
    }
}

// T4-U2 (H-2): a realized arc's member-owned pressure term K_b·u_free(ε_p)
// (its −c_b, the caps and the remainders are exact-pressure source groups).
// The same exact products as the thermal identity above, K_rc·fl(ε_p·d_c)
// per nonzero column, self-equilibrated; ε_p is the represented
// `arc_pressure_strain`, whose own formation error is each term's S11-G
// operand bound (`bend_pressure::strain_operand_bound`).
fn add_curved_bend_pressure_equivalent_load(
    ledger: &mut LoadLedger,
    exact: &pressure_runtime::ExactPressureCase,
    curved_bends_by_pipe: &HashMap<usize, &CurvedBendMacroBuild>,
) {
    for (pipe_index, state) in &exact.pipe_states {
        let (Some(arc), Some(bend)) = (state.arc, curved_bends_by_pipe.get(pipe_index)) else {
            continue;
        };
        let free_expansion = curved_bend_free_expansion_displacements(bend, arc.strain);
        let dof_map = element_dof_map(bend.node_i, bend.node_j);
        for (local_row, &global_row) in dof_map.iter().enumerate() {
            for (local_col, &free_value) in free_expansion.iter().enumerate() {
                if free_value != 0.0 {
                    let k = bend.global_stiffness[local_row][local_col];
                    ledger.push_formed_product(
                        &state.region_id,
                        global_row,
                        k,
                        free_value,
                        Formation::RoundedProduct {
                            k,
                            a: arc.strain,
                            b: bend.chord[local_col - DOF_PER_NODE],
                        },
                        bend_pressure::strain_operand_bound(k, free_value),
                        true,
                    );
                }
            }
        }
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

/// E5 (S11 section 4.4): each straight end force is one exact sum of the
/// formed elastic term `local_i`, minus every load's own fixed-end term (SP's
/// per-load E1 terms), plus each thermal `axial_load`
/// on the end UX rows (+ at i, - at j), rounded once. This replaces the two
/// roundings of `mechanical = local - equivalent` and the summed axial
/// correction. A non-finite or out-of-range sum keeps a non-finite value,
/// which `require_finite_mechanics` refuses as before.
fn exact_straight_end_forces(
    local_forces: &[f64],
    equivalent_terms: &[[f64; ELEMENT_DOF]],
    element_index: usize,
    thermal_loads: &[ThermalElementLoad],
) -> Vec<f64> {
    let axial_loads = thermal_loads
        .iter()
        .filter(|load| load.element_index == element_index)
        .map(|load| load.axial_load)
        .collect::<Vec<_>>();
    (0..local_forces.len())
        .map(|slot| {
            let mut accumulator = ExactAccumulator::new();
            let mut summed = accumulator.add(local_forces[slot]);
            for term in equivalent_terms {
                summed = summed.and_then(|()| accumulator.add(-term[slot]));
            }
            let sign = if slot == UX {
                1.0
            } else if slot == DOF_PER_NODE + UX {
                -1.0
            } else {
                0.0
            };
            if sign != 0.0 {
                for &axial in &axial_loads {
                    summed = summed.and_then(|()| accumulator.add(sign * axial));
                }
            }
            summed
                .and_then(|()| accumulator.round())
                .unwrap_or(f64::NAN)
        })
        .collect()
}

// Macro-span recovery: end forces are K_macro * (d - u_free) minus the
// arc-consistent distributed equivalent loads, in global coordinates — the
// exact free-expansion correction mirrors
// `corrected_local_forces_for_axial_effects` so recovered forces exclude the
// self-equilibrated thermal part, and the equivalent-load subtractions turn
// the nodal solve response into the true node-on-element end forces of the
// continuously loaded arc — then rotated to the chord frame of the replaced
// straight span so the existing result rows keep their convention. No
// pressure load reaches a macro span: the radial pressure treatment was
// retired, legacy pressure is refused on every route, and the exact pressure
// profile refuses bend components.
fn recover_curved_bend_local_forces(
    bend: &CurvedBendMacroBuild,
    pipe: &StraightPipeElement,
    displacements: &[f64],
    thermal_loads: &[ThermalElementLoad],
    uniform_intensities: &[[f64; 3]],
    pressure_strain: Option<f64>,
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
    // E8/E9 (S11 section 4.4): each global end force is one exact sum of the
    // products K_rc * d_c, minus K_rc * fl(eps_l * chord_c) for each thermal
    // load l (the force side's exact products), minus each uniform load's own
    // consistent equivalent, rounded once. The chord rotation below stays a
    // formed transform.
    // T4-U2 (H-2): an arc in a pressure region also subtracts the free
    // expansion of its represented ε_p, the value its load used.
    let free_expansions = thermal_loads
        .iter()
        .filter(|load| load.element_index == bend.pipe_index)
        .map(|load| load.thermal_strain)
        .chain(pressure_strain)
        .map(|strain| curved_bend_free_expansion_displacements(bend, strain))
        .collect::<Vec<_>>();
    let mut equivalents = Vec::new();
    for &intensity in uniform_intensities {
        if intensity != [0.0; 3] {
            equivalents.push(
                bend.macro_element
                    .consistent_uniform_nodal_loads(intensity)
                    .map_err(|error| error.to_string())?,
            );
        }
    }
    let mut global_forces = [0.0; ELEMENT_DOF];
    for (row, force) in global_forces.iter_mut().enumerate() {
        let mut accumulator = ExactAccumulator::new();
        let mut summed = Ok(());
        for (col, &displacement) in element_displacements.iter().enumerate() {
            summed = summed.and_then(|()| {
                accumulator.add_product(bend.global_stiffness[row][col], displacement)
            });
        }
        for free_expansion in &free_expansions {
            for (col, &free_value) in free_expansion.iter().enumerate() {
                if free_value != 0.0 {
                    summed = summed.and_then(|()| {
                        accumulator.add_product(-bend.global_stiffness[row][col], free_value)
                    });
                }
            }
        }
        for equivalent in &equivalents {
            summed = summed.and_then(|()| accumulator.add(-equivalent[row]));
        }
        *force = summed
            .and_then(|()| accumulator.round())
            .unwrap_or(f64::NAN);
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
    Ok(local_forces)
}

const SECTION_RESULTANT_BASIS: &str = "recovered_from_local_element_stiffness";
const STRAIGHT_ENDPOINT_SECTION_SIGN_CONVENTION: &str = "positive value follows the j-side section action in the element-local frame (local x toward end j); resultants come from section equilibrium over assembled end actions";
const CURVED_BEND_SECTION_SIGN_CONVENTION: &str = "local x is endpoint arc tangent toward j; local z is bend-plane normal; local y is z cross x toward arc center; resultants come from section equilibrium over assembled end actions";

// Global uniform intensities (force per unit arc length) per realized
// curved-bend span, one entry per load (E10 removed, S11 section 4.4): the
// force side forms one consistent equivalent per load, and recovery uses the
// same per-load intensities. Pressure and temperature-change dimensioned loads
// follow their own dedicated paths.
fn curved_bend_uniform_intensities_by_pipe(
    loads: &[open_pipe_stress_primitive_loads::ElementUniformLoadContribution],
    curved_bends_by_pipe: &HashMap<usize, &CurvedBendMacroBuild>,
) -> HashMap<usize, Vec<[f64; 3]>> {
    let mut intensities_by_pipe: HashMap<usize, Vec<[f64; 3]>> = HashMap::new();
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
        // Extent-bearing loads never enter bend intensity maps; the
        // application seam blocks them on macro-realized spans.
        if load.extent.is_some() {
            continue;
        }
        let dof = load.direction.dof_index();
        if dof >= 3 {
            continue;
        }
        let mut intensity = [0.0; 3];
        intensity[dof] = load.magnitude.value;
        intensities_by_pipe
            .entry(load.element_index)
            .or_default()
            .push(intensity);
    }
    intensities_by_pipe
}

// Arc sections from the assembled macro-element: rotate the
// recovered chord-frame end-j force back to global and evaluate section
// resultants along the arc by segment equilibrium (closed form in the
// curved-bend crate). No pressure load reaches a macro span (see
// `recover_curved_bend_local_forces`).
// The recovered end forces already exclude the self-equilibrated thermal
// free-expansion part and the distributed equivalent loads; the station
// grid mirrors the straight-span fractions.
fn curved_bend_section_resultants(
    bend: &CurvedBendMacroBuild,
    pipe: &StraightPipeElement,
    corrected_local_forces: &[f64],
    uniform_intensities: &[[f64; 3]],
    fraction: f64,
) -> Result<[f64; 6], String> {
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
    // E11 (S11 section 4.4): by linearity the section value is the exact sum
    // of the section function applied to the end-j force alone, to each
    // load's intensity alone, rounded once.
    bend.macro_element
        .arc_section_resultant_terms(
            fraction,
            node_j_force,
            uniform_intensities,
        )
        .map_err(|error| error.to_string())
}

fn curved_bend_station_resultants(
    bend: &CurvedBendMacroBuild,
    pipe: &StraightPipeElement,
    corrected_local_forces: &[f64],
    uniform_intensities: &[[f64; 3]],
) -> Result<[StationResultants; 3], String> {
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
        station.resultants = curved_bend_section_resultants(
            bend,
            pipe,
            corrected_local_forces,
            uniform_intensities,
            fraction,
        )?;
    }
    Ok(stations)
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
            value: displacements[base + dof] * scale,
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
        value: value,
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
        value: value,
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

// New pressure rows preserve wall force, effective force and material stress as distinct quantities.
fn append_signed_support_results(
    results: &mut Vec<ResultItem>,
    case: &PreviewLoadCase,
    support: &PreviewSupport,
    action: [f64; 6],
) {
    let mut append = |component: &str, kind: &str, value: f64, unit: &str| {
        results.push(ResultItem {
            id:format!("result:support-action:{}:{}:{}:{}:{}",case.id.len(),case.id,support.id.len(),support.id,component),
            kind:kind.to_string(), value, unit:unit.to_string(), entity_ref:support.id.clone(),
            basis_ref:Some(ResultBasisRef {ref_type:"load_case".to_string(),ref_id:case.id.clone()}),source_result_refs:Vec::new(),
            metadata:Some(ResultMetadata {component:component.to_string(),coordinate_system:"global".to_string(),location:"node".to_string(),
                basis:"recovered_from_assembled_support_law".to_string(),sign_convention:"support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate".to_string()}),
        });
    };
    for (slot, component) in ["Fx", "Fy", "Fz", "Mx", "My", "Mz"].into_iter().enumerate() {
        append(
            component,
            "support_reaction_component_v2",
            action[slot],
            if slot < 3 { "N" } else { "N*m" },
        );
    }
    append(
        "force_magnitude",
        "support_reaction_force_magnitude_v2",
        norm3(action[0], action[1], action[2]),
        "N",
    );
    append(
        "moment_magnitude",
        "support_reaction_moment_magnitude_v2",
        norm3(action[3], action[4], action[5]),
        "N*m",
    );
}

#[allow(clippy::too_many_arguments)]
fn append_exact_pressure_results(
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
    case: &PreviewLoadCase,
    pipe_id: &str,
    state: &pressure_runtime::ExactPressurePipeState,
    actions: &[f64],
    mechanical_actions: Option<&[f64]>,
    pipe: &StraightPipeElement,
    loads: &[SpannedUniformLocalLoad],
) {
    // T4-U0 (A3): a region member without straight mechanical/thermal
    // recovery (a realized curved bend) is refused by name, never recovered
    // on its chord.
    let Some(mechanical_actions) = pressure_runtime::exact_member_recovery(
        diagnostics,
        &case.id,
        pipe_id,
        state,
        mechanical_actions,
    ) else {
        return;
    };
    let (Ok([inner, outer]), Ok(_caps)) = (
        state.annulus.surface_stresses(state.pressure),
        state.annulus.cap_pair(state.pressure),
    ) else {
        diagnostics.push(diag(
            "diagnostic:exact-pressure:surface",
            "EXACT_PRESSURE_RECOVERY_FAILED",
            "blocking",
            "pressure surface stress or cap load is not representable",
            vec![case.id.clone(), pipe_id.to_string()],
        ));
        return;
    };
    results.retain(|r| {
        r.entity_ref != pipe_id
            || !matches!(
                r.kind.as_str(),
                "element_local_axial_force"
                    | "element_local_axial_normal_stress"
                    | "pipe_section_pressure_hoop_stress"
                    | "pipe_section_pressure_longitudinal_stress"
            )
    });
    let mut append =
        |kind: &str, component: &str, location: &str, value: f64, stress: bool, sign: &str| {
            results.push(ResultItem {
                id: format!(
                    "result:pressure-exact:{}:{}:{}:{}:{}:{}",
                    case.id.len(),
                    case.id,
                    pipe_id.len(),
                    pipe_id,
                    location,
                    component
                ),
                kind: kind.to_string(),
                value,
                unit: if stress { "Pa" } else { "N" }.to_string(),
                entity_ref: pipe_id.to_string(),
                basis_ref: Some(ResultBasisRef {
                    ref_type: "load_case".to_string(),
                    ref_id: case.id.clone(),
                }),
                source_result_refs: Vec::new(),
                metadata: Some(ResultMetadata {
                    component: component.to_string(),
                    coordinate_system: if stress {
                        "pipe_section"
                    } else {
                        "element_local"
                    }
                    .to_string(),
                    location: location.to_string(),
                    basis: if stress {
                        "recovered_from_open_mechanics_stress_components"
                    } else {
                        "recovered_from_local_element_stiffness"
                    }
                    .to_string(),
                    sign_convention: sign.to_string(),
                }),
            });
        };
    for (location, value) in [
        ("end_i", actions[UX]),
        ("end_j", actions[DOF_PER_NODE + UX]),
    ] {
        append("pipe_wall_endpoint_action_v2", "wall_axial_end_action", location, value, false, "node-on-element wall action, positive along authored local x toward end j; cap transfer is not subtracted from wall recovery");
    }
    for (location, fraction) in [
        ("end_i", 0.0),
        ("end_j", 1.0),
        ("quarter_1", 0.25),
        ("midspan", 0.5),
        ("quarter_3", 0.75),
    ] {
        let mechanical =
            match straight_section_resultants(pipe, mechanical_actions, loads, fraction) {
                Ok(value) => value,
                Err(error) => {
                    diagnostics.push(diag(
                        "diagnostic:exact-pressure:section",
                        "EXACT_PRESSURE_RECOVERY_FAILED",
                        "blocking",
                        error.to_string(),
                        vec![case.id.clone(), pipe_id.to_string()],
                    ));
                    return;
                }
            };
        let (wall, effective, membrane) = match state.annulus.recover_wall_effective_membrane(
            mechanical[0],
            state.material,
            state.pressure,
        ) {
            Ok(value) => value,
            Err(error) => {
                diagnostics.push(diag(
                    "diagnostic:exact-pressure:section",
                    "EXACT_PRESSURE_RECOVERY_FAILED",
                    "blocking",
                    format!("source pressure section recovery is unrepresentable: {error:?}"),
                    vec![case.id.clone(), pipe_id.to_string()],
                ));
                return;
            }
        };
        append(
            "pipe_wall_axial_force_v2",
            "wall_axial_force",
            location,
            wall,
            false,
            "tension-positive material wall section resultant Nw",
        );
        append(
            "pipe_effective_axial_force_v2",
            "effective_axial_force",
            location,
            effective,
            false,
            "effective wall-fluid resultant S=Nw-pAi; not material stress or a support reaction",
        );
        append("pipe_axial_membrane_stress_v2", "axial_membrane_stress", location, membrane, true, "tension-positive axial wall membrane stress Nw/As; no added longitudinal pressure scalar");
        for (component, stress) in [
            ("lame_inner_radial_stress", inner.radial_pa()),
            ("lame_outer_radial_stress", outer.radial_pa()),
        ] {
            append("pipe_lame_radial_stress_v2",component,location,stress,true,"tension-positive radial stress at named surface, inner traction -p and zero external pressure increment");
        }
        for (component, stress) in [
            ("lame_inner_hoop_stress", inner.hoop_pa()),
            ("lame_outer_hoop_stress", outer.hoop_pa()),
        ] {
            append("pipe_lame_hoop_stress_v2",component,location,stress,true,"tension-positive circumferential stress at named surface for long straight annulus, zero external pressure increment");
        }
    }
}

fn recover_section_stress(
    resultants: &[f64; 6],
    section: &DerivedSection,
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
        statuses: vec![AnalysisStatus::MechanicsSolved],
    })
}

fn open_formula_summary_mpa(
    stress: &open_pipe_stress_stress_recovery::StressRecoveryResult,
) -> Option<f64> {
    if !stress.findings.is_empty() {
        return None;
    }
    let components = &stress.components;
    let axial = components.axial_normal.unwrap_or(0.0);
    let bending_y = components.bending_normal_y.unwrap_or(0.0).abs();
    let bending_z = components.bending_normal_z.unwrap_or(0.0).abs();
    // H-1 (U3): the retired legacy pressure term was always +0.0 here; the
    // explicit + 0.0 keeps the arithmetic, and so the published bytes, as before.
    let base_normal = axial + 0.0;
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
    load_case_id: &str,
    pipe_id: &str,
    end_i_stress: &open_pipe_stress_stress_recovery::StressRecoveryResult,
    end_j_stress: &open_pipe_stress_stress_recovery::StressRecoveryResult,
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
        let Some(base_value_mpa) = open_formula_summary_mpa(stress) else {
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
                load_case_id,
                model
                    .load_cases
                    .first()
                    .is_some_and(|case| case.id == load_case_id),
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
    load_case_id: &str,
    is_default_case: bool,
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
    let value = base_value_mpa * multiplier;
    let mut basis = format!(
        "component_family={};component_side={};user_entered_sif={};user_entered_flexibility={};source={};solver_consumption={}",
        modifier.family,
        modifier.side,
        scalar_string(modifier.sif),
        scalar_string(modifier.flexibility),
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
            scalar_string(modifier.sif),
            pipe_id,
            scalar_string(modifier.flexibility),
            modifier.solver_consumption
        )
    } else {
        format!(
            "{} component {} applies user-entered {} SIF {} and flexibility factor {} to {} {location} stress-recovery review; solver_consumption remains {}; no protected or default component factor is supplied",
            modifier.family,
            component.id,
            modifier.side,
            scalar_string(modifier.sif),
            scalar_string(modifier.flexibility),
            pipe_id,
            modifier.solver_consumption
        )
    };
    diagnostics.push(diag(
        &format!(
            "diagnostic:component-stress-multiplier:{}",
            exact_source_identity(&[load_case_id, &component.id, pipe_id, location])
        ),
        "COMPONENT_STRESS_MULTIPLIER_APPLIED",
        "info",
        message,
        vec![
            component.id.clone(),
            pipe_id.to_string(),
            if is_default_case {
                result_id
            } else {
                qualified_load_case_result_id(load_case_id, &result_id)
            },
            modifier.source_reference.to_string(),
            load_case_id.to_string(),
        ],
    ));
}

// DEC-070 review rows: state that the user-entered bend flexibility factor is
// consumed by the assembled curved-bend macro-element (EJ precedent wording
// style), and record the arc-geometry conventions and load/recovery decisions.
fn append_curved_bend_macro_element_results(
    curved_bend_elements: &[CurvedBendMacroBuild],
    pressure_v3: bool,
    results: &mut Vec<ResultItem>,
) -> usize {
    // T4-U2: under v3 a realized bend carries its member-owned pressure term
    // (H-2); every other route keeps its text byte for byte.
    let pressure_treatment = if pressure_v3 {
        "exact_pressure_v3_member_owned_bend_term_k_u_free_eps_p_minus_end_caps"
    } else {
        "none_pressure_refused_outside_the_exact_straight_contract"
    };
    let mut appended = 0;
    for element in curved_bend_elements {
        let component_suffix = stable_suffix(&element.component_id);
        results.push(ResultItem {
            id: format!("result:component-stiffness:{component_suffix}:curved-bend-flexibility"),
            kind: "curved_bend_macro_element_review".to_string(),
            value: element.flexibility_factor,
            unit: "unitless".to_string(),
            entity_ref: element.component_id.clone(),
            basis_ref: None,
            source_result_refs: Vec::new(),
            metadata: Some(ResultMetadata {
                component: "curved_bend_flexibility".to_string(),
                coordinate_system: "component_local_preview".to_string(),
                location: element.pipe_id.clone(),
                basis: format!(
                    "component_family=bend;user_entered_flexibility={};flexibility_axis_mapping=single_user_factor_applied_to_in_plane_and_out_of_plane_bending;bend_radius_m={};arc_included_angle_rad={};arc_length_m={};arc_plane=chord_and_pipe_y_reference;arc_side=bows_toward_positive_pipe_y_reference;source={};solver_consumption={};macro_element_solve=assembled_curved_bend_stiffness;thermal_load_treatment=exact_free_expansion_identity;distributed_load_treatment=arc_consistent_fixed_end_integration;pressure_thrust_treatment={};recovery=end_forces_from_assembled_stiffness_in_chord_frame;interior_stations=arc_section_equilibrium_stations",
                    scalar_string(element.flexibility_factor),
                    scalar_string(element.bend_radius),
                    scalar_string(element.included_angle),
                    scalar_string(element.arc_length),
                    element.source_reference,
                    DEC_070_CURVED_BEND_SOLVER_CONSUMPTION,
                    pressure_treatment
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
                    "positive value is user-entered constant-effort support input evidence; a support meeting the DEC-049 consumption conditions (exactly one declared translational restraint DOF and a finite positive constant load) is consumed by the assembled solve as a constant nodal force along the positive axis of that DOF, recorded per load case in constant_effort_support_applied_load rows; a support not meeting those conditions stays review-only with a non-blocking warning; no nonlinear behavior and no catalog/default value is claimed by this preview row",
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
        value: quantity.value,
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

/// Verbatim direction convention for the DEC-049 constant-effort
/// assembled-solve consumption. Recorded identically in the applied-load
/// result rows and in the hand-calc witness
/// `validation/hand_calcs/mechanics/constant_effort_support_applied_load.md`.
const CONSTANT_EFFORT_APPLIED_SIGN_CONVENTION: &str = "positive value is the user-entered constant support force applied along the positive axis of the single declared translational restraint DOF in every solved load case; the ideal constant-effort element contributes zero stiffness and adds no solve restraint row; no gravity coupling, catalog/default value, or inferred direction is supplied";

/// One consuming constant-effort support resolved against the preview model
/// (DEC-049 assembled-solve consumption; ideal constant-effort element).
#[derive(Debug, Clone)]
struct ConstantEffortApplication {
    node_index: usize,
    dof: FrameDof,
    force_newtons: f64,
}

/// Why a constant-effort support stays review-only (data-driven opt-in:
/// consumption requires exactly one declared translational restraint DOF and
/// a positive user-entered `hanger.constant_load`; nothing is defaulted or
/// inferred, and no previously-accepted input shape becomes blocking).
#[derive(Debug, Clone, PartialEq)]
enum ConstantEffortNonConsumption {
    MissingConstantLoad,
    NonPositiveConstantLoad,
    UnparseableRestraintDof(String),
    NoTranslationalRestraintDof,
    MultipleTranslationalRestraintDofs(usize),
    UnknownNode,
}

impl ConstantEffortNonConsumption {
    fn unmet_condition(&self) -> String {
        match self {
            Self::MissingConstantLoad => {
                "no user-entered hanger.constant_load is present".to_string()
            }
            Self::NonPositiveConstantLoad => {
                "the user-entered hanger.constant_load is not a finite positive force".to_string()
            }
            Self::UnparseableRestraintDof(raw) => format!(
                "declared restraint DOF {raw} is not a recognized frame DOF, so the single acting translational DOF cannot be determined"
            ),
            Self::NoTranslationalRestraintDof => {
                "no translational restraint DOF is declared, so no acting direction is user-entered"
                    .to_string()
            }
            Self::MultipleTranslationalRestraintDofs(count) => format!(
                "{count} translational restraint DOFs are declared where exactly one acting DOF is required"
            ),
            Self::UnknownNode => "the support node is not present in the preview model".to_string(),
        }
    }
}

/// Classify one constant-effort support (caller guarantees
/// `is_constant_effort_support` and `nonlinear.is_none()`) against the
/// DEC-049 consumption conditions. No default, catalog value, or direction
/// inference: every ambiguous shape stays review-only.
fn classify_constant_effort_consumption(
    model: &PreviewModel,
    support: &PreviewSupport,
) -> Result<ConstantEffortApplication, ConstantEffortNonConsumption> {
    let constant_load = support
        .hanger
        .as_ref()
        .and_then(|hanger| hanger.constant_load.as_ref())
        .ok_or(ConstantEffortNonConsumption::MissingConstantLoad)?;
    if !positive_finite(constant_load.value) {
        return Err(ConstantEffortNonConsumption::NonPositiveConstantLoad);
    }
    let mut translational = Vec::new();
    for raw in &support.restraints {
        match parse_dof(raw) {
            Ok(dof) => {
                if dof.is_translational() {
                    translational.push(dof);
                }
            }
            Err(_) => {
                return Err(ConstantEffortNonConsumption::UnparseableRestraintDof(
                    raw.clone(),
                ))
            }
        }
    }
    match translational.as_slice() {
        [] => Err(ConstantEffortNonConsumption::NoTranslationalRestraintDof),
        [dof] => {
            let Some(node_index) = node_index(model, &support.node) else {
                return Err(ConstantEffortNonConsumption::UnknownNode);
            };
            Ok(ConstantEffortApplication {
                node_index,
                dof: *dof,
                force_newtons: constant_load.value,
            })
        }
        many => Err(ConstantEffortNonConsumption::MultipleTranslationalRestraintDofs(many.len())),
    }
}

/// Every solve-relevant constant-effort support with its consumption
/// disposition. A support carrying a `nonlinear` field keeps the existing
/// nonlinear-path handling and is not classified here.
fn constant_effort_solve_dispositions<'a>(
    model: &'a PreviewModel,
) -> Vec<(
    &'a PreviewSupport,
    Result<ConstantEffortApplication, ConstantEffortNonConsumption>,
)> {
    model
        .supports
        .iter()
        .filter(|support| support.nonlinear.is_none() && is_constant_effort_support(support))
        .map(|support| {
            (
                support,
                classify_constant_effort_consumption(model, support),
            )
        })
        .collect()
}

/// One non-blocking warning per non-consuming constant-effort support,
/// naming the unmet consumption condition. Emitted once per solve.
fn append_constant_effort_consumption_diagnostics(
    model: &PreviewModel,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (support, disposition) in constant_effort_solve_dispositions(model) {
        if let Err(reason) = disposition {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:constant-effort-support:{}:not-consumed",
                    stable_suffix(&support.id)
                ),
                "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED",
                "warning",
                format!(
                    "constant-effort support is not consumed by the assembled solve and remains user-entered review evidence: {}; assembled-solve consumption requires exactly one declared translational restraint DOF and a finite positive user-entered hanger.constant_load (DEC-049; no default, catalog value, or inferred direction is supplied)",
                    reason.unmet_condition()
                ),
                vec![support.id.clone(), support.node.clone()],
            ));
        }
    }
}

/// Single seam for the DEC-049 constant-effort consumption: each consuming
/// support contributes its constant nodal force to the per-load-case
/// assembled force vector, before `reduce_system`, so dense, sparse, and
/// nonlinear active-set solves consume it identically.
fn add_constant_effort_support_loads(ledger: &mut LoadLedger, model: &PreviewModel) {
    for (support, disposition) in constant_effort_solve_dispositions(model) {
        if let Ok(application) = disposition {
            // One term per application (S11 section 4.2).
            ledger.push(
                &support.id,
                application.node_index * DOF_PER_NODE + dof_index(application.dof),
                application.force_newtons,
            );
        }
    }
}

/// Per-load-case applied-load evidence rows for consuming constant-effort
/// supports, plus non-blocking user-limit comparison warnings against the
/// user's own `movement_limit` / `travel_range` entries (user-data-derived
/// comparison only; no software threshold, tolerance, or acceptance
/// criterion is introduced).
fn append_constant_effort_support_results(
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
    model: &PreviewModel,
    displacements: &[f64],
    load_case: &PreviewLoadCase,
) {
    for (support, disposition) in constant_effort_solve_dispositions(model) {
        let Ok(application) = disposition else {
            continue;
        };
        let hanger = support
            .hanger
            .as_ref()
            .expect("classified consuming constant-effort support carries hanger data");
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
        let suffix = stable_suffix(&support.id);
        let acting_dof = dof_name(application.dof);
        results.push(ResultItem {
            id: format!("result:constant-effort-support:{suffix}:applied-load"),
            kind: "constant_effort_support_applied_load".to_string(),
            value: application.force_newtons,
            unit: "N".to_string(),
            entity_ref: support.id.clone(),
            basis_ref: None,
            source_result_refs: Vec::new(),
            metadata: Some(ResultMetadata {
                component: "constant_effort_support_applied_load".to_string(),
                coordinate_system: "global".to_string(),
                location: format!("{}:{acting_dof} applied load", support.node),
                basis: format!(
                    "support_family=constant_effort_support;consumed_dof={acting_dof};source={source_reference};manufacturer={manufacturer_reference};load_side_review={load_side_review};mechanics_consumption=assembled_solve;dec_ref=DEC-049"
                ),
                sign_convention: CONSTANT_EFFORT_APPLIED_SIGN_CONVENTION.to_string(),
            }),
        });

        let computed =
            displacements[application.node_index * DOF_PER_NODE + dof_index(application.dof)];
        for (field_suffix, field_label, quantity) in [
            (
                "movement-limit",
                "movement_limit",
                hanger.movement_limit.as_ref(),
            ),
            ("travel-range", "travel_range", hanger.travel_range.as_ref()),
        ] {
            let Some(limit) = quantity else {
                continue;
            };
            if !positive_finite(limit.value) {
                continue;
            }
            if computed.abs() > limit.value {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:constant-effort-support:{suffix}:{field_suffix}:{}",
                        stable_suffix(&load_case.id)
                    ),
                    "SUPPORT_CONSTANT_EFFORT_USER_LIMIT_EXCEEDED",
                    "warning",
                    format!(
                        "computed displacement magnitude {} m at node {} along {acting_dof} exceeds the user-entered hanger.{field_label} value {} m in load case {}; this compares user-entered values only and introduces no software threshold, tolerance, or acceptance criterion",
                        computed.abs(),
                        support.node,
                        limit.value,
                        load_case.id
                    ),
                    vec![
                        support.id.clone(),
                        load_case.id.clone(),
                        format!("hanger.{field_label}"),
                    ],
                ));
            }
        }
    }
}

// Machine-consumed basis strings use the same finite round-trip decimal contract.
fn scalar_string(value: f64) -> String {
    optional_f64(Some(value))
}

fn append_endpoint_stress_results(
    results: &mut Vec<ResultItem>,
    pipe_id: &str,
    location: &str,
    components: &StressComponents,
    section_sign_convention: &str,
) {
    let suffix = stable_suffix(pipe_id);
    let id_location = endpoint_id_location(location);
    let local_components = [
        (
            "axial-normal",
            "element_local_axial_normal_stress",
            "axial_normal_stress",
            components.axial_normal,
            section_sign_convention,
        ),
        (
            "bending-normal-y",
            "element_local_bending_normal_stress_y",
            "bending_normal_stress_y",
            components.bending_normal_y,
            section_sign_convention,
        ),
        (
            "bending-normal-z",
            "element_local_bending_normal_stress_z",
            "bending_normal_stress_z",
            components.bending_normal_z,
            section_sign_convention,
        ),
        (
            "torsional-shear",
            "element_local_torsional_shear_stress",
            "torsional_shear_stress",
            components.torsional_shear,
            section_sign_convention,
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
                SECTION_RESULTANT_BASIS,
                sign_convention,
            );
        }
    }
}

fn append_station_stress_results(
    results: &mut Vec<ResultItem>,
    pipe_id: &str,
    location: &str,
    components: &StressComponents,
    basis: &str,
    section_sign_convention: Option<&str>,
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
            let sign_convention = section_sign_convention
                .map(str::to_string)
                .unwrap_or_else(|| {
                    format!(
                        "{}; j-side section action in the element-local frame (x toward end j); section equilibrium from stiffness-recovered end actions with consistent distributed-load fixed-end correction",
                        sign_convention.replace(
                            "interpolated element-local",
                            "section-equilibrium element-local"
                        )
                    )
                });
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
                &sign_convention,
            );
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
    basis: &str,
    sign_convention: &str,
) {
    results.push(ResultItem {
        id: id.to_string(),
        kind: kind.to_string(),
        value: value_pa / 1_000_000.0,
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
        value: value_pa / 1_000_000.0,
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

fn append_combined_vector_magnitude(
    combination: &PreviewCombination,
    expression: &CombinationExpression<'_>,
    base_id: &str,
    reference: &ResultItem,
    rows: &HashMap<String, HashMap<String, ResultItem>>,
    support_vectors: &HashMap<String, HashMap<String, [f64; 3]>>,
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let displacement = reference.kind == "displacement_magnitude";
    let dimension = if displacement {
        LoadDimension::Displacement
    } else {
        LoadDimension::Force
    };
    let mut vector = [0.0; 3];
    let mut source_refs = Vec::new();
    for (axis, suffix) in ["ux", "uy", "uz"].iter().enumerate() {
        let component_id = format!("{base_id}:{suffix}");
        let mut operands = Vec::new();
        for case in expression.operand_ids() {
            let value = if displacement {
                rows.get(&component_id)
                    .and_then(|cases| cases.get(case))
                    .filter(|row| {
                        row.entity_ref == reference.entity_ref
                            && row.unit == reference.unit
                            && row.kind
                                == [
                                    "global_nodal_displacement_x",
                                    "global_nodal_displacement_y",
                                    "global_nodal_displacement_z",
                                ][axis]
                    })
                    .map(|row| row.value)
            } else {
                support_vectors
                    .get(case)
                    .and_then(|supports| supports.get(&reference.entity_ref))
                    .map(|v| v[axis])
            };
            if let Some(value) = value.and_then(|v| AlgebraQuantity::new(v, dimension).ok()) {
                operands.push(AlgebraOperand::new(
                    case.to_string(),
                    case.to_string(),
                    value,
                    vec![
                        AlgebraAnalysisStatus::MechanicsSolved,
                        AlgebraAnalysisStatus::HumanReviewRequired,
                    ],
                ));
            } else {
                diagnostics.push(combination_diag(combination, base_id, "LOAD_COMBINATION_VECTOR_COMPONENT_MISSING", "warning",
                    "combined magnitude withheld because a signed source vector component is unavailable",
                    vec![combination.id.clone(), case.to_string(), reference.entity_ref.clone()]));
                return;
            }
        }
        let by_id = operands
            .iter()
            .map(|operand| (operand.operand_id.as_str(), operand))
            .collect();
        let algebra = expression.evaluate(&by_id);
        if algebra.is_blocked() || algebra.quantity.is_none() {
            diagnostics.push(combination_diag(
                combination,
                base_id,
                "LOAD_COMBINATION_VECTOR_INVALID",
                "warning",
                "combined magnitude withheld because signed vector algebra failed",
                vec![combination.id.clone(), reference.id.clone()],
            ));
            return;
        }
        vector[axis] = algebra.quantity.unwrap().value;
        if displacement {
            source_refs.push(qualified_combination_result_id(
                &combination.id,
                &component_id,
            ));
        }
    }
    if !displacement {
        for case in expression.operand_ids() {
            if let Some(row) = rows.get(base_id).and_then(|cases| cases.get(case)) {
                source_refs.push(row.id.clone());
            }
        }
    }
    let mut combined = reference.clone();
    combined.id = qualified_combination_result_id(&combination.id, base_id);
    combined.value = norm3(vector[0], vector[1], vector[2]);
    combined.basis_ref = Some(ResultBasisRef {
        ref_type: "combination".to_string(),
        ref_id: combination.id.clone(),
    });
    combined.source_result_refs = source_refs;
    // No new public component rows: reaction vectors are internal selected
    // support actions, and source edges retain their primitive reaction rows.
    results.push(combined);
}

fn append_combination_results(
    model: &PreviewModel,
    rows_by_base_id: &HashMap<String, HashMap<String, ResultItem>>,
    support_vectors_by_case: &HashMap<String, HashMap<String, [f64; 3]>>,
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
            if !matches!(expression, CombinationExpression::RangeEnvelope { .. })
                && matches!(
                    reference_row.kind.as_str(),
                    "displacement_magnitude" | "reaction_resultant"
                )
            {
                append_combined_vector_magnitude(
                    combination,
                    &expression,
                    base_id,
                    reference_row,
                    rows_by_base_id,
                    support_vectors_by_case,
                    results,
                    diagnostics,
                );
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
            combined.value = quantity.value;
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
    // T1 routes every blocked 0.4.0 envelope to load-reference-1 (0.4.0 is
    // exact-route only); it never publishes preview-physics-1.
    let exact_namespace = pressure_runtime::is_exact(&model) || case_state::is_load_state(&model);
    let preview = !exact_namespace;
    let mut envelope = MechanicsEnvelope {
        // An admitted exact namespace with blocked inputs has no recovered
        // physical evidence. Retain the empty namespace for truthful inspection;
        // this does not invent case/material/region evidence or solved results.
        contract_evidence: exact_namespace.then(|| {
            if case_state::is_load_state(&model) {
                serde_json::json!({"pressure": [], "connector": [], "exact_cases": [], "load_reference_states": []})
            } else {
                serde_json::json!({"pressure": [], "connector": [], "exact_cases": []})
            }
        }),
        schema_version: MECHANICS_SCHEMA_VERSION.to_string(),
        producer: mechanics_producer_for_model(&model),
        numerical_quality: assessed_numerical_quality(&model, &diagnostics),
        source_block_recovery: None,
        formulation_basis: formulation_basis_for_model(&model),
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
    };
    if preview {
        envelope.producer.semantic_contract_id = preview_physics::ID.into();
        envelope.formulation_basis = preview_physics::formulation_basis();
        envelope.contract_evidence = Some(preview_physics::empty_evidence());
        preview_physics::sanitize_blocked_diagnostics(&mut envelope.diagnostics);
    }
    envelope
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

fn max_abs_delta(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .fold(0.0, finite_observation_max)
}

fn max_abs_value(values: &[f64]) -> f64 {
    values
        .iter()
        .copied()
        .map(f64::abs)
        .fold(0.0, finite_observation_max)
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
        "rotation_x" => Ok(LoadDirection::Dof(FrameDof::Rx)),
        "rotation_y" => Ok(LoadDirection::Dof(FrameDof::Ry)),
        "rotation_z" => Ok(LoadDirection::Dof(FrameDof::Rz)),
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

// Historical quantization, retained only to demonstrate the superseded carrier loss.
#[cfg(test)]
fn round6(value: f64) -> f64 {
    let scaled = value * 1_000_000.0;
    let rounded = if value.is_finite() && !scaled.is_finite() {
        value
    } else {
        scaled.round() / 1_000_000.0
    };
    if rounded == 0.0 {
        0.0
    } else {
        rounded
    }
}

/// Crate-constant identity of this product-physics preview solver, for
/// producer-path metadata binding (R14 W1 T1). Derived from the crate
/// manifest, never hardcoded by consumers.
pub fn solver_component_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

/// Crate-constant version of this product-physics preview solver. See
/// [`solver_component_name`].
pub fn solver_component_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Assembled nonlinear active-set loop context for producer consumption
/// (additive pass-through; R14 W1 T1). Carries the loop component's
/// crate-constant identity/version and the loop's own assumptions and
/// limitations text, unchanged, so the governed analysis-run producer can
/// bind nonlinear metadata without depending on the solver crate directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonlinearAssembledLoopContext {
    pub component_name: &'static str,
    pub component_version: &'static str,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
}

/// Pass-through of `core/solver/nonlinear_integration`'s assembled-loop
/// context. Text and identity come from that crate's public surface; this
/// function adds nothing and rewrites nothing.
pub fn nonlinear_assembled_loop_context() -> NonlinearAssembledLoopContext {
    NonlinearAssembledLoopContext {
        component_name: open_pipe_stress_nonlinear_integration::assembled_loop_component_name(),
        component_version: open_pipe_stress_nonlinear_integration::assembled_loop_component_version(
        ),
        assumptions: open_pipe_stress_nonlinear_integration::assembled_loop_assumptions(),
        limitations: open_pipe_stress_nonlinear_integration::assembled_loop_limitations(),
    }
}

#[cfg(test)]
mod nonlinear_context_passthrough_tests {
    use super::nonlinear_assembled_loop_context;

    #[test]
    fn assembled_loop_context_is_a_pure_passthrough() {
        let context = nonlinear_assembled_loop_context();
        assert_eq!(
            context.component_name,
            open_pipe_stress_nonlinear_integration::assembled_loop_component_name()
        );
        assert_eq!(
            context.component_version,
            open_pipe_stress_nonlinear_integration::assembled_loop_component_version()
        );
        assert_eq!(
            context.assumptions,
            open_pipe_stress_nonlinear_integration::assembled_loop_assumptions()
        );
        assert_eq!(
            context.limitations,
            open_pipe_stress_nonlinear_integration::assembled_loop_limitations()
        );
        assert!(!context.assumptions.is_empty());
        assert!(!context.limitations.is_empty());
        assert!(!context.component_name.trim().is_empty());
        assert!(!context.component_version.trim().is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Analytic seed policy: relative 1e-9, no publication-rounding allowance.
    macro_rules! analytic_close {
        ($actual:expr, $expected:expr $(, $context:expr)? $(,)?) => {{
            let actual: f64 = $actual;
            let expected: f64 = $expected;
            assert!(actual.is_finite() && expected.is_finite());
            assert!(
                (actual - expected).abs() <= 1.0e-9 * expected.abs(),
                "actual={actual:.17e}, expected={expected:.17e}"
            );
        }};
    }

    /// Numerical admissibility of the Coulomb force relation in N for these
    /// nonzero SI fixtures, using the existing analytic 1e-9 relative criterion.
    /// This is not an active-set tolerance or a new nonlinear acceptance tier.
    fn assert_coulomb_force_balance_n(actual_n: f64, signed_mu_normal_n: f64) {
        assert_ne!(signed_mu_normal_n, 0.0);
        assert_eq!(
            actual_n.is_sign_negative(),
            signed_mu_normal_n.is_sign_negative()
        );
        analytic_close!(actual_n, signed_mu_normal_n);
    }

    fn precision_request(length: f64, torque: f64) -> LinearStaticPreviewRequest {
        // Reuse authored shape/provenance only; independent annulus/TL/GJ is below.
        let mut input = dec092_temperature_g_request();
        input.model.schema_version = "0.2.0".to_string();
        input.model.nodes[1].position.x = length;
        input.model.pipe_segments[0].section.outside_diameter.value = 0.20;
        input.model.load_cases.truncate(1);
        input.model.load_cases[0].modulus_basis_ref = None;
        input.model.load_cases[0].primitive_loads[0].magnitude.value = torque;
        input.model.combinations.clear();
        input
    }

    #[test]
    fn integrity_oblique_torsion_requires_actual_rotational_ground_in_both_modes() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            for torque in [0.0, 1.0] {
                for rotation in [None, Some("RZ"), Some("RX")] {
                    let mut input = precision_request(2.0, torque);
                    input.model.nodes[1].position.y = 2.0;
                    input.model.supports.truncate(1);
                    input.model.supports[0].family = Some("anchor".into());
                    input.model.supports[0].restraints =
                        vec!["UX".into(), "UY".into(), "UZ".into()];
                    if let Some(rotation) = rotation {
                        input.model.supports[0].restraints.push(rotation.into());
                    }
                    let mut tip = input.model.supports[0].clone();
                    tip.id = "support:integrity-tip-translations".into();
                    tip.node = input.model.nodes[1].id.clone();
                    tip.restraints = vec!["UX".into(), "UY".into(), "UZ".into()];
                    input.model.supports.push(tip);
                    let output = run_linear_static_preview_with_mode(input, mode);
                    let case = &output.numerical_quality.cases[0];
                    if rotation == Some("RX") {
                        assert_eq!(
                            output.status.mechanics, "MECHANICS_SOLVED",
                            "{:?}",
                            output.diagnostics
                        );
                        assert!(matches!(
                            case.solve_quality,
                            NumericalQualityStatus::ChecksPassed
                                | NumericalQualityStatus::Sensitive
                        ));
                    } else {
                        assert_ne!(output.status.mechanics, "MECHANICS_SOLVED");
                        assert_eq!(
                            case.structural_status,
                            StructuralStatus::PhysicalMechanismWitnessed,
                            "{:?}",
                            output.diagnostics
                        );
                        assert_eq!(case.solve_quality, NumericalQualityStatus::Failed);
                    }
                    assert_eq!(case.evidence_refs.len(), 1);
                    assert!(output
                        .diagnostics
                        .iter()
                        .any(|d| d.id == case.evidence_refs[0]));
                }
            }
        }
    }

    #[test]
    fn integrity_disconnected_body_is_not_restrained_by_another_bodys_anchor() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            let mut input = precision_request(2.0, 0.0);
            let mut a = input.model.nodes[0].clone();
            let mut b = input.model.nodes[1].clone();
            a.id = "node:integrity-disconnected-a".into();
            b.id = "node:integrity-disconnected-b".into();
            a.position.y = 5.0;
            b.position.y = 5.0;
            let mut pipe = input.model.pipe_segments[0].clone();
            pipe.id = "pipe:integrity-disconnected".into();
            pipe.from = a.id.clone();
            pipe.to = b.id.clone();
            input.model.nodes.extend([a, b]);
            input.model.pipe_segments.push(pipe);
            let output = run_linear_static_preview_with_mode(input, mode);
            assert_ne!(output.status.mechanics, "MECHANICS_SOLVED");
            assert_eq!(
                output.numerical_quality.cases[0].structural_status,
                StructuralStatus::PhysicalMechanismWitnessed,
                "{:?}",
                output.diagnostics
            );
        }
    }

    #[test]
    fn integrity_exact_case_ids_do_not_alias_passing_and_unassessed_cases() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            let mut input = precision_request(2.0, 1.0);
            input.model.load_cases[0].id = "load:a-b".into();
            let mut second = input.model.load_cases[0].clone();
            second.id = "load-a:b".into();
            for load in &mut second.primitive_loads {
                load.id.push_str(":second");
            }
            second.modulus_basis_ref = Some("temperature-point:intentionally-missing".into());
            input.model.load_cases.push(second);
            let output = run_linear_static_preview_with_mode(input, mode);
            assert_ne!(output.status.mechanics, "MECHANICS_SOLVED");
            let first = &output.numerical_quality.cases[0];
            let second = &output.numerical_quality.cases[1];
            assert_eq!(first.basis_ref.ref_id, "load:a-b");
            assert_eq!(
                first.solve_quality,
                NumericalQualityStatus::ChecksPassed,
                "{:?}",
                output.diagnostics
            );
            assert_eq!(second.basis_ref.ref_id, "load-a:b");
            assert_eq!(
                second.solve_quality,
                NumericalQualityStatus::NotAssessed,
                "{:?}",
                output.diagnostics
            );
            assert!(second.evidence_refs.is_empty());
            assert_ne!(
                integrity_diagnostic_id(&first.basis_ref.ref_id),
                integrity_diagnostic_id(&second.basis_ref.ref_id)
            );
            assert_eq!(
                output.numerical_quality.status,
                NumericalQualityStatus::NotAssessed
            );
        }
    }

    #[test]
    fn integrity_exact_case_ids_keep_actual_linear_passes_and_component_warnings_distinct() {
        let mut input = mechanical_fixture_for_test(request(), "tests::integrity_exact_case_ids_keep_actual_linear_passes_and_component_warnings_distinct");
        input.model.load_cases.truncate(2);
        assert_eq!(input.model.load_cases.len(), 2);
        input.model.load_cases[0].id = "load:a-b".into();
        input.model.load_cases[1].id = "load-a:b".into();
        input.model.combinations.clear();
        // A new explicit linear-only companion; the retained nonlinear fixture
        // remains separately checked against the adopted equilibrium successor.
        input
            .model
            .supports
            .retain(|support| support.nonlinear.is_none());
        let output = run_linear_static_preview(input);
        assert_eq!(
            output.status.mechanics, "MECHANICS_SOLVED",
            "{:?}",
            output.diagnostics
        );
        let cases = &output.numerical_quality.cases;
        assert_eq!(cases.len(), 2);
        assert!(cases.iter().all(|case| matches!(
            case.solve_quality,
            NumericalQualityStatus::ChecksPassed | NumericalQualityStatus::Sensitive
        )));
        assert_ne!(cases[0].evidence_refs, cases[1].evidence_refs);
        let warnings = output
            .diagnostics
            .iter()
            // T0R: preview-physics-1 retires the SIF×k multiplier diagnostic; its
            // case-scoped successor is the equal-factor intensification record.
            .filter(|d| d.code == "COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED")
            .collect::<Vec<_>>();
        assert!(!warnings.is_empty());
        assert_eq!(
            warnings.len(),
            warnings.iter().map(|d| &d.id).collect::<HashSet<_>>().len()
        );
        for case in cases {
            assert!(warnings
                .iter()
                .any(|warning| warning.affected_refs.contains(&case.basis_ref.ref_id)));
        }
        assert_ne!(
            exact_source_identity(&["a:b", "c"]),
            exact_source_identity(&["a", "b:c"])
        );
        assert_ne!(
            exact_source_identity(&["é", ":"]),
            exact_source_identity(&["é:", ""])
        );
    }

    #[test]
    fn integrity_multicase_evidence_and_component_diagnostics_have_unique_real_case_identity() {
        let input = mechanical_fixture_for_test(request(), "tests::integrity_multicase_evidence_and_component_diagnostics_have_unique_real_case_identity");
        let cases = input
            .model
            .load_cases
            .iter()
            .map(|c| c.id.clone())
            .collect::<Vec<_>>();
        let output = run_linear_static_preview(input);
        assert_eq!(
            output.status.mechanics, "MECHANICS_SOLVED",
            "{:?}",
            output.diagnostics
        );
        assert_eq!(output.numerical_quality.cases.len(), cases.len());
        let diagnostics = output
            .diagnostics
            .iter()
            // T0R successor of the retired multiplier diagnostic (case scoped).
            .filter(|d| d.code == "COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED")
            .collect::<Vec<_>>();
        assert!(!diagnostics.is_empty());
        assert_eq!(
            diagnostics
                .iter()
                .map(|d| &d.id)
                .collect::<HashSet<_>>()
                .len(),
            diagnostics.len()
        );
        for case in cases {
            assert!(diagnostics.iter().any(|d| d.affected_refs.contains(&case)));
            let quality = output
                .numerical_quality
                .cases
                .iter()
                .find(|q| q.basis_ref.ref_id == case)
                .unwrap();
            assert_eq!(
                quality.structural_status,
                StructuralStatus::PassiveModelBasis,
                "{:?}",
                output.diagnostics
            );
            assert!(matches!(
                quality.solve_quality,
                NumericalQualityStatus::ChecksPassed | NumericalQualityStatus::Sensitive
            ));
            assert!(output
                .diagnostics
                .iter()
                .any(|d| d.id == quality.evidence_refs[0]
                    && d.message.contains(
                        open_pipe_stress_nonlinear_integration::product_equilibrium::POLICY
                    )
                    && d.message.contains("passed: true")));
            assert_eq!(quality.evidence_refs.len(), 1);
            assert!(output
                .diagnostics
                .iter()
                .any(|d| d.id == quality.evidence_refs[0] && d.affected_refs.contains(&case)));
        }
    }

    #[test]
    fn precision_signed_subquantum_torsion_both_modes_matches_independent_annulus() {
        // Reviewed N08/N09: J=0.000017195*pi m^4, G=80e9 Pa.
        // No production section, stiffness or solver supplies the expected value.
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            for length in [2.0, 10.0] {
                for torque in [-1.0, -0.1, 0.1, 1.0] {
                    let output = run_linear_static_preview_with_mode(
                        precision_request(length, torque),
                        mode,
                    );
                    assert_eq!(output.status.mechanics, "MECHANICS_SOLVED");
                    let actual = result_value(&output, "result:disp:node-N-DEC092-TIP:rx");
                    let expected = torque * length / (80e9 * 0.000017195 * PI);
                    assert_ne!(actual, 0.0);
                    assert_eq!(actual.is_sign_negative(), torque.is_sign_negative());
                    analytic_close!(actual, expected);
                    let encoded = serde_json::to_string(&output).unwrap();
                    let parsed: serde_json::Value = serde_json::from_str(&encoded).unwrap();
                    let transported = find_result(&parsed, "result:disp:node-N-DEC092-TIP:rx")
                        ["value"]
                        .as_f64()
                        .unwrap();
                    assert_eq!(transported.to_bits(), actual.to_bits());
                }
            }
        }
    }

    #[test]
    fn precision_n01_bending_and_summary_match_independent_eb_reference() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            let mut input = precision_request(2.0, 0.0);
            let load = &mut input.model.load_cases[0].primitive_loads[0];
            load.category = "concentrated_force".to_string();
            load.direction = "UY".to_string();
            load.magnitude = Quantity {
                value: 1000.0,
                unit: "N".to_string(),
            };
            load.dimension = "force".to_string();
            let output = run_linear_static_preview_with_mode(input, mode);
            assert_eq!(output.status.mechanics, "MECHANICS_SOLVED");
            let uy = result_value(&output, "result:disp:node-N-DEC092-TIP:uy");
            analytic_close!(uy, 8_000_000.0 / (5_158_500.0 * PI)); // mm
            analytic_close!(
                result_value(&output, "result:disp:node-N-DEC092-TIP:rz"),
                2000.0 / (1_719_500.0 * PI)
            );
            let maximum = output.summary.max_displacement.as_ref().unwrap();
            assert_eq!(maximum.value.to_bits(), uy.to_bits());
            let parsed: serde_json::Value =
                serde_json::from_str(&serde_json::to_string(&output).unwrap()).unwrap();
            assert_eq!(
                parsed["summary"]["max_displacement"]["value"]
                    .as_f64()
                    .unwrap()
                    .to_bits(),
                maximum.value.to_bits()
            );
        }
    }

    #[test]
    fn precision_same_unit_raw_quantity_bits_and_numeric_strings_roundtrip() {
        // Raw serde transport scope only. I-JSON derivative export has a separate
        // unsafe-integer guard; these values do not assert universal exportability.
        let quantum = (0.5e-6_f64).to_bits();
        for bits in [
            1,
            0x000f_ffff_ffff_ffff,
            0x0010_0000_0000_0000,
            0x3fb9_9999_9999_999a,
            0x3ff0_0000_0000_0001,
            0x4340_0000_0000_0001,
            0x7fef_ffff_ffff_ffff,
            quantum - 1,
            quantum,
            quantum + 1,
            (1e-12_f64).to_bits(),
            (1e-6_f64).to_bits(),
            (1e6_f64).to_bits(),
        ] {
            for sign in [0, 1_u64 << 63] {
                let value = f64::from_bits(bits | sign);
                let quantity = LocatedQuantity {
                    value,
                    unit: "rad".to_string(),
                    location_ref: "node:test".to_string(),
                    result_ref: "result:test".to_string(),
                };
                let row = ResultItem {
                    id: "result:test".to_string(),
                    kind: "global_nodal_rotation_x".to_string(),
                    value,
                    unit: "rad".to_string(),
                    entity_ref: "node:test".to_string(),
                    basis_ref: None,
                    source_result_refs: vec![],
                    metadata: None,
                };
                for encoded in [
                    serde_json::to_string(&quantity).unwrap(),
                    serde_json::to_string(&row).unwrap(),
                ] {
                    let parsed: serde_json::Value = serde_json::from_str(&encoded).unwrap();
                    assert_eq!(parsed["value"].as_f64().unwrap().to_bits(), value.to_bits());
                    assert_eq!(parsed["unit"], "rad");
                }
                for text in [optional_f64(Some(value)), scalar_string(value)] {
                    assert_eq!(text.parse::<f64>().unwrap().to_bits(), value.to_bits());
                }
            }
        }
    }

    #[test]
    fn precision_nonfinite_values_are_rejected_not_serialized_as_null() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(require_finite_mechanics([value]).is_err());
            assert_eq!(optional_f64(Some(value)), "not_observed");
            let quantity = LocatedQuantity {
                value,
                unit: "rad".to_string(),
                location_ref: "node:test".to_string(),
                result_ref: "result:test".to_string(),
            };
            let row = ResultItem {
                id: "result:test".to_string(),
                kind: "global_nodal_rotation_x".to_string(),
                value,
                unit: "rad".to_string(),
                entity_ref: "node:test".to_string(),
                basis_ref: None,
                source_result_refs: vec![],
                metadata: None,
            };
            assert!(serde_json::to_string(&quantity).is_err());
            assert!(serde_json::to_string(&row).is_err());
        }
        assert_eq!(optional_f64(None), "not_observed");
    }

    #[test]
    fn precision_solved_and_blocked_headers_bind_actual_producer_without_assessment() {
        let solved = run_linear_static_preview(precision_request(2.0, 1.0));
        let mut input = precision_request(2.0, 1.0);
        input.materials.clear();
        let blocked = run_linear_static_preview(input);
        assert_eq!(solved.status.mechanics, "MECHANICS_SOLVED");
        assert_ne!(blocked.status.mechanics, "MECHANICS_SOLVED");
        for output in [solved, blocked] {
            let encoded = serde_json::to_value(&output).unwrap();
            assert_eq!(encoded["schema_version"], "0.2.0");
            assert_eq!(
                encoded["producer"]["component_name"],
                env!("CARGO_PKG_NAME")
            );
            assert_eq!(
                encoded["producer"]["component_version"],
                env!("CARGO_PKG_VERSION")
            );
            assert_eq!(env!("CARGO_PKG_VERSION"), "0.2.0");
            // T0R: fresh ordinary-route solves (solved or blocked) publish
            // preview-physics-1; precision-1 is historical only.
            assert_eq!(
                encoded["producer"]["semantic_contract_id"],
                "openpipestress.result_semantics/0.3.0/preview-physics-1"
            );
            assert_eq!(
                encoded["numerical_quality"]["value_representation"],
                "finite_binary64"
            );
            assert_eq!(
                encoded["numerical_quality"]["publication_quantization"],
                "none"
            );
            assert_eq!(
                encoded["numerical_quality"]["integrity_policy"],
                "M03-INTEGRITY-v1"
            );
            assert!(!output.numerical_quality.cases.is_empty());
            if output.status.mechanics == "MECHANICS_SOLVED" {
                assert!(matches!(
                    output.numerical_quality.status,
                    NumericalQualityStatus::ChecksPassed | NumericalQualityStatus::Sensitive
                ));
            } else {
                assert_ne!(
                    output.numerical_quality.status,
                    NumericalQualityStatus::ChecksPassed
                );
            }
            assert_eq!(
                encoded["formulation_basis"]["profile_id"],
                "product_preview_mechanics_v1"
            );
            assert_eq!(
                output.formulation_basis.limitations,
                preview_physics::LIMITATIONS.map(str::to_string).to_vec()
            );
            assert!(output.contract_evidence.is_some());
        }
    }

    #[test]
    fn historical_round6_demonstrates_subquantum_information_loss() {
        for value in [-4.63e-7, -4.63e-8, 4.63e-8, 4.63e-7] {
            assert_ne!(value, 0.0);
            assert_eq!(round6(value), 0.0);
        }
    }

    // Each call names an individually reviewed nonpressure purpose. The bundled request() stays unchanged.
    fn mechanical_fixture_for_test(
        mut input: LinearStaticPreviewRequest,
        purpose: &str,
    ) -> LinearStaticPreviewRequest {
        assert!(matches!(purpose,
            "tests::audit_retained_spring_fixture_uses_selected_node_state"
            | "tests::authored_coordinate_units_preserve_preview_results_and_source"
            | "tests::bend_component_user_multipliers_emit_stress_review_rows"
            | "tests::branch_component_user_multipliers_emit_side_specific_stress_review_rows"
            | "tests::combination_stress_summary_rows_are_skipped_with_diagnostics"
            | "tests::constant_effort_coexists_with_nonlinear_supports_and_nonlinear_field_precedence"
            | "tests::curved_bend_macro_element_emits_arc_interior_station_results"
            | "tests::dense_scrutiny_mode_keeps_sparse_parity_row"
            | "tests::f3_canonical_spring_retains_elastic_stiffness"
            | "tests::f3_explicit_six_dof_guide_keeps_family_and_reports_invalid_rotations"
            | "tests::f3_missing_and_null_family_preserve_existing_inference_and_payloads"
            | "tests::f3_real_hanger_nonlinear_and_dof_aliases_remain_supported"
            | "tests::legacy_bend_mode_keeps_multiplier_and_chord_realization_unchanged"
            | "tests::mill_tolerance_units_are_normalized_at_preview_boundary"
            | "tests::missing_load_input_blocks_with_diagnostic"
            | "tests::missing_material_blocks_with_diagnostic"
            | "tests::missing_or_invalid_project_length_units_block_preview"
            | "tests::missing_pipe_orientation_blocks_with_diagnostic"
            | "tests::models_without_constant_effort_supports_are_untouched_by_the_consumption_path"
            | "tests::operation_authored_primitive_categories_map_to_preview_mechanics"
            | "tests::range_combination_records_each_operand_modulus_basis"
            | "tests::range_envelope_combination_selects_each_shipped_mode_deterministically"
            | "tests::shared_section_blank_reference_identity_blocks_common_entry"
            | "tests::shared_section_equivalent_but_different_cache_representation_is_stale"
            | "tests::shared_section_local_mill_tolerance_must_leave_positive_wall"
            | "tests::shared_section_reference_failures_are_explicit"
            | "tests::shared_section_stale_cache_blocks_common_solver_entry"
            | "tests::spring_hanger_user_inputs_emit_review_rows_without_catalog_defaults"
            | "tests::subtraction_combination_subtracts_solved_rows_with_signed_determinism"
            | "tests::under_restrained_model_reports_solver_diagnostic"
            | "tests::valid_invented_model_exposes_element_force_components"
            | "tests::valid_invented_model_exposes_explicit_load_combination_results"
            | "tests::valid_invented_model_exposes_global_displacement_components"
            | "tests::valid_invented_model_solves_deterministically"
            | "tests::p5_adjacent_spans_and_qualified_case_edges_preserve_physics"
            | "tests::mixed_units_are_normalized_at_preview_mechanics_boundary_without_pressure"
            | "tests::valid_invented_model_exposes_endpoint_stress_components_without_pressure"
            | "tests::integrity_exact_case_ids_keep_actual_linear_passes_and_component_warnings_distinct"
            | "tests::integrity_multicase_evidence_and_component_diagnostics_have_unique_real_case_identity"
        ), "unreviewed pressure-free fixture purpose: {purpose}");
        // U3 (D-2 A1): a pressure primitive of any value, zero included, is refused
        // on every route, so the demo's named legacy pressures are removed.
        let mut changed = 0;
        for case in &mut input.model.load_cases {
            let case_id = case.id.clone();
            case.primitive_loads.retain(|load| {
                let named = matches!(
                    (case_id.as_str(), load.id.as_str()),
                    ("load:L-100", "load:L-100-P" | "load:L-100-P-EJ")
                        | ("load:L-200", "load:L-200-P" | "load:L-200-P-EJ")
                );
                if named {
                    assert_eq!(load.category, "pressure");
                    assert_eq!(load.dimension, "pressure");
                    changed += 1;
                }
                !named
            });
        }
        assert!(
            changed > 0 && changed <= 4,
            "expected named inherited fixture pressures for {purpose}"
        );
        // Omit the demo's legacy joint C-150, which every route refuses
        // (LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED, D-4).
        input.model.components.retain(|component| component.id != "component:C-150");
        input
    }

    fn mechanical_stress_fixture_for_test(
        input: LinearStaticPreviewRequest,
        purpose: &str,
    ) -> LinearStaticPreviewRequest {
        assert_eq!(
            purpose,
            "tests::valid_invented_model_exposes_endpoint_stress_components_without_pressure"
        );
        let mut input = mechanical_fixture_for_test(input, purpose);
        for case in &mut input.model.load_cases {
            case.primitive_loads.retain(|load| {
                !matches!(
                    load.id.as_str(),
                    "load:L-100-P" | "load:L-100-P-EJ" | "load:L-200-P" | "load:L-200-P-EJ"
                )
            });
        }
        input
    }

    #[test]
    fn bundled_demo_with_legacy_nonzero_pressure_is_refused_on_the_ordinary_route() {
        // U3: legacy pressure is retired; the unchanged bundled demo carries nonzero
        // legacy pressure primitives and is refused before any solve.
        let output = run_linear_static_preview(request());
        assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE");
        assert!(output.results.is_empty());
        assert!(output
            .diagnostics
            .iter()
            .any(|d| d.code == "PRESSURE_MODEL_REAUTHOR_REQUIRED"));
    }

    // Current arc/chord integration control: tip force plus uniform weight, no pressure input.
    #[test]
    fn endpoint_section_cut_curved_endpoints_use_all_six_arc_resultants_without_pressure() {
        let mut request = curved_bend_span_request();
        request.model.load_cases[0]
            .primitive_loads
            .push(curved_bend_uniform_weight_load());
        let derived = derive_pipe_section(
            &request.model.pipe_segments[0].section,
            "pipe:P-100",
            &mut Vec::new(),
        )
        .unwrap();
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");

        let row_ids = [
            "result:force:pipe-P-100:axial",
            "result:force:pipe-P-100:shear-y",
            "result:force:pipe-P-100:shear-z",
            "result:moment:pipe-P-100:torsion",
            "result:moment:pipe-P-100:bending-y",
            "result:moment:pipe-P-100:bending-z",
        ];
        let mut corrected = vec![0.0; ELEMENT_DOF];
        for (slot, row_id) in row_ids.iter().enumerate() {
            corrected[slot] = result_value(&result, row_id);
            corrected[DOF_PER_NODE + slot] = result_value(&result, &format!("{row_id}:end-j"));
        }
        let material = &invented_materials()[0];
        let pipe_section = StraightPipeSectionProperties::new(
            material.elastic_modulus.value,
            material
                .shear_modulus
                .as_ref()
                .expect("validated material G")
                .value,
            derived.area,
            derived.second_moment,
            derived.second_moment,
            derived.torsion_constant,
            None,
        )
        .unwrap();
        let pipe = StraightPipeElement::new(
            "pipe:P-100",
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, [CURVED_BEND_TEST_CHORD_M, 0.0, 0.0]).unwrap(),
            pipe_section,
            [0.0, 1.0, 0.0],
        )
        .unwrap();
        let build = curved_bend_direct_build();
        let intensity = [0.0, 0.0, -190.0];
        let node_j_force: [f64; DOF_PER_NODE] = corrected[DOF_PER_NODE..]
            .try_into()
            .expect("six j-end action slots");

        // The public endpoint rows stay the chord-frame node-on-element
        // actions. Reconstruct that independent direct solve before testing
        // the separate arc section-cut resultants consumed by stress.
        let uniform_equivalent = build
            .macro_element
            .consistent_uniform_nodal_loads(intensity)
            .unwrap();
        let mut applied = uniform_equivalent;
        applied[DOF_PER_NODE + UY] += 1000.0;
        let displacements = curved_bend_direct_solution(&applied);
        let stiffness = build.macro_element.global_stiffness().unwrap();
        let mut expected_raw = [0.0; ELEMENT_DOF];
        for row in 0..ELEMENT_DOF {
            for col in 0..ELEMENT_DOF {
                expected_raw[row] += stiffness[row][col] * displacements[col];
            }
            expected_raw[row] -= uniform_equivalent[row];
            assert!(
                (corrected[row] - round6(expected_raw[row])).abs() <= 1.1e-6,
                "raw chord-frame action slot {row}: {} != {}",
                corrected[row],
                round6(expected_raw[row])
            );
        }
        for row_id in &row_ids {
            for (id, location) in [
                ((*row_id).to_string(), "end_i"),
                (format!("{row_id}:end-j"), "end_j"),
            ] {
                let metadata = result
                    .results
                    .iter()
                    .find(|row| row.id == id)
                    .and_then(|row| row.metadata.as_ref())
                    .unwrap();
                assert_eq!(metadata.location, location);
                // T0R (N-2): arc endpoint force rows are labelled with their
                // actual chord frame on preview-physics-1; values unchanged.
                assert_eq!(metadata.coordinate_system, "arc_chord_frame");
                assert_eq!(metadata.basis, SECTION_RESULTANT_BASIS);
                assert!(metadata.sign_convention.contains("force vector"));
            }
        }

        for (fraction, location) in [(0.0, "end-i"), (1.0, "end-j")] {
            let actual = curved_bend_section_resultants(
                &build,
                &pipe,
                &corrected,
                &[intensity],
                fraction,
            )
            .unwrap();
            let expected = build
                .macro_element
                .arc_section_resultants(fraction, node_j_force, intensity)
                .unwrap();
            for slot in 0..6 {
                assert!(
                    (actual[slot] - expected[slot]).abs() <= 1.0e-9 * expected[slot].abs().max(1.0),
                    "{location} resultant slot {slot}: {} != {}",
                    actual[slot],
                    expected[slot]
                );
            }
            let recovered = recover_section_stress(&actual, &derived);
            let expected_stresses = [
                ("axial-normal", recovered.components.axial_normal.unwrap()),
                (
                    "bending-normal-y",
                    recovered.components.bending_normal_y.unwrap(),
                ),
                (
                    "bending-normal-z",
                    recovered.components.bending_normal_z.unwrap(),
                ),
                (
                    "torsional-shear",
                    recovered.components.torsional_shear.unwrap(),
                ),
            ];
            for (tail, expected_pa) in expected_stresses {
                p5_close(
                    result_value(
                        &result,
                        &format!("result:stress:pipe-P-100:{location}:{tail}"),
                    ),
                    expected_pa / 1.0e6,
                );
            }
            let metadata = result
                .results
                .iter()
                .find(|row| row.id == format!("result:stress:pipe-P-100:{location}:axial-normal"))
                .and_then(|row| row.metadata.as_ref())
                .unwrap();
            assert_eq!(metadata.coordinate_system, "element_local");
            // T0R (N-2): arc stress components carry the nominal-basis discriminator.
            assert_eq!(metadata.basis, "nominal_straight_beam_formula_on_arc_resultants");
            assert_eq!(
                metadata.sign_convention,
                CURVED_BEND_SECTION_SIGN_CONVENTION
            );
        }

        for (fraction, station) in [(0.25, "quarter-1"), (0.5, "midspan"), (0.75, "quarter-3")] {
            let expected = build
                .macro_element
                .arc_section_resultants(fraction, node_j_force, intensity)
                .unwrap();
            for (slot, (family, tail)) in [
                ("force", "axial"),
                ("force", "shear-y"),
                ("force", "shear-z"),
                ("moment", "torsion"),
                ("moment", "bending-y"),
                ("moment", "bending-z"),
            ]
            .into_iter()
            .enumerate()
            {
                let actual = result_value(
                    &result,
                    &format!("result:{family}:pipe-P-100:{station}:{tail}"),
                );
                assert!(
                    (actual - round6(expected[slot])).abs() <= 1.1e-6,
                    "{station} resultant slot {slot}: {actual} != {}",
                    round6(expected[slot])
                );
            }
            let metadata = result
                .results
                .iter()
                .find(|row| row.id == format!("result:force:pipe-P-100:{station}:axial"))
                .and_then(|row| row.metadata.as_ref())
                .unwrap();
            assert_eq!(metadata.coordinate_system, "element_local");
            assert_eq!(metadata.basis, SECTION_RESULTANT_BASIS);
            assert_eq!(
                metadata.sign_convention,
                CURVED_BEND_SECTION_SIGN_CONVENTION
            );
            let stress_metadata = result
                .results
                .iter()
                .find(|row| row.id == format!("result:stress:pipe-P-100:{station}:axial-normal"))
                .and_then(|row| row.metadata.as_ref())
                .unwrap();
            assert_eq!(stress_metadata.coordinate_system, "element_local");
            // T0R (N-2): arc station stress rows carry the nominal-basis discriminator.
            assert_eq!(stress_metadata.basis, "nominal_straight_beam_formula_on_arc_resultants");
            assert_eq!(
                stress_metadata.sign_convention,
                CURVED_BEND_SECTION_SIGN_CONVENTION
            );
        }
    }

    #[test]
    fn mixed_units_are_normalized_at_preview_mechanics_boundary_without_pressure() {
        let baseline = run_linear_static_preview(mechanical_fixture_for_test(
            request(),
            "tests::mixed_units_are_normalized_at_preview_mechanics_boundary_without_pressure",
        ));
        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::mixed_units_are_normalized_at_preview_mechanics_boundary_without_pressure",
        );
        request.materials[0].elastic_modulus = Quantity {
            value: 200_000.0,
            unit: "MPa".to_string(),
        };
        request.materials[0].shear_modulus = Some(Quantity {
            value: 77_000.0,
            unit: "MPa".to_string(),
        });
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
    }

    #[test]
    fn valid_invented_model_exposes_endpoint_stress_components_without_pressure() {
        let result = run_linear_static_preview(mechanical_stress_fixture_for_test(
            request(),
            "tests::valid_invented_model_exposes_endpoint_stress_components_without_pressure",
        ));
        let result_ids = result
            .results
            .iter()
            .map(|item| item.id.as_str())
            .collect::<HashSet<_>>();

        assert!(has_member_maximum(&result, "pipe:P-120"));
        assert!(result_ids.contains("result:stress:pipe-P-120:end-i:axial-normal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:end-i:torsional-shear"));
        assert!(result_ids.contains("result:stress:pipe-P-120:end-j:axial-normal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:end-j:torsional-shear"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-1:axial-normal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-1:bending-normal-y"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-1:bending-normal-z"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-1:torsional-shear"));
        assert!(result_ids.contains("result:stress:pipe-P-120:midspan:axial-normal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:midspan:bending-normal-y"));
        assert!(result_ids.contains("result:stress:pipe-P-120:midspan:bending-normal-z"));
        assert!(result_ids.contains("result:stress:pipe-P-120:midspan:torsional-shear"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-3:axial-normal"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-3:bending-normal-y"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-3:bending-normal-z"));
        assert!(result_ids.contains("result:stress:pipe-P-120:quarter-3:torsional-shear"));
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
                            && metadata.basis == "recovered_from_local_element_stiffness"
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
                            && metadata.basis == "recovered_from_open_mechanics_stress_components"
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
                            && metadata.basis == "recovered_from_open_mechanics_stress_components"
                    })
                    .unwrap_or(false)
        }));
        assert!(!result
            .diagnostics
            .iter()
            .any(|item| item.code == "PRESSURE_LOAD_NOT_APPLIED_TO_FRAME_VECTOR"));
    }

    #[test]
    fn nonlinear_support_current_pressure_free_law_in_both_modes() {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            let result =
                run_linear_static_preview_with_mode(friction_sliding_preview_request(), mode);
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
    }

    fn rigid_preview_support(family: &str, restraints: &[&str]) -> PreviewSupport {
        PreviewSupport {
            id: format!("support:{family}"),
            node: "node:N-100".to_string(),
            restraints: restraints.iter().map(|dof| (*dof).to_string()).collect(),
            family: Some(family.to_string()),
            stiffness: None,
            hanger: None,
            nonlinear: None,
            provenance: Some("invented_example_user_input".to_string()),
        }
    }

    #[test]
    fn f3_family_tokens_accept_exact_canonical_vocabulary() {
        let mut request = request();
        for family in [
            "anchor",
            "guide",
            "line_stop",
            "vertical_support",
            "spring",
            "variable_spring_hanger",
            "spring_hanger",
            "constant_effort_support",
            "nonlinear",
        ] {
            // This isolates spelling validity, not payload-class consistency.
            request.model.supports[0].family = Some(family.to_string());
            let mut diagnostics = Vec::new();
            validate_support_family_tokens(&request.model, &mut diagnostics);
            assert!(diagnostics.is_empty(), "{family}: {diagnostics:?}");
        }
    }

    #[test]
    fn f3_invalid_family_blocks_every_payload_before_mechanics() {
        for id in [
            "support:S-100",
            "support:SH-140",
            "support:CE-120",
            "support:NL-140",
            "spring-payload",
        ] {
            for token in [
                "",
                " ",
                " guide",
                "guide ",
                "Anchor",
                "Guide",
                "LineStop",
                "VerticalSupport",
                "Spring",
                "unknown",
            ] {
                let mut request = request();
                let support = request
                    .model
                    .supports
                    .iter_mut()
                    .find(|support| {
                        support.id
                            == if id == "spring-payload" {
                                "support:SH-140"
                            } else {
                                id
                            }
                    })
                    .unwrap();
                if id == "spring-payload" {
                    support.stiffness = support.hanger.as_ref().unwrap().stiffness.clone();
                    support.hanger = None;
                }
                support.family = Some(token.to_string());
                let support_id = support.id.clone();
                let result = run_linear_static_preview(request);
                assert_eq!(
                    result.status.mechanics, "MODEL_INCOMPLETE",
                    "{id}/{token:?}"
                );
                assert!(result.results.is_empty(), "{id}/{token:?}");
                let diagnostic = result
                    .diagnostics
                    .iter()
                    .find(|d| d.code == "SUPPORT_FAMILY_INVALID")
                    .unwrap();
                assert_eq!(diagnostic.severity, "blocking");
                assert_eq!(
                    diagnostic.id,
                    format!("diagnostic:support:{}:family", stable_suffix(&support_id))
                );
                assert!(diagnostic.message.contains(&format!("{token:?}")));
                assert_eq!(
                    diagnostic.affected_refs,
                    vec![support_id.clone(), format!("{support_id}.family")]
                );
            }
        }
    }

    #[test]
    fn f3_missing_and_null_family_preserve_existing_inference_and_payloads() {
        let baseline = run_linear_static_preview(mechanical_fixture_for_test(
            request(),
            "tests::f3_missing_and_null_family_preserve_existing_inference_and_payloads",
        ));
        for null in [false, true] {
            let mut raw: serde_json::Value = serde_json::from_str(include_str!(
                "../../../fixtures/product_preview/invented_preview_model.json"
            ))
            .unwrap();
            for support in raw["supports"].as_array_mut().unwrap() {
                if null {
                    support["family"] = serde_json::Value::Null;
                } else {
                    support.as_object_mut().unwrap().remove("family");
                }
            }
            let model: PreviewModel = serde_json::from_value(raw).unwrap();
            assert!(model
                .supports
                .iter()
                .all(|support| support.family.is_none()));
            let support = &model.supports[0];
            let dofs = support
                .restraints
                .iter()
                .map(|dof| parse_dof(dof).unwrap())
                .collect();
            assert_eq!(
                rigid_linear_support_from_preview(support, 0, dofs).family,
                SupportFamily::Anchor
            );
            let result = run_linear_static_preview(mechanical_fixture_for_test(
                LinearStaticPreviewRequest {
                    model,
                    materials: invented_materials(),
                },
                "tests::f3_missing_and_null_family_preserve_existing_inference_and_payloads",
            ));
            assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
            assert_eq!(
                serde_json::to_value(&result.results).unwrap(),
                serde_json::to_value(&baseline.results).unwrap()
            );
        }
    }

    #[test]
    fn f3_explicit_six_dof_guide_keeps_family_and_reports_invalid_rotations() {
        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::f3_explicit_six_dof_guide_keeps_family_and_reports_invalid_rotations",
        );
        let support = &mut request.model.supports[0];
        support.family = Some("guide".to_string());
        let dofs = support
            .restraints
            .iter()
            .map(|dof| parse_dof(dof).unwrap())
            .collect();
        let mapped = rigid_linear_support_from_preview(support, 0, dofs);
        assert_eq!(mapped.family, SupportFamily::Guide);
        let boundary = open_pipe_stress_linear_supports::prepare_boundary(
            request.model.nodes.len(),
            &[mapped],
        );
        assert_eq!(
            boundary
                .findings
                .iter()
                .filter(|finding| finding.code
                    == open_pipe_stress_linear_supports::FindingCode::InvalidSupportDof)
                .count(),
            3
        );
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result.results.is_empty());
        assert!(result
            .diagnostics
            .iter()
            .any(|d| d.code == "SUPPORT_INPUT_INVALID"));
    }

    #[test]
    fn f3_canonical_spring_retains_elastic_stiffness() {
        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::f3_canonical_spring_retains_elastic_stiffness",
        );
        let support = request
            .model
            .supports
            .iter_mut()
            .find(|s| s.id == "support:SH-140")
            .unwrap();
        support.family = Some("spring".to_string());
        support.stiffness = support.hanger.as_ref().unwrap().stiffness.clone();
        support.hanger = None;
        let expected = support.stiffness.as_ref().unwrap().value.value;
        let mut diagnostics = Vec::new();
        let built = build_model(&request.model, &request.materials, &mut diagnostics).unwrap();
        let mapped = built
            .supports
            .iter()
            .find(|s| s.support_id == "support:SH-140")
            .unwrap();
        assert_eq!(mapped.family, SupportFamily::Spring);
        let boundary = open_pipe_stress_linear_supports::prepare_boundary(
            built.nodes.len(),
            &[mapped.clone()],
        );
        assert!(boundary.findings.is_empty());
        assert!(boundary.restrained_dofs.is_empty());
        assert_eq!(boundary.springs.len(), 1);
        assert_eq!(boundary.springs[0].stiffness.value, expected);
        assert_eq!(
            run_linear_static_preview(request).status.mechanics,
            "MECHANICS_SOLVED"
        );
    }

    #[test]
    fn f3_real_hanger_nonlinear_and_dof_aliases_remain_supported() {
        for behavior in [
            "one_way", "one-way", "oneway", "lift_off", "lift-off", "liftoff",
        ] {
            let mut request = mechanical_fixture_for_test(
                request(),
                "tests::f3_real_hanger_nonlinear_and_dof_aliases_remain_supported",
            );
            let hanger = request
                .model
                .supports
                .iter_mut()
                .find(|s| s.id == "support:SH-140")
                .unwrap();
            hanger.family = Some("spring_hanger".to_string());
            let input = hanger.hanger.as_mut().unwrap();
            input.hanger_type = Some(" spring_hanger ".to_string());
            input.stiffness.as_mut().unwrap().dof = "uz".to_string();
            let nonlinear = request
                .model
                .supports
                .iter_mut()
                .find(|s| s.id == "support:NL-140")
                .unwrap()
                .nonlinear
                .as_mut()
                .unwrap();
            nonlinear.behavior = behavior.to_string();
            nonlinear.dof = "Uy".to_string();
            if behavior.starts_with("lift") {
                // Preserve the fixture's explicit reaction sense for contact.
                nonlinear.contact_when = nonlinear.active_when.take();
            }
            let result = run_linear_static_preview(request);
            assert_eq!(
                result.status.mechanics, "MECHANICS_SOLVED",
                "{behavior}: {:?}",
                result.diagnostics
            );
            assert!(result
                .results
                .iter()
                .any(|r| r.id == "result:spring-hanger:support-SH-140:stiffness"));
        }
    }

    #[test]
    fn explicit_anchor_rx_only_preserves_partial_rotation_through_boundary() {
        let support = rigid_preview_support("anchor", &["RX"]);
        let mapped = rigid_linear_support_from_preview(&support, 1, vec![FrameDof::Rx]);
        assert_eq!(mapped.support_id, support.id);
        assert_eq!(mapped.family, SupportFamily::Anchor);
        assert_eq!(mapped.node_index, 1);
        assert_eq!(mapped.restrained_dofs, vec![FrameDof::Rx]);

        let prepared = open_pipe_stress_linear_supports::prepare_boundary(2, &[mapped]);
        assert!(prepared.findings.is_empty());
        assert_eq!(prepared.restrained_dofs, vec![9]);
        assert!(prepared.springs.is_empty());
        assert!(prepared.imposed_displacements.is_empty());
    }

    #[test]
    fn explicit_anchor_mixed_dofs_leave_uy_spring_and_other_dofs_free() {
        let support = rigid_preview_support("anchor", &["UX", "RZ"]);
        let mapped =
            rigid_linear_support_from_preview(&support, 1, vec![FrameDof::Ux, FrameDof::Rz]);
        assert_eq!(mapped.family, SupportFamily::Anchor);
        assert_eq!(mapped.node_index, 1);
        assert_eq!(mapped.restrained_dofs, vec![FrameDof::Ux, FrameDof::Rz]);

        // Synthetic mechanics witness, not a default or equipment stiffness recommendation.
        let stiffness = SupportQuantity::positive(
            7.0,
            open_pipe_stress_linear_supports::QuantityDimension::TranslationalStiffness,
        )
        .unwrap();
        let spring = LinearSupport::spring("support:invented-uy", 1, FrameDof::Uy, Some(stiffness));
        let mut matrix = vec![vec![0.0; 12]; 12];
        for (index, row) in matrix.iter_mut().enumerate() {
            row[index] = 2.0;
        }
        let applied = open_pipe_stress_linear_supports::apply_linear_supports(
            &matrix,
            &vec![0.0; 12],
            &[mapped, spring],
        )
        .unwrap();
        let prepared = &applied.prepared_boundary;
        assert!(prepared.findings.is_empty());
        assert_eq!(prepared.restrained_dofs, vec![6, 11]);
        assert!(prepared.imposed_displacements.is_empty());
        assert_eq!(prepared.springs.len(), 1);
        assert_eq!(prepared.springs[0].node_dof.global_index(), 7);
        assert_eq!(prepared.springs[0].stiffness.value, 7.0);
        assert_eq!(applied.prescribed_dofs, vec![6, 11]);
        assert_eq!(applied.prescribed_displacements, vec![0.0, 0.0]);
        assert_eq!(
            applied.reduced_system.free_dofs,
            vec![0, 1, 2, 3, 4, 5, 7, 8, 9, 10]
        );
        matrix[7][7] += 7.0;
        assert_eq!(applied.stiffness_with_springs, matrix);
    }

    #[test]
    fn explicit_anchor_all_six_preserves_complete_restraint() {
        let support = rigid_preview_support("anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"]);
        let dofs = vec![
            FrameDof::Ux,
            FrameDof::Uy,
            FrameDof::Uz,
            FrameDof::Rx,
            FrameDof::Ry,
            FrameDof::Rz,
        ];
        let mapped = rigid_linear_support_from_preview(&support, 1, dofs.clone());
        assert_eq!(mapped.family, SupportFamily::Anchor);
        assert_eq!(mapped.node_index, 1);
        assert_eq!(mapped.restrained_dofs, dofs);
        let prepared = open_pipe_stress_linear_supports::prepare_boundary(2, &[mapped]);
        assert!(prepared.findings.is_empty());
        assert_eq!(prepared.restrained_dofs, vec![6, 7, 8, 9, 10, 11]);
        assert!(prepared.springs.is_empty());
    }

    #[test]
    fn explicit_anchor_repair_preserves_named_and_implicit_family_fallbacks() {
        for (family, dofs, expected_family, indices) in [
            (
                Some("guide"),
                vec![FrameDof::Uy, FrameDof::Uz],
                SupportFamily::Guide,
                vec![7, 8],
            ),
            (
                Some("line_stop"),
                vec![FrameDof::Ux],
                SupportFamily::LineStop,
                vec![6],
            ),
            (
                Some("vertical_support"),
                vec![FrameDof::Uz],
                SupportFamily::VerticalSupport,
                vec![8],
            ),
            (None, vec![FrameDof::Uy], SupportFamily::Guide, vec![7]),
            (
                None,
                vec![
                    FrameDof::Ux,
                    FrameDof::Uy,
                    FrameDof::Uz,
                    FrameDof::Rx,
                    FrameDof::Ry,
                    FrameDof::Rz,
                ],
                SupportFamily::Anchor,
                vec![6, 7, 8, 9, 10, 11],
            ),
        ] {
            let mut support = rigid_preview_support("guide", &[]);
            support.family = family.map(str::to_string);
            let mapped = rigid_linear_support_from_preview(&support, 1, dofs.clone());
            assert_eq!(mapped.family, expected_family);
            assert_eq!(mapped.node_index, 1);
            assert_eq!(mapped.restrained_dofs, dofs);
            let prepared = open_pipe_stress_linear_supports::prepare_boundary(2, &[mapped]);
            assert!(prepared.findings.is_empty());
            assert_eq!(prepared.restrained_dofs, indices);
            assert!(prepared.springs.is_empty());
        }
    }

    #[test]
    fn named_line_stop_reaches_solver_family_with_entered_dof() {
        let support = rigid_preview_support("line_stop", &["UX"]);
        let mapped = rigid_linear_support_from_preview(&support, 2, vec![FrameDof::Ux]);

        assert_eq!(mapped.family, SupportFamily::LineStop);
        assert_eq!(mapped.node_index, 2);
        assert_eq!(mapped.restrained_dofs, vec![FrameDof::Ux]);
    }

    #[test]
    fn named_vertical_support_reaches_solver_family_with_entered_dof() {
        let support = rigid_preview_support("vertical_support", &["UZ"]);
        let mapped = rigid_linear_support_from_preview(&support, 3, vec![FrameDof::Uz]);

        assert_eq!(mapped.family, SupportFamily::VerticalSupport);
        assert_eq!(mapped.node_index, 3);
        assert_eq!(mapped.restrained_dofs, vec![FrameDof::Uz]);
    }

    #[test]
    fn guide_preview_mapping_remains_guide_with_entered_dofs() {
        let support = rigid_preview_support("guide", &["UY", "UZ"]);
        let mapped =
            rigid_linear_support_from_preview(&support, 4, vec![FrameDof::Uy, FrameDof::Uz]);

        assert_eq!(mapped.family, SupportFamily::Guide);
        assert_eq!(mapped.node_index, 4);
        assert_eq!(mapped.restrained_dofs, vec![FrameDof::Uy, FrameDof::Uz]);
    }

    fn request() -> LinearStaticPreviewRequest {
        serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/invented_preview_model.json"
        ))
        .map(|mut model: PreviewModel| {
            // Every route refuses the demo's legacy joint C-150
            // (LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED, D-4). This shared test
            // basis omits it; the refusal assertions use
            // `request_with_refused_joint()`.
            model.components.retain(|component| component.id != "component:C-150");
            LinearStaticPreviewRequest {
                model,
                materials: invented_materials(),
            }
        })
        .unwrap()
    }

    /// The unchanged invented demo, including its realized joint C-150.
    fn request_with_refused_joint() -> LinearStaticPreviewRequest {
        serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/invented_preview_model.json"
        ))
        .map(|model| LinearStaticPreviewRequest {
            model,
            materials: invented_materials(),
        })
        .unwrap()
    }

    fn shared_section_request() -> LinearStaticPreviewRequest {
        let mut input = request();
        let pipe = &mut input.model.pipe_segments[0];
        pipe.section_ref = Some("section:test".into());
        input.model.sections.push(PreviewSection {
            id: "section:test".into(),
            name: "Explicit shared pipe".into(),
            section_type: "pipe".into(),
            properties: BTreeMap::from([
                (
                    "outside_diameter".into(),
                    pipe.section.outside_diameter.clone(),
                ),
                ("wall_thickness".into(), pipe.section.wall_thickness.clone()),
            ]),
            provenance: serde_json::json!({"source": "invented test"}),
        });
        input
    }

    #[test]
    fn shared_section_matching_reference_resolves_and_preserves_local_inputs() {
        let mut input = shared_section_request();
        input.model.pipe_segments[0].section.material_density = Some(Quantity {
            value: 7800.0,
            unit: "kg/m3".into(),
        });
        input.model.pipe_segments[0].section.mill_tolerance = Some(Quantity {
            value: 0.0001,
            unit: "m".into(),
        });
        let unbound = input.model.pipe_segments[1].section.outside_diameter.value;
        let mut diagnostics = Vec::new();
        resolve_shared_sections(&mut input.model, &mut diagnostics);
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        assert_eq!(
            input.model.pipe_segments[0]
                .section
                .material_density
                .as_ref()
                .unwrap()
                .value,
            7800.0
        );
        assert_eq!(
            input.model.pipe_segments[0]
                .section
                .mill_tolerance
                .as_ref()
                .unwrap()
                .value,
            0.0001
        );
        assert_eq!(
            input.model.pipe_segments[1].section.outside_diameter.value,
            unbound
        );
        let output = run_linear_static_preview(input);
        assert!(!output
            .diagnostics
            .iter()
            .any(|d| d.code.starts_with("SECTION_REFERENCE")));
    }

    #[test]
    fn shared_section_stale_cache_blocks_common_solver_entry() {
        let mut input = mechanical_fixture_for_test(
            shared_section_request(),
            "tests::shared_section_stale_cache_blocks_common_solver_entry",
        );
        input.model.pipe_segments[0].section.outside_diameter.value *= 1.1;
        let output = run_linear_static_preview(input);
        assert!(output
            .diagnostics
            .iter()
            .any(|d| d.code == "SECTION_REFERENCE_CACHE_STALE" && d.severity == "blocking"));
    }

    #[test]
    fn shared_section_equivalent_but_different_cache_representation_is_stale() {
        let mut input = mechanical_fixture_for_test(
            shared_section_request(),
            "tests::shared_section_equivalent_but_different_cache_representation_is_stale",
        );
        let pipe = &mut input.model.pipe_segments[0];
        assert_eq!(pipe.section.outside_diameter.unit, "m");
        pipe.section.outside_diameter.value *= 1000.0;
        pipe.section.outside_diameter.unit = "mm".into();
        let output = run_linear_static_preview(input);
        assert!(output
            .diagnostics
            .iter()
            .any(|d| d.code == "SECTION_REFERENCE_CACHE_STALE"));
    }

    #[test]
    fn shared_section_blank_reference_identity_blocks_common_entry() {
        for blank in ["", " ", "\t\n"] {
            let mut input = mechanical_fixture_for_test(
                shared_section_request(),
                "tests::shared_section_blank_reference_identity_blocks_common_entry",
            );
            input.model.pipe_segments[0].section_ref = Some(blank.into());
            input.model.sections[0].id = blank.into();
            let output = run_linear_static_preview(input);
            assert!(output
                .diagnostics
                .iter()
                .any(|d| d.code == "SECTION_REFERENCE_INVALID" && d.severity == "blocking"));
        }
        // Nonblank references remain exact; whitespace is never silently removed.
        let mut input = mechanical_fixture_for_test(
            shared_section_request(),
            "tests::shared_section_blank_reference_identity_blocks_common_entry",
        );
        input.model.pipe_segments[0].section_ref = Some(" section:test ".into());
        let output = run_linear_static_preview(input);
        assert!(output
            .diagnostics
            .iter()
            .any(|d| d.code == "SECTION_REFERENCE_INVALID"));
    }

    #[test]
    fn shared_section_reference_failures_are_explicit() {
        for case in [
            "missing",
            "duplicate",
            "nonpipe",
            "unsupported",
            "absent_dimension",
            "invalid_dimension",
        ] {
            let mut input = mechanical_fixture_for_test(
                shared_section_request(),
                "tests::shared_section_reference_failures_are_explicit",
            );
            match case {
                "missing" => input.model.sections.clear(),
                "duplicate" => input.model.sections.push(input.model.sections[0].clone()),
                "nonpipe" => input.model.sections[0].section_type = "rigid".into(),
                "unsupported" => {
                    input.model.sections[0].properties.insert(
                        "area".into(),
                        Quantity {
                            value: 1.0,
                            unit: "m2".into(),
                        },
                    );
                }
                "absent_dimension" => {
                    input.model.sections[0].properties.remove("wall_thickness");
                }
                _ => {
                    input.model.sections[0]
                        .properties
                        .get_mut("wall_thickness")
                        .unwrap()
                        .value = -1.0
                }
            }
            let output = run_linear_static_preview(input);
            assert!(
                output.diagnostics.iter().any(|d| d.severity == "blocking"
                    && (d.code.starts_with("SECTION_") || d.code == "PIPE_DIMENSION_INVALID")),
                "{case}"
            );
        }
    }

    #[test]
    fn shared_section_local_mill_tolerance_must_leave_positive_wall() {
        let mut input = mechanical_fixture_for_test(
            shared_section_request(),
            "tests::shared_section_local_mill_tolerance_must_leave_positive_wall",
        );
        let wall = input.model.pipe_segments[0].section.wall_thickness.clone();
        input.model.pipe_segments[0].section.mill_tolerance = Some(wall);
        let output = run_linear_static_preview(input);
        assert!(output
            .diagnostics
            .iter()
            .any(|d| d.code == "PIPE_DIMENSION_INVALID"
                && d.affected_refs.contains(&"mill_tolerance".to_string())));
    }

    #[test]
    fn shared_section_wire_is_optional_and_retained() {
        assert!(request().model.sections.is_empty());
        assert!(request()
            .model
            .pipe_segments
            .iter()
            .all(|p| p.section_ref.is_none()));
        let mut wire: serde_json::Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/invented_preview_model.json"
        ))
        .unwrap();
        wire["pipe_segments"][0]["section_ref"] = serde_json::json!("section:test");
        wire["sections"] = serde_json::json!([{"id":"section:test", "name":"test", "section_type":"pipe", "properties": {}, "provenance":"invented"}]);
        let model: PreviewModel = serde_json::from_value(wire).unwrap();
        assert_eq!(
            model.pipe_segments[0].section_ref.as_deref(),
            Some("section:test")
        );
        assert_eq!(model.sections.len(), 1);
    }

    fn invented_materials() -> Vec<MaterialInput> {
        vec![MaterialInput {
            constitutive_basis: None,
            poisson_ratio: None,
            id: "material:invented-carbon-steel".to_string(),
            elastic_modulus: Quantity {
                value: 200_000_000_000.0,
                unit: "Pa".to_string(),
            },
            shear_modulus: Some(Quantity {
                value: 77_000_000_000.0,
                unit: "Pa".to_string(),
            }),
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

    /// T0R: `reaction_resultant` is retired on preview-physics-1. Its legacy
    /// id `…reaction:<support-suffix>` maps to the v2 force magnitude, which is
    /// the same force norm computed from the signed components: the first case
    /// for `result:reaction:…`, else the case or combination named in the prefix.
    fn support_force_norm(envelope: &MechanicsEnvelope, legacy_id: &str) -> f64 {
        let (prefix, support) = legacy_id.rsplit_once("reaction:").expect("legacy reaction id");
        let scope = prefix
            .strip_prefix("result:loadcase:")
            .or_else(|| prefix.strip_prefix("result:combination:"))
            .map(|s| s.trim_end_matches(':'));
        envelope
            .results
            .iter()
            .find(|row| {
                row.kind == "support_reaction_force_magnitude_v2"
                    && stable_suffix(&row.entity_ref) == support
                    && scope.is_none_or(|scope| row.basis_ref.as_ref().is_some_and(|b| stable_suffix(&b.ref_id) == scope))
            })
            .unwrap_or_else(|| panic!("missing signed support force magnitude for {legacy_id}"))
            .value
    }

    /// T0R: `open_formula_stress_summary` is retired; the member's certified
    /// circular maximum (first case, Pa) is its successor, here in MPa.
    fn member_maximum_mpa(envelope: &MechanicsEnvelope, pipe_id: &str) -> f64 {
        envelope
            .results
            .iter()
            .find(|row| row.kind == "pipe_elastic_normal_stress_maximum_v2" && row.entity_ref == pipe_id)
            .unwrap_or_else(|| panic!("missing maximum for {pipe_id}"))
            .value
            / 1e6
    }

    fn has_member_maximum(envelope: &MechanicsEnvelope, pipe_id: &str) -> bool {
        envelope
            .results
            .iter()
            .any(|row| row.kind == "pipe_elastic_normal_stress_maximum_v2" && row.entity_ref == pipe_id)
    }

    fn legacy_value(envelope: &MechanicsEnvelope, id: &str) -> f64 {
        if id.contains("reaction:support-") {
            support_force_norm(envelope, id)
        } else {
            result_value(envelope, id)
        }
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
        let result = run_linear_static_preview(mechanical_fixture_for_test(
            request(),
            "tests::valid_invented_model_solves_deterministically",
        ));
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
            .contains("solution_basis=sparse_structural_integrity_primary"));
        assert!(metadata
            .basis
            .contains("profile_pivot_residual_observation_basis=legacy_unscaled_DEC050_DEC053"));
        assert!(metadata
            .basis
            .contains("default_sparse_promotion=interactive_default"));
        assert!(metadata.basis.contains("dense_scrutiny_available=true"));
    }

    #[test]
    fn dense_scrutiny_mode_keeps_sparse_parity_row() {
        let result = run_linear_static_preview_with_mode(
            mechanical_fixture_for_test(
                request(),
                "tests::dense_scrutiny_mode_keeps_sparse_parity_row",
            ),
            PreviewSolverMode::DenseScrutiny,
        );
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
    fn successor_mixed_product_evaluates_actual_final_rows_in_both_modes() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            let output =
                run_linear_static_preview_with_mode(mixed_nonlinear_preview_request(), mode);
            assert_eq!(
                output.status.mechanics, "MECHANICS_SOLVED",
                "{:?}",
                output.diagnostics
            );
            assert_eq!(
                output.numerical_quality.status,
                NumericalQualityStatus::Unresolved
            );
            assert_eq!(
                output.numerical_quality.cases[0].solve_quality,
                NumericalQualityStatus::Unresolved
            );
            let evidence = output
                .diagnostics
                .iter()
                .find(|d| d.id == output.numerical_quality.cases[0].evidence_refs[0])
                .unwrap();
            assert_eq!(
                evidence.code,
                "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED"
            );
            assert_eq!(
                evidence.id,
                integrity_diagnostic_id("load:L-MIXED-NONLINEAR")
            );
            assert!(evidence
                .message
                .contains(open_pipe_stress_nonlinear_integration::product_equilibrium::POLICY));
            assert!(evidence.message.contains("normalized_evaluation_allowance"));
            assert!(evidence.message.contains("observed_governing_work_dof"));
            assert!(evidence
                .message
                .contains("separate zero count/cap/contact/sliding checks passed"));
            for metric in ["force", "moment", "work"] {
                let id = format!("result:nonlinear-support:free-dof-{metric}-residual");
                let row = output.results.iter().find(|row| row.id == id).unwrap();
                assert!(row.value.is_finite());
                let basis = &row.metadata.as_ref().unwrap().basis;
                assert!(basis.contains("observed_compliance=true"));
                assert!(basis
                    .contains(open_pipe_stress_nonlinear_integration::product_equilibrium::POLICY));
                assert!(
                    !basis.contains("threshold_policy_ref=DEC-046-CV-B-product-preview-free-dof")
                );
            }
        }
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
        assert_eq!(
            result.numerical_quality.status,
            NumericalQualityStatus::Unresolved
        );
        assert!(result.diagnostics.iter().any(|d| d.code
            == "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED"
            && d.id == integrity_diagnostic_id("load:L-MIXED-NONLINEAR")));
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
        assert_coulomb_force_balance_n(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-MIX-FRIC-110:uz-reaction",
            ),
            -0.3 * 10.0,
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
        assert!(
            result_value(&result, "result:nonlinear-support:free-dof-force-residual").is_finite()
        );
        assert!(
            result_value(&result, "result:nonlinear-support:free-dof-moment-residual").is_finite()
        );
        assert!(
            result_value(&result, "result:nonlinear-support:free-dof-work-residual").is_finite()
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
            assert!(
                basis.contains(open_pipe_stress_nonlinear_integration::product_equilibrium::POLICY)
            );
            assert!(basis.contains("observed_compliance=true"));
            assert!(basis.contains("target=64*gamma(actual_row_operation_count)"));
            assert!(basis.contains("threshold_policy_status=accepted"));
            assert!(basis.contains("residual_basis=free_dof_force_moment_equilibrium"));
            assert!(!basis.contains("threshold=TBD"));
        }
        let work_basis = &work_residual.metadata.as_ref().unwrap().basis;
        assert!(work_basis.contains(DEC_046_PRODUCT_PREVIEW_FREE_DOF_WORK_POLICY_REF));
        assert!(work_basis.contains(DEC_046_PRODUCT_PREVIEW_GENERAL_ENERGY_POLICY_REF));
        assert!(work_basis.contains("threshold_policy_status=accepted"));
        assert!(work_basis.contains("residual_basis=free_dof_work_residual"));
        assert!(work_basis
            .contains(open_pipe_stress_nonlinear_integration::product_equilibrium::POLICY));
        assert!(work_basis.contains("observed_compliance=true"));
        assert!(work_basis.contains("general_energy_alias=residual_work_not_total_energy_balance"));
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

    // CONTACT_REPAIR_V2_EXPECTATIONS_BEGIN
    #[test]
    fn contact_recovery_failed_sparse_and_dense_attempt_emits_no_success_fallback() {
        let mut input = gap_closure_preview_request();
        input.model.supports[0].restraints.retain(|dof| dof != "UX");
        input.model.supports[1].nonlinear = None;
        input.model.supports[1].family = Some("guide".into());
        input.model.supports[1].restraints = vec!["UY".into()];
        let result =
            run_linear_static_preview_with_mode(input, PreviewSolverMode::SparseInteractive);
        assert_ne!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(result.diagnostics.iter().any(|d| d.severity == "blocking"
            && d.code == "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM"
            && d.id == integrity_diagnostic_id("load:L-GAP")
            && d.affected_refs.iter().any(|r| r == "load:L-GAP")));
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.code == "SPARSE_INTERACTIVE_DENSE_FALLBACK"),
            "{:?}",
            result.diagnostics
        );
        assert!(result.results.is_empty());
    }

    #[test]
    fn contact_recovery_product_selected_tip_matches_axial_oracle_and_reverse_blocks() {
        // Existing invented fixture: L=1 m, E=200e9 Pa, OD=.168 m,
        // wall=.007 m, A=pi/4*(OD^2-ID^2), F=100000 N, g=.05 mm.
        // Selected tip ux=(g+F*L/(E*A))*1000 mm; no output-derived expectation.
        let area = std::f64::consts::PI / 4.0 * (0.168_f64.powi(2) - 0.154_f64.powi(2));
        let expected_tip_mm = 0.05 + 100000.0 / (200e9 * area) * 1000.0;
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            for seed in ["active", "inactive"] {
                let mut input = gap_closure_preview_request();
                input.model.supports[0].restraints.retain(|dof| dof != "UX");
                input.model.supports[1].node = "node:N-100".into();
                input.model.supports[1]
                    .nonlinear
                    .as_mut()
                    .unwrap()
                    .initial_state = Some(seed.into());
                let solved = run_linear_static_preview_with_mode(input.clone(), mode);
                assert_eq!(solved.status.mechanics, "MECHANICS_SOLVED");
                assert!(
                    (result_value(&solved, "result:disp:node-N-110:ux") - expected_tip_mm).abs()
                        <= 0.5e-6 + 1e-10
                );
                assert!(
                    (result_value(&solved, "result:disp:node-N-100:ux") - 0.05).abs()
                        <= 0.5e-6 + 1e-10
                );
                input.model.load_cases[0].primitive_loads[0].magnitude.value = -100000.0;
                let reversed = run_linear_static_preview_with_mode(input, mode);
                assert_ne!(reversed.status.mechanics, "MECHANICS_SOLVED");
                assert!(reversed.results.is_empty());
                assert!(!reversed
                    .diagnostics
                    .iter()
                    .any(|d| d.code == "SPARSE_INTERACTIVE_DENSE_FALLBACK"));
                assert!(reversed.diagnostics.iter().any(|d| d.severity == "blocking"
                    && d.code == "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM"
                    && d.id == integrity_diagnostic_id("load:L-GAP")
                    && d.affected_refs.iter().any(|r| r == "load:L-GAP")));
            }
        }
    }
    // CONTACT_REPAIR_V2_EXPECTATIONS_END

    // CONTACT_CLASS_FROZEN_BEGIN
    #[test]
    fn contact_recovery_product_insufficient_or_invalid_potential_restraint_stays_blocked() {
        for case in 0..5 {
            let mut input = gap_closure_preview_request();
            input.model.supports[0].restraints.retain(|dof| dof != "UX");
            input.model.supports[1].node = "node:N-100".to_string();
            match case {
                0 => input.model.supports[0].restraints.retain(|dof| dof != "UY"),
                1 => input.model.supports[1].nonlinear.as_mut().unwrap().dof = "RX".to_string(),
                2 => {
                    input.model.supports[1]
                        .nonlinear
                        .as_mut()
                        .unwrap()
                        .initial_state = Some("sticking".to_string())
                }
                3 => {
                    input.model.supports[1]
                        .nonlinear
                        .as_mut()
                        .unwrap()
                        .gap
                        .as_mut()
                        .unwrap()
                        .value = -1.0
                }
                _ => input.model.supports[1].nonlinear.as_mut().unwrap().dof = "UY".to_string(),
            }
            let result = run_linear_static_preview(input);
            assert_ne!(result.status.mechanics, "MECHANICS_SOLVED");
            assert!(result.results.is_empty());
            assert!(result.diagnostics.iter().any(|d| d.severity == "blocking"));
        }
    }
    // CONTACT_CLASS_FROZEN_END

    // CONTACT_RECOVERY_FROZEN_BEGIN
    #[test]
    fn contact_recovery_product_crosses_preflight_and_absent_linear_solution() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            for seed in ["active", "inactive"] {
                let mut input = gap_closure_preview_request();
                input.model.supports[0].restraints.retain(|dof| dof != "UX");
                input.model.supports[1].node = "node:N-100".to_string();
                input.model.supports[1]
                    .nonlinear
                    .as_mut()
                    .unwrap()
                    .initial_state = Some(seed.to_string());
                let result = run_linear_static_preview_with_mode(input, mode);
                assert_eq!(
                    result.status.mechanics, "MECHANICS_SOLVED",
                    "{:?}",
                    result.diagnostics
                );
                assert!(
                    (result_value(
                        &result,
                        "result:nonlinear-support:support-NL-GAP-110:ux-displacement"
                    ) - 0.05)
                        .abs()
                        <= 0.5e-6 + 1e-10
                );
                assert!(
                    (result_value(
                        &result,
                        "result:nonlinear-support:support-NL-GAP-110:ux-reaction"
                    ) + 100000.0)
                        .abs()
                        <= 0.5e-6 + 1e-10
                );
                assert!(result
                    .results
                    .iter()
                    .all(|r| r.kind != "linear_solver_mode_basis"
                        && r.kind != "sparse_live_path_dense_parity_relative_delta"
                        && r.id != "result:sparse-live:dense-parity-relative-delta"));
                assert!(!result
                    .diagnostics
                    .iter()
                    .any(|d| d.code == "SPARSE_INTERACTIVE_DENSE_FALLBACK"));
                if seed == "inactive" {
                    assert!(result
                        .diagnostics
                        .iter()
                        .any(|d| d.severity == "warning" && d.message.contains("all-active")));
                }
            }
        }
    }
    // CONTACT_RECOVERY_FROZEN_END

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
    fn unit_contract_rotational_gap_cannot_consume_a_length_as_an_angle() {
        // The current product gap contract is length-based. Rotational gap
        // capability needs an explicit angular contract; neither metre input
        // nor a rejected angular input may silently become an angular stop.
        let mut missing_blocks = Vec::new();
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            for (dof, component) in [
                ("rotation_x", "rx"),
                ("RX", "rx"),
                ("rotation_y", "ry"),
                ("RY", "ry"),
                ("rotation_z", "rz"),
                ("RZ", "rz"),
            ] {
                for (value, unit) in [(0.05, "mm"), (0.00005, "m"), (0.00005, "rad")] {
                    let mut input = gap_closure_preview_request();
                    let nonlinear = input.model.supports[1].nonlinear.as_mut().unwrap();
                    nonlinear.dof = dof.into();
                    nonlinear.gap = Some(Quantity {
                        value,
                        unit: unit.into(),
                    });
                    let load = &mut input.model.load_cases[0].primitive_loads[0];
                    load.category = "concentrated_moment".into();
                    load.direction = dof.into();
                    load.dimension = "moment".into();
                    load.magnitude = Quantity {
                        value: 250.0,
                        unit: "N*m".into(),
                    };
                    let output = run_linear_static_preview_with_mode(input, mode);
                    let rotation_id = format!("result:disp:node-N-110:{component}");
                    let observed_rotation = output
                        .results
                        .iter()
                        .find(|row| row.id == rotation_id)
                        .map(|row| (row.value, row.unit.as_str()));
                    let codes = output
                        .diagnostics
                        .iter()
                        .map(|d| d.code.as_str())
                        .collect::<Vec<_>>();
                    println!("rotational-gap unit contract: mode={mode:?}, dof={dof}, gap={value} {unit}, mechanics={}, rotation={observed_rotation:?}, diagnostics={codes:?}", output.status.mechanics);
                    // rotation_* is an authored moment-load spelling, not a
                    // supported nonlinear-support DOF token in this contract.
                    let required_code = if dof.starts_with("rotation_") {
                        if unit == "rad" {
                            "UNIT_CONVERSION_UNAVAILABLE"
                        } else {
                            "NONLINEAR_SUPPORT_DOF_INVALID"
                        }
                    } else {
                        "NONLINEAR_ROTATIONAL_GAP_UNSUPPORTED"
                    };
                    let blocked = output
                        .diagnostics
                        .iter()
                        .any(|d| d.code == required_code && d.severity == "blocking");
                    if !blocked || output.status.mechanics == "MECHANICS_SOLVED" {
                        missing_blocks.push(format!("{mode:?}/{dof}/{unit}"));
                    }
                    assert!(!output.accepted_model_state_mutated);
                }
            }
        }
        assert!(
            missing_blocks.is_empty(),
            "rotational gap contract was not explicitly blocked: {missing_blocks:?}"
        );
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

        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::operation_authored_primitive_categories_map_to_preview_mechanics",
        );
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

        let native = run_linear_static_preview(mechanical_fixture_for_test(
            self::request(),
            "tests::operation_authored_primitive_categories_map_to_preview_mechanics",
        ));
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
        let result = run_linear_static_preview(mechanical_fixture_for_test(
            request(),
            "tests::valid_invented_model_exposes_element_force_components",
        ));
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
                            && metadata.basis == "recovered_from_local_element_stiffness"
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
                            && metadata.basis == "recovered_from_local_element_stiffness"
                    })
                    .unwrap_or(false)
        }));
    }

    #[test]
    fn valid_invented_model_exposes_global_displacement_components() {
        // T0R (B-1): the fixture's nonlinear supports gate every mechanics
        // combination; its linear companion keeps the combination algebra tested.
        let mut input = mechanical_fixture_for_test(request(), "tests::valid_invented_model_exposes_global_displacement_components");
        input.model.supports.retain(|support| support.nonlinear.is_none());
        let result = run_linear_static_preview(input);
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
        // the existing magnitude comparison tolerance.
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
        assert_eq!(combined_uy.value, default_uy + 0.5 * alternate_uy);
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
        // T0R: the signed support rows take the retired reaction rows' place.
        assert!(index_of("result:disp:node-N-140:rz") < index_of("result:support-action:10:load:L-100:13:support:S-100:Fx"));
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
    fn bend_component_user_multipliers_emit_stress_review_rows() {
        // T0R (SF-8/M08): the SIF×k review row is retired; a geometry-only bend
        // marker publishes the equal-factor measure i*hypot(My,Mz)/Z per case.
        let result = run_linear_static_preview(mechanical_fixture_for_test(
            request(),
            "tests::bend_component_user_multipliers_emit_stress_review_rows",
        ));
        assert!(!result.results.iter().any(|item| item.kind == "component_user_stress_multiplier_review"));
        let default_row = result
            .results
            .iter()
            .find(|item| item.id == "result:intensified-bending:component-C-110:pipe-P-100:end-j")
            .expect("bend equal-factor row should be emitted for the adjacent pipe endpoint");
        assert_eq!(default_row.kind, "component_equal_factor_intensified_bending_stress_v1");
        assert_eq!(default_row.entity_ref, "component:C-110");
        assert_eq!(default_row.unit, "Pa");
        assert!(default_row.value > 0.0);
        let refs = &default_row.source_result_refs;
        assert_eq!(refs, &vec!["result:stress:pipe-P-100:end-j:bending-normal-y".to_string(), "result:stress:pipe-P-100:end-j:bending-normal-z".to_string()]);
        // The measure equals i * hypot(sigma_by, sigma_bz) of the same end (MPa rows).
        let hypot = result_value(&result, &refs[0]).hypot(result_value(&result, &refs[1])) * 1e6;
        assert!((default_row.value - 1.15 * hypot).abs() <= 1e-9 * default_row.value);
        let metadata = default_row.metadata.as_ref().expect("metadata");
        assert_eq!(metadata.component, "equal_factor_intensified_bending_stress");
        assert_eq!(metadata.coordinate_system, "pipe_section");
        assert_eq!(metadata.location, "end_j");
        assert_eq!(metadata.basis, "user_sif_times_member_section_bending_stress_v1");
        assert!(metadata.sign_convention.contains("i=1.15 user-entered bend SIF"));
        assert!(metadata.sign_convention.contains("source: invented_user_entered_preview_no_code_table"));
        assert!(metadata.sign_convention.contains("no flexibility factor"));
        // Never combined (the fixture's combination is gated by nonlinear supports anyway).
        assert!(!result.results.iter().any(|item| item.kind == "component_equal_factor_intensified_bending_stress_v1"
            && item.basis_ref.as_ref().is_some_and(|b| b.ref_type == "combination")));
        assert_eq!(
            result.summary.component_stress_modifier_count,
            result.results.iter().filter(|item| item.kind == "component_equal_factor_intensified_bending_stress_v1").count()
        );
        assert_eq!(
            result.diagnostics.iter().filter(|d| d.code == "COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED").count(),
            result.summary.component_stress_modifier_count
        );
        assert!(!result.diagnostics.iter().any(|d| d.code == "COMPONENT_STRESS_MULTIPLIER_APPLIED"));
    }

    #[test]
    fn branch_component_user_multipliers_emit_side_specific_stress_review_rows() {
        // T0R (SF-8/M08): side-specific equal-factor rows replace the SIF×k rows.
        let result = run_linear_static_preview(mechanical_fixture_for_test(
            request(),
            "tests::branch_component_user_multipliers_emit_side_specific_stress_review_rows",
        ));
        let row = |id: &str| result.results.iter().find(|item| item.id == id).unwrap_or_else(|| panic!("missing {id}"));
        let branch_row = row("result:intensified-bending:component-C-120:pipe-P-110:end-j");
        let header_row = row("result:intensified-bending:component-C-120:pipe-P-120:end-i");
        for item in [branch_row, header_row] {
            assert_eq!(item.kind, "component_equal_factor_intensified_bending_stress_v1");
            assert_eq!(item.entity_ref, "component:C-120");
            assert!(item.value > 0.0);
        }
        let branch_metadata = branch_row.metadata.as_ref().unwrap();
        assert_eq!(branch_metadata.location, "end_j");
        assert!(branch_metadata.sign_convention.contains("i=1.31 user-entered branch branch SIF"));
        assert!(branch_metadata.sign_convention.contains("source: invented_user_entered_branch_modifiers_no_code_table"));
        let header_metadata = header_row.metadata.as_ref().unwrap();
        assert_eq!(header_metadata.location, "end_i");
        assert!(header_metadata.sign_convention.contains("i=1.22 user-entered branch header SIF"));
        assert!(!result.results.iter().any(|item| item.kind == "component_equal_factor_intensified_bending_stress_v1"
            && item.basis_ref.as_ref().is_some_and(|b| b.ref_type == "combination")));
    }

    #[test]
    fn realized_user_stiffness_joint_is_refused_on_the_ordinary_route() {
        // T4-U3 (D-4): the legacy four-rate joint is refused by name on every
        // route, neither solved nor converted.
        let mut refused_input = request_with_refused_joint();
        for case in &mut refused_input.model.load_cases {
            // The demo's legacy nonzero pressure is refused too; remove it here.
            case.primitive_loads.retain(|load| load.category != "pressure");
        }
        let refused = run_linear_static_preview(refused_input);
        assert_eq!(refused.status.mechanics, "MODEL_INCOMPLETE");
        assert!(refused.results.is_empty());
        let blocking = refused
            .diagnostics
            .iter()
            .filter(|d| d.severity == "blocking")
            .collect::<Vec<_>>();
        assert_eq!(blocking.len(), 1, "{blocking:?}");
        assert_eq!(blocking[0].code, "LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED");
        assert_eq!(blocking[0].affected_refs, vec!["component:C-150".to_string(), "pipe:P-130".to_string()]);
        for text in ["component:C-150", "pipe:P-130", "neither solved nor converted", "6x6 scaled work matrix", "replaces_span", "hardware", "pressure model"] {
            assert!(blocking[0].message.contains(text), "{text}: {}", blocking[0].message);
        }
    }

    #[test]
    fn flexibility_joint_missing_a_user_stiffness_is_refused_not_dropped() {
        // G11 (I111), T4-U3 (D-4): a legacy joint with any subset of its four
        // rates is refused by the legacy code, never skipped or solved as pipe,
        // and no joint review row is produced.
        let consumed = |output: &MechanicsEnvelope| {
            output
                .results
                .iter()
                .filter(|row| row.kind == "component_user_stiffness_macro_element_review")
                .count()
        };
        let mut observed = Vec::new();
        for missing in ["lateral", "axial", "angular", "torsional"] {
            let mut input = request_with_refused_joint();
            for case in &mut input.model.load_cases {
                // The demo's legacy nonzero pressure is refused first; remove it here.
                case.primitive_loads.retain(|load| load.category != "pressure");
            }
            let joint = input
                .model
                .components
                .iter_mut()
                .find(|component| component.id == "component:C-150")
                .unwrap();
            let modifiers = joint.modifiers.as_mut().unwrap();
            if missing != "lateral" {
                modifiers.lateral_stiffness_user_value.as_mut().unwrap().value = 0.0;
            }
            match missing {
                "lateral" => modifiers.lateral_stiffness_user_value = None,
                "axial" => modifiers.axial_stiffness_user_value = None,
                "angular" => modifiers.angular_stiffness_user_value = None,
                _ => modifiers.torsional_stiffness_user_value = None,
            }
            let output = run_linear_static_preview(input);
            let refused = output.diagnostics.iter().any(|d| {
                d.code == "LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED"
                    && d.severity == "blocking"
                    && d.affected_refs
                        == vec!["component:C-150".to_string(), "pipe:P-130".to_string()]
            });
            observed.push((
                missing,
                output.status.mechanics.clone(),
                consumed(&output),
                output.results.is_empty(),
                refused,
            ));
        }
        let expected = ["lateral", "axial", "angular", "torsional"]
            .map(|missing| (missing, "MODEL_INCOMPLETE".to_string(), 0, true, true))
            .to_vec();
        assert_eq!(observed, expected, "(missing value, status, consumed review rows, no results, refused by name)");
    }

    #[test]
    fn flexibility_joint_with_an_unresolved_mapping_is_refused_not_dropped() {
        // G11, T4-U3 (D-4): a legacy joint whose pipe or node does not resolve
        // is refused by the legacy code, refs [component, pipe] (or [component]
        // when it names no pipe), never skipped.
        let consumed = |output: &MechanicsEnvelope| {
            output
                .results
                .iter()
                .filter(|row| row.kind == "component_user_stiffness_macro_element_review")
                .count()
        };
        let mut observed = Vec::new();
        for case in ["no pipe", "unknown pipe", "unknown node", "node not on the pipe"] {
            let mut input = request_with_refused_joint();
            for load_case in &mut input.model.load_cases {
                // The demo's legacy nonzero pressure is refused first; remove it here.
                load_case.primitive_loads.retain(|load| load.category != "pressure");
            }
            let joint = input
                .model
                .components
                .iter_mut()
                .find(|component| component.id == "component:C-150")
                .unwrap();
            joint
                .modifiers
                .as_mut()
                .unwrap()
                .lateral_stiffness_user_value
                .as_mut()
                .unwrap()
                .value = 0.0;
            let pipe_ref = &mut joint.geometry.as_mut().unwrap().expansion_joint_pipe_ref;
            match case {
                "no pipe" => *pipe_ref = None,
                "unknown pipe" => *pipe_ref = Some("pipe:missing".into()),
                "unknown node" => joint.node = "node:missing".into(),
                _ => joint.node = "node:N-100".into(),
            }
            let output = run_linear_static_preview(input);
            let refusal = output
                .diagnostics
                .iter()
                .find(|d| d.code == "LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED" && d.severity == "blocking")
                .map(|d| d.affected_refs.clone());
            observed.push((
                case,
                output.status.mechanics.clone(),
                consumed(&output),
                output.results.is_empty(),
                refusal,
            ));
        }
        let refs = |pipe: Option<&str>| {
            let mut refs = vec!["component:C-150".to_string()];
            refs.extend(pipe.map(str::to_string));
            Some(refs)
        };
        let expected = vec![
            ("no pipe", "MODEL_INCOMPLETE".to_string(), 0, true, refs(None)),
            ("unknown pipe", "MODEL_INCOMPLETE".to_string(), 0, true, refs(Some("pipe:missing"))),
            ("unknown node", "MODEL_INCOMPLETE".to_string(), 0, true, refs(Some("pipe:P-130"))),
            ("node not on the pipe", "MODEL_INCOMPLETE".to_string(), 0, true, refs(Some("pipe:P-130"))),
        ];
        assert_eq!(observed, expected, "(case, status, consumed review rows, no results, mapping refusal refs)");
    }

    #[test]
    fn flexibility_joint_pipe_without_orientation_or_a_known_end_is_refused_by_the_pipe() {
        // G11, T4-U3 (D-4): a legacy joint whose pipe has no y_reference or an
        // unknown end node cannot drop the joint silently: the legacy refusal
        // runs first, refs [component, pipe].
        for (case, code) in [
            ("no y_reference", "LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED"),
            ("unknown end node", "LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED"),
        ] {
            let mut input = request_with_refused_joint();
            for load_case in &mut input.model.load_cases {
                load_case.primitive_loads.retain(|load| load.category != "pressure");
            }
            let joint = input
                .model
                .components
                .iter_mut()
                .find(|component| component.id == "component:C-150")
                .unwrap();
            let modifiers = joint.modifiers.as_mut().unwrap();
            modifiers.lateral_stiffness_user_value.as_mut().unwrap().value = 0.0;
            let pipe = input
                .model
                .pipe_segments
                .iter_mut()
                .find(|pipe| pipe.id == "pipe:P-130")
                .unwrap();
            match case {
                "no y_reference" => pipe.y_reference = None,
                // The joint sits at P-130's to-node; its from-node becomes unknown.
                _ => pipe.from = "node:missing".into(),
            }
            let output = run_linear_static_preview(input);
            assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE", "{case}");
            assert!(output.results.is_empty(), "{case}");
            assert!(
                output.diagnostics.iter().any(|d| d.code == code
                    && d.severity == "blocking"
                    && d.affected_refs == ["component:C-150", "pipe:P-130"]),
                "{case}: {:?}",
                output.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn spring_hanger_user_inputs_emit_review_rows_without_catalog_defaults() {
        let result = run_linear_static_preview(mechanical_fixture_for_test(
            request(),
            "tests::spring_hanger_user_inputs_emit_review_rows_without_catalog_defaults",
        ));
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
            .contains("consumed by the assembled solve as a constant nodal force"));
        assert!(constant_metadata
            .sign_convention
            .contains("stays review-only with a non-blocking warning"));
        assert!(!constant_metadata
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
        // The fixture's constant-effort support declares no restraints, so it
        // stays review-only under the DEC-049 data-driven opt-in rule and the
        // solve records one non-blocking warning naming the unmet condition.
        let not_consumed = result
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED")
            .expect("non-consuming constant-effort support records a warning");
        assert_eq!(not_consumed.severity, "warning");
        assert!(not_consumed
            .message
            .contains("no translational restraint DOF is declared"));
        assert!(not_consumed
            .affected_refs
            .contains(&"support:CE-120".to_string()));
        assert!(!result
            .results
            .iter()
            .any(|item| item.kind == "constant_effort_support_applied_load"));
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
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

    /// Two-node cantilever along global X (anchor at `node:N-100`, free tip
    /// at `node:N-110`) reusing the invented fixture's pipe section and
    /// material, with the fixture's constant-effort support re-homed to the
    /// tip for DEC-049 assembled-solve consumption tests.
    fn cantilever_constant_effort_request(
        restraints: &[&str],
        include_constant_effort: bool,
        tip_load_newtons: Option<f64>,
    ) -> LinearStaticPreviewRequest {
        let mut request = request();
        let constant_effort_template = request
            .model
            .supports
            .iter()
            .find(|support| support.id == "support:CE-120")
            .expect("fixture carries a constant-effort support")
            .clone();
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
        if include_constant_effort {
            let mut support = constant_effort_template;
            support.id = "support:CE-110".to_string();
            support.node = "node:N-110".to_string();
            support.restraints = restraints.iter().map(|dof| dof.to_string()).collect();
            request.model.supports.push(support);
        }
        request.model.load_cases.truncate(1);
        request.model.combinations.clear();
        request.model.load_cases[0].primitive_loads = match tip_load_newtons {
            Some(value) => vec![PreviewPrimitiveLoad {
                id: "load:L-TIP".to_string(),
                category: "occasional".to_string(),
                target: LoadTargetInput::Node {
                    node: "node:N-110".to_string(),
                },
                direction: "global_y".to_string(),
                magnitude: Quantity {
                    value,
                    unit: "N".to_string(),
                },
                dimension: "force".to_string(),
                provenance: Some("invented_example_user_input".to_string()),
            }],
            None => Vec::new(),
        };
        request
    }

    /// Classical cantilever tip deflection `F L^3 / (3 E I)` in metres for
    /// the fixture pipe section (outside diameter 0.168 m, wall 0.007 m,
    /// length 2.0 m, invented E = 200 GPa).
    fn cantilever_tip_point_load_deflection_m(force_newtons: f64) -> f64 {
        let od: f64 = 0.168;
        let wall: f64 = 0.007;
        let length: f64 = 2.0;
        let elastic_modulus: f64 = 200_000_000_000.0;
        let inner = od - 2.0 * wall;
        let second_moment = PI * (od.powi(4) - inner.powi(4)) / 64.0;
        force_newtons * length.powi(3) / (3.0 * elastic_modulus * second_moment)
    }

    #[test]
    fn constant_effort_consumption_matches_superposition_identity() {
        let tip_load = -350.0;
        let without = run_linear_static_preview(cantilever_constant_effort_request(
            &[],
            false,
            Some(tip_load),
        ));
        let with = run_linear_static_preview(cantilever_constant_effort_request(
            &["UY"],
            true,
            Some(tip_load),
        ));
        assert_eq!(without.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(with.status.mechanics, "MECHANICS_SOLVED");

        // Applied-load evidence row for the consuming support.
        let applied = with
            .results
            .iter()
            .find(|item| item.id == "result:constant-effort-support:support-CE-110:applied-load")
            .expect("consuming constant-effort support emits an applied-load row");
        assert_eq!(applied.kind, "constant_effort_support_applied_load");
        assert_eq!(applied.value, 375.0);
        assert_eq!(applied.unit, "N");
        assert_eq!(applied.entity_ref, "support:CE-110");
        let metadata = applied.metadata.as_ref().expect("applied row metadata");
        assert!(metadata.basis.contains("dec_ref=DEC-049"));
        assert!(metadata
            .basis
            .contains("mechanics_consumption=assembled_solve"));
        assert!(metadata.basis.contains("consumed_dof=UY"));
        assert_eq!(
            metadata.sign_convention,
            CONSTANT_EFFORT_APPLIED_SIGN_CONVENTION
        );
        assert!(!with
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED"));

        // Superposition identity: the solve with the constant-effort support
        // equals the solve without it plus the classical closed-form solution
        // for the equivalent point force at the tip, along +UY.
        let uy_without = result_value(&without, "result:disp:node-N-110:uy");
        let uy_with = result_value(&with, "result:disp:node-N-110:uy");
        let expected_delta_mm = cantilever_tip_point_load_deflection_m(375.0) * 1000.0;
        assert!(expected_delta_mm > 0.0);
        assert!(
            ((uy_with - uy_without) - expected_delta_mm).abs() <= 1.0e-4,
            "superposition identity failed: {uy_with} - {uy_without} != {expected_delta_mm}"
        );
    }

    #[test]
    fn constant_effort_direction_follows_declared_translational_dof_positive_axis() {
        let tip_load = -350.0;
        let with = run_linear_static_preview(cantilever_constant_effort_request(
            &["UZ"],
            true,
            Some(tip_load),
        ));
        assert_eq!(with.status.mechanics, "MECHANICS_SOLVED");
        let applied = with
            .results
            .iter()
            .find(|item| item.id == "result:constant-effort-support:support-CE-110:applied-load")
            .expect("consuming constant-effort support emits an applied-load row");
        let metadata = applied.metadata.as_ref().expect("applied row metadata");
        assert!(metadata.basis.contains("consumed_dof=UZ"));
        assert!(metadata.location.contains("node:N-110:UZ"));
        // No Z-direction primitive load exists, so the tip UZ displacement is
        // exactly the classical point-force deflection along +UZ.
        let uz_with = result_value(&with, "result:disp:node-N-110:uz");
        let expected_mm = cantilever_tip_point_load_deflection_m(375.0) * 1000.0;
        assert!(
            (uz_with - expected_mm).abs() <= 1.0e-4,
            "direction convention failed: {uz_with} != {expected_mm}"
        );
    }

    #[test]
    fn constant_effort_applies_in_every_solved_load_case() {
        let mut request = cantilever_constant_effort_request(&["UY"], true, Some(-350.0));
        let mut second_case = request.model.load_cases[0].clone();
        second_case.id = "load:L-200".to_string();
        second_case.primitive_loads[0].id = "load:L-TIP-ALT".to_string();
        second_case.primitive_loads[0].magnitude = Quantity {
            value: -150.0,
            unit: "N".to_string(),
        };
        request.model.load_cases.push(second_case);
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");

        let base = result
            .results
            .iter()
            .find(|item| item.id == "result:constant-effort-support:support-CE-110:applied-load")
            .expect("default load case applied-load row");
        let alternate = result
            .results
            .iter()
            .find(|item| {
                item.id
                    == "result:loadcase:load-L-200:constant-effort-support:support-CE-110:applied-load"
            })
            .expect("second load case applied-load row");
        assert_eq!(base.value, 375.0);
        assert_eq!(alternate.value, 375.0);
        assert_eq!(
            base.basis_ref.as_ref().map(|basis| basis.ref_id.as_str()),
            Some("load:L-100")
        );
        assert_eq!(
            alternate
                .basis_ref
                .as_ref()
                .map(|basis| basis.ref_id.as_str()),
            Some("load:L-200")
        );
        // The constant force enters both load cases identically: each case's
        // tip UY displacement carries the same +F L^3/(3EI) contribution.
        let delta_mm = cantilever_tip_point_load_deflection_m(375.0) * 1000.0;
        let case_one = result_value(&result, "result:disp:node-N-110:uy")
            - cantilever_tip_point_load_deflection_m(-350.0) * 1000.0;
        let case_two = result_value(&result, "result:loadcase:load-L-200:disp:node-N-110:uy")
            - cantilever_tip_point_load_deflection_m(-150.0) * 1000.0;
        assert!((case_one - delta_mm).abs() <= 1.0e-4);
        assert!((case_two - delta_mm).abs() <= 1.0e-4);
    }

    #[test]
    fn constant_effort_non_consuming_shapes_warn_without_force_or_blocking() {
        let tip_load = -350.0;
        let baseline = run_linear_static_preview(cantilever_constant_effort_request(
            &[],
            false,
            Some(tip_load),
        ));
        assert_eq!(baseline.status.mechanics, "MECHANICS_SOLVED");

        let cases: &[(&[&str], &str)] = &[
            (&[], "no translational restraint DOF is declared"),
            (&["RX"], "no translational restraint DOF is declared"),
            (&["UY", "UZ"], "2 translational restraint DOFs are declared"),
            (
                &["UQ"],
                "declared restraint DOF UQ is not a recognized frame DOF",
            ),
        ];
        for (restraints, expected_condition) in cases {
            let result = run_linear_static_preview(cantilever_constant_effort_request(
                restraints,
                true,
                Some(tip_load),
            ));
            assert_eq!(
                result.status.mechanics, "MECHANICS_SOLVED",
                "non-consuming shape {restraints:?} must not block"
            );
            let warning = result
                .diagnostics
                .iter()
                .find(|diagnostic| diagnostic.code == "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED")
                .unwrap_or_else(|| panic!("missing warning for {restraints:?}"));
            assert_eq!(warning.severity, "warning");
            assert!(
                warning.message.contains(expected_condition),
                "warning for {restraints:?} names the unmet condition: {}",
                warning.message
            );
            assert!(!result
                .results
                .iter()
                .any(|item| item.kind == "constant_effort_support_applied_load"));
            // No force entered the solve: every tip displacement/rotation
            // component matches the model without the support.
            for tail in ["ux", "uy", "uz", "rx", "ry", "rz"] {
                let id = format!("result:disp:node-N-110:{tail}");
                assert_eq!(
                    result_value(&result, &id),
                    result_value(&baseline, &id),
                    "non-consuming shape {restraints:?} changed {id}"
                );
            }
            // The DEC-049 review rows remain.
            assert!(result
                .results
                .iter()
                .any(|item| item.kind == "constant_effort_user_input_review"));
        }

        // Unknown support node: data conditions met but the node cannot be
        // resolved, so the support stays review-only with a warning.
        let mut unknown_node = cantilever_constant_effort_request(&["UY"], true, Some(tip_load));
        unknown_node
            .model
            .supports
            .iter_mut()
            .find(|support| support.id == "support:CE-110")
            .expect("constant-effort support present")
            .node = "node:MISSING".to_string();
        let result = run_linear_static_preview(unknown_node);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        let warning = result
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED")
            .expect("unknown-node constant-effort support records a warning");
        assert!(warning
            .message
            .contains("the support node is not present in the preview model"));
        assert!(!result
            .results
            .iter()
            .any(|item| item.kind == "constant_effort_support_applied_load"));
    }

    #[test]
    fn constant_effort_missing_or_nonpositive_load_keeps_existing_blocking_and_no_defaults() {
        // The landed DEC-049 validation slice already blocks a constant-effort
        // support without a finite positive constant load; that behavior is
        // unchanged and nothing is defaulted.
        let mut request = cantilever_constant_effort_request(&["UY"], true, Some(-350.0));
        request
            .model
            .supports
            .iter_mut()
            .find(|support| support.id == "support:CE-110")
            .expect("constant-effort support present")
            .hanger
            .as_mut()
            .expect("constant-effort support carries hanger data")
            .constant_load = None;
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result.diagnostics.iter().any(|diagnostic| diagnostic.code
            == "CONSTANT_EFFORT_LOAD_MISSING"
            && diagnostic.severity == "blocking"));
        assert!(!result
            .results
            .iter()
            .any(|item| item.kind == "constant_effort_support_applied_load"));

        // The classifier itself treats missing and non-positive loads as
        // non-consumption reasons (no default, no direction inference).
        let request = cantilever_constant_effort_request(&["UY"], true, Some(-350.0));
        let model = request.model;
        let mut support = model
            .supports
            .iter()
            .find(|support| support.id == "support:CE-110")
            .expect("constant-effort support present")
            .clone();
        support.hanger.as_mut().unwrap().constant_load = None;
        assert_eq!(
            classify_constant_effort_consumption(&model, &support).unwrap_err(),
            ConstantEffortNonConsumption::MissingConstantLoad
        );
        support.hanger.as_mut().unwrap().constant_load = Some(Quantity {
            value: -5.0,
            unit: "N".to_string(),
        });
        assert_eq!(
            classify_constant_effort_consumption(&model, &support).unwrap_err(),
            ConstantEffortNonConsumption::NonPositiveConstantLoad
        );
    }

    #[test]
    fn constant_effort_user_limit_comparison_warns_from_user_data_only() {
        let mut request = cantilever_constant_effort_request(&["UY"], true, Some(-350.0));
        let hanger = request
            .model
            .supports
            .iter_mut()
            .find(|support| support.id == "support:CE-110")
            .expect("constant-effort support present")
            .hanger
            .as_mut()
            .expect("constant-effort support carries hanger data");
        // Net tip force is +25 N, so |uy| at the tip is about 2.9e-5 m: the
        // user's own 1e-5 m movement limit is exceeded while the user's
        // 0.04 m travel range is not.
        hanger.movement_limit = Some(Quantity {
            value: 0.00001,
            unit: "m".to_string(),
        });
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        let warning = result
            .diagnostics
            .iter()
            .find(|diagnostic| {
                diagnostic.code == "SUPPORT_CONSTANT_EFFORT_USER_LIMIT_EXCEEDED"
                    && diagnostic.id
                        == "diagnostic:constant-effort-support:support-CE-110:movement-limit:load-L-100"
            })
            .expect("user movement-limit exceedance records a warning");
        assert_eq!(warning.severity, "warning");
        assert!(warning.message.contains("hanger.movement_limit"));
        assert!(warning
            .message
            .contains("no software threshold, tolerance, or acceptance criterion"));
        assert!(!result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "SUPPORT_CONSTANT_EFFORT_USER_LIMIT_EXCEEDED"
                && diagnostic.id.contains("travel-range")
        }));

        // With no user limit exceeded, no warning is emitted.
        let quiet = run_linear_static_preview(cantilever_constant_effort_request(
            &["UY"],
            true,
            Some(-350.0),
        ));
        assert!(!quiet
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "SUPPORT_CONSTANT_EFFORT_USER_LIMIT_EXCEEDED"));
    }

    #[test]
    fn constant_effort_coexists_with_nonlinear_supports_and_nonlinear_field_precedence() {
        // Consuming constant-effort support in a model whose solve also runs
        // the nonlinear active-set loop: both consume the same assembled
        // force vector.
        let mut request = mechanical_fixture_for_test(request(), "tests::constant_effort_coexists_with_nonlinear_supports_and_nonlinear_field_precedence");
        request
            .model
            .supports
            .iter_mut()
            .find(|support| support.id == "support:CE-120")
            .expect("fixture carries a constant-effort support")
            .restraints = vec!["UY".to_string()];
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(result
            .results
            .iter()
            .any(|item| item.id == "result:constant-effort-support:support-CE-120:applied-load"));
        assert!(result
            .results
            .iter()
            .any(|item| item.kind == "nonlinear_support_active_set_iteration_count"));
        assert!(!result
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED"));

        // A constant-effort support carrying a nonlinear field keeps the
        // existing nonlinear-path handling: it is neither classified for
        // constant-force consumption nor warned about.
        let mut precedence_request = mechanical_fixture_for_test(super::tests::request(), "tests::constant_effort_coexists_with_nonlinear_supports_and_nonlinear_field_precedence");
        let nonlinear_template = precedence_request
            .model
            .supports
            .iter()
            .find(|support| support.id == "support:NL-140")
            .expect("fixture carries a nonlinear support")
            .nonlinear
            .clone();
        precedence_request
            .model
            .supports
            .iter_mut()
            .find(|support| support.id == "support:CE-120")
            .expect("fixture carries a constant-effort support")
            .nonlinear = nonlinear_template;
        let result = run_linear_static_preview(precedence_request);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(!result
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED"));
        assert!(!result
            .results
            .iter()
            .any(|item| item.id == "result:constant-effort-support:support-CE-120:applied-load"));
    }

    #[test]
    fn models_without_constant_effort_supports_are_untouched_by_the_consumption_path() {
        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::models_without_constant_effort_supports_are_untouched_by_the_consumption_path",
        );
        request
            .model
            .supports
            .retain(|support| support.id != "support:CE-120");
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(!result
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code.starts_with("SUPPORT_CONSTANT_EFFORT")));
        assert!(!result
            .results
            .iter()
            .any(|item| item.kind.starts_with("constant_effort_")));
        // Only the variable-spring-hanger review rows remain.
        assert_eq!(result.summary.spring_hanger_user_input_count, 5);
    }

    #[test]
    fn valid_invented_model_exposes_explicit_load_combination_results() {
        // T0R (B-1): the fixture's nonlinear supports gate every mechanics
        // combination; its linear companion keeps the combination algebra tested.
        let mut input = mechanical_fixture_for_test(request(), "tests::valid_invented_model_exposes_explicit_load_combination_results");
        input.model.supports.retain(|support| support.nonlinear.is_none());
        let result = run_linear_static_preview(input);
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
        // T0R: maxima are never combined. The fixture's mechanics combination is
        // gated by its nonlinear supports, so drop them to reach an admitted one.
        let mut input = mechanical_fixture_for_test(
            request(),
            "tests::combination_stress_summary_rows_are_skipped_with_diagnostics",
        );
        input.model.supports.retain(|support| support.nonlinear.is_none());
        let result = run_linear_static_preview(input);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED", "{:?}", result.diagnostics);
        assert!(result
            .results
            .iter()
            .any(|item| item.basis_ref.as_ref().is_some_and(|b| b.ref_type == "combination")));
        assert!(!result.results.iter().any(|item| {
            item.basis_ref.as_ref().is_some_and(|b| b.ref_type == "combination")
                && matches!(item.kind.as_str(), "pipe_elastic_normal_stress_maximum_v2" | "open_formula_stress_summary")
        }));
        assert!(result.diagnostics.iter().any(|item| item.code
            == "COMBINATION_STRESS_MAXIMUM_UNAVAILABLE"
            && item.affected_refs == vec!["combination:C-OPER-ALT".to_string()]));
        assert!(!result.diagnostics.iter().any(|item| item.code == "COMBINATION_STRESS_SUMMARY_SKIPPED"));
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
        assert!((axial_i.value - expected_force).abs() < 1.0e-6);
        assert!((axial_j.value + expected_force).abs() < 1.0e-6);
        assert!((stress_i.value + expected_force / area / 1_000_000.0).abs() < 1.0e-6);
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
        let mut base = mechanical_fixture_for_test(
            request(),
            "tests::mill_tolerance_units_are_normalized_at_preview_boundary",
        );
        for pipe in &mut base.model.pipe_segments {
            pipe.section.mill_tolerance = Some(Quantity {
                value: 0.00125,
                unit: "m".to_string(),
            });
        }
        let mut millimeters = mechanical_fixture_for_test(
            request(),
            "tests::mill_tolerance_units_are_normalized_at_preview_boundary",
        );
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
                exposed_spans: Vec::new(),
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
                exposed_spans: Vec::new(),
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

    fn exposed_span_input(pipe: &str, start: f64, end: f64) -> WindExposedSpanInput {
        WindExposedSpanInput {
            pipe_ref: Some(pipe.to_string()),
            start_fraction: Some(Quantity {
                value: start,
                unit: "1".to_string(),
            }),
            end_fraction: Some(Quantity {
                value: end,
                unit: "1".to_string(),
            }),
        }
    }

    fn subspan_wind_request(
        exposed_pipe_refs: Vec<String>,
        exposed_spans: Vec<WindExposedSpanInput>,
    ) -> LinearStaticPreviewRequest {
        let mut request = equivalent_static_base_request();
        for pipe in &mut request.model.pipe_segments {
            pipe.section.insulation_thickness = Some(Quantity {
                value: 0.025,
                unit: "m".to_string(),
            });
            pipe.section.insulation_density = Some(Quantity {
                value: 120.0,
                unit: "kg/m^3".to_string(),
            });
        }
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
                direction: Some("global_x".to_string()),
                exposed_pipe_refs,
                exposed_spans,
            }),
            provenance: Some("invented_example_user_input".to_string()),
        });
        request
    }

    fn hand_computed_nodal_force(id: &str, node: &str, value: f64) -> PreviewPrimitiveLoad {
        PreviewPrimitiveLoad {
            id: id.to_string(),
            category: "wind".to_string(),
            target: LoadTargetInput::Node {
                node: node.to_string(),
            },
            direction: "global_x".to_string(),
            magnitude: Quantity {
                value,
                unit: "N".to_string(),
            },
            dimension: "force".to_string(),
            provenance: Some("invented_example_user_input".to_string()),
        }
    }

    #[test]
    fn subspan_wind_generation_matches_hand_computed_lever_rule_nodal_forces() {
        // Hand-calc witness tp_pmm_p3_subspan_wind_exposure.md, extent A:
        // w = 480 * 0.7 * (0.2 + 2 * 0.025) = 84 N/m over [0.2, 0.7] of the
        // 3 m span; W = 126 N at centroid fraction 0.45;
        // R_i = 69.3 N, R_j = 56.7 N.
        let generated =
            subspan_wind_request(Vec::new(), vec![exposed_span_input("pipe:P-100", 0.2, 0.7)]);

        let mut authored = subspan_wind_request(Vec::new(), Vec::new());
        authored.model.load_cases[0].equivalent_static = None;
        authored.model.load_cases[0].primitive_loads = vec![
            hand_computed_nodal_force("load:L-WIND-LEVER-I", "node:N-100", 69.3),
            hand_computed_nodal_force("load:L-WIND-LEVER-J", "node:N-110", 56.7),
        ];

        let generated_result = run_linear_static_preview(generated);
        let authored_result = run_linear_static_preview(authored);

        assert_eq!(generated_result.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(authored_result.status.mechanics, "MECHANICS_SOLVED");
        assert!(!generated_result.results.is_empty());
        assert_eq!(
            generated_result.results.len(),
            authored_result.results.len()
        );
        for (left, right) in generated_result
            .results
            .iter()
            .zip(authored_result.results.iter())
        {
            assert_eq!(left.id, right.id);
            // Axial shape functions retain lever-rule nodal shares; only
            // displacement/support action is equivalent. Member recovery must
            // account for the distributed load, unlike a nodal substitute.
            if !left.kind.starts_with("global_nodal_")
                && left.kind != "displacement_magnitude"
                && left.kind != "reaction_resultant"
            {
                continue;
            }
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
    fn subspan_wind_mixed_whole_and_partial_marking_superposes_on_distinct_pipes() {
        fn two_pipe_request(
            exposed_pipe_refs: Vec<String>,
            exposed_spans: Vec<WindExposedSpanInput>,
        ) -> LinearStaticPreviewRequest {
            let mut request = subspan_wind_request(exposed_pipe_refs, exposed_spans);
            let mut node = request.model.nodes[1].clone();
            node.id = "node:N-120".to_string();
            node.position = Vec3 {
                x: 6.0,
                y: 0.0,
                z: 0.0,
            };
            request.model.nodes.push(node);
            let mut pipe = request.model.pipe_segments[0].clone();
            pipe.id = "pipe:P-200".to_string();
            pipe.from = "node:N-110".to_string();
            pipe.to = "node:N-120".to_string();
            request.model.pipe_segments.push(pipe);
            let mut support = request.model.supports[1].clone();
            support.id = "support:S-120".to_string();
            support.node = "node:N-120".to_string();
            request.model.supports.push(support);
            request
        }

        // Whole-span marking on pipe:P-100 (w = 84 N/m over its full 3 m)
        // plus partial extent [0.5, 1.0] on pipe:P-200: W = 126 N at
        // centroid fraction 0.75 -> 31.5 N at node:N-110, 94.5 N at
        // node:N-120 (hand-calc lever rule).
        let generated = two_pipe_request(
            vec!["pipe:P-100".to_string()],
            vec![exposed_span_input("pipe:P-200", 0.5, 1.0)],
        );

        let mut authored = two_pipe_request(Vec::new(), Vec::new());
        authored.model.load_cases[0].equivalent_static = None;
        authored.model.load_cases[0].primitive_loads = vec![
            PreviewPrimitiveLoad {
                id: "load:L-WIND-WHOLE-P100".to_string(),
                category: "wind".to_string(),
                target: LoadTargetInput::Element {
                    pipe: "pipe:P-100".to_string(),
                },
                direction: "global_x".to_string(),
                magnitude: Quantity {
                    value: 84.0,
                    unit: "N/m".to_string(),
                },
                dimension: "force_per_length".to_string(),
                provenance: Some("invented_example_user_input".to_string()),
            },
            hand_computed_nodal_force("load:L-WIND-LEVER-N110", "node:N-110", 31.5),
            hand_computed_nodal_force("load:L-WIND-LEVER-N120", "node:N-120", 94.5),
        ];

        let generated_result = run_linear_static_preview(generated);
        let authored_result = run_linear_static_preview(authored);

        assert_eq!(generated_result.status.mechanics, "MECHANICS_SOLVED");
        assert_eq!(authored_result.status.mechanics, "MECHANICS_SOLVED");
        assert!(!generated_result.results.is_empty());
        assert_eq!(
            generated_result.results.len(),
            authored_result.results.len()
        );
        for (left, right) in generated_result
            .results
            .iter()
            .zip(authored_result.results.iter())
        {
            assert_eq!(left.id, right.id);
            // Axial shape functions retain lever-rule nodal shares; only
            // displacement/support action is equivalent. Member recovery must
            // account for the distributed load, unlike a nodal substitute.
            if !left.kind.starts_with("global_nodal_")
                && left.kind != "displacement_magnitude"
                && left.kind != "reaction_resultant"
            {
                continue;
            }
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
    fn subspan_wind_missing_span_fields_is_blocking() {
        let request = subspan_wind_request(
            Vec::new(),
            vec![WindExposedSpanInput {
                pipe_ref: None,
                start_fraction: None,
                end_fraction: None,
            }],
        );
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result.diagnostics.iter().any(|item| item.code
            == "EQUIVALENT_STATIC_INPUT_MISSING"
            && item.severity == "blocking"
            && item.message.contains("pipe_ref")
            && item.message.contains("start_fraction")
            && item.message.contains("end_fraction")));
        assert!(result.results.is_empty());
    }

    #[test]
    fn subspan_wind_unknown_pipe_ref_is_blocking() {
        let request =
            subspan_wind_request(Vec::new(), vec![exposed_span_input("pipe:P-999", 0.2, 0.7)]);
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "EQUIVALENT_STATIC_INPUT_INVALID"
                && item.severity == "blocking"
                && item.message.contains("not present in the preview model")));
        assert!(result.results.is_empty());
    }

    #[test]
    fn subspan_wind_double_marking_same_pipe_is_blocking() {
        let request = subspan_wind_request(
            vec!["pipe:P-100".to_string()],
            vec![exposed_span_input("pipe:P-100", 0.2, 0.7)],
        );
        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "EQUIVALENT_STATIC_INPUT_INVALID"
                && item.severity == "blocking"
                && item.message.contains("exactly one form")));
        assert!(result.results.is_empty());
    }

    #[test]
    fn subspan_wind_overlapping_extents_are_blocking_but_disjoint_extents_solve() {
        let overlapping = subspan_wind_request(
            Vec::new(),
            vec![
                exposed_span_input("pipe:P-100", 0.2, 0.6),
                exposed_span_input("pipe:P-100", 0.5, 0.9),
            ],
        );
        let result = run_linear_static_preview(overlapping);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "EQUIVALENT_STATIC_INPUT_INVALID"
                && item.severity == "blocking"
                && item.message.contains("overlap")));
        assert!(result.results.is_empty());

        // Disjoint extents on one pipe are allowed, each generating its own
        // load (touching endpoints have no interior overlap).
        let disjoint = subspan_wind_request(
            Vec::new(),
            vec![
                exposed_span_input("pipe:P-100", 0.2, 0.5),
                exposed_span_input("pipe:P-100", 0.5, 0.9),
            ],
        );
        let result = run_linear_static_preview(disjoint);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(!result
            .diagnostics
            .iter()
            .any(|item| item.code.starts_with("EQUIVALENT_STATIC_INPUT")));
    }

    #[test]
    fn subspan_wind_invalid_fractions_are_blocking() {
        for (start, end) in [(0.7, 0.2), (0.0, 1.5), (-0.1, 0.5), (0.5, 0.5)] {
            let request = subspan_wind_request(
                Vec::new(),
                vec![exposed_span_input("pipe:P-100", start, end)],
            );
            let result = run_linear_static_preview(request);
            assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
            assert!(result
                .diagnostics
                .iter()
                .any(|item| item.code == "EQUIVALENT_STATIC_INPUT_INVALID"
                    && item.severity == "blocking"
                    && item.message.contains("fractions are invalid")));
            assert!(result.results.is_empty());
        }
    }

    #[test]
    fn subspan_wind_on_curved_bend_macro_span_is_blocking() {
        let mut request = curved_bend_span_request();
        request.model.load_cases[0].primitive_loads = Vec::new();
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
                direction: Some("global_y".to_string()),
                exposed_pipe_refs: Vec::new(),
                exposed_spans: vec![exposed_span_input("pipe:P-100", 0.2, 0.7)],
            }),
            provenance: Some("invented_example_user_input".to_string()),
        });

        let result = run_linear_static_preview(request);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "LOAD_INPUT_INVALID"
                && item.severity == "blocking"
                && item
                    .message
                    .contains("partial extents are not supported on realized arcs")));
        assert!(result.results.is_empty());
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
            poisson_ratio: None,
            id: "temperature-point:hot".to_string(),
            temperature: Some(Quantity {
                value: 533.15,
                unit: "K".to_string(),
            }),
            elastic_modulus: Some(Quantity {
                value: 180_000_000_000.0,
                unit: "Pa".to_string(),
            }),
            shear_modulus: Some(Quantity {
                value: 50_000_000_000.0,
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
                poisson_ratio: None,
                id: "temperature-point:cold".to_string(),
                temperature: Some(Quantity {
                    value: 300.0,
                    unit: "K".to_string(),
                }),
                elastic_modulus: Some(Quantity {
                    value: 200_000_000_000.0,
                    unit: "Pa".to_string(),
                }),
                shear_modulus: Some(Quantity {
                    value: 60_000.0,
                    unit: "MPa".to_string(),
                }),
                thermal_expansion_coefficient: Some(Quantity {
                    value: 0.000012,
                    unit: "1/K".to_string(),
                }),
                provenance: Some("invented_cold_user_input".to_string()),
            },
            MaterialTemperaturePointInput {
                poisson_ratio: None,
                id: "temperature-point:hot".to_string(),
                temperature: Some(Quantity {
                    value: 500.0,
                    unit: "K".to_string(),
                }),
                elastic_modulus: Some(Quantity {
                    value: 180_000_000_000.0,
                    unit: "Pa".to_string(),
                }),
                shear_modulus: Some(Quantity {
                    value: 40_000.0,
                    unit: "MPa".to_string(),
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

    fn dec092_temperature_g_request() -> LinearStaticPreviewRequest {
        serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/invented_dec092_temperature_g_request.json"
        ))
        .expect("DEC-092 invented request fixture must deserialize")
    }

    fn dec092_torsion_constant() -> f64 {
        let outer_diameter: f64 = 0.12;
        let inner_diameter: f64 = 0.10;
        PI / 32.0 * (outer_diameter.powi(4) - inner_diameter.powi(4))
    }

    fn dec092_tip_rotation(shear_modulus: f64) -> f64 {
        12_000.0 * 4.0 / (shear_modulus * dec092_torsion_constant())
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
        assert!((axial.value - expected_force).abs() < 1.0e-6);

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
        assert!((axial.value - expected_force).abs() < 1.0e-6);

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
    fn dec092_fixture_consumes_exact_and_interpolated_g_with_provenance_and_combination_carry_through(
    ) {
        let result = run_linear_static_preview(dec092_temperature_g_request());

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        analytic_close!(
            result_value(&result, "result:disp:node-N-DEC092-TIP:rx"),
            dec092_tip_rotation(50.0e9)
        );
        analytic_close!(
            result_value(
                &result,
                "result:loadcase:load-L-DEC092-INTERPOLATED:disp:node-N-DEC092-TIP:rx"
            ),
            dec092_tip_rotation(47.5e9)
        );
        analytic_close!(
            result_value(
                &result,
                "result:loadcase:load-L-DEC092-BASE:disp:node-N-DEC092-TIP:rx"
            ),
            dec092_tip_rotation(80.0e9)
        );

        let exact_record = result
            .results
            .iter()
            .find(|item| {
                item.kind == "modulus_basis_record" && item.entity_ref == "load:L-DEC092-EXACT"
            })
            .expect("exact point-G basis record");
        let exact_basis = &exact_record.metadata.as_ref().unwrap().basis;
        assert!(exact_basis.contains("shear_modulus_source=temperature-point:exact"));
        assert!(exact_basis.contains("interpolation=not_performed"));

        let interpolated_record = result
            .results
            .iter()
            .find(|item| {
                item.kind == "modulus_basis_record"
                    && item.entity_ref == "load:L-DEC092-INTERPOLATED"
            })
            .expect("interpolated point-G basis record");
        let interpolated_basis = &interpolated_record.metadata.as_ref().unwrap().basis;
        assert!(interpolated_basis
            .contains("shear_modulus_sources=temperature-point:lower,temperature-point:upper"));
        assert!(interpolated_basis.contains("method=linear_temperature_interpolation"));
        assert!(interpolated_basis.contains("solve_temperature_kelvin=425"));

        let combination_records = result
            .results
            .iter()
            .filter(|item| item.kind == "combination_modulus_basis_record")
            .collect::<Vec<_>>();
        assert_eq!(combination_records.len(), 3);
        assert!(combination_records.iter().any(|item| {
            item.entity_ref == "load:L-DEC092-EXACT"
                && item
                    .metadata
                    .as_ref()
                    .is_some_and(|metadata| metadata.basis == *exact_basis)
        }));
        assert!(combination_records.iter().any(|item| {
            item.entity_ref == "load:L-DEC092-INTERPOLATED"
                && item
                    .metadata
                    .as_ref()
                    .is_some_and(|metadata| metadata.basis == *interpolated_basis)
        }));
        assert!(combination_records.iter().any(|item| {
            item.entity_ref == "load:L-DEC092-BASE"
                && item
                    .metadata
                    .as_ref()
                    .is_some_and(|metadata| metadata.basis == "material_base_values")
        }));
    }

    #[test]
    fn selected_point_g_is_base_independent_and_selected_g_sensitive() {
        let baseline = run_linear_static_preview(dec092_temperature_g_request());

        let mut changed_base = dec092_temperature_g_request();
        changed_base.materials[0]
            .shear_modulus
            .as_mut()
            .unwrap()
            .value = 20_000.0;
        let changed_base = run_linear_static_preview(changed_base);
        assert_eq!(changed_base.status.mechanics, "MECHANICS_SOLVED");
        for id in [
            "result:disp:node-N-DEC092-TIP:rx",
            "result:loadcase:load-L-DEC092-INTERPOLATED:disp:node-N-DEC092-TIP:rx",
        ] {
            assert_eq!(
                result_value(&changed_base, id),
                result_value(&baseline, id),
                "selected basis result {id} must not consume base G"
            );
        }
        analytic_close!(
            result_value(
                &changed_base,
                "result:loadcase:load-L-DEC092-BASE:disp:node-N-DEC092-TIP:rx"
            ),
            dec092_tip_rotation(20.0e9),
            "no-basis load case must continue to consume explicit base G"
        );

        let mut changed_exact = dec092_temperature_g_request();
        changed_exact.materials[0].temperature_points[0]
            .shear_modulus
            .as_mut()
            .unwrap()
            .value = 25_000.0;
        let changed_exact = run_linear_static_preview(changed_exact);
        assert_eq!(changed_exact.status.mechanics, "MECHANICS_SOLVED");
        analytic_close!(
            result_value(&changed_exact, "result:disp:node-N-DEC092-TIP:rx"),
            dec092_tip_rotation(25.0e9)
        );
        assert_ne!(
            result_value(&changed_exact, "result:disp:node-N-DEC092-TIP:rx"),
            result_value(&baseline, "result:disp:node-N-DEC092-TIP:rx")
        );
        assert_eq!(
            result_value(
                &changed_exact,
                "result:loadcase:load-L-DEC092-INTERPOLATED:disp:node-N-DEC092-TIP:rx"
            ),
            result_value(
                &baseline,
                "result:loadcase:load-L-DEC092-INTERPOLATED:disp:node-N-DEC092-TIP:rx"
            ),
            "non-coordinate exact point must not affect the adjacent temperature bracket"
        );
    }

    #[test]
    fn selected_basis_blocks_missing_invalid_or_dimensionally_wrong_point_g() {
        let mut missing_exact = dec092_temperature_g_request();
        missing_exact.materials[0].temperature_points[0].shear_modulus = None;
        let result = run_linear_static_preview(missing_exact);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result.diagnostics.iter().any(|item| {
            item.code == "MODULUS_BASIS_INPUT_MISSING"
                && item.severity == "blocking"
                && item.message.contains("never fall back to base G")
        }));
        assert!(result.results.is_empty());

        let mut missing_endpoint = dec092_temperature_g_request();
        missing_endpoint.materials[0].temperature_points[1].shear_modulus = None;
        let result = run_linear_static_preview(missing_endpoint);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result.diagnostics.iter().any(|item| {
            item.code == "MODULUS_BASIS_INPUT_MISSING"
                && item.severity == "blocking"
                && item.message.contains("temperature-point:lower")
                && item.message.contains("temperature-point:upper")
                && item.message.contains("never fall back to base G")
        }));

        for endpoint_index in [1, 2] {
            let mut invalid_endpoint = dec092_temperature_g_request();
            invalid_endpoint.materials[0].temperature_points[endpoint_index]
                .shear_modulus
                .as_mut()
                .unwrap()
                .value = 0.0;
            let result = run_linear_static_preview(invalid_endpoint);
            assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
            assert!(result.diagnostics.iter().any(|item| {
                item.code == "MODULUS_BASIS_INPUT_INVALID"
                    && item.severity == "blocking"
                    && item.message.contains("both source points")
                    && item.message.contains("never fall back to base G")
            }));
            assert!(result.results.is_empty());
        }

        for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut request = dec092_temperature_g_request();
            request.materials[0].temperature_points[0]
                .shear_modulus
                .as_mut()
                .unwrap()
                .value = invalid;
            let result = run_linear_static_preview(request);
            assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
            assert!(result.diagnostics.iter().any(|item| {
                item.code == "MODULUS_BASIS_INPUT_INVALID"
                    && item.severity == "blocking"
                    && item.message.contains("shear modulus")
            }));
            assert!(result.results.is_empty());
        }

        let mut wrong_dimension = dec092_temperature_g_request();
        wrong_dimension.materials[0].temperature_points[0]
            .shear_modulus
            .as_mut()
            .unwrap()
            .unit = "m".to_string();
        let result = run_linear_static_preview(wrong_dimension);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result.diagnostics.iter().any(|item| {
            item.code == "UNIT_CONVERSION_UNAVAILABLE"
                && item.severity == "blocking"
                && item.id.contains("shear-modulus")
        }));
        assert!(result.results.is_empty());
    }

    #[test]
    fn temperature_g_interpolation_uses_adjacent_points_and_duplicate_temperatures_still_block() {
        let mut adjacent = dec092_temperature_g_request();
        adjacent.materials[0]
            .temperature_points
            .push(MaterialTemperaturePointInput {
                poisson_ratio: None,
                id: "temperature-point:middle".to_string(),
                temperature: Some(Quantity {
                    value: 400.0,
                    unit: "K".to_string(),
                }),
                elastic_modulus: Some(Quantity {
                    value: 190_000.0,
                    unit: "MPa".to_string(),
                }),
                shear_modulus: Some(Quantity {
                    value: 100_000.0,
                    unit: "MPa".to_string(),
                }),
                thermal_expansion_coefficient: Some(Quantity {
                    value: 0.000013,
                    unit: "1/K".to_string(),
                }),
                provenance: Some("invented_dec092_middle_user_property_point".to_string()),
            });
        let result = run_linear_static_preview(adjacent);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        analytic_close!(
            result_value(
                &result,
                "result:loadcase:load-L-DEC092-INTERPOLATED:disp:node-N-DEC092-TIP:rx"
            ),
            dec092_tip_rotation(85.0e9)
        );
        let basis = &result
            .results
            .iter()
            .find(|item| {
                item.kind == "modulus_basis_record"
                    && item.entity_ref == "load:L-DEC092-INTERPOLATED"
            })
            .unwrap()
            .metadata
            .as_ref()
            .unwrap()
            .basis;
        assert!(basis
            .contains("shear_modulus_sources=temperature-point:middle,temperature-point:upper"));
        assert!(!basis
            .contains("shear_modulus_sources=temperature-point:lower,temperature-point:upper"));

        let mut duplicate = dec092_temperature_g_request();
        duplicate.materials[0].temperature_points[2]
            .temperature
            .as_mut()
            .unwrap()
            .value = 300.0;
        let result = run_linear_static_preview(duplicate);
        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result.diagnostics.iter().any(|item| {
            item.code == "MODULUS_BASIS_INPUT_INVALID"
                && item.severity == "blocking"
                && item
                    .message
                    .contains("duplicate user-entered temperature points")
        }));
        assert!(result.results.is_empty());
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
        assert!((axial.value - expected_force).abs() < 1.0e-6);
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
        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::range_combination_records_each_operand_modulus_basis",
        );
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
    fn missing_material_blocks_with_diagnostic() {
        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::missing_material_blocks_with_diagnostic",
        );
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
        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::missing_load_input_blocks_with_diagnostic",
        );
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
        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::missing_pipe_orientation_blocks_with_diagnostic",
        );
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
        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::subtraction_combination_subtracts_solved_rows_with_signed_determinism",
        );
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
        // Each published operand has at most half a last-place rounding
        // error; combination is rounded only after full-precision subtraction.
        assert!((combination.value - (base - alternate)).abs() <= 1.5e-6);
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
        assert!((reversed - (alternate - base)).abs() <= 1.5e-6);
        assert_eq!(combination.value, -reversed);
    }

    #[test]
    fn range_envelope_combination_selects_each_shipped_mode_deterministically() {
        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::range_envelope_combination_selects_each_shipped_mode_deterministically",
        );
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
    fn input_contract_repair_colocated_constant_force_and_contact_obey_equilibrium() {
        check_colocated_parallel_contact_equilibrium(false);
    }

    #[test]
    fn input_contract_repair_colocated_spring_and_contact_obey_equilibrium() {
        check_colocated_parallel_contact_equilibrium(true);
    }

    fn check_colocated_parallel_contact_equilibrium(spring: bool) {
        // Synthetic prismatic axial bar: exact EA/L stiffness, no bending/shear oracle.
        let area = std::f64::consts::PI * (0.168_f64.powi(2) - 0.154_f64.powi(2)) / 4.0;
        let bar_stiffness = 200e9 * area / 2.0;
        let gap_m = 0.05e-3;
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            for applied in [1000.0, 50_000.0] {
                let mut input = gap_closure_preview_request();
                input.model.combinations.clear();
                input.model.nodes[1].position.x = 2.0;
                input.model.load_cases[0].primitive_loads[0].magnitude.value = applied;
                let (parallel_stiffness, constant_force) =
                    if spring { (1e8, 0.0) } else { (0.0, 1500.0) };
                let mut parallel = if spring {
                    let mut support = input.model.supports[0].clone();
                    support.family = Some("spring".into());
                    support.stiffness = Some(SupportStiffnessInput {
                        dof: "UX".into(),
                        value: Quantity {
                            value: parallel_stiffness,
                            unit: "N/m".into(),
                        },
                    });
                    support
                } else {
                    let mut support = cantilever_constant_effort_request(&["UX"], true, None)
                        .model
                        .supports
                        .remove(1);
                    support.hanger.as_mut().unwrap().constant_load = Some(Quantity {
                        value: constant_force,
                        unit: "N".into(),
                    });
                    support
                };
                parallel.id = "support:PARALLEL".into();
                parallel.node = "node:N-110".into();
                parallel.restraints = vec!["UX".into()];
                input.model.supports.push(parallel);
                let result = run_linear_static_preview_with_mode(input.clone(), mode);
                assert_eq!(
                    result.status.mechanics, "MECHANICS_SOLVED",
                    "spring={spring}, force={applied}, {:?}",
                    result.diagnostics
                );
                let total_force = applied + constant_force;
                let expected_u = (total_force / (bar_stiffness + parallel_stiffness)).min(gap_m);
                let expected_contact =
                    (bar_stiffness + parallel_stiffness) * expected_u - total_force;
                let u = result_value(&result, "result:disp:node-N-110:ux") * 1e-3;
                assert!((u - expected_u).abs() <= 0.5e-9 + 1e-12);
                let contact = result_value(
                    &result,
                    "result:nonlinear-support:support-NL-GAP-110:ux-reaction",
                );
                assert!((contact - expected_contact).abs() <= 1e-6);
                assert!(contact <= 1e-6 && u <= gap_m + 0.5e-9 + 1e-12);
                assert!(
                    (support_force_norm(&result, "result:reaction:support-S-100")
                        - bar_stiffness * expected_u)
                        .abs()
                        <= 1e-6
                );
                if spring {
                    assert!(
                        (support_force_norm(&result, "result:reaction:support-PARALLEL")
                            - parallel_stiffness * expected_u)
                            .abs()
                            <= 1e-6
                    );
                    // Same-record law overlap is blocked: its current scalar action attribution cannot combine both laws.
                    input.model.supports[2].nonlinear = input.model.supports[1].nonlinear.clone();
                    input.model.supports.remove(1);
                    let blocked = run_linear_static_preview_with_mode(input, mode);
                    assert_eq!(blocked.status.mechanics, "MODEL_INCOMPLETE");
                    assert!(blocked
                        .diagnostics
                        .iter()
                        .any(|d| d.code == "SUPPORT_LINEAR_NONLINEAR_DOF_CONFLICT"));
                } else {
                    assert_eq!(
                        result_value(
                            &result,
                            "result:constant-effort-support:support-PARALLEL:applied-load"
                        ),
                        constant_force
                    );
                }
            }
        }
    }

    #[test]
    fn input_contract_repair_conflicting_spring_family_constant_hanger_blocks() {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            for stiffness in [false, true] {
                let mut input = cantilever_constant_effort_request(&["UY"], true, Some(350.0));
                input.model.components.clear();
                let support = &mut input.model.supports[1];
                support.family = Some("spring".into());
                if stiffness {
                    support.stiffness = Some(SupportStiffnessInput {
                        dof: "UY".into(),
                        value: Quantity {
                            value: 100_000.0,
                            unit: "N/m".into(),
                        },
                    });
                }
                let result = run_linear_static_preview_with_mode(input, mode);
                assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
                assert!(result
                    .diagnostics
                    .iter()
                    .any(|d| d.code == "SUPPORT_FAMILY_HANGER_CONFLICT"));
                if stiffness {
                    assert!(result
                        .diagnostics
                        .iter()
                        .any(|d| d.code == "SUPPORT_STIFFNESS_FAMILY_UNSUPPORTED"));
                }
                assert!(result.results.is_empty());
            }
        }
    }

    #[test]
    fn input_contract_repair_skew_authored_moments_match_rotated_analytical_couples() {
        // Proper rotation columns e1,e2,e3; no solver output supplies the oracle.
        let axes = [
            [1.0 / 3.0, 2.0 / 3.0, -2.0 / 3.0],
            [-2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0],
            [2.0 / 3.0, 1.0 / 3.0, 2.0 / 3.0],
        ];
        let inertia = std::f64::consts::PI * (0.168_f64.powi(4) - 0.154_f64.powi(4)) / 64.0;
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            for local_axis in 0..3 {
                for sign in [-1.0, 1.0] {
                    let mut input = p5_beam_request();
                    input.model.nodes[1].position = Vec3 {
                        x: 2.0 * axes[0][0],
                        y: 2.0 * axes[0][1],
                        z: 2.0 * axes[0][2],
                    };
                    input.model.pipe_segments[0].y_reference = Some(Vec3 {
                        x: axes[1][0],
                        y: axes[1][1],
                        z: axes[1][2],
                    });
                    let template = input.model.load_cases[0].primitive_loads[0].clone();
                    input.model.load_cases[0].primitive_loads =
                        ["rotation_x", "rotation_y", "rotation_z"]
                            .iter()
                            .enumerate()
                            .map(|(global, axis)| {
                                let mut load = template.clone();
                                load.id = format!("load:COUPLE-{global}");
                                load.category = "concentrated_moment".into();
                                load.direction = (*axis).into();
                                load.dimension = "moment".into();
                                load.magnitude = Quantity {
                                    value: sign * 250.0 * axes[local_axis][global],
                                    unit: "N*m".into(),
                                };
                                load
                            })
                            .collect();
                    let result = run_linear_static_preview_with_mode(input, mode);
                    assert_eq!(
                        result.status.mechanics, "MECHANICS_SOLVED",
                        "{:?}",
                        result.diagnostics
                    );
                    let rigidity = if local_axis == 0 {
                        77e9 * 2.0 * inertia
                    } else {
                        200e9 * inertia
                    };
                    let theta = sign * 250.0 * 2.0 / rigidity;
                    for (global, component) in ["rx", "ry", "rz"].iter().enumerate() {
                        let expected = axes[local_axis][global] * theta;
                        let actual =
                            result_value(&result, &format!("result:disp:node-N-110:{component}"));
                        assert!((actual - expected).abs() <= 0.5e-6 + 1e-12, "local={local_axis}, sign={sign}, global={component}: {actual} vs {expected}");
                    }
                    // u = L²/(2EI) (M cross e1), valid for the zero-shear end-couple case.
                    let moment = axes[local_axis].map(|value| sign * 250.0 * value);
                    let cross = [
                        moment[1] * axes[0][2] - moment[2] * axes[0][1],
                        moment[2] * axes[0][0] - moment[0] * axes[0][2],
                        moment[0] * axes[0][1] - moment[1] * axes[0][0],
                    ];
                    for (global, component) in ["ux", "uy", "uz"].iter().enumerate() {
                        let expected_mm =
                            1000.0 * 2.0_f64.powi(2) / (2.0 * 200e9 * inertia) * cross[global];
                        let actual_mm =
                            result_value(&result, &format!("result:disp:node-N-110:{component}"));
                        assert!((actual_mm - expected_mm).abs() <= 0.5e-6 + 1e-10);
                    }
                    for (axis, component) in
                        ["torsion", "bending-y", "bending-z"].iter().enumerate()
                    {
                        let expected = if axis == local_axis {
                            sign * 250.0
                        } else {
                            0.0
                        };
                        assert!(
                            (result_value(
                                &result,
                                &format!("result:moment:pipe-P-100:{component}")
                            ) + expected)
                                .abs()
                                <= 1e-6
                        );
                        assert!(
                            (result_value(
                                &result,
                                &format!("result:moment:pipe-P-100:{component}:end-j")
                            ) - expected)
                                .abs()
                                <= 1e-6
                        );
                    }
                    assert!(support_force_norm(&result, "result:reaction:support-S-100").abs() <= 1e-6);
                }
            }
        }
    }

    #[test]
    fn input_contract_local_provenance_is_traceability_not_clearance() {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            for source in ["user-entered", "local project drawing — not cleared"] {
                let mut input = p5_beam_request();
                input.model.load_cases[0].primitive_loads[0].provenance = Some(source.into());
                let preserved = input.clone();
                let result = run_linear_static_preview_with_mode(input, mode);
                assert_eq!(
                    result.status.mechanics, "MECHANICS_SOLVED",
                    "{:?}",
                    result.diagnostics
                );
                assert_eq!(
                    preserved.model.load_cases[0].primitive_loads[0]
                        .provenance
                        .as_deref(),
                    Some(source)
                );
            }
            let mut input = p5_beam_request();
            input.model.nodes[0].provenance = Some("  ".into());
            let result = run_linear_static_preview_with_mode(input, mode);
            assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
            assert!(result
                .diagnostics
                .iter()
                .any(|d| d.code == "PROVENANCE_INPUT_MISSING"));
        }
    }

    #[test]
    fn input_contract_authored_moment_axes_reach_live_solve() {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            for (axis, legacy, component) in [
                ("rotation_x", "RX", "rx"),
                ("rotation_y", "RY", "ry"),
                ("rotation_z", "RZ", "rz"),
            ] {
                let mut input = p5_beam_request();
                let load = &mut input.model.load_cases[0].primitive_loads[0];
                load.category = "concentrated_moment".into();
                load.direction = axis.into();
                load.dimension = "moment".into();
                load.magnitude = Quantity {
                    value: 250.0,
                    unit: "N*m".into(),
                };
                let mut alias = input.clone();
                alias.model.load_cases[0].primitive_loads[0].direction = legacy.into();
                let result = run_linear_static_preview_with_mode(input.clone(), mode);
                let reference = run_linear_static_preview_with_mode(alias, mode);
                assert_eq!(
                    result.status.mechanics, "MECHANICS_SOLVED",
                    "{:?}",
                    result.diagnostics
                );
                assert_eq!(reference.status.mechanics, "MECHANICS_SOLVED");
                let id = format!("result:disp:node-N-110:{component}");
                assert_ne!(result_value(&result, &id), 0.0);
                assert_eq!(result_value(&result, &id), result_value(&reference, &id));
                // Independent cantilever relation: theta = M L / (E I), or T L / (G J).
                let inertia = std::f64::consts::PI * (0.168_f64.powi(4) - 0.154_f64.powi(4)) / 64.0;
                let rigidity = if axis == "rotation_x" {
                    77e9 * 2.0 * inertia
                } else {
                    200e9 * inertia
                };
                assert!((result_value(&result, &id) - 250.0 * 2.0 / rigidity).abs() <= 1e-6);
                assert!(support_force_norm(&result, "result:reaction:support-S-100").abs() < 1e-6);
                let mut reversed = input.clone();
                reversed.model.load_cases[0].primitive_loads[0]
                    .magnitude
                    .value = -250.0;
                let negative = run_linear_static_preview_with_mode(reversed, mode);
                assert_eq!(negative.status.mechanics, "MECHANICS_SOLVED");
                assert!((result_value(&negative, &id) + 250.0 * 2.0 / rigidity).abs() <= 1e-6);
                input.model.load_cases[0].primitive_loads[0].dimension = "force".into();
                input.model.load_cases[0].primitive_loads[0].magnitude.unit = "N".into();
                assert_ne!(
                    run_linear_static_preview_with_mode(input, mode)
                        .status
                        .mechanics,
                    "MECHANICS_SOLVED"
                );
            }
        }
    }

    #[test]
    fn input_contract_guided_rest_preserves_disjoint_restraint() {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            let mut input = gap_closure_preview_request();
            input.model.combinations.clear();
            input.model.supports[1].nonlinear.as_mut().unwrap().dof = "UZ".into();
            input.model.supports[1].restraints = vec!["UY".into()];
            input.model.load_cases[0].primitive_loads[0].direction = "global_y".into();
            input.model.load_cases[0].primitive_loads[0].magnitude.value = 1000.0;
            let result = run_linear_static_preview_with_mode(input.clone(), mode);
            assert_eq!(
                result.status.mechanics, "MECHANICS_SOLVED",
                "{:?}",
                result.diagnostics
            );
            assert_eq!(result_value(&result, "result:disp:node-N-110:uy"), 0.0);
            // Tip load is applied at the guided DOF: its ground reaction is 1 kN.
            assert!(
                (support_force_norm(&result, "result:reaction:support-NL-GAP-110") - 1000.0).abs() < 1e-6
            );
            let mut split = input.clone();
            let mut guide = split.model.supports[1].clone();
            guide.id = "support:GUIDE".into();
            guide.nonlinear = None;
            guide.family = Some("guide".into());
            split.model.supports[1].restraints.clear();
            split.model.supports.push(guide);
            let split_result = run_linear_static_preview_with_mode(split, mode);
            assert_eq!(
                split_result.status.mechanics, "MECHANICS_SOLVED",
                "{:?}",
                split_result.diagnostics
            );
            assert_eq!(
                result_value(&split_result, "result:disp:node-N-110:uy"),
                0.0
            );
            assert!(
                (support_force_norm(&split_result, "result:reaction:support-GUIDE") - 1000.0).abs()
                    < 1e-6
            );
            // Activate the independent Z contact while retaining the Y guide.
            let mut contact = input.clone();
            let mut z_load = contact.model.load_cases[0].primitive_loads[0].clone();
            z_load.id = "load:Z-CONTACT".into();
            z_load.direction = "global_z".into();
            z_load.magnitude.value = 1000.0;
            contact.model.load_cases[0].primitive_loads.push(z_load);
            let active = run_linear_static_preview_with_mode(contact, mode);
            assert_eq!(
                active.status.mechanics, "MECHANICS_SOLVED",
                "{:?}",
                active.diagnostics
            );
            assert_eq!(result_value(&active, "result:disp:node-N-110:uy"), 0.0);
            assert!((result_value(&active, "result:disp:node-N-110:uz") - 0.05).abs() < 1e-6);
            input.model.supports[1].restraints.push("uz".into());
            let blocked = run_linear_static_preview_with_mode(input, mode);
            assert_eq!(blocked.status.mechanics, "MODEL_INCOMPLETE");
            assert!(blocked
                .diagnostics
                .iter()
                .any(|d| d.code == "SUPPORT_LINEAR_NONLINEAR_DOF_CONFLICT"));
            assert!(blocked.results.is_empty());
        }
    }

    #[test]
    fn input_contract_unused_stiffness_and_conflicting_spring_axis_block() {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            let mut input = p5_beam_request();
            input.model.supports[0].stiffness = Some(SupportStiffnessInput {
                dof: "UY".into(),
                value: Quantity {
                    value: 100_000.0,
                    unit: "N/m".into(),
                },
            });
            let blocked = run_linear_static_preview_with_mode(input.clone(), mode);
            assert_eq!(blocked.status.mechanics, "MODEL_INCOMPLETE");
            assert!(blocked
                .diagnostics
                .iter()
                .any(|d| d.code == "SUPPORT_STIFFNESS_FAMILY_UNSUPPORTED"));
            let mut spring = input.model.supports[0].clone();
            input.model.supports[0].stiffness = None;
            spring.id = "support:SPRING".into();
            spring.node = "node:N-110".into();
            spring.family = Some("spring".into());
            spring.restraints = vec!["UY".into()];
            input.model.supports.push(spring);
            assert_eq!(
                run_linear_static_preview_with_mode(input.clone(), mode)
                    .status
                    .mechanics,
                "MECHANICS_SOLVED"
            );
            input.model.supports[1].restraints.push("UZ".into());
            let blocked = run_linear_static_preview_with_mode(input, mode);
            assert_eq!(blocked.status.mechanics, "MODEL_INCOMPLETE");
            assert!(blocked
                .diagnostics
                .iter()
                .any(|d| d.code == "SUPPORT_SPRING_AXIS_CONFLICT"));
        }
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

    fn authored_coordinate_request(length_unit: &str) -> LinearStaticPreviewRequest {
        let mut wire: serde_json::Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/invented_preview_model.json"
        ))
        .unwrap();
        wire["project"]["units"]["length"] = length_unit.into();
        // Independent, invented equivalent encodings; the fixture already spans
        // all three axes. Translation makes every coordinate non-origin.
        let metres_per_unit = match length_unit {
            "m" => 1.0,
            "mm" => 0.001,
            "in" => 0.0254,
            _ => panic!("test encoding is not declared"),
        };
        for node in wire["nodes"].as_array_mut().unwrap() {
            for (axis, translation) in [("x", 1.27), ("y", -2.54), ("z", 3.81)] {
                node["position"][axis] = ((node["position"][axis].as_f64().unwrap() + translation)
                    / metres_per_unit)
                    .into();
            }
        }
        // Quantity records retain their own units, independently of coordinates.
        for pipe in wire["pipe_segments"].as_array_mut().unwrap() {
            for field in ["outside_diameter", "wall_thickness"] {
                let quantity = &mut pipe["section"][field];
                quantity["value"] = (quantity["value"].as_f64().unwrap() * 1000.0).into();
                quantity["unit"] = "mm".into();
            }
        }
        LinearStaticPreviewRequest {
            model: serde_json::from_value(wire).unwrap(),
            // Exercise the native caller's embedded-material path.
            materials: Vec::new(),
        }
    }

    #[test]
    fn authored_coordinate_units_preserve_preview_results_and_source() {
        for mode in [
            PreviewSolverMode::default(),
            PreviewSolverMode::DenseScrutiny,
        ] {
            let baseline = run_linear_static_preview_with_mode(
                mechanical_fixture_for_test(
                    authored_coordinate_request("m"),
                    "tests::authored_coordinate_units_preserve_preview_results_and_source",
                ),
                mode,
            );
            assert_eq!(baseline.status.mechanics, "MECHANICS_SOLVED");
            for unit in ["m", "mm", "in"] {
                let input = mechanical_fixture_for_test(
                    authored_coordinate_request(unit),
                    "tests::authored_coordinate_units_preserve_preview_results_and_source",
                );
                let source_before = format!("{input:?}");
                let result = run_linear_static_preview_with_mode(input.clone(), mode);
                assert_eq!(format!("{input:?}"), source_before);
                assert_eq!(input.model.project.units["length"], unit);
                assert!(!result.accepted_model_state_mutated);
                assert_eq!(result.status, baseline.status);
                assert_eq!(result.model_ref, baseline.model_ref);
                assert_eq!(result.results.len(), baseline.results.len());
                for (actual, expected) in result.results.iter().zip(&baseline.results) {
                    assert_eq!(actual.id, expected.id);
                    assert_eq!(actual.kind, expected.kind);
                    assert_eq!(actual.unit, expected.unit);
                    assert_eq!(actual.entity_ref, expected.entity_ref);
                    assert_eq!(actual.basis_ref, expected.basis_ref);
                    assert_eq!(actual.source_result_refs, expected.source_result_refs);
                    assert_eq!(actual.metadata, expected.metadata);
                    // Results are rounded to six decimals by the existing adapter.
                    assert!(
                        (actual.value - expected.value).abs() <= 1e-6 + expected.value.abs() * 1e-9,
                        "{mode:?}/{unit}/{}: {} != {}",
                        actual.id,
                        actual.value,
                        expected.value
                    );
                }
                assert_eq!(
                    serde_json::to_value(&result.summary).unwrap(),
                    serde_json::to_value(&baseline.summary).unwrap()
                );
            }
        }
    }

    #[test]
    fn node_normalization_preserves_quantities_directions_and_authored_metadata() {
        for unit in ["m", "mm", "in"] {
            let source = authored_coordinate_request(unit).model;
            let source_before = format!("{source:?}");
            let mut working = source.clone();
            let pipes_before = format!("{:?}", working.pipe_segments);
            let materials_before = format!("{:?}", working.materials);
            let loads_before = format!("{:?}", working.load_cases);
            let mut expected_units = source.project.units.clone();
            expected_units["length"] = "m".into();
            let mut diagnostics = Vec::new();
            normalize_node_coordinates(&mut working, &mut diagnostics);
            assert!(diagnostics.is_empty());
            assert_eq!(working.project.units, expected_units);
            assert_eq!(format!("{:?}", working.pipe_segments), pipes_before);
            assert_eq!(format!("{:?}", working.materials), materials_before);
            assert_eq!(format!("{:?}", working.load_cases), loads_before);
            assert_eq!(format!("{source:?}"), source_before);
            let normalized_before = format!("{working:?}");
            normalize_node_coordinates(&mut working, &mut diagnostics);
            assert!(diagnostics.is_empty());
            assert_eq!(format!("{working:?}"), normalized_before);
            if unit == "m" {
                assert_eq!(format!("{working:?}"), source_before);
            }
        }
    }

    #[test]
    fn missing_or_invalid_project_length_units_block_preview() {
        for mode in [
            PreviewSolverMode::default(),
            PreviewSolverMode::DenseScrutiny,
        ] {
            for units in [
                serde_json::Value::Null,
                serde_json::json!({}),
                serde_json::json!({"length": null}),
                serde_json::json!({"length": ""}),
                serde_json::json!({"length": "furlong_unknown"}),
                serde_json::json!({"length": "Pa"}),
                serde_json::json!({"length": 1000}),
                serde_json::json!("mm"),
            ] {
                let mut input = mechanical_fixture_for_test(
                    request(),
                    "tests::missing_or_invalid_project_length_units_block_preview",
                );
                input.model.project.units = units;
                let result = run_linear_static_preview_with_mode(input, mode);
                assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
                assert!(result.results.is_empty());
                assert!(result.diagnostics.iter().any(|item| {
                    item.code == "UNIT_INPUT_INVALID"
                        && item
                            .affected_refs
                            .contains(&"project.units.length".to_string())
                }));
            }
            let mut wire: serde_json::Value = serde_json::from_str(include_str!(
                "../../../fixtures/product_preview/invented_preview_model.json"
            ))
            .unwrap();
            wire["project"].as_object_mut().unwrap().remove("units");
            let result = run_linear_static_preview_with_mode(
                mechanical_fixture_for_test(
                    LinearStaticPreviewRequest {
                        model: serde_json::from_value(wire).unwrap(),
                        materials: Vec::new(),
                    },
                    "tests::missing_or_invalid_project_length_units_block_preview",
                ),
                mode,
            );
            assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
            assert!(result.results.is_empty());
            assert!(result
                .diagnostics
                .iter()
                .any(|item| item.code == "UNIT_INPUT_INVALID"));
        }
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
    fn audit_retained_spring_fixture_uses_selected_node_state() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            let result = run_linear_static_preview_with_mode(
                mechanical_fixture_for_test(
                    request(),
                    "tests::audit_retained_spring_fixture_uses_selected_node_state",
                ),
                mode,
            );
            assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
            for (node, support, dof) in [("N-140", "NL-140", "uy"), ("N-130", "NL-130-FRIC", "uz")]
            {
                assert_eq!(
                    result_value(&result, &format!("result:disp:node-{node}:{dof}")),
                    result_value(
                        &result,
                        &format!("result:nonlinear-support:support-{support}:{dof}-displacement")
                    )
                );
            }
        }
    }

    fn p5_beam_request() -> LinearStaticPreviewRequest {
        let mut input = cantilever_constant_effort_request(&[], false, Some(350.0));
        input.model.components.clear();
        input.model.load_cases[0].equivalent_static = None;
        input
    }

    fn p5_uniform_request(span: Option<(f64, f64)>) -> LinearStaticPreviewRequest {
        let mut input = p5_beam_request();
        input.model.load_cases[0].primitive_loads.clear();
        input.model.load_cases[0].equivalent_static = Some(EquivalentStaticGenerationInput {
            seismic: None,
            wind: Some(WindGenerationInput {
                pressure: Some(Quantity {
                    value: 100.0 / 0.168,
                    unit: "Pa".into(),
                }),
                shape_factor: Some(Quantity {
                    value: 1.0,
                    unit: "1".into(),
                }),
                direction: Some("global_y".into()),
                exposed_pipe_refs: if span.is_none() {
                    vec!["pipe:P-100".into()]
                } else {
                    vec![]
                },
                exposed_spans: span
                    .map(|(a, b)| vec![exposed_span_input("pipe:P-100", a, b)])
                    .unwrap_or_default(),
            }),
            provenance: Some("independent_p5_invented_uniform_fixture".into()),
        });
        input
    }

    fn p5_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= 0.5001e-6 + expected.abs() * 1e-10,
            "{actual} != {expected}"
        );
    }

    #[test]
    fn endpoint_section_cut_straight_axial_torsion_and_bending_match_every_station() {
        let cases = [
            ("global_x", "force", "N", "force", "axial", "axial-normal"),
            (
                "RX",
                "moment",
                "N*m",
                "moment",
                "torsion",
                "torsional-shear",
            ),
            (
                "RY",
                "moment",
                "N*m",
                "moment",
                "bending-y",
                "bending-normal-y",
            ),
            (
                "RZ",
                "moment",
                "N*m",
                "moment",
                "bending-z",
                "bending-normal-z",
            ),
        ];
        for reverse in [false, true] {
            for (direction, dimension, unit, family, resultant_tail, stress_tail) in cases {
                let mut request = p5_beam_request();
                let load = &mut request.model.load_cases[0].primitive_loads[0];
                load.direction = direction.into();
                load.dimension = dimension.into();
                load.magnitude.unit = unit.into();
                if reverse {
                    let pipe = &mut request.model.pipe_segments[0];
                    std::mem::swap(&mut pipe.from, &mut pipe.to);
                }
                let mut mode_rows = Vec::new();
                for mode in [
                    PreviewSolverMode::DenseScrutiny,
                    PreviewSolverMode::SparseInteractive,
                ] {
                    let result = run_linear_static_preview_with_mode(request.clone(), mode);
                    assert_eq!(
                        result.status.mechanics, "MECHANICS_SOLVED",
                        "{direction} reverse={reverse}: {:?}",
                        result.diagnostics
                    );
                    let raw_i_id = format!("result:{family}:pipe-P-100:{resultant_tail}");
                    let raw_j_id = format!("{raw_i_id}:end-j");
                    let raw_i = result_value(&result, &raw_i_id);
                    let raw_j = result_value(&result, &raw_j_id);
                    p5_close(raw_i.abs(), 350.0);
                    p5_close(raw_j.abs(), 350.0);
                    p5_close(raw_i, -raw_j);

                    let endpoint_i = format!("result:stress:pipe-P-100:end-i:{stress_tail}");
                    let endpoint_j = format!("result:stress:pipe-P-100:end-j:{stress_tail}");
                    let mut values = vec![
                        result_value(&result, &endpoint_i),
                        result_value(&result, &endpoint_j),
                    ];
                    for station in ["quarter-1", "midspan", "quarter-3"] {
                        values.push(result_value(
                            &result,
                            &format!("result:stress:pipe-P-100:{station}:{stress_tail}"),
                        ));
                    }
                    for value in &values[1..] {
                        p5_close(*value, values[0]);
                    }
                    assert!(
                        values[0].abs() > 0.0,
                        "{direction} must recover a nonzero section stress"
                    );

                    for (id, location) in [(&endpoint_i, "end_i"), (&endpoint_j, "end_j")] {
                        let metadata = result
                            .results
                            .iter()
                            .find(|row| row.id == id.as_str())
                            .and_then(|row| row.metadata.as_ref())
                            .expect("endpoint stress metadata exists");
                        assert_eq!(metadata.location, location);
                        assert_eq!(metadata.coordinate_system, "element_local");
                        assert_eq!(metadata.basis, SECTION_RESULTANT_BASIS);
                        assert_eq!(
                            metadata.sign_convention,
                            STRAIGHT_ENDPOINT_SECTION_SIGN_CONVENTION
                        );
                    }
                    mode_rows.push(values);
                }
                // Different factorization paths recover the same nonzero MPa
                // stresses under the existing relative 1e-9 numerical criterion.
                // This is cross-mode parity, not a repeat-determinism assertion.
                assert_eq!(mode_rows[0].len(), mode_rows[1].len());
                for (dense_mpa, sparse_mpa) in mode_rows[0].iter().zip(&mode_rows[1]) {
                    analytic_close!(*sparse_mpa, *dense_mpa);
                }
            }
        }
    }

    // U3 (RV127 B-1): the pressure-free thermal half of the retired
    // `endpoint_section_cut_fixed_and_free_pressure_thermal_match_uniform_stations_historical_pressure_premise`,
    // a current public check: fixed-fixed and free thermal states have uniform
    // endpoint and station axial-normal stress, and the fixed state is compressive.
    #[test]
    fn endpoint_section_cut_fixed_and_free_thermal_match_uniform_stations() {
        let mut fixed = fixed_fixed_thermal_request("global_x");
        for free in [false, true] {
            if free {
                fixed.model.supports.truncate(1);
            }
            for mode in [
                PreviewSolverMode::DenseScrutiny,
                PreviewSolverMode::SparseInteractive,
            ] {
                let result = run_linear_static_preview_with_mode(fixed.clone(), mode);
                assert_eq!(
                    result.status.mechanics, "MECHANICS_SOLVED",
                    "thermal free={free}: {:?}",
                    result.diagnostics
                );
                let endpoint_i =
                    result_value(&result, "result:stress:pipe-P-100:end-i:axial-normal");
                let endpoint_j =
                    result_value(&result, "result:stress:pipe-P-100:end-j:axial-normal");
                p5_close(endpoint_i, endpoint_j);
                for station in ["quarter-1", "midspan", "quarter-3"] {
                    p5_close(
                        result_value(
                            &result,
                            &format!("result:stress:pipe-P-100:{station}:axial-normal"),
                        ),
                        endpoint_i,
                    );
                }
                if !free {
                    assert!(endpoint_i < 0.0, "fixed thermal state is compressive");
                }
            }
        }
    }

    fn endpoint_section_cut_pressure_request(
        records: &[(&str, &str, f64)],
    ) -> LinearStaticPreviewRequest {
        let mut request = fixed_fixed_pressure_request("global_x");
        let template = request.model.load_cases[0].primitive_loads[0].clone();
        request.model.load_cases[0].primitive_loads = records
            .iter()
            .map(|(id, category, pressure)| PreviewPrimitiveLoad {
                id: (*id).to_string(),
                category: (*category).to_string(),
                magnitude: Quantity {
                    value: *pressure,
                    ..template.magnitude.clone()
                },
                ..template.clone()
            })
            .collect();
        request
    }

    #[test]
    fn input_contract_hydrotest_pressure_blocks_instead_of_silently_ignoring() {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            let request = endpoint_section_cut_pressure_request(&[(
                "load:L-HYDRO",
                "hydrotest",
                1_300_000.0,
            )]);
            let result = run_linear_static_preview_with_mode(request, mode);
            assert_eq!(
                result.status.mechanics, "MODEL_INCOMPLETE",
                "{:?}",
                result.diagnostics
            );
            assert!(result
                .diagnostics
                .iter()
                .any(|d| d.code == "HYDROTEST_PRESSURE_UNSUPPORTED"));
            assert!(result.results.is_empty());
        }
    }

    #[test]
    fn endpoint_section_cut_mixed_hydrotest_pressure_blocks_entire_solve() {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            let genuine = run_linear_static_preview_with_mode(
                endpoint_section_cut_pressure_request(&[("load:L-P-700", "pressure", 700_000.0)]),
                mode,
            );
            let mixed = run_linear_static_preview_with_mode(
                endpoint_section_cut_pressure_request(&[
                    ("load:L-P-700", "pressure", 700_000.0),
                    ("load:L-HYDRO-1100", "hydrotest", 1_100_000.0),
                ]),
                mode,
            );
            assert_eq!(genuine.status.mechanics, "MODEL_INCOMPLETE");
            assert!(genuine.results.is_empty());
            assert!(genuine
                .diagnostics
                .iter()
                .any(|d| d.code == "PRESSURE_MODEL_REAUTHOR_REQUIRED"));
            let hydrotest_only = run_linear_static_preview_with_mode(
                endpoint_section_cut_pressure_request(&[(
                    "load:L-HYDRO-1100",
                    "hydrotest",
                    1_100_000.0,
                )]),
                mode,
            );
            assert_eq!(hydrotest_only.status.mechanics, "MODEL_INCOMPLETE");
            assert!(hydrotest_only.results.is_empty());
            assert!(hydrotest_only
                .diagnostics
                .iter()
                .any(|d| d.code == "HYDROTEST_PRESSURE_UNSUPPORTED"
                    && d.affected_refs.contains(&"load:L-HYDRO-1100".to_string())));
            assert_eq!(mixed.status.mechanics, "MODEL_INCOMPLETE");
            assert!(mixed.results.is_empty());
            assert!(mixed
                .diagnostics
                .iter()
                .any(|d| d.code == "HYDROTEST_PRESSURE_UNSUPPORTED"
                    && d.affected_refs.contains(&"load:L-HYDRO-1100".to_string())));
        }
    }

    #[test]
    fn p5_uniform_full_partial_and_rotated_beams_match_independent_equilibrium() {
        let inertia = PI * (0.168_f64.powi(4) - 0.154_f64.powi(4)) / 64.0;
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            for (span, tip_numerator, root_force, root_moment) in [
                (None, 200.0, 200.0, 200.0),
                (Some((0.5, 1.0)), 1025.0 / 6.0, 100.0, 150.0),
            ] {
                let input = p5_uniform_request(span);
                let result = run_linear_static_preview_with_mode(input.clone(), mode);
                assert_eq!(
                    result.status.mechanics, "MECHANICS_SOLVED",
                    "{:?}",
                    result.diagnostics
                );
                p5_close(
                    result_value(&result, "result:disp:node-N-110:uy"),
                    1000.0 * tip_numerator / (200e9 * inertia),
                );
                p5_close(
                    support_force_norm(&result, "result:reaction:support-S-100"),
                    root_force,
                );
                p5_close(
                    result_value(&result, "result:force:pipe-P-100:shear-y"),
                    -root_force,
                );
                p5_close(
                    result_value(&result, "result:moment:pipe-P-100:bending-z"),
                    -root_moment,
                );
                p5_close(
                    result_value(&result, "result:force:pipe-P-100:midspan:shear-y"),
                    100.0,
                );
                p5_close(
                    result_value(&result, "result:moment:pipe-P-100:midspan:bending-z"),
                    50.0,
                );
                for id in [
                    "result:force:pipe-P-100:shear-y:end-j",
                    "result:moment:pipe-P-100:bending-z:end-j",
                ] {
                    p5_close(result_value(&result, id), 0.0);
                }
                // Rigid rotation: local X -> global Y, local Y -> global Z.
                let mut rotated = input;
                rotated.model.nodes[1].position = Vec3 {
                    x: 0.0,
                    y: 2.0,
                    z: 0.0,
                };
                rotated.model.pipe_segments[0].y_reference = Some(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                });
                rotated.model.load_cases[0]
                    .equivalent_static
                    .as_mut()
                    .unwrap()
                    .wind
                    .as_mut()
                    .unwrap()
                    .direction = Some("global_z".into());
                let other = run_linear_static_preview_with_mode(rotated, mode);
                assert_eq!(other.status.mechanics, "MECHANICS_SOLVED");
                p5_close(
                    result_value(&other, "result:disp:node-N-110:uz"),
                    1000.0 * tip_numerator / (200e9 * inertia),
                );
                p5_close(
                    member_maximum_mpa(&other, "pipe:P-100"),
                    root_moment / (inertia / 0.084) / 1e6,
                );
            }
        }
    }

    #[test]
    fn p5_piecewise_extrema_find_interior_peak_and_preserve_reversed_axes() {
        let inertia = PI * (0.168_f64.powi(4) - 0.154_f64.powi(4)) / 64.0;
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            for reverse in [false, true] {
                let mut input = p5_uniform_request(Some((0.0, 0.5)));
                input.model.supports[0].restraints.retain(|dof| dof != "RZ");
                let mut tip = input.model.supports[0].clone();
                tip.id = "support:tip".into();
                tip.node = "node:N-110".into();
                tip.restraints = vec!["UY".into()];
                tip.family = Some("anchor".into());
                input.model.supports.push(tip);
                if reverse {
                    let pipe = &mut input.model.pipe_segments[0];
                    std::mem::swap(&mut pipe.from, &mut pipe.to);
                    input.model.load_cases[0]
                        .equivalent_static
                        .as_mut()
                        .unwrap()
                        .wind
                        .as_mut()
                        .unwrap()
                        .exposed_spans = vec![exposed_span_input("pipe:P-100", 0.5, 1.0)];
                }
                let result = run_linear_static_preview_with_mode(input, mode);
                assert_eq!(
                    result.status.mechanics, "MECHANICS_SOLVED",
                    "{:?}",
                    result.diagnostics
                );
                p5_close(support_force_norm(&result, "result:reaction:support-S-100"), 75.0);
                p5_close(support_force_norm(&result, "result:reaction:support-tip"), 25.0);
                // Peak x=.75 (fraction .375) is not a public station sample.
                p5_close(
                    member_maximum_mpa(&result, "pipe:P-100"),
                    28.125 / (inertia / 0.084) / 1e6,
                );
            }
        }
    }

    #[test]
    fn p5_constant_signed_cut_fields_and_fixed_thermal_compression() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            for (direction, dimension, unit, tail) in [
                ("global_x", "force", "N", "axial"),
                ("global_y", "force", "N", "shear-y"),
                ("RX", "moment", "N*m", "torsion"),
            ] {
                let mut input = p5_beam_request();
                let load = &mut input.model.load_cases[0].primitive_loads[0];
                load.direction = direction.into();
                load.dimension = dimension.into();
                load.magnitude.unit = unit.into();
                let result = run_linear_static_preview_with_mode(input, mode);
                assert_eq!(
                    result.status.mechanics, "MECHANICS_SOLVED",
                    "{:?}",
                    result.diagnostics
                );
                for station in ["quarter-1", "midspan", "quarter-3"] {
                    let family = if unit == "N*m" { "moment" } else { "force" };
                    p5_close(
                        result_value(
                            &result,
                            &format!("result:{family}:pipe-P-100:{station}:{tail}"),
                        ),
                        350.0,
                    );
                }
            }
            let input = fixed_fixed_thermal_request("global_x");
            let result = run_linear_static_preview_with_mode(input, mode);
            let area = PI * (0.168_f64.powi(2) - 0.154_f64.powi(2)) / 4.0;
            for station in ["quarter-1", "midspan", "quarter-3"] {
                p5_close(
                    result_value(&result, &format!("result:force:pipe-P-100:{station}:axial")),
                    -200e9 * area * 12e-6 * 10.0,
                );
            }
        }
    }

    #[test]
    fn p5_combinations_cancel_vectors_and_preserve_subquantum_inputs() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            for factor in [2.0, 1e6] {
                let mut input = p5_beam_request();
                input.model.load_cases[0].primitive_loads[0].magnitude.value = 0.00035;
                let case = input.model.load_cases[0].id.clone();
                let mut combination = request().model.combinations[0].clone();
                combination.id = "combination:p5".into();
                combination.terms = vec![PreviewCombinationTerm {
                    load_case: case,
                    factor,
                }];
                input.model.combinations = vec![combination];
                let output = run_linear_static_preview_with_mode(input.clone(), mode);
                input.model.combinations.clear();
                input.model.load_cases[0].primitive_loads[0].magnitude.value *= factor;
                let direct = run_linear_static_preview_with_mode(input, mode);
                for tail in [
                    "disp:node-N-110:uy",
                    "disp:node-N-110",
                    "reaction:support-S-100",
                ] {
                    p5_close(
                        legacy_value(
                            &output,
                            &format!("result:combination:combination-p5:{tail}"),
                        ),
                        legacy_value(&direct, &format!("result:{tail}")),
                    );
                }
            }
            let mut input = p5_beam_request();
            let mut opposite = input.model.load_cases[0].clone();
            opposite.id = "load:opposite".into();
            opposite.primitive_loads[0].id = "load:opposite-force".into();
            opposite.primitive_loads[0].magnitude.value = -350.0;
            let mut combination = request().model.combinations[0].clone();
            combination.id = "combination:p5".into();
            combination.terms = vec![
                PreviewCombinationTerm {
                    load_case: input.model.load_cases[0].id.clone(),
                    factor: 1.0,
                },
                PreviewCombinationTerm {
                    load_case: opposite.id.clone(),
                    factor: 1.0,
                },
            ];
            input.model.load_cases.push(opposite);
            input.model.combinations = vec![combination];
            let output = run_linear_static_preview_with_mode(input, mode);
            for tail in [
                "disp:node-N-110:uy",
                "disp:node-N-110",
                "reaction:support-S-100",
            ] {
                p5_close(
                    legacy_value(
                        &output,
                        &format!("result:combination:combination-p5:{tail}"),
                    ),
                    0.0,
                );
            }
        }
    }

    #[test]
    fn p5_effective_thermal_alpha_and_duplicate_wind_fail_closed() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            for interpolate in [false, true] {
                let mut input = fixed_fixed_thermal_request("global_x");
                let mut material = interpolation_basis_material();
                material.temperature_points[1].thermal_expansion_coefficient = None;
                input.materials = vec![material];
                if interpolate {
                    input.model.load_cases[0].modulus_basis_temperature = Some(Quantity {
                        value: 400.0,
                        unit: "K".into(),
                    });
                } else {
                    input.model.load_cases[0].modulus_basis_ref =
                        Some("temperature-point:hot".into());
                }
                let output = run_linear_static_preview_with_mode(input, mode);
                assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE");
                assert!(output.results.is_empty());
                let expected_code = if interpolate {
                    "MODULUS_BASIS_INPUT_MISSING"
                } else {
                    "THERMAL_EXPANSION_INPUT_MISSING"
                };
                assert!(
                    output.diagnostics.iter().any(|d| d.code == expected_code),
                    "{interpolate}: {:?}",
                    output.diagnostics
                );
            }
            let mut input = p5_uniform_request(None);
            input.model.load_cases[0]
                .equivalent_static
                .as_mut()
                .unwrap()
                .wind
                .as_mut()
                .unwrap()
                .exposed_pipe_refs
                .push("pipe:P-100".into());
            let output = run_linear_static_preview_with_mode(input, mode);
            assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE");
            assert!(output
                .diagnostics
                .iter()
                .any(|d| d.code == "EQUIVALENT_STATIC_INPUT_INVALID"));
        }
    }

    #[test]
    fn p5_adjacent_spans_and_qualified_case_edges_preserve_physics() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            let full = run_linear_static_preview_with_mode(p5_uniform_request(None), mode);
            let mut input = p5_uniform_request(Some((0.0, 0.5)));
            input.model.load_cases[0]
                .equivalent_static
                .as_mut()
                .unwrap()
                .wind
                .as_mut()
                .unwrap()
                .exposed_spans
                .push(exposed_span_input("pipe:P-100", 0.5, 1.0));
            let split = run_linear_static_preview_with_mode(input, mode);
            assert_eq!(split.status.mechanics, "MECHANICS_SOLVED");
            for row in full.results.iter().filter(|r| {
                r.id.starts_with("result:disp:")
                    || r.id.starts_with("result:force:")
                    || r.id.starts_with("result:moment:")
                    || r.kind == "pipe_elastic_normal_stress_maximum_v2"
            }) {
                p5_close(result_value(&split, &row.id), row.value);
            }
            let output = run_linear_static_preview_with_mode(
                mechanical_fixture_for_test(
                    request(),
                    "tests::p5_adjacent_spans_and_qualified_case_edges_preserve_physics",
                ),
                mode,
            );
            let by_id = output
                .results
                .iter()
                .map(|row| (row.id.as_str(), row))
                .collect::<HashMap<_, _>>();
            let mut checked = 0;
            let mut qualified_case_edges = 0;
            for row in &output.results {
                let Some(basis) = row.basis_ref.as_ref().filter(|b| b.ref_type == "load_case")
                else {
                    continue;
                };
                for source in &row.source_result_refs {
                    let source = by_id.get(source.as_str()).unwrap_or_else(|| panic!("unresolved case edge {source}"));
                    assert_eq!(source.basis_ref.as_ref().unwrap().ref_id, basis.ref_id);
                    checked += 1;
                    qualified_case_edges += usize::from(source.id.starts_with("result:loadcase:"));
                }
            }
            // T0R: the retired SIF×k rows carried three edges each (24); their
            // equal-factor successors carry two each: 8 rows over two cases.
            // Every edge must now resolve, and the non-first case must be exercised.
            assert_eq!(checked, 16, "{checked} actual case source edges checked");
            assert_eq!(qualified_case_edges, 8);
        }
    }

    #[test]
    fn p5_disjoint_spans_superpose_signed_member_fields() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            let mut separate = p5_uniform_request(Some((0.0, 0.25)));
            let mut second = p5_uniform_request(Some((0.75, 1.0)))
                .model
                .load_cases
                .remove(0);
            second.id = "load:second-span".into();
            let mut combination = request().model.combinations[0].clone();
            combination.id = "combination:spans".into();
            combination.terms = vec![
                PreviewCombinationTerm {
                    load_case: separate.model.load_cases[0].id.clone(),
                    factor: 1.0,
                },
                PreviewCombinationTerm {
                    load_case: second.id.clone(),
                    factor: 1.0,
                },
            ];
            separate.model.load_cases.push(second);
            separate.model.combinations = vec![combination];
            let summed = run_linear_static_preview_with_mode(separate, mode);
            let mut union = p5_uniform_request(Some((0.0, 0.25)));
            union.model.load_cases[0]
                .equivalent_static
                .as_mut()
                .unwrap()
                .wind
                .as_mut()
                .unwrap()
                .exposed_spans
                .push(exposed_span_input("pipe:P-100", 0.75, 1.0));
            let union = run_linear_static_preview_with_mode(union, mode);
            assert_eq!(summed.status.mechanics, "MECHANICS_SOLVED");
            assert_eq!(union.status.mechanics, "MECHANICS_SOLVED");
            for row in union.results.iter().filter(|row| {
                row.id.starts_with("result:force:")
                    || row.id.starts_with("result:moment:")
                    || row.id.starts_with("result:disp:")
            }) {
                let combined = qualified_combination_result_id("combination:spans", &row.id);
                p5_close(result_value(&summed, &combined), row.value);
            }
        }
    }

    #[test]
    fn p5_combined_magnitude_withholds_incomplete_vector() {
        let input = p5_beam_request();
        let output = run_linear_static_preview(input);
        let case = "load:L-100".to_string();
        let mut rows = HashMap::new();
        for row in output.results {
            rows.insert(row.id.clone(), HashMap::from([(case.clone(), row)]));
        }
        rows.remove("result:disp:node-N-110:uy");
        let mut model = request().model;
        model.combinations.truncate(1);
        model.combinations[0].terms = vec![PreviewCombinationTerm {
            load_case: case,
            factor: 1.0,
        }];
        let mut results = Vec::new();
        let mut diagnostics = Vec::new();
        append_combination_results(
            &model,
            &rows,
            &HashMap::new(),
            &mut results,
            &mut diagnostics,
        );
        assert!(!results
            .iter()
            .any(|r| r.kind == "displacement_magnitude" && r.entity_ref == "node:N-110"));
        assert!(diagnostics
            .iter()
            .any(|d| d.code == "LOAD_COMBINATION_VECTOR_COMPONENT_MISSING"));
    }

    #[test]
    fn audit_ground_spring_stability_and_duplicate_constraints() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            for only_missing_spring in [false, true] {
                let mut input = friction_preview_request();
                input.model.supports.truncate(1);
                input.model.supports[0].restraints = if only_missing_spring {
                    vec!["UX", "UZ", "RX", "RY", "RZ"]
                        .into_iter()
                        .map(str::to_string)
                        .collect()
                } else {
                    Vec::new()
                };
                input.model.supports[0].family = Some("anchor".to_string());
                if !only_missing_spring {
                    input.model.supports.clear();
                }
                let dofs = if only_missing_spring {
                    vec!["UY"]
                } else {
                    vec!["UX", "UY", "UZ", "RX", "RY", "RZ"]
                };
                for dof in dofs {
                    input.model.supports.push(serde_json::from_value(serde_json::json!({
                        "id":format!("support:spring-{dof}"),"node":"node:N-100",
                        "family":"spring","provenance":"invented_audit_ground_spring","restraints":[],"stiffness":{"dof":dof,
                            "value":{"value":1e6,"unit":if dof.starts_with('R') {"N*m/rad"} else {"N/m"}}}
                    })).unwrap());
                }
                input.model.load_cases[0].primitive_loads[0].direction = "global_y".to_string();
                input.model.load_cases[0].primitive_loads[0].magnitude.value = 350.0;
                let output = run_linear_static_preview_with_mode(input, mode);
                assert_eq!(
                    output.status.mechanics, "MECHANICS_SOLVED",
                    "{:?}",
                    output.diagnostics
                );
                // Independent global equilibrium: the sole Y ground spring
                // carries the full 350 N, regardless of the beam's flexibility.
                // Compare the independently balanced force in N using the
                // analytic seed criterion; publication no longer quantizes it.
                analytic_close!(
                    support_force_norm(&output, "result:reaction:support-spring-UY"),
                    350.0
                );
            }
            let mut input = friction_preview_request();
            input.model.supports.truncate(1);
            input.model.supports[0].restraints = vec!["UX".into(), "UY".into(), "UZ".into()];
            assert_eq!(
                run_linear_static_preview_with_mode(input.clone(), mode)
                    .status
                    .mechanics,
                "MODEL_INCOMPLETE"
            );
            for index in 0..6 {
                let mut duplicate = input.model.supports[0].clone();
                duplicate.id = format!("support:duplicate-{index}");
                input.model.supports.push(duplicate);
            }
            assert_eq!(
                run_linear_static_preview_with_mode(input, mode)
                    .status
                    .mechanics,
                "MODEL_INCOMPLETE"
            );
        }
    }

    #[test]
    fn audit_nonfinite_computed_mechanics_never_publishes_solved_rows() {
        for mode in [
            PreviewSolverMode::DenseScrutiny,
            PreviewSolverMode::SparseInteractive,
        ] {
            let mut input = friction_preview_request();
            input.model.supports.truncate(1);
            input.model.load_cases[0].primitive_loads[0].magnitude.value = 1e308;
            let output = run_linear_static_preview_with_mode(input, mode);
            assert_ne!(output.status.mechanics, "MECHANICS_SOLVED");
            assert!(output.results.is_empty());
            assert!(!output
                .diagnostics
                .iter()
                .any(|d| d.code == "SPARSE_INTERACTIVE_DENSE_FALLBACK"));
            // F1b (ROOT's ruling at A2, an approved assertion change): the
            // truncated model is linear, so the case's range refusal on main
            // becomes W2: published at b = -494 (the integrity record carries
            // the `range_scaling:` line), then refused by today's derived-row
            // non-finite check (OQ6), which blocks the invocation. No solved
            // row is published either way.
            let integrity = output
                .diagnostics
                .iter()
                .find(|d| d.id == integrity_diagnostic_id("load:L-FRICTION"))
                .expect("the case's integrity record");
            assert!(
                integrity.message.contains(
                    " range_scaling: force_scale_exponent=-494; basis=exact power-of-two; "
                ),
                "{mode:?}: {}",
                integrity.message
            );
            assert!(output
                .diagnostics
                .iter()
                .any(|d| d.code == "ELEMENT_FORCE_RECOVERY_FAILED" && d.severity == "blocking"));
            let blocked: Vec<_> = output
                .diagnostics
                .iter()
                .filter(|d| d.code == "SOLVER_SYSTEM_BLOCKED")
                .collect();
            assert_eq!(blocked.len(), 1, "{mode:?}");
            assert_eq!(blocked[0].id, "diagnostic:physics:solver");
            assert_eq!(blocked[0].severity, "blocking");
            assert_eq!(
                blocked[0].message,
                "computed mechanics must be finite, got inf"
            );
        }
    }

    #[test]
    fn under_restrained_model_reports_solver_diagnostic() {
        let mut request = mechanical_fixture_for_test(
            request(),
            "tests::under_restrained_model_reports_solver_diagnostic",
        );
        request.model.supports.truncate(1);
        request.model.supports[0].restraints = vec!["UZ".to_string()];
        let contribution = format!(
            "{}@{}=UZ",
            request.model.supports[0].id, request.model.supports[0].node
        );

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        let diagnostic = result
            .diagnostics
            .iter()
            .find(|item| item.code == "SOLVER_SYSTEM_BLOCKED")
            .expect("under-restraint diagnostic should be present");
        // SUP-17 (T3 D1 §4.9): the whole message, in the design's wording.
        assert_eq!(
            diagnostic.message,
            format!(
                "fewer than six independent ground constraints including positive springs: \
                 the six rigid-body modes of a connected structure cannot all be removed; \
                 directly restrained global DOF classes: UZ; \
                 global DOF classes with no direct restraint: UX,UY,RX,RY,RZ \
                 (not a rigid-body mode analysis; separated restraints can resist rotations); \
                 support contributions: {contribution}"
            )
        );
        assert!(!diagnostic.message.contains("missing global rigid-body DOF classes"));
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
        // The arc bows toward +y (the fixture pipe's y_reference).
        let material = &invented_materials()[0];
        let od = 0.168_f64;
        let thickness = 0.007_f64;
        let inner = od - 2.0 * thickness;
        let area = PI * (od.powi(2) - inner.powi(2)) / 4.0;
        let second_moment = PI * (od.powi(4) - inner.powi(4)) / 64.0;
        CurvedBendMacroElement::new(
            node_i,
            node_j,
            CURVED_BEND_TEST_RADIUS_M,
            [0.0, 1.0, 0.0],
            material.elastic_modulus.value,
            material
                .shear_modulus
                .as_ref()
                .expect("validated material G")
                .value,
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
        let reduced =
            open_pipe_stress_frame_kernel::reduce_system(&dense, force.as_slice(), &restrained)
                .unwrap();
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
            (uy_mm - expected_tip[UY] * 1000.0).abs() <= 1.0e-6,
            "assembled arc tip displacement {uy_mm} mm must match the direct macro-element solve {} mm",
            expected_tip[UY] * 1000.0
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
            // Full-precision lane comparison under the existing DEC-053 relative criterion.
            analytic_close!(result_value(&sparse, row), result_value(&dense, row));
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
        assert!(metadata.basis.contains(
            "pressure_thrust_treatment=none_pressure_refused_outside_the_exact_straight_contract"
        ));
        assert!(!metadata
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
        // T0R (NOTE-1): a curved-bend macro-element publishes no stress
        // multiplier or intensified row anywhere; its SIF is recorded as not
        // applied until T4, and k enters stiffness only.
        let result = run_linear_static_preview(curved_bend_span_request());
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED", "{:?}", result.diagnostics);
        assert!(!result.results.iter().any(|item| matches!(item.kind.as_str(),
            "component_user_stress_multiplier_review" | "component_equal_factor_intensified_bending_stress_v1")));
        let diagnostic = result
            .diagnostics
            .iter()
            .find(|item| item.code == "COMPONENT_STRESS_INTENSIFICATION_NOT_APPLIED")
            .expect("not-applied diagnostic is present");
        assert_eq!(diagnostic.affected_refs, vec!["component:C-110".to_string()]);
        assert_eq!(result.summary.component_stress_modifier_count, 0);
        assert!(!result.diagnostics.iter().any(|item| item.code == "COMPONENT_STRESS_MULTIPLIER_APPLIED"));
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
                (value - expected[slot]).abs() <= 1.0e-3,
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
        assert_eq!(metadata.basis, SECTION_RESULTANT_BASIS);
        assert_eq!(metadata.coordinate_system, "element_local");
        assert_eq!(
            metadata.sign_convention,
            CURVED_BEND_SECTION_SIGN_CONVENTION
        );
        // Straight spans use the canonical stiffness-recovery category with
        // detailed section-equilibrium semantics in the sign convention.
        let straight = run_linear_static_preview(mechanical_fixture_for_test(
            request(),
            "tests::curved_bend_macro_element_emits_arc_interior_station_results",
        ));
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
            "recovered_from_local_element_stiffness"
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
        assert!(support_force_norm(&result, "result:reaction:support-S-100").abs() <= 1.0e-6);
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
        assert!(support_force_norm(&result, "result:reaction:support-S-100") > 1.0);
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
                (end_i - expected_forces[dof]).abs() <= 1.0e-3,
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
            (uz_mm - displacements[DOF_PER_NODE + UZ] * 1000.0).abs() <= 1.0e-6,
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
            (nonlinear_uy_mm - expected_tip[UY] * 1000.0).abs() <= 1.0e-6,
            "nonlinear loop tip displacement {nonlinear_uy_mm} mm must match the direct macro-element solve"
        );
    }

    // T4-U1 (I1 §5.3 #12): a realized bend takes its pipe's resolved E and G
    // (the 0.4.0 member pair when supplied), not the material's.
    #[test]
    fn t4_u1_realized_bend_uses_the_member_pair() {
        let request = curved_bend_span_request();
        let mut diagnostics = Vec::new();
        let base = build_model(&request.model, &request.materials, &mut diagnostics)
            .expect("material-ID build");
        let material = &request.materials[0];
        assert_eq!(
            base.curved_bend_elements[0].macro_element.elastic_modulus,
            material.elastic_modulus.value
        );
        let pair = pressure_exact::IsotropicENu::new(1.5e11, 0.3).unwrap();
        let pairs = HashMap::from([("pipe:P-100".to_string(), pair)]);
        let mut diagnostics = Vec::new();
        let built = build_model_for_members(
            &request.model,
            &request.materials,
            Some(&pairs),
            &mut diagnostics,
        )
        .expect("member-pair build");
        let element = &built.curved_bend_elements[0].macro_element;
        assert_eq!(element.elastic_modulus, pair.elastic_modulus_pa());
        assert_eq!(element.shear_modulus, pair.shear_modulus_pa());
        assert_ne!(element.elastic_modulus, material.elastic_modulus.value);
    }

    // SP-1 (T4-RV12 B-1): D-B's relaxed bend-geometry warning applies on every
    // route except documents declaring 2.0.0/exact_straight_pressure_v2, which
    // keep the pre-T4-U1 condition and text whatever their schema version.
    #[test]
    fn t4_u1_bend_geometry_warning_keeps_its_v2_condition_and_text() {
        const V2_TEXT: &str = "bend/elbow component requires explicit radius, angle, plane orientation, and invented or cleared geometry source to support component provenance review";
        const D_B_TEXT: &str = "bend/elbow component requires explicit radius, angle (unless realized as a curved bend), and invented or cleared geometry source to support component provenance review";
        let warnings = |model: &PreviewModel| {
            let mut diagnostics = Vec::new();
            validation::validate_model_inputs(model, &[], &mut diagnostics);
            diagnostics
                .into_iter()
                .filter(|d| d.code == "BEND_GEOMETRY_INPUT_MISSING")
                .map(|d| d.message)
                .collect::<Vec<_>>()
        };
        let v2 = |model: &mut PreviewModel, schema: &str| {
            model.schema_version = schema.to_string();
            model.pressure_contract = Some(PressureContractInput {
                version: Some("2.0.0".to_string()),
                mode: Some("exact_straight_pressure_v2".to_string()),
            });
        };
        let complete = curved_bend_span_request().model;
        assert!(complete.components[0].geometry.as_ref().unwrap().bend_plane_orientation.is_some());
        // A realized bend without angle and plane orientation, and the same
        // component as a geometry-only bend without plane orientation.
        let mut realized = complete.clone();
        let geometry = realized.components[0].geometry.as_mut().unwrap();
        geometry.bend_angle = None;
        geometry.bend_plane_orientation = None;
        let mut geometry_only = complete.clone();
        geometry_only.components[0].geometry.as_mut().unwrap().bend_plane_orientation = None;
        geometry_only.components[0]
            .mechanics_interface
            .as_mut()
            .unwrap()
            .solver_consumption = Some("mechanics_geometry_only".to_string());
        for model in [&realized, &geometry_only] {
            assert_eq!(warnings(model), Vec::<String>::new(), "D-B off v2");
            for schema in ["0.3.0", "0.4.0", "0.1.0"] {
                let mut declared = model.clone();
                v2(&mut declared, schema);
                assert_eq!(warnings(&declared), vec![V2_TEXT.to_string()], "v2 at {schema}");
            }
        }
        let mut no_radius = realized.clone();
        no_radius.components[0].geometry.as_mut().unwrap().bend_radius = None;
        assert_eq!(warnings(&no_radius), vec![D_B_TEXT.to_string()]);
        v2(&mut no_radius, "0.3.0");
        assert_eq!(warnings(&no_radius), vec![V2_TEXT.to_string()]);
        let mut complete_v2 = complete.clone();
        v2(&mut complete_v2, "0.4.0");
        assert_eq!(warnings(&complete_v2), Vec::<String>::new());
    }

    // T4-U1 (I1 §5.3 #12): a fit on a realized bend's span refers to its arc
    // length R·φ; a straight member keeps its chord length.
    #[test]
    fn t4_u1_fit_reference_length_is_the_arc_length_on_a_realized_bend() {
        let request = curved_bend_span_request();
        let model = &request.model;
        let from = [0.0, 0.0, 0.0];
        let to = [CURVED_BEND_TEST_CHORD_M, 0.0, 0.0];
        let y = model.pipe_segments[0].y_reference;
        let arc = case_state::resolve::fit_reference_length(model, "pipe:P-100", y, from, to);
        let expected = CURVED_BEND_TEST_RADIUS_M
            * 2.0
            * (0.5 * CURVED_BEND_TEST_CHORD_M / CURVED_BEND_TEST_RADIUS_M).asin();
        assert!((arc - expected).abs() <= 1e-14 * expected, "{arc} {expected}");
        assert!(arc > CURVED_BEND_TEST_CHORD_M);
        let straight = case_state::resolve::fit_reference_length(model, "pipe:other", y, from, to);
        assert_eq!(straight, CURVED_BEND_TEST_CHORD_M);
    }

    // The invented arc as a CurvedBendMacroBuild, mirroring the assembly of
    // `build_curved_bend_macro_elements` for the direct oracle element.
    fn curved_bend_direct_build() -> CurvedBendMacroBuild {
        let element = curved_bend_direct_element();
        let geometry = element.geometry().unwrap();
        CurvedBendMacroBuild {
            component_id: "component:C-110".to_string(),
            pipe_id: "pipe:P-100".to_string(),
            pipe_index: 0,
            node_i: 0,
            node_j: 1,
            chord: [CURVED_BEND_TEST_CHORD_M, 0.0, 0.0],
            global_stiffness: element.global_stiffness().unwrap(),
            arc_length: geometry.radius * geometry.included_angle,
            included_angle: geometry.included_angle,
            bend_radius: geometry.radius,
            flexibility_factor: CURVED_BEND_TEST_FLEXIBILITY,
            source_reference: "invented_example_user_input".to_string(),
            macro_element: element,
        }
    }

    #[test]
    fn legacy_bend_mode_keeps_multiplier_and_chord_realization_unchanged() {
        // The invented fixture bend stays on mechanics_geometry_only: the
        // straight chord is assembled. T0R: the SIF×k multiplier is retired and
        // the marker's equal-factor row uses the SIF only.
        let result = run_linear_static_preview(mechanical_fixture_for_test(
            request(),
            "tests::legacy_bend_mode_keeps_multiplier_and_chord_realization_unchanged",
        ));

        assert!(result
            .results
            .iter()
            .all(|item| item.kind != "curved_bend_macro_element_review"));
        let row = result
            .results
            .iter()
            .find(|item| item.id == "result:intensified-bending:component-C-110:pipe-P-100:end-j")
            .expect("marker equal-factor row is present");
        let metadata = row.metadata.as_ref().unwrap();
        assert!(metadata.sign_convention.contains("i=1.15"));
        assert!(!metadata.sign_convention.contains("1.08"));
    }
}

//! Product-preview physics adapter.
//!
//! This crate maps invented public preview-model data into the code-neutral
//! mechanics crates. It emits mechanics quantities and diagnostics only; it
//! does not encode standards criteria, allowables, SIF tables, private data, or
//! professional acceptance.

use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness, reduce_system, solve_dense, FrameElement, FrameKernelError,
    FrameNode, DOF_PER_NODE, RX, RY, RZ, UX, UY, UZ,
};
use open_pipe_stress_linear_supports::{
    prepare_boundary, FrameDof, LinearSupport, QuantityDimension, SupportFamily, SupportQuantity,
};
use open_pipe_stress_load_case_algebra::{
    evaluate_linear_combination, evaluate_range_envelope, evaluate_result_state_subtraction,
    AlgebraOperand, AlgebraQuantity, AlgebraResult, AnalysisStatus as AlgebraAnalysisStatus,
    CombinationTerm, FindingCode, RangeMode,
};
use open_pipe_stress_nonlinear_integration::{
    solve_active_set_frame, ConvergenceControl, ConvergencePolicyStatus,
    DerivedFrictionNormalReaction, FrictionNormalReaction, NonlinearFrameSolveInput,
    NonlinearIntegrationError,
};
use open_pipe_stress_nonlinear_supports::{
    ActivationSense, ActiveSetState, GapDirection, NonlinearSupport, SupportStateRecord,
};
use open_pipe_stress_primitive_loads::{
    prepare_loads, LoadDimension, LoadDirection, LoadQuantity, PrimitiveLoad, PrimitiveLoadCategory,
};
use open_pipe_stress_solver_diagnostics::{
    DiagnosticSeverity as SolverDiagnosticSeverity, SolverDiagnostic, SolverDiagnosticCode,
};
use open_pipe_stress_straight_pipe::{StraightPipeElement, StraightPipeSectionProperties};
use open_pipe_stress_stress_recovery::{
    recover_stresses, AnalysisStatus, ForceResultants, PressureBasis, StressComponents,
    StressRecoveryInput, StressSectionProperties,
};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

mod validation;
use validation::validate_model_inputs;

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
    #[serde(default)]
    pub provenance: Option<String>,
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
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LinearStaticPreviewRequest {
    pub model: PreviewModel,
    #[serde(default)]
    pub materials: Vec<MaterialInput>,
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
}

#[derive(Debug, Clone, Copy)]
struct ThermalElementLoad {
    element_index: usize,
    axial_load: f64,
}

#[derive(Debug, Clone, Copy)]
struct PressureThrustLoad {
    element_index: usize,
    axial_load: f64,
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

    let mut stiffness = match assemble_global_stiffness(built.nodes.len(), &built.frame_elements) {
        Ok(stiffness) => stiffness,
        Err(error) => return solver_blocked(model, diagnostics, error),
    };
    for spring in &boundary.springs {
        stiffness[spring.node_dof.global_index()][spring.node_dof.global_index()] +=
            spring.stiffness.value;
    }

    let mut load_case_solves = Vec::new();
    for load_case in &model.load_cases {
        match solve_load_case(
            &model,
            &built,
            &materials,
            &stiffness,
            &boundary.restrained_dofs,
            load_case,
            &mut diagnostics,
        ) {
            Ok(solve) => load_case_solves.push(solve),
            Err(error) => return solver_blocked(model, diagnostics, error),
        }
        if has_blocking(&diagnostics) {
            return blocked_envelope(model, diagnostics);
        }
    }

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
    let component_user_stiffness_macro_element_count =
        append_expansion_joint_user_stiffness_results(&model, &mut results);

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
    load_case: &PreviewLoadCase,
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
    let pressure_thrust_loads = build_pressure_thrust_loads(load_case, &pipe_map, &built.sections);

    let mut force = load_application.global_load_vector(built.nodes.len());
    add_uniform_element_loads(
        &mut force,
        model,
        &load_application.element_uniform_loads,
        &built.pipes,
    );
    add_pressure_thrust_loads(&mut force, &pressure_thrust_loads, &built.pipes);
    add_thermal_equivalent_loads(&mut force, &thermal_loads, &built.pipes);

    let reduced = reduce_system(stiffness, &force, restrained_dofs)?;
    let reduced_displacements = solve_dense(&reduced.stiffness, &reduced.force)?;

    let mut displacements = vec![0.0; built.nodes.len() * DOF_PER_NODE];
    for (index, dof) in reduced.free_dofs.iter().enumerate() {
        displacements[*dof] = reduced_displacements[index];
    }

    let mut results = Vec::new();
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
    );

    let mut max_stress = None;
    let mut component_stress_modifier_count = 0;
    for (pipe_index, pipe) in built.pipes.iter().enumerate() {
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
        let corrected_local_forces = corrected_local_forces_for_axial_effects(
            &local.local_forces,
            pipe_index,
            &thermal_loads,
            &pressure_thrust_loads,
        );
        append_element_force_results(&mut results, &pipe.element_id, &corrected_local_forces);
        let station_resultants = station_grid_resultants_from_endpoints(&corrected_local_forces);
        for station in &station_resultants {
            append_station_force_results(
                &mut results,
                &pipe.element_id,
                station.location,
                &station.resultants,
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
                "interpolated_from_endpoint_resultants",
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
) {
    if built.nonlinear_supports.is_empty() {
        return;
    }

    let convergence = match ConvergenceControl::new(
        "DEC-046-CV-B-preview-active-set-count-TBD",
        ConvergencePolicyStatus::Tbd,
        0.0,
        0.0,
        8,
    ) {
        Ok(convergence) => convergence,
        Err(error) => {
            diagnostics.push(nonlinear_loop_blocked_diag(&load_case.id, error));
            return;
        }
    };

    let input = NonlinearFrameSolveInput {
        node_count: built.nodes.len(),
        elements: built.frame_elements.clone(),
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

    match solve_active_set_frame(&input) {
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
                    "dense_active_set_loop; policy_ref={}; policy_status=TBD; support_count={}",
                    solve.policy_ref,
                    built.nonlinear_supports.len()
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
                    "dense_active_set_loop; policy_ref={}; residual_is_changed_support_count",
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
                    "dense_active_set_loop; policy_ref={}; policy_status=TBD",
                    solve.policy_ref
                ),
                "1 means the active-set state-change residual satisfied the supplied preview tolerance",
            );
            append_nonlinear_friction_normal_evidence(
                results,
                &built.nonlinear_friction_normal_reactions,
                &built.nonlinear_derived_friction_normal_reactions,
                &solve.reactions,
                &solve.policy_ref,
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
                        "dense_active_set_loop; policy_ref={}; final_state={}",
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
                        "dense_active_set_loop; policy_ref={}; final_state={}",
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
                        "dense_active_set_loop; policy_ref={}; final_state={}",
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
                        "nonlinear support {} ended in {} state for load case {}; dense preview loop evidence only",
                        support.support_id,
                        state.state.as_str(),
                        load_case.id
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
                    "dense nonlinear support active-set preview completed {} iteration(s); final residual count {}; policy_ref={} remains release-TBD",
                    solve.iterations.len(),
                    final_residual,
                    solve.policy_ref
                ),
                vec![load_case.id.clone(), "DEC-046".to_string()],
            ));
        }
        Err(error) => diagnostics.push(nonlinear_loop_blocked_diag(&load_case.id, error)),
    }
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
        frame_elements.push(
            element
                .frame_element()
                .expect("validated straight pipe frame element"),
        );
        sections.insert(pipe.id.clone(), derived);
        pipes.push(element);
    }

    let nonlinear = build_nonlinear_supports(model, &node_map, diagnostics);
    let supports = model
        .supports
        .iter()
        .filter_map(|support| {
            if support.nonlinear.is_some() {
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
            if support.family.as_deref() == Some("spring") {
                let stiffness = support.stiffness.as_ref().and_then(|input| {
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
                    dofs.first().copied().unwrap_or(FrameDof::Uz),
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
        supports,
        nonlinear_supports: nonlinear.supports,
        nonlinear_initial_states: nonlinear.initial_states,
        nonlinear_friction_normal_reactions: nonlinear.friction_normal_reactions,
        nonlinear_derived_friction_normal_reactions: nonlinear.derived_friction_normal_reactions,
        sections,
    })
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

    load_case
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
        .collect()
}

fn derive_pipe_section(
    input: &PipeSectionInput,
    pipe_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<DerivedSection> {
    let od = input.outside_diameter.value;
    let thickness = input.wall_thickness.value;
    if !od.is_finite()
        || !thickness.is_finite()
        || od <= 0.0
        || thickness <= 0.0
        || 2.0 * thickness >= od
    {
        diagnostics.push(diag(&format!("diagnostic:section:{}", stable_suffix(pipe_id)), "PIPE_DIMENSION_INVALID", "blocking", "outside diameter and wall thickness must be explicit positive values with wall thickness less than radius", vec![pipe_id.to_string()]));
        return None;
    }
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
) {
    for load in loads {
        if matches!(
            load.magnitude.dimension,
            LoadDimension::Pressure | LoadDimension::TemperatureChange
        ) {
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
        });
    }
    loads
}

fn build_pressure_thrust_loads(
    load_case: &PreviewLoadCase,
    pipe_map: &HashMap<&str, usize>,
    sections: &HashMap<String, DerivedSection>,
) -> Vec<PressureThrustLoad> {
    let mut loads = Vec::new();
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
        let Some(section) = sections.get(pipe) else {
            continue;
        };
        loads.push(PressureThrustLoad {
            element_index,
            axial_load: load.magnitude.value * section.internal_area,
        });
    }
    loads
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
) {
    for load in thermal_loads {
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

fn pressure_thrust_for_pipe(element_index: usize, pressure_loads: &[PressureThrustLoad]) -> f64 {
    pressure_loads
        .iter()
        .filter(|load| load.element_index == element_index)
        .map(|load| load.axial_load)
        .sum::<f64>()
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
            "positive value is linearly interpolated from endpoint element-local resultants",
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

fn append_station_force_result(
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
            basis: "interpolated_from_endpoint_resultants".to_string(),
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
    if solver_consumption != "mechanics_geometry_only" {
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
    let multiplier = modifier.sif * modifier.flexibility;
    let value = round6(base_value_mpa * multiplier);
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
            basis: format!(
                "component_family={};component_side={};user_entered_sif={};user_entered_flexibility={};source={};solver_consumption={}",
                modifier.family,
                modifier.side,
                rounded_scalar(modifier.sif),
                rounded_scalar(modifier.flexibility),
                modifier.source_reference,
                modifier.solver_consumption
            ),
            sign_convention:
                "positive value is base open-mechanics stress summary multiplied by user-entered component modifiers; base frame stiffness unchanged"
                    .to_string(),
        }),
    });
    diagnostics.push(diag(
        &format!(
            "diagnostic:component-stress-multiplier:{}:{}:{}",
            component_suffix, pipe_suffix, endpoint
        ),
        "COMPONENT_STRESS_MULTIPLIER_APPLIED",
        "info",
        format!(
            "{} component {} applies user-entered {} SIF {} and flexibility factor {} to {} {location} stress-recovery review; solver_consumption remains {}; no protected or default component factor is supplied",
            modifier.family,
            component.id,
            modifier.side,
            rounded_scalar(modifier.sif),
            rounded_scalar(modifier.flexibility),
            pipe_id,
            modifier.solver_consumption
        ),
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
                        "component_family=expansion_joint;user_entered_axis={axis};source={source_reference};solver_consumption={solver_consumption};pressure_thrust={}",
                        geometry
                            .pressure_thrust_reference
                            .as_deref()
                            .unwrap_or("load_side_pressure_thrust_reference_missing")
                    ),
                    sign_convention:
                        "positive value is user-entered expansion-joint stiffness input evidence; global macro-element solve is not claimed by this preview row"
                            .to_string(),
                }),
            });
            appended += 1;
        }
    }
    appended
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
                "dense_active_set_loop; policy_ref={policy_ref}; explicit_user_entered_normal_reaction; no_catalog_or_default_normal_force"
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
                "dense_active_set_loop; policy_ref={policy_ref}; derived_support_reaction; source_ref={}; source_dof={}",
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

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(!result.results.is_empty());
        assert!(result.summary.max_displacement.as_ref().unwrap().value > 0.0);
        assert!(result
            .results
            .iter()
            .any(|item| item.id == "result:disp:node-N-140"));
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
        assert_eq!(
            result_value(&result, "result:nonlinear-support:iteration-count"),
            1.0
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
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-130-FRIC:uz-reaction"
            ),
            0.0
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
        assert_eq!(normal_evidence.value, 158.102028);
        let normal_metadata = normal_evidence.metadata.as_ref().unwrap();
        assert!(normal_metadata.basis.contains("derived_support_reaction"));
        assert!(normal_metadata.basis.contains("source_ref=support:S-130"));
        assert!(normal_metadata.basis.contains("source_dof=uy"));
        assert!(!normal_metadata
            .basis
            .contains("derived_normal_force_model=TBD"));
        assert!(diagnostic_codes.contains("TOLERANCE_POLICY_TBD"));
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
                nonlinear: None,
                provenance: Some("invented_example".to_string()),
            },
            PreviewSupport {
                id: support_id.to_string(),
                node: "node:N-110".to_string(),
                restraints: Vec::new(),
                family: Some("nonlinear".to_string()),
                stiffness: None,
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
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-FRIC-SLIDE-110:ux-reaction"
            ),
            0.0
        );
        assert_eq!(
            result_value(
                &result,
                "result:nonlinear-support:support-NL-FRIC-SLIDE-110:friction-normal-reaction"
            ),
            10.0
        );
        assert!(diagnostic_codes.contains("TOLERANCE_POLICY_TBD"));
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
        assert!(diagnostic_codes.contains("TOLERANCE_POLICY_TBD"));
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
        assert!(diagnostic_codes.contains("TOLERANCE_POLICY_TBD"));
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
            .contains("pressure_thrust=load_side_pressure_thrust_user_review_required"));
        assert!(axial_metadata
            .sign_convention
            .contains("global macro-element solve is not claimed"));

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
}

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
    evaluate_linear_combination, AlgebraOperand, AlgebraQuantity,
    AnalysisStatus as AlgebraAnalysisStatus, CombinationTerm, FindingCode,
};
use open_pipe_stress_primitive_loads::{
    prepare_loads, LoadDimension, LoadDirection, LoadQuantity, PrimitiveLoad, PrimitiveLoadCategory,
};
use open_pipe_stress_straight_pipe::{StraightPipeElement, StraightPipeSectionProperties};
use open_pipe_stress_stress_recovery::{
    recover_stresses, AnalysisStatus, ForceResultants, PressureBasis, StressComponents,
    StressRecoveryInput, StressSectionProperties,
};
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
pub struct PreviewSupport {
    pub id: String,
    pub node: String,
    pub restraints: Vec<String>,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub stiffness: Option<SupportStiffnessInput>,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SupportStiffnessInput {
    pub dof: String,
    pub value: Quantity,
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
    pub basis: String,
    #[serde(default)]
    pub terms: Vec<PreviewCombinationTerm>,
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
    sections: HashMap<String, DerivedSection>,
}

#[derive(Debug, Clone)]
struct LoadCaseSolve {
    load_case_id: String,
    results: Vec<ResultItem>,
    max_displacement: Option<LocatedQuantity>,
    max_stress: Option<LocatedQuantity>,
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
    let model = request.model;
    let materials = if request.materials.is_empty() {
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

    let mut max_stress = None;
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
    }

    Ok(LoadCaseSolve {
        load_case_id: load_case.id.clone(),
        results,
        max_displacement,
        max_stress,
    })
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

    let supports = model
        .supports
        .iter()
        .filter_map(|support| {
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
        sections,
    })
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

fn append_combination_results(
    model: &PreviewModel,
    rows_by_base_id: &HashMap<String, HashMap<String, ResultItem>>,
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut base_ids = rows_by_base_id.keys().cloned().collect::<Vec<_>>();
    base_ids.sort();

    for combination in &model.combinations {
        for base_id in &base_ids {
            let Some(source_rows) = rows_by_base_id.get(base_id) else {
                continue;
            };
            let Some(first_term) = combination.terms.first() else {
                continue;
            };
            let Some(reference_row) = source_rows.get(&first_term.load_case) else {
                diagnostics.push(combination_diag(
                    combination,
                    base_id,
                    "LOAD_COMBINATION_SOURCE_RESULT_MISSING",
                    "warning",
                    format!(
                        "combination source result {base_id} is not available for load case {}",
                        first_term.load_case
                    ),
                    vec![combination.id.clone(), first_term.load_case.clone()],
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
            for term in &combination.terms {
                let Some(source) = source_rows.get(&term.load_case) else {
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
                    term.load_case.clone(),
                    term.load_case.clone(),
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
            let terms = combination
                .terms
                .iter()
                .filter_map(|term| CombinationTerm::new(term.load_case.clone(), term.factor).ok())
                .collect::<Vec<_>>();
            let operand_by_id = operands
                .iter()
                .map(|operand| (operand.operand_id.as_str(), operand))
                .collect::<HashMap<_, _>>();
            let algebra = evaluate_linear_combination(&operand_by_id, &terms);
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
                metadata.basis = "explicit_user_linear_combination".to_string();
                metadata.sign_convention =
                    "positive value follows explicit user linear combination of matching source result sign conventions"
                        .to_string();
            }
            results.push(combined);
        }
    }
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
        request.model.combinations[0].basis = "owner_design_basis".to_string();
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
    fn invalid_material_unit_blocks_with_diagnostic() {
        let mut request = request();
        request.materials[0].elastic_modulus.unit = "MPa".to_string();

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

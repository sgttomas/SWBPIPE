use crate::LoadTargetInput;
use crate::{
    canonical_unit_symbol, diag, expected_load_dimension, parse_dof, stable_suffix,
    unit_symbol_matches_dimension, Diagnostic, MaterialInput, PreviewCombination, PreviewModel,
    Quantity,
};
use open_pipe_stress_load_case_algebra::RangeMode;
use open_pipe_stress_units::Dimension;
use std::collections::{HashMap, HashSet};

pub(crate) fn validate_model_inputs(
    model: &PreviewModel,
    materials: &[MaterialInput],
    diagnostics: &mut Vec<Diagnostic>,
) {
    validate_required_collections(model, diagnostics);
    validate_ids(model, materials, diagnostics);
    validate_provenance(model, materials, diagnostics);
    validate_units(model, materials, diagnostics);
    validate_components(model, diagnostics);
    validate_thermal_inputs(model, materials, diagnostics);
    validate_combinations(model, diagnostics);
}

fn validate_required_collections(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    if model.nodes.len() < 2 {
        diagnostics.push(diag(
            "diagnostic:physics:nodes-missing",
            "NODE_INPUT_MISSING",
            "blocking",
            "linear-static preview requires at least two explicit nodes",
            vec!["nodes".to_string()],
        ));
    }
    if model.pipe_segments.is_empty() {
        diagnostics.push(diag(
            "diagnostic:physics:pipes-missing",
            "PIPE_INPUT_MISSING",
            "blocking",
            "linear-static preview requires at least one explicit pipe segment",
            vec!["pipe_segments".to_string()],
        ));
    }
    if model
        .load_cases
        .iter()
        .all(|case| case.primitive_loads.is_empty())
    {
        diagnostics.push(diag(
            "diagnostic:physics:loads-missing",
            "LOAD_INPUT_MISSING",
            "blocking",
            "linear-static preview requires at least one explicit primitive load; no hidden loads are applied",
            vec!["load_cases".to_string()],
        ));
    }
}

fn validate_ids(
    model: &PreviewModel,
    materials: &[MaterialInput],
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (entity, ids) in [
        (
            "node",
            model
                .nodes
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
        ),
        (
            "pipe",
            model
                .pipe_segments
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
        ),
        (
            "support",
            model
                .supports
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
        ),
        (
            "component",
            model
                .components
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
        ),
        (
            "material",
            materials
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
        ),
        (
            "load-case",
            model
                .load_cases
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
        ),
        (
            "primitive-load",
            model
                .load_cases
                .iter()
                .flat_map(|case| case.primitive_loads.iter())
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
        ),
        (
            "combination",
            model
                .combinations
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
        ),
    ] {
        detect_duplicate_ids(entity, ids.iter().copied(), diagnostics);
        detect_empty_ids(entity, ids.iter().copied(), diagnostics);
    }
}

fn validate_provenance(
    model: &PreviewModel,
    materials: &[MaterialInput],
    diagnostics: &mut Vec<Diagnostic>,
) {
    for node in &model.nodes {
        expect_public_preview_provenance("node", &node.id, node.provenance.as_deref(), diagnostics);
    }
    for pipe in &model.pipe_segments {
        expect_public_preview_provenance("pipe", &pipe.id, pipe.provenance.as_deref(), diagnostics);
    }
    for support in &model.supports {
        expect_public_preview_provenance(
            "support",
            &support.id,
            support.provenance.as_deref(),
            diagnostics,
        );
    }
    for component in &model.components {
        expect_public_preview_provenance(
            "component",
            &component.id,
            component.provenance.as_deref(),
            diagnostics,
        );
    }
    for material in materials {
        expect_public_preview_provenance(
            "material",
            &material.id,
            material.provenance.as_deref(),
            diagnostics,
        );
    }
    for load_case in &model.load_cases {
        expect_public_preview_provenance(
            "load-case",
            &load_case.id,
            load_case.provenance.as_deref(),
            diagnostics,
        );
        for load in &load_case.primitive_loads {
            expect_public_preview_provenance(
                "primitive-load",
                &load.id,
                load.provenance.as_deref(),
                diagnostics,
            );
        }
    }
    for combination in &model.combinations {
        expect_public_preview_provenance(
            "combination",
            &combination.id,
            combination.provenance.as_deref(),
            diagnostics,
        );
    }
}

fn validate_units(
    model: &PreviewModel,
    materials: &[MaterialInput],
    diagnostics: &mut Vec<Diagnostic>,
) {
    for material in materials {
        expect_unit(
            &material.elastic_modulus,
            Dimension::Stress,
            &format!(
                "diagnostic:unit:material:{}:elastic-modulus",
                stable_suffix(&material.id)
            ),
            vec![material.id.clone(), "elastic_modulus".to_string()],
            diagnostics,
        );
        expect_unit(
            &material.shear_modulus,
            Dimension::Stress,
            &format!(
                "diagnostic:unit:material:{}:shear-modulus",
                stable_suffix(&material.id)
            ),
            vec![material.id.clone(), "shear_modulus".to_string()],
            diagnostics,
        );
        if let Some(coefficient) = &material.thermal_expansion_coefficient {
            expect_unit(
                coefficient,
                Dimension::ThermalExpansionCoefficient,
                &format!(
                    "diagnostic:unit:material:{}:thermal-expansion",
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
    for pipe in &model.pipe_segments {
        expect_unit(
            &pipe.section.outside_diameter,
            Dimension::Length,
            &format!(
                "diagnostic:unit:pipe:{}:outside-diameter",
                stable_suffix(&pipe.id)
            ),
            vec![pipe.id.clone(), "outside_diameter".to_string()],
            diagnostics,
        );
        expect_unit(
            &pipe.section.wall_thickness,
            Dimension::Length,
            &format!(
                "diagnostic:unit:pipe:{}:wall-thickness",
                stable_suffix(&pipe.id)
            ),
            vec![pipe.id.clone(), "wall_thickness".to_string()],
            diagnostics,
        );
    }
    for support in &model.supports {
        if let Some(stiffness) = &support.stiffness {
            let expected = match parse_dof(&stiffness.dof) {
                Ok(dof) if dof.is_translational() => Some(Dimension::LinearStiffness),
                Ok(_) => Some(Dimension::RotationalStiffness),
                Err(message) => {
                    diagnostics.push(diag(
                        &format!(
                            "diagnostic:unit:support:{}:stiffness-dof",
                            stable_suffix(&support.id)
                        ),
                        "SUPPORT_STIFFNESS_DOF_INVALID",
                        "blocking",
                        message,
                        vec![support.id.clone(), stiffness.dof.clone()],
                    ));
                    None
                }
            };
            if let Some(dimension) = expected {
                expect_unit(
                    &stiffness.value,
                    dimension,
                    &format!(
                        "diagnostic:unit:support:{}:stiffness",
                        stable_suffix(&support.id)
                    ),
                    vec![support.id.clone(), "stiffness".to_string()],
                    diagnostics,
                );
            }
        }
    }
    for component in &model.components {
        if is_bend_component(component) {
            if let Some(geometry) = &component.geometry {
                if let Some(radius) = &geometry.bend_radius {
                    expect_unit(
                        radius,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit:component:{}:bend-radius",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.bend_radius".to_string()],
                        diagnostics,
                    );
                }
                if let Some(angle) = &geometry.bend_angle {
                    expect_unit(
                        angle,
                        Dimension::Angle,
                        &format!(
                            "diagnostic:unit:component:{}:bend-angle",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.bend_angle".to_string()],
                        diagnostics,
                    );
                }
            }
        }
        if is_branch_component(component) {
            if let Some(geometry) = &component.geometry {
                if let Some(size) = &geometry.branch_run_size {
                    expect_unit(
                        size,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit:component:{}:branch-run-size",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.branch_run_size".to_string()],
                        diagnostics,
                    );
                }
                if let Some(size) = &geometry.branch_header_size {
                    expect_unit(
                        size,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit:component:{}:branch-header-size",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "geometry.branch_header_size".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(angle) = &geometry.branch_connection_angle {
                    expect_unit(
                        angle,
                        Dimension::Angle,
                        &format!(
                            "diagnostic:unit:component:{}:branch-connection-angle",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "geometry.branch_connection_angle".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(area) = &geometry.branch_reinforcement_area {
                    expect_unit(
                        area,
                        Dimension::Area,
                        &format!(
                            "diagnostic:unit:component:{}:branch-reinforcement-area",
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
        }
        if is_bend_component(component) || is_branch_component(component) {
            if let Some(modifiers) = &component.modifiers {
                if let Some(sif) = &modifiers.sif_user_value {
                    expect_dimensionless_unit(
                        sif,
                        &format!(
                            "diagnostic:unit:component:{}:sif-user-value",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "modifiers.sif_user_value".to_string()],
                        diagnostics,
                    );
                }
                if let Some(sif) = &modifiers.branch_header_sif_user_value {
                    expect_dimensionless_unit(
                        sif,
                        &format!(
                            "diagnostic:unit:component:{}:branch-header-sif",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.branch_header_sif_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(sif) = &modifiers.branch_branch_sif_user_value {
                    expect_dimensionless_unit(
                        sif,
                        &format!(
                            "diagnostic:unit:component:{}:branch-branch-sif",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.branch_branch_sif_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(flexibility) = &modifiers.flexibility_factor_user_value {
                    expect_dimensionless_unit(
                        flexibility,
                        &format!(
                            "diagnostic:unit:component:{}:flexibility-factor",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.flexibility_factor_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
            }
        }
    }
    for load in model
        .load_cases
        .iter()
        .flat_map(|case| case.primitive_loads.iter())
    {
        if let Some(dimension) = expected_load_dimension(&load.dimension) {
            expect_unit(
                &load.magnitude,
                dimension,
                &format!("diagnostic:unit:load:{}:magnitude", stable_suffix(&load.id)),
                vec![load.id.clone(), "magnitude".to_string()],
                diagnostics,
            );
        }
    }
}

fn validate_components(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    let node_ids = model
        .nodes
        .iter()
        .map(|node| node.id.as_str())
        .collect::<HashSet<_>>();
    let pipe_map = model
        .pipe_segments
        .iter()
        .map(|pipe| (pipe.id.as_str(), pipe))
        .collect::<HashMap<_, _>>();
    for component in &model.components {
        if !component.node.trim().is_empty() && !node_ids.contains(component.node.as_str()) {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:component:{}:node",
                    stable_suffix(&component.id)
                ),
                "COMPONENT_NODE_UNKNOWN",
                "warning",
                "component node is not present in preview model; component stress modifier rows will not be generated for this record",
                vec![component.id.clone(), component.node.clone()],
            ));
        }
        if !is_bend_component(component) && !is_branch_component(component) {
            continue;
        }
        if component
            .mechanics_interface
            .as_ref()
            .and_then(|interface| interface.solver_consumption.as_deref())
            .map(|value| value != "mechanics_geometry_only")
            .unwrap_or(false)
        {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:component:{}:mechanics-interface",
                    stable_suffix(&component.id)
                ),
                "COMPONENT_MECHANICS_INTERFACE_UNSUPPORTED",
                "warning",
                "component stress modifier rows currently require solver_consumption=mechanics_geometry_only per DEC-045",
                vec![component.id.clone()],
            ));
        }
        if is_bend_component(component) {
            if bend_geometry_missing(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:bend-geometry",
                        stable_suffix(&component.id)
                    ),
                    "BEND_GEOMETRY_INPUT_MISSING",
                    "warning",
                    "bend/elbow component requires explicit radius, angle, plane orientation, and invented or cleared geometry source to support component provenance review",
                    vec![component.id.clone()],
                ));
            }
            if bend_modifier_missing(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:bend-modifiers",
                        stable_suffix(&component.id)
                    ),
                    "BEND_USER_MODIFIER_INPUT_MISSING",
                    "warning",
                    "bend/elbow component requires user-entered SIF, user-entered flexibility factor, and modifier source reference before stress modifier rows can be generated",
                    vec![component.id.clone()],
                ));
            }
            if bend_modifier_invalid(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:bend-modifiers-invalid",
                        stable_suffix(&component.id)
                    ),
                    "BEND_USER_MODIFIER_INPUT_INVALID",
                    "warning",
                    "bend/elbow user-entered SIF and flexibility factor must be finite positive dimensionless values before stress modifier rows can be generated",
                    vec![component.id.clone()],
                ));
            }
        }
        if is_branch_component(component) {
            if branch_geometry_missing(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:branch-geometry",
                        stable_suffix(&component.id)
                    ),
                    "BRANCH_GEOMETRY_INPUT_MISSING",
                    "warning",
                    "branch component requires explicit header pipe, branch pipe, run/header sizes, connection angle/type, reinforcement reference, and invented or cleared geometry source to support component provenance review",
                    vec![component.id.clone()],
                ));
            }
            if branch_mapping_invalid(component, &pipe_map) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:branch-mapping",
                        stable_suffix(&component.id)
                    ),
                    "BRANCH_MAPPING_INPUT_INVALID",
                    "warning",
                    "branch component header and branch pipe references must exist and terminate at the component node before stress modifier rows can be generated for those pipe sides",
                    vec![component.id.clone()],
                ));
            }
            if branch_modifier_missing(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:branch-modifiers",
                        stable_suffix(&component.id)
                    ),
                    "BRANCH_USER_MODIFIER_INPUT_MISSING",
                    "warning",
                    "branch component requires user-entered header SIF, branch SIF, flexibility factor, and modifier source reference before stress modifier rows can be generated",
                    vec![component.id.clone()],
                ));
            }
            if branch_modifier_invalid(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:branch-modifiers-invalid",
                        stable_suffix(&component.id)
                    ),
                    "BRANCH_USER_MODIFIER_INPUT_INVALID",
                    "warning",
                    "branch user-entered SIFs and flexibility factor must be finite positive dimensionless values before stress modifier rows can be generated",
                    vec![component.id.clone()],
                ));
            }
        }
    }
}

fn detect_empty_ids<'a>(
    entity: &str,
    ids: impl Iterator<Item = &'a str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for id in ids {
        if id.trim().is_empty() {
            diagnostics.push(diag(
                &format!("diagnostic:id:{entity}:empty"),
                "EMPTY_ID",
                "blocking",
                format!("{entity} ID must be explicit and non-empty"),
                vec![entity.to_string()],
            ));
        }
    }
}

fn detect_duplicate_ids<'a>(
    entity: &str,
    ids: impl Iterator<Item = &'a str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut seen = HashSet::new();
    let mut reported = HashSet::new();
    for id in ids {
        if !seen.insert(id) && reported.insert(id) {
            diagnostics.push(diag(
                &format!("diagnostic:id:{}:{}", entity, stable_suffix(id)),
                "DUPLICATE_ID",
                "blocking",
                format!("{entity} IDs must be unique within the preview mechanics model"),
                vec![id.to_string()],
            ));
        }
    }
}

fn expect_public_preview_provenance(
    entity: &str,
    id: &str,
    provenance: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = provenance else {
        diagnostics.push(provenance_diag(entity, id));
        return;
    };
    let normalized = value.to_ascii_lowercase();
    if !(normalized.contains("invented") || normalized.contains("cleared")) {
        diagnostics.push(provenance_diag(entity, id));
    }
}

fn provenance_diag(entity: &str, id: &str) -> Diagnostic {
    diag(
        &format!("diagnostic:provenance:{entity}:{}", stable_suffix(id)),
        "PROVENANCE_INPUT_MISSING",
        "blocking",
        format!(
            "{entity} record requires explicit invented or cleared provenance for public preview mechanics"
        ),
        vec![id.to_string()],
    )
}

fn expect_unit(
    quantity: &Quantity,
    dimension: Dimension,
    diagnostic_id: &str,
    affected_refs: Vec<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Err(message) = unit_symbol_matches_dimension(&quantity.unit, dimension) {
        let canonical = canonical_unit_symbol(dimension).unwrap_or("TBD");
        diagnostics.push(diag(
            diagnostic_id,
            "UNIT_INPUT_INVALID",
            "blocking",
            format!(
                "preview mechanics input requires a unit compatible with {} (canonical {canonical}); got {}: {message}",
                dimension.as_str(),
                quantity.unit
            ),
            affected_refs,
        ));
    }
}

fn expect_dimensionless_unit(
    quantity: &Quantity,
    diagnostic_id: &str,
    affected_refs: Vec<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if matches!(quantity.unit.as_str(), "1" | "none") {
        return;
    }
    expect_unit(
        quantity,
        Dimension::Dimensionless,
        diagnostic_id,
        affected_refs,
        diagnostics,
    );
}

fn is_bend_component(component: &crate::PreviewComponent) -> bool {
    matches!(component.kind.as_str(), "bend" | "elbow")
}

fn is_branch_component(component: &crate::PreviewComponent) -> bool {
    matches!(
        component.kind.as_str(),
        "branch" | "tee" | "branch_connection"
    )
}

fn bend_geometry_missing(component: &crate::PreviewComponent) -> bool {
    let Some(geometry) = &component.geometry else {
        return true;
    };
    geometry.bend_radius.is_none()
        || geometry.bend_angle.is_none()
        || geometry
            .bend_plane_orientation
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        || geometry
            .bend_geometry_source_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
}

fn bend_modifier_missing(component: &crate::PreviewComponent) -> bool {
    let Some(modifiers) = &component.modifiers else {
        return true;
    };
    modifiers.sif_user_value.is_none()
        || modifiers.flexibility_factor_user_value.is_none()
        || modifiers
            .source_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
}

fn bend_modifier_invalid(component: &crate::PreviewComponent) -> bool {
    let Some(modifiers) = &component.modifiers else {
        return false;
    };
    [
        modifiers.sif_user_value.as_ref(),
        modifiers.flexibility_factor_user_value.as_ref(),
    ]
    .into_iter()
    .flatten()
    .any(|quantity| !quantity.value.is_finite() || quantity.value <= 0.0)
}

fn branch_geometry_missing(component: &crate::PreviewComponent) -> bool {
    let Some(geometry) = &component.geometry else {
        return true;
    };
    geometry
        .branch_header_pipe_ref
        .as_deref()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
        || geometry
            .branch_branch_pipe_ref
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        || geometry.branch_run_size.is_none()
        || geometry.branch_header_size.is_none()
        || geometry.branch_connection_angle.is_none()
        || geometry
            .branch_connection_type
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        || geometry
            .branch_reinforcement_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        || geometry
            .branch_geometry_source_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
}

fn branch_mapping_invalid(
    component: &crate::PreviewComponent,
    pipe_map: &HashMap<&str, &crate::PreviewPipe>,
) -> bool {
    let Some(geometry) = &component.geometry else {
        return false;
    };
    [
        geometry.branch_header_pipe_ref.as_deref(),
        geometry.branch_branch_pipe_ref.as_deref(),
    ]
    .into_iter()
    .flatten()
    .any(|pipe_ref| {
        pipe_map
            .get(pipe_ref)
            .map(|pipe| pipe.from != component.node && pipe.to != component.node)
            .unwrap_or(true)
    })
}

fn branch_modifier_missing(component: &crate::PreviewComponent) -> bool {
    let Some(modifiers) = &component.modifiers else {
        return true;
    };
    modifiers.branch_header_sif_user_value.is_none()
        || modifiers.branch_branch_sif_user_value.is_none()
        || modifiers.flexibility_factor_user_value.is_none()
        || modifiers
            .source_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
}

fn branch_modifier_invalid(component: &crate::PreviewComponent) -> bool {
    let Some(modifiers) = &component.modifiers else {
        return false;
    };
    [
        modifiers.branch_header_sif_user_value.as_ref(),
        modifiers.branch_branch_sif_user_value.as_ref(),
        modifiers.flexibility_factor_user_value.as_ref(),
    ]
    .into_iter()
    .flatten()
    .any(|quantity| !quantity.value.is_finite() || quantity.value <= 0.0)
}

fn validate_thermal_inputs(
    model: &PreviewModel,
    materials: &[MaterialInput],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let material_map = materials
        .iter()
        .map(|material| (material.id.as_str(), material))
        .collect::<HashMap<_, _>>();
    let pipe_map = model
        .pipe_segments
        .iter()
        .map(|pipe| (pipe.id.as_str(), pipe))
        .collect::<HashMap<_, _>>();

    for load in model
        .load_cases
        .iter()
        .flat_map(|case| case.primitive_loads.iter())
        .filter(|load| load.category == "thermal")
    {
        let LoadTargetInput::Element { pipe } = &load.target else {
            continue;
        };
        let Some(pipe_input) = pipe_map.get(pipe.as_str()) else {
            continue;
        };
        let Some(material) = material_map.get(pipe_input.material.as_str()) else {
            continue;
        };
        let Some(coefficient) = &material.thermal_expansion_coefficient else {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:thermal:material:{}:expansion-missing",
                    stable_suffix(&material.id)
                ),
                "THERMAL_EXPANSION_INPUT_MISSING",
                "blocking",
                "thermal temperature-change loads require explicit material thermal_expansion_coefficient; no hidden material defaults are applied",
                vec![load.id.clone(), pipe_input.id.clone(), material.id.clone()],
            ));
            continue;
        };
        if !coefficient.value.is_finite() {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:thermal:material:{}:expansion-invalid",
                    stable_suffix(&material.id)
                ),
                "THERMAL_EXPANSION_INPUT_INVALID",
                "blocking",
                "thermal_expansion_coefficient value must be finite for thermal preview mechanics",
                vec![load.id.clone(), pipe_input.id.clone(), material.id.clone()],
            ));
        }
    }
}

fn validate_combinations(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    let load_case_ids = model
        .load_cases
        .iter()
        .map(|case| case.id.as_str())
        .collect::<HashSet<_>>();

    for combination in &model.combinations {
        match combination.basis.as_str() {
            "mechanics" => validate_mechanics_combination(combination, &load_case_ids, diagnostics),
            "result_state_subtraction" => {
                validate_subtraction_combination(combination, &load_case_ids, diagnostics)
            }
            "range_envelope" => {
                validate_range_envelope_combination(combination, &load_case_ids, diagnostics)
            }
            _ => {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:combination:{}:basis",
                        stable_suffix(&combination.id)
                    ),
                    "LOAD_COMBINATION_BASIS_UNSUPPORTED",
                    "blocking",
                    "load combination basis must be one of the explicit closed set `mechanics`, `result_state_subtraction`, `range_envelope`; code/rule and owner-basis combinations remain private/deferred",
                    vec![combination.id.clone(), combination.basis.clone()],
                ));
            }
        }
    }
}

fn combination_shape_diag(combination: &PreviewCombination, message: &str) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:combination:{}:shape",
            stable_suffix(&combination.id)
        ),
        "LOAD_COMBINATION_SHAPE_INVALID",
        "blocking",
        message,
        vec![combination.id.clone(), combination.basis.clone()],
    )
}

fn expect_known_combination_load_case(
    combination: &PreviewCombination,
    load_case_ids: &HashSet<&str>,
    load_case_ref: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !load_case_ids.contains(load_case_ref) {
        diagnostics.push(diag(
            &format!(
                "diagnostic:combination:{}:{}:load-case",
                stable_suffix(&combination.id),
                stable_suffix(load_case_ref)
            ),
            "LOAD_COMBINATION_LOAD_CASE_UNKNOWN",
            "blocking",
            "load combination references a load case that is not present in the preview model",
            vec![combination.id.clone(), load_case_ref.to_string()],
        ));
    }
}

fn validate_mechanics_combination(
    combination: &PreviewCombination,
    load_case_ids: &HashSet<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if combination.minuend_id.is_some()
        || combination.subtrahend_id.is_some()
        || combination.operand_ids.is_some()
        || combination.mode.is_some()
    {
        diagnostics.push(combination_shape_diag(
            combination,
            "mechanics-basis combinations carry explicit terms only; minuend/subtrahend and range operand/mode fields belong to the subtraction and range-envelope bases",
        ));
    }
    if combination.terms.is_empty() {
        diagnostics.push(diag(
            &format!(
                "diagnostic:combination:{}:terms-empty",
                stable_suffix(&combination.id)
            ),
            "LOAD_COMBINATION_TERMS_EMPTY",
            "blocking",
            "load combination requires at least one explicit load-case term",
            vec![combination.id.clone()],
        ));
    }
    for term in &combination.terms {
        if !term.factor.is_finite() {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:combination:{}:{}:factor",
                    stable_suffix(&combination.id),
                    stable_suffix(&term.load_case)
                ),
                "LOAD_COMBINATION_FACTOR_INVALID",
                "blocking",
                "load combination factor must be finite and explicitly user supplied",
                vec![combination.id.clone(), term.load_case.clone()],
            ));
        }
        expect_known_combination_load_case(
            combination,
            load_case_ids,
            &term.load_case,
            diagnostics,
        );
    }
}

fn validate_subtraction_combination(
    combination: &PreviewCombination,
    load_case_ids: &HashSet<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !combination.terms.is_empty()
        || combination.operand_ids.is_some()
        || combination.mode.is_some()
    {
        diagnostics.push(combination_shape_diag(
            combination,
            "result_state_subtraction combinations carry exactly minuend_id and subtrahend_id; terms and range operand/mode fields belong to the mechanics and range-envelope bases",
        ));
    }
    let minuend = combination.minuend_id.as_deref().unwrap_or("");
    let subtrahend = combination.subtrahend_id.as_deref().unwrap_or("");
    if minuend.is_empty() || subtrahend.is_empty() {
        diagnostics.push(combination_shape_diag(
            combination,
            "result_state_subtraction combinations require explicit non-empty minuend_id and subtrahend_id load-case references",
        ));
        return;
    }
    if minuend == subtrahend {
        diagnostics.push(diag(
            &format!(
                "diagnostic:combination:{}:{}:duplicate",
                stable_suffix(&combination.id),
                stable_suffix(minuend)
            ),
            "LOAD_COMBINATION_DUPLICATE_TERM",
            "blocking",
            "result_state_subtraction requires two distinct load-case references; self-subtraction is blocked",
            vec![combination.id.clone(), minuend.to_string()],
        ));
    }
    expect_known_combination_load_case(combination, load_case_ids, minuend, diagnostics);
    expect_known_combination_load_case(combination, load_case_ids, subtrahend, diagnostics);
}

fn validate_range_envelope_combination(
    combination: &PreviewCombination,
    load_case_ids: &HashSet<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !combination.terms.is_empty()
        || combination.minuend_id.is_some()
        || combination.subtrahend_id.is_some()
    {
        diagnostics.push(combination_shape_diag(
            combination,
            "range_envelope combinations carry exactly operand_ids and mode; terms and minuend/subtrahend fields belong to the mechanics and subtraction bases",
        ));
    }
    match combination.mode.as_deref() {
        Some(mode) if RangeMode::parse_token(mode).is_some() => {}
        other => {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:combination:{}:mode",
                    stable_suffix(&combination.id)
                ),
                "LOAD_COMBINATION_RANGE_MODE_UNKNOWN",
                "blocking",
                "range_envelope combinations require an explicit mode from the closed set `min`, `max`, `min_abs`, `max_abs`",
                vec![
                    combination.id.clone(),
                    other.unwrap_or("not_present").to_string(),
                ],
            ));
        }
    }
    let operand_ids = combination.operand_ids.as_deref().unwrap_or(&[]);
    if operand_ids.is_empty() {
        diagnostics.push(diag(
            &format!(
                "diagnostic:combination:{}:operands-empty",
                stable_suffix(&combination.id)
            ),
            "LOAD_COMBINATION_OPERANDS_EMPTY",
            "blocking",
            "range_envelope combinations require at least one explicit load-case operand reference",
            vec![combination.id.clone()],
        ));
        return;
    }
    let mut seen = HashSet::new();
    for operand_id in operand_ids {
        if !seen.insert(operand_id.as_str()) {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:combination:{}:{}:duplicate",
                    stable_suffix(&combination.id),
                    stable_suffix(operand_id)
                ),
                "LOAD_COMBINATION_DUPLICATE_TERM",
                "blocking",
                "range_envelope operand references must be unique; duplicate operands are blocked",
                vec![combination.id.clone(), operand_id.clone()],
            ));
            continue;
        }
        expect_known_combination_load_case(combination, load_case_ids, operand_id, diagnostics);
    }
}

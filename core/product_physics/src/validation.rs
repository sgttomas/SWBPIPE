use crate::LoadTargetInput;
use crate::{
    canonical_unit_symbol, diag, expected_load_dimension, is_constant_effort_support,
    is_variable_spring_hanger, parse_dof, stable_suffix, support_stiffness_input,
    unit_symbol_matches_dimension, Diagnostic, MaterialInput, PreviewCombination, PreviewModel,
    Quantity, SpringHangerInput, SupportStiffnessInput,
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
    validate_spring_hangers(model, diagnostics);
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
            expect_support_stiffness_unit(&support.id, "stiffness", stiffness, diagnostics);
        }
        if let Some(hanger) = &support.hanger {
            if let Some(stiffness) = &hanger.stiffness {
                expect_support_stiffness_unit(
                    &support.id,
                    "hanger.stiffness",
                    stiffness,
                    diagnostics,
                );
            }
            for (field, quantity) in [
                ("hanger.installed_load", hanger.installed_load.as_ref()),
                ("hanger.cold_load", hanger.cold_load.as_ref()),
                ("hanger.hot_load", hanger.hot_load.as_ref()),
                ("hanger.constant_load", hanger.constant_load.as_ref()),
            ] {
                if let Some(quantity) = quantity {
                    expect_unit(
                        quantity,
                        Dimension::Force,
                        &format!(
                            "diagnostic:unit:support:{}:{}",
                            stable_suffix(&support.id),
                            stable_suffix(field)
                        ),
                        vec![support.id.clone(), field.to_string()],
                        diagnostics,
                    );
                }
            }
            for (field, quantity) in [
                ("hanger.travel_range", hanger.travel_range.as_ref()),
                ("hanger.movement_limit", hanger.movement_limit.as_ref()),
            ] {
                if let Some(quantity) = quantity {
                    expect_unit(
                        quantity,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit:support:{}:{}",
                            stable_suffix(&support.id),
                            stable_suffix(field)
                        ),
                        vec![support.id.clone(), field.to_string()],
                        diagnostics,
                    );
                }
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
        if is_rigid_component(component) {
            if let Some(geometry) = &component.geometry {
                if let Some(length) = &geometry.rigid_body_length {
                    expect_unit(
                        length,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit:component:{}:rigid-body-length",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "geometry.rigid_body_length".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(size) = &geometry.end_a_size {
                    expect_unit(
                        size,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit:component:{}:end-a-size",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.end_a_size".to_string()],
                        diagnostics,
                    );
                }
                if let Some(size) = &geometry.end_b_size {
                    expect_unit(
                        size,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit:component:{}:end-b-size",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.end_b_size".to_string()],
                        diagnostics,
                    );
                }
                if let Some(weight) = &geometry.weight {
                    expect_unit(
                        weight,
                        Dimension::Force,
                        &format!(
                            "diagnostic:unit:component:{}:weight",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.weight".to_string()],
                        diagnostics,
                    );
                }
                if let Some(cog) = &geometry.center_of_gravity {
                    expect_vector_unit(
                        cog,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit:component:{}:center-of-gravity",
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
        }
        if is_expansion_joint_component(component) {
            if let Some(geometry) = &component.geometry {
                if let Some(area) = &geometry.effective_area {
                    expect_unit(
                        area,
                        Dimension::Area,
                        &format!(
                            "diagnostic:unit:component:{}:effective-area",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.effective_area".to_string()],
                        diagnostics,
                    );
                }
                if let Some(limit) = &geometry.movement_limit {
                    expect_unit(
                        limit,
                        Dimension::Length,
                        &format!(
                            "diagnostic:unit:component:{}:movement-limit",
                            stable_suffix(&component.id)
                        ),
                        vec![component.id.clone(), "geometry.movement_limit".to_string()],
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
        if is_rigid_component(component) {
            if let Some(modifiers) = &component.modifiers {
                if let Some(scale) = &modifiers.stiffness_scaling_user_value {
                    expect_dimensionless_unit(
                        scale,
                        &format!(
                            "diagnostic:unit:component:{}:stiffness-scaling",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.stiffness_scaling_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(stiffness) = &modifiers.linear_stiffness_user_value {
                    expect_unit(
                        stiffness,
                        Dimension::LinearStiffness,
                        &format!(
                            "diagnostic:unit:component:{}:linear-stiffness",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.linear_stiffness_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(stiffness) = &modifiers.rotational_stiffness_user_value {
                    expect_unit(
                        stiffness,
                        Dimension::RotationalStiffness,
                        &format!(
                            "diagnostic:unit:component:{}:rotational-stiffness",
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
        }
        if is_expansion_joint_component(component) {
            if let Some(modifiers) = &component.modifiers {
                if let Some(stiffness) = &modifiers.axial_stiffness_user_value {
                    expect_unit(
                        stiffness,
                        Dimension::LinearStiffness,
                        &format!(
                            "diagnostic:unit:component:{}:axial-stiffness",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.axial_stiffness_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(stiffness) = &modifiers.lateral_stiffness_user_value {
                    expect_unit(
                        stiffness,
                        Dimension::LinearStiffness,
                        &format!(
                            "diagnostic:unit:component:{}:lateral-stiffness",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.lateral_stiffness_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(stiffness) = &modifiers.angular_stiffness_user_value {
                    expect_unit(
                        stiffness,
                        Dimension::RotationalStiffness,
                        &format!(
                            "diagnostic:unit:component:{}:angular-stiffness",
                            stable_suffix(&component.id)
                        ),
                        vec![
                            component.id.clone(),
                            "modifiers.angular_stiffness_user_value".to_string(),
                        ],
                        diagnostics,
                    );
                }
                if let Some(stiffness) = &modifiers.torsional_stiffness_user_value {
                    expect_unit(
                        stiffness,
                        Dimension::RotationalStiffness,
                        &format!(
                            "diagnostic:unit:component:{}:torsional-stiffness",
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

fn validate_spring_hangers(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    for support in &model.supports {
        if !is_variable_spring_hanger(support) && !is_constant_effort_support(support) {
            continue;
        }
        let Some(hanger) = support.hanger.as_ref() else {
            diagnostics.push(spring_hanger_diag(
                &support.id,
                "hanger",
                "SPRING_HANGER_DATA_MISSING",
                "blocking",
                "dedicated spring-hanger support records require a named hanger object; no generic support properties or catalog defaults are substituted",
            ));
            continue;
        };
        let hanger_type = hanger
            .hanger_type
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty());
        if hanger_type.is_none() {
            diagnostics.push(spring_hanger_diag(
                &support.id,
                "hanger.hanger_type",
                "SPRING_HANGER_TYPE_MISSING",
                "blocking",
                "spring-hanger records require hanger.hanger_type to distinguish variable spring hangers from constant-effort supports",
            ));
            continue;
        }

        if is_variable_spring_hanger(support) {
            validate_variable_spring_hanger(
                &support.id,
                hanger,
                support_stiffness_input(support),
                diagnostics,
            );
        }
        if is_constant_effort_support(support) {
            validate_constant_effort_support(&support.id, hanger, diagnostics);
        }
    }
}

fn validate_variable_spring_hanger(
    support_id: &str,
    hanger: &SpringHangerInput,
    stiffness: Option<&SupportStiffnessInput>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut accepted = true;
    if !matches!(
        hanger.hanger_type.as_deref().map(str::trim),
        Some("variable_spring_hanger" | "spring_hanger")
    ) {
        accepted = false;
        diagnostics.push(spring_hanger_diag(
            support_id,
            "hanger.hanger_type",
            "SPRING_HANGER_TYPE_INVALID",
            "blocking",
            "variable spring-hanger support requires hanger.hanger_type=variable_spring_hanger",
        ));
    }
    match stiffness {
        Some(stiffness) if positive_quantity(&stiffness.value) => {}
        Some(_) => {
            accepted = false;
            diagnostics.push(spring_hanger_diag(
                support_id,
                "hanger.stiffness",
                "SPRING_HANGER_INPUT_INVALID",
                "blocking",
                "variable spring-hanger stiffness must be a finite positive user-entered value",
            ));
        }
        None => {
            accepted = false;
            diagnostics.push(spring_hanger_diag(
                support_id,
                "hanger.stiffness",
                "SPRING_HANGER_STIFFNESS_MISSING",
                "blocking",
                "variable spring hanger requires explicit user-entered stiffness before the preview may reuse the linear spring primitive",
            ));
        }
    }
    if !positive_optional_quantity(&hanger.installed_load)
        || !positive_optional_quantity(&hanger.cold_load)
        || !positive_optional_quantity(&hanger.hot_load)
    {
        accepted = false;
        diagnostics.push(spring_hanger_diag(
            support_id,
            "hanger.installed_load,hanger.cold_load,hanger.hot_load",
            "SPRING_HANGER_LOAD_MISSING",
            "blocking",
            "variable spring hanger requires finite positive installed, cold, and hot load metadata; no preload or operating-load default is supplied",
        ));
    }
    if !positive_optional_quantity(&hanger.travel_range)
        && !positive_optional_quantity(&hanger.movement_limit)
    {
        accepted = false;
        diagnostics.push(spring_hanger_diag(
            support_id,
            "hanger.travel_range",
            "SPRING_HANGER_TRAVEL_MISSING",
            "blocking",
            "variable spring hanger requires explicit finite positive travel range or movement limit metadata",
        ));
    }
    if !has_text(&hanger.source_reference) {
        accepted = false;
        diagnostics.push(spring_hanger_diag(
            support_id,
            "hanger.source_reference",
            "SPRING_HANGER_SOURCE_MISSING",
            "blocking",
            "variable spring hanger requires an invented or user-entered source reference; no catalog or protected source is bundled",
        ));
    }
    if accepted {
        diagnostics.push(spring_hanger_diag(
            support_id,
            "hanger",
            "SPRING_HANGER_USER_DATA_REVIEWED",
            "info",
            "variable spring hanger carries explicit user-entered stiffness, installed/cold/hot load metadata, travel metadata, and source reference under DEC-049; no catalog/default values are supplied",
        ));
    }
}

fn validate_constant_effort_support(
    support_id: &str,
    hanger: &SpringHangerInput,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut accepted = true;
    if !matches!(
        hanger.hanger_type.as_deref().map(str::trim),
        Some("constant_effort_support")
    ) {
        accepted = false;
        diagnostics.push(spring_hanger_diag(
            support_id,
            "hanger.hanger_type",
            "SPRING_HANGER_TYPE_INVALID",
            "blocking",
            "constant-effort support requires hanger.hanger_type=constant_effort_support",
        ));
    }
    if !positive_optional_quantity(&hanger.constant_load) {
        accepted = false;
        diagnostics.push(spring_hanger_diag(
            support_id,
            "hanger.constant_load",
            "CONSTANT_EFFORT_LOAD_MISSING",
            "blocking",
            "constant-effort support requires explicit finite positive constant load metadata; no support load default is supplied",
        ));
    }
    if !positive_optional_quantity(&hanger.travel_range)
        && !positive_optional_quantity(&hanger.movement_limit)
    {
        accepted = false;
        diagnostics.push(spring_hanger_diag(
            support_id,
            "hanger.travel_range",
            "CONSTANT_EFFORT_TRAVEL_MISSING",
            "blocking",
            "constant-effort support requires explicit finite positive travel range or movement limit metadata",
        ));
    }
    if !has_text(&hanger.source_reference) {
        accepted = false;
        diagnostics.push(spring_hanger_diag(
            support_id,
            "hanger.source_reference",
            "CONSTANT_EFFORT_SOURCE_MISSING",
            "blocking",
            "constant-effort support requires an invented or user-entered source reference; no catalog or protected source is bundled",
        ));
    }
    if accepted {
        diagnostics.push(spring_hanger_diag(
            support_id,
            "hanger",
            "CONSTANT_EFFORT_USER_DATA_REVIEWED",
            "info",
            "constant-effort support carries explicit user-entered constant load, travel metadata, and source reference under DEC-049; no global constant-effort solve behavior or catalog default is claimed",
        ));
    }
}

fn expect_support_stiffness_unit(
    support_id: &str,
    field_path: &str,
    stiffness: &SupportStiffnessInput,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let expected = match parse_dof(&stiffness.dof) {
        Ok(dof) if dof.is_translational() => Some(Dimension::LinearStiffness),
        Ok(_) => Some(Dimension::RotationalStiffness),
        Err(message) => {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:unit:support:{}:{}-dof",
                    stable_suffix(support_id),
                    stable_suffix(field_path)
                ),
                "SUPPORT_STIFFNESS_DOF_INVALID",
                "blocking",
                message,
                vec![support_id.to_string(), stiffness.dof.clone()],
            ));
            None
        }
    };
    if let Some(dimension) = expected {
        expect_unit(
            &stiffness.value,
            dimension,
            &format!(
                "diagnostic:unit:support:{}:{}",
                stable_suffix(support_id),
                stable_suffix(field_path)
            ),
            vec![support_id.to_string(), field_path.to_string()],
            diagnostics,
        );
    }
}

fn positive_optional_quantity(quantity: &Option<Quantity>) -> bool {
    quantity.as_ref().is_some_and(positive_quantity)
}

fn positive_quantity(quantity: &Quantity) -> bool {
    quantity.value.is_finite() && quantity.value > 0.0
}

fn has_text(value: &Option<String>) -> bool {
    value
        .as_deref()
        .map(str::trim)
        .is_some_and(|value| !value.is_empty())
}

fn spring_hanger_diag(
    support_id: &str,
    field_path: &str,
    code: &str,
    severity: &str,
    message: &str,
) -> Diagnostic {
    diag(
        &format!(
            "diagnostic:spring-hanger:{}:{}",
            stable_suffix(support_id),
            stable_suffix(field_path)
        ),
        code,
        severity,
        message,
        vec![support_id.to_string(), field_path.to_string()],
    )
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
        if !is_bend_component(component)
            && !is_branch_component(component)
            && !is_rigid_component(component)
            && !is_expansion_joint_component(component)
        {
            continue;
        }
        let solver_consumption = component
            .mechanics_interface
            .as_ref()
            .and_then(|interface| interface.solver_consumption.as_deref());
        if is_bend_component(component)
            || is_branch_component(component)
            || is_rigid_component(component)
        {
            if solver_consumption
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
        }
        if is_expansion_joint_component(component)
            && solver_consumption
                .map(|value| value != "mechanics_geometry_and_user_flexibility")
                .unwrap_or(true)
        {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:component:{}:expansion-joint-interface",
                    stable_suffix(&component.id)
                ),
                "EXPANSION_JOINT_MECHANICS_INTERFACE_UNSUPPORTED",
                "warning",
                "expansion joint components require solver_consumption=mechanics_geometry_and_user_flexibility under DEC-045; pressure thrust remains load-side input evidence",
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
        if is_rigid_component(component) {
            if rigid_geometry_missing(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:rigid-geometry",
                        stable_suffix(&component.id)
                    ),
                    "RIGID_COMPONENT_GEOMETRY_INPUT_MISSING",
                    "warning",
                    "rigid/semi-rigid component requires explicit mapped pipe, body length, end sizes, weight, center of gravity, connection references, stiffness behavior reference, and invented or cleared source before component provenance review is complete",
                    vec![component.id.clone()],
                ));
            }
            if rigid_mapping_invalid(component, &pipe_map) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:rigid-mapping",
                        stable_suffix(&component.id)
                    ),
                    "RIGID_COMPONENT_MAPPING_INPUT_INVALID",
                    "warning",
                    "rigid/semi-rigid component pipe mapping must reference an existing generic frame member that terminates at the component node; no stiffness scaling is silently applied without a valid mapping",
                    vec![component.id.clone()],
                ));
            }
            if rigid_modifier_missing(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:rigid-stiffness",
                        stable_suffix(&component.id)
                    ),
                    "RIGID_COMPONENT_STIFFNESS_INPUT_MISSING",
                    "warning",
                    "rigid/semi-rigid component requires user-entered stiffness scaling, optional semi-rigid stiffness quantities, and source reference for DEC-045 mechanics_geometry_only review evidence",
                    vec![component.id.clone()],
                ));
            }
            if rigid_modifier_invalid(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:rigid-stiffness-invalid",
                        stable_suffix(&component.id)
                    ),
                    "RIGID_COMPONENT_STIFFNESS_INPUT_INVALID",
                    "warning",
                    "rigid/semi-rigid user-entered stiffness scale and stiffness quantities must be finite positive values before they can be used as mapping evidence",
                    vec![component.id.clone()],
                ));
            }
            if !rigid_geometry_missing(component)
                && !rigid_mapping_invalid(component, &pipe_map)
                && !rigid_modifier_missing(component)
                && !rigid_modifier_invalid(component)
            {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:rigid-review",
                        stable_suffix(&component.id)
                    ),
                    "RIGID_COMPONENT_STIFFNESS_SCALING_REVIEWED",
                    "info",
                    "rigid/semi-rigid component carries user-entered dimensions, weight, center of gravity, mapping, stiffness scaling, semi-rigid stiffness quantities, and source notes under DEC-045 mechanics_geometry_only; no protected/default component values are supplied",
                    vec![component.id.clone()],
                ));
            }
        }
        if is_expansion_joint_component(component) {
            if expansion_joint_geometry_missing(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:expansion-joint-geometry",
                        stable_suffix(&component.id)
                    ),
                    "EXPANSION_JOINT_GEOMETRY_INPUT_MISSING",
                    "warning",
                    "expansion joint requires explicit mapped pipe, effective pressure area, movement limit, hardware reference, manufacturer reference, pressure-thrust handling reference, and invented or cleared source before provenance review is complete",
                    vec![component.id.clone()],
                ));
            }
            if expansion_joint_mapping_invalid(component, &pipe_map) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:expansion-joint-mapping",
                        stable_suffix(&component.id)
                    ),
                    "EXPANSION_JOINT_MAPPING_INPUT_INVALID",
                    "warning",
                    "expansion joint pipe mapping must reference an existing frame member that terminates at the component node before user-stiffness macro-element evidence can be generated",
                    vec![component.id.clone()],
                ));
            }
            if expansion_joint_modifier_missing(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:expansion-joint-stiffness",
                        stable_suffix(&component.id)
                    ),
                    "EXPANSION_JOINT_STIFFNESS_INPUT_MISSING",
                    "warning",
                    "expansion joint requires user-entered axial, lateral, angular, and torsional stiffness quantities plus source reference under DEC-045; no manufacturer or code default is supplied",
                    vec![component.id.clone()],
                ));
            }
            if expansion_joint_modifier_invalid(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:expansion-joint-stiffness-invalid",
                        stable_suffix(&component.id)
                    ),
                    "EXPANSION_JOINT_STIFFNESS_INPUT_INVALID",
                    "warning",
                    "expansion joint user-entered stiffness quantities must be finite positive values before they can be used as macro-element input evidence",
                    vec![component.id.clone()],
                ));
            }
            if !expansion_joint_geometry_missing(component)
                && !expansion_joint_mapping_invalid(component, &pipe_map)
                && !expansion_joint_modifier_missing(component)
                && !expansion_joint_modifier_invalid(component)
                && solver_consumption == Some("mechanics_geometry_and_user_flexibility")
            {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:expansion-joint-review",
                        stable_suffix(&component.id)
                    ),
                    "EXPANSION_JOINT_USER_STIFFNESS_REVIEWED",
                    "info",
                    "expansion joint carries user-entered stiffnesses, effective pressure area, movement limit, hardware/manufacturer provenance, and load-side pressure-thrust handling evidence under DEC-045; no protected/default manufacturer values are supplied",
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

fn expect_vector_unit(
    quantity: &crate::VectorQuantity,
    dimension: Dimension,
    diagnostic_id: &str,
    affected_refs: Vec<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let components = [
        Quantity {
            value: quantity.x,
            unit: quantity.unit.clone(),
        },
        Quantity {
            value: quantity.y,
            unit: quantity.unit.clone(),
        },
        Quantity {
            value: quantity.z,
            unit: quantity.unit.clone(),
        },
    ];
    for component in components {
        expect_unit(
            &component,
            dimension,
            diagnostic_id,
            affected_refs.clone(),
            diagnostics,
        );
    }
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

fn is_rigid_component(component: &crate::PreviewComponent) -> bool {
    matches!(
        component.kind.as_str(),
        "valve" | "flange" | "reducer" | "rigid" | "specialty"
    )
}

fn is_expansion_joint_component(component: &crate::PreviewComponent) -> bool {
    component.kind == "expansion_joint"
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

fn rigid_geometry_missing(component: &crate::PreviewComponent) -> bool {
    let Some(geometry) = &component.geometry else {
        return true;
    };
    geometry
        .rigid_pipe_ref
        .as_deref()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
        || geometry.rigid_body_length.is_none()
        || geometry.end_a_size.is_none()
        || geometry.end_b_size.is_none()
        || geometry.weight.is_none()
        || geometry.center_of_gravity.is_none()
        || geometry
            .connection_end_a_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        || geometry
            .connection_end_b_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        || geometry
            .stiffness_behavior_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        || geometry
            .rigid_component_source_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
}

fn rigid_mapping_invalid(
    component: &crate::PreviewComponent,
    pipe_map: &HashMap<&str, &crate::PreviewPipe>,
) -> bool {
    let Some(geometry) = &component.geometry else {
        return false;
    };
    let Some(pipe_ref) = geometry
        .rigid_pipe_ref
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    else {
        return false;
    };
    pipe_map
        .get(pipe_ref)
        .map(|pipe| pipe.from != component.node && pipe.to != component.node)
        .unwrap_or(true)
}

fn rigid_modifier_missing(component: &crate::PreviewComponent) -> bool {
    let Some(modifiers) = &component.modifiers else {
        return true;
    };
    modifiers.stiffness_scaling_user_value.is_none()
        || modifiers.linear_stiffness_user_value.is_none()
        || modifiers.rotational_stiffness_user_value.is_none()
        || modifiers
            .source_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
}

fn rigid_modifier_invalid(component: &crate::PreviewComponent) -> bool {
    let Some(modifiers) = &component.modifiers else {
        return false;
    };
    [
        modifiers.stiffness_scaling_user_value.as_ref(),
        modifiers.linear_stiffness_user_value.as_ref(),
        modifiers.rotational_stiffness_user_value.as_ref(),
    ]
    .into_iter()
    .flatten()
    .any(|quantity| !quantity.value.is_finite() || quantity.value <= 0.0)
}

fn expansion_joint_geometry_missing(component: &crate::PreviewComponent) -> bool {
    let Some(geometry) = &component.geometry else {
        return true;
    };
    geometry
        .expansion_joint_pipe_ref
        .as_deref()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
        || geometry.effective_area.is_none()
        || geometry.movement_limit.is_none()
        || geometry
            .hardware_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        || geometry
            .manufacturer_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        || geometry
            .pressure_thrust_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        || geometry
            .expansion_joint_source_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
}

fn expansion_joint_mapping_invalid(
    component: &crate::PreviewComponent,
    pipe_map: &HashMap<&str, &crate::PreviewPipe>,
) -> bool {
    let Some(geometry) = &component.geometry else {
        return false;
    };
    let Some(pipe_ref) = geometry
        .expansion_joint_pipe_ref
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    else {
        return false;
    };
    pipe_map
        .get(pipe_ref)
        .map(|pipe| pipe.from != component.node && pipe.to != component.node)
        .unwrap_or(true)
}

fn expansion_joint_modifier_missing(component: &crate::PreviewComponent) -> bool {
    let Some(modifiers) = &component.modifiers else {
        return true;
    };
    modifiers.axial_stiffness_user_value.is_none()
        || modifiers.lateral_stiffness_user_value.is_none()
        || modifiers.angular_stiffness_user_value.is_none()
        || modifiers.torsional_stiffness_user_value.is_none()
        || modifiers
            .source_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
}

fn expansion_joint_modifier_invalid(component: &crate::PreviewComponent) -> bool {
    let Some(modifiers) = &component.modifiers else {
        return false;
    };
    [
        modifiers.axial_stiffness_user_value.as_ref(),
        modifiers.lateral_stiffness_user_value.as_ref(),
        modifiers.angular_stiffness_user_value.as_ref(),
        modifiers.torsional_stiffness_user_value.as_ref(),
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

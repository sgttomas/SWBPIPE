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
    validate_connected_input_contracts(model, diagnostics);
    validate_units(model, materials, diagnostics);
    validate_spring_hangers(model, diagnostics);
    validate_components(model, diagnostics);
    validate_directional_sifs(model, diagnostics);
    validate_thermal_inputs(model, materials, diagnostics);
    validate_combinations(model, diagnostics);
    validate_model_settings(model, diagnostics);
}

/// REQ-04-06-012: the solve refuses a malformed `model_settings.node_proximity_tolerance`
/// with the accessor's code and message, as the document validator does, so the
/// topology check never runs on an invalid tolerance.
fn validate_model_settings(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    if let Err(error) = model.node_proximity_tolerance_m() {
        diagnostics.push(diag(
            "diagnostic:model-settings:node-proximity-tolerance",
            error.code,
            "blocking",
            error.message,
            vec![crate::model_settings::NODE_PROXIMITY_TOLERANCE_PATH.to_string()],
        ));
    }
    // Model document 0.8.0: `model_settings.shear_model` is refused by name when it is not one of
    // the two names, or on a document with `pressure_contract`. Nothing is then solved.
    let authored = model.model_settings.as_ref().and_then(|settings| settings.shear_model);
    if let Some(error) = authored.and_then(|setting| crate::model_settings::shear_model_refusal(setting, model.pressure_contract.is_some())) {
        diagnostics.push(diag(
            "diagnostic:model-settings:shear-model",
            error.code,
            "blocking",
            error.message,
            vec![crate::model_settings::SHEAR_MODEL_PATH.to_string()],
        ));
    }
}

// Local solve readiness only: records remain deserializable and editable.
fn validate_connected_input_contracts(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    for case in &model.load_cases {
        for load in &case.primitive_loads {
            if load.category == "hydrotest" && load.dimension == "pressure" {
                diagnostics.push(diag(
                    &format!("diagnostic:load:{}:hydrotest-pressure", stable_suffix(&load.id)),
                    "HYDROTEST_PRESSURE_UNSUPPORTED", "blocking",
                    "hydrotest pressure is not yet supported by the mechanics solver; keep this record for editing, but solve is blocked until a supported test-pressure state is implemented",
                    vec![load.id.clone(), case.id.clone()],
                ));
            }
        }
    }
    for support in &model.supports {
        let constant_effort = is_constant_effort_support(support);
        if support.family.as_deref() == Some("spring") && constant_effort {
            diagnostics.push(diag(
                &format!("diagnostic:support:{}:family-hanger", stable_suffix(&support.id)),
                "SUPPORT_FAMILY_HANGER_CONFLICT", "blocking",
                "spring family conflicts with constant-effort hanger type; choose one support law so stiffness and applied force cannot be silently substituted",
                vec![support.id.clone()],
            ));
        }
        // Match build_model: constant-effort records never contribute stiffness.
        let spring_family = !constant_effort
            && (support.family.as_deref() == Some("spring") || is_variable_spring_hanger(support));
        // Model document 0.9.0 (§2.4, r2 F5): an explicit snubber's stiffness is dynamic, and
        // its own rules are `document_members`'. An inferred family is never a snubber.
        let snubber = support.family.as_deref() == Some("snubber");
        if let Some(stiffness) = support_stiffness_input(support).filter(|_| !snubber) {
            if !spring_family {
                diagnostics.push(diag(
                    &format!("diagnostic:support:{}:unused-stiffness", stable_suffix(&support.id)),
                    "SUPPORT_STIFFNESS_FAMILY_UNSUPPORTED", "blocking",
                    "finite stiffness is only consumed by spring or variable spring hanger families; select a supported spring family or remove the unused stiffness",
                    vec![support.id.clone()],
                ));
            } else if support
                .restraints
                .iter()
                .any(|dof| parse_dof(dof).ok() != parse_dof(&stiffness.dof).ok())
            {
                diagnostics.push(diag(
                    &format!("diagnostic:support:{}:spring-axis", stable_suffix(&support.id)),
                    "SUPPORT_SPRING_AXIS_CONFLICT", "blocking",
                    "spring restraints declare its acting axis and must agree with the stiffness DOF; model additional rigid guides as separate support records",
                    vec![support.id.clone()],
                ));
            }
        }
    }
    for nonlinear_support in &model.supports {
        let Some(nonlinear) = &nonlinear_support.nonlinear else {
            continue;
        };
        let Ok(nonlinear_dof) = parse_dof(&nonlinear.dof) else {
            continue;
        };
        if nonlinear.behavior == "gap" && !nonlinear_dof.is_translational() {
            diagnostics.push(diag(
                &format!("diagnostic:support:{}:rotational-gap", nonlinear_support.id),
                "NONLINEAR_ROTATIONAL_GAP_UNSUPPORTED",
                "blocking",
                "the current gap input is a length; rotational gaps require a qualified angular input and recovery contract. This record remains editable, but solving is blocked",
                vec![nonlinear_support.id.clone(), nonlinear.dof.clone()],
            ));
        }
        // DEL-04-01 kd5-nonlinear-active-set.md (manager's ruling, 2026-10-12):
        // a rotational friction support would apply mu*N, a force, as a
        // moment; it is unit-inconsistent and refused by name.
        if nonlinear.behavior == "friction" && !nonlinear_dof.is_translational() {
            diagnostics.push(diag(
                &format!("diagnostic:support:{}:rotational-friction", nonlinear_support.id),
                "NONLINEAR_ROTATIONAL_FRICTION_UNSUPPORTED",
                "blocking",
                "a friction support acts on a translational DOF: its force mu*N cannot act on a rotation. This record remains editable, but solving is blocked",
                vec![nonlinear_support.id.clone(), nonlinear.dof.clone()],
            ));
        }
        for linear_support in model
            .supports
            .iter()
            .filter(|s| s.node == nonlinear_support.node)
        {
            // Match assembly semantics: a constant-effort restraint is a force axis,
            // not a rigid constraint. A separate finite spring can act in parallel
            // with contact; same-record overlap cannot yet attribute both actions.
            if is_constant_effort_support(linear_support) {
                continue;
            }
            let spring_family = linear_support.family.as_deref() == Some("spring")
                || is_variable_spring_hanger(linear_support);
            let overlap = if spring_family {
                linear_support.id == nonlinear_support.id
                    && support_stiffness_input(linear_support)
                        .and_then(|input| parse_dof(&input.dof).ok())
                        .or_else(|| {
                            linear_support
                                .restraints
                                .first()
                                .and_then(|dof| parse_dof(dof).ok())
                        })
                        == Some(nonlinear_dof)
            } else {
                linear_support
                    .restraints
                    .iter()
                    .any(|dof| parse_dof(dof).ok() == Some(nonlinear_dof))
            };
            if overlap {
                diagnostics.push(diag(
                    &format!("diagnostic:support:{}:{}:dof-overlap", stable_suffix(&nonlinear_support.id), stable_suffix(&linear_support.id)),
                    "SUPPORT_LINEAR_NONLINEAR_DOF_CONFLICT", "blocking",
                    format!("rigid restraint or same-record spring {} overlaps nonlinear support {} on node {} DOF {}; remove the overlap or use separate spring/contact records; disjoint linear restraints are retained", linear_support.id, nonlinear_support.id, nonlinear_support.node, nonlinear.dof),
                    vec![linear_support.id.clone(), nonlinear_support.id.clone(), nonlinear_support.node.clone()],
                ));
            }
        }
    }
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
    // An explicit empty pressure-region list (`[]`, the exact route's
    // "unpressurized") applies nothing.
    let case_applies_nothing = |case: &crate::PreviewLoadCase| {
        case.primitive_loads.is_empty()
            && case.equivalent_static.is_none()
            && case.pressure_regions.as_ref().is_none_or(|regions| regions.is_empty())
    };
    // Load-state cases are judged on their resolved actions
    // (`case_state::resolve`): a boundary motion or member eigenstrain
    // applies an action with no primitive load.
    if crate::case_state::is_load_state(model) {
        return;
    }
    let model_loads_every_case = model_loads_every_case(model);
    let all_cases_empty = if model_loads_every_case {
        model.load_cases.iter().all(|case| {
            case.primitive_loads.is_empty()
                && case.equivalent_static.is_none()
                && case.pressure_regions.is_none()
        })
    } else {
        model.load_cases.iter().all(case_applies_nothing)
    };
    if all_cases_empty {
        diagnostics.push(diag(
            "diagnostic:physics:loads-missing",
            "LOAD_INPUT_MISSING",
            "blocking",
            "linear-static preview requires at least one explicit primitive load; no hidden loads are applied",
            vec!["load_cases".to_string()],
        ));
    } else if !model_loads_every_case {
        // Silent-zero guard (DEL-04-07 Design/pressure-contract.md, "Load
        // cases that apply nothing"): an empty case among loaded ones
        // publishes an all-zero result; it may be deliberate (a cold operand
        // of a range), so it is reported, not refused.
        for case in model.load_cases.iter().filter(|case| case_applies_nothing(case)) {
            diagnostics.push(diag(
                &format!("diagnostic:load-case:{}:applies-no-action", stable_suffix(&case.id)),
                "LOAD_CASE_APPLIES_NO_ACTION",
                "warning",
                "the load case applies no action: it has no primitive load, equivalent-static load or pressure region, so its published result is all zero; add the loads the case should apply if that is not intended",
                vec![case.id.clone()],
            ));
        }
    }
}

/// Whether a model-level action loads every case (DEL-04-07 Design
/// pressure-contract.md, "Load cases that apply nothing"): a constant-effort
/// support, or an objective connector with a preload — K q_ref ≠ 0, decided
/// exactly as the kernel's `ObjectiveConnector::stress_free` decides it, or a
/// tie rod with a nonzero reference extension. A `q_ref` in K's null space
/// loads nothing (K may be positive semidefinite).
pub(crate) fn model_loads_every_case(model: &PreviewModel) -> bool {
    model.supports.iter().any(crate::is_constant_effort_support)
        || model.components.iter().any(connector_preloads)
}

fn connector_preloads(component: &crate::PreviewComponent) -> bool {
    use open_pipe_stress_frame_kernel::connector::symmetric_from_upper;
    use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
    // A connector that does not decode is refused by the connector findings.
    let Some(spec) = crate::joint::connector_spec(component) else {
        return false;
    };
    // A tie rod pretensions when its decoded (SI) reference extension is
    // nonzero, the value the kernel's rod load reads.
    if spec.tie_rods.iter().any(|rod| rod.input.reference_extension != 0.0) {
        return true;
    }
    // The kernel's decode: K = D⁻¹HD⁻¹, a translation index divides by Ls.
    let h = symmetric_from_upper(&spec.stiffness.upper_triangle);
    let ls = spec.stiffness.translation_scale;
    let k = |i: usize, j: usize| {
        let mut value = h[i][j];
        if i < 3 {
            value /= ls;
        }
        if j < 3 {
            value /= ls;
        }
        value
    };
    (0..6).any(|row| {
        let mut sum = ExactAccumulator::new();
        (0..6).all(|column| sum.add_product(k(row, column), spec.q_ref[column]).is_ok())
            && !sum.is_zero()
    })
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
        expect_local_input_provenance("node", &node.id, node.provenance.as_deref(), diagnostics);
    }
    for pipe in &model.pipe_segments {
        expect_local_input_provenance("pipe", &pipe.id, pipe.provenance.as_deref(), diagnostics);
    }
    for support in &model.supports {
        expect_local_input_provenance(
            "support",
            &support.id,
            support.provenance.as_deref(),
            diagnostics,
        );
    }
    for component in &model.components {
        expect_local_input_provenance(
            "component",
            &component.id,
            component.provenance.as_deref(),
            diagnostics,
        );
    }
    for material in materials {
        expect_local_input_provenance(
            "material",
            &material.id,
            material.provenance.as_deref(),
            diagnostics,
        );
    }
    for load_case in &model.load_cases {
        expect_local_input_provenance(
            "load-case",
            &load_case.id,
            load_case.provenance.as_deref(),
            diagnostics,
        );
        for load in &load_case.primitive_loads {
            expect_local_input_provenance(
                "primitive-load",
                &load.id,
                load.provenance.as_deref(),
                diagnostics,
            );
        }
        if let Some(equivalent_static) = &load_case.equivalent_static {
            expect_local_input_provenance(
                "equivalent-static-generation",
                &load_case.id,
                equivalent_static.provenance.as_deref(),
                diagnostics,
            );
        }
    }
    for combination in &model.combinations {
        expect_local_input_provenance(
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
        if let Some(shear_modulus) = &material.shear_modulus {
            expect_unit(
                shear_modulus,
                Dimension::Stress,
                &format!(
                    "diagnostic:unit:material:{}:shear-modulus",
                    stable_suffix(&material.id)
                ),
                vec![material.id.clone(), "shear_modulus".to_string()],
                diagnostics,
            );
        } else if !crate::pressure_runtime::is_exact(model) {
            diagnostics.push(diag(
                "diagnostic:material:missing-g",
                "MATERIAL_INPUT_MISSING",
                "blocking",
                "legacy material requires explicit shear_modulus",
                vec![material.id.clone()],
            ));
        }
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
        // §8.30 correction 3 (RT-S3): only the stiffness is consumed; each
        // entered load is review evidence and is never applied as preload.
        for (field, name, quantity) in [
            ("hanger.installed_load", "installed load", &hanger.installed_load),
            ("hanger.cold_load", "cold load", &hanger.cold_load),
            ("hanger.hot_load", "hot load", &hanger.hot_load),
        ] {
            if quantity.as_ref().is_some_and(|value| value.value != 0.0) {
                diagnostics.push(spring_hanger_diag(
                    support_id,
                    field,
                    SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD,
                    "warning",
                    &format!("variable spring hanger {support_id}: its {name} ({field}) is not applied as preload; results treat the hanger as a linear spring from zero load, whose force is its stiffness times the displacement at its DOF, and the entered load remains review evidence only"),
                ));
            }
        }
    }
}

/// §8.30 correction 3 (RT-S3): a variable spring hanger's entered installed,
/// cold or hot load is not applied as preload (one warning per support and
/// field). Hanger sizing (RT-A11) and preload (RT-S3) lift it.
pub(crate) const SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD: &str =
    "SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD";

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
    // W-8 (DEL-04-06 Design/model-topology-checks.md): members meeting at each node.
    let mut node_degree = HashMap::<&str, usize>::new();
    for pipe in &model.pipe_segments {
        *node_degree.entry(pipe.from.as_str()).or_default() += 1;
        *node_degree.entry(pipe.to.as_str()).or_default() += 1;
    }
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
            // DEC-070: bends may also declare the curved-bend macro-element
            // realization; every input insufficiency for that mode blocks in
            // build_model rather than warning here.
            let curved_bend_macro_realized = is_bend_component(component)
                && solver_consumption == Some(crate::DEC_070_CURVED_BEND_SOLVER_CONSUMPTION);
            if !curved_bend_macro_realized
                && solver_consumption
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
        if is_bend_component(component) {
            if bend_geometry_missing(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:bend-geometry",
                        stable_suffix(&component.id)
                    ),
                    "BEND_GEOMETRY_INPUT_MISSING",
                    "warning",
                    "bend/elbow component requires explicit radius, angle (unless realized as a curved bend), and invented or cleared geometry source to support component provenance review",
                    vec![component.id.clone()],
                ));
            }
            if let Some(message) = bend_geometry_only_on_preview(model, component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:bend-geometry-only",
                        stable_suffix(&component.id)
                    ),
                    BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD,
                    "warning",
                    message,
                    vec![component.id.clone()],
                ));
            }
            if crate::is_curved_bend_macro_component(component) && bend_modifier_missing(component) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:bend-modifiers",
                        stable_suffix(&component.id)
                    ),
                    "BEND_USER_MODIFIER_INPUT_MISSING",
                    "warning",
                    if bend_scalar_sif_on_realized_arc(component) {
                        "bend/elbow component realized as a curved bend: its scalar sif_user_value is not applied to realized arcs; the in-plane and out-of-plane SIF pair (sif_in_plane_user_value, sif_out_of_plane_user_value), user-entered flexibility factor, and modifier source reference are required before intensified stress rows can be generated"
                    } else {
                        "bend/elbow component requires user-entered SIF, user-entered flexibility factor, and modifier source reference before stress modifier rows can be generated"
                    },
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
            let degree = node_degree
                .get(component.node.as_str())
                .copied()
                .unwrap_or(0);
            // A component without a node is reported by the node checks above.
            if !component.node.trim().is_empty() && branch_topology_invalid(component, degree) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:component:{}:branch-topology",
                        stable_suffix(&component.id)
                    ),
                    "BRANCH_TOPOLOGY_INVALID",
                    "warning",
                    format!(
                        "a branch component's node must join exactly three pipes, and its header and branch pipes must differ; node {} joins {degree}",
                        component.node
                    ),
                    vec![component.id.clone(), component.node.clone()],
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
            // W-15 (DEL-03-05 Design `rigid-component-stiffness-scaling.md`
            // §2.3, §2.6): the eligibility predicate formation also reads; one
            // info for an applied scale, or one warning naming the first
            // unmet condition.
            crate::rigid_component::append_scaling_diagnostics(component, &pipe_map, diagnostics);
        }
    }
    // W-15 §2.3: the three mapping conflicts, blocking by name.
    crate::rigid_component::append_mapping_conflicts(model, diagnostics);
    // T4-U3 (D-4): the annotation disclosure, pressure-free route only. The
    // legacy joint's refusal is the profile gate's; no joint row is produced.
    crate::joint::disclose_annotation_joints(model, diagnostics);
    // T4-U3: v3 objective connectors carry no joint temperature law.
    crate::joint::disclose_connector_temperature_law(model, diagnostics);
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

fn expect_local_input_provenance(
    entity: &str,
    id: &str,
    provenance: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = provenance else {
        diagnostics.push(provenance_diag(entity, id));
        return;
    };
    if value.trim().is_empty() {
        diagnostics.push(provenance_diag(entity, id));
    }
}

fn provenance_diag(entity: &str, id: &str) -> Diagnostic {
    diag(
        &format!("diagnostic:provenance:{entity}:{}", stable_suffix(id)),
        "PROVENANCE_INPUT_MISSING",
        "blocking",
        format!(
            "{entity} record requires a nonempty source reference for local mechanics traceability; source text does not establish publication clearance"
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
    crate::rigid_component::is_rigid_family(component)
}

fn is_expansion_joint_component(component: &crate::PreviewComponent) -> bool {
    component.kind == "expansion_joint"
}

// T4-U1 (T4-I11 D-B): `bend_plane_orientation` is no longer required; the
// span's y_reference fixes the plane and the bow side. A realized bend's angle
// follows from R and its nodes, so only geometry-only bends need `bend_angle`.
fn bend_geometry_missing(component: &crate::PreviewComponent) -> bool {
    let Some(geometry) = &component.geometry else {
        return true;
    };
    geometry.bend_radius.is_none()
        || (geometry.bend_angle.is_none() && !crate::is_curved_bend_macro_component(component))
        || geometry
            .bend_geometry_source_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
}

/// A realized curved bend's modifiers (geometry-only markers take
/// [`BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD`] instead).
fn bend_modifier_missing(component: &crate::PreviewComponent) -> bool {
    let Some(modifiers) = &component.modifiers else {
        return true;
    };
    // The user SIF a realized arc's consumer uses is the whole directional pair
    // (the directional rows, `u4b_rows::directional_pair`); the scalar is never
    // applied to a realized arc. A half pair is refused by
    // `validate_directional_sifs` and supplies none.
    !bend_directional_pair_present(modifiers)
        || modifiers.flexibility_factor_user_value.is_none()
        || modifiers
            .source_reference
            .as_deref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
}

/// DEL-03-03: a geometry-only bend on preview-physics-2 is solved as a straight
/// chord. Nothing applies its k (only a realized arc's stiffness does); its
/// positive scalar SIF is applied as equal-factor intensification when it is a
/// `mechanics_geometry_only` marker (`preview_physics::associations`). On the
/// exact route the marker is refused by name (`EXACT_PRESSURE_FAMILY_NOT_ADMITTED`).
pub(crate) const BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD: &str = "BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD";
fn bend_geometry_only_on_preview(model: &PreviewModel, component: &crate::PreviewComponent) -> Option<&'static str> {
    if crate::is_curved_bend_macro_component(component) || crate::pressure_runtime::is_exact(model) {
        return None;
    }
    let scalar = component
        .modifiers
        .as_ref()
        .and_then(|modifiers| modifiers.sif_user_value.as_ref())
        .filter(|sif| sif.value.is_finite() && sif.value > 0.0);
    let marker = crate::component_solver_consumption(component, "mechanics_geometry_only") == "mechanics_geometry_only";
    Some(match (scalar.is_some(), marker) {
        (true, true) => "bend/elbow component is geometry only: it is solved as a straight chord, its flexibility factor k is not applied, and its scalar SIF is applied as equal-factor intensification when the run solves; realize the bend (mechanics_interface.solver_consumption curved_bend_macro_element) to apply k",
        (false, _) => "bend/elbow component is geometry only: it is solved as a straight chord, its flexibility factor k is not applied, and no positive scalar SIF was entered, so no intensified measure is published for it; realize the bend (mechanics_interface.solver_consumption curved_bend_macro_element) to apply k",
        (true, false) => "bend/elbow component is geometry only: it is solved as a straight chord, its flexibility factor k is not applied, and its scalar SIF is not applied under its mechanics_interface.solver_consumption; realize the bend (mechanics_interface.solver_consumption curved_bend_macro_element) to apply k",
    })
}

fn bend_directional_pair_present(modifiers: &crate::ComponentModifierInput) -> bool {
    modifiers.sif_in_plane_user_value.is_some() && modifiers.sif_out_of_plane_user_value.is_some()
}

/// A realized curved bend carrying the scalar SIF without the whole pair: its
/// only SIF is one no published row of a realized arc applies.
fn bend_scalar_sif_on_realized_arc(component: &crate::PreviewComponent) -> bool {
    crate::is_curved_bend_macro_component(component)
        && component.modifiers.as_ref().is_some_and(|modifiers| {
            modifiers.sif_user_value.is_some() && !bend_directional_pair_present(modifiers)
        })
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

/// W-8: a tee joins a run in, a run out and a branch; a header pipe that is
/// also the branch pipe names one side twice.
fn branch_topology_invalid(component: &crate::PreviewComponent, degree: usize) -> bool {
    let same_pipe = component.geometry.as_ref().is_some_and(|geometry| {
        matches!(
            (
                geometry.branch_header_pipe_ref.as_deref(),
                geometry.branch_branch_pipe_ref.as_deref()
            ),
            (Some(header), Some(branch)) if !header.trim().is_empty() && header == branch
        )
    });
    degree != 3 || same_pipe
}

/// W-8 (owner ruling, 2026-10-23: topology findings warn, not refuse, and the
/// warning "must state plainly that the component's branch stress factors are
/// still applied to the named pipes"). Called once, where a solved preview
/// envelope is assembled, after every refusal that can follow publication: each
/// BRANCH_TOPOLOGY_INVALID warning gains the sides whose intensified rows that
/// envelope publishes. They are read from the publication's own records
/// (`preview_physics::render`'s `intensified_measures`, one beside each published
/// row, from `associations` over the straight members only), so the statement
/// names exactly the published sides. A refused envelope, and an exact-route one
/// (which publishes no intensified row), keep the topology text alone.
pub(crate) fn state_applied_branch_factors(
    model_components: &[crate::PreviewComponent],
    preview_cases: &[serde_json::Value],
    diagnostics: &mut [Diagnostic],
) {
    for component in model_components {
        let Some(warning) = diagnostics.iter_mut().find(|d| {
            d.code == "BRANCH_TOPOLOGY_INVALID" && d.affected_refs.first() == Some(&component.id)
        }) else {
            continue;
        };
        let mut applied: Vec<String> = Vec::new();
        for (role, side) in [("branch_header", "header"), ("branch_branch", "branch")] {
            for record in preview_cases
                .iter()
                .filter_map(|case| case["intensified_measures"].as_array())
                .flatten()
                .filter(|record| record["component_id"] == component.id.as_str() && record["factor_role"] == role)
            {
                let named = format!("{side} SIF to {}", record["pipe_id"].as_str().unwrap_or_default());
                if !applied.contains(&named) {
                    applied.push(named);
                }
            }
        }
        if !applied.is_empty() {
            let sides = applied.join(", ");
            warning.message.push_str(&format!("; its branch stress factors are still applied ({sides})"));
        }
    }
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

fn validate_thermal_inputs(
    model: &PreviewModel,
    materials: &[MaterialInput],
    diagnostics: &mut Vec<Diagnostic>,
) {
    // Exact profile validates alpha at the actual selected common E/nu basis.
    // Requiring base alpha here would reject a complete selected point.
    if crate::pressure_runtime::is_exact(model) {
        return;
    }
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
            // Model document 0.9.0 (absolute-sum-srss §2): the unsigned bases take the
            // mechanics shape; the solve withholds them in S1 after this validation.
            "mechanics" | "absolute_sum" | "srss" => validate_mechanics_combination(combination, &load_case_ids, diagnostics),
            "result_state_subtraction" => validate_subtraction_combination(combination, diagnostics),
            "range_envelope" => validate_range_envelope_combination(combination, diagnostics),
            _ => {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:combination:{}:basis",
                        stable_suffix(&combination.id)
                    ),
                    "LOAD_COMBINATION_BASIS_UNSUPPORTED",
                    "blocking",
                    "load combination basis must be one of the explicit closed set `mechanics`, `result_state_subtraction`, `range_envelope`, `absolute_sum`, `srss`; code/rule and owner-basis combinations remain private/deferred",
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
    // Model document 0.9.0: a term names exactly one of a load case and a combination.
    if combination
        .terms
        .iter()
        .any(|term| term.authored_load_case() == term.combination.is_some())
    {
        diagnostics.push(combination_shape_diag(
            combination,
            "each combination term carries exactly one of load_case and combination",
        ));
        return;
    }
    fn operand(term: &crate::PreviewCombinationTerm) -> (bool, &str) {
        match &term.combination {
            Some(id) => (true, id.as_str()),
            None => (false, term.load_case.as_str()),
        }
    }
    let mut seen = HashSet::new();
    for (index, term) in combination.terms.iter().enumerate() {
        let (_, operand_id) = operand(term);
        // The load-case algebra refuses a repeated operand, which would leave
        // every combined row unevaluated; block it by name here instead, once
        // per repeated case (at its second occurrence), so the diagnostic id
        // stays unique however often the case repeats.
        if !seen.insert(operand(term)) {
            let earlier = combination.terms[..index]
                .iter()
                .filter(|other| operand(other) == operand(term))
                .count();
            if earlier != 1 {
                continue;
            }
            diagnostics.push(diag(
                &format!(
                    "diagnostic:combination:{}:{}:duplicate",
                    stable_suffix(&combination.id),
                    stable_suffix(operand_id)
                ),
                "LOAD_COMBINATION_DUPLICATE_TERM",
                "blocking",
                "mechanics combination terms must name each load case once; a duplicate term is blocked",
                vec![combination.id.clone(), operand_id.to_string()],
            ));
            continue;
        }
        if !term.factor.is_finite() {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:combination:{}:{}:factor",
                    stable_suffix(&combination.id),
                    stable_suffix(operand_id)
                ),
                "LOAD_COMBINATION_FACTOR_INVALID",
                "blocking",
                "load combination factor must be finite and explicitly user supplied",
                vec![combination.id.clone(), operand_id.to_string()],
            ));
        }
        // A term's `combination` is judged with the document's other operands
        // (`document_members`, LOAD_COMBINATION_OPERAND_UNKNOWN).
        if term.combination.is_none() {
            expect_known_combination_load_case(
                combination,
                load_case_ids,
                &term.load_case,
                diagnostics,
            );
        }
    }
}

fn validate_subtraction_combination(combination: &PreviewCombination, diagnostics: &mut Vec<Diagnostic>) {
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
    // Model document 0.9.0 (absolute-sum-srss §2): an operand names a load case, or a
    // combination when no load case has the id; one naming neither is judged with the
    // document's other operands (`document_members`, LOAD_COMBINATION_OPERAND_UNKNOWN).
}

fn validate_range_envelope_combination(combination: &PreviewCombination, diagnostics: &mut Vec<Diagnostic>) {
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
    for (index, operand_id) in operand_ids.iter().enumerate() {
        // Once per repeated operand (at its second occurrence), so the id stays unique.
        if !seen.insert(operand_id.as_str()) {
            if operand_ids[..index].iter().filter(|other| *other == operand_id).count() != 1 {
                continue;
            }
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
    }
    // Unknown operands: `document_members` (LOAD_COMBINATION_OPERAND_UNKNOWN).
}

/// T4-U4b (model 0.6.0; DEL-04-07 Design `u4b-u5b-implementation.md` §2.6):
/// the directional SIF pair `modifiers.sif_in_plane_user_value` (on M_z of the
/// arc's tangent frame) and `modifiers.sif_out_of_plane_user_value` (on M_y).
/// Both or neither; each a finite dimensionless user factor above zero (no
/// code floor); `modifiers.source_reference` required; only on a realized
/// curved bend. A violation blocks by name (`MODEL_INCOMPLETE`, no results):
/// `BEND_DIRECTIONAL_SIF_INCOMPLETE` or `BEND_DIRECTIONAL_SIF_NOT_REALIZED_ARC`.
pub(crate) const BEND_DIRECTIONAL_SIF_INCOMPLETE: &str = "BEND_DIRECTIONAL_SIF_INCOMPLETE";
pub(crate) const BEND_DIRECTIONAL_SIF_NOT_REALIZED_ARC: &str = "BEND_DIRECTIONAL_SIF_NOT_REALIZED_ARC";
fn validate_directional_sifs(model: &crate::PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    for component in &model.components {
        let Some(modifiers) = &component.modifiers else { continue };
        let pair = [&modifiers.sif_in_plane_user_value, &modifiers.sif_out_of_plane_user_value];
        if pair.iter().all(|value| value.is_none()) {
            continue;
        }
        let refs = vec![
            component.id.clone(),
            "modifiers.sif_in_plane_user_value".to_string(),
            "modifiers.sif_out_of_plane_user_value".to_string(),
        ];
        if !crate::is_curved_bend_macro_component(component) {
            diagnostics.push(diag(
                &format!("diagnostic:component:{}:directional-sif-not-realized-arc", stable_suffix(&component.id)),
                BEND_DIRECTIONAL_SIF_NOT_REALIZED_ARC,
                "blocking",
                "the directional SIF pair applies only to a bend realized as a curved_bend_macro_element; at a geometry-only marker or another component no vetted basis fixes the in-plane axis",
                refs,
            ));
            continue;
        }
        let valid = |value: &Option<crate::Quantity>| {
            value.as_ref().is_some_and(|q| {
                matches!(q.unit.as_str(), "1" | "none") && q.value.is_finite() && q.value > 0.0
            })
        };
        let sourced = modifiers.source_reference.as_deref().is_some_and(|text| !text.trim().is_empty());
        if !(pair.iter().all(|value| valid(value)) && sourced) {
            diagnostics.push(diag(
                &format!("diagnostic:component:{}:directional-sif-incomplete", stable_suffix(&component.id)),
                BEND_DIRECTIONAL_SIF_INCOMPLETE,
                "blocking",
                "a realized bend's directional SIFs are both required (in-plane on M_z, out-of-plane on M_y of the tangent frame), each a finite dimensionless user factor above zero, with modifiers.source_reference",
                refs,
            ));
        }
    }
}

/// On-demand model checks (DEL-04-06 Design `on-demand-model-checks.md`): the
/// solve's own checks before stiffness formation, without solving, exposed as
/// `LinearStaticPreviewRequest::pre_formation_checks_value`. Declared here, in
/// a module that no retained-pricing owner source is, so the owner sources
/// stay byte-identical (Codex reviews r1 and r2 of #1665).
mod model_checks;

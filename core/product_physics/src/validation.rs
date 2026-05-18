use crate::LoadTargetInput;
use crate::{diag, parse_dof, stable_suffix, Diagnostic, MaterialInput, PreviewModel, Quantity};
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
            "Pa",
            &format!(
                "diagnostic:unit:material:{}:elastic-modulus",
                stable_suffix(&material.id)
            ),
            vec![material.id.clone(), "elastic_modulus".to_string()],
            diagnostics,
        );
        expect_unit(
            &material.shear_modulus,
            "Pa",
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
                "1/degC",
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
            "m",
            &format!(
                "diagnostic:unit:pipe:{}:outside-diameter",
                stable_suffix(&pipe.id)
            ),
            vec![pipe.id.clone(), "outside_diameter".to_string()],
            diagnostics,
        );
        expect_unit(
            &pipe.section.wall_thickness,
            "m",
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
                Ok(dof) if dof.is_translational() => Some("N/m"),
                Ok(_) => Some("N*m/rad"),
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
            if let Some(unit) = expected {
                expect_unit(
                    &stiffness.value,
                    unit,
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
    for load in model
        .load_cases
        .iter()
        .flat_map(|case| case.primitive_loads.iter())
    {
        if let Some(unit) = expected_load_unit(&load.dimension) {
            expect_unit(
                &load.magnitude,
                unit,
                &format!("diagnostic:unit:load:{}:magnitude", stable_suffix(&load.id)),
                vec![load.id.clone(), "magnitude".to_string()],
                diagnostics,
            );
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
    expected: &str,
    diagnostic_id: &str,
    affected_refs: Vec<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if quantity.unit != expected {
        diagnostics.push(diag(
            diagnostic_id,
            "UNIT_INPUT_INVALID",
            "blocking",
            format!(
                "preview mechanics input requires unit {expected}; got {}",
                quantity.unit
            ),
            affected_refs,
        ));
    }
}

fn expected_load_unit(dimension: &str) -> Option<&'static str> {
    match dimension {
        "force" => Some("N"),
        "moment" => Some("N*m"),
        "force_per_length" => Some("N/m"),
        "pressure" => Some("Pa"),
        "temperature_change" => Some("degC"),
        "acceleration" => Some("m/s^2"),
        "displacement" => Some("m"),
        "rotation" => Some("rad"),
        _ => None,
    }
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
        if combination.basis != "mechanics" {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:combination:{}:basis",
                    stable_suffix(&combination.id)
                ),
                "LOAD_COMBINATION_BASIS_UNSUPPORTED",
                "blocking",
                "TP-MAC-08 supports only mechanics-basis user-defined load combinations; code/rule and owner-basis combinations remain private/deferred",
                vec![combination.id.clone(), combination.basis.clone()],
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
            if !load_case_ids.contains(term.load_case.as_str()) {
                diagnostics.push(diag(
                    &format!(
                        "diagnostic:combination:{}:{}:load-case",
                        stable_suffix(&combination.id),
                        stable_suffix(&term.load_case)
                    ),
                    "LOAD_COMBINATION_LOAD_CASE_UNKNOWN",
                    "blocking",
                    "load combination term references a load case that is not present in the preview model",
                    vec![combination.id.clone(), term.load_case.clone()],
                ));
            }
        }
    }
}

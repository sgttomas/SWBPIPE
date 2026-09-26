//! Closed authoring operations for the model 0.4.0 load/reference-state records
//! (`openpipestress.load_reference_state/1.0.0`), carried through the common
//! Review/Apply seam in the same shape as `pressure_authoring`.
//!
//! Each payload is parsed with the product's own public closed DTOs, re-exported
//! from `open_pipe_stress_product_physics`, directly from the authored text. The
//! operations therefore refuse exactly what the product's typed boundary refuses
//! (unknown fields, unknown discriminants, missing required fields, duplicated
//! keys) and add no second, looser schema. Beyond the typed parse they check only
//! identity and reference resolution against the current model. They set no
//! default, carry no library value and never change `schema_version`.
use crate::rich_authoring::{RichEdit, RichError};
use open_pipe_stress_product_physics::{
    AnalysisStateInput, ExpansionLawInput, ReferenceConfigurationInput, LOAD_STATE_MODEL_VERSION,
};
use serde_json::Value;
use std::collections::HashSet;
type Result<T> = std::result::Result<T, RichError>;

/// The inverse of authoring an absent key: the explicit removal after-value
/// already used by the delete operations. It restores absence exactly.
const NOT_PRESENT: &str = "not_present";

fn refuse(code: &'static str, message: impl Into<String>) -> RichError {
    RichError {
        code,
        message: message.into(),
    }
}

pub(crate) fn owns(object_type: &str, path: &str) -> bool {
    matches!(
        (object_type, path),
        ("Model", "reference_configurations")
            | ("Material", "expansion_laws")
            | ("Load", "analysis_state")
    )
}

/// The one model entity with this ID; a missing or duplicated ID is refused.
fn entity<'a>(model: &'a Value, collection: &str, id: &str) -> Result<&'a Value> {
    let mut found = model
        .get(collection)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|v| v.get("id").and_then(Value::as_str) == Some(id));
    let value = found.next().ok_or_else(|| {
        refuse(
            "OP-LOAD-STATE-TARGET-INVALID",
            format!("Unknown {collection} target {id}"),
        )
    })?;
    if found.next().is_some() {
        return Err(refuse(
            "OP-LOAD-STATE-TARGET-INVALID",
            format!("Ambiguous {collection} target {id}"),
        ));
    }
    Ok(value)
}

fn ids<'a>(owner: &'a Value, collection: &str) -> HashSet<&'a str> {
    owner.get(collection).map(array_ids).unwrap_or_default()
}

fn array_ids(array: &Value) -> HashSet<&str> {
    array
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|v| v.get("id").and_then(Value::as_str))
        .collect()
}

fn resolve_ref(known: &HashSet<&str>, kind: &str, id: &str) -> Result<()> {
    if known.contains(id) {
        Ok(())
    } else {
        Err(refuse(
            "OP-LOAD-STATE-REFERENCE-UNRESOLVED",
            format!("{kind} {id} does not resolve in the current model"),
        ))
    }
}

fn unique<'a>(values: impl IntoIterator<Item = &'a str>, kind: &str) -> Result<()> {
    let mut seen = HashSet::new();
    for id in values {
        if !seen.insert(id) {
            return Err(refuse(
                "OP-LOAD-STATE-DUPLICATE-ID",
                format!("Duplicate {kind} ID {id}"),
            ));
        }
    }
    Ok(())
}

fn typed<T: serde::de::DeserializeOwned>(after: &str, path: &str) -> Result<T> {
    serde_json::from_str(after).map_err(|e| {
        refuse(
            "OP-LOAD-STATE-PAYLOAD-INVALID",
            format!("{path} refused at the product's closed typed boundary: {e}"),
        )
    })
}

fn law_id(law: &ExpansionLawInput) -> &str {
    match law {
        ExpansionLawInput::EngineeringSecant { id, .. }
        | ExpansionLawInput::EngineeringDilation { id, .. }
        | ExpansionLawInput::DifferentialPerDatumLength { id, .. }
        | ExpansionLawInput::LogarithmicPerCurrentLength { id, .. } => id,
    }
}

fn validate_reference_configurations(model: &Value, after: &str) -> Result<()> {
    let configurations: Vec<ReferenceConfigurationInput> =
        typed(after, "reference_configurations")?;
    unique(
        configurations.iter().map(|c| c.id.as_str()),
        "reference configuration",
    )?;
    let pipes = ids(model, "pipe_segments");
    for configuration in &configurations {
        for member in &configuration.member_references {
            resolve_ref(&pipes, "pipe_ref", &member.pipe_ref)?;
        }
    }
    Ok(())
}

fn validate_expansion_laws(after: &str) -> Result<()> {
    let laws: Vec<ExpansionLawInput> = typed(after, "expansion_laws")?;
    unique(laws.iter().map(law_id), "expansion law")
}

fn validate_analysis_state(
    model: &Value,
    case: &Value,
    after: &str,
    after_value: &Value,
) -> Result<()> {
    let state: AnalysisStateInput = typed(after, "analysis_state")?;
    resolve_ref(
        &ids(model, "reference_configurations"),
        "reference_configuration_ref",
        &state.reference_configuration_ref,
    )?;
    let pipes = ids(model, "pipe_segments");
    // The selection and thermal unions are public only through their fields, so
    // their inner references are read from the same authored value, which the
    // typed parse above has just shown to be exactly this closed shape.
    let elements = after_value["element_states"]
        .as_array()
        .into_iter()
        .flatten();
    for (element, raw) in state.element_states.iter().zip(elements) {
        resolve_ref(&pipes, "pipe_ref", &element.pipe_ref)?;
        let selection = &raw["material_selection"];
        let material_ref = selection["material_ref"].as_str().unwrap_or_default();
        let material = model
            .get("materials")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|m| m.get("id").and_then(Value::as_str) == Some(material_ref))
            .ok_or_else(|| {
                refuse(
                    "OP-LOAD-STATE-REFERENCE-UNRESOLVED",
                    format!("material_ref {material_ref} does not resolve in the current model"),
                )
            })?;
        if let Some(point_ref) = selection.get("point_ref").and_then(Value::as_str) {
            resolve_ref(&ids(material, "temperature_points"), "point_ref", point_ref)?;
        }
        if let Some(law_ref) = raw["thermal_state"]
            .get("expansion_law_ref")
            .and_then(Value::as_str)
        {
            resolve_ref(
                &ids(material, "expansion_laws"),
                "expansion_law_ref",
                law_ref,
            )?;
        }
    }
    let supports = ids(model, "supports");
    for support in &state.support_states {
        resolve_ref(&supports, "support_ref", &support.support_ref)?;
    }
    let primitives = ids(case, "primitive_loads");
    for source in &state.load_sources {
        resolve_ref(&primitives, "load_sources[].source_ref", &source.source_ref)?;
    }
    Ok(())
}

fn case_id(case: &Value) -> &str {
    case.get("id")
        .and_then(Value::as_str)
        .unwrap_or("load_case:unknown")
}

fn cases(model: &Value) -> impl Iterator<Item = &Value> {
    model
        .get("load_cases")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
}

fn element_states(case: &Value) -> impl Iterator<Item = (usize, &Value)> {
    case.pointer("/analysis_state/element_states")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
}

/// A replacement or removal of an owner collection must not orphan a reference
/// that resolves now: `reference_configuration_ref` for the model's
/// configurations, and `expansion_law_ref` (on elements that select this
/// material) for a material's laws. A reference that was already unresolved is
/// not created by this edit and is left to the solve-time diagnostics.
fn refuse_orphans(
    model: &Value,
    object_type: &str,
    target: &str,
    owner: &Value,
    after: Option<&Value>,
) -> Result<()> {
    let path = match object_type {
        "Model" => "reference_configurations",
        "Material" => "expansion_laws",
        _ => return Ok(()),
    };
    let current = ids(owner, path);
    let next = after.map(array_ids);
    let lost = |id: &str| current.contains(id) && !next.as_ref().is_some_and(|n| n.contains(id));
    let mut orphans = Vec::new();
    for case in cases(model) {
        if object_type == "Model" {
            if let Some(r) = case
                .pointer("/analysis_state/reference_configuration_ref")
                .and_then(Value::as_str)
                .filter(|r| lost(r))
            {
                orphans.push(format!(
                    "{}.analysis_state.reference_configuration_ref ({r})",
                    case_id(case)
                ));
            }
            continue;
        }
        for (index, element) in element_states(case) {
            if element
                .pointer("/material_selection/material_ref")
                .and_then(Value::as_str)
                != Some(target)
            {
                continue;
            }
            if let Some(law) = element
                .pointer("/thermal_state/expansion_law_ref")
                .and_then(Value::as_str)
                .filter(|l| lost(l))
            {
                orphans.push(format!(
                    "{}.analysis_state.element_states.{index}.thermal_state.expansion_law_ref ({law})",
                    case_id(case)
                ));
            }
        }
    }
    if orphans.is_empty() {
        Ok(())
    } else {
        Err(refuse(
            "OP-LOAD-STATE-INBOUND-REFERENCE",
            format!(
                "This {path} edit would orphan references held at {}; remove or retarget them first",
                orphans.join(", ")
            ),
        ))
    }
}

/// Load/reference-state paths that name a pipe: element states and
/// reference-configuration members. Used by pipe deletion.
pub(crate) fn pipe_references(model: &Value, pipe_ref: &str) -> Vec<String> {
    let mut found = Vec::new();
    for configuration in model
        .get("reference_configurations")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let id = configuration
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("reference:unknown");
        for (index, member) in configuration
            .get("member_references")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
        {
            if member.get("pipe_ref").and_then(Value::as_str) == Some(pipe_ref) {
                found.push(format!(
                    "reference_configurations.{id}.member_references.{index}.pipe_ref"
                ));
            }
        }
    }
    for case in cases(model) {
        for (index, element) in element_states(case) {
            if element.get("pipe_ref").and_then(Value::as_str) == Some(pipe_ref) {
                found.push(format!(
                    "{}.analysis_state.element_states.{index}.pipe_ref",
                    case_id(case)
                ));
            }
        }
    }
    found
}

/// Load/reference-state paths that name a support. Used by support deletion.
pub(crate) fn support_references(model: &Value, support_ref: &str) -> Vec<String> {
    let mut found = Vec::new();
    for case in cases(model) {
        for (index, state) in case
            .pointer("/analysis_state/support_states")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
        {
            if state.get("support_ref").and_then(Value::as_str) == Some(support_ref) {
                found.push(format!(
                    "{}.analysis_state.support_states.{index}.support_ref",
                    case_id(case)
                ));
            }
        }
    }
    found
}

/// The owning case's `load_sources` entries that name a stored primitive.
/// Used by primitive-load deletion.
pub(crate) fn source_references(case: &Value, primitive_id: &str) -> Vec<String> {
    case.pointer("/analysis_state/load_sources")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
        .filter(|(_, source)| {
            source.get("source_ref").and_then(Value::as_str) == Some(primitive_id)
        })
        .map(|(index, _)| {
            format!(
                "{}.analysis_state.load_sources.{index}.source_ref",
                case_id(case)
            )
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn resolve(
    model: &Value,
    object_type: &str,
    target: &str,
    path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
) -> Result<Option<RichEdit>> {
    if !owns(object_type, path) {
        return Ok(None);
    }
    if unit != "none" || dimension != "dimensionless" {
        return Err(refuse(
            "OP-LOAD-STATE-ENVELOPE-INVALID",
            "Structured load/reference-state input requires unit none and dimension dimensionless",
        ));
    }
    if model.get("schema_version").and_then(Value::as_str) != Some(LOAD_STATE_MODEL_VERSION) {
        return Err(refuse(
            "OP-LOAD-STATE-SCHEMA-VERSION-INVALID",
            format!("{path} can be authored only on a model whose schema_version is already {LOAD_STATE_MODEL_VERSION}; no operation changes the version"),
        ));
    }
    let owner = match object_type {
        "Model" => {
            if model.pointer("/project/id").and_then(Value::as_str) != Some(target) {
                return Err(refuse(
                    "OP-LOAD-STATE-TARGET-INVALID",
                    "reference_configurations target must be the current project ID",
                ));
            }
            model
        }
        "Material" => entity(model, "materials", target)?,
        _ => entity(model, "load_cases", target)?,
    };
    let current_display = owner
        .get(path)
        .map(crate::canonical_json)
        .unwrap_or_else(|| NOT_PRESENT.to_string());
    if before != current_display {
        return Err(RichError {
            code: "OP-BEFORE-VALUE-MISMATCH",
            message: "Explicit input before-value is stale; refresh the draft".into(),
        });
    }
    let (write, after_display) = if after == NOT_PRESENT {
        (None, NOT_PRESENT.to_string())
    } else {
        let after_value: Value = serde_json::from_str(after).map_err(|e| {
            refuse(
                "OP-LOAD-STATE-PAYLOAD-INVALID",
                format!("{path} is not JSON: {e}"),
            )
        })?;
        if after_value.is_null() {
            return Err(refuse(
                "OP-LOAD-STATE-EXPLICIT-NULL",
                format!("An explicit null {path} is authored presence, not absence; supply the value or remove the key with {NOT_PRESENT}"),
            ));
        }
        match object_type {
            "Model" => validate_reference_configurations(model, after)?,
            "Material" => validate_expansion_laws(after)?,
            _ => validate_analysis_state(model, owner, after, &after_value)?,
        }
        let display = crate::canonical_json(&after_value);
        (Some(after_value), display)
    };
    refuse_orphans(model, object_type, target, owner, write.as_ref())?;
    Ok(Some(RichEdit {
        writes: if after_display == current_display {
            vec![]
        } else {
            vec![(vec![path.to_string()], write)]
        },
        current_display,
        warnings: vec![],
    }))
}

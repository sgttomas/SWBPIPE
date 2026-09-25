//! Explicit pressure-profile inputs, carried through the common operation seam.
use crate::rich_authoring::{RichEdit, RichError};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{Map, Value};
use std::collections::HashSet;
type Result<T> = std::result::Result<T, RichError>;
const MATERIAL_KEYS: &[&str] = &[
    "constitutive_basis",
    "elastic_modulus",
    "shear_modulus",
    "poisson_ratio",
    "provenance",
];
const PROFILE_KEYS: &[&str] = &["schema_version", "pressure_contract"];
fn err(message: impl Into<String>) -> RichError {
    RichError {
        code: "OP-PRESSURE-PAYLOAD-INVALID",
        message: message.into(),
    }
}
fn object<'a>(v: &'a Value, keys: &[&str]) -> Result<&'a Map<String, Value>> {
    let o = v.as_object().ok_or_else(|| err("Expected an object"))?;
    if let Some(k) = o.keys().find(|k| !keys.contains(&k.as_str())) {
        return Err(err(format!("Unknown field {k}")));
    }
    Ok(o)
}
fn text<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    v.get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| err(format!("{key} is required")))
}
fn quantity(v: &Value, dim: Dimension) -> Result<f64> {
    object(v, &["value", "unit"])?;
    let value = v["value"]
        .as_f64()
        .filter(|v| v.is_finite())
        .ok_or_else(|| err("Quantity must be finite"))?;
    let from = unit_by_symbol(text(v, "unit")?, dim).map_err(|e| err(e.to_string()))?;
    let normalized = convert_for_dimension(
        value,
        dim,
        from,
        canonical_unit(dim).ok_or_else(|| err("Canonical unit unavailable"))?,
    )
    .map_err(|e| err(e.to_string()))?;
    if !normalized.is_finite() {
        return Err(err("Normalized quantity must be finite"));
    }
    Ok(normalized)
}
pub(crate) fn validate_nu(v: &Value) -> Result<()> {
    object(v, &["value", "unit"])?;
    if v["unit"] != "1"
        || !v["value"]
            .as_f64()
            .is_some_and(|n| n.is_finite() && n > -1.0 && n < 0.5)
    {
        return Err(err(
            "Poisson ratio requires explicit unit 1 and -1 < nu < 0.5",
        ));
    }
    Ok(())
}
pub(crate) fn validate_material(v: &Value) -> Result<()> {
    let e = quantity(&v["elastic_modulus"], Dimension::Stress)?;
    if e <= 0.0 {
        return Err(err("Elastic modulus must be positive"));
    }
    text(v, "provenance")?;
    if v["constitutive_basis"] == "homogeneous_isotropic_E_nu_v1" {
        validate_nu(&v["poisson_ratio"])?;
        let g = e / (2.0 * (1.0 + v["poisson_ratio"]["value"].as_f64().unwrap()));
        if !g.is_finite() || g <= 0.0 {
            return Err(err("E/nu must produce finite positive G"));
        }
        if let Some(g) = v.get("shear_modulus") {
            if quantity(g, Dimension::Stress)? <= 0.0 {
                return Err(err("Retained shear modulus must be positive"));
            }
        }
    } else {
        if v.get("constitutive_basis").is_some() || v.get("poisson_ratio").is_some() {
            return Err(err("E/nu authoring requires homogeneous_isotropic_E_nu_v1"));
        }
        if quantity(&v["shear_modulus"], Dimension::Stress)? <= 0.0 {
            return Err(err("Legacy material requires positive shear modulus"));
        }
    }
    Ok(())
}
pub(crate) fn validate_create_material(v: &Value) -> Result<()> {
    object(
        v,
        &[
            "id",
            "label",
            "constitutive_basis",
            "elastic_modulus",
            "shear_modulus",
            "poisson_ratio",
            "thermal_expansion_coefficient",
            "provenance",
        ],
    )?;
    validate_material(v)
}
fn entity<'a>(model: &'a Value, collection: &str, id: &str) -> Result<&'a Value> {
    let mut found = model[collection]
        .as_array()
        .ok_or_else(|| err(format!("Missing {collection}")))?
        .iter()
        .filter(|v| v["id"] == id);
    let value = found
        .next()
        .ok_or_else(|| err(format!("Unknown {collection} reference {id}")))?;
    if found.next().is_some() {
        return Err(err(format!("Ambiguous {collection} reference {id}")));
    }
    Ok(value)
}
fn projection(v: &Value, keys: &[&str]) -> Value {
    Value::Object(
        keys.iter()
            .filter_map(|k| v.get(*k).map(|v| ((*k).to_string(), v.clone())))
            .collect(),
    )
}
pub(crate) fn owns(object_type: &str, path: &str) -> bool {
    matches!(
        (object_type, path),
        ("Model", "pressure_profile")
            | ("Material", "constitutive_properties")
            | ("Load", "pressure_regions")
    )
}
fn validate_regions(model: &Value, value: &Value) -> Result<()> {
    let regions = value
        .as_array()
        .ok_or_else(|| err("pressure_regions must be an explicit array"))?;
    let mut ids = HashSet::new();
    let mut owned_pipes = HashSet::new();
    for region in regions {
        object(
            region,
            &[
                "id",
                "member_pipe_ids",
                "pressure_basis",
                "pressure",
                "terminals",
                "provenance",
            ],
        )?;
        if !ids.insert(text(region, "id")?) {
            return Err(err("Duplicate region ID"));
        }
        text(region, "provenance")?;
        if region["pressure_basis"] != "internal_differential_zero_external_v1" {
            return Err(err(
                "Explicit internal differential zero-external pressure basis is required",
            ));
        }
        if quantity(&region["pressure"], Dimension::Stress)? < 0.0 {
            return Err(err("Pressure must be nonnegative"));
        }
        let pipes = region["member_pipe_ids"]
            .as_array()
            .filter(|p| !p.is_empty())
            .ok_or_else(|| err("Region requires member pipe IDs"))?;
        let mut nodes = HashSet::new();
        for id in pipes {
            let id = id
                .as_str()
                .ok_or_else(|| err("Member pipe ID must be a string"))?;
            if !owned_pipes.insert(id) {
                return Err(err(
                    "A pipe can belong to only one pressure region in a case",
                ));
            }
            let pipe = entity(model, "pipe_segments", id)?;
            nodes.insert(text(pipe, "from")?);
            nodes.insert(text(pipe, "to")?);
        }
        let terminals = region["terminals"]
            .as_array()
            .filter(|t| t.len() == 2)
            .ok_or_else(|| err("Region requires exactly two ordered terminals"))?;
        let mut terminal_ids = HashSet::new();
        for terminal in terminals {
            object(terminal, &["node_ref", "closure_transfer", "provenance"])?;
            let id = text(terminal, "node_ref")?;
            entity(model, "nodes", id)?;
            if !nodes.contains(id) || !terminal_ids.insert(id) {
                return Err(err(
                    "Terminals must be distinct nodes on the authored member pipes",
                ));
            }
            if !matches!(
                text(terminal, "closure_transfer")?,
                "transfers_to_wall" | "separately_supported_or_compensated"
            ) {
                return Err(err("Each terminal requires explicit closure transfer"));
            }
            text(terminal, "provenance")?;
        }
    }
    Ok(())
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
        return Err(err(
            "Structured input envelope requires unit none and dimension dimensionless",
        ));
    }
    let current = match object_type {
        "Model" => {
            if model.pointer("/project/id").and_then(Value::as_str) != Some(target) {
                return Err(err("Profile target must be the current project ID"));
            }
            projection(model, PROFILE_KEYS)
        }
        "Material" => projection(entity(model, "materials", target)?, MATERIAL_KEYS),
        _ => entity(model, "load_cases", target)?
            .get("pressure_regions")
            .cloned()
            .unwrap_or(Value::Null),
    };
    let current_display = if current.is_null() {
        "not_present".to_string()
    } else {
        crate::canonical_json(&current)
    };
    if before != current_display {
        return Err(RichError {
            code: "OP-BEFORE-VALUE-MISMATCH",
            message: "Explicit input before-value is stale; refresh the draft".into(),
        });
    }
    let after: Value = serde_json::from_str(after).map_err(|e| err(e.to_string()))?;
    let writes = match object_type {
        "Model" => {
            object(&after, PROFILE_KEYS)?;
            if after["schema_version"] != "0.3.0"
                || after["pressure_contract"]
                    != serde_json::json!({"version":"2.0.0","mode":"exact_straight_pressure_v2"})
            {
                return Err(err(
                    "Select explicit model 0.3.0 / exact_straight_pressure_v2 version 2.0.0",
                ));
            }
            for case in model["load_cases"]
                .as_array()
                .ok_or_else(|| err("Missing load cases"))?
            {
                validate_regions(model, &case["pressure_regions"])?;
                if case["primitive_loads"]
                    .as_array()
                    .is_some_and(|loads| loads.iter().any(|l| l["category"] == "pressure"))
                {
                    return Err(err("Exact profile admits no pressure primitives, including zero. Review and remove those loads explicitly before changing profile"));
                }
            }
            PROFILE_KEYS
                .iter()
                .map(|k| (vec![(*k).into()], after.get(*k).cloned()))
                .collect()
        }
        "Material" => {
            object(&after, MATERIAL_KEYS)?;
            validate_material(&after)?;
            MATERIAL_KEYS
                .iter()
                .map(|k| (vec![(*k).into()], after.get(*k).cloned()))
                .collect()
        }
        _ => {
            validate_regions(model, &after)?;
            vec![(vec!["pressure_regions".into()], Some(after.clone()))]
        }
    };
    Ok(Some(RichEdit {
        writes: if crate::canonical_json(&after) == current_display {
            vec![]
        } else {
            writes
        },
        current_display,
        warnings: vec![],
    }))
}

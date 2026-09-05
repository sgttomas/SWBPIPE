//! Validated, atomic rich-authoring replacements. No physics is computed here.
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone)]
pub(crate) struct RichError {
    pub code: &'static str,
    pub message: String,
}
#[derive(Debug, Clone)]
pub(crate) struct RichEdit {
    /// Entity-relative paths. None removes an optional configuration member.
    pub writes: Vec<(Vec<String>, Option<Value>)>,
    pub current_display: String,
    pub warnings: Vec<String>,
}
type Result<T> = std::result::Result<T, RichError>;
fn err(message: impl Into<String>) -> RichError {
    RichError {
        code: "OP-RICH-PAYLOAD-INVALID",
        message: message.into(),
    }
}
fn object<'a>(v: &'a Value, keys: &[&str]) -> Result<&'a Map<String, Value>> {
    let o = v.as_object().ok_or_else(|| err("Expected a JSON object"))?;
    if let Some(k) = o.keys().find(|k| !keys.contains(&k.as_str())) {
        return Err(err(format!("Unknown field {k}")));
    }
    Ok(o)
}
fn text<'a>(v: Option<&'a Value>, name: &str) -> Result<&'a str> {
    v.and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| err(format!("{name} must be a nonempty string")))
}
fn optional_text(o: &Map<String, Value>, keys: &[&str]) -> Result<()> {
    for k in keys {
        if let Some(v) = o.get(*k) {
            text(Some(v), k)?;
        }
    }
    Ok(())
}
fn enum_text<'a>(v: Option<&'a Value>, name: &str, choices: &[&str]) -> Result<&'a str> {
    let s = text(v, name)?;
    if !choices.contains(&s) {
        return Err(err(format!("Unsupported {name}: {s}")));
    }
    Ok(s)
}
fn quantity(v: &Value, dim: Dimension, positive: bool) -> Result<f64> {
    let o = object(v, &["value", "unit"])?;
    let n = o
        .get("value")
        .and_then(Value::as_f64)
        .filter(|n| n.is_finite())
        .ok_or_else(|| err("Quantity requires a finite value"))?;
    let unit = text(o.get("unit"), "quantity.unit")?;
    // Dimensionless model quantities also use the established `none` alias.
    let normalized = if dim == Dimension::Dimensionless && unit == "none" {
        n
    } else {
        let from = unit_by_symbol(unit, dim).map_err(|e| err(e.to_string()))?;
        let to = canonical_unit(dim).ok_or_else(|| err("No canonical unit"))?;
        convert_for_dimension(n, dim, from, to).map_err(|e| err(e.to_string()))?
    };
    if !normalized.is_finite() || (positive && normalized <= 0.0) {
        return Err(err("Quantity must be finite and positive"));
    }
    Ok(normalized)
}
fn nonnegative(v: &Value, dim: Dimension) -> Result<f64> {
    let n = quantity(v, dim, false)?;
    if n < 0.0 {
        return Err(err("Quantity must be nonnegative"));
    }
    Ok(n)
}
fn entity<'a>(model: &'a Value, collection: &str, id: &str) -> Result<&'a Value> {
    let list = model
        .get(collection)
        .and_then(Value::as_array)
        .ok_or_else(|| err(format!("Missing {collection} collection")))?;
    let mut matches = list
        .iter()
        .filter(|v| v.get("id").and_then(Value::as_str) == Some(id));
    let value = matches
        .next()
        .ok_or_else(|| err(format!("Unknown {collection} reference {id}")))?;
    if matches.next().is_some() {
        return Err(err(format!("Ambiguous {collection} reference {id}")));
    }
    Ok(value)
}
const CONFIG: &[&str] = &[
    "family",
    "restraints",
    "stiffness",
    "hanger",
    "nonlinear",
    "provenance",
];
fn projection(value: &Value, keys: &[&str]) -> Value {
    Value::Object(
        keys.iter()
            .filter_map(|k| value.get(*k).map(|v| ((*k).to_string(), v.clone())))
            .collect(),
    )
}
fn exposure(wind: &Value) -> Value {
    json!({"exposed_pipe_refs":wind.get("exposed_pipe_refs").cloned().unwrap_or(json!([])),"exposed_spans":wind.get("exposed_spans").cloned().unwrap_or(json!([]))})
}
/// None means this module does not own the requested field. Caller must gate the
/// operation kind: update_support, set_field, update_load respectively.
#[allow(clippy::too_many_arguments)]
pub(crate) fn resolve(
    model: &Value,
    object_type: &str,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
) -> Result<Option<RichEdit>> {
    let collection = match (object_type, field_path) {
        ("Support", "configuration") => "supports",
        ("Material", "temperature_points") => "materials",
        ("Load", "equivalent_static.wind.exposure") => "load_cases",
        _ => return Ok(None),
    };
    if unit != "none" || dimension != "dimensionless" {
        return Err(err("Rich replacement envelope requires unit none and dimension dimensionless; quantities carry individual units"));
    }
    let current = entity(model, collection, target_ref)?;
    let current_display = match collection {
        "supports" => crate::canonical_json(&projection(current, CONFIG)),
        "materials" => current
            .get("temperature_points")
            .map(crate::canonical_json)
            .unwrap_or_else(|| "not_present".into()),
        _ => crate::canonical_json(&exposure(
            current
                .pointer("/equivalent_static/wind")
                .ok_or_else(|| err("Wind configuration must exist before exposure authoring"))?,
        )),
    };
    if before != current_display {
        return Err(RichError {
            code: "OP-BEFORE-VALUE-MISMATCH",
            message: "Rich replacement before value is stale; refresh the configuration".into(),
        });
    }
    let mut after: Value =
        serde_json::from_str(after).map_err(|e| err(format!("Invalid replacement JSON: {e}")))?;
    // A semantic no-op keeps original numeric representations and entered units.
    let no_op = crate::canonical_json(&after) == current_display;
    if no_op {
        after = match collection {
            "supports" => projection(current, CONFIG),
            "materials" => current["temperature_points"].clone(),
            _ => exposure(current.pointer("/equivalent_static/wind").unwrap()),
        };
    }
    let mut warnings = Vec::new();
    let writes = match collection {
        "supports" => {
            object(&after, CONFIG)?;
            let mut prospective = current.clone();
            let o = prospective
                .as_object_mut()
                .ok_or_else(|| err("Support must be an object"))?;
            for key in CONFIG {
                if let Some(v) = after.get(*key) {
                    o.insert((*key).into(), v.clone());
                } else {
                    o.remove(*key);
                }
            }
            validate_support_configuration(model, &prospective)?;
            let prospective_id = prospective["id"].clone();
            let mut candidate = model.clone();
            let supports = candidate["supports"]
                .as_array_mut()
                .ok_or_else(|| err("Missing supports"))?;
            *supports
                .iter_mut()
                .find(|s| s["id"] == prospective_id)
                .ok_or_else(|| err("Support missing"))? = prospective;
            // Recheck derived reaction consumers against the prospective source.
            for support in candidate["supports"].as_array().unwrap() {
                if support
                    .pointer("/nonlinear/normal_reaction_source/support_ref")
                    .and_then(Value::as_str)
                    == Some(target_ref)
                {
                    validate_nonlinear(&candidate, &support["nonlinear"])?;
                }
            }
            CONFIG
                .iter()
                .map(|key| (vec![(*key).into()], after.get(*key).cloned()))
                .collect()
        }
        "materials" => {
            validate_temperature_points(model, current, &after, &mut warnings)?;
            vec![(vec!["temperature_points".into()], Some(after))]
        }
        _ => {
            validate_exposure(model, &after, &mut warnings)?;
            vec![
                (
                    vec![
                        "equivalent_static".into(),
                        "wind".into(),
                        "exposed_pipe_refs".into(),
                    ],
                    Some(after["exposed_pipe_refs"].clone()),
                ),
                (
                    vec![
                        "equivalent_static".into(),
                        "wind".into(),
                        "exposed_spans".into(),
                    ],
                    Some(after["exposed_spans"].clone()),
                ),
            ]
        }
    };
    Ok(Some(RichEdit {
        writes: if no_op { Vec::new() } else { writes },
        current_display,
        warnings,
    }))
}
fn dof(v: Option<&Value>) -> Result<&str> {
    enum_text(
        v,
        "DOF",
        &[
            "UX", "Ux", "ux", "UY", "Uy", "uy", "UZ", "Uz", "uz", "RX", "Rx", "rx", "RY", "Ry",
            "ry", "RZ", "Rz", "rz",
        ],
    )
}
fn stiffness_identity(v: &Value) -> Result<(String, f64)> {
    let o = object(v, &["dof", "value"])?;
    let d = dof(o.get("dof"))?.to_ascii_uppercase();
    let value = quantity(
        o.get("value")
            .ok_or_else(|| err("Missing stiffness value"))?,
        if d.starts_with('U') {
            Dimension::LinearStiffness
        } else {
            Dimension::RotationalStiffness
        },
        true,
    )?;
    Ok((d, value))
}
fn stiffness(v: &Value) -> Result<()> {
    stiffness_identity(v).map(|_| ())
}
const SUPPORT_FAMILIES: &[&str] = &[
    "anchor",
    "guide",
    "line_stop",
    "vertical_support",
    "spring",
    "variable_spring_hanger",
    "spring_hanger",
    "constant_effort_support",
    "nonlinear",
];
/// Validates top-level rich fields of a full support without mutating it. Legacy
/// properties remain owned by the existing support authoring implementation.
pub(crate) fn validate_support_configuration(model: &Value, support: &Value) -> Result<()> {
    let o = support
        .as_object()
        .ok_or_else(|| err("Support must be an object"))?;
    entity(model, "nodes", text(o.get("node"), "support.node")?)?;
    let restraints = o
        .get("restraints")
        .and_then(Value::as_array)
        .ok_or_else(|| err("Support requires restraints array"))?;
    let mut seen = HashSet::new();
    for v in restraints {
        let d = dof(Some(v))?.to_ascii_uppercase();
        if !seen.insert(d) {
            return Err(err("Duplicate restraint DOF"));
        }
    }
    optional_text(o, &["provenance"])?;
    if let Some(family) = o.get("family").filter(|value| !value.is_null()) {
        enum_text(Some(family), "family", SUPPORT_FAMILIES)?;
    }
    let top_stiffness = o.get("stiffness").map(stiffness_identity).transpose()?;
    let hanger_type = o
        .get("hanger")
        .and_then(|h| h.get("hanger_type"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .or_else(|| o.get("family").and_then(Value::as_str).map(str::trim));
    if let Some(h) = o.get("hanger") {
        let h = object(
            h,
            &[
                "hanger_type",
                "stiffness",
                "installed_load",
                "cold_load",
                "hot_load",
                "constant_load",
                "travel_range",
                "movement_limit",
                "manufacturer_reference",
                "source_reference",
                "load_side_review_reference",
                "mechanics_consumption",
            ],
        )?;
        optional_text(
            h,
            &[
                "hanger_type",
                "manufacturer_reference",
                "source_reference",
                "load_side_review_reference",
                "mechanics_consumption",
            ],
        )?;
        if let Some(t) = h.get("hanger_type") {
            enum_text(
                Some(t),
                "hanger_type",
                &[
                    "variable_spring_hanger",
                    "spring_hanger",
                    "constant_effort_support",
                ],
            )?;
        }
        let hanger_stiffness = h.get("stiffness").map(stiffness_identity).transpose()?;
        if let (Some(top), Some(hanger)) = (&top_stiffness, &hanger_stiffness) {
            if top != hanger {
                return Err(err("Support stiffness and hanger stiffness must have the same DOF and exactly equal normalized value; correct the conflicting definitions."));
            }
        }
        for k in ["installed_load", "cold_load", "hot_load", "constant_load"] {
            if let Some(v) = h.get(k) {
                quantity(v, Dimension::Force, false)?;
            }
        }
        for k in ["travel_range", "movement_limit"] {
            if let Some(v) = h.get(k) {
                quantity(v, Dimension::Length, false)?;
            }
        }
        if matches!(
            hanger_type,
            Some("variable_spring_hanger" | "spring_hanger" | "constant_effort_support")
        ) {
            text(h.get("hanger_type"), "hanger.hanger_type")?;
            text(h.get("source_reference"), "hanger.source_reference")?;
            let travel = h
                .get("travel_range")
                .map(|q| quantity(q, Dimension::Length, false))
                .transpose()?;
            let movement = h
                .get("movement_limit")
                .map(|q| quantity(q, Dimension::Length, false))
                .transpose()?;
            if !travel.is_some_and(|q| q > 0.0) && !movement.is_some_and(|q| q > 0.0) {
                return Err(err(
                    "Hanger requires positive travel_range or movement_limit",
                ));
            }
            if hanger_type == Some("constant_effort_support") {
                quantity(
                    h.get("constant_load")
                        .ok_or_else(|| err("Missing constant_load"))?,
                    Dimension::Force,
                    true,
                )?;
            } else {
                stiffness(
                    o.get("stiffness")
                        .or_else(|| h.get("stiffness"))
                        .ok_or_else(|| err("Variable hanger requires stiffness"))?,
                )?;
                for k in ["installed_load", "cold_load", "hot_load"] {
                    quantity(
                        h.get(k).ok_or_else(|| err(format!("Missing hanger {k}")))?,
                        Dimension::Force,
                        true,
                    )?;
                }
            }
        }
    } else if matches!(
        hanger_type,
        Some("variable_spring_hanger" | "spring_hanger" | "constant_effort_support")
    ) {
        return Err(err("Named hanger family requires hanger object"));
    }
    if o.get("family").and_then(Value::as_str) == Some("spring") && o.get("nonlinear").is_none() {
        stiffness(
            o.get("stiffness")
                .or_else(|| o.get("hanger").and_then(|h| h.get("stiffness")))
                .ok_or_else(|| err("Spring family requires explicit stiffness and DOF"))?,
        )?;
    }
    if let Some(v) = o.get("nonlinear") {
        validate_nonlinear(model, v)?;
    }
    Ok(())
}
fn validate_nonlinear(model: &Value, v: &Value) -> Result<()> {
    let o = object(
        v,
        &[
            "behavior",
            "dof",
            "initial_state",
            "active_when",
            "contact_when",
            "closes_when",
            "gap",
            "friction_coefficient",
            "normal_reaction",
            "normal_reaction_source",
        ],
    )?;
    let behavior = enum_text(
        o.get("behavior"),
        "nonlinear.behavior",
        &[
            "one_way", "one-way", "oneway", "gap", "lift_off", "lift-off", "liftoff", "friction",
        ],
    )?;
    dof(o.get("dof"))?;
    enum_text(
        o.get("initial_state"),
        "initial_state",
        &[
            "active", "ACTIVE", "inactive", "INACTIVE", "sticking", "STICKING", "sliding",
            "SLIDING",
        ],
    )?;
    for k in ["active_when", "contact_when"] {
        if o.contains_key(k)
            || (k == "active_when" && matches!(behavior, "one_way" | "one-way" | "oneway"))
            || (k == "contact_when" && matches!(behavior, "lift_off" | "lift-off" | "liftoff"))
        {
            enum_text(
                o.get(k),
                k,
                &[
                    "positive_reaction",
                    "positive",
                    "POSITIVE_REACTION",
                    "POSITIVE",
                    "negative_reaction",
                    "negative",
                    "NEGATIVE_REACTION",
                    "NEGATIVE",
                ],
            )?;
        }
    }
    if o.contains_key("closes_when") || behavior == "gap" {
        enum_text(
            o.get("closes_when"),
            "closes_when",
            &[
                "positive_displacement",
                "positive",
                "POSITIVE_DISPLACEMENT",
                "POSITIVE",
                "negative_displacement",
                "negative",
                "NEGATIVE_DISPLACEMENT",
                "NEGATIVE",
            ],
        )?;
    }
    for (key, dim, required) in [
        ("gap", Dimension::Length, behavior == "gap"),
        (
            "friction_coefficient",
            Dimension::Dimensionless,
            behavior == "friction",
        ),
        ("normal_reaction", Dimension::Force, false),
    ] {
        if let Some(v) = o.get(key) {
            nonnegative(v, dim)?;
        } else if required {
            return Err(err(format!("Missing nonlinear {key}")));
        }
    }
    if behavior == "friction"
        && (o.contains_key("normal_reaction") == o.contains_key("normal_reaction_source"))
    {
        return Err(err(
            "Friction requires exactly one normal reaction value or source",
        ));
    }
    if let Some(source) = o.get("normal_reaction_source") {
        let s = object(source, &["support_ref", "dof"])?;
        let target = entity(
            model,
            "supports",
            text(s.get("support_ref"), "support_ref")?,
        )?;
        if target.get("nonlinear").is_some_and(|n| !n.is_null()) {
            return Err(err("Friction normal source must be a linear support"));
        }
        let d = dof(s.get("dof"))?.to_ascii_uppercase();
        if !d.starts_with('U') {
            return Err(err("Friction normal source DOF must be translational"));
        }
        if !target
            .get("restraints")
            .and_then(Value::as_array)
            .is_some_and(|a| {
                a.iter()
                    .any(|v| v.as_str().is_some_and(|v| v.eq_ignore_ascii_case(&d)))
            })
        {
            return Err(err("Friction normal source DOF must be restrained"));
        }
        entity(model, "nodes", text(target.get("node"), "source.node")?)?;
    }
    Ok(())
}
fn validate_temperature_points(
    model: &Value,
    material: &Value,
    points: &Value,
    warnings: &mut Vec<String>,
) -> Result<()> {
    let points = points
        .as_array()
        .ok_or_else(|| err("temperature_points must be an array"))?;
    let mut ids = HashSet::new();
    let mut temperatures = Vec::new();
    for p in points {
        let o = object(
            p,
            &[
                "id",
                "temperature",
                "elastic_modulus",
                "shear_modulus",
                "thermal_expansion_coefficient",
                "provenance",
            ],
        )?;
        if !ids.insert(text(o.get("id"), "point.id")?) {
            return Err(err("Duplicate material temperature point id"));
        }
        optional_text(o, &["provenance"])?;
        for (key, dim, positive) in [
            ("temperature", Dimension::Temperature, false),
            ("elastic_modulus", Dimension::Stress, true),
            ("shear_modulus", Dimension::Stress, true),
            (
                "thermal_expansion_coefficient",
                Dimension::ThermalExpansionCoefficient,
                false,
            ),
        ] {
            if let Some(v) = o.get(key) {
                let q = quantity(v, dim, positive)?;
                if key == "temperature" {
                    temperatures.push((q, p));
                }
            }
        }
        if [
            "temperature",
            "elastic_modulus",
            "shear_modulus",
            "thermal_expansion_coefficient",
        ]
        .iter()
        .any(|k| !o.contains_key(*k))
        {
            warnings.push(format!(
                "Temperature point {} is incomplete for interpolation",
                p["id"]
            ));
        }
    }
    temperatures.sort_by(|a, b| a.0.total_cmp(&b.0));
    if temperatures.windows(2).any(|w| w[0].0 == w[1].0) {
        return Err(err("Duplicate normalized material temperatures"));
    }
    let used = model
        .get("pipe_segments")
        .and_then(Value::as_array)
        .is_some_and(|pipes| {
            pipes
                .iter()
                .any(|p| p.get("material") == material.get("id"))
        });
    if !used {
        return Ok(());
    }
    for case in model
        .get("load_cases")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let exact = case.get("modulus_basis_ref").filter(|v| !v.is_null());
        let temp = case
            .get("modulus_basis_temperature")
            .filter(|v| !v.is_null());
        if exact.is_some() && temp.is_some() {
            return Err(err("Load case declares conflicting modulus bases"));
        }
        if let Some(id) = exact {
            let id = text(Some(id), "modulus_basis_ref")?;
            let p = points
                .iter()
                .find(|p| p["id"].as_str() == Some(id))
                .ok_or_else(|| err(format!("Selected modulus basis {id} cannot be deleted")))?;
            for k in ["elastic_modulus", "shear_modulus"] {
                if p.get(k).is_none() {
                    return Err(err(format!("Selected point {id} requires {k}")));
                }
            }
        }
        if let Some(t) = temp {
            let t = quantity(t, Dimension::Temperature, false)?;
            let bracket=temperatures.windows(2).find(|w|w[0].0<t && t<w[1].0).ok_or_else(||err("Selected solve temperature requires a strict adjacent bracket; no extrapolation"))?;
            for (_, p) in bracket {
                for k in [
                    "elastic_modulus",
                    "shear_modulus",
                    "thermal_expansion_coefficient",
                ] {
                    if p.get(k).is_none() {
                        return Err(err(format!("Interpolation point requires {k}")));
                    }
                }
            }
        }
    }
    Ok(())
}
fn validate_exposure(model: &Value, v: &Value, warnings: &mut Vec<String>) -> Result<()> {
    let o = object(v, &["exposed_pipe_refs", "exposed_spans"])?;
    let refs = o
        .get("exposed_pipe_refs")
        .and_then(Value::as_array)
        .ok_or_else(|| err("exposed_pipe_refs must be an array"))?;
    let spans = o
        .get("exposed_spans")
        .and_then(Value::as_array)
        .ok_or_else(|| err("exposed_spans must be an array"))?;
    let mut whole = HashSet::new();
    for r in refs {
        let r = text(Some(r), "pipe_ref")?;
        entity(model, "pipe_segments", r)?;
        if !whole.insert(r) {
            return Err(err("Duplicate whole wind exposure"));
        }
    }
    let mut extents: BTreeMap<&str, Vec<(f64, f64)>> = BTreeMap::new();
    for span in spans {
        let o = object(span, &["pipe_ref", "start_fraction", "end_fraction"])?;
        let r = text(o.get("pipe_ref"), "pipe_ref")?;
        entity(model, "pipe_segments", r)?;
        if whole.contains(r) {
            return Err(err("Pipe cannot have both whole and partial wind exposure"));
        }
        let start = quantity(
            o.get("start_fraction")
                .ok_or_else(|| err("Missing start_fraction"))?,
            Dimension::Dimensionless,
            false,
        )?;
        let end = quantity(
            o.get("end_fraction")
                .ok_or_else(|| err("Missing end_fraction"))?,
            Dimension::Dimensionless,
            false,
        )?;
        if start < 0.0 || start >= end || end > 1.0 {
            return Err(err("Wind extent requires 0 <= start < end <= 1"));
        }
        extents.entry(r).or_default().push((start, end));
    }
    for list in extents.values_mut() {
        list.sort_by(|a, b| a.0.total_cmp(&b.0));
        if list.windows(2).any(|w| w[1].0 < w[0].1) {
            return Err(err("Wind extents overlap on the same pipe"));
        }
    }
    if refs.is_empty() && spans.is_empty() {
        warnings.push("Wind exposure is empty; wind generation is not solve-ready".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn q(n: f64, u: &str) -> Value {
        json!({"value":n,"unit":u})
    }
    fn model() -> Value {
        json!({"nodes":[{"id":"n"}],"supports":[{"id":"s","node":"n","label":"keep","restraints":["UY"],"provenance":"entered"},{"id":"f","node":"n","restraints":[],"nonlinear":{"behavior":"friction","dof":"UX","initial_state":"sticking","friction_coefficient":{"value":0.2,"unit":"1"},"normal_reaction_source":{"support_ref":"s","dof":"UY"}}}],"materials":[{"id":"m"}],"pipe_segments":[{"id":"p","material":"m"}],"load_cases":[{"id":"l","equivalent_static":{"wind":{"pressure":{"value":1,"unit":"Pa"},"exposed_pipe_refs":[],"exposed_spans":[]}}}]})
    }
    fn edit(m: &Value, ty: &str, id: &str, path: &str, after: &Value) -> Result<RichEdit> {
        let before = match ty {
            "Support" => crate::canonical_json(&projection(entity(m, "supports", id)?, CONFIG)),
            "Material" => entity(m, "materials", id)?
                .get("temperature_points")
                .map(crate::canonical_json)
                .unwrap_or_else(|| "not_present".into()),
            _ => crate::canonical_json(&exposure(&m["load_cases"][0]["equivalent_static"]["wind"])),
        };
        resolve(
            m,
            ty,
            id,
            path,
            &before,
            &crate::canonical_json(after),
            "none",
            "dimensionless",
        )
        .map(|e| e.unwrap())
    }
    fn point(id: &str, t: f64) -> Value {
        json!({"id":id,"temperature":q(t,"K"),"elastic_modulus":q(200.0,"MPa"),"shear_modulus":q(80.0,"MPa"),"thermal_expansion_coefficient":q(-0.00001,"1/K")})
    }
    fn span(a: f64, b: f64) -> Value {
        json!({"pipe_ref":"p","start_fraction":q(a,"none"),"end_fraction":q(b,"1")})
    }
    #[test]
    fn stale_before_and_bad_envelope_fail_without_mutating_input() {
        let m = model();
        let original = m.clone();
        assert_eq!(
            resolve(
                &m,
                "Material",
                "m",
                "temperature_points",
                "[]",
                "[]",
                "none",
                "dimensionless"
            )
            .unwrap_err()
            .code,
            "OP-BEFORE-VALUE-MISMATCH"
        );
        assert!(resolve(
            &m,
            "Material",
            "m",
            "temperature_points",
            "not_present",
            "[]",
            "K",
            "temperature"
        )
        .is_err());
        assert_eq!(m, original);
        assert!(
            resolve(&m, "Node", "n", "label", "a", "b", "none", "dimensionless")
                .unwrap()
                .is_none()
        );
    }
    #[test]
    fn support_projection_preserves_identity_and_rejects_dependent_reaction_breakage() {
        let m = model();
        let p = projection(&m["supports"][0], CONFIG);
        let e = edit(&m, "Support", "s", "configuration", &p).unwrap();
        assert_eq!(e.current_display, crate::canonical_json(&p));
        assert!(!e
            .writes
            .iter()
            .any(|(path, _)| ["id", "node", "label"].contains(&path[0].as_str())));
        let mut bad = p.clone();
        bad["restraints"] = json!(["UX"]);
        assert!(edit(&m, "Support", "s", "configuration", &bad)
            .unwrap_err()
            .message
            .contains("restrained"));
        bad = p.clone();
        bad["nonlinear"] = json!({"behavior":"gap","dof":"UY","initial_state":"inactive","closes_when":"positive_displacement","gap":q(0.0,"mm")});
        assert!(edit(&m, "Support", "s", "configuration", &bad)
            .unwrap_err()
            .message
            .contains("linear support"));
        bad = p;
        bad["id"] = json!("overwrite");
        assert!(edit(&m, "Support", "s", "configuration", &bad).is_err());
    }
    #[test]
    fn variable_hanger_requires_explicit_loads_travel_and_correct_stiffness_units() {
        let m = model();
        let mut s = json!({"id":"new","node":"n","restraints":["UY"],"family":"variable_spring_hanger","hanger":{"hanger_type":"variable_spring_hanger","source_reference":"user","stiffness":{"dof":"UY","value":q(20.0,"N/m")},"installed_load":q(10.0,"N"),"cold_load":q(11.0,"N"),"hot_load":q(12.0,"N"),"movement_limit":q(1.0,"in")}});
        validate_support_configuration(&m, &s).unwrap();
        let original = s.clone();
        s["hanger"].as_object_mut().unwrap().remove("cold_load");
        assert!(validate_support_configuration(&m, &s).is_err());
        s = original.clone();
        s["hanger"]["stiffness"]["value"] = q(1.0, "N");
        assert!(validate_support_configuration(&m, &s).is_err());
        s = original.clone();
        s["hanger"]["hanger_type"] = json!("invented");
        assert!(validate_support_configuration(&m, &s).is_err());
        s = original;
        s["hanger"]["stiffness"]["value"]["extra"] = json!(true);
        assert!(validate_support_configuration(&m, &s).is_err());
    }
    #[test]
    fn constant_hanger_and_spring_have_no_inferred_inputs() {
        let m = model();
        let mut s = json!({"id":"new","node":"n","restraints":[],"family":"constant_effort_support","hanger":{"hanger_type":"constant_effort_support","source_reference":"user","constant_load":q(2.0,"lbf"),"travel_range":q(1.0,"in")}});
        // Empty acting DOF remains existing consumer review-only semantics.
        validate_support_configuration(&m, &s).unwrap();
        s["hanger"]["constant_load"] = q(0.0, "N");
        assert!(validate_support_configuration(&m, &s).is_err());
        s = json!({"node":"n","restraints":["UX"],"family":"spring"});
        assert!(validate_support_configuration(&m, &s).is_err());
        s["stiffness"] = json!({"dof":"RX","value":q(3.0,"N*m/rad")});
        validate_support_configuration(&m, &s).unwrap();
    }
    #[test]
    fn nonlinear_aliases_and_explicit_normal_sources_match_consumers() {
        let m = model();
        for behavior in [
            "one_way", "one-way", "oneway", "lift_off", "lift-off", "liftoff",
        ] {
            let n = json!({"behavior":behavior,"dof":"ux","initial_state":"ACTIVE","active_when":"positive","contact_when":"NEGATIVE_REACTION"});
            validate_nonlinear(&m, &n).unwrap();
        }
        let mut n = m["supports"][1]["nonlinear"].clone();
        validate_nonlinear(&m, &n).unwrap();
        n["normal_reaction"] = q(1.0, "N");
        assert!(validate_nonlinear(&m, &n).is_err());
        n.as_object_mut().unwrap().remove("normal_reaction_source");
        validate_nonlinear(&m, &n).unwrap();
        n["friction_coefficient"] = q(-0.1, "none");
        assert!(validate_nonlinear(&m, &n).is_err());
        n["friction_coefficient"] = q(0.1, "N");
        assert!(validate_nonlinear(&m, &n).is_err());
        n["friction_coefficient"] = q(0.1, "1");
        n["initial_state"] = json!("guess");
        assert!(validate_nonlinear(&m, &n).is_err());
    }
    #[test]
    fn material_partial_rows_warn_and_exact_selection_cannot_be_broken() {
        let mut m = model();
        let e = edit(
            &m,
            "Material",
            "m",
            "temperature_points",
            &json!([{"id":"hot"}]),
        )
        .unwrap();
        assert_eq!(e.warnings.len(), 1);
        m["load_cases"][0]["modulus_basis_ref"] = json!("hot");
        assert!(edit(&m, "Material", "m", "temperature_points", &json!([])).is_err());
        assert!(edit(
            &m,
            "Material",
            "m",
            "temperature_points",
            &json!([{"id":"hot"}])
        )
        .is_err());
        let p =
            json!([{ "id":"hot","elastic_modulus":q(200.0,"MPa"),"shear_modulus":q(80.0,"MPa") }]);
        edit(&m, "Material", "m", "temperature_points", &p).unwrap();
    }
    #[test]
    fn interpolation_requires_strict_bracket_and_all_endpoint_properties() {
        let mut m = model();
        m["load_cases"][0]["modulus_basis_temperature"] = q(350.0, "K");
        let points = json!([point("cold", 300.0), point("hot", 400.0)]);
        let e = edit(&m, "Material", "m", "temperature_points", &points).unwrap();
        assert!(e.warnings.is_empty());
        assert_eq!(
            crate::canonical_json(e.writes[0].1.as_ref().unwrap()),
            crate::canonical_json(&points)
        );
        let mut missing = points.clone();
        missing[1]
            .as_object_mut()
            .unwrap()
            .remove("thermal_expansion_coefficient");
        assert!(edit(&m, "Material", "m", "temperature_points", &missing).is_err());
        m["load_cases"][0]["modulus_basis_temperature"] = q(400.0, "K");
        assert!(edit(&m, "Material", "m", "temperature_points", &points).is_err());
        m["load_cases"][0]["modulus_basis_temperature"] = q(500.0, "K");
        assert!(edit(&m, "Material", "m", "temperature_points", &points).is_err());
    }
    #[test]
    fn table_rejects_duplicate_ids_normalized_temperatures_and_unknown_fields() {
        let m = model();
        assert!(edit(
            &m,
            "Material",
            "m",
            "temperature_points",
            &json!([point("x", 300.0), point("x", 400.0)])
        )
        .is_err());
        let mut points = json!([point("x", 273.15), point("y", 400.0)]);
        points[1]["temperature"] = q(0.0, "degC");
        assert!(edit(&m, "Material", "m", "temperature_points", &points)
            .unwrap_err()
            .message
            .contains("Duplicate normalized"));
        points = json!([point("x", 300.0)]);
        points[0]["secret"] = json!(1);
        assert!(edit(&m, "Material", "m", "temperature_points", &points).is_err());
    }
    #[test]
    fn wind_atomic_switch_allows_adjacent_spans_and_preserves_authored_order() {
        let mut m = model();
        m["load_cases"][0]["equivalent_static"]["wind"]["exposed_pipe_refs"] = json!(["p"]);
        let after = json!({"exposed_pipe_refs":[],"exposed_spans":[span(0.5,1.0),span(0.0,0.5)]});
        let original = m.clone();
        let e = edit(&m, "Load", "l", "equivalent_static.wind.exposure", &after).unwrap();
        assert_eq!(
            crate::canonical_json(e.writes[1].1.as_ref().unwrap()),
            crate::canonical_json(&after["exposed_spans"])
        );
        assert_eq!(m, original);
        let empty = edit(
            &m,
            "Load",
            "l",
            "equivalent_static.wind.exposure",
            &json!({"exposed_pipe_refs":[],"exposed_spans":[]}),
        )
        .unwrap();
        assert_eq!(empty.warnings.len(), 1);
    }
    #[test]
    fn wind_rejects_overlap_collision_missing_refs_units_and_fraction_bounds() {
        let m = model();
        for after in [
            json!({"exposed_pipe_refs":[],"exposed_spans":[span(0.0,0.7),span(0.5,1.0)]}),
            json!({"exposed_pipe_refs":["p"],"exposed_spans":[span(0.0,1.0)]}),
            json!({"exposed_pipe_refs":["absent"],"exposed_spans":[]}),
            json!({"exposed_pipe_refs":[],"exposed_spans":[span(-0.1,0.5)]}),
            json!({"exposed_pipe_refs":[],"exposed_spans":[span(0.5,0.5)]}),
        ] {
            assert!(edit(&m, "Load", "l", "equivalent_static.wind.exposure", &after).is_err());
        }
        let mut after = json!({"exposed_pipe_refs":[],"exposed_spans":[span(0.0,1.0)]});
        after["exposed_spans"][0]["end_fraction"] = q(1.0, "m");
        assert!(edit(&m, "Load", "l", "equivalent_static.wind.exposure", &after).is_err());
    }
}

//! Shared Section binding operations. All writes are made to a private candidate.
use super::*;

type BindingResult<T> = Result<T, String>;

fn unique<'a>(model: &'a Value, collection: &str, id: &str) -> BindingResult<&'a Value> {
    let rows = model
        .get(collection)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{collection} must be an array"))?;
    let matches: Vec<_> = rows
        .iter()
        .filter(|r| r.get("id").and_then(Value::as_str) == Some(id))
        .collect();
    if id.trim().is_empty() || matches.len() != 1 {
        return Err(format!("{collection} requires exactly one record {id}"));
    }
    Ok(matches[0])
}

fn length(record: &Value, key: &str) -> BindingResult<f64> {
    let quantity = record
        .get(key)
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{key} must be an explicit quantity"))?;
    if quantity.len() != 2 || !quantity.contains_key("value") || !quantity.contains_key("unit") {
        return Err(format!("{key} requires only value and unit"));
    }
    let entered = EnteredQuantity {
        value: quantity
            .get("value")
            .and_then(Value::as_f64)
            .ok_or_else(|| format!("{key} value must be numeric"))?,
        unit: quantity
            .get("unit")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{key} unit must be text"))?
            .to_string(),
    };
    quantity_value_in_unit(&entered, "m", Dimension::Length)
        .filter(|v| v.is_finite())
        .ok_or_else(|| format!("{key} requires an accepted finite length quantity"))
}

fn source<'a>(model: &'a Value, id: &str) -> BindingResult<&'a Value> {
    let section = unique(model, "sections", id)?;
    if section.get("name").and_then(Value::as_str).is_none() || section.get("provenance").is_none()
    {
        return Err("Shared Section requires a text name and explicit provenance metadata".into());
    }
    let properties = section
        .get("properties")
        .and_then(Value::as_object)
        .ok_or("Section properties must be an object")?;
    if section.get("section_type").and_then(Value::as_str) != Some("pipe")
        || properties.len() != 2
        || !properties.contains_key("outside_diameter")
        || !properties.contains_key("wall_thickness")
    {
        return Err(
            "Shared Section requires type pipe and only outside_diameter/wall_thickness properties"
                .into(),
        );
    }
    let props = &section["properties"];
    geometry(props)?;
    Ok(props)
}

fn geometry(section: &Value) -> BindingResult<()> {
    let od = length(section, "outside_diameter")?;
    let wall = length(section, "wall_thickness")?;
    if od <= 0.0 || wall <= 0.0 || wall >= od / 2.0 {
        return Err("Section requires 0 < wall < outside diameter / 2".into());
    }
    if section.get("mill_tolerance").is_some_and(|v| !v.is_null()) {
        let tolerance = length(section, "mill_tolerance")?;
        if tolerance < 0.0 || wall - tolerance <= 0.0 {
            return Err(
                "Mill tolerance must be nonnegative and leave a positive effective wall".into(),
            );
        }
    }
    Ok(())
}

fn check_cache(pipe: &Value, props: &Value) -> BindingResult<()> {
    let section = pipe
        .get("section")
        .ok_or("Pipe inline section is missing")?;
    for key in ["outside_diameter", "wall_thickness"] {
        if section.get(key) != props.get(key) {
            return Err(format!(
                "Pipe {} shared section cache is stale at {key}; explicitly resolve before use",
                pipe["id"]
            ));
        }
    }
    geometry(section)
}

fn binding_ref(pipe: &Value) -> BindingResult<Option<&str>> {
    match pipe.get("section_ref") {
        None => Ok(None),
        Some(Value::String(id)) if !id.trim().is_empty() => Ok(Some(id)),
        _ => Err("section_ref must be an explicit nonblank string or absent".into()),
    }
}

fn materialize(pipe: &mut Value, props: &Value) -> BindingResult<()> {
    let local = pipe
        .get_mut("section")
        .and_then(Value::as_object_mut)
        .ok_or("Pipe section must be an object")?;
    for key in ["outside_diameter", "wall_thickness"] {
        local.insert(key.to_string(), props[key].clone());
    }
    geometry(&pipe["section"])
}

pub(super) fn assign(
    model: &Value,
    target: &str,
    detach: bool,
    after: &str,
) -> BindingResult<Value> {
    let pipe = unique(model, "pipe_segments", target)?;
    let payload: Value =
        serde_json::from_str(after).map_err(|_| "Assignment payload must be JSON")?;
    let key = if detach {
        "source_section_ref"
    } else {
        "section_ref"
    };
    let map = payload
        .as_object()
        .ok_or("Assignment payload must be an object")?;
    if map.len() != 1 {
        return Err(format!("Assignment payload requires only {key}"));
    }
    let id = map
        .get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| format!("{key} must be explicit text"))?;
    if let Some(old_id) = binding_ref(pipe)? {
        check_cache(pipe, source(model, old_id)?)?;
    }
    if detach && binding_ref(pipe)? != Some(id) {
        return Err("Detach source does not match the current shared Section".into());
    }
    let props = source(model, id)?;
    let mut candidate = model.clone();
    let next = find_entity_mut(&mut candidate, "pipe_segments", target).unwrap();
    materialize(next, props)?;
    if detach {
        next.as_object_mut().unwrap().remove("section_ref");
    } else {
        next["section_ref"] = Value::String(id.into());
    }
    Ok(candidate)
}

/// Verify old caches before writing any new shared geometry. Ordering is stable
/// by pipe ID, independent of discovery order in the input arrays.
pub(super) fn propagate(
    model: &Value,
    candidate: &mut Value,
    id: &str,
) -> BindingResult<Vec<DiffPreviewRow>> {
    unique(model, "sections", id)?;
    let rows = model
        .get("pipe_segments")
        .and_then(Value::as_array)
        .ok_or("pipe_segments must be an array")?;
    let mut bound = Vec::new();
    for pipe in rows {
        if binding_ref(pipe)? == Some(id) {
            let pipe_id = pipe
                .get("id")
                .and_then(Value::as_str)
                .ok_or("Pipe ID is missing")?;
            unique(model, "pipe_segments", pipe_id)?;
            bound.push((pipe_id, pipe));
        }
    }
    if bound.is_empty() {
        return Ok(vec![]);
    }
    let old_props = source(model, id)?;
    let new_props = source(candidate, id)?.clone();
    bound.sort_by_key(|(pipe_id, _)| *pipe_id);
    let mut diffs = Vec::new();
    for (pipe_id, old) in bound {
        check_cache(old, old_props)?;
        let next = find_entity_mut(candidate, "pipe_segments", pipe_id).unwrap();
        materialize(next, &new_props)?;
        if next["section"] != old["section"] {
            diffs.push(DiffPreviewRow {
                entity_ref: pipe_id.into(),
                object_type: "Element".into(),
                field_path: "section".into(),
                before: canonical_json(&old["section"]),
                after: canonical_json(&next["section"]),
                unit: "none".into(),
                dimension: "dimensionless".into(),
                change_kind: "set_field".into(),
            });
        }
    }
    Ok(diffs)
}

pub(super) fn validate_local(
    model: &Value,
    candidate: &Value,
    id: &str,
    path: &str,
) -> BindingResult<()> {
    let old = unique(model, "pipe_segments", id)?;
    if let Some(reference) = binding_ref(old)? {
        if matches!(
            path,
            "section.outside_diameter.value" | "section.wall_thickness.value"
        ) {
            return Err(
                "Edit the shared Section or explicitly detach before editing bound pipe OD/wall"
                    .into(),
            );
        }
        check_cache(old, source(model, reference)?)?;
        check_cache(
            unique(candidate, "pipe_segments", id)?,
            source(candidate, reference)?,
        )?;
    }
    Ok(())
}

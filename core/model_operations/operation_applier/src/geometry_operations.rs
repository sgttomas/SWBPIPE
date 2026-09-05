//! Bounded geometry-only operations; called solely by the common applier.
use super::{
    canonical_json, is_valid_schema_id, quantity_value_in_unit, DiffPreviewRow, Dimension,
    EnteredQuantity,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) struct GeometryEdit {
    pub model: Value,
    pub diff_preview: Vec<DiffPreviewRow>,
}
pub(crate) struct GeometryError {
    pub code: &'static str,
    pub message: String,
    pub affected_refs: Vec<String>,
}
type Result<T> = std::result::Result<T, GeometryError>;
type Vec3 = [f64; 3];
fn err(message: impl Into<String>) -> GeometryError {
    GeometryError {
        code: "OP-GEOMETRY-INVALID",
        message: message.into(),
        affected_refs: vec![],
    }
}
fn fields(value: &Value, required: &[&str], optional: &[&str]) -> Result<()> {
    let map = value
        .as_object()
        .ok_or_else(|| err("Expected an explicit object"))?;
    if required.iter().any(|k| !map.contains_key(*k))
        || map
            .keys()
            .any(|k| !required.contains(&k.as_str()) && !optional.contains(&k.as_str()))
    {
        return Err(err(format!(
            "Required fields {required:?}; optional {optional:?}; unknown fields are unsupported"
        )));
    }
    Ok(())
}
fn text<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    v.get(k)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| err(format!("{k} must be nonblank text")))
}
fn number(v: &Value) -> Result<f64> {
    v.as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| err("Expected finite number"))
}
fn vector(v: &Value) -> Result<Vec3> {
    fields(v, &["x", "y", "z"], &[])?;
    Ok([number(&v["x"])?, number(&v["y"])?, number(&v["z"])?])
}
fn value(v: Vec3) -> Result<Value> {
    if !v.iter().all(|n| n.is_finite()) {
        return Err(err("Realized coordinates or direction overflow"));
    }
    Ok(json!({"x":v[0],"y":v[1],"z":v[2]}))
}
fn add(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn scale(a: Vec3, s: f64) -> Vec3 {
    [a[0] * s, a[1] * s, a[2] * s]
}
fn dot(a: Vec3, b: Vec3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn normalized(v: Vec3) -> Result<Vec3> {
    let norm = dot(v, v).sqrt();
    if !norm.is_finite() || norm <= 1e-12 {
        return Err(err(
            "Degenerate or nonfinite axis/length (frame kernel AXIS_TOLERANCE 1e-12)",
        ));
    }
    Ok(scale(v, 1.0 / norm))
}
fn convert(n: f64, from: &str, to: &str, dim: Dimension) -> Result<f64> {
    quantity_value_in_unit(
        &EnteredQuantity {
            value: n,
            unit: from.into(),
        },
        to,
        dim,
    )
    .filter(|x| x.is_finite())
    .ok_or_else(|| err("Unavailable accepted-catalog unit conversion or nonfinite result"))
}
fn length_vector(v: &Value, unit: &str) -> Result<Vec3> {
    fields(v, &["x", "y", "z", "unit"], &[])?;
    let from = text(v, "unit")?;
    Ok([
        convert(number(&v["x"])?, from, unit, Dimension::Length)?,
        convert(number(&v["y"])?, from, unit, Dimension::Length)?,
        convert(number(&v["z"])?, from, unit, Dimension::Length)?,
    ])
}
fn rows<'a>(model: &'a Value, key: &str) -> Result<&'a Vec<Value>> {
    model
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| err(format!("{key} must be an array")))
}
fn unique<'a>(model: &'a Value, key: &str, id: &str) -> Result<&'a Value> {
    let matching: Vec<_> = rows(model, key)?
        .iter()
        .filter(|r| r.get("id").and_then(Value::as_str) == Some(id))
        .collect();
    if matching.len() != 1 {
        return Err(err(format!("Expected exactly one {key} record {id}")));
    }
    Ok(matching[0])
}
fn record_shape(record: &Value, node: bool) -> Result<()> {
    for key in ["label", "provenance"] {
        if record
            .get(key)
            .is_some_and(|v| !v.is_null() && !v.is_string())
        {
            return Err(err(format!("Selected {key} metadata must be text or null")));
        }
    }
    if node {
        fields(record, &["id", "position"], &["label", "provenance"])?;
        vector(&record["position"])?;
    } else {
        fields(
            record,
            &["id", "from", "to", "section", "material"],
            &["label", "provenance", "y_reference", "section_ref"],
        )?;
        fields(
            &record["section"],
            &["outside_diameter", "wall_thickness"],
            &[
                "mill_tolerance",
                "material_density",
                "contents_density",
                "insulation_thickness",
                "insulation_density",
            ],
        )?;
        for (_, q) in record["section"].as_object().unwrap() {
            if !q.is_null() {
                fields(q, &["value", "unit"], &[])?;
                number(&q["value"])?;
                text(q, "unit")?;
            }
        }
        text(record, "material")?;
        text(record, "from")?;
        text(record, "to")?;
        if let Some(reference) = record.get("section_ref") {
            if reference
                .as_str()
                .filter(|s| !s.trim().is_empty())
                .is_none()
            {
                return Err(err("section_ref must be nonblank text"));
            }
        }
    }
    Ok(())
}
fn endpoints(model: &Value, pipes: &[Value]) -> Result<Vec<Value>> {
    let mut ids = BTreeSet::new();
    for p in pipes {
        ids.insert(text(p, "from")?);
        ids.insert(text(p, "to")?);
    }
    ids.into_iter()
        .map(|id| {
            let n = unique(model, "nodes", id)?;
            record_shape(n, true)?;
            Ok(n.clone())
        })
        .collect()
}
fn global_ids(v: &Value, ids: &mut BTreeSet<String>) {
    match v {
        Value::Object(m) => {
            if let Some(id) = m.get("id").and_then(Value::as_str) {
                ids.insert(id.into());
            }
            for x in m.values() {
                global_ids(x, ids);
            }
        }
        Value::Array(a) => {
            for x in a {
                global_ids(x, ids)
            }
        }
        _ => {}
    }
}
fn metadata(v: &Value, ids: &mut BTreeSet<String>, copy: bool) -> Result<()> {
    fields(
        v,
        if copy {
            &["source_ref", "id", "label", "provenance"]
        } else {
            &["id", "label", "provenance"]
        },
        &[],
    )?;
    let id = text(v, "id")?;
    if !is_valid_schema_id(id) || !ids.insert(id.into()) {
        return Err(err(format!("New ID invalid or collides globally: {id}")));
    }
    text(v, "label")?;
    text(v, "provenance")?;
    Ok(())
}
// Detect any exact selected reference outside understood records. Text metadata
// is not a link; all other selected-ID occurrences fail closed, including aliases.
fn references(v: &Value, ids: &BTreeSet<String>, path: &str, out: &mut Vec<String>) {
    match v {
        Value::String(s) if ids.contains(s) => out.push(path.into()),
        Value::Array(a) => {
            for (i, x) in a.iter().enumerate() {
                references(x, ids, &format!("{path}[{i}]"), out);
            }
        }
        Value::Object(m) => {
            for (k, x) in m {
                if !matches!(k.as_str(), "label" | "name" | "provenance") {
                    references(x, ids, &format!("{path}.{k}"), out);
                }
            }
        }
        _ => {}
    }
}
fn attachments(
    model: &Value,
    pipes: &[Value],
    nodes: &[Value],
    split: bool,
    in_place: bool,
) -> Result<()> {
    let pipe_ids: BTreeSet<String> = pipes
        .iter()
        .map(|p| text(p, "id").map(str::to_string))
        .collect::<Result<_>>()?;
    let node_ids: BTreeSet<String> = nodes
        .iter()
        .map(|n| text(n, "id").map(str::to_string))
        .collect::<Result<_>>()?;
    let mut all = pipe_ids.clone();
    all.extend(node_ids.iter().cloned());
    let mut found = vec![];
    for (key, collection) in model.as_object().unwrap() {
        if key == "nodes" {
            for node in rows(model, "nodes")? {
                if !node_ids.contains(text(node, "id")?) {
                    references(node, &all, "nodes", &mut found);
                }
            }
            continue;
        }
        if key == "pipe_segments" {
            for pipe in rows(model, key)? {
                if pipe_ids.contains(text(pipe, "id")?) {
                    continue;
                }
                if in_place
                    && (node_ids.contains(text(pipe, "from")?)
                        || node_ids.contains(text(pipe, "to")?))
                {
                    found.push(format!("unselected incident pipe {}", pipe["id"]));
                }
                let mut rest = pipe.clone();
                if let Some(m) = rest.as_object_mut() {
                    m.remove("from");
                    m.remove("to");
                }
                references(&rest, &all, key, &mut found);
            }
            continue;
        }
        let set = if split && key != "components" {
            &pipe_ids
        } else {
            &all
        };
        references(collection, set, key, &mut found);
    }
    if found.is_empty() {
        Ok(())
    } else {
        Err(GeometryError {
            code: "OP-GEOMETRY-ATTACHMENT-UNSUPPORTED",
            message: format!(
                "Geometry-only operation cannot transform/repartition attachments: {}",
                found.join(", ")
            ),
            affected_refs: all.into_iter().collect(),
        })
    }
}
fn frame(model: &Value, pipe: &Value, unit: &str, require_y: bool) -> Result<()> {
    let a = vector(&unique(model, "nodes", text(pipe, "from")?)?["position"])?;
    let b = vector(&unique(model, "nodes", text(pipe, "to")?)?["position"])?;
    let delta = sub(b, a);
    let mut meters = [0.; 3];
    for i in 0..3 {
        meters[i] = convert(delta[i], unit, "m", Dimension::Length)?;
    }
    let x = normalized(meters)?;
    if let Some(y) = pipe.get("y_reference") {
        let y = vector(y)?;
        normalized(sub(y, scale(x, dot(y, x))))?;
    } else if require_y {
        return Err(err(
            "Rotate/mirror requires explicit nondegenerate y_reference",
        ));
    }
    Ok(())
}
fn diff(
    id: &str,
    ty: &str,
    path: &str,
    before: Option<&Value>,
    after: &Value,
    kind: &str,
) -> DiffPreviewRow {
    DiffPreviewRow {
        entity_ref: id.into(),
        object_type: ty.into(),
        field_path: path.into(),
        before: before
            .map(canonical_json)
            .unwrap_or_else(|| "not_present".into()),
        after: canonical_json(after),
        unit: "none".into(),
        dimension: "dimensionless".into(),
        change_kind: kind.into(),
    }
}
fn split(model: &Value, intent: &Value, payload: &Value, unit: &str) -> Result<GeometryEdit> {
    fields(
        payload,
        &["pipe_ref", "fraction", "new_node", "new_pipe"],
        &[],
    )?;
    let id = text(payload, "pipe_ref")?;
    if text(&intent["target"], "ref")? != id {
        return Err(err("Split target and pipe_ref must agree"));
    }
    let source = unique(model, "pipe_segments", id)?;
    record_shape(source, false)?;
    if text(&intent["change"], "before")? != canonical_json(source) {
        return Err(err("Stale canonical source pipe"));
    }
    let f = number(&payload["fraction"])?;
    if f <= 0. || f >= 1. {
        return Err(err("fraction must be strictly between zero and one"));
    }
    let nodes = endpoints(model, &[source.clone()])?;
    attachments(model, &[source.clone()], &nodes, true, false)?;
    frame(model, source, unit, false)?;
    let mut ids = BTreeSet::new();
    global_ids(model, &mut ids);
    metadata(&payload["new_node"], &mut ids, false)?;
    metadata(&payload["new_pipe"], &mut ids, false)?;
    let a = vector(&unique(model, "nodes", text(source, "from")?)?["position"])?;
    let b = vector(&unique(model, "nodes", text(source, "to")?)?["position"])?;
    let point = value(add(a, scale(sub(b, a), f)))?;
    let mut node = payload["new_node"].clone();
    node["position"] = point;
    let mut first = source.clone();
    first["to"] = node["id"].clone();
    let mut second = source.clone();
    second["from"] = node["id"].clone();
    for k in ["id", "label", "provenance"] {
        second[k] = payload["new_pipe"][k].clone();
    }
    let mut candidate = model.clone();
    candidate["nodes"]
        .as_array_mut()
        .unwrap()
        .push(node.clone());
    let list = candidate["pipe_segments"].as_array_mut().unwrap();
    let i = list.iter().position(|p| p["id"] == source["id"]).unwrap();
    list[i] = first.clone();
    list.push(second.clone());
    frame(&candidate, &first, unit, false)?;
    frame(&candidate, &second, unit, false)?;
    Ok(GeometryEdit {
        model: candidate,
        diff_preview: vec![
            diff(
                id,
                "Element",
                "pipe_segments",
                Some(source),
                &first,
                "split_pipe_run",
            ),
            diff(
                text(&node, "id")?,
                "Node",
                "nodes",
                None,
                &node,
                "create_node",
            ),
            diff(
                text(&second, "id")?,
                "Element",
                "pipe_segments",
                None,
                &second,
                "connect_pipe_run",
            ),
        ],
    })
}

enum Transform {
    Translate(Vec3),
    Rotate {
        origin: Vec3,
        axis: Vec3,
        angle: f64,
    },
    Mirror {
        origin: Vec3,
        normal: Vec3,
    },
}
impl Transform {
    fn parse(v: &Value, unit: &str) -> Result<Self> {
        match text(v, "kind")? {
            "translate" => {
                fields(v, &["kind", "translation"], &[])?;
                Ok(Self::Translate(length_vector(&v["translation"], unit)?))
            }
            "rotate" => {
                fields(v, &["kind", "origin", "axis", "angle"], &[])?;
                fields(&v["angle"], &["value", "unit"], &[])?;
                Ok(Self::Rotate {
                    origin: length_vector(&v["origin"], unit)?,
                    axis: normalized(vector(&v["axis"])?)?,
                    angle: convert(
                        number(&v["angle"]["value"])?,
                        text(&v["angle"], "unit")?,
                        "rad",
                        Dimension::Angle,
                    )?,
                })
            }
            "mirror" => {
                fields(v, &["kind", "origin", "normal"], &[])?;
                Ok(Self::Mirror {
                    origin: length_vector(&v["origin"], unit)?,
                    normal: normalized(vector(&v["normal"])?)?,
                })
            }
            _ => Err(err("Unsupported transform kind")),
        }
    }
    fn direction(&self, v: Vec3) -> Vec3 {
        match self {
            Self::Translate(_) => v,
            Self::Rotate { axis, angle, .. } => {
                let (c, s) = (angle.cos(), angle.sin());
                add(
                    add(scale(v, c), scale(cross(*axis, v), s)),
                    scale(*axis, dot(*axis, v) * (1. - c)),
                )
            }
            Self::Mirror { normal, .. } => sub(v, scale(*normal, 2. * dot(*normal, v))),
        }
    }
    fn point(&self, v: Vec3) -> Vec3 {
        match self {
            Self::Translate(t) => add(v, *t),
            Self::Rotate { origin, .. } | Self::Mirror { origin, .. } => {
                add(*origin, self.direction(sub(v, *origin)))
            }
        }
    }
}
fn maps(
    value: &Value,
    sources: &[Value],
    ids: &mut BTreeSet<String>,
) -> Result<BTreeMap<String, Value>> {
    let entries = value
        .as_array()
        .ok_or_else(|| err("Copy requires explicit maps"))?;
    let wanted: BTreeSet<String> = sources
        .iter()
        .map(|s| text(s, "id").map(str::to_string))
        .collect::<Result<_>>()?;
    let mut result = BTreeMap::new();
    for entry in entries {
        metadata(entry, ids, true)?;
        let source = text(entry, "source_ref")?;
        if !wanted.contains(source) || result.insert(source.into(), entry.clone()).is_some() {
            return Err(err("Copy map source is extra or duplicated"));
        }
    }
    if result.len() != wanted.len() {
        return Err(err(
            "Copy map must cover every selected pipe and endpoint exactly once",
        ));
    }
    Ok(result)
}
fn transform(model: &Value, intent: &Value, payload: &Value, unit: &str) -> Result<GeometryEdit> {
    fields(
        payload,
        &["mode", "pipe_refs", "transform"],
        &["copy_nodes", "copy_pipes"],
    )?;
    let copy = match text(payload, "mode")? {
        "copy" => true,
        "in_place" => false,
        _ => return Err(err("mode must be copy or in_place")),
    };
    if !copy && (payload.get("copy_nodes").is_some() || payload.get("copy_pipes").is_some()) {
        return Err(err("in_place forbids copy maps"));
    }
    let ids = payload["pipe_refs"]
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or_else(|| err("pipe_refs requires nonempty array"))?;
    if ids[0].as_str() != Some(text(&intent["target"], "ref")?) {
        return Err(err("Target must equal first selected pipe"));
    }
    let mut seen = BTreeSet::new();
    let mut pipes = Vec::new();
    for id in ids {
        let id = id
            .as_str()
            .ok_or_else(|| err("pipe_refs must contain IDs"))?;
        if !seen.insert(id) {
            return Err(err("Duplicate selected pipe"));
        }
        let pipe = unique(model, "pipe_segments", id)?;
        record_shape(pipe, false)?;
        pipes.push(pipe.clone());
    }
    pipes.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
    let nodes = endpoints(model, &pipes)?;
    if text(&intent["change"], "before")?
        != canonical_json(&json!({"nodes":nodes,"pipe_segments":pipes}))
    {
        return Err(err("Stale canonical transform selection"));
    }
    attachments(model, &pipes, &nodes, false, !copy)?;
    let transform = Transform::parse(&payload["transform"], unit)?;
    let require_y = !matches!(transform, Transform::Translate(_));
    for p in &pipes {
        frame(model, p, unit, require_y)?;
    }
    let mut all = BTreeSet::new();
    global_ids(model, &mut all);
    let (node_map, pipe_map) = if copy {
        (
            maps(&payload["copy_nodes"], &nodes, &mut all)?,
            maps(&payload["copy_pipes"], &pipes, &mut all)?,
        )
    } else {
        (BTreeMap::new(), BTreeMap::new())
    };
    let mut candidate = model.clone();
    let mut diffs = Vec::new();
    for old in &nodes {
        let id = text(old, "id")?;
        let mut next = old.clone();
        next["position"] = value(transform.point(vector(&old["position"])?))?;
        if copy {
            for k in ["id", "label", "provenance"] {
                next[k] = node_map[id][k].clone();
            }
            candidate["nodes"]
                .as_array_mut()
                .unwrap()
                .push(next.clone());
        } else {
            let list = candidate["nodes"].as_array_mut().unwrap();
            let i = list.iter().position(|r| r["id"] == old["id"]).unwrap();
            list[i] = next.clone();
        }
        if copy || next != *old {
            diffs.push(diff(
                text(&next, "id")?,
                "Node",
                "nodes",
                if copy { None } else { Some(old) },
                &next,
                "transform_pipe_run",
            ));
        }
    }
    for old in &pipes {
        let id = text(old, "id")?;
        let mut next = old.clone();
        if let Some(y) = old.get("y_reference") {
            next["y_reference"] = value(transform.direction(vector(y)?))?;
        }
        if copy {
            for k in ["id", "label", "provenance"] {
                next[k] = pipe_map[id][k].clone();
            }
            for k in ["from", "to"] {
                next[k] = node_map[text(old, k)?]["id"].clone();
            }
            candidate["pipe_segments"]
                .as_array_mut()
                .unwrap()
                .push(next.clone());
        } else {
            let list = candidate["pipe_segments"].as_array_mut().unwrap();
            let i = list.iter().position(|r| r["id"] == old["id"]).unwrap();
            list[i] = next.clone();
        }
        frame(&candidate, &next, unit, require_y)?;
        if copy || next != *old {
            diffs.push(diff(
                text(&next, "id")?,
                "Element",
                "pipe_segments",
                if copy { None } else { Some(old) },
                &next,
                "transform_pipe_run",
            ));
        }
    }
    Ok(GeometryEdit {
        model: candidate,
        diff_preview: diffs,
    })
}
pub(crate) fn resolve_geometry(model: &Value, intent: &Value) -> Result<Option<GeometryEdit>> {
    let kind = intent["change"]["change_kind"].as_str().unwrap_or("");
    if !matches!(kind, "split_pipe_run" | "transform_pipe_run") {
        return Ok(None);
    }
    if model["document_kind"] != "openpipestress.product_preview.model"
        || model["schema_version"] != "0.1.0"
    {
        return Err(err(
            "Geometry operations require recognized product_preview.model schema 0.1.0",
        ));
    }
    if intent["operation_kind"] != "modify"
        || intent["target"]["object_type"] != "Element"
        || intent["change"]["field_path"] != "pipe_segments"
        || intent["change"]["unit"] != "none"
        || intent["change"]["dimension"] != "dimensionless"
    {
        return Err(err(
            "Geometry wire requires modify/Element/pipe_segments/none/dimensionless",
        ));
    }
    let unit = text(&model["project"]["units"], "length")?;
    convert(1., unit, "m", Dimension::Length)?;
    let payload: Value = serde_json::from_str(text(&intent["change"], "after")?)
        .map_err(|_| err("after must be JSON"))?;
    if kind == "split_pipe_run" {
        split(model, intent, &payload, unit).map(Some)
    } else {
        transform(model, intent, &payload, unit).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn model() -> Value {
        json!({"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","project":{"id":"project:test","units":{"length":"m"}},"nodes":[{"id":"node:a","label":"A","position":{"x":0.,"y":0.,"z":0.},"provenance":"user"},{"id":"node:b","label":"B","position":{"x":4.,"y":0.,"z":0.},"provenance":"user"}],"pipe_segments":[{"id":"pipe:a","label":"Pipe","from":"node:a","to":"node:b","section":{"outside_diameter":{"value":0.1,"unit":"m"},"wall_thickness":{"value":0.005,"unit":"m"},"contents_density":{"value":800.,"unit":"kg/m^3"}},"material":"material:a","y_reference":{"x":0.,"y":1.,"z":0.},"provenance":"user"}],"materials":[{"id":"material:a"}],"supports":[],"components":[],"load_cases":[]})
    }
    fn split_payload() -> Value {
        json!({"pipe_ref":"pipe:a","fraction":0.25,"new_node":{"id":"node:new","label":"New","provenance":"user"},"new_pipe":{"id":"pipe:new","label":"New pipe","provenance":"user"}})
    }
    fn transform_payload(kind: &str, copy: bool) -> Value {
        let transform = match kind {
            "translate" => {
                json!({"kind":kind,"translation":{"x":1000.,"y":2000.,"z":0.,"unit":"mm"}})
            }
            "rotate" => {
                json!({"kind":kind,"origin":{"x":0.,"y":0.,"z":0.,"unit":"m"},"axis":{"x":0.,"y":0.,"z":1.},"angle":{"value":90.,"unit":"deg"}})
            }
            _ => {
                json!({"kind":"mirror","origin":{"x":1.,"y":0.,"z":0.,"unit":"m"},"normal":{"x":1.,"y":0.,"z":0.}})
            }
        };
        let mut p = json!({"mode":if copy{"copy"}else{"in_place"},"pipe_refs":["pipe:a"],"transform":transform});
        if copy {
            p["copy_nodes"] = json!([{"source_ref":"node:a","id":"node:c","label":"C","provenance":"user-copy"},{"source_ref":"node:b","id":"node:d","label":"D","provenance":"user-copy"}]);
            p["copy_pipes"] = json!([{"source_ref":"pipe:a","id":"pipe:c","label":"Copy","provenance":"user-copy"}]);
        }
        p
    }
    fn intent(model: &Value, payload: &Value, split: bool) -> Value {
        json!({"operation_kind":"modify","target":{"object_type":"Element","ref":"pipe:a"},"change":{"change_kind":if split{"split_pipe_run"}else{"transform_pipe_run"},"field_path":"pipe_segments","before":if split{canonical_json(&model["pipe_segments"][0])}else{canonical_json(&json!({"nodes":model["nodes"],"pipe_segments":model["pipe_segments"]}))},"after":payload.to_string(),"unit":"none","dimension":"dimensionless"}})
    }
    fn run(m: &Value, p: &Value, split: bool) -> GeometryEdit {
        resolve_geometry(m, &intent(m, p, split))
            .unwrap_or_else(|e| panic!("{}", e.message))
            .unwrap()
    }
    fn blocked(m: &Value, p: &Value, split: bool) {
        assert!(resolve_geometry(m, &intent(m, p, split)).is_err());
    }
    fn editor_intent(model: &Value, payload: &Value, split: bool) -> Value {
        let mut record = intent(model, payload, split);
        record["operation_id"] = json!("operation:geometry-integration");
        record["operation_status"] = json!("proposed");
        record["author_type"] = json!("user");
        record["change"]["change_id"] = json!("change:geometry-integration");
        record["change"]["field_label"] = json!("Geometry");
        record["change"]["source_note"] = json!("Explicit geometry integration fixture");
        record["rationale"] = json!("User-entered geometry operation");
        record["validation"] = json!({"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"});
        record["audit_boundary"] = json!({"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false});
        record["professional_boundary"] = json!({"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false});
        record
    }
    fn hash_claim(model: &Value) -> Value {
        json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",crate::sha256_hex(&canonical_json(model)))})
    }
    #[test]
    fn common_operation_route_geometry_preview_apply_and_hash_guards() {
        let model = model();
        let original = model.clone();
        let claim = hash_claim(&model);
        for (payload, is_split) in [
            (split_payload(), true),
            (transform_payload("translate", true), false),
        ] {
            let intent = editor_intent(&model, &payload, is_split);
            let preview = crate::validate_operation(&model, &intent, Some(&claim));
            let applied = crate::apply_operation(&model, &intent, Some(&claim));
            assert!(applied.applied_model.is_some(), "{:?}", applied.diagnostics);
            assert_eq!(
                applied.validation.application_status,
                "applied_to_session_model"
            );
            assert_eq!(
                serde_json::to_value(&preview.diff_preview).unwrap(),
                serde_json::to_value(&applied.diff_preview).unwrap()
            );
            assert_eq!(preview.diff_preview.len(), 3);
            assert!(preview.applied_model.is_none());
            assert_eq!(model, original);
            let mut stale = claim.clone();
            stale["value"] = json!(format!("sha256:{}", "0".repeat(64)));
            let mut malformed = claim.clone();
            malformed["algorithm"] = json!("md5");
            for invalid in [
                None,
                Some(Value::Null),
                Some(stale),
                Some(malformed),
                Some(json!({})),
            ] {
                for out in [
                    crate::validate_operation(&model, &intent, invalid.as_ref()),
                    crate::apply_operation(&model, &intent, invalid.as_ref()),
                ] {
                    assert!(out.applied_model.is_none());
                    assert!(
                        out.diff_preview.is_empty(),
                        "Invalid hash leaked geometry preview"
                    );
                }
            }
            let mut attached = model.clone();
            attached["components"] = json!([{"id":"component:attached","node":"node:a"}]);
            let current = hash_claim(&attached);
            for out in [
                crate::validate_operation(&attached, &intent, Some(&current)),
                crate::apply_operation(&attached, &intent, Some(&current)),
            ] {
                assert!(out.applied_model.is_none());
                assert!(out.diff_preview.is_empty());
            }
        }
    }
    #[test]
    fn split_conserves_lengths_preserves_endpoints_metadata_and_nodal_attachments() {
        let mut m = model();
        m["supports"] = json!([{"id":"support:a","node":"node:a"}]);
        m["load_cases"] = json!([{"primitive_loads":[{"target":{"node":"node:b"}}]}]);
        let saved = m.clone();
        let out = run(&m, &split_payload(), true);
        assert_eq!(out.model["nodes"][2]["position"]["x"], 1.);
        assert_eq!(out.model["pipe_segments"][0]["to"], "node:new");
        assert_eq!(out.model["pipe_segments"][1]["from"], "node:new");
        assert_eq!(out.model["pipe_segments"][1]["to"], "node:b");
        assert_eq!(
            out.model["pipe_segments"][1]["section"],
            m["pipe_segments"][0]["section"]
        );
        assert_eq!(out.model["supports"], m["supports"]);
        assert_eq!(out.model["load_cases"], m["load_cases"]);
        assert_eq!(out.diff_preview.len(), 3);
        assert_eq!(m, saved);
    }
    #[test]
    fn transforms_explicit_geometry_and_directions_without_mutating_source() {
        let m = model();
        let out = run(&m, &transform_payload("rotate", false), false);
        assert!(
            out.model["nodes"][1]["position"]["x"]
                .as_f64()
                .unwrap()
                .abs()
                < 1e-12
        );
        assert!((out.model["nodes"][1]["position"]["y"].as_f64().unwrap() - 4.).abs() < 1e-12);
        assert!(
            (out.model["pipe_segments"][0]["y_reference"]["x"]
                .as_f64()
                .unwrap()
                + 1.)
                .abs()
                < 1e-12
        );
        let out = run(&m, &transform_payload("mirror", false), false);
        assert_eq!(out.model["nodes"][0]["position"]["x"], 2.);
        assert_eq!(out.model["nodes"][1]["position"]["x"], -2.);
        let out = run(&m, &transform_payload("translate", true), false);
        assert_eq!(out.model["nodes"][0], m["nodes"][0]);
        assert_eq!(out.model["pipe_segments"][0], m["pipe_segments"][0]);
        assert_eq!(
            out.model["nodes"][2]["position"],
            json!({"x":1.,"y":2.,"z":0.})
        );
        assert_eq!(out.model["pipe_segments"][1]["from"], "node:c");
        assert_eq!(out.model["pipe_segments"][1]["to"], "node:d");
        assert_eq!(
            out.model["pipe_segments"][1]["y_reference"],
            m["pipe_segments"][0]["y_reference"]
        );
        assert_eq!(out.diff_preview.len(), 3);
    }
    #[test]
    fn reference_guard_matrix_blocks_attached_engineering_data() {
        let mut cases = Vec::new();
        let mut m = model();
        m["components"] = json!([{"id":"component:a","node":"node:a"}]);
        cases.push(m);
        for key in [
            "bend_pipe_ref",
            "branch_header_pipe_ref",
            "branch_branch_pipe_ref",
            "rigid_pipe_ref",
            "expansion_joint_pipe_ref",
        ] {
            let mut m = model();
            m["components"] = json!([{"geometry":{key:"pipe:a"}}]);
            cases.push(m);
        }
        for wind_key in ["equivalent_static", "equivalent_static_generation"] {
            let mut m = model();
            m["load_cases"] = json!([{wind_key:{"wind":{"exposed_pipe_refs":["pipe:a"]}}}]);
            cases.push(m);
            let mut m = model();
            m["load_cases"] = json!([{wind_key:{"wind":{"exposed_spans":[{"pipe_ref":"pipe:a","start_fraction":0.1,"end_fraction":0.5}]}}}]);
            cases.push(m);
        }
        let mut m = model();
        m["load_cases"] = json!([{"primitive_loads":[{"target":{"pipe":"pipe:a"}}]}]);
        cases.push(m);
        let mut m = model();
        m["extension"] = json!({"required_input_refs":[{"id":"pipe:a"}]});
        cases.push(m);
        for m in cases {
            blocked(&m, &split_payload(), true);
            blocked(&m, &transform_payload("translate", false), false);
            blocked(&m, &transform_payload("translate", true), false);
        }
        for key in ["supports", "load_cases"] {
            let mut m = model();
            m[key] = if key == "supports" {
                json!([{"node":"node:a"}])
            } else {
                json!([{"primitive_loads":[{"target":{"node":"node:a"}}]}])
            };
            blocked(&m, &transform_payload("translate", true), false);
            blocked(&m, &transform_payload("translate", false), false);
        }
    }
    #[test]
    fn split_rejects_rounding_overflow_zero_and_collisions() {
        for (a, b, f) in [
            (1e16, 1e16 + 2., 0.1),
            (-1e308, 1e308, 0.5),
            (0., 0., 0.5),
            (0., 1e-13, 0.5),
            (0., 4., 0.),
            (0., 4., 1.),
        ] {
            let mut m = model();
            m["nodes"][0]["position"]["x"] = json!(a);
            m["nodes"][1]["position"]["x"] = json!(b);
            let mut p = split_payload();
            p["fraction"] = json!(f);
            blocked(&m, &p, true);
        }
        let m = model();
        let mut p = split_payload();
        p["new_node"]["id"] = json!("material:a");
        blocked(&m, &p, true);
        let mut p = split_payload();
        p["new_node"]["id"] = p["new_pipe"]["id"].clone();
        blocked(&m, &p, true);
    }
    #[test]
    fn transform_requires_complete_maps_valid_frames_and_known_shapes() {
        let m = model();
        let mut p = transform_payload("translate", true);
        p["copy_nodes"].as_array_mut().unwrap().pop();
        blocked(&m, &p, false);
        let mut p = transform_payload("translate", true);
        p["copy_pipes"][0]["id"] = json!("node:c");
        blocked(&m, &p, false);
        let mut p = transform_payload("rotate", false);
        p["transform"]["axis"] = json!({"x":0.,"y":0.,"z":0.});
        blocked(&m, &p, false);
        let mut p = transform_payload("rotate", false);
        p["transform"]["angle"]["unit"] = json!("Pa");
        blocked(&m, &p, false);
        let mut absent = m.clone();
        absent["pipe_segments"][0]
            .as_object_mut()
            .unwrap()
            .remove("y_reference");
        blocked(&absent, &transform_payload("rotate", false), false);
        blocked(&absent, &transform_payload("mirror", true), false);
        let out = run(&absent, &transform_payload("translate", false), false);
        assert!(out.model["pipe_segments"][0].get("y_reference").is_none());
        let mut bad = m.clone();
        bad["pipe_segments"][0]["y_reference"] = json!({"x":1.,"y":0.,"z":0.});
        blocked(&bad, &transform_payload("rotate", false), false);
        for key in ["orientation_extension", "external_ref"] {
            let mut bad = m.clone();
            bad["nodes"][0][key] = json!("unsupported");
            blocked(&bad, &split_payload(), true);
            blocked(&bad, &transform_payload("translate", false), false);
        }
        let mut bad = m.clone();
        bad["schema_version"] = json!("1.0.0");
        blocked(&bad, &split_payload(), true);
        let mut bad = m.clone();
        bad["document_kind"] = json!("canonical");
        blocked(&bad, &transform_payload("translate", false), false);
        let mut i = intent(&m, &split_payload(), true);
        i["change"]["before"] = json!("{}");
        assert!(resolve_geometry(&m, &i).is_err());
    }
    #[test]
    fn transform_rejects_incident_unselected_pipe_and_realized_overflow() {
        let mut m = model();
        let mut second = m["pipe_segments"][0].clone();
        second["id"] = json!("pipe:other");
        m["pipe_segments"].as_array_mut().unwrap().push(second);
        let p = transform_payload("translate", false);
        let mut i = intent(&m, &p, false);
        i["change"]["before"] = json!(canonical_json(
            &json!({"nodes":m["nodes"],"pipe_segments":[m["pipe_segments"][0]]})
        ));
        assert!(resolve_geometry(&m, &i).is_err());
        let m = model();
        let mut p = transform_payload("translate", false);
        p["transform"]["translation"]["x"] = json!(1e308);
        p["transform"]["translation"]["unit"] = json!("m");
        blocked(&m, &p, false);
        let mut m = model();
        m["pipe_segments"][0]["y_reference"] = json!({"x":0.,"y":1e308,"z":1e308});
        blocked(&m, &transform_payload("rotate", false), false);
    }
}

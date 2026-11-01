//! Bend authoring for `3.0.0/exact_pressure_v3` (T4-U2; T4-I11 D-A..D-K):
//! realizing a bend (`Component` / `bend_realization`), inserting a realized
//! bend at a corner (`insert_bend_at_corner`) and the pressure-region checks
//! on bends. The arc, the direction change `kink` and the tangency tolerance
//! α_tan are CB's, reached through PP, so authoring and solve share one
//! definition. The flexibility factor k is user data: no value is suggested,
//! defaulted or accepted from an agent (the caller refuses agent authors).
//! T4-U4b (§2.6): the realization also takes the optional directional SIF pair
//! (`sif_in_plane_user_value`, `sif_out_of_plane_user_value`), both or neither,
//! under the same `source_reference`; it is user data as k is.
use crate::geometry_operations::{diff, global_ids, metadata, references, GeometryEdit, GeometryError};
use crate::rich_authoring::{RichEdit, RichError};
use crate::{canonical_json, quantity_value_in_unit, Dimension, EnteredQuantity};
use open_pipe_stress_product_physics::{arc_geometry, kink, ArcFrame, TANGENCY_TOLERANCE_RAD};
use serde_json::{json, Map, Value};
use std::collections::{BTreeSet, HashMap};

/// The solver consumption of a realized bend (PP's DEC-070 token).
pub(crate) const CURVED: &str = "curved_bend_macro_element";
const GEOMETRY_ONLY: &str = "mechanics_geometry_only";
/// PP's rule for an authored bend angle against the arc's included angle
/// (`DEC_070_CURVED_BEND_ANGLE_MATCH_TOLERANCE`, relative to max(|φ|, 1)).
const ANGLE_MATCH: f64 = 1.0e-6;
const REALIZATION_KEYS: &[&str] = &[
    "solver_consumption",
    "flexibility_factor_user_value",
    "source_reference",
    SIF_IN_PLANE,
    SIF_OUT_OF_PLANE,
];
/// The directional SIF pair (T4-U4b §2.6): i_ip multiplies the arc frame's M_z,
/// i_op its M_y.
const SIF_IN_PLANE: &str = "sif_in_plane_user_value";
const SIF_OUT_OF_PLANE: &str = "sif_out_of_plane_user_value";
type Vec3 = [f64; 3];

fn is_bend(component: &Value) -> bool {
    matches!(component.get("kind").and_then(Value::as_str), Some("bend" | "elbow"))
}
fn consumption(component: &Value) -> &str {
    component
        .pointer("/mechanics_interface/solver_consumption")
        .and_then(Value::as_str)
        .unwrap_or(GEOMETRY_ONLY)
}
fn is_realized(component: &Value) -> bool {
    is_bend(component) && consumption(component) == CURVED
}
fn nonblank(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_str).filter(|text| !text.trim().is_empty())
}
fn span(component: &Value) -> Option<&str> {
    nonblank(component.pointer("/geometry/bend_pipe_ref"))
}
fn rows<'a>(model: &'a Value, key: &str) -> impl Iterator<Item = &'a Value> {
    model.get(key).and_then(Value::as_array).into_iter().flatten()
}
fn find<'a>(model: &'a Value, key: &str, id: &str) -> Option<&'a Value> {
    let mut found = rows(model, key).filter(|row| row.get("id").and_then(Value::as_str) == Some(id));
    let first = found.next()?;
    found.next().is_none().then_some(first)
}
fn ends(pipe: &Value) -> Option<(&str, &str)> {
    Some((nonblank(pipe.get("from"))?, nonblank(pipe.get("to"))?))
}
fn vector(value: Option<&Value>) -> Option<Vec3> {
    let value = value?;
    let component = |key: &str| value.get(key).and_then(Value::as_f64).filter(|n| n.is_finite());
    Some([component("x")?, component("y")?, component("z")?])
}
/// A node position in the project's length unit.
fn position(model: &Value, node: &str) -> Option<Vec3> {
    vector(find(model, "nodes", node)?.get("position"))
}
fn length_unit(model: &Value) -> Option<&str> {
    nonblank(model.pointer("/project/units/length"))
}
/// A `{value, unit}` quantity in `target` units of `dimension`.
fn quantity_in(value: Option<&Value>, target: &str, dimension: Dimension) -> Option<f64> {
    let value = value?;
    let entered = EnteredQuantity {
        value: value.get("value").and_then(Value::as_f64).filter(|n| n.is_finite())?,
        unit: nonblank(value.get("unit"))?.to_string(),
    };
    quantity_value_in_unit(&entered, target, dimension).filter(|n| n.is_finite())
}
fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: Vec3, b: Vec3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn norm(a: Vec3) -> f64 {
    dot(a, a).sqrt()
}
fn unit(a: Vec3) -> Option<Vec3> {
    let length = norm(a);
    (length.is_finite() && length > 0.0).then(|| a.map(|v| v / length))
}
fn negate(a: Vec3) -> Vec3 {
    a.map(|v| -v)
}

/// The arc of bend `component` over its span, from (chord, R, y_reference)
/// in the project's length unit: (span id, from node, to node, arc).
fn arc_of(model: &Value, component: &Value) -> Result<(String, String, String, ArcFrame), String> {
    let span_id = span(component).ok_or("the bend names no span (geometry.bend_pipe_ref)")?;
    let pipe = find(model, "pipe_segments", span_id).ok_or_else(|| format!("span {span_id} is not a pipe of the model"))?;
    let (from, to) = ends(pipe).ok_or_else(|| format!("span {span_id} has no end nodes"))?;
    let (a, b) = (
        position(model, from).ok_or_else(|| format!("node {from} has no finite position"))?,
        position(model, to).ok_or_else(|| format!("node {to} has no finite position"))?,
    );
    let length = length_unit(model).ok_or("the project has no length unit")?;
    let radius = quantity_in(component.pointer("/geometry/bend_radius"), length, Dimension::Length)
        .filter(|r| *r > 0.0)
        .ok_or("the bend has no finite positive radius (geometry.bend_radius)")?;
    let y = vector(pipe.get("y_reference")).ok_or_else(|| format!("span {span_id} has no y_reference, which fixes the arc's plane and side"))?;
    let frame = arc_geometry(sub(b, a), radius, y).map_err(|error| error.to_string())?;
    Ok((span_id.to_string(), from.to_string(), to.to_string(), frame))
}

/// The arcs of every realized bend whose arc forms, by span.
fn realized_arcs(model: &Value) -> HashMap<String, ArcFrame> {
    rows(model, "components")
        .filter(|component| is_realized(component))
        .filter_map(|component| arc_of(model, component).ok())
        .map(|(span, _, _, frame)| (span, frame))
        .collect()
}

/// The direction leaving `node` into member `pipe`: a straight member's
/// chord, or a realized arc's end tangent.
fn outward(model: &Value, pipe: &Value, node: &str, arcs: &HashMap<String, ArcFrame>) -> Option<Vec3> {
    let (from, to) = ends(pipe)?;
    let id = pipe.get("id").and_then(Value::as_str)?;
    if let Some(frame) = arcs.get(id) {
        return if from == node {
            Some(frame.tangent_i)
        } else if to == node {
            Some(negate(frame.tangent_j))
        } else {
            None
        };
    }
    let other = if from == node { to } else if to == node { from } else { return None };
    unit(sub(position(model, other)?, position(model, node)?))
}

/// θ at `node` between `arriving` and `leaving` (0 for a tangent junction).
fn junction(model: &Value, arriving: &Value, leaving: &Value, node: &str, arcs: &HashMap<String, ArcFrame>) -> Option<f64> {
    let t_in = negate(outward(model, arriving, node, arcs)?);
    let t_out = outward(model, leaving, node, arcs)?;
    kink(t_in, t_out).ok()
}

/// W-10 reconnect (DEL-16-06 Design `pipe-reconnect.md` §2.4): θ at `node`
/// between pipe `pipe_id` and each realized arc other than it that ends at
/// `node`, in `model` (the reconnected candidate), by arc span id.
pub(crate) fn realized_arc_junctions(model: &Value, pipe_id: &str, node: &str) -> Vec<(String, f64)> {
    let arcs = realized_arcs(model);
    let Some(pipe) = find(model, "pipe_segments", pipe_id) else {
        return Vec::new();
    };
    let mut spans: Vec<&String> = arcs.keys().filter(|span| span.as_str() != pipe_id).collect();
    spans.sort();
    spans
        .into_iter()
        .filter_map(|span| {
            let arc = find(model, "pipe_segments", span)?;
            let (from, to) = ends(arc)?;
            if from != node && to != node {
                return None;
            }
            junction(model, arc, pipe, node, &arcs).map(|theta| (span.clone(), theta))
        })
        .collect()
}

fn mitre_message(node: &str, a: &str, b: &str, theta: f64) -> String {
    format!(
        "members {a} and {b} meet at {node} with a direction change of {theta:e} rad, above the tangency tolerance alpha_tan = {TANGENCY_TOLERANCE_RAD:e} rad at a realized bend's end; a kink this large is a mitre, which 3.0.0/exact_pressure_v3 does not model. Make the bend tangent (Insert bend at corner places tangent points exactly) or end the region at {node}"
    )
}

// ------------------------------------------------------------ pressure regions

/// O4: a pressure region's members (one load case's region). A geometry-only
/// bend's span is refused ("realize this bend"); at each end of a realized
/// arc member, a neighbouring member must meet it within α_tan.
pub(crate) fn check_region_members(model: &Value, members: &[&str]) -> Result<(), RichError> {
    for component in rows(model, "components").filter(|c| is_bend(c) && !is_realized(c)) {
        if let Some(span) = span(component).filter(|span| members.contains(span)) {
            return Err(RichError {
                code: "OP-PRESSURE-REGION-BEND-NOT-REALIZED",
                message: format!(
                    "pipe {span} is the span of geometry-only bend {} (a straight chord with no flexibility); pressure passes only through realized bends: realize this bend and enter its flexibility factor k",
                    component.get("id").and_then(Value::as_str).unwrap_or("(no id)")
                ),
            });
        }
    }
    let arcs = realized_arcs(model);
    for component in rows(model, "components").filter(|c| is_realized(c)) {
        let Some(span_id) = span(component).filter(|span| members.contains(span)) else { continue };
        let Some(frame_pipe) = find(model, "pipe_segments", span_id) else { continue };
        if !arcs.contains_key(span_id) {
            let reason = arc_of(model, component).err().unwrap_or_default();
            return Err(RichError {
                code: "OP-PRESSURE-REGION-BEND-GEOMETRY-INVALID",
                message: format!("realized bend {} cannot form its arc: {reason}", component.get("id").and_then(Value::as_str).unwrap_or("(no id)")),
            });
        }
        let Some((from, to)) = ends(frame_pipe) else { continue };
        for node in [from, to] {
            for neighbour in rows(model, "pipe_segments").filter(|pipe| {
                let id = pipe.get("id").and_then(Value::as_str).unwrap_or("");
                id != span_id && members.contains(&id) && ends(pipe).is_some_and(|(a, b)| a == node || b == node)
            }) {
                let neighbour_id = neighbour.get("id").and_then(Value::as_str).unwrap_or("");
                let theta = junction(model, neighbour, frame_pipe, node, &arcs).ok_or_else(|| RichError {
                    code: "OP-PRESSURE-REGION-BEND-GEOMETRY-INVALID",
                    message: format!("the direction change at {node} between {neighbour_id} and {span_id} is not defined"),
                })?;
                if !(theta <= TANGENCY_TOLERANCE_RAD) {
                    return Err(RichError {
                        code: "OP-PRESSURE-REGION-MITRE-UNSUPPORTED",
                        message: mitre_message(node, neighbour_id, span_id, theta),
                    });
                }
            }
        }
    }
    Ok(())
}

// --------------------------------------------------------- O1: realize a bend

pub(crate) fn owns(object_type: &str, path: &str) -> bool {
    object_type == "Component" && path == "bend_realization"
}

fn realize_err(code: &'static str, message: impl Into<String>) -> RichError {
    RichError { code, message: message.into() }
}

/// The canonical projection O1 replaces, or null when the bend has none.
fn realization_projection(component: &Value) -> Value {
    let mut out = Map::new();
    if let Some(value) = component.pointer("/mechanics_interface/solver_consumption") {
        out.insert("solver_consumption".into(), value.clone());
    }
    for key in ["flexibility_factor_user_value", "source_reference", SIF_IN_PLANE, SIF_OUT_OF_PLANE] {
        if let Some(value) = component.get("modifiers").and_then(|m| m.get(key)) {
            out.insert(key.into(), value.clone());
        }
    }
    if out.is_empty() { Value::Null } else { Value::Object(out) }
}

/// A user-entered k: `{value, unit: "none"}`, finite and positive (PP's rule).
fn flexibility(value: Option<&Value>) -> Result<Value, RichError> {
    let refuse = || realize_err(
        "OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED",
        "Enter the bend's flexibility factor k as {value, unit: none}, finite and positive (k = 1 means no extra flexibility beyond the curved beam); no value is suggested or defaulted",
    );
    let value = value.and_then(Value::as_object).ok_or_else(refuse)?;
    if value.len() != 2 || value.get("unit").and_then(Value::as_str) != Some("none") {
        return Err(refuse());
    }
    let k = value.get("value").and_then(Value::as_f64).filter(|k| k.is_finite() && *k > 0.0).ok_or_else(refuse)?;
    Ok(json!({"value": k, "unit": "none"}))
}

/// The optional directional SIF pair: both or neither, each a user-entered
/// `{value, unit}` with unit `1` (or `none`, which PP reads as `1`), finite and
/// positive. No code floor is imposed (F-PIP-1).
fn directional_pair(fields: &Map<String, Value>) -> Result<Option<(Value, Value)>, RichError> {
    let refuse = || realize_err(
        "OP-BEND-DIRECTIONAL-SIF-INCOMPLETE",
        "Enter both directional SIFs (in-plane i_ip on M_z, out-of-plane i_op on M_y) as {value, unit: 1}, finite and positive, with their source, or neither; no value is suggested or defaulted",
    );
    let factor = |value: &Value| -> Result<Value, RichError> {
        let value = value.as_object().ok_or_else(refuse)?;
        let unit = value.get("unit").and_then(Value::as_str).filter(|u| matches!(*u, "1" | "none")).ok_or_else(refuse)?;
        let i = value.get("value").and_then(Value::as_f64).filter(|i| i.is_finite() && *i > 0.0).ok_or_else(refuse)?;
        if value.len() != 2 {
            return Err(refuse());
        }
        Ok(json!({"value": i, "unit": unit}))
    };
    match (fields.get(SIF_IN_PLANE), fields.get(SIF_OUT_OF_PLANE)) {
        (None, None) => Ok(None),
        (Some(ip), Some(op)) => Ok(Some((factor(ip)?, factor(op)?))),
        _ => Err(refuse()),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn resolve(
    model: &Value,
    object_type: &str,
    target: &str,
    path: &str,
    before: &str,
    after: &str,
    unit_token: &str,
    dimension: &str,
) -> Result<Option<RichEdit>, RichError> {
    if !owns(object_type, path) {
        return Ok(None);
    }
    if unit_token != "none" || dimension != "dimensionless" {
        return Err(realize_err("OP-PRESSURE-PAYLOAD-INVALID", "Bend realization requires unit none and dimension dimensionless"));
    }
    let component = find(model, "components", target)
        .ok_or_else(|| realize_err("OP-BEND-REALIZE-TARGET-INVALID", format!("Unknown or ambiguous component {target}")))?;
    if !is_bend(component) {
        return Err(realize_err("OP-BEND-REALIZE-TARGET-INVALID", format!("Component {target} is not a bend or elbow; only bends are realized")));
    }
    let current = realization_projection(component);
    let current_display = if current.is_null() { "not_present".to_string() } else { canonical_json(&current) };
    if before != current_display {
        return Err(realize_err("OP-BEFORE-VALUE-MISMATCH", "Explicit input before-value is stale; refresh the draft"));
    }
    let after: Value = serde_json::from_str(after).map_err(|e| realize_err("OP-BEND-REALIZE-PAYLOAD-INVALID", e.to_string()))?;
    let fields = after.as_object().ok_or_else(|| realize_err("OP-BEND-REALIZE-PAYLOAD-INVALID", "Expected an object"))?;
    if let Some(key) = fields.keys().find(|k| !REALIZATION_KEYS.contains(&k.as_str())) {
        return Err(realize_err("OP-BEND-REALIZE-PAYLOAD-INVALID", format!("Unknown field {key}")));
    }
    let consumption_path = vec!["mechanics_interface".to_string(), "solver_consumption".to_string()];
    let (writes, warnings) = match fields.get("solver_consumption").and_then(Value::as_str) {
        Some(GEOMETRY_ONLY) => {
            // Back to a straight chord: k and its source stay user data.
            if fields.len() != 1 {
                return Err(realize_err("OP-BEND-REALIZE-PAYLOAD-INVALID", "Returning a bend to geometry-only changes solver_consumption only"));
            }
            // A directional pair is read in the arc's own frame; a chord has none
            // and the solve refuses it (BEND_DIRECTIONAL_SIF_NOT_REALIZED_ARC).
            if !current.get(SIF_IN_PLANE).unwrap_or(&Value::Null).is_null() || !current.get(SIF_OUT_OF_PLANE).unwrap_or(&Value::Null).is_null() {
                return Err(realize_err(
                    "OP-BEND-DIRECTIONAL-SIF-NOT-REALIZED-ARC",
                    "This bend carries directional SIFs, which apply only to a realized arc; realize it again without them before returning it to geometry-only",
                ));
            }
            (vec![(consumption_path, Some(json!(GEOMETRY_ONLY)))], vec![])
        }
        Some(CURVED) => {
            let k = flexibility(fields.get("flexibility_factor_user_value"))?;
            let source = nonblank(fields.get("source_reference")).ok_or_else(|| realize_err(
                "OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED",
                "Enter the source of the flexibility factor k (source_reference)",
            ))?;
            let pair = directional_pair(fields)?;
            let warnings = realization_geometry(model, component)?;
            let modifier = |key: &str| vec!["modifiers".to_string(), key.to_string()];
            let (ip, op) = match pair {
                Some((ip, op)) => (Some(ip), Some(op)),
                None => (None, None),
            };
            let mut writes = vec![
                (consumption_path, Some(json!(CURVED))),
                (modifier("flexibility_factor_user_value"), Some(k)),
                (modifier("source_reference"), Some(json!(source))),
            ];
            // The payload is the realization's whole projection: a pair it omits
            // is removed.
            for (key, value) in [(SIF_IN_PLANE, ip), (SIF_OUT_OF_PLANE, op)] {
                if value.is_some() || !current.get(key).unwrap_or(&Value::Null).is_null() {
                    writes.push((modifier(key), value));
                }
            }
            (writes, warnings)
        }
        _ => return Err(realize_err("OP-BEND-REALIZE-PAYLOAD-INVALID", format!("solver_consumption must be {CURVED} or {GEOMETRY_ONLY}"))),
    };
    Ok(Some(edit(&after, current_display, writes, warnings)))
}

fn edit(after: &Value, current_display: String, writes: Vec<(Vec<String>, Option<Value>)>, warnings: Vec<String>) -> RichEdit {
    RichEdit {
        writes: if canonical_json(after) == current_display { vec![] } else { writes },
        current_display,
        warnings,
    }
}

/// The realization's geometry: a span arc that forms, an authored angle that
/// agrees with it, and tangency at each end. Returns the warnings (an end
/// with several neighbours); refuses everything else.
fn realization_geometry(model: &Value, component: &Value) -> Result<Vec<String>, RichError> {
    let id = component.get("id").and_then(Value::as_str).unwrap_or("(no id)");
    let node = nonblank(component.get("node")).unwrap_or("");
    let span_id = span(component).ok_or_else(|| realize_err(
        "OP-BEND-REALIZE-SPAN-MISSING",
        format!("Bend {id} names no span: a bend without an arc span marks a corner; use Insert bend at corner"),
    ))?;
    let pipe = find(model, "pipe_segments", span_id)
        .ok_or_else(|| realize_err("OP-BEND-REALIZE-SPAN-MISSING", format!("Bend {id}'s span {span_id} is not a pipe of the model")))?;
    let (from, to) = ends(pipe).ok_or_else(|| realize_err("OP-BEND-REALIZE-SPAN-MISSING", format!("Span {span_id} has no end nodes")))?;
    if node != from && node != to {
        return Err(realize_err("OP-BEND-REALIZE-SPAN-MISSING", format!("Bend {id}'s node {node} is not an end of its span {span_id}")));
    }
    let length = length_unit(model).unwrap_or("");
    let radius = quantity_in(component.pointer("/geometry/bend_radius"), length, Dimension::Length).filter(|r| *r > 0.0);
    let chord = position(model, from).zip(position(model, to)).map(|(a, b)| norm(sub(b, a)));
    if let (Some(radius), Some(chord)) = (radius, chord) {
        if radius <= 0.5 * chord {
            return Err(realize_err(
                "OP-BEND-REALIZE-CORNER-MARKER",
                format!("Bend {id}'s radius cannot span its {chord} {length} chord {span_id}: this bend marks a corner, not an arc span; use Insert bend at corner, which places the tangent points"),
            ));
        }
    }
    let (_, _, _, frame) = arc_of(model, component)
        .map_err(|reason| realize_err("OP-BEND-GEOMETRY-INCONSISTENT", format!("Bend {id}'s arc cannot be formed: {reason}")))?;
    if let Some(angle) = quantity_in(component.pointer("/geometry/bend_angle"), "rad", Dimension::Angle) {
        let scale = angle.abs().max(frame.included_angle.abs()).max(1.0);
        if (angle - frame.included_angle).abs() > ANGLE_MATCH * scale {
            return Err(realize_err(
                "OP-BEND-GEOMETRY-INCONSISTENT",
                format!("Bend {id}'s authored angle {angle} rad disagrees with the included angle {} rad of its chord and radius", frame.included_angle),
            ));
        }
    }
    let mut arcs = realized_arcs(model);
    arcs.insert(span_id.to_string(), frame);
    let mut warnings = Vec::new();
    for end in [from, to] {
        let neighbours: Vec<&Value> = rows(model, "pipe_segments")
            .filter(|p| p.get("id").and_then(Value::as_str) != Some(span_id) && ends(p).is_some_and(|(a, b)| a == end || b == end))
            .collect();
        for neighbour in &neighbours {
            let neighbour_id = neighbour.get("id").and_then(Value::as_str).unwrap_or("");
            let Some(theta) = junction(model, neighbour, pipe, end, &arcs) else { continue };
            if theta <= TANGENCY_TOLERANCE_RAD {
                continue;
            }
            if neighbours.len() == 1 {
                return Err(realize_err("OP-BEND-TANGENCY-EXCEEDED", mitre_message(end, neighbour_id, span_id, theta)));
            }
            warnings.push(format!("Not tangent at a junction of several members: {}", mitre_message(end, neighbour_id, span_id, theta)));
        }
    }
    Ok(warnings)
}

// ------------------------------------------------ O2: insert a bend at a corner

fn corner_err(code: &'static str, message: impl Into<String>, refs: Vec<String>) -> GeometryError {
    GeometryError { code, message: message.into(), affected_refs: refs }
}
fn invalid(message: impl Into<String>) -> GeometryError {
    corner_err("OP-BEND-CORNER-PAYLOAD-INVALID", message, vec![])
}
fn exact_fields(value: &Value, required: &[&str], optional: &[&str]) -> Result<(), GeometryError> {
    let map = value.as_object().ok_or_else(|| invalid("Expected an explicit object"))?;
    if required.iter().any(|k| !map.contains_key(*k)) || map.keys().any(|k| !required.contains(&k.as_str()) && !optional.contains(&k.as_str())) {
        return Err(invalid(format!("Required fields {required:?}; optional {optional:?}; unknown fields are unsupported")));
    }
    Ok(())
}
fn xyz(v: Vec3) -> Result<Value, GeometryError> {
    if !v.iter().all(|n| n.is_finite()) {
        return Err(invalid("Realized coordinates or direction overflow"));
    }
    Ok(json!({"x": v[0], "y": v[1], "z": v[2]}))
}

/// The canonical corner context `before` names: the corner node, its two
/// incident pipes (model order) and the bend marker at it, if any.
pub(crate) fn corner_context(model: &Value, node: &str) -> Value {
    let pipes: Vec<&Value> = rows(model, "pipe_segments").filter(|p| ends(p).is_some_and(|(a, b)| a == node || b == node)).collect();
    let marker: Vec<&Value> = rows(model, "components").filter(|c| is_bend(c) && c.get("node").and_then(Value::as_str) == Some(node)).collect();
    json!({"node": find(model, "nodes", node), "pipes": pipes, "markers": marker})
}

/// O2 (T4-I11 D-A): insert a realized bend of radius R at corner node C
/// between its two straights. The tangent points are
/// T1 = C − L_t·u_in and T2 = C + L_t·u_out with L_t = R·tan(Φ/2), so both
/// ends are tangent by construction; the arc pipe T1→T2 copies the shared
/// section and material and takes y_reference = u_in (the corner-side bow).
pub(crate) fn insert_at_corner(model: &Value, intent: &Value) -> Result<GeometryEdit, GeometryError> {
    if model.get("document_kind").and_then(Value::as_str) != Some("openpipestress.product_preview.model")
        || model.get("schema_version").and_then(Value::as_str) != Some(open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION)
    {
        return Err(invalid("Insert bend at corner requires a recognized product_preview.model document"));
    }
    if intent["operation_kind"] != "modify"
        || intent["target"]["object_type"] != "Node"
        || intent["change"]["field_path"] != "bend_at_corner"
        || intent["change"]["unit"] != "none"
        || intent["change"]["dimension"] != "dimensionless"
    {
        return Err(invalid("Insert bend at corner requires modify/Node/bend_at_corner/none/dimensionless"));
    }
    let corner = nonblank(intent.pointer("/target/ref")).ok_or_else(|| invalid("Target node is required"))?;
    let corner_node = find(model, "nodes", corner)
        .ok_or_else(|| corner_err("OP-BEND-CORNER-NODE-INVALID", format!("Node {corner} is not in the model"), vec![corner.into()]))?;
    if intent.pointer("/change/before").and_then(Value::as_str) != Some(canonical_json(&corner_context(model, corner)).as_str()) {
        return Err(corner_err("OP-BEFORE-VALUE-MISMATCH", "Stale corner context; refresh the draft", vec![corner.into()]));
    }
    let payload: Value = serde_json::from_str(intent.pointer("/change/after").and_then(Value::as_str).unwrap_or(""))
        .map_err(|_| invalid("after must be JSON"))?;
    exact_fields(&payload, &["radius", "flexibility_factor", "flexibility_source_reference", "new_nodes", "new_pipe", "component"], &[])?;
    let length = length_unit(model).ok_or_else(|| invalid("The project has no length unit"))?;

    let pipes: Vec<&Value> = rows(model, "pipe_segments").filter(|p| ends(p).is_some_and(|(a, b)| a == corner || b == corner)).collect();
    if pipes.len() != 2 {
        return Err(corner_err("OP-BEND-CORNER-NODE-INVALID", format!("Corner {corner} must join exactly two pipes; it joins {}", pipes.len()), vec![corner.into()]));
    }
    let (p_in, p_out) = (pipes[0], pipes[1]);
    let id_of = |p: &Value| p.get("id").and_then(Value::as_str).unwrap_or("").to_string();
    let (in_id, out_id) = (id_of(p_in), id_of(p_out));
    for component in rows(model, "components").filter(|c| is_bend(c)) {
        if span(component).is_some_and(|s| s == in_id || s == out_id) && component.get("node").and_then(Value::as_str) != Some(corner) {
            return Err(corner_err("OP-BEND-CORNER-NODE-INVALID", format!("Pipe {} is already a bend span", span(component).unwrap()), vec![corner.into()]));
        }
    }
    let same = |key: &str| p_in.get(key) == p_out.get(key);
    if !same("section") || !same("material") || !same("section_ref") {
        return Err(corner_err("OP-BEND-CORNER-SECTION-MISMATCH", format!("Pipes {in_id} and {out_id} differ in section or material; a bend joins one section and material"), vec![in_id.clone(), out_id.clone()]));
    }
    // Model document 0.9.0 (§1.4): both straights carry the same condition set or none,
    // and the arc (a copy of the incoming straight) carries it.
    if !same("condition_set_ref") {
        return Err(corner_err("OP-BEND-CONDITION-SET-MISMATCH", format!("Pipes {in_id} and {out_id} carry different condition sets; a bend joins straights that carry the same condition set or none"), vec![in_id.clone(), out_id.clone()]));
    }
    fn far<'a>(p: &'a Value, corner: &str) -> Option<&'a str> {
        ends(p).map(|(a, b)| if a == corner { b } else { a })
    }
    let c = vector(corner_node.get("position")).ok_or_else(|| invalid(format!("Node {corner} has no finite position")))?;
    let x_in = far(p_in, corner).and_then(|n| position(model, n)).ok_or_else(|| invalid(format!("Pipe {in_id}'s far node has no finite position")))?;
    let x_out = far(p_out, corner).and_then(|n| position(model, n)).ok_or_else(|| invalid(format!("Pipe {out_id}'s far node has no finite position")))?;
    let (len_in, len_out) = (norm(sub(c, x_in)), norm(sub(x_out, c)));
    let (u_in, u_out) = (
        unit(sub(c, x_in)).ok_or_else(|| invalid(format!("Pipe {in_id} has zero length")))?,
        unit(sub(x_out, c)).ok_or_else(|| invalid(format!("Pipe {out_id} has zero length")))?,
    );
    let phi = kink(u_in, u_out).map_err(|e| invalid(e.to_string()))?;
    if phi <= TANGENCY_TOLERANCE_RAD || phi >= std::f64::consts::PI - 1.0e-9 {
        return Err(corner_err("OP-BEND-CORNER-ANGLE-INVALID", format!("The direction change at {corner} is {phi:e} rad: a bend needs a turn above alpha_tan = {TANGENCY_TOLERANCE_RAD:e} rad and below pi"), vec![corner.into()]));
    }
    let radius_value = &payload["radius"];
    exact_fields(radius_value, &["value", "unit"], &[])?;
    let radius = quantity_in(Some(radius_value), length, Dimension::Length)
        .filter(|r| *r > 0.0)
        .ok_or_else(|| corner_err("OP-BEND-CORNER-RADIUS-INVALID", "Bend radius must be a finite positive length in an accepted unit", vec![corner.into()]))?;
    let tan_half = norm(cross(u_in, u_out)) / (1.0 + dot(u_in, u_out));
    let tangent_length = radius * tan_half;
    if !(tangent_length.is_finite() && tangent_length < len_in && tangent_length < len_out) {
        return Err(corner_err(
            "OP-BEND-CORNER-RADIUS-INVALID",
            format!("Radius {radius} {length} needs {tangent_length} {length} of each straight; the largest admissible radius is below {} {length}", len_in.min(len_out) / tan_half),
            vec![corner.into(), in_id.clone(), out_id.clone()],
        ));
    }
    let k = flexibility(Some(&payload["flexibility_factor"])).map_err(|e| corner_err(e.code, e.message, vec![corner.into()]))?;
    let k_source = nonblank(payload.get("flexibility_source_reference"))
        .ok_or_else(|| corner_err("OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED", "Enter the source of the flexibility factor k", vec![corner.into()]))?;

    // New identities (no collision with any existing id).
    let mut ids = BTreeSet::new();
    global_ids(model, &mut ids);
    let new_nodes = payload["new_nodes"].as_array().filter(|n| n.len() == 2).ok_or_else(|| invalid("new_nodes must name the two tangent points"))?;
    for item in new_nodes {
        metadata(item, &mut ids, false)?;
    }
    metadata(&payload["new_pipe"], &mut ids, false)?;
    let markers: Vec<&Value> = rows(model, "components").filter(|c| is_bend(c) && c.get("node").and_then(Value::as_str) == Some(corner)).collect();
    let spec = &payload["component"];
    let converted = match spec.get("convert") {
        Some(target) => {
            exact_fields(spec, &["convert"], &["geometry_source_reference"])?;
            let target = target.as_str().unwrap_or("");
            let marker = markers.iter().find(|m| m.get("id").and_then(Value::as_str) == Some(target)).copied().ok_or_else(|| corner_err(
                "OP-BEND-CORNER-MARKER-INVALID",
                format!("{target} is not a bend marker at {corner}"),
                vec![corner.into(), target.into()],
            ))?;
            if let Some(angle) = quantity_in(marker.pointer("/geometry/bend_angle"), "rad", Dimension::Angle) {
                if (angle - phi).abs() > ANGLE_MATCH * angle.abs().max(phi).max(1.0) {
                    return Err(corner_err("OP-BEND-CORNER-ANGLE-CONFLICT", format!("Marker {target}'s angle {angle} rad disagrees with the corner's direction change {phi} rad"), vec![target.into()]));
                }
            }
            Some(marker)
        }
        None => {
            exact_fields(spec, &["id", "label", "provenance", "geometry_source_reference"], &[])?;
            metadata(&json!({"id": spec["id"], "label": spec["label"], "provenance": spec["provenance"]}), &mut ids, false)?;
            if let Some(marker) = markers.first() {
                return Err(corner_err(
                    "OP-BEND-CORNER-MARKER-PRESENT",
                    format!("Bend marker {} is at {corner}: convert it in place (component.convert) instead of adding a second bend", marker.get("id").and_then(Value::as_str).unwrap_or("")),
                    vec![corner.into()],
                ));
            }
            None
        }
    };
    // Required for a new bend; optional when a marker keeps its own.
    let source_reference = match spec.get("geometry_source_reference") {
        Some(value) => Some(nonblank(Some(value)).ok_or_else(|| invalid("geometry_source_reference must be nonblank text"))?.to_string()),
        None => None,
    };

    // No attachment is repartitioned: any other reference to C or to either pipe refuses.
    let selected: BTreeSet<String> = [corner.to_string(), in_id.clone(), out_id.clone()].into();
    let converted_id = converted.and_then(|m| m.get("id")).and_then(Value::as_str);
    let mut found = vec![];
    for (key, collection) in model.as_object().into_iter().flatten() {
        match key.as_str() {
            // W-13 §6: model-data records are not attachments; the arc joins
            // p_in's line below, and a corner named by an equipment interface
            // leaves the validator's missing-subject warning.
            slot if crate::model_data_authoring::MODEL_DATA_COLLECTIONS.contains(&slot) => {}
            "nodes" => {
                for node in rows(model, "nodes").filter(|n| n.get("id").and_then(Value::as_str) != Some(corner)) {
                    references(node, &selected, "nodes", &mut found);
                }
            }
            "pipe_segments" => {
                for pipe in rows(model, key).filter(|p| !selected.contains(&id_of(p))) {
                    references(pipe, &selected, key, &mut found);
                }
            }
            "components" => {
                for component in rows(model, key).filter(|c| c.get("id").and_then(Value::as_str) != converted_id) {
                    references(component, &selected, key, &mut found);
                }
            }
            _ => references(collection, &selected, key, &mut found),
        }
    }
    if !found.is_empty() {
        return Err(corner_err(
            "OP-BEND-CORNER-ATTACHMENT-UNSUPPORTED",
            format!("Insert bend at corner moves the corner and shortens both pipes, so nothing may refer to them: remove or re-author {} first", found.join(", ")),
            selected.into_iter().collect(),
        ));
    }

    // The construction.
    let t1 = [c[0] - tangent_length * u_in[0], c[1] - tangent_length * u_in[1], c[2] - tangent_length * u_in[2]];
    let t2 = [c[0] + tangent_length * u_out[0], c[1] + tangent_length * u_out[1], c[2] + tangent_length * u_out[2]];
    let mut node_1 = new_nodes[0].clone();
    node_1["position"] = xyz(t1)?;
    let mut node_2 = new_nodes[1].clone();
    node_2["position"] = xyz(t2)?;
    let (id_1, id_2) = (node_1["id"].clone(), node_2["id"].clone());
    let retarget = |pipe: &Value, to: &Value| {
        let mut pipe = pipe.clone();
        for end in ["from", "to"] {
            if pipe[end] == corner {
                pipe[end] = to.clone();
            }
        }
        pipe
    };
    let (shortened_in, shortened_out) = (retarget(p_in, &id_1), retarget(p_out, &id_2));
    let mut arc = p_in.clone();
    let arc_meta = &payload["new_pipe"];
    for key in ["id", "label", "provenance"] {
        arc[key] = arc_meta[key].clone();
    }
    arc["from"] = id_1.clone();
    arc["to"] = id_2.clone();
    arc["y_reference"] = xyz(u_in)?;
    let radius_record = json!({"value": radius_value["value"], "unit": radius_value["unit"]});
    let component = match converted {
        Some(marker) => {
            let mut component = marker.clone();
            component["node"] = id_1.clone();
            component["geometry"]["bend_pipe_ref"] = arc["id"].clone();
            component["geometry"]["bend_radius"] = radius_record;
            if let Some(source) = &source_reference {
                component["geometry"]["bend_geometry_source_reference"] = json!(source);
            }
            if !component.get("modifiers").is_some_and(Value::is_object) {
                component["modifiers"] = json!({});
            }
            component["modifiers"]["flexibility_factor_user_value"] = k;
            component["modifiers"]["source_reference"] = json!(k_source);
            if !component.get("mechanics_interface").is_some_and(Value::is_object) {
                component["mechanics_interface"] = json!({});
            }
            component["mechanics_interface"]["solver_consumption"] = json!(CURVED);
            component
        }
        None => json!({
            "id": spec["id"], "label": spec["label"], "kind": "bend", "node": id_1,
            "geometry": {"bend_pipe_ref": arc["id"], "bend_radius": radius_record, "bend_geometry_source_reference": source_reference},
            "modifiers": {"flexibility_factor_user_value": k, "source_reference": k_source},
            "mechanics_interface": {"solver_consumption": CURVED},
            "provenance": spec["provenance"],
        }),
    };

    let mut candidate = model.clone();
    let nodes = candidate["nodes"].as_array_mut().ok_or_else(|| invalid("nodes must be an array"))?;
    nodes.retain(|n| n.get("id").and_then(Value::as_str) != Some(corner));
    nodes.push(node_1.clone());
    nodes.push(node_2.clone());
    let list = candidate["pipe_segments"].as_array_mut().ok_or_else(|| invalid("pipe_segments must be an array"))?;
    for pipe in list.iter_mut() {
        if pipe.get("id").and_then(Value::as_str) == Some(in_id.as_str()) {
            *pipe = shortened_in.clone();
        } else if pipe.get("id").and_then(Value::as_str) == Some(out_id.as_str()) {
            *pipe = shortened_out.clone();
        }
    }
    list.push(arc.clone());
    // W-13 §3.2: the arc (a clone of p_in, with its tags and notes) joins the
    // line of the incoming pipe p_in.
    let line_row = crate::model_data_authoring::join_source_line(&mut candidate, &in_id, arc["id"].as_str().unwrap_or(""));
    let components = candidate
        .as_object_mut()
        .ok_or_else(|| invalid("model must be an object"))?
        .entry("components")
        .or_insert_with(|| json!([]))
        .as_array_mut()
        .ok_or_else(|| invalid("components must be an array"))?;
    match converted_id {
        Some(id) => {
            for existing in components.iter_mut() {
                if existing.get("id").and_then(Value::as_str) == Some(id) {
                    *existing = component.clone();
                }
            }
        }
        None => components.push(component.clone()),
    }
    let text_id = |v: &Value| v.as_str().unwrap_or("").to_string();
    Ok(GeometryEdit {
        model: candidate,
        diff_preview: vec![
            diff(corner, "Node", "nodes", Some(corner_node), &json!("not_present"), "insert_bend_at_corner"),
            diff(&text_id(&id_1), "Node", "nodes", None, &node_1, "create_node"),
            diff(&text_id(&id_2), "Node", "nodes", None, &node_2, "create_node"),
            diff(&in_id, "Element", "pipe_segments", Some(p_in), &shortened_in, "insert_bend_at_corner"),
            diff(&out_id, "Element", "pipe_segments", Some(p_out), &shortened_out, "insert_bend_at_corner"),
            diff(&text_id(&arc["id"]), "Element", "pipe_segments", None, &arc, "connect_pipe_run"),
            diff(
                &text_id(&component["id"]),
                "Component",
                "components",
                converted,
                &component,
                if converted.is_some() { "insert_bend_at_corner" } else { "insert_component_symbol" },
            ),
        ]
        .into_iter()
        .chain(line_row)
        .collect(),
    })
}

//! The subject census (DEL-04-09 Design §5.3.2): `binding.subject_census`,
//! the counts of the subjects the run read from the model and the SHA-256 of
//! their canonical list `[{kind, id}]`, sorted by (kind, id) in code-point
//! order.
//!
//! **Shape** (closure): an object of exactly the five counts and the digest on
//! a solved document, null on a refused one.
//!
//! **Payload, always** (evidence): along each axis the identity table's nodal
//! member rows count the straight members, its resultant member rows the arcs
//! and its component-body rows the bodies; the discretization table has one
//! row per straight member; the supports table has one row per nonlinear
//! support, and there are no more nonlinear supports than supports. The
//! `supports` count names every support record, which no table lists, so it
//! is held only with the request.
//!
//! **With the bound request**, the subject list is derived from the model by
//! the rule below and compared with every count, the digest and the ids the
//! identity, discretization and supports tables carry. Without it the
//! identities are not re-derived, which stays scoped.
//!
//! **The subject rule**, stated here from the Design, not imported:
//! 1. the members are the `pipe_segments` (ids unique) less the spans that
//!    objective connectors replace (exact pressure route only; a connector on
//!    the pressure-free route is outside the model-mass basis and scoped as
//!    there);
//! 2. a member whose distributed mass a rigid-family component body replaces
//!    (the model-mass pass's eligibility: mapping, `mechanics_geometry_only`,
//!    node at an end of the span, finite positive weight and an entered
//!    centre of gravity) is listed as `component_body` by the component's id,
//!    and not as a member;
//! 3. any other member is `arc` when a curved-bend component realizes it, and
//!    otherwise `straight_member`;
//! 4. every support record is `support` by its id; its nonlinear supports are
//!    those with a `nonlinear` record, as the partition pass reads them.
//!
//! The pass is structural: it reads no numerical enclosure, so a mass that is
//! unresolved or outside the rounding model drops no subject, and a body the
//! producer would refuse (outside the range premise, say) stays a body: that
//! refusal is the mass relation's failure.
//!
//! **Self-check.** When the model-mass pass formed its members, its member and
//! body list is this pass's. A difference is a defect of this reader, never a
//! verdict on the document, so it leaves the evidence relation unresolved
//! (never passed, failed or a panic).
use super::model_mass::{Derived, MM_ARC, MM_GEOMETRY_ONLY, MM_MODEL_KEYS, MM_PIPE_KEYS, MM_RIGID_FAMILY, MM_SCHEMA_VERSION};
use super::{modal_reader_sha256, mr_at, mr_count, mr_hex, mr_object, Parsed, DIRECTIONS, R};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};

const CS_PATH: &str = "/binding/subject_census";
const CS_COUNTS: [&str; 5] = ["straight_members", "arcs", "component_bodies", "supports", "nonlinear_supports"];
const CS_KEYS: [&str; 6] = ["straight_members", "arcs", "component_bodies", "supports", "nonlinear_supports", "subjects_sha256"];

/// A solved document's census, as published.
pub(super) struct Census {
    straight: usize,
    arcs: usize,
    bodies: usize,
    supports: usize,
    nonlinear: usize,
    sha: String,
}

/// The census's shape (closure): `None` on a refused document.
pub(super) fn cs_shape(value: &Value, solved: bool) -> R<Option<Census>> {
    if !solved {
        return if value.is_null() { Ok(None) } else { Err(format!("{CS_PATH}: a refused document forms no subject census (null)")) };
    }
    let map = mr_object(value, CS_PATH, &CS_KEYS)?;
    let mut counts = [0usize; 5];
    for (k, key) in CS_COUNTS.iter().enumerate() {
        counts[k] = mr_count(&map[*key], &mr_at(CS_PATH, key))?;
    }
    mr_hex(&map["subjects_sha256"], &mr_at(CS_PATH, "subjects_sha256"))?;
    let [straight, arcs, bodies, supports, nonlinear] = counts;
    Ok(Some(Census { straight, arcs, bodies, supports, nonlinear, sha: map["subjects_sha256"].as_str().unwrap_or_default().to_string() }))
}

/// Why the subject list was not derived.
enum Stop {
    /// Outside the model-mass basis this pass shares: scoped.
    Scope(String),
    /// The bound model is not one a solved document can come from.
    Fail(String),
}

/// The model's subjects: the (kind, id) list in model order, and the
/// nonlinear supports' ids.
struct Subjects {
    list: Vec<(&'static str, String)>,
    nonlinear: Vec<String>,
}

fn cs_list<'v>(value: Option<&'v Value>, what: &str) -> Result<&'v [Value], Stop> {
    match value {
        None | Some(Value::Null) => Ok(&[]),
        Some(Value::Array(items)) => Ok(items.as_slice()),
        Some(_) => Err(Stop::Fail(format!("{what}: not an array"))),
    }
}

/// Whether a value in `unit` normalizes to a finite value of `dimension`'s
/// canonical unit (the catalog conversion the model-mass pass uses).
fn cs_converts(value: f64, unit: &str, dimension: Dimension) -> Option<f64> {
    let from = unit_by_symbol(unit, dimension).ok()?;
    let to = canonical_unit(dimension)?;
    convert_for_dimension(value, dimension, from, to).ok().filter(|v| v.is_finite())
}

/// The subject rule (module docs) on the bound request's model.
fn cs_subjects(request: &Value) -> Result<Subjects, Stop> {
    let Some(model) = request.get("model").and_then(Value::as_object) else {
        return Err(Stop::Fail("request.model: not an object".to_string()));
    };
    if model.get("schema_version").and_then(Value::as_str) != Some(MM_SCHEMA_VERSION) {
        return Err(Stop::Scope(format!("model schema_version is not {MM_SCHEMA_VERSION}, the version this reader's model basis covers")));
    }
    if let Some(key) = model.keys().find(|k| !MM_MODEL_KEYS.contains(&k.as_str())) {
        return Err(Stop::Scope(format!("model member `{key}` is outside this reader's model basis")));
    }
    // The route: only the exact pressure route replaces spans.
    let exact = match model.get("pressure_contract").filter(|c| !c.is_null()) {
        None => false,
        Some(c) => {
            if c.get("version").and_then(Value::as_str) != Some("3.0.0") || c.get("mode").and_then(Value::as_str) != Some("exact_pressure_v3") {
                return Err(Stop::Scope("the pressure contract is not exact_pressure_v3 3.0.0, the exact route this reader covers".to_string()));
            }
            true
        }
    };
    let pipes = cs_list(model.get("pipe_segments"), "model.pipe_segments")?;
    let pipe_ids: HashSet<&str> = pipes.iter().filter_map(|p| p.get("id").and_then(Value::as_str)).collect();
    if pipe_ids.len() != pipes.len() {
        return Err(Stop::Fail("a pipe id is missing or not unique".to_string()));
    }
    let span = |id: &str| pipes.iter().find(|q| q.get("id").and_then(Value::as_str) == Some(id));
    let components = cs_list(model.get("components"), "model.components")?;

    // Objective connectors' replaced spans and realized arcs.
    let mut replaced: HashSet<&str> = HashSet::new();
    let mut arcs: HashSet<&str> = HashSet::new();
    for (i, component) in components.iter().enumerate() {
        let kind = component.get("kind").and_then(Value::as_str).unwrap_or("");
        let consumption = component.get("mechanics_interface").and_then(|m| m.get("solver_consumption")).and_then(Value::as_str);
        if component.get("objective_connector").is_some_and(|c| !c.is_null()) {
            if kind != "expansion_joint" {
                return Err(Stop::Scope(format!("component {i} carries an objective connector but is not an expansion joint")));
            }
            if !exact {
                return Err(Stop::Scope("an objective connector on the pressure-free route is outside this reader's model basis".to_string()));
            }
            let replaced_span = component.get("objective_connector").and_then(|c| c.get("topology")).and_then(|t| t.get("span_ref")).and_then(Value::as_str);
            match replaced_span {
                Some(s) if pipe_ids.contains(s) => {
                    if !replaced.insert(s) {
                        return Err(Stop::Fail(format!("pipe {s} is replaced by two connectors, which the contract refuses")));
                    }
                }
                _ => return Err(Stop::Scope(format!("component {i}: its connector's replaced span is not decoded by this reader"))),
            }
        }
        if matches!(kind, "bend" | "elbow") && consumption.unwrap_or(MM_GEOMETRY_ONLY) == MM_ARC {
            let pipe = component.get("geometry").and_then(|g| g.get("bend_pipe_ref")).and_then(Value::as_str).filter(|s| !s.trim().is_empty());
            let Some(pipe) = pipe else {
                return Err(Stop::Fail(format!("bend component {i} requests a realized arc without its pipe, which the contract refuses")));
            };
            if !arcs.insert(pipe) {
                return Err(Stop::Scope(format!("pipe {pipe} is realized by two arcs, outside this reader's model basis")));
            }
            let Some(arc_span) = span(pipe) else {
                return Err(Stop::Fail(format!("bend component {i} names pipe {pipe}, which is not in the model")));
            };
            let node = component.get("node").and_then(Value::as_str).unwrap_or("");
            if arc_span.get("from").and_then(Value::as_str) != Some(node) && arc_span.get("to").and_then(Value::as_str) != Some(node) {
                return Err(Stop::Fail(format!("bend component {i}: its node is not an end of its arc span, which the contract refuses")));
            }
        }
    }

    // Rigid-family bodies: the mapping, then the eligibility rule.
    let mut mapped: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, component) in components.iter().enumerate() {
        let kind = component.get("kind").and_then(Value::as_str).unwrap_or("");
        if !MM_RIGID_FAMILY.contains(&kind) {
            continue;
        }
        let pipe = component.get("geometry").and_then(|g| g.get("rigid_pipe_ref")).and_then(Value::as_str).filter(|s| !s.trim().is_empty());
        if let Some(pipe) = pipe.filter(|p| pipe_ids.contains(p)) {
            if arcs.contains(pipe) || replaced.contains(pipe) {
                return Err(Stop::Fail(format!("rigid component {i} maps pipe {pipe}, a realized arc or replaced span: a mapping conflict the contract refuses")));
            }
            mapped.entry(pipe).or_default().push(i);
        }
    }
    let mut bodies: HashMap<&str, String> = HashMap::new();
    for (pipe, list) in &mapped {
        if list.len() > 1 {
            return Err(Stop::Fail(format!("pipe {pipe} is mapped by more than one rigid component: a mapping conflict the contract refuses")));
        }
        let component = &components[list[0]];
        if component.get("mechanics_interface").and_then(|m| m.get("solver_consumption")).and_then(Value::as_str) != Some(MM_GEOMETRY_ONLY) {
            continue;
        }
        let node = component.get("node").and_then(Value::as_str).unwrap_or("");
        let ends = span(pipe).map(|s| (s.get("from").and_then(Value::as_str), s.get("to").and_then(Value::as_str)));
        if ends.is_some_and(|(a, b)| a != Some(node) && b != Some(node)) {
            continue;
        }
        let geometry = component.get("geometry");
        let weight = geometry.and_then(|g| g.get("weight")).filter(|w| !w.is_null()).and_then(|w| {
            cs_converts(w.get("value").and_then(Value::as_f64)?, w.get("unit").and_then(Value::as_str)?, Dimension::Force)
        });
        if !weight.is_some_and(|w| w > 0.0) {
            continue;
        }
        let Some(cog) = geometry.and_then(|g| g.get("center_of_gravity")).filter(|c| !c.is_null()) else { continue };
        let entered = [cog.get("x"), cog.get("y"), cog.get("z")].map(|c| c.and_then(Value::as_f64));
        let (Some(unit), [Some(x), Some(y), Some(z)]) = (cog.get("unit").and_then(Value::as_str), entered) else { continue };
        if ![x, y, z].iter().all(|c| c.is_finite() && cs_converts(*c, unit, Dimension::Length).is_some()) {
            continue;
        }
        bodies.insert(*pipe, component.get("id").and_then(Value::as_str).unwrap_or("").to_string());
    }

    // The members, in model order.
    let mut list = Vec::new();
    for (i, pipe) in pipes.iter().enumerate() {
        let Some(map) = pipe.as_object() else {
            return Err(Stop::Fail(format!("model.pipe_segments[{i}]: not an object")));
        };
        if let Some(key) = map.keys().find(|k| !MM_PIPE_KEYS.contains(&k.as_str())) {
            return Err(Stop::Scope(format!("pipe member `{key}` is outside this reader's model basis")));
        }
        let id = map.get("id").and_then(Value::as_str).unwrap_or("");
        if replaced.contains(id) {
            continue;
        }
        list.push(match bodies.get(id) {
            Some(component) => ("component_body", component.clone()),
            None if arcs.contains(id) => ("arc", id.to_string()),
            None => ("straight_member", id.to_string()),
        });
    }
    // Every support record; its nonlinear supports.
    let mut nonlinear = Vec::new();
    for (i, support) in cs_list(model.get("supports"), "model.supports")?.iter().enumerate() {
        let Some(id) = support.get("id").and_then(Value::as_str) else {
            return Err(Stop::Fail(format!("model.supports[{i}]: no id")));
        };
        if support.get("nonlinear").is_some_and(|v| !v.is_null()) {
            nonlinear.push(id.to_string());
        }
        list.push(("support", id.to_string()));
    }
    Ok(Subjects { list, nonlinear })
}

/// SHA-256 of the canonical sorted subject list (Design §5.3.2).
fn cs_digest(sorted: &[(&'static str, String)]) -> Option<String> {
    modal_reader_sha256(&Value::Array(sorted.iter().map(|(kind, id)| json!({"kind": kind, "id": id})).collect()))
}

fn cs_sorted<'a>(ids: impl IntoIterator<Item = &'a str>) -> Vec<&'a str> {
    let mut ids: Vec<&str> = ids.into_iter().collect();
    ids.sort_unstable();
    ids
}

/// The evidence relation's census checks (module docs), appended to its items.
/// `bound` is whether the binding standing admits the supplied request.
pub(super) fn cs_relation(p: &Parsed, request: Option<&Value>, bound: bool, model: &Derived, mut items: Vec<String>) -> R<Vec<String>> {
    let c = &p.census;
    // The payload, always.
    for (k, axis) in DIRECTIONS.iter().enumerate() {
        let rows = || p.identity.iter().filter(move |r| r.axis == k);
        let nodal = rows().filter(|r| r.subject_kind == "member" && r.comparison == "nodal").count();
        let resultant = rows().filter(|r| r.subject_kind == "member" && r.comparison == "resultant").count();
        let bodies = rows().filter(|r| r.subject_kind == "component_body").count();
        for (rows, kind, counted) in [(nodal, "nodal member", c.straight), (resultant, "resultant member", c.arcs), (bodies, "component-body", c.bodies)] {
            if rows != counted {
                let name = match kind { "nodal member" => "straight_members", "resultant member" => "arcs", _ => "component_bodies" };
                return Err(format!("subject census: the identity table has {rows} {kind} rows along {axis}, but the census counts {counted} {name}"));
            }
        }
    }
    if p.members.len() != c.straight {
        return Err(format!("subject census: the discretization table has {} rows, but the census counts {} straight_members", p.members.len(), c.straight));
    }
    if p.support_rows.len() != c.nonlinear {
        return Err(format!("subject census: the supports table has {} rows, but the census counts {} nonlinear_supports", p.support_rows.len(), c.nonlinear));
    }
    if c.nonlinear > c.supports {
        return Err(format!("subject census: {} nonlinear_supports exceed its {} supports", c.nonlinear, c.supports));
    }

    // The identities, from the bound request only.
    let Some(request) = request else {
        items.push("subject census: no request is supplied, so the subject identities and the support count are not re-derived from the model; the counts are held to the tables only".to_string());
        return Ok(items);
    };
    if !bound {
        items.push("subject census: the supplied request is not the bound request, so the subject identities and the support count are not re-derived from it".to_string());
        return Ok(items);
    }
    let s = match cs_subjects(request) {
        Ok(s) => s,
        Err(Stop::Scope(reason)) => {
            items.push(format!("subject census not re-derived: {reason}"));
            return Ok(items);
        }
        Err(Stop::Fail(reason)) => return Err(format!("model-derived subject census: {reason}")),
    };
    let mut list = s.list;
    list.sort_unstable();
    if let Derived::Formed(f) = model {
        let mut formed = f.subjects.clone();
        formed.sort_unstable();
        let census: Vec<(&'static str, String)> = list.iter().filter(|(kind, _)| *kind != "support").cloned().collect();
        // A disagreement is the reader's defect, not a document verdict: the
        // relation is left unresolved rather than passed or failed.
        if formed != census {
            items.push(format!("{}model-derived: the reader's own cross-check disagrees: the model-mass pass formed {formed:?}, the census pass lists {census:?}", super::MR_UNRESOLVED));
            return Ok(items);
        }
    }
    let ids = |kind: &str| -> Vec<&str> { cs_sorted(list.iter().filter(|(k, _)| *k == kind).map(|(_, id)| id.as_str())) };
    let (straight, arcs, bodies, supports) = (ids("straight_member"), ids("arc"), ids("component_body"), ids("support"));
    // The subject sets each table carries, kind included (a member, a body and
    // the aggregate spelled alike stay distinct), held to the derived sets.
    let along = |k: usize, kind: &str, nodal: Option<bool>| -> Vec<&str> {
        cs_sorted(p.identity.iter()
            .filter(|r| r.axis == k && r.subject_kind == kind && nodal.map_or(true, |n| (r.comparison == "nodal") == n))
            .map(|r| r.subject.as_str()))
    };
    let nonlinear = cs_sorted(s.nonlinear.iter().map(String::as_str));
    let mut tables = Vec::new();
    for (k, axis) in DIRECTIONS.iter().enumerate() {
        tables.push((format!("the identity table's nodal members along {axis}"), along(k, "member", Some(true)), &straight, "straight members"));
        tables.push((format!("the identity table's resultant members along {axis}"), along(k, "member", Some(false)), &arcs, "arcs"));
        tables.push((format!("the identity table's component bodies along {axis}"), along(k, "component_body", None), &bodies, "component bodies"));
    }
    tables.push(("the discretization table's members".to_string(), cs_sorted(p.members.iter().map(String::as_str)), &straight, "straight members"));
    tables.push(("the supports table's supports".to_string(), cs_sorted(p.support_rows.iter().map(|r| r.id.as_str())), &nonlinear, "nonlinear supports"));
    for (what, published, derived, name) in tables {
        if published != *derived {
            let missing: Vec<&&str> = derived.iter().filter(|id| !published.contains(id)).collect();
            let extra: Vec<&&str> = published.iter().filter(|id| !derived.contains(id)).collect();
            return Err(if missing.is_empty() && extra.is_empty() {
                format!("model-derived subject census: {what} are not the bound model's {name} (a subject is listed more than once)")
            } else {
                format!("model-derived subject census: {what} are not the bound model's {name} (missing {missing:?}, not in the model {extra:?})")
            });
        }
    }
    for (name, published, derived) in [("straight_members", c.straight, straight.len()), ("arcs", c.arcs, arcs.len()), ("component_bodies", c.bodies, bodies.len()),
        ("supports", c.supports, supports.len()), ("nonlinear_supports", c.nonlinear, s.nonlinear.len())] {
        if published != derived {
            return Err(format!("model-derived subject census: {name} is {published}, but the bound model has {derived}"));
        }
    }
    if cs_digest(&list).as_deref() != Some(c.sha.as_str()) {
        return Err("model-derived subject census: subjects_sha256 is not the digest of the bound model's subject list".to_string());
    }
    Ok(items)
}

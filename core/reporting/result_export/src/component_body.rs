//! W-15 slice d (DEL-03-05 Design `rigid-component-stiffness-scaling.md` §5):
//! the reference-section identification of a component body's results.
//!
//! A member formed with a rigid-family component's DEC-045 stiffness scale is
//! the component's body; its stress rows are stresses of the mapped pipe
//! section, a reference section, not of the body. Each such row (every case,
//! section basis and combination), each such member's stress extremum and
//! maximum row carries the closed record `component_body` =
//! {`component_ref`, `stiffness_scale`, `section_role`}; each stress-maximum
//! coverage entry lists the bodies in `component_body_pipe_ids` when there are
//! any; the case maxima and headlines range over the other members. A result
//! without an applied scale carries none of these.
//!
//! [`source_bodies`] is the model-free check of a received source; the
//! model-bound [`model_checks`] derives the expected bodies from the model by
//! the same eligibility predicate the producer reads, and refuses a missing,
//! forged or inconsistent record.

use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};

/// The record member of a row or extremum.
pub const RECORD: &str = "component_body";
/// The coverage entry's list of body members.
pub const COVERAGE_KEY: &str = "component_body_pipe_ids";
/// The record's only `section_role`.
pub const SECTION_ROLE: &str = "reference_pipe_section";
/// The rule-binding refusal of a body row (`RULE_INPUTS_INCOMPLETE`).
pub const BODY_REFERENCE_SECTION: &str = "COMPONENT_BODY_REFERENCE_SECTION";

type Check = Result<(), String>;

fn fail(code: &str) -> String {
    format!("SOURCE_COMPONENT_BODY_{code}")
}

fn require(condition: bool, code: &str) -> Check {
    if condition {
        Ok(())
    } else {
        Err(fail(code))
    }
}

/// A row kind of `table` whose physical dimension is stress.
pub fn is_stress_kind(table: &Value, kind: &str) -> bool {
    table["rows"].as_array().is_some_and(|rows| {
        rows.iter()
            .any(|row| row["kind"] == kind && row["source_physical_semantic_dimension"] == "stress")
    })
}

/// The closed record shape: a nonempty component reference, a finite positive
/// scale and the reference-section role.
pub fn record_shape(record: &Value) -> bool {
    record.as_object().is_some_and(|o| {
        o.len() == 3
            && o.get("component_ref").and_then(Value::as_str).is_some_and(|s| !s.is_empty())
            && o.get("stiffness_scale").and_then(Value::as_f64).is_some_and(|s| s.is_finite() && s > 0.0)
            && o.get("section_role").and_then(Value::as_str) == Some(SECTION_ROLE)
    })
}

/// Two records name the same component and scale bit for bit.
fn same_record(a: &Value, b: &Value) -> bool {
    a["component_ref"] == b["component_ref"]
        && a["section_role"] == b["section_role"]
        && a["stiffness_scale"].as_f64().map(f64::to_bits) == b["stiffness_scale"].as_f64().map(f64::to_bits)
}

/// A copy of `value` without the slice-d members, for the readers' closed
/// shape checks of a record that may carry them.
pub fn without_record(value: &Value) -> Value {
    let mut copy = value.clone();
    if let Some(o) = copy.as_object_mut() {
        o.remove(RECORD);
        o.remove(COVERAGE_KEY);
    }
    copy
}

fn evidence_cases(source: &Value) -> Vec<&Value> {
    let evidence = &source["contract_evidence"];
    ["preview_cases", "exact_cases"]
        .into_iter()
        .filter_map(|list| evidence.get(list).and_then(Value::as_array))
        .flatten()
        .collect()
}

fn coverage_entries(case: &Value) -> Vec<&Value> {
    match &case["stress_maximum_coverage"] {
        Value::Array(entries) => entries.iter().collect(),
        entry @ Value::Object(_) => vec![entry],
        _ => Vec::new(),
    }
}

/// The body members a received source declares, each with its record.
///
/// Every coverage entry of every case lists the same nonempty bodies or none.
/// A row carries the record exactly when it is a stress row of `table` whose
/// entity is a listed body; a stress extremum carries it exactly when its
/// member is a body. One body's records agree bit for bit (across cases,
/// section bases, combinations and extrema), every listed body carries one,
/// and two bodies never name one component.
pub fn source_bodies<'a>(source: &'a Value, table: &Value) -> Result<BTreeMap<&'a str, &'a Value>, String> {
    let mut listed: Option<Vec<&str>> = None;
    for case in evidence_cases(source) {
        for entry in coverage_entries(case) {
            let ids = match entry.get(COVERAGE_KEY) {
                None => Vec::new(),
                Some(list) => {
                    let list = list.as_array().ok_or_else(|| fail("COVERAGE_SHAPE"))?;
                    require(!list.is_empty(), "COVERAGE_SHAPE")?;
                    let mut ids = Vec::new();
                    for id in list {
                        let id = id.as_str().filter(|s| !s.is_empty()).ok_or_else(|| fail("COVERAGE_SHAPE"))?;
                        require(!ids.contains(&id), "COVERAGE_SHAPE")?;
                        ids.push(id);
                    }
                    ids
                }
            };
            match &listed {
                Some(previous) => require(*previous == ids, "COVERAGE_DISAGREEMENT")?,
                None => listed = Some(ids),
            }
        }
    }
    let listed: HashSet<&str> = listed.unwrap_or_default().into_iter().collect();
    let mut records: BTreeMap<&str, &Value> = BTreeMap::new();
    let rows = source["results"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    for row in rows {
        let entity = row["entity_ref"].as_str().unwrap_or_default();
        let kind = row["kind"].as_str().unwrap_or_default();
        let expected = listed.contains(entity) && is_stress_kind(table, kind);
        match row.get(RECORD) {
            Some(record) => {
                require(expected, "FORGED")?;
                require(record_shape(record), "RECORD_SHAPE")?;
                match records.get(entity) {
                    Some(first) => require(same_record(first, record), "INCONSISTENT")?,
                    None => {
                        records.insert(entity, record);
                    }
                }
            }
            None => require(!expected, "MISSING")?,
        }
    }
    require(records.len() == listed.len(), "MISSING")?;
    let components: HashSet<&str> = records.values().filter_map(|r| r["component_ref"].as_str()).collect();
    require(components.len() == records.len(), "INCONSISTENT")?;
    for case in evidence_cases(source) {
        for extremum in case["pipe_stress_extrema"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
            let pipe = extremum["pipe_id"].as_str().unwrap_or_default();
            match (extremum.get(RECORD), records.get(pipe)) {
                (Some(record), Some(expected)) => {
                    require(record_shape(record) && same_record(record, expected), "INCONSISTENT")?
                }
                (None, None) => {}
                (Some(_), None) => return Err(fail("FORGED")),
                (None, Some(_)) => return Err(fail("MISSING")),
            }
        }
    }
    Ok(records)
}

/// A headline names no body member.
pub fn headline_not_body(headline: &Value, bodies: &BTreeMap<&str, &Value>) -> Check {
    require(
        headline.is_null() || !bodies.contains_key(headline["location_ref"].as_str().unwrap_or_default()),
        "HEADLINE",
    )
}

/// The model's expected bodies, by the producer's eligibility predicate
/// (DEL-03-05 Design §2.3): a valve, flange, reducer, rigid or specialty
/// component declaring `mechanics_geometry_only`, whose `rigid_pipe_ref` names
/// a member ending at its node, with a present, finite, positive scale and a
/// nonblank source reference, and every supplied linear or rotational stiffness
/// finite positive. Keyed by member: (component, scale). A member two
/// eligible components map is a conflict the producer blocks; it is returned
/// as an error.
pub fn model_bodies(model: &Value) -> Result<BTreeMap<String, (String, f64)>, String> {
    let empty = Vec::new();
    let pipes = model["pipe_segments"].as_array().unwrap_or(&empty);
    let mut bodies = BTreeMap::new();
    for component in model["components"].as_array().unwrap_or(&empty) {
        if !matches!(component["kind"].as_str(), Some("valve" | "flange" | "reducer" | "rigid" | "specialty")) {
            continue;
        }
        if component["mechanics_interface"]["solver_consumption"] != "mechanics_geometry_only" {
            continue;
        }
        let node = component["node"].as_str().unwrap_or_default();
        let Some(pipe_ref) = component["geometry"]["rigid_pipe_ref"].as_str().filter(|s| !s.trim().is_empty()) else {
            continue;
        };
        if !pipes.iter().any(|p| p["id"] == pipe_ref && (p["from"] == node || p["to"] == node)) {
            continue;
        }
        let modifiers = &component["modifiers"];
        let Some(scale) = modifiers["stiffness_scaling_user_value"]["value"].as_f64() else { continue };
        if modifiers["source_reference"].as_str().is_none_or(|s| s.trim().is_empty()) || !(scale.is_finite() && scale > 0.0) {
            continue;
        }
        let optional_invalid = ["linear_stiffness_user_value", "rotational_stiffness_user_value"]
            .into_iter()
            .filter_map(|key| modifiers.get(key).filter(|q| !q.is_null()))
            .any(|q| !q["value"].as_f64().is_some_and(|v| v.is_finite() && v > 0.0));
        if optional_invalid {
            continue;
        }
        let id = component["id"].as_str().unwrap_or_default().to_string();
        require(bodies.insert(pipe_ref.to_string(), (id, scale)).is_none(), "MODEL_CONFLICT")?;
    }
    Ok(bodies)
}

/// The model-bound check: the source's bodies and records are exactly the
/// model's (component and scale bit for bit).
pub fn model_checks(source: &Value, model: &Value, table: &Value) -> Check {
    let received = source_bodies(source, table)?;
    let expected = model_bodies(model)?;
    require(received.len() == expected.len(), "MODEL_MISMATCH")?;
    for (pipe, (component, scale)) in &expected {
        let record = received.get(pipe.as_str()).ok_or_else(|| fail("MODEL_MISMATCH"))?;
        require(
            record["component_ref"] == component.as_str()
                && record["stiffness_scale"].as_f64().map(f64::to_bits) == Some(scale.to_bits()),
            "MODEL_MISMATCH",
        )?;
    }
    Ok(())
}

/// The record a derivative quantity carries for a source row: the row's own,
/// verbatim; `None` without one.
pub fn derivative_record(row: &Value) -> Option<Value> {
    row.get(RECORD).cloned()
}

/// A derivative quantity's record agrees with its source row: both absent, or
/// the row's record by value (the scale bit for bit, so a JSON form change such
/// as JCS's `15` for `15.0` still binds).
pub fn derivative_agrees(quantity: &Value, row: &Value) -> bool {
    match (quantity.get(RECORD), row.get(RECORD)) {
        (None, None) => true,
        (Some(q), Some(r)) => record_shape(q) && record_shape(r) && same_record(q, r),
        _ => false,
    }
}

/// HashMap view for the readers' member loops.
pub fn body_set<'a>(bodies: &BTreeMap<&'a str, &'a Value>) -> HashMap<&'a str, &'a Value> {
    bodies.iter().map(|(k, v)| (*k, *v)).collect()
}

//! An independent twin builder for model document 0.9.0's pressure-free
//! condition resolution (DEL-02-01 Work `model-document-0.9.0.md` §1.5, §1.8),
//! written from the design text on JSON values. It does not call the product's
//! lowering or its arithmetic: each temperature difference is read from the
//! independent reference table `validation/references/rt_g5_s2_conditions/`
//! (route A: Python `Fraction(repr(v))`, exact unit definitions, `float()`).
//!
//! T(D) is D with every condition member removed and each case's generated
//! records added: thermal primitives in K (temperature), then weight primitives
//! (added weight), each in `pipe_segments` order after the case's authored
//! primitives, and the case's wind exposure from the sets' `exposure.wind`.
//! Shared by the in-crate structure test (E1) and the public-path tests (E2-E7).
#![allow(dead_code)]
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

pub const REFERENCES: &str =
    include_str!("../../../../validation/references/rt_g5_s2_conditions/references.json");

pub fn references() -> Value {
    serde_json::from_str(REFERENCES).expect("the reference table parses")
}

fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}

/// The reference ΔT (K) for (T, unit) − (T₀, unit); the fixtures use only
/// pairs the table holds, so the twin never computes a difference itself.
pub fn reference_difference(temperature: &Value, installation: &Value) -> f64 {
    let key = |q: &Value| (bits(q["value"].as_f64().unwrap()), q["unit"].as_str().unwrap().to_string());
    let (t, t0) = (key(temperature), key(installation));
    let table = references();
    let row = table["temperature_differences"].as_array().unwrap().iter()
        .find(|row| {
            let operand = |member: &str| (row[member]["bits"].as_str().unwrap().to_string(), row[member]["unit"].as_str().unwrap().to_string());
            operand("temperature") == t && operand("installation") == t0 && row.get("expected").is_some()
        })
        .unwrap_or_else(|| panic!("no admitted reference row for {temperature} - {installation}"));
    f64::from_bits(u64::from_str_radix(row["expected"]["bits"].as_str().unwrap(), 16).unwrap())
}

/// JCS of a flat object of strings: members sorted by key (all ASCII here, so
/// byte order is UTF-16 order), JSON string escaping.
fn jcs(members: &[(&str, &str)]) -> String {
    let sorted: BTreeMap<&str, &str> = members.iter().copied().collect();
    serde_json::to_string(&sorted).unwrap()
}

fn prefixed(kind: &str, case: &str, pipe: &str) -> String {
    format!("load:condition-{kind}:{}:{case}:{}:{pipe}", case.len(), pipe.len())
}

/// Where a generated primitive goes relative to the authored ones; `Prepended`
/// and set order are mutants the tests use to show order is observable.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Governing,
    Prepended,
    SetOrder,
}

pub fn twin(document: &Value) -> Value {
    twin_with(document, Order::Governing)
}

pub fn twin_with(document: &Value, order: Order) -> Value {
    let mut out = document.clone();
    let sets: Vec<Value> = document.get("condition_sets").and_then(Value::as_array).cloned().unwrap_or_default();
    let set_of = |id: &str| sets.iter().find(|set| set["id"] == id).unwrap_or_else(|| panic!("set {id}")).clone();
    let mut pipes: Vec<(String, Value)> = document["pipe_segments"].as_array().unwrap().iter()
        .filter_map(|pipe| pipe.get("condition_set_ref").map(|r| (pipe["id"].as_str().unwrap().to_string(), set_of(r.as_str().unwrap()))))
        .collect();
    assert_eq!(pipes.len(), document["pipe_segments"].as_array().unwrap().len(), "the twin is defined when every pipe carries a set");
    if order == Order::SetOrder {
        let rank = |set: &Value| sets.iter().position(|s| s["id"] == set["id"]).unwrap();
        pipes.sort_by_key(|(_, set)| rank(set));
    }
    for case in out["load_cases"].as_array_mut().unwrap() {
        let Some(using) = case.as_object_mut().unwrap().remove("condition_use") else { continue };
        let case_id = case["id"].as_str().unwrap().to_string();
        let mut generated = Vec::new();
        if let Some(key) = using.get("temperature").and_then(Value::as_str) {
            for (pipe, set) in &pipes {
                let condition = set["conditions"].as_array().unwrap().iter().find(|c| c["key"] == key).unwrap();
                let set_id = set["id"].as_str().unwrap();
                let dt = reference_difference(&condition["temperature"], &set["installation_temperature"]);
                generated.push(json!({
                    "id": prefixed("temperature", &case_id, pipe), "category": "thermal",
                    "target": {"type": "element", "pipe": pipe}, "direction": "global_z",
                    "magnitude": {"value": dt, "unit": "K"}, "dimension": "temperature_change",
                    "provenance": jcs(&[("condition", key), ("condition_set", set_id), ("method", "operating_condition_set_v1"), ("quantity", "temperature")]),
                }));
            }
        }
        if using.get("added_weight") == Some(&json!(true)) {
            for (pipe, set) in &pipes {
                let Some(weight) = set.get("added_weight") else { continue };
                generated.push(json!({
                    "id": prefixed("added-weight", &case_id, pipe), "category": "weight",
                    "target": {"type": "element", "pipe": pipe}, "direction": weight["direction"].clone(),
                    "magnitude": weight["magnitude"].clone(), "dimension": "force_per_length",
                    "provenance": jcs(&[("condition_set", set["id"].as_str().unwrap()), ("method", "operating_condition_set_v1"), ("quantity", "added_weight")]),
                }));
            }
        }
        if using.get("wind_exposure") == Some(&json!(true)) {
            let exposed: Vec<Value> = pipes.iter()
                .filter(|(_, set)| set.get("exposure").and_then(|e| e.get("wind")) == Some(&json!(true)))
                .map(|(pipe, _)| json!(pipe)).collect();
            case["equivalent_static"]["wind"]["exposed_pipe_refs"] = Value::Array(exposed);
        }
        let loads = case.as_object_mut().unwrap().entry("primitive_loads").or_insert_with(|| json!([]));
        let authored = loads.as_array_mut().unwrap();
        if order == Order::Prepended {
            let mut all = generated;
            all.append(authored);
            *authored = all;
        } else {
            authored.extend(generated);
        }
    }
    let object: &mut Map<String, Value> = out.as_object_mut().unwrap();
    object.remove("condition_sets");
    object.remove("spectra");
    for pipe in object["pipe_segments"].as_array_mut().unwrap() {
        pipe.as_object_mut().unwrap().remove("condition_set_ref");
    }
    out
}

pub const PROV: &str = "invented_rt_g5_s2_test_input";

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// A pressure-free line of three straight pipes along x (invented inputs): an
/// anchor at N0, a UZ line stop at N1 and a UX spring at N3. Pipes carry three
/// sets listed in an order different from the pipes'. Cases: OPE (temperature
/// and added weight, with authored weights 1e8, -1e8 and -0.3 N/m on P2, so the
/// sum with the set's -0.3 N/m depends on order),
/// SUS (added weight), WIND (wind exposure) and PLAIN (typed ΔT in degF, no
/// condition use); combinations over OPE and SUS.
/// `version` is the caller's `MODEL_SCHEMA_VERSION`, so the version step carries it.
pub fn document(version: &str) -> Value {
    let pipe = |id: &str, from: &str, to: &str, set: &str| json!({"id": id, "from": from, "to": to, "material": "mat",
        "y_reference": {"x": 0, "y": 0, "z": 1}, "section": {"outside_diameter": q(0.2, "m"), "wall_thickness": q(0.01, "m")},
        "condition_set_ref": set, "provenance": PROV});
    let node = |id: &str, x: f64| json!({"id": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV});
    let weight = |id: &str, pipe: &str, value: f64| json!({"id": id, "category": "weight", "target": {"type": "element", "pipe": pipe},
        "direction": "global_z", "magnitude": q(value, "N/m"), "dimension": "force_per_length", "provenance": PROV});
    let typed = |id: &str, pipe: &str| json!({"id": id, "category": "thermal", "target": {"type": "element", "pipe": pipe},
        "direction": "global_x", "magnitude": q(7.0, "degF"), "dimension": "temperature_change", "provenance": PROV});
    json!({
        "schema_version": version,
        "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": "invented:rt-g5-s2", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC", "stress": "Pa"}},
        "nodes": [node("N0", 0.0), node("N1", 3.0), node("N2", 6.0), node("N3", 9.0)],
        "pipe_segments": [pipe("P1", "N0", "N1", "cs:warm"), pipe("P2", "N1", "N2", "cs:hot"), pipe("P3", "N2", "N3", "cs:cold")],
        "materials": [{"id": "mat", "elastic_modulus": q(2.0e11, "Pa"), "shear_modulus": q(8.0e10, "Pa"),
                       "thermal_expansion_coefficient": q(1.2e-5, "1/degC"), "provenance": PROV}],
        "supports": [
            {"id": "anchor", "node": "N0", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
            {"id": "stop", "node": "N1", "family": "line_stop", "restraints": ["UZ"], "provenance": PROV},
            {"id": "spring", "node": "N3", "family": "spring", "restraints": ["UX"], "stiffness": {"dof": "UX", "value": q(1.0e7, "N/m")}, "provenance": PROV},
        ],
        "condition_sets": [
            {"id": "cs:cold", "provenance": PROV, "installation_temperature": q(20.0, "degC"),
             "conditions": [{"key": "OPE1", "temperature": q(373.15, "K")}],
             "added_weight": {"magnitude": q(200.0, "N/m"), "direction": "global_y"}, "exposure": {"wind": true}},
            {"id": "cs:hot", "provenance": PROV, "installation_temperature": q(20.0, "degC"),
             "conditions": [{"key": "HYDRO", "temperature": q(20.0, "degC")}, {"key": "OPE1", "temperature": q(150.0, "degC"), "pressure": q(1.5, "MPa")}],
             "added_weight": {"magnitude": q(-0.3, "N/m"), "direction": "global_z"}, "exposure": {"wind": true, "snow_ice": true}},
            {"id": "cs:warm", "provenance": PROV, "installation_temperature": q(70.0, "degF"),
             "conditions": [{"key": "OPE1", "temperature": q(212.0, "degF")}], "exposure": {"wind": false}},
        ],
        "load_cases": [
            {"id": "OPE", "condition_use": {"temperature": "OPE1", "added_weight": true},
             "primitive_loads": [weight("OPE:w1", "P2", 1.0e8), weight("OPE:w2", "P2", -1.0e8), weight("OPE:w3", "P2", -0.3)], "provenance": PROV},
            {"id": "SUS", "condition_use": {"added_weight": true}, "primitive_loads": [], "provenance": PROV},
            {"id": "WIND", "condition_use": {"wind_exposure": true}, "primitive_loads": [], "provenance": PROV,
             "equivalent_static": {"wind": {"pressure": q(1200.0, "Pa"), "shape_factor": q(0.7, "1"), "direction": "global_y"}, "provenance": PROV}},
            {"id": "PLAIN", "primitive_loads": [typed("PLAIN:t1", "P1"), typed("PLAIN:t3", "P3")], "provenance": PROV},
        ],
        "combinations": [
            {"id": "C-SUM", "basis": "mechanics", "terms": [{"load_case": "OPE", "factor": 1.0}, {"load_case": "SUS", "factor": 1.0}], "provenance": PROV},
            {"id": "C-SUB", "basis": "result_state_subtraction", "minuend_id": "OPE", "subtrahend_id": "SUS", "provenance": PROV},
            {"id": "C-DB", "basis": "result_state_subtraction", "minuend_id": "OPE", "subtrahend_id": "SUS", "difference_basis": "operating_minus_sustained", "provenance": PROV},
        ],
    })
}

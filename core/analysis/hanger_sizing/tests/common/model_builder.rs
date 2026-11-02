//! An invented pressure-free solve request builder shared by the crate's
//! integration tests (`derived_solves.rs`, `publication.rs`).
#![allow(dead_code)]

use serde_json::{json, Value};

pub struct Member<'a> {
    pub id: &'a str,
    pub from: &'a str,
    pub to: &'a str,
    pub outside_diameter_m: f64,
    pub wall_m: f64,
    pub elastic_modulus_pa: f64,
    pub expansion_per_k: f64,
}

pub fn material_id(member: &str) -> String {
    format!("material:{member}")
}

/// An invented pressure-free solve request: `{model, materials}`.
pub fn request(nodes: &[(&str, [f64; 3])], members: &[Member], supports: Vec<Value>, load_cases: Vec<Value>) -> Value {
    let materials: Vec<Value> = members
        .iter()
        .map(|m| json!({
            "id": material_id(m.id),
            "label": "Invented test material",
            "elastic_modulus": {"value": m.elastic_modulus_pa, "unit": "Pa"},
            "shear_modulus": {"value": m.elastic_modulus_pa / 2.6, "unit": "Pa"},
            "thermal_expansion_coefficient": {"value": m.expansion_per_k, "unit": "1/degC"},
            "provenance": "invented_test_value"
        }))
        .collect();
    let pipes: Vec<Value> = members
        .iter()
        .map(|m| {
            let a = nodes.iter().find(|(id, _)| *id == m.from).unwrap().1;
            let b = nodes.iter().find(|(id, _)| *id == m.to).unwrap().1;
            let vertical = (a[0] - b[0]).abs() < 1e-12 && (a[1] - b[1]).abs() < 1e-12;
            let y_reference = if vertical { json!({"x": 1, "y": 0, "z": 0}) } else { json!({"x": 0, "y": 0, "z": 1}) };
            json!({
                "id": m.id, "from": m.from, "to": m.to,
                "section": {"outside_diameter": {"value": m.outside_diameter_m, "unit": "m"}, "wall_thickness": {"value": m.wall_m, "unit": "m"}},
                "material": material_id(m.id), "y_reference": y_reference, "provenance": "invented_test_value"
            })
        })
        .collect();
    let nodes: Vec<Value> = nodes
        .iter()
        .map(|(id, p)| json!({"id": id, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": "invented_test_value"}))
        .collect();
    json!({
        "model": {
            "schema_version": "0.9.0",
            "document_kind": "openpipestress.product_preview.model",
            "data_boundary": {
                "public_examples_policy": "invented_or_cleared_data_only",
                "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
                "private_data_policy": "no_private_project_data",
                "professional_boundary": "technical_preview_requires_human_engineering_review"
            },
            "project": {"id": "project:hanger-sizing-test", "name": "Invented hanger sizing test",
                        "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC"}},
            "materials": materials,
            "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                                "rule_check": "not_performed_user_rule_inputs_missing",
                                "professional_acceptance": "not_provided"},
            "nodes": nodes,
            "pipe_segments": pipes,
            "supports": supports,
            "components": [],
            "load_cases": load_cases,
            "combinations": []
        },
        "materials": []
    })
}

pub fn support(id: &str, node: &str, family: &str, restraints: &[&str]) -> Value {
    json!({"id": id, "node": node, "family": family, "restraints": restraints, "provenance": "invented_test_value"})
}

/// A stiff rotational spring that removes a beam's torsional mechanism.
pub fn torsion_spring(id: &str, node: &str) -> Value {
    json!({"id": id, "node": node, "family": "spring", "restraints": ["RX"],
           "stiffness": {"dof": "RX", "value": {"value": 1.0e9, "unit": "N*m/rad"}}, "provenance": "invented_test_value"})
}

pub fn lift_off(id: &str, node: &str) -> Value {
    json!({"id": id, "node": node, "family": "nonlinear", "restraints": [],
           "nonlinear": {"behavior": "lift_off", "dof": "UZ", "initial_state": "active", "contact_when": "positive_reaction"},
           "provenance": "invented_test_value"})
}

pub fn line_load(id: &str, pipe: &str, w_down_n_per_m: f64) -> Value {
    json!({"id": id, "category": "weight", "target": {"type": "element", "pipe": pipe}, "direction": "global_z",
           "magnitude": {"value": -w_down_n_per_m, "unit": "N/m"}, "dimension": "force_per_length", "provenance": "invented_test_value"})
}

pub fn thermal(id: &str, pipe: &str, delta_t: f64) -> Value {
    json!({"id": id, "category": "thermal", "target": {"type": "element", "pipe": pipe}, "direction": "global_z",
           "magnitude": {"value": delta_t, "unit": "degC"}, "dimension": "temperature_interval", "provenance": "invented_test_value"})
}

pub fn nodal_fz(id: &str, node: &str, fz_n: f64) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": node}, "direction": "global_z",
           "magnitude": {"value": fz_n, "unit": "N"}, "dimension": "force", "provenance": "invented_test_value"})
}

pub fn case(id: &str, loads: Vec<Value>) -> Value {
    json!({"id": id, "kind": "primitive_user_load", "primitive_loads": loads, "provenance": "invented_test_value"})
}

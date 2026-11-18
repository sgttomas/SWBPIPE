//! The preload reference's models (`validation/references/hanger_sizing/preload.json`,
//! P-A and P-B) authored as product documents, and readers of the published rows.
//! Shared by `spring_preload_references.rs` and the crate's own preload tests;
//! it imports no product code.
#![allow(dead_code)]
use serde_json::{json, Value};

pub const REFERENCE: &str = include_str!("../../../../validation/references/hanger_sizing/preload.json");
pub const PROV: &str = "invented_spring_preload_reference";
pub const MATERIAL: &str = "material:preload";
pub const STATE_CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
pub const REFERENCE_CONFIGURATION: &str = "reference:installed";
pub const UNHELD: [&str; 8] = ["PA-WT", "PA-EMPTY", "PA-BASE", "PA-B", "PA-AB", "PB-WT", "PB-EMPTY", "PB-BASE"];
pub const APPLIED: &str = "variable_spring_hanger_applied_load";
pub const COMPANION: &str = "companion";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Route {
    PressureFree,
    Exact,
}
pub const ROUTES: [Route; 2] = [Route::PressureFree, Route::Exact];

/// How the hangers are authored: as the reference states them, or as the
/// force-plus-spring twin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hangers {
    Variable,
    Twin,
}

pub fn reference() -> Value {
    serde_json::from_str(REFERENCE).unwrap()
}

pub fn close(observed: f64, expected: f64) -> bool {
    (observed - expected).abs() <= 1e-9 * observed.abs().max(expected.abs()) + 1e-12
}

pub fn assert_close(observed: f64, expected: f64, what: &str) {
    assert!(close(observed, expected), "{what}: observed {observed:e}, expected {expected:e}");
}

pub fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

pub fn model_of<'a>(reference: &'a Value, case_id: &str) -> (&'a Value, &'a Value) {
    for model in reference["models"].as_array().unwrap() {
        if let Some(case) = model["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == case_id) {
            return (model, case);
        }
    }
    panic!("no case {case_id}")
}

pub fn primitive(load: &Value, factor: f64) -> Value {
    assert_eq!(load["kind"], "nodal_force");
    assert_eq!(load["direction"], "+Z");
    json!({"id": load["id"], "category": "concentrated_force", "target": {"type": "node", "node": load["node"]},
        "direction": "global_z", "magnitude": q(load["value_n"].as_f64().unwrap() * factor, "N"), "dimension": "force", "provenance": PROV})
}

pub fn hanger_record(support: &Value) -> Value {
    let k = support["stiffness_n_per_m"].as_f64().unwrap();
    let cold = support["cold_load_n"].as_f64().unwrap();
    json!({"id": support["id"], "node": support["node"], "family": "variable_spring_hanger", "restraints": ["UZ"],
        "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ", "value": q(k, "N/m")},
            "installed_load": q(cold, "N"), "cold_load": q(cold, "N"), "hot_load": q(cold, "N"),
            "travel_range": q(0.05, "m"), "source_reference": "invented spring preload reference",
            "manufacturer_reference": "invented_no_catalog", "load_side_review_reference": "invented_review"},
        "provenance": PROV})
}

/// The one-case document of a reference case on a route.
pub fn document(case_id: &str, route: Route, schema_version: &str) -> Value {
    authored(case_id, route, Hangers::Variable, schema_version)
}

pub fn authored(case_id: &str, route: Route, hangers: Hangers, schema_version: &str) -> Value {
    let reference = reference();
    let (model, case) = model_of(&reference, case_id);
    assert!(case["test_condition"].is_null(), "{case_id}: a test condition is out of scope");
    let section = &model["section"];
    let e = section["elastic_modulus_pa"].as_f64().unwrap();
    let nu = section["poisson_ratio"].as_f64().unwrap();
    let nodes: Vec<Value> = model["nodes"].as_object().unwrap().iter().map(|(id, xyz)| {
        json!({"id": id, "position": {"x": xyz[0], "y": xyz[1], "z": xyz[2]}, "provenance": PROV})
    }).collect();
    let mut pipe_section = json!({"outside_diameter": q(section["outside_diameter_m"].as_f64().unwrap(), "m"),
        "wall_thickness": q(section["wall_thickness_m"].as_f64().unwrap(), "m")});
    if route == Route::Exact {
        pipe_section["mill_tolerance"] = q(0.0, "m");
        pipe_section["corrosion_allowance"] = q(0.0, "m");
    }
    let pipes: Vec<Value> = model["members"].as_array().unwrap().iter().map(|m| {
        json!({"id": m["id"], "from": m["end_nodes"][0], "to": m["end_nodes"][1], "material": MATERIAL,
            "section": pipe_section, "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV})
    }).collect();
    let mut loads: Vec<Value> = case["includes"].as_array().unwrap().iter().map(|include| {
        let stored = model["stored_loads"].as_array().unwrap().iter().find(|l| l["id"] == include["load"]).unwrap();
        primitive(stored, include["factor"].as_f64().unwrap())
    }).collect();
    let mut supports = Vec::new();
    for support in model["supports"].as_array().unwrap() {
        match support["kind"].as_str().unwrap() {
            "rigid" => supports.push(json!({"id": support["id"], "node": support["node"],
                // The rigid family that admits exactly the stated DOFs (a rotation needs an anchor's record).
                "family": if support["dofs"] == json!(["UZ"]) { "vertical_support" }
                    else if support["dofs"].as_array().unwrap().iter().any(|d| d.as_str().unwrap().starts_with('R')) { "anchor" } else { "guide" },
                "restraints": support["dofs"], "provenance": PROV})),
            "variable_spring_hanger" => match hangers {
                Hangers::Variable => supports.push(hanger_record(support)),
                Hangers::Twin => {
                    supports.push(json!({"id": support["id"], "node": support["node"], "family": "spring", "restraints": ["UZ"],
                        "stiffness": {"dof": "UZ", "value": q(support["stiffness_n_per_m"].as_f64().unwrap(), "N/m")}, "provenance": PROV}));
                    loads.push(json!({"id": format!("load:twin:{}", support["id"].as_str().unwrap()), "category": "concentrated_force",
                        "target": {"type": "node", "node": support["node"]}, "direction": "global_z",
                        "magnitude": q(support["cold_load_n"].as_f64().unwrap(), "N"), "dimension": "force", "provenance": PROV}));
                }
            },
            other => panic!("support kind {other}"),
        }
    }
    let moved = case["spring_base_displacements"].as_array().unwrap();
    let materials = match route {
        Route::PressureFree => json!([{"id": MATERIAL, "elastic_modulus": q(e, "Pa"), "shear_modulus": q(e / (2.0 * (1.0 + nu)), "Pa"), "provenance": PROV}]),
        Route::Exact => json!([{"id": MATERIAL, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": q(e, "Pa"), "poisson_ratio": q(nu, "1"), "provenance": PROV}]),
    };
    let mut load_case = json!({"id": case_id, "kind": "primitive_user_load", "primitive_loads": loads, "provenance": PROV});
    if route == Route::PressureFree {
        for movement in moved {
            load_case["primitive_loads"].as_array_mut().unwrap().push(json!({"id": format!("motion:{}", movement["support"].as_str().unwrap()),
                "category": "imposed_displacement", "target": {"type": "support", "support": movement["support"], "dof": movement["dof"]},
                "direction": movement["dof"], "magnitude": q(movement["value_m"].as_f64().unwrap(), "m"), "dimension": "displacement", "provenance": PROV}));
        }
    }
    let mut document = json!({"model": {"schema_version": schema_version, "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:spring-preload", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "materials": materials, "supports": supports, "components": [],
        "load_cases": [load_case], "combinations": []}, "materials": []});
    // PF refuses a document in which no case has a primitive load
    // (LOAD_INPUT_MISSING), as it does for a model whose only action is a
    // constant-effort force: an unloaded reference case is solved beside a
    // loaded companion, after it, and only the reference case's rows are read.
    if route == Route::PressureFree && document["model"]["load_cases"][0]["primitive_loads"].as_array().unwrap().is_empty() {
        let first = model["stored_loads"].as_array().unwrap()[0].clone();
        document["model"]["load_cases"].as_array_mut().unwrap().push(json!({"id": COMPANION, "kind": "primitive_user_load",
            "primitive_loads": [primitive(&first, 1.0)], "provenance": PROV}));
    }
    if route == Route::Exact {
        let m = &mut document["model"];
        m["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
        m["load_cases"][0]["pressure_regions"] = json!([]);
        if !moved.is_empty() {
            // EX moves a spring's base through the load/reference-state group: `base_motion` on
            // its stiffness axis (R-c, `case_state/resolve.rs`).
            let pipe_ids: Vec<Value> = m["pipe_segments"].as_array().unwrap().iter().map(|p| p["id"].clone()).collect();
            m["reference_configurations"] = json!([{"id": REFERENCE_CONFIGURATION, "label": "Installed reference",
                "geometry_ref": {"kind": "authored_model_geometry"},
                "member_references": pipe_ids.iter().map(|id| json!({"pipe_ref": id, "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": PROV})).collect::<Vec<_>>(),
                "provenance": PROV}]);
            let support_states: Vec<Value> = m["supports"].as_array().unwrap().iter().map(|s| {
                let mut state = json!({"support_ref": s["id"], "participation": {"kind": "active_model_device"}});
                let motion: Vec<Value> = moved.iter().filter(|g| g["support"] == s["id"])
                    .map(|g| json!({"dof": g["dof"], "value": q(g["value_m"].as_f64().unwrap(), "m"), "meaning": "absolute_reference_displacement"})).collect();
                if !motion.is_empty() {
                    state["base_motion"] = Value::Array(motion);
                }
                state
            }).collect();
            let sources: Vec<Value> = m["load_cases"][0]["primitive_loads"].as_array().unwrap().iter()
                .map(|l| json!({"source_ref": l["id"], "factor": 1.0})).collect();
            m["load_cases"][0]["analysis_state"] = json!({"contract": STATE_CONTRACT, "reference_configuration_ref": REFERENCE_CONFIGURATION,
                "element_states": pipe_ids.iter().map(|id| json!({"pipe_ref": id,
                    "material_selection": {"kind": "explicit_base_properties", "material_ref": MATERIAL,
                        "applicability_reference": "invented analytical basis declared applicable to this member"},
                    "thermal_state": {"kind": "unchanged_reference", "provenance": PROV}})).collect::<Vec<_>>(),
                "support_states": support_states, "load_sources": sources,
                "history": {"kind": "independent_equilibrium"}, "provenance": PROV});
        }
    }
    document
}



pub fn rows<'a>(envelope: &'a Value, kind: &str, entity: &str, component: Option<&str>) -> Vec<&'a Value> {
    envelope["results"].as_array().unwrap().iter().filter(|r| {
        r["kind"] == kind && r["entity_ref"] == entity
            && r.get("basis_ref").is_none_or(|b| b.is_null() || (b["ref_type"] == "load_case" && b["ref_id"] != COMPANION))
            && !r["id"].as_str().unwrap().starts_with(&format!("result:loadcase:{COMPANION}:"))
            && component.is_none_or(|c| r["metadata"]["component"] == c)
    }).collect()
}

fn one(envelope: &Value, kind: &str, entity: &str, component: Option<&str>) -> f64 {
    let found = rows(envelope, kind, entity, component);
    assert_eq!(found.len(), 1, "{kind} {entity} {component:?}: {} rows", found.len());
    let unit = found[0]["unit"].as_str().unwrap();
    let scale = match unit { "m" | "N" => 1.0, "mm" => 1e-3, "kN" => 1e3, other => panic!("unit {other}") };
    found[0]["value"].as_f64().unwrap() * scale
}

/// A node's published UZ displacement in metres.
pub fn uz(envelope: &Value, node: &str) -> f64 {
    one(envelope, "global_nodal_displacement_z", node, None)
}

/// A support's published force along +Z on the pipe, in newtons.
pub fn fz(envelope: &Value, support: &str) -> f64 {
    one(envelope, "support_reaction_component_v2", support, Some("Fz"))
}

/// A hanger's published applied force, in newtons.
pub fn applied(envelope: &Value, support: &str) -> f64 {
    one(envelope, APPLIED, support, None)
}

pub fn compare_block(envelope: &Value, expected: &Value, what: &str) {
    for (node, u) in expected["u_nodes_m"].as_object().unwrap() {
        assert_close(uz(envelope, node), u.as_f64().unwrap(), &format!("{what} u {node}"));
    }
    for (support, force) in expected["published_support_force_n"].as_object().unwrap() {
        assert_close(fz(envelope, support), force.as_f64().unwrap(), &format!("{what} support force {support}"));
    }
    for (support, force) in expected["published_applied_force_n"].as_object().unwrap() {
        let force = force.as_f64().expect("an unheld case has an applied force");
        assert_close(applied(envelope, support), force, &format!("{what} applied force {support}"));
    }
    assert!(expected["applied_force_null_reason"].as_object().unwrap().is_empty(), "{what}");
}


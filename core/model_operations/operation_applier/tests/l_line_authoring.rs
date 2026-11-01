//! T4-U2 phase 3: the first usable path, authored end to end through the
//! common operation seam and solved by PP in both modes. An L line (straight,
//! realized bend, straight) is authored from its corner: E/ν material, Insert
//! bend at corner (R and the user's k), the pressure region through the bend,
//! the `3.0.0/exact_pressure_v3` profile, then thermal and weight loads. The
//! authored document solves under `pressure-2` and agrees with T4-I7's L-line
//! document (U2-L-ANCH-PTW-K2), whose bend was placed by hand.
//!
//! The bend operations (realize, insert at corner), the region checks and the
//! agent refusal of k are covered here as well. All data are invented.
//!
//! B7b: the corner document is the current 0.9.0 pressure-free document; the
//! user enters each member's corrosion allowance (an explicit zero) before
//! activating the complete `3.0.0/exact_pressure_v3` contract (euler_bernoulli).
//! T4-I7's frozen 0.3.0 sketch is unchanged; its copy is re-authored to 0.9.0
//! by the one shared rule (`core/product_physics/tests/common/current_model.rs`).
use open_pipe_stress_operation_applier::{apply_operation, canonical_json, sha256_hex, validate_operation};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use std::collections::HashMap;

#[path = "../../../product_physics/tests/common/current_model.rs"]
mod current_model;

const I7_DOCUMENTS: &str = include_str!("../../../../validation/references/t4_i7/u2_document_sketches.json");
const PROVENANCE: &str = "invented T4-U2 authoring input (from T4-I7's L line); not library, component, catalog or code-rule data";
/// The desktop's authoring check reads these bytes: the corner document, the
/// document authored from it here, and PP's sparse pressure-2 result for it.
/// Regenerate with `T4_U2_WRITE_L_LINE_FIXTURE=1`; otherwise they must be current.
const FIXTURE: &str = include_str!("../../../../fixtures/model_operations/pressure_v3_l_line_authoring.json");
const FIXTURE_PATH: &str = "../../../fixtures/model_operations/pressure_v3_l_line_authoring.json";

fn i7_l_line() -> Value {
    let documents: Value = serde_json::from_str(I7_DOCUMENTS).unwrap();
    documents["sketches"]["U2-L-ANCH-PTW-K2"]["0.3.0"]["document"]["model"].clone()
}

fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}

#[allow(clippy::too_many_arguments)]
fn intent(kind: &str, change_kind: &str, object: &str, id: &str, path: &str, before: &str, after: &str, unit: &str, dimension: &str) -> Value {
    json!({"operation_id":format!("op:l-line:{id}:{path}"),"operation_kind":kind,"operation_status":"proposed","author_type":"user",
      "target":{"object_type":object,"ref":id},
      "change":{"change_id":format!("change:l-line:{id}:{path}"),"change_kind":change_kind,"field_label":path,"field_path":path,"before":before,"after":after,"unit":unit,"dimension":dimension,"source_note":"invented explicit user entry"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "rationale":"invented explicit user authoring through the operation pipeline",
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false}})
}

fn structured(change_kind: &str, object: &str, id: &str, path: &str, before: Option<&Value>, after: &Value) -> Value {
    let before = before.map(canonical_json).unwrap_or_else(|| "not_present".into());
    intent("modify", change_kind, object, id, path, &before, &canonical_json(after), "none", "dimensionless")
}

fn apply(model: &Value, intent: &Value) -> Value {
    let claim = hash(model);
    let preview = validate_operation(model, intent, Some(&claim));
    assert!(preview.applied_model.is_none());
    assert!(!preview.diff_preview.is_empty(), "{:?}", preview.diagnostics);
    let outcome = apply_operation(model, intent, Some(&claim));
    outcome.applied_model.unwrap_or_else(|| panic!("{:?}", outcome.diagnostics))
}

fn codes(model: &Value, intent: &Value) -> Vec<String> {
    let outcome = apply_operation(model, intent, Some(&hash(model)));
    assert!(outcome.applied_model.is_none(), "applied unexpectedly");
    outcome.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect()
}

/// The L line's corner document before any bend or load: A —S1— X —S2— D,
/// anchored at A and D, pressure-free (current 0.9.0, no pressure contract and
/// no corrosion allowance yet), with a legacy E/G material.
fn corner_document() -> Value {
    let mut model = i7_l_line();
    model["schema_version"] = json!(current_model::CURRENT_MODEL_VERSION);
    model.as_object_mut().unwrap().remove("pressure_contract");
    model["project"]["id"] = json!("project:t4-u2-l-line-authoring");
    let nodes = model["nodes"].as_array_mut().unwrap();
    nodes.retain(|n| n["id"] == "node:A" || n["id"] == "node:D");
    nodes.insert(1, json!({"id": "node:X", "label": "Corner", "position": {"x": 3.25, "y": 0.0, "z": 0.0}, "provenance": PROVENANCE}));
    let pipes = model["pipe_segments"].as_array_mut().unwrap();
    pipes.retain(|p| p["id"] != "pipe:BEND");
    pipes[0]["to"] = json!("node:X");
    pipes[1]["from"] = json!("node:X");
    model["components"] = json!([]);
    let material = &mut model["materials"][0];
    for key in ["constitutive_basis", "poisson_ratio"] {
        material.as_object_mut().unwrap().remove(key);
    }
    material["shear_modulus"] = json!({"value": 7.7e10, "unit": "Pa"});
    for case in model["load_cases"].as_array_mut().unwrap() {
        case["primitive_loads"] = json!([]);
        case.as_object_mut().unwrap().remove("pressure_regions");
    }
    model
}

fn insert_bend_payload() -> Value {
    json!({
        "radius": {"value": 250.0, "unit": "mm"},
        "flexibility_factor": {"value": 2.0, "unit": "none"},
        "flexibility_source_reference": "invented user value for this test (k = 2)",
        "new_nodes": [
            {"id": "node:B", "label": "Bend start", "provenance": PROVENANCE},
            {"id": "node:C", "label": "Bend end", "provenance": PROVENANCE}
        ],
        "new_pipe": {"id": "pipe:BEND", "label": "Bend arc", "provenance": PROVENANCE},
        "component": {"id": "component:bend-1", "label": "Bend", "provenance": PROVENANCE, "geometry_source_reference": "inserted at the corner by the user"}
    })
}

fn corner_before(model: &Value, node: &str) -> String {
    let pipes: Vec<&Value> = model["pipe_segments"].as_array().unwrap().iter().filter(|p| p["from"] == node || p["to"] == node).collect();
    let markers: Vec<&Value> = model["components"].as_array().unwrap().iter()
        .filter(|c| (c["kind"] == "bend" || c["kind"] == "elbow") && c["node"] == node).collect();
    let corner = model["nodes"].as_array().unwrap().iter().find(|n| n["id"] == node).unwrap();
    canonical_json(&json!({"node": corner, "pipes": pipes, "markers": markers}))
}

fn insert_bend(model: &Value, payload: &Value) -> Value {
    intent("modify", "insert_bend_at_corner", "Node", "node:X", "bend_at_corner", &corner_before(model, "node:X"), &payload.to_string(), "none", "dimensionless")
}

/// The I7 load case, authored explicitly in its pieces.
fn authored(material_op: bool) -> Value {
    let reference = i7_l_line();
    let mut model = corner_document();
    if material_op {
        let keys = ["constitutive_basis", "elastic_modulus", "shear_modulus", "poisson_ratio", "thermal_expansion_coefficient", "provenance"];
        let projection = |m: &Value| Value::Object(keys.iter().filter_map(|k| m.get(*k).map(|v| ((*k).into(), v.clone()))).collect());
        let target = projection(&reference["materials"][0]);
        model = apply(&model, &structured("set_field", "Material", "material:steel", "constitutive_properties", Some(&projection(&model["materials"][0])), &target));
    }
    model = apply(&model, &insert_bend(&model, &insert_bend_payload()));
    // B7b: each member's corrosion allowance, entered explicitly (zero is an
    // authored value, never a default) before the contract is activated.
    for pipe in ["pipe:S1", "pipe:BEND", "pipe:S2"] {
        model = apply(&model, &intent("modify", "set_field", "Element", pipe, "section.corrosion_allowance.value", "TBD", "0", "m", "length"));
    }
    let regions = reference["load_cases"][0]["pressure_regions"].clone();
    model = apply(&model, &structured("update_load", "Load", "case:u2", "pressure_regions", None, &regions));
    let profile = json!({"schema_version": current_model::CURRENT_MODEL_VERSION, "pressure_contract": current_model::current_contract()});
    model = apply(&model, &structured("set_field", "Model", "project:t4-u2-l-line-authoring", "pressure_profile", Some(&json!({"schema_version": current_model::CURRENT_MODEL_VERSION})), &profile));
    for load in reference["load_cases"][0]["primitive_loads"].as_array().unwrap() {
        let (unit, dimension) = (load["magnitude"]["unit"].as_str().unwrap(), load["dimension"].as_str().unwrap());
        model = apply(&model, &intent("create", "create_primitive_load", "Load", "case:u2", "primitive_loads", "not_present", &load.to_string(), unit, dimension));
    }
    model
}

fn solve(model: &Value, mode: PreviewSolverMode) -> Value {
    let envelope = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), mode).unwrap();
    serde_json::to_value(envelope).unwrap()
}

/// A row's identity; D-5 stress rows are published once per section basis
/// (`; section_basis=…` on the recovery token), which is part of the key.
fn row_key(row: &Value) -> String {
    let basis = row["metadata"]["basis"].as_str().and_then(|b| b.rsplit_once("; section_basis=")).map_or("", |(_, b)| b);
    format!("{}|{}|{}|{}|{}|{basis}", row["basis_ref"]["ref_id"], row["entity_ref"], row["kind"], row["metadata"]["component"], row["metadata"]["location"])
}

#[test]
fn the_l_line_is_authored_from_its_corner_and_solves_under_pressure_1_in_both_modes() {
    let model = authored(true);
    // The construction is exact: the tangent points are the hand-placed B and C.
    let position = |id: &str| model["nodes"].as_array().unwrap().iter().find(|n| n["id"] == id).unwrap()["position"].clone();
    assert_eq!(position("node:B"), json!({"x": 3.0, "y": 0.0, "z": 0.0}));
    assert_eq!(position("node:C"), json!({"x": 3.25, "y": 0.25, "z": 0.0}));
    assert!(model["nodes"].as_array().unwrap().iter().all(|n| n["id"] != "node:X"));
    let bend = &model["components"][0];
    assert_eq!(bend["mechanics_interface"]["solver_consumption"], "curved_bend_macro_element");
    assert_eq!(bend["modifiers"]["flexibility_factor_user_value"], json!({"value": 2.0, "unit": "none"}));
    assert_eq!(bend["geometry"]["bend_pipe_ref"], "pipe:BEND");
    assert!(bend["geometry"].get("bend_angle").is_none() && bend["geometry"].get("bend_plane_orientation").is_none());
    assert_eq!(model["pressure_contract"], current_model::current_contract());
    assert_eq!(model["schema_version"], current_model::CURRENT_MODEL_VERSION);
    for pipe in model["pipe_segments"].as_array().unwrap() {
        assert_eq!(pipe["section"]["corrosion_allowance"], json!({"value": 0.0, "unit": "m"}), "{}", pipe["id"]);
    }
    // Plain JSON transport preserves the authored document.
    let restored: Value = serde_json::from_str(&serde_json::to_string(&model).unwrap()).unwrap();
    assert_eq!(canonical_json(&restored), canonical_json(&model));

    // T4-I7's hand-placed document: the same geometry, loads and k (its frozen
    // 0.3.0 copy re-authored to the current document).
    let mut reference = i7_l_line();
    current_model::reauthor_model(&mut reference);
    reference["project"]["id"] = model["project"]["id"].clone();
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let authored_result = solve(&restored, mode);
        assert_eq!(authored_result["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {}", authored_result["diagnostics"]);
        assert_eq!(authored_result["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/pressure-2");
        assert_eq!(authored_result["formulation_basis"]["profile_id"], "exact_pressure_v3");
        let reference_result = solve(&reference, mode);
        assert_eq!(reference_result["status"]["mechanics"], "MECHANICS_SOLVED");
        let expected: HashMap<String, f64> = reference_result["results"].as_array().unwrap().iter()
            .map(|row| (row_key(row), row["value"].as_f64().unwrap())).collect();
        let rows = authored_result["results"].as_array().unwrap();
        assert_eq!(rows.len(), expected.len(), "{mode:?}");
        let scale = expected.values().fold(0.0_f64, |m, v| m.max(v.abs()));
        let mut compared = 0;
        for row in rows {
            let key = row_key(row);
            let Some(&value) = expected.get(&key) else { panic!("{mode:?}: no reference row {key}") };
            let actual = row["value"].as_f64().unwrap();
            // Same arc up to y_reference's rounding (u_in versus I7's (1,-1,0)) and node order.
            assert!((actual - value).abs() <= 1e-9 * value.abs().max(1e-6 * scale), "{mode:?} {key}: {actual} vs {value}");
            compared += 1;
        }
        assert!(compared > 50, "{mode:?}: {compared} rows");
    }
}

#[test]
fn insert_bend_at_corner_refuses_what_it_cannot_construct() {
    let model = corner_document();
    let mut payload = insert_bend_payload();
    // A radius whose tangent length reaches a straight; a radius in no length unit.
    payload["radius"] = json!({"value": 3.5, "unit": "m"});
    assert!(codes(&model, &insert_bend(&model, &payload)).contains(&"OP-BEND-CORNER-RADIUS-INVALID".into()));
    payload["radius"] = json!({"value": 0.25, "unit": "N"});
    assert!(codes(&model, &insert_bend(&model, &payload)).contains(&"OP-BEND-CORNER-RADIUS-INVALID".into()));
    // k missing, non-positive or not dimensionless.
    for k in [json!(null), json!({"value": 0.0, "unit": "none"}), json!({"value": 2.0, "unit": "1"})] {
        let mut payload = insert_bend_payload();
        payload["flexibility_factor"] = k;
        assert!(codes(&model, &insert_bend(&model, &payload)).contains(&"OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED".into()));
    }
    // An agent never authors k.
    let mut agent = insert_bend(&model, &insert_bend_payload());
    agent["author_type"] = json!("agent");
    assert!(codes(&model, &agent).contains(&"OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED".into()));
    // Without model-hash evidence; with a stale corner context.
    let outcome = apply_operation(&model, &insert_bend(&model, &insert_bend_payload()), None);
    assert!(outcome.diagnostics.iter().any(|d| d.code == "OP-GEOMETRY-MODEL-HASH-REQUIRED"));
    let mut stale = insert_bend(&model, &insert_bend_payload());
    stale["change"]["before"] = json!("{}");
    assert!(codes(&model, &stale).contains(&"OP-BEFORE-VALUE-MISMATCH".into()));
    // Different sections; a collinear node; an attachment at the corner.
    let mut mismatch = model.clone();
    mismatch["pipe_segments"][1]["section"]["wall_thickness"]["value"] = json!(0.008);
    assert!(codes(&mismatch, &insert_bend(&mismatch, &insert_bend_payload())).contains(&"OP-BEND-CORNER-SECTION-MISMATCH".into()));
    let mut straight = model.clone();
    straight["nodes"][2]["position"] = json!({"x": 7.5, "y": 0.0, "z": 0.0});
    assert!(codes(&straight, &insert_bend(&straight, &insert_bend_payload())).contains(&"OP-BEND-CORNER-ANGLE-INVALID".into()));
    let mut attached = model.clone();
    attached["supports"].as_array_mut().unwrap().push(json!({"id": "support:X", "node": "node:X", "family": "anchor", "restraints": ["UX"], "provenance": PROVENANCE}));
    assert!(codes(&attached, &insert_bend(&attached, &insert_bend_payload())).contains(&"OP-BEND-CORNER-ATTACHMENT-UNSUPPORTED".into()));
    // An id collision.
    let mut collision = insert_bend_payload();
    collision["new_pipe"]["id"] = json!("pipe:S1");
    assert!(!codes(&model, &insert_bend(&model, &collision)).is_empty());
}

#[test]
fn a_corner_marker_is_converted_in_place_keeping_its_identity() {
    let mut model = corner_document();
    let marker = json!({"id": "component:C-7", "label": "Marker", "kind": "bend", "node": "node:X",
        "geometry": {"bend_pipe_ref": "pipe:S1", "bend_radius": {"value": 0.45, "unit": "m"}, "bend_angle": {"value": 90.0, "unit": "deg"}, "bend_geometry_source_reference": "user"},
        "modifiers": {"sif_user_value": {"value": 1.3, "unit": "none"}}, "provenance": "user marker"});
    model["components"] = json!([marker]);
    // A second bend beside the marker is refused; the marker is converted instead.
    assert!(codes(&model, &insert_bend(&model, &insert_bend_payload())).contains(&"OP-BEND-CORNER-MARKER-PRESENT".into()));
    let mut payload = insert_bend_payload();
    payload["component"] = json!({"convert": "component:C-7"});
    let applied = apply(&model, &insert_bend(&model, &payload));
    let converted = &applied["components"][0];
    assert_eq!(converted["id"], "component:C-7");
    assert_eq!(converted["label"], "Marker");
    assert_eq!(converted["node"], "node:B");
    assert_eq!(converted["geometry"]["bend_pipe_ref"], "pipe:BEND");
    assert_eq!(converted["geometry"]["bend_radius"], json!({"value": 250.0, "unit": "mm"}));
    assert_eq!(converted["modifiers"]["sif_user_value"], json!({"value": 1.3, "unit": "none"}));
    assert_eq!(converted["mechanics_interface"]["solver_consumption"], "curved_bend_macro_element");
    // A marker whose angle disagrees with the corner is refused.
    let mut conflicting = model.clone();
    conflicting["components"][0]["geometry"]["bend_angle"] = json!({"value": 45.0, "unit": "deg"});
    assert!(codes(&conflicting, &insert_bend(&conflicting, &payload)).contains(&"OP-BEND-CORNER-ANGLE-CONFLICT".into()));
}

fn realization(component: &Value) -> Value {
    let mut out = serde_json::Map::new();
    if let Some(v) = component.pointer("/mechanics_interface/solver_consumption") {
        out.insert("solver_consumption".into(), v.clone());
    }
    for key in ["flexibility_factor_user_value", "source_reference", "sif_in_plane_user_value", "sif_out_of_plane_user_value"] {
        if let Some(v) = component.get("modifiers").and_then(|m| m.get(key)) {
            out.insert(key.into(), v.clone());
        }
    }
    Value::Object(out)
}

fn realize(model: &Value, after: &Value) -> Value {
    let component = &model["components"][0];
    let current = realization(component);
    let before = if current.as_object().unwrap().is_empty() { None } else { Some(&current) };
    structured("set_field", "Component", component["id"].as_str().unwrap(), "bend_realization", before, after)
}

/// The authored L line with its bend returned to a geometry-only span bend
/// (pressure-free): the subject of the realize operation.
fn span_bend_document() -> Value {
    let mut model = authored(true);
    model["components"][0]["mechanics_interface"]["solver_consumption"] = json!("mechanics_geometry_only");
    model["components"][0].as_object_mut().unwrap().remove("modifiers");
    for case in model["load_cases"].as_array_mut().unwrap() {
        case["pressure_regions"] = json!([]);
    }
    model
}

/// T4-U4b (§2.6): the realization takes the directional SIF pair with its
/// source, both or neither; the solve then publishes the directional rows.
#[test]
fn realizing_a_bend_takes_the_directional_sif_pair_with_its_source() {
    let model = span_bend_document();
    let k = json!({"value": 2.0, "unit": "none"});
    let with_pair = json!({"solver_consumption": "curved_bend_macro_element", "flexibility_factor_user_value": k, "source_reference": "invented user values",
        "sif_in_plane_user_value": {"value": 2.3, "unit": "1"}, "sif_out_of_plane_user_value": {"value": 1.7, "unit": "1"}});
    let applied = apply(&model, &realize(&model, &with_pair));
    assert_eq!(
        applied["components"][0]["modifiers"],
        json!({"flexibility_factor_user_value": k, "source_reference": "invented user values",
            "sif_in_plane_user_value": {"value": 2.3, "unit": "1"}, "sif_out_of_plane_user_value": {"value": 1.7, "unit": "1"}})
    );
    let solved = solve(&applied, PreviewSolverMode::SparseInteractive);
    assert!(
        solved["results"].as_array().unwrap().iter().any(|r| r["kind"] == "curved_bend_directional_intensified_bending_stress_v1"),
        "{:?}", solved["diagnostics"]
    );
    // Refused by name: one of the pair, a non-positive or non-finite value, a wrong unit or extra key.
    let pair = |ip: Value, op: Option<Value>| {
        let mut after = with_pair.clone();
        after["sif_in_plane_user_value"] = ip;
        match op {
            Some(op) => after["sif_out_of_plane_user_value"] = op,
            None => { after.as_object_mut().unwrap().remove("sif_out_of_plane_user_value"); }
        }
        after
    };
    for bad in [
        pair(json!({"value": 2.3, "unit": "1"}), None),
        pair(json!({"value": 0.0, "unit": "1"}), Some(json!({"value": 1.7, "unit": "1"}))),
        pair(json!({"value": -2.3, "unit": "1"}), Some(json!({"value": 1.7, "unit": "1"}))),
        pair(json!({"value": 2.3, "unit": "mm"}), Some(json!({"value": 1.7, "unit": "1"}))),
        pair(json!({"value": 2.3, "unit": "1", "basis": "code"}), Some(json!({"value": 1.7, "unit": "1"}))),
        pair(json!(2.3), Some(json!({"value": 1.7, "unit": "1"}))),
    ] {
        assert!(codes(&model, &realize(&model, &bad)).contains(&"OP-BEND-DIRECTIONAL-SIF-INCOMPLETE".into()), "{bad}");
    }
    // The source stays required.
    let mut unsourced = with_pair.clone();
    unsourced.as_object_mut().unwrap().remove("source_reference");
    assert!(codes(&model, &realize(&model, &unsourced)).contains(&"OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED".into()));
    // An agent never writes the pair.
    let mut agent = realize(&model, &with_pair);
    agent["author_type"] = json!("agent");
    assert!(codes(&model, &agent).contains(&"OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED".into()));
    // A pair is read in the arc's frame: returning to geometry-only with it is refused.
    let back = json!({"solver_consumption": "mechanics_geometry_only"});
    assert!(codes(&applied, &realize(&applied, &back)).contains(&"OP-BEND-DIRECTIONAL-SIF-NOT-REALIZED-ARC".into()));
    // Realizing again without the pair removes it; then the bend may return to geometry-only.
    let without = json!({"solver_consumption": "curved_bend_macro_element", "flexibility_factor_user_value": k, "source_reference": "invented user values"});
    let cleared = apply(&applied, &realize(&applied, &without));
    assert_eq!(cleared["components"][0]["modifiers"], json!({"flexibility_factor_user_value": k, "source_reference": "invented user values"}));
    let chord = apply(&cleared, &realize(&cleared, &back));
    assert_eq!(chord["components"][0]["mechanics_interface"]["solver_consumption"], "mechanics_geometry_only");
}

#[test]
fn realizing_a_span_bend_writes_k_and_the_consumption_only() {
    let model = span_bend_document();
    let after = json!({"solver_consumption": "curved_bend_macro_element", "flexibility_factor_user_value": {"value": 2.0, "unit": "none"}, "source_reference": "invented user value"});
    let applied = apply(&model, &realize(&model, &after));
    let mut expected = model.clone();
    expected["components"][0]["mechanics_interface"]["solver_consumption"] = json!("curved_bend_macro_element");
    expected["components"][0]["modifiers"] = json!({"flexibility_factor_user_value": {"value": 2.0, "unit": "none"}, "source_reference": "invented user value"});
    assert_eq!(canonical_json(&applied), canonical_json(&expected));
    // Back to geometry-only changes the consumption only (k stays user data).
    let back = apply(&applied, &realize(&applied, &json!({"solver_consumption": "mechanics_geometry_only"})));
    assert_eq!(back["components"][0]["mechanics_interface"]["solver_consumption"], "mechanics_geometry_only");
    assert_eq!(back["components"][0]["modifiers"], applied["components"][0]["modifiers"]);
    // k missing or non-positive, a missing source, an agent author.
    for bad in [
        json!({"solver_consumption": "curved_bend_macro_element", "source_reference": "s"}),
        json!({"solver_consumption": "curved_bend_macro_element", "flexibility_factor_user_value": {"value": -1.0, "unit": "none"}, "source_reference": "s"}),
        json!({"solver_consumption": "curved_bend_macro_element", "flexibility_factor_user_value": {"value": 2.0, "unit": "none"}}),
    ] {
        assert!(codes(&model, &realize(&model, &bad)).contains(&"OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED".into()));
    }
    let mut agent = realize(&model, &after);
    agent["author_type"] = json!("agent");
    assert!(codes(&model, &agent).contains(&"OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED".into()));
    let mut agent_k = structured("set_field", "Component", "component:bend-1", "modifiers.flexibility_factor_user_value.value", None, &json!(2.0));
    agent_k["author_type"] = json!("agent");
    assert!(codes(&applied, &agent_k).contains(&"OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED".into()));
    // A stale before-value.
    let mut stale = realize(&model, &after);
    stale["change"]["before"] = json!("not_present");
    assert!(codes(&applied, &stale).contains(&"OP-BEFORE-VALUE-MISMATCH".into()));
}

#[test]
fn realization_refuses_markers_inconsistent_geometry_and_kinks() {
    let model = span_bend_document();
    let after = json!({"solver_consumption": "curved_bend_macro_element", "flexibility_factor_user_value": {"value": 2.0, "unit": "none"}, "source_reference": "invented user value"});
    // A desktop-made marker whose span is an incident straight: the radius cannot span it.
    let mut marker = model.clone();
    marker["components"][0]["geometry"]["bend_pipe_ref"] = json!("pipe:S1");
    assert!(codes(&marker, &realize(&marker, &after)).contains(&"OP-BEND-REALIZE-CORNER-MARKER".into()));
    // No span at all.
    let mut no_span = model.clone();
    no_span["components"][0]["geometry"].as_object_mut().unwrap().remove("bend_pipe_ref");
    assert!(codes(&no_span, &realize(&no_span, &after)).contains(&"OP-BEND-REALIZE-SPAN-MISSING".into()));
    // An authored angle that disagrees; a y_reference parallel to the chord.
    let mut angle = model.clone();
    angle["components"][0]["geometry"]["bend_angle"] = json!({"value": 80.0, "unit": "deg"});
    assert!(codes(&angle, &realize(&angle, &after)).contains(&"OP-BEND-GEOMETRY-INCONSISTENT".into()));
    let mut parallel = model.clone();
    let arc = parallel["pipe_segments"].as_array_mut().unwrap().iter_mut().find(|p| p["id"] == "pipe:BEND").unwrap();
    arc["y_reference"] = json!({"x": 1.0, "y": 1.0, "z": 0.0});
    assert!(codes(&parallel, &realize(&parallel, &after)).contains(&"OP-BEND-GEOMETRY-INCONSISTENT".into()));
    // A kink of 2e-3 rad at the single neighbour of an end: D moves off the tangent.
    let mut kinked = model.clone();
    let d = kinked["nodes"].as_array_mut().unwrap().iter_mut().find(|n| n["id"] == "node:D").unwrap();
    d["position"]["x"] = json!(3.25 + 4.0 * 2.0e-3);
    assert!(codes(&kinked, &realize(&kinked, &after)).contains(&"OP-BEND-TANGENCY-EXCEEDED".into()));
    // A non-bend target.
    let mut valve = model.clone();
    valve["components"][0]["kind"] = json!("valve");
    assert!(codes(&valve, &realize(&valve, &after)).contains(&"OP-BEND-REALIZE-TARGET-INVALID".into()));
}

#[test]
fn pressure_regions_admit_realized_bends_and_refuse_geometry_only_spans_and_mitres() {
    let realized = authored(true);
    let regions = realized["load_cases"][0]["pressure_regions"].clone();
    let op = |_: &Value| structured("update_load", "Load", "case:u2", "pressure_regions", Some(&regions), &regions);
    // The authored region is admitted again (an unchanged replacement).
    let outcome = apply_operation(&realized, &op(&realized), Some(&hash(&realized)));
    assert!(outcome.diagnostics.iter().all(|d| d.severity != "blocking"), "{:?}", outcome.diagnostics);
    // A geometry-only bend span in the region: realize this bend.
    let mut geometry_only = realized.clone();
    geometry_only["components"][0]["mechanics_interface"]["solver_consumption"] = json!("mechanics_geometry_only");
    let refused = codes(&geometry_only, &op(&geometry_only));
    assert!(refused.contains(&"OP-PRESSURE-REGION-BEND-NOT-REALIZED".into()), "{refused:?}");
    let message = apply_operation(&geometry_only, &op(&geometry_only), Some(&hash(&geometry_only))).diagnostics[0].message.clone();
    assert!(message.contains("realize this bend"), "{message}");
    // A mitre at a bend end: 2e-3 rad between the arc and S2.
    let mut kinked = realized.clone();
    let d = kinked["nodes"].as_array_mut().unwrap().iter_mut().find(|n| n["id"] == "node:D").unwrap();
    d["position"]["x"] = json!(3.25 + 4.0 * 2.0e-3);
    assert!(codes(&kinked, &op(&kinked)).contains(&"OP-PRESSURE-REGION-MITRE-UNSUPPORTED".into()));
    // An admitted kink (5e-4 rad) stays admitted, as at solve.
    let mut admitted = realized.clone();
    let d = admitted["nodes"].as_array_mut().unwrap().iter_mut().find(|n| n["id"] == "node:D").unwrap();
    d["position"]["x"] = json!(3.25 + 4.0 * 5.0e-4);
    let outcome = apply_operation(&admitted, &op(&admitted), Some(&hash(&admitted)));
    assert!(outcome.diagnostics.iter().all(|d| d.severity != "blocking"), "{:?}", outcome.diagnostics);
}

#[test]
fn the_desktop_l_line_fixture_is_current() {
    let authored = authored(true);
    let fixture = json!({
        "fixture": "T4-U2 L line authored from its corner (invented); read by the desktop's authoring and pressure-2 refusal tests",
        "generator": "core/model_operations/operation_applier/tests/l_line_authoring.rs::the_desktop_l_line_fixture_is_current",
        "corner_document": corner_document(),
        "authored_model": authored,
        "sparse_result": solve(&authored, PreviewSolverMode::SparseInteractive),
    });
    let bytes = serde_json::to_string_pretty(&fixture).unwrap() + "\n";
    if std::env::var_os("T4_U2_WRITE_L_LINE_FIXTURE").is_some() {
        std::fs::write(FIXTURE_PATH, &bytes).unwrap();
        return;
    }
    assert!(FIXTURE == bytes, "the desktop L-line fixture is stale; regenerate it");
}

/// An objective connector replaces a straight span only: a realized arc span is refused.
#[test]
fn a_connector_cannot_replace_a_realized_arc_span() {
    let mut model = authored(true);
    for case in model["load_cases"].as_array_mut().unwrap() {
        case["pressure_regions"] = json!([]);
        case["primitive_loads"] = json!([]);
    }
    let connector = |span: &str, from: &str, to: &str| json!({"id": "component:J-1", "label": "Joint", "kind": "expansion_joint", "node": to, "provenance": PROVENANCE,
        "objective_connector": {"version": "1.0.0", "end_i": {"node_ref": from}, "end_j": {"node_ref": to}, "topology": {"type": "replaces_span", "span_ref": span}}});
    let create = |payload: &Value| intent("insert", "insert_component_symbol", "Component", "component:J-1", "components", "not_present", &payload.to_string(), "none", "dimensionless");
    assert!(codes(&model, &create(&connector("pipe:BEND", "node:B", "node:C"))).contains(&"OP-CREATE-EXPANSION-JOINT-CONNECTOR-INVALID".into()));
    let outcome = apply_operation(&model, &create(&connector("pipe:S1", "node:A", "node:B")), Some(&hash(&model)));
    assert!(outcome.applied_model.is_some(), "{:?}", outcome.diagnostics);
}

/// Model document 0.9.0 (DEL-02-01 Design `model-document-0.9.0.md` §1.4): the bend joins
/// straights that carry the same condition set (or none) and the arc carries it; straights
/// that differ are refused by name.
#[test]
fn insert_bend_at_corner_carries_the_straights_condition_set_or_refuses_by_name() {
    let mut model = corner_document();
    model["condition_sets"] = json!([{"id": "cs:L", "provenance": PROVENANCE}, {"id": "cs:M", "provenance": PROVENANCE}]);
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe["condition_set_ref"] = json!("cs:L");
    }
    let applied = apply(&model, &insert_bend(&model, &insert_bend_payload()));
    let pipes = applied["pipe_segments"].as_array().unwrap();
    assert_eq!(pipes.len(), 3);
    assert!(pipes.iter().all(|pipe| pipe["condition_set_ref"] == "cs:L"), "{pipes:?}");
    let mut differing = model.clone();
    differing["pipe_segments"][1]["condition_set_ref"] = json!("cs:M");
    assert!(codes(&differing, &insert_bend(&differing, &insert_bend_payload())).contains(&"OP-BEND-CONDITION-SET-MISMATCH".into()));
    let mut one_absent = model.clone();
    one_absent["pipe_segments"][1].as_object_mut().unwrap().remove("condition_set_ref");
    assert!(codes(&one_absent, &insert_bend(&one_absent, &insert_bend_payload())).contains(&"OP-BEND-CONDITION-SET-MISMATCH".into()));
}

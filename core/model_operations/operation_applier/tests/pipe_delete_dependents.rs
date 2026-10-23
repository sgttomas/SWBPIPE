//! W-10 §7 (DEL-16-06 Design `pipe-reconnect.md`): `delete_pipe_run` refuses,
//! with named dependents, while any record still names the pipe. It never
//! cascades. Inline invented fixtures from `MODEL_SCHEMA_VERSION`.
use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_operation,
    OutcomeDiagnostic,
};
use open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION;
use serde_json::{json, Value};

const PROV: &str = "invented W-10 delete test input";

fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}
#[allow(clippy::too_many_arguments)]
fn op(id: &str, kind: &str, change_kind: &str, object: &str, target: &str, path: &str, before: &str, after: &str) -> Value {
    json!({"operation_id":format!("op:{id}"),"operation_kind":kind,"operation_status":"proposed","author_type":"user",
      "target":{"object_type":object,"ref":target},
      "change":{"change_id":format!("change:{id}"),"change_kind":change_kind,"field_label":path,"field_path":path,"before":before,"after":after,"unit":"none","dimension":"dimensionless","source_note":"invented explicit test entry"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "rationale":"invented W-10 delete test operation",
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false}})
}
fn delete_pipe(model: &Value, id: &str) -> Value {
    let p = model["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == id).unwrap();
    let before = format!("{}; {}->{}; material={}", p["label"].as_str().unwrap(), p["from"].as_str().unwrap(), p["to"].as_str().unwrap(), p["material"].as_str().unwrap());
    op(&format!("delete-pipe:{id}"), "delete", "delete_pipe_run", "Element", id, "pipe_segments", &before, "not_present")
}
fn delete_component(model: &Value, id: &str) -> Value {
    let c = model["components"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap();
    op(&format!("delete-component:{id}"), "delete", "delete_component", "Component", id, "components", &canonical_json(c), "deleted")
}
fn model() -> Value {
    let section = json!({"outside_diameter": {"value": 0.1, "unit": "m"}, "wall_thickness": {"value": 0.005, "unit": "m"}});
    json!({"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w10-delete", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "nodes": [
            {"id": "node:A", "label": "A", "position": {"x": 0, "y": 0, "z": 0}, "provenance": PROV},
            {"id": "node:B", "label": "B", "position": {"x": 2, "y": 0, "z": 0}, "provenance": PROV},
            {"id": "node:C", "label": "C", "position": {"x": 4, "y": 0, "z": 0}, "provenance": PROV}],
        "pipe_segments": [
            {"id": "pipe:P", "label": "P", "from": "node:A", "to": "node:B", "section": section, "material": "material:steel", "y_reference": {"x": 0, "y": 0, "z": 1}, "provenance": PROV},
            {"id": "pipe:Q", "label": "Q", "from": "node:B", "to": "node:C", "section": section, "material": "material:steel", "y_reference": {"x": 0, "y": 0, "z": 1}, "provenance": PROV}],
        "supports": [], "components": [],
        "materials": [{"id": "material:steel", "elastic_modulus": {"value": 200e9, "unit": "Pa"}, "shear_modulus": {"value": 80e9, "unit": "Pa"}, "provenance": PROV}],
        "load_cases": [{"id": "case:w", "primitive_loads": [], "provenance": PROV}], "combinations": []})
}
/// Validate and apply refuse with OP-PIPE-DELETE-REFERENCED, the model is
/// unchanged and no model returns. Returns the apply diagnostic.
fn refused(model: &Value, intent: &Value) -> OutcomeDiagnostic {
    let original = model.clone();
    let claim = hash(model);
    let validate = validate_operation(model, intent, Some(&claim));
    let apply = apply_operation(model, intent, Some(&claim));
    for outcome in [&validate, &apply] {
        assert!(outcome.applied_model.is_none());
        assert!(outcome.diff_preview.is_empty());
    }
    assert_eq!(model, &original);
    let blocking: Vec<_> = apply.diagnostics.into_iter().filter(|d| d.severity == "blocking").collect();
    assert_eq!(blocking.len(), 1, "{blocking:#?}");
    assert_eq!(blocking[0].code, "OP-PIPE-DELETE-REFERENCED");
    blocking[0].clone()
}

#[test]
fn each_new_dependent_slot_refuses_naming_its_owner_and_slot() {
    let component = |geometry: Value| json!([{"id": "component:dep", "kind": "rigid", "node": "node:C", "geometry": geometry, "provenance": PROV}]);
    let mut cases: Vec<(&str, &str, Value)> = Vec::new();
    for key in ["bend_pipe_ref", "branch_header_pipe_ref", "branch_branch_pipe_ref", "rigid_pipe_ref", "expansion_joint_pipe_ref"] {
        let mut m = model();
        m["components"] = component(json!({key: "pipe:P"}));
        cases.push(("component:dep", Box::leak(format!("geometry.{key}").into_boxed_str()), m));
    }
    let mut m = model();
    m["components"] = json!([{"id": "component:joint", "kind": "expansion_joint", "node": "node:C", "objective_connector": {"topology": {"type": "replaces_span", "span_ref": "pipe:P"}}, "provenance": PROV}]);
    cases.push(("component:joint", "objective_connector.topology.span_ref", m));
    let mut m = model();
    m["load_cases"][0]["equivalent_static"] = json!({"wind": {"exposed_pipe_refs": ["pipe:P"]}});
    cases.push(("case:w", "equivalent_static.wind.exposed_pipe_refs[0]", m));
    let mut m = model();
    m["load_cases"][0]["equivalent_static"] = json!({"wind": {"exposed_spans": [{"pipe_ref": "pipe:P", "start_fraction": {"value": 0.1, "unit": "1"}, "end_fraction": {"value": 0.4, "unit": "1"}}]}});
    cases.push(("case:w", "equivalent_static.wind.exposed_spans[0].pipe_ref", m));
    let mut m = model();
    m["extension"] = json!({"required_input_refs": [{"id": "pipe:P"}]});
    cases.push(("extension", "required_input_refs[0].id", m));
    for (owner, slot, m) in cases {
        let d = refused(&m, &delete_pipe(&m, "pipe:P"));
        assert_eq!(d.affected_refs, vec!["pipe:P".to_string(), owner.to_string()], "{slot}");
        assert_eq!(d.message, format!("Pipe segment pipe:P is still named by: {owner} ({slot})."));
        assert_eq!(d.remediation, "Remove or retarget each dependent first; deletion never cascades.");
    }
}

#[test]
fn existing_refs_come_first_and_each_owner_is_reported_once() {
    let mut m = model();
    m["load_cases"][0]["primitive_loads"] = json!([{"id": "load:w", "category": "distributed_force", "target": {"type": "element", "pipe": "pipe:P"}, "direction": "global_z", "magnitude": {"value": 1.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": PROV}]);
    m["load_cases"][0]["analysis_state"] = json!({"element_states": [{"pipe_ref": "pipe:P"}]});
    m["components"] = json!([
        {"id": "component:z", "kind": "tee", "node": "node:B", "geometry": {"branch_header_pipe_ref": "pipe:P", "branch_branch_pipe_ref": "pipe:P"}, "provenance": PROV},
        {"id": "component:a", "kind": "bend", "node": "node:A", "geometry": {"bend_pipe_ref": "pipe:P"}, "provenance": PROV}]);
    let d = refused(&m, &delete_pipe(&m, "pipe:P"));
    assert_eq!(d.affected_refs, vec!["pipe:P", "load:w", "case:w.analysis_state.element_states.0.pipe_ref", "component:a", "component:z"]);
    assert_eq!(d.message, "Pipe segment pipe:P is still named by: load:w, case:w.analysis_state.element_states.0.pipe_ref, component:a (geometry.bend_pipe_ref), component:z (geometry.branch_branch_pipe_ref), component:z (geometry.branch_header_pipe_ref).");
}

#[test]
fn todays_refusals_and_messages_hold_when_no_new_dependent_exists() {
    let mut m = model();
    m["load_cases"][0]["primitive_loads"] = json!([{"id": "load:w", "category": "distributed_force", "target": {"type": "element", "pipe": "pipe:P"}, "direction": "global_z", "magnitude": {"value": 1.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": PROV}]);
    let d = refused(&m, &delete_pipe(&m, "pipe:P"));
    assert_eq!(d.affected_refs, vec!["pipe:P", "load:w"]);
    assert_eq!(d.message, "Pipe segment `pipe:P` is still referenced by loads, pressure regions, or nonlinear supports: load:w.");
}

#[test]
fn a_key_spelled_as_path_syntax_is_a_named_dependent() {
    // The literal key is one key: it is not mistaken for the real
    // primitive_loads[0] (which targets Q), so deleting P refuses.
    let mut m = model();
    m["load_cases"][0]["primitive_loads"] = json!([{"id": "load:q", "category": "distributed_force", "target": {"type": "element", "pipe": "pipe:Q"}, "direction": "global_z", "magnitude": {"value": 1.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": PROV}]);
    m["load_cases"][0]["primitive_loads[0].target"] = json!({"pipe": "pipe:P"});
    let d = refused(&m, &delete_pipe(&m, "pipe:P"));
    assert_eq!(d.affected_refs, vec!["pipe:P", "case:w"]);
    assert_eq!(d.message, "Pipe segment pipe:P is still named by: case:w ([\"primitive_loads[0].target\"].pipe).");
}

#[test]
fn an_object_key_spelled_star_is_a_named_dependent() {
    let mut m = model();
    m["load_cases"][0]["primitive_loads"] = json!({"*": {"id": "load:star", "category": "distributed_force", "target": {"type": "element", "pipe": "pipe:P"}, "direction": "global_z", "magnitude": {"value": 1.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": PROV}});
    let d = refused(&m, &delete_pipe(&m, "pipe:P"));
    assert_eq!(d.affected_refs, vec!["pipe:P", "case:w"]);
    assert_eq!(d.message, "Pipe segment pipe:P is still named by: case:w (primitive_loads, not an array), case:w (primitive_loads[\"*\"].target.pipe).");
}

#[test]
fn a_collection_that_is_not_an_array_is_a_named_dependent() {
    // components as an object map naming the pipe.
    let mut m = model();
    m["components"] = json!({"*": {"geometry": {"bend_pipe_ref": "pipe:P"}}});
    let d = refused(&m, &delete_pipe(&m, "pipe:P"));
    assert_eq!(d.affected_refs, vec!["pipe:P", "components"]);
    assert!(d.message.contains("components (not an array)"), "{}", d.message);
    // pressure_regions as an object: unscannable, so the delete refuses even
    // where no readable record names the pipe.
    let mut m = model();
    m["load_cases"][0]["pressure_regions"] = json!({"region:q": {"member_pipe_ids": ["pipe:Q"]}});
    let d = refused(&m, &delete_pipe(&m, "pipe:P"));
    assert_eq!(d.affected_refs, vec!["pipe:P", "case:w"]);
    assert_eq!(d.message, "Pipe segment pipe:P is still named by: case:w (pressure_regions, not an array).");
}

#[test]
fn stored_diagnostics_and_text_metadata_do_not_block() {
    let mut m = model();
    m["diagnostics"] = json!([{"id": "diagnostic:stored", "code": "X", "affected_refs": ["pipe:P"]}]);
    m["nodes"][0]["label"] = json!("pipe:P");
    let outcome = apply_operation(&m, &delete_pipe(&m, "pipe:P"), Some(&hash(&m)));
    let applied = outcome.applied_model.unwrap_or_else(|| panic!("{:#?}", outcome.diagnostics));
    assert!(applied["pipe_segments"].as_array().unwrap().iter().all(|p| p["id"] != "pipe:P"));
}

#[test]
fn delete_component_then_delete_pipe_applies_atomically() {
    let mut m = model();
    m["components"] = json!([{"id": "component:bend", "kind": "bend", "node": "node:C", "geometry": {"bend_pipe_ref": "pipe:P"}, "provenance": PROV}]);
    let mut without = m.clone();
    without["components"] = json!([]);
    let batch = json!({"batch_id": "batch:delete", "operations": [delete_component(&m, "component:bend"), delete_pipe(&without, "pipe:P")]});
    let outcome = apply_operation_batch(&m, &batch, Some(&hash(&m)));
    assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{outcome:#}");
    let applied = &outcome["applied_model"];
    assert_eq!(applied["components"], json!([]));
    assert_eq!(applied["pipe_segments"].as_array().unwrap().len(), 1);
    // The delete alone, first, refuses: no cascade.
    let alone = json!({"batch_id": "batch:alone", "operations": [delete_pipe(&m, "pipe:P")]});
    let outcome = apply_operation_batch(&m, &alone, Some(&hash(&m)));
    assert!(outcome["applied_model"].is_null());
}

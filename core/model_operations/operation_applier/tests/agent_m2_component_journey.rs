//! RT-E1 milestone 2 (DEL-16-06 Design `embedded-agent-host.md`, catalog
//! version 4): a pressure-free model with one of each component family and
//! six support families, built from the desktop's blank document by sequential
//! embedded-agent proposals, each applied before the next, then corrected by an
//! agent edit and two agent deletions. The person then enters every bend and
//! tee flexibility factor k and SIF as their own operations (owner ruling: users
//! enter their own k and SIF), and the model solves in both solver modes.
//!
//! The fixture's corrections name their `before` texts by placeholder; this
//! test derives them from the applied document as the applier compares them,
//! and the desktop test (`embeddedAgentM2.test.ts`) takes them from the
//! session's `swbpipe_read_records` producer instead. This is a build-and-solve
//! check on invented data, not a numerical reference. All data are invented.
use open_pipe_stress_operation_applier::{apply_operation, apply_operation_batch, canonical_json, sha256_hex};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Map, Value};

const FIXTURE: &str = include_str!("../../../../fixtures/model_operations/agent_m2_component_journey.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const SUPPORT_CONFIGURATION: [&str; 6] = ["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"];

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("fixture parses")
}
fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}
fn record<'a>(model: &'a Value, collection: &str, id: &str) -> &'a Value {
    model[collection].as_array().unwrap().iter().find(|r| r["id"] == id).unwrap_or_else(|| panic!("no {collection} {id}"))
}

/// The applier's `before` for an admitted correction, from the current document.
fn before_text(model: &Value, placeholder: &str) -> String {
    let parts: Vec<&str> = placeholder.splitn(4, ':').collect();
    assert_eq!(parts[0], "BEFORE", "{placeholder}");
    let (id, key) = (format!("{}:{}", parts[1], parts[2]), parts[3]);
    match (parts[1], key) {
        ("support", "configuration") => {
            let support = record(model, "supports", &id);
            let projection: Map<String, Value> = SUPPORT_CONFIGURATION.iter().filter_map(|k| support.get(*k).map(|v| ((*k).into(), v.clone()))).collect();
            canonical_json(&Value::Object(projection))
        }
        ("support", "delete") => record(model, "supports", &id)["label"].as_str().unwrap().trim().to_string(),
        ("component", "delete") => canonical_json(record(model, "components", &id)),
        other => panic!("unsupported placeholder {other:?}"),
    }
}

fn stamped(operation: &Value, host: &Value, model: &Value) -> Value {
    let mut native = operation.clone();
    if let Some(placeholder) = native["change"]["before"].as_str().filter(|b| b.starts_with("BEFORE:")).map(str::to_owned) {
        native["change"]["before"] = json!(before_text(model, &placeholder));
    }
    let object = native.as_object_mut().unwrap();
    let mut fields = host["stamp"].as_object().unwrap().clone();
    fields.insert("queue_id".into(), host["proposalHandle"].clone());
    fields.insert("source".into(), json!({"source_ref": host["requestRef"], "source_channel": "embedded_agent", "source_role": "agent"}));
    for (key, value) in fields {
        assert!(object.insert(key.clone(), value).is_none(), "agent-form operations carry no host field `{key}`");
    }
    native
}

/// Apply the named phases as agent batches at the then-current basis, starting
/// from `model`.
fn proposals(mut model: Value, phases: &[&str]) -> Value {
    let f = fixture();
    for phase in phases {
        let proposal = f["proposals"].as_array().unwrap().iter().find(|p| p["phase"] == *phase).unwrap_or_else(|| panic!("no phase {phase}"));
        let operations: Vec<Value> = proposal["operations"].as_array().unwrap().iter().map(|o| stamped(o, &f["host"], &model)).collect();
        assert!(operations.len() <= 16, "{phase}: one proposal stays within the default operation cap");
        let claim = hash(&model);
        let batch = json!({"batch_id": format!("agent-{phase}"), "operations": operations, "source_model_hash": claim["value"]});
        let outcome = apply_operation_batch(&model, &batch, Some(&claim));
        assert_eq!(outcome["validation"]["batch_validation_status"], "passed", "{phase}: {:#}", outcome["diagnostics"]);
        model = outcome["applied_model"].clone();
    }
    model
}
const ALL_PHASES: [&str; 5] = ["geometry", "components", "supports", "loads", "corrections"];
fn built() -> Value {
    proposals(fixture()["blank"]["model"].clone(), &ALL_PHASES)
}

#[allow(clippy::too_many_arguments)]
fn user_intent(kind: &str, change_kind: &str, object: &str, id: &str, path: &str, before: &str, after: &str, unit: &str, dimension: &str, author: &str) -> Value {
    json!({"operation_id":format!("op:m2-person:{id}:{path}"),"operation_kind":kind,"operation_status":"proposed","author_type":author,
      "target":{"object_type":object,"ref":id},
      "change":{"change_id":format!("change:m2-person:{id}:{path}"),"change_kind":change_kind,"field_label":path,"field_path":path,"before":before,"after":after,"unit":unit,"dimension":dimension,"source_note":"entered by the person"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "rationale":"the person's own modifier entry",
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false}})
}
fn apply(model: &Value, intent: &Value) -> Value {
    let outcome = apply_operation(model, intent, Some(&hash(model)));
    outcome.applied_model.unwrap_or_else(|| panic!("{:?}", outcome.diagnostics))
}
fn corner_before(model: &Value, node: &str) -> String {
    let pipes: Vec<&Value> = model["pipe_segments"].as_array().unwrap().iter().filter(|p| p["from"] == node || p["to"] == node).collect();
    let markers: Vec<&Value> = model["components"].as_array().unwrap().iter().filter(|c| (c["kind"] == "bend" || c["kind"] == "elbow") && c["node"] == node).collect();
    canonical_json(&json!({"node": record(model, "nodes", node), "pipes": pipes, "markers": markers}))
}

/// The person's entries: Insert bend at corner converting the agent's marker
/// (k and its source), then the realized bend's in-plane and out-of-plane SIF
/// pair (a realized arc never applies the scalar SIF), and the tee's k, SIFs
/// and source.
fn person_entries(mut model: Value) -> Value {
    let p = fixture()["person"].clone();
    let source = p["source"].as_str().unwrap();
    let corner = p["bendCorner"].as_str().unwrap();
    let payload = json!({"radius": p["bendRadius"], "flexibility_factor": p["bendK"], "flexibility_source_reference": source,
        "new_nodes": [{"id": "node:n3-b", "label": "Bend start", "provenance": source}, {"id": "node:n3-c", "label": "Bend end", "provenance": source}],
        "new_pipe": {"id": "pipe:n3-bend", "label": "Bend arc", "provenance": source},
        "component": {"convert": p["bendMarker"]}});
    let before = corner_before(&model, corner);
    model = apply(&model, &user_intent("modify", "insert_bend_at_corner", "Node", corner, "bend_at_corner", &before, &payload.to_string(), "none", "dimensionless", "user"));
    let bend = p["bendMarker"].as_str().unwrap();
    let realized = record(&model, "components", bend).clone();
    let mut realization = Map::new();
    realization.insert("solver_consumption".into(), realized["mechanics_interface"]["solver_consumption"].clone());
    for key in ["flexibility_factor_user_value", "source_reference"] {
        realization.insert(key.into(), realized["modifiers"][key].clone());
    }
    let before = canonical_json(&Value::Object(realization.clone()));
    realization.insert("sif_in_plane_user_value".into(), p["bendSifInPlane"].clone());
    realization.insert("sif_out_of_plane_user_value".into(), p["bendSifOutOfPlane"].clone());
    model = apply(&model, &user_intent("modify", "set_field", "Component", bend, "bend_realization", &before, &canonical_json(&Value::Object(realization)), "none", "dimensionless", "user"));
    let tee = p["tee"].as_str().unwrap();
    for (path, value) in [("modifiers.flexibility_factor_user_value.value", &p["teeK"]), ("modifiers.branch_header_sif_user_value.value", &p["teeHeaderSif"]), ("modifiers.branch_branch_sif_user_value.value", &p["teeBranchSif"])] {
        model = apply(&model, &user_intent("modify", "set_field", "Component", tee, path, "TBD", &value.to_string(), "none", "dimensionless", "user"));
    }
    apply(&model, &user_intent("modify", "set_field", "Component", tee, "modifiers.source_reference", "TBD", source, "none", "dimensionless", "user"))
}

fn diagnostics(model: &Value, mode: PreviewSolverMode) -> Vec<(String, String)> {
    let out = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), mode).unwrap_or_else(|e| panic!("typed boundary rejected the document: {e}"));
    let blocking: Vec<String> = out.diagnostics.iter().filter(|d| d.severity == "blocking" || d.severity == "failure").map(|d| format!("{}: {}", d.code, d.message)).collect();
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{}: {blocking:?}", mode.as_str());
    assert!(blocking.is_empty(), "{}: {blocking:?}", mode.as_str());
    out.diagnostics.iter().map(|d| (d.code.clone(), d.severity.clone())).collect()
}

#[test]
fn sequential_agent_proposals_build_every_component_and_support_family() {
    let model = built();
    let kinds: Vec<&str> = model["components"].as_array().unwrap().iter().map(|c| c["kind"].as_str().unwrap()).collect();
    assert_eq!(kinds, ["bend", "tee", "reducer", "valve", "expansion_joint"], "the flange was removed by the agent's correction");
    let families: Vec<&str> = model["supports"].as_array().unwrap().iter().map(|s| s["family"].as_str().unwrap()).collect();
    assert_eq!(families, ["anchor", "anchor", "guide", "vertical_support", "nonlinear", "variable_spring_hanger", "constant_effort_support"]);
    assert_eq!(record(&model, "supports", "support:n4-guide")["restraints"], json!(["UX"]));
    assert_eq!(record(&model, "components", "component:reducer-n4")["geometry"]["eccentric_offset"], json!({"x": 0.0, "y": 0.0, "z": -0.025, "unit": "m"}));
    // The agent's bend and tee carry no modifiers: nothing it sent became k or a SIF.
    for id in ["component:bend-n3", "component:tee-n2"] {
        assert!(record(&model, "components", id).get("modifiers").is_none(), "{id}");
    }
}

#[test]
fn the_agent_cannot_write_k_and_the_person_enters_it_before_solving() {
    // Agent-only build: it solves, and the solver warns that the person's
    // modifiers are missing; it never refuses for them.
    let model = built();
    let agent_k = user_intent("modify", "set_field", "Component", "component:tee-n2", "modifiers.flexibility_factor_user_value.value", "TBD", "{\"value\":1,\"unit\":\"none\"}", "none", "dimensionless", "agent");
    let refused = apply_operation(&model, &agent_k, Some(&hash(&model)));
    assert!(refused.applied_model.is_none());
    assert!(refused.diagnostics.iter().any(|d| d.code == "OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED"));
    // RT-E1 M2-D1: nor any other user-entered modifier, on any route an agent
    // operation could use; the person's own entries of the same members apply.
    for (path, value) in [
        ("modifiers.sif_user_value.value", "{\"value\":1.5,\"unit\":\"none\"}"),
        ("modifiers.source_reference", "agent-stated source"),
    ] {
        let agent = user_intent("modify", "set_field", "Component", "component:bend-n3", path, "TBD", value, "none", "dimensionless", "agent");
        let outcome = apply_operation(&model, &agent, Some(&hash(&model)));
        assert!(outcome.applied_model.is_none(), "{path}");
        assert!(outcome.diagnostics.iter().any(|d| d.code == "OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED"), "{path}");
        let person = user_intent("modify", "set_field", "Component", "component:bend-n3", path, "TBD", value, "none", "dimensionless", "user");
        assert!(apply_operation(&model, &person, Some(&hash(&model))).applied_model.is_some(), "{path}");
    }
    for path in ["modifiers.branch_header_sif_user_value.value", "modifiers.branch_branch_sif_user_value.value"] {
        let agent = user_intent("modify", "set_field", "Component", "component:tee-n2", path, "TBD", "{\"value\":1.4,\"unit\":\"none\"}", "none", "dimensionless", "agent");
        assert!(apply_operation(&model, &agent, Some(&hash(&model))).diagnostics.iter().any(|d| d.code == "OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED"), "{path}");
    }
    let scaling = user_intent("modify", "set_field", "Component", "component:valve-n5", "modifiers.stiffness_scaling_user_value.value", "TBD", "{\"value\":15,\"unit\":\"none\"}", "none", "dimensionless", "agent");
    assert!(apply_operation(&model, &scaling, Some(&hash(&model))).diagnostics.iter().any(|d| d.code == "OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED"));
    let mut valve: Value = serde_json::from_str(fixture()["proposals"][1]["operations"][3]["change"]["after"].as_str().unwrap()).unwrap();
    valve["id"] = json!("component:valve-agent");
    valve["modifiers"] = json!({"stiffness_scaling_user_value": {"value": 15, "unit": "none"}, "source_reference": "agent-stated"});
    let insert = user_intent("insert", "insert_component_symbol", "Component", "component:valve-agent", "components", "not_present", &valve.to_string(), "none", "dimensionless", "agent");
    assert!(apply_operation(&model, &insert, Some(&hash(&model))).diagnostics.iter().any(|d| d.code == "OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED"));
    // The agent's bend is a geometry-only marker: the pressure-free route
    // solves it as a straight chord and says so (DEL-03-03).
    for mode in MODES {
        let before = diagnostics(&model, mode);
        for code in ["BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD", "BRANCH_USER_MODIFIER_INPUT_MISSING"] {
            assert!(before.iter().any(|(c, s)| c == code && s == "warning"), "{} without the person's entries: {code}", mode.as_str());
        }
    }
    // The journey: the person realizes the bend and enters every k and SIF
    // right after the components proposal, before anything attaches to the
    // corner, then the agent continues.
    let blank = fixture()["blank"]["model"].clone();
    let entered = person_entries(proposals(blank, &["geometry", "components"]));
    let finished = proposals(entered, &["supports", "loads", "corrections"]);
    let bend = record(&finished, "components", "component:bend-n3");
    assert_eq!(bend["mechanics_interface"]["solver_consumption"], "curved_bend_macro_element");
    assert_eq!(bend["modifiers"]["flexibility_factor_user_value"]["value"], json!(1.8));
    for mode in MODES {
        let after = diagnostics(&finished, mode);
        for code in ["BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD", "BEND_USER_MODIFIER_INPUT_MISSING", "BRANCH_USER_MODIFIER_INPUT_MISSING"] {
            assert!(!after.iter().any(|(c, _)| c == code), "{} after the person's entries: {code}", mode.as_str());
        }
    }
}

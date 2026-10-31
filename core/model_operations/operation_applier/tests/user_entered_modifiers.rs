//! RT-E1 M2-D1 (owner ruling: users enter their own k and SIF; DEL-16-06
//! Design `embedded-agent-host.md`): a component's user-entered modifiers are
//! written only by an operation authored by the user. An agent-authored
//! operation is refused whatever route it takes: a component field edit, a
//! component created with modifiers, or a library record copying modifier
//! members. k keeps its own refusal; every other modifier member (SIFs,
//! stiffness scaling and stiffness, and the modifier source) is refused with
//! `OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED`. All data are invented.
use open_pipe_stress_operation_applier::{apply_operation, canonical_json, sha256_hex};
use serde_json::{json, Value};

const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
const MODIFIER: &str = "OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED";
const K: &str = "OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED";

fn model() -> Value {
    serde_json::from_str(DEMO).unwrap()
}
fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}
#[allow(clippy::too_many_arguments)]
fn intent(author: &str, kind: &str, change_kind: &str, object: &str, id: &str, path: &str, before: &str, after: &str) -> Value {
    json!({"operation_id":format!("op:modifier:{id}:{path}"),"operation_kind":kind,"operation_status":"proposed","author_type":author,
      "target":{"object_type":object,"ref":id},
      "change":{"change_id":format!("change:modifier:{id}:{path}"),"change_kind":change_kind,"field_label":path,"field_path":path,"before":before,"after":after,"unit":"none","dimension":"dimensionless","source_note":"invented"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "rationale":"invented",
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false}})
}
fn codes(model: &Value, intent: &Value) -> Vec<String> {
    let outcome = apply_operation(model, intent, Some(&hash(model)));
    assert!(outcome.applied_model.is_none(), "applied unexpectedly: {intent}");
    outcome.diagnostics.iter().map(|d| d.code.clone()).collect()
}

#[test]
fn an_agent_cannot_write_any_component_modifier_by_field_edit() {
    let m = model();
    for (id, path, after) in [
        ("component:C-110", "modifiers.sif_user_value.value", r#"{"value":1.5,"unit":"none"}"#),
        ("component:C-110", "modifiers.source_reference", "agent-stated source"),
        ("component:C-130", "modifiers.stiffness_scaling_user_value.value", r#"{"value":12,"unit":"none"}"#),
    ] {
        let found = codes(&m, &intent("agent", "modify", "set_field", "Component", id, path, "TBD", after));
        assert!(found.contains(&MODIFIER.to_string()), "{path}: {found:?}");
    }
    // k keeps its own refusal and code.
    let found = codes(&m, &intent("agent", "modify", "set_field", "Component", "component:C-110", "modifiers.flexibility_factor_user_value.value", "TBD", r#"{"value":2,"unit":"none"}"#));
    assert!(found.contains(&K.to_string()) && !found.contains(&MODIFIER.to_string()), "{found:?}");
}

#[test]
fn an_agent_cannot_create_a_component_carrying_modifiers() {
    let m = model();
    let valve = m["components"].as_array().unwrap().iter().find(|c| c["id"] == "component:C-130").unwrap().clone();
    let mut created = valve.clone();
    created["id"] = json!("component:C-131");
    let found = codes(&m, &intent("agent", "insert", "insert_component_symbol", "Component", "component:C-131", "components", "not_present", &created.to_string()));
    assert!(found.contains(&MODIFIER.to_string()), "{found:?}");
}

#[test]
fn an_agent_cannot_copy_modifier_members_from_a_library_record() {
    let m = model();
    let after = json!({"modifiers.sif_user_value": {"value": 1.4, "unit": "1"}}).to_string();
    let found = codes(&m, &intent("agent", "modify", "apply_library_record", "Component", "component:C-110", "components", "TBD", &after));
    assert!(found.contains(&MODIFIER.to_string()), "{found:?}");
}

#[test]
fn the_refusal_reads_only_the_author_claim() {
    // The applier cannot verify who authored an operation: the same edit with
    // `author_type: user` is not refused for its author. Routes that carry
    // agent or imported content must therefore stamp `agent` (DEL-16-06,
    // DEL-16-03); the bundled webview is trusted not to claim `user` falsely.
    let m = model();
    let user = intent("user", "modify", "set_field", "Component", "component:C-110", "modifiers.source_reference", "TBD", "person-entered source");
    let outcome = apply_operation(&m, &user, Some(&hash(&m)));
    assert!(!outcome.diagnostics.iter().any(|d| d.code == MODIFIER || d.code == K), "{:?}", outcome.diagnostics);
}

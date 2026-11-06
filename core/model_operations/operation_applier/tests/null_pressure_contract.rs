//! Agent 0's decision (2026-11-05, S-K1 applier slice): a present `pressure_contract: null`
//! is refused by name by the one shared document validator (`MODEL-SLOT-SHAPE` at
//! `pressure_contract`), as the solve refuses it at decode. Absence still means the
//! pressure-free route; a present current or retired object keeps its designed behaviour.
//! The applier validates every resulting candidate at its common post-apply boundary, so
//! an operation that replaces the null member with a valid object still repairs it.
use open_pipe_stress_operation_applier::document_validator::SLOT_SHAPE;
use open_pipe_stress_operation_applier::{apply_operation, canonical_json, sha256_hex, validate_model_document};
use serde_json::{json, Value};

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../fixtures/model_operations/exact_pressure_authoring_model.json")).unwrap()
}
fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}
fn intent(object: &str, id: &str, path: &str, before: &Value, after: &Value) -> Value {
    raw_intent(object, id, path, &canonical_json(before), &canonical_json(after))
}
fn raw_intent(object: &str, id: &str, path: &str, before: &str, after: &str) -> Value {
    json!({"operation_id":format!("op:null:{id}:{path}"),"operation_kind":"modify","operation_status":"proposed","author_type":"user","target":{"object_type":object,"ref":id},
      "change":{"change_id":format!("change:null:{id}:{path}"),"change_kind":"set_field","field_label":path,"field_path":path,"before":before,"after":after,"unit":"none","dimension":"dimensionless","source_note":"Synthetic explicitly entered fixture values"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "rationale":"Synthetic explicit user authoring",
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false}})
}
fn null_shape(model: &Value) -> bool {
    validate_model_document(model).iter().any(|d| d.code == SLOT_SHAPE && d.path == "pressure_contract" && d.is_blocking())
}
fn profile(model: &Value) -> Value {
    json!({"schema_version": model["schema_version"], "pressure_contract": model["pressure_contract"]})
}

#[test]
fn absence_null_current_and_retired_contracts() {
    let current = fixture();
    assert!(!null_shape(&current), "a valid current object");
    let mut retired = fixture();
    retired["pressure_contract"] = json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"});
    assert!(!null_shape(&retired), "a retired object keeps its own refusals, never the null shape");
    let mut absent = fixture();
    absent.as_object_mut().unwrap().remove("pressure_contract");
    assert!(!null_shape(&absent), "absence means the pressure-free route");
    let mut null = fixture();
    null["pressure_contract"] = Value::Null;
    assert!(null_shape(&null), "a present null is refused by name");
    let finding = validate_model_document(&null).into_iter().find(|d| d.path == "pressure_contract").unwrap();
    assert_eq!(finding.code, SLOT_SHAPE);
    // The webview open check (`nullPressureContract.engine.test.ts`) pins the same text.
    assert_eq!(finding.message, "pressure_contract is null; an authored object is required, and absence means the pressure-free route.");
    assert!(finding.is_blocking());
}

#[test]
fn a_candidate_that_keeps_null_is_refused_and_nothing_is_applied() {
    let mut model = fixture();
    model["pressure_contract"] = Value::Null;
    let node = model["nodes"][0].clone();
    let id = node["id"].as_str().unwrap().to_string();
    let op = raw_intent("Node", &id, "label", node["label"].as_str().unwrap(), "Renamed");
    // The same edit applies on the absent-contract (pressure-free) document.
    let mut absent = model.clone();
    absent.as_object_mut().unwrap().remove("pressure_contract");
    assert!(apply_operation(&absent, &op, Some(&hash(&absent))).applied_model.is_some());
    let outcome = apply_operation(&model, &op, Some(&hash(&model)));
    assert!(outcome.applied_model.is_none(), "{:?}", outcome.diagnostics);
    assert!(outcome.diagnostics.iter().any(|d| d.code == SLOT_SHAPE), "{:?}", outcome.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
}

/// The existing repair path (the desktop's B7b activation, `b7bAdmission.test.tsx`): the
/// pressure profile operation replaces the null member with a valid object, and the
/// resulting candidate validates, so it applies.
#[test]
fn the_pressure_profile_operation_repairs_a_null_contract() {
    let valid = fixture();
    let mut model = valid.clone();
    model["pressure_contract"] = Value::Null;
    let op = intent("Model", model["project"]["id"].as_str().unwrap(), "pressure_profile", &profile(&model), &profile(&valid));
    let outcome = apply_operation(&model, &op, Some(&hash(&model)));
    let applied = outcome.applied_model.unwrap_or_else(|| panic!("{:?}", outcome.diagnostics.iter().map(|d| (&d.code, &d.message)).collect::<Vec<_>>()));
    assert_eq!(applied["pressure_contract"], valid["pressure_contract"]);
    assert!(!null_shape(&applied));
}

//! W-13 §5.3 at apply: the one document validator checks every applied
//! result. An operation whose result carries a malformed model-data slot is
//! refused; warnings ride along without blocking; a document carrying every
//! slot survives an unrelated operation and its undo with an equal model
//! hash. Invented data only.
use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, OperationOutcome,
};
use serde_json::{json, Value};

fn every_slot() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/model_document/invented_every_slot_model.json"
    ))
    .unwrap()
}

fn model_hash(model: &Value) -> String {
    format!("sha256:{}", sha256_hex(&canonical_json(model)))
}

fn claim(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":model_hash(model)})
}

#[allow(clippy::too_many_arguments)]
fn intent(id: &str, kind: &str, object_type: &str, target: &str, change_kind: &str, path: &str, before: &str, after: &str) -> Value {
    json!({
        "operation_id": format!("operation:{id}"), "operation_kind": kind, "operation_status": "proposed",
        "author_type": "user", "target": {"object_type": object_type, "ref": target},
        "change": {"change_id": format!("change:{id}"), "change_kind": change_kind, "field_label": path,
                   "field_path": path, "before": before, "after": after, "unit": "none",
                   "dimension": "dimensionless", "source_note": "invented test input"},
        "validation": {"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run",
                       "diff_preview_status":"not_generated","application_status":"not_applied"},
        "audit_boundary": {"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,
                           "requires_user_acceptance":true,"mutates_accepted_model_state":false},
        "professional_boundary": {"human_review_required":true,"software_makes_compliance_claim":false,
            "software_makes_certification_claim":false,"software_makes_sealing_claim":false,
            "software_makes_approval_claim":false,"software_makes_authentication_claim":false},
        "rationale": "Invented validator test"
    })
}

fn rename_node(before: &str, after: &str) -> Value {
    intent("rename", "modify", "Node", "node:N-2", "set_field", "label", before, after)
}

fn delete_pipe(target: &str, display: &str) -> Value {
    intent("delete-pipe", "delete", "Element", target, "delete_pipe_run", "pipe_segments", display, "not_present")
}

fn codes(outcome: &OperationOutcome) -> Vec<(&str, &str)> {
    outcome.diagnostics.iter().map(|item| (item.code.as_str(), item.severity.as_str())).collect()
}

/// An operation whose result would carry a malformed model-data slot is refused with the
/// validator's named diagnostic and no document returns. Here the line already names a pipe the
/// document lacks, so any applied result is malformed. (Deleting a pipe that a line or a
/// constraint still names is refused earlier, by W-10's dependent-reference check, before any
/// result exists; S2 makes pipe deletion remove the pipe from its line, §3.2 and §6.)
#[test]
fn an_operation_whose_result_is_malformed_is_refused() {
    let mut model = every_slot();
    model["lines"][0]["pipe_refs"] = json!(["pipe:P-absent"]);
    let before = model.clone();
    let outcome = apply_operation(&model, &rename_node("Elbow", "Elbow (renamed)"), None);
    assert_eq!(model, before, "the input is never mutated");
    assert!(outcome.applied_model.is_none());
    assert!(outcome.applied_model_backend_hash.is_none());
    assert_eq!(outcome.validation.application_status, "blocked");
    assert_eq!(outcome.validation.schema_validation, "blocked");
    assert_eq!(outcome.validation.diff_preview_status, "blocked_by_validation");
    let line = outcome.diagnostics.iter().find(|item| item.code == "MODEL-LINE-PIPE-MISSING").expect("named refusal");
    assert_eq!(line.severity, "blocking");
    assert!(line.message.contains("(at lines[0].pipe_refs[0])"), "{}", line.message);
    assert_eq!(line.affected_refs, vec!["line:L-100".to_string(), "pipe:P-absent".to_string()]);
    assert_eq!(outcome.acceptance.persistence_status, "not_applicable_no_application");

    // The same malformed result inside an atomic batch rolls the whole batch back.
    let batch = json!({"batch_id": "batch:validator", "operations": [
        rename_node("Elbow", "Elbow (renamed)"),
    ]});
    let outcome = apply_operation_batch(&model, &batch, Some(&claim(&model)));
    assert!(outcome["applied_model"].is_null());
    assert_eq!(outcome["validation"]["application_status"], json!("blocked"));
    assert_eq!(outcome["simulation_disposition"], json!("rolled_back_no_model_published"));
    assert!(outcome["diagnostics"].as_array().unwrap().iter().any(|item| item["code"] == json!("MODEL-LINE-PIPE-MISSING")));
}

/// W-10 and the validator together: deleting a pipe that the line and a constraint still name
/// is refused by W-10's dependent-reference check, which names both; a pipe nothing names
/// deletes normally, and the constraint that names the other pipe is untouched.
#[test]
fn pipe_deletion_with_model_data_references() {
    let model = every_slot();
    let outcome = apply_operation(&model, &delete_pipe("pipe:P-1", "Run; node:N-1->node:N-2; material=material:steel"), None);
    assert!(outcome.applied_model.is_none());
    assert_eq!(outcome.validation.application_status, "blocked");
    let refused = outcome.diagnostics.iter().find(|item| item.code == "OP-PIPE-DELETE-REFERENCED").expect("dependent refusal");
    assert!(refused.message.contains("constraints[0]") && refused.message.contains("line:L-100"), "{}", refused.message);

    let outcome = apply_operation(&model, &delete_pipe("pipe:P-2", "Riser; node:N-2->node:N-3; material=material:steel"), None);
    assert_eq!(outcome.validation.application_status, "applied_to_session_model", "{:?}", codes(&outcome));
    assert!(codes(&outcome).is_empty(), "{:?}", codes(&outcome));
}

/// Warnings surface on the outcome without blocking.
#[test]
fn warnings_ride_along_without_blocking() {
    let mut model = every_slot();
    model["unresolved_assumptions"][0]["affected_refs"] = json!(["support:S-absent"]);
    model["components"][0]["provenance_record"]["review_status"] = json!("quarantined");
    let outcome = apply_operation(&model, &rename_node("Elbow", "Elbow (renamed)"), None);
    assert_eq!(outcome.validation.application_status, "applied_to_session_model");
    assert_eq!(outcome.validation.schema_validation, "passed");
    assert_eq!(codes(&outcome), vec![
        ("MODEL-ASSUMPTION-SUBJECT-MISSING", "warning"),
        ("MODEL-PROVENANCE-QUARANTINE", "warning"),
    ]);
    assert_eq!(outcome.applied_model.unwrap()["nodes"][1]["label"], json!("Elbow (renamed)"));
}

/// A document carrying every slot survives an unrelated operation and its
/// undo (the inverse operation) with an equal model hash and equal content,
/// on the single route and the batch route.
#[test]
fn every_slot_document_survives_an_operation_and_its_undo() {
    let model = every_slot();
    let original_hash = model_hash(&model);

    let edit = apply_operation(&model, &rename_node("Elbow", "Elbow (renamed)"), Some(&claim(&model)));
    assert_eq!(edit.validation.application_status, "applied_to_session_model", "{:?}", codes(&edit));
    assert!(edit.diagnostics.is_empty(), "{:?}", codes(&edit));
    let edited = edit.applied_model.unwrap();
    assert_ne!(model_hash(&edited), original_hash);
    for slot in ["lines", "unresolved_assumptions", "constraints", "equipment_interfaces", "rule_pack_refs"] {
        assert_eq!(edited[slot], model[slot], "{slot} is carried unchanged");
    }

    let undo = apply_operation(&edited, &rename_node("Elbow (renamed)", "Elbow"), Some(&claim(&edited)));
    assert_eq!(undo.validation.application_status, "applied_to_session_model", "{:?}", codes(&undo));
    let restored = undo.applied_model.unwrap();
    assert_eq!(restored, model);
    assert_eq!(model_hash(&restored), original_hash);
    assert_eq!(undo.applied_model_backend_hash.as_deref(), Some(original_hash.as_str()));

    let mut back = rename_node("Elbow (renamed)", "Elbow");
    back["operation_id"] = json!("operation:rename-back");
    back["change"]["change_id"] = json!("change:rename-back");
    let batch = json!({"batch_id": "batch:round-trip", "operations": [rename_node("Elbow", "Elbow (renamed)"), back]});
    let outcome = apply_operation_batch(&model, &batch, Some(&claim(&model)));
    assert_eq!(outcome["validation"]["application_status"], json!("applied_to_session_model"), "{outcome}");
    assert_eq!(outcome["applied_model"], model);
    assert_eq!(outcome["applied_model_backend_hash"], json!(original_hash));
}

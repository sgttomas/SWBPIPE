//! Model document 0.8.0 (DEL-02-01 Design `model-document-0.8.0.md` §3.1) and
//! 0.9.0 (`model-document-0.9.0.md` §3.2): the applier operates only on a
//! current document. A retired one (0.8.0 and older) is refused by name with MODEL_SCHEMA_VERSION_RETIRED before any
//! change is checked or written, on validate, apply and batch alike; nothing
//! is migrated. The model is the committed 0.7.0 handoff control's.
use open_pipe_stress_operation_applier::{apply_operation, apply_operation_batch, validate_operation};
use serde_json::{json, Value};

const RETIRED_MODEL: &str = include_str!("../../../../fixtures/handoff/retired_0_7_0_witness.swbhandoff/model/model.json");

fn model(version: &str) -> Value {
    let mut model: Value = serde_json::from_str(RETIRED_MODEL).unwrap();
    model["schema_version"] = json!(version);
    model["components"] = json!([{"id": "component:r", "kind": "reducer", "node": "node:a", "geometry": {}, "provenance": "invented review"}]);
    model
}

fn offset_intent() -> Value {
    json!({
        "operation_id": "op:test-eccentric-offset", "operation_kind": "modify", "operation_status": "proposed", "author_type": "user",
        "target": {"object_type": "Component", "ref": "component:r"},
        "change": {"change_id": "change:test-eccentric-offset", "change_kind": "set_field", "field_label": "geometry.eccentric_offset",
            "field_path": "geometry.eccentric_offset", "before": "TBD", "after": r#"{"x":0,"y":-50,"z":0,"unit":"mm"}"#,
            "unit": "mm", "dimension": "length", "source_note": "test"},
        "validation": {"schema_validation": "not_run", "constraint_validation": "not_run", "unit_validation": "not_run",
            "diff_preview_status": "not_generated", "application_status": "not_applied"},
        "audit_boundary": {"mutation_route": "structured_operations_only", "direct_model_mutation_allowed": false,
            "requires_user_acceptance": true, "mutates_accepted_model_state": false},
        "professional_boundary": {"human_review_required": true, "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false, "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false, "software_makes_authentication_claim": false},
        "rationale": "test intent"
    })
}

#[test]
fn a_retired_document_is_refused_by_name_on_validate_and_apply() {
    for version in ["0.8.0", "0.7.0", "0.6.0", "0.1.0"] {
        let message = format!("model document {version} is retired; re-author it as model document 0.9.0 (a pressure-free model omits pressure_contract). Nothing is migrated.");
        for outcome in [validate_operation(&model(version), &offset_intent(), None), apply_operation(&model(version), &offset_intent(), None)] {
            assert!(outcome.applied_model.is_none(), "{version}: nothing is written");
            assert_ne!(outcome.validation.application_status, "applied_to_session_model", "{version}");
            let refusal: Vec<_> = outcome.diagnostics.iter().filter(|d| d.code == "MODEL_SCHEMA_VERSION_RETIRED").collect();
            assert_eq!(refusal.len(), 1, "{version}: {:?}", outcome.diagnostics);
            assert_eq!((refusal[0].severity.as_str(), refusal[0].message.as_str()), ("blocking", message.as_str()));
        }
    }
    // The current document takes the same edit.
    let current = apply_operation(&model("0.9.0"), &offset_intent(), None);
    assert!(current.diagnostics.is_empty(), "{:?}", current.diagnostics);
    assert_eq!(current.applied_model.unwrap()["components"][0]["geometry"]["eccentric_offset"], json!({"x": 0.0, "y": -50.0, "z": 0.0, "unit": "mm"}));
}

fn claim(model: &Value) -> Value {
    let hash = open_pipe_stress_operation_applier::sha256_hex(&open_pipe_stress_operation_applier::canonical_json(model));
    json!({"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_scope": "model_payload", "value": format!("sha256:{hash}")})
}

#[test]
fn a_retired_document_is_refused_in_a_batch_and_no_model_is_returned() {
    let batch = json!({"batch_id": "batch:retired", "operations": [offset_intent()]});
    let current = model("0.9.0");
    let applied = apply_operation_batch(&current, &batch, Some(&claim(&current)));
    assert_eq!(applied["validation"]["application_status"], json!("applied_to_session_model"), "{applied}");
    let retired = model("0.8.0");
    let outcome = apply_operation_batch(&retired, &batch, Some(&claim(&retired)));
    assert!(outcome["applied_model"].is_null(), "{outcome}");
    assert!(outcome.to_string().contains("MODEL_SCHEMA_VERSION_RETIRED"), "{outcome}");
}

/// R7-2: every non-current state (`fixtures/model_document/version_states.json`, the
/// desktop's states) is refused at the common boundary with its code, on validate,
/// apply and batch, for each kind of edit; nothing is written.
fn shear_intent(project: &str) -> Value {
    let mut intent = offset_intent();
    intent["target"] = json!({"object_type": "Project", "ref": project});
    intent["change"] = json!({"change_id": "change:test-shear", "change_kind": "set_model_setting", "field_label": "shear model",
        "field_path": "model_settings.shear_model", "before": "not_present", "after": "euler_bernoulli",
        "unit": "none", "dimension": "dimensionless", "source_note": "test"});
    intent
}

#[test]
fn every_non_current_version_is_refused_by_state_on_every_entry() {
    let table: Value = serde_json::from_str(include_str!("../../../../fixtures/model_document/version_states.json")).unwrap();
    let mut refused = 0;
    for case in table["cases"].as_array().unwrap() {
        let label = case["label"].as_str().unwrap();
        let mut document = model("0.9.0");
        match case.get("schema_version") {
            Some(version) => document["schema_version"] = version.clone(),
            None => {
                document.as_object_mut().unwrap().remove("schema_version");
            }
        }
        let project = document["project"]["id"].as_str().unwrap().to_string();
        for intent in [offset_intent(), shear_intent(&project)] {
            let kind = intent["change"]["change_kind"].as_str().unwrap().to_string();
            let outcomes = [validate_operation(&document, &intent, None), apply_operation(&document, &intent, None)];
            let batch = apply_operation_batch(&document, &json!({"batch_id": "batch:version", "operations": [intent]}), Some(&claim(&document)));
            let Some(code) = case["code"].as_str() else {
                for outcome in outcomes {
                    assert!(outcome.diagnostics.is_empty(), "{label} {kind}: {:?}", outcome.diagnostics);
                }
                assert_eq!(batch["validation"]["application_status"], json!("applied_to_session_model"), "{label} {kind}: {batch}");
                continue;
            };
            let detail = case["detail"].as_str().unwrap();
            let message = detail.strip_prefix("MODEL_SCHEMA_VERSION_RETIRED: ").unwrap_or(detail);
            for outcome in outcomes {
                assert!(outcome.applied_model.is_none(), "{label} {kind}: nothing is written");
                let named: Vec<_> = outcome.diagnostics.iter().filter(|d| d.code == code).collect();
                assert_eq!(named.len(), 1, "{label} {kind}: {:?}", outcome.diagnostics);
                assert_eq!((named[0].severity.as_str(), named[0].message.as_str()), ("blocking", message), "{label} {kind}");
            }
            assert!(batch["applied_model"].is_null(), "{label} {kind}: {batch}");
            assert!(batch.to_string().contains(code), "{label} {kind}: {batch}");
            refused += 1;
        }
    }
    assert_eq!(refused, 2 * 29, "twenty-nine non-current states, two edits each");
}

//! The explicit removal of a support's stored direction (DEL-04-03 REQ-014, DEL-04-11
//! REQ-014): a direction is not an input of a hanger, and a stored direction is an entered
//! engineering value, so it is never dropped as a side effect of writing a hanger. Its
//! removal is a change of its own: update_support on the Support, field path `direction`,
//! before the stored direction's canonical JSON, after `not_present`, unit none, dimension
//! dimensionless. Removal only. In one batch, the removal then the configuration update
//! that writes the hanger; each step leaves a document that passes the rules. Invented
//! fixtures only.
use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_model_document, OperationOutcome,
};
use serde_json::{json, Value};

const SKEWED: &str = r#"{"x":0.1,"y":-0.2,"z":5e-300}"#;
const AXIS_ALIGNED: &str = r#"{"x":0,"y":-1,"z":0}"#;
const CONFIG: [&str; 6] = ["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"];

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

fn direction(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// A skewed line stop on UY, or an axis-aligned spring on UY.
fn directed(text: &str) -> Value {
    if text == SKEWED {
        json!({"id":"support:s","label":"Stop","node":"node:n","family":"line_stop","restraints":["UY"],"direction":direction(text),"provenance":"user"})
    } else {
        json!({"id":"support:s","label":"Spring","node":"node:n","family":"spring","restraints":["UY"],"stiffness":{"dof":"UY","value":q(1000.0,"N/m")},"direction":direction(text),"provenance":"user"})
    }
}

fn model_with(support: Value) -> Value {
    json!({"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model","project":{"id":"project:test","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"K"}},
        "nodes":[{"id":"node:n","label":"Node","position":{"x":0,"y":0,"z":0},"provenance":"user"}],
        "supports":[support],"materials":[],"sections":[],"pipe_segments":[],"components":[],"load_cases":[],"combinations":[]})
}

fn projection(support: &Value) -> Value {
    Value::Object(CONFIG.into_iter().filter_map(|k| support.get(k).map(|v| (k.into(), v.clone()))).collect())
}

fn op(path: &str, before: &str, after: &str) -> Value {
    json!({"operation_id":format!("operation:{path}"),"operation_kind":"modify","operation_status":"proposed","author_type":"user",
        "target":{"object_type":"Support","ref":"support:s"},
        "change":{"change_id":format!("change:{path}"),"change_kind":"update_support","field_label":"Support","field_path":path,"before":before,"after":after,"unit":"none","dimension":"dimensionless","source_note":"Explicit fixture"},
        "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
        "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
        "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},
        "rationale":"Explicit fixture"})
}

/// The removal as a caller sends it.
fn removal(stored: &Value) -> Value {
    op("direction", &canonical_json(stored), "not_present")
}

/// The configuration update that writes a variable spring hanger on UY onto `current`.
fn hanger_update(current: &Value) -> Value {
    let after = json!({"family":"variable_spring_hanger","restraints":["UY"],"provenance":"user",
        "hanger":{"hanger_type":"variable_spring_hanger","source_reference":"user","stiffness":{"dof":"UY","value":q(42000.0,"N/m")},
            "installed_load":q(460.0,"N"),"cold_load":q(430.0,"N"),"hot_load":q(390.0,"N"),"travel_range":q(0.045,"m")}});
    op("configuration", &canonical_json(&projection(current)), &after.to_string())
}

fn claim(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}

fn codes(outcome: &OperationOutcome) -> Vec<String> {
    outcome.diagnostics.iter().map(|d| d.code.clone()).collect()
}

fn without_direction(model: &Value) -> Value {
    let mut expected = model.clone();
    expected["supports"][0].as_object_mut().unwrap().remove("direction");
    expected
}

fn passes_rules(model: &Value) {
    let findings = validate_model_document(model);
    assert!(findings.iter().all(|f| !f.is_blocking()), "{findings:?}");
}

#[test]
fn the_removal_alone_removes_the_direction_and_nothing_else() {
    for text in [SKEWED, AXIS_ALIGNED] {
        let model = model_with(directed(text));
        let stored = model["supports"][0]["direction"].clone();
        let outcome = apply_operation(&model, &removal(&stored), None);
        let applied = outcome.applied_model.clone().unwrap_or_else(|| panic!("{text}: {:?}", outcome.diagnostics));
        assert!(outcome.diagnostics.iter().all(|d| d.severity != "blocking"), "{text}: {:?}", outcome.diagnostics);
        assert_eq!(outcome.validation.before_state_validation, "passed");
        assert!(applied["supports"][0].get("direction").is_none(), "{text}");
        // Everything else byte for byte.
        assert_eq!(serde_json::to_string(&applied).unwrap(), serde_json::to_string(&without_direction(&model)).unwrap(), "{text}");
        passes_rules(&applied);
        // What a person reads in the review: the stored direction, then absent.
        assert_eq!(outcome.diff_preview.len(), 1);
        let row = &outcome.diff_preview[0];
        assert_eq!((row.field_path.as_str(), row.before.as_str(), row.after.as_str()), ("direction", canonical_json(&stored).as_str(), "not_present"));
        assert_eq!((row.change_kind.as_str(), row.unit.as_str(), row.dimension.as_str()), ("update_support", "none", "dimensionless"));
    }
    // The before text is the canonical JSON of the stored value: the skewed components keep their bits.
    assert_eq!(canonical_json(&direction(SKEWED)), r#"{"x":0.1,"y":-0.2,"z":5e-300}"#);
}

#[test]
fn removal_then_hanger_configuration_applies_as_one_batch() {
    for text in [SKEWED, AXIS_ALIGNED] {
        let model = model_with(directed(text));
        let current = model["supports"][0].clone();
        let batch = json!({"batch_id":"batch:hanger","operations":[removal(&current["direction"]), hanger_update(&current)]});
        let outcome = apply_operation_batch(&model, &batch, Some(&claim(&model)));
        assert_eq!(outcome["simulation_disposition"], "committed_as_one_batch", "{text}: {outcome}");
        assert_eq!(outcome["operation_outcomes"].as_array().unwrap().len(), 2);
        let applied = &outcome["applied_model"];
        let support = &applied["supports"][0];
        assert!(support.get("direction").is_none(), "{text}");
        assert_eq!(support["hanger"]["hanger_type"], "variable_spring_hanger");
        assert_eq!(support["family"], "variable_spring_hanger");
        assert_eq!((support["label"].clone(), support["node"].clone()), (current["label"].clone(), current["node"].clone()));
        passes_rules(applied);
        // The input is never written: discarding the result (the session's undo checkpoint) restores the stored direction byte for byte.
        assert_eq!(canonical_json(&model["supports"][0]["direction"]), text);
    }
}

#[test]
fn the_wrong_order_is_refused_whole() {
    let model = model_with(directed(SKEWED));
    let current = model["supports"][0].clone();
    let batch = json!({"batch_id":"batch:hanger","operations":[hanger_update(&current), removal(&current["direction"])]});
    let outcome = apply_operation_batch(&model, &batch, Some(&claim(&model)));
    assert_eq!(outcome["simulation_disposition"], "rolled_back_no_model_published", "{outcome}");
    assert!(outcome["applied_model"].is_null());
}

const HANGER_MESSAGE: &str = "supports[0].direction of support `support:s` is not an input of a hanger: a hanger's load acts along the positive axis of its stiffness DOF (or of its single translational restraint), so remove the direction. (at supports[0].direction)";
/// Main's words for a direction on a hanger family record, which the hanger message is to
/// replace for every record that carries a hanger (pull request 464, open finding 9).
const FAMILY_MESSAGE: &str = "supports[0].direction of support `support:s` is admitted only on an explicit line_stop, spring or snubber family. (at supports[0].direction)";
const DOCUMENT_REMEDY: &str = "The operation's result would leave a malformed model-data slot; correct the slot or the operation and retry.";

/// The configuration update alone that writes a hanger onto a directed record is refused by
/// the document rule, whose code, message and remedy reach the caller unchanged.
#[test]
fn the_configuration_update_alone_is_refused_by_the_document_rule() {
    for text in [SKEWED, AXIS_ALIGNED] {
        let model = model_with(directed(text));
        let current = &model["supports"][0];
        // As the hanger selection writes it (family the hanger type), and a spring family carrying the hanger.
        let mut spring_family = hanger_update(current);
        let mut after: Value = serde_json::from_str(spring_family["change"]["after"].as_str().unwrap()).unwrap();
        after["family"] = json!("spring");
        spring_family["change"]["after"] = json!(after.to_string());
        for (intent, messages) in [(hanger_update(current), vec![FAMILY_MESSAGE, HANGER_MESSAGE]), (spring_family, vec![HANGER_MESSAGE])] {
            let outcome = apply_operation(&model, &intent, None);
            assert!(outcome.applied_model.is_none(), "{text}");
            assert_eq!(outcome.validation.application_status, "blocked");
            assert_eq!(codes(&outcome), vec!["SUPPORT_DIRECTION_RECORD_UNSUPPORTED".to_string()], "{text}");
            let refusal = &outcome.diagnostics[0];
            assert_eq!(refusal.severity, "blocking");
            assert!(messages.contains(&refusal.message.as_str()), "{text}: {}", refusal.message);
            assert_eq!(refusal.remediation, DOCUMENT_REMEDY);
            assert_eq!(refusal.affected_refs, vec!["support:s".to_string()]);
            assert_eq!(refusal.source, "core/model_operations/operation_applier");
        }
    }
}

/// Each named refusal blocks with no result, its code, message and remedy.
#[test]
fn each_refusal_is_named() {
    let skewed = model_with(directed(SKEWED));
    let stored = skewed["supports"][0]["direction"].clone();
    let refused = |model: &Value, intent: Value, code: &str, message: &str, remedy: &str| {
        let outcome = apply_operation(model, &intent, None);
        assert!(outcome.applied_model.is_none(), "{code}");
        assert_eq!(outcome.validation.application_status, "blocked");
        let found: Vec<_> = outcome.diagnostics.iter().map(|d| (d.code.as_str(), d.message.as_str(), d.remediation.as_str())).collect();
        assert_eq!(found, vec![(code, message, remedy)]);
        outcome
    };
    // Setting or changing a direction is not provided.
    for after in [AXIS_ALIGNED, SKEWED, "null", "{}"] {
        refused(&skewed, op("direction", &canonical_json(&stored), after), "OP-DELETE-AFTER-VALUE-INVALID",
            &format!("support `support:s` direction: an operation can only remove a stored direction (after-value `not_present`), not set or change it to `{after}`"),
            "Send after-value `not_present` to remove the stored direction; a direction is entered only when the support is created.");
    }
    // A stale before.
    let stale = refused(&skewed, op("direction", AXIS_ALIGNED, "not_present"), "OP-BEFORE-VALUE-MISMATCH",
        &format!("support `support:s` direction: the before-value `{AXIS_ALIGNED}` is not the stored direction `{SKEWED}`"),
        "Prepare the removal again from the current model: its before-value is the stored direction's canonical JSON.");
    assert_eq!(stale.validation.before_state_validation, "blocked");
    // A record with no direction has nothing to remove.
    let undirected = without_direction(&skewed);
    for before in ["not_present", SKEWED] {
        refused(&undirected, op("direction", before, "not_present"), "OP-BEFORE-VALUE-MISMATCH",
            "support `support:s` has no stored direction, so there is nothing to remove",
            "Leave this change out: the support has no direction to remove.");
    }
    // Another change kind on the path, and another unit, are the envelope's refusals.
    let mut wrong_kind = removal(&stored);
    wrong_kind["change"]["change_kind"] = json!("set_field");
    assert!(codes(&apply_operation(&skewed, &wrong_kind, None)).contains(&"OP-RICH-KIND-INVALID".to_string()));
    let mut wrong_unit = removal(&stored);
    wrong_unit["change"]["unit"] = json!("m");
    assert!(codes(&apply_operation(&skewed, &wrong_unit, None)).contains(&"OP-RICH-PAYLOAD-INVALID".to_string()));
}

/// The removal does not touch the configuration envelope: the configuration's before is the
/// same text before and after it.
#[test]
fn the_configuration_envelope_is_unchanged() {
    let model = model_with(directed(SKEWED));
    let applied = apply_operation(&model, &removal(&model["supports"][0]["direction"]), None).applied_model.unwrap();
    assert_eq!(canonical_json(&projection(&applied["supports"][0])), canonical_json(&projection(&model["supports"][0])));
}

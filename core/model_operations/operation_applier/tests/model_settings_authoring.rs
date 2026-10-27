//! The user's means to change the node-proximity tolerance (owner ruling,
//! 2026-10-23: "I choose C, but my value is "5mm" and give the user the means
//! to change it"; DEL-02-01 Design `model-document-0.7.0.md` §3.8, DEL-04-06
//! Design `model-topology-checks.md`): `set_model_setting` and
//! `clear_model_setting` on `model_settings.node_proximity_tolerance`.
//!
//! Each operation is applied through the common seam, undone with its inverse
//! (the exact prior document bytes and model hash) and redone; its refusals
//! carry the setting reader's codes and the document validator's messages.
//! Through the solve (the desktop's value entry, both solver modes), a changed
//! tolerance changes which nodes warn `NODE_NEAR_DUPLICATE`. The models are
//! #1574's cantilevers (`product_physics/tests/topology_checks_runtime.rs`).
//! All data are invented.
use open_pipe_stress_operation_applier::{
    apply_operation, canonical_json, sha256_hex, validate_model_document, validate_operation, OperationOutcome,
};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const SETTING: &str = "model_settings.node_proximity_tolerance";
const PROJECT: &str = "project:w8";

// ---- #1574's cantilever models (topology_checks_runtime.rs) ----------------

fn node(id: &str, x: f64, y: f64, z: f64) -> Value {
    json!({"id":id,"position":{"x":x,"y":y,"z":z},"provenance":"w8"})
}
fn pipe(id: &str, from: &str, to: &str) -> Value {
    json!({"id":id,"from":from,"to":to,"section":{"outside_diameter":{"value":0.12,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"material":"material:m","y_reference":{"x":0.0,"y":1.0,"z":0.0},"provenance":"w8"})
}
fn anchor(id: &str, node: &str) -> Value {
    json!({"id":id,"node":node,"family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"w8"})
}

/// The model document: a 1 m cantilever a-b along +x (a anchored, b loaded)
/// and an anchored cantilever c-d along +z whose end c lies `gap_m` from b,
/// so b and c are distinct nodes no element joins.
fn cantilevers(gap_m: f64) -> Value {
    json!({"schema_version":"0.8.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":PROJECT,"units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[node("node:a",0.0,0.0,0.0),node("node:b",1.0,0.0,0.0),node("node:c",1.0,gap_m,0.0),node("node:d",1.0,gap_m,1.0)],
        "pipe_segments":[pipe("pipe:a-b","node:a","node:b"),pipe("pipe:c-d","node:c","node:d")],
        "supports":[anchor("support:a","node:a"),anchor("support:c","node:c")],"components":[],
        "materials":[{"id":"material:m","elastic_modulus":{"value":200e9,"unit":"Pa"},"shear_modulus":{"value":76.923076923e9,"unit":"Pa"},
            "thermal_expansion_coefficient":{"value":1.2e-5,"unit":"1/degC"},"provenance":"w8_invented_test_values"}],
        "load_cases":[{"id":"case:f","primitive_loads":[
            {"id":"load:f","category":"concentrated_force","target":{"type":"node","node":"node:b"},"direction":"global_y","magnitude":{"value":1000.0,"unit":"N"},"dimension":"force","provenance":"w8"}],
            "provenance":"w8"}],
        "combinations":[]})
}

/// The `NODE_NEAR_DUPLICATE` diagnostics of `model` solved through the
/// desktop's value entry (`{"model": .., "materials": []}`), after checking
/// that the solve published and nothing blocked.
fn near_duplicates(model: &Value, mode: PreviewSolverMode) -> Vec<Value> {
    let envelope = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), mode).expect("captured entry");
    let envelope = serde_json::to_value(envelope).unwrap();
    let diagnostics = envelope["diagnostics"].as_array().cloned().unwrap_or_default();
    let blocking: Vec<_> = diagnostics.iter().filter(|d| d["severity"] == "blocking").collect();
    assert!(blocking.is_empty(), "{blocking:?}");
    assert!(!envelope["results"].as_array().unwrap().is_empty(), "the solve publishes results");
    diagnostics.into_iter().filter(|d| d["code"] == "NODE_NEAR_DUPLICATE").collect()
}

// ---- operations -----------------------------------------------------------

fn model_hash(model: &Value) -> String {
    format!("sha256:{}", sha256_hex(&canonical_json(model)))
}

fn claim(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":model_hash(model)})
}

#[allow(clippy::too_many_arguments)]
fn intent(change_kind: &str, target: &str, path: &str, before: &str, after: &str, unit: &str, dimension: &str) -> Value {
    json!({
        "operation_id": format!("operation:{change_kind}"), "operation_kind": "modify", "operation_status": "proposed",
        "author_type": "user", "target": {"object_type": "Project", "ref": target},
        "change": {"change_id": format!("change:{change_kind}"), "change_kind": change_kind, "field_label": "Node-proximity tolerance",
                   "field_path": path, "before": before, "after": after, "unit": unit,
                   "dimension": dimension, "source_note": "invented test input"},
        "validation": {"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run",
                       "diff_preview_status":"not_generated","application_status":"not_applied"},
        "audit_boundary": {"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,
                           "requires_user_acceptance":true,"mutates_accepted_model_state":false},
        "professional_boundary": {"human_review_required":true,"software_makes_compliance_claim":false,
            "software_makes_certification_claim":false,"software_makes_sealing_claim":false,
            "software_makes_approval_claim":false,"software_makes_authentication_claim":false},
        "rationale": "Invented model-settings authoring test"
    })
}

/// The current member's wire `before`.
fn before_of(model: &Value) -> String {
    model.pointer("/model_settings/node_proximity_tolerance").map(canonical_json).unwrap_or_else(|| "not_present".into())
}

fn set(model: &Value, value: &str, unit: &str) -> Value {
    intent("set_model_setting", PROJECT, SETTING, &before_of(model), value, unit, "length")
}

fn clear(model: &Value) -> Value {
    intent("clear_model_setting", PROJECT, SETTING, &before_of(model), "not_present", "none", "dimensionless")
}

fn codes(outcome: &OperationOutcome) -> Vec<(&str, &str)> {
    outcome.diagnostics.iter().map(|item| (item.code.as_str(), item.severity.as_str())).collect()
}

/// Validate, then apply: the preview and the apply agree and the input is never mutated.
fn applied(model: &Value, intent: &Value) -> (Value, OperationOutcome) {
    let before = model.clone();
    let preview = validate_operation(model, intent, Some(&claim(model)));
    assert!(preview.applied_model.is_none());
    assert_eq!(preview.validation.application_status, "not_applied");
    assert!(preview.diagnostics.is_empty(), "preview: {:?}", codes(&preview));
    let outcome = apply_operation(model, intent, Some(&claim(model)));
    assert_eq!(*model, before, "the input is never mutated");
    assert_eq!(outcome.validation.application_status, "applied_to_session_model", "{:?}", codes(&outcome));
    assert!(outcome.diagnostics.is_empty(), "{:?}", codes(&outcome));
    assert_eq!(serde_json::to_value(&preview.diff_preview).unwrap(), serde_json::to_value(&outcome.diff_preview).unwrap());
    let next = outcome.applied_model.clone().unwrap();
    assert_eq!(outcome.applied_model_backend_hash.as_deref(), Some(model_hash(&next).as_str()));
    assert!(validate_model_document(&next).is_empty(), "the result is a valid document");
    (next, outcome)
}

/// Apply in both modes and expect one blocking refusal with `code`; returns its message.
fn refused(model: &Value, intent: &Value, code: &str) -> String {
    let preview = validate_operation(model, intent, Some(&claim(model)));
    let outcome = apply_operation(model, intent, Some(&claim(model)));
    assert!(outcome.applied_model.is_none(), "applied unexpectedly");
    assert_eq!(outcome.validation.application_status, "blocked");
    assert!(outcome.diff_preview.is_empty());
    assert_eq!(codes(&outcome), vec![(code, "blocking")], "{:?}", outcome.diagnostics);
    assert_eq!(codes(&preview), codes(&outcome), "the preview refuses as the apply does");
    outcome.diagnostics[0].message.clone()
}

fn bytes(model: &Value) -> String {
    serde_json::to_string(model).unwrap()
}

#[test]
fn set_writes_the_authored_value_and_creates_the_record() {
    let model = cantilevers(0.003);
    assert!(model.get("model_settings").is_none());
    let (next, outcome) = applied(&model, &set(&model, "2", "mm"));
    assert_eq!(next["model_settings"], json!({"node_proximity_tolerance": {"value": 2.0, "unit": "mm"}}));
    assert_eq!(outcome.change_kind, "set_model_setting");
    assert_eq!(outcome.operation_kind, "modify");
    let row = serde_json::to_value(&outcome.diff_preview).unwrap();
    assert_eq!(
        row,
        json!([{"entity_ref": PROJECT, "object_type": "Project", "field_path": SETTING, "before": "not_present",
                "after": "{\"unit\":\"mm\",\"value\":2}", "unit": "mm", "dimension": "length", "change_kind": "set_model_setting"}])
    );
    // Only the setting changed.
    let mut without = next.clone();
    without.as_object_mut().unwrap().remove("model_settings");
    assert_eq!(bytes(&without), bytes(&model));

    // A second set replaces the value, in another catalog length unit.
    let (again, outcome) = applied(&next, &set(&next, "0.01", "m"));
    assert_eq!(again["model_settings"], json!({"node_proximity_tolerance": {"value": 0.01, "unit": "m"}}));
    assert_eq!(outcome.diff_preview[0].before, "{\"unit\":\"mm\",\"value\":2}");
}

#[test]
fn clear_removes_the_member_and_the_emptied_record() {
    let model = cantilevers(0.003);
    let (set_model, _) = applied(&model, &set(&model, "2", "mm"));
    let (cleared, outcome) = applied(&set_model, &clear(&set_model));
    assert!(cleared.get("model_settings").is_none(), "the default is never written");
    assert_eq!(bytes(&cleared), bytes(&model));
    let row = serde_json::to_value(&outcome.diff_preview).unwrap();
    assert_eq!(
        row,
        json!([{"entity_ref": PROJECT, "object_type": "Project", "field_path": SETTING, "before": "{\"unit\":\"mm\",\"value\":2}",
                "after": "not_present", "unit": "none", "dimension": "dimensionless", "change_kind": "clear_model_setting"}])
    );
    // Clearing an absent setting is refused by name; nothing changes.
    let message = refused(&model, &clear(&model), "OP-MODEL-SETTING-UNCHANGED");
    assert!(message.contains("nothing to clear"), "{message}");
    // So is setting the stored value again (an authored integer 2 equals "2").
    let mut authored = model.clone();
    authored["model_settings"] = json!({"node_proximity_tolerance": {"value": 2, "unit": "mm"}});
    refused(&authored, &set(&authored, "2", "mm"), "OP-MODEL-SETTING-UNCHANGED");
}

/// Undo is the inverse operation, built from the diff row's `before`: it
/// restores the exact prior bytes and model hash; redo gives the forward result.
#[test]
fn each_operation_undoes_to_the_exact_prior_bytes() {
    let base = cantilevers(0.003);
    let mut authored = base.clone();
    authored["model_settings"] = json!({"node_proximity_tolerance": {"value": 1, "unit": "in"}});
    for start in [&base, &authored] {
        // set, undo, redo
        let forward = set(start, "2", "mm");
        let (next, _) = applied(start, &forward);
        let undo = match start.pointer("/model_settings/node_proximity_tolerance") {
            None => clear(&next),
            Some(previous) => {
                let previous = previous.clone();
                set(&next, &previous["value"].to_string(), previous["unit"].as_str().unwrap())
            }
        };
        let (undone, _) = applied(&next, &undo);
        assert_eq!(model_hash(&undone), model_hash(start));
        assert_eq!(canonical_json(&undone), canonical_json(start), "undo restores the document");
        let (redone, _) = applied(&undone, &forward);
        assert_eq!(bytes(&redone), bytes(&next));
    }
    // clear, undo (set back the cleared value), redo
    let forward = clear(&authored);
    let (next, _) = applied(&authored, &forward);
    let (undone, _) = applied(&next, &set(&next, "1", "in"));
    assert_eq!(canonical_json(&undone), canonical_json(&authored));
    assert_eq!(model_hash(&undone), model_hash(&authored));
    let (redone, _) = applied(&undone, &forward);
    assert_eq!(bytes(&redone), bytes(&next));
}

/// The refusals are the setting reader's, by name, with the messages the
/// document validator gives for the same authored value.
#[test]
fn an_invalid_value_or_unit_is_refused_by_name() {
    let model = cantilevers(0.003);
    let validator_message = |value: Value, unit: &str| {
        let mut authored = model.clone();
        authored["model_settings"] = json!({"node_proximity_tolerance": {"value": value, "unit": unit}});
        let found = validate_model_document(&authored);
        assert_eq!(found.len(), 1, "{found:?}");
        (found[0].code, found[0].message.clone())
    };
    for (value, json_value, unit, code) in [
        ("0", json!(0.0), "mm", "MODEL-SETTING-TOLERANCE-NOT-POSITIVE"),
        ("-1.5", json!(-1.5), "mm", "MODEL-SETTING-TOLERANCE-NOT-POSITIVE"),
        ("5e-324", json!(5e-324), "mm", "MODEL-SETTING-TOLERANCE-NOT-POSITIVE"),
        ("5", json!(5.0), "N", "MODEL-SETTING-TOLERANCE-UNIT-NOT-LENGTH"),
        ("5", json!(5.0), "furlong_invented", "MODEL-SETTING-TOLERANCE-UNIT-NOT-LENGTH"),
    ] {
        let message = refused(&model, &set(&model, value, unit), code);
        assert_eq!(validator_message(json_value, unit), (code, message), "{value} {unit}");
    }
    // A value JSON cannot carry is still named by the reader. (No catalog length
    // unit is larger than a metre, so no finite value overflows in metres.)
    for (value, shown) in [("inf", "inf"), ("NaN", "NaN"), ("1e999", "inf")] {
        let message = refused(&model, &set(&model, value, "mm"), "MODEL-SETTING-TOLERANCE-NOT-FINITE");
        assert_eq!(message, format!("{SETTING}.value is {shown}; a finite length greater than zero is required."));
    }
    let message = refused(&model, &set(&model, "0", "mm"), "MODEL-SETTING-TOLERANCE-NOT-POSITIVE");
    assert_eq!(message, format!("{SETTING}.value is 0; the tolerance must be greater than zero."));
    let message = refused(&model, &set(&model, "5", "N"), "MODEL-SETTING-TOLERANCE-UNIT-NOT-LENGTH");
    assert_eq!(message, format!("{SETTING}.unit `N` is not a length unit of the units catalog."));
    refused(&model, &set(&model, "five", "mm"), "OP-VALUE-NOT-NUMERIC");
}

#[test]
fn the_wire_is_checked() {
    let model = cantilevers(0.003);
    let mut wrong = set(&model, "2", "mm");
    wrong["change"]["dimension"] = json!("dimensionless");
    refused(&model, &wrong, "OP-UNIT-DIMENSION-MISMATCH");
    let mut stale = set(&model, "2", "mm");
    stale["change"]["before"] = json!("{\"unit\":\"mm\",\"value\":3}");
    refused(&model, &stale, "OP-STALE-BEFORE-VALUE");
    let mut other_project = set(&model, "2", "mm");
    other_project["target"]["ref"] = json!("project:other");
    refused(&model, &other_project, "OP-TARGET-NOT-FOUND");
    let mut other_type = set(&model, "2", "mm");
    other_type["target"]["object_type"] = json!("Node");
    refused(&model, &other_type, "OP-MODEL-SETTING-SHAPE-INVALID");
    let mut other_path = set(&model, "2", "mm");
    other_path["change"]["field_path"] = json!("model_settings.other");
    refused(&model, &other_path, "OP-MODEL-SETTING-UNKNOWN");
    let mut wrong_kind = set(&model, "2", "mm");
    wrong_kind["operation_kind"] = json!("create");
    refused(&model, &wrong_kind, "OP-CHANGE-KIND-MISMATCH");
    let (set_model, _) = applied(&model, &set(&model, "2", "mm"));
    let mut clear_with_value = clear(&set_model);
    clear_with_value["change"]["after"] = json!("2");
    refused(&set_model, &clear_with_value, "OP-MODEL-SETTING-SHAPE-INVALID");
    let mut old = model.clone();
    old["schema_version"] = json!("0.6.0");
    refused(&old, &set(&old, "2", "mm"), "OP-MODEL-SETTING-DOCUMENT-INVALID");
}

/// Through the solve: two nodes 3 mm apart warn at the 5 mm default; after
/// the tolerance is set to 2 mm they do not; after it is set to 1 cm (10 mm), a pair
/// 8 mm apart warns; reset to the default, it does not.
#[test]
fn a_changed_tolerance_changes_which_nodes_warn_through_the_solve() {
    for mode in MODES {
        let three_mm = cantilevers(0.003);
        let found = near_duplicates(&three_mm, mode);
        assert_eq!(found.len(), 1, "3 mm warns at the default");
        assert_eq!(found[0]["affected_refs"], json!(["node:b", "node:c"]));
        assert!(found[0]["message"].as_str().unwrap().contains("node-proximity tolerance (5e-3 m)"));

        let (two_mm_tolerance, _) = applied(&three_mm, &set(&three_mm, "2", "mm"));
        assert!(near_duplicates(&two_mm_tolerance, mode).is_empty(), "3 mm does not warn at a 2 mm tolerance");

        let eight_mm = cantilevers(0.008);
        assert!(near_duplicates(&eight_mm, mode).is_empty(), "8 mm does not warn at the default");
        let (one_cm_tolerance, _) = applied(&eight_mm, &set(&eight_mm, "10", "mm"));
        let found = near_duplicates(&one_cm_tolerance, mode);
        assert_eq!(found.len(), 1, "8 mm warns at a 1 cm tolerance");
        assert_eq!(found[0]["affected_refs"], json!(["node:b", "node:c"]));
        assert!(found[0]["message"].as_str().unwrap().contains("node-proximity tolerance (1e-2 m)"));

        let (reset, _) = applied(&one_cm_tolerance, &clear(&one_cm_tolerance));
        assert!(near_duplicates(&reset, mode).is_empty(), "reset to the default, 8 mm does not warn");
    }
}

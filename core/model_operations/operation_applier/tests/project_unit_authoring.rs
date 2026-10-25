//! Editing the project units after creation (DEL-02-02 Design
//! `project-unit-selection.md` §2.5, §2.7, §4 "Applier"): `set_project_unit`
//! on one key of `project.units`, through the common operation seam.
//!
//! Every refusal of the §2.7 table is reached by name, in the table's order;
//! a change touches one string of the document; the length unit is locked
//! once a node exists; a key absent from an opened document is set from
//! `before: not_present`; a legacy `C` is replaced by `degC`; and the inverse
//! change restores the exact prior document. All data are invented.
use open_pipe_stress_operation_applier::{
    apply_operation, canonical_json, sha256_hex, validate_model_document, validate_operation, OperationOutcome,
};
use serde_json::{json, Value};

const PROJECT: &str = "project:units-b";
const SI: [(&str, &str); 7] =
    [("length", "m"), ("force", "N"), ("moment", "N*m"), ("pressure", "Pa"), ("stress", "Pa"), ("temperature", "degC"), ("angle", "rad")];
const US: [(&str, &str); 7] =
    [("length", "in"), ("force", "lbf"), ("moment", "lbf*in"), ("pressure", "psi"), ("stress", "psi"), ("temperature", "degF"), ("angle", "deg")];

/// A current model document with the SI preset and no nodes.
fn blank() -> Value {
    let units: serde_json::Map<String, Value> = SI.iter().map(|(key, symbol)| (key.to_string(), json!(symbol))).collect();
    json!({"schema_version":"0.7.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":PROJECT,"name":"Invented units project","units":units},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[],"pipe_segments":[],"supports":[],"components":[],"materials":[],"load_cases":[],"combinations":[]})
}

/// [`blank`] with one node and an anchor on it.
fn with_node() -> Value {
    let mut model = blank();
    model["nodes"] = json!([{"id":"node:a","position":{"x":0.0,"y":0.0,"z":1.5},"provenance":"invented"}]);
    model["supports"] = json!([{"id":"support:a","node":"node:a","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"invented"}]);
    model
}

fn model_hash(model: &Value) -> String {
    format!("sha256:{}", sha256_hex(&canonical_json(model)))
}

fn claim(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":model_hash(model)})
}

/// The stored value of `key` as the wire's `before`.
fn before_of(model: &Value, key: &str) -> String {
    match model.pointer(&format!("/project/units/{key}")) {
        None => "not_present".into(),
        Some(Value::String(text)) => text.clone(),
        Some(other) => canonical_json(other),
    }
}

/// `set_project_unit` of `key` to `after`, prepared from the current `model`.
fn set(model: &Value, key: &str, after: &str) -> Value {
    json!({
        "operation_id": format!("operation:set_project_unit:{key}"), "operation_kind": "modify", "operation_status": "proposed",
        "author_type": "user", "target": {"object_type": "Project", "ref": PROJECT},
        "change": {"change_id": format!("change:set_project_unit:{key}"), "change_kind": "set_project_unit",
                   "field_label": format!("Project {key} unit"), "field_path": format!("project.units.{key}"),
                   "before": before_of(model, key), "after": after, "unit": after, "dimension": key,
                   "source_note": "invented test input"},
        "validation": {"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run",
                       "diff_preview_status":"not_generated","application_status":"not_applied"},
        "audit_boundary": {"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,
                           "requires_user_acceptance":true,"mutates_accepted_model_state":false},
        "professional_boundary": {"human_review_required":true,"software_makes_compliance_claim":false,
            "software_makes_certification_claim":false,"software_makes_sealing_claim":false,
            "software_makes_approval_claim":false,"software_makes_authentication_claim":false},
        "rationale": "Invented project-unit authoring test"
    })
}

fn codes(outcome: &OperationOutcome) -> Vec<(&str, &str)> {
    outcome.diagnostics.iter().map(|item| (item.code.as_str(), item.severity.as_str())).collect()
}

/// Validate, then apply: the preview and the apply agree and the input is never mutated.
fn applied(model: &Value, intent: &Value) -> (Value, OperationOutcome) {
    let before = model.clone();
    let preview = validate_operation(model, intent, Some(&claim(model)));
    assert!(preview.applied_model.is_none());
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

/// Validate and apply, expecting one blocking refusal with `code` and no change; returns its message.
fn refused(model: &Value, intent: &Value, code: &str) -> String {
    let before = model.clone();
    let preview = validate_operation(model, intent, Some(&claim(model)));
    let outcome = apply_operation(model, intent, Some(&claim(model)));
    assert_eq!(*model, before);
    assert!(outcome.applied_model.is_none(), "applied unexpectedly");
    assert_eq!(outcome.validation.application_status, "blocked");
    assert!(outcome.diff_preview.is_empty());
    assert_eq!(codes(&outcome), vec![(code, "blocking")], "{:?}", outcome.diagnostics);
    assert_eq!(codes(&preview), codes(&outcome), "the preview refuses as the apply does");
    outcome.diagnostics[0].message.clone()
}

/// `model` with `project.units.<key>` set to `value` (or removed), by hand: the expected result.
fn with_unit(model: &Value, key: &str, value: Option<&str>) -> Value {
    let mut expected = model.clone();
    let units = expected["project"]["units"].as_object_mut().unwrap();
    match value {
        Some(value) => units.insert(key.into(), json!(value)),
        None => units.remove(key),
    };
    expected
}

fn bytes(model: &Value) -> String {
    serde_json::to_string(model).unwrap()
}

#[test]
fn a_key_is_set_on_a_document_with_no_nodes_with_one_diff_row() {
    let model = blank();
    let (next, outcome) = applied(&model, &set(&model, "length", "mm"));
    assert_eq!(bytes(&next), bytes(&with_unit(&model, "length", Some("mm"))), "only project.units.length changed");
    assert_eq!(outcome.change_kind, "set_project_unit");
    assert_eq!(outcome.operation_kind, "modify");
    assert_eq!(
        serde_json::to_value(&outcome.diff_preview).unwrap(),
        json!([{"entity_ref": PROJECT, "object_type": "Project", "field_path": "project.units.length", "before": "m",
                "after": "mm", "unit": "mm", "dimension": "length", "change_kind": "set_project_unit"}])
    );
    // Every key moves from the SI preset to the US preset, one change each.
    let mut current = model.clone();
    for (key, symbol) in US {
        current = applied(&current, &set(&current, key, symbol)).0;
    }
    for (key, symbol) in US {
        assert_eq!(current["project"]["units"][key], json!(symbol));
    }
    assert_eq!(current["project"]["units"].as_object().unwrap().len(), 7);
}

#[test]
fn a_non_length_change_leaves_every_other_byte_unchanged() {
    let model = with_node();
    for (key, after) in [("force", "lbf"), ("moment", "kN*m"), ("pressure", "bar"), ("stress", "MPa"), ("temperature", "K"), ("angle", "deg")] {
        let (next, outcome) = applied(&model, &set(&model, key, after));
        assert_eq!(bytes(&next), bytes(&with_unit(&model, key, Some(after))), "{key}");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].dimension, key);
    }
    // Stress takes the pressure entries of the catalog, as the desktop's dimension match does.
    applied(&model, &set(&model, "stress", "psi"));
}

#[test]
fn the_length_unit_is_locked_once_a_node_exists() {
    let model = with_node();
    let message = refused(&model, &set(&model, "length", "mm"), "OP-PROJECT-UNIT-LENGTH-LOCKED");
    assert!(message.contains("fixed once nodes exist"), "{message}");
    // A missing length with nodes is a hard lock too: assigning one would be an interpretation.
    let missing = with_unit(&model, "length", None);
    refused(&missing, &set(&missing, "length", "m"), "OP-PROJECT-UNIT-LENGTH-LOCKED");
    // The checks run in the §2.7 order: a stale before is reported before the lock.
    let mut stale = set(&model, "length", "mm");
    stale["change"]["before"] = json!("in");
    refused(&model, &stale, "OP-STALE-BEFORE-VALUE");
}

#[test]
fn a_key_absent_from_an_opened_document_is_set_from_not_present() {
    for key in ["angle", "moment", "stress", "length"] {
        let model = with_unit(&blank(), key, None);
        let symbol = SI.iter().find(|(name, _)| *name == key).unwrap().1;
        let (next, outcome) = applied(&model, &set(&model, key, symbol));
        assert_eq!(outcome.diff_preview[0].before, "not_present");
        assert_eq!(outcome.diff_preview[0].after, symbol);
        assert_eq!(canonical_json(&next), canonical_json(&blank()), "{key}");
    }
    // An absent key with nodes: only length is locked.
    let model = with_unit(&with_node(), "angle", None);
    let (next, _) = applied(&model, &set(&model, "angle", "rad"));
    assert_eq!(canonical_json(&next), canonical_json(&with_node()));
    // A document without a units map gains it with the one key.
    let mut bare = blank();
    bare["project"].as_object_mut().unwrap().remove("units");
    let (next, _) = applied(&bare, &set(&bare, "force", "N"));
    assert_eq!(next["project"]["units"], json!({"force": "N"}));
}

#[test]
fn a_legacy_c_temperature_changes_to_deg_c_and_cannot_be_chosen() {
    let legacy = with_unit(&with_node(), "temperature", Some("C"));
    let (next, outcome) = applied(&legacy, &set(&legacy, "temperature", "degC"));
    assert_eq!(outcome.diff_preview[0].before, "C");
    assert_eq!(outcome.diff_preview[0].after, "degC");
    assert_eq!(bytes(&next), bytes(&with_node()));
    // `C` is an alias, never a new choice, even where it is stored.
    let message = refused(&legacy, &set(&legacy, "temperature", "C"), "OP-PROJECT-UNIT-NOT-CANONICAL");
    assert!(message.contains("legacy spelling; choose the catalog symbol degC"), "{message}");
    let model = blank();
    refused(&model, &set(&model, "temperature", "C"), "OP-PROJECT-UNIT-NOT-CANONICAL");
}

/// The inverse change, built from the diff row, restores the exact prior document and model hash.
#[test]
fn undo_restores_the_previous_map() {
    let model = with_node();
    let forward = set(&model, "pressure", "psi");
    let (next, outcome) = applied(&model, &forward);
    let row = &outcome.diff_preview[0];
    let (undone, _) = applied(&next, &set(&next, "pressure", &row.before));
    assert_eq!(bytes(&undone), bytes(&model));
    assert_eq!(model_hash(&undone), model_hash(&model));
    let (redone, _) = applied(&undone, &forward);
    assert_eq!(bytes(&redone), bytes(&next));
}

/// Every code of the §2.7 table, by name; each refusal changes nothing.
#[test]
fn each_refusal_is_reached_by_name() {
    let model = with_node();
    let base = || set(&model, "force", "lbf");

    // OP-PROJECT-UNIT-SHAPE-INVALID: another target type, or a member missing or not a string.
    let mut wrong = base();
    wrong["target"]["object_type"] = json!("Node");
    refused(&model, &wrong, "OP-PROJECT-UNIT-SHAPE-INVALID");
    for pointer in ["/change/field_path", "/change/before", "/change/unit", "/change/dimension", "/target/ref", "/target/object_type"] {
        let mut missing = base();
        let (parent, leaf) = pointer.rsplit_once('/').unwrap();
        missing.pointer_mut(parent).unwrap().as_object_mut().unwrap().remove(leaf);
        refused(&model, &missing, "OP-PROJECT-UNIT-SHAPE-INVALID");
        let mut number = base();
        *number.pointer_mut(pointer).unwrap() = json!(1);
        refused(&model, &number, "OP-PROJECT-UNIT-SHAPE-INVALID");
    }
    let mut number_after = base();
    number_after["change"]["after"] = json!(1);
    refused(&model, &number_after, "OP-PROJECT-UNIT-SHAPE-INVALID");
    let mut other_unit = base();
    other_unit["change"]["unit"] = json!("N");
    refused(&model, &other_unit, "OP-PROJECT-UNIT-SHAPE-INVALID");

    // OP-TARGET-NOT-FOUND: a ref other than the project id.
    let mut other_project = base();
    other_project["target"]["ref"] = json!("project:other");
    refused(&model, &other_project, "OP-TARGET-NOT-FOUND");

    // OP-PROJECT-UNIT-DOCUMENT-INVALID: an older document, another kind, a units member that is not a map.
    let mut old = model.clone();
    old["schema_version"] = json!("0.6.0");
    refused(&old, &set(&old, "force", "lbf"), "OP-PROJECT-UNIT-DOCUMENT-INVALID");
    let mut other_kind = model.clone();
    other_kind["document_kind"] = json!("openpipestress.other");
    refused(&other_kind, &set(&other_kind, "force", "lbf"), "OP-PROJECT-UNIT-DOCUMENT-INVALID");
    let mut not_a_map = model.clone();
    not_a_map["project"]["units"] = json!("SI");
    refused(&not_a_map, &base(), "OP-PROJECT-UNIT-DOCUMENT-INVALID");

    // OP-PROJECT-UNIT-UNKNOWN-KEY: not project.units.<one of the seven keys>.
    for path in ["project.units.mass", "project.units", "project.units.force.x", "model_settings.force", "project.name"] {
        let mut unknown = base();
        unknown["change"]["field_path"] = json!(path);
        refused(&model, &unknown, "OP-PROJECT-UNIT-UNKNOWN-KEY");
    }

    // OP-PROJECT-UNIT-REMOVAL-REFUSED: after absent, null or not_present.
    for after in [None, Some(Value::Null), Some(json!("not_present"))] {
        let mut removal = base();
        match after {
            None => {
                removal["change"].as_object_mut().unwrap().remove("after");
            }
            Some(after) => removal["change"]["after"] = after,
        }
        removal["change"]["unit"] = json!("none");
        let message = refused(&model, &removal, "OP-PROJECT-UNIT-REMOVAL-REFUSED");
        assert!(message.contains("cannot be removed"), "{message}");
    }

    // OP-PROJECT-UNIT-NOT-CANONICAL: unknown, an alias, or a different spelling.
    for after in ["furlong_invented", "C", "lbs", "LBF", "lbf ", "", "N·m"] {
        let mut intent = base();
        intent["change"]["after"] = json!(after);
        intent["change"]["unit"] = json!(after);
        refused(&model, &intent, "OP-PROJECT-UNIT-NOT-CANONICAL");
    }

    // OP-UNIT-DIMENSION-MISMATCH: a canonical symbol of another dimension, or another intent dimension.
    for (key, after) in [("force", "m"), ("moment", "N"), ("pressure", "degC"), ("stress", "N*m"), ("angle", "K"), ("length", "rad"), ("temperature", "1/degF")] {
        let message = refused(&model, &set(&model, key, after), "OP-UNIT-DIMENSION-MISMATCH");
        assert!(message.contains(&format!("takes a {key} unit")), "{message}");
    }
    let mut dimension = base();
    dimension["change"]["dimension"] = json!("moment");
    refused(&model, &dimension, "OP-UNIT-DIMENSION-MISMATCH");

    // OP-STALE-BEFORE-VALUE: the stored value moved on.
    let mut stale = base();
    stale["change"]["before"] = json!("kN");
    refused(&model, &stale, "OP-STALE-BEFORE-VALUE");
    let mut stale_absent = base();
    stale_absent["change"]["before"] = json!("not_present");
    refused(&model, &stale_absent, "OP-STALE-BEFORE-VALUE");

    // OP-PROJECT-UNIT-UNCHANGED: the stored symbol again.
    refused(&model, &set(&model, "force", "N"), "OP-PROJECT-UNIT-UNCHANGED");

    // OP-PROJECT-UNIT-LENGTH-LOCKED: length with a node.
    refused(&model, &set(&model, "length", "ft"), "OP-PROJECT-UNIT-LENGTH-LOCKED");

    // The operation kind is modify.
    let mut kind = base();
    kind["operation_kind"] = json!("create");
    refused(&model, &kind, "OP-CHANGE-KIND-MISMATCH");
}

/// The refusals run in the table's order: each intent below breaks two rules
/// and is refused by the earlier one.
#[test]
fn the_refusals_run_in_the_table_order() {
    let model = with_node();
    let both = |edit: &dyn Fn(&mut Value), model: &Value, code: &str| {
        let mut intent = set(model, "length", "mm");
        edit(&mut intent);
        refused(model, &intent, code);
    };
    // Shape before target.
    both(&|intent| { intent["target"]["ref"] = json!("project:other"); intent["change"]["before"] = json!(3); }, &model, "OP-PROJECT-UNIT-SHAPE-INVALID");
    // Target before document.
    let mut old = model.clone();
    old["schema_version"] = json!("0.6.0");
    both(&|intent| intent["target"]["ref"] = json!("project:other"), &old, "OP-TARGET-NOT-FOUND");
    // Document before key.
    both(&|intent| intent["change"]["field_path"] = json!("project.units.mass"), &old, "OP-PROJECT-UNIT-DOCUMENT-INVALID");
    // Key before removal.
    both(&|intent| { intent["change"]["field_path"] = json!("project.units.mass"); intent["change"]["after"] = json!("not_present"); }, &model, "OP-PROJECT-UNIT-UNKNOWN-KEY");
    // Removal before canonical (the removal form is refused whatever its unit).
    both(&|intent| { intent["change"]["after"] = Value::Null; intent["change"]["unit"] = json!("C"); }, &model, "OP-PROJECT-UNIT-REMOVAL-REFUSED");
    // Canonical before dimension.
    both(&|intent| { intent["change"]["after"] = json!("C"); intent["change"]["unit"] = json!("C"); intent["change"]["dimension"] = json!("force"); }, &model, "OP-PROJECT-UNIT-NOT-CANONICAL");
    // Dimension before stale.
    both(&|intent| { intent["change"]["after"] = json!("N"); intent["change"]["unit"] = json!("N"); intent["change"]["before"] = json!("ft"); }, &model, "OP-UNIT-DIMENSION-MISMATCH");
    // Stale before unchanged.
    both(&|intent| { intent["change"]["after"] = json!("m"); intent["change"]["unit"] = json!("m"); intent["change"]["before"] = json!("ft"); }, &model, "OP-STALE-BEFORE-VALUE");
    // Unchanged before the length lock.
    both(&|intent| { intent["change"]["after"] = json!("m"); intent["change"]["unit"] = json!("m"); }, &model, "OP-PROJECT-UNIT-UNCHANGED");
}

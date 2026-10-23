//! W-13 S2a (design `model-document-0.7.0.md` §6): authoring of the model-data
//! slots through the common operation seam — record operations on lines,
//! assumptions, constraints, equipment interfaces and rule-pack references;
//! tags, notes and provenance records on each family's existing change kind;
//! and the geometry and reference scans that must neither read nor be blocked
//! by those slots. Every new operation is checked with its undo (the inverse
//! operation restores the equal document and model hash) and redo.
//!
//! The applier golden (`fixtures/model_document/applier_emitted.json`) holds
//! one applied document per new operation kind; the schema pytest validates
//! each against the model format. Regenerate it with
//! `MODEL_DATA_GOLDEN_BLESS=1 cargo test --test model_data_authoring` and
//! review the diff. All data are invented.
use open_pipe_stress_operation_applier::{
    apply_operation, canonical_json, sha256_hex, validate_operation, OperationOutcome,
};
use serde_json::{json, Value};

const EVERY_SLOT: &str = include_str!("../../../../fixtures/model_document/invented_every_slot_model.json");
const GOLDEN_PATH: &str = "../../../fixtures/model_document/applier_emitted.json";

fn every_slot() -> Value {
    serde_json::from_str(EVERY_SLOT).unwrap()
}

fn model_hash(model: &Value) -> String {
    format!("sha256:{}", sha256_hex(&canonical_json(model)))
}

fn claim(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":model_hash(model)})
}

#[allow(clippy::too_many_arguments)]
fn intent(kind: &str, object_type: &str, target: &str, change_kind: &str, path: &str, before: &str, after: &str) -> Value {
    let token = format!("{change_kind}:{target}:{path}");
    json!({
        "operation_id": format!("operation:{token}"), "operation_kind": kind, "operation_status": "proposed",
        "author_type": "user", "target": {"object_type": object_type, "ref": target},
        "change": {"change_id": format!("change:{token}"), "change_kind": change_kind, "field_label": path,
                   "field_path": path, "before": before, "after": after, "unit": "none",
                   "dimension": "dimensionless", "source_note": "invented test input"},
        "validation": {"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run",
                       "diff_preview_status":"not_generated","application_status":"not_applied"},
        "audit_boundary": {"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,
                           "requires_user_acceptance":true,"mutates_accepted_model_state":false},
        "professional_boundary": {"human_review_required":true,"software_makes_compliance_claim":false,
            "software_makes_certification_claim":false,"software_makes_sealing_claim":false,
            "software_makes_approval_claim":false,"software_makes_authentication_claim":false},
        "rationale": "Invented model-data authoring test"
    })
}

/// A record operation's intent. `record` is `None` for a delete.
fn record_op(change_kind: &str, object_type: &str, collection: &str, id: &str, before: Option<&Value>, record: Option<&Value>) -> Value {
    let kind = if change_kind.starts_with("create_") || change_kind.starts_with("add_") {
        "create"
    } else if change_kind.starts_with("update_") {
        "modify"
    } else {
        "delete"
    };
    let before = before.map(canonical_json).unwrap_or_else(|| "not_present".into());
    let after = record.map(canonical_json).unwrap_or_else(|| "not_present".into());
    intent(kind, object_type, id, change_kind, collection, &before, &after)
}

/// An annotation's intent on the family's change kind.
fn annotate(object_type: &str, id: &str, field: &str, current: Option<&Value>, next: &Value) -> Value {
    let change_kind = match object_type {
        "Load" => "update_load",
        "Support" => "update_support",
        _ => "set_field",
    };
    let before = current.map(canonical_json).unwrap_or_else(|| "not_present".into());
    intent("modify", object_type, id, change_kind, field, &before, &canonical_json(next))
}

fn codes(outcome: &OperationOutcome) -> Vec<(&str, &str)> {
    outcome.diagnostics.iter().map(|item| (item.code.as_str(), item.severity.as_str())).collect()
}

fn blocking(outcome: &OperationOutcome) -> Vec<&str> {
    outcome.diagnostics.iter().filter(|item| item.severity == "blocking").map(|item| item.code.as_str()).collect()
}

/// Validate, then apply; the preview and the apply agree and the input is
/// never mutated.
fn applied(model: &Value, intent: &Value) -> (Value, OperationOutcome) {
    let before = model.clone();
    let preview = validate_operation(model, intent, Some(&claim(model)));
    assert!(preview.applied_model.is_none());
    assert_eq!(preview.validation.application_status, "not_applied");
    assert!(blocking(&preview).is_empty(), "preview blocked: {:?}", codes(&preview));
    let outcome = apply_operation(model, intent, Some(&claim(model)));
    assert_eq!(*model, before, "the input is never mutated");
    assert_eq!(outcome.validation.application_status, "applied_to_session_model", "{:?}", codes(&outcome));
    assert_eq!(
        serde_json::to_value(&preview.diff_preview).unwrap(),
        serde_json::to_value(&outcome.diff_preview).unwrap()
    );
    let next = outcome.applied_model.clone().unwrap();
    assert_eq!(outcome.applied_model_backend_hash.as_deref(), Some(model_hash(&next).as_str()));
    (next, outcome)
}

fn refused(model: &Value, intent: &Value) -> OperationOutcome {
    let outcome = apply_operation(model, intent, Some(&claim(model)));
    assert!(outcome.applied_model.is_none(), "applied unexpectedly: {:?}", codes(&outcome));
    assert_eq!(outcome.validation.application_status, "blocked");
    outcome
}

/// Apply `forward`, undo with `inverse`, redo with `forward`: undo returns the
/// equal document and model hash, redo the equal forward result.
fn round_trip(model: &Value, forward: &Value, inverse: impl Fn(&Value) -> Value) -> Value {
    let (next, _) = applied(model, forward);
    let (undone, _) = applied(&next, &inverse(&next));
    assert_eq!(undone, *model, "undo restores the document");
    assert_eq!(model_hash(&undone), model_hash(model));
    let (redone, _) = applied(&undone, forward);
    assert_eq!(redone, next, "redo restores the forward result");
    next
}

fn find<'a>(model: &'a Value, collection: &str, key: &str, id: &str) -> &'a Value {
    model[collection].as_array().unwrap().iter().find(|record| record[key] == id).unwrap()
}

fn line(id: &str, pipes: &[&str]) -> Value {
    json!({"id": id, "label": "Invented line", "pipe_refs": pipes, "provenance": "user_entered"})
}

fn assumption(id: &str) -> Value {
    json!({"assumption_id": id, "kind": "open_issue", "statement": "Invented open issue.", "status": "unresolved", "affected_refs": ["pipe:P-2"]})
}

fn constraint(id: &str) -> Value {
    json!({"constraint_id": id, "name": "Invented access", "constraint_kind": "access", "state": "proposed", "target_refs": ["node:N-3"]})
}

fn equipment_interface(id: &str) -> Value {
    json!({"id": id, "label": "Invented nozzle", "node_ref": "node:N-3", "equipment_label": "Invented vessel V-1"})
}

fn rule_pack_ref(id: &str) -> Value {
    json!({"rule_pack_id": id, "name": "Invented pack", "rule_pack_version": "0.1.0",
           "checksum": {"algorithm": "sha256", "canonicalization": "JCS", "payload_scope": "rule_pack_payload",
                        "payload_excludes": ["checksums"], "value": "a".repeat(64)},
           "redistribution_status": "private_only"})
}

/// The families: (create, update, delete, object type, collection, id key, new record).
fn families() -> Vec<(&'static str, Option<&'static str>, &'static str, &'static str, &'static str, &'static str, Value)> {
    vec![
        ("create_line", Some("update_line"), "delete_line", "Line", "lines", "id", line("line:L-300", &["pipe:P-2"])),
        ("create_assumption", Some("update_assumption"), "delete_assumption", "Assumption", "unresolved_assumptions", "assumption_id", assumption("assumption:A-003")),
        ("create_constraint", Some("update_constraint"), "delete_constraint", "Constraint", "constraints", "constraint_id", constraint("constraint:K-002")),
        ("create_equipment_interface", Some("update_equipment_interface"), "delete_equipment_interface", "EquipmentInterface", "equipment_interfaces", "id", equipment_interface("equipment-interface:E-002")),
        ("add_rule_pack_ref", None, "remove_rule_pack_ref", "RulePackRef", "rule_pack_refs", "rule_pack_id", rule_pack_ref("invented_private_pack")),
    ]
}

#[test]
fn record_operations_create_update_and_delete_with_undo_and_redo() {
    let model = every_slot();
    for (create, update, delete, object_type, collection, key, record) in families() {
        let id = record[key].as_str().unwrap().to_string();
        // create, undone by delete, redone by create
        let created = round_trip(&model, &record_op(create, object_type, collection, &id, None, Some(&record)), |next| {
            record_op(delete, object_type, collection, &id, Some(find(next, collection, key, &id)), None)
        });
        assert_eq!(find(&created, collection, key, &id), &record);
        // delete, undone by create, redone by delete
        let deleted = round_trip(&created, &record_op(delete, object_type, collection, &id, Some(&record), None), |_| {
            record_op(create, object_type, collection, &id, None, Some(&record))
        });
        assert_eq!(deleted, model);
        if let Some(update) = update {
            let mut changed = record.clone();
            changed["tags"] = json!(["edited"]);
            let updated = round_trip(&created, &record_op(update, object_type, collection, &id, Some(&record), Some(&changed)), |next| {
                record_op(update, object_type, collection, &id, Some(find(next, collection, key, &id)), Some(&record))
            });
            assert_eq!(find(&updated, collection, key, &id), &changed);
            // An update never changes the id (FR-MOD-003).
            let mut renamed = record.clone();
            renamed[key] = json!(format!("{id}-renamed"));
            let outcome = refused(&created, &record_op(update, object_type, collection, &id, Some(&record), Some(&renamed)));
            assert_eq!(blocking(&outcome), vec!["OP-DATA-ID-CHANGE-REFUSED"]);
            // A stale before refuses.
            let outcome = refused(&updated, &record_op(update, object_type, collection, &id, Some(&record), Some(&changed)));
            assert_eq!(outcome.validation.before_state_validation, "blocked_stale");
        }
        // A duplicate create refuses; a delete of an absent record refuses.
        let outcome = refused(&created, &record_op(create, object_type, collection, &id, None, Some(&record)));
        assert_eq!(blocking(&outcome), vec!["OP-DATA-ID-COLLISION"]);
        let outcome = refused(&model, &record_op(delete, object_type, collection, &id, Some(&record), None));
        assert_eq!(blocking(&outcome), vec!["OP-TARGET-NOT-FOUND"]);
    }
}

/// A delete's inverse puts the record back at its index
/// (`<collection>[<index>]`), so deleting a record that is not last is undone
/// exactly; a plain create would append it out of order.
#[test]
fn deleting_a_record_that_is_not_last_is_undone_at_its_index() {
    let model = every_slot();
    let first = find(&model, "lines", "id", "line:L-100").clone();
    let deleted = round_trip(&model, &record_op("delete_line", "Line", "lines", "line:L-100", Some(&first), None), |_| {
        let mut inverse = record_op("create_line", "Line", "lines", "line:L-100", None, Some(&first));
        inverse["change"]["field_path"] = json!("lines[0]");
        inverse
    });
    assert_eq!(deleted["lines"].as_array().unwrap().len(), 1);
    let (appended, _) = applied(&deleted, &record_op("create_line", "Line", "lines", "line:L-100", None, Some(&first)));
    assert_ne!(appended, model, "a plain create appends");
    let mut out_of_range = record_op("create_line", "Line", "lines", "line:L-100", None, Some(&first));
    out_of_range["change"]["field_path"] = json!("lines[2]");
    assert_eq!(blocking(&refused(&deleted, &out_of_range)), vec!["OP-DATA-INDEX-INVALID"]);
    let mut on_delete = record_op("delete_line", "Line", "lines", "line:L-100", Some(&first), None);
    on_delete["change"]["field_path"] = json!("lines[0]");
    assert_eq!(blocking(&refused(&model, &on_delete)), vec!["OP-DATA-SHAPE-INVALID"]);
    // The assumption family as well: delete A-001 of two and restore it first.
    let assumption = find(&model, "unresolved_assumptions", "assumption_id", "assumption:A-001").clone();
    round_trip(&model, &record_op("delete_assumption", "Assumption", "unresolved_assumptions", "assumption:A-001", Some(&assumption), None), |_| {
        let mut inverse = record_op("create_assumption", "Assumption", "unresolved_assumptions", "assumption:A-001", None, Some(&assumption));
        inverse["change"]["field_path"] = json!("unresolved_assumptions[0]");
        inverse
    });
}

#[test]
fn a_collection_emptied_by_a_delete_is_removed_and_restored_by_create() {
    let mut model = every_slot();
    model.as_object_mut().unwrap().remove("constraints");
    let record = constraint("constraint:K-009");
    let (created, _) = applied(&model, &record_op("create_constraint", "Constraint", "constraints", "constraint:K-009", None, Some(&record)));
    assert_eq!(created["constraints"], json!([record]));
    let (deleted, _) = applied(&created, &record_op("delete_constraint", "Constraint", "constraints", "constraint:K-009", Some(&record), None));
    assert!(deleted.get("constraints").is_none(), "absence means none");
    assert_eq!(model_hash(&deleted), model_hash(&model));
}

#[test]
fn record_operations_refuse_malformed_records_by_the_validator_name() {
    let model = every_slot();
    let cases = [
        // A line naming a pipe another line already groups.
        (record_op("create_line", "Line", "lines", "line:L-400", None, Some(&line("line:L-400", &["pipe:P-1"]))), "MODEL-LINE-PIPE-SHARED"),
        (record_op("create_line", "Line", "lines", "line:L-400", None, Some(&line("line:L-400", &["pipe:P-absent"]))), "MODEL-LINE-PIPE-MISSING"),
        (
            record_op("create_assumption", "Assumption", "unresolved_assumptions", "assumption:A-009", None,
                Some(&json!({"assumption_id": "assumption:A-009", "kind": "assumption", "statement": "Invented.", "status": "resolved"}))),
            "MODEL-ASSUMPTION-RESOLUTION-NOTE-REQUIRED",
        ),
        (
            record_op("create_constraint", "Constraint", "constraints", "constraint:K-009", None,
                Some(&json!({"constraint_id": "constraint:K-009", "name": "Invented", "constraint_kind": "equipment_interface", "state": "proposed"}))),
            "MODEL-CONSTRAINT-KIND-INVALID",
        ),
        (
            record_op("create_equipment_interface", "EquipmentInterface", "equipment_interfaces", "equipment-interface:E-009", None,
                Some(&json!({"id": "equipment-interface:E-009", "node_ref": "node:N-3", "evaluated": true}))),
            "MODEL-SLOT-SHAPE",
        ),
        (
            {
                let mut bad = rule_pack_ref("invented_bad_pack");
                bad["checksum"]["value"] = json!("not-a-digest");
                record_op("add_rule_pack_ref", "RulePackRef", "rule_pack_refs", "invented_bad_pack", None, Some(&bad))
            },
            "MODEL-RULE-PACK-REF-INVALID",
        ),
    ];
    for (operation, code) in cases {
        let outcome = refused(&model, &operation);
        assert_eq!(blocking(&outcome), vec![code]);
        assert_eq!(outcome.validation.schema_validation, "blocked");
        // The validate-only preview names the same finding.
        let preview = validate_operation(&model, &operation, Some(&claim(&model)));
        assert_eq!(blocking(&preview), vec![code], "the preview runs the validator");
        assert_eq!(preview.validation.diff_preview_status, "blocked_by_validation");
        assert!(preview.applied_model.is_none());
    }
    // Shape of the wire itself.
    let mut wrong = record_op("create_line", "Line", "lines", "line:L-400", None, Some(&line("line:L-400", &["pipe:P-2"])));
    wrong["target"]["object_type"] = json!("Element");
    assert_eq!(blocking(&refused(&model, &wrong)), vec!["OP-DATA-SHAPE-INVALID"]);
    let mut wrong = record_op("create_line", "Line", "lines", "line:L-400", None, Some(&line("line:L-400", &["pipe:P-2"])));
    wrong["operation_kind"] = json!("modify");
    assert_eq!(blocking(&refused(&model, &wrong)), vec!["OP-CHANGE-KIND-MISMATCH"]);
    let mismatched = record_op("create_line", "Line", "lines", "line:L-401", None, Some(&line("line:L-400", &["pipe:P-2"])));
    assert_eq!(blocking(&refused(&model, &mismatched)), vec!["OP-DATA-ID-MISMATCH"]);
    // An authored id repeats no id of the document.
    let colliding = record_op("create_line", "Line", "lines", "pipe:P-2", None, Some(&line("pipe:P-2", &[])));
    assert_eq!(blocking(&refused(&model, &colliding)), vec!["OP-DATA-ID-COLLISION"]);
    let colliding = record_op("create_line", "Line", "lines", "assumption:A-001", None, Some(&line("assumption:A-001", &[])));
    assert_eq!(blocking(&refused(&model, &colliding)), vec!["OP-DATA-ID-COLLISION"]);
    // Model-data records are authored on the current document only.
    let mut retired = model.clone();
    retired["schema_version"] = json!("0.6.0");
    let operation = record_op("create_line", "Line", "lines", "line:L-400", None, Some(&line("line:L-400", &["pipe:P-2"])));
    assert_eq!(blocking(&refused(&retired, &operation)), vec!["OP-DATA-DOCUMENT-INVALID"]);
}

/// Tags and notes on every entity type and the project; provenance records on
/// materials, sections, components and supports — each on its family's change
/// kind, with undo and redo. Clearing removes the member.
#[test]
fn annotations_on_every_entity_type_with_undo_and_redo() {
    // The witness's equipment interface and first line carry an authored
    // `"tags": []`, which the inverse edit restores exactly.
    let model = every_slot();
    let targets: [(&str, &str, &str, &str); 13] = [
        ("Project", "project:slots", "project", ""),
        ("Node", "node:N-3", "nodes", "id"),
        ("Element", "pipe:P-2", "pipe_segments", "id"),
        ("Section", "section:std", "sections", "id"),
        ("Material", "material:steel", "materials", "id"),
        ("Support", "support:S-1", "supports", "id"),
        ("Component", "component:C-1", "components", "id"),
        ("Load", "load:L-1", "load_cases", "id"),
        ("Combination", "combination:C-OP", "combinations", "id"),
        ("Line", "line:L-200", "lines", "id"),
        ("Assumption", "assumption:A-002", "unresolved_assumptions", "assumption_id"),
        ("Constraint", "constraint:K-001", "constraints", "constraint_id"),
        ("EquipmentInterface", "equipment-interface:E-001", "equipment_interfaces", "id"),
    ];
    let entity = |document: &Value, collection: &str, key: &str, id: &str| -> Value {
        if collection == "project" { document["project"].clone() } else { find(document, collection, key, id).clone() }
    };
    for (object_type, id, collection, key) in targets {
        let current = entity(&model, collection, key, id);
        for (field, value) in [("tags", json!(["invented", "rev-C"])), ("notes", json!("Invented note."))] {
            let existing = current.get(field);
            let next = round_trip(&model, &annotate(object_type, id, field, existing, &value), |next| {
                let now = entity(next, collection, key, id);
                annotate(object_type, id, field, now.get(field), existing.unwrap_or(&Value::Null))
            });
            assert_eq!(entity(&next, collection, key, id)[field], value, "{object_type} {field}");
            // Clearing removes the member (no TBD sentinel).
            let (cleared, _) = applied(&next, &annotate(object_type, id, field, Some(&value), &Value::Null));
            assert!(entity(&cleared, collection, key, id).get(field).is_none(), "{object_type} {field} cleared");
        }
    }
    // An empty tag list is stored as authored (only null clears), so an
    // authored `[]` is restorable.
    let (emptied, _) = applied(&model, &annotate("Element", "pipe:P-1", "tags", Some(&json!(["discharge"])), &json!([])));
    assert_eq!(find(&emptied, "pipe_segments", "id", "pipe:P-1")["tags"], json!([]));
    let restored = round_trip(&model, &annotate("EquipmentInterface", "equipment-interface:E-001", "tags", Some(&json!([])), &json!(["x"])), |next| {
        annotate("EquipmentInterface", "equipment-interface:E-001", "tags", find(next, "equipment_interfaces", "id", "equipment-interface:E-001").get("tags"), &json!([]))
    });
    assert_eq!(find(&restored, "equipment_interfaces", "id", "equipment-interface:E-001")["tags"], json!(["x"]));

    let record = json!({"source_name": "Invented catalogue", "redistribution_status": "private_only", "review_status": "pending"});
    for (object_type, id, collection) in [
        ("Material", "material:steel", "materials"),
        ("Section", "section:std", "sections"),
        ("Component", "component:C-1", "components"),
        ("Support", "support:S-1", "supports"),
    ] {
        let existing = find(&model, collection, "id", id).get("provenance_record").cloned();
        let next = round_trip(&model, &annotate(object_type, id, "provenance_record", existing.as_ref(), &record), |next| {
            let now = find(next, collection, "id", id).get("provenance_record").cloned();
            annotate(object_type, id, "provenance_record", now.as_ref(), existing.as_ref().unwrap_or(&Value::Null))
        });
        assert_eq!(find(&next, collection, "id", id)["provenance_record"], record);
        let (cleared, _) = applied(&next, &annotate(object_type, id, "provenance_record", Some(&record), &Value::Null));
        assert!(find(&cleared, collection, "id", id).get("provenance_record").is_none());
    }
}

#[test]
fn annotations_refuse_malformed_values_wrong_kinds_and_wrong_targets() {
    let model = every_slot();
    let anchor = json!(["anchor"]);
    let tags = Some(&anchor);
    // The validator names malformed tag lists, blank notes and provenance records.
    for (operation, code) in [
        (annotate("Node", "node:N-1", "tags", tags, &json!(["dup", "dup"])), "MODEL-TAGS-INVALID"),
        (annotate("Node", "node:N-1", "tags", tags, &json!([" padded"])), "MODEL-TAGS-INVALID"),
        (annotate("Node", "node:N-1", "tags", tags, &json!(["x".repeat(65)])), "MODEL-TAGS-INVALID"),
        (annotate("Node", "node:N-3", "notes", None, &json!("   ")), "MODEL-NOTES-INVALID"),
        (annotate("Section", "section:std", "provenance_record", find(&model, "sections", "id", "section:std").get("provenance_record"),
            &json!({"source_name": "Invented", "review_status": "approved"})), "MODEL-PROVENANCE-RECORD-INVALID"),
        (annotate("Support", "support:S-1", "provenance_record", find(&model, "supports", "id", "support:S-1").get("provenance_record"),
            &json!({"review_status": "pending"})), "MODEL-PROVENANCE-RECORD-INVALID"),
    ] {
        let outcome = refused(&model, &operation);
        assert!(blocking(&outcome).iter().all(|found| *found == code), "{:?}", codes(&outcome));
        let preview = validate_operation(&model, &operation, Some(&claim(&model)));
        assert!(blocking(&preview).contains(&code), "the preview runs the validator: {:?}", codes(&preview));
    }
    // A quarantined disposition applies with the validator's warning.
    let (_, outcome) = applied(&model, &annotate("Support", "support:S-1", "provenance_record",
        find(&model, "supports", "id", "support:S-1").get("provenance_record"),
        &json!({"source_name": "Invented", "review_status": "quarantined"})));
    assert_eq!(codes(&outcome), vec![("MODEL-PROVENANCE-QUARANTINE", "warning")]);
    // Payload JSON types.
    for (field, value) in [("tags", json!("one")), ("tags", json!([1])), ("notes", json!(["x"])), ("provenance_record", json!("x"))] {
        let object_type = if field == "provenance_record" { "Material" } else { "Node" };
        let id = if field == "provenance_record" { "material:steel" } else { "node:N-3" };
        let current = if field == "provenance_record" { find(&model, "materials", "id", id).get(field) } else { None };
        let outcome = refused(&model, &annotate(object_type, id, field, current, &value));
        assert_eq!(blocking(&outcome), vec!["OP-ANNOTATION-PAYLOAD-INVALID"], "{field}");
    }
    let mut raw = annotate("Node", "node:N-3", "notes", None, &json!("x"));
    raw["change"]["after"] = json!("not json");
    assert_eq!(blocking(&refused(&model, &raw)), vec!["OP-ANNOTATION-PAYLOAD-INVALID"]);
    // Nothing to apply.
    let outcome = refused(&model, &annotate("Node", "node:N-3", "notes", None, &Value::Null));
    assert_eq!(blocking(&outcome), vec!["OP-ANNOTATION-UNCHANGED"]);
    // A stale before.
    let outcome = refused(&model, &annotate("Node", "node:N-1", "tags", Some(&json!(["other"])), &json!(["x"])));
    assert_eq!(blocking(&outcome), vec!["OP-STALE-BEFORE-VALUE"]);
    // The family's change kind is required.
    let mut wrong = annotate("Load", "load:L-1", "notes", Some(&json!("Invented case.")), &json!("Edited."));
    wrong["change"]["change_kind"] = json!("set_field");
    assert_eq!(blocking(&refused(&model, &wrong)), vec!["OP-RICH-KIND-INVALID"]);
    // Nodes and pipes carry no provenance_record.
    let outcome = refused(&model, &annotate("Node", "node:N-3", "provenance_record", None, &json!({"source_name": "x"})));
    assert_eq!(blocking(&outcome), vec!["OP-FIELD-PATH-UNSUPPORTED"]);
    // Unknown targets.
    let outcome = refused(&model, &annotate("Project", "project:other", "notes", None, &json!("x")));
    assert_eq!(blocking(&outcome), vec!["OP-TARGET-NOT-FOUND"]);
    let outcome = refused(&model, &annotate("Assumption", "assumption:absent", "notes", None, &json!("x")));
    assert_eq!(blocking(&outcome), vec!["OP-TARGET-NOT-FOUND"]);
}

fn delete_pipe(model: &Value, id: &str) -> Value {
    let pipe = find(model, "pipe_segments", "id", id);
    let display = format!("{}; {}->{}; material={}", pipe["label"].as_str().unwrap(), pipe["from"].as_str().unwrap(),
        pipe["to"].as_str().unwrap(), pipe["material"].as_str().unwrap());
    intent("delete", "Element", id, "delete_pipe_run", "pipe_segments", &display, "not_present")
}

/// Deleting a pipe removes it from its line in the same operation, so a lined
/// pipe can be deleted; the constraint that names it keeps the id and warns.
#[test]
fn deleting_a_lined_pipe_removes_it_from_its_line() {
    let model = every_slot();
    let (next, outcome) = applied(&model, &delete_pipe(&model, "pipe:P-1"));
    assert_eq!(find(&next, "lines", "id", "line:L-100")["pipe_refs"], json!([]));
    assert_eq!(find(&next, "constraints", "constraint_id", "constraint:K-001")["target_refs"], json!(["pipe:P-1"]));
    assert_eq!(codes(&outcome), vec![("MODEL-ASSUMPTION-SUBJECT-MISSING", "warning")]);
    let line_rows: Vec<_> = outcome.diff_preview.iter().filter(|row| row.object_type == "Line").collect();
    assert_eq!(line_rows.len(), 1);
    assert_eq!(line_rows[0].entity_ref, "line:L-100");
    assert_eq!(line_rows[0].change_kind, "update_line");
}

/// Deleting an entity named by an assumption, constraint or equipment
/// interface is never blocked; the validator warns of the missing subject.
#[test]
fn deleting_a_named_entity_is_never_blocked() {
    let mut model = every_slot();
    model["nodes"].as_array_mut().unwrap().push(json!({"id": "node:N-9", "label": "Free", "position": {"x": 9.0, "y": 0.0, "z": 0.0}, "provenance": "invented_example"}));
    model["equipment_interfaces"][0]["node_ref"] = json!("node:N-9");
    model["constraints"][0]["target_refs"] = json!(["component:C-1", "node:N-9"]);
    let (_, outcome) = applied(&model, &intent("delete", "Node", "node:N-9", "delete_node", "nodes", "Free; x=9; y=0; z=0", "not_present"));
    assert_eq!(codes(&outcome), vec![("MODEL-ASSUMPTION-SUBJECT-MISSING", "warning"), ("MODEL-ASSUMPTION-SUBJECT-MISSING", "warning")]);

    let component = find(&model, "components", "id", "component:C-1").clone();
    let (_, outcome) = applied(&model, &intent("delete", "Component", "component:C-1", "delete_component", "components", &canonical_json(&component), "deleted"));
    assert_eq!(codes(&outcome), vec![("MODEL-ASSUMPTION-SUBJECT-MISSING", "warning")]);

    // The support is named by an assumption.
    let (_, outcome) = applied(&model, &intent("delete", "Support", "support:S-1", "delete_support", "supports", "Anchor support", "not_present"));
    assert_eq!(codes(&outcome), vec![("MODEL-ASSUMPTION-SUBJECT-MISSING", "warning")]);

    // A material named only in a provenance record or a tag is not referenced.
    let mut tagged = model.clone();
    tagged["materials"].as_array_mut().unwrap().push(json!({"id": "material:spare", "label": "Spare", "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "provenance": "invented_example"}));
    tagged["nodes"][0]["tags"] = json!(["material:spare"]);
    tagged["unresolved_assumptions"][0]["affected_refs"] = json!(["material:spare"]);
    let spare = find(&tagged, "materials", "id", "material:spare").clone();
    let (_, outcome) = applied(&tagged, &intent("delete", "Material", "material:spare", "delete_material", "materials", &canonical_json(&spare), "deleted"));
    assert_eq!(codes(&outcome), vec![("MODEL-ASSUMPTION-SUBJECT-MISSING", "warning")]);
}

fn geometry_intent(model: &Value, change_kind: &str, before: String, payload: &Value) -> Value {
    let mut operation = intent("modify", "Element", "pipe:P-1", change_kind, "pipe_segments", &before, &payload.to_string());
    let _ = model;
    operation["change"]["field_label"] = json!("Geometry");
    operation
}

/// Split: the new pipe inherits the source's tags and notes and joins its
/// line; the line, constraint and equipment interface never block the split.
#[test]
fn split_carries_annotations_and_joins_the_source_line() {
    let mut model = every_slot();
    model["load_cases"][0]["primitive_loads"] = json!([]);
    model["components"] = json!([]);
    let source = find(&model, "pipe_segments", "id", "pipe:P-1").clone();
    let payload = json!({"pipe_ref": "pipe:P-1", "fraction": 0.5,
        "new_node": {"id": "node:N-1a", "label": "Split", "provenance": "user"},
        "new_pipe": {"id": "pipe:P-1a", "label": "Run (2)", "provenance": "user"}});
    let operation = geometry_intent(&model, "split_pipe_run", canonical_json(&source), &payload);
    let (next, outcome) = applied(&model, &operation);
    let second = find(&next, "pipe_segments", "id", "pipe:P-1a");
    assert_eq!(second["tags"], source["tags"]);
    assert_eq!(second["notes"], source["notes"]);
    assert_eq!(find(&next, "lines", "id", "line:L-100")["pipe_refs"], json!(["pipe:P-1", "pipe:P-1a"]));
    assert_eq!(find(&next, "constraints", "constraint_id", "constraint:K-001")["target_refs"], json!(["pipe:P-1"]), "the source id survives");
    assert!(outcome.diff_preview.iter().any(|row| row.object_type == "Line" && row.entity_ref == "line:L-100"));
}

/// Copy: the copied pipe keeps its annotations and joins no line.
#[test]
fn copy_joins_no_line() {
    let mut model = every_slot();
    model["load_cases"][0]["primitive_loads"] = json!([]);
    model["supports"] = json!([]);
    model["components"] = json!([]);
    let pipes = vec![find(&model, "pipe_segments", "id", "pipe:P-1").clone()];
    let nodes: Vec<Value> = ["node:N-1", "node:N-2"].iter().map(|id| find(&model, "nodes", "id", id).clone()).collect();
    let before = canonical_json(&json!({"nodes": nodes, "pipe_segments": pipes}));
    let payload = json!({"mode": "copy", "pipe_refs": ["pipe:P-1"],
        "transform": {"kind": "translate", "translation": {"x": 0.0, "y": 0.0, "z": 5.0, "unit": "m"}},
        "copy_nodes": [{"source_ref": "node:N-1", "id": "node:N-1c", "label": "Copy A", "provenance": "user"},
                       {"source_ref": "node:N-2", "id": "node:N-2c", "label": "Copy B", "provenance": "user"}],
        "copy_pipes": [{"source_ref": "pipe:P-1", "id": "pipe:P-1c", "label": "Copy", "provenance": "user"}]});
    let (next, _) = applied(&model, &geometry_intent(&model, "transform_pipe_run", before, &payload));
    assert_eq!(find(&next, "pipe_segments", "id", "pipe:P-1c")["tags"], json!(["discharge"]));
    assert_eq!(next["lines"], model["lines"], "a copy is a new run and joins no line");
}

/// Insert bend at corner: the arc joins the line of the incoming pipe p_in.
#[test]
fn bend_arc_joins_the_incoming_pipe_line() {
    let mut model = every_slot();
    model["load_cases"][0]["primitive_loads"] = json!([]);
    model["components"] = json!([]);
    model["unresolved_assumptions"][0]["affected_refs"] = json!(["node:N-2", "pipe:P-2"]);
    let pipes: Vec<&Value> = model["pipe_segments"].as_array().unwrap().iter().collect();
    let corner = find(&model, "nodes", "id", "node:N-2");
    let before = canonical_json(&json!({"node": corner, "pipes": pipes, "markers": []}));
    let payload = json!({
        "radius": {"value": 0.25, "unit": "m"},
        "flexibility_factor": {"value": 2.0, "unit": "none"},
        "flexibility_source_reference": "invented user value",
        "new_nodes": [{"id": "node:B1", "label": "Bend start", "provenance": "user"}, {"id": "node:B2", "label": "Bend end", "provenance": "user"}],
        "new_pipe": {"id": "pipe:ARC", "label": "Arc", "provenance": "user"},
        "component": {"id": "component:bend", "label": "Bend", "provenance": "user", "geometry_source_reference": "inserted by the user"}
    });
    let operation = intent("modify", "Node", "node:N-2", "insert_bend_at_corner", "bend_at_corner", &before, &payload.to_string());
    let (next, outcome) = applied(&model, &operation);
    assert_eq!(find(&next, "lines", "id", "line:L-100")["pipe_refs"], json!(["pipe:P-1", "pipe:ARC"]));
    let arc = find(&next, "pipe_segments", "id", "pipe:ARC");
    assert_eq!(arc["tags"], json!(["discharge"]), "the arc is p_in's clone");
    // The deleted corner stays named by the assumption, with a warning.
    assert_eq!(codes(&outcome), vec![("MODEL-ASSUMPTION-SUBJECT-MISSING", "warning")]);
}

/// The applier golden: one applied document per new operation kind, each a
/// complete document the schema pytest validates. Bless with
/// `MODEL_DATA_GOLDEN_BLESS=1`.
#[test]
fn applier_golden_records_one_document_per_new_operation_kind() {
    let mut base = every_slot();
    base["project"]["id"] = json!("project:applier-golden");
    let mut entries = Vec::new();
    let mut push = |name: &str, model: &Value, operation: Value| {
        let (next, _) = applied(model, &operation);
        entries.push(json!({"operation": name, "change_kind": operation["change"]["change_kind"], "applied_model": next}));
        next
    };
    for (create, update, delete, object_type, collection, key, record) in families() {
        let id = record[key].as_str().unwrap().to_string();
        let created = push(create, &base, record_op(create, object_type, collection, &id, None, Some(&record)));
        if let Some(update) = update {
            let mut changed = record.clone();
            changed["notes"] = json!("Edited invented record.");
            push(update, &created, record_op(update, object_type, collection, &id, Some(&record), Some(&changed)));
        }
        push(delete, &created, record_op(delete, object_type, collection, &id, Some(&record), None));
    }
    push("tags", &base, annotate("Element", "pipe:P-2", "tags", None, &json!(["invented"])));
    push("notes", &base, annotate("Project", "project:applier-golden", "notes", base["project"].get("notes"), &json!("Edited project note.")));
    push("provenance_record", &base, annotate("Support", "support:S-1", "provenance_record",
        find(&base, "supports", "id", "support:S-1").get("provenance_record"),
        &json!({"source_name": "Invented vendor sheet (revised)", "source_location": "invented://catalogue/rev-2",
                "source_license": "invented-permissive", "contributor": "Invented contributor",
                "contributor_certification": "invented-attestation", "redistribution_status": "public_permissive",
                "review_status": "pending"})));
    let golden = json!({
        "document_kind": "openpipestress.test.applier_model_data_golden",
        "description": "One applied model document per W-13 model-data operation kind (design model-document-0.7.0.md §5.2). Invented data. Regenerate with MODEL_DATA_GOLDEN_BLESS=1 cargo test --test model_data_authoring.",
        "entries": entries,
    });
    let text = serde_json::to_string_pretty(&golden).unwrap() + "\n";
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_PATH);
    if std::env::var_os("MODEL_DATA_GOLDEN_BLESS").is_some() {
        std::fs::write(&path, &text).unwrap();
        return;
    }
    let committed = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(committed == text, "{} differs from a fresh emission; regenerate with MODEL_DATA_GOLDEN_BLESS=1 and review the diff", path.display());
}

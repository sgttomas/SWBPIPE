//! Model document 0.9.0, slice S3a (DEL-02-01 Work `model-document-0.9.0.md`
//! §11): condition-set create, update and delete with the reference guard, pipe
//! assignment, a case's condition use and `create_load_case`'s use, alone and in
//! atomic batches, with the solve's judgement classified on every candidate.
//!
//! Expectations come from the contract text and, for the judgement, from the
//! solve itself on the same candidate (product_physics' public preview and the
//! public solve); the applier computes no temperature difference. Every document
//! is an invented inline fixture at the public `MODEL_SCHEMA_VERSION`.
use open_pipe_stress_operation_applier::document_validator::{validate_model_document, SLOT_SHAPE};
use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_operation, OperationOutcome,
};
use open_pipe_stress_product_physics::operating_conditions::preview_operating_conditions;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};

const PROV: &str = "invented_s3a_condition_set_test_input";

/// The eight staged findings of §11, by code.
const STAGED: [&str; 8] = [
    "OPERATING_CONDITION_SET_MISSING",
    "OPERATING_CONDITION_KEY_MISSING",
    "OPERATING_CONDITION_VALUE_MISSING",
    "OPERATING_CONDITION_INSTALLATION_TEMPERATURE_MISSING",
    "LOAD_CASE_CONDITION_TEMPERATURE_CONFLICT",
    "LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT",
    "LOAD_CASE_CONDITION_WIND_INPUT_MISSING",
    "EQUIVALENT_STATIC_INPUT_MISSING",
];

/// (1, 2^-1023, 0), a skewed direction with a subnormal canonical
/// component: admitted by the document rules, refused by name at the solve.
static RANGE_DIRECTION: std::sync::LazyLock<Value> = std::sync::LazyLock::new(|| json!({"x": 1.0, "y": f64::from_bits(1u64 << 51), "z": 0.0}));

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// A pressure-free document with three pipes and no condition member.
fn base() -> Value {
    let node = |id: &str, x: f64| json!({"id": id, "label": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV});
    let pipe = |id: &str, from: &str, to: &str| json!({"id": id, "from": from, "to": to, "material": "mat",
        "y_reference": {"x": 0, "y": 0, "z": 1}, "section": {"outside_diameter": q(0.2, "m"), "wall_thickness": q(0.01, "m")}, "provenance": PROV});
    json!({
        "schema_version": MODEL_SCHEMA_VERSION,
        "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": "invented:rt-g5-s3a", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC", "stress": "Pa"}},
        "nodes": [node("N0", 0.0), node("N1", 3.0), node("N2", 6.0), node("N3", 9.0)],
        "pipe_segments": [pipe("P1", "N0", "N1"), pipe("P2", "N1", "N2"), pipe("P3", "N2", "N3")],
        "materials": [{"id": "mat", "elastic_modulus": q(2.0e11, "Pa"), "shear_modulus": q(8.0e10, "Pa"),
                       "thermal_expansion_coefficient": q(1.2e-5, "1/degC"), "provenance": PROV}],
        "supports": [
            {"id": "anchor", "node": "N0", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
            {"id": "stop", "node": "N1", "family": "line_stop", "restraints": ["UZ"], "provenance": PROV},
            {"id": "spring", "node": "N3", "family": "spring", "restraints": ["UX"], "stiffness": {"dof": "UX", "value": q(1.0e7, "N/m")}, "provenance": PROV},
        ],
        "load_cases": [
            {"id": "SUS", "primitive_loads": [], "provenance": PROV},
            {"id": "WIND", "primitive_loads": [], "provenance": PROV,
             "equivalent_static": {"wind": {"pressure": q(1200.0, "Pa"), "shape_factor": q(0.7, "1"), "direction": "global_y"}, "provenance": PROV}},
        ],
        "combinations": [],
    })
}

/// A complete set: one condition, an installation temperature, added weight and
/// a wind mark.
fn set_a() -> Value {
    json!({"id": "cs:a", "label": "Line A", "provenance": PROV, "installation_temperature": q(20.0, "degC"),
        "conditions": [{"key": "OPE1", "temperature": q(150.0, "degC")}],
        "added_weight": {"magnitude": q(-50.0, "N/m"), "direction": "global_z"}, "exposure": {"wind": true}})
}

fn set_b() -> Value {
    json!({"id": "cs:b", "provenance": PROV, "installation_temperature": q(70.0, "degF"),
        "conditions": [{"key": "OPE1", "temperature": q(212.0, "degF")}], "exposure": {"wind": false}})
}

fn bytes(model: &Value) -> String {
    canonical_json(model)
}

fn claim(model: &Value) -> Value {
    json!({"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_scope": "model_payload",
        "value": format!("sha256:{}", sha256_hex(&canonical_json(model)))})
}

#[allow(clippy::too_many_arguments)]
fn op_with(kind: &str, change_kind: &str, object: &str, target: &str, path: &str, before: &str, after: &str, unit: &str, dimension: &str) -> Value {
    let token = format!("{change_kind}:{target}:{path}:{}", sha256_hex(&format!("{before}|{after}"))[..12].to_string());
    json!({
        "operation_id": format!("operation:{token}"), "operation_kind": kind, "operation_status": "proposed", "author_type": "user",
        "target": {"object_type": object, "ref": target},
        "change": {"change_id": format!("change:{token}"), "change_kind": change_kind, "field_label": path, "field_path": path,
                   "before": before, "after": after, "unit": unit, "dimension": dimension, "source_note": "invented test input"},
        "validation": {"schema_validation": "not_run", "constraint_validation": "not_run", "unit_validation": "not_run",
                       "diff_preview_status": "not_generated", "application_status": "not_applied"},
        "audit_boundary": {"mutation_route": "structured_operations_only", "direct_model_mutation_allowed": false,
                           "requires_user_acceptance": true, "mutates_accepted_model_state": false},
        "professional_boundary": {"human_review_required": true, "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false, "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false, "software_makes_authentication_claim": false},
        "rationale": "Invented S3a condition-set operation",
    })
}

fn op(kind: &str, change_kind: &str, object: &str, target: &str, path: &str, before: &str, after: &str) -> Value {
    op_with(kind, change_kind, object, target, path, before, after, "none", "dimensionless")
}

fn find<'a>(model: &'a Value, collection: &str, id: &str) -> Option<&'a Value> {
    model.get(collection)?.as_array()?.iter().find(|record| record["id"] == id)
}

fn create_set(record: &Value) -> Value {
    op("create", "create_condition_set", "ConditionSet", record["id"].as_str().unwrap(), "condition_sets", "not_present", &record.to_string())
}

fn create_set_at(record: &Value, index: usize) -> Value {
    op("create", "create_condition_set", "ConditionSet", record["id"].as_str().unwrap(), &format!("condition_sets[{index}]"), "not_present", &record.to_string())
}

fn update_set(model: &Value, next: &Value) -> Value {
    let id = next["id"].as_str().unwrap();
    op("modify", "update_condition_set", "ConditionSet", id, "condition_sets", &canonical_json(find(model, "condition_sets", id).unwrap()), &next.to_string())
}

fn delete_set(model: &Value, id: &str) -> Value {
    op("delete", "delete_condition_set", "ConditionSet", id, "condition_sets", &canonical_json(find(model, "condition_sets", id).unwrap()), "not_present")
}

fn member_text(model: &Value, collection: &str, id: &str, member: &str, wrap: bool) -> String {
    match find(model, collection, id).and_then(|record| record.get(member)) {
        None => "not_present".into(),
        Some(value) if wrap => canonical_json(&json!({member: value})),
        Some(value) => canonical_json(value),
    }
}

fn assign(model: &Value, pipe: &str, set: Option<&str>) -> Value {
    let after = set.map(|id| json!({"condition_set_ref": id}).to_string()).unwrap_or_else(|| "not_present".into());
    op("modify", "assign_condition_set", "Element", pipe, "condition_set_ref", &member_text(model, "pipe_segments", pipe, "condition_set_ref", true), &after)
}

fn set_use(model: &Value, case: &str, using: &Value) -> Value {
    op("modify", "set_load_case_condition_use", "Load", case, "condition_use", &member_text(model, "load_cases", case, "condition_use", false), &using.to_string())
}

fn clear_use(model: &Value, case: &str) -> Value {
    op("modify", "clear_load_case_condition_use", "Load", case, "condition_use", &member_text(model, "load_cases", case, "condition_use", false), "not_present")
}

fn create_case(id: &str, extra: Value) -> Value {
    let mut payload = json!({"id": id, "label": id, "kind": "operating", "status": "draft", "provenance": PROV, "primitive_loads": []});
    for (key, value) in extra.as_object().unwrap() {
        payload[key] = value.clone();
    }
    op("create", "create_load_case", "Load", id, "load_cases", "not_present", &payload.to_string())
}

fn records(outcome: &OperationOutcome) -> Vec<(String, String, String)> {
    let mut found: Vec<_> = outcome.diagnostics.iter().map(|d| (d.code.clone(), d.severity.clone(), d.message.clone())).collect();
    found.sort();
    found
}

fn blocking(outcome: &OperationOutcome) -> Vec<String> {
    outcome.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect()
}

fn warnings(outcome: &OperationOutcome) -> Vec<(String, String)> {
    let mut found: Vec<_> = outcome.diagnostics.iter().filter(|d| d.severity == "warning").map(|d| (d.code.clone(), d.message.clone())).collect();
    found.sort();
    found
}

/// Validate, then apply: the same diagnostics, no document from validation, the
/// applied document from apply; the input never changes.
fn applied(model: &Value, intent: &Value) -> (Value, OperationOutcome) {
    let original = bytes(model);
    let preview = validate_operation(model, intent, Some(&claim(model)));
    assert!(preview.applied_model.is_none());
    let outcome = apply_operation(model, intent, Some(&claim(model)));
    assert_eq!(records(&preview), records(&outcome), "validate-only reports exactly what apply reports");
    assert_eq!(preview.diff_preview, outcome.diff_preview);
    let next = outcome.applied_model.clone().unwrap_or_else(|| panic!("not applied: {:?}", records(&outcome)));
    assert_eq!(outcome.validation.application_status, "applied_to_session_model");
    assert!(blocking(&outcome).is_empty(), "{:?}", records(&outcome));
    assert!(!validate_model_document(&next).iter().any(|d| d.is_blocking()));
    assert_eq!(bytes(model), original, "the input is never mutated");
    (next, outcome)
}

/// Validate and apply are both refused with `code` among the blocking codes and
/// no document; the input never changes. A refusal before the candidate exists
/// has no diff row (the validator's findings on a candidate keep the row it
/// judged, as for every applier edit).
fn refused(model: &Value, intent: &Value, code: &str) -> OperationOutcome {
    let original = bytes(model);
    let mut last = None;
    for outcome in [validate_operation(model, intent, Some(&claim(model))), apply_operation(model, intent, Some(&claim(model)))] {
        assert!(outcome.applied_model.is_none(), "{code}: applied unexpectedly");
        assert_ne!(outcome.validation.application_status, "applied_to_session_model");
        assert!(blocking(&outcome).iter().any(|found| found == code), "{code} not in {:?}", records(&outcome));
        last = Some(outcome);
    }
    assert_eq!(bytes(model), original);
    last.unwrap()
}

fn with_sets(sets: &[Value]) -> Value {
    let mut model = base();
    model["condition_sets"] = json!(sets);
    model
}

fn with_refs(mut model: Value, refs: &[(&str, &str)]) -> Value {
    for (pipe, set) in refs {
        let index = model["pipe_segments"].as_array().unwrap().iter().position(|p| p["id"] == *pipe).unwrap();
        model["pipe_segments"][index]["condition_set_ref"] = json!(set);
    }
    model
}

fn case_mut<'a>(model: &'a mut Value, id: &str) -> &'a mut Value {
    model["load_cases"].as_array_mut().unwrap().iter_mut().find(|c| c["id"] == id).unwrap()
}

// ------------------------------------------------------------------ records

/// Create, update and delete, each undone by its inverse to the prior document;
/// the diff rows carry the records' canonical JSON.
#[test]
fn create_update_and_delete_with_their_inverses() {
    let model = base();
    let (created, outcome) = applied(&model, &create_set(&set_a()));
    assert_eq!(created["condition_sets"], json!([set_a()]));
    assert_eq!(outcome.diff_preview.len(), 1);
    let row = &outcome.diff_preview[0];
    assert_eq!((row.entity_ref.as_str(), row.object_type.as_str(), row.field_path.as_str()), ("cs:a", "ConditionSet", "condition_sets"));
    assert_eq!((row.before.as_str(), row.after.clone()), ("not_present", canonical_json(&set_a())));
    assert!(warnings(&outcome).is_empty(), "no case uses conditions: {:?}", warnings(&outcome));

    let mut changed = set_a();
    changed["label"] = json!("Line A, revised");
    changed["conditions"][0]["temperature"] = q(160.0, "degC");
    let (updated, outcome) = applied(&created, &update_set(&created, &changed));
    assert_eq!(updated["condition_sets"], json!([changed.clone()]));
    assert_eq!(outcome.diff_preview[0].before, canonical_json(&set_a()));
    let (reverted, _) = applied(&updated, &update_set(&updated, &set_a()));
    assert_eq!(bytes(&reverted), bytes(&created), "update's inverse restores the document");

    // Delete the last set: the collection stays an empty array, and the
    // delete's inverse (create at the deleted index) restores the document.
    let (deleted, outcome) = applied(&created, &delete_set(&created, "cs:a"));
    assert_eq!(deleted["condition_sets"], json!([]));
    assert_eq!(outcome.diff_preview[0].after, "not_present");
    let (restored, _) = applied(&deleted, &create_set_at(&set_a(), 0));
    assert_eq!(bytes(&restored), bytes(&created));
    let mut without = deleted.clone();
    without.as_object_mut().unwrap().remove("condition_sets");
    assert_eq!(bytes(&without), bytes(&model), "create then delete returns the members, not the bytes");

    // A set that is not last is put back at its index.
    let two = with_sets(&[set_a(), set_b()]);
    let (one, _) = applied(&two, &delete_set(&two, "cs:a"));
    assert_eq!(one["condition_sets"], json!([set_b()]));
    let (back, _) = applied(&one, &create_set_at(&set_a(), 0));
    assert_eq!(bytes(&back), bytes(&two));
    refused(&one, &create_set_at(&set_a(), 2), "OP-DATA-INDEX-INVALID");

    // The delete's own diff row is enough to invert it: the row names the
    // deleted index, so [A, B] and [B, A] are told apart (review r1, P2).
    let inverse_from_row = |outcome: &OperationOutcome| {
        let row = &outcome.diff_preview[0];
        assert_eq!(row.after, "not_present");
        op("create", "create_condition_set", "ConditionSet", &row.entity_ref, &row.field_path, "not_present", &row.before)
    };
    for order in [[set_a(), set_b()], [set_b(), set_a()]] {
        let start = with_sets(&order);
        let (rest, outcome) = applied(&start, &delete_set(&start, "cs:a"));
        assert_eq!(rest["condition_sets"], json!([set_b()]), "both orders leave the same document");
        let index = order.iter().position(|record| record["id"] == "cs:a").unwrap();
        assert_eq!(outcome.diff_preview[0].field_path, format!("condition_sets[{index}]"));
        let (restored, _) = applied(&rest, &inverse_from_row(&outcome));
        assert_eq!(bytes(&restored), bytes(&start), "the row alone restores the original order");
    }
}

/// The record refusals of §11's table, and the validator's document-level codes
/// on the candidate with the solve's own messages.
#[test]
fn record_operations_refuse_by_name() {
    let model = with_sets(&[set_a()]);
    let mut wrong_id = set_b();
    wrong_id["id"] = json!("cs:other");
    let wrong = op("create", "create_condition_set", "ConditionSet", "cs:b", "condition_sets", "not_present", &wrong_id.to_string());
    refused(&model, &wrong, "OP-DATA-ID-MISMATCH");
    refused(&model, &op("create", "create_condition_set", "ConditionSet", "cs:b", "condition_sets", "not_present", "[1]"), "OP-DATA-PAYLOAD-INVALID");
    refused(&model, &op("create", "create_condition_set", "ConditionSet", "cs:b", "condition_sets", "not_present", "not json"), "OP-DATA-PAYLOAD-INVALID");
    let mut bad_id = set_b();
    bad_id["id"] = json!("1-bad");
    refused(&model, &create_set(&bad_id), "OP-DATA-ID-INVALID");
    refused(&model, &create_set(&set_a()), "OP-DATA-ID-COLLISION");
    let mut pipe_id = set_b();
    pipe_id["id"] = json!("P1");
    refused(&model, &create_set(&pipe_id), "OP-DATA-ID-COLLISION");
    let mut stale = create_set(&set_b());
    stale["change"]["before"] = json!(canonical_json(&set_b()));
    refused(&model, &stale, "OP-STALE-BEFORE-VALUE");
    let mut shape = create_set(&set_b());
    shape["change"]["unit"] = json!("m");
    refused(&model, &shape, "OP-DATA-SHAPE-INVALID");

    // Update.
    refused(&model, &op("modify", "update_condition_set", "ConditionSet", "cs:b", "condition_sets", &canonical_json(&set_b()), &set_b().to_string()), "OP-TARGET-NOT-FOUND");
    let mut stale = update_set(&model, &set_a());
    stale["change"]["before"] = json!(canonical_json(&set_b()));
    refused(&model, &stale, "OP-STALE-BEFORE-VALUE");
    let mut renamed = set_a();
    renamed["id"] = json!("cs:renamed");
    let mut rename = update_set(&model, &set_a());
    rename["change"]["after"] = json!(renamed.to_string());
    refused(&model, &rename, "OP-DATA-ID-MISMATCH");
    refused(&model, &update_set(&model, &set_a()), "OP-DATA-UNCHANGED");

    // Delete.
    let mut after = delete_set(&model, "cs:a");
    after["change"]["after"] = json!("{}");
    refused(&model, &after, "OP-DELETE-AFTER-VALUE-INVALID");
    let mut stale = delete_set(&model, "cs:a");
    stale["change"]["before"] = json!(canonical_json(&set_b()));
    refused(&model, &stale, "OP-STALE-BEFORE-VALUE");

    // The validator's codes and messages, which the solve also gives.
    let validator_cases: Vec<(&str, Box<dyn Fn(&mut Value)>)> = vec![
        ("OPERATING_CONDITION_PROVENANCE_MISSING", Box::new(|s| s["provenance"] = json!("  "))),
        ("OPERATING_CONDITION_KEY_DUPLICATE", Box::new(|s| s["conditions"] = json!([{"key": "K"}, {"key": "K"}]))),
        ("OPERATING_CONDITION_QUANTITY_INVALID", Box::new(|s| s["installation_temperature"] = q(20.0, "MPa"))),
        ("OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED", Box::new(|s| s["installation_temperature"] = q(2.0e6, "K"))),
        (SLOT_SHAPE, Box::new(|s| s["label"] = Value::Null)),
        (SLOT_SHAPE, Box::new(|s| s["colour"] = json!("red"))),
    ];
    for (code, mutate) in validator_cases {
        let mut record = set_b();
        mutate(&mut record);
        let outcome = refused(&model, &create_set(&record), code);
        let mut candidate = model.clone();
        candidate["condition_sets"].as_array_mut().unwrap().push(record);
        let expected: Vec<String> = validate_model_document(&candidate).into_iter().filter(|d| d.is_blocking()).map(|d| format!("{} (at {})", d.message, d.path)).collect();
        let got: Vec<String> = outcome.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.message.clone()).collect();
        assert_eq!(got, expected, "{code}: the validator's findings, verbatim");
    }
}

/// Delete is refused while any pipe references the set, naming each pipe.
#[test]
fn delete_is_refused_while_a_pipe_references_the_set() {
    let model = with_refs(with_sets(&[set_a(), set_b()]), &[("P1", "cs:a"), ("P3", "cs:a"), ("P2", "cs:b")]);
    let outcome = refused(&model, &delete_set(&model, "cs:a"), "OP-CONDITION-SET-IN-USE");
    let finding = outcome.diagnostics.iter().find(|d| d.code == "OP-CONDITION-SET-IN-USE").unwrap();
    assert_eq!(finding.affected_refs, ["cs:a", "P1", "P3"]);
    assert!(finding.message.contains("`P1`, `P3`") && !finding.message.contains("P2"), "{}", finding.message);
    // Clearing both references releases it.
    let (one, _) = applied(&model, &assign(&model, "P1", None));
    refused(&one, &delete_set(&one, "cs:a"), "OP-CONDITION-SET-IN-USE");
    let (none, _) = applied(&one, &assign(&one, "P3", None));
    let (deleted, _) = applied(&none, &delete_set(&none, "cs:a"));
    assert_eq!(deleted["condition_sets"], json!([set_b()]));
}

// ------------------------------------------------------------------ assignment

#[test]
fn assign_and_clear_a_pipes_set_with_their_inverses() {
    let model = with_sets(&[set_a(), set_b()]);
    let (assigned, outcome) = applied(&model, &assign(&model, "P2", Some("cs:a")));
    assert_eq!(assigned["pipe_segments"][1]["condition_set_ref"], "cs:a");
    let row = &outcome.diff_preview[0];
    assert_eq!((row.entity_ref.as_str(), row.object_type.as_str(), row.field_path.as_str()), ("P2", "Element", "condition_set_ref"));
    assert_eq!((row.before.as_str(), row.after.as_str()), ("not_present", r#"{"condition_set_ref":"cs:a"}"#));
    let (moved, outcome) = applied(&assigned, &assign(&assigned, "P2", Some("cs:b")));
    assert_eq!(outcome.diff_preview[0].before, r#"{"condition_set_ref":"cs:a"}"#);
    let (back, _) = applied(&moved, &assign(&moved, "P2", Some("cs:a")));
    assert_eq!(bytes(&back), bytes(&assigned));
    let (cleared, outcome) = applied(&assigned, &assign(&assigned, "P2", None));
    assert_eq!(outcome.diff_preview[0].after, "not_present");
    assert_eq!(bytes(&cleared), bytes(&model), "clear is assign's inverse");

    refused(&assigned, &assign(&assigned, "P2", Some("cs:a")), "OP-DATA-UNCHANGED");
    refused(&model, &assign(&model, "P2", None), "OP-DATA-UNCHANGED");
    refused(&model, &assign(&model, "P2", Some("cs:none")), "OPERATING_CONDITION_SET_UNRESOLVED");
    refused(&model, &assign(&model, "P9", Some("cs:a")), "OP-TARGET-NOT-FOUND");
    let mut stale = assign(&assigned, "P2", None);
    stale["change"]["before"] = json!("not_present");
    refused(&assigned, &stale, "OP-STALE-BEFORE-VALUE");
    for after in [r#""cs:a""#, r#"{"condition_set_ref":null}"#, r#"{"condition_set_ref":"cs:a","x":1}"#, "{}", "nope"] {
        let mut bad = assign(&model, "P2", Some("cs:a"));
        bad["change"]["after"] = json!(after);
        refused(&model, &bad, "OP-DATA-PAYLOAD-INVALID");
    }
    let mut shape = assign(&model, "P2", Some("cs:a"));
    shape["change"]["field_path"] = json!("condition_set");
    refused(&model, &shape, "OP-DATA-SHAPE-INVALID");
}

// ------------------------------------------------------------------ condition use

#[test]
fn set_and_clear_a_cases_use_with_their_inverses() {
    let model = with_refs(with_sets(&[set_a(), set_b()]), &[("P1", "cs:a"), ("P2", "cs:b"), ("P3", "cs:a")]);
    let using = json!({"temperature": "OPE1", "added_weight": true});
    let (set, outcome) = applied(&model, &set_use(&model, "SUS", &using));
    assert_eq!(find(&set, "load_cases", "SUS").unwrap()["condition_use"], using);
    assert!(warnings(&outcome).is_empty(), "fully resolved: {:?}", warnings(&outcome));
    let row = &outcome.diff_preview[0];
    assert_eq!((row.entity_ref.as_str(), row.object_type.as_str(), row.field_path.as_str()), ("SUS", "Load", "condition_use"));
    assert_eq!((row.before.as_str(), row.after.clone()), ("not_present", canonical_json(&using)));

    let replaced = json!({"temperature": "OPE1"});
    let (changed, _) = applied(&set, &set_use(&set, "SUS", &replaced));
    let (back, _) = applied(&changed, &set_use(&changed, "SUS", &using));
    assert_eq!(bytes(&back), bytes(&set), "set's inverse is set with the before use");
    let (cleared, outcome) = applied(&set, &clear_use(&set, "SUS"));
    assert_eq!(outcome.diff_preview[0].before, canonical_json(&using));
    assert_eq!(bytes(&cleared), bytes(&model), "clear restores the document");

    refused(&set, &set_use(&set, "SUS", &using), "OP-DATA-UNCHANGED");
    refused(&model, &clear_use(&model, "SUS"), "OP-DATA-UNCHANGED");
    refused(&model, &set_use(&model, "NOPE", &using), "OP-TARGET-NOT-FOUND");
    let mut stale = set_use(&set, "SUS", &replaced);
    stale["change"]["before"] = json!("not_present");
    refused(&set, &stale, "OP-STALE-BEFORE-VALUE");
    for after in ["[]", r#""OPE1""#, "nope"] {
        let mut bad = set_use(&model, "SUS", &using);
        bad["change"]["after"] = json!(after);
        refused(&model, &bad, "OP-DATA-PAYLOAD-INVALID");
    }
    let mut clear_with_value = clear_use(&set, "SUS");
    clear_with_value["change"]["after"] = json!(using.to_string());
    refused(&set, &clear_with_value, "OP-DATA-SHAPE-INVALID");
    let mut set_absent = set_use(&model, "SUS", &using);
    set_absent["change"]["after"] = json!("not_present");
    refused(&model, &set_absent, "OP-DATA-SHAPE-INVALID");
    // The validator's findings.
    refused(&model, &set_use(&model, "SUS", &json!({})), "LOAD_CASE_CONDITION_USE_EMPTY");
    refused(&model, &set_use(&model, "SUS", &json!({"added_weight": false})), "LOAD_CASE_CONDITION_USE_EMPTY");
    refused(&model, &set_use(&model, "SUS", &json!({"temperature": "OPE1", "colour": "red"})), SLOT_SHAPE);
    refused(&model, &set_use(&model, "SUS", &json!({"temperature": null})), SLOT_SHAPE);
}

/// `create_load_case` admits a use under the same judgement and carries it.
#[test]
fn create_load_case_admits_a_use_under_the_same_judgement() {
    let model = with_refs(with_sets(&[set_a()]), &[("P1", "cs:a"), ("P2", "cs:a"), ("P3", "cs:a")]);
    let using = json!({"temperature": "OPE1", "added_weight": true});
    let (created, outcome) = applied(&model, &create_case("OPE", json!({"condition_use": using})));
    assert_eq!(find(&created, "load_cases", "OPE").unwrap()["condition_use"], using, "the use is carried, never dropped");
    assert!(warnings(&outcome).is_empty(), "{:?}", warnings(&outcome));
    // Staged: a pipe without a set is a warning naming the pipe.
    let partial = with_refs(with_sets(&[set_a()]), &[("P1", "cs:a")]);
    let (_, outcome) = applied(&partial, &create_case("OPE", json!({"condition_use": {"temperature": "OPE1"}})));
    let staged = warnings(&outcome);
    assert_eq!(staged.iter().filter(|(code, _)| code == "OPERATING_CONDITION_SET_MISSING").count(), 2, "{staged:?}");
    assert!(staged.iter().any(|(_, message)| message.contains("`P2`")) && staged.iter().any(|(_, message)| message.contains("`P3`")));
    // Refusals: route and validator.
    refused(&model, &create_case("OPE", json!({"condition_use": {"contents_density": "OPE1"}})), "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED");
    refused(&model, &create_case("OPE", json!({"condition_use": {"pressure": "OPE1"}})), "LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT");
    refused(&model, &create_case("OPE", json!({"condition_use": {}})), "LOAD_CASE_CONDITION_USE_EMPTY");
    refused(&model, &create_case("OPE", json!({"condition_use": {"wind_exposure": false}})), "LOAD_CASE_CONDITION_USE_EMPTY");
}

/// Route admission: contents density and pressure-free pressure are refused;
/// every use on a document with a pressure contract is refused (S4).
#[test]
fn the_exact_route_and_contents_refusals() {
    let model = with_refs(with_sets(&[set_a()]), &[("P1", "cs:a"), ("P2", "cs:a"), ("P3", "cs:a")]);
    refused(&model, &set_use(&model, "SUS", &json!({"contents_density": "OPE1"})), "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED");
    refused(&model, &set_use(&model, "SUS", &json!({"pressure": "OPE1"})), "LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT");
    let both = refused(&model, &set_use(&model, "SUS", &json!({"temperature": "OPE1", "pressure": "OPE1", "contents_density": "OPE1"})), "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED");
    assert_eq!(blocking(&both).len(), 2, "each refused member is named: {:?}", records(&both));
    let mut exact = model.clone();
    exact["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    for using in [json!({"temperature": "OPE1"}), json!({"added_weight": true}), json!({"wind_exposure": true}), json!({"pressure": "OPE1"})] {
        let outcome = refused(&exact, &set_use(&exact, "SUS", &using), "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
        assert_eq!(blocking(&outcome), ["LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED"]);
        refused(&exact, &create_case("OPE", json!({"condition_use": using})), "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
    }
}

/// Review r1 (#93): on the exact route a declared operating comparison is
/// refused, and that refusal is never carried as info. Combination C compares
/// case L, which carries the only condition use; clearing L's use removes L's
/// route refusal, but C's unchanged refusal still blocks (route code, and C is
/// in the edit's scope as a combination naming L).
#[test]
fn an_exact_route_comparison_refusal_is_never_carried() {
    let mut exact = with_refs(with_sets(&[set_a()]), &[("P1", "cs:a"), ("P2", "cs:a"), ("P3", "cs:a")]);
    exact["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    let mut cases = exact["load_cases"].as_array().unwrap().clone();
    cases.push(json!({"id": "L", "label": "Operating", "kind": "operating", "status": "draft", "primitive_loads": [], "provenance": PROV, "condition_use": {"temperature": "OPE1"}}));
    exact["load_cases"] = json!(cases);
    exact["combinations"] = json!([{"id": "C", "basis": "result_state_subtraction", "minuend_id": "L", "subtrahend_id": "SUS", "difference_basis": "operating_minus_sustained", "provenance": PROV}]);
    for pipe in exact["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["corrosion_allowance"] = q(0.0, "m");
    }
    for case in exact["load_cases"].as_array_mut().unwrap() {
        case["pressure_regions"] = json!([]);
    }
    let outcome = refused(&exact, &clear_use(&exact, "L"), "LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED");
    assert!(!outcome.diagnostics.iter().any(|d| d.code == "LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED" && d.severity == "info"), "{:?}", records(&outcome));
    // Operating-case slice 1b: authoring a declared difference on the exact route stays refused by name.
    let declared = json!({"id": "C-NEW", "label": "New", "basis": "result_state_subtraction", "minuend_id": "L", "subtrahend_id": "SUS",
        "difference_basis": "operating_minus_sustained", "provenance": PROV});
    refused(&exact, &op("create", "create_combination", "Combination", "C-NEW", "combinations", "not_present", &declared.to_string()), "LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED");
}

// ------------------------------------------------------------------ S1 refusals kept

#[test]
fn every_s1_authoring_refusal_is_preserved() {
    let mut model = with_sets(&[set_a()]);
    model["combinations"] = json!([{"id": "C-SUB", "basis": "result_state_subtraction", "minuend_id": "WIND", "subtrahend_id": "SUS", "provenance": PROV}]);
    // connect_pipe_run still refuses condition_set_ref: assignment is its own operation.
    let mut nodes = model["nodes"].as_array().unwrap().clone();
    nodes.push(json!({"id": "N4", "position": {"x": 12.0, "y": 0.0, "z": 0.0}, "provenance": PROV}));
    model["nodes"] = json!(nodes);
    let connect = |extra: Value| {
        let mut payload = json!({"id": "P4", "label": "P4", "from": "N3", "to": "N4", "material": "mat", "y_reference": {"x": 0, "y": 0, "z": 1},
            "section": {"outside_diameter": q(0.2, "m"), "wall_thickness": q(0.01, "m")}, "provenance": PROV});
        for (key, value) in extra.as_object().unwrap() {
            payload[key] = value.clone();
        }
        op_with("connect", "connect_pipe_run", "Element", "P4", "pipe_segments", "not_present", &payload.to_string(), "m", "length")
    };
    let outcome = refused(&model, &connect(json!({"condition_set_ref": "cs:a"})), "OP-CONNECT-PIPE-PAYLOAD-INVALID");
    assert!(outcome.diagnostics.iter().any(|d| d.message.contains("condition_set_ref")));
    applied(&model, &connect(json!({})));
    // test_condition.
    refused(&model, &create_case("HT", json!({"test_condition": {"kind": "hydrostatic", "held_supports": "none"}})), "LOAD_CASE_TEST_CONDITION_UNSUPPORTED");
    refused(&model, &create_case("HT", json!({"condition_use": {"temperature": "OPE1"}, "test_condition": {"kind": "hydrostatic", "held_supports": "none"}})), "LOAD_CASE_TEST_CONDITION_UNSUPPORTED");
    // direction on supports.
    let support = |direction: Value| op("create", "create_support", "Support", "S-2", "supports", "not_present",
        &json!({"id": "S-2", "label": "Stop", "node": "N2", "family": "line_stop", "restraints": ["UY"], "direction": direction, "provenance": PROV}).to_string());
    // a skewed direction is
    // authored on the pressure-free route, written as entered.
    let authored = apply_operation(&model, &support(json!({"x": 0.0, "y": 3.0, "z": 4.0})), Some(&claim(&model)));
    let written = authored.applied_model.as_ref().unwrap_or_else(|| panic!("{:?}", records(&authored)));
    assert!(written["supports"].as_array().unwrap().iter().any(|s| s["id"] == "S-2" && s["direction"] == json!({"x": 0.0, "y": 3.0, "z": 4.0})));
    // An exactly axis-aligned direction is written too (`OP-CREATE-SUPPORT-PAYLOAD-INVALID`
    // for it is removed); it lowers to today's record at the solve.
    let axis = apply_operation(&model, &support(json!({"x": 0.0, "y": 1.0, "z": 0.0})), Some(&claim(&model)));
    assert!(axis.applied_model.is_some(), "{:?}", records(&axis));
    // The combination members.
    let combination = |extra: Value| {
        let mut payload = json!({"id": "C-NEW", "label": "New", "basis": "result_state_subtraction", "minuend_id": "WIND", "subtrahend_id": "SUS", "provenance": PROV});
        for (key, value) in extra.as_object().unwrap() {
            payload[key] = value.clone();
        }
        op("create", "create_combination", "Combination", "C-NEW", "combinations", "not_present", &payload.to_string())
    };
    // a declared difference is
    // authored on the pressure-free route, as authored, with `own` (which behaves as absence)
    // or `minuend_final`; an unknown or misplaced member is the document
    // validator's.
    for extra in [
        json!({"difference_basis": "operating_minus_sustained"}),
        json!({"difference_basis": "operating_minus_sustained", "subtrahend_support_state": "own"}),
        json!({"difference_basis": "operating_minus_sustained", "subtrahend_support_state": "minuend_final"}),
    ] {
        let (next, _) = applied(&model, &combination(extra.clone()));
        let created = next["combinations"].as_array().unwrap().iter().find(|c| c["id"] == "C-NEW").unwrap().clone();
        for (key, value) in extra.as_object().unwrap() {
            assert_eq!(&created[key], value, "{key} is written as authored");
        }
    }
    for (extra, code) in [
        (json!({"subtrahend_support_state": "minuend_final"}), "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_MISPLACED"),
        (json!({"subtrahend_support_state": "own"}), "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_MISPLACED"),
        (json!({"difference_basis": null}), "LOAD_COMBINATION_DIFFERENCE_BASIS_UNKNOWN"),
        (json!({"difference_basis": "sustained"}), "LOAD_COMBINATION_DIFFERENCE_BASIS_UNKNOWN"),
        (json!({"difference_basis": "operating_minus_sustained", "subtrahend_support_state": "held"}), "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNKNOWN"),
        (json!({"difference_basis": "operating_minus_sustained", "minuend_id": "C-SUB"}), "LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE"),
    ] {
        refused(&model, &combination(extra), code);
    }
    let misplaced = json!({"id": "C-NEW", "label": "New", "basis": "range_envelope", "operand_ids": ["WIND", "SUS"], "mode": "max",
        "difference_basis": "operating_minus_sustained", "provenance": PROV});
    refused(&model, &op("create", "create_combination", "Combination", "C-NEW", "combinations", "not_present", &misplaced.to_string()), "LOAD_COMBINATION_DIFFERENCE_BASIS_MISPLACED");
    applied(&model, &combination(json!({})));
    // The RT-A4 wiring: the unsigned bases and combination operands are authored.
    for payload in [
        json!({"id": "C-NEW", "label": "New", "basis": "absolute_sum", "terms": [{"load_case": "SUS", "factor": 1.0}], "provenance": PROV}),
        json!({"id": "C-NEW", "label": "New", "basis": "srss", "terms": [{"combination": "C-SUB", "factor": 1.0}], "provenance": PROV}),
        json!({"id": "C-NEW", "label": "New", "basis": "result_state_subtraction", "minuend_id": "C-SUB", "subtrahend_id": "SUS", "provenance": PROV}),
    ] {
        applied(&model, &op("create", "create_combination", "Combination", "C-NEW", "combinations", "not_present", &payload.to_string()));
    }
}

// ------------------------------------------------------------------ the judgement

fn staged_of(diagnostics: &Value) -> Vec<(String, String)> {
    let mut found: Vec<(String, String)> = diagnostics.as_array().unwrap().iter()
        .filter(|d| STAGED.contains(&d["code"].as_str().unwrap()))
        .map(|d| (d["code"].as_str().unwrap().to_string(), d["message"].as_str().unwrap().to_string()))
        .collect();
    found.sort();
    found
}

fn solve(model: &Value) -> Value {
    let envelope = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), PreviewSolverMode::SparseInteractive)
        .unwrap_or_else(|refusal| panic!("refused at the request boundary: {refusal}"));
    serde_json::to_value(envelope).unwrap()
}

/// Every staged finding at once: the applier's warnings are exactly the
/// preview's and the solve's findings on the same candidate, code and message.
#[test]
fn the_staged_warnings_equal_the_solves_findings_on_the_same_candidate() {
    let mut set_c = json!({"id": "cs:c", "provenance": PROV, "conditions": [{"key": "OPE1"}]});
    set_c["exposure"] = json!({"wind": true});
    let mut model = with_refs(with_sets(&[set_a(), set_c]), &[("P1", "cs:a"), ("P2", "cs:c")]);
    case_mut(&mut model, "SUS")["primitive_loads"] = json!([{"id": "SUS:t", "category": "thermal", "target": {"type": "element", "pipe": "P1"},
        "direction": "global_x", "magnitude": q(5.0, "K"), "dimension": "temperature_change", "provenance": PROV}]);
    let mut cases = model["load_cases"].as_array().unwrap().clone();
    cases.push(json!({"id": "OPE2", "primitive_loads": [], "provenance": PROV, "condition_use": {"temperature": "OPE2"}}));
    cases.push(json!({"id": "W2", "primitive_loads": [], "provenance": PROV, "condition_use": {"wind_exposure": true}}));
    cases.push(json!({"id": "WC", "primitive_loads": [], "provenance": PROV, "condition_use": {"wind_exposure": true},
        "equivalent_static": {"wind": {"pressure": q(1200.0, "Pa"), "shape_factor": q(0.7, "1"), "direction": "global_y", "exposed_pipe_refs": ["P1"]}, "provenance": PROV}}));
    model["load_cases"] = json!(cases);

    let (next, outcome) = applied(&model, &set_use(&model, "SUS", &json!({"temperature": "OPE1", "added_weight": true})));
    let got = warnings(&outcome);
    let preview = preview_operating_conditions(&next);
    assert_eq!(got, staged_of(&preview["diagnostics"]), "the preview's findings, code and message");
    assert_eq!(got, staged_of(&solve(&next)["diagnostics"]), "the solve's findings, code and message");
    for code in &STAGED[..7] {
        assert!(got.iter().any(|(found, _)| found == code), "{code} staged: {got:?}");
    }
    // Each names its case, pipe, set and key as the solve does.
    assert!(got.iter().any(|(code, m)| code == "OPERATING_CONDITION_SET_MISSING" && m.contains("`SUS`") && m.contains("`P3`")), "{got:?}");
    assert!(got.iter().any(|(code, m)| code == "OPERATING_CONDITION_KEY_MISSING" && m.contains("`OPE2`") && m.contains("`cs:a`")), "{got:?}");
    // The solve still refuses the cases by name.
    assert_eq!(preview["resolved"], json!(false));
}

fn with_case(mut model: Value, case: Value) -> Value {
    let mut cases = model["load_cases"].as_array().unwrap().clone();
    cases.push(case);
    model["load_cases"] = json!(cases);
    model
}

fn hydrostatic(extra: Value) -> Value {
    let mut case = json!({"id": "HT", "primitive_loads": [], "provenance": PROV, "test_condition": {"kind": "hydrostatic", "held_supports": "none"}});
    for (key, value) in extra.as_object().unwrap() {
        case[key] = value.clone();
    }
    case
}

/// The one diagnostic with `code`, which must be the permitted carried refusal:
/// severity `info`, the solve's message leading, the carried statement after it.
fn carried_info<'a>(outcome: &'a OperationOutcome, code: &str, solved: &Value) -> &'a open_pipe_stress_operation_applier::OutcomeDiagnostic {
    let found: Vec<_> = outcome.diagnostics.iter().filter(|d| d.code == code).collect();
    assert_eq!(found.len(), 1, "{:?}", records(outcome));
    assert_eq!(found[0].severity, "info", "{:?}", records(outcome));
    let message = solved["diagnostics"].as_array().unwrap().iter().find(|d| d["code"] == code).unwrap()["message"].as_str().unwrap();
    assert!(found[0].message.starts_with(message), "the solve's message leads: {}", found[0].message);
    let said = &found[0].message[message.len()..];
    assert!(said.contains("already carried this refusal before this edit"), "{said}");
    assert!(said.contains("outside this edit's dependency scope"), "{said}");
    assert!(said.contains("The solve still refuses it by name"), "{said}");
    found[0]
}

/// An unrelated carried refusal (a snubber; a skewed support direction) does
/// not block a set create, a pipe assignment or a case's use: each applies, the
/// refusal is reported as `info` with the solve's code and message, the solve
/// still refuses the applied document by name, and nothing in the outcome or
/// the applied document claims or records that the document solves.
#[test]
fn an_unrelated_carried_refusal_is_reported_and_does_not_block() {
    let mut snubbed = with_sets(&[set_a()]);
    snubbed["supports"][1] = json!({"id": "snub", "node": "N1", "family": "snubber", "restraints": ["UY"],
        "stiffness": {"dof": "UY", "value": q(1.0e7, "N/m")}, "provenance": PROV});
    let mut skewed = with_sets(&[set_a()]);
    // S-K1: a skewed spring now solves; a skewed direction outside §4.2's range is
    // still a solve-time refusal the document validator does not make.
    skewed["supports"][2]["direction"] = RANGE_DIRECTION.clone();
    for (model, code, support) in [(snubbed, "SNUBBER_DYNAMIC_ONLY_UNSUPPORTED", "snub"), (skewed, "SUPPORT_DIRECTION_RANGE_UNSUPPORTED", "spring")] {
        assert!(!validate_model_document(&model).iter().any(|d| d.is_blocking()), "the document opens and validates");
        assert!(solve(&model).to_string().contains(code), "the solve refuses the carried member");
        for intent in [
            create_set(&set_b()),
            assign(&model, "P1", Some("cs:a")),
            set_use(&model, "SUS", &json!({"temperature": "OPE1"})),
            create_case("OPE", json!({"condition_use": {"temperature": "OPE1"}})),
        ] {
            let (next, outcome) = applied(&model, &intent);
            let solved = solve(&next);
            let found = carried_info(&outcome, code, &solved);
            assert_eq!(found.affected_refs, [support]);
            assert!(solved["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == code && d["severity"] == "blocking"), "the solve still refuses it");
            assert_eq!(preview_operating_conditions(&next)["resolved"], json!(false));
            let outcome_text = serde_json::to_string(&outcome).unwrap().to_lowercase();
            for claim in ["solvable", "solves", "\"resolved\":true", "\"resolved\": true"] {
                assert!(!outcome_text.contains(claim), "the outcome claims {claim}");
                assert!(!bytes(&next).to_lowercase().contains(claim), "the applied document records {claim}");
            }
        }
    }
}

/// Editing the hydrostatic test case itself blocks: HT is the edit's target.
/// An edit of another case's use leaves HT outside its scope (HT uses no
/// conditions), so there the refusal is carried.
#[test]
fn an_edit_of_the_hydrostatic_case_blocks() {
    let model = with_case(with_sets(&[set_a()]), hydrostatic(json!({})));
    assert!(!validate_model_document(&model).iter().any(|d| d.is_blocking()), "the document opens and validates");
    let outcome = refused(&model, &set_use(&model, "HT", &json!({"temperature": "OPE1"})), "LOAD_CASE_TEST_CONDITION_UNSUPPORTED");
    assert!(warnings(&outcome).iter().any(|(code, _)| code == "OPERATING_CONDITION_SET_MISSING"), "staged findings still warn beside it");
    let (next, outcome) = applied(&model, &set_use(&model, "SUS", &json!({"temperature": "OPE1"})));
    carried_info(&outcome, "LOAD_CASE_TEST_CONDITION_UNSUPPORTED", &solve(&next));
}

/// HT uses conditions (temperature) and its pipes use set `cs:a`, so HT is an
/// effective case of every set: an update of `cs:a`, an assignment moving a
/// pipe to or from it, a set create and the delete of an unreferenced set all
/// block with HT's refusal. A use edit of another case does not reach HT.
#[test]
fn an_edit_of_a_set_feeding_the_hydrostatic_case_blocks() {
    let set_c = json!({"id": "cs:c", "provenance": PROV});
    let model = with_case(
        with_refs(with_sets(&[set_a(), set_b(), set_c]), &[("P1", "cs:a"), ("P2", "cs:a"), ("P3", "cs:b")]),
        hydrostatic(json!({"condition_use": {"temperature": "OPE1"}})),
    );
    assert!(!validate_model_document(&model).iter().any(|d| d.is_blocking()), "the document opens and validates");
    let code = "LOAD_CASE_TEST_CONDITION_UNSUPPORTED";
    let mut relabelled = set_a();
    relabelled["label"] = json!("Line A, relabelled");
    refused(&model, &update_set(&model, &relabelled), code);
    refused(&model, &assign(&model, "P1", Some("cs:b")), code);
    refused(&model, &assign(&model, "P1", None), code);
    refused(&model, &assign(&model, "P3", Some("cs:a")), code);
    refused(&model, &create_set(&json!({"id": "cs:d", "provenance": PROV})), code);
    refused(&model, &delete_set(&model, "cs:c"), code);
    let (next, outcome) = applied(&model, &set_use(&model, "SUS", &json!({"temperature": "OPE1"})));
    carried_info(&outcome, code, &solve(&next));
}

/// A refusal the before-document's judgement does not give blocks, even on a
/// member the edit does not touch: here the before-document fails the document
/// validator (a pipe names a set that does not exist), so its judgement gives
/// nothing, and the create that repairs it meets the skewed support as new.
#[test]
fn a_refusal_new_to_the_judgement_blocks() {
    let mut model = with_refs(with_sets(&[set_a()]), &[("P1", "cs:z")]);
    model["supports"][2]["direction"] = RANGE_DIRECTION.clone();
    assert!(validate_model_document(&model).iter().any(|d| d.is_blocking()), "the before-document does not validate");
    refused(&model, &create_set(&json!({"id": "cs:z", "provenance": PROV})), "SUPPORT_DIRECTION_RANGE_UNSUPPORTED");
    // The same create on the valid document meets the skew as carried.
    let mut skew = with_sets(&[set_a()]);
    skew["supports"][2]["direction"] = RANGE_DIRECTION.clone();
    let (next, outcome) = applied(&skew, &create_set(&json!({"id": "cs:z", "provenance": PROV})));
    carried_info(&outcome, "SUPPORT_DIRECTION_RANGE_UNSUPPORTED", &solve(&next));
    // Operations outside this slice are not judged: the same document still edits.
    let rename = op("modify", "set_field", "Node", "N1", "label", "N1", "Midspan");
    let outcome = apply_operation(&skew, &rename, Some(&claim(&skew)));
    assert!(outcome.applied_model.is_some(), "{:?}", records(&outcome));
}

/// Sets exist and none marks wind: the case's empty generated exposure is
/// reported by name as the solve reports it, with its source.
#[test]
fn an_explicitly_empty_wind_exposure_is_reported_by_name() {
    let still = json!({"id": "cs:still", "provenance": PROV, "exposure": {"wind": false}});
    let unmarked = json!({"id": "cs:unmarked", "provenance": PROV});
    let model = with_refs(with_sets(&[still, unmarked]), &[("P1", "cs:still"), ("P2", "cs:unmarked"), ("P3", "cs:still")]);
    let (next, outcome) = applied(&model, &set_use(&model, "WIND", &json!({"wind_exposure": true})));
    let preview = preview_operating_conditions(&next);
    assert_eq!(preview["resolved"], json!(true));
    let case = preview["cases"].as_array().unwrap().iter().find(|c| c["case_id"] == "WIND").unwrap();
    assert_eq!(case["wind_exposed_pipe_refs"], json!([]));
    let found: Vec<_> = outcome.diagnostics.iter().filter(|d| d.code == "EQUIVALENT_STATIC_INPUT_MISSING").collect();
    assert_eq!(found.len(), 1, "{:?}", records(&outcome));
    assert_eq!(found[0].severity, "warning");
    assert_eq!(found[0].affected_refs, ["WIND"]);
    assert!(found[0].message.contains("load case `WIND`") && found[0].message.contains("no condition set of its pipes marks exposure.wind"));
    let solved = solve(&next);
    let reported: Vec<_> = solved["diagnostics"].as_array().unwrap().iter()
        .filter(|d| d["code"] == "EQUIVALENT_STATIC_INPUT_MISSING" && d["affected_refs"] == json!(["WIND"])).collect();
    assert_eq!(reported.len(), 1, "{}", solved["diagnostics"]);
    let message = reported[0]["message"].as_str().unwrap();
    assert!(message.contains("exposed_pipe_refs or exposed_spans"), "{message}");
    assert!(found[0].message.starts_with(message), "the solve's message leads: {} / {message}", found[0].message);
    // Once a set marks wind, the exposure resolves and nothing is reported.
    let marked = with_refs(with_sets(&[set_a()]), &[("P1", "cs:a"), ("P2", "cs:a"), ("P3", "cs:a")]);
    let (_, outcome) = applied(&marked, &set_use(&marked, "WIND", &json!({"wind_exposure": true})));
    assert!(warnings(&outcome).is_empty(), "{:?}", warnings(&outcome));
}

// ------------------------------------------------------------------ batches and persistence

fn batch(model: &Value, operations: &[Value], apply: bool) -> Value {
    let request = json!({"batch_id": "batch:s3a", "operations": operations});
    if apply {
        apply_operation_batch(model, &request, Some(&claim(model)))
    } else {
        open_pipe_stress_operation_applier::validate_operation_batch(model, &request, Some(&claim(model)))
    }
}

fn batch_blocking(outcome: &Value) -> Vec<String> {
    outcome["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking").map(|d| d["code"].as_str().unwrap().to_string()).collect()
}

/// A batch mixing the operations with `create_load_case`, `create_node` and
/// `connect_pipe_run` applies all of them, equal to applying each in turn; a
/// refusal at any step publishes nothing.
#[test]
fn atomic_batches_apply_all_or_nothing() {
    let model = base();
    let node = op_with("create", "create_node", "Node", "N4", "nodes", "not_present",
        &json!({"id": "N4", "label": "N4", "position": {"x": 12.0, "y": 0.0, "z": 0.0}, "provenance": PROV}).to_string(), "m", "length");
    let connect = op_with("connect", "connect_pipe_run", "Element", "P4", "pipe_segments", "not_present",
        &json!({"id": "P4", "label": "P4", "from": "N3", "to": "N4", "material": "mat", "y_reference": {"x": 0, "y": 0, "z": 1},
            "section": {"outside_diameter": q(0.2, "m"), "wall_thickness": q(0.01, "m")}, "provenance": PROV}).to_string(), "m", "length");
    // Each step is built against the document the previous steps leave.
    let mut steps = vec![create_set(&set_a()), node, connect];
    let mut sequential = model.clone();
    for step in steps.clone() {
        sequential = applied(&sequential, &step).0;
    }
    for pipe in ["P1", "P2", "P3", "P4"] {
        let step = assign(&sequential, pipe, Some("cs:a"));
        sequential = applied(&sequential, &step).0;
        steps.push(step);
    }
    for step in [create_case("OPE", json!({"condition_use": {"temperature": "OPE1", "added_weight": true}})), set_use(&sequential, "WIND", &json!({"wind_exposure": true}))] {
        sequential = applied(&sequential, &step).0;
        steps.push(step);
    }
    let checked = batch(&model, &steps, false);
    assert_eq!(checked["validation"]["batch_validation_status"], "passed", "{:?}", batch_blocking(&checked));
    assert!(checked["applied_model"].is_null());
    let outcome = batch(&model, &steps, true);
    assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{:?}", batch_blocking(&outcome));
    assert_eq!(bytes(&outcome["applied_model"]), bytes(&sequential), "the batch equals the operations in turn");
    assert_eq!(find(&sequential, "pipe_segments", "P4").unwrap()["condition_set_ref"], "cs:a");

    // A refusal at the end, in the middle, or a stale step: nothing is applied.
    let original = bytes(&model);
    let last_refused = {
        let mut ops = steps.clone();
        ops.push(op("modify", "set_load_case_condition_use", "Load", "SUS", "condition_use", "not_present", r#"{"contents_density":"OPE1"}"#));
        ops
    };
    let middle_refused = {
        let mut ops = steps[..4].to_vec();
        ops.push(op("delete", "delete_condition_set", "ConditionSet", "cs:a", "condition_sets", &canonical_json(&set_a()), "not_present"));
        ops.extend(steps[4..].iter().cloned());
        ops
    };
    let stale = {
        let mut ops = steps.clone();
        ops[3]["change"]["before"] = json!(r#"{"condition_set_ref":"cs:a"}"#);
        ops
    };
    for (ops, code) in [(last_refused, "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED"), (middle_refused, "OP-CONDITION-SET-IN-USE"), (stale, "OP-STALE-BEFORE-VALUE")] {
        for apply in [false, true] {
            let outcome = batch(&model, &ops, apply);
            assert!(outcome["applied_model"].is_null(), "{code}: no document from a refused batch");
            assert_eq!(outcome["validation"]["batch_validation_status"], "blocked");
            assert!(batch_blocking(&outcome).iter().any(|found| found == code), "{code}: {:?}", batch_blocking(&outcome));
        }
        assert_eq!(bytes(&model), original);
    }
}

/// Every operation undone by its inverse, in reverse order, restores the
/// document byte for byte.
#[test]
fn undo_by_inverse_operations_restores_the_document() {
    // An empty collection, so the first create's inverse returns the bytes too.
    let model = with_sets(&[]);
    let mut document = model.clone();
    let mut inverses = Vec::new();
    let forward: Vec<Box<dyn Fn(&Value) -> Value>> = vec![
        Box::new(|_| create_set(&set_a())),
        Box::new(|_| create_set(&set_b())),
        Box::new(|d| assign(d, "P1", Some("cs:a"))),
        Box::new(|d| assign(d, "P2", Some("cs:b"))),
        Box::new(|d| assign(d, "P2", Some("cs:a"))),
        Box::new(|d| set_use(d, "SUS", &json!({"temperature": "OPE1"}))),
        Box::new(|d| set_use(d, "SUS", &json!({"added_weight": true}))),
    ];
    for step in &forward {
        let intent = step(&document);
        let before = document.clone();
        let (next, outcome) = applied(&document, &intent);
        let row = outcome.diff_preview[0].clone();
        // The inverse, built from the diff row's `before` alone.
        let inverse = match row.change_kind.as_str() {
            "create_condition_set" => op("delete", "delete_condition_set", "ConditionSet", &row.entity_ref, "condition_sets", &row.after, "not_present"),
            "assign_condition_set" => op("modify", "assign_condition_set", "Element", &row.entity_ref, "condition_set_ref", &row.after, &row.before),
            "set_load_case_condition_use" if row.before == "not_present" => op("modify", "clear_load_case_condition_use", "Load", &row.entity_ref, "condition_use", &row.after, "not_present"),
            "set_load_case_condition_use" => op("modify", "set_load_case_condition_use", "Load", &row.entity_ref, "condition_use", &row.after, &row.before),
            other => panic!("{other}"),
        };
        inverses.push((inverse, before));
        document = next;
    }
    let mut changed = set_a();
    changed["installation_temperature"] = q(15.0, "degC");
    let before = document.clone();
    let (next, outcome) = applied(&document, &update_set(&document, &changed));
    let row = outcome.diff_preview[0].clone();
    inverses.push((op("modify", "update_condition_set", "ConditionSet", "cs:a", "condition_sets", &row.after, &row.before), before));
    document = next;
    while let Some((inverse, expected)) = inverses.pop() {
        document = applied(&document, &inverse).0;
        assert_eq!(bytes(&document), bytes(&expected), "{}", inverse["change"]["change_kind"]);
    }
    assert_eq!(bytes(&document), bytes(&model));
}

/// A document authored through the operations saves and reopens byte for byte,
/// validates, and keeps its members through an unrelated edit.
#[test]
fn an_authored_document_saves_and_reopens_byte_for_byte() {
    let mut document = base();
    for step in [create_set(&set_a()), create_set(&set_b())] {
        document = applied(&document, &step).0;
    }
    for (pipe, set) in [("P1", "cs:a"), ("P2", "cs:b"), ("P3", "cs:a")] {
        let step = assign(&document, pipe, Some(set));
        document = applied(&document, &step).0;
    }
    let step = set_use(&document, "SUS", &json!({"temperature": "OPE1", "added_weight": true}));
    document = applied(&document, &step).0;
    document = applied(&document, &create_case("OPE", json!({"condition_use": {"temperature": "OPE1"}}))).0;

    let saved = serde_json::to_string_pretty(&document).unwrap();
    let reopened: Value = serde_json::from_str(&saved).unwrap();
    assert_eq!(serde_json::to_string_pretty(&reopened).unwrap(), saved, "save and reopen byte for byte");
    assert_eq!(bytes(&reopened), bytes(&document));
    assert!(validate_model_document(&reopened).is_empty(), "{:?}", validate_model_document(&reopened));
    let preview = preview_operating_conditions(&reopened);
    assert_eq!(preview["resolved"], json!(true), "{}", preview["diagnostics"]);

    let members = |d: &Value| json!({"sets": d["condition_sets"], "refs": d["pipe_segments"].as_array().unwrap().iter().map(|p| p["condition_set_ref"].clone()).collect::<Vec<_>>(),
        "uses": d["load_cases"].as_array().unwrap().iter().map(|c| c.get("condition_use").cloned()).collect::<Vec<_>>()});
    let rename = op("modify", "set_field", "Node", "N1", "label", "N1", "Midspan");
    let (renamed, _) = applied(&reopened, &rename);
    assert_eq!(bytes(&members(&renamed)), bytes(&members(&reopened)), "an unrelated edit keeps every member");
}

// ------------------------------------------------------------------ unchanged by canonical value

/// `OP-DATA-UNCHANGED` judges "the same value" on canonical JSON text (RFC 8785),
/// as the stale before-value check and the model hash do, so a model that reached
/// the engine through JavaScript (`JSON.parse` gives integer form) and an `after`
/// written in float or exponent form are one value (corpus case `s3a_14`). A real
/// change of value, unit or any other member still applies, stores the authored
/// `after` as parsed, and is undone (canonically, with the model hash) by the
/// inverse its own diff row states. Numerically different large values stay
/// different even where their canonical text coincides.
#[test]
fn unchanged_is_judged_on_canonical_value_across_number_forms() {
    let float_text = set_a().to_string();
    assert!(float_text.contains(r#""value":20.0"#), "serde writes the float form: {float_text}");
    let mut integer_form = set_a();
    integer_form["installation_temperature"]["value"] = json!(20);
    let integer_model = with_sets(&[integer_form.clone()]);
    let float_model = with_sets(&[set_a()]);
    let with_after = |model: &Value, after: String| {
        let mut intent = update_set(model, &set_a());
        intent["change"]["after"] = json!(after);
        intent
    };

    // Integer-form model (the JS lane), float- and exponent-form after: unchanged.
    refused(&integer_model, &with_after(&integer_model, float_text.clone()), "OP-DATA-UNCHANGED");
    let exponent_text = float_text.replace(r#""value":20.0"#, r#""value":2e1"#);
    assert_ne!(exponent_text, float_text);
    refused(&integer_model, &with_after(&integer_model, exponent_text), "OP-DATA-UNCHANGED");
    // Float-form model (the native lane), integer-form after: unchanged.
    refused(&float_model, &with_after(&float_model, integer_form.to_string()), "OP-DATA-UNCHANGED");
    refused(&float_model, &update_set(&float_model, &set_a()), "OP-DATA-UNCHANGED");
    // -0 and 0 have one canonical text and one model hash, so they are one value.
    let mut negative_zero = set_a();
    negative_zero["conditions"][0]["temperature"]["value"] = json!(-0.0);
    let zero_model = with_sets(&[negative_zero]);
    let mut zero = set_a();
    zero["conditions"][0]["temperature"]["value"] = json!(0);
    refused(&zero_model, &with_after(&zero_model, zero.to_string()), "OP-DATA-UNCHANGED");

    // Real changes still apply, with their inverses: a value, a unit, another member.
    for edit in [
        json!({"installation_temperature": q(20.5, "degC")}),
        json!({"installation_temperature": q(20.0, "degF")}),
        json!({"label": "Line A2"}),
    ] {
        let mut next = set_a();
        for (key, value) in edit.as_object().unwrap() {
            next[key] = value.clone();
        }
        let (changed, outcome) = applied(&integer_model, &update_set(&integer_model, &next));
        assert_eq!(outcome.diff_preview[0].after, canonical_json(&next));
        assert_eq!(find(&changed, "condition_sets", "cs:a").unwrap(), &next, "the authored after is stored as parsed");
        let mut inverse = update_set(&changed, &next);
        inverse["change"]["after"] = json!(outcome.diff_preview[0].before.clone());
        let (back, _) = applied(&changed, &inverse);
        assert_eq!(bytes(&back), bytes(&integer_model), "the diff row's inverse restores the canonical document");
        assert_eq!(claim(&back), claim(&integer_model), "and its model hash");
    }

    // Distinct large integers stay distinct: 2^53 + 1 against the float 2^53.
    let mut large = set_a();
    large["installation_temperature"]["value"] = json!(9_007_199_254_740_993_u64);
    let large_model = with_sets(&[large]);
    let mut rounded = set_a();
    rounded["installation_temperature"]["value"] = json!(9_007_199_254_740_992.0_f64);
    let outcome = apply_operation(&large_model, &with_after(&large_model, rounded.to_string()), Some(&claim(&large_model)));
    assert!(!blocking(&outcome).iter().any(|code| code == "OP-DATA-UNCHANGED"), "{:?}", records(&outcome));
    // ... and where their canonical text coincides: the integer 10^18 + 100 and the
    // float 10^18 + 128 both render `1000000000000000100`, but differ by 28.
    let mut integer_large = set_a();
    integer_large["installation_temperature"]["value"] = json!(1_000_000_000_000_000_100_u64);
    let mut float_large = set_a();
    float_large["installation_temperature"]["value"] = json!(1_000_000_000_000_000_128.0_f64);
    assert_eq!(canonical_json(&integer_large), canonical_json(&float_large), "the collision this guards");
    let collision_model = with_sets(&[integer_large]);
    let outcome = apply_operation(&collision_model, &with_after(&collision_model, float_large.to_string()), Some(&claim(&collision_model)));
    assert!(!blocking(&outcome).iter().any(|code| code == "OP-DATA-UNCHANGED"), "{:?}", records(&outcome));

    // A case's condition use: the same use is unchanged, a different one applies.
    let model = with_refs(integer_model.clone(), &[("P1", "cs:a")]);
    let using = json!({"temperature": "OPE1", "added_weight": true});
    let (set, _) = applied(&model, &set_use(&model, "SUS", &using));
    refused(&set, &set_use(&set, "SUS", &json!({"added_weight": true, "temperature": "OPE1"})), "OP-DATA-UNCHANGED");
    applied(&set, &set_use(&set, "SUS", &json!({"temperature": "OPE1"})));
}

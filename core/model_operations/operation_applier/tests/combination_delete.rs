//! Deleting combinations and combination terms of every closed basis, and the
//! restore form of `create_combination` (`combinations[i]`) and
//! `create_combination_term` (`terms.i`) that is each delete's exact inverse.
//!
//! - A record (or term) of flat `{load_case, factor}` terms keeps its historical
//!   before display; every other record (a subtraction, an envelope, a combination
//!   operand, no `terms`) and every other term is named by its canonical JSON.
//! - A combination another combination still names (a term of any factor,
//!   `minuend_id`, `subtrahend_id`, `operand_ids`) is refused by
//!   `OP-COMBINATION-DELETE-REFERENCED`; nothing cascades.
//! - The inverse is the stored record (or term) itself, never a display text:
//!   restored at its index, it reproduces the document exactly, value types and
//!   number forms included (canonical text alone would collapse them).
//! - The document validator still judges every result (cycles, operand rules).
//!
//! All data are invented.
use open_pipe_stress_operation_applier::{
    apply_operation, canonical_json, sha256_hex, validate_operation, OperationOutcome,
};
use serde_json::{json, Value};

const EVERY_SLOT: &str = include_str!("../../../../fixtures/model_document/invented_every_slot_model.json");

fn every_slot() -> Value {
    serde_json::from_str(EVERY_SLOT).unwrap()
}

fn model_hash(model: &Value) -> String {
    format!("sha256:{}", sha256_hex(&canonical_json(model)))
}

fn claim(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":model_hash(model)})
}

fn intent(kind: &str, target: &str, change_kind: &str, path: &str, before: &str, after: &str) -> Value {
    let token = format!("{change_kind}:{target}:{path}:{}", &sha256_hex(&format!("{before}|{after}"))[..12]);
    json!({
        "operation_id": format!("operation:{token}"), "operation_kind": kind, "operation_status": "proposed",
        "author_type": "user", "target": {"object_type": "Combination", "ref": target},
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
        "rationale": "Invented combination-delete test"
    })
}

fn blocking(outcome: &OperationOutcome) -> Vec<String> {
    outcome.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect()
}

/// Validate, then apply; both agree, the input never changes.
fn applied(model: &Value, intent: &Value) -> Value {
    let before = model.clone();
    let preview = validate_operation(model, intent, Some(&claim(model)));
    assert!(preview.applied_model.is_none());
    assert!(blocking(&preview).is_empty(), "preview blocked: {:?} {:?}", blocking(&preview), preview.diagnostics.iter().map(|d| d.message.clone()).collect::<Vec<_>>());
    let outcome = apply_operation(model, intent, Some(&claim(model)));
    assert_eq!(*model, before, "the input is never mutated");
    assert_eq!(outcome.validation.application_status, "applied_to_session_model", "{:?}", blocking(&outcome));
    let next = outcome.applied_model.clone().unwrap();
    assert_eq!(outcome.applied_model_backend_hash.as_deref(), Some(model_hash(&next).as_str()));
    next
}

/// Validate and apply are both refused with `code` blocking; the input never changes.
fn refused(model: &Value, intent: &Value, code: &str) -> OperationOutcome {
    let before = model.clone();
    let mut last = None;
    for outcome in [validate_operation(model, intent, Some(&claim(model))), apply_operation(model, intent, Some(&claim(model)))] {
        assert!(outcome.applied_model.is_none(), "{code}: applied unexpectedly");
        assert!(blocking(&outcome).iter().any(|found| found == code), "{code} not in {:?}", blocking(&outcome));
        last = Some(outcome);
    }
    assert_eq!(*model, before);
    last.unwrap()
}

fn combinations(model: &Value) -> &Vec<Value> {
    model["combinations"].as_array().unwrap()
}

fn index_of(model: &Value, id: &str) -> usize {
    combinations(model).iter().position(|c| c["id"] == id).unwrap()
}

fn record(model: &Value, id: &str) -> Value {
    combinations(model)[index_of(model, id)].clone()
}

/// The engine's term-factor display (`display_number`).
fn number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 { format!("{}", value as i64) } else { format!("{value}") }
}

/// The historical before display of a record of flat `{load_case, factor}` terms.
fn flat_display(record: &Value) -> Option<String> {
    let terms = record.get("terms")?.as_array()?;
    let mut items = Vec::new();
    for term in terms {
        let load_case = term.get("load_case")?.as_str()?.trim();
        let factor = term.get("factor")?.as_f64()?;
        if load_case.is_empty() || !factor.is_finite() {
            return None;
        }
        items.push(format!("{load_case} x {}", number(factor)));
    }
    let label = record["label"].as_str().unwrap_or("TBD").trim();
    let basis = record["basis"].as_str().unwrap_or("TBD").trim();
    let terms = if items.is_empty() { "none".to_string() } else { items.join("; ") };
    Some(format!("{}; {label}; basis={basis}; terms={terms}", record["id"].as_str().unwrap()))
}

fn delete(model: &Value, id: &str) -> Value {
    let stored = record(model, id);
    let before = flat_display(&stored).unwrap_or_else(|| canonical_json(&stored));
    intent("delete", id, "delete_combination", "combinations", &before, "not_present")
}

/// The exact inverse of a delete: the stored record, at its index.
fn restore(id: &str, index: usize, stored: &Value) -> Value {
    intent("create", id, "create_combination", &format!("combinations[{index}]"), "not_present", &serde_json::to_string(stored).unwrap())
}

fn delete_term(model: &Value, id: &str, index: usize) -> Value {
    let term = &record(model, id)["terms"][index];
    let before = match (term.get("load_case").and_then(Value::as_str), term.get("factor").and_then(Value::as_f64)) {
        (Some(load_case), Some(factor)) => format!("{load_case} x {}", number(factor)),
        _ => canonical_json(term),
    };
    intent("delete", id, "delete_combination_term", &format!("terms.{index}"), &before, "not_present")
}

fn restore_term(id: &str, index: usize, term: &Value) -> Value {
    intent("create", id, "create_combination_term", &format!("terms.{index}"), "not_present", &serde_json::to_string(term).unwrap())
}

/// Delete, check the record is gone, restore at its index, and require the exact
/// document back: equal values (number forms included), equal serialized bytes and
/// an equal model hash.
fn delete_and_restore(model: &Value, id: &str) -> Value {
    let index = index_of(model, id);
    let stored = record(model, id);
    let deleted = applied(model, &delete(model, id));
    assert!(combinations(&deleted).iter().all(|c| c["id"] != id));
    let back = applied(&deleted, &restore(id, index, &stored));
    assert_eq!(back, *model, "{id}: the restore reproduces the document");
    assert_eq!(serde_json::to_string(&back).unwrap(), serde_json::to_string(model).unwrap());
    assert_eq!(model_hash(&back), model_hash(model));
    deleted
}

#[test]
fn a_referenced_combination_is_refused_by_name_and_nothing_cascades() {
    let model = every_slot();
    let outcome = refused(&model, &delete(&model, "combination:C-EXP"), "OP-COMBINATION-DELETE-REFERENCED");
    let message = &outcome.diagnostics.iter().find(|d| d.code == "OP-COMBINATION-DELETE-REFERENCED").unwrap().message;
    assert!(message.contains("combination:C-ABS.terms.1"), "{message}");
    refused(&model, &delete(&model, "combination:C-ABS"), "OP-COMBINATION-DELETE-REFERENCED");

    // A reference of factor 0, a subtraction operand and an envelope operand all count.
    let mut zero = model.clone();
    zero["combinations"][index_of(&model, "combination:C-SRSS")]["terms"][0]["factor"] = json!(0.0);
    refused(&zero, &delete(&zero, "combination:C-ABS"), "OP-COMBINATION-DELETE-REFERENCED");
    for referrer in [
        json!({"id": "combination:C-SUB", "label": "S", "basis": "result_state_subtraction", "minuend_id": "combination:C-OP", "subtrahend_id": "load:L-1", "provenance": "invented_example"}),
        json!({"id": "combination:C-ENV", "label": "E", "basis": "range_envelope", "operand_ids": ["load:L-1", "combination:C-OP"], "mode": "max", "provenance": "invented_example"}),
    ] {
        let mut referenced = model.clone();
        referenced["combinations"].as_array_mut().unwrap().push(referrer.clone());
        let outcome = apply_operation(&referenced, &delete(&referenced, "combination:C-OP"), Some(&claim(&referenced)));
        assert!(outcome.applied_model.is_none());
        assert!(blocking(&outcome).contains(&"OP-COMBINATION-DELETE-REFERENCED".to_string()), "{:?}", blocking(&outcome));
    }
}

#[test]
fn every_basis_deletes_top_down_and_each_restore_is_exact() {
    let model = every_slot();
    // The unsigned chain C-SRSS -> C-ABS -> C-EXP, top down; each restore reproduces
    // the document before its delete.
    let without_srss = delete_and_restore(&model, "combination:C-SRSS");
    let without_abs = delete_and_restore(&without_srss, "combination:C-ABS");
    // C-EXP carries `subtrahend_support_state: minuend_final`, which no create
    // authors yet: it deletes, and its restore is refused by the same name.
    let index = index_of(&without_abs, "combination:C-EXP");
    let stored = record(&without_abs, "combination:C-EXP");
    let deleted = applied(&without_abs, &delete(&without_abs, "combination:C-EXP"));
    refused(&deleted, &restore("combination:C-EXP", index, &stored), "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED");

    // A declared O - S subtraction without the refused member, stored with no
    // `terms` member (a normalizing create would add `"terms": []`).
    let mut subtraction = without_abs.clone();
    subtraction["combinations"][index]["subtrahend_support_state"] = json!("own");
    assert!(record(&subtraction, "combination:C-EXP").get("terms").is_none());
    delete_and_restore(&subtraction, "combination:C-EXP");

    // A range envelope with an untrimmed label and notes, kept exactly.
    let mut envelope = model.clone();
    envelope["combinations"].as_array_mut().unwrap().insert(1, json!({"id": "combination:C-ENV", "label": " Envelope ", "basis": "range_envelope",
        "operand_ids": ["load:L-1", "load:L-2"], "mode": "max", "provenance": "invented_example", "notes": "kept"}));
    delete_and_restore(&envelope, "combination:C-ENV");

    // The flat mechanics record keeps its historical display. With no terms it
    // deletes, but no create authors a termless record, so its restore is refused
    // by create's own name (no admission is widened); with a term, its tags and
    // notes restore exactly at index 0.
    assert_eq!(flat_display(&record(&model, "combination:C-OP")).unwrap(), "combination:C-OP; Operating; basis=mechanics; terms=none");
    let stored = record(&model, "combination:C-OP");
    let deleted = applied(&model, &delete(&model, "combination:C-OP"));
    refused(&deleted, &restore("combination:C-OP", 0, &stored), "OP-CREATE-COMBINATION-PAYLOAD-INVALID");
    let mut flat = model.clone();
    flat["combinations"][0]["terms"] = json!([{"load_case": "load:L-1", "factor": 1.5}, {"load_case": "load:L-2", "factor": -2}]);
    assert_eq!(flat_display(&record(&flat, "combination:C-OP")).unwrap(), "combination:C-OP; Operating; basis=mechanics; terms=load:L-1 x 1.5; load:L-2 x -2");
    delete_and_restore(&flat, "combination:C-OP");
}

#[test]
fn restores_keep_number_forms_that_canonical_text_would_collapse() {
    let mut model = every_slot();
    let integer = json!(1_000_000_000_000_000_100_u64);
    let float = json!(1_000_000_000_000_000_128.0_f64);
    assert_eq!(canonical_json(&integer), canonical_json(&float), "the collision this guards");
    let numbers = json!({"id": "combination:C-NUM", "label": "Numbers", "basis": "mechanics", "provenance": "invented_example",
        "terms": [{"load_case": "load:L-1", "factor": 20}, {"load_case": "load:L-2", "factor": 20.0}, {"load_case": "load:L-3", "factor": integer}]});
    model["combinations"].as_array_mut().unwrap().insert(0, numbers);
    let deleted = delete_and_restore(&model, "combination:C-NUM");
    // The float form of the large factor is a different record, kept as given.
    let mut other = record(&model, "combination:C-NUM");
    other["terms"][2]["factor"] = float;
    let restored = applied(&deleted, &restore("combination:C-NUM", 0, &other));
    assert_eq!(record(&restored, "combination:C-NUM"), other);
    assert_ne!(restored, model);
    assert!(record(&restored, "combination:C-NUM")["terms"][0]["factor"].is_u64());
    assert!(record(&restored, "combination:C-NUM")["terms"][1]["factor"].is_f64());
}

#[test]
fn a_nested_term_deletes_and_restores_at_its_index_with_its_type() {
    let model = every_slot();
    // C-ABS: term 0 is a load case (historical display), term 1 names C-EXP
    // (canonical JSON of the stored term).
    let id = "combination:C-ABS";
    for index in [1, 0] {
        let term = record(&model, id)["terms"][index].clone();
        let deleted = applied(&model, &delete_term(&model, id, index));
        assert_eq!(record(&deleted, id)["terms"].as_array().unwrap().len(), 1);
        let back = applied(&deleted, &restore_term(id, index, &term));
        assert_eq!(back, model, "term {index}: the restore reproduces the document");
        assert_eq!(model_hash(&back), model_hash(&model));
    }
    // Deleting the nested term frees C-EXP from C-ABS (C-SRSS still names C-ABS).
    let freed = applied(&model, &delete_term(&model, id, 1));
    applied(&freed, &delete(&freed, "combination:C-EXP"));
}

#[test]
fn negative_controls() {
    let model = every_slot();
    // A wrong before: the flat display of a record that is not flat, or of a term naming a combination.
    let mut stale = delete(&model, "combination:C-SRSS");
    stale["change"]["before"] = json!("combination:C-SRSS; SRSS; basis=srss; terms=absent");
    refused(&model, &stale, "OP-STALE-BEFORE-VALUE");
    let mut stale_term = delete_term(&model, "combination:C-ABS", 1);
    stale_term["change"]["before"] = json!("combination:C-EXP x 1");
    refused(&model, &stale_term, "OP-STALE-BEFORE-VALUE");
    // Already deleted.
    let deleted = applied(&model, &delete(&model, "combination:C-SRSS"));
    refused(&deleted, &delete(&model, "combination:C-SRSS"), "OP-TARGET-NOT-FOUND");
    // A restore index past the end, a restore whose id differs, and a restore over an existing id.
    let stored = record(&model, "combination:C-SRSS");
    refused(&deleted, &restore("combination:C-SRSS", 9, &stored), "OP-CREATE-COMBINATION-INDEX-INVALID");
    let mut renamed = stored.clone();
    renamed["id"] = json!("combination:C-OTHER");
    refused(&deleted, &restore("combination:C-SRSS", 0, &renamed), "OP-CREATE-COMBINATION-PAYLOAD-INVALID");
    refused(&model, &restore("combination:C-SRSS", 0, &stored), "OP-TARGET-ALREADY-EXISTS");
    // A term restore index past the end.
    let term = record(&model, "combination:C-ABS")["terms"][1].clone();
    let without_term = applied(&model, &delete_term(&model, "combination:C-ABS", 1));
    refused(&without_term, &restore_term("combination:C-ABS", 5, &term), "OP-CREATE-COMBINATION-TERM-INDEX-INVALID");
    // A restored term that closes a cycle is the document validator's refusal.
    let cycle = json!({"combination": "combination:C-SRSS", "factor": 1.0});
    refused(&model, &restore_term("combination:C-ABS", 0, &cycle), "LOAD_COMBINATION_OPERAND_CYCLE");
    // The normalizing create keeps appending, and is refused on a malformed index path.
    let mut bad_path = restore("combination:C-SRSS", 0, &stored);
    bad_path["change"]["field_path"] = json!("combinations[x]");
    refused(&deleted, &bad_path, "OP-CREATE-COMBINATION-SHAPE-INVALID");
}

/// The existing cardinality rule, unchanged: the model format admits a combination
/// with no terms (the witness's `C-OP`), so a last term deletes as a load-case term
/// always has, and the creates that author records still require a term.
#[test]
fn the_last_term_follows_the_existing_cardinality_rule() {
    let model = every_slot();
    let term = record(&model, "combination:C-SRSS")["terms"][0].clone();
    let emptied = applied(&model, &delete_term(&model, "combination:C-SRSS", 0));
    assert_eq!(record(&emptied, "combination:C-SRSS")["terms"], json!([]));
    assert_eq!(applied(&emptied, &restore_term("combination:C-SRSS", 0, &term)), model);
}

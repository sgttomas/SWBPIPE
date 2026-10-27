//! Rule-by-rule tests of the model-document validator (W-13 §5.3, §5.5).
//! Invented data only. The documents carry the §3 slots whatever their
//! version string says: the validator is version-agnostic.

use super::*;
use serde_json::json;

/// An invented document carrying every §3 slot, well formed: the shared
/// fixture the native and webview open tests read too.
pub(crate) fn every_slot_document() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/model_document/invented_every_slot_model.json"
    ))
    .unwrap()
}

fn findings(document: &Value) -> Vec<(&'static str, &'static str, String)> {
    validate_model_document(document)
        .into_iter()
        .map(|item| (item.code, item.severity, item.path))
        .collect()
}

/// Exactly one blocking finding, with this code at this path.
#[track_caller]
fn blocks(document: &Value, code: &str, path: &str) {
    let found = findings(document);
    let blocking: Vec<_> = found.iter().filter(|item| item.1 == SEVERITY_BLOCKING).collect();
    assert!(
        blocking.iter().any(|item| item.0 == code && item.2 == path),
        "expected {code} at {path}; found {found:?}"
    );
}

#[track_caller]
fn clean(document: &Value) {
    let found = findings(document);
    assert!(found.is_empty(), "unexpected findings: {found:?}");
}

#[track_caller]
fn mutated(edit: impl FnOnce(&mut Value)) -> Value {
    let mut document = every_slot_document();
    edit(&mut document);
    document
}

#[test]
fn every_slot_document_is_clean_and_absence_means_none() {
    clean(&every_slot_document());
    // A document without any §3 slot has nothing to report.
    let mut bare = every_slot_document();
    let root = bare.as_object_mut().unwrap();
    for member in ["lines", "unresolved_assumptions", "constraints", "equipment_interfaces", "rule_pack_refs"] {
        root.remove(member);
    }
    clean(&bare);
    // Not an object: no slots, no findings (the callers name the shape).
    assert!(validate_model_document(&json!([1, 2])).is_empty());
    // The version string is not read.
    clean(&mutated(|d| d["schema_version"] = json!("0.8.0")));
}

#[test]
fn tags_are_unique_nonempty_trimmed_strings_of_at_most_64_chars() {
    blocks(&mutated(|d| d["project"]["tags"] = json!("tie-in")), TAGS_INVALID, "project.tags");
    blocks(&mutated(|d| d["nodes"][0]["tags"] = json!([3])), TAGS_INVALID, "nodes[0].tags[0]");
    blocks(&mutated(|d| d["nodes"][0]["tags"] = json!(["  "])), TAGS_INVALID, "nodes[0].tags[0]");
    blocks(&mutated(|d| d["pipe_segments"][0]["tags"] = json!([" lead"])), TAGS_INVALID, "pipe_segments[0].tags[0]");
    blocks(&mutated(|d| d["supports"][0]["tags"] = json!(["a", "a"])), TAGS_INVALID, "supports[0].tags[1]");
    blocks(&mutated(|d| d["lines"][0]["tags"] = json!(["x".repeat(65)])), TAGS_INVALID, "lines[0].tags[0]");
    // 64 characters (not bytes) is the limit; order is as authored.
    clean(&mutated(|d| d["load_cases"][0]["tags"] = json!(["é".repeat(64), "b", "a"])));
}

#[test]
fn notes_are_one_nonempty_string() {
    blocks(&mutated(|d| d["combinations"][0]["notes"] = json!("")), NOTES_INVALID, "combinations[0].notes");
    blocks(&mutated(|d| d["components"][0]["notes"] = json!(["a"])), NOTES_INVALID, "components[0].notes");
    blocks(&mutated(|d| d["constraints"][0]["notes"] = json!("   ")), NOTES_INVALID, "constraints[0].notes");
    clean(&mutated(|d| d["sections"][0]["notes"] = json!("Section note.")));
}

#[test]
fn annotations_are_refused_on_the_load_reference_group_and_pressure_contract() {
    blocks(
        &mutated(|d| d["reference_configurations"] = json!([{ "id": "reference:R-1", "tags": ["x"] }])),
        ANNOTATION_NOT_ALLOWED,
        "reference_configurations[0].tags",
    );
    blocks(
        &mutated(|d| d["pressure_contract"] = json!({ "version": "3.0.0", "notes": "x" })),
        ANNOTATION_NOT_ALLOWED,
        "pressure_contract.notes",
    );
    blocks(
        &mutated(|d| d["materials"][0]["expansion_laws"] = json!([{ "id": "law:1", "notes": "x" }])),
        ANNOTATION_NOT_ALLOWED,
        "materials[0].expansion_laws[0].notes",
    );
    blocks(
        &mutated(|d| d["load_cases"][0]["analysis_state"] = json!({ "kind": "x", "provenance_record": { "source_name": "x" } })),
        ANNOTATION_NOT_ALLOWED,
        "load_cases[0].analysis_state.provenance_record",
    );
    // The same groups without annotations are not this validator's business.
    clean(&mutated(|d| d["reference_configurations"] = json!([{ "id": "reference:R-1" }])));
}

#[test]
fn provenance_record_only_on_materials_sections_components_supports() {
    for (collection, path) in [
        ("nodes", "nodes[0].provenance_record"),
        ("pipe_segments", "pipe_segments[0].provenance_record"),
        ("load_cases", "load_cases[0].provenance_record"),
        ("combinations", "combinations[0].provenance_record"),
    ] {
        blocks(
            &mutated(|d| d[collection][0]["provenance_record"] = json!({ "source_name": "Invented" })),
            ANNOTATION_NOT_ALLOWED,
            path,
        );
    }
    blocks(&mutated(|d| d["project"]["provenance_record"] = json!({ "source_name": "x" })), ANNOTATION_NOT_ALLOWED, "project.provenance_record");
    // On a new record it is an unknown member.
    blocks(&mutated(|d| d["lines"][0]["provenance_record"] = json!({ "source_name": "x" })), SLOT_SHAPE, "lines[0].provenance_record");
}

#[test]
fn provenance_record_requires_source_name_and_uses_the_library_import_vocabulary() {
    blocks(&mutated(|d| d["sections"][0]["provenance_record"] = json!({})), PROVENANCE_RECORD_INVALID, "sections[0].provenance_record.source_name");
    blocks(&mutated(|d| d["sections"][0]["provenance_record"] = json!("Invented")), PROVENANCE_RECORD_INVALID, "sections[0].provenance_record");
    blocks(&mutated(|d| d["sections"][0]["provenance_record"]["source_name"] = json!(" ")), PROVENANCE_RECORD_INVALID, "sections[0].provenance_record.source_name");
    blocks(&mutated(|d| d["materials"][0]["provenance_record"]["redistribution_status"] = json!("TBD")), PROVENANCE_RECORD_INVALID, "materials[0].provenance_record.redistribution_status");
    blocks(&mutated(|d| d["materials"][0]["provenance_record"]["review_status"] = json!("approved")), PROVENANCE_RECORD_INVALID, "materials[0].provenance_record.review_status");
    blocks(&mutated(|d| d["materials"][0]["provenance_record"]["reviewed_on"] = json!("2026-02-30")), PROVENANCE_RECORD_INVALID, "materials[0].provenance_record.reviewed_on");
    blocks(&mutated(|d| d["materials"][0]["provenance_record"]["license"] = json!("x")), PROVENANCE_RECORD_INVALID, "materials[0].provenance_record.license");
    blocks(&mutated(|d| d["materials"][0]["provenance_record"]["library_ref"]["library_kind"] = json!("support")), PROVENANCE_RECORD_INVALID, "materials[0].provenance_record.library_ref.library_kind");
    blocks(&mutated(|d| { d["materials"][0]["provenance_record"]["library_ref"].as_object_mut().unwrap().remove("library_id"); }), PROVENANCE_RECORD_INVALID, "materials[0].provenance_record.library_ref.library_id");
    blocks(&mutated(|d| d["materials"][0]["provenance_record"]["library_ref"]["payload"] = json!({})), PROVENANCE_RECORD_INVALID, "materials[0].provenance_record.library_ref.payload");
    for kind in LIBRARY_KINDS {
        clean(&mutated(|d| d["materials"][0]["provenance_record"]["library_ref"]["library_kind"] = json!(kind)));
    }
    clean(&mutated(|d| d["materials"][0]["provenance_record"]["reviewed_on"] = json!("2028-02-29")));
}

#[test]
fn quarantine_dispositions_warn_without_blocking() {
    for (member, token) in [("redistribution_status", "protected_suspected"), ("review_status", "quarantined")] {
        let document = mutated(|d| d["supports"][0]["provenance_record"][member] = json!(token));
        let found = validate_model_document(&document);
        assert!(!has_blocking(&found), "{found:?}");
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].code, PROVENANCE_QUARANTINE);
        assert_eq!(found[0].severity, SEVERITY_WARNING);
        assert_eq!(found[0].path, format!("supports[0].provenance_record.{member}"));
        assert_eq!(found[0].affected_refs, vec!["support:S-1".to_string()]);
    }
    // `rejected` is blocked only for export (W-6), not in the model.
    clean(&mutated(|d| d["supports"][0]["provenance_record"]["review_status"] = json!("rejected")));
}

#[test]
fn lines_group_existing_pipes_each_in_at_most_one_line() {
    blocks(&mutated(|d| d["lines"] = json!({})), SLOT_SHAPE, "lines");
    blocks(&mutated(|d| d["lines"][1] = json!("line:L-200")), SLOT_SHAPE, "lines[1]");
    blocks(&mutated(|d| d["lines"][1]["id"] = json!("line:L-100")), SLOT_ID_DUPLICATE, "lines[1].id");
    blocks(&mutated(|d| { d["lines"][1].as_object_mut().unwrap().remove("pipe_refs"); }), SLOT_SHAPE, "lines[1].pipe_refs");
    blocks(&mutated(|d| { d["lines"][1].as_object_mut().unwrap().remove("id"); }), SLOT_SHAPE, "lines[1].id");
    blocks(&mutated(|d| d["lines"][0]["id"] = json!("")), SLOT_SHAPE, "lines[0].id");
    blocks(&mutated(|d| d["lines"][0]["pipe_refs"] = json!(["pipe:P-1", "pipe:P-1"])), LINE_PIPE_DUPLICATE, "lines[0].pipe_refs[1]");
    blocks(&mutated(|d| d["lines"][0]["pipe_refs"] = json!(["pipe:P-9"])), LINE_PIPE_MISSING, "lines[0].pipe_refs[0]");
    blocks(&mutated(|d| d["lines"][0]["pipe_refs"] = json!([7])), SLOT_SHAPE, "lines[0].pipe_refs[0]");
    blocks(&mutated(|d| d["lines"][1]["pipe_refs"] = json!(["pipe:P-1"])), LINE_PIPE_SHARED, "lines[1].pipe_refs[0]");
    blocks(&mutated(|d| d["lines"][0]["colour"] = json!("red")), SLOT_SHAPE, "lines[0].colour");
    // A node id is not a pipe.
    blocks(&mutated(|d| d["lines"][0]["pipe_refs"] = json!(["node:N-1"])), LINE_PIPE_MISSING, "lines[0].pipe_refs[0]");
    clean(&mutated(|d| d["lines"][1]["pipe_refs"] = json!(["pipe:P-2"])));
    // An empty collection is allowed; the assumption naming the removed line
    // now warns, without blocking.
    let emptied = validate_model_document(&mutated(|d| d["lines"] = json!([])));
    assert_eq!(emptied.iter().map(|item| (item.code, item.severity)).collect::<Vec<_>>(),
        vec![(ASSUMPTION_SUBJECT_MISSING, SEVERITY_WARNING)]);
}

#[test]
fn assumptions_use_the_assumption_record_vocabulary() {
    blocks(&mutated(|d| d["unresolved_assumptions"][1]["assumption_id"] = json!("assumption:A-001")), SLOT_ID_DUPLICATE, "unresolved_assumptions[1].assumption_id");
    blocks(&mutated(|d| d["unresolved_assumptions"][0]["kind"] = json!("risk")), ASSUMPTION_KIND_INVALID, "unresolved_assumptions[0].kind");
    blocks(&mutated(|d| d["unresolved_assumptions"][0]["status"] = json!("open")), ASSUMPTION_STATUS_INVALID, "unresolved_assumptions[0].status");
    blocks(&mutated(|d| { d["unresolved_assumptions"][0].as_object_mut().unwrap().remove("statement"); }), SLOT_SHAPE, "unresolved_assumptions[0].statement");
    blocks(&mutated(|d| d["unresolved_assumptions"][0]["affected_refs"] = json!("support:S-1")), SLOT_SHAPE, "unresolved_assumptions[0].affected_refs");
    blocks(&mutated(|d| d["unresolved_assumptions"][0]["affected_refs"] = json!(["support:S-1", "support:S-1"])), SLOT_REF_DUPLICATE, "unresolved_assumptions[0].affected_refs[1]");
    blocks(&mutated(|d| d["unresolved_assumptions"][0]["state"] = json!("x")), SLOT_SHAPE, "unresolved_assumptions[0].state");
    for status in ["resolved", "rejected"] {
        blocks(
            &mutated(|d| { d["unresolved_assumptions"][1]["status"] = json!(status); d["unresolved_assumptions"][1].as_object_mut().unwrap().remove("resolution_note"); }),
            ASSUMPTION_RESOLUTION_NOTE_REQUIRED,
            "unresolved_assumptions[1].resolution_note",
        );
        clean(&mutated(|d| d["unresolved_assumptions"][1]["status"] = json!(status)));
    }
    blocks(&mutated(|d| d["unresolved_assumptions"][1]["resolution_note"] = json!("")), SLOT_SHAPE, "unresolved_assumptions[1].resolution_note");
    // A note on an unresolved record is allowed; project level is empty refs.
    clean(&mutated(|d| { d["unresolved_assumptions"][0]["resolution_note"] = json!("Pending."); d["unresolved_assumptions"][0]["affected_refs"] = json!([]); }));
}

#[test]
fn constraints_use_the_constraint_record_vocabulary_without_evaluation() {
    blocks(&mutated(|d| d["constraints"][0]["constraint_kind"] = json!("TBD")), CONSTRAINT_KIND_INVALID, "constraints[0].constraint_kind");
    blocks(&mutated(|d| d["constraints"][0]["constraint_kind"] = json!("equipment_interface")), CONSTRAINT_KIND_INVALID, "constraints[0].constraint_kind");
    blocks(&mutated(|d| d["constraints"][0]["constraint_kind"] = json!("missing_required_data")), CONSTRAINT_KIND_INVALID, "constraints[0].constraint_kind");
    blocks(&mutated(|d| d["constraints"][0]["state"] = json!("satisfied")), CONSTRAINT_STATE_INVALID, "constraints[0].state");
    // No TBD sentinel in these slots (design §6).
    blocks(&mutated(|d| d["constraints"][0]["state"] = json!("TBD")), CONSTRAINT_STATE_INVALID, "constraints[0].state");
    blocks(&mutated(|d| { let k = d["constraints"][0].clone(); d["constraints"].as_array_mut().unwrap().push(k); }), SLOT_ID_DUPLICATE, "constraints[1].constraint_id");
    blocks(&mutated(|d| { d["constraints"][0].as_object_mut().unwrap().remove("name"); }), SLOT_SHAPE, "constraints[0].name");
    blocks(&mutated(|d| d["constraints"][0]["parameters"] = json!([])), SLOT_SHAPE, "constraints[0].parameters");
    for kind in CONSTRAINT_KINDS {
        clean(&mutated(|d| d["constraints"][0]["constraint_kind"] = json!(kind)));
    }
    for state in CONSTRAINT_STATES {
        clean(&mutated(|d| d["constraints"][0]["state"] = json!(state)));
    }
}

#[test]
fn equipment_interfaces_have_unique_ids_and_a_node_ref() {
    blocks(&mutated(|d| { let e = d["equipment_interfaces"][0].clone(); d["equipment_interfaces"].as_array_mut().unwrap().push(e); }), SLOT_ID_DUPLICATE, "equipment_interfaces[1].id");
    blocks(&mutated(|d| { d["equipment_interfaces"][0].as_object_mut().unwrap().remove("node_ref"); }), SLOT_SHAPE, "equipment_interfaces[0].node_ref");
    blocks(&mutated(|d| d["equipment_interfaces"][0]["node_ref"] = json!(["node:N-1"])), SLOT_SHAPE, "equipment_interfaces[0].node_ref");
}

#[test]
fn missing_subjects_warn_and_never_block() {
    let document = mutated(|d| {
        d["unresolved_assumptions"][0]["affected_refs"] = json!(["support:S-gone", "load:L-1-Z"]);
        d["constraints"][0]["target_refs"] = json!(["pipe:P-gone"]);
        d["equipment_interfaces"][0]["node_ref"] = json!("pipe:P-1");
    });
    let found = validate_model_document(&document);
    assert!(!has_blocking(&found), "{found:?}");
    let missing: Vec<_> = found.iter().map(|item| (item.code, item.severity, item.path.as_str())).collect();
    assert_eq!(missing, vec![
        (ASSUMPTION_SUBJECT_MISSING, SEVERITY_WARNING, "unresolved_assumptions[0].affected_refs[0]"),
        (ASSUMPTION_SUBJECT_MISSING, SEVERITY_WARNING, "constraints[0].target_refs[0]"),
        // node_ref names a node; a pipe id is not one.
        (ASSUMPTION_SUBJECT_MISSING, SEVERITY_WARNING, "equipment_interfaces[0].node_ref"),
    ]);
    assert_eq!(found[0].affected_refs, vec!["assumption:A-001".to_string(), "support:S-gone".to_string()]);
}

#[test]
fn rule_pack_refs_carry_the_stamped_checksum_record() {
    let at = "rule_pack_refs[0].checksum";
    blocks(&mutated(|d| d["rule_pack_refs"][0]["checksum"] = json!("c2b4")), RULE_PACK_REF_INVALID, at);
    for (member, value) in [
        ("algorithm", json!("sha512")),
        ("canonicalization", json!("rfc8785_jcs")),
        ("payload_scope", json!("model_payload")),
        ("payload_excludes", json!([])),
        ("value", json!("C2B4DDCE48CD07BED4BB2FF0BE3A81D3755365E255896A0BBF583725A3FA47BD")),
        // Closed: members outside the common subset are refused.
        ("payload_ref", json!("invented_demo_rule_pack")),
        ("basis", json!("rfc8785_jcs_sha256_excluding_checksums")),
        ("grammar_version", json!("1.0.0")),
        ("verification_status", json!("verified")),
    ] {
        blocks(
            &mutated(|d| d["rule_pack_refs"][0]["checksum"][member] = value.clone()),
            RULE_PACK_REF_INVALID,
            &format!("{at}.{member}"),
        );
    }
    blocks(&mutated(|d| { d["rule_pack_refs"][0]["checksum"].as_object_mut().unwrap().remove("value"); }), RULE_PACK_REF_INVALID, &format!("{at}.value"));
    blocks(&mutated(|d| d["rule_pack_refs"][0]["redistribution_status"] = json!("TBD")), RULE_PACK_REF_INVALID, "rule_pack_refs[0].redistribution_status");
    blocks(&mutated(|d| d["rule_pack_refs"][0]["name"] = json!("")), RULE_PACK_REF_INVALID, "rule_pack_refs[0].name");
    blocks(&mutated(|d| d["rule_pack_refs"][0]["rules"] = json!([])), SLOT_SHAPE, "rule_pack_refs[0].rules");
    blocks(&mutated(|d| { let r = d["rule_pack_refs"][0].clone(); d["rule_pack_refs"].as_array_mut().unwrap().push(r); }), SLOT_ID_DUPLICATE, "rule_pack_refs[1].rule_pack_id");
    for status in RULE_PACK_REDISTRIBUTION_STATUSES.iter().filter(|status| **status != "protected_suspected") {
        clean(&mutated(|d| d["rule_pack_refs"][0]["redistribution_status"] = json!(status)));
    }
    let suspected = validate_model_document(&mutated(|d| d["rule_pack_refs"][0]["redistribution_status"] = json!("protected_suspected")));
    assert_eq!(suspected.iter().map(|item| (item.code, item.severity)).collect::<Vec<_>>(), vec![(PROVENANCE_QUARANTINE, SEVERITY_WARNING)]);
}

/// Parity with the owners of the vocabularies: a reference's checksum is the
/// subset common to the pack's stored stamped record and its computed
/// record, and both projections validate; the library import's
/// required-provenance fields and kinds are this validator's.
#[test]
fn vocabularies_match_their_owning_crates() {
    use open_pipe_stress_library_import_document::{LibraryKind, REQUIRED_PROVENANCE_FIELDS};
    use open_pipe_stress_rule_pack_document::{compute_rule_pack_checksum, CHECKSUM_PAYLOAD_EXCLUDES};

    assert_eq!(PROVENANCE_RECORD_FIELDS, REQUIRED_PROVENANCE_FIELDS);
    for kind in LIBRARY_KINDS {
        assert_eq!(LibraryKind::from_token(kind).map(LibraryKind::token), Some(kind));
    }
    assert_eq!(RULE_PACK_CHECKSUM_PAYLOAD_EXCLUDES.as_slice(), CHECKSUM_PAYLOAD_EXCLUDES);

    let pack: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/invented_demo_rule_pack.json"
    ))
    .unwrap();
    let project = |record: &Value| -> Value {
        Value::Object(
            RULE_PACK_CHECKSUM_MEMBERS
                .iter()
                .map(|member| (member.to_string(), record.get(*member).cloned().unwrap_or(Value::Null)))
                .collect(),
        )
    };
    let stored = project(&pack["checksums"]["rule_pack_checksum"]);
    let computed = project(&serde_json::to_value(compute_rule_pack_checksum(&pack).unwrap()).unwrap());
    // Every common member is present in both records.
    for record in [&stored, &computed] {
        assert!(record.as_object().unwrap().values().all(|value| !value.is_null()), "{record}");
    }
    for checksum in [stored, computed] {
        clean(&mutated(|d| {
            d["rule_pack_refs"][0]["rule_pack_id"] = pack["metadata"]["rule_pack_id"].clone();
            d["rule_pack_refs"][0]["rule_pack_version"] = pack["metadata"]["rule_pack_version"].clone();
            d["rule_pack_refs"][0]["checksum"] = checksum.clone();
            d["rule_pack_refs"][0]["redistribution_status"] = pack["classification"]["redistribution_status"].clone();
        }));
    }

    // Constraint kinds and states are constraint.schema.json's, less the
    // excluded kinds and the TBD sentinel (design §3.6, §6).
    let schema: Value = serde_json::from_str(include_str!("../../../../schemas/constraint.schema.json")).unwrap();
    let tokens = |pointer: &str| -> Vec<String> {
        schema.pointer(pointer).unwrap().as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect()
    };
    let kinds: Vec<String> = tokens("/$defs/ConstraintKind/enum").into_iter()
        .filter(|kind| !matches!(kind.as_str(), "TBD" | "equipment_interface" | "missing_required_data")).collect();
    assert_eq!(CONSTRAINT_KINDS.as_slice(), kinds.as_slice());
    let states: Vec<String> = tokens("/$defs/ConstraintRecord/properties/state/enum").into_iter()
        .filter(|state| state != "TBD").collect();
    assert_eq!(CONSTRAINT_STATES.as_slice(), states.as_slice());
}

#[test]
fn report_envelope_names_blocking() {
    let report = validation_report(&every_slot_document());
    assert_eq!(report["document_kind"], json!(VALIDATION_DOCUMENT_KIND));
    assert_eq!(report["blocking"], json!(false));
    assert_eq!(report["diagnostics"], json!([]));
    let report = validation_report(&mutated(|d| d["lines"][0]["pipe_refs"] = json!(["pipe:P-9"])));
    assert_eq!(report["blocking"], json!(true));
    assert_eq!(report["diagnostics"][0]["code"], json!(LINE_PIPE_MISSING));
    assert_eq!(report["diagnostics"][0]["path"], json!("lines[0].pipe_refs[0]"));
    assert_eq!(report["diagnostics"][0]["severity"], json!("blocking"));
}

/// The bundled demo and the committed product documents carry no §3 slot and
/// stay clean.
#[test]
fn bundled_demo_is_clean() {
    let demo: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/invented_demo_model.json"
    ))
    .unwrap();
    clean(&demo);
}

/// §3.8: `model_settings.node_proximity_tolerance` is a closed length quantity, finite, greater
/// than zero, in a catalog length unit. Each refusal is pinned with its code, path and message
/// (the same messages product_physics' accessor gives); absence and a valid tolerance are clean.
#[test]
fn model_settings_tolerance_refusals_are_named() {
    #[track_caller]
    fn refused(document: &Value, code: &str, path: &str, message: &str) {
        let found = validate_model_document(document);
        let blocking: Vec<_> = found.iter().filter(|item| item.is_blocking()).collect();
        assert_eq!(blocking.len(), 1, "{found:?}");
        assert_eq!((blocking[0].code, blocking[0].path.as_str(), blocking[0].message.as_str()), (code, path, message));
    }
    let tolerance = |value: Value| mutated(|d| d["model_settings"] = json!({ "node_proximity_tolerance": value }));
    clean(&mutated(|d| { d.as_object_mut().unwrap().remove("model_settings"); }));
    clean(&mutated(|d| d["model_settings"] = json!({})));
    clean(&tolerance(json!({"value": 5, "unit": "mm"})));
    clean(&tolerance(json!({"value": 0.25, "unit": "in"})));
    refused(&tolerance(json!({"value": 0, "unit": "mm"})), "MODEL-SETTING-TOLERANCE-NOT-POSITIVE",
        "model_settings.node_proximity_tolerance.value",
        "model_settings.node_proximity_tolerance.value is 0; the tolerance must be greater than zero.");
    refused(&tolerance(json!({"value": -2.5, "unit": "mm"})), "MODEL-SETTING-TOLERANCE-NOT-POSITIVE",
        "model_settings.node_proximity_tolerance.value",
        "model_settings.node_proximity_tolerance.value is -2.5; the tolerance must be greater than zero.");
    refused(&tolerance(json!({"value": 5, "unit": "N"})), "MODEL-SETTING-TOLERANCE-UNIT-NOT-LENGTH",
        "model_settings.node_proximity_tolerance.unit",
        "model_settings.node_proximity_tolerance.unit `N` is not a length unit of the units catalog.");
    refused(&tolerance(json!({"value": 5, "unit": "mmm"})), "MODEL-SETTING-TOLERANCE-UNIT-NOT-LENGTH",
        "model_settings.node_proximity_tolerance.unit",
        "model_settings.node_proximity_tolerance.unit `mmm` is not a length unit of the units catalog.");
    // JSON carries no non-finite number; the finite check is product_physics' accessor's, pinned there.
    refused(&tolerance(json!({"value": "5", "unit": "mm"})), "MODEL-SLOT-SHAPE",
        "model_settings.node_proximity_tolerance.value",
        "model_settings.node_proximity_tolerance.value is string; number required.");
    refused(&tolerance(json!({"value": 5, "unit": 3})), "MODEL-SLOT-SHAPE",
        "model_settings.node_proximity_tolerance.unit",
        "model_settings.node_proximity_tolerance.unit is number; string required.");
    refused(&tolerance(json!({"value": 5})), "MODEL-SLOT-SHAPE",
        "model_settings.node_proximity_tolerance.unit",
        "model_settings.node_proximity_tolerance.unit is missing (required).");
    refused(&tolerance(json!({"value": 5, "unit": "mm", "basis": "x"})), "MODEL-SLOT-SHAPE",
        "model_settings.node_proximity_tolerance.basis",
        "model_settings.node_proximity_tolerance.basis is not a member of this record.");
    refused(&tolerance(json!(5)), "MODEL-SLOT-SHAPE", "model_settings.node_proximity_tolerance",
        "model_settings.node_proximity_tolerance is number; a length quantity {value, unit} is required.");
    // An authored null is not absence (product_physics refuses it at the wire too).
    refused(&tolerance(Value::Null), "MODEL-SLOT-SHAPE", "model_settings.node_proximity_tolerance",
        "model_settings.node_proximity_tolerance is null; a length quantity {value, unit} is required.");
    // A positive value that is zero in metres is refused with the accessor's code and message.
    refused(&tolerance(json!({"value": 5e-324, "unit": "mm"})), "MODEL-SETTING-TOLERANCE-NOT-POSITIVE",
        "model_settings.node_proximity_tolerance",
        "model_settings.node_proximity_tolerance is 5e-324 mm, which is zero in metres; the tolerance must be greater than zero.");
    refused(&mutated(|d| d["model_settings"] = json!({"proximity": 1})), "MODEL-SLOT-SHAPE",
        "model_settings.proximity", "model_settings.proximity is not a member of this record.");
    refused(&mutated(|d| d["model_settings"] = json!([])), "MODEL-SLOT-SHAPE", "model_settings",
        "model_settings is array; object required.");
}

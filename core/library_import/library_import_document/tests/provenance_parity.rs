//! Cross-language parity against the DEL-03-07 Python provenance contract.
//!
//! These cases mirror `tests/test_library_import_provenance.py` one-for-one,
//! over the *same* invented fixtures, so the runtime Rust port and the authored
//! Python contract agree on outcome and finding codes for every shared input.
//! If the Python contract changes, these fixtures change too, and both
//! implementations must move together — this test is the guard against silent
//! divergence.

use std::fs;
use std::path::PathBuf;

use open_pipe_stress_library_import_document::{
    validate_library_import, IntendedVisibility, LibraryKind, DIAGNOSTIC_SOURCE,
};
use serde_json::{json, Value};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn load_fixture(relative: &str) -> Value {
    let path = repo_root().join(relative);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must be readable: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("fixture {} must be JSON: {err}", path.display()))
}

fn material_fixture() -> Value {
    load_fixture("fixtures/material/invented_material_library_valid.json")
}

/// Mirror of `accepted_public_component_payload()` in the Python test: the
/// shared component fixture with public-permissive reviewed dispositions and a
/// single fully-provenanced public record.
fn accepted_public_component_payload() -> Value {
    let mut payload = load_fixture("fixtures/component/invented_section_component_library_valid.json");

    payload["component_library"]["library_scope"] = json!("public_permissive_reviewed");
    payload["component_library"]["privacy_class"] = json!("public_permissive_reviewed");
    payload["component_library"]["review_status"] = json!("accepted");
    payload["component_library"]["provenance"]["source_license"] = json!("public test license");
    payload["component_library"]["provenance"]["redistribution_status"] = json!("public_permissive");
    payload["component_library"]["provenance"]["review_status"] = json!("accepted");

    let reviewed_provenance = json!({
        "source_name": "Invented permissive test source",
        "source_location": "tests/test_library_import_provenance.py",
        "source_license": "public test license",
        "contributor": "OpenPipeStress",
        "contributor_certification": "invented non-engineering value",
        "redistribution_status": "public_permissive",
        "review_status": "accepted",
    });

    payload["component_records"] = json!([{
        "component_id": "comp.public.example",
        "name": "Public Reviewed Example",
        "component_type": "other",
        "privacy_class": "public_permissive_reviewed",
        "redistribution_status": "public_permissive",
        "fields": [{
            "field_id": "comp.public.example.weight",
            "field_kind": "weight",
            "value_status": "public_permissive_reviewed",
            "public_repository_value_policy": "public_permissive_reviewed_only",
            "required_for": "mechanics_solve",
            "value": {
                "magnitude": 1.0,
                "unit": "N",
                "dimension": "force",
                "value_status": "public_permissive_reviewed",
                "provenance": reviewed_provenance.clone(),
            },
            "provenance": reviewed_provenance.clone(),
            "review_status": "accepted",
        }],
        "completeness": [],
        "provenance": reviewed_provenance,
        "review_status": "accepted",
    }]);

    payload
}

fn codes(result: &open_pipe_stress_library_import_document::ImportValidationResult) -> Vec<String> {
    result.findings.iter().map(|f| f.code.clone()).collect()
}

#[test]
fn public_component_import_requires_accepted_public_provenance() {
    let result = validate_library_import(
        &accepted_public_component_payload(),
        LibraryKind::Component,
        IntendedVisibility::Public,
    );
    assert!(result.accepted);
    assert_eq!(result.outcome, "ACCEPTED_PUBLIC");
    assert!(
        result.findings.is_empty(),
        "expected no findings, got {:?}",
        codes(&result)
    );
}

#[test]
fn public_material_import_with_tbd_rights_is_rejected_for_review() {
    let result = validate_library_import(
        &material_fixture(),
        LibraryKind::Material,
        IntendedVisibility::Public,
    );
    assert!(!result.accepted);
    assert_eq!(result.outcome, "REJECTED");
    let found = codes(&result);
    assert!(found.contains(&"IMPORT_REDIS_RIGHTS_MISSING".to_string()));
    assert!(found.contains(&"IMPORT_REVIEW_REQUIRED".to_string()));
}

#[test]
fn private_material_import_can_remain_local_with_tbd_rights() {
    let result = validate_library_import(
        &material_fixture(),
        LibraryKind::Material,
        IntendedVisibility::Private,
    );
    assert!(result.accepted);
    assert_eq!(result.outcome, "PRIVATE_LOCAL_ONLY");
    assert!(!codes(&result).contains(&"IMPORT_REDIS_RIGHTS_MISSING".to_string()));
}

#[test]
fn missing_provenance_blocks_import_without_defaults() {
    let mut payload = material_fixture();
    payload["material_records"][0]
        .as_object_mut()
        .expect("record is an object")
        .remove("provenance");
    let result =
        validate_library_import(&payload, LibraryKind::Material, IntendedVisibility::Public);
    assert!(!result.accepted);
    assert_eq!(result.outcome, "REJECTED");
    assert!(codes(&result).contains(&"IMPORT_PROVENANCE_MISSING".to_string()));
}

#[test]
fn protected_suspected_metadata_quarantines_import() {
    let mut payload = material_fixture();
    payload["material_library"]["provenance"]["redistribution_status"] = json!("protected_suspected");
    let result =
        validate_library_import(&payload, LibraryKind::Material, IntendedVisibility::Private);
    assert!(!result.accepted);
    assert_eq!(result.outcome, "QUARANTINE");
    assert!(codes(&result).contains(&"IMPORT_PROTECTED_CONTENT_SUSPECTED".to_string()));
}

#[test]
fn unit_metadata_is_preserved_for_imported_values() {
    let mut payload = accepted_public_component_payload();
    payload["component_records"][0]["fields"][0]["value"]
        .as_object_mut()
        .expect("value is an object")
        .remove("unit");
    let result =
        validate_library_import(&payload, LibraryKind::Component, IntendedVisibility::Public);
    assert!(!result.accepted);
    assert_eq!(result.outcome, "REJECTED");
    assert!(codes(&result).contains(&"IMPORT_UNIT_METADATA_MISSING".to_string()));
}

#[test]
fn import_findings_map_to_pkg02_diagnostic_envelope() {
    let mut payload = accepted_public_component_payload();
    payload["component_records"][0]["fields"][0]["value"]
        .as_object_mut()
        .expect("value is an object")
        .remove("unit");
    let result =
        validate_library_import(&payload, LibraryKind::Component, IntendedVisibility::Public);

    let diagnostic = result
        .diagnostics()
        .into_iter()
        .find(|d| d["code"] == "IMPORT_UNIT_METADATA_MISSING")
        .expect("unit metadata diagnostic present");
    assert_eq!(diagnostic["class"], "import_boundary");
    assert_eq!(diagnostic["source"], DIAGNOSTIC_SOURCE);
    assert!(diagnostic["affected_object"]
        .as_str()
        .expect("affected_object is a string")
        .ends_with(".fields[0].value"));
    assert!(!diagnostic["remediation"]
        .as_str()
        .expect("remediation is a string")
        .is_empty());
    assert_eq!(
        diagnostic["provenance"]["source_name"],
        "library_import_payload"
    );
}

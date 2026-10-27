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

#[test]
fn hanger_shared_cases_match_python_codes_paths_severities_and_outcomes() {
    let cases = load_fixture("fixtures/hanger/invented_hanger_cases.json");
    // Run the authored Python boundary on precisely the same JSON cases. Full
    // finding tuples are compared, rather than just independently accepting
    // fixture expectations generated by one implementation.
    let python = std::process::Command::new("python3").current_dir(repo_root())
        .args(["-c", r#"
import json
from core.library_import.provenance_checker import validate_library_import
cases=json.load(open('fixtures/hanger/invented_hanger_cases.json'))
out=[]
for c in cases:
 r=validate_library_import(c['payload'],library_kind='hanger',intended_visibility=c['visibility'])
 out.append({'outcome':r.outcome,'findings':[[f.code,f.path,f.severity] for f in r.findings]})
print(json.dumps(out))
"#]).output().expect("Python parity runner");
    assert!(python.status.success(), "{}", String::from_utf8_lossy(&python.stderr));
    let expected: Value = serde_json::from_slice(&python.stdout).unwrap();
    for (i, case) in cases.as_array().unwrap().iter().enumerate() {
        let visibility = IntendedVisibility::from_token(case["visibility"].as_str().unwrap()).unwrap();
        let before = case["payload"].clone();
        let result = validate_library_import(&case["payload"], LibraryKind::Hanger, visibility);
        assert_eq!(result.outcome, case["outcome"], "{}", case["name"]);
        if let Some(code) = case["required_code"].as_str() {
            assert!(result.findings.iter().any(|f| f.code == code), "{}", case["name"]);
        }
        let tuples: Vec<Value> = result.findings.iter().map(|f| json!([f.code, f.path, f.severity])).collect();
        if case.get("required_path").is_some() {
            assert!(tuples.contains(&json!([case["required_code"], case["required_path"], case["required_severity"]])), "{}", case["name"]);
        }
        assert_eq!(json!({"outcome":result.outcome,"findings":tuples}), expected[i], "{}", case["name"]);
        assert_eq!(case["payload"], before);
    }
    assert_eq!(LibraryKind::from_token("hanger"), Some(LibraryKind::Hanger));
    assert_eq!(LibraryKind::Hanger.token(), "hanger");
}

/// W-17 object form: the cases of `tests/test_library_import_provenance.py`'s
/// unreadable-disposition tests, over every library kind, run through both
/// implementations; full ordered `(code, path, severity, message)` tuples and
/// outcomes must agree. Every unreadable case is refused by name
/// (`MODEL-DISPOSITION-RECORD-MALFORMED`) and every control is not.
#[test]
fn unreadable_disposition_cases_match_python() {
    use open_pipe_stress_library_import_document::DISPOSITION_RECORD_MALFORMED;
    let bases = [
        ("material", "fixtures/material/invented_material_library_valid.json", "material_library", "material_records"),
        ("section", "fixtures/component/invented_section_library_valid.json", "section_library", "section_records"),
        ("component", "fixtures/component/invented_component_library_valid.json", "component_library", "component_records"),
        ("hanger", "fixtures/hanger/invented_hanger_library_valid.json", "hanger_library", "hanger_records"),
    ];
    let unreadable: [(&str, Value); 4] = [
        ("review_status", json!(["quarantined"])),
        ("review_status", json!({ "status": "rejected" })),
        ("redistribution_status", json!(["protected_suspected"])),
        ("redistribution_status", json!({ "status": "rejected" })),
    ];
    let mut cases = Vec::new();
    for (kind, fixture, library_key, records_key) in bases {
        let base = load_fixture(fixture);
        cases.push((kind, "control", false, base.clone()));
        for (member, value) in &unreadable {
            for site in ["library", "record", "record-provenance", "nested"] {
                let mut payload = base.clone();
                let target = match site {
                    "library" => &mut payload[library_key],
                    "record" => &mut payload[records_key][0],
                    "record-provenance" => &mut payload[records_key][0]["provenance"],
                    _ => &mut payload[records_key][0],
                };
                if site == "nested" {
                    let mut leaf = serde_json::Map::new();
                    leaf.insert(member.to_string(), value.clone());
                    target["invented_nested"] = json!({ "deeper": [Value::Object(leaf)] });
                } else {
                    target[*member] = value.clone();
                }
                cases.push((kind, "unreadable", true, payload));
            }
        }
        for site in ["record", "nested"] {
            let mut payload = base.clone();
            if site == "record" {
                payload[records_key][0]["provenance"] = json!(["Invented", "private_only", "pending"]);
            } else {
                payload[records_key][0]["invented_nested"] = json!({ "provenance": ["Invented", "quarantined"] });
            }
            cases.push((kind, "positional", true, payload));
        }
        // Controls: an object-valued classification at every level and a
        // non-text scalar status state no disposition the verdict reads (the
        // hanger schema admits neither member, so hangers have no such control).
        if kind == "hanger" {
            continue;
        }
        let mut payload = base.clone();
        payload[library_key]["classification"] = json!({ "redistribution_status": "private_only" });
        payload[records_key][0]["classification"] = json!({ "value": "protected_suspected" });
        payload[records_key][0]["invented_nested"] = json!({ "classification": { "review_status": "accepted" }, "review_status": null });
        cases.push((kind, "classification", false, payload));
    }
    let input = json!(cases.iter().map(|(kind, _, _, payload)| json!({ "kind": kind, "payload": payload })).collect::<Vec<_>>());
    let scratch = std::env::temp_dir().join(format!("unreadable-disposition-cases-{}.json", std::process::id()));
    fs::write(&scratch, serde_json::to_vec(&input).unwrap()).unwrap();
    let python = std::process::Command::new("python3").current_dir(repo_root())
        .args(["-c", r#"
import json, sys
from core.library_import.provenance_checker import validate_library_import
out=[]
for c in json.load(open(sys.argv[1])):
 for v in ('private','public'):
  r=validate_library_import(c['payload'],library_kind=c['kind'],intended_visibility=v)
  out.append({'outcome':r.outcome,'findings':[[f.code,f.path,f.severity,f.message,f.remediation] for f in r.findings]})
print(json.dumps(out))
"#, scratch.to_str().unwrap()]).output().expect("Python parity runner");
    let _ = fs::remove_file(&scratch);
    assert!(python.status.success(), "{}", String::from_utf8_lossy(&python.stderr));
    let expected: Value = serde_json::from_slice(&python.stdout).unwrap();
    let mut index = 0;
    for (kind, label, refused, payload) in &cases {
        for visibility in [IntendedVisibility::Private, IntendedVisibility::Public] {
            let result = validate_library_import(payload, LibraryKind::from_token(kind).unwrap(), visibility);
            let tuples: Vec<Value> = result.findings.iter().map(|f| json!([f.code, f.path, f.severity, f.message, f.remediation])).collect();
            assert_eq!(json!({ "outcome": result.outcome, "findings": tuples }), expected[index], "{kind}/{label}/{visibility:?}");
            let named = result.findings.iter().any(|f| f.code == DISPOSITION_RECORD_MALFORMED);
            assert_eq!(named, *refused, "{kind}/{label}/{visibility:?}: {:?}", codes(&result));
            if *refused {
                assert!(!result.accepted, "{kind}/{label}");
            }
            index += 1;
        }
        if !refused {
            let private = validate_library_import(payload, LibraryKind::from_token(kind).unwrap(), IntendedVisibility::Private);
            assert!(private.accepted, "{kind}/{label}: the control imports privately: {:?}", codes(&private));
        }
    }
    assert_eq!(index, cases.len() * 2);
    assert_eq!(cases.len(), 4 * (1 + 16 + 2) + 3);
}

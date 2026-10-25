//! Rule-pack format 0.5.0 at validation (W-17; DEL-06-01 Design
//! `rule-pack-0.5.0.md` §2.1, §2.5, §2.9 and §5 "Format").
//!
//! Every pack is the invented demonstration pack or an edit of it; category,
//! load case and mapping names are invented. No protected standards content.
//! Nothing here is a professional, certification, sealing, authentication, or
//! code-compliance claim.

use open_pipe_stress_rule_pack_document::{
    compute_rule_pack_checksum, plan_check_bases, validate_rule_pack_document, BasisRef,
    RulePackDocumentValidation, RULE_PACK_DUPLICATE_ID, RULE_PACK_MAPPING_INCOMPLETE,
    RULE_PACK_REFERENCE_UNRESOLVED, RULE_PACK_SCHEMA_VERSION, RULE_PACK_SCHEMA_VERSION_UNSUPPORTED,
    RULE_SOLVER_RESULT_REF_MISSING,
};
use serde_json::{json, Value};

fn demo() -> Value {
    serde_json::from_str(include_str!(
        "../../../../examples/rule_packs/invented_demo.yaml"
    ))
    .expect("demo pack is strict JSON")
}

/// Re-stamp the checksum so only the edit under test is a finding.
fn restamped(mut pack: Value) -> Value {
    let computed = compute_rule_pack_checksum(&pack).expect("checksum computes");
    pack["checksums"]["rule_pack_checksum"]["value"] = json!(computed.value);
    pack
}

fn validate(pack: &Value) -> RulePackDocumentValidation {
    validate_rule_pack_document(pack, false)
}

fn codes(outcome: &RulePackDocumentValidation) -> Vec<&str> {
    outcome
        .document_findings
        .iter()
        .map(|finding| finding.code.as_str())
        .collect()
}

fn category_ref(id: &str) -> Value {
    json!({"ref_id": id, "ref_type": "stress_category"})
}

fn load_case_ref(id: &str) -> Value {
    json!({"ref_id": id, "ref_type": "required_load_case"})
}

#[test]
fn the_demo_pack_is_current_and_validates_clean() {
    let outcome = validate(&demo());
    assert_eq!(RULE_PACK_SCHEMA_VERSION, "0.5.0");
    assert!(
        outcome.document_findings.is_empty(),
        "{:?}",
        outcome.document_findings
    );
    assert!(!outcome.has_blocking_findings);
    assert_eq!(outcome.checksum.match_status, "match");
}

#[test]
fn other_versions_are_refused_by_name_and_the_checksum_is_still_computed() {
    let edits: [(&str, fn(&mut Value)); 4] = [
        ("0.4.0", |pack| {
            pack["schema_version"] = json!("0.4.0");
            pack["metadata"]["schema_version"] = json!("0.4.0");
        }),
        ("absent", |pack| {
            pack.as_object_mut().unwrap().remove("schema_version");
        }),
        ("metadata absent", |pack| {
            pack["metadata"]
                .as_object_mut()
                .unwrap()
                .remove("schema_version");
        }),
        ("mismatched metadata", |pack| {
            pack["metadata"]["schema_version"] = json!("0.4.0");
        }),
    ];
    for (label, edit) in edits {
        let mut pack = demo();
        edit(&mut pack);
        let pack = restamped(pack);
        let outcome = validate(&pack);
        assert!(outcome.has_blocking_findings, "{label}");
        let refusal = outcome
            .document_findings
            .iter()
            .find(|finding| finding.code == RULE_PACK_SCHEMA_VERSION_UNSUPPORTED)
            .unwrap_or_else(|| panic!("{label}: {:?}", outcome.document_findings));
        assert!(refusal.blocking, "{label}");
        assert!(
            refusal.message.contains("'0.5.0'"),
            "{label}: {}",
            refusal.message
        );
        // A refused pack is never also a checksum mismatch (design §2.1).
        assert_eq!(outcome.checksum.match_status, "match", "{label}");
        assert!(outcome.checksum.computed.is_some(), "{label}");
        assert!(!codes(&outcome).contains(&"CHECKSUM_MISMATCH"), "{label}");
    }
    // The 0.4.0 refusal names the version found.
    let mut pack = demo();
    pack["schema_version"] = json!("0.4.0");
    pack["metadata"]["schema_version"] = json!("0.4.0");
    let outcome = validate(&restamped(pack));
    let message = &outcome.document_findings[0].message;
    assert!(message.contains("'0.4.0'"), "{message}");
}

/// Each §2.9 finding, reached by one edit of the valid demo pack.
#[test]
fn each_static_finding_is_reached() {
    type Edit = fn(&mut Value);
    let cases: Vec<(&str, Edit, &str)> = vec![
        (
            "mapping names an unknown category",
            |p| {
                p["load_case_mappings"][0]["category_ref"] = category_ref("no_such_category");
            },
            RULE_PACK_REFERENCE_UNRESOLVED,
        ),
        (
            "mapping names an unknown load case",
            |p| {
                p["load_case_mappings"][0]["required_load_case_refs"][1] =
                    load_case_ref("no_such_case");
            },
            RULE_PACK_REFERENCE_UNRESOLVED,
        ),
        (
            "check names an unknown category",
            |p| {
                p["check_definitions"][0]["category_ref"] = category_ref("no_such_category");
            },
            RULE_PACK_REFERENCE_UNRESOLVED,
        ),
        (
            "check names no category",
            |p| {
                p["check_definitions"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("category_ref");
            },
            RULE_PACK_REFERENCE_UNRESOLVED,
        ),
        (
            "duplicate required load case id",
            |p| {
                p["required_load_cases"][1]["required_load_case_id"] =
                    json!("demo_required_case_1");
            },
            RULE_PACK_DUPLICATE_ID,
        ),
        (
            "duplicate basis_ref",
            |p| {
                p["required_load_cases"][1]["basis_ref"]["ref_id"] = json!("load:L-100");
            },
            RULE_PACK_DUPLICATE_ID,
        ),
        (
            "duplicate category id",
            |p| {
                let copy = p["stress_categories"][0].clone();
                p["stress_categories"].as_array_mut().unwrap().push(copy);
            },
            RULE_PACK_DUPLICATE_ID,
        ),
        (
            "same load case twice in one mapping",
            |p| {
                p["load_case_mappings"][0]["required_load_case_refs"][1] =
                    load_case_ref("demo_required_case_1");
            },
            RULE_PACK_DUPLICATE_ID,
        ),
        (
            "category with no mapping",
            |p| {
                p["stress_categories"].as_array_mut().unwrap().push(json!({
                    "category_id": "demo_category_b", "name": "Invented category B"
                }));
                let mut check = p["check_definitions"][0].clone();
                check["check_id"] = json!("demo_second_check");
                check["category_ref"] = category_ref("demo_category_b");
                p["check_definitions"].as_array_mut().unwrap().push(check);
            },
            RULE_PACK_MAPPING_INCOMPLETE,
        ),
        (
            "category with two mappings",
            |p| {
                let mut copy = p["load_case_mappings"][0].clone();
                copy["mapping_id"] = json!("demo_mapping_copy");
                p["load_case_mappings"].as_array_mut().unwrap().push(copy);
            },
            RULE_PACK_MAPPING_INCOMPLETE,
        ),
        (
            "required load case in no mapping",
            |p| {
                p["load_case_mappings"][0]["required_load_case_refs"]
                    .as_array_mut()
                    .unwrap()
                    .pop();
            },
            RULE_PACK_MAPPING_INCOMPLETE,
        ),
        (
            "no required load cases",
            |p| {
                p["required_load_cases"] = json!([]);
                p["load_case_mappings"][0]["required_load_case_refs"] = json!([]);
            },
            RULE_PACK_MAPPING_INCOMPLETE,
        ),
        (
            "solver input without a selector",
            |p| {
                p["required_inputs"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("solver_result_ref");
            },
            RULE_SOLVER_RESULT_REF_MISSING,
        ),
        (
            "solver input with a 0.4.0 result_id selector",
            |p| {
                p["required_inputs"][0]["solver_result_ref"] =
                    json!({"result_id": "result:stress:demo"});
            },
            RULE_SOLVER_RESULT_REF_MISSING,
        ),
        (
            "selector carrying result_id beside the new members",
            |p| {
                p["required_inputs"][0]["solver_result_ref"]["result_id"] =
                    json!("result:stress:demo");
            },
            RULE_SOLVER_RESULT_REF_MISSING,
        ),
        (
            "selector with an unknown section basis",
            |p| {
                p["required_inputs"][0]["solver_result_ref"]["section_basis"] =
                    json!("invented_section");
            },
            RULE_SOLVER_RESULT_REF_MISSING,
        ),
    ];
    for (label, edit, expected) in cases {
        let mut pack = demo();
        edit(&mut pack);
        let outcome = validate(&restamped(pack));
        assert!(outcome.has_blocking_findings, "{label}");
        assert!(
            codes(&outcome).contains(&expected),
            "{label}: expected {expected}, got {:?}",
            outcome.document_findings
        );
        assert!(
            !codes(&outcome).contains(&RULE_PACK_SCHEMA_VERSION_UNSUPPORTED),
            "{label}"
        );
    }
}

/// Design §2.9 (review round 2, finding 5): a required load case mapped only
/// to a category no check names is refused, so a missing load case cannot
/// escape the aggregate.
#[test]
fn a_required_load_case_no_check_reaches_is_refused() {
    let mut pack = demo();
    pack["required_load_cases"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "required_load_case_id": "demo_required_case_3",
            "name": "Invented required load case 3",
            "basis_ref": {"ref_type": "combination", "ref_id": "combination:C-OPER-ALT"}
        }));
    pack["stress_categories"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "category_id": "demo_category_unused", "name": "Invented category with no checks"
        }));
    pack["load_case_mappings"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "mapping_id": "demo_mapping_unused",
            "category_ref": category_ref("demo_category_unused"),
            "required_load_case_refs": [load_case_ref("demo_required_case_3")]
        }));
    let outcome = validate(&restamped(pack.clone()));
    let unreached: Vec<_> = outcome
        .document_findings
        .iter()
        .filter(|finding| finding.code == RULE_PACK_MAPPING_INCOMPLETE)
        .collect();
    assert_eq!(unreached.len(), 1, "{:?}", outcome.document_findings);
    assert_eq!(unreached[0].subject, "demo_required_case_3");
    assert!(unreached[0].message.contains("reached by no check"));

    // The same load case also mapped by the checked category is reached: a
    // load case may feed more than one category (design §2.3).
    pack["load_case_mappings"][0]["required_load_case_refs"]
        .as_array_mut()
        .unwrap()
        .push(load_case_ref("demo_required_case_3"));
    let outcome = validate(&restamped(pack));
    assert!(
        outcome.document_findings.is_empty(),
        "{:?}",
        outcome.document_findings
    );
}

#[test]
fn a_load_case_shared_by_two_categories_validates_and_plans_in_mapping_order() {
    let mut pack = demo();
    pack["stress_categories"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "category_id": "demo_category_b", "name": "Invented category B"
        }));
    pack["load_case_mappings"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "mapping_id": "demo_mapping_b",
            "category_ref": category_ref("demo_category_b"),
            "required_load_case_refs": [load_case_ref("demo_required_case_2")]
        }));
    let mut check = pack["check_definitions"][0].clone();
    check["check_id"] = json!("demo_second_check");
    check["category_ref"] = category_ref("demo_category_b");
    pack["check_definitions"]
        .as_array_mut()
        .unwrap()
        .push(check);
    let outcome = validate(&restamped(pack.clone()));
    assert!(
        outcome.document_findings.is_empty(),
        "{:?}",
        outcome.document_findings
    );

    let plans = plan_check_bases(&pack);
    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].check_id, "demo_training_check");
    assert_eq!(plans[0].category_ref, "demo_category_a");
    let first: Vec<BasisRef> = plans[0]
        .bases
        .as_ref()
        .unwrap()
        .iter()
        .map(|basis| basis.basis_ref.clone())
        .collect();
    assert_eq!(
        first,
        vec![
            BasisRef::new("load_case", "load:L-100"),
            BasisRef::new("load_case", "load:L-200")
        ]
    );
    let second = plans[1].bases.as_ref().unwrap();
    assert_eq!(second.len(), 1);
    assert_eq!(second[0].required_load_case_id, "demo_required_case_2");
}

#[test]
fn a_structural_defect_gives_the_check_no_basis_list() {
    let mut pack = demo();
    pack["check_definitions"][0]["category_ref"] = category_ref("no_such_category");
    let plans = plan_check_bases(&pack);
    assert!(plans[0].bases.is_err());
    let mut pack = demo();
    pack["load_case_mappings"][0]["required_load_case_refs"][1] =
        load_case_ref("demo_required_case_1");
    assert!(plan_check_bases(&pack)[0].bases.is_err());
}

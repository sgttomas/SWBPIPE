//! Object form on the rule check (W-17; DEL-06-01 Design `rule-pack-0.5.0.md`
//! §6c). #1650 found that serde's typed decoders read a struct written as a
//! JSON array (its positional form) while the native screens read members by
//! key. The rule check has no such split: the pack, the model, the solved
//! envelope, the invocation and the supplied values reach it as
//! `serde_json::Value` and are read by key both by its gates (pack
//! validation and lifecycle, mechanics qualification) and by its engine (row
//! selection, the runner); no typed decoder reads any of them. A record in
//! positional form therefore reads as absent to both, and each gate refuses
//! an absent record. These tests pin that: every record a gate reads,
//! written positionally (with a blocking disposition where it has one),
//! refuses, and the run never evaluates a check. Packs, models and values are
//! invented; the envelope is a committed result fixture. Nothing here is a
//! professional or code-compliance claim.

use super::*;
use crate::command_ingress::test_support::{admit_json, registry};
use crate::memory_pool::test_host::ScriptedHost;
use serde_json::json;
use std::path::Path;

fn project_file(relative: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(relative);
    let text = fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_str(&text).unwrap()
}

fn restamp(mut pack: Value) -> Value {
    let computed = rule_pack_document::compute_rule_pack_checksum(&pack).expect("checksum recomputes");
    pack["checksums"]["rule_pack_checksum"]["value"] = json!(computed.value);
    pack
}

/// [`restamp`] where the checksum is computable (a pack without a readable
/// identity has none, and its validation names that instead).
fn restamp_if_computable(pack: Value) -> Value {
    if rule_pack_document::compute_rule_pack_checksum(&pack).is_ok() { restamp(pack) } else { pack }
}

/// The invented demo pack with a selector naming the demo envelope's row.
fn pack() -> Value {
    let mut pack = project_file("examples/rule_packs/invented_demo.yaml");
    pack["required_inputs"][0]["solver_result_ref"] = json!({"result_kind": "stress", "entity_ref": "pipe:demo"});
    restamp(pack)
}

fn model() -> Value {
    json!({
        "project": {"id": "project:invented-rule-demo"},
        "load_cases": [{"id": "load:L-100"}, {"id": "load:L-200"}],
        "combinations": [{"id": "combination:C-OPER-ALT"}]
    })
}

fn envelope() -> Value {
    let row = |id: &str, case: &str| {
        json!({"id": id, "kind": "stress", "value": 50.0, "unit": "demo_unit", "entity_ref": "pipe:demo",
               "basis_ref": {"ref_type": "load_case", "ref_id": case}})
    };
    json!({
        "status": {"mechanics": "MECHANICS_SOLVED", "rule_check": "RULE_INPUTS_INCOMPLETE"},
        "results": [row("result:stress:demo", "load:L-100"), row("result:loadcase:load-L-200:stress:demo", "load:L-200")]
    })
}

fn supplied() -> Value {
    json!([
        {"ref_id": "demo_limit_quantity", "value": 100.0, "unit": "demo_unit", "dimension": "stress"},
        {"ref_id": "demo_limit_slot", "value": 1.0, "unit": "ratio", "dimension": "dimensionless"}
    ])
}

/// The positional form of an object: its member values, in order.
fn positional(value: &Value) -> Value {
    Value::Array(value.as_object().expect("an object").values().cloned().collect())
}

fn make_positional(document: &mut Value, pointer: &str) {
    let target = document.pointer_mut(pointer).unwrap_or_else(|| panic!("{pointer}"));
    *target = positional(target);
}

fn run(pack: Value, envelope: Value, supplied: Value) -> Result<Value, String> {
    run_rule_checks_core(pack, &model(), envelope, None, Some(supplied), None)
}

/// The blocking validation codes of a refused run.
fn blocking_codes(outcome: &Value) -> Vec<String> {
    let validation = &outcome["rule_pack_validation"];
    ["document_findings", "lifecycle_findings"]
        .iter()
        .flat_map(|key| validation[*key].as_array().cloned().unwrap_or_default())
        .filter(|finding| finding["blocking"] == true)
        .map(|finding| finding["code"].as_str().unwrap().to_string())
        .collect()
}

#[test]
fn the_object_form_control_runs_and_a_blocking_object_disposition_refuses() {
    let outcome = run(pack(), envelope(), supplied()).unwrap();
    assert_eq!(outcome["aggregate_status"], "USER_RULE_CHECKED");
    for (pointer, value, code) in [
        ("/provenance/review_status", "quarantined", "PROTECTED_CONTENT_SUSPECTED"),
        ("/classification/redistribution_status", "protected_suspected", "PROTECTED_CONTENT_SUSPECTED"),
        ("/provenance/protected_content_review", "quarantined_protected_suspected", "PROTECTED_CONTENT_SUSPECTED"),
    ] {
        let mut blocked = pack();
        *blocked.pointer_mut(pointer).unwrap() = json!(value);
        let outcome = run(restamp(blocked), envelope(), supplied()).unwrap();
        assert_eq!(outcome["aggregate_status"], "RULE_INPUTS_INCOMPLETE", "{pointer}");
        assert_eq!(outcome["checks"], json!([]), "{pointer}");
        assert!(blocking_codes(&outcome).iter().any(|found| found == code), "{pointer}: {:?}", blocking_codes(&outcome));
    }
}

/// Every pack record the validation or lifecycle gate reads, positional (with
/// a blocking disposition written into it where it carries one, and the pack
/// re-stamped so that its checksum matches): the run is refused before any
/// check is evaluated, naming the record as absent.
#[test]
fn a_positional_pack_record_refuses_the_run_before_any_check() {
    let cases: [(&str, Option<(&str, &str)>, &str); 9] = [
        ("/provenance", Some(("review_status", "quarantined")), "MISSING_REVIEW_STATUS"),
        ("/provenance", Some(("review_status", "rejected")), "MISSING_REVIEW_STATUS"),
        ("/provenance", Some(("protected_content_review", "quarantined_protected_suspected")), "MISSING_REVIEW_STATUS"),
        ("/classification", Some(("redistribution_status", "protected_suspected")), "MISSING_REDISTRIBUTION_STATUS"),
        ("/metadata", Some(("lifecycle_status", "quarantined")), "MISSING_RULE_PACK_IDENTITY"),
        ("/professional_boundary", None, "PROFESSIONAL_BOUNDARY_VIOLATION"),
        ("/required_load_cases/0/provenance", Some(("review_status", "quarantined")), rule_pack_document::RULE_PACK_RECORD_INCOMPLETE),
        ("/stress_categories/0/provenance", Some(("redistribution_status", "protected_suspected")), rule_pack_document::RULE_PACK_RECORD_INCOMPLETE),
        ("/required_inputs/0/solver_result_ref", None, rule_pack_document::RULE_SOLVER_RESULT_REF_MISSING),
    ];
    for (pointer, disposition, code) in cases {
        let mut probe = pack();
        if let Some((member, value)) = disposition {
            probe.pointer_mut(pointer).unwrap()[member] = json!(value);
        }
        make_positional(&mut probe, pointer);
        let probe = restamp_if_computable(probe);
        let outcome = run(probe, envelope(), supplied()).unwrap();
        assert_eq!(outcome["aggregate_status"], "RULE_INPUTS_INCOMPLETE", "{pointer}");
        assert_eq!(outcome["checks"], json!([]), "{pointer}: no check is evaluated");
        let codes = blocking_codes(&outcome);
        assert!(codes.iter().any(|found| found == code), "{pointer}: {codes:?}");
        assert!(!codes.iter().any(|found| found == "CHECKSUM_MISMATCH"), "{pointer}: the stamp matches, so the record itself refuses");
    }
    // The checksum record itself, positional: no declared checksum.
    let mut probe = pack();
    make_positional(&mut probe, "/checksums/rule_pack_checksum");
    let outcome = run(probe, envelope(), supplied()).unwrap();
    assert_eq!(outcome["checks"], json!([]));
    assert!(blocking_codes(&outcome).iter().any(|found| found == "MISSING_CHECKSUM"), "{:?}", blocking_codes(&outcome));
}

/// A positional check definition, required input or formula is not read as
/// one: the run evaluates nothing from it and never reports a check passed.
#[test]
fn a_positional_check_input_or_formula_never_reads_as_checked() {
    for pointer in ["/check_definitions/0", "/required_inputs/0", "/required_inputs/1", "/formula_declarations/0"] {
        let mut probe = pack();
        make_positional(&mut probe, pointer);
        let outcome = run(restamp(probe), envelope(), supplied()).unwrap();
        assert_ne!(outcome["aggregate_status"], "USER_RULE_CHECKED", "{pointer}: {outcome}");
        assert!(
            outcome["checks"].as_array().unwrap().iter().all(|check| check["status"] != "USER_RULE_CHECKED"),
            "{pointer}: {outcome}"
        );
    }
}

/// A value slot declaration supplies only the limit's declared dimension and
/// unit; the limit's value is the supplied one, and a slot the pack does not
/// declare takes its unit from the entered value (the unit check's warning
/// `RULE_PACK_LIMIT_SLOT_UNDECLARED`). A positional slot reads exactly as an
/// undeclared one: the same outcome and the same warning, with no member of
/// it read.
#[test]
fn a_positional_value_slot_reads_as_an_undeclared_slot() {
    let mut undeclared = pack();
    undeclared["value_slots"] = json!([]);
    let undeclared = restamp(undeclared);
    let mut probe = pack();
    probe["value_slots"][0]["review_status"] = json!("quarantined");
    probe["value_slots"][0]["provenance"]["review_status"] = json!("quarantined");
    make_positional(&mut probe, "/value_slots/0");
    let probe = restamp(probe);
    let warned = |pack: &Value| {
        rule_pack_document::validate_rule_pack_document(pack, false)
            .unit_check
            .checks
            .iter()
            .flat_map(|check| check.findings.iter())
            .any(|finding| finding.code == "RULE_PACK_LIMIT_SLOT_UNDECLARED")
    };
    assert!(warned(&undeclared) && warned(&probe));
    assert!(!warned(&pack()));
    let expected = run(undeclared, envelope(), supplied()).unwrap();
    let outcome = run(probe, envelope(), supplied()).unwrap();
    assert_eq!(outcome["checks"], expected["checks"]);
    assert_eq!(outcome["aggregate_status"], expected["aggregate_status"]);
}

/// The solved envelope's rows and the supplied values, positional: a row is
/// not selected (its input stays unsupplied), and a supplied value is refused
/// by name. (The envelope's `status` is the mechanics qualification's, which
/// runs first in the command: `a_positional_member_refuses_the_mechanics_qualification`.)
#[test]
fn a_positional_row_or_supplied_value_is_not_bound() {
    for pointer in ["/results/0", "/results/0/basis_ref"] {
        let mut probe = envelope();
        make_positional(&mut probe, pointer);
        let outcome = run(pack(), probe, supplied()).unwrap();
        assert_eq!(outcome["aggregate_status"], "RULE_INPUTS_INCOMPLETE", "{pointer}: {outcome}");
        assert!(
            outcome["checks"].as_array().unwrap().iter().any(|check| check["status"] == "RULE_INPUTS_INCOMPLETE"),
            "{pointer}: {outcome}"
        );
    }
    let mut values = supplied();
    make_positional(&mut values, "/0");
    let error = run(pack(), envelope(), values).unwrap_err();
    assert_eq!(error, "each supplied value binding requires ref_id");
}

/// The mechanics qualification on an admitted exact source: the model's
/// project or a load case, the envelope's status, producer or numerical
/// quality, and the invocation's request, each positional, refuse by name.
#[test]
fn a_positional_member_refuses_the_mechanics_qualification() {
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    let model = request["model"].clone();
    let source = project_file("fixtures/results/pressure_connected_mechanics_sparse.json");
    qualify_rule_mechanics_with_context(&model, &source, None).expect("the control is admitted");
    for (pointer, prefix) in [("/project", "RULE_NUMERICAL_CASE_COVERAGE_UNAVAILABLE"), ("/load_cases/0", "RULE_NUMERICAL_CASE_COVERAGE_UNAVAILABLE")] {
        let mut probe = model.clone();
        make_positional(&mut probe, pointer);
        let error = qualify_rule_mechanics_with_context(&probe, &source, None).unwrap_err();
        assert!(error.starts_with(prefix), "{pointer}: {error}");
    }
    for (pointer, prefix) in [
        ("/status", "RULE_MECHANICS_NOT_SOLVED"),
        ("/producer", "RULE_NUMERICAL_SOURCE_UNSUPPORTED"),
        ("/numerical_quality", "RULE_NUMERICAL_"),
        ("/contract_evidence", "RULE_NUMERICAL_"),
    ] {
        let mut probe = source.clone();
        make_positional(&mut probe, pointer);
        let error = qualify_rule_mechanics_with_context(&model, &probe, None).unwrap_err();
        assert!(error.starts_with(prefix), "{pointer}: {error}");
    }
    let invocation = json!({"request": positional(&request), "solver_mode": "sparse_interactive"});
    let error = qualify_rule_mechanics_with_context(&model, &source, Some(&invocation)).unwrap_err();
    assert_eq!(error, "RULE_NUMERICAL_INVOCATION_MODEL_MISMATCH");
}

/// The raw IPC form (`args-json-v1`, `run_rule_checks_args`): positional
/// arguments are refused at ingress, and a positional, quarantined pack
/// provenance refuses the run before any check, as its object form does.
#[test]
fn the_raw_ipc_form_refuses_positional_arguments_and_records() {
    let pool = registry(ScriptedHost::normal());
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    let source = project_file("fixtures/results/pressure_connected_mechanics_sparse.json");
    let no_store = || -> Result<PathBuf, String> { Err("no store in this test".to_string()) };
    let positional_args = json!([pack(), request["model"], source]);
    let error = admit_json(&pool, "run_rule_checks", ArgsDecoding::Moved, &positional_args).unwrap_err();
    assert!(format!("{error:?}").contains("INGRESS-ARGS-NOT-OBJECT"), "{error:?}");
    for positional_provenance in [false, true] {
        let mut probe = pack();
        probe["provenance"]["review_status"] = json!("quarantined");
        if positional_provenance {
            make_positional(&mut probe, "/provenance");
        }
        let args = json!({"rulePackDocument": restamp(probe), "model": request["model"], "solvedEnvelope": source});
        let (mut args, _lease) = admit_json(&pool, "run_rule_checks", ArgsDecoding::Moved, &args).expect("admitted");
        let outcome = run_rule_checks_args(&mut args, &no_store).expect("the run answers");
        assert_eq!(outcome["aggregate_status"], "RULE_INPUTS_INCOMPLETE", "{positional_provenance}");
        assert_eq!(outcome["checks"], json!([]), "{positional_provenance}");
        let expected = if positional_provenance { "MISSING_REVIEW_STATUS" } else { "PROTECTED_CONTENT_SUSPECTED" };
        assert!(blocking_codes(&outcome).iter().any(|found| found == expected), "{positional_provenance}: {:?}", blocking_codes(&outcome));
    }
}

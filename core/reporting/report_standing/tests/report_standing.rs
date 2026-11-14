//! DEL-05-02 Standing and DEL-08-01 REQ-024 on real producer output: the
//! reader reads the two report-standing witnesses that are not numerically
//! eligible, and derives their standing records under the historical
//! vocabularies; an inconsistent case record and a code outside the
//! vocabulary make a result unreadable rather than giving it a known meaning.
use open_pipe_stress_report_standing::reader_readability::{readability, NUMERICAL_CASE_RECORD_INCONSISTENT};
use open_pipe_stress_result_export::reader_verdict::standing;
use open_pipe_stress_report_standing::report_standing::historical_standing_record;
use serde_json::{json, Value};

const GAP_RAW: &str = include_str!("../../../../fixtures/reports/invented/report_standing_witnesses/pressure_3_gap_unresolved-sparse_interactive.raw.json");
const GAP_REQUEST: &str = include_str!("../../../../validation/references/rt_a3_s6/requests/c1_gap.json");
const FRICTION_RAW: &str =
    include_str!("../../../../fixtures/reports/invented/report_standing_witnesses/pressure_3_coupled_friction_sensitive-sparse_interactive.raw.json");
const FRICTION_REQUEST: &str = include_str!("../../../../fixtures/reports/invented/report_standing_witnesses/coupled_friction.request.json");

fn read(raw: &str, request: &str) -> (Value, Value, Value) {
    let source: Value = serde_json::from_str(raw).unwrap();
    let model = serde_json::from_str::<Value>(request).unwrap()["model"].clone();
    let invocation = json!({"request": {"model": model.clone(), "materials": []}, "solver_mode": "sparse_interactive"});
    let standing = standing(&source, Some(&model), Some(&invocation));
    (source, model, standing)
}

fn case<'a>(record: &'a Value, id: &str) -> &'a Value {
    record["cases"].as_array().unwrap().iter().find(|c| c["case_id"] == id).unwrap_or_else(|| panic!("no case {id}"))
}

#[test]
fn the_gap_witness_is_read_and_states_an_unresolved_unconfirmed_case() {
    let (source, model, standing) = read(GAP_RAW, GAP_REQUEST);
    assert_eq!(standing["eligible"], false);
    assert_eq!(readability(&source, &standing), json!({"readable": true, "solved": true, "unreadable": []}));
    let record = historical_standing_record(&source, &model, &standing).unwrap();
    assert_eq!(record["record_version"], "historical_1");
    assert_eq!(record["figures"], json!({"status": "unavailable", "reason": "older_document"}));
    assert_eq!(record["standing_findings"], json!(["NUMERICAL_INTEGRITY_NOT_QUALIFIED", "NUMERICAL_CASE_EVIDENCE_INCOMPLETE"]));
    for id in ["load:PRESSURE", "load:VACUUM"] {
        let c = case(&record, id);
        assert_eq!(c["health"], "quality_unresolved", "{id}");
        assert_eq!(c["integrity_code"], "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "{id}");
        assert_eq!(c["causes"].as_array().unwrap().iter().map(|x| x["cause"].clone()).collect::<Vec<_>>(), vec![json!("recovery_basis_unqualified")], "{id}");
        // No sentence is published: not recorded, never "not attempted".
        assert_eq!(c["confirmation"], json!({"outcome": "no_confirmation_record", "detail": null, "status": "not_established", "premises": []}), "{id}");
        assert_eq!(c["supports"][0]["support_id"], "support:GUIDE");
        assert_eq!(c["supports"][0]["behavior"], "gap");
    }
    assert_eq!(case(&record, "load:PRESSURE")["supports"][0]["state"], "active");
    assert_eq!(case(&record, "load:VACUUM")["supports"][0]["state"], "inactive");
}

#[test]
fn the_coupled_friction_witness_is_read_and_states_a_sensitive_case_with_a_conditional_confirmation() {
    let (source, model, standing) = read(FRICTION_RAW, FRICTION_REQUEST);
    assert_eq!(standing["eligible"], false);
    assert_eq!(readability(&source, &standing), json!({"readable": true, "solved": true, "unreadable": []}));
    let record = historical_standing_record(&source, &model, &standing).unwrap();
    let c = case(&record, "load:RS-SUS");
    assert_eq!(c["health"], "sensitive");
    assert_eq!(c["integrity_code"], "NUMERICAL_INTEGRITY_SENSITIVE");
    assert!(c["causes"].as_array().unwrap().iter().any(|x| x["cause"] == "formation_recovery"), "{}", c["causes"]);
    assert_eq!(c["confirmation"], json!({"outcome": "solver_reported_established", "detail": null, "status": "conditional", "premises": ["inverse_norm_step"]}));
    let fr1 = c["supports"].as_array().unwrap().iter().find(|s| s["support_id"] == "support:RS-FR1").unwrap();
    assert_eq!(fr1["behavior"], "friction");
    assert_eq!(fr1["friction_normal"], "derived");
    assert_eq!(fr1["state"], "sliding");
    assert_eq!(record["counts"], json!([
        {"health": "checks_passed", "cases": 0}, {"health": "sensitive", "cases": 1}, {"health": "quality_unresolved", "cases": 0},
        {"health": "solve_failed", "cases": 0}, {"health": "not_assessed", "cases": 0}
    ]));
}

#[test]
fn an_inconsistent_case_record_makes_the_result_unreadable() {
    let (mut source, model, _) = read(GAP_RAW, GAP_REQUEST);
    source["numerical_quality"]["cases"][0]["model_matrix_fidelity"] = json!("represented_equations_retained");
    let invocation = json!({"request": {"model": model.clone(), "materials": []}, "solver_mode": "sparse_interactive"});
    let standing = standing(&source, Some(&model), Some(&invocation));
    let verdict = readability(&source, &standing);
    // The route's evidence reader refuses it on its own (its nonlinear standing check), and
    // the record check names the inconsistency beside it.
    assert_eq!(verdict["readable"], false);
    assert_eq!(verdict["unreadable"], json!(["SOURCE_PHYSICS_NONLINEAR_STANDING", NUMERICAL_CASE_RECORD_INCONSISTENT]));
}

#[test]
fn an_integrity_code_outside_the_vocabulary_is_never_given_a_known_meaning() {
    let (mut source, model, _) = read(GAP_RAW, GAP_REQUEST);
    for d in source["diagnostics"].as_array_mut().unwrap() {
        if d["id"] == "diagnostic:numerical-integrity:load:PRESSURE" {
            d["code"] = json!("NUMERICAL_INTEGRITY_SENSITIVE_ENOUGH");
        }
    }
    let invocation = json!({"request": {"model": model.clone(), "materials": []}, "solver_mode": "sparse_interactive"});
    let standing = standing(&source, Some(&model), Some(&invocation));
    assert_eq!(readability(&source, &standing)["unreadable"], json!(["SOURCE_PHYSICS_NONLINEAR_STANDING_CODE", NUMERICAL_CASE_RECORD_INCONSISTENT]));
    assert_eq!(historical_standing_record(&source, &model, &standing).unwrap_err(), "NUMERICAL_TAG_UNSUPPORTED: historical_integrity_code");
}

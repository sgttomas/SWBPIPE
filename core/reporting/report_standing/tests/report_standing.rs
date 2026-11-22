//! DEL-05-02 Standing and DEL-08-01 REQ-024 on real producer output: the
//! reader reads the two report-standing witnesses that are not numerically
//! eligible, and derives their standing records under the historical
//! vocabularies; an inconsistent case record and a code outside the
//! vocabulary make a result unreadable rather than giving it a known meaning.
use open_pipe_stress_numerical_vocabulary::{IntegrityBound, IntegrityBoundReason, VocabularyTag};
use open_pipe_stress_report_standing::reader_readability::{readability, NUMERICAL_CASE_RECORD_INCONSISTENT};
use open_pipe_stress_result_export::reader_verdict::{integrity_bound, standing};
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

/// The reader's `integrity_bound` tags are the numerical vocabulary's, one
/// to one: every tag the reader emits parses back to itself, and every tag
/// the vocabulary lists is one the reader emits. Every verdict carries the
/// same bound, on the real witnesses and on an unsupported refusal.
#[test]
fn reader_emitted_integrity_bound_tags_match_the_vocabulary() {
    fn tag<T: VocabularyTag>(text: &str) -> T {
        T::parse(text).unwrap_or_else(|| panic!("{text} is not in the {} vocabulary", T::VOCABULARY))
    }
    let bound = integrity_bound();
    let (status, reason) = (bound["status"].as_str().unwrap(), bound["reason"].as_str().unwrap());
    let (parsed_status, parsed_reason) = (tag::<IntegrityBound>(status), tag::<IntegrityBoundReason>(reason));
    assert_eq!(IntegrityBound::ALL, &[parsed_status]);
    assert_eq!(IntegrityBoundReason::ALL, &[parsed_reason]);
    assert_eq!(parsed_status.as_str(), status);
    assert_eq!(parsed_reason.as_str(), reason);
    assert_eq!(bound.as_object().unwrap().len(), 2, "{bound}");

    for (raw, request) in [(GAP_RAW, GAP_REQUEST), (FRICTION_RAW, FRICTION_REQUEST)] {
        let (_, _, standing) = read(raw, request);
        assert_eq!(standing["integrity_bound"], bound);
    }
    let refused = standing(&json!({}), None, None);
    assert_eq!(refused["route"], "unsupported");
    assert_eq!(refused["integrity_bound"], bound);

    // Negative controls: a verified bound is not a tag of either vocabulary.
    assert!(IntegrityBound::parse("verified").is_none());
    assert!(IntegrityBoundReason::parse("inverse_norm_bound_verified").is_none());
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

/// The not-assessed record is the producer's only for a case with no integrity
/// diagnostic: with forged empty evidence beside a published diagnostic, the
/// case record is inconsistent (scrutiny of PR 328, finding 3a).
#[test]
fn a_not_assessed_case_record_beside_its_integrity_diagnostic_is_inconsistent() {
    let (mut source, _, _) = read(GAP_RAW, GAP_REQUEST);
    assert!(open_pipe_stress_report_standing::reader_readability::case_records_consistent(&source));
    let case = &mut source["numerical_quality"]["cases"][0];
    assert_eq!(case["basis_ref"]["ref_id"], "load:PRESSURE");
    for (field, value) in [("structural_status", "numerically_unresolved"), ("solve_quality", "not_assessed"), ("model_matrix_fidelity", "not_assessed"), ("accuracy_evidence", "not_claimed")] {
        case[field] = json!(value);
    }
    case["evidence_refs"] = json!([]);
    assert!(!open_pipe_stress_report_standing::reader_readability::case_records_consistent(&source));
}

/// The standing record never states a health beside an integrity code whose
/// closed record names another (scrutiny of PR 328, finding 3b), even where
/// the case record was altered to the eligible record, which the readability
/// check leaves as the ordinary standing found it.
#[test]
fn a_health_contradicting_its_integrity_code_refuses_the_record() {
    let (mut source, model, standing) = read(GAP_RAW, GAP_REQUEST);
    let case = &mut source["numerical_quality"]["cases"][0];
    for (field, value) in [("structural_status", "passive_model_basis"), ("solve_quality", "checks_passed"), ("model_matrix_fidelity", "represented_equations_retained"), ("accuracy_evidence", "not_claimed")] {
        case[field] = json!(value);
    }
    assert_eq!(historical_standing_record(&source, &model, &standing).unwrap_err(), format!("{NUMERICAL_CASE_RECORD_INCONSISTENT}: load:PRESSURE"));
}

/// Scrutiny of PR 328, finding 5, as Agent 0 settled it: a duplicate is a
/// statement about the document, not about a case. Two rows for one load case
/// and support, or two diagnostics with one id, make the result unreadable by a
/// finding that names the key or id, whatever their order and whether or not
/// they are identical; and a direct caller of the derivation gets the same
/// refusal, never a record.
mod duplicates {
    use super::*;
    use open_pipe_stress_report_standing::reader_readability::{NUMERICAL_DIAGNOSTIC_ID_DUPLICATED, NUMERICAL_SUPPORT_STATE_DUPLICATED};

    const STATE: &str = "nonlinear_support_active_set_state_code";
    const FR1: &str = "support:RS-FR1";

    fn fr1_state(source: &Value) -> (usize, Value) {
        source["results"].as_array().unwrap().iter().enumerate()
            .find(|(_, r)| r["kind"] == STATE && r["entity_ref"] == FR1)
            .map(|(i, r)| (i, r.clone()))
            .expect("RS-FR1's state row")
    }

    fn refused(source: &Value, model: &Value, standing: &Value, finding: &str) {
        let verdict = readability(source, standing);
        assert_eq!(verdict["readable"], false, "{verdict}");
        assert!(verdict["unreadable"].as_array().unwrap().contains(&json!(finding)), "{verdict}");
        assert_eq!(historical_standing_record(source, model, standing).unwrap_err(), finding);
    }

    #[test]
    fn a_support_state_published_twice_refuses_in_either_order_and_when_identical() {
        let (source, model, standing) = read(FRICTION_RAW, FRICTION_REQUEST);
        let (at, row) = fr1_state(&source);
        let case = row["basis_ref"]["ref_id"].as_str().unwrap().to_string();
        let finding = format!("{NUMERICAL_SUPPORT_STATE_DUPLICATED}: {case} {FR1}");
        let mut conflicting = row.clone();
        conflicting["id"] = json!(format!("{}:twin", row["id"].as_str().unwrap()));
        conflicting["value"] = json!(if row["value"] == json!(1.0) { 3.0 } else { 1.0 });
        // The conflicting row after the published one, then before it.
        for before in [false, true] {
            let mut altered = source.clone();
            let rows = altered["results"].as_array_mut().unwrap();
            if before { rows.insert(at, conflicting.clone()) } else { rows.push(conflicting.clone()) }
            refused(&altered, &model, &standing, &finding);
        }
        // An identical copy of the row (same id and value).
        let mut identical = source.clone();
        identical["results"].as_array_mut().unwrap().push(row.clone());
        refused(&identical, &model, &standing, &finding);
    }

    #[test]
    fn a_diagnostic_id_published_twice_refuses_in_either_order_and_when_identical() {
        let (source, model, standing) = read(GAP_RAW, GAP_REQUEST);
        let id = "diagnostic:numerical-integrity:load:PRESSURE";
        let finding = format!("{NUMERICAL_DIAGNOSTIC_ID_DUPLICATED}: {id}");
        let (at, original) = source["diagnostics"].as_array().unwrap().iter().enumerate().find(|(_, d)| d["id"] == id).map(|(i, d)| (i, d.clone())).unwrap();
        let mut conflicting = original.clone();
        conflicting["code"] = json!("NUMERICAL_INTEGRITY_CHECKS_PASSED");
        conflicting["message"] = json!("Invented other message.");
        for before in [false, true] {
            let mut altered = source.clone();
            let diagnostics = altered["diagnostics"].as_array_mut().unwrap();
            if before { diagnostics.insert(at, conflicting.clone()) } else { diagnostics.push(conflicting.clone()) }
            refused(&altered, &model, &standing, &finding);
        }
        let mut identical = source.clone();
        identical["diagnostics"].as_array_mut().unwrap().push(original);
        refused(&identical, &model, &standing, &finding);
    }

    /// Genuinely absent historical evidence is not a duplicate: the gap witness
    /// publishes no confirmation record, and it still reads, with that stated.
    #[test]
    fn absent_historical_evidence_still_reads() {
        let (source, model, standing) = read(GAP_RAW, GAP_REQUEST);
        assert!(open_pipe_stress_report_standing::reader_readability::duplicated_evidence(&source).is_empty());
        assert_eq!(readability(&source, &standing)["readable"], true);
        let record = historical_standing_record(&source, &model, &standing).unwrap();
        assert_eq!(case(&record, "load:PRESSURE")["confirmation"]["outcome"], "no_confirmation_record");
    }
}

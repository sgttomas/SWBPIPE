//! DEL-04-07 *Blocked document* and DEL-05-02 *Standing*, through the desktop's
//! own reader entry (`operation_applier::reader_report_standing_json`, which the
//! desktop reaches in wasm and natively): report readability reads the producer's
//! inspection-only record in a document whose mechanics are not solved as the
//! not-assessed record, never as an inconsistent one; malformed records, and the
//! same code in a solved document, stay inconsistent.
use open_pipe_stress_operation_applier::reader_report_standing_json;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const MODES: [(PreviewSolverMode, &str); 2] = [
    (PreviewSolverMode::SparseInteractive, "sparse_interactive"),
    (PreviewSolverMode::DenseScrutiny, "dense_scrutiny"),
];
const INCONSISTENT: &str = "NUMERICAL_CASE_RECORD_INCONSISTENT";
const INCOMPLETE: &str = "NUMERICAL_CASE_EVIDENCE_INCOMPLETE";
const INSPECTION_ONLY: &str = "NUMERICAL_INTEGRITY_INSPECTION_ONLY";
const PRIOR_PASSED: &str = "Inspection only (prior_outcome=NUMERICAL_INTEGRITY_CHECKS_PASSED): ";

fn demo_model() -> Value {
    serde_json::from_str(include_str!("../../../../fixtures/product_preview/invented_demo_model.json")).unwrap()
}

/// The demo model, and the same model with a further case the producer refuses
/// (a modulus basis that does not exist), which blocks the document after the
/// first case has passed.
fn requests() -> (Value, Value, String) {
    let control = json!({"model": demo_model(), "materials": []});
    let first = control["model"]["load_cases"][0]["id"].as_str().unwrap().to_string();
    let mut blocked = control.clone();
    let mut refused = blocked["model"]["load_cases"][0].clone();
    refused["id"] = json!("load:readability-refused");
    for load in refused["primitive_loads"].as_array_mut().unwrap() {
        let id = format!("{}:refused", load["id"].as_str().unwrap());
        load["id"] = json!(id);
    }
    refused["modulus_basis_ref"] = json!("temperature-point:intentionally-missing");
    blocked["model"]["load_cases"].as_array_mut().unwrap().push(refused);
    (control, blocked, first)
}

fn run(request: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap()
}

fn standing(source: &Value, model: &Value) -> Value {
    serde_json::from_str(&reader_report_standing_json(&source.to_string(), &model.to_string(), None).unwrap()).unwrap()
}

fn unreadable(out: &Value) -> Vec<&str> {
    out["readability"]["unreadable"].as_array().unwrap().iter().filter_map(Value::as_str).collect()
}

fn findings(out: &Value) -> Vec<&str> {
    out["standing"]["findings"].as_array().unwrap().iter().filter_map(Value::as_str).collect()
}

fn case_mut<'a>(source: &'a mut Value, case: &str) -> &'a mut Value {
    source["numerical_quality"]["cases"].as_array_mut().unwrap().iter_mut()
        .find(|c| c["basis_ref"]["ref_id"] == case).unwrap()
}

/// The supported blocked record: a live document whose first case's passed
/// evaluation the producer rewrote as inspection-only, with the not-assessed
/// record and its canonical evidence reference. Readability does not call it
/// inconsistent, by `status` alone (no `analysis_status` list) and as produced.
/// The control solves, passes and stays consistent.
#[test]
fn a_blocked_documents_inspection_only_record_is_consistent() {
    let (control, blocked_request, first) = requests();
    let model = blocked_request["model"].clone();
    let integrity_id = format!("diagnostic:numerical-integrity:{first}");
    for (mode, wire) in MODES {
        let solved = run(&control, mode);
        assert_eq!(solved["status"]["mechanics"], "MECHANICS_SOLVED", "{wire}");
        let out = standing(&solved, &control["model"]);
        assert!(!unreadable(&out).contains(&INCONSISTENT), "{wire}: solved control: {out}");
        assert_eq!(out["readability"]["solved"], true, "{wire}");

        let blocked = run(&blocked_request, mode);
        // The premises: blocked, and the producer's inspection-only record.
        assert_eq!(blocked["status"]["mechanics"], "MODEL_INCOMPLETE", "{wire}");
        assert!(blocked.get("analysis_status").is_none(), "{wire}: status alone says blocked");
        let quality = blocked["numerical_quality"]["cases"].as_array().unwrap().iter()
            .find(|c| c["basis_ref"]["ref_id"] == first.as_str()).unwrap();
        assert_eq!(
            [&quality["structural_status"], &quality["solve_quality"], &quality["model_matrix_fidelity"], &quality["accuracy_evidence"]],
            ["numerically_unresolved", "not_assessed", "not_assessed", "not_claimed"],
            "{wire}"
        );
        assert_eq!(quality["evidence_refs"], json!([integrity_id]), "{wire}");
        let evaluations: Vec<&Value> = blocked["diagnostics"].as_array().unwrap().iter().filter(|d| d["id"] == integrity_id.as_str()).collect();
        assert_eq!(evaluations.len(), 1, "{wire}");
        assert_eq!(evaluations[0]["code"], INSPECTION_ONLY, "{wire}");
        assert!(evaluations[0]["message"].as_str().unwrap().starts_with(PRIOR_PASSED), "{wire}");

        let out = standing(&blocked, &model);
        assert!(findings(&out).contains(&INCOMPLETE), "{wire}: the consistency check runs: {out}");
        assert!(!unreadable(&out).contains(&INCONSISTENT), "{wire}: {out}");
        assert_eq!(out["readability"]["solved"], false, "{wire}");
        assert_eq!(out["standing"]["eligible"], false, "{wire}: never eligible");
        assert!(out["numerical_standing"].is_null(), "{wire}: no record for an unsolved document");
    }
}

/// Malformed records are inconsistent: a changed record field, a wrong or
/// duplicated evidence reference, and a duplicated evaluation. So is the same
/// record in a document that is not positively blocked: solved, or with its
/// mechanics status missing or malformed. Each asserts the diagnosis itself.
#[test]
fn a_malformed_or_solved_inspection_only_record_is_inconsistent() {
    let (_, blocked_request, first) = requests();
    let model = blocked_request["model"].clone();
    let integrity_id = format!("diagnostic:numerical-integrity:{first}");
    for (mode, wire) in MODES {
        let blocked = run(&blocked_request, mode);
        let mut variants: Vec<(&str, Value)> = Vec::new();
        let mut v = blocked.clone();
        case_mut(&mut v, &first)["structural_status"] = json!("passive_model_basis");
        variants.push(("a changed record field", v));
        let mut v = blocked.clone();
        case_mut(&mut v, &first)["model_matrix_fidelity"] = json!("assembly_uncertainty");
        variants.push(("another changed record field", v));
        let mut v = blocked.clone();
        case_mut(&mut v, &first)["evidence_refs"] = json!(["diagnostic:numerical-integrity:load:another"]);
        variants.push(("a wrong evidence reference", v));
        let mut v = blocked.clone();
        case_mut(&mut v, &first)["evidence_refs"] = json!([integrity_id, integrity_id]);
        variants.push(("a duplicated evidence reference", v));
        let mut v = blocked.clone();
        let copy = v["diagnostics"].as_array().unwrap().iter().find(|d| d["id"] == integrity_id.as_str()).unwrap().clone();
        v["diagnostics"].as_array_mut().unwrap().push(copy);
        variants.push(("a duplicated evaluation", v));
        let mut v = blocked.clone();
        v["status"]["mechanics"] = json!("MECHANICS_SOLVED");
        variants.push(("the code in a solved document", v));
        let mut v = blocked.clone();
        v.as_object_mut().unwrap().remove("status");
        variants.push(("no mechanics status", v));
        let mut v = blocked.clone();
        v["status"]["mechanics"] = json!(0);
        variants.push(("a mechanics status of the wrong type", v));
        let mut v = blocked.clone();
        v["status"]["mechanics"] = json!("MECHANICS_UNKNOWN");
        variants.push(("an unknown mechanics status", v));
        for (label, source) in variants {
            let out = standing(&source, &model);
            assert!(unreadable(&out).contains(&INCONSISTENT), "{wire}: {label}: {out}");
        }
    }
}

/// The older blocked forms read as before, independently of the arm: every
/// inspection-only evaluation of the live document is put back in the form
/// main's producer wrote (its original code and message, and the passed or
/// sensitive record), or removed with its evidence (a case with no evaluation,
/// the not-assessed record). Neither carries the inspection-only code.
#[test]
fn older_blocked_records_read_as_before() {
    const TAIL: &str = "evaluation is retained for inspection, not as a verdict on a published result. ";
    let (_, blocked_request, _) = requests();
    let model = blocked_request["model"].clone();
    for (mode, wire) in MODES {
        let blocked = run(&blocked_request, mode);
        let wrapped: Vec<(String, String, String)> = blocked["diagnostics"].as_array().unwrap().iter()
            .filter(|d| d["code"] == INSPECTION_ONLY)
            .map(|d| {
                let message = d["message"].as_str().unwrap();
                let prior = message.strip_prefix("Inspection only (prior_outcome=").unwrap().split(')').next().unwrap().to_string();
                let original = message[message.find(TAIL).unwrap() + TAIL.len()..].to_string();
                (d["id"].as_str().unwrap().to_string(), prior, original)
            })
            .collect();
        assert!(!wrapped.is_empty(), "{wire}: the live document carries inspection-only evaluations");
        let mut old = blocked.clone();
        let mut none = blocked.clone();
        for (id, prior, original) in &wrapped {
            let case = id.strip_prefix("diagnostic:numerical-integrity:").unwrap();
            let d = old["diagnostics"].as_array_mut().unwrap().iter_mut().find(|d| d["id"] == id.as_str()).unwrap();
            d["code"] = json!(prior);
            d["message"] = json!(original);
            let quality = match prior.as_str() {
                "NUMERICAL_INTEGRITY_CHECKS_PASSED" => "checks_passed",
                "NUMERICAL_INTEGRITY_SENSITIVE" => "sensitive",
                other => panic!("{wire}: unexpected prior outcome {other}"),
            };
            let q = case_mut(&mut old, case);
            q["structural_status"] = json!("passive_model_basis");
            q["solve_quality"] = json!(quality);
            q["model_matrix_fidelity"] = json!("represented_equations_retained");
            none["diagnostics"].as_array_mut().unwrap().retain(|d| d["id"] != id.as_str());
            case_mut(&mut none, case)["evidence_refs"] = json!([]);
        }
        for (label, source) in [("old passed form", &old), ("no evaluation", &none)] {
            assert!(!source["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == INSPECTION_ONLY), "{wire}: {label}");
            let out = standing(source, &model);
            assert!(!unreadable(&out).contains(&INCONSISTENT), "{wire}: {label}: {out}");
        }
    }
}

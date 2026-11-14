//! DEL-04-07: a blocked document (status.mechanics MODEL_INCOMPLETE) publishes no
//! result, so no case verdict stands in it. Before this change a blocked document
//! could publish numerical quality checks_passed; the desktop indicator and
//! `numerical_use_standing` would have read it as assessed had any consumer passed
//! it to them (none did). These tests send blocked documents carrying passed
//! verdicts THROUGH the readers, in the old form (old codes, checks_passed quality)
//! and the new one (NUMERICAL_INTEGRITY_INSPECTION_ONLY, not_assessed), and a solved
//! document carrying an inspection-only evaluation, which is refused by name.
use open_pipe_stress_result_export::{reader_verdict as rv, semantic_contract as s};
use serde_json::{json, Value};

fn fixture(name: &str) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/results").join(name);
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// A solved preview-physics-2 result whose two cases passed (nonlinear supports).
fn solved() -> Value {
    fixture("preview_physics_invented_sparse.json")
}

fn bases(raw: &Value) -> Vec<Value> {
    raw["numerical_quality"]["cases"].as_array().unwrap().iter().map(|c| c["basis_ref"].clone()).collect()
}

fn integrity(raw: &Value) -> Vec<Value> {
    raw["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["id"].as_str().unwrap().starts_with("diagnostic:numerical-integrity:"))
        .cloned()
        .collect()
}

/// The sanitized blocked envelope of `preview_physics_contract.rs`, keeping the
/// solved document's integrity diagnostics and numerical quality: an old-form
/// blocked document with passed verdicts.
fn old_blocked() -> Value {
    let solved = solved();
    let mut blocked = solved.clone();
    blocked["status"]["mechanics"] = json!("MODEL_INCOMPLETE");
    blocked["results"] = json!([]);
    blocked["summary"]["max_open_formula_stress"] = Value::Null;
    blocked["summary"]["max_displacement"] = Value::Null;
    blocked["summary"]["component_stress_modifier_count"] = json!(0);
    blocked["contract_evidence"] = json!({"preview_cases": [], "combination_gates": []});
    let mut diagnostics = integrity(&solved);
    diagnostics.push(json!({"id": "diagnostic:preview-physics:joint-equilibrium:13:component:C-1", "code": "JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED", "severity": "blocking", "message": "hand-built blocked control", "source": "core/product_physics", "affected_refs": ["component:C-1", "pipe:P-1"]}));
    blocked["diagnostics"] = json!(diagnostics);
    blocked
}

/// The same document in the new form, as `blocked_envelope` now writes it.
fn new_blocked() -> Value {
    let mut blocked = old_blocked();
    for d in blocked["diagnostics"].as_array_mut().unwrap() {
        let code = d["code"].as_str().unwrap().to_string();
        if matches!(code.as_str(), "NUMERICAL_INTEGRITY_CHECKS_PASSED" | "NUMERICAL_INTEGRITY_SENSITIVE") {
            d["message"] = json!(format!("{}{code}{}{}", s::INSPECTION_ONLY_LEAD, s::INSPECTION_ONLY_TAIL, d["message"].as_str().unwrap()));
            d["code"] = json!(s::INSPECTION_ONLY_CODE);
            d["severity"] = json!("info");
        }
    }
    blocked["numerical_quality"]["status"] = json!("not_assessed");
    for case in blocked["numerical_quality"]["cases"].as_array_mut().unwrap() {
        case["structural_status"] = json!("numerically_unresolved");
        case["solve_quality"] = json!("not_assessed");
        case["model_matrix_fidelity"] = json!("not_assessed");
        case["accuracy_evidence"] = json!("not_claimed");
    }
    blocked
}

fn findings(standing: &Value) -> Vec<String> {
    standing["findings"].as_array().map_or_else(Vec::new, |f| f.iter().filter_map(|x| x.as_str().map(String::from)).collect())
}

#[test]
fn a_blocked_document_with_passed_verdicts_is_never_eligible_old_or_new() {
    for (label, doc) in [("old", old_blocked()), ("new", new_blocked())] {
        // The document is readable, so the mechanics check, not a refusal, decides.
        s::for_source(&doc).unwrap_or_else(|e| panic!("{label}: {e}"));
        assert_eq!(s::numerical_use_standing(&doc, &bases(&doc)), "needs_recompute", "{label}");
        let standing = rv::standing(&doc, None, None);
        assert_ne!(standing["eligible"], true, "{label}: {standing}");
        assert!(findings(&standing).iter().any(|f| f == "MECHANICS_NOT_SOLVED"), "{label}: {standing}");
        assert!(!findings(&standing).iter().any(|f| f == "INTEGRITY_INSPECTION_ONLY_IN_SOLVED_DOCUMENT"), "{label}");
    }
}

#[test]
fn a_solved_document_carrying_an_inspection_only_evaluation_is_refused_by_name() {
    let mut doc = solved();
    assert_eq!(s::numerical_use_standing(&doc, &bases(&doc)), "numerically_eligible", "the control");
    let target = doc["diagnostics"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|d| d["code"] == "NUMERICAL_INTEGRITY_CHECKS_PASSED")
        .unwrap();
    let message = target["message"].as_str().unwrap().to_string();
    target["message"] = json!(format!("{}NUMERICAL_INTEGRITY_CHECKS_PASSED{}{message}", s::INSPECTION_ONLY_LEAD, s::INSPECTION_ONLY_TAIL));
    target["code"] = json!(s::INSPECTION_ONLY_CODE);
    target["severity"] = json!("info");
    assert_eq!(s::numerical_use_standing(&doc, &bases(&doc)), "unsupported");
    let standing = rv::standing(&doc, None, None);
    assert_ne!(standing["eligible"], true, "{standing}");
    assert!(findings(&standing).iter().any(|f| f == "INTEGRITY_INSPECTION_ONLY_IN_SOLVED_DOCUMENT"), "{standing}");
    // The nonlinear reader's own named refusal is shown on solved nonlinear
    // envelopes by tests/test_rt_a3_nonlinear_reader.py (this fixture
    // publishes no nonlinear support rows, so that reader does not run here).
}

#[test]
fn the_prior_outcome_token_is_parsed_strictly() {
    let original = "M03-INTEGRITY-v1 represented original-equation structural evidence for load case a: …";
    for prior in ["NUMERICAL_INTEGRITY_CHECKS_PASSED", "NUMERICAL_INTEGRITY_SENSITIVE"] {
        let message = format!("{}{prior}{}{original}", s::INSPECTION_ONLY_LEAD, s::INSPECTION_ONLY_TAIL);
        assert_eq!(s::inspection_only_prior(&message), Some((prior, original)));
    }
    let lead = s::INSPECTION_ONLY_LEAD;
    let tail = s::INSPECTION_ONLY_TAIL;
    let near_misses = [
        format!("{}NUMERICAL_INTEGRITY_CHECKS_PASSED{tail}{original}", lead.to_lowercase()),
        format!("{lead} NUMERICAL_INTEGRITY_CHECKS_PASSED{tail}{original}"),
        format!("{lead}NUMERICAL_INTEGRITY_CHECKS_PASSED {tail}{original}"),
        format!("{lead}numerical_integrity_checks_passed{tail}{original}"),
        format!("{lead}NUMERICAL_INTEGRITY_FAILED{tail}{original}"),
        format!("{lead}NUMERICAL_INTEGRITY_INSPECTION_ONLY{tail}{original}"),
        format!("{lead}{tail}{original}"),
        format!("{lead}NUMERICAL_INTEGRITY_CHECKS_PASSED{original}"),
        original.to_string(),
        String::new(),
    ];
    for message in &near_misses {
        assert_eq!(s::inspection_only_prior(message), None, "{message}");
    }
}

//! Acceptability-relation runs over the invented example rule pack
//! (`examples/rule_packs/invented_demo.yaml`, DEL-06-05), exercising the
//! additive optional `acceptability_relation` member on a CheckDefinition
//! (TP-C4-ACCEPTREL-001). The member is backward compatible: when absent the
//! runner uses `less_than_or_equal` (proven by the `invented_demo_run` tests).
//! These tests inject an explicit relation into the example check and assert the
//! pass/fail semantics flip accordingly, that an explicit relation reproduces
//! the historical default, and that an unrecognized token blocks the check
//! rather than defaulting silently (no-silent-defaults, CONTRACT).
//!
//! All values are invented non-engineering demonstration content. Nothing here
//! is a professional, certification, sealing, authentication, or code-compliance
//! claim.

use std::fs;
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::AnalysisStatus;
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, RuleCheckRunInput, RuleCheckRunResult, RuleCheckStatus, SolverResultBinding,
    SuppliedValueBinding,
};
use serde_json::Value;

fn example_document() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../examples/rule_packs/invented_demo.yaml");
    let raw = fs::read_to_string(path).expect("example rule pack readable");
    serde_json::from_str(&raw).expect("example rule pack is strict JSON")
}

/// Inject an `acceptability_relation` onto the example pack's single check.
fn with_relation(relation: &str) -> Value {
    let mut document = example_document();
    document["check_definitions"][0]["acceptability_relation"] =
        Value::String(relation.to_string());
    document
}

fn solver(value: f64) -> SolverResultBinding {
    SolverResultBinding {
        input_id: "demo_actual_quantity".to_string(),
        result_id: "result:stress:demo".to_string(),
        value,
        unit: "demo_unit".to_string(),
    }
}

fn limit_supplied() -> Vec<SuppliedValueBinding> {
    vec![
        // The user-supplied rule limit quantity (stress, demo_unit).
        SuppliedValueBinding {
            ref_id: "demo_limit_quantity".to_string(),
            value: 100.0,
            unit: "demo_unit".to_string(),
            dimension: "stress".to_string(),
        },
        // The user-supplied ratio acceptability limit value slot.
        SuppliedValueBinding {
            ref_id: "demo_limit_slot".to_string(),
            value: 1.0,
            unit: "ratio".to_string(),
            dimension: "dimensionless".to_string(),
        },
    ]
}

fn run(document: &Value, solver_value: f64) -> RuleCheckRunResult {
    run_rule_checks(&RuleCheckRunInput {
        rule_pack_document: document,
        solver_results: vec![solver(solver_value)],
        supplied_values: limit_supplied(),
        library_values: vec![],
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    })
}

#[test]
fn greater_than_or_equal_relation_flips_the_acceptance() {
    // The demo formula computes ratio = actual / 100, compared against the ratio
    // limit 1.0. Under `>=`, the `<=` cases invert:
    //  - actual 50 -> ratio 0.5; 0.5 >= 1.0 is false -> FAILED (was the passing case).
    let failed = run(&with_relation("greater_than_or_equal"), 50.0);
    assert_eq!(failed.aggregate_status, RuleCheckStatus::UserRuleFailed);
    let check = &failed.checks[0];
    assert_eq!(check.status, RuleCheckStatus::UserRuleFailed);
    assert_eq!(check.acceptability_relation, "greater_than_or_equal");
    assert_eq!(check.computed_value.as_ref().expect("ratio").value, 0.5);
    assert!(check.evaluator_findings.is_empty());

    //  - actual 150 -> ratio 1.5; 1.5 >= 1.0 is true -> CHECKED (was the failing case).
    let checked = run(&with_relation("greater_than_or_equal"), 150.0);
    assert_eq!(checked.aggregate_status, RuleCheckStatus::UserRuleChecked);
    assert_eq!(checked.checks[0].status, RuleCheckStatus::UserRuleChecked);
    assert_eq!(
        checked.checks[0].acceptability_relation,
        "greater_than_or_equal"
    );
}

#[test]
fn less_than_relation_is_strict_at_the_boundary() {
    // actual 100 -> ratio exactly 1.0. `<= 1.0` passes; the strict `< 1.0` fails.
    // This witnesses that the relation, not just the default, drives the result.
    let result = run(&with_relation("less_than"), 100.0);
    let check = &result.checks[0];
    assert_eq!(check.computed_value.as_ref().expect("ratio").value, 1.0);
    assert_eq!(check.status, RuleCheckStatus::UserRuleFailed);
    assert_eq!(check.acceptability_relation, "less_than");
}

#[test]
fn explicit_less_than_or_equal_matches_the_absent_default() {
    // The explicit token reproduces the historical hard-coded behaviour exactly:
    // ratio 1.0 <= 1.0 passes.
    let result = run(&with_relation("less_than_or_equal"), 100.0);
    let check = &result.checks[0];
    assert_eq!(check.status, RuleCheckStatus::UserRuleChecked);
    assert_eq!(check.acceptability_relation, "less_than_or_equal");
}

#[test]
fn unrecognized_relation_blocks_rather_than_defaulting() {
    // An explicit but unsupported token is never silently coerced to `<=`: the
    // check blocks at RULE_INPUTS_INCOMPLETE with an evaluator finding.
    let result = run(&with_relation("between"), 50.0);
    let check = &result.checks[0];
    assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
    assert_eq!(check.acceptability_relation, "none");
    assert!(check
        .evaluator_findings
        .iter()
        .any(|finding| finding.code == "RULE_EVALUATOR_ERROR"
            && finding
                .message
                .contains("unsupported acceptability_relation")));
}

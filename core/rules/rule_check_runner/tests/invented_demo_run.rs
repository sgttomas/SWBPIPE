//! End-to-end rule-check run over the repository's invented example rule pack
//! (`examples/rule_packs/invented_demo.yaml`, DEL-06-05). The orchestrator
//! resolves a solver value and a user-supplied limit, decodes and evaluates the
//! invented ratio formula through the frozen-grammar evaluator, composes the
//! `<=` acceptability against the user-supplied ratio limit, and emits one of
//! the three automatic rule-check statuses.
//!
//! All values are invented non-engineering demonstration content. Nothing here
//! is a professional, certification, sealing, authentication, or code-compliance
//! claim.

use std::fs;
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::AnalysisStatus;
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, RuleCheckRunInput, RuleCheckStatus, SolverResultBinding, SuppliedValueBinding,
};

fn example_document() -> serde_json::Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../examples/rule_packs/invented_demo.yaml");
    let raw = fs::read_to_string(path).expect("example rule pack readable");
    serde_json::from_str(&raw).expect("example rule pack is strict JSON")
}

fn supplied(ref_id: &str, value: f64, unit: &str, dimension: &str) -> SuppliedValueBinding {
    SuppliedValueBinding {
        ref_id: ref_id.to_string(),
        value,
        unit: unit.to_string(),
        dimension: dimension.to_string(),
    }
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
        supplied("demo_limit_quantity", 100.0, "demo_unit", "stress"),
        // The user-supplied ratio acceptability limit value slot.
        supplied("demo_limit_slot", 1.0, "ratio", "dimensionless"),
    ]
}

#[test]
fn example_pack_passing_run_is_user_rule_checked() {
    let document = example_document();
    let result = run_rule_checks(&RuleCheckRunInput {
        rule_pack_document: &document,
        solver_results: vec![solver(50.0)],
        supplied_values: limit_supplied(),
        library_values: vec![],
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    });

    assert_eq!(result.rule_pack_id, "invented_demo_rule_pack");
    assert_eq!(result.grammar_version, "1.0.0");
    assert_eq!(result.aggregate_status, RuleCheckStatus::UserRuleChecked);

    let check = &result.checks[0];
    assert_eq!(check.check_id, "demo_training_check");
    assert_eq!(check.status, RuleCheckStatus::UserRuleChecked);
    let computed = check.computed_value.as_ref().expect("ratio computed");
    assert_eq!(computed.value, 0.5);
    assert_eq!(computed.dimension, "dimensionless");
    assert_eq!(computed.unit_ref, "ratio");
    assert!(check.evaluator_findings.is_empty());
    assert!(check.completeness_findings.is_empty());
}

#[test]
fn example_pack_failing_run_is_user_rule_failed() {
    let document = example_document();
    let result = run_rule_checks(&RuleCheckRunInput {
        rule_pack_document: &document,
        solver_results: vec![solver(150.0)],
        supplied_values: limit_supplied(),
        library_values: vec![],
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    });

    assert_eq!(result.aggregate_status, RuleCheckStatus::UserRuleFailed);
    let check = &result.checks[0];
    assert_eq!(check.status, RuleCheckStatus::UserRuleFailed);
    assert_eq!(check.computed_value.as_ref().expect("ratio").value, 1.5);
}

#[test]
fn example_pack_missing_solver_input_is_rule_inputs_incomplete() {
    let document = example_document();
    let result = run_rule_checks(&RuleCheckRunInput {
        rule_pack_document: &document,
        // Drop the solver binding for demo_actual_quantity.
        solver_results: vec![],
        supplied_values: limit_supplied(),
        library_values: vec![],
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    });

    assert_eq!(
        result.aggregate_status,
        RuleCheckStatus::RuleInputsIncomplete
    );
    let check = &result.checks[0];
    assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
    assert!(check.computed_value.is_none());
    assert_eq!(check.acceptability_relation, "none");
}

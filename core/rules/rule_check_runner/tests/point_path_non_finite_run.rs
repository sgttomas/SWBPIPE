//! T3-SI1b: rule checks whose point-path evaluation used to panic (an
//! overflowing same-dimension quotient, or a NaN interpolation or step-lookup
//! argument) now block with the evaluator's `NonFiniteInput` finding, and the
//! run carries on to its other checks. A verified bound of exactly zero runs
//! the same point path (T3 D2 §4.11.2) and gives the same bytes.
//!
//! All values are invented non-engineering demonstration content. Nothing here
//! is a professional, certification, sealing, authentication, or code-compliance
//! claim.

use std::fs;
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::AnalysisStatus;
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, run_rule_checks_with_bounds, RuleCheckRunInput, RuleCheckRunResult,
    RuleCheckStatus, SolverResultBinding, SolverResultBound, SuppliedValueBinding,
};
use serde_json::{json, Value};

fn example_document() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../examples/rule_packs/invented_demo.yaml");
    let raw = fs::read_to_string(path).expect("example rule pack readable");
    serde_json::from_str(&raw).expect("example rule pack is strict JSON")
}

/// The example pack plus a second, boolean check (`actual <= limit`) that does
/// not divide, so a blocked first check must not stop the second.
fn two_check_document() -> Value {
    let mut document = example_document();
    document["formula_declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "formula_id": "demo_predicate_expression",
            "declaration_payload": { "expression_ast": {
                "node": "compare", "operator": "less_than_or_equal",
                "left": { "node": "variable_ref", "variable_id": "demo_actual_quantity" },
                "right": { "node": "variable_ref", "variable_id": "demo_limit_quantity" } } },
            "input_refs": [
                { "ref_id": "demo_actual_quantity", "ref_type": "required_input" },
                { "ref_id": "demo_limit_quantity", "ref_type": "required_input" }
            ]
        }));
    let mut second = document["check_definitions"][0].clone();
    second["check_id"] = json!("demo_predicate_check");
    second["formula_ref"]["ref_id"] = json!("demo_predicate_expression");
    second.as_object_mut().unwrap().remove("value_slot_refs");
    document["check_definitions"]
        .as_array_mut()
        .unwrap()
        .push(second);
    document
}

/// A pack whose formula is a table over the ratio argument
/// `actual*1e300 - actual*1e300`, which is NaN once `actual*1e300` overflows.
fn table_document(kind: &str) -> Value {
    let product = json!({ "node": "binary", "operator": "multiply",
        "left": { "node": "variable_ref", "variable_id": "actual" },
        "right": { "node": "literal",
                   "quantity": { "value": 1e300, "dimension": "dimensionless", "unit_ref": "ratio" } } });
    let argument = json!({ "node": "binary", "operator": "subtract", "left": product.clone(), "right": product });
    let table = json!({ "table_id": "invented_table", "argument_dimension": "dimensionless",
        "argument_unit_ref": "ratio", "result_dimension": "stress", "result_unit_ref": "demo_unit",
        "rows": [ { "argument": 0.0, "result": 1.0 }, { "argument": 1.0, "result": 2.0 },
                  { "argument": 2.0, "result": 3.0 } ] });
    let ast = match kind {
        "interpolate" => json!({ "node": "interpolate", "table": table, "argument": argument }),
        mode => json!({ "node": "lookup", "mode": mode, "table": table, "argument": argument }),
    };
    json!({
        "grammar_version": "1.0.0",
        "metadata": { "rule_pack_id": "invented_table_pack" },
        "required_inputs": [
            { "input_id": "actual", "name": "actual", "source_kind": "solver_result",
              "required_for": "rule_check", "provenance_required": true,
              "redistribution_status_required": true,
              "quantity_intent": { "dimension": "dimensionless", "unit_ref": "ratio",
                                   "unit_required": true, "dimension_check_required": true } }
        ],
        "formula_declarations": [{
            "formula_id": "table_formula",
            "declaration_payload": { "expression_ast": ast },
            "input_refs": [ { "ref_id": "actual", "ref_type": "required_input" } ]
        }],
        "value_slots": [{ "slot_id": "stress_limit", "slot_kind": "limit",
            "quantity_intent": { "dimension": "stress", "unit_ref": "demo_unit",
                                 "unit_required": true, "dimension_check_required": true } }],
        "check_definitions": [{
            "check_id": "table_check",
            "required_input_refs": [ { "ref_id": "actual", "ref_type": "required_input" } ],
            "value_slot_refs": [ { "ref_id": "stress_limit", "ref_type": "value_slot" } ],
            "formula_ref": { "ref_id": "table_formula", "ref_type": "formula" },
            "result_statuses": [ "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED" ],
            "diagnostic_policy": { "missing_input": "RULE_INPUT_MISSING",
                                   "evaluator_error": "RULE_EVALUATOR_ERROR" }
        }]
    })
}

fn supplied(ref_id: &str, value: f64, unit: &str, dimension: &str) -> SuppliedValueBinding {
    SuppliedValueBinding {
        ref_id: ref_id.to_string(),
        value,
        unit: unit.to_string(),
        dimension: dimension.to_string(),
    }
}

fn solver(input_id: &str, value: f64, unit: &str) -> SolverResultBinding {
    SolverResultBinding {
        input_id: input_id.to_string(),
        result_id: "result:invented".to_string(),
        value,
        unit: unit.to_string(),
    }
}

/// `run_rule_checks`, checked against `run_rule_checks_with_bounds` with a zero
/// bound on `bound_input` (the exact point): the two must serialize identically.
fn run_point_and_zero_bound(input: &RuleCheckRunInput, bound_input: &str) -> RuleCheckRunResult {
    let plain = run_rule_checks(input);
    let zero_bound = run_rule_checks_with_bounds(
        input,
        &[SolverResultBound {
            input_id: bound_input.to_string(),
            absolute_bound: 0.0,
        }],
    );
    assert_eq!(
        serde_json::to_string(&plain).unwrap(),
        serde_json::to_string(&zero_bound).unwrap()
    );
    plain
}

fn evaluator_records(result: &RuleCheckRunResult, check: usize) -> Vec<(String, String, String)> {
    result.checks[check]
        .evaluator_findings
        .iter()
        .map(|f| (f.code.clone(), f.severity.clone(), f.subject_id.clone()))
        .collect()
}

#[test]
fn an_overflowing_ratio_check_blocks_and_the_run_carries_on() {
    let document = two_check_document();
    let result = run_point_and_zero_bound(
        &RuleCheckRunInput {
            rule_pack_document: &document,
            solver_results: vec![solver("demo_actual_quantity", 1.0e308, "demo_unit")],
            refused_solver_results: Vec::new(),
            supplied_values: vec![
                supplied("demo_limit_quantity", 1.0e-308, "demo_unit", "stress"),
                supplied("demo_limit_slot", 1.0, "ratio", "dimensionless"),
            ],
            library_values: Vec::new(),
            current_statuses: vec![AnalysisStatus::MechanicsSolved],
        },
        "demo_actual_quantity",
    );

    assert_eq!(result.checks.len(), 2);
    let ratio = &result.checks[0];
    assert_eq!(ratio.check_id, "demo_training_check");
    assert_eq!(ratio.status, RuleCheckStatus::RuleInputsIncomplete);
    assert_eq!(
        evaluator_records(&result, 0),
        vec![(
            "NonFiniteInput".to_string(),
            "blocking".to_string(),
            "divide".to_string()
        )]
    );
    assert_eq!(
        ratio.evaluator_findings[0].message,
        "same-dimension quotient (ratio) must be finite"
    );
    assert!(ratio.computed_value.is_none());
    assert_eq!(ratio.acceptability_relation, "none");
    assert_eq!(ratio.diagnostic_codes, vec!["RULE_EVALUATOR_ERROR"]);

    // The second check is still evaluated: 1e308 <= 1e-308 fails.
    let predicate = &result.checks[1];
    assert_eq!(predicate.check_id, "demo_predicate_check");
    assert_eq!(predicate.status, RuleCheckStatus::UserRuleFailed);
    assert!(predicate.evaluator_findings.is_empty());
    assert_eq!(result.aggregate_status, RuleCheckStatus::UserRuleFailed);
}

#[test]
fn a_nan_table_argument_check_blocks() {
    for (kind, code) in [
        ("interpolate", "NonFiniteInput"),
        ("step", "NonFiniteInput"),
        // An exact lookup already blocked, and still does, unchanged.
        ("exact", "TableKeyNotFound"),
    ] {
        let document = table_document(kind);
        let result = run_point_and_zero_bound(
            &RuleCheckRunInput {
                rule_pack_document: &document,
                solver_results: vec![solver("actual", 1.0e10, "ratio")],
                refused_solver_results: Vec::new(),
                supplied_values: vec![supplied("stress_limit", 100.0, "demo_unit", "stress")],
                library_values: Vec::new(),
                current_statuses: vec![AnalysisStatus::MechanicsSolved],
            },
            "actual",
        );
        assert_eq!(
            result.aggregate_status,
            RuleCheckStatus::RuleInputsIncomplete,
            "{kind}"
        );
        assert_eq!(
            evaluator_records(&result, 0),
            vec![(
                code.to_string(),
                "blocking".to_string(),
                "invented_table".to_string()
            )],
            "{kind}"
        );
        assert_eq!(
            result.checks[0].diagnostic_codes,
            vec!["RULE_EVALUATOR_ERROR"]
        );
        assert!(result.checks[0].computed_value.is_none());
    }
}

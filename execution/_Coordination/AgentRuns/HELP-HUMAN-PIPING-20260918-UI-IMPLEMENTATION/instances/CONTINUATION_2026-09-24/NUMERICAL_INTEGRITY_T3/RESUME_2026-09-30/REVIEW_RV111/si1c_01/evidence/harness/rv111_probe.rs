//! RV111 (T3-SI1c review): a small runner probe for two N-4 details the committed tests do not
//! pin: a formula that lists a non-finite input twice (one finding per input), and a non-finite
//! raw value whose entered unit equals the declared unit only after trimming. Scratch only.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::AnalysisStatus;
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, RuleCheckRunInput, SolverResultBinding, SuppliedValueBinding,
};
use serde_json::{json, Value};

fn input(id: &str, source: &str) -> Value {
    json!({ "input_id": id, "name": id, "source_kind": source, "required_for": "rule_check",
            "provenance_required": true, "redistribution_status_required": true,
            "quantity_intent": { "dimension": "stress", "unit_ref": "Pa",
                                 "unit_required": true, "dimension_check_required": true } })
}
fn reference(id: &str) -> Value {
    json!({ "ref_id": id, "ref_type": "required_input" })
}
fn doc(formula_inputs: &[&str], check_inputs: &[&str]) -> Value {
    json!({
        "grammar_version": "1.0.0",
        "metadata": { "rule_pack_id": "rv111_probe" },
        "required_inputs": [ input("x", "solver_result"), input("s", "user_supplied_rule_value"),
                             input("u", "user_supplied_rule_value") ],
        "formula_declarations": [{ "formula_id": "f",
            "declaration_payload": { "expression_ast": { "node": "compare", "operator": "less_than_or_equal",
                "left": { "node": "variable_ref", "variable_id": "x" },
                "right": { "node": "variable_ref", "variable_id": "s" } } },
            "input_refs": formula_inputs.iter().map(|i| reference(i)).collect::<Vec<_>>() }],
        "check_definitions": [{ "check_id": "c",
            "required_input_refs": check_inputs.iter().map(|i| reference(i)).collect::<Vec<_>>(),
            "formula_ref": { "ref_id": "f", "ref_type": "formula" },
            "result_statuses": [ "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED" ],
            "diagnostic_policy": { "missing_input": "RULE_INPUT_MISSING", "evaluator_error": "RULE_EVALUATOR_ERROR" } }]
    })
}
fn run(document: &Value, x: (f64, &str), u: (f64, &str)) -> String {
    let result = run_rule_checks(&RuleCheckRunInput {
        rule_pack_document: document,
        solver_results: vec![SolverResultBinding { input_id: "x".into(), result_id: "r".into(), value: x.0, unit: x.1.into() }],
        refused_solver_results: Vec::new(),
        supplied_values: vec![
            SuppliedValueBinding { ref_id: "s".into(), value: 3.0, unit: "Pa".into(), dimension: "stress".into() },
            SuppliedValueBinding { ref_id: "u".into(), value: u.0, unit: u.1.into(), dimension: "stress".into() },
        ],
        library_values: Vec::new(),
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    });
    serde_json::to_string(&result).unwrap()
}

#[test]
fn rv111_probe() {
    let out = PathBuf::from(std::env::var("RV111_OUT").expect("RV111_OUT")).join("probe.tsv");
    let mut f = File::create(out).unwrap();
    let cases: Vec<(&str, Value, (f64, &str), (f64, &str))> = vec![
        ("x_listed_twice_nan", doc(&["x", "x", "s"], &["x", "s"]), (f64::NAN, "Pa"), (4.0, "Pa")),
        ("x_listed_twice_finite", doc(&["x", "x", "s"], &["x", "s"]), (1.0, "Pa"), (4.0, "Pa")),
        ("x_nan_padded_unit_referenced", doc(&["x", "s"], &["x", "s"]), (f64::NAN, " Pa "), (4.0, "Pa")),
        ("u_nan_padded_unit_unreferenced", doc(&["x", "s"], &["x", "s", "u"]), (1.0, "Pa"), (f64::NAN, " Pa ")),
        ("u_nan_same_unit_unreferenced", doc(&["x", "s"], &["x", "s", "u"]), (1.0, "Pa"), (f64::NAN, "Pa")),
    ];
    for (name, document, x, u) in cases {
        writeln!(f, "{name}\t{}", run(&document, x, u)).unwrap();
    }
}

//! T3-SI1b: rule checks whose point-path evaluation used to panic (an
//! overflowing same-dimension quotient, or a NaN interpolation or step-lookup
//! argument) now block with the evaluator's `NonFiniteInput` finding, and the
//! run carries on to its other checks. A verified bound of exactly zero runs
//! the same point path (T3 D2 §4.11.2) and gives the same bytes.
//!
//! T3-SI1c: every arithmetic and interpolation result must be finite (option
//! D), so a check whose formula overflows blocks at the operation that
//! overflowed, whatever consumes it, with no computed value. A supplied input
//! or limit that is not finite, raw or after unit normalization, is named by
//! its own `NonFiniteInput` finding and never serialized as a JSON `null`
//! (N-4); no status changes.
//!
//! All values are invented non-engineering demonstration content. Nothing here
//! is a professional, certification, sealing, authentication, or code-compliance
//! claim.

use std::fs;
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::AnalysisStatus;
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, run_rule_checks_with_bounds, LibraryValueBinding, RuleCheckRunInput,
    RuleCheckRunResult, RuleCheckStatus, SolverResultBinding, SolverResultBound,
    SuppliedValueBinding,
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
fn a_table_check_over_an_overflowing_argument_blocks_at_the_multiply() {
    // Since T3-SI1c the NaN argument is never formed: `actual*1e300`
    // overflows and blocks at the multiply, for every lookup mode (an exact
    // lookup used to read `TableKeyNotFound`).
    for kind in ["interpolate", "step", "exact"] {
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
                "NonFiniteInput".to_string(),
                "blocking".to_string(),
                "multiply".to_string()
            )],
            "{kind}"
        );
        assert_eq!(
            result.checks[0].evaluator_findings[0].message,
            "product must be finite (it overflowed)"
        );
        assert_eq!(
            result.checks[0].diagnostic_codes,
            vec!["RULE_EVALUATOR_ERROR"]
        );
        assert!(result.checks[0].computed_value.is_none());
    }
}

// -- T3-SI1c ------------------------------------------------------------------

fn variable(id: &str) -> Value {
    json!({ "node": "variable_ref", "variable_id": id })
}

fn ratio(value: f64) -> Value {
    json!({ "node": "literal",
            "quantity": { "value": value, "dimension": "dimensionless", "unit_ref": "ratio" } })
}

fn stress(value: f64) -> Value {
    json!({ "node": "literal",
            "quantity": { "value": value, "dimension": "stress", "unit_ref": "Pa" } })
}

fn node(kind: &str, operator: &str, left: Value, right: Value) -> Value {
    json!({ "node": kind, "operator": operator, "left": left, "right": right })
}

/// `x * 1e300`: +inf once `x` is 1e300 (I79's reproducer).
fn overflowing_x() -> Value {
    node("binary", "multiply", variable("x"), ratio(1.0e300))
}

/// One check over the solver input `x` and the user input `s` (both stress in
/// Pa), with `formula` as its formula. A quantity formula is compared with the
/// stress limit slot `limit` (Pa). `unreferenced` adds the user input `u` to
/// the check's required inputs, but not to the formula's.
fn si1c_document(formula: Value, quantity: bool, unreferenced: bool) -> Value {
    let input = |id: &str, source: &str| {
        json!({ "input_id": id, "name": id, "source_kind": source,
                "required_for": "rule_check", "provenance_required": true,
                "redistribution_status_required": true,
                "quantity_intent": { "dimension": "stress", "unit_ref": "Pa",
                                     "unit_required": true, "dimension_check_required": true } })
    };
    let reference = |id: &str| json!({ "ref_id": id, "ref_type": "required_input" });
    let mut required = vec![reference("x"), reference("s")];
    if unreferenced {
        required.push(reference("u"));
    }
    let mut check = json!({
        "check_id": "si1c_check",
        "required_input_refs": required,
        "formula_ref": { "ref_id": "si1c_formula", "ref_type": "formula" },
        "result_statuses": [ "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED" ],
        "diagnostic_policy": { "missing_input": "RULE_INPUT_MISSING",
                               "evaluator_error": "RULE_EVALUATOR_ERROR" }
    });
    if quantity {
        check["value_slot_refs"] = json!([{ "ref_id": "limit", "ref_type": "value_slot" }]);
    }
    json!({
        "grammar_version": "1.0.0",
        "metadata": { "rule_pack_id": "invented_si1c_pack" },
        "required_inputs": [ input("x", "solver_result"), input("s", "user_supplied_rule_value"),
                             input("u", "user_supplied_rule_value") ],
        "formula_declarations": [{
            "formula_id": "si1c_formula",
            "declaration_payload": { "expression_ast": formula },
            "input_refs": [ reference("x"), reference("s") ]
        }],
        "value_slots": [{ "slot_id": "limit", "slot_kind": "limit",
            "quantity_intent": { "dimension": "stress", "unit_ref": "Pa",
                                 "unit_required": true, "dimension_check_required": true } }],
        "check_definitions": [check]
    })
}

/// Values for `si1c_document`: `x` and `s` (and `u`, and the limit) with
/// their entered units.
struct Values<'a> {
    x: (f64, &'a str),
    s: (f64, &'a str),
    u: (f64, &'a str),
    limit: (f64, &'a str),
}

impl Default for Values<'_> {
    fn default() -> Self {
        Values {
            x: (1.0e300, "Pa"),
            s: (2.0, "Pa"),
            u: (3.0, "Pa"),
            limit: (100.0, "Pa"),
        }
    }
}

fn si1c_run(document: &Value, values: &Values, bound: Option<f64>) -> RuleCheckRunResult {
    let input = RuleCheckRunInput {
        rule_pack_document: document,
        solver_results: vec![solver("x", values.x.0, values.x.1)],
        refused_solver_results: Vec::new(),
        supplied_values: vec![
            supplied("s", values.s.0, values.s.1, "stress"),
            supplied("u", values.u.0, values.u.1, "stress"),
            supplied("limit", values.limit.0, values.limit.1, "stress"),
        ],
        library_values: Vec::new(),
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    let result = match bound {
        None => run_point_and_zero_bound(&input, "x"),
        Some(b) => run_rule_checks_with_bounds(
            &input,
            &[SolverResultBound {
                input_id: "x".to_string(),
                absolute_bound: b,
            }],
        ),
    };
    assert_no_null(&serde_json::to_value(&result).unwrap());
    result
}

/// No serialized outcome carries a JSON `null` (the schema has none).
fn assert_no_null(value: &Value) {
    match value {
        Value::Null => panic!("a serialized outcome carries null"),
        Value::Array(items) => items.iter().for_each(assert_no_null),
        Value::Object(members) => members.values().for_each(assert_no_null),
        _ => {}
    }
}

fn findings(result: &RuleCheckRunResult) -> Vec<(String, String, String)> {
    result.checks[0]
        .evaluator_findings
        .iter()
        .map(|f| (f.code.clone(), f.subject_id.clone(), f.message.clone()))
        .collect()
}

fn record(code: &str, subject: &str, message: &str) -> (String, String, String) {
    (code.to_string(), subject.to_string(), message.to_string())
}

const PRODUCT_OVERFLOWED: &str = "product must be finite (it overflowed)";
const INPUT_NOT_FINITE: &str =
    "supplied value must be finite (NaN or ±inf after unit normalization)";
const LIMIT_NOT_FINITE: &str =
    "value-slot limit must be finite (NaN or ±inf after unit normalization)";
const NOT_BOUND_NOTE: &str = "non-finite value (NaN or ±inf, after unit normalization): not bound";

#[test]
fn a_boolean_check_over_an_overflow_blocks_at_the_producer() {
    // I79's reproducer `not((x*1e300 - x*1e300) > 100)` read USER_RULE_CHECKED
    // before SI1c. Now it blocks at the multiply, plain and at b = 0.
    let formula = json!({ "node": "unary", "operator": "not", "operand": node("compare", "greater_than",
        node("binary", "subtract", overflowing_x(), overflowing_x()), stress(100.0)) });
    let document = si1c_document(formula, false, false);
    let result = si1c_run(&document, &Values::default(), None);
    let check = &result.checks[0];
    assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
    assert_eq!(
        findings(&result),
        vec![record("NonFiniteInput", "multiply", PRODUCT_OVERFLOWED)]
    );
    assert_eq!(check.acceptability_relation, "none");
    assert_eq!(check.diagnostic_codes, vec!["RULE_EVALUATOR_ERROR"]);
    assert_eq!(
        result.aggregate_status,
        RuleCheckStatus::RuleInputsIncomplete
    );

    // The same formula over a finite x is decided, as before.
    let finite = Values {
        x: (1.0, "Pa"),
        ..Values::default()
    };
    let decided = si1c_run(&document, &finite, None);
    assert_eq!(decided.checks[0].status, RuleCheckStatus::UserRuleChecked);
    assert!(decided.checks[0].evaluator_findings.is_empty());
}

#[test]
fn a_quantity_check_over_an_overflow_blocks_with_no_computed_value() {
    // `max(x*1e300 - x*1e300, s)` read USER_RULE_CHECKED (computed = s) before
    // SI1c, and `x*1e300` blocked at the synthesized comparison's literal with
    // a `null` computed value. Both now block at the multiply.
    let absorbed = json!({ "node": "aggregate", "function": "max",
        "operands": [ node("binary", "subtract", overflowing_x(), overflowing_x()), variable("s") ] });
    for formula in [absorbed, overflowing_x()] {
        let document = si1c_document(formula, true, false);
        let result = si1c_run(&document, &Values::default(), None);
        let check = &result.checks[0];
        assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
        assert_eq!(
            findings(&result),
            vec![record("NonFiniteInput", "multiply", PRODUCT_OVERFLOWED)]
        );
        assert!(check.computed_value.is_none());
        assert!(check.limit_value.is_none());
        assert_eq!(check.acceptability_relation, "none");
        assert_eq!(check.diagnostic_codes, vec!["RULE_EVALUATOR_ERROR"]);
    }
}

#[test]
fn a_non_finite_input_is_named_and_never_bound() {
    // N-4: 1e300 GPa is +inf Pa after normalization; NaN and +inf arrive
    // through the Rust API in the declared unit. Each input is supplied, has
    // no value and the note, and one finding names it; the status and the
    // diagnostic are those of any evaluator block, as before.
    let formula = node(
        "compare",
        "less_than_or_equal",
        variable("x"),
        variable("s"),
    );
    let document = si1c_document(formula, false, false);
    for (x, s, input_id) in [
        ((1.0e300, "GPa"), (2.0, "Pa"), "x"),
        ((f64::NAN, "Pa"), (2.0, "Pa"), "x"),
        ((1.0, "Pa"), (f64::INFINITY, "Pa"), "s"),
    ] {
        let values = Values {
            x,
            s,
            ..Values::default()
        };
        for bound in [None, Some(0.5)] {
            let result = si1c_run(&document, &values, bound);
            let check = &result.checks[0];
            assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
            assert_eq!(
                findings(&result),
                vec![record("NonFiniteInput", input_id, INPUT_NOT_FINITE)],
                "{input_id} {bound:?}"
            );
            assert!(check.completeness_findings.is_empty());
            assert_eq!(check.diagnostic_codes, vec!["RULE_EVALUATOR_ERROR"]);
            assert_eq!(check.acceptability_relation, "none");
            let bound_input = check
                .bound_inputs
                .iter()
                .find(|b| b.input_id == input_id)
                .unwrap();
            assert!(bound_input.supplied);
            assert_eq!(bound_input.value, None);
            assert_eq!(bound_input.unit.as_deref(), Some("Pa"));
            // A bounded solver input keeps its interval note, then N-4's.
            let note = match (input_id, bound) {
                ("x", Some(_)) => format!("interval ±5e-1 from receipt; {NOT_BOUND_NOTE}"),
                _ => NOT_BOUND_NOTE.to_string(),
            };
            assert_eq!(bound_input.note.as_deref(), Some(note.as_str()));
        }
    }
}

#[test]
fn a_non_finite_raw_input_in_another_unit_is_named_and_still_unsupplied() {
    // A raw NaN entered in kPa used to read UnitMismatch ("quantity must be
    // finite"). It is now tested before normalization and named; it stays
    // unsupplied, so completeness blocks the check as before.
    let formula = node(
        "compare",
        "less_than_or_equal",
        variable("x"),
        variable("s"),
    );
    let document = si1c_document(formula, false, false);
    let values = Values {
        x: (f64::NAN, "kPa"),
        ..Values::default()
    };
    let result = si1c_run(&document, &values, None);
    let check = &result.checks[0];
    assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
    assert_eq!(
        findings(&result),
        vec![record("NonFiniteInput", "x", INPUT_NOT_FINITE)]
    );
    assert_eq!(check.completeness_findings.len(), 1);
    assert_eq!(check.completeness_findings[0].subject_id, "x");
    assert_eq!(check.diagnostic_codes, vec!["RULE_INPUT_MISSING"]);
    let x = &check.bound_inputs[0];
    assert!(!x.supplied);
    assert_eq!(x.value, None);
    assert_eq!(x.note.as_deref(), Some(NOT_BOUND_NOTE));
}

#[test]
fn an_unreferenced_non_finite_input_blocks_nothing() {
    // `u` is a required input of the check but not of its formula: as before,
    // its value blocks nothing, and the check is decided on `x` and `s`. It
    // carries the note and no value.
    let formula = node(
        "compare",
        "less_than_or_equal",
        variable("x"),
        variable("s"),
    );
    let document = si1c_document(formula, false, true);
    for u in [(f64::NAN, "Pa"), (1.0e300, "GPa")] {
        let values = Values {
            x: (1.0, "Pa"),
            u,
            ..Values::default()
        };
        let result = si1c_run(&document, &values, None);
        let check = &result.checks[0];
        assert_eq!(check.status, RuleCheckStatus::UserRuleChecked);
        assert!(check.evaluator_findings.is_empty());
        let u = check
            .bound_inputs
            .iter()
            .find(|b| b.input_id == "u")
            .unwrap();
        assert!(u.supplied);
        assert_eq!(u.value, None);
        assert_eq!(u.note.as_deref(), Some(NOT_BOUND_NOTE));
    }
}

#[test]
fn a_non_finite_limit_is_named_in_both_limit_blocks() {
    // N-4: a limit of 1e300 GPa (+inf Pa), NaN, or NaN in kPa used to read
    // "missing or unknown unit/dimension metadata" (or a unit mismatch). It
    // now names the slot, on the point path and in interval mode (b = 0.5).
    let document = si1c_document(variable("s"), true, false);
    for limit in [(1.0e300, "GPa"), (f64::NAN, "Pa"), (f64::NAN, "kPa")] {
        let values = Values {
            x: (1.0, "Pa"),
            limit,
            ..Values::default()
        };
        for bound in [None, Some(0.5)] {
            let result = si1c_run(&document, &values, bound);
            let check = &result.checks[0];
            assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
            assert_eq!(
                findings(&result),
                vec![record("NonFiniteInput", "limit", LIMIT_NOT_FINITE)],
                "{limit:?} {bound:?}"
            );
            assert_eq!(check.diagnostic_codes, vec!["RULE_EVALUATOR_ERROR"]);
            assert_eq!(check.acceptability_relation, "less_than_or_equal");
            assert!(check.limit_value.is_none());
        }
    }
}

/// `document` with its formula's `input_refs` replaced by `ids`.
fn with_formula_inputs(mut document: Value, ids: &[&str]) -> Value {
    document["formula_declarations"][0]["input_refs"] = Value::Array(
        ids.iter()
            .map(|id| json!({ "ref_id": id, "ref_type": "required_input" }))
            .collect(),
    );
    document
}

#[test]
fn a_non_finite_input_listed_twice_is_named_once() {
    // RV111 SF-1 (a), its probe `x_listed_twice_nan`: the formula's
    // input_refs list `x` twice. One finding names it, once.
    let formula = node(
        "compare",
        "less_than_or_equal",
        variable("x"),
        variable("s"),
    );
    let document = with_formula_inputs(si1c_document(formula, false, false), &["x", "x", "s"]);
    let values = Values {
        x: (f64::NAN, "Pa"),
        ..Values::default()
    };
    for bound in [None, Some(0.5)] {
        let result = si1c_run(&document, &values, bound);
        let check = &result.checks[0];
        assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
        assert_eq!(
            findings(&result),
            vec![record("NonFiniteInput", "x", INPUT_NOT_FINITE)],
            "{bound:?}"
        );
        assert_eq!(check.diagnostic_codes, vec!["RULE_EVALUATOR_ERROR"]);
    }
}

#[test]
fn a_non_finite_value_in_a_padded_copy_of_the_declared_unit_is_named_not_unsupplied() {
    // RV111 SF-1 (b), its probes `x_nan_padded_unit_referenced` and
    // `u_nan_padded_unit_unreferenced`: " Pa " is the declared "Pa" once
    // trimmed, as normalization reads it, so the NaN is not a value in
    // another unit. It is supplied, with the note.
    let formula = node(
        "compare",
        "less_than_or_equal",
        variable("x"),
        variable("s"),
    );

    // Referenced: one finding and the evaluator-error diagnostic.
    let referenced = Values {
        x: (f64::NAN, " Pa "),
        ..Values::default()
    };
    let result = si1c_run(
        &si1c_document(formula.clone(), false, false),
        &referenced,
        None,
    );
    let check = &result.checks[0];
    assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
    assert_eq!(
        findings(&result),
        vec![record("NonFiniteInput", "x", INPUT_NOT_FINITE)]
    );
    assert!(check.completeness_findings.is_empty());
    assert_eq!(check.diagnostic_codes, vec!["RULE_EVALUATOR_ERROR"]);
    let x = &check.bound_inputs[0];
    assert!(x.supplied);
    assert_eq!(x.value, None);
    assert_eq!(x.unit.as_deref(), Some("Pa"));
    assert_eq!(x.note.as_deref(), Some(NOT_BOUND_NOTE));

    // Unreferenced: it blocks nothing, and the check is decided on x and s.
    let unreferenced = Values {
        x: (1.0, "Pa"),
        u: (f64::NAN, " Pa "),
        ..Values::default()
    };
    let result = si1c_run(&si1c_document(formula, false, true), &unreferenced, None);
    let check = &result.checks[0];
    assert_eq!(check.status, RuleCheckStatus::UserRuleChecked);
    assert!(check.evaluator_findings.is_empty());
    assert!(check.completeness_findings.is_empty());
    let u = check
        .bound_inputs
        .iter()
        .find(|b| b.input_id == "u")
        .unwrap();
    assert!(u.supplied);
    assert_eq!(u.value, None);
    assert_eq!(u.note.as_deref(), Some(NOT_BOUND_NOTE));
}

#[test]
fn the_n4_note_follows_an_existing_note() {
    // Ruling 2 (RV111 N-1): N-4's note is appended, after "; ", to a note the
    // input already carries, which says where the value came from.
    let formula = node(
        "compare",
        "less_than_or_equal",
        variable("x"),
        variable("s"),
    );

    // A bounded solver input carries its interval note.
    let document = si1c_document(formula.clone(), false, false);
    let values = Values {
        x: (f64::INFINITY, "Pa"),
        ..Values::default()
    };
    let result = si1c_run(&document, &values, Some(2.0));
    let x = &result.checks[0].bound_inputs[0];
    assert_eq!(
        x.note.as_deref(),
        Some(format!("interval ±2e0 from receipt; {NOT_BOUND_NOTE}").as_str())
    );
    assert_eq!(
        findings(&result),
        vec![record("NonFiniteInput", "x", INPUT_NOT_FINITE)]
    );

    // A private library input carries its provenance.
    let mut document = si1c_document(formula, false, false);
    document["required_inputs"][1]["source_kind"] = json!("private_library_value");
    let input = RuleCheckRunInput {
        rule_pack_document: &document,
        solver_results: vec![solver("x", 1.0, "Pa")],
        refused_solver_results: Vec::new(),
        supplied_values: Vec::new(),
        library_values: vec![LibraryValueBinding {
            input_id: "s".to_string(),
            value: f64::NAN,
            unit: "Pa".to_string(),
            library_kind: "invented_kind".to_string(),
            library_id: "invented_library".to_string(),
            record_id: "invented_record".to_string(),
            slot_id: "invented_slot".to_string(),
        }],
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    let result = run_point_and_zero_bound(&input, "x");
    assert_no_null(&serde_json::to_value(&result).unwrap());
    let check = &result.checks[0];
    assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
    assert_eq!(
        findings(&result),
        vec![record("NonFiniteInput", "s", INPUT_NOT_FINITE)]
    );
    let s = &check.bound_inputs[1];
    assert!(s.supplied);
    assert_eq!(s.value, None);
    assert_eq!(
        s.note.as_deref(),
        Some(
            format!(
                "resolved from private library invented_kind:invented_library record \
                 invented_record slot invented_slot (value stays in the private library; never \
                 embedded in the rule pack); {NOT_BOUND_NOTE}"
            )
            .as_str()
        )
    );
}

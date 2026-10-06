//! Interval binding through the runner (T3 D2 §4.11, option C): verified
//! absolute bounds on solver results, the three-valued outcome with its codes,
//! outward unit normalization, and the unchanged point path when no interval
//! input is bound. All packs and values are invented demonstration content;
//! nothing here is a professional or code-compliance claim.

use open_pipe_stress_expression_evaluator::AnalysisStatus;
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, run_rule_checks_with_bounds, CheckOutcome, RuleCheckRunInput,
    RuleCheckRunResult, RuleCheckStatus, SolverResultBinding, SolverResultBound,
    SuppliedValueBinding, RULE_INTERVAL_ALL_FAIL, RULE_INTERVAL_ALL_PASS,
    RULE_INTERVAL_DIVIDE_BY_ZERO_RANGE, RULE_RESULT_INDETERMINATE,
};
use open_pipe_stress_units::{convert_for_dimension, unit_by_symbol, Dimension as UnitDimension};
use serde_json::{json, Value};

fn var(id: &str) -> Value {
    json!({ "node": "variable_ref", "variable_id": id })
}

fn input_decl(id: &str, source_kind: &str, dimension: &str, unit: &str) -> Value {
    json!({
        "input_id": id, "name": id, "source_kind": source_kind,
        "required_for": "rule_check", "provenance_required": true,
        "redistribution_status_required": true,
        "quantity_intent": {
            "dimension": dimension, "unit_ref": unit,
            "unit_required": true, "dimension_check_required": true
        }
    })
}

fn formula(id: &str, ast: Value, inputs: &[&str]) -> Value {
    json!({
        "formula_id": id,
        "declaration_payload": { "expression_ast": ast },
        "input_refs": inputs.iter().map(|i| json!({"ref_id": i, "ref_type": "required_input"})).collect::<Vec<_>>()
    })
}

fn check(id: &str, formula_id: &str, inputs: &[&str], slot: Option<&str>) -> Value {
    let mut value = json!({
        "check_id": id,
        "required_input_refs": inputs.iter().map(|i| json!({"ref_id": i, "ref_type": "required_input"})).collect::<Vec<_>>(),
        "formula_ref": { "ref_id": formula_id, "ref_type": "formula" },
        "result_statuses": ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"],
        "diagnostic_policy": {
            "missing_input": "RULE_INPUT_MISSING",
            "evaluator_error": "RULE_EVALUATOR_ERROR"
        }
    });
    if let Some(slot) = slot {
        value["value_slot_refs"] = json!([{ "ref_id": slot, "ref_type": "value_slot" }]);
    }
    value
}

/// Invented pack: `actual` (solver result, stress, MPa), `limit` (user,
/// stress, MPa), `temp` (solver result, temperature, degC) and `temp_limit`
/// (user, temperature, degC), with five checks.
fn pack() -> Value {
    json!({
        "grammar_version": "1.0.0",
        "metadata": { "rule_pack_id": "invented_interval_pack" },
        "required_inputs": [
            input_decl("actual", "solver_result", "stress", "MPa"),
            input_decl("limit", "user_supplied_rule_value", "stress", "MPa"),
            input_decl("temp", "solver_result", "temperature", "degC"),
            input_decl("temp_limit", "user_supplied_rule_value", "temperature", "degC")
        ],
        "formula_declarations": [
            formula("ratio", json!({"node": "binary", "operator": "divide",
                "left": var("actual"), "right": var("limit")}), &["actual", "limit"]),
            formula("predicate", json!({"node": "compare", "operator": "less_than_or_equal",
                "left": var("actual"), "right": var("limit")}), &["actual", "limit"]),
            formula("temperature", json!({"node": "compare", "operator": "less_than_or_equal",
                "left": var("temp"), "right": var("temp_limit")}), &["temp", "temp_limit"]),
            formula("limit_only", json!({"node": "binary", "operator": "divide",
                "left": var("limit"), "right": var("limit")}), &["limit"]),
            formula("headroom", json!({"node": "binary", "operator": "divide",
                "left": var("limit"),
                "right": {"node": "binary", "operator": "subtract",
                          "left": var("limit"), "right": var("actual")}}), &["actual", "limit"])
        ],
        "value_slots": [{
            "slot_id": "ratio_limit", "slot_kind": "ratio_limit",
            "quantity_intent": { "dimension": "dimensionless", "unit_ref": "ratio",
                                  "unit_required": true, "dimension_check_required": true }
        }],
        "check_definitions": [
            check("ratio_check", "ratio", &["actual", "limit"], Some("ratio_limit")),
            check("predicate_check", "predicate", &["actual", "limit"], None),
            check("temperature_check", "temperature", &["temp", "temp_limit"], None),
            check("unused_interval_check", "limit_only", &["actual", "limit"], Some("ratio_limit")),
            check("headroom_check", "headroom", &["actual", "limit"], Some("ratio_limit"))
        ]
    })
}

fn solver(input_id: &str, value: f64, unit: &str) -> SolverResultBinding {
    SolverResultBinding {
        input_id: input_id.to_string(),
        result_id: format!("result:invented:{input_id}"),
        value,
        unit: unit.to_string(),
    }
}

fn supplied(ref_id: &str, value: f64, unit: &str, dimension: &str) -> SuppliedValueBinding {
    SuppliedValueBinding {
        ref_id: ref_id.to_string(),
        value,
        unit: unit.to_string(),
        dimension: dimension.to_string(),
    }
}

fn bound(input_id: &str, absolute_bound: f64) -> SolverResultBound {
    SolverResultBound {
        input_id: input_id.to_string(),
        absolute_bound,
    }
}

fn input<'a>(
    document: &'a Value,
    actual: f64,
    actual_unit: &str,
    temp: f64,
) -> RuleCheckRunInput<'a> {
    RuleCheckRunInput {
        rule_pack_document: document,
        solver_results: vec![
            solver("actual", actual, actual_unit),
            solver("temp", temp, "degC"),
        ],
        refused_solver_results: Vec::new(),
        supplied_values: vec![
            supplied("limit", 100.0, "MPa", "stress"),
            supplied("temp_limit", 300.0, "degC", "temperature"),
            supplied("ratio_limit", 1.0, "ratio", "dimensionless"),
        ],
        library_values: Vec::new(),
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    }
}

fn outcome<'a>(result: &'a RuleCheckRunResult, check_id: &str) -> &'a CheckOutcome {
    result
        .checks
        .iter()
        .find(|c| c.check_id == check_id)
        .expect("check present")
}

fn bytes(result: &RuleCheckRunResult) -> String {
    serde_json::to_string(result).expect("serializable")
}

fn codes(outcome: &CheckOutcome) -> Vec<&str> {
    outcome
        .diagnostic_codes
        .iter()
        .map(String::as_str)
        .collect()
}

fn finding<'a>(
    outcome: &'a CheckOutcome,
    code: &str,
) -> &'a open_pipe_stress_rule_check_runner::RunFinding {
    outcome
        .evaluator_findings
        .iter()
        .find(|f| f.code == code)
        .unwrap_or_else(|| panic!("finding {code} in {:?}", outcome.evaluator_findings))
}

#[test]
fn without_bounds_the_run_is_byte_identical_to_run_rule_checks() {
    let document = pack();
    for (actual, unit, temp) in [
        (50.0, "MPa", 20.0),
        (100.0, "MPa", 300.0),
        (150.0, "MPa", 400.0),
        (14_503.77, "psi", 20.0),
        (0.0, "MPa", -10.0),
        (100.0, "bogus_unit", 20.0),
    ] {
        let input = input(&document, actual, unit, temp);
        let point = bytes(&run_rule_checks(&input));
        assert_eq!(bytes(&run_rule_checks_with_bounds(&input, &[])), point);
        // A bound for an input the pack does not bind, and b = 0 (the exact
        // point q, D2 §4.11.2), leave the run unchanged too.
        assert_eq!(
            bytes(&run_rule_checks_with_bounds(
                &input,
                &[bound("not_an_input", 5.0)]
            )),
            point
        );
        assert_eq!(
            bytes(&run_rule_checks_with_bounds(
                &input,
                &[bound("actual", 0.0), bound("temp", 0.0)]
            )),
            point
        );
        // A bound on a user-supplied input is never consulted.
        assert_eq!(
            bytes(&run_rule_checks_with_bounds(&input, &[bound("limit", 5.0)])),
            point
        );
    }
}

#[test]
fn every_value_passing_is_checked_with_the_all_pass_code() {
    let document = pack();
    let result = run_rule_checks_with_bounds(
        &input(&document, 50.0, "MPa", 20.0),
        &[bound("actual", 1.0)],
    );
    let ratio = outcome(&result, "ratio_check");
    assert_eq!(ratio.status, RuleCheckStatus::UserRuleChecked);
    assert_eq!(codes(ratio), vec![RULE_INTERVAL_ALL_PASS]);
    let pass = finding(ratio, RULE_INTERVAL_ALL_PASS);
    assert_eq!(pass.severity, "info");
    assert!(pass.message.starts_with("enclosure=[0x") && pass.message.ends_with("] unit=ratio"));
    // No single value is computed in interval mode; the limit is a point.
    assert!(ratio.computed_value.is_none());
    assert_eq!(ratio.limit_value.as_ref().map(|l| l.value), Some(1.0));
    assert_eq!(ratio.acceptability_relation, "less_than_or_equal");
    let actual = ratio
        .bound_inputs
        .iter()
        .find(|b| b.input_id == "actual")
        .unwrap();
    assert_eq!(actual.note.as_deref(), Some("interval ±1e0 from receipt"));
    assert_eq!(actual.value, Some(50.0));
    let predicate = outcome(&result, "predicate_check");
    assert_eq!(predicate.status, RuleCheckStatus::UserRuleChecked);
    assert_eq!(predicate.acceptability_relation, "formula_predicate");
    assert_eq!(
        finding(predicate, RULE_INTERVAL_ALL_PASS).message,
        "enclosure=none unit=none"
    );
    // The temperature check binds no interval input: it is the point path.
    let temperature = outcome(&result, "temperature_check");
    assert_eq!(temperature.status, RuleCheckStatus::UserRuleChecked);
    assert!(temperature.diagnostic_codes.is_empty());
    assert!(temperature.evaluator_findings.is_empty());
}

#[test]
fn every_value_failing_is_failed_with_the_all_fail_code() {
    let document = pack();
    let result = run_rule_checks_with_bounds(
        &input(&document, 150.0, "MPa", 20.0),
        &[bound("actual", 1.0)],
    );
    for check_id in ["ratio_check", "predicate_check"] {
        let failed = outcome(&result, check_id);
        assert_eq!(failed.status, RuleCheckStatus::UserRuleFailed, "{check_id}");
        assert_eq!(codes(failed), vec![RULE_INTERVAL_ALL_FAIL]);
        assert_eq!(finding(failed, RULE_INTERVAL_ALL_FAIL).severity, "info");
    }
    assert_eq!(result.aggregate_status, RuleCheckStatus::UserRuleFailed);
}

#[test]
fn a_straddling_result_is_indeterminate_never_a_pass() {
    let document = pack();
    // q = 100 = the limit: the point path passes (100/100 <= 1), but values
    // above 100 within the bound fail, so interval mode must not pass.
    let input = input(&document, 100.0, "MPa", 20.0);
    let point = run_rule_checks(&input);
    assert_eq!(
        outcome(&point, "ratio_check").status,
        RuleCheckStatus::UserRuleChecked
    );
    for b in [1.0, 1.0e-12, f64::from_bits(1)] {
        let result = run_rule_checks_with_bounds(&input, &[bound("actual", b)]);
        for check_id in ["ratio_check", "predicate_check"] {
            let straddle = outcome(&result, check_id);
            assert_eq!(
                straddle.status,
                RuleCheckStatus::RuleInputsIncomplete,
                "{check_id} b={b:e}"
            );
            assert_eq!(codes(straddle), vec![RULE_RESULT_INDETERMINATE]);
            assert_eq!(
                finding(straddle, RULE_RESULT_INDETERMINATE).severity,
                "warning"
            );
        }
        assert_ne!(result.aggregate_status, RuleCheckStatus::UserRuleChecked);
    }
}

#[test]
fn a_divisor_range_containing_zero_is_indeterminate_with_its_finding() {
    let document = pack();
    let result = run_rule_checks_with_bounds(
        &input(&document, 100.0, "MPa", 20.0),
        &[bound("actual", 1.0)],
    );
    let headroom = outcome(&result, "headroom_check");
    assert_eq!(headroom.status, RuleCheckStatus::RuleInputsIncomplete);
    assert_eq!(codes(headroom), vec![RULE_RESULT_INDETERMINATE]);
    let indeterminate = finding(headroom, RULE_RESULT_INDETERMINATE);
    assert_eq!(
        indeterminate.message,
        "enclosure=none unit=ratio; causes=divide_by_zero_range"
    );
    assert_eq!(
        finding(headroom, RULE_INTERVAL_DIVIDE_BY_ZERO_RANGE).subject_id,
        "divide"
    );
}

#[test]
fn an_interval_input_outside_the_formula_leaves_the_point_path() {
    let document = pack();
    let input = input(&document, 50.0, "MPa", 20.0);
    let point = run_rule_checks(&input);
    let bounded = run_rule_checks_with_bounds(&input, &[bound("actual", 1.0)]);
    let point_check = outcome(&point, "unused_interval_check");
    let bounded_check = outcome(&bounded, "unused_interval_check");
    assert_eq!(bounded_check.status, point_check.status);
    assert_eq!(
        serde_json::to_value(&bounded_check.computed_value).unwrap(),
        serde_json::to_value(&point_check.computed_value).unwrap()
    );
    assert!(bounded_check.diagnostic_codes.is_empty());
}

#[test]
fn an_invalid_bound_blocks_the_input() {
    let document = pack();
    for b in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let result = run_rule_checks_with_bounds(
            &input(&document, 50.0, "MPa", 20.0),
            &[bound("actual", b)],
        );
        let blocked = outcome(&result, "ratio_check");
        assert_eq!(
            blocked.status,
            RuleCheckStatus::RuleInputsIncomplete,
            "b={b}"
        );
        assert!(blocked
            .completeness_findings
            .iter()
            .any(|f| f.code == "RULE_EVALUATOR_ERROR" && f.severity == "blocking"));
        let actual = blocked
            .bound_inputs
            .iter()
            .find(|b| b.input_id == "actual")
            .unwrap();
        assert!(!actual.supplied);
        assert_eq!(codes(blocked), vec!["RULE_INPUT_MISSING"]);
    }
}

#[test]
fn unit_normalization_steps_each_end_outward() {
    // Entered psi, declared MPa: each end goes through the units crate's four
    // operations, each stepped outward, so the enclosure strictly contains the
    // units crate's own conversion of q - b and q + b.
    let document = pack();
    let (q, b): (f64, f64) = (14_000.0, 2.0);
    let dimension = UnitDimension::from_schema_value("stress").unwrap();
    let (psi, mpa) = (
        unit_by_symbol("psi", dimension).unwrap(),
        unit_by_symbol("MPa", dimension).unwrap(),
    );
    let lo = convert_for_dimension((q - b).next_down(), dimension, psi, mpa).unwrap();
    let hi = convert_for_dimension((q + b).next_up(), dimension, psi, mpa).unwrap();
    // A predicate whose limit sits just inside the converted range reads
    // indeterminate; one just outside reads decided.
    let mut input = input(&document, q, "psi", 20.0);
    input.supplied_values[0] = supplied("limit", hi, "MPa", "stress");
    let result = run_rule_checks_with_bounds(&input, &[bound("actual", b)]);
    assert_eq!(
        outcome(&result, "predicate_check").status,
        RuleCheckStatus::RuleInputsIncomplete,
        "a limit equal to the units crate's own upper end must not pass"
    );
    input.supplied_values[0] = supplied("limit", hi * 1.0001, "MPa", "stress");
    let result = run_rule_checks_with_bounds(&input, &[bound("actual", b)]);
    assert_eq!(
        outcome(&result, "predicate_check").status,
        RuleCheckStatus::UserRuleChecked
    );
    input.supplied_values[0] = supplied("limit", lo * 0.9999, "MPa", "stress");
    let result = run_rule_checks_with_bounds(&input, &[bound("actual", b)]);
    assert_eq!(
        outcome(&result, "predicate_check").status,
        RuleCheckStatus::UserRuleFailed
    );

    // Affine units (degF entered, degC declared) convert outward too.
    let temperature = UnitDimension::from_schema_value("temperature").unwrap();
    let (degf, degc) = (
        unit_by_symbol("degF", temperature).unwrap(),
        unit_by_symbol("degC", temperature).unwrap(),
    );
    let upper = convert_for_dimension((572.0f64 + 1.0).next_up(), temperature, degf, degc).unwrap();
    let mut input = input_with_temp_unit(&document, 572.0, "degF");
    input.supplied_values[1] = supplied("temp_limit", upper, "degC", "temperature");
    let result = run_rule_checks_with_bounds(&input, &[bound("temp", 1.0)]);
    assert_eq!(
        outcome(&result, "temperature_check").status,
        RuleCheckStatus::RuleInputsIncomplete
    );
}

fn input_with_temp_unit<'a>(document: &'a Value, temp: f64, unit: &str) -> RuleCheckRunInput<'a> {
    let mut run = input(document, 50.0, "MPa", temp);
    run.solver_results[1] = solver("temp", temp, unit);
    run
}

#[test]
fn bounded_checks_never_panic_where_the_point_path_can() {
    // Inputs at which the ordinary point path panics (T3-SI1b): a same-dimension
    // quotient that overflows. The interval path reads indeterminate instead.
    let document = pack();
    let mut input = input(&document, 1.0e308, "MPa", 20.0);
    input.supplied_values[0] = supplied("limit", 1.0e-308, "MPa", "stress");
    let result = run_rule_checks_with_bounds(&input, &[bound("actual", 1.0e292)]);
    let ratio = outcome(&result, "ratio_check");
    assert_eq!(ratio.status, RuleCheckStatus::RuleInputsIncomplete);
    assert_eq!(codes(ratio), vec![RULE_RESULT_INDETERMINATE]);
    assert!(finding(ratio, RULE_RESULT_INDETERMINATE)
        .message
        .ends_with("causes=non_finite_enclosure"));
    // A bound so large that the input itself has no finite enclosure.
    let result = run_rule_checks_with_bounds(
        &crate_input_at(&document, f64::MAX),
        &[bound("actual", f64::MAX)],
    );
    assert_eq!(
        outcome(&result, "predicate_check").status,
        RuleCheckStatus::RuleInputsIncomplete
    );
}

fn crate_input_at(document: &Value, actual: f64) -> RuleCheckRunInput<'_> {
    input(document, actual, "MPa", 20.0)
}

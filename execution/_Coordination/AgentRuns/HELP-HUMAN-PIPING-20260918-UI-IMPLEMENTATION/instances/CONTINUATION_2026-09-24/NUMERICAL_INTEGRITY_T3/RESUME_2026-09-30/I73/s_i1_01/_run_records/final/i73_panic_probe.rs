//! I73 scratch probe (not repository content): the inputs on which the ordinary
//! point path panics (T3-SI1b), run through the point path under catch_unwind
//! and through interval mode / the bounded runner without it.
use open_pipe_stress_expression_evaluator::*;
use open_pipe_stress_rule_check_runner::*;
use serde_json::json;

fn q(v: f64, d: Dimension, u: &str) -> Quantity { Quantity::new(v, d, u).unwrap() }
fn lit_r(v: f64) -> Expression { Expression::Literal(q(v, Dimension::Dimensionless, "ratio")) }
fn var(i: &str) -> Expression { Expression::VariableRef(i.into()) }
fn bin(o: BinaryOperator, l: Expression, r: Expression) -> Expression { Expression::Binary { operator: o, left: Box::new(l), right: Box::new(r) } }

#[test]
fn i73_panic_probe() {
    let rows = vec![TableRow { argument: 0.0, result: 1.0 }, TableRow { argument: 1.0, result: 2.0 }, TableRow { argument: 2.0, result: 3.0 }];
    let table = |id: &str| UserTable { table_id: id.into(), argument_dimension: Dimension::Dimensionless, argument_unit_ref: "ratio".into(),
        result_dimension: Dimension::Stress, result_unit_ref: "u".into(), rows: rows.clone() };
    let huge = || bin(BinaryOperator::Multiply, bin(BinaryOperator::Multiply, var("z"), lit_r(1e300)), lit_r(1e300));
    let nan = || bin(BinaryOperator::Subtract, huge(), huge());
    let cases: Vec<(&str, Expression)> = vec![
        ("overflowing_same_dimension_quotient", bin(BinaryOperator::Divide, var("x"), var("y"))),
        ("nan_interpolate_argument", Expression::Interpolate { table: table("t"), argument: Box::new(nan()) }),
        ("nan_step_lookup_argument", Expression::Lookup { table: table("t"), mode: LookupMode::Step, argument: Box::new(nan()) }),
    ];
    let bindings = vec![
        VariableBinding::new("x", BindingSource::SolverResultField, q(1e308, Dimension::Stress, "u")),
        VariableBinding::new("y", BindingSource::SolverResultField, q(1e-308, Dimension::Stress, "u")),
        VariableBinding::new("z", BindingSource::SolverResultField, q(1.0, Dimension::Dimensionless, "ratio")),
    ];
    for (name, expression) in cases {
        let input = EvaluationInput { expression, bindings: bindings.clone(), required_variable_ids: vec![],
            statuses: vec![AnalysisStatus::MechanicsSolved], declared_grammar_version: GRAMMAR_VERSION.into() };
        let point = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| evaluate(&input)));
        let points_only = evaluate_interval(&input, &[]);
        let with_interval = evaluate_interval(&input, &[IntervalBinding { variable_id: "x".into(), enclosure: enclosure_from_bound(1e308, 1e292) },
                                                        IntervalBinding { variable_id: "z".into(), enclosure: enclosure_from_bound(1.0, 0.5) }]);
        println!("PROBE {name}: point_path={} interval_points_only={:?} notes={:?} interval_with_bounds={:?}",
                 if point.is_err() { "PANIC" } else { "no panic" },
                 points_only.value, points_only.notes.iter().map(|n| n.code.as_str()).collect::<Vec<_>>(), with_interval.value);
    }
    // Runner: the ratio pack at actual = 1e308, limit = 1e-308.
    let pack = json!({"grammar_version":"1.0.0","metadata":{"rule_pack_id":"probe"},
      "required_inputs":[
        {"input_id":"actual","name":"actual","source_kind":"solver_result","required_for":"rule_check","provenance_required":true,"redistribution_status_required":true,
         "quantity_intent":{"dimension":"stress","unit_ref":"u","unit_required":true,"dimension_check_required":true}},
        {"input_id":"limit","name":"limit","source_kind":"user_supplied_rule_value","required_for":"rule_check","provenance_required":true,"redistribution_status_required":true,
         "quantity_intent":{"dimension":"stress","unit_ref":"u","unit_required":true,"dimension_check_required":true}}],
      "formula_declarations":[{"formula_id":"ratio","declaration_payload":{"expression_ast":{"node":"binary","operator":"divide",
         "left":{"node":"variable_ref","variable_id":"actual"},"right":{"node":"variable_ref","variable_id":"limit"}}},
         "input_refs":[{"ref_id":"actual","ref_type":"required_input"},{"ref_id":"limit","ref_type":"required_input"}]}],
      "value_slots":[{"slot_id":"ratio_limit","slot_kind":"ratio_limit","quantity_intent":{"dimension":"dimensionless","unit_ref":"ratio","unit_required":true,"dimension_check_required":true}}],
      "check_definitions":[{"check_id":"ratio_check","required_input_refs":[{"ref_id":"actual","ref_type":"required_input"},{"ref_id":"limit","ref_type":"required_input"}],
         "value_slot_refs":[{"ref_id":"ratio_limit","ref_type":"value_slot"}],"formula_ref":{"ref_id":"ratio","ref_type":"formula"},
         "result_statuses":["RULE_INPUTS_INCOMPLETE","USER_RULE_CHECKED","USER_RULE_FAILED"]}]});
    let input = RuleCheckRunInput { rule_pack_document: &pack,
        solver_results: vec![SolverResultBinding { input_id: "actual".into(), result_id: "r".into(), value: 1e308, unit: "u".into() }],
        refused_solver_results: vec![],
        supplied_values: vec![SuppliedValueBinding { ref_id: "limit".into(), value: 1e-308, unit: "u".into(), dimension: "stress".into() },
                              SuppliedValueBinding { ref_id: "ratio_limit".into(), value: 1.0, unit: "ratio".into(), dimension: "dimensionless".into() }],
        library_values: vec![], current_statuses: vec![AnalysisStatus::MechanicsSolved] };
    let point = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run_rule_checks(&input)));
    let bounded = run_rule_checks_with_bounds(&input, &[SolverResultBound { input_id: "actual".into(), absolute_bound: 1e292 }]);
    println!("PROBE runner: run_rule_checks={} run_rule_checks_with_bounds status={:?} codes={:?}",
             if point.is_err() { "PANIC" } else { "no panic" }, bounded.checks[0].status, bounded.checks[0].diagnostic_codes);
}

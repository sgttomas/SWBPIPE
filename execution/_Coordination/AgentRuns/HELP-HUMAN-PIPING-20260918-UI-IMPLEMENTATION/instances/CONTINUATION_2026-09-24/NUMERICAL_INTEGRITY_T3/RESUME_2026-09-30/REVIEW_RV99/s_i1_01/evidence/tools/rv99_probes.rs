//! RV99 probes (scratch; not a candidate file): contract edges of the bounded
//! runner. Prints JSON lines; asserts nothing. Invented values only.

use open_pipe_stress_expression_evaluator::AnalysisStatus;
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, run_rule_checks_with_bounds, RuleCheckRunInput, SolverResultBinding,
    SolverResultBound, SuppliedValueBinding,
};
use serde_json::{json, Value};

fn var(id: &str) -> Value {
    json!({ "node": "variable_ref", "variable_id": id })
}

fn decl(id: &str, kind: &str, dim: &str, unit: &str) -> Value {
    json!({"input_id": id, "name": id, "source_kind": kind, "required_for": "rule_check",
        "provenance_required": true, "redistribution_status_required": true,
        "quantity_intent": {"dimension": dim, "unit_ref": unit, "unit_required": true,
                            "dimension_check_required": true}})
}

fn pack(formula: Value, formula_inputs: &[&str], statuses: &[&str], slot: bool) -> Value {
    let mut check = json!({
        "check_id": "c1",
        "required_input_refs": [{"ref_id": "actual", "ref_type": "required_input"},
                                {"ref_id": "limit", "ref_type": "required_input"}],
        "formula_ref": {"ref_id": "f1", "ref_type": "formula"},
        "result_statuses": statuses,
        "diagnostic_policy": {"missing_input": "RULE_INPUT_MISSING", "evaluator_error": "RULE_EVALUATOR_ERROR"}
    });
    if slot {
        check["value_slot_refs"] = json!([{"ref_id": "ratio_limit", "ref_type": "value_slot"}]);
    }
    json!({
        "grammar_version": "1.0.0",
        "metadata": {"rule_pack_id": "rv99_probe"},
        "required_inputs": [decl("actual", "solver_result", "stress", "MPa"),
                            decl("limit", "user_supplied_rule_value", "stress", "MPa")],
        "formula_declarations": [{"formula_id": "f1",
            "declaration_payload": {"expression_ast": formula},
            "input_refs": formula_inputs.iter().map(|i| json!({"ref_id": i, "ref_type": "required_input"})).collect::<Vec<_>>()}],
        "value_slots": [{"slot_id": "ratio_limit", "slot_kind": "ratio_limit",
            "quantity_intent": {"dimension": "dimensionless", "unit_ref": "ratio",
                                "unit_required": true, "dimension_check_required": true}}],
        "check_definitions": [check]
    })
}

fn input<'a>(doc: &'a Value, actual: f64, limit: f64, statuses: Vec<AnalysisStatus>) -> RuleCheckRunInput<'a> {
    RuleCheckRunInput {
        rule_pack_document: doc,
        solver_results: vec![SolverResultBinding {
            input_id: "actual".into(), result_id: "r:actual".into(), value: actual, unit: "MPa".into()}],
        refused_solver_results: Vec::new(),
        supplied_values: vec![
            SuppliedValueBinding { ref_id: "limit".into(), value: limit, unit: "MPa".into(), dimension: "stress".into() },
            SuppliedValueBinding { ref_id: "ratio_limit".into(), value: 1.0, unit: "ratio".into(), dimension: "dimensionless".into() },
        ],
        library_values: Vec::new(),
        current_statuses: statuses,
    }
}

fn b(id: &str, v: f64) -> SolverResultBound {
    SolverResultBound { input_id: id.into(), absolute_bound: v }
}

fn show(label: &str, r: &open_pipe_stress_rule_check_runner::RuleCheckRunResult) {
    let c = &r.checks[0];
    println!("{}", json!({
        "probe": label,
        "status": c.status.as_str(),
        "diagnostic_codes": c.diagnostic_codes,
        "evaluator_findings": c.evaluator_findings.iter().map(|f| vec![f.code.clone(), f.severity.clone(), f.message.clone()]).collect::<Vec<_>>(),
        "completeness_findings": c.completeness_findings.iter().map(|f| vec![f.code.clone(), f.severity.clone()]).collect::<Vec<_>>(),
        "notes": c.bound_inputs.iter().map(|x| x.note.clone()).collect::<Vec<_>>(),
        "computed_value": c.computed_value.as_ref().map(|q| q.value),
    }));
}

#[test]
fn rv99_probes() {
    let ratio = json!({"node": "binary", "operator": "divide", "left": var("actual"), "right": var("limit")});
    let all = ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"];
    let ms = || vec![AnalysisStatus::MechanicsSolved];

    // (a) pre-existing: a completeness "info" finding when the caller's
    // current statuses include USER_RULE_CHECKED (point path, no bounds).
    let doc = pack(ratio.clone(), &["actual", "limit"], &all, true);
    show("a_point_info_severity", &run_rule_checks(&input(&doc, 50.0, 100.0, vec![AnalysisStatus::MechanicsSolved, AnalysisStatus::UserRuleChecked])));

    // (b) a pack that does not declare USER_RULE_CHECKED: an all-pass interval
    // check is downgraded, and keeps the all-pass code.
    let doc_b = pack(ratio.clone(), &["actual", "limit"], &["RULE_INPUTS_INCOMPLETE", "USER_RULE_FAILED"], true);
    show("b_undeclared_checked_point", &run_rule_checks(&input(&doc_b, 50.0, 100.0, ms())));
    show("b_undeclared_checked_interval", &run_rule_checks_with_bounds(&input(&doc_b, 50.0, 100.0, ms()), &[b("actual", 1.0)]));

    // (c) duplicate bounds for one input: the last one wins.
    show("c_duplicate_bounds_small_then_large", &run_rule_checks_with_bounds(&input(&doc, 50.0, 100.0, ms()), &[b("actual", 1.0), b("actual", 60.0)]));
    show("c_duplicate_bounds_large_then_small", &run_rule_checks_with_bounds(&input(&doc, 50.0, 100.0, ms()), &[b("actual", 60.0), b("actual", 1.0)]));

    // (g) a bounded input listed by the check but not used by its formula.
    let limit_only = json!({"node": "binary", "operator": "divide", "left": var("limit"), "right": var("limit")});
    let doc_g = pack(limit_only, &["limit"], &all, true);
    show("g_bound_input_not_in_formula", &run_rule_checks_with_bounds(&input(&doc_g, 50.0, 100.0, ms()), &[b("actual", 1.0)]));

    // (h) a subnormal bound binds an interval (D2 N-1): q exactly at the limit.
    show("h_subnormal_bound_at_limit", &run_rule_checks_with_bounds(&input(&doc, 100.0, 100.0, ms()), &[b("actual", 5e-324)]));
    show("h_zero_bound_at_limit", &run_rule_checks_with_bounds(&input(&doc, 100.0, 100.0, ms()), &[b("actual", 0.0)]));
    for bad in [f64::NAN, -1.0, f64::INFINITY, -0.0] {
        show(&format!("h_bound_{bad:?}"), &run_rule_checks_with_bounds(&input(&doc, 50.0, 100.0, ms()), &[b("actual", bad)]));
    }
    // (i) a boolean formula with a straddle, and the message form.
    let pred = json!({"node": "compare", "operator": "less_than_or_equal", "left": var("actual"), "right": var("limit")});
    let doc_i = pack(pred, &["actual", "limit"], &all, false);
    show("i_boolean_straddle", &run_rule_checks_with_bounds(&input(&doc_i, 100.0, 100.0, ms()), &[b("actual", 1e-9)]));
    // (j) ROOT ruling 3 reading: with b = 0 (or no bound) the bounded entry is
    // the point path, which panics on T3-SI1b's input exactly as today.
    let r = std::panic::catch_unwind(|| run_rule_checks_with_bounds(&input(&doc, 1e308, 1e-308, ms()), &[b("actual", 0.0)]));
    println!("{}", json!({"probe": "j_zero_bound_point_path_panic", "panicked": r.is_err()}));
    let r = std::panic::catch_unwind(|| run_rule_checks_with_bounds(&input(&doc, 1e308, 1e-308, ms()), &[b("actual", 5e-324)]));
    println!("{}", json!({"probe": "j_subnormal_bound_interval_path", "panicked": r.is_err()}));
}

//! I73 S-I1 control 1 (scratch harness, not repository content): dump the
//! serialized `RuleCheckRunResult` over the committed rule packs and the
//! numeric rows of the committed solved-result fixtures. Base and candidate
//! dumps must be byte-identical when no interval input is bound.
//!
//! I79 T3-SI1b: `run_under_test` is the plain `run_rule_checks`; the added
//! `i79_runner_extreme_dump` runs extreme values (and b = 0 bounds) through
//! the committed and inline packs under `catch_unwind`, recording each panic's
//! site; positive bounds (0.5, 1e-300, 1e300) run the interval path, which
//! must be unchanged.

use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::AnalysisStatus;
#[allow(unused_imports)]
use open_pipe_stress_rule_check_runner::*;
use serde_json::{json, Value};

fn run_under_test(input: &RuleCheckRunInput) -> RuleCheckRunResult {
    run_rule_checks(input)
}

fn project() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn read_json(rel: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(project().join(rel)).unwrap()).unwrap()
}

/// The inline pack of `rule_check_runner/src/lib.rs` `tests::demo_pack`.
fn inline_demo_pack() -> Value {
    json!({
        "grammar_version": "1.0.0",
        "metadata": { "rule_pack_id": "test_demo_pack" },
        "required_inputs": [
            { "input_id": "actual", "name": "actual", "source_kind": "solver_result",
              "required_for": "rule_check", "provenance_required": true,
              "redistribution_status_required": true,
              "quantity_intent": { "dimension": "stress", "unit_ref": "demo_unit",
                                   "unit_required": true, "dimension_check_required": true } },
            { "input_id": "limit", "name": "limit", "source_kind": "user_supplied_rule_value",
              "required_for": "rule_check", "provenance_required": true,
              "redistribution_status_required": true,
              "quantity_intent": { "dimension": "stress", "unit_ref": "demo_unit",
                                   "unit_required": true, "dimension_check_required": true } }
        ],
        "formula_declarations": [{
            "formula_id": "ratio",
            "declaration_payload": { "expression_ast": {
                "node": "binary", "operator": "divide",
                "left": { "node": "variable_ref", "variable_id": "actual" },
                "right": { "node": "variable_ref", "variable_id": "limit" } } },
            "input_refs": [ { "ref_id": "actual", "ref_type": "required_input" },
                            { "ref_id": "limit", "ref_type": "required_input" } ]
        }],
        "value_slots": [{ "slot_id": "ratio_limit", "slot_kind": "ratio_limit",
            "quantity_intent": { "dimension": "dimensionless", "unit_ref": "ratio",
                                 "unit_required": true, "dimension_check_required": true } }],
        "check_definitions": [{
            "check_id": "ratio_check",
            "required_input_refs": [ { "ref_id": "actual", "ref_type": "required_input" },
                                     { "ref_id": "limit", "ref_type": "required_input" } ],
            "value_slot_refs": [ { "ref_id": "ratio_limit", "ref_type": "value_slot" } ],
            "formula_ref": { "ref_id": "ratio", "ref_type": "formula" },
            "result_statuses": [ "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED" ],
            "diagnostic_policy": { "missing_input": "RULE_INPUT_MISSING",
                                   "evaluator_error": "RULE_EVALUATOR_ERROR" }
        }]
    })
}

struct Pack {
    label: String,
    document: Value,
    solver_id: &'static str,
    limit_id: &'static str,
    slot_id: &'static str,
    unit: String,
}

fn packs() -> Vec<Pack> {
    let demo = read_json("examples/rule_packs/invented_demo.yaml");
    let fixture = read_json("fixtures/product_preview/invented_demo_rule_pack.json");
    let mut out = vec![
        Pack { label: "examples_invented_demo".into(), document: demo.clone(), solver_id: "demo_actual_quantity",
               limit_id: "demo_limit_quantity", slot_id: "demo_limit_slot", unit: "demo_unit".into() },
        Pack { label: "fixture_invented_demo_rule_pack".into(), document: fixture, solver_id: "demo_actual_quantity",
               limit_id: "demo_limit_quantity", slot_id: "demo_limit_slot", unit: "demo_unit".into() },
        Pack { label: "inline_demo_pack".into(), document: inline_demo_pack(), solver_id: "actual",
               limit_id: "limit", slot_id: "ratio_limit", unit: "demo_unit".into() },
    ];
    for relation in ["less_than", "less_than_or_equal", "greater_than", "greater_than_or_equal", "bogus_relation"] {
        let mut document = demo.clone();
        document["check_definitions"][0]["acceptability_relation"] = json!(relation);
        out.push(Pack { label: format!("examples_invented_demo_{relation}"), document, solver_id: "demo_actual_quantity",
                        limit_id: "demo_limit_quantity", slot_id: "demo_limit_slot", unit: "demo_unit".into() });
    }
    for unit in ["MPa", "Pa"] {
        let mut document = demo.clone();
        document["required_inputs"][0]["quantity_intent"]["unit_ref"] = json!(unit);
        document["required_inputs"][1]["quantity_intent"]["unit_ref"] = json!(unit);
        out.push(Pack { label: format!("examples_invented_demo_unit_{unit}"), document, solver_id: "demo_actual_quantity",
                        limit_id: "demo_limit_quantity", slot_id: "demo_limit_slot", unit: unit.into() });
    }
    out
}

/// Distinct (value bits, unit) pairs of every finite numeric result row.
fn fixture_rows() -> Vec<(u64, String)> {
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(project().join("fixtures/product_preview")).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        if name.starts_with("invented_mechanics_result") && name.ends_with(".json") {
            files.push(path);
        }
    }
    fn walk(dir: PathBuf, files: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(path, files);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                files.push(path);
            }
        }
    }
    walk(project().join("fixtures/results"), &mut files);
    files.sort();
    let mut rows = BTreeSet::new();
    for path in files {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let Ok(document) = serde_json::from_str::<Value>(&text) else { continue };
        let Some(results) = document.get("results").and_then(Value::as_array) else { continue };
        for row in results {
            if let (Some(value), Some(unit)) = (row.get("value").and_then(Value::as_f64), row.get("unit").and_then(Value::as_str)) {
                if row.get("value").map(Value::is_number) == Some(true) && value.is_finite() {
                    rows.insert((value.to_bits(), unit.to_string()));
                }
            }
        }
    }
    rows.into_iter().collect()
}

fn supplied(ref_id: &str, value: f64, unit: &str, dimension: &str) -> SuppliedValueBinding {
    SuppliedValueBinding { ref_id: ref_id.into(), value, unit: unit.into(), dimension: dimension.into() }
}

#[test]
fn i73_runner_dump() {
    let out_path = std::env::var("I73_RUN_DUMP_OUT").expect("I73_RUN_DUMP_OUT");
    let mut out = std::io::BufWriter::new(fs::File::create(out_path).unwrap());
    let rows = fixture_rows();
    writeln!(out, "# rows {}", rows.len()).unwrap();
    for pack in packs() {
        for (index, (value_bits, row_unit)) in rows.iter().enumerate() {
            let value = f64::from_bits(*value_bits);
            for (mode, unit) in [("declared_unit", pack.unit.as_str()), ("row_unit", row_unit.as_str())] {
                let input = RuleCheckRunInput {
                    rule_pack_document: &pack.document,
                    solver_results: vec![SolverResultBinding { input_id: pack.solver_id.into(),
                        result_id: format!("result:fixture:{index}"), value, unit: unit.into() }],
                    refused_solver_results: Vec::new(),
                    supplied_values: vec![supplied(pack.limit_id, 100.0, &pack.unit, "stress"),
                                          supplied(pack.slot_id, 1.0, "ratio", "dimensionless")],
                    library_values: Vec::new(),
                    current_statuses: vec![AnalysisStatus::MechanicsSolved],
                };
                let result = run_under_test(&input);
                writeln!(out, "{}\t{:016x}\t{}\t{}\t{}", pack.label, value_bits, row_unit, mode,
                         serde_json::to_string(&result).unwrap()).unwrap();
            }
            if index % 50 == 0 {
                // Scenario variants on a sample of rows.
                let scenarios: Vec<(&str, RuleCheckRunInput)> = vec![
                    ("no_solver", RuleCheckRunInput { rule_pack_document: &pack.document, solver_results: vec![],
                        refused_solver_results: vec![], supplied_values: vec![supplied(pack.limit_id, 100.0, &pack.unit, "stress"),
                        supplied(pack.slot_id, 1.0, "ratio", "dimensionless")], library_values: vec![],
                        current_statuses: vec![AnalysisStatus::MechanicsSolved] }),
                    ("refused", RuleCheckRunInput { rule_pack_document: &pack.document, solver_results: vec![],
                        refused_solver_results: vec![RefusedSolverResult { input_id: pack.solver_id.into(), reason: "RESULT_REFUSED_INVENTED".into() }],
                        supplied_values: vec![supplied(pack.limit_id, 100.0, &pack.unit, "stress"),
                        supplied(pack.slot_id, 1.0, "ratio", "dimensionless")], library_values: vec![],
                        current_statuses: vec![AnalysisStatus::MechanicsSolved] }),
                    ("no_slot", RuleCheckRunInput { rule_pack_document: &pack.document,
                        solver_results: vec![SolverResultBinding { input_id: pack.solver_id.into(), result_id: "r".into(), value, unit: pack.unit.clone() }],
                        refused_solver_results: vec![], supplied_values: vec![supplied(pack.limit_id, 100.0, &pack.unit, "stress")],
                        library_values: vec![], current_statuses: vec![AnalysisStatus::MechanicsSolved] }),
                    ("model_incomplete", RuleCheckRunInput { rule_pack_document: &pack.document,
                        solver_results: vec![SolverResultBinding { input_id: pack.solver_id.into(), result_id: "r".into(), value, unit: pack.unit.clone() }],
                        refused_solver_results: vec![], supplied_values: vec![supplied(pack.limit_id, 100.0, &pack.unit, "stress"),
                        supplied(pack.slot_id, 1.0, "ratio", "dimensionless")],
                        library_values: vec![], current_statuses: vec![AnalysisStatus::ModelIncomplete] }),
                    ("zero_limit", RuleCheckRunInput { rule_pack_document: &pack.document,
                        solver_results: vec![SolverResultBinding { input_id: pack.solver_id.into(), result_id: "r".into(), value, unit: pack.unit.clone() }],
                        refused_solver_results: vec![], supplied_values: vec![supplied(pack.limit_id, 0.0, &pack.unit, "stress"),
                        supplied(pack.slot_id, 1.0, "ratio", "dimensionless")],
                        library_values: vec![], current_statuses: vec![AnalysisStatus::MechanicsSolved] }),
                ];
                for (name, input) in scenarios {
                    let result = run_under_test(&input);
                    writeln!(out, "{}\t{:016x}\t{}\tscenario_{}\t{}", pack.label, value_bits, row_unit, name,
                             serde_json::to_string(&result).unwrap()).unwrap();
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// I79 T3-SI1b: extreme values through the runner's point path.
// ---------------------------------------------------------------------------

thread_local! {
    static LAST_PANIC: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
}
static HOOK: std::sync::Once = std::sync::Once::new();

fn install_hook() {
    HOOK.call_once(|| {
        std::panic::set_hook(Box::new(|info| {
            let site = info
                .location()
                .map(|l| {
                    let file = std::path::Path::new(l.file()).file_name().unwrap().to_string_lossy().to_string();
                    format!("{file}:{}:{}", l.line(), l.column())
                })
                .unwrap_or_default();
            LAST_PANIC.with(|p| *p.borrow_mut() = site);
        }));
    });
}

fn guarded<F: FnOnce() -> RuleCheckRunResult>(f: F) -> String {
    install_hook();
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(result) => serde_json::to_string(&result).unwrap(),
        Err(_) => format!("PANIC\t{}", LAST_PANIC.with(|p| p.borrow().clone())),
    }
}

/// A pack whose formula is a table over a ratio argument `actual*1e300 -
/// actual*1e300` (NaN once `actual*1e300` overflows), compared against a
/// stress value-slot limit.
fn inline_table_pack(kind: &str) -> Value {
    let product = json!({ "node": "binary", "operator": "multiply",
        "left": { "node": "variable_ref", "variable_id": "actual" },
        "right": { "node": "literal", "quantity": { "value": 1e300, "dimension": "dimensionless", "unit_ref": "ratio" } } });
    let argument = json!({ "node": "binary", "operator": "subtract", "left": product.clone(), "right": product });
    let table = json!({ "table_id": "nan_table", "argument_dimension": "dimensionless", "argument_unit_ref": "ratio",
        "result_dimension": "stress", "result_unit_ref": "demo_unit",
        "rows": [ { "argument": 0.0, "result": 1.0 }, { "argument": 1.0, "result": 2.0 }, { "argument": 2.0, "result": 3.0 } ] });
    let ast = match kind {
        "interpolate" => json!({ "node": "interpolate", "table": table, "argument": argument }),
        mode => json!({ "node": "lookup", "mode": mode, "table": table, "argument": argument }),
    };
    json!({
        "grammar_version": "1.0.0",
        "metadata": { "rule_pack_id": format!("i79_{kind}_pack") },
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

/// The inline ratio pack plus a second, boolean check (`actual <= limit`), so
/// a run with a blocked first check still evaluates the second.
fn inline_two_check_pack() -> Value {
    let mut pack = inline_demo_pack();
    pack["formula_declarations"].as_array_mut().unwrap().push(json!({
        "formula_id": "predicate",
        "declaration_payload": { "expression_ast": {
            "node": "compare", "operator": "less_than_or_equal",
            "left": { "node": "variable_ref", "variable_id": "actual" },
            "right": { "node": "variable_ref", "variable_id": "limit" } } },
        "input_refs": [ { "ref_id": "actual", "ref_type": "required_input" },
                        { "ref_id": "limit", "ref_type": "required_input" } ]
    }));
    let mut second = pack["check_definitions"][0].clone();
    second["check_id"] = json!("predicate_check");
    second["formula_ref"]["ref_id"] = json!("predicate");
    second.as_object_mut().unwrap().remove("value_slot_refs");
    pack["check_definitions"].as_array_mut().unwrap().push(second);
    pack
}

#[test]
fn i79_runner_extreme_dump() {
    let out_path = std::env::var("I79_RUN_EXTREME_OUT").expect("I79_RUN_EXTREME_OUT");
    let mut out = std::io::BufWriter::new(fs::File::create(out_path).unwrap());
    // Caller-supplied values include non-finite ones (NaN, +inf, -inf).
    let actuals = [1e308, -1e308, f64::MAX, 1e300, 1e10, 150.0, 1.0, 0.5, 0.0, 5e-324, -1e-300, f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
    let limits = [1e-308, 5e-324, -5e-324, 1e-300, 100.0, 1e308, 0.0, f64::NAN, f64::INFINITY];
    let slots = [1.0, 1e308, f64::NAN, f64::INFINITY];
    let mut all = packs();
    all.push(Pack { label: "inline_two_check_pack".into(), document: inline_two_check_pack(), solver_id: "actual",
                    limit_id: "limit", slot_id: "ratio_limit", unit: "demo_unit".into() });
    for pack in &all {
        for &actual in &actuals {
            for &limit in &limits {
                for &slot in &slots {
                    let input = RuleCheckRunInput {
                        rule_pack_document: &pack.document,
                        solver_results: vec![SolverResultBinding { input_id: pack.solver_id.into(),
                            result_id: "r".into(), value: actual, unit: pack.unit.clone() }],
                        refused_solver_results: Vec::new(),
                        supplied_values: vec![supplied(pack.limit_id, limit, &pack.unit, "stress"),
                                              supplied(pack.slot_id, slot, "ratio", "dimensionless")],
                        library_values: Vec::new(),
                        current_statuses: vec![AnalysisStatus::MechanicsSolved],
                    };
                    let plain = guarded(|| run_rule_checks(&input));
                    let b0 = guarded(|| run_rule_checks_with_bounds(&input, &[SolverResultBound {
                        input_id: pack.solver_id.into(), absolute_bound: 0.0 }]));
                    writeln!(out, "{}\t{:016x}\t{:016x}\t{:016x}\tplain\t{}", pack.label, actual.to_bits(), limit.to_bits(), slot.to_bits(), plain).unwrap();
                    writeln!(out, "{}\t{:016x}\t{:016x}\t{:016x}\tb0\t{}", pack.label, actual.to_bits(), limit.to_bits(), slot.to_bits(), b0).unwrap();
                    for bound in [0.5, 1e-300, 1e300, f64::NAN, f64::INFINITY] {
                        let bounded = guarded(|| run_rule_checks_with_bounds(&input, &[SolverResultBound {
                            input_id: pack.solver_id.into(), absolute_bound: bound }]));
                        writeln!(out, "{}\t{:016x}\t{:016x}\t{:016x}\tb{:e}\t{}", pack.label, actual.to_bits(), limit.to_bits(), slot.to_bits(), bound, bounded).unwrap();
                    }
                }
            }
        }
    }
    for kind in ["interpolate", "step", "exact"] {
        let document = inline_table_pack(kind);
        for &actual in &actuals {
            for &slot in &[100.0, 1.0, f64::NAN] {
                let input = RuleCheckRunInput {
                    rule_pack_document: &document,
                    solver_results: vec![SolverResultBinding { input_id: "actual".into(), result_id: "r".into(),
                        value: actual, unit: "ratio".into() }],
                    refused_solver_results: Vec::new(),
                    supplied_values: vec![supplied("stress_limit", slot, "demo_unit", "stress")],
                    library_values: Vec::new(),
                    current_statuses: vec![AnalysisStatus::MechanicsSolved],
                };
                let plain = guarded(|| run_rule_checks(&input));
                let b0 = guarded(|| run_rule_checks_with_bounds(&input, &[SolverResultBound {
                    input_id: "actual".into(), absolute_bound: 0.0 }]));
                writeln!(out, "table_{kind}\t{:016x}\t-\t{:016x}\tplain\t{}", actual.to_bits(), slot.to_bits(), plain).unwrap();
                writeln!(out, "table_{kind}\t{:016x}\t-\t{:016x}\tb0\t{}", actual.to_bits(), slot.to_bits(), b0).unwrap();
                for bound in [0.5, 1e-300, 1e300] {
                    let bounded = guarded(|| run_rule_checks_with_bounds(&input, &[SolverResultBound {
                        input_id: "actual".into(), absolute_bound: bound }]));
                    writeln!(out, "table_{kind}\t{:016x}\t-\t{:016x}\tb{:e}\t{}", actual.to_bits(), slot.to_bits(), bound, bounded).unwrap();
                }
            }
        }
    }
}

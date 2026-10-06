//! I73 S-I1 control 1 (scratch harness, not repository content): dump the
//! serialized `RuleCheckRunResult` over the committed rule packs and the
//! numeric rows of the committed solved-result fixtures. Base and candidate
//! dumps must be byte-identical when no interval input is bound.

use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::AnalysisStatus;
#[allow(unused_imports)]
use open_pipe_stress_rule_check_runner::*;
use serde_json::{json, Value};

__RUN_UNDER_TEST__

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

//! Issue 441 (DEL-08-04): the export of a connector row of every kind in a
//! combination of every expression kind, through the product's own producer.
//!
//! The input is the T4-U5a J3-3 free pressurized chain (an untied joint with a
//! constant effective area, read from its frozen reference request) in two
//! invented load cases, with a linear combination, an absolute sum, an SRSS, a
//! result-state subtraction and a range envelope of them. The producer
//! publishes the connector's 26 rows in each (the elastic rows, `pe_x` and
//! `s_x`); the reader admits the envelope. The Rust derivative
//! (`derive_document`, `validate_document`) exports each combined connector
//! row as a quantity with the five metadata members the row carries, as the
//! reader classifies it. Expected values come from the combination records and
//! the table's names, never from the rule under test.
use open_pipe_stress_product_physics::result_export::{derivative, reader_verdict};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Map, Value};
use std::collections::BTreeSet;

#[path = "common/current_model.rs"]
mod current_model;

const J3_3: &str = include_str!("../../../validation/references/t4_u5a/requests/J3-3.request.json");
const BASE: &str = include_str!("../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json");
const PROV: &str = "invented issue-441 combined connector export input; not library, catalog, manufacturer or code-rule data";
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const METADATA: [&str; 5] = ["component", "coordinate_system", "location", "basis", "sign_convention"];

/// The metadata basis the pressure-3 table's vocabulary names for each
/// combination record's expression kind.
fn expression_basis(record_basis: &str) -> &'static str {
    match record_basis {
        "mechanics" => "explicit_user_linear_combination",
        "absolute_sum" => "explicit_user_absolute_sum",
        "srss" => "explicit_user_srss",
        "result_state_subtraction" => "explicit_user_result_state_subtraction",
        "range_envelope" => "explicit_user_range_envelope",
        other => panic!("unknown combination basis {other}"),
    }
}

fn request() -> Value {
    let mut wire = current_model::reauthor_request(serde_json::from_str(J3_3).unwrap());
    let model = &mut wire["model"];
    model["project"]["id"] = json!("project:invented-issue-441-combined-connector");
    // The second case: the same region at twice the pressure.
    let mut second = model["load_cases"][0].clone();
    second["id"] = json!("case:twice");
    let pressure = second["pressure_regions"][0]["pressure"]["value"].as_f64().unwrap();
    second["pressure_regions"][0]["pressure"]["value"] = json!(2.0 * pressure);
    second["provenance"] = json!(PROV);
    model["load_cases"].as_array_mut().unwrap().push(second);
    let first = model["load_cases"][0]["id"].as_str().unwrap().to_string();
    // Both pressurised cases are operating cases of the pressure-elongation setting.
    model["model_settings"]["pressure_elongation"]["operating_load_cases"] = json!([first, "case:twice"]);
    let terms = json!([{"load_case": first, "factor": 1.0}, {"load_case": "case:twice", "factor": -0.5}]);
    model["combinations"] = json!([
        {"id": "comb:linear", "basis": "mechanics", "terms": terms, "provenance": PROV},
        {"id": "comb:absolute", "basis": "absolute_sum", "terms": terms, "provenance": PROV},
        {"id": "comb:srss", "basis": "srss", "terms": terms, "provenance": PROV},
        {"id": "comb:difference", "basis": "result_state_subtraction", "minuend_id": "case:twice", "subtrahend_id": first, "provenance": PROV},
        {"id": "comb:envelope", "basis": "range_envelope", "operand_ids": [first, "case:twice"], "mode": "max_abs", "provenance": PROV},
    ]);
    wire
}

fn derive(source: &Value, wire: &Value) -> Result<Value, String> {
    let base: Value = serde_json::from_str(BASE).unwrap();
    let carrier = derivative::checksum(source, "attested_headless_producer_carrier", derivative::reference("test_carrier", "issue-441"))?;
    let origin = json!({"origin_id": "issue-441-combined-connector-export", "origin_class": "attested_headless_producer",
        "qualification_ref": derivative::reference("test_fixture", "not-authentication"), "authentic_producer_available": true,
        "received_carrier_checksum": carrier, "original_producer_checksum": carrier,
        "origin_limit": "In-memory producer output; consumer contract test only",
        "actual_model_ref": derivative::reference("model_payload", source["model_ref"].as_str().unwrap_or("")),
        "mechanics_run_ref": derivative::reference("mechanics_run", source["run_id"].as_str().unwrap_or("")),
        "request_model_ref": null, "request_run_ref": null, "request_alias_disclosure": null});
    derivative::derive_document(base, &wire["model"], source, origin, Some(wire))
}

#[test]
fn every_combined_connector_row_of_every_expression_is_an_exported_quantity_with_its_metadata() {
    let wire = request();
    for mode in MODES {
        let source: Value = serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap()).unwrap();
        assert_eq!(source["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
        for model in [None, Some(&wire["model"])] {
            let verdict = reader_verdict::validate("pressure", &source, model, None);
            assert_eq!(verdict["ok"], true, "{mode:?}: reader {verdict}");
        }
        let records = source["contract_evidence"]["exact_combinations"].as_array().unwrap();
        assert!(records.iter().all(|r| r["gate"]["withheld"] == false), "{mode:?}: no combination is withheld");
        let doc = derive(&source, &wire).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        derivative::validate_document(&doc, &source).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
        let mut rows = 0;
        for (index, row) in source["results"].as_array().unwrap().iter().enumerate() {
            if !row["kind"].as_str().unwrap().starts_with("connector_") || row["basis_ref"]["ref_type"] != "combination" {
                continue;
            }
            rows += 1;
            let record = records.iter().find(|r| r["combination_id"] == row["basis_ref"]["ref_id"]).unwrap();
            let basis = expression_basis(record["basis"].as_str().unwrap());
            assert_eq!(row["metadata"]["basis"], basis, "{mode:?}: {}", row["id"]);
            let account = &doc["result_envelope"]["row_accounting"][index];
            assert_eq!(account["source_result_id"], row["id"]);
            assert_eq!(account["disposition"], "exported_quantity", "{mode:?}: {}", row["id"]);
            let target = doc.pointer(account["target_field_path"].as_str().unwrap()).unwrap();
            let expected = Value::Object(METADATA.iter().map(|k| (k.to_string(), row["metadata"][*k].clone())).collect::<Map<_, _>>());
            assert_eq!(target["metadata"], expected, "{mode:?}: {}", row["id"]);
            seen.insert((row["kind"].as_str().unwrap().to_string(), basis.to_string()));
        }
        // 24 elastic rows, pe_x and s_x of the one connector, in each of the five combinations.
        assert_eq!(rows, 26 * 5, "{mode:?}");
        assert_eq!(seen.len(), 8 * 5, "{mode:?}: every connector kind in every expression kind: {seen:?}");
    }
}

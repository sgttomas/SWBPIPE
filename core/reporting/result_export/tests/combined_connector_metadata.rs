//! Issue 441 (DEL-08-04): a connector row of a combination leaves in the
//! derivative document as the ordinary reader classifies it. The engine gives
//! every non-stress combined row its expression's metadata basis and sign
//! convention and keeps the operand row's component, location and frame (the
//! pressure-3 table's `combination_policy.stress_metadata`), and the reader
//! requires exactly that. So the Rust derivative (`derive_document`,
//! `validate_document`) gives such a row its five metadata members: a combined
//! force or moment row is an exported quantity, not a
//! `physical_metadata_incomplete` disclosure, and a combined translation or
//! rotation row keeps its component and frame.
//!
//! Inputs are committed product outputs only: the shared combination reader
//! corpus (connector rows in linear combinations) and the exact-report witness
//! input of a connector in a result-state subtraction and a sum
//! (`core/product_physics/tests/exact_route_limitations.rs`). Expected values
//! come from the bound table and the combination records, never from the rule
//! under test.
use open_pipe_stress_result_export::{derivative as d, semantic_contract as s};
use serde_json::{json, Map, Value};
use std::collections::BTreeSet;

const CORPUS: &str = include_str!("../../../../fixtures/results/pressure_v3_combination_reader_corpus.json");
const SUBTRACTION: &str = include_str!(
    "../../../../fixtures/reports/invented/exact_report_witness_inputs/pressure_2_connector_combinations-sparse_interactive.raw.json"
);
const BASE: &str = include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json");
const METADATA: [&str; 5] = ["component", "coordinate_system", "location", "basis", "sign_convention"];

/// The metadata basis the table's vocabulary names for a combination record's
/// expression kind (`combination_policy`: every other combined row takes the
/// expression's basis).
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

fn is_combined_connector(row: &Value) -> bool {
    row["kind"].as_str().unwrap().starts_with("connector_") && row["basis_ref"]["ref_type"] == "combination"
}

/// Every committed source with a combined connector row, by name.
fn sources() -> Vec<(String, Value)> {
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    let mut out: Vec<(String, Value)> = corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["envelope"]["results"].as_array().unwrap().iter().any(is_combined_connector))
        .map(|case| (case["label"].as_str().unwrap().to_string(), case["envelope"].clone()))
        .collect();
    out.push(("connector subtraction and sum".to_string(), serde_json::from_str(SUBTRACTION).unwrap()));
    out
}

fn derive(source: &Value) -> Result<Value, String> {
    let carrier = d::checksum(source, "attested_headless_producer_carrier", d::reference("test_carrier", "issue-441"))?;
    let model = source["model_ref"].as_str().unwrap().to_owned();
    let origin = json!({"origin_id": "issue-441-combined-connector-test", "origin_class": "attested_headless_producer",
        "qualification_ref": d::reference("test_fixture", "not-authentication"), "authentic_producer_available": true,
        "received_carrier_checksum": carrier, "original_producer_checksum": carrier,
        "origin_limit": "Committed producer bytes; derivative contract test only",
        "actual_model_ref": d::reference("model_payload", &model),
        "mechanics_run_ref": d::reference("mechanics_run", source["run_id"].as_str().unwrap()),
        "request_model_ref": null, "request_run_ref": null, "request_alias_disclosure": null});
    d::derive_document(serde_json::from_str(BASE).unwrap(), &json!({"project": {"id": model}}), source, origin, None)
}

fn five_members(row: &Value) -> Value {
    Value::Object(METADATA.iter().map(|key| (key.to_string(), row["metadata"][*key].clone())).collect::<Map<_, _>>())
}

/// The quantity target of source row `index`, from the document's row accounting.
fn target<'a>(doc: &'a Value, index: usize, row: &Value) -> (&'a Value, &'a Value) {
    let account = &doc["result_envelope"]["row_accounting"][index];
    assert_eq!(account["source_result_id"], row["id"]);
    (account, doc.pointer(account["target_field_path"].as_str().unwrap()).unwrap())
}

#[test]
fn combined_connector_rows_are_exported_quantities_with_their_five_metadata_members() {
    let table = s::pressure_rows_contract();
    let vocabulary: BTreeSet<&str> =
        table["canonical_metadata_vocabulary"]["basis"]["enum"].as_array().unwrap().iter().filter_map(Value::as_str).collect();
    let (mut rows, mut bases, mut kinds) = (0, BTreeSet::new(), BTreeSet::new());
    for (name, source) in sources() {
        let doc = derive(&source).unwrap_or_else(|e| panic!("{name}: {e}"));
        d::validate_document(&doc, &source).unwrap_or_else(|e| panic!("{name}: {e}"));
        let records = source["contract_evidence"]["exact_combinations"].as_array().unwrap();
        for (index, row) in source["results"].as_array().unwrap().iter().enumerate() {
            if !is_combined_connector(row) {
                continue;
            }
            rows += 1;
            let (account, quantity) = target(&doc, index, row);
            assert_eq!(account["disposition"], "exported_quantity", "{name}: {}", row["id"]);
            assert_eq!(quantity["metadata"], five_members(row), "{name}: {}", row["id"]);
            // The basis is the one the row's own combination's expression gives, in the table's vocabulary.
            let record = records.iter().find(|r| r["combination_id"] == row["basis_ref"]["ref_id"]).unwrap();
            let basis = expression_basis(record["basis"].as_str().unwrap());
            assert_eq!(row["metadata"]["basis"], basis, "{name}: {}", row["id"]);
            assert!(vocabulary.contains(basis));
            bases.insert(basis);
            kinds.insert(row["kind"].as_str().unwrap().to_string());
        }
    }
    // 73 rows in the corpus's linear combinations, 24 each in the witness's subtraction and sum.
    assert_eq!(rows, 73 + 48);
    assert_eq!(bases, BTreeSet::from(["explicit_user_linear_combination", "explicit_user_result_state_subtraction"]));
    assert_eq!(kinds.len(), 7, "{kinds:?}");
}

/// A case connector row keeps the connector row basis requirement: its
/// document is unchanged in kind (an exported quantity with its five members).
#[test]
fn case_connector_rows_keep_the_connector_row_basis() {
    let (name, source) = sources().pop().unwrap();
    let doc = derive(&source).unwrap();
    let mut seen = 0;
    for (index, row) in source["results"].as_array().unwrap().iter().enumerate() {
        if row["kind"].as_str().unwrap().starts_with("connector_") && row["basis_ref"]["ref_type"] == "load_case" {
            assert_eq!(row["metadata"]["basis"], s::CONNECTOR_ROW_BASIS, "{name}");
            let (account, quantity) = target(&doc, index, row);
            assert_eq!(account["disposition"], "exported_quantity");
            assert_eq!(quantity["metadata"], five_members(row));
            seen += 1;
        }
    }
    assert_eq!(seen, 48);
}

/// The rule gives no metadata to a connector row whose basis is not one its
/// row type allows, or whose component, location or frame is outside its
/// kind's lists. Each case changes one member of a row the product wrote.
#[test]
fn the_connector_rule_gives_no_metadata_outside_its_conditions() {
    let table = s::pressure_rows_contract();
    let (_, source) = sources().pop().unwrap();
    let rows = source["results"].as_array().unwrap();
    let local = |row: &&Value| row["metadata"]["location"] == "connector_local";
    let combined = rows.iter().filter(|r| is_combined_connector(r) && r["kind"] == "connector_generalized_force_v1").find(local).unwrap();
    let case_row = rows.iter().filter(|r| r["basis_ref"]["ref_type"] == "load_case" && r["kind"] == "connector_generalized_force_v1").find(local).unwrap();
    assert_eq!(s::canonical_metadata_for(&source, table, combined), Some(five_members(combined)));
    assert_eq!(s::canonical_metadata_for(&source, table, case_row), Some(five_members(case_row)));
    let changed = |row: &Value, pointer: &str, value: Value| {
        let mut row = row.clone();
        *row.pointer_mut(pointer).unwrap() = value;
        row
    };
    let refused = [
        ("a combined row with the case basis", changed(combined, "/metadata/basis", json!(s::CONNECTOR_ROW_BASIS))),
        ("a combined row with a straight-member basis", changed(combined, "/metadata/basis", json!("recovered_from_local_element_stiffness"))),
        ("a combined row with the tie-rod basis", changed(combined, "/metadata/basis", json!("recovered_from_tie_rod_extension_v1"))),
        ("a case row with a combination basis", changed(case_row, "/metadata/basis", json!("explicit_user_result_state_subtraction"))),
        ("a combined row with another kind's component", changed(combined, "/metadata/component", json!("Fx"))),
        ("a combined row at an end location", changed(combined, "/metadata/location", json!("end_i"))),
        ("a combined row in the element frame", changed(combined, "/metadata/coordinate_system", json!("element_local"))),
        ("a combined row without a sign convention", changed(combined, "/metadata/sign_convention", json!(""))),
        ("a combined tie-rod row", {
            let rod = changed(combined, "/kind", json!("connector_tie_rod_force_v1"));
            let rod = changed(&rod, "/metadata/component", json!("t_r"));
            let rod = changed(&rod, "/metadata/location", json!("tie_rod"));
            changed(&rod, "/metadata/coordinate_system", json!("element_local"))
        }),
    ];
    for (case, row) in refused {
        assert_eq!(s::canonical_metadata_for(&source, table, &row), None, "{case}");
    }
}

/// A combined connector row whose basis is another expression's is not the
/// row the engine writes: the reader refuses the source before any metadata
/// is given, on both the derive and the validate path.
#[test]
fn a_combined_connector_row_with_another_expressions_basis_is_refused() {
    let (_, source) = sources().pop().unwrap();
    let doc = derive(&source).unwrap();
    let mut forged = source.clone();
    let row = forged["results"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| is_combined_connector(r) && r["basis_ref"]["ref_id"] == "comb:1-minus-2")
        .unwrap();
    assert_eq!(row["metadata"]["basis"], "explicit_user_result_state_subtraction");
    row["metadata"]["basis"] = json!("explicit_user_linear_combination");
    let derived = derive(&forged).unwrap_err();
    assert_eq!(d::validate_document(&doc, &forged).unwrap_err(), derived);
    assert_eq!(derived, "SOURCE_PHYSICS_COMBINATION_ROW");
}

/// The row-naming entry the native package assembly uses: a refusal raised
/// while one source row is checked names its index and id; `validate_document`
/// keeps its code alone.
#[test]
fn a_row_refusal_names_the_source_row() {
    let (_, source) = sources().pop().unwrap();
    let mut doc = derive(&source).unwrap();
    let (index, row) = source["results"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, r)| is_combined_connector(r) && r["kind"] == "connector_generalized_translation_v1")
        .unwrap();
    let path = doc["result_envelope"]["row_accounting"][index]["target_field_path"].as_str().unwrap().to_string();
    doc.pointer_mut(&path).unwrap().as_object_mut().unwrap().remove("metadata").unwrap();
    assert_eq!(d::validate_document(&doc, &source).unwrap_err(), "PHYSICAL_SEMANTICS_MISMATCH");
    assert_eq!(
        d::validate_document_naming_row(&doc, &source).unwrap_err(),
        format!("PHYSICAL_SEMANTICS_MISMATCH: source row {index} {}", row["id"].as_str().unwrap())
    );
    // A refusal after the row checks names no row.
    let mut stale = derive(&source).unwrap();
    stale["result_envelope"]["reproducibility"]["derivative_hash"]["value"] = json!("0".repeat(64));
    assert_eq!(d::validate_document(&stale, &source).unwrap_err(), "DERIVATIVE_HASH_MISMATCH");
    assert_eq!(d::validate_document_naming_row(&stale, &source).unwrap_err(), "DERIVATIVE_HASH_MISMATCH");
    // An unchanged document passes both.
    let clean = derive(&source).unwrap();
    assert_eq!(d::validate_document_naming_row(&clean, &source), Ok(()));
}

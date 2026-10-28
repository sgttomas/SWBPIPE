//! Run-sheet entry S1 (DEL-07-02 Design `run-sheet-entry.md` §7): the native
//! lane of native/wasm equivalence (DEL-16-06 CLM-003). The hand-built batches of
//! `fixtures/model_operations/run_sheet_authoring.json` are applied natively to
//! the fixture's basis model and must give its expected applied models, the same
//! documents the desktop's wasm engine gives in
//! `apps/desktop/src/features/model-tree/runSheet/runSheet.test.ts`. All data are
//! invented; no section, material, radius or k is library, catalog or code data.
use open_pipe_stress_operation_applier::{apply_operation_batch, canonical_json, sha256_hex, validate_operation_batch};
use open_pipe_stress_units::{convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{json, Value};

const FIXTURE: &str = include_str!("../../../../fixtures/model_operations/run_sheet_authoring.json");

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("fixture is JSON")
}

fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}

fn case<'a>(fixture: &'a Value, id: &str) -> &'a Value {
    fixture["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap_or_else(|| panic!("case {id}"))
}

fn apply(model: &Value, batch: &Value) -> Value {
    apply_operation_batch(model, batch, Some(&hash(model)))
}

fn blocking_codes(step: &Value) -> Vec<String> {
    step["diagnostics"].as_array().unwrap().iter()
        .filter(|d| d["severity"] == "blocking")
        .map(|d| d["code"].as_str().unwrap().to_string())
        .collect()
}

#[test]
fn every_batch_applies_natively_to_its_expected_model() {
    let fixture = fixture();
    let basis = &fixture["basis_model"];
    let mut applied_cases = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let expected = &case["expected_applied_model"];
        if expected.is_null() {
            continue;
        }
        let batch = &case["expected_batch"];
        let validated = validate_operation_batch(basis, batch, Some(&hash(basis)));
        assert_eq!(validated["validation"]["batch_validation_status"], "passed", "{id}: {:#}", validated["diagnostics"]);
        assert!(validated["applied_model"].is_null(), "{id}: validation returns no model");
        let outcome = apply(basis, batch);
        assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{id}: {:#}", outcome["diagnostics"]);
        assert_eq!(outcome["simulation_disposition"], "committed_as_one_batch", "{id}");
        // Equality of canonical texts: the same JSON value, numbers by value.
        let applied = canonical_json(&outcome["applied_model"]);
        assert_eq!(applied, canonical_json(expected), "{id}");
        // The wasm lane asserts the same digest of its applied model.
        assert_eq!(json!(format!("sha256:{}", sha256_hex(&applied))), case["expected_applied_model_sha256"], "{id}");
        applied_cases += 1;
    }
    assert_eq!(applied_cases, 11);
}

/// Every coordinate the batch creates is stored with the exact bits the
/// builder emitted (serde_json `float_roundtrip`; `create_node` in the stored unit).
#[test]
fn created_coordinates_keep_the_emitted_bits() {
    let fixture = fixture();
    let basis = &fixture["basis_model"];
    for case in fixture["cases"].as_array().unwrap() {
        if case["expected_applied_model"].is_null() {
            continue;
        }
        let outcome = apply(basis, &case["expected_batch"]);
        let nodes = outcome["applied_model"]["nodes"].as_array().unwrap();
        for op in case["expected_batch"]["operations"].as_array().unwrap() {
            if op["change"]["change_kind"] != "create_node" {
                continue;
            }
            assert_eq!(op["change"]["unit"], "m");
            let emitted: Value = serde_json::from_str(op["change"]["after"].as_str().unwrap()).unwrap();
            let Some(stored) = nodes.iter().find(|n| n["id"] == emitted["id"]) else {
                continue; // a corner a later bend replaced
            };
            for axis in ["x", "y", "z"] {
                let (a, b) = (emitted["position"][axis].as_f64().unwrap(), stored["position"][axis].as_f64().unwrap());
                assert_eq!(a.to_bits(), b.to_bits(), "{} {}.{axis}", case["id"], emitted["id"]);
                assert!(!(b == 0.0 && b.is_sign_negative()), "no stored -0");
            }
        }
    }
}

/// §6.2: 0.1 m + 0.3 in is convert_for_dimension(0.3, in → m) plus one addition,
/// 0.10762000000000001 m, not the absolute-entry value 0.10762000000000002 m.
#[test]
fn non_identity_increment_is_converted_then_added() {
    let fixture = fixture();
    let case = case(&fixture, "inch-increments");
    let (inch, metre) = (unit_by_symbol("in", Dimension::Length).unwrap(), unit_by_symbol("m", Dimension::Length).unwrap());
    let converted_then_added = 0.1 + convert_for_dimension(0.3, Dimension::Length, inch, metre).unwrap();
    let absolute = convert_for_dimension(convert_for_dimension(0.1, Dimension::Length, metre, inch).unwrap() + 0.3, Dimension::Length, inch, metre).unwrap();
    assert_eq!(converted_then_added, 0.10762000000000001);
    assert_eq!(absolute, 0.10762000000000002);
    assert_ne!(converted_then_added.to_bits(), absolute.to_bits());
    assert_eq!(case["arithmetic"]["converted_then_added_m"].as_f64().unwrap().to_bits(), converted_then_added.to_bits());
    let outcome = apply(&fixture["basis_model"], &case["expected_batch"]);
    let node = outcome["applied_model"]["nodes"].as_array().unwrap().iter().find(|n| n["id"] == "node:N20").unwrap();
    assert_eq!(node["position"]["x"].as_f64().unwrap().to_bits(), converted_then_added.to_bits());
    let next = outcome["applied_model"]["nodes"].as_array().unwrap().iter().find(|n| n["id"] == "node:N30").unwrap();
    let y = 10.0 + convert_for_dimension(12.0, Dimension::Length, inch, metre).unwrap();
    assert_eq!(next["position"]["y"].as_f64().unwrap().to_bits(), y.to_bits());
}

/// A refused last step publishes nothing (`rolled_back_no_model_published`).
#[test]
fn refused_last_step_rolls_back() {
    let fixture = fixture();
    let case = case(&fixture, "refused-last-step");
    let expected = &case["expected_refusal"];
    let basis = &fixture["basis_model"];
    let operations = case["expected_batch"]["operations"].as_array().unwrap();
    for outcome in [validate_operation_batch(basis, &case["expected_batch"], Some(&hash(basis))), apply(basis, &case["expected_batch"])] {
        assert!(outcome["applied_model"].is_null());
        assert_eq!(outcome["simulation_disposition"], expected["simulation_disposition"]);
        let steps = outcome["operation_outcomes"].as_array().unwrap();
        assert_eq!(steps.len(), operations.len());
        let index = expected["step_index"].as_u64().unwrap() as usize;
        assert_eq!(index, operations.len() - 1);
        assert_eq!(steps[index]["simulation_status"], "blocked");
        assert_eq!(blocking_codes(&steps[index]), vec![expected["code"].as_str().unwrap().to_string()]);
        assert_eq!(steps[index]["operation_id"], format!("op:run-sheet:{}:2", expected["row"].as_str().unwrap()));
        assert!(steps[..index].iter().all(|s| s["simulation_status"] == "validated_on_temporary_state"));
    }
}

/// A projection that missed an earlier bend's retargeting names a stale corner
/// context; the engine refuses it.
#[test]
fn corrupted_projection_is_refused_as_a_before_mismatch() {
    let fixture = fixture();
    let mut batch = case(&fixture, "two-bend-chain")["expected_batch"].clone();
    let operations = batch["operations"].as_array_mut().unwrap();
    let index = operations.iter().rposition(|op| op["change"]["change_kind"] == "insert_bend_at_corner").unwrap();
    let before = operations[index]["change"]["before"].as_str().unwrap().to_string();
    let corrupted = before.replace("\"from\":\"node:N20-T2\"", "\"from\":\"node:N20\"");
    assert_ne!(corrupted, before);
    operations[index]["change"]["before"] = json!(corrupted);
    let outcome = apply(&fixture["basis_model"], &batch);
    assert!(outcome["applied_model"].is_null());
    assert_eq!(outcome["simulation_disposition"], "rolled_back_no_model_published");
    let steps = outcome["operation_outcomes"].as_array().unwrap();
    assert_eq!(steps.len(), index + 1);
    assert!(blocking_codes(&steps[index]).contains(&"OP-BEFORE-VALUE-MISMATCH".to_string()));
}

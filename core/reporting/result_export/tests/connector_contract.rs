//! T4-U3 (S14): the pressure-1 reader admits objective connector records and
//! rows over the shared connector corpus (also read by the Python and
//! TypeScript readers), refuses each broken binding by name, and every other
//! contract refuses connector evidence (`CONNECTOR_UNSUPPORTED`).
use open_pipe_stress_result_export::semantic_contract as s;
use serde_json::{json, Value};

fn corpus() -> Vec<(String, Value)> {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/pressure_v3_connector_reader_corpus.json"
    ))
    .unwrap();
    corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| (case["label"].as_str().unwrap().to_string(), case["envelope"].clone()))
        .collect()
}

fn load_state(envelope: &Value) -> bool {
    envelope["contract_evidence"].get("load_reference_states").is_some()
}

fn rows_mut(envelope: &mut Value) -> &mut Vec<Value> {
    envelope["results"].as_array_mut().unwrap()
}

fn first_connector_row(envelope: &mut Value) -> &mut Value {
    rows_mut(envelope)
        .iter_mut()
        .find(|r| r["kind"].as_str().unwrap().starts_with("connector_"))
        .unwrap()
}

fn record(envelope: &mut Value) -> &mut Value {
    &mut envelope["contract_evidence"]["connector"][0]
}

/// (label, mutation, the pressure-1 reader's error suffix).
#[allow(clippy::type_complexity)]
fn mutations() -> Vec<(&'static str, Box<dyn Fn(&mut Value)>, &'static str)> {
    vec![
        ("record removed", Box::new(|e: &mut Value| e["contract_evidence"]["connector"] = json!([])), "CONNECTOR_ROW_UNBOUND"),
        ("record duplicated", Box::new(|e: &mut Value| {
            let r = e["contract_evidence"]["connector"][0].clone();
            e["contract_evidence"]["connector"].as_array_mut().unwrap().push(r);
        }), "CONNECTOR_RECORD_DUPLICATE"),
        ("record extra key", Box::new(|e: &mut Value| record(e)["extra"] = json!(1)), "CONNECTOR_RECORD_SHAPE"),
        ("record hardware tied", Box::new(|e: &mut Value| record(e)["hardware"] = json!("tied")), "CONNECTOR_RECORD_LAW"),
        ("record q_ref short", Box::new(|e: &mut Value| record(e)["q_ref"] = json!([0.0, 0.0])), "CONNECTOR_RECORD_FRAME"),
        ("record rotation scale", Box::new(|e: &mut Value| record(e)["work_matrix"]["rotation_scale_rad"] = json!(2.0)), "CONNECTOR_WORK_MATRIX"),
        ("replaced span published", Box::new(|e: &mut Value| record(e)["replaced_pipe_id"] = json!("pipe:P-120")), "CONNECTOR_REPLACED_SPAN_PUBLISHED"),
        ("row removed", Box::new(|e: &mut Value| {
            let index = rows_mut(e).iter().position(|r| r["kind"].as_str().unwrap().starts_with("connector_")).unwrap();
            rows_mut(e).remove(index);
        }), "CONNECTOR_ROW_COVERAGE"),
        ("row unbound", Box::new(|e: &mut Value| first_connector_row(e)["entity_ref"] = json!("component:C-999")), "CONNECTOR_ROW_UNBOUND"),
        ("row unknown kind", Box::new(|e: &mut Value| first_connector_row(e)["kind"] = json!("connector_energy_v1")), "CONNECTOR_ROW_KIND"),
        ("row unit", Box::new(|e: &mut Value| first_connector_row(e)["unit"] = json!("mm")), "CONNECTOR_ROW_SEMANTICS"),
        ("row basis", Box::new(|e: &mut Value| first_connector_row(e)["metadata"]["basis"] = json!("objective_connector_v1;replaces_span=pipe:P-120;symmetric_midpoint_small_rotation_v1")), "CONNECTOR_ROW_SEMANTICS"),
        ("row sign", Box::new(|e: &mut Value| first_connector_row(e)["metadata"]["sign_convention"] = json!("global end action of the connector on its node (node on element), f = B^T g")), "CONNECTOR_ROW_SEMANTICS"),
        ("row duplicated", Box::new(|e: &mut Value| {
            let mut r = first_connector_row(e).clone();
            r["id"] = json!("result:connector:duplicate");
            rows_mut(e).push(r);
        }), "CONNECTOR_ROW_DUPLICATE"),
        ("replaced span row", Box::new(|e: &mut Value| {
            let mut r = rows_mut(e).iter().find(|r| r["kind"] == "element_local_axial_force").unwrap().clone();
            r["id"] = json!("result:replaced-span");
            r["entity_ref"] = json!("pipe:P-130");
            rows_mut(e).push(r);
        }), "CONNECTOR_REPLACED_SPAN_PUBLISHED"),
    ]
}

#[test]
fn the_connector_corpus_is_admitted_by_the_pressure_1_reader_and_dispatch() {
    let cases = corpus();
    assert_eq!(cases.len(), 2);
    assert!(cases.iter().any(|(_, e)| load_state(e)) && cases.iter().any(|(_, e)| !load_state(e)));
    for (label, envelope) in &cases {
        assert_eq!(envelope["producer"]["semantic_contract_id"], s::PRESSURE_ID, "{label}");
        assert_eq!(envelope["contract_evidence"]["connector"].as_array().unwrap().len(), 1, "{label}");
        s::validate_pressure_evidence(envelope).unwrap_or_else(|e| panic!("{label}: {e}"));
        s::for_source(envelope).unwrap_or_else(|e| panic!("{label}: {e}"));
    }
}

#[test]
fn broken_connector_bindings_are_refused_by_name() {
    for (label, envelope) in corpus() {
        for (what, mutate, code) in mutations() {
            let mut broken = envelope.clone();
            mutate(&mut broken);
            let error = s::validate_pressure_evidence(&broken).unwrap_err();
            assert!(error.ends_with(code), "{label} {what}: {error}");
        }
    }
}

/// Connector records or rows offered under physics-1 / load-reference-1
/// (v2) are refused: the records by the namespace check, the rows on their own.
#[test]
fn every_other_contract_refuses_connector_evidence() {
    for (label, envelope) in corpus() {
        let mut v2 = envelope.clone();
        for case in v2["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
            case["profile_mode"] = json!("exact_straight_pressure_v2");
        }
        let read = |e: &Value| {
            if load_state(e) { s::validate_load_reference_evidence(e) } else { s::validate_physics_evidence(e) }
        };
        let error = read(&v2).unwrap_err();
        assert!(error.ends_with("CONNECTOR_UNSUPPORTED"), "{label}: {error}");
        // Rows alone, with no record, are refused too.
        v2["contract_evidence"]["connector"] = json!([]);
        let error = read(&v2).unwrap_err();
        assert!(error.ends_with("CONNECTOR_UNSUPPORTED"), "{label}: {error}");
    }
}

//! Unit R: the semantic binding consumes the accepted reader's verdict wire and
//! returns the existing Rust helpers' outcomes. Corrupt verdicts fail closed.
use open_pipe_stress_operation_applier::retained_precision_semantics_json;
use open_pipe_stress_product_physics::result_export::{retained_precision as rp, semantic_contract as sc};
use serde_json::{json, Value};

const SPARSE: &str = include_str!("../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json");
const DENSE: &str = include_str!("../../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json");
fn read(source: &Value, wire: &Value, requested: &Value) -> Result<Value, String> {
    retained_precision_semantics_json(&source.to_string(), &wire.to_string(), &requested.to_string())
        .map(|text| serde_json::from_str(&text).unwrap())
}
#[test]
fn accepted_verdicts_return_rust_standing_and_classification_in_every_context() {
    for text in [SPARSE, DENSE] {
        let doc: Value = serde_json::from_str(text).unwrap();
        let source = &doc["source"];
        let validation = rp::validate(source, Some(&doc["invocation"])).unwrap();
        let requested = json!([{"ref_type":"load_case","ref_id":"case"}]);
        for refs in [requested, json!([]), json!([{"ref_type":"load_case","ref_id":"other"}])] {
            for variant in 0..4 {
                let mut v = validation.clone();
                match variant {
                    1 => v.invocation_bound = false,
                    2 => v.numerical_eligible = false,
                    3 => {
                        let row = v.classifications.iter_mut().find(|r| r.class == rp::AccuracyClass::RelativeVerified).unwrap();
                        row.class = rp::AccuracyClass::NotCovered;
                    }
                    _ => {}
                }
                let wire = rp::outcome_json(&Ok(v.clone()))["validation"].clone();
                assert_eq!(read(source, &wire, &refs).unwrap(), json!({
                    "standing": sc::retained_standing_from(&v, source, refs.as_array().unwrap()),
                    "classification_summary": sc::classification_summary_from(&v, source, refs.as_array().unwrap()),
                }));
            }
        }
    }
}
#[test]
fn malformed_verdicts_and_refs_never_supply_standing() {
    let doc: Value = serde_json::from_str(SPARSE).unwrap();
    let v = rp::validate(&doc["source"], Some(&doc["invocation"])).unwrap();
    let wire = rp::outcome_json(&Ok(v))["validation"].clone();
    for (pointer, value) in [
        ("/invocation_bound", json!("true")),
        ("/numerical_eligible", Value::Null),
        ("/publication_sha256", json!("broken")),
        ("/classifications/0/class", json!("unknown")),
        ("/classifications/0/normalized_bits", json!("0000")),
        ("/classifications/0/normalized_bits", json!("7ff8000000000000")),
        ("/classifications/0/scale_bits", json!(123)),
    ] {
        let mut bad = wire.clone();
        *bad.pointer_mut(pointer).unwrap() = value;
        assert!(read(&doc["source"], &bad, &json!([])).unwrap_err().starts_with("WASM-ENGINE-INPUT-VALIDATION-INVALID"), "{pointer}");
    }
    let mut bad = wire.clone();
    let absolute = bad["classifications"].as_array_mut().unwrap().iter_mut().find(|r| r["class"] == "absolute_verified").unwrap();
    absolute["bound_bits"] = Value::Null;
    assert!(read(&doc["source"], &bad, &json!([])).is_err());
    assert!(read(&doc["source"], &wire, &json!({})).is_err());
    assert!(retained_precision_semantics_json("{", "{}", "[]").unwrap_err().starts_with("WASM-ENGINE-INPUT-JSON-INVALID"));
}

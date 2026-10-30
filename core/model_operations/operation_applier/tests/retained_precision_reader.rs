//! R1 (b7-plan Unit R): the wasm engine's retained-precision reader entry points, run
//! natively. They parse the JSON text and return the result reader's own outcome; the
//! desktop's wasm build compiles these same functions (`wasm_api`).
use open_pipe_stress_operation_applier::{
    retained_precision_outcome_json, retained_precision_transport_outcome_json,
};
use open_pipe_stress_product_physics::result_export::retained_precision as rp;
use serde_json::Value;

const MILESTONE: &str = include_str!(
    "../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"
);

#[test]
fn the_entry_points_return_the_readers_outcome() {
    let doc: Value = serde_json::from_str(MILESTONE).unwrap();
    let (source, invocation) = (doc["source"].to_string(), doc["invocation"].to_string());
    let outcome = |text: String| serde_json::from_str::<Value>(&text).unwrap();
    let bound = outcome(retained_precision_outcome_json(&source, Some(&invocation)).unwrap());
    assert_eq!(bound, rp::outcome_json(&rp::validate(&doc["source"], Some(&doc["invocation"]))));
    assert_eq!((bound["ok"].clone(), bound["validation"]["numerical_eligible"].clone()), (true.into(), true.into()));
    let unbound = outcome(retained_precision_outcome_json(&source, None).unwrap());
    assert_eq!(unbound, rp::outcome_json(&rp::validate(&doc["source"], None)));
    assert_eq!(unbound["validation"]["invocation_bound"], false);
    // The text `null` reads unbound, as an absent invocation (and the runner's null).
    let null = outcome(retained_precision_outcome_json(&source, Some("null")).unwrap());
    assert_eq!(null, unbound);
    let transport = outcome(retained_precision_transport_outcome_json(&source).unwrap());
    assert_eq!(transport, rp::outcome_json(&rp::validate_transport_metadata(&doc["source"])));
    assert_eq!(transport["validation"]["classifications"], serde_json::json!([]));
    // A refusal carries the reader's gate, code and detail.
    let mut edited = doc["source"].clone();
    edited["results"][0]["value"] = 12345.0.into();
    let refused = outcome(retained_precision_outcome_json(&edited.to_string(), Some(&invocation)).unwrap());
    assert_eq!(refused, serde_json::json!({"ok": false, "error": {"gate": "G1", "code": "RETAINED_PRECISION_RECEIPT_MISMATCH", "detail": null}}));
}

#[test]
fn text_that_is_not_json_is_an_input_error() {
    for (source, invocation, label) in [("{", None, "source"), ("{}", Some("[1,"), "invocation")] {
        let error = retained_precision_outcome_json(source, invocation).unwrap_err();
        assert!(error.starts_with(&format!("WASM-ENGINE-INPUT-JSON-INVALID: {label} is not valid JSON")), "{error}");
    }
    assert!(retained_precision_transport_outcome_json("nul").unwrap_err().starts_with("WASM-ENGINE-INPUT-JSON-INVALID"));
}

//! R2 (b7-plan Unit R): the wasm engine's base-reader entry points, run natively.
//! They parse the JSON text and return `result_export::reader_verdict`'s outcome;
//! the desktop's wasm build compiles these same functions (`wasm_api`).
use open_pipe_stress_operation_applier::{
    reader_dispatch_json, reader_row_semantics_json, reader_transport_json, reader_validate_json,
};
use open_pipe_stress_product_physics::result_export::reader_verdict as rv;
use serde_json::{json, Value};

// B4/D1 (b): the composite physics-source-1 statement is the source route here.
const SOURCE: &str = include_str!("../../../../fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json");
const REQUEST: &str = include_str!("../../../../fixtures/product_preview/physics_source/n05.request.json");
const LOAD_REFERENCE: &str = include_str!("../../../../fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json");
const LOAD_REFERENCE_REQUEST: &str = include_str!("../../../../fixtures/product_preview/load_reference/connected.request.json");

fn outcome(text: String) -> Value {
    serde_json::from_str(&text).unwrap()
}

#[test]
fn the_entry_points_return_the_verdicts() {
    let source: Value = serde_json::from_str(SOURCE).unwrap();
    let request: Value = serde_json::from_str(REQUEST).unwrap();
    let invocation = json!({"request": request, "solver_mode": "sparse_interactive"});
    let (model_text, invocation_text) = (request["model"].to_string(), invocation.to_string());
    let bound = outcome(reader_dispatch_json(SOURCE, Some(&model_text), Some(&invocation_text)).unwrap());
    assert_eq!(bound, rv::dispatch(&source, Some(&request["model"]), Some(&invocation)));
    assert_eq!((bound["route"].as_str(), bound["standing"]["eligible"].as_bool()), (Some("physics_source"), Some(true)));
    // An absent model or invocation, or the text `null`, reads as absent.
    let unbound = outcome(reader_dispatch_json(SOURCE, None, None).unwrap());
    assert_eq!(unbound, rv::dispatch(&source, None, None));
    assert_eq!(outcome(reader_dispatch_json(SOURCE, Some("null"), Some("null")).unwrap()), unbound);
    assert_eq!(unbound["standing"]["findings"], json!([rv::SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED]));
    let validated = outcome(reader_validate_json("physics-source", SOURCE, None, Some(&invocation_text)).unwrap());
    assert_eq!(validated, json!({"ok": true, "eligible": true, "findings": []}));
    assert_eq!(outcome(reader_validate_json("source-recovery", SOURCE, None, None).unwrap()), json!({"ok": true}));
    assert_eq!(outcome(reader_transport_json("physics-source", SOURCE).unwrap()), json!({"ok": true}));
    let mut bad = source.clone();
    bad["source_block_recovery"]["receipt_sha256"] = json!("0".repeat(64));
    assert_eq!(
        outcome(reader_transport_json("physics-source", &bad.to_string()).unwrap()),
        rv::transport("physics-source", &bad)
    );
    assert_eq!(outcome(reader_transport_json("physics-source", &bad.to_string()).unwrap())["ok"], false);
    // The retired source-blocks readers have no entry.
    assert_eq!(outcome(reader_transport_json("source-blocks", SOURCE).unwrap())["error"]["code"], "READER_UNKNOWN");
    // The model reaches the load-reference reader (G2).
    let lr_model: Value = serde_json::from_str(LOAD_REFERENCE_REQUEST).unwrap();
    let mut fewer = lr_model["model"].clone();
    fewer["load_cases"].as_array_mut().unwrap().remove(0);
    let refused = outcome(reader_validate_json("load-reference", LOAD_REFERENCE, Some(&fewer.to_string()), None).unwrap());
    assert_eq!(refused["error"]["code"], "SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE");
    assert_eq!(outcome(reader_validate_json("load-reference", LOAD_REFERENCE, None, None).unwrap()), json!({"ok": true}));
    assert_eq!(outcome(reader_transport_json("load-reference", LOAD_REFERENCE).unwrap()), json!({"ok": true}));
    assert_eq!(
        outcome(reader_transport_json("invented", LOAD_REFERENCE).unwrap())["error"],
        json!({"code": "READER_UNKNOWN", "detail": "READER_UNKNOWN: invented"})
    );
    let row = source["results"][0].to_string();
    assert_eq!(
        outcome(reader_row_semantics_json(&row, Some(SOURCE)).unwrap()),
        rv::row_semantics(&source["results"][0], Some(&source))
    );
    assert_eq!(
        outcome(reader_row_semantics_json(&row, Some("null")).unwrap()),
        rv::row_semantics(&source["results"][0], None)
    );
}

#[test]
fn text_that_is_not_json_is_an_input_error() {
    let invalid = |error: String, label: &str| {
        assert!(error.starts_with(&format!("WASM-ENGINE-INPUT-JSON-INVALID: {label} is not valid JSON")), "{error}")
    };
    invalid(reader_dispatch_json("{", None, None).unwrap_err(), "source");
    invalid(reader_dispatch_json("{}", Some("[1,"), None).unwrap_err(), "model");
    invalid(reader_dispatch_json("{}", None, Some("[1,")).unwrap_err(), "invocation");
    invalid(reader_validate_json("physics", "nul", None, None).unwrap_err(), "source");
    invalid(reader_transport_json("physics", "nul").unwrap_err(), "source");
    invalid(reader_row_semantics_json("{", None).unwrap_err(), "row");
}

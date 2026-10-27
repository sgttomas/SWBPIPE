//! Model document 0.8.0's `model_settings.shear_model` (DEL-02-01 Design
//! `model-document-0.8.0.md` §1, §4; Agent 0's decision under MD8 §11), at
//! solve, through the public value and typed entries in both solver modes:
//! - `euler_bernoulli` solves exactly as absent (the envelope is byte-identical);
//! - the member on a document with `pressure_contract` is refused by name, for
//!   either value (MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT);
//! - `timoshenko_annulus_energy_v1` on a pressure-free document is refused by name
//!   and nothing is solved (MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED);
//! - any other value, a non-string and `null` included, is refused by name
//!   (MODEL-SETTING-SHEAR-MODEL-UNKNOWN).
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewSolverMode};
use serde_json::{json, Value};

const DEMO: &str = include_str!("../../../fixtures/product_preview/invented_demo_model.json");
const EXACT: &str = include_str!("fixtures/exact_pressure_connected_request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

fn with_shear_model(request: &Value, value: Value) -> Value {
    let mut request = request.clone();
    let model = request["model"].as_object_mut().unwrap();
    let settings = model.entry("model_settings").or_insert_with(|| json!({}));
    settings["shear_model"] = value;
    request
}

fn solve(request: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).expect("admitted at the wire")).unwrap()
}

fn typed(request: &Value, mode: PreviewSolverMode) -> Value {
    let request: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).expect("typed decode");
    serde_json::to_value(run_linear_static_preview_with_mode(request, mode)).unwrap()
}

fn refused_by(envelope: &Value, code: &str) {
    let codes: Vec<&str> = envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking").map(|d| d["code"].as_str().unwrap()).collect();
    assert!(codes.contains(&code), "{code} not in {codes:?}");
    assert_ne!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "nothing is solved");
    assert!(envelope["results"].as_array().is_none_or(Vec::is_empty), "no result rows");
}

#[test]
fn euler_bernoulli_solves_exactly_as_absent() {
    let demo = json!({"model": serde_json::from_str::<Value>(DEMO).unwrap(), "materials": []});
    assert!(demo["model"]["model_settings"].get("shear_model").is_none());
    let authored = with_shear_model(&demo, json!("euler_bernoulli"));
    for mode in MODES {
        let reference = solve(&demo, mode);
        assert_eq!(reference["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}");
        assert_eq!(serde_json::to_string(&solve(&authored, mode)).unwrap(), serde_json::to_string(&reference).unwrap(), "{mode:?}: value entry");
        assert_eq!(serde_json::to_string(&typed(&authored, mode)).unwrap(), serde_json::to_string(&typed(&demo, mode)).unwrap(), "{mode:?}: typed entry");
    }
}

#[test]
fn the_member_is_refused_with_a_pressure_contract_for_either_value() {
    let exact: Value = serde_json::from_str(EXACT).unwrap();
    assert!(exact["model"]["pressure_contract"].is_object());
    for value in ["euler_bernoulli", "timoshenko_annulus_energy_v1"] {
        let request = with_shear_model(&exact, json!(value));
        for mode in MODES {
            refused_by(&solve(&request, mode), "MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT");
            refused_by(&typed(&request, mode), "MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT");
        }
    }
}

#[test]
fn timoshenko_is_refused_on_a_pressure_free_document() {
    let demo = json!({"model": serde_json::from_str::<Value>(DEMO).unwrap(), "materials": []});
    let request = with_shear_model(&demo, json!("timoshenko_annulus_energy_v1"));
    for mode in MODES {
        let envelope = solve(&request, mode);
        refused_by(&envelope, "MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED");
        let message = envelope["diagnostics"].as_array().unwrap().iter().find(|d| d["code"] == "MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED").unwrap()["message"].clone();
        assert_eq!(message, json!("model_settings.shear_model `timoshenko_annulus_energy_v1` is not yet solved on a pressure-free document; set `euler_bernoulli` or remove the member."));
        refused_by(&typed(&request, mode), "MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED");
    }
}

#[test]
fn unknown_and_null_values_are_refused_by_name() {
    let demo = json!({"model": serde_json::from_str::<Value>(DEMO).unwrap(), "materials": []});
    for value in [json!(null), json!("timoshenko"), json!("Euler_Bernoulli"), json!(1), json!(-1), json!(1.5), json!(true),
        json!({}), json!([]), json!({"a": [1, {"b": "euler_bernoulli"}]}), json!(["euler_bernoulli"])] {
        let request = with_shear_model(&demo, value.clone());
        for mode in MODES {
            refused_by(&solve(&request, mode), "MODEL-SETTING-SHEAR-MODEL-UNKNOWN");
            refused_by(&typed(&request, mode), "MODEL-SETTING-SHEAR-MODEL-UNKNOWN");
        }
    }
}

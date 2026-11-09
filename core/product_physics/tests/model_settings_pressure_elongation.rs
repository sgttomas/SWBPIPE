//! Model document 0.9.0's `model_settings.pressure_elongation` (DEL-04-07 Design
//! `pressure-elongation.md` §3.1), at solve, through the public value entry in both solver
//! modes: each content refusal is named and nothing is solved; an admitted setting raises
//! none of them; a document without the member solves as before. The mechanics that read
//! the setting are not this file's (DEL-04-07 §4).
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const DEMO: &str = include_str!("../../../fixtures/product_preview/invented_demo_model.json");
const EXACT: &str = include_str!("fixtures/exact_pressure_connected_request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const CODES: [&str; 3] = [
    "MODEL-SETTING-PRESSURE-ELONGATION-WITHOUT-PRESSURE-CONTRACT",
    "MODEL-SETTING-PRESSURE-ELONGATION-CASE-UNKNOWN",
    "MODEL-SETTING-PRESSURE-ELONGATION-CASES-INVALID",
];

fn with_setting(request: &Value, value: Value) -> Value {
    let mut request = request.clone();
    let model = request["model"].as_object_mut().unwrap();
    let settings = model.entry("model_settings").or_insert_with(|| json!({}));
    settings["pressure_elongation"] = value;
    request
}

fn solve(request: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).expect("admitted at the wire")).unwrap()
}

fn blocking(envelope: &Value) -> Vec<(String, String)> {
    envelope["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["severity"] == "blocking")
        .map(|d| (d["code"].as_str().unwrap().to_string(), d["message"].as_str().unwrap().to_string()))
        .collect()
}

fn refused_with(envelope: &Value, expected: &[(&str, &str)]) {
    let named: Vec<(String, String)> = blocking(envelope).into_iter().filter(|(code, _)| CODES.contains(&code.as_str())).collect();
    let expected: Vec<(String, String)> = expected.iter().map(|(c, m)| (c.to_string(), m.to_string())).collect();
    assert_eq!(named, expected);
    assert_ne!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "nothing is solved");
    assert!(envelope["results"].as_array().is_none_or(Vec::is_empty), "no result rows");
}

#[test]
fn each_refusal_is_named_at_solve() {
    let demo = json!({"model": serde_json::from_str::<Value>(DEMO).unwrap(), "materials": []});
    let exact: Value = serde_json::from_str(EXACT).unwrap();
    for mode in MODES {
        refused_with(
            &solve(&with_setting(&demo, json!({"operating_load_cases": ["load:L-100"]})), mode),
            &[("MODEL-SETTING-PRESSURE-ELONGATION-WITHOUT-PRESSURE-CONTRACT", "model_settings.pressure_elongation is set on a document without pressure_contract; there is no pressure to elongate, so remove model_settings.pressure_elongation.")],
        );
        refused_with(
            &solve(&with_setting(&exact, json!({"operating_load_cases": ["case:x"]})), mode),
            &[("MODEL-SETTING-PRESSURE-ELONGATION-CASE-UNKNOWN", "model_settings.pressure_elongation.operating_load_cases names `case:x`, which is not a load case of this document.")],
        );
        refused_with(
            &solve(&with_setting(&exact, json!({"operating_load_cases": []})), mode),
            &[("MODEL-SETTING-PRESSURE-ELONGATION-CASES-INVALID", "model_settings.pressure_elongation.operating_load_cases is empty; name at least one operating load case, or remove model_settings.pressure_elongation to switch pressure elongation off.")],
        );
        refused_with(
            &solve(&with_setting(&exact, json!({"operating_load_cases": ["case:closed-pressure", "case:closed-pressure"]})), mode),
            &[("MODEL-SETTING-PRESSURE-ELONGATION-CASES-INVALID", "model_settings.pressure_elongation.operating_load_cases names `case:closed-pressure` more than once; name each operating load case once.")],
        );
    }
}

#[test]
fn an_admitted_setting_raises_no_refusal_and_absence_solves_as_before() {
    let exact: Value = serde_json::from_str(EXACT).unwrap();
    assert!(exact["model"].get("model_settings").is_none());
    let authored = with_setting(&exact, json!({"operating_load_cases": ["case:closed-pressure"]}));
    for mode in MODES {
        let reference = solve(&exact, mode);
        assert_eq!(reference["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}: {:?}", blocking(&reference));
        let envelope = solve(&authored, mode);
        assert!(blocking(&envelope).iter().all(|(code, _)| !CODES.contains(&code.as_str())), "{mode:?}: {:?}", blocking(&envelope));
    }
}

#[test]
fn a_malformed_record_is_refused_at_decode() {
    let exact: Value = serde_json::from_str(EXACT).unwrap();
    for value in [json!(null), json!({}), json!({"operating_load_cases": "case:closed-pressure"}), json!({"operating_load_cases": ["case:closed-pressure"], "x": 1})] {
        assert!(run_linear_static_preview_value_with_mode(with_setting(&exact, value.clone()), PreviewSolverMode::DenseScrutiny).is_err(), "{value}");
    }
}

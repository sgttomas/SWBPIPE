//! Projection-sensitive authored options (DEL-04-11 REQ-003/009).
//! These are document-preservation and admission checks, not numerical references.

use open_pipe_stress_hanger_sizing::derived::derived_document;
use open_pipe_stress_hanger_sizing::publish::size_hangers;
use open_pipe_stress_product_physics::operating_conditions::preview_operating_conditions;
use serde_json::{json, Value};

fn input() -> Value {
    serde_json::from_str(include_str!("../../../../fixtures/results/hanger_sizing_input_ex_pressurised.json")).unwrap()
}

fn pf_input() -> Value {
    serde_json::from_str(include_str!("../../../../fixtures/results/hanger_sizing_input_two_span.json")).unwrap()
}

fn assert_authored_refusal(input: &Value, code: &str) {
    // The existing no-solve product judgement independently identifies the authored
    // condition/combination refusal before sizing removes cases or combinations.
    let admission = preview_operating_conditions(&input["request"]["model"]);
    assert_eq!(admission["resolved"], false, "{admission}");
    assert!(admission["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == code), "{admission}");
    let sized = size_hangers(&serde_json::to_string(input).unwrap());
    assert!(!sized.is_sized(), "an authored refusal must survive projection: {}", sized.document());
    assert!(sized.document().to_string().contains(code), "the product refusal must remain named: {}", sized.document());
}

#[test]
fn an_unselected_cases_condition_use_is_not_hidden_by_projection() {
    let mut input = input();
    let mut unused = input["request"]["model"]["load_cases"][0].clone();
    unused["id"] = json!("unused-condition-case");
    unused["primitive_loads"] = json!([]);
    unused["analysis_state"]["load_sources"] = json!([]);
    unused["condition_use"] = json!({"temperature": "operating"});
    input["request"]["model"]["load_cases"].as_array_mut().unwrap().push(unused);
    assert_authored_refusal(&input, "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
}

#[test]
fn an_unsupported_authored_combination_is_not_hidden_by_projection() {
    let mut input = input();
    let cases = &input["request"]["model"]["load_cases"];
    let operating = cases[1]["id"].clone();
    let weight = cases[0]["id"].clone();
    input["request"]["model"]["combinations"] = json!([{
        "id": "authored-operating-difference", "basis": "result_state_subtraction",
        "minuend_id": operating, "subtrahend_id": weight,
        "difference_basis": "operating_minus_sustained", "subtrahend_support_state": "own"
    }]);
    assert_authored_refusal(&input, "LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED");
}

#[test]
fn pf_unselected_condition_and_invalid_combination_keep_the_product_refusals() {
    let mut input = pf_input();
    let mut unused = input["request"]["model"]["load_cases"][0].clone();
    unused["id"] = json!("unused-condition-case");
    unused["primitive_loads"] = json!([]);
    unused["condition_use"] = json!({"contents_density": "operating"});
    input["request"]["model"]["load_cases"].as_array_mut().unwrap().push(unused);
    assert_authored_refusal(&input, "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED");

    let mut input = pf_input();
    let weight = input["sizing"]["weight_case"].clone();
    input["request"]["model"]["combinations"] = json!([{
        "id": "invalid-difference", "basis": "mechanics",
        "terms": [{"load_case": weight, "factor": 1.0}],
        "difference_basis": "operating_minus_sustained"
    }]);
    assert_authored_refusal(&input, "LOAD_COMBINATION_DIFFERENCE_BASIS_MISPLACED");
}

#[test]
fn a_supported_pf_combination_is_checked_then_omitted_from_derived_solves() {
    let mut input = pf_input();
    let weight = input["sizing"]["weight_case"].clone();
    input["request"]["model"]["combinations"] = json!([{
        "id": "weight-copy", "basis": "mechanics",
        "terms": [{"load_case": weight, "factor": 1.0}]
    }]);
    let admission = preview_operating_conditions(&input["request"]["model"]);
    assert_eq!(admission["resolved"], true, "{admission}");
    let result = size_hangers(&serde_json::to_string(&input).unwrap());
    assert!(result.is_sized(), "{}", result.document());
    let projected = derived_document(&input["request"], weight.as_str().unwrap(), &[], &[]);
    assert_eq!(projected["model"]["combinations"], json!([]));
    assert_eq!(input["request"]["model"]["combinations"][0]["id"], "weight-copy");
}

#[test]
fn conflicting_model_settings_keep_the_product_refusal_at_the_real_entry() {
    for (setting, code) in [
        (json!({"shear_model": "euler_bernoulli"}), "MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT"),
        (json!({"shear_model": "unknown"}), "MODEL-SETTING-SHEAR-MODEL-UNKNOWN"),
        (json!({"node_proximity_tolerance": {"value": -1.0, "unit": "m"}}), "MODEL-SETTING-TOLERANCE-NOT-POSITIVE"),
    ] {
        let mut input = input();
        input["request"]["model"]["model_settings"] = setting;
        let result = size_hangers(&serde_json::to_string(&input).unwrap());
        assert!(!result.is_sized());
        assert!(result.document().to_string().contains(code), "{code}: {}", result.document());
    }
}

#[test]
fn retained_case_options_and_their_model_records_survive_projection() {
    let mut input = input();
    let request = &mut input["request"];
    request["model"]["model_settings"] = json!({"node_proximity_tolerance": {"value": 0.002, "unit": "m"}});
    request["model"]["condition_sets"] = json!([{
        "id": "conditions", "provenance": "invented_test_value",
        "conditions": [{"key": "operating", "temperature": {"value": 350.0, "unit": "K"},
            "pressure": {"value": 2.0e6, "unit": "Pa"}, "contents_density": {"value": 900.0, "unit": "kg/m^3"}}],
        "added_weight": {"magnitude": {"value": 12.0, "unit": "N/m"}, "direction": "global_z"},
        "exposure": {"wind": true}
    }]);
    request["model"]["pipe_segments"][0]["condition_set_ref"] = json!("conditions");
    let retained = &mut request["model"]["load_cases"][1];
    retained["condition_use"] = json!({"temperature": "operating", "pressure": "operating",
        "contents_density": "operating", "added_weight": true, "wind_exposure": true});
    retained["equivalent_static"] = json!({"wind": {"pressure": {"value": 100.0, "unit": "Pa"},
        "shape_factor": {"value": 1.0, "unit": "1"}, "direction": "global_x"}});
    // These records are inspected at the projection boundary. This test does not
    // claim the EX condition-use combination is admitted; the refusal is above.
    let case = retained["id"].as_str().unwrap().to_owned();
    let before = request.clone();
    let projected = derived_document(request, &case, &[], &[]);
    assert_eq!(*request, before, "the original source remains bound and unchanged");
    assert_eq!(projected["model"]["load_cases"], json!([before["model"]["load_cases"][1]]));
    for key in ["condition_sets", "pipe_segments", "materials", "reference_configurations", "model_settings", "pressure_contract"] {
        assert_eq!(projected["model"][key], before["model"][key], "{key}");
    }
    assert_eq!(projected["materials"], before["materials"]);
    assert_eq!(projected["model"]["combinations"], json!([]));

    // The two mutually exclusive material-basis forms each survive verbatim;
    // this checks projection, without asserting their physical admission here.
    for (member, value) in [
        ("modulus_basis_ref", json!("authored-temperature-point")),
        ("modulus_basis_temperature", json!({"value": 350.0, "unit": "K"})),
    ] {
        let mut variant = before.clone();
        variant["model"]["load_cases"][1][member] = value.clone();
        let projected = derived_document(&variant, &case, &[], &[]);
        assert_eq!(projected["model"]["load_cases"][0][member], value);
    }
}

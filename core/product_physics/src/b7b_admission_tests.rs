//! Current-format parser controls use the actual public/captured deserializer.
use super::*;
use serde_json::{json, Value};

fn request() -> Value {
    json!({"model": {
        "schema_version": "0.5.0",
        "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:b7b"},
        "analysis_status": {"mechanics": "MODEL_INCOMPLETE", "rule_check": "RULE_INPUTS_INCOMPLETE", "professional_acceptance": "HUMAN_REVIEW_REQUIRED"},
        "nodes": [], "pipe_segments": [], "supports": []
    }})
}

fn contract() -> Value {
    json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"})
}

fn assert_decode_refused(raw: Value) {
    assert!(serde_json::from_value::<LinearStaticPreviewRequest>(raw.clone()).is_err());
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        assert!(source_receipt::CapturedInvocation::parse(raw.clone(), mode).is_err());
        assert!(run_linear_static_preview_value_with_mode(raw.clone(), mode).is_err());
        let invocation = json!({"request": &raw, "solver_mode": mode.as_str()});
        let request_id = "request:b7b".to_string();
        let context = RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &request_id);
        assert!(run_linear_static_preview_value_with_retained_headless(
            raw.clone(), mode, context, RetainedByteLimit::new(0),
        ).is_err());
    }
}

#[test]
fn b7b_contract_absence_and_current_object_are_distinct_positive_controls() {
    let raw = request();
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(raw.clone()).unwrap();
    assert!(model_format::is_pressure_free(&parsed.model));
    assert!(source_receipt::CapturedInvocation::parse(raw, PreviewSolverMode::SparseInteractive).is_ok());
    let mut exact = request();
    exact["model"]["pressure_contract"] = contract();
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(exact.clone()).unwrap();
    assert!(!model_format::is_pressure_free(&parsed.model));
    assert_eq!(exact_admission::ExactContract::of(&parsed.model), Some(exact_admission::ExactContract::PressureV3));
    assert!(source_receipt::CapturedInvocation::parse(exact, PreviewSolverMode::DenseScrutiny).is_ok());
}

#[test]
fn b7b_authored_contract_null_array_scalar_and_malformed_fail_closed() {
    for value in [Value::Null, json!([]), json!(["3.0.0", "exact_pressure_v3", "euler_bernoulli"]),
        json!(false), json!(1), json!("exact_pressure_v3"),
        json!({"version": 3, "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}),
        json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli", "extra": 1}),
    ] {
        let mut raw = request();
        raw["model"]["pressure_contract"] = value;
        assert_decode_refused(raw);
    }
}

#[test]
fn b7b_contract_duplicate_keys_are_refused_on_the_actual_text_decoder() {
    let mut raw = request();
    raw["model"]["pressure_contract"] = contract();
    let encoded = serde_json::to_string(&raw).unwrap();
    for (key, value) in [("version", "3.0.0"), ("mode", "exact_pressure_v3"), ("shear_model", "euler_bernoulli")] {
        let old = format!("\"{key}\":\"{value}\"");
        let duplicate = encoded.replacen(&old, &format!("{old},{old}"), 1);
        let error = serde_json::from_str::<LinearStaticPreviewRequest>(&duplicate).unwrap_err().to_string();
        assert!(error.contains("duplicate field"), "{error}");
    }
}

#[test]
fn b7b_top_level_shear_is_rejection_only_even_when_null() {
    for value in [Value::Null, json!("euler_bernoulli"), json!(false), json!({})] {
        let mut raw = request();
        raw["model"]["shear_model"] = value;
        let error = serde_json::from_value::<LinearStaticPreviewRequest>(raw.clone()).unwrap_err().to_string();
        assert!(error.contains("SHEAR_MODEL_WITHOUT_PRESSURE_CONTRACT"), "{error}");
        assert_decode_refused(raw.clone());
        raw["model"]["pressure_contract"] = contract();
        let error = serde_json::from_value::<LinearStaticPreviewRequest>(raw.clone()).unwrap_err().to_string();
        assert!(error.contains("unknown field"), "{error}");
        assert_decode_refused(raw);
    }
}

#[test]
fn b7b_corrosion_allowance_is_an_authored_object_without_a_zero_default() {
    let section = json!({"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}});
    let absent: PipeSectionInput = serde_json::from_value(section.clone()).unwrap();
    assert!(absent.corrosion_allowance.is_none());
    let mut zero = section.clone();
    zero["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    let explicit: PipeSectionInput = serde_json::from_value(zero).unwrap();
    assert_eq!(explicit.corrosion_allowance.unwrap().value.to_bits(), 0.0f64.to_bits());
    for value in [Value::Null, json!([]), json!([0.0, "m"]), json!(0), json!({"value": 0.0}), json!({"value": "0", "unit": "m"})] {
        let mut invalid = section.clone();
        invalid["corrosion_allowance"] = value;
        assert!(serde_json::from_value::<PipeSectionInput>(invalid.clone()).is_err());
        let mut raw = request();
        raw["model"]["pipe_segments"] = json!([{"id": "pipe:b7b", "from": "node:a", "to": "node:b", "material": "material:b7b", "section": invalid}]);
        assert_decode_refused(raw);
    }
}

#[test]
fn b7b_valid_retired_versions_keep_the_named_refusal() {
    for version in RETIRED_MODEL_SCHEMA_VERSIONS {
        let mut raw = request();
        raw["model"]["schema_version"] = json!(version);
        raw["model"]["pressure_contract"] = contract();
        let parsed: LinearStaticPreviewRequest = serde_json::from_value(raw).unwrap();
        let mut diagnostics = Vec::new();
        pressure_runtime::validate_profile(&parsed.model, &mut diagnostics);
        assert_eq!(diagnostics[0].code, MODEL_SCHEMA_VERSION_RETIRED);
    }
}

fn current_joint_request() -> Value {
    // Re-expression of invented authored data only; no user-model default.
    let mut raw: Value = serde_json::from_str(include_str!(
        "../../../validation/references/t4_u5a/requests/J3-0.request.json"
    )).unwrap();
    raw["model"]["schema_version"] = json!("0.5.0");
    raw["model"]["pressure_contract"] = contract();
    for pipe in raw["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    }
    raw
}

fn public_codes(raw: Value) -> Vec<String> {
    run_linear_static_preview_value_with_mode(raw, PreviewSolverMode::SparseInteractive)
        .unwrap().diagnostics.into_iter().map(|diagnostic| diagnostic.code).collect()
}

#[test]
fn b7b_explicit_ca_refusals_preserve_unit_and_section_priority() {
    let base = current_joint_request();
    let variants = [
        (None, "EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED"),
        (Some(json!({"value": 0.0, "unit": "Pa"})), "UNIT_CONVERSION_UNAVAILABLE"),
        (Some(json!({"value": -0.001, "unit": "m"})), "PIPE_DIMENSION_INVALID"),
        (Some(json!({"value": 1.0, "unit": "m"})), "PIPE_DIMENSION_INVALID"),
        (Some(json!({"value": 0.001, "unit": "m"})), "EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED"),
    ];
    for (quantity, expected) in variants {
        let mut raw = base.clone();
        let section = raw["model"]["pipe_segments"][0]["section"].as_object_mut().unwrap();
        match quantity {
            Some(value) => { section.insert("corrosion_allowance".into(), value); }
            None => { section.remove("corrosion_allowance"); }
        }
        let codes = public_codes(raw);
        assert!(codes.iter().any(|code| code == expected), "{expected}: {codes:?}");
        if expected != "EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED" {
            assert!(!codes.iter().any(|code| code == "EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED"), "{codes:?}");
        }
    }
    let mut raw = base;
    raw["model"].as_object_mut().unwrap().remove("pressure_contract");
    let codes = public_codes(raw);
    assert!(codes.iter().any(|code| code == "CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT"), "{codes:?}");
}

#[test]
fn b7b_shear_required_and_known_unsupported_have_named_public_refusals() {
    let mut raw = current_joint_request();
    raw["model"]["pressure_contract"].as_object_mut().unwrap().remove("shear_model");
    assert!(public_codes(raw).iter().any(|code| code == "EXACT_PRESSURE_SHEAR_MODEL_REQUIRED"));
    let mut raw = current_joint_request();
    raw["model"]["pressure_contract"]["shear_model"] = json!("timoshenko_annulus_energy_v1");
    assert!(public_codes(raw).iter().any(|code| code == "EXACT_PRESSURE_SHEAR_MODEL_UNSUPPORTED"));
}

#[test]
fn b7b_absent_contract_objective_connector_keeps_its_named_refusal() {
    let mut raw = current_joint_request();
    raw["model"].as_object_mut().unwrap().remove("pressure_contract");
    for pipe in raw["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"].as_object_mut().unwrap().remove("corrosion_allowance");
    }
    assert!(public_codes(raw).iter().any(|code| code == "PREVIEW_CONTRACT_VERSION_MISMATCH"));
}

#[test]
fn b7b_changed_source_is_unregistered_and_cannot_issue_a_permit() {
    assert!(retained_memory::retained_profile().is_err());
    let raw = request();
    let output = run_linear_static_preview_value_with_retained_direct(
        raw, PreviewSolverMode::SparseInteractive, RetainedByteLimit::new(0),
    ).unwrap();
    assert!(matches!(output.into_publication(), RetainedPublication::Ordinary(_)));
}

//! Public operation coverage for explicitly authored pipe section inputs.
use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_operation,
    validate_operation_batch,
};
use open_pipe_stress_product_physics::{
    self_weight::{generate_self_weight_operations, SelfWeightRequest},
    PreviewModel,
};
use serde_json::{json, Value};

fn model() -> Value {
    json!({"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:invented","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"K"}},
        "analysis_status":{"mechanics":"not_run","rule_check":"not_run","professional_acceptance":"not_reviewed"},
        "nodes":[
            {"id":"node:root","position":{"x":0,"y":0,"z":0},"provenance":"invented"},
            {"id":"node:tip","position":{"x":2,"y":0,"z":0},"provenance":"invented"}],
        "materials":[{"id":"material:invented","label":"Invented elastic material","elastic_modulus":{"value":200000000000.0,"unit":"Pa"},"shear_modulus":{"value":77000000000.0,"unit":"Pa"},"provenance":"invented native M35 witness, no material standard"}],
        "supports":[],"sections":[],"pipe_segments":[],"components":[],"load_cases":[],"combinations":[]})
}

fn pipe() -> Value {
    // Original explicit connect payload from the native M35 witness.
    json!({"id":"pipe:beam","label":"Invented beam","from":"node:root","to":"node:tip","material":"material:invented","y_reference":{"x":0,"y":1,"z":0},"provenance":"invented native M35 witness","section":{"outside_diameter":{"value":0.1,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"},"material_density":{"value":1000,"unit":"kg/m^3"}}})
}

fn operation(payload: &Value) -> Value {
    json!({"operation_id":"operation:explicit-section","operation_kind":"connect","operation_status":"proposed","author_type":"user","target":{"object_type":"Element","ref":"pipe:beam"},"change":{"change_id":"change:explicit-section","change_kind":"connect_pipe_run","field_label":"pipe_segments","field_path":"pipe_segments","before":"not_present","after":payload.to_string(),"unit":"m","dimension":"length","source_note":"Explicit invented fixture"},"validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},"audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},"professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},"rationale":"Explicit invented fixture"})
}

fn claim(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}

fn accepted(payload: &Value) -> Value {
    let m = model();
    let before = m.clone();
    let op = operation(payload);
    let hash = claim(&m);
    let validation = validate_operation(&m, &op, Some(&hash));
    let outcome = apply_operation(&m, &op, Some(&hash));
    assert!(validation.applied_model.is_none());
    assert_eq!(
        serde_json::to_value(&validation.diff_preview).unwrap(),
        serde_json::to_value(&outcome.diff_preview).unwrap()
    );
    let applied = outcome
        .applied_model
        .unwrap_or_else(|| panic!("{:?}", outcome.diagnostics));
    assert_eq!(m, before);
    assert_eq!(applied["pipe_segments"][0]["section"], payload["section"]);
    let preview: Value = serde_json::from_str(&outcome.diff_preview[0].after).unwrap();
    assert_eq!(
        canonical_json(&preview["section"]),
        canonical_json(&payload["section"])
    );
    applied
}

fn rejected(payload: &Value) {
    let m = model();
    let before = m.clone();
    let op = operation(payload);
    for outcome in [
        validate_operation(&m, &op, None),
        apply_operation(&m, &op, None),
    ] {
        assert!(
            outcome.applied_model.is_none(),
            "accepted invalid payload {payload}"
        );
        assert!(outcome.diff_preview.is_empty());
        assert!(outcome.diagnostics.iter().any(|d| d.severity == "blocking"));
    }
    assert_eq!(m, before);
}

#[test]
fn original_native_density_survives_public_create_and_self_weight_planning() {
    let applied = accepted(&pipe());
    let batch = json!({"batch_id":"batch:original-density","operations":[operation(&pipe())]});
    let batched = apply_operation_batch(&model(), &batch, Some(&claim(&model())));
    assert_eq!(
        batched["applied_model"]["pipe_segments"][0]["section"],
        applied["pipe_segments"][0]["section"],
        "{batched}"
    );
    let request: SelfWeightRequest = serde_json::from_value(json!({"case_id":"case:weight","label":"Invented self weight","pipe_refs":["pipe:beam"],"gravity":{"value":-7,"unit":"m/s^2","axis":"global_y"},"provenance":"explicit invented gravity","source_model_hash":claim(&applied)["value"]})).unwrap();
    let typed: PreviewModel = serde_json::from_value(applied).unwrap();
    let result = generate_self_weight_operations(&typed, &request);
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn creation_matches_set_field_for_all_optional_section_quantities() {
    let quantities = [
        (
            "material_density",
            "density",
            json!({"value":0.04,"unit":"lb/in^3"}),
        ),
        ("mill_tolerance", "length", json!({"value":1.0,"unit":"mm"})),
        (
            "contents_density",
            "density",
            json!({"value":0.0,"unit":"kg/m^3"}),
        ),
        (
            "insulation_thickness",
            "length",
            json!({"value":0.0,"unit":"in"}),
        ),
        (
            "insulation_density",
            "density",
            json!({"value":0.0,"unit":"lb/in^3"}),
        ),
    ];
    let mut payload = pipe();
    payload["section"]
        .as_object_mut()
        .unwrap()
        .remove("material_density");
    let mut via_edits = accepted(&payload);
    for (key, dimension, quantity) in quantities {
        payload["section"][key] = quantity.clone();
        let mut edit = operation(&payload);
        edit["operation_kind"] = json!("modify");
        edit["change"]["change_kind"] = json!("set_field");
        edit["change"]["field_path"] = json!(format!("section.{key}.value"));
        edit["change"]["before"] = json!("TBD");
        edit["change"]["after"] = json!(quantity.to_string());
        edit["change"]["unit"] = quantity["unit"].clone();
        edit["change"]["dimension"] = json!(dimension);
        let outcome = apply_operation(&via_edits, &edit, None);
        via_edits = outcome
            .applied_model
            .unwrap_or_else(|| panic!("{:?}", outcome.diagnostics));
    }
    let created = accepted(&payload);
    assert_eq!(
        created["pipe_segments"][0]["section"],
        via_edits["pipe_segments"][0]["section"]
    );
}

#[test]
fn absent_null_and_incomplete_inputs_are_preserved_without_defaults() {
    let mut payload = pipe();
    payload["section"]
        .as_object_mut()
        .unwrap()
        .remove("material_density");
    accepted(&payload);
    for key in [
        "material_density",
        "mill_tolerance",
        "contents_density",
        "insulation_thickness",
        "insulation_density",
    ] {
        payload["section"][key] = Value::Null;
    }
    accepted(&payload);
    for key in ["insulation_thickness", "insulation_density"] {
        let mut payload = pipe();
        payload["section"][key] =
            json!({"value":0,"unit":if key == "insulation_thickness" { "mm" } else { "kg/m^3" }});
        accepted(&payload);
        let outcome = apply_operation(&model(), &operation(&payload), None);
        assert!(outcome
            .diagnostics
            .iter()
            .any(|d| d.code == "OP-MASS-NOT-SOLVE-READY"));
    }
}

#[test]
fn invalid_optional_quantities_and_unknown_section_fields_are_rejected() {
    for key in [
        "material_density",
        "mill_tolerance",
        "contents_density",
        "insulation_thickness",
        "insulation_density",
    ] {
        let unit = if matches!(key, "mill_tolerance" | "insulation_thickness") {
            "mm"
        } else {
            "kg/m^3"
        };
        for invalid in [
            json!({"value":-1,"unit":unit}),
            json!({"value":1,"unit":"Pa"}),
            json!({"value":"1","unit":unit}),
            json!({"value":1}),
            json!({"unit":unit}),
            json!({"value":1,"unit":unit,"unknown":true}),
            json!(1),
        ] {
            let mut payload = pipe();
            payload["section"][key] = invalid;
            rejected(&payload);
        }
    }
    for (key, invalid) in [
        ("material_density", json!({"value":0,"unit":"kg/m^3"})),
        ("mill_tolerance", json!({"value":10,"unit":"mm"})),
        ("mill_tolerance", json!({"value":11,"unit":"mm"})),
        (
            "unrecognized_density",
            json!({"value":1000,"unit":"kg/m^3"}),
        ),
    ] {
        let mut payload = pipe();
        payload["section"][key] = invalid;
        rejected(&payload);
    }
}

#[test]
fn creation_rejects_shared_section_reference_instead_of_silently_detaching() {
    let mut payload = pipe();
    payload["section_ref"] = json!("section:authored");
    rejected(&payload);
    let outcome = apply_operation(&model(), &operation(&payload), None);
    assert!(outcome
        .diagnostics
        .iter()
        .any(|d| d.message.contains("assign_section")));
}

#[test]
fn late_invalid_creation_rolls_back_prior_creation_atomically() {
    let m = model();
    let first = operation(&pipe());
    let mut invalid = pipe();
    invalid["id"] = json!("pipe:second");
    invalid["section"]["material_density"] = json!({"value":0,"unit":"kg/m^3"});
    let mut second = operation(&invalid);
    second["operation_id"] = json!("operation:second");
    second["change"]["change_id"] = json!("change:second");
    second["target"]["ref"] = json!("pipe:second");
    let batch = json!({"batch_id":"batch:explicit-section","operations":[first,second]});
    for outcome in [
        validate_operation_batch(&m, &batch, Some(&claim(&m))),
        apply_operation_batch(&m, &batch, Some(&claim(&m))),
    ] {
        assert_eq!(
            outcome["simulation_disposition"],
            "rolled_back_no_model_published"
        );
        assert!(outcome["applied_model"].is_null());
        assert!(outcome["acceptance"].is_null());
        assert_eq!(outcome["operation_outcomes"].as_array().unwrap().len(), 2);
        assert_eq!(
            outcome["operation_outcomes"][0]["simulation_status"],
            "validated_on_temporary_state"
        );
    }
    assert!(m["pipe_segments"].as_array().unwrap().is_empty());
}

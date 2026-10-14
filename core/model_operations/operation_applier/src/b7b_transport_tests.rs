use super::*;
use serde_json::json;

fn model() -> Value {
    json!({
        "schema_version": "0.5.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:b7b"},
        "analysis_status": {"mechanics": "MODEL_INCOMPLETE", "rule_check": "RULE_INPUTS_INCOMPLETE", "professional_acceptance": "HUMAN_REVIEW_REQUIRED"},
        "nodes": [], "supports": [], "components": [], "materials": [],
        "load_cases": [{"id": "case:b7b", "primitive_loads": [], "pressure_regions": []}],
        "pipe_segments": [{"id": "pipe:b7b", "from": "node:a", "to": "node:b", "material": "material:b7b",
            "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"},
                "mill_tolerance": {"value": 0.002, "unit": "m"}, "corrosion_allowance": {"value": 0.0, "unit": "m"}}}],
        "sections": [{"id": "section:b7b", "name": "Invented OD/wall", "section_type": "pipe",
            "properties": {"outside_diameter": {"value": 0.3, "unit": "m"}, "wall_thickness": {"value": 0.02, "unit": "m"}},
            "provenance": {"source": "invented"}}]
    })
}

fn profile_edit(model: &Value) -> Result<Option<rich_authoring::RichEdit>, rich_authoring::RichError> {
    let before = canonical_json(&json!({"schema_version": model["schema_version"]}));
    let after = canonical_json(&json!({"schema_version": "0.5.0",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}}));
    pressure_authoring::resolve(model, "Model", "project:b7b", "pressure_profile",
        &before, &after, "none", "dimensionless")
}

#[test]
fn b7b_profile_activation_requires_explicit_ca_and_never_writes_user_sections() {
    let model = model();
    let before = model.clone();
    let edit = profile_edit(&model).unwrap().unwrap();
    assert_eq!(model, before);
    assert_eq!(edit.writes.iter().map(|(path, _)| path.clone()).collect::<Vec<_>>(),
        vec![vec!["schema_version".to_string()], vec!["pressure_contract".to_string()]]);
    let mut absent = model.clone();
    absent["pipe_segments"][0]["section"].as_object_mut().unwrap().remove("corrosion_allowance");
    assert_eq!(profile_edit(&absent).unwrap_err().code, "EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED");
    let mut malformed = model.clone();
    malformed["pipe_segments"][0]["section"]["corrosion_allowance"] = Value::Null;
    assert!(profile_edit(&malformed).is_err());
    assert!(malformed["pipe_segments"][0]["section"]["corrosion_allowance"].is_null());
}

#[test]
fn b7b_ca_is_preserved_locally_through_shared_assignment_and_detach() {
    let original = model();
    let local = original["pipe_segments"][0]["section"].clone();
    let assigned = section_bindings::assign(&original, "pipe:b7b", false,
        r#"{"section_ref":"section:b7b"}"#).unwrap();
    for key in ["mill_tolerance", "corrosion_allowance"] {
        assert_eq!(assigned["pipe_segments"][0]["section"][key], local[key]);
    }
    assert_eq!(assigned["pipe_segments"][0]["section"]["outside_diameter"]["value"], 0.3);
    let detached = section_bindings::assign(&assigned, "pipe:b7b", true,
        r#"{"source_section_ref":"section:b7b"}"#).unwrap();
    assert!(detached["pipe_segments"][0].get("section_ref").is_none());
    for key in ["mill_tolerance", "corrosion_allowance"] {
        assert_eq!(detached["pipe_segments"][0]["section"][key], local[key]);
    }
    let mut shared_ca = original.clone();
    shared_ca["sections"][0]["properties"]["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    assert!(section_bindings::assign(&shared_ca, "pipe:b7b", false,
        r#"{"section_ref":"section:b7b"}"#).is_err());
}

#[test]
fn b7b_local_ca_authoring_rejects_null_wrong_dimension_and_exhausted_wall() {
    let section = model()["pipe_segments"][0]["section"].clone();
    assert!(rich_authoring::validate_pipe_section_inputs(&section).is_ok());
    for value in [Value::Null, json!({"value": 0.0, "unit": "Pa"}),
        json!({"value": -0.001, "unit": "m"}), json!({"value": 0.009, "unit": "m"})] {
        let mut invalid = section.clone();
        invalid["corrosion_allowance"] = value;
        assert!(rich_authoring::validate_pipe_section_inputs(&invalid).is_err());
    }
    let mut absent = section;
    absent.as_object_mut().unwrap().remove("corrosion_allowance");
    assert!(rich_authoring::validate_pipe_section_inputs(&absent).is_ok(), "incomplete sections remain authorable");
}

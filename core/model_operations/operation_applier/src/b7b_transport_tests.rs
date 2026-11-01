use super::*;
use serde_json::json;

fn model() -> Value {
    json!({
        "schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
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

/// T4-U8 (D-6/O3): both admitted shear models; the authoring default first.
const SHEAR_MODELS: [&str; 2] = ["timoshenko_annulus_energy_v1", "euler_bernoulli"];

fn profile_edit(model: &Value, shear_model: &str) -> Result<Option<rich_authoring::RichEdit>, rich_authoring::RichError> {
    let before = canonical_json(&json!({"schema_version": model["schema_version"]}));
    let after = canonical_json(&json!({"schema_version": "0.9.0",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear_model}}));
    pressure_authoring::resolve(model, "Model", "project:b7b", "pressure_profile",
        &before, &after, "none", "dimensionless")
}

/// T4-U8: the allowance rule is independent of the shear selection, so it is
/// checked under both admitted models (until U8 only euler_bernoulli was admitted).
#[test]
fn b7b_profile_activation_requires_explicit_ca_and_never_writes_user_sections() {
    for shear_model in SHEAR_MODELS {
        let model = model();
        let before = model.clone();
        let edit = profile_edit(&model, shear_model).unwrap().unwrap();
        assert_eq!(model, before);
        assert_eq!(edit.writes.iter().map(|(path, _)| path.clone()).collect::<Vec<_>>(),
            vec![vec!["schema_version".to_string()], vec!["pressure_contract".to_string()]], "{shear_model}");
        assert_eq!(edit.writes[1].1.as_ref().unwrap()["shear_model"], shear_model);
        let mut absent = model.clone();
        absent["pipe_segments"][0]["section"].as_object_mut().unwrap().remove("corrosion_allowance");
        assert_eq!(profile_edit(&absent, shear_model).unwrap_err().code, "EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED");
        let mut malformed = model.clone();
        malformed["pipe_segments"][0]["section"]["corrosion_allowance"] = Value::Null;
        assert!(profile_edit(&malformed, shear_model).is_err());
        assert!(malformed["pipe_segments"][0]["section"]["corrosion_allowance"].is_null());
    }
    // An unknown shear model is refused before any section is read.
    assert_eq!(profile_edit(&model(), "timoshenko").unwrap_err().code, "OP-PRESSURE-PROFILE-SUCCESSOR-REQUIRED");
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

fn editor_model() -> Value {
    let mut model = model();
    model["project"]["units"] = json!({"length": "m"});
    model["nodes"] = json!([
        {"id": "node:a", "label": "A", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": "invented"},
        {"id": "node:b", "label": "B", "position": {"x": 4.0, "y": 0.0, "z": 0.0}, "provenance": "invented"}]);
    model["pipe_segments"][0]["y_reference"] = json!({"x": 0.0, "y": 1.0, "z": 0.0});
    model
}

fn intent(kind: &str, path: &str, before: String, after: String, unit: &str, dimension: &str) -> Value {
    json!({"operation_id": "operation:b7b:transport", "operation_kind": "modify",
        "operation_status": "proposed", "author_type": "user",
        "target": {"object_type": "Element", "ref": "pipe:b7b"},
        "change": {"change_id": "change:b7b:transport", "change_kind": kind,
            "field_label": path, "field_path": path, "before": before, "after": after,
            "unit": unit, "dimension": dimension, "source_note": "invented explicit user entry"},
        "validation": {"schema_validation": "not_run", "constraint_validation": "not_run",
            "unit_validation": "not_run", "diff_preview_status": "not_generated", "application_status": "not_applied"},
        "rationale": "Invented transport regression",
        "audit_boundary": {"mutation_route": "structured_operations_only", "direct_model_mutation_allowed": false,
            "requires_user_acceptance": true, "mutates_accepted_model_state": false},
        "professional_boundary": {"human_review_required": true, "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false, "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false, "software_makes_authentication_claim": false}})
}

fn claim(model: &Value) -> Value {
    json!({"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_scope": "model_payload",
        "value": format!("sha256:{}", sha256_hex(&canonical_json(model)))})
}

#[test]
fn b7b_common_unbound_scalar_ca_review_apply_enforces_the_wall_sum() {
    let model = editor_model();
    for (after, accepted) in [("0.007", true), ("0.008", false), ("0.009", false)] {
        let operation = intent("set_field", "section.corrosion_allowance.value",
            "0".into(), after.into(), "m", "length");
        let hash = claim(&model);
        let preview = validate_operation(&model, &operation, Some(&hash));
        let applied = apply_operation(&model, &operation, Some(&hash));
        assert!(preview.applied_model.is_none());
        assert_eq!(applied.applied_model.is_some(), accepted, "{after}: {:?}", applied.diagnostics);
        if accepted {
            let updated = applied.applied_model.unwrap();
            assert_eq!(updated["pipe_segments"][0]["section"]["corrosion_allowance"]["value"], 0.007);
            assert_eq!(updated["pipe_segments"][0]["section"]["mill_tolerance"], model["pipe_segments"][0]["section"]["mill_tolerance"]);
        } else {
            assert!(preview.diagnostics.iter().any(|diagnostic| diagnostic.code == "OP-SECTION-BINDING-INVALID"));
            assert!(applied.diagnostics.iter().any(|diagnostic| diagnostic.code == "OP-SECTION-BINDING-INVALID"));
        }
    }
    for (value, unit) in [("-0.001", "m"), ("0", "Pa")] {
        let operation = intent("set_field", "section.corrosion_allowance.value", "0".into(), value.into(), unit, "length");
        let applied = apply_operation(&model, &operation, Some(&claim(&model)));
        assert!(applied.applied_model.is_none());
        assert!(!applied.diagnostics.iter().any(|diagnostic| diagnostic.code == "OP-SECTION-BINDING-INVALID"),
            "existing scalar/unit refusal must precede the new wall-sum check");
    }
    let mut invalid_mill = model.clone();
    invalid_mill["pipe_segments"][0]["section"]["mill_tolerance"] = json!({"value": -1.0, "unit": "m"});
    let operation = intent("set_field", "section.corrosion_allowance.value", "0".into(), "0.1".into(), "m", "length");
    let outcome = apply_operation(&invalid_mill, &operation, Some(&claim(&invalid_mill)));
    assert!(outcome.applied_model.is_none());
    assert!(outcome.diagnostics.iter().any(|diagnostic| diagnostic.code == "OP-SECTION-BINDING-INVALID"));
}

fn geometry_intent(model: &Value, split: bool) -> Value {
    let payload = if split {
        json!({"pipe_ref": "pipe:b7b", "fraction": 0.25,
            "new_node": {"id": "node:new", "label": "New", "provenance": "invented"},
            "new_pipe": {"id": "pipe:new", "label": "New pipe", "provenance": "invented"}})
    } else {
        json!({"mode": "in_place", "pipe_refs": ["pipe:b7b"],
            "transform": {"kind": "translate", "translation": {"x": 1.0, "y": 0.0, "z": 0.0, "unit": "m"}}})
    };
    let before = if split { canonical_json(&model["pipe_segments"][0]) }
        else { canonical_json(&json!({"nodes": model["nodes"], "pipe_segments": model["pipe_segments"]})) };
    intent(if split { "split_pipe_run" } else { "transform_pipe_run" }, "pipe_segments",
        before, canonical_json(&payload), "none", "dimensionless")
}

#[test]
fn b7b_common_split_transform_preserve_valid_or_absent_ca_and_refuse_invalid_ca() {
    for split in [true, false] {
        for allowance in [None, Some(json!({"value": 0.0, "unit": "m"}))] {
            let mut model = editor_model();
            if let Some(value) = &allowance { model["pipe_segments"][0]["section"]["corrosion_allowance"] = value.clone(); }
            else { model["pipe_segments"][0]["section"].as_object_mut().unwrap().remove("corrosion_allowance"); }
            let operation = geometry_intent(&model, split);
            let hash = claim(&model);
            let preview = validate_operation(&model, &operation, Some(&hash));
            assert!(preview.applied_model.is_none());
            let applied = apply_operation(&model, &operation, Some(&hash));
            let updated = applied.applied_model.unwrap_or_else(|| panic!("{:?}", applied.diagnostics));
            for pipe in updated["pipe_segments"].as_array().unwrap() {
                assert_eq!(pipe["section"].get("corrosion_allowance"), allowance.as_ref());
            }
        }
        for value in [Value::Null, json!({"value": -0.001, "unit": "m"}),
            json!({"value": 0.0, "unit": "Pa"}), json!({"value": 0.009, "unit": "m"})] {
            let mut model = editor_model();
            model["pipe_segments"][0]["section"]["corrosion_allowance"] = value;
            let operation = geometry_intent(&model, split);
            let hash = claim(&model);
            let preview = validate_operation(&model, &operation, Some(&hash));
            let applied = apply_operation(&model, &operation, Some(&hash));
            assert!(preview.applied_model.is_none());
            assert!(applied.applied_model.is_none(), "{:?}", applied.diagnostics);
            assert!(applied.diagnostics.iter().any(|diagnostic| diagnostic.severity == "blocking"));
        }
        let mut invalid_mill = editor_model();
        invalid_mill["pipe_segments"][0]["section"]["mill_tolerance"] = json!({"value": -1.0, "unit": "m"});
        invalid_mill["pipe_segments"][0]["section"]["corrosion_allowance"] = json!({"value": 0.1, "unit": "m"});
        let operation = geometry_intent(&invalid_mill, split);
        assert!(apply_operation(&invalid_mill, &operation, Some(&claim(&invalid_mill))).applied_model.is_none());
        invalid_mill["pipe_segments"][0]["section"].as_object_mut().unwrap().remove("corrosion_allowance");
        let operation = geometry_intent(&invalid_mill, split);
        assert!(apply_operation(&invalid_mill, &operation, Some(&claim(&invalid_mill))).applied_model.is_some(),
            "absent CA preserves unrelated incomplete authoring behavior");
    }
}

/// T4-U8: the sequence runs with each admitted shear model (the default first).
#[test]
fn b7b_common_explicit_region_then_profile_sequence_has_no_preactivation_cycle() {
    for shear_model in SHEAR_MODELS {
        explicit_region_then_profile(shear_model);
    }
}

fn explicit_region_then_profile(shear_model: &str) {
    let mut current = editor_model();
    current["load_cases"][0].as_object_mut().unwrap().remove("pressure_regions");
    let sections = current["pipe_segments"].clone();
    let mut region = intent("update_load", "pressure_regions", "not_present".into(), "[]".into(), "none", "dimensionless");
    region["target"] = json!({"object_type": "Load", "ref": "case:b7b"});
    let hash = claim(&current);
    let reviewed = validate_operation(&current, &region, Some(&hash));
    assert!(reviewed.applied_model.is_none());
    let outcome = apply_operation(&current, &region, Some(&hash));
    current = outcome.applied_model.unwrap_or_else(|| panic!("{:?}", outcome.diagnostics));
    assert_eq!(current["load_cases"][0]["pressure_regions"], json!([]));
    assert!(current.get("pressure_contract").is_none());
    assert_eq!(current["pipe_segments"], sections);
    let staged = open_pipe_stress_product_physics::run_linear_static_preview_value_with_mode(
        json!({"model": &current}), open_pipe_stress_product_physics::PreviewSolverMode::SparseInteractive,
    ).unwrap();
    assert!(staged.diagnostics.iter().any(|finding| finding.code == "PREVIEW_CONTRACT_VERSION_MISMATCH"),
        "explicit preactivation authoring does not admit pressure regions on the preview solve");
    let after = json!({"schema_version": "0.9.0", "pressure_contract": {
        "version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear_model}});
    let mut profile = intent("set_field", "pressure_profile",
        canonical_json(&json!({"schema_version": "0.9.0"})), canonical_json(&after), "none", "dimensionless");
    profile["target"] = json!({"object_type": "Model", "ref": "project:b7b"});
    let hash = claim(&current);
    let reviewed = validate_operation(&current, &profile, Some(&hash));
    assert!(reviewed.applied_model.is_none());
    let outcome = apply_operation(&current, &profile, Some(&hash));
    let activated = outcome.applied_model.unwrap_or_else(|| panic!("{:?}", outcome.diagnostics));
    assert_eq!(activated["pressure_contract"], after["pressure_contract"]);
    assert_eq!(activated["load_cases"], current["load_cases"]);
    assert_eq!(activated["pipe_segments"], sections);
}

//! Issue 380 (DEL-07-02 REQ-002 to REQ-004): the person enters an absent
//! material shear modulus G through the scalar `shear_modulus.value` field,
//! with an explicit stress unit, and nothing supplies it. The whole
//! `{value, unit}` quantity is written from that entry. Controls: no unit,
//! a unit or dimension that is not stress, a value that is not positive, a
//! stale before value, an existing G edited in place, E and alpha unchanged
//! in their existing-only rule, and a material on the E/nu basis keeping its
//! basis with no value derived for G. All data are invented.
use open_pipe_stress_operation_applier::{apply_operation, OperationOutcome};
use serde_json::{json, Value};

fn model(material: Value) -> Value {
    json!({
        "schema_version": "0.9.0",
        "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:g-entry", "name": "G entry", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "materials": [material],
        "nodes": [
            {"id": "node:a", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": "invented"},
            {"id": "node:b", "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": "invented"}
        ],
        "pipe_segments": [{"id": "pipe:a-b", "from": "node:a", "to": "node:b",
            "section": {"outside_diameter": {"value": 0.1, "unit": "m"}, "wall_thickness": {"value": 0.005, "unit": "m"}},
            "material": "material:g", "provenance": "invented"}],
        "supports": [], "components": [], "load_cases": [], "combinations": []
    })
}

/// A material with E and alpha and no G.
fn without_g() -> Value {
    model(json!({"id": "material:g", "label": "Invented", "elastic_modulus": {"value": 200e9, "unit": "Pa"},
        "thermal_expansion_coefficient": {"value": 1.2e-5, "unit": "1/degC"}, "provenance": "invented"}))
}

fn with_g() -> Value {
    let mut model = without_g();
    model["materials"][0]["shear_modulus"] = json!({"value": 77e9, "unit": "Pa"});
    model
}

/// The intent the property inspector emits for a unit-editable quantity
/// field: `after` is the `{value, unit}` payload, `unit` is the entered unit.
fn edit(field: &str, before: &str, after: &str, unit: &str, dimension: &str) -> Value {
    json!({
        "operation_id": "op:editor-intent-material-g", "operation_kind": "modify", "operation_status": "proposed",
        "author_type": "user", "target": {"object_type": "Material", "ref": "material:g"},
        "change": {"change_id": "change:material-g", "change_kind": "set_field", "field_label": field,
            "field_path": field, "before": before, "after": after, "unit": unit, "dimension": dimension,
            "source_note": "unit metadata required; entered unit captured explicitly"},
        "validation": {"schema_validation": "not_run", "constraint_validation": "not_run", "unit_validation": "not_run",
            "diff_preview_status": "not_generated", "application_status": "not_applied"},
        "audit_boundary": {"mutation_route": "structured_operations_only", "direct_model_mutation_allowed": false,
            "requires_user_acceptance": true, "mutates_accepted_model_state": false},
        "professional_boundary": {"human_review_required": true, "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false, "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false, "software_makes_authentication_claim": false},
        "rationale": "invented G entry"
    })
}

fn g(before: &str, value: Value, unit: &str) -> Value {
    edit("shear_modulus.value", before, &json!({"value": value, "unit": unit}).to_string(), unit, "stress")
}

fn codes(outcome: &OperationOutcome) -> Vec<&str> {
    outcome.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect()
}

fn applied(start: &Value, intent: &Value) -> Value {
    let outcome = apply_operation(start, intent, None);
    assert!(codes(&outcome).is_empty(), "{:?}", outcome.diagnostics);
    assert_eq!(outcome.validation.application_status, "applied_to_session_model");
    outcome.applied_model.expect("applied")
}

fn refused(start: &Value, intent: &Value, code: &str) {
    let before = start.to_string();
    let outcome = apply_operation(start, intent, None);
    assert!(codes(&outcome).contains(&code), "expected {code}: {:?}", outcome.diagnostics);
    assert!(outcome.applied_model.is_none());
    assert_eq!(start.to_string(), before, "the input model is unchanged");
}

/// Everything but the named material member is bit for bit what it was.
fn only_member_changed(start: &Value, after: &Value, member: &str) {
    let (mut left, mut right) = (start.clone(), after.clone());
    left["materials"][0].as_object_mut().unwrap().remove(member);
    right["materials"][0].as_object_mut().unwrap().remove(member);
    assert_eq!(left, right);
}

#[test]
fn an_absent_g_is_created_from_the_persons_value_and_unit() {
    let start = without_g();
    for (value, unit) in [(json!(79.3), "GPa"), (json!(79300.0), "MPa"), (json!(79.3e9), "Pa")] {
        let after = applied(&start, &g("TBD", value.clone(), unit));
        assert_eq!(after["materials"][0]["shear_modulus"], json!({"value": value, "unit": unit}));
        only_member_changed(&start, &after, "shear_modulus");
    }
    // A plain number with the entered unit is the same entry.
    let after = applied(&start, &edit("shear_modulus.value", "TBD", "79.3", "GPa", "stress"));
    assert_eq!(after["materials"][0]["shear_modulus"], json!({"value": 79.3, "unit": "GPa"}));
}

#[test]
fn an_absent_g_without_a_stress_unit_or_value_is_refused_and_nothing_is_supplied() {
    let start = without_g();
    // No unit: the inspector sends its missing marker.
    refused(&start, &g("TBD", json!(79.3), "TBD"), "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
    refused(&start, &edit("shear_modulus.value", "TBD", "79.3", "", "stress"), "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
    // A unit of another dimension, and a dimension other than stress.
    refused(&start, &g("TBD", json!(79.3), "m"), "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
    refused(&start, &edit("shear_modulus.value", "TBD", &json!({"value": 79.3, "unit": "m"}).to_string(), "m", "length"),
        "OP-UNIT-DIMENSION-MISMATCH");
    // Payload and intent units disagree.
    refused(&start, &edit("shear_modulus.value", "TBD", &json!({"value": 79.3, "unit": "MPa"}).to_string(), "GPa", "stress"),
        "OP-QUANTITY-PAYLOAD-INVALID");
    // Not positive, or no number.
    refused(&start, &g("TBD", json!(0.0), "GPa"), "OP-VALUE-NOT-POSITIVE");
    refused(&start, &g("TBD", json!(-79.3), "GPa"), "OP-VALUE-NOT-POSITIVE");
    refused(&start, &edit("shear_modulus.value", "TBD", "TBD", "GPa", "stress"), "OP-VALUE-NOT-NUMERIC");
    // A before value that is not the absence the person saw.
    refused(&start, &g("77000000000", json!(79.3), "GPa"), "OP-STALE-BEFORE-VALUE");
}

#[test]
fn an_existing_g_is_edited_in_place_and_a_stale_edit_is_refused() {
    let start = with_g();
    let after = applied(&start, &g("77000000000", json!(80.0), "GPa"));
    assert_eq!(after["materials"][0]["shear_modulus"], json!({"value": 80.0, "unit": "GPa"}));
    only_member_changed(&start, &after, "shear_modulus");
    // The absence marker is stale once G exists, and so is another value.
    refused(&start, &g("TBD", json!(80.0), "GPa"), "OP-STALE-BEFORE-VALUE");
    refused(&start, &g("76000000000", json!(80.0), "GPa"), "OP-STALE-BEFORE-VALUE");
    // A unit change without the value-and-unit payload is not converted silently.
    refused(&start, &edit("shear_modulus.value", "77000000000", "80", "GPa", "stress"), "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
    refused(&start, &g("77000000000", json!(0.0), "GPa"), "OP-VALUE-NOT-POSITIVE");
    refused(&start, &edit("shear_modulus.value", "77000000000", &json!({"value": 80.0, "unit": "m"}).to_string(), "m", "length"),
        "OP-UNIT-DIMENSION-MISMATCH");
}

#[test]
fn e_and_alpha_keep_their_existing_quantity_rule() {
    for member in ["elastic_modulus", "thermal_expansion_coefficient"] {
        let mut start = without_g();
        start["materials"][0].as_object_mut().unwrap().remove(member);
        let (unit, dimension) = if member == "elastic_modulus" { ("GPa", "stress") } else { ("1/degC", "thermal_expansion_coefficient") };
        let field = format!("{member}.value");
        refused(&start, &edit(&field, "TBD", &json!({"value": 1.0, "unit": unit}).to_string(), unit, dimension),
            "OP-QUANTITY-OBJECT-MISSING");
    }
}

#[test]
fn a_material_on_the_e_nu_basis_keeps_its_basis_and_no_g_is_derived() {
    let start = model(json!({"id": "material:g", "label": "Invented", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": {"value": 200e9, "unit": "Pa"}, "poisson_ratio": {"value": 0.3, "unit": "none"},
        "provenance": "invented"}));
    let after = applied(&start, &g("TBD", json!(79.3), "GPa"));
    // The entered value, never E / (2 (1 + nu)).
    assert_eq!(after["materials"][0]["shear_modulus"], json!({"value": 79.3, "unit": "GPa"}));
    only_member_changed(&start, &after, "shear_modulus");
    assert_eq!(after["materials"][0]["constitutive_basis"], "homogeneous_isotropic_E_nu_v1");
}

//! Issue 382 (DEL-16-02 REQ-010 and AC-006; DEL-07-02 REQ-003): one rule for
//! every quantity stored as a value beside its unit.
//!
//! A bare number is read in the stored unit, so the intent must name that
//! unit; otherwise the edit is refused with the stored unit and the explicit
//! value/unit pair named, and nothing is written. An explicit pair writes value
//! and unit together. The unit is judged against the field's own dimension
//! (for a primitive-load magnitude, the one its category, target and direction
//! give), never against the dimension the intent states.
//!
//! The first part is the family matrix: every family member against every
//! admission and refusal the rule names. The second carries an edit from the
//! operation to the stored document to the solve, and compares the published
//! result with a closed form computed here from the entered numbers and the
//! exact unit definitions, sharing no code with the product. All data are
//! invented.
use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_operation, validate_operation_batch, OperationOutcome,
};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use std::f64::consts::PI;

// Geometry and loads of the invented two-span cantilever, in SI.
const SPAN: f64 = 1.0;
const OUTSIDE_DIAMETER: f64 = 0.1143;
const WALL: f64 = 0.006;
const ELASTIC_MODULUS: f64 = 200.0e9;
const SHEAR_MODULUS: f64 = 77.0e9;
const EXPANSION: f64 = 1.2e-5;
const HOT_SHEAR_MODULUS: f64 = 68.0e9;
const AXIAL_FORCE: f64 = 1.0;
const TORQUE: f64 = 1.0;
const TEMPERATURE_RISE: f64 = 100.0;

const MATERIAL: &str = "material:steel-like";
const INBOARD: &str = "pipe:inboard";
const OUTBOARD: &str = "pipe:outboard";
const SECTION: &str = "section:shared";
const HOT_POINT: &str = "temperature.point:hot";
const AXIAL_CASE: &str = "case:axial";
const TORSION_CASE: &str = "case:torsion";
const THERMAL_CASE: &str = "case:thermal";
const HOT_TORSION_CASE: &str = "case:hot-torsion";
const SHAPES_CASE: &str = "case:shapes";

fn pipe(id: &str, from: &str, to: &str) -> Value {
    json!({"id": id, "label": id, "from": from, "to": to,
        "section": {"outside_diameter": {"value": OUTSIDE_DIAMETER, "unit": "m"},
            "wall_thickness": {"value": WALL, "unit": "m"},
            "material_density": {"value": 7850.0, "unit": "kg/m^3"}},
        "material": MATERIAL, "y_reference": {"x": 0, "y": 1, "z": 0}, "provenance": "invented"})
}

fn primitive(id: &str, category: &str, target: Value, direction: &str, value: f64, unit: &str, dimension: &str) -> Value {
    json!({"id": id, "category": category, "target": target, "direction": direction,
        "magnitude": {"value": value, "unit": unit}, "dimension": dimension, "provenance": "invented"})
}

fn tip() -> Value {
    json!({"type": "node", "node": "node:3"})
}

fn element(pipe: &str) -> Value {
    json!({"type": "element", "pipe": pipe})
}

/// The solved model: an anchored two-span cantilever along global X with an
/// axial force, a torque and a uniform temperature rise in separate cases, and
/// a torque case that selects the material's hot temperature point.
fn solved_model() -> Value {
    json!({
        "schema_version": "0.9.0",
        "document_kind": "openpipestress.product_preview.model",
        "data_boundary": {"source": "invented_example_not_library_data"},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": "project:unit-pair", "name": "Invented unit-pair regression",
            "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "materials": [{"id": MATERIAL, "label": "Invented steel-like material",
            "elastic_modulus": {"value": ELASTIC_MODULUS, "unit": "Pa"},
            "shear_modulus": {"value": SHEAR_MODULUS, "unit": "Pa"},
            "thermal_expansion_coefficient": {"value": EXPANSION, "unit": "1/degC"},
            "temperature_points": [{"id": HOT_POINT, "temperature": {"value": 200.0, "unit": "degC"},
                "elastic_modulus": {"value": 180.0e9, "unit": "Pa"},
                "shear_modulus": {"value": HOT_SHEAR_MODULUS, "unit": "Pa"},
                "provenance": "invented"}],
            "provenance": "invented"}],
        "nodes": [
            {"id": "node:1", "label": "1", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": "invented"},
            {"id": "node:2", "label": "2", "position": {"x": SPAN, "y": 0.0, "z": 0.0}, "provenance": "invented"},
            {"id": "node:3", "label": "3", "position": {"x": 2.0 * SPAN, "y": 0.0, "z": 0.0}, "provenance": "invented"}
        ],
        "pipe_segments": [pipe(INBOARD, "node:1", "node:2"), pipe(OUTBOARD, "node:2", "node:3")],
        "supports": [{"id": "support:anchor", "label": "Anchor", "node": "node:1", "family": "anchor",
            "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": "invented"}],
        "components": [],
        "load_cases": [
            {"id": AXIAL_CASE, "label": "Axial force", "provenance": "invented", "primitive_loads": [
                primitive("load:axial", "concentrated_force", tip(), "global_x", AXIAL_FORCE, "N", "force")]},
            {"id": TORSION_CASE, "label": "Torque", "provenance": "invented", "primitive_loads": [
                primitive("load:torque", "concentrated_moment", tip(), "rotation_x", TORQUE, "N*m", "moment")]},
            {"id": THERMAL_CASE, "label": "Temperature rise", "provenance": "invented", "primitive_loads": [
                primitive("load:rise-inboard", "thermal", element(INBOARD), "global_x", TEMPERATURE_RISE, "degC", "temperature_interval"),
                primitive("load:rise-outboard", "thermal", element(OUTBOARD), "global_x", TEMPERATURE_RISE, "degC", "temperature_interval")]},
            {"id": HOT_TORSION_CASE, "label": "Torque at the hot point", "provenance": "invented",
                "modulus_basis_ref": HOT_POINT, "primitive_loads": [
                primitive("load:hot-torque", "concentrated_moment", tip(), "rotation_x", TORQUE, "N*m", "moment")]}
        ],
        "combinations": [], "diagnostics": []
    })
}

/// The matrix model: the solved model with a shared section record (no pipe
/// is bound to it, so the inline quantities stay editable) and a case holding
/// every stored primitive shape the rule governs. It is edited, never solved.
fn matrix_model() -> Value {
    let mut model = solved_model();
    model["sections"] = json!([{"id": SECTION, "name": "Invented shared", "section_type": "pipe",
        "properties": {"outside_diameter": {"value": OUTSIDE_DIAMETER, "unit": "m"},
            "wall_thickness": {"value": WALL, "unit": "m"}}, "provenance": "invented"}]);
    let anchor = |dof: &str| json!({"type": "support", "support": "support:anchor", "dof": dof});
    model["load_cases"].as_array_mut().unwrap().push(json!({"id": SHAPES_CASE, "label": "Every shape",
        "provenance": "invented", "primitive_loads": [
            primitive("shape:force", "concentrated_force", tip(), "global_y", -40.0, "N", "force"),
            primitive("shape:moment", "concentrated_moment", tip(), "rotation_z", 12.0, "N*m", "moment"),
            primitive("shape:line", "distributed_force", element(OUTBOARD), "global_z", -190.0, "N/m", "force_per_length"),
            primitive("shape:rise", "thermal", element(OUTBOARD), "global_x", 55.0, "degC", "temperature_interval"),
            primitive("shape:settlement", "imposed_displacement", anchor("UY"), "UY", -0.002, "m", "displacement"),
            primitive("shape:tilt", "imposed_displacement", anchor("RZ"), "RZ", 0.001, "rad", "rotation"),
            primitive("shape:weight", "weight", tip(), "global_y", -420.0, "N", "force"),
            primitive("shape:weight-moment", "weight", tip(), "rotation_z", -50.0, "N*m", "moment"),
            primitive("shape:occasional", "occasional", tip(), "global_z", -190.0, "N", "force"),
            primitive("shape:occasional-line", "occasional", element(INBOARD), "global_y", 75.0, "N/m", "force_per_length"),
            primitive("shape:test-fill", "hydrotest", element(OUTBOARD), "global_y", -310.0, "N/m", "force_per_length")
        ]}));
    model
}

/// One family member: where it is, what it holds, and the units the cases
/// use. `other_unit` is another catalog unit of the field's dimension;
/// `foreign_unit` is a catalog unit of another dimension; `foreign_dimension`
/// is a catalog dimension that is not the field's, chosen where one exists so
/// that `other_unit` is one of its units too (the case in which trusting the
/// intent's dimension would have admitted the edit).
struct Member {
    name: &'static str,
    object_type: &'static str,
    target: &'static str,
    change_kind: &'static str,
    path: String,
    dimension: &'static str,
    stored_value: f64,
    stored_unit: &'static str,
    other_unit: &'static str,
    foreign_unit: &'static str,
    foreign_dimension: &'static str,
    positive_only: bool,
    /// A replacement number for the unit cases, as entered in the stored unit
    /// and as entered in `other_unit`; of the stored value's sign, so that a
    /// sign constraint is not what a unit case exercises.
    replacement: (f64, f64),
}

impl Member {
    /// A section's geometry is checked as a whole (0 < wall < diameter / 2),
    /// so its replacements are real pipe sizes in each unit.
    fn replacing_with(mut self, in_stored_unit: f64, in_other_unit: f64) -> Self {
        self.replacement = (in_stored_unit, in_other_unit);
        self
    }
}

fn member(
    name: &'static str,
    object_type: &'static str,
    target: &'static str,
    change_kind: &'static str,
    path: &str,
    dimension: &'static str,
    stored: (f64, &'static str),
    other_unit: &'static str,
    foreign: (&'static str, &'static str),
    positive_only: bool,
) -> Member {
    let replacement = if stored.0 < 0.0 { -3.5 } else { 3.5 };
    Member { name, object_type, target, change_kind, path: path.to_string(), dimension,
        stored_value: stored.0, stored_unit: stored.1, other_unit,
        foreign_unit: foreign.0, foreign_dimension: foreign.1, positive_only,
        replacement: (replacement, replacement) }
}

fn shape(index: usize) -> String {
    format!("primitive_loads.{index}.magnitude.value")
}

fn family() -> Vec<Member> {
    let stress = ("kg", "pressure");
    let length = ("Pa", "displacement");
    vec![
        member("elastic modulus", "Material", MATERIAL, "set_field", "elastic_modulus.value", "stress", (ELASTIC_MODULUS, "Pa"), "GPa", stress, true),
        member("base shear modulus", "Material", MATERIAL, "set_field", "shear_modulus.value", "stress", (SHEAR_MODULUS, "Pa"), "GPa", stress, true),
        member("expansion coefficient", "Material", MATERIAL, "set_field", "thermal_expansion_coefficient.value", "thermal_expansion_coefficient", (EXPANSION, "1/degC"), "1/degF", ("Pa", "stress"), false),
        member("temperature-point shear modulus", "Material", MATERIAL, "set_field", &format!("temperature_points.{HOT_POINT}.shear_modulus.value"), "stress", (HOT_SHEAR_MODULUS, "Pa"), "GPa", stress, true),
        member("inline outside diameter", "Element", OUTBOARD, "set_field", "section.outside_diameter.value", "length", (OUTSIDE_DIAMETER, "m"), "mm", length, true),
        member("inline wall thickness", "Element", INBOARD, "set_field", "section.wall_thickness.value", "length", (WALL, "m"), "mm", length, true),
        member("inline material density", "Element", INBOARD, "set_field", "section.material_density.value", "density", (7850.0, "kg/m^3"), "lb/in^3", ("Pa", "stress"), true),
        member("section outside diameter", "Section", SECTION, "set_field", "properties.outside_diameter.value", "length", (OUTSIDE_DIAMETER, "m"), "mm", length, true).replacing_with(0.1683, 168.3),
        member("section wall thickness", "Section", SECTION, "set_field", "properties.wall_thickness.value", "length", (WALL, "m"), "mm", length, true).replacing_with(0.00711, 7.11),
        member("concentrated force", "Load", SHAPES_CASE, "update_load", &shape(0), "force", (-40.0, "N"), "kN", ("N*m", "moment"), false),
        member("concentrated moment", "Load", SHAPES_CASE, "update_load", &shape(1), "moment", (12.0, "N*m"), "kN*m", ("N", "force"), false),
        member("distributed force", "Load", SHAPES_CASE, "update_load", &shape(2), "force_per_length", (-190.0, "N/m"), "kN/m", ("N", "force"), false),
        member("thermal interval", "Load", SHAPES_CASE, "update_load", &shape(3), "temperature_interval", (55.0, "degC"), "degF", ("Pa", "temperature"), false),
        member("imposed displacement", "Load", SHAPES_CASE, "update_load", &shape(4), "displacement", (-0.002, "m"), "mm", ("rad", "length"), false),
        member("imposed rotation", "Load", SHAPES_CASE, "update_load", &shape(5), "rotation", (0.001, "rad"), "deg", ("m", "angle"), false),
        member("nodal weight", "Load", SHAPES_CASE, "update_load", &shape(6), "force", (-420.0, "N"), "kN", ("N*m", "moment"), false),
        member("nodal weight moment", "Load", SHAPES_CASE, "update_load", &shape(7), "moment", (-50.0, "N*m"), "kN*m", ("N", "force"), false),
        member("nodal occasional force", "Load", SHAPES_CASE, "update_load", &shape(8), "force", (-190.0, "N"), "kN", ("N/m", "force_per_length"), false),
        member("occasional line load", "Load", SHAPES_CASE, "update_load", &shape(9), "force_per_length", (75.0, "N/m"), "kN/m", ("N", "force"), false),
        member("hydrotest line load", "Load", SHAPES_CASE, "update_load", &shape(10), "force_per_length", (-310.0, "N/m"), "kN/m", ("Pa", "pressure"), false),
    ]
}

fn intent(member: &Member, before: &str, after: &str, unit: &str, dimension: &str) -> Value {
    json!({
        "operation_id": "op:unit-pair", "operation_kind": "modify", "operation_status": "proposed",
        "author_type": "user", "target": {"object_type": member.object_type, "ref": member.target},
        "change": {"change_id": "change:unit-pair", "change_kind": member.change_kind, "field_label": member.name,
            "field_path": member.path, "before": before, "after": after, "unit": unit, "dimension": dimension,
            "source_note": "invented"},
        "validation": {"schema_validation": "not_run", "constraint_validation": "not_run", "unit_validation": "not_run",
            "diff_preview_status": "not_generated", "application_status": "not_applied"},
        "audit_boundary": {"mutation_route": "structured_operations_only", "direct_model_mutation_allowed": false,
            "requires_user_acceptance": true, "mutates_accepted_model_state": false},
        "professional_boundary": {"human_review_required": true, "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false, "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false, "software_makes_authentication_claim": false},
        "rationale": "invented unit-pair regression"
    })
}

fn pair(value: f64, unit: &str) -> String {
    json!({"value": value, "unit": unit}).to_string()
}

fn blocking(outcome: &OperationOutcome) -> Vec<&str> {
    outcome.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect()
}

/// The stored `{value, unit}` record a member's path names, read from a
/// document by walking it, the temperature point by its id.
fn stored(model: &Value, member: &Member) -> Value {
    let collection = match member.object_type {
        "Material" => "materials",
        "Element" => "pipe_segments",
        "Section" => "sections",
        "Load" => "load_cases",
        other => panic!("no collection for {other}"),
    };
    let entity = model[collection].as_array().unwrap().iter()
        .find(|entity| entity["id"] == member.target).unwrap_or_else(|| panic!("{} is absent", member.target));
    let quantity_path = member.path.strip_suffix(".value").expect("a value path");
    if let Some(rest) = quantity_path.strip_prefix("temperature_points.") {
        let point_id = rest.strip_suffix(".shear_modulus").expect("a temperature-point shear modulus path");
        return entity["temperature_points"].as_array().unwrap().iter()
            .find(|point| point["id"] == point_id).expect("the temperature point")["shear_modulus"].clone();
    }
    quantity_path.split('.').fold(entity, |value, segment| match segment.parse::<usize>() {
        Ok(index) => &value[index],
        Err(_) => &value[segment],
    }).clone()
}

fn apply(model: &Value, member: &Member, before: &str, after: &str, unit: &str, dimension: &str) -> OperationOutcome {
    apply_operation(model, &intent(member, before, after, unit, dimension), None)
}

/// The edit is applied and the stored record is exactly `{value, unit}`.
fn assert_stored(model: &Value, member: &Member, after: &str, unit: &str, value: f64, stored_unit: &str) -> Value {
    let outcome = apply(model, member, &member.stored_value.to_string(), after, unit, member.dimension);
    assert!(blocking(&outcome).is_empty(), "{}: {:?}", member.name, outcome.diagnostics);
    assert_eq!(outcome.validation.application_status, "applied_to_session_model", "{}", member.name);
    let applied = outcome.applied_model.unwrap_or_else(|| panic!("{}: no applied model", member.name));
    assert_eq!(stored(&applied, member), json!({"value": value, "unit": stored_unit}), "{}", member.name);
    applied
}

/// The edit is refused with `code` and no model is published.
fn assert_refused(model: &Value, member: &Member, before: &str, after: &str, unit: &str, dimension: &str, code: &str) -> OperationOutcome {
    let outcome = apply(model, member, before, after, unit, dimension);
    assert!(blocking(&outcome).contains(&code), "{}: expected {code}, got {:?}", member.name, outcome.diagnostics);
    assert!(outcome.applied_model.is_none(), "{}: a refused edit published a model", member.name);
    assert_ne!(outcome.validation.application_status, "applied_to_session_model", "{}", member.name);
    outcome
}

/// A refusal of an entered unit names the stored unit and the explicit-pair
/// route, whichever check meets the unit first.
fn assert_names_stored_unit_and_pair(outcome: &OperationOutcome, member: &Member) {
    let finding = outcome.diagnostics.iter().find(|d| d.code == "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE").unwrap();
    assert!(finding.message.contains(&format!("stored unit `{}`", member.stored_unit)), "{}: {}", member.name, finding.message);
    assert!(finding.remediation.contains("explicit JSON value/unit pair"), "{}: {}", member.name, finding.remediation);
}

/// The member's replacement number as entered in its stored unit.
fn replacement(member: &Member) -> f64 {
    member.replacement.0
}

#[test]
fn the_family_holds_every_quantity_the_rule_governs_in_this_model() {
    let model = matrix_model();
    for member in family() {
        assert_eq!(stored(&model, &member), json!({"value": member.stored_value, "unit": member.stored_unit}), "{}", member.name);
    }
}

#[test]
fn a_bare_number_in_the_stored_unit_replaces_the_number_and_keeps_the_unit() {
    let model = matrix_model();
    for member in family() {
        let value = replacement(&member);
        assert_stored(&model, &member, &value.to_string(), member.stored_unit, value, member.stored_unit);
    }
}

#[test]
fn a_bare_number_naming_another_unit_is_refused_with_the_stored_unit_and_the_pair_named() {
    let model = matrix_model();
    for member in family() {
        let before = member.stored_value.to_string();
        let outcome = assert_refused(&model, &member, &before, &replacement(&member).to_string(), member.other_unit,
            member.dimension, "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
        let finding = outcome.diagnostics.iter().find(|d| d.code == "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE").unwrap();
        assert!(finding.message.contains(&format!("stored unit `{}`", member.stored_unit)), "{}: {}", member.name, finding.message);
        assert!(finding.message.contains("explicit value-and-unit payload"), "{}: {}", member.name, finding.message);
        assert!(finding.remediation.contains("explicit JSON value/unit pair"), "{}: {}", member.name, finding.remediation);
        assert_eq!(outcome.validation.unit_validation, "blocked", "{}", member.name);
    }
}

#[test]
fn an_explicit_pair_in_another_unit_of_the_dimension_writes_value_and_unit_together() {
    let model = matrix_model();
    for member in family() {
        let value = member.replacement.1;
        let applied = assert_stored(&model, &member, &pair(value, member.other_unit), member.other_unit, value, member.other_unit);
        // Nothing else in the document moved: restoring the one record gives the start.
        let mut restored = applied;
        let outcome = apply(&restored, &member, &value.to_string(), &pair(member.stored_value, member.stored_unit),
            member.stored_unit, member.dimension);
        assert!(blocking(&outcome).is_empty(), "{}: {:?}", member.name, outcome.diagnostics);
        restored = outcome.applied_model.unwrap();
        assert_eq!(restored, model, "{}: the edit moved more than its quantity", member.name);
    }
}

#[test]
fn a_unit_of_another_dimension_is_refused_bare_or_paired() {
    let model = matrix_model();
    for member in family() {
        let before = member.stored_value.to_string();
        let value = replacement(&member);
        for after in [value.to_string(), pair(value, member.foreign_unit)] {
            let outcome = assert_refused(&model, &member, &before, &after, member.foreign_unit, member.dimension,
                "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
            assert_names_stored_unit_and_pair(&outcome, &member);
        }
    }
}

#[test]
fn an_unknown_unit_is_refused_bare_or_paired() {
    let model = matrix_model();
    for member in family() {
        let before = member.stored_value.to_string();
        let value = replacement(&member);
        for after in [value.to_string(), pair(value, "furlong")] {
            let outcome = assert_refused(&model, &member, &before, &after, "furlong", member.dimension,
                "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
            assert_names_stored_unit_and_pair(&outcome, &member);
        }
    }
}

#[test]
fn a_missing_unit_is_refused_and_nothing_is_written() {
    let model = matrix_model();
    for member in family() {
        let before = member.stored_value.to_string();
        let value = replacement(&member);
        for (after, unit) in [(value.to_string(), ""), (json!({"value": value}).to_string(), member.stored_unit),
            (json!({"value": value, "unit": ""}).to_string(), member.stored_unit)] {
            let outcome = apply(&model, &member, &before, &after, unit, member.dimension);
            assert!(!blocking(&outcome).is_empty(), "{}: `{after}` with unit `{unit}` was not refused", member.name);
            assert!(outcome.applied_model.is_none(), "{}: `{after}` with unit `{unit}` published a model", member.name);
        }
    }
}

#[test]
fn the_dimension_is_the_fields_and_an_intent_stating_another_is_refused() {
    let model = matrix_model();
    for member in family() {
        let before = member.stored_value.to_string();
        let value = replacement(&member);
        // Bare in the stored unit, and paired in a unit the stated dimension
        // may itself accept: neither is admitted on the intent's word.
        for (after, unit) in [(value.to_string(), member.stored_unit), (pair(value, member.other_unit), member.other_unit),
            (pair(value, member.foreign_unit), member.foreign_unit)] {
            let outcome = assert_refused(&model, &member, &before, &after, unit, member.foreign_dimension,
                "OP-UNIT-DIMENSION-MISMATCH");
            let finding = outcome.diagnostics.iter().find(|d| d.code == "OP-UNIT-DIMENSION-MISMATCH").unwrap();
            assert!(finding.message.contains(&format!("`{}`", member.dimension)), "{}: {}", member.name, finding.message);
        }
    }
}

#[test]
fn a_dimension_outside_the_vocabulary_is_refused() {
    let model = matrix_model();
    for member in family() {
        let before = member.stored_value.to_string();
        let outcome = apply(&model, &member, &before, &replacement(&member).to_string(), member.stored_unit, "not_a_dimension");
        assert!(blocking(&outcome).iter().any(|code| matches!(*code, "OP-UNIT-DIMENSION-UNKNOWN" | "OP-UNIT-DIMENSION-MISMATCH")),
            "{}: {:?}", member.name, outcome.diagnostics);
        assert!(outcome.applied_model.is_none(), "{}", member.name);
    }
}

#[test]
fn a_pair_whose_unit_is_not_the_intents_is_refused() {
    let model = matrix_model();
    for member in family() {
        let before = member.stored_value.to_string();
        assert_refused(&model, &member, &before, &pair(replacement(&member), member.other_unit), member.stored_unit,
            member.dimension, "OP-QUANTITY-PAYLOAD-INVALID");
    }
}

#[test]
fn a_stale_before_value_is_refused_bare_or_paired() {
    let model = matrix_model();
    for member in family() {
        let stale = (member.stored_value * 2.0).to_string();
        let value = replacement(&member);
        for (after, unit) in [(value.to_string(), member.stored_unit), (pair(value, member.other_unit), member.other_unit)] {
            assert_refused(&model, &member, &stale, &after, unit, member.dimension, "OP-STALE-BEFORE-VALUE");
        }
    }
}

#[test]
fn sign_constraints_are_unchanged() {
    let model = matrix_model();
    for member in family() {
        let before = member.stored_value.to_string();
        for value in [0.0, -1.5] {
            for (after, unit) in [(value.to_string(), member.stored_unit), (pair(value, member.other_unit), member.other_unit)] {
                if member.positive_only {
                    assert_refused(&model, &member, &before, &after, unit, member.dimension, "OP-VALUE-NOT-POSITIVE");
                } else {
                    assert_stored(&model, &member, &after, unit, value, unit);
                }
            }
        }
    }
}

#[test]
fn a_primitive_whose_category_gives_no_single_dimension_is_not_edited_in_place() {
    // A retired pressure primitive, a hydrotest primitive stated as a
    // pressure, and a record with no category or target give none.
    let mut model = matrix_model();
    let shapes = model["load_cases"].as_array().unwrap().len() - 1;
    model["load_cases"][shapes]["primitive_loads"][2] =
        primitive("shape:retired", "pressure", element(OUTBOARD), "global_z", 5.0, "Pa", "pressure");
    model["load_cases"][shapes]["primitive_loads"][8] = json!({"id": "shape:bare", "magnitude": {"value": -190.0, "unit": "N"}, "dimension": "force"});
    model["load_cases"][shapes]["primitive_loads"][10] =
        primitive("shape:test-pressure", "hydrotest", element(OUTBOARD), "global_z", 5.0, "Pa", "pressure");
    for (index, before, unit, dimension) in [(2, "5", "Pa", "pressure"), (8, "-190", "N", "force"), (10, "5", "Pa", "pressure")] {
        let member = member("ungoverned primitive", "Load", SHAPES_CASE, "update_load", &shape(index), dimension, (0.0, unit), unit, ("m", "length"), false);
        for after in ["7".to_string(), pair(7.0, unit)] {
            assert_refused(&model, &member, before, &after, unit, dimension, "OP-UNIT-DIMENSION-UNKNOWN");
        }
    }
}

#[test]
fn a_primitive_whose_shape_the_solve_does_not_read_is_not_edited_in_place() {
    // A direction outside the solve's vocabulary, an element body load about a
    // rotational direction, and a support motion whose target DOF and
    // direction disagree: the solve refuses each, so none governs an edit.
    let mut model = matrix_model();
    let shapes = model["load_cases"].as_array().unwrap().len() - 1;
    model["load_cases"][shapes]["primitive_loads"][8] =
        primitive("shape:no-direction", "occasional", tip(), "bogus", -190.0, "N", "force");
    model["load_cases"][shapes]["primitive_loads"][2] =
        primitive("shape:line-about-z", "distributed_force", element(OUTBOARD), "rotation_z", -190.0, "N/m", "force_per_length");
    model["load_cases"][shapes]["primitive_loads"][4] =
        primitive("shape:settlement-disagrees", "imposed_displacement",
            json!({"type": "support", "support": "support:anchor", "dof": "UX"}), "UY", -0.002, "m", "displacement");
    for (index, before, unit, dimension) in [(8, "-190", "N", "force"), (2, "-190", "N/m", "force_per_length"), (4, "-0.002", "m", "displacement")] {
        let member = member("unread primitive", "Load", SHAPES_CASE, "update_load", &shape(index), dimension, (0.0, unit), unit, ("Pa", "pressure"), false);
        for after in ["7".to_string(), pair(7.0, unit)] {
            assert_refused(&model, &member, before, &after, unit, dimension, "OP-UNIT-DIMENSION-UNKNOWN");
        }
    }
}

// ---- Operation to stored document to solve --------------------------------

fn annulus_area(outside_diameter: f64, wall: f64) -> f64 {
    let bore = outside_diameter - 2.0 * wall;
    PI / 4.0 * (outside_diameter * outside_diameter - bore * bore)
}

fn annulus_polar_moment(outside_diameter: f64, wall: f64) -> f64 {
    let bore = outside_diameter - 2.0 * wall;
    PI / 32.0 * (outside_diameter.powi(4) - bore.powi(4))
}

/// The published value of one result kind at the tip for one case, in the
/// unit the product publishes it in.
fn tip_result(model: &Value, case: &str, kind: &str) -> (f64, String) {
    let envelope = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}),
        PreviewSolverMode::SparseInteractive).expect("a readable model");
    let result = serde_json::to_value(envelope).unwrap();
    let blocking: Vec<&Value> = result["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking").collect();
    assert!(blocking.is_empty(), "the solve was refused: {blocking:?}");
    let row = result["results"].as_array().unwrap().iter()
        .find(|row| row["kind"] == kind && row["entity_ref"] == "node:3" && row["basis_ref"]["ref_id"] == case)
        .unwrap_or_else(|| panic!("no {kind} at the tip for {case}"));
    (row["value"].as_f64().unwrap(), row["unit"].as_str().unwrap().to_string())
}

fn assert_close(actual: f64, expected: f64, what: &str) {
    let relative = ((actual - expected) / expected).abs();
    assert!(relative < 1.0e-9, "{what}: published {actual:e}, closed form {expected:e}, relative difference {relative:e}");
}

/// Tip axial displacement in mm: F times the sum of L / (E A) over the spans.
fn axial_mm(force: f64, modulus: f64, areas: [f64; 2]) -> f64 {
    1000.0 * force * (SPAN / (modulus * areas[0]) + SPAN / (modulus * areas[1]))
}

/// Tip twist in rad: T times the sum of L / (G J) over the spans.
fn twist_rad(torque: f64, shear_modulus: f64, polar_moments: [f64; 2]) -> f64 {
    torque * (SPAN / (shear_modulus * polar_moments[0]) + SPAN / (shear_modulus * polar_moments[1]))
}

fn start_areas() -> [f64; 2] {
    [annulus_area(OUTSIDE_DIAMETER, WALL); 2]
}

fn start_polar_moments() -> [f64; 2] {
    [annulus_polar_moment(OUTSIDE_DIAMETER, WALL); 2]
}

fn solved_member(name: &str) -> Member {
    family().into_iter().find(|member| member.name == name).unwrap_or_else(|| panic!("no family member {name}"))
}

/// Applies an explicit pair to the solved model and returns the stored
/// document, having checked the stored record.
fn enter_pair(model: &Value, member: &Member, value: f64, unit: &str) -> Value {
    assert_stored(model, member, &pair(value, unit), unit, value, unit)
}

/// The refused bare edit of each solved member leaves the document, and so
/// every published result, as it was.
fn assert_bare_other_unit_refused(model: &Value, member: &Member, value: f64, unit: &str) {
    assert_refused(model, member, &member.stored_value.to_string(), &value.to_string(), unit, member.dimension,
        "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
}

#[test]
fn the_start_model_publishes_its_closed_forms() {
    let model = solved_model();
    let (axial, unit) = tip_result(&model, AXIAL_CASE, "global_nodal_displacement_x");
    assert_eq!(unit, "mm");
    assert_close(axial, axial_mm(AXIAL_FORCE, ELASTIC_MODULUS, start_areas()), "axial displacement");
    let (twist, unit) = tip_result(&model, TORSION_CASE, "global_nodal_rotation_x");
    assert_eq!(unit, "rad");
    assert_close(twist, twist_rad(TORQUE, SHEAR_MODULUS, start_polar_moments()), "twist");
    let (growth, _) = tip_result(&model, THERMAL_CASE, "global_nodal_displacement_x");
    assert_close(growth, 1000.0 * EXPANSION * TEMPERATURE_RISE * 2.0 * SPAN, "thermal growth");
    let (hot_twist, _) = tip_result(&model, HOT_TORSION_CASE, "global_nodal_rotation_x");
    assert_close(hot_twist, twist_rad(TORQUE, HOT_SHEAR_MODULUS, start_polar_moments()), "twist at the hot point");
}

#[test]
fn elastic_modulus_entered_as_210_gpa_is_consumed_as_210_gpa() {
    // The issue's witness. The bare "210" with unit GPa was stored as 210 Pa
    // and the solve published a displacement a billion times too large.
    let model = solved_model();
    let member = solved_member("elastic modulus");
    assert_bare_other_unit_refused(&model, &member, 210.0, "GPa");

    let entered = enter_pair(&model, &member, 210.0, "GPa");
    let (axial, _) = tip_result(&entered, AXIAL_CASE, "global_nodal_displacement_x");
    assert_close(axial, axial_mm(AXIAL_FORCE, 210.0e9, start_areas()), "axial displacement at 210 GPa");

    // The control: 210 Pa is what the solve consumes when 210 Pa is what is
    // stored, so the defect was a changed input and never a display matter.
    let pascals = enter_pair(&model, &member, 210.0, "Pa");
    let (soft, _) = tip_result(&pascals, AXIAL_CASE, "global_nodal_displacement_x");
    assert_close(soft, axial_mm(AXIAL_FORCE, 210.0, start_areas()), "axial displacement at 210 Pa");
    assert_close(soft / axial, 1.0e9, "the ratio the units give");
}

#[test]
fn base_shear_modulus_entered_in_gpa_is_consumed_in_gpa() {
    let model = solved_model();
    let member = solved_member("base shear modulus");
    assert_bare_other_unit_refused(&model, &member, 79.3, "GPa");
    let entered = enter_pair(&model, &member, 79.3, "GPa");
    let (twist, _) = tip_result(&entered, TORSION_CASE, "global_nodal_rotation_x");
    assert_close(twist, twist_rad(TORQUE, 79.3e9, start_polar_moments()), "twist at 79.3 GPa");
}

#[test]
fn expansion_coefficient_entered_per_degree_fahrenheit_is_consumed_per_degree_fahrenheit() {
    let model = solved_model();
    let member = solved_member("expansion coefficient");
    assert_bare_other_unit_refused(&model, &member, 6.5e-6, "1/degF");
    let entered = enter_pair(&model, &member, 6.5e-6, "1/degF");
    let (growth, _) = tip_result(&entered, THERMAL_CASE, "global_nodal_displacement_x");
    // One Fahrenheit degree is 5/9 of a kelvin, so a coefficient per degF is 9/5 of it per kelvin.
    assert_close(growth, 1000.0 * (6.5e-6 * 9.0 / 5.0) * TEMPERATURE_RISE * 2.0 * SPAN, "thermal growth");
}

#[test]
fn temperature_point_shear_modulus_entered_in_gpa_is_consumed_in_gpa() {
    let model = solved_model();
    let member = solved_member("temperature-point shear modulus");
    assert_bare_other_unit_refused(&model, &member, 64.0, "GPa");
    let entered = enter_pair(&model, &member, 64.0, "GPa");
    let (twist, _) = tip_result(&entered, HOT_TORSION_CASE, "global_nodal_rotation_x");
    assert_close(twist, twist_rad(TORQUE, 64.0e9, start_polar_moments()), "twist at the hot point");
    // The base shear modulus and its case are untouched.
    let (base, _) = tip_result(&entered, TORSION_CASE, "global_nodal_rotation_x");
    assert_close(base, twist_rad(TORQUE, SHEAR_MODULUS, start_polar_moments()), "twist on the base modulus");
}

#[test]
fn inline_diameter_and_wall_entered_in_millimetres_are_consumed_in_millimetres() {
    let model = solved_model();
    let diameter = solved_member("inline outside diameter");
    let wall = solved_member("inline wall thickness");
    assert_bare_other_unit_refused(&model, &diameter, 168.3, "mm");
    assert_bare_other_unit_refused(&model, &wall, 7.11, "mm");

    // The outboard pipe's diameter and the inboard pipe's wall, one after the other.
    let entered = enter_pair(&enter_pair(&model, &diameter, 168.3, "mm"), &wall, 7.11, "mm");
    let areas = [annulus_area(OUTSIDE_DIAMETER, 0.00711), annulus_area(0.1683, WALL)];
    let (axial, _) = tip_result(&entered, AXIAL_CASE, "global_nodal_displacement_x");
    assert_close(axial, axial_mm(AXIAL_FORCE, ELASTIC_MODULUS, areas), "axial displacement");
    let polar_moments = [annulus_polar_moment(OUTSIDE_DIAMETER, 0.00711), annulus_polar_moment(0.1683, WALL)];
    let (twist, _) = tip_result(&entered, TORSION_CASE, "global_nodal_rotation_x");
    assert_close(twist, twist_rad(TORQUE, SHEAR_MODULUS, polar_moments), "twist");
}

#[test]
fn a_primitive_magnitude_entered_in_kilonewtons_is_consumed_in_kilonewtons() {
    let model = solved_model();
    let force = member("axial force", "Load", AXIAL_CASE, "update_load", &shape(0), "force", (AXIAL_FORCE, "N"), "kN", ("N*m", "moment"), false);
    assert_bare_other_unit_refused(&model, &force, 2.0, "kN");
    let entered = enter_pair(&model, &force, 2.0, "kN");
    let (axial, _) = tip_result(&entered, AXIAL_CASE, "global_nodal_displacement_x");
    assert_close(axial, axial_mm(2000.0, ELASTIC_MODULUS, start_areas()), "axial displacement under 2 kN");

    let rise = member("temperature rise", "Load", THERMAL_CASE, "update_load", &shape(1), "temperature_interval", (TEMPERATURE_RISE, "degC"), "degF", ("Pa", "temperature"), false);
    assert_bare_other_unit_refused(&model, &rise, 90.0, "degF");
    let entered = enter_pair(&model, &rise, 90.0, "degF");
    let (growth, _) = tip_result(&entered, THERMAL_CASE, "global_nodal_displacement_x");
    // The inboard span keeps its 100 K rise; the outboard span rises 90 degF, which is 50 K.
    assert_close(growth, 1000.0 * EXPANSION * (TEMPERATURE_RISE + 90.0 * 5.0 / 9.0) * SPAN, "thermal growth");
}

// The rule at the engine's other entries: the validate-only entry, and the
// atomic batch, where each step is judged against the document the earlier
// steps produced.

const MISMATCH: &str = "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE";

/// The current-model hash claim an atomic batch requires.
fn claim(model: &Value) -> Value {
    json!({"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_scope": "model_payload",
        "value": format!("sha256:{}", sha256_hex(&canonical_json(model)))})
}

/// An intent with its own identifiers, so that several can share a batch.
fn step(member: &Member, token: &str, before: &str, after: &str, unit: &str) -> Value {
    let mut operation = intent(member, before, after, unit, member.dimension);
    operation["operation_id"] = json!(format!("op:{token}"));
    operation["change"]["change_id"] = json!(format!("change:{token}"));
    operation
}

/// The batch is refused at step `index` with `code`, and publishes no model.
fn assert_batch_rolled_back(outcome: &Value, index: usize, code: &str, what: &str) {
    assert_eq!(outcome["simulation_disposition"], "rolled_back_no_model_published", "{what}");
    assert!(outcome["applied_model"].is_null(), "{what}: a refused batch published a model");
    assert!(outcome["applied_model_backend_hash"].is_null(), "{what}");
    assert!(outcome["acceptance"].is_null(), "{what}");
    let steps = outcome["operation_outcomes"].as_array().unwrap();
    assert_eq!(steps.len(), index + 1, "{what}: the batch stops at its first refusal");
    assert_eq!(steps[index]["simulation_status"], "blocked", "{what}");
    assert!(steps[index]["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == code && d["severity"] == "blocking"),
        "{what}: expected {code}, got {}", steps[index]["diagnostics"]);
}

#[test]
fn validating_a_bare_number_naming_another_unit_refuses_it_and_returns_no_model() {
    let model = matrix_model();
    for member in family() {
        let operation = intent(&member, &member.stored_value.to_string(), &replacement(&member).to_string(), member.other_unit, member.dimension);
        let outcome = validate_operation(&model, &operation, None);
        assert!(blocking(&outcome).contains(&MISMATCH), "{}: {:?}", member.name, outcome.diagnostics);
        assert!(outcome.applied_model.is_none(), "{}", member.name);
        assert_eq!(outcome.validation.unit_validation, "blocked", "{}", member.name);
        assert_names_stored_unit_and_pair(&outcome, &member);
    }
}

#[test]
fn a_batch_whose_step_is_a_bare_number_naming_another_unit_is_rolled_back_whole() {
    let model = matrix_model();
    let original = model.clone();
    for member in family() {
        let before = member.stored_value.to_string();
        let batch = json!({"batch_id": "batch:bare-other-unit",
            "operations": [step(&member, "bare-other-unit", &before, &replacement(&member).to_string(), member.other_unit)]});
        for outcome in [validate_operation_batch(&model, &batch, Some(&claim(&model))), apply_operation_batch(&model, &batch, Some(&claim(&model)))] {
            assert_batch_rolled_back(&outcome, 0, MISMATCH, member.name);
        }
    }
    assert_eq!(model, original);
}

#[test]
fn a_later_bare_number_is_judged_against_the_unit_an_earlier_step_of_the_batch_stored() {
    let model = matrix_model();
    let original = model.clone();
    for member in family() {
        let before = member.stored_value.to_string();
        let (in_stored_unit, in_other_unit) = member.replacement;
        // Step 1 stores the member in its other unit, as an explicit pair.
        let first = step(&member, "pair", &before, &pair(in_other_unit, member.other_unit), member.other_unit);
        // Step 2 is a bare number naming the unit the document held before step 1.
        let stale_unit = step(&member, "bare-first-unit", &in_other_unit.to_string(), &in_stored_unit.to_string(), member.stored_unit);
        let batch = json!({"batch_id": "batch:unit-then-bare", "operations": [first.clone(), stale_unit]});
        for outcome in [validate_operation_batch(&model, &batch, Some(&claim(&model))), apply_operation_batch(&model, &batch, Some(&claim(&model)))] {
            assert_batch_rolled_back(&outcome, 1, MISMATCH, member.name);
            let finding = outcome["operation_outcomes"][1]["diagnostics"].as_array().unwrap().iter().find(|d| d["code"] == MISMATCH).unwrap();
            assert!(finding["message"].as_str().unwrap().contains(&format!("stored unit `{}`", member.other_unit)),
                "{}: the refusal names the unit step 1 stored: {}", member.name, finding["message"]);
        }
        // The control: the same second step naming the unit step 1 stored is applied,
        // and the batch stores that number beside that unit.
        let second = step(&member, "bare-new-unit", &in_other_unit.to_string(), &in_other_unit.to_string(), member.other_unit);
        let batch = json!({"batch_id": "batch:unit-then-bare-control", "operations": [first, second]});
        let outcome = apply_operation_batch(&model, &batch, Some(&claim(&model)));
        assert_eq!(outcome["simulation_disposition"], "committed_as_one_batch", "{}: {}", member.name, outcome["diagnostics"]);
        assert_eq!(stored(&outcome["applied_model"], &member), json!({"value": in_other_unit, "unit": member.other_unit}), "{}", member.name);
    }
    assert_eq!(model, original);
}

/// A stored unit the catalog does not hold for the field's dimension is not
/// admitted by being repeated: a bare number naming that same unit is refused,
/// with the stored unit and the explicit pair named, and nothing is written.
/// The explicit pair in a unit of the field's dimension is the way out.
#[test]
fn a_stored_unit_outside_the_fields_dimension_is_refused_even_when_the_edit_repeats_it() {
    let elastic = family().into_iter().find(|member| member.name == "elastic modulus").unwrap();
    for stored_unit in ["furlong", "m"] {
        let mut model = matrix_model();
        model["materials"][0]["elastic_modulus"] = json!({"value": 200.0, "unit": stored_unit});
        for operation in [apply_operation, validate_operation] {
            let outcome = operation(&model, &intent(&elastic, "200", "210", stored_unit, elastic.dimension), None);
            assert!(blocking(&outcome).contains(&MISMATCH), "{stored_unit}: {:?}", outcome.diagnostics);
            assert!(outcome.applied_model.is_none(), "{stored_unit}");
            let finding = outcome.diagnostics.iter().find(|d| d.code == MISMATCH).unwrap();
            assert!(finding.message.contains(&format!("stored unit `{stored_unit}`")), "{stored_unit}: {}", finding.message);
            assert!(finding.remediation.contains("explicit JSON value/unit pair"), "{stored_unit}: {}", finding.remediation);
        }
        let outcome = apply_operation(&model, &intent(&elastic, "200", &pair(210.0, "GPa"), "GPa", elastic.dimension), None);
        assert!(blocking(&outcome).is_empty(), "{stored_unit}: {:?}", outcome.diagnostics);
        assert_eq!(stored(&outcome.applied_model.unwrap(), &elastic), json!({"value": 210.0, "unit": "GPa"}), "{stored_unit}");
    }
}

//! Model document 0.9.0, slice S1-core, at the applier (DEL-02-01 Design
//! `model-document-0.9.0.md` §1.3, §2.1, §2.2, §2.4): the document validator gives the
//! solve's code and message for every document-level rule, and a document carrying every
//! 0.9.0 member opens, validates and edits, keeping the members byte for byte. Every
//! document is built at the public `MODEL_SCHEMA_VERSION`. Invented data only.
use open_pipe_stress_operation_applier::document_validator::{validate_model_document, SLOT_SHAPE};
use open_pipe_stress_operation_applier::{apply_operation, canonical_json, sha256_hex};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};

const PROV: &str = "invented_s1_core_applier_test_input";

fn model() -> Value {
    let node = |id: &str, x: f64| json!({"id": id, "label": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV});
    let pipe = |id: &str, from: &str, to: &str| json!({"id": id, "label": id, "from": from, "to": to, "material": "mat",
        "y_reference": {"x": 0, "y": 0, "z": 1}, "section": {"outside_diameter": {"value": 0.2, "unit": "m"},
        "wall_thickness": {"value": 0.01, "unit": "m"}}, "provenance": PROV, "condition_set_ref": "cs:line"});
    let force = |id: &str, node: &str, direction: &str, value: f64| json!({"id": id, "category": "concentrated_force",
        "target": {"type": "node", "node": node}, "direction": direction, "magnitude": {"value": value, "unit": "N"},
        "dimension": "force", "provenance": PROV});
    json!({
        "schema_version": MODEL_SCHEMA_VERSION,
        "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": "invented:s1-core-applier", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC", "stress": "Pa"}},
        "nodes": [node("N0", 0.0), node("N1", 2.0), node("N2", 4.0)],
        "pipe_segments": [pipe("P1", "N0", "N1"), pipe("P2", "N1", "N2")],
        "materials": [{"id": "mat", "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "shear_modulus": {"value": 8.0e10, "unit": "Pa"}, "provenance": PROV}],
        "supports": [
            {"id": "anchor", "node": "N0", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
            {"id": "spring", "node": "N2", "family": "spring", "restraints": ["UX"], "stiffness": {"dof": "UX", "value": {"value": 1.0e6, "unit": "N/m"}},
             "direction": {"x": 0.0, "y": 3.0, "z": 4.0}, "provenance": PROV},
            {"id": "snub", "node": "N1", "family": "snubber", "restraints": ["UY"], "stiffness": {"dof": "UY", "value": {"value": 1.0e7, "unit": "N/m"}}, "provenance": PROV},
        ],
        "load_cases": [
            {"id": "L1", "primitive_loads": [force("L1:x", "N2", "global_x", 1000.0)], "provenance": PROV,
             "condition_use": {"temperature": "OPE1", "added_weight": true}},
            {"id": "L2", "primitive_loads": [force("L2:y", "N2", "global_y", -300.0)], "provenance": PROV,
             "test_condition": {"kind": "hydrostatic", "held_supports": "springs_and_constant_effort"}},
        ],
        "combinations": [
            {"id": "C-EXP", "basis": "result_state_subtraction", "minuend_id": "L1", "subtrahend_id": "L2",
             "difference_basis": "operating_minus_sustained", "subtrahend_support_state": "minuend_final", "provenance": PROV},
            {"id": "C-ABS", "basis": "absolute_sum", "terms": [{"load_case": "L1", "factor": 1.0}, {"combination": "C-EXP", "factor": 1.0}], "provenance": PROV},
            {"id": "C-SRSS", "basis": "srss", "terms": [{"combination": "C-ABS", "factor": 1.0}], "provenance": PROV},
        ],
        "condition_sets": [{"id": "cs:line", "label": "Line", "provenance": PROV, "tags": ["line"], "notes": "invented",
            "conditions": [{"key": "OPE1", "temperature": {"value": 150.0, "unit": "degC"}, "pressure": {"value": 1.5, "unit": "MPa"}}],
            "installation_temperature": {"value": 20.0, "unit": "degC"},
            "added_weight": {"magnitude": {"value": -50.0, "unit": "N/m"}, "direction": "global_z"},
            "exposure": {"wind": true}}],
        "spectra": [{"id": "sp:g", "abscissa": "frequency", "ordinate": "g_multiple", "units": {"abscissa": "Hz"},
            "g_value": {"value": 9.80665, "unit": "m/s^2"}, "damping_ratio": 0.05, "interpolation": {"abscissa": "log", "ordinate": "log"},
            "points": [[1.0, 0.2], [10.0, 0.5]], "tags": ["dynamics"]}],
    })
}

/// The members 0.9.0 adds, as they sit in `model()`.
fn members(document: &Value) -> Value {
    json!({
        "condition_sets": document["condition_sets"], "spectra": document["spectra"],
        "refs": document["pipe_segments"].as_array().unwrap().iter().map(|p| p["condition_set_ref"].clone()).collect::<Vec<_>>(),
        "uses": document["load_cases"].as_array().unwrap().iter().map(|c| json!([c.get("condition_use"), c.get("test_condition")])).collect::<Vec<_>>(),
        "supports": document["supports"], "combinations": document["combinations"],
    })
}

fn blocking(document: &Value) -> Vec<(String, String)> {
    validate_model_document(document).into_iter().filter(|d| d.is_blocking()).map(|d| (d.code.to_string(), d.message)).collect()
}

/// A document carrying every 0.9.0 member, interim ones included, validates with no
/// blocking finding (the interim members are refused at solve, not at open), and an
/// unrelated edit keeps every member byte for byte.
#[test]
fn a_document_carrying_every_member_opens_validates_and_edits_byte_for_byte() {
    let document = model();
    assert_eq!(blocking(&document), Vec::<(String, String)>::new());
    let claim = json!({"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_scope": "model_payload",
        "value": format!("sha256:{}", sha256_hex(&canonical_json(&document)))});
    let intent = json!({
        "operation_id": "operation:rename", "operation_kind": "modify", "operation_status": "proposed", "author_type": "user",
        "target": {"object_type": "Node", "ref": "N1"},
        "change": {"change_id": "change:rename", "change_kind": "set_field", "field_label": "label", "field_path": "label",
                   "before": "N1", "after": "Midspan", "unit": "none", "dimension": "dimensionless", "source_note": "invented"},
        "validation": {"schema_validation": "not_run", "constraint_validation": "not_run", "unit_validation": "not_run",
                       "diff_preview_status": "not_generated", "application_status": "not_applied"},
        "audit_boundary": {"mutation_route": "structured_operations_only", "direct_model_mutation_allowed": false,
                           "requires_user_acceptance": true, "mutates_accepted_model_state": false},
        "professional_boundary": {"human_review_required": true, "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false, "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false, "software_makes_authentication_claim": false},
        "rationale": "Invented edit",
    });
    let outcome = apply_operation(&document, &intent, Some(&claim));
    let applied = outcome.applied_model.unwrap_or_else(|| panic!("{:?}", outcome.diagnostics));
    assert_eq!(applied["nodes"][1]["label"], "Midspan");
    assert_eq!(canonical_json(&members(&applied)), canonical_json(&members(&document)));
    let mut reverted = applied.clone();
    reverted["nodes"][1]["label"] = json!("N1");
    assert_eq!(canonical_json(&reverted), canonical_json(&document), "nothing else moved");
}

fn solve_codes(document: &Value) -> Vec<(String, String)> {
    let envelope = run_linear_static_preview_value_with_mode(json!({"model": document, "materials": []}), PreviewSolverMode::SparseInteractive)
        .unwrap_or_else(|refusal| panic!("{refusal}"));
    let envelope = serde_json::to_value(envelope).unwrap();
    envelope["diagnostics"].as_array().unwrap().iter()
        .map(|d| (d["code"].as_str().unwrap().to_string(), d["message"].as_str().unwrap().to_string()))
        .collect()
}

/// A valid base for the parity cases: every member, none interim.
fn admissible() -> Value {
    let mut document = model();
    document["supports"].as_array_mut().unwrap().truncate(2);
    document["supports"][1].as_object_mut().unwrap().remove("direction");
    for case in document["load_cases"].as_array_mut().unwrap() {
        let case = case.as_object_mut().unwrap();
        case.remove("condition_use");
        case.remove("test_condition");
    }
    document["combinations"] = json!([]);
    document
}

/// Each document-level rule: the validator blocks with exactly the code and message the
/// solve gives (the `shear_model` precedent).
#[test]
fn the_validator_gives_the_solves_code_and_message_for_every_document_level_rule() {
    let cases: Vec<(&str, Box<dyn Fn(&mut Value)>)> = vec![
        ("OPERATING_CONDITION_KEY_DUPLICATE", Box::new(|d| d["condition_sets"][0]["conditions"].as_array_mut().unwrap().push(json!({"key": "OPE1"})))),
        ("OPERATING_CONDITION_QUANTITY_INVALID", Box::new(|d| d["condition_sets"][0]["conditions"][0]["pressure"]["unit"] = json!("degC"))),
        ("OPERATING_CONDITION_QUANTITY_INVALID", Box::new(|d| d["condition_sets"][0]["installation_temperature"]["value"] = json!(-500.0))),
        ("OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED", Box::new(|d| d["condition_sets"][0]["installation_temperature"]["value"] = json!(2.0e6))),
        ("OPERATING_CONDITION_PROVENANCE_MISSING", Box::new(|d| d["condition_sets"][0]["provenance"] = json!(""))),
        ("OPERATING_CONDITION_SET_UNRESOLVED", Box::new(|d| d["pipe_segments"][0]["condition_set_ref"] = json!("cs:other"))),
        ("LOAD_CASE_CONDITION_USE_EMPTY", Box::new(|d| d["load_cases"][0]["condition_use"] = json!({}))),
        ("LOAD_CASE_CONDITION_USE_EMPTY", Box::new(|d| d["load_cases"][0]["condition_use"] = json!({"added_weight": false}))),
        ("DUPLICATE_ID", Box::new(|d| { let set = d["condition_sets"][0].clone(); d["condition_sets"].as_array_mut().unwrap().push(set); })),
        ("EMPTY_ID", Box::new(|d| d["condition_sets"][0]["id"] = json!(" "))),
        ("SUPPORT_DIRECTION_INVALID", Box::new(|d| d["supports"][1]["direction"] = Value::Null)),
        ("SUPPORT_DIRECTION_ZERO", Box::new(|d| d["supports"][1]["direction"] = json!({"x": 0, "y": 0, "z": 0}))),
        ("SUPPORT_DIRECTION_RECORD_UNSUPPORTED", Box::new(|d| d["supports"][0]["direction"] = json!({"x": 1, "y": 0, "z": 0}))),
        ("SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED", Box::new(|d| { d["supports"][1]["direction"] = json!({"x": 1, "y": 0, "z": 0});
            d["supports"][1]["restraints"] = json!(["RX"]); d["supports"][1]["stiffness"] = json!({"dof": "RX", "value": {"value": 1.0, "unit": "N*m/rad"}}); })),
        ("SNUBBER_ROTATIONAL_RESTRAINT_UNSUPPORTED", Box::new(|d| d["supports"].as_array_mut().unwrap().push(
            json!({"id": "snub", "node": "N1", "family": "snubber", "restraints": ["RX"], "provenance": PROV})))),
        ("SNUBBER_STIFFNESS_INVALID", Box::new(|d| d["supports"].as_array_mut().unwrap().push(
            json!({"id": "snub", "node": "N1", "family": "snubber", "restraints": ["UY"], "stiffness": {"dof": "UZ", "value": {"value": 1.0, "unit": "N/m"}}, "provenance": PROV})))),
        ("SNUBBER_MEMBER_CONFLICT", Box::new(|d| d["supports"].as_array_mut().unwrap().push(
            json!({"id": "snub", "node": "N1", "family": "snubber", "restraints": ["UY"], "hanger": {"hanger_type": "spring_hanger"}, "provenance": PROV})))),
        ("LOAD_COMBINATION_DIFFERENCE_BASIS_MISPLACED", Box::new(|d| d["combinations"] = json!([{"id": "C", "basis": "range_envelope",
            "operand_ids": ["L1"], "mode": "max", "difference_basis": "operating_minus_sustained", "provenance": PROV}]))),
        ("LOAD_COMBINATION_DIFFERENCE_BASIS_UNKNOWN", Box::new(|d| d["combinations"] = json!([{"id": "C", "basis": "result_state_subtraction",
            "minuend_id": "L1", "subtrahend_id": "L2", "difference_basis": null, "provenance": PROV}]))),
        ("LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_MISPLACED", Box::new(|d| d["combinations"] = json!([{"id": "C", "basis": "result_state_subtraction",
            "minuend_id": "L1", "subtrahend_id": "L2", "subtrahend_support_state": "minuend_final", "provenance": PROV}]))),
        ("LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNKNOWN", Box::new(|d| d["combinations"] = json!([{"id": "C", "basis": "result_state_subtraction",
            "minuend_id": "L1", "subtrahend_id": "L2", "difference_basis": "operating_minus_sustained", "subtrahend_support_state": "held", "provenance": PROV}]))),
        ("LOAD_COMBINATION_OPERAND_UNKNOWN", Box::new(|d| d["combinations"] = json!([{"id": "C", "basis": "result_state_subtraction",
            "minuend_id": "L1", "subtrahend_id": "L9", "provenance": PROV}]))),
        ("LOAD_COMBINATION_OPERAND_CYCLE", Box::new(|d| d["combinations"] = json!([
            {"id": "C-A", "basis": "mechanics", "terms": [{"combination": "C-B", "factor": 1.0}], "provenance": PROV},
            {"id": "C-B", "basis": "srss", "terms": [{"combination": "C-A", "factor": 1.0}], "provenance": PROV}]))),
        ("LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE", Box::new(|d| d["combinations"] = json!([
            {"id": "C-S", "basis": "mechanics", "terms": [{"load_case": "L1", "factor": 1.0}], "provenance": PROV},
            {"id": "C", "basis": "result_state_subtraction", "minuend_id": "L1", "subtrahend_id": "C-S", "difference_basis": "operating_minus_sustained", "provenance": PROV}]))),
        ("SPECTRUM_RECORD_INVALID", Box::new(|d| d["spectra"][0]["points"] = json!([[2.0, 0.2], [1.0, 0.5]]))),
        ("SPECTRUM_ID_DUPLICATE", Box::new(|d| { let spectrum = d["spectra"][0].clone(); d["spectra"].as_array_mut().unwrap().push(spectrum); })),
        ("EMPTY_ID", Box::new(|d| d["spectra"][0]["id"] = json!(""))),
    ];
    assert_eq!(blocking(&admissible()), Vec::<(String, String)>::new());
    for (code, mutate) in cases {
        let mut document = admissible();
        mutate(&mut document);
        let validator: Vec<_> = blocking(&document).into_iter().filter(|(c, _)| c == code).collect();
        assert!(!validator.is_empty(), "{code}: {:?}", blocking(&document));
        let solve = solve_codes(&document);
        for finding in &validator {
            assert!(solve.contains(finding), "{code}: the solve gives {finding:?}; it gave {solve:?}");
        }
    }
}

/// A malformed new member is the validator's shape finding; the solve refuses it at decode.
#[test]
fn a_malformed_new_member_is_a_slot_shape_finding() {
    for mutate in [
        Box::new(|d: &mut Value| d["condition_sets"][0]["label"] = Value::Null) as Box<dyn Fn(&mut Value)>,
        Box::new(|d: &mut Value| d["condition_sets"][0]["extra"] = json!(1)),
        Box::new(|d: &mut Value| d["pipe_segments"][0]["condition_set_ref"] = json!(3)),
        Box::new(|d: &mut Value| d["load_cases"][0]["condition_use"] = json!({"temperature": null})),
        Box::new(|d: &mut Value| d["load_cases"][1]["test_condition"] = json!({"kind": "pneumatic", "held_supports": "none"})),
        Box::new(|d: &mut Value| d["supports"][1]["direction"] = json!({"x": 1, "y": 0, "z": 0, "w": 0})),
        Box::new(|d: &mut Value| d["spectra"][0]["units"]["ordinate"] = Value::Null),
        Box::new(|d: &mut Value| d["spectra"] = json!({})),
    ] {
        let mut document = model();
        mutate(&mut document);
        let found = blocking(&document);
        assert!(found.iter().any(|(code, _)| code == SLOT_SHAPE), "{found:?}");
        assert!(run_linear_static_preview_value_with_mode(json!({"model": document, "materials": []}), PreviewSolverMode::SparseInteractive).is_err());
    }
}

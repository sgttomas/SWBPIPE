//! Authoring a hydrostatic test case through the operation route (DEL-05-01
//! REQ-014, AC-010): `set_load_case_test_condition` and
//! `clear_load_case_test_condition`, and a test case's contents selection
//! (`set_load_case_condition_use` with `contents_density` alone) on a document
//! with a pressure contract, each previewed, applied and undone by the inverse
//! its own diff row states, with the solve's judgement on every candidate.
//!
//! Every document is an invented inline fixture at the public
//! `MODEL_SCHEMA_VERSION`; every value is invented.
use open_pipe_stress_operation_applier::document_validator::validate_model_document;
use open_pipe_stress_operation_applier::{apply_operation, canonical_json, sha256_hex, validate_operation, OperationOutcome};
use open_pipe_stress_product_physics::operating_conditions::preview_operating_conditions;
use open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION;
use serde_json::{json, Value};

const PROV: &str = "invented_hydrostatic_test_authoring_input";
const HELD: &str = "springs_and_constant_effort";

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// An exact-route document: two spans along X on a pinned end, a guided end and
/// a middle spring; one condition set with a test condition; two cases with
/// explicit empty pressure regions.
fn exact() -> Value {
    let node = |id: &str, x: f64| json!({"id": id, "label": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV});
    let pipe = |id: &str, from: &str, to: &str| json!({"id": id, "from": from, "to": to, "material": "mat", "condition_set_ref": "cs:line",
        "y_reference": {"x": 0, "y": 0, "z": 1},
        "section": {"outside_diameter": q(0.29, "m"), "wall_thickness": q(0.0135, "m"), "corrosion_allowance": q(0.0021, "m"),
                    "material_density": q(7830.0, "kg/m^3"), "contents_density": q(865.0, "kg/m^3")},
        "provenance": PROV});
    json!({
        "schema_version": MODEL_SCHEMA_VERSION,
        "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": "invented:hydrostatic-authoring", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC", "stress": "Pa"}},
        "nodes": [node("node:a", 0.0), node("node:b", 4.3), node("node:c", 8.6)],
        "pipe_segments": [pipe("pipe:a-b", "node:a", "node:b"), pipe("pipe:b-c", "node:b", "node:c")],
        "materials": [{"id": "mat", "elastic_modulus": q(2.03e11, "Pa"), "poisson_ratio": q(0.29, "1"),
                       "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "provenance": PROV}],
        "condition_sets": [{"id": "cs:line", "label": "Line", "provenance": PROV,
                            "conditions": [{"key": "HYDRO", "contents_density": q(1015.0, "kg/m^3")}, {"key": "DRY"}]}],
        "supports": [
            {"id": "support:a", "node": "node:a", "family": "guide", "restraints": ["UX", "UY", "UZ", "RX"], "provenance": PROV},
            {"id": "support:b", "node": "node:b", "family": "spring", "restraints": ["UZ"], "stiffness": {"dof": "UZ", "value": q(1.35e6, "N/m")}, "provenance": PROV},
            {"id": "support:c", "node": "node:c", "family": "guide", "restraints": ["UY", "UZ"], "provenance": PROV},
        ],
        "load_cases": [
            {"id": "SUS", "label": "Sustained", "kind": "primitive_user_load", "status": "draft", "primitive_loads": [], "pressure_regions": [], "provenance": PROV},
            {"id": "HT", "label": "Hydrotest", "kind": "primitive_user_load", "status": "draft", "primitive_loads": [], "pressure_regions": [], "provenance": PROV},
        ],
        "combinations": [],
    })
}

fn pressure_free() -> Value {
    let mut model = exact();
    model.as_object_mut().unwrap().remove("pressure_contract");
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"].as_object_mut().unwrap().remove("corrosion_allowance");
    }
    for case in model["load_cases"].as_array_mut().unwrap() {
        case.as_object_mut().unwrap().remove("pressure_regions");
    }
    model["materials"] = json!([{"id": "mat", "elastic_modulus": q(2.03e11, "Pa"), "shear_modulus": q(7.868e10, "Pa"), "provenance": PROV}]);
    model
}

fn bytes(model: &Value) -> String {
    canonical_json(model)
}

fn claim(model: &Value) -> Value {
    json!({"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_scope": "model_payload",
        "value": format!("sha256:{}", sha256_hex(&canonical_json(model)))})
}

fn op(kind: &str, change_kind: &str, object: &str, target: &str, path: &str, before: &str, after: &str) -> Value {
    let token = format!("{change_kind}:{target}:{path}:{}", &sha256_hex(&format!("{before}|{after}"))[..12]);
    json!({
        "operation_id": format!("operation:{token}"), "operation_kind": kind, "operation_status": "proposed", "author_type": "user",
        "target": {"object_type": object, "ref": target},
        "change": {"change_id": format!("change:{token}"), "change_kind": change_kind, "field_label": path, "field_path": path,
                   "before": before, "after": after, "unit": "none", "dimension": "dimensionless", "source_note": "invented test input"},
        "validation": {"schema_validation": "not_run", "constraint_validation": "not_run", "unit_validation": "not_run",
                       "diff_preview_status": "not_generated", "application_status": "not_applied"},
        "audit_boundary": {"mutation_route": "structured_operations_only", "direct_model_mutation_allowed": false,
                           "requires_user_acceptance": true, "mutates_accepted_model_state": false},
        "professional_boundary": {"human_review_required": true, "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false, "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false, "software_makes_authentication_claim": false},
        "rationale": "Invented hydrostatic test authoring operation",
    })
}

fn case<'a>(model: &'a Value, id: &str) -> &'a Value {
    model["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap()
}

fn member(model: &Value, id: &str, name: &str) -> String {
    model["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == id).and_then(|c| c.get(name)).map(canonical_json).unwrap_or_else(|| "not_present".into())
}

fn set_test(model: &Value, id: &str, held: &str) -> Value {
    op("modify", "set_load_case_test_condition", "Load", id, "test_condition", &member(model, id, "test_condition"),
        &json!({"kind": "hydrostatic", "held_supports": held}).to_string())
}

fn clear_test(model: &Value, id: &str) -> Value {
    op("modify", "clear_load_case_test_condition", "Load", id, "test_condition", &member(model, id, "test_condition"), "not_present")
}

fn set_use(model: &Value, id: &str, using: &Value) -> Value {
    op("modify", "set_load_case_condition_use", "Load", id, "condition_use", &member(model, id, "condition_use"), &using.to_string())
}

fn clear_use(model: &Value, id: &str) -> Value {
    op("modify", "clear_load_case_condition_use", "Load", id, "condition_use", &member(model, id, "condition_use"), "not_present")
}

fn records(outcome: &OperationOutcome) -> Vec<(String, String)> {
    outcome.diagnostics.iter().map(|d| (d.code.clone(), d.severity.clone())).collect()
}

fn codes_at(outcome: &OperationOutcome, severity: &str) -> Vec<String> {
    let mut found: Vec<String> = outcome.diagnostics.iter().filter(|d| d.severity == severity).map(|d| d.code.clone()).collect();
    found.sort();
    found
}

/// Validate (preview), then apply: the same diagnostics and diff, a document only
/// from apply, the input never mutated, the result admitted by the validator.
fn applied(model: &Value, intent: &Value) -> (Value, OperationOutcome) {
    let original = bytes(model);
    let preview = validate_operation(model, intent, Some(&claim(model)));
    assert!(preview.applied_model.is_none());
    let outcome = apply_operation(model, intent, Some(&claim(model)));
    assert_eq!(records(&preview), records(&outcome), "preview reports exactly what apply reports");
    assert_eq!(preview.diff_preview, outcome.diff_preview);
    let next = outcome.applied_model.clone().unwrap_or_else(|| panic!("not applied: {:?}", records(&outcome)));
    assert_eq!(outcome.validation.application_status, "applied_to_session_model");
    assert!(codes_at(&outcome, "blocking").is_empty(), "{:?}", records(&outcome));
    assert!(!validate_model_document(&next).iter().any(|d| d.is_blocking()));
    assert_eq!(bytes(model), original, "the input is never mutated");
    (next, outcome)
}

fn refused(model: &Value, intent: &Value, code: &str) -> OperationOutcome {
    let original = bytes(model);
    let mut last = None;
    for outcome in [validate_operation(model, intent, Some(&claim(model))), apply_operation(model, intent, Some(&claim(model)))] {
        assert!(outcome.applied_model.is_none(), "{code}: applied unexpectedly");
        assert!(codes_at(&outcome, "blocking").iter().any(|found| found == code), "{code} not in {:?}", records(&outcome));
        last = Some(outcome);
    }
    assert_eq!(bytes(model), original);
    last.unwrap()
}

/// The inverse of an applied modify, from its diff row alone.
fn inverse(intent: &Value, outcome: &OperationOutcome) -> Value {
    let row = &outcome.diff_preview[0];
    let kind = intent["change"]["change_kind"].as_str().unwrap();
    let (set, clear) = if kind.ends_with("test_condition") {
        ("set_load_case_test_condition", "clear_load_case_test_condition")
    } else {
        ("set_load_case_condition_use", "clear_load_case_condition_use")
    };
    let change_kind = if row.before == "not_present" { clear } else { set };
    op("modify", change_kind, "Load", &row.entity_ref, &row.field_path, &row.after, &row.before)
}

/// Both orders of authoring a test case, previewed, applied and undone: the
/// declaration first (staged: no contents yet), then the contents selection
/// (staged: no generated self-weight yet), and back by the inverses to the
/// original bytes. The document stays admitted by the validator at each step.
#[test]
fn a_test_case_is_declared_and_given_contents_and_undone() {
    let model = exact();
    let declare = set_test(&model, "HT", HELD);
    let (declared, outcome) = applied(&model, &declare);
    assert_eq!(case(&declared, "HT")["test_condition"], json!({"kind": "hydrostatic", "held_supports": HELD}));
    assert_eq!(codes_at(&outcome, "warning"), ["LOAD_CASE_TEST_CONTENTS_UNSELECTED"], "{:?}", records(&outcome));
    let row = &outcome.diff_preview[0];
    assert_eq!((row.entity_ref.as_str(), row.object_type.as_str(), row.field_path.as_str(), row.before.as_str()), ("HT", "Load", "test_condition", "not_present"));
    assert_eq!(row.after, canonical_json(&json!({"kind": "hydrostatic", "held_supports": HELD})));

    let select = set_use(&declared, "HT", &json!({"contents_density": "HYDRO"}));
    let (selected, outcome2) = applied(&declared, &select);
    assert_eq!(case(&selected, "HT")["condition_use"], json!({"contents_density": "HYDRO"}));
    assert_eq!(codes_at(&outcome2, "warning"), ["LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID"], "{:?}", records(&outcome2));
    assert_eq!(preview_operating_conditions(&selected)["resolved"], json!(false), "staged: the solve still refuses the case by name");

    // The held choice changes in place; its inverse restores it.
    let change = set_test(&selected, "HT", "none");
    let (changed, outcome3) = applied(&selected, &change);
    assert_eq!(case(&changed, "HT")["test_condition"]["held_supports"], "none");
    let (back, _) = applied(&changed, &inverse(&change, &outcome3));
    assert_eq!(bytes(&back), bytes(&selected));

    // Undo in reverse order, each by the inverse its own row states.
    let (undone, _) = applied(&selected, &inverse(&select, &outcome2));
    assert_eq!(bytes(&undone), bytes(&declared));
    let (undone, _) = applied(&undone, &inverse(&declare, &outcome));
    assert_eq!(bytes(&undone), bytes(&model), "undo restores the original bytes");

    // The other order: the selection is refused until the case is a test.
    refused(&model, &set_use(&model, "HT", &json!({"contents_density": "HYDRO"})), "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
    // Clearing the declaration while the selection stands would leave an
    // ordinary case using contents on the exact route: refused, by the route code.
    refused(&selected, &clear_test(&selected, "HT"), "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
    let (unselected, _) = applied(&selected, &clear_use(&selected, "HT"));
    let (cleared, _) = applied(&unselected, &clear_test(&unselected, "HT"));
    assert_eq!(bytes(&cleared), bytes(&model));
}

/// Save and reopen: the authored members survive the canonical bytes unchanged,
/// and the reopened document judges exactly as before.
#[test]
fn the_authored_test_case_survives_save_and_reopen() {
    let model = exact();
    let (declared, _) = applied(&model, &set_test(&model, "HT", HELD));
    let (selected, _) = applied(&declared, &set_use(&declared, "HT", &json!({"contents_density": "HYDRO"})));
    let saved = bytes(&selected);
    let reopened: Value = serde_json::from_str(&saved).unwrap();
    assert_eq!(bytes(&reopened), saved);
    assert_eq!(case(&reopened, "HT")["test_condition"], json!({"kind": "hydrostatic", "held_supports": HELD}));
    assert_eq!(case(&reopened, "HT")["condition_use"], json!({"contents_density": "HYDRO"}));
    assert_eq!(preview_operating_conditions(&reopened), preview_operating_conditions(&selected));
}

/// The route and shape refusals of the two operations, and of a contents
/// selection on the exact route.
#[test]
fn the_refusals_of_the_test_condition_operations() {
    let model = exact();
    let (declared, _) = applied(&model, &set_test(&model, "HT", HELD));
    refused(&declared, &set_test(&declared, "HT", HELD), "OP-DATA-UNCHANGED");
    refused(&model, &clear_test(&model, "HT"), "OP-DATA-UNCHANGED");
    refused(&model, &set_test(&model, "NOPE", HELD), "OP-TARGET-NOT-FOUND");
    let mut stale = set_test(&declared, "HT", "none");
    stale["change"]["before"] = json!("not_present");
    refused(&declared, &stale, "OP-STALE-BEFORE-VALUE");
    for after in ["[]", r#""hydrostatic""#, "nope"] {
        let mut bad = set_test(&model, "HT", HELD);
        bad["change"]["after"] = json!(after);
        refused(&model, &bad, "OP-DATA-PAYLOAD-INVALID");
    }
    let mut clear_with_value = clear_test(&declared, "HT");
    clear_with_value["change"]["after"] = json!(json!({"kind": "hydrostatic", "held_supports": HELD}).to_string());
    refused(&declared, &clear_with_value, "OP-DATA-SHAPE-INVALID");
    let mut wrong_path = set_test(&model, "HT", HELD);
    wrong_path["change"]["field_path"] = json!("condition_use");
    assert!(apply_operation(&model, &wrong_path, Some(&claim(&model))).applied_model.is_none());
    // Its shape is the document validator's: both members, closed, known values.
    for bad in [json!({"kind": "hydrostatic"}), json!({"held_supports": HELD}), json!({"kind": "pneumatic", "held_supports": HELD}),
                json!({"kind": "hydrostatic", "held_supports": "all"}), json!({"kind": "hydrostatic", "held_supports": HELD, "pressure": 1})] {
        let mut intent = set_test(&model, "HT", HELD);
        intent["change"]["after"] = json!(bad.to_string());
        let outcome = apply_operation(&model, &intent, Some(&claim(&model)));
        assert!(outcome.applied_model.is_none(), "{bad}: applied");
        assert!(!codes_at(&outcome, "blocking").is_empty(), "{bad}");
    }
    // A contents selection on the exact route: only on a test case and only alone.
    refused(&declared, &set_use(&declared, "SUS", &json!({"contents_density": "HYDRO"})), "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
    for using in [json!({"contents_density": "HYDRO", "temperature": "HYDRO"}), json!({"contents_density": "HYDRO", "added_weight": true}), json!({"pressure": "HYDRO"})] {
        refused(&declared, &set_use(&declared, "HT", &using), "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
    }
    // A selected key the set lacks, or a condition without contents, is staged.
    let (missing, outcome) = applied(&declared, &set_use(&declared, "HT", &json!({"contents_density": "NOPE"})));
    let _ = missing;
    assert!(codes_at(&outcome, "warning").contains(&"LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID".to_string()), "{:?}", records(&outcome));
}

/// The pressure-free route: a test condition is not authored; one the document
/// already carries can be cleared.
#[test]
fn the_pressure_free_route_refuses_a_test_condition() {
    let model = pressure_free();
    refused(&model, &set_test(&model, "HT", "none"), "LOAD_CASE_TEST_CONDITION_PRESSURE_FREE");
    let create = op("create", "create_load_case", "Load", "HT2", "load_cases", "not_present",
        &json!({"id": "HT2", "label": "Test", "kind": "primitive_user_load", "status": "draft", "provenance": PROV, "primitive_loads": [],
                "test_condition": {"kind": "hydrostatic", "held_supports": "none"}}).to_string());
    refused(&model, &create, "LOAD_CASE_TEST_CONDITION_PRESSURE_FREE");
    let mut carrying = model.clone();
    carrying["load_cases"][1]["test_condition"] = json!({"kind": "hydrostatic", "held_supports": "none"});
    let (cleared, _) = applied(&carrying, &clear_test(&carrying, "HT"));
    assert_eq!(bytes(&cleared), bytes(&model));
}

/// `create_load_case` admits a test condition and its contents selection on the
/// exact route by the same route admission, and the candidate is judged by the
/// solve. A created case carries no `pressure_regions` (they are authored by
/// `update_load`), so on the exact route the candidate is refused for them by
/// name; the test condition is then set on the case with
/// `set_load_case_test_condition`. On the pressure-free route the test
/// condition is refused at admission.
#[test]
fn create_load_case_admits_a_test_condition_by_the_same_route() {
    let model = exact();
    let payload = json!({"id": "HT2", "label": "Test", "kind": "primitive_user_load", "status": "draft", "provenance": PROV, "primitive_loads": [],
        "test_condition": {"kind": "hydrostatic", "held_supports": "none"}, "condition_use": {"contents_density": "HYDRO"}});
    let outcome = refused(&model, &op("create", "create_load_case", "Load", "HT2", "load_cases", "not_present", &payload.to_string()), "EXACT_PRESSURE_REGIONS_REQUIRED");
    assert!(!codes_at(&outcome, "blocking").iter().any(|code| code.starts_with("LOAD_CASE_")), "admitted by the route: {:?}", records(&outcome));
    let mut ordinary = payload.clone();
    ordinary.as_object_mut().unwrap().remove("test_condition");
    refused(&model, &op("create", "create_load_case", "Load", "HT2", "load_cases", "not_present", &ordinary.to_string()), "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
}

/// A replacement that cannot be formed, or a combination over the test case,
/// is refused at authoring by name.
#[test]
fn held_support_and_combination_refusals_block_authoring() {
    let mut model = exact();
    model["supports"].as_array_mut().unwrap().push(json!({"id": "support:b-ce", "node": "node:b", "family": "constant_effort_support", "restraints": ["UZ"],
        "hanger": {"hanger_type": "constant_effort_support", "constant_load": q(5600.0, "N"), "travel_range": q(0.05, "m"), "source_reference": "invented"},
        "provenance": PROV}));
    refused(&model, &set_test(&model, "HT", HELD), "LOAD_CASE_TEST_HELD_SUPPORT_DUPLICATE");
    // With `none` nothing is held, so nothing is duplicated.
    applied(&model, &set_test(&model, "HT", "none"));

    let mut rigid = exact();
    rigid["supports"].as_array_mut().unwrap().push(json!({"id": "support:b-rigid", "node": "node:b", "family": "vertical_support", "restraints": ["UZ"], "provenance": PROV}));
    refused(&rigid, &set_test(&rigid, "HT", HELD), "LOAD_CASE_TEST_HELD_SUPPORT_RESTRAINED");

    let mut combined = exact();
    combined["combinations"] = json!([{"id": "C1", "label": "Envelope", "basis": "mechanics", "terms": [{"load_case": "SUS", "factor": 1.0}, {"load_case": "HT", "factor": 1.0}], "provenance": PROV}]);
    let outcome = apply_operation(&combined, &set_test(&combined, "HT", "none"), Some(&claim(&combined)));
    assert!(outcome.applied_model.is_none());
    assert!(codes_at(&outcome, "blocking").contains(&"COMBINATION_HYDROSTATIC_TEST_WITHHELD".to_string()), "{:?}", records(&outcome));
}

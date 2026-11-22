//! The maintained four-span hydrostatic test model a person opens
//! (`fixtures/hydrostatic_test/four_span_test_model.json`) taken through the
//! steps a person takes in the desktop, and solved to the frozen held
//! four-span rows of `validation/references/hydrostatic_test` (HT-4S-TEST-HELD)
//! in both solver modes (DEL-05-01 REQ-014, AC-010; DEL-04-07 REQ-011).
//!
//! The document holds no load case. The steps:
//!
//! 1. the product's self-weight plan creates the case and its generated
//!    weight (the desktop's self-weight control: gravity -9.79 m/s^2 along
//!    global Z on every pipe), applied as one batch;
//! 2. the case's pressure region (`update_load` of `pressure_regions`): the
//!    test pressure over every pipe, both ends closed with the cap thrust
//!    carried by the wall;
//! 3. the hydrostatic test condition, supports held
//!    (`set_load_case_test_condition`);
//! 4. the contents selection of the condition set's `HYDRO`
//!    (`set_load_case_condition_use`).
//!
//! The order is fixed; each step out of order is refused by the operation
//! that would take it, with the document unchanged:
//!
//! - a second self-weight plan for the case: the plan creates its case and
//!   refuses one that exists, SELF_WEIGHT_INPUT_INVALID ("case identity
//!   collides with existing case or primitive load");
//! - the contents selection before the test condition:
//!   LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED, since the exact route admits a
//!   contents selection only on a hydrostatic test case;
//! - the test condition before the pressure region:
//!   EXACT_PRESSURE_REGIONS_REQUIRED, "every exact-profile case requires
//!   pressure_regions, including explicit [] when no pressure region is
//!   present". The operation judges its candidate document as the solve
//!   admits it, and a newly created case carries no pressure_regions, whether
//!   the self-weight plan or the Loads page's create form created it.
//!
//! The admitted order solves to the held reference rows, with the test
//! contents entering each pipe's generated weight once.
use open_pipe_stress_operation_applier::document_validator::validate_model_document;
use open_pipe_stress_operation_applier::{apply_operation, apply_operation_batch, canonical_json, sha256_hex};
use open_pipe_stress_product_physics::self_weight::{generate_self_weight_operations, SelfWeightGravity, SelfWeightRequest, MANUAL_OVERRIDE_METHOD};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewModel, PreviewSolverMode};
use serde_json::{json, Value};

const FIXTURE: &str = include_str!("../../../../fixtures/hydrostatic_test/four_span_test_model.json");
const REFERENCES: &str = include_str!("../../../../validation/references/hydrostatic_test/references.json");
const CASE: &str = "HT-4S-TEST";
const REFERENCE_CASE: &str = "HT-4S-TEST-HELD";
const HELD: &str = "springs_and_constant_effort";
const PROV: &str = "invented hydrostatic test journey input";
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;

fn refs() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}
fn reference() -> Value {
    refs()["cases"].as_array().unwrap().iter().find(|c| c["id"] == REFERENCE_CASE).unwrap().clone()
}
fn fixture() -> Value {
    serde_json::from_str(FIXTURE).unwrap()
}
fn claim(model: &Value) -> Value {
    json!({"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_scope": "model_payload",
        "value": format!("sha256:{}", sha256_hex(&canonical_json(model)))})
}

fn operation(object: &str, target: &str, kind: &str, change_kind: &str, path: &str, before: &str, after: &str, unit: &str, dimension: &str, note: &str) -> Value {
    let token = format!("{change_kind}:{target}:{path}:{}", &sha256_hex(&format!("{before}|{after}"))[..12]);
    json!({
        "operation_id": format!("operation:{token}"), "operation_kind": kind, "operation_status": "proposed", "author_type": "user",
        "target": {"object_type": object, "ref": target},
        "change": {"change_id": format!("change:{token}"), "change_kind": change_kind, "field_label": path, "field_path": path,
                   "before": before, "after": after, "unit": unit, "dimension": dimension, "source_note": note},
        "validation": {"schema_validation": "not_run", "constraint_validation": "not_run", "unit_validation": "not_run",
                       "diff_preview_status": "not_generated", "application_status": "not_applied"},
        "audit_boundary": {"mutation_route": "structured_operations_only", "direct_model_mutation_allowed": false,
                           "requires_user_acceptance": true, "mutates_accepted_model_state": false},
        "professional_boundary": {"human_review_required": true, "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false, "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false, "software_makes_authentication_claim": false},
        "rationale": "Invented hydrostatic test journey operation",
    })
}

fn member(model: &Value, name: &str) -> String {
    model["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == CASE).and_then(|c| c.get(name))
        .map(canonical_json).unwrap_or_else(|| "not_present".into())
}

fn apply(model: &Value, intent: &Value, what: &str) -> Value {
    let outcome = apply_operation(model, intent, Some(&claim(model)));
    let next = outcome.applied_model.unwrap_or_else(|| panic!("{what} was refused: {:?}", outcome.diagnostics));
    assert!(!validate_model_document(&next).iter().any(|d| d.is_blocking()), "{what}: the applied document is invalid");
    next
}

fn self_weight_request(model: &Value) -> SelfWeightRequest {
    let pipes = model["pipe_segments"].as_array().unwrap().iter().map(|p| p["id"].as_str().unwrap().to_string()).collect();
    SelfWeightRequest {
        case_id: CASE.into(), label: "Hydrostatic test".into(), pipe_refs: pipes,
        gravity: SelfWeightGravity { value: -refs()["inputs"]["g"]["value"].as_f64().unwrap(), unit: "m/s^2".into(), axis: "global_z".into() },
        provenance: PROV.into(), source_model_hash: claim(model)["value"].as_str().unwrap().into(),
    }
}

/// Step 1: the product's self-weight plan, each draft as the operation the
/// desktop queues, applied as one batch.
fn generate_self_weight(model: &Value) -> Value {
    let typed: PreviewModel = serde_json::from_value(model.clone()).unwrap();
    let plan = generate_self_weight_operations(&typed, &self_weight_request(model)).unwrap_or_else(|d| panic!("self-weight plan refused: {d:?}"));
    let operations: Vec<Value> = plan.changes.iter().map(|draft| operation(&draft.object_type, &draft.target_ref, &draft.operation_kind,
        &draft.change_kind, &draft.field_path, &draft.before, &draft.after, &draft.unit, &draft.dimension, &draft.source_note)).collect();
    let batch = json!({"batch_id": "batch:hydrostatic-self-weight", "source_model_hash": plan.source_model_hash, "operations": operations});
    let applied = apply_operation_batch(model, &batch, Some(&claim(model)));
    let next = applied["applied_model"].clone();
    assert!(next.is_object(), "the self-weight batch was refused: {applied}");
    next
}

/// Step 2: the test pressure over every pipe, both ends closed with the cap
/// thrust carried by the wall.
fn set_region(model: &Value) -> Value {
    let pipes: Vec<Value> = model["pipe_segments"].as_array().unwrap().iter().map(|p| p["id"].clone()).collect();
    let terminal = |node: &str| json!({"node_ref": node, "closure_transfer": "transfers_to_wall", "provenance": PROV});
    let regions = json!([{"id": format!("region:{CASE}"), "member_pipe_ids": pipes, "pressure_basis": "internal_differential_zero_external_v1",
        "pressure": {"value": reference()["pressure"]["value"], "unit": "Pa"}, "terminals": [terminal("node:a"), terminal("node:e")], "provenance": PROV}]);
    let intent = operation("Load", CASE, "modify", "update_load", "pressure_regions", &member(model, "pressure_regions"), &regions.to_string(),
        "none", "dimensionless", "invented test input");
    apply(model, &intent, "the pressure region")
}

fn set_test_condition(model: &Value) -> Value {
    apply(model, &test_condition_intent(model), "the test condition")
}

fn select_contents_intent(model: &Value) -> Value {
    operation("Load", CASE, "modify", "set_load_case_condition_use", "condition_use", &member(model, "condition_use"),
        &json!({"contents_density": "HYDRO"}).to_string(), "none", "dimensionless", "invented test input")
}

fn select_contents(model: &Value) -> Value {
    apply(model, &select_contents_intent(model), "the contents selection")
}

fn solve(model: &Value, mode: PreviewSolverMode) -> Value {
    let out = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), mode)
        .unwrap_or_else(|e| panic!("typed boundary rejected the request: {e}"));
    let out = serde_json::to_value(out).unwrap();
    assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {}", out["diagnostics"]);
    out
}

// ------------------------------------------------------------------ reading rows

fn si(row: &Value) -> f64 {
    let value = row["value"].as_f64().expect("a numeric row");
    value * match row["unit"].as_str().expect("a row unit") {
        "N" | "m" | "Pa" | "N*m" => 1.0,
        "kN" | "kPa" | "kN*m" => 1.0e3,
        "MPa" => 1.0e6,
        "mm" => 1.0e-3,
        other => panic!("unexpected row unit {other}"),
    }
}
fn case_rows(out: &Value) -> impl Iterator<Item = &Value> {
    out["results"].as_array().unwrap().iter().filter(|row| row["basis_ref"]["ref_id"] == CASE)
}
fn observe(out: &Value, row_id: &str) -> Vec<f64> {
    if let Some((basis, rest)) = row_id.split_once("_section.") {
        let (kind, surface) = rest.split_once('.').unwrap();
        let component = format!("lame_{}_{kind}_stress", if surface == "bore" { "inner" } else { "outer" });
        let row_kind = if kind == "hoop" { "pipe_lame_hoop_stress_v2" } else { "pipe_lame_radial_stress_v2" };
        let found: Vec<f64> = case_rows(out).filter(|row| row["kind"] == row_kind && row["metadata"]["component"] == component.as_str()
            && row["metadata"]["basis"].as_str().is_some_and(|b| b.contains(&format!("section_basis={basis}_section")))).map(si).collect();
        assert!(!found.is_empty(), "no {component} rows on {basis}");
        return found;
    }
    let (entity, component) = row_id.split_once('.').unwrap();
    if component == "uz" {
        let found: Vec<f64> = case_rows(out).filter(|row| row["kind"] == "global_nodal_displacement_z" && row["entity_ref"] == entity).map(si).collect();
        assert_eq!(found.len(), 1, "one uz row for {entity}");
        return found;
    }
    let component = component.to_ascii_uppercase();
    let found: Vec<f64> = case_rows(out).filter(|row| row["kind"] == "support_reaction_component_v2" && row["entity_ref"] == entity
        && row["metadata"]["component"].as_str().is_some_and(|c| c.eq_ignore_ascii_case(&component))).map(si).collect();
    assert!(found.len() <= 1, "{entity} {component} published {} times", found.len());
    vec![found.first().copied().unwrap_or(0.0)]
}
fn scale(row: &Value) -> f64 {
    let reference = reference();
    let w = reference["weight_per_length"]["value"].as_f64().unwrap();
    let span = refs()["inputs"]["a"]["value"].as_f64().unwrap();
    match row["unit"].as_str().unwrap() {
        "N" => w * span,
        "N*m" => w * span * span,
        "m" => refs()["derived"]["two_span_delta0_test_euler_bernoulli"]["value"].as_f64().unwrap(),
        "Pa" => reference["pressure"]["value"].as_f64().unwrap(),
        other => panic!("unit {other}"),
    }
}

/// Every held four-span reference row, the mark, and the test contents once.
fn assert_held_reference(out: &Value, what: &str) {
    let reference = reference();
    let mut compared = 0;
    for row in reference["rows"].as_array().unwrap() {
        let id = row["id"].as_str().unwrap();
        let expected = row["value"].as_f64().unwrap();
        for observed in observe(out, id) {
            assert!((observed - expected).abs() <= RELATIVE * expected.abs().max(scale(row)), "{what} {id}: observed {observed:e}, expected {expected:e}");
            compared += 1;
        }
    }
    assert!(compared >= reference["rows"].as_array().unwrap().len(), "{what}: compared {compared}");
    let evidence = out["contract_evidence"]["exact_cases"].as_array().unwrap().iter().find(|c| c["load_case_id"] == CASE).expect("the case's exact evidence");
    let mark = &evidence["test_condition"];
    assert_eq!(mark["held_supports"], HELD, "{what}");
    assert_eq!(mark["held_support_refs"], reference["expected_outcome"]["held_support_ids"], "{what}");
    assert_eq!(mark["contents_condition"], "HYDRO", "{what}");
    let w = reference["weight_per_length"]["value"].as_f64().unwrap();
    let resolution = mark["contents_resolution"].as_array().unwrap();
    assert_eq!(resolution.len(), 4, "{what}: each pipe's generated weight takes the test contents once");
    for item in resolution {
        assert!((-item["applied_n_per_m"].as_f64().unwrap() - w).abs() <= RELATIVE * w, "{what}: {item}");
    }
    // The held constant-effort support applies no force.
    assert!(!case_rows(out).any(|row| row["kind"] == "constant_effort_support_applied_load"), "{what}");
}

// ------------------------------------------------------------------ the journey

#[test]
fn the_committed_model_is_valid_and_holds_no_case() {
    let model = fixture();
    assert!(!validate_model_document(&model).iter().any(|d| d.is_blocking()), "{:?}", validate_model_document(&model));
    assert_eq!(model["load_cases"], json!([]));
    assert_eq!(model["condition_sets"][0]["id"], "cs:test");
    assert_eq!(model["condition_sets"][0]["conditions"][0]["key"], "HYDRO");
}

/// The admitted order solves to the held four-span reference rows in both
/// solver modes.
#[test]
fn the_admitted_order_solves_to_the_held_reference() {
    let weighed = generate_self_weight(&fixture());
    let orders: [(&str, Vec<fn(&Value) -> Value>); 1] = [
        ("region, test, contents", vec![set_region, set_test_condition, select_contents]),
    ];
    let mut documents = Vec::new();
    for (name, steps) in orders {
        let model = steps.into_iter().fold(weighed.clone(), |model, step| step(&model));
        documents.push((name, model));
    }
    // The same document whatever the order.
    for (name, model) in &documents[1..] {
        assert_eq!(canonical_json(model), canonical_json(&documents[0].1), "{name}");
    }
    for mode in MODES {
        let out = solve(&documents[0].1, mode);
        assert_held_reference(&out, &format!("{} {mode:?}", documents[0].0));
    }
}

fn refused_with(model: &Value, intent: &Value, code: &str) {
    let outcome = apply_operation(model, intent, Some(&claim(model)));
    assert!(outcome.applied_model.is_none(), "{code}: the operation applied");
    assert!(outcome.diagnostics.iter().any(|d| d.code == code && d.severity == "blocking"), "{code}: {:?}", outcome.diagnostics);
}

fn test_condition_intent(model: &Value) -> Value {
    operation("Load", CASE, "modify", "set_load_case_test_condition", "test_condition", &member(model, "test_condition"),
        &json!({"kind": "hydrostatic", "held_supports": HELD}).to_string(), "none", "dimensionless", "invented test input")
}

/// The steps out of order, each refused by name.
#[test]
fn each_step_out_of_order_is_refused_by_name() {
    let weighed = generate_self_weight(&fixture());
    // A second plan for the same case: the plan creates its case.
    let typed: PreviewModel = serde_json::from_value(weighed.clone()).unwrap();
    let refused = generate_self_weight_operations(&typed, &self_weight_request(&weighed)).err().expect("a second plan is refused");
    assert!(refused.iter().any(|d| d.code == "SELF_WEIGHT_INPUT_INVALID" && d.message == "case identity collides with existing case or primitive load"), "{refused:?}");
    // The contents selection on a case that is not yet a test case.
    refused_with(&weighed, &select_contents_intent(&weighed), "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
    // The test condition on the plan's case before its pressure region.
    refused_with(&weighed, &test_condition_intent(&weighed), "EXACT_PRESSURE_REGIONS_REQUIRED");
}

/// A case created by the Loads page's own create form (its payload exactly:
/// id, label, kind, status, provenance, empty primitive loads) carries no
/// pressure_regions either, so the test condition waits for its region there too.
#[test]
fn a_case_created_on_the_loads_page_also_takes_its_region_first() {
    let model = fixture();
    let payload = json!({"id": CASE, "label": "Hydrostatic test", "kind": "primitive_user_load", "status": "draft", "provenance": PROV, "primitive_loads": []});
    let created = apply(&model, &operation("Load", CASE, "create", "create_load_case", "load_cases", "not_present", &payload.to_string(),
        "none", "dimensionless", "explicit user-entered empty load-case shell; primitive loads are not inferred"), "the created case");
    assert!(created["load_cases"][0].get("pressure_regions").is_none());
    refused_with(&created, &test_condition_intent(&created), "EXACT_PRESSURE_REGIONS_REQUIRED");
    let regioned = set_region(&created);
    assert!(set_test_condition(&regioned)["load_cases"][0]["test_condition"].is_object());
}

/// Contents selected on a test case that carries no generated self-weight yet
/// (the Loads page's created case) are a staged state: the selection applies
/// with LOAD_CASE_TEST_SELF_WEIGHT_MISSING as a warning, and the solve refuses
/// the case by that name with no result until the weight is generated. A
/// case whose self-weight the contents cannot enter, one generated load
/// overridden by hand, is not staged: the selection is refused by name with
/// the document unchanged (LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID).
#[test]
fn a_missing_self_weight_is_staged_and_one_the_contents_cannot_enter_is_refused() {
    let model = fixture();
    let payload = json!({"id": CASE, "label": "Hydrostatic test", "kind": "primitive_user_load", "status": "draft", "provenance": PROV, "primitive_loads": []});
    let created = apply(&model, &operation("Load", CASE, "create", "create_load_case", "load_cases", "not_present", &payload.to_string(),
        "none", "dimensionless", "explicit user-entered empty load-case shell; primitive loads are not inferred"), "the created case");
    let tested = set_test_condition(&set_region(&created));
    let outcome = apply_operation(&tested, &select_contents_intent(&tested), Some(&claim(&tested)));
    let selected = outcome.applied_model.clone().unwrap_or_else(|| panic!("the staged selection was refused: {:?}", outcome.diagnostics));
    assert!(outcome.diagnostics.iter().any(|d| d.code == "LOAD_CASE_TEST_SELF_WEIGHT_MISSING" && d.severity == "warning"), "{:?}", outcome.diagnostics);
    assert!(!outcome.diagnostics.iter().any(|d| d.code == "LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID"), "{:?}", outcome.diagnostics);
    for mode in MODES {
        let out = serde_json::to_value(run_linear_static_preview_value_with_mode(json!({"model": selected, "materials": []}), mode).unwrap()).unwrap();
        assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        assert!(out["results"].as_array().unwrap().is_empty(), "{mode:?}: no rows");
        assert!(out["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "LOAD_CASE_TEST_SELF_WEIGHT_MISSING" && d["severity"] == "blocking"),
            "{mode:?}: {}", out["diagnostics"]);
    }

    let mut overridden = set_test_condition(&set_region(&generate_self_weight(&fixture())));
    let load = &mut overridden["load_cases"][0]["primitive_loads"][0];
    let prior: Value = serde_json::from_str(load["provenance"].as_str().unwrap()).unwrap();
    load["provenance"] = json!(json!({"method": MANUAL_OVERRIDE_METHOD, "original_generation_provenance": prior,
        "decision": "preserve_modified_generated_load", "source_model_hash": format!("sha256:{}", "0".repeat(64))}).to_string());
    refused_with(&overridden, &select_contents_intent(&overridden), "LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID");
}

// ------------------------------------------------------------------ the reader models

const READER_MODELS: &str = include_str!("../../../../fixtures/hydrostatic_test/four_span_reader_models.json");
const READER_MODELS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../fixtures/hydrostatic_test/four_span_reader_models.json");
/// The two documents' sparse envelopes, as the producer serializes them.
const READER_RAW_PATHS: [&str; 2] = ["fixtures/results/hydrostatic_test_journey-sparse_interactive.raw.json",
    "fixtures/results/hydrostatic_test_load_state_twin-sparse_interactive.raw.json"];
const READER_RAW: [&str; 2] = [include_str!("../../../../fixtures/results/hydrostatic_test_journey-sparse_interactive.raw.json"),
    include_str!("../../../../fixtures/results/hydrostatic_test_load_state_twin-sparse_interactive.raw.json")];
const WRITE_READER_MODELS: &str = "OPENPIPESTRESS_WRITE_HYDROSTATIC_READER_MODELS";
const ORDINARY: &str = "HT-4S-OPERATING";

/// An ordinary case beside the test case: the self-weight plan for `ORDINARY`
/// and its own pressure region, by the same operations.
fn add_ordinary_case(model: &Value) -> Value {
    let typed: PreviewModel = serde_json::from_value(model.clone()).unwrap();
    let request = SelfWeightRequest { case_id: ORDINARY.into(), label: "Operating weight".into(), ..self_weight_request(model) };
    let plan = generate_self_weight_operations(&typed, &request).unwrap_or_else(|d| panic!("self-weight plan refused: {d:?}"));
    let operations: Vec<Value> = plan.changes.iter().map(|draft| operation(&draft.object_type, &draft.target_ref, &draft.operation_kind,
        &draft.change_kind, &draft.field_path, &draft.before, &draft.after, &draft.unit, &draft.dimension, &draft.source_note)).collect();
    let batch = json!({"batch_id": "batch:hydrostatic-ordinary-weight", "source_model_hash": plan.source_model_hash, "operations": operations});
    let weighed = apply_operation_batch(model, &batch, Some(&claim(model)))["applied_model"].clone();
    assert!(weighed.is_object(), "the ordinary self-weight batch was refused");
    let pipes: Vec<Value> = weighed["pipe_segments"].as_array().unwrap().iter().map(|p| p["id"].clone()).collect();
    let terminal = |node: &str| json!({"node_ref": node, "closure_transfer": "transfers_to_wall", "provenance": PROV});
    let regions = json!([{"id": format!("region:{ORDINARY}"), "member_pipe_ids": pipes, "pressure_basis": "internal_differential_zero_external_v1",
        "pressure": {"value": reference()["pressure"]["value"], "unit": "Pa"}, "terminals": [terminal("node:a"), terminal("node:e")], "provenance": PROV}]);
    let before = weighed["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == ORDINARY).and_then(|c| c.get("pressure_regions"))
        .map(canonical_json).unwrap_or_else(|| "not_present".into());
    apply(&weighed, &operation("Load", ORDINARY, "modify", "update_load", "pressure_regions", &before, &regions.to_string(),
        "none", "dimensionless", "invented test input"), "the ordinary case's pressure region")
}

/// The load and reference-state twin of `model`, authored inline: one
/// installed reference configuration, and on every case an analysis state with
/// its members unchanged from it, every support an active model device and a
/// ledger over the case's generated self-weight at factor 1.
fn load_state_twin(model: &Value) -> Value {
    let mut twin = model.clone();
    let pipes: Vec<String> = twin["pipe_segments"].as_array().unwrap().iter().map(|p| p["id"].as_str().unwrap().to_string()).collect();
    let supports: Vec<String> = twin["supports"].as_array().unwrap().iter().map(|s| s["id"].as_str().unwrap().to_string()).collect();
    twin["reference_configurations"] = json!([{"id": "reference:installed", "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": pipes.iter().map(|pipe| json!({"pipe_ref": pipe,
            "basis": {"kind": "temperature_reference", "installation_temperature": {"value": 20.0, "unit": "degC"}},
            "fit": {"kind": "none"}, "provenance": PROV})).collect::<Vec<_>>(), "provenance": PROV}]);
    for case in twin["load_cases"].as_array_mut().unwrap() {
        let ledger: Vec<Value> = case["primitive_loads"].as_array().unwrap().iter().map(|load| json!({"source_ref": load["id"], "factor": 1.0})).collect();
        case["analysis_state"] = json!({"contract": "openpipestress.load_reference_state/1.0.0", "reference_configuration_ref": "reference:installed",
            "element_states": pipes.iter().map(|pipe| json!({"pipe_ref": pipe,
                "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:ht",
                    "applicability_reference": "invented analytical basis declared applicable to this member"},
                "thermal_state": {"kind": "unchanged_reference", "provenance": PROV}})).collect::<Vec<_>>(),
            "support_states": supports.iter().map(|support| json!({"support_ref": support, "participation": {"kind": "active_model_device"}})).collect::<Vec<_>>(),
            "load_sources": ledger, "history": {"kind": "independent_equilibrium"}, "provenance": PROV});
    }
    twin
}

/// The two solved documents the readers are tested on, committed as
/// `fixtures/hydrostatic_test/four_span_reader_models.json`: the four-span
/// model taken through its journey, and its load and reference-state twin with
/// an ordinary case beside the test case; and their sparse envelopes, committed
/// as `fixtures/results/hydrostatic_test_*-sparse_interactive.raw.json`. Both
/// solve in both modes, and the test case reproduces the held reference rows.
/// Regenerate both with
/// `OPENPIPESTRESS_WRITE_HYDROSTATIC_READER_MODELS=1`; otherwise the committed
/// bytes must be current.
#[test]
fn the_reader_models_are_current() {
    let journey = [set_region, set_test_condition, select_contents].into_iter().fold(generate_self_weight(&fixture()), |model, step| step(&model));
    let twin = load_state_twin(&add_ordinary_case(&journey));
    assert!(!validate_model_document(&twin).iter().any(|d| d.is_blocking()), "{:?}", validate_model_document(&twin));
    for (name, model) in [("journey", &journey), ("load and reference-state twin", &twin)] {
        for mode in MODES {
            assert_held_reference(&solve(model, mode), &format!("{name} {mode:?}"));
        }
    }
    let models = json!({
        "about": "The four-span hydrostatic test model (four_span_test_model.json) taken through its journey (self-weight, pressure region, held test condition, contents HYDRO), and its load and reference-state twin with an ordinary case beside the test case; the documents the product readers are tested on. Invented inputs; not library, catalog or code data.",
        "generator": "core/model_operations/operation_applier/tests/hydrostatic_four_span_journey.rs::the_reader_models_are_current",
        "models": {"journey": journey, "load_state_twin": twin},
    });
    let bytes = serde_json::to_string_pretty(&models).unwrap() + "\n";
    // The solved envelopes, for the readers that take a result rather than solve
    // (the desktop's web view and native host): each document in sparse mode, as
    // the raw bytes the producer's envelope serializes to.
    let raw: Vec<(&str, String)> = [(READER_RAW_PATHS[0], &journey), (READER_RAW_PATHS[1], &twin)].into_iter()
        .map(|(path, model)| (path, serde_json::to_string(&solve(model, PreviewSolverMode::SparseInteractive)).unwrap())).collect();
    if std::env::var_os(WRITE_READER_MODELS).is_some() {
        std::fs::write(READER_MODELS_PATH, &bytes).unwrap();
        for (path, text) in &raw {
            std::fs::write(format!("{}/../../../{path}", env!("CARGO_MANIFEST_DIR")), text).unwrap();
        }
        return;
    }
    assert!(READER_MODELS == bytes, "fixtures/hydrostatic_test/four_span_reader_models.json is stale; regenerate it with {WRITE_READER_MODELS}=1");
    for ((path, text), committed) in raw.iter().zip(READER_RAW) {
        assert!(committed == text, "{path} is stale; regenerate it with {WRITE_READER_MODELS}=1");
    }
}

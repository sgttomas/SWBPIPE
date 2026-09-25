//! Invented straight-pipe regression: plan -> atomic apply -> structured edit ->
//! JSON save/reopen -> product solve. No production mass helper is the oracle.
use open_pipe_stress_operation_applier::{apply_operation, apply_operation_batch};
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_product_physics::{run_linear_static_preview, MechanicsEnvelope};
use open_pipe_stress_self_weight_wasm::{generate_plan, generate_self_weight_plan_json};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const GRAVITY: f64 = -7.0; // Explicit invented user acceleration; no standard default.
const TIP_FORCE: f64 = -11.0;
fn weight_per_m(density: f64) -> f64 {
    std::f64::consts::PI / 4.0 * (0.1_f64.powi(2) - 0.08_f64.powi(2)) * density * GRAVITY
}
fn close(label: &str, actual: f64, expected: f64) {
    // Product result serialization rounds to six decimal places.
    assert!((actual - expected).abs() <= 0.5001e-6 + expected.abs() * 1e-10,
        "{label}: actual={actual:.12}, expected={expected:.12}");
}
fn model() -> Value {
    json!({"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:invented-self-weight","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},
        "analysis_status":{"mechanics":"draft","rule_check":"not_run","professional_acceptance":"not_assessed"},
        "nodes":[{"id":"node:root","label":"Root","position":{"x":0,"y":0,"z":0},"provenance":"invented"},
            {"id":"node:tip","label":"Tip","position":{"x":2,"y":0,"z":0},"provenance":"invented"}],
        "pipe_segments":[{"id":"pipe:beam","label":"Invented beam","from":"node:root","to":"node:tip","material":"material:invented","y_reference":{"x":0,"y":1,"z":0},"provenance":"invented",
            "section":{"outside_diameter":{"value":0.1,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"},"material_density":{"value":1000,"unit":"kg/m^3"}}}],
        "materials":[{"id":"material:invented","label":"Invented elastic material","elastic_modulus":{"value":200e9,"unit":"Pa"},"shear_modulus":{"value":77e9,"unit":"Pa"},"provenance":"invented_no_material_standard"}],
        "supports":[{"id":"support:root","label":"Fixed root","node":"node:root","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"invented"}],
        "sections":[],"components":[],"load_cases":[],"combinations":[]})
}
fn hash(model: &Value) -> String {
    format!("sha256:{:x}", Sha256::digest(canonical_json(model).as_bytes()))
}
fn claim(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":hash(model)})
}
// Same public operation-envelope fields as operation_applier's maintained
// support_authoring_conflicts and atomic_batch tests; all changes use the applier.
fn operation(draft: &Value, id: &str) -> Value {
    json!({"operation_id":format!("operation:{id}"),"operation_kind":draft["operation_kind"],"operation_status":"proposed","author_type":"user",
        "target":{"object_type":draft["object_type"],"ref":draft["target_ref"]},
        "change":{"change_id":format!("change:{id}"),"change_kind":draft["change_kind"],"field_label":draft["field_label"],"field_path":draft["field_path"],"before":draft["before"],"after":draft["after"],"unit":draft["unit"],"dimension":draft["dimension"],"source_note":draft["source_note"]},
        "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
        "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
        "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},"rationale":"Invented connected regression"})
}
fn generate(model: &Value) -> (Value, Value) {
    let request = json!({"case_id":"case:weight","label":"Invented self weight","pipe_refs":["pipe:beam"],"gravity":{"value":GRAVITY,"unit":"m/s^2","axis":"global_y"},"provenance":"explicit invented gravity","source_model_hash":hash(model)});
    let before = model.clone();
    let outcome = generate_plan(model, &request);
    let transport: Value = serde_json::from_str(&generate_self_weight_plan_json(&model.to_string(), &request.to_string())).unwrap();
    assert_eq!(outcome, transport);
    assert_eq!(outcome["status"], "ready", "{outcome}");
    assert_eq!(*model, before);
    let operations: Vec<_> = outcome["plan"]["changes"].as_array().unwrap().iter()
        .enumerate().map(|(i, draft)| operation(draft, &format!("weight-{i}"))).collect();
    (request, json!({"batch_id":"batch:weight","source_model_hash":hash(model),"operations":operations}))
}
fn applied() -> Value {
    applied_from(model())
}
fn applied_from(original: Value) -> Value {
    let before = original.clone();
    let (_, batch) = generate(&original);
    let outcome = apply_operation_batch(&original, &batch, Some(&claim(&original)));
    assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{outcome}");
    assert_eq!(original, before);
    let weighted = outcome["applied_model"].clone();
    let tip = json!({"id":"load:manual-tip","category":"concentrated_force","target":{"type":"node","node":"node:tip"},"direction":"global_y","magnitude":{"value":TIP_FORCE,"unit":"N"},"dimension":"force","provenance":"manual invented tip load"});
    let draft = json!({"object_type":"Load","target_ref":"case:weight","operation_kind":"create","change_kind":"create_primitive_load","field_label":"Manual tip","field_path":"primitive_loads","before":"not_present","after":tip.to_string(),"unit":"N","dimension":"force","source_note":"manual invented tip load"});
    let out = apply_operation(&weighted, &operation(&draft, "manual-tip"), Some(&claim(&weighted)));
    let with_tip = out.applied_model.expect(&format!("{:?}", out.diagnostics));
    assert_eq!(with_tip["load_cases"][0]["primitive_loads"][1], tip);
    with_tip
}
fn edit(model: &Value, object: &str, target: &str, field: &str, before: &str, after: f64, unit: &str, dimension: &str) -> Value {
    let snapshot = model.clone();
    let draft = json!({"object_type":object,"target_ref":target,"operation_kind":"modify","change_kind":"set_field","field_label":field,"field_path":field,"before":before,"after":after.to_string(),"unit":unit,"dimension":dimension,"source_note":"explicit invented dependency edit"});
    let out = apply_operation(model, &operation(&draft, field), Some(&claim(model)));
    assert_eq!(*model, snapshot);
    out.applied_model.expect(&format!("{:?}", out.diagnostics))
}
fn reopen(model: &Value) -> Value {
    let saved = serde_json::to_vec_pretty(model).unwrap();
    let reopened: Value = serde_json::from_slice(&saved).unwrap();
    assert_eq!(*model, reopened);
    reopened
}
fn attempt_solve(model: &Value) -> MechanicsEnvelope {
    let before = model.clone();
    let out = run_linear_static_preview(serde_json::from_value(json!({"model":model,"materials":[]})).unwrap());
    assert_eq!(*model, before);
    assert!(!out.accepted_model_state_mutated);
    out
}
fn solve(model: &Value) -> MechanicsEnvelope {
    let out = attempt_solve(model);
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{:?}", out.diagnostics);
    out
}
fn refresh_batch(model: &Value, policy: &str) -> Value {
    let request = json!({"mode":"refresh","case_id":"case:weight","source_model_hash":hash(model),"manual_overrides":policy});
    let before = model.clone();
    let outcome = generate_plan(model, &request);
    assert_eq!(outcome, serde_json::from_str::<Value>(&generate_self_weight_plan_json(&model.to_string(), &request.to_string())).unwrap());
    assert_eq!(*model, before);
    assert_eq!(outcome["status"], "ready", "{outcome}");
    let operations: Vec<_> = outcome["plan"]["changes"].as_array().unwrap().iter().enumerate()
        .map(|(i, draft)| operation(draft, &format!("refresh-{i}"))).collect();
    json!({"batch_id":"batch:refresh","source_model_hash":outcome["plan"]["source_model_hash"],"operations":operations})
}
fn apply_refresh(model: &Value, policy: &str) -> Value {
    let before = model.clone();
    let batch = refresh_batch(model, policy);
    let outcome = apply_operation_batch(model, &batch, Some(&claim(model)));
    assert_eq!(*model, before);
    assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{outcome}");
    outcome["applied_model"].clone()
}
fn result(out: &MechanicsEnvelope, id: &str) -> f64 {
    out.results.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("missing {id}: {:?}", out.results)).value
}
fn equilibrium(out: &MechanicsEnvelope, density: f64, length: f64) {
    let q = weight_per_m(density);
    let total = -(q * length + TIP_FORCE);
    let first_moment = -(q * length * length / 2.0 + TIP_FORCE * length);
    close("total root reaction", result(out, "result:reaction:support-root"), total);
    close("root shear", result(out, "result:force:pipe-beam:shear-y"), total);
    close("root first moment", result(out, "result:moment:pipe-beam:bending-z"), first_moment);
}
#[test]
fn span_edit_scales_weight_and_preserves_manual_load_after_save_reopen() {
    let initial = applied();
    equilibrium(&solve(&reopen(&initial)), 1000.0, 2.0);
    let changed = edit(&initial, "Node", "node:tip", "position.x", "2", 3.0, "m", "length");
    assert_eq!(changed["load_cases"], initial["load_cases"]);
    close("unchanged intensity", changed["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"].as_f64().unwrap(), weight_per_m(1000.0));
    equilibrium(&solve(&reopen(&changed)), 1000.0, 3.0);
}
#[test]
fn density_edit_must_not_solve_with_stale_applied_self_weight() {
    let initial = applied();
    equilibrium(&solve(&reopen(&initial)), 1000.0, 2.0);
    let changed = edit(&initial, "Element", "pipe:beam", "section.material_density.value", "1000", 2000.0, "kg/m^3", "density");
    assert_eq!(changed["load_cases"][0]["primitive_loads"][1], initial["load_cases"][0]["primitive_loads"][1]);
    let reopened = reopen(&changed);
    let blocked = attempt_solve(&reopened);
    assert_eq!(blocked.status.mechanics, "MODEL_INCOMPLETE");
    assert!(blocked.diagnostics.iter().any(|d| d.code == "SELF_WEIGHT_INPUTS_STALE"), "{:?}", blocked.diagnostics);
    let refreshed = apply_refresh(&reopened, "block");
    assert_eq!(refreshed["load_cases"][0]["primitive_loads"].as_array().unwrap().len(), 2);
    assert_eq!(refreshed["load_cases"][0]["primitive_loads"][1], initial["load_cases"][0]["primitive_loads"][1]);
    assert_eq!(refreshed["load_cases"][0]["primitive_loads"][0]["id"], initial["load_cases"][0]["primitive_loads"][0]["id"]);
    equilibrium(&solve(&reopen(&refreshed)), 2000.0, 2.0);
    let repeated = generate_plan(&refreshed, &json!({"mode":"refresh","case_id":"case:weight","source_model_hash":hash(&refreshed),"manual_overrides":"block"}));
    assert_eq!(repeated["diagnostics"][0]["code"], "SELF-WEIGHT-REFRESH-NOT-NEEDED");
}

#[test]
fn queued_refresh_rejects_a_later_dependency_edit_even_with_current_claim() {
    let initial = applied();
    let changed = edit(&initial, "Element", "pipe:beam", "section.material_density.value", "1000", 2000.0, "kg/m^3", "density");
    let batch = refresh_batch(&changed, "block");
    let later = edit(&changed, "Element", "pipe:beam", "section.material_density.value", "2000", 3000.0, "kg/m^3", "density");
    let outcome = apply_operation_batch(&later, &batch, Some(&claim(&later)));
    assert!(outcome["applied_model"].is_null());
    assert_eq!(outcome["operation_outcomes"], json!([]));
}

#[test]
fn modified_weight_requires_explicit_preservation_and_remains_fixed_manual() {
    let initial = applied();
    let previous = initial["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"].to_string();
    let modified = edit(&initial, "Load", "case:weight", "primitive_loads.0.magnitude.value", &previous, -30.0, "N/m", "force_per_length");
    let blocked = attempt_solve(&modified);
    assert!(blocked.diagnostics.iter().any(|d| d.code == "SELF_WEIGHT_GENERATED_LOAD_MODIFIED"));
    let refused = generate_plan(&modified, &json!({"mode":"refresh","case_id":"case:weight","source_model_hash":hash(&modified),"manual_overrides":"block"}));
    assert_eq!(refused["status"], "blocked");
    let preserved = apply_refresh(&modified, "preserve");
    let mut physical_before = modified["load_cases"][0]["primitive_loads"][0].clone();
    let mut physical_after = preserved["load_cases"][0]["primitive_loads"][0].clone();
    physical_before.as_object_mut().unwrap().remove("provenance"); physical_after.as_object_mut().unwrap().remove("provenance");
    assert_eq!(physical_before, physical_after);
    assert_eq!(preserved["load_cases"][0]["primitive_loads"][1], modified["load_cases"][0]["primitive_loads"][1]);
    let later = edit(&preserved, "Element", "pipe:beam", "section.material_density.value", "1000", 2000.0, "kg/m^3", "density");
    let solved = solve(&reopen(&later));
    assert!(solved.diagnostics.iter().any(|d| d.code == "SELF_WEIGHT_MANUAL_OVERRIDE"));
    close("explicit fixed manual force", result(&solved, "result:reaction:support-root"), 71.0);
    close("explicit fixed manual moment", result(&solved, "result:moment:pipe-beam:bending-z"), 82.0);
}

#[test]
fn known_v1_requires_explicit_method_refresh_and_retains_original_provenance() {
    // Captured from the actual pre-repair plan/apply route; never synthesized
    // with the successor's area calculation.
    let legacy: Value = serde_json::from_str(include_str!("fixtures/applied_self_weight_v1.json")).unwrap();
    let before = legacy.clone();
    let original_provenance = legacy["load_cases"][0]["primitive_loads"][0]["provenance"].clone();
    let blocked = attempt_solve(&legacy);
    assert_eq!(blocked.status.mechanics, "MODEL_INCOMPLETE");
    assert!(blocked.diagnostics.iter().any(|d| d.code == "SELF_WEIGHT_METHOD_REFRESH_REQUIRED"), "{:?}", blocked.diagnostics);
    let refreshed = apply_refresh(&legacy, "block");
    assert_eq!(legacy, before);
    let provenance: Value = serde_json::from_str(refreshed["load_cases"][0]["primitive_loads"][0]["provenance"].as_str().unwrap()).unwrap();
    assert_eq!(provenance["mass_method"], "source_od_effective_wall_areas/v1");
    assert_eq!(provenance["legacy_generation_provenance"], original_provenance);
    equilibrium(&solve(&reopen(&refreshed)), 1000.0, 2.0);
    assert_eq!(refreshed["load_cases"][0]["primitive_loads"][1], legacy["load_cases"][0]["primitive_loads"][1]);
}

#[test]
fn real_generated_mill_contents_and_insulation_record_refreshes_through_applier() {
    let mut source = model();
    source["pipe_segments"][0]["section"]["mill_tolerance"] = json!({"value":1.0,"unit":"mm"});
    source["pipe_segments"][0]["section"]["contents_density"] = json!({"value":400.0,"unit":"kg/m^3"});
    source["pipe_segments"][0]["section"]["insulation_thickness"] = json!({"value":10.0,"unit":"mm"});
    source["pipe_segments"][0]["section"]["insulation_density"] = json!({"value":50.0,"unit":"kg/m^3"});
    let original = applied_from(source);
    let changed = edit(&original, "Element", "pipe:beam", "section.material_density.value", "1000", 2000.0, "kg/m^3", "density");
    assert!(attempt_solve(&changed).diagnostics.iter().any(|d| d.code == "SELF_WEIGHT_INPUTS_STALE"));
    let refreshed = apply_refresh(&changed, "block");
    let solved = solve(&reopen(&refreshed));
    // Independent disk/ring geometry with the existing absolute mill deduction.
    let mass = std::f64::consts::PI * (0.009 * 0.091 * 2000.0 + 0.041_f64.powi(2) * 400.0 + 0.01 * 0.11 * 50.0);
    let q = mass * GRAVITY;
    close("all mass source reaction", result(&solved, "result:reaction:support-root"), -(q*2.0 + TIP_FORCE));
    close("all mass source first moment", result(&solved, "result:moment:pipe-beam:bending-z"), -(q*2.0 + TIP_FORCE*2.0));
    assert_eq!(refreshed["load_cases"][0]["primitive_loads"][1], original["load_cases"][0]["primitive_loads"][1]);
}

#[test]
fn valid_imported_retarget_can_be_kept_manual_after_old_pipe_is_removed() {
    let mut imported = applied();
    // This is an explicit imported-model edit, not a claimed target-edit UI route.
    imported["pipe_segments"][0]["id"] = json!("pipe:manual");
    imported["load_cases"][0]["primitive_loads"][0]["target"]["pipe"] = json!("pipe:manual");
    imported["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(-30.0);
    let before = imported.clone();
    let blocked = attempt_solve(&imported);
    assert!(blocked.diagnostics.iter().any(|d| d.code == "SELF_WEIGHT_GENERATED_LOAD_MODIFIED"), "{:?}", blocked.diagnostics);
    let preserved = apply_refresh(&imported, "preserve");
    assert_eq!(imported, before);
    assert_eq!(preserved["load_cases"][0]["primitive_loads"][0]["target"]["pipe"], "pipe:manual");
    assert_eq!(preserved["load_cases"][0]["primitive_loads"][0]["magnitude"], before["load_cases"][0]["primitive_loads"][0]["magnitude"]);
    let solved = solve(&reopen(&preserved));
    close("retargeted fixed force", result(&solved, "result:reaction:support-root"), 71.0);
    close("retargeted fixed first moment", result(&solved, "result:moment:pipe-manual:bending-z"), 82.0);
}

#[test]
fn shared_section_edit_propagates_then_requires_weight_refresh() {
    let mut source = model();
    source["sections"] = json!([{"id":"section:weight","name":"Invented shared annulus","section_type":"pipe",
        "properties":{"outside_diameter":{"value":0.1,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"explicit invented section"}]);
    source["pipe_segments"][0]["section_ref"] = json!("section:weight");
    let original = applied_from(source);
    let changed = edit(&original, "Section", "section:weight", "properties.outside_diameter.value", "0.1", 0.2, "m", "length");
    assert_eq!(changed["pipe_segments"][0]["section"]["outside_diameter"], changed["sections"][0]["properties"]["outside_diameter"]);
    assert!(attempt_solve(&changed).diagnostics.iter().any(|d| d.code == "SELF_WEIGHT_INPUTS_STALE"));
    let refreshed = apply_refresh(&changed, "block");
    let solved = solve(&reopen(&refreshed));
    let q = std::f64::consts::PI * 0.01 * 0.19 * 1000.0 * GRAVITY;
    close("shared section force", result(&solved, "result:reaction:support-root"), -(q*2.0 + TIP_FORCE));
    close("shared section first moment", result(&solved, "result:moment:pipe-beam:bending-z"), -(q*2.0 + TIP_FORCE*2.0));
    assert_eq!(refreshed["load_cases"][0]["primitive_loads"][1], original["load_cases"][0]["primitive_loads"][1]);
}
#[test]
fn preapply_source_hash_guard_rejects_dependency_edit() {
    let original = model();
    let (request, batch) = generate(&original);
    let changed = edit(&original, "Element", "pipe:beam", "section.material_density.value", "1000", 2000.0, "kg/m^3", "density");
    let outcome = generate_plan(&changed, &request);
    assert_eq!(outcome["status"], "blocked");
    assert_eq!(outcome["diagnostics"][0]["code"], "SELF-WEIGHT-SOURCE-HASH-MISMATCH");
    let rejected = apply_operation_batch(&changed, &batch, Some(&claim(&original)));
    assert!(rejected["applied_model"].is_null(), "{rejected}");
    assert_eq!(rejected["validation"]["application_status"], "blocked");
}

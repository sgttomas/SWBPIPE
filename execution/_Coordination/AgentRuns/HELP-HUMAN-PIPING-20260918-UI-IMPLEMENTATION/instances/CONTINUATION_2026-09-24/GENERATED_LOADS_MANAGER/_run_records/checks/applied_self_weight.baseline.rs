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
    (request, json!({"batch_id":"batch:weight","operations":operations}))
}
fn applied() -> Value {
    let original = model();
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
fn solve(model: &Value) -> MechanicsEnvelope {
    let before = model.clone();
    let out = run_linear_static_preview(serde_json::from_value(json!({"model":model,"materials":[]})).unwrap());
    assert_eq!(*model, before);
    assert!(!out.accepted_model_state_mutated);
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{:?}", out.diagnostics);
    out
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
    let solved = solve(&reopened);
    eprintln!("density changed 1000 -> 2000 kg/m^3; stored q={} N/m; independently expected q={} N/m; actual reaction={} N; expected reaction={} N; expected first moment={} N*m",
        reopened["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"], weight_per_m(2000.0), result(&solved, "result:reaction:support-root"), -(weight_per_m(2000.0)*2.0+TIP_FORCE), -(weight_per_m(2000.0)*2.0+TIP_FORCE*2.0));
    // Deliberately asserts correct fresh equilibrium: baseline must fail here.
    // A repair choosing explicit stale blocking will require a separate precise
    // blocking/refresh contract test, not substituting the stale number as oracle.
    equilibrium(&solved, 2000.0, 2.0);
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

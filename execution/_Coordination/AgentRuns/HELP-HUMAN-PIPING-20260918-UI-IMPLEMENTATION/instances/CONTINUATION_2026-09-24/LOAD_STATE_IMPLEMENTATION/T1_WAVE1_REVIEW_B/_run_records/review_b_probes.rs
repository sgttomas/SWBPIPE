//! REVIEW_B probes (invented inputs; not part of the candidate).
use open_pipe_stress_operation_applier::{apply_operation, canonical_json, sha256_hex, validate_operation};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const CONNECTED: &str =
    include_str!("../../../../fixtures/product_preview/load_reference/connected.request.json");

fn request() -> Value { serde_json::from_str(CONNECTED).unwrap() }
fn connected() -> Value { request()["model"].clone() }
fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload",
        "value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}
fn intent(object: &str, id: &str, kind: &str, path: &str, before: &str, after: &str) -> Value {
    json!({"operation_id":format!("op:probe:{id}:{path}"),"operation_kind":"modify","operation_status":"proposed","author_type":"user",
      "target":{"object_type":object,"ref":id},
      "change":{"change_id":format!("change:probe:{id}:{path}"),"change_kind":kind,
        "field_label":path,"field_path":path,"before":before,"after":after,"unit":"none","dimension":"dimensionless","source_note":"Invented review probe"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "rationale":"Invented review probe",
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false}})
}
fn run(model: &Value, op: &Value) -> (Option<Value>, Vec<String>) {
    let out = apply_operation(model, op, Some(&hash(model)));
    let codes = out.diagnostics.iter().map(|d| format!("{}:{}", d.severity, d.code)).collect();
    (out.applied_model, codes)
}

/// P1: the pre-existing pressure_profile operation rewrites schema_version on a 0.4.0 model.
#[test]
fn p1_pressure_profile_downgrades_a_0_4_0_model() {
    let model = connected();
    let project = model["project"]["id"].as_str().unwrap().to_string();
    let before = canonical_json(&json!({"schema_version": model["schema_version"], "pressure_contract": model["pressure_contract"]}));
    let after = json!({"schema_version":"0.3.0","pressure_contract":{"version":"2.0.0","mode":"exact_straight_pressure_v2"}}).to_string();
    let op = intent("Model", &project, "set_field", "pressure_profile", &before, &after);
    let review = validate_operation(&model, &op, Some(&hash(&model)));
    println!("P1 review codes: {:?}", review.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
    let (applied, codes) = run(&model, &op);
    println!("P1 apply codes: {codes:?}");
    let applied = applied.expect("P1: pressure_profile applied");
    println!("P1 applied schema_version = {}", applied["schema_version"]);
    assert_eq!(applied["schema_version"], "0.3.0");
    assert!(applied.get("reference_configurations").is_some());
    // The product refuses the downgraded model (never silently reinterpreted).
    let mut req = request();
    req["model"] = applied.clone();
    let solved = run_linear_static_preview_value_with_mode(req, PreviewSolverMode::SparseInteractive);
    match solved {
        Err(e) => println!("P1 product: Err {e}"),
        Ok(env) => { let v = serde_json::to_value(&env).unwrap(); println!("P1 product: status {} codes {:?}", v["status"], v["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"]=="blocking").map(|d| d["code"].clone()).collect::<Vec<_>>()); }
    }
    // The typed load-state operations now refuse, so the records cannot be removed through them.
    let current = canonical_json(&applied["reference_configurations"]);
    let op2 = intent("Model", &project, "set_field", "reference_configurations", &current, "not_present");
    let (a2, codes2) = run(&applied, &op2);
    println!("P1 follow-up removal codes: {codes2:?}");
    assert!(a2.is_none());
    // Operation-level inverse (restore 0.4.0) through pressure_profile.
    let undo = intent("Model", &project, "set_field", "pressure_profile", &canonical_json(&json!({"schema_version":"0.3.0","pressure_contract":{"version":"2.0.0","mode":"exact_straight_pressure_v2"}})), &before);
    let (a3, codes3) = run(&applied, &undo);
    println!("P1 inverse codes: {codes3:?}; applied={}", a3.is_some());
}

/// P2: replacing Material/temperature_points can orphan an analysis_state point_ref.
#[test]
fn p2_temperature_points_replacement_orphans_point_ref() {
    let model = connected();
    let material = model["materials"][0]["id"].as_str().unwrap().to_string();
    let current = model["materials"][0]["temperature_points"].clone();
    let kept: Vec<Value> = current.as_array().unwrap().iter().filter(|p| p["id"] != "point:cold").cloned().collect();
    // Re-id the cold point instead of removing it, so nothing else changes.
    let mut renamed = current.clone();
    renamed[0]["id"] = json!("point:cold-renamed");
    for (label, after) in [("remove", Value::Array(kept)), ("rename", renamed)] {
        let op = intent("Material", &material, "set_field", "temperature_points", &canonical_json(&current), &after.to_string());
        let (applied, codes) = run(&model, &op);
        println!("P2 {label}: applied={} codes={codes:?}", applied.is_some());
        if let Some(applied) = applied {
            let names = applied["load_cases"][0]["analysis_state"]["element_states"][0]["material_selection"]["point_ref"].clone();
            println!("P2 {label}: case:cold element 0 still names {names}");
            let mut req = request();
            req["model"] = applied;
            match run_linear_static_preview_value_with_mode(req, PreviewSolverMode::SparseInteractive) {
                Err(e) => println!("P2 {label} product: Err {e}"),
                Ok(env) => { let v = serde_json::to_value(&env).unwrap(); println!("P2 {label} product: mechanics {} blocking {:?}", v["status"]["mechanics"], v["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"]=="blocking").map(|d| d["code"].clone()).collect::<Vec<_>>()); }
            }
        }
    }
}

/// P3: an explicit-null prior value cannot be restored by the operation-level inverse.
#[test]
fn p3_explicit_null_prior_is_not_restorable() {
    let mut model = connected();
    let state = model["load_cases"][0]["analysis_state"].clone();
    model["load_cases"][0]["analysis_state"] = Value::Null;
    let op = intent("Load", "case:cold", "update_load", "analysis_state", "null", &state.to_string());
    let (applied, codes) = run(&model, &op);
    println!("P3 forward: applied={} codes={codes:?}", applied.is_some());
    let applied = applied.unwrap();
    let undo = intent("Load", "case:cold", "update_load", "analysis_state", &canonical_json(&state), "null");
    let (undone, codes) = run(&applied, &undo);
    println!("P3 inverse: applied={} codes={codes:?}", undone.is_some());
}

/// P4: dense-mode value is not compared by the WP3 closed-form helper; show both modes agree here.
#[test]
fn p4_dense_and_sparse_agree_on_connected_middle_ux() {
    let req = request();
    let mut vals = vec![];
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let v = serde_json::to_value(&run_linear_static_preview_value_with_mode(req.clone(), mode).unwrap()).unwrap();
        let row = v["results"].as_array().unwrap().iter().find(|r| r["entity_ref"]=="node:middle" && r["kind"]=="global_nodal_displacement_x" && r["basis_ref"]["ref_id"]=="case:cold").unwrap().clone();
        vals.push(row["value"].as_f64().unwrap());
    }
    println!("P4 sparse {} dense {} diff {:e}", vals[0], vals[1], vals[0]-vals[1]);
}

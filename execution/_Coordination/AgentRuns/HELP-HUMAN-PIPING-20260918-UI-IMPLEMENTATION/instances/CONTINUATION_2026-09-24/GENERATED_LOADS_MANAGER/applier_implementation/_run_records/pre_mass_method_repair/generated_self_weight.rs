use open_pipe_stress_operation_applier::{apply_operation, apply_operation_batch, canonical_json, sha256_hex};
use serde_json::{json, Value};
fn operation(payload: &Value, before: &str, create: bool, id: &str) -> Value {
    json!({"operation_id":format!("operation:{id}"),"operation_kind":if create{"create"}else{"modify"},"operation_status":"proposed","author_type":"user","target":{"object_type":"Load","ref":"load:s"},"change":{"change_id":format!("change:{id}"),"change_kind":if create{"create_support"}else{"update_load"},"field_label":"Load","field_path":if create{"supports"}else{"generated_self_weight"},"before":before,"after":canonical_json(payload),"unit":"none","dimension":"dimensionless","source_note":"Explicit fixture"},"validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},"audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},"professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},"rationale":"Explicit fixture"})
}
fn claim(m: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(m)))})
}

fn fixture() -> (Value, Value) {
    let mass=json!({"outside_diameter":{"value":0.1,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"},"material_density":{"value":1000,"unit":"kg/m^3"},"mill_tolerance":null,"contents_density":null,"insulation_thickness":null,"insulation_density":null});
    let prior = json!({"method":"pipe_mass_per_length_times_explicit_axis_acceleration/v1","pipe_id":"pipe:p","request_provenance":"fixture","gravity":{"value":-10,"unit":"m/s^2","axis":"global_y"},"normalized_acceleration_m_per_s2":-10,"mass_kg_per_m":1,"mass_inputs":mass,"section_ref":null,"referenced_section":null});
    let primitive = json!({"id":"weight:p","category":"distributed_force","target":{"type":"element","pipe":"pipe:p"},"direction":"global_y","magnitude":{"value":-10,"unit":"N/m"},"dimension":"force_per_length","provenance":prior.to_string(),"annotation":"preserve exactly"});
    let m = json!({"project":{},"pipe_segments":[{"id":"pipe:p"}],"load_cases":[{"id":"load:s","primitive_loads":[primitive]}]});
    let mut next = m["load_cases"][0]["primitive_loads"].clone();
    next[0]["magnitude"]["value"] = json!(-20);
    let mut payload = next[0].clone();
    payload.as_object_mut().unwrap().remove("provenance");
    payload.as_object_mut().unwrap().remove("annotation");
    let mut updated=prior.clone(); updated["method"]=json!("pipe_mass_per_length_times_explicit_axis_acceleration/v2"); updated["generated_payload"]=payload; updated["normalized_dependencies"]=mass; updated["normalized_dependencies"]["section_ref"]=Value::Null; updated["source_model_hash"]=claim(&m)["value"].clone();
    next[0]["provenance"] = json!(updated.to_string());
    (m,next)
}
fn edit(m: &Value, next: &Value, id: &str) -> Value {
    operation(next,&canonical_json(&m["load_cases"][0]["primitive_loads"]),false,id)
}
#[test]
fn refresh_needs_current_hash_and_preserves_identity() {
    let (m,next)=fixture(); let op=edit(&m,&next,"refresh");
    assert!(apply_operation(&m,&op,None).applied_model.is_none());
    let out=apply_operation(&m,&op,Some(&claim(&m)));
    assert_eq!(out.applied_model.as_ref().unwrap()["load_cases"][0]["primitive_loads"],next,"{:?}",out.diagnostics);
    let mut altered=next.clone(); altered[0]["annotation"]=json!("changed");
    assert!(apply_operation(&m,&edit(&m,&altered,"bad"),Some(&claim(&m))).applied_model.is_none());
    let mut duplicate=next.clone(); duplicate.as_array_mut().unwrap().push(next[0].clone());
    assert!(apply_operation(&m,&edit(&m,&duplicate,"dup"),Some(&claim(&m))).applied_model.is_none());
}
#[test]
fn manual_detach_preserves_every_physical_field() {
    let (m,_)=fixture(); let mut next=m["load_cases"][0]["primitive_loads"].clone();
    let prior: Value=serde_json::from_str(next[0]["provenance"].as_str().unwrap()).unwrap();
    next[0]["provenance"]=json!(json!({"method":"manual_override_of_generated_self_weight/v1","original_generation_provenance":prior,"decision":"preserve_modified_generated_load","source_model_hash":claim(&m)["value"]}).to_string());
    assert!(apply_operation(&m,&edit(&m,&next,"manual"),Some(&claim(&m))).applied_model.is_some());
    next[0]["magnitude"]["value"]=json!(-99);
    assert!(apply_operation(&m,&edit(&m,&next,"bad"),Some(&claim(&m))).applied_model.is_none());
}
#[test]
fn source_plan_guard_and_late_failure_publish_no_model() {
    let (m,next)=fixture(); let op=edit(&m,&next,"one");
    let mut batch=json!({"batch_id":"batch","source_model_hash":claim(&m)["value"],"operations":[op.clone()]});
    assert!(apply_operation_batch(&m,&batch,Some(&claim(&m)))["applied_model"].is_object());
    batch["source_model_hash"]=json!(format!("sha256:{}","0".repeat(64)));
    let failed=apply_operation_batch(&m,&batch,Some(&claim(&m)));
    assert!(failed["applied_model"].is_null()); assert_eq!(failed["operation_outcomes"],json!([]));
    batch["source_model_hash"]=claim(&m)["value"].clone();
    batch["operations"].as_array_mut().unwrap().push(edit(&m,&next,"two"));
    let failed=apply_operation_batch(&m,&batch,Some(&claim(&m)));
    assert!(failed["applied_model"].is_null()); assert_eq!(failed["operation_outcomes"].as_array().unwrap().len(),2);
    batch["source_model_hash"]=json!("malformed");
    assert!(apply_operation_batch(&m,&batch,Some(&claim(&m)))["operation_outcomes"].as_array().unwrap().is_empty());
}


#[test]
fn canonical_transport_preserves_integral_float_manual_records_and_detached_physics() {
    let (mut m, mut next) = fixture();
    let manual = json!({"id":"manual-tip","category":"point_force","target":{"type":"node","node":"tip"},"direction":"global_y","magnitude":{"value":-11.0,"unit":"N"},"dimension":"force","provenance":"ordinary manual input","extra":{"number":2.0}});
    m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(manual.clone());
    next.as_array_mut().unwrap().push(manual.clone());
    m["load_cases"][0]["primitive_loads"][0]["extra"] = json!({"number":3.0});
    next[0]["extra"] = json!({"number":3.0});
    let mut record: Value = serde_json::from_str(next[0]["provenance"].as_str().unwrap()).unwrap();
    record["source_model_hash"] = claim(&m)["value"].clone();
    next[0]["provenance"] = json!(record.to_string());
    let out = apply_operation(&m, &edit(&m,&next,"refresh-float"), Some(&claim(&m)));
    let applied = out.applied_model.as_ref().unwrap_or_else(|| panic!("{:?}",out.diagnostics));
    assert_eq!(applied["load_cases"][0]["primitive_loads"][1], manual);
    assert_eq!(applied["load_cases"][0]["primitive_loads"][0]["extra"], m["load_cases"][0]["primitive_loads"][0]["extra"]);
    next[1]["magnitude"]["value"] = json!(-12);
    assert!(apply_operation(&m,&edit(&m,&next,"changed-manual"),Some(&claim(&m))).applied_model.is_none());

    m["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(-99.0);
    let original = m["load_cases"][0]["primitive_loads"][0].clone();
    let prior: Value = serde_json::from_str(original["provenance"].as_str().unwrap()).unwrap();
    let mut detached = m["load_cases"][0]["primitive_loads"].clone();
    detached[0]["provenance"] = json!(json!({"method":"manual_override_of_generated_self_weight/v1","original_generation_provenance":prior,"decision":"preserve_modified_generated_load","source_model_hash":claim(&m)["value"]}).to_string());
    let out = apply_operation(&m,&edit(&m,&detached,"detach-float"),Some(&claim(&m)));
    let applied = out.applied_model.as_ref().unwrap_or_else(|| panic!("{:?}",out.diagnostics));
    let mut expected = original.clone(); expected["provenance"] = detached[0]["provenance"].clone();
    assert_eq!(applied["load_cases"][0]["primitive_loads"][0], expected);
    assert_eq!(applied["load_cases"][0]["primitive_loads"][1], manual);
}


fn manual_preservation(m: &Value) -> Value {
    let mut next = m["load_cases"][0]["primitive_loads"].clone();
    let prior: Value = serde_json::from_str(next[0]["provenance"].as_str().unwrap()).unwrap();
    next[0]["provenance"] = json!(json!({"method":"manual_override_of_generated_self_weight/v1","original_generation_provenance":prior,"decision":"preserve_modified_generated_load","source_model_hash":claim(m)["value"]}).to_string());
    next
}

#[test]
fn explicit_manual_preservation_allows_existing_retarget_with_deleted_original_pipe() {
    let (mut m, _) = fixture();
    m["pipe_segments"] = json!([{"id":"pipe:retarget"}]);
    m["load_cases"][0]["primitive_loads"][0]["target"]["pipe"] = json!("pipe:retarget");
    let next = manual_preservation(&m);
    let out = apply_operation(&m,&edit(&m,&next,"preserve-retarget"),Some(&claim(&m)));
    assert_eq!(out.applied_model.as_ref().unwrap_or_else(|| panic!("{:?}",out.diagnostics))["load_cases"][0]["primitive_loads"],next);
    let mut nonexistent = m.clone(); nonexistent["pipe_segments"] = json!([]);
    assert!(apply_operation(&nonexistent,&edit(&nonexistent,&manual_preservation(&nonexistent),"missing-retarget"),Some(&claim(&nonexistent))).applied_model.is_none());
    m["pipe_segments"].as_array_mut().unwrap().push(json!({"id":"pipe:other"}));
    let mut overwritten = manual_preservation(&m); overwritten[0]["target"]["pipe"] = json!("pipe:other");
    assert!(apply_operation(&m,&edit(&m,&overwritten,"overwrite-retarget"),Some(&claim(&m))).applied_model.is_none());
}

#[test]
fn mill_tolerance_uses_absolute_length_for_refresh_and_preservation() {
    for unit in ["mm", "1"] {
        let (mut m, mut next) = fixture();
        let mut prior: Value = serde_json::from_str(m["load_cases"][0]["primitive_loads"][0]["provenance"].as_str().unwrap()).unwrap();
        prior["mass_inputs"]["mill_tolerance"] = json!({"value":0.5,"unit":unit});
        m["load_cases"][0]["primitive_loads"][0]["provenance"] = json!(prior.to_string());
        let mut refreshed: Value = serde_json::from_str(next[0]["provenance"].as_str().unwrap()).unwrap();
        refreshed["mass_inputs"]["mill_tolerance"] = prior["mass_inputs"]["mill_tolerance"].clone();
        refreshed["normalized_dependencies"]["mill_tolerance"] = json!({"value":0.0005,"unit":"m"});
        refreshed["source_model_hash"] = claim(&m)["value"].clone();
        next[0]["provenance"] = json!(refreshed.to_string());
        for (name, replacement) in [("refresh-mill",next),("preserve-mill",manual_preservation(&m))] {
            let out = apply_operation(&m,&edit(&m,&replacement,name),Some(&claim(&m)));
            assert_eq!(out.applied_model.is_some(),unit == "mm","{unit}: {:?}",out.diagnostics);
            if let Some(applied) = out.applied_model { assert_eq!(applied["load_cases"][0]["primitive_loads"],replacement); }
        }
    }
}

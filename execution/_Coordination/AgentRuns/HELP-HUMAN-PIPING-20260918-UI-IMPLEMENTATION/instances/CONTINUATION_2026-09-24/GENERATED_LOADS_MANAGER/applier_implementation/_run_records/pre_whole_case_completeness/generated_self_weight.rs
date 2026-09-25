use open_pipe_stress_operation_applier::{apply_operation, apply_operation_batch, canonical_json, sha256_hex};
use serde_json::{json, Value};
use open_pipe_stress_product_physics::{PreviewModel, self_weight::{inspect_applied_self_weight, generate_self_weight_operations, SelfWeightRequest, AppliedSelfWeightState}};
fn operation(payload: &Value, before: &str, create: bool, id: &str) -> Value {
    json!({"operation_id":format!("operation:{id}"),"operation_kind":if create{"create"}else{"modify"},"operation_status":"proposed","author_type":"user","target":{"object_type":"Load","ref":"load:s"},"change":{"change_id":format!("change:{id}"),"change_kind":if create{"create_support"}else{"update_load"},"field_label":"Load","field_path":if create{"supports"}else{"generated_self_weight"},"before":before,"after":canonical_json(payload),"unit":"none","dimension":"dimensionless","source_note":"Explicit fixture"},"validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},"audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},"professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},"rationale":"Explicit fixture"})
}
fn claim(m: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(m)))})
}

fn legacy_model() -> Value {
    let mut model: Value = serde_json::from_str(include_str!("../../../loads/self_weight_wasm/tests/fixtures/applied_self_weight_v1.json")).unwrap();
    model["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().truncate(1);
    model["load_cases"][0]["primitive_loads"][0]["annotation"] = json!("preserve exactly");
    model
}
fn refresh(m: &Value) -> Value {
    let typed: PreviewModel = serde_json::from_value(m.clone()).unwrap();
    let hash = claim(m)["value"].as_str().unwrap().to_owned();
    let states = inspect_applied_self_weight(&typed, Some(&hash));
    let mut next = m["load_cases"][0]["primitive_loads"].clone();
    for status in states.iter().filter(|s| s.case_id == m["load_cases"][0]["id"].as_str().unwrap()) {
        if let Some(replacement) = &status.replacement {
            next[status.primitive_index]["magnitude"] = replacement["magnitude"].clone();
            next[status.primitive_index]["provenance"] = replacement["provenance"].clone();
        }
    }
    assert_ne!(next,m["load_cases"][0]["primitive_loads"],"{states:?}");
    next
}
fn fixture() -> (Value, Value) { let model = legacy_model(); let next = refresh(&model); (model,next) }
fn generated_model(mill: bool) -> Value {
    let mut model = legacy_model(); model["load_cases"] = json!([]);
    if mill { model["pipe_segments"][0]["section"]["mill_tolerance"] = json!({"value":0.5,"unit":"mm"}); }
    let request: SelfWeightRequest = serde_json::from_value(json!({"case_id":"case:weight","label":"Invented self weight","pipe_refs":["pipe:beam"],"gravity":{"value":-7,"unit":"m/s^2","axis":"global_y"},"provenance":"explicit invented gravity","source_model_hash":claim(&model)["value"]})).unwrap();
    let typed: PreviewModel = serde_json::from_value(model.clone()).unwrap();
    let plan = generate_self_weight_operations(&typed,&request).unwrap();
    let mut case: Value = serde_json::from_str(&plan.changes[0].after).unwrap();
    let primitive: Value = serde_json::from_str(&plan.changes[1].after).unwrap();
    case["primitive_loads"] = json!([primitive]); model["load_cases"] = json!([case]); model
}
fn edit(m: &Value, next: &Value, id: &str) -> Value {
    let mut op = operation(next,&canonical_json(&m["load_cases"][0]["primitive_loads"]),false,id);
    op["target"]["ref"] = m["load_cases"][0]["id"].clone(); op
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
    let (mut m,_)=fixture(); m["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(-70.0);
    let mut next=m["load_cases"][0]["primitive_loads"].clone();
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
    let manual = json!({"id":"manual-tip","category":"concentrated_force","target":{"type":"node","node":"node:tip"},"direction":"global_y","magnitude":{"value":-11.0,"unit":"N"},"dimension":"force","provenance":"ordinary manual input","extra":{"number":2.0}});
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
    m["pipe_segments"][0]["id"] = json!("pipe:retarget");
    m["load_cases"][0]["primitive_loads"][0]["target"]["pipe"] = json!("pipe:retarget");
    let next = manual_preservation(&m);
    let out = apply_operation(&m,&edit(&m,&next,"preserve-retarget"),Some(&claim(&m)));
    assert_eq!(out.applied_model.as_ref().unwrap_or_else(|| panic!("{:?}",out.diagnostics))["load_cases"][0]["primitive_loads"],next);
    let mut nonexistent = m.clone(); nonexistent["pipe_segments"] = json!([]);
    assert!(apply_operation(&nonexistent,&edit(&nonexistent,&manual_preservation(&nonexistent),"missing-retarget"),Some(&claim(&nonexistent))).applied_model.is_none());
    let mut other = m["pipe_segments"][0].clone(); other["id"] = json!("pipe:other");
    m["pipe_segments"].as_array_mut().unwrap().push(other);
    let mut overwritten = manual_preservation(&m); overwritten[0]["target"]["pipe"] = json!("pipe:other");
    assert!(apply_operation(&m,&edit(&m,&overwritten,"overwrite-retarget"),Some(&claim(&m))).applied_model.is_none());
}

#[test]
fn mill_tolerance_uses_absolute_length_for_refresh_and_preservation() {
    let mut m = generated_model(true);
    m["pipe_segments"][0]["section"]["material_density"]["value"] = json!(2000);
    let next = refresh(&m);
    let out = apply_operation(&m,&edit(&m,&next,"refresh-mill"),Some(&claim(&m)));
    assert!(out.applied_model.is_some(),"{:?}",out.diagnostics);
    m["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(-99.0);
    let preserved = manual_preservation(&m);
    let out = apply_operation(&m,&edit(&m,&preserved,"preserve-mill"),Some(&claim(&m)));
    assert!(out.applied_model.is_some(),"{:?}",out.diagnostics);
    let mut prior: Value = serde_json::from_str(m["load_cases"][0]["primitive_loads"][0]["provenance"].as_str().unwrap()).unwrap();
    prior["mass_inputs"]["mill_tolerance"]["unit"] = json!("1");
    m["load_cases"][0]["primitive_loads"][0]["provenance"] = json!(prior.to_string());
    assert!(apply_operation(&m,&edit(&m,&manual_preservation(&m),"dimensionless-mill"),Some(&claim(&m))).applied_model.is_none());
}

#[test]
fn stable_mass_method_and_verbatim_legacy_lineage_are_required() {
    let (m, next) = fixture();
    let original = m["load_cases"][0]["primitive_loads"][0]["provenance"].clone();
    let record: Value = serde_json::from_str(next[0]["provenance"].as_str().unwrap()).unwrap();
    assert_eq!(record["legacy_generation_provenance"], original);
    for key in ["mass_method", "legacy_generation_provenance"] {
        let mut bad = next.clone(); let mut changed = record.clone();
        changed.as_object_mut().unwrap().remove(key);
        bad[0]["provenance"] = json!(changed.to_string());
        assert!(apply_operation(&m,&edit(&m,&bad,key),Some(&claim(&m))).applied_model.is_none());
    }
    // Even semantically identical reserialization of historical provenance is
    // not verbatim lineage retention.
    let mut bad = next.clone(); let mut changed = record.clone();
    changed["legacy_generation_provenance"] = json!(format!(" {}",original.as_str().unwrap()));
    bad[0]["provenance"] = json!(changed.to_string());
    assert!(apply_operation(&m,&edit(&m,&bad,"rewritten-legacy"),Some(&claim(&m))).applied_model.is_none());
    let first = apply_operation(&m,&edit(&m,&next,"first"),Some(&claim(&m))).applied_model.unwrap();
    let mut subsequent = first["load_cases"][0]["primitive_loads"].clone();
    let mut refreshed: Value = serde_json::from_str(subsequent[0]["provenance"].as_str().unwrap()).unwrap();
    refreshed["source_model_hash"] = claim(&first)["value"].clone();
    subsequent[0]["provenance"] = json!(refreshed.to_string());
    assert!(apply_operation(&first,&edit(&first,&subsequent,"later"),Some(&claim(&first))).applied_model.is_some());
    let mut native_v2 = first.clone();
    let mut native_record: Value = serde_json::from_str(native_v2["load_cases"][0]["primitive_loads"][0]["provenance"].as_str().unwrap()).unwrap();
    native_record.as_object_mut().unwrap().remove("legacy_generation_provenance");
    native_v2["load_cases"][0]["primitive_loads"][0]["provenance"] = json!(native_record.to_string());
    let mut invented = native_v2["load_cases"][0]["primitive_loads"].clone();
    native_record["source_model_hash"] = claim(&native_v2)["value"].clone();
    native_record["legacy_generation_provenance"] = original;
    invented[0]["provenance"] = json!(native_record.to_string());
    assert!(apply_operation(&native_v2,&edit(&native_v2,&invented,"invented-lineage"),Some(&claim(&native_v2))).applied_model.is_none());
    refreshed["legacy_generation_provenance"] = first["load_cases"][0]["primitive_loads"][0]["provenance"].clone();
    subsequent[0]["provenance"] = json!(refreshed.to_string());
    assert!(apply_operation(&first,&edit(&first,&subsequent,"nested"),Some(&claim(&first))).applied_model.is_none());
}


fn assert_direct_and_imported_blocked(m: &Value, next: &Value, id: &str) {
    let op = edit(m,next,id);
    assert!(apply_operation(m,&op,Some(&claim(m))).applied_model.is_none());
    let batch = json!({"batch_id":id,"source_model_hash":claim(m)["value"],"operations":[op]});
    let out = apply_operation_batch(m,&batch,Some(&claim(m)));
    assert!(out["applied_model"].is_null(),"{out}");
}
#[test]
fn source_inspection_prevents_direct_and_imported_override_replacement() {
    let mut m = generated_model(false);
    m["pipe_segments"][0]["section"]["material_density"]["value"] = json!(2000);
    let mut next = refresh(&m);
    m["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(-123.0);
    let mut provenance: Value = serde_json::from_str(next[0]["provenance"].as_str().unwrap()).unwrap();
    provenance["source_model_hash"] = claim(&m)["value"].clone();
    next[0]["provenance"] = json!(provenance.to_string());
    assert_direct_and_imported_blocked(&m,&next,"overwrite-v2");
    let detached = manual_preservation(&m);
    let out = apply_operation(&m,&edit(&m,&detached,"preserve-v2"),Some(&claim(&m)));
    assert!(out.applied_model.is_some(),"{:?}",out.diagnostics);
    let pristine = generated_model(false);
    assert_direct_and_imported_blocked(&pristine,&manual_preservation(&pristine),"detach-unmodified");
}
#[test]
fn source_inspection_rejects_forged_or_stale_after_basis_even_with_current_hash() {
    let mut m = generated_model(false);
    m["pipe_segments"][0]["section"]["material_density"]["value"] = json!(2000);
    let mut stale = m["load_cases"][0]["primitive_loads"].clone();
    let mut record: Value = serde_json::from_str(stale[0]["provenance"].as_str().unwrap()).unwrap();
    record["source_model_hash"] = claim(&m)["value"].clone();
    stale[0]["provenance"] = json!(record.to_string());
    assert_direct_and_imported_blocked(&m,&stale,"stale-after");
    let mut forged = refresh(&m);
    let mut record: Value = serde_json::from_str(forged[0]["provenance"].as_str().unwrap()).unwrap();
    forged[0]["magnitude"]["value"] = json!(-999.0);
    record["generated_payload"]["magnitude"] = forged[0]["magnitude"].clone();
    forged[0]["provenance"] = json!(record.to_string());
    assert_direct_and_imported_blocked(&m,&forged,"forged-after");
    let mut forged = refresh(&m);
    let mut record: Value = serde_json::from_str(forged[0]["provenance"].as_str().unwrap()).unwrap();
    record["normalized_dependencies"]["material_density"]["value"] = json!(3000.0);
    forged[0]["provenance"] = json!(record.to_string());
    assert_direct_and_imported_blocked(&m,&forged,"forged-dependencies");
}
#[test]
fn changed_or_ambiguous_legacy_record_cannot_be_managed_refreshed() {
    for ambiguous in [false,true] {
        let (mut m,mut next) = fixture();
        if ambiguous {
            let mut prior: Value = serde_json::from_str(m["load_cases"][0]["primitive_loads"][0]["provenance"].as_str().unwrap()).unwrap();
            prior["mass_kg_per_m"] = json!(3.0);
            m["load_cases"][0]["primitive_loads"][0]["provenance"] = json!(prior.to_string());
        } else { m["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(-123.0); }
        let mut record: Value = serde_json::from_str(next[0]["provenance"].as_str().unwrap()).unwrap();
        record["source_model_hash"] = claim(&m)["value"].clone();
        record["legacy_generation_provenance"] = m["load_cases"][0]["primitive_loads"][0]["provenance"].clone();
        next[0]["provenance"] = json!(record.to_string());
        assert_direct_and_imported_blocked(&m,&next,"legacy-override");
        let typed: PreviewModel = serde_json::from_value(m.clone()).unwrap();
        assert_eq!(inspect_applied_self_weight(&typed,None)[0].state,AppliedSelfWeightState::Modified);
    }
}


#[test]
fn internally_valid_doubled_gravity_cannot_replace_original_explicit_gravity() {
    let m = generated_model(false);
    let original = m.clone();
    // Generate a fully source-consistent record using an unauthorized new
    // gravity. Neither hash freshness nor candidate self-consistency grants
    // authority to replace the original user's acceleration during refresh.
    let mut generation_source = m.clone(); generation_source["load_cases"] = json!([]);
    let request: SelfWeightRequest = serde_json::from_value(json!({"case_id":"case:weight","label":"Invented self weight","pipe_refs":["pipe:beam"],"gravity":{"value":-14,"unit":"m/s^2","axis":"global_y"},"provenance":"explicit invented gravity","source_model_hash":claim(&m)["value"]})).unwrap();
    let typed: PreviewModel = serde_json::from_value(generation_source).unwrap();
    let plan = generate_self_weight_operations(&typed,&request).unwrap();
    let regenerated: Value = serde_json::from_str(&plan.changes[1].after).unwrap();
    let mut replacement = m["load_cases"][0]["primitive_loads"].clone();
    replacement[0]["magnitude"] = regenerated["magnitude"].clone();
    replacement[0]["provenance"] = regenerated["provenance"].clone();
    let mut consistent_candidate = m.clone();
    consistent_candidate["load_cases"][0]["primitive_loads"] = replacement.clone();
    let typed_candidate: PreviewModel = serde_json::from_value(consistent_candidate).unwrap();
    assert_eq!(inspect_applied_self_weight(&typed_candidate,None)[0].state,AppliedSelfWeightState::Fresh);
    assert_direct_and_imported_blocked(&m,&replacement,"unauthorized-gravity");
    assert_eq!(m,original);
}


#[test]
fn managed_refresh_cannot_rewrite_original_user_generation_provenance() {
    let mut m = generated_model(false);
    m["pipe_segments"][0]["section"]["material_density"]["value"] = json!(2000);
    let original = m.clone();
    let mut replacement = refresh(&m);
    let mut provenance: Value = serde_json::from_str(replacement[0]["provenance"].as_str().unwrap()).unwrap();
    provenance["request_provenance"] = json!("rewritten user intent");
    replacement[0]["provenance"] = json!(provenance.to_string());
    let mut candidate = m.clone(); candidate["load_cases"][0]["primitive_loads"] = replacement.clone();
    let typed: PreviewModel = serde_json::from_value(candidate).unwrap();
    assert_eq!(inspect_applied_self_weight(&typed,None)[0].state,AppliedSelfWeightState::Fresh);
    assert_direct_and_imported_blocked(&m,&replacement,"rewritten-generation-intent");
    assert_eq!(m,original);
}

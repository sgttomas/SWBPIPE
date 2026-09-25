use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_operation,
    validate_operation_batch,
};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewSolverMode,
};
use serde_json::{json, Value};
fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/model_operations/exact_pressure_authoring_model.json"
    ))
    .unwrap()
}
fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}
fn intent(object: &str, id: &str, path: &str, before: Option<&Value>, after: &Value) -> Value {
    json!({"operation_id":format!("op:author:{id}:{path}"),"operation_kind":"modify","operation_status":"proposed","author_type":"user","target":{"object_type":object,"ref":id},
      "change":{"change_id":format!("change:author:{id}:{path}"),"change_kind":if object=="Load" {"update_load"} else {"set_field"},"field_label":path,"field_path":path,"before":before.map(canonical_json).unwrap_or("not_present".into()),"after":canonical_json(after),"unit":"none","dimension":"dimensionless","source_note":"Synthetic explicitly entered fixture values"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "rationale":"Synthetic explicit user authoring, reviewed through the operation pipeline",
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false}})
}
fn apply(model: &Value, intent: &Value) -> Value {
    let claim = hash(model);
    let preview = validate_operation(model, intent, Some(&claim));
    assert!(preview.applied_model.is_none());
    assert_eq!(preview.diff_preview.len(), 1, "{:?}", preview.diagnostics);
    let outcome = apply_operation(model, intent, Some(&claim));
    outcome
        .applied_model
        .expect(&format!("{:?}", outcome.diagnostics))
}
fn projection(value: &Value, keys: &[&str]) -> Value {
    Value::Object(
        keys.iter()
            .filter_map(|k| value.get(*k).map(|v| ((*k).into(), v.clone())))
            .collect(),
    )
}
fn legacy_authoring_model(expected: &Value) -> Value {
    let mut model = expected.clone();
    model["schema_version"] = json!("0.2.0");
    model.as_object_mut().unwrap().remove("pressure_contract");
    model["materials"][0]
        .as_object_mut()
        .unwrap()
        .remove("poisson_ratio");
    model["materials"][0]
        .as_object_mut()
        .unwrap()
        .remove("constitutive_basis");
    model["materials"][0]["shear_modulus"] = json!({"value":7.7e10,"unit":"Pa"});
    for case in model["load_cases"].as_array_mut().unwrap() {
        case.as_object_mut().unwrap().remove("pressure_regions");
    }
    model
}
#[test]
fn exact_authoring_review_apply_serialization_and_real_solve() {
    let expected = fixture();
    let mut model = legacy_authoring_model(&expected);
    let expected_material = expected["materials"][0].clone();
    let keys = [
        "constitutive_basis",
        "elastic_modulus",
        "shear_modulus",
        "poisson_ratio",
        "provenance",
    ];
    let op = intent(
        "Material",
        expected_material["id"].as_str().unwrap(),
        "constitutive_properties",
        Some(&projection(&model["materials"][0], &keys)),
        &projection(&expected_material, &keys),
    );
    model = apply(&model, &op);
    for case in expected["load_cases"].as_array().unwrap() {
        let op = intent(
            "Load",
            case["id"].as_str().unwrap(),
            "pressure_regions",
            None,
            &case["pressure_regions"],
        );
        model = apply(&model, &op);
    }
    let op = intent(
        "Model",
        expected["project"]["id"].as_str().unwrap(),
        "pressure_profile",
        Some(&json!({"schema_version":"0.2.0"})),
        &projection(&expected, &["schema_version", "pressure_contract"]),
    );
    model = apply(&model, &op);
    assert_eq!(canonical_json(&model), canonical_json(&expected));
    // Plain JSON transport preserves the authored namespace; native store is checked by integration.
    let restored: Value = serde_json::from_str(&serde_json::to_string(&model).unwrap()).unwrap();
    assert_eq!(canonical_json(&restored), canonical_json(&expected));
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let request: LinearStaticPreviewRequest =
            serde_json::from_value(json!({"model":restored,"materials":[]})).unwrap();
        let result = run_linear_static_preview_with_mode(request, mode);
        assert_eq!(
            result.status.mechanics, "MECHANICS_SOLVED",
            "{:?}",
            result.diagnostics
        );
        let row = result
            .results
            .iter()
            .find(|r| {
                r.basis_ref
                    .as_ref()
                    .is_some_and(|b| b.ref_id == "case:closed-pressure")
                    && r.entity_ref == "node:fixture-tip"
                    && r.kind == "global_nodal_displacement_x"
            })
            .unwrap();
        // Closed ends: axial strain = p*ri^2/(ro^2-ri^2)*(1-2nu)/E, L=1 m.
        assert_eq!(row.unit, "mm");
        let expected =
            1000.0 * 2e6 * 0.05_f64.powi(2) / (0.06_f64.powi(2) - 0.05_f64.powi(2)) * 0.4 / 2e11;
        assert!(
            (row.value - expected).abs() < expected * 1e-9,
            "{} vs {}",
            row.value,
            expected
        );
    }
}
#[test]
fn exact_authoring_blocks_missing_explicit_inputs_and_stale_basis() {
    let model = fixture();
    let profile = projection(&model, &["schema_version", "pressure_contract"]);
    let op = intent(
        "Model",
        model["project"]["id"].as_str().unwrap(),
        "pressure_profile",
        Some(&profile),
        &profile,
    );
    assert!(apply_operation(&model, &op, None).applied_model.is_none());
    let mut missing = model.clone();
    missing["load_cases"][0]
        .as_object_mut()
        .unwrap()
        .remove("pressure_regions");
    assert!(apply_operation(&missing, &op, Some(&hash(&missing)))
        .applied_model
        .is_none());
    let mut primitive = model.clone();
    primitive["load_cases"][0]["primitive_loads"] =
        json!([{"category":"pressure","magnitude":{"value":0,"unit":"Pa"}}]);
    assert!(apply_operation(&primitive, &op, Some(&hash(&primitive)))
        .applied_model
        .is_none());
    let mut region = model["load_cases"][0]["pressure_regions"].clone();
    region[0]["terminals"][0]
        .as_object_mut()
        .unwrap()
        .remove("closure_transfer");
    let op = intent(
        "Load",
        "case:closed-pressure",
        "pressure_regions",
        Some(&model["load_cases"][0]["pressure_regions"]),
        &region,
    );
    assert!(apply_operation(&model, &op, Some(&hash(&model)))
        .applied_model
        .is_none());
}
#[test]
fn negative_nu_temperature_points_and_exact_material_creation_keep_values() {
    let mut model = fixture();
    let mut material = model["materials"][0].clone();
    material["id"] = json!("material:new");
    material["poisson_ratio"]["value"] = json!(-0.2);
    let mut op = intent("Material", "material:new", "materials", None, &material);
    op["operation_kind"] = json!("create");
    op["change"]["change_kind"] = json!("create_material");
    op["change"]["unit"] = json!("Pa");
    op["change"]["dimension"] = json!("stress");
    model = apply(&model, &op);
    assert_eq!(model["materials"][1], material);
    assert!(model["materials"][1].get("shear_modulus").is_none());
    let points = json!([{"id":"point:auxetic","temperature":{"value":20,"unit":"degC"},"elastic_modulus":{"value":200,"unit":"GPa"},"poisson_ratio":{"value":-0.2,"unit":"1"},"provenance":"synthetic"}]);
    let op = intent(
        "Material",
        "material:new",
        "temperature_points",
        None,
        &points,
    );
    model = apply(&model, &op);
    assert_eq!(model["materials"][1]["temperature_points"], points);
    let mut invalid = points.clone();
    invalid[0]["poisson_ratio"]["value"] = json!(-1);
    let op = intent(
        "Material",
        "material:new",
        "temperature_points",
        Some(&points),
        &invalid,
    );
    assert!(apply_operation(&model, &op, Some(&hash(&model)))
        .applied_model
        .is_none());
}

fn exact_authoring_batch() -> (Value, Value, Value) {
    let expected = fixture();
    let model = legacy_authoring_model(&expected);
    let material = &expected["materials"][0];
    let keys = [
        "constitutive_basis",
        "elastic_modulus",
        "shear_modulus",
        "poisson_ratio",
        "provenance",
    ];
    let mut operations = vec![intent(
        "Material",
        material["id"].as_str().unwrap(),
        "constitutive_properties",
        Some(&projection(&model["materials"][0], &keys)),
        &projection(material, &keys),
    )];
    for case in expected["load_cases"].as_array().unwrap() {
        operations.push(intent(
            "Load",
            case["id"].as_str().unwrap(),
            "pressure_regions",
            None,
            &case["pressure_regions"],
        ));
    }
    operations.push(intent(
        "Model",
        expected["project"]["id"].as_str().unwrap(),
        "pressure_profile",
        Some(&projection(
            &model,
            &["schema_version", "pressure_contract"],
        )),
        &projection(&expected, &["schema_version", "pressure_contract"]),
    ));
    let batch = json!({"batch_id":"batch:exact-authoring-complete", "source_model_hash":hash(&model)["value"], "operations":operations});
    (model, expected, batch)
}
fn assert_batch_unpublished(outcome: &Value) {
    assert!(outcome["applied_model"].is_null(), "{outcome:#}");
    assert!(outcome["applied_model_backend_hash"].is_null());
    assert!(outcome["acceptance"].is_null());
    // Private intermediate models and acceptance must never leak in step records.
    for step in outcome["operation_outcomes"].as_array().unwrap() {
        for field in [
            "applied_model",
            "applied_model_backend_hash",
            "acceptance",
            "model_basis",
        ] {
            assert!(
                step.get(field).is_none(),
                "Private simulation field {field} published"
            );
        }
    }
}
#[test]
fn exact_authoring_atomic_batch_complete_review_and_apply() {
    let (model, expected, batch) = exact_authoring_batch();
    let original = model.clone();
    let claim = hash(&model);
    let reviewed = validate_operation_batch(&model, &batch, Some(&claim));
    assert_eq!(
        reviewed["validation"]["batch_validation_status"], "passed",
        "{reviewed:#}"
    );
    assert_eq!(
        reviewed["simulation_disposition"],
        "validation_only_discarded"
    );
    assert_batch_unpublished(&reviewed);
    let applied = apply_operation_batch(&model, &batch, Some(&claim));
    assert_eq!(
        applied["validation"]["application_status"], "applied_to_session_model",
        "{applied:#}"
    );
    assert_eq!(applied["simulation_disposition"], "committed_as_one_batch");
    assert_eq!(
        canonical_json(&applied["applied_model"]),
        canonical_json(&expected)
    );
    assert_eq!(
        applied["applied_model_backend_hash"],
        hash(&expected)["value"]
    );
    assert_eq!(
        reviewed["operation_outcomes"],
        applied["operation_outcomes"]
    );
    let steps = applied["operation_outcomes"].as_array().unwrap();
    assert_eq!(steps.len(), 4);
    assert_eq!(steps[0]["target_object_type"], "Material");
    assert_eq!(steps[1]["target_object_type"], "Load");
    assert_eq!(steps[2]["target_object_type"], "Load");
    assert_eq!(steps[3]["target_object_type"], "Model");
    for step in steps {
        assert_eq!(step["simulation_status"], "validated_on_temporary_state");
        assert_eq!(step["diff_preview"].as_array().unwrap().len(), 1);
    }
    assert_eq!(model, original);
}
#[test]
fn exact_authoring_atomic_batch_late_invalid_profile_rolls_back_every_edit() {
    let (model, _, mut batch) = exact_authoring_batch();
    let original = model.clone();
    let claim = hash(&model);
    let last = batch["operations"]
        .as_array_mut()
        .unwrap()
        .last_mut()
        .unwrap();
    let mut unsupported: Value =
        serde_json::from_str(last["change"]["after"].as_str().unwrap()).unwrap();
    unsupported["pressure_contract"]["mode"] = json!("unsupported_profile");
    last["change"]["after"] = json!(canonical_json(&unsupported));
    for outcome in [
        validate_operation_batch(&model, &batch, Some(&claim)),
        apply_operation_batch(&model, &batch, Some(&claim)),
    ] {
        assert_eq!(outcome["validation"]["batch_validation_status"], "blocked");
        assert_eq!(
            outcome["simulation_disposition"],
            "rolled_back_no_model_published"
        );
        assert_batch_unpublished(&outcome);
        let steps = outcome["operation_outcomes"].as_array().unwrap();
        assert_eq!(steps.len(), 4, "The invalid last operation must follow three genuine successful simulations: {outcome:#}");
        assert!(steps[..3]
            .iter()
            .all(|step| step["simulation_status"] == "validated_on_temporary_state"));
        assert_eq!(steps[3]["simulation_status"], "blocked");
        assert!(steps[3]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["code"] == "OP-PRESSURE-PAYLOAD-INVALID"));
    }
    assert_eq!(model, original);
    assert_eq!(model["schema_version"], "0.2.0");
    assert!(model["materials"][0].get("poisson_ratio").is_none());
    assert!(model["load_cases"]
        .as_array()
        .unwrap()
        .iter()
        .all(|case| case.get("pressure_regions").is_none()));
}
#[test]
fn exact_authoring_atomic_batch_rejects_stale_caller_and_plan_hashes_before_simulation() {
    let (model, _, batch) = exact_authoring_batch();
    let stale_claim = hash(&model);
    let mut changed = model.clone();
    changed["project"]["name"] = json!("Changed after review");
    let original = changed.clone();
    let current_claim = hash(&changed);
    let mut current_plan = batch.clone();
    current_plan["source_model_hash"] = current_claim["value"].clone();
    // Each binding is independently enforced: stale caller/current plan, then current caller/stale plan.
    for (claim, submitted_batch, expected_code) in [
        (
            &stale_claim,
            &current_plan,
            "OP-CLAIMED-MODEL-HASH-MISMATCH",
        ),
        (
            &current_claim,
            &batch,
            "OP-BATCH-SOURCE-MODEL-HASH-MISMATCH",
        ),
    ] {
        for outcome in [
            validate_operation_batch(&changed, submitted_batch, Some(claim)),
            apply_operation_batch(&changed, submitted_batch, Some(claim)),
        ] {
            assert_batch_unpublished(&outcome);
            assert_eq!(
                outcome["simulation_disposition"],
                "rolled_back_no_model_published"
            );
            assert!(outcome["operation_outcomes"].as_array().unwrap().is_empty());
            assert!(
                outcome["diagnostics"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|d| d["code"] == expected_code),
                "{outcome:#}"
            );
        }
    }
    assert_eq!(changed, original);
}

fn geometry_delete_intent(object: &str, id: &str, path: &str, kind: &str, before: &str) -> Value {
    let mut operation = intent(object, id, path, None, &Value::Null);
    operation["operation_kind"] = json!("delete");
    operation["change"]["change_kind"] = json!(kind);
    operation["change"]["before"] = json!(before);
    operation["change"]["after"] = json!("not_present");
    operation
}
fn clear_pressure_regions(model: &Value) -> Value {
    intent(
        "Load",
        "case:closed-pressure",
        "pressure_regions",
        Some(&model["load_cases"][0]["pressure_regions"]),
        &json!([]),
    )
}
fn assert_reference_deletion_blocked(model: &Value, deletion: &Value, code: &str) {
    let original = model.clone();
    let claim = hash(model);
    for outcome in [
        validate_operation(model, deletion, Some(&claim)),
        apply_operation(model, deletion, Some(&claim)),
    ] {
        assert!(outcome.applied_model.is_none());
        assert!(outcome.applied_model_backend_hash.is_none());
        assert!(outcome.diff_preview.is_empty());
        assert_eq!(outcome.validation.reference_validation, "blocked");
        assert!(
            outcome.diagnostics.iter().any(|d| d.code == code
                && d.affected_refs
                    .iter()
                    .any(|r| r == "case:closed-pressure.pressure_regions.region:fixture-pressure")),
            "{:?}",
            outcome.diagnostics
        );
    }
    assert_eq!(*model, original);
}
#[test]
fn pressure_region_members_block_direct_and_batch_pipe_delete_until_explicit_removal() {
    let model = fixture();
    let original = model.clone();
    let claim = hash(&model);
    let deletion = geometry_delete_intent(
        "Element",
        "pipe:fixture-span",
        "pipe_segments",
        "delete_pipe_run",
        "fixture-span; node:fixture-root->node:fixture-tip; material=material:fixture-isotropic",
    );
    assert_reference_deletion_blocked(&model, &deletion, "OP-PIPE-DELETE-REFERENCED");
    // Region references must be checked even for a case with no primitive_loads member.
    let mut incomplete = model.clone();
    incomplete["load_cases"][0]
        .as_object_mut()
        .unwrap()
        .remove("primitive_loads");
    assert_reference_deletion_blocked(&incomplete, &deletion, "OP-PIPE-DELETE-REFERENCED");
    let removal = clear_pressure_regions(&model);
    let wrong_order = json!({"batch_id":"batch:delete-before-region-removal", "operations":[deletion.clone(),removal.clone()]});
    for outcome in [
        validate_operation_batch(&model, &wrong_order, Some(&claim)),
        apply_operation_batch(&model, &wrong_order, Some(&claim)),
    ] {
        assert_batch_unpublished(&outcome);
        assert_eq!(
            outcome["simulation_disposition"],
            "rolled_back_no_model_published"
        );
        assert_eq!(outcome["operation_outcomes"].as_array().unwrap().len(), 1);
        assert!(outcome["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["code"] == "OP-PIPE-DELETE-REFERENCED"));
    }
    let right_order = json!({"batch_id":"batch:explicit-region-removal-before-delete", "operations":[removal,deletion]});
    let outcome = apply_operation_batch(&model, &right_order, Some(&claim));
    assert_eq!(
        outcome["validation"]["application_status"], "applied_to_session_model",
        "{outcome:#}"
    );
    assert_eq!(outcome["applied_model"]["pipe_segments"], json!([]));
    assert_eq!(
        outcome["applied_model"]["load_cases"][0]["pressure_regions"],
        json!([])
    );
    assert_eq!(
        outcome["applied_model"]["load_cases"][1],
        model["load_cases"][1]
    );
    assert_eq!(model, original);
}
#[test]
fn pressure_terminal_nodes_block_direct_and_batch_delete_in_incomplete_models() {
    // Imported/in-progress data may already lack member geometry. Its surviving
    // terminal references must still protect nodes independently of pipe/support/load guards.
    let mut model = fixture();
    model["pipe_segments"] = json!([]);
    model["supports"] = json!([]);
    for case in model["load_cases"].as_array_mut().unwrap() {
        case.as_object_mut().unwrap().remove("primitive_loads");
    }
    let original = model.clone();
    let claim = hash(&model);
    let deletion = geometry_delete_intent(
        "Node",
        "node:fixture-tip",
        "nodes",
        "delete_node",
        "fixture-tip; x=1; y=0; z=0",
    );
    assert_reference_deletion_blocked(&model, &deletion, "OP-NODE-DELETE-REFERENCED");
    let removal = clear_pressure_regions(&model);
    let wrong_order = json!({"batch_id":"batch:terminal-delete-before-region-removal","operations":[deletion.clone(),removal.clone()]});
    for outcome in [
        validate_operation_batch(&model, &wrong_order, Some(&claim)),
        apply_operation_batch(&model, &wrong_order, Some(&claim)),
    ] {
        assert_batch_unpublished(&outcome);
        assert_eq!(outcome["operation_outcomes"].as_array().unwrap().len(), 1);
        assert!(outcome["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["code"] == "OP-NODE-DELETE-REFERENCED"));
    }
    let right_order = json!({"batch_id":"batch:explicit-region-removal-before-terminal-delete","operations":[removal,deletion]});
    let outcome = apply_operation_batch(&model, &right_order, Some(&claim));
    assert_eq!(
        outcome["validation"]["application_status"], "applied_to_session_model",
        "{outcome:#}"
    );
    assert_eq!(
        outcome["applied_model"]["nodes"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        outcome["applied_model"]["nodes"][0]["id"],
        "node:fixture-root"
    );
    assert_eq!(
        outcome["applied_model"]["load_cases"][0]["pressure_regions"],
        json!([])
    );
    assert_eq!(model, original);
}

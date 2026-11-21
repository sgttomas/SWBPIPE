use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_operation,
    validate_operation_batch,
};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewSolverMode,
};
use serde_json::{json, Value};
/// The committed exact authoring document (B7b: the current 0.9.0 document
/// with `3.0.0/exact_pressure_v3`, its explicit euler_bernoulli shear model and
/// an explicit corrosion allowance): what authoring produces from its inputs.
fn fixture() -> Value {
    let model: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/model_operations/exact_pressure_authoring_model.json"
    ))
    .unwrap();
    assert_eq!(model["schema_version"], "0.9.0");
    assert_eq!(
        model["pressure_contract"],
        json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"})
    );
    model
}
/// A refusal control: the same document declaring the retired
/// `2.0.0/exact_straight_pressure_v2` contract (never written, never solved).
fn retired_v2_fixture() -> Value {
    let mut model = fixture();
    model["pressure_contract"] = json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"});
    model
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
/// The pressure elongation setting of `expected`, authored through the `set_model_setting`
/// operation on its project, after the pressure profile it needs.
fn elongation_intent(expected: &Value) -> Value {
    let id = expected["project"]["id"].as_str().unwrap();
    let path = "model_settings.pressure_elongation";
    let mut op = intent("Project", id, path, None, &expected["model_settings"]["pressure_elongation"]);
    op["change"]["change_kind"] = json!("set_model_setting");
    op
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
/// `apply` for an operation whose review shows `rows` diff rows: the applier admits it, with no blocking finding in its
/// review or its application. Activating a document that states a selection shows two: the profile, and the
/// selection's removal from the model settings.
fn admitted(model: &Value, intent: &Value, rows: usize) -> Value {
    let claim = hash(model);
    let preview = validate_operation(model, intent, Some(&claim));
    assert!(preview.applied_model.is_none());
    assert_eq!(preview.diff_preview.len(), rows, "{:?}", preview.diagnostics);
    let outcome = apply_operation(model, intent, Some(&claim));
    for diagnostic in preview.diagnostics.iter().chain(&outcome.diagnostics) {
        assert_ne!(diagnostic.severity, "blocking", "{diagnostic:?}");
    }
    outcome.applied_model.unwrap_or_else(|| panic!("{:?}", outcome.diagnostics))
}
fn projection(value: &Value, keys: &[&str]) -> Value {
    Value::Object(
        keys.iter()
            .filter_map(|k| value.get(*k).map(|v| ((*k).into(), v.clone())))
            .collect(),
    )
}
/// Removes the pressure elongation setting, which needs a pressure contract, and an
/// emptied `model_settings`: a pressure-free document carries neither.
fn without_pressure_elongation(model: &mut Value) {
    let object = model.as_object_mut().unwrap();
    let emptied = object.get_mut("model_settings").and_then(Value::as_object_mut).is_some_and(|settings| {
        settings.remove("pressure_elongation");
        settings.is_empty()
    });
    if emptied {
        object.remove("model_settings");
    }
}

fn legacy_authoring_model(expected: &Value) -> Value {
    let mut model = expected.clone();
    model["schema_version"] = json!("0.9.0");
    model.as_object_mut().unwrap().remove("pressure_contract");
    without_pressure_elongation(&mut model);
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
        Some(&json!({"schema_version":"0.9.0"})),
        &projection(&expected, &["schema_version", "pressure_contract"]),
    );
    model = apply(&model, &op);
    model = apply(&model, &elongation_intent(&expected));
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
        assert_eq!(
            serde_json::to_value(&result).unwrap()["producer"]["semantic_contract_id"],
            "openpipestress.result_semantics/0.3.0/pressure-3"
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
    operations.push(elongation_intent(&expected));
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
    assert_eq!(steps.len(), 5);
    assert_eq!(steps[0]["target_object_type"], "Material");
    assert_eq!(steps[1]["target_object_type"], "Load");
    assert_eq!(steps[2]["target_object_type"], "Load");
    assert_eq!(steps[3]["target_object_type"], "Model");
    assert_eq!(steps[4]["target_object_type"], "Project");
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
    // The profile operation, followed only by the setting that needs it.
    let last = batch["operations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|op| op["change"]["field_path"] == "pressure_profile")
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
            .any(|d| d["code"] == "OP-PRESSURE-PROFILE-SUCCESSOR-REQUIRED"));
    }
    assert_eq!(model, original);
    assert_eq!(model["schema_version"], "0.9.0");
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

/// O3: new authoring writes only the complete `3.0.0/exact_pressure_v3`
/// contract. A retired v2 write is refused (as a downgrade and as a re-selection
/// on a retired document), as is a v3 contract without its shear model; a
/// retired v2 contract on a current document is replaced explicitly; a retired
/// model version is never authored (B7b: 0.9.0 is the only current version).
#[test]
fn pressure_profile_writes_only_the_current_contract() {
    let v3 = fixture();
    let v2 = retired_v2_fixture();
    let profile = |m: &Value| projection(m, &["schema_version", "pressure_contract"]);
    let id = v3["project"]["id"].as_str().unwrap().to_string();
    let code = |model: &Value, before: &Value, after: &Value| {
        let outcome = apply_operation(model, &intent("Model", &id, "pressure_profile", Some(before), after), Some(&hash(model)));
        assert!(outcome.applied_model.is_none());
        outcome.diagnostics.iter().map(|d| d.code.clone()).collect::<Vec<_>>()
    };
    // v2 is never written: not from v3, and not as a re-selection on a v2 document.
    assert!(code(&v3, &profile(&v3), &profile(&v2)).contains(&"OP-PRESSURE-PROFILE-SUCCESSOR-REQUIRED".to_string()));
    assert!(code(&v2, &profile(&v2), &profile(&v2)).contains(&"OP-PRESSURE-PROFILE-SUCCESSOR-REQUIRED".to_string()));
    // A v3 contract without its explicit shear model is never written.
    let mut no_shear = profile(&v3);
    no_shear["pressure_contract"].as_object_mut().unwrap().remove("shear_model");
    assert!(code(&v3, &profile(&v3), &no_shear).contains(&"OP-PRESSURE-PROFILE-SUCCESSOR-REQUIRED".to_string()));
    // Retired v2 -> v3 on the current document: the explicit replacement keeps
    // every other byte.
    let upgraded = apply(&v2, &intent("Model", &id, "pressure_profile", Some(&profile(&v2)), &profile(&v3)));
    assert_eq!(canonical_json(&upgraded), canonical_json(&v3));
    // A retired model version is locked: neither authored on (refused by name
    // before any operation is checked: model document 0.8.0, r6) nor written to.
    for retired in ["0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0", "0.8.0"] {
        let mut old = v3.clone();
        old["schema_version"] = json!(retired);
        assert_eq!(code(&old, &profile(&old), &profile(&v3)), vec!["MODEL_SCHEMA_VERSION_RETIRED".to_string()], "{retired}");
        assert!(code(&v3, &profile(&v3), &profile(&old))
            .contains(&"OP-PRESSURE-PROFILE-SCHEMA-VERSION-LOCKED".to_string()), "{retired}");
    }
}

const TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
const EULER_BERNOULLI: &str = "euler_bernoulli";
fn contract(shear_model: Value) -> Value {
    json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear_model})
}
fn with_shear(model: &Value, shear_model: &str) -> Value {
    let mut model = model.clone();
    model["pressure_contract"] = contract(json!(shear_model));
    model
}
fn pressure_free(model: &Value) -> Value {
    let mut model = model.clone();
    model.as_object_mut().unwrap().remove("pressure_contract");
    without_pressure_elongation(&mut model);
    model
}

/// T4-U8 (D-6/O3): the profile gate admits both shear models, and from U8 on the
/// authoring default writes `timoshenko_annulus_energy_v1`; Euler-Bernoulli stays an
/// explicit choice. Activation writes only the profile: every user section keeps
/// its own authored allowance and no allowance is manufactured. An unknown,
/// malformed or absent shear model is still refused by name.
#[test]
fn pressure_profile_admits_both_shear_models_and_refuses_an_unknown_one() {
    let expected = fixture();
    let free = pressure_free(&expected);
    let id = expected["project"]["id"].as_str().unwrap().to_string();
    let free_before = json!({"schema_version": "0.9.0"});
    for shear_model in [TIMOSHENKO, EULER_BERNOULLI] {
        let after = json!({"schema_version": "0.9.0", "pressure_contract": contract(json!(shear_model))});
        let activated = apply(&free, &intent("Model", &id, "pressure_profile", Some(&free_before), &after));
        assert_eq!(activated["pressure_contract"], contract(json!(shear_model)), "{shear_model}");
        // Only the contract is written: the result is the fixture with this selection (and,
        // as the pressure-free document it starts from, without the elongation setting).
        let mut selected = with_shear(&expected, shear_model);
        without_pressure_elongation(&mut selected);
        assert_eq!(canonical_json(&activated), canonical_json(&selected), "{shear_model}");
        assert_eq!(activated["pipe_segments"], free["pipe_segments"], "{shear_model}");
        // Activation never manufactures an allowance, whichever model is selected.
        let mut missing = free.clone();
        missing["pipe_segments"][0]["section"].as_object_mut().unwrap().remove("corrosion_allowance");
        let outcome = apply_operation(&missing, &intent("Model", &id, "pressure_profile", Some(&free_before), &after), Some(&hash(&missing)));
        assert!(outcome.applied_model.is_none(), "{shear_model}");
        assert!(outcome.diagnostics.iter().any(|d| d.code == "EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED"), "{shear_model}: {:?}", outcome.diagnostics);
        assert!(missing["pipe_segments"][0]["section"].get("corrosion_allowance").is_none());
    }
    // Unknown, misspelled, malformed and null values stay refused; so does an extra member.
    for shear_model in [json!("timoshenko"), json!("timoshenko_annulus_energy_v2"), json!("Euler_Bernoulli"),
        json!("cowper"), json!(""), json!(null), json!(5), json!({"model": TIMOSHENKO})] {
        let after = json!({"schema_version": "0.9.0", "pressure_contract": contract(shear_model.clone())});
        let outcome = apply_operation(&free, &intent("Model", &id, "pressure_profile", Some(&free_before), &after), Some(&hash(&free)));
        assert!(outcome.applied_model.is_none(), "{shear_model}");
        let codes: Vec<_> = outcome.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert!(codes.contains(&"OP-PRESSURE-PROFILE-SUCCESSOR-REQUIRED"), "{shear_model}: {codes:?}");
    }
    let mut extra = json!({"schema_version": "0.9.0", "pressure_contract": contract(json!(TIMOSHENKO))});
    extra["pressure_contract"]["shear_factor"] = json!(0.5);
    let outcome = apply_operation(&free, &intent("Model", &id, "pressure_profile", Some(&free_before), &extra), Some(&hash(&free)));
    assert!(outcome.diagnostics.iter().any(|d| d.code == "OP-PRESSURE-PROFILE-SUCCESSOR-REQUIRED"));
    assert!(outcome.applied_model.is_none());
}

/// T4-U8: a document's selection is the user's. Changing it is an explicit edit
/// that rewrites only the contract; re-queuing the profile with the document's own
/// selection keeps it (the applier never substitutes the default).
#[test]
fn pressure_profile_changes_and_preserves_the_shear_selection_explicitly() {
    let base = fixture();
    let id = base["project"]["id"].as_str().unwrap().to_string();
    let profile = |m: &Value| projection(m, &["schema_version", "pressure_contract"]);
    for (from, to) in [(EULER_BERNOULLI, TIMOSHENKO), (TIMOSHENKO, EULER_BERNOULLI)] {
        let model = with_shear(&base, from);
        let target = with_shear(&base, to);
        let changed = apply(&model, &intent("Model", &id, "pressure_profile", Some(&profile(&model)), &profile(&target)));
        assert_eq!(canonical_json(&changed), canonical_json(&target), "{from} -> {to}");
    }
    for shear_model in [TIMOSHENKO, EULER_BERNOULLI] {
        let model = with_shear(&base, shear_model);
        // The identical edit is admitted and writes nothing: the document keeps its
        // own selection and every other byte.
        let requeue = intent("Model", &id, "pressure_profile", Some(&profile(&model)), &profile(&model));
        let applied = apply(&model, &requeue);
        assert_eq!(canonical_json(&applied), canonical_json(&model), "{shear_model}");
        assert_eq!(applied["pressure_contract"]["shear_model"], shear_model);
    }
}

/// A pressure-free document's shear selection (`model_settings.shear_model`) is the
/// user's choice for its straight members. Activating the pressure profile carries it
/// into `pressure_contract.shear_model` and removes it from the model settings in the
/// same operation, shown as its own row, so no document holds both: an explicit
/// Euler-Bernoulli choice is kept, not overwritten. A contract naming the other model is
/// refused and the document is unchanged. Other model settings stay.
#[test]
fn pressure_profile_carries_the_pressure_free_shear_selection_into_the_contract() {
    let expected = fixture();
    let free = pressure_free(&expected);
    assert!(free.get("model_settings").is_none());
    let id = expected["project"]["id"].as_str().unwrap().to_string();
    let free_before = json!({"schema_version": "0.9.0"});
    let tolerance = json!({"value": 5.0, "unit": "mm"});
    for stated in [TIMOSHENKO, EULER_BERNOULLI] {
        for other in [None, Some(&tolerance)] {
            let mut document = free.clone();
            document["model_settings"] = json!({"shear_model": stated});
            if let Some(other) = other {
                document["model_settings"]["node_proximity_tolerance"] = other.clone();
            }
            let after = json!({"schema_version": "0.9.0", "pressure_contract": contract(json!(stated))});
            let op = intent("Model", &id, "pressure_profile", Some(&free_before), &after);
            let claim = hash(&document);
            let preview = validate_operation(&document, &op, Some(&claim));
            let rows: Vec<_> = preview.diff_preview.iter().map(|row| (row.object_type.as_str(), row.field_path.as_str(), row.before.as_str(), row.after.as_str(), row.change_kind.as_str())).collect();
            let quoted = format!("\"{stated}\"");
            assert_eq!(rows[1..], [("Project", "model_settings.shear_model", quoted.as_str(), "not_present", "clear_model_setting")], "{stated}: {:?}", preview.diagnostics);
            assert_eq!(rows[0].1, "pressure_profile");
            let outcome = apply_operation(&document, &op, Some(&claim));
            let activated = outcome.applied_model.unwrap_or_else(|| panic!("{stated}: {:?}", outcome.diagnostics));
            // The result is the exact document with the stated selection; only the stated
            // selection left the model settings.
            let mut selected = with_shear(&expected, stated);
            without_pressure_elongation(&mut selected);
            if let Some(other) = other {
                selected["model_settings"] = json!({"node_proximity_tolerance": other});
            }
            assert_eq!(canonical_json(&activated), canonical_json(&selected), "{stated} {other:?}");
            // The other model is refused, and nothing is written.
            let theirs = if stated == TIMOSHENKO { EULER_BERNOULLI } else { TIMOSHENKO };
            let conflicting = json!({"schema_version": "0.9.0", "pressure_contract": contract(json!(theirs))});
            let outcome = apply_operation(&document, &intent("Model", &id, "pressure_profile", Some(&free_before), &conflicting), Some(&claim));
            assert!(outcome.applied_model.is_none(), "{stated}");
            let codes: Vec<_> = outcome.diagnostics.iter().map(|d| d.code.as_str()).collect();
            assert!(codes.contains(&"OP-PRESSURE-PROFILE-SHEAR-MODEL-CONFLICT"), "{stated}: {codes:?}");
            assert!(outcome.diff_preview.is_empty());
        }
    }
    // A saved document that states none takes the contract's own selection, and the
    // operation writes only the profile.
    for shear_model in [TIMOSHENKO, EULER_BERNOULLI] {
        let after = json!({"schema_version": "0.9.0", "pressure_contract": contract(json!(shear_model))});
        let activated = apply(&free, &intent("Model", &id, "pressure_profile", Some(&free_before), &after));
        assert_eq!(activated["pressure_contract"]["shear_model"], shear_model);
        assert!(activated.get("model_settings").is_none());
    }
    // A document that arrives holding both is still refused by name.
    let mut both = with_shear(&free, TIMOSHENKO);
    both["model_settings"] = json!({"shear_model": TIMOSHENKO});
    let requeue = projection(&both, &["schema_version", "pressure_contract"]);
    let outcome = apply_operation(&both, &intent("Model", &id, "pressure_profile", Some(&requeue), &requeue), Some(&hash(&both)));
    assert!(outcome.applied_model.is_none());
    let codes: Vec<_> = outcome.diagnostics.iter().map(|d| d.code.as_str()).collect();
    assert!(codes.iter().any(|code| code.contains("SHEAR_MODEL_WITH_PRESSURE_CONTRACT") || code.contains("SHEAR-MODEL-WITH-PRESSURE-CONTRACT")), "{codes:?}");
}

/// The carry is shown in a result, not only in the document. A pressure-free document stating a model is solved, and
/// the fixture's document stating the same model is activated and solved on the exact route: the straight members'
/// theory is the same in both results (identity, member record, tip motions to 1e-9), and the other theory's result
/// is measurably different, so the agreement cannot pass with the selection lost or with both solves on one theory.
///
/// Activation and the exact solve use the fixture's own document. The pressure-free route refuses three of its
/// inputs by name, so the pressure-free solve takes its twin, which differs only there, none of it entering a
/// straight member's stiffness or the load:
/// - MATERIAL_INPUT_MISSING (an E/nu material; the route takes E and G): the twin states E and G = E/(2(1 + nu)),
///   the relation the exact route forms from E and nu (`pressure_exact::IsotropicENu::new`), so both routes build
///   with the same E and G.
/// - CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT: the fixture's allowance is 0.0 m. The exact route's stiffness
///   section is the nominal wall whatever the allowance (product_physics `lib.rs`, the D-5 block of the section
///   build: "The built section below is the nominal one: it carries stiffness"); the allowance forms only the reduced
///   section of the stress rows and the pressure bore. The 1e-9 agreement is the evidence.
/// - PREVIEW_CONTRACT_VERSION_MISMATCH (a case's `pressure_regions` member on the pressure-free route): the case's
///   list is empty, so it applies no load; activation needs the explicit list, the pressure-free route its absence.
#[test]
fn activation_keeps_the_stated_theory_in_the_solve() {
    let base = fixture();
    let id = base["project"]["id"].as_str().unwrap().to_string();
    let mut free = pressure_free(&base);
    // The six-component case alone, whose region list is empty: activation requires the explicit list.
    free["load_cases"] = json!([base["load_cases"][1].clone()]);
    assert_eq!(free["load_cases"][0]["pressure_regions"], json!([]));
    assert_eq!(free["pipe_segments"][0]["section"]["corrosion_allowance"], json!({"value": 0.0, "unit": "m"}));
    let solve = |model: &Value| {
        let request: LinearStaticPreviewRequest = serde_json::from_value(json!({"model": model, "materials": []})).unwrap();
        let result = run_linear_static_preview_with_mode(request, PreviewSolverMode::SparseInteractive);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED", "{:?}", result.diagnostics);
        serde_json::to_value(&result).unwrap()
    };
    let identity = |result: &Value| result["producer"]["semantic_contract_id"].as_str().unwrap().rsplit('/').next().unwrap().to_string();
    let tip = |result: &Value| -> Vec<f64> {
        ["global_nodal_displacement_y", "global_nodal_displacement_z", "global_nodal_rotation_y", "global_nodal_rotation_z"].iter().map(|kind| {
            result["results"].as_array().unwrap().iter()
                .find(|row| row["kind"] == *kind && row["entity_ref"] == "node:fixture-tip")
                .and_then(|row| row["value"].as_f64()).unwrap_or_else(|| panic!("{kind}"))
        }).collect()
    };
    // The pressure-free twin: E and G, no allowance, no pressure_regions member (see above).
    let twin = |document: &Value| {
        let mut twin = document.clone();
        for pipe in twin["pipe_segments"].as_array_mut().unwrap() {
            pipe["section"].as_object_mut().unwrap().remove("corrosion_allowance");
        }
        for case in twin["load_cases"].as_array_mut().unwrap() {
            case.as_object_mut().unwrap().remove("pressure_regions");
        }
        let material = &mut twin["materials"][0];
        let (e, nu) = (material["elastic_modulus"]["value"].as_f64().unwrap(), material["poisson_ratio"]["value"].as_f64().unwrap());
        let material = material.as_object_mut().unwrap();
        material.remove("constitutive_basis");
        material.remove("poisson_ratio");
        material.insert("shear_modulus".into(), json!({"value": e / (2.0 * (1.0 + nu)), "unit": "Pa"}));
        twin
    };
    // kappa_E of the fixture's annulus (OD 0.12 m, wall 0.01 m, the nominal stiffness section) with nu = 0.3, in
    // DEL-05-03's operation order.
    let (ro, ri, nu) = (0.06_f64, 0.05_f64, 0.3_f64);
    let (m, n1) = (ri / ro, 1.0 + nu);
    let (m2, a, b) = (m * m, 1.0 + m * m, 1.0 - m * m);
    let c = (8.0 * nu + 14.0) * nu + 7.0;
    let kappa = (6.0 * (n1 * n1)) * (a * a) / (c * (b * b) + 48.0 * (m2 * (n1 * n1)));
    // For each stated model: (pressure-free twin's tip, activated document's tip).
    let mut tips = Vec::new();
    for stated in [TIMOSHENKO, EULER_BERNOULLI] {
        let mut document = free.clone();
        document["model_settings"] = json!({"shear_model": stated});
        let before = solve(&twin(&document));
        assert_eq!(identity(&before), if stated == TIMOSHENKO { "preview-physics-timoshenko-1" } else { "preview-physics-2" }, "{stated}");
        let after = json!({"schema_version": "0.9.0", "pressure_contract": contract(json!(stated))});
        let activated = admitted(&document, &intent("Model", &id, "pressure_profile", Some(&json!({"schema_version": "0.9.0"})), &after), 2);
        assert_eq!(activated["pressure_contract"]["shear_model"], stated);
        assert!(activated.get("model_settings").is_none(), "{:#}", activated["model_settings"]);
        let exact = solve(&activated);
        assert_eq!(identity(&exact), "pressure-3", "{stated}");
        let theory = &exact["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]["beam_theory"];
        assert_eq!(theory["model"], stated, "{theory}");
        if stated == TIMOSHENKO {
            let factor = theory["shear_factor"].as_f64().unwrap();
            assert!((factor - kappa).abs() <= 1e-14 * kappa, "{factor} against {kappa}");
        }
        let (pf, ex) = (tip(&before), tip(&exact));
        for (p, e) in pf.iter().zip(&ex) {
            assert!((p - e).abs() <= 1e-9 * p.abs().max(e.abs()), "{stated}: {pf:?} against {ex:?}");
        }
        tips.push((pf, ex));
    }
    // The cross pair differs by the shear term, far above the 1e-9 agreement (measured: uy about 1.6 %, uz about
    // 3.0 %): the activated Timoshenko tip against the Euler-Bernoulli twin's, and the reverse.
    let ((timoshenko_twin, timoshenko_exact), (euler_twin, euler_exact)) = (&tips[0], &tips[1]);
    for (label, x, y) in [("activated Timoshenko against the Euler-Bernoulli twin", timoshenko_exact, euler_twin), ("activated Euler-Bernoulli against the Timoshenko twin", euler_exact, timoshenko_twin)] {
        for (axis, index, floor) in [("uy", 0, 2e-3), ("uz", 1, 5e-3)] {
            let relative = (x[index] - y[index]).abs() / x[index].abs().max(y[index].abs());
            assert!(relative > floor, "{label}: tip {axis} differs by {relative:e}, not above {floor:e}");
        }
    }
}

const RICH_SOURCE: &str = "apps/desktop/src/features/rich-authoring";
/// An intent as the desktop's builders write it; each step below names its builder.
fn desktop_intent(operation_id: String, operation_kind: &str, source: Option<&str>, target: Value, change: Value, unit_validation: &str, rationale: String) -> Value {
    let mut intent = json!({"operation_id": operation_id, "operation_kind": operation_kind, "operation_status": "proposed", "author_type": "user", "target": target, "change": change,
      "validation": {"schema_validation": "not_run", "constraint_validation": "not_run", "unit_validation": unit_validation, "diff_preview_status": "not_generated", "application_status": "not_applied"},
      "audit_boundary": {"mutation_route": "structured_operations_only", "direct_model_mutation_allowed": false, "requires_user_acceptance": true, "mutates_accepted_model_state": false},
      "professional_boundary": {"human_review_required": true, "software_makes_compliance_claim": false, "software_makes_certification_claim": false, "software_makes_sealing_claim": false, "software_makes_approval_claim": false, "software_makes_authentication_claim": false},
      "rationale": rationale});
    if let Some(source) = source {
        intent["source"] = json!({"source_ref": source, "source_channel": "local_desktop_preview", "source_role": "gui_editor"});
    }
    intent
}
/// `makeRichIntent` (rich-authoring `formSupport.tsx`), which the pressure panel's forms queue through.
fn rich_intent(object: &str, reference: &str, change_kind: &str, path: &str, before: String, after: String, label: &str) -> Value {
    let id = format!("{reference}:{path}");
    desktop_intent(format!("op:rich:{id}"), "modify", Some(RICH_SOURCE), json!({"object_type": object, "ref": reference}),
        json!({"change_id": format!("change:rich:{id}"), "change_kind": change_kind, "field_label": label, "field_path": path, "before": before, "after": after,
          "unit": "none", "dimension": "dimensionless", "source_note": "Explicit user-authored values and units; missing optional fields remain absent."}),
        "not_run", format!("User-authored {}; requires validation and review before application.", label.to_lowercase()))
}

/// The path a document travels. A solvable pressure-free document stating a shear model is solved, then taken to the
/// exact route by the operations a person's edits make, each built as the desktop builds it and applied in turn,
/// in the order the desktop admits them: the explicit E/nu material (the pressure panel's "Create explicit E/nu
/// material"), the members' reassignment to it (Bulk change, material), the corrosion allowance on every section
/// (the property inspector, an explicit zero), the `pressure_regions` list on every case (the pressure panel's
/// regions editor, an empty list: the case has no pressure) and the activation (the pressure panel, its selector
/// left on the stated model). The panel admits the activation only once every section's allowance is applied.
/// The exact solve states the document's theory in its contract and on each member record, and its tip motions are
/// the first solve's to 1e-9; the other theory's path is measurably different.
///
/// `activation_keeps_the_stated_theory_in_the_solve` stays: it isolates the activation operation. This test shows
/// the path, and that between the first of these edits and the activation the document solves on neither route.
///
/// The person enters nu; the panel computes none (`checkedMaterial` takes the entered number). Entered as
/// E/(2G) - 1 from the document's own moduli, the pressure-free route's own nu (`u4b_rows::preview_poisson_ratio`),
/// it is that number exactly, so both routes build with the same E, G and shear factor.
#[test]
fn a_pressure_free_document_reaches_the_exact_route_by_the_desktops_edits_and_keeps_its_theory() {
    const EXACT_MATERIAL: &str = "material:path-exact";
    let base = fixture();
    let id = base["project"]["id"].as_str().unwrap().to_string();
    // The pressure-free document: the six-component case, a material stating E and G, no allowance, no regions.
    let mut free = pressure_free(&base);
    free["load_cases"] = json!([base["load_cases"][1].clone()]);
    free["load_cases"][0].as_object_mut().unwrap().remove("pressure_regions");
    for pipe in free["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"].as_object_mut().unwrap().remove("corrosion_allowance");
    }
    let material = free["materials"][0].as_object_mut().unwrap();
    material.remove("constitutive_basis");
    material.remove("poisson_ratio");
    material.insert("shear_modulus".into(), json!({"value": 7.7e10, "unit": "Pa"}));
    let (e, g) = (free["materials"][0]["elastic_modulus"]["value"].as_f64().unwrap(), free["materials"][0]["shear_modulus"]["value"].as_f64().unwrap());
    let entered = format!("{}", e / (2.0 * g) - 1.0);
    let nu: f64 = entered.parse().unwrap();
    assert_eq!(nu, e / (2.0 * g) - 1.0, "the entered nu is E/(2G) - 1 exactly");
    assert!((e / (2.0 * (1.0 + nu)) - g).abs() <= 1e-14 * g, "the exact route's G = E/(2(1 + nu)) is the document's G");

    let run = |model: &Value| {
        let request: LinearStaticPreviewRequest = serde_json::from_value(json!({"model": model, "materials": []})).unwrap();
        serde_json::to_value(run_linear_static_preview_with_mode(request, PreviewSolverMode::SparseInteractive)).unwrap()
    };
    let blocking = |result: &Value| -> std::collections::BTreeSet<String> {
        result["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking").map(|d| d["code"].as_str().unwrap().to_string()).collect()
    };
    let solved = |result: &Value, label: &str| {
        assert_eq!(result["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {:#}", result["diagnostics"]);
        result["producer"]["semantic_contract_id"].as_str().unwrap().rsplit('/').next().unwrap().to_string()
    };
    let tip = |result: &Value| -> Vec<f64> {
        ["global_nodal_displacement_y", "global_nodal_displacement_z", "global_nodal_rotation_y", "global_nodal_rotation_z"].iter().map(|kind| {
            result["results"].as_array().unwrap().iter()
                .find(|row| row["kind"] == *kind && row["entity_ref"] == "node:fixture-tip")
                .and_then(|row| row["value"].as_f64()).unwrap_or_else(|| panic!("{kind}"))
        }).collect()
    };
    let (m, n1) = (0.05_f64 / 0.06, 1.0 + nu);
    let (m2, a, b) = (m * m, 1.0 + m * m, 1.0 - m * m);
    let c = (8.0 * nu + 14.0) * nu + 7.0;
    let kappa = (6.0 * (n1 * n1)) * (a * a) / (c * (b * b) + 48.0 * (m2 * (n1 * n1)));

    let mut tips = Vec::new();
    for stated in [TIMOSHENKO, EULER_BERNOULLI] {
        let mut document = free.clone();
        document["model_settings"] = json!({"shear_model": stated});
        let first = run(&document);
        assert_eq!(solved(&first, stated), if stated == TIMOSHENKO { "preview-physics-timoshenko-1" } else { "preview-physics-2" });

        // 1. Create explicit E/nu material: the person enters the document's E and the nu above.
        let after = format!(r#"{{"id":"{EXACT_MATERIAL}","label":"Path E/nu","constitutive_basis":"homogeneous_isotropic_E_nu_v1","elastic_modulus":{{"value":{e},"unit":"Pa"}},"poisson_ratio":{{"value":{entered},"unit":"1"}},"provenance":"synthetic_integration_input_not_library_data"}}"#);
        let mut create = rich_intent("Material", EXACT_MATERIAL, "create_material", "materials", "not_present".into(), after, "Exact E/nu material");
        create["operation_kind"] = json!("create");
        create["change"]["dimension"] = json!("stress");
        create["change"]["unit"] = json!("Pa");
        document = admitted(&document, &create, 1);
        let mut states = vec![("the E/nu material", document.clone())];

        // 2. Bulk change, material: one batch, an operation per member (`bulkChange.ts` `setFieldIntent`).
        let operations: Vec<Value> = document["pipe_segments"].as_array().unwrap().iter().map(|pipe| {
            let pipe_id = pipe["id"].as_str().unwrap();
            let token = format!("{pipe_id}-material");
            desktop_intent(format!("op:bulk-{token}"), "modify", Some("apps/desktop/src/features/bulk-change"), json!({"object_type": "Element", "ref": pipe_id}),
                json!({"change_id": format!("change:bulk-{token}"), "change_kind": "set_field", "field_label": "Material", "field_path": "material",
                  "before": pipe["material"], "after": EXACT_MATERIAL, "unit": "none", "dimension": "dimensionless", "source_note": "material reference"}),
                "not_required_dimensionless", format!("Bulk change: material {EXACT_MATERIAL}; requires validation and review before application."))
        }).collect();
        let outcome = apply_operation_batch(&document, &json!({"batch_id": "batch:bulk-material", "operations": operations}), Some(&hash(&document)));
        assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{outcome:#}");
        let findings = outcome["diagnostics"].as_array().unwrap().iter()
            .chain(outcome["operation_outcomes"].as_array().unwrap().iter().flat_map(|step| step["diagnostics"].as_array().unwrap()));
        for finding in findings {
            assert_ne!(finding["severity"], "blocking", "{finding}");
        }
        document = outcome["applied_model"].clone();
        states.push(("the members' reassignment", document.clone()));

        // 3. The property inspector's corrosion allowance on every section: an explicit 0 m (`buildOperationIntent`).
        for pipe_id in document["pipe_segments"].as_array().unwrap().iter().map(|pipe| pipe["id"].as_str().unwrap().to_string()).collect::<Vec<_>>() {
            let token = format!("pipe-{pipe_id}-section.corrosion_allowance.value");
            document = admitted(&document, &desktop_intent(format!("op:editor-intent-{token}"), "modify", None, json!({"object_type": "Element", "ref": pipe_id}),
                json!({"change_id": format!("change:{token}"), "change_kind": "set_field", "field_label": "Corrosion allowance (explicit zero or greater)",
                  "field_path": "section.corrosion_allowance.value", "before": "TBD", "after": r#"{"value":0,"unit":"m"}"#, "unit": "m", "dimension": "length",
                  "source_note": "unit metadata required; entered unit captured explicitly"}),
                "dec018_catalog_dimension_match", format!("preview edit intent for {id}")), 1);
        }
        states.push(("the allowance", document.clone()));

        // 4. The regions editor's list on every case.
        for case_id in document["load_cases"].as_array().unwrap().iter().map(|case| case["id"].as_str().unwrap().to_string()).collect::<Vec<_>>() {
            document = admitted(&document, &rich_intent("Load", &case_id, "update_load", "pressure_regions", "not_present".into(), "[]".into(), "Pressure regions"), 1);
        }
        states.push(("the regions", document.clone()));

        // Each of steps 1 to 4 was admitted with no blocking finding, so the desktop lets a person into this state.
        // Between the first edit and the activation the document solves on neither route: it has no contract, so
        // it goes to the pressure-free route, which refuses each exact input by name (a material without G, then
        // the allowance, then the regions list).
        let mut expected = std::collections::BTreeSet::from(["MATERIAL_INPUT_MISSING".to_string()]);
        for (label, state) in &states {
            if *label == "the allowance" {
                expected.insert("CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT".into());
            }
            if *label == "the regions" {
                expected.insert("PREVIEW_CONTRACT_VERSION_MISMATCH".into());
            }
            let result = run(state);
            assert_ne!(result["status"]["mechanics"], "MECHANICS_SOLVED", "{stated} after {label}");
            let refused = blocking(&result);
            assert!(expected.is_subset(&refused), "{stated} after {label}: {refused:?}");
        }

        // 5. The activation, the selector on the stated model (`PressureAuthoringPanel` `queueProfile`).
        let after = format!(r#"{{"schema_version":"0.9.0","pressure_contract":{{"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"{stated}"}}}}"#);
        document = admitted(&document, &rich_intent("Model", &id, "set_field", "pressure_profile", canonical_json(&json!({"schema_version": "0.9.0"})), after, "Exact pressure profile"), 2);

        assert_eq!(document["pressure_contract"], contract(json!(stated)));
        assert!(document.get("model_settings").is_none(), "{:#}", document["model_settings"]);
        let created = document["materials"].as_array().unwrap().iter().find(|m| m["id"] == EXACT_MATERIAL).unwrap();
        assert_eq!(created["poisson_ratio"]["value"].as_f64(), Some(nu));
        for pipe in document["pipe_segments"].as_array().unwrap() {
            assert_eq!(pipe["material"], EXACT_MATERIAL);
            assert_eq!((pipe["section"]["corrosion_allowance"]["value"].as_f64(), &pipe["section"]["corrosion_allowance"]["unit"]), (Some(0.0), &json!("m")));
        }
        assert!(document["load_cases"].as_array().unwrap().iter().all(|case| case["pressure_regions"] == json!([])));

        let exact = run(&document);
        assert_eq!(solved(&exact, stated), "pressure-3");
        let sections: Vec<&Value> = exact["contract_evidence"]["exact_cases"].as_array().unwrap().iter()
            .flat_map(|case| case["pipe_sections"].as_array().unwrap()).collect();
        assert!(!sections.is_empty());
        for section in sections {
            let theory = &section["beam_theory"];
            assert_eq!(theory["model"], stated, "{theory}");
            if stated == TIMOSHENKO {
                let factor = theory["shear_factor"].as_f64().unwrap();
                assert!((factor - kappa).abs() <= 1e-14 * kappa, "{factor} against {kappa}");
            }
        }
        let (pf, ex) = (tip(&first), tip(&exact));
        for (p, x) in pf.iter().zip(&ex) {
            assert!((p - x).abs() <= 1e-9 * p.abs().max(x.abs()), "{stated}: {pf:?} against {ex:?}");
        }
        tips.push((pf, ex));
    }
    let ((timoshenko_first, timoshenko_exact), (euler_first, euler_exact)) = (&tips[0], &tips[1]);
    for (label, x, y) in [("Timoshenko's exact solve against Euler-Bernoulli's first", timoshenko_exact, euler_first), ("Euler-Bernoulli's exact solve against Timoshenko's first", euler_exact, timoshenko_first)] {
        for (axis, index, floor) in [("uy", 0, 2e-3), ("uz", 1, 5e-3)] {
            let relative = (x[index] - y[index]).abs() / x[index].abs().max(y[index].abs());
            assert!(relative > floor, "{label}: tip {axis} differs by {relative:e}, not above {floor:e}");
        }
    }
}

/// The embedded agent's batch goes through the same applier: a batch that sets the
/// pressure-free selection and then activates pressure with it leaves only the contract's
/// selection; one that activates with the other model is refused whole, so a stated
/// Euler-Bernoulli choice is never replaced by the authored default.
#[test]
fn a_batch_that_activates_pressure_carries_the_selection_it_set() {
    let expected = fixture();
    let free = pressure_free(&expected);
    let id = expected["project"]["id"].as_str().unwrap().to_string();
    let free_before = json!({"schema_version": "0.9.0"});
    let claim = hash(&free);
    let batch = |profile: &str| {
        let mut setting = intent("Project", &id, "model_settings.shear_model", None, &json!(EULER_BERNOULLI));
        setting["change"]["change_kind"] = json!("set_model_setting");
        setting["change"]["after"] = json!(EULER_BERNOULLI);
        let profile = intent("Model", &id, "pressure_profile", Some(&free_before),
            &json!({"schema_version": "0.9.0", "pressure_contract": contract(json!(profile))}));
        json!({"batch_id": "batch:carry-shear-selection", "source_model_hash": claim["value"], "operations": [setting, profile]})
    };
    let applied = apply_operation_batch(&free, &batch(EULER_BERNOULLI), Some(&claim));
    assert_eq!(applied["validation"]["application_status"], "applied_to_session_model", "{applied:#}");
    assert_eq!(applied["applied_model"]["pressure_contract"], contract(json!(EULER_BERNOULLI)));
    assert!(applied["applied_model"].get("model_settings").is_none(), "{applied:#}");
    let refused = apply_operation_batch(&free, &batch(TIMOSHENKO), Some(&claim));
    assert_batch_unpublished(&refused);
    let steps = refused["operation_outcomes"].as_array().unwrap();
    assert!(steps[1]["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "OP-PRESSURE-PROFILE-SHEAR-MODEL-CONFLICT"), "{refused:#}");
}

/// O4: signed pressure is admitted under v3 only (the retired v2 contract is a
/// refusal control).
#[test]
fn signed_region_pressure_follows_the_document_contract() {
    for (model, admitted) in [(fixture(), true), (retired_v2_fixture(), false)] {
        let case = &model["load_cases"][0];
        let mut regions = case["pressure_regions"].clone();
        regions[0]["pressure"]["value"] = json!(-1.0e5);
        let op = intent("Load", case["id"].as_str().unwrap(), "pressure_regions", Some(&case["pressure_regions"]), &regions);
        let outcome = apply_operation(&model, &op, Some(&hash(&model)));
        assert_eq!(outcome.applied_model.is_some(), admitted, "{:?}", outcome.diagnostics);
    }
}

/// an existing E/nu material's
/// thermal expansion coefficient is added, changed and removed with its
/// constitutive inputs; the project temperature unit and the coefficient's unit
/// are checked as on `create_material`, and nothing else on the material moves.
#[test]
fn constitutive_properties_add_change_and_remove_the_thermal_expansion_coefficient() {
    const KEYS: [&str; 6] = [
        "constitutive_basis",
        "elastic_modulus",
        "shear_modulus",
        "poisson_ratio",
        "thermal_expansion_coefficient",
        "provenance",
    ];
    let mut model = fixture();
    assert!(model["materials"][0].get("thermal_expansion_coefficient").is_none());
    let id = model["materials"][0]["id"].as_str().unwrap().to_string();
    let untouched = |m: &Value| {
        let mut m = m.clone();
        m.as_object_mut().unwrap().remove("thermal_expansion_coefficient");
        m
    };
    let original = model["materials"][0].clone();
    let edit = |model: &Value, alpha: Option<Value>| {
        let before = projection(&model["materials"][0], &KEYS);
        let mut after = before.clone();
        match alpha {
            Some(alpha) => after["thermal_expansion_coefficient"] = alpha,
            None => {
                after.as_object_mut().unwrap().remove("thermal_expansion_coefficient");
            }
        }
        intent("Material", &id, "constitutive_properties", Some(&before), &after)
    };
    let refused = |model: &Value, op: &Value| {
        let outcome = apply_operation(model, op, Some(&hash(model)));
        assert!(outcome.applied_model.is_none(), "{:?}", outcome.diagnostics);
        outcome
            .diagnostics
            .iter()
            .map(|d| d.code.clone())
            .collect::<Vec<_>>()
    };

    model = apply(&model, &edit(&model, Some(json!({"value": 1.2e-5, "unit": "1/K"}))));
    assert_eq!(model["materials"][0]["thermal_expansion_coefficient"], json!({"value": 1.2e-5, "unit": "1/K"}));
    // Compared as persisted (RFC 8785): an integer modulus re-serializes as the same number.
    assert_eq!(canonical_json(&untouched(&model["materials"][0])), canonical_json(&original));
    model = apply(&model, &edit(&model, Some(json!({"value": -6.5e-6, "unit": "1/degF"}))));
    assert_eq!(model["materials"][0]["thermal_expansion_coefficient"], json!({"value": -6.5e-6, "unit": "1/degF"}));

    // A non-thermal-expansion unit, a non-finite or missing value, and an extra
    // member are refused; nothing is converted or completed.
    for alpha in [
        json!({"value": 1.2e-5, "unit": "Pa"}),
        json!({"value": 1.2e-5}),
        json!({"value": "1.2e-5", "unit": "1/K"}),
        json!({"value": 1.2e-5, "unit": "1/K", "basis": "secant"}),
    ] {
        assert!(!refused(&model, &edit(&model, Some(alpha.clone()))).is_empty(), "{alpha}");
    }
    // The project temperature unit must be stated and be a temperature unit.
    let mut no_unit = model.clone();
    no_unit["project"]["units"].as_object_mut().unwrap().remove("temperature");
    assert!(refused(&no_unit, &edit(&no_unit, Some(json!({"value": 1.2e-5, "unit": "1/K"}))))
        .contains(&"OP-UNIT-METADATA-MISSING".to_string()));
    let mut bad_unit = model.clone();
    bad_unit["project"]["units"]["temperature"] = json!("Pa");
    assert!(refused(&bad_unit, &edit(&bad_unit, Some(json!({"value": 1.2e-5, "unit": "1/K"}))))
        .contains(&"OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE".to_string()));

    // A stale projection that omits the coefficient is refused, never read as a removal.
    let mut stale_before = projection(&model["materials"][0], &KEYS);
    stale_before.as_object_mut().unwrap().remove("thermal_expansion_coefficient");
    let stale = intent("Material", &id, "constitutive_properties", Some(&stale_before), &stale_before);
    assert!(refused(&model, &stale).contains(&"OP-BEFORE-VALUE-MISMATCH".to_string()));

    // Removal is explicit, and returns the material to its original record.
    model = apply(&model, &edit(&model, None));
    assert_eq!(canonical_json(&model["materials"][0]), canonical_json(&original));
}

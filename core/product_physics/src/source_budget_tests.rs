use super::*;

fn request() -> serde_json::Value {
    let mut model: serde_json::Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/numerical_sensitive_torsion_model.json")).unwrap();
    let mut second = model["load_cases"][0].clone();
    second["id"] = "case:signed-companion".into();
    second["primitive_loads"][0]["id"] = "torque:signed-companion".into();
    second["primitive_loads"][0]["magnitude"]["value"] = (-2e-8).into();
    // The existing summary is normal stress, not an equivalent torsional stress.
    // Add an independent axial action so the second case truly governs it.
    second["primitive_loads"].as_array_mut().unwrap().push(serde_json::json!({
        "id":"force:axial-companion","category":"concentrated_force","dimension":"force",
        "direction":"global_x","magnitude":{"value":1000,"unit":"N"},
        "target":{"type":"node","node":"tip"},
        "provenance":"invented_source_recovery_multicase_control_no_external_project_data"
    }));
    model["load_cases"].as_array_mut().unwrap().push(second);
    serde_json::json!({"model":model,"materials":[]})
}

fn run(value: serde_json::Value, mode: PreviewSolverMode, per_case_limit: usize, invocation_limit: usize) -> (MechanicsEnvelope, SourceRecoveryBudget) {
    let (request, capture) = source_receipt::CapturedInvocation::parse(value, mode).unwrap();
    let mut budget = SourceRecoveryBudget { per_case_limit, invocation_limit, charged: 0, failed_charged: 0, publication_charged: 0, rejected: 0, attempts: 0 };
    let result = run_linear_static_preview_captured(request, mode, Some(&capture), &mut budget);
    (result, budget)
}

#[test]
fn real_multicase_run_debits_one_invocation_and_keeps_actual_maximum() {
    for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
        let input = request();
        let (result, budget) = run(input.clone(), mode, SOURCE_BLOCKS_WORK_LIMIT, SOURCE_BLOCKS_INVOCATION_WORK_LIMIT);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED", "{:?}", result.diagnostics);
        let receipt = result.source_block_recovery.as_ref().expect("actual receipt");
        assert_eq!(receipt["body"]["status"], "qualified");
        let cases = receipt["body"]["cases"].as_array().unwrap();
        assert_eq!(cases.len(), 2);
        assert_eq!(budget.attempts, 2);
        assert_eq!(budget.failed_charged, 0);
        assert_eq!(cases.iter().map(|c| c["work"]["charged"].as_u64().unwrap() as usize).sum::<usize>() + budget.publication_charged, budget.charged);
        assert!(budget.charged <= SOURCE_BLOCKS_INVOCATION_WORK_LIMIT);
        assert_eq!(receipt["body"]["invocation_work"]["charged"], budget.charged);
        assert_eq!(receipt["body"]["invocation_work"]["publication_charged"], budget.publication_charged);
        assert!(cases.iter().all(|c| c["work"]["limit"] == SOURCE_BLOCKS_WORK_LIMIT));
        let maximum = result.summary.max_open_formula_stress.as_ref().unwrap();
        let row = result.results.iter().find(|row| row.id == maximum.result_ref).unwrap();
        assert_eq!(row.basis_ref.as_ref().unwrap().ref_id, "case:signed-companion");
        assert_eq!(row.value.to_bits(), maximum.value.to_bits());
        if let Ok(folder) = std::env::var("OPS_SOURCE_BLOCK_FIXTURE_DIR") {
            let folder = std::path::Path::new(&folder);
            std::fs::create_dir_all(folder).unwrap();
            let name = format!("multicase-{}", mode.as_str());
            std::fs::write(folder.join(format!("{name}.request.json")), serde_json::to_vec_pretty(&input).unwrap()).unwrap();
            std::fs::write(folder.join(format!("{name}.raw.json")), serde_json::to_vec_pretty(&result).unwrap()).unwrap();
        }
        eprintln!("MULTICASE_SOURCE_WORK {}", serde_json::json!({"mode":mode.as_str(),"cases":2,"charged":budget.charged,"failed_charged":budget.failed_charged,"per_case_limit":budget.per_case_limit,"invocation_limit":budget.invocation_limit}));
    }
}

#[test]
fn smaller_case_and_invocation_caps_deny_without_refunding_work() {
    let (case_denied, first) = run(request(), PreviewSolverMode::DenseScrutiny, 100_000, SOURCE_BLOCKS_INVOCATION_WORK_LIMIT);
    assert_eq!(case_denied.status.mechanics, "MECHANICS_SOLVED");
    assert!(case_denied.source_block_recovery.is_none());
    assert!(first.failed_charged > 0 && first.failed_charged == first.charged);
    assert_eq!(first.attempts, 2);
    assert!(case_denied.diagnostics.iter().filter(|d| d.code == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE").count() == 2);
    let (run_denied, second) = run(request(), PreviewSolverMode::DenseScrutiny, SOURCE_BLOCKS_WORK_LIMIT, 5_500_000);
    assert!(second.attempts == 2 && second.failed_charged > 0 && second.charged > second.failed_charged);
    assert!(second.charged <= 5_500_000);
    assert!(run_denied.source_block_recovery.as_ref().is_none_or(|r| r["body"]["status"] != "qualified"));
    assert!(run_denied.diagnostics.iter().any(|d| d.code == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE" || d.code == "SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED"));
}

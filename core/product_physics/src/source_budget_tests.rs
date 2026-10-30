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
    let mut budget = SourceRecoveryBudget { per_case_limit, invocation_limit, charged: 0, failed_charged: 0, publication_charged: 0, rejected: 0, attempts: 0, ..Default::default() };
    let result = run_linear_static_preview_captured(request, mode, Some(&capture), &mut budget);
    (result, budget)
}

/// B4/D1 (b): the pressure-free preview route selects no source blocks. A captured
/// multicase preview invocation at the full budget attempts no retained-source recovery,
/// charges nothing, publishes no receipt and discloses no source-recovery outcome; its
/// cases are W1's. (The exact route's composite budgets are tested with physics-source-1.)
#[test]
fn preview_route_attempts_no_source_recovery() {
    for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
        let (result, budget) = run(request(), mode, PHYSICS_SOURCE_WORK_LIMIT, SOURCE_BLOCKS_INVOCATION_WORK_LIMIT);
        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED", "{:?}", result.diagnostics);
        assert_eq!(result.producer.semantic_contract_id, preview_physics::ID, "{mode:?}");
        assert!(result.source_block_recovery.is_none(), "{mode:?}");
        assert!(!result.diagnostics.iter().any(|d| d.code.starts_with("SOURCE_BLOCK_RECOVERY")), "{mode:?}");
        assert_eq!((budget.attempts, budget.charged, budget.failed_charged, budget.publication_charged), (0, 0, 0, 0), "{mode:?}");
    }
}

use super::*;
use open_pipe_stress_primitive_loads::LoadApplication;
fn raw() -> Value {
    json!({"model":serde_json::from_str::<Value>(include_str!("../../../../fixtures/product_preview/numerical_sensitive_torsion_model.json")).unwrap()})
}
struct Fixture {
    model: PreviewModel,
    built: BuiltModel,
    k: Vec<Vec<f64>>,
    f: Vec<f64>,
    free: Vec<usize>,
    prescribed: Vec<(usize, f64)>,
    springs: Vec<SpringEntry>,
    loads: LoadApplication,
}
impl Fixture {
    fn new(mut model: PreviewModel) -> Self {
        let mut diagnostics = vec![];
        let mut materials = model.materials.clone();
        resolve_shared_sections(&mut model, &mut diagnostics);
        normalize_model_units(&mut model, &mut materials, &mut diagnostics);
        let built = build_model(&model, &materials, &mut diagnostics).unwrap();
        let boundary = prepare_boundary(built.nodes.len(), &built.supports);
        let mut k = assemble_global_stiffness_with_user_elements(
            built.nodes.len(),
            &built.frame_elements,
            &[],
        )
        .unwrap();
        for s in &boundary.springs {
            let d = s.node_dof.global_index();
            k[d][d] += s.stiffness.value;
        }
        let primitive =
            build_load_case_primitive_loads(&model, &model.load_cases[0], &mut diagnostics);
        let loads = prepare_loads(built.nodes.len(), built.pipes.len(), &primitive);
        let f = loads.global_load_vector(built.nodes.len());
        let prescribed = boundary.restrained_dofs.iter().map(|d| (*d, 0.0)).collect();
        let free = (0..f.len())
            .filter(|d| !boundary.restrained_dofs.contains(d))
            .collect();
        Self {
            model,
            built,
            k,
            f,
            free,
            prescribed,
            springs: boundary.springs,
            loads,
        }
    }
    fn input(&self) -> source_recovery::Input<'_> {
        source_recovery::Input {
            model: &self.model,
            built: &self.built,
            stiffness: &self.k,
            force: &self.f,
            free: &self.free,
            prescribed: &self.prescribed,
            spring_entries: &self.springs,
            load_case: &self.model.load_cases[0],
            load_application: &self.loads,
            thermal_loads: &[],
            pressure_thrust_loads: &[],
        }
    }
}
#[test]
fn captured_invocation_keeps_unknown_absence_material_and_actual_mode() {
    let original = raw();
    let (_, a) =
        CapturedInvocation::parse(original.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
    let mut changed = original.clone();
    changed["unknown_preparse_marker"] = json!({"b":2,"a":1});
    let (_, b) =
        CapturedInvocation::parse(changed.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
    assert_ne!(a.digest, b.digest);
    assert_eq!(b.raw, changed);
    let mut explicit = original.clone();
    explicit["materials"] = json!([]);
    let (_, c) = CapturedInvocation::parse(explicit, PreviewSolverMode::DenseScrutiny).unwrap();
    assert_ne!(a.digest, c.digest);
    let (_, d) = CapturedInvocation::parse(original, PreviewSolverMode::SparseInteractive).unwrap();
    assert_ne!(a.digest, d.digest);
    let mut unsafe_integer = raw();
    unsafe_integer["unknown_preparse_marker"] = json!(9_007_199_254_740_992_u64);
    assert!(CapturedInvocation::parse(unsafe_integer, PreviewSolverMode::DenseScrutiny).is_err());
}
#[test]
fn selected_projection_mapping_rejects_value_unit_entity_missing_duplicate() {
    let (request, _) = CapturedInvocation::parse(raw(), PreviewSolverMode::DenseScrutiny).unwrap();
    let f = Fixture::new(request.model);
    let selected = source_recovery::solve(
        f.input(),
        exact::Limits {
            operations: SOURCE_BLOCKS_WORK_LIMIT,
            ..Default::default()
        },
    )
    .unwrap();
    let expected = rows::expected_primary(&f.input(), &selected).unwrap();
    let bindings: Vec<_> = expected
        .iter()
        .map(|(i, r)| FunctionalRowBinding {
            functional_index: *i,
            result_id: r.id.clone(),
        })
        .collect();
    let mut actual: Vec<_> = expected.values().cloned().collect();
    actual.extend(
        rows::derived(&f.input(), &selected, &expected)
            .unwrap()
            .into_iter()
            .map(|d| d.row),
    );
    assert!(rows::bind(&f.input(), &selected, &actual, &bindings).is_ok());
    for mutation in 0..4 {
        let mut bad_rows = actual.clone();
        match mutation {
            0 => bad_rows[0].value = f64::from_bits(bad_rows[0].value.to_bits() ^ 1),
            1 => bad_rows[0].unit = "invalid".into(),
            2 => bad_rows[0].entity_ref = "other entity".into(),
            _ => {
                bad_rows.remove(0);
            }
        }
        assert!(rows::bind(&f.input(), &selected, &bad_rows, &bindings).is_err());
    }
    let mut duplicate = bindings.clone();
    duplicate[1].functional_index = duplicate[0].functional_index;
    assert!(rows::bind(&f.input(), &selected, &actual, &duplicate).is_err());
}
#[test]
fn actual_capture_replay_rejects_changed_material_or_load_source() {
    let original = raw();
    let (request, capture) =
        CapturedInvocation::parse(original.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
    let f = Fixture::new(request.model);
    let mut selected = source_recovery::solve(
        f.input(),
        exact::Limits {
            operations: SOURCE_BLOCKS_WORK_LIMIT,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(capture.check_input(&f.input(), &mut selected).is_ok());
    let mut changed = original;
    changed["model"]["materials"][0]["shear_modulus"]["value"] = json!(81e9);
    let (_, other) = CapturedInvocation::parse(changed, PreviewSolverMode::DenseScrutiny).unwrap();
    let mut selected = source_recovery::solve(
        f.input(),
        exact::Limits {
            operations: SOURCE_BLOCKS_WORK_LIMIT,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(other.check_input(&f.input(), &mut selected).is_err());
}
#[test]
fn checked_norm_does_not_hide_overflow_or_underflow() {
    assert_eq!(rows::finite_norm([1e308, 0., 0.]).unwrap(), 1e308);
    assert!(rows::finite_norm([1.7e308, 1.7e308, 0.]).is_err());
    assert!(rows::finite_norm([f64::from_bits(1), 0., 0.]).is_err());
    assert!((1e-160_f64 * 1e-160).is_subnormal());
    assert_eq!(rows::finite_norm([1e-160, 0., 0.]).unwrap(), 1e-160);
    assert_eq!(rows::finite_norm([1e308, 1e-300, 0.]).unwrap(), 1e308);
    assert_eq!(rows::finite_norm([3., 4., 0.]).unwrap(), 5.);
}
#[test]
fn receipt_domain_hashes_are_acyclic_and_distinct() {
    let value = json!({"v":-0.0,"bits":bits(-0.0)});
    assert_ne!(
        hash("source_blocks_publication_v1", &value).unwrap(),
        hash("source_blocks_receipt_v1", &value).unwrap()
    );
    assert_ne!(
        hash(
            "source_blocks_invocation_v1",
            &json!({"request":value,"solver_mode":"dense_scrutiny"})
        )
        .unwrap(),
        hash(
            "source_blocks_invocation_v1",
            &json!({"request":value,"solver_mode":"sparse_interactive"})
        )
        .unwrap()
    );
}

// Append to source_receipt/tests.rs after parent's bounded public run.
fn exact_raw() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/physics_source/n05.request.json"
    ))
    .unwrap()
}
fn exact_fixture(raw: &Value) -> Fixture {
    let request: LinearStaticPreviewRequest = serde_json::from_value(raw.clone()).unwrap();
    let mut model = request.model;
    let mut materials = if request.materials.is_empty() {
        model.materials.clone()
    } else {
        request.materials
    };
    let mut diagnostics = vec![];
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    if modulus_basis_key(&model.load_cases[0], &mut diagnostics).is_some() {
        materials = pressure_material::resolve_case(
            &model,
            &materials,
            &model.load_cases[0],
            &mut diagnostics,
        )
        .unwrap()
        .0;
    }
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    model.materials = materials;
    Fixture::new(model)
}
#[test]
fn composite_captured_exact_empty_regions_replay_and_overrides() {
    let mut raw = exact_raw();
    raw["unknown_actual_capture_marker"] = json!({"kept":true});
    raw["materials"] = raw["model"]["materials"].clone();
    // Material override is the actual source; a disagreeing model copy cannot win.
    raw["model"]["materials"][0]["elastic_modulus"]["value"] = json!(123.0);
    let f = exact_fixture(&raw);
    let (_, capture) =
        CapturedInvocation::parse(raw.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
    let mut selected = source_recovery::solve(
        f.input(),
        exact::Limits {
            operations: SOURCE_BLOCKS_WORK_LIMIT,
            ..Default::default()
        },
    )
    .unwrap();
    capture.check_input(&f.input(), &mut selected).unwrap();
    let member = selected.members()[0].member_id.clone();
    let maximum = composite_member_maximum(&f.input(), &mut selected, &member).unwrap();
    let evidence = maximum.evidence("test:maximum");
    assert_eq!(evidence["basis"], "retained_source_endpoint_normal_max_v1");
    assert_eq!(evidence["load_case_id"], f.model.load_cases[0].id);
    assert_eq!(
        evidence["endpoints"][0]["functional_indices"],
        json!([0usize, 4, 5].map(|c| selected.members()[0].section_functional_indices[0][c]))
    );
    let support = selected.support_actions()[0].support_id.clone();
    assert!(composite_support_norms(&f.input(), &mut selected, &support)
        .unwrap()
        .iter()
        .all(|x| x.is_finite()));
    let mut wrong = raw;
    wrong["materials"][0]["poisson_ratio"]["value"] = json!(0.123);
    let (_, capture) = CapturedInvocation::parse(wrong, PreviewSolverMode::DenseScrutiny).unwrap();
    let mut selected = source_recovery::solve(
        f.input(),
        exact::Limits {
            operations: SOURCE_BLOCKS_WORK_LIMIT,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(capture.check_input(&f.input(), &mut selected).is_err());
}
#[test]
fn composite_adapter_refuses_missing_zero_and_nonzero_regions() {
    for pressure in [None, Some(0.0), Some(123.0)] {
        let mut f = exact_fixture(&exact_raw());
        f.model.load_cases[0].pressure_regions = pressure.map(|value| {
            vec![pressure_runtime::PressureRegionInput {
                pressure: Some(Quantity {
                    value,
                    unit: "Pa".into(),
                }),
                ..Default::default()
            }]
        });
        let error = source_recovery::solve(
            f.input(),
            exact::Limits {
                operations: SOURCE_BLOCKS_WORK_LIMIT,
                ..Default::default()
            },
        )
        .unwrap_err();
        assert!(matches!(
            error.error,
            source_recovery::RecoveryError::Unsupported(_)
        ));
        assert!(error.work.charged > 0);
    }
}
#[test]
fn composite_recipe_reserves_before_failures_and_geometry_changes() {
    let mut f = exact_fixture(&exact_raw());
    let mut selected = source_recovery::solve(
        f.input(),
        exact::Limits {
            operations: SOURCE_BLOCKS_WORK_LIMIT,
            ..Default::default()
        },
    )
    .unwrap();
    let before = selected.summary().work.charged;
    assert!(composite_member_maximum(&f.input(), &mut selected, "missing").is_err());
    assert!(selected.summary().work.charged > before);
    let member = selected.members()[0].member_id.clone();
    f.built.sections.get_mut(&member).unwrap().area *= 2.0;
    assert!(composite_member_maximum(&f.input(), &mut selected, &member).is_err());
    let remaining = selected.summary().work.limit - selected.summary().work.charged;
    selected.charge_finalization(remaining).unwrap();
    let before = selected.summary().work.charged;
    assert!(composite_support_norms(&f.input(), &mut selected, "missing").is_err());
    assert_eq!(selected.summary().work.charged, before);
    assert!(selected.summary().work.rejected > 0);
}

// Measurement only: explicit command; never a production budget override.
#[test]
#[ignore = "bounded resource measurement with private 16M test budget; not public qualification"]
fn composite_fields_work_measurement() {
    let raw: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/physics_source/fields.request.json"
    ))
    .unwrap();
    for mode in [
        PreviewSolverMode::DenseScrutiny,
        PreviewSolverMode::SparseInteractive,
    ] {
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let mut budget = SourceRecoveryBudget {
            per_case_limit: 16_000_000,
            ..Default::default()
        };
        composite::start_trace();
        let envelope =
            run_linear_static_preview_captured(request, mode, Some(&capture), &mut budget);
        let trace = composite::take_trace();
        let wire = serialized(&envelope).unwrap();
        eprintln!(
            "COMPOSITE_WORK_MEASUREMENT {}",
            json!({"purpose":"private resource measurement; not public policy or qualification","mode":mode.as_str(),"private_per_case_limit":16_000_000,"public_per_case_limit":SOURCE_BLOCKS_WORK_LIMIT,"stage_trace":trace,"invocation_charged":budget.charged,"publication_charged":budget.publication_charged,"case_work":wire["source_block_recovery"]["body"]["cases"][0]["work"],"receipt_status":wire["source_block_recovery"]["body"]["status"],"diagnostics":wire["diagnostics"].as_array().unwrap().iter().filter(|d|d["severity"]=="blocking").collect::<Vec<_>>()})
        );
        assert!(
            envelope.source_block_recovery.is_none(),
            "private oversized limit must not create a selected-policy receipt"
        );
        assert!(
            trace.last().unwrap()["charged"].as_u64().unwrap() > SOURCE_BLOCKS_WORK_LIMIT as u64
        );
        assert!(envelope.diagnostics.iter().any(|d| d
            .message
            .contains("case budget exceeds selected receipt policy")));
    }
}

#[test]
fn composite_resource_policy_cannot_promote_old_method_or_refund_failure() {
    fn run(
        raw: Value,
        case_limit: usize,
        invocation_limit: usize,
    ) -> (MechanicsEnvelope, SourceRecoveryBudget) {
        let (request, capture) =
            CapturedInvocation::parse(raw, PreviewSolverMode::DenseScrutiny).unwrap();
        let mut budget = SourceRecoveryBudget {
            per_case_limit: case_limit,
            invocation_limit,
            ..Default::default()
        };
        let envelope = run_linear_static_preview_captured(
            request,
            PreviewSolverMode::DenseScrutiny,
            Some(&capture),
            &mut budget,
        );
        (envelope, budget)
    }
    assert_eq!(SOURCE_BLOCKS_WORK_LIMIT, 4_000_000);
    assert_eq!(PHYSICS_SOURCE_WORK_LIMIT, 8_000_000);
    for (value, cap) in [
        (raw(), PHYSICS_SOURCE_WORK_LIMIT),
        (exact_raw(), PHYSICS_SOURCE_WORK_LIMIT + 1),
    ] {
        let (envelope, budget) = run(value, cap, SOURCE_BLOCKS_INVOCATION_WORK_LIMIT);
        assert!(envelope.source_block_recovery.is_none());
        assert!(envelope.diagnostics.iter().any(|d| d
            .message
            .contains("case budget exceeds selected receipt policy")));
        assert!(budget.charged > budget.publication_charged && budget.publication_charged > 0);
    }
    let (denied, budget) = run(exact_raw(), 100_000, SOURCE_BLOCKS_INVOCATION_WORK_LIMIT);
    assert!(denied.source_block_recovery.is_none());
    assert!(budget.charged > 0 && budget.failed_charged == budget.charged);
    let (positive, budget) = run(
        exact_raw(),
        PHYSICS_SOURCE_WORK_LIMIT,
        SOURCE_BLOCKS_INVOCATION_WORK_LIMIT,
    );
    let receipt = positive.source_block_recovery.as_ref().unwrap();
    let case_work = receipt["body"]["cases"][0]["work"]["charged"]
        .as_u64()
        .unwrap() as usize;
    assert_eq!(case_work + budget.publication_charged, budget.charged);
    let (denied, limited) = run(exact_raw(), PHYSICS_SOURCE_WORK_LIMIT, case_work + 1);
    assert!(denied.source_block_recovery.is_none());
    assert_eq!(limited.charged, case_work);
    assert!(limited.rejected > 0 && limited.publication_charged == 0);
}

#[test]
fn legacy_model_three_retains_ordinary_route_without_old_source_namespace() {
    let mut value = exact_raw();
    value["model"]["pressure_contract"] = json!({"version":"1.0.0","mode":"legacy_pressure_v1"});
    value["model"]["load_cases"][0]
        .as_object_mut()
        .unwrap()
        .remove("pressure_regions");
    for mode in [
        PreviewSolverMode::DenseScrutiny,
        PreviewSolverMode::SparseInteractive,
    ] {
        let envelope = run_linear_static_preview_value_with_mode(value.clone(), mode).unwrap();
        assert!(envelope.source_block_recovery.is_none());
        assert!(!envelope
            .diagnostics
            .iter()
            .any(|d| d.code == "SOURCE_BLOCK_RECOVERY_SELECTED"));
        assert!(
            envelope
                .diagnostics
                .iter()
                .any(|d| d.code == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"
                    && d.message.contains("legacy source-blocks namespace")),
            "{:?}",
            envelope.diagnostics
        );
        assert!(!envelope
            .diagnostics
            .iter()
            .any(|d| d.code == "PREVIEW_CONTRACT_VERSION_MISMATCH"));
    }
}

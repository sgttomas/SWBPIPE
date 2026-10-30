use super::*;
use open_pipe_stress_primitive_loads::LoadApplication;

/// The owner-adopted per-invocation default, 12 GiB: the registered profile's qualified M.
const RETAINED_LIMIT: crate::RetainedByteLimit = crate::RetainedByteLimit::new(12_884_901_888);
fn raw() -> Value {
    json!({"model":serde_json::from_str::<Value>(include_str!("../../../../fixtures/product_preview/numerical_sensitive_torsion_model.json")).unwrap()})
}
struct Fixture {
    model: PreviewModel,
    built: BuiltModel,
    k: Vec<Vec<f64>>,
    f: AssembledForce,
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
        let mut k = assemble_global_stiffness_with_connectors(
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
        let f = crate::nodal_and_eigen_case_force(&loads, &[], &built).unwrap();
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
            load_state: None,
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
/// B4/D1 (b): the composite row binding (the only one) on the exact n05 control.
#[test]
fn selected_projection_mapping_rejects_value_unit_entity_missing_duplicate() {
    let f = exact_fixture(&exact_raw());
    let selected = source_recovery::solve(
        f.input(),
        exact::Limits {
            operations: PHYSICS_SOURCE_WORK_LIMIT,
            ..Default::default()
        },
    )
    .unwrap();
    let expected = rows::expected_primary_for(&f.input(), &selected).unwrap();
    let bindings: Vec<_> = expected
        .iter()
        .map(|(i, r)| FunctionalRowBinding {
            functional_index: *i,
            result_id: r.id.clone(),
        })
        .collect();
    let mut actual: Vec<_> = expected.values().cloned().collect();
    actual.extend(
        rows::derived_for(&f.input(), &selected, &expected)
            .unwrap()
            .into_iter()
            .map(|d| d.row),
    );
    assert!(rows::bind_for(&f.input(), &selected, &actual, &bindings, vec![]).is_ok());
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
        assert!(rows::bind_for(&f.input(), &selected, &bad_rows, &bindings, vec![]).is_err());
    }
    let mut duplicate = bindings.clone();
    duplicate[1].functional_index = duplicate[0].functional_index;
    assert!(rows::bind_for(&f.input(), &selected, &actual, &duplicate, vec![]).is_err());
}
#[test]
fn actual_capture_replay_rejects_changed_material_or_load_source() {
    let original = exact_raw();
    let (_, capture) =
        CapturedInvocation::parse(original.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
    let f = exact_fixture(&original);
    let mut selected = source_recovery::solve(
        f.input(),
        exact::Limits {
            operations: PHYSICS_SOURCE_WORK_LIMIT,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(capture.check_input(&f.input(), &mut selected).is_ok());
    let mut changed = original;
    // The exact contract derives G from E and nu, so the replay is changed through E.
    changed["model"]["materials"][0]["elastic_modulus"]["value"] = json!(201e9);
    let (_, other) = CapturedInvocation::parse(changed, PreviewSolverMode::DenseScrutiny).unwrap();
    let mut selected = source_recovery::solve(
        f.input(),
        exact::Limits {
            operations: PHYSICS_SOURCE_WORK_LIMIT,
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

/// D3 X1 budget (U6-SH, T4-S6): `hash` streams the checked canonical text of the borrowed
/// `{domain, payload}` (a whole selected-source publication, for one) into the hasher. Its hash
/// and its refusal, message included, are the former path's — `json!({"domain": d, "payload": p})`,
/// serialized, checked-parsed, canonicalized, hashed — over every receipt domain and representative
/// payloads: a selected pressure-source-1 publication envelope, its receipt body, a case evidence
/// block, the invocation wrapper and adversarial values (key order, escapes, non-ASCII, signed
/// zero, extreme and unsafe numbers, nesting at and past the parser's depth, a payload naming
/// `domain`). The former construction lives only here.
#[test]
fn receipt_hash_borrowed_wrapper_text_and_hash_are_the_former_objects() {
    let former = |domain: &str, payload: &Value| json!({"domain": domain, "payload": payload});
    let envelope: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json"
    ))
    .unwrap();
    assert_eq!(
        envelope["producer"]["semantic_contract_id"],
        json!(exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID),
        "a selected-source publication"
    );
    let body = envelope["source_block_recovery"]["body"].clone();
    let case = body["cases"][0].clone();
    let mut publication = envelope.clone();
    publication.as_object_mut().unwrap().remove("source_block_recovery");
    let invocation = json!({"request": exact_raw(), "solver_mode": "dense_scrutiny"});
    let adversarial = json!({"z": [1, -0.0, 1e-300, 1.7976931348623157e308, 18446744073709551615u64, -9223372036854775808i64],
        "a": {"payload": null, "domain": "inner", "": [], "\u{1F600}\"\\\n": {}}, "m": [true, false, null, "\u{0}\u{1f}\u{7f}é"]});
    let nest = |depth: usize| (0..depth).fold(json!({"\u{1}é": [0.5]}), |inner, i| if i % 2 == 0 { json!([inner]) } else { json!({"k": inner, "a": "\""}) });
    let mut payloads = vec![publication, envelope.clone(), body.clone(), case, invocation, adversarial, json!(null), json!([]), json!({}), json!("text"), json!(0.1), json!(-0.0)];
    payloads.extend([123, 124, 125, 126].map(nest));
    let domains: [&'static str; 10] = ["source_blocks_invocation_v1", "source_blocks_publication_v1", "source_blocks_receipt_v1",
        "pressure_source_case_evidence_v1", "physics_source_case_evidence_v1", "load_reference_state_resolved_evidence_v1",
        "source_blocks_normalized_source_v1", "source_blocks_functional_plan_v1", "", "a\"b\\c\u{1F600}"];
    let mut refused = 0;
    for domain in domains {
        for payload in &payloads {
            let expected = checked(&former(domain, payload)).map(|text| format!("{:x}", Sha256::digest(text.as_bytes())));
            refused += usize::from(expected.is_err());
            assert_eq!(hash(domain, payload), expected, "{domain}: hash");
        }
    }
    // The adversarial payload's unsafe integers and the two deepest nestings (the leaf object adds
    // a level and its array another) refuse under every domain, with the former messages.
    assert_eq!(refused, 3 * domains.len());
    // The committed selected-source receipt digest is reproduced from its own body.
    assert_eq!(
        hash("source_blocks_receipt_v1", &body).unwrap(),
        envelope["source_block_recovery"]["receipt_sha256"].as_str().unwrap()
    );
}

// Append to source_receipt/tests.rs after parent's bounded public run.
fn exact_raw() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"
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
            operations: PHYSICS_SOURCE_WORK_LIMIT,
            ..Default::default()
        },
    )
    .unwrap();
    capture.check_input(&f.input(), &mut selected).unwrap();
    let member = selected.members()[0].member_id.clone();
    let section = f.built.sections[&member];
    let maximum = composite_member_maximum(&f.input(), &mut selected, &member, pressure_exact::SectionBasis::Nominal, &section).unwrap();
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
            operations: PHYSICS_SOURCE_WORK_LIMIT,
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
                operations: PHYSICS_SOURCE_WORK_LIMIT,
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
            operations: PHYSICS_SOURCE_WORK_LIMIT,
            ..Default::default()
        },
    )
    .unwrap();
    let before = selected.summary().work.charged;
    let member = selected.members()[0].member_id.clone();
    let section = f.built.sections[&member];
    assert!(composite_member_maximum(&f.input(), &mut selected, "missing", pressure_exact::SectionBasis::Nominal, &section).is_err());
    assert!(selected.summary().work.charged > before);
    f.built.sections.get_mut(&member).unwrap().area *= 2.0;
    let section = f.built.sections[&member];
    assert!(composite_member_maximum(&f.input(), &mut selected, &member, pressure_exact::SectionBasis::Nominal, &section).is_err());
    let remaining = selected.summary().work.limit - selected.summary().work.charged;
    selected.charge_finalization(remaining).unwrap();
    let before = selected.summary().work.charged;
    assert!(composite_support_norms(&f.input(), &mut selected, "missing").is_err());
    assert_eq!(selected.summary().work.charged, before);
    assert!(selected.summary().work.rejected.exact().unwrap() > 0);
}

// Measurement only: explicit command; never a production budget override.
#[test]
#[ignore = "bounded resource measurement with private 16M test budget; not public qualification"]
fn composite_fields_work_measurement() {
    let raw: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure_source/physics_source/fields.request.json"
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
            json!({"purpose":"private resource measurement; not public policy or qualification","mode":mode.as_str(),"private_per_case_limit":16_000_000,"public_per_case_limit":PHYSICS_SOURCE_WORK_LIMIT,"stage_trace":trace,"invocation_charged":budget.charged,"publication_charged":budget.publication_charged,"case_work":wire["source_block_recovery"]["body"]["cases"][0]["work"],"receipt_status":wire["source_block_recovery"]["body"]["status"],"diagnostics":wire["diagnostics"].as_array().unwrap().iter().filter(|d|d["severity"]=="blocking").collect::<Vec<_>>()})
        );
        assert!(
            envelope.source_block_recovery.is_none(),
            "private oversized limit must not create a selected-policy receipt"
        );
        assert!(
            trace.last().unwrap()["charged"].as_u64().unwrap() > PHYSICS_SOURCE_WORK_LIMIT as u64
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
    assert_eq!(PHYSICS_SOURCE_WORK_LIMIT, 8_000_000);
    // B4/D1 (b): the preview route attempts no retained-source recovery at any cap.
    let (preview, budget) = run(raw(), PHYSICS_SOURCE_WORK_LIMIT + 1, SOURCE_BLOCKS_INVOCATION_WORK_LIMIT);
    assert!(preview.source_block_recovery.is_none());
    assert!(!preview.diagnostics.iter().any(|d| d.code.starts_with("SOURCE_BLOCK_RECOVERY")));
    assert_eq!((budget.attempts, budget.charged, budget.publication_charged), (0, 0, 0));
    for (value, cap) in [(exact_raw(), PHYSICS_SOURCE_WORK_LIMIT + 1)] {
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

/// U3 (D-2 A1): the retired `1.0.0/legacy_pressure_v1` label is refused before any
/// solve with `PRESSURE_MODEL_REAUTHOR_REQUIRED`, zero pressure included; nothing is
/// published and no source blocks are recovered.
#[test]
fn retired_legacy_pressure_label_is_refused_on_the_ordinary_route() {
    // B7b: the retired v2 label is refused by the same name beside the legacy one.
    for label in [
        json!({"version":"1.0.0","mode":"legacy_pressure_v1"}),
        json!({"version":"2.0.0","mode":"exact_straight_pressure_v2"}),
    ] {
    let mut value = exact_raw();
    value["model"]["pressure_contract"] = label;
    value["model"]["load_cases"][0]
        .as_object_mut()
        .unwrap()
        .remove("pressure_regions");
    for mode in [
        PreviewSolverMode::DenseScrutiny,
        PreviewSolverMode::SparseInteractive,
    ] {
        let envelope = run_linear_static_preview_value_with_mode(value.clone(), mode).unwrap();
        assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE");
        assert!(envelope.results.is_empty());
        assert!(envelope.source_block_recovery.is_none());
        let refusal = envelope
            .diagnostics
            .iter()
            .find(|d| d.code == "PRESSURE_MODEL_REAUTHOR_REQUIRED")
            .unwrap_or_else(|| panic!("{:?}", envelope.diagnostics));
        assert_eq!(refusal.severity, "blocking");
        assert_eq!(refusal.affected_refs, ["pressure_contract"]);
        // B7b A5: the re-author diagnostic names the current contract.
        assert!(refusal.message.contains("3.0.0/exact_pressure_v3"), "{}", refusal.message);
        assert!(!refusal.message.contains("exact_straight_pressure_v2"));
        assert!(!envelope
            .diagnostics
            .iter()
            .any(|d| d.code == "SOURCE_BLOCK_RECOVERY_SELECTED"));
    }
    }
}

/// U3 (D-2 A1; RV127 N-6): a zero-valued legacy pressure primitive in a 0.1.0
/// document is refused on the ordinary route and on the retained entry, which
/// publishes the ordinary refusal byte for byte and never a successor.
#[test]
fn zero_legacy_pressure_primitive_is_refused_on_the_ordinary_route_and_the_retained_entry() {
    let mut value = raw();
    value["model"]["load_cases"][0]["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"load:zero-pressure","category":"pressure",
            "target":{"type":"element","pipe":"pipe"},"direction":"global_x",
            "dimension":"pressure","magnitude":{"value":0.0,"unit":"Pa"},
            "provenance":"retired_legacy_zero_pressure"}));
    for mode in [
        PreviewSolverMode::DenseScrutiny,
        PreviewSolverMode::SparseInteractive,
    ] {
        let envelope = run_linear_static_preview_value_with_mode(value.clone(), mode).unwrap();
        assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE");
        assert!(envelope.results.is_empty());
        let refusal = envelope
            .diagnostics
            .iter()
            .find(|d| d.code == "PRESSURE_MODEL_REAUTHOR_REQUIRED")
            .unwrap_or_else(|| panic!("{:?}", envelope.diagnostics));
        assert_eq!(refusal.affected_refs, ["case", "load:zero-pressure"]);
        // B7b A5: the re-author diagnostic names the current contract.
        assert!(refusal.message.contains("3.0.0/exact_pressure_v3"), "{}", refusal.message);
        assert!(!refusal.message.contains("exact_straight_pressure_v2"));
        let ordinary = serde_json::to_vec(&envelope).unwrap();
        let direct = crate::run_linear_static_preview_value_with_retained_direct(value.clone(), mode, RETAINED_LIMIT).unwrap();
        assert!(direct.retained().is_none());
        match direct.into_publication() {
            crate::RetainedPublication::Ordinary(published) => {
                assert_eq!(serde_json::to_vec(&published).unwrap(), ordinary)
            }
            crate::RetainedPublication::Successor(_) => panic!("a successor for a refused document"),
        }
    }
}

/// U3 (D-2 A1), the retained route: the labelled document is outside D1 (D1.3), so the
/// retained entry publishes the ordinary route's refusal and never a successor.
#[test]
fn retired_legacy_pressure_label_is_refused_on_the_retained_entry() {
    use crate::retained_memory::{AdmissionRefusal, D1Clause};
    // B7b: the retired v2 label is refused by the same name beside the legacy one.
    for label in [
        json!({"version":"1.0.0","mode":"legacy_pressure_v1"}),
        json!({"version":"2.0.0","mode":"exact_straight_pressure_v2"}),
    ] {
    let mut value = exact_raw();
    value["model"]["pressure_contract"] = label;
    value["model"]["load_cases"][0]
        .as_object_mut()
        .unwrap()
        .remove("pressure_regions");
    for mode in [
        PreviewSolverMode::DenseScrutiny,
        PreviewSolverMode::SparseInteractive,
    ] {
        let ordinary = serde_json::to_vec(&run_linear_static_preview_value_with_mode(value.clone(), mode).unwrap()).unwrap();
        let direct = crate::run_linear_static_preview_value_with_retained_direct(value.clone(), mode, RETAINED_LIMIT).unwrap();
        assert!(matches!(
            direct.admission().unwrap().law().domain,
            Some(AdmissionRefusal::Family(D1Clause::Namespace, _))
        ));
        assert!(direct.retained().is_none());
        assert!(direct
            .envelope()
            .diagnostics
            .iter()
            .any(|d| d.code == "PRESSURE_MODEL_REAUTHOR_REQUIRED" && d.affected_refs == ["pressure_contract"]));
        match direct.into_publication() {
            crate::RetainedPublication::Ordinary(envelope) => {
                assert_eq!(serde_json::to_vec(&envelope).unwrap(), ordinary)
            }
            crate::RetainedPublication::Successor(_) => panic!("a successor for a refused document"),
        }
    }
    }
}

/// RF-18 (RF source-method design §3.2 (1), §7): the congruence closure's
/// transform decline keeps the receipt code `unsupported_source_closure`, in
/// every stage it can be reported from, and `Budget` (the unit precheck)
/// keeps `budget`.
#[test]
fn transform_declines_map_to_unsupported_source_closure() {
    use exact::functionals::AttemptStage;
    use source_recovery::{RecoveryError, RecoveryFailure, BLOCK_ROTATION_DECLINE};
    assert!(BLOCK_ROTATION_DECLINE.contains("transform") && !BLOCK_ROTATION_DECLINE.contains("coincident"));
    let failure = |error: RecoveryError, helper_stage: AttemptStage| RecoveryFailure {
        stage: "source closure",
        helper_stage,
        error,
        work: exact::WorkReport::new(0, 0, 0),
    };
    for stage in [AttemptStage::SourceClosure, AttemptStage::Preparation] {
        assert_eq!(
            source::failure_fields(&failure(RecoveryError::Unsupported(BLOCK_ROTATION_DECLINE), stage)),
            ("source_validation", "unsupported_source_closure", None)
        );
        assert_eq!(
            source::failure_fields(&failure(RecoveryError::Exact(exact::Error::Budget), stage)),
            ("source_validation", "budget", None)
        );
    }
}

/// Every support id under a `support_ids` key anywhere in `value`: a plan's
/// strings, or the `id` of an id-map record.
fn support_ids_anywhere(value: &Value, found: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, item) in map {
                if key == "support_ids" {
                    for id in item.as_array().into_iter().flatten() {
                        if let Some(id) = id.as_str().or_else(|| id["id"].as_str()) {
                            found.push(id.to_owned());
                        }
                    }
                }
                support_ids_anywhere(item, found);
            }
        }
        Value::Array(items) => items.iter().for_each(|item| support_ids_anywhere(item, found)),
        _ => {}
    }
}

/// PR #1686 (HELP_HUMAN, 2026-10-27, condition (1)): no solver-produced
/// receipt puts a constant-effort support (`is_constant_effort_support`, by
/// the solver's own reading) in a plan's `support_ids`, which result_export's
/// reader therefore refuses (`SOURCE_BLOCKS_CONSTANT_EFFORT_SUPPORT`). The plan's
/// `support_ids` are `selected.support_actions()` (`source.rs`), built only
/// from `source.support_ids` (`source_recovery.rs` `solve`), which
/// `prepare_sources` fills only from `built.supports`; the linear-support
/// assembly (`lib.rs`) puts no constant-effort support there. `prepare_sources`
/// also declines any model holding one before that: first at its dimension
/// closure (`built.supports` is then shorter than the model's supports), and
/// again by name (`non-nodal load producer present`). The exact route also
/// refuses the family at admission (`pressure_runtime.rs`).
/// Each declaration, by family, by explicit type, padded as `str::trim`
/// removes (the padded types are accepted by the solver's validation; the
/// padded families are refused as tokens), is added to the exact n05 control:
/// the solver reads it as constant effort, builds no support from it, source
/// recovery declines the model, and no publication in either mode names it.
#[test]
fn constant_effort_supports_never_enter_a_plan() {
    const CE: &str = "support:ce";
    let limits = || exact::Limits {
        operations: PHYSICS_SOURCE_WORK_LIMIT,
        ..Default::default()
    };
    let modes = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
    let published = |request: &Value, mode| {
        let envelope = crate::run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
        let mut found = Vec::new();
        support_ids_anywhere(&serde_json::to_value(envelope).unwrap(), &mut found);
        found
    };
    // The control: the plan names the n05 supports, so the walk reaches it.
    let base = exact_raw();
    let control = exact_fixture(&base);
    let selected = source_recovery::solve(control.input(), limits()).unwrap();
    let ids = selected
        .support_actions()
        .iter()
        .map(|s| s.support_id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, ["anchor", "independent-spring"]);
    for mode in modes {
        let found = published(&base, mode);
        assert!(ids.iter().all(|id| found.iter().any(|f| f == id)), "{found:?}");
    }
    let provenance = base["model"]["supports"][0]["provenance"].clone();
    let nel = "\u{85}";
    let declarations: [(&str, Option<String>); 9] = [
        ("constant_effort_support", Some("constant_effort_support".into())),
        ("constant_effort_support", None),
        ("constant_effort_support", Some("  ".into())),
        ("guide", Some("constant_effort_support".into())),
        ("spring", Some("constant_effort_support".into())),
        ("vertical_support", Some(" constant_effort_support ".into())),
        ("spring", Some(format!("{nel}constant_effort_support{nel}"))),
        (" constant_effort_support ", None),
        ("\u{85}constant_effort_support\u{85}", None),
    ];
    for (family, hanger_type) in declarations {
        let mut hanger = json!({"constant_load": {"value": 375.0, "unit": "N"},
            "travel_range": {"value": 0.05, "unit": "m"}, "source_reference": "invented"});
        if let Some(hanger_type) = &hanger_type {
            hanger["hanger_type"] = json!(hanger_type);
        }
        let support = json!({"id": CE, "node": "independent-tip", "family": family,
            "restraints": ["UY"], "hanger": hanger, "provenance": provenance});
        let at = format!("{family:?} / {hanger_type:?}");
        let typed: PreviewSupport = serde_json::from_value(support.clone()).unwrap();
        assert!(crate::is_constant_effort_support(&typed), "{at}");
        let mut request = base.clone();
        request["model"]["supports"].as_array_mut().unwrap().push(support);
        let fixture = exact_fixture(&request);
        let built = fixture
            .built
            .supports
            .iter()
            .map(|s| s.support_id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(built, ["anchor", "independent-spring"], "{at}");
        let declined = source_recovery::solve(fixture.input(), limits());
        assert!(
            matches!(
                &declined,
                Err(source_recovery::RecoveryFailure {
                    error: source_recovery::RecoveryError::SourceMismatch("actual model/source dimensions"),
                    ..
                })
            ),
            "{at}: {:?}",
            declined.err()
        );
        for mode in modes {
            let found = published(&request, mode);
            assert!(!found.iter().any(|f| f == CE), "{at}: {found:?}");
        }
    }
}

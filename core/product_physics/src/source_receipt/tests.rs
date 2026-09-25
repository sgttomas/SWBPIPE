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
    let selected = source_recovery::solve(f.input(), exact::Limits { operations: SOURCE_BLOCKS_WORK_LIMIT, ..Default::default() }).unwrap();
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
    let mut selected = source_recovery::solve(f.input(), exact::Limits { operations: SOURCE_BLOCKS_WORK_LIMIT, ..Default::default() }).unwrap();
    assert!(capture.check_input(&f.input(), &mut selected).is_ok());
    let mut changed = original;
    changed["model"]["materials"][0]["shear_modulus"]["value"] = json!(81e9);
    let (_, other) = CapturedInvocation::parse(changed, PreviewSolverMode::DenseScrutiny).unwrap();
    let mut selected = source_recovery::solve(f.input(), exact::Limits { operations: SOURCE_BLOCKS_WORK_LIMIT, ..Default::default() }).unwrap();
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

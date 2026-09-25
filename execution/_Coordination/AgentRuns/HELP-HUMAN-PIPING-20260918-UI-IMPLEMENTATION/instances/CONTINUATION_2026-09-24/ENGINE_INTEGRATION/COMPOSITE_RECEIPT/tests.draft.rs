// Append to source_receipt/tests.rs after parent's bounded public run.
fn exact_raw() -> Value {
    serde_json::from_str(include_str!("../../../../fixtures/product_preview/physics_source/n05.request.json")).unwrap()
}
fn exact_fixture(raw: &Value) -> Fixture {
    let request: LinearStaticPreviewRequest=serde_json::from_value(raw.clone()).unwrap();
    let mut model=request.model;
    let mut materials=if request.materials.is_empty(){model.materials.clone()}else{request.materials};
    let mut diagnostics=vec![];
    resolve_shared_sections(&mut model,&mut diagnostics);
    normalize_model_units(&mut model,&mut materials,&mut diagnostics);
    pressure_material::resolve_base(&model,&mut materials,&mut diagnostics);
    if modulus_basis_key(&model.load_cases[0],&mut diagnostics).is_some() {
        materials=pressure_material::resolve_case(&model,&materials,&model.load_cases[0],&mut diagnostics).unwrap().0;
    }
    assert!(!has_blocking(&diagnostics),"{diagnostics:?}");
    model.materials=materials;
    Fixture::new(model)
}
#[test]
fn composite_captured_exact_empty_regions_replay_and_overrides() {
    let mut raw=exact_raw();
    raw["unknown_actual_capture_marker"]=json!({"kept":true});
    raw["materials"]=raw["model"]["materials"].clone();
    // Material override is the actual source; a disagreeing model copy cannot win.
    raw["model"]["materials"][0]["elastic_modulus"]["value"]=json!(123.0);
    let f=exact_fixture(&raw);
    let (_,capture)=CapturedInvocation::parse(raw.clone(),PreviewSolverMode::DenseScrutiny).unwrap();
    let mut selected=source_recovery::solve(f.input(),exact::Limits{operations:SOURCE_BLOCKS_WORK_LIMIT,..Default::default()}).unwrap();
    capture.check_input(&f.input(),&mut selected).unwrap();
    let member=selected.members()[0].member_id.clone();
    let maximum=composite_member_maximum(&f.input(),&mut selected,&member).unwrap();
    let evidence=maximum.evidence("test:maximum");
    assert_eq!(evidence["basis"],"retained_source_endpoint_normal_max_v1");
    assert_eq!(evidence["load_case_id"],f.model.load_cases[0].id);
    assert_eq!(evidence["endpoints"][0]["functional_indices"],json!([0usize,4,5].map(|c|selected.members()[0].section_functional_indices[0][c])));
    let support=selected.support_actions()[0].support_id.clone();
    assert!(composite_support_norms(&f.input(),&mut selected,&support).unwrap().iter().all(|x|x.is_finite()));
    let mut wrong=raw;
    wrong["materials"][0]["poisson_ratio"]["value"]=json!(0.123);
    let (_,capture)=CapturedInvocation::parse(wrong,PreviewSolverMode::DenseScrutiny).unwrap();
    let mut selected=source_recovery::solve(f.input(),exact::Limits{operations:SOURCE_BLOCKS_WORK_LIMIT,..Default::default()}).unwrap();
    assert!(capture.check_input(&f.input(),&mut selected).is_err());
}
#[test]
fn composite_adapter_refuses_missing_zero_and_nonzero_regions() {
    for pressure in [None,Some(0.0),Some(123.0)] {
        let mut f=exact_fixture(&exact_raw());
        f.model.load_cases[0].pressure_regions=pressure.map(|value|vec![pressure_runtime::PressureRegionInput {pressure:Some(Quantity{value,unit:"Pa".into()}),..Default::default()}]);
        let error=source_recovery::solve(f.input(),exact::Limits{operations:SOURCE_BLOCKS_WORK_LIMIT,..Default::default()}).unwrap_err();
        assert!(matches!(error.error,source_recovery::RecoveryError::Unsupported(_)));
        assert!(error.work.charged>0);
    }
}
#[test]
fn composite_recipe_reserves_before_failures_and_geometry_changes() {
    let mut f=exact_fixture(&exact_raw());
    let mut selected=source_recovery::solve(f.input(),exact::Limits{operations:SOURCE_BLOCKS_WORK_LIMIT,..Default::default()}).unwrap();
    let before=selected.summary().work.charged;
    assert!(composite_member_maximum(&f.input(),&mut selected,"missing").is_err());
    assert!(selected.summary().work.charged>before);
    let member=selected.members()[0].member_id.clone();
    f.built.sections.get_mut(&member).unwrap().area *= 2.0;
    assert!(composite_member_maximum(&f.input(),&mut selected,&member).is_err());
    let remaining=selected.summary().work.limit-selected.summary().work.charged;
    selected.charge_finalization(remaining).unwrap();
    let before=selected.summary().work.charged;
    assert!(composite_support_norms(&f.input(),&mut selected,"missing").is_err());
    assert_eq!(selected.summary().work.charged,before);
    assert!(selected.summary().work.rejected>0);
}

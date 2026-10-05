
// Fresh RV60 tests appended only to the external copy of retained_product_tests.rs.
#[test]
fn rv60_closed_boundary_mutations() {
    let (e, o) = observed(specimen(true));
    let (_, case)=o.native.as_ref().unwrap();
    let k::ExecutionOutcome::Selected(owner)=&case.outcome else { panic!() };
    let mut changed=e.clone();
    let row=changed.results.iter_mut().find(|r|r.kind=="linear_solver_mode_basis").unwrap();
    row.metadata.as_mut().unwrap().basis="foreign basis".into();
    row.metadata.as_mut().unwrap().sign_convention="foreign sign".into();
    println!("RV60_MODE_METADATA_ACCEPTED {}",o.bind_rows(&changed,owner).is_ok());
    let mut changed=e.clone();
    changed.contract_evidence.as_mut().unwrap()["preview_cases"][0]["support_attribution"]["foreign"]=serde_json::json!(true);
    println!("RV60_EXTRA_ATTRIBUTION_ACCEPTED {}",o.observables(&changed).is_ok());
    let mut changed=e.clone();
    changed.contract_evidence.as_mut().unwrap()["preview_cases"][0]["stress_maximum_coverage"]["foreign"]=serde_json::json!(true);
    println!("RV60_EXTRA_COVERAGE_ACCEPTED {}",o.observables(&changed).is_ok());
    let mut changed=e.clone();
    changed.contract_evidence.as_mut().unwrap()["preview_cases"][0]["pipe_stress_extrema"][0]["foreign"]=serde_json::json!(true);
    assert!(o.observables(&changed).is_err());
    let (request,_)=source_receipt::CapturedInvocation::parse(specimen(true),PreviewSolverMode::SparseInteractive).unwrap();
    let (_,foreign)=source_receipt::CapturedInvocation::parse(specimen(false),PreviewSolverMode::SparseInteractive).unwrap();
    let mut observer=ProductCapture::default();
    let foreign_e=run_linear_static_preview_observed(request,PreviewSolverMode::SparseInteractive,Some(&foreign),&mut SourceRecoveryBudget::default(),Some(&mut observer));
    println!("RV60_FOREIGN_CAPTURE_ERROR {:?}",observer.error);
    assert_eq!(serde_json::to_string(&foreign_e).unwrap(),serde_json::to_string(&e).unwrap());
    // Native source/run identity must reject another actual selected invocation.
    let (_,foreign_o)=observed(specimen(true));
    let (_,foreign_case)=foreign_o.native.as_ref().unwrap();
    let k::ExecutionOutcome::Selected(foreign_owner)=&foreign_case.outcome else { panic!() };
    let (invocation,case)=o.native.as_ref().unwrap();
    let rows=o.bind_rows(&e,owner).unwrap();
    let spent=invocation.certify_product_case(case.run,foreign_owner,&o.facts,&rows);
    assert_eq!(spent.failure().unwrap().category(),"association");
    let spent=invocation.certify_product_case(usize::MAX,owner,&o.facts,&rows);
    assert_eq!(spent.failure().unwrap().category(),"association");
    println!("RV60_FOREIGN_NATIVE_AND_RUN_REJECTED true");
}
#[test]
fn rv60_source_ownership_and_occurrence_controls() {
    // Preserve exact authored source ordinals through reversed input ordering.
    let mut raw=specimen(true);
    raw["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().reverse();
    let (_,o)=observed(raw);
    assert!(o.error.is_none(),"{:?}",o.error);
    for t in &o.terms { assert_eq!(t.original,2-t.occurrence); }
    let mut canonical=o.terms.iter().map(|t|t.canonical).collect::<Vec<_>>();canonical.sort();
    assert_eq!(canonical,vec![0,1,2]);
    // Original model material custody, rather than a synthetic request override.
    let mut raw=specimen(true);
    raw["model"]["materials"]=raw["materials"].clone();raw["materials"]=serde_json::json!([]);
    let (_,o)=observed(raw);assert!(o.error.is_none(),"{:?}",o.error);assert!(!o.request_materials);
    println!("RV60_ORIGINAL_MODEL_AND_REVERSED_TERMS true");
}

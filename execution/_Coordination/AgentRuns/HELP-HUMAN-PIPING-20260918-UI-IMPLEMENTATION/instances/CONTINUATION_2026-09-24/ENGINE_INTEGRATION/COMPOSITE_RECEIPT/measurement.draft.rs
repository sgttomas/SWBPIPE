// Measurement only: explicit command; never a production budget override.
#[test]
#[ignore = "bounded resource measurement with private 16M test budget; not public qualification"]
fn composite_fields_work_measurement() {
    let raw:Value=serde_json::from_str(include_str!("../../../../fixtures/product_preview/physics_source/fields.request.json")).unwrap();
    for mode in [PreviewSolverMode::DenseScrutiny,PreviewSolverMode::SparseInteractive] {
        let (request,capture)=CapturedInvocation::parse(raw.clone(),mode).unwrap();
        let mut budget=SourceRecoveryBudget{per_case_limit:16_000_000,..Default::default()};
        composite::start_trace();
        let envelope=run_linear_static_preview_captured(request,mode,Some(&capture),&mut budget);
        let trace=composite::take_trace();
        let wire=serialized(&envelope).unwrap();
        eprintln!("COMPOSITE_WORK_MEASUREMENT {}",json!({"purpose":"private resource measurement; not public policy or qualification","mode":mode.as_str(),"private_per_case_limit":16_000_000,"public_per_case_limit":SOURCE_BLOCKS_WORK_LIMIT,"stage_trace":trace,"invocation_charged":budget.charged,"publication_charged":budget.publication_charged,"case_work":wire["source_block_recovery"]["body"]["cases"][0]["work"],"receipt_status":wire["source_block_recovery"]["body"]["status"],"diagnostics":wire["diagnostics"].as_array().unwrap().iter().filter(|d|d["severity"]=="blocking").collect::<Vec<_>>()}));
        assert_eq!(wire["source_block_recovery"]["body"]["status"],"qualified");
        assert!(wire["source_block_recovery"]["body"]["cases"][0]["work"]["charged"].as_u64().unwrap()>SOURCE_BLOCKS_WORK_LIMIT as u64);
    }
}

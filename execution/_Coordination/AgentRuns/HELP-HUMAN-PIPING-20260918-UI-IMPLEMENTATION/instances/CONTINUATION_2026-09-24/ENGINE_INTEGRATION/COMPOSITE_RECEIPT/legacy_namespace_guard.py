from pathlib import Path
p=Path('projects/chirality-piping/core/product_physics/src/source_recovery.rs');s=p.read_text();old='''    } else if input.load_case.pressure_regions.is_some() {
        return Err(unsupported(
            "pressure region namespace outside exact source profile",
        ));
    }''';new='''    } else if !matches!(input.model.schema_version.as_str(),"0.1.0"|"0.2.0") || input.model.pressure_contract.is_some() || input.load_case.pressure_regions.is_some() {
        return Err(unsupported("legacy source-blocks namespace requires model0.1/0.2 without pressure contract or regions"));
    }''';assert old in s;s=s.replace(old,new,1);p.write_text(s)
p=Path('projects/chirality-piping/core/product_physics/src/source_receipt/tests.rs');s=p.read_text();s+='''
#[test]
fn legacy_model_three_retains_ordinary_route_without_old_source_namespace() {
    let mut value=exact_raw();
    value["model"]["pressure_contract"]=json!({"version":"1.0.0","mode":"legacy_pressure_v1"});
    value["model"]["load_cases"][0].as_object_mut().unwrap().remove("pressure_regions");
    for mode in [PreviewSolverMode::DenseScrutiny,PreviewSolverMode::SparseInteractive] {
        let envelope=run_linear_static_preview_value_with_mode(value.clone(),mode).unwrap();
        assert!(envelope.source_block_recovery.is_none());
        assert!(!envelope.diagnostics.iter().any(|d|d.code=="SOURCE_BLOCK_RECOVERY_SELECTED"));
        assert!(envelope.diagnostics.iter().any(|d|d.code=="SOURCE_BLOCK_RECOVERY_UNAVAILABLE"&&d.message.contains("legacy source-blocks namespace")),"{:?}",envelope.diagnostics);
        assert!(!envelope.diagnostics.iter().any(|d|d.code=="PREVIEW_CONTRACT_VERSION_MISMATCH"));
    }
}
''';p.write_text(s)

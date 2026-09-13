    // CONTACT_REPAIR_V2_EXPECTATIONS_BEGIN
    #[test]
    fn contact_recovery_failed_sparse_and_dense_attempt_emits_no_success_fallback() {
        let mut input=gap_closure_preview_request();
        input.model.supports[0].restraints.retain(|dof|dof!="UX");
        input.model.supports[1].nonlinear=None;
        input.model.supports[1].family=Some("guide".into());
        input.model.supports[1].restraints=vec!["UY".into()];
        let result=run_linear_static_preview_with_mode(input,PreviewSolverMode::SparseInteractive);
        assert_ne!(result.status.mechanics,"MECHANICS_SOLVED");
        assert!(result.diagnostics.iter().any(|d|d.severity=="blocking" && d.message.contains("singular")));
        assert!(!result.diagnostics.iter().any(|d|d.code=="SPARSE_INTERACTIVE_DENSE_FALLBACK"),"{:?}",result.diagnostics);
        assert!(result.results.is_empty());
    }

    #[test]
    fn contact_recovery_product_selected_tip_matches_axial_oracle_and_reverse_blocks() {
        // Existing invented fixture: L=1 m, E=200e9 Pa, OD=.168 m,
        // wall=.007 m, A=pi/4*(OD^2-ID^2), F=100000 N, g=.05 mm.
        // Selected tip ux=(g+F*L/(E*A))*1000 mm; no output-derived expectation.
        let area=std::f64::consts::PI/4.0*(0.168_f64.powi(2)-0.154_f64.powi(2));
        let expected_tip_mm=0.05+100000.0/(200e9*area)*1000.0;
        for mode in [PreviewSolverMode::DenseScrutiny,PreviewSolverMode::SparseInteractive] {
            for seed in ["active","inactive"] {
                let mut input=gap_closure_preview_request();
                input.model.supports[0].restraints.retain(|dof|dof!="UX");
                input.model.supports[1].node="node:N-100".into();
                input.model.supports[1].nonlinear.as_mut().unwrap().initial_state=Some(seed.into());
                let solved=run_linear_static_preview_with_mode(input.clone(),mode);
                assert_eq!(solved.status.mechanics,"MECHANICS_SOLVED");
                assert!((result_value(&solved,"result:disp:node-N-110:ux")-expected_tip_mm).abs()<=0.5e-6+1e-10);
                assert!((result_value(&solved,"result:disp:node-N-100:ux")-0.05).abs()<=0.5e-6+1e-10);
                input.model.load_cases[0].primitive_loads[0].magnitude.value=-100000.0;
                let reversed=run_linear_static_preview_with_mode(input,mode);
                assert_ne!(reversed.status.mechanics,"MECHANICS_SOLVED"); assert!(reversed.results.is_empty());
                assert!(reversed.diagnostics.iter().any(|d|d.severity=="blocking" && d.message.contains("singular")));
            }
        }
    }
    // CONTACT_REPAIR_V2_EXPECTATIONS_END

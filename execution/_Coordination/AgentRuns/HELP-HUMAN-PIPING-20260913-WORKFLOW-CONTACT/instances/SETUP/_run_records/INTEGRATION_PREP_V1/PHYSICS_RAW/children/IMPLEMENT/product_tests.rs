    // CONTACT_RECOVERY_FROZEN_BEGIN
    #[test]
    fn contact_recovery_product_crosses_preflight_and_absent_linear_solution() {
        for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
            for seed in ["active", "inactive"] {
                let mut input=gap_closure_preview_request();
                input.model.supports[0].restraints.retain(|dof|dof!="UX");
                input.model.supports[1].node="node:N-100".to_string();
                input.model.supports[1].nonlinear.as_mut().unwrap().initial_state=Some(seed.to_string());
                let result=run_linear_static_preview_with_mode(input,mode);
                assert_eq!(result.status.mechanics,"MECHANICS_SOLVED","{:?}",result.diagnostics);
                assert!((result_value(&result,"result:nonlinear-support:support-NL-GAP-110:ux-displacement")-0.05).abs()<=0.5e-6+1e-10);
                assert!((result_value(&result,"result:nonlinear-support:support-NL-GAP-110:ux-reaction")+100000.0).abs()<=0.5e-6+1e-10);
                assert!(result.results.iter().all(|r|r.kind!="linear_solver_mode_basis" && !r.kind.contains("dense_sparse")));
                assert!(!result.diagnostics.iter().any(|d|d.code=="SPARSE_INTERACTIVE_DENSE_FALLBACK"));
                if seed=="inactive" { assert!(result.diagnostics.iter().any(|d|d.severity=="warning" && d.message.contains("all-active"))); }
            }
        }
    }
    // CONTACT_RECOVERY_FROZEN_END

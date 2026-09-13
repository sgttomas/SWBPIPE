    // CONTACT_LATER_GUARD_FROZEN_BEGIN
    #[test]
    fn contact_recovery_later_changed_boundary_rejects_classifier_tolerance_one() {
        for mode in [LinearSolveMode::DenseScrutiny,LinearSolveMode::SparseInteractive] {
            let mut input=contact_recovery_problem(ActiveSetState::Inactive,ActiveSetState::Inactive,true,2);
            input.convergence.residual_tolerance=1.0;
            let result=solve_active_set_frame_with_mode(&input,mode).unwrap();
            assert!(!result.converged); assert_eq!(result.iterations.len(),2);
            assert_eq!(result.iterations[0].active_set.changed_supports,vec!["tip".to_string()]);
            assert_eq!(result.iterations[1].active_set.changed_supports,vec!["root".to_string()]);
            assert!(result.iterations[1].active_set.converged);
            assert!(result.diagnostics.iter().any(|d|d.code==SolverDiagnosticCode::NonConvergence));
            let mixed=solve_active_set_frame_with_mode(&contact_recovery_problem(ActiveSetState::Inactive,ActiveSetState::Active,false,4),mode).unwrap();
            assert!(mixed.converged); assert!(!mixed.diagnostics.iter().any(|d|d.message.contains("all-active")));
        }
    }
    // CONTACT_LATER_GUARD_FROZEN_END

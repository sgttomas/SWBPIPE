    #[test]
    fn contact_recovery_warning_preserves_original_singular_seed_provenance() {
        for mode in [LinearSolveMode::DenseScrutiny,LinearSolveMode::SparseInteractive] {
            let mut input=contact_recovery_problem(ActiveSetState::Inactive,ActiveSetState::Inactive,false,4);
            input.initial_states.reverse(); input.nonlinear_supports.reverse();
            let solved=solve_active_set_frame_with_mode(&input,mode).unwrap();
            let warning=solved.diagnostics.iter().find(|d|d.severity==DiagnosticSeverity::Warning && d.message.contains("all-active")).unwrap();
            assert!(warning.message.contains("original_singular_pivot="));
            assert!(warning.message.contains("original_states=[root=inactive,tip=inactive]"));
            assert!(warning.message.contains("recovery_states=[root=active,tip=active]"));
            assert!(warning.message.contains("original_ground_dofs=[1, 2, 3, 4, 5, 7, 8, 9, 10, 11]"));
            assert!(warning.message.contains("recovery_ground_dofs=[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]"));
        }
    }

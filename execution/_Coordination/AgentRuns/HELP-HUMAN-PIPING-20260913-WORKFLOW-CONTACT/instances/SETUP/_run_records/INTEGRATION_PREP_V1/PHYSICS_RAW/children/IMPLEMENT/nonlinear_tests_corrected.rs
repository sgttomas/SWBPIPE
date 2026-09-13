    // CONTACT_RECOVERY_FROZEN_BEGIN
    fn contact_recovery_problem(root: ActiveSetState, tip: ActiveSetState, reverse: bool, cap: usize) -> NonlinearFrameSolveInput {
        let supports = vec![
            NonlinearSupport::gap("root", 0, FrameDof::Ux, 0.05, GapDirection::PositiveDisplacement).unwrap(),
            NonlinearSupport::gap("tip", 1, FrameDof::Ux, 0.20, GapDirection::PositiveDisplacement).unwrap(),
        ];
        let mut input = two_node_axial_problem(supports, vec![SupportStateRecord::new("root", root), SupportStateRecord::new("tip", tip)], cap);
        input.base_restrained_dofs.retain(|dof| *dof != 0);
        if reverse { input.force[6] = -10.0; }
        input
    }

    #[test]
    fn contact_recovery_two_gap_independent_oracle_seeds_order_modes() {
        for mode in [LinearSolveMode::DenseScrutiny, LinearSolveMode::SparseInteractive] {
            for root in [ActiveSetState::Active, ActiveSetState::Inactive] {
                for tip in [ActiveSetState::Active, ActiveSetState::Inactive] {
                    for reverse_order in [false, true] {
                        let mut input = contact_recovery_problem(root, tip, false, 4);
                        if reverse_order { input.nonlinear_supports.reverse(); input.initial_states.reverse(); }
                        let solved = solve_active_set_frame_with_mode(&input, mode).unwrap();
                        assert!(solved.converged);
                        for (actual, expected) in [(solved.displacements[0],0.05),(solved.displacements[6],0.15),(solved.reactions[0],-10.0),(solved.reactions[6],0.0)] { assert!((actual-expected).abs() < 1e-10); }
                        assert_eq!(solved.final_states, vec![SupportStateRecord::new("root",ActiveSetState::Active),SupportStateRecord::new("tip",ActiveSetState::Inactive)]);
                        if root == ActiveSetState::Inactive && tip == ActiveSetState::Inactive {
                            let first=&solved.iterations[0];
                            assert_eq!(first.iteration,1);
                            assert!((first.reactions[0]+15.0).abs()<1e-10);
                            assert!((first.reactions[6]-5.0).abs()<1e-10);
                            assert_eq!(first.active_set.changed_supports, vec!["tip".to_string()]);
                            assert!(solved.diagnostics.iter().any(|d| d.severity==DiagnosticSeverity::Warning && d.message.contains("all-active") && d.message.contains("first") && d.message.contains(mode.as_str())));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn contact_recovery_caps_and_later_singularity_are_honest() {
        for mode in [LinearSolveMode::DenseScrutiny, LinearSolveMode::SparseInteractive] {
            let mut stable=contact_recovery_problem(ActiveSetState::Inactive,ActiveSetState::Inactive,false,1);
            stable.nonlinear_supports.pop(); stable.initial_states.pop();
            let solved=solve_active_set_frame_with_mode(&stable,mode).unwrap();
            assert!(solved.converged); assert_eq!(solved.iterations.len(),1);
            let mut changing=contact_recovery_problem(ActiveSetState::Inactive,ActiveSetState::Inactive,false,2);
            changing.convergence.residual_tolerance=1.0;
            let solved=solve_active_set_frame_with_mode(&changing,mode).unwrap();
            assert!(solved.converged); assert_eq!(solved.iterations.len(),2);
            changing.convergence.max_iterations=1;
            let capped=solve_active_set_frame_with_mode(&changing,mode).unwrap();
            assert!(!capped.converged); assert!(capped.diagnostics.iter().any(|d| d.code==SolverDiagnosticCode::NonConvergence));
            let reverse=contact_recovery_problem(ActiveSetState::Inactive,ActiveSetState::Active,true,4);
            assert!(matches!(solve_active_set_frame_with_mode(&contact_recovery_problem(ActiveSetState::Active,ActiveSetState::Active,true,4),mode),Err(NonlinearIntegrationError::FrameKernel(FrameKernelError::SingularSystem{..}))));
            assert!(matches!(solve_active_set_frame_with_mode(&reverse,mode),Err(NonlinearIntegrationError::FrameKernel(FrameKernelError::SingularSystem{..}))));
            let mut reverse_cap=reverse.clone(); reverse_cap.convergence.max_iterations=2; reverse_cap.convergence.residual_tolerance=1.0;
            let released=solve_active_set_frame_with_mode(&reverse_cap,mode).unwrap();
            assert!(!released.converged);
            assert_eq!(released.iterations[0].active_set.changed_supports,vec!["root".to_string(),"tip".to_string()]);
            assert_eq!(released.iterations[1].active_set.changed_supports,vec!["root".to_string()]);
        }
    }

    #[test]
    fn contact_recovery_invalid_classes_do_not_rescue_and_springs_survive() {
        let original=contact_recovery_problem(ActiveSetState::Inactive,ActiveSetState::Inactive,false,4);
        for case in 0..7 {
            let mut input=original.clone();
            match case {
                0=>input.nonlinear_supports[0].dof=FrameDof::Rx,
                1=>input.initial_states[0].state=ActiveSetState::Sticking,
                2=>input.nonlinear_supports[0].gap=Some(f64::NAN),
                3=>{input.nonlinear_supports[1].node_index=0;},
                4=>{input.base_restrained_dofs.push(0); input.base_restrained_dofs.retain(|dof|*dof!=1);},
                5=>input.nonlinear_supports[0].node_index=2,
                _=>input.nonlinear_supports[0].behavior=NonlinearSupportBehavior::Friction,
            }
            assert!(solve_active_set_frame(&input).is_err(),"invalid eligibility case {case}");
        }
        let mut spring=original;
        spring.nonlinear_supports.clear(); spring.initial_states.clear();
        let solved=solve_active_set_frame_with_mode_and_springs(&spring,LinearSolveMode::SparseInteractive,&[(0,200.0)]).unwrap();
        assert!(solved.converged); assert!((solved.displacements[0]-0.05).abs()<1e-10); assert!((solved.displacements[6]-0.15).abs()<1e-10);
    }
    // CONTACT_RECOVERY_FROZEN_END

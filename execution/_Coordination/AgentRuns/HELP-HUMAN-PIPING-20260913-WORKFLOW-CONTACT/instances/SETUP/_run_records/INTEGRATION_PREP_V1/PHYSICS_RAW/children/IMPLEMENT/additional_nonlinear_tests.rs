    // CONTACT_CLASS_FROZEN_BEGIN
    #[test]
    fn contact_recovery_signed_gap_one_way_lift_off_and_zero_conventions() {
        for mode in [LinearSolveMode::DenseScrutiny,LinearSolveMode::SparseInteractive] {
            for sign in [-1.0,1.0] {
                for family in 0..3 {
                    let sense=if sign>0.0 { ActivationSense::NegativeReaction } else { ActivationSense::PositiveReaction };
                    let contact=match family {
                        0=>NonlinearSupport::gap("root",0,FrameDof::Ux,0.05,if sign>0.0 {GapDirection::PositiveDisplacement} else {GapDirection::NegativeDisplacement}).unwrap(),
                        1=>NonlinearSupport::one_way("root",0,FrameDof::Ux,sense),
                        _=>NonlinearSupport::lift_off("root",0,FrameDof::Ux,sense),
                    };
                    let mut input=two_node_axial_problem(vec![contact],vec![SupportStateRecord::new("root",ActiveSetState::Inactive)],1);
                    input.base_restrained_dofs.retain(|dof|*dof!=0); input.force[6]=sign*10.0;
                    let solved=solve_active_set_frame_with_mode(&input,mode).unwrap(); assert!(solved.converged);
                    let gap=if family==0 {sign*0.05} else {0.0};
                    assert!((solved.displacements[0]-gap).abs()<1e-10); assert!((solved.displacements[6]-(gap+sign*0.1)).abs()<1e-10); assert!((solved.reactions[0]+sign*10.0).abs()<1e-10);
                    input.force[6]=0.0;
                    let zero=solve_active_set_frame_with_mode(&input,mode).unwrap();
                    assert_eq!(zero.final_states[0].state,if family==0 {ActiveSetState::Active} else {ActiveSetState::Inactive});
                    assert_eq!(zero.converged,family==0);
                }
            }
        }
    }
    // CONTACT_CLASS_FROZEN_END

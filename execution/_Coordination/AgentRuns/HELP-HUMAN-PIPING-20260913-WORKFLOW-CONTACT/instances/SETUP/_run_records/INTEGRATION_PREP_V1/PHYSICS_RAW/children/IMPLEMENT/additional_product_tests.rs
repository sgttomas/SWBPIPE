    // CONTACT_CLASS_FROZEN_BEGIN
    #[test]
    fn contact_recovery_product_insufficient_or_invalid_potential_restraint_stays_blocked() {
        for case in 0..5 {
            let mut input=gap_closure_preview_request();
            input.model.supports[0].restraints.retain(|dof|dof!="UX"); input.model.supports[1].node="node:N-100".to_string();
            match case {
                0=>input.model.supports[0].restraints.retain(|dof|dof!="UY"),
                1=>input.model.supports[1].nonlinear.as_mut().unwrap().dof="RX".to_string(),
                2=>input.model.supports[1].nonlinear.as_mut().unwrap().initial_state=Some("sticking".to_string()),
                3=>input.model.supports[1].nonlinear.as_mut().unwrap().gap.as_mut().unwrap().value=-1.0,
                _=>input.model.supports[1].nonlinear.as_mut().unwrap().dof="UY".to_string(),
            }
            let result=run_linear_static_preview(input); assert_ne!(result.status.mechanics,"MECHANICS_SOLVED");
            assert!(result.results.is_empty()); assert!(result.diagnostics.iter().any(|d|d.severity=="blocking"));
        }
    }
    // CONTACT_CLASS_FROZEN_END

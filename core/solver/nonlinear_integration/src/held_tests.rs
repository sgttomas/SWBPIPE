//! RT-A5 `minuend_final` (DEL-05-02 Work `operating-case.md` §7.3, §7.9 H-10):
//! the held-status wrapper against the loop it shares its step and tail with.
//!
//! H-10 (the wrapper against the loop): a case held at its own final states,
//! oriented by its own penultimate iterate (the loop's previous iterate for its
//! final step), and separately by its own final iterate, reproduces the loop's
//! final iterate under the projection §7.9 names, in both modes:
//! - compared bit for bit: the boundary (DOFs and prescribed values), the
//!   applied sliding forces, the displacements and reactions, the final solve's
//!   reports and sparse evidence, the classifier's decisions (the active-set
//!   record), the branch admissibility, the strict-gap evidence, the converged
//!   predicate, the qualification verdict as a value, and the confirmation's
//!   outcome;
//! - excluded, by name: the iteration index (the record's and its active-set
//!   record's), the history, the strict-gap report's identity, work and private
//!   proof metadata (its public decisions, projections and partitions are compared), (the held result's
//!   first record is the orienting iterate it was given) and, for the
//!   final-iterate orientation only, the residual deltas, which are measured
//!   from the orienting iterate.
//! Each receipt authenticates against its own input and history, and a one-bit
//! edit of the held result invalidates it.
use crate::confirmation::Confirmation;
use crate::*;
use open_pipe_stress_frame_kernel::load_ledger::LoadLedger;
use open_pipe_stress_frame_kernel::{FrameDof, FrameElement, FrameNode, FrameSection};
use open_pipe_stress_nonlinear_supports::{ActivationSense, GapDirection, SupportStateRecord};

const MODES: [LinearSolveMode; 2] = [LinearSolveMode::DenseScrutiny, LinearSolveMode::SparseInteractive];

/// A 2 m cantilever along X (as `confirmation_tests::cantilever`): node 0
/// anchored, node 1 the tip; `base` adds tip restraints; nodal loads are input
/// terms of a ledger.
fn cantilever(
    supports: Vec<NonlinearSupport>,
    seeds: Vec<SupportStateRecord>,
    explicit: Vec<FrictionNormalReaction>,
    derived: Vec<DerivedFrictionNormalReaction>,
    base: &[usize],
    loads: &[(usize, f64)],
) -> (NonlinearFrameSolveInput, AssembledForce) {
    let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
    let node_j = FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap();
    let section = FrameSection::new(2.0e11, 8.0e10, 3.5e-3, 1.15e-5, 1.15e-5, 2.3e-5).unwrap();
    let element = FrameElement::new(node_i, node_j, section, [0.0, 0.0, 1.0]).unwrap();
    let mut ledger = LoadLedger::new();
    for &(dof, value) in loads {
        ledger.push("nodal", dof, value);
    }
    let force = ledger.finish(12).unwrap();
    let mut base_restrained = (0..6).collect::<Vec<_>>();
    base_restrained.extend_from_slice(base);
    (
        NonlinearFrameSolveInput {
            base_prescribed: Vec::new(),
            ground_motion: Vec::new(),
            node_count: 2,
            elements: vec![element],
            connectors: Vec::new(),
            curved_bend_elements: Vec::new(),
            force: force.values().to_vec(),
            base_restrained_dofs: base_restrained,
            nonlinear_supports: supports,
            initial_states: seeds,
            friction_normal_reactions: explicit,
            derived_friction_normal_reactions: derived,
            convergence: ConvergenceControl::new("held-test", ConvergencePolicyStatus::Accepted, 0.0, 0.0, 12)
                .unwrap(),
        },
        force,
    )
}

fn state(id: &str, s: ActiveSetState) -> SupportStateRecord {
    SupportStateRecord::new(id, s)
}

/// The corpus: one-way (engaging, releasing), explicit friction (sticking,
/// sliding, from both seeds), derived-normal friction (k = 1 coupled, sliding
/// both senses and sticking), and a strict-gap two-gap chain (qualified).
fn corpus() -> Vec<(&'static str, NonlinearFrameSolveInput, Option<AssembledForce>)> {
    let mut out = Vec::new();
    for (name, load, seed) in [
        ("one-way pressed", -100.0, ActiveSetState::Inactive),
        ("one-way lifted", 100.0, ActiveSetState::Active),
    ] {
        let (input, force) = cantilever(
            vec![NonlinearSupport::one_way("OW", 1, FrameDof::Uy, ActivationSense::PositiveReaction)],
            vec![state("OW", seed)],
            Vec::new(),
            Vec::new(),
            &[],
            &[(7, load)],
        );
        out.push((name, input, Some(force)));
    }
    for (name, px, seed) in [
        ("explicit sliding from sticking", 40.0, ActiveSetState::Sticking),
        ("explicit sliding from sliding", -40.0, ActiveSetState::Sliding),
        ("explicit sticking", 10.0, ActiveSetState::Sliding),
    ] {
        let (input, force) = cantilever(
            vec![NonlinearSupport::friction("NL-E", 1, FrameDof::Ux, 0.3).unwrap()],
            vec![state("NL-E", seed)],
            vec![FrictionNormalReaction::new("NL-E", 70.0).unwrap()],
            Vec::new(),
            &[],
            &[(6, px)],
        );
        out.push((name, input, Some(force)));
    }
    for (name, px, seed) in [
        ("derived sliding +", 80.0, ActiveSetState::Sliding),
        ("derived sliding -", -80.0, ActiveSetState::Sticking),
        ("derived sticking", 30.0, ActiveSetState::Sliding),
    ] {
        let (input, force) = cantilever(
            vec![NonlinearSupport::friction("NL-F", 1, FrameDof::Ux, 0.5).unwrap()],
            vec![state("NL-F", seed)],
            Vec::new(),
            vec![DerivedFrictionNormalReaction::from_support_reaction("NL-F", 1, FrameDof::Uy, "source").unwrap()],
            &[7],
            &[(7, -100.0), (6, px)],
        );
        out.push((name, input, Some(force)));
    }
    // Two gaps in series, qualified by the exact gap proof (the strict-gap regression's values).
    out.push(("strict gap chain", gap_chain(), None));
    out
}

/// Two members along x, gaps at nodes 1 and 2 on UX closing in +, and a tip load
/// (`strict_gap_27_independent_neighbors_all_senses_seeds_and_backends`' values).
fn gap_chain() -> NonlinearFrameSolveInput {
    let force_value = f64::from_bits(4623226492472524801);
    let gaps = [f64::from_bits(4593671619917905920), f64::from_bits(4598175219545276417)];
    let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let elements = (0..2)
        .map(|i| {
            FrameElement::new(
                FrameNode::new(i, [i as f64, 0.0, 0.0]).unwrap(),
                FrameNode::new(i + 1, [(i + 1) as f64, 0.0, 0.0]).unwrap(),
                section,
                [0.0, 1.0, 0.0],
            )
            .unwrap()
        })
        .collect();
    let mut force = vec![0.0; 18];
    force[node_dof_index(2, FrameDof::Ux)] = force_value;
    let base_restrained_dofs = (0..18).filter(|&d| d < 6 || d % 6 != 0).collect();
    NonlinearFrameSolveInput {
        base_prescribed: Vec::new(),
        ground_motion: Vec::new(),
        node_count: 3,
        elements,
        connectors: Vec::new(),
        curved_bend_elements: Vec::new(),
        force,
        base_restrained_dofs,
        nonlinear_supports: vec![
            NonlinearSupport::gap("G1", 1, FrameDof::Ux, gaps[0], GapDirection::PositiveDisplacement).unwrap(),
            NonlinearSupport::gap("G2", 2, FrameDof::Ux, gaps[1], GapDirection::PositiveDisplacement).unwrap(),
        ],
        initial_states: vec![state("G1", ActiveSetState::Inactive), state("G2", ActiveSetState::Inactive)],
        friction_normal_reactions: Vec::new(),
        derived_friction_normal_reactions: Vec::new(),
        convergence: ConvergenceControl::new("held-test", ConvergencePolicyStatus::Accepted, 0.0, 0.0, 6).unwrap(),
    }
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|v| v.to_bits()).collect()
}

/// The projection of one iterate record that H-10 compares (`iteration` and,
/// when asked, the residual deltas excluded).
fn projection(iteration: &NonlinearFrameIteration, with_residuals: bool) -> String {
    let mut record = iteration.clone();
    record.iteration = 0;
    record.active_set.iteration = 0;
    // Strict-gap evidence: the compared fields (§7.9 H-10); `source_identity` (it names the
    // iteration), `work` and the private proof, limits and work snapshot are excluded.
    let strict_gap = match &record.strict_gap {
        StrictGapEvidence::NotApplicable => "not applicable".to_string(),
        StrictGapEvidence::Qualified(report) => format!(
            "qualified {:?} {:?} {:?} {:?} {:?} {}",
            report.free_dofs,
            report.prescribed.iter().map(|&(dof, value)| (dof, value.to_bits())).collect::<Vec<_>>(),
            report.decisions,
            report.projections,
            bits(&report.ordinary_displacements),
            report.corrected_coordinate_count
        ),
        StrictGapEvidence::Unsupported { reason, .. } => format!("unsupported {reason}"),
    };
    record.strict_gap = StrictGapEvidence::NotApplicable;
    if !with_residuals {
        record.residuals = NonlinearResidualObservation {
            active_set_changed_support_count: 0.0,
            max_abs_translation_delta_from_previous: None,
            max_abs_rotation_delta_from_previous: None,
            max_abs_force_reaction_delta_from_previous: None,
            max_abs_moment_reaction_delta_from_previous: None,
            max_abs_free_dof_force_residual: 0.0,
            max_abs_free_dof_moment_residual: 0.0,
            max_abs_free_dof_work_residual: 0.0,
        };
    }
    format!(
        "{record:?}|{strict_gap}|{:?}|{:?}|{:?}",
        bits(&iteration.displacements),
        bits(&iteration.reactions),
        bits(&iteration.active_prescribed_displacements)
    )
}

fn confirmation_outcome(confirmation: &Confirmation) -> String {
    match confirmation {
        Confirmation::NotEstablished { reason } => format!("not established: {reason}"),
        Confirmation::Established(state) => {
            let mut rows = state
                .supports
                .iter()
                .chain(state.derived_normals.iter())
                .map(|(id, row)| format!("{id}@{}={:016x}/{:016x}", row.global_dof, row.row.published.to_bits(), row.row.radius.to_bits()))
                .collect::<Vec<_>>();
            rows.sort();
            format!("established: {}", rows.join(","))
        }
    }
}

#[test]
fn h10_held_at_its_own_final_status_reproduces_the_loops_final_iterate() {
    let mut compared = 0;
    for (name, input, force) in corpus() {
        for mode in MODES {
            let looped = solve_active_set_frame_with_mode_and_springs(&input, mode, &[]).unwrap();
            assert!(looped.converged, "{name} {mode:?}");
            let n = looped.iterations.len();
            let loop_last = looped.iterations.last().unwrap();
            let loop_qualified = product_selected_state_is_qualified(&input, &[], &looped).unwrap();
            let loop_confirmation = force.as_ref().map(|force| {
                confirmation::confirm_converged_state(&input, &[], &looped, force, &[], &[], mode).unwrap()
            });
            // The penultimate iterate (the loop's own previous iterate) and the final one.
            let orientations: Vec<(Option<&NonlinearFrameIteration>, bool)> =
                vec![(n.checked_sub(2).map(|i| &looped.iterations[i]), true), (Some(loop_last), false)];
            for (orienting, with_residuals) in orientations {
                let held = solve_held_status_iteration(
                    &input,
                    &[],
                    &looped.final_states,
                    orienting,
                    mode,
                    force.as_ref().map(|force| HeldConfirmation { force, curved_sources: &[], frame_members: &[] }),
                )
                .unwrap();
                let what = format!("{name} {mode:?} orienting={} residuals={with_residuals}", orienting.map_or(0, |i| i.iteration));
                let held_last = held.result.iterations.last().unwrap();
                // The history: the orienting iterate first (the source's, not a held iterate), then the held one.
                assert_eq!(held.result.iterations.len(), usize::from(orienting.is_some()) + 1, "{what}");
                if let Some(orienting) = orienting {
                    assert_eq!(&held.result.iterations[0], orienting, "{what}");
                }
                assert_eq!(held_last.iteration, 1, "{what}");
                // A penultimate orientation reproduces the loop's own previous-iterate deltas;
                // the final-iterate orientation measures them from the final iterate (excluded).
                let residuals = with_residuals && (orienting.is_some() || n == 1);
                let (h, l) = (projection(held_last, residuals), projection(loop_last, residuals));
                if h != l {
                    let at = h.bytes().zip(l.bytes()).position(|(a, b)| a != b).unwrap_or(h.len().min(l.len()));
                    let lo = at.saturating_sub(120);
                    panic!("{what}: projections differ at {at}:\nheld: {}\nloop: {}", &h[lo..(at + 120).min(h.len())], &l[lo..(at + 120).min(l.len())]);
                }
                assert_eq!(held.result.converged, looped.converged, "{what}");
                assert_eq!(held.result.final_states, looped.final_states, "{what}");
                assert_eq!(bits(&held.result.displacements), bits(&looped.displacements), "{what}");
                assert_eq!(bits(&held.result.reactions), bits(&looped.reactions), "{what}");
                // Receipt authentication against its own input and history, separately from the verdict.
                assert!(product_completion_matches_source(&held.input, &[], &held.result).unwrap(), "{what}");
                assert_eq!(held.input.initial_states, held.held_states, "{what}");
                // The qualification verdict, compared as a value (false is preserved).
                assert_eq!(
                    product_selected_state_is_qualified(&held.input, &[], &held.result).unwrap(),
                    loop_qualified,
                    "{what}"
                );
                if let Some(expected) = &loop_confirmation {
                    assert_eq!(confirmation_outcome(&held.confirmation), confirmation_outcome(expected), "{what}");
                } else {
                    assert_eq!(confirmation_outcome(&held.confirmation), "not established: confirmation attempt: not run");
                }
                compared += 1;
            }
        }
    }
    assert_eq!(compared, corpus().len() * MODES.len() * 2);
}

#[test]
fn a_held_receipt_is_invalidated_by_a_one_bit_edit_and_never_matches_the_authored_seed() {
    let (input, _) = cantilever(
        vec![NonlinearSupport::one_way("OW", 1, FrameDof::Uy, ActivationSense::PositiveReaction)],
        vec![state("OW", ActiveSetState::Active)],
        Vec::new(),
        Vec::new(),
        &[],
        &[(7, -100.0)],
    );
    for mode in MODES {
        let held = solve_held_status_iteration(&input, &[], &[state("OW", ActiveSetState::Active)], None, mode, None).unwrap();
        assert!(product_completion_matches_source(&held.input, &[], &held.result).unwrap());
        let mut edited = held.result.clone();
        edited.displacements[7] = f64::from_bits(edited.displacements[7].to_bits() ^ 1);
        assert!(!product_completion_matches_source(&held.input, &[], &edited).unwrap());
        // The receipt binds the held input (initial states = held), not the authored seed.
        let mut seeded = input.clone();
        seeded.initial_states = vec![state("OW", ActiveSetState::Inactive)];
        assert!(!product_completion_matches_source(&seeded, &[], &held.result).unwrap());
    }
}

/// Outcome R at NI level (§7.3): a held state the target's classifier changes
/// gives `converged == false`, the classifier's state in `final_states`, the
/// loop's non-converged exit diagnostic, no qualification and no confirmation;
/// a held lift-off pressed onto its support is the same (H-1's shape), and a
/// reversed derived normal keeps Sliding with an inadmissible branch (H-13).
#[test]
fn released_held_states_are_unconverged_with_the_loops_exit_diagnostic() {
    // H-2's shape: the tip one-way held active while the target's load lifts it (tensile).
    let (lifted, lifted_force) = cantilever(
        vec![NonlinearSupport::one_way("OW", 1, FrameDof::Uy, ActivationSense::PositiveReaction)],
        vec![state("OW", ActiveSetState::Inactive)],
        Vec::new(),
        Vec::new(),
        &[],
        &[(7, 100.0)],
    );
    // H-13's shape: held sliding + with beta = +1 from a source pressing the shoe, target lifting it.
    let (pressed, _) = cantilever(
        vec![NonlinearSupport::friction("NL-F", 1, FrameDof::Ux, 0.5).unwrap()],
        vec![state("NL-F", ActiveSetState::Sliding)],
        Vec::new(),
        vec![DerivedFrictionNormalReaction::from_support_reaction("NL-F", 1, FrameDof::Uy, "source").unwrap()],
        &[7],
        &[(7, -100.0), (6, 80.0)],
    );
    let (reversed, reversed_force) = cantilever(
        vec![NonlinearSupport::friction("NL-F", 1, FrameDof::Ux, 0.5).unwrap()],
        vec![state("NL-F", ActiveSetState::Sliding)],
        Vec::new(),
        vec![DerivedFrictionNormalReaction::from_support_reaction("NL-F", 1, FrameDof::Uy, "source").unwrap()],
        &[7],
        &[(7, 100.0), (6, 80.0)],
    );
    for mode in MODES {
        let held = solve_held_status_iteration(
            &lifted,
            &[],
            &[state("OW", ActiveSetState::Active)],
            None,
            mode,
            Some(HeldConfirmation { force: &lifted_force, curved_sources: &[], frame_members: &[] }),
        )
        .unwrap();
        assert!(!held.result.converged);
        assert!(held.result.iterations[0].reactions[7] < 0.0, "a tensile held reaction");
        assert_eq!(held.result.final_states, vec![state("OW", ActiveSetState::Inactive)]);
        assert!(held.result.diagnostics.iter().any(|d| d.code == SolverDiagnosticCode::NonConvergence));
        assert!(!product_selected_state_is_qualified(&held.input, &[], &held.result).unwrap());
        assert_eq!(
            confirmation_outcome(&held.confirmation),
            "not established: confirmation attempt: the converged state is not qualified"
        );

        let source = solve_active_set_frame_with_mode_and_springs(&pressed, mode, &[]).unwrap();
        assert_eq!(source.final_states, vec![state("NL-F", ActiveSetState::Sliding)]);
        assert!(source.reactions[7] > 0.0, "the source presses the shoe (beta = +1)");
        let held = solve_held_status_iteration(
            &reversed,
            &[],
            &source.final_states,
            source.iterations.last(),
            mode,
            Some(HeldConfirmation { force: &reversed_force, curved_sources: &[], frame_members: &[] }),
        )
        .unwrap();
        let last = held.result.iterations.last().unwrap();
        assert!(last.reactions[7] < 0.0, "the target's normal reverses");
        assert_eq!(held.result.final_states, vec![state("NL-F", ActiveSetState::Sliding)], "kept Sliding");
        assert!(!last.contact_branches_admissible, "an inadmissible derived-normal branch");
        assert!(!held.result.converged);
        assert!(held.result.diagnostics.iter().any(|d| d.code == SolverDiagnosticCode::NonConvergence));
    }
}

#[test]
fn held_inputs_are_refused_when_malformed() {
    let (input, _) = cantilever(
        vec![NonlinearSupport::friction("NL-E", 1, FrameDof::Ux, 0.3).unwrap()],
        vec![state("NL-E", ActiveSetState::Sticking)],
        vec![FrictionNormalReaction::new("NL-E", 70.0).unwrap()],
        Vec::new(),
        &[],
        &[(6, 40.0)],
    );
    let mode = LinearSolveMode::SparseInteractive;
    let refused = |held: &[SupportStateRecord], orienting: Option<&NonlinearFrameIteration>| {
        match solve_held_status_iteration(&input, &[], held, orienting, mode, None) {
            Err(NonlinearIntegrationError::InvalidInput { detail }) => detail,
            other => panic!("expected a refusal, got {other:?}"),
        }
    };
    assert!(refused(&[], None).contains("initial active-set state is required"));
    assert!(refused(&[state("NL-E", ActiveSetState::Sticking), state("X", ActiveSetState::Active)], None)
        .contains("unknown nonlinear support"));
    assert!(refused(&[state("NL-E", ActiveSetState::Sticking), state("NL-E", ActiveSetState::Sticking)], None)
        .contains("repeated"));
    assert!(refused(&[state("NL-E", ActiveSetState::Sliding)], None).contains("needs the orienting iterate"));
    let looped = solve_active_set_frame_with_mode_and_springs(&input, mode, &[]).unwrap();
    let mut short = looped.iterations.last().unwrap().clone();
    short.displacements.pop();
    assert!(refused(&[state("NL-E", ActiveSetState::Sliding)], Some(&short)).contains("DOF count"));
}

/// R1 (kd5 R2 as amended): a gap-bearing held input is never confirmed through the wrapper,
/// even when the caller supplies a confirmation and the held state is the strict-gap qualified
/// one; the attempt is "not run", decided beside the attempt (outcome G at the product). A
/// mutant without the wrapper's gap rule reaches the confirmation's own R1 refusal instead and
/// is killed by the exact reason.
#[test]
fn a_held_gap_state_is_never_confirmed_through_the_wrapper() {
    let input = gap_chain();
    let mut ledger = LoadLedger::new();
    for (dof, &value) in input.force.iter().enumerate() {
        if value != 0.0 {
            ledger.push("nodal", dof, value);
        }
    }
    let force = ledger.finish(input.force.len()).unwrap();
    for mode in MODES {
        let looped = solve_active_set_frame_with_mode_and_springs(&input, mode, &[]).unwrap();
        assert!(looped.iterations.iter().all(|i| matches!(i.strict_gap, StrictGapEvidence::Qualified(_))));
        let held = solve_held_status_iteration(
            &input,
            &[],
            &looped.final_states,
            looped.iterations.last(),
            mode,
            Some(HeldConfirmation { force: &force, curved_sources: &[], frame_members: &[] }),
        )
        .unwrap();
        assert!(held.result.converged);
        assert_eq!(confirmation_outcome(&held.confirmation), "not established: confirmation attempt: not run");
    }
}

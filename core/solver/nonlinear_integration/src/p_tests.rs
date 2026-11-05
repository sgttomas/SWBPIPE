//! RT-A3 P (DEL-04-07 `Work/rt-a3-p-scale-repair.md`): the loop on the
//! pattern. The shadow check in `solve_active_set_frame_with_mode_and_springs`
//! compares every sparse-mode loop solve of this crate's tests with today's
//! dense-derived sequence; these tests show it runs and cover the strict-gap
//! view's dimension boundary on both sides.
use crate::*;
use open_pipe_stress_frame_kernel::{FrameNode, FrameSection};
use open_pipe_stress_nonlinear_supports::{GapDirection, NonlinearSupport, SupportStateRecord};
use crate::structural_adapter::SHARED_FACTOR_COUNTS;

/// A chain of `members` unit members along x, anchored at node 0, with every
/// other DOF restrained except UY and RZ, a downward tip load and a gap at the
/// tip's UY closing in -y with clearance 2^-12 m (closed at the solution).
fn gap_chain(members: usize) -> NonlinearFrameSolveInput {
    let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let nodes = members + 1;
    let elements = (0..members)
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
    let mut force = vec![0.0; 6 * nodes];
    force[6 * members + 1] = -1.0;
    let base_restrained_dofs = (0..6 * nodes)
        .filter(|&d| d < 6 || !matches!(d % 6, 1 | 5))
        .collect();
    NonlinearFrameSolveInput {
        base_prescribed: Vec::new(),
        ground_motion: Vec::new(),
        node_count: nodes,
        elements,
        connectors: Vec::new(),
        curved_bend_elements: vec![],
        force,
        base_restrained_dofs,
        nonlinear_supports: vec![NonlinearSupport::gap(
            "tip",
            members,
            FrameDof::Uy,
            2.0_f64.powi(-12),
            GapDirection::NegativeDisplacement,
        )
        .unwrap()],
        initial_states: vec![SupportStateRecord::new("tip", ActiveSetState::Inactive)],
        friction_normal_reactions: vec![],
        derived_friction_normal_reactions: vec![],
        convergence: ConvergenceControl::new(
            "DEC-046-fixture-active-set-count-tightening",
            ConvergencePolicyStatus::Accepted,
            0.0,
            0.0,
            4,
        )
        .unwrap(),
    }
}

fn shadow_checks() -> usize {
    SHADOW_CHECKS.with(|cell| cell.get())
}

/// The sparse mode's loop runs on the pattern, and the shadow comparison runs
/// once per sparse solve (never for dense scrutiny or a forced representation).
#[test]
fn p_sparse_mode_runs_on_the_pattern_and_is_shadowed() {
    assert_eq!(loop_representation(LinearSolveMode::SparseInteractive), LoopRepresentation::Pattern);
    assert_eq!(loop_representation(LinearSolveMode::DenseScrutiny), LoopRepresentation::Dense);
    let input = gap_chain(2);
    let before = shadow_checks();
    let sparse = solve_active_set_frame_with_mode(&input, LinearSolveMode::SparseInteractive).unwrap();
    assert_eq!(shadow_checks(), before + 1);
    let dense = solve_active_set_frame_with_mode(&input, LinearSolveMode::DenseScrutiny).unwrap();
    assert_eq!(shadow_checks(), before + 1);
    assert!(sparse.converged && dense.converged);
    assert!(matches!(
        sparse.completion.as_ref().unwrap().original_stiffness,
        LoopStiffness::Pattern(_)
    ));
    assert!(matches!(
        dense.completion.as_ref().unwrap().original_stiffness,
        LoopStiffness::Dense(_)
    ));
}

/// The strict-gap proof's dimension boundary: at 6 * 42 = 252 DOFs the pattern
/// loop lends the dense views; at 6 * 43 = 258 it lends empty ones, and the
/// exact context refuses on its dimension before reading K, so in both cases
/// the evidence is the dense-derived loop's (the shadow check asserts it).
/// A one-member chain (free block order 2) is Qualified (non-vacuous); a
/// 258-DOF chain is refused on its dimension. If the exact context's limit
/// moved, the 258-DOF shadow comparison would fail.
#[test]
fn p_gap_scrutiny_on_the_pattern_equals_the_dense_loop() {
    let small = solve_active_set_frame_with_mode(&gap_chain(1), LinearSolveMode::SparseInteractive).unwrap();
    assert!(
        small.iterations.iter().all(|i| matches!(i.strict_gap, StrictGapEvidence::Qualified(_))),
        "{:?}",
        small.strict_gap_unqualified_reason()
    );
    for (members, lent) in [(41, true), (42, false)] {
        let input = gap_chain(members);
        let system = LoopSystem::assemble(&input, &[], LoopRepresentation::Pattern).unwrap();
        let views = system.gap_views(&input).unwrap();
        let source = system.gap_source(Some(&views));
        assert_eq!(!source.stiffness.is_empty(), lent, "{members} members");
        assert_eq!(!source.absolute_roundoff.is_empty(), lent, "{members} members");
        let before = shadow_checks();
        let solved = solve_active_set_frame_with_mode(&input, LinearSolveMode::SparseInteractive).unwrap();
        assert_eq!(shadow_checks(), before + 1);
        if !lent {
            let reason = solved.strict_gap_unqualified_reason().unwrap();
            assert!(reason.contains("Budget"), "{reason}");
        }
    }
}

/// A 2 m member inclined at (2, 1, 0) m, anchored at node 0, its tip UY a base
/// restraint (the normal source) and its tip UX a derived-normal friction
/// support (mu = 0.5, seeded sliding) under 80 N along x and 100 N down.
fn inclined_coupled() -> NonlinearFrameSolveInput {
    let section = FrameSection::new(2.0e11, 8.0e10, 3.5e-3, 1.15e-5, 1.15e-5, 2.3e-5).unwrap();
    let element = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [2.0, 1.0, 0.0]).unwrap(),
        section,
        [0.0, 0.0, 1.0],
    )
    .unwrap();
    let mut force = vec![0.0; 12];
    force[6] = 80.0;
    force[7] = -100.0;
    NonlinearFrameSolveInput {
        base_prescribed: Vec::new(),
        ground_motion: Vec::new(),
        node_count: 2,
        elements: vec![element],
        connectors: Vec::new(),
        curved_bend_elements: Vec::new(),
        force,
        base_restrained_dofs: (0..6).chain([7]).collect(),
        nonlinear_supports: vec![NonlinearSupport::friction("NL-F", 1, FrameDof::Ux, 0.5).unwrap()],
        initial_states: vec![SupportStateRecord::new("NL-F", ActiveSetState::Sliding)],
        friction_normal_reactions: vec![],
        derived_friction_normal_reactions: vec![DerivedFrictionNormalReaction::from_support_reaction(
            "NL-F",
            1,
            FrameDof::Uy,
            "source",
        )
        .unwrap()],
        convergence: ConvergenceControl::new("p-test", ConvergencePolicyStatus::Accepted, 0.0, 0.0, 12)
            .unwrap(),
    }
}

/// RT-A3 P step 3: in sparse mode each iterate factors once. An iterate with
/// one derived-normal sliding candidate (k = 1) solves the base, one unit load
/// and the final right-hand side: one factorization and two completions on the
/// kept factor. The first iterate (no friction force yet) is one factorization.
/// Dense scrutiny makes no shared-factor call. The results equal the
/// dense-derived separate solves (the shadow check).
#[test]
fn p_sparse_mode_factors_once_per_iterate() {
    let input = inclined_coupled();
    SHARED_FACTOR_COUNTS.with(|c| c.set((0, 0)));
    let before = shadow_checks();
    let solved = solve_active_set_frame_with_mode(&input, LinearSolveMode::SparseInteractive).unwrap();
    assert!(solved.converged);
    assert_eq!(shadow_checks(), before + 1);
    // The shadow's dense-derived reference makes no shared-factor call either.
    let (factorizations, completions) = SHARED_FACTOR_COUNTS.with(|c| c.get());
    let iterations = solved.iterations.len();
    let coupled = solved
        .iterations
        .iter()
        .filter(|i| !i.applied_sliding_friction_forces.is_empty())
        .count();
    assert!(coupled > 0, "precondition: a coupled iterate");
    assert_eq!(factorizations, iterations, "one factorization per iterate");
    assert_eq!(completions, 2 * coupled, "base, unit and final on one factor");
    SHARED_FACTOR_COUNTS.with(|c| c.set((0, 0)));
    solve_active_set_frame_with_mode(&input, LinearSolveMode::DenseScrutiny).unwrap();
    assert_eq!(SHARED_FACTOR_COUNTS.with(|c| c.get()), (0, 0));
}

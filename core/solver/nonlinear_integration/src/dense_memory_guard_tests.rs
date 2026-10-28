//! §8.30 correction 2 (RT-U4, RT-A2): the active-set loop's dense memory
//! guard. The arithmetic is checked against hand values, overflow refuses
//! instead of wrapping, and the loop entry refuses in both modes before any
//! n^2 allocation when the limit is lowered (test-only override; the product
//! applies `DENSE_LOOP_MEMORY_CEILING_BYTES`).
use crate::*;
use open_pipe_stress_frame_kernel::{FrameNode, FrameSection};
use open_pipe_stress_nonlinear_supports::{
    ActivationSense, ActiveSetState, NonlinearSupport, SupportStateRecord,
};

/// A two-member cantilever along x, anchored at node 0, with a one-way +y
/// support at its tip: 18 global DOFs, 12 free.
fn cantilever() -> NonlinearFrameSolveInput {
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
    force[13] = -10.0;
    NonlinearFrameSolveInput {
        base_prescribed: Vec::new(),
        ground_motion: Vec::new(),
        node_count: 3,
        elements,
        connectors: Vec::new(),
        curved_bend_elements: vec![],
        force,
        base_restrained_dofs: (0..6).collect(),
        nonlinear_supports: vec![NonlinearSupport::one_way(
            "tip",
            2,
            FrameDof::Uy,
            ActivationSense::PositiveReaction,
        )],
        initial_states: vec![SupportStateRecord::new("tip", ActiveSetState::Active)],
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

fn with_ceiling<T>(ceiling: Option<u128>, f: impl FnOnce() -> T) -> T {
    DENSE_LOOP_CEILING_OVERRIDE.with(|cell| cell.set(ceiling));
    let value = f();
    DENSE_LOOP_CEILING_OVERRIDE.with(|cell| cell.set(None));
    value
}

#[test]
fn the_estimate_is_88_n_squared_plus_16_free_squared() {
    // 18 global DOFs, 6 restrained (one repeated, one out of range ignored).
    let restrained = [0, 1, 2, 3, 4, 5, 5, 18];
    assert_eq!(
        dense_loop_memory_estimate(3, &restrained),
        (Some(18), Some(12), Some(88 * 18 * 18 + 16 * 12 * 12))
    );
    // R-4 C-6: 2,000 nodes, every DOF free, is 14,976,000,000 bytes.
    assert_eq!(dense_loop_memory_estimate(2_000, &[]).2, Some(14_976_000_000));
}

#[test]
fn the_provisional_limit_is_nine_tenths_of_twelve_gib() {
    assert_eq!(DENSE_LOOP_MEMORY_CEILING_BYTES, 11_596_411_699);
    assert_eq!(DENSE_LOOP_MEMORY_CEILING_BYTES, 12 * (1u128 << 30) * 9 / 10);
    // A free 1,759-node model passes; 1,760 nodes are refused.
    assert!(dense_loop_memory_guard(1_759, &[], DENSE_LOOP_MEMORY_CEILING_BYTES).is_ok());
    let refusal = dense_loop_memory_guard(1_760, &[], DENSE_LOOP_MEMORY_CEILING_BYTES).unwrap_err();
    assert_eq!(refusal.estimated_bytes, Some(104 * 10_560 * 10_560));
    assert_eq!(refusal.ceiling_bytes, DENSE_LOOP_MEMORY_CEILING_BYTES);
}

#[test]
fn overflow_refuses_instead_of_wrapping() {
    let refusal = dense_loop_memory_guard(usize::MAX, &[], u128::MAX).unwrap_err();
    assert_eq!((refusal.global_dofs, refusal.estimated_bytes), (None, None));
    // 6 * (usize::MAX / 6) DOFs fit usize, but 88 x their square exceeds u128.
    let refusal = dense_loop_memory_guard(usize::MAX / 6, &[], u128::MAX).unwrap_err();
    assert!(refusal.global_dofs.is_some());
    assert_eq!(refusal.estimated_bytes, None);
    assert!(refusal.to_string().contains("more than u128::MAX bytes"));
}

#[test]
fn the_loop_entry_refuses_in_both_modes_below_the_estimate() {
    let input = cantilever();
    let estimate = 88 * 18 * 18 + 16 * 12 * 12;
    for mode in [LinearSolveMode::SparseInteractive, LinearSolveMode::DenseScrutiny] {
        let error = with_ceiling(Some(estimate - 1), || {
            solve_active_set_frame_with_mode_and_springs(&input, mode, &[])
        })
        .unwrap_err();
        let NonlinearIntegrationError::DenseMemoryLimitExceeded(refusal) = &error else {
            panic!("{mode:?}: {error:?}");
        };
        assert_eq!(refusal.estimated_bytes, Some(estimate));
        assert_eq!(refusal.ceiling_bytes, estimate - 1);
        assert_eq!(
            error.to_string(),
            format!("nonlinear dense loop memory guard: estimated leading dense storage {estimate} bytes (88 bytes x 18^2 global DOFs + 16 bytes x 12^2 free DOFs) exceeds the provisional limit {} bytes; the model is refused before any n^2 allocation. The estimate counts the loop's leading dense buffers only and is not a complete memory bound; the limit is provisional until the sparse nonlinear loop (R-4 stage P)", estimate - 1)
        );
        // At the estimate itself the model is under the limit and solves.
        let solved = with_ceiling(Some(estimate), || {
            solve_active_set_frame_with_mode_and_springs(&input, mode, &[])
        });
        assert!(solved.is_ok(), "{mode:?}: {solved:?}");
    }
}

/// Negative: under the product's limit the loop is unchanged.
#[test]
fn a_model_under_the_product_limit_solves() {
    for mode in [LinearSolveMode::SparseInteractive, LinearSolveMode::DenseScrutiny] {
        let solved = solve_active_set_frame_with_mode_and_springs(&cantilever(), mode, &[]).unwrap();
        assert!(solved.converged, "{mode:?}");
    }
}

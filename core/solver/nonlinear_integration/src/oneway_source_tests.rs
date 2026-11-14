//! DEL-04-04 REQ-012: a friction normal derived from a one-way or lift-off
//! support (`derived_friction_normal`, the three sites that read it, the input
//! validation and the held-status refusal).
//!
//! The rule is pinned from both sides:
//! - a source that is not active gives N = +0 whatever its reaction row holds
//!   (at a free DOF that row is the solve's rounded residual, not zero);
//! - an active source gives its bearing-sense reaction s·R, and +0 where that
//!   would pull, never |R|;
//! - a linear source keeps |R|, bit for bit.
use crate::*;
use open_pipe_stress_frame_kernel::{FrameDof, FrameElement, FrameNode, FrameSection};
use open_pipe_stress_nonlinear_supports::{classify_support_state, ActivationSense, GapDirection, SupportStateRecord};
use open_pipe_stress_solver_diagnostics::SolverDiagnosticCode;

const MODES: [LinearSolveMode; 2] = [LinearSolveMode::SparseInteractive, LinearSolveMode::DenseScrutiny];

/// The source of F's normal: the one-way support W on node 1's UY, or a linear
/// (base) restraint there named `fixed_y`.
#[derive(Clone, Copy, PartialEq)]
enum Source {
    OneWay(ActivationSense),
    Linear,
}

/// T4-U3's off-axis chord (3, -4, 0), L = 5, as `coupled_normal_friction_problem`:
/// node 1's UX and UY are coupled (Kxx = 1668/125, Kxy = -624/125,
/// Kyy = 2032/125), so the friction force on UX changes the normal on UY. Node 0
/// is anchored and node 1's UZ and rotations are restrained. F is the friction
/// support on UX, its normal taken from UY.
fn chord(force_x: f64, force_y: f64, mu: f64, source: Source, seed: ActiveSetState, max_iterations: usize) -> NonlinearFrameSolveInput {
    let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
    let node_j = FrameNode::new(1, [3.0, -4.0, 0.0]).unwrap();
    let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let mut force = vec![0.0; 2 * DOF_PER_NODE];
    force[node_dof_index(1, FrameDof::Ux)] = force_x;
    force[node_dof_index(1, FrameDof::Uy)] = force_y;
    let mut base = (0..6).collect::<Vec<_>>();
    base.extend([FrameDof::Uz, FrameDof::Rx, FrameDof::Ry, FrameDof::Rz].map(|dof| node_dof_index(1, dof)));
    let mut supports = vec![NonlinearSupport::friction("F", 1, FrameDof::Ux, mu).unwrap()];
    let mut seeds = vec![SupportStateRecord::new("F", seed)];
    let source_ref = match source {
        Source::OneWay(sense) => {
            supports.push(NonlinearSupport::one_way("W", 1, FrameDof::Uy, sense));
            seeds.push(SupportStateRecord::new("W", ActiveSetState::Active));
            "W"
        }
        Source::Linear => {
            base.push(node_dof_index(1, FrameDof::Uy));
            "fixed_y"
        }
    };
    NonlinearFrameSolveInput {
        base_prescribed: Vec::new(),
        ground_motion: Vec::new(),
        node_count: 2,
        elements: vec![FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap()],
        connectors: Vec::new(),
        curved_bend_elements: Vec::new(),
        force,
        base_restrained_dofs: base,
        nonlinear_supports: supports,
        initial_states: seeds,
        friction_normal_reactions: Vec::new(),
        derived_friction_normal_reactions: vec![DerivedFrictionNormalReaction::from_support_reaction("F", 1, FrameDof::Uy, source_ref).unwrap()],
        convergence: ConvergenceControl::new("oneway-source-test", ConvergencePolicyStatus::Accepted, 0.0, 0.0, max_iterations).unwrap(),
    }
}

fn state_of(states: &[SupportStateRecord], id: &str) -> ActiveSetState {
    states.iter().find(|s| s.support_id == id).unwrap().state
}

fn record(id: &str, state: ActiveSetState) -> SupportStateRecord {
    SupportStateRecord::new(id, state)
}

#[test]
fn the_rule_gives_zero_while_the_source_is_inactive_and_never_a_pulling_reaction() {
    let input = chord(10.0, -10.0, 0.3, Source::OneWay(ActivationSense::PositiveReaction), ActiveSetState::Sticking, 4);
    let source = &input.derived_friction_normal_reactions[0];
    let supports = &input.nonlinear_supports;
    let active = [record("F", ActiveSetState::Sticking), record("W", ActiveSetState::Active)];
    let inactive = [record("F", ActiveSetState::Sticking), record("W", ActiveSetState::Inactive)];
    let bits = |v: f64| v.to_bits();
    // Inactive: +0 whatever the row holds (a residual of either sign).
    for residual in [3.637978807091713e-12, -4.547473508864641e-13, 0.0, -0.0, 7.0] {
        assert_eq!(bits(derived_friction_normal(supports, &inactive, source, residual)), bits(0.0), "{residual:e}");
    }
    // Active: the bearing-sense reaction; a pulling reaction gives +0, not |R|.
    assert_eq!(derived_friction_normal(supports, &active, source, 5.0), 5.0);
    assert_eq!(bits(derived_friction_normal(supports, &active, source, -5.0)), bits(0.0));
    assert_eq!(bits(derived_friction_normal(supports, &active, source, -0.0)), bits(0.0));
    // A negative-sense source bears with a negative reaction.
    let negative = chord(10.0, 10.0, 0.3, Source::OneWay(ActivationSense::NegativeReaction), ActiveSetState::Sticking, 4);
    let n_source = &negative.derived_friction_normal_reactions[0];
    assert_eq!(derived_friction_normal(&negative.nonlinear_supports, &active, n_source, -5.0), 5.0);
    assert_eq!(bits(derived_friction_normal(&negative.nonlinear_supports, &active, n_source, 5.0)), bits(0.0));
    // A linear source keeps |R| exactly, whatever the states.
    let linear = chord(10.0, -10.0, 0.3, Source::Linear, ActiveSetState::Sticking, 4);
    let l_source = &linear.derived_friction_normal_reactions[0];
    for (reaction, expected) in [(5.0, 5.0), (-5.0, 5.0), (3.637978807091713e-12, 3.637978807091713e-12), (-0.0, 0.0)] {
        assert_eq!(bits(derived_friction_normal(&linear.nonlinear_supports, &[record("F", ActiveSetState::Sticking)], l_source, reaction)), bits(expected));
    }
}

#[test]
fn trial_states_read_the_iterates_own_source_state() {
    // An inactive source with a nonzero residual at its DOF gives the friction
    // trial N = +0, so F classifies inactive: no spurious bearing normal.
    let input = chord(10.0, -10.0, 0.3, Source::OneWay(ActivationSense::PositiveReaction), ActiveSetState::Sticking, 4);
    let derived = derived_friction_normal_map(&input).unwrap();
    let explicit = HashMap::new();
    let displacements = vec![0.0; 12];
    let mut reactions = vec![0.0; 12];
    reactions[node_dof_index(1, FrameDof::Uy)] = 3.637978807091713e-12;
    reactions[node_dof_index(1, FrameDof::Ux)] = -1.0e-13;
    let solved_with = [record("F", ActiveSetState::Sticking), record("W", ActiveSetState::Inactive)];
    let trials = build_trial_states(&input.nonlinear_supports, &solved_with, &[], &displacements, &reactions, &explicit, &derived).unwrap();
    let friction_trial = trials.iter().find(|t| t.support_id == "F").unwrap();
    assert_eq!(friction_trial.normal_reaction.unwrap().to_bits(), 0.0_f64.to_bits());
    let friction = &input.nonlinear_supports[0];
    assert_eq!(classify_support_state(friction, friction_trial, Some(ActiveSetState::Sticking)).unwrap(), ActiveSetState::Inactive);
    // An active source whose reaction pulls gives N = +0 too (|R| would be 5).
    reactions[node_dof_index(1, FrameDof::Uy)] = -5.0;
    let solved_with = [record("F", ActiveSetState::Sticking), record("W", ActiveSetState::Active)];
    let trials = build_trial_states(&input.nonlinear_supports, &solved_with, &[], &displacements, &reactions, &explicit, &derived).unwrap();
    let friction_trial = trials.iter().find(|t| t.support_id == "F").unwrap();
    assert_eq!(friction_trial.normal_reaction.unwrap().to_bits(), 0.0_f64.to_bits());
    assert_eq!(classify_support_state(friction, friction_trial, Some(ActiveSetState::Sticking)).unwrap(), ActiveSetState::Inactive);
}

#[test]
fn a_sliding_support_forms_no_force_while_its_source_is_inactive() {
    let input = chord(10.0, -10.0, 0.3, Source::OneWay(ActivationSense::PositiveReaction), ActiveSetState::Sliding, 4);
    let derived = derived_friction_normal_map(&input).unwrap();
    let explicit = HashMap::new();
    let previous = solve_active_set_frame_with_mode(&chord(10.0, -10.0, 0.3, Source::OneWay(ActivationSense::PositiveReaction), ActiveSetState::Sticking, 1), LinearSolveMode::DenseScrutiny)
        .unwrap()
        .iterations
        .pop()
        .unwrap();
    let mut previous = previous;
    previous.displacements[node_dof_index(1, FrameDof::Ux)] = 1.0;
    let sliding_inactive = [record("F", ActiveSetState::Sliding), record("W", ActiveSetState::Inactive)];
    let none = sliding_friction_candidates(&input, &sliding_inactive, Some(&previous), &explicit, &derived).unwrap();
    assert!(none.is_empty(), "no candidate while the source is inactive");
    let sliding_active = [record("F", ActiveSetState::Sliding), record("W", ActiveSetState::Active)];
    let one = sliding_friction_candidates(&input, &sliding_active, Some(&previous), &explicit, &derived).unwrap();
    assert_eq!(one.len(), 1);
    assert!(matches!(one[0].normal_source, SlidingNormalSource::Derived { bearing_sense: Some(sense), .. } if sense == 1.0));
}

#[test]
fn a_bearing_one_way_source_solves_as_the_linear_source_does() {
    // T4-U3's two signed oracles (`current_normal_affine_fixture_matches_both_signed_oracles_seeds_and_modes`):
    // where the one-way bears, the coupled branch is its sense, which is the
    // linear source's observed sign, so the arithmetic and the result are the same.
    for (force_x, force_y) in [(10.0, -10.0), (-10.0, -10.0)] {
        for seed in [ActiveSetState::Sticking, ActiveSetState::Sliding] {
            for mode in MODES {
                let linear = solve_active_set_frame_with_mode(&chord(force_x, force_y, 0.3, Source::Linear, seed, 4), mode).unwrap();
                let one_way = solve_active_set_frame_with_mode(&chord(force_x, force_y, 0.3, Source::OneWay(ActivationSense::PositiveReaction), seed, 4), mode).unwrap();
                let label = format!("{force_x} {force_y} {seed:?} {mode:?}");
                assert!(linear.converged && one_way.converged, "{label}");
                assert_eq!(state_of(&one_way.final_states, "F"), ActiveSetState::Sliding, "{label}");
                assert_eq!(state_of(&one_way.final_states, "W"), ActiveSetState::Active, "{label}");
                assert_eq!(one_way.displacements, linear.displacements, "{label}");
                assert_eq!(one_way.reactions, linear.reactions, "{label}");
                let applied = |r: &NonlinearFrameSolveResult| r.iterations.last().unwrap().applied_sliding_friction_forces.iter().map(|f| f.force.to_bits()).collect::<Vec<_>>();
                assert_eq!(applied(&one_way), applied(&linear), "{label}");
            }
        }
    }
}

#[test]
fn a_lifted_source_releases_the_friction_support_with_no_force() {
    // UY loaded off the seat (+10 against a positive-reaction one-way) with a
    // small lateral load: W lifts, F has N = +0, is inactive and applies nothing.
    for seed in [ActiveSetState::Sticking, ActiveSetState::Sliding] {
        for mode in MODES {
            let result = solve_active_set_frame_with_mode(&chord(1.0, 10.0, 0.3, Source::OneWay(ActivationSense::PositiveReaction), seed, 8), mode).unwrap();
            let label = format!("{seed:?} {mode:?}");
            assert!(result.converged, "{label}");
            assert_eq!(state_of(&result.final_states, "W"), ActiveSetState::Inactive, "{label}");
            assert_eq!(state_of(&result.final_states, "F"), ActiveSetState::Inactive, "{label}");
            assert!(result.iterations.last().unwrap().applied_sliding_friction_forces.is_empty(), "{label}");
        }
    }
}

/// A lift-off/friction cycle (DEL-04-07: a case that does not settle fails by
/// name). Lateral -100 and vertical +3 against a positive-reaction one-way at
/// μ = 0.3: held by stick (u = 0) the shoe pulls (R_y = -3), so W releases and F
/// with it; both free, the lateral motion presses the shoe down (u_y < 0), so W
/// re-engages; with F free and W bearing, F's trial force on its free DOF is 0,
/// so it sticks; and the three repeat. The loop never publishes a state: it ends
/// in its named non-convergence at the cap.
///
/// Not established by the loop: the state (W active, F sliding −) is admissible
/// here by hand (N = 34.41/1.1122 ≈ 30.9 N > 0, u_x ≈ −6.8 < 0, slipping
/// against +μN). The loop's simultaneous update does not reach it; the case
/// fails closed rather than publishing a wrong state.
#[test]
fn a_lift_off_friction_cycle_ends_in_named_non_convergence() {
    for mode in MODES {
        let result = solve_active_set_frame_with_mode(&chord(-100.0, 3.0, 0.3, Source::OneWay(ActivationSense::PositiveReaction), ActiveSetState::Sticking, 16), mode).unwrap();
        assert!(!result.converged, "{mode:?}");
        assert!(result.diagnostics.iter().any(|d| d.code == SolverDiagnosticCode::NonConvergence), "{mode:?}");
        let path = result.iterations.iter().map(|i| (state_of(&i.active_set.states, "F"), state_of(&i.active_set.states, "W"))).collect::<Vec<_>>();
        assert_eq!(path.len(), 16, "{mode:?}");
        assert_eq!(
            &path[..3],
            &[
                (ActiveSetState::Inactive, ActiveSetState::Inactive),
                (ActiveSetState::Inactive, ActiveSetState::Active),
                (ActiveSetState::Sticking, ActiveSetState::Active),
            ],
            "{mode:?}"
        );
        assert!((3..path.len()).all(|k| path[k] == path[k - 3]), "{mode:?}: {path:?}");
    }
}

/// A one-way or lift-off source is named on its own translational DOF. A gap
/// source keeps the rule it had before this change, |R| at its DOF, bit for bit
/// (`s11k_tests::kd5_friction_probe_input` relies on it); the product admits no
/// gap or friction source (`NONLINEAR_FRICTION_NORMAL_SOURCE_INVALID`).
#[test]
fn a_one_way_or_lift_off_source_is_named_on_its_own_translational_dof() {
    let refused = |input: &NonlinearFrameSolveInput| {
        let error = solve_active_set_frame(input).unwrap_err().to_string();
        assert!(error.contains("is a one-way or lift-off support and must be named on its own translational DOF"), "{error}");
    };
    let base = chord(10.0, -10.0, 0.3, Source::OneWay(ActivationSense::PositiveReaction), ActiveSetState::Sticking, 4);
    // The one-way named on a DOF that is not its own.
    let mut elsewhere = base.clone();
    elsewhere.derived_friction_normal_reactions = vec![DerivedFrictionNormalReaction::from_support_reaction("F", 1, FrameDof::Uz, "W").unwrap()];
    refused(&elsewhere);
    // A rotational one-way.
    let mut rotational = base.clone();
    rotational.nonlinear_supports[1] = NonlinearSupport::one_way("W", 1, FrameDof::Rz, ActivationSense::PositiveReaction);
    rotational.base_restrained_dofs.retain(|&d| d != node_dof_index(1, FrameDof::Rz));
    rotational.derived_friction_normal_reactions = vec![DerivedFrictionNormalReaction::from_support_reaction("F", 1, FrameDof::Rz, "W").unwrap()];
    refused(&rotational);
    // A lift-off support is admitted like a one-way.
    let mut lift_off = base.clone();
    lift_off.nonlinear_supports[1] = NonlinearSupport::lift_off("W", 1, FrameDof::Uy, ActivationSense::PositiveReaction);
    let result = solve_active_set_frame(&lift_off).unwrap();
    assert!(result.converged);
    assert_eq!(state_of(&result.final_states, "W"), ActiveSetState::Active);
    // A gap source keeps |R|, whatever its state.
    let mut gap = base;
    gap.nonlinear_supports[1] = NonlinearSupport::gap("W", 1, FrameDof::Uy, 0.0, GapDirection::NegativeDisplacement).unwrap();
    let source = &gap.derived_friction_normal_reactions[0];
    for state in [ActiveSetState::Active, ActiveSetState::Inactive] {
        let states = [record("F", ActiveSetState::Sticking), record("W", state)];
        for reaction in [5.0, -5.0, 3.637978807091713e-12] {
            assert_eq!(derived_friction_normal(&gap.nonlinear_supports, &states, source, reaction).to_bits(), f64::abs(reaction).to_bits());
        }
    }
}

#[test]
fn held_status_refuses_a_one_way_normal_source_by_name() {
    let input = chord(10.0, -10.0, 0.3, Source::OneWay(ActivationSense::PositiveReaction), ActiveSetState::Sticking, 4);
    let solved = solve_active_set_frame_with_mode(&input, LinearSolveMode::DenseScrutiny).unwrap();
    let error = solve_held_status_iteration(&input, &[], &solved.final_states, solved.iterations.last(), LinearSolveMode::DenseScrutiny, None).unwrap_err().to_string();
    assert!(error.contains("held status: support F's friction normal is derived from the one-way or lift-off support W, which has no held-status rule"), "{error}");
}

//! K-D5 on the converged active set (DEL-04-01 kd5-nonlinear-active-set.md
//! §6): the certificate on synthetic balls, the Stage-A ball, the closure
//! mutants and the named preconditions.
use super::*;
use crate::{
    solve_active_set_frame_with_mode_and_springs_assembled, ConvergenceControl,
    ConvergencePolicyStatus, DerivedFrictionNormalReaction, FrictionNormalReaction,
};
use open_pipe_stress_frame_kernel::{FrameElement, FrameNode, FrameSection};
use open_pipe_stress_nonlinear_supports::{GapDirection, SupportStateRecord};

const MODES: [LinearSolveMode; 2] = [LinearSolveMode::SparseInteractive, LinearSolveMode::DenseScrutiny];

fn ball(m: f64, r: f64) -> CertifiedRow {
    CertifiedValue::with_radius(m, r).unwrap().into_row().unwrap()
}

// ------------------------------------------------ the §2.2 predicate

#[test]
fn unilateral_signs_need_the_whole_ball_strictly_on_their_side() {
    let positive = |active, m, r| support_reason(&SupportEvidence::Unilateral { positive: true, active, ball: ball(m, r), ground: 0.0 });
    let negative = |active, m, r| support_reason(&SupportEvidence::Unilateral { positive: false, active, ball: ball(m, r), ground: 0.0 });
    assert_eq!(positive(true, 1.0, 0.5), None);
    // Midpoint of the right sign, radius straddling 0: the loop's binary64
    // value alone would pass (kills the midpoint-only and trial-value mutants).
    assert_eq!(positive(true, 1.0, 2.0), Some("contact force sign not established"));
    assert_eq!(positive(true, 1.0, 1.0), Some("contact force sign not established"));
    // Exact zero (R3): kills `>=` in place of `>`.
    assert_eq!(positive(true, 0.0, 0.0), Some("contact force sign not established"));
    assert_eq!(positive(false, 0.0, 0.0), Some("separation not established"));
    // The sign convention (kills a reversed reaction sign).
    assert_eq!(positive(true, -1.0, 0.0), Some("contact force sign not established"));
    assert_eq!(negative(true, -1.0, 0.5), None);
    assert_eq!(negative(true, 1.0, 0.5), Some("contact force sign not established"));
    assert_eq!(positive(false, 1e-3, 1e-4), None);
    assert_eq!(negative(false, -1e-3, 1e-4), None);
    assert_eq!(negative(false, 1e-3, 1e-4), Some("separation not established"));
}

#[test]
fn stick_capacity_is_decided_exactly_with_both_radii() {
    let explicit = |mu, n, m, r| {
        support_reason(&SupportEvidence::ExplicitFriction {
            coefficient: mu,
            normal: n,
            state: ActiveSetState::Sticking,
            tangential: ball(m, r),
            direction: 0.0,
            ground: 0.0,
        })
    };
    assert_eq!(explicit(0.5, 100.0, 49.9, 0.05), None);
    assert_eq!(explicit(0.5, 100.0, -49.9, 0.05), None);
    assert_eq!(explicit(0.5, 100.0, 49.9, 0.2), Some("stick capacity not established"));
    assert_eq!(explicit(0.5, 100.0, 50.0, 0.0), Some("stick capacity not established"));
    // mu*N is inexact: 0.1*10 = 1 + 5.55e-17 exactly but fl(0.1*10) = 1, so
    // |m_t| = 1 sticks only by the exact product (kills a rounded comparison).
    assert_eq!(0.1_f64 * 10.0, 1.0);
    assert_eq!(explicit(0.1, 10.0, 1.0, 0.0), None);
    assert_eq!(explicit(0.0, 10.0, 0.0, 0.0), Some("stick capacity not established"));
    let derived = |n: CertifiedRow, m, r| {
        support_reason(&SupportEvidence::DerivedFriction {
            coefficient: 0.5,
            state: ActiveSetState::Sticking,
            normal: n,
            tangential: ball(m, r),
            direction: 0.0,
            beta: None,
            ground: 0.0,
        })
    };
    // mu*(|m_n| - rho_n) = 49.5 against |m_t| + rho_t: 49.45 sticks; 49.55
    // does not (kills dropping rho_t), nor does it with rho_n dropped (50).
    assert_eq!(derived(ball(-100.0, 1.0), 49.4, 0.05), None);
    assert_eq!(derived(ball(-100.0, 1.0), 49.4, 0.15), Some("stick capacity not established"));
    assert_eq!(derived(ball(-100.0, 1.0), 49.6, 0.0), Some("stick capacity not established"));
    // |R_n|, not the signed reaction (kills a signed N).
    assert_eq!(derived(ball(100.0, 1.0), 49.4, 0.05), None);
    assert_eq!(derived(ball(0.5, 1.0), 0.0, 0.0), Some("friction normal not established"));
}

#[test]
fn sliding_needs_the_slip_sign_and_the_coupling_branch() {
    let derived = |normal: CertifiedRow, beta, direction, m, r| {
        support_reason(&SupportEvidence::DerivedFriction {
            coefficient: 0.5,
            state: ActiveSetState::Sliding,
            normal,
            tangential: ball(m, r),
            direction,
            beta,
            ground: 0.0,
        })
    };
    assert_eq!(derived(ball(-100.0, 1.0), Some(-1.0), 1.0, 1e-3, 1e-4), None);
    assert_eq!(derived(ball(-100.0, 1.0), Some(-1.0), -1.0, -1e-3, 1e-4), None);
    // The slip sign is checked (kills an unchecked slip sign).
    assert_eq!(derived(ball(-100.0, 1.0), Some(-1.0), 1.0, -1e-3, 1e-4), Some("slip direction not established"));
    assert_eq!(derived(ball(-100.0, 1.0), Some(-1.0), 1.0, 1e-3, 2e-3), Some("slip direction not established"));
    // beta is checked against the certified normal (kills an unchecked beta).
    assert_eq!(derived(ball(-100.0, 1.0), Some(1.0), 1.0, 1e-3, 1e-4), Some("friction normal not established"));
    assert_eq!(derived(ball(0.5, 1.0), Some(1.0), 1.0, 1e-3, 1e-4), Some("friction normal not established"));
    let explicit = |n, direction, m, r| {
        support_reason(&SupportEvidence::ExplicitFriction {
            coefficient: 0.3,
            normal: n,
            state: ActiveSetState::Sliding,
            tangential: ball(m, r),
            direction,
            ground: 0.0,
        })
    };
    assert_eq!(explicit(70.0, 1.0, 1e-3, 1e-4), None);
    assert_eq!(explicit(70.0, 1.0, -1e-3, 1e-4), Some("slip direction not established"));
    assert_eq!(explicit(0.0, 1.0, 1e-3, 1e-4), Some("friction normal not established"));
    let inactive = |n| support_reason(&SupportEvidence::ExplicitFriction { coefficient: 0.3, normal: n, state: ActiveSetState::Inactive, tangential: ball(0.0, 0.0), direction: 0.0, ground: 0.0 });
    assert_eq!(inactive(0.0), None);
    assert_eq!(inactive(-5.0), None);
    assert_eq!(inactive(5.0), Some("friction normal not established"));
    let derived_inactive = support_reason(&SupportEvidence::DerivedFriction { coefficient: 0.5, state: ActiveSetState::Inactive, normal: ball(0.0, 0.0), tangential: ball(0.0, 0.0), direction: 0.0, beta: None, ground: 0.0 });
    assert_eq!(derived_inactive, Some("friction normal not established"));
}

#[test]
fn at_most_six_supports_are_named_in_support_id_order() {
    let failures = (0..8).map(|i| (format!("S{i}"), "separation not established")).collect::<Vec<_>>();
    let reason = certificate_reason(&failures).unwrap();
    assert!(reason.starts_with("support S0: separation not established; support S1"));
    assert!(reason.contains("support S5: separation not established; 2 more"));
    assert!(!reason.contains("S6"));
    assert_eq!(certificate_reason(&[]), None);
}

/// Review M2: a support whose own ball cannot be formed fails under the name
/// of its kind and state (§2.4 item 7), not under one name for every support.
#[test]
fn an_unformed_ball_is_named_by_support_kind_and_state() {
    use NonlinearSupportBehavior as B;
    let one_way = B::OneWay { active_when: ActivationSense::PositiveReaction };
    let lift_off = B::LiftOff { contact_when: ActivationSense::NegativeReaction };
    for unilateral in [&one_way, &lift_off] {
        assert_eq!(ball_failure_reason(unilateral, ActiveSetState::Active), "contact force sign not established");
        assert_eq!(ball_failure_reason(unilateral, ActiveSetState::Inactive), "separation not established");
    }
    assert_eq!(ball_failure_reason(&B::Friction, ActiveSetState::Sticking), "stick capacity not established");
    assert_eq!(ball_failure_reason(&B::Friction, ActiveSetState::Sliding), "slip direction not established");
    assert_eq!(ball_failure_reason(&B::Friction, ActiveSetState::Inactive), "friction normal not established");
    let gap = B::Gap { closes_when: GapDirection::NegativeDisplacement };
    assert_eq!(ball_failure_reason(&gap, ActiveSetState::Active), "gap clearance not established");
}

#[test]
fn prescribed_rows_must_match_bit_for_bit() {
    assert!(!prescribed_row_mismatch(&[0.0, 1.0], &[(0, 0.0)]));
    assert!(prescribed_row_mismatch(&[-0.0, 1.0], &[(0, 0.0)]));
    assert!(prescribed_row_mismatch(&[1e-300, 1.0], &[(0, 0.0)]));
    assert!(prescribed_row_mismatch(&[0.0], &[(3, 0.0)]));
}

// ------------------------------------------------ the Stage-A ball (§1.4)

/// The exact coupled force of a constructed case: φ = c·(N⁰ + g·(φ − φ⁰)).
fn exact_coupled(c: f64, normal: f64, gain: f64, phi0: f64) -> f64 {
    (c * normal - c * gain * phi0) / (1.0 - c * gain)
}

fn contains(b: &CoupledBall, x: f64, slack: f64) -> bool {
    (b.midpoint - x).abs() <= b.radius + slack
}

#[test]
fn coupled_ball_contains_the_coupled_force_far_from_the_loop_value() {
    // c = -dir*mu*beta = 0.5; φ⁰ deliberately far from φ* (−503/9).
    let b = coupled_ball(0.5, 1.0, -1.0, 3.0, &ball(-100.0, 0.0), &ball(0.2, 0.0)).unwrap();
    let exact = -503.0 / 9.0;
    assert!(contains(&b, exact, 1e-13), "{b:?}");
    // Without (1 - c*g)^-1 the midpoint would be c*N⁰ = -50 (killed).
    assert!(!contains(&b, -50.0, 0.0));
    assert!(b.radius < 1e-12);
    // The gain's and the normal's radii widen the ball (kills dropping them):
    // the coupled force of every gain in [0.19, 0.21] and normal in [-101, -99].
    let wide = coupled_ball(0.5, 1.0, -1.0, 3.0, &ball(-100.0, 1.0), &ball(0.2, 0.01)).unwrap();
    for g in [0.19, 0.21] {
        for n in [-101.0, -99.0] {
            assert!(contains(&wide, exact_coupled(0.5, n, g, 3.0), 1e-12), "{wide:?} g={g} n={n}");
        }
    }
    // 0 in M = 1 - c*g fails closed.
    assert!(coupled_ball(0.5, 1.0, -1.0, 3.0, &ball(-100.0, 0.0), &ball(2.0, 0.5)).is_none());
}

// ------------------------------------------------ product-shaped models

/// A 2 m cantilever along X: node 0 anchored, node 1 the tip; `base` adds tip
/// restraints. Nodal loads are input terms of a ledger.
fn cantilever(
    supports: Vec<NonlinearSupport>,
    states: Vec<SupportStateRecord>,
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
            initial_states: states,
            friction_normal_reactions: explicit,
            derived_friction_normal_reactions: derived,
            convergence: ConvergenceControl::new("kd5-nonlinear-test", ConvergencePolicyStatus::Accepted, 0.0, 0.0, 12).unwrap(),
        },
        force,
    )
}

fn derived_friction(id: &str, dof: FrameDof, source: FrameDof, mu: f64) -> (NonlinearSupport, SupportStateRecord, DerivedFrictionNormalReaction) {
    (
        NonlinearSupport::friction(id, 1, dof, mu).unwrap(),
        SupportStateRecord::new(id, ActiveSetState::Sliding),
        DerivedFrictionNormalReaction::from_support_reaction(id, 1, source, "source").unwrap(),
    )
}

fn confirm(input: &NonlinearFrameSolveInput, force: &AssembledForce, mode: LinearSolveMode, applied: Option<&dyn Fn(&mut Vec<AppliedSlidingFrictionForce>)>) -> Confirmation {
    let solve = solve_active_set_frame_with_mode_and_springs_assembled(input, force, mode, &[]).unwrap();
    assert!(solve.converged, "{:?}", solve.diagnostics);
    let mut forces = solve.iterations.last().unwrap().applied_sliding_friction_forces.clone();
    if let Some(mutate) = applied {
        mutate(&mut forces);
    }
    confirm_with_applied(input, &[], &solve, force, &[], &[], mode, &forces).unwrap()
}

fn reason(c: Confirmation) -> String {
    match c {
        Confirmation::NotEstablished { reason } => reason,
        Confirmation::Established(_) => panic!("expected a named reason"),
    }
}

fn f1(px: f64) -> (NonlinearFrameSolveInput, AssembledForce) {
    let (s, st, d) = derived_friction("NL-F", FrameDof::Ux, FrameDof::Uy, 0.5);
    cantilever(vec![s], vec![st], Vec::new(), vec![d], &[7], &[(7, -100.0), (6, px)])
}

#[test]
fn derived_sliding_and_sticking_are_established_in_both_modes() {
    for mode in MODES {
        for (px, sliding) in [(80.0, true), (-80.0, true), (30.0, false)] {
            let (input, force) = f1(px);
            let Confirmation::Established(state) = confirm(&input, &force, mode, None) else {
                panic!("{px} {mode:?}")
            };
            let row = state.supports["NL-F"].row;
            let normal = state.derived_normals["NL-F"].row;
            assert_eq!(normal.published.abs(), 100.0);
            if sliding {
                // F = -sign(u)*mu*N, exactly here (g = 0).
                assert_eq!(row.published, -px.signum() * 50.0);
            } else {
                assert_eq!(row.published, -px);
            }
        }
    }
}

#[test]
fn friction_closure_fails_on_a_missing_duplicate_extra_or_rebound_primitive() {
    let explicit = || {
        cantilever(
            vec![NonlinearSupport::friction("NL-E", 1, FrameDof::Ux, 0.3).unwrap()],
            vec![SupportStateRecord::new("NL-E", ActiveSetState::Sliding)],
            vec![FrictionNormalReaction::new("NL-E", 70.0).unwrap()],
            Vec::new(),
            &[],
            &[(6, 40.0)],
        )
    };
    for mode in MODES {
        let (input, force) = explicit();
        let Confirmation::Established(state) = confirm(&input, &force, mode, None) else { panic!() };
        // The exact product -0.3*70, rounded once (kills a fl(mu*N) re-formation:
        // the bound would then miss |fl - exact|).
        assert_eq!(state.supports["NL-E"].row.published, -(0.3_f64 * 70.0));
        assert!(state.supports["NL-E"].row.bound() > 0.0);
        type Mutation = fn(&mut Vec<AppliedSlidingFrictionForce>);
        let cases: [(Mutation, &str); 4] = [
            (|f| f.clear(), "missing, duplicate or extra summand"),
            (|f| { let d = f[0].clone(); f.push(d) }, "missing, duplicate or extra summand"),
            (|f| { let mut d = f[0].clone(); d.support_id = "NL-OTHER".into(); f.push(d) }, "extra or mismatched-term primitive"),
            (|f| f[0].force = -f[0].force, "wrong operand binding"),
        ];
        for (mutation, expected) in cases {
            let reason = reason(confirm(&input, &force, mode, Some(&mutation)));
            assert!(reason.starts_with("formation check: stage B: load closure:") && reason.contains(expected), "{reason}");
        }
    }
    // A derived term whose applied force is absent cannot form Stage A.
    let (input, force) = f1(80.0);
    let reason = reason(confirm(&input, &force, LinearSolveMode::DenseScrutiny, Some(&|f: &mut Vec<AppliedSlidingFrictionForce>| f.clear())));
    assert_eq!(reason, "formation check: stage A: load closure: NL-F: missing applied force");
}

#[test]
fn named_preconditions_fail_closed_before_the_attempt() {
    // k > 1: two coupled derived-normal sliding supports.
    let (a, sa, da) = derived_friction("NL-FX", FrameDof::Ux, FrameDof::Uy, 0.5);
    let (b, sb, db) = derived_friction("NL-FZ", FrameDof::Uz, FrameDof::Uy, 0.5);
    let (input, force) = cantilever(vec![a, b], vec![sa, sb], Vec::new(), vec![da, db], &[7], &[(7, -100.0), (6, 80.0), (8, 80.0)]);
    assert_eq!(reason(confirm(&input, &force, LinearSolveMode::DenseScrutiny, None)), "coupled friction with 2 derived-normal sliding supports is not supported");
    // A derived source that is not a base restraint (a free DOF).
    let (s, st, d) = derived_friction("NL-F", FrameDof::Ux, FrameDof::Uz, 0.5);
    let (input, force) = cantilever(vec![s], vec![st], Vec::new(), vec![d], &[], &[(8, -100.0), (6, 80.0)]);
    let solve = solve_active_set_frame_with_mode_and_springs_assembled(&input, &force, LinearSolveMode::DenseScrutiny, &[]).unwrap();
    assert!(solve.converged, "{:?}", solve.diagnostics);
    let c = confirm_converged_state(&input, &[], &solve, &force, &[], &[], LinearSolveMode::DenseScrutiny).unwrap();
    let r = reason(c);
    assert_eq!(r, "support NL-F: friction normal source is not a base restraint");
    // A rotational friction support (refused at PP validation; NI is defensive).
    let (input, force) = cantilever(
        vec![NonlinearSupport::friction("NL-R", 1, FrameDof::Rx, 0.3).unwrap()],
        vec![SupportStateRecord::new("NL-R", ActiveSetState::Sliding)],
        vec![FrictionNormalReaction::new("NL-R", 70.0).unwrap()],
        Vec::new(),
        &[],
        &[(9, 40.0)],
    );
    assert_eq!(reason(confirm(&input, &force, LinearSolveMode::DenseScrutiny, None)), "support NL-R: rotational friction has no confirmation");
    // A one-way support on a base-restrained DOF.
    let (input, force) = cantilever(
        vec![NonlinearSupport::one_way("NL-U", 1, FrameDof::Uy, ActivationSense::PositiveReaction)],
        vec![SupportStateRecord::new("NL-U", ActiveSetState::Inactive)],
        Vec::new(),
        Vec::new(),
        &[7],
        &[(8, -100.0)],
    );
    assert_eq!(reason(confirm(&input, &force, LinearSolveMode::DenseScrutiny, None)), "support NL-U: shares its DOF with another restraint");
    // A gap-bearing case keeps its own branch (R1).
    let (input, force) = cantilever(
        vec![NonlinearSupport::gap("NL-G", 1, FrameDof::Uz, 1.0, GapDirection::NegativeDisplacement).unwrap()],
        vec![SupportStateRecord::new("NL-G", ActiveSetState::Inactive)],
        Vec::new(),
        Vec::new(),
        &[],
        &[(8, -1.0)],
    );
    let solve = solve_active_set_frame_with_mode_and_springs_assembled(&input, &force, LinearSolveMode::DenseScrutiny, &[]).unwrap();
    let r = reason(confirm_converged_state(&input, &[], &solve, &force, &[], &[], LinearSolveMode::DenseScrutiny).unwrap());
    assert_eq!(r, "confirmation attempt: gap supports keep their recovery-basis branch");
}

#[test]
fn a_zero_coefficient_sliding_support_carries_no_term_and_is_established() {
    for mode in MODES {
        let (s, st, d) = derived_friction("NL-F", FrameDof::Ux, FrameDof::Uy, 0.0);
        let (input, force) = cantilever(vec![s], vec![st], Vec::new(), vec![d], &[7], &[(7, -100.0), (6, 10.0)]);
        let Confirmation::Established(state) = confirm(&input, &force, mode, None) else { panic!() };
        assert_eq!(state.supports["NL-F"].row.published.to_bits(), 0.0_f64.to_bits());
        assert!(!state.force.load_primitives().iter().any(|p| p.identity.family == SLIDING_FRICTION_FAMILY));
    }
}

/// The K-D5 evaluations judge the loop's own displacement (§1.5): Stage A and
/// Stage B take `solve.displacements`, never the gate's solution, and the
/// influence system is the only unjudged evaluation.
#[test]
fn evaluations_take_the_loops_displacement_and_only_the_influence_is_unjudged() {
    let code = include_str!("confirmation.rs");
    let code = &code[..code.find("#[cfg(test)]").unwrap()];
    assert_eq!(code.matches("let u = &solve.displacements;").count(), 1);
    assert_eq!(code.matches("context.evaluate_at(&stage_b, u, true)").count(), 1);
    assert_eq!(code.matches("context.evaluate_at(stage_a_force, u, true)").count(), 1);
    assert_eq!(code.matches("context.evaluate_at(&unit, &zero, false)").count(), 1);
    assert_eq!(code.matches("evaluate_at(").count(), 3);
    assert!(!code.contains("gate.displacements"));
}

// ------------------------------------------------ W-16 stage 2, R-a
// (DEL-04-03 Design pf-nonlinear-imposed-motion.md §1, §2.4): rigid anchors
// and restraints carry the case's motion as base prescribed values.

/// The 2 m cantilever with a tip one-way prop on UZ (pushing up), a 100 N
/// downward tip load, and the anchor's UZ (global DOF 2) prescribed.
fn moved_prop(base_prescribed: Vec<(usize, f64)>) -> (NonlinearFrameSolveInput, AssembledForce) {
    let (mut input, force) = cantilever(
        vec![NonlinearSupport::one_way("PROP", 1, FrameDof::Uz, open_pipe_stress_nonlinear_supports::ActivationSense::PositiveReaction)],
        vec![SupportStateRecord::new("PROP", ActiveSetState::Active)],
        Vec::new(),
        Vec::new(),
        &[],
        &[(8, -100.0)],
    );
    input.base_prescribed = base_prescribed;
    (input, force)
}

#[test]
fn base_prescribed_values_move_the_anchor_and_the_state_follows() {
    // k = 3EI/L^3 at the tip; the prop lifts off beyond s* = P/k.
    let k = 3.0 * 2.0e11 * 1.15e-5 / 8.0;
    for mode in MODES {
        for (rise, active) in [(5.0e-5, true), (2.0e-4, false)] {
            let (input, force) = moved_prop(vec![(2, rise)]);
            let solve = solve_active_set_frame_with_mode_and_springs_assembled(&input, &force, mode, &[]).unwrap();
            assert!(solve.converged, "{rise} {mode:?}");
            let last = solve.iterations.last().unwrap();
            let at = last.active_restrained_dofs.iter().position(|&d| d == 2).unwrap();
            assert_eq!(last.active_prescribed_displacements[at].to_bits(), rise.to_bits());
            assert_eq!(solve.displacements[2].to_bits(), rise.to_bits(), "the moved anchor carries g exactly");
            let state = solve.final_states[0].state;
            assert_eq!(state == ActiveSetState::Active, active, "{rise} {mode:?}");
            if active {
                let expected = 100.0 - k * rise;
                assert!((solve.reactions[8] - expected).abs() <= 1e-9 * expected, "{} {expected}", solve.reactions[8]);
            } else {
                let expected = rise - 100.0 / k;
                assert!((solve.displacements[8] - expected).abs() <= 1e-9 * expected, "{} {expected}", solve.displacements[8]);
            }
            let Confirmation::Established(_) = confirm(&input, &force, mode, None) else { panic!("{rise} {mode:?}") };
        }
    }
}

#[test]
fn an_authored_zero_base_value_is_the_no_motion_path() {
    for mode in MODES {
        let (plain, force) = moved_prop(Vec::new());
        let (zero, _) = moved_prop(vec![(2, 0.0)]);
        let a = solve_active_set_frame_with_mode_and_springs_assembled(&plain, &force, mode, &[]).unwrap();
        let b = solve_active_set_frame_with_mode_and_springs_assembled(&zero, &force, mode, &[]).unwrap();
        let bits = |v: &[f64]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(&a.displacements), bits(&b.displacements));
        assert_eq!(bits(&a.reactions), bits(&b.reactions));
        assert_eq!(a.iterations, b.iterations);
        assert_eq!(a.final_states, b.final_states);
    }
}

#[test]
fn base_prescribed_entries_are_validated_at_the_loop_boundary() {
    for (entries, detail) in [
        (vec![(8, 1.0e-4)], "is not a base restrained DOF"),
        (vec![(2, 1.0e-4), (2, 1.0e-4)], "is repeated"),
        (vec![(2, f64::NAN)], "base prescribed displacement"),
        (vec![(2, f64::INFINITY)], "base prescribed displacement"),
    ] {
        let (input, force) = moved_prop(entries.clone());
        let error = solve_active_set_frame_with_mode_and_springs_assembled(&input, &force, LinearSolveMode::DenseScrutiny, &[]).unwrap_err();
        assert!(error.to_string().contains(detail), "{entries:?}: {error}");
    }
    let (input, _) = moved_prop(vec![(2, 1.0e-4)]);
    let boundary = crate::active_boundary(input.node_count, &input.base_restrained_dofs, &input.base_prescribed, &input.ground_motion, &input.nonlinear_supports, &input.initial_states).unwrap();
    // Every base DOF takes its case value, or +0 when the case prescribes none.
    let expected: [(usize, f64); 7] = [(0, 0.0), (1, 0.0), (2, 1.0e-4), (3, 0.0), (4, 0.0), (5, 0.0), (8, 0.0)];
    assert_eq!(boundary.dofs, expected.iter().map(|p| p.0).collect::<Vec<_>>());
    assert_eq!(
        boundary.displacements.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        expected.iter().map(|p| p.1.to_bits()).collect::<Vec<_>>()
    );
}

/// §2.4 item 2, revision 2: the intended map, formed independently of the
/// loop's boundary, against the loop's active prescribed map: complete
/// canonical maps (default zeros, coverage, duplicates and bits).
#[test]
fn the_intended_map_binds_the_loops_boundary_bit_for_bit() {
    let rise = 5.0e-5;
    let (input, force) = moved_prop(vec![(2, rise)]);
    let solve = solve_active_set_frame_with_mode_and_springs_assembled(&input, &force, LinearSolveMode::DenseScrutiny, &[]).unwrap();
    let last = solve.iterations.last().unwrap();
    let actual = last.active_restrained_dofs.iter().copied().zip(last.active_prescribed_displacements.iter().copied()).collect::<Vec<_>>();
    let intended = intended_prescribed_map(&input, &solve.final_states).unwrap();
    assert!(!prescribed_map_mismatch(Some(&intended), &actual));
    // A base value off by one unit in the last place.
    let mut shifted = input.clone();
    shifted.base_prescribed = vec![(2, rise.next_up())];
    assert!(prescribed_map_mismatch(intended_prescribed_map(&shifted, &solve.final_states).as_deref(), &actual));
    // A default zero carried as -0.0, a missing default zero, a duplicated
    // entry, an extra entry, and a DOF out of order.
    let mut negative = actual.clone();
    negative[0].1 = -0.0;
    let mut missing = actual.clone();
    missing.remove(0);
    let mut duplicated = actual.clone();
    duplicated.insert(1, actual[0]);
    let mut extra = actual.clone();
    extra.push((11, 0.0));
    let mut swapped = actual.clone();
    swapped.swap(0, 1);
    for mutated in [negative, missing, duplicated, extra, swapped] {
        assert!(prescribed_map_mismatch(Some(&intended), &mutated), "{mutated:?}");
    }
    // An intended map that cannot be formed is a mismatch: a repeated base
    // value, a base value off the base set, or a missing state.
    let mut repeated = input.clone();
    repeated.base_prescribed = vec![(2, rise), (2, rise)];
    let mut off = input.clone();
    off.base_prescribed = vec![(8, rise)];
    assert!(intended_prescribed_map(&repeated, &solve.final_states).is_none());
    assert!(intended_prescribed_map(&off, &solve.final_states).is_none());
    assert!(intended_prescribed_map(&input, &[]).is_none());
    assert!(prescribed_map_mismatch(None, &actual));
}

// ------------------------------------------------ W-16 stage 2, R-b
// (DEL-04-03 Design pf-nonlinear-imposed-motion.md §1-§2): a primitive on a
// nonlinear support's own DOF moves its ground by g_s.

/// The 2 m cantilever with a tip lift-off on UZ (pushing up), a 100 N
/// downward tip load, and the lift-off's ground moved by `ground` (when given).
fn moved_ground(ground: Option<f64>) -> (NonlinearFrameSolveInput, AssembledForce) {
    let (mut input, force) = cantilever(
        vec![NonlinearSupport::lift_off("LIFT", 1, FrameDof::Uz, open_pipe_stress_nonlinear_supports::ActivationSense::PositiveReaction)],
        vec![SupportStateRecord::new("LIFT", ActiveSetState::Active)],
        Vec::new(),
        Vec::new(),
        &[],
        &[(8, -100.0)],
    );
    input.ground_motion = ground.map(|g| vec![("LIFT".to_string(), g)]).unwrap_or_default();
    (input, force)
}

#[test]
fn exact_gap_closing_positions_are_exact_or_refused() {
    let tiny = 2.0_f64.powi(-10);
    assert_eq!(crate::exact_gap_closing_position(2.0 * tiny, -1.0, tiny), Some(tiny));
    // Cancellation is exact: the closing position is zero.
    assert_eq!(crate::exact_gap_closing_position(tiny, -1.0, tiny), Some(0.0));
    // A negative effective clearance is admitted.
    assert_eq!(crate::exact_gap_closing_position(tiny, -1.0, 4.0 * tiny), Some(-3.0 * tiny));
    // 1 m + 1 in is exact as authored numbers, but 1.0 + 0.0254 is not in binary64.
    assert_eq!(crate::exact_gap_closing_position(1.0, 1.0, 0.0254), None);
    // Overflow is refused as a non-finite closing position.
    assert_eq!(crate::exact_gap_closing_position(f64::MAX, 1.0, f64::MAX), None);
    assert_eq!(crate::exact_gap_closing_position(f64::MAX, -1.0, -f64::MAX), None);
}

#[test]
fn ground_motion_entries_are_validated_at_the_loop_boundary() {
    let gap = |c: f64| {
        let (mut input, force) = cantilever(
            vec![NonlinearSupport::gap("GAP", 1, FrameDof::Uz, c, GapDirection::PositiveDisplacement).unwrap()],
            vec![SupportStateRecord::new("GAP", ActiveSetState::Inactive)],
            Vec::new(),
            Vec::new(),
            &[],
            &[(8, 100.0)],
        );
        input.ground_motion = vec![("GAP".to_string(), 0.0254)];
        (input, force)
    };
    let unknown = {
        let (mut input, force) = moved_ground(None);
        input.ground_motion = vec![("NONE".to_string(), 1.0e-4)];
        (input, force)
    };
    let repeated = {
        let (mut input, force) = moved_ground(None);
        input.ground_motion = vec![("LIFT".to_string(), 1.0e-4), ("LIFT".to_string(), 1.0e-4)];
        (input, force)
    };
    for ((input, force), detail) in [
        (unknown, "unknown nonlinear support"),
        (repeated, "is repeated"),
        (moved_ground(Some(f64::NAN)), "ground motion must be finite"),
        (gap(1.0), "closing position c + sigma*g_s is not finite and exact"),
    ] {
        let error = solve_active_set_frame_with_mode_and_springs_assembled(&input, &force, LinearSolveMode::DenseScrutiny, &[]).unwrap_err();
        assert!(error.to_string().contains(detail), "{detail}: {error}");
    }
    // The exact operand of a dyadic clearance and motion is admitted.
    let (exact, force) = gap(0.0254);
    let mut exact = exact;
    exact.ground_motion = vec![("GAP".to_string(), 0.0254)];
    assert!(solve_active_set_frame_with_mode_and_springs_assembled(&exact, &force, LinearSolveMode::DenseScrutiny, &[]).is_ok());
}

#[test]
fn a_moved_ground_is_measured_relative_and_prescribed_when_active() {
    // k = 3EI/L^3 at the tip; free tip deflection d* = P/k.
    let k = 3.0 * 2.0e11 * 1.15e-5 / 8.0;
    let star = 100.0 / k;
    for mode in MODES {
        for (drop, active) in [(0.5 * star, true), (2.0 * star, false)] {
            let (input, force) = moved_ground(Some(-drop));
            let solve = solve_active_set_frame_with_mode_and_springs_assembled(&input, &force, mode, &[]).unwrap();
            assert!(solve.converged, "{drop} {mode:?}");
            assert_eq!(solve.final_states[0].state == ActiveSetState::Active, active, "{drop} {mode:?}");
            if active {
                // An active lift-off prescribes its moved ground.
                assert_eq!(solve.displacements[8].to_bits(), (-drop).to_bits());
                let expected = 100.0 - k * drop;
                assert!((solve.reactions[8] - expected).abs() <= 1e-9 * expected);
            } else {
                // Inactive: u = -d*, below the absolute zero but above the moved ground.
                assert!(solve.displacements[8] < 0.0 && solve.displacements[8] > -drop);
                assert!((solve.displacements[8] + star).abs() <= 1e-9 * star);
            }
            let Confirmation::Established(state) = confirm(&input, &force, mode, None) else { panic!("{drop} {mode:?}") };
            if !active {
                assert_eq!(state.supports["LIFT"].row.published.to_bits(), 0.0_f64.to_bits());
            }
        }
    }
}

#[test]
fn the_intended_map_carries_the_moved_ground() {
    let k = 3.0 * 2.0e11 * 1.15e-5 / 8.0;
    let drop = 0.5 * 100.0 / k;
    let (input, force) = moved_ground(Some(-drop));
    let solve = solve_active_set_frame_with_mode_and_springs_assembled(&input, &force, LinearSolveMode::DenseScrutiny, &[]).unwrap();
    let last = solve.iterations.last().unwrap();
    let actual = last.active_restrained_dofs.iter().copied().zip(last.active_prescribed_displacements.iter().copied()).collect::<Vec<_>>();
    let intended = intended_prescribed_map(&input, &solve.final_states).unwrap();
    assert!(intended.contains(&(8, -drop)));
    assert!(!prescribed_map_mismatch(Some(&intended), &actual));
    let mut shifted = input.clone();
    shifted.ground_motion = vec![("LIFT".to_string(), (-drop).next_up())];
    assert!(prescribed_map_mismatch(intended_prescribed_map(&shifted, &solve.final_states).as_deref(), &actual));
    let mut fixed = input.clone();
    fixed.ground_motion = Vec::new();
    assert!(prescribed_map_mismatch(intended_prescribed_map(&fixed, &solve.final_states).as_deref(), &actual));
    let mut unknown = input.clone();
    unknown.ground_motion = vec![("NONE".to_string(), 0.0)];
    assert!(intended_prescribed_map(&unknown, &solve.final_states).is_none());
}

/// §2.2, revision 2: the inactive unilateral and the slip conditions compare
/// the displacement ball with the ground g_s, exactly.
#[test]
fn shifted_conditions_compare_with_the_ground_exactly() {
    let separated = |ground: f64| support_reason(&SupportEvidence::Unilateral { positive: true, active: false, ball: ball(1.0, 0.25), ground });
    assert_eq!(separated(0.0), None);
    assert_eq!(separated(0.5), None);
    // The ball's lower end touches the ground: weakly separated, not established.
    assert_eq!(separated(0.75), Some("separation not established"));
    assert_eq!(separated(0.9), Some("separation not established"));
    // An active support is decided on its reaction against 0, whatever its ground.
    assert_eq!(support_reason(&SupportEvidence::Unilateral { positive: true, active: true, ball: ball(1.0, 0.25), ground: 5.0 }), None);
    let slip = |direction: f64, ground: f64| support_reason(&SupportEvidence::ExplicitFriction {
        coefficient: 0.5, normal: 10.0, state: ActiveSetState::Sliding, tangential: ball(1.0, 0.25), direction, ground });
    assert_eq!(slip(1.0, 0.5), None);
    assert_eq!(slip(1.0, 0.75), Some("slip direction not established"));
    assert_eq!(slip(-1.0, 1.5), None);
    assert_eq!(slip(-1.0, 1.25), Some("slip direction not established"));
}

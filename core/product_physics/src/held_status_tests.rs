//! RT-A5 `minuend_final` (DEL-05-02 Work `operating-case.md` §7.4, §7.6, §7.9 H-8): the
//! binding's nine checks in order, each refused by its own name (and each mutant of a check
//! killed), and the evidence's outcomes on real held solves.
use super::*;
use crate::{NonlinearQualification, NonlinearStateEvidence, NonlinearStateStanding, PreviewSolverMode};
use open_pipe_stress_frame_kernel::{node_dof_index, FrameDof, FrameElement, FrameNode, FrameSection};
use open_pipe_stress_nonlinear_integration::{
    solve_active_set_frame_with_mode_and_springs, solve_held_status_iteration, ConvergenceControl,
    ConvergencePolicyStatus, DerivedFrictionNormalReaction, FrictionNormalReaction, NonlinearFrameSolveInput,
};
use open_pipe_stress_nonlinear_supports::{ActivationSense, ActiveSetState, NonlinearSupport, SupportStateRecord};

const MODE: PreviewSolverMode = PreviewSolverMode::SparseInteractive;

/// A 2 m cantilever along X; node 1 the tip; `base` adds tip restraints; `loads` are nodal.
fn cantilever(supports: Vec<NonlinearSupport>, base: &[usize], loads: &[(usize, f64)]) -> NonlinearFrameSolveInput {
    let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
    let node_j = FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap();
    let section = FrameSection::new(2.0e11, 8.0e10, 3.5e-3, 1.15e-5, 1.15e-5, 2.3e-5).unwrap();
    let mut force = vec![0.0; 12];
    for &(dof, value) in loads {
        force[dof] += value;
    }
    let mut base_restrained = (0..6).collect::<Vec<_>>();
    base_restrained.extend_from_slice(base);
    let seeds = supports.iter().map(|s| SupportStateRecord::new(&s.support_id, ActiveSetState::Inactive)).collect();
    NonlinearFrameSolveInput {
        base_prescribed: Vec::new(),
        ground_motion: Vec::new(),
        node_count: 2,
        elements: vec![FrameElement::new(node_i, node_j, section, [0.0, 0.0, 1.0]).unwrap()],
        connectors: Vec::new(),
        curved_bend_elements: Vec::new(),
        force,
        base_restrained_dofs: base_restrained,
        nonlinear_supports: supports,
        initial_states: seeds,
        friction_normal_reactions: Vec::new(),
        derived_friction_normal_reactions: Vec::new(),
        convergence: ConvergenceControl::new("held-status-test", ConvergencePolicyStatus::Accepted, 0.0, 0.0, 12).unwrap(),
    }
}

fn explicit_shoe(px: f64) -> NonlinearFrameSolveInput {
    let mut input = cantilever(vec![NonlinearSupport::friction("NL-E", 1, FrameDof::Ux, 0.3).unwrap()], &[], &[(6, px)]);
    input.friction_normal_reactions = vec![FrictionNormalReaction::new("NL-E", 70.0).unwrap()];
    input
}

fn derived_shoe(py: f64, px: f64) -> NonlinearFrameSolveInput {
    let mut input = cantilever(vec![NonlinearSupport::friction("NL-F", 1, FrameDof::Ux, 0.5).unwrap()], &[7], &[(7, py), (6, px)]);
    input.derived_friction_normal_reactions =
        vec![DerivedFrictionNormalReaction::from_support_reaction("NL-F", 1, FrameDof::Uy, "source").unwrap()];
    input
}

fn node_ids() -> Vec<String> {
    vec!["node:A".to_string(), "node:T".to_string()]
}

/// A source case solved and captured as the product captures it.
fn source(input: &NonlinearFrameSolveInput, invocation: InvocationToken) -> (NonlinearStateEvidence, Box<HeldStatusSource>) {
    let solve = solve_active_set_frame_with_mode_and_springs(input, MODE.nonlinear_mode(), &[]).unwrap();
    assert!(solve.converged);
    let evidence = NonlinearStateEvidence {
        case_id: "case:O".to_string(),
        solver_mode: MODE,
        converged: true,
        qualification: NonlinearQualification::Qualified,
        standing: NonlinearStateStanding::NotEstablished { reason: "test".to_string() },
        final_states: solve.final_states.clone(),
        motion: crate::CaseMotionEvidence::default(),
    };
    let captured = capture_held_status_source(invocation, true, Some(&evidence), input, &[], Some(&solve), &node_ids()).unwrap();
    (evidence, captured)
}

fn context<'a>(
    invocation: InvocationToken,
    evidence: Option<&'a NonlinearStateEvidence>,
    target: &'a NonlinearFrameSolveInput,
    ids: &'a [String],
) -> BindingContext<'a> {
    BindingContext {
        invocation,
        minuend_id: "case:O",
        minuend_evidence: evidence,
        solver_mode: MODE,
        target_input: target,
        target_springs: &[],
        target_node_ids: ids,
    }
}

fn code(result: Result<Bound<'_>, HeldStatusRefusal>) -> (&'static str, String) {
    match result {
        Err(refusal) => (refusal.code, refusal.message),
        Ok(_) => ("bound", String::new()),
    }
}

#[test]
fn capture_needs_the_combination_and_the_cases_evidence() {
    let token = InvocationToken::mint();
    let input = explicit_shoe(40.0);
    let solve = solve_active_set_frame_with_mode_and_springs(&input, MODE.nonlinear_mode(), &[]).unwrap();
    let (evidence, _) = source(&input, token);
    assert!(capture_held_status_source(token, false, Some(&evidence), &input, &[], Some(&solve), &node_ids()).is_none());
    assert!(capture_held_status_source(token, true, None, &input, &[], Some(&solve), &node_ids()).is_none());
    assert!(capture_held_status_source(token, true, Some(&evidence), &input, &[], None, &node_ids()).is_none());
    assert_ne!(InvocationToken::mint(), InvocationToken::mint());
}

/// H-8: each of §7.4's checks refuses by its own name, in order, and binding passes otherwise.
#[test]
fn h8_every_binding_check_refuses_by_name_in_order() {
    let token = InvocationToken::mint();
    let source_input = explicit_shoe(40.0);
    let target = explicit_shoe(10.0);
    let ids = node_ids();
    let (evidence, captured) = source(&source_input, token);
    let bound = bind(Some(&captured), &context(token, Some(&evidence), &target, &ids)).unwrap();
    assert_eq!(bound.held, evidence.final_states.as_slice());
    assert_eq!(bound.orienting, captured.solve.iterations.last().unwrap());

    // 1. Presence.
    assert_eq!(code(bind(None, &context(token, Some(&evidence), &target, &ids))).0, SOURCE_UNAVAILABLE);
    assert_eq!(code(bind(Some(&captured), &context(token, None, &target, &ids))).0, SOURCE_UNAVAILABLE);
    // 2. Qualification.
    let mut unqualified = source(&source_input, token).1;
    unqualified.evidence.qualification = NonlinearQualification::InspectableUnqualifiedGap;
    assert_eq!(code(bind(Some(&unqualified), &context(token, Some(&evidence), &target, &ids))).0, SOURCE_UNQUALIFIED);
    // 3. Another invocation's source, built identically.
    let (other_evidence, other) = source(&source_input, InvocationToken::mint());
    assert_eq!(other_evidence, evidence);
    let (c, m) = code(bind(Some(&other), &context(token, Some(&evidence), &target, &ids)));
    assert_eq!(c, BINDING);
    assert!(m.contains("another invocation"), "{m}");
    // 4. The evidence, its case and its mode.
    let mut edited = evidence.clone();
    edited.standing = NonlinearStateStanding::Confirmed;
    assert!(code(bind(Some(&captured), &context(token, Some(&edited), &target, &ids))).1.contains("source evidence"));
    let mut renamed = source(&source_input, token);
    renamed.1.evidence.case_id = "case:X".to_string();
    renamed.0.case_id = "case:X".to_string();
    assert!(code(bind(Some(&renamed.1), &context(token, Some(&renamed.0), &target, &ids))).1.contains("is not the minuend case"));
    let mut dense = source(&source_input, token);
    dense.1.evidence.solver_mode = PreviewSolverMode::DenseScrutiny;
    dense.0.solver_mode = PreviewSolverMode::DenseScrutiny;
    assert!(code(bind(Some(&dense.1), &context(token, Some(&dense.0), &target, &ids))).1.contains("source mode"));
    // 5. One bit of the source's displacements or reactions edited: the receipt fails.
    for vector in ["displacements", "reactions"] {
        let mut forged = source(&source_input, token).1;
        let values = if vector == "displacements" { &mut forged.solve.displacements } else { &mut forged.solve.reactions };
        values[6] = f64::from_bits(values[6].to_bits() ^ 1);
        assert!(code(bind(Some(&forged), &context(token, Some(&evidence), &target, &ids))).1.contains("not receipt-bound"), "{vector}");
    }
    // 6. Each model-context member, the springs and the node ids.
    let mutations: Vec<(&str, Box<dyn Fn(&mut NonlinearFrameSolveInput)>)> = vec![
        ("elements", Box::new(|t| t.elements.clear())),
        ("base_restrained_dofs", Box::new(|t| t.base_restrained_dofs.push(8))),
        ("nonlinear_supports", Box::new(|t| t.nonlinear_supports[0].node_index = 0)),
        ("nonlinear_supports", Box::new(|t| t.nonlinear_supports[0].friction_coefficient = Some(f64::from_bits(0.3f64.to_bits() + 1)))),
        ("friction_normal_reactions", Box::new(|t| t.friction_normal_reactions[0].normal_reaction = 71.0)),
        ("derived_friction_normal_reactions", Box::new(|t| t.derived_friction_normal_reactions.push(
            DerivedFrictionNormalReaction::from_support_reaction("X", 1, FrameDof::Uy, "s").unwrap()))),
        ("convergence", Box::new(|t| t.convergence = ConvergenceControl::new("other", ConvergencePolicyStatus::Accepted, 0.0, 0.0, 12).unwrap())),
    ];
    for (member, mutate) in mutations {
        let mut changed = target.clone();
        mutate(&mut changed);
        let (c, m) = code(bind(Some(&captured), &context(token, Some(&evidence), &changed, &ids)));
        assert_eq!(c, BINDING, "{member}");
        assert!(m.ends_with(member), "{member}: {m}");
    }
    let mut spring_context = context(token, Some(&evidence), &target, &ids);
    let springs = [(7usize, 1.0f64)];
    spring_context.target_springs = &springs;
    assert!(code(bind(Some(&captured), &spring_context)).1.ends_with("springs"));
    let other_ids = vec!["node:A".to_string(), "node:Z".to_string()];
    assert!(code(bind(Some(&captured), &context(token, Some(&evidence), &target, &other_ids))).1.ends_with("node ids"));
    // 7. A state outside its behaviour, a repeated record, a missing support.
    for (states, needle) in [
        (vec![SupportStateRecord::new("NL-E", ActiveSetState::Active)], "is not a state of its behaviour"),
        (vec![SupportStateRecord::new("NL-E", ActiveSetState::Sliding), SupportStateRecord::new("NL-E", ActiveSetState::Sliding)], "repeated"),
        (vec![], "missing"),
        (vec![SupportStateRecord::new("NL-X", ActiveSetState::Sliding)], "not in the target"),
    ] {
        let mut forged = source(&source_input, token).1;
        forged.evidence.final_states = states.clone();
        forged.solve.final_states = states.clone();
        let mut minuend = evidence.clone();
        minuend.final_states = states;
        // Check 7 is defensive: a receipt-bound result's final states are the loop's own, one per
        // support of a source whose supports equal the target's (check 6), so every such shape is
        // refused first by check 5, by name.
        let (c, m) = code(bind(Some(&forged), &context(token, Some(&minuend), &target, &ids)));
        assert_eq!(c, BINDING, "{needle}");
        assert!(m.contains("not receipt-bound"), "{needle}: {m}");
    }
    // 9. A sliding ground that differs between the cases.
    let mut moved = target.clone();
    moved.ground_motion = vec![("NL-E".to_string(), 1.0e-3)];
    assert_eq!(code(bind(Some(&captured), &context(token, Some(&evidence), &moved, &ids))).0, SLIDING_GROUND_DIFFERS);
}

/// outcomes on real held solves: kept and established (E), released by a
/// tensile held reaction (R), and released by a reversed derived normal (R, Sliding kept).
#[test]
fn derived_evidence_states_each_outcome() {
    let ids = node_ids();
    // Kept: the explicit shoe slides + in both cases.
    let source_input = explicit_shoe(40.0);
    let o = solve_active_set_frame_with_mode_and_springs(&source_input, MODE.nonlinear_mode(), &[]).unwrap();
    let target = explicit_shoe(30.0);
    let force = {
        let mut ledger = open_pipe_stress_frame_kernel::load_ledger::LoadLedger::new();
        ledger.push("nodal", 6, 30.0);
        ledger.finish(12).unwrap()
    };
    let held = solve_held_status_iteration(
        &target,
        &[],
        &o.final_states,
        o.iterations.last(),
        MODE.nonlinear_mode(),
        Some(open_pipe_stress_nonlinear_integration::HeldConfirmation { force: &force, curved_sources: &[], frame_members: &[] }),
    )
    .unwrap();
    let evidence = derive(&held, o.iterations.last().unwrap(), &ids);
    assert_eq!(evidence.outcome, Outcome::Established, "{:?}", held.confirmation);
    assert_eq!(evidence.standing, ESTABLISHED);
    assert!(evidence.released.is_empty());
    let conditions = evidence.conditions.unwrap();
    assert_eq!(conditions[0]["condition"], "slip_explicit");
    assert_eq!(conditions[0]["class"], "strict_ball");
    assert_eq!(evidence.supports[0]["inputs"]["slip_sense"], 1);

    // R: the one-way prop held active while the target's load lifts the tip (a tensile reaction).
    let prop = |py: f64| cantilever(vec![NonlinearSupport::one_way("OW", 1, FrameDof::Uy, ActivationSense::PositiveReaction)], &[], &[(7, py)]);
    let o = solve_active_set_frame_with_mode_and_springs(&prop(-100.0), MODE.nonlinear_mode(), &[]).unwrap();
    assert_eq!(o.final_states[0].state, ActiveSetState::Active);
    let held = solve_held_status_iteration(&prop(100.0), &[], &o.final_states, o.iterations.last(), MODE.nonlinear_mode(), None).unwrap();
    let evidence = derive(&held, o.iterations.last().unwrap(), &ids);
    assert_eq!(evidence.outcome, Outcome::Released);
    assert_eq!(evidence.outcome.kind(), "not_established");
    assert_eq!(evidence.released[0].reason, "changed_state");
    // the recorded reaction bit token that decided it (-100.0 N: the tensile reaction), never a
    // formatted binary64.
    assert_eq!(evidence.released[0].quantity, "reaction_bits c059000000000000");
    assert_eq!(evidence.supports[0]["inputs"]["reaction_bits"], "c059000000000000");
    assert!(evidence.conditions.is_none());
    assert_eq!(evidence.supports[0]["released"], "changed_state");
    assert!(released_message("comb:held", &evidence.released).contains("OW held active, classified inactive (changed state: reaction_bits c059000000000000)"));

    // R by branch: a source pressing the shoe (beta = +1), a target lifting it; Sliding is kept.
    let o = solve_active_set_frame_with_mode_and_springs(&derived_shoe(-100.0, 80.0), MODE.nonlinear_mode(), &[]).unwrap();
    let held = solve_held_status_iteration(&derived_shoe(100.0, 80.0), &[], &o.final_states, o.iterations.last(), MODE.nonlinear_mode(), None).unwrap();
    let evidence = derive(&held, o.iterations.last().unwrap(), &ids);
    let shoe = &evidence.supports[0];
    assert_eq!(shoe["held_state"], "sliding");
    assert_eq!(shoe["classified_state"], "sliding");
    assert_eq!(shoe["released"], "branch");
    assert_eq!(shoe["inputs"]["beta"], 1);
    assert_eq!(shoe["inputs"]["branch_admissible"], false);
    assert_eq!(evidence.outcome, Outcome::Released);
    let normal = f64::from_bits(u64::from_str_radix(shoe["inputs"]["normal_signed_bits"].as_str().unwrap(), 16).unwrap());
    assert!(normal < 0.0);
    assert_eq!(node_dof_index(1, FrameDof::Uy), 7);
}

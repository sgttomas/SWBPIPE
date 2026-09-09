use open_pipe_stress_frame_kernel::{
    node_dof_index, FrameDof, FrameElement, FrameNode, FrameSection, DOF_PER_NODE,
};
use open_pipe_stress_nonlinear_integration::{
    solve_active_set_frame, ConvergenceControl, ConvergencePolicyStatus, FrictionNormalReaction,
    NonlinearFrameSolveInput,
};
use open_pipe_stress_nonlinear_supports::{
    ActiveSetState, NonlinearSupport, SupportStateRecord,
};

const SUPPORT_ID: &str = "NS-01-FRICTION";

fn input(seed: ActiveSetState) -> NonlinearFrameSolveInput {
    let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
    let node_j = FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap();
    let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let element = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap();
    let mut force = vec![0.0; 2 * DOF_PER_NODE];
    force[node_dof_index(1, FrameDof::Ux)] = 1.0;

    NonlinearFrameSolveInput {
        node_count: 2,
        elements: vec![element],
        user_stiffness_elements: Vec::new(),
        curved_bend_elements: Vec::new(),
        force,
        base_restrained_dofs: vec![
            node_dof_index(0, FrameDof::Ux),
            node_dof_index(0, FrameDof::Uy),
            node_dof_index(0, FrameDof::Uz),
            node_dof_index(0, FrameDof::Rx),
            node_dof_index(0, FrameDof::Ry),
            node_dof_index(0, FrameDof::Rz),
            node_dof_index(1, FrameDof::Uy),
            node_dof_index(1, FrameDof::Uz),
            node_dof_index(1, FrameDof::Rx),
            node_dof_index(1, FrameDof::Ry),
            node_dof_index(1, FrameDof::Rz),
        ],
        nonlinear_supports: vec![
            NonlinearSupport::friction(SUPPORT_ID, 1, FrameDof::Ux, 0.30).unwrap(),
        ],
        initial_states: vec![SupportStateRecord::new(SUPPORT_ID, seed)],
        friction_normal_reactions: vec![FrictionNormalReaction::new(SUPPORT_ID, 10.0).unwrap()],
        derived_friction_normal_reactions: Vec::new(),
        convergence: ConvergenceControl::new(
            "NS-01-runtime-probe",
            ConvergencePolicyStatus::Accepted,
            0.0,
            0.0,
            4,
        )
        .unwrap(),
    }
}

fn run(seed: ActiveSetState) {
    let result = solve_active_set_frame(&input(seed)).unwrap();
    let ux = node_dof_index(1, FrameDof::Ux);
    let final_iteration = result.iterations.last().unwrap();
    let applied_friction = final_iteration
        .applied_sliding_friction_forces
        .first()
        .map(|item| item.force);
    println!(
        "seed={} converged={} iterations={} final_state={} displacement_ux={:.17} reported_friction_reaction={:.17} applied_sliding_friction={} friction_times_displacement={:.17}",
        seed.as_str(),
        result.converged,
        result.iterations.len(),
        result.final_states[0].state.as_str(),
        result.displacements[ux],
        result.reactions[ux],
        applied_friction
            .map(|value| format!("{value:.17}"))
            .unwrap_or_else(|| "none".to_string()),
        result.reactions[ux] * result.displacements[ux],
    );
    for iteration in &result.iterations {
        let applied = iteration
            .applied_sliding_friction_forces
            .first()
            .map(|item| format!("{:.17}", item.force))
            .unwrap_or_else(|| "none".to_string());
        println!(
            "  iteration={} state={} displacement_ux={:.17} reported_reaction={:.17} applied_sliding_friction={}",
            iteration.iteration,
            iteration.active_set.states[0].state.as_str(),
            iteration.displacements[ux],
            iteration.reactions[ux],
            applied,
        );
    }
}

fn main() {
    println!("NS-01 current-SHA runtime probe: k=100 F=+1 mu=0.3 N=10 muN=3");
    run(ActiveSetState::Sticking);
    run(ActiveSetState::Sliding);
}

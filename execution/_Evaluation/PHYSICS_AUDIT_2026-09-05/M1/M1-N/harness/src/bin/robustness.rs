use open_pipe_stress_frame_kernel::*;
use open_pipe_stress_nonlinear_integration::*;
use open_pipe_stress_nonlinear_supports::*;
fn base(s:NonlinearSupport, seed:ActiveSetState,f:f64)->NonlinearFrameSolveInput {
 let e=FrameElement::new(FrameNode::new(0,[0.,0.,0.]).unwrap(),FrameNode::new(1,[1.,0.,0.]).unwrap(),FrameSection::new(100.,40.,1.,1.,1.,1.).unwrap(),[0.,1.,0.]).unwrap();
 let mut force=vec![0.;12];force[6]=f;
 NonlinearFrameSolveInput{node_count:2,elements:vec![e],user_stiffness_elements:vec![],curved_bend_elements:vec![],force,base_restrained_dofs:(0..12).filter(|i|*i!=6).collect(),initial_states:vec![SupportStateRecord::new(s.support_id.clone(),seed)],nonlinear_supports:vec![s],friction_normal_reactions:vec![],derived_friction_normal_reactions:vec![],convergence:ConvergenceControl::new("M1-N-observation-zero-state-change",ConvergencePolicyStatus::Tbd,0.,0.,8).unwrap()}
}
fn main(){
 let s=NonlinearSupport::one_way("U",1,FrameDof::Ux,ActivationSense::NegativeReaction);
 let mut i=base(s.clone(),ActiveSetState::Inactive,10.); i.convergence.max_iterations=0;
 let r=std::panic::catch_unwind(||solve_active_set_frame(&i)); println!("ZERO_MAX_ITERATIONS {r:?}");
 for engaged in [true,false]{let t=TrialSupportState{support_id:"U".into(),displacement:f64::NAN,reaction:f64::NAN,normal_reaction:None,tangential_reaction:None};println!("NAN_TRIAL engaged={engaged} {:?}",classify_support_state(&s,&t,Some(if engaged{ActiveSetState::Active}else{ActiveSetState::Inactive})));}
 let mut conflict=base(s.clone(),ActiveSetState::Active,10.);conflict.base_restrained_dofs.push(6);println!("CONFLICT_BOUNDARY {:?}",solve_active_set_frame(&conflict));
 let mut missing=base(s.clone(),ActiveSetState::Active,10.);missing.initial_states.clear();println!("MISSING_SEED {:?}",solve_active_set_frame(&missing));
 let mut duplicate=base(s.clone(),ActiveSetState::Inactive,-10.);duplicate.nonlinear_supports.push(s.clone());println!("DUPLICATE_SUPPORT {:?}",solve_active_set_frame(&duplicate));
}

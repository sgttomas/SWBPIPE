use open_pipe_stress_frame_kernel::*;
use open_pipe_stress_nonlinear_integration::*;
use open_pipe_stress_nonlinear_supports::*;
fn base(s:NonlinearSupport, seed:ActiveSetState,f:f64)->NonlinearFrameSolveInput {
 let e=FrameElement::new(FrameNode::new(0,[0.,0.,0.]).unwrap(),FrameNode::new(1,[1.,0.,0.]).unwrap(),FrameSection::new(100.,40.,1.,1.,1.,1.).unwrap(),[0.,1.,0.]).unwrap();
 let mut force=vec![0.;12];force[6]=f;
 NonlinearFrameSolveInput{node_count:2,elements:vec![e],user_stiffness_elements:vec![],curved_bend_elements:vec![],force,base_restrained_dofs:(0..12).filter(|i|*i!=6).collect(),initial_states:vec![SupportStateRecord::new(s.support_id.clone(),seed)],nonlinear_supports:vec![s],friction_normal_reactions:vec![],derived_friction_normal_reactions:vec![],convergence:ConvergenceControl::new("M1-N-observation-zero-state-change",ConvergencePolicyStatus::Tbd,0.,0.,8).unwrap()}
}
fn run(name:&str,input:NonlinearFrameSolveInput){
 for mode in [LinearSolveMode::SparseInteractive,LinearSolveMode::DenseScrutiny]{
  println!("CASE {name} MODE {mode:?}");
  match solve_active_set_frame_with_mode(&input,mode){Err(e)=>println!("ERROR {e}"),Ok(r)=>{
   println!("SUMMARY converged={} blocked={} iterations={} u={:?} reaction={:?} states={:?} diagnostics={:?}",r.converged,r.is_blocked(),r.iterations.len(),r.displacements,r.reactions,r.final_states,r.diagnostics);
   for i in r.iterations{println!("ITER {} u6={} r6={} applied={:?} states={:?} residuals={:?}",i.iteration,i.displacements[6],i.reactions[6],i.applied_sliding_friction_forces,i.active_set.states,i.residuals);}
  }}
 }
}
fn main(){
 for sign in [1.,-1.] {for f in [5.,10.,-10.] {for seed in [ActiveSetState::Active,ActiveSetState::Inactive]{
  run(&format!("gap_sign{sign}_f{f}_seed{seed:?}"),base(NonlinearSupport::gap("G",1,FrameDof::Ux,0.05,if sign>0.{GapDirection::PositiveDisplacement}else{GapDirection::NegativeDisplacement}).unwrap(),seed,sign*f));
 }}}
 for lift in [false,true]{for f in [10.,-10.,0.]{for seed in [ActiveSetState::Active,ActiveSetState::Inactive]{
 let s=if lift{NonlinearSupport::lift_off("U",1,FrameDof::Ux,ActivationSense::NegativeReaction)}else{NonlinearSupport::one_way("U",1,FrameDof::Ux,ActivationSense::NegativeReaction)};
 run(&format!("unilateral_lift{lift}_f{f}_seed{seed:?}"),base(s,seed,f));
 }}}
 for f in [1.,-1.,10.,-10.]{for seed in [ActiveSetState::Sticking,ActiveSetState::Sliding]{
  let mut i=base(NonlinearSupport::friction("F",1,FrameDof::Ux,0.3).unwrap(),seed,f);i.friction_normal_reactions=vec![FrictionNormalReaction::new("F",10.).unwrap()];run(&format!("friction_f{f}_seed{seed:?}"),i);
 }}
 for (mu,n,cap) in [(0.,10.,8),(0.3,0.,8),(0.3,10.,1)]{let mut i=base(NonlinearSupport::friction("F",1,FrameDof::Ux,mu).unwrap(),ActiveSetState::Sliding,10.);i.friction_normal_reactions=vec![FrictionNormalReaction::new("F",n).unwrap()];i.convergence.max_iterations=cap;run(&format!("friction_mu{mu}_N{n}_cap{cap}"),i);}
 for seed in [ActiveSetState::Active,ActiveSetState::Inactive]{let mut i=base(NonlinearSupport::one_way("STABLE",0,FrameDof::Ux,ActivationSense::NegativeReaction),seed,10.);i.base_restrained_dofs.retain(|d|*d!=0);run(&format!("stability_seed{seed:?}"),i);}
}

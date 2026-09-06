use open_pipe_stress_frame_kernel::*;
use open_pipe_stress_nonlinear_integration::*;
use open_pipe_stress_nonlinear_supports::*;
fn node(i:usize)->FrameNode{FrameNode::new(i,[i as f64,0.,0.]).unwrap()}
fn bar(a:usize,b:usize)->FrameElement{FrameElement::new(node(a),node(b),FrameSection::new(100.,40.,1.,1.,1.,1.).unwrap(),[0.,1.,0.]).unwrap()}
fn base(n:usize,free:&[usize])->NonlinearFrameSolveInput {NonlinearFrameSolveInput{node_count:n,elements:vec![],user_stiffness_elements:vec![],curved_bend_elements:vec![],force:vec![0.;n*6],base_restrained_dofs:(0..n*6).filter(|x|!free.contains(x)).collect(),nonlinear_supports:vec![],initial_states:vec![],friction_normal_reactions:vec![],derived_friction_normal_reactions:vec![],convergence:ConvergenceControl::new("M1-N-V2-zero-state-change-observation",ConvergencePolicyStatus::Tbd,0.,0.,12).unwrap()}}
fn run(name:String,i:NonlinearFrameSolveInput){for mode in [LinearSolveMode::SparseInteractive,LinearSolveMode::DenseScrutiny]{println!("CASE {name} {mode:?}");println!("{:#?}",solve_active_set_frame_with_mode(&i,mode));}}
fn main(){
for f in [10.,-10.]{for a in [ActiveSetState::Active,ActiveSetState::Inactive]{for b in [ActiveSetState::Active,ActiveSetState::Inactive]{
let mut i=base(3,&[6,12]);i.elements=vec![bar(0,1),bar(1,2)];i.force[12]=f;
i.nonlinear_supports=vec![NonlinearSupport::gap("G1",1,FrameDof::Ux,0.04,GapDirection::PositiveDisplacement).unwrap(),NonlinearSupport::gap("G2",2,FrameDof::Ux,0.1,GapDirection::PositiveDisplacement).unwrap()];i.initial_states=vec![SupportStateRecord::new("G1",a),SupportStateRecord::new("G2",b)];run(format!("series_F{f}_{a:?}_{b:?}"),i);
}}}
for f in [10.,20.]{for seed in [ActiveSetState::Active,ActiveSetState::Inactive]{let mut i=base(2,&[6]);i.elements=vec![bar(0,1)];i.user_stiffness_elements=vec![UserStiffnessElement::new(node(0),node(1),[0.,1.,0.],200.,200.,200.,200.).unwrap()];i.force[6]=f;i.nonlinear_supports=vec![NonlinearSupport::gap("G",1,FrameDof::Ux,0.04,GapDirection::PositiveDisplacement).unwrap()];i.initial_states=vec![SupportStateRecord::new("G",seed)];run(format!("parallel_F{f}_{seed:?}"),i);}}
for sign in [1.,-1.]{for seed in [ActiveSetState::Active,ActiveSetState::Inactive]{let mut i=base(2,&[9]);i.elements=vec![bar(0,1)];i.force[9]=4.*sign;i.nonlinear_supports=vec![NonlinearSupport::gap("RG",1,FrameDof::Rx,0.05,if sign>0.{GapDirection::PositiveDisplacement}else{GapDirection::NegativeDisplacement}).unwrap()];i.initial_states=vec![SupportStateRecord::new("RG",seed)];run(format!("rotation_sign{sign}_{seed:?}"),i);}}
for seed in [ActiveSetState::Sticking,ActiveSetState::Sliding]{let mut i=base(2,&[6]);i.user_stiffness_elements=vec![UserStiffnessElement::new(node(0),FrameNode::new(1,[1.,1.,0.]).unwrap(),[-1.,1.,0.],100.,200.,200.,200.).unwrap()];i.force[6]=10.;i.force[7]=-10.;i.nonlinear_supports=vec![NonlinearSupport::friction("F",1,FrameDof::Ux,0.3).unwrap()];i.initial_states=vec![SupportStateRecord::new("F",seed)];i.derived_friction_normal_reactions=vec![DerivedFrictionNormalReaction::from_support_reaction("F",1,FrameDof::Uy,"fixed_y").unwrap()];run(format!("coupled_normal_{seed:?}"),i);}
}

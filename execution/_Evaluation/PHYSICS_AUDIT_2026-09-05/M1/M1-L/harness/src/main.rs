#![allow(dead_code)]
use open_pipe_stress_frame_kernel as f;
use open_pipe_stress_sparse_direct as s;
use open_pipe_stress_solver_diagnostics as d;
mod performance_exact;
fn witness(id:&str,a:Vec<Vec<f64>>,b:Vec<f64>) {
 println!("INPUT|{}|{:?}|{:?}",id,a,b);
 println!("DENSE|{}|{:?}",id,f::solve_dense(&a,&b));
 println!("SPARSE|{}|{:?}",id,s::solve_symmetric_system(&a,&b));
 let entries:Vec<_>=(0..a.len()).flat_map(|i|(0..=i).map(move|j|(i,j))).map(|(i,j)|s::SymmetricMatrixEntry{row:i,col:j,value:a[i][j]}).collect();
 println!("ENTRY|{}|{:?}",id,s::solve_symmetric_system_from_entries(a.len(),&entries,&b));
}
fn main(){
 witness("finite_solution_overflow",vec![vec![1e308,1e308],vec![1e308,-1e308]],vec![1e308,-1e308]);
 witness("scalar_overflow",vec![vec![0.5]],vec![1e308]);
 witness("dense_elimination_overflow",vec![vec![1e308,1e308],vec![1e308,-1e308]],vec![1.,1.]);
 witness("scalar_small_scale",vec![vec![2f64.powi(-40)]],vec![2f64.powi(-40)]);
 witness("scalar_ordinary",vec![vec![2.]],vec![4.]);
 witness("zero_load",vec![vec![4.,1.],vec![1.,3.]],vec![0.,0.]);
 witness("singular",vec![vec![1.,1.],vec![1.,1.]],vec![1.,1.]);
 witness("indefinite_pivot",vec![vec![0.,1.],vec![1.,0.]],vec![2.,1.]);
 witness("negative_pivot",vec![vec![-2.,0.],vec![0.,1.]],vec![2.,1.]);
 for n in [1usize,2,3,6,12,24] {
  let mut b=vec![vec![0.;n];n]; for i in 0..n{for j in 0..n{b[i][j]=if i==j {(n*4)as f64} else {((i*17+j*13+7)%5)as f64-2.};}}
  let mut a=vec![vec![0.;n];n];for i in 0..n{for j in 0..n{a[i][j]=(0..n).map(|k|b[k][i]*b[k][j]).sum::<f64>()+if i==j{1.}else{0.};}}
  let x:Vec<_>=(0..n).map(|i|(i as i32%7-3)as f64).collect();let force:Vec<_>=a.iter().map(|r|r.iter().zip(&x).map(|(a,x)|a*x).sum()).collect();
  println!("EXPECTED|spd{}|{:?}",n,x);witness(&format!("spd{}",n),a.clone(),force.clone());
  let first=s::solve_symmetric_system(&a,&force).unwrap();let second=s::solve_symmetric_system(&a,&force).unwrap();println!("REPEAT|spd{}|{}",n,first==second);
 }
 let a=vec![vec![4.,1.],vec![1.,3.]];let b=vec![6.,7.];
 for (name,ix,u) in [("prescribed",vec![0],vec![1.]),("fully_prescribed",vec![0,1],vec![1.,2.]),("repeated",vec![0,0],vec![1.,1.]),("out_of_range",vec![2],vec![1.]),("mismatched",vec![0],vec![])] {
  match f::reduce_system_with_prescribed_displacements(&a,&b,&ix,&u){Ok(r)=>{println!("REDUCTION|{}|{:?}|{:?}|{:?}",name,r.free_dofs,r.stiffness,r.force);println!("REDUCED_DENSE|{}|{:?}",name,f::solve_dense(&r.stiffness,&r.force));println!("REDUCED_SPARSE|{}|{:?}",name,s::solve_symmetric_system(&r.stiffness,&r.force));},Err(e)=>println!("REDUCTION_ERROR|{}|{:?}",name,e)}
 }
 println!("INVALID|ragged_dense|{:?}",f::solve_dense(&[vec![1.,2.]],&[1.]));
 println!("INVALID|nonfinite_dense|{:?}",f::solve_dense(&[vec![f64::NAN]],&[1.]));
 println!("INVALID|nonfinite_sparse|{:?}",s::solve_symmetric_system(&[vec![f64::INFINITY]],&[1.]));
 println!("INVALID|short_rhs|{:?}",s::solve_symmetric_system(&a,&[1.]));
 println!("INVALID|bad_permutation|{:?}",s::SymmetricProfileMatrix::from_dense_with_order(&a,&[0,0]));
 println!("INVALID|bad_entry|{:?}",s::solve_symmetric_system_from_entries(1,&[s::SymmetricMatrixEntry{row:1,col:0,value:1.}],&[1.]));
 for (ratio,warn,fail) in [(2.,2.,3.),(3.,2.,3.),(1.,3.,2.),(f64::NAN,2.,3.)] {println!("DIAGNOSTIC|{:?}",d::classify_condition_ratio(ratio,warn,fail));}
 for (name,a) in [("tridiag32",(0..32).map(|i|(0..32).map(|j|if i==j{2.}else if (i as i32-j as i32).abs()==1{-1.}else{0.}).collect()).collect()),("eigen_floor",vec![vec![1e-10,0.],vec![0.,1.]]),("singular_eigen",vec![vec![0.,0.],vec![0.,1.]])]{println!("COND_MATRIX|{}|{:?}",name,a);println!("CONDITION|{}|{:?}",name,performance_exact::audit_condition(&a));println!("EIGEN_DIAGONAL|{}|{:?}",name,performance_exact::audit_eigen(&a));}
 let fixture=performance_exact::invented_cantilever_chain_fixture(8).unwrap();let a=f::assemble_global_stiffness(fixture.node_count,&fixture.elements).unwrap();let r=f::reduce_system(&a,&fixture.force,&fixture.restrained_dofs).unwrap();println!("COND_MATRIX|existing_chain8|{:?}",r.stiffness);println!("CONDITION|existing_chain8|{:?}",performance_exact::audit_condition(&r.stiffness));println!("EIGEN_DIAGONAL|existing_chain8|{:?}",performance_exact::audit_eigen(&r.stiffness));
 let mut extreme=performance_exact::invented_cantilever_chain_fixture(1).unwrap(); extreme.force[7]=1e308;
 let ek=f::assemble_global_stiffness(extreme.node_count,&extreme.elements).unwrap();let er=f::reduce_system(&ek,&extreme.force,&extreme.restrained_dofs).unwrap();
 for (kind,x) in [("dense",f::solve_dense(&er.stiffness,&er.force).unwrap()),("sparse",s::solve_symmetric_system(&er.stiffness,&er.force).unwrap().solution)] {println!("EXTREME_VECTOR|{}|{:?}",kind,x);let residual:Vec<_>=er.stiffness.iter().zip(&er.force).map(|(row,b)|row.iter().zip(&x).map(|(a,x)|a*x).sum::<f64>()-b).collect();println!("EXTREME_RESIDUAL|{}|{:?}",kind,residual);}
 println!("HARNESS_EXTREME|{:?}",performance_exact::run_fixture_repeat(&extreme,&performance_exact::HarnessSettings::default()));
}

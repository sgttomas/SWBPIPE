//! Observation only: this executable does not manufacture analytical expectations.
use open_pipe_stress_product_physics::{run_linear_static_preview_with_mode,LinearStaticPreviewRequest,PreviewSolverMode};
use serde_json::{json,Value};
fn model(id:&str,l:f64,torque:f64)->Value {
 json!({"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},"project":{"id":format!("reference:{id}"),"units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},
 "nodes":[{"id":"root","position":{"x":0,"y":0,"z":0}},{"id":"tip","position":{"x":l,"y":0,"z":0}}],
 "pipe_segments":[{"id":"pipe","from":"root","to":"tip","material":"material","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}}}],
 "materials":[{"id":"material","elastic_modulus":{"value":200e9,"unit":"Pa"},"shear_modulus":{"value":80e9,"unit":"Pa"}}],
 "supports":[{"id":"anchor","node":"root","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"]}],
 "load_cases":[{"id":"case","label":id,"kind":"primitive_user_load","primitive_loads":[{"id":"torque","category":"concentrated_moment","target":{"type":"node","node":"tip"},"direction":"RX","magnitude":{"value":torque,"unit":"N*m"},"dimension":"moment"}]}],"combinations":[]})
}
fn add_original_provenance(model:&mut Value) {
 const SOURCE:&str="original_analytical_fixture_numerical_integrity_v1_no_external_project_data";
 for key in ["nodes","pipe_segments","materials","supports","load_cases"] {
  for entity in model[key].as_array_mut().unwrap() {
   entity["provenance"]=json!(SOURCE);
   if let Some(loads)=entity.get_mut("primitive_loads").and_then(Value::as_array_mut) {
    for load in loads {load["provenance"]=json!(SOURCE);}
   }
  }
 }
}
fn main(){
 let mut cases=Vec::new();
 for (id,l,t) in [("N08-positive",2.,1.),("N08-negative",2.,-1.),("N08-small",2.,0.1),("N08-small-negative",2.,-0.1),("N09-torsion",10.,0.1)] {cases.push((id,model(id,l,t)));}
 for (id,k,t) in [("N05",1e-4,1e-8),("N06",1e-12,1e-16)] {
  let mut m=model(id,2.,t);m["supports"][0]["restraints"]=json!(["UX","UY","UZ","RY","RZ"]);
  m["supports"].as_array_mut().unwrap().push(json!({"id":"soft","node":"root","family":"spring","restraints":["RX"],"stiffness":{"dof":"RX","value":{"value":k,"unit":"N*m/rad"}}}));cases.push((id,m));
 }
 for (id,t) in [("N02-loaded",1.),("N02-unloaded",0.)] {
  let mut m=model(id,2.,t);m["nodes"][1]["position"]=json!({"x":1.2,"y":1.6,"z":0});
  m["supports"]=json!([{"id":"root-translations","node":"root","restraints":["UX","UY","UZ"]},{"id":"tip-translations","node":"tip","restraints":["UX","UY","UZ"]}]);
  m["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"]=json!(t*0.6);
  let mut my=m["load_cases"][0]["primitive_loads"][0].clone();my["id"]=json!("torque-y");my["direction"]=json!("RY");my["magnitude"]["value"]=json!(t*0.8);m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(my);cases.push((id,m));
 }
 for (id,l,force) in [("N01",2.,1000.),("N09-bending",10.,100.)] {
  let mut m=model(id,l,0.);m["load_cases"][0]["primitive_loads"][0]=json!({"id":"force","category":"concentrated_force","target":{"type":"node","node":"tip"},"direction":"global_y","magnitude":{"value":force,"unit":"N"},"dimension":"force"});cases.push((id,m));
 }
 let n02=cases.iter().find(|(id,_)|*id=="N02-loaded").unwrap().1.clone();
 for (id,dof) in [("N03-RX-stabilized","RX"),("N03-RZ-not-stabilized","RZ")] {
  let mut m=n02.clone();m["supports"][0]["restraints"].as_array_mut().unwrap().push(json!(dof));cases.push((id,m));
 }
 let mut m=model("N04",2.,0.);m["nodes"].as_array_mut().unwrap().extend([json!({"id":"free-root","position":{"x":0,"y":3,"z":0}}),json!({"id":"free-tip","position":{"x":2,"y":3,"z":0}})]);
 let mut pipe=m["pipe_segments"][0].clone();pipe["id"]=json!("free-pipe");pipe["from"]=json!("free-root");pipe["to"]=json!("free-tip");m["pipe_segments"].as_array_mut().unwrap().push(pipe);
 m["load_cases"][0]["primitive_loads"][0]=json!({"id":"force","category":"concentrated_force","target":{"type":"node","node":"tip"},"direction":"global_y","magnitude":{"value":1000,"unit":"N"},"dimension":"force"});cases.push(("N04",m));
 for (id,mut m) in cases {add_original_provenance(&mut m);for mode in [PreviewSolverMode::DenseScrutiny,PreviewSolverMode::SparseInteractive] {
  let request=serde_json::from_value::<LinearStaticPreviewRequest>(json!({"model":m,"materials":[]}));
  match request {Ok(r)=>println!("{}",json!({"case":id,"mode":mode.as_str(),"input":m,"observed":run_linear_static_preview_with_mode(r,mode)})),Err(e)=>println!("{}",json!({"case":id,"input":m,"dto_error":e.to_string()}))}
 }}
 let references:Value=serde_json::from_str(include_str!("../fixtures.json")).unwrap();
 for row in references["NP"]["A"].as_array().unwrap() {
  let k:Vec<Vec<f64>>=row["K_stored_exact"].as_array().unwrap().iter().map(|r|r.as_array().unwrap().iter().map(|v|v.as_str().unwrap().parse().unwrap()).collect()).collect();
  let f:Vec<f64>=row["f_stored_exact"].as_array().unwrap().iter().map(|v|v.as_str().unwrap().parse().unwrap()).collect();
  println!("{}",json!({"case":format!("NP-A-{}",row["id"].as_str().unwrap()),"dense_generic_observation":format!("{:?}",open_pipe_stress_frame_kernel::solve_dense(&k,&f)),"sparse_observation":format!("{:?}",open_pipe_stress_sparse_direct::solve_symmetric_system(&k,&f))}));
 }
 for (id,k,f) in [("N07",vec![vec![1.,2.],vec![2.,1.]],vec![1.,1.]),("NP-D-skew",vec![vec![2.,1.],vec![1.01,2.]],vec![1.,1.])] {
  println!("{}",json!({"case":id,"dense_generic_observation":format!("{:?}",open_pipe_stress_frame_kernel::solve_dense(&k,&f)),"sparse_observation":format!("{:?}",open_pipe_stress_sparse_direct::solve_symmetric_system(&k,&f))}));
 }
}

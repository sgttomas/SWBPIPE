use serde_json::{json,Value};
use open_pipe_stress_operation_applier::{canonical_json,sha256_hex};
use open_pipe_stress_self_weight_wasm::{generate_plan,generate_self_weight_plan_json};
fn main(){
 let path=std::env::var("SELF_WEIGHT_MASS_BASE_MODEL").unwrap();
 let artifact:Value=serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
 let cases=[("ordinary",0.1,0.01,1000.0,0.0,0.0,0.0),
 ("thin_1e9",1.0,1e-9,1.0,0.0,0.0,0.0),
 ("thin_1e12",1.0,1e-12,1.0,0.0,0.0,0.0),
 ("sub_ulp_wall",1.0,2f64.powi(-55),1.0,0.0,0.0,0.0),
 ("thin_insulation",1.0,0.01,1e-30,0.0,1e-12,1e12),
 ("sub_ulp_insulation",1.0,0.01,1e-30,0.0,2f64.powi(-55),1e15),
 ("small_bore_contents",1.0,0.5-2f64.powi(-30),1e-30,1e12,0.0,0.0)];
 let mut output=Vec::new();
 for(name,od,wall,rho,contents,insulation,insulation_rho) in cases {
  let mut model=artifact["initial_applied_model"].clone();model["load_cases"]=json!([]);
  model["pipe_segments"][0]["section"]=json!({"outside_diameter":{"value":od,"unit":"m"},"wall_thickness":{"value":wall,"unit":"m"},
   "material_density":{"value":rho,"unit":"kg/m^3"},"contents_density":{"value":contents,"unit":"kg/m^3"},"insulation_thickness":{"value":insulation,"unit":"m"},"insulation_density":{"value":insulation_rho,"unit":"kg/m^3"}});
  let request=json!({"case_id":"case:mass-probe","label":"Invented numerical mass probe","pipe_refs":["pipe:beam"],"gravity":{"value":-1,"unit":"m/s^2","axis":"global_y"},"provenance":"Invented numeric boundary probe, no engineering qualification","source_model_hash":format!("sha256:{}",sha256_hex(&canonical_json(&model)))});
  let before=model.clone();let outcome=generate_plan(&model,&request);
  let wire:Value=serde_json::from_str(&generate_self_weight_plan_json(&model.to_string(),&request.to_string())).unwrap();
  let transport_equal=outcome==wire;assert_eq!(before,model);
  output.push(json!({"name":name,"input_model":model,"request":request,"outcome":outcome,"wire_outcome":wire,"transport_equal":transport_equal}));
 }
 println!("{}",serde_json::to_string_pretty(&output).unwrap());
}

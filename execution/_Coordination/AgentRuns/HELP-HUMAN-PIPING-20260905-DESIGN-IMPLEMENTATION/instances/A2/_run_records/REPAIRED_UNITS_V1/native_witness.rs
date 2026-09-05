use open_pipe_stress_product_physics::{LinearStaticPreviewRequest, PreviewModel, PreviewSolverMode, run_linear_static_preview_with_mode};
use serde_json::{Value,json};
fn main() {
 let args:Vec<String>=std::env::args().collect();
 let payload:Value=serde_json::from_str(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();
 let model:PreviewModel=serde_json::from_value(payload).unwrap();
 let result=run_linear_static_preview_with_mode(LinearStaticPreviewRequest{model,materials:vec![]},PreviewSolverMode::default());
 println!("{}",serde_json::to_string_pretty(&json!(result)).unwrap());
}

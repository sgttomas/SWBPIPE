use open_pipe_stress_product_physics::{
    run_linear_static_preview, LinearStaticPreviewRequest, PreviewModel,
};

fn main() {
    let model: PreviewModel = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/invented_preview_model.json"
    ))
    .expect("invented preview model fixture should parse");
    let result = run_linear_static_preview(LinearStaticPreviewRequest {
        model,
        materials: vec![],
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&result).expect("result should serialize")
    );
}

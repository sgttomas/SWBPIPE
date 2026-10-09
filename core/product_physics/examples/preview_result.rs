use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewModel,
    PreviewSolverMode,
};

/// The bundled models this example can run, by name. The first is the default:
/// the invented preview model, whose legacy pressure and joint the product
/// refuses. The second is the valid demo behind the browser's bundled reference.
const MODELS: [(&str, &str); 2] = [
    (
        "invented_preview_model",
        include_str!("../../../fixtures/product_preview/invented_preview_model.json"),
    ),
    (
        "invented_demo_model",
        include_str!("../../../fixtures/product_preview/invented_demo_model.json"),
    ),
];

fn usage() -> ! {
    eprintln!(
        "usage: preview_result [sparse_interactive|dense_scrutiny] [invented_preview_model|invented_demo_model]"
    );
    std::process::exit(2);
}

fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let parse_mode = |mode: &str| {
        PreviewSolverMode::from_wire(mode).unwrap_or_else(|| {
            eprintln!("unsupported preview solver mode: {mode}");
            std::process::exit(2);
        })
    };
    let (mode, model_text) = match arguments.as_slice() {
        [] => (PreviewSolverMode::default(), MODELS[0].1),
        [mode] => (parse_mode(mode), MODELS[0].1),
        [mode, name] => (
            parse_mode(mode),
            MODELS
                .iter()
                .find(|(model, _)| model == name)
                .map(|(_, text)| *text)
                .unwrap_or_else(|| usage()),
        ),
        _ => usage(),
    };
    let model: PreviewModel =
        serde_json::from_str(model_text).expect("bundled preview model fixture should parse");
    let result = run_linear_static_preview_with_mode(
        LinearStaticPreviewRequest {
            model,
            materials: vec![],
        },
        mode,
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&result).expect("result should serialize")
    );
}

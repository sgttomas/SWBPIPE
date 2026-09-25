//! Emit actual raw results for the maintained exact-profile connected fixture.
//! Usage: cargo run --example exact_pressure_connected -- [sparse|dense] [request.json | --model model.json]
//! The result is core evidence only; numerical/consumer admission remains a separate join.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewSolverMode,
};
use std::{env, error::Error, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let mode = match args.next().as_deref() {
        None | Some("sparse") => PreviewSolverMode::SparseInteractive,
        Some("dense") => PreviewSolverMode::DenseScrutiny,
        Some(_) => return Err("mode must be sparse or dense".into()),
    };
    let request: LinearStaticPreviewRequest = match args.next().as_deref() {
        Some("--model") => {
            let path = args.next().ok_or("--model requires a model JSON path")?;
            LinearStaticPreviewRequest {
                model: serde_json::from_str(&fs::read_to_string(path)?)?,
                // Explicit model-only invocation uses its authored material list;
                // no request-level override or physical default is supplied.
                materials: Vec::new(),
            }
        }
        Some(path) => serde_json::from_str(&fs::read_to_string(path)?)?,
        None => serde_json::from_str(include_str!(
            "../tests/fixtures/exact_pressure_connected_request.json"
        ))?,
    };
    if args.next().is_some() {
        return Err(
            "usage: exact_pressure_connected [sparse|dense] [request.json | --model model.json]"
                .into(),
        );
    }
    let result = run_linear_static_preview_with_mode(request, mode);
    serde_json::to_writer_pretty(io::stdout().lock(), &result)?;
    println!();
    if result.status.mechanics != "MECHANICS_SOLVED" {
        return Err("core solve did not succeed; inspect the emitted diagnostics".into());
    }
    Ok(())
}

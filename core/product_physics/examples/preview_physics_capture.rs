//! Emit actual raw preview-physics-1 results through the captured value entry the
//! desktop uses. Usage:
//! cargo run --example preview_physics_capture -- [sparse|dense] --model model.json
//! The output is example transport data from the actual producer, not an oracle.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use std::{env, error::Error, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let [mode, flag, path] = args.as_slice() else {
        return Err("usage: preview_physics_capture [sparse|dense] --model model.json".into());
    };
    let mode = match mode.as_str() {
        "sparse" => PreviewSolverMode::SparseInteractive,
        "dense" => PreviewSolverMode::DenseScrutiny,
        _ => return Err("mode must be sparse or dense".into()),
    };
    if flag != "--model" {
        return Err("usage: preview_physics_capture [sparse|dense] --model model.json".into());
    }
    let model: serde_json::Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    // Model-only invocation uses the model's authored material list.
    let request = serde_json::json!({"model": model, "materials": []});
    let result = run_linear_static_preview_value_with_mode(request, mode)?;
    serde_json::to_writer_pretty(io::stdout().lock(), &result)?;
    println!();
    if result.status.mechanics != "MECHANICS_SOLVED" {
        return Err("core solve did not succeed; inspect the emitted diagnostics".into());
    }
    Ok(())
}

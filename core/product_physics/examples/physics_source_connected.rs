//! Emit the actual value-aware producer result for an explicitly authored request.
//! The output retains source/physical evidence; this command is not a native witness.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, PreviewSolverMode,
};
use std::{env, error::Error, fs, io};
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let mode = match args.next().as_deref() {
        Some("sparse") => PreviewSolverMode::SparseInteractive,
        Some("dense") => PreviewSolverMode::DenseScrutiny,
        _ => return Err("usage: physics_source_connected [sparse|dense] request.json".into()),
    };
    let path = args
        .next()
        .ok_or("an actual complete request path is required")?;
    if args.next().is_some() {
        return Err("unexpected extra arguments".into());
    }
    let request: serde_json::Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    let result = run_linear_static_preview_value_with_mode(request, mode)?;
    serde_json::to_writer_pretty(io::stdout().lock(), &result)?;
    println!();
    if result.status.mechanics != "MECHANICS_SOLVED" {
        return Err(
            "mechanics did not solve; emitted diagnostics retain the actual failure".into(),
        );
    }
    if result
        .source_block_recovery
        .as_ref()
        .is_some_and(|r| r["body"]["status"] != "qualified")
    {
        return Err("selected source method did not qualify the whole envelope".into());
    }
    Ok(())
}

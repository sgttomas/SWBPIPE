// RV89 probe (copy only): what the S-1 oracle's Direct call does in the runner workspace.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_retained_direct, PreviewSolverMode};
use serde_json::Value;
#[test]
fn rv89_runner_workspace_direct_call() {
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let raw: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap();
        let out = run_linear_static_preview_value_with_retained_direct(raw, mode).unwrap();
        let a = out.admission().unwrap();
        let succ = out.successor().map(|s| { let b = serde_json::to_vec(s).unwrap(); (b.len(), format!("{:x}", { use std::hash::{Hash, Hasher}; let mut h = std::collections::hash_map::DefaultHasher::new(); b.hash(&mut h); h.finish() })) });
        if let Some(x) = out.successor() { std::fs::write(format!("{}/g6r/runner_direct_{mode:?}.json", env!("RV89_S")), serde_json::to_vec(x).unwrap()).unwrap(); }
        eprintln!("RV89_RUNNER_DIRECT {mode:?} profile={:?} successor={:?}", a.profile, succ);
    }
}

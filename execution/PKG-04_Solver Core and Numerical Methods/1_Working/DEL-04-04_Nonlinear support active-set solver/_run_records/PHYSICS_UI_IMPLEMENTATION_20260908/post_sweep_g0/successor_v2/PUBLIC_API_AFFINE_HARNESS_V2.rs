use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, MechanicsEnvelope,
    PreviewSolverMode,
};
use std::{env, fs};

fn value(result: &MechanicsEnvelope, id: &str) -> Option<f64> {
    result.results.iter().find(|item| item.id == id).map(|item| item.value)
}

fn run(label: &str, variant: &str, path: &str, mode: PreviewSolverMode) {
    let body = fs::read_to_string(path).unwrap();
    let input: LinearStaticPreviewRequest = serde_json::from_str(&body).unwrap();
    let result = run_linear_static_preview_with_mode(input, mode);
    println!(
        "{}",
        serde_json::json!({
            "fixture": label,
            "variant": variant,
            "mode": mode.as_str(),
            "l100_friction_n": value(&result, "result:nonlinear-support:support-NL-130-FRIC:uz-reaction"),
            "l100_normal_n": value(&result, "result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction"),
            "l100_iterations": value(&result, "result:nonlinear-support:iteration-count"),
            "l100_friction_state": value(&result, "result:nonlinear-support:support-NL-130-FRIC:state-code"),
            "l100_one_way_state": value(&result, "result:nonlinear-support:support-NL-140:state-code"),
            "l100_converged": value(&result, "result:nonlinear-support:converged-flag"),
            "l200_friction_n": value(&result, "result:loadcase:load-L-200:nonlinear-support:support-NL-130-FRIC:uz-reaction"),
            "l200_normal_n": value(&result, "result:loadcase:load-L-200:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction"),
            "l200_iterations": value(&result, "result:loadcase:load-L-200:nonlinear-support:iteration-count"),
            "l200_friction_state": value(&result, "result:loadcase:load-L-200:nonlinear-support:support-NL-130-FRIC:state-code"),
            "l200_one_way_state": value(&result, "result:loadcase:load-L-200:nonlinear-support:support-NL-140:state-code"),
            "l200_converged": value(&result, "result:loadcase:load-L-200:nonlinear-support:converged-flag"),
            "mechanics": result.status.mechanics,
            "diagnostic_codes": result.diagnostics.iter().map(|item| item.code.as_str()).collect::<Vec<_>>()
        })
    );
}

fn main() {
    let root = &env::args().collect::<Vec<_>>()[1];
    let mut variants = vec!["base_mu0".to_string()];
    for case_label in ["l100", "l200"] {
        for magnitude in [1_i32, -1, 2, -2, 3, -3, 5, -5, 7, -7, 10, -10, 17, -17, 23, -23, 37, -37, 50, -50] {
            let sign = if magnitude > 0 { "plus" } else { "minus" };
            variants.push(format!("probe_{case_label}_{sign}{}_mu0", magnitude.abs()));
        }
    }
    for index in 0..26 {
        variants.push(format!("probe_l200_fractional_{index:02}_mu0"));
    }
    variants.extend(["target_mu001".to_string(), "cross_mu0005".to_string(), "cross_mu002".to_string()]);
    for label in ["retained_spring", "historical_no_spring"] {
        for variant in &variants {
            let path = format!("{root}/{label}_{variant}.json");
            for mode in [
                PreviewSolverMode::DenseScrutiny,
                PreviewSolverMode::SparseInteractive,
            ] {
                run(label, variant, &path, mode);
            }
        }
    }
}

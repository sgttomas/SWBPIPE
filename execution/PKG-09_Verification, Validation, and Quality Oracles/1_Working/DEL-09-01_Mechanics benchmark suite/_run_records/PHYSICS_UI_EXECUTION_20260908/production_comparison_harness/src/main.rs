use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewModel,
    PreviewSolverMode,
};
use serde_json::Value;
use std::{env, fs, path::PathBuf};

fn pressure_free_model(fixture: &str, include_c150: bool, include_friction: bool) -> PreviewModel {
    let mut value: Value = serde_json::from_str(fixture).expect("fixture JSON");
    value["load_cases"]
        .as_array_mut()
        .expect("load_cases")
        .retain(|case| case["id"] == "load:L-100");
    value["combinations"] = Value::Array(Vec::new());
    for case in value["load_cases"].as_array_mut().expect("load_cases") {
        case["primitive_loads"]
            .as_array_mut()
            .expect("primitive_loads")
            .retain(|load| load["dimension"] != "pressure");
    }
    if !include_c150 {
        value["components"]
            .as_array_mut()
            .expect("components")
            .retain(|component| component["id"] != "component:C-150");
    }
    if !include_friction {
        value["supports"]
            .as_array_mut()
            .expect("supports")
            .retain(|support| support["id"] != "support:NL-130-FRIC");
    }
    serde_json::from_value(value).expect("transformed model")
}

fn main() {
    let mut args = env::args().skip(1);
    let fixture_path = PathBuf::from(args.next().expect("fixture path"));
    let output_dir = PathBuf::from(args.next().expect("output directory"));
    assert!(args.next().is_none(), "unexpected arguments");
    fs::create_dir_all(&output_dir).expect("create output directory");
    let fixture = fs::read_to_string(&fixture_path).expect("read fixture");

    for (case_name, include_c150, include_friction) in [
        ("physical_frame_frictionless", false, false),
        ("literal_adapter_frictionless", true, false),
        ("physical_frame", false, true),
        ("literal_adapter", true, true),
    ] {
        for (mode_name, mode) in [
            ("dense_scrutiny", PreviewSolverMode::DenseScrutiny),
            ("sparse_interactive", PreviewSolverMode::SparseInteractive),
        ] {
            let result = run_linear_static_preview_with_mode(
                LinearStaticPreviewRequest {
                    model: pressure_free_model(&fixture, include_c150, include_friction),
                    materials: Vec::new(),
                },
                mode,
            );
            let path = output_dir.join(format!("{case_name}.{mode_name}.json"));
            fs::write(path, serde_json::to_string_pretty(&result).unwrap() + "\n")
                .expect("write output");
        }
    }
}

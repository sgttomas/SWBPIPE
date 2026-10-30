//! B4/D1 (b): the pressure-free captured (preview) route no longer selects source
//! blocks. The former source-block controls publish the preview route's own
//! identity with no source-block receipt; W1 covers each formerly selected case
//! (`w1_source_census_tests::b4_w1_covers_every_formerly_selected_source_block_case`).
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::Value;

#[test]
fn the_preview_route_publishes_no_source_blocks() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/product_preview/w1_controls");
    for stem in ["n05", "n06", "multicase", "ui/n05", "ui/n06", "ui/multicase"] {
        for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
            let path = root.join(format!("{stem}-{}.request.json", mode.as_str()));
            let request: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            let envelope = run_linear_static_preview_value_with_mode(request, mode).unwrap();
            if let Ok(folder) = std::env::var("OPS_W1_CONTROL_RAW_DIR") {
                // The actual preview-route output, bytes as published (zero signs kept).
                let path = std::path::Path::new(&folder).join(format!("{stem}-{}.raw.json", mode.as_str()));
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, serde_json::to_vec_pretty(&envelope).unwrap()).unwrap();
            }
            let raw = serde_json::to_value(envelope).unwrap();
            // n06's ordinary run genuinely fails (W1's failed-ordinary extension retains it).
            assert_eq!(raw["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/preview-physics-2", "{stem} {mode:?}");
            assert!(raw.get("source_block_recovery").is_none(), "{stem} {mode:?}");
            assert!(!raw["diagnostics"].as_array().unwrap().iter().any(|d| d["code"].as_str().unwrap().starts_with("SOURCE_BLOCK")), "{stem} {mode:?}");
        }
    }
}

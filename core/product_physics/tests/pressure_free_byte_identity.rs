//! T4-U6 (DEL-05-03 Design `section-bases-and-timoshenko.md` §2.5, §9): D-5's
//! two section bases are exact-contract only. Pressure-free envelopes do not
//! change, and any pressure-free byte change is a defect. Each committed
//! pressure-free product output below is regenerated through the entry its
//! writer uses and compared byte for byte:
//! - the preview-physics-2 demo and mechanics fixtures, the raw stdout of
//!   `examples/preview_result.rs` (`tools/serialization/generate_product_preview_mechanics.mjs`),
//!   both of whose models carry a combination, so the preview route's
//!   combination rows (U6-RB) are among the compared bytes;
//! - the G10 desktop milestone copies, the desktop solve request's bytes
//!   (`apps/desktop/src-tauri/src/lib.rs` replays the same copies).
//!
//! The W1 milestone successors (`fixtures/results/retained_precision_milestone_successor_*.json`)
//! are pinned live by `retained_facade_tests`, and the milestone's ordinary
//! bytes by `g10_desktop_milestone_model.rs`.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode, LinearStaticPreviewRequest,
    PreviewModel, PreviewSolverMode,
};
use serde_json::json;

const MODES: [(PreviewSolverMode, &str); 2] =
    [(PreviewSolverMode::SparseInteractive, "sparse"), (PreviewSolverMode::DenseScrutiny, "dense")];

fn first_difference(actual: &str, expected: &str) -> Option<usize> {
    actual.bytes().zip(expected.bytes()).position(|(a, b)| a != b).or((actual.len() != expected.len()).then(|| actual.len().min(expected.len())))
}

/// `examples/preview_result.rs`: the bundled model with no request materials,
/// printed pretty with one trailing newline.
#[test]
fn preview_physics_fixtures_are_byte_identical() {
    let fixtures: [(&str, &str, [&str; 2]); 2] = [
        (
            "invented_demo_model",
            include_str!("../../../fixtures/product_preview/invented_demo_model.json"),
            [
                include_str!("../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json"),
                include_str!("../../../fixtures/product_preview/invented_demo_result_preview_physics_1_dense.json"),
            ],
        ),
        (
            "invented_preview_model",
            include_str!("../../../fixtures/product_preview/invented_preview_model.json"),
            [
                include_str!("../../../fixtures/product_preview/invented_mechanics_result_preview_physics_1_sparse.json"),
                include_str!("../../../fixtures/product_preview/invented_mechanics_result_preview_physics_1_dense.json"),
            ],
        ),
    ];
    for (name, model_text, committed) in fixtures {
        let model: PreviewModel = serde_json::from_str(model_text).unwrap();
        assert!(model.pressure_contract.is_none(), "{name}: a pressure-free model");
        assert!(!model.combinations.is_empty(), "{name}: the preview route's combination rows are compared");
        for ((mode, suffix), expected) in MODES.into_iter().zip(committed) {
            let result = run_linear_static_preview_with_mode(LinearStaticPreviewRequest { model: model.clone(), materials: vec![] }, mode);
            let actual = format!("{}\n", serde_json::to_string_pretty(&result).unwrap());
            assert!(actual == expected, "{name} {suffix}: actual_len={} expected_len={} first_difference={:?}",
                actual.len(), expected.len(), first_difference(&actual, expected));
            assert!(result.summary.max_open_formula_stress_nominal_section.is_none()
                && result.summary.max_open_formula_stress_reduced_section.is_none(), "{name} {suffix}: no per-basis headline keys");
        }
    }
}

/// The preview-physics-2 envelopes the readers consume, the raw stdout of
/// `examples/preview_physics_capture.rs` (`-- <mode> --model <model>`: the
/// model with no request materials through the public value entry, printed
/// pretty with one trailing newline). Nothing else re-runs the producer on them.
#[test]
fn preview_physics_capture_envelopes_are_current() {
    const INVENTED: &str = include_str!("fixtures/preview_physics_invented_model.json");
    let captures: [(&str, &str, PreviewSolverMode, &str); 4] = [
        ("preview_physics_invented_sparse", INVENTED, PreviewSolverMode::SparseInteractive,
            include_str!("../../../fixtures/results/preview_physics_invented_sparse.json")),
        ("preview_physics_invented_dense", INVENTED, PreviewSolverMode::DenseScrutiny,
            include_str!("../../../fixtures/results/preview_physics_invented_dense.json")),
        ("preview_physics_range_envelope_sparse", include_str!("fixtures/preview_physics_range_envelope_model.json"),
            PreviewSolverMode::SparseInteractive, include_str!("../../../fixtures/results/preview_physics_range_envelope_sparse.json")),
        ("preview_physics_unicode_ids_sparse", include_str!("fixtures/preview_physics_unicode_ids_model.json"),
            PreviewSolverMode::SparseInteractive, include_str!("../../../fixtures/results/preview_physics_unicode_ids_sparse.json")),
    ];
    for (name, model, mode, expected) in captures {
        let model: serde_json::Value = serde_json::from_str(model).unwrap();
        let envelope = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), mode).unwrap();
        let actual = format!("{}\n", serde_json::to_string_pretty(&envelope).unwrap());
        assert!(actual == expected, "fixtures/results/{name}.json: regenerate with `cargo run --example preview_physics_capture -- {} --model ...`; actual_len={} expected_len={} first_difference={:?}",
            if mode == PreviewSolverMode::SparseInteractive { "sparse" } else { "dense" }, actual.len(), expected.len(), first_difference(&actual, expected));
    }
}

/// The G10 desktop copies: the desktop's solve request `{"model", "materials": []}` through the
/// public value entry, serialized compactly. `G10_DESKTOP_MILESTONE_WRITE=1` rewrites the two
/// committed copies from this computation; otherwise they must be current.
#[test]
fn desktop_milestone_copies_are_byte_identical() {
    let model: serde_json::Value =
        serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json")).unwrap();
    let committed = [
        include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_sparse_interactive.json"),
        include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_dense_scrutiny.json"),
    ];
    for ((mode, suffix), expected) in MODES.into_iter().zip(committed) {
        let envelope = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), mode).unwrap();
        let actual = serde_json::to_string(&envelope).unwrap();
        if std::env::var("G10_DESKTOP_MILESTONE_WRITE").as_deref() == Ok("1") {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(format!("../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_{}.json", mode.as_str()));
            std::fs::write(path, &actual).unwrap();
            continue;
        }
        assert!(actual == expected, "{suffix}: actual_len={} expected_len={} first_difference={:?}",
            actual.len(), expected.len(), first_difference(&actual, expected));
    }
}

/// U6-RB (DEL-05-03 Design §9): the per-base-id row index is built only when the preview
/// exists and has combinations, and its one reader, `append_combination_results`, runs only then;
/// with no preview or no combinations there are no combinations to append. So the
/// clones are skipped exactly where nothing reads them, and the published rows are unchanged
/// (the preview fixtures above carry combinations and are byte-identical).
#[test]
fn u6_rb_rows_by_base_id_is_built_and_read_only_with_the_preview() {
    let lib = include_str!("../src/lib.rs");
    let uses: Vec<_> = lib.match_indices("rows_by_base_id").map(|(at, _)| at).collect();
    assert_eq!(uses.len(), 3, "the declaration, the guarded insert and the one read");
    let guarded = &lib[..uses[1]];
    assert!(guarded.contains("let combining = preview.is_some() && !model.combinations.is_empty();"));
    let guard = guarded.rfind("if combining {").expect("the insert is guarded");
    assert!(!guarded[guard..].contains('}'), "the insert is inside the preview guard");
    let call_guard = lib[..uses[2]].rfind("let preview_gates = if combining {").expect("the one reader is guarded");
    let read = &lib[call_guard..uses[2]];
    assert!(read.contains("preview_physics::append_combination_results("), "{read}");
    assert!(!read.contains('}'), "the call is inside the combination guard");
    assert!(lib[uses[2]..].starts_with("rows_by_base_id, &mut results, &mut diagnostics)\n    } else {\n        Vec::new()"));
}

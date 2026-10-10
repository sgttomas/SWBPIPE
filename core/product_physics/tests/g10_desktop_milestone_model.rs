//! G10: the F2a milestone model (RF-SKEW-T-CANT-OFF-122-r1e-04) shaped for the
//! desktop app. `rf_skew_t_cant_off_122_r1e-04.desktop_model.json` is the request's
//! `model` with display fields added so the desktop shell can open it. Through the
//! desktop's own solve request (`{"model": <model>, "materials": []}`, built in
//! `apps/desktop/src-tauri/src/lib.rs::solve_preview_mechanics_with_mode`) and the
//! public entry it calls, it publishes the milestone's protected ordinary bytes.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const REQUEST: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const DESKTOP_MODEL: &str =
    include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
/// The milestone's protected ordinary bytes, by value. The pin is private: it lives in
/// `src/retained_wire_tests.rs` as `ORDINARY_SHA256` (a `#[cfg(test)]` module), and
/// `copied_pin_matches_its_source` keeps this copy equal to it.
const ORDINARY_SHA256: [(&str, usize, &str); 2] = [
    ("sparse_interactive", 68250, "9c7ec1a144a729f456a25fedb5dbe6a7dafde67d52b9cc8caff6c39bb0050871"),
    ("dense_scrutiny", 69366, "aa8b93ba9717bcc61bf708bb3dd7ab4848d655334dd1f3f6d5e852c2d40f8a8d"),
];
/// Fields of the request's model entries that receive a `label` equal to their `id`.
const LABELLED: [&str; 4] = ["materials", "nodes", "pipe_segments", "supports"];

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn desktop_model() -> Value {
    serde_json::from_str(DESKTOP_MODEL).unwrap()
}

#[test]
fn copied_pin_matches_its_source() {
    let source = include_str!("../src/retained_wire_tests.rs");
    for (name, len, digest) in ORDINARY_SHA256 {
        let row = format!("(\"{name}\", {len}, \"{digest}\")");
        assert!(source.contains(&row), "{name}: retained_wire_tests.rs ORDINARY_SHA256 no longer holds {row}");
    }
}

#[test]
fn desktop_request_publishes_the_milestone_ordinary_bytes() {
    for (mode, (name, len, digest)) in MODES.into_iter().zip(ORDINARY_SHA256) {
        assert_eq!(mode.as_str(), name);
        let request = json!({"model": desktop_model(), "materials": []});
        let envelope = run_linear_static_preview_value_with_mode(request, mode).unwrap();
        let bytes = serde_json::to_vec(&envelope).unwrap();
        assert_eq!((bytes.len(), sha(&bytes).as_str()), (len, digest), "{name}: milestone ordinary bytes");
    }
}

/// The variant is the request's model plus the display fields only: removing them gives
/// the request's model back, and each added field has its stated value.
#[test]
fn desktop_model_adds_only_display_fields() {
    let request: Value = serde_json::from_str(REQUEST).unwrap();
    assert_eq!(request["materials"], json!([]));
    let mut variant = desktop_model();
    let top = variant.as_object_mut().unwrap();

    let demo: Value =
        serde_json::from_str(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap();
    assert_eq!(top.remove("data_boundary").unwrap(), demo["data_boundary"]);
    assert_eq!(top.remove("components").unwrap(), json!([]));
    assert_eq!(top.remove("diagnostics").unwrap(), json!([]));

    let project = top["project"].as_object_mut().unwrap();
    assert_eq!(project.remove("name").unwrap(), json!("RF-SKEW-T-CANT-OFF-122-r1e-04 milestone (G10 witness)"));
    assert!(project.remove("description").unwrap().as_str().unwrap().contains("Ids, values and schema_version are unchanged."));

    for field in LABELLED {
        for entry in top[field].as_array_mut().unwrap() {
            let entry = entry.as_object_mut().unwrap();
            let label = entry.remove("label").unwrap_or_else(|| panic!("{field}: label missing"));
            assert_eq!(label, entry["id"], "{field}: label equals id");
        }
    }
    for case in top["load_cases"].as_array_mut().unwrap() {
        assert_eq!(case.as_object_mut().unwrap().remove("status").unwrap(), json!("preview_only"));
    }

    assert_eq!(variant, request["model"], "the variant differs from the request only by the display fields");
}

//! PF Timoshenko round trips (DEL-05-03 Design `pf-timoshenko.md` §5, "Readers and
//! round trips"): the committed product envelopes the downstream readers consume
//! (`fixtures/results/preview_physics_timoshenko_*_sparse.json`) are the merged
//! product's actual output, in sparse mode, for
//! - `straight`: the frozen `PFT-SB-L1-CM0-TIMOSHENKO` request, solved;
//! - `arc`: the frozen `PFT-ARC-TIP-TIMOSHENKO` request, solved, with its realized
//!   arc and the named arc disclosure;
//! - `partial_span_blocked`: the straight request with a partial-span wind load on
//!   its member, blocked by `PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED`.
//!
//! This test re-solves each and requires the committed bytes;
//! `PF_TIMOSHENKO_WRITE_ENVELOPES=1` rewrites them. They are example transport data
//! from the product, not oracles (the rows are judged in `pf_timoshenko_references.rs`).
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

// Model document 0.9.0 (DEL-02-01 Design model-document-0.9.0.md §3.6): the frozen 0.8.0 requests keep their
// bytes; each parsed copy is re-authored to the current version by its token (common/current_model.rs).
#[path = "common/current_model.rs"]
mod current_model;

fn current(text: &str) -> Value {
    let mut request: Value = serde_json::from_str(text).unwrap();
    current_model::reauthor_model(&mut request["model"]);
    request
}

const STRAIGHT: &str = include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json");
const ARC_TIP: &str = include_str!("../../../validation/references/pf_timoshenko/requests/PFT-ARC-TIP-TIMOSHENKO.request.json");
const TIMOSHENKO_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1";

/// The straight request with a partial-span wind load on its one member.
fn partial_span() -> Value {
    let mut request = current(STRAIGHT);
    let pipe = request["model"]["pipe_segments"][0]["id"].clone();
    request["model"]["load_cases"][0]["equivalent_static"] = json!({
        "wind": {"pressure": {"value": 480.0, "unit": "Pa"}, "shape_factor": {"value": 0.7, "unit": "1"},
            "direction": "global_y", "exposed_pipe_refs": [],
            "exposed_spans": [{"pipe_ref": pipe, "start_fraction": {"value": 0.25, "unit": "1"}, "end_fraction": {"value": 0.75, "unit": "1"}}]},
        "provenance": "invented PF Timoshenko test input"});
    request
}

#[test]
fn committed_pf_timoshenko_envelopes_are_the_products_output() {
    let cases = [
        ("straight", current(STRAIGHT), true),
        ("arc", current(ARC_TIP), true),
        ("partial_span_blocked", partial_span(), false),
    ];
    let write = std::env::var_os("PF_TIMOSHENKO_WRITE_ENVELOPES").is_some();
    for (name, request, solved) in cases {
        let envelope = run_linear_static_preview_value_with_mode(request, PreviewSolverMode::SparseInteractive).expect("admitted at the wire");
        assert_eq!(envelope.producer.semantic_contract_id, TIMOSHENKO_ID, "{name}");
        assert_eq!(envelope.status.mechanics == "MECHANICS_SOLVED", solved, "{name}: {:?}", envelope.diagnostics);
        let codes: Vec<&str> = envelope.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes.contains(&"PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI"), name == "arc", "{name}: {codes:?}");
        assert_eq!(codes.contains(&"PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED"), !solved, "{name}: {codes:?}");
        let bytes = serde_json::to_string_pretty(&envelope).unwrap() + "\n";
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../fixtures/results/preview_physics_timoshenko_{name}_sparse.json"));
        if write {
            std::fs::write(&path, &bytes).unwrap();
        }
        assert!(std::fs::read_to_string(&path).is_ok_and(|text| text == bytes), "{}: regenerate with PF_TIMOSHENKO_WRITE_ENVELOPES=1", path.display());
    }
}

//! S11-G T23 (S11G_GUARD revision 2.2 erratum E-1; ROOT's D22-1 condition and
//! the manager's reader-test ruling): the result_export source-blocks reader
//! accepts the zero-work formation decline entry.
//!
//! The captured envelope is produced live from S11-G's T19 construction (no
//! committed fixture): N05's sensitive torsion cantilever with two cases on
//! per-case modulus bases. Case A (cancelling tip torques, base basis) is
//! Sensitive and source-selected; case B (a uniform load cancelled at the tip
//! by authored nodal inputs, plus 0.2 N*m at the tip RZ) runs on an invented
//! soft temperature point (E = 1 Pa, G = 0.4 Pa), is Passed, and its load-row
//! formation guard fires. Case B's receipt entry is the zero-work decline:
//! `unsupported`, `source_validation` / `unsupported_family`, work
//! {limit 0, charged 0, rejected 0}, ordinary outcome `sensitive`. Inputs are
//! invented; no material, component, catalogue or code-rule data is used.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, PreviewSolverMode,
};
use open_pipe_stress_result_export::source_blocks;
use serde_json::{json, Value};

const INVENTED: &str = "invented_t3_s11g_test_input_not_library_data";

fn nodal(id: &str, node: &str, direction: &str, value: f64) -> Value {
    let moment = direction.starts_with('R');
    json!({"id": id, "category": if moment { "concentrated_moment" } else { "concentrated_force" },
        "target": {"type": "node", "node": node}, "direction": direction,
        "magnitude": {"value": value, "unit": if moment { "N*m" } else { "N" }},
        "dimension": if moment { "moment" } else { "force" }, "provenance": INVENTED})
}

fn request() -> Value {
    let mut model: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/numerical_sensitive_torsion_model.json"
    ))
    .unwrap();
    model["materials"][0]["temperature_points"] = json!([{
        "id": "point:soft", "temperature": {"value": 20.0, "unit": "degC"},
        "elastic_modulus": {"value": 1.0, "unit": "Pa"}, "shear_modulus": {"value": 0.4, "unit": "Pa"},
        "provenance": INVENTED}]);
    let w = 1e8_f64;
    model["load_cases"] = json!([
        {"id": "case", "label": "torques", "kind": "primitive_user_load", "provenance": INVENTED,
            "primitive_loads": [nodal("tip:0", "tip", "RX", 1e8), nodal("tip:1", "tip", "RX", 0.3), nodal("tip:2", "tip", "RX", -1e8)]},
        {"id": "case-b", "label": "tip noise", "kind": "primitive_user_load", "provenance": INVENTED,
            "modulus_basis_ref": "point:soft",
            "primitive_loads": [
                {"id": "case-b:w", "category": "distributed_force", "target": {"type": "element", "pipe": "pipe"},
                    "direction": "global_y", "magnitude": {"value": w, "unit": "N/m"},
                    "dimension": "force_per_length", "provenance": INVENTED},
                nodal("case-b:uy", "tip", "global_y", -w),
                nodal("case-b:rz", "tip", "RZ", w / 3.0 + 0.2)
            ]}
    ]);
    json!({"model": model, "materials": []})
}

#[test]
fn t23_source_blocks_reader_accepts_the_zero_work_formation_decline() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let request = request();
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode)
            .unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        let raw = serde_json::to_value(&envelope).unwrap();
        // Precondition (paths differ): case B's entry is the zero-work decline,
        // and case A is qualified, so the reader sees a real receipt with it.
        let cases = raw["source_block_recovery"]["body"]["cases"]
            .as_array()
            .expect("a receipt");
        let entry = |id: &str| {
            cases
                .iter()
                .find(|c| c["basis_ref"]["ref_id"] == json!(id))
                .unwrap()
        };
        assert_eq!(entry("case")["outcome"], json!("qualified"), "{mode:?}");
        let b = entry("case-b");
        assert_eq!(b["outcome"], json!("unsupported"), "{mode:?}: {b}");
        assert_eq!(
            b["failure"]["stage"],
            json!("source_validation"),
            "{mode:?}"
        );
        assert_eq!(
            b["failure"]["code"],
            json!("unsupported_family"),
            "{mode:?}"
        );
        assert_eq!(
            b["ordinary_attempt"]["outcome"],
            json!("sensitive"),
            "{mode:?}"
        );
        assert_eq!(b["work"]["limit"], json!(0), "{mode:?}");
        assert_eq!(b["work"]["charged"], json!(0), "{mode:?}");
        assert_eq!(
            b["work"]["rejected_reservation"],
            json!({"kind": "finite", "amount": 0}),
            "{mode:?}"
        );
        // The reader accepts the receipt, with and without the actual invocation.
        let invocation = json!({"request": request, "solver_mode": mode.as_str()});
        source_blocks::validate(&raw, Some(&invocation))
            .unwrap_or_else(|e| panic!("{mode:?}: reader refused with the invocation: {e}"));
        source_blocks::validate(&raw, None)
            .unwrap_or_else(|e| panic!("{mode:?}: reader refused without the invocation: {e}"));
    }
}

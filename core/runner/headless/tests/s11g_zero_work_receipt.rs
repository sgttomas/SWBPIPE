//! S11-G T23 (S11G_GUARD revision 2.2 erratum E-1; ROOT's D22-1 condition and
//! the manager's reader-test ruling), after B4/D1 (b): the preview route no
//! longer selects source blocks, so this construction publishes no source-block
//! receipt (its former zero-work decline entry went with source-blocks-1); the
//! S11-G formation-noise guard's disclosure is unchanged.
//!
//! The captured envelope is produced live from S11-G's T19 construction (no
//! committed fixture): N05's sensitive torsion cantilever with two cases on
//! per-case modulus bases. Case A (cancelling tip torques, base basis) is
//! Sensitive and source-selected. Case B (a uniform load cancelled at the tip
//! by authored nodal inputs, plus 0.2 N*m at the tip RZ and a 1e-10 N*m tip
//! torque) runs on an invented temperature point (E = 1 Pa, G = 1e-12 Pa). The
//! torque twists the torsionally soft pipe by about 3e6 rad, so K-D5's per-DOF
//! rule passes the tip:RZ formation defect on the closed residual (DEL-04-01
//! kd5-member-relative-bound.md §1) and case B is Passed, while its load-row
//! formation guard still fires. Case B's receipt entry is the zero-work
//! decline: `unsupported`, `source_validation` / `unsupported_family`, work
//! {limit 0, charged 0, rejected 0}, ordinary outcome `sensitive`. Inputs are
//! invented; no material, component, catalogue or code-rule data is used.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, PreviewSolverMode,
};
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
        "elastic_modulus": {"value": 1.0, "unit": "Pa"}, "shear_modulus": {"value": 1e-12, "unit": "Pa"},
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
                nodal("case-b:rz", "tip", "RZ", w / 3.0 + 0.2),
                nodal("case-b:rx", "tip", "RX", 1e-10)
            ]}
    ]);
    json!({"model": model, "materials": []})
}

#[test]
fn t23_the_preview_route_publishes_no_source_block_receipt() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let request = request();
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode)
            .unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        // Precondition: case B's report is Passed and the load-row guard
        // demotes it (the case the ordinary route never attempts).
        let integrity = envelope
            .diagnostics
            .iter()
            .find(|d| {
                d.code.starts_with("NUMERICAL_INTEGRITY_")
                    && d.affected_refs == vec!["case-b".to_string()]
            })
            .expect("case B's integrity diagnostic");
        assert_eq!(integrity.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{mode:?}");
        assert!(integrity.message.contains("quality: Passed"), "{mode:?}: {}", integrity.message);
        assert!(
            integrity.message.contains("S11-G formation-noise guard"),
            "{mode:?}: {}",
            integrity.message
        );
        let raw = serde_json::to_value(&envelope).unwrap();
        assert!(raw.get("source_block_recovery").is_none(), "{mode:?}");
        assert_eq!(raw["producer"]["semantic_contract_id"], json!("openpipestress.result_semantics/0.3.0/preview-physics-2"), "{mode:?}");
    }
}

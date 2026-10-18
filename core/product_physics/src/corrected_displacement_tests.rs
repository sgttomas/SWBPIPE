//! T4-U5b item 6 (HELP_HUMAN ruling (A), 2026-10-18): nodal displacements
//! publish the formation check's corrected displacement fl(u + w + w2)
//! exactly when the load case's certified member bound exists, and the
//! solve's own u otherwise. Both paths on pressure-2 and preview-physics-2
//! (the fallback forced by the test-only `U4A_FORCE_BINARY64` hook, which
//! removes the bound as a refused or unavailable bound does); in every run
//! each displacement magnitude is the correctly rounded norm of the published
//! components (the base G7 magnitude guard).
use super::*;
use serde_json::Value;

#[path = "../tests/common/current_model.rs"]
mod current_model;
#[path = "../tests/common/t4_u5b.rs"]
mod t4_u5b;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

fn solve(request: &Value, mode: PreviewSolverMode, force_binary64: bool) -> Value {
    U4A_FORCE_BINARY64.with(|flag| flag.set(force_binary64));
    let source = run_linear_static_preview_value_with_mode(request.clone(), mode);
    U4A_FORCE_BINARY64.with(|flag| flag.set(false));
    serde_json::to_value(source.unwrap()).unwrap()
}

fn value(source: &Value, id: &str) -> f64 {
    source["results"].as_array().unwrap().iter().find(|r| r["id"] == id)
        .unwrap_or_else(|| panic!("{id}"))["value"].as_f64().unwrap()
}

fn integrity(source: &Value) -> String {
    source["diagnostics"].as_array().unwrap().iter()
        .find(|d| d["code"].as_str().unwrap().starts_with("NUMERICAL_INTEGRITY_"))
        .map(|d| format!("{} {}", d["code"].as_str().unwrap(), d["message"].as_str().unwrap()))
        .unwrap_or_default()
}

/// Every node's magnitude row is the correctly rounded norm of its published
/// mm components.
fn assert_magnitudes(source: &Value, label: &str) {
    for row in source["results"].as_array().unwrap().iter().filter(|r| r["kind"] == "displacement_magnitude") {
        let id = row["id"].as_str().unwrap();
        let [x, y, z] = ["ux", "uy", "uz"].map(|c| value(source, &format!("{id}:{c}")));
        assert_eq!(row["value"].as_f64().unwrap().to_bits(), norm3(x, y, z).to_bits(), "{label}: {id}");
    }
}

/// pressure-2, AS-F-R: with the bound, node B's published translation and
/// rotation lie within t4_u5b's criterion of the exact solution in both modes;
/// the forced fallback publishes the solve's own u, binary64 member rows and
/// the unavailable-check sentence, and its node B x translation (sparse) is
/// over the criterion: the defect ruling (A) repairs.
#[test]
fn pressure_2_publishes_the_corrected_displacement_only_with_the_bound() {
    let oracle = t4_u5b::oracle();
    let case = &oracle["cases"]["AS-F-R"];
    let request = t4_u5b::request("AS-F-R", case);
    let node = &case["nodes"]["node:B"];
    let fraction = |source: &Value, kind: &str, component: usize| {
        let (tail, scale) = match kind { "translation_global_m" => (["ux", "uy", "uz"][component], 1e-3), _ => (["rx", "ry", "rz"][component], 1.0) };
        let observed = value(source, &format!("result:disp:node-B:{tail}")) * scale;
        let expected = t4_u5b::number(&node[kind][component]);
        (observed - expected).abs() / (1e-9 * expected.abs().max(1e-5))
    };
    for mode in MODES {
        let corrected = solve(&request, mode, false);
        assert!(integrity(&corrected).starts_with("NUMERICAL_INTEGRITY_CHECKS_PASSED"), "{mode:?}: {}", integrity(&corrected));
        for kind in ["translation_global_m", "rotation_global_rad"] {
            for component in 0..3 {
                let f = fraction(&corrected, kind, component);
                assert!(f <= 1.0, "{mode:?} node B {kind}[{component}]: {f:.3e} of the criterion");
            }
        }
        assert_magnitudes(&corrected, &format!("corrected {mode:?}"));
        let fallback = solve(&request, mode, true);
        assert!(integrity(&fallback).contains("member rows are binary64 recoveries"), "{mode:?}: {}", integrity(&fallback));
        assert_magnitudes(&fallback, &format!("fallback {mode:?}"));
        if mode == PreviewSolverMode::SparseInteractive {
            let f = fraction(&fallback, "translation_global_m", 0);
            assert!(f > 1.0, "the solve's own u at node B x: {f:.3e}");
        }
    }
}

fn nodal_bits(source: &Value) -> Vec<(String, u64)> {
    source["results"].as_array().unwrap().iter()
        .filter(|r| r["kind"].as_str().unwrap().starts_with("global_nodal_"))
        .map(|r| (r["id"].as_str().unwrap().to_string(), r["value"].as_f64().unwrap().to_bits()))
        .collect()
}

/// preview-physics-2, both paths. The bundled demo model (Passed) publishes
/// the corrected displacement, differing from the forced fallback's solve u
/// in last bits at some component. The F2a milestone request is Sensitive:
/// it has no certified bound, so its nodal rows are the solve's own u, bit
/// for bit the forced fallback's. Magnitudes stay consistent on every path.
#[test]
fn preview_physics_2_publishes_the_corrected_displacement_only_with_the_bound() {
    let demo: Value = serde_json::json!({
        "model": serde_json::from_str::<Value>(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap(),
        "materials": [],
    });
    let milestone: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap();
    for mode in MODES {
        let corrected = solve(&demo, mode, false);
        assert_eq!(corrected["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/preview-physics-2");
        assert!(integrity(&corrected).starts_with("NUMERICAL_INTEGRITY_CHECKS_PASSED"), "{mode:?}: {}", integrity(&corrected));
        let fallback = solve(&demo, mode, true);
        assert_magnitudes(&corrected, &format!("demo corrected {mode:?}"));
        assert_magnitudes(&fallback, &format!("demo fallback {mode:?}"));
        let (c, f) = (nodal_bits(&corrected), nodal_bits(&fallback));
        assert_eq!(c.iter().map(|r| &r.0).collect::<Vec<_>>(), f.iter().map(|r| &r.0).collect::<Vec<_>>());
        assert!(c.iter().zip(&f).any(|(a, b)| a.1 != b.1), "{mode:?}: the corrected and the solve's displacement agree in every bit");
        let sensitive = solve(&milestone, mode, false);
        assert_eq!(sensitive["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/preview-physics-2");
        assert!(integrity(&sensitive).starts_with("NUMERICAL_INTEGRITY_SENSITIVE"), "{mode:?}: {}", integrity(&sensitive));
        assert_eq!(nodal_bits(&sensitive), nodal_bits(&solve(&milestone, mode, true)), "{mode:?}: a case without the bound keeps the solve's u");
        assert_magnitudes(&sensitive, &format!("milestone {mode:?}"));
    }
}

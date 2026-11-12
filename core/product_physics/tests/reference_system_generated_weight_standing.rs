//! The maintained reference system (DEL-09-01, `fixtures/reference_system/`,
//! read only), whose generated self-weight records keep their sections in
//! millimetres, solved unmodified through PP's public entry: each of RS-SUS,
//! RS-OPE and RS-SAM publishes `NUMERICAL_INTEGRITY_CHECKS_PASSED` in both
//! solver modes, and no K-D5 formation finding is reported. While the check
//! re-formed the load from the record's millimetre values as metres it was
//! given a mass 10^6 times the one the load was formed from, and every case
//! was demoted to sensitive.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const FIXTURE: &str = include_str!("../../../fixtures/reference_system/reference_system.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const CASES: [&str; 3] = ["load:RS-SUS", "load:RS-OPE", "load:RS-SAM"];

#[test]
fn the_reference_systems_generated_weight_cases_publish_checks_passed() {
    let model: Value = serde_json::from_str(FIXTURE).unwrap();
    for mode in MODES {
        let envelope = serde_json::to_value(
            run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), mode).unwrap(),
        )
        .unwrap();
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
        let diagnostics = envelope["diagnostics"].as_array().unwrap();
        for case in CASES {
            let id = format!("diagnostic:numerical-integrity:{case}");
            let integrity: Vec<&Value> = diagnostics.iter().filter(|d| d["id"] == id).collect();
            assert_eq!(integrity.len(), 1, "{mode:?} {case}");
            assert_eq!(integrity[0]["code"], "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{mode:?} {case}: {}", integrity[0]["message"]);
            assert!(!integrity[0]["message"].as_str().unwrap().contains("formation_check: reason="), "{mode:?} {case}");
        }
        let quality = envelope["numerical_quality"]["cases"].as_array().unwrap();
        for case in CASES {
            let entry = quality.iter().find(|c| c["basis_ref"]["ref_id"] == case).unwrap();
            assert_eq!(entry["solve_quality"], "checks_passed", "{mode:?} {case}");
        }
    }
}

//! Review-triggered public coverage and deterministic-tie controls.
//! Authored by physics_manager after independent review identified the defects.
//! The extreme finite material is a binary64 boundary control, not a physical benchmark.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewSolverMode,
};
use serde_json::{json, Value};
use std::f64::consts::PI;

fn fixture() -> Value {
    json!({"model":{
        "schema_version":"0.3.0","document_kind":"openpipestress.product_preview.model",
        "pressure_contract":{"version":"2.0.0","mode":"exact_straight_pressure_v2"},
        "project":{"id":"project:elastic-extrema","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[{"id":"node:a","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"independent_statics"},{"id":"node:b","position":{"x":1.0,"y":0.0,"z":0.0},"provenance":"independent_statics"}],
        "pipe_segments":[{"id":"pipe:a-b","from":"node:a","to":"node:b","section":{"outside_diameter":{"value":0.12,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"material":"material:m","y_reference":{"x":0.0,"y":1.0,"z":0.0},"provenance":"independent_statics"}],
        "supports":[{"id":"support:a","node":"node:a","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"independent_statics"}],"components":[],
        "materials":[{"id":"material:m","constitutive_basis":"homogeneous_isotropic_E_nu_v1","elastic_modulus":{"value":200e9,"unit":"Pa"},"poisson_ratio":{"value":0.3,"unit":"1"},"provenance":"synthetic_isotropic_annulus"}],
        "load_cases":[{"id":"case:x1","pressure_regions":[],"primitive_loads":[],"provenance":"independent_statics"}],"combinations":[]},"materials":[]})
}
fn nodal(id: &str, direction: &str, value: f64, moment: bool) -> Value {
    json!({"id":id,"category":if moment {"concentrated_moment"} else {"concentrated_force"},"target":{"type":"node","node":"node:b"},"direction":direction,"magnitude":{"value":value,"unit":if moment {"N*m"} else {"N"}},"dimension":if moment {"moment"} else {"force"},"provenance":"independent_statics"})
}

// Finite binary64 edge control, not a realistic material or engineering benchmark.
// The two signed normal components are finite; their aggregate exceeds f64.
fn coverage_wire(two_members: bool) -> Value {
    let mut wire = fixture();
    wire["model"]["materials"][0]["elastic_modulus"]["value"] = json!(1e306);
    wire["model"]["load_cases"][0]["primitive_loads"] =
        json!([nodal("load:small", "global_x", 1000.0, false)]);
    let area = PI * (0.06_f64.powi(2) - 0.05_f64.powi(2));
    let z = PI * (0.06_f64.powi(4) - 0.05_f64.powi(4)) / 4.0 / 0.06;
    let mut force = nodal("load:large-force", "global_x", 0.7 * f64::MAX * area, false);
    let mut moment = nodal("load:large-moment", "rotation_z", 0.7 * f64::MAX * z, true);
    if two_members {
        for (id, x) in [("node:c", 0.0), ("node:d", 1.0)] {
            wire["model"]["nodes"].as_array_mut().unwrap().push(json!({"id":id,"position":{"x":x,"y":1.0,"z":0.0},"provenance":"finite_edge_control"}));
        }
        let mut pipe = wire["model"]["pipe_segments"][0].clone();
        pipe["id"] = json!("pipe:c-d");
        pipe["from"] = json!("node:c");
        pipe["to"] = json!("node:d");
        wire["model"]["pipe_segments"]
            .as_array_mut()
            .unwrap()
            .push(pipe);
        let mut support = wire["model"]["supports"][0].clone();
        support["id"] = json!("support:c");
        support["node"] = json!("node:c");
        wire["model"]["supports"]
            .as_array_mut()
            .unwrap()
            .push(support);
        force["target"]["node"] = json!("node:d");
        moment["target"]["node"] = json!("node:d");
        let loads = wire["model"]["load_cases"][0]["primitive_loads"]
            .as_array_mut()
            .unwrap();
        loads.push(force);
        loads.push(moment);
    } else {
        let mut second = wire["model"]["load_cases"][0].clone();
        second["id"] = json!("case:large");
        second["primitive_loads"] = json!([force, moment]);
        wire["model"]["load_cases"]
            .as_array_mut()
            .unwrap()
            .push(second);
    }
    wire
}
fn check_incomplete_domain(wire: Value, mode: PreviewSolverMode) {
    let request: LinearStaticPreviewRequest = serde_json::from_value(wire).unwrap();
    let output = run_linear_static_preview_with_mode(request, mode);
    assert_eq!(
        output.status.mechanics, "MECHANICS_SOLVED",
        "{:?}",
        output.diagnostics
    );
    assert!(output.results.iter().all(|r| r.value.is_finite()));
    assert!(output
        .diagnostics
        .iter()
        .any(|d| d.code == "EXACT_STRESS_GOVERNING_MAXIMUM_UNAVAILABLE"));
    assert_eq!(
        output
            .results
            .iter()
            .filter(|r| r.kind == "pipe_elastic_normal_stress_maximum_v2")
            .count(),
        1,
        "valid individual maximum must remain inspectable"
    );
    assert!(
        output.summary.max_open_formula_stress.is_none(),
        "incomplete domain must not publish an available-subset overall maximum"
    );
    let cases = output.contract_evidence.as_ref().unwrap()["exact_cases"]
        .as_array()
        .unwrap();
    assert!(cases
        .iter()
        .any(|case| case["stress_maximum_coverage"]["complete"] == false));
    for case in cases
        .iter()
        .filter(|case| case["stress_maximum_coverage"]["complete"] == false)
    {
        assert!(!case["stress_maximum_coverage"]["unavailable_pipe_ids"]
            .as_array()
            .unwrap()
            .is_empty());
    }
}
#[test]
fn incomplete_case_domain_withholds_overall_maximum_in_both_case_orders() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for reverse in [false, true] {
            let mut wire = coverage_wire(false);
            if reverse {
                wire["model"]["load_cases"]
                    .as_array_mut()
                    .unwrap()
                    .reverse();
            }
            check_incomplete_domain(wire, mode);
        }
    }
}
#[test]
fn incomplete_member_domain_withholds_case_maximum_in_both_member_orders() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for reverse in [false, true] {
            let mut wire = coverage_wire(true);
            if reverse {
                wire["model"]["pipe_segments"]
                    .as_array_mut()
                    .unwrap()
                    .reverse();
            }
            check_incomplete_domain(wire, mode);
        }
    }
}

#[test]
fn exact_zero_displacement_ties_use_location_identity_not_node_order() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for reverse in [false, true] {
            let mut wire = fixture();
            if reverse {
                wire["model"]["nodes"].as_array_mut().unwrap().reverse();
            }
            let request: LinearStaticPreviewRequest = serde_json::from_value(wire).unwrap();
            let output = run_linear_static_preview_with_mode(request, mode);
            assert_eq!(output.status.mechanics, "MECHANICS_SOLVED");
            assert_eq!(
                output
                    .summary
                    .max_displacement
                    .as_ref()
                    .unwrap()
                    .location_ref,
                "node:a"
            );
        }
    }
}
#[test]
fn exact_zero_stress_ties_use_location_identity_not_member_order() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for reverse in [false, true] {
            let mut wire = coverage_wire(true);
            wire["model"]["materials"][0]["elastic_modulus"]["value"] = json!(200e9);
            wire["model"]["load_cases"][0]["primitive_loads"] = json!([]);
            if reverse {
                wire["model"]["pipe_segments"]
                    .as_array_mut()
                    .unwrap()
                    .reverse();
            }
            let request: LinearStaticPreviewRequest = serde_json::from_value(wire).unwrap();
            let output = run_linear_static_preview_with_mode(request, mode);
            assert_eq!(output.status.mechanics, "MECHANICS_SOLVED");
            assert_eq!(
                output
                    .summary
                    .max_open_formula_stress
                    .as_ref()
                    .unwrap()
                    .location_ref,
                "pipe:a-b"
            );
        }
    }
}

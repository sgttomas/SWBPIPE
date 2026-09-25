//! Actual-entrypoint companion to independently derived STRESS_REFERENCE X1.
//! The construction was independently checked by ASSEMBLY_ORACLE before this test.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, MechanicsEnvelope,
    PreviewSolverMode,
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
fn solve(wire: Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let request: LinearStaticPreviewRequest = serde_json::from_value(wire).unwrap();
    let output = run_linear_static_preview_with_mode(request, mode);
    assert_eq!(
        output.status.mechanics, "MECHANICS_SOLVED",
        "{:?}",
        output.diagnostics
    );
    assert!(!output
        .diagnostics
        .iter()
        .any(|d| d.code == "EXACT_STRESS_GOVERNING_MAXIMUM_UNAVAILABLE"));
    output
}
fn close(a: f64, b: f64, scale: f64) {
    assert!(
        a.is_finite() && (a - b).abs() <= 1e-9 * b.abs().max(scale),
        "{a:.17e} != {b:.17e}"
    );
}
#[test]
fn actual_x1_loads_find_peak_missed_by_old_signed_sum_candidates() {
    let inertia = PI * (0.06_f64.powi(4) - 0.05_f64.powi(4)) / 4.0;
    let z = inertia / 0.06;
    let a = 1e6 * z;
    let expected = 1e6 * (71.0 + 8.0 * 2.0_f64.sqrt()).sqrt() / 8.0;
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let mut wire = fixture();
        wire["model"]["load_cases"][0]["primitive_loads"] = json!([
            {"id":"load:wz","category":"distributed_force","target":{"type":"element","pipe":"pipe:a-b"},"direction":"global_z","magnitude":{"value":8.0*a,"unit":"N/m"},"dimension":"force_per_length","provenance":"independent_statics"},
            nodal("load:fz","global_z",-4.0*a,false),nodal("load:fy","global_y",-a,false),nodal("load:mz","rotation_z",a,true)]);
        let output = solve(wire, mode);
        let maximum = output
            .results
            .iter()
            .find(|r| r.kind == "pipe_elastic_normal_stress_maximum_v2")
            .unwrap();
        close(maximum.value, expected, 0.0);
        assert!(maximum.value > 1e6 * 325.0_f64.sqrt() / 16.0);
        assert_eq!(maximum.unit, "Pa");
        assert_eq!(
            output
                .summary
                .max_open_formula_stress
                .as_ref()
                .unwrap()
                .result_ref,
            maximum.id
        );
        let evidence =
            &output.contract_evidence.as_ref().unwrap()["exact_cases"][0]["pipe_stress_extrema"][0];
        let lower = evidence["value_lower_pa"].as_f64().unwrap();
        let upper = evidence["global_upper_bound_pa"].as_f64().unwrap();
        assert!(upper >= lower);
        assert!(upper - lower <= 1.001e-12 * (1.0 + lower));
        assert_eq!(evidence["result_id"], maximum.id);
        assert_eq!(
            output.contract_evidence.as_ref().unwrap()["pressure"],
            json!([])
        );
        for (component, value) in [
            ("Fx", 0.0),
            ("Fy", a),
            ("Fz", -4.0 * a),
            ("Mx", 0.0),
            ("My", 0.0),
            ("Mz", 0.0),
        ] {
            let row = output
                .results
                .iter()
                .find(|r| {
                    r.kind == "support_reaction_component_v2"
                        && r.metadata.as_ref().unwrap().component == component
                })
                .unwrap();
            close(row.value, value, 8.0 * a);
        }
    }
}
#[test]
fn pure_torque_has_zero_normal_maximum_and_signed_torsion() {
    let inertia = PI * (0.06_f64.powi(4) - 0.05_f64.powi(4)) / 4.0;
    for torque in [-1000.0, 1000.0] {
        let mut wire = fixture();
        wire["model"]["load_cases"][0]["primitive_loads"] =
            json!([nodal("load:t", "rotation_x", torque, true)]);
        let output = solve(wire, PreviewSolverMode::SparseInteractive);
        let maximum = output
            .results
            .iter()
            .find(|r| r.kind == "pipe_elastic_normal_stress_maximum_v2")
            .unwrap();
        close(maximum.value, 0.0, 1.0);
        let torsion = output
            .results
            .iter()
            .find(|r| {
                r.kind == "element_local_torsional_shear_stress"
                    && r.metadata.as_ref().unwrap().location == "end_j"
            })
            .unwrap();
        close(torsion.value, torque * 0.06 / (2.0 * inertia) / 1e6, 0.0);
        let reaction = output
            .results
            .iter()
            .find(|r| {
                r.kind == "support_reaction_component_v2"
                    && r.entity_ref == "support:a"
                    && r.metadata.as_ref().is_some_and(|m| m.component == "Mx")
            })
            .unwrap();
        close(reaction.value, -torque, 0.0);
    }
}

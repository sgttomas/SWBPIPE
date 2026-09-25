//! Review-triggered public signed-reaction and spring-attribution controls.
//! Authored by physics_manager; expectations follow whole-body equilibrium and
//! the explicit spring laws, without calling a production reaction helper.
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

fn support_value(output: &MechanicsEnvelope, id: &str, component: &str) -> f64 {
    let row = output
        .results
        .iter()
        .find(|r| {
            r.kind == "support_reaction_component_v2"
                && r.entity_ref == id
                && r.metadata
                    .as_ref()
                    .is_some_and(|m| m.component == component)
        })
        .unwrap();
    let meta = row.metadata.as_ref().unwrap();
    assert_eq!(meta.coordinate_system, "global");
    assert_eq!(meta.location, "node");
    assert_eq!(
        row.unit,
        if component.starts_with('F') {
            "N"
        } else {
            "N*m"
        }
    );
    row.value
}
fn rotate(v: [f64; 3]) -> [f64; 3] {
    [
        (2.0 * v[0] - 2.0 * v[1] + v[2]) / 3.0,
        (v[0] + 2.0 * v[1] + 2.0 * v[2]) / 3.0,
        (-2.0 * v[0] - v[1] + 2.0 * v[2]) / 3.0,
    ]
}
#[test]
fn six_nonzero_support_components_follow_global_force_and_moment_balance() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for rotated in [false, true] {
            let mut wire = fixture();
            let map = |v| if rotated { rotate(v) } else { v };
            let tip = map([1.0, 0.0, 0.0]);
            let yref = map([0.0, 1.0, 0.0]);
            wire["model"]["nodes"][1]["position"] = json!({"x":tip[0],"y":tip[1],"z":tip[2]});
            wire["model"]["pipe_segments"][0]["y_reference"] =
                json!({"x":yref[0],"y":yref[1],"z":yref[2]});
            let force = map([1000.0, 2000.0, -3000.0]);
            let moment = map([400.0, -500.0, 600.0]);
            let mut loads = Vec::new();
            for axis in 0..3 {
                loads.push(nodal(
                    &format!("load:f{axis}"),
                    ["global_x", "global_y", "global_z"][axis],
                    force[axis],
                    false,
                ));
                loads.push(nodal(
                    &format!("load:m{axis}"),
                    ["rotation_x", "rotation_y", "rotation_z"][axis],
                    moment[axis],
                    true,
                ));
            }
            wire["model"]["load_cases"][0]["primitive_loads"] = json!(loads);
            let output = solve(wire, mode);
            // Independent whole-body balance about root: Rf=-F, Rm=-(M+r×F).
            let rf = map([-1000.0, -2000.0, 3000.0]);
            let rm = map([-400.0, -2500.0, -2600.0]);
            for axis in 0..3 {
                close(
                    support_value(&output, "support:a", ["Fx", "Fy", "Fz"][axis]),
                    rf[axis],
                    0.0,
                );
                close(
                    support_value(&output, "support:a", ["Mx", "My", "Mz"][axis]),
                    rm[axis],
                    0.0,
                );
            }
            for (kind, value, unit) in [
                (
                    "support_reaction_force_magnitude_v2",
                    1000.0_f64.hypot(2000.0).hypot(3000.0),
                    "N",
                ),
                (
                    "support_reaction_moment_magnitude_v2",
                    400.0_f64.hypot(2500.0).hypot(2600.0),
                    "N*m",
                ),
            ] {
                let row = output
                    .results
                    .iter()
                    .find(|r| r.kind == kind && r.entity_ref == "support:a")
                    .unwrap();
                close(row.value, value, 0.0);
                assert_eq!(row.unit, unit);
            }
        }
    }
}
#[test]
fn two_coincident_tip_springs_and_root_rigid_support_have_distinct_actions() {
    let ea = 200e9 * PI * (0.06_f64.powi(2) - 0.05_f64.powi(2));
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let mut wire = fixture();
        for (id, k) in [("support:s1", ea), ("support:s2", 2.0 * ea)] {
            wire["model"]["supports"].as_array_mut().unwrap().push(json!({"id":id,"node":"node:b","family":"spring","restraints":["UX"],"stiffness":{"dof":"UX","value":{"value":k,"unit":"N/m"}},"provenance":"explicit_parallel_spring_reference"}));
        }
        wire["model"]["load_cases"][0]["primitive_loads"] =
            json!([nodal("load:tip", "global_x", 1000.0, false)]);
        let output = solve(wire, mode);
        // u=F/(EA/L+k1+k2); each spring=-ki*u, root=-EA/L*u.
        for (id, expected) in [
            ("support:a", -250.0),
            ("support:s1", -250.0),
            ("support:s2", -500.0),
        ] {
            close(support_value(&output, id, "Fx"), expected, 0.0);
            for component in ["Fy", "Fz", "Mx", "My", "Mz"] {
                close(support_value(&output, id, component), 0.0, 1000.0);
            }
        }
        let ux = output
            .results
            .iter()
            .find(|r| {
                r.entity_ref == "node:b"
                    && r.metadata
                        .as_ref()
                        .is_some_and(|m| m.component == "nodal_displacement_x")
            })
            .unwrap();
        close(ux.value, 1000.0 / (4.0 * ea) * 1000.0, 0.0);
    }
}
#[test]
fn duplicate_rigid_devices_cannot_each_publish_the_complete_nodal_reaction() {
    let mut wire = fixture();
    let mut duplicate = wire["model"]["supports"][0].clone();
    duplicate["id"] = json!("support:duplicate");
    wire["model"]["supports"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    wire["model"]["load_cases"][0]["primitive_loads"] =
        json!([nodal("load:tip", "global_x", 1000.0, false)]);
    let output = run_linear_static_preview_with_mode(
        serde_json::from_value(wire).unwrap(),
        PreviewSolverMode::SparseInteractive,
    );
    assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE");
    assert!(output.results.is_empty());
    assert!(output
        .diagnostics
        .iter()
        .any(|d| d.code == "SUPPORT_INPUT_INVALID"));
}
#[test]
fn same_record_spring_cannot_silently_replace_an_extra_rigid_guide() {
    let mut wire = fixture();
    wire["model"]["supports"].as_array_mut().unwrap().push(json!({"id":"support:bad","node":"node:b","family":"spring","restraints":["UX","UY"],"stiffness":{"dof":"UX","value":{"value":1000.0,"unit":"N/m"}},"provenance":"ambiguous_support_control"}));
    let output = run_linear_static_preview_with_mode(
        serde_json::from_value(wire).unwrap(),
        PreviewSolverMode::SparseInteractive,
    );
    assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE");
    assert!(output.results.is_empty());
    assert!(output
        .diagnostics
        .iter()
        .any(|d| d.code == "SUPPORT_SPRING_AXIS_CONFLICT"));
}

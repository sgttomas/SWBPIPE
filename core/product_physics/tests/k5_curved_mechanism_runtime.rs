//! K5 (T3 D1 revision 5a.2 §4.9, W4; ROOT's K5 rulings Q1(b) and Q2(b)):
//! product pins through both entries (the captured value entry and the typed
//! entry) and both modes. Invented inputs only (a 0.12/0.01 m pipe, E = 200
//! GPa); the models are PP's `arc_model` shape with dyadic geometry, so the
//! arc is consistent in binary64 at any offset.
//!
//! On Mac main (I14's checkpoint-0 product run), the constructed curved
//! mechanism was refused `NUMERICAL_INTEGRITY_UNRESOLVED` (a pivot failure)
//! on both entries and in both modes; with W4 it is refused as a physical
//! mechanism with its witness (change class K5-C1). The same model with a
//! nonlinear support is never selected and keeps today's refusal.
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    LinearStaticPreviewRequest, MechanicsEnvelope, PreviewSolverMode,
};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn node(id: &str, x: f64, y: f64) -> Value {
    json!({"id": id, "position": {"x": x, "y": y, "z": 0.0}, "provenance": "k5"})
}
fn pipe(id: &str, from: &str, to: &str, yref: [f64; 3]) -> Value {
    json!({"id": id, "from": from, "to": to,
        "section": {"outside_diameter": {"value": 0.12, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}},
        "material": "material:m", "y_reference": {"x": yref[0], "y": yref[1], "z": yref[2]}, "provenance": "k5"})
}
fn support(id: &str, node: &str, restraints: &[&str]) -> Value {
    json!({"id": id, "node": node, "family": "anchor", "restraints": restraints, "provenance": "k5"})
}
fn force(id: &str, node: &str, dir: &str, v: f64) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": node},
        "direction": dir, "magnitude": {"value": v, "unit": "N"}, "dimension": "force", "provenance": "k5"})
}

/// a (o, o) – b (o+1, o) straight; b – c (o+1.25, o+0.25) a realized 90°
/// bend of radius 0.25 m; c – d (o+1.25, o+1.25) straight. Translation pins
/// at a and d: the rotation about the line a–d is free. `rx` adds RX at a;
/// `gap` adds an open gap support at b (a nonlinear support).
fn model(o: f64, rx: bool, gap: bool) -> Value {
    model_with(o, 0.25, rx, gap, false)
}

/// `model` with bend radius r (c = (o+1+r, o+r), d = (o+1+r, o+1+r)); `x_case`
/// adds the load case `case:x` (1 kN along global x at b), as I14's product-run
/// harness builds its constructed mechanisms (`_run_records/a2/`).
fn model_with(o: f64, r: f64, rx: bool, gap: bool, x_case: bool) -> Value {
    let bend = json!({"id": "component:bend", "label": "k5 arc", "kind": "bend", "node": "node:c",
        "geometry": {"bend_pipe_ref": "pipe:b-c", "bend_radius": {"value": r, "unit": "m"},
            "bend_angle": {"value": std::f64::consts::FRAC_PI_2, "unit": "rad"},
            "bend_plane_orientation": "global_xy_preview", "bend_geometry_source_reference": "k5"},
        "modifiers": {"flexibility_factor_user_value": {"value": 2.0, "unit": "none"}, "source_reference": "k5_invented"},
        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element",
            "rule_check_consumption": "user_rule_pack_inputs_only"},
        "provenance": "k5"});
    let pin_a: &[&str] = if rx {
        &["UX", "UY", "UZ", "RX"]
    } else {
        &["UX", "UY", "UZ"]
    };
    let mut supports = vec![
        support("support:pa", "node:a", pin_a),
        support("support:pd", "node:d", &["UX", "UY", "UZ"]),
    ];
    if gap {
        supports.push(
            json!({"id": "support:gap", "node": "node:b", "family": "nonlinear", "restraints": [],
            "nonlinear": {"behavior": "gap", "dof": "UZ", "initial_state": "inactive",
                "closes_when": "positive_displacement", "gap": {"value": 1000.0, "unit": "mm"}},
            "provenance": "k5 invented: an open gap support that never closes"}),
        );
    }
    let mut load_cases = vec![json!({"id": "case:z",
        "primitive_loads": [force("z", "node:b", "global_z", 1000.0)], "provenance": "k5"})];
    if x_case {
        load_cases.push(json!({"id": "case:x",
            "primitive_loads": [force("x", "node:b", "global_x", 1000.0)], "provenance": "k5"}));
    }
    json!({"model": {"schema_version": "0.2.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:k5", "units": {"length": "m", "force": "N", "angle": "rad",
            "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [node("node:a", o, o), node("node:b", o + 1.0, o), node("node:c", o + 1.0 + r, o + r),
            node("node:d", o + 1.0 + r, o + 1.0 + r)],
        "pipe_segments": [pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0]),
            pipe("pipe:b-c", "node:b", "node:c", [std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2, 0.0]),
            pipe("pipe:c-d", "node:c", "node:d", [1.0, 0.0, 0.0])],
        "supports": supports,
        "components": [bend],
        "materials": [{"id": "material:m", "elastic_modulus": {"value": 200e9, "unit": "Pa"},
            "shear_modulus": {"value": 76.923076923e9, "unit": "Pa"},
            "thermal_expansion_coefficient": {"value": 1.2e-5, "unit": "1/degC"},
            "temperature_points": [{"id": "point:hot", "temperature": {"value": 300.0, "unit": "degC"},
                "elastic_modulus": {"value": 180e9, "unit": "Pa"}, "shear_modulus": {"value": 69.2e9, "unit": "Pa"}}],
            "provenance": "k5_invented_test_values"}],
        "load_cases": load_cases,
        "combinations": []},
        "materials": []})
}

/// Both entries, one mode.
fn both_entries(value: &Value, mode: PreviewSolverMode) -> [(&'static str, MechanicsEnvelope); 2] {
    let captured = run_linear_static_preview_value_with_mode(value.clone(), mode).unwrap();
    let typed_request: LinearStaticPreviewRequest = serde_json::from_value(value.clone()).unwrap();
    [
        ("captured", captured),
        (
            "typed",
            run_linear_static_preview_with_mode(typed_request, mode),
        ),
    ]
}

fn blocking(envelope: &MechanicsEnvelope) -> Vec<(String, String)> {
    envelope
        .diagnostics
        .iter()
        .filter(|d| d.severity == "blocking")
        .map(|d| (d.code.clone(), d.message.clone()))
        .collect()
}

/// The witness direction in a `NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM`
/// message (`Mechanism { direction: [...] }`, printed with `{:?}`, which
/// round-trips every binary64 exactly).
fn direction(message: &str) -> Vec<f64> {
    let start = message.find("direction: [").expect("direction") + "direction: [".len();
    let end = start + message[start..].find(']').unwrap();
    message[start..end]
        .split(", ")
        .map(|v| v.parse().unwrap())
        .collect()
}

/// K5-C1 (Q1(b), Q2(b)): the curved mechanism is refused as a physical
/// mechanism with W4's witness on both entries and in both modes, at the
/// origin and at 5e6 m. The witness is exact: one rotation θ at every node,
/// zero translation at the pins a and d, and u(x) − u(a) = θ × (x − a) exactly
/// at every node. Its bits are the same on every entry, mode and offset, and
/// equal the derived θ = (1, 1, 0), u(b) = u(c) = (0, 0, −1) (W4 calls no
/// libm function; the coordinates are dyadic).
#[test]
fn k5_curved_mechanism_is_refused_as_a_physical_mechanism_with_its_witness() {
    let expected: Vec<f64> = [
        [0.0, 0.0, 0.0, 1.0, 1.0, 0.0],
        [0.0, 0.0, -1.0, 1.0, 1.0, 0.0],
        [0.0, 0.0, -1.0, 1.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0, 1.0, 0.0],
    ]
    .concat();
    for o in [0.0, 5.0e6] {
        let points = [
            [o, o, 0.0],
            [o + 1.0, o, 0.0],
            [o + 1.25, o + 0.25, 0.0],
            [o + 1.25, o + 1.25, 0.0],
        ];
        for mode in MODES {
            for (entry, envelope) in both_entries(&model(o, false, false), mode) {
                let ctx = format!("{o} {entry} {mode:?}");
                assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE", "{ctx}");
                let refusals = blocking(&envelope);
                assert_eq!(refusals.len(), 1, "{ctx}: {refusals:?}");
                assert_eq!(
                    refusals[0].0, "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM",
                    "{ctx}"
                );
                let d = direction(&refusals[0].1);
                assert_eq!(d.len(), 24, "{ctx}");
                let theta = [d[3], d[4], d[5]];
                assert!(theta.iter().any(|&t| t != 0.0), "{ctx}");
                for n in 0..4 {
                    assert_eq!(&d[6 * n + 3..6 * n + 6], &theta, "{ctx}");
                    for k in 0..3 {
                        let (k1, k2) = ((k + 1) % 3, (k + 2) % 3);
                        let mut acc = ExactAccumulator::new();
                        acc.add(d[6 * n + k]).unwrap();
                        acc.add(-d[k]).unwrap();
                        acc.add_product(-theta[k1], points[n][k2]).unwrap();
                        acc.add_product(theta[k1], points[0][k2]).unwrap();
                        acc.add_product(theta[k2], points[n][k1]).unwrap();
                        acc.add_product(-theta[k2], points[0][k1]).unwrap();
                        assert!(acc.is_zero(), "{ctx}: node {n} component {k}");
                    }
                }
                assert_eq!(&d[0..3], &[0.0; 3], "{ctx}: pinned a");
                assert_eq!(&d[18..21], &[0.0; 3], "{ctx}: pinned d");
                assert_eq!(
                    d.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    "{ctx}"
                );
                assert!(envelope.results.is_empty(), "{ctx}: no rows are published");
            }
        }
    }
}

/// The stabilized companion (RX at a) is restrained: W4 adds nothing and the
/// solve publishes as today, on both entries and in both modes.
#[test]
fn k5_curved_mechanism_companion_is_solved() {
    for o in [0.0, 5.0e6] {
        for mode in MODES {
            for (entry, envelope) in both_entries(&model(o, true, false), mode) {
                assert_eq!(
                    envelope.status.mechanics, "MECHANICS_SOLVED",
                    "{o} {entry} {mode:?}"
                );
                assert!(blocking(&envelope).is_empty(), "{o} {entry} {mode:?}");
            }
        }
    }
}

/// Q1(b): with a nonlinear support the invocation is never selected, W4 does
/// not run, and the refusal is today's (`NUMERICAL_INTEGRITY_UNRESOLVED`, a
/// pivot failure on Mac main), never a physical mechanism.
#[test]
fn k5_curved_mechanism_with_a_nonlinear_support_keeps_todays_refusal() {
    for o in [0.0, 5.0e6] {
        for mode in MODES {
            for (entry, envelope) in both_entries(&model(o, false, true), mode) {
                let codes = blocking(&envelope)
                    .into_iter()
                    .map(|b| b.0)
                    .collect::<Vec<_>>();
                assert_eq!(
                    codes,
                    vec!["NUMERICAL_INTEGRITY_UNRESOLVED".to_string()],
                    "{o} {entry} {mode:?}"
                );
            }
        }
    }
}

/// RV14-3: K5-C2 is a published change class. I14's product input
/// `constructed_mechanism_r0.2_o0` (R = 0.2 m at the origin) is an exact
/// mechanism whose canonical witness is not representable (u(c) needs
/// 0.2 − 1.2, 54 significant bits). Mac main refused it with a pivot failure
/// ("nonpositive or cancellation-unresolved structural pivot"); W4 refuses it
/// as unresolved with its own reason, on both entries and in both modes.
#[test]
fn k5_curved_mechanism_without_a_representable_witness_is_unresolved() {
    let value = model_with(0.0, 0.2, false, false, true);
    let reason =
        r#"NumericallyUnresolved { reason: "constrained-body rank unresolved", global_dof: None }"#;
    for mode in MODES {
        for (entry, envelope) in both_entries(&value, mode) {
            let ctx = format!("{entry} {mode:?}");
            assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE", "{ctx}");
            let refusals = blocking(&envelope);
            assert!(!refusals.is_empty(), "{ctx}");
            for (code, message) in &refusals {
                assert_eq!(code, "NUMERICAL_INTEGRITY_UNRESOLVED", "{ctx}");
                assert!(message.contains(reason), "{ctx}: {message}");
            }
            assert!(envelope.results.is_empty(), "{ctx}: no rows are published");
        }
    }
}

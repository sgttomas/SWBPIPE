//! K-D5 §3.2 on the ordinary preview route: every public ordinary maximum row
//! publishes the global maximum enclosure, `value_upper_pa ==
//! global_upper_bound_pa`, the row value is the enclosure midpoint the Rust
//! reader requires, and the published upper contains the member's own
//! published normal-stress station samples by the exact squared circular
//! test. The models are invented UDL lines
//! (one with two bends), not library, catalog or code-rule data.
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_product_physics::{run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewSolverMode};
use serde_json::{json, Value};

const P: &str = "invented K-D5 maximum-enclosure control; not library, catalog or code-rule data";

fn node(id: &str, x: f64, y: f64, z: f64) -> Value {
    json!({"id":id,"position":{"x":x,"y":y,"z":z},"provenance":P})
}
fn pipe(id: &str, from: &str, to: &str, y: [f64; 3]) -> Value {
    json!({"id":id,"from":from,"to":to,"material":"mat","y_reference":{"x":y[0],"y":y[1],"z":y[2]},
        "section":{"outside_diameter":{"value":0.168,"unit":"m"},"wall_thickness":{"value":0.007,"unit":"m"}},"provenance":P})
}
fn anchor(id: &str, node: &str) -> Value {
    json!({"id":id,"node":node,"family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":P})
}
fn udl(pipe: &str, direction: &str, value: f64) -> Value {
    json!({"id":format!("w:{pipe}:{direction}"),"category":"distributed_force","target":{"type":"element","pipe":pipe},"direction":direction,
        "magnitude":{"value":value,"unit":"N/m"},"dimension":"force_per_length","provenance":P})
}
fn request(id: &str, nodes: Vec<Value>, pipes: Vec<Value>, supports: Vec<Value>, loads: Vec<Value>) -> LinearStaticPreviewRequest {
    serde_json::from_value(json!({"model":{"schema_version":"0.8.0","document_kind":"openpipestress.product_preview.model",
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "project":{"id":id,"units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},
        "nodes":nodes,"pipe_segments":pipes,
        "materials":[{"id":"mat","elastic_modulus":{"value":200e9,"unit":"Pa"},"shear_modulus":{"value":80e9,"unit":"Pa"},"thermal_expansion_coefficient":{"value":1.2e-5,"unit":"1/degC"},"provenance":P}],
        "supports":supports,"components":[],
        "load_cases":[{"id":"case","label":id,"kind":"primitive_user_load","primitive_loads":loads,"provenance":P}],"combinations":[]},"materials":[]}))
    .unwrap()
}
/// An invented U line at UTM-like coordinates (two 90-degree bends) whose
/// middle-leg end maxima tie, so the witness interval upper lies below the
/// global upper bound in both modes. That precondition is internal (the
/// published evidence carries the global upper only); the crate's
/// `kd5_maximum_witness_tests` asserts it on this same fixture.
const U_LINE: &str = include_str!("fixtures/kd5_maximum_u_line.json");
fn models() -> Vec<(&'static str, LinearStaticPreviewRequest)> {
    vec![
        ("U line with a middle-leg tie", serde_json::from_str(U_LINE).unwrap()),
        ("fixed-fixed skew UDL line", request("kd5-max:fixed-fixed",
            vec![node("N0", 0.0, 0.0, 0.0), node("N1", 2.5, 1.0, 0.5), node("N2", 6.0, 2.0, 1.0)],
            vec![pipe("P0", "N0", "N1", [0.0, 0.0, 1.0]), pipe("P1", "N1", "N2", [0.0, 0.0, 1.0])],
            vec![anchor("aN0", "N0"), anchor("aN2", "N2")],
            vec![udl("P0", "global_z", -450.0), udl("P1", "global_z", -450.0), udl("P1", "global_y", 120.0)])),
        ("L line with UDL legs", request("kd5-max:l-line",
            vec![node("N0", 0.0, 0.0, 0.0), node("N1", 4.0, 0.0, 0.0), node("N2", 4.0, 3.0, 0.0)],
            vec![pipe("P0", "N0", "N1", [0.0, 0.0, 1.0]), pipe("P1", "N1", "N2", [0.0, 0.0, 1.0])],
            vec![anchor("aN0", "N0"), anchor("aN2", "N2")],
            vec![udl("P0", "global_z", -900.0), udl("P1", "global_z", -300.0), udl("P1", "global_x", 250.0)])),
    ]
}

/// |x| MPa in Pa as an exact binary64 pair: h + l = |x|*1e6 exactly
/// (TwoProduct by fused multiply-add; exact away from the subnormal range,
/// which the assertion keeps).
fn pa(x: f64) -> (f64, f64) {
    let h = x.abs() * 1e6;
    let l = x.abs().mul_add(1e6, -h);
    assert!(x == 0.0 || h > 2f64.powi(-900), "{x:e}: outside the exact TwoProduct range");
    (h, l)
}

/// Adds sign*(h + l)^2 exactly.
fn add_square(acc: &mut ExactAccumulator, (h, l): (f64, f64), sign: f64) {
    acc.add_product(sign * h, h).unwrap();
    acc.add_product(sign * 2.0 * h, l).unwrap();
    acc.add_product(sign * l, l).unwrap();
}

/// The strict circular coverage test of §4 ("Public samples and units"): a
/// finite global upper U covers a published sample (axial a, bending y and
/// z, in MPa) exactly when U - |a| >= 0 and (U - |a|)^2 >= y^2 + z^2, every
/// operand the exact rational value of the published binary64 MPa times
/// 10^6 Pa/MPa. Evaluated as exact sums of binary64 products: no rounded
/// square root, unit product or display value decides it.
fn covers_circular(upper: f64, a: f64, y: f64, z: f64) -> bool {
    let a = pa(a);
    let mut difference = ExactAccumulator::new();
    difference.add(upper).unwrap();
    difference.add(-a.0).unwrap();
    difference.add(-a.1).unwrap();
    if difference.signum() < 0 {
        return false;
    }
    // (U - A)^2 - Y^2 - Z^2 = U^2 - 2UA + A^2 - Y^2 - Z^2.
    let mut squares = ExactAccumulator::new();
    squares.add_product(upper, upper).unwrap();
    squares.add_product(-2.0 * upper, a.0).unwrap();
    squares.add_product(-2.0 * upper, a.1).unwrap();
    add_square(&mut squares, a, 1.0);
    add_square(&mut squares, pa(y), -1.0);
    add_square(&mut squares, pa(z), -1.0);
    squares.signum() >= 0
}

#[test]
fn circular_coverage_is_the_exact_squared_test() {
    // A 3-4-5 sample: U = 6 MPa covers |a| = 1, hypot(3, 4) = 5 exactly, but
    // not one Pa less; the old necessary condition U >= |a| + max(|y|,|z|)
    // passes U = 5 MPa, which the exact test refuses.
    assert!(covers_circular(6e6, -1.0, 3.0, -4.0));
    assert!(!covers_circular(6e6 - 1.0, -1.0, 3.0, -4.0));
    assert!(5e6 >= 1e6 + 4e6 && !covers_circular(5e6, 1.0, 3.0, 4.0));
    assert!(!covers_circular(0.5e6, 1.0, 0.0, 0.0));
    assert!(covers_circular(0.0, 0.0, 0.0, 0.0));
}

#[test]
fn kd5_preview_maximum_publishes_global_enclosure_midpoint_and_covers_public_samples() {
    for (label, base) in models() {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let envelope = run_linear_static_preview_with_mode(base.clone(), mode);
            let integrity: Vec<_> = envelope.diagnostics.iter().filter(|d| d.code.starts_with("NUMERICAL_INTEGRITY_")).map(|d| d.code.clone()).collect();
            assert_eq!(integrity, vec!["NUMERICAL_INTEGRITY_CHECKS_PASSED".to_string()], "{label} {mode:?}: precondition, a certified Passed case");
            let evidence = serde_json::to_value(&envelope.contract_evidence).unwrap();
            // T4-U4b (§2.3): arcs publish their own maximum, under the arc record's tokens.
            let extrema: Vec<Value> = evidence["preview_cases"].as_array().unwrap().iter()
                .flat_map(|case| case["pipe_stress_extrema"].as_array().cloned().unwrap_or_default())
                .filter(|x| x["approximation"] == "piecewise_quadratic_straight_section_statics").collect();
            let straight_ids: Vec<&str> = extrema.iter().map(|x| x["result_id"].as_str().unwrap()).collect();
            let rows: Vec<_> = envelope.results.iter().filter(|r| r.kind == "pipe_elastic_normal_stress_maximum_v2" && straight_ids.contains(&r.id.as_str())).collect();
            let straight = if label.starts_with("U line") { 3 } else { 2 };
            assert_eq!(extrema.len(), straight, "{label} {mode:?}: one maximum per straight member");
            assert_eq!(rows.len(), extrema.len(), "{label} {mode:?}");
            for x in &extrema {
                let pipe_id = x["pipe_id"].as_str().unwrap();
                let lower = x["value_lower_pa"].as_f64().unwrap();
                let upper = x["value_upper_pa"].as_f64().unwrap();
                let global = x["global_upper_bound_pa"].as_f64().unwrap();
                // §3.2: the public upper is the global maximum enclosure, never the
                // local witness interval or a midpoint below it.
                assert_eq!(upper.to_bits(), global.to_bits(), "{label} {mode:?} {pipe_id}: value_upper_pa must equal global_upper_bound_pa");
                assert!(lower.is_finite() && lower <= upper, "{label} {mode:?} {pipe_id}");
                let row = rows.iter().find(|r| r.entity_ref == pipe_id).unwrap();
                assert_eq!(row.value.to_bits(), (lower + 0.5 * (upper - lower)).to_bits(), "{label} {mode:?} {pipe_id}: the row is the global enclosure midpoint");
                // The strict public-sample check (`covers_circular`).
                let component = |kind: &str, location: &str| envelope.results.iter().find(|r| r.entity_ref == pipe_id && r.kind == kind
                    && r.metadata.as_ref().is_some_and(|m| m.location == location)).map(|r| r.value);
                let mut samples = 0;
                for location in ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"] {
                    let (Some(a), Some(y), Some(z)) = (component("element_local_axial_normal_stress", location),
                        component("element_local_bending_normal_stress_y", location), component("element_local_bending_normal_stress_z", location)) else { continue };
                    samples += 1;
                    assert!(covers_circular(upper, a, y, z), "{label} {mode:?} {pipe_id} {location}: the public upper misses a published station sample");
                }
                assert!(samples >= 3, "{label} {mode:?} {pipe_id}: published station samples found: {samples}");
            }
        }
    }
}

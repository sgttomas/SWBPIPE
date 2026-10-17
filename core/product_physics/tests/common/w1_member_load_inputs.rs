//! Additive W1 stage-2 controls. These invented inputs do not replace the
//! independently pinned nodal-only B1/SQ inputs or serve as physics references.
#![allow(dead_code)]
use serde_json::{json, Value};

pub const CASES: usize = 3;
pub const TERMS_PER_CASE: usize = 128;
pub const BASIS_SCALES: [f64; CASES] = [1.0, 1.25, 0.8];
pub const PROVENANCE: &str = "invented_w1_member_cap_control_no_library_data";

pub fn basis_id(case: usize) -> String {
    format!("w1-member-basis:{case}")
}

/// Retain the supplied geometry and material roster. Replace its first case
/// with three cases of exactly 128 full-span member loads each (no nodal
/// leaves), and supply three distinct, valid named modulus bases. Existing
/// 16-point rosters remain 16 points; the first three points are replaced.
pub fn three_basis_member_cases(mut raw: Value) -> Value {
    let first = raw["model"]["load_cases"][0].clone();
    let members: Vec<String> = raw["model"]["pipe_segments"].as_array().unwrap().iter()
        .map(|member| member["id"].as_str().unwrap().to_owned()).collect();
    assert!(!members.is_empty());
    for materials in [&["model", "materials"][..], &["materials"][..]] {
        let mut roster = &mut raw;
        for key in materials { roster = &mut roster[*key]; }
        if let Some(roster) = roster.as_array_mut() {
            for material in roster {
                let e = material["elastic_modulus"]["value"].as_f64().unwrap();
                let g = material["shear_modulus"]["value"].as_f64().unwrap();
                if !material["temperature_points"].is_array() {
                    material["temperature_points"] = json!([]);
                }
                let points = material["temperature_points"].as_array_mut().unwrap();
                while points.len() < CASES { points.push(Value::Null); }
                for case in 0..CASES {
                    points[case] = json!({"id": basis_id(case),
                        "temperature": {"value": 20.0 + case as f64, "unit": "degC"},
                        "elastic_modulus": {"value": e * BASIS_SCALES[case], "unit": "Pa"},
                        "shear_modulus": {"value": g * BASIS_SCALES[case], "unit": "Pa"},
                        "provenance": PROVENANCE});
                }
            }
        }
    }
    raw["model"]["load_cases"] = Value::Array((0..CASES).map(|case| {
        let mut value = first.clone();
        value["id"] = json!(format!("w1-member-case:{case}"));
        value["modulus_basis_ref"] = json!(basis_id(case));
        value["provenance"] = json!(PROVENANCE);
        value["primitive_loads"] = Value::Array((0..TERMS_PER_CASE).map(|term| json!({
            "id": format!("w1-member:{case}:{term}"), "category": "distributed_force",
            "target": {"type": "element", "pipe": members[term % members.len()]},
            "direction": "global_x", "dimension": "force_per_length",
            "magnitude": {"value": (case + 1) as f64 / TERMS_PER_CASE as f64, "unit": "N/m"},
            "provenance": PROVENANCE
        })).collect());
        value
    }).collect());
    raw
}

/// This executable control has two nodes and one member. It reaches all three
/// case/basis/load caps, but does not claim the ring input's geometry caps.
pub fn narrow_three_basis_member_cases() -> Value {
    let model: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/numerical_sensitive_torsion_model.json")).unwrap();
    three_basis_member_cases(json!({"model": model, "materials": []}))
}

/// A second invented control makes the ideal axial response a power of two:
/// with L=2, 128 terms of E_case*A_hat*2^-28 give u_tip=2^-20 m.
/// A_hat comes from the actual maintained closed section preparation; this is
/// input construction, not an independent section or response oracle.
pub fn narrow_dyadic_three_basis_member_cases() -> Value {
    let mut raw = narrow_three_basis_member_cases();
    let material = &mut raw["model"]["materials"][0];
    material["elastic_modulus"]["value"] = json!(2f64.powi(30));
    material["shear_modulus"]["value"] = json!(2f64.powi(29));
    for case in 0..CASES {
        material["temperature_points"][case]["elastic_modulus"]["value"] = json!(2f64.powi(30 + case as i32));
        material["temperature_points"][case]["shear_modulus"]["value"] = json!(2f64.powi(29 + case as i32));
    }
    let d = raw["model"]["pipe_segments"][0]["section"]["outside_diameter"]["value"].as_f64().unwrap();
    let t = raw["model"]["pipe_segments"][0]["section"]["wall_thickness"]["value"].as_f64().unwrap();
    let prepared = open_pipe_stress_frame_kernel::structural::retained_api::prepare_product_annulus(d, t);
    let area = prepared.result().unwrap().section_bits().values()[0];
    for case in 0..CASES {
        let intensity = 2f64.powi(30 + case as i32) * area * 2f64.powi(-28);
        for load in raw["model"]["load_cases"][case]["primitive_loads"].as_array_mut().unwrap() {
            load["magnitude"]["value"] = json!(intensity);
        }
    }
    raw
}

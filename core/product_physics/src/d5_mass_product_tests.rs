//! T4-U6 (DEL-05-03 Design `section-bases-and-timoshenko.md` §1, "D-5
//! mass"): generated self-weight on an exact-contract document. These are
//! product checks of the mass basis (which wall, which bore, which method,
//! which inputs, which validation) against independent binary64 arithmetic in
//! the test; they do not compare product output with the frozen references.
use super::*;
use crate::certified_loads::supplied_mutant;
use crate::self_weight::{
    generate_self_weight_operations, inspect_applied_self_weight, AppliedSelfWeightState, SelfWeightGravity,
    SelfWeightRequest, EXACT_MASS_METHOD, GENERATED_METHOD, STABLE_MASS_METHOD,
};
use open_pipe_stress_frame_kernel::structural::{IntensityPrimitive, LoadFormula, LoadPrimitive};
use serde_json::{json, Value};
use std::f64::consts::PI;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const INVENTED: &str = "invented_t4_u6_d5_mass_test_not_library_data";
const CASE: &str = "case:weight";
const PIPE: &str = "pipe";
// Outside diameter, nominal wall, mill tolerance, corrosion allowance (m),
// pipe and contents densities (kg/m^3) and gravity (m/s^2, global_y).
const OD: f64 = 0.2191;
const WALL: f64 = 0.00818;
const MILL: f64 = 0.00102;
const ALLOWANCE: f64 = 0.0015;
const STEEL: f64 = 7850.0;
const WATER: f64 = 1000.0;
const GRAVITY: f64 = -9.80665;

/// A minimal exact-contract 0.7.0 request: one straight member anchored at
/// its root, with the given mill tolerance and allowance and no load case.
fn document(mill: f64, allowance: f64) -> Value {
    json!({"materials": [], "model": {
        "schema_version": "0.7.0",
        "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "invented:d5-mass", "units": {"angle": "rad", "force": "N", "length": "m",
            "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "pressure_contract": {"mode": "exact_pressure_v3", "version": "3.0.0", "shear_model": "euler_bernoulli"},
        "nodes": [{"id": "root", "position": {"x": 0, "y": 0, "z": 0}, "provenance": INVENTED},
                  {"id": "tip", "position": {"x": 3, "y": 0, "z": 0}, "provenance": INVENTED}],
        "materials": [{"id": "steel", "label": "Invented steel", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "poisson_ratio": {"value": 0.25, "unit": "1"},
            "shear_modulus": {"value": 8.0e10, "unit": "Pa"}, "provenance": INVENTED}],
        "pipe_segments": [{"id": PIPE, "from": "root", "to": "tip", "material": "steel",
            "y_reference": {"x": 0, "y": 1, "z": 0}, "provenance": INVENTED,
            "section": {"outside_diameter": {"value": OD, "unit": "m"}, "wall_thickness": {"value": WALL, "unit": "m"},
                "mill_tolerance": {"value": mill, "unit": "m"}, "corrosion_allowance": {"value": allowance, "unit": "m"},
                "material_density": {"value": STEEL, "unit": "kg/m^3"}, "contents_density": {"value": WATER, "unit": "kg/m^3"}}}],
        "supports": [{"id": "anchor", "family": "anchor", "node": "root", "provenance": INVENTED,
            "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]}],
        "load_cases": []
    }})
}
/// The same member on the preview route: no contract and no allowance.
fn preview(mill: f64) -> Value {
    let mut request = document(mill, 0.0);
    request["model"].as_object_mut().unwrap().remove("pressure_contract");
    request["model"]["pipe_segments"][0]["section"].as_object_mut().unwrap().remove("corrosion_allowance");
    request
}
fn typed(request: &Value) -> PreviewModel {
    serde_json::from_value(request["model"].clone()).unwrap()
}
fn generation() -> SelfWeightRequest {
    SelfWeightRequest {
        case_id: CASE.into(),
        label: "Invented self-weight and contents".into(),
        pipe_refs: vec![PIPE.into()],
        gravity: SelfWeightGravity { value: GRAVITY, unit: "m/s^2".into(), axis: "global_y".into() },
        provenance: INVENTED.into(),
        source_model_hash: format!("sha256:{}", "0".repeat(64)),
    }
}
/// The generated evidence record and the request with the generated case
/// applied as the operation route applies it (the case shell, then its
/// primitive), plus the exact route's explicit `pressure_regions: []`.
fn generated(mut request: Value) -> (Value, Value) {
    let plan = generate_self_weight_operations(&typed(&request), &generation()).unwrap();
    let mut case: Value = serde_json::from_str(&plan.changes[0].after).unwrap();
    case["primitive_loads"] = json!([serde_json::from_str::<Value>(&plan.changes[1].after).unwrap()]);
    if request["model"].get("pressure_contract").is_some() {
        case["pressure_regions"] = json!([]);
    }
    request["model"]["load_cases"] = json!([case]);
    (plan.source_evidence[0].clone(), request)
}
fn state(request: &Value) -> AppliedSelfWeightState {
    inspect_applied_self_weight(&typed(request), None)[0].state
}
/// The applied primitive's provenance record, edited and written back.
fn with_record(mut request: Value, edit: impl FnOnce(&mut Value)) -> Value {
    let load = &mut request["model"]["load_cases"][0]["primitive_loads"][0];
    let mut record: Value = serde_json::from_str(load["provenance"].as_str().unwrap()).unwrap();
    edit(&mut record);
    load["provenance"] = json!(record.to_string());
    request
}
/// D-5 in the test's own binary64 order: metal on the nominal wall t,
/// contents on the pressure bore D/2 - ((t - c_m) - c_a).
fn exact_mass(mill: f64, allowance: f64) -> f64 {
    let metal = (PI * WALL) * (OD - WALL);
    let bore_radius = OD * 0.5 - ((WALL - mill) - allowance);
    metal * STEEL + ((PI * bore_radius) * bore_radius) * WATER
}
/// The preview basis: both on the single wall t - c_m.
fn preview_mass(mill: f64) -> f64 {
    let wall = WALL - mill;
    let bore_radius = OD * 0.5 - wall;
    ((PI * wall) * (OD - wall)) * STEEL + ((PI * bore_radius) * bore_radius) * WATER
}

/// Design §1 ("`stable_mass_per_length`, FK `MassPrimitive`"): on an exact
/// document the generated mass puts the metal on the nominal wall and the
/// contents on the pressure bore, under `EXACT_MASS_METHOD`, with the
/// allowance among the record's inputs and dependencies; with c_m = c_a = 0 it
/// is the preview mass bit for bit, and the preview route is unchanged.
#[test]
fn d5_generated_self_weight_uses_the_exact_mass_basis() {
    for (mill, allowance) in [(MILL, ALLOWANCE), (MILL, 0.0), (0.0, 0.0)] {
        let (record, _) = generated(document(mill, allowance));
        let label = format!("c_m {mill}, c_a {allowance}");
        assert_eq!(record["method"], json!(GENERATED_METHOD), "{label}");
        assert_eq!(record["mass_method"], json!(EXACT_MASS_METHOD), "{label}");
        assert_eq!(record["mass_inputs"]["corrosion_allowance"], json!({"value": allowance, "unit": "m"}), "{label}");
        assert_eq!(record["normalized_dependencies"]["corrosion_allowance"]["value"], json!(allowance), "{label}");
        let mass = record["mass_kg_per_m"].as_f64().unwrap();
        assert_eq!(mass.to_bits(), exact_mass(mill, allowance).to_bits(), "{label}: mass");
        let intensity = record["generated_payload"]["magnitude"]["value"].as_f64().unwrap();
        assert_eq!(intensity.to_bits(), (mass * GRAVITY).to_bits(), "{label}: intensity");
        if mill == 0.0 && allowance == 0.0 {
            assert_eq!(mass.to_bits(), preview_mass(0.0).to_bits(), "{label}: one wall, one mass");
        } else {
            assert_ne!(mass.to_bits(), preview_mass(mill).to_bits(), "{label}: the metal is on the nominal wall");
        }
    }
    // The preview route keeps its method, inputs and mass.
    let (record, _) = generated(preview(MILL));
    assert_eq!(record["mass_method"], json!(STABLE_MASS_METHOD));
    assert!(record["mass_inputs"].get("corrosion_allowance").is_none());
    assert!(record["normalized_dependencies"].get("corrosion_allowance").is_none());
    assert_eq!(record["mass_kg_per_m"].as_f64().unwrap().to_bits(), preview_mass(MILL).to_bits());
    // An exact section without its allowance has no mass basis.
    let mut missing = document(MILL, ALLOWANCE);
    missing["model"]["pipe_segments"][0]["section"].as_object_mut().unwrap().remove("corrosion_allowance");
    let refused = generate_self_weight_operations(&typed(&missing), &generation()).unwrap_err();
    assert!(refused.iter().any(|d| d.message.contains("corrosion allowance")), "{refused:?}");
}

/// Design §1 and the applied-weight validation: an exact record is Fresh on
/// its own document, Stale when the allowance (a consumed input) or the mass
/// method (the document moved off the exact contract) changes, and Invalid
/// when its allowance or method is not the one its mass was formed with.
#[test]
fn d5_applied_self_weight_validation_tracks_the_exact_basis() {
    let (_, request) = generated(document(MILL, ALLOWANCE));
    assert_eq!(state(&request), AppliedSelfWeightState::Fresh);

    let mut drifted = request.clone();
    drifted["model"]["pipe_segments"][0]["section"]["corrosion_allowance"]["value"] = json!(0.002);
    let statuses = inspect_applied_self_weight(&typed(&drifted), Some(&format!("sha256:{}", "1".repeat(64))));
    assert_eq!(statuses[0].state, AppliedSelfWeightState::Stale);
    assert_eq!(statuses[0].diagnostics[0].code, "SELF_WEIGHT_INPUTS_STALE");
    let replacement = statuses[0].replacement.as_ref().expect("a refresh is offered");
    let refreshed: Value = serde_json::from_str(replacement["provenance"].as_str().unwrap()).unwrap();
    assert_eq!(refreshed["mass_inputs"]["corrosion_allowance"]["value"], json!(0.002));
    assert_eq!(refreshed["mass_kg_per_m"].as_f64().unwrap().to_bits(), exact_mass(MILL, 0.002).to_bits());

    // The same record on the preview route: the method changes.
    let mut moved = request.clone();
    moved["model"].as_object_mut().unwrap().remove("pressure_contract");
    moved["model"]["pipe_segments"][0]["section"].as_object_mut().unwrap().remove("corrosion_allowance");
    moved["model"]["load_cases"][0].as_object_mut().unwrap().remove("pressure_regions");
    assert_eq!(state(&moved), AppliedSelfWeightState::Stale);

    // A record whose allowance or method is not its mass's own is invalid.
    for (label, tampered) in [
        ("no allowance input", with_record(request.clone(), |r| {
            r["mass_inputs"].as_object_mut().unwrap().remove("corrosion_allowance");
        })),
        ("no allowance dependency", with_record(request.clone(), |r| {
            r["normalized_dependencies"].as_object_mut().unwrap().remove("corrosion_allowance");
        })),
        ("another allowance", with_record(request.clone(), |r| {
            r["mass_inputs"]["corrosion_allowance"]["value"] = json!(0.002);
        })),
        ("the preview method", with_record(request.clone(), |r| r["mass_method"] = json!(STABLE_MASS_METHOD))),
        ("no method", with_record(request.clone(), |r| {
            r.as_object_mut().unwrap().remove("mass_method");
        })),
    ] {
        assert_eq!(state(&tampered), AppliedSelfWeightState::Invalid, "{label}");
    }
}

/// The exact document with its generated self-weight solves on both routes
/// and its K-D5 certificate passes: FK re-forms the load from the record's
/// mass inputs and allowance (`MassPrimitive::corrosion_allowance`). A
/// supplied primitive stripped of its allowance fails closed.
#[test]
fn d5_exact_self_weight_solves_with_its_certified_mass() {
    let (_, request) = generated(document(MILL, ALLOWANCE));
    for mode in MODES {
        let envelope =
            serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
        assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}: {}", envelope["diagnostics"]);
        assert_eq!(envelope["numerical_quality"]["status"], json!("checks_passed"), "{mode:?}: {}", envelope["diagnostics"]);

        supplied_mutant::arm(Box::new(|_, supplied: &mut Vec<LoadPrimitive>| {
            let mut hit = false;
            for primitive in supplied.iter_mut() {
                if let LoadFormula::UniformStraight { intensity: IntensityPrimitive::SelfWeight { mass, .. }, .. } =
                    &mut primitive.formula
                {
                    hit |= mass.corrosion_allowance.take().is_some();
                }
            }
            hit
        }));
        let mutated =
            serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
        assert!(supplied_mutant::disarm().is_some(), "{mode:?}: the mutant reached a self-weight primitive");
        assert_ne!(mutated["numerical_quality"]["status"], json!("checks_passed"), "{mode:?}");
    }
}

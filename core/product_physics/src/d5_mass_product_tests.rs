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
use std::collections::BTreeMap;
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

/// A minimal exact-contract 0.9.0 request: one straight member anchored at
/// its root, with the given mill tolerance and allowance and no load case.
fn document(mill: f64, allowance: f64) -> Value {
    json!({"materials": [], "model": {
        "schema_version": "0.9.0",
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

// ---------------------------------------------------------------- authored units
//
// The K-D5 check re-forms a generated self-weight load from the record's SI
// operands whatever units the section was authored in. One physical section,
// chosen in inches (OD 8.625 in, wall 0.322 in, mill 0.040 in, allowance
// 0.0625 in, insulation 2 in), is authored in m, mm and in; its densities in
// kg/m^3 or lb/in^3. Gravity has one unit (m/s^2) and is read from the
// record's validated `normalized_acceleration_m_per_s2`.

/// Each length the mass path reads, its value in m, mm and in (decimal
/// equivalents of the same length).
const LENGTHS: [(&str, [f64; 3]); 5] = [
    ("outside_diameter", [0.219075, 219.075, 8.625]),
    ("wall_thickness", [0.0081788, 8.1788, 0.322]),
    ("mill_tolerance", [0.001016, 1.016, 0.040]),
    ("corrosion_allowance", [0.0015875, 1.5875, 0.0625]),
    ("insulation_thickness", [0.0508, 50.8, 2.0]),
];
const LENGTH_UNITS: [&str; 3] = ["m", "mm", "in"];
/// Each density the mass path reads, in kg/m^3 and (another physical set) in lb/in^3.
const DENSITIES: [(&str, [f64; 2]); 3] = [
    ("material_density", [7850.0, 0.2836]),
    ("contents_density", [1000.0, 0.0361]),
    ("insulation_density", [128.0, 0.00462]),
];
const DENSITY_UNITS: [&str; 2] = ["kg/m^3", "lb/in^3"];
const INTEGRITY: &str = "diagnostic:numerical-integrity:case:weight";

/// The member of `document` with the section authored as given (no
/// allowance on the preview route), its self-weight generated and applied.
fn authored(exact: bool, section: Value) -> Value {
    let mut request = if exact { document(0.0, 0.0) } else { preview(0.0) };
    request["model"]["pipe_segments"][0]["section"] = section;
    generated(request).1
}
/// The section authored with lengths in `LENGTH_UNITS[l]` and densities in `DENSITY_UNITS[d]`.
fn section_in(exact: bool, l: usize, d: usize) -> Value {
    let mut section = json!({});
    for (name, values) in LENGTHS {
        if exact || name != "corrosion_allowance" {
            section[name] = json!({"value": values[l], "unit": LENGTH_UNITS[l]});
        }
    }
    for (name, values) in DENSITIES {
        section[name] = json!({"value": values[d], "unit": DENSITY_UNITS[d]});
    }
    section
}
/// The same section authored in m and kg/m^3 at its exact normalized binary64 values.
fn normalized_twin(section: &Value) -> Value {
    let mut twin = json!({});
    for (name, quantity) in section.as_object().unwrap() {
        let dimension = if name.ends_with("density") { Dimension::Density } else { Dimension::Length };
        let from = unit_by_symbol(quantity["unit"].as_str().unwrap(), dimension).unwrap();
        let to = canonical_unit(dimension).unwrap();
        let value = convert_for_dimension(quantity["value"].as_f64().unwrap(), dimension, from, to).unwrap();
        twin[name] = json!({"value": value, "unit": to.definition().symbol});
    }
    twin
}
/// The case's K-D5 outcome (its integrity diagnostic and numerical quality)
/// and every published value by row id, bit for bit.
fn solved(request: &Value, mode: PreviewSolverMode) -> (Value, BTreeMap<String, String>) {
    let envelope =
        serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
    assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}: {}", envelope["diagnostics"]);
    let integrity: Vec<&Value> =
        envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["id"] == INTEGRITY).collect();
    assert_eq!(integrity.len(), 1, "{mode:?}");
    let outcome = json!({"integrity": {"code": integrity[0]["code"], "message": integrity[0]["message"]},
        "numerical_quality": envelope["numerical_quality"]});
    let values = envelope["results"].as_array().unwrap().iter()
        .map(|row| (row["id"].as_str().unwrap().to_string(),
            row["value"].as_f64().map_or_else(|| row["value"].to_string(), |v| format!("{:016x}", v.to_bits()))))
        .collect();
    (outcome, values)
}
fn checks_passed(outcome: &Value, label: &str) {
    assert_eq!(outcome["integrity"]["code"], json!("NUMERICAL_INTEGRITY_CHECKS_PASSED"), "{label}: {outcome}");
    assert_eq!(outcome["numerical_quality"]["status"], json!("checks_passed"), "{label}: {outcome}");
}

/// A generated self-weight load on a section in millimetres or inches is not
/// demoted by the formation check, and its metre twin (the same SI operands)
/// gives the identical check outcome and identical published values. On the
/// former raw read the check was given a mass 10^6 (mm) or 1/0.0254^2 (in)
/// times the one the load was formed from.
#[test]
fn generated_self_weight_on_millimetre_and_inch_sections_is_not_demoted() {
    for l in [1, 2] {
        let section = section_in(true, l, 0);
        let (request, twin) = (authored(true, section.clone()), authored(true, normalized_twin(&section)));
        for mode in MODES {
            let label = format!("{} {mode:?}", LENGTH_UNITS[l]);
            let (outcome, values) = solved(&request, mode);
            checks_passed(&outcome, &label);
            let (twin_outcome, twin_values) = solved(&twin, mode);
            assert_eq!(outcome, twin_outcome, "{label}: the metre twin's check outcome");
            assert_eq!(values, twin_values, "{label}: the metre twin's published values");
        }
    }
}

/// Property: the K-D5 outcome of a generated self-weight load does not depend
/// on the units its section was authored in. Every length the mass path reads
/// (outside diameter, wall, mill tolerance, allowance, insulation) and every
/// density (pipe, contents, insulation), on both mass routes and in both
/// modes: the decimal-equivalent authorings in m, mm and in give the
/// identical outcome, and each authoring not already in m and kg/m^3 gives,
/// against its twin at its exact normalized binary64 values, the identical
/// outcome and bit-identical published values. A raw read of any of these
/// from the record differs by a factor of at least 1/0.0254 and is caught.
#[test]
fn generated_self_weight_check_is_invariant_under_the_authored_units() {
    for exact in [true, false] {
        for mode in MODES {
            for d in 0..DENSITY_UNITS.len() {
                let mut first: Option<Value> = None;
                for l in 0..LENGTH_UNITS.len() {
                    let label = format!("exact {exact} {mode:?} {} {}", LENGTH_UNITS[l], DENSITY_UNITS[d]);
                    let section = section_in(exact, l, d);
                    let (outcome, values) = solved(&authored(exact, section.clone()), mode);
                    checks_passed(&outcome, &label);
                    let outcome_kind = json!([outcome["integrity"]["code"], outcome["numerical_quality"]["status"],
                        outcome["numerical_quality"]["cases"].as_array().unwrap().iter().map(|c| &c["solve_quality"]).collect::<Vec<_>>()]);
                    assert_eq!(first.get_or_insert_with(|| outcome_kind.clone()), &outcome_kind, "{label}: the same outcome in every unit");
                    if l != 0 || d != 0 {
                        let (twin_outcome, twin_values) = solved(&authored(exact, normalized_twin(&section)), mode);
                        assert_eq!(outcome, twin_outcome, "{label}: the normalized twin's check outcome");
                        assert_eq!(values, twin_values, "{label}: the normalized twin's published values");
                    }
                }
            }
        }
    }
}

/// Mixed authoring: the lengths of one section in different units from each
/// other, and its densities likewise. Length i of `LENGTHS` takes unit
/// (i + s) mod 3 and density j of `DENSITIES` unit (j + s) mod 2, for s = 0, 1,
/// 2, on both mass routes and in both modes. Each authoring is checks passed
/// and gives its normalized twin's check outcome and published values bit for
/// bit. The uniform matrix above cannot see a repair that converts every
/// length with one length's unit (or every density with one density's): this
/// one does.
#[test]
fn generated_self_weight_check_is_invariant_under_mixed_authored_units() {
    for exact in [true, false] {
        for s in 0..3 {
            let mut section = json!({});
            for (i, (name, values)) in LENGTHS.iter().enumerate() {
                if exact || *name != "corrosion_allowance" {
                    let l = (i + s) % LENGTH_UNITS.len();
                    section[*name] = json!({"value": values[l], "unit": LENGTH_UNITS[l]});
                }
            }
            for (j, (name, values)) in DENSITIES.iter().enumerate() {
                let d = (j + s) % DENSITY_UNITS.len();
                section[*name] = json!({"value": values[d], "unit": DENSITY_UNITS[d]});
            }
            let (request, twin) = (authored(exact, section.clone()), authored(exact, normalized_twin(&section)));
            for mode in MODES {
                let label = format!("exact {exact} shift {s} {mode:?}: {section}");
                let (outcome, values) = solved(&request, mode);
                checks_passed(&outcome, &label);
                let (twin_outcome, twin_values) = solved(&twin, mode);
                assert_eq!(outcome, twin_outcome, "{label}: the normalized twin's check outcome");
                assert_eq!(values, twin_values, "{label}: the normalized twin's published values");
            }
        }
    }
}

/// The independent reader's witness (round one), kept as it was run: the wall
/// in metres, every other length in millimetres or inches, densities in either
/// unit, on both mass routes and in both modes. It is the authoring shown to
/// reject a repair that converts every length with the outside diameter's unit.
#[test]
fn generated_self_weight_with_a_metre_wall_and_other_lengths_in_mm_or_in_matches_its_twin() {
    for exact in [true, false] {
        for l in [1, 2] {
            for d in 0..DENSITY_UNITS.len() {
                let mut section = section_in(exact, l, d);
                section["wall_thickness"] = json!({"value": LENGTHS[1].1[0], "unit": LENGTH_UNITS[0]});
                let (request, twin) = (authored(exact, section.clone()), authored(exact, normalized_twin(&section)));
                for mode in MODES {
                    let label = format!("exact {exact} {} {} metre wall {mode:?}", LENGTH_UNITS[l], DENSITY_UNITS[d]);
                    let (outcome, values) = solved(&request, mode);
                    checks_passed(&outcome, &label);
                    let (twin_outcome, twin_values) = solved(&twin, mode);
                    assert_eq!(outcome, twin_outcome, "{label}: the normalized twin's check outcome");
                    assert_eq!(values, twin_values, "{label}: the normalized twin's published values");
                }
            }
        }
    }
}

/// Negative witness: a generated self-weight load whose formation really is
/// wrong is still found, in every unit. The s11f RF-CANCEL-UDL geometry (two
/// 2 m members, anchored ends, S1 pinned) carries the generated self-weight
/// of A and B, about 1e8 N/m each and about 1 N/m apart (pipe densities
/// 1.88e9 and 1.88e9 + 18 kg/m^3 on the inch-chosen section): their
/// fixed-end moments, about 3.3e7 N*m, cancel at S1 to about 0.3 N*m, so the
/// binary64 formation's rounding (an ulp of 3.3e7 is 3.7e-9) exceeds 1e-9 of
/// the net. The check re-forms from the SI operands and finds it at S1:RZ,
/// as it finds the authored UDL twin (`f1a_tests`), by a ratio of order one
/// to a thousand, never the 10^10 or more a unit factor gives.
#[test]
fn a_generated_self_weight_formed_wrong_is_still_flagged_in_every_unit() {
    let data: Value = serde_json::from_str(include_str!("../tests/fixtures/s11f/rf_cancel_cases.json")).unwrap();
    let base = data["cases"].as_array().unwrap().iter().find(|c| c["id"] == json!("RF-CANCEL-UDL-W1e8")).unwrap()["request"].clone();
    for l in 0..LENGTH_UNITS.len() {
        let mut request = base.clone();
        request["model"]["load_cases"] = json!([]);
        for (pipe, density) in [(0, 1.88e9), (1, 1.88e9 + 18.0)] {
            let mut section = json!({});
            for (name, values) in &LENGTHS[..3] {
                section[*name] = json!({"value": values[l], "unit": LENGTH_UNITS[l]});
            }
            section["material_density"] = json!({"value": density, "unit": "kg/m^3"});
            request["model"]["pipe_segments"][pipe]["section"] = section;
        }
        let generation = SelfWeightRequest { pipe_refs: vec!["A".into(), "B".into()], ..generation() };
        let plan = generate_self_weight_operations(&typed(&request), &generation).unwrap();
        let mut case: Value = serde_json::from_str(&plan.changes[0].after).unwrap();
        case["primitive_loads"] =
            json!(plan.changes[1..].iter().map(|c| serde_json::from_str::<Value>(&c.after).unwrap()).collect::<Vec<_>>());
        request["model"]["load_cases"] = json!([case]);
        assert_eq!(state(&request), AppliedSelfWeightState::Fresh);
        for mode in MODES {
            let label = format!("{} {mode:?}", LENGTH_UNITS[l]);
            let envelope =
                serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
            let integrity = envelope["diagnostics"].as_array().unwrap().iter().find(|d| d["id"] == INTEGRITY).unwrap();
            assert_eq!(integrity["code"], json!("NUMERICAL_INTEGRITY_SENSITIVE"), "{label}: {integrity}");
            let message = integrity["message"].as_str().unwrap();
            let at = message.find("formation_check: reason=estimate; row=S1:RZ;").unwrap_or_else(|| panic!("{label}: {message}"));
            let ratio: f64 = message[at..].split("trigger_ratio=").nth(1).unwrap().split(|c: char| c != '.' && c != 'e' && c != '-' && c != '+' && !c.is_ascii_digit()).next().unwrap().parse().unwrap();
            assert!(ratio > 1.0 && ratio < 1e6, "{label}: the formation's own rounding, not a unit factor: {ratio:e}");
        }
    }
}

/// The fail-closed path of a generated record's K-D5 operands (DEL-04-07 Scope:
/// a load term the check cannot re-form is not published as established).
/// Called directly (`certified_loads::intensity`, the one place the path is
/// reachable): a record whose `normalized_dependencies` holds a length in mm, a
/// density in lb/in^3, or lacks an operand gives NaN operands, never a mass
/// from the authored `mass_inputs`; K-D5 refuses a non-finite operand
/// (`Wide::from_f64`), so such a case would be published sensitive with
/// `formation_check: reason=formation_check_unavailable`. Through the product
/// entry the same records never reach the check: the applied-weight validation
/// refuses each by name before the solve. The NaN path is the second line.
#[test]
fn a_record_whose_dependencies_are_not_si_or_incomplete_gives_no_mass() {
    let (_, request) = generated(document(MILL, ALLOWANCE));
    let operands = |request: &Value| {
        let model = typed(request);
        let mut diagnostics = Vec::new();
        let loads = build_load_case_primitive_loads(&model, &model.load_cases[0], &mut diagnostics);
        let application = prepare_loads(model.nodes.len(), model.pipe_segments.len(), &loads);
        match certified_loads::intensity(&model, &application.element_uniform_loads[0], CASE) {
            IntensityPrimitive::SelfWeight { mass, .. } => mass,
            other => panic!("not a self-weight intensity: {other:?}"),
        }
    };
    let mass = operands(&request);
    let si = [mass.outer_diameter, mass.wall_thickness, mass.mill_tolerance, mass.pipe_density, mass.contents_density];
    assert_eq!(si.map(f64::to_bits), [OD, WALL, MILL, STEEL, WATER].map(f64::to_bits), "as generated: the SI operands");
    assert_eq!(mass.corrosion_allowance.map(f64::to_bits), Some(ALLOWANCE.to_bits()));
    let load = request["model"]["load_cases"][0]["primitive_loads"][0]["id"].clone();
    // Every case is judged before any assertion, so a defect names each case it reaches.
    let mut defects = Vec::new();
    for (label, tampered) in [
        ("a length in mm", with_record(request.clone(), |r| {
            r["normalized_dependencies"]["outside_diameter"] = json!({"value": OD * 1000.0, "unit": "mm"});
        })),
        ("a density in lb/in^3", with_record(request.clone(), |r| {
            r["normalized_dependencies"]["material_density"] = json!({"value": 0.2836, "unit": "lb/in^3"});
        })),
        ("a missing operand", with_record(request.clone(), |r| {
            r["normalized_dependencies"].as_object_mut().unwrap().remove("contents_density");
        })),
    ] {
        let mass = operands(&tampered);
        let all = [mass.outer_diameter, mass.wall_thickness, mass.mill_tolerance, mass.pipe_density, mass.contents_density,
            mass.insulation_thickness, mass.insulation_density, mass.corrosion_allowance.unwrap_or(0.0)];
        if !all.iter().all(|v| v.is_nan()) || mass.corrosion_allowance.is_none() {
            defects.push(format!("{label}: NaN operands on the exact basis, not a mass: {mass:?}"));
        }
        let envelope = serde_json::to_value(
            run_linear_static_preview_value_with_mode(tampered.clone(), PreviewSolverMode::SparseInteractive).unwrap(),
        )
        .unwrap();
        let refusal: Vec<&Value> = envelope["diagnostics"].as_array().unwrap().iter()
            .filter(|d| d["code"] == "SELF_WEIGHT_INPUT_INVALID" && d["severity"] == "blocking").collect();
        let refused = envelope["status"]["mechanics"] != json!("MECHANICS_SOLVED")
            && refusal.len() == 1
            && refusal[0]["affected_refs"] == json!([CASE, CASE, load])
            && refusal[0]["message"].as_str().is_some_and(|m| m.contains("generated self-weight provenance is incomplete or inconsistent"));
        if !refused {
            let integrity = envelope["diagnostics"].as_array().unwrap().iter().find(|d| d["id"] == INTEGRITY);
            let shown = integrity.map(|d| format!("{} ...{}", d["code"], d["message"].as_str().unwrap_or("").rsplit(". ").next().unwrap_or("")));
            defects.push(format!("{label}: not refused by name before the solve: {} {shown:?}", envelope["status"]["mechanics"]));
        }
    }
    assert!(defects.is_empty(), "{defects:#?}");
}

/// Signed zero (round one's reader, extended through re-formation). The
/// applied-weight validation compares a record's `normalized_dependencies` by
/// JSON value equality, so a dependency of -0.0 where the generator wrote +0.0
/// stays fresh and reaches the check's primitive with its sign bit (the
/// reader's witness, kept as the first assertion). This test requires that the
/// sign of a zero operand changes nothing the check derives from it: on the
/// exact route (zero mill tolerance and zero allowance, each alone and both)
/// and on the preview route (zero mill tolerance), in both modes, the case's
/// integrity code and message, its numerical quality, every published value
/// bit for bit, and every certified recovery record (each group's norm, bound
/// and scale, printed with their signs) equal those of the record as
/// generated. Any difference is a dependency on the sign of zero, to be
/// repaired, not documented.
#[test]
fn a_zero_operand_of_either_sign_re_forms_to_the_same_check() {
    let probe = |request: &Value, mode| {
        crate::tests::KD5_RECOVERY_PROBE.with(|p| *p.borrow_mut() = Some(Vec::new()));
        let solved = solved(request, mode);
        let records = crate::tests::KD5_RECOVERY_PROBE.with(|p| p.borrow_mut().take().unwrap());
        (solved, format!("{records:?}"))
    };
    for (exact, operands) in [
        (true, vec!["mill_tolerance"]),
        (true, vec!["corrosion_allowance"]),
        (true, vec!["mill_tolerance", "corrosion_allowance"]),
        (false, vec!["mill_tolerance"]),
    ] {
        let (_, request) = generated(if exact { document(0.0, 0.0) } else { preview(0.0) });
        let negated = with_record(request.clone(), |r| {
            for name in &operands {
                assert_eq!(r["normalized_dependencies"][*name]["value"].as_f64().map(f64::to_bits), Some(0.0f64.to_bits()), "{name}");
                r["normalized_dependencies"][*name]["value"] = json!(-0.0);
            }
        });
        let label = format!("exact {exact} {operands:?}");
        assert_eq!(state(&negated), AppliedSelfWeightState::Fresh, "{label}: value equality keeps the record fresh");
        let model = typed(&negated);
        let loads = build_load_case_primitive_loads(&model, &model.load_cases[0], &mut Vec::new());
        let application = prepare_loads(model.nodes.len(), model.pipe_segments.len(), &loads);
        let IntensityPrimitive::SelfWeight { mass, .. } = certified_loads::intensity(&model, &application.element_uniform_loads[0], CASE)
        else { panic!("{label}: not a self-weight intensity") };
        if operands.contains(&"mill_tolerance") {
            assert_eq!(mass.mill_tolerance.to_bits(), (-0.0f64).to_bits(), "{label}: the sign reaches the primitive");
        }
        if operands.contains(&"corrosion_allowance") {
            assert_eq!(mass.corrosion_allowance.map(f64::to_bits), Some((-0.0f64).to_bits()), "{label}: the sign reaches the primitive");
        }
        for mode in MODES {
            let ((outcome, values), records) = probe(&request, mode);
            let ((negated_outcome, negated_values), negated_records) = probe(&negated, mode);
            checks_passed(&outcome, &format!("{label} {mode:?}"));
            assert_eq!(negated_outcome, outcome, "{label} {mode:?}: the check outcome");
            assert_eq!(negated_values, values, "{label} {mode:?}: every published value, bit for bit");
            assert_eq!(negated_records, records, "{label} {mode:?}: every certified recovery record");
            assert!(records.contains("groups"), "{label} {mode:?}: the records were collected");
        }
    }
}

// ------------------------------------------------- the defect's faces against statics
//
// While the formation check read a generated record's `mass_inputs` as SI, it
// re-formed the weight of a section authored in millimetres 10^6 times too
// large (1/0.0254^2 in inches). Two faces follow, and a third arm of the same
// read. (a) The ordinary face: a case whose generated weight matters is
// demoted to sensitive. (b) The negligible-weight face: a case whose generated
// weight is negligible beside an authored load still passes, and its support
// rows are published with internal bounds that exclude the exact value. (c) A
// case factored from the generated record (the check's Seismic arm) re-forms
// from the same operands. Each is held here against the closed form of the
// 3 m cantilever of `document` from statics alone, never against the product.

/// Statics of the 3 m cantilever of `document` under uniform loads `loads`
/// (N/m along global y): the anchor's action on the pipe is Fy = -(sum w) L and
/// Mz = -(sum w) L^2/2, each formed here as an exact sum of its terms
/// (`ExactAccumulator`). Asserts that the case is checks passed and that each
/// published anchor Fy and Mz row, and its group norm, lies within its group's
/// internal bound plus `slack` of the closed form, decided exactly.
///
/// The bounds are the formation check's INTERNAL group bounds, read from the
/// certified recovery record through the crate's test hook; an ordinary result
/// document prints no bound for these rows. The ground is the closed form; the
/// formation check's inverse-norm premise is not used as an oracle.
///
/// The slack is a bound on rounding, not a tolerance. Where a load is a
/// generated weight w_g, the test and the generator each evaluate the same real
/// formula w_g = rho g pi t (D - t) (D-5, nominal wall) from the same decimal
/// inputs in binary64 and may differ in its last places. With u = 2^-53, to
/// first order: the test rounds D, t, pi, rho and g once each and makes one
/// subtraction and four multiplications; the error of D - t carries those of
/// D and t amplified by (D + t)/(D - t) = 1.078; in all at most 10.1 u. The
/// generator converts D and t from the authored unit (a rounded decimal, a
/// rounded factor and one multiplication: 3 u each) and forms pi t (D - t) in
/// scaled arithmetic, one binary64 rounding an operation
/// (`pressure_exact::Scaled`), then multiplies by rho and by g; counted the same
/// way, at most 15.3 u. So the two values of w_g differ by at most 25.4 u |w_g|;
/// 32 u is taken. The weight enters Fy times L and Mz times L^2/2, both at most
/// L^2 here, so the slack is 32 u |w_g| L^2. (The generator's count is by
/// reading `stable_mass_per_length` and `source_wall_area_scaled`.)
fn assert_anchor_rows_meet_statics(request: &Value, loads: &[f64], slack: f64, label: &str) {
    use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
    const L: f64 = 3.0;
    let fy = |acc: &mut ExactAccumulator, sign: f64| {
        for w in loads {
            acc.add_product(-sign * w, L).unwrap();
        }
    };
    let mz = |acc: &mut ExactAccumulator, sign: f64| {
        for w in loads {
            acc.add_product(-sign * w, L * L / 2.0).unwrap();
        }
    };
    // |published - closed form| <= bound + slack, decided exactly.
    let contains = |published: f64, bound: f64, closed: &dyn Fn(&mut ExactAccumulator, f64)| {
        let side = |sign: f64| {
            let mut acc = ExactAccumulator::new();
            acc.add(sign * published).unwrap();
            closed(&mut acc, -sign);
            acc.add(bound).unwrap();
            acc.add(slack).unwrap();
            acc.signum() >= 0
        };
        side(1.0) && side(-1.0)
    };
    for mode in MODES {
        crate::tests::KD5_RECOVERY_PROBE.with(|p| *p.borrow_mut() = Some(Vec::new()));
        let envelope =
            serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
        let records = crate::tests::KD5_RECOVERY_PROBE.with(|p| p.borrow_mut().take().unwrap());
        assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{label} {mode:?}: {}", envelope["diagnostics"]);
        let integrity = envelope["diagnostics"].as_array().unwrap().iter().find(|d| d["id"] == INTEGRITY).unwrap();
        assert_eq!(integrity["code"], json!("NUMERICAL_INTEGRITY_CHECKS_PASSED"), "{label} {mode:?}: {}", integrity["message"]);
        let group = |kind: &str| {
            let g = records.iter().filter(|r| r.member == "anchor").flat_map(|r| &r.groups)
                .find(|g| g.location == "node" && g.kind == kind).unwrap_or_else(|| panic!("{label} {mode:?}: no anchor {kind} group"));
            (g.q, g.bound.clone().unwrap_or_else(|e| panic!("{label} {mode:?}: anchor {kind} bound unavailable: {e}")))
        };
        let row = |component: &str| {
            envelope["results"].as_array().unwrap().iter()
                .find(|r| r["kind"] == "support_reaction_component_v2" && r["entity_ref"] == "anchor"
                    && r["metadata"]["component"] == component)
                .unwrap_or_else(|| panic!("{label} {mode:?}: no anchor {component} row"))["value"].as_f64().unwrap()
        };
        for (component, kind, closed) in
            [("Fy", "support_force", &fy as &dyn Fn(&mut ExactAccumulator, f64)), ("Mz", "support_moment", &mz)]
        {
            let (q, bound) = group(kind);
            let published = row(component);
            eprintln!("{label} {mode:?} anchor {component}: published {published:.17e}, group norm {q:.17e}, internal group bound {bound:e}");
            assert!(contains(published, bound, closed),
                "{label} {mode:?} {component}: published {published:.17e} with the check's internal group bound {bound:e} excludes the closed form");
            assert!(contains(q, bound, closed),
                "{label} {mode:?} {kind}: group norm {q:.17e} with the check's internal group bound {bound:e} excludes the closed form");
        }
    }
}

/// The section of `document` (OD 0.219075 m, nominal wall 0.0081788 m, no mill
/// tolerance, allowance, contents or insulation) authored in `LENGTH_UNITS[l]`,
/// with pipe density `density` kg/m^3.
fn bare_section(l: usize, density: f64) -> Value {
    let lengths = |name: &str| LENGTHS.iter().find(|(n, _)| *n == name).unwrap().1[l];
    json!({"outside_diameter": {"value": lengths("outside_diameter"), "unit": LENGTH_UNITS[l]},
        "wall_thickness": {"value": lengths("wall_thickness"), "unit": LENGTH_UNITS[l]},
        "mill_tolerance": {"value": 0.0, "unit": LENGTH_UNITS[l]}, "corrosion_allowance": {"value": 0.0, "unit": LENGTH_UNITS[l]},
        "material_density": {"value": density, "unit": "kg/m^3"}})
}
/// The test's own w_g = rho g pi t (D - t) for `bare_section`, in metres.
fn bare_weight(density: f64) -> f64 {
    let (od, wall) = (0.219075, 0.0081788);
    density * ((PI * wall) * (od - wall)) * GRAVITY
}
/// 32 u |w| L^2 with u = EPSILON / 2 and L = 3 m (derivation at `assert_anchor_rows_meet_statics`).
fn weight_slack(w: f64) -> f64 {
    16.0 * f64::EPSILON * w.abs() * 9.0
}

/// Face (a): a generated self weight that matters, on a steel section authored
/// in m, mm and in, is checks passed and its anchor rows meet statics. While
/// the check read the record's millimetres or inches as metres, the mm and in
/// cases were demoted to sensitive (the check was given a mass 10^6 or
/// 1/0.0254^2 times the one the load was formed from); the metre case is the
/// control the defect never reached.
#[test]
fn a_generated_weight_that_matters_meets_statics_in_every_length_unit() {
    let w_g = bare_weight(STEEL);
    assert!((w_g + 417.156).abs() < 1e-3, "the intended weight per length: {w_g:e} N/m");
    for l in 0..LENGTH_UNITS.len() {
        let mut request = document(0.0, 0.0);
        request["model"]["pipe_segments"][0]["section"] = bare_section(l, STEEL);
        let request = generated(request).1;
        assert_anchor_rows_meet_statics(&request, &[w_g], weight_slack(w_g), &format!("face (a) {}", LENGTH_UNITS[l]));
    }
}

/// Face (b): a generated self weight negligible beside an authored load. The
/// section in millimetres with pipe density 1.88e-11 kg/m^3 gives w_g about
/// -1.0e-12 N/m beside an authored uniform -1e4 N/m on the same member, a
/// weight ratio of about 1e-16. While the check re-formed w_g 10^6 times too large the
/// case still passed, and its anchor rows were published with internal bounds
/// that exclude the closed form.
#[test]
fn a_negligible_generated_weight_keeps_the_checks_support_bounds_honest() {
    const AUTHORED: f64 = -1.0e4;
    const DENSITY: f64 = 1.88e-11;
    let mut request = document(0.0, 0.0);
    request["model"]["pipe_segments"][0]["section"] = bare_section(1, DENSITY);
    let mut request = generated(request).1;
    request["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(json!({
        "id": "load:authored-udl", "category": "distributed_force", "target": {"type": "element", "pipe": PIPE},
        "direction": "global_y", "magnitude": {"value": AUTHORED, "unit": "N/m"}, "dimension": "force_per_length",
        "provenance": "invented dominant authored load"}));
    let w_g = bare_weight(DENSITY);
    assert!(w_g < 0.0 && (w_g / AUTHORED - 1e-16).abs() < 1e-17, "the intended weight ratio: {w_g:e}");
    assert_anchor_rows_meet_statics(&request, &[AUTHORED, w_g], weight_slack(w_g), "face (b) mm");
}

/// Arm (c): a load/reference-state case whose source factor scales the
/// generated self-weight primitive (`certified_loads::source_factor`; the
/// check's `IntensityPrimitive::Seismic` arm, coefficient = the factor). The
/// steel section in m, mm and in, factor 0.25 (exact in binary64, so the slack
/// carries over scaled by it): Fy = -0.25 w_g L, Mz = -0.25 w_g L^2/2. While the
/// check read the record's mm or in as metres this arm re-formed from the same
/// wrong mass.
#[test]
fn a_case_factored_from_a_generated_weight_meets_statics_in_every_length_unit() {
    const FACTOR: f64 = 0.25;
    let w = FACTOR * bare_weight(STEEL);
    for l in 0..LENGTH_UNITS.len() {
        let mut request = document(0.0, 0.0);
        request["model"]["pipe_segments"][0]["section"] = bare_section(l, STEEL);
        let mut request = generated(request).1;
        let model = &mut request["model"];
        model["reference_configurations"] = json!([{"id": "reference:installed", "label": "Installed reference",
            "geometry_ref": {"kind": "authored_model_geometry"},
            "member_references": [{"pipe_ref": PIPE, "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": INVENTED}],
            "provenance": INVENTED}]);
        let case = &mut model["load_cases"][0];
        let load = case["primitive_loads"][0]["id"].clone();
        case["analysis_state"] = json!({"contract": LOAD_REFERENCE_STATE_CONTRACT, "reference_configuration_ref": "reference:installed",
            "element_states": [{"pipe_ref": PIPE,
                "material_selection": {"kind": "explicit_base_properties", "material_ref": "steel", "applicability_reference": "invented analytical basis declared applicable"},
                "thermal_state": {"kind": "unchanged_reference", "provenance": INVENTED}}],
            "support_states": [{"support_ref": "anchor", "participation": {"kind": "active_model_device"}}],
            "load_sources": [{"source_ref": load, "factor": FACTOR}],
            "history": {"kind": "independent_equilibrium"}, "provenance": INVENTED});
        assert_anchor_rows_meet_statics(&request, &[w], FACTOR * weight_slack(bare_weight(STEEL)), &format!("arm (c) {}", LENGTH_UNITS[l]));
    }
}

// Review witness: factored self-weight retains the exact allowance with contents present.
#[test]
fn review_factored_generated_weight_with_nonzero_allowance_and_contents() {
    const FACTOR: f64 = 0.25;
    for l in 0..LENGTH_UNITS.len() {
        let mut request = document(MILL, ALLOWANCE);
        request["model"]["pipe_segments"][0]["section"] = section_in(true, l, 0);
        let mut request = generated(request).1;
        let model = &mut request["model"];
        model["reference_configurations"] = json!([{"id": "reference:installed", "label": "Installed reference",
            "geometry_ref": {"kind": "authored_model_geometry"},
            "member_references": [{"pipe_ref": PIPE, "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": INVENTED}],
            "provenance": INVENTED}]);
        let case = &mut model["load_cases"][0];
        let load = case["primitive_loads"][0]["id"].clone();
        case["analysis_state"] = json!({"contract": LOAD_REFERENCE_STATE_CONTRACT, "reference_configuration_ref": "reference:installed",
            "element_states": [{"pipe_ref": PIPE,
                "material_selection": {"kind": "explicit_base_properties", "material_ref": "steel", "applicability_reference": "invented analytical basis declared applicable"},
                "thermal_state": {"kind": "unchanged_reference", "provenance": INVENTED}}],
            "support_states": [{"support_ref": "anchor", "participation": {"kind": "active_model_device"}}],
            "load_sources": [{"source_ref": load, "factor": FACTOR}],
            "history": {"kind": "independent_equilibrium"}, "provenance": INVENTED});
        for mode in MODES {
            checks_passed(&solved(&request, mode).0, &format!("factored nonzero allowance and contents {} {mode:?}", LENGTH_UNITS[l]));
        }
    }
}

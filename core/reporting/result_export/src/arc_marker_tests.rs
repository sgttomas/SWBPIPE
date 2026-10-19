//! T4-U8 (DEL-05-03 `section-bases-and-timoshenko.md` §2.5, §3.6, §5): a
//! realized arc's `pipe_sections` entry, and only an arc's, carries
//! `member_kind = "realized_arc"`, so the evidence reader knows every arc
//! without the model, including an arc outside every pressure region; with the
//! model the marked entries are exactly its realized arcs; and the
//! pressure-source-1 composite, which never carries an arc (D7), refuses one by
//! name.
//!
//! The documents are built in memory in the current D-5 shape; no committed
//! fixture is read. Two exact cases, a straight member and a realized arc with
//! explicitly empty pressure regions, distinct section bases (a nonzero
//! corrosion allowance), a bending row pair per member and per case, and the
//! straight member's maximum on both bases (the arc's is withheld). Python's
//! twins are the arc-marker tests in `tests/test_section_bases_readers.py`.
use super::*;
use crate::section_basis as sb;
use serde_json::{json, Value};

const CASES: [&str; 2] = ["case:A", "case:B"];
const STRAIGHT: &str = "pipe:A";
const ARC: &str = "pipe:BEND";
const OD: f64 = 0.1683;
const WALL: f64 = 0.0071;
const ALLOWANCE: f64 = 0.0015;
const E: f64 = 2.0e11;
const NU: f64 = 0.3;
const MAXIMUM: f64 = 30.0e6;
const MAX_SIGN: &str = "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim";
const BENDING_SIGN: &str = "positive value follows the j-side section action in the element-local frame (local x toward end j); resultants come from section equilibrium over assembled end actions";

fn f(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn annulus(wall: f64) -> Value {
    let (ro, ri) = (OD * 0.5, OD * 0.5 - wall);
    let area = (std::f64::consts::PI * wall) * (OD - wall);
    let second = (area * ((ro * ro) + (ri * ri))) * 0.25;
    json!({"wall_m": wall, "ri_m": ri, "As_m2": area, "I_m4": second, "J_m4": second * 2.0, "Z_m3": second / ro})
}
fn reduced_wall() -> f64 {
    sb::reduced_wall(WALL, 0.0, ALLOWANCE)
}
/// §3.1: kappa_E and A_s on the nominal section with the member's nu.
fn timoshenko_beam() -> Value {
    let kappa = sb::timoshenko_shear_factor(OD * 0.5 - WALL, OD * 0.5, NU);
    json!({"model": sb::TIMOSHENKO, "shear_factor": kappa, "shear_area_m2": kappa * f(&annulus(WALL)["As_m2"])})
}
fn euler_bernoulli() -> Value {
    json!({"model": sb::EULER_BERNOULLI})
}
fn section(pipe: &str, beam: Value, marked: bool) -> Value {
    let ri = OD * 0.5 - reduced_wall();
    let mut entry = json!({"pipe_id": pipe, "geometry_basis": sb::GEOMETRY_BASIS, "outside_diameter_m": OD,
        "ro_m": OD * 0.5, "wall_thickness_m": WALL, "mill_tolerance_m": 0.0, "corrosion_allowance_m": ALLOWANCE,
        "nominal_section": annulus(WALL), "reduced_section": annulus(reduced_wall()),
        "pressure_bore": {"ri_m": ri, "Ai_m2": (std::f64::consts::PI * ri) * ri}, "beam_theory": beam});
    if marked {
        entry[sb::MEMBER_KIND] = json!(sb::REALIZED_ARC);
    }
    entry
}
fn material(pipe: &str) -> Value {
    json!({"pipe_id": pipe, "material_id": "material:steel", "E_pa": E, "nu": NU, "G_pa": E / (2.0 * (1.0 + NU)),
        "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "thermal_consumed": false, "alpha_per_kelvin": null,
        "provenance": "authored"})
}
fn rhs(case: &str) -> Value {
    let zeros = vec![0.0; 12];
    json!({"method": "source_factor_grouped_pressure_rhs_v1", "load_case_id": case, "node_order": ["node:1", "node:2"],
        "dof_order": ["Fx", "Fy", "Fz", "Mx", "My", "Mz"], "dof_units": ["N", "N", "N", "N*m", "N*m", "N*m"],
        "assembled_pressure_rhs_global": zeros, "groups": [], "rounded_cap_rhs_global": zeros,
        "rounded_poisson_rhs_global": zeros, "rounded_cap_and_eigen_ledgers_are_observational": true,
        "cancellation_screen": 0.0, "screen_limit": 1e-9, "screen_roundoff_multiplier": 32,
        "screen_is_not_numerical_qualification": true})
}

/// A pressure-2 document under `shear_model` with explicitly empty pressure
/// regions: the straight member states the contract's model; with `arc`, the
/// realized arc's entry is marked and Euler-Bernoulli and its maximum is
/// withheld on both bases (so both headlines are null). Without the arc both
/// coverages are complete and each basis's headline binds the first case's maximum.
fn document(shear_model: &str, arc: bool) -> Value {
    let straight_beam = if shear_model == sb::TIMOSHENKO { timoshenko_beam() } else { euler_bernoulli() };
    let scale = f(&annulus(WALL)["Z_m3"]) / f(&annulus(reduced_wall())["Z_m3"]);
    let members: Vec<(&str, f64)> = if arc { vec![(STRAIGHT, 12.5), (ARC, 7.25)] } else { vec![(STRAIGHT, 12.5)] };
    let (mut results, mut cases, mut headlines) = (Vec::new(), Vec::new(), serde_json::Map::new());
    for case in CASES {
        for (pipe, value) in &members {
            for (basis, value) in [(sb::NOMINAL, *value), (sb::REDUCED, *value * scale)] {
                results.push(json!({"id": format!("result:loadcase:{case}:stress:{pipe}:end-i:bending-normal-y:{}", basis.replace('_', "-")),
                    "kind": "element_local_bending_normal_stress_y", "value": value, "unit": "MPa", "entity_ref": pipe,
                    "basis_ref": {"ref_type": "load_case", "ref_id": case},
                    "metadata": {"component": "bending_normal_stress_y", "coordinate_system": "element_local", "location": "end_i",
                        "basis": format!("recovered_from_local_element_stiffness; section_basis={basis}"), "sign_convention": BENDING_SIGN}}));
            }
        }
        let mut extrema = Vec::new();
        for (basis, value) in [(sb::NOMINAL, MAXIMUM), (sb::REDUCED, MAXIMUM * scale)] {
            let id = format!("result:elastic-maximum:{}:{case}:{}:{STRAIGHT}:{basis}", case.len(), STRAIGHT.len());
            results.push(json!({"id": id, "kind": "pipe_elastic_normal_stress_maximum_v2", "value": value, "unit": "Pa",
                "entity_ref": STRAIGHT, "basis_ref": {"ref_type": "load_case", "ref_id": case},
                "metadata": {"component": "maximum_absolute_normal_stress", "coordinate_system": "pipe_section",
                    "location": "governing_station", "basis": format!("recovered_from_open_mechanics_stress_components; section_basis={basis}"),
                    "sign_convention": MAX_SIGN}}));
            extrema.push(json!({"pipe_id": STRAIGHT, "section_basis": basis, "result_id": id, "station_fraction": 0.0,
                "span_index": 0, "local_fraction": 0.0, "value_lower_pa": value, "value_upper_pa": value,
                "global_upper_bound_pa": value, "certified_gap_pa": 0.0, "subdivisions": 0,
                "approximation": "piecewise_quadratic_straight_section_statics",
                "coefficient_basis": "j_side_section_equilibrium_binary64_enclosures",
                "enclosure_scope": "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate"}));
            let key = format!("max_open_formula_stress_{basis}");
            if !headlines.contains_key(&key) {
                headlines.insert(key, if arc { Value::Null } else {
                    json!({"value": value, "unit": "Pa", "location_ref": STRAIGHT, "result_ref": id})
                });
            }
        }
        let withheld: Vec<&str> = if arc { vec![ARC] } else { vec![] };
        cases.push(json!({"load_case_id": case, "profile_mode": "exact_pressure_v3", "material_basis": "base_material_common_E_nu",
            "pipe_materials": members.iter().map(|(p, _)| material(p)).collect::<Vec<_>>(),
            "pipe_sections": members.iter().map(|(p, _)| if *p == ARC { section(p, euler_bernoulli(), true) } else { section(p, straight_beam.clone(), false) }).collect::<Vec<_>>(),
            "pipe_stress_extrema": extrema,
            "stress_maximum_coverage": sb::BASES.map(|b| json!({"section_basis": b, "complete": !arc, "unavailable_pipe_ids": withheld})),
            "pressure_rhs_assembly": rhs(case)}));
    }
    let mut summary = serde_json::Map::new();
    summary.insert("max_open_formula_stress".into(), Value::Null);
    summary.extend(headlines);
    json!({"status": {"mechanics": "MECHANICS_SOLVED"},
        "numerical_quality": {"cases": CASES.map(|c| json!({"basis_ref": {"ref_type": "load_case", "ref_id": c}, "evidence_refs": []}))},
        "results": results, "diagnostics": [], "summary": summary,
        "contract_evidence": {"pressure": [], "connector": [], "exact_cases": cases}})
}
/// Each case's entry for `pipe`, edited.
fn edit_sections(doc: &mut Value, pipe: &str, cases: &[&str], edit: impl Fn(&mut Value)) {
    for case in doc["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
        if !cases.contains(&case["load_case_id"].as_str().unwrap()) {
            continue;
        }
        for s in case["pipe_sections"].as_array_mut().unwrap() {
            if s["pipe_id"] == pipe {
                edit(s);
            }
        }
    }
}
fn unmark(s: &mut Value) {
    s.as_object_mut().unwrap().remove(sb::MEMBER_KIND);
}
fn mark(s: &mut Value) {
    s[sb::MEMBER_KIND] = json!(sb::REALIZED_ARC);
}
fn read(doc: &Value) -> Check {
    validate_pressure_evidence(doc)
}
/// A model carrying what G2's checks read: the cases, the contract and, with
/// `realized`, the bend the producer realizes as the arc.
fn model(shear_model: &str, realized: bool) -> Value {
    let consumption = if realized { "curved_bend_macro_element" } else { "mechanics_geometry_only" };
    json!({"load_cases": CASES.map(|c| json!({"id": c})),
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear_model},
        "components": [{"id": "component:bend", "kind": "bend", "geometry": {"bend_pipe_ref": ARC},
            "mechanics_interface": {"solver_consumption": consumption}}]})
}

#[test]
fn an_arc_outside_every_pressure_region_is_known_by_its_marker() {
    let (eb, tim) = (sb::EULER_BERNOULLI, sb::TIMOSHENKO);
    // The arc outside every region in a Timoshenko document is admitted, as under Euler-Bernoulli.
    for shear_model in [eb, tim] {
        assert_eq!(read(&document(shear_model, true)), Ok(()), "{shear_model}");
        assert_eq!(read(&document(shear_model, false)), Ok(()), "{shear_model}");
    }
    let timo = document(tim, true);
    // Without its marker the arc's Euler-Bernoulli entry reads as a second straight model.
    let mut unmarked = timo.clone();
    edit_sections(&mut unmarked, ARC, &CASES, unmark);
    assert_eq!(read(&unmarked), Err("SOURCE_PHYSICS_SHEAR_MODEL_CONSISTENCY".into()));
    // The arc stating Timoshenko (its kappa and A_s consistent) is refused.
    let mut claimed = timo.clone();
    edit_sections(&mut claimed, ARC, &CASES, |s| s["beam_theory"] = timoshenko_beam());
    assert_eq!(read(&claimed), Err("SOURCE_PHYSICS_ARC_BEAM_THEORY".into()));
    // Any other member_kind, on the arc or on a straight member, is refused by name.
    for pipe in [ARC, STRAIGHT] {
        let mut unknown = timo.clone();
        edit_sections(&mut unknown, pipe, &CASES, |s| s[sb::MEMBER_KIND] = json!("straight_pipe"));
        assert_eq!(read(&unknown), Err("SOURCE_PHYSICS_MEMBER_KIND".into()), "{pipe}");
    }
    // The marker on a straight member: an arc stating Timoshenko, or (stating
    // Euler-Bernoulli) an arc whose rows keep the straight sign conventions
    // (T4-U4b: arcs publish their maximum, under the arc's own convention).
    let mut marked = timo.clone();
    edit_sections(&mut marked, STRAIGHT, &CASES, mark);
    assert_eq!(read(&marked), Err("SOURCE_PHYSICS_ARC_BEAM_THEORY".into()));
    let mut marked = document(eb, true);
    edit_sections(&mut marked, STRAIGHT, &CASES, mark);
    assert_eq!(read(&marked), Err("SOURCE_PHYSICS_ROW_SEMANTICS".into()));
    // Realized arcs are a model fact: every case marks the same members.
    let mut one_case = document(eb, true);
    edit_sections(&mut one_case, ARC, &CASES[1..], unmark);
    assert_eq!(read(&one_case), Err("SOURCE_PHYSICS_ARC_MARKER".into()));
}

#[test]
fn the_marked_entries_are_the_models_realized_arcs() {
    let (eb, tim) = (sb::EULER_BERNOULLI, sb::TIMOSHENKO);
    let check = crate::reader_verdict::physics_model_checks;
    let verdict = |doc: &Value, model: &Value| crate::reader_verdict::validate("pressure", doc, Some(model), None);
    for shear_model in [eb, tim] {
        let doc = document(shear_model, true);
        assert_eq!(check(&doc, &model(shear_model, true)), Ok(()), "{shear_model}");
        assert_eq!(verdict(&doc, &model(shear_model, true)), json!({"ok": true}), "{shear_model}");
        // A model that realizes no arc: the marked entry is a straight member's.
        assert_eq!(check(&doc, &model(shear_model, false)), Err("SOURCE_PHYSICS_ARC_MARKER".into()), "{shear_model}");
    }
    // The marker missing on the arc: an Euler-Bernoulli document the evidence
    // reader admits (the arc reads as a straight member), refused against the model.
    let mut unmarked = document(eb, true);
    edit_sections(&mut unmarked, ARC, &CASES, unmark);
    assert_eq!(read(&unmarked), Ok(()));
    assert_eq!(check(&unmarked, &model(eb, true)), Err("SOURCE_PHYSICS_ARC_MARKER".into()));
    assert_eq!(verdict(&unmarked, &model(eb, true))["error"]["code"], "SOURCE_PHYSICS_ARC_MARKER");
    // The marker on a straight member.
    let mut marked = document(eb, true);
    edit_sections(&mut marked, STRAIGHT, &CASES, mark);
    assert_eq!(check(&marked, &model(eb, true)), Err("SOURCE_PHYSICS_ARC_MARKER".into()));
    // Any other member_kind.
    let mut unknown = document(eb, true);
    edit_sections(&mut unknown, ARC, &CASES, |s| s[sb::MEMBER_KIND] = json!("straight_pipe"));
    assert_eq!(check(&unknown, &model(eb, true)), Err("SOURCE_PHYSICS_MEMBER_KIND".into()));
    // bind_shear_model keeps its checks: the arc stating Timoshenko, and the
    // straight member stating a model other than the contract's.
    let mut claimed = document(tim, true);
    edit_sections(&mut claimed, ARC, &CASES, |s| s["beam_theory"] = timoshenko_beam());
    assert_eq!(check(&claimed, &model(tim, true)), Err("SOURCE_PHYSICS_ARC_BEAM_THEORY".into()));
    assert_eq!(check(&document(eb, true), &model(tim, true)), Err("SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT".into()));
    assert_eq!(check(&document(tim, true), &model(eb, true)), Err("SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT".into()));
}

#[test]
fn the_pressure_source_1_composite_refuses_a_realized_arc_by_name() {
    let composite = |doc: &Value| {
        let mut doc = doc.clone();
        for case in doc["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
            case["recovery_method"] = json!("ordinary_dense_structural_v1");
        }
        validate_physics_evidence_in(&doc, true, PressureContract::PressureV3)
    };
    let straight = document(sb::EULER_BERNOULLI, false);
    assert_eq!(composite(&straight), Ok(()));
    // A marked arc section, outside every pressure region.
    assert_eq!(composite(&document(sb::EULER_BERNOULLI, true)), Err("SOURCE_PHYSICS_ARC_UNSUPPORTED".into()));
    // An arc region record, refused before anything else is read from it; the
    // pressure-2 reading of the same evidence does not refuse it by that name.
    let mut region = straight.clone();
    let mut geometry = section(ARC, euler_bernoulli(), true);
    geometry["traversal_forward"] = json!(true);
    geometry["arc_end_tangents_global"] = json!([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
    region["contract_evidence"]["pressure"] = json!([{"load_case_id": CASES[0], "region_id": "region:arc", "geometry": [geometry]}]);
    assert_eq!(composite(&region), Err("SOURCE_PHYSICS_ARC_UNSUPPORTED".into()));
    let ordinary = read(&region).unwrap_err();
    assert_ne!(ordinary, "SOURCE_PHYSICS_ARC_UNSUPPORTED");
}

//! RT-A3 reducer increment 2a (DEL-04-07 Design `route-unification-r4.md`
//! §00 C-3/C-4 and §3.1; `pressure-contract.md`, "Tees and pressure
//! junctions" (D-1); DEL-03-05 `rigid-component-stiffness-scaling.md` §§2.1,
//! 2.4, 2.7 and `eccentric-reducer-offset.md` §§2, 3.2; DEL-05-03
//! `section-bases-and-timoshenko.md` §§1, 2.1): the concentric pressure-bore
//! step on the exact route against the frozen independent references
//! (`validation/references/rt_a3_reducer/`), through PP's public entry in both
//! solver modes and under both shear models.
//!
//! Each intended-admission case is authored as a current
//! `3.0.0/exact_pressure_v3` document from the case's own inputs: a large
//! upstream member, the reducer's mapped body member and a small downstream
//! member on one x-axis line, each its own pressure region whose two end nodes
//! are `transfers_to_wall` terminals, so each body end is a D-1 junction. The
//! reducer maps the body as `mechanics_geometry_only` with the case's DEC-045
//! scale; its end sizes are descriptive metadata only. Each published value is
//! compared at |observed − expected| ≤ 1e-9·max(|expected|, zero_scale(family)),
//! the RT-A3 tee binding's criterion, with each family's zero scale the
//! largest magnitude of that family in the case (rotation and moment from the
//! translation and force scales over the case's shortest and longest member;
//! a fully held case's translation scale from its wall forces' elastic
//! stretch). Transverse translations, rotations, shears and moments are
//! analytically zero on this unloaded collinear chain.
//!
//! The omitted-cap case (R2A-FREE-N-DROPCAP) is a mutation control the product
//! never publishes: a compensated terminal that would omit the cap is refused.
//! A mismatch here is the product's to fix, never the reference's.

use open_pipe_stress_product_physics::result_export::semantic_contract;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const REFERENCES: &str = include_str!("../../../validation/references/rt_a3_reducer/references.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const SHEAR_MODELS: [&str; 2] = ["euler_bernoulli", "timoshenko_annulus_energy_v1"];
const CASE: &str = "case:pressure";
const REDUCER: &str = "component:reducer";
const BODY: &str = "pipe:body";
const PROVENANCE: &str = "invented_rt_a3_reducer_reference";
const BASES: [&str; 2] = ["nominal_section", "reduced_section"];
const REDUCER_IN_REGION: &str = "EXACT_PRESSURE_REDUCER_IN_REGION";
const REDUCER_TEXT_HEAD: &str = "reducer maps a member of this pressure region but is not the admitted concentric pressure-bore step";
const PRESSURES_DIFFER: &str = "EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER";
const OUTSIDE_REGIONS: &str = "EXACT_PRESSURE_JUNCTION_PIPE_OUTSIDE_REGIONS";
const SHARED_TERMINAL: &str = "EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED";
const LAME: [(&str, &str, &str); 4] = [
    ("radial_inner", "pipe_lame_radial_stress_v2", "lame_inner_radial_stress"),
    ("radial_outer", "pipe_lame_radial_stress_v2", "lame_outer_radial_stress"),
    ("hoop_inner", "pipe_lame_hoop_stress_v2", "lame_inner_hoop_stress"),
    ("hoop_outer", "pipe_lame_hoop_stress_v2", "lame_outer_hoop_stress"),
];

fn references() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}

fn case<'a>(references: &'a Value, label: &str) -> &'a Value {
    references["cases"].as_array().unwrap().iter().find(|c| c["label"] == label).unwrap_or_else(|| panic!("no case {label}"))
}

/// A reference input number, written as an integer or a ratio "a/b" of two
/// integers (input data only; expected values are read from `value`).
fn ratio(text: &Value) -> f64 {
    let text = text.as_str().unwrap_or_else(|| panic!("not a ratio string: {text}"));
    match text.split_once('/') {
        Some((a, b)) => a.parse::<f64>().unwrap() / b.parse::<f64>().unwrap(),
        None => text.parse::<f64>().unwrap(),
    }
}

/// The frozen binary64 value of an expected quantity.
fn value(v: &Value) -> f64 {
    v["value"].as_f64().unwrap_or_else(|| panic!("no value in {v}"))
}

fn quantity(v: f64, unit: &str) -> Value {
    json!({"value": v, "unit": unit})
}

fn node_id(name: &Value) -> String {
    format!("node:{}", name.as_str().unwrap())
}

fn support_id(name: &str) -> String {
    format!("support:{name}")
}

fn members(case: &Value) -> &Vec<Value> {
    case["results"]["members"].as_array().unwrap()
}

/// The case as a current exact document under `shear`.
fn document(references: &Value, case: &Value, shear: &str) -> Value {
    let model = &references["model"];
    let inputs = &case["inputs"];
    let members = members(case);
    let nodes = inputs["nodes_x_m"].as_object().unwrap().iter().map(|(name, x)| {
        json!({"id": format!("node:{name}"), "position": {"x": ratio(x), "y": 0.0, "z": 0.0}, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let pipes = members.iter().map(|member| {
        let section = &model["sections"][member["section"].as_str().unwrap()];
        let length = |key: &str| quantity(ratio(&section[key]), "m");
        json!({"id": member["id"], "from": node_id(&member["from"]), "to": node_id(&member["to"]),
            "section": {"outside_diameter": length("D_m"), "wall_thickness": length("t_m"),
                "mill_tolerance": length("c_m_m"), "corrosion_allowance": length("c_a_m")},
            "material": "material:rt-a3-reducer", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let supports = inputs["anchors_all_six_dof"].as_array().unwrap().iter().map(|name| {
        json!({"id": support_id(name.as_str().unwrap()), "node": node_id(name), "family": "anchor",
            "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let regions = members.iter().map(|member| {
        let terminal = |end: &str| json!({"node_ref": node_id(&member[end]), "closure_transfer": "transfers_to_wall", "provenance": PROVENANCE});
        json!({"id": member["region"], "member_pipe_ids": [member["id"]],
            "pressure_basis": "internal_differential_zero_external_v1",
            "pressure": quantity(ratio(&member["pressure_Pa"]), "Pa"),
            "terminals": [terminal("from"), terminal("to")], "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let body = members.iter().find(|member| member["role"] == "reducer_body").unwrap();
    assert_eq!(body["id"], BODY);
    // Descriptive end sizes: the outside diameter of the other member at each
    // body end. They supply no bore and enter no number.
    let size_at = |end: &str| {
        let other = members.iter().find(|m| m["id"] != BODY && (m["from"] == body[end] || m["to"] == body[end])).unwrap();
        quantity(ratio(&model["sections"][other["section"].as_str().unwrap()]["D_m"]), "m")
    };
    let reducer = json!({"id": REDUCER, "kind": "reducer", "node": node_id(&body["from"]),
        "geometry": {"rigid_pipe_ref": BODY, "end_a_size": size_at("from"), "end_b_size": size_at("to")},
        "modifiers": {"stiffness_scaling_user_value": quantity(ratio(&body["stiffness_scale"]), "1"),
            "source_reference": "invented RT-A3 reducer 2a scale"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROVENANCE});
    assert_eq!(model["material"]["nu"], "3/10");
    json!({"model": {"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear},
        "project": {"id": "project:rt-a3-reducer", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes,
        "materials": [{"id": "material:rt-a3-reducer", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": quantity(ratio(&model["material"]["E_Pa"]), "Pa"), "poisson_ratio": quantity(0.3, "1"), "provenance": PROVENANCE}],
        "supports": supports, "components": [reducer],
        "load_cases": [{"id": CASE, "primitive_loads": [], "pressure_regions": regions, "provenance": PROVENANCE}],
        "combinations": []},
        "materials": []})
}

fn solve(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).expect("the entry accepts the document")).unwrap()
}

fn diagnostics(envelope: &Value) -> &Vec<Value> {
    envelope["diagnostics"].as_array().unwrap()
}

fn with_code<'a>(envelope: &'a Value, code: &str) -> Vec<&'a Value> {
    diagnostics(envelope).iter().filter(|d| d["code"] == code).collect()
}

fn basis_of(row: &Value) -> &str {
    let basis = row["metadata"]["basis"].as_str().unwrap_or("");
    basis.split("; section_basis=").nth(1).unwrap_or("")
}

/// The one published case row matching.
fn row(envelope: &Value, kind: &str, entity: &str, location: Option<&str>, component: Option<&str>, basis: &str) -> f64 {
    let found = envelope["results"].as_array().unwrap().iter().filter(|r| {
        r["kind"] == kind && r["entity_ref"] == entity && basis_of(r) == basis
            && location.is_none_or(|l| r["metadata"]["location"] == l)
            && component.is_none_or(|c| r["metadata"]["component"] == c)
            && r["basis_ref"]["ref_id"] == CASE
    }).collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{kind} {entity} {location:?} {component:?} [{basis}]: {} rows", found.len());
    found[0]["value"].as_f64().unwrap()
}

fn close(observed: f64, expected: f64, scale: f64, what: &str) {
    let tolerance = 1e-9 * expected.abs().max(scale);
    assert!((observed - expected).abs() <= tolerance, "{what}: observed {observed:e}, expected {expected:e} (tolerance {tolerance:e})");
}

/// The zero scales of one case's families (module docs).
struct Scales {
    translation: f64,
    rotation: f64,
    force: f64,
    moment: f64,
    stress: f64,
}

fn largest<'a>(values: impl Iterator<Item = &'a Value>) -> f64 {
    values.map(|v| value(v).abs()).fold(0.0, f64::max)
}

fn scales(references: &Value, case: &Value) -> Scales {
    let results = &case["results"];
    let members = members(case);
    let lengths = members.iter().map(|m| ratio(&m["length_m"])).collect::<Vec<_>>();
    let (shortest, longest) = (lengths.iter().copied().fold(f64::INFINITY, f64::min), lengths.iter().copied().fold(0.0, f64::max));
    let force = largest(members.iter().map(|m| &m["wall_force_N_w_N"]).chain(results["reaction_x_N"].as_object().unwrap().values()));
    let mut translation = largest(results["axial_displacement_x_m"].as_object().unwrap().values().chain(members.iter().map(|m| &m["elongation_m"])));
    if translation == 0.0 {
        // Fully held: the wall force's elastic stretch N_w L / (s E A_n).
        let e = ratio(&references["model"]["material"]["E_Pa"]);
        translation = members.iter().map(|m| {
            let area = value(&references["model"]["sections"][m["section"].as_str().unwrap()]["A_nominal_m2"]);
            value(&m["wall_force_N_w_N"]).abs() * ratio(&m["length_m"]) / (ratio(&m["stiffness_scale"]) * e * area)
        }).fold(0.0, f64::max);
    }
    let mut stress = members.iter().map(|m| ratio(&m["pressure_Pa"]).abs()).fold(0.0, f64::max);
    for m in members {
        for basis in BASES {
            stress = stress.max(value(&m["axial_membrane_stress_Pa"][basis]["axial_membrane_sigma_m"]).abs());
            stress = stress.max(largest(lame(references, m)[basis].as_object().unwrap().values()));
        }
    }
    assert!(translation > 0.0 && force > 0.0 && stress > 0.0);
    Scales { translation, rotation: translation / shortest, force, moment: force * longest, stress }
}

/// The member's Lamé rows: its section's annulus at its region's pressure.
fn lame<'a>(references: &'a Value, member: &Value) -> &'a Value {
    let key = format!("{}@{}Pa", member["section"].as_str().unwrap(), member["pressure_Pa"].as_str().unwrap());
    let rows = &references["lame_rows_Pa"][key.as_str()];
    assert!(rows.is_object(), "no Lamé rows {key}");
    rows
}

/// Every published row of one admitted case against its reference.
fn check_numbers(references: &Value, case: &Value, envelope: &Value, label: &str) {
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {:#?}", envelope["diagnostics"]);
    let blocking = diagnostics(envelope).iter().filter(|d| d["severity"] == "blocking").count();
    assert_eq!(blocking, 0, "{label}");
    assert!(with_code(envelope, REDUCER_IN_REGION).is_empty(), "{label}");
    let s = scales(references, case);
    let results = &case["results"];
    for (name, ux) in results["axial_displacement_x_m"].as_object().unwrap() {
        let node = format!("node:{name}");
        let what = |q: &str| format!("{label} {q} {node}");
        close(row(envelope, "global_nodal_displacement_x", &node, None, None, ""), 1000.0 * value(ux), 1000.0 * s.translation, &what("ux"));
        for kind in ["global_nodal_displacement_y", "global_nodal_displacement_z"] {
            close(row(envelope, kind, &node, None, None, ""), 0.0, 1000.0 * s.translation, &what(kind));
        }
        for kind in ["global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z"] {
            close(row(envelope, kind, &node, None, None, ""), 0.0, s.rotation, &what(kind));
        }
    }
    for (name, rx) in results["reaction_x_N"].as_object().unwrap() {
        let support = support_id(name);
        let reaction = |component: &str| row(envelope, "support_reaction_component_v2", &support, None, Some(component), "");
        close(reaction("Fx"), value(rx), s.force, &format!("{label} {support} Fx"));
        for component in ["Fy", "Fz"] {
            close(reaction(component), 0.0, s.force, &format!("{label} {support} {component}"));
        }
        for component in ["Mx", "My", "Mz"] {
            close(reaction(component), 0.0, s.moment, &format!("{label} {support} {component}"));
        }
    }
    for member in members(case) {
        let pipe = member["id"].as_str().unwrap();
        let what = |q: &str| format!("{label} {pipe} {q}");
        for location in ["end_i", "end_j"] {
            close(row(envelope, "pipe_wall_axial_force_v2", pipe, Some(location), None, ""), value(&member["wall_force_N_w_N"]), s.force, &what(location));
            close(row(envelope, "pipe_effective_axial_force_v2", pipe, Some(location), None, ""), value(&member["effective_force_S_N"]), s.force, &what(location));
            for kind in ["element_local_shear_force_y", "element_local_shear_force_z"] {
                close(row(envelope, kind, pipe, Some(location), None, ""), 0.0, s.force, &what(kind));
            }
            for kind in ["element_local_torsional_moment", "element_local_bending_moment_y", "element_local_bending_moment_z"] {
                close(row(envelope, kind, pipe, Some(location), None, ""), 0.0, s.moment, &what(kind));
            }
        }
        // The node's action on the member in member axes: -N_w at from, +N_w at to.
        for (location, key) in [("end_i", "end_action_axial_local_from_N"), ("end_j", "end_action_axial_local_to_N")] {
            close(row(envelope, "pipe_wall_endpoint_action_v2", pipe, Some(location), Some("wall_axial_end_action"), ""),
                value(&member[key]), s.force, &what(key));
        }
        let lame = lame(references, member);
        for basis in BASES {
            let sigma = value(&member["axial_membrane_stress_Pa"][basis]["axial_membrane_sigma_m"]);
            for location in ["end_i", "end_j"] {
                close(row(envelope, "pipe_axial_membrane_stress_v2", pipe, Some(location), None, basis), sigma, s.stress, &what(basis));
            }
            for (key, kind, component) in LAME {
                close(row(envelope, kind, pipe, Some("end_i"), Some(component), basis), value(&lame[basis][key]), s.stress, &what(&format!("{key} {basis}")));
            }
        }
    }
}

fn stress_kind(kind: &str) -> bool {
    semantic_contract::pressure_rows_contract()["rows"].as_array().unwrap().iter()
        .any(|r| r["kind"] == kind && r["source_physical_semantic_dimension"] == "stress")
}

/// DEL-03-05 §2.4 / W-15 slice d: every stress row of the body, and only its,
/// carries the reference-section record, and no headline lies on the body.
fn check_body_record(envelope: &Value, scale: f64, label: &str) {
    let record = json!({"component_ref": REDUCER, "stiffness_scale": scale, "section_role": "reference_pipe_section"});
    let mut tagged = 0;
    for r in envelope["results"].as_array().unwrap() {
        let expected = r["entity_ref"] == BODY && stress_kind(r["kind"].as_str().unwrap());
        match r.get("component_body") {
            Some(found) => {
                assert!(expected, "{label}: record on {}", r["id"]);
                assert_eq!(found, &record, "{label}: {}", r["id"]);
                tagged += 1;
            }
            None => assert!(!expected, "{label}: no record on {}", r["id"]),
        }
    }
    assert!(tagged > 0, "{label}");
    for key in ["max_open_formula_stress_nominal_section", "max_open_formula_stress_reduced_section"] {
        let location = envelope["summary"][key]["location_ref"].as_str().unwrap_or_else(|| panic!("{label}: no {key}"));
        assert_ne!(location, BODY, "{label}: {key}");
    }
}

fn admitted_cases(references: &Value) -> Vec<&Value> {
    references["cases"].as_array().unwrap().iter().filter(|c| c["admitted_case"] == true).collect()
}

#[test]
fn every_admitted_reference_case_solves_on_the_exact_route_within_its_criterion() {
    let references = references();
    let mut checked = 0;
    for case in admitted_cases(&references) {
        let id = case["label"].as_str().unwrap();
        let scale = ratio(&members(case).iter().find(|m| m["id"] == BODY).unwrap()["stiffness_scale"]);
        for shear in SHEAR_MODELS {
            let document = document(&references, case, shear);
            for mode in MODES {
                let label = format!("{id} {shear} {mode:?}");
                let envelope = solve(&document, mode);
                check_numbers(&references, case, &envelope, &label);
                let verdict = open_pipe_stress_product_physics::result_export::reader_verdict::validate(
                    "pressure", &envelope, Some(&document["model"]), None,
                );
                assert_eq!(verdict["ok"], true, "{label}: {verdict:#?}");
                check_body_record(&envelope, scale, &label);
                // D-1: only the unequal-pressure control warns, at junction F.
                let differ = with_code(&envelope, PRESSURES_DIFFER);
                if id == "R2A-HELD-PDIFF" {
                    assert_eq!(differ.len(), 1, "{label}: {differ:#?}");
                    assert_eq!(differ[0]["severity"], "warning");
                    assert_eq!(differ[0]["affected_refs"], json!([CASE, "node:F", "region:body", "region:dn"]));
                } else {
                    assert!(differ.is_empty(), "{label}");
                }
                assert!(with_code(&envelope, OUTSIDE_REGIONS).is_empty(), "{label}");
            }
        }
        checked += 1;
    }
    assert_eq!(checked, 10);
}

/// R2A-FREE-N-DROPCAP: omitting the body's cap at junction N would make the
/// junction anchor carry -p*A_i,body. No admitted document omits it: with
/// every terminal transferring the anchor carries 0 (R2A-FREE-N), and a
/// compensated terminal at N, on either region, is refused by name.
#[test]
fn the_omitted_cap_control_is_never_published() {
    let references = references();
    let control = case(&references, "R2A-FREE-N-DROPCAP");
    assert_eq!(control["admitted_case"], false);
    let wrong = value(&control["results"]["reaction_x_N"]["N"]);
    assert!(wrong < -6.0e5);
    let base = document(&references, case(&references, "R2A-FREE-N"), SHEAR_MODELS[0]);
    for mode in MODES {
        let observed = row(&solve(&base, mode), "support_reaction_component_v2", "support:N", None, Some("Fx"), "");
        assert!((observed - wrong).abs() > 6.0e5, "{mode:?}: {observed}");
    }
    for (region, terminal) in [("region:body", 0), ("region:up", 1)] {
        let mut document = base.clone();
        let regions = document["model"]["load_cases"][0]["pressure_regions"].as_array_mut().unwrap();
        let found = regions.iter_mut().find(|r| r["id"] == region).unwrap();
        assert_eq!(found["terminals"][terminal]["node_ref"], "node:N");
        found["terminals"][terminal]["closure_transfer"] = json!("separately_supported_or_compensated");
        for mode in MODES {
            let envelope = solve(&document, mode);
            assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{region} {mode:?}");
            assert!(with_code(&envelope, SHARED_TERMINAL).iter().any(|d| d["severity"] == "blocking"), "{region} {mode:?}");
            // The step needs transfers_to_wall at every terminal of its ends.
            assert!(with_code(&envelope, REDUCER_IN_REGION).iter().any(|d| d["severity"] == "blocking"), "{region} {mode:?}");
            assert_eq!(envelope["results"], json!([]), "{region} {mode:?}");
        }
    }
}

fn regions_mut(document: &mut Value) -> &mut Vec<Value> {
    document["model"]["load_cases"][0]["pressure_regions"].as_array_mut().unwrap()
}

fn model_array<'a>(document: &'a mut Value, key: &str) -> &'a mut Vec<Value> {
    document["model"][key].as_array_mut().unwrap()
}

fn position_mut<'a>(document: &'a mut Value, node: &str) -> &'a mut Value {
    &mut model_array(document, "nodes").iter_mut().find(|n| n["id"] == node).unwrap()["position"]
}

fn reducer_mut(document: &mut Value) -> &mut Value {
    model_array(document, "components").iter_mut().find(|c| c["id"] == REDUCER).unwrap()
}

/// Refused by name in both modes, exactly once, with `region`'s references.
fn assert_reducer_refused(document: &Value, region: &str, label: &str) {
    for mode in MODES {
        let envelope = solve(document, mode);
        assert_ne!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label} {mode:?}");
        let found = with_code(&envelope, REDUCER_IN_REGION);
        assert_eq!(found.len(), 1, "{label} {mode:?}: {:#?}", envelope["diagnostics"]);
        assert_eq!(found[0]["severity"], "blocking", "{label}");
        assert_eq!(found[0]["affected_refs"], json!([CASE, region, REDUCER, BODY]), "{label}");
        assert!(found[0]["message"].as_str().unwrap().starts_with(REDUCER_TEXT_HEAD), "{label}");
    }
}

fn assert_solved_without_refusal(document: &Value, label: &str) -> Vec<Value> {
    MODES.iter().map(|&mode| {
        let envelope = solve(document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label} {mode:?}: {:#?}", envelope["diagnostics"]);
        assert!(with_code(&envelope, REDUCER_IN_REGION).is_empty(), "{label} {mode:?}");
        envelope
    }).collect()
}

/// Each published row's value bits by row ID.
fn values(envelope: &Value) -> BTreeMap<String, u64> {
    envelope["results"].as_array().unwrap().iter()
        .map(|r| (r["id"].as_str().unwrap().to_string(), r["value"].as_f64().unwrap().to_bits()))
        .collect()
}

/// Absent and explicit zero offsets (signed zeros included) declare the same
/// concentric body: admitted with identical numbers; the zero offset keeps
/// W-15c's realized check. A nonzero offset is refused even where W-15c would
/// realize it, and so is the same slanted (RS-like eccentric) geometry with
/// the slot absent, on the built coordinates.
#[test]
fn offsets_and_slanted_bodies() {
    let references = references();
    let held = case(&references, "R2A-HELD");
    let base = document(&references, held, SHEAR_MODELS[0]);
    let plain = assert_solved_without_refusal(&base, "absent offset");
    for (label, offset) in [("zero offset", json!({"x": 0.0, "y": 0.0, "z": 0.0, "unit": "mm"})),
        ("signed zero offset", json!({"x": -0.0, "y": 0.0, "z": -0.0, "unit": "m"}))] {
        let mut document = base.clone();
        reducer_mut(&mut document)["geometry"]["eccentric_offset"] = offset;
        for (envelope, plain) in assert_solved_without_refusal(&document, label).iter().zip(&plain) {
            check_numbers(&references, held, envelope, label);
            assert_eq!(values(envelope), values(plain), "{label}");
            assert!(!with_code(envelope, "RIGID_COMPONENT_ECCENTRIC_OFFSET_REALIZED").is_empty(), "{label}");
        }
    }
    // RS-like: the body rises 25 mm in z from N to F, the downstream member
    // stays parallel to x; the declared offset matches the geometry.
    let mut slanted = base.clone();
    for node in ["node:F", "node:Q"] {
        position_mut(&mut slanted, node)["z"] = json!(0.025);
    }
    let mut declared = slanted.clone();
    reducer_mut(&mut declared)["geometry"]["eccentric_offset"] = json!({"x": 0.0, "y": 0.0, "z": 25.0, "unit": "mm"});
    assert_reducer_refused(&declared, "region:body", "declared eccentric offset");
    assert_reducer_refused(&slanted, "region:body", "slanted body, slot absent");
    // A nonzero offset on collinear geometry is refused by the step rule too.
    let mut collinear = base.clone();
    reducer_mut(&mut collinear)["geometry"]["eccentric_offset"] = json!({"x": 0.0, "y": 0.0, "z": 25.0, "unit": "mm"});
    for mode in MODES {
        let envelope = solve(&collinear, mode);
        assert!(with_code(&envelope, REDUCER_IN_REGION).iter().any(|d| d["severity"] == "blocking"), "{mode:?}");
    }
    // The 64*epsilon representation guard on the unit directions: a 1e-16 m
    // rise over the 0.5 m body (cross norm about 2e-16) is admitted, a 1e-12 m
    // rise (about 2e-12) is refused. These coordinates are entered directly as binary64
    // metre values, so normalization applies the identity conversion.
    for (rise, admitted) in [(1.0e-16, true), (1.0e-12, false)] {
        let mut document = base.clone();
        for node in ["node:F", "node:Q"] {
            position_mut(&mut document, node)["z"] = json!(rise);
        }
        if admitted {
            assert_solved_without_refusal(&document, &format!("rise {rise:e}"));
        } else {
            assert_reducer_refused(&document, "region:body", &format!("rise {rise:e}"));
        }
    }
}

/// Region, mapping and neighbour conditions of the step: each failure is
/// refused by name; an unmapped reducer and a body outside every region keep
/// their existing admission.
#[test]
fn step_topology_conditions() {
    let references = references();
    let free = document(&references, case(&references, "R2A-FREE-N"), SHEAR_MODELS[0]);
    let held = document(&references, case(&references, "R2A-HELD"), SHEAR_MODELS[0]);

    // An equal-bore multi-member region (R2A-FREE-N's body is on L like the
    // upstream member): the body joins region:up, which ends at F.
    let mut merged = free.clone();
    regions_mut(&mut merged).retain(|r| r["id"] != "region:body");
    let up = regions_mut(&mut merged).iter_mut().find(|r| r["id"] == "region:up").unwrap();
    up["member_pipe_ids"] = json!(["pipe:up", BODY]);
    up["terminals"][1]["node_ref"] = json!("node:F");
    assert_reducer_refused(&merged, "region:up", "multi-member region");

    // The mapped member does not end at the reducer's node.
    let mut misplaced = held.clone();
    reducer_mut(&mut misplaced)["node"] = json!("node:P");
    assert_reducer_refused(&misplaced, "region:body", "mapping not at the node");

    // A dry body end: the downstream member belongs to no region.
    let mut dry = held.clone();
    regions_mut(&mut dry).retain(|r| r["id"] != "region:dn");
    assert_reducer_refused(&dry, "region:body", "dry body end");

    // A free body end: no member beyond F.
    let mut open = free.clone();
    regions_mut(&mut open).retain(|r| r["id"] != "region:dn");
    model_array(&mut open, "pipe_segments").retain(|p| p["id"] != "pipe:dn");
    model_array(&mut open, "nodes").retain(|n| n["id"] != "node:Q");
    assert_reducer_refused(&open, "region:body", "free body end");

    // Two other pipes at F: a pressurized side branch, with and without a tee record.
    let mut branched = held.clone();
    model_array(&mut branched, "nodes").push(json!({"id": "node:G", "position": {"x": 4.5, "y": 1.0, "z": 0.0}, "provenance": PROVENANCE}));
    let mut side = model_array(&mut branched, "pipe_segments").iter().find(|p| p["id"] == "pipe:dn").unwrap().clone();
    side["id"] = json!("pipe:side");
    side["to"] = json!("node:G");
    side["y_reference"] = json!({"x": 1.0, "y": 0.0, "z": 0.0});
    model_array(&mut branched, "pipe_segments").push(side);
    let mut region = regions_mut(&mut branched).iter().find(|r| r["id"] == "region:dn").unwrap().clone();
    region["id"] = json!("region:side");
    region["member_pipe_ids"] = json!(["pipe:side"]);
    region["terminals"][1]["node_ref"] = json!("node:G");
    regions_mut(&mut branched).push(region);
    assert_reducer_refused(&branched, "region:body", "two neighbours");
    let mut tee = branched.clone();
    model_array(&mut tee, "components").push(json!({"id": "component:tee", "kind": "tee", "node": "node:F",
        "geometry": {"branch_header_pipe_ref": "pipe:dn", "branch_branch_pipe_ref": "pipe:side", "branch_geometry_source_reference": "invented"},
        "modifiers": {"branch_header_sif_user_value": quantity(1.5, "none"), "branch_branch_sif_user_value": quantity(1.5, "none"),
            "flexibility_factor_user_value": quantity(1.0, "none"), "source_reference": "invented_rt_a3_reducer"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only", "rule_check_consumption": "user_rule_pack_inputs_only"},
        "provenance": PROVENANCE}));
    assert_reducer_refused(&tee, "region:body", "tee at a body end");

    // A neighbour that is a bend member is not straight run.
    let mut bend = held.clone();
    model_array(&mut bend, "components").push(json!({"id": "component:bend", "kind": "bend", "node": "node:Q",
        "geometry": {"bend_pipe_ref": "pipe:dn"}, "provenance": PROVENANCE}));
    assert_reducer_refused(&bend, "region:body", "bend neighbour");

    // Overlap stays refused (and the overlapping multi-member listing too).
    let mut overlap = held.clone();
    regions_mut(&mut overlap).iter_mut().find(|r| r["id"] == "region:up").unwrap()["member_pipe_ids"] = json!(["pipe:up", BODY]);
    for mode in MODES {
        let envelope = solve(&overlap, mode);
        assert_ne!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
        assert!(with_code(&envelope, "PRESSURE_REGION_PIPE_OVERLAP").iter().any(|d| d["severity"] == "blocking"), "{mode:?}");
        assert!(with_code(&envelope, REDUCER_IN_REGION).iter().any(|d| d["severity"] == "blocking"), "{mode:?}");
    }

    // An internal branch of a neighbouring region stays refused although the
    // step itself holds: the upstream member is split at M with a dry stub.
    let mut internal = free.clone();
    for (id, x, y) in [("node:M", 2.0, 0.0), ("node:S", 2.0, 1.0)] {
        model_array(&mut internal, "nodes").push(json!({"id": id, "position": {"x": x, "y": y, "z": 0.0}, "provenance": PROVENANCE}));
    }
    let pipes = model_array(&mut internal, "pipe_segments");
    let up = pipes.iter_mut().find(|p| p["id"] == "pipe:up").unwrap();
    up["to"] = json!("node:M");
    let mut rest = up.clone();
    rest["id"] = json!("pipe:up-rest");
    rest["from"] = json!("node:M");
    rest["to"] = json!("node:N");
    let mut stub = rest.clone();
    stub["id"] = json!("pipe:stub");
    stub["to"] = json!("node:S");
    stub["y_reference"] = json!({"x": 1.0, "y": 0.0, "z": 0.0});
    pipes.push(rest);
    pipes.push(stub);
    regions_mut(&mut internal).iter_mut().find(|r| r["id"] == "region:up").unwrap()["member_pipe_ids"] = json!(["pipe:up", "pipe:up-rest"]);
    for mode in MODES {
        let envelope = solve(&internal, mode);
        assert_ne!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
        assert!(with_code(&envelope, "PRESSURE_REGION_INTERNAL_BRANCH").iter().any(|d| d["severity"] == "blocking"), "{mode:?}: {:#?}", envelope["diagnostics"]);
        assert!(with_code(&envelope, REDUCER_IN_REGION).is_empty(), "{mode:?}");
    }

    // Unchanged admissions: a reducer with no mapped member, and a body outside every region.
    let mut unmapped = held.clone();
    reducer_mut(&mut unmapped)["geometry"].as_object_mut().unwrap().remove("rigid_pipe_ref");
    assert_solved_without_refusal(&unmapped, "unmapped reducer");
    let mut outside = held.clone();
    regions_mut(&mut outside).retain(|r| r["id"] != "region:body");
    for envelope in assert_solved_without_refusal(&outside, "body outside every region") {
        assert!(with_code(&envelope, PRESSURES_DIFFER).is_empty() && with_code(&envelope, OUTSIDE_REGIONS).is_empty());
    }
}

/// End sizes are descriptive: swapped, changed or removed, every published
/// value is bit-identical.
#[test]
fn end_sizes_do_not_enter_the_numbers() {
    let references = references();
    let base = document(&references, case(&references, "R2A-HELD"), SHEAR_MODELS[0]);
    let plain = assert_solved_without_refusal(&base, "authored end sizes");
    let mut swapped = base.clone();
    let geometry = &mut reducer_mut(&mut swapped)["geometry"];
    let (a, b) = (geometry["end_a_size"].clone(), geometry["end_b_size"].clone());
    geometry["end_a_size"] = b;
    geometry["end_b_size"] = a;
    let mut changed = base.clone();
    reducer_mut(&mut changed)["geometry"]["end_a_size"] = quantity(1.0, "m");
    reducer_mut(&mut changed)["geometry"]["end_b_size"] = quantity(10.0, "mm");
    let mut removed = base.clone();
    let geometry = reducer_mut(&mut removed)["geometry"].as_object_mut().unwrap();
    geometry.remove("end_a_size");
    geometry.remove("end_b_size");
    for (label, document) in [("swapped", swapped), ("changed", changed), ("removed", removed)] {
        for (envelope, plain) in assert_solved_without_refusal(&document, label).iter().zip(&plain) {
            assert_eq!(values(envelope), values(plain), "{label}");
        }
    }
}

/// W-15 eligibility still governs a step body: without a scale, or as a
/// marker (`not_solver_consumed`), it is admitted unscaled with its existing
/// warning, and at s = 1 its numbers are R2A-HELD's.
#[test]
fn unscaled_and_marker_bodies_keep_their_warnings() {
    let references = references();
    let held = case(&references, "R2A-HELD");
    assert_eq!(members(held).iter().find(|m| m["id"] == BODY).unwrap()["stiffness_scale"], "1");
    let base = document(&references, held, SHEAR_MODELS[1]);
    let mut unscaled = base.clone();
    reducer_mut(&mut unscaled).as_object_mut().unwrap().remove("modifiers");
    let mut marker = base.clone();
    reducer_mut(&mut marker)["mechanics_interface"]["solver_consumption"] = json!("not_solver_consumed");
    for (label, document, warning) in [("unscaled", unscaled, "RIGID_COMPONENT_STIFFNESS_INPUT_MISSING"),
        ("marker", marker, "COMPONENT_MECHANICS_INTERFACE_UNSUPPORTED")] {
        for envelope in assert_solved_without_refusal(&document, label) {
            assert!(with_code(&envelope, warning).iter().any(|d| d["severity"] == "warning"), "{label}: {:#?}", envelope["diagnostics"]);
            assert!(with_code(&envelope, "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED").is_empty(), "{label}");
            check_numbers(&references, held, &envelope, label);
        }
    }
}

/// A pressure connector's faces must be internal to one region. Use that
/// valid mixed region and prove it solves without the reducer, so neither a
/// parse error nor an invalid joint-face terminal stands in for the refusal.
#[test]
fn a_valid_pressure_connector_neighbour_stays_outside_the_initial_step() {
    let references = references();
    let mut document = document(&references, case(&references, "R2A-HELD"), SHEAR_MODELS[0]);
    let template: Value = serde_json::from_str(include_str!("fixtures/silent_zero/stress_free_connector_empty.request.json")).unwrap();
    let mut connector = template["model"]["components"][0].clone();
    connector["id"] = json!("component:neighbour-joint");
    connector["node"] = json!("node:Q");
    let law = &mut connector["objective_connector"];
    law["end_i"]["node_ref"] = json!("node:F");
    law["end_j"]["node_ref"] = json!("node:Q");
    law["topology"]["span_ref"] = json!("pipe:dn");
    law["pressure_model"] = json!({
        "kind": "constant_effective_area_reference_v1",
        "effective_area": quantity(0.01, "m^2"),
        "pressure_basis": "internal_differential_zero_external_v1",
        "source_reference": PROVENANCE,
    });
    model_array(&mut document, "components").push(connector);
    model_array(&mut document, "nodes").push(json!({
        "id": "node:R", "position": {"x": 8.5, "y": 0.0, "z": 0.0}, "provenance": PROVENANCE,
    }));
    let mut tail = document["model"]["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == "pipe:dn").unwrap().clone();
    tail["id"] = json!("pipe:tail");
    tail["from"] = json!("node:Q");
    tail["to"] = json!("node:R");
    tail["section"] = document["model"]["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == BODY).unwrap()["section"].clone();
    model_array(&mut document, "pipe_segments").push(tail);
    let region = regions_mut(&mut document).iter_mut().find(|r| r["id"] == "region:dn").unwrap();
    region["id"] = json!("region:body-and-joint");
    region["member_pipe_ids"] = json!([BODY, "pipe:tail"]);
    region["member_connector_ids"] = json!(["component:neighbour-joint"]);
    region["terminals"][0]["node_ref"] = json!("node:N");
    region["terminals"][1]["node_ref"] = json!("node:R");
    regions_mut(&mut document).retain(|r| r["id"] != "region:body");

    let mut without_reducer = document.clone();
    model_array(&mut without_reducer, "components").retain(|c| c["id"] != REDUCER);
    assert_solved_without_refusal(&without_reducer, "valid connector neighbour without reducer");
    assert_reducer_refused(&document, "region:body-and-joint", "valid connector neighbour with reducer");
}

/// The admitted run is not restricted to a global coordinate axis. Rotate
/// the independent axial case by the exact 3-4-5 direction and project its
/// motions/reactions; local wall forces and stresses keep their meanings.
#[test]
fn the_step_and_its_reference_response_rotate_with_the_entered_geometry() {
    let references = references();
    let case = case(&references, "R2A-HELD-S2");
    for shear in SHEAR_MODELS {
        let mut document = document(&references, case, shear);
        for node in model_array(&mut document, "nodes") {
            let x = node["position"]["x"].as_f64().unwrap();
            node["position"] = json!({"x": 0.6 * x, "y": 0.8 * x, "z": 0.0});
        }
        for pipe in model_array(&mut document, "pipe_segments") {
            pipe["y_reference"] = json!({"x": 0.0, "y": 0.0, "z": 1.0});
        }
        let scale = scales(&references, case);
        for mode in MODES {
            let envelope = solve(&document, mode);
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{shear} {mode:?}: {:#?}", envelope["diagnostics"]);
            for (name, ux) in case["results"]["axial_displacement_x_m"].as_object().unwrap() {
                let node = format!("node:{name}");
                for (kind, projection) in [("global_nodal_displacement_x", 0.6), ("global_nodal_displacement_y", 0.8)] {
                    close(row(&envelope, kind, &node, None, None, ""), 1000.0 * projection * value(ux), 1000.0 * scale.translation, kind);
                }
            }
            for (name, force) in case["results"]["reaction_x_N"].as_object().unwrap() {
                for (component, projection) in [("Fx", 0.6), ("Fy", 0.8), ("Fz", 0.0)] {
                    close(row(&envelope, "support_reaction_component_v2", &support_id(name), None, Some(component), ""), projection * value(force), scale.force, component);
                }
            }
            for member in members(case) {
                let pipe = member["id"].as_str().unwrap();
                close(row(&envelope, "pipe_wall_axial_force_v2", pipe, Some("end_i"), None, ""), value(&member["wall_force_N_w_N"]), scale.force, pipe);
                for basis in BASES {
                    close(row(&envelope, "pipe_axial_membrane_stress_v2", pipe, Some("end_i"), None, basis), value(&member["axial_membrane_stress_Pa"][basis]["axial_membrane_sigma_m"]), scale.stress, pipe);
                }
            }
            check_body_record(&envelope, 2.0, &format!("rotated {shear} {mode:?}"));
        }
    }
}

#[test]
fn the_pressure_reader_binds_the_admitted_body_record_to_the_model() {
    let references = references();
    let document = document(&references, case(&references, "R2A-HELD-S2"), SHEAR_MODELS[0]);
    for mode in MODES {
        let mut envelope = solve(&document, mode);
        let read = |source: &Value| open_pipe_stress_product_physics::result_export::reader_verdict::validate(
            "pressure", source, Some(&document["model"]), None,
        );
        assert_eq!(read(&envelope)["ok"], true, "{mode:?}");
        let row = envelope["results"].as_array_mut().unwrap().iter_mut().find(|r| r.get("component_body").is_some()).unwrap();
        row["component_body"]["component_ref"] = json!("component:forged");
        assert_eq!(read(&envelope)["ok"], false, "{mode:?}: a forged reference-section owner was accepted");
    }
}

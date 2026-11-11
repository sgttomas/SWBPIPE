//! gap, one-way and friction supports on the exact pressure route against the
//! frozen independent references (`validation/references/rt_a3_nonlinear/`),
//! through PP's public entry in both solver modes.
//!
//! Each case is authored as a plain current `3.0.0/exact_pressure_v3`
//! document from the case's own inputs (README binding notes): the S1 header
//! section with an explicit zero mill tolerance and corrosion allowance,
//! `euler_bernoulli` except the F1-T twins (`timoshenko_annulus_energy_v1`),
//! one region over every member with the stated terminal tokens (an explicit
//! `[]` for the p = 0 controls), the anchor, the linear restraint as a
//! `line_stop` record, and each nonlinear support with the product's own
//! fields (`closes_when`, `active_when`, `normal_reaction` or
//! `normal_reaction_source`). O-1's realized arc is a `curved_bend_macro_element`
//! bend over its member, radius 0.3 m, angle π/2 and the user's k = 2, with
//! `y_reference` from the arc centre toward the chord midpoint (the product's
//! bow convention). The authored initial states are seeds only.
//!
//! Every published value of every case is compared at |observed − expected|
//! ≤ 1e-9·max(|expected|, family floor). A family's floor is the largest
//! magnitude of that family in the case (translations include the supports'
//! deciding displacements and separations, forces include their reactions,
//! normals and contacts). Where a family is wholly zero its floor is derived:
//! with no motion at all (a stuck case) translation = force × longest /
//! (E·A_n); rotation = translation / shortest member, translation = rotation × longest,
//! force = E·A_n·translation / longest, moment = force × longest, stress =
//! force / A_n. Nominal and reduced bases coincide (c_m = c_a = 0), so each
//! stress value binds both basis families.
//!
//! The support values bind the product's own rows: the state token, the final
//! deciding displacement and reaction, the explicit or derived friction normal
//! and its source reaction; separation, contact, μN and the stick margin are
//! formed from those published rows by the packet's conventions. A mismatch
//! here is the product's to fix, never the reference's or the tolerance's.
//!
//! Standing (README "Expected standing"): gap-bearing cases publish
//! `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`; the derived-normal p = 0
//! controls publish `NUMERICAL_INTEGRITY_SENSITIVE` with the loop's "friction
//! normal not established" row; every other case's K-D5 outcome is the
//! product's and is only required to be the same in both modes. Every envelope
//! passes the pressure reader with and without its model.
//!
//! G1-P0 (no load of any kind) is not expressible: the product requires an
//! explicit primitive load and refuses it by name (`LOAD_INPUT_MISSING`); the
//! binding pins that refusal and invents no load. The other 20 cases bind.
//!
//! The pressure reader binds the nonlinear rows, their DEC-067 laws and their
//! standings (`result_export::nonlinear_support_evidence`): every
//! forged control is refused by this binding and by the reader, with and
//! without the model, by the code named in the test, except the readers'
//! limits pinned there (a consistently scaled contact; a non-Passed inactive
//! reaction's magnitude, which no published record bounds).
//!
//! Combinations: the EX gate `NONLINEAR_COMBINATION_REQUIRES_SOLVE` and decision
//! OQ-1 are RT-A4's (#98), consumed unchanged: every flat mechanics combination
//! of a nonlinear model is withheld, OQ-1's one solved case at factor 1 plus
//! κ-zero combinations is admitted, and both readers refuse forged gates.

use open_pipe_stress_product_physics::result_export::reader_verdict;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const REFERENCES: &str = include_str!("../../../validation/references/rt_a3_nonlinear/references.json");
/// The RT-A3 increment 4 review's (nl3) loaded B1 counterexample, byte for
/// byte: untouched sparse producer output and its model (`{source, model}`).
const B1_LOADED: &str = include_str!("fixtures/rt_a3_nonlinear/b1_loaded_inactive_reaction.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const PROVENANCE: &str = "invented_rt_a3_nonlinear_reference";
const MATERIAL: &str = "material:rt-a3-nl";
const CASE: &str = "case:main";
const BASES: [&str; 2] = ["nominal_section", "reduced_section"];
const UNQUALIFIED: &str = "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED";
const SENSITIVE: &str = "NUMERICAL_INTEGRITY_SENSITIVE";
const PASSED: &str = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
const NORMAL_NOT_ESTABLISHED: &str = "support support:FRICTION: friction normal not established";
const FAMILY: &str = "EXACT_PRESSURE_FAMILY_NOT_ADMITTED";
const GATE_CODE: &str = "NONLINEAR_COMBINATION_REQUIRES_SOLVE";
/// The gate's text (PF's and EX's `gate_message`).
const GATE_TEXT: &str = "Combination withheld: the model has nonlinear supports, so superposing case states is not a solved state.";
/// The readers' refusal of a gate that is not the one they re-derive.
const COMBINATION_GATE: &str = "SOURCE_PHYSICS_COMBINATION_GATE";
/// Decision OQ-1's sign convention on an admitted combined row.
const OQ1_SIGN: &str = "positive value follows explicit user linear combination of matching source result sign conventions; a solved state plus derived differences or bounds; not an equilibrium state";
/// Cases the model format cannot state as the packet does (see the test).
const INEXPRESSIBLE: [&str; 1] = ["G1-P0"];
const LAME: [(&str, &str, &str); 4] = [
    ("radial_inner", "pipe_lame_radial_stress_v2", "lame_inner_radial_stress"),
    ("radial_outer", "pipe_lame_radial_stress_v2", "lame_outer_radial_stress"),
    ("hoop_inner", "pipe_lame_hoop_stress_v2", "lame_inner_hoop_stress"),
    ("hoop_outer", "pipe_lame_hoop_stress_v2", "lame_outer_hoop_stress"),
];
const DOFS: [(&str, &str, f64); 6] = [
    ("UX", "global_nodal_displacement_x", 1000.0),
    ("UY", "global_nodal_displacement_y", 1000.0),
    ("UZ", "global_nodal_displacement_z", 1000.0),
    ("RX", "global_nodal_rotation_x", 1.0),
    ("RY", "global_nodal_rotation_y", 1.0),
    ("RZ", "global_nodal_rotation_z", 1.0),
];
const ACTIONS: [(&str, &str, &str); 6] = [
    ("FX", "Fx", "element_global_end_force_v1"),
    ("FY", "Fy", "element_global_end_force_v1"),
    ("FZ", "Fz", "element_global_end_force_v1"),
    ("MX", "Mx", "element_global_end_moment_v1"),
    ("MY", "My", "element_global_end_moment_v1"),
    ("MZ", "Mz", "element_global_end_moment_v1"),
];

// ----------------------------------------------------------------- inputs

fn references() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}

fn cases(references: &Value) -> &Vec<Value> {
    references["cases"].as_array().unwrap()
}

fn case<'a>(references: &'a Value, id: &str) -> &'a Value {
    cases(references).iter().find(|c| c["id"] == id).unwrap_or_else(|| panic!("no case {id}"))
}

/// A reference input number, an integer or a ratio "a/b" (input data only;
/// expected values are read from `value`).
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

fn node_id(name: &str) -> String {
    format!("node:{name}")
}

// ----------------------------------------------------------------- documents

/// The plain v3 document of one case (module docs).
fn document(references: &Value, case: &Value) -> Value {
    let model = &references["model"];
    let m = &case["model"];
    let section = &model["section"];
    let length = |key: &str| quantity(ratio(&section[key]), "m");
    let nodes = m["nodes_m"].as_object().unwrap().iter().map(|(name, xyz)| {
        let xyz = xyz.as_array().unwrap();
        json!({"id": node_id(name), "position": {"x": ratio(&xyz[0]), "y": ratio(&xyz[1]), "z": ratio(&xyz[2])}, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let mut components = Vec::new();
    let pipes = m["members"].as_array().unwrap().iter().map(|member| {
        let y_reference = match member["kind"].as_str().unwrap() {
            "straight" => json!({"x": 0.0, "y": 1.0, "z": 0.0}),
            "realized_arc" => {
                let at = |name: &str| m["nodes_m"][name].as_array().unwrap().iter().map(ratio).collect::<Vec<_>>();
                let (a, b) = (at(member["from"].as_str().unwrap()), at(member["to"].as_str().unwrap()));
                let c = member["center_m"].as_array().unwrap().iter().map(ratio).collect::<Vec<_>>();
                let toward = (0..3).map(|k| 0.5 * (a[k] + b[k]) - c[k]).collect::<Vec<_>>();
                assert_eq!(member["angle"], "pi/2");
                components.push(json!({"id": format!("component:{}", member["id"].as_str().unwrap()), "kind": "bend",
                    "node": node_id(member["from"].as_str().unwrap()),
                    "geometry": {"bend_pipe_ref": member["id"], "bend_radius": quantity(ratio(&member["radius_m"]), "m"),
                        "bend_angle": quantity(std::f64::consts::FRAC_PI_2, "rad"),
                        "bend_plane_orientation": "plane of the span chord and the pipe y_reference; the arc bows toward +y_reference"},
                    "modifiers": {"flexibility_factor_user_value": quantity(ratio(&member["flexibility_factor_k"]), "none")},
                    "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}, "provenance": PROVENANCE}));
                json!({"x": toward[0], "y": toward[1], "z": toward[2]})
            }
            other => panic!("member kind {other}"),
        };
        json!({"id": member["id"], "from": node_id(member["from"].as_str().unwrap()), "to": node_id(member["to"].as_str().unwrap()),
            "section": {"outside_diameter": length("D_m"), "wall_thickness": length("t_m"), "mill_tolerance": length("c_m_m"), "corrosion_allowance": length("c_a_m")},
            "material": MATERIAL, "y_reference": y_reference, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let anchor = &m["anchor"];
    let mut supports = vec![json!({"id": anchor["id"], "node": node_id(anchor["node"].as_str().unwrap()), "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE})];
    for r in m["linear_restraints"].as_array().unwrap() {
        supports.push(json!({"id": r["id"], "node": node_id(r["node"].as_str().unwrap()), "family": "line_stop",
            "restraints": [r["dof"]], "provenance": PROVENANCE}));
    }
    for s in m["nonlinear_supports"].as_array().unwrap() {
        let sign = |t: &Value| match t.as_str().unwrap() { "+" => "positive", "-" => "negative", o => panic!("{o}") };
        let nonlinear = match s["kind"].as_str().unwrap() {
            "gap" => json!({"behavior": "gap", "dof": s["dof"], "initial_state": "inactive",
                "closes_when": format!("{}_displacement", sign(&s["closes"])), "gap": quantity(ratio(&s["clearance_m"]), "m")}),
            "one_way" => json!({"behavior": "one_way", "dof": s["dof"], "initial_state": "active",
                "active_when": format!("{}_reaction", sign(&s["sense"]))}),
            "friction" => {
                let mut f = json!({"behavior": "friction", "dof": s["dof"], "initial_state": "sticking",
                    "friction_coefficient": quantity(ratio(&s["mu"]), "1")});
                let normal = &s["normal"];
                match normal["kind"].as_str().unwrap() {
                    "explicit" => f["normal_reaction"] = quantity(ratio(&normal["value_N"]), "N"),
                    "derived" => {
                        let source = m["linear_restraints"].as_array().unwrap().iter()
                            .find(|r| r["node"] == normal["source_node"] && r["dof"] == normal["source_dof"]).unwrap();
                        f["normal_reaction_source"] = json!({"support_ref": source["id"], "dof": normal["source_dof"]});
                    }
                    o => panic!("{o}"),
                }
                f
            }
            o => panic!("{o}"),
        };
        supports.push(json!({"id": s["id"], "node": node_id(s["node"].as_str().unwrap()), "family": "nonlinear", "restraints": [],
            "nonlinear": nonlinear, "provenance": PROVENANCE}));
    }
    let lc = &case["load_case"];
    let p = ratio(&lc["p_Pa"]);
    let regions = if lc["regions"] == "one region over every member" {
        let r = &m["region"];
        vec![json!({"id": "region:all", "member_pipe_ids": r["members"], "pressure_basis": "internal_differential_zero_external_v1",
            "pressure": quantity(p, "Pa"),
            "terminals": r["terminals"].as_object().unwrap().iter().map(|(node, t)| json!({"node_ref": node_id(node), "closure_transfer": t, "provenance": PROVENANCE})).collect::<Vec<_>>(),
            "provenance": PROVENANCE})]
    } else {
        assert_eq!(p, 0.0);
        Vec::new()
    };
    let mut loads = Vec::new();
    for (node, by) in lc["nodal_loads"].as_object().unwrap() {
        for (component, v) in by.as_object().unwrap() {
            let direction = match component.as_str() { "FX" => "global_x", "FY" => "global_y", "FZ" => "global_z", o => panic!("{o}") };
            loads.push(json!({"id": format!("load:{node}:{component}"), "category": "concentrated_force",
                "target": {"type": "node", "node": node_id(node)}, "direction": direction,
                "magnitude": quantity(ratio(v), "N"), "dimension": "force", "provenance": PROVENANCE}));
        }
    }
    let shear = match m["shear_model"].as_str().unwrap() {
        "euler_bernoulli" => "euler_bernoulli",
        "timoshenko" => "timoshenko_annulus_energy_v1",
        o => panic!("{o}"),
    };
    json!({"model": {"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear},
        "project": {"id": "project:rt-a3-nonlinear", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes,
        "materials": [{"id": MATERIAL, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": quantity(ratio(&model["material"]["E_Pa"]), "Pa"), "poisson_ratio": quantity(0.3, "1"), "provenance": PROVENANCE}],
        "supports": supports, "components": components,
        "load_cases": [{"id": "case:main", "primitive_loads": loads, "pressure_regions": regions, "provenance": PROVENANCE}],
        "combinations": []},
        "materials": []})
}

// ----------------------------------------------------------------- solving

fn solve(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).expect("the entry accepts the document")).unwrap()
}

fn diagnostics(envelope: &Value) -> &Vec<Value> {
    envelope["diagnostics"].as_array().unwrap()
}

fn with_code<'a>(envelope: &'a Value, code: &str) -> Vec<&'a Value> {
    diagnostics(envelope).iter().filter(|d| d["code"] == code).collect()
}

/// The reader's verdict: `None` when it accepts, its refusal text otherwise.
fn reader(envelope: &Value, document: Option<&Value>) -> Option<String> {
    let found = reader_verdict::validate("pressure", envelope, document.map(|d| &d["model"]), None);
    (found["ok"] != true).then(|| found["error"].to_string())
}

fn assert_reader_ok(envelope: &Value, document: &Value, label: &str) {
    assert_eq!(reader(envelope, Some(document)), None, "{label}: the reader with the model");
    assert_eq!(reader(envelope, None), None, "{label}: the reader without the model");
}

fn solved(document: &Value, mode: PreviewSolverMode, label: &str) -> Value {
    let envelope = solve(document, mode);
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {:#?}", envelope["diagnostics"]);
    let blocking = diagnostics(&envelope).iter().filter(|d| d["severity"] == "blocking").collect::<Vec<_>>();
    assert!(blocking.is_empty(), "{label}: {blocking:#?}");
    assert!(with_code(&envelope, FAMILY).is_empty(), "{label}");
    assert_eq!(envelope["formulation_basis"]["profile_id"], "exact_pressure_v3", "{label}");
    envelope
}

/// The case's one integrity record: (code, message).
fn standing(envelope: &Value) -> (String, String) {
    let found = diagnostics(envelope).iter()
        .filter(|d| d["affected_refs"] == json!([CASE]) && d["code"].as_str().unwrap_or("").starts_with("NUMERICAL_INTEGRITY_"))
        .collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{found:#?}");
    (found[0]["code"].as_str().unwrap().to_string(), found[0]["message"].as_str().unwrap().to_string())
}

// ----------------------------------------------------------------- rows

fn basis_of(row: &Value) -> &str {
    row["metadata"]["basis"].as_str().unwrap_or("").split("; section_basis=").nth(1).unwrap_or("")
}

fn rows<'a>(envelope: &'a Value, kind: &str, entity: &str, location: Option<&str>, component: Option<&str>, basis: &str) -> Vec<&'a Value> {
    envelope["results"].as_array().unwrap().iter().filter(|r| {
        r["kind"] == kind && r["entity_ref"] == entity && basis_of(r) == basis
            && location.is_none_or(|l| r["metadata"]["location"] == l)
            && component.is_none_or(|c| r["metadata"]["component"] == c)
            && r["basis_ref"]["ref_type"] == "load_case" && r["basis_ref"]["ref_id"] == CASE
    }).collect()
}

/// The one published row matching, if any.
fn maybe_row(envelope: &Value, kind: &str, entity: &str, location: Option<&str>, component: Option<&str>, basis: &str) -> Option<f64> {
    let found = rows(envelope, kind, entity, location, component, basis);
    assert!(found.len() <= 1, "{kind} {entity} {location:?} {component:?} [{basis}]: {} rows", found.len());
    found.first().map(|r| r["value"].as_f64().unwrap())
}

fn row(envelope: &Value, kind: &str, entity: &str, location: Option<&str>, component: Option<&str>, basis: &str) -> f64 {
    maybe_row(envelope, kind, entity, location, component, basis)
        .unwrap_or_else(|| panic!("no {kind} {entity} {location:?} {component:?} [{basis}]"))
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Family {
    Translation,
    Rotation,
    Force,
    Moment,
    Stress,
}

#[derive(Clone, Copy, Debug)]
struct Floors {
    translation: f64,
    rotation: f64,
    force: f64,
    moment: f64,
    stress: f64,
}

impl Floors {
    fn of(self, family: Family) -> f64 {
        match family {
            Family::Translation => self.translation,
            Family::Rotation => self.rotation,
            Family::Force => self.force,
            Family::Moment => self.moment,
            Family::Stress => self.stress,
        }
    }
}

fn close(observed: f64, expected: f64, floor: f64, what: &str) {
    let tolerance = 1e-9 * expected.abs().max(floor);
    assert!((observed - expected).abs() <= tolerance, "{what}: observed {observed:e}, expected {expected:e} (tolerance {tolerance:e})");
}

fn member_lengths(case: &Value) -> Vec<f64> {
    let m = &case["model"];
    let at = |name: &Value| m["nodes_m"][name.as_str().unwrap()].as_array().unwrap().iter().map(ratio).collect::<Vec<_>>();
    m["members"].as_array().unwrap().iter().map(|member| {
        let (a, b) = (at(&member["from"]), at(&member["to"]));
        (0..3).map(|k| (b[k] - a[k]).powi(2)).sum::<f64>().sqrt()
    }).collect()
}

/// Every expected value of a case, keyed and with its family.
fn expected(case: &Value) -> BTreeMap<String, (f64, Family)> {
    let mut out = BTreeMap::new();
    let family_of = |key: &str| if key.starts_with('F') { Family::Force } else { Family::Moment };
    for (node, by) in case["displacements"].as_object().unwrap() {
        for (dof, _, _) in DOFS {
            let family = if dof.starts_with('U') { Family::Translation } else { Family::Rotation };
            out.insert(format!("displacements/{node}/{dof}"), (value(&by[dof]), family));
        }
    }
    for (support, by) in case["reactions"].as_object().unwrap() {
        for (key, v) in by.as_object().unwrap() {
            out.insert(format!("reactions/{support}/{key}"), (value(v), family_of(key)));
        }
    }
    for (pipe, by_node) in case["members"].as_object().unwrap() {
        for (node, end) in by_node.as_object().unwrap() {
            for (key, v) in end["wall_end_action_global"].as_object().unwrap() {
                out.insert(format!("members/{pipe}/{node}/wall_end_action/{key}"), (value(v), family_of(key)));
            }
            out.insert(format!("members/{pipe}/{node}/N_w"), (value(&end["N_w"]), Family::Force));
            out.insert(format!("members/{pipe}/{node}/S_effective"), (value(&end["S_effective"]), Family::Force));
            for basis in BASES {
                out.insert(format!("members/{pipe}/{node}/axial_wall_stress/{basis}"), (value(&end["axial_wall_stress"]), Family::Stress));
            }
        }
    }
    if let Some(lame) = case.get("lame_rows_unchanged_by_supports") {
        for member in case["model"]["members"].as_array().unwrap().iter().filter(|m| m["kind"] == "straight") {
            for node in [&member["from"], &member["to"]] {
                for (key, _, _) in LAME {
                    for basis in BASES {
                        out.insert(format!("lame/{}/{}/{key}/{basis}", member["id"].as_str().unwrap(), node.as_str().unwrap()), (value(&lame[key]), Family::Stress));
                    }
                }
            }
        }
    }
    for (support, block) in case["nonlinear_supports"].as_object().unwrap() {
        for (key, v) in block.as_object().unwrap() {
            let family = match key.as_str() {
                "state" => continue,
                "u_d" | "separation" => Family::Translation,
                _ => Family::Force,
            };
            out.insert(format!("nonlinear/{support}/{key}"), (value(v), family));
        }
    }
    out
}

fn floors(references: &Value, case: &Value, expected: &BTreeMap<String, (f64, Family)>) -> Floors {
    let largest = |family: Family| expected.values().filter(|(_, f)| *f == family).map(|(v, _)| v.abs()).fold(0.0, f64::max);
    let lengths = member_lengths(case);
    let (shortest, longest) = (lengths.iter().copied().fold(f64::INFINITY, f64::min), lengths.iter().copied().fold(0.0, f64::max));
    let area = value(&references["model"]["section"]["A_n_m2"]);
    let modulus = ratio(&references["model"]["material"]["E_Pa"]);
    let mut f = Floors { translation: largest(Family::Translation), rotation: largest(Family::Rotation), force: largest(Family::Force), moment: largest(Family::Moment), stress: largest(Family::Stress) };
    if f.translation == 0.0 && f.rotation == 0.0 {
        f.translation = f.force * longest / (modulus * area);
    }
    if f.rotation == 0.0 {
        f.rotation = f.translation / shortest;
    }
    if f.translation == 0.0 {
        f.translation = f.rotation * longest;
    }
    if f.force == 0.0 {
        f.force = modulus * area * f.translation / longest;
    }
    if f.moment == 0.0 {
        f.moment = f.force * longest;
    }
    if f.stress == 0.0 {
        f.stress = f.force / area;
    }
    assert!(f.translation > 0.0 && f.rotation > 0.0 && f.force > 0.0 && f.moment > 0.0 && f.stress > 0.0, "{f:?}");
    f
}

fn end_of(case: &Value, pipe: &str, node: &str) -> &'static str {
    let member = case["model"]["members"].as_array().unwrap().iter().find(|m| m["id"] == pipe).unwrap();
    if member["from"] == node {
        "end_i"
    } else {
        assert_eq!(member["to"], node);
        "end_j"
    }
}

/// The product's published state token of a nonlinear support.
fn state_token(envelope: &Value, support: &str) -> String {
    let found = rows(envelope, "nonlinear_support_active_set_state_code", support, None, None, "");
    assert_eq!(found.len(), 1, "{support}: state rows");
    let basis = found[0]["metadata"]["basis"].as_str().unwrap();
    basis.split("final_state=").nth(1).unwrap().split(';').next().unwrap().trim().to_string()
}

fn nonlinear_input<'a>(case: &'a Value, support: &str) -> &'a Value {
    case["model"]["nonlinear_supports"].as_array().unwrap().iter().find(|s| s["id"] == support).unwrap()
}

/// The derived normal's source: (support id, component).
fn normal_source(case: &Value, input: &Value) -> Option<(String, String)> {
    let normal = &input["normal"];
    (normal["kind"] == "derived").then(|| {
        let source = case["model"]["linear_restraints"].as_array().unwrap().iter()
            .find(|r| r["node"] == normal["source_node"] && r["dof"] == normal["source_dof"]).unwrap();
        let dof = normal["source_dof"].as_str().unwrap();
        (source["id"].as_str().unwrap().to_string(), format!("F{}", &dof[1..].to_lowercase()))
    })
}

/// The product's values of a case in the expected keys; support quantities
/// that the packet forms (separation, contact, μN, stick margin) are formed
/// from the product's own published rows by the packet's conventions.
fn observed(case: &Value, envelope: &Value) -> BTreeMap<String, f64> {
    let mut out = BTreeMap::new();
    let m = &case["model"];
    for node in m["nodes_m"].as_object().unwrap().keys() {
        for (dof, kind, factor) in DOFS {
            out.insert(format!("displacements/{node}/{dof}"), row(envelope, kind, &node_id(node), None, None, "") / factor);
        }
    }
    for (support, by) in case["reactions"].as_object().unwrap() {
        for key in by.as_object().unwrap().keys() {
            let component = format!("{}{}", &key[..1], key[1..].to_lowercase());
            out.insert(format!("reactions/{support}/{key}"), row(envelope, "support_reaction_component_v2", support, None, Some(&component), ""));
        }
    }
    let in_region = case["load_case"]["regions"] == "one region over every member";
    for member in m["members"].as_array().unwrap() {
        let pipe = member["id"].as_str().unwrap();
        for node in [member["from"].as_str().unwrap(), member["to"].as_str().unwrap()] {
            let end = end_of(case, pipe, node);
            for (key, component, kind) in ACTIONS {
                out.insert(format!("members/{pipe}/{node}/wall_end_action/{key}"), row(envelope, kind, pipe, Some(end), Some(component), ""));
            }
            // In a region the wall rows; outside every region (p = 0, no
            // regions) the element axial force, -N_w at end_i and +N_w at end_j.
            let wall = maybe_row(envelope, "pipe_wall_axial_force_v2", pipe, Some(end), None, "");
            assert_eq!(wall.is_some(), in_region, "{pipe} {end}: wall rows exactly in a region");
            let (n_w, s) = match wall {
                Some(n_w) => (n_w, row(envelope, "pipe_effective_axial_force_v2", pipe, Some(end), None, "")),
                None => {
                    let local = row(envelope, "element_local_axial_force", pipe, Some(end), None, "");
                    let n_w = if end == "end_i" { -local } else { local };
                    (n_w, n_w)
                }
            };
            out.insert(format!("members/{pipe}/{node}/N_w"), n_w);
            out.insert(format!("members/{pipe}/{node}/S_effective"), s);
            for basis in BASES {
                let stress = match maybe_row(envelope, "pipe_axial_membrane_stress_v2", pipe, Some(end), None, basis) {
                    Some(stress) => stress,
                    None => 1.0e6 * row(envelope, "element_local_axial_normal_stress", pipe, Some(end), None, basis),
                };
                out.insert(format!("members/{pipe}/{node}/axial_wall_stress/{basis}"), stress);
            }
            if member["kind"] == "straight" && in_region {
                for (key, kind, component) in LAME {
                    for basis in BASES {
                        out.insert(format!("lame/{pipe}/{node}/{key}/{basis}"), row(envelope, kind, pipe, Some(end), Some(component), basis));
                    }
                }
            }
        }
    }
    for support in case["nonlinear_supports"].as_object().unwrap().keys() {
        let input = nonlinear_input(case, support);
        let u = row(envelope, "nonlinear_support_final_displacement", support, None, None, "") / 1000.0;
        let r = row(envelope, "nonlinear_support_final_reaction", support, None, None, "");
        out.insert(format!("nonlinear/{support}/u_d"), u);
        out.insert(format!("nonlinear/{support}/reaction"), r);
        match input["kind"].as_str().unwrap() {
            "gap" => {
                let g = ratio(&input["clearance_m"]);
                let positive = input["closes"] == "+";
                out.insert(format!("nonlinear/{support}/separation"), if positive { g - u } else { g + u });
                out.insert(format!("nonlinear/{support}/contact"), if positive { -r } else { r });
            }
            "one_way" => {
                assert_eq!(input["sense"], "+");
                out.insert(format!("nonlinear/{support}/separation"), u);
                out.insert(format!("nonlinear/{support}/contact"), r);
            }
            "friction" => {
                let mu = ratio(&input["mu"]);
                let n = match normal_source(case, input) {
                    Some((source, component)) => {
                        out.insert(format!("nonlinear/{support}/source_reaction"), row(envelope, "support_reaction_component_v2", &source, None, Some(&component), ""));
                        row(envelope, "nonlinear_support_friction_normal_reaction_derived", support, None, None, "")
                    }
                    None => row(envelope, "nonlinear_support_friction_normal_reaction_input", support, None, None, ""),
                };
                out.insert(format!("nonlinear/{support}/N"), n);
                out.insert(format!("nonlinear/{support}/muN"), mu * n);
                out.insert(format!("nonlinear/{support}/stick_margin"), mu * n - r.abs());
            }
            other => panic!("{other}"),
        }
    }
    out
}

/// The product state token, sliding sense and normal sign the reference state
/// names: gap closed/open = active/inactive; friction stick = sticking and
/// slide± = sliding with sign(u_d) = ±; `/b±` = sign of the source reaction.
fn assert_states(case: &Value, envelope: &Value, observed: &BTreeMap<String, f64>, label: &str) {
    for (support, want) in case["support_states"].as_object().unwrap() {
        let want = want.as_str().unwrap();
        let (base, beta) = match want.split_once('/') {
            Some((base, beta)) => (base, Some(beta)),
            None => (want, None),
        };
        let token = state_token(envelope, support);
        let expected_token = match base {
            "closed" | "active" => "active",
            "open" | "inactive" => "inactive",
            "stick" => "sticking",
            "slide+" | "slide-" => "sliding",
            other => panic!("state {other}"),
        };
        assert_eq!(token, expected_token, "{label} {support}: reference {want}");
        let u = observed[&format!("nonlinear/{support}/u_d")];
        match base {
            "slide+" => assert!(u > 0.0, "{label} {support}: slip sense"),
            "slide-" => assert!(u < 0.0, "{label} {support}: slip sense"),
            _ => {}
        }
        if let Some(beta) = beta {
            let source = observed[&format!("nonlinear/{support}/source_reaction")];
            match beta {
                "b+" => assert!(source > 0.0, "{label} {support}: beta"),
                "b-" => assert!(source < 0.0, "{label} {support}: beta"),
                other => panic!("{other}"),
            }
        }
    }
}

/// The DEC-067 laws and strict complementarity on the published rows (kd5
/// §0 and §2.2), independent of the reference values: the binding's own
/// refusal of a forged state, contact or friction force. Empty when lawful.
fn law_violations(references: &Value, case: &Value, envelope: &Value) -> Vec<String> {
    let mut found = Vec::new();
    let o = observed(case, envelope);
    let floors = floors(references, case, &expected(case));
    // kd5 R4: a confirmed (Passed) case publishes an inactive row as exact +0;
    // otherwise the loop's binary64 row stands, within the force floor.
    let confirmed = standing(envelope).0 == PASSED;
    let zero = |x: f64| if confirmed { x == 0.0 && x.is_sign_positive() } else { x.abs() <= 1e-9 * floors.force };
    for support in case["nonlinear_supports"].as_object().unwrap().keys() {
        let input = nonlinear_input(case, support);
        let token = state_token(envelope, support);
        let get = |q: &str| o[&format!("nonlinear/{support}/{q}")];
        let support_row = |component: &str| row(envelope, "support_reaction_component_v2", support, None, Some(component), "");
        let dof = input["dof"].as_str().unwrap();
        let component = format!("F{}", &dof[1..].to_lowercase());
        let r = get("reaction");
        if (support_row(&component) - r).abs() > 1e-9 * r.abs().max(1.0) {
            found.push(format!("{support}: final reaction {r:e} differs from its support action row"));
        }
        let u = get("u_d");
        let tol = |x: f64| 1e-9 * x.abs().max(floors.force);
        match input["kind"].as_str().unwrap() {
            "gap" | "one_way" => {
                let (separation, contact) = (get("separation"), get("contact"));
                if separation < -1e-12 || contact < -tol(contact) {
                    found.push(format!("{support}: separation {separation:e} or contact {contact:e} negative"));
                }
                let engaged = contact > tol(contact);
                let open = separation > 1e-12;
                if engaged == open {
                    found.push(format!("{support}: not strictly complementary (separation {separation:e}, contact {contact:e})"));
                }
                if (token == "active") != engaged {
                    found.push(format!("{support}: state {token} with contact {contact:e}"));
                }
                if token == "inactive" && !zero(r) {
                    found.push(format!("{support}: inactive with reaction {r:e}"));
                }
            }
            "friction" => {
                let mu_n = get("muN");
                match token.as_str() {
                    "inactive" => {
                        if get("N") != 0.0 || !zero(r) {
                            found.push(format!("{support}: inactive with N {:e} and force {r:e}", get("N")));
                        }
                    }
                    "sticking" => {
                        if u.abs() > 1e-12 || !(r.abs() < mu_n) {
                            found.push(format!("{support}: stick with u {u:e}, |R_t| {:e} >= muN {mu_n:e}", r.abs()));
                        }
                    }
                    "sliding" => {
                        if (r.abs() - mu_n).abs() > tol(mu_n) || r * u >= 0.0 {
                            found.push(format!("{support}: slide force {r:e} is not -sign(u) muN {mu_n:e}"));
                        }
                    }
                    other => found.push(format!("{support}: friction state {other}")),
                }
                if r.abs() > mu_n + tol(mu_n) {
                    found.push(format!("{support}: |R_t| {:e} above muN {mu_n:e}", r.abs()));
                }
            }
            other => panic!("{other}"),
        }
    }
    found
}

fn is_gap_case(case: &Value) -> bool {
    case["model"]["nonlinear_supports"].as_array().unwrap().iter().any(|s| s["kind"] == "gap")
}

fn is_derived_inactive(case: &Value) -> bool {
    case["expected_standing"].as_str().unwrap().starts_with("Expected NUMERICAL_INTEGRITY_SENSITIVE")
}

/// The README's expected label, where it states one; the binding's refusal of
/// a wrong standing.
fn standing_violation(case: &Value, envelope: &Value) -> Option<String> {
    let (code, message) = standing(envelope);
    if is_gap_case(case) {
        assert!(case["expected_standing"].as_str().unwrap().starts_with(UNQUALIFIED));
        (code != UNQUALIFIED).then(|| format!("gap-bearing case published {code}"))
    } else if is_derived_inactive(case) {
        (code != SENSITIVE || !message.contains(NORMAL_NOT_ESTABLISHED)).then(|| format!("derived inactive case published {code}: {message}"))
    } else {
        (code == UNQUALIFIED).then(|| format!("non-gap case published R1's gap label {code}"))
    }
}

// ----------------------------------------------------------------- tests

/// Every reference case, both modes: every published value, the states, the
/// laws, the standing and the reader.
#[test]
fn every_reference_case_binds_in_both_modes() {
    let references = references();
    let mut summary = Vec::new();
    for case in cases(&references).iter().filter(|c| !INEXPRESSIBLE.contains(&c["id"].as_str().unwrap())) {
        let id = case["id"].as_str().unwrap();
        let document = document(&references, case);
        let want = expected(case);
        let floors = floors(&references, case, &want);
        let mut standings = Vec::new();
        for mode in MODES {
            let label = format!("{id} {mode:?}");
            let envelope = solved(&document, mode, &label);
            let got = observed(case, &envelope);
            for (key, (expected, family)) in &want {
                let observed = got.get(key).unwrap_or_else(|| panic!("{label}: the product publishes no {key}"));
                close(*observed, *expected, floors.of(*family), &format!("{label} {key}"));
            }
            assert_states(case, &envelope, &got, &label);
            assert_eq!(law_violations(&references, case, &envelope), Vec::<String>::new(), "{label}");
            assert_eq!(standing_violation(case, &envelope), None, "{label}");
            assert_reader_ok(&envelope, &document, &label);
            standings.push(standing(&envelope).0);
            summary.push(format!("{label}: {} values; {}", want.len(), standings.last().unwrap()));
        }
        assert_eq!(standings[0], standings[1], "{id}: the standing differs between modes");
    }
    assert_eq!(summary.len(), 2 * (21 - INEXPRESSIBLE.len()));
    println!("{}", summary.join("\n"));
}

/// G1-P0 has no load of any kind (p = 0, no regions, no nodal load). The
/// product requires at least one explicit primitive load and refuses the
/// document by name, so the case is not expressible as the packet states it;
/// no zero-valued load is invented to make it solvable (reported to the
/// manager). Its values are all exactly zero.
#[test]
fn a_load_free_control_is_refused_by_the_product_not_adapted() {
    let references = references();
    for id in INEXPRESSIBLE {
        let case = case(&references, id);
        assert!(case["load_case"]["nodal_loads"].as_object().unwrap().is_empty() && ratio(&case["load_case"]["p_Pa"]) == 0.0);
        for mode in MODES {
            let envelope = solve(&document(&references, case), mode);
            assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{id} {mode:?}");
            assert_eq!(with_code(&envelope, "LOAD_INPUT_MISSING").len(), 1, "{id} {mode:?}");
            assert!(with_code(&envelope, FAMILY).is_empty(), "{id} {mode:?}");
        }
    }
}

/// The authored inputs are what the packet states (a guard against a silent
/// authoring change): pressure, shear model and the seeds are inputs only.
#[test]
fn documents_are_plain_v3_with_the_packet_inputs() {
    let references = references();
    for case in cases(&references) {
        let document = document(&references, case);
        let model = &document["model"];
        assert_eq!(model["pressure_contract"]["mode"], "exact_pressure_v3");
        assert!(model.get("reference_configurations").is_none());
        assert!(model["load_cases"][0].get("analysis_state").is_none());
        let expected_shear = if case["id"].as_str().unwrap().starts_with("F1-T") { "timoshenko_annulus_energy_v1" } else { "euler_bernoulli" };
        assert_eq!(model["pressure_contract"]["shear_model"], expected_shear);
        for pipe in model["pipe_segments"].as_array().unwrap() {
            assert_eq!(pipe["section"]["mill_tolerance"]["value"], 0.0);
            assert_eq!(pipe["section"]["corrosion_allowance"]["value"], 0.0);
        }
    }
}

/// The pressure reader's refusal of a forged envelope: `(without, with)` the
/// model, each the refusal code or `None` for an admission.
fn reader_codes(envelope: &Value, document: &Value) -> (Option<String>, Option<String>) {
    let code = |model: Option<&Value>| {
        let found = reader_verdict::validate("pressure", envelope, model, None);
        (found["ok"] != true).then(|| found["error"]["code"].as_str().unwrap().to_string())
    };
    (code(None), code(Some(&document["model"])))
}

fn set_rows(envelope: &mut Value, kind: &str, entity: &str, f: &dyn Fn(&mut Value)) {
    let mut hit = 0;
    for r in envelope["results"].as_array_mut().unwrap() {
        if r["kind"] == kind && r["entity_ref"] == entity {
            f(r);
            hit += 1;
        }
    }
    assert!(hit > 0, "{kind} {entity}");
}

/// Scale a support's final reaction and its slot component (`component`) by
/// `factor`; with `magnitude`, its force magnitude row too.
fn scale_support(envelope: &mut Value, support: &str, component: &str, factor: f64, magnitude: bool) {
    for kind in ["nonlinear_support_final_reaction", "support_reaction_component_v2", "support_reaction_force_magnitude_v2"] {
        if kind == "support_reaction_force_magnitude_v2" && !magnitude {
            continue;
        }
        set_rows(envelope, kind, support, &|r| {
            if r["kind"] != "support_reaction_component_v2" || r["metadata"]["component"] == component {
                r["value"] = json!(factor * r["value"].as_f64().unwrap());
            }
        });
    }
}

/// Restate a support's final state on its rows and, with `review`, on its
/// `NONLINEAR_SUPPORT_STATE_REVIEW` diagnostic.
fn restate(envelope: &mut Value, support: &str, from: &str, to: &str, code: f64, review: bool) {
    for kind in ["nonlinear_support_active_set_state_code", "nonlinear_support_final_reaction", "nonlinear_support_final_displacement"] {
        set_rows(envelope, kind, support, &|r| {
            if r["kind"] == "nonlinear_support_active_set_state_code" {
                r["value"] = json!(code);
            }
            let basis = r["metadata"]["basis"].as_str().unwrap().replace(&format!("final_state={from}"), &format!("final_state={to}"));
            r["metadata"]["basis"] = json!(basis);
        });
    }
    if review {
        for d in envelope["diagnostics"].as_array_mut().unwrap() {
            if d["code"] == "NONLINEAR_SUPPORT_STATE_REVIEW" && d["affected_refs"][0] == support {
                d["message"] = json!(d["message"].as_str().unwrap().replace(&format!(" in {from} state"), &format!(" in {to} state")));
            }
        }
    }
}

/// Relabel the case's integrity code `from` as Passed; with `quality`, its
/// `numerical_quality` record and aggregate too, and with `line`,
/// evidence line appended to its message.
fn relabel_passed(envelope: &mut Value, from: &str, quality: bool, line: bool) {
    for d in envelope["diagnostics"].as_array_mut().unwrap() {
        if d["code"] == from {
            d["code"] = json!(PASSED);
            d["severity"] = json!("info");
            if line {
                d["message"] = json!(format!("{} Converged support state: established at 128-bit precision by one formation-checked solve; (forged)", d["message"].as_str().unwrap()));
            }
        }
    }
    if quality {
        let q = &mut envelope["numerical_quality"];
        q["status"] = json!("checks_passed");
        for case in q["cases"].as_array_mut().unwrap() {
            case["solve_quality"] = json!("checks_passed");
            case["structural_status"] = json!("passive_model_basis");
            case["model_matrix_fidelity"] = json!("represented_equations_retained");
            case["accuracy_evidence"] = json!("not_claimed");
        }
    }
}

/// Forged rows: a gap contact, a support state, a friction force above μN and
/// a wrong standing. The binding refuses each, and so does the pressure reader
/// (`result_export::nonlinear_support_evidence`), with and without the model,
/// by the code named. Deeper variants that keep the cheaper evidence
/// consistent reach the reader's later rules; the Python mirror
/// (`tests/test_rt_a3_nonlinear_reader.py`) refuses the same envelopes by the
/// same codes.
#[test]
fn forged_contacts_states_friction_and_standings_are_refused() {
    let references = references();
    let mode = PreviewSolverMode::DenseScrutiny;
    let both = |code: &str| (Some(format!("SOURCE_PHYSICS_NONLINEAR_{code}")), Some(format!("SOURCE_PHYSICS_NONLINEAR_{code}")));
    let model_only = |code: &str| (None, Some(format!("SOURCE_PHYSICS_NONLINEAR_{code}")));

    // 1. A forged contact: G1's gap reaction halved in its loop row and its
    //    support action row (so the two agree).
    let g1 = case(&references, "G1");
    let document_g1 = document(&references, g1);
    let clean = solved(&document_g1, mode, "G1");
    let mut forged = clean.clone();
    scale_support(&mut forged, "support:GAP", "Fx", 0.5, false);
    let want = expected(g1);
    let got = observed(g1, &forged);
    // A halved contact is still lawful, so the reference comparison refuses it.
    let floor = floors(&references, g1, &want).force;
    for key in ["nonlinear/support:GAP/contact", "nonlinear/support:GAP/reaction", "reactions/support:GAP/FX"] {
        let expected = want[key].0;
        assert!((got[key] - expected).abs() > 1e-9 * expected.abs().max(floor), "{key}: the binding refuses the forged contact");
    }
    // The reader: the support's magnitude no longer follows its components.
    assert_eq!(reader_codes(&forged, &document_g1), both("MAGNITUDE"), "G1 contact halved");
    // Limit: halved with its magnitude too, the contact is lawful and
    // self-consistent; only the references (or an equilibrium binding the
    // reader does not have) refuse it.
    let mut forged = clean.clone();
    scale_support(&mut forged, "support:GAP", "Fx", 0.5, true);
    assert_eq!(reader_codes(&forged, &document_g1), (None, None), "G1 contact and magnitude halved: the reader's limit");

    // 2. A forged state: G1's closed gap published inactive with its contact.
    let mut forged = clean.clone();
    restate(&mut forged, "support:GAP", "active", "inactive", 0.0, false);
    assert!(law_violations(&references, g1, &forged).iter().any(|v| v.contains("state inactive")), "the binding refuses the forged state");
    assert_eq!(reader_codes(&forged, &document_g1), both("STATE_REVIEW"), "G1 closed gap stated inactive");
    // With its state diagnostic restated too. A non-Passed inactive reaction
    // is inspection evidence with no published bound (the nl3 review's B1), so
    // the readers admit its contact force; the binding's law check above
    // refuses it. Limit, pinned.
    restate(&mut forged, "support:GAP", "active", "inactive", 0.0, true);
    assert_eq!(reader_codes(&forged, &document_g1), (None, None), "G1 inactive with its contact: the readers' limit");
    // An open gap (G2) stated active: closed only at its clearance (model).
    let g2 = case(&references, "G2");
    let document_g2 = document(&references, g2);
    let mut forged = solved(&document_g2, mode, "G2");
    restate(&mut forged, "support:GAP", "inactive", "active", 1.0, true);
    assert_eq!(reader_codes(&forged, &document_g2), model_only("MODEL_COMPLEMENTARITY"), "G2 open gap stated active");

    // 3. Friction above μN: F1-SLIDE's sliding force set to 1.1·μN.
    let f1 = case(&references, "F1-SLIDE");
    let document_f1 = document(&references, f1);
    let clean_f1 = solved(&document_f1, mode, "F1-SLIDE");
    let mut forged = clean_f1.clone();
    scale_support(&mut forged, "support:FRICTION", "Fz", 1.1, false);
    assert!(law_violations(&references, f1, &forged).iter().any(|v| v.contains("above muN")), "the binding refuses friction above muN");
    assert_eq!(reader_codes(&forged, &document_f1), both("MAGNITUDE"), "F1-SLIDE friction 1.1 muN");
    // With its magnitude too: μ is an input, so the model refuses it.
    let mut forged = clean_f1.clone();
    scale_support(&mut forged, "support:FRICTION", "Fz", 1.1, true);
    assert_eq!(reader_codes(&forged, &document_f1), model_only("MODEL_FRICTION_CAPACITY"), "F1-SLIDE friction and magnitude 1.1 muN");
    // The sliding force turned with the slip (its magnitude unchanged).
    let mut forged = clean_f1.clone();
    scale_support(&mut forged, "support:FRICTION", "Fz", -1.0, false);
    assert_eq!(reader_codes(&forged, &document_f1), both("SLIP"), "F1-SLIDE force along the slip");
    // The derived normal no longer its source's published reaction.
    let mut forged = clean_f1.clone();
    set_rows(&mut forged, "nonlinear_support_friction_normal_reaction_derived", "support:FRICTION", &|r| r["value"] = json!(2.0 * r["value"].as_f64().unwrap()));
    assert_eq!(reader_codes(&forged, &document_f1), both("NORMAL"), "F1-SLIDE derived normal doubled");

    // 4. A wrong standing: G1's unqualified label replaced by Passed.
    let mut forged = clean.clone();
    relabel_passed(&mut forged, UNQUALIFIED, false, false);
    assert!(standing_violation(g1, &forged).is_some(), "the binding refuses the wrong standing");
    assert_eq!(reader_codes(&forged, &document_g1), both("STANDING"), "G1 standing forged Passed");
    // With its quality record too: a gap-bearing case is never Passed (R1).
    let mut forged = clean.clone();
    relabel_passed(&mut forged, UNQUALIFIED, true, true);
    assert_eq!(reader_codes(&forged, &document_g1), both("GAP_STANDING"), "G1 standing and quality forged Passed");

    // 5. A wrong standing: F1-P0-SLIDELOAD's Sensitive replaced by Passed.
    let p0 = case(&references, "F1-P0-SLIDELOAD");
    let document_p0 = document(&references, p0);
    let clean_p0 = solved(&document_p0, mode, "F1-P0-SLIDELOAD");
    let mut forged = clean_p0.clone();
    relabel_passed(&mut forged, SENSITIVE, false, false);
    assert!(standing_violation(p0, &forged).is_some(), "the binding refuses the wrong standing");
    assert_eq!(reader_codes(&forged, &document_p0), both("STANDING"), "F1-P0 standing forged Passed");
    // With its quality record and the evidence line: a derived normal that is
    // not established fails closed, never Passed.
    let mut forged = clean_p0.clone();
    relabel_passed(&mut forged, SENSITIVE, true, true);
    assert_eq!(reader_codes(&forged, &document_p0), both("FRICTION_NORMAL_STANDING"), "F1-P0 standing, quality and line forged Passed");
}

/// The review's inclined-gap document (RT-A3 increment 4 review, B1): G2's
/// section, material and anchored first node, the tip at (1000, 1000/3,
/// 1000/7) m, its +UX gap open by 0.001 m, no pressure region and one tip
/// global-Z load of 1e-6 N.
fn inclined_gap_document(references: &Value) -> Value {
    let mut document = document(references, case(references, "G2"));
    let model = &mut document["model"];
    let tip = model["pipe_segments"][0]["to"].as_str().unwrap().to_string();
    for node in model["nodes"].as_array_mut().unwrap() {
        if node["id"] == tip.as_str() {
            node["position"] = json!({"x": 1000.0, "y": 1000.0 / 3.0, "z": 1000.0 / 7.0});
        }
    }
    for support in model["supports"].as_array_mut().unwrap() {
        if !support["nonlinear"].is_null() {
            support["nonlinear"]["gap"] = quantity(0.001, "m");
        }
    }
    let lc = &mut model["load_cases"][0];
    lc["pressure_regions"] = json!([]);
    lc["primitive_loads"] = json!([{"id": "load:tip", "category": "concentrated_force", "target": {"type": "node", "node": tip},
        "direction": "global_z", "magnitude": quantity(1e-6, "N"), "dimension": "force", "provenance": PROVENANCE}]);
    document
}

fn one_value(envelope: &Value, kind: &str, entity: &str) -> f64 {
    let found = envelope["results"].as_array().unwrap().iter().filter(|r| r["kind"] == kind && r["entity_ref"] == entity).collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{kind} {entity}");
    found[0]["value"].as_f64().unwrap()
}

/// The independent review's three blocking reader findings (RT-A3 increment 4
/// review, B1-B3), each with the untouched producer output it concerned or its
/// mutant, and the negative controls of the repair. The Python mirror refuses
/// the same envelopes by the same codes.
#[test]
fn reader_binds_inactive_residuals_support_nodes_and_standings() {
    let references = references();
    let mode = PreviewSolverMode::DenseScrutiny;
    let both = |code: &str| (Some(format!("SOURCE_PHYSICS_NONLINEAR_{code}")), Some(format!("SOURCE_PHYSICS_NONLINEAR_{code}")));
    let model_only = |code: &str| (None, Some(format!("SOURCE_PHYSICS_NONLINEAR_{code}")));

    // B1. The untouched producer output, both modes: an inactive gap's reaction
    // is its free row's binary64 residual (here far above 1e-9 of the largest
    // support component in dense mode), admitted as inspection evidence beside
    // the producer's own compliant free-row evaluation record. No magnitude
    // relation between the two is required (next block).
    let inclined = inclined_gap_document(&references);
    let mut dense = Value::Null;
    for m in MODES {
        let label = format!("inclined gap {m:?}");
        let envelope = solved(&inclined, m, &label);
        assert_eq!(state_token(&envelope, "support:GAP"), "inactive", "{label}");
        assert_eq!(standing(&envelope).0, UNQUALIFIED, "{label}");
        let reaction = one_value(&envelope, "nonlinear_support_final_reaction", "support:GAP");
        assert_reader_ok(&envelope, &inclined, &label);
        if m == mode {
            let largest = envelope["results"].as_array().unwrap().iter()
                .filter(|r| r["kind"] == "support_reaction_component_v2" && ["Fx", "Fy", "Fz"].contains(&r["metadata"]["component"].as_str().unwrap()))
                .map(|r| r["value"].as_f64().unwrap().abs()).fold(0.0, f64::max);
            assert!(reaction.abs() > 1e-9 * largest, "{label}: the regime the review's B1 concerned ({reaction} against {largest})");
            dense = envelope;
        }
    }
    // The nl3 review's loaded counterexample, untouched sparse output with its
    // model: the inactive reaction (−3.24e-13 N) exceeds the free-row residual
    // record (2.10e-13 N, compliant). Recovery (a row dot product, then the
    // load subtracted) and the residual evaluation (the negative load, then
    // normalized products) are different evaluations, and neither row's
    // denominator is published, so the record bounds nothing. Admitted.
    let fixture: Value = serde_json::from_str(B1_LOADED).unwrap();
    let loaded = json!({"model": fixture["model"].clone(), "materials": []});
    let source = &fixture["source"];
    assert_eq!(source["status"]["mechanics"], "MECHANICS_SOLVED");
    assert_eq!(standing(source).0, UNQUALIFIED, "the loaded fixture's standing");
    assert_eq!(state_token(source, "support:GAP"), "inactive", "the loaded fixture's state");
    let reaction = one_value(source, "nonlinear_support_final_reaction", "support:GAP");
    let record = one_value(source, "nonlinear_support_observed_free_dof_force_residual", "nonlinear_supports");
    assert_eq!(reaction.to_bits(), (-3.2445904686942417e-13_f64).to_bits(), "the fixture's reaction is untouched");
    assert_eq!(record.to_bits(), 2.0991282984831466e-13_f64.to_bits(), "the fixture's record is untouched");
    assert!(reaction.abs() > record, "the regime the nl3 review's B1 concerned ({reaction} against {record})");
    assert_reader_ok(source, &loaded, "the nl3 loaded B1 fixture");
    // Its model solved now, sparse: the producer's current output is admitted too.
    let regenerated = solved(&loaded, PreviewSolverMode::SparseInteractive, "the nl3 loaded B1 model");
    assert_eq!(state_token(&regenerated, "support:GAP"), "inactive", "the nl3 loaded B1 model");
    assert_reader_ok(&regenerated, &loaded, "the nl3 loaded B1 model");

    // Forged-force controls under the revised rule. Limit, pinned: the inactive
    // reaction doubled with its slot and magnitude (self-consistent) is admitted;
    // no published bound relates a non-Passed inactive reaction's magnitude to
    // any record, so only the references or an equilibrium binding refuse it.
    let mut forged = dense.clone();
    scale_support(&mut forged, "support:GAP", "Fx", 2.0, true);
    assert_eq!(reader_codes(&forged, &inclined), (None, None), "inactive force doubled: the readers' limit");
    // A non-numeric inactive reaction (and its slot): the outer number rules
    // refuse it before this module.
    let mut forged = dense.clone();
    set_rows(&mut forged, "nonlinear_support_final_reaction", "support:GAP", &|r| r["value"] = Value::Null);
    set_rows(&mut forged, "support_reaction_component_v2", "support:GAP", &|r| {
        if r["metadata"]["component"] == "Fx" {
            r["value"] = Value::Null;
        }
    });
    let outer = Some("SOURCE_PHYSICS_NUMBER_INVALID".to_string());
    assert_eq!(reader_codes(&forged, &inclined), (outer.clone(), outer), "inactive reaction not a number");
    // The inactive reaction in the wrong family (moment units).
    let mut forged = dense.clone();
    set_rows(&mut forged, "nonlinear_support_final_reaction", "support:GAP", &|r| r["unit"] = json!("N*m"));
    assert_eq!(reader_codes(&forged, &inclined), both("UNIT"), "inactive reaction in moment units");
    // The required record: in the wrong family, negative, or duplicated. The
    // family is the outer validators' before this module: the force record in
    // N·m is a unit contradiction.
    let mut forged = dense.clone();
    set_rows(&mut forged, "nonlinear_support_observed_free_dof_force_residual", "nonlinear_supports", &|r| r["unit"] = json!("N*m"));
    let outer = Some("SOURCE_UNIT_CONTRADICTION".to_string());
    assert_eq!(reader_codes(&forged, &inclined), (outer.clone(), outer), "free-row force record in moment units");
    let mut forged = dense.clone();
    set_rows(&mut forged, "nonlinear_support_observed_free_dof_force_residual", "nonlinear_supports", &|r| r["value"] = json!(-1.0));
    assert_eq!(reader_codes(&forged, &inclined), both("RESIDUAL"), "free-row force record negative");
    let mut forged = dense.clone();
    let mut duplicate = forged["results"].as_array().unwrap().iter()
        .find(|r| r["kind"] == "nonlinear_support_observed_free_dof_force_residual").unwrap().clone();
    duplicate["id"] = json!(format!("{}-duplicate", duplicate["id"].as_str().unwrap()));
    forged["results"].as_array_mut().unwrap().push(duplicate);
    assert_eq!(reader_codes(&forged, &inclined), both("RESIDUAL"), "free-row force record duplicated");
    // The evaluation record: its verdict non-compliant, or the record absent.
    let mut forged = dense.clone();
    set_rows(&mut forged, "nonlinear_support_observed_free_dof_force_residual", "nonlinear_supports", &|r| {
        r["metadata"]["basis"] = json!(r["metadata"]["basis"].as_str().unwrap().replace("observed_compliance=true", "observed_compliance=false"));
    });
    assert_eq!(reader_codes(&forged, &inclined), both("RESIDUAL"), "free-row evaluation not compliant");
    let mut forged = dense.clone();
    forged["results"].as_array_mut().unwrap().retain(|r| r["kind"] != "nonlinear_support_observed_free_dof_force_residual");
    assert_eq!(reader_codes(&forged, &inclined), both("RESIDUAL"), "free-row evaluation absent");
    // Passed keeps exact +0 (O1-LIFT's lifted support).
    let lift = case(&references, "O1-LIFT");
    let document_lift = document(&references, lift);
    let mut forged = solved(&document_lift, mode, "O1-LIFT");
    assert_eq!(standing(&forged).0, PASSED, "O1-LIFT is confirmed");
    set_rows(&mut forged, "nonlinear_support_final_reaction", "support:REST", &|r| r["value"] = json!(-0.0));
    set_rows(&mut forged, "support_reaction_component_v2", "support:REST", &|r| {
        if r["metadata"]["component"] == "Fz" {
            r["value"] = json!(-0.0);
        }
    });
    assert_eq!(reader_codes(&forged, &document_lift), both("PASSED_ZERO"), "Passed inactive published -0");

    // B2. The deciding displacement is the support node's published row.
    let g2 = case(&references, "G2");
    let document_g2 = document(&references, g2);
    let clean_g2 = solved(&document_g2, mode, "G2");
    let clearance = document_g2["model"]["supports"].as_array().unwrap().iter()
        .find(|s| s["id"] == "support:GAP").unwrap()["nonlinear"]["gap"]["value"].as_f64().unwrap() * 1000.0;
    // A false closed gap: stated active with its displacement at the clearance.
    let mut forged = clean_g2.clone();
    restate(&mut forged, "support:GAP", "inactive", "active", 1.0, true);
    set_rows(&mut forged, "nonlinear_support_final_displacement", "support:GAP", &|r| r["value"] = json!(clearance));
    assert_eq!(reader_codes(&forged, &document_g2), both("DISPLACEMENT"), "G2 false closed gap");
    // An invented ground: a relative-displacement row at the clearance.
    let mut forged = clean_g2.clone();
    restate(&mut forged, "support:GAP", "inactive", "active", 1.0, true);
    let mut relative = forged["results"].as_array().unwrap().iter()
        .find(|r| r["kind"] == "nonlinear_support_final_displacement" && r["entity_ref"] == "support:GAP").unwrap().clone();
    relative["id"] = json!(format!("{}-relative", relative["id"].as_str().unwrap()));
    relative["kind"] = json!("nonlinear_support_final_relative_displacement");
    relative["value"] = json!(clearance);
    forged["results"].as_array_mut().unwrap().push(relative);
    assert_eq!(reader_codes(&forged, &document_g2), both("RELATIVE_DISPLACEMENT"), "G2 invented ground");
    // Another node's row (the anchor's +0): only the model names the node.
    let mut forged = clean_g2.clone();
    set_rows(&mut forged, "nonlinear_support_final_displacement", "support:GAP", &|r| r["value"] = json!(0.0));
    assert_eq!(reader_codes(&forged, &document_g2), model_only("MODEL_DISPLACEMENT"), "G2 gap displacement from the anchor's row");
    // A Passed slip times 1000.
    let f1 = case(&references, "F1-SLIDE");
    let document_f1 = document(&references, f1);
    let mut forged = solved(&document_f1, mode, "F1-SLIDE");
    assert_eq!(standing(&forged).0, PASSED, "F1-SLIDE is confirmed");
    set_rows(&mut forged, "nonlinear_support_final_displacement", "support:FRICTION", &|r| r["value"] = json!(1000.0 * r["value"].as_f64().unwrap()));
    assert_eq!(reader_codes(&forged, &document_f1), both("DISPLACEMENT"), "F1-SLIDE slip times 1000");

    // B3. The whole code-to-quality assignment, unknown codes, gap precedence.
    let set_code = |envelope: &mut Value, from: &str, to: &str| {
        let mut hit = 0;
        for d in envelope["diagnostics"].as_array_mut().unwrap() {
            if d["code"] == from {
                d["code"] = json!(to);
                hit += 1;
            }
        }
        assert_eq!(hit, 1, "{from}");
    };
    let set_quality = |envelope: &mut Value, tokens: [&str; 4]| {
        for case in envelope["numerical_quality"]["cases"].as_array_mut().unwrap() {
            for (field, token) in ["structural_status", "solve_quality", "model_matrix_fidelity", "accuracy_evidence"].iter().zip(tokens) {
                case[*field] = json!(token);
            }
        }
    };
    let p0 = case(&references, "F1-P0-SLIDELOAD");
    let document_p0 = document(&references, p0);
    let clean_p0 = solved(&document_p0, mode, "F1-P0-SLIDELOAD");
    let g1 = case(&references, "G1");
    let document_g1 = document(&references, g1);
    let clean_g1 = solved(&document_g1, mode, "G1");
    // The review's mutant: Sensitive's code relabelled, its quality left sensitive.
    let mut forged = clean_p0.clone();
    set_code(&mut forged, SENSITIVE, UNQUALIFIED);
    assert_eq!(reader_codes(&forged, &document_p0), both("STANDING"), "F1-P0 code unqualified, quality sensitive");
    let mut forged = clean_p0.clone();
    set_code(&mut forged, SENSITIVE, "NUMERICAL_INTEGRITY_FAILED");
    assert_eq!(reader_codes(&forged, &document_p0), both("STANDING"), "F1-P0 code failed, quality sensitive");
    // One token off: Sensitive's accuracy evidence stated unresolved.
    let mut forged = clean_p0.clone();
    set_quality(&mut forged, ["passive_model_basis", "sensitive", "represented_equations_retained", "unresolved"]);
    assert_eq!(reader_codes(&forged, &document_p0), both("STANDING"), "F1-P0 accuracy evidence off");
    // The quality record no longer names its diagnostic.
    let mut forged = clean_p0.clone();
    forged["numerical_quality"]["cases"][0]["evidence_refs"] = json!([]);
    assert_eq!(reader_codes(&forged, &document_p0), both("STANDING"), "F1-P0 evidence refs emptied");
    // A code the producer does not publish.
    let mut forged = clean_p0.clone();
    set_code(&mut forged, SENSITIVE, "NUMERICAL_INTEGRITY_SENSITIVE_ENOUGH");
    assert_eq!(reader_codes(&forged, &document_p0), both("STANDING_CODE"), "F1-P0 unknown code");
    // G1's unqualified code relabelled Sensitive, its quality left unresolved.
    let mut forged = clean_g1.clone();
    set_code(&mut forged, UNQUALIFIED, SENSITIVE);
    assert_eq!(reader_codes(&forged, &document_g1), both("STANDING"), "G1 code sensitive, quality unresolved");
    // Consistently relabelled Sensitive: a gap-bearing case stays unqualified (R1).
    set_quality(&mut forged, ["passive_model_basis", "sensitive", "represented_equations_retained", "not_claimed"]);
    assert_eq!(reader_codes(&forged, &document_g1), both("GAP_STANDING"), "G1 consistently relabelled sensitive");
    // Limit (preserved earlier reasons): a consistent non-Passed
    // relabel of a non-gap case states a standing the producer can publish.
    let mut forged = clean_p0.clone();
    set_code(&mut forged, SENSITIVE, "NUMERICAL_INTEGRITY_UNRESOLVED");
    set_quality(&mut forged, ["numerically_unresolved", "unresolved", "assembly_uncertainty", "unresolved"]);
    assert_eq!(reader_codes(&forged, &document_p0), (None, None), "F1-P0 consistently relabelled unresolved: the reader's limit");
}

/// The admission seam after S6: a nonlinear support beside an
/// objective connector is admitted, so the family refusal no longer names it,
/// while a malformed connector law is still refused by the connector's own
/// validation; nothing is solved.
#[test]
fn s6_admits_the_support_and_still_refuses_a_malformed_connector() {
    let references = references();
    let g1 = case(&references, "G1");
    let mut connector = document(&references, g1);
    connector["model"]["components"] = json!([{"id": "component:connector", "kind": "expansion_joint", "node": "node:N1",
        "objective_connector": {"version": "1.0.0"}, "provenance": PROVENANCE}]);
    for mode in MODES {
        let envelope = solve(&connector, mode);
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        assert!(envelope["results"].as_array().unwrap().is_empty(), "{mode:?}");
        assert!(with_code(&envelope, FAMILY).into_iter().all(|d| d["affected_refs"] != json!(["support:GAP"])), "{mode:?}: {:#?}", envelope["diagnostics"]);
        let connector_refusals = envelope["diagnostics"].as_array().unwrap().iter()
            .filter(|d| d["severity"] == "blocking" && d["affected_refs"].as_array().unwrap().iter().any(|r| r == "component:connector"))
            .count();
        assert!(connector_refusals > 0, "{mode:?}: {:#?}", envelope["diagnostics"]);
    }
}

/// Every flat mechanics combination of these nonlinear models (the singleton
/// included) is withheld on EX with `NONLINEAR_COMBINATION_REQUIRES_SOLVE`, in
/// both solver modes, with the gate's text and no combined row (#98's gate,
/// `exact_combinations::gate_reason`); both readers
/// admit each envelope with and without its model.
#[test]
fn flat_mechanics_combinations_of_nonlinear_models_are_withheld() {
    for mode in MODES {
        for (document, envelope, ids) in flat_combination_envelopes(mode) {
            for id in &ids {
                let record = envelope["contract_evidence"]["exact_combinations"].as_array().unwrap().iter().find(|r| r["combination_id"] == *id).unwrap();
                assert_eq!(record["gate"], json!({"withheld": true, "reason": GATE_CODE}), "{mode:?} {id}");
                assert!(combination_rows(&envelope, id).is_empty(), "{mode:?} {id}: a withheld combination publishes no row");
            }
            let gated = with_code(&envelope, GATE_CODE);
            assert!(!gated.is_empty(), "{mode:?}: {:#?}", envelope["diagnostics"]);
            for diagnostic in gated {
                assert!(diagnostic["message"].as_str().unwrap().contains(GATE_TEXT), "{mode:?}: {diagnostic}");
            }
            assert_reader_ok(&envelope, &document, &format!("{mode:?} {ids:?}"));
        }
    }
}

/// The README's withheld examples as two-case documents of one model each.
fn flat_combination_envelopes(mode: PreviewSolverMode) -> Vec<(Value, Value, Vec<String>)> {
    let references = references();
    let mut out = Vec::new();
    for (first, second, terms) in [
        ("G1", "G1-P0", vec![("combination:g1", vec![(0, 1.0)]), ("combination:g-half", vec![(0, 0.5), (1, 0.5)])]),
        ("F1-SLIDE", "F1-P0-SLIDELOAD", vec![("combination:f-sum", vec![(0, 1.0), (1, 1.0)])]),
        ("O1-LIFT", "O1-CONTACT", vec![("combination:o-unit", vec![(0, 0.3), (1, 0.7)])]),
    ] {
        let (mut document, names) = two_case_document(&references, first, second);
        document["model"]["combinations"] = Value::Array(terms.iter().map(|(id, t)| json!({"id": id, "basis": "mechanics",
            "terms": t.iter().map(|(i, f)| json!({"load_case": names[*i], "factor": f})).collect::<Vec<_>>(), "provenance": PROVENANCE})).collect());
        let envelope = solve(&document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{first}: {:#?}", envelope["diagnostics"]);
        out.push((document, envelope, terms.iter().map(|(id, _)| id.to_string()).collect()));
    }
    out
}

/// The model of `first` with two load cases, `first`'s and `second`'s, renamed
/// `case:{first}` and `case:{second}` (their loads' ids qualified by the case).
fn two_case_document(references: &Value, first: &str, second: &str) -> (Value, [String; 2]) {
    let mut document = document(references, case(references, first));
    let other = document_for_case(references, second);
    let mut a = document["model"]["load_cases"][0].clone();
    let mut b = other["model"]["load_cases"][0].clone();
    a["id"] = json!(format!("case:{first}"));
    b["id"] = json!(format!("case:{second}"));
    for (c, name) in [(&mut a, first), (&mut b, second)] {
        for load in c["primitive_loads"].as_array_mut().unwrap() {
            load["id"] = json!(format!("{}:{name}", load["id"].as_str().unwrap()));
        }
    }
    document["model"]["load_cases"] = json!([a, b]);
    (document, [format!("case:{first}"), format!("case:{second}")])
}

fn combination_rows<'a>(envelope: &'a Value, id: &str) -> Vec<&'a Value> {
    envelope["results"].as_array().unwrap().iter()
        .filter(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == id && r["kind"] != "combination_modulus_basis_record")
        .collect()
}

fn exact_gate(envelope: &Value, id: &str) -> Value {
    envelope["contract_evidence"]["exact_combinations"].as_array().unwrap().iter()
        .find(|r| r["combination_id"] == id).unwrap_or_else(|| panic!("no record {id}"))["gate"].clone()
}

/// Decision OQ-1 on EX (#98's
/// `combination_plan::Plan::solved_state_plus_increments`, unchanged): in a model
/// with nonlinear supports a mechanics combination is admitted exactly when it is
/// one solved load case at factor exactly 1 plus at least one combination whose
/// fixed-load count κ is exactly 0. Nothing broader superposes: a factor other
/// than 1, two load cases, a κ = 1 or undefined operand, the singleton and the
/// flat sum are withheld with `NONLINEAR_COMBINATION_REQUIRES_SOLVE`. Subtractions,
/// SRSS, absolute sums and range envelopes are not mechanics combinations and are
/// not gated by it. The admitted rows state OQ-1's sign convention; both readers
/// admit the envelope with and without its model and refuse each forged gate.
#[test]
fn oq1_admits_one_solved_case_plus_kappa_zero_combinations_on_ex() {
    let references = references();
    let (mut document, [s, o]) = two_case_document(&references, "O1-LIFT", "O1-CONTACT");
    let comb = |id: &str, basis: &str, terms: &[(&str, &str, f64)]| json!({"id": id, "basis": basis, "provenance": PROVENANCE,
        "terms": terms.iter().map(|(kind, operand, factor)| if *kind == "case" { json!({"load_case": operand, "factor": factor}) }
            else { json!({"combination": operand, "factor": factor}) }).collect::<Vec<_>>()});
    document["model"]["combinations"] = json!([
        {"id": "D", "basis": "result_state_subtraction", "minuend_id": o, "subtrahend_id": s, "provenance": PROVENANCE},
        comb("E", "srss", &[("comb", "D", 1.0)]),
        comb("occ", "mechanics", &[("case", &s, 1.0), ("comb", "E", 1.0)]),
        comb("occ-minus", "mechanics", &[("comb", "E", -1.0), ("case", &s, 1.0), ("comb", "D", 0.5)]),
        comb("factor", "mechanics", &[("case", &s, 1.5), ("comb", "E", 1.0)]),
        comb("two-cases", "mechanics", &[("case", &s, 1.0), ("case", &o, 1.0), ("comb", "E", 1.0)]),
        {"id": "R", "basis": "range_envelope", "operand_ids": [s, o], "mode": "max_abs", "provenance": PROVENANCE},
        comb("kappa-one", "mechanics", &[("case", &s, 1.0), ("comb", "R", 1.0)]),
        comb("srss-case", "srss", &[("case", &o, 1.0)]),
        comb("undefined", "mechanics", &[("case", &s, 1.0), ("comb", "srss-case", 1.0)]),
        comb("singleton", "mechanics", &[("case", &s, 1.0)]),
        comb("flat", "mechanics", &[("case", &s, 1.0), ("case", &o, 1.0)]),
        comb("abs", "absolute_sum", &[("comb", "D", 1.0), ("case", &o, 1.0)]),
    ]);
    let admitted = json!({"withheld": false, "reason": null});
    let withheld = json!({"withheld": true, "reason": GATE_CODE});
    for mode in MODES {
        let envelope = solve(&document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:#?}", envelope["diagnostics"]);
        for id in ["D", "E", "occ", "occ-minus", "R", "srss-case", "abs"] {
            assert_eq!(exact_gate(&envelope, id), admitted, "{mode:?} {id}");
        }
        for id in ["factor", "two-cases", "kappa-one", "undefined", "singleton", "flat"] {
            assert_eq!(exact_gate(&envelope, id), withheld, "{mode:?} {id}");
            assert!(combination_rows(&envelope, id).is_empty(), "{mode:?} {id}");
        }
        for id in ["occ", "occ-minus"] {
            let rows = combination_rows(&envelope, id);
            assert!(!rows.is_empty(), "{mode:?} {id}");
            for row in rows {
                assert_eq!(row["metadata"]["sign_convention"], OQ1_SIGN, "{mode:?} {id}: {row}");
            }
        }
        assert_reader_ok(&envelope, &document, &format!("OQ-1 {mode:?}"));
        // Forged gates: an admitted OQ-1 combination stated withheld, a withheld one
        // stated admitted, and the nonlinear reason swapped for gate 3's and gate 4's.
        for (id, reason) in [("occ", Some(GATE_CODE)), ("singleton", None), ("flat", None), ("factor", None),
            ("flat", Some("COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE")), ("flat", Some("CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE"))] {
            let mut forged = envelope.clone();
            let record = forged["contract_evidence"]["exact_combinations"].as_array_mut().unwrap().iter_mut().find(|r| r["combination_id"] == id).unwrap();
            record["gate"] = json!({"withheld": reason.is_some(), "reason": reason});
            let refused = Some(COMBINATION_GATE.to_string());
            assert_eq!(reader_codes(&forged, &document), (refused.clone(), refused), "{mode:?} {id} forged {reason:?}");
        }
    }
}
fn document_for_case(references: &Value, id: &str) -> Value {
    document(references, case(references, id))
}


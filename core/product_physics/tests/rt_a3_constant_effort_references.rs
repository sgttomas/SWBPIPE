//! RT-A3 increment 3 (DEL-04-07 Design `route-unification-r4.md` §3.2 S2 and
//! §6.3 D-2; `pressure-contract.md`, "Constant-effort supports (RT-A3
//! increment 3)" and "Constant-effort combinations: the unit factor-sum rule";
//! DEC-049 and `validation/hand_calcs/mechanics/constant_effort_support_applied_load.md`):
//! ideal constant-effort supports on the exact pressure route against the
//! frozen independent references (`validation/references/rt_a3_constant_effort/`),
//! through PP's public entry in both solver modes.
//!
//! Each case is authored as a current `3.0.0/exact_pressure_v3` document from
//! the case's own inputs (nodes, members, anchors, the constant-effort record,
//! nodal loads, regions, shear model). The CE-3 cases are load/reference-state
//! documents: one case per state, each support an `active_model_device` (D-2),
//! the states' temperatures absolute (the installation temperature 293.15 K in
//! S, 393.15 K in O) under a constant secant law whose datum is the
//! installation temperature, so the thermal eigenstrain is α·100 K (README
//! binding note). The record carries a `travel_range` the product requires as
//! user metadata; it enters no number (only the user-limit comparison, which
//! it does not trigger here).
//!
//! Every published value is compared at |observed − expected| ≤
//! 1e-9·max(|expected|, zero scale), each family's zero scale the largest
//! magnitude of that family in the case's block (rotation and moment derived
//! from translation and force when a family is wholly zero); combinations and
//! O − S use the larger scale of their operand blocks. The bending row is
//! `hypot(sigma_y, sigma_z)` of the product's signed local rows at the same
//! end and basis; component signs are bound through the signed global end
//! actions. A mismatch here is the product's to fix, never the reference's.
//!
//! Every envelope also passes the pressure reader with and without its model
//! (`reader_verdict::validate`), whose combination pass recomputes each gate,
//! so a withheld combination's reason is recomputed by the reader; forged
//! reasons, factor sums and dropped constant-effort terms are refused.

use open_pipe_stress_product_physics::result_export::reader_verdict;
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode, LinearStaticPreviewRequest,
    PreviewSolverMode, MODEL_SCHEMA_VERSION,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const REFERENCES: &str = include_str!("../../../validation/references/rt_a3_constant_effort/references.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const PROVENANCE: &str = "invented_rt_a3_constant_effort_reference";
const TRAVEL_RANGE_M: f64 = 0.2;
const MATERIAL: &str = "material:rt-a3-ce";
const BASES: [&str; 2] = ["nominal_section", "reduced_section"];
const APPLIED: &str = "constant_effort_support_applied_load";
const NOT_CONSUMED: &str = "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED";
const GATE_CODE: &str = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE";
/// PF's message (`preview_physics.rs` `gate_message`), on EX.
const GATE_TEXT: &str = "Combination withheld: a constant-effort support would be counted by the sum of factors instead of once.";
const STATE_CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
const REFERENCE_CONFIGURATION: &str = "reference:installed";
const LAW: &str = "law:ce3";
/// The installation (reference) temperature: the README's CE3 binding note.
const INSTALLED_K: f64 = 293.15;
const LAME: [(&str, &str, &str); 4] = [
    ("radial_inner", "pipe_lame_radial_stress_v2", "lame_inner_radial_stress"),
    ("radial_outer", "pipe_lame_radial_stress_v2", "lame_outer_radial_stress"),
    ("hoop_inner", "pipe_lame_hoop_stress_v2", "lame_inner_hoop_stress"),
    ("hoop_outer", "pipe_lame_hoop_stress_v2", "lame_outer_hoop_stress"),
];

// ----------------------------------------------------------------- inputs

fn references() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}

fn case<'a>(references: &'a Value, label: &str) -> &'a Value {
    references["cases"].as_array().unwrap().iter().find(|c| c["label"] == label).unwrap_or_else(|| panic!("no case {label}"))
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

fn members(case: &Value) -> &Vec<Value> {
    case["inputs"]["members"].as_array().unwrap()
}

fn member<'a>(case: &'a Value, pipe: &str) -> &'a Value {
    members(case).iter().find(|m| m["id"] == pipe).unwrap_or_else(|| panic!("no member {pipe}"))
}

/// The product end (`end_i` at `from`, `end_j` at `to`) of a member at a node.
fn end_of(case: &Value, pipe: &str, node: &str) -> &'static str {
    let m = member(case, pipe);
    if m["from"] == node {
        "end_i"
    } else {
        assert_eq!(m["to"], node, "{pipe} does not end at {node}");
        "end_j"
    }
}

fn primitive_loads(case: &str, loads: &Value) -> Vec<Value> {
    let mut out = Vec::new();
    for (node, components) in loads.as_object().into_iter().flatten() {
        for (component, value) in components.as_object().unwrap() {
            let direction = match component.as_str() {
                "FX" => "global_x",
                "FY" => "global_y",
                "FZ" => "global_z",
                other => panic!("load component {other}"),
            };
            out.push(json!({"id": format!("load:{case}:{node}:{component}"), "category": "concentrated_force",
                "target": {"type": "node", "node": node_id(node)}, "direction": direction,
                "magnitude": quantity(ratio(value), "N"), "dimension": "force", "provenance": PROVENANCE}));
        }
    }
    out
}

/// Each stated region: one member, both terminals `transfers_to_wall`.
fn regions(case: &Value, stated: &Value) -> Vec<Value> {
    stated.as_array().into_iter().flatten().map(|region| {
        let ids = region["members"].as_array().unwrap();
        assert_eq!(ids.len(), 1);
        let m = member(case, ids[0].as_str().unwrap());
        assert_eq!(region["terminals"], "both transfers_to_wall");
        let terminal = |end: &str| json!({"node_ref": node_id(m[end].as_str().unwrap()), "closure_transfer": "transfers_to_wall", "provenance": PROVENANCE});
        json!({"id": region["id"], "member_pipe_ids": ids, "pressure_basis": "internal_differential_zero_external_v1",
            "pressure": quantity(ratio(&region["pressure_Pa"]), "Pa"), "terminals": [terminal("from"), terminal("to")], "provenance": PROVENANCE})
    }).collect()
}

/// The pipes in a region of one case or state's stated inputs.
fn pressurized(stated: &Value) -> Vec<String> {
    stated["pressure_regions"].as_array().into_iter().flatten()
        .flat_map(|r| r["members"].as_array().unwrap().iter().map(|m| m.as_str().unwrap().to_string()))
        .collect()
}

fn constant_effort_records(case: &Value) -> &Vec<Value> {
    case["inputs"]["constant_effort_records"].as_array().unwrap()
}

/// The plain document of a CE-1, CE-1n or CE-2 case; `with_record` false
/// removes the constant-effort record (the references' "without the record").
fn plain_document(references: &Value, case: &Value, with_record: bool) -> Value {
    let model = &references["model"];
    let inputs = &case["inputs"];
    let section = &model["section"];
    let length = |key: &str| quantity(ratio(&section[key]), "m");
    let nodes = inputs["nodes_m"].as_object().unwrap().iter().map(|(name, xyz)| {
        let xyz = xyz.as_array().unwrap();
        json!({"id": node_id(name), "position": {"x": ratio(&xyz[0]), "y": ratio(&xyz[1]), "z": ratio(&xyz[2])}, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let pipes = members(case).iter().map(|m| {
        json!({"id": m["id"], "from": node_id(m["from"].as_str().unwrap()), "to": node_id(m["to"].as_str().unwrap()),
            "section": {"outside_diameter": length("D_m"), "wall_thickness": length("t_m"), "mill_tolerance": length("c_m_m"), "corrosion_allowance": length("c_a_m")},
            "material": MATERIAL, "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let mut supports = inputs["anchors_all_six_dof"].as_array().unwrap().iter().map(|name| {
        let name = name.as_str().unwrap();
        json!({"id": format!("support:{name}"), "node": node_id(name), "family": "anchor",
            "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    if with_record {
        for record in constant_effort_records(case) {
            supports.push(json!({"id": record["id"], "node": node_id(record["node"].as_str().unwrap()), "family": "constant_effort_support",
                "restraints": record["declared_restraint_dofs"],
                "hanger": {"hanger_type": "constant_effort_support", "constant_load": quantity(ratio(&record["hanger.constant_load_N"]), "N"),
                    "travel_range": quantity(TRAVEL_RANGE_M, "m"), "source_reference": "invented RT-A3 increment 3 reference"},
                "provenance": PROVENANCE}));
        }
    }
    let cases = if let Some(cases) = inputs["load_cases"].as_object() {
        cases.iter().map(|(name, c)| json!({"id": format!("case:{name}"), "primitive_loads": primitive_loads(name, &c["nodal_loads_N"]),
            "pressure_regions": regions(case, &c["pressure_regions"]), "provenance": PROVENANCE})).collect::<Vec<_>>()
    } else {
        vec![json!({"id": "case:main", "primitive_loads": primitive_loads("main", &inputs["nodal_loads_N"]),
            "pressure_regions": regions(case, &inputs["pressure_regions"]), "provenance": PROVENANCE})]
    };
    assert_eq!(model["material"]["nu"], "3/10");
    json!({"model": {"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": inputs["shear_model"]},
        "project": {"id": "project:rt-a3-constant-effort", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes,
        "materials": [{"id": MATERIAL, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": quantity(ratio(&model["material"]["E_Pa"]), "Pa"), "poisson_ratio": quantity(0.3, "1"), "provenance": PROVENANCE}],
        "supports": supports, "components": [], "load_cases": cases, "combinations": []},
        "materials": []})
}

/// The load/reference-state document of a CE-3 case (module docs).
fn state_document(references: &Value, case: &Value, with_record: bool) -> Value {
    let mut document = plain_document(references, case, with_record);
    let inputs = &case["inputs"];
    let model = &mut document["model"];
    let alpha = ratio(&references["model"]["material"]["alpha_per_K"]);
    if let Some(stated) = inputs.get("material_alpha_per_K") {
        assert_eq!(ratio(stated), alpha);
    }
    model["materials"][0]["expansion_laws"] = json!([{"id": LAW, "definition": "engineering_secant",
        "datum_temperature": quantity(INSTALLED_K, "K"), "data": {"kind": "constant", "coefficient": quantity(alpha, "1/K")}, "provenance": PROVENANCE}]);
    let pipes = members(case).iter().map(|m| m["id"].as_str().unwrap().to_string()).collect::<Vec<_>>();
    model["reference_configurations"] = json!([{"id": REFERENCE_CONFIGURATION, "label": "Invented installed reference",
        "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": pipes.iter().map(|pipe| json!({"pipe_ref": pipe,
            "basis": {"kind": "temperature_reference", "installation_temperature": quantity(INSTALLED_K, "K")},
            "fit": {"kind": "none"}, "provenance": PROVENANCE})).collect::<Vec<_>>(),
        "provenance": PROVENANCE}]);
    let supports = model["supports"].as_array().unwrap().iter().map(|s| s["id"].as_str().unwrap().to_string()).collect::<Vec<_>>();
    model["load_cases"] = Value::Array(inputs["states"].as_object().unwrap().iter().map(|(name, state)| {
        assert!(state.get("MUTATION_constant_force_multiplier").is_none(), "a mutation control is never authored");
        let loads = primitive_loads(name, &state["nodal_loads_N"]);
        let above = state.get("temperature_above_reference_K").map_or(0.0, ratio);
        let operating = INSTALLED_K + above;
        assert_eq!(operating - INSTALLED_K, above);
        json!({"id": format!("case:{name}"), "primitive_loads": loads.clone(),
            "pressure_regions": regions(case, &state["pressure_regions"]),
            "analysis_state": {"contract": STATE_CONTRACT, "reference_configuration_ref": REFERENCE_CONFIGURATION,
                "element_states": pipes.iter().map(|pipe| json!({"pipe_ref": pipe, "operating_temperature": quantity(operating, "K"),
                    "material_selection": {"kind": "explicit_base_properties", "material_ref": MATERIAL,
                        "applicability_reference": "invented analytical basis declared applicable to this member"},
                    "thermal_state": {"kind": "free_length_state", "expansion_law_ref": LAW}})).collect::<Vec<_>>(),
                // D-2: the constant-effort record is a device of the active support in every state.
                "support_states": supports.iter().map(|s| json!({"support_ref": s, "participation": {"kind": "active_model_device"}})).collect::<Vec<_>>(),
                "load_sources": loads.iter().map(|l| json!({"source_ref": l["id"], "factor": 1.0})).collect::<Vec<_>>(),
                "history": {"kind": "independent_equilibrium"}, "provenance": PROVENANCE},
            "provenance": PROVENANCE})
    }).collect());
    document
}

fn solve(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).expect("the entry accepts the document")).unwrap()
}

fn assert_reader_ok(envelope: &Value, document: &Value, label: &str) {
    let found = reader_verdict::validate("pressure", envelope, Some(&document["model"]), None);
    assert_eq!(found["ok"], true, "{label}: {found:#?}");
    // Without the model the received document alone passes too.
    let alone = reader_verdict::validate("pressure", envelope, None, None);
    assert_eq!(alone["ok"], true, "{label} (no model): {alone:#?}");
}

/// The reader's refusal of a forgery, as text.
fn refusal(envelope: &Value, document: Option<&Value>) -> String {
    let found = reader_verdict::validate("pressure", envelope, document.map(|d| &d["model"]), None);
    assert_eq!(found["ok"], false, "a forgery passed the reader");
    found["error"].to_string()
}

fn solved(document: &Value, mode: PreviewSolverMode, label: &str) -> Value {
    let envelope = solve(document, mode);
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {:#?}", envelope["diagnostics"]);
    let blocking = diagnostics(&envelope).iter().filter(|d| d["severity"] == "blocking").count();
    assert_eq!(blocking, 0, "{label}");
    assert!(with_code(&envelope, "EXACT_PRESSURE_FAMILY_NOT_ADMITTED").is_empty(), "{label}");
    assert_reader_ok(&envelope, document, label);
    envelope
}

fn diagnostics(envelope: &Value) -> &Vec<Value> {
    envelope["diagnostics"].as_array().unwrap()
}

fn with_code<'a>(envelope: &'a Value, code: &str) -> Vec<&'a Value> {
    diagnostics(envelope).iter().filter(|d| d["code"] == code).collect()
}

// ----------------------------------------------------------------- rows

fn basis_of(row: &Value) -> &str {
    row["metadata"]["basis"].as_str().unwrap_or("").split("; section_basis=").nth(1).unwrap_or("")
}

/// A row's basis reference: a load case (or state) or a combination.
#[derive(Clone, Copy)]
struct At<'a> {
    ref_type: &'a str,
    ref_id: &'a str,
}

fn case_at(id: &str) -> At<'_> {
    At { ref_type: "load_case", ref_id: id }
}

/// The one published row matching.
fn row(envelope: &Value, at: At<'_>, kind: &str, entity: &str, location: Option<&str>, component: Option<&str>, basis: &str) -> f64 {
    let found = envelope["results"].as_array().unwrap().iter().filter(|r| {
        r["kind"] == kind && r["entity_ref"] == entity && basis_of(r) == basis
            && location.is_none_or(|l| r["metadata"]["location"] == l)
            && component.is_none_or(|c| r["metadata"]["component"] == c)
            && r["basis_ref"]["ref_type"] == at.ref_type && r["basis_ref"]["ref_id"] == at.ref_id
    }).collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{kind} {entity} {location:?} {component:?} [{basis}] @{}: {} rows", at.ref_id, found.len());
    found[0]["value"].as_f64().unwrap()
}

fn close(observed: f64, expected: f64, scale: f64, what: &str) {
    let tolerance = 1e-9 * expected.abs().max(scale);
    assert!((observed - expected).abs() <= tolerance, "{what}: observed {observed:e}, expected {expected:e} (tolerance {tolerance:e})");
}

/// The zero scales of one block's families (module docs).
#[derive(Clone, Copy, Debug)]
struct Scales {
    translation: f64,
    rotation: f64,
    force: f64,
    moment: f64,
    stress: f64,
}

impl Scales {
    fn max(self, other: Scales) -> Scales {
        Scales {
            translation: self.translation.max(other.translation),
            rotation: self.rotation.max(other.rotation),
            force: self.force.max(other.force),
            moment: self.moment.max(other.moment),
            stress: self.stress.max(other.stress),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Family {
    Translation,
    Rotation,
    Force,
    Moment,
    Stress,
}

fn scale_of(s: Scales, family: Family) -> f64 {
    match family {
        Family::Translation => s.translation,
        Family::Rotation => s.rotation,
        Family::Force => s.force,
        Family::Moment => s.moment,
        Family::Stress => s.stress,
    }
}

fn largest(values: impl Iterator<Item = f64>) -> f64 {
    values.map(f64::abs).fold(0.0, f64::max)
}

/// The values of a `{name: {DOF: quantity}}` block whose DOF starts with one of `prefixes`.
fn components(block: &Value, prefix: char) -> Vec<f64> {
    block.as_object().into_iter().flatten()
        .flat_map(|(_, by_dof)| by_dof.as_object().unwrap().iter().filter(|(k, _)| k.starts_with(prefix)).map(|(_, v)| value(v)).collect::<Vec<_>>())
        .collect()
}

fn stress_values(block: &Value) -> Vec<f64> {
    let mut out = Vec::new();
    for (_, by_basis) in block["section_stress_rows"].as_object().into_iter().flatten() {
        for basis in BASES {
            let b = &by_basis[basis];
            for (_, end) in b["ends"].as_object().into_iter().flatten() {
                out.extend(end.as_object().unwrap().values().map(value));
            }
            if b.get("elastic_normal_maximum").is_some() {
                out.push(value(&b["elastic_normal_maximum"]));
            }
        }
    }
    out
}

fn scales(case: &Value, block: &Value) -> Scales {
    let lengths = members(case).iter().map(|m| ratio(&m["length_m"])).collect::<Vec<_>>();
    let (shortest, longest) = (lengths.iter().copied().fold(f64::INFINITY, f64::min), lengths.iter().copied().fold(0.0, f64::max));
    let mut ends: Vec<(String, f64)> = Vec::new();
    for (_, by_node) in block["member_end_actions_global"].as_object().into_iter().flatten() {
        for (_, by) in by_node.as_object().unwrap() {
            ends.extend(by.as_object().unwrap().iter().map(|(k, v)| (k.clone(), value(v))));
        }
    }
    let mut translation = largest(components(&block["displacements"], 'U').into_iter());
    let mut rotation = largest(components(&block["displacements"], 'R').into_iter());
    let force = largest(components(&block["reactions"], 'F').into_iter()
        .chain(ends.iter().filter(|(k, _)| k.starts_with('F')).map(|(_, v)| *v))
        .chain(block["axial_wall_force_N_w"].as_object().into_iter().flatten().map(|(_, v)| value(v))));
    let mut moment = largest(components(&block["reactions"], 'M').into_iter().chain(ends.iter().filter(|(k, _)| k.starts_with('M')).map(|(_, v)| *v)));
    if rotation == 0.0 {
        rotation = translation / shortest;
    }
    if translation == 0.0 {
        translation = rotation * longest;
    }
    if moment == 0.0 {
        moment = force * longest;
    }
    let stress = largest(stress_values(block).into_iter());
    let scales = Scales { translation, rotation, force, moment, stress };
    assert!(translation > 0.0 && rotation > 0.0 && force > 0.0 && moment > 0.0, "{scales:?}");
    scales
}

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

/// The product's linear values of one case/state (or combination) for every
/// node, anchor and member of `case`, in the reference's units (m, rad, N,
/// N·m, Pa) and keys: path -> (value, family).
fn linear_values(case: &Value, envelope: &Value, at: At<'_>, regions_of_case: &[String]) -> BTreeMap<String, (f64, Family)> {
    let mut out = BTreeMap::new();
    for name in case["inputs"]["nodes_m"].as_object().unwrap().keys() {
        let node = node_id(name);
        for (dof, kind, factor) in DOFS {
            let family = if dof.starts_with('U') { Family::Translation } else { Family::Rotation };
            out.insert(format!("displacements/{name}/{dof}"), (row(envelope, at, kind, &node, None, None, "") / factor, family));
        }
    }
    for name in case["inputs"]["anchors_all_six_dof"].as_array().unwrap() {
        let name = name.as_str().unwrap();
        for (key, component, _) in ACTIONS {
            let family = if key.starts_with('F') { Family::Force } else { Family::Moment };
            out.insert(format!("reactions/{name}/{key}"), (row(envelope, at, "support_reaction_component_v2", &format!("support:{name}"), None, Some(component), ""), family));
        }
    }
    for m in members(case) {
        let pipe = m["id"].as_str().unwrap();
        for node in [m["from"].as_str().unwrap(), m["to"].as_str().unwrap()] {
            let end = end_of(case, pipe, node);
            for (key, component, kind) in ACTIONS {
                let family = if key.starts_with('F') { Family::Force } else { Family::Moment };
                out.insert(format!("member_end_actions_global/{pipe}/{node}/{key}"), (row(envelope, at, kind, pipe, Some(end), Some(component), ""), family));
            }
        }
        // N_w: the wall-force rows in a region (and on a combination, where an
        // operand outside every region is recovered); otherwise the element
        // axial force, which is -N_w at end_i and +N_w at end_j.
        let wall_rows = regions_of_case.iter().any(|p| p == pipe) || at.ref_type == "combination";
        let (wall_i, wall_j) = if wall_rows {
            (row(envelope, at, "pipe_wall_axial_force_v2", pipe, Some("end_i"), None, ""), row(envelope, at, "pipe_wall_axial_force_v2", pipe, Some("end_j"), None, ""))
        } else {
            (-row(envelope, at, "element_local_axial_force", pipe, Some("end_i"), None, ""), row(envelope, at, "element_local_axial_force", pipe, Some("end_j"), None, ""))
        };
        out.insert(format!("axial_wall_force_N_w/{pipe}"), (wall_i, Family::Force));
        out.insert(format!("axial_wall_force_N_w/{pipe}@end_j"), (wall_j, Family::Force));
        for basis in BASES {
            for (node, end) in [(m["from"].as_str().unwrap(), "end_i"), (m["to"].as_str().unwrap(), "end_j")] {
                let axial = if wall_rows {
                    row(envelope, at, "pipe_axial_membrane_stress_v2", pipe, Some(end), None, basis)
                } else {
                    1.0e6 * row(envelope, at, "element_local_axial_normal_stress", pipe, Some(end), None, basis)
                };
                out.insert(format!("axial_normal_stress/{pipe}/{basis}/{node}"), (axial, Family::Stress));
            }
        }
    }
    out
}

/// The reference's linear values of one block, in the same keys.
fn expected_linear(case: &Value, block: &Value) -> BTreeMap<String, f64> {
    let mut out = BTreeMap::new();
    for (name, by_dof) in block["displacements"].as_object().unwrap() {
        for (dof, _, _) in DOFS {
            out.insert(format!("displacements/{name}/{dof}"), value(&by_dof[dof]));
        }
    }
    for (name, by) in block["reactions"].as_object().unwrap() {
        for (key, _, _) in ACTIONS {
            out.insert(format!("reactions/{name}/{key}"), value(&by[key]));
        }
    }
    for (pipe, by_node) in block["member_end_actions_global"].as_object().unwrap() {
        for (node, by) in by_node.as_object().unwrap() {
            for (key, _, _) in ACTIONS {
                out.insert(format!("member_end_actions_global/{pipe}/{node}/{key}"), value(&by[key]));
            }
        }
    }
    for (pipe, n) in block["axial_wall_force_N_w"].as_object().unwrap() {
        out.insert(format!("axial_wall_force_N_w/{pipe}"), value(n));
        out.insert(format!("axial_wall_force_N_w/{pipe}@end_j"), value(n));
    }
    for (pipe, by_basis) in block["section_stress_rows"].as_object().into_iter().flatten() {
        for basis in BASES {
            for (node, end) in by_basis[basis]["ends"].as_object().unwrap() {
                out.insert(format!("axial_normal_stress/{pipe}/{basis}/{node}"), value(&end["axial_normal_stress"]));
            }
        }
    }
    // A combination states its axial stress per member and basis (constant along it).
    for (key, by_basis) in block.as_object().unwrap() {
        if let Some(pipe) = key.strip_prefix("axial_normal_stress_") {
            let m = member(case, pipe);
            for basis in BASES {
                for node in [m["from"].as_str().unwrap(), m["to"].as_str().unwrap()] {
                    out.insert(format!("axial_normal_stress/{pipe}/{basis}/{node}"), value(&by_basis[basis]));
                }
            }
        }
    }
    out
}

/// Every linear reference value of a block against the product's.
fn compare_linear(observed: &BTreeMap<String, (f64, Family)>, expected: &BTreeMap<String, f64>, s: Scales, label: &str) -> usize {
    for (key, want) in expected {
        let (got, family) = observed.get(key).unwrap_or_else(|| panic!("{label}: the product publishes no {key}"));
        close(*got, *want, scale_of(s, *family), &format!("{label} {key}"));
    }
    expected.len()
}

/// The per-basis stress rows (bending as the resultant, torsion as magnitude,
/// the elastic maximum) and the Lamé rows of a solved case or state.
fn compare_stresses(case: &Value, block: &Value, lame: Option<&Value>, envelope: &Value, at: At<'_>, regions_of_case: &[String], s: Scales, label: &str) -> usize {
    let mut checked = 0;
    for (pipe, by_basis) in block["section_stress_rows"].as_object().unwrap() {
        for basis in BASES {
            for (node, end_rows) in by_basis[basis]["ends"].as_object().unwrap() {
                let end = end_of(case, pipe, node);
                let local = |kind: &str| 1.0e6 * row(envelope, at, kind, pipe, Some(end), None, basis);
                let bending = local("element_local_bending_normal_stress_y").hypot(local("element_local_bending_normal_stress_z"));
                let what = |q: &str| format!("{label} {pipe} {basis} {node} {q}");
                close(bending, value(&end_rows["bending_normal_stress_resultant"]), s.stress, &what("bending resultant"));
                close(local("element_local_torsional_shear_stress").abs(), value(&end_rows["torsional_shear_stress"]), s.stress, &what("torsion"));
                checked += 2;
            }
            let maximum = row(envelope, at, "pipe_elastic_normal_stress_maximum_v2", pipe, Some("governing_station"), None, basis);
            close(maximum, value(&by_basis[basis]["elastic_normal_maximum"]), s.stress, &format!("{label} {pipe} {basis} elastic maximum"));
            checked += 1;
        }
    }
    if let Some(lame) = lame {
        assert!(!regions_of_case.is_empty(), "{label}: Lamé rows without a region");
        for pipe in regions_of_case {
            let m = member(case, pipe);
            for basis in BASES {
                for (key, kind, component) in LAME {
                    let expected = value(&lame[basis][key]);
                    for node in [m["from"].as_str().unwrap(), m["to"].as_str().unwrap()] {
                        let end = end_of(case, pipe, node);
                        close(row(envelope, at, kind, pipe, Some(end), Some(component), basis), expected, s.stress, &format!("{label} {pipe} {basis} {key}"));
                        checked += 1;
                    }
                }
            }
        }
    }
    checked
}

/// The product's applied-load rows at one basis: (support, location, value).
fn applied_rows(envelope: &Value, at: At<'_>) -> Vec<(String, String, f64)> {
    envelope["results"].as_array().unwrap().iter()
        .filter(|r| r["kind"] == APPLIED && r["basis_ref"]["ref_type"] == at.ref_type && r["basis_ref"]["ref_id"] == at.ref_id)
        .map(|r| (r["entity_ref"].as_str().unwrap().to_string(), r["metadata"]["location"].as_str().unwrap().to_string(), r["value"].as_f64().unwrap()))
        .collect()
}

/// The reference's applied-load rows: +F along the positive axis of the DOF.
/// The record's own support-action rows (support on pipe) state the same
/// force on that axis and zero elsewhere; a record not consumed states zero.
fn compare_applied(case: &Value, block: &Value, envelope: &Value, at: At<'_>, label: &str) -> usize {
    let expected = block["constant_effort_applied_load"].as_array().unwrap();
    let observed = applied_rows(envelope, at);
    assert_eq!(observed.len(), expected.len(), "{label}: {observed:?}");
    for ((support, location, got), want) in observed.iter().zip(expected) {
        assert_eq!(support, want["support"].as_str().unwrap(), "{label}");
        let dof = want["acting_dof"].as_str().unwrap();
        assert_eq!(want["direction"], format!("+{}", &dof[1..]), "{label}");
        assert_eq!(location, &format!("{}:{dof} applied load", node_id(want["node"].as_str().unwrap())), "{label}");
        close(*got, value(&want["applied_load"]), 0.0, &format!("{label} applied load"));
        assert!(*got > 0.0, "{label}");
    }
    for record in constant_effort_records(case) {
        let id = record["id"].as_str().unwrap();
        let applied = expected.iter().find(|w| w["support"] == id);
        // A document without the record publishes nothing for it.
        if !envelope["results"].as_array().unwrap().iter().any(|r| r["entity_ref"] == id) {
            assert!(applied.is_none(), "{label}");
            continue;
        }
        for (key, component, _) in ACTIONS {
            let want = applied.filter(|w| w["acting_dof"].as_str().unwrap()[1..] == key[1..] && key.starts_with('F')).map_or(0.0, |w| value(&w["applied_load"]));
            close(row(envelope, at, "support_reaction_component_v2", id, None, Some(component), ""), want, 0.0, &format!("{label} {id} support action {key}"));
        }
    }
    expected.len()
}

/// One solved case or state of `case` against its block.
fn check_block(case: &Value, block: &Value, lame: Option<&Value>, envelope: &Value, at: At<'_>, regions_of_case: &[String], label: &str) -> usize {
    let s = scales(case, block);
    let observed = linear_values(case, envelope, at, regions_of_case);
    let mut checked = compare_linear(&observed, &expected_linear(case, block), s, label);
    // A capped member's effective force S = N_w - p A_i.
    for (pipe, effective) in block["effective_axial_force_S"].as_object().into_iter().flatten() {
        if regions_of_case.iter().any(|p| p == pipe) {
            for end in ["end_i", "end_j"] {
                close(row(envelope, at, "pipe_effective_axial_force_v2", pipe, Some(end), None, ""), value(effective), s.force, &format!("{label} {pipe} S {end}"));
                checked += 1;
            }
        }
    }
    let lame = lame.or_else(|| block.get("lame_rows"));
    checked += compare_stresses(case, block, lame, envelope, at, regions_of_case, s, label);
    checked + compare_applied(case, block, envelope, at, label)
}

// ----------------------------------------------------------------- CE-1

/// CE-1: the R-4 S2 cantilever with and without the record, under both shear
/// models, unpressurized and capped; with − without is the F-only solution.
#[test]
fn ce1_cantilever_binds_with_and_without_the_record() {
    let references = references();
    let mut checked = 0;
    for label in ["CE1-EB", "CE1-EB-P", "CE1-T", "CE1-T-P"] {
        let case = case(&references, label);
        let results = &case["results"];
        let in_region = pressurized(&case["inputs"]);
        let with = plain_document(&references, case, true);
        let without = plain_document(&references, case, false);
        let s = scales(case, &results["with_support"]).max(scales(case, &results["without_record"]));
        for mode in MODES {
            let what = format!("{label} {mode:?}");
            let at = case_at("case:main");
            let a = solved(&with, mode, &what);
            let b = solved(&without, mode, &format!("{what} without"));
            checked += check_block(case, &results["with_support"], None, &a, at, &in_region, &what);
            checked += check_block(case, &results["without_record"], None, &b, at, &in_region, &format!("{what} without"));
            assert!(with_code(&a, NOT_CONSUMED).is_empty(), "{what}");
            // with − without == F-only (superposition, exact in the reference).
            let (va, vb) = (linear_values(case, &a, at, &in_region), linear_values(case, &b, at, &in_region));
            for (dof, _, _) in DOFS {
                let key = format!("displacements/B/{dof}");
                close(va[&key].0 - vb[&key].0, value(&results["F_only"]["tip_displacement_B"][dof]), scale_of(s, va[&key].1), &format!("{what} F-only {dof}"));
            }
            for (key, _, _) in ACTIONS {
                let path = format!("reactions/A/{key}");
                close(va[&path].0 - vb[&path].0, value(&results["F_only"]["reaction_A"][key]), scale_of(s, va[&path].1), &format!("{what} F-only {key}"));
            }
            checked += 12;
        }
    }
    assert!(checked > 900, "{checked}");
}

/// The sign and double-count mutation controls: the product's CE1-EB values
/// are distinguished from each mutant's far beyond the binding tolerance.
#[test]
fn ce1_mutation_controls_are_distinguished() {
    let references = references();
    let admitted = case(&references, "CE1-EB");
    let document = plain_document(&references, admitted, true);
    let s = scales(admitted, &admitted["results"]["with_support"]);
    for label in ["CE1-EB-MUT-SIGN", "CE1-EB-MUT-TWICE"] {
        let control = case(&references, label);
        assert_eq!(control["admitted_case"], false);
        for mode in MODES {
            let envelope = solved(&document, mode, label);
            let observed = linear_values(admitted, &envelope, case_at("case:main"), &[]);
            let (uz, wrong) = (observed["displacements/B/UZ"].0, value(&control["results"]["tip_displacement_B"]["UZ"]));
            assert!((uz - wrong).abs() > 1e6 * 1e-9 * wrong.abs().max(s.translation), "{label} {mode:?}: {uz} vs {wrong}");
            let (fz, wrong) = (observed["reactions/A/FZ"].0, value(&control["results"]["reaction_A"]["FZ"]));
            assert!((fz - wrong).abs() > 1.0, "{label} {mode:?}: {fz} vs {wrong}");
        }
    }
}

// ----------------------------------------------------------------- CE-1n

/// CE-1n: a record declaring two translational DOFs, or a rotation only, is
/// not consumed: no force, no applied-load row, one named warning, and every
/// published mechanics value identical to the model without the record.
#[test]
fn ce1n_non_consumed_records_apply_nothing() {
    let references = references();
    for label in ["CE1N-2T-EB", "CE1N-ROT-EB", "CE1N-2T-T", "CE1N-ROT-T"] {
        let case = case(&references, label);
        let record = &constant_effort_records(case)[0];
        assert_eq!(record["consumed"], false);
        let with = plain_document(&references, case, true);
        let without = plain_document(&references, case, false);
        for mode in MODES {
            let what = format!("{label} {mode:?}");
            let a = solved(&with, mode, &what);
            check_block(case, &case["results"], None, &a, case_at("case:main"), &[], &what);
            assert!(applied_rows(&a, case_at("case:main")).is_empty(), "{what}");
            let warnings = with_code(&a, NOT_CONSUMED);
            assert_eq!(warnings.len(), 1, "{what}");
            assert_eq!(warnings[0]["severity"], "warning");
            assert_eq!(warnings[0]["affected_refs"], json!([record["id"], node_id(record["node"].as_str().unwrap())]), "{what}");
            let reason = if label.contains("2T") { "2 translational restraint DOFs are declared" } else { "no translational restraint DOF is declared" };
            assert!(warnings[0]["message"].as_str().unwrap().contains(reason), "{what}");
            // Identical to the model without the record, bit for bit, but for
            // the record's review rows and its (zero) support-action rows.
            let b = solved(&without, mode, &format!("{what} without"));
            let mechanics = |envelope: &Value| envelope["results"].as_array().unwrap().iter()
                .filter(|r| r["entity_ref"] != record["id"])
                .map(|r| (r["id"].as_str().unwrap().to_string(), r["value"].as_f64().unwrap().to_bits()))
                .collect::<BTreeMap<_, _>>();
            assert_eq!(mechanics(&a), mechanics(&b), "{what}");
        }
    }
}

// ----------------------------------------------------------------- CE-2

fn combination_id(index: usize) -> String {
    format!("combination:ce2-{index}")
}

/// CE-2's document with every reference combination whose factors the value
/// entry admits (2⁵³·A − 2⁵³·B is exercised separately), each term's factor
/// the stated exact dyadic (equal to its decimal input's binary64).
fn ce2_document(references: &Value) -> (Value, Vec<(usize, Value)>) {
    let case = case(references, "CE2");
    let mut document = plain_document(references, case, true);
    let mut authored = Vec::new();
    let mut combinations = Vec::new();
    for (index, combination) in case["results"]["combinations"].as_array().unwrap().iter().enumerate() {
        let Some(factors) = combination["factors_binary64"].as_object() else { continue };
        let terms = ["A", "B"].iter().map(|name| {
            let f = ratio(&factors[*name]["exact_dyadic"]);
            assert_eq!(f, factors[*name]["decimal_input"].as_str().unwrap().parse::<f64>().unwrap());
            json!({"load_case": format!("case:{name}"), "factor": f})
        }).collect::<Vec<_>>();
        combinations.push(json!({"id": combination_id(index), "basis": "mechanics", "terms": terms, "provenance": PROVENANCE}));
        authored.push((index, combination.clone()));
    }
    document["model"]["combinations"] = Value::Array(combinations);
    (document, authored)
}

fn gate<'a>(envelope: &'a Value, id: &str) -> &'a Value {
    envelope["contract_evidence"]["exact_combinations"].as_array().unwrap().iter().find(|r| r["combination_id"] == id).unwrap()
}

fn combination_rows(envelope: &Value, id: &str) -> usize {
    envelope["results"].as_array().unwrap().iter().filter(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == id).count()
}

/// CE-2: both cases bind; every admitted combination publishes the exact
/// factor sums of the case values (the constant force counted Σf times); every
/// withheld one is withheld under PF's reason and text, and the reader
/// recomputes each decision.
#[test]
fn ce2_combinations_bind_under_the_unit_factor_sum_rule() {
    let references = references();
    let case = case(&references, "CE2");
    let results = &case["results"];
    let (document, authored) = ce2_document(&references);
    assert_eq!(authored.len(), 8);
    let regions_b = pressurized(&case["inputs"]["load_cases"]["B"]);
    let s = scales(case, &results["case_A"]).max(scales(case, &results["case_B"]));
    for mode in MODES {
        let what = format!("CE2 {mode:?}");
        let envelope = solved(&document, mode, &what);
        check_block(case, &results["case_A"], None, &envelope, case_at("case:A"), &[], &format!("{what} A"));
        check_block(case, &results["case_B"], None, &envelope, case_at("case:B"), &regions_b, &format!("{what} B"));
        let (mut admitted, mut withheld) = (0, 0);
        for (index, combination) in &authored {
            let id = combination_id(*index);
            let label = format!("{what} {}", combination["combination"].as_str().unwrap());
            let record = gate(&envelope, &id);
            let stated = with_code(&envelope, GATE_CODE).into_iter().filter(|d| d["affected_refs"] == json!([id])).collect::<Vec<_>>();
            if combination["gate"] == "admitted" {
                assert_eq!(record["gate"], json!({"withheld": false, "reason": null}), "{label}");
                assert!(stated.is_empty(), "{label}");
                let combined = &combination["combined"];
                let at = At { ref_type: "combination", ref_id: &id };
                compare_linear(&linear_values(case, &envelope, at, &[]), &expected_linear(case, combined), s, &label);
                // The constant force is counted sum(f) times: a combined
                // applied-load row, where the product emits one, is that count times F.
                for (_, _, applied) in applied_rows(&envelope, at) {
                    close(applied, value(&combined["constant_force_in_combined_state"]), 0.0, &format!("{label} applied"));
                }
                admitted += 1;
            } else {
                assert_eq!(combination["gate"], format!("withheld: {GATE_CODE}"), "{label}");
                assert_eq!(record["gate"], json!({"withheld": true, "reason": GATE_CODE}), "{label}");
                assert_eq!(combination_rows(&envelope, &id), 0, "{label}");
                assert_eq!(stated.len(), 1, "{label}");
                assert_eq!(stated[0]["severity"], "warning");
                assert_eq!(stated[0]["message"], GATE_TEXT, "{label}");
                withheld += 1;
            }
        }
        assert_eq!((admitted, withheld), (5, 3), "{what}");
    }
}

/// 2⁵³·A − 2⁵³·B: refused upstream by the value entry; a typed entry reaches
/// the gate, which withholds it, and the reader recomputes the withholding.
#[test]
fn ce2_out_of_domain_factors_are_refused_upstream_and_withheld_at_the_gate() {
    let references = references();
    let case = case(&references, "CE2");
    let stated = case["results"]["combinations"].as_array().unwrap().iter().find(|c| c["combination"] == "2^53*A - 2^53*B").unwrap();
    assert_eq!(stated["exact_factor_sum"], "0");
    let p53 = 9007199254740992.0_f64;
    let mut document = plain_document(&references, case, true);
    document["model"]["combinations"] = json!([{"id": "combination:ce2-p53", "basis": "mechanics",
        "terms": [{"load_case": "case:A", "factor": p53}, {"load_case": "case:B", "factor": -p53}], "provenance": PROVENANCE}]);
    for mode in MODES {
        let refused = run_linear_static_preview_value_with_mode(document.clone(), mode).map(|_| ()).unwrap_err();
        assert!(refused.starts_with("CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT"), "{refused}");
        let request: LinearStaticPreviewRequest = serde_json::from_value(document.clone()).unwrap();
        let envelope = serde_json::to_value(run_linear_static_preview_with_mode(request, mode)).unwrap();
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
        assert_eq!(gate(&envelope, "combination:ce2-p53")["gate"], json!({"withheld": true, "reason": GATE_CODE}), "{mode:?}");
        assert_eq!(combination_rows(&envelope, "combination:ce2-p53"), 0);
        assert_eq!(reader_verdict::validate("pressure", &envelope, None, None)["ok"], true, "{mode:?}");
    }
}

fn record_index(source: &Value, id: &str) -> usize {
    source["contract_evidence"]["exact_combinations"].as_array().unwrap().iter().position(|r| r["combination_id"] == id).unwrap()
}

/// Negative controls on CE-2, CE-3 and CE-1n: a forged reason, a forged factor
/// sum, a constant-effort term dropped from a case or state, a changed force
/// and a forged consumption are each refused by the pressure reader.
#[test]
fn forged_reasons_factor_sums_and_dropped_terms_are_refused() {
    let references = references();
    let (document, authored) = ce2_document(&references);
    let envelope = solved(&document, PreviewSolverMode::SparseInteractive, "CE2");
    let withheld = combination_id(authored.iter().find(|(_, c)| c["combination"] == "2*A").unwrap().0);
    let admitted = combination_id(authored.iter().find(|(_, c)| c["combination"] == "0.3*A + 0.7*B").unwrap().0);
    // 1. A forged reason: the withheld record (and its diagnostic) names gate
    // 3's reason, or an admitted record states gate 4.
    let mut forged = envelope.clone();
    let i = record_index(&forged, &withheld);
    forged["contract_evidence"]["exact_combinations"][i]["gate"]["reason"] = json!("COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE");
    for d in forged["diagnostics"].as_array_mut().unwrap() {
        if d["affected_refs"] == json!([withheld.clone()]) && d["code"] == GATE_CODE {
            d["code"] = json!("COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE");
        }
    }
    assert!(refusal(&forged, None).contains("SOURCE_PHYSICS_COMBINATION_GATE"), "forged reason");
    let mut forged = envelope.clone();
    let i = record_index(&forged, &admitted);
    forged["contract_evidence"]["exact_combinations"][i]["gate"] = json!({"withheld": true, "reason": GATE_CODE});
    assert!(refusal(&forged, None).contains("SOURCE_PHYSICS_COMBINATION_GATE"), "forged withholding");
    // 2. A forged factor sum: the withheld 2*A record states factor 1, which
    // the reader's gate admits, so the stated withholding is refused; the
    // admitted 0.3/0.7 record restated as 0.3/0.8 is withheld by the reader.
    let mut forged = envelope.clone();
    let i = record_index(&forged, &withheld);
    forged["contract_evidence"]["exact_combinations"][i]["expression"]["terms"][0]["factor"] = json!(1.0);
    assert!(refusal(&forged, None).contains("SOURCE_PHYSICS_COMBINATION_GATE"), "forged factor sum");
    assert!(refusal(&forged, Some(&document)).contains("SOURCE_PHYSICS_COMBINATION_GATE"), "forged factor sum (model)");
    let mut forged = envelope.clone();
    let i = record_index(&forged, &admitted);
    forged["contract_evidence"]["exact_combinations"][i]["expression"]["terms"][1]["factor"] = json!(0.8);
    assert!(refusal(&forged, None).contains("SOURCE_PHYSICS_COMBINATION_GATE"), "forged admitted factor sum");
    // 3. Dropped constant-effort terms, from every case or from case B: the
    // combined applied-load rows lose their operands (and, were those forged
    // too, the gate input becomes "no consuming support" and the stated
    // withholdings are refused; with the model, the model binding refuses it,
    // as on CE-3 below, which has no combination).
    let mut forged = envelope.clone();
    forged["results"].as_array_mut().unwrap().retain(|r| !(r["kind"] == APPLIED && r["basis_ref"]["ref_type"] == "load_case"));
    // (The combined applied-load rows then lack their operand rows.)
    let text = refusal(&forged, None);
    assert!(text.contains("SOURCE_PHYSICS_COMBINATION_ROW_EVIDENCE"), "dropped terms: {text}");
    let mut forged = envelope.clone();
    forged["results"].as_array_mut().unwrap().retain(|r| !(r["kind"] == APPLIED && r["basis_ref"]["ref_id"] == "case:B"));
    let text = refusal(&forged, Some(&document));
    assert!(text.contains("SOURCE_PHYSICS_COMBINATION_ROW_EVIDENCE"), "dropped case:B term: {text}");
    // The gate's input is the model's: the same received CE-2 document against
    // a model whose record is not consumed (two translational DOFs, a
    // nonlinear field, a zero load) or consumes another force is refused.
    for (field, forged_value) in [("restraints", json!(["UY", "UZ"])), ("nonlinear", json!({"behavior": "gap"})),
        ("constant_load", json!({"value": 0.0, "unit": "N"})), ("constant_load", json!({"value": 8.0, "unit": "kN"})),
        ("constant_load", json!({"value": 8001.0, "unit": "N"}))] {
        let mut model = document.clone();
        let record = model["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["family"] == "constant_effort_support").unwrap();
        if field == "constant_load" {
            record["hanger"][field] = forged_value.clone();
        } else {
            record[field] = forged_value.clone();
        }
        let found = reader_verdict::validate("pressure", &envelope, Some(&model["model"]), None);
        if forged_value == json!({"value": 8.0, "unit": "kN"}) {
            // 8 kN is 8000 N bit for bit: the same consumption.
            assert_eq!(found["ok"], true, "{field}: {found}");
        } else {
            assert!(refusal(&envelope, Some(&model)).contains("MODEL_CONSTANT_EFFORT"), "{field} {forged_value}");
        }
    }
    // CE-3: the term dropped from state O, or its force doubled.
    let ce3 = case(&references, "CE3-EB");
    let state = state_document(&references, ce3, true);
    let envelope = solved(&state, PreviewSolverMode::SparseInteractive, "CE3-EB");
    let mut forged = envelope.clone();
    forged["results"].as_array_mut().unwrap().retain(|r| !(r["kind"] == APPLIED && r["basis_ref"]["ref_id"] == "case:O"));
    assert!(refusal(&forged, Some(&state)).contains("MODEL_CONSTANT_EFFORT"), "dropped state O term");
    let mut forged = envelope.clone();
    for r in forged["results"].as_array_mut().unwrap() {
        if r["kind"] == APPLIED && r["basis_ref"]["ref_id"] == "case:O" {
            r["value"] = json!(2.0 * r["value"].as_f64().unwrap());
        }
    }
    assert!(refusal(&forged, Some(&state)).contains("MODEL_CONSTANT_EFFORT"), "doubled state O term");
    // CE-1n: an applied-load row forged for a record the model does not consume.
    let ce1n = case(&references, "CE1N-ROT-EB");
    let document = plain_document(&references, ce1n, true);
    let envelope = solved(&document, PreviewSolverMode::SparseInteractive, "CE1N-ROT-EB");
    let consumed = solved(&plain_document(&references, case(&references, "CE1-EB"), true), PreviewSolverMode::SparseInteractive, "CE1-EB");
    let mut forged = envelope.clone();
    forged["results"].as_array_mut().unwrap().push(consumed["results"].as_array().unwrap().iter().find(|r| r["kind"] == APPLIED).unwrap().clone());
    assert!(refusal(&forged, Some(&document)).contains("MODEL_CONSTANT_EFFORT"), "forged consumption");
}

// ----------------------------------------------------------------- CE-3

/// CE-3 (D-2): on a load/reference-state document the constant force is a
/// device action of the active support in both states. Each state binds; O − S
/// equals the thermal-only solution (the force, weight and pressure cancel);
/// with − without the record is the F-only solution in both states; O − S does
/// not depend on the force; the O-omission mutant is distinguished.
#[test]
fn ce3_load_state_documents_bind_and_the_force_cancels_in_o_minus_s() {
    let references = references();
    let control = case(&references, "CE3-EB-MUT-OMIT-O");
    assert_eq!(control["admitted_case"], false);
    for label in ["CE3-EB", "CE3-EB-P", "CE3-T-P"] {
        let case = case(&references, label);
        let results = &case["results"];
        let states = &case["inputs"]["states"];
        let (regions_s, regions_o) = (pressurized(&states["S"]), pressurized(&states["O"]));
        assert_eq!(regions_s, regions_o);
        let lame = results.get("lame_rows_both_states");
        let with = state_document(&references, case, true);
        let without = state_document(&references, case, false);
        let s = scales(case, &results["state_S"]).max(scales(case, &results["state_O"]));
        for mode in MODES {
            let what = format!("{label} {mode:?}");
            let a = solved(&with, mode, &what);
            check_block(case, &results["state_S"], lame, &a, case_at("case:S"), &regions_s, &format!("{what} S"));
            check_block(case, &results["state_O"], lame, &a, case_at("case:O"), &regions_o, &format!("{what} O"));
            // O − S, formed from the product's own states.
            let (o, st) = (linear_values(case, &a, case_at("case:O"), &regions_o), linear_values(case, &a, case_at("case:S"), &regions_s));
            let difference = o.iter().map(|(k, (v, f))| (k.clone(), (v - st[k].0, *f))).collect::<BTreeMap<_, _>>();
            compare_linear(&difference, &expected_linear(case, &results["O_minus_S"]), s, &format!("{what} O-S"));
            // The thermal motion moves the hanger node along its DOF.
            assert!(difference["displacements/C/UZ"].0.abs() > 1e-3, "{what}");
            // With − without the record is the F-only solution in both states,
            // and O − S does not depend on the force.
            let b = solved(&without, mode, &format!("{what} without"));
            assert!(applied_rows(&b, case_at("case:S")).is_empty() && applied_rows(&b, case_at("case:O")).is_empty());
            let (ob, sb) = (linear_values(case, &b, case_at("case:O"), &regions_o), linear_values(case, &b, case_at("case:S"), &regions_s));
            for (dof, _, _) in DOFS {
                let key = format!("displacements/C/{dof}");
                let scale = scale_of(s, o[&key].1);
                let f_only = value(&results["F_only"]["displacement_C"][dof]);
                close(st[&key].0 - sb[&key].0, f_only, scale, &format!("{what} S F-only {dof}"));
                close(o[&key].0 - ob[&key].0, f_only, scale, &format!("{what} O F-only {dof}"));
                close(difference[&key].0, ob[&key].0 - sb[&key].0, scale, &format!("{what} O-S without the force {dof}"));
            }
            // The mutant omitting the force from O only (CE3-EB's numbers) is distinguished.
            if label == "CE3-EB" {
                let (got, wrong) = (difference["displacements/C/UZ"].0, value(&control["results"]["O_minus_S_displacement_C"]["UZ"]));
                assert!((got - wrong).abs() > 1e3 * 1e-9 * wrong.abs().max(s.translation), "{what}: {got} vs {wrong}");
            }
        }
    }
}

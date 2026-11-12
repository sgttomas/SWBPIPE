//! RT-A3 reducer increment 2b (DEL-04-07 `pressure-contract.md`, "Eccentric
//! reducer pressure step"; DEL-03-05 `eccentric-reducer-offset.md` §3.7 and
//! `rigid-component-stiffness-scaling.md` §2.7; DEL-05-03
//! `section-bases-and-timoshenko.md` §2.1): the eccentric (and slanted)
//! pressure step on the exact route, through PP's public entry in both solver
//! modes, against the candidate independent references in
//! `validation/references/rt_a3_reducer_2b/` (27 cases on the RS-R1 four-node
//! neighbourhood A = RS-170, N = RS-160, F = RS-150, C = RS-140).
//!
//! Each case is authored as a current `3.0.0/exact_pressure_v3` document from
//! its own inputs: every member its own region (pressurized cases), each
//! terminal `transfers_to_wall` unless the case omits that cap (then
//! `separately_supported_or_compensated`), the reducer at N mapping the body
//! as `mechanics_geometry_only` with the case's DEC-045 scale and the RS-R1
//! offset, 25 mm along the members' `y_reference`. Zero-pressure mechanical
//! cases have no region, so their body lies outside every region.
//!
//! Every published binding row is compared at |observed − expected| ≤
//! 1e-9·max(|expected|, zero_scale) with the packet's own family and scope
//! zero scales (`zero_scales`, `lame_zero_scales_Pa`); none is chosen from
//! product output. Element stress rows are MPa and compared after ×1e6;
//! membrane, Lamé and maximum rows are Pa. The packet's values are the exact
//! decimal-input model, not product binary64 leaves, so equality is never
//! asserted. A mismatch is the product's to explain, never the reference's.
//!
//! Row contracts (existing, unchanged): section axial = `pipe_wall_axial_force_v2`
//! (N_w) in a region, `element_local_axial_force` outside; S =
//! `pipe_effective_axial_force_v2`; node-on-member axial end action =
//! `pipe_wall_endpoint_action_v2`; global end wrenches =
//! `element_global_end_force_v1`/`_moment_v1` (node on element); station
//! shears and moments are the j-side section action. σ± fibre coordinates are
//! absent from the packet and are not compared.
use open_pipe_stress_product_physics::result_export::semantic_contract;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const REFERENCES: &str = include_str!("../../../validation/references/rt_a3_reducer_2b/references.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const CASE: &str = "case:pressure";
const REDUCER: &str = "component:reducer";
const BODY: &str = "pipe:body";
const PROVENANCE: &str = "invented_rt_a3_reducer_2b_reference";
const BASES: [&str; 2] = ["nominal_section", "reduced_section"];
const STATIONS: [&str; 5] = ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"];
const REDUCER_IN_REGION: &str = "EXACT_PRESSURE_REDUCER_IN_REGION";
const REDUCER_TEXT_HEAD: &str = "reducer maps a member of this pressure region but is not the admitted concentric pressure-bore step";
const PRESSURES_DIFFER: &str = "EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER";
const OUTSIDE_REGIONS: &str = "EXACT_PRESSURE_JUNCTION_PIPE_OUTSIDE_REGIONS";
const ECC_REALIZED: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_REALIZED";
const ECC_AXIS_ALLOWANCE: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_AXIS_ALLOWANCE_USED";
const ECC_INVALID: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_INVALID";
const ECC_MISMATCH: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_MISMATCH";

fn references() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}

fn case<'a>(references: &'a Value, label: &str) -> &'a Value {
    references["cases"].as_array().unwrap().iter().find(|c| c["label"] == label).unwrap_or_else(|| panic!("no case {label}"))
}

/// A reference input number: an integer or a ratio "a/b" (inputs only).
fn ratio(text: &Value) -> f64 {
    let text = text.as_str().unwrap_or_else(|| panic!("not a ratio string: {text}"));
    match text.split_once('/') {
        Some((a, b)) => a.parse::<f64>().unwrap() / b.parse::<f64>().unwrap(),
        None => text.parse::<f64>().unwrap(),
    }
}

fn vec3(v: &Value) -> [f64; 3] {
    let a = v.as_array().unwrap();
    [ratio(&a[0]), ratio(&a[1]), ratio(&a[2])]
}

/// The decided binary64 of an expected quantity (the packet has no undecided value).
fn b64(v: &Value) -> f64 {
    v["binary64"].as_f64().unwrap_or_else(|| panic!("no decided binary64 in {v}"))
}

fn quantity(v: f64, unit: &str) -> Value {
    json!({"value": v, "unit": unit})
}

fn short(member_id: &str) -> &str {
    member_id.split(':').next().unwrap()
}

fn pipe_id(member_id: &str) -> String {
    format!("pipe:{}", short(member_id))
}

fn inputs_members(case: &Value) -> &Vec<Value> {
    case["inputs"]["members"].as_array().unwrap()
}

fn body_input(case: &Value) -> &Value {
    inputs_members(case).iter().find(|m| m["role"] == "reducer_body").unwrap()
}

/// The RS-R1 declared offset: 25 mm along the body's `y_reference` (the
/// rotated +z of the ROT cases), from N (R-c1 option A).
fn rs_offset(case: &Value) -> [f64; 3] {
    vec3(&body_input(case)["y_reference"]).map(|v| 0.025 * v)
}

fn pressurized(case: &Value) -> bool {
    inputs_members(case).iter().any(|m| ratio(&m["pressure_Pa"]) != 0.0)
}

/// The case as a current exact document with the reducer's `offset` (m).
fn document(references: &Value, case: &Value, offset: Option<[f64; 3]>) -> Value {
    let model = &references["model"];
    let inputs = &case["inputs"];
    let nodes = inputs["nodes_m"].as_object().unwrap().iter().map(|(name, x)| {
        let p = vec3(x);
        json!({"id": format!("node:{name}"), "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let members = inputs_members(case);
    let pipes = members.iter().map(|m| {
        let section = &model["sections"][m["section"].as_str().unwrap()];
        let length = |key: &str| quantity(ratio(&section[key]), "m");
        let y = vec3(&m["y_reference"]);
        json!({"id": pipe_id(m["id"].as_str().unwrap()), "from": format!("node:{}", m["from"].as_str().unwrap()), "to": format!("node:{}", m["to"].as_str().unwrap()),
            "section": {"outside_diameter": length("D"), "wall_thickness": length("t"), "mill_tolerance": length("cm"), "corrosion_allowance": length("ca")},
            "material": "material:rt-a3-2b", "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let supports = inputs["anchors_all_six_dof"].as_array().unwrap().iter().map(|name| {
        let name = name.as_str().unwrap();
        json!({"id": format!("support:{name}"), "node": format!("node:{name}"), "family": "anchor",
            "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let regions = if pressurized(case) {
        members.iter().map(|m| {
            let transferring = m["transferring_terminals"].as_array().unwrap();
            let terminal = |end: &str| {
                let node = m[end].as_str().unwrap();
                let closure = if transferring.iter().any(|t| t == node) { "transfers_to_wall" } else { "separately_supported_or_compensated" };
                json!({"node_ref": format!("node:{node}"), "closure_transfer": closure, "provenance": PROVENANCE})
            };
            json!({"id": format!("region:{}", short(m["id"].as_str().unwrap())), "member_pipe_ids": [pipe_id(m["id"].as_str().unwrap())],
                "pressure_basis": "internal_differential_zero_external_v1",
                "pressure": quantity(ratio(&m["pressure_Pa"]), "Pa"),
                "terminals": [terminal("from"), terminal("to")], "provenance": PROVENANCE})
        }).collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let mut loads = Vec::new();
    for (node, load) in inputs["mechanical_nodal_loads"].as_object().unwrap() {
        let (f, m) = (vec3(&load["F_N"]), vec3(&load["M_Nm"]));
        for (k, axis) in ["x", "y", "z"].iter().enumerate() {
            loads.push(json!({"id": format!("load:{node}:f{axis}"), "category": "concentrated_force", "target": {"type": "node", "node": format!("node:{node}")},
                "direction": format!("global_{axis}"), "dimension": "force", "magnitude": quantity(f[k], "N"), "provenance": PROVENANCE}));
            loads.push(json!({"id": format!("load:{node}:m{axis}"), "category": "concentrated_moment", "target": {"type": "node", "node": format!("node:{node}")},
                "direction": format!("rotation_{axis}"), "dimension": "moment", "magnitude": quantity(m[k], "N*m"), "provenance": PROVENANCE}));
        }
    }
    let body = body_input(case);
    assert_eq!(pipe_id(body["id"].as_str().unwrap()), BODY);
    let mut geometry = json!({"rigid_pipe_ref": BODY});
    if let Some(e) = offset {
        geometry["eccentric_offset"] = json!({"x": e[0], "y": e[1], "z": e[2], "unit": "m"});
    }
    let reducer = json!({"id": REDUCER, "kind": "reducer", "node": "node:N", "geometry": geometry,
        "modifiers": {"stiffness_scaling_user_value": quantity(ratio(&body["stiffness_scale"]), "1"), "source_reference": "invented RT-A3 reducer 2b scale"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROVENANCE});
    let material = &model["material_invented_control"];
    let mut document = json!({"model": {"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": inputs["shear_model"]},
        "project": {"id": "project:rt-a3-reducer-2b", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes,
        "materials": [{"id": "material:rt-a3-2b", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": quantity(ratio(&material["E_Pa"]), "Pa"), "poisson_ratio": quantity(ratio(&material["nu"]), "1"), "provenance": PROVENANCE}],
        "supports": supports, "components": [reducer],
        "load_cases": [{"id": CASE, "primitive_loads": loads, "pressure_regions": regions, "provenance": PROVENANCE}],
        "combinations": []},
        "materials": []});
    // RT-L2: the references are today's mechanics, pressure elongation included.
    if pressurized(case) {
        document["model"]["model_settings"] = json!({"pressure_elongation": {"operating_load_cases": [CASE]}});
    }
    document
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

fn blocking(envelope: &Value) -> Vec<String> {
    diagnostics(envelope).iter().filter(|d| d["severity"] == "blocking").map(|d| format!("{}: {}", d["code"], d["message"])).collect()
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

/// One binding comparison; counts it.
struct Binder<'a> {
    label: &'a str,
    count: usize,
    /// The largest |error|/tolerance seen, with its row.
    worst: (f64, String),
}

impl Binder<'_> {
    fn close(&mut self, observed: f64, expected: f64, scale: f64, what: &str) {
        assert!(scale > 0.0 && scale.is_finite(), "{} {what}: zero scale {scale}", self.label);
        let tolerance = 1e-9 * expected.abs().max(scale);
        assert!((observed - expected).abs() <= tolerance,
            "{} {what}: observed {observed:e}, expected {expected:e} (tolerance {tolerance:e}, |error|/tolerance {:e})",
            self.label, (observed - expected).abs() / tolerance);
        self.count += 1;
        let used = (observed - expected).abs() / tolerance;
        if used > self.worst.0 {
            self.worst = (used, format!("{} {what}", self.label));
        }
    }
}

const LAME: [(&str, &str, &str); 4] = [
    ("radial_inner", "pipe_lame_radial_stress_v2", "lame_inner_radial_stress"),
    ("radial_outer", "pipe_lame_radial_stress_v2", "lame_outer_radial_stress"),
    ("hoop_inner", "pipe_lame_hoop_stress_v2", "lame_inner_hoop_stress"),
    ("hoop_outer", "pipe_lame_hoop_stress_v2", "lame_outer_hoop_stress"),
];

/// A Lamé value: its decided binary64 where emitted, else its exact integer.
fn lame_value(rows: &Value, key: &str) -> f64 {
    rows.get(format!("{key}_binary64")).and_then(Value::as_f64).unwrap_or_else(|| ratio(&rows[format!("{key}_Pa")]))
}

/// Every binding row of one admitted case against its reference; returns the
/// number of comparisons and the largest fraction of a tolerance used.
fn check_case(references: &Value, case: &Value, envelope: &Value, label: &str) -> (usize, (f64, String)) {
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {:#?}", blocking(envelope));
    assert!(blocking(envelope).is_empty(), "{label}: {:#?}", blocking(envelope));
    assert!(with_code(envelope, REDUCER_IN_REGION).is_empty(), "{label}");
    let mut b = Binder { label, count: 0, worst: (0.0, String::new()) };
    let results = &case["results"];
    let scales = &case["zero_scales"];
    let s = |family: &str| b64(&scales["case"][family]);
    for (name, motion) in results["displacements"].as_object().unwrap() {
        let Some(motion) = motion.as_object() else { continue };
        let node = format!("node:{name}");
        for (key, kind) in [("ux_m", "global_nodal_displacement_x"), ("uy_m", "global_nodal_displacement_y"), ("uz_m", "global_nodal_displacement_z")] {
            b.close(row(envelope, kind, &node, None, None, ""), 1000.0 * b64(&motion[key]), 1000.0 * s("translation"), &format!("{node} {key} (mm)"));
        }
        for (key, kind) in [("rx_rad", "global_nodal_rotation_x"), ("ry_rad", "global_nodal_rotation_y"), ("rz_rad", "global_nodal_rotation_z")] {
            b.close(row(envelope, kind, &node, None, None, ""), b64(&motion[key]), s("rotation"), &format!("{node} {key}"));
        }
    }
    for (name, reaction) in results["reactions"].as_object().unwrap() {
        let support = format!("support:{name}");
        for (key, component, family) in [("Fx_N", "Fx", "reaction_force"), ("Fy_N", "Fy", "reaction_force"), ("Fz_N", "Fz", "reaction_force"),
            ("Mx_Nm", "Mx", "reaction_moment"), ("My_Nm", "My", "reaction_moment"), ("Mz_Nm", "Mz", "reaction_moment")] {
            b.close(row(envelope, "support_reaction_component_v2", &support, None, Some(component), ""), b64(&reaction[key]), s(family), &format!("{support} {component}"));
        }
    }
    let in_region = pressurized(case);
    let sections = &case["section_recovery"]["members"];
    for (member, input) in results["members"].as_array().unwrap().iter().zip(inputs_members(case)) {
        let id = member["id"].as_str().unwrap();
        assert_eq!(input["id"], member["id"]);
        let pipe = pipe_id(id);
        let ms = &scales["members"][short(id)];
        let (force, moment) = (b64(&ms["member_force"]), b64(&ms["member_moment"]));
        // Node-on-member end wrenches in global axes.
        for (end, location) in [("from", "end_i"), ("to", "end_j")] {
            let wrench = &member["end_wrench_on_member_global"][end];
            for (key, kind, component, scale) in [("Fx_N", "element_global_end_force_v1", "Fx", force), ("Fy_N", "element_global_end_force_v1", "Fy", force),
                ("Fz_N", "element_global_end_force_v1", "Fz", force), ("Mx_Nm", "element_global_end_moment_v1", "Mx", moment),
                ("My_Nm", "element_global_end_moment_v1", "My", moment), ("Mz_Nm", "element_global_end_moment_v1", "Mz", moment)] {
                b.close(row(envelope, kind, &pipe, Some(location), Some(component), ""), b64(&wrench[key]), scale, &format!("{pipe} global end {end} {component}"));
            }
        }
        // The node's axial action on the member, member axes: −N_w at from, +N_w at to.
        if in_region {
            for (end, location) in [("from", "end_i"), ("to", "end_j")] {
                b.close(row(envelope, "pipe_wall_endpoint_action_v2", &pipe, Some(location), Some("wall_axial_end_action"), ""),
                    b64(&member["end_wrench_on_member_local"][end]["Fx_N"]), force, &format!("{pipe} wall endpoint action {end}"));
            }
        }
        let recovery = sections.as_array().unwrap().iter().find(|m| m["id"] == member["id"]).unwrap();
        for station in recovery["stations"].as_array().unwrap() {
            let location = station["location"].as_str().unwrap();
            assert!(STATIONS.contains(&location));
            let action = &station["section_action_j_side_local"];
            let what = |q: &str| format!("{pipe} {location} {q}");
            if in_region {
                b.close(row(envelope, "pipe_wall_axial_force_v2", &pipe, Some(location), None, ""), b64(&action["N_w_N"]), force, &what("N_w"));
                b.close(row(envelope, "pipe_effective_axial_force_v2", &pipe, Some(location), None, ""), b64(&member["effective_axial_force_S_N"]), force, &what("S"));
            }
            // The element_local_* force and moment rows at end_i and end_j are
            // the element-local end-DOF force vectors (the node's action on
            // the member, as each row's sign convention states); the interior
            // stations are the j-side section action. At end_j the two
            // coincide; at end_i the j-side action is the negated end action
            // (checked here on the reference itself).
            let end = match location { "end_i" => Some("from"), "end_j" => Some("to"), _ => None };
            let mut actions = vec![("V_y_N", "Fy_N", "element_local_shear_force_y", force), ("V_z_N", "Fz_N", "element_local_shear_force_z", force),
                ("T_Nm", "Mx_Nm", "element_local_torsional_moment", moment), ("M_y_Nm", "My_Nm", "element_local_bending_moment_y", moment),
                ("M_z_Nm", "Mz_Nm", "element_local_bending_moment_z", moment)];
            if !in_region {
                actions.insert(0, ("N_w_N", "Fx_N", "element_local_axial_force", force));
            }
            for (key, end_key, kind, scale) in actions {
                let expected = match end {
                    Some(end) => {
                        let end_action = b64(&member["end_wrench_on_member_local"][end][end_key]);
                        let sign = if end == "from" { -1.0 } else { 1.0 };
                        assert_eq!(sign * end_action, b64(&action[key]), "{label} {pipe} {location} {key}: reference j-side vs end action");
                        end_action
                    }
                    None => b64(&action[key]),
                };
                b.close(row(envelope, kind, &pipe, Some(location), None, ""), expected, scale, &what(key));
            }
            for basis in BASES {
                let stress = &station["stress"][basis];
                let normal = b64(&ms["normal_stress"][basis]);
                let shear = b64(&ms["shear_stress"][basis]);
                let what = |q: &str| format!("{pipe} {location} [{basis}] {q}");
                if in_region {
                    b.close(row(envelope, "pipe_axial_membrane_stress_v2", &pipe, Some(location), None, basis), b64(&stress["axial_membrane_Pa"]), normal, &what("membrane (Pa)"));
                } else {
                    b.close(1e6 * row(envelope, "element_local_axial_normal_stress", &pipe, Some(location), None, basis), b64(&stress["axial_membrane_Pa"]), normal, &what("axial normal (MPa)"));
                }
                for (key, kind, component, scale) in [
                    ("bending_normal_y_Pa", "element_local_bending_normal_stress_y", None, normal),
                    ("bending_normal_z_Pa", "element_local_bending_normal_stress_z", None, normal),
                    ("torsional_shear_Pa", "element_local_torsional_shear_stress", None, shear),
                    ("normal_extreme_plus_Pa", "element_local_normal_stress_extreme", Some("sigma_plus"), normal),
                    ("normal_extreme_minus_Pa", "element_local_normal_stress_extreme", Some("sigma_minus"), normal),
                    ("transverse_shear_maximum_Pa", "element_local_transverse_shear_stress_maximum", None, shear)] {
                    b.close(1e6 * row(envelope, kind, &pipe, Some(location), component, basis), b64(&stress[key]), scale, &what(&format!("{key} (MPa)")));
                }
                if in_region {
                    let key = format!("{}@{}Pa", input["section"].as_str().unwrap(), input["pressure_Pa"].as_str().unwrap());
                    let rows = &references["lame_rows_Pa"][key.as_str()][basis];
                    let scale = b64(&references["package"]["lame_zero_scales_Pa"][key.as_str()][basis]);
                    for (lame_key, kind, component) in LAME {
                        b.close(row(envelope, kind, &pipe, Some(location), Some(component), basis), lame_value(rows, lame_key), scale, &what(lame_key));
                    }
                }
            }
        }
        for basis in BASES {
            let maximum = &recovery["member_maximum_abs_normal_Pa"][basis];
            b.close(row(envelope, "pipe_elastic_normal_stress_maximum_v2", &pipe, None, None, basis), b64(&maximum["value"]), b64(&ms["normal_stress"][basis]), &format!("{pipe} [{basis}] member maximum (Pa)"));
        }
    }
    (b.count, b.worst)
}

fn stress_kind(kind: &str) -> bool {
    semantic_contract::pressure_rows_contract()["rows"].as_array().unwrap().iter()
        .any(|r| r["kind"] == kind && r["source_physical_semantic_dimension"] == "stress")
}

/// every stress row of the body, and only its,
/// carries the reference-section record; per-member body maxima remain; no
/// headline lies on the body.
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
    assert!(envelope["results"].as_array().unwrap().iter()
        .any(|r| r["entity_ref"] == BODY && r["kind"] == "pipe_elastic_normal_stress_maximum_v2" && r["component_body"] == record), "{label}: body per-member maximum");
    for key in ["max_open_formula_stress_nominal_section", "max_open_formula_stress_reduced_section"] {
        let location = envelope["summary"][key]["location_ref"].as_str().unwrap_or_else(|| panic!("{label}: no {key}"));
        assert_ne!(location, BODY, "{label}: {key}");
    }
}

fn reader_ok(envelope: &Value, document: &Value, label: &str) {
    let verdict = open_pipe_stress_product_physics::result_export::reader_verdict::validate("pressure", envelope, Some(&document["model"]), None);
    assert_eq!(verdict["ok"], true, "{label}: {verdict:#?}");
}

/// Each published row's value bits by row ID.
fn values(envelope: &Value) -> BTreeMap<String, u64> {
    envelope["results"].as_array().unwrap().iter()
        .map(|r| (r["id"].as_str().unwrap().to_string(), r["value"].as_f64().unwrap().to_bits()))
        .collect()
}

#[test]
fn every_reference_case_binds_on_the_exact_route_within_its_criterion() {
    let references = references();
    let cases = references["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 27);
    let mut compared = 0;
    let mut worst = (0.0, String::new());
    for case in cases {
        let id = case["label"].as_str().unwrap();
        let scale = ratio(&body_input(case)["stiffness_scale"]);
        let document = document(&references, case, Some(rs_offset(case)));
        for mode in MODES {
            let label = format!("{id} {mode:?}");
            let envelope = solve(&document, mode);
            let (count, used) = check_case(&references, case, &envelope, &label);
            compared += count;
            if used.0 > worst.0 {
                worst = used;
            }
            reader_ok(&envelope, &document, &label);
            check_body_record(&envelope, scale, &label);
            // The RS-R1 offset is W-15c-realized on every case's actual nodes.
            assert_eq!(with_code(&envelope, ECC_REALIZED).len(), 1, "{label}");
            assert!(with_code(&envelope, ECC_AXIS_ALLOWANCE).is_empty(), "{label}");
            // D-1: only the unequal-pressure closure warns, at N and F.
            let differ = with_code(&envelope, PRESSURES_DIFFER);
            if id == "RS2B-HELD-UNEQ-TIMO-S2" {
                let refs: Vec<&Value> = differ.iter().map(|d| &d["affected_refs"]).collect();
                assert_eq!(refs.len(), 2, "{label}: {differ:#?}");
                assert!(differ.iter().all(|d| d["severity"] == "warning"));
                assert!(refs.contains(&&json!([CASE, "node:N", "region:nbrN", "region:body"])) || refs.contains(&&json!([CASE, "node:N", "region:body", "region:nbrN"])), "{label}: {refs:?}");
            } else {
                assert!(differ.is_empty(), "{label}: {differ:#?}");
            }
            assert!(with_code(&envelope, OUTSIDE_REGIONS).is_empty(), "{label}");
        }
    }
    eprintln!("RT-A3 2b bindings: {compared} row comparisons over 27 cases x 2 solver modes; largest |error|/tolerance {:e} at {}", worst.0, worst.1);
    assert!(compared > 20_000, "{compared}");
}

/// The offset never enters the mechanics: absent (no declaration, W-15c run
/// evidence still required) and declared give bit-identical rows; on the
/// scale-1 case an unread marker offset gives the same values unscaled.
#[test]
fn absent_and_unread_offsets_admit_the_same_actual_geometry_with_identical_numbers() {
    let references = references();
    for label in ["RS2B-HELD-TIMO-S2", "RS2B-HELD-REV-EB-S2", "RS2B-NONPAR-HELD-TIMO-S2", "RS2B-ROT-HELD-TIMO-S2", "RS2B-DET-COMP-TIMO-S2", "RS2B-C3-JHELD-FREE-TIMO-S2"] {
        let case = case(&references, label);
        let declared = document(&references, case, Some(rs_offset(case)));
        let absent = document(&references, case, None);
        for mode in MODES {
            let (with, without) = (solve(&declared, mode), solve(&absent, mode));
            check_case(&references, case, &without, &format!("{label} absent {mode:?}"));
            assert_eq!(values(&with), values(&without), "{label} {mode:?}");
            assert!(with_code(&without, ECC_REALIZED).is_empty(), "{label}: an absent offset is not checked");
        }
    }
    let case = case(&references, "RS2B-HELD-EB-S1");
    let mut marker = document(&references, case, Some(rs_offset(case)));
    reducer_mut(&mut marker)["mechanics_interface"]["solver_consumption"] = json!("not_solver_consumed");
    for mode in MODES {
        let envelope = solve(&marker, mode);
        check_case(&references, case, &envelope, &format!("marker {mode:?}"));
        assert!(with_code(&envelope, ECC_REALIZED).is_empty(), "an unread offset is not checked");
        assert!(with_code(&envelope, "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED").is_empty());
    }
}

fn model_array<'a>(document: &'a mut Value, key: &str) -> &'a mut Vec<Value> {
    document["model"][key].as_array_mut().unwrap()
}

fn regions_mut(document: &mut Value) -> &mut Vec<Value> {
    document["model"]["load_cases"][0]["pressure_regions"].as_array_mut().unwrap()
}

fn reducer_mut(document: &mut Value) -> &mut Value {
    model_array(document, "components").iter_mut().find(|c| c["id"] == REDUCER).unwrap()
}

fn set_node(document: &mut Value, node: &str, p: [f64; 3]) {
    let found = model_array(document, "nodes").iter_mut().find(|n| n["id"] == node).unwrap_or_else(|| panic!("no {node}"));
    found["position"] = json!({"x": p[0], "y": p[1], "z": p[2]});
}

fn set_offset(document: &mut Value, offset: Option<[f64; 3]>) {
    let geometry = reducer_mut(document)["geometry"].as_object_mut().unwrap();
    match offset {
        Some(e) => {
            geometry.insert("eccentric_offset".into(), json!({"x": e[0], "y": e[1], "z": e[2], "unit": "m"}));
        }
        None => {
            geometry.remove("eccentric_offset");
        }
    }
}

/// Removes a neighbour member with its far node, its region and the far node's support.
fn remove_neighbour(document: &mut Value, pipe: &str, far: &str) {
    model_array(document, "pipe_segments").retain(|p| p["id"] != pipe);
    model_array(document, "nodes").retain(|n| n["id"] != far);
    model_array(document, "supports").retain(|s| s["node"] != far);
    if let Some(regions) = document["model"]["load_cases"][0]["pressure_regions"].as_array_mut() {
        regions.retain(|r| r["member_pipe_ids"] != json!([pipe]));
    }
}

/// The same document on the pressure-free route: W-15c's own (PF) semantics.
fn preview(document: &Value) -> Value {
    let mut document = document.clone();
    let model = document["model"].as_object_mut().unwrap();
    model.remove("pressure_contract");
    // RT-L2: pressure elongation has no meaning without a pressure contract.
    model.remove("model_settings");
    model["materials"] = json!([{"id": "material:rt-a3-2b", "elastic_modulus": quantity(2.0e11, "Pa"), "shear_modulus": quantity(2.0e11 / 2.6, "Pa"),
        "thermal_expansion_coefficient": quantity(1.2e-5, "1/degC"), "provenance": PROVENANCE}]);
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        let section = pipe["section"].as_object_mut().unwrap();
        section.remove("mill_tolerance");
        section.remove("corrosion_allowance");
    }
    let case = model["load_cases"][0].as_object_mut().unwrap();
    case.remove("pressure_regions");
    // The pressure-free route requires an explicit load; W-15c reads none.
    case.insert("primitive_loads".into(), json!([{"id": "load:N:fx", "category": "concentrated_force", "target": {"type": "node", "node": "node:N"},
        "direction": "global_x", "dimension": "force", "magnitude": quantity(1000.0, "N"), "provenance": PROVENANCE}]));
    document
}

/// The expected W-15c outcome as product codes.
fn expected_codes(expected: &str) -> (bool, Vec<&'static str>, Option<String>) {
    match expected {
        "REALIZED" => (true, vec![ECC_REALIZED], None),
        "REALIZED with AXIS_ALLOWANCE_USED warning" => (true, vec![ECC_REALIZED, ECC_AXIS_ALLOWANCE], None),
        "no_check" => (true, vec![], None),
        "MISMATCH" => (false, vec![ECC_MISMATCH], None),
        "NOT_TRANSVERSE" => (false, vec!["RIGID_COMPONENT_ECCENTRIC_OFFSET_NOT_TRANSVERSE"], None),
        "BELOW_PRECISION" => (false, vec!["RIGID_COMPONENT_ECCENTRIC_OFFSET_BELOW_PRECISION"], None),
        other => {
            let reason = other.strip_prefix("INVALID ").unwrap_or_else(|| panic!("unmapped expectation {other}"));
            (false, vec![ECC_INVALID], Some(format!("(reason {reason})")))
        }
    }
}

fn offset_codes(envelope: &Value) -> Vec<&str> {
    diagnostics(envelope).iter().filter_map(|d| d["code"].as_str()).filter(|c| c.starts_with("RIGID_COMPONENT_ECCENTRIC_OFFSET")).collect()
}

/// The RS2B-HELD-EB-S1 document with one W-15c geometry control applied:
/// N, F and the neighbours' far nodes A and C at the control's positions, the
/// control's offset, and its end evidence (a second member at N, another
/// reducer's body, an unrealized bend member, or no member). Returns whether
/// the pressure step's topology refuses it before W-15c is read.
fn control_document(references: &Value, control: &Value) -> (Value, bool) {
    let mut document = document(references, case(references, "RS2B-HELD-EB-S1"), None);
    let inputs = &control["inputs"];
    set_node(&mut document, "node:N", vec3(&inputs["x_N"]));
    set_node(&mut document, "node:F", vec3(&inputs["x_F"]));
    // A body along the members' y_reference (G-NO-AXIAL-LENGTH, across the
    // run) takes another reference direction; it enters no W-15c quantity.
    let (n, f) = (vec3(&inputs["x_N"]), vec3(&inputs["x_F"]));
    if n[0] == f[0] && n[1] == f[1] {
        let body = model_array(&mut document, "pipe_segments").iter_mut().find(|p| p["id"] == BODY).unwrap();
        body["y_reference"] = json!({"x": 1.0, "y": 0.0, "z": 0.0});
    }
    set_offset(&mut document, inputs["declared_offset_m"].as_array().map(|_| vec3(&inputs["declared_offset_m"])));
    let nonzero_offset = inputs["declared_offset_m"].as_array().is_some_and(|e| e.iter().any(|v| ratio(v) != 0.0));
    let mut topology_refuses = false;
    for (end, pipe, far) in [("N", "pipe:nbrN", "node:A"), ("F", "pipe:nbrF", "node:C")] {
        let evidence = inputs["ends"][end].as_array().unwrap();
        match evidence.as_slice() {
            [] => {
                remove_neighbour(&mut document, pipe, far);
                topology_refuses = true;
            }
            [first, rest @ ..] => {
                set_node(&mut document, far, vec3(&first[1]));
                match first[0].as_str().unwrap() {
                    "run" => {}
                    "reducer_body" => {
                        model_array(&mut document, "components").push(json!({"id": "component:other-reducer", "kind": "reducer", "node": far,
                            "geometry": {"rigid_pipe_ref": pipe}, "modifiers": {"stiffness_scaling_user_value": quantity(1.0, "1"), "source_reference": "invented"},
                            "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROVENANCE}));
                        topology_refuses |= nonzero_offset;
                    }
                    "bend_unrealized" => {
                        model_array(&mut document, "components").push(json!({"id": "component:bend", "kind": "bend", "node": far,
                            "geometry": {"bend_pipe_ref": pipe}, "provenance": PROVENANCE}));
                        topology_refuses = true;
                    }
                    other => panic!("unmapped end evidence {other}"),
                }
                for (k, extra) in rest.iter().enumerate() {
                    let node = format!("node:{end}-extra-{k}");
                    model_array(&mut document, "nodes").push(json!({"id": node, "position": {"x": vec3(&extra[1])[0], "y": vec3(&extra[1])[1], "z": vec3(&extra[1])[2]}, "provenance": PROVENANCE}));
                    let mut member = document["model"]["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == pipe).unwrap().clone();
                    member["id"] = json!(format!("pipe:{end}-extra-{k}"));
                    member["from"] = json!(format!("node:{end}"));
                    member["to"] = json!(node);
                    member["y_reference"] = json!({"x": 0.0, "y": 0.0, "z": 1.0});
                    model_array(&mut document, "pipe_segments").push(member);
                    topology_refuses = true;
                }
            }
        }
    }
    if model_array(&mut document, "supports").is_empty() {
        model_array(&mut document, "supports").push(json!({"id": "support:N", "node": "node:N", "family": "anchor",
            "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE}));
    }
    (document, topology_refuses)
}

fn assert_reducer_refused(envelope: &Value, label: &str) {
    assert_ne!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}");
    let found: Vec<&Value> = with_code(envelope, REDUCER_IN_REGION).into_iter().filter(|d| d["affected_refs"][2] == REDUCER).collect();
    assert_eq!(found.len(), 1, "{label}: {:#?}", blocking(envelope));
    assert_eq!(found[0]["severity"], "blocking", "{label}");
    assert_eq!(found[0]["affected_refs"], json!([CASE, "region:body", REDUCER, BODY]), "{label}");
    assert!(found[0]["message"].as_str().unwrap().starts_with(REDUCER_TEXT_HEAD), "{label}");
    assert_eq!(envelope["results"], json!([]), "{label}");
}

/// The 24 cleared W-15c controls, each on the pressure-free route (W-15c's
/// existing verdict, unchanged) and in pressure as a reducer step: where the
/// step topology holds, W-15c's verdict alone decides (an absent offset is
/// not checked and the run evidence still has to pass); where the topology
/// refuses first (a free or branched end, a bend neighbour, another reducer's
/// body beside an authored nonzero offset), the pressure refusal is named and
/// the PF verdict stays W-15c's.
#[test]
fn the_w15c_geometry_controls_decide_the_eccentric_step() {
    let references = references();
    let controls = references["geometry_admission_controls_w15c"].as_array().unwrap();
    assert_eq!(controls.len(), 24);
    let mut admitted = 0;
    for control in controls {
        let name = control["control"].as_str().unwrap();
        let expected = control["expected_by_design"].as_str().unwrap();
        assert_eq!(control["exact_decimal_inputs"]["outcome"].as_str().unwrap().split(" (").next().unwrap(), expected.split(" (").next().unwrap(), "{name}");
        let (solves, codes, reason) = expected_codes(expected);
        let (document, topology_refuses) = control_document(&references, control);
        for mode in MODES {
            let label = format!("{name} {mode:?}");
            // W-15c's own verdict on the pressure-free route.
            let pf = solve(&preview(&document), mode);
            assert_eq!(offset_codes(&pf), codes, "{label} PF: {:#?}", blocking(&pf));
            if let Some(reason) = &reason {
                assert!(with_code(&pf, ECC_INVALID).iter().any(|d| d["message"].as_str().unwrap().contains(reason.as_str())), "{label} PF {reason}");
            }
            if solves {
                assert_eq!(pf["status"]["mechanics"], "MECHANICS_SOLVED", "{label} PF: {:#?}", blocking(&pf));
            } else {
                assert_ne!(pf["status"]["mechanics"], "MECHANICS_SOLVED", "{label} PF");
            }
            // In pressure.
            let exact = solve(&document, mode);
            if topology_refuses {
                assert_reducer_refused(&exact, &format!("{label} EX topology"));
                assert!(offset_codes(&exact).is_empty(), "{label}: refused on topology before W-15c is read");
            } else if solves {
                assert_eq!(exact["status"]["mechanics"], "MECHANICS_SOLVED", "{label} EX: {:#?}", blocking(&exact));
                assert!(with_code(&exact, REDUCER_IN_REGION).is_empty(), "{label}");
                assert_eq!(offset_codes(&exact), codes, "{label} EX");
                admitted += 1;
            } else {
                assert_ne!(exact["status"]["mechanics"], "MECHANICS_SOLVED", "{label} EX");
                assert_eq!(offset_codes(&exact), codes, "{label} EX: {:#?}", blocking(&exact));
                assert_eq!(exact["results"], json!([]), "{label} EX");
            }
        }
    }
    // Eleven controls are admitted steps (both modes).
    assert_eq!(admitted, 2 * 11);
}

/// The two ends' agreement bound θ ≤ 1e-3 + β_N + β_F, at its accepted and
/// just-refused sides, with the offset declared, absent and unread (marker):
/// the bound is W-15c's, and no offset state widens it.
#[test]
fn the_axis_agreement_boundary_is_w15cs_for_every_offset_state() {
    let references = references();
    let controls = references["geometry_admission_controls_w15c"].as_array().unwrap();
    for (name, inside) in [("G-AGREE-inside (RS-170 x + 0.0065837 m)", true), ("G-AGREE-outside (RS-170 x + 0.0065838 m)", false)] {
        let control = controls.iter().find(|c| c["control"] == name).unwrap();
        let (declared, topology_refuses) = control_document(&references, control);
        assert!(!topology_refuses);
        let mut absent = declared.clone();
        set_offset(&mut absent, None);
        let mut marker = declared.clone();
        reducer_mut(&mut marker)["mechanics_interface"]["solver_consumption"] = json!("not_solver_consumed");
        for mode in MODES {
            for (state, document) in [("declared", &declared), ("absent", &absent), ("marker", &marker)] {
                let label = format!("{name} {state} {mode:?}");
                let envelope = solve(document, mode);
                if inside {
                    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {:#?}", blocking(&envelope));
                } else if state == "declared" {
                    assert!(with_code(&envelope, ECC_INVALID).iter().any(|d| d["message"].as_str().unwrap().contains("(reason run_directions_disagree)")), "{label}");
                    assert!(with_code(&envelope, REDUCER_IN_REGION).is_empty(), "{label}");
                } else {
                    assert_reducer_refused(&envelope, &label);
                }
            }
        }
    }
}

/// Mutants: a new eccentric or slanted geometry cannot reach admission
/// through an absent, zero or unread offset, or with failed W-15c evidence.
#[test]
fn eccentric_geometry_cannot_bypass_w15c() {
    let references = references();
    let controls = references["geometry_admission_controls_w15c"].as_array().unwrap();
    let control = |name: &str| controls.iter().find(|c| c["control"].as_str().unwrap().starts_with(name)).unwrap_or_else(|| panic!("{name}"));
    // Failed run evidence or no axial length with the offset absent, or on a
    // marker: refused in pressure at assembly, never by the legacy branch.
    for name in ["G-SHORT-UNRESOLVABLE", "G-NO-AXIAL-LENGTH", "G-AGREE-outside"] {
        let (mut document, topology_refuses) = control_document(&references, control(name));
        assert!(!topology_refuses);
        set_offset(&mut document, None);
        let mut marker = document.clone();
        set_offset(&mut marker, Some([0.0, 0.0, 0.025]));
        reducer_mut(&mut marker)["mechanics_interface"]["solver_consumption"] = json!("not_solver_consumed");
        for mode in MODES {
            for (state, document) in [("absent", &document), ("marker", &marker)] {
                let envelope = solve(document, mode);
                assert_reducer_refused(&envelope, &format!("{name} {state} {mode:?}"));
                assert!(offset_codes(&envelope).is_empty());
            }
        }
    }
    // Another reducer's body beside an absent-offset slanted body: no run
    // evidence, so the W-15c branch refuses; the slant fails the legacy guard.
    // The other reducer's body continues past A into its own region, so its
    // own step topology holds and both are judged at assembly.
    let (mut unsupported, _) = control_document(&references, control("G-UNSUPPORTED (other"));
    set_offset(&mut unsupported, None);
    model_array(&mut unsupported, "nodes").push(json!({"id": "node:A2", "position": {"x": 6.0, "y": 24.0, "z": 6.975}, "provenance": PROVENANCE}));
    let mut far = unsupported["model"]["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == "pipe:nbrN").unwrap().clone();
    far["id"] = json!("pipe:far");
    far["from"] = json!("node:A");
    far["to"] = json!("node:A2");
    model_array(&mut unsupported, "pipe_segments").push(far);
    let mut region = regions_mut(&mut unsupported).iter().find(|r| r["id"] == "region:nbrN").unwrap().clone();
    region["id"] = json!("region:far");
    region["member_pipe_ids"] = json!(["pipe:far"]);
    region["terminals"][0]["node_ref"] = json!("node:A");
    region["terminals"][1]["node_ref"] = json!("node:A2");
    regions_mut(&mut unsupported).push(region);
    for mode in MODES {
        let envelope = solve(&unsupported, mode);
        assert_reducer_refused(&envelope, &format!("reducer-body neighbour, absent offset {mode:?}"));
        assert!(with_code(&envelope, REDUCER_IN_REGION).iter().any(|d| d["affected_refs"][2] == "component:other-reducer"), "{mode:?}");
    }
    // A zero offset on the RS-R1 eccentric geometry is a W-15c mismatch, not
    // a concentric declaration; so is a 25 mm offset on concentric geometry.
    let rs = case(&references, "RS2B-HELD-TIMO-S2");
    let zero = document(&references, rs, Some([0.0, 0.0, 0.0]));
    let mut concentric = document(&references, rs, Some([0.0, 0.0, 0.025]));
    for node in ["node:F", "node:C"] {
        let p = model_array(&mut concentric, "nodes").iter().find(|n| n["id"] == node).unwrap()["position"].clone();
        set_node(&mut concentric, node, [p["x"].as_f64().unwrap(), p["y"].as_f64().unwrap(), 6.975]);
    }
    for mode in MODES {
        for (label, document) in [("zero offset on eccentric geometry", &zero), ("25 mm offset on concentric geometry", &concentric)] {
            let envelope = solve(document, mode);
            assert_ne!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}");
            assert_eq!(offset_codes(&envelope), vec![ECC_MISMATCH], "{label} {mode:?}");
            assert!(with_code(&envelope, REDUCER_IN_REGION).is_empty(), "{label}: refused by W-15c's own diagnostic before assembly");
            assert_eq!(envelope["results"], json!([]), "{label}");
        }
    }
}

/// Where both proofs apply (concentric RS-like geometry: the legacy 2a guard
/// and W-15c's run evidence with a zero offset), they agree on admission, and
/// the absent, zero and signed-zero offsets give bit-identical rows.
#[test]
fn both_branches_agree_where_both_apply() {
    let references = references();
    let rs = case(&references, "RS2B-HELD-TIMO-S2");
    let mut base = document(&references, rs, None);
    for node in ["node:F", "node:C"] {
        let p = model_array(&mut base, "nodes").iter().find(|n| n["id"] == node).unwrap()["position"].clone();
        set_node(&mut base, node, [p["x"].as_f64().unwrap(), p["y"].as_f64().unwrap(), 6.975]);
    }
    for mode in MODES {
        let plain = solve(&base, mode);
        assert_eq!(plain["status"]["mechanics"], "MECHANICS_SOLVED", "{:#?}", blocking(&plain));
        for (label, offset) in [("zero", [0.0, 0.0, 0.0]), ("signed zero", [-0.0, 0.0, -0.0])] {
            let mut document = base.clone();
            set_offset(&mut document, Some(offset));
            let envelope = solve(&document, mode);
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}");
            assert_eq!(offset_codes(&envelope), vec![ECC_REALIZED], "{label}");
            assert_eq!(values(&envelope), values(&plain), "{label} {mode:?}");
        }
        // The concentric step carries no bending: its caps are collinear.
        for pipe in ["pipe:nbrN", BODY, "pipe:nbrF"] {
            for location in STATIONS {
                assert!(row(&plain, "element_local_bending_moment_y", pipe, Some(location), None, "").abs() < 1e-6, "{pipe} {location}");
                assert!(row(&plain, "element_local_bending_moment_z", pipe, Some(location), None, "").abs() < 1e-6, "{pipe} {location}");
            }
        }
    }
}

/// RS2B-HELD-TIMO-S2 made concentric: N, F and C on A's line (z = 6.975).
fn concentric_rs(references: &Value, offset: Option<[f64; 3]>) -> Value {
    let mut document = document(references, case(references, "RS2B-HELD-TIMO-S2"), offset);
    for node in ["node:F", "node:C"] {
        let p = model_array(&mut document, "nodes").iter().find(|n| n["id"] == node).unwrap()["position"].clone();
        set_node(&mut document, node, [p["x"].as_f64().unwrap(), p["y"].as_f64().unwrap(), 6.975]);
    }
    document
}

/// The legacy 2a branch is unchanged and still decides where W-15c cannot:
/// back-to-back bodies (absent offsets; each body's neighbour at N is the
/// other reducer's body, never W-15c run evidence). The 64*epsilon guard on
/// the normalized directions admits a 4-ulp rise of F and C (cross norm about
/// 9e-15) and refuses a 1e-12 m rise (about 2.5e-12), for both bodies.
#[test]
fn the_legacy_guard_still_decides_where_w15c_cannot() {
    let references = references();
    let mut base = concentric_rs(&references, None);
    model_array(&mut base, "components").push(json!({"id": "component:other-reducer", "kind": "reducer", "node": "node:A",
        "geometry": {"rigid_pipe_ref": "pipe:nbrN"}, "modifiers": {"stiffness_scaling_user_value": quantity(1.0, "1"), "source_reference": "invented"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROVENANCE}));
    model_array(&mut base, "nodes").push(json!({"id": "node:A2", "position": {"x": 6.0, "y": 24.0, "z": 6.975}, "provenance": PROVENANCE}));
    let mut far = base["model"]["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == "pipe:nbrN").unwrap().clone();
    far["id"] = json!("pipe:far");
    far["from"] = json!("node:A");
    far["to"] = json!("node:A2");
    model_array(&mut base, "pipe_segments").push(far);
    let mut region = regions_mut(&mut base).iter().find(|r| r["id"] == "region:nbrN").unwrap().clone();
    region["id"] = json!("region:far");
    region["member_pipe_ids"] = json!(["pipe:far"]);
    region["terminals"][0]["node_ref"] = json!("node:A");
    region["terminals"][1]["node_ref"] = json!("node:A2");
    regions_mut(&mut base).push(region);
    let within = f64::from_bits(6.975f64.to_bits() + 4);
    for (z, admitted) in [(within, true), (6.975 + 1.0e-12, false)] {
        let mut document = base.clone();
        for node in ["node:F", "node:C"] {
            let p = model_array(&mut document, "nodes").iter().find(|n| n["id"] == node).unwrap()["position"].clone();
            set_node(&mut document, node, [p["x"].as_f64().unwrap(), p["y"].as_f64().unwrap(), z]);
        }
        for mode in MODES {
            let label = format!("back-to-back rise to z = {z:e} {mode:?}");
            let envelope = solve(&document, mode);
            if admitted {
                assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {:#?}", blocking(&envelope));
                assert!(with_code(&envelope, REDUCER_IN_REGION).is_empty(), "{label}");
            } else {
                assert_reducer_refused(&envelope, &label);
                assert!(with_code(&envelope, REDUCER_IN_REGION).iter().any(|d| d["affected_refs"][2] == "component:other-reducer"), "{label}");
            }
        }
    }
}

/// W-15c's run axis is undirected, so a folded-back neighbour is run
/// evidence there (PF realizes the offset); the pressure step's pass-through
/// sense refuses it, with the offset declared or absent.
#[test]
fn a_folded_back_neighbour_is_refused_by_the_pass_through_sense() {
    let references = references();
    let rs = case(&references, "RS2B-HELD-EB-S1");
    for offset in [Some(rs_offset(rs)), None] {
        let mut document = document(&references, rs, offset);
        set_node(&mut document, "node:C", [6.0, 22.0, 7.0]);
        for mode in MODES {
            let label = format!("folded back, offset {offset:?} {mode:?}");
            let pf = solve(&preview(&document), mode);
            assert_eq!(pf["status"]["mechanics"], "MECHANICS_SOLVED", "{label} PF: {:#?}", blocking(&pf));
            assert_eq!(offset_codes(&pf), if offset.is_some() { vec![ECC_REALIZED] } else { vec![] }, "{label} PF");
            assert_reducer_refused(&solve(&document, mode), &label);
        }
    }
}

#[path = "common/pressure_elongation.rs"]
mod pressure_elongation;

/// RT-L2 (DEL-04-07): with pressure elongation not
/// included, each pressurised eccentric (or slanted) step is the unbalanced-thrust
/// model: closed terminals carry no pressure load, an omitted cap its member's
/// p·A_i, and every member's N_w exceeds the p = 0 value by its own p·A_i.
#[test]
fn not_included_eccentric_steps_carry_only_the_unbalanced_thrust() {
    let references = references();
    let mut checked = 0;
    for case in references["cases"].as_array().unwrap().iter().filter(|c| pressurized(c)) {
        let document = pressure_elongation::without_elongation(&document(&references, case, Some(rs_offset(case))));
        for mode in MODES {
            let off = pressure_elongation::assert_off_is_the_unbalanced_thrust(&document, mode, &format!("{} {mode:?}", case["label"]));
            pressure_elongation::assert_case_token(&off, CASE, "not_included");
        }
        checked += 1;
    }
    assert!(checked > 0);
}

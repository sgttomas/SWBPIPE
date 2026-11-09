//! RT-L2 (DEL-04-07 `pressure-elongation.md` §7 items 1–2): shared test
//! oracles for the pressure-elongation switch.
//!
//! - [`with_elongation`] names every case with a non-empty pressure region in
//!   `model_settings.pressure_elongation` (the "on" path).
//! - [`assert_physical_identity`] compares every published physical number of a
//!   solve with a committed envelope bit for bit: every result row (by id, with
//!   its kind, entity, unit and value bits) and every number in the summary.
//!   Identity, setting, limitation and evidence bytes are not physical numbers.
//! - [`assert_off_is_the_unbalanced_thrust`] judges a "not included" solve
//!   against an independent model: the same document at p = 0 with an explicit
//!   nodal force for each uncancelled cap at a non-transferring terminal of a
//!   straight member. Motions, reactions, member actions and S must agree, and
//!   N_w must exceed the p = 0 value by p·A_i on every pressurised member
//!   (straight and arc alike). It assumes no beam theory and no particular arc
//!   stiffness: both sides are solved by the product with the same stiffness.
#![allow(dead_code)]
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::f64::consts::PI;

/// The ids of the cases that carry at least one pressure region.
pub fn pressurised_cases(document: &Value) -> Vec<String> {
    document["model"]["load_cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["pressure_regions"].as_array().is_some_and(|regions| !regions.is_empty()))
        .map(|case| case["id"].as_str().unwrap().to_string())
        .collect()
}

/// The document with `model_settings.pressure_elongation` naming `cases`
/// (other settings kept).
pub fn with_cases(document: &Value, cases: &[String]) -> Value {
    let mut document = document.clone();
    let model = document["model"].as_object_mut().unwrap();
    let settings = model.entry("model_settings").or_insert_with(|| json!({}));
    settings["pressure_elongation"] = json!({"operating_load_cases": cases});
    document
}

/// The document with every pressurised case named (the "on" path).
pub fn with_elongation(document: &Value) -> Value {
    let cases = pressurised_cases(document);
    assert!(!cases.is_empty(), "the document has a pressurised case");
    with_cases(document, &cases)
}

/// The document without `model_settings.pressure_elongation` (the default, off).
pub fn without_elongation(document: &Value) -> Value {
    let mut document = document.clone();
    if let Some(settings) = document["model"].get_mut("model_settings").and_then(Value::as_object_mut) {
        settings.remove("pressure_elongation");
        if settings.is_empty() {
            document["model"].as_object_mut().unwrap().remove("model_settings");
        }
    }
    document
}

pub fn solve(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).unwrap()).unwrap()
}

pub fn mode(name: &str) -> PreviewSolverMode {
    PreviewSolverMode::from_wire(name).unwrap()
}

fn numbers(value: &Value, path: String, out: &mut BTreeMap<String, u64>) {
    match value {
        Value::Number(n) => {
            out.insert(path, n.as_f64().unwrap().to_bits());
        }
        Value::Array(items) => items.iter().enumerate().for_each(|(i, v)| numbers(v, format!("{path}[{i}]"), out)),
        Value::Object(map) => map.iter().for_each(|(k, v)| numbers(v, format!("{path}.{k}"), out)),
        _ => {}
    }
}

/// Every published physical number of `actual` equals `expected`'s, bit for bit.
/// Returns the number of rows compared.
pub fn assert_physical_identity(actual: &Value, expected: &Value, label: &str) -> usize {
    assert_eq!(actual["status"]["mechanics"], expected["status"]["mechanics"], "{label}: mechanics status");
    let rows = |envelope: &Value| {
        envelope["results"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| {
                (
                    row["id"].as_str().unwrap().to_string(),
                    (row["kind"].clone(), row["entity_ref"].clone(), row["unit"].clone(), row["value"].as_f64().unwrap().to_bits()),
                )
            })
            .collect::<BTreeMap<_, _>>()
    };
    let (a, e) = (rows(actual), rows(expected));
    assert_eq!(a.len(), actual["results"].as_array().unwrap().len(), "{label}: unique row ids");
    assert_eq!(a.keys().collect::<Vec<_>>(), e.keys().collect::<Vec<_>>(), "{label}: row ids");
    for (id, row) in &e {
        assert_eq!(&a[id], row, "{label}: row {id} (kind, entity, unit, value bits)");
    }
    let (mut sa, mut se) = (BTreeMap::new(), BTreeMap::new());
    numbers(&actual["summary"], "summary".into(), &mut sa);
    numbers(&expected["summary"], "summary".into(), &mut se);
    assert_eq!(sa, se, "{label}: summary numbers");
    // The numerical standing of each case is part of what the numbers mean.
    let standing = |envelope: &Value| {
        envelope["numerical_quality"]["cases"]
            .as_array()
            .map(|cases| cases.iter().map(|c| (c["basis_ref"].clone(), c["solve_quality"].clone())).collect::<Vec<_>>())
    };
    assert_eq!(standing(actual), standing(expected), "{label}: per-case solve quality");
    e.len()
}

/// The per-case `pressure_elongation` evidence of a solve: every region record
/// of the case and its `pressure_rhs_assembly` carry `token`.
pub fn assert_case_token(envelope: &Value, case: &str, token: &str) {
    let regions = envelope["contract_evidence"]["pressure"].as_array().unwrap();
    let mine = regions.iter().filter(|r| r["load_case_id"] == case).collect::<Vec<_>>();
    assert!(!mine.is_empty(), "{case}: region evidence");
    for region in mine {
        assert_eq!(region["pressure_elongation"], token, "{case}: region {}", region["region_id"]);
    }
    let cases = envelope["contract_evidence"]["exact_cases"].as_array().unwrap();
    let entry = cases.iter().find(|c| c["load_case_id"] == case).unwrap_or_else(|| panic!("{case}: exact case"));
    assert_eq!(entry["pressure_rhs_assembly"]["pressure_elongation"], token, "{case}: assembly evidence");
}

fn quantity_m(section: &Value, key: &str) -> f64 {
    match &section[key] {
        Value::Null => 0.0,
        q => {
            assert_eq!(q["unit"], "m", "{key} in metres");
            q["value"].as_f64().unwrap()
        }
    }
}

/// A_i = π r_i², r_i = D/2 − t_r on the pressure bore (t_r = (t − c_m) − c_a).
pub fn bore_area(document: &Value, pipe: &str) -> f64 {
    let pipe = document["model"]["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == pipe).unwrap();
    let section = &pipe["section"];
    let od = quantity_m(section, "outside_diameter");
    let t_r = (quantity_m(section, "wall_thickness") - quantity_m(section, "mill_tolerance")) - quantity_m(section, "corrosion_allowance");
    let r = od * 0.5 - t_r;
    PI * r * r
}

fn position(document: &Value, node: &str) -> [f64; 3] {
    let n = document["model"]["nodes"].as_array().unwrap().iter().find(|n| n["id"] == node).unwrap();
    let p = &n["position"];
    [p["x"].as_f64().unwrap(), p["y"].as_f64().unwrap(), p["z"].as_f64().unwrap()]
}

fn is_arc(document: &Value, pipe: &str) -> bool {
    document["model"]["components"].as_array().into_iter().flatten().any(|c| {
        c["geometry"]["bend_pipe_ref"] == pipe
            && c["mechanics_interface"]["solver_consumption"].as_str().is_some_and(|s| s.starts_with("curved"))
    })
}

/// The independent model of a "not included" case: every region at p = 0, and at
/// each non-transferring terminal the straight member's uncancelled cap
/// ±p·A_i·x̂ (+ at its `from` node, − at its `to` node: the κ = 1 pair's
/// unbalanced end) as an explicit nodal force. Returns the document and, per
/// pressurised member, its p·A_i (the N_w increment).
fn unbalanced_thrust_model(document: &Value) -> (Value, BTreeMap<(String, String), f64>) {
    let mut reference = document.clone();
    let mut increments = BTreeMap::new();
    let cases = reference["model"]["load_cases"].as_array_mut().unwrap();
    for case in cases {
        let case_id = case["id"].as_str().unwrap().to_string();
        let mut forces: Vec<(String, [f64; 3])> = Vec::new();
        for region in case["pressure_regions"].as_array_mut().into_iter().flatten() {
            let p = region["pressure"]["value"].as_f64().unwrap();
            assert_eq!(region["pressure"]["unit"], "Pa");
            let members = region["member_pipe_ids"].as_array().unwrap().iter().map(|m| m.as_str().unwrap().to_string()).collect::<Vec<_>>();
            for member in &members {
                increments.insert((case_id.clone(), member.clone()), p * bore_area(document, member));
            }
            for terminal in region["terminals"].as_array().unwrap() {
                if terminal["closure_transfer"] == "transfers_to_wall" {
                    continue;
                }
                let node = terminal["node_ref"].as_str().unwrap();
                let pipes = document["model"]["pipe_segments"].as_array().unwrap();
                let owner = members
                    .iter()
                    .map(|id| pipes.iter().find(|p| p["id"] == id.as_str()).unwrap())
                    .find(|p| p["from"] == node || p["to"] == node)
                    .expect("the terminal's member");
                let id = owner["id"].as_str().unwrap();
                assert!(!is_arc(document, id), "the oracle states straight terminals only");
                let (a, b) = (position(document, owner["from"].as_str().unwrap()), position(document, owner["to"].as_str().unwrap()));
                let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
                let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                let sign = if owner["from"] == node { 1.0 } else { -1.0 };
                let f = p * bore_area(document, id) * sign;
                forces.push((node.to_string(), [f * d[0] / l, f * d[1] / l, f * d[2] / l]));
            }
            region["pressure"]["value"] = json!(0.0);
        }
        let loads = case.as_object_mut().unwrap().entry("primitive_loads").or_insert_with(|| json!([]));
        for (index, (node, force)) in forces.into_iter().enumerate() {
            for (axis, value) in ["x", "y", "z"].into_iter().zip(force) {
                if value != 0.0 {
                    loads.as_array_mut().unwrap().push(json!({"id": format!("load:uncancelled-cap-{index}-{axis}"),
                        "category": "concentrated_force", "target": {"type": "node", "node": node},
                        "direction": format!("global_{axis}"), "magnitude": {"value": value, "unit": "N"},
                        "dimension": "force", "provenance": "rt_l2_unbalanced_thrust_oracle"}));
                }
            }
        }
    }
    (reference, increments)
}

fn by_id(envelope: &Value) -> BTreeMap<String, &Value> {
    envelope["results"].as_array().unwrap().iter().map(|r| (r["id"].as_str().unwrap().to_string(), r)).collect()
}

fn case_of(row: &Value) -> String {
    row["basis_ref"]["ref_id"].as_str().unwrap_or("").to_string()
}

/// A "not included" solve of `document` (no setting) against its unbalanced-thrust
/// model. Every compared kind agrees to 1e-9 of its own scale (the larger
/// magnitude over both solves, floored by the largest p·A_i for forces), and the
/// straight maxima are no smaller than the published membrane stress.
pub fn assert_off_is_the_unbalanced_thrust(document: &Value, solver_mode: PreviewSolverMode, label: &str) -> Value {
    assert!(document["model"].get("model_settings").and_then(|s| s.get("pressure_elongation")).is_none());
    let off = solve(document, solver_mode);
    assert_eq!(off["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {:?}", off["diagnostics"]);
    let (reference_document, increments) = unbalanced_thrust_model(document);
    let reference = solve(&reference_document, solver_mode);
    assert_eq!(reference["status"]["mechanics"], "MECHANICS_SOLVED", "{label} oracle: {:?}", reference["diagnostics"]);
    let largest_thrust = increments.values().fold(0.0_f64, |m, v| m.max(v.abs()));
    let (off_rows, reference_rows) = (by_id(&off), by_id(&reference));
    let compared = [
        "global_nodal_displacement_",
        "global_nodal_rotation_",
        "support_reaction_component_v2",
        "pipe_effective_axial_force_v2",
        "pipe_wall_axial_force_v2",
        "element_local_bending_moment_",
        "element_local_shear_force_",
        "element_local_torsional_moment",
    ];
    let mut count = 0;
    for prefix in compared {
        let ids = off_rows.keys().filter(|id| off_rows[*id]["kind"].as_str().unwrap().starts_with(prefix)).collect::<Vec<_>>();
        let force = !prefix.starts_with("global_nodal");
        let mut scale = if force { largest_thrust } else { 0.0 };
        for id in &ids {
            let r = reference_rows.get(*id).unwrap_or_else(|| panic!("{label}: oracle row {id}"));
            scale = scale.max(off_rows[*id]["value"].as_f64().unwrap().abs()).max(r["value"].as_f64().unwrap().abs());
        }
        for id in ids {
            let (o, r) = (off_rows[id], reference_rows[id]);
            let mut expected = r["value"].as_f64().unwrap();
            if prefix == "pipe_wall_axial_force_v2" {
                let key = (case_of(o), o["entity_ref"].as_str().unwrap().to_string());
                expected += increments.get(&key).unwrap_or_else(|| panic!("{label}: {id} has no pressurised member"));
            }
            let actual = o["value"].as_f64().unwrap();
            assert!((actual - expected).abs() <= 1e-9 * scale + f64::MIN_POSITIVE,
                "{label}: {id}: off {actual:e} != oracle {expected:e} (scale {scale:e})");
            count += 1;
        }
    }
    assert!(count > 0, "{label}: rows compared");
    // Published maxima agree with the published membrane: never below |N_w/A|.
    for (id, row) in &off_rows {
        if row["kind"] != "pipe_axial_membrane_stress_v2" {
            continue;
        }
        let basis = row["metadata"]["basis"].as_str().unwrap_or("");
        let suffix = if basis.ends_with("reduced_section") { "reduced_section" } else { "nominal_section" };
        let (case, pipe) = (case_of(row), row["entity_ref"].as_str().unwrap().to_string());
        let stem = format!("result:elastic-maximum:{}:{case}:{}:{pipe}:", case.len(), pipe.len());
        let maximum = off_rows
            .values()
            .find(|m| m["kind"] == "pipe_elastic_normal_stress_maximum_v2" && m["id"].as_str().unwrap() == format!("{stem}{suffix}"));
        if let Some(maximum) = maximum {
            let (m, s) = (maximum["value"].as_f64().unwrap(), row["value"].as_f64().unwrap().abs());
            assert!(m >= s * (1.0 - 1e-9), "{label}: {id}: maximum {m:e} below membrane {s:e}");
        }
    }
    off
}

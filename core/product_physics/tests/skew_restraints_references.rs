//! R-6/R-7 S-K1 product cases
//! against the frozen references `validation/references/skew_restraints/references.json`
//! (`product`): the guided cantilever (one Euler-Bernoulli member A–B along +X, anchor at
//! A, rotations held at B) with each case's records at B and a concentrated force at B,
//! in both static solver modes (no sparse-modal claim). Model inputs are the generator's
//! (`generators/skew_reference.py`, `product_cases`), restated here; every expectation is
//! read from the references, never derived from product code.
//!
//! Criterion: |obs − exp| ≤ 1e-9 |exp|; an exact zero uses 1e-9 of the case's largest
//! magnitude.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewSolverMode,
};
use serde_json::{json, Value};

const REFERENCES: &str = include_str!("../../../validation/references/skew_restraints/references.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const PROV: &str = "skew_restraints invented benchmark input; not library, component, material-catalog or code data";
const CASE: &str = "case:skew";
const ALONG: &str = "support_reaction_along_direction_v1";

fn refs() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}

fn hex(text: &str) -> f64 {
    let (negative, body) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let body = body.strip_prefix("0x").unwrap();
    let (mantissa, exponent) = body.split_once('p').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let mut m = u64::from_str_radix(int, 16).unwrap();
    for ch in frac.chars() {
        m = m * 16 + u64::from(ch.to_digit(16).unwrap());
    }
    let mut value = m as f64;
    let mut k = exponent - 4 * frac.len() as i32;
    while k != 0 {
        let step = k.clamp(-1000, 1000);
        value *= 2.0_f64.powi(step);
        k -= step;
    }
    if negative { -value } else { value }
}

fn b64(record: &Value) -> f64 {
    hex(record["binary64"].as_str().unwrap_or_else(|| panic!("binary64 record: {record}")))
}

fn product_case(name: &str) -> Value {
    refs()["product"]["cases"].as_array().unwrap().iter().find(|c| c["name"] == name).unwrap_or_else(|| panic!("case {name}")).clone()
}

/// One record at B: (id, family, restraint token, direction, spring stiffness).
#[derive(Clone)]
struct Record {
    id: &'static str,
    family: &'static str,
    token: &'static str,
    direction: Option<[f64; 3]>,
    spring: bool,
}

fn stop(id: &'static str, d: [f64; 3]) -> Record {
    Record { id, family: "line_stop", token: "UY", direction: Some(d), spring: false }
}
fn spring(id: &'static str, d: [f64; 3]) -> Record {
    Record { id, family: "spring", token: "UY", direction: Some(d), spring: true }
}
fn axis(id: &'static str, token: &'static str) -> Record {
    Record { id, family: "line_stop", token, direction: None, spring: false }
}

fn p(k: i32) -> f64 {
    2.0_f64.powi(k)
}

/// The S-K1 product cases admitted on PF, with the generator's records.
fn s_k1_cases() -> Vec<(&'static str, Vec<Record>)> {
    vec![
        ("P_S1_spring", vec![spring("spring_B", [3.0, 4.0, 0.0])]),
        ("P_S1_spring_scaled_2m600", vec![spring("spring_B", [3.0 * p(-600), 4.0 * p(-600), 0.0])]),
        ("P_S2_spring_pair", vec![spring("spring_1", [3.0, 4.0, 0.0]), spring("spring_2", [-4.0, 3.0, 0.0])]),
        ("P_R1_stop", vec![stop("stop_B", [3.0, 4.0, 0.0])]),
        ("P_R1_stop_scaled_2p600", vec![stop("stop_B", [3.0 * p(600), 4.0 * p(600), 0.0])]),
        ("P_R1_stop_scaled_2m600", vec![stop("stop_B", [3.0 * p(-600), 4.0 * p(-600), 0.0])]),
        ("P_R1_stop_times_3", vec![stop("stop_B", [9.0, 12.0, 0.0])]),
        ("P_R1_stop_negative", vec![stop("stop_B", [-3.0, -4.0, 0.0])]),
        ("P_R3_stop_plane_13", vec![stop("stop_B", [1.0, 3.0, 0.0])]),
        ("P_R4_guide_plus_stop", vec![axis("guide_uz_B", "UZ"), stop("stop_B", [3.0, 4.0, 5.0])]),
        ("P_R4_full_fix", vec![axis("ux_B", "UX"), axis("uz_B", "UZ"), stop("stop_B", [3.0, 4.0, 0.0])]),
        ("P_R4_tiny_restricted_direction", vec![axis("ux_B", "UX"), axis("uz_B", "UZ"), stop("stop_B", [1.0, p(-600), 0.0])]),
        ("P_RS_stop_and_spring", vec![stop("stop_B", [3.0, 4.0, 0.0]), spring("spring_B", [0.0, 3.0, 4.0])]),
    ]
}

fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// The guided cantilever document with `records` at B and the force `load` (N) at B.
fn document(records: &[Record], load: [f64; 3]) -> Value {
    let mut supports = vec![
        json!({"id": "anchor_A", "node": "A", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}),
        json!({"id": "rot_B", "node": "B", "family": "anchor", "restraints": ["RX", "RY", "RZ"], "provenance": PROV}),
    ];
    for r in records {
        let mut s = json!({"id": r.id, "node": "B", "family": r.family, "restraints": [r.token], "provenance": PROV});
        if let Some(d) = r.direction {
            s["direction"] = json!({"x": d[0], "y": d[1], "z": d[2]});
        }
        if r.spring {
            s["stiffness"] = json!({"dof": r.token, "value": qv(1_048_576.0, "N/m")});
        }
        supports.push(s);
    }
    let loads: Vec<Value> = ["global_x", "global_y", "global_z"]
        .iter()
        .zip(load)
        .filter(|(_, v)| *v != 0.0)
        .map(|(direction, v)| json!({"id": format!("load:{direction}"), "category": "concentrated_force", "target": {"type": "node", "node": "B"},
            "direction": direction, "magnitude": qv(v, "N"), "dimension": "force", "provenance": PROV}))
        .collect();
    json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:skew", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "A", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV}, {"id": "B", "position": {"x": 4.0, "y": 0.0, "z": 0.0}, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:AB", "from": "A", "to": "B", "section": {"outside_diameter": qv(0.206, "m"), "wall_thickness": qv(0.011, "m")},
            "material": "material:skew", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": PROV}],
        "supports": supports, "components": [],
        "materials": [{"id": "material:skew", "elastic_modulus": qv(171_798_691_840.0, "Pa"), "shear_modulus": qv(68_719_476_736.0, "Pa"), "provenance": PROV}],
        "load_cases": [{"id": CASE, "primitive_loads": loads, "provenance": PROV}], "combinations": []},
        "materials": []})
}

fn run(doc: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(doc.clone(), mode).unwrap()).unwrap()
}
/// The outputs of every entry that admits `doc`: the value entry and the typed entry
/// agree bytewise; a direction beyond the checked JSON domain (|x| ≥ 2^53 integral,
/// the 2^600 scaling) is refused upstream by the value entry, as today's CE2
/// combination factor is, and reaches the solve only through the typed entry.
fn entries(doc: &Value, mode: PreviewSolverMode, json_domain: bool) -> Vec<Value> {
    let request: LinearStaticPreviewRequest = serde_json::from_value(doc.clone()).unwrap();
    let typed = serde_json::to_value(run_linear_static_preview_with_mode(request, mode)).unwrap();
    let value = run_linear_static_preview_value_with_mode(doc.clone(), mode);
    if json_domain {
        let value = serde_json::to_value(value.unwrap()).unwrap();
        assert_eq!(serde_json::to_string(&value).unwrap(), serde_json::to_string(&typed).unwrap(), "value entry = typed entry");
        vec![value]
    } else {
        let refused = value.map(|_| ()).unwrap_err();
        assert!(refused.starts_with("CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT"), "{refused}");
        vec![typed]
    }
}
fn codes(out: &Value) -> Vec<String> {
    out["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap().to_string()).collect()
}
fn blocking(out: &Value) -> Vec<String> {
    out["diagnostics"].as_array().unwrap().iter()
        .filter(|d| d["severity"] == "blocking" || d["severity"] == "failure")
        .map(|d| format!("{}: {}", d["code"].as_str().unwrap(), d["message"].as_str().unwrap()))
        .collect()
}
fn rows(out: &Value) -> &Vec<Value> {
    out["results"].as_array().unwrap()
}
fn row<'a>(out: &'a Value, kind: &str, entity: &str, component: Option<&str>) -> Option<&'a Value> {
    rows(out).iter().find(|r| {
        r["kind"] == kind && r["entity_ref"] == entity && component.is_none_or(|c| r["metadata"]["component"] == c)
    })
}
fn close(actual: f64, expect: f64, scale: f64, what: &str) {
    let tolerance = 1e-9 * if expect == 0.0 { scale } else { expect.abs() };
    assert!((actual - expect).abs() <= tolerance, "{what}: {actual:e} against {expect:e}");
}

fn load_of(case: &Value) -> [f64; 3] {
    let l = case["load_at_B"].as_array().unwrap();
    [b64(&l[0]), b64(&l[1]), b64(&l[2])]
}

#[test]
fn s_k1_product_cases_match_references_in_both_modes() {
    for (name, records) in s_k1_cases() {
        let reference = product_case(name);
        assert_eq!(reference["slice"], "S-K1", "{name}");
        let expect = &reference["expect"];
        let doc = document(&records, load_of(&reference));
        let u: Vec<f64> = expect["u_B_translations"].as_array().unwrap().iter().map(b64).collect();
        let mut scale = u.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
        let reaction_scale = expect["records"].as_object().unwrap().values()
            .flat_map(|r| r.get("global_force").or_else(|| r.get("components")).and_then(Value::as_array).cloned().unwrap_or_default())
            .map(|v| b64(&v).abs())
            .fold(0.0_f64, f64::max);
        for (mode, out) in MODES.into_iter().flat_map(|mode| entries(&doc, mode, !name.ends_with("2p600")).into_iter().map(move |out| (mode, out))) {
            let what = format!("{name} {mode:?}");
            assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:?}", blocking(&out));
            assert!(blocking(&out).is_empty(), "{what}: {:?}", blocking(&out));
            for (k, axis) in ["x", "y", "z"].iter().enumerate() {
                let observed = row(&out, &format!("global_nodal_displacement_{axis}"), "B", None)
                    .unwrap_or_else(|| panic!("{what}: u_{axis} row"));
                assert_eq!(observed["unit"], "mm");
                let observed = observed["value"].as_f64().unwrap() / 1000.0;
                scale = scale.max(u[k].abs());
                close(observed, u[k], scale, &format!("{what}: u_B,{axis}"));
            }
            for (id, record) in expect["records"].as_object().unwrap() {
                match record["kind"].as_str().unwrap() {
                    "skew_stop" | "skew_spring" => {
                        let g: Vec<f64> = record["global_force"].as_array().unwrap().iter().map(b64).collect();
                        for (k, c) in ["Fx", "Fy", "Fz"].iter().enumerate() {
                            let observed = row(&out, "support_reaction_component_v2", id, Some(c))
                                .unwrap_or_else(|| panic!("{what}: {id} {c}"))["value"].as_f64().unwrap();
                            close(observed, g[k], reaction_scale, &format!("{what}: {id} {c}"));
                        }
                        let along = row(&out, ALONG, id, Some("along_direction")).unwrap_or_else(|| panic!("{what}: {id} along"));
                        close(along["value"].as_f64().unwrap(), b64(&record["along_direction"]), reaction_scale, &format!("{what}: {id} along"));
                        assert_eq!(along["unit"], "N");
                        assert_eq!(along["metadata"]["coordinate_system"], "support_direction");
                        assert_eq!(along["metadata"]["sign_convention"], "support_on_pipe_positive_along_authored_direction");
                    }
                    "rigid_axis_rows" => {
                        let components = record["components"].as_array().unwrap();
                        let names: &[&str] = match id.as_str() {
                            "anchor_A" => &["Fx", "Fy", "Fz", "Mx", "My", "Mz"],
                            "rot_B" => &["Mx", "My", "Mz"],
                            "guide_uz_B" | "uz_B" => &["Fz"],
                            "ux_B" => &["Fx"],
                            other => panic!("{what}: axis record {other}"),
                        };
                        for (value, c) in components.iter().zip(names) {
                            let observed = row(&out, "support_reaction_component_v2", id, Some(c))
                                .unwrap_or_else(|| panic!("{what}: {id} {c}"))["value"].as_f64().unwrap();
                            close(observed, b64(value), reaction_scale, &format!("{what}: {id} {c}"));
                        }
                        assert!(row(&out, ALONG, id, None).is_none(), "{what}: no along row for an axis record");
                    }
                    other => panic!("{what}: kind {other}"),
                }
            }
        }
    }
}

/// on the static product path (the modal review asked for every direction
/// consumer): a stop or a spring authored at d·2^600, d·2^-600, d·2^-1020, or as the
/// subnormal pair (2^-1074, 2^-1074, 0) against (1, 1, 0), publishes its twin's result
/// rows and diagnostics bit for bit, in both modes, through the typed entry (2^600 is
/// beyond the value entry's checked JSON domain, as for CE2's factor).
#[test]
fn power_of_two_rescaled_directions_publish_their_twins_rows_bit_for_bit() {
    let tiny = f64::from_bits(1);
    let groups: [Vec<[f64; 3]>; 2] = [
        vec![[3.0, 4.0, 0.0], [3.0 * p(600), 4.0 * p(600), 0.0], [3.0 * p(-600), 4.0 * p(-600), 0.0], [3.0 * p(-1020), 4.0 * p(-1020), 0.0]],
        vec![[1.0, 1.0, 0.0], [tiny, tiny, 0.0]],
    ];
    let typed = |doc: &Value, mode| {
        let request: LinearStaticPreviewRequest = serde_json::from_value(doc.clone()).unwrap();
        serde_json::to_value(run_linear_static_preview_with_mode(request, mode)).unwrap()
    };
    for make in [stop as fn(&'static str, [f64; 3]) -> Record, spring] {
        for group in &groups {
            for mode in MODES {
                let base = typed(&document(&[make("r_B", group[0])], [0.0, -1024.0, 0.0]), mode);
                assert_eq!(base["status"]["mechanics"], "MECHANICS_SOLVED", "{:?}: {:?}", group[0], blocking(&base));
                for &d in &group[1..] {
                    let twin = typed(&document(&[make("r_B", d)], [0.0, -1024.0, 0.0]), mode);
                    let what = format!("{d:?} {mode:?} against {:?}", group[0]);
                    assert_eq!(twin["status"], base["status"], "{what}: {:?}", blocking(&twin));
                    assert_eq!(serde_json::to_string(&twin["results"]).unwrap(), serde_json::to_string(&base["results"]).unwrap(), "{what}: rows");
                    assert_eq!(serde_json::to_string(&twin["diagnostics"]).unwrap(), serde_json::to_string(&base["diagnostics"]).unwrap(), "{what}: diagnostics");
                }
            }
        }
    }
}

/// an exactly axis-aligned direction is lowered and publishes the
/// same document bytes as the plain record (no along-direction row).
#[test]
fn axis_aligned_twins_are_byte_identical() {
    for (name, with_direction, plain) in [
        ("P_AX_axis_y_direction", Record { id: "stop_B", family: "line_stop", token: "UX", direction: Some([0.0, 2.0, 0.0]), spring: false }, axis("stop_B", "UY")),
        ("P_AX_axis_neg_x_direction", Record { id: "stop_B", family: "line_stop", token: "UX", direction: Some([-1.0, 0.0, 0.0]), spring: false }, axis("stop_B", "UX")),
    ] {
        let reference = product_case(name);
        let load = load_of(&reference);
        for mode in MODES {
            let a = run(&document(&[with_direction.clone()], load), mode);
            let b = run(&document(&[plain.clone()], load), mode);
            assert_eq!(a["status"]["mechanics"], "MECHANICS_SOLVED", "{name}: {:?}", blocking(&a));
            assert_eq!(serde_json::to_string(&a["results"]).unwrap(), serde_json::to_string(&b["results"]).unwrap(), "{name} {mode:?}");
            assert!(row(&a, ALONG, "stop_B", None).is_none());
        }
    }
}

/// Each code of S-K1, named on its object, with the whole document withheld.
#[test]
fn s_k1_refusals_are_named() {
    let refused = |doc: &Value, code: &str, what: &str| {
        for mode in MODES {
            let out = run(doc, mode);
            assert!(codes(&out).iter().any(|c| c == code), "{what} {mode:?}: expected {code}, got {:?}", codes(&out));
            assert!(rows(&out).iter().all(|r| r["kind"] != "support_reaction_component_v2"), "{what}: nothing published");
        }
    };
    let ly = [0.0, -1024.0, 0.0];
    // Item 8 (P_R4_redundant) and item 7 (every P_K2_* case).
    let reference = product_case("P_R4_redundant");
    assert_eq!(reference["expect"]["code"], "SUPPORT_CONSTRAINT_REDUNDANT");
    refused(&document(&[axis("ux_B", "UX"), axis("uy_B", "UY"), stop("stop_B", [3.0, 4.0, 0.0])], ly), "SUPPORT_CONSTRAINT_REDUNDANT", "P_R4_redundant");
    for (name, a, b) in [
        ("P_K2_two_stops_3d", [3.0, 4.0, 0.0], [0.0, 3.0, 4.0]),
        ("P_K2_orthogonal_pair", [3.0, 4.0, 0.0], [-4.0, 3.0, 0.0]),
        ("P_K2_near_parallel", [3.0, 4.0, 0.0], [3.0, 4.0 + p(-48), 0.0]),
        ("P_K2_parallel", [3.0, 4.0, 0.0], [6.0, 8.0, 0.0]),
    ] {
        assert_eq!(product_case(name)["s_k1_refusal"], "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED");
        refused(&document(&[stop("stop_1", a), stop("stop_2", b)], ly), "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED", name);
    }
    // Item 2: a pressure contract.
    let mut exact = document(&[stop("stop_B", [3.0, 4.0, 0.0])], ly);
    exact["model"]["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    refused(&exact, "SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED", "EX");
    // §2.2's order, earliest stage first and one code per object: a skewed record
    // item 1 refuses (a `guide` family) reports its record code and never the EX code.
    let mut guide = exact.clone();
    guide["model"]["supports"][2]["family"] = json!("guide");
    refused(&guide, "SUPPORT_DIRECTION_RECORD_UNSUPPORTED", "EX guide");
    for mode in MODES {
        assert!(!codes(&run(&guide, mode)).iter().any(|c| c == "SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED"), "{mode:?}: one code per object");
    }
    // Item 3: any nonlinear support in the model.
    let mut nonlinear = document(&[stop("stop_B", [3.0, 4.0, 0.0])], ly);
    nonlinear["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "gap_B", "node": "B", "family": "nonlinear", "restraints": [], "provenance": PROV,
        "nonlinear": {"behavior": "gap", "dof": "UZ", "initial_state": "inactive", "closes_when": "positive_displacement", "gap": qv(1000.0, "mm")}}));
    refused(&nonlinear, "SUPPORT_DIRECTION_SKEW_NONLINEAR_MODEL_UNSUPPORTED", "nonlinear");
    // Item 4: §4.2's range rule.
    refused(&document(&[stop("stop_B", [1.0, f64::from_bits(1u64 << 51), 0.0])], ly), "SUPPORT_DIRECTION_RANGE_UNSUPPORTED", "range");
    // Item 6: a skewed record on a node with no element.
    let mut lone = document(&[stop("stop_B", [3.0, 4.0, 0.0])], ly);
    lone["model"]["nodes"].as_array_mut().unwrap().push(json!({"id": "C", "position": {"x": 0.0, "y": 5.0, "z": 0.0}, "provenance": PROV}));
    lone["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "stop_C", "node": "C", "family": "line_stop", "restraints": ["UY"], "direction": {"x": 1.0, "y": 1.0, "z": 0.0}, "provenance": PROV}));
    refused(&lone, "SUPPORT_DIRECTION_NODE_UNCONNECTED", "unconnected");
    // Item 9: a nonzero prescribed translation at a stop's node (P_K3_moving_guide_plus_stop).
    let mut moving = document(&[axis("guide_uz_B", "UZ"), stop("stop_B", [3.0, 4.0, 5.0])], ly);
    moving["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(json!({"id": "motion:uz", "category": "imposed_displacement",
        "target": {"type": "support", "support": "guide_uz_B", "dof": "UZ"}, "direction": "UZ", "magnitude": qv(1.0 / 1024.0, "m"), "dimension": "displacement", "provenance": PROV}));
    refused(&moving, "SUPPORT_SKEW_NODE_PRESCRIBED_MOTION_UNSUPPORTED", "moving guide");
    // Item 5: motion of the skewed record itself (P_K3_stop_ground_meaning_*).
    let mut ground = document(&[stop("stop_B", [3.0, 4.0, 0.0])], ly);
    ground["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(json!({"id": "motion:stop", "category": "imposed_displacement",
        "target": {"type": "support", "support": "stop_B", "dof": "UY"}, "direction": "UY", "magnitude": qv(1.0 / 1024.0, "m"), "dimension": "displacement", "provenance": PROV}));
    refused(&ground, "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED", "skew ground motion");
}

/// RT-A3's condition on hunk 02 (`InvalidInput` is a backstop only): the named
/// codes for an unconnected skewed node (item 6) and a force-scaled skew case
/// (item 10) are what the product reports, never the backstop's "invalid input".
#[test]
fn named_codes_win_over_the_invalid_input_backstop() {
    let backstop = |out: &Value| out["diagnostics"].as_array().unwrap().iter()
        .any(|d| d["message"].as_str().is_some_and(|m| m.contains("invalid input:")));
    let mut lone = document(&[stop("stop_B", [3.0, 4.0, 0.0])], [0.0, -1024.0, 0.0]);
    lone["model"]["nodes"].as_array_mut().unwrap().push(json!({"id": "C", "position": {"x": 0.0, "y": 5.0, "z": 0.0}, "provenance": PROV}));
    lone["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "stop_C", "node": "C", "family": "line_stop", "restraints": ["UY"], "direction": {"x": 1.0, "y": 1.0, "z": 0.0}, "provenance": PROV}));
    // A tip load whose ordinary attempt leaves the binary64 normal range: the W2
    // force-scaled route, which S-K1 does not carry.
    let tiny = document(&[stop("stop_B", [3.0, 4.0, 0.0])], [0.0, -1.0e-307, 0.0]);
    // a coordinate transform product below the split range (P1: a guide UZ
    // and a stop (1, 2^-600, 0) give q = 2^-600 on two free axes, so k·q·q < 2^-969)
    // is refused on the ordinary failure path with the error named.
    let split = document(&[axis("guide_uz_B", "UZ"), stop("stop_B", [1.0, p(-600), 0.0])], [0.0, -1024.0, 0.0]);
    for mode in MODES {
        let out = run(&split, mode);
        assert!(blocking(&out).iter().any(|m| m.contains("restraint_basis: split range in the coordinate transform")), "{mode:?}: {:?}", blocking(&out));
        assert!(!backstop(&out) && !blocking(&out).iter().any(|m| m.contains("InvalidInput")), "{mode:?}: named, not the backstop: {:?}", blocking(&out));
        assert!(row(&out, ALONG, "stop_B", None).is_none());
    }
    for (doc, code) in [(&lone, "SUPPORT_DIRECTION_NODE_UNCONNECTED"), (&tiny, "SUPPORT_DIRECTION_SKEW_FORCE_SCALED_UNSUPPORTED")] {
        for mode in MODES {
            let out = run(doc, mode);
            assert!(codes(&out).iter().any(|c| c == code), "{mode:?}: expected {code}, got {:?}", blocking(&out));
            assert!(!backstop(&out), "{mode:?}: {code} wins over the backstop: {:?}", blocking(&out));
            assert!(row(&out, ALONG, "stop_B", None).is_none(), "{mode:?}: nothing published for the case");
        }
    }
}

/// R-1: the independent reader (`reader_verdict`, RE `along_direction_evidence`) accepts
/// every S-K1 product case's document with its model and refuses the tampered
/// documents by name: a wrong sign, a scaled s, a missing along row, a missing
/// component row, and an along row on a record that is not skewed.
#[test]
fn the_reader_re_checks_the_along_direction_row() {
    use open_pipe_stress_product_physics::result_export::reader_verdict;
    let verdict = |out: &Value, doc: &Value| reader_verdict::validate("preview-physics", out, Some(&doc["model"]), None);
    let refused = |out: &Value, doc: &Value, code: &str, what: &str| {
        let found = verdict(out, doc);
        assert_eq!(found["ok"], false, "{what}: {found}");
        assert!(found.to_string().contains(code), "{what}: expected {code}: {found}");
    };
    for (name, records) in s_k1_cases().into_iter().filter(|(name, _)| !name.ends_with("2p600")) {
        let doc = document(&records, load_of(&product_case(name)));
        for mode in MODES {
            let out = run(&doc, mode);
            assert_eq!(verdict(&out, &doc)["ok"], true, "{name} {mode:?}: {}", verdict(&out, &doc));
        }
    }
    let doc = document(&[stop("stop_B", [3.0, 4.0, 0.0])], [0.0, -1024.0, 0.0]);
    let out = run(&doc, MODES[0]);
    let at = |out: &Value, kind: &str, component: &str| rows(out).iter().position(|r| r["kind"] == kind && r["entity_ref"] == "stop_B" && r["metadata"]["component"] == component).unwrap();
    let along = at(&out, ALONG, "along_direction");
    let mut negated = out.clone();
    negated["results"][along]["value"] = json!(-out["results"][along]["value"].as_f64().unwrap());
    refused(&negated, &doc, "ALONG_DIRECTION_INCONSISTENT", "wrong sign");
    let mut scaled = out.clone();
    scaled["results"][along]["value"] = json!(out["results"][along]["value"].as_f64().unwrap() * (1.0 + 1e-9));
    refused(&scaled, &doc, "ALONG_DIRECTION_INCONSISTENT", "scaled s");
    let mut missing = out.clone();
    missing["results"].as_array_mut().unwrap().remove(along);
    refused(&missing, &doc, "ALONG_DIRECTION_MISSING", "missing along row");
    let mut unexpected = doc.clone();
    unexpected["model"]["supports"][2].as_object_mut().unwrap().remove("direction");
    refused(&out, &unexpected, "ALONG_DIRECTION_UNEXPECTED", "a record that is not skewed");
    let mut no_fy = out.clone();
    let fy = at(&out, "support_reaction_component_v2", "Fy");
    no_fy["results"].as_array_mut().unwrap().remove(fy);
    let found = verdict(&no_fy, &doc);
    assert_eq!(found["ok"], false, "missing component row: {found}");
    // A linear state kind: a mechanics combination 2·case publishes 2s, and the reader
    // (its combination pass re-derives every combined row) accepts it.
    let mut combined = doc.clone();
    combined["model"]["combinations"] = json!([{"id": "C2", "basis": "mechanics", "terms": [{"load_case": CASE, "factor": 2.0}], "provenance": PROV}]);
    for mode in MODES {
        let out = run(&combined, mode);
        let case_s = rows(&out).iter().find(|r| r["kind"] == ALONG && r["basis_ref"]["ref_type"] == "load_case").unwrap()["value"].as_f64().unwrap();
        let combo = rows(&out).iter().find(|r| r["kind"] == ALONG && r["basis_ref"]["ref_type"] == "combination").unwrap_or_else(|| panic!("{mode:?}: combined along row"));
        assert_eq!(combo["value"].as_f64().unwrap().to_bits(), (2.0 * case_s).to_bits(), "{mode:?}");
        assert_eq!(verdict(&out, &combined)["ok"], true, "{mode:?}: {}", verdict(&out, &combined));
    }
}

/// R-1: the committed reader fixture (`fixtures/results/skew_along_direction_preview.json`,
/// P_RS_stop_and_spring with a mechanics combination 2·case, sparse) for the Python
/// verifier's tests (`tests/test_skew_along_direction_reader.py`): the request and its
/// envelope, bytes checked here; `OPENPIPESTRESS_WRITE_SKEW_ALONG_FIXTURE=<path>` rewrites it.
#[test]
fn the_reader_fixture_is_the_product_output() {
    let mut doc = document(&[stop("stop_B", [3.0, 4.0, 0.0]), spring("spring_B", [0.0, 3.0, 4.0])], [0.0, -1024.0, 0.0]);
    doc["model"]["combinations"] = json!([{"id": "C2", "basis": "mechanics", "terms": [{"load_case": CASE, "factor": 2.0}], "provenance": PROV}]);
    let out = run(&doc, PreviewSolverMode::SparseInteractive);
    let text = serde_json::to_string_pretty(&json!({"request": doc, "envelope": out})).unwrap() + "\n";
    if let Ok(path) = std::env::var("OPENPIPESTRESS_WRITE_SKEW_ALONG_FIXTURE") {
        std::fs::write(path, &text).unwrap();
        return;
    }
    assert_eq!(text, include_str!("../../../fixtures/results/skew_along_direction_preview.json"), "rewrite with OPENPIPESTRESS_WRITE_SKEW_ALONG_FIXTURE");
}

/// R-1 desktop (Agent 0's correction 2): the committed combinations fixture
/// (`fixtures/results/skew_along_direction_combinations_preview.json`) for the desktop's
/// along-direction list. P_RS_stop_and_spring with two cases (case:skew, −1024 N along Y;
/// case:b, 512 N along X) and one combination of each PF form: a signed mechanics
/// combination, an absolute sum, an SRSS, a range envelope and a result-state
/// subtraction. Each combination publishes an along-direction row per skewed record with
/// its own basis and sign convention, and the independent reader accepts the document
/// with its model. `OPENPIPESTRESS_WRITE_SKEW_ALONG_COMBINATIONS_FIXTURE=<path>` rewrites it.
#[test]
fn the_combinations_reader_fixture_is_the_product_output() {
    use open_pipe_stress_product_physics::result_export::reader_verdict;
    let mut doc = document(&[stop("stop_B", [3.0, 4.0, 0.0]), spring("spring_B", [0.0, 3.0, 4.0])], [0.0, -1024.0, 0.0]);
    doc["model"]["load_cases"].as_array_mut().unwrap().push(json!({"id": "case:b", "primitive_loads": [{"id": "load:b", "category": "concentrated_force",
        "target": {"type": "node", "node": "B"}, "direction": "global_x", "magnitude": qv(512.0, "N"), "dimension": "force", "provenance": PROV}], "provenance": PROV}));
    let terms = json!([{"load_case": CASE, "factor": 1.0}, {"load_case": "case:b", "factor": 1.0}]);
    doc["model"]["combinations"] = json!([
        {"id": "C_SIGNED", "basis": "mechanics", "terms": [{"load_case": CASE, "factor": 2.0}, {"load_case": "case:b", "factor": -1.0}], "provenance": PROV},
        {"id": "C_ABS", "basis": "absolute_sum", "terms": terms, "provenance": PROV},
        {"id": "C_SRSS", "basis": "srss", "terms": terms, "provenance": PROV},
        {"id": "C_ENV", "basis": "range_envelope", "operand_ids": [CASE, "case:b"], "mode": "max", "provenance": PROV},
        {"id": "C_DIFF", "basis": "result_state_subtraction", "minuend_id": CASE, "subtrahend_id": "case:b", "provenance": PROV},
    ]);
    let out = run(&doc, PreviewSolverMode::SparseInteractive);
    assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{:?}", blocking(&out));
    let verdict = reader_verdict::validate("preview-physics", &out, Some(&doc["model"]), None);
    assert_eq!(verdict["ok"], true, "{verdict}");
    let along_of = |id: &str, basis: &str| -> Vec<&Value> { rows(&out).iter().filter(|r| r["kind"] == ALONG && r["entity_ref"] == id && r["basis_ref"]["ref_id"] == basis).collect() };
    for id in ["stop_B", "spring_B"] {
        // Each case's row under the agreed id, unqualified in the second case too.
        for case in [CASE, "case:b"] {
            let row = along_of(id, case);
            assert_eq!(row.len(), 1, "{case} {id}");
            assert_eq!(row[0]["id"], format!("result:support-along-direction-v1:{}:{case}{}:{id}", case.len(), id.len()), "{case} {id}");
        }
        let (a, b) = (along_of(id, CASE)[0]["value"].as_f64().unwrap(), along_of(id, "case:b")[0]["value"].as_f64().unwrap());
        for combination in ["C_SIGNED", "C_ABS", "C_SRSS", "C_ENV", "C_DIFF"] {
            let along = along_of(id, combination);
            assert_eq!(along.len(), 1, "{combination} {id}: {along:?}");
            assert_eq!(along[0]["id"], format!("result:combination:{combination}:support-action:{}:{id}:along_direction", id.len()), "{combination} {id}");
            let component = rows(&out).iter().find(|r| r["kind"] == "support_reaction_component_v2" && r["entity_ref"] == id && r["basis_ref"]["ref_id"] == combination && r["metadata"]["component"] == "Fx").unwrap();
            // The group's derivation and sign meaning, as its components'.
            assert_eq!(along[0]["metadata"]["basis"], component["metadata"]["basis"], "{combination} {id}");
            assert_eq!(along[0]["metadata"]["sign_convention"], component["metadata"]["sign_convention"], "{combination} {id}");
        }
        assert_eq!(along_of(id, "C_SIGNED")[0]["value"].as_f64().unwrap(), 2.0 * a - b, "{id}: 2a - b");
        assert_eq!(along_of(id, "C_ABS")[0]["value"].as_f64().unwrap(), a.abs() + b.abs(), "{id}: |a| + |b|");
        assert_eq!(along_of(id, "C_DIFF")[0]["value"].as_f64().unwrap(), a - b, "{id}: a - b");
    }
    let text = serde_json::to_string_pretty(&json!({"request": doc, "envelope": out})).unwrap() + "\n";
    if let Ok(path) = std::env::var("OPENPIPESTRESS_WRITE_SKEW_ALONG_COMBINATIONS_FIXTURE") {
        std::fs::write(path, &text).unwrap();
        return;
    }
    assert_eq!(text, include_str!("../../../fixtures/results/skew_along_direction_combinations_preview.json"), "rewrite with OPENPIPESTRESS_WRITE_SKEW_ALONG_COMBINATIONS_FIXTURE");
}

/// Astra B4 (RT-A3's re-check criteria): a skewed stop d = (1.5, 1, 0) at B under the
/// finite force (−1e308, 1e308, 0) has a free-coordinate force 2.5e308 outside binary64. The
/// case is refused on the existing `NumericallyUnresolved` arm with its `restraint_basis:`
/// reason in both modes, never the `InvalidInput` backstop, and the case publishes no rows.
#[test]
fn a_transformed_load_outside_binary64_is_refused_on_the_numerically_unresolved_arm() {
    let doc = document(&[stop("stop_B", [1.5, 1.0, 0.0])], [-1e308, 1e308, 0.0]);
    for mode in MODES {
        // 1e308 is beyond the value entry's checked JSON domain: the typed entry (as CE2).
        let request: LinearStaticPreviewRequest = serde_json::from_value(doc.clone()).unwrap();
        let out = serde_json::to_value(run_linear_static_preview_with_mode(request, mode)).unwrap();
        let blocking: Vec<&Value> = out["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking").collect();
        // One blocking finding: the existing arm's code, with the reason named.
        assert_eq!(blocking.len(), 1, "{mode:?}: {blocking:?}");
        assert_eq!(blocking[0]["code"], "NUMERICAL_INTEGRITY_UNRESOLVED", "{mode:?}");
        let message = blocking[0]["message"].as_str().unwrap();
        assert!(message.contains("NumericallyUnresolved { reason: \"restraint_basis: the transformed load is outside binary64\", global_dof: None }"), "{mode:?}: {message}");
        assert!(!message.contains("invalid input:") && !message.contains("InvalidInput"), "{mode:?}: {message}");
        assert!(rows(&out).iter().all(|r| r["basis_ref"]["ref_id"] != CASE), "{mode:?}: no partial rows for the case");
    }
}

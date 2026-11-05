//! R-6/R-7 S-K1 product cases (DEL-04-03 Work `skew-restraints.md` §13, §15 step 8)
//! against the frozen references `validation/references/skew_restraints/references.json`
//! (`product`): the guided cantilever (one Euler-Bernoulli member A–B along +X, anchor at
//! A, rotations held at B) with each case's records at B and a concentrated force at B,
//! in both static solver modes (no sparse-modal claim). Model inputs are the generator's
//! (`generators/skew_reference.py`, `product_cases`), restated here; every expectation is
//! read from the references, never derived from product code.
//!
//! Criterion (§13): |obs − exp| ≤ 1e-9 |exp|; an exact zero uses 1e-9 of the case's largest
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

/// §7.1 and §4.3: an exactly axis-aligned direction is lowered and publishes the
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

/// Each §2.2 code of S-K1, named on its object, with the whole document withheld.
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

/// RT-A3's condition on hunk 02 (`InvalidInput` is a backstop only): the named §2.2
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
    // §6.2: a coordinate transform product below the split range (P1: a guide UZ
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

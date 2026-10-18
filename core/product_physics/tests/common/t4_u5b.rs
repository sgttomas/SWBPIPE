//! T4-U5b: the model 0.6.0 request builder for the frozen references in
//! `validation/references/t4_u5b/` (its README, "Binding to model 0.6.0").
//!
//! Each case description names its t4_u5a base request and that request's
//! SHA-256. The builder checks the oracle's `case_sha256` of the description
//! and the base's SHA-256, re-expresses the base in model 0.6.0 in memory
//! through the shared rule (`current_model.rs`), and applies the listed
//! changes in 0.6.0's defined rod shape (DEL-04-07 Design
//! `u4b-u5b-implementation.md` §3.1): the hardware with offsets in the
//! connector axes, the added loads, the orientation pattern (connector frame,
//! end attachments and node positions of the named t4_u5a request), and for
//! MO-* the removed region, the unpressurized connector and MO-T's thermal
//! loads from J3-1T. Every physical input is then checked against the case
//! description. No frozen byte is written: the requests exist in memory only.
//!
//! The including crate or test module declares `current_model` (`#[path]`) beside this module.
#![allow(dead_code)]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub const ORACLE: &str = include_str!("../../../../validation/references/t4_u5b/oracle.json");
const PROVENANCE: &str =
    "invented T4-U5b reference fixture (analytical control); not library, catalog, manufacturer or code-rule data";

pub fn oracle() -> Value {
    serde_json::from_str(ORACLE).unwrap()
}

fn references() -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../validation/references")
}

/// The project root relative to the including crate (`core/product_physics`
/// or `core/runner/headless`): the references sit under it either way.
fn reference_root() -> PathBuf {
    let direct = references();
    if direct.join("t4_u5b").exists() {
        return direct;
    }
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../validation/references")
}

fn read_bound(path: PathBuf, sha256: &str) -> Value {
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), sha256, "{}: SHA-256 pin", path.display());
    serde_json::from_slice(&bytes).unwrap()
}

/// The exact binary64 `value` of an `{exact, decimal, value}` number.
pub fn number(v: &Value) -> f64 {
    v["value"].as_f64().unwrap_or_else(|| panic!("not an oracle number: {v}"))
}

fn vector(v: &Value) -> [f64; 3] {
    std::array::from_fn(|i| number(&v[i]))
}

/// The case description of `case` (the oracle entry), SHA-checked.
pub fn description(case: &Value) -> Value {
    read_bound(
        reference_root().join("t4_u5b").join(case["case_description"].as_str().unwrap()),
        case["case_sha256"].as_str().unwrap(),
    )
}

fn quantity(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

fn offset(v: &Value) -> Value {
    let [x, y, z] = vector(v);
    json!({"x": quantity(x, "m"), "y": quantity(y, "m"), "z": quantity(z, "m")})
}

/// The 0.6.0 `hardware` object of a case description's rods (§3.1).
pub fn hardware(description: &Value) -> Value {
    let rods = description["physical_inputs"]["connector"]["hardware"]["rods"].as_array().unwrap();
    json!({"kind": "explicit_axial_rods_v1", "rods": rods.iter().map(|rod| {
        assert_eq!(rod["law"]["kind"], "bilateral_linear");
        json!({
            "id": rod["id"],
            "law": "bilateral_linear",
            "offset_i": offset(&rod["end_i"]["offset_connector_axes_m"]),
            "offset_j": offset(&rod["end_j"]["offset_connector_axes_m"]),
            "stiffness": quantity(number(&rod["law"]["stiffness_n_per_m"]), "N/m"),
            "reference_extension": quantity(number(&rod["law"]["reference_extension_m"]), "m"),
            "source_reference": rod["source_reference"],
        })
    }).collect::<Vec<_>>()})
}

fn by_id<'a>(items: &'a mut Value, id: &str) -> &'a mut Value {
    items.as_array_mut().unwrap().iter_mut().find(|item| item["id"] == id)
        .unwrap_or_else(|| panic!("no item {id}"))
}

/// The model 0.6.0 request of oracle case `name`.
pub fn request(name: &str, case: &Value) -> Value {
    let description = description(case);
    assert_eq!(description["id"], name);
    let derives = &description["derives_from"];
    let u5a = reference_root().join("t4_u5a");
    let base = read_bound(u5a.join(derives["request"].as_str().unwrap()), derives["request_sha256"].as_str().unwrap());
    let mut request = super::current_model::reauthor_request(base);
    let model = &mut request["model"];
    model["project"]["id"] = json!(format!("project:invented-t4-u5b-{}", name.to_ascii_lowercase()));
    if let Some(pattern) = derives.get("orientation_pattern").filter(|p| !p.is_null()) {
        let source = read_bound(u5a.join(pattern["request"].as_str().unwrap()), pattern["request_sha256"].as_str().unwrap());
        let source = &source["model"];
        model["components"][0]["node"] = source["components"][0]["node"].clone();
        for key in ["connector_axes_global", "end_i", "end_j"] {
            model["components"][0]["objective_connector"][key] = source["components"][0]["objective_connector"][key].clone();
        }
        for node in source["nodes"].as_array().unwrap() {
            by_id(&mut model["nodes"], node["id"].as_str().unwrap())["position"] = node["position"].clone();
        }
        for pipe in source["pipe_segments"].as_array().unwrap() {
            by_id(&mut model["pipe_segments"], pipe["id"].as_str().unwrap())["y_reference"] = pipe["y_reference"].clone();
        }
    }
    let inputs = &description["physical_inputs"];
    let connector = &mut model["components"][0]["objective_connector"];
    connector["hardware"] = hardware(&description);
    if inputs["connector"]["pressure_model"]["kind"] == "unpressurized" {
        connector["pressure_model"] = json!({"kind": "unpressurized"});
    }
    let case = &mut model["load_cases"][0];
    if inputs["pressure_region"].is_null() {
        // The exact profile states "no region" as an explicit empty list.
        case["pressure_regions"] = json!([]);
    }
    if let Some(pattern) = derives.get("thermal_pattern").filter(|p| !p.is_null()) {
        let source = read_bound(u5a.join(pattern["request"].as_str().unwrap()), pattern["request_sha256"].as_str().unwrap());
        for thermal in inputs["thermal_loads"].as_array().unwrap() {
            let load = source["model"]["load_cases"][0]["primitive_loads"].as_array().unwrap().iter()
                .find(|load| load["id"] == thermal["id"]).unwrap().clone();
            case["primitive_loads"].as_array_mut().unwrap().push(load);
        }
    }
    for load in inputs["loads"].as_array().into_iter().flatten() {
        let node = load["node"].as_str().unwrap();
        for (key, category, directions, unit, dimension) in [
            ("force_global_n", "concentrated_force", ["global_x", "global_y", "global_z"], "N", "force"),
            ("moment_global_nm", "concentrated_moment", ["rotation_x", "rotation_y", "rotation_z"], "N*m", "moment"),
        ] {
            for (axis, value) in vector(&load[key]).into_iter().enumerate() {
                if value == 0.0 {
                    continue;
                }
                case["primitive_loads"].as_array_mut().unwrap().push(json!({
                    "id": format!("{}:{}", load["id"].as_str().unwrap(), directions[axis]),
                    "category": category,
                    "target": {"type": "node", "node": node},
                    "direction": directions[axis],
                    "magnitude": quantity(value, unit),
                    "dimension": dimension,
                    "provenance": PROVENANCE,
                }));
            }
        }
    }
    check_physical_inputs(&request["model"], inputs, name);
    request
}

/// Every built physical input equals the case description's binary64 value.
fn check_physical_inputs(model: &Value, inputs: &Value, name: &str) {
    for node in inputs["nodes"].as_array().unwrap() {
        let built = model["nodes"].as_array().unwrap().iter().find(|n| n["id"] == node["id"]).unwrap();
        let p = vector(&node["position_m"]);
        for (i, axis) in ["x", "y", "z"].iter().enumerate() {
            assert_eq!(built["position"][axis].as_f64(), Some(p[i]), "{name}: {} position", node["id"]);
        }
    }
    assert_eq!(model["nodes"].as_array().unwrap().len(), inputs["nodes"].as_array().unwrap().len(), "{name}: node count");
    for pipe in inputs["pipes"].as_array().unwrap() {
        let built = model["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == pipe["id"]).unwrap();
        assert_eq!(built["from"], pipe["from"], "{name}");
        assert_eq!(built["to"], pipe["to"], "{name}");
        let y = vector(&pipe["y_reference"]);
        for (i, axis) in ["x", "y", "z"].iter().enumerate() {
            assert_eq!(built["y_reference"][axis].as_f64(), Some(y[i]), "{name}: {} y_reference", pipe["id"]);
        }
        assert_eq!(built["section"]["outside_diameter"]["value"].as_f64(), Some(number(&pipe["outside_diameter_m"])));
        assert_eq!(built["section"]["wall_thickness"]["value"].as_f64(), Some(number(&pipe["wall_thickness_m"])));
    }
    let connector = &model["components"][0]["objective_connector"];
    let expected = &inputs["connector"];
    assert_eq!(model["components"][0]["id"], expected["id"], "{name}");
    assert_eq!(connector["end_i"]["node_ref"], expected["end_i"]["node_ref"], "{name}");
    assert_eq!(connector["end_j"]["node_ref"], expected["end_j"]["node_ref"], "{name}");
    for r in 0..3 {
        for c in 0..3 {
            assert_eq!(connector["connector_axes_global"][r][c].as_f64(), Some(number(&expected["connector_axes_global"][r][c])), "{name}: Q");
        }
    }
    for end in ["end_i", "end_j"] {
        for axis in ["x", "y", "z"] {
            assert_eq!(connector[end]["offset_local"][axis].as_f64(), Some(0.0), "{name}: zero attachment offsets");
        }
    }
    assert_eq!(connector["topology"]["span_ref"], expected["topology"]["span_ref"], "{name}");
    assert_eq!(connector["pressure_model"]["kind"], expected["pressure_model"]["kind"], "{name}");
    let regions = model["load_cases"][0].get("pressure_regions").and_then(Value::as_array).cloned().unwrap_or_default();
    match inputs["pressure_region"].as_object() {
        None => assert!(regions.is_empty(), "{name}: no pressure region"),
        Some(region) => {
            assert_eq!(regions.len(), 1, "{name}");
            assert_eq!(regions[0]["id"], region["id"], "{name}");
            assert_eq!(regions[0]["member_pipe_ids"], region["member_pipe_ids"], "{name}");
            let listed = regions[0].get("member_connector_ids").cloned().unwrap_or(json!([]));
            assert_eq!(listed, region["member_connector_ids"], "{name}: listed connectors");
            assert_eq!(regions[0]["pressure"]["value"].as_f64(), Some(number(&region["pressure_pa"])), "{name}");
        }
    }
    let supports: Vec<&str> = model["supports"].as_array().unwrap().iter().map(|s| s["node"].as_str().unwrap()).collect();
    let expected_supports: Vec<&str> = inputs["supports"].as_array().unwrap().iter().map(|s| s["node"].as_str().unwrap()).collect();
    assert_eq!(supports, expected_supports, "{name}: supports");
}

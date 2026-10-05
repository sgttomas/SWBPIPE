//! I61 U9 G8 (disposable copies only, never committed): pressure refusal with no-pressure
//! success through the actual Direct entry. For each request and mode it records the value
//! route's envelope and the Direct entry's one publication: its bytes, class (`exact`, the
//! value route's bytes; `notice`, those plus anything; `successor`), build status, G-A's
//! refusal (clause and fact) and the private W1 result. Expectations are checked by the
//! records-side summary, not here.
use super::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::Path;

const PROV: &str = "invented_t3_f1b_w2_runtime_input_no_library_data";
const ALL: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];

fn sha(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
fn bytes(e: &MechanicsEnvelope) -> Vec<u8> { serde_json::to_vec(e).unwrap() }

/// Verbatim port of `tests/f1b_w2_runtime.rs::phys_r4` at the PR head.
fn phys_r4(pressurized: bool) -> Value {
    let (a, b) = ("node:section-a", "node:section-b");
    let fixed = |id: &str, node: &str| {
        json!({"id": id, "node": node, "family": "anchor", "restraints": ALL,
               "provenance": "independent_section_geometry_control"})
    };
    let mut supports = vec![fixed("support:section-a", a)];
    let mut loads = Vec::new();
    let mut regions = Vec::new();
    if pressurized {
        supports.push(fixed("support:section-b", b));
        regions.push(json!({"id": "region:source-section", "member_pipe_ids": ["pipe:source-section"],
            "pressure_basis": "internal_differential_zero_external_v1",
            "pressure": {"value": 4.7e-170, "unit": "Pa"},
            "terminals": [
                {"node_ref": a, "closure_transfer": "transfers_to_wall", "provenance": "explicit_test_closure"},
                {"node_ref": b, "closure_transfer": "transfers_to_wall", "provenance": "explicit_test_closure"}],
            "provenance": "independent_geometry_pressure_reference"}));
    } else {
        let tip = f64::from_bits(0x0031fa182c40c60d);
        loads.push(json!({"id": "load:tip-y", "category": "concentrated_force",
            "target": {"type": "node", "node": b}, "direction": "global_y", "dimension": "force",
            "magnitude": {"value": tip, "unit": "N"}, "provenance": PROV}));
        loads.push(
            json!({"id": "load:tip-torque", "category": "concentrated_moment",
            "target": {"type": "node", "node": b}, "direction": "rotation_x", "dimension": "moment",
            "magnitude": {"value": tip, "unit": "N*m"}, "provenance": PROV}),
        );
    }
    json!({"model": {
        "schema_version": "0.3.0", "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "2.0.0", "mode": "exact_straight_pressure_v2"},
        "project": {"id": "project:section-oracle", "units": {"length": "m", "force": "N", "angle": "rad",
            "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": a, "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV},
            {"id": b, "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:source-section", "from": a, "to": b, "section": {
            "outside_diameter": {"value": 4e-77, "unit": "m"},
            "wall_thickness": {"value": 1e-77, "unit": "m"}},
            "material": "material:section", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "provenance": PROV}],
        "materials": [{"id": "material:section", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": {"value": 1.0, "unit": "Pa"},
            "poisson_ratio": {"value": 0.1, "unit": "1"}, "provenance": PROV}],
        "supports": supports, "components": [],
        "load_cases": [{"id": "case:source-section", "primitive_loads": loads, "pressure_regions": regions,
            "provenance": PROV}], "combinations": []
    }, "materials": []})
}

fn milestone() -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

/// The milestone with one pressure region on its first member, in its own (0.1.0) namespace:
/// the only change is the case's `pressure_regions`.
fn milestone_with_pressure() -> Value {
    let mut raw = milestone();
    let pipe = raw["model"]["pipe_segments"][0].clone();
    let (id, from, to) = (pipe["id"].clone(), pipe["from"].clone(), pipe["to"].clone());
    raw["model"]["load_cases"][0]["pressure_regions"] = json!([{"id": "region:i61-u9g8", "member_pipe_ids": [id],
        "pressure_basis": "internal_differential_zero_external_v1",
        "pressure": {"value": 1.0e5, "unit": "Pa"},
        "terminals": [
            {"node_ref": from, "closure_transfer": "transfers_to_wall", "provenance": "explicit_test_closure"},
            {"node_ref": to, "closure_transfer": "transfers_to_wall", "provenance": "explicit_test_closure"}],
        "provenance": "invented_i61_u9g8_pressure_control"}]);
    raw
}

#[test]
#[ignore]
fn zz_i61_u9g8_pressure() {
    let cases = [("milestone", milestone()), ("milestone_with_pressure", milestone_with_pressure()),
        ("phys_r4_pressurized", phys_r4(true)), ("phys_r4_without_pressure", phys_r4(false))];
    let mut rows = Vec::new();
    for (label, raw) in cases {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let value = run_linear_static_preview_value_with_mode(raw.clone(), mode);
            let value_bytes = value.as_ref().map(bytes).ok();
            let (value_status, blocking): (String, Vec<String>) = match &value {
                Ok(e) => (e.status.mechanics.clone(), e.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect()),
                Err(s) => (format!("ERR:{s}"), vec![]),
            };
            let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode);
            let row = match out {
                Err(s) => json!({"label": label, "mode": mode.as_str(), "direct": format!("ERR:{s}")}),
                Ok(o) => {
                    let profile = o.admission().map(|r| format!("{:?}", r.profile));
                    let refusal = o.admission().and_then(|r| r.law().refusal.map(|x| format!("{x:?}")));
                    let clause = o.admission().and_then(|r| r.law().refusal.map(|x| x.clause().map_or("bound", |c| c.id()).to_string()));
                    let cause = match o.retained() { None => "none".to_string(), Some(Ok(_)) => "successor".into(), Some(Err(f)) => format!("{f:?}") };
                    let successor = o.successor().is_some();
                    let published = match o.into_publication() {
                        RetainedPublication::Successor(v) => serde_json::to_vec(&v).unwrap(),
                        RetainedPublication::Ordinary(e) => bytes(&e),
                    };
                    if successor {   // the published successor, for a value comparison with U1's pinned fixture
                        let dir = std::env::var("I61_U9G8_OUT").expect("I61_U9G8_OUT");
                        std::fs::write(format!("{dir}.successor_{}.json", mode.as_str()), &published).unwrap();
                    }
                    let notices = serde_json::from_slice::<Value>(&published).ok()
                        .and_then(|p| p["diagnostics"].as_array().map(|d| d.iter().filter(|x| x["code"] == "RETAINED_PRECISION_UNAVAILABLE").count()))
                        .unwrap_or(0);
                    let class = if successor { "successor" } else if Some(&published) == value_bytes.as_ref() { "exact" } else { "OTHER" };
                    json!({"label": label, "mode": mode.as_str(), "value_sha": value_bytes.as_ref().map(|b| sha(b)),
                        "value_status": value_status, "value_blocking": blocking,
                        "direct_sha": sha(&published), "class": class, "notices": notices,
                        "profile": profile, "refusal": refusal, "clause": clause, "cause": cause})
                }
            };
            rows.push(row);
        }
    }
    let out = std::env::var("I61_U9G8_OUT").expect("I61_U9G8_OUT");
    std::fs::write(&out, serde_json::to_string_pretty(&rows).unwrap() + "\n").unwrap();
    println!("I61_U9G8_OUT rows={}", rows.len());
}

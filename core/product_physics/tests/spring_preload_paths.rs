//! DEL-04-11 REQ-014 and DEL-04-03 REQ-014: a variable spring hanger's cold
//! load on the two case-force paths that reuse the ordinary case force, the
//! held-status solve (RT-A5 `minuend_final`) and the modal state case
//! (DEL-04-09 §1.2). Each must apply the cold load exactly once.
//!
//! The model is invented. A cantilever of length L along X is anchored at A;
//! at its tip B it carries two one-way supports and two variable spring
//! hangers, each spring a separate record beside its contact:
//!
//! - on UX, one-way NL-X (bearing when its reaction is positive) and spring
//!   SH-X (stiffness k_x, cold load F_x), with an authored force -1.5 F_x;
//! - on UZ, one-way NL-Z and spring SH-Z (k_z, F_z), with -0.5 F_z.
//!
//! If the cold load were counted n times, the bearing reactions would be
//! R_x = 1.5 F_x - n F_x and R_z = 0.5 F_z - n F_z. So n = 0 leaves both
//! bearing, n = 2 lifts both, and only n = 1 gives NL-X bearing and NL-Z
//! lifted. The expectations are closed forms, independent of the product. The
//! straight cantilever's axial and bending responses are uncoupled. With NL-X
//! bearing, u_x = 0 and R_x = 0.5 F_x. With NL-Z lifted, the tip's bending
//! stiffness is k_b = 3EI/L³ (Euler-Bernoulli, rotation free) and
//! u_z = (P_z + F_z)/(k_b + k_z). Criterion: |obs - exp| <= 1e-9 |exp|.
use open_pipe_stress_product_physics::{modal_experimental_result_document, run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const PROV: &str = "invented_spring_preload_paths";
const L: f64 = 4.0;
const OD: f64 = 0.12;
const WALL: f64 = 0.01;
const E: f64 = 2.0e11;
const F_X: f64 = 1000.0;
const F_Z: f64 = 800.0;
const K_X: f64 = 2.0e5;
const K_Z: f64 = 1.0e5;
const COMB: &str = "comb:o-s-held";
/// O's added lift at the tip, as a fraction of F_z.
const EXTRA: f64 = 0.25;

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}
fn force(id: &str, direction: &str, value: f64) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": "node:b"}, "direction": direction,
        "magnitude": q(value, "N"), "dimension": "force", "provenance": PROV})
}
fn one_way(id: &str, dof: &str) -> Value {
    json!({"id": id, "node": "node:b", "family": "nonlinear", "restraints": [], "provenance": PROV,
        "nonlinear": {"behavior": "one_way", "dof": dof, "initial_state": "active", "active_when": "positive_reaction"}})
}
fn hanger(id: &str, dof: &str, k: f64, cold: f64) -> Value {
    json!({"id": id, "node": "node:b", "family": "variable_spring_hanger", "restraints": [dof], "provenance": PROV,
        "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": dof, "value": q(k, "N/m")},
            "installed_load": q(cold, "N"), "cold_load": q(cold, "N"), "hot_load": q(cold, "N"), "travel_range": q(0.05, "m"),
            "source_reference": "invented"}})
}
/// S: the two authored tip forces. O: S's loads and an added lift of EXTRA·F_z.
fn s_loads(suffix: &str) -> Vec<Value> {
    vec![force(&format!("px-{suffix}"), "global_x", -1.5 * F_X), force(&format!("pz-{suffix}"), "global_z", -0.5 * F_Z)]
}
fn document(combinations: Value, density: bool) -> Value {
    let mut section = json!({"outside_diameter": q(OD, "m"), "wall_thickness": q(WALL, "m")});
    if density {
        section["material_density"] = q(7850.0, "kg/m^3");
    }
    let mut o = s_loads("o");
    o.push(force("lift-o", "global_z", EXTRA * F_Z));
    json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:spring-preload-paths", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "node:a", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV}, {"id": "node:b", "position": {"x": L, "y": 0.0, "z": 0.0}, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:a-b", "from": "node:a", "to": "node:b", "section": section, "material": "material:m",
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV}],
        "supports": [
            {"id": "support:a", "node": "node:a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
            one_way("support:NL-X", "UX"), one_way("support:NL-Z", "UZ"),
            hanger("support:SH-X", "UX", K_X, F_X), hanger("support:SH-Z", "UZ", K_Z, F_Z)],
        "components": [],
        "materials": [{"id": "material:m", "elastic_modulus": q(E, "Pa"), "shear_modulus": q(E / 2.6, "Pa"),
            "thermal_expansion_coefficient": q(1.2e-5, "1/degC"), "provenance": PROV}],
        "load_cases": [{"id": "case:S", "primitive_loads": s_loads("s"), "provenance": PROV}, {"id": "case:O", "primitive_loads": o, "provenance": PROV}],
        "combinations": combinations}, "materials": []})
}

fn k_b() -> f64 {
    let inner = OD - 2.0 * WALL;
    let i = std::f64::consts::PI * (OD.powi(4) - inner.powi(4)) / 64.0;
    3.0 * E * i / L.powi(3)
}
fn close(observed: f64, expected: f64, what: &str) {
    assert!(observed.is_finite() && (observed - expected).abs() <= 1e-9 * expected.abs(), "{what}: observed {observed:.17e} expected {expected:.17e}");
}
fn rows(v: &Value) -> &Vec<Value> {
    v["results"].as_array().unwrap()
}
fn metres(row: &Value) -> f64 {
    let scale = match row["unit"].as_str().unwrap() { "mm" => 1e-3, "m" => 1.0, u => panic!("{u}") };
    row["value"].as_f64().unwrap() * scale
}
/// The one row of `kind` at `entity` under `basis`, held or not.
fn row<'v>(v: &'v Value, kind: &str, entity: &str, basis: (&str, &str), held: bool) -> &'v Value {
    let hits: Vec<&Value> = rows(v)
        .iter()
        .filter(|r| {
            r["kind"] == kind && r["entity_ref"] == entity && r["basis_ref"]["ref_type"] == basis.0 && r["basis_ref"]["ref_id"] == basis.1
                && r["id"].as_str().unwrap().contains(":held:") == held
        })
        .collect();
    assert_eq!(hits.len(), 1, "{kind} {entity} {basis:?} held={held}: {hits:#?}");
    hits[0]
}

/// The held-status solve: S solved at O's final status. O's own state is the
/// once-counted one (NL-X bearing, NL-Z lifted), so the held system restrains
/// UX and frees UZ, and S's held tip displacement is u_z = (-0.5 F_z + F_z)/(k_b + k_z)
/// with the cold load counted once (it would be 1.5 F_z/(k_b + k_z) counted
/// twice, and negative not counted). The held NL-X reaction is
/// 1.5 F_x - F_x = 0.5 F_x, and O - S_held is EXTRA·F_z/(k_b + k_z).
#[test]
fn the_held_status_solve_applies_the_cold_load_once() {
    let combination = json!([{"id": COMB, "basis": "result_state_subtraction", "minuend_id": "case:O", "subtrahend_id": "case:S",
        "difference_basis": "operating_minus_sustained", "subtrahend_support_state": "minuend_final", "provenance": PROV}]);
    let wire = document(combination, false);
    let stiffness = k_b() + K_Z;
    let held_uz = 0.5 * F_Z / stiffness;
    let source_uz = (0.5 + EXTRA) * F_Z / stiffness;
    for mode in MODES {
        let what = format!("{mode:?}");
        let v = serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).expect("captured entry")).unwrap();
        assert_eq!(v["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:#}", v["diagnostics"]);
        let records = v["contract_evidence"]["held_status_derivation"].as_array().unwrap_or_else(|| panic!("{what}: {:#}", v["diagnostics"]));
        assert_eq!(records.len(), 1, "{what}");
        assert_eq!(records[0]["outcome"], "admitted", "{what}: {:#}", records[0]);
        let held: Vec<(&str, &str)> = records[0]["held"].as_array().unwrap().iter()
            .map(|h| (h["support_id"].as_str().unwrap(), h["held_state"].as_str().unwrap())).collect();
        assert_eq!(held, [("support:NL-X", "active"), ("support:NL-Z", "inactive")], "{what}: O's once-counted state is held");
        let comb = ("combination", COMB);
        close(metres(row(&v, "global_nodal_displacement_z", "node:b", comb, true)), held_uz, &format!("{what}: held u_z"));
        close(metres(row(&v, "global_nodal_displacement_z", "node:b", ("load_case", "case:O"), false)), source_uz, &format!("{what}: O's own u_z"));
        close(metres(row(&v, "global_nodal_displacement_z", "node:b", comb, false)), source_uz - held_uz, &format!("{what}: O - S_held"));
        let entry = records[0]["held"].as_array().unwrap().iter().find(|h| h["support_id"] == "support:NL-X").unwrap();
        let reaction = f64::from_bits(u64::from_str_radix(entry["inputs"]["reaction_bits"].as_str().unwrap(), 16).unwrap());
        close(reaction, 0.5 * F_X, &format!("{what}: held NL-X reaction"));
    }
}

/// The modal state case: the static case solve the modal path runs at the
/// named state case. Only the once-counted cold load gives NL-X bearing
/// (restrained) and NL-Z lifted (free) at S, which the modal document's
/// supports table and restrained DOFs record.
#[test]
fn the_modal_state_case_applies_the_cold_load_once() {
    let mut request = document(json!([]), true);
    request["mode_count"] = json!(3);
    request["state_case"] = json!("case:S");
    let doc = modal_experimental_result_document(&request);
    assert_eq!(doc["status"], "solved", "{doc:#}");
    assert_eq!(doc["basis"]["state_case"]["case_id"], "case:S", "{:#}", doc["basis"]);
    assert_eq!(doc["basis"]["state_case"]["linearized"], true);
    let supports = doc["tables"]["supports"].to_string();
    let table: Value = doc["tables"]["supports"].clone();
    let found: Vec<(String, String, String)> = table["rows"].as_array().unwrap_or_else(|| panic!("{supports}")).iter()
        .map(|r| {
            let r = r.get("values").unwrap_or(r);
            (r["support_id"].as_str().unwrap().to_string(), r["state"].as_str().unwrap().to_string(), r["treatment"].as_str().unwrap().to_string())
        })
        .collect();
    let expected = [("support:NL-X", "active", "restrained"), ("support:NL-Z", "inactive", "free")];
    assert_eq!(found.len(), 2, "{supports}");
    for (id, state, treatment) in expected {
        assert!(found.contains(&(id.to_string(), state.to_string(), treatment.to_string())), "{id}: {supports}");
    }
}

//! RT-A4 wiring (DEL-05-02 Work `absolute-sum-srss-combinations.md` §7): absolute
//! sums, SRSS and combinations of combinations on both product routes.
//!
//! - **V-5**, the closed form: an invented cantilever along (3, 4, 0)/5 under a
//!   uniform force per length in X, Y and Z in three cases; its absolute sums and
//!   SRSS are read from the frozen references in
//!   `validation/references/rt_a4_combinations/v5/` (exact rationals, an
//!   independent π, no product code), on PF and EX, in both solver modes.
//! - **V-6**, the κ gates: a consuming constant-effort support on PF and a nonzero
//!   installed reference on EX, r1 F1's amplified chain, and the modulus and
//!   resolved-stiffness gates over every load case a nested operand reaches.
//! - **V-7**, nesting: cycles, load-case-first resolution, a declared operating
//!   comparison naming a combination, sustained plus SRSS pairing on PF, a
//!   later-declared operand, a withheld dependant and the result order.
//! - **V-9**, decision OQ-1 in a model with nonlinear supports.
//! - **V-10**, the κ representation chain, through the product.
//!
//! Every published envelope is also read by the product's own reader
//! (`reader_verdict`), with and without its model.
//!
//! Criterion (DEC-024/DEC-026, unchanged): |obs − exp| ≤ 1e-9 |exp|; a zero
//! expectation uses 1e-9 times its family's largest case value. Codes, gates,
//! evidence and texts are compared exactly.
use open_pipe_stress_product_physics::{
    result_export::reader_verdict, run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    LinearStaticPreviewRequest, PreviewSolverMode,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const V5: &str = include_str!("../../../validation/references/rt_a4_combinations/v5/v5_cantilever.json");
const V5_SUMS: &str = include_str!("../../../validation/references/rt_a4_combinations/v5/SHA256SUMS");
const CONNECTOR: &str = include_str!("fixtures/silent_zero/stress_free_connector_empty.request.json");
const LOAD_STATE_CONNECTED: &str = include_str!("../../../fixtures/product_preview/pressure/load_reference/connected.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const PROV: &str = "rt-a4 invented benchmark input; not library, component, material-catalog or code data";
const ABS_SIGN: &str = "unsigned value: the absolute sum of |factor × source result| over explicit user terms; not an equilibrium state";
const SRSS_SIGN: &str = "unsigned value: the square root of the sum of squares of factor × source result over explicit user terms; not an equilibrium state";
const OQ1_SIGN: &str = "positive value follows explicit user linear combination of matching source result sign conventions; a solved state plus derived differences or bounds; not an equilibrium state";
const CE: &str = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE";
const CE_UNSIGNED: &str = "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND";
const CE_MIXED: &str = "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED";
const IR: &str = "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE";
const IR_UNSIGNED: &str = "INSTALLED_REFERENCE_UNSIGNED_COMBINATION_OPERAND";
const WITHHELD_OPERAND: &str = "LOAD_COMBINATION_OPERAND_WITHHELD";
const UNREPRESENTABLE: &str = "COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE";
const NONLINEAR: &str = "NONLINEAR_COMBINATION_REQUIRES_SOLVE";
const T: f64 = 4.9406564584124654e-324; // 2^-1074

// ------------------------------------------------------------------ shared
fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}
fn terms(id: &str, basis: &str, terms: &[(&str, &str, f64)]) -> Value {
    // (kind: "case" | "comb", id, factor)
    json!({"id": id, "basis": basis, "provenance": PROV, "terms": terms.iter().map(|(kind, operand, factor)| if *kind == "case" {
        json!({"load_case": operand, "factor": factor})
    } else {
        json!({"combination": operand, "factor": factor})
    }).collect::<Vec<_>>()})
}
fn sub(id: &str, minuend: &str, subtrahend: &str) -> Value {
    json!({"id": id, "basis": "result_state_subtraction", "minuend_id": minuend, "subtrahend_id": subtrahend, "provenance": PROV})
}
fn range(id: &str, operands: &[&str]) -> Value {
    json!({"id": id, "basis": "range_envelope", "operand_ids": operands, "mode": "max_abs", "provenance": PROV})
}
fn run_value(wire: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap_or_else(|e| panic!("value entry: {e}"))).unwrap()
}
fn run_typed(wire: &Value, mode: PreviewSolverMode) -> Value {
    let request: LinearStaticPreviewRequest = serde_json::from_value(wire.clone()).unwrap();
    serde_json::to_value(run_linear_static_preview_with_mode(request, mode)).unwrap()
}
fn diagnostics(out: &Value) -> &Vec<Value> {
    out["diagnostics"].as_array().unwrap()
}
fn with_code<'a>(out: &'a Value, code: &str) -> Vec<&'a Value> {
    diagnostics(out).iter().filter(|d| d["code"] == code).collect()
}
fn blocking(out: &Value) -> Vec<String> {
    diagnostics(out).iter().filter(|d| d["severity"] == "blocking").map(|d| format!("{}: {}", d["code"], d["message"])).collect()
}
/// A solved envelope that the product reader admits with its model and without it
/// (design §10.3: preview-physics-2 publishes `combination_derivation`, so a document
/// verifies its combinations without its model).
fn read_solved(out: Value, wire: &Value, what: &str) -> Value {
    assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:?}", blocking(&out));
    let pressure = wire["model"].get("pressure_contract").is_some();
    let reader = if pressure { "pressure" } else { "preview-physics" };
    let verdict = reader_verdict::validate(reader, &out, Some(&wire["model"]), None);
    assert_eq!(verdict["ok"], true, "{what}: reader with the model {verdict}");
    let verdict = reader_verdict::validate(reader, &out, None, None);
    assert_eq!(verdict["ok"], true, "{what}: model-free reader {verdict}");
    out
}
fn solved(wire: &Value, mode: PreviewSolverMode, what: &str) -> Value {
    read_solved(run_value(wire, mode), wire, what)
}
fn solved_typed(wire: &Value, mode: PreviewSolverMode, what: &str) -> Value {
    read_solved(run_typed(wire, mode), wire, what)
}
fn rows(out: &Value) -> &Vec<Value> {
    out["results"].as_array().unwrap()
}
fn combination_rows<'a>(out: &'a Value, id: &str) -> Vec<&'a Value> {
    rows(out).iter().filter(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == id && r["kind"] != "combination_modulus_basis_record").collect()
}
fn case_rows<'a>(out: &'a Value, id: &str) -> Vec<&'a Value> {
    rows(out).iter().filter(|r| r["basis_ref"]["ref_type"] == "load_case" && r["basis_ref"]["ref_id"] == id).collect()
}
/// The gate of a combination on either route: PF's `combination_gates` or EX's record.
fn gate(out: &Value, id: &str) -> Value {
    let evidence = &out["contract_evidence"];
    if let Some(gates) = evidence["combination_gates"].as_array() {
        let g = gates.iter().find(|g| g["combination_id"] == id).unwrap_or_else(|| panic!("no gate {id}"));
        return json!({"withheld": g["withheld"], "reason": g["reason"]});
    }
    let record = evidence["exact_combinations"].as_array().unwrap().iter().find(|r| r["combination_id"] == id).unwrap_or_else(|| panic!("no record {id}"));
    record["gate"].clone()
}
fn admitted() -> Value {
    json!({"withheld": false, "reason": null})
}
fn withheld(code: &str) -> Value {
    json!({"withheld": true, "reason": code})
}
fn agrees(obs: f64, exp: f64, zero_scale: f64) -> bool {
    if exp == 0.0 {
        obs.abs() <= 1e-9 * zero_scale
    } else {
        (obs - exp).abs() <= 1e-9 * exp.abs()
    }
}

// ------------------------------------------------------------------ V-5
fn v5() -> Value {
    serde_json::from_str(V5).unwrap()
}

#[test]
fn v5_references_are_the_frozen_bytes() {
    let digest = format!("{:x}", Sha256::digest(V5.as_bytes()));
    assert!(V5_SUMS.lines().any(|l| l == format!("{digest}  v5_cantilever.json")), "v5_cantilever.json does not match SHA256SUMS");
}

fn input(path: &str) -> f64 {
    v5()["inputs"].pointer(path).unwrap()["binary64"].as_f64().unwrap()
}

/// The V-5 cantilever on PF (`exact` false) or EX (`exact` true).
fn v5_model(exact: bool, combinations: Value) -> Value {
    let (l, od, wall, e, nu) = (input("/length_m"), input("/outside_diameter_m"), input("/wall_thickness_m"), input("/elastic_modulus_Pa"), input("/poisson_ratio"));
    let axis = v5()["inputs"]["axis_unit_vector"].as_array().unwrap().iter().map(|c| c["binary64"].as_f64().unwrap()).collect::<Vec<_>>();
    let tip = json!({"x": axis[0] * l, "y": axis[1] * l, "z": axis[2] * l});
    let mut section = json!({"outside_diameter": qv(od, "m"), "wall_thickness": qv(wall, "m")});
    let line = |case: &str, axis: &str| {
        let w = input(&format!("/w_N_per_m/{axis}"));
        json!({"id": format!("load:{case}"), "category": "distributed_force", "target": {"type": "element", "pipe": "pipe:root-tip"},
            "direction": format!("global_{axis}"), "magnitude": qv(w, "N/m"), "dimension": "force_per_length", "provenance": PROV})
    };
    let mut cases: Vec<Value> = v5()["cases"].as_array().unwrap().iter().map(|c| {
        let id = c["id"].as_str().unwrap();
        json!({"id": id, "primitive_loads": [line(id, c["axis"].as_str().unwrap())], "provenance": PROV})
    }).collect();
    let material = if exact {
        section["mill_tolerance"] = qv(0.0, "m");
        section["corrosion_allowance"] = qv(0.0, "m");
        for case in &mut cases {
            case["pressure_regions"] = json!([]);
        }
        json!({"id": "material:v5", "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "elastic_modulus": qv(e, "Pa"), "poisson_ratio": qv(nu, "1"), "provenance": PROV})
    } else {
        json!({"id": "material:v5", "elastic_modulus": qv(e, "Pa"), "shear_modulus": qv(e / (2.0 * (1.0 + nu)), "Pa"), "provenance": PROV})
    };
    let mut model = json!({"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:rt-a4-v5", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "node:root", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV}, {"id": "node:tip", "position": tip, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:root-tip", "from": "node:root", "to": "node:tip", "section": section, "material": "material:v5",
            "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": PROV}],
        "supports": [{"id": "support:root", "node": "node:root", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}],
        "components": [], "materials": [material], "load_cases": cases, "combinations": combinations});
    if exact {
        model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    }
    json!({"model": model, "materials": []})
}

fn v5_combinations() -> Value {
    json!(v5()["combinations"].as_array().unwrap().iter().map(|c| {
        json!({"id": c["id"], "basis": c["basis"], "provenance": PROV, "terms": c["terms"].as_array().unwrap().iter()
            .map(|t| json!({"load_case": t["load_case"], "factor": t["factor"]["binary64"]})).collect::<Vec<_>>()})
    }).collect::<Vec<_>>())
}

/// A published row's value in the reference's units (m, rad, N, N*m).
fn si(row: &Value) -> f64 {
    let value = row["value"].as_f64().unwrap();
    match row["unit"].as_str().unwrap() {
        "mm" => value / 1000.0,
        _ => value,
    }
}

/// The row of one V-5 quantity (by its reference name) among `candidates`.
fn quantity<'a>(candidates: &[&'a Value], name: &str) -> &'a Value {
    let (entity, kind, component): (&str, &str, Option<&str>) = match name {
        "tip_displacement_magnitude" => ("node:tip", "displacement_magnitude", None),
        "anchor_force_magnitude" => ("support:root", "support_reaction_force_magnitude_v2", None),
        "anchor_moment_magnitude" => ("support:root", "support_reaction_moment_magnitude_v2", None),
        _ => {
            let q = v5()["quantities"].as_array().unwrap().iter().find(|q| q["name"] == name).unwrap().clone();
            let entity = if q["entity"] == "node:tip" { "node:tip" } else { "support:root" };
            let token = q["kind_or_component"].as_str().unwrap().to_string();
            let token: &'static str = Box::leak(token.into_boxed_str());
            if entity == "support:root" {
                (entity, "support_reaction_component_v2", Some(token))
            } else {
                (entity, token, None)
            }
        }
    };
    let hits: Vec<&&Value> = candidates.iter().filter(|r| r["entity_ref"] == entity && r["kind"] == kind
        && component.is_none_or(|c| r["metadata"]["component"] == c)).collect();
    assert_eq!(hits.len(), 1, "{name}: {hits:?}");
    hits[0]
}

/// The largest case value of a quantity's family (displacement, rotation, force, moment).
fn family_scale(name: &str) -> f64 {
    let family = |n: &str| if n.starts_with("tip_u") || n == "tip_displacement_magnitude" { "u" } else if n.starts_with("tip_r") { "r" }
        else if n.starts_with("anchor_f") || n == "anchor_force_magnitude" { "f" } else { "m" };
    let mut scale: f64 = 0.0;
    for case in v5()["cases"].as_array().unwrap() {
        for (n, v) in case["unsigned_values"].as_object().unwrap() {
            if family(n) == family(name) {
                scale = scale.max(v["binary64"].as_f64().unwrap());
            }
        }
    }
    scale
}

#[test]
fn v5_three_axis_static_seismic_on_both_routes() {
    let refs = v5();
    for exact in [false, true] {
        let wire = v5_model(exact, v5_combinations());
        for mode in MODES {
            let what = format!("V-5 exact={exact} {mode:?}");
            let out = solved(&wire, mode, &what);
            // Each case reproduces its closed form (unsigned).
            for case in refs["cases"].as_array().unwrap() {
                let published = case_rows(&out, case["id"].as_str().unwrap());
                for (name, value) in case["unsigned_values"].as_object().unwrap() {
                    let expected = value["binary64"].as_f64().unwrap();
                    let observed = si(quantity(&published, name)).abs();
                    assert!(agrees(observed, expected, family_scale(name)), "{what} {} {name}: {observed} vs {expected}", case["id"]);
                }
            }
            for combination in refs["combinations"].as_array().unwrap() {
                let id = combination["id"].as_str().unwrap();
                assert_eq!(gate(&out, id), admitted(), "{what} {id}");
                let published = combination_rows(&out, id);
                for (name, value) in combination["rows"].as_object().unwrap() {
                    let expected = value["binary64"].as_f64().unwrap();
                    let row = quantity(&published, name);
                    let observed = si(row);
                    assert!(observed.to_bits() >> 63 == 0, "{what} {id} {name}: unsigned, never -0.0");
                    assert!(agrees(observed, expected, family_scale(name)), "{what} {id} {name}: {observed} vs {expected}");
                }
                let (basis, sign) = if combination["basis"] == "srss" { ("explicit_user_srss", SRSS_SIGN) } else { ("explicit_user_absolute_sum", ABS_SIGN) };
                for row in &published {
                    // Every row of an unsigned combination states its derivation (review r1
                    // P4): absent metadata fails.
                    let metadata = row["metadata"].as_object().unwrap_or_else(|| panic!("{what} {id}: no metadata on {row}"));
                    // EX's combined stress rows keep their operand's whole basis (§4).
                    if !metadata["basis"].as_str().unwrap().contains("; section_basis=") {
                        assert_eq!(metadata["basis"], basis, "{what} {id}");
                    }
                    assert_eq!(metadata["sign_convention"], sign, "{what} {id}");
                    assert_eq!(row["basis_ref"], json!({"ref_type": "combination", "ref_id": id}));
                }
                // No row of these combinations is withheld or missing.
                for code in ["LOAD_COMBINATION_ROW_WITHHELD", "LOAD_COMBINATION_SOURCE_RESULT_MISSING"] {
                    assert!(with_code(&out, code).iter().all(|d| !d["affected_refs"].as_array().unwrap().iter().any(|r| r == id)), "{what} {id} {code}");
                }
            }
            // SRSS ≤ absolute sum, componentwise and for the magnitudes, from the same rows.
            for name in ["tip_ux", "tip_uy", "tip_rz", "anchor_mz", "tip_displacement_magnitude"] {
                let value = |id: &str| si(quantity(&combination_rows(&out, id), name));
                assert!(value("comb:SRSS") < value("comb:ABS"), "{what} {name}");
            }
        }
    }
}

// ------------------------------------------------------------------ V-6 (PF)
/// The 1 m cantilever of `preview_physics_runtime.rs` with a consuming constant-effort
/// support at its tip, when `ce`.
fn pf_cantilever(ce: bool, cases: Value, combinations: Value) -> Value {
    let mut supports = vec![json!({"id": "support:a", "node": "node:a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV})];
    if ce {
        supports.push(json!({"id": "support:ce", "node": "node:b", "family": "constant_effort_support", "restraints": ["UY"],
            "hanger": {"hanger_type": "constant_effort_support", "constant_load": qv(375.0, "N"), "travel_range": qv(0.05, "m"), "source_reference": "rt_a4_invented"}, "provenance": PROV}));
    }
    json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:rt-a4", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "node:a", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV}, {"id": "node:b", "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:a-b", "from": "node:a", "to": "node:b", "section": {"outside_diameter": qv(0.12, "m"), "wall_thickness": qv(0.01, "m")},
            "material": "material:m", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV}],
        "supports": supports, "components": [],
        "materials": [{"id": "material:m", "elastic_modulus": qv(200e9, "Pa"), "shear_modulus": qv(76.923076923e9, "Pa"),
            "thermal_expansion_coefficient": qv(1.2e-5, "1/degC"),
            "temperature_points": [{"id": "point:hot", "temperature": qv(300.0, "degC"), "elastic_modulus": qv(180e9, "Pa"), "shear_modulus": qv(69.2e9, "Pa")}],
            "provenance": PROV}],
        "load_cases": cases, "combinations": combinations}, "materials": []})
}
fn force_case(id: &str, direction: &str, value: f64) -> Value {
    json!({"id": id, "provenance": PROV, "primitive_loads": [{"id": format!("load:{id}"), "category": "concentrated_force", "target": {"type": "node", "node": "node:b"},
        "direction": direction, "magnitude": qv(value, "N"), "dimension": "force", "provenance": PROV}]})
}
fn pf_cases() -> Value {
    json!([force_case("W1", "global_y", -1000.0), force_case("W2", "global_y", -500.0), force_case("W3", "global_z", 250.0)])
}

#[test]
fn v6_the_constant_effort_gate_reads_the_fixed_load_count() {
    let bound = 2f64.powi(-46);
    let combinations = json!([
        terms("srss-cases", "srss", &[("case", "W1", 1.0), ("case", "W2", 1.0)]),
        sub("d1", "W1", "W2"),
        sub("d2", "W3", "W2"),
        terms("srss-d", "srss", &[("comb", "d1", 1.0), ("comb", "d2", 1.0)]),
        terms("abs-d", "absolute_sum", &[("comb", "d1", 1.0), ("comb", "d2", -0.5)]),
        terms("sus+srss", "mechanics", &[("case", "W2", 1.0), ("comb", "srss-d", 1.0)]),
        range("env-mixed", &["W1", "d1"]),
        terms("srss-env", "srss", &[("comb", "env-mixed", 1.0)]),
        terms("mech-env", "mechanics", &[("comb", "env-mixed", 1.0)]),
        terms("mech-d", "mechanics", &[("comb", "d1", 1.0)]),
        // r1 F1: M = (1 + 2^-46)·W1 (κ within the unit bound), D = M − W2 (κ = 2^-46).
        terms("m-near", "mechanics", &[("case", "W1", 1.0 + bound)]),
        sub("d-near", "m-near", "W2"),
        terms("abs-amplified", "absolute_sum", &[("comb", "d-near", 2f64.powi(46))]),
        terms("mech-amplified", "mechanics", &[("comb", "d-near", 2f64.powi(46))]),
        // A dependant of a withheld combination.
        sub("dep", "srss-cases", "d1"),
    ]);
    let expected = [
        ("srss-cases", withheld(CE_UNSIGNED)),
        ("d1", admitted()),
        ("d2", admitted()),
        ("srss-d", admitted()),
        ("abs-d", admitted()),
        ("sus+srss", admitted()),
        ("env-mixed", admitted()),
        ("srss-env", withheld(CE_UNSIGNED)),
        ("mech-env", withheld(CE_MIXED)),
        ("mech-d", withheld(CE)),
        ("m-near", admitted()),
        ("d-near", admitted()),
        ("abs-amplified", withheld(CE_UNSIGNED)),
        ("mech-amplified", admitted()),
        ("dep", withheld(WITHHELD_OPERAND)),
    ];
    for ce in [true, false] {
        let wire = pf_cantilever(ce, pf_cases(), combinations.clone());
        for mode in MODES {
            let out = solved(&wire, mode, "V-6 PF");
            for (id, gate_expected) in &expected {
                // Without a fixed-load source, only the withheld-operand rule remains, and
                // only for a dependant of a withheld combination (there is none).
                let want = if ce { gate_expected.clone() } else { admitted() };
                assert_eq!(gate(&out, id), want, "ce={ce} {mode:?} {id}");
                assert_eq!(combination_rows(&out, id).is_empty(), want["withheld"] == true, "ce={ce} {mode:?} {id} rows");
            }
            if ce {
                let dep = with_code(&out, WITHHELD_OPERAND);
                assert_eq!(dep.len(), 1);
                assert_eq!(dep[0]["message"], format!("Combination withheld: its operand combination(s) srss-cases ({CE_UNSIGNED}) are withheld, so it is withheld whole."));
                assert_eq!(dep[0]["affected_refs"], json!(["dep", "srss-cases"]));
            }
        }
    }
}

#[test]
fn v6_the_modulus_gate_reads_every_reached_load_case() {
    let mut cases = pf_cases();
    let mut hot = force_case("W4", "global_y", -800.0);
    hot["modulus_basis_ref"] = json!("point:hot");
    cases.as_array_mut().unwrap().push(hot);
    let combinations = json!([
        sub("d4", "W4", "W1"),
        terms("srss-d4", "srss", &[("comb", "d4", 1.0)]),
        terms("abs-d4", "absolute_sum", &[("comb", "d4", 1.0)]),
        terms("mech-d4", "mechanics", &[("comb", "d4", 1.0)]),
        range("env", &["W1", "W4"]),
        terms("srss-env", "srss", &[("comb", "env", 1.0)]),
        terms("srss-equal", "srss", &[("case", "W1", 1.0), ("case", "W2", 1.0)]),
    ]);
    let wire = pf_cantilever(false, cases, combinations);
    for mode in MODES {
        let out = solved(&wire, mode, "V-6 modulus");
        for id in ["d4", "env", "srss-equal"] {
            assert_eq!(gate(&out, id), admitted(), "{id}");
        }
        for id in ["srss-d4", "abs-d4", "mech-d4", "srss-env"] {
            assert_eq!(gate(&out, id), withheld("COMBINATION_MODULUS_BASIS_MIXED"), "{id}");
        }
    }
}

// ------------------------------------------------------------------ V-6 (EX)
fn connector_model(q_ref_x: f64, combinations: Value) -> Value {
    let mut wire: Value = serde_json::from_str(CONNECTOR).unwrap();
    let model = &mut wire["model"];
    let oc = &mut model["components"][0]["objective_connector"];
    if q_ref_x != 0.0 {
        oc["reference_state"] = json!("prestressed");
        oc["q_ref"]["translation"]["x"] = json!(q_ref_x);
    }
    let force = |id: &str, value: f64| json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": "node:b"},
        "direction": "global_y", "magnitude": qv(value, "N"), "dimension": "force", "provenance": PROV});
    model["load_cases"] = json!([
        {"id": "case:1", "primitive_loads": [force("load:1", 1000.0)], "pressure_regions": [], "provenance": PROV},
        {"id": "case:2", "primitive_loads": [force("load:2", -400.0)], "pressure_regions": [], "provenance": PROV},
        {"id": "case:3", "primitive_loads": [force("load:3", 250.0)], "pressure_regions": [], "provenance": PROV},
    ]);
    model["combinations"] = combinations;
    wire
}

#[test]
fn v6_gate_3_reads_the_fixed_load_count_on_the_exact_route() {
    let combinations = json!([
        sub("d", "case:1", "case:2"),
        terms("mech-d", "mechanics", &[("comb", "d", 1.0)]),
        terms("srss-cases", "srss", &[("case", "case:1", 1.0), ("case", "case:3", 1.0)]),
        terms("srss-d", "srss", &[("comb", "d", 1.0)]),
        terms("abs-d", "absolute_sum", &[("comb", "d", 2.0)]),
        terms("sus+srss", "mechanics", &[("case", "case:3", 1.0), ("comb", "srss-d", 1.0)]),
        range("env-mixed", &["case:1", "d"]),
        terms("mech-env", "mechanics", &[("comb", "env-mixed", 1.0)]),
        sub("dep", "srss-cases", "case:1"),
    ]);
    let expected = [
        ("d", admitted()),
        ("mech-d", withheld(IR)),
        ("srss-cases", withheld(IR_UNSIGNED)),
        ("srss-d", admitted()),
        ("abs-d", admitted()),
        ("sus+srss", admitted()),
        ("env-mixed", admitted()),
        ("mech-env", withheld(IR)),
        ("dep", withheld(WITHHELD_OPERAND)),
    ];
    for (q_ref, gated) in [(0.001, true), (0.0, false)] {
        let wire = connector_model(q_ref, combinations.clone());
        for mode in MODES {
            let out = solved(&wire, mode, "V-6 EX");
            for (id, want) in &expected {
                let want = if gated { want.clone() } else { admitted() };
                assert_eq!(gate(&out, id), want, "q_ref {q_ref} {mode:?} {id}");
                assert_eq!(combination_rows(&out, id).is_empty(), want["withheld"] == true, "{id} rows");
            }
            // A nested operand's rows are its own published rows.
            let record = out["contract_evidence"]["exact_combinations"].as_array().unwrap().iter().find(|r| r["combination_id"] == "srss-d").unwrap();
            assert_eq!(record["operands"], json!([{"combination_id": "d"}]));
            assert_eq!(record["expression"], json!({"terms": [{"combination_id": "d", "factor": 1.0}]}));
            for evidence in record["rows"].as_array().unwrap() {
                for source in evidence["sources"].as_array().unwrap() {
                    assert_eq!(source["combination_id"], "d");
                    let row = rows(&out).iter().find(|r| r["id"] == source["result_id"]).unwrap();
                    assert_eq!(row["basis_ref"], json!({"ref_type": "combination", "ref_id": "d"}));
                }
            }
        }
    }
}

/// `connector_model` with a consuming constant-effort support (DEC-049: one
/// translational DOF, a finite positive constant load) at the connector's free end,
/// when `ce`.
fn connector_ce_model(q_ref_x: f64, ce: bool, combinations: Value) -> Value {
    let mut wire = connector_model(q_ref_x, combinations);
    if ce {
        wire["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "support:ce", "node": "node:b", "family": "constant_effort_support",
            "restraints": ["UY"], "hanger": {"hanger_type": "constant_effort_support", "constant_load": qv(375.0, "N"), "travel_range": qv(0.05, "m"),
            "source_reference": "rt_a4_invented"}, "provenance": PROV}));
    }
    wire
}

/// The RT-A3/RT-A4 seam: EX gate 4 (a consuming constant-effort support) is one more
/// fixed-load source of the κ rule. Nested, a `mechanics` combination reads |κ − 1| ≤ 2⁻⁴⁶
/// under `CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE`, one naming an operand with an undefined
/// κ is withheld as `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED` (as on PF), and an absolute sum
/// or SRSS needs every operand's κ exactly 0 (`CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND`);
/// flat, the decisions are CE's unit factor-sum rule. Where an installed reference also
/// applies, gate 3 names the reason. Every envelope is admitted by the pressure reader with and
/// without its model.
#[test]
fn v6_gate_4_joins_the_fixed_load_count_on_the_exact_route() {
    let bound = 2f64.powi(-46);
    let third = 1.0 / 3.0;
    let combinations = json!([
        // Flat: CE's decisions.
        terms("flat-unit", "mechanics", &[("case", "case:1", 0.3), ("case", "case:2", 0.7)]),
        terms("flat-thirds", "mechanics", &[("case", "case:1", third), ("case", "case:2", 2.0 * third)]),
        terms("flat-over", "mechanics", &[("case", "case:1", 1.0), ("case", "case:2", 1.0)]),
        terms("flat-cancel", "mechanics", &[("case", "case:1", 2f64.powi(52)), ("case", "case:2", -(2f64.powi(52)))]),
        // Nested mechanics: CE as a source of κ.
        sub("d", "case:1", "case:2"),
        terms("mech-d", "mechanics", &[("comb", "d", 1.0)]),
        terms("sus+d", "mechanics", &[("case", "case:3", 1.0), ("comb", "d", 1.0)]),
        terms("srss-d", "srss", &[("comb", "d", 1.0)]),
        terms("abs-d", "absolute_sum", &[("comb", "d", 2.0)]),
        terms("sus+srss", "mechanics", &[("case", "case:3", 1.0), ("comb", "srss-d", 1.0)]),
        terms("m-near", "mechanics", &[("case", "case:1", 1.0 + bound)]),
        sub("d-near", "m-near", "case:2"),
        terms("mech-amplified", "mechanics", &[("comb", "d-near", 2f64.powi(46))]),
        terms("abs-amplified", "absolute_sum", &[("comb", "d-near", 2f64.powi(46))]),
        // Unsigned over CE-carrying load cases.
        terms("srss-cases", "srss", &[("case", "case:1", 1.0), ("case", "case:3", 1.0)]),
        terms("abs-cases", "absolute_sum", &[("case", "case:1", 1.0), ("case", "case:2", -0.5)]),
        // An undefined κ.
        range("env-mixed", &["case:1", "d"]),
        terms("mech-env", "mechanics", &[("comb", "env-mixed", 1.0)]),
        terms("srss-env", "srss", &[("comb", "env-mixed", 1.0)]),
        sub("dep", "srss-cases", "case:1"),
    ]);
    // (id, with the constant-effort source alone, with the installed reference as well).
    let expected = [
        ("flat-unit", admitted(), admitted()),
        ("flat-thirds", admitted(), admitted()),
        ("flat-over", withheld(CE), withheld(IR)),
        ("flat-cancel", withheld(CE), withheld(IR)),
        ("d", admitted(), admitted()),
        ("mech-d", withheld(CE), withheld(IR)),
        ("sus+d", admitted(), admitted()),
        ("srss-d", admitted(), admitted()),
        ("abs-d", admitted(), admitted()),
        ("sus+srss", admitted(), admitted()),
        ("m-near", admitted(), admitted()),
        ("d-near", admitted(), admitted()),
        ("mech-amplified", admitted(), admitted()),
        ("abs-amplified", withheld(CE_UNSIGNED), withheld(IR_UNSIGNED)),
        ("srss-cases", withheld(CE_UNSIGNED), withheld(IR_UNSIGNED)),
        ("abs-cases", withheld(CE_UNSIGNED), withheld(IR_UNSIGNED)),
        ("env-mixed", admitted(), admitted()),
        ("mech-env", withheld(CE_MIXED), withheld(IR)),
        ("srss-env", withheld(CE_UNSIGNED), withheld(IR_UNSIGNED)),
        ("dep", withheld(WITHHELD_OPERAND), withheld(WITHHELD_OPERAND)),
    ];
    for (q_ref, ce) in [(0.0, true), (0.001, true), (0.0, false)] {
        let wire = connector_ce_model(q_ref, ce, combinations.clone());
        for mode in MODES {
            let what = format!("seam q_ref {q_ref} ce {ce} {mode:?}");
            let out = solved(&wire, mode, &what);
            if ce {
                assert!(rows(&out).iter().any(|r| r["kind"] == "constant_effort_support_applied_load"), "{what}: the support is consumed");
            }
            for (id, ce_only, both) in &expected {
                let want = match (q_ref != 0.0, ce) {
                    (true, _) => both.clone(),
                    (false, true) => ce_only.clone(),
                    (false, false) => admitted(),
                };
                assert_eq!(gate(&out, id), want, "{what} {id}");
                assert_eq!(combination_rows(&out, id).is_empty(), want["withheld"] == true, "{what} {id} rows");
            }
            if ce && q_ref == 0.0 {
                let dep = with_code(&out, WITHHELD_OPERAND);
                assert_eq!(dep.len(), 1, "{what}");
                assert_eq!(dep[0]["message"], format!("Combination withheld: its operand combination(s) srss-cases ({CE_UNSIGNED}) are withheld, so it is withheld whole."));
                let unsigned = with_code(&out, CE_UNSIGNED);
                assert_eq!(unsigned.len(), 4, "{what}");
                assert_eq!(unsigned[0]["message"], "Combination withheld: a constant-effort support's force acts in every case state; an absolute sum or SRSS of operands whose fixed-load count is not exactly 0 would count it once per term with its sign discarded.");
                assert_eq!(with_code(&out, CE)[0]["message"], "Combination withheld: a constant-effort support would be counted by the sum of factors instead of once.");
            }
        }
    }
}

/// A forged gate on a constant-effort EX document is refused by the pressure reader, with
/// and without the model: a κ withholding renamed to the other source's code, an admitted
/// nested combination withheld, and a withheld unsigned one admitted.
#[test]
fn v6_gate_4_forgeries_are_refused_on_the_exact_route() {
    let combinations = json!([
        sub("d", "case:1", "case:2"),
        terms("mech-d", "mechanics", &[("comb", "d", 1.0)]),
        terms("srss-d", "srss", &[("comb", "d", 1.0)]),
        terms("srss-cases", "srss", &[("case", "case:1", 1.0), ("case", "case:3", 1.0)]),
    ]);
    let wire = connector_ce_model(0.0, true, combinations);
    let out = solved(&wire, PreviewSolverMode::SparseInteractive, "seam forgeries");
    let forge = |id: &str, reason: Option<&str>| {
        let mut forged = out.clone();
        let record = forged["contract_evidence"]["exact_combinations"].as_array_mut().unwrap().iter_mut().find(|r| r["combination_id"] == id).unwrap();
        record["gate"] = json!({"withheld": reason.is_some(), "reason": reason});
        forged
    };
    for (id, reason) in [("mech-d", Some(IR)), ("srss-cases", Some(IR_UNSIGNED)), ("srss-cases", Some(CE)), ("srss-d", Some(CE_UNSIGNED)), ("srss-cases", None), ("mech-d", None)] {
        let forged = forge(id, reason);
        for model in [Some(&wire["model"]), None] {
            let verdict = reader_verdict::validate("pressure", &forged, model, None);
            assert_eq!(verdict["ok"], false, "{id} {reason:?} model {}: {verdict}", model.is_some());
            assert_eq!(verdict["error"]["code"], "SOURCE_PHYSICS_COMBINATION_GATE", "{id} {reason:?} model {}: {verdict}", model.is_some());
        }
    }
}

/// The fixed-load withholdings that §5.1 distinguishes, by EX source: an operand with an
/// undefined κ (`mech-env`) and a combination whose own κ is undefined although its operands'
/// are defined (`M3`, `Q`: V-10's chain, κ(M3) = 1 + t⁴ needs more than 4,096 bits).
const GATE_TEXTS: [(&str, &str); 3] = [
    (CE, "Combination withheld: a constant-effort support would be counted by the sum of factors instead of once."),
    (CE_MIXED, "Combination withheld: an operand's fixed-load count is undefined, so the number of times this combination counts a constant-effort support's force cannot be decided."),
    (IR, "Combination withheld: a connector or tie rod has a nonzero installed reference, which superposition counts by the sum of the factors instead of once, and the factors do not sum to 1 within 64 epsilon."),
];
fn gate_text(code: &str) -> &'static str {
    GATE_TEXTS.iter().find(|(c, _)| *c == code).unwrap().1
}
fn undefined_kappa_combinations() -> Value {
    json!([
        sub("d", "case:1", "case:2"),
        range("env-mixed", &["case:1", "d"]),
        terms("mech-env", "mechanics", &[("comb", "env-mixed", 1.0)]),
        terms("M0", "mechanics", &[("case", "case:1", 1.0), ("case", "case:2", T)]),
        sub("D0", "M0", "case:1"),
        terms("M1", "mechanics", &[("case", "case:1", 1.0), ("comb", "D0", T)]),
        sub("D1", "M1", "case:1"),
        terms("M2", "mechanics", &[("case", "case:1", 1.0), ("comb", "D1", T)]),
        sub("D2", "M2", "case:1"),
        terms("M3", "mechanics", &[("case", "case:1", 1.0), ("comb", "D2", T)]),
        terms("Q", "mechanics", &[("case", "case:1", 2.0), ("comb", "D2", T)]),
    ])
}
/// The four sources: (installed reference, constant effort) → (mech-env, M3 and Q).
const UNDEFINED_KAPPA_SOURCES: [((f64, bool), Option<&str>, Option<&str>); 4] = [
    // A constant-effort source alone: an undefined operand is PF's operand-mixed code;
    // an undefined own κ is the source's code.
    ((0.0, true), Some(CE_MIXED), Some(CE)),
    // An installed reference alone: §5.1 names no operand-mixed code for it, so both
    // are gate 3's code (§10).
    ((0.001, false), Some(IR), Some(IR)),
    // Both: gate 3 precedes gate 4, so gate 3's code names both.
    ((0.001, true), Some(IR), Some(IR)),
    ((0.0, false), None, None),
];

#[test]
fn v6_undefined_kappa_withholdings_on_the_exact_route_by_source() {
    for ((q_ref, ce), operand, own) in UNDEFINED_KAPPA_SOURCES {
        let wire = connector_ce_model(q_ref, ce, undefined_kappa_combinations());
        for mode in MODES {
            let what = format!("q_ref {q_ref} ce {ce} {mode:?}");
            let out = solved_typed(&wire, mode, &what);
            for id in ["d", "env-mixed", "M0", "D0", "M1", "D1", "M2", "D2"] {
                assert_eq!(gate(&out, id), admitted(), "{what} {id}");
            }
            let want = |code: Option<&str>| code.map_or_else(admitted, withheld);
            assert_eq!(gate(&out, "mech-env"), want(operand), "{what} mech-env");
            for id in ["M3", "Q"] {
                assert_eq!(gate(&out, id), want(own), "{what} {id}");
                assert_eq!(combination_rows(&out, id).is_empty(), own.is_some(), "{what} {id} rows");
            }
            let named: Vec<&str> = with_code(&out, UNREPRESENTABLE).iter().map(|d| d["affected_refs"][0].as_str().unwrap()).collect();
            assert_eq!(named, ["M3", "Q"], "{what}");
            // Each withholding's diagnostic: a warning with its code's text.
            let gates: Vec<&Value> = diagnostics(&out).iter().filter(|d| d["id"].as_str().unwrap().starts_with("diagnostic:exact-combination:gate:")).collect();
            let expected: Vec<(&str, &str)> = [("mech-env", operand), ("M3", own), ("Q", own)].iter().filter_map(|(id, code)| code.map(|c| (*id, c))).collect();
            assert_eq!(gates.len(), expected.len(), "{what}");
            for (id, code) in expected {
                let d = gates.iter().find(|d| d["affected_refs"] == json!([id])).unwrap_or_else(|| panic!("{what}: no gate diagnostic for {id}"));
                assert_eq!((d["code"].as_str().unwrap(), d["severity"].as_str().unwrap(), d["message"].as_str().unwrap()), (code, "warning", gate_text(code)), "{what} {id}");
            }
        }
    }
}

/// A swapped undefined-κ code, in the record and its diagnostic together, is refused by the
/// pressure reader with and without the model, under every source.
#[test]
fn v6_swapped_undefined_kappa_codes_are_refused_on_the_exact_route() {
    for ((q_ref, ce), operand, own) in UNDEFINED_KAPPA_SOURCES {
        let (Some(operand), Some(own)) = (operand, own) else { continue };
        let wire = connector_ce_model(q_ref, ce, undefined_kappa_combinations());
        let out = solved_typed(&wire, PreviewSolverMode::SparseInteractive, "swapped codes");
        let swap = |id: &str, code: &str| {
            let mut forged = out.clone();
            let record = forged["contract_evidence"]["exact_combinations"].as_array_mut().unwrap().iter_mut().find(|r| r["combination_id"] == id).unwrap();
            record["gate"]["reason"] = json!(code);
            let d = forged["diagnostics"].as_array_mut().unwrap().iter_mut().find(|d| d["id"].as_str().unwrap().starts_with("diagnostic:exact-combination:gate:") && d["affected_refs"] == json!([id])).unwrap();
            d["code"] = json!(code);
            d["message"] = json!(gate_text(code));
            forged
        };
        let mut swaps = Vec::new();
        for code in [CE, CE_MIXED, IR] {
            if code != operand {
                swaps.push(("mech-env", code));
            }
            if code != own {
                swaps.push(("M3", code));
            }
        }
        for (id, code) in swaps {
            let forged = swap(id, code);
            for model in [Some(&wire["model"]), None] {
                let verdict = reader_verdict::validate("pressure", &forged, model, None);
                let what = format!("q_ref {q_ref} ce {ce} {id} as {code} model {}", model.is_some());
                assert_eq!(verdict["ok"], false, "{what}: {verdict}");
                assert_eq!(verdict["error"]["code"], "SOURCE_PHYSICS_COMBINATION_GATE", "{what}: {verdict}");
            }
        }
    }
}

#[test]
fn v6_the_resolved_stiffness_gate_reads_every_reached_load_case() {
    let mut wire: Value = serde_json::from_str(LOAD_STATE_CONNECTED).unwrap();
    wire["model"]["combinations"] = json!([
        sub("hot-cold", "case:hot", "case:cold"),
        terms("srss-hc", "srss", &[("comb", "hot-cold", 1.0)]),
        terms("abs-cold", "absolute_sum", &[("case", "case:cold", 1.0)]),
    ]);
    for mode in MODES {
        let out = solved(&wire, mode, "V-6 stiffness");
        assert_eq!(gate(&out, "hot-cold"), admitted());
        assert_eq!(gate(&out, "srss-hc"), withheld("COMBINATION_RESOLVED_STIFFNESS_MIXED"));
        // One reached case: no stiffness to differ (the fixture's installed reference, if
        // any, is gate 3's).
        assert_ne!(gate(&out, "abs-cold")["reason"], "COMBINATION_RESOLVED_STIFFNESS_MIXED");
    }
}

// ------------------------------------------------------------------ V-7
#[test]
fn v7_cycles_are_refused_at_the_solve() {
    for combinations in [
        json!([terms("A", "mechanics", &[("comb", "A", 1.0)])]),
        json!([terms("A", "srss", &[("comb", "B", 1.0)]), sub("B", "A", "W1")]),
        json!([terms("A", "absolute_sum", &[("comb", "C", 1.0)]), range("B", &["W1", "A"]), terms("C", "mechanics", &[("comb", "B", 1.0)])]),
    ] {
        let wire = pf_cantilever(false, pf_cases(), combinations.clone());
        for mode in MODES {
            let out = run_value(&wire, mode);
            assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE", "{combinations}");
            assert_eq!(with_code(&out, "LOAD_COMBINATION_OPERAND_CYCLE").len(), 1, "{combinations}");
        }
    }
}

#[test]
fn v7_a_shared_id_resolves_to_the_load_case() {
    let shared = |name: &str| pf_cantilever(false, json!([force_case("S", "global_y", -1000.0), force_case("W2", "global_y", -500.0)]), json!([
        terms(name, "mechanics", &[("case", "W2", 3.0)]),
        sub("X", "S", "W2"),
        range("E", &["S", "W2"]),
    ]));
    for mode in MODES {
        let with_shared = solved(&shared("S"), mode, "shared id");
        let renamed = solved(&shared("other"), mode, "renamed");
        for id in ["X", "E"] {
            let a: Vec<&Value> = combination_rows(&with_shared, id);
            let b: Vec<&Value> = combination_rows(&renamed, id);
            assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap(), "{mode:?} {id}: byte for byte");
            for row in a {
                for source in row["source_result_refs"].as_array().unwrap() {
                    let source = rows(&with_shared).iter().find(|r| &r["id"] == source).unwrap();
                    // A recomputed magnitude names its own combination's components.
                    let own = source["basis_ref"] == json!({"ref_type": "combination", "ref_id": id});
                    assert!(own || source["basis_ref"]["ref_type"] == "load_case", "{id} reads the load case: {source}");
                }
            }
        }
        // The shadowed combination is named through a term's `combination`.
        let mut by_kind = shared("S");
        by_kind["model"]["combinations"].as_array_mut().unwrap().push(terms("Y", "srss", &[("comb", "S", 1.0), ("case", "S", 1.0)]));
        let out = solved(&by_kind, mode, "by kind");
        let row = combination_rows(&out, "Y").into_iter().find(|r| r["kind"] == "support_reaction_component_v2" && r["metadata"]["component"] == "Fy").unwrap();
        let fy = |ref_type: &str, id: &str| rows(&out).iter().find(|r| r["kind"] == "support_reaction_component_v2" && r["entity_ref"] == "support:a"
            && r["metadata"]["component"] == "Fy" && r["basis_ref"] == json!({"ref_type": ref_type, "ref_id": id})).unwrap()["value"].as_f64().unwrap();
        let (c, l) = (fy("combination", "S"), fy("load_case", "S"));
        assert_ne!(c, l);
        assert_eq!(row["value"].as_f64().unwrap(), (c * c + l * l).sqrt(), "SRSS of the combination and the load case of one id");
    }
}

#[test]
fn v7_a_declared_operating_comparison_names_load_cases_only() {
    let mut operating = sub("O-S", "C", "W2");
    operating["difference_basis"] = json!("operating_minus_sustained");
    let wire = pf_cantilever(false, pf_cases(), json!([terms("C", "srss", &[("case", "W1", 1.0)]), operating]));
    for mode in MODES {
        let out = run_value(&wire, mode);
        assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE");
        assert_eq!(with_code(&out, "LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE").len(), 1);
    }
}

/// Sustained (a load case) plus an SRSS pairs every ordinary row on PF (r1 F4): it
/// publishes the same rows as the flat sustained plus a load case, and misses none.
#[test]
fn v7_sustained_plus_srss_pairs_every_row() {
    let wire = pf_cantilever(false, pf_cases(), json!([
        sub("d", "W1", "W2"),
        terms("srss", "srss", &[("comb", "d", 1.0), ("case", "W3", 1.0)]),
        terms("occ", "mechanics", &[("case", "W2", 1.0), ("comb", "srss", 1.0)]),
        terms("flat", "mechanics", &[("case", "W2", 1.0), ("case", "W3", 1.0)]),
    ]));
    for mode in MODES {
        let out = solved(&wire, mode, "sustained + SRSS");
        for code in ["LOAD_COMBINATION_SOURCE_RESULT_MISSING", "LOAD_COMBINATION_ROW_WITHHELD", "LOAD_COMBINATION_VECTOR_COMPONENT_MISSING"] {
            assert!(with_code(&out, code).is_empty(), "{mode:?} {code}: {:?}", with_code(&out, code));
        }
        // A magnitude row is identified by kind and node; its metadata states only its
        // derivation, on a nested or unsigned combination, and is absent on a flat one (V-8).
        let shape = |id: &str| combination_rows(&out, id).iter().map(|r| if r["kind"] == "displacement_magnitude" {
            (r["kind"].clone(), r["entity_ref"].clone(), Value::Null, Value::Null)
        } else {
            (r["kind"].clone(), r["entity_ref"].clone(), r["metadata"]["component"].clone(), r["metadata"]["location"].clone())
        }).collect::<Vec<_>>();
        assert_eq!(shape("occ"), shape("flat"), "{mode:?}");
        assert_eq!(shape("srss"), shape("flat"), "{mode:?}");
        for (id, basis, sign) in [("occ", "explicit_user_linear_combination", "positive value follows explicit user linear combination of matching source result sign conventions"),
            ("srss", "explicit_user_srss", SRSS_SIGN)] {
            for row in combination_rows(&out, id).iter().filter(|r| r["kind"] == "displacement_magnitude") {
                assert_eq!(row["metadata"], json!({"component": "displacement_magnitude", "coordinate_system": "global", "location": "node", "basis": basis, "sign_convention": sign}), "{mode:?} {id}");
            }
        }
        assert!(combination_rows(&out, "flat").iter().filter(|r| r["kind"] == "displacement_magnitude").all(|r| r.get("metadata").is_none()), "{mode:?}");
        // Each occ row is the exact sum of its two source rows, rounded once.
        for row in combination_rows(&out, "occ") {
            let refs = row["source_result_refs"].as_array().unwrap();
            if row["kind"] == "displacement_magnitude" || row["kind"].as_str().unwrap().ends_with("magnitude_v2") {
                continue;
            }
            assert_eq!(refs.len(), 2, "{row}");
            let value = |i: usize| rows(&out).iter().find(|r| r["id"] == refs[i]).unwrap()["value"].as_f64().unwrap();
            assert_eq!(row["value"].as_f64().unwrap(), value(0) + value(1), "{row}");
            assert_eq!(row["metadata"]["sign_convention"], "positive value follows explicit user linear combination of matching source result sign conventions");
        }
    }
}

#[test]
fn v7_evaluation_order_is_topological_and_a_function_of_the_document() {
    let wire = pf_cantilever(true, pf_cases(), json!([
        terms("late", "srss", &[("comb", "early", 1.0)]),
        sub("early", "W1", "W2"),
        terms("blocked", "srss", &[("case", "W1", 1.0)]),
        terms("dependant", "absolute_sum", &[("comb", "blocked", 1.0), ("comb", "late", 1.0)]),
        terms("flat", "mechanics", &[("case", "W1", 0.5), ("case", "W2", 0.5)]),
    ]));
    for mode in MODES {
        let out = solved(&wire, mode, "order");
        let gates: Vec<&str> = out["contract_evidence"]["combination_gates"].as_array().unwrap().iter().map(|g| g["combination_id"].as_str().unwrap()).collect();
        assert_eq!(gates, ["early", "late", "blocked", "dependant", "flat"]);
        assert_eq!(gate(&out, "late"), admitted());
        assert_eq!(gate(&out, "blocked"), withheld(CE_UNSIGNED));
        assert_eq!(gate(&out, "dependant"), withheld(WITHHELD_OPERAND));
        let first = |id: &str| rows(&out).iter().position(|r| r["basis_ref"]["ref_id"] == id && r["basis_ref"]["ref_type"] == "combination").unwrap();
        assert!(first("early") < first("late") && first("late") < first("flat"));
        // Solving again publishes the same bytes.
        assert_eq!(serde_json::to_string(&out).unwrap(), serde_json::to_string(&solved(&wire, mode, "again")).unwrap());
    }
}

/// §6: an unsigned result outside the binary64 range withholds the row as
/// "load-case algebra refused"; the other rows publish.
#[test]
fn a_non_representable_unsigned_row_is_withheld_by_name() {
    let wire = pf_cantilever(false, pf_cases(), json!([terms("huge", "absolute_sum", &[("case", "W1", f64::MAX)])]));
    for mode in MODES {
        let out = solved_typed(&wire, mode, "non-representable");
        let withheld = with_code(&out, "LOAD_COMBINATION_ROW_WITHHELD");
        assert_eq!(withheld.len(), 1);
        assert!(withheld[0]["message"].as_str().unwrap().contains("(load-case algebra refused)"), "{}", withheld[0]["message"]);
        // A zero source row (|x| ≤ 1) still publishes, as +0.0 or a finite value.
        assert!(combination_rows(&out, "huge").iter().all(|r| r["value"].as_f64().unwrap().is_finite()));
        assert!(!combination_rows(&out, "huge").is_empty());
    }
}

/// The V-5 cantilever with two cases: `A`, unit forces in global X and Y at the
/// anchored root (finite reaction components, zero displacements and stresses), and
/// `B`, a large tip force in global X (comparable tip displacements in X and Y).
fn overflow_model(exact: bool, combinations: Value) -> Value {
    let mut wire = v5_model(exact, json!([]));
    let force = |id: &str, node: &str, direction: &str, value: f64| json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": node},
        "direction": direction, "magnitude": qv(value, "N"), "dimension": "force", "provenance": PROV});
    let mut cases = vec![
        json!({"id": "A", "provenance": PROV, "primitive_loads": [force("load:A:x", "node:root", "global_x", 1.0), force("load:A:y", "node:root", "global_y", 1.0)]}),
        json!({"id": "B", "provenance": PROV, "primitive_loads": [force("load:B:x", "node:tip", "global_x", 1.0e9)]}),
    ];
    if exact {
        for case in &mut cases {
            case["pressure_regions"] = json!([]);
        }
    }
    wire["model"]["load_cases"] = json!(cases);
    wire["model"]["combinations"] = combinations;
    wire
}

/// P2 r1 (absolute-sum-srss §3, §6): an unsigned combination whose recomputed
/// magnitude leaves the binary64 range from finite combined components withholds
/// that row, or its whole support group, as "load-case algebra refused", and every
/// other result publishes. Before the fix the norm published as an infinity and the
/// whole solve was refused.
#[test]
fn an_overflowing_unsigned_magnitude_is_withheld_and_the_rest_publishes() {
    for exact in [false, true] {
        for mode in MODES {
            let what = format!("exact={exact} {mode:?}");
            // The tip displacement components of B, in the published unit.
            let probe = solved_typed(&overflow_model(exact, json!([])), mode, &what);
            let tip = |kind: &str| case_rows(&probe, "B").iter().find(|r| r["kind"] == kind && r["entity_ref"] == "node:tip").unwrap()["value"].as_f64().unwrap().abs();
            let (ux, uy, uz) = (tip("global_nodal_displacement_x"), tip("global_nodal_displacement_y"), tip("global_nodal_displacement_z"));
            let largest = ux.max(uy).max(uz);
            assert!(largest >= 1.0 && ux.min(uy) > 0.25 * largest, "{what}: {ux} {uy} {uz}");
            // Every component of f·B at the tip stays finite; their norm does not.
            let f = f64::MAX / largest / (1.0 + 2f64.powi(-8));
            assert!((f * ux).is_finite() && (f * uy).is_finite() && (f * uz).is_finite() && !(f * ux).hypot(f * uy).is_finite(), "{what}");
            let wire = overflow_model(exact, json!([
                terms("support", "absolute_sum", &[("case", "A", 1.4e308)]),
                terms("tip", "absolute_sum", &[("case", "B", f)]),
                terms("srss-support", "srss", &[("case", "A", 1.4e308)]),
            ]));
            let out = solved_typed(&wire, mode, &what);
            for id in ["support", "tip", "srss-support"] {
                assert_eq!(gate(&out, id), admitted(), "{what} {id}");
                let held: Vec<&Value> = with_code(&out, "LOAD_COMBINATION_ROW_WITHHELD").into_iter().filter(|d| d["affected_refs"][0] == id).collect();
                assert_eq!(held.len(), 1, "{what} {id}");
                let message = held[0]["message"].as_str().unwrap();
                let published = combination_rows(&out, id);
                assert!(published.iter().all(|r| r["value"].as_f64().unwrap().is_finite()), "{what} {id}");
                if id == "tip" {
                    assert!(message.contains("(load-case algebra refused)"), "{what}: {message}");
                    assert!(!published.iter().any(|r| r["kind"] == "displacement_magnitude" && r["entity_ref"] == "node:tip"), "{what}");
                    // The root's zero magnitude still publishes, with its derivation stated.
                    let root = published.iter().find(|r| r["kind"] == "displacement_magnitude" && r["entity_ref"] == "node:root").unwrap();
                    assert_eq!(root["metadata"]["basis"], "explicit_user_absolute_sum", "{what}");
                } else {
                    // The anchor's group is withheld whole: no component and no magnitude.
                    assert!(message.contains("support-action:12:support:root (load-case algebra refused)"), "{what}: {message}");
                    assert!(!published.iter().any(|r| r["entity_ref"] == "support:root"), "{what} {id}");
                    // A's displacements are zero, so its node rows publish.
                    assert!(published.iter().any(|r| r["kind"] == "displacement_magnitude"), "{what} {id}");
                }
            }
            // The load cases publish whole.
            assert_eq!(case_rows(&out, "A").len(), case_rows(&probe, "A").len(), "{what}");
            assert_eq!(case_rows(&out, "B").len(), case_rows(&probe, "B").len(), "{what}");
        }
    }
}

// ------------------------------------------------------------------ V-9
fn gap_bar(combinations: Value) -> Value {
    // The op1a bar (`op1a_operand_standing.rs` V-1): a tip gap closing at c in UX.
    let (l, od, wall, k, alpha) = (4.0, 0.12, 0.01, 1_048_576.0f64, 1.0 / 65_536.0);
    let inner = od - 2.0 * wall;
    let area = std::f64::consts::PI / 4.0 * (od * od - inner * inner);
    let e = 4.0 * k / area;
    let force = |id: &str, f: f64| json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": "node:b"}, "direction": "global_x",
        "magnitude": qv(f, "N"), "dimension": "force", "provenance": PROV});
    let thermal = json!({"id": "t-o", "category": "thermal", "target": {"type": "element", "pipe": "pipe:a-b"}, "direction": "global_x",
        "magnitude": qv(16.0, "degC"), "dimension": "temperature_interval", "provenance": PROV});
    json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:rt-a4-v9", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "node:a", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV}, {"id": "node:b", "position": {"x": l, "y": 0.0, "z": 0.0}, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:a-b", "from": "node:a", "to": "node:b", "section": {"outside_diameter": qv(od, "m"), "wall_thickness": qv(wall, "m")},
            "material": "material:m", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV}],
        "supports": [{"id": "support:a", "node": "node:a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
            {"id": "support:gap", "node": "node:b", "family": "nonlinear", "restraints": [], "provenance": PROV,
             "nonlinear": {"behavior": "gap", "closes_when": "positive_displacement", "dof": "UX", "gap": qv(2f64.powi(-9), "m"), "initial_state": "inactive"}}],
        "components": [],
        "materials": [{"id": "material:m", "elastic_modulus": qv(e, "Pa"), "shear_modulus": qv(e / 2.6, "Pa"), "thermal_expansion_coefficient": qv(alpha, "1/degC"), "provenance": PROV}],
        "load_cases": [
            {"id": "S", "primitive_loads": [force("f-s", 1536.0)], "provenance": PROV},
            {"id": "O", "primitive_loads": [force("f-o", 1536.0), thermal], "provenance": PROV},
            {"id": "Q", "primitive_loads": [force("f-q", 512.0)], "provenance": PROV},
        ],
        "combinations": combinations}, "materials": []})
}

#[test]
fn v9_decision_oq1_admits_a_solved_state_plus_increments() {
    let wire = gap_bar(json!([
        sub("D", "O", "S"),
        terms("E", "srss", &[("comb", "D", 1.0)]),
        terms("occ", "mechanics", &[("case", "S", 1.0), ("comb", "E", 1.0)]),
        terms("occ-minus", "mechanics", &[("comb", "E", -1.0), ("case", "S", 1.0), ("comb", "D", 0.5)]),
        terms("factor", "mechanics", &[("case", "S", 1.5), ("comb", "E", 1.0)]),
        terms("two-cases", "mechanics", &[("case", "S", 1.0), ("case", "Q", 1.0), ("comb", "E", 1.0)]),
        range("R", &["S", "O"]),
        terms("kappa-one", "mechanics", &[("case", "S", 1.0), ("comb", "R", 1.0)]),
        terms("srss-case", "srss", &[("case", "O", 1.0)]),
        terms("undefined", "mechanics", &[("case", "S", 1.0), ("comb", "srss-case", 1.0)]),
        terms("singleton", "mechanics", &[("case", "S", 1.0)]),
        terms("flat", "mechanics", &[("case", "S", 1.0), ("case", "O", 1.0)]),
        terms("abs", "absolute_sum", &[("comb", "D", 1.0), ("case", "Q", 1.0)]),
    ]));
    for mode in MODES {
        let out = solved(&wire, mode, "V-9");
        for id in ["D", "E", "occ", "occ-minus", "R", "srss-case", "abs"] {
            assert_eq!(gate(&out, id), admitted(), "{mode:?} {id}");
        }
        for id in ["factor", "two-cases", "kappa-one", "undefined", "singleton", "flat"] {
            assert_eq!(gate(&out, id), withheld(NONLINEAR), "{mode:?} {id}");
        }
        for id in ["occ", "occ-minus"] {
            for row in combination_rows(&out, id) {
                // Every row states its derivation, the magnitudes included (review r1 P4).
                let metadata = row["metadata"].as_object().unwrap_or_else(|| panic!("{id}: no metadata on {row}"));
                assert_eq!(metadata["sign_convention"], OQ1_SIGN, "{id}");
                assert_eq!(metadata["basis"], "explicit_user_linear_combination", "{id}");
            }
            assert!(!combination_rows(&out, id).is_empty());
        }
        // The operand standing records (slice 1a's shape), one per admitted combination.
        let records = out["contract_evidence"]["combination_operands"].as_array().unwrap();
        let ids: Vec<&str> = records.iter().map(|r| r["combination_id"].as_str().unwrap()).collect();
        assert_eq!(ids, ["D", "E", "occ", "occ-minus", "R", "srss-case", "abs"]);
        let occ = records.iter().find(|r| r["combination_id"] == "occ").unwrap();
        assert_eq!(occ["basis"], "mechanics");
        assert_eq!(occ["expression"], json!({"terms": [{"load_case_id": "S", "factor": 1.0}, {"combination_id": "E", "factor": 1.0}]}));
        assert_eq!(occ["operands"][1], json!({"combination_id": "E"}));
        assert_eq!(occ["operands"][0]["load_case_id"], "S");
        assert!(occ["operands"][0]["numerical_quality"].is_object());
    }
}

// ------------------------------------------------------------------ reader tampers (review r1 P2)
fn pf_verdict(out: &Value, model: Option<&Value>) -> Value {
    reader_verdict::validate("preview-physics", out, model, None)
}
fn assert_refused(out: &Value, model: Option<&Value>, code: &str, what: &str) {
    let verdict = pf_verdict(out, model);
    assert_eq!(verdict["error"]["code"], code, "{what} (model given: {}): {verdict}", model.is_some());
}
fn set_gate(out: &mut Value, id: &str, reason: Option<&str>) {
    let gates = out["contract_evidence"]["combination_gates"].as_array_mut().unwrap();
    let gate = gates.iter_mut().find(|g| g["combination_id"] == id).unwrap();
    gate["withheld"] = json!(reason.is_some());
    gate["reason"] = json!(reason);
}
/// Sets the value of one combination row of `kind` on `entity` at `location`, which
/// must be `from` before the tamper.
fn set_row(out: &mut Value, id: &str, kind: &str, location: &str, from: f64, to: f64) {
    let row = out["results"].as_array_mut().unwrap().iter_mut()
        .find(|r| r["basis_ref"] == json!({"ref_type": "combination", "ref_id": id}) && r["kind"] == kind && r["metadata"]["location"] == location)
        .unwrap_or_else(|| panic!("no {kind} row of {id}"));
    assert_eq!(row["value"].as_f64().unwrap(), from, "{id} {kind}");
    row["value"] = json!(to);
}

/// preview-physics-2's readers re-derive the gates and the unsigned rows: with the
/// model, exactly; without it, from what the document states, refusing what it cannot
/// verify. Before review r1 P2 both readers admitted every tamper below.
#[test]
fn pf_readers_refuse_tampered_gates_and_unsigned_rows() {
    // Reason swap, in a nonlinear model: the flat mechanics combination is withheld
    // NONLINEAR, restated as an unsigned-operand κ withholding.
    let wire = gap_bar(json!([
        sub("D", "O", "S"),
        terms("E", "srss", &[("comb", "D", 1.0)]),
        terms("occ", "mechanics", &[("case", "S", 1.0), ("comb", "E", 1.0)]),
        terms("flat", "mechanics", &[("case", "S", 1.0), ("case", "O", 1.0)]),
    ]));
    let model = Some(&wire["model"]);
    let out = solved(&wire, PreviewSolverMode::SparseInteractive, "tamper base (nonlinear)");
    assert_eq!(pf_verdict(&out, None)["ok"], true, "the records state E's factors");
    let mut swapped = out.clone();
    set_gate(&mut swapped, "flat", Some(CE_UNSIGNED));
    assert_refused(&swapped, None, "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED", "reason swap");
    assert_refused(&swapped, model, "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_DERIVATION", "reason swap");
    // Value change: E's SRSS of a bar's axial difference has a zero transverse shear
    // row; restated as 123. The record states E's factor, so the model-free reader
    // re-derives it too.
    let mut changed = out.clone();
    set_row(&mut changed, "E", "element_local_shear_force_y", "midspan", 0.0, 123.0);
    assert_refused(&changed, None, "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_VALUE", "value change (nonlinear)");
    assert_refused(&changed, model, "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_VALUE", "value change (nonlinear)");

    // A linear model with a consuming constant-effort support: `combination_derivation`
    // states every unsigned and nested combination's factors and κ inputs (§10.3).
    let wire = pf_cantilever(true, pf_cases(), json!([
        terms("srss-cases", "srss", &[("case", "W1", 1.0), ("case", "W2", 1.0)]),
        sub("d1", "W1", "W2"),
        terms("abs-d", "absolute_sum", &[("comb", "d1", 1.0)]),
        terms("srss-d", "srss", &[("comb", "d1", 1.0)]),
    ]));
    let model = Some(&wire["model"]);
    let out = solved(&wire, PreviewSolverMode::SparseInteractive, "tamper base (constant effort)");
    assert_eq!(gate(&out, "srss-cases"), withheld(CE_UNSIGNED));
    assert_eq!(pf_verdict(&out, model)["ok"], true);
    assert_eq!(pf_verdict(&out, None)["ok"], true);
    // Value change: the axial normal stress under transverse loads is zero.
    let mut changed = out.clone();
    set_row(&mut changed, "srss-d", "element_local_axial_normal_stress", "midspan", 0.0, 123.0);
    for given in [None, model] {
        assert_refused(&changed, given, "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_VALUE", "value change (linear)");
    }
    // κ-gate changes: the unsigned-operand withholding restated as the flat unit
    // factor-sum code; withdrawn; and an admitted κ = 0 combination restated withheld.
    for (id, reason) in [("srss-cases", Some(CE)), ("srss-cases", None), ("abs-d", Some(CE_UNSIGNED))] {
        let mut gated = out.clone();
        set_gate(&mut gated, id, reason);
        gated["results"].as_array_mut().unwrap().retain(|r| !(reason.is_some() && r["basis_ref"] == json!({"ref_type": "combination", "ref_id": id})));
        for given in [None, model] {
            assert_refused(&gated, given, "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_DERIVATION", &format!("κ gate {id} {reason:?}"));
        }
    }
}

// ------------------------------------------------------------------ model-free derivation (§10.3)
const DERIVATION: &str = "combination_derivation";
const EVIDENCE: &str = "SOURCE_PREVIEW_PHYSICS_COMBINATION_DERIVATION_EVIDENCE";
fn derivation_record<'a>(out: &'a mut Value, id: &str) -> &'a mut Value {
    out["contract_evidence"][DERIVATION]["combinations"].as_array_mut().unwrap().iter_mut().find(|r| r["combination_id"] == id).unwrap()
}
fn linear_combinations() -> Value {
    json!([
        terms("srss-w", "srss", &[("case", "W1", 1.0), ("case", "W3", -2.0)]),
        sub("d1", "W1", "W2"),
        terms("abs-d", "absolute_sum", &[("comb", "d1", 0.5), ("case", "W3", 1.0)]),
        terms("sus+srss", "mechanics", &[("case", "W2", 1.0), ("comb", "srss-w", 1.0)]),
    ])
}

/// §10.3: a linear-model document with an SRSS, an absolute sum and a nested
/// combination reads without its model; a tampered factor, operand, row or model
/// condition, an incomplete closure and a missing member are refused, with and
/// without the model. Before §10.3 the model-free reader refused the untampered
/// document (`..._UNSIGNED_UNVERIFIED`).
#[test]
fn pf_model_free_readers_verify_from_the_published_derivation() {
    let wire = pf_cantilever(false, pf_cases(), linear_combinations());
    let model = Some(&wire["model"]);
    let out = solved(&wire, PreviewSolverMode::SparseInteractive, "linear derivation base");
    let derivation = &out["contract_evidence"][DERIVATION];
    assert_eq!(derivation["nonlinear_supports"], false);
    assert_eq!(derivation["consuming_constant_effort"], false);
    let ids: Vec<&str> = derivation["combinations"].as_array().unwrap().iter().map(|r| r["combination_id"].as_str().unwrap()).collect();
    assert_eq!(ids, ["srss-w", "d1", "abs-d", "sus+srss"]);
    assert_eq!(derivation["combinations"][2], json!({"combination_id": "abs-d", "basis": "absolute_sum", "expression": {
        "terms": [{"combination_id": "d1", "factor": 0.5}, {"load_case_id": "W3", "factor": 1.0}]}}));
    assert_eq!(derivation["combinations"][1]["expression"], json!({"minuend_id": "W1", "subtrahend_id": "W2"}));

    let mut factor = out.clone();
    derivation_record(&mut factor, "srss-w")["expression"]["terms"][1]["factor"] = json!(-3.0);
    assert_refused(&factor, None, "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_VALUE", "factor");
    assert_refused(&factor, model, EVIDENCE, "factor");
    let mut operand = out.clone();
    derivation_record(&mut operand, "srss-w")["expression"]["terms"][0]["load_case_id"] = json!("W2");
    assert_refused(&operand, None, "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_SOURCES", "operand");
    assert_refused(&operand, model, EVIDENCE, "operand");
    let mut row = out.clone();
    let target = row["results"].as_array_mut().unwrap().iter_mut()
        .find(|r| r["basis_ref"] == json!({"ref_type": "combination", "ref_id": "srss-w"}) && r["kind"] == "element_local_shear_force_y" && r["value"] != 0.0)
        .unwrap();
    target["value"] = json!(target["value"].as_f64().unwrap() * 2.0);
    for given in [None, model] {
        assert_refused(&row, given, "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_VALUE", "row");
    }
    // A consuming constant effort stated where there is none: srss-w over load cases
    // would then be withheld.
    let mut condition = out.clone();
    condition["contract_evidence"][DERIVATION]["consuming_constant_effort"] = json!(true);
    assert_refused(&condition, None, "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_DERIVATION", "condition");
    assert_refused(&condition, model, EVIDENCE, "condition");
    let mut closure = out.clone();
    closure["contract_evidence"][DERIVATION]["combinations"].as_array_mut().unwrap().retain(|r| r["combination_id"] != "d1");
    for given in [None, model] {
        assert_refused(&closure, given, EVIDENCE, "closure");
    }
    let mut missing = out.clone();
    missing["contract_evidence"].as_object_mut().unwrap().remove(DERIVATION);
    assert_refused(&missing, None, "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_UNVERIFIED", "missing");
    assert_refused(&missing, model, EVIDENCE, "missing");

    // With a consuming constant effort, a κ withholding without its record is unverifiable.
    let wire = pf_cantilever(true, pf_cases(), json!([terms("srss-cases", "srss", &[("case", "W1", 1.0), ("case", "W2", 1.0)])]));
    let out = solved(&wire, PreviewSolverMode::SparseInteractive, "constant-effort derivation base");
    assert_eq!(gate(&out, "srss-cases"), withheld(CE_UNSIGNED));
    assert_eq!(out["contract_evidence"][DERIVATION]["consuming_constant_effort"], true);
    let mut missing = out.clone();
    missing["contract_evidence"].as_object_mut().unwrap().remove(DERIVATION);
    assert_refused(&missing, None, "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED", "missing (constant effort)");
    assert_refused(&missing, Some(&wire["model"]), EVIDENCE, "missing (constant effort)");
}

// ------------------------------------------------------------------ nested rows (review r2 P2)
const NESTED_ROW: &str = "SOURCE_PREVIEW_PHYSICS_COMBINATION_NESTED_ROW";
const NESTED_WITHHELD: &str = "SOURCE_PREVIEW_PHYSICS_COMBINATION_NESTED_WITHHELD";
fn is_of(row: &Value, id: &str) -> bool {
    row["basis_ref"] == json!({"ref_type": "combination", "ref_id": id}) && row["kind"] != "combination_modulus_basis_record"
}
/// The first row of combination `id` of `kind` with a nonzero value.
fn nonzero_row<'a>(out: &'a mut Value, id: &str, kind: &str) -> &'a mut Value {
    out["results"].as_array_mut().unwrap().iter_mut().find(|r| is_of(r, id) && r["kind"] == kind && r["value"] != 0.0)
        .unwrap_or_else(|| panic!("no nonzero {kind} row of {id}"))
}
/// One unit in the last place up: below the readers' former 64ε magnitude guard.
fn ulp_up(row: &mut Value) {
    let value = row["value"].as_f64().unwrap();
    row["value"] = json!(f64::from_bits(value.to_bits() + 1));
}

/// Review r2 P2: both readers re-derive every row of a recorded combination with a
/// combination operand (mechanics, subtraction, range envelope) from its operand rows,
/// with the model's factors or the derivation record's, bit for bit, and refuse a row
/// with no derivable counterpart and a derivable row missing without its withholding.
/// Before the repair both readers admitted tampers (a)-(c) with and without the model
/// and (b) without it.
#[test]
fn pf_readers_re_derive_nested_combination_rows() {
    let mut combinations = linear_combinations().as_array().unwrap().clone();
    combinations.push(sub("dn", "sus+srss", "W2"));
    combinations.push(range("en", &["srss-w", "W1"]));
    let wire = pf_cantilever(false, pf_cases(), Value::Array(combinations));
    let model = Some(&wire["model"]);
    let out = solved(&wire, PreviewSolverMode::SparseInteractive, "nested rows base");
    for id in ["sus+srss", "dn", "en"] {
        assert_eq!(gate(&out, id), admitted(), "{id}");
        assert!(!combination_rows(&out, id).is_empty(), "{id}");
    }
    // (a) A mechanics row with a combination operand restated.
    let mut a = out.clone();
    set_row(&mut a, "sus+srss", "element_local_shear_force_y", "midspan", 500.0, 123456.0);
    for given in [None, model] {
        assert_refused(&a, given, NESTED_ROW, "(a) sus+srss value");
    }
    // (b) Its derivation record's factor on srss-w restated, the rows unchanged.
    let mut b = out.clone();
    derivation_record(&mut b, "sus+srss")["expression"]["terms"][1]["factor"] = json!(99.0);
    assert_refused(&b, None, NESTED_ROW, "(b) sus+srss factor");
    assert_refused(&b, model, EVIDENCE, "(b) sus+srss factor");
    // (c) A subtraction and a range envelope with a combination operand, restated by one ulp.
    for (id, kind) in [("dn", "element_local_bending_moment_z"), ("en", "element_local_bending_moment_z"), ("dn", "displacement_magnitude"),
        ("sus+srss", "displacement_magnitude"), ("sus+srss", "support_reaction_force_magnitude_v2"), ("en", "support_reaction_component_v2")] {
        let mut c = out.clone();
        ulp_up(nonzero_row(&mut c, id, kind));
        for given in [None, model] {
            assert_refused(&c, given, NESTED_ROW, &format!("(c) {id} {kind} + 1 ulp"));
        }
    }
    // (d) A row with no derivable counterpart; a derivable row, and a whole support
    // group's magnitude, missing without a stated withholding; a source reference moved.
    let mut extra = out.clone();
    let mut copied = nonzero_row(&mut extra, "dn", "element_local_shear_force_y").clone();
    copied["id"] = json!("result:combination:dn:invented");
    extra["results"].as_array_mut().unwrap().push(copied);
    let mut missing = out.clone();
    let gone = nonzero_row(&mut missing, "en", "element_local_bending_moment_z")["id"].clone();
    missing["results"].as_array_mut().unwrap().retain(|r| r["id"] != gone);
    let mut group = out.clone();
    group["results"].as_array_mut().unwrap().retain(|r| !(is_of(r, "dn") && r["kind"] == "support_reaction_moment_magnitude_v2"));
    let mut moved = out.clone();
    let row = nonzero_row(&mut moved, "sus+srss", "element_local_bending_moment_z");
    let refs = row["source_result_refs"].as_array_mut().unwrap();
    refs.swap(0, 1);
    for (tampered, what) in [(&extra, "extra row"), (&missing, "missing row"), (&group, "partial support group"), (&moved, "source refs")] {
        for given in [None, model] {
            assert_refused(tampered, given, NESTED_ROW, what);
        }
    }
    // (e) A withholding stated where every row is derivable.
    let mut stated = out.clone();
    stated["diagnostics"].as_array_mut().unwrap().push(json!({"id": "diagnostic:preview-physics:combination-row-withheld:2:dn", "code": "LOAD_COMBINATION_ROW_WITHHELD",
        "severity": "warning", "message": "combination dn withholds 1 combined row(s) that could not be formed: invented", "source": "core/product_physics", "affected_refs": ["dn"]}));
    for given in [None, model] {
        assert_refused(&stated, given, NESTED_WITHHELD, "(e) stated withholding");
    }

    // Positive control with withholdings: an operand's support group withheld whole
    // withholds the dependant's group by name; an operand row missing omits the
    // dependant's row with its LOAD_COMBINATION_SOURCE_RESULT_MISSING. Both readers
    // admit it; either withholding removed is refused.
    let probe = solved_typed(&overflow_model(false, json!([])), PreviewSolverMode::SparseInteractive, "overflow probe");
    let tip = |kind: &str| case_rows(&probe, "B").iter().find(|r| r["kind"] == kind && r["entity_ref"] == "node:tip").unwrap()["value"].as_f64().unwrap().abs();
    let f = f64::MAX / tip("global_nodal_displacement_x").max(tip("global_nodal_displacement_y")).max(tip("global_nodal_displacement_z")) / (1.0 + 2f64.powi(-8));
    let wire = overflow_model(false, json!([
        terms("support", "absolute_sum", &[("case", "A", 1.4e308)]),
        terms("tip", "absolute_sum", &[("case", "B", f)]),
        terms("sup-n", "mechanics", &[("case", "A", 1.0), ("comb", "support", 1.0)]),
        range("env-n", &["tip", "B"]),
    ]));
    let model = Some(&wire["model"]);
    let out = solved_typed(&wire, PreviewSolverMode::SparseInteractive, "nested withholdings");
    let named = |code: &str, id: &str| with_code(&out, code).into_iter().filter(|d| d["affected_refs"][0] == id).count();
    assert_eq!(named("LOAD_COMBINATION_ROW_WITHHELD", "sup-n"), 1);
    assert!(!combination_rows(&out, "sup-n").iter().any(|r| r["entity_ref"] == "support:root"));
    assert!(named("LOAD_COMBINATION_SOURCE_RESULT_MISSING", "env-n") >= 1);
    assert!(!combination_rows(&out, "env-n").iter().any(|r| r["kind"] == "displacement_magnitude" && r["entity_ref"] == "node:tip"));
    for (code, id) in [("LOAD_COMBINATION_ROW_WITHHELD", "sup-n"), ("LOAD_COMBINATION_SOURCE_RESULT_MISSING", "env-n")] {
        let mut unstated = out.clone();
        let first = with_code(&out, code).into_iter().find(|d| d["affected_refs"][0] == id).unwrap()["id"].clone();
        unstated["diagnostics"].as_array_mut().unwrap().retain(|d| d["id"] != first);
        for given in [None, model] {
            assert_refused(&unstated, given, NESTED_WITHHELD, &format!("{id} without its {code}"));
        }
    }
}

/// §10.3, V-8: a document with flat combinations only publishes no derivation, and
/// one stated on it is refused.
#[test]
fn pf_flat_documents_publish_no_derivation() {
    let wire = pf_cantilever(true, pf_cases(), json!([
        terms("flat", "mechanics", &[("case", "W1", 1.0), ("case", "W2", 0.5)]),
        sub("d1", "W1", "W2"),
        range("env", &["W1", "W2"]),
    ]));
    let out = solved(&wire, PreviewSolverMode::SparseInteractive, "flat");
    let mut members: Vec<&String> = out["contract_evidence"].as_object().unwrap().keys().collect();
    members.sort();
    assert_eq!(members, ["combination_gates", "preview_cases"]);
    let mut stated = out.clone();
    stated["contract_evidence"][DERIVATION] = json!({"nonlinear_supports": false, "consuming_constant_effort": true, "combinations": [
        {"combination_id": "d1", "basis": "result_state_subtraction", "expression": {"minuend_id": "W1", "subtrahend_id": "W2"}}]});
    for given in [None, Some(&wire["model"])] {
        assert_refused(&stated, given, EVIDENCE, "stated on a flat document");
    }
}

// ------------------------------------------------------------------ V-10
#[test]
fn v10_the_fixed_load_count_chain_through_the_product() {
    let combinations = json!([
        terms("M0", "mechanics", &[("case", "W1", 1.0), ("case", "W2", T)]),
        sub("D0", "M0", "W1"),
        terms("M1", "mechanics", &[("case", "W1", 1.0), ("comb", "D0", T)]),
        sub("D1", "M1", "W1"),
        terms("M2", "mechanics", &[("case", "W1", 1.0), ("comb", "D1", T)]),
        sub("D2", "M2", "W1"),
        terms("U", "srss", &[("comb", "D2", 1.0)]),
        terms("M3", "mechanics", &[("case", "W1", 1.0), ("comb", "D2", T)]),
        terms("Q", "mechanics", &[("case", "W1", 2.0), ("comb", "D2", T)]),
    ]);
    for ce in [true, false] {
        let wire = pf_cantilever(ce, pf_cases(), combinations.clone());
        for mode in MODES {
            let out = solved_typed(&wire, mode, "V-10");
            for id in ["M0", "D0", "M1", "D1", "M2", "D2"] {
                assert_eq!(gate(&out, id), admitted(), "ce={ce} {id}");
            }
            // κ(D2) = t³ exactly, not 0; M3 and Q are unrepresentable and named.
            let named: Vec<&str> = with_code(&out, UNREPRESENTABLE).iter().map(|d| d["affected_refs"][0].as_str().unwrap()).collect();
            assert_eq!(named, ["M3", "Q"], "ce={ce}");
            assert_eq!(with_code(&out, UNREPRESENTABLE)[0]["severity"], "info");
            if ce {
                assert_eq!(gate(&out, "U"), withheld(CE_UNSIGNED));
                assert_eq!(gate(&out, "M3"), withheld(CE));
                assert_eq!(gate(&out, "Q"), withheld(CE));
            } else {
                for id in ["U", "M3", "Q"] {
                    assert_eq!(gate(&out, id), admitted(), "{id}");
                }
            }
        }
    }
}

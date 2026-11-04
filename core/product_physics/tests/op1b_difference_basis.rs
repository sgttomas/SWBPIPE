//! Operating-case slice 1b (DEL-05-02 Work `operating-case.md` §1.2, §1.3 c5, §1.5,
//! §5 B option 3; DEL-02-01 Work `model-document-0.9.0.md` §2.2): a pressure-free
//! `result_state_subtraction` declaring `difference_basis = "operating_minus_sustained"`,
//! its gates G-1 and G-2, its row sign convention, its G-1 derivation evidence
//! (`contract_evidence.difference_basis_derivation`) and the product readers'
//! verdicts, through the captured value entry, in both solver modes.
//!
//! Expectations are independent: the closed forms and G-1 verdicts of
//! `validation/references/op1b_difference_basis/` (exact rationals, written as
//! binary64 hex), and an expected evidence record written in this file from §2.2's
//! text. Criterion for solved values (DEC-024/DEC-026): |obs − exp| ≤ 1e-9 |exp|, a
//! zero expectation 1e-9 times the case's scale. The derived rows' bits are compared
//! exactly with the plain subtraction of the same operands and with fl(O − S) of the
//! published operand rows; codes, gates, evidence and texts exactly.
use open_pipe_stress_product_physics::{result_export::reader_verdict, run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

#[path = "common/current_model.rs"]
mod current_model;
#[path = "common/condition_twin.rs"]
mod twin;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const REFERENCES: &str = include_str!("../../../validation/references/op1b_difference_basis/references.json");
const L: f64 = 4.0;
const OD: f64 = 0.12;
const WALL: f64 = 0.01;
const K: f64 = 1_048_576.0;
const ALPHA: f64 = 1.0 / 65_536.0;
const PROV: &str = "op1b";
const MEMBER: &str = "difference_basis_derivation";
const G1: &str = "EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING";
const G2: &str = "EXPANSION_BASIS_MODULUS_MIXED";
const DECLARED_SIGN: &str = "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; operating minus sustained difference: a signed difference of two independently solved states, each using its own reported support state and carrying its own numerical standing under the static, history-free friction model; derived from case results; not an equilibrium state";
const PLAIN_SIGN: &str = "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; a signed difference of solved states, not an equilibrium state";
const G1_MESSAGE: &str = "Combination withheld: it declares an operating minus sustained difference, and the subtrahend case's primitive loads, as a multiset by content (category, target, direction, dimension, value and unit after normalization; ids ignored), are not contained in the minuend case's.";
const G2_MESSAGE: &str = "Combination withheld: it declares an operating minus sustained difference, and its two load cases were solved on different modulus bases.";
const UNVERIFIED: &str = "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED";

/// A binary64 written by `float.hex()` (the references), read exactly.
fn hex(text: &str) -> f64 {
    let (sign, rest) = text.strip_prefix('-').map_or((1.0, text), |r| (-1.0, r));
    let rest = rest.strip_prefix("0x").unwrap();
    let (mantissa, exponent) = rest.split_once('p').unwrap();
    let (lead, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let digits = format!("{lead}{fraction}");
    let m = u64::from_str_radix(&digits, 16).unwrap() as f64;
    let e: i32 = exponent.parse::<i32>().unwrap() - 4 * fraction.len() as i32;
    let v = sign * m * 2f64.powi(e);
    assert_eq!(v, sign * m * 2f64.powi(e), "exact");
    v
}
fn references() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}
fn reference(section: &str, name: &str) -> Value {
    references()[section].as_array().unwrap().iter().find(|r| r["name"] == name).unwrap_or_else(|| panic!("{section} {name}")).clone()
}

fn modulus() -> f64 {
    let inner = OD - 2.0 * WALL;
    let area = std::f64::consts::PI / 4.0 * (OD * OD - inner * inner);
    4.0 * K / area
}
fn force(id: &str, f: f64) -> Value {
    json!({"id":id,"category":"concentrated_force","target":{"type":"node","node":"node:b"},"direction":"global_x","magnitude":{"value":f,"unit":"N"},"dimension":"force","provenance":PROV})
}
fn thermal(id: &str, dt: f64) -> Value {
    json!({"id":id,"category":"thermal","target":{"type":"element","pipe":"pipe:a-b"},"direction":"global_x","magnitude":{"value":dt,"unit":"degC"},"dimension":"temperature_interval","provenance":PROV})
}
fn weight(id: &str) -> Value {
    json!({"id":id,"category":"weight","target":{"type":"node","node":"node:b"},"direction":"global_z","magnitude":{"value":-100.0,"unit":"N"},"dimension":"force","provenance":PROV})
}
fn case(id: &str, loads: Value) -> Value {
    json!({"id":id,"primitive_loads":loads,"provenance":PROV})
}
fn gap(id: &str, c: f64) -> Value {
    json!({"id":id,"node":"node:b","family":"nonlinear","restraints":[],"provenance":PROV,
        "nonlinear":{"behavior":"gap","closes_when":"positive_displacement","dof":"UX","gap":{"value":c,"unit":"m"},"initial_state":"inactive"}})
}
fn plain(id: &str, a: &str, b: &str) -> Value {
    json!({"id":id,"basis":"result_state_subtraction","minuend_id":a,"subtrahend_id":b,"provenance":PROV})
}
fn declared(id: &str, a: &str, b: &str) -> Value {
    let mut c = plain(id, a, b);
    c["difference_basis"] = json!("operating_minus_sustained");
    c
}

fn bar(supports: Vec<Value>, cases: Value, combinations: Value) -> Value {
    let mut all = vec![json!({"id":"support:a","node":"node:a","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":PROV})];
    all.extend(supports);
    json!({"model":{"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:op1b","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[{"id":"node:a","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":PROV},{"id":"node:b","position":{"x":L,"y":0.0,"z":0.0},"provenance":PROV}],
        "pipe_segments":[{"id":"pipe:a-b","from":"node:a","to":"node:b","section":{"outside_diameter":{"value":OD,"unit":"m"},"wall_thickness":{"value":WALL,"unit":"m"}},
            "material":"material:m","y_reference":{"x":0.0,"y":1.0,"z":0.0},"provenance":PROV}],
        "supports":all,"components":[],
        "materials":[{"id":"material:m","elastic_modulus":{"value":modulus(),"unit":"Pa"},"shear_modulus":{"value":modulus() / 2.6,"unit":"Pa"},
            "thermal_expansion_coefficient":{"value":ALPHA,"unit":"1/degC"},
            "temperature_points":[{"id":"temperature-point:hot","elastic_modulus":{"value":modulus() / 2.0,"unit":"Pa"},"shear_modulus":{"value":modulus() / 5.2,"unit":"Pa"},
                "thermal_expansion_coefficient":{"value":ALPHA,"unit":"1/degC"},"provenance":"op1b_invented_test_values"}],
            "provenance":"op1b_invented_test_values"}],
        "load_cases":cases,"combinations":combinations},"materials":[]})
}

/// V-1's model: sustained F, operating F plus ΔT = 16 K; the declared difference and
/// its plain twin over the same operands.
fn v1(c: f64, f: f64) -> Value {
    bar(
        vec![gap("support:gap", c)],
        json!([case("case:S", json!([force("f-s", f)])), case("case:O", json!([force("f-o", f), thermal("t-o", 16.0)]))]),
        json!([declared("comb:o-s", "case:O", "case:S"), plain("comb:plain", "case:O", "case:S")]),
    )
}

fn run(wire: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).expect("captured entry")).unwrap()
}
fn solve(wire: &Value, mode: PreviewSolverMode) -> Value {
    let v = run(wire, mode);
    assert_eq!(v["status"]["mechanics"], "MECHANICS_SOLVED", "{:#}", v["diagnostics"]);
    v
}
fn rows(v: &Value) -> &Vec<Value> {
    v["results"].as_array().unwrap()
}
fn combination_rows<'a>(v: &'a Value, id: &str) -> Vec<&'a Value> {
    rows(v).iter().filter(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == id && r["kind"] != "combination_modulus_basis_record").collect()
}
fn value(v: &Value, kind: &str, entity: &str, basis: (&str, &str), component: Option<&str>, location: Option<&str>) -> f64 {
    let hits: Vec<&Value> = rows(v)
        .iter()
        .filter(|r| {
            r["kind"] == kind && r["entity_ref"] == entity && r["basis_ref"]["ref_type"] == basis.0 && r["basis_ref"]["ref_id"] == basis.1
                && component.is_none_or(|c| r["metadata"]["component"] == c)
                && location.is_none_or(|l| r["metadata"]["location"] == l)
        })
        .collect();
    assert_eq!(hits.len(), 1, "{kind} {entity} {basis:?}");
    hits[0]["value"].as_f64().unwrap()
}
fn close(observed: f64, expected: f64, zero_scale: f64, what: &str) {
    let tolerance = if expected != 0.0 { 1e-9 * expected.abs() } else { 1e-9 * zero_scale };
    assert!(observed.is_finite() && (observed - expected).abs() <= tolerance, "{what}: observed {observed:.17e} expected {expected:.17e}");
}
fn gate(v: &Value, id: &str) -> Value {
    v["contract_evidence"]["combination_gates"].as_array().unwrap().iter().find(|g| g["combination_id"] == id).unwrap().clone()
}
fn codes(v: &Value) -> Vec<String> {
    v["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap().to_string()).collect()
}
fn verdict(v: &Value, model: Option<&Value>) -> Value {
    reader_verdict::validate("preview-physics", v, model, None)
}
fn assert_reads(v: &Value, wire: &Value, what: &str) {
    let with = verdict(v, Some(&wire["model"]));
    assert_eq!(with["ok"], true, "{what}: with the model {with}");
    // Fail closed until option A's review: a reader without the model refuses the claim.
    let without = verdict(v, None);
    assert_eq!(without["error"]["code"], UNVERIFIED, "{what}: model-free {without}");
}

/// The evidence record §2.2 implies, written from the authored loads (independent of
/// the producer's normalization code): force in N, a temperature interval in K.
fn content(category: &str, target: Value, direction: &str, dimension: &str, value: f64, unit: &str, count: u64) -> Value {
    json!({"category": category, "target": target, "direction": direction, "dimension": dimension,
           "magnitude": {"value_bits": format!("{:016x}", value.to_bits()), "unit": unit}, "count": count})
}
fn force_content(f: f64, count: u64) -> Value {
    content("concentrated_force", json!({"type":"node","node":"node:b"}), "global_x", "force", f, "N", count)
}
fn thermal_content(dt: f64) -> Value {
    content("thermal", json!({"type":"element","pipe":"pipe:a-b"}), "global_x", "temperature_interval", dt, "K", 1)
}
fn record(id: &str, minuend: (&str, Value), subtrahend: (&str, Value)) -> Value {
    json!({"combination_id": id, "difference_basis": "operating_minus_sustained",
           "minuend": {"load_case_id": minuend.0, "primitive_contents": minuend.1},
           "subtrahend": {"load_case_id": subtrahend.0, "primitive_contents": subtrahend.1}})
}

/// `OP1B_WRITE_CORPUS=<dir>` writes the reader corpus part (RE and Python read
/// `fixtures/results/preview_physics_difference_basis_corpus.json`).
fn write_corpus(part: &str, documents: Vec<Value>) {
    if let Ok(dir) = std::env::var("OP1B_WRITE_CORPUS") {
        let path = std::path::Path::new(&dir).join(format!("op1b_{part}.json"));
        std::fs::write(path, serde_json::to_string_pretty(&json!(documents)).unwrap() + "\n").unwrap();
    }
}

// ------------------------------------------------------------------ V-1
#[test]
fn v1_a_declared_difference_publishes_with_its_sign_evidence_and_the_closed_forms() {
    let mut corpus = Vec::new();
    for name in ["closing", "both_open", "both_active"] {
        let r = reference("v1", name);
        let (c, f) = (hex(r["c"].as_str().unwrap()), hex(r["F"].as_str().unwrap()));
        let wire = v1(c, f);
        for mode in MODES {
            let v = solve(&wire, mode);
            let what = format!("{name} {mode:?}");
            assert_eq!(gate(&v, "comb:o-s"), json!({"combination_id": "comb:o-s", "withheld": false, "reason": null}), "{what}");
            assert!(!codes(&v).iter().any(|c| c.starts_with("EXPANSION_BASIS") || c == "LOAD_COMBINATION_DIFFERENCE_BASIS_UNSUPPORTED"), "{what}");
            // The closed forms, through the declared rows.
            let d = &r["difference"];
            let basis = ("combination", "comb:o-s");
            close(value(&v, "global_nodal_displacement_x", "node:b", basis, None, None), hex(d["u"].as_str().unwrap()) * 1000.0, 1000.0 * f / K, &format!("{what} Δu"));
            close(value(&v, "element_local_axial_force", "pipe:a-b", basis, None, Some("end_j")), hex(d["N"].as_str().unwrap()), f, &format!("{what} ΔN"));
            close(value(&v, "support_reaction_component_v2", "support:gap", basis, Some("Fx"), None), hex(d["stop_fx"].as_str().unwrap()), f, &format!("{what} Δstop"));
            // Every declared row is its plain twin's, bit for bit, with the declared sign;
            // the plain twin keeps the plain sign.
            let (declared_rows, plain_rows) = (combination_rows(&v, "comb:o-s"), combination_rows(&v, "comb:plain"));
            assert_eq!(declared_rows.len(), plain_rows.len(), "{what}");
            assert!(!declared_rows.is_empty());
            for (d, p) in declared_rows.iter().zip(&plain_rows) {
                assert_eq!(d["value"].as_f64().unwrap().to_bits(), p["value"].as_f64().unwrap().to_bits(), "{what} {}", d["id"]);
                assert_eq!((&d["kind"], &d["entity_ref"]), (&p["kind"], &p["entity_ref"]));
                if let Some(sign) = d["metadata"]["sign_convention"].as_str() {
                    assert_eq!(sign, DECLARED_SIGN, "{what} {}", d["id"]);
                    assert_eq!(p["metadata"]["sign_convention"], PLAIN_SIGN);
                    assert_eq!(d["metadata"]["basis"], p["metadata"]["basis"], "metadata.basis is unchanged");
                }
                // fl(O − S) of the cited operand rows.
                let refs = d["source_result_refs"].as_array().unwrap();
                if refs.len() == 2 && d["kind"] != "displacement_magnitude" {
                    let source = |i: usize| rows(&v).iter().find(|r| r["id"] == refs[i]).unwrap();
                    assert_eq!((source(0)["basis_ref"]["ref_id"].as_str(), source(1)["basis_ref"]["ref_id"].as_str()), (Some("case:O"), Some("case:S")));
                    let expected = source(0)["value"].as_f64().unwrap() - source(1)["value"].as_f64().unwrap();
                    assert_eq!(d["value"].as_f64().unwrap().to_bits(), expected.to_bits(), "{what} {}", d["id"]);
                }
            }
            // Slice 1a's operand standing is still published (R-5).
            let operands = v["contract_evidence"]["combination_operands"].as_array().unwrap();
            assert!(operands.iter().any(|o| o["combination_id"] == "comb:o-s"), "{what}");
            // G-1's evidence: both multisets, canonical order, ids not content.
            assert_eq!(
                v["contract_evidence"][MEMBER],
                json!([record("comb:o-s", ("case:O", json!([force_content(f, 1), thermal_content(16.0)])), ("case:S", json!([force_content(f, 1)])))]),
                "{what}"
            );
            assert_reads(&v, &wire, &what);
            if mode == PreviewSolverMode::SparseInteractive {
                corpus.push(json!({"name": format!("v1_{name}"), "model": wire["model"], "document": v}));
            }
        }
    }
    write_corpus("v1", corpus);
}

/// A document without a declared subtraction publishes no member (V-8 byte identity is
/// `pressure_free_byte_identity`'s; this checks the member's absence directly).
#[test]
fn a_plain_subtraction_publishes_no_difference_evidence() {
    let mut wire = v1(2f64.powi(-9), 1536.0);
    wire["model"]["combinations"] = json!([plain("comb:plain", "case:O", "case:S")]);
    for mode in MODES {
        let v = solve(&wire, mode);
        assert!(v["contract_evidence"].get(MEMBER).is_none());
        assert!(!v.to_string().contains("operating minus sustained"));
        assert_eq!(verdict(&v, None)["ok"], true);
        assert_eq!(verdict(&v, Some(&wire["model"]))["ok"], true);
    }
}

// ------------------------------------------------------------------ V-2
/// V-2: the W-16 A-1 propped cantilever, a one-way prop lifting off under anchor rise.
#[test]
fn v2_a_declared_difference_over_a_prop_lift_off() {
    let mut request: Value = serde_json::from_str(include_str!("../../../validation/references/w16_nonlinear_motion/requests/a1.json")).unwrap();
    current_model::reauthor_model(&mut request["model"]);
    let loads = request["model"]["load_cases"][0]["primitive_loads"].as_array().unwrap().clone();
    let point = loads.iter().find(|l| l["category"] == "concentrated_force").unwrap().clone();
    let rise = |id: &str, s: f64| {
        let mut motion = loads.iter().find(|l| l["category"] == "imposed_displacement").unwrap().clone();
        motion["id"] = json!(format!("{id}-S"));
        motion["magnitude"]["value"] = json!(s);
        motion
    };
    let with = |id: &str, extra: Option<Value>| {
        let mut p = point.clone();
        p["id"] = json!(format!("{id}-P"));
        let mut all = vec![p];
        all.extend(extra);
        json!({"id": id, "primitive_loads": all, "provenance": PROV})
    };
    let mut wire = request.clone();
    let (lift, active) = (reference("v2", "lift"), reference("v2", "active"));
    wire["model"]["load_cases"] = json!([with("case:S", None), with("case:O-LIFT", Some(rise("case:O-LIFT", hex(lift["rise"].as_str().unwrap())))),
        with("case:O-ACT", Some(rise("case:O-ACT", hex(active["rise"].as_str().unwrap()))))]);
    wire["model"]["combinations"] = json!([declared("comb:lift", "case:O-LIFT", "case:S"), declared("comb:act", "case:O-ACT", "case:S")]);
    let p_value = point["magnitude"]["value"].as_f64().unwrap().abs();
    let pl = p_value * 4.0;
    let mut corpus = Vec::new();
    for mode in MODES {
        let v = solve(&wire, mode);
        for (id, r) in [("comb:lift", &lift), ("comb:act", &active)] {
            assert_eq!(gate(&v, id)["withheld"], false, "{id} {mode:?}");
            let d = &r["difference"];
            let basis = ("combination", id);
            close(value(&v, "global_nodal_displacement_z", "node:T", basis, None, None), hex(d["u_tip"].as_str().unwrap()) * 1000.0, 1.0, &format!("{id} u"));
            close(value(&v, "support_reaction_component_v2", "support:PROP", basis, Some("Fz"), None), hex(d["R_over_P"].as_str().unwrap()) * p_value, p_value, &format!("{id} R"));
            close(value(&v, "support_reaction_component_v2", "support:ANCHOR", basis, Some("My"), None), hex(d["My_over_PL"].as_str().unwrap()) * pl, pl, &format!("{id} My"));
            assert!(combination_rows(&v, id).iter().all(|r| r["metadata"]["sign_convention"].as_str().is_none_or(|s| s == DECLARED_SIGN)));
        }
        assert_reads(&v, &wire, &format!("v2 {mode:?}"));
        if mode == PreviewSolverMode::SparseInteractive {
            corpus.push(json!({"name": "v2_anchor_rise", "model": wire["model"], "document": v}));
        }
    }
    write_corpus("v2", corpus);
}

// ------------------------------------------------------------------ V-4
/// V-4: a plain S − O negates the declared O − S row for row (finite nonzero values;
/// a zero's sign bit is ignored), and the declared S − O is withheld by G-1 (the
/// operating case's thermal load is not in the sustained case): G-1 reads the operand
/// order, so a reversed-order mutant is killed. Case independence is unchanged by 1b
/// (no solve changes); its seed instrumentation is `op1a_seed_tests`.
#[test]
fn v4_swapping_operands_negates_and_g1_is_not_symmetric() {
    let mut wire = v1(2f64.powi(-9), 1536.0);
    wire["model"]["combinations"] = json!([declared("comb:o-s", "case:O", "case:S"), plain("comb:s-o", "case:S", "case:O"), declared("comb:s-o-declared", "case:S", "case:O")]);
    for mode in MODES {
        let v = solve(&wire, mode);
        let key = |r: &&Value| format!("{}|{}|{}|{}", r["kind"], r["entity_ref"], r["metadata"]["location"], r["metadata"]["component"]);
        let mut a = combination_rows(&v, "comb:o-s");
        let mut b = combination_rows(&v, "comb:s-o");
        a.sort_by_key(key);
        b.sort_by_key(key);
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            if !x["kind"].as_str().unwrap().contains("magnitude") && x["value"].as_f64().unwrap() != 0.0 {
                assert_eq!(y["value"].as_f64().unwrap().to_bits(), (-x["value"].as_f64().unwrap()).to_bits(), "{}", x["id"]);
            }
        }
        assert_eq!(gate(&v, "comb:s-o-declared"), json!({"combination_id": "comb:s-o-declared", "withheld": true, "reason": G1}));
        assert!(combination_rows(&v, "comb:s-o-declared").is_empty());
        assert_reads(&v, &wire, &format!("v4 {mode:?}"));
    }
}

// ------------------------------------------------------------------ V-5
/// Withheld by `code` in both modes; returns the sparse-mode document.
fn withheld_by(wire: &Value, code: &str, message: &str, what: &str) -> Value {
    let mut last = Value::Null;
    for mode in MODES {
        let v = solve(wire, mode);
        assert_eq!(gate(&v, "comb:o-s"), json!({"combination_id": "comb:o-s", "withheld": true, "reason": code}), "{what} {mode:?}");
        let named: Vec<&Value> = v["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code).collect();
        assert_eq!(named.len(), 1, "{what}");
        assert_eq!(named[0]["severity"], "warning");
        assert_eq!(named[0]["message"], message);
        assert_eq!(named[0]["affected_refs"], json!(["comb:o-s"]));
        assert_eq!(named[0]["id"], "diagnostic:preview-physics:combination-gate:8:comb:o-s");
        assert!(combination_rows(&v, "comb:o-s").is_empty(), "{what}: withheld whole");
        assert!(v["contract_evidence"][MEMBER].is_array(), "{what}: a withheld declared subtraction keeps its evidence");
        assert_reads(&v, wire, &format!("{what} {mode:?}"));
        if mode == PreviewSolverMode::SparseInteractive {
            last = v;
        }
    }
    last
}

#[test]
fn v5_g1_withholds_by_content_and_multiplicity_and_admits_other_ids() {
    let f = 1536.0;
    let ulp = hex(reference("g1", "one_ulp")["one_ulp_value"].as_str().unwrap());
    assert_eq!(ulp.to_bits(), f64::to_bits(f) + 1);
    let mut corpus = Vec::new();
    let named = |loads_o: Value, loads_s: Value| {
        bar(vec![gap("support:gap", 2f64.powi(-9))], json!([case("case:S", loads_s), case("case:O", loads_o)]),
            json!([declared("comb:o-s", "case:O", "case:S"), json!({"id":"comb:dependant","basis":"srss","terms":[{"combination":"comb:o-s","factor":1.0}],"provenance":PROV})]))
    };
    // A sustained-only load; a magnitude one ulp away; two identical sustained
    // primitives against one operating primitive.
    for (name, wire) in [
        ("sustained_only_load", named(json!([force("f-o", f), thermal("t-o", 16.0)]), json!([force("f-s", f), weight("w-s")]))),
        ("one_ulp", named(json!([force("f-o", f), thermal("t-o", 16.0)]), json!([force("f-s", ulp)]))),
        ("two_against_one", named(json!([force("f-o", f), thermal("t-o", 16.0)]), json!([force("f-s1", f), force("f-s2", f)]))),
    ] {
        assert_eq!(reference("g1", name)["contained"], false);
        let v = withheld_by(&wire, G1, G1_MESSAGE, name);
        // The dependant is withheld whole by the existing operand rule.
        assert_eq!(gate(&v, "comb:dependant")["reason"], "LOAD_COMBINATION_OPERAND_WITHHELD", "{name}");
        corpus.push(json!({"name": format!("v5_{name}"), "model": wire["model"], "document": v}));
    }
    let two = withheld_by(&named(json!([force("f-o", f), thermal("t-o", 16.0)]), json!([force("f-s1", f), force("f-s2", f)])), G1, G1_MESSAGE, "two");
    assert_eq!(two["contract_evidence"][MEMBER][0]["subtrahend"]["primitive_contents"], json!([force_content(f, 2)]));
    // Two against two, and identical content under other ids and provenance: admitted
    // (a G-1 comparing ids would withhold the second).
    let mut other = force("an-unrelated-id", f);
    other["provenance"] = json!("another provenance");
    for (name, wire) in [
        ("two_against_two", named(json!([force("f-o1", f), force("f-o2", f), thermal("t-o", 16.0)]), json!([force("f-s1", f), force("f-s2", f)]))),
        ("ids_differ", named(json!([force("f-o", f), thermal("t-o", 16.0)]), json!([other]))),
    ] {
        for mode in MODES {
            let v = solve(&wire, mode);
            assert_eq!(gate(&v, "comb:o-s")["withheld"], false, "{name} {mode:?}");
            assert_eq!(gate(&v, "comb:dependant")["withheld"], false, "{name} {mode:?}");
            assert_reads(&v, &wire, name);
            if mode == PreviewSolverMode::SparseInteractive {
                corpus.push(json!({"name": format!("v5_{name}"), "model": wire["model"], "document": v}));
            }
        }
    }
    write_corpus("v5", corpus);
}

#[test]
fn v5_g2_withholds_mixed_modulus_bases_after_g1() {
    let mut wire = v1(2f64.powi(-9), 1536.0);
    wire["model"]["load_cases"][1]["modulus_basis_ref"] = json!("temperature-point:hot");
    let v = withheld_by(&wire, G2, G2_MESSAGE, "g2");
    write_corpus("g2", vec![json!({"name": "v5_g2", "model": wire["model"], "document": v})]);
    // The same basis on both: admitted.
    wire["model"]["load_cases"][0]["modulus_basis_ref"] = json!("temperature-point:hot");
    for mode in MODES {
        let v = solve(&wire, mode);
        assert_eq!(gate(&v, "comb:o-s")["withheld"], false);
        assert_reads(&v, &wire, "g2 same");
    }
    // G-1 precedes G-2: both failing names G-1.
    wire["model"]["load_cases"][0]["modulus_basis_ref"] = Value::Null;
    wire["model"]["load_cases"][0].as_object_mut().unwrap().remove("modulus_basis_ref");
    wire["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(weight("w-s"));
    withheld_by(&wire, G1, G1_MESSAGE, "g1 before g2");
}

#[test]
fn v5_the_member_is_refused_off_a_pf_subtraction_and_minuend_final_stays_withheld() {
    for (combination, code) in [
        (json!({"id":"comb:x","basis":"mechanics","terms":[{"load_case":"case:O","factor":1.0}],"difference_basis":"operating_minus_sustained","provenance":PROV}), "LOAD_COMBINATION_DIFFERENCE_BASIS_MISPLACED"),
        (json!({"id":"comb:x","basis":"range_envelope","operand_ids":["case:O","case:S"],"mode":"max","difference_basis":"operating_minus_sustained","provenance":PROV}), "LOAD_COMBINATION_DIFFERENCE_BASIS_MISPLACED"),
        (json!({"id":"comb:x","basis":"result_state_subtraction","minuend_id":"comb:p","subtrahend_id":"case:S","difference_basis":"operating_minus_sustained","provenance":PROV}), "LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE"),
    ] {
        let mut wire = v1(2f64.powi(-9), 1536.0);
        wire["model"]["combinations"] = json!([plain("comb:p", "case:O", "case:S"), combination]);
        for mode in MODES {
            let v = run(&wire, mode);
            assert_ne!(v["status"]["mechanics"], "MECHANICS_SOLVED", "{code}");
            assert!(v["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == code && d["severity"] == "blocking"), "{code}");
        }
    }
    // minuend_final: withheld by its own code alone (0.9.0 §2.2, §3.8).
    let mut wire = v1(2f64.powi(-9), 1536.0);
    wire["model"]["combinations"][0]["subtrahend_support_state"] = json!("minuend_final");
    for mode in MODES {
        let v = solve(&wire, mode);
        let warnings: Vec<String> = codes(&v).into_iter().filter(|c| c.starts_with("LOAD_COMBINATION_") || c.starts_with("EXPANSION_")).collect();
        assert_eq!(warnings, ["LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED"]);
        assert!(v["contract_evidence"]["combination_gates"].as_array().unwrap().iter().all(|g| g["combination_id"] != "comb:o-s"));
        assert!(v["contract_evidence"].get(MEMBER).is_none());
        assert_eq!(verdict(&v, Some(&wire["model"]))["ok"], true);
    }
    // `own` behaves as absence.
    wire["model"]["combinations"][0]["subtrahend_support_state"] = json!("own");
    for mode in MODES {
        let v = solve(&wire, mode);
        assert_eq!(gate(&v, "comb:o-s")["withheld"], false);
        assert_reads(&v, &wire, "own");
    }
}

/// On the exact route the member stays refused by name (blocking), before any gate.
#[test]
fn v5_the_member_on_the_exact_route_is_refused_by_name() {
    let mut request: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json")).unwrap();
    current_model::reauthor_model(&mut request["model"]);
    let cases: Vec<String> = request["model"]["load_cases"].as_array().unwrap().iter().map(|c| c["id"].as_str().unwrap().to_string()).collect();
    assert!(cases.len() >= 2 || !cases.is_empty());
    let (a, b) = (cases[0].clone(), cases.get(1).cloned().unwrap_or_else(|| cases[0].clone()));
    request["model"]["combinations"] = json!([declared("comb:o-s", &a, &b)]);
    for mode in MODES {
        let v = run(&request, mode);
        assert_ne!(v["status"]["mechanics"], "MECHANICS_SOLVED");
        assert!(v["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED" && d["severity"] == "blocking"));
        assert!(!codes(&v).iter().any(|c| c.starts_with("EXPANSION_BASIS")));
    }
}

/// A non-converged or blocked operand case blocks the whole document (c4, unchanged):
/// two gaps closing on one DOF are refused before any combination is formed.
#[test]
fn v5_a_blocked_operand_blocks_the_document() {
    let mut wire = v1(2f64.powi(-12), 1536.0);
    wire["model"]["supports"].as_array_mut().unwrap().push(gap("support:gap-2", 2f64.powi(-12)));
    for mode in MODES {
        let v = run(&wire, mode);
        assert_ne!(v["status"]["mechanics"], "MECHANICS_SOLVED", "{:#}", v["diagnostics"]);
        assert!(v["contract_evidence"].get(MEMBER).is_none_or(Value::is_null));
        assert!(rows(&v).is_empty());
    }
}

// ------------------------------------------------------------------ readers
/// Decision M1: with the model, the reader cannot re-derive G-1 where an operand case
/// carries a `condition_use` (the lowering adds its primitives), so it refuses by name
/// and never falls back to the model-free reading.
#[test]
fn the_model_bound_reader_refuses_g1_over_condition_use_by_name() {
    let document = twin::document("0.9.0");
    let declared_ids: Vec<&Value> = document["combinations"].as_array().unwrap().iter().filter(|c| c.get("difference_basis").is_some()).collect();
    assert!(!declared_ids.is_empty(), "the S2 document declares a difference");
    let wire = json!({"model": document, "materials": []});
    for mode in MODES {
        let v = solve(&wire, mode);
        let with = verdict(&v, Some(&wire["model"]));
        assert_eq!(with["error"]["code"], UNVERIFIED, "{with}");
        assert_eq!(verdict(&v, None)["error"]["code"], UNVERIFIED);
    }
}

/// With the model, evidence that differs from the model's lowering, a forged gate and
/// forged sign texts are refused.
#[test]
fn the_model_bound_reader_refuses_forged_evidence_gates_and_signs() {
    let wire = v1(2f64.powi(-9), 1536.0);
    let model = &wire["model"];
    let v = solve(&wire, PreviewSolverMode::SparseInteractive);
    let refused = |forged: &Value, code: &str, what: &str| {
        let got = verdict(forged, Some(model));
        assert_eq!(got["error"]["code"], code, "{what}: {got}");
    };
    let mut bit = v.clone();
    let entry = &mut bit["contract_evidence"][MEMBER][0]["subtrahend"]["primitive_contents"][0]["magnitude"]["value_bits"];
    *entry = json!(format!("{:016x}", 1536f64.to_bits() + 1));
    refused(&bit, "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_MODEL", "a value bit");
    let mut unit = v.clone();
    unit["contract_evidence"][MEMBER][0]["minuend"]["primitive_contents"][1]["magnitude"]["unit"] = json!("degC");
    refused(&unit, "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_MODEL", "a unit");
    let mut missing = v.clone();
    missing["contract_evidence"].as_object_mut().unwrap().remove(MEMBER);
    refused(&missing, "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_MODEL", "missing evidence");
    let mut swapped = v.clone();
    let record = swapped["contract_evidence"][MEMBER][0].clone();
    swapped["contract_evidence"][MEMBER][0]["minuend"] = record["subtrahend"].clone();
    swapped["contract_evidence"][MEMBER][0]["subtrahend"] = record["minuend"].clone();
    refused(&swapped, "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_MODEL", "swapped cases");
    let mut gate = v.clone();
    for g in gate["contract_evidence"]["combination_gates"].as_array_mut().unwrap() {
        if g["combination_id"] == "comb:o-s" {
            *g = json!({"combination_id": "comb:o-s", "withheld": true, "reason": G1});
        }
    }
    gate["results"] = json!(rows(&v).iter().filter(|r| r["basis_ref"]["ref_id"] != "comb:o-s" || r["kind"] == "combination_modulus_basis_record").cloned().collect::<Vec<_>>());
    // The slice-1a operand record and notices of an admitted combination go with it.
    gate["contract_evidence"]["combination_operands"].as_array_mut().unwrap().retain(|r| r["combination_id"] != "comb:o-s");
    gate["diagnostics"].as_array_mut().unwrap().retain(|d| d["affected_refs"][0] != "comb:o-s");
    refused(&gate, "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_DERIVATION", "a forged G-1 withholding");
    let mut sign = v.clone();
    for r in sign["results"].as_array_mut().unwrap() {
        if r["basis_ref"]["ref_id"] == "comb:plain" && r["metadata"]["sign_convention"] == PLAIN_SIGN {
            r["metadata"]["sign_convention"] = json!(DECLARED_SIGN);
        }
    }
    refused(&sign, "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_ROW", "the declared sign on a plain subtraction");
    let mut plain_sign = v.clone();
    for r in plain_sign["results"].as_array_mut().unwrap() {
        if r["basis_ref"]["ref_id"] == "comb:o-s" && r["metadata"]["sign_convention"] == DECLARED_SIGN {
            r["metadata"]["sign_convention"] = json!(PLAIN_SIGN);
        }
    }
    refused(&plain_sign, "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_ROW", "the plain sign on a declared subtraction");
}

/// The committed reader corpus is this test file's own output (sparse mode).
#[test]
fn committed_reader_corpus_is_the_producer_output() {
    let corpus: Value = serde_json::from_str(include_str!("../../../fixtures/results/preview_physics_difference_basis_corpus.json")).unwrap();
    let documents = corpus["documents"].as_array().unwrap();
    assert!(documents.len() >= 9);
    for entry in documents {
        let wire = json!({"model": entry["model"], "materials": []});
        assert_eq!(solve(&wire, PreviewSolverMode::SparseInteractive), entry["document"], "{}", entry["name"]);
    }
}

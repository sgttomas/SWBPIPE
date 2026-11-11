//! RT-A5 `minuend_final`: a pressure-free
//! `result_state_subtraction` declaring `difference_basis: operating_minus_sustained` and
//! `subtrahend_support_state: minuend_final`, through the captured value entry, in both
//! solver modes.
//!
//! Expectations are independent: `validation/references/op_mf_held_status/` (exact
//! rationals, written as binary64 hex; H-1 to H-18), and records written here from §7.3,
//! §7.5 and §7.6's text. Criterion for solved values (DEC-024/DEC-026): |obs − exp| ≤
//! 1e-9 |exp|, a zero expectation 1e-9 times the case's scale. Classifications, outcomes,
//! codes and texts are compared exactly.
use open_pipe_stress_product_physics::{result_export::reader_verdict, run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const REFERENCES: &str = include_str!("../../../validation/references/op_mf_held_status/references.json");
const L: f64 = 4.0;
const OD: f64 = 0.12;
const WALL: f64 = 0.01;
const K: f64 = 1_048_576.0;
const ALPHA: f64 = 1.0 / 65_536.0;
const PROV: &str = "op_mf";
const COMB: &str = "comb:o-s-held";
const MEMBER: &str = "held_status_derivation";
const RELEASED: &str = "HELD_STATUS_RELEASED";
const HELD_SIGN: &str = "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; operating minus sustained difference at the operating support status: the minuend case's own solved state minus the subtrahend case's loads solved with each nonlinear support held at the minuend's final status, a held-constraint comparison that can need a reaction a unilateral support cannot supply and is not an admissible sustained state, under the static, history-free friction model; derived from case results; not an equilibrium state";
const CLAUSE: &str = "held-status operand of comb:o-s-held: the subtrahend case's loads solved with the minuend's final support status held; not the subtrahend case's result";
const ESTABLISHED: &str = "Established within its stated numerical bound for the explicitly held system at this case's loads. The verdict does not establish this case's own support state, an admissible sustained state, a load history or uniqueness.";

/// A binary64 written by `float.hex()` (the references), read exactly.
fn hex(text: &str) -> f64 {
    let (sign, rest) = text.strip_prefix('-').map_or((1.0, text), |r| (-1.0, r));
    let rest = rest.strip_prefix("0x").unwrap();
    let (mantissa, exponent) = rest.split_once('p').unwrap();
    let (lead, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let digits = format!("{lead}{fraction}");
    let m = u64::from_str_radix(&digits, 16).unwrap() as f64;
    let e: i32 = exponent.parse::<i32>().unwrap() - 4 * fraction.len() as i32;
    sign * m * 2f64.powi(e)
}
fn reference(name: &str) -> Value {
    let all: Value = serde_json::from_str(REFERENCES).unwrap();
    all["cases"][name].clone()
}
fn bits(v: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap())
}

fn modulus() -> f64 {
    let inner = OD - 2.0 * WALL;
    let area = std::f64::consts::PI / 4.0 * (OD * OD - inner * inner);
    4.0 * K / area
}
fn fx(id: &str, node: &str, f: f64) -> Value {
    json!({"id":id,"category":"concentrated_force","target":{"type":"node","node":node},"direction":"global_x","magnitude":{"value":f,"unit":"N"},"dimension":"force","provenance":PROV})
}
fn fz(id: &str, f: f64) -> Value {
    json!({"id":id,"category":"concentrated_force","target":{"type":"node","node":"node:b"},"direction":"global_z","magnitude":{"value":f,"unit":"N"},"dimension":"force","provenance":PROV})
}
fn thermal(id: &str, dt: f64) -> Value {
    json!({"id":id,"category":"thermal","target":{"type":"element","pipe":"pipe:a-b"},"direction":"global_x","magnitude":{"value":dt,"unit":"degC"},"dimension":"temperature_interval","provenance":PROV})
}
fn ground(id: &str, g: f64) -> Value {
    json!({"id":id,"category":"imposed_displacement","target":{"type":"support","support":"support:t","dof":"UX"},"direction":"UX","magnitude":{"value":g,"unit":"m"},"dimension":"displacement","provenance":PROV})
}
fn case(id: &str, loads: Vec<Value>) -> Value {
    json!({"id":id,"primitive_loads":loads,"provenance":PROV})
}
fn nonlinear(spec: Value) -> Value {
    json!({"id":"support:t","node":"node:b","family":"nonlinear","restraints":[],"provenance":PROV,"nonlinear":spec})
}
fn one_way() -> Value {
    nonlinear(json!({"behavior":"one_way","dof":"UX","initial_state":"active","active_when":"positive_reaction"}))
}
fn gap(c: f64) -> Value {
    nonlinear(json!({"behavior":"gap","closes_when":"positive_displacement","dof":"UX","gap":{"value":c,"unit":"m"},"initial_state":"inactive"}))
}
fn friction(mu: f64, normal: Option<f64>) -> Value {
    let mut spec = json!({"behavior":"friction","dof":"UX","initial_state":"sticking","friction_coefficient":{"value":mu,"unit":"1"}});
    match normal {
        Some(n) => spec["normal_reaction"] = json!({"value":n,"unit":"N"}),
        None => spec["normal_reaction_source"] = json!({"support_ref":"support:z","dof":"UZ"}),
    }
    nonlinear(spec)
}
/// The tip-UZ bar's normal source: a linear UZ restraint at the tip.
fn tip_uz() -> Value {
    json!({"id":"support:z","node":"node:b","family":"line_stop","restraints":["UZ"],"provenance":PROV})
}
fn held_combination() -> Value {
    json!({"id":COMB,"basis":"result_state_subtraction","minuend_id":"case:O","subtrahend_id":"case:S",
        "difference_basis":"operating_minus_sustained","subtrahend_support_state":"minuend_final","provenance":PROV})
}

/// The bar of V-1 (L = 4 m, EA/L = 2^20 N/m, alpha = 2^-16 /K) along X, anchored at A.
fn bar(supports: Vec<Value>, o: Vec<Value>, s: Vec<Value>, combinations: Value) -> Value {
    let mut all = vec![json!({"id":"support:a","node":"node:a","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":PROV})];
    all.extend(supports);
    json!({"model":{"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:op-mf","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[{"id":"node:a","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":PROV},{"id":"node:b","position":{"x":L,"y":0.0,"z":0.0},"provenance":PROV}],
        "pipe_segments":[{"id":"pipe:a-b","from":"node:a","to":"node:b","section":{"outside_diameter":{"value":OD,"unit":"m"},"wall_thickness":{"value":WALL,"unit":"m"}},
            "material":"material:m","y_reference":{"x":0.0,"y":1.0,"z":0.0},"provenance":PROV}],
        "supports":all,"components":[],
        "materials":[{"id":"material:m","elastic_modulus":{"value":modulus(),"unit":"Pa"},"shear_modulus":{"value":modulus() / 2.6,"unit":"Pa"},
            "thermal_expansion_coefficient":{"value":ALPHA,"unit":"1/degC"},"provenance":"op_mf_invented_test_values"}],
        "load_cases":[case("case:S", s), case("case:O", o)],"combinations":combinations},"materials":[]})
}

/// Each bar case of the references as a product document: the target S and the source O
/// (O's loads contain S's, so G-1 admits the comparison), and the reference's own name.
fn bar_case(name: &str) -> Value {
    let f = |id: &str, v: f64| fx(id, "node:b", v);
    let anchor = |id: &str, v: f64| fx(id, "node:a", v);
    let comb = json!([held_combination()]);
    match name {
        "H-2" => bar(vec![one_way()], vec![f("f-o", 512.0), thermal("t-o", -16.0)], vec![f("f-s", 512.0)], comb),
        "H-3" | "H-4" | "H-5" => {
            let dt = match name { "H-3" => -24.0, "H-4" => -8.0, _ => -64.0 };
            bar(vec![friction(0.25, Some(4096.0))], vec![f("f-o", 2048.0), thermal("t-o", dt)], vec![f("f-s", 2048.0)], comb)
        }
        "H-6" | "H-6b" => {
            let (fs, dt) = if name == "H-6" { (2048.0, -80.0) } else { (3072.0, -8.0) };
            bar(vec![tip_uz(), friction(0.25, None)],
                vec![fz("v-o", -4096.0), fz("v2-o", -4096.0), f("f-o", fs), thermal("t-o", dt)],
                vec![fz("v-s", -4096.0), f("f-s", fs)], comb)
        }
        "H-13" => bar(vec![tip_uz(), friction(0.25, None)],
            vec![fz("u-o", 4096.0), fz("v2-o", -8192.0), f("f-o", 2048.0), thermal("t-o", -8.0)],
            vec![fz("u-s", 4096.0), f("f-s", 2048.0)], comb),
        "H-7" => bar(vec![gap(1.0 / 512.0)], vec![f("f-o", 1536.0), thermal("t-o", 16.0)], vec![f("f-s", 1536.0)], comb),
        "H-15a" => bar(vec![friction(0.0, Some(4096.0))], vec![f("f-o", 512.0), thermal("t-o", -16.0)], vec![f("f-s", 512.0)], comb),
        "H-15b" => bar(vec![friction(0.0, Some(4096.0))], vec![anchor("f-o", 512.0), thermal("t-o", -16.0)], vec![anchor("f-s", 512.0)], comb),
        "H-15c" => bar(vec![friction(f64::from_bits(1), Some(0.25))], vec![f("f-o", 512.0), thermal("t-o", -16.0)], vec![f("f-s", 512.0)], comb),
        "H-16a" => bar(vec![friction(0.25, Some(0.0))], vec![f("f-o", 512.0), thermal("t-o", -16.0)], vec![f("f-s", 512.0)], comb),
        "H-16b" => bar(vec![tip_uz(), friction(0.25, None)],
            vec![fz("v-o", -4096.0), fz("u-o", 4096.0), f("f-o", 512.0)],
            vec![fz("v-s", -4096.0), f("f-s", 512.0)], comb),
        "H-17c" => bar(vec![friction(0.25, Some(4096.0))],
            vec![f("f-o", 768.0), f("f2-o", 1280.0), ground("g-o", 3.0 / 2048.0)],
            vec![f("f-s", 768.0)], comb),
        "H-18" => bar(vec![one_way()], vec![anchor("f-o", 512.0), thermal("t-o", 16.0)], vec![anchor("f-s", 512.0)], comb),
        other => panic!("{other}"),
    }
}

/// The L-frame of H-14 (the references' generator): A(0,0,0), B(1,0,0) and C(1,0,-1) in the XZ
/// plane, EA = 2^24 and EI = 2^20 (OD 0.8 m and wall 0.1 m give I/A = 1/16 exactly in reals);
/// A anchored, C's UZ restrained (the derived normal's source) and friction on C's UX,
/// mu = 1/4, initially sticking. S: W = 4096 N down at B and P = 2048 N +X at C.
fn l_frame(o: Vec<Value>) -> Value {
    let (od, wall) = (0.8, 0.1);
    let inner = od - 2.0 * wall;
    let area = std::f64::consts::PI / 4.0 * (od * od - inner * inner);
    let e = 16_777_216.0 / area;
    let section = json!({"outside_diameter":{"value":od,"unit":"m"},"wall_thickness":{"value":wall,"unit":"m"}});
    let pipe = |id: &str, from: &str, to: &str| json!({"id":id,"from":from,"to":to,"section":section,"material":"material:m","y_reference":{"x":0.0,"y":1.0,"z":0.0},"provenance":PROV});
    let node = |id: &str, x: f64, z: f64| json!({"id":id,"position":{"x":x,"y":0.0,"z":z},"provenance":PROV});
    let s = vec![fz("w-s", -4096.0), fx("p-s", "node:c", 2048.0)];
    json!({"model":{"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:op-mf-l-frame","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[node("node:a", 0.0, 0.0), node("node:b", 1.0, 0.0), node("node:c", 1.0, -1.0)],
        "pipe_segments":[pipe("pipe:a-b", "node:a", "node:b"), pipe("pipe:b-c", "node:b", "node:c")],
        "supports":[
            {"id":"support:a","node":"node:a","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":PROV},
            {"id":"support:z","node":"node:c","family":"line_stop","restraints":["UZ"],"provenance":PROV},
            {"id":"support:t","node":"node:c","family":"nonlinear","restraints":[],"provenance":PROV,
                "nonlinear":{"behavior":"friction","dof":"UX","initial_state":"sticking","friction_coefficient":{"value":0.25,"unit":"1"},
                    "normal_reaction_source":{"support_ref":"support:z","dof":"UZ"}}}],
        "components":[],
        "materials":[{"id":"material:m","elastic_modulus":{"value":e,"unit":"Pa"},"shear_modulus":{"value":e / 2.6,"unit":"Pa"},
            "thermal_expansion_coefficient":{"value":ALPHA,"unit":"1/degC"},"provenance":"op_mf_invented_test_values"}],
        "load_cases":[case("case:S", s), case("case:O", o)],"combinations":[held_combination()]},"materials":[]})
}

/// H-14 (O: S and dT = +64 K on AB) and H-14r (O: S and P' = -4096 N at C).
fn l_frame_case(name: &str) -> Value {
    let s = || vec![fz("w-o", -4096.0), fx("p-o", "node:c", 2048.0)];
    match name {
        "H-14" => l_frame([s(), vec![thermal("t-o", 64.0)]].concat()),
        "H-14r" => l_frame([s(), vec![fx("p2-o", "node:c", -4096.0)]].concat()),
        other => panic!("{other}"),
    }
}

/// A reference case's product document and the node of its held support.
fn case_wire(name: &str) -> (Value, &'static str) {
    if name.starts_with("H-14") { (l_frame_case(name), "node:c") } else { (bar_case(name), "node:b") }
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
fn codes(v: &Value) -> Vec<String> {
    v["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap().to_string()).collect()
}
fn record(v: &Value) -> Value {
    let records = v["contract_evidence"][MEMBER].as_array().unwrap_or_else(|| panic!("no {MEMBER}: {:#}", v["diagnostics"]));
    assert_eq!(records.len(), 1);
    records[0].clone()
}
fn gate(v: &Value) -> Value {
    v["contract_evidence"]["combination_gates"].as_array().unwrap().iter().find(|g| g["combination_id"] == COMB).unwrap().clone()
}
/// The tip's UX displacement row (m) under `basis`: a case, the combination, or a held row.
fn tip_ux(v: &Value, basis: (&str, &str), held: bool) -> f64 {
    node_ux(v, "node:b", basis, held)
}
/// A node's UX displacement row (m) under `basis`.
fn node_ux(v: &Value, node: &str, basis: (&str, &str), held: bool) -> f64 {
    let hits: Vec<&Value> = rows(v)
        .iter()
        .filter(|r| {
            r["kind"] == "global_nodal_displacement_x" && r["entity_ref"] == node && r["basis_ref"]["ref_type"] == basis.0
                && r["basis_ref"]["ref_id"] == basis.1 && r["id"].as_str().unwrap().contains(":held:") == held
        })
        .collect();
    assert_eq!(hits.len(), 1, "{basis:?} held={held}");
    let scale = match hits[0]["unit"].as_str().unwrap() { "mm" => 1e-3, "m" => 1.0, u => panic!("{u}") };
    hits[0]["value"].as_f64().unwrap() * scale
}
fn close(observed: f64, expected: f64, zero_scale: f64, what: &str) {
    let tolerance = if expected != 0.0 { 1e-9 * expected.abs() } else { 1e-9 * zero_scale };
    assert!(observed.is_finite() && (observed - expected).abs() <= tolerance, "{what}: observed {observed:.17e} expected {expected:.17e}");
}
fn verdict(v: &Value, model: Option<&Value>) -> Value {
    reader_verdict::validate("preview-physics", v, model, None)
}
fn assert_reads(v: &Value, wire: &Value, what: &str) {
    let with = verdict(v, Some(&wire["model"]));
    assert_eq!(with["ok"], true, "{what}: with the model {with:#}");
    let without = verdict(v, None);
    assert_eq!(without["ok"], true, "{what}: model-free {without:#}");
}

/// The held record's one support entry.
fn held_entry(v: &Value) -> Value {
    let r = record(v);
    let held = r["held"].as_array().unwrap();
    assert_eq!(held.len(), 1, "{r:#}");
    held[0].clone()
}

/// What the reference's outcome note admits: (released, gap).
fn expected_outcome(note: &str) -> (bool, bool) {
    (note.starts_with('R') || note.contains("and R"), note.starts_with('G'))
}

/// §7.2 and §7.3 against H-2 to H-18's bar cases: the classified state, the release, the
/// held displacement and reaction, the difference row, the standing and the disclosure,
/// in both modes; both readers admit the document.
#[test]
fn held_bar_cases_match_the_references_in_both_modes() {
    for name in ["H-2", "H-3", "H-4", "H-5", "H-6", "H-6b", "H-13", "H-15a", "H-15b", "H-15c", "H-16a", "H-16b", "H-17c", "H-18"] {
        let r = reference(name);
        let wire = bar_case(name);
        let (released, _) = expected_outcome(r["outcome"].as_str().unwrap());
        for mode in MODES {
            let what = format!("{name} {mode:?}");
            let v = solve(&wire, mode);
            assert_eq!(gate(&v)["withheld"], false, "{what}: {:#}", v["diagnostics"]);
            let rec = record(&v);
            assert_eq!(rec["outcome"], "admitted", "{what}");
            assert_eq!(rec["source"]["load_case_id"], "case:O");
            assert_eq!(rec["target"]["load_case_id"], "case:S");
            let entry = held_entry(&v);
            let th = &r["target_held"];
            assert_eq!(entry["held_state"], th["state"], "{what}: held state");
            assert_eq!(entry["classified_state"], th["classified_state"], "{what}: classified");
            assert_eq!(entry["released"] != "none", released, "{what}: released {entry:#}");
            assert_eq!(th["released"], released, "{what}: the reference's own release");
            if th["derived_branch_admissible"] == false {
                assert_eq!(entry["released"], "branch", "{what}");
                assert_eq!(entry["inputs"]["branch_admissible"], false, "{what}");
            }
            let scale = 1e-3;
            close(tip_ux(&v, ("combination", COMB), true), hex(th["u"]["binary64"].as_str().unwrap()), scale, &format!("{what}: held u"));
            close(bits(&entry["inputs"]["reaction_bits"]), hex(th["reaction"]["binary64"].as_str().unwrap()), 1.0, &format!("{what}: held reaction"));
            close(tip_ux(&v, ("load_case", "case:O"), false), hex(r["source_own"]["u"]["binary64"].as_str().unwrap()), scale, &format!("{what}: source u"));
            close(tip_ux(&v, ("load_case", "case:S"), false), hex(r["target_own"]["u"]["binary64"].as_str().unwrap()), scale, &format!("{what}: target own u"));
            close(tip_ux(&v, ("combination", COMB), false), hex(r["source_minus_held"]["du"]["binary64"].as_str().unwrap()), scale, &format!("{what}: O - S_held"));
            let kind = rec["standing"]["kind"].as_str().unwrap();
            if released {
                assert_eq!(kind, "not_established", "{what}");
                assert!(codes(&v).contains(&RELEASED.to_string()), "{what}");
                assert!(rec.get("conditions").is_none(), "{what}");
            } else {
                assert!(matches!(kind, "established" | "not_established"), "{what}: {kind}");
                assert!(!codes(&v).contains(&RELEASED.to_string()), "{what}");
                assert_eq!(rec.get("conditions").is_some(), kind == "established", "{what}");
                if kind == "established" {
                    assert_eq!(rec["standing"]["reason"], ESTABLISHED, "{what}");
                }
            }
            if name == "H-16a" {
                assert_eq!(kind, "established", "{what}: an input decision");
                assert_eq!(rec["conditions"][0]["class"], "input_decision", "{what}");
            }
            if name == "H-18" {
                assert_eq!(kind, "not_established", "{what}: its separation ball contains 0");
            }
            if name == "H-15c" {
                assert_eq!(entry["inputs"]["coefficient_bits"], format!("{:016x}", 1u64), "{what}");
            }
            // The rows: held rows under the combination, typed, with the clause; the
            // difference rows with the held sign; nothing under a case is held.
            for row in rows(&v).iter().filter(|r| r["kind"] != "combination_modulus_basis_record") {
                let id = row["id"].as_str().unwrap();
                if id.contains(":held:") {
                    assert_eq!(row["basis_ref"], json!({"ref_type":"combination","ref_id":COMB}), "{what}: {id}");
                    assert_eq!(row["metadata"]["basis"], "held_status_operand", "{what}: {id}");
                    assert!(row["metadata"]["sign_convention"].as_str().unwrap().ends_with(CLAUSE), "{what}: {id}");
                } else if row["basis_ref"]["ref_id"] == COMB && !row["metadata"].is_null() {
                    assert_eq!(row["metadata"]["sign_convention"], HELD_SIGN, "{what}: {id}");
                    assert_ne!(row["metadata"]["basis"], "held_status_operand", "{what}: {id}");
                }
            }
            assert_reads(&v, &wire, &what);
        }
    }
}

/// The mutants: the product's held solve is not the frozen source normal, an
/// uncoupled normal, or a hold of the source's displacement.
#[test]
fn the_held_solve_kills_the_reference_mutants() {
    for name in ["H-6", "H-6b", "H-17c"] {
        let r = reference(name);
        let wire = bar_case(name);
        for mode in MODES {
            let v = solve(&wire, mode);
            let u = tip_ux(&v, ("combination", COMB), true);
            let expected = hex(r["target_held"]["u"]["binary64"].as_str().unwrap());
            for (mutant, values) in r["mutants"].as_object().unwrap() {
                let wrong = hex(values["u"]["binary64"].as_str().unwrap());
                assert!((wrong - expected).abs() > 1e-6 * expected.abs().max(1e-6), "{name} {mutant}: the mutant must differ from the reference");
                assert!((u - wrong).abs() > 1e-6 * wrong.abs().max(1e-6), "{name} {mode:?}: the product matches the {mutant} mutant");
            }
        }
    }
}

fn reference_value(v: &Value) -> f64 {
    hex(v["binary64"].as_str().unwrap())
}

/// H-14 and H-14r, the strongly coupled L-frame (coupling g = dR_n/dphi = -24/19): the
/// held friction force follows the held solve's own normal, not the source's normal (frozen
/// mutant) and not the normal of the frame without the friction force (uncoupled mutant), in
/// both modes; H-14 keeps its state, H-14r is released by its slip sense.
#[test]
fn h14_the_coupled_l_frame_matches_the_references_and_kills_its_mutants() {
    for name in ["H-14", "H-14r"] {
        let r = reference(name);
        let (wire, node) = case_wire(name);
        let (released, _) = expected_outcome(r["outcome"].as_str().unwrap());
        let th = &r["target_held"];
        for mode in MODES {
            let what = format!("{name} {mode:?}");
            let v = solve(&wire, mode);
            let rec = record(&v);
            assert_eq!(rec["outcome"], "admitted", "{what}: {rec:#}");
            let entry = held_entry(&v);
            assert_eq!(entry["held_state"], th["state"], "{what}: held state");
            assert_eq!(entry["classified_state"], th["classified_state"], "{what}: classified");
            assert_eq!(entry["released"] != "none", released, "{what}: released {entry:#}");
            assert_eq!(entry["inputs"]["normal_source"], "derived", "{what}");
            assert_eq!(entry["inputs"]["normal_source_ref"], json!({"support":"support:z","node":"node:c","dof":"UZ"}), "{what}");
            assert_eq!(entry["inputs"]["beta"], th["beta"], "{what}: beta");
            assert_eq!(entry["inputs"]["slip_sense"], th["slip_sense"], "{what}: slip sense");
            assert_eq!(entry["inputs"]["branch_admissible"], th["derived_branch_admissible"], "{what}: branch");
            let held_u = node_ux(&v, node, ("combination", COMB), true);
            close(held_u, reference_value(&th["u"]), 1e-3, &format!("{what}: held u_C"));
            close(bits(&entry["inputs"]["reaction_bits"]), reference_value(&th["reaction"]), 1.0, &format!("{what}: held reaction"));
            close(bits(&entry["inputs"]["normal_trial_bits"]), reference_value(&th["normal_signed"]).abs(), 1.0, &format!("{what}: held normal"));
            let applied = bits(&entry["inputs"]["applied_force_bits"]);
            close(applied, reference_value(&th["friction_force"]), 1.0, &format!("{what}: held friction force"));
            close(node_ux(&v, node, ("load_case", "case:O"), false), reference_value(&r["source_own"]["u"]), 1e-3, &format!("{what}: source u_C"));
            close(node_ux(&v, node, ("load_case", "case:S"), false), reference_value(&r["target_own"]["u"]), 1e-3, &format!("{what}: target own u_C"));
            close(node_ux(&v, node, ("combination", COMB), false), reference_value(&r["source_minus_held"]["du"]), 1e-3, &format!("{what}: O - S_held"));
            let kind = rec["standing"]["kind"].as_str().unwrap();
            if released {
                assert_eq!(kind, "not_established", "{what}");
                assert!(codes(&v).contains(&RELEASED.to_string()), "{what}");
                assert_eq!(rec["held_solve"]["converged"], false, "{what}");
            } else {
                assert!(matches!(kind, "established" | "not_established"), "{what}: {kind}");
                assert!(!codes(&v).contains(&RELEASED.to_string()), "{what}");
                assert_eq!(rec["held_solve"]["converged"], true, "{what}");
            }
            // The mutants. In H-14 the source's and the held normal coincide (dR_n = 0), so
            // the frozen-normal mutant is the reference there and only H-14r can kill it.
            let mut killed = Vec::new();
            for (mutant, values) in r["mutants"].as_object().unwrap() {
                let (wrong_u, wrong_phi) = (reference_value(&values["u"]), reference_value(&values["friction_force"]));
                let distinct = (wrong_u - reference_value(&th["u"])).abs() > 1e-6 * wrong_u.abs();
                if name == "H-14" && mutant == "frozen_source_normal" {
                    assert!(!distinct, "{what}: H-14's frozen mutant is its reference (k = 1)");
                    continue;
                }
                assert!(distinct, "{what} {mutant}: the mutant must differ from the reference");
                assert!((held_u - wrong_u).abs() > 1e-6 * wrong_u.abs(), "{what}: the product matches the {mutant} mutant's u_C");
                if (wrong_phi - reference_value(&th["friction_force"])).abs() > 1e-6 * wrong_phi.abs() {
                    assert!((applied - wrong_phi).abs() > 1e-6 * wrong_phi.abs(), "{what}: the product matches the {mutant} mutant's force");
                }
                killed.push(mutant.as_str());
            }
            let expected: &[&str] = if name == "H-14" { &["uncoupled_normal"] } else { &["frozen_source_normal", "uncoupled_normal"] };
            assert_eq!(killed, expected, "{what}");
            assert_reads(&v, &wire, &what);
        }
    }
}

/// §7.9's linear held twins: each reference model with its held support replaced by its held
/// linear boundary (Active or Sticking: a rigid restraint on its DOF; Sliding: its applied
/// friction force; Inactive: nothing), the subtrahend's loads only, solved on the linear
/// route. Its rows equal S_held's closed forms by the 1e-9 rule, as do the held-operand rows;
/// bit equality between the routes is not claimed.
#[test]
fn linear_held_twins_reproduce_the_held_closed_forms() {
    let names = ["H-2", "H-3", "H-4", "H-5", "H-6", "H-6b", "H-13", "H-15a", "H-15b", "H-15c", "H-16a", "H-16b", "H-17c", "H-14", "H-14r"];
    for name in names {
        let r = reference(name);
        let th = &r["target_held"];
        let (wire, node) = case_wire(name);
        let mut twin = wire.clone();
        let model = &mut twin["model"];
        model["supports"].as_array_mut().unwrap().retain(|s| s["id"] != "support:t");
        let mut s_loads = model["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == "case:S").unwrap()["primitive_loads"].clone();
        match th["state"].as_str().unwrap() {
            "active" | "sticking" => {
                model["supports"].as_array_mut().unwrap().push(json!({"id":"support:twin","node":node,"family":"line_stop","restraints":["UX"],"provenance":PROV}));
            }
            "sliding" => {
                let phi = reference_value(&th["friction_force"]);
                if phi != 0.0 {
                    s_loads.as_array_mut().unwrap().push(fx("phi-s", node, phi));
                }
            }
            "inactive" => {}
            other => panic!("{name}: {other}"),
        }
        model["load_cases"] = json!([case("case:S", serde_json::from_value(s_loads).unwrap())]);
        model["combinations"] = json!([]);
        for mode in MODES {
            let what = format!("{name} {mode:?} twin");
            let solved = solve(&twin, mode);
            let twin_u = node_ux(&solved, node, ("load_case", "case:S"), false);
            close(twin_u, reference_value(&th["u"]), 1e-3, &what);
            let held = solve(&wire, mode);
            close(node_ux(&held, node, ("combination", COMB), true), twin_u, 1e-3, &format!("{what}: the held-operand row"));
        }
    }
}

/// H-7g, the product witness for G: a strict-gap source held in the target is never
/// confirmed; the standing is gap inspection, the decision strict, and the release is
/// disclosed.
#[test]
fn h7g_a_held_gap_is_gap_inspection_and_never_confirmed() {
    let r = reference("H-7g");
    let wire = gap_chain();
    for mode in MODES {
        let what = format!("H-7g {mode:?}");
        let v = solve(&wire, mode);
        let rec = record(&v);
        assert_eq!(rec["outcome"], "admitted", "{what}: {rec:#}");
        assert_eq!(rec["source"]["qualification"], "qualified", "{what}");
        assert_eq!(rec["standing"]["kind"], "gap_inspection", "{what}");
        assert!(rec.get("conditions").is_none(), "{what}");
        let entry = held_entry(&v);
        assert_eq!(entry["held_state"], r["target_held"]["state"], "{what}");
        assert_eq!(entry["classified_state"], r["target_held"]["classified_state"], "{what}");
        assert_eq!(entry["decision"], "strict_gap", "{what}: {entry:#}");
        assert_eq!(entry["released"], "changed_state", "{what}");
        assert!(codes(&v).contains(&RELEASED.to_string()), "{what}");
        assert_reads(&v, &wire, &what);
    }
}

/// NI `p_tests.rs` `gap_chain(1)` as a product model: a unit cantilever along X with
/// EI = 100 (E chosen from the section), the tip's UX restrained, a tip gap on UZ closing
/// negative with c = 2^-12; S: 2^-6 N toward the gap; O: S and 1 N more.
fn gap_chain() -> Value {
    let inner = OD - 2.0 * WALL;
    // Fourth powers by explicit, fixed-order products: `powi` is not guaranteed to round the
    // same on every platform and optimization level (CI's opt-level=2 Linux build differed
    // from the committed corpus by an ulp in E), and the corpus must be reproducible.
    let fourth = |x: f64| { let x2 = x * x; x2 * x2 };
    let i = std::f64::consts::PI / 64.0 * (fourth(OD) - fourth(inner));
    let e = 100.0 / i;
    let load = |id: &str, f: f64| json!({"id":id,"category":"concentrated_force","target":{"type":"node","node":"node:b"},"direction":"global_z","magnitude":{"value":f,"unit":"N"},"dimension":"force","provenance":PROV});
    json!({"model":{"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:op-mf-gap","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[{"id":"node:a","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":PROV},{"id":"node:b","position":{"x":1.0,"y":0.0,"z":0.0},"provenance":PROV}],
        "pipe_segments":[{"id":"pipe:a-b","from":"node:a","to":"node:b","section":{"outside_diameter":{"value":OD,"unit":"m"},"wall_thickness":{"value":WALL,"unit":"m"}},
            "material":"material:m","y_reference":{"x":0.0,"y":1.0,"z":0.0},"provenance":PROV}],
        "supports":[
            {"id":"support:a","node":"node:a","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":PROV},
            {"id":"support:x","node":"node:b","family":"line_stop","restraints":["UX"],"provenance":PROV},
            {"id":"support:t","node":"node:b","family":"nonlinear","restraints":[],"provenance":PROV,
                "nonlinear":{"behavior":"gap","closes_when":"negative_displacement","dof":"UZ","gap":{"value":1.0 / 4096.0,"unit":"m"},"initial_state":"inactive"}}],
        "components":[],
        "materials":[{"id":"material:m","elastic_modulus":{"value":e,"unit":"Pa"},"shear_modulus":{"value":e / 2.6,"unit":"Pa"},
            "thermal_expansion_coefficient":{"value":ALPHA,"unit":"1/degC"},"provenance":"op_mf_invented_test_values"}],
        "load_cases":[case("case:S", vec![load("p-s", -1.0 / 64.0)]), case("case:O", vec![load("p-o", -1.0 / 64.0), load("p2-o", -1.0)])],
        "combinations":[held_combination()]},"materials":[]})
}

/// A non-converged held result (a release) publishes as not converged with the loop's
/// exit diagnostic, and the `converged || held` branch never promotes it (manager's
/// condition at `solve.converged || held`).
#[test]
fn a_released_held_result_is_published_unconverged_and_never_promoted() {
    let wire = bar_case("H-2");
    for mode in MODES {
        let v = solve(&wire, mode);
        let rec = record(&v);
        assert_eq!(rec["held_solve"]["converged"], false, "{mode:?}: {rec:#}");
        assert_eq!(rec["held_solve"]["exit_diagnostic"]["code"], "NONLINEAR_SUPPORT_NONCONVERGENCE", "{mode:?}");
        assert_eq!(rec["standing"]["kind"], "not_established", "{mode:?}");
        assert!(rec.get("conditions").is_none());
        // The held rows are published; none is a certified (`_v2`) row and no case row is held.
        let held: Vec<&Value> = rows(&v).iter().filter(|r| r["id"].as_str().unwrap().contains(":held:")).collect();
        assert!(!held.is_empty());
        assert!(held.iter().all(|r| !r["kind"].as_str().unwrap().ends_with("_v2") || r["kind"] == "support_reaction_component_v2"));
        // The held solver's diagnostics are not republished as case diagnostics.
        let held_exit = rec["held_solve"]["exit_diagnostic"]["message"].as_str().unwrap();
        assert!(v["diagnostics"].as_array().unwrap().iter().all(|d| d["message"] != held_exit), "{mode:?}");
    }
    // The branch itself: the held flag only widens which result is published.
    let lib = include_str!("../src/lib.rs");
    assert_eq!(lib.matches("Some(solve) if solve.converged || held => displacements = solve.displacements.clone(),").count(), 1);
    let held_status = include_str!("../src/held_status.rs");
    assert!(held_status.contains("(Outcome::Established, Confirmation::Established(state)) => NonlinearStanding::Confirmed(state),"));
}

/// Repeat calls are byte-identical, and the target case's own rows are the rows of the
/// document without the combination (it never fakes case rows).
#[test]
fn repeat_calls_are_identical_and_the_case_rows_are_untouched() {
    for name in ["H-2", "H-4", "H-13"] {
        let wire = bar_case(name);
        let mut bare = wire.clone();
        bare["model"]["combinations"] = json!([]);
        for mode in MODES {
            let first = solve(&wire, mode);
            let second = solve(&wire, mode);
            assert_eq!(first["results"], second["results"], "{name} {mode:?}");
            assert_eq!(first["contract_evidence"], second["contract_evidence"], "{name} {mode:?}");
            assert_eq!(first["diagnostics"], second["diagnostics"], "{name} {mode:?}");
            let without = solve(&bare, mode);
            let case_rows = |v: &Value| rows(v).iter().filter(|r| r["basis_ref"]["ref_type"] == "load_case").cloned().collect::<Vec<_>>();
            assert_eq!(case_rows(&first), case_rows(&without), "{name} {mode:?}");
        }
    }
}

/// §7.5's held gates, each withholding the combination by name with an empty record; the
/// solve of every other combination and case publishes.
#[test]
fn the_held_gates_withhold_by_name() {
    // SOURCE_UNAVAILABLE: no nonlinear support, so the minuend has no state to lend.
    let mut linear = bar_case("H-2");
    linear["model"]["supports"].as_array_mut().unwrap().truncate(1);
    // SLIDING_GROUND_DIFFERS: the source slides with its ground moved; the target's ground is not.
    let sliding = bar(vec![friction(0.25, Some(4096.0))],
        vec![fx("f-o", "node:b", 2048.0), ground("g-o", 1.0 / 1024.0), thermal("t-o", -64.0)],
        vec![fx("f-s", "node:b", 2048.0)], json!([held_combination()]));
    for (wire, code) in [(linear, "HELD_STATUS_SOURCE_UNAVAILABLE"), (sliding, "HELD_STATUS_SLIDING_GROUND_DIFFERS")] {
        for mode in MODES {
            let v = solve(&wire, mode);
            assert_eq!(gate(&v), json!({"combination_id":COMB,"withheld":true,"reason":code}), "{code} {mode:?}: {:#}", v["diagnostics"]);
            let rec = record(&v);
            assert_eq!(rec["outcome"], code);
            assert_eq!(rec["held"], json!([]));
            assert!(rec.get("conditions").is_none());
            assert_eq!(rec["standing"]["kind"], "withheld");
            assert!(rows(&v).iter().all(|r| r["basis_ref"]["ref_id"] != COMB || r["kind"] == "combination_modulus_basis_record"), "{code}: no combination row");
            assert!(codes(&v).contains(&code.to_string()));
            assert_reads(&v, &wire, code);
        }
    }
    // G-1 precedes the held gates: a target whose loads O does not contain reaches no held gate.
    let g1 = bar(vec![one_way()], vec![fx("f-o", "node:b", 512.0)], vec![fx("f-s", "node:b", 1024.0)], json!([held_combination()]));
    let v = solve(&g1, PreviewSolverMode::SparseInteractive);
    assert_eq!(gate(&v)["reason"], "EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING");
    assert!(v["contract_evidence"].get(MEMBER).is_none());
}

const CORPUS_DESCRIPTION: &str = "RT-A5 minuend_final reader corpus (DEL-05-02 Work operating-case.md section 7.6): actual producer output, sparse mode, of core/product_physics/tests/op_mf_held_status.rs. Regenerate with OP_MF_WRITE_CORPUS=<file> on that test (committed_reader_corpus_is_the_producer_output) and copy the file here.";

/// The reader corpus: every outcome (E, N, R by a changed state and by a branch, G) and two
/// held gates, as the producer publishes them.
fn corpus_documents() -> Vec<(&'static str, Value)> {
    let mut linear = bar_case("H-2");
    linear["model"]["supports"].as_array_mut().unwrap().truncate(1);
    let sliding = bar(vec![friction(0.25, Some(4096.0))],
        vec![fx("f-o", "node:b", 2048.0), ground("g-o", 1.0 / 1024.0), thermal("t-o", -64.0)],
        vec![fx("f-s", "node:b", 2048.0)], json!([held_combination()]));
    vec![
        ("h2_released", bar_case("H-2")),
        ("h4_kept_explicit", bar_case("H-4")),
        ("h5_slip_sense", bar_case("H-5")),
        ("h6b_kept_derived", bar_case("H-6b")),
        ("h13_branch", bar_case("H-13")),
        ("h15c_zero_limit", bar_case("H-15c")),
        ("h16a_input_decision", bar_case("H-16a")),
        ("h17c_moved_ground", bar_case("H-17c")),
        ("h7g_gap", gap_chain()),
        ("source_unavailable", linear),
        ("sliding_ground_differs", sliding),
    ]
}

/// The committed reader corpus is this test file's own output (sparse mode);
/// `OP_MF_WRITE_CORPUS=<file>` writes it.
#[test]
fn committed_reader_corpus_is_the_producer_output() {
    let documents: Vec<Value> = corpus_documents()
        .into_iter()
        .map(|(name, wire)| json!({"name": name, "model": wire["model"], "document": solve(&wire, PreviewSolverMode::SparseInteractive)}))
        .collect();
    let corpus = json!({"description": CORPUS_DESCRIPTION, "documents": documents});
    if let Ok(path) = std::env::var("OP_MF_WRITE_CORPUS") {
        std::fs::write(path, serde_json::to_string_pretty(&corpus).unwrap() + "\n").unwrap();
        return;
    }
    let committed: Value = serde_json::from_str(include_str!("../../../fixtures/results/preview_physics_held_status_corpus.json")).unwrap();
    assert_eq!(committed, corpus);
}

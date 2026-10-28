//! Operating-case slice 1a (DEL-05-02 Design `pf-nonlinear-operand-standing.md`):
//! a subtraction or range envelope of a pressure-free model with nonlinear supports
//! states each operand's numerical standing and names the supports whose reported
//! state differs between its operands. Every model goes through the captured value
//! entry the desktop uses, in both solver modes.
//!
//! The expectations are independent closed forms, not product output: an axial bar
//! along X anchored at x = 0, L = 4 m, k = EA/L = 2^20 N/m (E is chosen from the
//! annulus area so that EA = 2^22 N), α = 2^-16 /K, so a temperature change of
//! 16 K grows the free bar by δ_T = α ΔT L = 2^-10 m. With the tip gap closing at
//! c (u ≥ c), the tip force F and the free growth δ_T:
//! - gap open: u = δ_T + F/k, N = k(u − δ_T) = F;
//! - gap closed: u = c, N = k(c − δ_T), stop on the pipe R = N − F.
//!
//! With a tip friction support (explicit normal 4096 N, μ = 0.25, capacity 1024 N,
//! zero reference): sticking needs |k(0 − δ_T) − F| ≤ 1024 N; otherwise it slides
//! with R = −1024 N along the slip, u = δ_T + (F + R)/k.
//!
//! Criterion: |observed − expected| ≤ 1e-9 |expected|; an exactly-zero expectation
//! uses 1e-9 times the case's force scale (N) or 1e-9 times c (mm). The state codes,
//! the evidence records and the diagnostics are compared exactly.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const L: f64 = 4.0;
const OD: f64 = 0.12;
const WALL: f64 = 0.01;
const K: f64 = 1_048_576.0; // 2^20 N/m
const ALPHA: f64 = 1.0 / 65_536.0; // 2^-16 /K
const STATES_DIFFER: &str = "COMBINATION_OPERAND_SUPPORT_STATES_DIFFER";
const NOT_PASSED: &str = "COMBINATION_OPERAND_CHECKS_NOT_PASSED";
const STANDING_TAIL: &str = "; its rows are still published, as derived inspection results without their own numerical enclosure; each operand's standing is stated in contract_evidence.combination_operands";
const STATES_TAIL: &str = ". Each operand case is solved independently from the model's authored initial support states, under the static, history-free support model, and keeps its own reported support state; the combination's rows are derived from those case results and are not one solved state.";

fn p(e: i32) -> f64 {
    2f64.powi(e)
}

/// E with E·A = 2^22 N for the authored annulus (independent of product code).
fn modulus() -> f64 {
    let inner = OD - 2.0 * WALL;
    let area = std::f64::consts::PI / 4.0 * (OD * OD - inner * inner);
    4.0 * K / area
}

fn force(id: &str, f: f64) -> Value {
    json!({"id":id,"category":"concentrated_force","target":{"type":"node","node":"node:b"},"direction":"global_x","magnitude":{"value":f,"unit":"N"},"dimension":"force","provenance":"op1a"})
}
fn thermal(id: &str, dt: f64) -> Value {
    json!({"id":id,"category":"thermal","target":{"type":"element","pipe":"pipe:a-b"},"direction":"global_x","magnitude":{"value":dt,"unit":"degC"},"dimension":"temperature_interval","provenance":"op1a"})
}
fn case(id: &str, loads: Value) -> Value {
    json!({"id":id,"primitive_loads":loads,"provenance":"op1a"})
}
fn gap(c: f64) -> Value {
    json!({"id":"support:gap","node":"node:b","family":"nonlinear","restraints":[],"provenance":"op1a",
        "nonlinear":{"behavior":"gap","closes_when":"positive_displacement","dof":"UX","gap":{"value":c,"unit":"m"},"initial_state":"inactive"}})
}
fn friction() -> Value {
    json!({"id":"support:shoe","node":"node:b","family":"nonlinear","restraints":[],"provenance":"op1a",
        "nonlinear":{"behavior":"friction","dof":"UX","initial_state":"sticking","friction_coefficient":{"value":0.25,"unit":"none"},"normal_reaction":{"value":4096.0,"unit":"N"}}})
}
fn subtraction(id: &str, a: &str, b: &str) -> Value {
    json!({"id":id,"basis":"result_state_subtraction","minuend_id":a,"subtrahend_id":b,"provenance":"op1a"})
}
fn range(id: &str, operands: &[&str]) -> Value {
    json!({"id":id,"basis":"range_envelope","operand_ids":operands,"mode":"max","provenance":"op1a"})
}
fn mechanics(id: &str, a: &str, b: &str) -> Value {
    json!({"id":id,"basis":"mechanics","terms":[{"load_case":a,"factor":1.0},{"load_case":b,"factor":1.0}],"provenance":"op1a"})
}

fn bar(extra: Vec<Value>, cases: Value, combinations: Value) -> Value {
    let mut supports = vec![json!({"id":"support:a","node":"node:a","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"op1a"})];
    supports.extend(extra);
    json!({"model":{"schema_version":"0.8.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:op1a","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[{"id":"node:a","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"op1a"},{"id":"node:b","position":{"x":L,"y":0.0,"z":0.0},"provenance":"op1a"}],
        "pipe_segments":[{"id":"pipe:a-b","from":"node:a","to":"node:b","section":{"outside_diameter":{"value":OD,"unit":"m"},"wall_thickness":{"value":WALL,"unit":"m"}},
            "material":"material:m","y_reference":{"x":0.0,"y":1.0,"z":0.0},"provenance":"op1a"}],
        "supports":supports,"components":[],
        "materials":[{"id":"material:m","elastic_modulus":{"value":modulus(),"unit":"Pa"},"shear_modulus":{"value":modulus() / 2.6,"unit":"Pa"},
            "thermal_expansion_coefficient":{"value":ALPHA,"unit":"1/degC"},"provenance":"op1a_invented_test_values"}],
        "load_cases":cases,"combinations":combinations},"materials":[]})
}

/// V-1: sustained F; operating F plus ΔT = 16 K; O−S, a range over both, and a
/// mechanics combination (withheld).
fn v1(c: f64, f: f64) -> Value {
    bar(
        vec![gap(c)],
        json!([case("case:S", json!([force("f-s", f)])), case("case:O", json!([force("f-o", f), thermal("t-o", 16.0)]))]),
        json!([subtraction("comb:o-s", "case:O", "case:S"), range("comb:range", &["case:S", "case:O"]), mechanics("comb:sum", "case:S", "case:O")]),
    )
}

/// V-3: sustained F = 2048 N; operating F plus cooling ΔT = −8 K.
fn v3() -> Value {
    bar(
        vec![friction()],
        json!([case("case:S", json!([force("f-s", 2048.0)])), case("case:O", json!([force("f-o", 2048.0), thermal("t-o", -8.0)]))]),
        json!([subtraction("comb:o-s", "case:O", "case:S")]),
    )
}

fn solve(wire: &Value, mode: PreviewSolverMode) -> Value {
    let envelope = run_linear_static_preview_value_with_mode(wire.clone(), mode).expect("captured entry");
    let v = serde_json::to_value(envelope).unwrap();
    assert_eq!(v["status"]["mechanics"], "MECHANICS_SOLVED", "{:#}", v["diagnostics"]);
    v
}

fn rows(v: &Value) -> &Vec<Value> {
    v["results"].as_array().unwrap()
}
fn row<'a>(v: &'a Value, kind: &str, entity: &str, basis: (&str, &str), component: Option<&str>, location: Option<&str>) -> &'a Value {
    let hits: Vec<&Value> = rows(v)
        .iter()
        .filter(|r| {
            r["kind"] == kind && r["entity_ref"] == entity && r["basis_ref"]["ref_type"] == basis.0 && r["basis_ref"]["ref_id"] == basis.1
                && component.is_none_or(|c| r["metadata"]["component"] == c)
                && location.is_none_or(|l| r["metadata"]["location"] == l)
        })
        .collect();
    assert_eq!(hits.len(), 1, "{kind} {entity} {basis:?} {component:?} {location:?}");
    hits[0]
}
fn value(v: &Value, kind: &str, entity: &str, basis: (&str, &str), component: Option<&str>, location: Option<&str>) -> f64 {
    row(v, kind, entity, basis, component, location)["value"].as_f64().unwrap()
}
fn close(observed: f64, expected: f64, zero_scale: f64, what: &str) {
    let tolerance = if expected != 0.0 { 1e-9 * expected.abs() } else { 1e-9 * zero_scale };
    assert!(observed.is_finite() && (observed - expected).abs() <= tolerance, "{what}: observed {observed:.17e} expected {expected:.17e}");
}

/// Tip displacement (mm), axial force at end_j (N, tension positive), state code and
/// the support's Fx on the pipe (N) of one case or combination.
struct State {
    u_mm: f64,
    n: f64,
    support_fx: f64,
}
fn state(v: &Value, basis: (&str, &str), support: &str) -> State {
    State {
        u_mm: value(v, "global_nodal_displacement_x", "node:b", basis, None, None),
        n: value(v, "element_local_axial_force", "pipe:a-b", basis, None, Some("end_j")),
        support_fx: value(v, "support_reaction_component_v2", support, basis, Some("Fx"), None),
    }
}
fn check(v: &Value, basis: (&str, &str), support: &str, u_m: f64, n: f64, fx: f64, scale: f64, what: &str) {
    let s = state(v, basis, support);
    close(s.u_mm, u_m * 1000.0, scale * 1000.0 / K, &format!("{what} u"));
    close(s.n, n, scale, &format!("{what} N"));
    close(s.support_fx, fx, scale, &format!("{what} support Fx"));
}
fn state_code(v: &Value, case: &str, support: &str) -> f64 {
    value(v, "nonlinear_support_active_set_state_code", support, ("load_case", case), None, None)
}

fn diagnostics<'a>(v: &'a Value, code: &str) -> Vec<&'a Value> {
    v["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code).collect()
}
fn identity(id: &str) -> String {
    format!("{}:{id}", id.len())
}
fn quality(v: &Value, case: &str) -> Value {
    v["numerical_quality"]["cases"].as_array().unwrap().iter().find(|c| c["basis_ref"]["ref_id"] == case).unwrap().clone()
}
fn operand(v: &Value, case: &str) -> Value {
    json!({"load_case_id": case, "modulus_basis_ref": null, "modulus_basis_temperature": null, "numerical_quality": quality(v, case)})
}

/// The records an independent reading of §2.1 expects for V-1's model.
fn expected_records(v: &Value) -> Value {
    json!([
        {"combination_id": "comb:o-s", "basis": "result_state_subtraction", "expression": {"minuend_id": "case:O", "subtrahend_id": "case:S"},
         "operands": [operand(v, "case:O"), operand(v, "case:S")], "standing": "derived_from_case_results"},
        {"combination_id": "comb:range", "basis": "range_envelope", "expression": {"operand_ids": ["case:S", "case:O"], "mode": "max"},
         "operands": [operand(v, "case:O"), operand(v, "case:S")], "standing": "derived_from_case_results"},
    ])
}

fn not_passed_message(id: &str, failing: &[(&str, &str)]) -> String {
    let named = failing.iter().map(|(c, q)| format!("{c} ({q})")).collect::<Vec<_>>().join(", ");
    format!("combination {id} uses operand case(s) whose numerical integrity checks did not pass: {named}{STANDING_TAIL}")
}

#[test]
fn v1_gap_closing_under_thermal_growth_states_operand_standing_and_the_state_change() {
    let c = p(-9);
    let mut corpus = Vec::new();
    for mode in MODES {
        let v = solve(&v1(c, 1536.0), mode);
        // Cases: sustained open, operating closed (closed forms in the module header).
        assert_eq!(state_code(&v, "case:S", "support:gap"), 0.0);
        assert_eq!(state_code(&v, "case:O", "support:gap"), 1.0);
        check(&v, ("load_case", "case:S"), "support:gap", 3.0 * p(-11), 1536.0, 0.0, 1536.0, "S");
        check(&v, ("load_case", "case:O"), "support:gap", c, 1024.0, -512.0, 1536.0, "O");
        check(&v, ("combination", "comb:o-s"), "support:gap", p(-11), -512.0, -512.0, 1536.0, "O-S");
        // The range envelope selects each row's maximum across the operands.
        check(&v, ("combination", "comb:range"), "support:gap", c, 1536.0, 0.0, 1536.0, "range max");
        // The mechanics combination stays withheld.
        assert!(rows(&v).iter().all(|r| r["basis_ref"]["ref_id"] != "comb:sum"));
        assert_eq!(diagnostics(&v, "NONLINEAR_COMBINATION_REQUIRES_SOLVE").len(), 1);

        // §2.1: the records, with the cases' own quality records verbatim.
        assert_eq!(v["contract_evidence"]["combination_operands"], expected_records(&v));
        // Both operands are gap-bearing, so neither passes (DEL-04-01 kd5 §0, R1).
        for case in ["case:O", "case:S"] {
            assert_ne!(quality(&v, case)["solve_quality"], "checks_passed", "{case}");
        }
        // §2.2: one state notice and one standing notice per combination.
        let states = diagnostics(&v, STATES_DIFFER);
        assert_eq!(states.len(), 2);
        for (id, notice) in ["comb:o-s", "comb:range"].iter().zip(&states) {
            assert_eq!(notice["id"], format!("diagnostic:preview-physics:combination-operand-states:{}", identity(id)));
            assert_eq!(notice["severity"], "info");
            assert_eq!(notice["source"], "core/product_physics");
            assert_eq!(notice["affected_refs"], json!([id, "support:gap"]));
            assert_eq!(
                notice["message"],
                format!("combination {id}: the reported final state of 1 nonlinear support(s) differs between its operand cases: support:gap (case:O: active, case:S: inactive){STATES_TAIL}")
            );
        }
        let standing = diagnostics(&v, NOT_PASSED);
        assert_eq!(standing.len(), 2);
        for (id, notice) in ["comb:o-s", "comb:range"].iter().zip(&standing) {
            let failing = [("case:O", quality(&v, "case:O")["solve_quality"].as_str().unwrap().to_string()), ("case:S", quality(&v, "case:S")["solve_quality"].as_str().unwrap().to_string())];
            let failing: Vec<(&str, &str)> = failing.iter().map(|(c, q)| (*c, q.as_str())).collect();
            assert_eq!(notice["id"], format!("diagnostic:preview-physics:combination-operand-standing:{}", identity(id)));
            assert_eq!(notice["severity"], "warning");
            assert_eq!(notice["affected_refs"], json!([id, "case:O", "case:S"]));
            assert_eq!(notice["message"], not_passed_message(id, &failing));
        }
        // R-5: published with standing, never withheld for it.
        assert!(rows(&v).iter().any(|r| r["basis_ref"]["ref_id"] == "comb:o-s"));
        if mode == PreviewSolverMode::SparseInteractive {
            corpus.push(json!({"name": "v1_gap_closes", "mode": "sparse_interactive", "model": v1(c, 1536.0)["model"], "document": v}));
        }
    }
    write_corpus("v1", corpus);
}

#[test]
fn v1_twins_equal_states_publish_no_state_notice() {
    for mode in MODES {
        // c = 2^-8: neither case closes; O−S is the free thermal growth.
        let v = solve(&v1(p(-8), 1536.0), mode);
        assert_eq!((state_code(&v, "case:S", "support:gap"), state_code(&v, "case:O", "support:gap")), (0.0, 0.0));
        check(&v, ("combination", "comb:o-s"), "support:gap", p(-10), 0.0, 0.0, 1536.0, "open O-S");
        assert!(diagnostics(&v, STATES_DIFFER).is_empty());
        assert_eq!(diagnostics(&v, NOT_PASSED).len(), 2, "gap-bearing operands still do not pass");
        assert_eq!(v["contract_evidence"]["combination_operands"], expected_records(&v));
        // F = 2560 N: closed in both; equal codes, yet O−S is not a thermal-only result.
        let v = solve(&v1(p(-9), 2560.0), mode);
        assert_eq!((state_code(&v, "case:S", "support:gap"), state_code(&v, "case:O", "support:gap")), (1.0, 1.0));
        check(&v, ("load_case", "case:S"), "support:gap", p(-9), 2048.0, -512.0, 2560.0, "S closed");
        check(&v, ("load_case", "case:O"), "support:gap", p(-9), 1024.0, -1536.0, 2560.0, "O closed");
        check(&v, ("combination", "comb:o-s"), "support:gap", 0.0, -1024.0, -1024.0, 2560.0, "closed O-S");
        assert!(diagnostics(&v, STATES_DIFFER).is_empty());
    }
}

#[test]
fn v3_friction_slides_in_both_cases_static_history_free() {
    let mut corpus = Vec::new();
    for mode in MODES {
        let v = solve(&v3(), mode);
        assert_eq!((state_code(&v, "case:S", "support:shoe"), state_code(&v, "case:O", "support:shoe")), (3.0, 3.0));
        check(&v, ("load_case", "case:S"), "support:shoe", p(-10), 1024.0, -1024.0, 2048.0, "S");
        check(&v, ("load_case", "case:O"), "support:shoe", p(-11), 1024.0, -1024.0, 2048.0, "O");
        // As solved today, each case independent: ΔN = 0 (a sequential history would give +512 N).
        check(&v, ("combination", "comb:o-s"), "support:shoe", -p(-11), 0.0, 0.0, 2048.0, "O-S");
        assert!(diagnostics(&v, STATES_DIFFER).is_empty());
        let records = &v["contract_evidence"]["combination_operands"];
        assert_eq!(records, &json!([{"combination_id": "comb:o-s", "basis": "result_state_subtraction",
            "expression": {"minuend_id": "case:O", "subtrahend_id": "case:S"}, "operands": [operand(&v, "case:O"), operand(&v, "case:S")],
            "standing": "derived_from_case_results"}]));
        // The notice follows the copied standing exactly.
        let failing: Vec<(&str, String)> = ["case:O", "case:S"].iter()
            .map(|c| (*c, quality(&v, c)["solve_quality"].as_str().unwrap().to_string()))
            .filter(|(_, q)| q != "checks_passed").collect();
        let notices = diagnostics(&v, NOT_PASSED);
        if failing.is_empty() {
            assert!(notices.is_empty());
        } else {
            let failing: Vec<(&str, &str)> = failing.iter().map(|(c, q)| (*c, q.as_str())).collect();
            assert_eq!(notices.len(), 1);
            assert_eq!(notices[0]["message"], not_passed_message("comb:o-s", &failing));
        }
        if mode == PreviewSolverMode::SparseInteractive {
            corpus.push(json!({"name": "v3_friction", "mode": "sparse_interactive", "model": v3()["model"], "document": v}));
        }
    }
    write_corpus("v3", corpus);
}

/// Codex r1 finding 1: two operands that both slide, in opposite senses, differ in
/// reported state. Sustained +2048 N, operating −2048 N: S slides + (u = 2^-10 m,
/// friction −1024 N on the pipe), O slides − (u = −2^-10 m, +1024 N). The notice names
/// the slip senses, for the subtraction and the range envelope alike.
fn slip_reversal() -> Value {
    bar(
        vec![friction()],
        json!([case("case:S", json!([force("f-s", 2048.0)])), case("case:O", json!([force("f-o", -2048.0)]))]),
        json!([subtraction("comb:o-s", "case:O", "case:S"), range("comb:range", &["case:S", "case:O"])]),
    )
}

#[test]
fn opposite_slip_senses_differ_in_reported_state() {
    let mut corpus = Vec::new();
    for mode in MODES {
        let v = solve(&slip_reversal(), mode);
        assert_eq!((state_code(&v, "case:S", "support:shoe"), state_code(&v, "case:O", "support:shoe")), (3.0, 3.0));
        check(&v, ("load_case", "case:S"), "support:shoe", p(-10), 1024.0, -1024.0, 2048.0, "S");
        check(&v, ("load_case", "case:O"), "support:shoe", -p(-10), -1024.0, 1024.0, 2048.0, "O");
        check(&v, ("combination", "comb:o-s"), "support:shoe", -p(-9), -2048.0, 2048.0, 2048.0, "O-S");
        let notices = diagnostics(&v, STATES_DIFFER);
        assert_eq!(notices.len(), 2);
        for (id, notice) in ["comb:o-s", "comb:range"].iter().zip(&notices) {
            assert_eq!(notice["affected_refs"], json!([id, "support:shoe"]));
            assert_eq!(
                notice["message"],
                format!("combination {id}: the reported final state of 1 nonlinear support(s) differs between its operand cases: support:shoe (case:O: sliding (negative slip), case:S: sliding (positive slip)){STATES_TAIL}")
            );
        }
        if mode == PreviewSolverMode::SparseInteractive {
            corpus.push(json!({"name": "slip_reversal", "mode": "sparse_interactive", "model": slip_reversal()["model"], "document": v}));
        }
    }
    write_corpus("slip", corpus);
}

/// V-2 (design note §1.5; stage 2 R-a, W-16 A-1 operands): the A-1 propped
/// cantilever (L = 4 m, P at midspan, one-way prop at the tip on UZ) whose
/// anchor rises by s, with s* = 5PL^3/(48EI) = 2^-10 m by A-1's construction.
/// Sustained: prop active, R = 5P/16, anchor My = RL - PL/2 = -3PL/16. Anchor
/// rise s = 2s*: the prop lifts off, u_tip = s*, My = -PL/2; O−S: ΔR = -5P/16,
/// ΔMy = -5PL/16. s = s*/2: R = 5P/32, My = -11PL/32; O−S: ΔR = -5P/32,
/// ΔMy = -5PL/32. Only the lifted difference changes the prop's state.
fn v2() -> Value {
    let request: Value = serde_json::from_str(include_str!("../../../validation/references/w16_nonlinear_motion/requests/a1.json")).unwrap();
    let mut wire = request.clone();
    let loads = &request["model"]["load_cases"][0]["primitive_loads"];
    let point = loads.as_array().unwrap().iter().find(|l| l["category"] == "concentrated_force").unwrap().clone();
    let rise = |id: &str, s: f64| {
        let mut motion = loads.as_array().unwrap().iter().find(|l| l["category"] == "imposed_displacement").unwrap().clone();
        motion["id"] = json!(format!("{id}-S"));
        motion["magnitude"]["value"] = json!(s);
        motion
    };
    let with = |id: &str, extra: Option<Value>| {
        let mut p = point.clone();
        p["id"] = json!(format!("{id}-P"));
        let mut loads = vec![p];
        loads.extend(extra);
        json!({"id": id, "primitive_loads": loads, "provenance": "op1a"})
    };
    wire["model"]["load_cases"] = json!([with("case:S", None), with("case:O-LIFT", Some(rise("case:O-LIFT", p(-9)))), with("case:O-ACT", Some(rise("case:O-ACT", p(-11))))]);
    wire["model"]["combinations"] = json!([subtraction("comb:lift", "case:O-LIFT", "case:S"), subtraction("comb:act", "case:O-ACT", "case:S"), range("comb:range", &["case:S", "case:O-LIFT"])]);
    wire
}

#[test]
fn v2_anchor_rise_lifts_the_prop_in_one_operand_only() {
    let wire = v2();
    let point = wire["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"].as_f64().unwrap().abs();
    let (pl, s_star) = (point * 4.0, p(-10));
    let mut corpus = Vec::new();
    for mode in MODES {
        let v = solve(&wire, mode);
        let at = |basis: (&str, &str)| {
            (value(&v, "global_nodal_displacement_z", "node:T", basis, None, None),
             value(&v, "support_reaction_component_v2", "support:PROP", basis, Some("Fz"), None),
             value(&v, "support_reaction_component_v2", "support:ANCHOR", basis, Some("My"), None))
        };
        let expect = |basis: (&str, &str), u_m: f64, r: f64, my: f64| {
            let (u, rr, m) = at(basis);
            close(u, u_m * 1000.0, s_star * 1000.0, &format!("{basis:?} u"));
            close(rr, r, point, &format!("{basis:?} R"));
            close(m, my, pl, &format!("{basis:?} My"));
        };
        assert_eq!(state_code(&v, "case:S", "support:PROP"), 1.0);
        assert_eq!(state_code(&v, "case:O-LIFT", "support:PROP"), 0.0);
        assert_eq!(state_code(&v, "case:O-ACT", "support:PROP"), 1.0);
        expect(("load_case", "case:S"), 0.0, 5.0 * point / 16.0, -3.0 * pl / 16.0);
        expect(("load_case", "case:O-LIFT"), s_star, 0.0, -pl / 2.0);
        expect(("load_case", "case:O-ACT"), 0.0, 5.0 * point / 32.0, -11.0 * pl / 32.0);
        expect(("combination", "comb:lift"), s_star, -5.0 * point / 16.0, -5.0 * pl / 16.0);
        expect(("combination", "comb:act"), 0.0, -5.0 * point / 32.0, -5.0 * pl / 32.0);
        let notices = diagnostics(&v, STATES_DIFFER);
        let ids: Vec<&str> = notices.iter().map(|d| d["affected_refs"][0].as_str().unwrap()).collect();
        assert_eq!(ids, ["comb:lift", "comb:range"], "only the lifted operand changes the prop's state");
        assert_eq!(
            notices[0]["message"],
            format!("combination comb:lift: the reported final state of 1 nonlinear support(s) differs between its operand cases: support:PROP (case:O-LIFT: inactive, case:S: active){STATES_TAIL}")
        );
        let records = v["contract_evidence"]["combination_operands"].as_array().unwrap();
        assert_eq!(records.iter().map(|r| r["combination_id"].as_str().unwrap()).collect::<Vec<_>>(), ["comb:lift", "comb:act", "comb:range"]);
        for record in records {
            for operand in record["operands"].as_array().unwrap() {
                assert_eq!(operand["numerical_quality"], quality(&v, operand["load_case_id"].as_str().unwrap()));
            }
        }
        if mode == PreviewSolverMode::SparseInteractive {
            corpus.push(json!({"name": "v2_anchor_rise", "mode": "sparse_interactive", "model": wire["model"], "document": v}));
        }
    }
    write_corpus("v2", corpus);
}

/// V-4: swapping a subtraction's operands negates every finite nonzero signed row;
/// displacement and support-action magnitudes are unchanged, and zero comparisons
/// ignore the sign bit. The case order changes no case row.
#[test]
fn v4_operand_swap_negates_and_case_order_changes_no_case_row() {
    for mode in MODES {
        let mut wire = v1(p(-9), 1536.0);
        wire["model"]["combinations"] = json!([subtraction("comb:o-s", "case:O", "case:S"), subtraction("comb:s-o", "case:S", "case:O")]);
        let v = solve(&wire, mode);
        let combined = |id: &str| -> Vec<(String, f64)> {
            // The combination's modulus-basis record names the combination and is not a value.
            let mut out: Vec<(String, f64)> = rows(&v).iter().filter(|r| r["basis_ref"]["ref_id"] == id && r["kind"] != "combination_modulus_basis_record")
                .map(|r| (format!("{}|{}|{}|{}", r["kind"], r["entity_ref"], r["metadata"]["location"], r["metadata"]["component"]), r["value"].as_f64().unwrap()))
                .collect();
            out.sort_by(|a, b| a.0.cmp(&b.0));
            out
        };
        let (a, b) = (combined("comb:o-s"), combined("comb:s-o"));
        assert_eq!(a.len(), b.len());
        assert!(!a.is_empty());
        for ((ka, va), (kb, vb)) in a.iter().zip(&b) {
            assert_eq!(ka, kb);
            if ka.contains("magnitude") {
                assert_eq!(va.to_bits(), vb.to_bits(), "{ka}");
            } else if *va == 0.0 {
                assert_eq!(*vb, 0.0, "{ka}");
            } else {
                assert_eq!(va.to_bits(), (-vb).to_bits(), "{ka}");
            }
        }
        // The swapped record names the operands in its own order.
        assert_eq!(v["contract_evidence"]["combination_operands"][1]["operands"][0]["load_case_id"], "case:S");
        // Case order: the operating case first publishes the same case rows bit for bit.
        let mut reordered = wire.clone();
        let cases = reordered["model"]["load_cases"].as_array_mut().unwrap();
        cases.reverse();
        let w = solve(&reordered, mode);
        let case_rows = |doc: &Value, case: &str| -> Vec<(String, u64)> {
            let mut out: Vec<(String, u64)> = rows(doc).iter().filter(|r| r["basis_ref"]["ref_type"] == "load_case" && r["basis_ref"]["ref_id"] == case)
                .map(|r| (format!("{}|{}|{}|{}", r["kind"], r["entity_ref"], r["metadata"]["location"], r["metadata"]["component"]), r["value"].as_f64().unwrap().to_bits()))
                .collect();
            out.sort();
            out
        };
        for case in ["case:S", "case:O"] {
            assert_eq!(case_rows(&v, case), case_rows(&w, case), "{case}");
            assert_eq!(quality(&v, case), quality(&w, case), "{case}");
        }
    }
}

/// §2.1: every other document publishes no member and no notice.
#[test]
fn documents_without_the_condition_publish_no_member_and_no_notice() {
    let absent = |v: &Value, what: &str| {
        let keys: Vec<&str> = v["contract_evidence"].as_object().unwrap().keys().map(String::as_str).collect();
        assert_eq!(keys, ["combination_gates", "preview_cases"], "{what}");
        assert!(diagnostics(v, STATES_DIFFER).is_empty() && diagnostics(v, NOT_PASSED).is_empty(), "{what}");
    };
    for mode in MODES {
        // A nonlinear model whose only combination is mechanics (withheld).
        let mut wire = v1(p(-9), 1536.0);
        wire["model"]["combinations"] = json!([mechanics("comb:sum", "case:S", "case:O")]);
        absent(&solve(&wire, mode), "mechanics only");
        // A nonlinear model without combinations.
        wire["model"]["combinations"] = json!([]);
        absent(&solve(&wire, mode), "no combinations");
        // A linear model (a rigid tip stop instead of the gap) with every basis.
        let mut linear = bar(
            vec![json!({"id":"support:stop","node":"node:b","family":"guide","restraints":["UY"],"provenance":"op1a"})],
            json!([case("case:S", json!([force("f-s", 1536.0)])), case("case:O", json!([force("f-o", 1536.0), thermal("t-o", 16.0)]))]),
            json!([subtraction("comb:o-s", "case:O", "case:S"), range("comb:range", &["case:S", "case:O"]), mechanics("comb:sum", "case:S", "case:O")]),
        );
        let v = solve(&linear, mode);
        absent(&v, "linear");
        assert!(rows(&v).iter().any(|r| r["basis_ref"]["ref_id"] == "comb:sum"));
        linear["model"]["combinations"] = json!([]);
        absent(&solve(&linear, mode), "linear without combinations");
    }
}

/// `OP1A_WRITE_CORPUS=<dir>` writes the shared reader corpus parts (the RE and
/// Python readers read `fixtures/results/preview_physics_nonlinear_operands_corpus.json`).
fn write_corpus(part: &str, documents: Vec<Value>) {
    if let Ok(dir) = std::env::var("OP1A_WRITE_CORPUS") {
        let path = std::path::Path::new(&dir).join(format!("op1a_{part}.json"));
        std::fs::write(path, serde_json::to_string_pretty(&json!(documents)).unwrap() + "\n").unwrap();
    }
}

/// The committed corpus is this test's own output (sparse mode).
#[test]
fn committed_reader_corpus_is_the_producer_output() {
    let corpus: Value = serde_json::from_str(include_str!("../../../fixtures/results/preview_physics_nonlinear_operands_corpus.json")).unwrap();
    for entry in corpus["documents"].as_array().unwrap() {
        let wire = json!({"model": entry["model"], "materials": []});
        assert_eq!(solve(&wire, PreviewSolverMode::SparseInteractive), entry["document"], "{}", entry["name"]);
    }
}

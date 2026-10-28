//! Operating-case slice 1a, V-4 (DEL-05-02 Design `pf-nonlinear-operand-standing.md`
//! §4): case independence by seed instrumentation. `append_nonlinear_support_loop_results`
//! reports the initial support states each case's active-set loop starts from; every
//! case, in every case order, must start from the model's authored `initial_state`
//! tokens, read here from the test's own model JSON. A seed carried from one case
//! into the next (each test model ends some case in a state other than its seed)
//! fails these tests, which final values alone would not show when the carried seed
//! converges to the same state.
use super::*;
use std::cell::RefCell;

thread_local! {
    static SEEDS: RefCell<Option<Vec<(String, Vec<(String, &'static str)>)>>> = const { RefCell::new(None) };
}

/// Called by the loop with the seed it was given (tests only).
pub(crate) fn observe_seed(case_id: &str, seed: &[SupportStateRecord]) {
    SEEDS.with(|seeds| {
        if let Some(seeds) = seeds.borrow_mut().as_mut() {
            seeds.push((case_id.to_string(), seed.iter().map(|s| (s.support_id.clone(), s.state.as_str())).collect()));
        }
    });
}

fn with_seeds<T>(run: impl FnOnce() -> T) -> (T, Vec<(String, Vec<(String, &'static str)>)>) {
    SEEDS.with(|seeds| *seeds.borrow_mut() = Some(Vec::new()));
    let out = run();
    (out, SEEDS.with(|seeds| seeds.borrow_mut().take().unwrap()))
}

fn bar(supports: serde_json::Value, cases: serde_json::Value) -> serde_json::Value {
    let inner: f64 = 0.12 - 0.02;
    let area = std::f64::consts::PI / 4.0 * (0.12 * 0.12 - inner * inner);
    let e = 4.0 * 1_048_576.0 / area;
    let mut all = vec![serde_json::json!({"id":"support:a","node":"node:a","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"op1a"})];
    all.extend(supports.as_array().unwrap().iter().cloned());
    serde_json::json!({"model":{"schema_version":"0.8.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:op1a","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[{"id":"node:a","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"op1a"},{"id":"node:b","position":{"x":4.0,"y":0.0,"z":0.0},"provenance":"op1a"}],
        "pipe_segments":[{"id":"pipe:a-b","from":"node:a","to":"node:b","section":{"outside_diameter":{"value":0.12,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},
            "material":"material:m","y_reference":{"x":0.0,"y":1.0,"z":0.0},"provenance":"op1a"}],
        "supports":all,"components":[],
        "materials":[{"id":"material:m","elastic_modulus":{"value":e,"unit":"Pa"},"shear_modulus":{"value":e / 2.6,"unit":"Pa"},
            "thermal_expansion_coefficient":{"value":1.0 / 65_536.0,"unit":"1/degC"},"provenance":"op1a_invented_test_values"}],
        "load_cases":cases,"combinations":[]},"materials":[]})
}

fn case(id: &str, f: f64, dt: f64) -> serde_json::Value {
    let mut loads = vec![serde_json::json!({"id":format!("f-{id}"),"category":"concentrated_force","target":{"type":"node","node":"node:b"},"direction":"global_x","magnitude":{"value":f,"unit":"N"},"dimension":"force","provenance":"op1a"})];
    if dt != 0.0 {
        loads.push(serde_json::json!({"id":format!("t-{id}"),"category":"thermal","target":{"type":"element","pipe":"pipe:a-b"},"direction":"global_x","magnitude":{"value":dt,"unit":"degC"},"dimension":"temperature_interval","provenance":"op1a"}));
    }
    serde_json::json!({"id":id,"primitive_loads":loads,"provenance":"op1a"})
}

/// The authored seed, from the model JSON: (support id, initial_state) in support order.
fn authored(wire: &serde_json::Value) -> Vec<(String, String)> {
    wire["model"]["supports"].as_array().unwrap().iter()
        .filter(|s| s.get("nonlinear").is_some())
        .map(|s| (s["id"].as_str().unwrap().to_string(), s["nonlinear"]["initial_state"].as_str().unwrap().to_string()))
        .collect()
}

/// Every case of every order starts from the authored seed, and each case's
/// published state codes and quality do not depend on the order.
fn check(supports: serde_json::Value, cases: Vec<serde_json::Value>, orders: &[&[usize]], changed: &str) {
    let mut by_order: Vec<Vec<(String, Vec<(String, u64)>)>> = Vec::new();
    for order in orders {
        let wire = bar(supports.clone(), serde_json::json!(order.iter().map(|i| cases[*i].clone()).collect::<Vec<_>>()));
        let expected = authored(&wire);
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let (envelope, seeds) = with_seeds(|| run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap());
            assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED");
            let ids: Vec<&str> = order.iter().map(|i| cases[*i]["id"].as_str().unwrap()).collect();
            assert_eq!(seeds.iter().map(|(c, _)| c.as_str()).collect::<Vec<_>>(), ids, "one loop per case, in case order");
            for (case, seed) in &seeds {
                let seed: Vec<(String, String)> = seed.iter().map(|(s, t)| (s.clone(), t.to_string())).collect();
                assert_eq!(seed, expected, "{case} in order {order:?} must start from the authored seed");
            }
            let mut states: Vec<(String, Vec<(String, u64)>)> = ids.iter().map(|case| {
                let codes = envelope.results.iter()
                    .filter(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_id == *case) && r.kind.starts_with("nonlinear_support_"))
                    .map(|r| (format!("{}|{}", r.kind, r.entity_ref), r.value.to_bits()))
                    .collect::<Vec<_>>();
                (case.to_string(), codes)
            }).collect();
            states.sort();
            if mode == PreviewSolverMode::SparseInteractive {
                by_order.push(states);
            }
        }
    }
    for other in &by_order[1..] {
        assert_eq!(other, &by_order[0], "case order must not change any case's nonlinear rows");
    }
    // The models are chosen so that some case ends away from its seed.
    let ended = &by_order[0];
    assert!(ended.iter().any(|(_, rows)| rows.iter().any(|(k, bits)| k.starts_with("nonlinear_support_active_set_state_code") && f64::from_bits(*bits) != state_code(changed))), "no case leaves its seed");
}

fn state_code(token: &str) -> f64 {
    match token {
        "inactive" => 0.0,
        "active" => 1.0,
        "sticking" => 2.0,
        _ => 3.0,
    }
}

#[test]
fn every_gap_case_starts_from_the_authored_seed_in_every_order() {
    let gap = serde_json::json!([{"id":"support:gap","node":"node:b","family":"nonlinear","restraints":[],"provenance":"op1a",
        "nonlinear":{"behavior":"gap","closes_when":"positive_displacement","dof":"UX","gap":{"value":2f64.powi(-9),"unit":"m"},"initial_state":"inactive"}}]);
    // Sustained (open), operating (closes) and a heavier operating case (closes).
    let cases = vec![case("case:S", 1536.0, 0.0), case("case:O", 1536.0, 16.0), case("case:O2", 2560.0, 16.0)];
    check(gap, cases, &[&[0, 1, 2], &[1, 0, 2], &[2, 1, 0]], "inactive");
}

#[test]
fn every_friction_case_starts_from_the_authored_seed_in_every_order() {
    let shoe = serde_json::json!([{"id":"support:shoe","node":"node:b","family":"nonlinear","restraints":[],"provenance":"op1a",
        "nonlinear":{"behavior":"friction","dof":"UX","initial_state":"sticking","friction_coefficient":{"value":0.25,"unit":"none"},"normal_reaction":{"value":4096.0,"unit":"N"}}}]);
    // Sustained and operating slide; a light case sticks.
    let cases = vec![case("case:S", 2048.0, 0.0), case("case:O", 2048.0, -8.0), case("case:L", 512.0, 0.0)];
    check(shoe, cases, &[&[0, 1, 2], &[1, 0, 2], &[2, 0, 1]], "sticking");
}

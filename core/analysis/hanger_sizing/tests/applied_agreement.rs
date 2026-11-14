//! The applied model's solve against the run's verification pass (DEL-04-11
//! REQ-005, REQ-008). The applied model solved here is the very bytes the desktop
//! writes when it applies a run's selections: `batch_after_model` of a committed
//! fixture that `apps/desktop/src/features/hanger-sizing/runApplication.fixture.test.ts`
//! writes and pins. Every expected value is a number the run already published
//! (each candidate's `verification.u_cold_m` and `verification.operating[].u_m`);
//! this file sets none.
//!
//! Two fixtures, two claims:
//! - `hanger_sizing_applied_four_span.json` (CONDITIONAL). The four-span run
//!   selected constant records at B and C and found no record for D, which the
//!   application leaves a rigid support. The run's verification solved D as a force
//!   equal to its hot-load basis (its stated `verification_assumption`), so it is
//!   conditional. (i) With D put in that assumption by a test-only change, the
//!   applied model reproduces the verified positions at B and C. (ii) As applied,
//!   with D rigid, it does not: that is what "conditional" means, and what REQ-005
//!   states ("which applying the selected records alone does not reproduce").
//! - `hanger_sizing_applied_four_span_constant.json` (UNCONDITIONAL). The run with B
//!   and C under the constant policy selected a constant record at each, and the
//!   verification is unconditional. The applied model, solved as it is with no
//!   test-side change, reproduces the verified positions at both.
//!
//! A constant-effort support's applied force is its `constant_load`, consumed by the
//! solve as a constant force along its DOF; a solve that warns
//! `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` fails this test.
//!
//! Neither fixture is evidence for a variable spring. Until the solve applies a
//! spring's cold load (REQ-014), an applied variable selection does not reproduce its
//! verification; that evidence is owed by the preload unit.
//!
//! Tolerance, as `derived_solves.rs` and `ex_references.rs`: 1e-9 relative, with a
//! floor of 1e-9 of the case's largest verified position, never below 1e-12 m. Both
//! solver modes.

use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode};
use serde_json::{json, Value};
use std::path::PathBuf;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive];

fn read_json(name: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/results").join(name);
    serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{name}: {e}"))).unwrap()
}

/// The applied model's solve request for one case: the model the desktop wrote,
/// with the committed input's request materials, and only that case.
fn applied_request(applied: &Value, input: &Value, case: &str) -> Value {
    let mut model = applied["batch_after_model"].clone();
    model["load_cases"].as_array_mut().unwrap().retain(|c| c["id"] == case);
    json!({"model": model, "materials": input["request"]["materials"]})
}

fn solve(request: Value, mode: PreviewSolverMode, what: &str) -> MechanicsEnvelope {
    let envelope = run_linear_static_preview_value_with_mode(request, mode).unwrap_or_else(|e| panic!("{what}: refused: {e}"));
    let codes: Vec<&str> = envelope.diagnostics.iter().map(|d| d.code.as_str()).collect();
    assert!(!codes.contains(&"SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED"), "{what}: a constant-effort support was not consumed: {codes:?}");
    let blocking: Vec<&str> = envelope.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect();
    assert!(blocking.is_empty(), "{what}: {blocking:?}");
    envelope
}

fn displacement(envelope: &MechanicsEnvelope, node: &str) -> f64 {
    let row = envelope.results.iter().find(|r| r.kind == "global_nodal_displacement_z" && r.entity_ref == node).unwrap();
    match row.unit.as_str() {
        "mm" => row.value / 1000.0,
        "m" => row.value,
        other => panic!("unit {other}"),
    }
}

/// Each named case with, for each selected candidate, its node and the verified
/// position the result published in that case.
fn verified(result: &Value) -> Vec<(String, Vec<(String, f64)>)> {
    let request = &result["inputs"]["sizing_request"];
    let mut cases = vec![request["weight_case"].as_str().unwrap().to_string(), request["design_operating_case"].as_str().unwrap().to_string()];
    cases.extend(request["check_operating_cases"].as_array().unwrap().iter().map(|c| c.as_str().unwrap().to_string()));
    let selected: Vec<&Value> = result["result"]["candidates"].as_array().unwrap().iter().filter(|c| !c["verification"].is_null()).collect();
    cases
        .iter()
        .enumerate()
        .map(|(i, case)| {
            let positions = selected
                .iter()
                .map(|c| {
                    let v = &c["verification"];
                    let u = if i == 0 { v["u_cold_m"].as_f64().unwrap() } else { v["operating"][i - 1]["u_m"].as_f64().unwrap() };
                    (c["node"].as_str().unwrap().to_string(), u)
                })
                .collect();
            (case.clone(), positions)
        })
        .collect()
}

fn floor(positions: &[(String, f64)]) -> f64 {
    1e-9 * positions.iter().map(|(_, u)| u.abs()).fold(1e-3, f64::max)
}

/// Asserts agreement and returns the largest relative difference seen
/// (|a − e| / max(|a|, |e|), over values larger than their floor).
fn agree(envelope: &MechanicsEnvelope, positions: &[(String, f64)], what: &str) -> f64 {
    let floor = floor(positions);
    let mut largest: f64 = 0.0;
    for (node, expected) in positions {
        let actual = displacement(envelope, node);
        let difference = (actual - expected).abs();
        assert!(difference <= 1e-9 * actual.abs().max(expected.abs()) + floor, "{what} {node}: {actual} against the verified {expected}");
        if expected.abs() > floor {
            largest = largest.max(difference / actual.abs().max(expected.abs()));
        }
    }
    largest
}

#[test]
fn the_conditional_application_reproduces_its_verification_only_under_the_stated_assumption() {
    let input = read_json("hanger_sizing_input_four_span.json");
    let result = read_json("hanger_sizing_result_four_span.json");
    let applied = read_json("hanger_sizing_applied_four_span.json");
    assert_eq!(applied["source"]["result"], "hanger_sizing_result_four_span.json");
    assert_eq!(result["result"]["verification_standing"], "conditional");
    let unfitted: Vec<&Value> = result["result"]["candidates"].as_array().unwrap().iter().filter(|c| c["outcome"] == "no_record_fits").collect();
    assert_eq!(unfitted.len(), 1);
    let (d_support, d_node) = (unfitted[0]["support"].as_str().unwrap(), unfitted[0]["node"].as_str().unwrap());
    let assumption = &unfitted[0]["verification_assumption"];
    assert_eq!(assumption["role"], "force");
    let force_n = assumption["force_n"].as_f64().unwrap();
    assert_eq!(force_n, unfitted[0]["w_n"].as_f64().unwrap(), "the stated force is D's hot-load basis");

    let mut largest_i: f64 = 0.0;
    let mut smallest_ii = f64::INFINITY;
    for mode in MODES {
        for (case, positions) in verified(&result) {
            let what = format!("{case} {mode:?}");
            // (i) The applied model with D in the run's stated assumption: D removed
            // and replaced by a force equal to its published hot-load basis, along +Z.
            let mut assumed = applied_request(&applied, &input, &case);
            let model = &mut assumed["model"];
            model["supports"].as_array_mut().unwrap().retain(|s| s["id"] != d_support);
            model["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(json!({
                "id": "load:test-assumption:support:D", "category": "concentrated_force", "target": {"type": "node", "node": d_node},
                "direction": "global_z", "magnitude": {"value": force_n, "unit": "N"}, "dimension": "force",
                "provenance": "test-only: the run's stated verification assumption for the unfitted candidate"}));
            let envelope = solve(assumed, mode, &format!("{what} under the assumption"));
            largest_i = largest_i.max(agree(&envelope, &positions, &format!("{what} under the assumption")));

            // (ii) As applied, with D rigid: in each operating case the verified
            // positions are not reproduced. This is what a conditional verification is:
            // REQ-005's verification of a candidate with no fit assumes it carries its
            // hot-load basis, "which applying the selected records alone does not
            // reproduce". In the weight case the two agree by construction, since D's
            // hot-load basis is its rigid reaction in that case, with B and C carrying
            // theirs; so the weight case is asserted to agree.
            let envelope = solve(applied_request(&applied, &input, &case), mode, &format!("{what} as applied"));
            if case == result["inputs"]["sizing_request"]["weight_case"] {
                agree(&envelope, &positions, &format!("{what} as applied, the weight case"));
            } else {
                let floor = floor(&positions);
                let apart = positions
                    .iter()
                    .map(|(node, expected)| {
                        let actual = displacement(&envelope, node);
                        (actual - expected).abs() - (1e-9 * actual.abs().max(expected.abs()) + floor)
                    })
                    .fold(f64::NEG_INFINITY, f64::max);
                assert!(apart > 0.0, "{what}: as applied (D rigid) the positions equal the conditional verification's");
                smallest_ii = smallest_ii.min(apart);
            }
            for (node, expected) in &positions {
                eprintln!("conditional {what} {node}: verified {expected}, as applied {}", displacement(&envelope, node));
            }
        }
    }
    eprintln!("conditional (i): largest relative difference {largest_i:e}; (ii): operating cases apart by at least {smallest_ii:e} m beyond the tolerance");
}

#[test]
fn the_unconditional_constant_application_reproduces_its_verification_as_applied() {
    let input = read_json("hanger_sizing_input_four_span_constant.json");
    let result = read_json("hanger_sizing_result_four_span_constant.json");
    let applied = read_json("hanger_sizing_applied_four_span_constant.json");
    assert_eq!(applied["source"]["result"], "hanger_sizing_result_four_span_constant.json");
    assert_eq!(result["result"]["verification_standing"], "unconditional");
    for candidate in result["result"]["candidates"].as_array().unwrap() {
        assert_eq!(candidate["selected"]["family"], "constant_effort", "{}", candidate["support"]);
    }
    let mut largest: f64 = 0.0;
    for mode in MODES {
        for (case, positions) in verified(&result) {
            assert_eq!(positions.len(), 2);
            let envelope = solve(applied_request(&applied, &input, &case), mode, &format!("{case} {mode:?}"));
            largest = largest.max(agree(&envelope, &positions, &format!("{case} {mode:?} as applied")));
            for (node, expected) in &positions {
                eprintln!("unconditional {case} {mode:?} {node}: verified {expected}, as applied {}", displacement(&envelope, node));
            }
        }
    }
    eprintln!("unconditional: largest relative difference {largest:e}");
}

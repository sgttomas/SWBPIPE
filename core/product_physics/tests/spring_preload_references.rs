//! The variable spring preload (DEL-04-03 REQ-014 and AC-009; DEL-04-11 REQ-014
//! and AC-014) against the frozen independent reference
//! `validation/references/hanger_sizing/preload.json` (P-A, P-B), whose
//! generator imports no product code.
//!
//! Each unheld case (PA-WT, PA-EMPTY, PA-BASE, PA-B, PA-AB, PB-WT, PB-EMPTY,
//! PB-BASE) is authored as a one-case document from the reference's own inputs
//! and solved on the pressure-free route (PF) and the exact pressure route
//! (EX), in both solver modes. Compared, at 1e-9 relative with a 1e-12 absolute
//! floor (the hanger-sizing reference tests' tolerance): every node's UZ
//! displacement, every published support force along +Z (the force on the
//! pipe), and each hanger's published applied force. A moved base is a
//! support-targeted imposed displacement on PF and the support's
//! `base_motion` on its stiffness axis on EX (a load/reference-state document).
//!
//! The force-plus-spring twin (each hanger a plain spring of its rate plus a
//! separate nodal force F_cold) reproduces the same displacements and the
//! twin's plain-spring force; the superposition block's naive sum minus the
//! direct solve equals PA-EMPTY. The held cases PA-HT and PA-HT-NONE are out of
//! this test's scope: a test-condition case is refused before any solve.
//!
//! A mismatch is the product's to fix; the reference is never altered and the
//! tolerance never loosened.

use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};

#[path = "common/spring_preload_model.rs"]
mod model;
use model::*;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

pub fn solve(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).expect("the entry accepts the document")).unwrap()
}

pub fn solved(document: &Value, mode: PreviewSolverMode, what: &str) -> Value {
    let envelope = solve(document, mode);
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:#?}", envelope["diagnostics"]);
    assert!(!envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["severity"] == "blocking"), "{what}");
    envelope
}

#[test]
fn the_preload_reference_agrees_on_both_routes_and_both_modes() {
    let reference = reference();
    for case_id in UNHELD {
        let (_, case) = model_of(&reference, case_id);
        for route in ROUTES {
            for mode in MODES {
                let what = format!("{case_id} {route:?} {mode:?}");
                let envelope = solved(&document(case_id, route, MODEL_SCHEMA_VERSION), mode, &what);
                compare_block(&envelope, &case["expected"], &what);
                // The retired warning is produced by no solve.
                assert!(!envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD"), "{what}");
            }
        }
    }
}

#[test]
fn the_force_plus_spring_twin_has_the_same_displacements_and_its_parts_add_to_the_published_force() {
    let reference = reference();
    for case_id in UNHELD {
        let (_, case) = model_of(&reference, case_id);
        let expected = &case["expected"];
        for route in ROUTES {
            for mode in MODES {
                let what = format!("{case_id} twin {route:?} {mode:?}");
                let twin = solved(&authored(case_id, route, Hangers::Twin, MODEL_SCHEMA_VERSION), mode, &what);
                for (node, u) in expected["u_nodes_m"].as_object().unwrap() {
                    assert_close(uz(&twin, node), u.as_f64().unwrap(), &format!("{what} u {node}"));
                }
                for (support, parts) in expected["force_plus_spring_twin"].as_object().unwrap() {
                    let plain = parts["plain_spring_force_n"].as_f64().unwrap();
                    let cold = parts["applied_force_n"].as_f64().unwrap();
                    assert_close(fz(&twin, support), plain, &format!("{what} plain spring {support}"));
                    assert_close(plain + cold, expected["published_support_force_n"][support].as_f64().unwrap(), &format!("{what} parts {support}"));
                    // The twin's plain spring is no variable spring hanger: no applied row.
                    assert!(rows(&twin, APPLIED, support, None).is_empty(), "{what}");
                }
            }
        }
    }
}

#[test]
fn the_naive_sum_minus_the_direct_solve_is_the_unloaded_case() {
    let reference = reference();
    let block = &reference["superposition"];
    let (model, empty) = model_of(&reference, "PA-EMPTY");
    let nodes: Vec<String> = model["nodes"].as_object().unwrap().keys().cloned().collect();
    let supports = ["SH-3", "S-1", "S-5"];
    for route in ROUTES {
        for mode in MODES {
            let what = format!("superposition {route:?} {mode:?}");
            let a = solved(&document("PA-WT", route, MODEL_SCHEMA_VERSION), mode, &what);
            let b = solved(&document("PA-B", route, MODEL_SCHEMA_VERSION), mode, &what);
            let direct = solved(&document("PA-AB", route, MODEL_SCHEMA_VERSION), mode, &what);
            for node in &nodes {
                let naive = uz(&a, node) + uz(&b, node);
                assert_close(uz(&direct, node), block["direct_u_nodes_m"][node].as_f64().unwrap(), &format!("{what} direct u {node}"));
                assert_close(naive, block["naive_sum_u_nodes_m"][node].as_f64().unwrap(), &format!("{what} naive u {node}"));
                assert_close(naive - uz(&direct, node), empty["expected"]["u_nodes_m"][node].as_f64().unwrap(), &format!("{what} difference u {node}"));
            }
            for support in supports {
                let naive = fz(&a, support) + fz(&b, support);
                assert_close(fz(&direct, support), block["direct_published_support_force_n"][support].as_f64().unwrap(), &format!("{what} direct {support}"));
                assert_close(naive, block["naive_sum_published_support_force_n"][support].as_f64().unwrap(), &format!("{what} naive {support}"));
                assert_close(naive - fz(&direct, support), empty["expected"]["published_support_force_n"][support].as_f64().unwrap(), &format!("{what} difference {support}"));
            }
        }
    }
    assert_eq!(block["equals_case"], "PA-EMPTY");
}

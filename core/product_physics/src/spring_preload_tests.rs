//! The preload on the binary64 support-action path (DEL-04-11 REQ-014).
//!
//! Wherever the formation check gives a member bound, the published support
//! action is the certified row (`certified_publication::support`), which
//! `tests/spring_preload_references.rs` examines. Without the bound the case
//! loop's binary64 action is published ([`super::with_cold_load`]); the
//! test-only `U4A_FORCE_BINARY64` hook withdraws the bound so that this path
//! is compared with the same frozen reference, on both routes and modes.
use crate::PreviewSolverMode;
use serde_json::Value;

#[path = "../tests/common/spring_preload_model.rs"]
mod model;
use model::{compare_block, document, model_of, reference, Route, ROUTES, UNHELD};
#[allow(unused_imports)]
use model::Hangers;

fn binary64_solve(document: &Value, mode: PreviewSolverMode) -> Value {
    crate::U4A_FORCE_BINARY64.with(|flag| flag.set(true));
    let envelope = crate::run_linear_static_preview_value_with_mode(document.clone(), mode);
    crate::U4A_FORCE_BINARY64.with(|flag| flag.set(false));
    serde_json::to_value(envelope.expect("the entry accepts the document")).unwrap()
}

#[test]
fn the_binary64_support_action_carries_the_cold_load() {
    let reference = reference();
    for case_id in UNHELD {
        let (_, case) = model_of(&reference, case_id);
        for route in ROUTES {
            for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
                let what = format!("{case_id} {route:?} {mode:?} binary64");
                let envelope = binary64_solve(&document(case_id, route, crate::MODEL_SCHEMA_VERSION), mode);
                assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:#?}", envelope["diagnostics"]);
                compare_block(&envelope, &case["expected"], &what);
            }
        }
    }
}

/// The source replay (`source_recovery`) with the preload: its case force is
/// the captured replay's `replay_case_force`, which must equal the live
/// ledger's fold, and its support row is F_cold - k·u. The replay is entered
/// directly, as its own fixture does (the product reaches it on the exact
/// route for a sensitive case only), on that fixture's two-node cantilever
/// (`fixtures/product_preview/numerical_sensitive_torsion_model.json`, a size
/// the replay's exact kernel admits) with a variable spring hanger added at the
/// tip on UZ. Oracle, closed form and independent of the product: the tip's
/// vertical stiffness is k_b = 3EI/L³ (Euler-Bernoulli cantilever), so
/// u = (P + F_cold)/(k_b + k) and the spring publishes F_cold - k·u.
#[test]
fn the_source_replay_carries_the_cold_load_in_its_force_and_support_row() {
    let (k, cold, load) = (2.0e5, 500.0, -300.0);
    let mut document: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/numerical_sensitive_torsion_model.json")).unwrap();
    let q = |value: f64, unit: &str| serde_json::json!({"value": value, "unit": unit});
    document["supports"].as_array_mut().unwrap().push(serde_json::json!({"id": "hanger", "node": "tip", "family": "variable_spring_hanger", "restraints": ["UZ"],
        "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ", "value": q(k, "N/m")},
            "installed_load": q(cold, "N"), "cold_load": q(cold, "N"), "hot_load": q(cold, "N"), "travel_range": q(0.05, "m"), "source_reference": "invented"},
        "provenance": "invented"}));
    document["load_cases"][0]["primitive_loads"] = serde_json::json!([{"id": "down", "category": "concentrated_force", "dimension": "force", "direction": "global_z",
        "magnitude": q(load, "N"), "target": {"type": "node", "node": "tip"}, "provenance": "invented"}]);
    let model: crate::PreviewModel = serde_json::from_value(document).unwrap();
    let mut diagnostics = Vec::new();
    let built = crate::build_model(&model, &model.materials, &mut diagnostics).unwrap();
    let boundary = crate::prepare_boundary(built.nodes.len(), &built.supports);
    let mut stiffness = crate::assemble_global_stiffness_with_connectors(built.nodes.len(), &built.frame_elements, &built.connectors).unwrap();
    for spring in &boundary.springs {
        let dof = spring.node_dof.global_index();
        stiffness[dof][dof] += spring.stiffness.value;
    }
    let load_case = &model.load_cases[0];
    let primitives = crate::build_load_case_primitive_loads(&model, load_case, &mut diagnostics);
    let loads = crate::prepare_loads(built.nodes.len(), built.pipes.len(), &primitives);
    let force = super::replay_case_force(&loads, &[], &built, &model, load_case).unwrap();
    let prescribed: Vec<_> = boundary.restrained_dofs.iter().map(|&d| (d, 0.0)).collect();
    let free: Vec<usize> = (0..force.len()).filter(|d| !boundary.restrained_dofs.contains(d)).collect();
    let input = crate::source_recovery::Input {
        model: &model, built: &built, stiffness: &stiffness, force: &force, free: &free, prescribed: &prescribed,
        spring_entries: &boundary.springs, load_case, load_application: &loads, thermal_loads: &[], load_state: None,
    };
    let recovery = crate::source_recovery::solve_ordinary(input, Default::default(), open_pipe_stress_frame_kernel::ForceScale::UNSCALED)
        .unwrap_or_else(|failure| panic!("{failure:?}"));
    let (d, t, e, l) = (0.2_f64, 0.01_f64, 2.0e11_f64, 2.0_f64);
    let i = std::f64::consts::PI * (d.powi(4) - (d - 2.0 * t).powi(4)) / 64.0;
    let k_b = 3.0 * e * i / l.powi(3);
    let u = (load + cold) / (k_b + k);
    let action = recovery.support_actions().iter().find(|a| a.support_id == "hanger").unwrap();
    model::assert_close(action.values[2], cold - k * u, "replay hanger force");
    // Published nodal translations are in millimetres; the tip is node 1.
    model::assert_close(recovery.published_nodal_components()[6 + 2] / 1000.0, u, "replay tip UZ");
}

#[test]
fn each_not_applied_reason_has_its_fixed_token() {
    use super::NotApplied as N;
    use crate::FrameDof;
    let pinned = [
        (N::NonlinearRecord, "nonlinear_record"),
        (N::SkewedRecord, "skewed_record"),
        (N::ColdLoadMissing, "cold_load_missing"),
        (N::ColdLoadNotPositive, "cold_load_not_positive"),
        (N::StiffnessDofUnreadable, "stiffness_dof_unreadable"),
        (N::RotationalDof(FrameDof::Ry), "rotational_dof"),
        (N::UnknownNode, "unknown_node"),
    ];
    for (reason, token) in &pinned {
        assert_eq!(reason.token(), *token);
    }
    // No payload enters the token.
    assert_eq!(N::RotationalDof(FrameDof::Rx).token(), N::RotationalDof(FrameDof::Rz).token());
}

/// The captured replay's ledger (`replay_case_force`) agrees bit for bit with
/// the live case ledger (`case_force_ledger` then `add_loads`, as
/// `solve_prepared_load_case` forms it), on P-A's pressure-free cases.
#[test]
fn the_replay_ledger_equals_the_live_ledger_bit_for_bit() {
    for case_id in ["PA-WT", "PA-B", "PB-WT"] {
        let document = document(case_id, Route::PressureFree, crate::MODEL_SCHEMA_VERSION);
        let model: crate::PreviewModel = serde_json::from_value(document["model"].clone()).unwrap();
        let mut diagnostics = Vec::new();
        let built = crate::build_model(&model, &model.materials, &mut diagnostics).unwrap();
        let load_case = &model.load_cases[0];
        let primitives = crate::build_load_case_primitive_loads(&model, load_case, &mut diagnostics);
        let loads = crate::prepare_loads(built.nodes.len(), built.pipes.len(), &primitives);
        let mut live = crate::case_force_ledger(&model, &built, &loads, &std::collections::HashMap::new(), &[], None, &load_case.id, &mut diagnostics);
        super::add_loads(&mut live, &model, load_case);
        let live = live.finish(built.nodes.len() * crate::DOF_PER_NODE).unwrap();
        let replay = super::replay_case_force(&loads, &[], &built, &model, load_case).unwrap();
        let bits = |f: &open_pipe_stress_frame_kernel::load_ledger::AssembledForce| f.values().iter().map(|v| v.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(&live), bits(&replay), "{case_id}");
        // And the preload is in it: the hanger DOF carries the cold load.
        let applying = super::dispositions(&model, load_case).into_iter().filter(|(_, d)| d.is_ok()).count();
        assert!(applying > 0, "{case_id}");
        assert_eq!(live.terms().iter().filter(|t| t.source.starts_with("SH-")).count(), applying, "{case_id}: one term per applying spring");
    }
}

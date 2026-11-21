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
        // The replay's own terms: exactly one preload term per applying spring, under the
        // support's id, at the global DOF its node and stiffness DOF name, carrying the
        // entered cold load. The expectation is read from the document, not from the law.
        let mut checked = 0;
        for support in document["model"]["supports"].as_array().unwrap().iter().filter(|s| s["hanger"]["hanger_type"] == "variable_spring_hanger") {
            let id = support["id"].as_str().unwrap();
            let node = document["model"]["nodes"].as_array().unwrap().iter().position(|n| n["id"] == support["node"]).unwrap();
            let axis = ["UX", "UY", "UZ"].iter().position(|a| *a == support["hanger"]["stiffness"]["dof"].as_str().unwrap()).unwrap();
            let cold = support["hanger"]["cold_load"]["value"].as_f64().unwrap();
            assert_eq!(support["hanger"]["cold_load"]["unit"], "N", "{case_id} {id}");
            let terms: Vec<_> = replay.terms().iter().filter(|t| t.source == id).collect();
            assert_eq!(terms.len(), 1, "{case_id} {id}: {terms:?}");
            assert_eq!(terms[0].dof, node * crate::DOF_PER_NODE + axis, "{case_id} {id}");
            assert_eq!(terms[0].kind, open_pipe_stress_frame_kernel::load_ledger::ForceTermKind::Term(cold), "{case_id} {id}");
            checked += 1;
        }
        assert!(checked > 0, "{case_id}: no spring");
        assert_eq!(super::dispositions(&model, load_case).into_iter().filter(|(_, d)| d.is_ok()).count(), checked, "{case_id}");
    }
}

/// The retained sub-route declines a model with a variable spring hanger by
/// the supports rule's hanger reason (D1.6, `AdmissionLaw::domain`, which the
/// law records for every caller and build), forms no successor, and publishes
/// the ordinary result byte for byte, which carries the preload (P-A's SH-3
/// applies 6000 N). Where the build's own refusal is a domain clause, it is
/// that one: a build that declines earlier for another reason (an unregistered
/// profile) still shows the request outside the retained domain for its hanger.
#[test]
fn the_retained_sub_route_declines_a_hanger_by_its_hanger_reason() {
    use crate::retained_memory::{AdmissionRefusal, D1Clause, FamilyFact};
    let hanger = Some(AdmissionRefusal::Family(D1Clause::Supports, FamilyFact::Hanger));
    let document = document("PA-WT", Route::PressureFree, crate::MODEL_SCHEMA_VERSION);
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let ordinary = crate::run_linear_static_preview_value_with_mode(document.clone(), mode).unwrap();
        let retained = crate::run_linear_static_preview_value_with_retained_direct(document.clone(), mode, crate::RetainedByteLimit::new(12_884_901_888)).unwrap();
        let law = retained.admission().expect("the census ran").law();
        assert_eq!(law.domain, hanger, "{mode:?}: the request is outside D1 for its hanger");
        if matches!(law.refusal, Some(AdmissionRefusal::Family(..))) {
            assert_eq!(law.refusal, hanger, "{mode:?}");
        }
        assert!(law.refusal.is_some(), "{mode:?}: admitted");
        assert!(retained.successor().is_none(), "{mode:?}");
        assert_eq!(serde_json::to_vec(retained.envelope()).unwrap(), serde_json::to_vec(&ordinary).unwrap(), "{mode:?}");
        let ordinary = serde_json::to_value(&ordinary).unwrap();
        model::assert_close(model::applied(&ordinary, "SH-3"), 6000.0, &format!("{mode:?}"));
    }
}

/// The demo's one combination, `combination:C-OPER-ALT` (mechanics, `load:L-100`
/// factor 1.0 and `load:L-200` factor 0.5), on the demo without its nonlinear
/// supports, for three tests in `lib.rs`. Its only fixed-load source is
/// `support:SH-140`, a variable spring hanger whose 430 N cold load is applied in
/// every case; `support:CE-120` declares no restraint DOF and is not consumed. By
/// the count rule of DEL-05-02 (Scope "Fixed-load count", REQ-011): a load case has
/// κ = 1, so κ = 1.0·1 + 0.5·1 = 1.5, which differs from 1 by more than 2⁻⁴⁶; its
/// operands are load cases, none withheld (tested first); no nonlinear support
/// remains; it is not a subtraction or envelope. It is withheld whole, by name,
/// under the unit factor-sum code with the message naming the spring, and publishes
/// no row, so no note about its stress maxima either.
///
/// Its twin, the same document with the first factor 0.5, has κ = 0.5·1 + 0.5·1 = 1
/// exactly and is admitted (the cold load counted once); it keeps the combination
/// algebra tested. Returns (the demo's result, the twin's result).
pub(crate) fn c_oper_alt_withheld_and_its_unit_twin(
    input: crate::LinearStaticPreviewRequest,
) -> (crate::MechanicsEnvelope, crate::MechanicsEnvelope) {
    const CODE: &str = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE";
    const TEXT: &str = "Combination withheld: a variable spring hanger's cold load would be counted by the sum of factors instead of once. The difference of two solved cases is available as a result state subtraction.";
    let mut twin = input.clone();
    assert_eq!(twin.model.combinations.len(), 1);
    let combination = &mut twin.model.combinations[0];
    assert_eq!(combination.id, "combination:C-OPER-ALT");
    let factors: Vec<(&str, f64)> = combination.terms.iter().map(|t| (t.load_case.as_str(), t.factor)).collect();
    assert_eq!(factors, vec![("load:L-100", 1.0), ("load:L-200", 0.5)]);
    combination.terms[0].factor = 0.5;

    let withheld = crate::run_linear_static_preview(input);
    assert_eq!(withheld.status.mechanics, "MECHANICS_SOLVED", "{:?}", withheld.diagnostics);
    let gate: Vec<_> = withheld.diagnostics.iter().filter(|item| item.code == CODE).collect();
    assert_eq!(gate.len(), 1, "factor sum 1.5 withholds C-OPER-ALT once: {:?}", withheld.diagnostics);
    assert_eq!(gate[0].affected_refs, vec!["combination:C-OPER-ALT".to_string()]);
    assert_eq!(gate[0].message, TEXT);
    assert!(!withheld.results.iter().any(|item| item.basis_ref.as_ref().is_some_and(|b| b.ref_type == "combination")));
    assert!(!withheld.diagnostics.iter().any(|item| item.code == "COMBINATION_STRESS_MAXIMUM_UNAVAILABLE"));

    let unit = crate::run_linear_static_preview(twin);
    assert_eq!(unit.status.mechanics, "MECHANICS_SOLVED", "{:?}", unit.diagnostics);
    assert!(!unit.diagnostics.iter().any(|item| item.code == CODE), "factor sum 1 is admitted: {:?}", unit.diagnostics);
    (withheld, unit)
}

/// `s(PreviewSupport)` is a priced size atom of the retained memory law
/// (`retained_memory.rs`: `size_of::<crate::PreviewSupport>()`), and a
/// `PreviewSupport` holds its `SpringHangerInput` inline. The preload adds no
/// byte to either; a change of these sizes moves a priced size and is for the
/// pricing audit.
#[test]
fn the_support_and_hanger_records_keep_their_priced_sizes() {
    use std::mem::{align_of, size_of};
    assert_eq!((size_of::<crate::SpringHangerInput>(), align_of::<crate::SpringHangerInput>()), (368, 8));
    assert_eq!((size_of::<crate::PreviewSupport>(), align_of::<crate::PreviewSupport>()), (864, 8));
}

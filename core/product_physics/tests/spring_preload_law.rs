//! Targeted regressions for the variable spring preload (DEL-04-11 REQ-014,
//! AC-014): the combination gate, a spring beside a nonlinear support, the
//! not-applied diagnostic, the installed-load warning, the retired warning's
//! absence and the test-condition refusal with a spring in the model (the
//! retained sub-route's decline, with its reason, is in
//! `src/spring_preload_tests.rs`). Models are the preload reference's P-A
//! (`validation/references/hanger_sizing/preload.json`); where no frozen value
//! exists the oracle is the force-plus-spring twin, the model with each hanger
//! a plain spring of its rate and a separate nodal force F_cold. Invented data.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION,
};
use serde_json::{json, Value};

#[path = "common/spring_preload_model.rs"]
mod model;
use model::*;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RETIRED: &str = "SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD";
const NOT_APPLIED: &str = "SPRING_HANGER_COLD_LOAD_NOT_APPLIED";
const INSTALLED_DIFFERS: &str = "SPRING_HANGER_INSTALLED_LOAD_DIFFERS_FROM_COLD_LOAD";
const GATE: &str = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE";
const GATE_TEXT: &str = "Combination withheld: a variable spring hanger's cold load would be counted by the sum of factors instead of once. The difference of two solved cases is available as a result state subtraction.";

fn solve(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).expect("the entry accepts the document")).unwrap()
}

fn solved(document: &Value, mode: PreviewSolverMode, what: &str) -> Value {
    let envelope = solve(document, mode);
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:#?}", envelope["diagnostics"]);
    envelope
}

fn diagnostics<'a>(envelope: &'a Value, code: &str) -> Vec<&'a Value> {
    envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code).collect()
}

fn pa(case_id: &str, route: Route) -> Value {
    document(case_id, route, MODEL_SCHEMA_VERSION)
}

/// P-A with PA-WT and PA-B as two cases of one document and the given combinations.
fn pa_two_cases(route: Route, combinations: Value) -> Value {
    let mut wt = pa("PA-WT", route);
    let b = pa("PA-B", route);
    wt["model"]["load_cases"].as_array_mut().unwrap().push(b["model"]["load_cases"][0].clone());
    wt["model"]["combinations"] = combinations;
    wt
}

fn hanger_mut<'a>(document: &'a mut Value, id: &str) -> &'a mut Value {
    document["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == id).unwrap()
}

#[test]
fn a_combination_counting_the_cold_load_other_than_once_is_withheld_and_a_unit_sum_is_admitted() {
    let combinations = json!([
        {"id": "comb:twice", "basis": "mechanics", "terms": [{"load_case": "PA-WT", "factor": 1.0}, {"load_case": "PA-B", "factor": 1.0}], "provenance": "invented"},
        {"id": "comb:once", "basis": "mechanics", "terms": [{"load_case": "PA-WT", "factor": 0.5}, {"load_case": "PA-B", "factor": 0.5}], "provenance": "invented"}]);
    for route in ROUTES {
        for mode in MODES {
            let what = format!("gate {route:?} {mode:?}");
            let envelope = solved(&pa_two_cases(route, combinations.clone()), mode, &what);
            let withheld = diagnostics(&envelope, GATE);
            assert_eq!(withheld.len(), 1, "{what}: {:#?}", envelope["diagnostics"]);
            assert!(withheld[0]["affected_refs"].as_array().unwrap().contains(&json!("comb:twice")), "{what}");
            assert_eq!(withheld[0]["message"], GATE_TEXT, "{what}");
            let rows_of = |id: &str| envelope["results"].as_array().unwrap().iter()
                .filter(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == id).count();
            assert_eq!(rows_of("comb:twice"), 0, "{what}");
            assert!(rows_of("comb:once") > 0, "{what}");
            // The admitted combination carries the cold load once.
            let applied = envelope["results"].as_array().unwrap().iter()
                .find(|r| r["basis_ref"]["ref_id"] == "comb:once" && r["kind"] == APPLIED && r["entity_ref"] == "SH-3")
                .unwrap_or_else(|| panic!("{what}: no combined applied force"));
            assert_close(applied["value"].as_f64().unwrap(), 6000.0, &what);
        }
    }
}

#[test]
fn a_spring_beside_a_nonlinear_support_agrees_with_its_force_plus_spring_twin() {
    let lift_off = json!({"id": "LO-4", "node": "N4", "family": "nonlinear", "restraints": [],
        "nonlinear": {"behavior": "lift_off", "dof": "UZ", "initial_state": "active", "contact_when": "positive_reaction"},
        "provenance": "invented"});
    for route in ROUTES {
        for mode in MODES {
            let what = format!("nonlinear {route:?} {mode:?}");
            let mut with_hanger = pa("PA-WT", route);
            let mut twin = authored("PA-WT", route, Hangers::Twin, MODEL_SCHEMA_VERSION);
            for doc in [&mut with_hanger, &mut twin] {
                doc["model"]["supports"].as_array_mut().unwrap().push(lift_off.clone());
                if let Some(state) = doc["model"]["load_cases"][0].get_mut("analysis_state") {
                    state["support_states"].as_array_mut().unwrap().push(json!({"support_ref": "LO-4", "participation": {"kind": "active_model_device"}}));
                }
            }
            let hanger = solved(&with_hanger, mode, &what);
            let twin = solved(&twin, mode, &what);
            for node in ["N1", "N2", "N3", "N4", "N5"] {
                assert_close(uz(&hanger, node), uz(&twin, node), &format!("{what} u {node}"));
            }
            assert_close(fz(&hanger, "SH-3"), fz(&twin, "SH-3") + 6000.0, &format!("{what} SH-3"));
            assert_close(fz(&hanger, "LO-4"), fz(&twin, "LO-4"), &format!("{what} LO-4"));
            assert!(fz(&hanger, "LO-4") > 0.0, "{what}: the lift-off support stays in contact");
            assert_close(applied(&hanger, "SH-3"), 6000.0, &what);
        }
    }
}

#[test]
fn a_spring_that_does_not_apply_its_cold_load_says_so_and_publishes_no_applied_force() {
    // A variable spring hanger on a rotational DOF: a cold load is a force, so it is not applied.
    let rotational = json!({"id": "SH-R", "node": "N2", "family": "variable_spring_hanger", "restraints": ["RY"],
        "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "RY", "value": q(1.0e6, "N*m/rad")},
            "installed_load": q(500.0, "N"), "cold_load": q(500.0, "N"), "hot_load": q(500.0, "N"),
            "travel_range": q(0.05, "m"), "source_reference": "invented"},
        "provenance": "invented"});
    for route in ROUTES {
        for mode in MODES {
            let what = format!("not applied {route:?} {mode:?}");
            let mut doc = pa("PA-WT", route);
            doc["model"]["supports"].as_array_mut().unwrap().push(rotational.clone());
            let envelope = solved(&doc, mode, &what);
            let named = diagnostics(&envelope, NOT_APPLIED);
            assert_eq!(named.len(), 1, "{what}: {:#?}", envelope["diagnostics"]);
            assert_eq!(named[0]["severity"], "warning", "{what}");
            assert_eq!(named[0]["affected_refs"], json!(["SH-R", "PA-WT"]), "{what}");
            assert!(named[0]["message"].as_str().unwrap().contains("stiffness DOF RY is rotational"), "{what}");
            assert!(rows(&envelope, APPLIED, "SH-R", None).is_empty(), "{what}");
            // SH-3 still applies its own.
            assert_close(applied(&envelope, "SH-3"), 6000.0, &what);
        }
    }
}

#[test]
fn an_installed_load_that_differs_from_the_cold_load_warns_and_nothing_else_does() {
    for route in ROUTES {
        for mode in MODES {
            let what = format!("installed {route:?} {mode:?}");
            // Equal in value after unit normalization, and a different hot load: no warning.
            let mut quiet = pa("PA-WT", route);
            hanger_mut(&mut quiet, "SH-3")["hanger"]["installed_load"] = q(6.0, "kN");
            hanger_mut(&mut quiet, "SH-3")["hanger"]["hot_load"] = q(7100.0, "N");
            let envelope = solved(&quiet, mode, &what);
            assert!(diagnostics(&envelope, INSTALLED_DIFFERS).is_empty(), "{what}");
            assert!(diagnostics(&envelope, RETIRED).is_empty(), "{what}");
            // A differing installed load warns once; the solve still applies the cold load.
            let mut differs = pa("PA-WT", route);
            hanger_mut(&mut differs, "SH-3")["hanger"]["installed_load"] = q(6500.0, "N");
            let envelope = solved(&differs, mode, &what);
            let named = diagnostics(&envelope, INSTALLED_DIFFERS);
            assert_eq!(named.len(), 1, "{what}");
            assert_eq!(named[0]["severity"], "warning", "{what}");
            assert_eq!(named[0]["affected_refs"], json!(["SH-3", "hanger.installed_load", "hanger.cold_load"]), "{what}");
            assert!(diagnostics(&envelope, RETIRED).is_empty(), "{what}");
            assert_close(applied(&envelope, "SH-3"), 6000.0, &what);
            let reference = reference();
            let expected = &model_of(&reference, "PA-WT").1["expected"];
            assert_close(fz(&envelope, "SH-3"), expected["published_support_force_n"]["SH-3"].as_f64().unwrap(), &what);
        }
    }
}

#[test]
fn a_test_condition_case_is_still_refused_before_any_solve_with_a_spring_in_the_model() {
    for route in ROUTES {
        for mode in MODES {
            let what = format!("test condition {route:?} {mode:?}");
            let mut doc = pa("PA-WT", route);
            doc["model"]["load_cases"][0]["test_condition"] = json!({"kind": "hydrostatic", "held_supports": "springs_and_constant_effort"});
            let envelope = solve(&doc, mode);
            assert_ne!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}");
            let refused = diagnostics(&envelope, "LOAD_CASE_TEST_CONDITION_UNSUPPORTED");
            assert_eq!(refused.len(), 1, "{what}: {:#?}", envelope["diagnostics"]);
            assert_eq!(refused[0]["severity"], "blocking", "{what}");
            assert!(envelope["results"].as_array().is_none_or(|r| r.is_empty()), "{what}: a refused document publishes no row");
        }
    }
}

/// DEL-05-02's count rule for an operand whose fixed-load count is undefined, with the
/// spring's cold load the only fixed-load source (P-A on both routes and modes):
/// `c:sub` (PA-WT − PA-B) has κ = 1 − 1 = 0; the range envelope `c:env_mixed` of
/// PA-WT (κ = 1) and `c:sub` (κ = 0) has no common κ, so its κ is undefined, and an
/// envelope carries no gate of its own, so it publishes; the mechanics combination
/// `c:mech_of_env` (1.0 · `c:env_mixed`) names an operand with an undefined κ, its
/// operand is not withheld (that rule is tested first), and it is withheld under
/// CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED with the spring named as the source.
#[test]
fn a_mechanics_combination_of_an_operand_with_an_undefined_count_is_withheld_as_mixed() {
    const MIXED: &str = "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED";
    const MIXED_TEXT: &str = "Combination withheld: an operand's fixed-load count is undefined, so the number of times this combination counts a variable spring hanger's cold load cannot be decided.";
    let combinations = json!([
        {"id": "c:sub", "basis": "result_state_subtraction", "minuend_id": "PA-WT", "subtrahend_id": "PA-B", "provenance": "invented"},
        {"id": "c:env_mixed", "basis": "range_envelope", "operand_ids": ["PA-WT", "c:sub"], "mode": "max_abs", "provenance": "invented"},
        {"id": "c:mech_of_env", "basis": "mechanics", "terms": [{"combination": "c:env_mixed", "factor": 1.0}], "provenance": "invented"}]);
    for route in ROUTES {
        for mode in MODES {
            let what = format!("operand undefined {route:?} {mode:?}");
            let envelope = solved(&pa_two_cases(route, combinations.clone()), mode, &what);
            let rows_of = |id: &str| envelope["results"].as_array().unwrap().iter()
                .filter(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == id).count();
            assert!(rows_of("c:sub") > 0, "{what}");
            assert!(rows_of("c:env_mixed") > 0, "{what}: an envelope carries no gate of its own");
            assert_eq!(rows_of("c:mech_of_env"), 0, "{what}");
            let mixed = diagnostics(&envelope, MIXED);
            assert_eq!(mixed.len(), 1, "{what}: {:#?}", envelope["diagnostics"]);
            assert_eq!(mixed[0]["affected_refs"], json!(["c:mech_of_env"]), "{what}");
            assert_eq!(mixed[0]["message"], MIXED_TEXT, "{what}");
            for code in [GATE, "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND", "LOAD_COMBINATION_OPERAND_WITHHELD"] {
                assert!(diagnostics(&envelope, code).is_empty(), "{what}: {code}");
            }
        }
    }
}

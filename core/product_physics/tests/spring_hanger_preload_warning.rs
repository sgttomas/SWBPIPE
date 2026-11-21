//! DEL-04-11 REQ-014: the two warnings of the variable spring preload, which
//! replace the retired `SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD`.
//!
//! `SPRING_HANGER_COLD_LOAD_NOT_APPLIED` names each spring whose cold load a
//! case does not apply, with the reason. Of the seven reasons of
//! `spring_preload::NotApplied`, a document that passes validation reaches
//! two: `nonlinear_record` (a nonlinear member on another DOF; on the
//! stiffness DOF the record is refused, SUPPORT_LINEAR_NONLINEAR_DOF_CONFLICT)
//! and `rotational_dof`. Validation refuses the other five first:
//! `skewed_record` by SUPPORT_DIRECTION_RECORD_UNSUPPORTED (a direction is not
//! an input of a hanger), `cold_load_missing` and `cold_load_not_positive` by
//! SPRING_HANGER_LOAD_MISSING, `stiffness_dof_unreadable` by
//! SPRING_HANGER_STIFFNESS_MISSING (or SUPPORT_STIFFNESS_DOF_INVALID for an
//! unknown token), and `unknown_node` by SUPPORT_NODE_UNKNOWN.
//!
//! `SPRING_HANGER_INSTALLED_LOAD_DIFFERS_FROM_COLD_LOAD` names an entered
//! installed load that differs from the cold load after unit normalization;
//! equal loads in different units are quiet. Invented inputs only.

use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const NOT_APPLIED: &str = "SPRING_HANGER_COLD_LOAD_NOT_APPLIED";
const DIFFERS: &str = "SPRING_HANGER_INSTALLED_LOAD_DIFFERS_FROM_COLD_LOAD";
const APPLIED: &str = "variable_spring_hanger_applied_load";
const HANGER: &str = "support:SH";
const PROV: &str = "invented_830_hanger_preload_input_no_library_data";
const EXACT: &str = include_str!("fixtures/shared_terminal_junction.request.json");
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

fn hanger(node: &str) -> Value {
    json!({"id": HANGER, "node": node, "family": "variable_spring_hanger", "restraints": [],
        "hanger": {"hanger_type": "variable_spring_hanger",
                   "stiffness": {"dof": "UZ", "value": q(42_000.0, "N/m")},
                   "installed_load": q(460.0, "N"), "cold_load": q(430.0, "N"), "hot_load": q(390.0, "N"),
                   "travel_range": q(0.045, "m"),
                   "manufacturer_reference": "invented_user_entered_variable_hanger_reference_no_catalog",
                   "source_reference": "invented_user_entered_spring_hanger_values_no_catalog",
                   "load_side_review_reference": "invented_review_only_hot_cold_load_metadata",
                   "mechanics_consumption": "linear_spring_primitive_user_stiffness"},
        "provenance": PROV})
}

/// A pressure-free two-member cantilever along x, anchored at N0, with the
/// hanger at its tip and a downward tip load.
fn pressure_free() -> Value {
    let nodes: Vec<Value> = (0..=2)
        .map(|i| json!({"id": format!("N{i}"), "position": {"x": i as f64, "y": 0.0, "z": 0.0}, "provenance": PROV}))
        .collect();
    let pipes: Vec<Value> = (1..=2)
        .map(|i| {
            json!({"id": format!("M{i}"), "from": format!("N{}", i - 1), "to": format!("N{i}"), "material": "mat:H",
                   "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
                   "section": {"outside_diameter": q(0.2, "m"), "wall_thickness": q(0.01, "m")},
                   "provenance": PROV})
        })
        .collect();
    json!({"model": {
        "schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": "invented:830:hanger-preload",
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": nodes,
        "pipe_segments": pipes,
        "materials": [{"id": "mat:H", "elastic_modulus": q(2.0e11, "Pa"),
                       "shear_modulus": q(8.0e10, "Pa"), "provenance": PROV}],
        "supports": [
            {"id": "anchor:N0", "node": "N0", "family": "anchor",
             "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
            hanger("N2")],
        "load_cases": [{"id": "case", "label": "tip", "kind": "primitive_user_load",
                        "primitive_loads": [{"id": "load:tip", "category": "concentrated_force",
                            "target": {"type": "node", "node": "N2"},
                            "direction": "global_z", "magnitude": q(-500.0, "N"),
                            "dimension": "force", "provenance": PROV}],
                        "provenance": PROV}],
        "combinations": []
    }, "materials": []})
}

/// The exact-route tee of correction 1, every terminal transferring, with the
/// hanger at the branch end.
fn exact() -> Value {
    let mut document: Value = serde_json::from_str(EXACT).unwrap();
    for region in document["model"]["load_cases"][0]["pressure_regions"].as_array_mut().unwrap() {
        for terminal in region["terminals"].as_array_mut().unwrap() {
            terminal["closure_transfer"] = json!("transfers_to_wall");
        }
    }
    document["model"]["supports"].as_array_mut().unwrap().push(hanger("node:C"));
    document
}

fn hanger_mut(document: &mut Value) -> &mut Value {
    document["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == HANGER).unwrap()
}

fn run(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).unwrap()).unwrap()
}

fn with_code<'a>(envelope: &'a Value, code: &str) -> Vec<&'a Value> {
    envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code).collect()
}

/// The case's one not-applied warning names the support, the case and `reason`,
/// and the case publishes no applied force for the spring.
fn not_applied(document: &Value, case: &str, reason: &str, what: &str) {
    for mode in MODES {
        let what = format!("{what} {mode:?}");
        let envelope = run(document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:#?}", envelope["diagnostics"]);
        let warned = with_code(&envelope, NOT_APPLIED);
        assert_eq!(warned.len(), 1, "{what}: {:#?}", envelope["diagnostics"]);
        assert_eq!(warned[0]["severity"], "warning", "{what}");
        assert_eq!(warned[0]["id"], format!("diagnostic:spring-hanger:support-SH:cold-load-not-applied:{}", case.replace(':', "-")), "{what}");
        assert_eq!(warned[0]["affected_refs"], json!([HANGER, case]), "{what}");
        assert_eq!(warned[0]["message"], format!("variable spring hanger {HANGER}: its cold load is not applied in load case {case}, because {reason}; the spring acts with its rate only, and no applied force is published for it in this case"), "{what}");
        let applied = envelope["results"].as_array().unwrap().iter().filter(|r| r["kind"] == APPLIED).count();
        assert_eq!(applied, 0, "{what}");
    }
}

#[test]
fn a_nonlinear_member_on_another_dof_is_named_as_not_applying_the_cold_load() {
    let mut document = pressure_free();
    hanger_mut(&mut document)["nonlinear"] = json!({"behavior": "one_way", "dof": "UX", "initial_state": "active", "active_when": "positive_reaction"});
    not_applied(&document, "case", "the record carries a nonlinear member, whose own law governs its DOF", "nonlinear");
}

#[test]
fn a_rotational_stiffness_dof_is_named_on_both_routes() {
    for (route, mut document, case) in [("pressure-free", pressure_free(), "case".to_string()), ("exact", exact(), String::new())] {
        let case = if case.is_empty() { document["model"]["load_cases"][0]["id"].as_str().unwrap().to_string() } else { case };
        let support = hanger_mut(&mut document);
        support["hanger"]["stiffness"] = json!({"dof": "RY", "value": q(42_000.0, "N*m/rad")});
        not_applied(&document, &case, "its stiffness DOF RY is rotational, and the cold load is a force", route);
    }
}

/// The five reasons validation refuses before any solve, each by its rule.
#[test]
fn the_other_reasons_are_refused_by_validation_first() {
    let cases: [(&str, &str, fn(&mut Value)); 6] = [
        ("skewed_record", "SUPPORT_DIRECTION_RECORD_UNSUPPORTED", |s| {
            s["family"] = json!("spring");
            s["restraints"] = json!(["UZ"]);
            s["direction"] = json!({"x": 0.0, "y": 0.6, "z": 0.8});
        }),
        ("cold_load_missing", "SPRING_HANGER_LOAD_MISSING", |s| { s["hanger"].as_object_mut().unwrap().remove("cold_load"); }),
        ("cold_load_not_positive", "SPRING_HANGER_LOAD_MISSING", |s| { s["hanger"]["cold_load"] = q(-5.0, "N"); }),
        ("stiffness_dof_unreadable", "SPRING_HANGER_STIFFNESS_MISSING", |s| { s["hanger"].as_object_mut().unwrap().remove("stiffness"); }),
        ("stiffness_dof_unreadable (unknown token)", "SUPPORT_STIFFNESS_DOF_INVALID", |s| { s["hanger"]["stiffness"]["dof"] = json!("UQ"); }),
        ("unknown_node", "SUPPORT_NODE_UNKNOWN", |s| { s["node"] = json!("N9"); }),
    ];
    for (reason, code, patch) in cases {
        let mut document = pressure_free();
        patch(hanger_mut(&mut document));
        let envelope = run(&document, MODES[0]);
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{reason}");
        let blocking: Vec<&Value> = envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking").collect();
        assert!(blocking.iter().any(|d| d["code"] == code), "{reason}: {blocking:#?}");
        assert!(with_code(&envelope, NOT_APPLIED).is_empty(), "{reason}");
    }
}

#[test]
fn an_installed_load_differing_from_the_cold_load_is_named_on_both_routes() {
    for (route, document) in [("pressure-free", pressure_free()), ("exact", exact())] {
        for mode in MODES {
            let what = format!("{route} {mode:?}");
            let envelope = run(&document, mode);
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:#?}", envelope["diagnostics"]);
            let warned = with_code(&envelope, DIFFERS);
            assert_eq!(warned.len(), 1, "{what}");
            assert_eq!(warned[0]["severity"], "warning", "{what}");
            assert_eq!(warned[0]["id"], "diagnostic:spring-hanger:support-SH:installed-differs-from-cold", "{what}");
            assert_eq!(warned[0]["affected_refs"], json!([HANGER, "hanger.installed_load", "hanger.cold_load"]), "{what}");
            assert_eq!(warned[0]["message"], format!("variable spring hanger {HANGER}: its installed load (460 N) differs from its cold load (430 N); the solve applies the cold load, and the installed load is review data only"), "{what}");
            // The cold load is applied; the hanger's other loads are not named.
            assert!(with_code(&envelope, NOT_APPLIED).is_empty(), "{what}");
        }
    }
}

#[test]
fn equal_loads_in_different_units_are_quiet() {
    for (installed, cold) in [(q(0.43, "kN"), q(430.0, "N")), (q(430.0, "N"), q(430.0, "N"))] {
        let mut document = pressure_free();
        let support = hanger_mut(&mut document);
        support["hanger"]["installed_load"] = installed.clone();
        support["hanger"]["cold_load"] = cold;
        for mode in MODES {
            let envelope = run(&document, mode);
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{installed} {mode:?}");
            assert!(with_code(&envelope, DIFFERS).is_empty(), "{installed} {mode:?}: {:#?}", with_code(&envelope, DIFFERS));
        }
    }
}

// ---------------------------------------------------------------------------
// The review rows' two keys (DEL-04-11 REQ-014): `authored_mechanics_consumption`
// holds the document's own `hanger.mechanics_consumption`, verbatim, or
// `mechanics_consumption_missing`; `mechanics_consumption` holds what the solve
// consumed, in the product's words, from the solve's own decision.
// ---------------------------------------------------------------------------

const REVIEW: &str = "spring_hanger_user_input_review";
const REVIEW_SENTENCE: &str = "positive value is user-entered variable spring hanger input evidence; load-side and travel review remain human-reviewed preview metadata";
const COLD_APPLIED_SENTENCE: &str = "positive value is the user-entered variable spring hanger cold load, applied in the solve as a constant force along the positive axis of its stiffness DOF in every load case";

/// The hanger's review rows by field (`stiffness` for the rate row): (basis, sign convention).
fn review_rows(envelope: &Value) -> Vec<(String, String, String)> {
    envelope["results"].as_array().unwrap().iter()
        .filter(|r| r["kind"] == REVIEW && r["entity_ref"] == HANGER)
        .map(|r| {
            let id = r["id"].as_str().unwrap();
            let field = id.rsplit(':').next().unwrap().to_string();
            (field, r["metadata"]["basis"].as_str().unwrap().to_string(), r["metadata"]["sign_convention"].as_str().unwrap().to_string())
        })
        .collect()
}

/// Every review row states `authored` under the authored key and, under the product's
/// key, what the solve consumed: the rate in the stiffness row, `cold` in the cold-load
/// row, review only elsewhere; the cold-load row's sentence is `cold_sentence`.
fn two_keys(document: &Value, authored: &str, cold: &str, cold_sentence: &str, what: &str) {
    for mode in MODES {
        let what = format!("{what} {mode:?}");
        let envelope = run(document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:#?}", envelope["diagnostics"]);
        let rows = review_rows(&envelope);
        let fields: Vec<&str> = rows.iter().map(|(f, _, _)| f.as_str()).collect();
        assert_eq!(fields, ["stiffness", "installed-load", "cold-load", "hot-load", "travel-range"], "{what}");
        for (field, basis, sign) in &rows {
            let consumed = match field.as_str() { "stiffness" => "assembled_solve_spring_rate", "cold-load" => cold, _ => "review_only" };
            let keys = format!(";authored_mechanics_consumption={authored};mechanics_consumption={consumed};dec_ref=DEC-049");
            assert!(basis.ends_with(&keys), "{what} {field}: {basis}");
            assert_eq!(basis.matches("mechanics_consumption=").count(), 2, "{what} {field}: one authored and one product statement");
            match field.as_str() {
                "cold-load" => assert_eq!(sign, cold_sentence, "{what}"),
                "stiffness" => {}
                _ => assert_eq!(sign, REVIEW_SENTENCE, "{what} {field}"),
            }
        }
    }
}

/// By the integrator's description: a document whose authored string is the old
/// `linear_spring_primitive_user_stiffness`, solved under the law, shows on every
/// review row the authored key with the old words and the product's key with what
/// the solve did (the rate and the cold load in the assembled solve).
#[test]
fn an_authored_linear_spring_consumption_is_kept_under_its_own_key_beside_what_the_solve_consumed() {
    for (route, document) in [("pressure-free", pressure_free()), ("exact", exact())] {
        two_keys(&document, "linear_spring_primitive_user_stiffness", "assembled_solve_cold_load", COLD_APPLIED_SENTENCE, route);
    }
}

#[test]
fn a_document_without_the_member_is_stated_as_missing_and_the_product_still_says_what_it_consumed() {
    let mut document = pressure_free();
    hanger_mut(&mut document)["hanger"].as_object_mut().unwrap().remove("mechanics_consumption");
    two_keys(&document, "mechanics_consumption_missing", "assembled_solve_cold_load", COLD_APPLIED_SENTENCE, "no member");
}

/// A spring whose cold load is not applied (a rotational stiffness DOF): the cold-load
/// row names the reason's token and its sentence says the load is not applied.
#[test]
fn a_cold_load_not_applied_is_stated_with_its_reason_token() {
    let mut document = pressure_free();
    hanger_mut(&mut document)["hanger"]["stiffness"] = json!({"dof": "RY", "value": q(42_000.0, "N*m/rad")});
    two_keys(&document, "linear_spring_primitive_user_stiffness", "cold_load_not_applied_rotational_dof",
        "positive value is the user-entered variable spring hanger cold load, not applied in the solve because its stiffness DOF RY is rotational, and the cold load is a force; the spring acts with its rate only",
        "rotational");
}

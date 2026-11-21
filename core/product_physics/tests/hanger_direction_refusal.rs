//! A hanger's axis and the record that carries it (DEL-04-03 REQ-014, DEL-04-11
//! REQ-014). A hanger's axis is its stiffness DOF (a variable spring hanger) or
//! its single translational restraint (a constant-effort hanger), and its load
//! acts along the positive axis of that DOF. A `direction` is not an input of a
//! hanger: a record that carries a hanger and a direction is refused by name
//! (`SUPPORT_DIRECTION_RECORD_UNSUPPORTED`), whatever the direction's sign or
//! axis and whatever the record's family, with the one message that corrects it. A `line_stop` family record carrying either hanger family is refused
//! as a family and hanger conflict (`SUPPORT_FAMILY_HANGER_CONFLICT`), as a
//! `spring` family record carrying a constant-effort hanger already is. Every
//! refusal names the record and publishes no result row.
//!
//! Model P-A of the frozen preload reference (`validation/references/hanger_sizing/
//! preload.json`), with SH-3 authored in each form, on both routes and both
//! modes. The control without a direction compares with the reference's own
//! expected values, and SH-3's published row with F_cold - k*(u - g) formed from
//! the reference's own inputs and expected hanger displacement. Invented data.

use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};

#[path = "common/spring_preload_model.rs"]
mod model;
use model::*;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const DIRECTION_REFUSED: &str = "SUPPORT_DIRECTION_RECORD_UNSUPPORTED";
const CONFLICT: &str = "SUPPORT_FAMILY_HANGER_CONFLICT";
const HANGER_REASON: &str = "is not an input of a hanger: a hanger's load acts along the positive axis of its stiffness DOF (or of its single translational restraint), so remove the direction.";
const FAMILY_REASON: &str = "is admitted only on an explicit line_stop, spring or snubber family.";

fn solve(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).expect("the entry accepts the document")).unwrap()
}

/// A reference case's document with SH-3 rewritten by `edit`.
fn with_sh3(case_id: &str, route: Route, edit: impl Fn(&mut Value)) -> Value {
    let mut document = document(case_id, route, MODEL_SCHEMA_VERSION);
    let sh3 = document["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "SH-3").unwrap();
    edit(sh3);
    document
}

/// SH-3 as a constant-effort hanger of the given family, holding the reference's cold load.
fn constant_effort(sh3: &mut Value, family: &str) {
    *sh3 = json!({"id": "SH-3", "node": "N3", "family": family, "restraints": ["UZ"],
        "hanger": {"hanger_type": "constant_effort_support", "constant_load": q(6000.0, "N"),
            "travel_range": q(0.05, "m"), "source_reference": "invented"},
        "provenance": PROV});
}

fn direction(sh3: &mut Value, [x, y, z]: [f64; 3]) {
    sh3["direction"] = json!({"x": x, "y": y, "z": z});
}

/// The document is refused by `code` naming SH-3 alone, with `message`, and publishes no row.
fn refused(document: &Value, code: &str, message: &str, what: &str) {
    for mode in MODES {
        let envelope = solve(document, mode);
        let what = format!("{what} {mode:?}");
        assert_ne!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}");
        assert_eq!(envelope["results"].as_array().map(Vec::len), Some(0), "{what}");
        let named: Vec<&Value> = envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code).collect();
        assert_eq!(named.len(), 1, "{what}: {:#?}", envelope["diagnostics"]);
        assert_eq!(named[0]["severity"], "blocking", "{what}");
        assert_eq!(named[0]["affected_refs"], json!(["SH-3"]), "{what}");
        assert_eq!(named[0]["message"], message, "{what}");
    }
}

fn direction_message(reason: &str) -> String {
    format!("supports[4].direction of support `SH-3` {reason}")
}

const DIRECTIONS: [[f64; 3]; 5] = [[0.0, 0.0, 1.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 0.0, 1.0]];

#[test]
fn sh3_is_the_fifth_support_of_the_reference_model() {
    let document = document("PA-WT", Route::PressureFree, MODEL_SCHEMA_VERSION);
    assert_eq!(document["model"]["supports"][4]["id"], "SH-3");
}

/// A variable spring hanger on the `spring` family, the family on which a
/// direction is otherwise admitted, with any direction: refused by name, of
/// either sign, along its stiffness DOF or another axis, or skewed.
#[test]
fn a_variable_spring_hanger_with_a_direction_is_refused_by_name() {
    for route in ROUTES {
        for d in DIRECTIONS {
            let document = with_sh3("PA-WT", route, |sh3| {
                sh3["family"] = json!("spring");
                direction(sh3, d);
            });
            refused(&document, DIRECTION_REFUSED, &direction_message(HANGER_REASON), &format!("spring {d:?} {route:?}"));
        }
        // Its own family, and a family on which no direction is admitted, are told the same.
        for family in ["variable_spring_hanger", "spring_hanger", "anchor"] {
            let document = with_sh3("PA-WT", route, |sh3| {
                sh3["family"] = json!(family);
                direction(sh3, [0.0, 0.0, -1.0]);
            });
            refused(&document, DIRECTION_REFUSED, &direction_message(HANGER_REASON), &format!("{family} {route:?}"));
        }
    }
}

/// A constant-effort hanger with a direction is refused by the same message on
/// its own family and on the `spring` family (a direction is checked before the
/// family and hanger conflict).
#[test]
fn a_constant_effort_hanger_with_a_direction_is_refused_by_name() {
    for route in ROUTES {
        for d in DIRECTIONS {
            let own = with_sh3("PA-WT", route, |sh3| {
                constant_effort(sh3, "constant_effort_support");
                direction(sh3, d);
            });
            refused(&own, DIRECTION_REFUSED, &direction_message(HANGER_REASON), &format!("constant_effort_support {d:?} {route:?}"));
            let spring = with_sh3("PA-WT", route, |sh3| {
                constant_effort(sh3, "spring");
                direction(sh3, d);
            });
            refused(&spring, DIRECTION_REFUSED, &direction_message(HANGER_REASON), &format!("spring constant-effort {d:?} {route:?}"));
        }
    }
}

/// Control: a record of a hanger family that carries no hanger member (refused on
/// its own as `SPRING_HANGER_DATA_MISSING`) has no hanger to be told about, and a
/// direction on it keeps the family's words.
#[test]
fn a_hanger_family_record_without_a_hanger_keeps_the_family_refusal() {
    for route in ROUTES {
        for family in ["variable_spring_hanger", "constant_effort_support"] {
            let document = with_sh3("PA-WT", route, |sh3| {
                *sh3 = json!({"id": "SH-3", "node": "N3", "family": family, "restraints": ["UZ"], "provenance": PROV});
                direction(sh3, [0.0, 0.0, 1.0]);
            });
            refused(&document, DIRECTION_REFUSED, &direction_message(FAMILY_REASON), &format!("{family} without a hanger {route:?}"));
            for mode in MODES {
                let envelope = solve(&document, mode);
                assert!(envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "SPRING_HANGER_DATA_MISSING"),
                    "{family} {route:?} {mode:?}: {:#?}", envelope["diagnostics"]);
            }
        }
    }
}

#[test]
fn a_line_stop_carrying_a_variable_spring_hanger_is_a_family_and_hanger_conflict() {
    for route in ROUTES {
        let document = with_sh3("PA-WT", route, |sh3| sh3["family"] = json!("line_stop"));
        refused(&document, CONFLICT,
            "line_stop family conflicts with variable spring hanger type; choose one support law so stiffness and applied force cannot be silently substituted",
            &format!("line_stop variable spring {route:?}"));
        // With a direction, the direction is refused first.
        let directed = with_sh3("PA-WT", route, |sh3| {
            sh3["family"] = json!("line_stop");
            direction(sh3, [0.0, 0.0, -1.0]);
        });
        refused(&directed, DIRECTION_REFUSED, &direction_message(HANGER_REASON), &format!("line_stop variable spring directed {route:?}"));
    }
}

#[test]
fn a_line_stop_carrying_a_constant_effort_hanger_is_a_family_and_hanger_conflict() {
    for route in ROUTES {
        let document = with_sh3("PA-WT", route, |sh3| constant_effort(sh3, "line_stop"));
        refused(&document, CONFLICT,
            "line_stop family conflicts with constant-effort hanger type; choose one support law so stiffness and applied force cannot be silently substituted",
            &format!("line_stop constant-effort {route:?}"));
        // Main's spring family conflict keeps its words.
        let spring = with_sh3("PA-WT", route, |sh3| constant_effort(sh3, "spring"));
        refused(&spring, CONFLICT,
            "spring family conflicts with constant-effort hanger type; choose one support law so stiffness and applied force cannot be silently substituted",
            &format!("spring constant-effort {route:?}"));
    }
}

/// Control: a variable spring hanger without a direction, on its own family and
/// on the `spring` family, solves under the law: the reference's expected
/// displacements, support forces and applied force, and SH-3's row equal to
/// F_cold - k*(u - g) from the reference's inputs, with and without a moved base.
#[test]
fn a_variable_spring_hanger_without_a_direction_solves_under_the_law() {
    let reference = reference();
    for case_id in ["PA-WT", "PA-BASE"] {
        let (model, case) = model_of(&reference, case_id);
        let sh3 = model["supports"].as_array().unwrap().iter().find(|s| s["id"] == "SH-3").unwrap();
        let (cold, k) = (sh3["cold_load_n"].as_f64().unwrap(), sh3["stiffness_n_per_m"].as_f64().unwrap());
        let u = case["expected"]["u_hanger_m"]["SH-3"].as_f64().unwrap();
        let g = case["spring_base_displacements"].as_array().unwrap().iter()
            .find(|m| m["support"] == "SH-3").map_or(0.0, |m| m["value_m"].as_f64().unwrap());
        let row = cold - k * (u - g);
        assert_close(row, case["expected"]["published_support_force_n"]["SH-3"].as_f64().unwrap(), &format!("{case_id}: the reference's own row"));
        for family in ["variable_spring_hanger", "spring"] {
            for route in ROUTES {
                let document = with_sh3(case_id, route, |sh3| sh3["family"] = json!(family));
                for mode in MODES {
                    let what = format!("{case_id} {family} {route:?} {mode:?}");
                    let envelope = solve(&document, mode);
                    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:#?}", envelope["diagnostics"]);
                    assert!(!envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["severity"] == "blocking"), "{what}");
                    compare_block(&envelope, &case["expected"], &what);
                    assert_close(fz(&envelope, "SH-3"), row, &format!("{what}: F_cold - k(u - g)"));
                }
            }
        }
    }
}

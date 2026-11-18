//! DEL-04-11 REQ-014: the sign of a variable spring hanger record's single-axis
//! `direction`. The lowering (`document_members::lower_axis_aligned_directions`)
//! removes the direction before the preload decision runs; a negative one is kept as
//! a fact on the hanger record, and the decision then does not apply the cold load
//! (reason token `direction_opposes_positive_axis`): the spring acts with its rate
//! only. A positive one changes nothing. A single-axis direction on another axis than
//! the stiffness DOF's moves that DOF to the direction's axis.
//!
//! Model P-A of the frozen preload reference (`validation/references/hanger_sizing/
//! preload.json`), SH-3 authored as a `spring` record of hanger type
//! `variable_spring_hanger` (the families on which a direction is admitted), on both
//! routes and both modes. Expected values come from the reference: its PA-WT block,
//! and by superposition its PA-WT block less its PA-EMPTY block (the response to
//! the cold load alone) for the bare spring; and from the reference's closed form
//! for the off-axis record. Every result also passes the exact reader. Invented data.

use open_pipe_stress_product_physics::result_export::reader_verdict;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};

#[path = "common/spring_preload_model.rs"]
mod model;
use model::*;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const NOT_APPLIED: &str = "SPRING_HANGER_COLD_LOAD_NOT_APPLIED";
const REVIEW: &str = "spring_hanger_user_input_review";
const REASON: &str = "the record's direction points along the negative axis of its stiffness DOF, and the cold load acts only along the positive axis";

fn solve(document: &Value, mode: PreviewSolverMode, what: &str) -> Value {
    let envelope = serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).expect("the entry accepts the document")).unwrap();
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:#?}", envelope["diagnostics"]);
    envelope
}

/// PA-WT with SH-3 a `spring` record of hanger type `variable_spring_hanger`, with `direction`.
fn pa_wt(route: Route, direction: Option<[f64; 3]>) -> Value {
    let mut document = document("PA-WT", route, MODEL_SCHEMA_VERSION);
    let sh3 = document["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "SH-3").unwrap();
    sh3["family"] = json!("spring");
    if let Some([x, y, z]) = direction {
        sh3["direction"] = json!({"x": x, "y": y, "z": z});
    }
    document
}

fn expected(case_id: &str) -> Value {
    let reference = reference();
    model_of(&reference, case_id).1["expected"].clone()
}

fn not_applied<'a>(envelope: &'a Value) -> Vec<&'a Value> {
    envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == NOT_APPLIED).collect()
}

fn applied_rows(envelope: &Value) -> usize {
    envelope["results"].as_array().unwrap().iter().filter(|r| r["kind"] == APPLIED).count()
}

fn cold_load_review_basis(envelope: &Value) -> String {
    let rows: Vec<&Value> = envelope["results"].as_array().unwrap().iter()
        .filter(|r| r["kind"] == REVIEW && r["entity_ref"] == "SH-3" && r["id"].as_str().unwrap().ends_with(":cold-load")).collect();
    assert_eq!(rows.len(), 1);
    rows[0]["metadata"]["basis"].as_str().unwrap().to_string()
}

/// The exact reader accepts the result, with and without the model (the preview
/// reader waits for the contract tables to admit the applied-force kind).
fn exact_reader_accepts(route: Route, envelope: &Value, document: &Value, what: &str) {
    if route == Route::Exact {
        assert_eq!(reader_verdict::validate("pressure", envelope, Some(&document["model"]), None), json!({"ok": true}), "{what}");
        assert_eq!(reader_verdict::validate("pressure", envelope, None, None), json!({"ok": true}), "{what} (no model)");
    }
}

#[test]
fn a_positive_single_axis_direction_changes_no_byte() {
    for route in ROUTES {
        for mode in MODES {
            let what = format!("+Z {route:?} {mode:?}");
            let bare = pa_wt(route, None);
            let none = solve(&bare, mode, &what);
            for d in [[0.0, 0.0, 1.0], [-0.0, 0.0, 2.5]] {
                let document = pa_wt(route, Some(d));
                let positive = solve(&document, mode, &what);
                assert_eq!(serde_json::to_string(&positive).unwrap(), serde_json::to_string(&none).unwrap(), "{what} {d:?}");
                compare_block(&positive, &expected("PA-WT"), &what);
                exact_reader_accepts(route, &positive, &document, &what);
            }
        }
    }
}

#[test]
fn a_negative_single_axis_direction_solves_as_the_bare_spring_and_says_why() {
    let wt = expected("PA-WT");
    let empty = expected("PA-EMPTY");
    for route in ROUTES {
        for mode in MODES {
            let what = format!("-Z {route:?} {mode:?}");
            let document = pa_wt(route, Some([0.0, 0.0, -1.0]));
            let envelope = solve(&document, mode, &what);
            // By superposition: PA-WT less the cold load's response alone (PA-EMPTY).
            for (node, u) in wt["u_nodes_m"].as_object().unwrap() {
                assert_close(uz(&envelope, node), u.as_f64().unwrap() - empty["u_nodes_m"][node].as_f64().unwrap(), &format!("{what} u {node}"));
            }
            for (support, force) in wt["published_support_force_n"].as_object().unwrap() {
                let bare = force.as_f64().unwrap() - empty["published_support_force_n"][support].as_f64().unwrap();
                assert_close(fz(&envelope, support), bare, &format!("{what} support force {support}"));
            }
            assert_eq!(applied_rows(&envelope), 0, "{what}");
            let warned = not_applied(&envelope);
            assert_eq!(warned.len(), 1, "{what}: {:#?}", envelope["diagnostics"]);
            let case = document["model"]["load_cases"][0]["id"].as_str().unwrap();
            assert_eq!(warned[0]["message"], format!("variable spring hanger SH-3: its cold load is not applied in load case {case}, because {REASON}; the spring acts with its rate only, and no applied force is published for it in this case"), "{what}");
            assert!(cold_load_review_basis(&envelope).contains(";mechanics_consumption=cold_load_not_applied_direction_opposes_positive_axis;"), "{what}");
            exact_reader_accepts(route, &envelope, &document, &what);
        }
    }
}

/// A single-axis direction along X on SH-3, whose stiffness DOF is UZ: the lowering
/// moves the DOF (and the restraint) to UX. Then nothing holds N3 along Z, which is
/// the simply supported span of the reference's closed form with k = 0; along X the
/// spring of rate k sits beside the bar N1-N3 (two 2 m members, EA/4), so +X applies
/// the cold load, u_x = F_cold / (EA/4 + k) and the spring publishes
/// F_cold - k u_x, while -X declines it (u_x = 0, no force), naming the reason.
#[test]
fn a_single_axis_direction_on_another_axis_moves_the_dof_to_it() {
    let reference = reference();
    let (model, _) = model_of(&reference, "PA-WT");
    let section = &model["section"];
    let (od, t, e) = (section["outside_diameter_m"].as_f64().unwrap(), section["wall_thickness_m"].as_f64().unwrap(), section["elastic_modulus_pa"].as_f64().unwrap());
    let id = od - 2.0 * t;
    let area = std::f64::consts::PI / 4.0 * (od * od - id * id);
    let inertia = std::f64::consts::PI / 64.0 * (od.powi(4) - id.powi(4));
    let (span, k, cold) = (8.0, 150_000.0, 6000.0);
    // f(x, a): deflection at x of a unit load at a on the simply supported span.
    let f = |x: f64, a: f64| {
        let (x, a) = if x <= a { (x, a) } else { (span - x, span - a) };
        let b = span - a;
        b * x * (span * span - b * b - x * x) / (6.0 * e * inertia * span)
    };
    // PA-WT: F-A -5000 N at N2 (x = 2), F-B -7000 N at N4 (x = 6).
    let u3 = f(4.0, 2.0) * -5000.0 + f(4.0, 6.0) * -7000.0;
    let ux = cold / (e * area / 4.0 + k);
    for route in ROUTES {
        for mode in MODES {
            for (sign, applies) in [(1.0, true), (-1.0, false)] {
                let what = format!("X {sign} {route:?} {mode:?}");
                let document = pa_wt(route, Some([sign, 0.0, 0.0]));
                let envelope = solve(&document, mode, &what);
                assert_close(uz(&envelope, "N3"), u3, &format!("{what} u_z N3"));
                let rows = rows(&envelope, "global_nodal_displacement_x", "N3", None);
                let unit = match rows[0]["unit"].as_str().unwrap() { "mm" => 1e-3, "m" => 1.0, other => panic!("{other}") };
                let published_ux = rows[0]["value"].as_f64().unwrap() * unit;
                let fx = rows_of_support(&envelope, "Fx");
                if applies {
                    assert_close(published_ux, ux, &format!("{what} u_x N3"));
                    assert_close(fx, cold - k * ux, &format!("{what} Fx"));
                    let applied: Vec<&Value> = envelope["results"].as_array().unwrap().iter().filter(|r| r["kind"] == APPLIED).collect();
                    assert_eq!(applied.len(), 1, "{what}");
                    assert_eq!(applied[0]["metadata"]["location"], "N3:UX applied load", "{what}");
                    assert!(not_applied(&envelope).is_empty(), "{what}");
                } else {
                    assert_eq!(published_ux, 0.0, "{what}");
                    assert_eq!(fx, 0.0, "{what}");
                    assert_eq!(applied_rows(&envelope), 0, "{what}");
                    assert_eq!(not_applied(&envelope).len(), 1, "{what}");
                    assert!(not_applied(&envelope)[0]["message"].as_str().unwrap().contains(REASON), "{what}");
                }
                assert_close(fz(&envelope, "SH-3"), 0.0, &format!("{what} Fz"));
                exact_reader_accepts(route, &envelope, &document, &what);
            }
        }
    }
}

fn rows_of_support(envelope: &Value, component: &str) -> f64 {
    let found = rows(envelope, "support_reaction_component_v2", "SH-3", Some(component));
    assert_eq!(found.len(), 1, "SH-3 {component}");
    assert_eq!(found[0]["unit"], "N");
    found[0]["value"].as_f64().unwrap()
}

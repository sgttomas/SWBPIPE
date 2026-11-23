//! DEL-05-01 REQ-014: the certificate of a hydrostatic test case's generated
//! self-weight. The certificate forms each such load's intended intensity from
//! the record's mass operands with the contents the model declares for it
//! (`hydrostatic::declared_contents`, captured from the model's condition
//! before the lowering, by a lookup separate from the resolution) and the
//! record's gravity (`certified_loads::intensity`); the solve applies the load
//! the contents resolution forms (`hydrostatic::apply_contents`). These checks
//! edit one side at a time through test-only mutants and require the
//! formation check to report the disagreement: the resolution itself, the
//! applied load (its contents, so its mass; its gravity; the load the contents
//! enter), the capture, and the certificate's contents operand. The document
//! is the four-span journey model
//! (`fixtures/hydrostatic_test/four_span_reader_models.json`).
//!
//! A mutation passes vacuously on a case that is already not qualified. The
//! first form of this repair read the model's conditions in the certificate,
//! after the lowering had cleared them, and had no effect; every mutation below
//! still passed, and only the unmutated baseline caught it. So each mutation
//! test solves the unmutated document first, in the same test, and requires it
//! to be qualified before it mutates. Standard claim fence applies (claims
//! taxonomy per DEC-081).
use super::*;
use crate::certified_loads::contents_operand_mutant;
use crate::hydrostatic::contents_mutant;
use serde_json::{json, Value};

const MODELS: &str = include_str!("../../../fixtures/hydrostatic_test/four_span_reader_models.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const TEST: &str = "HT-4S-TEST";
/// A contents density no condition of the model states (kg/m^3).
const OTHER_DENSITY: f64 = 940.0;

fn journey() -> Value {
    serde_json::from_str::<Value>(MODELS).unwrap()["models"]["journey"].clone()
}

fn solve(model: &Value, mode: PreviewSolverMode) -> Value {
    let out = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), mode)
        .unwrap_or_else(|error| panic!("typed boundary rejected the request: {error}"));
    serde_json::to_value(out).unwrap()
}

/// The test case's published solve quality and the code of its integrity
/// diagnostic.
fn standing(out: &Value) -> (String, String) {
    assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{:#}", out["diagnostics"]);
    let quality = out["numerical_quality"]["cases"].as_array().unwrap().iter()
        .find(|case| case["basis_ref"]["ref_id"] == TEST).expect("the test case's quality")["solve_quality"]
        .as_str().unwrap().to_string();
    let code = out["diagnostics"].as_array().unwrap().iter()
        .find(|d| d["id"] == format!("diagnostic:numerical-integrity:{TEST}")).expect("the test case's integrity diagnostic")["code"]
        .as_str().unwrap().to_string();
    (quality, code)
}

/// The test case's standing with a mutant armed, after requiring the
/// unmutated document to be qualified in the same mode.
fn under(mode: PreviewSolverMode, arm: impl FnOnce(), disarm: impl FnOnce()) -> (String, String) {
    assert_eq!(standing(&solve(&journey(), mode)), ("checks_passed".into(), "NUMERICAL_INTEGRITY_CHECKS_PASSED".into()),
        "{mode:?}: the unmutated baseline must be qualified before a mutation can show anything");
    arm();
    let out = solve(&journey(), mode);
    disarm();
    standing(&out)
}

#[test]
fn the_test_case_is_certified_with_its_selected_contents() {
    for mode in MODES {
        assert_eq!(standing(&solve(&journey(), mode)), ("checks_passed".into(), "NUMERICAL_INTEGRITY_CHECKS_PASSED".into()), "{mode:?}");
    }
}

#[test]
fn a_wrong_contents_operand_in_the_certificate_is_reported() {
    for mode in MODES {
        let found = under(mode, || contents_operand_mutant::arm(OTHER_DENSITY), contents_operand_mutant::disarm);
        assert_eq!(found, ("sensitive".into(), "NUMERICAL_INTEGRITY_SENSITIVE".into()), "{mode:?}");
    }
}

/// The applied load of the first pipe is formed with another contents density
/// (its mass with those contents, times the record's gravity), while the mark
/// and the model keep the selected condition's.
#[test]
fn an_applied_load_formed_with_other_contents_is_reported() {
    for mode in MODES {
        let found = under(mode, || contents_mutant::arm(Box::new(|model, applied| {
            let item = &mut applied[0];
            let other = Quantity { value: OTHER_DENSITY, unit: "kg/m^3".into() };
            let mass = crate::self_weight::mass_with_contents(model, &item.pipe_id, &other).expect("the mass with other contents");
            let gravity = item.intensity_n_per_m / item.mass_kg_per_m;
            item.contents_density = other;
            item.mass_kg_per_m = mass;
            item.intensity_n_per_m = mass * gravity;
        })), contents_mutant::disarm);
        assert_eq!(found, ("sensitive".into(), "NUMERICAL_INTEGRITY_SENSITIVE".into()), "{mode:?}");
    }
}

/// The applied load of the first pipe is formed with a gravity one part in a
/// thousand from the record's, with the selected contents.
#[test]
fn an_applied_load_formed_with_another_gravity_is_reported() {
    for mode in MODES {
        let found = under(mode, || contents_mutant::arm(Box::new(|_, applied| {
            applied[0].intensity_n_per_m *= 1.001;
        })), contents_mutant::disarm);
        assert_eq!(found, ("sensitive".into(), "NUMERICAL_INTEGRITY_SENSITIVE".into()), "{mode:?}");
    }
}

/// The first contents item names its own load, but the contents enter the
/// second pipe's load instead; the first load keeps the document's weight.
#[test]
fn contents_entering_another_load_are_reported() {
    for mode in MODES {
        let found = under(mode, || contents_mutant::arm(Box::new(|_, applied| {
            assert!(applied.len() > 1 && applied[0].load_id != applied[1].load_id);
            applied[0].load_id = applied[1].load_id.clone();
        })), contents_mutant::disarm);
        assert_eq!(found, ("sensitive".into(), "NUMERICAL_INTEGRITY_SENSITIVE".into()), "{mode:?}");
    }
}

/// The resolution itself is formed with another contents density, so the
/// applied load, the resolution and the mark agree with one another and not
/// with the model's condition. A certificate whose intended contents came from
/// the resolution could not report this; the declared contents come from the
/// model.
#[test]
fn a_resolution_formed_with_other_contents_is_reported() {
    for mode in MODES {
        let found = under(mode, || contents_mutant::arm_source_density(OTHER_DENSITY), contents_mutant::disarm);
        assert_eq!(found, ("sensitive".into(), "NUMERICAL_INTEGRITY_SENSITIVE".into()), "{mode:?}");
    }
}

/// No declared contents are captured, while the test contents enter every
/// generated self-weight load: the certificate has no intended contents for
/// them, and the case is not qualified.
#[test]
fn contents_with_no_declared_contents_are_not_qualified() {
    for mode in MODES {
        let found = under(mode, contents_mutant::arm_capture_dropped, contents_mutant::disarm);
        assert_eq!(found, ("sensitive".into(), "NUMERICAL_INTEGRITY_SENSITIVE".into()), "{mode:?}");
    }
}

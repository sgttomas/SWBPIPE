//! The maintained reference system (DEL-09-01, `fixtures/reference_system/`,
//! read only), whose generated self-weight records keep their sections in
//! millimetres, solved unmodified through PP's public entry: each of RS-SUS,
//! RS-OPE and RS-SAM publishes the verdict its metre twin publishes,
//! `NUMERICAL_INTEGRITY_CHECKS_PASSED`, in both solver modes, and no K-D5
//! formation finding is reported.
//!
//! The expected verdict is fixed by unit equivalence, not by the repaired
//! check. The metre twin is the same physical model: every section length
//! re-authored in m at the exact normalized binary64 value of its mm authoring,
//! and each generated record's `mass_inputs` (and referenced-section
//! properties) set to the record's own `normalized_dependencies`, so the
//! applied-weight validation still finds every record fresh and the applied
//! loads are bit for bit the fixture's. The formation check read a
//! metre-authored record correctly before and after the self-weight units
//! repair. While it re-formed the load from the record's millimetre values as
//! metres it was given a mass 10^6 times the one the load was formed from, and
//! the fixture's cases were demoted to sensitive.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{json, Value};

const FIXTURE: &str = include_str!("../../../fixtures/reference_system/reference_system.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const CASES: [&str; 3] = ["load:RS-SUS", "load:RS-OPE", "load:RS-SAM"];
const GENERATED_METHOD: &str = "pipe_mass_per_length_times_explicit_axis_acceleration/v2";
const LENGTHS: [&str; 5] = ["outside_diameter", "wall_thickness", "mill_tolerance", "corrosion_allowance", "insulation_thickness"];
const DENSITIES: [&str; 3] = ["material_density", "contents_density", "insulation_density"];

fn to_metres(q: &mut Value) {
    let from = unit_by_symbol(q["unit"].as_str().unwrap(), Dimension::Length).unwrap();
    let to = canonical_unit(Dimension::Length).unwrap();
    let value = convert_for_dimension(q["value"].as_f64().unwrap(), Dimension::Length, from, to).unwrap();
    *q = json!({"value": value, "unit": to.definition().symbol});
}

/// The fixture's metre twin (module comment).
fn metre_twin(fixture: &Value) -> Value {
    let mut model = fixture.clone();
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        for key in LENGTHS {
            if pipe["section"].get(key).is_some_and(Value::is_object) {
                to_metres(&mut pipe["section"][key]);
            }
        }
    }
    for section in model["sections"].as_array_mut().unwrap() {
        for key in ["outside_diameter", "wall_thickness"] {
            to_metres(&mut section["properties"][key]);
        }
    }
    let mut rewritten = 0;
    for case in model["load_cases"].as_array_mut().unwrap() {
        for load in case["primitive_loads"].as_array_mut().unwrap() {
            let Some(text) = load["provenance"].as_str() else { continue };
            let Ok(mut evidence) = serde_json::from_str::<Value>(text) else { continue };
            if evidence["method"] != GENERATED_METHOD {
                continue;
            }
            for key in LENGTHS.iter().chain(&DENSITIES) {
                if evidence["mass_inputs"].get(*key).is_some() {
                    evidence["mass_inputs"][*key] = evidence["normalized_dependencies"][*key].clone();
                }
            }
            if evidence["referenced_section"].is_object() {
                for key in ["outside_diameter", "wall_thickness"] {
                    evidence["referenced_section"]["properties"][key] = evidence["normalized_dependencies"][key].clone();
                }
            }
            assert_eq!(evidence["mass_inputs"]["outside_diameter"]["unit"], "m");
            load["provenance"] = json!(evidence.to_string());
            rewritten += 1;
        }
    }
    assert_eq!(rewritten, 66, "every generated record of the fixture");
    model
}

/// Each case's integrity code, after checking the envelope solved and that the
/// diagnostic and the numerical quality agree.
fn verdicts(model: &Value, mode: PreviewSolverMode, label: &str) -> Vec<String> {
    let envelope = serde_json::to_value(
        run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), mode).unwrap(),
    )
    .unwrap();
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label} {mode:?}");
    let diagnostics = envelope["diagnostics"].as_array().unwrap();
    assert!(!diagnostics.iter().any(|d| d["code"].as_str().is_some_and(|c| c.starts_with("SELF_WEIGHT"))),
        "{label} {mode:?}: every generated record is fresh");
    let quality = envelope["numerical_quality"]["cases"].as_array().unwrap();
    CASES.iter().map(|case| {
        let id = format!("diagnostic:numerical-integrity:{case}");
        let integrity: Vec<&Value> = diagnostics.iter().filter(|d| d["id"] == id).collect();
        assert_eq!(integrity.len(), 1, "{label} {mode:?} {case}");
        let code = integrity[0]["code"].as_str().unwrap().to_string();
        let entry = quality.iter().find(|c| c["basis_ref"]["ref_id"] == *case).unwrap();
        if code == "NUMERICAL_INTEGRITY_CHECKS_PASSED" {
            assert_eq!(entry["solve_quality"], "checks_passed", "{label} {mode:?} {case}");
            assert!(!integrity[0]["message"].as_str().unwrap().contains("formation_check: reason="), "{label} {mode:?} {case}");
        }
        code
    }).collect()
}

/// The metre twin publishes checks passed in every case; the fixture as
/// committed publishes the same verdicts.
#[test]
fn the_reference_systems_generated_weight_cases_publish_their_metre_twins_verdict() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let twin = metre_twin(&fixture);
    // The twin is judged in every mode before the fixture, so its verdicts stand on their own.
    let expected: Vec<Vec<String>> = MODES.iter().map(|&mode| verdicts(&twin, mode, "metre twin")).collect();
    for (mode, verdicts_of_twin) in MODES.iter().zip(&expected) {
        assert_eq!(verdicts_of_twin, &["NUMERICAL_INTEGRITY_CHECKS_PASSED"; 3], "metre twin {mode:?}");
    }
    for (&mode, verdicts_of_twin) in MODES.iter().zip(&expected) {
        assert_eq!(&verdicts(&fixture, mode, "fixture (mm)"), verdicts_of_twin, "fixture {mode:?}: its metre twin's verdicts");
    }
}

//! RT-L2 regression anchor (DEL-04-07):
//! "included" is today's mechanics exactly. Each committed pressure envelope
//! below was produced before the switch existed, when every pressurised case
//! included the closed-end free pressure strain. Its document is solved again
//! with `model_settings.pressure_elongation` naming every pressurised case,
//! and every published physical number (each result row's value bits, kind,
//! entity and unit, the summary's numbers and each case's solve quality) must
//! equal the committed envelope's. Only identity, setting, limitation and
//! evidence bytes may differ, and they are not compared here.
//!
//! Families: straight (X0, p < 0), load/reference-state (Y0), realized arc
//! (three L lines, one with a kink remainder, one with directional SIFs and a
//! corrosion allowance; the realized-bend report witness), expansion joint
//! with a tie rod (the connector report witness) and mechanics combinations
//! over pressurised cases. Tees and reducers have no committed envelope; their
//! frozen-reference tests run with the setting injected
//! (`rt_a3_tee_references.rs`, `rt_a3_reducer_references.rs`,
//! `w15c_eccentric_reducer.rs`).

#[path = "common/pressure_elongation.rs"]
mod pressure_elongation;
#[path = "common/current_model.rs"]
mod current_model;

use pressure_elongation::{assert_case_token, assert_physical_identity, mode, solve, with_elongation};
use serde_json::{json, Value};

fn read(path: &str) -> Value {
    let full = current_model::project_root().join(path);
    serde_json::from_str(&std::fs::read_to_string(&full).unwrap()).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn identity(document: &Value, solver_mode: &str, expected: &Value, label: &str) {
    let on = with_elongation(document);
    let actual = solve(&on, mode(solver_mode));
    let rows = assert_physical_identity(&actual, expected, label);
    assert!(rows > 0, "{label}");
    for case in pressure_elongation::pressurised_cases(document) {
        assert_case_token(&actual, &case, "included");
    }
}

#[test]
fn straight_and_load_reference_state_are_bit_identical_when_included() {
    let corpus = read("fixtures/results/pressure_v3_straight_reader_corpus.json");
    let mut x0 = read("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    let case = x0["model"]["load_cases"].as_array_mut().unwrap().iter_mut().find(|c| c["id"] == "case:closed-pressure").unwrap();
    case["pressure_regions"][0]["pressure"] = json!({"value": -2000, "unit": "kPa"});
    let y0 = read("fixtures/product_preview/pressure/load_reference/pressure.request.json");
    for (index, (document, label)) in [(&x0, "X0 p<0"), (&x0, "X0 p<0"), (&y0, "Y0"), (&y0, "Y0")].into_iter().enumerate() {
        let entry = &corpus["cases"][index];
        assert!(entry["label"].as_str().unwrap().starts_with(label), "{}", entry["label"]);
        identity(document, entry["solver_mode"].as_str().unwrap(), &entry["envelope"], entry["label"].as_str().unwrap());
    }
}

#[test]
fn realized_arcs_are_bit_identical_when_included() {
    let corpus = read("fixtures/results/pressure_v3_arc_reader_corpus.json");
    let mut checked = 0;
    for entry in corpus["cases"].as_array().unwrap() {
        let document = &entry["document"];
        if pressure_elongation::pressurised_cases(document).is_empty() {
            continue;
        }
        identity(document, entry["solver_mode"].as_str().unwrap(), &entry["envelope"], entry["label"].as_str().unwrap());
        checked += 1;
    }
    assert_eq!(checked, 4, "the four pressurised arc documents");
    let request = read("fixtures/reports/invented/exact_report_witness_inputs/pressure_2_realized_bend.request.json");
    let raw = read("fixtures/reports/invented/exact_report_witness_inputs/pressure_2_realized_bend-sparse_interactive.raw.json");
    identity(&request, "sparse_interactive", &raw, "realized-bend report witness");
}

#[test]
fn expansion_joint_with_tie_rod_is_bit_identical_when_included() {
    let request = read("fixtures/reports/invented/exact_report_witness_inputs/pressure_2_connector.request.json");
    assert!(request["model"]["components"].as_array().unwrap().iter().any(|c| c["kind"] == "expansion_joint"
        && c["objective_connector"]["hardware"]["kind"] == "explicit_axial_rods_v1"));
    let raw = read("fixtures/reports/invented/exact_report_witness_inputs/pressure_2_connector-sparse_interactive.raw.json");
    identity(&request, "sparse_interactive", &raw, "expansion-joint report witness");
}

#[test]
fn combinations_over_pressurised_cases_are_bit_identical_when_included() {
    let corpus = read("fixtures/results/pressure_v3_combination_reader_corpus.json");
    let mut checked = 0;
    for entry in corpus["cases"].as_array().unwrap() {
        let document = json!({"model": entry["model"], "materials": []});
        if pressure_elongation::pressurised_cases(&document).is_empty() {
            continue;
        }
        identity(&document, entry["solver_mode"].as_str().unwrap(), &entry["envelope"], entry["label"].as_str().unwrap());
        checked += 1;
    }
    assert!(checked >= 1);
}

//! The streamed producer against `derive_document` on every committed source that the
//! result export, report package and headless tests derive (the headless runner derives
//! only sources it solves at test time, of the identities below), and on the sources of
//! the thirteen report inputs of `report_package/tests/report_members_parity.rs`. Each
//! source is derived with an attested origin and with a received-current origin; the two
//! derivations must give the same refusal text, or the same canonical bytes, derivative
//! hash, header and row lists, and the walk must stage exactly what it shows.
//! Invented data only.

mod common;

use common::{compare, parse, Input, Outcome};
use serde_json::Value;

macro_rules! fixture {
    ($path:literal) => {
        ($path, include_str!(concat!("../../../../", $path)))
    };
}

/// Whole-file sources.
const RAW: &[(&str, &str)] = &[
    fixture!("fixtures/results/preview_physics_invented_sparse.json"),
    fixture!("fixtures/results/preview_physics_invented_dense.json"),
    fixture!("fixtures/results/preview_physics_connected_sparse.json"),
    fixture!("fixtures/results/preview_physics_connected_dense.json"),
    fixture!("fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json"),
    fixture!("fixtures/results/preview_physics_timoshenko_arc_sparse.json"),
    fixture!("fixtures/results/preview_physics_timoshenko_straight_sparse.json"),
    fixture!("fixtures/results/preview_physics_timoshenko_partial_span_blocked_sparse.json"),
    fixture!("fixtures/results/pressure_connected_mechanics_sparse.json"),
    fixture!("fixtures/results/pressure_connected_mechanics_dense.json"),
    fixture!("fixtures/results/pressure_thermal_ui_mechanics_sparse.json"),
    fixture!("fixtures/results/pressure_thermal_ui_mechanics_dense.json"),
    fixture!("fixtures/results/pressure_u4b_arc_sparse.json"),
    fixture!("fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json"),
    fixture!("fixtures/product_preview/pressure/load_reference/connected-dense_scrutiny.raw.json"),
    fixture!("fixtures/product_preview/pressure/load_reference/pressure-sparse_interactive.raw.json"),
    fixture!("fixtures/product_preview/pressure/load_reference/pressure-dense_scrutiny.raw.json"),
    fixture!("fixtures/reports/invented/report_standing_witnesses/pressure_3_coupled_friction_sensitive-sparse_interactive.raw.json"),
    fixture!("fixtures/reports/invented/report_standing_witnesses/pressure_3_gap_unresolved-sparse_interactive.raw.json"),
];

const PRESSURE_SOURCE: &[(&str, &str)] = &[
    fixture!("fixtures/product_preview/pressure_source/load_reference_source/n05-sparse_interactive.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/load_reference_source/n05-dense_scrutiny.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/load_reference_source/n06-sparse_interactive.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/load_reference_source/n06-dense_scrutiny.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/load_reference_source/mixed-sparse_interactive.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/load_reference_source/mixed-dense_scrutiny.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/load_reference_source/fields-sparse_interactive.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/load_reference_source/fields-dense_scrutiny.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/load_reference_source/eigen_motion-sparse_interactive.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/load_reference_source/eigen_motion-dense_scrutiny.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/physics_source/n06-sparse_interactive.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/physics_source/n06-dense_scrutiny.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/physics_source/mixed-dense_scrutiny.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/physics_source/fields-sparse_interactive.raw.json"),
    fixture!("fixtures/product_preview/pressure_source/physics_source/fields-dense_scrutiny.raw.json"),
];

/// Successor statements: `source`, with the request's model.
const SUCCESSORS: &[(&str, &str)] = &[
    fixture!("fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"),
    fixture!("fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json"),
    fixture!("fixtures/results/retained_precision_exact_successor_sparse_interactive.json"),
    fixture!("fixtures/results/retained_precision_exact_successor_dense_scrutiny.json"),
    fixture!("fixtures/results/retained_precision_combination_successor_sparse_interactive.json"),
    fixture!("fixtures/results/retained_precision_combination_successor_dense_scrutiny.json"),
    fixture!("fixtures/results/retained_precision_l0_successor_sparse_interactive.json"),
    fixture!("fixtures/results/retained_precision_l0_successor_dense_scrutiny.json"),
    fixture!("fixtures/results/retained_precision_w_c2_successor_sparse_interactive.json"),
    fixture!("fixtures/results/retained_precision_w_c2_successor_dense_scrutiny.json"),
];

/// The report inputs of `report_members_parity.rs` whose fixture carries its source: the
/// five exact witness requests (`request.source_results[0]`) and the two exact user-release
/// fixtures (`source`); the four pressure-free vectors' two sources and the two standing
/// witnesses' raw results are in `RAW`.
const WITNESS_REQUESTS: &[(&str, &str)] = &[
    fixture!("fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json"),
    fixture!("fixtures/reports/invented/exact_report_package_requests/pressure_2_load_reference_states.json"),
    fixture!("fixtures/reports/invented/exact_report_package_requests/pressure_2_connector.json"),
    fixture!("fixtures/reports/invented/exact_report_package_requests/pressure_2_realized_bend.json"),
    fixture!("fixtures/reports/invented/exact_report_package_requests/pressure_source_1.json"),
];
const USER_RELEASE_EXACT: &[(&str, &str)] = &[
    fixture!("fixtures/reports/invented/user_release_exact/pressure_2_straight.json"),
    fixture!("fixtures/reports/invented/user_release_exact/pressure_source_1.json"),
];

const COMBINATION_CORPUS: (&str, &str) = fixture!("fixtures/results/pressure_v3_combination_reader_corpus.json");
const DIFFERENCE_CORPUS: (&str, &str) = fixture!("fixtures/results/preview_physics_difference_basis_corpus.json");

/// Derives `source` with both origins; returns how many derived.
fn both(name: &str, source: Value, model: Option<Value>) -> usize {
    let mut derived = 0;
    for input in [Input::attested(format!("{name} (attested)"), source.clone(), model.clone()), Input::received(format!("{name} (received)"), source, model)] {
        if let Outcome::Derived(_) = compare(&input) {
            derived += 1;
        }
    }
    derived
}

fn raw(list: &[(&str, &str)]) -> usize {
    list.iter().map(|(name, text)| both(name, parse(text), None)).sum()
}

#[test]
fn preview_physics_and_pressure_sources() {
    // The blocked partial-span Timoshenko statement is refused by both.
    assert_eq!(raw(RAW), 2 * (RAW.len() - 1));
}

#[test]
fn pressure_source_sources() {
    assert_eq!(raw(PRESSURE_SOURCE), 2 * PRESSURE_SOURCE.len());
}

#[test]
fn successor_sources() {
    let derived: usize = SUCCESSORS
        .iter()
        .map(|(name, text)| {
            let fixture = parse(text);
            both(name, fixture["source"].clone(), Some(fixture["invocation"]["request"]["model"].clone()))
        })
        .sum();
    assert_eq!(derived, 2 * SUCCESSORS.len());
}

#[test]
fn report_input_sources() {
    let mut derived = 0;
    for (name, text) in WITNESS_REQUESTS {
        let fixture = parse(text);
        derived += both(name, fixture["request"]["source_results"][0].clone(), Some(fixture["model"].clone()));
    }
    for (name, text) in USER_RELEASE_EXACT {
        let fixture = parse(text);
        derived += both(name, fixture["source"].clone(), Some(fixture["model"].clone()));
    }
    assert_eq!(derived, 2 * (WITNESS_REQUESTS.len() + USER_RELEASE_EXACT.len()));
}

#[test]
fn combination_corpus_sources() {
    let (name, text) = COMBINATION_CORPUS;
    let corpus = parse(text);
    let cases = corpus["cases"].as_array().unwrap();
    let derived: usize = cases
        .iter()
        .enumerate()
        .map(|(index, case)| both(&format!("{name} case {index}"), case["envelope"].clone(), Some(case["model"].clone())))
        .sum();
    assert_eq!(derived, 2 * cases.len());
}

#[test]
fn difference_basis_corpus_sources() {
    let (name, text) = DIFFERENCE_CORPUS;
    let corpus = parse(text);
    let mut derived = 0;
    for document in ["v1_closing", "v5_sustained_only_load"] {
        let entry = corpus["documents"].as_array().unwrap().iter().find(|e| e["name"] == document).unwrap();
        derived += both(&format!("{name} {document}"), entry["document"].clone(), None);
    }
    assert_eq!(derived, 4);
}

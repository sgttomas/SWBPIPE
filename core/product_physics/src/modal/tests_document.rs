//! I3a tests (DEL-04-09 Design §5.3): the modal result document, projected
//! from real `ModalOutcome`s of the reference models with **TEST-LOCAL,
//! EXPERIMENTAL** parameters, and the maintained fixtures it generates.
//!
//! **The fixtures** (`fixtures/results/modal_*.json`, each with its request
//! `modal_*.request.json`) are maintained test fixtures, not run evidence:
//! `fixtures_are_the_projection_of_their_requests` re-projects every one and
//! requires its committed bytes; the ignored `write_modal_fixtures` is the
//! deterministic generation script (it writes the same bytes; see
//! `fixtures/results/modal_README.md`). The semantic table
//! (`semantic_contract_modal_1.json`) is the producer's statement of it; the
//! Rust reader (`core/reporting/modal_result`) and the Python reader are each
//! held to the same committed bytes and read every committed fixture, so the
//! producer and both readers meet only at committed bytes. All models are
//! invented.
use super::document::{modal_doc_sha256, modal_result_document, modal_semantic_table, MODAL_RESULT_KIND};
use super::integrity::ModalIntegrityParameters;
use super::tests::{arc_request, ex_request_with_density, pf_request, q, with};
use super::tests_eigen::{sec_a_cantilever, test_local_parameters};
use super::tests_state::kd5;
use serde_json::{json, Value};

const U1: &str = include_str!("../../../../validation/references/kd5_nonlinear/u1_request.json");
const F1: &str = include_str!("../../../../validation/references/kd5_nonlinear/f1_request.json");
const SEMANTIC_TABLE: &str = include_str!("../../../../fixtures/results/semantic_contract_modal_1.json");

/// The label every fixture's parameter set carries.
const TEST_LOCAL_LABEL: &str = "TEST_LOCAL_I3A_EXPERIMENTAL";
/// The Sensitive fixture's set: the test-local set with τ_r = 10⁻³⁰, so that
/// every reported mode's η exceeds it. **TEST-LOCAL, EXPERIMENTAL.**
const TEST_LOCAL_SENSITIVE_LABEL: &str = "TEST_LOCAL_I3A_EXPERIMENTAL_SENSITIVE";

/// One maintained fixture: its name, its request and its parameters.
struct Fixture {
    name: &'static str,
    request: Value,
    params: ModalIntegrityParameters,
    label: &'static str,
    /// The blocking codes when the run refuses.
    refused: Option<&'static [&'static str]>,
}

fn state_case(text: &str, case: &str) -> Value {
    with(kd5(text), json!({"mode_count": 3, "state_case": case}))
}

/// The fixture set, in a fixed order.
fn fixtures() -> Vec<Fixture> {
    let local = test_local_parameters;
    let mut sensitive = test_local_parameters();
    sensitive.residual_threshold = 1e-30;
    let f = |name, request, refused| Fixture { name, request, params: local(), label: TEST_LOCAL_LABEL, refused };
    vec![
        f("ex_n05", with(ex_request_with_density(), json!({"mode_count": 3})), None),
        f("arc", with(arc_request(false), json!({"mode_count": 6})), None),
        f("component_body", with(pf_request(true), json!({"mode_count": 3, "weight_to_mass_acceleration": q(9.8125, "m/s^2")})), None),
        f("state_one_way_pressed", state_case(U1, "load:PRESSED"), None),
        f("state_friction_sliding", state_case(F1, "load:SLIDE-POS"), None),
        f("cantilever_whole_spectrum", with(sec_a_cantilever(2), json!({"mode_count": 1000})), None),
        f("cantilever_cluster_extension", with(sec_a_cantilever(2), json!({"mode_count": 1})), None),
        f("cantilever_below_cutoff", with(sec_a_cantilever(2), json!({"frequency_cutoff": q(0.5, "Hz")})), None),
        Fixture {
            name: "cantilever_sensitive",
            request: with(sec_a_cantilever(2), json!({"mode_count": 2})),
            params: sensitive,
            label: TEST_LOCAL_SENSITIVE_LABEL,
            refused: None,
        },
        f("refused_unseparated", with(ex_request_with_density(), json!({"mode_count": 4})), Some(&["MODAL_STURM_SHIFT_UNSEPARATED"])),
        f("refused_no_bound", pf_request(false), Some(&["MODAL_EXTRACTION_BOUND_REQUIRED"])),
    ]
}

/// The committed text of a document or request: pretty JSON and a newline.
fn fixture_text(value: &Value) -> String {
    let mut text = serde_json::to_string_pretty(value).expect("a document serializes");
    text.push('\n');
    text
}

fn committed(name: &str) -> (&'static str, &'static str) {
    macro_rules! pair {
        ($n:literal) => {
            (
                include_str!(concat!("../../../../fixtures/results/modal_", $n, ".json")),
                include_str!(concat!("../../../../fixtures/results/modal_", $n, ".request.json")),
            )
        };
    }
    match name {
        "ex_n05" => pair!("ex_n05"),
        "arc" => pair!("arc"),
        "component_body" => pair!("component_body"),
        "state_one_way_pressed" => pair!("state_one_way_pressed"),
        "state_friction_sliding" => pair!("state_friction_sliding"),
        "cantilever_whole_spectrum" => pair!("cantilever_whole_spectrum"),
        "cantilever_cluster_extension" => pair!("cantilever_cluster_extension"),
        "cantilever_below_cutoff" => pair!("cantilever_below_cutoff"),
        "cantilever_sensitive" => pair!("cantilever_sensitive"),
        "refused_unseparated" => pair!("refused_unseparated"),
        "refused_no_bound" => pair!("refused_no_bound"),
        _ => unreachable!("every fixture has its committed pair"),
    }
}

#[test]
fn the_semantic_table_is_the_committed_bytes() {
    let committed: Value = serde_json::from_str(SEMANTIC_TABLE).unwrap();
    assert_eq!(modal_semantic_table(), committed, "the producer's modal-1 is the committed table");
    assert_eq!(fixture_text(&modal_semantic_table()), SEMANTIC_TABLE);
    // Every document names the committed table's digest.
    let document = modal_result_document(&with(pf_request(false), json!({"mode_count": 1})), &test_local_parameters(), TEST_LOCAL_LABEL);
    assert_eq!(document["semantic_contract"]["sha256"].as_str(), modal_doc_sha256(&committed).as_deref());
}

#[test]
fn fixtures_are_the_projection_of_their_requests() {
    for fixture in fixtures() {
        let document = modal_result_document(&fixture.request, &fixture.params, fixture.label);
        let (text, request_text) = committed(fixture.name);
        assert_eq!(fixture_text(&fixture.request), request_text, "{}: the committed request", fixture.name);
        assert!(fixture_text(&document) == text, "{}: the committed document is the projection of its request (regenerate with write_modal_fixtures)", fixture.name);
        assert_eq!(document["document_kind"], MODAL_RESULT_KIND);
        // Deterministic: a second run gives the same bytes.
        assert_eq!(modal_result_document(&fixture.request, &fixture.params, fixture.label), document, "{}", fixture.name);
    }
}

#[test]
fn the_fixtures_cover_both_cases_every_endpoint_states_and_refusals() {
    let docs: Vec<(&str, Value)> = fixtures()
        .into_iter()
        .map(|f| (f.name, modal_result_document(&f.request, &f.params, f.label)))
        .collect();
    let cases: Vec<&str> = docs
        .iter()
        .filter(|(_, d)| d["status"] == "solved")
        .flat_map(|(_, d)| d["tables"]["modes"]["rows"].as_array().unwrap().iter().map(|m| m["normalization_case"].as_str().unwrap()))
        .collect();
    assert!(cases.contains(&"unit_max_translation") && cases.contains(&"unit_max_rotation"), "both normalization cases");
    let endpoint = |name: &str| docs.iter().find(|(n, _)| *n == name).unwrap().1["extraction"]["endpoint"]["kind"].clone();
    assert_eq!(endpoint("cantilever_whole_spectrum"), "whole_finite_spectrum");
    assert_eq!(endpoint("cantilever_below_cutoff"), "below_lowest_mode");
    assert_eq!(endpoint("ex_n05"), "below_computed_cluster");
    let extension = &docs.iter().find(|(n, _)| *n == "cantilever_cluster_extension").unwrap().1["extraction"];
    assert_eq!((extension["bounded_count"].as_u64(), extension["extension_count"].as_u64()), (Some(1), Some(1)));
    for (name, treatment) in [("state_one_way_pressed", "restrained"), ("state_friction_sliding", "free_frozen_friction")] {
        let doc = &docs.iter().find(|(n, _)| *n == name).unwrap().1;
        assert_eq!(doc["basis"]["state_case"]["linearized"], true, "{name}");
        assert!(doc["tables"]["supports"]["rows"].as_array().unwrap().iter().any(|s| s["treatment"] == treatment), "{name}");
    }
    let ex = &docs.iter().find(|(n, _)| *n == "ex_n05").unwrap().1;
    assert_eq!(ex["basis"]["route"], "exact_pressure");
    let body = &docs.iter().find(|(n, _)| *n == "component_body").unwrap().1;
    assert_eq!(body["basis"]["weight_to_mass_acceleration"]["value"], 9.8125);
}

/// The deterministic generation script of the committed fixtures: writes
/// each request and its document, and the semantic table, under
/// `fixtures/results/` (run explicitly; `--ignored`).
#[test]
#[ignore = "writes the committed modal fixtures; run explicitly to regenerate them"]
fn write_modal_fixtures() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/results");
    std::fs::write(root.join("semantic_contract_modal_1.json"), fixture_text(&modal_semantic_table())).unwrap();
    for fixture in fixtures() {
        let document = modal_result_document(&fixture.request, &fixture.params, fixture.label);
        std::fs::write(root.join(format!("modal_{}.request.json", fixture.name)), fixture_text(&fixture.request)).unwrap();
        std::fs::write(root.join(format!("modal_{}.json", fixture.name)), fixture_text(&document)).unwrap();
    }
}

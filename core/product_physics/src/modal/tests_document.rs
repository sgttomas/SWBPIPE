//! I3a tests: the modal result document, projected
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
use super::document::{modal_doc_sha256, modal_doc_solved, modal_result_document, modal_semantic_table, MODAL_RESULT_KIND};
use super::outcome::run_modal_dense;
use super::request::ModalPreviewRequest;
use open_pipe_stress_modal_eigen::modal_quantities::{
    cumulative_effective_mass_fractions, normalize_mode, BelowNormalRange, Dof, FreeDof, ModalBasis, NormalizedMode,
};
use open_pipe_stress_modal_eigen::SymmetricMatrix;
use super::integrity::ModalIntegrityParameters;
use super::tests::{arc_request, ex_request_with_density, pf_request, q, with};
use super::tests_eigen::{sec_a_cantilever, test_local_parameters};
use super::tests_state::kd5;
use serde_json::{json, Value};

const U1: &str = include_str!("../../../../validation/references/kd5_nonlinear/u1_request.json");
const F1: &str = include_str!("../../../../validation/references/kd5_nonlinear/f1_request.json");
const SEMANTIC_TABLE: &str = include_str!("../../../../fixtures/results/semantic_contract_modal_1.json");
const PRODUCER_EDGES: &str = include_str!("../../../../fixtures/results/modal_producer_edges.json");

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
        // The review of 9b22b5a2b: at f_c = 1e-160 Hz, (2π·f_c)² is below the normal range, so the
        // published lower end fl(√fl(ω_c²)) is not within a relative allowance of 2π·f_c.
        f("cantilever_subnormal_cutoff", with(sec_a_cantilever(2), json!({"frequency_cutoff": q(1e-160, "Hz")})), None),
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
    match name {
        "ex_n05" => (include_str!("../../../../fixtures/results/modal_ex_n05.json"), include_str!("../../../../fixtures/results/modal_ex_n05.request.json")),
        "arc" => (include_str!("../../../../fixtures/results/modal_arc.json"), include_str!("../../../../fixtures/results/modal_arc.request.json")),
        "component_body" => (include_str!("../../../../fixtures/results/modal_component_body.json"), include_str!("../../../../fixtures/results/modal_component_body.request.json")),
        "state_one_way_pressed" => (include_str!("../../../../fixtures/results/modal_state_one_way_pressed.json"), include_str!("../../../../fixtures/results/modal_state_one_way_pressed.request.json")),
        "state_friction_sliding" => (include_str!("../../../../fixtures/results/modal_state_friction_sliding.json"), include_str!("../../../../fixtures/results/modal_state_friction_sliding.request.json")),
        "cantilever_whole_spectrum" => (include_str!("../../../../fixtures/results/modal_cantilever_whole_spectrum.json"), include_str!("../../../../fixtures/results/modal_cantilever_whole_spectrum.request.json")),
        "cantilever_cluster_extension" => (include_str!("../../../../fixtures/results/modal_cantilever_cluster_extension.json"), include_str!("../../../../fixtures/results/modal_cantilever_cluster_extension.request.json")),
        "cantilever_below_cutoff" => (include_str!("../../../../fixtures/results/modal_cantilever_below_cutoff.json"), include_str!("../../../../fixtures/results/modal_cantilever_below_cutoff.request.json")),
        "cantilever_subnormal_cutoff" => (include_str!("../../../../fixtures/results/modal_cantilever_subnormal_cutoff.json"), include_str!("../../../../fixtures/results/modal_cantilever_subnormal_cutoff.request.json")),
        "cantilever_sensitive" => (include_str!("../../../../fixtures/results/modal_cantilever_sensitive.json"), include_str!("../../../../fixtures/results/modal_cantilever_sensitive.request.json")),
        "refused_unseparated" => (include_str!("../../../../fixtures/results/modal_refused_unseparated.json"), include_str!("../../../../fixtures/results/modal_refused_unseparated.request.json")),
        "refused_no_bound" => (include_str!("../../../../fixtures/results/modal_refused_no_bound.json"), include_str!("../../../../fixtures/results/modal_refused_no_bound.request.json")),
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
    assert_eq!(endpoint("cantilever_subnormal_cutoff"), "below_lowest_mode");
    // Its gap's lower end is the producer's root of a subnormal square, 8.1e-166 from 2π·f_c.
    let tiny = &docs.iter().find(|(n, _)| *n == "cantilever_subnormal_cutoff").unwrap().1["sturm"]["placement"];
    let omega_c = 2.0 * std::f64::consts::PI * 1e-160;
    assert!((omega_c * omega_c).is_subnormal());
    assert_eq!(tiny["lower"]["value"].as_f64(), Some((omega_c * omega_c).sqrt()));
    assert!(((omega_c * omega_c).sqrt() - omega_c).abs() > 8.0 * (f64::EPSILON / 2.0) * omega_c);
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

#[test]
fn a_member_named_model_is_not_the_aggregate_subject() {
    // The aggregate resultant's subject is `model`; a straight member may carry
    // the same id. Each identity row states what its subject names.
    let mut request = with(pf_request(false), json!({"mode_count": 1}));
    request["model"]["pipe_segments"][0]["id"] = json!("model");
    let document = modal_result_document(&request, &test_local_parameters(), TEST_LOCAL_LABEL);
    assert_eq!(document["status"], "solved", "{:#}", document.get("refusals").unwrap_or(&Value::Null));
    let rows = document["tables"]["identity"]["rows"].as_array().unwrap();
    for axis in ["x", "y", "z"] {
        let named: Vec<(&str, &str)> = rows
            .iter()
            .filter(|r| r["axis"] == axis && r["subject"] == "model")
            .map(|r| (r["subject_kind"].as_str().unwrap(), r["comparison"].as_str().unwrap()))
            .collect();
        assert_eq!(named, vec![("member", "nodal"), ("model", "resultant")], "{axis}");
    }
    assert!(rows.iter().all(|r| ["member", "component_body", "model"].contains(&r["subject_kind"].as_str().unwrap())));
}

/// Producer edges: reported modes
/// whose participation quantities are formed by the producer's own
/// `normalize_mode` and `cumulative_effective_mass_fractions` on edge bases,
/// and projected by the producer's own `modal_doc_solved`. Each is a set of
/// three modes sharing one T_k. They are false-refusal controls: both readers
/// and the feasibility oracle must accept every one.
fn modal_edge_modes() -> Vec<(&'static str, &'static str, [f64; 3], Vec<NormalizedMode>)> {
    const TAU: f64 = 1e-6;
    let p2 = |e: i32| 2f64.powi(e);
    let ux = |node| FreeDof { node, dof: Dof::Ux };
    let one = |m: f64, p: f64, totals: [f64; 3]| {
        let mass = SymmetricMatrix::from_diagonal(&[m]).unwrap();
        let (p, zero, dofs) = ([p], [0.0], [ux(1)]);
        let basis = ModalBasis::new(&mass, &dofs, [&p, &zero, &zero], totals, 1.0, TAU).unwrap();
        normalize_mode(0, &[1.0], &basis).unwrap()
    };
    let mut out = Vec::new();
    // modal_quantities.rs `a_contribution_dropped_by_alignment_is_not_exposed_by_cancellation`:
    // m = 2, Γ_x = 2⁻¹⁰⁰², m_eff,x = 2⁻²⁰⁰³ and its fraction below the normal
    // range (flagged, any extended exponent).
    let totals = [2.0 + p2(-300), 1.0, 1.0];
    let mass = SymmetricMatrix::from_diagonal(&[1.0, p2(-300), 1.0]).unwrap();
    let (p, z) = ([1.0, p2(-300), 1.0], [0.0; 3]);
    let dofs = [ux(0), ux(1), ux(2)];
    let basis = ModalBasis::new(&mass, &dofs, [&p, &z, &z], totals, 1.0, TAU).unwrap();
    let mode = normalize_mode(0, &[1.0, p2(-701), -1.0], &basis).unwrap();
    assert!(mode.below_normal_range.effective_mass[0] && mode.below_normal_range.effective_mass_fraction[0]);
    out.push(("extended_exponent_effective_mass", "m_eff,x = 2⁻²⁰⁰³ and its fraction flagged; Γ_x = 2⁻¹⁰⁰² normal", totals, vec![mode.clone(), mode.clone(), mode]));
    // A flagged fraction inside the cumulative sums: mode 1's m_eff,x and
    // fraction are about 10⁻³²⁰ (flagged); the producer sums the published
    // fractions, so mode 1's cumulative is an unflagged 0.
    let totals = [6.0, 1.0, 1.0];
    let first = one(1.0, 1e-160, totals);
    assert!(first.below_normal_range.effective_mass_fraction[0]);
    out.push(("flagged_fraction_inside_the_cumulative_sums", "mode 1's fraction flagged; cumulative over the published fractions", totals,
        vec![first, one(2.0, 0.3, totals), one(2.0, 0.6, totals)]));
    // The reviewer's probe: Γ_x 10⁻²⁰⁰, T_x 10⁻³⁰⁰, m 33.7…: m_eff,x ≈ 3.4·10⁻³⁹⁹
    // is flagged, its fraction ≈ 3.4·10⁻⁹⁹ is normal.
    let totals = [1e-300, 1.0, 1.0];
    let m = 33.718650870752306;
    let probe = one(m, m * 1e-200, totals);
    assert!(probe.below_normal_range.effective_mass[0] && !probe.below_normal_range.effective_mass_fraction[0]);
    out.push(("normal_fraction_of_a_flagged_effective_mass", "Γ_x 10⁻²⁰⁰, T_x 10⁻³⁰⁰: m_eff,x flagged, its fraction normal", totals,
        vec![probe.clone(), probe.clone(), probe]));
    out
}

/// The producer-edge corpus entries: each edge's published values, as the
/// producer's projection publishes them, onto the committed component-body
/// fixture, with the payload withheld (it did not form these values).
fn modal_edge_entries() -> Value {
    let base = fixtures().into_iter().find(|f| f.name == "component_body").unwrap();
    let parsed = ModalPreviewRequest::parse_modal_request(base.request.clone()).unwrap();
    let committed_doc: Value = serde_json::from_str(committed("component_body").0).unwrap();
    let mut entries = Vec::new();
    for (id, about, totals, modes) in modal_edge_modes() {
        let mut outcome = run_modal_dense(&parsed, &base.params).unwrap();
        assert_eq!(outcome.modes.len(), modes.len());
        let cumulative = cumulative_effective_mass_fractions(&modes);
        for ((mode, edge), sums) in outcome.modes.iter_mut().zip(&modes).zip(&cumulative) {
            mode.generalized_mass = edge.generalized_mass;
            mode.participation = edge.participation;
            mode.effective_mass = edge.effective_mass;
            mode.effective_mass_fraction = edge.effective_mass_fraction;
            mode.cumulative_effective_mass_fraction = *sums;
            let shape = std::mem::take(&mut mode.below_normal_range.shape);
            mode.below_normal_range = BelowNormalRange { shape, ..edge.below_normal_range.clone() };
        }
        outcome.mass_summary.total_by_direction = totals;
        let doc = modal_doc_solved(&outcome, &committed_doc["binding"], base.label);
        let mut operations = vec![json!({"op": "withhold", "table": "mass_matrix"}), json!({"op": "withhold", "table": "shapes"})];
        let mut take = |path: String| {
            let value = doc.pointer(&path).unwrap_or_else(|| panic!("{path}")).clone();
            operations.push(json!({"op": "replace", "path": path, "value": value}));
        };
        for k in 0..3 {
            take(format!("/tables/mass_totals/rows/{k}/total_mass"));
        }
        for m in 0..modes.len() {
            take(format!("/tables/modes/rows/{m}/generalized_mass"));
            for k in 0..3 {
                for field in ["participation_factor", "effective_mass", "effective_mass_fraction", "cumulative_effective_mass_fraction"] {
                    take(format!("/tables/participation/rows/{}/{field}", 3 * m + k));
                }
            }
        }
        entries.push(json!({"id": format!("producer:{id}"), "about": about, "document": "modal_component_body.json",
            "request": "modal_component_body.request.json", "request_operations": [], "operations": operations, "reseal": true}));
    }
    json!({"generator": "core/product_physics/src/modal/tests_document.rs modal_edge_entries", "entries": entries,
        "relation_probes": [modal_generalized_mass_probe()]})
}

/// A relation-level producer probe (the payload of a real model cannot carry
/// it): the reviewer's input driven through the producer's own
/// `normalize_mode`, whose m is `mass_bilinear`'s exact sum
/// (modal_quantities.rs:294). A unit rotation and a 2⁻⁶⁰⁰ translation with
/// mass only on the translation (M_ii = 1) give m = 2⁻¹²⁰⁰ > 0, published as
/// a flagged 0. Each reader's m reconstruction, and the oracle, accept it.
fn modal_generalized_mass_probe() -> Value {
    let p2 = |e: i32| 2f64.powi(e);
    // Global DOFs 11 (node 1, rz) and 6 (node 1, ux), in that free order.
    let dofs = [FreeDof { node: 1, dof: Dof::Rz }, FreeDof { node: 1, dof: Dof::Ux }];
    let mass = SymmetricMatrix::from_diagonal(&[0.0, 1.0]).unwrap();
    let (p, z) = ([0.0, 1.0], [0.0, 0.0]);
    let basis = ModalBasis::new(&mass, &dofs, [&p, &z, &z], [1.0, 1.0, 1.0], 1.0, 1e-6).unwrap();
    let mode = normalize_mode(0, &[1.0, p2(-600)], &basis).unwrap();
    assert!(mode.generalized_mass == 0.0 && mode.below_normal_range.generalized_mass);
    let flagged = |i: usize| mode.below_normal_range.shape.contains(&i);
    json!({"id": "generalized_mass_of_an_underflowing_triple_product",
        "formation": "modal_quantities.rs:294 mass_bilinear, through normalize_mode",
        "global_dofs": [11, 6],
        "shape": [{"value": mode.shape[0], "below_normal": flagged(0)}, {"value": mode.shape[1], "below_normal": flagged(1)}],
        "mass_entries": [[6, 6, 1.0]],
        "generalized_mass": {"value": mode.generalized_mass, "below_normal": mode.below_normal_range.generalized_mass}})
}

#[test]
fn the_producer_edges_are_the_producers_own_formation() {
    assert!(fixture_text(&modal_edge_entries()) == PRODUCER_EDGES, "regenerate with write_modal_producer_edges");
}

#[test]
#[ignore = "writes the producer-edge corpus; run explicitly to regenerate it"]
fn write_modal_producer_edges() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/results");
    std::fs::write(root.join("modal_producer_edges.json"), fixture_text(&modal_edge_entries())).unwrap();
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

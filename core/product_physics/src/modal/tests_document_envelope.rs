//! Tests of the modal document envelope (`document_envelope`, DEL-04-09
//! REQ-007, landing A1). Every depth cap and parameter set here is
//! **TEST-LOCAL**; no cap or limit is selected.
//!
//! The oracle is the built document: `observe` walks the `Value` the
//! existing writer returns, measuring each length with serde_json and the
//! canonical crate themselves (not the module's escaping or digit code), and
//! every count of the tally must equal it. The fixtures are used only as that
//! oracle's inputs; nothing is sized from them.
//!
//! **Allocation.** No existing mechanism observes this module's allocation:
//! the completed-set seam's failpoints count only its own fallible requests
//! (`resource.rs`, `completed_set_seam`), and the crate's counting global
//! allocator exists only in release test builds, with private counters
//! (`sparse_loop_memory_tests.rs`). So the no-allocation claim rests on
//! reading the module, guarded here by a scan of its source for allocating
//! constructs, and no observed construction or hashing peak is compared with
//! the derived peaks. What can be observed without an allocator is: the
//! capacities of every string and array in the built tree, held to H's
//! string and array terms.
use super::document::{modal_doc_solved, modal_result_document, modal_semantic_table, ModalParameterStanding, MODAL_SELECTED_PARAMETER_LABEL};
use super::document_envelope::*;
use super::integrity::ModalIntegrityParameters;
use super::outcome::{run_modal_dense, ModalOutcome, ModalStanding};
use super::request::ModalPreviewRequest;
use super::tests::{modal, pf_request, q, section, with, PROV};
use super::tests_eigen::test_local_parameters;
use crate::Diagnostic;
use open_pipe_stress_canonical_json::{canonical_json, canonical_json_checked_v1_text, ecma_number_to_string};
use open_pipe_stress_modal_eigen::clusters::ExtractionBound;
use open_pipe_stress_modal_eigen::sturm::{FactoredShift, FormationOperation, Placement, RangeViolation, ShiftDecision, ShiftEvaluation};
use open_pipe_stress_sparse_direct::SparseDirectError;
use serde_json::{json, Value};

/// TEST-LOCAL depth cap for every walk here.
const CAP: u64 = 200;

// ---------------------------------------------------------------------------
// The oracle: an independent walk of a built value.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct ModalEnvelopeObserved {
    wire: u64,
    float_bytes: u64,
    floats: u64,
    nodes: u64,
    objects: u64,
    entries: u64,
    key_bytes: u64,
    arrays: u64,
    elements: u64,
    strings: u64,
    string_bytes: u64,
    depth: u64,
    longest_literal: u64,
    unsafe_numbers: u64,
    pretty: u64,
    canonical: u64,
    canonical_floats: u64,
    string_capacity: u64,
    array_capacity: u64,
}

fn modal_env_text_len<T: serde::Serialize + ?Sized>(value: &T) -> u64 {
    serde_json::to_string(value).unwrap().len() as u64
}

fn modal_env_observe(value: &Value) -> ModalEnvelopeObserved {
    fn modal_env_walk(v: &Value, o: &mut ModalEnvelopeObserved) -> u64 {
        o.nodes += 1;
        match v {
            Value::Null | Value::Bool(_) => {
                o.longest_literal = o.longest_literal.max(modal_env_text_len(v));
                0
            }
            Value::Number(n) => {
                let len = modal_env_text_len(v);
                o.longest_literal = o.longest_literal.max(len);
                if n.is_f64() {
                    o.floats += 1;
                    o.float_bytes += len;
                    o.canonical_floats += ecma_number_to_string(n.as_f64().unwrap()).len() as u64;
                }
                // The checked profile itself decides what is unsafe.
                if canonical_json_checked_v1_text(&serde_json::to_string(v).unwrap()).is_err() {
                    o.unsafe_numbers += 1;
                }
                0
            }
            Value::String(s) => {
                o.strings += 1;
                o.string_bytes += s.len() as u64;
                o.string_capacity += s.capacity() as u64;
                o.longest_literal = o.longest_literal.max(modal_env_text_len(v));
                0
            }
            Value::Array(items) => {
                o.arrays += 1;
                o.elements += items.len() as u64;
                o.array_capacity += items.capacity() as u64;
                1 + items.iter().map(|item| modal_env_walk(item, o)).max().unwrap_or(0)
            }
            Value::Object(map) => {
                o.objects += 1;
                o.entries += map.len() as u64;
                for key in map.keys() {
                    o.key_bytes += key.len() as u64;
                    o.string_capacity += key.capacity() as u64;
                    o.longest_literal = o.longest_literal.max(modal_env_text_len(key));
                }
                1 + map.values().map(|item| modal_env_walk(item, o)).max().unwrap_or(0)
            }
        }
    }
    let mut o = ModalEnvelopeObserved::default();
    o.depth = modal_env_walk(value, &mut o);
    o.wire = modal_env_text_len(value);
    o.pretty = serde_json::to_string_pretty(value).unwrap().len() as u64;
    o.canonical = canonical_json(value).len() as u64;
    o
}

/// Every count of `t` equals the oracle's.
fn modal_env_assert_tally(name: &str, t: &ModalDocumentTally, value: &Value) -> ModalEnvelopeObserved {
    let o = modal_env_observe(value);
    let counts = |t: &ModalDocumentTally| {
        [
            t.wire,
            t.float_bytes,
            t.floats,
            t.nodes,
            t.objects,
            t.entries,
            t.key_bytes,
            t.arrays,
            t.elements,
            t.strings,
            t.string_bytes,
            t.depth,
            t.longest_literal,
            t.unsafe_numbers,
        ]
    };
    let observed = [
        o.wire,
        o.float_bytes,
        o.floats,
        o.nodes,
        o.objects,
        o.entries,
        o.key_bytes,
        o.arrays,
        o.elements,
        o.strings,
        o.string_bytes,
        o.depth,
        o.longest_literal,
        o.unsafe_numbers,
    ];
    assert_eq!(
        counts(t),
        observed,
        "{name}: [wire, float_bytes, floats, nodes, objects, entries, key_bytes, arrays, elements, strings, string_bytes, depth, longest_literal, unsafe]"
    );
    assert_eq!(modal_env_pretty(t).unwrap(), o.pretty, "{name}: pretty text");
    // Exact apart from floats; floats within their 25-byte maximum.
    assert_eq!(t.wire - t.float_bytes + o.canonical_floats, o.canonical, "{name}: canonical text less floats");
    assert!(o.canonical <= modal_env_canonical_bound(t).unwrap(), "{name}: canonical bound");
    o
}

/// The tally, and observations of the built tree's capacities against H.
fn modal_env_assert_envelope(name: &str, envelope: &ModalDocumentEnvelope, doc: &Value) {
    let t = &envelope.tally;
    let o = modal_env_assert_tally(name, t, doc);
    // OBSERVATIONS, not the proof: every string's and array's capacity in
    // the built document is within H's string and array terms.
    assert!(o.string_capacity <= 2 * (t.string_bytes + t.key_bytes) + 8 * (t.strings + t.entries), "{name}: string capacity");
    assert!(o.array_capacity <= 2 * t.elements + 4 * t.arrays, "{name}: array capacity");
    assert_eq!(envelope.heap_bytes, modal_env_heap(t).unwrap());
    assert!(envelope.construction_peak_bytes >= envelope.heap_bytes + envelope.hash_peak_bytes.min(1), "{name}");
    assert_eq!(envelope.pretty_bytes, o.pretty);
}

// ---------------------------------------------------------------------------
// The maintained writer documents.

struct ModalEnvelopeCase {
    name: &'static str,
    request: Value,
    params: ModalIntegrityParameters,
    label: &'static str,
    text: &'static str,
}

macro_rules! fixture {
    ($name:literal) => {
        (
            $name,
            include_str!(concat!("../../../../fixtures/results/modal_", $name, ".request.json")),
            include_str!(concat!("../../../../fixtures/results/modal_", $name, ".json")),
        )
    };
}

/// The fourteen maintained writer documents (`tests_document.rs`
/// `fixtures`), from their committed requests.
fn modal_env_cases() -> Vec<ModalEnvelopeCase> {
    let all = [
        fixture!("ex_n05"),
        fixture!("arc"),
        fixture!("component_body"),
        fixture!("state_one_way_pressed"),
        fixture!("state_friction_sliding"),
        fixture!("cantilever_whole_spectrum"),
        fixture!("cantilever_cluster_extension"),
        fixture!("cantilever_below_cutoff"),
        fixture!("cantilever_subnormal_cutoff"),
        fixture!("cantilever_sensitive"),
        fixture!("refused_unseparated"),
        fixture!("refused_no_bound"),
        fixture!("selected_cantilever"),
        fixture!("selected_refused_no_bound"),
    ];
    all.into_iter()
        .map(|(name, request, text)| {
            let (params, label) = match name {
                "cantilever_sensitive" => {
                    let mut p = test_local_parameters();
                    p.residual_threshold = 1e-30;
                    (p, "TEST_LOCAL_I3A_EXPERIMENTAL_SENSITIVE")
                }
                "selected_cantilever" | "selected_refused_no_bound" => {
                    (ModalIntegrityParameters::selected_a1_a2_a3_criteria(), MODAL_SELECTED_PARAMETER_LABEL)
                }
                _ => (test_local_parameters(), "TEST_LOCAL_I3A_EXPERIMENTAL"),
            };
            ModalEnvelopeCase { name, request: serde_json::from_str(request).unwrap(), params, label, text }
        })
        .collect()
}

/// The diagnostic the builder makes on the parse-refusal branch
/// (`document.rs:511`), restated here (test code only).
fn modal_env_parse_diagnostic(refusal: String) -> Diagnostic {
    crate::diag("diagnostic:modal:request:parse", "MODAL_REQUEST_INVALID", "blocking", refusal, vec!["request".into()])
}

/// The diagnostic the builder makes on the skew-refusal branch
/// (`document.rs:527-533`), restated here (test code only).
fn modal_env_skew_diagnostic() -> Diagnostic {
    crate::diag(
        "diagnostic:modal:document:skew-restraint",
        super::integrity::MODAL_DOCUMENT_SKEW_RESTRAINT_UNSUPPORTED,
        "blocking",
        "the model has a skewed restraint, so its modal pencil is on reduced coordinates; the modal-1 0.1.0 document states a global M_ff and global shapes only, so no document is published for it".to_string(),
        vec!["supports".into()],
    )
}

/// The envelope of `modal_result_document(request, params, label)`, by the
/// same branches (`document.rs:505-540`).
fn modal_env_mirror(request: &Value, params: &ModalIntegrityParameters, label: &str, cap: u64) -> ModalEnvelopeResult<ModalDocumentEnvelope> {
    let standing = ModalParameterStanding::modal_doc_standing_of(params, label);
    match ModalPreviewRequest::parse_modal_request(request.clone()) {
        Err(refusal) => modal_env_parse_refused(request, params, standing, &modal_env_parse_diagnostic(refusal), cap),
        Ok(parsed) => match run_modal_dense(&parsed, params) {
            Err(refusals) => {
                let state = parsed.state_case.as_deref().and_then(|id| parsed.model.load_cases.iter().position(|c| c.id == id).map(|i| (i, id)));
                modal_env_run_refused(request, params, standing, state, &refusals, cap)
            }
            Ok(outcome) => {
                if parsed.model.supports.iter().any(crate::document_members::is_skewed_record) {
                    let state = outcome.state_case.as_ref().map(|case| (case.case_index, case.case_id.as_str()));
                    modal_env_skew_refused(request, params, standing, state, &modal_env_skew_diagnostic(), cap)
                } else {
                    modal_env_solved(&outcome, request, params, standing, cap)
                }
            }
        },
    }
}

fn modal_env_fixture_text(value: &Value) -> String {
    let mut text = serde_json::to_string_pretty(value).unwrap();
    text.push('\n');
    text
}

#[test]
fn modal_env_the_tally_is_every_maintained_writer_document() {
    let mut branches = std::collections::BTreeSet::new();
    for case in modal_env_cases() {
        let doc = modal_result_document(&case.request, &case.params, case.label);
        assert!(modal_env_fixture_text(&doc) == case.text, "{}: the writer's bytes are the committed document", case.name);
        let envelope = modal_env_mirror(&case.request, &case.params, case.label, CAP).unwrap_or_else(|e| panic!("{}: {e:?}", case.name));
        modal_env_assert_envelope(case.name, &envelope, &doc);
        // The committed text is the pretty text and its newline.
        assert_eq!(envelope.pretty_bytes + 1, case.text.len() as u64, "{}", case.name);
        branches.insert(doc["status"].as_str().unwrap().to_owned());
    }
    assert_eq!(branches.into_iter().collect::<Vec<_>>(), ["refused", "solved"]);
}

#[test]
fn modal_env_the_semantic_table_and_its_restated_unit_rules_are_the_documents() {
    let table = modal_semantic_table();
    modal_env_assert_tally("semantic table", &modal_env_semantic_table().unwrap(), &table);
    let kinds = table["kinds"].as_array().unwrap();
    assert_eq!(kinds.len(), MODAL_ENV_KINDS.len());
    for (built, (kind, name, selector, rules)) in kinds.iter().zip(MODAL_ENV_KINDS) {
        assert_eq!((built["kind"].as_str(), built["table"].as_str(), built["selector"].as_str()), (Some(*kind), Some(*name), Some(*selector)));
        let units = built["units"].as_object().unwrap();
        assert_eq!(units.len(), rules.len(), "{kind}");
        for (key, id) in *rules {
            assert_eq!(units[*key]["unit"].as_str(), Some(id.catalog_id()), "{kind} {key}");
        }
    }
}

// ---------------------------------------------------------------------------
// Synthetic outcomes through `modal_doc_solved` (the producer-edge path).

fn modal_env_base(name: &str) -> (ModalEnvelopeCase, ModalOutcome, Value) {
    let case = modal_env_cases().into_iter().find(|c| c.name == name).unwrap();
    let outcome = run_modal_dense(&modal(case.request.clone()), &case.params).unwrap();
    let binding = modal_result_document(&case.request, &case.params, case.label)["binding"].clone();
    (case, outcome, binding)
}

fn modal_env_check_solved(name: &str, outcome: &ModalOutcome, binding: &Value, standing: ModalParameterStanding<'_>) {
    let doc = modal_doc_solved(outcome, binding, standing);
    let envelope = modal_env_solved_with_binding(outcome, binding, standing, CAP).unwrap_or_else(|e| panic!("{name}: {e:?}"));
    modal_env_assert_envelope(name, &envelope, &doc);
}

const AWKWARD: &str = "q\"\\\u{0}\u{1}\u{8}\t\n\u{b}\u{c}\r\u{1f} \u{7f}é€😀";

fn modal_env_diagnostic(text: &str, refs: usize) -> Diagnostic {
    crate::diag(
        &format!("diagnostic:{text}"),
        "MODAL_TEST_LOCAL",
        "warning",
        format!("message {text}"),
        (0..refs).map(|i| format!("ref:{i}:{text}")).collect(),
    )
}

#[test]
fn modal_env_branches_the_fixtures_do_not_reach_are_exact() {
    let (case, outcome, binding) = modal_env_base("component_body");
    let standing = ModalParameterStanding::modal_doc_standing_of(&case.params, case.label);
    let mut variants: Vec<(&str, ModalOutcome)> = Vec::new();
    let mut add = |name, edit: &dyn Fn(&mut ModalOutcome)| {
        let mut o = outcome.clone();
        edit(&mut o);
        variants.push((name, o));
    };
    add("non-finite values", &|o| {
        o.modes[0].frequency = f64::NAN;
        o.modes[0].circular_frequency = f64::INFINITY;
        o.node_coordinates[0][0] = f64::NEG_INFINITY;
        o.integrity.computed[0].residual.condition = f64::NAN;
    });
    add("outside the canonical profile", &|o| {
        o.modes[0].period = 1e300;
        o.modes[0].generalized_mass = -9_007_199_254_740_992.0;
        o.bounding_box_diagonal = 2e16;
    });
    add("null checksums when hashing refuses", &|o| {
        o.integrity.computed[0].index = 1 << 60;
        o.clusters[0].id = 1 << 53;
        o.reported.bounded_count = usize::MAX;
    });
    add("an unresolved census", &|o| {
        o.integrity.census = super::eigen::FiniteCensus::Unresolved {
            structurally_massless: 3,
            nonpositive_mu: 12_345,
            rejected_mu: 0,
            mass_block_established: false,
        }
    });
    add("a given placement", &|o| o.integrity.sturm.evidence.placement = Placement::Given);
    add("a gap with an unformed end", &|o| o.integrity.sturm.evidence.placement = Placement::Gap { lower: -1.0, upper: f64::NAN });
    add("above the spectrum", &|o| {
        o.integrity.sturm.evidence.placement = Placement::AboveSpectrum { top_eigenvalue: 1e10, established_finite_mode_count: 1_000_000 }
    });
    add("an orthogonality pair", &|o| {
        o.integrity.orthogonality.pair = Some((0, 2));
        o.integrity.orthogonality.max_ratio_below_normal_range = true;
    });
    add("no orthogonality pair", &|o| o.integrity.orthogonality.pair = None);
    add("Sensitive reasons", &|o| {
        o.standing = ModalStanding::Sensitive(vec![format!("mode 1: {AWKWARD}"), "M-orthogonality deviation 1e-3 of modes Some((1, 2))".into()]);
        o.modes[1].standing = ModalStanding::Sensitive(vec![AWKWARD.repeat(40)]);
    });
    add("cut-off crossing disclosures", &|o| {
        o.disclosures.push("computed mode 4 (not reported) has a first-order eigenvalue error estimate e_λ = 1e-3 that reaches across the cut-off".into());
        o.identity.disclosures.push(AWKWARD.into());
    });
    add("awkward identifiers", &|o| {
        o.node_ids[0] = AWKWARD.into();
        o.node_ids[1] = "ü".repeat(300);
        for (_, id) in o.subjects.iter_mut() {
            id.push_str(AWKWARD);
        }
        for m in o.discretization.iter_mut() {
            m.pipe.push_str(AWKWARD);
        }
        for d in o.identity.discrepancies.iter_mut() {
            d.subject.push_str(AWKWARD);
        }
    });
    add("an empty reported set", &|o| {
        o.modes.clear();
        o.reported.count = 0;
    });
    add("free DOFs out of order and repeated", &|o| {
        o.free_dofs.reverse();
        let first = o.free_dofs[0];
        o.free_dofs.push(first);
    });
    add("no section, an unnamed row, no centre, no acceleration", &|o| {
        o.integrity.formation.section = None;
        o.integrity.formation.stiffness.row = usize::MAX;
        o.mass_summary.center_of_gravity = None;
        o.weight_to_mass_acceleration = None;
    });
    add("both bounds", &|o| o.bound = ExtractionBound::Both { mode_count: 7, frequency_cutoff: 12.5 });
    add("a cut-off bound", &|o| o.bound = ExtractionBound::FrequencyCutoff(3.0));
    add("wide guard figures", &|o| {
        o.guard.static_estimate_bytes = u128::MAX;
        o.guard.operation_estimate = 0;
    });
    add("undefined fractions", &|o| {
        o.modes[0].effective_mass_fraction = [None, Some(0.5), None];
        o.modes[0].cumulative_effective_mass_fraction = [None, None, Some(1.0)];
        o.modes[0].below_normal_range.effective_mass_fraction = [false, true, false];
        o.modes[0].below_normal_range.shape = vec![0, 6, 7];
    });
    add("diagnostics with awkward text", &|o| {
        o.diagnostics.push(modal_env_diagnostic(AWKWARD, 0));
        o.diagnostics.push(modal_env_diagnostic("plain", 5));
    });
    add("every candidate decision", &|o| {
        let factored = FactoredShift {
            negative_pivot_count: 1 << 54,
            factor_term: 1e-12,
            backward_error: 2e-12,
            inverse_norm_estimate: f64::INFINITY,
            inverse_norm_solves: 3,
            conditioning_product: 1.5,
            separation_ratio: None,
            growth: 1.0,
            max_abs_lower_factor: 0.5,
            max_abs_pivot: 1e300,
            max_abs_scaled_diagonal: 1.0,
            separated: false,
            within_growth_limit: true,
            conditioned: false,
        };
        let candidate = |decision, formation_term, factored| ShiftEvaluation {
            sigma: 4.0,
            decision,
            formation_term,
            formation_separation_ratio: formation_term.map(|t: f64| t * 2.0),
            factored,
        };
        let candidates = &mut o.integrity.sturm.evidence.candidates;
        candidates.push(candidate(ShiftDecision::OutsideGap, None, None));
        candidates.push(candidate(
            ShiftDecision::OutOfRange(RangeViolation { row: 1, col: 2, operation: FormationOperation::Difference, value: 1e-320 }),
            Some(1e-9),
            None,
        ));
        candidates.push(candidate(ShiftDecision::SeparationOutOfRange { neighbour: 0, value: f64::NAN }, Some(1e-9), None));
        candidates.push(candidate(ShiftDecision::FactorizationRejected(SparseDirectError::InvalidVectorLength { expected: 1, actual: 2 }), Some(1e-9), None));
        candidates.push(candidate(ShiftDecision::Unseparated, Some(1e-9), Some(factored.clone())));
        candidates.push(candidate(ShiftDecision::GrowthLimit, Some(1e-9), Some(FactoredShift { separation_ratio: Some(3.0), ..factored.clone() })));
        candidates.push(candidate(ShiftDecision::IllConditioned, Some(1e-9), Some(factored)));
        candidates[0].sigma = -1.0;
    });
    for (name, o) in &variants {
        modal_env_check_solved(name, o, &binding, standing);
    }
    // A test-local label with every escaping class.
    modal_env_check_solved("an awkward label", &outcome, &binding, ModalParameterStanding::TestLocal(AWKWARD));
    modal_env_check_solved("the selected standing", &outcome, &binding, ModalParameterStanding::Selected);
}

#[test]
fn modal_env_the_other_solved_fixtures_with_synthetic_edits_are_exact() {
    // A linearized state case: supports with awkward ids and a static diagnostic.
    let (case, outcome, binding) = modal_env_base("state_friction_sliding");
    let standing = ModalParameterStanding::modal_doc_standing_of(&case.params, case.label);
    let mut o = outcome.clone();
    let state = o.nonlinear_state.as_mut().unwrap();
    for s in state.supports.iter_mut() {
        s.support_id.push_str(AWKWARD);
    }
    state.static_diagnostics.push(modal_env_diagnostic(AWKWARD, 2));
    modal_env_check_solved("state case with awkward supports", &o, &binding, standing);
    let mut o = outcome;
    o.state_case = None;
    o.nonlinear_state = None;
    modal_env_check_solved("state case removed", &o, &binding, standing);
    // The cut-off fixture: an empty set below the lowest mode.
    let (case, outcome, binding) = modal_env_base("cantilever_below_cutoff");
    modal_env_check_solved("below the cut-off", &outcome, &binding, ModalParameterStanding::modal_doc_standing_of(&case.params, case.label));
}

// ---------------------------------------------------------------------------
// Skew: the refused branch, and its solved projection (bypassing the refusal).

/// An invented skew model (as `tests_skew.rs` `request`): A–M–B along +X,
/// anchored at A, rotations held at B, and an oblique stop at B.
fn modal_env_skew_request() -> Value {
    let node = |id: &str, x: f64| json!({"id": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV});
    let pipe = |id: &str, from: &str, to: &str| {
        json!({"id": id, "from": from, "to": to, "section": section(), "material": "m", "y_reference": {"x": 0, "y": 0, "z": 1}, "provenance": PROV})
    };
    let model = json!({"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:skew-modal", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [node("A", 0.0), node("M", 2.0), node("B", 4.0)],
        "pipe_segments": [pipe("pipe:AM", "A", "M"), pipe("pipe:MB", "M", "B")],
        "materials": [{"id": "m", "elastic_modulus": q(200e9, "Pa"), "shear_modulus": q(80e9, "Pa"), "provenance": PROV}],
        "supports": [
            {"id": "anchor_A", "node": "A", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
            {"id": "rot_B", "node": "B", "family": "anchor", "restraints": ["RX", "RY", "RZ"], "provenance": PROV},
            {"id": "stop_B", "node": "B", "family": "line_stop", "restraints": ["UY"], "direction": {"x": 3.0, "y": 4.0, "z": 0.0}, "provenance": PROV},
        ],
        "components": [],
        "load_cases": [{"id": "case:f", "primitive_loads": [{"id": "load:f", "category": "concentrated_force", "target": {"type": "node", "node": "B"},
            "direction": "global_z", "magnitude": q(1000.0, "N"), "dimension": "force", "provenance": PROV}], "provenance": PROV}],
        "combinations": []});
    with(json!({"model": model, "materials": []}), json!({"mode_count": 8}))
}

#[test]
fn modal_env_a_skew_model_is_exact_refused_and_as_a_solved_projection() {
    let params = test_local_parameters();
    let request = modal_env_skew_request();
    let doc = modal_result_document(&request, &params, "test-local");
    assert_eq!(doc["refusals"]["rows"][0]["code"], super::integrity::MODAL_DOCUMENT_SKEW_RESTRAINT_UNSUPPORTED);
    modal_env_assert_envelope("skew refused", &modal_env_mirror(&request, &params, "test-local", CAP).unwrap(), &doc);
    let outcome = run_modal_dense(&modal(request), &params).unwrap();
    modal_env_check_solved("skew solved", &outcome, &doc["binding"], ModalParameterStanding::TestLocal("test-local"));
}

// ---------------------------------------------------------------------------
// Adversarial requests on the parse-refusal branch.

fn modal_env_nested(depth: usize) -> Value {
    let mut v = json!(1);
    for i in 0..depth {
        v = if i % 2 == 0 { json!([v]) } else { json!({"k": v}) };
    }
    v
}

fn modal_env_assert_branch(name: &str, request: &Value, params: &ModalIntegrityParameters, label: &str) -> ModalDocumentEnvelope {
    let doc = modal_result_document(request, params, label);
    let envelope = modal_env_mirror(request, params, label, CAP).unwrap_or_else(|e| panic!("{name}: {e:?}"));
    modal_env_assert_envelope(name, &envelope, &doc);
    envelope
}

#[test]
fn modal_env_adversarial_requests_are_exact_on_the_parse_refusal_branch() {
    let params = test_local_parameters();
    let mut wide = with(pf_request(false), json!({"mode_count": modal_env_nested(60), "frequency_cutoff": (0..5000).map(|i| json!([i, -i, 0.1 * i as f64])).collect::<Vec<_>>()}));
    wide[AWKWARD] = json!(AWKWARD);
    wide["ü".repeat(500)] = json!({"x": [1e-7, -0.0, 5e-324, 1.7976931348623157e308]});
    for name in ["state_case", "modulus_basis_ref", "modulus_basis_temperature", "weight_to_mass_acceleration", "include_dynamic_only_restraints"] {
        wide[name] = json!({ "echo": name });
    }
    let doc = modal_result_document(&wide, &params, "test-local");
    assert_eq!(doc["refusals"]["rows"][0]["code"], "MODAL_REQUEST_INVALID", "{:#}", doc["refusals"]);
    modal_env_assert_branch("deep and wide echo", &wide, &params, "test-local");
    // Each escaping class in an undeclared name alone.
    for (i, class) in ["\"", "\\", "\u{8}", "\u{9}", "\u{a}", "\u{c}", "\u{d}", "\u{0}", "\u{1f}", "\u{7f}", "é", "€", "😀"].iter().enumerate() {
        let request = with(pf_request(false), json!({ format!("member{class}{i}"): class }));
        modal_env_assert_branch(&format!("escape class {i}"), &request, &params, class);
    }
    // Requests that are not objects, a request whose hash refuses (an unsafe
    // integer; a container 130 deep), and parameters that are not finite or
    // not safe integers (the parse refusal precedes their validation).
    for (name, request) in [
        ("a string request", json!(AWKWARD)),
        ("an array request", json!([1, [2, [3]]])),
        ("an unsafe integer", with(pf_request(false), json!({"mode_count": 9_007_199_254_740_993u64, "zz": -9_007_199_254_740_993i64}))),
        ("a 130-deep echo", with(pf_request(false), json!({"mode_count": modal_env_nested(130)}))),
        ("an integral float echo", with(pf_request(false), json!({"mode_count": 1e300, "zz": 2.0}))),
    ] {
        let envelope = modal_env_assert_branch(name, &request, &params, "test-local");
        assert!(envelope.construction_peak_bytes > envelope.heap_bytes);
    }
    let mut odd = test_local_parameters();
    odd.residual_threshold = f64::NAN;
    odd.sturm_growth_limit = 1e300;
    odd.sturm_max_candidates = 1 << 60;
    odd.sturm_gap_log_positions.push(f64::INFINITY);
    let request = with(pf_request(false), json!({"zz": 1}));
    let doc = modal_result_document(&request, &odd, "test-local");
    assert!(doc["binding"]["integrity_parameters_sha256"].is_null());
    modal_env_assert_branch("parameters outside the profile", &request, &odd, "test-local");
}

#[test]
fn modal_env_the_depth_cap_is_reached_exactly_and_exceeded() {
    let params = test_local_parameters();
    let request = with(pf_request(false), json!({"mode_count": modal_env_nested(40), "zz": true}));
    let depth = modal_env_observe(&request).depth;
    assert!(depth > 40);
    let standing = ModalParameterStanding::TestLocal("test-local");
    let refusal = modal_env_parse_diagnostic(ModalPreviewRequest::parse_modal_request(request.clone()).err().unwrap());
    let at = modal_env_parse_refused(&request, &params, standing, &refusal, depth).unwrap();
    assert_eq!(at, modal_env_mirror(&request, &params, "test-local", CAP).unwrap(), "the cap at the depth walks it all");
    assert_eq!(modal_env_parse_refused(&request, &params, standing, &refusal, depth - 1), Err(ModalEnvelopeError::DepthCap { cap: depth - 1 }));
    assert_eq!(modal_env_value(&request, depth).unwrap().depth, depth);
    assert_eq!(modal_env_value(&request, depth - 1), Err(ModalEnvelopeError::DepthCap { cap: depth - 1 }));
    assert_eq!(modal_env_value(&json!(1), 0).unwrap().depth, 0);
    assert_eq!(modal_env_value(&json!([]), 0), Err(ModalEnvelopeError::DepthCap { cap: 0 }));
}

// ---------------------------------------------------------------------------
// The fallback refused document, from the request alone.

fn modal_env_text_of(s: &str) -> ModalEnvelopeText {
    super::document_envelope::modal_env_text(s).unwrap()
}

#[test]
fn modal_env_the_fallback_document_is_exact_for_its_diagnostic_and_monotone_in_its_bound() {
    // `refused_no_bound` is refused by one run diagnostic, the shape of A2's
    // fallback: with every text bound taken exactly from that diagnostic,
    // the counts are its document's, and the bytes exceed it by exactly the
    // slack of bounding each affected ref by the longest.
    let case = modal_env_cases().into_iter().find(|c| c.name == "refused_no_bound").unwrap();
    let doc = modal_result_document(&case.request, &case.params, case.label);
    let parsed = modal(case.request.clone());
    let refusals = run_modal_dense(&parsed, &case.params).err().unwrap();
    assert_eq!(refusals.len(), 1);
    let d = &refusals[0];
    assert!(!d.affected_refs.is_empty());
    let longest = d.affected_refs.iter().map(|r| modal_env_text_of(r)).max_by_key(|t| (t.escaped, t.raw)).unwrap();
    assert!(d.affected_refs.iter().all(|r| modal_env_text_of(r).raw <= longest.raw), "the longest escaped ref is also the longest raw one here");
    let slack = |f: fn(ModalEnvelopeText) -> u64| d.affected_refs.iter().map(|r| f(longest) - f(modal_env_text_of(r))).sum::<u64>();
    let (escaped_slack, raw_slack) = (slack(|t| t.escaped), slack(|t| t.raw));
    let exact = ModalEnvelopeDiagnosticBound {
        id: modal_env_text_of(&d.id),
        code: modal_env_text_of(&d.code),
        severity: modal_env_text_of(&d.severity),
        message: modal_env_text_of(&d.message),
        affected_refs: d.affected_refs.len() as u64,
        affected_ref: longest,
    };
    let standing = ModalParameterStanding::modal_doc_standing_of(&case.params, case.label);
    let fallback = modal_env_fallback_refused(&case.request, &case.params, standing, None, &exact, CAP).unwrap();
    let mut adjusted = fallback.tally;
    adjusted.wire -= escaped_slack;
    adjusted.string_bytes -= raw_slack;
    let o = modal_env_observe(&doc);
    assert_eq!(adjusted.longest_literal, fallback.tally.longest_literal);
    adjusted.longest_literal = o.longest_literal;
    assert!(fallback.tally.longest_literal >= o.longest_literal);
    modal_env_assert_tally("fallback, exact texts", &adjusted, &doc);
    let mut looser = exact;
    looser.message = modal_env_text_join(exact.message, modal_env_text_of(&AWKWARD.repeat(10))).unwrap();
    looser.affected_refs += 3;
    looser.affected_ref = modal_env_text_join(longest, modal_env_text_of(AWKWARD)).unwrap();
    let wider = modal_env_fallback_refused(&case.request, &case.params, standing, None, &looser, CAP).unwrap();
    let (a, b) = (fallback.tally, wider.tally);
    assert!(b.wire > a.wire && b.nodes > a.nodes && b.strings > a.strings && b.string_bytes > a.string_bytes && b.depth == a.depth);
    assert!(wider.construction_peak_bytes > fallback.construction_peak_bytes);
}

// ---------------------------------------------------------------------------
// Checked arithmetic.

#[test]
fn modal_env_every_composition_refuses_overflow_by_name() {
    let big = ModalDocumentTally { wire: u64::MAX - 1, nodes: 1, ..Default::default() };
    let mut object = ModalEnvelopeObject::default();
    assert!(matches!(object.modal_env_member("k", big), Err(ModalEnvelopeError::Overflow { term: "container wire", .. })));
    let mut array = ModalEnvelopeArray::default();
    array.modal_env_element(ModalDocumentTally { wire: u64::MAX - 1, ..Default::default() }).unwrap();
    assert!(matches!(array.modal_env_array(), Err(ModalEnvelopeError::Overflow { term: "container wire", .. })));
    let many = ModalDocumentTally { entries: u64::MAX, ..Default::default() };
    assert!(matches!(modal_env_heap(&many), Err(ModalEnvelopeError::Overflow { .. })));
    assert!(matches!(modal_env_hash_peak(&ModalDocumentTally { wire: u64::MAX / 2, ..Default::default() }), Err(ModalEnvelopeError::Overflow { term: "hash text", .. })));
    assert!(matches!(modal_env_pretty(&ModalDocumentTally { wire: u64::MAX, nonempty_containers: 1, ..Default::default() }), Err(ModalEnvelopeError::Overflow { .. })));
    assert!(matches!(modal_env_canonical_bound(&ModalDocumentTally { floats: u64::MAX / 2, ..Default::default() }), Err(ModalEnvelopeError::Overflow { term: "canonical text", .. })));
    assert!(matches!(modal_env_sum(&big, &big), Err(ModalEnvelopeError::Overflow { term: "forest wire", left, right }) if left == u64::MAX - 1 && right == u64::MAX - 1));
    let long = ModalEnvelopeText { raw: u64::MAX, escaped: 1 };
    assert!(matches!(modal_env_text_join(long, long), Err(ModalEnvelopeError::Overflow { term: "joined text", .. })));
    assert!(matches!(modal_env_string(ModalEnvelopeText { raw: 1, escaped: u64::MAX - 1 }), Err(ModalEnvelopeError::Overflow { term: "string literal", .. })));
    let deep = ModalDocumentTally { depth: u64::MAX, ..Default::default() };
    let mut object = ModalEnvelopeObject::default();
    object.modal_env_member("k", deep).unwrap();
    assert!(matches!(object.modal_env_object(), Err(ModalEnvelopeError::Overflow { term: "container depth", .. })));
    let levels = ModalDocumentTally { slot_levels: u64::MAX, ..Default::default() };
    let mut array = ModalEnvelopeArray::default();
    assert!(matches!(array.modal_env_element(levels), Err(ModalEnvelopeError::Overflow { term: "slot levels", .. })));
}

// ---------------------------------------------------------------------------
// Leaves and premises.

#[test]
fn modal_env_escaping_is_serde_jsons_for_every_byte_class() {
    for c in (0u32..0x80).chain([0xe9, 0x20ac, 0x1f600, 0x10ffff]) {
        let s = char::from_u32(c).unwrap().to_string();
        assert_eq!(modal_env_text_of(&s).escaped + 2, modal_env_text_len(&s), "U+{c:04X}");
    }
    assert_eq!(modal_env_text_of(AWKWARD).escaped + 2, modal_env_text_len(AWKWARD));
    assert_eq!(modal_env_digest(&ModalDocumentTally::default()).wire, 66);
}

#[test]
fn modal_env_float_texts_are_within_their_maxima_observed() {
    // OBSERVATIONS of the derivation's maxima (zmij's 24-byte buffer, the
    // canonical branches' 25), on each branch's extremes and a sweep.
    let mut values = vec![0.0, -0.0, 5e-324, -2.2250738585072014e-308, -1.7976931348623157e308, -0.0000012345678901234567, -1.2345678901234567e-5, 1e21, 1e-7, 123456789012345680.0];
    let mut state = 0x9e37_79b9_7f4a_7c15u64;
    for _ in 0..200_000 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        values.push(f64::from_bits(state));
    }
    let (mut wire, mut canonical) = (0, 0);
    for v in values.into_iter().filter(|v| v.is_finite()) {
        let t = modal_env_value(&json!(v), 1).unwrap();
        assert_eq!(t.wire, modal_env_text_len(&v));
        wire = wire.max(t.wire);
        canonical = canonical.max(ecma_number_to_string(v).len() as u64);
    }
    assert!(wire <= MODAL_ENV_WIRE_FLOAT_MAX && canonical <= MODAL_ENV_CANONICAL_FLOAT_MAX, "{wire} {canonical}");
    assert_eq!(canonical, MODAL_ENV_CANONICAL_FLOAT_MAX, "the 0.00000ddd branch reaches 25");
}

#[test]
fn modal_env_the_library_premises_hold_on_this_toolchain() {
    // No `preserve_order`: a Map iterates its names in sorted order.
    let mut map = serde_json::Map::new();
    map.insert("b".into(), json!(1));
    map.insert("a".into(), json!(2));
    assert_eq!(map.keys().collect::<Vec<_>>(), ["a", "b"]);
    // P2: a pushed Vec<Value> of n has capacity ≤ max(4, 2n).
    let mut values = Vec::new();
    for n in 1..=5000u64 {
        values.push(Value::Null);
        assert!(values.capacity() as u64 <= (2 * n).max(4), "{n}");
    }
    // P3: format! capacity ≤ max(2L, 8) for this module's id formats.
    let long = "ü".repeat(200);
    for i in [0usize, 9, 10, 99_999, usize::MAX] {
        for s in [
            format!("mode:{i}"),
            format!("computed:{i}"),
            format!("cluster:{i}"),
            format!("dof:{long}:{}", "rz"),
            format!("shape:{}:dof:{long}:ux", format!("mode:{i}")),
            format!("mass:dof:{long}:ux|dof:{long}:rz"),
            format!("refusal:{i}"),
            format!("modal-run:{}", &"0123456789abcdef0123"[..16]),
            format!("{:02x}", i as u8),
        ] {
            assert!(s.capacity() <= (2 * s.len()).max(8), "{s}");
        }
    }
    // P4: HashSet<String> capacity after k names ≤ max(3, 2k).
    let mut set = std::collections::HashSet::new();
    for k in 1..=3000usize {
        set.insert(k.to_string());
        assert!(set.capacity() <= (2 * k).max(3), "{k}");
    }
}

#[test]
fn modal_env_the_tally_allocates_nothing_by_its_source() {
    // A guard on the module's source, not an observation: no allocating
    // construct appears in its code (comments excluded).
    let source = include_str!("document_envelope.rs");
    let forbidden = [
        "Vec<", "Vec::", "vec!", "String::", ".to_string(", ".to_owned(", ".clone()", ".cloned()", ".collect", "format!(", "json!", "Box<", "Box::",
        "Value::from", "to_vec", "with_capacity", "push_str", ".push(", "HashMap", "HashSet", "BTreeMap", "Rc<", "Arc<", "serde_json::to_", "Map::",
        "write!(", "writeln!(", "String {", "-> String",
    ];
    for (i, line) in source.lines().enumerate() {
        let code = line.split("//").next().unwrap();
        for token in forbidden {
            assert!(!code.contains(token), "document_envelope.rs:{}: `{token}` in `{}`", i + 1, line.trim());
        }
    }
}

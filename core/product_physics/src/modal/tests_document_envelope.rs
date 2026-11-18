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
//! **Allocation.** The no-allocation claim rests on reading the module,
//! guarded in every build by a scan of its source for allocating constructs.
//! Every build also holds the capacities of each string and array in the
//! built tree to H's string and array terms. In release test builds only,
//! where the crate's counting global allocator is registered
//! (`sparse_loop_memory_tests.rs`, whose readers this module uses and does
//! not change), `modal_env_observed` OBSERVES the peaks of requested bytes
//! while the writer builds and hashes documents, against C and HP, and that
//! the tally requests none: observations that corroborate the bounds, not
//! their proof (an optimizer may also remove allocations it can prove
//! unused).
use super::document::{
    modal_doc_sha256, modal_doc_solved, modal_result_document, modal_semantic_table, ModalParameterStanding, MODAL_SELECTED_PARAMETER_LABEL,
};
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

/// Synthetic edits of `outcome` that reach the branches no fixture does.
fn modal_env_variants(outcome: &ModalOutcome) -> Vec<(&'static str, ModalOutcome)> {
    let mut variants: Vec<(&'static str, ModalOutcome)> = Vec::new();
    let mut add = |name: &'static str, edit: &dyn Fn(&mut ModalOutcome)| {
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
    variants
}

#[test]
fn modal_env_branches_the_fixtures_do_not_reach_are_exact() {
    let (case, outcome, binding) = modal_env_base("component_body");
    let standing = ModalParameterStanding::modal_doc_standing_of(&case.params, case.label);
    for (name, o) in &modal_env_variants(&outcome) {
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
    // Every support treatment, each in its own row.
    let mut o = outcome.clone();
    let state = o.nonlinear_state.as_mut().unwrap();
    let first = state.supports[0].clone();
    for (i, treatment) in [
        super::state::ModalSupportTreatment::Restrained,
        super::state::ModalSupportTreatment::AlreadyRestrained,
        super::state::ModalSupportTreatment::Free,
        super::state::ModalSupportTreatment::FreeFrozenFriction,
    ]
    .into_iter()
    .enumerate()
    {
        let mut support = first.clone();
        support.support_id = format!("{}:{i}", first.support_id);
        support.treatment = treatment;
        state.supports.push(support);
    }
    let doc = modal_doc_solved(&o, &binding, standing);
    let treatments: Vec<&str> = doc["tables"]["supports"]["rows"].as_array().unwrap().iter().map(|r| r["treatment"].as_str().unwrap()).collect();
    for name in ["restrained", "already_restrained", "free", "free_frozen_friction"] {
        assert!(treatments.contains(&name), "{name} in {treatments:?}");
    }
    modal_env_check_solved("every support treatment", &o, &binding, standing);
    // A state case without a linearization (the outcome a model without a
    // nonlinear support gives, `state.rs:326`).
    let mut o = outcome.clone();
    o.nonlinear_state = None;
    let doc = modal_doc_solved(&o, &binding, standing);
    assert_eq!((doc["basis"]["state_case"]["linearized"].as_bool(), doc["basis"]["state_case"]["solver_mode"].is_null()), (Some(false), true));
    modal_env_check_solved("state case, not linearized", &o, &binding, standing);
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

/// The adversarial requests (name, request, parameters, label), every one
/// refused.
fn modal_env_adversarial() -> Vec<(String, Value, ModalIntegrityParameters, String)> {
    let params = test_local_parameters();
    let local = |name: &str, request: Value| (name.to_owned(), request, test_local_parameters(), "test-local".to_owned());
    let mut wide = with(pf_request(false), json!({"mode_count": modal_env_nested(60), "frequency_cutoff": (0..5000).map(|i| json!([i, -i, 0.1 * i as f64])).collect::<Vec<_>>()}));
    wide[AWKWARD] = json!(AWKWARD);
    wide["ü".repeat(500)] = json!({"x": [1e-7, -0.0, 5e-324, 1.7976931348623157e308]});
    for name in ["state_case", "modulus_basis_ref", "modulus_basis_temperature", "weight_to_mass_acceleration", "include_dynamic_only_restraints"] {
        wide[name] = json!({ "echo": name });
    }
    let mut all = vec![local("deep and wide echo", wide)];
    // Each escaping class in an undeclared name alone, and as the label.
    for (i, class) in ["\"", "\\", "\u{8}", "\u{9}", "\u{a}", "\u{c}", "\u{d}", "\u{0}", "\u{1f}", "\u{7f}", "é", "€", "😀"].iter().enumerate() {
        let request = with(pf_request(false), json!({ format!("member{class}{i}"): class }));
        all.push((format!("escape class {i}"), request, params.clone(), (*class).to_owned()));
    }
    // Requests that are not objects, a request whose hash refuses (an unsafe
    // integer; a container 130 deep), and an integral float echo.
    for (name, request) in [
        ("a string request", json!(AWKWARD)),
        ("an array request", json!([1, [2, [3]]])),
        ("an unsafe integer", with(pf_request(false), json!({"mode_count": 9_007_199_254_740_993u64, "zz": -9_007_199_254_740_993i64}))),
        ("a 130-deep echo", with(pf_request(false), json!({"mode_count": modal_env_nested(130)}))),
        ("an integral float echo", with(pf_request(false), json!({"mode_count": 1e300, "zz": 2.0}))),
        // The float parser's slow path (`bhcomp`) on the writer's path.
        ("a float echo on the slow parse path", with(pf_request(false), json!({"mode_count": 4.1e-23, "zz": [5e-324, 2.2250738585072014e-308]}))),
    ] {
        all.push(local(name, request));
    }
    // Parameters that are not finite or not safe integers (the parse refusal
    // precedes their validation).
    let mut odd = test_local_parameters();
    odd.residual_threshold = f64::NAN;
    odd.sturm_growth_limit = 1e300;
    odd.sturm_max_candidates = 1 << 60;
    odd.sturm_gap_log_positions.push(f64::INFINITY);
    all.push(("parameters outside the profile".to_owned(), with(pf_request(false), json!({"zz": 1})), odd, "test-local".to_owned()));
    all
}

#[test]
fn modal_env_adversarial_requests_are_exact_on_the_parse_refusal_branch() {
    for (name, request, params, label) in modal_env_adversarial() {
        let doc = modal_result_document(&request, &params, &label);
        assert_eq!(doc["status"], "refused", "{name}");
        if name == "deep and wide echo" {
            assert_eq!(doc["refusals"]["rows"][0]["code"], "MODAL_REQUEST_INVALID", "{name}: {:#}", doc["refusals"]);
        }
        let envelope = modal_env_assert_branch(&name, &request, &params, &label);
        assert!(envelope.construction_peak_bytes > envelope.heap_bytes, "{name}");
        if name == "parameters outside the profile" {
            assert!(doc["binding"]["integrity_parameters_sha256"].is_null());
        }
    }
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

#[test]
fn modal_env_hashes_at_parser_depths_127_and_128_are_a_string_and_null() {
    // The checked re-parse refuses a container nested 128 deep: a value of
    // depth 127 hashes, one of 128 does not, in the writer and the tally.
    let params = test_local_parameters();
    for (depth, hashed) in [(127u64, true), (128u64, false)] {
        let nested = modal_env_nested(depth as usize);
        assert_eq!(modal_env_observe(&nested).depth, depth);
        assert_eq!(modal_doc_sha256(&nested).is_some(), hashed, "the writer's hash at depth {depth}");
        assert_eq!(modal_env_digest(&modal_env_value(&nested, CAP).unwrap()).strings == 1, hashed, "the tally's hash at depth {depth}");
        // A request of that depth: its own hash, and its model's (depth 4) beside it.
        let request = with(pf_request(false), json!({"mode_count": 1, "zz": modal_env_nested(depth as usize - 1)}));
        assert_eq!(modal_env_observe(&request).depth, depth);
        let doc = modal_result_document(&request, &params, "test-local");
        assert_eq!(doc["binding"]["request_sha256"].is_string(), hashed, "request_sha256 at depth {depth}");
        assert_eq!(doc["binding"]["request_sha256"].is_null(), !hashed, "request_sha256 at depth {depth}");
        assert!(doc["binding"]["model_sha256"].is_string());
        // The run identity is a string either way (`modal-run:` and 16 digits).
        assert_eq!(doc["run"]["run_id"].as_str().unwrap().len(), "modal-run:".len() + 16);
        modal_env_assert_branch(&format!("a request {depth} deep"), &request, &params, "test-local");
    }
}

#[test]
fn modal_env_a_run_refusal_with_several_diagnostics_and_a_resolved_state_case_is_exact() {
    let mut odd = test_local_parameters();
    odd.residual_threshold = f64::NAN;
    odd.cluster_tolerance = 2.0;
    odd.sturm_growth_limit = 0.5;
    let request = with(pf_request(false), json!({"mode_count": 1, "state_case": "case:f"}));
    let doc = modal_result_document(&request, &odd, "test-local");
    assert_eq!(doc["status"], "refused");
    assert_eq!(doc["refusals"]["row_count"], 3, "{:#}", doc["refusals"]);
    assert_eq!((doc["binding"]["state_case"]["case_id"].as_str(), doc["binding"]["state_case"]["case_index"].as_u64()), (Some("case:f"), Some(0)));
    assert!(doc["binding"]["state_case"]["case_sha256"].is_string());
    modal_env_assert_branch("a run refusal of three with a state case", &request, &odd, "test-local");
}

#[test]
fn modal_env_a_state_case_on_a_linear_model_is_exact_unlinearized() {
    let params = test_local_parameters();
    let request = with(pf_request(false), json!({"mode_count": 2, "state_case": "case:f"}));
    let doc = modal_result_document(&request, &params, "test-local");
    assert_eq!(doc["status"], "solved", "{:#}", doc["refusals"]);
    assert_eq!(doc["basis"]["state_case"]["linearized"], false);
    assert!(doc["basis"]["state_case"]["restrained_dof_refs"].is_null());
    modal_env_assert_branch("a state case on a linear model", &request, &params, "test-local");
}

#[test]
fn modal_env_a_given_binding_is_tallied_as_it_is() {
    // `modal_doc_solved` takes any binding value; its run identity hashes
    // the binding's two hash members whatever they hold.
    let (case, outcome, binding) = modal_env_base("component_body");
    let standing = ModalParameterStanding::modal_doc_standing_of(&case.params, case.label);
    let mut unsafe_hash = binding.clone();
    unsafe_hash["request_sha256"] = json!(9_007_199_254_740_992u64);
    let doc = modal_doc_solved(&outcome, &unsafe_hash, standing);
    assert_eq!(doc["run"]["run_id"], "modal-run:", "the run hash is refused, so the id has no digits");
    modal_env_check_solved("an unsafe integer as the request hash", &outcome, &unsafe_hash, standing);
    let mut long = binding.clone();
    long["integrity_parameters_sha256"] = json!(AWKWARD.repeat(5000));
    long["request_sha256"] = json!({"nested": [AWKWARD, 1.5, null]});
    modal_env_check_solved("long text and an object as the hashes", &outcome, &long, standing);
    let mut absent = binding.clone();
    absent.as_object_mut().unwrap().remove("request_sha256");
    modal_env_check_solved("an absent request hash", &outcome, &absent, standing);
    modal_env_check_solved("a binding that is not an object", &outcome, &json!([1, 2]), standing);
    // The larger run input raises the identity's transient, so C.
    let base = modal_env_solved_with_binding(&outcome, &binding, standing, CAP).unwrap();
    let wide = modal_env_solved_with_binding(&outcome, &long, standing, CAP).unwrap();
    assert!(wide.hash_peak_bytes > base.hash_peak_bytes && wide.construction_peak_bytes > base.construction_peak_bytes);
}

#[test]
fn modal_env_a_made_diagnostic_counts_its_capacities() {
    let params = test_local_parameters();
    let request = with(pf_request(false), json!({"zz": 1}));
    let standing = ModalParameterStanding::TestLocal("test-local");
    let refusal = ModalPreviewRequest::parse_modal_request(request.clone()).err().unwrap();
    let tight = modal_env_parse_diagnostic(refusal.clone());
    let mut spare = refusal;
    spare.reserve_exact(4096 - spare.len());
    let loose = modal_env_parse_diagnostic(spare);
    assert_eq!(loose.message.capacity(), 4096);
    let a = modal_env_parse_refused(&request, &params, standing, &tight, CAP).unwrap();
    let b = modal_env_parse_refused(&request, &params, standing, &loose, CAP).unwrap();
    assert_eq!(a.tally, b.tally, "the document is the same");
    assert_eq!(b.construction_peak_bytes - a.construction_peak_bytes, (4096 - tight.message.capacity()) as u64, "D is the capacity held");
}

#[test]
fn modal_env_a_bounded_diagnostic_is_checked_by_capacity() {
    let d = modal_env_diagnostic("plain", 2);
    let fit = |text: &str| modal_env_text_of(text);
    let longest = d.affected_refs.iter().map(|r| fit(r)).max_by_key(|t| t.raw).unwrap();
    let bound = ModalEnvelopeDiagnosticBound {
        id: fit(&d.id),
        code: fit(&d.code),
        severity: fit(&d.severity),
        message: fit(&d.message),
        affected_refs: d.affected_refs.capacity() as u64,
        affected_ref: ModalEnvelopeText { raw: longest.raw.max(d.affected_refs.iter().map(|r| r.capacity() as u64).max().unwrap()), ..longest },
    };
    let shrink = |mut d: Diagnostic| {
        for s in [&mut d.id, &mut d.code, &mut d.severity, &mut d.message] {
            s.shrink_to_fit();
        }
        d
    };
    let d = shrink(d);
    assert!(modal_env_diagnostic_within(&d, &bound).unwrap());
    // A one-byte message held in a larger buffer meets the length bound but
    // not the capacity bound.
    let mut wide = d.clone();
    wide.message = String::with_capacity(bound.message.raw as usize + 1);
    wide.message.push('m');
    assert!(!modal_env_diagnostic_within(&wide, &bound).unwrap());
    let mut refs = d.clone();
    refs.affected_refs.reserve_exact(10);
    assert!(!modal_env_diagnostic_within(&refs, &bound).unwrap());
    let mut source = d.clone();
    source.source = Some(String::with_capacity(MODAL_ENV_SOURCE_MAX as usize + 1));
    assert!(!modal_env_diagnostic_within(&source, &bound).unwrap());
    let mut escaped = d;
    escaped.code = "\u{1}".repeat(bound.code.raw as usize);
    escaped.code.shrink_to_fit();
    assert!(!modal_env_diagnostic_within(&escaped, &bound).unwrap(), "within the raw length, not the escaped");
}

#[test]
fn modal_env_writer_width_arithmetic_is_checked_at_the_usize_limit() {
    // Each `usize` operation of the writer that the mirror reproduces,
    // performed by the mirror in `usize` and checked (`document.rs:555, 593,
    // 650, 736-737, 751, 768, 785, 797-798, 815`). Run at this host's
    // `usize::MAX`; a narrower `usize` takes the same checked operation and
    // is not run separately.
    let unprojectable = |r: ModalEnvelopeResult<u64>| matches!(r, Err(ModalEnvelopeError::Unprojectable { .. }));
    assert!(unprojectable(modal_env_next(usize::MAX)));
    assert_eq!(modal_env_next(usize::MAX - 1).unwrap(), usize::MAX as u64);
    assert!(matches!(modal_env_previous(0), Err(ModalEnvelopeError::Unprojectable { what: "mode index 0" })));
    assert!(unprojectable(modal_env_ordinal(0, 6, 0)), "a mode index 0");
    assert!(unprojectable(modal_env_ordinal(usize::MAX, 6, 0)), "a shape ordinal (:751)");
    assert!(unprojectable(modal_env_ordinal(usize::MAX / 3 + 2, 3, 0)), "a participation ordinal (:768)");
    assert!(unprojectable(modal_env_ordinal(2, 3, usize::MAX)), "an ordinal's offset");
    assert_eq!(modal_env_ordinal(1, 6, usize::MAX).unwrap(), usize::MAX as u64);
    assert_eq!(modal_env_ordinal(usize::MAX / 3 + 1, 3, 0).unwrap(), (usize::MAX / 3 * 3) as u64);
    assert!(unprojectable(modal_env_ordinal(usize::MAX / 3 + 1, 3, usize::MAX - usize::MAX / 3 * 3 + 1)));
    // Every site the writer reaches first, on a solved outcome: the mirror
    // refuses it by name, and the writer panics where overflow checks are on
    // (debug-assertion builds here) or wraps where they are off.
    let (case, outcome, binding) = modal_env_base("component_body");
    let standing = ModalParameterStanding::modal_doc_standing_of(&case.params, case.label);
    let mut sites: Vec<(&str, ModalOutcome)> = Vec::new();
    let mut add = |name, edit: &dyn Fn(&mut ModalOutcome)| {
        let mut o = outcome.clone();
        edit(&mut o);
        sites.push((name, o));
    };
    add("the endpoint's next_mode + 1 (:593)", &|o| {
        o.completed.endpoint = super::eigen::ReportedEndpoint::BelowComputedCluster { next_mode: usize::MAX }
    });
    add("an orthogonality pair's a + 1 (:650)", &|o| o.integrity.orthogonality.pair = Some((usize::MAX, 0)));
    add("an orthogonality pair's b + 1 (:650)", &|o| o.integrity.orthogonality.pair = Some((0, usize::MAX)));
    add("a mode index 0 (:736)", &|o| o.modes[0].index = 0);
    add("a mode's cluster + 1 (:737)", &|o| o.modes[0].cluster = usize::MAX);
    add("a shape ordinal (:751)", &|o| o.modes[0].index = usize::MAX);
    add("a cluster's id + 1 (:785)", &|o| o.clusters[0].id = usize::MAX);
    add("a cluster's first mode + 1 (:785)", &|o| o.clusters[0].modes.start = usize::MAX);
    add("a computed mode's index + 1 (:797)", &|o| o.integrity.computed[0].index = usize::MAX);
    add("a computed mode's cluster + 1 (:797)", &|o| o.integrity.computed[0].cluster = usize::MAX);
    for (name, o) in &sites {
        let found = modal_env_solved_with_binding(o, &binding, standing, CAP);
        assert!(matches!(found, Err(ModalEnvelopeError::Unprojectable { .. })), "{name}: {found:?}");
        if cfg!(debug_assertions) {
            let built = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| modal_doc_solved(o, &binding, standing)));
            assert!(built.is_err(), "{name}: the writer panics with overflow checks on");
        }
    }
    // Not reachable as a document's first failure, so held above at the
    // helper only: the participation ordinal (:768) follows the shape
    // ordinal (:751) of the same mode, which overflows first when there is a
    // DOF, and a model without one fails at the mode's reference DOF; a
    // candidate's j + 1 (:815) and 6·nodes (:555) need a vector longer than
    // memory allows.
}

#[test]
fn modal_env_the_longest_refusal_message_is_within_the_refusal_stage() {
    let stage = modal_env_refusal_stage();
    assert_eq!(stage, 3 * MODAL_ENV_VALIDATION_MESSAGE_MAX);
    let refuse = |value: &Value| canonical_json_checked_v1_text(&serde_json::to_string(value).unwrap()).unwrap_err();
    // The reader's witness, and the longest message the derivation names.
    assert_eq!(format!("{}", -f64::MAX).len(), 310);
    for (name, value, length) in [
        ("f64::MAX", json!(f64::MAX), Some(345)),
        ("-f64::MAX", json!(-f64::MAX), Some(346)),
        ("i64::MIN", json!(i64::MIN), Some(49)),
        ("u64::MAX", json!(u64::MAX), Some(49)),
        ("2^53 + 2", json!(9_007_199_254_740_994.0), None),
    ] {
        let message = refuse(&value);
        let (len, capacity) = (message.len() as u64, message.capacity() as u64);
        println!("refusal {name}: {len} bytes in a capacity of {capacity}");
        if let Some(length) = length {
            assert_eq!(len, length, "{name}: {message}");
        }
        assert!(len <= MODAL_ENV_VALIDATION_MESSAGE_MAX, "{name}");
        assert!(capacity <= (2 * len).max(8), "{name}: P3");
        // Its last growth: the old buffer (< len) beside the new.
        assert!(capacity + len <= stage, "{name}: the refusal stage");
    }
    // Every integral binary64 above 2^53 in magnitude, sampled across its
    // exponents.
    for exponent in 53..=1023 {
        for v in [2f64.powi(exponent), -2f64.powi(exponent) * 1.75, f64::MAX / 2f64.powi(1023 - exponent)] {
            if v.fract() == 0.0 && v.abs() > 9_007_199_254_740_991.0 {
                let message = refuse(&json!(v));
                assert!(message.len() as u64 <= MODAL_ENV_VALIDATION_MESSAGE_MAX && message.capacity() as u64 + message.len() as u64 <= stage, "{v}");
            }
        }
    }
    // The parse refusal on serialized text: the depth limit.
    let message = refuse(&modal_env_nested(128));
    assert!(message.starts_with("CHECKED-JSON-INVALID: recursion limit exceeded at line 1 column "), "{message}");
    assert!(message.len() as u64 <= MODAL_ENV_PARSE_MESSAGE_MAX, "{message}");
    assert!(3 * message.len() as u64 + 64 <= stage);
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
    // A bound of u64::MAX refs is refused at once, in closed form.
    let params = test_local_parameters();
    let huge = ModalEnvelopeDiagnosticBound { affected_refs: u64::MAX, affected_ref: modal_env_text_of("r"), ..Default::default() };
    let found = modal_env_fallback_refused(&pf_request(false), &params, ModalParameterStanding::TestLocal("t"), None, &huge, CAP);
    assert!(matches!(found, Err(ModalEnvelopeError::Overflow { .. })), "{found:?}");
}

#[test]
fn modal_env_a_repeated_array_is_the_elementwise_one() {
    for each in [
        modal_env_value(&json!("r\u{1}é"), CAP).unwrap(),
        modal_env_value(&json!({"a": [1.5, {"b": null}], "c": []}), CAP).unwrap(),
        modal_env_value(&json!([]), CAP).unwrap(),
    ] {
        for n in [0u64, 1, 2, 7] {
            let mut array = ModalEnvelopeArray::default();
            for _ in 0..n {
                array.modal_env_element(each).unwrap();
            }
            assert_eq!(modal_env_repeated(n, &each).unwrap(), array.modal_env_array().unwrap(), "{n} of {each:?}");
        }
    }
}

#[test]
fn modal_env_the_hash_peak_is_not_monotone_in_a_componentwise_maximum() {
    // Why X_row and the row hash peak are maxima over rows, not of a maximum
    // tally: HP subtracts float bytes.
    let a = modal_env_value(&json!([0.0, "x".repeat(1000)]), CAP).unwrap();
    let b = modal_env_value(&json!([5e-324]), CAP).unwrap();
    let joint = modal_env_max(&a, &b);
    assert!(modal_env_hash_peak(&joint).unwrap() < modal_env_hash_peak(&a).unwrap());
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
    // P2 (corroborated, not established: layout and overlap are the
    // premise): capacity after pushes to n is max(2·old, required, minimum),
    // so ≤ max(4, 2n) for values and ≤ max(8, 2n) for bytes.
    let mut values = Vec::new();
    let mut text = String::new();
    for n in 1..=5000u64 {
        values.push(Value::Null);
        text.push('x');
        assert!(values.capacity() as u64 <= (2 * n).max(4), "{n}");
        assert!(text.capacity() as u64 <= (2 * n).max(8), "{n}");
    }
    // P3: format! capacity ≤ max(2L, 8), sampled with the writer's own format
    // strings (`document.rs`) and argument types, since the literal pieces set
    // the first reservation.
    let long = "ü".repeat(200);
    let ids = [long.clone(), "a".to_owned(), String::new()];
    let components = ["ux", "rz"];
    for i in [0usize, 1, 9, 10, 99_999, usize::MAX] {
        let dof = |n: usize| format!("dof:{}:{}", ids[n % 3], components[n % 2]);
        let (a, b) = (dof(i), dof(i.wrapping_add(1)));
        let id = dof(i.wrapping_add(2));
        let node_id: &str = &ids[i % 3];
        let index = i;
        let prefix = if i % 2 == 0 { "refusal" } else { "diagnostic" };
        let ordinal = i;
        let run = "0123456789abcdef0123456789abcdef";
        let mut samples = vec![
            dof(i),
            format!("node:{node_id}"),
            format!("mode:{index}"),
            format!("computed:{index}"),
            format!("cluster:{index}"),
            format!("{prefix}:{ordinal}"),
            format!("mass:{a}|{b}"),
            format!("mass-total:{}", ["x", "y", "z"][i % 3]),
            format!("shape:{}:{id}", format!("mode:{index}")),
            format!("participation:{}:{}", format!("mode:{index}"), ["x", "y", "z"][i % 3]),
            format!("sturm-candidate:{}", i.wrapping_add(1)),
            format!("identity:{i}"),
            format!("support:{}", ids[i % 3]),
            format!("member:{}", ids[i % 3]),
            format!("disclosure:{i}"),
            format!("modal-run:{}", run.get(..16).unwrap_or("")),
            format!("modal-run:{}", "".get(..16).unwrap_or("")),
        ];
        for b in 0..=255u8 {
            samples.push(format!("{b:02x}"));
        }
        for s in samples {
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

// ---------------------------------------------------------------------------
// OBSERVATIONS of the peaks (release test builds only).

/// OBSERVATIONS that corroborate C and HP, and that the tally requests no
/// bytes; not their proof. They read the crate's existing counting global
/// allocator (`sparse_loop_memory_tests.rs`, registered in release lib test
/// builds), serialized with its own tests by its `SERIAL` lock. Each region
/// reports the measuring thread's peak of requested bytes above its start (a
/// reallocation counted as its larger buffer) and the process's (a moving
/// reallocation counted as its old and new buffers together; run with
/// `--test-threads 1`, so no other test allocates meanwhile). Run:
/// `cargo test --release --lib modal::tests_document_envelope::modal_env_observed -- --test-threads 1 --nocapture`.
#[cfg(not(debug_assertions))]
mod modal_env_observed {
    use super::super::document::modal_doc_subject_census;
    use super::*;
    use crate::sparse_loop_memory_tests::{endpoint, reset_peaks, SERIAL};

    #[derive(Debug, Clone, Copy)]
    struct ModalEnvelopeRegion {
        thread_peak: u64,
        process_peak: u64,
        allocations: u64,
        held: i64,
    }

    fn modal_env_region<R>(f: impl FnOnce() -> R) -> (R, ModalEnvelopeRegion) {
        let start = reset_peaks(1);
        let out = std::hint::black_box(f());
        let end = endpoint(1);
        let region = ModalEnvelopeRegion {
            thread_peak: (end.thread_peak - start.thread_live).max(0) as u64,
            process_peak: end.peak_move.saturating_sub(start.live_req) as u64,
            allocations: (end.at.allocs - start.allocs) as u64,
            held: (end.at.thread_live - start.thread_live) as i64,
        };
        (out, region)
    }

    /// Every observation and its bound; the tightest is reported.
    #[derive(Default)]
    struct ModalEnvelopeLog {
        rows: Vec<(String, u64, u64)>,
    }

    impl ModalEnvelopeLog {
        fn modal_env_within(&mut self, name: String, region: &ModalEnvelopeRegion, bound: u64) {
            let observed = region.thread_peak.max(region.process_peak);
            println!("modal-env-observed {name}: thread {} process {} bound {bound}", region.thread_peak, region.process_peak);
            assert!(observed <= bound, "OBSERVATION exceeds the bound: {name}: observed {observed} (thread {}, process {}), bound {bound}", region.thread_peak, region.process_peak);
            self.rows.push((name, observed, bound));
        }

        fn modal_env_report(&self, kind: &str) {
            let tightest = self.rows.iter().filter(|(_, o, _)| *o > 0).min_by(|a, b| (a.2 as f64 / a.1 as f64).total_cmp(&(b.2 as f64 / b.1 as f64)));
            if let Some((name, observed, bound)) = tightest {
                println!("modal-env-observed {kind} TIGHTEST {name}: observed {observed} bound {bound} ratio {:.3} over {} regions", *bound as f64 / *observed as f64, self.rows.len());
            }
        }
    }

    /// The writer's whole call against C: the parse and solve before the
    /// document are excluded from C, so the region's peak is held to
    /// max(their own peak, what they leave held + C), each observed in a
    /// separate run of the same parse and solve.
    fn modal_env_whole(log: &mut ModalEnvelopeLog, name: &str, request: &Value, params: &ModalIntegrityParameters, label: &str) {
        let c = modal_env_mirror(request, params, label, CAP).unwrap_or_else(|e| panic!("{name}: {e:?}")).construction_peak_bytes;
        let (before, excluded) = modal_env_region(|| {
            ModalPreviewRequest::parse_modal_request(request.clone()).map(|parsed| {
                let run = run_modal_dense(&parsed, params);
                (parsed, run)
            })
        });
        drop(before);
        let (doc, whole) = modal_env_region(|| modal_result_document(request, params, label));
        drop(doc);
        let held = excluded.held.max(0) as u64;
        let bound = excluded.thread_peak.max(excluded.process_peak).max(held + c);
        log.modal_env_within(format!("whole {name} (excluded peak {}, held {held}, C {c})", excluded.thread_peak.max(excluded.process_peak)), &whole, bound);
    }

    /// `modal_doc_solved` alone, for a given binding, against its C.
    fn modal_env_direct(log: &mut ModalEnvelopeLog, name: &str, outcome: &ModalOutcome, binding: &Value, standing: ModalParameterStanding<'_>) {
        let c = modal_env_solved_with_binding(outcome, binding, standing, CAP).unwrap_or_else(|e| panic!("{name}: {e:?}")).construction_peak_bytes;
        let (doc, region) = modal_env_region(|| modal_doc_solved(outcome, binding, standing));
        drop(doc);
        log.modal_env_within(format!("solved {name}"), &region, c);
    }

    #[test]
    fn modal_env_observed_construction_is_within_c() {
        let _serial = SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut log = ModalEnvelopeLog::default();
        for case in modal_env_cases() {
            modal_env_whole(&mut log, case.name, &case.request, &case.params, case.label);
            let doc = modal_result_document(&case.request, &case.params, case.label);
            if doc["status"] != "solved" {
                continue;
            }
            let standing = ModalParameterStanding::modal_doc_standing_of(&case.params, case.label);
            let outcome = run_modal_dense(&modal(case.request.clone()), &case.params).unwrap();
            modal_env_direct(&mut log, case.name, &outcome, &doc["binding"], standing);
            // The solved branch's tail: the census into the binding, then the
            // document, against the branch's C.
            let c = modal_env_solved(&outcome, &case.request, &case.params, standing, CAP).unwrap().construction_peak_bytes;
            let mut binding = doc["binding"].clone();
            binding["subject_census"] = Value::Null;
            let (built, region) = modal_env_region(|| {
                binding["subject_census"] = modal_doc_subject_census(&outcome);
                modal_doc_solved(&outcome, &binding, standing)
            });
            assert_eq!(built, doc, "{}", case.name);
            drop(built);
            log.modal_env_within(format!("tail {}", case.name), &region, c);
        }
        // Synthetic outcomes, the B3 cases (retained originals, rows unlike
        // one another) among them, and given bindings.
        let (case, outcome, binding) = modal_env_base("component_body");
        let standing = ModalParameterStanding::modal_doc_standing_of(&case.params, case.label);
        for (name, o) in &modal_env_variants(&outcome) {
            modal_env_direct(&mut log, name, o, &binding, standing);
        }
        let mut long = binding.clone();
        long["integrity_parameters_sha256"] = json!(AWKWARD.repeat(5000));
        modal_env_direct(&mut log, "a long run input", &outcome, &long, standing);
        let (case, outcome, binding) = modal_env_base("state_friction_sliding");
        let standing = ModalParameterStanding::modal_doc_standing_of(&case.params, case.label);
        let mut o = outcome.clone();
        o.nonlinear_state.as_mut().unwrap().supports.iter_mut().for_each(|s| s.support_id.push_str(&AWKWARD.repeat(50)));
        modal_env_direct(&mut log, "a linearized state with long support ids", &o, &binding, standing);
        // Refused branches and the writer's float slow path.
        for (name, request, params, label) in modal_env_adversarial() {
            modal_env_whole(&mut log, &name, &request, &params, &label);
        }
        let params = test_local_parameters();
        modal_env_whole(&mut log, "skew", &modal_env_skew_request(), &params, "test-local");
        let mut odd = test_local_parameters();
        odd.residual_threshold = f64::NAN;
        odd.cluster_tolerance = 2.0;
        modal_env_whole(&mut log, "a run refusal of two with a state case", &with(pf_request(false), json!({"mode_count": 1, "state_case": "case:f"})), &odd, "test-local");
        modal_env_whole(&mut log, "a state case on a linear model", &with(pf_request(false), json!({"mode_count": 2, "state_case": "case:f"})), &params, "test-local");
        for depth in [127usize, 128] {
            let request = with(pf_request(false), json!({"mode_count": 1, "zz": modal_env_nested(depth - 1)}));
            modal_env_whole(&mut log, &format!("a request {depth} deep"), &request, &params, "test-local");
        }
        log.modal_env_report("construction");
    }

    /// The unsealed rows and checksum arrays a document's builder hashed.
    fn modal_env_hashed_parts(doc: &Value, out: &mut Vec<(String, Value)>) {
        let mut tables: Vec<(String, &Value)> = Vec::new();
        if let Some(refusals) = doc.get("refusals") {
            tables.push(("refusals".to_owned(), refusals));
        }
        if let Some(map) = doc.get("tables").and_then(Value::as_object) {
            tables.extend(map.iter().map(|(name, table)| (name.clone(), table)));
        }
        for (name, table) in tables {
            let rows = table["rows"].as_array().unwrap();
            out.push((format!("{name} checksums"), Value::Array(rows.iter().map(|row| row["checksum"].clone()).collect())));
            for (i, row) in rows.iter().enumerate() {
                let mut unsealed = row.clone();
                unsealed.as_object_mut().unwrap().remove("checksum");
                out.push((format!("{name} row {i}"), unsealed));
            }
        }
    }

    #[test]
    fn modal_env_observed_hashing_is_within_hp() {
        let _serial = SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut values: Vec<(String, Value)> = vec![
            // B2's witnesses: the float parser's slow path.
            ("4.1e-23".to_owned(), json!(4.1e-23)),
            ("[4.1e-23]".to_owned(), json!([4.1e-23])),
            ("{mode_count: 4.1e-23}".to_owned(), json!({"mode_count": 4.1e-23})),
            ("float extremes".to_owned(), json!([5e-324, 4.9406564584124654e-324, 2.2250738585072014e-308, 1.7976931348623157e308, 1e-300, 1e300, 0.1, 123456789012345680.0])),
            // B3's pair.
            ("[0.0, 1000 bytes]".to_owned(), json!([0.0, "x".repeat(1000)])),
            ("[5e-324]".to_owned(), json!([5e-324])),
            ("a value 127 deep".to_owned(), modal_env_nested(127)),
            // N2's witness and the longest validation refusal.
            ("f64::MAX".to_owned(), json!(f64::MAX)),
            ("-f64::MAX".to_owned(), json!(-f64::MAX)),
            ("[-f64::MAX]".to_owned(), json!([-f64::MAX])),
            ("null".to_owned(), json!(null)),
            ("the semantic table".to_owned(), modal_semantic_table()),
        ];
        for case in modal_env_cases() {
            let doc = modal_result_document(&case.request, &case.params, case.label);
            values.push((format!("{} request", case.name), case.request.clone()));
            values.push((format!("{} binding", case.name), doc["binding"].clone()));
            values.push((format!("{} document", case.name), doc.clone()));
            let mut parts = Vec::new();
            modal_env_hashed_parts(&doc, &mut parts);
            values.extend(parts.into_iter().map(|(part, v)| (format!("{} {part}", case.name), v)));
        }
        for (name, request, ..) in modal_env_adversarial() {
            values.push((name, request));
        }
        let mut log = ModalEnvelopeLog::default();
        for (name, value) in &values {
            let hp = modal_env_hash_peak(&modal_env_value(value, CAP).unwrap()).unwrap();
            let (digest, region) = modal_env_region(|| modal_doc_sha256(value));
            drop(digest);
            log.modal_env_within(format!("hash {name}"), &region, hp);
        }
        log.modal_env_report("hashing");
    }

    /// Stage observations tight enough to fail without the fixed term each
    /// names: the bound for the stage without that term is exceeded by what
    /// is observed. Not cheap to isolate, so not observed this way: C's 2048
    /// and HP's 128, 256 and 384 fixed pieces (their stages are inside the
    /// writer's and the canonical crate's private code, where no region can
    /// be set around them alone); the whole-value comparisons above would
    /// still pass without them.
    #[test]
    fn modal_env_observed_fixed_terms_are_needed() {
        let _serial = SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        // The float parser's limbs (B2): a lone number parsed on the slow
        // path holds nothing but them (the number is inline in its Value, and
        // `from_str` starts with an empty scratch).
        for text in ["4.1e-23", "[4.1e-23]", "-4.1e-23"] {
            let (value, region) = modal_env_region(|| serde_json::from_str::<Value>(text).unwrap());
            drop(value);
            let observed = region.thread_peak.max(region.process_peak);
            let elements = if text.starts_with('[') { 4 * MODAL_ENV_VALUE_BYTES } else { 0 };
            println!("modal-env-observed stage float parse {text}: {region:?}, limbs term {MODAL_ENV_FLOAT_PARSE_BYTES}");
            assert!(observed <= MODAL_ENV_FLOAT_PARSE_BYTES + elements, "{text}: {region:?}");
            assert!(observed > elements, "{text}: the bound without the limbs term would hold, so this would not detect it: {region:?}");
        }
        // The refusal stage (N2): the checked text of -f64::MAX, whose parse
        // holds at most the limbs and whose validation the refusal stage.
        for value in [json!(f64::MAX), json!(-f64::MAX)] {
            let text = serde_json::to_string(&value).unwrap();
            let (refused, region) = modal_env_region(|| canonical_json_checked_v1_text(&text));
            assert!(refused.is_err());
            drop(refused);
            let observed = region.thread_peak.max(region.process_peak);
            let stage = modal_env_refusal_stage();
            println!("modal-env-observed stage validation refusal {text}: {region:?}, limbs {MODAL_ENV_FLOAT_PARSE_BYTES} + stage {stage}; the former 512 would have bounded {}", MODAL_ENV_FLOAT_PARSE_BYTES + 512);
            assert!(observed <= MODAL_ENV_FLOAT_PARSE_BYTES + stage, "{text}: {region:?}");
            assert!(observed > MODAL_ENV_FLOAT_PARSE_BYTES, "{text}: the bound without the refusal stage would hold: {region:?}");
        }
    }

    #[test]
    fn modal_env_observed_the_tally_requests_nothing() {
        let _serial = SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut regions = Vec::new();
        for case in modal_env_cases() {
            let standing = ModalParameterStanding::modal_doc_standing_of(&case.params, case.label);
            let doc = modal_result_document(&case.request, &case.params, case.label);
            match ModalPreviewRequest::parse_modal_request(case.request.clone()) {
                Err(refusal) => {
                    let d = modal_env_parse_diagnostic(refusal);
                    regions.push(modal_env_region(|| modal_env_parse_refused(&case.request, &case.params, standing, &d, CAP).is_ok()));
                }
                Ok(parsed) => match run_modal_dense(&parsed, &case.params) {
                    Err(refusals) => {
                        regions.push(modal_env_region(|| modal_env_run_refused(&case.request, &case.params, standing, None, &refusals, CAP).is_ok()));
                        let d = &refusals[0];
                        let bound = ModalEnvelopeDiagnosticBound {
                            id: modal_env_text_of(&d.id),
                            code: modal_env_text_of(&d.code),
                            severity: modal_env_text_of(&d.severity),
                            message: modal_env_text_of(&d.message),
                            affected_refs: 4,
                            affected_ref: modal_env_text_of("model"),
                        };
                        regions.push(modal_env_region(|| modal_env_fallback_refused(&case.request, &case.params, standing, None, &bound, CAP).is_ok()));
                        regions.push(modal_env_region(|| modal_env_diagnostic_within(d, &bound).is_ok()));
                    }
                    Ok(outcome) => {
                        regions.push(modal_env_region(|| modal_env_solved(&outcome, &case.request, &case.params, standing, CAP).is_ok()));
                        regions.push(modal_env_region(|| modal_env_solved_with_binding(&outcome, &doc["binding"], standing, CAP).is_ok()));
                        let d = modal_env_skew_diagnostic();
                        regions.push(modal_env_region(|| modal_env_skew_refused(&case.request, &case.params, standing, None, &d, CAP).is_ok()));
                    }
                },
            }
            regions.push(modal_env_region(|| modal_env_value(&doc, CAP).is_ok()));
        }
        for (ok, region) in &regions {
            println!("modal-env-observed tally: {region:?}");
            assert!(*ok);
            assert_eq!((region.thread_peak, region.held), (0, 0), "the tally requested bytes: {region:?}");
            assert_eq!(region.allocations, 0, "an allocation during the tally: {region:?}");
        }
        println!("modal-env-observed tally: {} regions, none requested a byte", regions.len());
    }
}

//! I61 U3 grants 1 and 1b: the facade's W1 phases through the private driver. A permit
//! needs the registered build, and decision 7 forbids a test permit, so these tests enter
//! `retained_w1` (the body the permitted dispatch runs after G-C) with an observer
//! installed in the actual single ordinary run, exactly as the facade does.
use super::retained_product as rp;
use open_pipe_stress_frame_kernel::structural::retained_api as k;
use super::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
/// U1's committed successor bytes (retained_wire_tests::SUCCESSOR_SHA256).
const PINNED: [(&str, &str, &str); 2] = [
    ("sparse_interactive", "ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc", "efc1a39bbe83840df6bd0761c932b8020285d3b8c005ba8fd0b45ba10d667494"),
    ("dense_scrutiny", "6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5", "3e26499f17caff8f5fc0d46406bbe54acf43e8cb16e761784aa5074413b0ac4a"),
];

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn raw() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
fn plain(mode: PreviewSolverMode, raw: &Value) -> Vec<u8> {
    serde_json::to_vec(&run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap()
}
/// The facade's single observed ordinary run: the captured invocation, the
/// observer with capture installed, and the ordinary envelope it returned. Its
/// diagnostics carry no spare capacity, so only R-2's reservation provides the
/// notice's slot (the publish-time check would otherwise fire).
fn observed(mode: PreviewSolverMode, raw: &Value) -> (source_receipt::CapturedInvocation, rp::ProductCapture, MechanicsEnvelope) {
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
    // B3b-P (P-1, P-2): the route and its exact-block budget, as `permitted_run` decides them
    // (the preview route's are the default observer and budget, as before).
    let route = w1_route(&request.model).unwrap_or_default();
    let mut observer = rp::ProductCapture::prepared_probe_on(route);
    let mut ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut w1_budget(route), Some(&mut observer));
    ordinary.diagnostics.shrink_to_fit();
    (capture, observer, ordinary)
}
/// R-2 (N1)'s exact notice bytes, pinned here independently of the product
/// constants: the plain text, or the receipt-encoding text with its C1:68 detail.
fn notice_json(case: &str, detail: Option<&str>) -> String {
    let reason = detail.map_or(String::new(), |d| format!(" Reason: receipt_encoding; detail: {d}."));
    format!(r#"{{"id":"diagnostic:retained-precision:{case}:unavailable","code":"RETAINED_PRECISION_UNAVAILABLE","severity":"info","message":"Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics.{reason}","source":"core/product_physics","affected_refs":["{case}"]}}"#)
}
/// The ordinary bytes with the notice appended after the ordinary diagnostic prefix.
fn with_notice(plain: &[u8], case: &str, detail: Option<&str>) -> Vec<u8> {
    let text = std::str::from_utf8(plain).unwrap();
    let (head, tail) = text.split_once(r#""diagnostics":["#).unwrap();
    let close = tail.find("],\"professional_boundary\"").unwrap();
    let (items, rest) = tail.split_at(close);
    let sep = if items.is_empty() { "" } else { "," };
    format!(r#"{head}"diagnostics":[{items}{sep}{}{rest}"#, notice_json(case, detail)).into_bytes()
}
/// B1 (PLAN_v2 §2.1; RV107 SF-2): an out-of-domain oracle's request, `raw` with C + 1 load
/// cases (`caps::LOAD_CASES` + 1). Each added case copies the first. With `renamed` the
/// copies take the ids `case-2`, `case-3`, …; otherwise they keep the first case's id, as
/// each oracle built its second case before B1. While C = 1 (before SA) this is the same
/// two-case request as before.
fn beyond_load_cases(raw: &Value, renamed: bool) -> Value {
    let mut over = raw.clone();
    let first = over["model"]["load_cases"][0].clone();
    for ordinal in 2..=crate::retained_memory::caps::LOAD_CASES + 1 {
        let mut case = first.clone();
        if renamed {
            case["id"] = json!(format!("case-{ordinal}"));
        }
        over["model"]["load_cases"].as_array_mut().unwrap().push(case);
    }
    over
}

/// Control 2: the permitted path publishes U1's pinned successor bytes, in both
/// modes, through the frozen candidate and its staging copy; the ordinary
/// envelope it returns beside the successor is byte-identical to the plain route.
#[test]
fn u3_permitted_path_publishes_the_pinned_successor() {
    let out = std::env::var("I61_U3_OUT").ok().map(std::path::PathBuf::from);
    for (mode, (name, file_sha, receipt_sha)) in MODES.into_iter().zip(PINNED) {
        assert_eq!(mode.as_str(), name);
        let raw = raw();
        let plain = plain(mode, &raw);
        let (capture, observer, ordinary) = observed(mode, &raw);
        assert_eq!(serde_json::to_vec(&ordinary).unwrap(), plain, "{name}: the observed run is the plain run");
        let (envelope, retained) = retained_w1(observer, ordinary, &capture);
        assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "{name}: the ordinary owner is untouched");
        let successor = retained.unwrap_or_else(|f| panic!("{name}: {f:?}"));
        let text = serde_json::to_string_pretty(&json!({"id":format!("u1_milestone_{name}"),"source":successor.value(),
            "invocation":{"request":raw,"solver_mode":mode.as_str()}})).unwrap();
        assert_eq!((sha(text.as_bytes()).as_str(), successor.value()["retained_precision"]["receipt_sha256"].as_str()), (file_sha, Some(receipt_sha)),
            "{name}: U1's pinned successor bytes");
        if let Some(dir) = &out {
            std::fs::write(dir.join(format!("u3_successor_{name}.json")), &text).unwrap();
        }
    }
}

/// Control 3: a fault at each W1 stage falls back to the preserved ordinary bytes
/// with its private cause. R-2 (N1): where W1 work ran (preparation, native,
/// candidate, serializer, precommit) exactly one info notice follows the ordinary
/// diagnostic prefix; where none ran (coexistence, scope, notice reservation) the
/// bytes are exactly the ordinary bytes.
#[test]
fn u3_each_stage_fault_falls_back_to_the_ordinary_bytes() {
    use super::retained_receipt::TraceFault as F;
    use super::retained_wire::{ReceiptCheck as C, ReceiptFailure};
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = raw();
    let plain = plain(mode, &raw);
    let noticed = |detail: Option<&str>| with_notice(&plain, "case", detail);
    let check = |envelope: &MechanicsEnvelope, expected: &[u8], label: &str| {
        assert_eq!(String::from_utf8(serde_json::to_vec(envelope).unwrap()).unwrap(), String::from_utf8(expected.to_vec()).unwrap(), "{label}");
    };
    // Preparation: the closed annulus helper refuses (experiment 02's trigger).
    let (capture, mut observer, ordinary) = observed(mode, &raw);
    observer.facts[0].diameter = 0.0;
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::Preparation));
    check(&envelope, &noticed(None), "preparation");
    // Native: the prepared source is withdrawn before the solve (D38's trigger).
    let (capture, observer, ordinary) = observed(mode, &raw);
    retained_tests_hooks::withdraw_next_native_source();
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::Native));
    check(&envelope, &noticed(None), "native");
    // Proof/facade: injected faults after the proof started.
    for fault in [F::Maxima, F::ValuesCompletion] {
        let (capture, mut observer, ordinary) = observed(mode, &raw);
        observer.trace_fault = Some(fault);
        let (envelope, retained) = retained_w1(observer, ordinary, &capture);
        assert_eq!(retained.err(), Some(W1Fallback::Candidate), "{fault:?}");
        check(&envelope, &noticed(None), "candidate");
    }
    // Staging (RV85 N6): a broken overlay invariant falls back typed, with the notice.
    let (capture, observer, ordinary) = observed(mode, &raw);
    retained_tests_hooks::break_next_staging();
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::Staging(rp::StagingFault("pipe_stress_extrema[]"))));
    check(&envelope, &noticed(None), "staging");
    // Serializer: an invocation other than the observed one (RV82-S2) refuses typed.
    // An association refusal is not a receipt-encoding fallback: the plain notice.
    let (_, observer, ordinary) = observed(mode, &raw);
    let mut other = raw.clone();
    other["model"]["load_cases"][0]["label"] = json!("another label");
    let (_, foreign) = source_receipt::CapturedInvocation::parse(other, mode).unwrap();
    let (envelope, retained) = retained_w1(observer, ordinary, &foreign);
    assert_eq!(retained.err(), Some(W1Fallback::Serializer(ReceiptFailure { check: C::Association, field_path: "invocation" })));
    check(&envelope, &noticed(None), "serializer association");
    // Serializer receipt-encoding refusals carry C1:68's reason and detail token.
    for (fault, detail) in [(C::WorkCounterRange, Some("work_counter_range")), (C::WorkCounterInconsistent, Some("work_counter_inconsistent")),
        (C::SaturationNotExcluded, Some("saturation_not_excluded")), (C::PublicationHashRange, Some("publication_hash_range")),
        (C::WorkCounter(k::WorkFault::Overflow), Some("work_counter_range")), (C::WorkCounter(k::WorkFault::Both), Some("work_counter_inconsistent")),
        (C::Encoding, None), (C::Untranslated, None), (C::Scope, None)] {
        let (capture, observer, ordinary) = observed(mode, &raw);
        retained_tests_hooks::fail_next_serializer(fault);
        let (envelope, retained) = retained_w1(observer, ordinary, &capture);
        assert_eq!(retained.err(), Some(W1Fallback::Serializer(ReceiptFailure { check: fault, field_path: "cases[].run.invocation_after" })));
        check(&envelope, &noticed(detail), &format!("serializer {fault:?}"));
    }
    // Precommit validation: a corrupted receipt hash is the reader's G1 refusal.
    let (capture, observer, ordinary) = observed(mode, &raw);
    retained_tests_hooks::corrupt_next_precommit();
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::Precommit { gate: "G1", code: "RETAINED_PRECISION_RECEIPT_MISMATCH".into() }));
    check(&envelope, &noticed(None), "precommit");
    // Precommit binding: the reader validates against the actual invocation, so a
    // successor checked against another solver mode is refused.
    let (capture, observer, ordinary) = observed(mode, &raw);
    retained_tests_hooks::rebind_next_precommit_invocation();
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::Precommit { gate: "G8", code: "RETAINED_PRECISION_INVOCATION_MISMATCH".into() }));
    check(&envelope, &noticed(None), "precommit binding");
    // No W1 work ran: coexistence (an exact source-block publication) is returned as is.
    let (capture, observer, mut ordinary) = observed(mode, &raw);
    ordinary.source_block_recovery = Some(json!({"marker":"exact"}));
    let marked = serde_json::to_vec(&ordinary).unwrap();
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::Coexistence));
    check(&envelope, &marked, "coexistence");
    // No W1 work ran: the base already carries the notice's id, so its space is not
    // reserved and W1 does not start.
    let (capture, observer, mut ordinary) = observed(mode, &raw);
    ordinary.diagnostics.push(Diagnostic { id: "diagnostic:retained-precision:case:unavailable".into(), code: "X".into(),
        severity: "info".into(), message: "m".into(), source: None, affected_refs: Vec::new() });
    let marked = serde_json::to_vec(&ordinary).unwrap();
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::NoticeReservation));
    check(&envelope, &marked, "notice reservation");
    // No W1 work ran: an invocation outside D1.4 (C + 1 load cases).
    let over = beyond_load_cases(&raw, true);
    let (capture, observer, ordinary) = observed(mode, &over);
    let over_plain = serde_json::to_vec(&ordinary).unwrap();
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::Domain), "C + 1 cases: outside D1.4");
    check(&envelope, &over_plain, "C + 1 cases");
    // No W1 work ran: a combination D1.4 does not admit. B2-A widened D1.4 to combinations
    // (B2-C §9: z ≤ 2, C_eq ≤ 3, h ≤ 3, a range over ≤ 3 ids, ids disjoint), and B2-P's
    // re-check reads the same clauses, so a mechanics combination of four terms (h = 4,
    // C_eq = 2) is outside D1.4 (RV122 N-3).
    let mut combined = raw.clone();
    combined["model"]["combinations"] = json!([{"id":"combo","basis":"mechanics","terms":vec![json!({"load_case":"case","factor":0.25}); 4]}]);
    let (capture, observer, ordinary) = observed(mode, &combined);
    let combined_plain = serde_json::to_vec(&ordinary).unwrap();
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::Domain), "h = 4: outside D1.4");
    check(&envelope, &combined_plain, "combination outside D1.4");
    assert_eq!(retained_tests_hooks::armed(), retained_tests_hooks::Armed::default(), "every armed fault fired");
}

/// The notice text ROOT ruled, pinned in bytes against the product constant.
#[test]
fn u3_r2_notice_bytes_are_pinned() {
    assert_eq!(RETAINED_UNAVAILABLE_NOTICE, "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics.");
    assert!(!RETAINED_UNAVAILABLE_NOTICE.contains("receipt"), "no receipt reference");
    let longest = ["work_counter_range", "work_counter_inconsistent", "saturation_not_excluded", "publication_hash_range"].map(str::len).into_iter().max();
    assert_eq!(longest, Some(25), "the reserved message capacity covers every detail token");
    // RV85 T1: the constant the reservation uses is the longest detail
    // `receipt_encoding_detail` can return, over every typed check.
    use super::retained_wire::ReceiptCheck as C;
    let every = [C::Encoding, C::PublicationHashRange, C::WorkCounter(k::WorkFault::Overflow), C::WorkCounter(k::WorkFault::Inconsistent),
        C::WorkCounter(k::WorkFault::Both), C::WorkCounterRange, C::WorkCounterInconsistent, C::SaturationNotExcluded, C::Association, C::Scope, C::Untranslated];
    assert_eq!(every.iter().filter_map(|c| receipt_encoding_detail(*c)).map(str::len).max(), Some(RECEIPT_ENCODING_DETAIL_MAX));
    assert_eq!(RETAINED_UNAVAILABLE_NOTICE.len() + RECEIPT_ENCODING_REASON.len() + RECEIPT_ENCODING_DETAIL_MAX + 1, 196, "the reserved message bytes");
}

/// Control 1: with no permit both retained entries and the shared route are the
/// unchanged ordinary route, with no W1 result. G6: a profile is registered, so the
/// control uses the milestone made out of D1 (C + 1 load cases, D1.4), which no build
/// admits.
#[test]
fn u3_no_permit_entries_are_the_ordinary_route() {
    for mode in MODES {
        let raw = beyond_load_cases(&raw(), false);
        let plain = plain(mode, &raw);
        let direct = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
        assert_eq!(serde_json::to_vec(direct.envelope()).unwrap(), plain);
        assert!(direct.retained().is_none());
        assert!(direct.admission().is_some(), "G-A's census still runs");
        // R-1: without a permit the publication is always the ordinary one.
        assert!(direct.successor().is_none());
        match direct.into_publication() {
            RetainedPublication::Ordinary(envelope) => assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain),
            RetainedPublication::Successor(_) => panic!("a successor without a permit"),
        }
    }
}

/// R-1 (Proposal A): `successor()` and `into_publication()` name exactly one
/// publication. A completed transfer publishes the successor; every fallback, with
/// or without R-2's notice, publishes the ordinary owner it returned.
#[test]
fn u3_r1_carrier_names_exactly_one_publication() {
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = raw();
    let plain = plain(mode, &raw);
    let (capture, observer, ordinary) = observed(mode, &raw);
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    let expected = retained.as_ref().unwrap().value().clone();
    let output = RetainedPreviewOutput { envelope, admission: None, retained: Some(retained) };
    assert_eq!(output.successor(), Some(&expected));
    assert_eq!(serde_json::to_vec(output.envelope()).unwrap(), plain, "envelope() stays the ordinary base");
    match output.into_publication() {
        RetainedPublication::Successor(value) => assert_eq!(value, expected),
        RetainedPublication::Ordinary(_) => panic!("the transfer completed"),
    }
    let (capture, observer, ordinary) = observed(mode, &raw);
    retained_tests_hooks::corrupt_next_precommit();
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    let fallback = serde_json::to_vec(&envelope).unwrap();
    assert_eq!(fallback, with_notice(&plain, "case", None));
    let output = RetainedPreviewOutput { envelope, admission: None, retained: Some(retained) };
    assert_eq!(output.successor(), None);
    match output.into_publication() {
        RetainedPublication::Ordinary(envelope) => assert_eq!(serde_json::to_vec(&envelope).unwrap(), fallback),
        RetainedPublication::Successor(_) => panic!("a fallback published a successor"),
    }
}

/// STACK_PLAN §1: the reserved-stack runner returns the work's value, re-raises a
/// panic with its original payload, and reports a spawn failure without running.
#[test]
fn u3_reserved_stack_runner() {
    // A 3 MiB frame needs more than the 2 MiB default spawned-thread stack: it
    // runs only because the reservation is honoured.
    let observed_stack = on_reserved_stack(8 << 20, || {
        let frame = std::hint::black_box([7u8; 3 << 20]);
        frame.iter().map(|b| *b as usize).sum::<usize>()
    });
    assert_eq!(observed_stack, Some(7 * (3 << 20)));
    let borrowed = vec![1, 2, 3];
    assert_eq!(on_reserved_stack(1 << 20, || borrowed.iter().sum::<i32>()), Some(6), "borrows, no clone");
    let panicked = std::panic::catch_unwind(|| on_reserved_stack(1 << 20, || -> u8 { std::panic::panic_any(41usize) }));
    assert_eq!(panicked.err().and_then(|p| p.downcast::<usize>().ok()).map(|b| *b), Some(41), "original payload");
    let mut ran = false;
    assert_eq!(on_reserved_stack(1usize << 62, || ran = true), None, "spawn failure");
    assert!(!ran, "the work did not run");
}

/// ROOT's flag on grant 1: the armed fault hooks are thread-local, so on the
/// reserved-stack thread they would not fire and a fault test could pass vacuously.
/// `carry_test_hooks` (which the permitted dispatch wraps its work in) moves them
/// there; uncarried, the hazard is real.
#[test]
fn u3_test_hooks_follow_the_reserved_stack_thread() {
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = raw();
    let run_w1 = || {
        let (capture, observer, ordinary) = observed(mode, &raw);
        retained_w1(observer, ordinary, &capture).1.map(|_| ())
    };
    // Uncarried: armed here, it does not fire there, and stays armed here.
    retained_tests_hooks::corrupt_next_precommit();
    assert_eq!(on_reserved_stack(64 << 20, run_w1), Some(Ok(())), "the hazard: no fault fired on the other thread");
    assert_ne!(retained_tests_hooks::armed(), retained_tests_hooks::Armed::default());
    // Carried: it fires there, once, and is no longer armed here.
    assert_eq!(on_reserved_stack(64 << 20, carry_test_hooks(run_w1)),
        Some(Err(W1Fallback::Precommit { gate: "G1", code: "RETAINED_PRECISION_RECEIPT_MISMATCH".into() })));
    assert_eq!(retained_tests_hooks::armed(), retained_tests_hooks::Armed::default());
    // Each hook kind is carried.
    retained_tests_hooks::withdraw_next_native_source();
    assert_eq!(on_reserved_stack(64 << 20, carry_test_hooks(run_w1)), Some(Err(W1Fallback::Native)));
    retained_tests_hooks::fail_next_serializer(retained_wire::ReceiptCheck::Encoding);
    assert!(matches!(on_reserved_stack(64 << 20, carry_test_hooks(run_w1)), Some(Err(W1Fallback::Serializer(_)))));
    retained_tests_hooks::rebind_next_precommit_invocation();
    assert!(matches!(on_reserved_stack(64 << 20, carry_test_hooks(run_w1)), Some(Err(W1Fallback::Precommit { gate: "G8", .. }))));
    // The dense-scrutiny ceiling override is a setting: copied there and kept here.
    DENSE_SCRUTINY_CEILING_OVERRIDE.with(|c| c.set(Some(7)));
    assert_eq!(on_reserved_stack(1 << 20, carry_test_hooks(dense_scrutiny_ceiling_bytes)), Some(7));
    assert_eq!(on_reserved_stack(1 << 20, dense_scrutiny_ceiling_bytes), Some(DENSE_SCRUTINY_CEILING_BYTES), "uncarried: the constant");
    assert_eq!(DENSE_SCRUTINY_CEILING_OVERRIDE.with(|c| c.replace(None)), Some(7));
}

/// RV85 U2: faults that do not fire on the reserved-stack thread are handed back to
/// the caller, which re-arms them, rather than vanishing with the worker. That holds
/// after the work ran and when the work never ran (a spawn failure).
#[test]
fn u3_unfired_hooks_come_back_across_the_hop() {
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = raw();
    // Preparation refuses, so an armed precommit fault never fires there.
    let preparation_refuses = || {
        let (capture, mut observer, ordinary) = observed(mode, &raw);
        observer.facts[0].diameter = 0.0;
        retained_w1(observer, ordinary, &capture).1.map(|_| ())
    };
    retained_tests_hooks::corrupt_next_precommit();
    retained_tests_hooks::fail_next_serializer(retained_wire::ReceiptCheck::Encoding);
    assert_eq!(on_reserved_stack(64 << 20, carry_test_hooks(preparation_refuses)), Some(Err(W1Fallback::Preparation)));
    assert!(retained_tests_hooks::armed_names().is_empty(), "in transit until reclaimed");
    retained_tests_hooks::reclaim_handed_back();
    assert_eq!(retained_tests_hooks::armed_names(), ["precommit", "serializer"], "the unfired faults are visible to the caller");
    retained_tests_hooks::disarm();
    // A fault that does fire there does not come back.
    retained_tests_hooks::withdraw_next_native_source();
    let run_w1 = || {
        let (capture, observer, ordinary) = observed(mode, &raw);
        retained_w1(observer, ordinary, &capture).1.map(|_| ())
    };
    assert_eq!(on_reserved_stack(64 << 20, carry_test_hooks(run_w1)), Some(Err(W1Fallback::Native)));
    retained_tests_hooks::reclaim_handed_back();
    assert!(retained_tests_hooks::armed_names().is_empty(), "the native fault fired and was consumed");
    // A spawn failure: the unrun work drops here and hands its faults straight back.
    retained_tests_hooks::rebind_next_precommit_invocation();
    let mut ran = false;
    assert_eq!(on_reserved_stack(1usize << 62, carry_test_hooks(|| ran = true)), None);
    assert!(!ran);
    retained_tests_hooks::reclaim_handed_back();
    assert_eq!(retained_tests_hooks::armed_names(), ["rebind"]);
    retained_tests_hooks::disarm();
}

/// RV82 N9 (single-parse custody): each invocation is parsed exactly once. The
/// typed request that runs and the captured invocation the serializer binds (S2's
/// digest) are the two halves of that one `CapturedInvocation::parse`: the capture
/// is not `Clone`, `parse` is its only constructor, the dispatch is the only
/// production parse site, and the permitted path only moves or borrows its halves.
#[test]
fn u3_n9_single_parse_custody() {
    const PARSE: &str = "CapturedInvocation::parse(";
    let lib = include_str!("lib.rs");
    let section = |start: &str, end: &str| {
        let suffix = &lib[lib.find(start).unwrap()..];
        &suffix[..suffix.find(end).unwrap()]
    };
    assert_eq!(lib.matches(PARSE).count(), 1, "lib.rs has one parse site");
    let dispatch = section("fn run_linear_static_preview_value_dispatch(", "fn ordinary_dispatch(");
    assert!(dispatch.contains("let (request, capture) = source_receipt::CapturedInvocation::parse(actual_request, solver_mode)"));
    assert!(dispatch.contains("Some(Ok((permit, report))) => return permitted_dispatch(permit, report, request, capture, solver_mode),"));
    assert!(dispatch.contains("ordinary_dispatch(request, &capture, solver_mode, admission, None)"));
    // RV85 N7: exactly one call each of the permitted path's functions in lib.rs (the
    // definition plus one call; `carry_test_hooks`' generic definitions do not match),
    // and the dispatch's call sits in the permit arm.
    for (function, calls) in [("permitted_dispatch(", 2), ("permitted_run(", 2), ("retained_w1(", 2), ("carry_test_hooks(", 1)] {
        assert_eq!(lib.matches(function).count(), calls, "{function}");
    }
    assert_eq!(dispatch.matches("permitted_dispatch(").count(), 1, "the dispatch's one call");
    // The permitted path: the parse's two halves, moved or borrowed, never re-derived.
    let permitted = section("fn permitted_dispatch(", "pub(crate) mod retained_tests_hooks");
    for forbidden in ["parse(", "CapturedInvocation {", "capture.clone()", "request.clone()", "from_value(", "from_str(", "from_slice(",
        "from_reader(", "deserialize(", "Deserialize", "LinearStaticPreviewRequest {"] {
        assert!(!permitted.contains(forbidden), "{forbidden}");
    }
    // The raw custody is read in exactly two places: the attempted case's id and the
    // precommit invocation.
    assert_eq!(permitted.matches("borrowed_raw()").count(), 2, "borrowed_raw() reads");
    assert!(permitted.contains("let model = &capture.borrowed_raw()[\"model\"];"));
    assert!(permitted.contains("serde_json::json!({\"request\": capture.borrowed_raw(), \"solver_mode\": capture.mode().as_str()})"));
    for required in [
        "pending.take().map(|request| permitted_run(permit, report, request, captured, solver_mode))",
        "(None, Some(request)) => ordinary_dispatch(request, &capture, solver_mode, Some(report), Some(Err(W1Fallback::StackReservation))),",
        "run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer))",
        "Some(Ok(())) => retained_w1(observer, ordinary, capture),",
        // B1 SP (T-11): the n-case serializer over the transaction replaces the one-case
        // `serialize_frozen`; the call still takes the parse's borrowed half.
        "retained_wire::serialize_cases(&mut prepared, &staged, capture)",
        // ROOT's flag: the permitted work carries the armed test hooks, and (RV85 U2)
        // the caller re-arms the unfired ones after the hop.
        "on_reserved_stack(bytes, carry_test_hooks(move || {",
        "    }));\n    #[cfg(test)]\n    retained_tests_hooks::reclaim_handed_back();\n    match (ran.flatten(), slot) {",
    ] {
        assert!(permitted.contains(required), "{required}");
    }
    // No other production parse site, and no second custody constructor.
    for (name, text) in [
        ("retained_product.rs", include_str!("retained_product.rs")),
        ("retained_wire.rs", include_str!("retained_wire.rs")),
        ("retained_receipt.rs", include_str!("retained_receipt.rs")),
    ] {
        assert!(!text.contains(PARSE) && !text.contains("impl Clone for CapturedInvocation"), "{name}");
    }
    let memory = include_str!("retained_memory.rs");
    assert!(!memory[..memory.find("#[cfg(test)]\npub(super) mod tests {").unwrap()].contains(PARSE), "retained_memory.rs");
    let receipt = include_str!("source_receipt.rs");
    assert!(receipt.contains("#[derive(Debug)]\npub(super) struct CapturedInvocation {"), "not Clone");
    assert_eq!(receipt.matches("impl CapturedInvocation {").count(), 1);
    assert!(!receipt.contains("for CapturedInvocation") && !lib.contains("for CapturedInvocation"), "no trait constructor or Clone");
    let block = &receipt[receipt.find("\nimpl CapturedInvocation {\n").unwrap()..];
    let block = &block[..block.find("\n}\n").unwrap()];
    assert_eq!(block.matches("Self {").count(), 1, "parse's literal is the only constructor");
    assert_eq!(block.matches("-> Result<(LinearStaticPreviewRequest, Self), ReceiptError>").count(), 1);
    assert!(!block.contains("-> Self"));
}

/// RV85 S3 and N1 (structural; the permitted path needs a permit, so its behaviour
/// is in the stub evidence). S3: `admit` yields the report with the permit and every
/// permitted output carries it. N1: G-C is consulted only after coexistence and G-B's
/// outcome, in I51 COMPOSITION §2's order, each recorded as its own cause.
#[test]
fn u3_permitted_outputs_keep_the_report_and_gate_order() {
    let lib = include_str!("lib.rs");
    let permitted = &lib[lib.find("fn permitted_dispatch(").unwrap()..lib.find("pub(crate) const RETAINED_UNAVAILABLE_NOTICE").unwrap()];
    assert!(!permitted.contains("admission: None"), "S3: no permitted output drops the report");
    assert_eq!(permitted.matches("Some(report)").count(), 3, "S3: StackReservation, Domain and the W1 output");
    assert!(include_str!("retained_memory.rs").contains(") -> Result<(CapturePermit, RetainedAdmissionReport), RetainedAdmissionReport> {"));
    assert!(include_str!("retained_memory.rs").contains("admission(report).map(|permit| (permit, report))"));
    let run = &permitted[permitted.find("fn permitted_run(").unwrap()..];
    let at = |text: &str| run.find(text).unwrap_or_else(|| panic!("{text}"));
    let (exact, late, complete) = (at("if ordinary.source_block_recovery.is_some() {"), at("observer.late_refusal().cloned()"), at("permit.check_complete("));
    assert!(exact < late && late < complete, "N1: exact selection, then G-B's outcome, then G-C");
    assert!(run[exact..late].contains("Err(W1Fallback::Coexistence)") && run[late..complete].contains("Err(W1Fallback::LateGate(refusal))"));
}

/// U3 grant 1b (ROOT's flag): the capture permit is linear. It derives neither
/// `Clone` nor `Copy` and has no `Clone`/`Copy` impl; the facade moves it onto the
/// reserved-stack thread and into the observer, which alone uses it (G-B, then
/// G-C through `permit()`), and nothing re-reads a moved permit.
#[test]
fn u3_capture_permit_is_linear() {
    let memory = include_str!("retained_memory.rs");
    let at = memory.find("pub(super) struct CapturePermit {").unwrap();
    let attributes = &memory[memory[..at].rfind("\n\n").unwrap()..at];
    assert!(!attributes.contains("derive"), "no derive on CapturePermit: {attributes}");
    for text in [memory, include_str!("lib.rs"), include_str!("retained_product.rs")] {
        assert!(!text.contains("impl Copy for CapturePermit") && !text.contains("impl Clone for CapturePermit"), "no Clone or Copy impl");
    }
    let lib = include_str!("lib.rs");
    let run = &lib[lib.find("fn permitted_run(").unwrap()..lib.find("pub(crate) const RETAINED_UNAVAILABLE_NOTICE").unwrap()];
    // B3b-P (B3-D P-1): the observer is constructed on the invocation's route, decided once just
    // before; the permit still moves into it, unchanged.
    assert!(run.contains("ProductCapture::permitted_probe_on(permit, route);"), "the permit moves into the observer");
    assert!(run.contains("observer.permit().map(|permit| permit.check_complete("), "G-C borrows the observer's permit");
    assert!(!run.contains("permit.check_complete(&retained_memory::CompleteFacts { ordinary: &ordinary }) {"), "no use of a moved permit");
    let product = include_str!("retained_product.rs");
    assert!(product.contains("if let Some(permit)=self.permit.as_ref() {"), "G-B borrows the observer's permit");
}

/// R-2's condition (ROOT, NUM efde9ca2d1): result_export's base preview-physics-1
/// readers accept a base publication carrying one info RETAINED_PRECISION_UNAVAILABLE
/// diagnostic as N1 specifies (case refs only, fixed text, no receipt reference),
/// and its classification and standing are unchanged: on the Sensitive milestone and
/// on an exportable (checks-passed) solve. The negative control shows the same readers
/// do refuse a malformed notice. `I61_R2_OUT` writes the bytes for the evidence lanes.
///
/// B1 SP (DESIGN_v2 T-12; RV105 N-4): W-C2's base carries several N1 notices, one per case in
/// A (case-a's, then case-c's), plain, or with C1:68's detail on the selected case-a's notice
/// only. The same readers accept it with the same contract and standing; the bytes are written
/// out (`w_c2_noticed_*`) for SR-PY and SR-TS.
#[test]
fn u3_r2_base_readers_accept_the_unavailable_notice() {
    use open_pipe_stress_result_export::semantic_contract as sc;
    const PLAIN: &str = "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics.";
    let notice = |case: &str, message: &str, refs: Value| json!({"id":format!("diagnostic:retained-precision:{case}:unavailable"),
        "code":"RETAINED_PRECISION_UNAVAILABLE","severity":"info","message":message,"source":"core/product_physics","affected_refs":refs});
    let mut exportable = json!({"model": serde_json::from_str::<Value>(include_str!("../tests/fixtures/preview_physics_invented_model.json")).unwrap(), "materials": []});
    for case in exportable["model"]["load_cases"].as_array_mut().unwrap() {
        for load in case["primitive_loads"].as_array_mut().unwrap() {
            if load["category"] == "pressure" || load["dimension"] == "pressure" { load["magnitude"]["value"] = json!(0.0); }
        }
    }
    let mut standings = std::collections::BTreeSet::new();
    for (label, raw) in [("milestone", raw()), ("exportable", exportable), ("w_c2", w_c2())] {
        let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap().to_owned();
        // T-12: the notices of A, in request order (W-C2: A and C; otherwise the one case).
        let noticed_cases: Vec<String> = if label == "w_c2" { vec!["case-a".into(), "case-c".into()] } else { vec![case.clone()] };
        let bases: Vec<Value> = raw["model"]["load_cases"].as_array().unwrap().iter().map(|c| json!({"ref_type":"load_case","ref_id":c["id"]})).collect();
        for mode in MODES {
            let base = serde_json::to_value(run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
            assert_eq!(base["producer"]["semantic_contract_id"], json!("openpipestress.result_semantics/0.3.0/preview-physics-1"), "{label}");
            assert!(sc::for_source(&base).is_ok(), "precondition: the base publication is admitted");
            let receipt = format!("{PLAIN} Reason: receipt_encoding; detail: work_counter_inconsistent.");
            for message in [PLAIN, receipt.as_str()] {
                let mut noticed = base.clone();
                for (k, case) in noticed_cases.iter().enumerate() {
                    // The detail goes only on the first (selected) case's notice.
                    let text = if k == 0 { message } else { PLAIN };
                    noticed["diagnostics"].as_array_mut().unwrap().push(notice(case, text, json!([case])));
                }
                assert_eq!(sc::for_source(&noticed), sc::for_source(&base), "{label} {mode:?}: admitted with the same contract");
                assert_eq!(sc::standing_reason(&noticed), sc::standing_reason(&base));
                let standing = sc::numerical_use_standing_with_context(&noticed, &bases, Some(&invocation));
                assert_eq!(standing, sc::numerical_use_standing_with_context(&base, &bases, Some(&invocation)));
                assert_eq!(sc::numerical_use_standing(&noticed, &bases), sc::numerical_use_standing(&base, &bases));
                standings.insert(standing);
                println!("I61_R2_RUST {label} {} for_source=ok standing={standing}", mode.as_str());
                if let Ok(dir) = std::env::var("I61_R2_OUT") {
                    let kind = if message == PLAIN { "plain" } else { "receipt" };
                    let dir = std::path::Path::new(&dir);
                    std::fs::write(dir.join(format!("{label}_noticed_{kind}_{}.json", mode.as_str())), serde_json::to_vec(&noticed).unwrap()).unwrap();
                    std::fs::write(dir.join(format!("{label}_base_{}.json", mode.as_str())), serde_json::to_vec(&base).unwrap()).unwrap();
                    std::fs::write(dir.join(format!("{label}_request.json")), serde_json::to_vec(&raw).unwrap()).unwrap();
                }
            }
            // Negative control: a notice naming a result row that does not exist is refused.
            let mut dangling = base.clone();
            dangling["diagnostics"].as_array_mut().unwrap().push(notice(&case, PLAIN, json!(["result:not-a-row"])));
            assert!(sc::for_source(&dangling).unwrap_err().contains("DANGLING_RESULT_REF"));
        }
    }
    println!("I61_R2_RUST standings={standings:?}");
}

// ---- U3 grant 2: the permitted path on the actual Direct entry --------------------
//
// Decision 7 holds: no test permit exists. `admit` grants a permit here only because this
// build matches the registered profile (0c7827b6ad, M = 4,026,531,840 B). In any other
// build (Stale) each test below asserts the unchanged ordinary route instead, so the same
// suite is the control in both. The permitted work runs on the reserved-stack thread; the
// armed faults and the run tally travel there with it (`carry_test_hooks`).

use super::retained_tests_hooks::{self as hooks, Counts};

/// The registered dev/test identity (R/I65/u4_g6_01/QUALIFICATION.md; the admission test's).
const REGISTERED_IDENTITY: &str = "v1;rustc.release=1.97.1;rustc.commit=8bab26f4f68e0e26f0bb7960be334d5b520ea452;rustc.host=aarch64-apple-darwin;rustc.llvm=22.1.6;target=aarch64-apple-darwin;target.arch=aarch64;target.pointer_width=64;target.endian=little;target.os=macos;target.env=;panic=unwind;profile=debug;opt_level=0;debug_assertions=true;rustflags=;pkg=open_pipe_stress_product_physics@0.2.0";
fn registered() -> bool {
    option_env!("OPS_RETAINED_BUILD_IDENTITY") == Some(REGISTERED_IDENTITY)
}
/// One actual Direct invocation, counted: its output and the runs and G-C consultations.
fn direct(raw: &Value, mode: PreviewSolverMode) -> (RetainedPreviewOutput, Counts) {
    let raw = raw.clone();
    let (output, counts) = hooks::counted(move || run_linear_static_preview_value_with_retained_direct(raw, mode).unwrap());
    let profile = output.admission().expect("G-A ran").profile;
    assert_eq!(profile, if registered() { ProfileStatus::Registered } else { ProfileStatus::Stale }, "the build's status");
    (output, counts)
}
/// The one publication's bytes (R-1).
fn published(output: RetainedPreviewOutput) -> Vec<u8> {
    match output.into_publication() {
        RetainedPublication::Successor(value) => serde_json::to_vec(&value).unwrap(),
        RetainedPublication::Ordinary(envelope) => serde_json::to_vec(&envelope).unwrap(),
    }
}
const ONE_RUN: Counts = Counts { runs: 1, complete_gates: 0 };
const ONE_RUN_THROUGH_G_C: Counts = Counts { runs: 1, complete_gates: 1 };
fn notices(bytes: &[u8]) -> usize {
    let value: Value = serde_json::from_slice(bytes).unwrap();
    value["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == "RETAINED_PRECISION_UNAVAILABLE").count()
}

/// Deliverables 1 and 2 (B′, RV82 N3) and B-1/S-7: in the registered build the milestone's
/// actual Direct entry publishes U1's pinned successor in both modes, from exactly one
/// ordinary run; the ordinary envelope beside it is the plain run's bytes. In any other
/// build it publishes exactly the plain bytes, also from one run.
#[test]
fn u3g2_direct_entry_publishes_the_pinned_successor() {
    let out = std::env::var("I61_U3G2_OUT").ok().map(std::path::PathBuf::from);
    for (mode, (name, file_sha, receipt_sha)) in MODES.into_iter().zip(PINNED) {
        let raw = raw();
        let plain = plain(mode, &raw);
        let (output, counts) = direct(&raw, mode);
        assert_eq!(serde_json::to_vec(output.envelope()).unwrap(), plain, "{name}: B′, the ordinary envelope is the plain run");
        if !registered() {
            assert!(output.retained().is_none() && output.successor().is_none(), "{name}: no permit, no W1");
            assert_eq!(counts, ONE_RUN, "{name}");
            assert_eq!(published(output), plain, "{name}: the ordinary route");
            continue;
        }
        assert_eq!(output.admission().unwrap().law().refusal, None, "{name}: admitted");
        assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{name}: B-1, one ordinary run, then G-C once");
        let successor = match output.retained() {
            Some(Ok(successor)) => successor.value().clone(),
            other => panic!("{name}: {other:?}"),
        };
        assert_eq!(output.successor(), Some(&successor));
        let text = serde_json::to_string_pretty(&json!({"id":format!("u1_milestone_{name}"),"source":successor,
            "invocation":{"request":raw,"solver_mode":mode.as_str()}})).unwrap();
        assert_eq!((sha(text.as_bytes()).as_str(), successor["retained_precision"]["receipt_sha256"].as_str()), (file_sha, Some(receipt_sha)),
            "{name}: U1's pinned successor bytes");
        assert_eq!(published(output), serde_json::to_vec(&successor).unwrap(), "{name}: the one publication is the successor");
        if let Some(dir) = &out {
            std::fs::write(dir.join(format!("u3g2_successor_{name}.json")), &text).unwrap();
        }
    }
}

/// Deliverable 1: every W1 fallback on the actual Direct entry (preparation, native,
/// candidate, staging, serializer, precommit) publishes the ordinary bytes plus exactly
/// one N1 notice, from one ordinary run that reached G-C once. Without a permit no fault
/// fires (it stays armed on the caller) and the bytes are the plain ones.
#[test]
fn u3g2_direct_entry_w1_fallbacks_append_one_notice() {
    use super::retained_receipt::TraceFault as F;
    use super::retained_wire::{ReceiptCheck as C, ReceiptFailure};
    let serializer = |check| W1Fallback::Serializer(ReceiptFailure { check, field_path: "cases[].run.invocation_after" });
    let faults: Vec<(&str, &str, fn(), W1Fallback, Option<&str>)> = vec![
        ("preparation", "preparation", hooks::fail_next_preparation, W1Fallback::Preparation, None),
        ("native", "native", hooks::withdraw_next_native_source, W1Fallback::Native, None),
        ("candidate maxima", "candidate", || hooks::fault_next_candidate(F::Maxima), W1Fallback::Candidate, None),
        ("candidate values", "candidate", || hooks::fault_next_candidate(F::ValuesCompletion), W1Fallback::Candidate, None),
        ("staging", "staging", hooks::break_next_staging, W1Fallback::Staging(rp::StagingFault("pipe_stress_extrema[]")), None),
        ("serializer receipt encoding", "serializer", || hooks::fail_next_serializer(C::WorkCounterInconsistent),
            serializer(C::WorkCounterInconsistent), Some("work_counter_inconsistent")),
        ("serializer association", "serializer", || hooks::fail_next_serializer(C::Association), serializer(C::Association), None),
        ("precommit", "precommit", hooks::corrupt_next_precommit,
            W1Fallback::Precommit { gate: "G1", code: "RETAINED_PRECISION_RECEIPT_MISMATCH".into() }, None),
        ("precommit binding", "rebind", hooks::rebind_next_precommit_invocation,
            W1Fallback::Precommit { gate: "G8", code: "RETAINED_PRECISION_INVOCATION_MISMATCH".into() }, None),
    ];
    for mode in MODES {
        let raw = raw();
        let plain = plain(mode, &raw);
        for (label, armed, arm, cause, detail) in &faults {
            arm();
            let (output, counts) = direct(&raw, mode);
            if !registered() {
                assert!(output.retained().is_none(), "{label}: no W1");
                assert_eq!(counts, ONE_RUN, "{label}");
                assert_eq!(published(output), plain, "{label} {mode:?}");
                assert_eq!(hooks::armed_names(), [*armed], "{label}: never fired");
                hooks::disarm();
                continue;
            }
            assert_eq!(output.retained().and_then(|r| r.as_ref().err()), Some(cause), "{label} {mode:?}");
            assert!(output.successor().is_none(), "{label}");
            assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{label} {mode:?}: B-1");
            let bytes = published(output);
            assert_eq!(notices(&bytes), 1, "{label} {mode:?}: exactly one N1 notice");
            assert_eq!(String::from_utf8(bytes).unwrap(), String::from_utf8(with_notice(&plain, "case", *detail)).unwrap(),
                "{label} {mode:?}: the ordinary bytes, then the notice");
            assert!(hooks::armed_names().is_empty(), "{label}: fired on the reserved-stack thread, not handed back");
        }
    }
}

/// Deliverable 1: every refusal before W1 work (G-A, G-C including OrdinarySolveNotAttempted,
/// the reserved stack, coexistence; G-B in the next test) publishes exactly the ordinary
/// bytes, with no notice, from one ordinary run.
#[test]
fn u3g2_direct_entry_no_w1_refusals_keep_exact_bytes() {
    use super::retained_memory::{D1Clause, PhaseFact, PhaseGate};
    for mode in MODES {
        let milestone = raw();
        let plain_milestone = plain(mode, &milestone);
        // G-A: outside D1 (D1.4: C + 1 load cases; D1.9: C_eq + 1 case-equivalents, as three
        // combinations, since B2-A admits z ≤ 2 with c + z ≤ 3; D1.3: another namespace).
        let over = beyond_load_cases(&milestone, true);
        let mut combined = milestone.clone();
        combined["model"]["combinations"] = json!((1..=3).map(|k| json!({"id":format!("combo-{k}"),"basis":"mechanics","terms":[{"load_case":"case","factor":1.0}]})).collect::<Vec<_>>());
        let mut namespace = milestone.clone();
        namespace["model"]["schema_version"] = json!("0.3.0");
        for (label, raw, clause) in [("C + 1 cases", &over, D1Clause::Invocation), ("C_eq + 1 (three combinations)", &combined, D1Clause::Caps),
            ("namespace", &namespace, D1Clause::Namespace)] {
            let plain = plain(mode, raw);
            let (output, counts) = direct(raw, mode);
            let refusal = output.admission().unwrap().law().refusal.expect("G-A refuses");
            assert_eq!(refusal.clause(), Some(if registered() { clause } else { D1Clause::Build }), "{label} {mode:?}");
            assert!(output.retained().is_none(), "{label}");
            assert_eq!(counts, ONE_RUN, "{label} {mode:?}");
            assert_eq!(published(output), plain, "{label} {mode:?}");
        }
        // G-A: Headless is refused at D1.0 (D-2).
        let invocation = json!({"request": milestone.clone(), "solver_mode": mode.as_str()});
        let request_id = String::from("i61-u3g2");
        let headless = hooks::counted(|| run_linear_static_preview_value_with_retained_headless(milestone.clone(), mode,
            RetainedHeadlessContext::from_borrowed_roots(&milestone, &invocation, &request_id)).unwrap());
        assert_eq!(headless.0.admission().unwrap().law().refusal.and_then(|r| r.clause()), Some(D1Clause::Caller));
        assert!(headless.0.retained().is_none());
        assert_eq!(headless.1, ONE_RUN);
        assert_eq!(published(headless.0), plain_milestone, "Headless");
        // G-C: a D1 request whose ordinary route returned before attempting the solve
        // (ROOT's G6 ruling 2(a)): declined, no notice.
        let mut unattempted = milestone.clone();
        unattempted["model"]["document_kind"] = json!("invalid-kind");
        let plain_unattempted = plain(mode, &unattempted);
        let (output, counts) = direct(&unattempted, mode);
        if registered() {
            match output.retained() {
                Some(Err(W1Fallback::CompleteGate(r))) => assert_eq!((r.gate, r.fact, r.observed, r.cap), (PhaseGate::Complete, PhaseFact::OrdinarySolveNotAttempted, 1, 0)),
                other => panic!("{mode:?}: {other:?}"),
            }
            assert_eq!(counts, ONE_RUN_THROUGH_G_C);
        } else {
            assert_eq!(counts, ONE_RUN);
        }
        assert_eq!(published(output), plain_unattempted, "unattempted solve {mode:?}");
        // G-C refuses an admitted milestone run (its observation record above the bound).
        hooks::fail_next_complete_gate();
        let (output, counts) = direct(&milestone, mode);
        if registered() {
            match output.retained() {
                Some(Err(W1Fallback::CompleteGate(r))) => assert_eq!((r.gate, r.fact), (PhaseGate::Complete, PhaseFact::ObservationBytes)),
                other => panic!("{mode:?}: {other:?}"),
            }
            assert_eq!(counts, ONE_RUN_THROUGH_G_C);
        } else {
            assert_eq!(counts, ONE_RUN);
            assert_eq!(hooks::armed_names(), ["complete_gate"]);
            hooks::disarm();
        }
        assert_eq!(published(output), plain_milestone, "G-C {mode:?}");
        // The reserved stack cannot be spawned: the ordinary route on the caller's thread,
        // with the report and the private cause (STACK_PLAN §1); faults come back (RV85 U2).
        super::retained_memory::RESERVED_STACK_OVERRIDE.with(|c| c.set(Some(1usize << 62)));
        hooks::corrupt_next_precommit();
        let (output, counts) = direct(&milestone, mode);
        super::retained_memory::RESERVED_STACK_OVERRIDE.with(|c| c.set(None));
        assert_eq!(hooks::armed_names(), ["precommit"], "the unrun work's fault is back on the caller");
        hooks::disarm();
        if registered() {
            assert_eq!(output.retained().and_then(|r| r.as_ref().err()), Some(&W1Fallback::StackReservation), "{mode:?}");
        } else {
            assert!(output.retained().is_none());
        }
        assert_eq!(counts, ONE_RUN, "stack {mode:?}");
        assert_eq!(published(output), plain_milestone, "stack {mode:?}");
        // Coexistence (D-15): an admitted source-block request whose ordinary run settled by
        // exact selection is published as it is, before G-B's outcome and G-C are consulted
        // (RV85 N1).
        let exact: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.request.json")).unwrap();
        let plain_exact = plain(mode, &exact);
        assert!(serde_json::from_slice::<Value>(&plain_exact).unwrap()["source_block_recovery"].is_object(), "exact blocks selected");
        let (output, counts) = direct(&exact, mode);
        if registered() {
            assert_eq!(output.admission().unwrap().law().refusal, None, "admitted");
            assert_eq!(output.retained().and_then(|r| r.as_ref().err()), Some(&W1Fallback::Coexistence), "{mode:?}");
        }
        assert_eq!(counts, ONE_RUN, "coexistence {mode:?}: G-C not consulted");
        assert_eq!(published(output), plain_exact, "coexistence {mode:?}");
    }
}

/// RV85 U4 and N2 (SV18): on the actual Direct entry, a G-B refusal is final. Its cause is
/// recorded (LateGate, the actual `check_late`'s fact), the exact ordinary bytes are
/// published, and G-C is never consulted, although G-C would also refuse this run.
#[test]
fn u3g2_late_gate_refusal_is_final_and_g_c_is_not_consulted() {
    use super::retained_memory::{PhaseFact, PhaseGate};
    for mode in MODES {
        let raw = raw();
        let plain = plain(mode, &raw);
        hooks::fail_next_late_gate();
        let (output, counts) = direct(&raw, mode);
        if registered() {
            match output.retained() {
                Some(Err(W1Fallback::LateGate(r))) => assert_eq!((r.gate, r.fact), (PhaseGate::Late, PhaseFact::LateObservationBytes), "{mode:?}"),
                other => panic!("{mode:?}: {other:?}"),
            }
            assert_eq!(counts, ONE_RUN, "{mode:?}: G-C is not consulted after G-B refused");
            assert!(hooks::armed_names().is_empty());
        } else {
            assert_eq!(counts, ONE_RUN);
            assert_eq!(hooks::armed_names(), ["late_gate"], "no permit, no G-B");
            hooks::disarm();
        }
        assert_eq!(published(output), plain, "{mode:?}: exact ordinary bytes, no notice");
    }
}

/// B′ and B-1/S-7 on the no-permit path: the shared value route and every refused retained
/// entry run the ordinary route exactly once and never reach G-C, and the no-permit
/// dispatch makes no copy of the request or its custody (the parse's halves are moved
/// or borrowed).
#[test]
fn u3g2_no_permit_path_runs_once_without_a_copy() {
    // Out of D1 (D1.4): C + 1 load cases.
    let over = beyond_load_cases(&raw(), false);
    for mode in MODES {
        let shared = hooks::counted(|| run_linear_static_preview_value_with_mode(over.clone(), mode).unwrap());
        assert_eq!(shared.1, ONE_RUN, "{mode:?}: the shared value route");
        let (output, counts) = direct(&over, mode);
        assert!(output.retained().is_none() && output.admission().unwrap().law().refusal.is_some());
        assert_eq!(counts, ONE_RUN, "{mode:?}: the refused Direct entry");
        assert_eq!(serde_json::to_vec(output.envelope()).unwrap(), serde_json::to_vec(&shared.0).unwrap());
    }
    let lib = include_str!("lib.rs");
    let section = |start: &str, end: &str| {
        let suffix = &lib[lib.find(start).unwrap()..];
        &suffix[..suffix.find(end).unwrap()]
    };
    let dispatch = section("fn run_linear_static_preview_value_dispatch(", "fn source_finalization_failed(");
    assert!(!dispatch.contains(".clone()") && !dispatch.contains("to_owned()") && !dispatch.contains("to_vec()"), "no copy on the dispatch");
    assert_eq!(dispatch.matches("run_linear_static_preview_captured(").count(), 1, "one ordinary run call");
    assert!(dispatch.contains("retained_memory::admit(&capture, &request, entry)"), "G-A borrows the parse's halves");
}

/// D-U6-5: U6's two carrier fixtures are byte-identical copies of PP's pinned successors.
/// In the registered build each is compared byte for byte with the successor document the
/// actual Direct entry publishes; in any other build (no permit), with the private driver's
/// successor document, which U1 pinned. Either way the fixture carries U1's pinned hashes.
#[test]
fn u3g2_d_u6_5_carrier_fixtures_are_the_live_successors() {
    const CARRIERS: [&str; 2] = [
        include_str!("../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"),
        include_str!("../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json"),
    ];
    for ((mode, (name, file_sha, receipt_sha)), carrier) in MODES.into_iter().zip(PINNED).zip(CARRIERS) {
        let raw = raw();
        let successor = if registered() {
            let (output, counts) = direct(&raw, mode);
            assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{name}");
            match output.into_publication() {
                RetainedPublication::Successor(value) => value,
                RetainedPublication::Ordinary(_) => panic!("{name}: the registered Direct entry did not publish its successor"),
            }
        } else {
            let (capture, observer, ordinary) = observed(mode, &raw);
            retained_w1(observer, ordinary, &capture).1.unwrap_or_else(|f| panic!("{name}: {f:?}")).value().clone()
        };
        let document = serde_json::to_string_pretty(&json!({"id":format!("u1_milestone_{name}"),"source":successor,
            "invocation":{"request":raw,"solver_mode":mode.as_str()}})).unwrap();
        assert!(document == carrier, "{name}: U6's carrier fixture is the live successor document, byte for byte");
        assert_eq!((sha(carrier.as_bytes()).as_str(), successor["retained_precision"]["receipt_sha256"].as_str()), (file_sha, Some(receipt_sha)),
            "{name}: U1's pinned hashes");
    }
}

// ---- U8 (I68): producer-solved witnesses from real D1 inputs ------------------------------
//
// RR "I61's U8 plan ruled…" and "I68's probe verified…" (R/I68/u8_probe_01). Real inputs, no
// hooks: RV93 N-5's Candidate and Preparation fallbacks, W-C1's Native fallback (since B1, W-C2's
// case C; its kernel reason, Unresolved(Ceiling), is recorded by the B1-0 probe, R/I81/b1_probe_01,
// and not asserted here), and the L = 0 base, whose successor is pinned and copied to the corpus
// fixtures (D-U6-5). B1 (T-4): two-body case B, W-C1's input before B1, is a `NoTriggeredCase` pin.

/// RV93 N-5's Candidate input (RV93's probe, `zz_rv93_input_fallbacks`): the milestone with only
/// its first load.
fn u8_first_load_only() -> Value {
    let mut raw = raw();
    let first = raw["model"]["load_cases"][0]["primitive_loads"][0].clone();
    raw["model"]["load_cases"][0]["primitive_loads"] = json!([first]);
    raw
}
/// RV93's Preparation sibling: support 1's spring stiffness set to 1e-300. The ordinary run does
/// not solve (MODEL_INCOMPLETE), so preparation's MECHANICS_SOLVED requirement refuses.
fn u8_tiny_spring() -> Value {
    let mut raw = raw();
    raw["model"]["supports"][1]["stiffness"]["value"]["value"] = json!(1e-300);
    raw
}
/// W-C2's two-body model (PLAN §1.3) with case A's loads: the milestone body (body 0) and its
/// three moments, plus PHYS-R4's cantilever as body 1 (W6's body, `w6_input()` in
/// retained_memory_witness_tests.rs, moved to x = 5..6 so that no node coincides with N0), unloaded.
fn u8_two_body_case_a() -> Value {
    let mut raw = raw();
    let p = "invented_t3_g5_witness_input_no_library_data";
    let model = &mut raw["model"];
    model["nodes"].as_array_mut().unwrap().extend([
        json!({"id": "node:section-a", "position": {"x": 5.0, "y": 0.0, "z": 0.0}, "provenance": p}),
        json!({"id": "node:section-b", "position": {"x": 6.0, "y": 0.0, "z": 0.0}, "provenance": p}),
    ]);
    model["pipe_segments"].as_array_mut().unwrap().push(json!({"id": "pipe:source-section", "from": "node:section-a", "to": "node:section-b",
        "material": "material:section", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
        "section": {"outside_diameter": {"value": 4e-77, "unit": "m"}, "wall_thickness": {"value": 1e-77, "unit": "m"}}, "provenance": p}));
    model["materials"].as_array_mut().unwrap().push(json!({"id": "material:section", "elastic_modulus": {"value": 1.0, "unit": "Pa"},
        "shear_modulus": {"value": 0.4545, "unit": "Pa"}, "provenance": p}));
    model["supports"].as_array_mut().unwrap().push(json!({"id": "support:section-a", "node": "node:section-a", "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}));
    raw
}
/// Two-body case B, with W6's tip force and tip torque on body 1 only: W-C1's input before B1
/// (RR "I68's probe verified…"). It is W2-published with the published verdict `checks_passed`
/// (R/I81/b1_probe_01 PROBE §2.3), so under T-4 it is `not_required`: B1's `NoTriggeredCase` pin.
fn u8_two_body_case_b() -> Value {
    let mut raw = u8_two_body_case_a();
    let (tip, p) = (f64::from_bits(0x0031fa182c40c60d), "invented_t3_g5_witness_input_no_library_data");
    raw["model"]["load_cases"][0]["primitive_loads"] = json!([
        {"id": "load:tip-y", "category": "concentrated_force", "target": {"type": "node", "node": "node:section-b"}, "direction": "global_y",
            "dimension": "force", "magnitude": {"value": tip, "unit": "N"}, "provenance": p},
        {"id": "load:tip-torque", "category": "concentrated_moment", "target": {"type": "node", "node": "node:section-b"}, "direction": "rotation_x",
            "dimension": "moment", "magnitude": {"value": tip, "unit": "N*m"}, "provenance": p}]);
    raw
}
/// W-C2's case C (B0 DESIGN_v2 §1.4; R/I81/b1_probe_01 PROBE §4): case A's loads (the milestone's
/// three moments on body 0) followed by case B's (the tip force and torque on body 1), as one load
/// case. W-C1's input and W6's stack-witness input since B1 (RR "I81's B1-0 probe verified…",
/// ruling 3): Sensitive, W2-published (b = 518), and the native run ends Unresolved(Ceiling).
pub(super) fn w_c2_case_c() -> Value {
    let mut raw = u8_two_body_case_a();
    let tip = u8_two_body_case_b()["model"]["load_cases"][0]["primitive_loads"].as_array().unwrap().clone();
    raw["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().extend(tip);
    raw
}
/// PROBE §4's input sha256 of case C, and PROBE §2.3's of two-body case B (`serde_json::to_vec`).
pub(super) const W_C2_CASE_C_INPUT_SHA256: &str = "3649b4dcb96ecb9e8a32ee9ae95bddea65648d3d6aff10860785cb95f73306b3";
const TWO_BODY_B_INPUT_SHA256: &str = "cf688351686bbaff9843052438479a9b5410d0e722245375b1d2aa729f21a406";
/// The L = 0 base (PLAN §1.2): the milestone plus node N2 at (3, 0, 0), which no member
/// references, with one rigid support restraining all six DOFs (the milestone's rigid-support
/// shape, no family). Body 1 is a single node: extent 0.
fn u8_l0_isolated_node() -> Value {
    let mut raw = raw();
    let p = "invented_t3_p1_detection_input_no_library_data";
    raw["model"]["nodes"].as_array_mut().unwrap().push(json!({"id": "N2", "position": {"x": 3.0, "y": 0.0, "z": 0.0}, "provenance": p}));
    raw["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "rigid:N2", "node": "N2", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}));
    raw
}

/// U8 (RV93 N-5; W-C1): real D1 inputs reach the Candidate, Preparation and Native fallbacks on
/// the actual Direct entry, with no fault hook. In the registered build each publishes the plain
/// bytes plus exactly one N1 notice, from one ordinary run that reached G-C once. In any other
/// build each publishes the plain bytes from one run. B1 (T-4; RR "I81's B1-0 probe verified…",
/// ruling 3): W-C1's variant is case C alone, since two-body case B is now `not_required`.
#[test]
fn u8_real_input_fallbacks_append_one_notice() {
    let case_c = w_c2_case_c();
    assert_eq!(sha(&serde_json::to_vec(&case_c).unwrap()), W_C2_CASE_C_INPUT_SHA256, "PROBE §4's case C");
    let variants = [
        ("first_load_only", u8_first_load_only(), W1Fallback::Candidate),
        ("tiny_spring", u8_tiny_spring(), W1Fallback::Preparation),
        ("w_c1_case_c", case_c, W1Fallback::Native),
    ];
    for mode in MODES {
        for (label, raw, cause) in &variants {
            let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap();
            let plain = plain(mode, raw);
            assert!(hooks::armed_names().is_empty(), "{label} {mode:?}: no fault armed before");
            let (output, counts) = direct(raw, mode);
            if !registered() {
                assert!(output.retained().is_none(), "{label}: no permit, no W1");
                assert_eq!(counts, ONE_RUN, "{label} {mode:?}");
                assert_eq!(published(output), plain, "{label} {mode:?}: the ordinary route");
                assert!(hooks::armed_names().is_empty(), "{label} {mode:?}: no fault armed after");
                continue;
            }
            assert_eq!(output.admission().unwrap().law().refusal, None, "{label} {mode:?}: admitted (inside D1)");
            assert_eq!(output.retained().and_then(|r| r.as_ref().err()), Some(cause), "{label} {mode:?}: the real input's cause");
            assert!(output.successor().is_none(), "{label} {mode:?}");
            assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{label} {mode:?}: one ordinary run, then G-C once");
            let bytes = published(output);
            assert_eq!(String::from_utf8(bytes.clone()).unwrap(), String::from_utf8(with_notice(&plain, case, None)).unwrap(),
                "{label} {mode:?}: the ordinary bytes, then the notice");
            assert_eq!(notices(&bytes), 1, "{label} {mode:?}: exactly one N1 notice");
            assert!(hooks::armed_names().is_empty(), "{label} {mode:?}: no fault armed after");
        }
    }
}

/// U8's L = 0 pins, per mode: the successor document's sha256 (id `u8_l0_isolated_node_<mode>`,
/// U1's document form), its receipt sha256 and the published bytes' sha256 (R/I68/u8_probe_01 §3).
const U8_L0_PINNED: [(&str, &str, &str, &str); 2] = [
    ("sparse_interactive", "93c6c86548b9d263cba9d9869010043d23ed1c9f2f304dd9f82eb705eb350876", "c00cbe76954e5188c63b0ef69738cd40a8db15d303b1113a86524e6c3119dd72",
        "9b425066029969b992278306a678303717400a5f16636d3427c5b9b8150ac83a"),
    ("dense_scrutiny", "dbb3d477364248fb9ae15f7b9cff44410c2bffe45f783dd02d96eca663b0ac88", "0b4250c8139ba25ab9d35fc2d443a01d85de943a8e3d0eb9193f5dd5061ce994",
        "5d84fce64bb0810da24df9ea5d6ad354b18d864f07826dd09e191e345e4dea48"),
];
/// The milestone's pinned successor documents (U1; the U5-verified reference rows for body 0).
const U8_MILESTONE_DOCUMENTS: [&str; 2] = [
    include_str!("../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"),
    include_str!("../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json"),
];
/// The L = 0 successor document, in U1's form.
fn u8_l0_document(name: &str, raw: &Value, successor: &Value) -> String {
    serde_json::to_string_pretty(&json!({"id": format!("u8_l0_isolated_node_{name}"), "source": successor,
        "invocation": {"request": raw, "solver_mode": name}})).unwrap()
}
/// L = 0's value controls on its published successor (PLAN §1.2's "Tests and controls" row).
/// Body 1 (N2 and rigid:N2): every row is exactly +0 and is input-derived or an exact zero
/// (absolute-verified with scale and bound 0); the receipt gives the body extent-0 coverage.
/// Body 0: every milestone row is present and bit-identical (as observed), carries the milestone's
/// class claim, and agrees within the unchanged U5 criterion (relative rows within 1e-9 of their
/// value, absolute rows within their published bound, input-derived rows exactly), with the
/// milestone's U5-verified rows as the reference.
fn u8_l0_value_controls(name: &str, raw: &Value, successor: &Value, milestone_document: &str) {
    use open_pipe_stress_result_export::retained_precision::{validate, AccuracyClass};
    let classes = |source: &Value, invocation: &Value| -> std::collections::BTreeMap<String, (u64, Option<u64>, AccuracyClass)> {
        let validation = validate(source, Some(invocation)).unwrap_or_else(|e| panic!("{name}: the reader refuses: {e:?}"));
        validation.classifications.into_iter().map(|c| (c.result_id, (c.normalized_bits, c.scale_bits, c.class))).collect()
    };
    let classes_l0 = classes(successor, &json!({"request": raw, "solver_mode": name}));
    let milestone: Value = serde_json::from_str(milestone_document).unwrap();
    let classes_milestone = classes(&milestone["source"], &milestone["invocation"]);
    let rows = successor["results"].as_array().unwrap();
    let bits = |row: &Value| row["value"].as_f64().map(f64::to_bits);
    // Body 1.
    let body1: Vec<&Value> = rows.iter().filter(|r| r["entity_ref"] == "N2" || r["entity_ref"] == "rigid:N2").collect();
    let (mut input_derived, mut exact_zero) = (0, 0);
    for row in &body1 {
        let id = row["id"].as_str().unwrap();
        assert_eq!(bits(row), Some(0), "{name} {id}: body 1's row is exactly +0");
        match &classes_l0[id] {
            (_, _, AccuracyClass::InputDerived) => input_derived += 1,
            (0, Some(0), AccuracyClass::AbsoluteVerified { bound_bits: 0 }) => exact_zero += 1,
            other => panic!("{name} {id}: {other:?} is neither input-derived nor an exact zero"),
        }
    }
    assert_eq!((input_derived, exact_zero), (6, 9), "{name}: body 1's six displacements, its magnitude and its nine reaction rows");
    let body = &successor["retained_precision"]["body"];
    assert_eq!(body["sources"][0]["body_membership"][1], json!({"body": 1, "members": [], "nodes": [2]}), "{name}: body 1 is N2 alone");
    assert_eq!(body["product_attempts"][0]["proof"]["summary_coverage"][1], json!({"body": 1, "has_data": false, "stop": [false, false, false, false]}),
        "{name}: body 1's coverage (extent 0, no free DOF)");
    assert_eq!(body["cases"][0]["selection"]["body_scales"][1], json!({"body": 1, "force": "0000000000000000", "moment": "0000000000000000",
        "rotation": "0000000000000000", "translation": "0000000000000000"}), "{name}: body 1's scales");
    // Body 0.
    let milestone_rows = milestone["source"]["results"].as_array().unwrap();
    assert_eq!(rows.len(), milestone_rows.len() + body1.len(), "{name}: the milestone's rows plus body 1's");
    for reference in milestone_rows {
        let id = reference["id"].as_str().unwrap();
        let row = rows.iter().find(|r| r["id"] == reference["id"]).unwrap_or_else(|| panic!("{name} {id}: missing"));
        assert_eq!(bits(row), bits(reference), "{name} {id}: bit-identical to the milestone");
        let ((normalized, _, class), (reference_normalized, _, reference_class)) = (&classes_l0[id], &classes_milestone[id]);
        assert_eq!(class, reference_class, "{name} {id}: the milestone's class claim");
        let (value, reference_value) = (f64::from_bits(*normalized), f64::from_bits(*reference_normalized));
        let within = match class {
            AccuracyClass::RelativeVerified => (value - reference_value).abs() <= value.abs() / 1e9,
            AccuracyClass::AbsoluteVerified { bound_bits } => (value - reference_value).abs() <= f64::from_bits(*bound_bits),
            AccuracyClass::InputDerived => value.to_bits() == reference_value.to_bits(),
            AccuracyClass::NonQuantity => true,
            AccuracyClass::NotCovered => false,
        };
        assert!(within, "{name} {id}: {class:?} outside the U5 criterion ({value:e} against {reference_value:e})");
    }
}

/// U8: the L = 0 base on the actual Direct entry. In the registered build it is admitted (inside
/// D1) and publishes its pinned successor in both modes, from exactly one ordinary run that reached
/// G-C once, with the plain run's envelope beside it (B′); `I68_U8_OUT` writes the fixtures. In any
/// other build it publishes exactly the plain bytes, also from one run.
#[test]
fn u8_l0_isolated_node_publishes_pinned_successor() {
    let out = std::env::var("I68_U8_OUT").ok().map(std::path::PathBuf::from);
    for ((mode, (name, file_sha, receipt_sha, bytes_sha)), milestone) in MODES.into_iter().zip(U8_L0_PINNED).zip(U8_MILESTONE_DOCUMENTS) {
        assert_eq!(mode.as_str(), name);
        let raw = u8_l0_isolated_node();
        let plain = plain(mode, &raw);
        let (output, counts) = direct(&raw, mode);
        assert_eq!(serde_json::to_vec(output.envelope()).unwrap(), plain, "{name}: B′, the ordinary envelope is the plain run");
        if !registered() {
            assert!(output.retained().is_none() && output.successor().is_none(), "{name}: no permit, no W1");
            assert_eq!(counts, ONE_RUN, "{name}");
            assert_eq!(published(output), plain, "{name}: the ordinary route");
            continue;
        }
        assert_eq!(output.admission().unwrap().law().refusal, None, "{name}: admitted (inside D1)");
        assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{name}: one ordinary run, then G-C once");
        let successor = match output.retained() {
            Some(Ok(successor)) => successor.value().clone(),
            other => panic!("{name}: {other:?}"),
        };
        assert_eq!(output.successor(), Some(&successor));
        let text = u8_l0_document(name, &raw, &successor);
        let bytes = published(output);
        assert_eq!((sha(text.as_bytes()).as_str(), successor["retained_precision"]["receipt_sha256"].as_str(), sha(&bytes).as_str()),
            (file_sha, Some(receipt_sha), bytes_sha), "{name}: the pinned L = 0 successor");
        assert_eq!(bytes, serde_json::to_vec(&successor).unwrap(), "{name}: the one publication is the successor");
        u8_l0_value_controls(name, &raw, &successor, milestone);
        if let Some(dir) = &out {
            std::fs::write(dir.join(format!("retained_precision_l0_successor_{name}.json")), &text).unwrap();
        }
    }
}

/// D-U6-5 (U8): the two L = 0 fixtures are byte-identical copies of PP's pinned L = 0 successor
/// documents. In the registered build each is compared byte for byte with the document the actual
/// Direct entry publishes; in any other build (no permit), with the private driver's successor
/// document. Either way the fixture carries the pinned hashes.
#[test]
fn u8_d_u6_5_l0_fixtures_are_the_live_successors() {
    const FIXTURES: [&str; 2] = [
        include_str!("../../../fixtures/results/retained_precision_l0_successor_sparse_interactive.json"),
        include_str!("../../../fixtures/results/retained_precision_l0_successor_dense_scrutiny.json"),
    ];
    for ((mode, (name, file_sha, receipt_sha, _)), fixture) in MODES.into_iter().zip(U8_L0_PINNED).zip(FIXTURES) {
        let raw = u8_l0_isolated_node();
        let successor = if registered() {
            let (output, counts) = direct(&raw, mode);
            assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{name}");
            match output.into_publication() {
                RetainedPublication::Successor(value) => value,
                RetainedPublication::Ordinary(_) => panic!("{name}: the registered Direct entry did not publish its successor"),
            }
        } else {
            let (capture, observer, ordinary) = observed(mode, &raw);
            retained_w1(observer, ordinary, &capture).1.unwrap_or_else(|f| panic!("{name}: {f:?}")).value().clone()
        };
        assert!(u8_l0_document(name, &raw, &successor) == fixture, "{name}: the L = 0 fixture is the live successor document, byte for byte");
        assert_eq!((sha(fixture.as_bytes()).as_str(), successor["retained_precision"]["receipt_sha256"].as_str()), (file_sha, Some(receipt_sha)),
            "{name}: the pinned L = 0 hashes");
    }
}

// ---- B1 ST (I85): T-4's per-case trigger, decision 21 and `NoTriggeredCase` ------------------
//
// R/I84/b1_plan_01/PLAN_v2.md §2.1 (B0 DESIGN_v2 §1.2, T-4; decisions 1 and 21). The classifier
// `case_triggers` is pinned on hand-built published verdicts and seeds. Decision 21's branch is
// pinned only here, because no audited committed input reaches it (R/I81/b1_probe_01 PROBE §3).
// `NoTriggeredCase` is pinned on real inputs (RR "I81's B1-0 probe verified…", ruling 4).

fn verdict_entry(case: &str, verdict: NumericalQualityStatus) -> NumericalCaseQuality {
    NumericalCaseQuality {
        basis_ref: ResultBasisRef { ref_type: "load_case".into(), ref_id: case.into() },
        structural_status: StructuralStatus::PassiveModelBasis,
        solve_quality: verdict,
        model_matrix_fidelity: ModelMatrixFidelity::NotAssessed,
        accuracy_evidence: AccuracyEvidence::NotClaimed,
        evidence_refs: Vec::new(),
    }
}
fn quality_of(entries: Vec<NumericalCaseQuality>) -> NumericalQuality {
    NumericalQuality { cases: entries, ..unassessed_numerical_quality() }
}
fn seed_of(case: &str, initial: Option<rp::InitialSeed>, w2: rp::W2Seed) -> rp::OrdinarySeed {
    rp::OrdinarySeed { case: case.into(), initial, w2, load_row_finding: None, d5_diagnostic_ref: None, recovery_demoted: false, legacy: None }
}
fn report_seed(code: &str) -> Option<rp::InitialSeed> {
    Some(rp::InitialSeed::Report { code: code.into(), report_diagnostic_ref: "diagnostic:numerical-integrity:case".into() })
}
fn failure_seed(error: StructuralError) -> Option<rp::InitialSeed> {
    Some(rp::InitialSeed::StructuralFailure { error, diagnostic_ref: None })
}
fn w2_published() -> rp::W2Seed {
    rp::W2Seed::Published { trigger: RangeTrigger::Evaluation(StructuralError::Range("arithmetic outside normal range")), force_scale_exponent: 518,
        report_diagnostic_ref: Some("diagnostic:numerical-integrity:case".into()) }
}
fn w2_failed() -> rp::W2Seed {
    rp::W2Seed::Failed { trigger: RangeTrigger::Evaluation(StructuralError::Range("arithmetic outside normal range")),
        failure: ForceScalingFailure::NotAdmitted { family: "spring", b: 518 }, diagnostic_ref: "diagnostic:numerical-integrity:case".into() }
}
fn classified(quality: &NumericalQuality, seeds: &[rp::OrdinarySeed], ids: &[&str]) -> Vec<CaseTrigger> {
    case_triggers(quality, seeds, ids).collect()
}

/// T-4's unit tests (PLAN_v2 §2.1's list): each published verdict; W2-published Passed; report
/// Passed; the verdict, not the seed's `initial`, decides; each excluded tag with and without W2;
/// the other failure tags; no seed (in A, unless the verdict is Passed); quality entries and seeds
/// out of request order, which the case-id lookup reads correctly; and an entry or seed that is
/// not unique.
#[test]
fn b1_t4_classifier_reads_the_published_verdict_by_case_id() {
    use CaseTrigger::{Attempted, Excluded, NotRequired};
    use NumericalQualityStatus as V;
    let one = |verdict: Option<V>, seed: Option<rp::OrdinarySeed>| {
        let quality = quality_of(verdict.map(|v| verdict_entry("case", v)).into_iter().collect());
        let seeds: Vec<rp::OrdinarySeed> = seed.into_iter().collect();
        let got = classified(&quality, &seeds, &["case"]);
        assert_eq!(got.len(), 1, "one class per requested case");
        got[0]
    };
    // Each verdict, with the report seed that verdict comes from.
    for (verdict, code, expected) in [(V::ChecksPassed, "NUMERICAL_INTEGRITY_CHECKS_PASSED", NotRequired), (V::Sensitive, "NUMERICAL_INTEGRITY_SENSITIVE", Attempted),
        (V::NotAssessed, "NUMERICAL_INTEGRITY_SENSITIVE", Attempted), (V::Unresolved, "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED", Attempted),
        (V::Failed, "NUMERICAL_INTEGRITY_FAILED", Attempted)] {
        assert_eq!(one(Some(verdict), Some(seed_of("case", report_seed(code), rp::W2Seed::NotTriggered))), expected, "{verdict:?}");
    }
    // W2-published Passed (W6's and two-body B's shape): not_required, though its initial attempt failed.
    assert_eq!(one(Some(V::ChecksPassed), Some(seed_of("case", failure_seed(StructuralError::Range("arithmetic outside normal range")), w2_published()))), NotRequired,
        "W2-published Passed");
    // Report Passed (W2b's shape).
    assert_eq!(one(Some(V::ChecksPassed), Some(seed_of("case", report_seed("NUMERICAL_INTEGRITY_CHECKS_PASSED"), rp::W2Seed::NotTriggered))), NotRequired,
        "report Passed");
    // The published verdict decides, not the seed's initial outcome (R-b′ demotes the verdict after the report).
    let mut demoted = seed_of("case", report_seed("NUMERICAL_INTEGRITY_CHECKS_PASSED"), rp::W2Seed::NotTriggered);
    demoted.recovery_demoted = true;
    assert_eq!(one(Some(V::Sensitive), Some(demoted)), Attempted, "a report-Passed seed under a Sensitive verdict");
    // Decision 21: each excluded tag, without W2 (not triggered, or failed), and with W2 published.
    let excluded_tags = || [StructuralError::Mechanism { direction: vec![1.0, 0.0] }, StructuralError::Asymmetric { row: 0, col: 1, relative_skew: 1e-3 },
        StructuralError::InvalidInput("invalid structural input")];
    for error in excluded_tags() {
        for verdict in [Some(V::Failed), Some(V::Unresolved), Some(V::NotAssessed), None] {
            assert_eq!(one(verdict, Some(seed_of("case", failure_seed(error.clone()), rp::W2Seed::NotTriggered))), Excluded, "{error:?} {verdict:?}, no W2");
            assert_eq!(one(verdict, Some(seed_of("case", failure_seed(error.clone()), w2_failed()))), Excluded, "{error:?} {verdict:?}, W2 failed");
            assert_eq!(one(verdict, Some(seed_of("case", failure_seed(error.clone()), w2_published()))), Attempted, "{error:?} {verdict:?}, W2 published");
        }
        // A Passed verdict is not_required whatever the seed says.
        assert_eq!(one(Some(V::ChecksPassed), Some(seed_of("case", failure_seed(error.clone()), rp::W2Seed::NotTriggered))), NotRequired, "{error:?} Passed");
    }
    // The other attempted failures stay in A (tiny_spring's NumericallyUnresolved among them; PROBE §3).
    for initial in [failure_seed(StructuralError::Range("arithmetic outside normal range")),
        failure_seed(StructuralError::NumericallyUnresolved { reason: "positive diagonal contribution absorbed by assembly; stabilization unresolved", global_dof: Some(3) }),
        failure_seed(StructuralError::NegativeEnergy { direction: vec![1.0], energy: -1.0, allowance: 0.0 }),
        Some(rp::InitialSeed::FormationFailure { error: FrameKernelError::NumericalRange { name: "12EIy/L^3: (12*E)*Iy" } }),
        None] {
        for w2 in [rp::W2Seed::NotTriggered, w2_failed()] {
            assert_eq!(one(Some(V::Unresolved), Some(seed_of("case", initial.clone(), w2))), Attempted, "{initial:?}");
        }
    }
    // No seed (W2's witness on the private driver: `not_assessed`, never attempted): in A
    // (RR "I81's B1-0 probe verified…", ruling 2), and with no quality entry either.
    assert_eq!(one(Some(V::NotAssessed), None), Attempted, "no seed");
    assert_eq!(one(Some(V::Failed), None), Attempted, "no seed, Failed");
    // A Passed verdict decides alone: with no seed the case is still `not_required` (RV109 N-1).
    assert_eq!(one(Some(V::ChecksPassed), None), NotRequired, "Passed with no seed");
    assert_eq!(one(None, None), Attempted, "no entry and no seed");
    // Request order a, b, c, d; quality entries and seeds in other orders. Positions would read
    // c's verdict for a, a's for b and b's for c.
    let quality = quality_of(vec![verdict_entry("c", V::ChecksPassed), verdict_entry("a", V::Sensitive), verdict_entry("b", V::ChecksPassed), verdict_entry("d", V::Failed)]);
    let seeds = [
        seed_of("d", failure_seed(StructuralError::Mechanism { direction: vec![0.0, 1.0] }), rp::W2Seed::NotTriggered),
        seed_of("b", report_seed("NUMERICAL_INTEGRITY_CHECKS_PASSED"), rp::W2Seed::NotTriggered),
        seed_of("c", failure_seed(StructuralError::Range("arithmetic outside normal range")), w2_published()),
        seed_of("a", report_seed("NUMERICAL_INTEGRITY_SENSITIVE"), rp::W2Seed::NotTriggered),
    ];
    assert_eq!(classified(&quality, &seeds, &["a", "b", "c", "d"]), [Attempted, NotRequired, NotRequired, Excluded], "request order, looked up by id");
    assert_eq!(classified(&quality, &seeds, &["d", "c", "b", "a"]), [Excluded, NotRequired, NotRequired, Attempted], "another request order");
    assert_eq!(classified(&quality, &seeds, &[]), Vec::<CaseTrigger>::new(), "no requested case");
    // An entry or a seed that is not unique is absent.
    let doubled = quality_of(vec![verdict_entry("case", V::ChecksPassed), verdict_entry("case", V::ChecksPassed)]);
    assert_eq!(classified(&doubled, &[], &["case"]), [Attempted], "two entries for one case");
    let mechanism = || seed_of("case", failure_seed(StructuralError::Mechanism { direction: vec![1.0] }), rp::W2Seed::NotTriggered);
    assert_eq!(classified(&quality_of(vec![verdict_entry("case", V::Failed)]), &[mechanism(), mechanism()], &["case"]), [Attempted], "two seeds for one case");
}

/// B1 ST (T-4; RR "I81's B1-0 probe verified…", ruling 4): a `NoTriggeredCase` pin on a real
/// one-case input whose published verdict is `checks_passed`, in both modes. A is empty, so:
/// - **The private driver, in every build:** `retained_w1` returns `NoTriggeredCase` with the
///   ordinary owner's exact bytes, and no W1 stage runs (an armed native-stage fault stays armed).
/// - **T-4 runs before R-2's reservation** (ST repair 1, RV109 SF-1). After `NoTriggeredCase` the
///   owner's diagnostics capacity is unchanged and equal to its length: no slot was reserved. And
///   when the base already carries the notice's id, the cause is still `NoTriggeredCase` with the
///   exact bytes; a reservation ahead of T-4 would end at `NoticeReservation` there.
/// - **The actual Direct entry:** in the registered build it is admitted and publishes the exact
///   ordinary bytes, with no notice and no W1 work, from one ordinary run that reached G-C once. In
///   any other build it publishes the plain bytes from one run.
///
/// `input_sha256` is the input's PROBE pin (`serde_json::to_vec`).
pub(super) fn assert_no_triggered_case_pin(label: &str, raw: &Value, input_sha256: &str) {
    assert_eq!(sha(&serde_json::to_vec(raw).unwrap()), input_sha256, "{label}: PROBE's input");
    let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap();
    for mode in MODES {
        let plain = plain(mode, raw);
        // The private driver. The native-stage fault is a sentinel: it fires only if W1 reaches native.
        let (capture, observer, ordinary) = observed(mode, raw);
        assert_eq!(serde_json::to_vec(&ordinary).unwrap(), plain, "{label} {mode:?}: the observed run is the plain run");
        assert_eq!(ordinary.numerical_quality.cases.iter().map(|c| (c.basis_ref.ref_id.as_str(), c.solve_quality)).collect::<Vec<_>>(),
            [(case, NumericalQualityStatus::ChecksPassed)], "{label} {mode:?}: the published verdict");
        let capacity = ordinary.diagnostics.capacity();
        assert_eq!(capacity, ordinary.diagnostics.len(), "{label} {mode:?}: precondition: no spare diagnostics slot before W1");
        hooks::withdraw_next_native_source();
        let (envelope, retained) = retained_w1(observer, ordinary, &capture);
        assert_eq!(retained.err(), Some(W1Fallback::NoTriggeredCase), "{label} {mode:?}");
        assert_eq!(String::from_utf8(serde_json::to_vec(&envelope).unwrap()).unwrap(), String::from_utf8(plain.clone()).unwrap(),
            "{label} {mode:?}: the exact ordinary bytes, no notice");
        assert_eq!((envelope.diagnostics.capacity(), envelope.diagnostics.len()), (capacity, capacity),
            "{label} {mode:?}: T-4 precedes R-2: no diagnostics slot was reserved");
        assert_eq!(hooks::armed_names(), ["native"], "{label} {mode:?}: no W1 stage ran");
        hooks::disarm();
        // The private driver, with the notice's id already in the base (R-2's collision).
        let (capture, observer, mut ordinary) = observed(mode, raw);
        ordinary.diagnostics.push(Diagnostic { id: format!("diagnostic:retained-precision:{case}:unavailable"), code: "X".into(),
            severity: "info".into(), message: "m".into(), source: None, affected_refs: Vec::new() });
        ordinary.diagnostics.shrink_to_fit();
        let marked = serde_json::to_vec(&ordinary).unwrap();
        let (envelope, retained) = retained_w1(observer, ordinary, &capture);
        assert_eq!(retained.err(), Some(W1Fallback::NoTriggeredCase), "{label} {mode:?}: T-4 precedes R-2's collision check");
        assert_eq!(serde_json::to_vec(&envelope).unwrap(), marked, "{label} {mode:?}: the colliding base's exact bytes");
        // The actual Direct entry.
        hooks::withdraw_next_native_source();
        let (output, counts) = direct(raw, mode);
        if !registered() {
            assert!(output.retained().is_none(), "{label} {mode:?}: no permit, no W1");
            assert_eq!(counts, ONE_RUN, "{label} {mode:?}");
        } else {
            assert_eq!(output.admission().unwrap().law().refusal, None, "{label} {mode:?}: admitted (inside D1)");
            assert_eq!(output.retained().and_then(|r| r.as_ref().err()), Some(&W1Fallback::NoTriggeredCase), "{label} {mode:?}");
            assert!(output.successor().is_none(), "{label} {mode:?}");
            assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{label} {mode:?}: one ordinary run, then G-C once");
        }
        assert_eq!(hooks::armed_names(), ["native"], "{label} {mode:?}: no W1 stage ran on the reserved-stack thread");
        hooks::disarm();
        let bytes = published(output);
        assert_eq!(notices(&bytes), 0, "{label} {mode:?}: no notice");
        assert_eq!(String::from_utf8(bytes).unwrap(), String::from_utf8(plain).unwrap(), "{label} {mode:?}: the exact ordinary bytes");
    }
}

/// B1 ST (T-4; RR "I81's B1-0 probe verified…", ruling 4): two-body case B is W2-published with
/// the published verdict `checks_passed` (PROBE §2.3), so it is `not_required` and A is empty:
/// the `NoTriggeredCase` pin above, on the private driver and the actual Direct entry.
#[test]
fn b1_t4_two_body_case_b_is_a_no_triggered_case_pin() {
    let raw = u8_two_body_case_b();
    for mode in MODES {
        let (_, observer, _) = observed(mode, &raw);
        assert!(matches!(observer.ordinary.as_slice(), [rp::OrdinarySeed { initial: Some(rp::InitialSeed::StructuralFailure { .. }), w2: rp::W2Seed::Published { .. }, .. }]),
            "{mode:?}: W2-published");
    }
    assert_no_triggered_case_pin("two-body B", &raw, TWO_BODY_B_INPUT_SHA256);
}

/// B1 ST (decision 21, and ruling 2 on a seedless case) through `retained_w1` on the private
/// driver, with the milestone's actual observer and one hand-set seed. A Mechanism failure that
/// W2 did not publish is excluded, so A is empty: `NoTriggeredCase`, exact bytes. With W2
/// published it is in A, and so is a case with no seed: W1 runs (the notice, or a successor).
#[test]
fn b1_t4_retained_w1_applies_decision_21_and_keeps_a_seedless_case() {
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = raw();
    let plain = plain(mode, &raw);
    let mechanism = || failure_seed(StructuralError::Mechanism { direction: vec![1.0, 0.0] });
    let (capture, mut observer, ordinary) = observed(mode, &raw);
    observer.ordinary[0].initial = mechanism();
    observer.ordinary[0].w2 = rp::W2Seed::NotTriggered;
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::NoTriggeredCase), "excluded: A is empty");
    assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "excluded: the exact ordinary bytes");
    for label in ["W2 published", "no seed"] {
        let (capture, mut observer, ordinary) = observed(mode, &raw);
        if label == "no seed" {
            observer.ordinary.clear();
        } else {
            observer.ordinary[0].initial = mechanism();
            observer.ordinary[0].w2 = w2_published();
        }
        let (envelope, retained) = retained_w1(observer, ordinary, &capture);
        assert_ne!(retained.as_ref().err(), Some(&W1Fallback::NoTriggeredCase), "{label}: in A, so W1 runs");
        let bytes = serde_json::to_vec(&envelope).unwrap();
        assert!(retained.is_ok() && bytes == plain || bytes == with_notice(&plain, "case", None), "{label}: {:?}", retained.err());
    }
}

// ---- B1 SP (I85): the n-case transaction, T-2 to T-7 ------------------------------------------
//
// R/I84/b1_plan_01/PLAN_v2.md §2.2 (B0 DESIGN_v2 §1.2, T-2 to T-7). Until I2 merges SA's
// `LOAD_CASES` = 3, `retained_w1` refuses c ≥ 2 with `Domain` (RV107 A1-S-2), so these tests drive
// the capture and the preparation directly, below `retained_w1`: the observed ordinary run with the
// capture installed (T-2), invocation custody (T-6) and the per-case product attempts (T-7).

/// W-C2 (B0 DESIGN_v2 §1.4; decision 2): three cases on U8's two-body model, in request order:
/// - `case-a`: the milestone's three moments on body 0 (two-body case A's loads);
/// - `case-b`: the tip force and torque on body 1 (two-body case B's loads);
/// - `case-c`: A's loads followed by B's (case C), each id suffixed `:c`, because primitive-load ids
///   are unique across the model.
pub(super) fn w_c2() -> Value {
    let mut raw = u8_two_body_case_a();
    let template = raw["model"]["load_cases"][0].clone();
    let loads_a = template["primitive_loads"].clone();
    let loads_b = u8_two_body_case_b()["model"]["load_cases"][0]["primitive_loads"].clone();
    let mut loads_c = w_c2_case_c()["model"]["load_cases"][0]["primitive_loads"].clone();
    for load in loads_c.as_array_mut().unwrap() {
        load["id"] = json!(format!("{}:c", load["id"].as_str().unwrap()));
    }
    let case = |id: &str, loads: Value| {
        let mut case = template.clone();
        case["id"] = json!(id);
        case["primitive_loads"] = loads;
        case
    };
    raw["model"]["load_cases"] = json!([case("case-a", loads_a), case("case-b", loads_b), case("case-c", loads_c)]);
    raw
}
const W_C2_IDS: [&str; 3] = ["case-a", "case-b", "case-c"];

/// T-2 (one owner, per-case capture), both modes, on W-C2: the observed ordinary run captures each
/// requested case in its own slot, in request order: its scope and custody counters, its solver
/// observations, and its late old-source capture (the source with that case's loads, and its facts
/// and operational records). The seeds are per case, and the published verdicts are A Sensitive,
/// B Passed and C Sensitive (PROBE §2.3 and §4), so T-4's A is {A, C}.
#[test]
fn b1_sp_t2_the_capture_holds_each_requested_case() {
    let raw = w_c2();
    for mode in MODES {
        let plain = plain(mode, &raw);
        let (capture, mut observer, ordinary) = observed(mode, &raw);
        assert_eq!(serde_json::to_vec(&ordinary).unwrap(), plain, "{mode:?}: the observed run is the plain run");
        assert_eq!(ordinary.status.mechanics, "MECHANICS_SOLVED", "{mode:?}");
        assert!(observer.error.is_none() && observer.parked_cases().iter().all(|slot| slot.error.is_none()), "{mode:?}: {:?}", observer.error);
        assert_eq!((observer.cases_seen(), observer.parked_cases().len()), (3, 2), "{mode:?}: three slots, the last in the capture's fields");
        let loads = [3, 2, 5];
        for (index, id) in W_C2_IDS.into_iter().enumerate() {
            let (case_id, calls, observations, source_loads, facts, operational) = observer.with_case(index, |c| {
                (c.case_id.clone(), (c.case_calls, c.prepared_late_calls, c.source_capture_entries, c.observation_calls),
                    c.observations.as_ref().map(|o| (o.case.clone(), o.mode)), c.source.as_ref().map(|s| s.loads().len()),
                    c.facts.len(), c.operational.len())
            });
            assert_eq!(case_id, id, "{mode:?}: slot {index} is request case {index}");
            assert_eq!(calls, (1, 1, 1, 1), "{mode:?} {id}: one early hook, one late hook, one late capture, one observation");
            assert_eq!(observations, Some((id.to_owned(), mode)), "{mode:?} {id}: its own observations");
            assert_eq!(source_loads, Some(loads[index]), "{mode:?} {id}: its own loads in its captured source");
            assert_eq!((facts, operational), (2, 2), "{mode:?} {id}: both members' facts and operational records");
        }
        assert_eq!(observer.ordinary.iter().map(|s| s.case.as_str()).collect::<Vec<_>>(), W_C2_IDS, "{mode:?}: one seed per case");
        assert_eq!(ordinary.numerical_quality.cases.iter().map(|c| (c.basis_ref.ref_id.as_str(), c.solve_quality)).collect::<Vec<_>>(),
            [("case-a", NumericalQualityStatus::Sensitive), ("case-b", NumericalQualityStatus::ChecksPassed), ("case-c", NumericalQualityStatus::Sensitive)],
            "{mode:?}: the published verdicts");
        assert_eq!(classified(&ordinary.numerical_quality, &observer.ordinary, &W_C2_IDS),
            [CaseTrigger::Attempted, CaseTrigger::NotRequired, CaseTrigger::Attempted], "{mode:?}: T-4's A is {{A, C}}");
        let _ = capture;
    }
}

/// T-6 and T-7, both modes, on W-C2 with A = {A, C} (request indices 0 and 2):
/// - custody is checked once, then each case in A gets one product attempt, in request order, with
///   ids in actual start order (0, 1), and each prepares its own case's source;
/// - case B, `not_required`, gets no attempt: its slot keeps its captured source, unprepared;
/// - with `fail_preparation_of_case(i)`, only that case's attempt fails: its preparation stage
///   fails with the helper's refusal, its error stays in its slot, its terminal snapshot is
///   preparation's, and it has no prepared source; the other case's attempt still prepares.
#[test]
fn b1_sp_t6_t7_custody_once_then_one_attempt_per_case_in_a() {
    use super::retained_receipt::{Stage, StageState};
    let raw = w_c2();
    for mode in MODES {
        for failing in [None, Some(0), Some(2)] {
            let label = format!("{mode:?} failing {failing:?}");
            let (_, observer, ordinary) = observed(mode, &raw);
            if let Some(index) = failing {
                hooks::fail_preparation_of_case(index);
            }
            let mut prepared = observer.prepare_cases(ordinary, 3, &[0, 2]).unwrap_or_else(|f| panic!("{label}: custody {:?}", f.error));
            assert!(hooks::armed_names().is_empty(), "{label}: the case's fault fired");
            assert_eq!(prepared.attempts.iter().map(|a| (a.request, a.attempt)).collect::<Vec<_>>(), [(0, 0), (2, 1)],
                "{label}: request order, ids in start order");
            for attempt in &prepared.attempts {
                let fails = failing == Some(attempt.request);
                assert_eq!(attempt.prepared, !fails, "{label} request {}", attempt.request);
                assert_eq!(attempt.trace.stages[Stage::Preparation as usize], if fails { StageState::Failed } else { StageState::Completed },
                    "{label} request {}", attempt.request);
                assert_eq!(attempt.trace.source_ready, !fails, "{label} request {}: a prepared source exactly when preparation completed", attempt.request);
                assert_eq!(attempt.trace.adapter.is_some(), fails, "{label} request {}: a failed attempt's terminal snapshot", attempt.request);
                assert_eq!(attempt.parts.preparation_error.is_some(), fails, "{label} request {}: the helper's refusal", attempt.request);
                assert_eq!(attempt.trace.stages[Stage::Native as usize], StageState::NotEntered, "{label}: native is T-8's");
                let error = prepared.capture.with_case(attempt.request, |c| c.error.as_ref().map(|e| format!("{e:?}")));
                assert_eq!(error.is_some(), fails, "{label} request {}: {error:?}", attempt.request);
            }
            let untouched = prepared.capture.with_case(1, |c| (c.case_id.clone(), c.source.as_ref().map(|s| s.loads().len()), c.error.is_some()));
            assert_eq!(untouched, ("case-b".to_owned(), Some(2), false), "{label}: B has no attempt");
            assert_eq!(serde_json::to_vec(&prepared.ordinary).unwrap(), plain(mode, &raw), "{label}: the ordinary owner is untouched");
        }
    }
}

/// T-6's refusals on W-C2, each before any product attempt: a requested case the capture never
/// saw; a prior capture error in any case's slot (parked or not); a case's captured observation
/// that no longer matches the envelope; and an envelope observation row naming no requested case.
#[test]
fn b1_sp_t6_custody_refuses_the_whole_invocation() {
    use super::retained_product::CaptureError;
    let mode = PreviewSolverMode::DenseScrutiny;
    let raw = w_c2();
    let refused = |observer: rp::ProductCapture, ordinary: MechanicsEnvelope, requested: usize| -> String {
        match observer.prepare_cases(ordinary, requested, &[0, 2]) {
            Ok(_) => panic!("custody passed"),
            Err(failure) => {
                assert!(failure.capture.prepared_capacity_bytes == [0; 16], "no attempt started");
                match failure.error { CaptureError::Association(text) => text, other => format!("{other:?}") }
            }
        }
    };
    let (_, observer, ordinary) = observed(mode, &raw);
    assert_eq!(refused(observer, ordinary, 4), "prepared case count");
    for index in [0, 2] {
        let (_, mut observer, ordinary) = observed(mode, &raw);
        observer.with_case(index, |c| c.error = Some("planted".into()));
        assert_eq!(refused(observer, ordinary, 3), "planted", "case {index}'s prior error");
    }
    let (_, mut observer, ordinary) = observed(mode, &raw);
    observer.with_case(1, |c| c.observations.as_mut().unwrap().mode_row.value_bits = 7f64.to_bits());
    assert_eq!(refused(observer, ordinary, 3), "observation captured value/text", "case B's observation");
    let (_, observer, mut ordinary) = observed(mode, &raw);
    let row = ordinary.results.iter_mut().find(|r| r.kind == "linear_solver_mode_basis").unwrap();
    row.basis_ref.as_mut().unwrap().ref_id = "foreign".into();
    assert_eq!(refused(observer, ordinary, 3), "observation final case", "a mode row naming no requested case");
}

/// T-5 (R-2 per case), for every count the domain admits (1 to `caps::LOAD_CASES`): one notice per
/// case in A, in request order, every slot reserved before any W1 work, then published with no
/// allocation, with N1's exact bytes. A collision with the base, or between the notices, and a
/// count outside 1..=C reserve nothing.
#[test]
fn b1_sp_t5_one_reserved_notice_per_case_in_a() {
    let mode = PreviewSolverMode::SparseInteractive;
    let ids: Vec<String> = (1..=crate::retained_memory::caps::LOAD_CASES + 1).map(|k| format!("case-{k}")).collect();
    let ids: Vec<&str> = ids.iter().map(String::as_str).collect();
    let (_, _, base) = observed(mode, &raw());
    for count in 1..=crate::retained_memory::caps::LOAD_CASES {
        let mut ordinary = base.clone();
        ordinary.diagnostics.shrink_to_fit();
        let before = serde_json::to_vec(&ordinary).unwrap();
        let notices = ReservedNotices::reserve(&mut ordinary, &ids[..count]).unwrap_or_else(|| panic!("{count} notices"));
        assert!(ordinary.diagnostics.capacity() >= ordinary.diagnostics.len() + count, "{count}: every slot reserved before W1");
        let (published, cause) = notices.publish(ordinary, W1Fallback::Preparation, 0);
        assert_eq!(cause.err(), Some(W1Fallback::Preparation));
        let mut expected = before;
        for id in &ids[..count] {
            expected = with_notice(&expected, id, None);
        }
        assert_eq!(String::from_utf8(serde_json::to_vec(&published).unwrap()).unwrap(), String::from_utf8(expected).unwrap(),
            "{count}: the notices in request order");
    }
    let mut ordinary = base.clone();
    assert!(ReservedNotices::reserve(&mut ordinary, &ids).is_none(), "C + 1 cases");
    assert!(ReservedNotices::reserve(&mut ordinary, &[]).is_none(), "no case");
    if crate::retained_memory::caps::LOAD_CASES >= 2 {
        let mut ordinary = base.clone();
        assert!(ReservedNotices::reserve(&mut ordinary, &["case-1", "case-1"]).is_none(), "two notices with one id");
    }
    let mut colliding = base.clone();
    colliding.diagnostics.push(Diagnostic { id: format!("diagnostic:retained-precision:{}:unavailable", ids[0]), code: "X".into(),
        severity: "info".into(), message: "m".into(), source: None, affected_refs: Vec::new() });
    let before = serde_json::to_vec(&colliding).unwrap();
    assert!(ReservedNotices::reserve(&mut colliding, &ids[..1]).is_none(), "the base carries the id");
    assert_eq!(serde_json::to_vec(&colliding).unwrap(), before, "nothing reserved, nothing changed");
}

/// The domain re-check (PLAN_v2 §2.2; RV107 SF-2; B2-P, B2-C §9): `w1_case_ids` gives the
/// request's cases in request order for 1 ≤ c ≤ C with the combinations D1.4 admits, and `None`
/// (`Domain`) for C + 1 cases and for no case. B2-A widened D1.4 to combinations (RV122 N-3): a
/// combination within it is admitted, and one outside it (z = 3 so C_eq = 4, h = 4, a range
/// over 4 ids, an id equal to a load case's) gives `None`.
#[test]
fn b1_sp_domain_recheck_names_the_requested_cases() {
    let mode = PreviewSolverMode::SparseInteractive;
    let ids = |raw: &Value| {
        let (_, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
        w1_case_ids(&capture).map(|set| (set.requests().to_vec(), set.ids().iter().map(|s| s.to_string()).collect::<Vec<_>>()))
    };
    assert_eq!(ids(&raw()), Some((vec![0], vec!["case".to_owned()])));
    let mut full = raw();
    let first = full["model"]["load_cases"][0].clone();
    full["model"]["load_cases"] = json!((0..crate::retained_memory::caps::LOAD_CASES).map(|k| {
        let mut case = first.clone();
        case["id"] = json!(format!("case-{k}"));
        case
    }).collect::<Vec<_>>());
    let expected = (0..crate::retained_memory::caps::LOAD_CASES).map(|k| format!("case-{k}")).collect::<Vec<_>>();
    assert_eq!(ids(&full), Some(((0..crate::retained_memory::caps::LOAD_CASES).collect(), expected)), "C cases");
    assert_eq!(ids(&beyond_load_cases(&raw(), true)), None, "C + 1 cases");
    let mut none = raw();
    none["model"]["load_cases"] = json!([]);
    assert_eq!(ids(&none), None, "no case");
    let mut combined = raw();
    combined["model"]["combinations"] = json!([{"id":"combo","basis":"mechanics","terms":[{"load_case":"case","factor":1.0}]}]);
    assert_eq!(ids(&combined), Some((vec![0], vec!["case".to_owned()])), "a combination within D1.4");
    let mut two = raw();
    two["model"]["combinations"] = json!([{"id":"a","basis":"mechanics","terms":[{"load_case":"case","factor":2.0}]},
        {"id":"b","basis":"range_envelope","operand_ids":["case"],"mode":"max"}]);
    assert!(ids(&two).is_some(), "z = 2, C_eq = 3");
    let outside = |combinations: Value| {
        let mut over = raw();
        over["model"]["combinations"] = combinations;
        ids(&over)
    };
    let mechanics = |id: &str, terms: usize| json!({"id":id,"basis":"mechanics","terms":vec![json!({"load_case":"case","factor":1.0}); terms]});
    assert_eq!(outside(json!([mechanics("a", 1), mechanics("b", 1), mechanics("c", 1)])), None, "z = 3: C_eq = 4");
    assert_eq!(outside(json!([mechanics("a", 4)])), None, "h = 4");
    assert!(outside(json!([mechanics("a", 3)])).is_some(), "h = 3");
    assert_eq!(outside(json!([{"id":"a","basis":"range_envelope","operand_ids":["case","case","case","case"],"mode":"max"}])), None, "4 range operands");
    assert_eq!(outside(json!([mechanics("case", 1)])), None, "a combination id equal to a load case's");
    let mut full = raw();
    let first = full["model"]["load_cases"][0].clone();
    let renamed = |id: &str| { let mut case = first.clone(); case["id"] = json!(id); case };
    full["model"]["load_cases"] = json!([first.clone(), renamed("b"), renamed("c")]);
    full["model"]["combinations"] = json!([mechanics("a", 1)]);
    assert_eq!(ids(&full), None, "c = 3 with z = 1: C_eq = 4");
}

// ---- B1 SP (I85): the n-case transaction, T-8 to T-13 -----------------------------------------
//
// PLAN_v2 §2.2's tests. Below `retained_w1`, `w1_transaction` (T-6 to T-11) returns the cause and
// the selected attempts that T-12 publishes with (written before I2, when `LOAD_CASES` = 1). Since
// I2 the same inputs also run through `retained_w1` with their notices
// (`b1_sp_w_c2_through_retained_w1_publishes_t12`) and the Direct entry. Before SR-RS (I3) the accepted Rust
// reader refused W-C2's successor at precommit (G5, B's `not_required`), so its per-case outcomes
// were read from the successor that precommit received (RV107 A1-N-1:
// `hooks::counted_with_successor`). Since I3 the reader accepts it, and W-C2 publishes its
// successor, pinned with its fixtures (`b1_sp_w_c2_direct_entry_publishes_the_pinned_successor`).

/// The successor body's per-case facts that PLAN_v2's W-C2 and ordinal-mapping checks read.
fn w_c2_body(successor: &Value) -> &Value {
    &successor["retained_precision"]["body"]
}
fn case_ref(index: usize) -> Value {
    json!({"kind":"case","index":index})
}
/// One kernel outcome's comparable facts (PLAN_v2 N-16): the terminal (kind and reason), the
/// physical attempt count, and the selected precision.
fn outcome_facts(outcome: &k::ExecutionOutcome) -> (String, usize, Option<u32>) {
    match outcome {
        k::ExecutionOutcome::Selected(owner) => ("selected".into(), owner.evidence().attempts.len(), Some(owner.evidence().selected_precision)),
        k::ExecutionOutcome::Refused { refusal, attempts, .. } => (format!("refused {refusal:?}"), attempts.len(), None),
        k::ExecutionOutcome::Unresolved { reason, attempts, .. } => (format!("unresolved {reason:?}"), attempts.len(), None),
    }
}
/// The one-case native Run of `raw` (one requested case), through the one-case private driver.
fn one_case_run(raw: &Value, mode: PreviewSolverMode) -> k::RecordedCase {
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
    let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|_| panic!("one-case preparation"));
    let _ = prepared.solve_native();
    prepared.capture().native.clone().expect("one Run")
}
/// The one-case native outcome's facts of `raw` (one requested case).
fn one_case_outcome(raw: &Value, mode: PreviewSolverMode) -> (String, usize, Option<u32>) {
    outcome_facts(&one_case_run(raw, mode).outcome)
}
/// A Run's physical attempt records.
fn run_records(outcome: &k::ExecutionOutcome) -> &[k::AttemptRecord] {
    match outcome {
        k::ExecutionOutcome::Selected(owner) => &owner.evidence().attempts,
        k::ExecutionOutcome::Refused { attempts, .. } | k::ExecutionOutcome::Unresolved { attempts, .. } => attempts,
    }
}
/// PLAN_v2 N-16 with RV109's SF-1 (RR "RV109 passes SP in RV-P round 2; …", ruling 1): one
/// `CaseBatchCall` over `raw`'s cases `attempted` (all prepared), against each case's one-case Run
/// (`singles`, in batch order). Each Run's terminal facts are the one-case ones, and so is every
/// field of every physical record, but for C2 §4's group-build sharing: one stiffness gives one
/// group, the first Run builds s128, s256 and v256, and every later Run reuses them. So a later
/// Run's records differ from its one-case records in exactly `shared_built_here` at p128 and p256
/// and `verification_shared_built_here` at p256 (false in the batch, true alone). Returns each
/// Run's facts.
fn n16_batch_against_one_case(label: &str, raw: &Value, mode: PreviewSolverMode, requested: usize, attempted: &[usize], singles: &[Value])
    -> Vec<(String, usize, Option<u32>)> {
    let (_, observer, ordinary) = observed(mode, raw);
    let mut prepared = observer.prepare_cases(ordinary, requested, attempted).unwrap_or_else(|f| panic!("{label}: {:?}", f.error));
    prepared.native();
    let mut facts = Vec::new();
    for (position, (&request, single)) in attempted.iter().zip(singles).enumerate() {
        let batch = prepared.capture.with_case(request, |c| c.native.clone()).unwrap_or_else(|| panic!("{label}: request {request}'s Run"));
        let alone = one_case_run(single, mode);
        assert_eq!(outcome_facts(&batch.outcome), outcome_facts(&alone.outcome), "{label} request {request}: N-16, the terminal facts");
        let (in_batch, by_itself) = (run_records(&batch.outcome), run_records(&alone.outcome));
        assert_eq!(in_batch.len(), by_itself.len(), "{label} request {request}");
        let mut flags = Vec::new();
        for (x, y) in in_batch.iter().zip(by_itself) {
            let mut y = y.clone();
            if x.shared_built_here != y.shared_built_here {
                flags.push((x.precision, "shared_built_here", x.shared_built_here, y.shared_built_here));
                y.shared_built_here = x.shared_built_here;
            }
            if x.verification_shared_built_here != y.verification_shared_built_here {
                flags.push((x.precision, "verification_shared_built_here", x.verification_shared_built_here, y.verification_shared_built_here));
                y.verification_shared_built_here = x.verification_shared_built_here;
            }
            assert!(*x == y, "{label} request {request}: record p{} differs beyond the build-provenance flags", x.precision);
        }
        let expected: &[(u32, &str, bool, bool)] = if position == 0 { &[] } else {
            &[(128, "shared_built_here", false, true), (256, "shared_built_here", false, true), (256, "verification_shared_built_here", false, true)]
        };
        println!("B1_SP_N16_RECORDS {label} request={request} position={position} records={} flags={flags:?}", in_batch.len());
        assert_eq!(flags, expected, "{label} request {request} (batch position {position}): C2 §4's shared builds only");
        facts.push(outcome_facts(&batch.outcome));
    }
    facts
}

/// W-C2 (PLAN_v2 §2.2, N-1, N-2, N-16) below `retained_w1`, both modes, with A = {A, C}:
/// - T-8: one call over A's and C's prepared sources; A's Run ends `selected`, C's
///   `unresolved {space: unresolved, tag: ceiling}`; T-9 freezes A.
/// - Since I3 the precommit (SR-RS's reader, with the invocation) accepts the successor, which
///   the transfer moves out; the ordinary owner is untouched. (Before I3: `G5`,
///   `RETAINED_PRECISION_ATTEMPT_MISMATCH`, B's `not_required`, with A selected.)
/// - The successor: A `selected`, B `not_required` (no product attempt), C `unavailable`
///   (`kernel_unresolved`, its Run and source).
/// - **The ordinal mapping (N-2):** the batch ordinals {0, 1} name request indices {0, 2} in the
///   Runs' owners, the call's `owner_refs`, `work.execution_order`, the sources' owners and the
///   product attempts; each source's preparation names its attempt (T-7's `attempt_ref`).
/// - One material basis for every case, one group (one stiffness), and `charged` = the call's
///   `invocation_after`.
/// - **The record point (T-11):** C's snapshot is taken at its Run (T-8), before A's freeze, so
///   it records fewer adapter events than A's, taken at A's last proof stage.
/// - **N-16** (with RV109's SF-1): each Run's terminal facts and every physical record equal the
///   case's one-case Run's, but for C's p128 and p256 build-provenance flags: C2 §4's group-build
///   sharing (`n16_batch_against_one_case`).
/// - Staging order (T-11): A's selected diagnostic, then C's unavailable diagnostic, last.
#[test]
fn b1_sp_w_c2_transaction_outcomes_and_ordinal_mapping() {
    let raw = w_c2();
    for mode in MODES {
        let label = format!("{mode:?}");
        let plain = plain(mode, &raw);
        let (capture, observer, ordinary) = observed(mode, &raw);
        let ((envelope, outcome), counts, successor) = hooks::counted_with_successor(|| w1_transaction(observer, ordinary, &capture, 3, &[0, 2]));
        assert_eq!(counts, Counts { runs: 0, complete_gates: 0 }, "{label}: no ordinary run inside the transaction");
        assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "{label}: the ordinary owner is untouched");
        let published = outcome.unwrap_or_else(|f| panic!("{label}: since I3 the precommit accepts W-C2: {f:?}"));
        let successor = successor.unwrap_or_else(|| panic!("{label}: precommit received the successor"));
        assert_eq!(published, successor, "{label}: the transfer moves the validated successor");
        let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
        assert!(open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)).is_ok(), "{label}: the Rust reader");
        let body = w_c2_body(&successor);
        let cases = body["cases"].as_array().unwrap();
        assert_eq!(cases.iter().map(|c| c["status"].as_str().unwrap()).collect::<Vec<_>>(), ["selected", "not_required", "unavailable"], "{label}");
        assert_eq!(cases.iter().map(|c| c["basis_ref"]["ref_id"].as_str().unwrap()).collect::<Vec<_>>(), W_C2_IDS, "{label}");
        assert_eq!(cases.iter().map(|c| c["ordinary"]["attempt_ref"].clone()).collect::<Vec<_>>(), [json!(0), json!(1), json!(2)], "{label}");
        assert_eq!(cases.iter().map(|c| c["product_attempt_ref"].clone()).collect::<Vec<_>>(), [json!(0), Value::Null, json!(1)], "{label}");
        assert_eq!(cases[1].as_object().unwrap().len(), 4, "{label}: not_required carries basis, ordinary, attempt ref and status only");
        assert_eq!(cases[0]["run"]["kernel_terminal"]["kind"], "selected", "{label}");
        assert_eq!(cases[2]["run"]["kernel_terminal"], json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}}), "{label}");
        assert_eq!(cases[2]["reason"], json!({"code":"kernel_unresolved","phase":"kernel","cause":{"kind":"prepared_product_failure","product_attempt_ref":1}}), "{label}");
        assert_eq!((cases[0]["source_ref"].clone(), cases[2]["source_ref"].clone()), (json!(0), json!(1)), "{label}");
        // N-2: ordinals {0, 1} are request indices {0, 2}.
        assert_eq!((cases[0]["run"]["origin"]["owner_ref"].clone(), cases[2]["run"]["origin"]["owner_ref"].clone()), (case_ref(0), case_ref(2)), "{label}");
        assert_eq!(body["calls"].as_array().unwrap().len(), 1, "{label}: one CaseBatchCall");
        assert_eq!(body["calls"][0]["owner_refs"], json!([case_ref(0), case_ref(2)]), "{label}");
        assert_eq!(body["work"]["execution_order"], json!([case_ref(0), case_ref(2)]), "{label}");
        let sources = body["sources"].as_array().unwrap();
        assert_eq!(sources.iter().map(|s| (s["index"].clone(), s["owner"]["case_index"].clone(), s["owner"]["case_id"].clone(), s["preparation"]["attempt_ref"].clone()))
            .collect::<Vec<_>>(), [(json!(0), json!(0), json!("case-a"), json!(0)), (json!(1), json!(2), json!("case-c"), json!(1))], "{label}: registration order, attempt refs");
        let attempts = body["product_attempts"].as_array().unwrap();
        assert_eq!(attempts.iter().map(|a| (a["id"].clone(), a["owner_ref"].clone(), a["ordinary_attempt_ref"].clone(), a["source_ref"].clone(), a["run_ref"].clone()))
            .collect::<Vec<_>>(), [(json!(0), case_ref(0), json!(0), json!(0), cases[0]["run"]["id"].clone()), (json!(1), case_ref(2), json!(2), json!(1), cases[2]["run"]["id"].clone())],
            "{label}: start order");
        assert_eq!(body["ordinary_attempts"].as_array().unwrap().iter().map(|o| o["case_id"].as_str().unwrap()).collect::<Vec<_>>(), W_C2_IDS, "{label}");
        assert_eq!(body["material_bases"].as_array().unwrap().len(), 1, "{label}");
        assert_eq!(body["material_bases"][0]["case_indices"], json!([0, 1, 2]), "{label}");
        assert_eq!(body["groups"].as_array().unwrap().len(), 1, "{label}: D1.5's one stiffness, one group");
        assert_eq!(body["groups"][0]["source_refs"], json!([0, 1]), "{label}");
        assert_eq!(body["work"]["charged"], body["calls"][0]["invocation_after"], "{label}");
        // The record point: C's snapshot (its Run) precedes A's freeze.
        let counts = |a: &Value| a["adapter"]["counts"].as_array().unwrap().iter().map(|n| n.as_u64().unwrap()).collect::<Vec<_>>();
        let (a_counts, c_counts) = (counts(&attempts[0]), counts(&attempts[1]));
        assert!(c_counts.iter().zip(&a_counts).all(|(c, a)| c <= a) && c_counts[1] < a_counts[1], "{label}: C {c_counts:?} before A {a_counts:?}");
        // RV109 R3P-8 (S20): the capture's observation capacity is summed over the requested cases
        // (cumulative snapshots, T-11): its case-id slot holds the three ids' bytes.
        let ids_bytes: usize = W_C2_IDS.iter().map(|id| id.len()).sum();
        for attempt in attempts {
            assert_eq!(attempt["adapter"]["observation_capacity_bytes"][0], json!(ids_bytes), "{label}: every case's id capacity");
        }
        assert_eq!(attempts[1]["stages"]["native"], "failed", "{label}");
        assert_eq!(attempts[1]["stages"]["proof_start"], "not_entered", "{label}");
        // Staging order: A's selected diagnostic, then C's unavailable one, last.
        let diagnostics = successor["diagnostics"].as_array().unwrap();
        let tail: Vec<_> = diagnostics[diagnostics.len() - 2..].iter().map(|d| d["id"].as_str().unwrap()).collect();
        assert_eq!(tail, ["diagnostic:retained-precision:case-a:selected", "diagnostic:retained-precision:case-c:unavailable"], "{label}");
        assert!(successor["results"].as_array().unwrap().iter().all(|r| r.get("recovery_method").is_some()
            == (r["basis_ref"]["ref_id"] == "case-a")), "{label}: the method token on A's rows only");
        // T-11 staging, c ≥ 2: each summary headline is the staged rows' maximum of its kind, a tie
        // going to the smaller case id, then the smaller location.
        for (kind, headline) in [("displacement_magnitude", "max_displacement"), ("pipe_elastic_normal_stress_maximum_v2", "max_open_formula_stress")] {
            let best = successor["results"].as_array().unwrap().iter().filter(|r| r["kind"] == kind)
                .max_by(|a, b| a["value"].as_f64().unwrap().total_cmp(&b["value"].as_f64().unwrap())
                    .then_with(|| b["basis_ref"]["ref_id"].as_str().cmp(&a["basis_ref"]["ref_id"].as_str()))
                    .then_with(|| b["entity_ref"].as_str().cmp(&a["entity_ref"].as_str()))).unwrap();
            assert_eq!(successor["summary"][headline], json!({"value":best["value"],"unit":best["unit"],"location_ref":best["entity_ref"],"result_ref":best["id"]}),
                "{label}: the {headline} headline");
            println!("B1_SP_HEADLINE {label} {headline} case={} ordinary={}", best["basis_ref"]["ref_id"], serde_json::to_value(&envelope.summary).unwrap()[headline]);
        }
        // N-16 (SF-1): the batch Runs against the one-case Runs, record by record.
        let batch_facts = n16_batch_against_one_case(&label, &raw, mode, 3, &[0, 2], &[u8_two_body_case_a(), w_c2_case_c()]);
        let single = [one_case_outcome(&u8_two_body_case_a(), mode), one_case_outcome(&w_c2_case_c(), mode)];
        println!("B1_SP_N16 {label} batch={batch_facts:?} one_case={single:?}");
        assert_eq!(batch_facts, single, "{label}: N-16, the batch Runs' terminal facts are the one-case ones");
        assert_eq!(batch_facts[1].0, "unresolved Ceiling", "{label}");
        println!("B1_SP_WC2 {label} statuses={:?} owner_refs={} charged={} a_counts={a_counts:?} c_counts={c_counts:?}",
            cases.iter().map(|c| c["status"].as_str().unwrap()).collect::<Vec<_>>(), body["calls"][0]["owner_refs"], body["work"]["charged"]);
        assert!(hooks::armed_names().is_empty(), "{label}");
    }
}

/// The W-C2 transaction's faults below `retained_w1` (sparse): each gives its T-12 cause and the
/// selected attempts T-12 places C1:68's detail on (bit 0: A), with the ordinary owner untouched
/// and every armed fault fired. Decision 5's abandonment set is each of them:
/// - T-6 custody (case B's tampered observation): `Preparation`, nothing selected;
/// - T-7 on C (`fail_preparation_of_case(2)`): A still selected, C `unavailable` at preparation,
///   with no `CaseSource`, Run, run or source reference, and one call over A alone. The fault zeroes
///   C's diameter fact, which C's attempt records truthfully, so since I3 the reader refuses the
///   successor at G8 (`PREPARATION_MISMATCH`: the recorded old fact is not the request's);
/// - T-7 on A: only C's Run, which is not selected, so no case is selected: `Native` (T-10);
/// - a T-8 call failure (the first prepared source withdrawn: D38's hook shape): `Native`;
/// - staging, each serializer refusal, and precommit: A selected (bit 0);
/// - the R-b′ limit (DESIGN_v2 N-5): B, not attempted, with `recovery_demoted`, abandons A's
///   successor (`Untranslated`, `ordinary_attempts[].formation.recovery_finding`).
#[test]
fn b1_sp_w_c2_transaction_faults_and_abandonment() {
    use super::retained_wire::{ReceiptCheck as C, ReceiptFailure};
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = w_c2();
    let plain = plain(mode, &raw);
    let run = |prepare: &dyn Fn(&mut rp::ProductCapture)| {
        let (capture, mut observer, ordinary) = observed(mode, &raw);
        prepare(&mut observer);
        let ((envelope, outcome), _, successor) = hooks::counted_with_successor(|| w1_transaction(observer, ordinary, &capture, 3, &[0, 2]));
        assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "the ordinary owner is untouched");
        assert!(hooks::armed_names().is_empty(), "every armed fault fired: {:?}", hooks::armed_names());
        (outcome.err().expect("abandoned"), successor)
    };
    let tamper = |o: &mut rp::ProductCapture| o.with_case(1, |c| c.observations.as_mut().unwrap().mode_row.value_bits = 7f64.to_bits());
    assert_eq!(run(&tamper).0, (W1Fallback::Preparation, 0), "T-6 custody");
    // T-7 beside a selected case: C's preparation fails, A's successor goes on.
    hooks::fail_preparation_of_case(2);
    let (cause, successor) = run(&|_| ());
    assert_eq!(cause, (W1Fallback::Precommit { gate: "G8", code: "RETAINED_PRECISION_PREPARATION_MISMATCH".into() }, 0b01), "T-7 on C");
    let successor = successor.expect("precommit received the successor");
    let body = w_c2_body(&successor);
    assert_eq!(body["cases"][2]["status"], "unavailable");
    assert_eq!(body["cases"][2]["reason"], json!({"code":"source_unavailable","phase":"preparation","cause":{"kind":"prepared_product_failure","product_attempt_ref":1}}));
    assert_eq!((body["cases"][2]["run"].clone(), body["cases"][2]["source_ref"].clone()), (Value::Null, Value::Null));
    assert_eq!(body["sources"].as_array().unwrap().len(), 1, "no CaseSource for C");
    assert_eq!(body["sources"][0]["owner"]["case_index"], 0);
    assert_eq!((body["product_attempts"][1]["source_ref"].clone(), body["product_attempts"][1]["run_ref"].clone()), (Value::Null, Value::Null));
    assert_eq!(body["product_attempts"][1]["stages"]["preparation"], "failed");
    assert_eq!(body["product_attempts"][1]["result"]["error"]["kind"], "preparation");
    assert_eq!((body["calls"][0]["owner_refs"].clone(), body["work"]["execution_order"].clone()), (json!([case_ref(0)]), json!([case_ref(0)])), "one call over A alone");
    hooks::fail_preparation_of_case(0);
    assert_eq!(run(&|_| ()).0, (W1Fallback::Native, 0), "T-7 on A: no case selected, C's Run reached");
    hooks::withdraw_next_native_source();
    assert_eq!(run(&|_| ()).0, (W1Fallback::Native, 0), "a T-8 call failure");
    hooks::break_next_staging();
    assert_eq!(run(&|_| ()).0, (W1Fallback::Staging(rp::StagingFault("pipe_stress_extrema[]")), 0b01), "staging");
    for check in [C::WorkCounterRange, C::WorkCounterInconsistent, C::SaturationNotExcluded, C::PublicationHashRange, C::Encoding, C::Untranslated, C::Scope, C::Association] {
        hooks::fail_next_serializer(check);
        assert_eq!(run(&|_| ()).0, (W1Fallback::Serializer(ReceiptFailure { check, field_path: "cases[].run.invocation_after" }), 0b01), "serializer {check:?}");
    }
    hooks::corrupt_next_precommit();
    let (cause, selected) = run(&|_| ()).0;
    assert!(matches!(cause, W1Fallback::Precommit { gate: "G1", .. }) && selected == 0b01, "precommit: {cause:?}");
    let demoted = |o: &mut rp::ProductCapture| o.ordinary[1].recovery_demoted = true;
    assert_eq!(run(&demoted).0, (W1Fallback::Serializer(ReceiptFailure { check: C::Untranslated, field_path: "ordinary_attempts[].formation.recovery_finding" }), 0b01),
        "the R-b′ limit: B's demotion abandons A's successor");
}

/// W-C2 through `retained_w1` (since I2: SA's C = 3; RV107 A1-S-2), both modes:
/// - T-4: A = {A, C}; with `.all` for `.any` (RV109's R17) W-C2 would be `NoTriggeredCase`;
/// - since I3 the successor is published (A selected, B `not_required`, C unavailable), with the
///   ordinary owner untouched beside it (before I3: the precommit's refusal, then the two N1
///   notices, case-a's and case-c's, both plain);
/// - a serializer refusal with C1:68's detail puts it on A's notice only (selected), C's plain;
///   one without a detail leaves both plain; a T-8 call failure leaves both plain.
#[test]
fn b1_sp_w_c2_through_retained_w1_publishes_t12() {
    use super::retained_wire::ReceiptCheck as C;
    let raw = w_c2();
    for mode in MODES {
        let plain = plain(mode, &raw);
        let run = || {
            let (capture, observer, ordinary) = observed(mode, &raw);
            let (envelope, retained) = retained_w1(observer, ordinary, &capture);
            assert!(hooks::armed_names().is_empty(), "{mode:?}: every armed fault fired");
            (String::from_utf8(serde_json::to_vec(&envelope).unwrap()).unwrap(), retained.err())
        };
        assert_eq!(crate::retained_memory::caps::LOAD_CASES, W_C2_IDS.len(), "I2: C = 3");
        let noticed = |detail: Option<&str>| String::from_utf8(with_notice(&with_notice(&plain, "case-a", detail), "case-c", None)).unwrap();
        let (capture, observer, ordinary) = observed(mode, &raw);
        let (envelope, retained) = retained_w1(observer, ordinary, &capture);
        let successor = retained.unwrap_or_else(|f| panic!("{mode:?}: not NoTriggeredCase (R17): {f:?}"));
        assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "{mode:?}: the ordinary owner beside the successor");
        assert_eq!(w_c2_body(successor.value())["cases"].as_array().unwrap().iter().map(|c| c["status"].as_str().unwrap()).collect::<Vec<_>>(),
            ["selected", "not_required", "unavailable"], "{mode:?}");
        hooks::fail_next_serializer(C::WorkCounterInconsistent);
        assert_eq!(run().0, noticed(Some("work_counter_inconsistent")), "{mode:?}: the detail on A's notice only");
        hooks::fail_next_serializer(C::Association);
        assert_eq!(run().0, noticed(None), "{mode:?}: no detail");
        hooks::withdraw_next_native_source();
        assert_eq!(run(), (noticed(None), Some(W1Fallback::Native)), "{mode:?}: a T-8 call failure");
    }
}

/// The pinned W-C2 successors (PLAN_v2 §2.2, after I3): (name, document sha256, receipt sha256,
/// published bytes' sha256). One pin for every platform: support magnitudes are correctly rounded
/// norms (`correct_norm`, I109), so the bytes no longer depend on the platform's libm. Dense was
/// re-pinned by I109 (PR-N): case C's `rigid:N0` support force magnitude is the correctly rounded
/// 1.6258317075882523e-12, one ulp above macOS libm's `hypot` chain (glibc's value).
const W_C2_PINNED: [(&str, &str, &str, &str); 2] = [
    ("sparse_interactive", "7922e3e5278d0d87dc5faf79dfbc1f2a384899e97df306cc742355cdacdb6269", "cccb9664e1c58f0582348df3348d8b6e0b0941bcb0294a5b351a4d092ed18886", "c7a1859330e9e36e18f5572838dbad72ea251a817241acc6968f8f83b70170fa"),
    ("dense_scrutiny", "c11f7566f1f0c469bc0a2808466dd9dd137ea64abed327fb9e4d35ff92ded22f", "ca6a62a6187a08d7b2e2643911fd232b02076b9032be1455754ff780540995f2", "604e4a3380b28d3757d7a7f20aa5d72eae8a93bdc9e24dc966afe6b48fafca3f"),
];
/// The W-C2 successor document, in U1's form.
fn w_c2_document(name: &str, raw: &Value, successor: &Value) -> String {
    serde_json::to_string_pretty(&json!({"id": format!("w_c2_{name}"), "source": successor,
        "invocation": {"request": raw, "solver_mode": name}})).unwrap()
}

/// T-13 and W-C2's pinned successor on the actual Direct entry, both modes: one ordinary run, and
/// G-C reached once (`ONE_RUN_THROUGH_G_C`), with no hook armed. In the registered build (since I2,
/// C = 3; since I3, SR-RS's reader) the one publication is W-C2's successor, pinned by sha256 (the
/// document, the receipt and the published bytes), with the ordinary envelope beside it the plain
/// run's. `I85_WC2_OUT` writes the documents (the D-U6-5 fixtures). In any other build (Stale),
/// exactly the plain bytes.
#[test]
fn b1_sp_w_c2_direct_entry_publishes_the_pinned_successor() {
    let out = std::env::var("I85_WC2_OUT").ok().map(std::path::PathBuf::from);
    let raw = w_c2();
    for (mode, (name, file_sha, receipt_sha, bytes_sha)) in MODES.into_iter().zip(W_C2_PINNED) {
        assert_eq!(mode.as_str(), name);
        let plain = plain(mode, &raw);
        assert!(hooks::armed_names().is_empty(), "{mode:?}: no hook armed");
        let (output, counts) = direct(&raw, mode);
        assert_eq!(serde_json::to_vec(output.envelope()).unwrap(), plain, "{mode:?}: the ordinary envelope is the plain run");
        if !registered() {
            assert!(output.retained().is_none(), "{mode:?}: Stale, no W1");
            assert_eq!((counts, published(output)), (ONE_RUN, plain), "{mode:?}: Stale's plain bytes");
            continue;
        }
        assert_eq!(crate::retained_memory::caps::LOAD_CASES, W_C2_IDS.len(), "I2: C = 3");
        assert_eq!(output.admission().unwrap().law().refusal, None, "{mode:?}: admitted");
        assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{mode:?}: T-13");
        let successor = match output.retained() {
            Some(Ok(successor)) => successor.value().clone(),
            other => panic!("{mode:?}: {other:?}"),
        };
        assert_eq!(output.successor(), Some(&successor));
        let text = w_c2_document(name, &raw, &successor);
        let bytes = published(output);
        println!("B1_SP_WC2_PIN {name} {} {} {}", sha(text.as_bytes()), successor["retained_precision"]["receipt_sha256"].as_str().unwrap(), sha(&bytes));
        if let Some(dir) = &out {
            std::fs::write(dir.join(format!("retained_precision_w_c2_successor_{name}.json")), &text).unwrap();
        }
        assert_eq!((sha(text.as_bytes()).as_str(), successor["retained_precision"]["receipt_sha256"].as_str(), sha(&bytes).as_str()),
            (file_sha, Some(receipt_sha), bytes_sha), "{name}: the pinned W-C2 successor");
        assert_eq!(bytes, serde_json::to_vec(&successor).unwrap(), "{name}: the one publication is the successor");
        // T-12's outcome table: an unavailable case carries its receipt-backed diagnostic, never an
        // N1 notice: C's, with the receipt's text.
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        let unavailable: Vec<&Value> = value["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == "RETAINED_PRECISION_UNAVAILABLE").collect();
        assert_eq!(unavailable.iter().map(|d| (d["id"].clone(), d["message"].clone())).collect::<Vec<_>>(),
            [(json!("diagnostic:retained-precision:case-c:unavailable"), json!(super::retained_wire::UNAVAILABLE_MESSAGE))], "{name}");
        assert!(hooks::armed_names().is_empty(), "{mode:?}");
    }
}

/// Multi-case coexistence (PLAN_v2 §2.2, N-5; decision 26, RV107 A1-N-7): n05's source-block
/// request with a second case (a copy of its one case, `case-2`, its load id suffixed `:2`).
fn n05_two_cases() -> Value {
    let mut raw: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.request.json")).unwrap();
    let mut second = raw["model"]["load_cases"][0].clone();
    second["id"] = json!("case-2");
    for load in second["primitive_loads"].as_array_mut().unwrap() {
        load["id"] = json!(format!("{}:2", load["id"].as_str().unwrap()));
    }
    raw["model"]["load_cases"].as_array_mut().unwrap().push(second);
    raw
}

/// The multi-case coexistence pin, both modes: exact-block selection settles n05 with two cases,
/// so W1 is not attempted for any case (T-3 (c)): `Coexistence`, exactly the ordinary bytes, no
/// notice and no reservation, on the private driver; and on the actual Direct entry one ordinary
/// run with G-C not consulted (`{runs: 1, complete_gates: 0}`), exactly the ordinary bytes (since
/// I2, C = 3, G-A admits it; in Stale, the same bytes and count without a permit).
#[test]
fn b1_sp_multi_case_coexistence_pin() {
    let raw = n05_two_cases();
    for mode in MODES {
        let plain = plain(mode, &raw);
        let value: Value = serde_json::from_slice(&plain).unwrap();
        assert!(value["source_block_recovery"].is_object(), "{mode:?}: exact blocks selected: {}", value["status"]);
        assert_eq!(value["summary"]["load_case_count"], 2, "{mode:?}");
        let (capture, observer, ordinary) = observed(mode, &raw);
        let capacity = ordinary.diagnostics.capacity();
        let (envelope, retained) = retained_w1(observer, ordinary, &capture);
        assert_eq!(retained.err(), Some(W1Fallback::Coexistence), "{mode:?}: private driver");
        assert_eq!(envelope.diagnostics.capacity(), capacity, "{mode:?}: no reservation");
        assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "{mode:?}: exact bytes, no notice");
        let (output, counts) = direct(&raw, mode);
        if registered() {
            assert_eq!(output.admission().unwrap().law().refusal, None, "{mode:?}: admitted");
            assert_eq!(output.retained().and_then(|r| r.as_ref().err()), Some(&W1Fallback::Coexistence), "{mode:?}");
        } else {
            assert!(output.retained().is_none(), "{mode:?}: Stale, no W1");
        }
        assert_eq!(counts, ONE_RUN, "{mode:?}: G-C not consulted");
        let bytes = published(output);
        assert_eq!(notices(&bytes), 0, "{mode:?}");
        assert_eq!(bytes, plain, "{mode:?}: exactly the ordinary bytes");
    }
}

/// W-C2's cases `ids`, in that order, as one request (two-case requests for RV109's R3P-1).
fn w_c2_cases(ids: &[&str]) -> Value {
    let mut raw = w_c2();
    let all = raw["model"]["load_cases"].as_array().unwrap().clone();
    raw["model"]["load_cases"] = json!(ids.iter().map(|id| all.iter().find(|c| c["id"] == *id).unwrap().clone()).collect::<Vec<_>>());
    raw
}

/// RV109 R3P-1: a multi-case invocation with one case in A takes the n-case transaction on that
/// case's own slot, never the one-case path on the last requested case's fields (`into_single` is
/// retired: c = 1 and c ≥ 2 run the same T-8 to T-11). Both modes, below `retained_w1` (and through
/// it, since I2, with A's one notice):
/// - (A, B), A = {0}: A, the first case, is attempted, solved and frozen on its own source (the call
///   and the one source are A's); B is `not_required`; since I3 the successor is published (before
///   I3 the precommit refused it at G5, with A selected);
/// - (B, A), A = {1}: the same with A second (case-qualified ids in its freeze);
/// - (C, B), A = {0}: C's own Run ends at Ceiling, so no case is selected: `Native`, with nothing
///   selected (not B's unprepared source).
#[test]
fn b1_sp_r3p_1_one_case_in_a_runs_on_its_own_slot() {
    for mode in MODES {
        for (ids, attempted, request) in [(["case-a", "case-b"], 0usize, 0usize), (["case-b", "case-a"], 1, 1), (["case-c", "case-b"], 0, 0)] {
            let label = format!("{mode:?} {ids:?}");
            let raw = w_c2_cases(&ids);
            let plain = plain(mode, &raw);
            let (capture, observer, ordinary) = observed(mode, &raw);
            assert_eq!(classified(&ordinary.numerical_quality, &observer.ordinary, &ids).iter().filter(|t| **t == CaseTrigger::Attempted).count(), 1, "{label}: |A| = 1");
            let ((envelope, outcome), _, successor) = hooks::counted_with_successor(|| w1_transaction(observer, ordinary, &capture, 2, &[attempted]));
            assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "{label}: the ordinary owner is untouched");
            if ids[0] == "case-c" {
                assert_eq!(outcome.err(), Some((W1Fallback::Native, 0)), "{label}: C's own Run, not selected");
                assert!(successor.is_none(), "{label}");
                continue;
            }
            let published = outcome.unwrap_or_else(|f| panic!("{label}: {f:?}"));
            let successor = successor.unwrap();
            assert_eq!(published, successor, "{label}");
            let body = w_c2_body(&successor);
            let statuses: Vec<_> = body["cases"].as_array().unwrap().iter().map(|c| c["status"].as_str().unwrap()).collect();
            let mut expected = ["not_required", "not_required"];
            expected[request] = "selected";
            assert_eq!(statuses, expected, "{label}");
            assert_eq!((body["sources"].as_array().unwrap().len(), body["sources"][0]["owner"]["case_id"].clone()), (1, json!("case-a")), "{label}: A's own source");
            assert_eq!(body["sources"][0]["nodal_terms"].as_array().unwrap().len(), 3, "{label}: A's three loads, not B's two");
            assert_eq!(body["calls"][0]["owner_refs"], json!([case_ref(request)]), "{label}");
        }
        {
            let raw = w_c2_cases(&["case-a", "case-b"]);
            let plain = plain(mode, &raw);
            let (capture, observer, ordinary) = observed(mode, &raw);
            let (envelope, retained) = retained_w1(observer, ordinary, &capture);
            assert!(retained.is_ok(), "{mode:?}: {:?}", retained.err());
            assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "{mode:?}: the ordinary owner beside the successor");
            // T-12's one notice on a fallback: a serializer refusal after A's freeze.
            let (capture, observer, ordinary) = observed(mode, &raw);
            hooks::fail_next_serializer(super::retained_wire::ReceiptCheck::Association);
            let (envelope, _) = retained_w1(observer, ordinary, &capture);
            assert_eq!(serde_json::to_vec(&envelope).unwrap(), with_notice(&plain, "case-a", None), "{mode:?}: A's one notice");
        }
    }
}

/// RV109 R3P-2 and R3P-3: custody validates A (`attempted`) before any attempt: each index in the
/// request and strictly increasing; and a prior capture error is taken in request order (case 0's
/// before case 2's, though case 2 is the one in the capture's own fields).
#[test]
fn b1_sp_r3p_2_3_custody_validates_a_and_orders_prior_errors() {
    use super::retained_product::CaptureError;
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = w_c2();
    let refused = |observer: rp::ProductCapture, ordinary: MechanicsEnvelope, attempted: &[usize]| -> String {
        match observer.prepare_cases(ordinary, 3, attempted) {
            Ok(_) => panic!("custody passed: {attempted:?}"),
            Err(failure) => {
                assert!(failure.capture.prepared_capacity_bytes == [0; 16], "no attempt started: {attempted:?}");
                match failure.error { CaptureError::Association(text) => text, other => format!("{other:?}") }
            }
        }
    };
    for attempted in [&[0, 3][..], &[3], &[2, 0], &[0, 0], &[2, 2]] {
        let (_, observer, ordinary) = observed(mode, &raw);
        assert_eq!(refused(observer, ordinary, attempted), "attempted cases outside the request or out of order", "{attempted:?}");
    }
    let (_, mut observer, ordinary) = observed(mode, &raw);
    observer.with_case(0, |c| c.error = Some("case 0's cause".into()));
    observer.with_case(2, |c| c.error = Some("case 2's cause".into()));
    assert_eq!(refused(observer, ordinary, &[0, 2]), "case 0's cause", "the first in request order");
}

/// RV109 R3P-7: T-6's c ≥ 2 checks, each a refusal before any attempt, on W-C2:
/// - per-case final presence: a case missing its final mode row, or carrying two;
/// - a parked case whose late capture did not complete (`finish` checks every slot);
/// - native work in a parked slot.
#[test]
fn b1_sp_r3p_7_custody_per_case_presence_parked_capture_and_native() {
    use super::retained_product::CaptureError;
    let mode = PreviewSolverMode::DenseScrutiny;
    let raw = w_c2();
    let refused = |observer: rp::ProductCapture, ordinary: MechanicsEnvelope| -> String {
        match observer.prepare_cases(ordinary, 3, &[0, 2]) {
            Ok(_) => panic!("custody passed"),
            Err(failure) => {
                assert!(failure.capture.prepared_capacity_bytes == [0; 16], "no attempt started");
                match failure.error { CaptureError::Association(text) => text, other => format!("{other:?}") }
            }
        }
    };
    let is_mode_of = |r: &ResultItem, case: &str| r.kind == "linear_solver_mode_basis" && r.basis_ref.as_ref().is_some_and(|b| b.ref_id == case);
    // Case B's final mode row missing.
    let (_, observer, mut ordinary) = observed(mode, &raw);
    ordinary.results.retain(|r| !is_mode_of(r, "case-b"));
    assert_eq!(refused(observer, ordinary), "observation final presence", "B without its mode row");
    // Case A's final mode row twice.
    let (_, observer, mut ordinary) = observed(mode, &raw);
    let at = ordinary.results.iter().position(|r| is_mode_of(r, "case-a")).unwrap();
    let copy = ordinary.results[at].clone();
    ordinary.results.insert(at, copy);
    assert_eq!(refused(observer, ordinary), "observation final presence", "A with two mode rows");
    // A parked case's late capture did not complete: `finish` (here re-entered once) refuses it.
    let (_, mut observer, ordinary) = observed(mode, &raw);
    observer.with_case(0, |c| {
        c.prepared_late_calls = 0;
        c.source_capture_entries = 0;
        c.source = None;
    });
    observer.final_calls = 0;
    observer.finish(&ordinary);
    assert_eq!(refused(observer, ordinary), "missing successful prepared late source hook", "a parked case without its late capture");
    // Native work in a parked slot.
    let (request, capture) = source_receipt::CapturedInvocation::parse(u8_two_body_case_a(), mode).unwrap();
    let mut one = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|_| panic!("one-case preparation"));
    let _ = one.solve_native();
    let run = one.test_capture_mut().native.take();
    let (_, mut observer, ordinary) = observed(mode, &raw);
    observer.with_case(1, |c| c.native = run);
    assert_eq!(refused(observer, ordinary), "prepared case custody/permit", "native work in a parked slot");
}

/// The seam's saturation (RR "I89's SA verified and ruled…", ruling 2; R3 ruling 2; RV109 N-3):
/// `late_loads_total` saturates instead of failing the capture, so an overflow is G-B's typed
/// refusal on `CaseLoadsTotal` (observed `u64::MAX`, cap L = 384): the late capture is skipped, no
/// capture error is recorded, and (as for any G-B refusal) the ordinary run is untouched. In the
/// registered build: a permitted probe with the total preset to `usize::MAX`, both modes.
#[test]
fn b1_sp_seam_overflow_is_a_typed_g_b_refusal() {
    use super::retained_memory::{admit, Entry, PhaseFact};
    if !registered() {
        return;
    }
    let raw = raw();
    for mode in MODES {
        let plain = plain(mode, &raw);
        let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let (permit, _report) = admit(&capture, &request, Entry::Direct).unwrap_or_else(|r| panic!("{mode:?}: {:?}", r.law().refusal));
        let mut observer = rp::ProductCapture::permitted_probe(permit);
        observer.late_loads_total = usize::MAX;
        let ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
        let refusal = observer.late_refusal().cloned().unwrap_or_else(|| panic!("{mode:?}: G-B refused"));
        assert_eq!((refusal.fact, refusal.observed, refusal.cap), (PhaseFact::CaseLoadsTotal, u64::MAX, 384), "{mode:?}");
        // The seam records no capture error (before the ruling: CountRange("late loads total")).
        // As after any G-B refusal, the skipped late capture is what `finish` then reports.
        assert_eq!(observer.error.as_ref().map(|e| format!("{e:?}")), Some("Association(\"missing successful prepared late source hook\")".to_owned()),
            "{mode:?}: only the skipped capture's report");
        assert_eq!(observer.late_loads_total, usize::MAX, "{mode:?}: saturated");
        assert_eq!(serde_json::to_vec(&ordinary).unwrap(), plain, "{mode:?}: the ordinary run is untouched");
    }
}

/// RV112 N-4: at c ≥ 2 the first G-B refusal stands, and no later case's late hook runs (no later
/// G-B check, running total or late capture). Registered build, W-C2, both modes, with G-B's
/// fault armed (consumed at case A's late hook): the refusal is A's, the running total holds A's
/// three loads only (not 3 + 2 + 5), and no case has a late capture. The Direct entry publishes
/// exactly the ordinary bytes (`LateGate`), with G-C not consulted.
#[test]
fn b1_sp_first_g_b_refusal_stands_and_stops_later_late_hooks() {
    use super::retained_memory::{admit, Entry};
    if !registered() {
        return;
    }
    let raw = w_c2();
    for mode in MODES {
        let plain = plain(mode, &raw);
        let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let (permit, _report) = admit(&capture, &request, Entry::Direct).unwrap_or_else(|r| panic!("{mode:?}: {:?}", r.law().refusal));
        let mut observer = rp::ProductCapture::permitted_probe(permit);
        hooks::fail_next_late_gate();
        let ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
        assert!(hooks::armed_names().is_empty(), "{mode:?}: the fault fired");
        assert!(observer.late_refusal().is_some(), "{mode:?}: G-B refused");
        assert_eq!(observer.late_loads_total, 3, "{mode:?}: G-B ran once, at case A (3 loads)");
        assert_eq!(observer.cases_seen(), 3, "{mode:?}: every case's early hook ran");
        for index in 0..3 {
            let (late_calls, entries, source) = observer.with_case(index, |c| (c.prepared_late_calls, c.source_capture_entries, c.source.is_some()));
            assert_eq!((entries, source), (0, false), "{mode:?} case {index}: no late capture");
            assert_eq!(late_calls, usize::from(index == 0), "{mode:?} case {index}: only A's late hook ran");
        }
        assert_eq!(serde_json::to_vec(&ordinary).unwrap(), plain, "{mode:?}: the ordinary run is untouched");
        hooks::fail_next_late_gate();
        let (output, counts) = direct(&raw, mode);
        assert!(matches!(output.retained(), Some(Err(W1Fallback::LateGate(_)))), "{mode:?}: {:?}", output.retained());
        assert_eq!((counts, published(output)), (ONE_RUN, plain), "{mode:?}: exact bytes, G-C not consulted");
    }
}

/// D-U6-5 (PLAN_v2 §2.2, after I3): the two W-C2 fixtures are byte-identical copies of the pinned
/// W-C2 successor documents. In the registered build each is compared byte for byte with the
/// document the actual Direct entry publishes; in any other build (no permit), with the private
/// driver's successor document. Either way the fixture carries the pinned hashes.
#[test]
fn b1_sp_w_c2_fixtures_are_the_live_successors() {
    const FIXTURES: [&str; 2] = [
        include_str!("../../../fixtures/results/retained_precision_w_c2_successor_sparse_interactive.json"),
        include_str!("../../../fixtures/results/retained_precision_w_c2_successor_dense_scrutiny.json"),
    ];
    for ((mode, (name, file_sha, receipt_sha, _)), fixture) in MODES.into_iter().zip(W_C2_PINNED).zip(FIXTURES) {
        let raw = w_c2();
        let successor = if registered() {
            let (output, counts) = direct(&raw, mode);
            assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{name}");
            match output.into_publication() {
                RetainedPublication::Successor(value) => value,
                RetainedPublication::Ordinary(_) => panic!("{name}: the registered Direct entry did not publish its successor"),
            }
        } else {
            let (capture, observer, ordinary) = observed(mode, &raw);
            retained_w1(observer, ordinary, &capture).1.unwrap_or_else(|f| panic!("{name}: {f:?}")).value().clone()
        };
        assert!(w_c2_document(name, &raw, &successor) == fixture, "{name}: the W-C2 fixture is the live successor document, byte for byte");
        assert_eq!((sha(fixture.as_bytes()).as_str(), successor["retained_precision"]["receipt_sha256"].as_str()), (file_sha, Some(receipt_sha)),
            "{name}: the pinned W-C2 hashes");
    }
}

// ---- B1 SP after I3 (I85): RV109's SF-2, and the nodal-term ordinal ------------------------------
//
// RR "RV109 passes SP in RV-P round 2; …", ruling 2, and RR "I98's B2-W verified; …".

/// W-C2's cases and RV109's A2 (a copy of case A, `case-a2`, its load ids suffixed `:a2`), the
/// cases `ids`, in that order, as one request.
fn w_c2_plus_a2(ids: &[&str]) -> Value {
    let mut raw = w_c2();
    let mut a2 = raw["model"]["load_cases"][0].clone();
    a2["id"] = json!("case-a2");
    for load in a2["primitive_loads"].as_array_mut().unwrap() {
        load["id"] = json!(format!("{}:a2", load["id"].as_str().unwrap()));
    }
    raw["model"]["load_cases"].as_array_mut().unwrap().push(a2);
    let all = raw["model"]["load_cases"].as_array().unwrap().clone();
    raw["model"]["load_cases"] = json!(ids.iter().map(|id| all.iter().find(|c| c["id"] == *id).unwrap().clone()).collect::<Vec<_>>());
    raw
}
/// The pinned successors of RV109's SF-2 inputs: (mode, receipt sha256, successor bytes sha256).
const CBA_PINNED: [(&str, &str, &str); 2] = [
    ("sparse_interactive", "863d692fa90d450240cbfacec1628937b8ccc9af2396637d4f913cc0bc416e80", "ea9a484657ca3de9a831a737b6a682966cc4b1a8783114b26298f34571cc7ebb"),
    // Re-pinned by I109 (PR-N): W-C2 case C's correctly rounded `rigid:N0` magnitude; every platform.
    ("dense_scrutiny", "255785d20cf0aa9f497ea324d744eb3e946871d5aac863ed8d0081d0521e8c92", "a320a5d33707c1fc8c12a35de624720dddbc35084979522c96ab1906e17c708f"),
];
const AA2_PINNED: [(&str, &str, &str); 2] = [
    ("sparse_interactive", "41f330856d4c6e94e2e1308fcd467818604c8f7e999ce499c8f3b49e26ef0d56", "529233eb989aca3553bdedfc9d1813ab3287675748ff076c27cb024c2b7fef63"),
    ("dense_scrutiny", "30001ccf42ad12acd9dbb458392fee514946c1a52aa0d4e5f0b20bde5b09ea71", "f4075cdc80eff27099a28f787cec07080b745eca2d598eb3381a84ec03964176"),
];

/// RV109's SF-2 (RR "RV109 passes SP in RV-P round 2; …", ruling 2): two successors pinned in both
/// modes, through `retained_w1` (any build), where the selected case is not first, and where two
/// cases are selected:
/// - **(C, B, A):** A = {C, A}; C `unavailable` (its Ceiling Run), B `not_required`, A `selected`
///   as request 2, attempt 1 and batch ordinal 1;
/// - **(A, A2):** both `selected`, attempts 0 and 1.
///
/// Each selected case's Run owner is its request index (N-2; M19); its product attempt's id and its
/// source's `preparation.attempt_ref` are its attempt id (T-7, T-11; M16, M17). The
/// `RETAINED_PRECISION_*` diagnostics are the selected cases' in request order, then the
/// unavailable cases' (T-11; M15, which the reader accepts). Each summary headline is the
/// governing row over the staged rows (M28: (C, B, A)'s displacement tie between B and C, won by
/// the smaller case id; M32: (A, A2)'s restaged displacement). The accepted Rust reader passes with
/// the invocation, the ordinary owner is untouched, and N-16 holds record by record
/// (`n16_batch_against_one_case`).
#[test]
fn b1_sp_sf2_selected_not_first_and_two_selected_pins() {
    let a2_alone = w_c2_plus_a2(&["case-a2"]);
    let inputs: [(&str, Value, Vec<usize>, Vec<&str>, [(&str, &str, &str); 2], Vec<Value>); 2] = [
        ("c_b_a", w_c2_cases(&["case-c", "case-b", "case-a"]), vec![0, 2], vec!["unavailable", "not_required", "selected"], CBA_PINNED,
            vec![w_c2_case_c(), u8_two_body_case_a()]),
        ("a_a2", w_c2_plus_a2(&["case-a", "case-a2"]), vec![0, 1], vec!["selected", "selected"], AA2_PINNED, vec![u8_two_body_case_a(), a2_alone]),
    ];
    for (name, raw, attempted, statuses, pins, singles) in inputs {
        for (mode, (mode_name, receipt_sha, bytes_sha)) in MODES.into_iter().zip(pins) {
            assert_eq!(mode.as_str(), mode_name);
            let label = format!("{name} {mode_name}");
            let plain = plain(mode, &raw);
            let (capture, observer, ordinary) = observed(mode, &raw);
            let ((envelope, retained), counts, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
            assert_eq!(counts, Counts { runs: 0, complete_gates: 0 }, "{label}");
            // The structure is asserted on the successor precommit received, before its validation
            // and before the outcome (a refused successor leaves a fallback notice on the ordinary
            // owner), so that a defect the reader also refuses is killed here by its own assertion.
            let successor = captured.unwrap_or_else(|| panic!("{label}: precommit received the successor"));
            let body = w_c2_body(&successor);
            let cases = body["cases"].as_array().unwrap();
            assert_eq!(cases.iter().map(|c| c["status"].as_str().unwrap()).collect::<Vec<_>>(), statuses, "{label}");
            let attempts = body["product_attempts"].as_array().unwrap();
            assert_eq!(attempts.iter().map(|a| a["id"].clone()).collect::<Vec<_>>(), (0..attempted.len()).map(|k| json!(k)).collect::<Vec<_>>(),
                "{label}: product attempt ids in start order (M17)");
            for (attempt, &request) in attempted.iter().enumerate() {
                let case = &cases[request];
                assert_eq!(case["product_attempt_ref"], json!(attempt), "{label} request {request}");
                assert_eq!(attempts[attempt]["owner_ref"], case_ref(request), "{label} request {request}");
                assert_eq!(case["run"]["origin"]["owner_ref"], case_ref(request), "{label} request {request}: the Run's owner is the request index (N-2; M19)");
                let source = &body["sources"][case["source_ref"].as_u64().unwrap() as usize];
                assert_eq!((source["owner"]["case_index"].clone(), source["preparation"]["attempt_ref"].clone()), (json!(request), json!(attempt)),
                    "{label} request {request}: its source's owner and attempt (T-7; M16)");
            }
            // T-11's staging order: the selected cases' diagnostics in request order, then the unavailable cases'.
            let ids: Vec<&str> = raw["model"]["load_cases"].as_array().unwrap().iter().map(|c| c["id"].as_str().unwrap()).collect();
            let expected: Vec<String> = ["selected", "unavailable"].iter().flat_map(|status| {
                ids.iter().zip(&statuses).filter(move |(_, s)| *s == status).map(move |(id, _)| format!("diagnostic:retained-precision:{id}:{status}"))
            }).collect();
            let staged: Vec<&str> = successor["diagnostics"].as_array().unwrap().iter()
                .filter(|d| d["code"].as_str().is_some_and(|c| c.starts_with("RETAINED_PRECISION_"))).map(|d| d["id"].as_str().unwrap()).collect();
            assert_eq!(staged, expected, "{label}: T-11's staging order (M15)");
            // T-11's headlines: the governing row over the staged rows.
            let ordinary: Value = serde_json::from_slice(&plain).unwrap();
            for (kind, headline) in [("displacement_magnitude", "max_displacement"), ("pipe_elastic_normal_stress_maximum_v2", "max_open_formula_stress")] {
                let best = successor["results"].as_array().unwrap().iter().filter(|r| r["kind"] == kind)
                    .max_by(|a, b| a["value"].as_f64().unwrap().total_cmp(&b["value"].as_f64().unwrap())
                        .then_with(|| b["basis_ref"]["ref_id"].as_str().cmp(&a["basis_ref"]["ref_id"].as_str()))
                        .then_with(|| b["entity_ref"].as_str().cmp(&a["entity_ref"].as_str()))).unwrap();
                assert_eq!(successor["summary"][headline], json!({"value":best["value"],"unit":best["unit"],"location_ref":best["entity_ref"],"result_ref":best["id"]}),
                    "{label}: the {headline} headline (M28, M32)");
                println!("B1_SP_SF2_HEADLINE {label} {headline} successor={} ordinary={}", successor["summary"][headline], ordinary["summary"][headline]);
            }
            assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "{label}: the ordinary owner is untouched");
            let validated = retained.unwrap_or_else(|f| panic!("{label}: {f:?}")).value().clone();
            assert_eq!(validated, successor, "{label}: the successor precommit validated");
            let invocation = json!({"request": raw, "solver_mode": mode_name});
            assert!(open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)).is_ok(), "{label}: the Rust reader");
            let bytes = serde_json::to_vec(&successor).unwrap();
            println!("B1_SP_SF2_PIN {label} {} {}", successor["retained_precision"]["receipt_sha256"].as_str().unwrap(), sha(&bytes));
            assert_eq!((successor["retained_precision"]["receipt_sha256"].as_str(), sha(&bytes).as_str()), (Some(receipt_sha), bytes_sha), "{label}: the pinned successor");
            n16_batch_against_one_case(&label, &raw, mode, ids.len(), &attempted, &singles);
            assert!(hooks::armed_names().is_empty(), "{label}");
        }
    }
}

/// I98's `cause_milestone_reversed` (R/I98/b2_w_probe_01/PROBE.md §7): the milestone with its
/// three moments authored RZ, RY, RX, so its authored order is not the kernel's canonical order.
fn milestone_reversed() -> Value {
    let mut raw = raw();
    raw["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().reverse();
    raw
}
/// Its pinned successor: (mode, receipt sha256, published bytes sha256).
const REVERSED_PINNED: [(&str, &str, &str); 2] = [
    ("sparse_interactive", "b79f691a4a899e3ed4a7d13957f74d4c4ee7430ee3062396214261144cbabd82", "93aa043538bbc01e0a7bcef4e381f88468edfd6792769b7041d7f6483ad959b1"),
    ("dense_scrutiny", "c6b03683c8a2591f4d59429407608a6aafd0abf25663c7d771de60f221135832", "b70edc6d002c92692f00a247702353f79e6410acf17eb385b3d2c2500ca76d8e"),
];

/// The nodal-term ordinal (C2 `CONTRACT_DELTA`:104; RR "I98's B2-W verified; …"): a CaseSource's
/// `constructor_ordinal` is the term's ordinal in the constructor's input, its authored
/// primitive-load index, and the array stays in kernel canonical order. On the reversed milestone,
/// both modes, the successor precommit receives has `constructor_ordinal == primitive_load_index`
/// for every term, in canonical order (indices 2, 1, 0); the reader accepts it, and the Direct
/// entry publishes it (registered: one run, G-C once) with its pinned bytes. Before the fix the
/// ordinal was the canonical position and every reader refused at G8 `PREPARATION_MISMATCH`.
#[test]
fn b1_sp_constructor_ordinal_is_the_authored_index() {
    for (mode, (name, receipt_sha, bytes_sha)) in MODES.into_iter().zip(REVERSED_PINNED) {
        assert_eq!(mode.as_str(), name);
        let raw = milestone_reversed();
        let plain = plain(mode, &raw);
        let (capture, observer, ordinary) = observed(mode, &raw);
        let ((_, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
        let captured = captured.unwrap_or_else(|| panic!("{name}: precommit received the successor"));
        let terms = captured["retained_precision"]["body"]["sources"][0]["nodal_terms"].as_array().unwrap();
        assert_eq!(terms.iter().map(|t| (t["constructor_ordinal"].clone(), t["primitive_load_index"].clone())).collect::<Vec<_>>(),
            [(json!(2), json!(2)), (json!(1), json!(1)), (json!(0), json!(0))], "{name}: the authored index, in canonical order");
        let successor = retained.unwrap_or_else(|f| panic!("{name}: {f:?}")).value().clone();
        assert_eq!(successor, captured, "{name}");
        let invocation = json!({"request": raw, "solver_mode": name});
        assert!(open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)).is_ok(), "{name}: the Rust reader");
        let bytes = serde_json::to_vec(&successor).unwrap();
        println!("B1_SP_REVERSED_PIN {name} {} {}", successor["retained_precision"]["receipt_sha256"].as_str().unwrap(), sha(&bytes));
        assert_eq!((successor["retained_precision"]["receipt_sha256"].as_str(), sha(&bytes).as_str()), (Some(receipt_sha), bytes_sha), "{name}: the pinned successor");
        let (output, counts) = direct(&raw, mode);
        if registered() {
            assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{name}");
            assert_eq!(published(output), bytes, "{name}: the Direct entry publishes the successor");
        } else {
            assert_eq!((counts, published(output)), (ONE_RUN, plain), "{name}: Stale's plain bytes");
        }
    }
}

// ---- B3b-P: the exact route under `physics-retained-1` (I93 PLAN §1.4; B3-D with REVISION_01) ----
//
// RR "I99's B3-W verified; B3's witnesses selected; …": the exact successor is `m3x` (the
// milestone authored as 0.3.0 exact), the coexistence pins are n05 and n06 (with `fields` for
// P-2), and the mixed exact base is `m3x_mix_anchor`. B3a is dropped; `m3l` is its refusal witness.

/// `raw` authored as 0.3.0 exact (I99's `gen_inputs.py`, item 1): the exact contract; each
/// material's shear modulus removed, with the common E/ν basis and ν = 0.25; explicitly empty
/// pressure regions on every case. On the milestone this is B3-W's `m3x`.
fn exact3(mut raw: Value) -> Value {
    let m = &mut raw["model"];
    m["schema_version"] = json!("0.3.0");
    m["pressure_contract"] = json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"});
    for material in m["materials"].as_array_mut().unwrap() {
        material.as_object_mut().unwrap().remove("shear_modulus");
        material["constitutive_basis"] = json!("homogeneous_isotropic_E_nu_v1");
        material["poisson_ratio"] = json!({"value": 0.25, "unit": "1"});
    }
    for case in m["load_cases"].as_array_mut().unwrap() {
        case["pressure_regions"] = json!([]);
    }
    raw
}
/// B3-W's `m3x`.
fn m3x() -> Value { exact3(raw()) }

/// B3-W's mixed exact base `m3x_mix_anchor` (I99 item 2, variant 2): the milestone with a second
/// case `case:b`, a 1 N global-X force on N0, whose translations are rigid (a zero response, so
/// `case:b` is `checks_passed`: `not_required`), authored as 0.3.0 exact.
fn m3x_mix_anchor() -> Value {
    let mut raw = raw();
    raw["model"]["load_cases"].as_array_mut().unwrap().push(json!({"id": "case:b", "label": "I99 B3-W second case (anchor)",
        "kind": "primitive_user_load", "primitive_loads": [{"id": "load:b:0", "category": "concentrated_force", "target": {"type": "node", "node": "N0"},
        "direction": "global_x", "magnitude": {"value": 1.0, "unit": "N"}, "dimension": "force", "provenance": "invented_t3_p1_detection_input_no_library_data"}],
        "provenance": "invented_t3_p1_detection_input_no_library_data"}));
    exact3(raw)
}
/// B3-W's `m3l` (B3a): the milestone authored as 0.3.0 `legacy_pressure_v1` with zero pressure.
/// B3a is dropped, so `m3l` is a refusal witness (D1.3 `PressureContract`; no W1 route).
fn m3l() -> Value {
    let mut raw = raw();
    raw["model"]["schema_version"] = json!("0.3.0");
    raw["model"]["pressure_contract"] = json!({"version": "1.0.0", "mode": "legacy_pressure_v1"});
    raw
}
/// The committed physics-source requests: B3b's coexistence pins n05 and n06, and P-2's
/// selection discriminator `fields` (RR "I99's B3-W verified; …", rulings 1 and 2).
const N05_EXACT: &str = include_str!("../../../fixtures/product_preview/physics_source/n05.request.json");
const N06_EXACT: &str = include_str!("../../../fixtures/product_preview/physics_source/n06.request.json");
const FIELDS_EXACT: &str = include_str!("../../../fixtures/product_preview/physics_source/fields.request.json");
fn model_of(raw: &Value) -> PreviewModel {
    source_receipt::CapturedInvocation::parse(raw.clone(), PreviewSolverMode::SparseInteractive).unwrap().0.model
}

/// The witnesses are B3-W's (I99 §2: the Value sha256 of each built input; the committed
/// requests' file sha256), and P-1's route is decided from the namespace branch: L on the
/// preview route, E on the exact route, a load-state or 0.4.0 model, and B3a's dropped `m3l`, on
/// none.
#[test]
fn b3b_witness_inputs_and_routes() {
    use rp::W1Route as R;
    let value_sha = |v: &Value| sha(&serde_json::to_vec(v).unwrap());
    assert_eq!(value_sha(&m3x()), "c920a96dc542c3ecadb63f724cfcf243d73d5dd71692e4f48827a620bb0e5497", "m3x");
    assert_eq!(value_sha(&m3x_mix_anchor()), "6ca777a6e3ed658bcf58813d277e839033ac07f0b84efb8526351a20c67d1ee0", "m3x_mix_anchor");
    assert_eq!(value_sha(&m3l()), "2f5ff465bfa97005a581d88e02c0a5478ea24da3bdc1c2eb9fcda491803fc0c3", "m3l");
    for (name, text, file_sha) in [("n05", N05_EXACT, "332319ee6f47870a074a6c16fbf2f43c0171e376a0b4cd1cb7f7ad8b59254b00"),
        ("n06", N06_EXACT, "5551f164b9e8ab88f04f3abce810e1e1b1ff47236bba6a296e32905f628be931"),
        ("fields", FIELDS_EXACT, "7f8ff9d5e23712cedd6ca58a86a690517a8e820b9b072962ca43ad60be286002")] {
        assert_eq!(sha(text.as_bytes()), file_sha, "{name}");
        assert_eq!(w1_route(&model_of(&serde_json::from_str(text).unwrap())), Some(R::Exact), "{name}");
    }
    assert_eq!(w1_route(&model_of(&raw())), Some(R::Preview), "L");
    assert_eq!(w1_route(&model_of(&m3l())), None, "m3l (B3a dropped): no branch");
    assert_eq!(w1_route(&model_of(&m3x())), Some(R::Exact), "E");
    assert_eq!(w1_route(&model_of(&m3x_mix_anchor())), Some(R::Exact), "E, two cases");
    let mut four = m3x();
    four["model"]["schema_version"] = json!("0.4.0");
    assert_eq!(w1_route(&model_of(&four)), None, "0.4.0 (a load-state document): no W1 route");
    let mut contractless = m3x();
    contractless["model"]["pressure_contract"] = Value::Null;
    assert_eq!(w1_route(&model_of(&contractless)), None, "0.3.0 without a contract: no branch");
    // P-2: the exact route's budget is the ordinary route's.
    assert_eq!(w1_budget(R::Exact).per_case_limit, PHYSICS_SOURCE_WORK_LIMIT);
    assert_eq!(w1_budget(R::Preview).per_case_limit, SourceRecoveryBudget::default().per_case_limit);
}

/// RS's precommit gates publication (decision 5). B3's readers are in, so the Rust reader
/// validates every `physics-retained-1` successor the tests below produce.
/// The exact successor's pinned bytes, both modes: (mode, receipt sha256, successor bytes sha256,
/// fixture document sha256).
const EXACT_PINNED: [(&str, &str, &str, &str); 2] = [
    ("sparse_interactive", "b1b4a6682260ca6bc499950b30f0f7179a77c038e266cc4b42045ed86ed3896f", "f18227f7c5eb10d849b0499c7afd139ba2aab493bbdc581805db3ba3ebd4b20d", "02465c6c92ac2e4360a77910cb54803590b5a11042dfddb223bf78f9e856e5d6"),
    ("dense_scrutiny", "eabd2fc57b42158ad415ae664e7c712c1c4db21258b667251758172f3a5b776d", "e31f03a45d8028f32ae518d2004e97423d33139c3c807700755bcbe0113cdb2a", "31f10f04f6f335dfb1a7e5f904198972903bfc9208660031bbfaa5c547d347cc"),
];
/// The mixed exact base's pinned successor: (mode, receipt sha256, successor bytes sha256).
const MIX_PINNED: [(&str, &str, &str); 2] = [("sparse_interactive", "71703ab120645b7c7b903a072b5e2cc95b72e0a0759db75f3b4b7bcfbe7cb29f", "ca2cd75096cee1fe318d2623ae5c4737438e300ab92dbe8f0f61310d19c4a53b"), ("dense_scrutiny", "b5cf5a4f42bc4e4d1b576250096cf18875c8601fb29b92e6ce0f2fdd86a0eec3", "a50c530faba48a8d0afe87b79ac7c85771a529fd3ce4fbef251189f13c621337")];
/// The exact successor's fixture document (D-U6-5's form).
fn exact_document(raw: &Value, mode: PreviewSolverMode, successor: &Value) -> String {
    serde_json::to_string_pretty(&json!({"id":format!("b3b_m3x_{}", mode.as_str()),"source":successor,
        "invocation":{"request":raw,"solver_mode":mode.as_str()}})).unwrap()
}
/// The private driver's exact transaction (`retained_w1` after the observed run, as the facade
/// runs it): the ordinary owner it returns, W1's result, and the successor precommit received.
fn exact_w1(raw: &Value, mode: PreviewSolverMode) -> (MechanicsEnvelope, Result<RetainedSuccessor, W1Fallback>, Option<Value>) {
    let (capture, observer, ordinary) = observed(mode, raw);
    let ((envelope, retained), counts, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
    assert_eq!(counts, Counts { runs: 0, complete_gates: 0 });
    (envelope, retained, captured)
}
/// W1's result: the validated successor, equal to the one precommit received, with the ordinary
/// owner untouched (B3's readers validate it). Returns whether the successor was published.
fn exact_outcome(label: &str, retained: &Result<RetainedSuccessor, W1Fallback>, captured: &Value, envelope: &MechanicsEnvelope,
    plain: &[u8], _noticed: &[&str]) -> bool {
    match retained {
        Ok(successor) => {
            assert_eq!(successor.value(), captured, "{label}: the validated successor is the one precommit received");
            assert_eq!(serde_json::to_vec(envelope).unwrap(), plain, "{label}: the ordinary owner is untouched");
            println!("B3B_PRECOMMIT {label} validated");
            true
        }
        Err(other) => panic!("{label}: B3's RS reader validates the exact successor: {other:?}"),
    }
}
/// RN64(E/(2·RN64(1+ν))) (B3-D §1.2).
fn derived_g(e: f64, nu: f64) -> f64 { e / (2.0 * (1.0 + nu)) }
fn hex_bits(v: &Value) -> u64 { u64::from_str_radix(v.as_str().unwrap(), 16).unwrap() }
fn num_bits(v: &Value) -> u64 { v.as_f64().unwrap().to_bits() }
/// C3 §2's preparation payload of a product attempt, with the definition hash `h` (built here
/// from the receipt, independently of the serializer).
fn preparation_hash(attempt: &Value, h: &str) -> String {
    let members: Vec<Value> = attempt["preparation"]["members"].as_array().unwrap().iter().map(|m| json!({
        "member":m["member"],"old_source":m["old_source"],"old_facts":m["old_facts"],"section":m["result"]["section"]})).collect();
    super::retained_wire::domain_hash("retained_precision_preparation_v1", &json!({"definition_id":attempt["definition_id"],"definition_sha256":h,
        "owner_ref":attempt["owner_ref"],"ordinary_attempt_ref":attempt["ordinary_attempt_ref"],"material_basis_ref":attempt["material_basis_ref"],"members":members})).unwrap()
}
/// B3-D's producer requirements on one exact successor (P-3, P-5 to P-9, P-11; §1.4; REVISION_01
/// S-1, S-2 and N-6), against the ordinary exact envelope `plain` of the same invocation.
/// `statuses` are the cases' receipt statuses in request order.
fn assert_exact_successor(label: &str, successor: &Value, plain: &Value, statuses: &[&str]) {
    use super::retained_wire as wire;
    // P-9: the identity and profile; the profile's limitations are the base producer's.
    assert_eq!(successor["producer"]["semantic_contract_id"], json!(wire::EXACT_SEMANTIC_ID), "{label}");
    assert_eq!(successor["formulation_basis"]["profile_id"], json!(wire::EXACT_PROFILE_ID), "{label}");
    assert_eq!(successor["formulation_basis"]["limitations"], plain["formulation_basis"]["limitations"], "{label}: limitations unchanged");
    let body = &successor["retained_precision"]["body"];
    let cases = body["cases"].as_array().unwrap();
    assert_eq!(cases.iter().map(|c| c["status"].as_str().unwrap()).collect::<Vec<_>>(), statuses, "{label}");
    // P-9 and S-1: every attempt is DEF-E's, and every prepared source's preparation hash is made
    // with DEF-E's H, not DEF-O's.
    for attempt in body["product_attempts"].as_array().unwrap() {
        assert_eq!(attempt["definition_id"], json!(wire::EXACT_DEFINITION_ID), "{label}");
    }
    for source in body["sources"].as_array().unwrap() {
        let attempt = &body["product_attempts"][source["preparation"]["attempt_ref"].as_u64().unwrap() as usize];
        assert_eq!(source["preparation"]["sha256"], json!(preparation_hash(attempt, wire::EXACT_DEFINITION_SHA256)), "{label}: S-1");
        assert_ne!(source["preparation"]["sha256"], json!(preparation_hash(attempt, wire::DEFINITION_SHA256)), "{label}: not DEF-O's H");
        for term in source["section_terms"].as_array().unwrap() {
            assert_eq!(term["geometry"]["route"], json!("exact"), "{label}: P-9");
        }
    }
    // P-3 and P-9: the material basis: E, Ĝ = RN64(E/(2·RN64(1+ν))) and the derived origin.
    let materials = body["material_bases"][0]["materials"].as_array().unwrap();
    assert!(!materials.is_empty(), "{label}");
    for m in materials {
        let input = &plain["contract_evidence"]["exact_cases"][0]["pipe_materials"].as_array().unwrap().iter()
            .find(|x| x["material_id"] == m["id"]).unwrap().clone();
        let (e, nu) = (input["E_pa"].as_f64().unwrap(), input["nu"].as_f64().unwrap());
        assert_eq!(hex_bits(&m["elastic_modulus"]), e.to_bits(), "{label}");
        assert_eq!(hex_bits(&m["shear_modulus"]), derived_g(e, nu).to_bits(), "{label}: Ĝ");
        assert_eq!(m["shear_origin"], json!({"kind":"derived_e_nu","poisson_ratio":format!("{:016x}", nu.to_bits()),
            "constitutive_basis":"homogeneous_isotropic_E_nu_v1"}), "{label}");
        assert_eq!(m["selection"], json!({"kind":"base"}), "{label}");
    }
    // P-11 (and C2 D39): every legacy exact-block attempt is physics-source-1's, under P-2's budget.
    for work in body["legacy_source_work"].as_array().unwrap() {
        assert_eq!(work["limit"], json!(PHYSICS_SOURCE_WORK_LIMIT), "{label}: P-2's limit in the receipt");
    }
    // The evidence (B3D-4, S-2): pressure and connector unchanged; each case's entry, located by
    // its load case id.
    let (ev, base) = (&successor["contract_evidence"], &plain["contract_evidence"]);
    assert_eq!((&ev["pressure"], &ev["connector"]), (&base["pressure"], &base["connector"]), "{label}");
    assert_eq!(ev.as_object().unwrap().keys().collect::<Vec<_>>(), base.as_object().unwrap().keys().collect::<Vec<_>>(), "{label}");
    for (index, case) in cases.iter().enumerate() {
        let id = case["basis_ref"]["ref_id"].as_str().unwrap();
        let entry = |e: &Value| e["exact_cases"].as_array().unwrap().iter().find(|c| c["load_case_id"] == id).unwrap().clone();
        let (now, was) = (entry(ev), entry(base));
        if case["status"] != "selected" {
            assert_eq!(now, was, "{label} {id}: an unselected case's entry is byte-identical (S-2)");
            assert!(successor["results"].as_array().unwrap().iter().filter(|r| r["basis_ref"]["ref_id"] == id).all(|r| r.get("recovery_method").is_none()));
            continue;
        }
        for key in was.as_object().unwrap().keys().filter(|k| !["pipe_sections", "pipe_stress_extrema"].contains(&k.as_str())) {
            assert_eq!(now[key], was[key], "{label} {id}: `{key}` byte-identical (DEF-E evidence.unchanged)");
        }
        assert!(now.get("recovery_method").is_none(), "{label} {id}");
        // N-6: the published G is the receipt's Ĝ, bit for bit.
        for x in now["pipe_materials"].as_array().unwrap() {
            let m = materials.iter().find(|m| m["id"] == x["material_id"]).unwrap();
            assert_eq!(num_bits(&x["G_pa"]), hex_bits(&m["shear_modulus"]), "{label} {id}: N-6");
        }
        // P-8 and G5b: each section is the prepared one, equal to the receipt's section terms; OD,
        // wall, the radii, Ai, the basis and the order are unchanged.
        let source = &body["sources"][case["source_ref"].as_u64().unwrap() as usize];
        let attempt = &body["product_attempts"][case["product_attempt_ref"].as_u64().unwrap() as usize];
        let (sections, old) = (now["pipe_sections"].as_array().unwrap(), was["pipe_sections"].as_array().unwrap());
        assert_eq!(sections.len(), old.len(), "{label}");
        for (s, o) in sections.iter().zip(old) {
            for key in o.as_object().unwrap().keys().filter(|k| !["As_m2", "I_m4", "J_m4", "Z_m3"].contains(&k.as_str())) {
                assert_eq!(s[key], o[key], "{label} {id}: pipe_sections `{key}` unchanged");
            }
            let member = source["id_maps"]["members"].as_array().unwrap().iter().find(|m| m["id"] == s["pipe_id"]).unwrap()["kernel_member"].as_u64().unwrap();
            let term = &source["section_terms"][member as usize];
            let prepared = &attempt["preparation"]["members"].as_array().unwrap().iter().find(|m| m["member"] == json!(member)).unwrap()["result"]["section"];
            let g = &term["geometry"];
            for (key, bits) in [("As_m2", hex_bits(&term["area"])), ("Z_m3", hex_bits(&term["section_modulus"])), ("I_m4", hex_bits(&g["actual_second_moment"])),
                ("J_m4", hex_bits(&g["actual_polar_moment"])), ("outside_diameter_m", hex_bits(&g["normalized_od"])),
                ("effective_wall_thickness_m", hex_bits(&g["effective_wall"])), ("ro_m", hex_bits(&g["actual_radius"]))] {
                assert_eq!(num_bits(&s[key]), bits, "{label} {id}: G5b `{key}`");
            }
            for (k, key) in ["As_m2", "I_m4", "J_m4", "Z_m3"].into_iter().enumerate() {
                assert_eq!(num_bits(&s[key]), hex_bits(&prepared[k]), "{label} {id}: `{key}` is the prepared value");
            }
            println!("B3B_SECTION {label} {id} {} old=[{:016x},{:016x},{:016x},{:016x}] prepared=[{:016x},{:016x},{:016x},{:016x}]", s["pipe_id"],
                num_bits(&o["As_m2"]), num_bits(&o["I_m4"]), num_bits(&o["J_m4"]), num_bits(&o["Z_m3"]),
                num_bits(&s["As_m2"]), num_bits(&s["I_m4"]), num_bits(&s["J_m4"]), num_bits(&s["Z_m3"]));
        }
        // P-7: one extremum per member, its other keys unchanged; its row's value is the midpoint.
        let (extrema, old) = (now["pipe_stress_extrema"].as_array().unwrap(), was["pipe_stress_extrema"].as_array().unwrap());
        assert_eq!(extrema.len(), old.len(), "{label}");
        for (x, o) in extrema.iter().zip(old) {
            for key in o.as_object().unwrap().keys().filter(|k| !PREPARED_MAX_KEYS_TEST.contains(&k.as_str())) {
                assert_eq!(x[key], o[key], "{label} {id}: pipe_stress_extrema `{key}` unchanged");
            }
            let row = successor["results"].as_array().unwrap().iter().find(|r| r["id"] == x["result_id"]).unwrap();
            let (lo, hi) = (x["value_lower_pa"].as_f64().unwrap(), x["value_upper_pa"].as_f64().unwrap());
            assert_eq!(row["value"].as_f64().unwrap().to_bits(), (lo + 0.5 * (hi - lo)).to_bits(), "{label} {id}: the maximum is the midpoint");
        }
        // The rows (C1 §4) and diagnostics (G-a; T1 (a), P-11).
        assert!(successor["results"].as_array().unwrap().iter().filter(|r| r["basis_ref"]["ref_id"] == id)
            .all(|r| r["recovery_method"] == json!(wire::METHOD)), "{label} {id}");
        let diagnostics = successor["diagnostics"].as_array().unwrap();
        assert!(!diagnostics.iter().any(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE" && d["affected_refs"].as_array().unwrap().iter().any(|r| r == id)),
            "{label} {id}: P-11 omits the legacy disclosure of a selected case");
        assert_eq!(diagnostics.iter().filter(|d| d["code"] == wire::SELECTED_CODE && d["affected_refs"] == json!([id])).count(), 1, "{label} {id}");
        let _ = index;
    }
}
const PREPARED_MAX_KEYS_TEST: [&str; 8] = ["station_fraction", "span_index", "local_fraction", "value_lower_pa", "value_upper_pa",
    "global_upper_bound_pa", "certified_gap_pa", "subdivisions"];

/// B3b-P's exact successor (P-13; B3-D §5 and REVISION_01): `m3x` in both modes, through the
/// private driver. Its one case is `selected` (native Selected, the dual-readout certificate, the
/// observables and G5a pass), and the successor precommit receives meets B3-D's requirements and
/// is pinned; the ordinary owner is untouched. Today RS refuses it at G0 (no reader yet), so W1
/// falls back with one notice; the pins hold either way. `I105_B3B_OUT` writes the documents.
#[test]
fn b3b_exact_successor_is_pinned_in_both_modes() {
    let out = std::env::var("I105_B3B_OUT").ok().map(std::path::PathBuf::from);
    for (mode, (name, receipt_sha, bytes_sha, document_sha)) in MODES.into_iter().zip(EXACT_PINNED) {
        assert_eq!(mode.as_str(), name);
        let raw = m3x();
        let plain = plain(mode, &raw);
        let plain_value: Value = serde_json::from_slice(&plain).unwrap();
        assert_eq!(plain_value["producer"]["semantic_contract_id"], json!(PHYSICS_SEMANTIC_CONTRACT_ID), "{name}: the physics-1 base");
        assert!(plain_value["source_block_recovery"].is_null(), "{name}: no exact-block selection (T-3 (c) does not fire)");
        let (envelope, retained, captured) = exact_w1(&raw, mode);
        let successor = captured.unwrap_or_else(|| panic!("{name}: {retained:?}"));
        exact_outcome(name, &retained, &successor, &envelope, &plain, &["case"]);
        assert_exact_successor(name, &successor, &plain_value, &["selected"]);
        assert_eq!(successor["results"].as_array().unwrap().len(), if mode == PreviewSolverMode::DenseScrutiny { 99 } else { 98 }, "{name}: I96 §3's rows");
        let bytes = serde_json::to_vec(&successor).unwrap();
        let document = exact_document(&raw, mode, &successor);
        println!("B3B_EXACT_PIN {name} {} {} {}", successor["retained_precision"]["receipt_sha256"].as_str().unwrap(), sha(&bytes), sha(document.as_bytes()));
        assert_eq!((successor["retained_precision"]["receipt_sha256"].as_str(), sha(&bytes).as_str(), sha(document.as_bytes()).as_str()),
            (Some(receipt_sha), bytes_sha, document_sha), "{name}: the pinned exact successor");
        if let Some(dir) = &out {
            std::fs::write(dir.join(format!("retained_precision_exact_successor_{name}.json")), &document).unwrap();
        }
        assert!(hooks::armed_names().is_empty());
    }
}

/// D-U6-5's form for the new fixtures: `retained_precision_exact_successor_{mode}.json` are,
/// byte for byte, the live exact successor documents (the successor the transaction produces,
/// on the private driver; on the registered Direct entry the same bytes reach precommit).
#[test]
fn b3b_exact_successor_fixtures_are_the_live_successors() {
    const FIXTURES: [&str; 2] = [
        include_str!("../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json"),
        include_str!("../../../fixtures/results/retained_precision_exact_successor_dense_scrutiny.json"),
    ];
    for ((mode, (name, _, _, document_sha)), fixture) in MODES.into_iter().zip(EXACT_PINNED).zip(FIXTURES) {
        let raw = m3x();
        let (_, _, captured) = exact_w1(&raw, mode);
        let document = exact_document(&raw, mode, &captured.unwrap());
        assert!(document == fixture, "{name}: the fixture is the live exact successor document, byte for byte");
        assert_eq!(sha(fixture.as_bytes()), document_sha, "{name}");
    }
}

/// The exact route on the actual Direct entry (registered: admitted at J2 by B3b-A): one
/// ordinary run, G-C once, then W1 on the exact route; the successor precommit receives is the
/// private driver's, byte for byte (P-1 and P-2 decided alike). RS validates it, so the one
/// publication is the successor. Stale: the plain bytes from one run.
#[test]
fn b3b_direct_entry_runs_the_exact_route() {
    for mode in MODES {
        let raw = m3x();
        let plain = plain(mode, &raw);
        let (_, _, private) = exact_w1(&raw, mode);
        let direct_raw = raw.clone();
        let (output, counts, captured) = hooks::counted_with_successor(move || run_linear_static_preview_value_with_retained_direct(direct_raw, mode).unwrap());
        if !registered() {
            assert!(output.retained().is_none() && captured.is_none(), "{mode:?}: no permit, no W1");
            assert_eq!(counts, ONE_RUN, "{mode:?}");
            assert_eq!(published(output), plain, "{mode:?}: the ordinary route");
            continue;
        }
        assert_eq!(output.admission().unwrap().law().refusal, None, "{mode:?}: admitted (branch E)");
        assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{mode:?}: one ordinary run, then G-C once");
        let captured = captured.expect("precommit received the successor");
        assert_eq!(captured, private.unwrap(), "{mode:?}: the Direct entry's successor is the private driver's, byte for byte");
        let envelope = output.envelope().clone();
        let retained = output.retained().unwrap().clone();
        assert!(exact_outcome(&format!("direct {mode:?}"), &retained, &captured, &envelope, &plain, &["case"]));
        assert_eq!(published(output), serde_json::to_vec(&captured).unwrap(), "{mode:?}: the one publication is the successor");
    }
}

/// N-11's acceptance: physics-1's Rust base readers accept the noticed ordinary envelope with the
/// same contract and standing as the plain one (`I105_N11_OUT` writes both for PY and TS).
fn n11_base_readers_accept(raw: &Value, mode: PreviewSolverMode, plain: &[u8], noticed: &[u8], label: &str) {
    use open_pipe_stress_result_export::semantic_contract as sc;
    let (base, noticed): (Value, Value) = (serde_json::from_slice(plain).unwrap(), serde_json::from_slice(noticed).unwrap());
    assert_eq!(base["producer"]["semantic_contract_id"], json!(PHYSICS_SEMANTIC_CONTRACT_ID), "{label}");
    assert!(sc::for_source(&base).is_ok(), "{label}: precondition, the base is admitted");
    assert_eq!(sc::for_source(&noticed), sc::for_source(&base), "{label}: admitted with the same contract");
    assert_eq!(sc::standing_reason(&noticed), sc::standing_reason(&base), "{label}");
    let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
    let bases: Vec<Value> = raw["model"]["load_cases"].as_array().unwrap().iter().map(|c| json!({"ref_type":"load_case","ref_id":c["id"]})).collect();
    let standing = sc::numerical_use_standing_with_context(&noticed, &bases, Some(&invocation));
    assert_eq!(standing, sc::numerical_use_standing_with_context(&base, &bases, Some(&invocation)), "{label}");
    println!("B3B_N11_RUST {label} for_source=ok standing={standing}");
    if let Ok(dir) = std::env::var("I105_N11_OUT") {
        let dir = std::path::Path::new(&dir);
        let tag = label.replace(' ', "_").replace(['(', ')', ':', '"'], "");
        std::fs::write(dir.join(format!("{tag}_noticed.json")), serde_json::to_vec(&noticed).unwrap()).unwrap();
        std::fs::write(dir.join(format!("{tag}_base.json")), plain).unwrap();
        std::fs::write(dir.join(format!("{tag}_invocation.json")), serde_json::to_vec(&invocation).unwrap()).unwrap();
    }
}

/// N-11 (B3-D REVISION_01 §9): an exact invocation whose W1 ran and was abandoned after W1 work
/// started (a serializer refusal with C1:68's detail, an evidence-overlay staging fault, a
/// precommit corruption) publishes the ordinary physics-1 envelope byte for byte, plus `case`'s N1
/// notice (the detail only on the serializer's), which physics-1's Rust base readers accept with
/// the same contract and standing. Registered: the actual Direct entry; Stale: the private driver.
#[test]
fn b3b_n11_abandoned_exact_w1_publishes_physics_1_with_the_notice() {
    use super::retained_wire::{ReceiptCheck as C, ReceiptFailure};
    let faults: Vec<(&str, fn(), W1Fallback, Option<&str>)> = vec![
        ("serializer", || hooks::fail_next_serializer(C::WorkCounterInconsistent),
            W1Fallback::Serializer(ReceiptFailure { check: C::WorkCounterInconsistent, field_path: "cases[].run.invocation_after" }), Some("work_counter_inconsistent")),
        ("section overlay", hooks::break_next_section_overlay, W1Fallback::Staging(rp::StagingFault("pipe_sections[]")), None),
        ("precommit", hooks::corrupt_next_precommit, W1Fallback::Precommit { gate: "G0", code: "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into() }, None),
    ];
    for mode in MODES {
        let raw = m3x();
        let plain = plain(mode, &raw);
        for (label, arm, cause, detail) in &faults {
            let label = format!("{label} {mode:?}");
            arm();
            let (envelope, retained) = if registered() {
                let (output, counts) = direct(&raw, mode);
                assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{label}");
                let retained = output.retained().unwrap().clone().map(|_| ());
                (output.envelope().clone(), retained)
            } else {
                let (envelope, retained, _) = exact_w1(&raw, mode);
                (envelope, retained.map(|_| ()))
            };
            assert!(hooks::armed_names().is_empty(), "{label}: fired");
            match (&retained, cause) {
                // Today's reader refuses at G0 before the corrupted receipt hash is read.
                (Err(W1Fallback::Precommit { .. }), W1Fallback::Precommit { .. }) => {}
                (Err(actual), expected) => assert_eq!(actual, expected, "{label}"),
                (Ok(()), _) => panic!("{label}: a fault was armed"),
            }
            let bytes = serde_json::to_vec(&envelope).unwrap();
            assert_eq!(String::from_utf8(bytes.clone()).unwrap(), String::from_utf8(with_notice(&plain, "case", *detail)).unwrap(),
                "{label}: the ordinary physics-1 bytes, then the notice");
            n11_base_readers_accept(&raw, mode, &plain, &bytes, &label);
        }
    }
}

/// P-2 and the coexistence pins (P-13; RR "I99's B3-W verified; …", rulings 1 and 2): n05 and n06
/// select exact blocks under physics-source-1, and so does `fields` under the exact route's
/// 8,000,000 budget (not under the default 4,000,000). Through the private driver W1 is never
/// attempted (`Coexistence`) and the ordinary owner is the ordinary route's bytes; through the
/// registered Direct entry the publication is exactly the ordinary route's bytes, from one run with
/// G-C not consulted (Stale: the same bytes, one run).
#[test]
fn b3b_coexistence_publishes_the_exact_ordinary_bytes_under_p2() {
    for (name, text) in [("n05", N05_EXACT), ("n06", N06_EXACT), ("fields", FIELDS_EXACT)] {
        let raw: Value = serde_json::from_str(text).unwrap();
        for mode in MODES {
            let label = format!("{name} {mode:?}");
            let plain = plain(mode, &raw);
            let value: Value = serde_json::from_slice(&plain).unwrap();
            assert!(value["source_block_recovery"].is_object(), "{label}: exact blocks selected (physics-source-1)");
            // The discriminator: the default budget does not select `fields` (its finalization
            // replay exceeds it), so a W1 budget other than the ordinary route's would lose T-3 (c).
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            let at_default = run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), None);
            assert_eq!(at_default.source_block_recovery.is_some(), name != "fields", "{label}: selection at 4,000,000");
            assert_ne!(serde_json::to_vec(&at_default).unwrap(), plain, "{label}: the default budget's bytes differ (work.limit at least)");
            let (capture, observer, ordinary) = observed(mode, &raw);
            assert_eq!(serde_json::to_vec(&ordinary).unwrap(), plain, "{label}: the private driver's run is the ordinary route's (P-2)");
            let (envelope, retained) = retained_w1(observer, ordinary, &capture);
            assert_eq!(retained.err(), Some(W1Fallback::Coexistence), "{label}");
            assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "{label}: exact bytes");
            let (output, counts) = direct(&raw, mode);
            if registered() {
                assert_eq!(output.admission().unwrap().law().refusal, None, "{label}: admitted");
                assert_eq!(output.retained().and_then(|r| r.as_ref().err()), Some(&W1Fallback::Coexistence), "{label}");
            }
            assert_eq!(counts, ONE_RUN, "{label}: G-C not consulted");
            assert_eq!(published(output), plain, "{label}: exactly the ordinary route's bytes");
        }
    }
}

/// B3-W's mixed exact base (P-13; RR ruling 1): `m3x_mix_anchor`'s `case` is `selected` beside its
/// zero-response `case:b`, `not_required`; `case:b`'s evidence entry and rows stay ordinary. The
/// successor precommit receives meets B3-D's requirements and is pinned; the Direct entry's is
/// the same (registered), with `case`'s notice only while RS refuses it at G0.
#[test]
fn b3b_mixed_exact_base_selects_case_beside_not_required() {
    for (mode, (name, receipt_sha, bytes_sha)) in MODES.into_iter().zip(MIX_PINNED) {
        let raw = m3x_mix_anchor();
        let plain = plain(mode, &raw);
        let plain_value: Value = serde_json::from_slice(&plain).unwrap();
        assert_eq!(plain_value["numerical_quality"]["cases"].as_array().unwrap().iter().map(|c| c["solve_quality"].as_str().unwrap()).collect::<Vec<_>>(),
            ["sensitive", "checks_passed"], "{name}: B3-W's verdicts");
        let (envelope, retained, captured) = exact_w1(&raw, mode);
        let successor = captured.unwrap_or_else(|| panic!("{name}: {retained:?}"));
        exact_outcome(name, &retained, &successor, &envelope, &plain, &["case"]);
        assert_exact_successor(name, &successor, &plain_value, &["selected", "not_required"]);
        let bytes = serde_json::to_vec(&successor).unwrap();
        println!("B3B_MIX_PIN {name} {} {}", successor["retained_precision"]["receipt_sha256"].as_str().unwrap(), sha(&bytes));
        assert_eq!((successor["retained_precision"]["receipt_sha256"].as_str(), sha(&bytes).as_str()), (Some(receipt_sha), bytes_sha), "{name}: the pinned mixed successor");
        let direct_raw = raw.clone();
        let (output, counts, direct_captured) = hooks::counted_with_successor(move || run_linear_static_preview_value_with_retained_direct(direct_raw, mode).unwrap());
        if registered() {
            assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{name}");
            assert_eq!(direct_captured.as_ref(), Some(&successor), "{name}: the Direct entry's successor");
            let retained = output.retained().unwrap().clone();
            exact_outcome(&format!("direct {name}"), &retained, &successor, output.envelope(), &plain, &["case"]);
        } else {
            assert_eq!((counts, published(output)), (ONE_RUN, plain), "{name}: Stale");
        }
    }
}

/// P-12's exact-route faults, both modes: an exact-capture fault (the Ĝ check refuses) makes the
/// attempt fail at preparation (custody: the capture's typed error), and an evidence-overlay fault
/// (a section patch past `pipe_sections`) makes staging refuse; each falls back with the ordinary
/// bytes and one notice. Registered: through the Direct entry; Stale: the private driver.
#[test]
fn b3b_exact_route_faults_fall_back_with_one_notice() {
    let faults: Vec<(&str, fn(), W1Fallback)> = vec![
        ("exact capture", hooks::fault_next_exact_capture, W1Fallback::Preparation),
        ("section overlay", hooks::break_next_section_overlay, W1Fallback::Staging(rp::StagingFault("pipe_sections[]"))),
    ];
    for mode in MODES {
        let raw = m3x();
        let plain = plain(mode, &raw);
        for (label, arm, cause) in &faults {
            arm();
            let (envelope, retained) = if registered() {
                let (output, counts) = direct(&raw, mode);
                assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{label} {mode:?}");
                (output.envelope().clone(), output.retained().unwrap().clone().map(|_| ()))
            } else {
                let (envelope, retained, _) = exact_w1(&raw, mode);
                (envelope, retained.map(|_| ()))
            };
            assert!(hooks::armed_names().is_empty(), "{label} {mode:?}: fired");
            assert_eq!(retained.err().as_ref(), Some(cause), "{label} {mode:?}");
            assert_eq!(String::from_utf8(serde_json::to_vec(&envelope).unwrap()).unwrap(), String::from_utf8(with_notice(&plain, "case", None)).unwrap(),
                "{label} {mode:?}: the ordinary bytes, then the notice");
        }
        // The exact-capture fault is the capture's typed error at preparation (custody).
        hooks::fault_next_exact_capture();
        let (_, observer, ordinary) = observed(mode, &raw);
        assert!(hooks::armed_names().is_empty());
        let failure = match observer.prepare_cases(ordinary, 1, &[0]) { Err(f) => f, Ok(_) => panic!("custody refuses") };
        assert_eq!(failure.error.to_string(), "exact material derived shear modulus", "{mode:?}");
    }
}

/// P-6's exact observables, each check on its own: one tamper of the ordinary exact evidence
/// between the run and the transaction, then custody, the native call and the freeze. Each is
/// refused where its check sits: at the observables (`facade_certificate`, observable), or for
/// the section identity first at the maxima stage (P-8's own check).
#[test]
fn b3b_exact_observables_refuse_each_evidence_defect() {
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = m3x();
    let freeze_with = |tamper: &dyn Fn(&mut Value)| -> String {
        let (_, observer, mut ordinary) = observed(mode, &raw);
        tamper(ordinary.contract_evidence.as_mut().unwrap());
        let mut prepared = match observer.prepare_cases(ordinary, 1, &[0]) { Ok(p) => p, Err(f) => return format!("custody: {}", f.error) };
        prepared.native();
        prepared.freeze();
        match &prepared.attempts[0].end {
            rp::AttemptEnd::Frozen(_) => "frozen".to_owned(),
            rp::AttemptEnd::Candidate(refused) => match &refused.error {
                rp::PreparedCandidateError::Observable => format!("observable: {}", prepared.capture.observable_error.as_ref().unwrap()),
                rp::PreparedCandidateError::Abandoned { cause, .. } => format!("abandoned: {cause}"),
                other => format!("{other:?}"),
            },
            _ => "other".to_owned(),
        }
    };
    assert_eq!(freeze_with(&|_| {}), "frozen", "control: the untampered evidence freezes");
    // The ordinary view (no overlay) reads the same route's evidence: its extrema numbers come
    // from `exact_cases`, so the untampered ordinary envelope passes the observables too.
    // Its section-record coverage is its own check there (in the freeze, P-8's prepared-section
    // domain refuses a missing section first: "a missing section" below).
    {
        let (_, observer, mut ordinary) = observed(mode, &raw);
        observer.observables(&ordinary).expect("control: the ordinary view of the exact evidence passes");
        ordinary.contract_evidence.as_mut().unwrap()["exact_cases"][0]["pipe_sections"] = json!([]);
        assert_eq!(observer.observables(&ordinary).unwrap_err().to_string(), "exact section/material coverage", "ordinary view, a missing section");
    }
    let cases: Vec<(&str, Box<dyn Fn(&mut Value)>, &str)> = vec![
        ("an extra evidence member", Box::new(|e: &mut Value| { e["preview_cases"] = json!([]); }), "observable: evidence shape"),
        ("pressure evidence", Box::new(|e: &mut Value| { e["pressure"] = json!([{}]); }), "observable: exact pressure/connector evidence"),
        ("connector evidence", Box::new(|e: &mut Value| { e["connector"] = json!([{}]); }), "observable: exact pressure/connector evidence"),
        ("a second exact case", Box::new(|e: &mut Value| { let c = e["exact_cases"][0].clone(); e["exact_cases"].as_array_mut().unwrap().push(c); }), "observable: evidence case"),
        ("recovery_method", Box::new(|e: &mut Value| { e["exact_cases"][0]["recovery_method"] = json!("x"); }), "observable: case shape"),
        ("load case id", Box::new(|e: &mut Value| { e["exact_cases"][0]["load_case_id"] = json!("other"); }), "observable: evidence case"),
        ("profile mode", Box::new(|e: &mut Value| { e["exact_cases"][0]["profile_mode"] = json!("legacy_pressure_v1"); }), "observable: exact profile/material basis"),
        ("material basis", Box::new(|e: &mut Value| { e["exact_cases"][0]["material_basis"] = json!("resolved_per_member_load_reference_state_v1"); }), "observable: exact profile/material basis"),
        ("incomplete coverage", Box::new(|e: &mut Value| { e["exact_cases"][0]["stress_maximum_coverage"]["complete"] = json!(false); }), "observable: maximum coverage"),
        ("an unavailable pipe", Box::new(|e: &mut Value| { e["exact_cases"][0]["stress_maximum_coverage"]["unavailable_pipe_ids"] = json!(["M1"]); }), "observable: maximum coverage"),
        ("a coverage member", Box::new(|e: &mut Value| { e["exact_cases"][0]["stress_maximum_coverage"]["outside_domain_pipe_ids"] = json!([]); }), "observable: stress coverage shape"),
        ("an assembly group", Box::new(|e: &mut Value| { e["exact_cases"][0]["pressure_rhs_assembly"]["groups"] = json!([{}]); }), "observable: exact pressure assembly groups"),
        ("a nonzero assembled entry", Box::new(|e: &mut Value| { e["exact_cases"][0]["pressure_rhs_assembly"]["assembled_pressure_rhs_global"][3] = json!(1.0); }), "observable: exact pressure assembly vector"),
        ("a nonzero cap entry", Box::new(|e: &mut Value| { e["exact_cases"][0]["pressure_rhs_assembly"]["rounded_cap_rhs_global"][0] = json!(-1e-300); }), "observable: exact pressure assembly vector"),
        ("a negative-zero Poisson entry", Box::new(|e: &mut Value| { e["exact_cases"][0]["pressure_rhs_assembly"]["rounded_poisson_rhs_global"][0] = json!(-0.0); }), "observable: exact pressure assembly vector"),
        ("a missing material record", Box::new(|e: &mut Value| { e["exact_cases"][0]["pipe_materials"] = json!([]); }), "observable: exact section/material coverage"),
        ("the section's OD", Box::new(|e: &mut Value| { e["exact_cases"][0]["pipe_sections"][0]["outside_diameter_m"] = json!(0.25); }), "observable: exact section geometry"),
        ("the section's wall", Box::new(|e: &mut Value| { e["exact_cases"][0]["pipe_sections"][0]["effective_wall_thickness_m"] = json!(0.0125); }), "observable: exact section geometry"),
        ("the material's G", Box::new(|e: &mut Value| { let g = e["exact_cases"][0]["pipe_materials"][0]["G_pa"].as_f64().unwrap(); e["exact_cases"][0]["pipe_materials"][0]["G_pa"] = json!(f64::from_bits(g.to_bits() + 1)); }), "observable: exact material values"),
        ("the material's nu", Box::new(|e: &mut Value| { e["exact_cases"][0]["pipe_materials"][0]["nu"] = json!(0.3); }), "observable: exact material values"),
        ("the material's E", Box::new(|e: &mut Value| { e["exact_cases"][0]["pipe_materials"][0]["E_pa"] = json!(2.1e11); }), "observable: exact material values"),
        ("the material's pipe", Box::new(|e: &mut Value| { e["exact_cases"][0]["pipe_materials"][0]["pipe_id"] = json!("M9"); }), "observable: exact material identity"),
        ("the section's pipe", Box::new(|e: &mut Value| { e["exact_cases"][0]["pipe_sections"][0]["pipe_id"] = json!("M9"); }), "abandoned: prepared section identity"),
        ("a missing section", Box::new(|e: &mut Value| { e["exact_cases"][0]["pipe_sections"] = json!([]); }), "abandoned: prepared section complete domain"),
        ("a section's numeric slot", Box::new(|e: &mut Value| { e["exact_cases"][0]["pipe_sections"][0]["J_m4"] = json!("x"); }), "abandoned: prepared section numeric slot"),
    ];
    for (label, tamper, expected) in &cases {
        assert_eq!(freeze_with(tamper.as_ref()), *expected, "{label}");
    }
}

/// B3a is dropped (RR "Owner decisions: the legacy pressure contract is retired product-wide;
/// …"): `m3l`, B3a's former witness, is now its refusal witness. G-A refuses it at D1.3 with
/// `PressureContract`, so it takes the ordinary route: the plain bytes, no W1, one run, in both
/// modes and every build.
#[test]
fn b3a_dropped_m3l_takes_the_ordinary_route() {
    use super::retained_memory::{AdmissionRefusal, D1Clause, FamilyFact};
    for mode in MODES {
        let raw = m3l();
        let plain = plain(mode, &raw);
        let (output, counts) = direct(&raw, mode);
        assert_eq!(output.admission().unwrap().law().domain, Some(AdmissionRefusal::Family(D1Clause::Namespace, FamilyFact::PressureContract)),
            "{mode:?}: D1.3 refuses the label");
        assert!(output.admission().unwrap().law().refusal.is_some(), "{mode:?}: G-A refuses");
        assert!(output.retained().is_none() && output.successor().is_none(), "{mode:?}: no W1");
        assert_eq!(counts, ONE_RUN, "{mode:?}");
        assert_eq!(published(output), plain, "{mode:?}: the ordinary route's bytes");
    }
}

/// P-13's refusals: an exact request outside D1's exact branch (regions absent or non-empty, a
/// combination, schema 0.4.0, a named point basis) is refused at G-A and takes the ordinary route:
/// the plain bytes, no W1, one run.
#[test]
fn b3b_refused_exact_requests_take_the_ordinary_route() {
    let variants: Vec<(&str, Box<dyn Fn(&mut Value)>)> = vec![
        ("regions absent", Box::new(|r: &mut Value| { r["model"]["load_cases"][0].as_object_mut().unwrap().remove("pressure_regions"); })),
        ("one region", Box::new(|r: &mut Value| r["model"]["load_cases"][0]["pressure_regions"] = json!([{"id": "region", "member_pipe_ids": ["M1"],
            "pressure_basis": "gauge", "pressure": {"value": 0.0, "unit": "Pa"}}]))),
        ("a combination", Box::new(|r: &mut Value| r["model"]["combinations"] = json!([{"id": "combination", "basis": "mechanics",
            "terms": [{"load_case": "case", "factor": 1.0}], "provenance": "invented_t3_p1_detection_input_no_library_data"}]))),
        ("schema 0.4.0", Box::new(|r: &mut Value| r["model"]["schema_version"] = json!("0.4.0"))),
        ("a named point basis", Box::new(|r: &mut Value| r["model"]["load_cases"][0]["modulus_basis_ref"] = json!("T0"))),
    ];
    for (label, change) in &variants {
        for mode in MODES {
            let mut raw = m3x();
            change(&mut raw);
            let plain = plain(mode, &raw);
            let (output, counts) = direct(&raw, mode);
            assert!(output.admission().unwrap().law().refusal.is_some(), "{label} {mode:?}: G-A refuses");
            assert!(output.retained().is_none(), "{label} {mode:?}: no W1");
            assert_eq!(counts, ONE_RUN, "{label} {mode:?}");
            assert_eq!(published(output), plain, "{label} {mode:?}: the ordinary route's bytes");
        }
    }
}

/// B3b-P (B3-D P-5; D1.5-exact, DEF-E `scope.materials`): the exact route's capture admits the
/// base common E/ν selection only. A named point basis on the exact route (which the ordinary
/// route publishes, and G-A refuses with `ModulusBasisRef`) reaches the private driver's capture,
/// which refuses it typed, so the attempt fails at preparation (custody) and W1 falls back.
#[test]
fn b3b_exact_capture_refuses_a_selected_basis() {
    for mode in MODES {
        let mut raw = m3x();
        raw["model"]["materials"][0]["temperature_points"] = json!([{"id": "T0", "temperature": {"value": 20, "unit": "degC"},
            "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "poisson_ratio": {"value": 0.25, "unit": "1"}}]);
        raw["model"]["load_cases"][0]["modulus_basis_ref"] = json!("T0");
        let ordinary_value: Value = serde_json::from_slice(&plain(mode, &raw)).unwrap();
        assert_eq!(ordinary_value["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}: the ordinary route solves it");
        assert!(ordinary_value["results"].as_array().unwrap().iter().any(|r| r["kind"] == "modulus_basis_record"), "{mode:?}");
        let (_, observer, ordinary) = observed(mode, &raw);
        let failure = match observer.prepare_cases(ordinary, 1, &[0]) { Err(f) => f, Ok(_) => panic!("{mode:?}: custody refuses") };
        assert_eq!(failure.error.to_string(), "exact route: base common E/nu selection only", "{mode:?}");
    }
}

/// B3-D P-4 (I95's ruling 2): W1 never calls the pressure runtime's builders. The capture reads
/// D1.5-exact from the model and the ordinary run's own outputs; no retained file, and no W1
/// function of lib.rs's retained section, names `build_pressure_case`, `finish_source_groups` or
/// `traverse_region` (the ordinary route calls them once per case, unchanged).
#[test]
fn b3b_p4_w1_calls_no_pressure_runtime_builder() {
    let lib = include_str!("lib.rs");
    let w1 = &lib[lib.find("fn permitted_dispatch(").unwrap()..lib.find("pub(crate) mod retained_tests_hooks {").unwrap()];
    for (name, text) in [("retained_product.rs", include_str!("retained_product.rs")), ("retained_wire.rs", include_str!("retained_wire.rs")),
        ("retained_receipt.rs", include_str!("retained_receipt.rs")), ("lib.rs W1", w1)] {
        for builder in ["build_pressure_case", "finish_source_groups", "traverse_region", "pressure_runtime::build", "pressure_material::resolve"] {
            assert!(!text.contains(builder), "{name}: {builder}");
        }
    }
    assert!(w1.contains("fn w1_route(") && w1.contains("fn w1_budget("), "the route and budget are decided in the W1 section");
}

/// The exact route at c = 2 with both cases selected: `m3x` with a copy of its case (`case-2`, its
/// load ids suffixed). Each selected case's attempt regenerates its own `exact_cases` entry only
/// (DEF-E `evidence`, S-2's "owner case"), at evidence index 0 and 1, with case-qualified row ids
/// after the first; the successor meets B3-D's requirements (no pin: not a selected witness).
#[test]
fn b3b_two_selected_exact_cases_each_regenerate_their_own_entry() {
    let raw = two_case_exact();
    for mode in MODES {
        let label = format!("two cases {mode:?}");
        let plain = plain(mode, &raw);
        let plain_value: Value = serde_json::from_slice(&plain).unwrap();
        let (envelope, retained, captured) = exact_w1(&raw, mode);
        let successor = captured.unwrap_or_else(|| panic!("{label}: {retained:?}"));
        exact_outcome(&label, &retained, &successor, &envelope, &plain, &["case", "case-2"]);
        assert_exact_successor(&label, &successor, &plain_value, &["selected", "selected"]);
        let entries = successor["contract_evidence"]["exact_cases"].as_array().unwrap();
        assert_eq!(entries.iter().map(|e| e["load_case_id"].as_str().unwrap()).collect::<Vec<_>>(), ["case", "case-2"], "{label}");
        assert_eq!(entries[0]["pipe_sections"], entries[1]["pipe_sections"], "{label}: the same prepared section in both entries");
        assert!(successor["results"].as_array().unwrap().iter().any(|r| r["id"].as_str().unwrap().starts_with("result:loadcase:case-2:")), "{label}: qualified ids");
    }
}

/// m3x with a second load case `case-2` (the same moments, ids suffixed `:2`).
fn two_case_exact() -> Value {
    let mut raw = raw();
    let mut second = raw["model"]["load_cases"][0].clone();
    second["id"] = json!("case-2");
    for load in second["primitive_loads"].as_array_mut().unwrap() {
        load["id"] = json!(format!("{}:2", load["id"].as_str().unwrap()));
    }
    raw["model"]["load_cases"].as_array_mut().unwrap().push(second);
    exact3(raw)
}

// ---- RV123 (RV-P2 round 1 on B3b-P): S-1, S-2 and N-2 ------------------------------------------

/// RV123 S-1's input: m3x plus a collinear second member M2 (N1 to a new N2, OD 0.15 m, wall
/// 0.008 m, the same material), its members authored in either order (`reversed`).
fn two_member_exact(reversed: bool) -> Value {
    let mut raw = m3x();
    let p = "invented_t3_p1_detection_input_no_library_data";
    raw["model"]["nodes"].as_array_mut().unwrap().push(json!({"id": "N2", "position": {"x": 2.0, "y": 4.0, "z": 4.0}, "provenance": p}));
    raw["model"]["pipe_segments"].as_array_mut().unwrap().push(json!({"id": "M2", "from": "N1", "to": "N2", "material": "mat:N",
        "y_reference": {"x": 1, "y": 0, "z": 0}, "section": {"outside_diameter": {"value": 0.15, "unit": "m"}, "wall_thickness": {"value": 0.008, "unit": "m"}}, "provenance": p}));
    if reversed {
        raw["model"]["pipe_segments"].as_array_mut().unwrap().reverse();
    }
    raw
}
/// RV123 S-1's pins: (members reversed, mode, sha256 of the successor's bytes).
const TWO_MEMBER_PINNED: [(bool, &str, &str); 4] = [
    (false, "sparse_interactive", "94549ef80998798af7a7068007f4c2cd27bf1edac1b3e0efb80ef4022dd90f8b"),
    (false, "dense_scrutiny", "58d053642f6b3bcc4d475dde530c28651a01130cc8c50105ffe9ba2a0e853eb2"),
    (true, "sparse_interactive", "cb20f6dd4c70c5b96b47ad996ef0dd469be895c183ab0dc8d43924f6a7338d9a"),
    (true, "dense_scrutiny", "3fd88b489fb030d82a4b678ab6ff3d1ba96097088fff71dd1843aff6ef8e0368")];
/// RV123 S-1: a two-member exact successor, in both authored member orders and both modes: each
/// member's `pipe_sections` entry takes its own prepared A, I, J and Z (`assert_exact_successor`),
/// and M2's prepared section differs from its source annulus's in at least one value, so the
/// per-member overlay is pinned beyond member 0. The successors are pinned.
#[test]
fn b3b_rv123_s1_two_member_exact_overlays_each_member() {
    for reversed in [false, true] {
        let raw = two_member_exact(reversed);
        for mode in MODES {
            let label = format!("two members reversed={reversed} {mode:?}");
            let plain = plain(mode, &raw);
            let plain_value: Value = serde_json::from_slice(&plain).unwrap();
            let (envelope, retained, captured) = exact_w1(&raw, mode);
            let successor = captured.unwrap_or_else(|| panic!("{label}: {retained:?}"));
            exact_outcome(&label, &retained, &successor, &envelope, &plain, &["case"]);
            assert_exact_successor(&label, &successor, &plain_value, &["selected"]);
            let section = |e: &Value| e["exact_cases"][0]["pipe_sections"].as_array().unwrap().iter().find(|s| s["pipe_id"] == "M2").unwrap().clone();
            let (now, was) = (section(&successor["contract_evidence"]), section(&plain_value["contract_evidence"]));
            assert!(["As_m2", "I_m4", "J_m4", "Z_m3"].iter().any(|k| num_bits(&now[*k]) != num_bits(&was[*k])), "{label}: M2's prepared section is not its source annulus's");
            let bytes = sha(&serde_json::to_vec(&successor).unwrap());
            println!("RV123_S1 {label} successor_sha256={bytes}");
            let pinned = TWO_MEMBER_PINNED.iter().find(|(r, m, _)| *r == reversed && *m == mode.as_str()).unwrap().2;
            assert_eq!(bytes, pinned, "{label}: pinned");
        }
    }
}
/// RV123 S-2's pins: (mode, sha256 of the successor's bytes).
const UNAVAILABLE_EXACT_PINNED: [(&str, &str); 2] = [("sparse_interactive", "a056ac91ce498ec79505e68bf63559d1cc7abba69428a84ece70c8bb293d070a"),
    ("dense_scrutiny", "5275a75381cd8d0797b598f6057d206b7f57510bc32132687ac2470977d800b5")];
/// RV123 S-2: an exact successor with an `unavailable` prepared case, at the n-case serializer's
/// unavailable branch: the two-case exact input with case-2's freeze refused after its selected
/// Run (`fail_freeze_of_case`). Case-2 is `unavailable` (`facade_certificate`, phase `facade`),
/// its attempt is DEF-E's and its CaseSource's preparation binding is made with DEF-E's H (S-1's
/// route H), not DEF-O's. Today's readers refuse at G0, so W1 falls back with both notices.
#[test]
fn b3b_rv123_s2_unavailable_exact_case_binds_def_e() {
    use super::retained_wire as wire;
    let raw = two_case_exact();
    for mode in MODES {
        let label = format!("unavailable case-2 {mode:?}");
        let plain = plain(mode, &raw);
        let plain_value: Value = serde_json::from_slice(&plain).unwrap();
        hooks::fail_freeze_of_case(1);
        let (envelope, retained, captured) = exact_w1(&raw, mode);
        assert!(hooks::armed_names().is_empty(), "{label}: the fault fired");
        let successor = captured.unwrap_or_else(|| panic!("{label}: {retained:?}"));
        exact_outcome(&label, &retained, &successor, &envelope, &plain, &["case", "case-2"]);
        assert_exact_successor(&label, &successor, &plain_value, &["selected", "unavailable"]);
        let body = &successor["retained_precision"]["body"];
        let case = &body["cases"][1];
        assert_eq!((&case["reason"]["code"], &case["reason"]["phase"]), (&json!("facade_certificate"), &json!("facade")), "{label}");
        let attempt = &body["product_attempts"][case["product_attempt_ref"].as_u64().unwrap() as usize];
        let source = &body["sources"][case["source_ref"].as_u64().unwrap() as usize];
        assert_eq!(attempt["definition_id"], json!(wire::EXACT_DEFINITION_ID), "{label}");
        assert_eq!(source["preparation"]["attempt_ref"], case["product_attempt_ref"], "{label}");
        assert_eq!(source["preparation"]["sha256"], json!(preparation_hash(attempt, wire::EXACT_DEFINITION_SHA256)), "{label}: DEF-E's H");
        let bytes = sha(&serde_json::to_vec(&successor).unwrap());
        println!("RV123_S2 {label} successor_sha256={bytes}");
        let pinned = UNAVAILABLE_EXACT_PINNED.iter().find(|(m, _)| *m == mode.as_str()).unwrap().1;
        assert_eq!(bytes, pinned, "{label}: pinned");
    }
}
/// RV123 S-2 (its R-02 and R-03): the one-case serializer's unavailable branch on the exact route.
/// m3x's candidate refused after its selected Run (an injected maxima fault) serializes with
/// physics-retained-1's identity and profile, and its CaseSource's preparation binding with DEF-E's
/// H.
#[test]
fn b3b_rv123_s2_one_case_unavailable_exact_serializer() {
    use super::retained_receipt::TraceFault as F;
    use super::retained_wire as wire;
    for mode in MODES {
        let label = format!("one-case unavailable {mode:?}");
        let (request, capture) = source_receipt::CapturedInvocation::parse(m3x(), mode).unwrap();
        let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|e| panic!("{label}: {:?}", e.capture.error));
        prepared.test_capture_mut().trace_fault = Some(F::Maxima);
        prepared.solve_native().unwrap();
        let refused = match prepared.project_candidate() { Err(refused) => refused, Ok(_) => panic!("{label}: a refusal") };
        let successor = wire::serialize_unavailable(wire::Refused::Candidate(&refused), &capture).unwrap_or_else(|f| panic!("{label}: {f:?}"));
        assert_eq!(successor["producer"]["semantic_contract_id"], json!(wire::EXACT_SEMANTIC_ID), "{label}: R-03");
        assert_eq!(successor["formulation_basis"]["profile_id"], json!(wire::EXACT_PROFILE_ID), "{label}");
        let body = &successor["retained_precision"]["body"];
        assert_eq!(body["cases"][0]["status"], json!("unavailable"), "{label}");
        assert_eq!(body["cases"][0]["reason"]["code"], json!("facade_certificate"), "{label}");
        let attempt = &body["product_attempts"][0];
        assert_eq!(attempt["definition_id"], json!(wire::EXACT_DEFINITION_ID), "{label}");
        assert_eq!(body["sources"][0]["preparation"]["sha256"], json!(preparation_hash(attempt, wire::EXACT_DEFINITION_SHA256)), "{label}: R-02");
    }
}
/// RV123 N-2: an unused material on the exact route, with no ν and no basis, is not checked: m3x
/// plus such a material still selects, and only the used material is in `material_bases`.
#[test]
fn b3b_rv123_n2_unused_exact_material_is_not_checked() {
    let mut raw = m3x();
    raw["model"]["materials"].as_array_mut().unwrap().push(json!({"id": "mat:unused", "elastic_modulus": {"value": 1.0e11, "unit": "Pa"},
        "provenance": "invented_t3_p1_detection_input_no_library_data"}));
    for mode in MODES {
        let label = format!("unused material {mode:?}");
        let plain = plain(mode, &raw);
        let plain_value: Value = serde_json::from_slice(&plain).unwrap();
        let (envelope, retained, captured) = exact_w1(&raw, mode);
        let successor = captured.unwrap_or_else(|| panic!("{label}: {retained:?}"));
        exact_outcome(&label, &retained, &successor, &envelope, &plain, &["case"]);
        assert_exact_successor(&label, &successor, &plain_value, &["selected"]);
        let ids: Vec<&str> = successor["retained_precision"]["body"]["material_bases"][0]["materials"].as_array().unwrap().iter()
            .map(|m| m["id"].as_str().unwrap()).collect();
        assert_eq!(ids, ["mat:N"], "{label}: only the used material");
    }
}

// ---- B2-P (I105): combinations (B2-C; PLAN §1.2.4, §1.2.6) ------------------------------------

/// I98's witness provenance (R/I98/b2_w_probe_01 `gen_inputs.py`), and lane P's for the
/// witnesses B2-C REVISION_01 §5.1 adds beyond I98's.
const B2W: &str = "invented_t3_b2_w_probe_input_no_library_data";
const B2P: &str = "invented_t3_b2_p_witness_input_no_library_data";
/// I98's `case`: `template` with its id and loads replaced.
fn b2w_case(id: &str, loads: Value, template: &Value) -> Value {
    let mut case = template.clone();
    case["id"] = json!(id);
    case["primitive_loads"] = loads;
    case
}
/// I98's `combination(a, b)`: mechanics `a` + `factor`·`b`.
fn b2w_combination(a: &str, b: &str, factor: f64, label: &str) -> Value {
    json!({"id": "combination:ab", "label": label, "basis": "mechanics",
        "terms": [{"load_case": a, "factor": 1.0}, {"load_case": b, "factor": factor}], "provenance": B2W})
}
/// W-CB3 (I98 `r7_cb3_v1`; REVISION_01 §5.1): the L = 0 base, case A its milestone moments,
/// case B a 1 N `global_y` force on the restrained isolated node N2, and A + B.
fn w_cb3() -> Value {
    let mut raw = u8_l0_isolated_node();
    let c0 = raw["model"]["load_cases"][0].clone();
    let b = json!([{"id": "load:n2-y", "category": "concentrated_force", "target": {"type": "node", "node": "N2"}, "direction": "global_y",
        "dimension": "force", "magnitude": {"value": 1.0, "unit": "N"}, "provenance": B2W}]);
    raw["model"]["load_cases"] = json!([b2w_case("case:a", c0["primitive_loads"].clone(), &c0), b2w_case("case:b", b, &c0)]);
    raw["model"]["combinations"] = json!([b2w_combination("case:a", "case:b", 1.0, "I98 B2-W R-7 count: A + B")]);
    raw
}
/// W-CB2 (I98 `r7_cb2`): U8's two-body cases A and B, and A + B.
fn w_cb2() -> Value {
    let mut raw = u8_two_body_case_a();
    let c0 = raw["model"]["load_cases"][0].clone();
    let b = u8_two_body_case_b()["model"]["load_cases"][0]["primitive_loads"].clone();
    raw["model"]["load_cases"] = json!([b2w_case("case:a", c0["primitive_loads"].clone(), &c0), b2w_case("case:b", b, &c0)]);
    raw["model"]["combinations"] = json!([b2w_combination("case:a", "case:b", 1.0, "I98 B2-W R-7 count: A + B")]);
    raw
}
/// W-CB3's two cases with one other combination (W-CB4a, W-CB4b, W-CB5; REVISION_01 §5.1).
fn w_cb3_with(combination: Value) -> Value {
    let mut raw = w_cb3();
    raw["model"]["combinations"] = json!([combination]);
    raw
}
/// W-CB4a: A − B. W-CB4b: range(A, B), `max_abs`. W-CB5: 2·B (ordinary-only mechanics).
fn w_cb4a() -> Value {
    w_cb3_with(json!({"id": "combination:a-minus-b", "label": "B2-P W-CB4a: A - B", "basis": "result_state_subtraction",
        "minuend_id": "case:a", "subtrahend_id": "case:b", "provenance": B2P}))
}
fn w_cb4b() -> Value {
    w_cb3_with(json!({"id": "combination:range-ab", "label": "B2-P W-CB4b: range(A, B)", "basis": "range_envelope",
        "operand_ids": ["case:a", "case:b"], "mode": "max_abs", "provenance": B2P}))
}
fn w_cb5() -> Value {
    w_cb3_with(json!({"id": "combination:2b", "label": "B2-P W-CB5: 2 B", "basis": "mechanics",
        "terms": [{"load_case": "case:b", "factor": 2.0}], "provenance": B2P}))
}
/// `b2_c1_range_mechanics` (REVISION_01 §5.1, REVISION_02 A-3): the milestone (c = 1) with
/// `[range(case), 2·case]`, in that order (z = 2, C_eq = 3): S-2's layout.
fn b2_c1_range_mechanics() -> Value {
    let mut raw = raw();
    raw["model"]["combinations"] = json!([
        {"id": "combination:range", "label": "B2-P c = 1: range(case)", "basis": "range_envelope", "operand_ids": ["case"], "mode": "max_abs", "provenance": B2P},
        {"id": "combination:2case", "label": "B2-P c = 1: 2 case", "basis": "mechanics", "terms": [{"load_case": "case", "factor": 2.0}], "provenance": B2P}]);
    raw
}

/// B2-P (B2-C §2.3–§2.6; REVISION_01 §5.1): each producer-solved witness through the private
/// driver, both modes: its combinations' dispositions and its cases' statuses in the successor
/// precommit receives. W-CB3: `retained_selected` with one operand preparation; W-CB2:
/// `retained_unavailable` (`combination_unresolved`, phase `kernel`); W-CB4a, W-CB4b and W-CB5:
/// `ordinary`; `b2_c1_range_mechanics`: range `ordinary`, 2·case `retained_selected`. With
/// `I105_B2P_OUT` set, each successor document is written there.
#[test]
fn b2p_witness_dispositions_on_the_private_driver() {
    let out = std::env::var("I105_B2P_OUT").ok();
    let unresolved = json!({"code":"combination_unresolved","phase":"kernel","cause":{"kind":"prepared_product_failure","product_attempt_ref":1}});
    let witnesses: [(&str, Value, &[(&str, Value)], &[&str]); 6] = [
        ("w_cb3", w_cb3(), &[("retained_selected", Value::Null)], &["selected", "not_required"]),
        ("w_cb2", w_cb2(), &[("retained_unavailable", unresolved)], &["selected", "not_required"]),
        ("w_cb4a", w_cb4a(), &[("ordinary", json!("no_retained_mechanics"))], &["selected", "not_required"]),
        ("w_cb4b", w_cb4b(), &[("ordinary", json!("no_retained_mechanics"))], &["selected", "not_required"]),
        ("w_cb5", w_cb5(), &[("ordinary", json!("no_retained_mechanics"))], &["selected", "not_required"]),
        ("c1_range_mechanics", b2_c1_range_mechanics(), &[("ordinary", json!("no_retained_mechanics")), ("retained_selected", Value::Null)], &["selected"]),
    ];
    for (name, raw, dispositions, statuses) in witnesses {
        println!("B2P_INPUT {name} value_sha={}", sha(&serde_json::to_vec(&raw).unwrap()));
        for mode in MODES {
            let label = format!("{name} {mode:?}");
            let (capture, observer, ordinary) = observed(mode, &raw);
            let ((_, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
            println!("B2P_OUTCOME {label} retained={:?}", retained.as_ref().err());
            let successor = captured.unwrap_or_else(|| panic!("{label}: {retained:?}"));
            let body = &successor["retained_precision"]["body"];
            let got: Vec<(&str, Value)> = body["combinations"].as_array().unwrap().iter()
                .map(|c| (c["disposition"].as_str().unwrap(), c["reason"].clone())).collect();
            assert_eq!(got, dispositions.iter().map(|(d, r)| (*d, r.clone())).collect::<Vec<_>>(), "{label}");
            assert_eq!(body["cases"].as_array().unwrap().iter().map(|c| c["status"].as_str().unwrap()).collect::<Vec<_>>(), statuses, "{label}");
            if let Some(dir) = &out {
                std::fs::write(format!("{dir}/{name}_{}.json", mode.as_str()), serde_json::to_vec_pretty(&json!({"source": successor, "invocation": {"request": raw, "solver_mode": mode.as_str()}})).unwrap()).unwrap();
            }
        }
    }
}

/// Two copies of the milestone's case (`case:a`, `case:b`, both Sensitive and selected) and A + B:
/// the hooks' two-selected-case base.
fn b2p_two_cases() -> Value {
    let mut raw = raw();
    let c0 = raw["model"]["load_cases"][0].clone();
    let suffixed = |suffix: &str| json!(c0["primitive_loads"].as_array().unwrap().iter().map(|load| {
        let mut load = load.clone();
        load["id"] = json!(format!("{}{suffix}", load["id"].as_str().unwrap()));
        load
    }).collect::<Vec<_>>());
    raw["model"]["load_cases"] = json!([b2w_case("case:a", suffixed(""), &c0), b2w_case("case:b", suffixed(":b"), &c0)]);
    raw["model"]["combinations"] = json!([b2w_combination("case:a", "case:b", 1.0, "B2-P hooks: A + B")]);
    raw
}
/// The successor precommit receives from the private driver (both modes are not needed here).
/// With `I105_B2P_OUT` set, it is written there (named by its sha256) for the SCHEMA check.
fn b2p_successor(raw: &Value, mode: PreviewSolverMode) -> (Result<RetainedSuccessor, W1Fallback>, Value) {
    let (capture, observer, ordinary) = observed(mode, raw);
    let ((_, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
    let successor = captured.unwrap_or_else(|| panic!("{retained:?}"));
    if let Ok(dir) = std::env::var("I105_B2P_OUT") {
        let bytes = serde_json::to_vec(&json!({"source": successor})).unwrap();
        std::fs::write(format!("{dir}/b2p_successor_{}.json", &sha(&bytes)[..16]), bytes).unwrap();
    }
    (retained, successor)
}

/// B2-P's hooks and failure set (B2-C §2.6; C3a-6; REVISION_01 C-4), each one combination's own
/// outcome, never an abandonment, in both modes:
/// - an operand preparation refused (`fail_operand_preparation`): W-CB3's combination is
///   `retained_unavailable`, `operand_preparation_failure`, with no Call; the record is refused,
///   `failed`, with no source;
/// - a Call refused before any source (`fail_combination_call`): `pre_source_refusal`
///   (`no_operands`), with its Call but no Run, source or attempt;
/// - a combination freeze fault (`fault_next_combination_freeze`): `facade_certificate` after its
///   selected Run, its attempt `unavailable` with an observable error;
/// - an operand with no CaseSource (B's T-7 preparation refused, `fail_preparation_of_case`):
///   `operand_source_unavailable` at operand 1, with no preparation and no Call;
/// - an `unavailable` operand with a CaseSource (B's freeze refused, `fail_freeze_of_case`): its
///   source rebuilt and passed as a prepared operand at its batch source id, importing nothing.
#[test]
fn b2p_hooks_and_failure_set() {
    for mode in MODES {
        let label = |what: &str| format!("{what} {mode:?}");
        // An operand preparation refused.
        hooks::fail_operand_preparation(1);
        let (_, s) = b2p_successor(&w_cb3(), mode);
        assert!(hooks::armed_names().is_empty());
        let b = &s["retained_precision"]["body"];
        let c = &b["combinations"][0];
        assert_eq!((&c["disposition"], &c["reason"]["code"], &c["reason"]["phase"], &c["reason"]["cause"]),
            (&json!("retained_unavailable"), &json!("combination_unresolved"), &json!("preparation"),
                &json!({"kind":"operand_preparation_failure","operand_preparation_ref":0})), "{}", label("operand preparation"));
        assert_eq!((&c["call_ref"], &c["run"], &c["source_ref"], &c["product_attempt_ref"]), (&Value::Null, &Value::Null, &Value::Null, &Value::Null));
        let record = &b["operand_preparations"][0];
        assert_eq!((&record["result"]["kind"], &record["stage"], &record["source_ref"], &record["requested_by"]),
            (&json!("refused"), &json!("failed"), &Value::Null, &json!([0])), "{}", label("operand preparation record"));
        assert_eq!((b["calls"].as_array().unwrap().len(), b["sources"].as_array().unwrap().len()), (1, 1), "{}", label("no Call, no source"));
        // A Call refused before any source.
        hooks::fail_combination_call(0);
        let (_, s) = b2p_successor(&w_cb3(), mode);
        assert!(hooks::armed_names().is_empty());
        let b = &s["retained_precision"]["body"];
        let c = &b["combinations"][0];
        assert_eq!((&c["disposition"], &c["reason"]["phase"], &c["reason"]["cause"], &c["call_ref"]),
            (&json!("retained_unavailable"), &json!("preparation"), &json!({"space":"combination","tag":"no_operands"}), &json!(1)), "{}", label("pre-source"));
        assert_eq!((&c["run"], &c["source_ref"], &c["product_attempt_ref"]), (&Value::Null, &Value::Null, &Value::Null));
        let call = &b["calls"][1];
        assert_eq!((&call["result"]["kind"], &call["result"]["stage"], &call["run_refs"], &call["source_refs"]),
            (&json!("pre_source_refusal"), &json!("operand_validation"), &json!([]), &json!([])), "{}", label("pre-source call"));
        assert_eq!(call["invocation_after"], call["invocation_before"], "{}", label("no work"));
        assert_eq!(b["work"]["execution_order"].as_array().unwrap().len(), 1, "{}", label("no combination Run"));
        // A combination freeze fault.
        hooks::fault_next_combination_freeze();
        let (_, s) = b2p_successor(&w_cb3(), mode);
        assert!(hooks::armed_names().is_empty());
        let b = &s["retained_precision"]["body"];
        let c = &b["combinations"][0];
        assert_eq!((&c["disposition"], &c["reason"]["code"], &c["reason"]["phase"], &c["reason"]["cause"]),
            (&json!("retained_unavailable"), &json!("facade_certificate"), &json!("facade"), &json!({"kind":"prepared_product_failure","product_attempt_ref":1})),
            "{}", label("freeze fault"));
        let attempt = &b["product_attempts"][1];
        assert_eq!((&attempt["result"]["error"]["kind"], &attempt["stages"]["observables"], &attempt["stages"]["g5a"]),
            (&json!("observable"), &json!("failed"), &json!("completed")), "{}", label("freeze fault attempt: G5a still runs, as for a case"));
        assert!(s["results"].as_array().unwrap().iter().filter(|r| r["basis_ref"]["ref_type"] == "combination").all(|r| r.get("recovery_method").is_none()));
        // An operand with no CaseSource.
        hooks::fail_preparation_of_case(1);
        let (_, s) = b2p_successor(&b2p_two_cases(), mode);
        assert!(hooks::armed_names().is_empty());
        let b = &s["retained_precision"]["body"];
        let c = &b["combinations"][0];
        assert_eq!(b["cases"][1]["status"], json!("unavailable"));
        assert_eq!((&c["disposition"], &c["reason"]["phase"], &c["reason"]["cause"], &c["call_ref"]),
            (&json!("retained_unavailable"), &json!("preparation"), &json!({"kind":"operand_source_unavailable","operand_index":1}), &Value::Null),
            "{}", label("operand source unavailable"));
        assert!(b.get("operand_preparations").is_none(), "{}", label("C-6: absent when empty"));
        // An unavailable operand with a CaseSource: rebuilt, no import.
        hooks::fail_freeze_of_case(1);
        let (_, s) = b2p_successor(&b2p_two_cases(), mode);
        assert!(hooks::armed_names().is_empty());
        let b = &s["retained_precision"]["body"];
        assert_eq!((&b["cases"][1]["status"], &b["cases"][1]["source_ref"]), (&json!("unavailable"), &json!(1)));
        let c = &b["combinations"][0];
        let call = &b["calls"][1];
        assert_eq!(call["requested_operands"], json!([{"source_ref":0,"factor":"3ff0000000000000"},{"source_ref":1,"factor":"3ff0000000000000"}]),
            "{}", label("the batch source ids"));
        let group = b["groups"].as_array().unwrap().iter().find(|g| g["call"] == json!(1)).unwrap();
        assert!(group["imports"].as_array().unwrap().iter().all(|i| i["operand_index"] == json!(0)), "{}", label("imports from the selected operand only"));
        let source = &b["sources"][c["source_ref"].as_u64().unwrap() as usize];
        assert_eq!(source["operands"][1]["source_ref"], json!(1));
        assert_eq!(c["disposition"], json!("retained_selected"), "{}", label("the rebuilt operand combines"));
    }
}

/// B2-P (T-9′; REVISION_01 S-1): a case freeze checks the gate entries' shape and consistency
/// only. W-CB3's ordinary envelope with one tampered entry, an extra entry or none refuses every
/// case freeze (`Candidate`, the cause in the case's observables); a withheld entry with a gate
/// code is not a case-freeze failure (T-10a then gives that combination `base_withheld`).
#[test]
fn b2p_gate_entries_shape_and_consistency() {
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = w_cb3();
    let run = |tamper: &dyn Fn(&mut Value)| {
        let (capture, observer, mut ordinary) = observed(mode, &raw);
        tamper(&mut ordinary.contract_evidence.as_mut().unwrap()["combination_gates"]);
        let ((_, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
        (retained, captured)
    };
    for (label, tamper) in [
        ("an extra key", Box::new(|g: &mut Value| { g[0]["extra"] = json!(1); }) as Box<dyn Fn(&mut Value)>),
        ("another id", Box::new(|g: &mut Value| { g[0]["combination_id"] = json!("combination:other"); })),
        ("withheld with no code", Box::new(|g: &mut Value| { g[0]["withheld"] = json!(true); })),
        ("not withheld with a code", Box::new(|g: &mut Value| { g[0]["reason"] = json!("NONLINEAR_COMBINATION_REQUIRES_SOLVE"); })),
        ("withheld with another code", Box::new(|g: &mut Value| { g[0]["withheld"] = json!(true); g[0]["reason"] = json!("OTHER"); })),
        ("an extra entry", Box::new(|g: &mut Value| { let mut extra = g[0].clone(); extra["combination_id"] = json!("combination:extra"); g.as_array_mut().unwrap().push(extra); })),
        ("no entry", Box::new(|g: &mut Value| { g.as_array_mut().unwrap().clear(); })),
    ] {
        let (retained, captured) = run(tamper.as_ref());
        assert_eq!(retained.err(), Some(W1Fallback::Candidate), "{label}: every case freeze refuses");
        assert!(captured.is_none(), "{label}");
    }
    let (_, captured) = run(&|g: &mut Value| { g[0]["withheld"] = json!(true); g[0]["reason"] = json!("CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE"); });
    let successor = captured.expect("a withheld entry is not a case-freeze failure");
    let c = &successor["retained_precision"]["body"]["combinations"][0];
    assert_eq!((&c["disposition"], &c["reason"]), (&json!("base_withheld"), &json!("CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE")), "T-10a rule 1");
}

/// B2-P (T-6′; REVISION_01 S-2): custody binds the rows after the case blocks to their
/// combinations, a mechanics combination's as one contiguous run. `b2_c1_range_mechanics`'s
/// envelope (case rows, range rows, 2·case rows, then the range's record) with one 2·case row moved
/// into the range rows, a combination row naming no combination, or a case row after the
/// combination rows: custody refuses (`Preparation`, then the notice).
#[test]
fn b2p_custody_binds_the_combination_rows() {
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = b2_c1_range_mechanics();
    let run = |tamper: &dyn Fn(&mut Vec<ResultItem>)| {
        let (capture, observer, mut ordinary) = observed(mode, &raw);
        tamper(&mut ordinary.results);
        hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture)).0 .1
    };
    let first = |rows: &Vec<ResultItem>, id: &str| rows.iter().position(|r| r.basis_ref.as_ref().unwrap().ref_id == id).unwrap();
    for (label, tamper) in [
        ("a split run", Box::new(move |rows: &mut Vec<ResultItem>| {
            let (range, twice) = (first(rows, "combination:range"), first(rows, "combination:2case"));
            let row = rows.remove(twice);
            rows.insert(range, row);
        }) as Box<dyn Fn(&mut Vec<ResultItem>)>),
        ("an unknown combination", Box::new(move |rows: &mut Vec<ResultItem>| {
            let twice = first(rows, "combination:2case");
            rows[twice].basis_ref.as_mut().unwrap().ref_id = "combination:other".into();
        })),
        ("a case row after them", Box::new(move |rows: &mut Vec<ResultItem>| {
            let row = rows.remove(0);
            rows.push(row);
        })),
    ] {
        assert_eq!(run(tamper.as_ref()).err(), Some(W1Fallback::Preparation), "{label}");
    }
    assert!(run(&|_| {}).is_ok(), "control: the untampered layout reaches precommit and validates (B2's RS reader)");
}

/// B2-P (C-1): a mechanics combination naming one case twice is `ordinary` (the ordinary route
/// publishes no row for it), not retained.
#[test]
fn b2p_repeated_case_combination_is_ordinary() {
    let mut raw = raw();
    raw["model"]["combinations"] = json!([{"id": "combination:twice", "basis": "mechanics", "terms": [{"load_case": "case", "factor": 1.0},
        {"load_case": "case", "factor": 1.0}], "provenance": B2P}]);
    for mode in MODES {
        let (_, s) = b2p_successor(&raw, mode);
        let b = &s["retained_precision"]["body"];
        assert_eq!((&b["combinations"][0]["disposition"], &b["combinations"][0]["result_ids"]), (&json!("ordinary"), &json!([])), "{mode:?}");
        assert_eq!(b["calls"].as_array().unwrap().len(), 1, "{mode:?}: no combination Call");
    }
}

/// B2-P (REVISION_01 §1.2; DEF-C r2 `stages.observables`): the combination observables stage on
/// W-CB3's ordinary combination block (its values pass the guard, as the ordinary route forms
/// them), and each check refusing one tamper.
#[test]
fn b2p_combination_observables_stage() {
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = w_cb3();
    let (capture, observer, ordinary) = observed(mode, &raw);
    let mut prepared = match observer.prepare_cases(ordinary, 2, &[0]) { Ok(p) => p, Err(f) => panic!("{:?}", f.error) };
    let _ = &capture;
    let rows = prepared.capture.combination_rows[0].clone();
    let check = |prepared: &rp::PreparedCases, tamper: &dyn Fn(&mut MechanicsEnvelope)| {
        let mut ordinary = prepared.ordinary.clone();
        tamper(&mut ordinary);
        prepared.capture.test_combination_observables(&ordinary, rows.clone(), 0, "combination:ab").err().map(|e| e.to_string())
    };
    assert_eq!(check(&prepared, &|_| {}), None, "control");
    let (start, end) = (rows.start, rows.end);
    // The block's first row of `kind` with a nonzero value.
    let find = move |e: &MechanicsEnvelope, kind: &str| (start..end).find(|&i| e.results[i].kind == kind && e.results[i].value != 0.0).unwrap();
    let cases: Vec<(&str, Box<dyn Fn(&mut MechanicsEnvelope)>, &str)> = vec![
        ("a magnitude off by more than 64 eps", Box::new(move |e| { let i = find(e, "displacement_magnitude"); e.results[i].value *= 1.0 + 2f64.powi(-40); }), "combination magnitude guard"),
        ("a duplicated translation", Box::new(move |e| {
            let i = find(e, "global_nodal_displacement_x");
            let node = e.results[i].entity_ref.clone();
            let j = (start..end).find(|&j| e.results[j].kind == "global_nodal_displacement_y" && e.results[j].entity_ref == node).unwrap();
            e.results[j].kind = "global_nodal_displacement_x".into();
        }), "combination displacement identity"),
        ("a second row of one translation", Box::new(move |e| {
            let i = find(e, "global_nodal_displacement_x");
            let node = e.results[i].entity_ref.clone();
            let j = (start..end).find(|&j| e.results[j].kind == "global_nodal_rotation_x" && e.results[j].entity_ref == node).unwrap();
            e.results[j].kind = "global_nodal_displacement_x".into();
            e.results[j].value = e.results[i].value;
        }), "combination displacement identity"),
        ("a support magnitude off", Box::new(move |e| { let i = find(e, "support_reaction_force_magnitude_v2"); e.results[i].value = e.results[i].value * 2.0 + 1.0; }), "support guard"),
        ("a withheld gate entry", Box::new(|e| { e.contract_evidence.as_mut().unwrap()["combination_gates"][0]["withheld"] = json!(true); }), "combination gate entry"),
        ("a maximum row", Box::new(move |e| { let i = find(e, "displacement_magnitude"); e.results[i].kind = "pipe_elastic_normal_stress_maximum_v2".into(); }), "combination maximum or intensified row"),
    ];
    for (label, tamper, expected) in &cases {
        assert_eq!(check(&prepared, tamper.as_ref()).as_deref(), Some(*expected), "{label}");
    }
    prepared.capture.native_invocation = None;
}

/// B2-P (B2-C §2.7): a combination's expression from the invocation: a range's operand ids are
/// sorted as the producer sorts them (UTF-8 byte order), whatever their authored order; a
/// subtraction keeps minuend then subtrahend; mechanics terms keep their authored order and their
/// factors' bits.
#[test]
fn b2p_combination_expressions() {
    let mode = PreviewSolverMode::SparseInteractive;
    let range = w_cb3_with(json!({"id": "combination:range-ba", "basis": "range_envelope", "operand_ids": ["case:b", "case:a"], "mode": "min", "provenance": B2P}));
    let (_, s) = b2p_successor(&range, mode);
    assert_eq!(s["retained_precision"]["body"]["combinations"][0]["expression"], json!({"kind":"range_envelope","operand_ids":["case:a","case:b"],"mode":"min"}));
    let (_, s) = b2p_successor(&w_cb4a(), mode);
    assert_eq!(s["retained_precision"]["body"]["combinations"][0]["expression"], json!({"kind":"result_state_subtraction","minuend_id":"case:a","subtrahend_id":"case:b"}));
    let mut halves = w_cb3();
    halves["model"]["combinations"][0]["terms"] = json!([{"load_case": "case:b", "factor": 0.1}, {"load_case": "case:a", "factor": -2.5}]);
    let (_, s) = b2p_successor(&halves, mode);
    assert_eq!(s["retained_precision"]["body"]["combinations"][0]["expression"], json!({"kind":"mechanics","terms":[
        {"case_id":"case:b","factor":format!("{:016x}", 0.1f64.to_bits())},{"case_id":"case:a","factor":format!("{:016x}", (-2.5f64).to_bits())}]}));
}

/// B2-P (T-2′; REVISION_01 S-1): the capture's normalization refuses a combination D1.4 does not
/// admit (here h = 4), by the predicate T-4's re-check uses, so custody refuses with that cause.
#[test]
fn b2p_capture_refuses_combinations_outside_d14() {
    let mut raw = raw();
    raw["model"]["combinations"] = json!([{"id": "combination:four", "basis": "mechanics", "terms": vec![json!({"load_case": "case", "factor": 1.0}); 4], "provenance": B2P}]);
    let (_, observer, ordinary) = observed(PreviewSolverMode::SparseInteractive, &raw);
    assert_eq!(ordinary.status.mechanics, "MECHANICS_SOLVED", "the ordinary route solves it");
    assert_eq!(observer.error.as_ref().map(|e| e.to_string()).as_deref(), Some("outside private ordinary no-component/no-combination scope"));
    assert!(observer.prepare_cases(ordinary, 1, &[0]).is_err(), "custody refuses");
}

// ---- W-CB1's inputs: SW's cap-maximal cases A and B (R/I86 `gen_inputs.py` `build_i3`, c1), and
// I98's `r7_cb1_halfb` and `r7_cb1` on them --------------------------------------------------------

const SW_PROV: &str = "invented_t3_b1_sw_probe_input_no_library_data";
/// I86's `split_exact`: `v` split into parts proportional to `weights`, each an integer multiple
/// of v's quantum 2^E whose integer sum is v's mantissa M, so every partial sum is exact.
fn sw_split_exact(v: f64, weights: &[u64]) -> Vec<f64> {
    let bits = v.to_bits();
    assert!(v.is_normal(), "a normal net");
    let (sign, exponent, mantissa) = (if v < 0.0 { -1.0 } else { 1.0 }, ((bits >> 52) & 0x7ff) as i32 - 1075, (bits & ((1u64 << 52) - 1)) | (1u64 << 52));
    let total: u64 = weights.iter().sum();
    let mut ints: Vec<u64> = weights.iter().map(|&w| ((mantissa as u128 * w as u128) / total as u128) as u64).collect();
    let sum: u64 = ints.iter().sum();
    *ints.last_mut().unwrap() += mantissa - sum;
    let quantum = f64::from_bits(((exponent + 1023) as u64) << 52);
    let parts: Vec<f64> = ints.iter().map(|&i| sign * (i as f64) * quantum).collect();
    assert_eq!(parts.iter().fold(0.0, |a, p| a + p), v, "exact in order");
    parts
}
/// I86's `copies_loads`: `total` moments over 7 milestone copies, three rotational DOFs at each
/// copy's N1; copy k's net on each DOF is exactly `scale` times the milestone's.
fn sw_copies_loads(prefix: &str, total: usize, scale: f64, weights: impl Fn(usize) -> Vec<u64>) -> Vec<Value> {
    let ms = raw();
    let base = |d: &str| ms["model"]["load_cases"][0]["primitive_loads"].as_array().unwrap().iter()
        .find(|l| l["direction"] == d).unwrap()["magnitude"]["value"].as_f64().unwrap();
    let slots: Vec<(usize, &str)> = (0..7).flat_map(|k| ["RX", "RY", "RZ"].map(|d| (k, d))).collect();
    let mut per = vec![total / slots.len(); slots.len()];
    for p in per.iter_mut().take(total - (total / slots.len()) * slots.len()) {
        *p += 1;
    }
    let mut out = Vec::new();
    for (&(k, d), &n) in slots.iter().zip(&per) {
        for (j, part) in sw_split_exact(base(d) * scale, &weights(n)).into_iter().enumerate() {
            out.push(json!({"id": format!("{prefix}C{k}:{d}:{j}"), "category": "concentrated_moment", "target": {"type": "node", "node": format!("C{k}:N1")},
                "direction": d, "magnitude": {"value": part, "unit": "N*m"}, "dimension": "moment", "provenance": SW_PROV}));
        }
    }
    assert_eq!(out.len(), total);
    out
}
/// I86's `perpendicular_reference`.
fn sw_perpendicular(d: [f64; 3]) -> Value {
    for r in [[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
        let c = [d[1] * r[2] - d[2] * r[1], d[2] * r[0] - d[0] * r[2], d[0] * r[1] - d[1] * r[0]];
        let n = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
        if n > 0.5 * (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() {
            return json!({"x": r[0], "y": r[1], "z": r[2]});
        }
    }
    unreachable!("a perpendicular reference")
}
/// I86's `filler_bodies`: four anchored, connected, unloaded bodies (18 nodes, 25 members, 4 supports).
fn sw_filler() -> (Vec<Value>, Vec<Value>, Vec<Value>) {
    let shapes: [&[[f64; 3]]; 4] = [
        &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0, 1.0]],
        &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.5, 0.5, 1.0]],
        &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0]],
    ];
    let all = |n: usize| (0..n).flat_map(move |i| (i + 1..n).map(move |j| (i, j))).collect::<Vec<_>>();
    let edges: [Vec<(usize, usize)>; 4] = [all(5), vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 4), (2, 4)], all(4), vec![(0, 1), (1, 2), (2, 3)]];
    let (mut nodes, mut pipes, mut supports) = (Vec::new(), Vec::new(), Vec::new());
    for (b, (shape, es)) in shapes.iter().zip(&edges).enumerate() {
        let origin = [100.0 + 10.0 * b as f64, 50.0, 0.0];
        let ids: Vec<String> = (0..shape.len()).map(|i| format!("F{b}:N{i}")).collect();
        for (i, p) in shape.iter().enumerate() {
            nodes.push(json!({"id": ids[i], "position": {"x": origin[0] + p[0], "y": origin[1] + p[1], "z": origin[2] + p[2]}, "provenance": SW_PROV}));
        }
        for &(i, j) in es {
            let d = [shape[j][0] - shape[i][0], shape[j][1] - shape[i][1], shape[j][2] - shape[i][2]];
            pipes.push(json!({"id": format!("F{b}:M{i}{j}"), "from": ids[i], "to": ids[j], "material": "mat:1", "y_reference": sw_perpendicular(d),
                "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}}, "provenance": SW_PROV}));
        }
        supports.push(json!({"id": format!("F{b}:anchor"), "node": ids[0], "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": SW_PROV}));
    }
    assert_eq!((nodes.len(), pipes.len(), supports.len()), (18, 25, 4));
    (nodes, pipes, supports)
}
/// I86's `materials` with 16 temperature points.
fn sw_materials() -> Value {
    let points: Vec<Value> = (0..16).map(|i| json!({"id": format!("T{i}"), "provenance": SW_PROV})).collect();
    json!((0..4).map(|i| json!({"id": format!("mat:{i}"), "elastic_modulus": {"value": 200000000000.0, "unit": "Pa"},
        "shear_modulus": {"value": 80000000000.0, "unit": "Pa"}, "provenance": SW_PROV, "temperature_points": points})).collect::<Vec<_>>())
}
/// I86's `stressed`: every provenance string escaped, and a depth-16 unknown member.
fn sw_stressed(mut raw: Value) -> Value {
    fn escape(v: &mut Value) {
        match v {
            Value::Object(o) => for (k, x) in o.iter_mut() {
                match x { Value::String(s) if k == "provenance" => s.push_str(" q\"b\\"), _ => escape(x) }
            },
            Value::Array(a) => a.iter_mut().for_each(escape),
            _ => {}
        }
    }
    escape(&mut raw);
    let mut deep = json!(1);
    for _ in 0..14 {
        deep = json!([deep]);
    }
    raw["model"]["unknown_depth_witness"] = deep;
    raw
}
/// I86's `build_model` (7 copies, 16 points) with one case `case_id` of `loads`, stressed.
fn sw_component(case_id: &str, loads: Vec<Value>) -> Value {
    let ms = raw();
    let m = &ms["model"];
    let (mut nodes, mut pipes, mut supports) = (Vec::new(), Vec::new(), Vec::new());
    for k in 0..7 {
        let dx = 10.0 * k as f64;
        for n in m["nodes"].as_array().unwrap() {
            let p = &n["position"];
            nodes.push(json!({"id": format!("C{k}:{}", n["id"].as_str().unwrap()), "position": {"x": p["x"].as_f64().unwrap() + dx, "y": p["y"], "z": p["z"]},
                "provenance": n["provenance"]}));
        }
        for pipe in m["pipe_segments"].as_array().unwrap() {
            let mut q = pipe.clone();
            q["id"] = json!(format!("C{k}:{}", pipe["id"].as_str().unwrap()));
            q["from"] = json!(format!("C{k}:{}", pipe["from"].as_str().unwrap()));
            q["to"] = json!(format!("C{k}:{}", pipe["to"].as_str().unwrap()));
            q["material"] = json!("mat:0");
            pipes.push(q);
        }
        for s in m["supports"].as_array().unwrap() {
            let mut q = s.clone();
            q["id"] = json!(format!("C{k}:{}", s["id"].as_str().unwrap()));
            q["node"] = json!(format!("C{k}:{}", s["node"].as_str().unwrap()));
            supports.push(q);
        }
    }
    let (fn_, fp, fs) = sw_filler();
    nodes.extend(fn_);
    pipes.extend(fp);
    supports.extend(fs);
    assert_eq!((nodes.len(), pipes.len(), supports.len()), (32, 32, 32));
    let mut project_id = "invented:t3-b1-sw:".to_owned();
    while project_id.len() < 128 {
        project_id.push('x');
    }
    let case = json!({"id": case_id, "label": "I86 B1-SW probe case", "kind": "primitive_user_load", "primitive_loads": loads, "provenance": SW_PROV});
    sw_stressed(json!({"model": {"schema_version": m["schema_version"], "document_kind": m["document_kind"], "analysis_status": m["analysis_status"],
        "project": {"id": project_id, "units": m["project"]["units"]}, "nodes": nodes, "pipe_segments": pipes, "materials": sw_materials(),
        "supports": supports, "load_cases": [case], "combinations": []}, "materials": sw_materials()}))
}
/// W-CB1 (I98 `r7_cb1_halfb`): SW's components A (each copy's net the milestone's) and B (minus
/// it, parts weighted 1..n) as two cases, and 1·A + 0.5·B. `factor` 1.0 gives W-CB1z (`r7_cb1`).
fn w_cb1_with(factor: f64, label: &str) -> Value {
    let a = sw_component("case:a", sw_copies_loads("a:", 128, 1.0, |n| vec![1; n]));
    let b = sw_component("case:b", sw_copies_loads("b:", 128, -1.0, |n| (1..=n as u64).collect()));
    let mut raw = a.clone();
    raw["model"]["load_cases"] = json!([a["model"]["load_cases"][0], b["model"]["load_cases"][0]]);
    raw["model"]["combinations"] = json!([b2w_combination("case:a", "case:b", factor, label)]);
    raw
}
fn w_cb1() -> Value { w_cb1_with(0.5, "I98 B2-W R-7 count: A + 0.5 B") }
fn w_cb1z() -> Value { w_cb1_with(1.0, "I98 B2-W R-7 count: A + B") }


/// The witnesses' combination dispositions, in authored order (W-CB1z, A + B, is the labelled
/// cancellation pin: its copies' nets cancel exactly).
fn b2p_dispositions(name: &str) -> &'static [&'static str] {
    match name {
        "w_cb2" => &["retained_unavailable"],
        "w_cb4a" | "w_cb4b" | "w_cb5" => &["ordinary"],
        "c1_range_mechanics" => &["ordinary", "retained_selected"],
        "rv123_c1_two_mechanics" => &["retained_selected", "retained_unavailable"],
        _ => &["retained_selected"],
    }
}
/// B2-P's witness pins (PLAN §1.2.6; REVISION_01 §5.1), both modes: (witness, mode, receipt
/// sha256, sha256 of the successor's bytes), from the private driver's successor that precommit
/// receives.
const B2P_PINNED: [(&str, &str, &str, &str); 20] = [
    ("w_cb1", "sparse_interactive", "6345c3a32dd243ecc877ebce349307602cfe65e700048981d9171f4c15ee7880",
        "d5fa0b8bc48646c88506490eda71ba9264244dbe3f6658816d83e6b6d62b7e7a"),
    ("w_cb1", "dense_scrutiny", "3b24b62e72b366554e51d621f395c6128a75f1fe9e92f6b42e4da47e697750ed",
        "295f3575f6a475e69f3d7cb7db4326983aefaba3caa1e31d9a711c68ea431b70"),
    ("w_cb1z", "sparse_interactive", "133b60a763a9cc6bb85541d18dce65e8ecdb550e8538e5d167d5a706adfd0b12",
        "ad01b6adf436777a4b370bfaa152e8fe4459ae8f28d048b7ff992405004311ec"),
    ("w_cb1z", "dense_scrutiny", "70e5bfc9569f4d6f2a958dfecee0f541602caf9e84796187c0754c8fbfede71b",
        "d38ff80c33deae242fc3e440ac9270a5299d198a053e1e273a7b8b23789441fa"),
    ("w_cb2", "sparse_interactive", "154c52607fe0a4a91d591e48af428c360f17cd043d9f5621acfcadf15db48605",
        "9731586e98ab443c08636931f29ace24be49e0006ac574b2b4526c9155993b9e"),
    ("w_cb2", "dense_scrutiny", "94e468af39678f62bffc9ab8ec426ff3d4b18f59c5cf4232eb42a136f7263edd",
        "cf9d4c6bb8a7546b515a5d74f25276598be3954953a2be4d5480589c732500cf"),
    ("w_cb3", "sparse_interactive", "f86ffd2b3b403c84fca91aeb686028e7d613c23f291f23b2acdc91633bea3e99",
        "82edc28b3f797c3d41a14dd31fcc2185baf630a76e6a857681dd435c00a6a9f5"),
    ("w_cb3", "dense_scrutiny", "beb1461c0860663d74dd2b012aeedfa2667e701c71073ca59c6ea842bb9f786d",
        "d8a46a71f9d3a5b32dc03b5a9724eacc2956ab56936ab488182c3a48577e9f1e"),
    ("w_cb4a", "sparse_interactive", "7b8773530d63fb32542210bb6ad4c709fe957f554da835b9efb7ccd485883fe9",
        "318c4abd082c1df910a91803ccc14973f498cdad7d2c13a40004bbc7ca7afbc0"),
    ("w_cb4a", "dense_scrutiny", "00e21ac355d409cdff9b666ce9587427dd0ff77ccd412f3fe568735df0c9cc4a",
        "f0210262e38db5e7d5b7aa46c4d03d869c6cb0b4940646e7b9b3149feb43f714"),
    ("w_cb4b", "sparse_interactive", "31e0fd4804064ffbc298fccec0981cc0aab7db9348f583094ff2f83795f0a643",
        "c291b532c781092d6b13aeec9dbf055631181169fb43edb40a18f6120ddb6e73"),
    ("w_cb4b", "dense_scrutiny", "2c5b46caf32f625089637b545294dc3a24739fad151274e29d2831066afbf330",
        "c558ccfe5492902f231d01eab776fa88a352dbb70addf3042929aeb4e0195c3b"),
    ("w_cb5", "sparse_interactive", "e26015faa2ab14d571ad67c037a3a4afa69b1b4d97841a31b2fb4713b2bda3fe",
        "8e83f75890ef99057942f4e10b6b378fb56b1363e59fdd52050d1659078124a9"),
    ("w_cb5", "dense_scrutiny", "f27870de60f57d3e28998305f856079e3f788762f562f58e38af1419d5144c58",
        "265a84bbd39b4380cb2c950847f0e47d60fe56b74bcb6e207b86e2701baf1834"),
    ("c1_range_mechanics", "sparse_interactive", "877e0c7a32f4740b6a907ac4cb9d63e6f018687208ba396148b22112ee5956d8",
        "d32ed8bf6594ce7107763bc31f9fc1d2d989a644d1538243b4b78aec79951ce8"),
    ("c1_range_mechanics", "dense_scrutiny", "5b2812606ff71ba390602070528ea00a13848e59847ba6d7a13da424aaf8d450",
        "bc641fe3414556f2e74d0b9f57f685d58029cc7de9feb250cabba8b432c2fbbf"),
    ("rv123_w_cb3_ba", "sparse_interactive", "86d0f1f821bb212fdea7f431291fe4b22b92c3564320b91911abe04ab50686ca",
        "ec58bcab704f8e870947a90a48321f211637e0131288a86ea1116aeb188751ba"),
    ("rv123_w_cb3_ba", "dense_scrutiny", "03a8eec6106b3c507ca4a7648e0010f6c382e9ece099521b2a0d9044486c2454",
        "6149647c6cd9264c0fad7760e05c6148462ec36b986eac1e9ad62978f98ecd6c"),
    ("rv123_c1_two_mechanics", "sparse_interactive", "6d9f6fc9a26d91402cec7dab0ec3996a22ddecc84bd8a2a8925b1eb1ba7984ee",
        "98bddd8285e488ff6789048dd0fba68da963a0226cd2be4d408db7be8d780402"),
    ("rv123_c1_two_mechanics", "dense_scrutiny", "88324f91e52426b4f966acaf28063d6b75c4b1b4163b0d233efdfcdffddda577",
        "d608a0b03e2cb7fa8720bddc706b9706c958d78d8c9a9383b6a7cef05cbb4e1c"),
];
/// B2-P's small witnesses: (name, request).
fn b2p_witnesses() -> Vec<(&'static str, Value)> {
    vec![("w_cb2", w_cb2()), ("w_cb3", w_cb3()), ("w_cb4a", w_cb4a()), ("w_cb4b", w_cb4b()), ("w_cb5", w_cb5()), ("c1_range_mechanics", b2_c1_range_mechanics())]
}
/// W-CB1 and W-CB1z: I98's `r7_cb1_halfb` and `r7_cb1` (SW's cap-maximal cases A and B; 1·A + 0.5·B,
/// and A + B, the labelled cancellation pin, NB-3).
fn b2p_cap_witnesses() -> Vec<(&'static str, Value)> {
    vec![("w_cb1", w_cb1()), ("w_cb1z", w_cb1z())]
}
/// The combination successor fixtures (PLAN §1.2.6: "W-CB1 or W-CB3, whichever selects"; both
/// select, and W-CB3 is the one with an operand preparation, at a fixture's size): the live W-CB3
/// successor documents `{id, source, invocation}`, both modes, byte for byte.
const COMBINATION_FIXTURES: [&str; 2] = [
    include_str!("../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json"),
    include_str!("../../../fixtures/results/retained_precision_combination_successor_dense_scrutiny.json")];
fn combination_document(raw: &Value, mode: PreviewSolverMode, successor: &Value) -> String {
    serde_json::to_string_pretty(&json!({"id": format!("b2p_w_cb3_{}", mode.as_str()), "source": successor,
        "invocation": {"request": raw, "solver_mode": mode.as_str()}})).unwrap()
}
/// W-CB1's and W-CB1z's request Value sha256s (`w_cb1`, `w_cb1z`; JSON-equal to I98's
/// `r7_cb1_halfb` and `r7_cb1`, checked on the records).
const W_CB1_INPUT_SHA256: &str = "c1b85bd47605ca0df6f847be1b2b23b91c544d6981c4208f68b67d2c499a7ceb";
const W_CB1Z_INPUT_SHA256: &str = "9c21d45e0ee9314946a7f2a7e88e55f73488d8cb1af0efd4912eb0a07399d3c1";
/// The B2-P witnesses' inputs: W-CB2 and W-CB3 are I98's `r7_cb2` and `r7_cb3_v1`, W-CB1 and
/// W-CB1z its `r7_cb1_halfb` and `r7_cb1` (JSON-equal, checked on the records); their Value
/// sha256s. With `I105_B2P_OUT` set, W-CB1's and W-CB1z's requests are written there.
#[test]
fn b2p_witness_inputs() {
    let out = std::env::var("I105_B2P_OUT").ok();
    let mut got = Vec::new();
    for (name, raw, expected) in [("w_cb1", w_cb1(), W_CB1_INPUT_SHA256), ("w_cb1z", w_cb1z(), W_CB1Z_INPUT_SHA256),
        ("w_cb2", w_cb2(), "7f07d08e694947addf9e90f023fee9080b8b2a71d647260d528ce01c44c509a0"),
        ("w_cb3", w_cb3(), "05e9ae15f4b6155b5907dec9ed6a56084b511d26777cfd6c7e2ea8ebe998d28d")] {
        let value_sha = sha(&serde_json::to_vec(&raw).unwrap());
        println!("B2P_INPUT {name} value_sha={value_sha}");
        if let (Some(dir), true) = (&out, name.starts_with("w_cb1")) {
            std::fs::write(format!("{dir}/{name}_request.json"), serde_json::to_vec(&raw).unwrap()).unwrap();
        }
        got.push((name, value_sha, expected));
    }
    for (name, value_sha, expected) in got {
        assert_eq!(value_sha, expected, "{name}");
    }
    // W-CB1: two cases of 128 loads each at D1's caps (C_eq = 3), 1·A + 0.5·B.
    let raw = w_cb1();
    assert_eq!(raw["model"]["load_cases"].as_array().unwrap().iter().map(|c| c["primitive_loads"].as_array().unwrap().len()).collect::<Vec<_>>(), [128, 128]);
    assert_eq!(raw["model"]["combinations"][0]["terms"], json!([{"load_case":"case:a","factor":1.0},{"load_case":"case:b","factor":0.5}]));
}
/// B2-P: DEF-C's id and its table-bound H against the in-tree statics (B2-C REVISION_02: DEF-C r2,
/// PTABLE r2), as the receipt's combination attempts name them.
#[test]
fn b2p_constants_bound_to_in_tree_fixtures() {
    use super::retained_wire as wire;
    let definition: Value = serde_json::from_str(include_str!("../../../fixtures/results/retained_precision_prepared_combination_v1.json")).unwrap();
    let table: Value = serde_json::from_str(include_str!("../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json")).unwrap();
    let h = wire::domain_hash("retained_precision_formation_v1", &definition).unwrap();
    assert_eq!(definition["id"], json!(wire::COMBINATION_DEFINITION_ID));
    assert_eq!(h, "d3fde142aff9c05d709b2fc2a04add42e14c66be3e2b2ba82012da57edf3d957");
    assert!(table["product_formation_definitions"].as_array().unwrap().contains(&json!({"id": wire::COMBINATION_DEFINITION_ID, "sha256": h})));
}

/// B2-P's producer requirements on one witness successor (B2-C §2.5–§2.7, §4, C3a; REVISION_01),
/// against its ordinary envelope `plain`:
/// - `combinations[]` one entry per model combination, in authored order, with its basis, its
///   rows in publication order and its expression; R-COMB-1's producer side: an `ordinary` or
///   `retained_unavailable` combination's rows keep their ordinary values and carry no
///   `recovery_method`, and a `retained_selected` one's carry it;
/// - every Call and Run in order (the meter chain; `execution_order` with combination owners by
///   authored index); each CombinationSource's operands, K4CMB and ledger hashes, and identity;
/// - each combination diagnostic, after the case diagnostics; no headline from a combination row.
fn assert_combination_successor(label: &str, successor: &Value, plain: &Value) {
    let body = &successor["retained_precision"]["body"];
    let model = &successor_request(successor, plain);
    let combinations = body["combinations"].as_array().unwrap();
    assert_eq!(combinations.len(), model.len(), "{label}");
    let rows = |s: &Value, id: &str| s["results"].as_array().unwrap().iter().filter(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == id).cloned().collect::<Vec<_>>();
    for (c, authored) in combinations.iter().zip(model) {
        let id = authored["id"].as_str().unwrap();
        assert_eq!(c["basis_ref"], json!({"ref_type":"combination","ref_id":id}), "{label}");
        let (now, was) = (rows(successor, id), rows(plain, id));
        assert_eq!(c["result_ids"], json!(now.iter().map(|r| r["id"].clone()).collect::<Vec<_>>()), "{label} {id}: result_ids");
        assert_eq!(now.len(), was.len(), "{label} {id}");
        match c["disposition"].as_str().unwrap() {
            "retained_selected" => {
                assert!(now.iter().all(|r| r["recovery_method"] == "contribution_preserving_multiprecision_v1"), "{label} {id}");
                let diag = successor["diagnostics"].as_array().unwrap().iter().find(|d| d["id"] == json!(format!("diagnostic:retained-precision:{id}:selected"))).unwrap();
                assert_eq!(diag["code"], json!("RETAINED_PRECISION_SELECTED"), "{label}");
            }
            disposition => {
                // R-COMB-1's producer side: the ordinary rows, untouched.
                for (n, w) in now.iter().zip(&was) {
                    assert_eq!(n, w, "{label} {id}: {disposition} rows keep their ordinary values");
                }
                if disposition == "retained_unavailable" {
                    assert_eq!(c["diagnostic_ref"], json!(format!("diagnostic:retained-precision:{id}:unavailable")), "{label}");
                }
            }
        }
    }
    // The summary's headlines are load-case rows (T-11′).
    for headline in ["max_displacement", "max_open_formula_stress"] {
        let r = &successor["summary"][headline]["result_ref"];
        if r.is_null() { continue; }
        let row = successor["results"].as_array().unwrap().iter().find(|x| &x["id"] == r).unwrap();
        assert_eq!(row["basis_ref"]["ref_type"], json!("load_case"), "{label}: {headline}");
    }
    // The meter chain and the Runs (B2-C §2.7): calls[0] is the batch; each later Call's before is
    // the previous Call's after; charged is the last Call's after.
    let calls = body["calls"].as_array().unwrap();
    assert_eq!(calls[0]["kind"], json!("case_batch"), "{label}");
    for pair in calls.windows(2) {
        assert_eq!(pair[1]["invocation_before"], pair[0]["invocation_after"], "{label}: the meter chain");
        assert_eq!(pair[1]["kind"], json!("mechanics_combination"), "{label}");
    }
    assert_eq!(body["work"]["charged"], calls.last().unwrap()["invocation_after"], "{label}");
    for c in combinations.iter().filter(|c| c["run"].is_object()) {
        let index = combinations.iter().position(|x| x == c).unwrap();
        assert!(body["work"]["execution_order"].as_array().unwrap().contains(&json!({"kind":"combination","index":index})), "{label}: the ordinal mapping");
        assert_eq!(c["run"]["origin"]["owner_ref"], json!({"kind":"combination","index":index}), "{label}");
        let call = &calls[c["call_ref"].as_u64().unwrap() as usize];
        assert_eq!(call["owner_refs"], json!([{"kind":"combination","index":index}]), "{label}");
        let source = &body["sources"][c["source_ref"].as_u64().unwrap() as usize];
        assert_eq!(source["owner"]["combination_index"], json!(index), "{label}");
        assert_eq!(source["representative_source_ref"], source["operands"][0]["source_ref"], "{label}: operand 0 is the representative");
        for (operand, requested) in source["operands"].as_array().unwrap().iter().zip(call["requested_operands"].as_array().unwrap()) {
            assert_eq!((&operand["source_ref"], &operand["factor"]), (&requested["source_ref"], &requested["factor"]), "{label}");
            let case_source = &body["sources"][operand["source_ref"].as_u64().unwrap() as usize];
            let mut binding = case_source.clone();
            binding.as_object_mut().unwrap().remove("index");
            assert_eq!(operand["source_identity_sha256"], json!(super::retained_wire::domain_hash("retained_precision_source_mp_v2", &binding).unwrap()), "{label}");
        }
        if c["disposition"] == "retained_selected" {
            let mut binding = source.clone();
            binding.as_object_mut().unwrap().remove("index");
            assert_eq!(c["source_identity_sha256"], json!(super::retained_wire::domain_hash("retained_precision_source_mp_v2", &binding).unwrap()), "{label}");
            assert_eq!(c["selection"]["ledger_sha256"], source["ledger_sha256"], "{label}: the combined ledger");
        }
    }
}
/// The authored combinations of the successor's own request (its envelope echoes no request, so
/// the caller's `plain` carries the model's combinations through the published gates' order).
fn successor_request(_successor: &Value, plain: &Value) -> Vec<Value> {
    plain["contract_evidence"]["combination_gates"].as_array().unwrap().iter().map(|g| json!({"id": g["combination_id"]})).collect()
}

/// B2-P's witnesses through the private driver, both modes: the successor precommit receives meets
/// B2-C's producer requirements (`assert_combination_successor`) and is pinned; precommit (B2's RS
/// reader) validates it, so W1's result is that successor and the ordinary owner is untouched.
/// With `I105_B2P_OUT` set, each successor document, and W-CB3's fixture documents, are written
/// there.
#[test]
fn b2p_witness_successors_are_pinned_in_both_modes() {
    b2p_pin_witnesses(b2p_witnesses());
}
/// `b2p_witness_successors_are_pinned_in_both_modes` for W-CB1 and W-CB1z (D1's caps; slower).
#[test]
fn b2p_w_cb1_successors_are_pinned_in_both_modes() {
    b2p_pin_witnesses(b2p_cap_witnesses());
}
fn b2p_pin_witnesses(witnesses: Vec<(&'static str, Value)>) {
    let out = std::env::var("I105_B2P_OUT").ok();
    let mut pins = Vec::new();
    for (name, raw) in witnesses {
        for mode in MODES {
            let label = format!("{name} {mode:?}");
            let plain = plain(mode, &raw);
            let plain_value: Value = serde_json::from_slice(&plain).unwrap();
            let (capture, observer, ordinary) = observed(mode, &raw);
            let ((envelope, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
            let successor = captured.unwrap_or_else(|| panic!("{label}: {retained:?}"));
            let dispositions: Vec<&str> = successor["retained_precision"]["body"]["combinations"].as_array().unwrap().iter().map(|c| c["disposition"].as_str().unwrap()).collect();
            println!("B2P_DISPOSITIONS {label} {dispositions:?}");
            assert_eq!(dispositions, b2p_dispositions(name), "{label}");
            assert_combination_successor(&label, &successor, &plain_value);
            match &retained {
                Ok(validated) => {
                    assert_eq!(validated.value(), &successor, "{label}: the validated successor is the one precommit received");
                    assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "{label}: the ordinary owner is untouched");
                }
                Err(other) => panic!("{label}: B2's RS reader validates every witness successor: {other:?}"),
            }
            let (receipt, bytes) = (successor["retained_precision"]["receipt_sha256"].as_str().unwrap().to_owned(), sha(&serde_json::to_vec(&successor).unwrap()));
            println!("B2P_PIN {name} {} receipt={receipt} bytes={bytes}", mode.as_str());
            let pinned = B2P_PINNED.iter().find(|(n, m, _, _)| *n == name && *m == mode.as_str()).unwrap();
            if let (Some(dir), "w_cb3") = (&out, name) {
                std::fs::write(format!("{dir}/retained_precision_combination_successor_{}.json", mode.as_str()), combination_document(&raw, mode, &successor)).unwrap();
            }
            if let Some(dir) = &out {
                std::fs::write(format!("{dir}/{name}.{}.successor.json", mode.as_str()),
                    serde_json::to_vec(&json!({"source": successor, "invocation": {"request": raw, "solver_mode": mode.as_str()}})).unwrap()).unwrap();
            }
            pins.push((label, (receipt, bytes), (pinned.2, pinned.3)));
        }
    }
    for (label, got, pinned) in pins {
        assert_eq!((got.0.as_str(), got.1.as_str()), pinned, "{label}: pinned");
    }
}
/// The combination successor fixtures are the live W-CB3 successor documents, byte for byte (the
/// successor the private driver hands precommit; on the registered Direct entry the same bytes).
#[test]
fn b2p_combination_successor_fixtures_are_the_live_successors() {
    for (mode, fixture) in MODES.into_iter().zip(COMBINATION_FIXTURES) {
        let raw = w_cb3();
        let (capture, observer, ordinary) = observed(mode, &raw);
        let ((_, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
        let successor = captured.unwrap_or_else(|| panic!("{mode:?}: {retained:?}"));
        assert!(combination_document(&raw, mode, &successor) == fixture, "{mode:?}: the fixture is the live successor document, byte for byte");
        let pinned = B2P_PINNED.iter().find(|(n, m, _, _)| *n == "w_cb3" && *m == mode.as_str()).unwrap();
        assert_eq!(fixture_source_sha(fixture), pinned.3, "{mode:?}");
    }
}
fn fixture_source_sha(fixture: &str) -> String {
    let document: Value = serde_json::from_str(fixture).unwrap();
    sha(&serde_json::to_vec(&document["source"]).unwrap())
}

/// B2-P on the actual Direct entry, both modes (registered build): each witness is admitted (D1.4
/// with combinations), runs one ordinary run and G-C once, and precommit receives the private
/// driver's successor byte for byte. B2's RS reader validates it, so the publication is that
/// successor's bytes. Unregistered builds: no W1.
#[test]
fn b2p_direct_entry_runs_the_combinations() {
    b2p_direct(b2p_witnesses());
}
/// `b2p_direct_entry_runs_the_combinations` for W-CB1 and W-CB1z.
#[test]
fn b2p_w_cb1_direct_entry_runs_the_combinations() {
    b2p_direct(b2p_cap_witnesses());
}
fn b2p_direct(witnesses: Vec<(&'static str, Value)>) {
    for (name, raw) in witnesses {
        for mode in MODES {
            let label = format!("{name} {mode:?}");
            let plain = plain(mode, &raw);
            let (capture, observer, ordinary) = observed(mode, &raw);
            let (_, _, private) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
            let direct_raw = raw.clone();
            let (output, counts, captured) = hooks::counted_with_successor(move || run_linear_static_preview_value_with_retained_direct(direct_raw, mode).unwrap());
            if !registered() {
                assert!(output.retained().is_none() && captured.is_none(), "{label}: no permit, no W1");
                assert_eq!(published(output), plain, "{label}");
                continue;
            }
            assert_eq!(output.admission().unwrap().law().refusal, None, "{label}: admitted");
            assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{label}");
            assert_eq!(captured, private, "{label}: the Direct entry's successor is the private driver's");
            match output.retained().unwrap().clone() {
                Ok(_) => assert_eq!(published(output), serde_json::to_vec(&captured.unwrap()).unwrap(), "{label}: the published bytes are the successor's"),
                Err(other) => panic!("{label}: B2's RS reader validates every witness successor: {other:?}"),
            }
        }
    }
}

/// RV125 N-2 (a forward constraint on lane P): the capture's modulus-basis custody
/// (`selections`, `basis_record`, `basis_record_calls`, `basis_expected`) is the invocation's,
/// not a case's. No B2-P or B3b-P path admits a material selector at c ≥ 2, so it stays there:
/// - on the Direct entry, G-A refuses a selector on any case (D1.5, `ModulusBasisRef`): no W1,
///   one run, the plain bytes;
/// - on the private driver (no G-A), the preview route's capture refuses it, so no successor
///   reaches precommit: a selector on case 0 fails the custody (`Preparation`), and one on case 1
///   only fails every attempted case's freeze (`Candidate`). The exact route refuses any
///   selector (P-5).
///
/// Pinned for c = 2 without and with a combination, a point basis on either case or both.
#[test]
fn b2p_selectors_at_two_cases_are_refused() {
    use super::retained_memory::{AdmissionRefusal, D1Clause, FamilyFact};
    let mut no_combination = b2p_two_cases();
    no_combination["model"]["combinations"] = json!([]);
    for (base, raw) in [("c = 2", no_combination), ("c = 2, z = 1", b2p_two_cases())] {
        for selected in [&[0usize][..], &[1][..], &[0, 1][..]] {
            let mut raw = raw.clone();
            raw["model"]["materials"][0]["temperature_points"] = json!([{"id": "T0", "temperature": {"value": 20, "unit": "degC"},
                "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "shear_modulus": {"value": 8.0e10, "unit": "Pa"}, "provenance": B2P}]);
            for &case in selected {
                raw["model"]["load_cases"][case]["modulus_basis_ref"] = json!("T0");
            }
            for mode in MODES {
                let label = format!("{base}, selected {selected:?} {mode:?}");
                let plain = plain(mode, &raw);
                let plain_value: Value = serde_json::from_slice(&plain).unwrap();
                assert_eq!(plain_value["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{label}: the ordinary route solves it");
                assert_eq!(plain_value["results"].as_array().unwrap().iter().filter(|r| r["kind"] == "modulus_basis_record").count(), selected.len(), "{label}");
                // The Direct entry: G-A refuses (D1.5), whatever the build.
                let (output, counts) = direct(&raw, mode);
                assert_eq!(output.admission().unwrap().law().domain, Some(AdmissionRefusal::Family(D1Clause::Case, FamilyFact::ModulusBasisRef)), "{label}");
                assert!(output.retained().is_none(), "{label}: no W1");
                assert_eq!(counts, ONE_RUN, "{label}");
                assert_eq!(published(output), plain, "{label}: the plain bytes");
                // The private driver: the capture's custody refuses, so no successor.
                let (capture, observer, ordinary) = observed(mode, &raw);
                let ((_, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
                // A selector on case 0 fails the custody (`Preparation`); on case 1 only, every
                // attempted case's freeze refuses (`Candidate`).
                let expected = if selected.contains(&0) { W1Fallback::Preparation } else { W1Fallback::Candidate };
                assert_eq!(retained.err(), Some(expected), "{label}");
                assert!(captured.is_none(), "{label}: no successor reaches precommit");
            }
        }
    }
}

/// RV125 N-3: inside `with_case(0, ..)` at c = 2, case 0's parked slot holds a default and the
/// last-seen case's fields hold case 0, so reading either through the per-case accessors is a
/// cross-case read, which debug builds refuse. Outside `with_case` both reads are allowed.
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "read across with_case")]
fn b2p_with_case_guards_cross_case_reads() {
    let (_, mut observer, _) = observed(PreviewSolverMode::SparseInteractive, &b2p_two_cases());
    assert_eq!(observer.cases_seen(), 2);
    let _control = (observer.case_scope(0), observer.case_scope(1));
    let _ = observer.with_case(0, |capture| capture.case_scope(1));
}

/// B2-P (D1.4; T-4's re-check and T-2′): `w1_combinations_admitted` at each cap's boundary,
/// directly, including z ≤ 2 where C_eq ≤ 3 alone would admit three combinations (c = 0).
#[test]
fn b2p_d14_predicate_at_its_caps() {
    let admitted = |cases: &[&str], combinations: &[(&str, usize, usize)]| super::w1_combinations_admitted(cases.iter().copied(), combinations.iter().copied());
    assert!(admitted(&["a"], &[]), "z = 0");
    assert!(admitted(&["a"], &[("x", 3, 0), ("y", 0, 3)]), "at the caps");
    assert!(!admitted(&[], &[("x", 1, 0), ("y", 1, 0), ("z", 1, 0)]), "z <= 2");
    assert!(!admitted(&["a", "b"], &[("x", 1, 0), ("y", 1, 0)]), "C_eq <= 3");
    assert!(!admitted(&["a"], &[("x", 4, 0)]), "h <= 3");
    assert!(!admitted(&["a"], &[("x", 0, 4)]), "range operands <= 3");
    assert!(!admitted(&["a"], &[("a", 1, 0)]), "ids disjoint (C-9)");
}

/// B2-P (B2-C §2.7): the serializer refuses a successor whose Calls do not chain the
/// invocation's meter (`break_next_meter_chain`): W1 falls back at the serializer with
/// `work_counter_inconsistent` (`work.charged`), and publishes the plain bytes plus case A's
/// notice with that detail. With one Call (z = 0) there is no pair to check: a control.
#[test]
fn b2p_serializer_refuses_an_unchained_meter() {
    use super::retained_wire::{ReceiptCheck as C, ReceiptFailure};
    for mode in MODES {
        hooks::break_next_meter_chain();
        let raw = w_cb3();
        let plain = plain(mode, &raw);
        let (capture, observer, ordinary) = observed(mode, &raw);
        let ((envelope, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
        assert!(hooks::armed_names().is_empty(), "{mode:?}");
        assert_eq!(retained.err(), Some(W1Fallback::Serializer(ReceiptFailure { check: C::WorkCounterInconsistent, field_path: "work.charged" })), "{mode:?}");
        assert!(captured.is_none(), "{mode:?}: nothing reaches precommit");
        assert_eq!(String::from_utf8(serde_json::to_vec(&envelope).unwrap()).unwrap(),
            String::from_utf8(with_notice(&plain, "case:a", Some("work_counter_inconsistent"))).unwrap(), "{mode:?}: T-12");
        // The control: the milestone (one Call).
        hooks::break_next_meter_chain();
        let milestone = self::raw();
        let (capture, observer, ordinary) = observed(mode, &milestone);
        let ((_, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
        assert!(hooks::armed_names().is_empty(), "{mode:?}");
        assert!(!matches!(retained, Err(W1Fallback::Serializer(_))) && captured.is_some(), "{mode:?}: the control reaches precommit");
    }
}

/// RV123 (B2-P round 2) N-3: W-CB3 with its terms reversed (`combination:ba`, B + A), as RV123
/// probed it: operand 0, the representative, is the `not_required` case, operand-prepared, so
/// the combination's freeze runs on a prepared slot.
fn rv123_w_cb3_ba() -> Value {
    let mut raw = w_cb3();
    raw["model"]["combinations"][0]["terms"] = json!([{"load_case": "case:b", "factor": 1.0}, {"load_case": "case:a", "factor": 1.0}]);
    raw["model"]["combinations"][0]["id"] = json!("combination:ba");
    raw
}
/// RV123 (B2-P round 2) N-3: c = 1 with two retained mechanics combinations, [2·case, −3·case],
/// as RV123 probed it: three Calls with the meter chained; −3·case refuses its own certificate,
/// a natural, hook-free `facade_certificate`.
fn rv123_c1_two_mechanics() -> Value {
    let mut raw = raw();
    raw["model"]["combinations"] = json!([
        {"id": "combination:2case", "label": "RV123 probe: 2 case", "basis": "mechanics", "terms": [{"load_case": "case", "factor": 2.0}], "provenance": B2P},
        {"id": "combination:m3case", "label": "RV123 probe: -3 case", "basis": "mechanics", "terms": [{"load_case": "case", "factor": -3.0}], "provenance": B2P}]);
    raw
}
/// RV123 (B2-P round 2) N-3: the two in-domain shapes, pinned in both modes through the
/// witnesses' checks (`b2p_pin_witnesses`: B2-C's producer requirements, the validated
/// successor at precommit) and on the Direct entry (`b2p_direct`), and:
/// - B + A: the CombinationSource's representative is the operand preparation's source (case
///   B's, the `not_required` operand 0);
/// - [2·case, −3·case]: three Calls, one per combination after the batch, and −3·case
///   `retained_unavailable` with `facade_certificate` after its selected Run.
#[test]
fn b2p_rv123_n3_shapes_are_pinned() {
    let shapes = || vec![("rv123_w_cb3_ba", rv123_w_cb3_ba()), ("rv123_c1_two_mechanics", rv123_c1_two_mechanics())];
    b2p_pin_witnesses(shapes());
    b2p_direct(shapes());
    for mode in MODES {
        let (_, s) = b2p_successor(&rv123_w_cb3_ba(), mode);
        let b = &s["retained_precision"]["body"];
        let c = &b["combinations"][0];
        let source = &b["sources"][c["source_ref"].as_u64().unwrap() as usize];
        let preparation = &b["operand_preparations"][0];
        assert_eq!((&preparation["owner_ref"], &preparation["result"]["kind"]), (&json!({"kind":"case","index":1}), &json!("prepared")), "{mode:?}");
        assert_eq!(source["representative_source_ref"], preparation["source_ref"], "{mode:?}: the prepared operand 0 represents");
        let (_, s) = b2p_successor(&rv123_c1_two_mechanics(), mode);
        let b = &s["retained_precision"]["body"];
        assert_eq!(b["calls"].as_array().unwrap().iter().map(|c| c["owner_refs"].clone()).collect::<Vec<_>>(),
            [json!([{"kind":"case","index":0}]), json!([{"kind":"combination","index":0}]), json!([{"kind":"combination","index":1}])], "{mode:?}");
        assert_eq!((&b["combinations"][1]["reason"]["code"], &b["combinations"][1]["reason"]["phase"]), (&json!("facade_certificate"), &json!("facade")), "{mode:?}");
        assert!(b["combinations"][1]["run"].is_object(), "{mode:?}: after its Run");
    }
}

/// RV123 (B2-P round 2) S-1: B2-C §2.4 (i), row 2 (and G5's rule keyed on it): a term whose batch
/// Run refused `ledger_unavailable` (`refuse_ledger_of_case`) makes its combination
/// `operand_source_unavailable` at that operand, with no rebuild, no operand preparation and no
/// Call. The case is `unavailable` with its CaseSource and its Run's refusal. (An unavailable
/// operand whose Run did not refuse its ledger is rebuilt instead: `b2p_hooks_and_failure_set`.)
#[test]
fn b2p_ledger_unavailable_operand_has_no_source() {
    for mode in MODES {
        hooks::refuse_ledger_of_case(1);
        let (_, s) = b2p_successor(&b2p_two_cases(), mode);
        assert!(hooks::armed_names().is_empty(), "{mode:?}");
        let b = &s["retained_precision"]["body"];
        let case = &b["cases"][1];
        assert_eq!((&case["status"], &case["source_ref"], &case["reason"]["code"]), (&json!("unavailable"), &json!(1), &json!("kernel_refused")), "{mode:?}");
        assert_eq!(case["run"]["kernel_terminal"], json!({"kind":"refused","reason":{"space":"refusal","tag":"ledger_unavailable",
            "error":{"tag":"accumulator","error":{"tag":"non_finite"}}}}), "{mode:?}");
        let c = &b["combinations"][0];
        assert_eq!((&c["disposition"], &c["reason"]), (&json!("retained_unavailable"), &json!({"code":"combination_unresolved","phase":"preparation",
            "cause":{"kind":"operand_source_unavailable","operand_index":1}})), "{mode:?}");
        assert_eq!((&c["call_ref"], &c["run"], &c["source_ref"], &c["product_attempt_ref"]), (&Value::Null, &Value::Null, &Value::Null, &Value::Null), "{mode:?}");
        assert_eq!((b["calls"].as_array().unwrap().len(), b["sources"].as_array().unwrap().len()), (1, 2), "{mode:?}: no Call, no rebuilt or prepared registration");
        assert!(b.get("operand_preparations").is_none(), "{mode:?}: no operand preparation");
    }
}

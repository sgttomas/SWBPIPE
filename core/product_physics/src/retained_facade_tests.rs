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
    let mut observer = rp::ProductCapture::prepared_probe();
    let mut ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
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
    // No W1 work ran: one case with a combination (also outside D1.4).
    let mut combined = raw.clone();
    combined["model"]["combinations"] = json!([{"id":"combo","basis":"mechanics","terms":[{"load_case":"case","factor":1.0}]}]);
    let (capture, observer, ordinary) = observed(mode, &combined);
    let combined_plain = serde_json::to_vec(&ordinary).unwrap();
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::Domain), "a combination: outside D1.4");
    check(&envelope, &combined_plain, "combination");
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
        "retained_wire::serialize_frozen(&frozen, &staged, capture)",
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
    assert!(run.contains("ProductCapture::permitted_probe(permit);"), "the permit moves into the observer");
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
    for (label, raw) in [("milestone", raw()), ("exportable", exportable)] {
        let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap().to_owned();
        let bases: Vec<Value> = raw["model"]["load_cases"].as_array().unwrap().iter().map(|c| json!({"ref_type":"load_case","ref_id":c["id"]})).collect();
        for mode in MODES {
            let base = serde_json::to_value(run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
            assert_eq!(base["producer"]["semantic_contract_id"], json!("openpipestress.result_semantics/0.3.0/preview-physics-1"), "{label}");
            assert!(sc::for_source(&base).is_ok(), "precondition: the base publication is admitted");
            let receipt = format!("{PLAIN} Reason: receipt_encoding; detail: work_counter_inconsistent.");
            for message in [PLAIN, receipt.as_str()] {
                let mut noticed = base.clone();
                noticed["diagnostics"].as_array_mut().unwrap().push(notice(&case, message, json!([case])));
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
        // G-A: outside D1 (D1.4: C + 1 load cases; a combination; D1.3: another namespace).
        let over = beyond_load_cases(&milestone, true);
        let mut combined = milestone.clone();
        combined["model"]["combinations"] = json!([{"id":"combo","basis":"mechanics","terms":[{"load_case":"case","factor":1.0}]}]);
        let mut namespace = milestone.clone();
        namespace["model"]["schema_version"] = json!("0.3.0");
        for (label, raw, clause) in [("C + 1 cases", &over, D1Clause::Invocation), ("combination", &combined, D1Clause::Invocation),
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
/// the other failure tags; no seed; quality entries and seeds out of request order, which the
/// case-id lookup reads correctly; and an entry or seed that is not unique.
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

/// B1 ST (T-4; RR "I81's B1-0 probe verified…", ruling 4): two-body case B is W2-published with
/// the published verdict `checks_passed` (PROBE §2.3), so it is `not_required` and A is empty:
/// `NoTriggeredCase`. On the private driver, in every build, `retained_w1` returns that cause with
/// the ordinary owner untouched and no W1 stage entered. On the actual Direct entry in the
/// registered build it publishes the exact ordinary bytes, with no notice and no W1 work, from one
/// ordinary run that reached G-C once; in any other build, the plain bytes from one run.
#[test]
fn b1_t4_two_body_case_b_is_a_no_triggered_case_pin() {
    let raw = u8_two_body_case_b();
    assert_eq!(sha(&serde_json::to_vec(&raw).unwrap()), TWO_BODY_B_INPUT_SHA256, "PROBE §2.3's two-body case B");
    for mode in MODES {
        let plain = plain(mode, &raw);
        // The private driver. The native-stage fault is a sentinel: it fires only if W1 reaches native.
        let (capture, observer, ordinary) = observed(mode, &raw);
        assert_eq!(serde_json::to_vec(&ordinary).unwrap(), plain, "{mode:?}: the observed run is the plain run");
        assert_eq!(ordinary.numerical_quality.cases.iter().map(|c| (c.basis_ref.ref_id.as_str(), c.solve_quality)).collect::<Vec<_>>(),
            [("case", NumericalQualityStatus::ChecksPassed)], "{mode:?}: the published verdict");
        assert!(matches!(observer.ordinary.as_slice(), [rp::OrdinarySeed { initial: Some(rp::InitialSeed::StructuralFailure { .. }), w2: rp::W2Seed::Published { .. }, .. }]),
            "{mode:?}: W2-published");
        hooks::withdraw_next_native_source();
        let (envelope, retained) = retained_w1(observer, ordinary, &capture);
        assert_eq!(retained.err(), Some(W1Fallback::NoTriggeredCase), "{mode:?}");
        assert_eq!(String::from_utf8(serde_json::to_vec(&envelope).unwrap()).unwrap(), String::from_utf8(plain.clone()).unwrap(),
            "{mode:?}: the exact ordinary bytes, no notice");
        assert_eq!(hooks::armed_names(), ["native"], "{mode:?}: no W1 stage ran");
        hooks::disarm();
        // The actual Direct entry.
        hooks::withdraw_next_native_source();
        let (output, counts) = direct(&raw, mode);
        if !registered() {
            assert!(output.retained().is_none(), "{mode:?}: no permit, no W1");
            assert_eq!(counts, ONE_RUN, "{mode:?}");
        } else {
            assert_eq!(output.admission().unwrap().law().refusal, None, "{mode:?}: admitted (inside D1)");
            assert_eq!(output.retained().and_then(|r| r.as_ref().err()), Some(&W1Fallback::NoTriggeredCase), "{mode:?}");
            assert!(output.successor().is_none(), "{mode:?}");
            assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{mode:?}: one ordinary run, then G-C once");
        }
        assert_eq!(hooks::armed_names(), ["native"], "{mode:?}: no W1 stage ran on the reserved-stack thread");
        hooks::disarm();
        let bytes = published(output);
        assert_eq!(notices(&bytes), 0, "{mode:?}: no notice");
        assert_eq!(String::from_utf8(bytes).unwrap(), String::from_utf8(plain).unwrap(), "{mode:?}: the exact ordinary bytes");
    }
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

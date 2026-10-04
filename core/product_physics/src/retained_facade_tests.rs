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
    // No W1 work ran: an invocation outside D1.4 (two load cases).
    let mut two = raw.clone();
    let mut second = two["model"]["load_cases"][0].clone();
    second["id"] = json!("case-2");
    two["model"]["load_cases"].as_array_mut().unwrap().push(second);
    let (capture, observer, ordinary) = observed(mode, &two);
    let two_plain = serde_json::to_vec(&ordinary).unwrap();
    let (envelope, retained) = retained_w1(observer, ordinary, &capture);
    assert_eq!(retained.err(), Some(W1Fallback::Domain), "outside D1.4");
    check(&envelope, &two_plain, "two cases");
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
/// control uses the milestone made out of D1 (a second load case, D1.4), which no build
/// admits.
#[test]
fn u3_no_permit_entries_are_the_ordinary_route() {
    for mode in MODES {
        let mut raw = raw();
        let case = raw["model"]["load_cases"][0].clone();
        raw["model"]["load_cases"].as_array_mut().unwrap().push(case);
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
        // G-A: outside D1 (D1.4: a second load case; a combination; D1.3: another namespace).
        let mut two = milestone.clone();
        let mut second = two["model"]["load_cases"][0].clone();
        second["id"] = json!("case-2");
        two["model"]["load_cases"].as_array_mut().unwrap().push(second);
        let mut combined = milestone.clone();
        combined["model"]["combinations"] = json!([{"id":"combo","basis":"mechanics","terms":[{"load_case":"case","factor":1.0}]}]);
        let mut namespace = milestone.clone();
        namespace["model"]["schema_version"] = json!("0.3.0");
        for (label, raw, clause) in [("two cases", &two, D1Clause::Invocation), ("combination", &combined, D1Clause::Invocation),
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
    let mut two = raw();
    let case = two["model"]["load_cases"][0].clone();
    two["model"]["load_cases"].as_array_mut().unwrap().push(case);
    for mode in MODES {
        let shared = hooks::counted(|| run_linear_static_preview_value_with_mode(two.clone(), mode).unwrap());
        assert_eq!(shared.1, ONE_RUN, "{mode:?}: the shared value route");
        let (output, counts) = direct(&two, mode);
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

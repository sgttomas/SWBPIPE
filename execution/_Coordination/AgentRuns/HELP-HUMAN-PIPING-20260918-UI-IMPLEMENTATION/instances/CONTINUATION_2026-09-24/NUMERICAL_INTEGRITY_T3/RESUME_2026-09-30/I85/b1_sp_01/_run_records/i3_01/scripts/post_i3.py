#!/usr/bin/env python3
"""I85 B1-SP: the post-I3 test edits (SR-RS's reader accepts W-C2's successor). Usage: post_i3.py <retained_facade_tests.rs> [pins.json]"""
import json, sys
p = sys.argv[1]
pins = json.load(open(sys.argv[2])) if len(sys.argv) > 2 else {"sparse_interactive": ["0"*64]*3, "dense_scrutiny": ["0"*64]*3}
s = open(p).read()
def rep(a, b):
    global s
    assert s.count(a) == 1, (a[:100], s.count(a))
    s = s.replace(a, b)
G5 = 'W1Fallback::Precommit { gate: "G5", code: "RETAINED_PRECISION_ATTEMPT_MISMATCH".into() }'
# ---- the section comment
rep("""// (`b1_sp_w_c2_through_retained_w1_publishes_t12`) and the Direct entry. Before SR-RS (I3) the accepted Rust
// reader refuses W-C2's successor at precommit, so its per-case outcomes are read from the
// successor that precommit received (RV107 A1-N-1: `hooks::counted_with_successor`).""",
"""// (`b1_sp_w_c2_through_retained_w1_publishes_t12`) and the Direct entry. Before SR-RS (I3) the accepted Rust
// reader refused W-C2's successor at precommit (G5, B's `not_required`), so its per-case outcomes
// were read from the successor that precommit received (RV107 A1-N-1:
// `hooks::counted_with_successor`). Since I3 the reader accepts it, and W-C2 publishes its
// successor, pinned with its fixtures (`b1_sp_w_c2_direct_entry_publishes_the_pinned_successor`).""")
# ---- the transaction test
rep("""/// - Before SR-RS the precommit refuses (`G5`, `RETAINED_PRECISION_ATTEMPT_MISMATCH`: B's
///   `not_required`), with A the selected attempt (bit 0), and the ordinary owner untouched.
/// - The successor precommit received: A `selected`, B `not_required` (no product attempt), C
///   `unavailable` (`kernel_unresolved`, its Run and source).""",
"""/// - Since I3 the precommit (SR-RS's reader, with the invocation) accepts the successor, which
///   the transfer moves out; the ordinary owner is untouched. (Before I3: `G5`,
///   `RETAINED_PRECISION_ATTEMPT_MISMATCH`, B's `not_required`, with A selected.)
/// - The successor: A `selected`, B `not_required` (no product attempt), C `unavailable`
///   (`kernel_unresolved`, its Run and source).""")
rep("""        assert_eq!(outcome.err(), Some((""" + G5 + """, 0b01)),
            "{label}: before SR-RS, B's not_required fails G5; A was selected");
        let successor = successor.unwrap_or_else(|| panic!("{label}: precommit received the successor"));""",
"""        let published = outcome.unwrap_or_else(|f| panic!("{label}: since I3 the precommit accepts W-C2: {f:?}"));
        let successor = successor.unwrap_or_else(|| panic!("{label}: precommit received the successor"));
        assert_eq!(published, successor, "{label}: the transfer moves the validated successor");
        let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
        assert!(open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)).is_ok(), "{label}: the Rust reader");""")
# ---- the faults test: C's injected fact is visible to SR-RS's G8.
rep("""/// - T-7 on C (`fail_preparation_of_case(2)`): A still selected, C `unavailable` at preparation,
///   with no `CaseSource`, Run, run or source reference, and one call over A alone;""",
"""/// - T-7 on C (`fail_preparation_of_case(2)`): A still selected, C `unavailable` at preparation,
///   with no `CaseSource`, Run, run or source reference, and one call over A alone. The fault zeroes
///   C's diameter fact, which C's attempt records truthfully, so since I3 the reader refuses the
///   successor at G8 (`PREPARATION_MISMATCH`: the recorded old fact is not the request's);""")
rep("""    assert_eq!(cause, (""" + G5 + """, 0b01), "T-7 on C");""",
"""    assert_eq!(cause, (W1Fallback::Precommit { gate: "G8", code: "RETAINED_PRECISION_PREPARATION_MISMATCH".into() }, 0b01), "T-7 on C");""")
# ---- through retained_w1
rep("""/// - before SR-RS, the precommit's refusal publishes the ordinary bytes, then the two N1 notices
///   (case-a's, then case-c's), both plain (T-12);""",
"""/// - since I3 the successor is published (A selected, B `not_required`, C unavailable), with the
///   ordinary owner untouched beside it (before I3: the precommit's refusal, then the two N1
///   notices, case-a's and case-c's, both plain);""")
rep("""        let (bytes, cause) = run();
        assert_eq!(cause, Some(""" + G5 + """), "{mode:?}: not NoTriggeredCase (R17)");
        assert_eq!(bytes, noticed(None), "{mode:?}: two plain notices");""",
"""        let (capture, observer, ordinary) = observed(mode, &raw);
        let (envelope, retained) = retained_w1(observer, ordinary, &capture);
        let successor = retained.unwrap_or_else(|f| panic!("{mode:?}: not NoTriggeredCase (R17): {f:?}"));
        assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "{mode:?}: the ordinary owner beside the successor");
        assert_eq!(w_c2_body(successor.value())["cases"].as_array().unwrap().iter().map(|c| c["status"].as_str().unwrap()).collect::<Vec<_>>(),
            ["selected", "not_required", "unavailable"], "{mode:?}");""")
# ---- Direct entry: T-13 and the pinned successor.
rep("""/// T-13 on the actual Direct entry, W-C2, both modes: one ordinary run, and G-C reached once
/// (`ONE_RUN_THROUGH_G_C`), with no hook armed. In the registered build (since I2, C = 3) the
/// transaction runs: before SR-RS the precommit refuses, and the publication is the ordinary
/// bytes then case-a's and case-c's plain notices. In any other build (Stale), exactly the plain
/// bytes.
#[test]
fn b1_sp_w_c2_direct_entry_counts_one_run_through_g_c() {""",
"""/// The pinned W-C2 successors (PLAN_v2 §2.2, after I3): (name, document sha256, receipt sha256,
/// published bytes' sha256).
const W_C2_PINNED: [(&str, &str, &str, &str); 2] = [
    ("sparse_interactive", "%s", "%s", "%s"),
    ("dense_scrutiny", "%s", "%s", "%s"),
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
    let out = std::env::var("I85_WC2_OUT").ok().map(std::path::PathBuf::from);""" % tuple(pins["sparse_interactive"] + pins["dense_scrutiny"]))
rep("""    let raw = w_c2();
    for mode in MODES {
        let plain = plain(mode, &raw);
        assert!(hooks::armed_names().is_empty(), "{mode:?}: no hook armed");
        let (output, counts) = direct(&raw, mode);""",
"""    let raw = w_c2();
    for (mode, (name, file_sha, receipt_sha, bytes_sha)) in MODES.into_iter().zip(W_C2_PINNED) {
        assert_eq!(mode.as_str(), name);
        let plain = plain(mode, &raw);
        assert!(hooks::armed_names().is_empty(), "{mode:?}: no hook armed");
        let (output, counts) = direct(&raw, mode);
        assert_eq!(serde_json::to_vec(output.envelope()).unwrap(), plain, "{mode:?}: the ordinary envelope is the plain run");""")
rep("""        assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{mode:?}: T-13");
        assert_eq!(output.retained().and_then(|r| r.as_ref().err()),
            Some(&""" + G5 + """), "{mode:?}: before SR-RS");
        let bytes = published(output);
        assert_eq!(notices(&bytes), 2, "{mode:?}: one notice per case in A");
        assert_eq!(String::from_utf8(bytes).unwrap(), String::from_utf8(with_notice(&with_notice(&plain, "case-a", None), "case-c", None)).unwrap(),
            "{mode:?}: the ordinary bytes, then the notices in request order");
        assert!(hooks::armed_names().is_empty(), "{mode:?}");""",
"""        assert_eq!(counts, ONE_RUN_THROUGH_G_C, "{mode:?}: T-13");
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
        assert!(hooks::armed_names().is_empty(), "{mode:?}");""")
# ---- R3P-1
rep("""/// - (A, B), A = {0}: A, the first case, is attempted, solved and frozen on its own source (the call
///   and the one source are A's); B is `not_required`; before SR-RS the precommit refuses (G5) with A
///   selected;""",
"""/// - (A, B), A = {0}: A, the first case, is attempted, solved and frozen on its own source (the call
///   and the one source are A's); B is `not_required`; since I3 the successor is published (before
///   I3 the precommit refused it at G5, with A selected);""")
rep("""            assert_eq!(outcome.err(), Some((""" + G5 + """, 0b1)), "{label}");
            let successor = successor.unwrap();""",
"""            let published = outcome.unwrap_or_else(|f| panic!("{label}: {f:?}"));
            let successor = successor.unwrap();
            assert_eq!(published, successor, "{label}");""")
rep("""            let (envelope, retained) = retained_w1(observer, ordinary, &capture);
            assert_eq!(retained.err(), Some(""" + G5 + """), "{mode:?}");
            assert_eq!(serde_json::to_vec(&envelope).unwrap(), with_notice(&plain, "case-a", None), "{mode:?}: A's one notice");""",
"""            let (envelope, retained) = retained_w1(observer, ordinary, &capture);
            assert!(retained.is_ok(), "{mode:?}: {:?}", retained.err());
            assert_eq!(serde_json::to_vec(&envelope).unwrap(), plain, "{mode:?}: the ordinary owner beside the successor");
            // T-12's one notice on a fallback: a serializer refusal after A's freeze.
            let (capture, observer, ordinary) = observed(mode, &raw);
            hooks::fail_next_serializer(super::retained_wire::ReceiptCheck::Association);
            let (envelope, _) = retained_w1(observer, ordinary, &capture);
            assert_eq!(serde_json::to_vec(&envelope).unwrap(), with_notice(&plain, "case-a", None), "{mode:?}: A's one notice");""")
s += """
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
"""
open(p, "w").write(s)
print("ok")

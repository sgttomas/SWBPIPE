//! Tests of the private retained-precision serializer (`retained_wire`): `serialize_cases` over
//! the W1 transaction (`transaction`, as `w1_transaction` runs it), and the serializer's typed
//! projections through the private one-case driver. The accepted Rust reader runs in-process
//! (dev-dependency); Python and TypeScript validate the emitted receipts in their own lanes
//! (I61_U1_OUT).
use super::retained_product as rp;
use super::retained_tests_hooks as hooks;
use super::retained_wire as wire;
use super::*;
use open_pipe_stress_frame_kernel::structural::retained_api as k;
use open_pipe_stress_result_export::retained_precision as reader;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const DEFINITION: &str = include_str!("../../../fixtures/results/retained_precision_prepared_ordinary_v1.json");
const TABLE: &str = include_str!("../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json");
/// The owner-adopted per-invocation default, 12 GiB: the registered profile's qualified M.
const RETAINED_LIMIT: RetainedByteLimit = RetainedByteLimit::new(12_884_901_888);
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
/// Protected ordinary bytes of the milestone (experiment 03 controls B and B'):
/// the plain route's serialized envelope, equal with and without capture. Pinned on
/// every target: the dense bytes used to differ across targets through libm `hypot`
/// (`rigid:N0`'s support force magnitude); since I109 (PR-N) magnitudes are correctly
/// rounded norms, and dense is re-pinned at the correctly rounded value.
/// K-D5 (DEL-04-01 kd5-member-relative-bound.md §§3.2, 3.4, 6) re-pinned both
/// modes: the milestone case stays Sensitive (K-D5's estimate), and against main
/// the bytes differ only by the two linear limitation sentences, the current
/// maximum declaration literals and the fallback maximum's enclosure. B7a's H-4
/// re-pinned them for the summary (the curved macro-element count's name, the
/// deleted retired thrust count) and the reworded pressure/imposed-displacement
/// limitation sentence; B4/D1 (b) re-pinned both modes: the preview route
/// attempts no source blocks, so the envelope loses only its
/// `diagnostic:source-recovery:case` disclosure; E14 re-pinned dense, which no
/// longer publishes the retired parity observation row.
/// C5 adds the adopted arc end-tangent limitation sentence; the current ordinary
/// companions preserve every physical row and are still equal under capture.
/// W-7 global end rows added (12 element_global_end rows on the one member); no other byte moved.
const ORDINARY_SHA256: [(&str, usize, &str); 2] = [
    ("sparse_interactive", 86484, "ac3bde77f91d0a9d595b08799cae5907c8fbef78c6c4c94761514ff6967b2e0d"),
    ("dense_scrutiny", 86510, "6da64fa02c206a88e670c75ff6b74364263c94e1ac8722a963f603ddf01ac7db"),
];
/// The serializer's committed milestone output: sha256 of the pretty-printed
/// successor envelope, and its receipt_sha256 (byte-stable, both modes).
/// K-D5 re-pin (DEL-04-01 kd5-member-relative-bound.md §§3.2, 3.4, 6): against main the milestone
/// successor differs only by the current maximum declaration and fallback enclosure, the two
/// linear limitation sentences, and the receipt and publication hashes that bind them.
/// Model 0.7.0 re-pin (W-13 S1b; DEL-02-01 Design `model-document-0.7.0.md` §2): every pinned
/// request and witness input was re-authored by its version token alone. The input sha256 pins, the
/// successors' invocation digests, the exact route's DEF-E-bound preparation and source identity
/// hashes, and the receipt, publication and document hashes binding them moved and follow the
/// maintained writers' bytes; no numeric leaf, class, selection, status or verdict moved.
/// W-7 re-pin (DEL-04-01 Design `global-end-actions.md`): the member's twelve not-covered global end
/// rows, the receipt figures counted over the rows, and the hashes binding them moved.
const SUCCESSOR_SHA256: [(&str, &str, &str); 2] = [
    ("sparse_interactive", "7fc66eeef48d03b64da78db705fe5b4d34f12e9b2b90ab0578eb5af9699d3c92", "c238347e1e0f32f1e484e84a0c0e4679be5c86546ab87af59da469cf560d6374"),
    ("dense_scrutiny", "ce0f6a6a80a327fbcdf594b166290cc2c86ca8e91e852ce3de84303f54713357", "51f2d8b75c12305ac184cb4378dc6d543389e958655465032cee3161a89eee49"),
];

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn milestone_raw() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
/// The private prepared driver on the unchanged 0.8.0 milestone: the candidate,
/// its captured invocation and the bytes of its single ordinary run.
fn milestone_candidate(mode: PreviewSolverMode) -> (rp::PrivatePreparedCandidate, source_receipt::CapturedInvocation, Vec<u8>) {
    let raw = milestone_raw();
    assert_eq!(raw["model"]["schema_version"], json!(crate::MODEL_SCHEMA_VERSION), "milestone request unchanged at 0.8.0");
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw, mode).unwrap();
    let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
    let ordinary = serde_json::to_vec(prepared.ordinary()).unwrap();
    prepared.solve_native().unwrap();
    let candidate = match prepared.project_candidate() { Ok(c) => c, Err(e) => panic!("{:?}", e.error) };
    (candidate, capture, ordinary)
}
fn invocation(mode: PreviewSolverMode) -> Value {
    json!({"request":milestone_raw(),"solver_mode":mode.as_str()})
}
fn pretty(v: &Value) -> String {
    serde_json::to_string_pretty(v).unwrap()
}
/// The milestone with a second, identical load case `case-2` (its loads renamed): both cases are
/// in A, and unfaulted both are selected.
fn milestone_two_cases() -> Value {
    let mut raw = milestone_raw();
    let mut second = raw["model"]["load_cases"][0].clone();
    second["id"] = json!("case-2");
    for load in second["primitive_loads"].as_array_mut().unwrap() {
        load["id"] = json!(format!("{}:2", load["id"].as_str().unwrap()));
    }
    raw["model"]["load_cases"].as_array_mut().unwrap().push(second);
    raw
}
/// The W1 transaction as `w1_transaction` runs it, through the call, with every requested case
/// in A: the observed ordinary run (the route and budget `permitted_run` decides), custody and one
/// product attempt per case (T-6, T-7), and the one call (T-8). The armed test hooks fire at their
/// own stages. Returns the transaction and the captured invocation.
fn called(raw: &Value, mode: PreviewSolverMode) -> (rp::PreparedCases, source_receipt::CapturedInvocation) {
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
    let route = w1_route(&request.model).unwrap_or_default();
    let mut observer = rp::ProductCapture::prepared_probe_on(route);
    let ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut w1_budget(route), Some(&mut observer));
    let requested = raw["model"]["load_cases"].as_array().unwrap().len();
    let attempted: Vec<usize> = (0..requested).collect();
    let mut prepared = match observer.prepare_cases(ordinary, requested, &attempted) { Ok(p) => p, Err(f) => panic!("custody: {:?}", f.error) };
    hooks::before_native(&mut prepared);
    prepared.native();
    (prepared, capture)
}
/// The rest of the transaction up to the serializer: the freezes (T-9), the combinations'
/// dispositions and custody, and staging (T-11). Returns the staged envelope.
fn staged(prepared: &mut rp::PreparedCases) -> MechanicsEnvelope {
    prepared.freeze();
    prepared.dispositions();
    prepared.combine().unwrap_or_else(|c| panic!("combination custody: {}", c.0));
    hooks::before_staging(prepared);
    prepared.staged_envelope().unwrap_or_else(|f| panic!("staging: {}", f.0))
}
/// The whole transaction up to `serialize_cases`: the transaction, its staged envelope and the
/// captured invocation.
fn transaction(raw: &Value, mode: PreviewSolverMode) -> (rp::PreparedCases, MechanicsEnvelope, source_receipt::CapturedInvocation) {
    let (mut prepared, capture) = called(raw, mode);
    let staged = staged(&mut prepared);
    (prepared, staged, capture)
}
/// The frozen case of request `index`.
fn frozen(prepared: &rp::PreparedCases, index: usize) -> &rp::FrozenCase {
    match &prepared.attempts.iter().find(|a| a.request == index).unwrap().end {
        rp::AttemptEnd::Frozen(frozen) => frozen,
        _ => panic!("case {index}: frozen expected"),
    }
}
/// The refused case of request `index` (refused after its selected Run).
fn refused_mut(prepared: &mut rp::PreparedCases, index: usize) -> &mut rp::RefusedCase {
    match &mut prepared.attempts.iter_mut().find(|a| a.request == index).unwrap().end {
        rp::AttemptEnd::Candidate(refused) => refused,
        _ => panic!("case {index}: refused candidate expected"),
    }
}
/// The two-case milestone with case-2's freeze refused after its selected Run at `fault` (a
/// facade fault after its proof started); case 0 is selected.
fn refused_second_case(mode: PreviewSolverMode, fault: super::retained_receipt::TraceFault)
    -> (rp::PreparedCases, MechanicsEnvelope, source_receipt::CapturedInvocation) {
    hooks::fault_case(1, fault);
    let outcome = transaction(&milestone_two_cases(), mode);
    assert!(hooks::armed_names().is_empty(), "the fault fired: {:?}", hooks::armed_names());
    outcome
}

/// Protected byte control 1 (experiment 03 A and B): capture never changes the
/// ordinary bytes, and the Direct entry's ordinary envelope is the plain route's. B'
/// (the envelope returned on the permit path) is U3 grant 2's committed test,
/// `retained_facade_tests::u3g2_direct_entry_publishes_the_pinned_successor` (RV82 N3);
/// in the registered build the Direct entry below is that permitted path.
#[test]
fn u1_ordinary_bytes_unchanged_under_capture() {
    for (mode, (name, len, digest)) in MODES.into_iter().zip(ORDINARY_SHA256) {
        assert_eq!(mode.as_str(), name);
        let plain = serde_json::to_vec(&run_linear_static_preview_value_with_mode(milestone_raw(), mode).unwrap()).unwrap();
        assert_eq!((plain.len(), sha(&plain).as_str()), (len, digest), "{name}: protected ordinary bytes");
        let direct = run_linear_static_preview_value_with_retained_direct(milestone_raw(), mode, RETAINED_LIMIT).unwrap();
        assert_eq!(serde_json::to_vec(direct.envelope()).unwrap(), plain, "{name}: A, the Direct entry's ordinary envelope");
        let (candidate, _, captured) = milestone_candidate(mode);
        assert_eq!(captured, plain, "{name}: B, the ordinary run with capture installed");
        drop(candidate);
    }
}

/// G-b and G-l: the typed seed of the milestone's single ordinary attempt.
#[test]
fn u1_typed_ordinary_capture_milestone() {
    for mode in MODES {
        let (candidate, _, ordinary) = milestone_candidate(mode);
        let env: Value = serde_json::from_slice(&ordinary).unwrap();
        let [seed] = &candidate.capture().ordinary[..] else { panic!("one seed") };
        assert_eq!(seed.case, "case");
        let integrity = "diagnostic:numerical-integrity:case";
        match &seed.initial {
            Some(rp::InitialSeed::Report { code, report_diagnostic_ref }) => {
                assert_eq!((code.as_str(), report_diagnostic_ref.as_str()), ("NUMERICAL_INTEGRITY_SENSITIVE", integrity));
            }
            other => panic!("initial report expected: {other:?}"),
        }
        assert!(matches!(seed.w2, rp::W2Seed::NotTriggered));
        // K-D5's record is one evidence line of the integrity diagnostic (F1a D5C-3).
        assert_eq!(seed.d5_diagnostic_ref.as_deref(), Some(integrity));
        let record = env["diagnostics"].as_array().unwrap().iter().find(|d| d["id"] == integrity).unwrap();
        assert!(record["message"].as_str().unwrap().contains("formation_check: reason="), "test oracle only: the D5 line is present");
        assert!(!seed.recovery_demoted);
        println!("I61_U1_SEED {} finding={:?}", mode.as_str(), seed.load_row_finding);
    }
}

/// The deliverable: the serializer's milestone receipts (c = 1), both modes, validated
/// in-process by the accepted Rust reader, eligible with the invocation (U7), with exact
/// class parity against the certificate's own verdicts, byte-stable.
#[test]
fn u1_milestone_successor_both_modes() {
    let out = std::env::var("I61_U1_OUT").ok().map(std::path::PathBuf::from);
    for (mode, (name, file_sha, receipt_sha)) in MODES.into_iter().zip(SUCCESSOR_SHA256) {
        assert_eq!(mode.as_str(), name);
        let (mut prepared, staged, capture) = transaction(&milestone_raw(), mode);
        let successor = wire::serialize_cases(&mut prepared, &staged, &capture).unwrap_or_else(|f| panic!("{name}: {f:?}"));
        let again = wire::serialize_cases(&mut prepared, &staged, &capture).unwrap();
        assert_eq!(pretty(&successor), pretty(&again), "{name}: deterministic");
        let invocation = invocation(mode);
        let validation = reader::validate(&successor, Some(&invocation)).unwrap_or_else(|e| panic!("{name}: reader {e:?}"));
        assert!(validation.invocation_bound && validation.numerical_eligible, "{name}: eligible with the actual invocation (U7)");
        // Parity: every reader class equals the certificate's verdict class.
        let rows: Vec<&str> = successor["results"].as_array().unwrap().iter().map(|r| r["id"].as_str().unwrap()).collect();
        let verdicts = frozen(&prepared, 0).certificate().verdicts();
        assert_eq!(validation.classifications.len(), verdicts.len(), "{name}: one class per row");
        for (c, v) in validation.classifications.iter().zip(verdicts) {
            assert_eq!(c.result_id, rows[v.row]);
            let derived = rp::derived_kind(successor["results"][v.row]["kind"].as_str().unwrap());
            if derived {
                // T4-U4b §4.1: a recipe-free not-covered verdict carries the published value as
                // published (final_case.rs records `r.value`), while the reader normalizes a
                // stress row to SI; the reader's value is the published one, normalized.
                let published = successor["results"][v.row]["value"].as_f64().unwrap();
                let si = match successor["results"][v.row]["unit"].as_str().unwrap() { "MPa" => published * 1_000_000.0, _ => published };
                assert_eq!(v.normalized_bits, published.to_bits(), "{name}: {}", c.result_id);
                assert_eq!(c.normalized_bits, si.to_bits(), "{name}: {} (normalized to SI)", c.result_id);
            } else {
                assert_eq!(c.normalized_bits, v.normalized_bits, "{name}: {}", c.result_id);
            }
            let expected = match v.class {
                Some(k::RowClass::RelativeVerified) => reader::AccuracyClass::RelativeVerified,
                Some(k::RowClass::AbsoluteVerified { bound_bits }) => reader::AccuracyClass::AbsoluteVerified { bound_bits },
                Some(k::RowClass::InputDerived) => reader::AccuracyClass::InputDerived,
                Some(k::RowClass::Unpublishable) => panic!("unpublishable"),
                // T4-U4b §4.1: a class-None verdict is a record's (non_quantity) or a
                // not-covered derived row's (its recipe is NotCovered).
                // W-7: a global end row is a named not-covered row too (GLOBAL_END_ROWS_NOT_COVERED).
                None if rp::derived_kind(successor["results"][v.row]["kind"].as_str().unwrap())
                    || crate::is_global_end_kind(successor["results"][v.row]["kind"].as_str().unwrap()) => reader::AccuracyClass::NotCovered,
                None => reader::AccuracyClass::NonQuantity,
            };
            assert_eq!(c.class, expected, "{name}: {}", c.result_id);
            assert!(v.passed);
        }
        let text = pretty(&json!({"id":format!("u1_milestone_{name}"),"source":successor,"invocation":invocation}));
        let receipt = successor["retained_precision"]["receipt_sha256"].as_str().unwrap();
        println!("I61_U1_SUCCESSOR {name} file_sha256={} receipt_sha256={receipt} rows={} classes={}", sha(text.as_bytes()), rows.len(), validation.classifications.len());
        if let Some(dir) = &out {
            std::fs::write(dir.join(format!("u1_milestone_{name}.json")), &text).unwrap();
        }
        assert_eq!((sha(text.as_bytes()).as_str(), receipt), (file_sha, receipt_sha), "{name}: committed successor bytes");
    }
}

/// U2 (RV77-N4): a certified proof of another selected owner with identical
/// public facts is refused structurally. This is the deliberate flip of the
/// custody limit `I61_FOREIGN_OWNER` documented at the C3 seam.
#[test]
fn u2_foreign_owner_refused() {
    let mode = PreviewSolverMode::SparseInteractive;
    let (mut first, first_staged, capture) = transaction(&milestone_raw(), mode);
    let (mut second, second_staged, _) = transaction(&milestone_raw(), mode);
    assert!(wire::serialize_cases(&mut first, &first_staged, &capture).is_ok(), "own proof serializes");
    let (rp::AttemptEnd::Frozen(a), rp::AttemptEnd::Frozen(b)) = (&mut first.attempts[0].end, &mut second.attempts[0].end) else { panic!("selected") };
    a.test_swap_certificate(b);
    let refused = wire::serialize_cases(&mut first, &first_staged, &capture);
    println!("I61_FOREIGN_OWNER serializer_refused={}", refused.is_err());
    assert_eq!(refused, Err(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "cases[].selection.owner" }));
    assert_eq!(wire::serialize_cases(&mut second, &second_staged, &capture).err().map(|f| f.field_path), Some("cases[].selection.owner"));
}

/// ROOT correction (I65 D-4 §3): every record amount is read through its checked
/// accessor. A record whose `work_status` latch holds a fault (here a stage total
/// overflow) refuses on every latched view while its legacy fields look plausible.
#[test]
fn u1_checked_record_work_honours_the_latch() {
    let (candidate, _, _) = milestone_candidate(PreviewSolverMode::SparseInteractive);
    let k::ExecutionOutcome::Selected(owner) = &candidate.capture().native_pair().unwrap().1.outcome else { panic!("selected") };
    for record in &owner.evidence().attempts {
        let (values, failures) = wire::test_record_work(record);
        assert!(failures.is_empty(), "exact milestone record: {failures:?}");
        let exact = |w: k::WorkTotal| w.exact().unwrap();
        assert_eq!(values, vec![exact(record.work.checked_lme()), exact(record.k4_work.checked_lme()), exact(record.checked_own_work()),
            exact(record.checked_shared_work()), exact(record.checked_stop_rule_work()), exact(record.checked_verification_work()),
            exact(record.checked_verification_shared_work()), exact(record.checked_case_charge()), exact(record.checked_invocation_increment())]);
        let mut latched = record.clone();
        latched.stages.formation = u64::MAX;
        latched.stages.assembly = 1;
        assert_eq!(latched.work_status().fault(), Some(k::WorkFault::Overflow), "precondition: the record latch holds a fault");
        assert!(latched.shared_work < u64::MAX && latched.stop_rule_work < u64::MAX, "precondition: the legacy fields look plausible");
        let (_, failures) = wire::test_record_work(&latched);
        let mut paths: Vec<&str> = failures.iter().map(|f| {
            assert_eq!(f.check, wire::ReceiptCheck::WorkCounter(k::WorkFault::Overflow));
            f.field_path
        }).collect();
        paths.sort_unstable();
        assert_eq!(paths, vec!["run.attempts[].case_charge", "run.attempts[].invocation_increment", "run.records[].work.own_lme",
            "run.records[].work.shared_lme", "run.records[].work.stop_rule_lme", "run.records[].work.verification_lme",
            "run.records[].work.verification_shared_lme"], "every latched view refuses");
    }
}

/// ROOT correction: the serializer's production code names no legacy saturating
/// work field or accessor (the invocation meter's latch is not constructible from
/// PP, so this source guard covers `charged()` alongside the record fields).
#[test]
fn u1_serializer_reads_no_legacy_work_field() {
    let source = include_str!("retained_wire.rs");
    let production = source.split("\n#[cfg(test)]").next().unwrap();
    for legacy in [".shared_work", ".stop_rule_work", ".verification_work", ".verification_shared_work", ".charged()", "legacy_saturated", ".total()"] {
        assert!(!production.contains(legacy), "legacy work read {legacy}");
    }
}

/// ROOT correction: the per-stage slots are emitted only under an exact status.
#[test]
fn u1_stage_slots_require_an_exact_status() {
    let mut exact = k::StageWork::default();
    exact.formation = 5;
    exact.uc = 7;
    let (slots, failures) = wire::test_stages(&exact);
    assert!(failures.is_empty());
    assert_eq!((slots["formation"].as_u64(), slots["uc"].as_u64(), slots.as_object().map(|o| o.len())), (Some(5), Some(7), Some(19)));
    let mut overflow = k::StageWork::default();
    overflow.formation = u64::MAX;
    overflow.assembly = 1;
    let (slots, failures) = wire::test_stages(&overflow);
    assert_eq!(slots, Value::Null, "no slot is emitted from a faulted StageWork");
    assert_eq!(failures, vec![wire::ReceiptFailure { check: wire::ReceiptCheck::WorkCounter(k::WorkFault::Overflow), field_path: "stages" }]);
}

/// D-4 §3 item 5: the producer checks C1 §1 conservation before emission
/// (record: own = W+K, own stages = O, shared stages = S+V, D+Q <= O; run:
/// the logical partition and after = before + increment).
#[test]
fn u1_conservation_checked_before_emission() {
    let (mut prepared, staged, capture) = transaction(&milestone_raw(), PreviewSolverMode::SparseInteractive);
    let successor = wire::serialize_cases(&mut prepared, &staged, &capture).unwrap();
    let (inv, case) = prepared.capture.native_pair().unwrap();
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else { panic!("selected") };
    let run = &inv.runs()[case.run];
    let records = &owner.evidence().attempts;
    let inconsistent = |path: &'static str| vec![wire::ReceiptFailure { check: wire::ReceiptCheck::WorkCounterInconsistent, field_path: path }];
    for (i, record) in records.iter().enumerate() {
        assert!(wire::test_physical(record, run.records[i]).is_empty(), "exact milestone record {i}");
        let mut own_stages = record.clone();
        own_stages.stages.formation += 1;
        let failures = wire::test_physical(&own_stages, run.records[i]);
        assert!(failures.contains(&inconsistent("run.records[].work.own_stages")[0]), "{failures:?}");
        let mut shared = record.clone();
        shared.shared_work += 1;
        let failures = wire::test_physical(&shared, run.records[i]);
        assert!(failures.contains(&inconsistent("run.records[].work.shared_stages")[0]), "{failures:?}");
        let mut stop = record.clone();
        stop.stop_rule_work = stop.checked_own_work().exact().unwrap() + 1;
        let failures = wire::test_physical(&stop, run.records[i]);
        assert!(failures.contains(&inconsistent("run.records[].work.stop_rule_lme")[0]), "{failures:?}");
    }
    let attempts = successor["retained_precision"]["body"]["cases"][0]["run"]["attempts"].as_array().unwrap().clone();
    assert!(wire::test_run_conservation(&attempts, run).is_empty());
    let mut broken = attempts.clone();
    broken[0]["case_charge"] = json!(broken[0]["case_charge"].as_u64().unwrap() + 1);
    assert_eq!(wire::test_run_conservation(&broken, run), inconsistent("cases[].run.case_charge"));
    let mut broken = attempts;
    broken[0]["invocation_increment"] = json!(broken[0]["invocation_increment"].as_u64().unwrap() + 1);
    assert_eq!(wire::test_run_conservation(&broken, run), inconsistent("cases[].run.invocation_increment"));
}

/// D-4b (ROOT, NUM fe38ea55bc): the typed cause maps totally onto the accepted
/// `receipt_failure.check` enum, C1:68's tokens only.
#[test]
fn u1_receipt_check_wire_vocabulary() {
    use wire::ReceiptCheck as C;
    let table = [
        (C::Encoding, "encoding"),
        (C::PublicationHashRange, "publication_hash_range"),
        (C::WorkCounter(k::WorkFault::Overflow), "work_counter_range"),
        (C::WorkCounter(k::WorkFault::Inconsistent), "work_counter_inconsistent"),
        (C::WorkCounter(k::WorkFault::Both), "work_counter_inconsistent"),
        (C::WorkCounterRange, "work_counter_range"),
        (C::WorkCounterInconsistent, "work_counter_inconsistent"),
        (C::Association, "association"),
        (C::Scope, "association"),
        (C::Untranslated, "encoding"),
    ];
    let schema: Value = serde_json::from_str(include_str!("../../../schemas/retained_precision_mp_v2.schema.json")).unwrap();
    let accepted = schema.to_string();
    for (check, token) in table {
        assert_eq!(check.wire(), token, "{check:?}");
        assert!(accepted.contains(&format!("\"{token}\"")), "{token} is an accepted check");
        assert!(!["work_counter_overflow", "work_counter_unknown"].contains(&check.wire()));
    }
}

/// D-4 §3 item 2: a trace Count is exact or abandons.
#[test]
fn u1_exact_or_abandon_counts() {
    let mut overflow = k::StageWork::default();
    overflow.formation = u64::MAX;
    overflow.assembly = 1;
    let (count, failures) = wire::test_count(overflow.checked_total());
    assert_eq!(count["kind"], json!("unavailable"));
    assert_eq!(failures, vec![wire::ReceiptFailure { check: wire::ReceiptCheck::WorkCounter(k::WorkFault::Overflow), field_path: "Count.value" }]);
    let (count, failures) = wire::test_count(k::StageWork::default().checked_total());
    assert_eq!((count, failures.len()), (json!({"kind":"exact","value":0}), 0));
}

/// U6-DH, T4-S6 (DEL-05-03 Design §9): `domain_hash` streams the checked canonical text of the
/// borrowed `{domain, payload}` into the hasher. Its hash, and its refusal (None), are the former
/// path's — `json!({"domain": d, "payload": p})`, serialized, checked-parsed, canonicalized, hashed —
/// over the in-tree definition and table, every hashed domain name and adversarial payloads (key
/// order, escapes, non-ASCII, signed zero, extreme and unsafe numbers, nesting at and past the
/// parser's depth, a payload naming `domain`). The former path lives only here.
#[test]
fn u6_dh_borrowed_wrapper_text_and_hash_are_the_former_objects() {
    use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
    let definition: Value = serde_json::from_str(DEFINITION).unwrap();
    let table: Value = serde_json::from_str(TABLE).unwrap();
    let adversarial = json!({"z": [1, -0.0, 1e-300, 1.7976931348623157e308, 18446744073709551615u64, -9223372036854775808i64],
        "a": {"payload": null, "domain": "inner", "": [], "\u{1F600}\"\\\n": {}}, "m": [true, false, null, "\u{0}\u{1f}\u{7f}é"]});
    let nest = |depth: usize| (0..depth).fold(json!("x"), |inner, _| json!([inner]));
    let safe = json!({"z": [1, -0.0, 1e-300, 9007199254740991u64, -9007199254740991i64, 5e-324], "a": {"\u{1F600}": "\u{0}"}});
    let mut payloads = vec![definition, table, adversarial, safe, json!(null), json!([]), json!({}), json!("text"), json!(0.1)];
    payloads.extend([125, 126, 127, 128].map(nest));
    let domains = ["retained_precision_formation_v1", "retained_precision_publication_mp_v2", "retained_precision_receipt_mp_v2",
        "retained_precision_preparation_v1", "retained_precision_operand_preparation_v1", "retained_precision_source_mp_v2", "", "a\"b\\c\u{1F600}"];
    let mut refused = 0;
    for domain in domains {
        for payload in &payloads {
            let former = serde_json::to_string(&json!({"domain": domain, "payload": payload.clone()})).unwrap();
            let expected = canonical_json_checked_v1_text(&former).ok().map(|c| format!("{:x}", Sha256::digest(c.as_bytes())));
            refused += usize::from(expected.is_none());
            assert_eq!(wire::domain_hash(domain, payload), expected, "{domain}: hash");
        }
    }
    // The adversarial payload's unsafe integers and the two deepest nestings refuse under every domain.
    assert_eq!(refused, 3 * domains.len());
    assert_eq!(wire::domain_hash("retained_precision_formation_v1", &payloads[0]).as_deref(), Some(wire::DEFINITION_SHA256));
}

/// L3: exercise both moved Values through the real entry serializer and
/// compare compact/pretty text and receipt-domain hashes with the former json! copies.
#[test]
fn ordinary_formation_value_move_is_byte_identical() {
    for mode in MODES {
        for raw in [milestone_raw(), crate::f1a_tests::udl_w1e8_with_soft_branch()] {
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw, mode).unwrap();
            let mut observer = rp::ProductCapture::prepared_probe();
            let observed = run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
            let env = serde_json::to_value(&observed).unwrap();
            let [captured] = &observer.ordinary[..] else { panic!("one ordinary seed") };
            let mut adversarial = captured.clone();
            adversarial.load_row_finding = Some(rp::FindingSeed {
                sentence: "quote \" slash \\ newline\n é 😀".into(),
                fired: vec!["z".into(), "a".into(), "\u{0}".into()], diagnostic_ref: None,
            });
            for seed in [captured, &adversarial] {
                let actual = wire::test_ordinary_value(&env, &seed.case, seed).unwrap();
                let ((initial, w2, _), failures) = wire::test_ordinary_members(seed);
                assert!(failures.is_empty(), "{failures:?}");
                let finding = seed.load_row_finding.as_ref().map(|f| json!({
                    "guard":"load_row","sentence":f.sentence,"fired":f.fired,"diagnostic_ref":f.diagnostic_ref}));
                let formation = json!({"load_row_finding":finding,"d5_diagnostic_ref":seed.d5_diagnostic_ref});
                let former = json!({"case_index":0,"case_id":seed.case,"material_basis_ref":0,"requested_mode":"sparse_interactive",
                    "initial":initial,"w2":w2,"formation":formation,"diagnostic_refs":actual["diagnostic_refs"]});
                assert_eq!(serde_json::to_vec(&actual).unwrap(), serde_json::to_vec(&former).unwrap());
                assert_eq!(pretty(&actual), pretty(&former));
                assert_eq!(wire::domain_hash("retained_precision_receipt_mp_v2", &actual), wire::domain_hash("retained_precision_receipt_mp_v2", &former));
            }
        }
    }
}

/// The in-tree fixtures bind the serializer's constants (A4 closed by the fan-in).
#[test]
fn u1_constants_bound_to_in_tree_fixtures() {
    let definition: Value = serde_json::from_str(DEFINITION).unwrap();
    let table: Value = serde_json::from_str(TABLE).unwrap();
    assert_eq!(wire::domain_hash("retained_precision_formation_v1", &definition).as_deref(), Some(wire::DEFINITION_SHA256));
    assert_eq!(table["product_formation_definitions"], json!([{"id":wire::DEFINITION_ID,"sha256":wire::DEFINITION_SHA256},{"id":"RP-PREPARED-COMBINATION-DUAL-v1","sha256":"a9219849d4dfecfc4e9529c6b648eb21cdf2e9872f2d681cc0c26cdfd2bdf6f1"}]));
    assert_eq!(table["semantic_contract_id"], json!(wire::RETAINED_SEMANTIC_ID));
    assert_eq!(table["formulation_profile_id"], json!(wire::RETAINED_PROFILE_ID));
    assert_eq!(table["receipt_policy"], json!(wire::POLICY));
    assert_eq!(table["accuracy_classification"]["policy"], json!(wire::FACADE_POLICY));
}

/// Scope (G-j) and untranslated variants refuse typed instead of guessing.
#[test]
fn u1_refusals_are_typed() {
    let mode = PreviewSolverMode::SparseInteractive;
    // Another invocation's custody for the same transaction: the mode differs.
    let (mut prepared, staged, _) = transaction(&milestone_raw(), mode);
    let (_, other) = source_receipt::CapturedInvocation::parse(milestone_raw(), PreviewSolverMode::DenseScrutiny).unwrap();
    assert_eq!(wire::serialize_cases(&mut prepared, &staged, &other).err().map(|f| f.field_path), Some("invocation"));
    // An invocation with a second requested case is not the one this transaction observed
    // (RV82-S2), so it is refused as foreign before its cases are scoped.
    let (_, two) = source_receipt::CapturedInvocation::parse(milestone_two_cases(), mode).unwrap();
    assert_eq!(wire::serialize_cases(&mut prepared, &staged, &two).err(), Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "invocation" }));
}

/// G-b on an actual load-row case (captured entry): the finding demotes a
/// Passed report, so its sentence is disclosed in the integrity diagnostic. Capture leaves the ordinary bytes
/// unchanged here too. S11-G T1's RF-CANCEL-UDL-W1e8 is now Sensitive at the solve
/// under K-D5's closure (DEL-04-01 kd5-member-relative-bound.md §§1, 4.2), so the
/// Passed load-row case is the same request with a soft branch
/// (`f1a_tests::udl_w1e8_with_soft_branch`).
#[test]
fn u1_load_row_case_capture() {
    let raw = crate::f1a_tests::udl_w1e8_with_soft_branch();
    for mode in MODES {
        let plain = serde_json::to_vec(&run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
        let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let mut observer = rp::ProductCapture::prepared_probe();
        let observed = run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
        assert_eq!(serde_json::to_vec(&observed).unwrap(), plain, "{mode:?}: capture leaves the ordinary bytes unchanged");
        let [seed] = &observer.ordinary[..] else { panic!("one seed") };
        let integrity = format!("diagnostic:numerical-integrity:{}", seed.case);
        match &seed.initial {
            Some(rp::InitialSeed::Report { code, report_diagnostic_ref }) => {
                assert_eq!((code.as_str(), report_diagnostic_ref), ("NUMERICAL_INTEGRITY_SENSITIVE", &integrity), "{mode:?}: demoted report");
            }
            other => panic!("{mode:?}: report expected: {other:?}"),
        }
        let finding = seed.load_row_finding.as_ref().expect("load-row finding");
        assert_eq!(finding.diagnostic_ref.as_deref(), Some(integrity.as_str()), "{mode:?}: disclosed by demotion");
        assert!(!finding.sentence.is_empty() && finding.fired.iter().any(|f| f.contains("S1:RZ")), "{mode:?}: {finding:?}");
        // RV82-S1: F1 in the absent direction; this case's report carries no K-D5 line.
        assert_eq!(seed.d5_diagnostic_ref, None, "{mode:?}: no K-D5 line, so no d5 reference");
        let record = observed.diagnostics.iter().find(|d| d.id == integrity).unwrap();
        assert!(record.message.ends_with(&finding.sentence) || record.message.contains(&finding.sentence), "test oracle only");
        println!("I61_U1_LOAD_ROW {} initial=report(sensitive) finding_ref={:?} d5={:?}", mode.as_str(), finding.diagnostic_ref, seed.d5_diagnostic_ref);
    }
}

/// G-i (U1 grant 2): every constructible variant of each closed translation table,
/// against the pinned golden corpus (each value is also validated against its
/// schema `$def` in the Python lane). Variants without a wire form record their
/// typed failure instead of a value.
#[test]
fn u1g2_translation_corpus() {
    let entries = wire::corpus::build();
    let corpus: Vec<Value> = entries.iter().map(|x| json!({"def":x.def,"label":x.label,"value":x.value,
        "failures":x.failures.iter().map(|f| json!({"check":f.check.wire(),"path":f.field_path,"typed":format!("{:?}", f.check)})).collect::<Vec<_>>()})).collect();
    let text = serde_json::to_string_pretty(&corpus).unwrap();
    if let Some(dir) = std::env::var("I61_U1_OUT").ok().map(std::path::PathBuf::from) {
        std::fs::write(dir.join("translation_corpus.json"), &text).unwrap();
    }
    println!("I61_U1G2_CORPUS entries={} sha256={}", corpus.len(), sha(text.as_bytes()));
    assert_eq!(sha(text.as_bytes()), GOLDEN_CORPUS_SHA256, "pinned translation corpus");
}
const GOLDEN_CORPUS_SHA256: &str = "4489a9df4ba668811ce5948b307b55432bc7ba95d801d103c47351a497b4c6f6";

fn write_receipt(name: &str, successor: &Value, mode: PreviewSolverMode) {
    if let Some(dir) = std::env::var("I61_U1_OUT").ok().map(std::path::PathBuf::from) {
        std::fs::write(dir.join(format!("{name}.json")), pretty(&json!({"id":name,"source":successor,"invocation":invocation(mode)}))).unwrap();
    }
}
fn body(successor: &Value) -> &Value {
    &successor["retained_precision"]["body"]
}

/// The C1-C3 unavailable representation of an actual preparation refusal (the closed annulus
/// helper refuses a zero diameter: `fail_preparation_of_case`) beside a selected case: case-2
/// has no CaseSource, Run or execution entry.
#[test]
fn u1g2_preparation_refusal_representation() {
    let mode = PreviewSolverMode::SparseInteractive;
    hooks::fail_preparation_of_case(1);
    let (mut prepared, staged, capture) = transaction(&milestone_two_cases(), mode);
    assert!(hooks::armed_names().is_empty(), "the fault fired");
    let successor = wire::serialize_cases(&mut prepared, &staged, &capture).unwrap_or_else(|f| panic!("{f:?}"));
    let b = body(&successor);
    let case = &b["cases"][1];
    assert_eq!(b["cases"][0]["status"], json!("selected"));
    assert_eq!((case["status"].as_str(), &case["reason"]), (Some("unavailable"),
        &json!({"code":"source_unavailable","phase":"preparation","cause":{"kind":"prepared_product_failure","product_attempt_ref":1}})));
    assert_eq!((&case["run"], &case["source_ref"]), (&Value::Null, &Value::Null));
    assert_eq!((b["sources"].as_array().map(Vec::len), &b["work"]["execution_order"], &b["calls"][0]["owner_refs"]),
        (Some(1), &json!([{"kind":"case","index":0}]), &json!([{"kind":"case","index":0}])), "no CaseSource, Run or call entry for case-2");
    let attempt = &b["product_attempts"][1];
    assert_eq!((attempt["result"]["kind"].as_str(), attempt["result"]["error"]["kind"].as_str(), &attempt["run_ref"], &attempt["source_ref"]),
        (Some("unavailable"), Some("preparation"), &Value::Null, &Value::Null));
    // Unselected: no method token. B4/D1 (b): the preview route attempts no source
    // blocks, so the attempt carries no legacy record and no source-recovery disclosure.
    assert!(b["ordinary_attempts"][1].get("legacy_source").is_none() && b.get("legacy_source_work").is_none());
    let diagnostics = successor["diagnostics"].as_array().unwrap();
    assert!(!diagnostics.iter().any(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"));
    let case_rows = || successor["results"].as_array().unwrap().iter().filter(|r| r["basis_ref"]["ref_id"] == "case-2");
    assert!(case_rows().count() > 0 && case_rows().all(|r| r.get("recovery_method").is_none()));
    let unavailable: Vec<&Value> = diagnostics.iter().filter(|d| d["code"] == wire::UNAVAILABLE_CODE).collect();
    assert_eq!(unavailable.len(), 1);
    assert_eq!((unavailable[0]["id"].as_str(), &case["diagnostic_ref"]), (Some("diagnostic:retained-precision:case-2:unavailable"), &unavailable[0]["id"]));
    write_receipt("u1g2_preparation_refusal_sparse_interactive", &successor, mode);
}

/// D38 on the W1 transaction: a prepared case whose native source is withdrawn fails the call
/// itself before any Run, so every prepared attempt ends `Native` with that `capture` cause, the
/// native stage entered and failed. No case is selected, so `serialize_cases` refuses (`Scope`,
/// `cases[].status`): no successor carries a prepared attempt without its Run.
#[test]
fn u1g2_d38_call_failure_selects_no_case() {
    for mode in MODES {
        hooks::withdraw_next_native_source();
        let (mut prepared, staged, capture) = transaction(&milestone_raw(), mode);
        assert!(hooks::armed_names().is_empty(), "the fault fired");
        assert!(prepared.capture.native_pair().is_none(), "precondition: no kernel schedule ran");
        let attempt = &prepared.attempts[0];
        assert!(matches!(attempt.end, rp::AttemptEnd::Native), "{mode:?}");
        assert!(matches!(&attempt.trace.native_error, Some(rp::CaptureError::Association(d)) if d == "prepared source"), "{:?}", attempt.trace.native_error);
        assert_eq!(attempt.trace.stages[super::retained_receipt::Stage::Native as usize], super::retained_receipt::StageState::Failed, "the native stage was entered");
        assert_eq!(wire::serialize_cases(&mut prepared, &staged, &capture).err(),
            Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Scope, field_path: "cases[].status" }));
    }
}

/// A candidate refused after its selected native Run (an injected facade fault), beside a
/// selected case. The refused proof binds to its owner (U2, failure path); the case is
/// unavailable at the facade with the actual selected Run and its CaseSource.
#[test]
fn u1g2_candidate_refusal_after_selected_run() {
    use super::retained_receipt::TraceFault as F;
    let mode = PreviewSolverMode::SparseInteractive;
    for (fault, error) in [(F::Maxima, "abandoned"), (F::ValuesCompletion, "values")] {
        let (mut prepared, staged, capture) = refused_second_case(mode, fault);
        let successor = wire::serialize_cases(&mut prepared, &staged, &capture).unwrap_or_else(|f| panic!("{fault:?}: {f:?}"));
        let b = body(&successor);
        let (case, attempt) = (&b["cases"][1], &b["product_attempts"][1]);
        assert_eq!((b["cases"][0]["status"].as_str(), case["status"].as_str(), &case["product_attempt_ref"]), (Some("selected"), Some("unavailable"), &json!(1)), "{fault:?}");
        assert_eq!(attempt["result"]["error"]["kind"].as_str(), Some(error), "{fault:?}");
        assert_eq!((&case["reason"]["code"], &case["reason"]["phase"]), (&json!("facade_certificate"), &json!("facade")));
        assert_eq!(case["run"]["kernel_terminal"], json!({"kind":"selected","reason":null}));
        assert_eq!((&case["source_ref"], &attempt["source_ref"], &attempt["run_ref"]), (&json!(1), &json!(1), &json!(1)));
        assert_eq!(b["sources"].as_array().map(Vec::len), Some(2));
        assert!(case.get("selection").is_none() && case.get("method").is_none());
        write_receipt(&format!("u1g2_candidate_{error}_sparse_interactive"), &successor, mode);
    }
}

/// U2 on the failure path: a refused proof of another selected owner with
/// identical public facts is refused structurally.
#[test]
fn u2_failure_path_foreign_owner_refused() {
    use super::retained_receipt::TraceFault as F;
    let mode = PreviewSolverMode::SparseInteractive;
    let (mut first, staged, capture) = refused_second_case(mode, F::ValuesCompletion);
    let (mut second, _, _) = refused_second_case(mode, F::ValuesCompletion);
    assert!(wire::serialize_cases(&mut first, &staged, &capture).is_ok(), "own refused proof serializes");
    refused_mut(&mut first, 1).test_swap_error(refused_mut(&mut second, 1));
    let refused = wire::serialize_cases(&mut first, &staged, &capture);
    println!("I61_FOREIGN_OWNER failure_path_serializer_refused={}", refused.is_err());
    assert_eq!(refused.err(), Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "product_attempts[].proof.owner" }));
}

/// C1 §1 item 2 (grant 2): a failed verification keeps its translated failure
/// reason; a completed one has none. The milestone's records, with the
/// verification outcome replaced (a projection control, not a native run).
#[test]
fn u1g2_failed_verification_keeps_its_reason() {
    let (candidate, _, _) = milestone_candidate(PreviewSolverMode::SparseInteractive);
    let k::ExecutionOutcome::Selected(owner) = &candidate.capture().native_pair().unwrap().1.outcome else { panic!("selected") };
    let mut records = owner.evidence().attempts.clone();
    assert!(matches!(records[1].role, k::AttemptRole::Verification), "precondition: a candidate/verification pair");
    let (completed, _) = wire::test_logical(&records);
    assert_eq!(completed[0]["verification"]["phase"], json!("completed"));
    assert_eq!(completed[0]["verification"]["reason"], Value::Null);
    records[1].outcome = k::AttemptOutcome::Failed(k::AttemptReason::Stop(k::AttemptStop::Condition));
    let (failed, failures) = wire::test_logical(&records);
    assert!(failures.is_empty(), "{failures:?}");
    assert_eq!(failed[0]["verification"], json!({"record":1,"precision":records[1].precision,"phase":"failed",
        "reason":{"space":"attempt","tag":"stop","stop":{"space":"stop","tag":"condition"}}}));
    records[1].outcome = k::AttemptOutcome::Rejected(k::AttemptReason::VerificationFailed);
    let (_, failures) = wire::test_logical(&records);
    assert_eq!(failures.iter().map(|f| f.field_path).collect::<Vec<_>>(), vec!["run.attempts[].verification.phase"]);
}

/// C1 §4 Run.kernel_terminal for each kernel outcome (grant 2).
#[test]
fn u1g2_kernel_terminals() {
    let (candidate, _, _) = milestone_candidate(PreviewSolverMode::SparseInteractive);
    let outcome = &candidate.capture().native_pair().unwrap().1.outcome;
    let k::ExecutionOutcome::Selected(owner) = outcome else { panic!("selected") };
    let records = owner.evidence().attempts.clone();
    assert_eq!(wire::test_kernel_terminal(outcome), (records.len(), json!({"kind":"selected","reason":null})));
    let unresolved = k::ExecutionOutcome::Unresolved { reason: k::UnresolvedReason::Ceiling, attempts: records.clone(), geometry: Vec::new() };
    assert_eq!(wire::test_kernel_terminal(&unresolved), (records.len(), json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}})));
    let refused = k::ExecutionOutcome::Refused { refusal: k::Refusal::Structure, attempts: Vec::new(), geometry: Vec::new() };
    assert_eq!(wire::test_kernel_terminal(&refused), (0, json!({"kind":"refused","reason":{"space":"refusal","tag":"structure"}})));
}

/// C2/C3: a source-constructor refusal needs `source_decline`, whose constructor counts are not
/// captured typed, so it fails closed (typed) even beside a selected case.
#[test]
fn u1g2_source_constructor_refusal_fails_closed() {
    use super::retained_receipt::TraceFault as F;
    let mode = PreviewSolverMode::SparseInteractive;
    hooks::fault_case(1, F::SourceConstruction);
    let (mut prepared, staged, capture) = transaction(&milestone_two_cases(), mode);
    assert!(hooks::armed_names().is_empty(), "the fault fired");
    assert!(matches!(prepared.attempts[1].end, rp::AttemptEnd::Preparation) && matches!(prepared.attempts[0].end, rp::AttemptEnd::Frozen(_)));
    let error = prepared.capture.with_case(1, |c| c.error.clone());
    assert!(matches!(error, Some(rp::CaptureError::Source(_))), "precondition: {error:?}");
    assert_eq!(wire::serialize_cases(&mut prepared, &staged, &capture).err(),
        Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Untranslated, field_path: "cases[].source_decline" }));
}

/// U2 on the failure path, a refused case's surviving certified proof: case-2's freeze is
/// refused at its observables stage, after its certificate (its first stress extremum's
/// `approximation` is altered in the ordinary owner after custody), beside a selected case; a
/// foreign certificate is refused.
#[test]
fn u2_failure_path_foreign_certificate_refused() {
    let mode = PreviewSolverMode::SparseInteractive;
    let raw = milestone_two_cases();
    let observable_refusal = || {
        let (mut prepared, capture) = called(&raw, mode);
        prepared.ordinary.contract_evidence.as_mut().unwrap()["preview_cases"][1]["pipe_stress_extrema"][0]["approximation"] = json!("altered");
        let staged = staged(&mut prepared);
        assert!(matches!(prepared.attempts[0].end, rp::AttemptEnd::Frozen(_)), "precondition: case 0 selected");
        let refusal = refused_mut(&mut prepared, 1);
        assert!(matches!(refusal.error, rp::PreparedCandidateError::Observable), "precondition: {:?}", refusal.error);
        assert!(refusal.certificate().is_some() && refusal.proof_failure().is_none(), "precondition: certified proof survives");
        (prepared, staged, capture)
    };
    let (mut first, staged, capture) = observable_refusal();
    let (mut second, _, _) = observable_refusal();
    assert!(wire::serialize_cases(&mut first, &staged, &capture).is_ok(), "own certified proof serializes");
    refused_mut(&mut first, 1).test_swap_certificate(refused_mut(&mut second, 1));
    let refused = wire::serialize_cases(&mut first, &staged, &capture);
    assert_eq!(refused.err(), Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "product_attempts[].proof.owner" }));
}

/// RV82-S2: the supplied invocation binds structurally (its digest), not by mode
/// and case id alone: a same-mode invocation with another case label, project id
/// or load is refused before projection, with a selected case alone and beside an
/// unavailable one.
#[test]
fn u1g2_foreign_invocation_refused() {
    let mode = PreviewSolverMode::SparseInteractive;
    let (mut prepared, staged, own) = transaction(&milestone_raw(), mode);
    assert!(wire::serialize_cases(&mut prepared, &staged, &own).is_ok(), "own invocation serializes");
    let foreign = |raw: Value, edit: &dyn Fn(&mut Value)| {
        let mut raw = raw;
        edit(&mut raw);
        source_receipt::CapturedInvocation::parse(raw, mode).unwrap().1
    };
    let refused = Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "invocation" });
    for (label, edit) in [
        ("label", &(|r: &mut Value| r["model"]["load_cases"][0]["label"] = json!("another label")) as &dyn Fn(&mut Value)),
        ("project_id", &|r: &mut Value| r["model"]["project"]["id"] = json!("another-project")),
        ("load", &|r: &mut Value| r["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(0.0049)),
    ] {
        let other = foreign(milestone_raw(), edit);
        assert_eq!(other.mode(), mode, "{label}: same mode");
        assert_ne!(other.borrowed_digest(), own.borrowed_digest(), "{label}: a different invocation");
        assert_eq!(wire::serialize_cases(&mut prepared, &staged, &other).err(), refused, "{label}");
    }
    let (mut refusal, refusal_staged, own_refusal_invocation) = refused_second_case(mode, super::retained_receipt::TraceFault::ValuesCompletion);
    assert!(wire::serialize_cases(&mut refusal, &refusal_staged, &own_refusal_invocation).is_ok());
    let other = foreign(milestone_two_cases(), &|r: &mut Value| r["model"]["load_cases"][1]["label"] = json!("another label"));
    assert_eq!(wire::serialize_cases(&mut refusal, &refusal_staged, &other).err(), refused);
}

/// RV82 N1′ (R08 at its call site): `run_conservation`'s own checks over explicit
/// amounts. Each violated amount is refused at its own path, and only there.
#[test]
fn u1g2_run_conservation_pins_each_check_at_its_call_site() {
    let inconsistent = |path: &'static str| vec![wire::ReceiptFailure { check: wire::ReceiptCheck::WorkCounterInconsistent, field_path: path }];
    let attempts = [json!({"case_charge": 3, "invocation_increment": 4}), json!({"case_charge": 2, "invocation_increment": 1})];
    assert!(wire::test_run_conservation_amounts(&attempts, 5, 10, 5, 15).is_empty(), "conserved");
    assert_eq!(wire::test_run_conservation_amounts(&attempts, 5, 10, 5, 16), inconsistent("cases[].run.invocation_after"), "R08");
    assert_eq!(wire::test_run_conservation_amounts(&attempts, 5, 10, 5, 14), inconsistent("cases[].run.invocation_after"), "R08");
    assert_eq!(wire::test_run_conservation_amounts(&attempts, 6, 10, 5, 15), inconsistent("cases[].run.case_charge"));
    let mut both = inconsistent("cases[].run.invocation_increment");
    both.extend(inconsistent("cases[].run.invocation_after"));
    assert_eq!(wire::test_run_conservation_amounts(&attempts, 5, 10, 6, 15), both, "increment, then the after it implies");
}

/// RV82-N1 (R08, R09): run after = before + increment, and body charged = the
/// final after, each refused as `work_counter_inconsistent` when violated.
#[test]
fn u1g2_run_and_body_charge_conservation_negatives() {
    assert_eq!(wire::test_after_conserved(5, 7, 12), (true, Vec::new()));
    let (ok, failures) = wire::test_after_conserved(5, 7, 13);
    assert!(!ok && failures.is_empty(), "R08: the after check fails without an encoder fault");
    let (sparse, _, _) = milestone_candidate(PreviewSolverMode::SparseInteractive);
    let (inv, case) = sparse.capture().native_pair().unwrap();
    let run = &inv.runs()[case.run];
    assert!(wire::test_invocation_arrays(inv, run).is_ok(), "own run");
    // An invocation whose meter never charged this run (same policy limit).
    let capacity = k::OriginCapacity::for_calls(&[1], &[]).unwrap();
    let fresh = k::RecordedInvocation::new(wire::INVOCATION_LIMIT, capacity).unwrap();
    assert_eq!(fresh.meter().checked_charged().exact(), Ok(0));
    assert_eq!(wire::test_invocation_arrays(&fresh, run).err(),
        Some(wire::ReceiptFailure { check: wire::ReceiptCheck::WorkCounterInconsistent, field_path: "work.charged" }), "R09");
}

/// RV82-N1 (R10) after B4/D1 (b): the preview route attempts no source blocks, so the
/// actual envelope carries no source-unavailable disclosure; one naming the selected case
/// (a route-E composite's) is omitted, and a second still naming it is refused (G4).
#[test]
fn u1g2_t1a_guard_negatives() {
    let (candidate, _, _) = milestone_candidate(PreviewSolverMode::SparseInteractive);
    let base = serde_json::to_value(candidate.envelope()).unwrap();
    assert!(!base["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"));
    let mut own = base.clone();
    assert!(wire::test_successor_envelope(&mut own, "case").is_ok(), "precondition: the actual envelope");
    let disclosure = |id: &str| json!({"id": id, "code": "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", "severity": "info",
        "message": "m", "source": "core/product_physics", "affected_refs": ["case"]});
    let mut one = base.clone();
    one["diagnostics"].as_array_mut().unwrap().push(disclosure("diagnostic:source-recovery:case"));
    assert!(wire::test_successor_envelope(&mut one, "case").is_ok());
    assert!(!one["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"), "omitted");
    let mut doubled = base.clone();
    doubled["diagnostics"].as_array_mut().unwrap().push(disclosure("diagnostic:source-recovery:case"));
    doubled["diagnostics"].as_array_mut().unwrap().push(disclosure("diagnostic:source-recovery:case:second"));
    assert_eq!(wire::test_successor_envelope(&mut doubled, "case").err(),
        Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "diagnostics[-source_unavailable]" }), "R10");
}

/// RV82-N1 (R13): the report outcome must equal the unchanged assessed quality.
#[test]
fn u1g2_initial_quality_crosscheck_negative() {
    let (candidate, _, _) = milestone_candidate(PreviewSolverMode::SparseInteractive);
    let [seed] = &candidate.capture().ordinary[..] else { panic!("one seed") };
    let mut env = serde_json::to_value(candidate.envelope()).unwrap();
    assert!(wire::test_ordinary_value(&env, "case", seed).is_ok(), "precondition: sensitive = sensitive");
    env["numerical_quality"]["cases"][0]["solve_quality"] = json!("checks_passed");
    assert_eq!(wire::test_ordinary_value(&env, "case", seed).err(),
        Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "ordinary_attempts[].initial.outcome" }), "R13");
}

/// B3b-P (B3-D P-9; REVISION_01 §1.2, S-1): the exact route's wire constants are bound to the
/// in-tree statics J1 landed: DEF-E's H is `EXACT_DEFINITION_SHA256` and the physics-retained-1
/// table binds it, its identity, profile, policies and `receipt_bindings` (B3D-8, the serializer's
/// own values); its inherited hash is pressure-2's table bytes (B7b D4 (a)). The preview route's descriptor is
/// the unchanged module constants.
#[test]
fn b3b_exact_constants_bound_to_in_tree_fixtures() {
    const EXACT_DEFINITION: &str = include_str!("../../../fixtures/results/retained_precision_prepared_exact_v1.json");
    const EXACT_TABLE: &str = include_str!("../../../fixtures/results/semantic_contract_v0_3_physics_retained_1.json");
    // B7b D4 (a): physics-retained-1 is based on pressure-2 (exact_pressure_v3).
    const PRESSURE_1_TABLE: &str = include_str!("../../../fixtures/results/semantic_contract_v0_3_pressure_2.json");
    let definition: Value = serde_json::from_str(EXACT_DEFINITION).unwrap();
    let table: Value = serde_json::from_str(EXACT_TABLE).unwrap();
    assert_eq!(definition["id"], json!(wire::EXACT_DEFINITION_ID));
    assert_eq!(wire::domain_hash("retained_precision_formation_v1", &definition).as_deref(), Some(wire::EXACT_DEFINITION_SHA256));
    assert_eq!(table["product_formation_definitions"], json!([{"id":wire::EXACT_DEFINITION_ID,"sha256":wire::EXACT_DEFINITION_SHA256}]));
    assert_eq!(table["formation_warrant"]["definition_id"], json!(wire::EXACT_DEFINITION_ID));
    assert_eq!(table["semantic_contract_id"], json!(wire::EXACT_SEMANTIC_ID));
    assert_eq!(table["formulation_profile_id"], json!(wire::EXACT_PROFILE_ID));
    assert_eq!(table["receipt_policy"], json!(wire::POLICY));
    assert_eq!(table["accuracy_classification"]["policy"], json!(wire::FACADE_POLICY));
    assert_eq!(table["receipt_bindings"], json!({"canonicalization":wire::CANONICALIZATION,"method":wire::METHOD,"projection_policy":wire::PROJECTION_POLICY,
        "work":{"case_limit":wire::CASE_LIMIT,"invocation_limit":wire::INVOCATION_LIMIT},"work_policy":wire::WORK_POLICY}));
    assert_eq!(table["inherited_semantic_contract_sha256"], json!(format!("{:x}", Sha256::digest(PRESSURE_1_TABLE.as_bytes()))));
    assert_eq!(serde_json::from_str::<Value>(PRESSURE_1_TABLE).unwrap()["semantic_contract_id"], json!(crate::exact_admission::PRESSURE_SEMANTIC_CONTRACT_ID));
    // The two descriptors.
    let preview = wire::route_wire(rp::W1Route::Preview);
    assert_eq!((preview.semantic_id, preview.profile_id, preview.definition_id, preview.definition_sha256, preview.geometry_route),
        (wire::RETAINED_SEMANTIC_ID, wire::RETAINED_PROFILE_ID, wire::DEFINITION_ID, wire::DEFINITION_SHA256, "preview"));
    let exact = wire::route_wire(rp::W1Route::Exact);
    assert_eq!((exact.semantic_id, exact.profile_id, exact.definition_id, exact.definition_sha256, exact.geometry_route),
        (wire::EXACT_SEMANTIC_ID, wire::EXACT_PROFILE_ID, wire::EXACT_DEFINITION_ID, wire::EXACT_DEFINITION_SHA256, "exact"));
    // SCHEMA admits the exact id for a product attempt (J1's enum).
    let schema: Value = serde_json::from_str(include_str!("../../../schemas/retained_precision_mp_v2.schema.json")).unwrap();
    assert!(schema["$defs"]["ProductAttempt"]["properties"]["definition_id"]["enum"].as_array().unwrap().contains(&json!(wire::EXACT_DEFINITION_ID)));
}

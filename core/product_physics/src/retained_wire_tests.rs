//! I61 U1 grant 1 (with U2): tests of the private retained-precision serializer
//! (`retained_wire`) through the private prepared driver. The accepted Rust
//! reader runs in-process (dev-dependency); Python and TypeScript validate the
//! emitted receipts in their own lanes (I61_U1_OUT).
use super::retained_product as rp;
use super::retained_wire as wire;
use super::*;
use open_pipe_stress_frame_kernel::structural::exact_boundary::functionals::AttemptStage;
use open_pipe_stress_frame_kernel::structural::retained_api as k;
use open_pipe_stress_result_export::retained_precision as reader;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const DEFINITION: &str = include_str!("../../../fixtures/results/retained_precision_prepared_ordinary_v1.json");
const TABLE: &str = include_str!("../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
/// Protected ordinary bytes of the milestone (experiment 03 controls B and B'):
/// the plain route's serialized envelope, equal with and without capture.
const ORDINARY_SHA256: [(&str, usize, &str); 2] = [
    ("sparse_interactive", 68250, "9c7ec1a144a729f456a25fedb5dbe6a7dafde67d52b9cc8caff6c39bb0050871"),
    ("dense_scrutiny", 69366, "21ca629c27e6ca03b1411c8c51097f7a50f90429045b1e36313163014dd4278a"),
];
/// The serializer's committed milestone output: sha256 of the pretty-printed
/// successor envelope, and its receipt_sha256 (byte-stable, both modes).
const SUCCESSOR_SHA256: [(&str, &str, &str); 2] = [
    ("sparse_interactive", "ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc", "efc1a39bbe83840df6bd0761c932b8020285d3b8c005ba8fd0b45ba10d667494"),
    ("dense_scrutiny", "6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5", "3e26499f17caff8f5fc0d46406bbe54acf43e8cb16e761784aa5074413b0ac4a"),
];

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn milestone_raw() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
/// The private prepared driver on the unchanged 0.1.0 milestone: the candidate,
/// its captured invocation and the bytes of its single ordinary run.
fn milestone_candidate(mode: PreviewSolverMode) -> (rp::PrivatePreparedCandidate, source_receipt::CapturedInvocation, Vec<u8>) {
    let raw = milestone_raw();
    assert_eq!(raw["model"]["schema_version"], json!("0.1.0"), "milestone request unchanged at 0.1.0");
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
        let direct = run_linear_static_preview_value_with_retained_direct(milestone_raw(), mode).unwrap();
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
        match &seed.legacy {
            Some(rp::LegacySeed::Unavailable { work, diagnostic_ref }) => {
                assert_eq!(diagnostic_ref, "diagnostic:source-recovery:case");
                assert_eq!(*work, rp::LegacyWork { stage: "source closure", helper_stage: AttemptStage::SourceClosure,
                    charged: 46628, rejected: 0, limit: 4_000_000 });
            }
            other => panic!("legacy unavailable expected: {other:?}"),
        }
    }
}

/// The deliverable: the serializer's milestone receipts, both modes, validated
/// in-process by the accepted Rust reader, eligible with the invocation (U7), with exact
/// class parity against the certificate's own verdicts, byte-stable.
#[test]
fn u1_milestone_successor_both_modes() {
    let out = std::env::var("I61_U1_OUT").ok().map(std::path::PathBuf::from);
    for (mode, (name, file_sha, receipt_sha)) in MODES.into_iter().zip(SUCCESSOR_SHA256) {
        assert_eq!(mode.as_str(), name);
        let (candidate, capture, _) = milestone_candidate(mode);
        let successor = wire::serialize_selected(&candidate, &capture).unwrap_or_else(|f| panic!("{name}: {f:?}"));
        let again = wire::serialize_selected(&candidate, &capture).unwrap();
        assert_eq!(pretty(&successor), pretty(&again), "{name}: deterministic");
        let invocation = invocation(mode);
        let validation = reader::validate(&successor, Some(&invocation)).unwrap_or_else(|e| panic!("{name}: reader {e:?}"));
        assert!(validation.invocation_bound && validation.numerical_eligible, "{name}: eligible with the actual invocation (U7)");
        // Parity: every reader class equals the certificate's verdict class.
        let rows: Vec<&str> = successor["results"].as_array().unwrap().iter().map(|r| r["id"].as_str().unwrap()).collect();
        let verdicts = candidate.certificate().verdicts();
        assert_eq!(validation.classifications.len(), verdicts.len(), "{name}: one class per row");
        for (c, v) in validation.classifications.iter().zip(verdicts) {
            assert_eq!(c.result_id, rows[v.row]);
            assert_eq!(c.normalized_bits, v.normalized_bits, "{name}: {}", c.result_id);
            let expected = match v.class {
                Some(k::RowClass::RelativeVerified) => reader::AccuracyClass::RelativeVerified,
                Some(k::RowClass::AbsoluteVerified { bound_bits }) => reader::AccuracyClass::AbsoluteVerified { bound_bits },
                Some(k::RowClass::InputDerived) => reader::AccuracyClass::InputDerived,
                Some(k::RowClass::Unpublishable) => panic!("unpublishable"),
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
    let (mut first, capture, _) = milestone_candidate(mode);
    let (mut second, _, _) = milestone_candidate(mode);
    assert!(wire::serialize_selected(&first, &capture).is_ok(), "own proof serializes");
    first.test_swap_certificate(&mut second);
    let refused = wire::serialize_selected(&first, &capture);
    println!("I61_FOREIGN_OWNER serializer_refused={}", refused.is_err());
    assert_eq!(refused, Err(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "cases[].selection.owner" }));
    assert_eq!(wire::serialize_selected(&second, &capture).err().map(|f| f.field_path), Some("cases[].selection.owner"));
}

/// ROOT correction (I65 D-4 §3): every record amount is read through its checked
/// accessor. A record whose `work_status` latch holds a fault (here a stage total
/// overflow) refuses on every latched view while its legacy fields look plausible.
#[test]
fn u1_checked_record_work_honours_the_latch() {
    let (candidate, _, _) = milestone_candidate(PreviewSolverMode::SparseInteractive);
    let k::ExecutionOutcome::Selected(owner) = &candidate.capture().native.as_ref().unwrap().1.outcome else { panic!("selected") };
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
    let (candidate, _, _) = milestone_candidate(PreviewSolverMode::SparseInteractive);
    let (inv, case) = candidate.capture().native.as_ref().unwrap();
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
    let successor = wire::serialize_selected(&candidate, &source_receipt::CapturedInvocation::parse(milestone_raw(), PreviewSolverMode::SparseInteractive).unwrap().1).unwrap();
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
        (C::SaturationNotExcluded, "saturation_not_excluded"),
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

/// D-4 §3 items 2 and 4: a trace Count is exact or abandons, and a legacy
/// `rejected` beyond the safe range is `saturation_not_excluded`.
#[test]
fn u1_exact_or_abandon_counts_and_legacy_rejected() {
    let mut overflow = k::StageWork::default();
    overflow.formation = u64::MAX;
    overflow.assembly = 1;
    let (count, failures) = wire::test_count(overflow.checked_total());
    assert_eq!(count["kind"], json!("unavailable"));
    assert_eq!(failures, vec![wire::ReceiptFailure { check: wire::ReceiptCheck::WorkCounter(k::WorkFault::Overflow), field_path: "Count.value" }]);
    let (count, failures) = wire::test_count(k::StageWork::default().checked_total());
    assert_eq!((count, failures.len()), (json!({"kind":"exact","value":0}), 0));
    let saturated = rp::LegacyWork { stage: "solve", helper_stage: AttemptStage::Solve, charged: 7, rejected: usize::MAX, limit: 9 };
    let mut work = Vec::new();
    assert_eq!(wire::test_legacy_source(Some(&rp::LegacySeed::Unavailable { work: saturated, diagnostic_ref: "d".into() }), 0, &mut work),
        Err(wire::ReceiptFailure { check: wire::ReceiptCheck::SaturationNotExcluded, field_path: "legacy_source_work[].rejected" }));
    let ranged = rp::LegacyWork { charged: 1 << 53, rejected: 0, ..saturated };
    assert_eq!(wire::test_legacy_source(Some(&rp::LegacySeed::Unavailable { work: ranged, diagnostic_ref: "d".into() }), 0, &mut Vec::new()).err().map(|f| f.check),
        Some(wire::ReceiptCheck::WorkCounterRange));
}

/// D39 (ROOT, 2026-10-04) with T1 (a): each legacy route row's disposition.
#[test]
fn u1_d39_legacy_dispositions() {
    let work = rp::LegacyWork { stage: "formation guard", helper_stage: AttemptStage::SourceClosure, charged: 0, rejected: 0, limit: 0 };
    let attempted = rp::LegacyWork { stage: "solve", helper_stage: AttemptStage::Solve, charged: 7, rejected: 3, limit: 9 };
    let rows: [(rp::LegacySeed, Value, Option<&str>, Option<Value>); 4] = [
        (rp::LegacySeed::NotEligible, json!({"disposition":"not_eligible","diagnostic_ref":null,"work_ref":null}), None, None),
        (rp::LegacySeed::NotRequired, json!({"disposition":"not_required","diagnostic_ref":null,"work_ref":null}), None, None),
        (rp::LegacySeed::DeclinedWithoutAttempt { work, diagnostic_ref: "d:decline".into() },
            json!({"disposition":"declined_without_attempt","diagnostic_ref":null,"work_ref":0}), Some("d:decline"),
            Some(json!({"case_index":2,"stage":"formation guard","helper_stage":"source_closure","charged":0,"rejected":0,"limit":0,"settlement":"booked"}))),
        (rp::LegacySeed::Unavailable { work: attempted, diagnostic_ref: "d:attempt".into() },
            json!({"disposition":"unavailable","diagnostic_ref":null,"work_ref":0}), Some("d:attempt"),
            Some(json!({"case_index":2,"stage":"solve","helper_stage":"solve","charged":7,"rejected":3,"limit":9,"settlement":"booked"}))),
    ];
    for (seed, expected, omitted, entry) in rows {
        let mut work_list = Vec::new();
        let (value, omit) = wire::test_legacy_source(Some(&seed), 2, &mut work_list).unwrap();
        assert_eq!(value, expected, "{seed:?}");
        assert_eq!(omit.as_deref(), omitted, "{seed:?}");
        assert_eq!(work_list.first(), entry.as_ref(), "{seed:?}");
    }
    // Ok(recovery): exact-block selected, the coexistence bypass (D-15): no successor.
    let mut work_list = Vec::new();
    assert_eq!(wire::test_legacy_source(Some(&rp::LegacySeed::ExactSelected), 0, &mut work_list).err().map(|f| f.check),
        Some(wire::ReceiptCheck::Scope));
    // No captured route: refused typed, never guessed.
    assert_eq!(wire::test_legacy_source(None, 0, &mut work_list).err().map(|f| f.check), Some(wire::ReceiptCheck::Untranslated));
}

/// D39 at the capture: the route recorders keep each branch distinct (G-l).
#[test]
fn u1_d39_route_capture() {
    let failure = |attempted: bool| {
        let mut capture = rp::ProductCapture::default();
        let decline = source_recovery::formation_decline_without_attempt();
        capture.ordinary_legacy_failure("c", attempted, &decline, "d:legacy");
        // The route call after the attempt block never overwrites a failure.
        capture.ordinary_legacy_route("c", true, true, false);
        capture.ordinary[0].legacy.clone()
    };
    assert!(matches!(failure(false), Some(rp::LegacySeed::DeclinedWithoutAttempt { work, ref diagnostic_ref })
        if work.stage == "formation guard" && (work.charged, work.rejected, work.limit) == (0, 0, 0) && diagnostic_ref == "d:legacy"));
    assert!(matches!(failure(true), Some(rp::LegacySeed::Unavailable { .. })));
    let route = |eligible: bool, needs: bool, exact: bool| {
        let mut capture = rp::ProductCapture::default();
        capture.ordinary_legacy_route("c", eligible, needs, exact);
        capture.ordinary[0].legacy.clone()
    };
    assert!(matches!(route(false, true, false), Some(rp::LegacySeed::NotEligible)));
    assert!(matches!(route(false, false, false), Some(rp::LegacySeed::NotEligible)));
    assert!(matches!(route(true, false, false), Some(rp::LegacySeed::NotRequired)));
    assert!(matches!(route(true, true, true), Some(rp::LegacySeed::ExactSelected)));
    assert!(route(true, true, false).is_none(), "an attempted branch is recorded only by its failure");
}

/// The in-tree fixtures bind the serializer's constants (A4 closed by the fan-in).
#[test]
fn u1_constants_bound_to_in_tree_fixtures() {
    let definition: Value = serde_json::from_str(DEFINITION).unwrap();
    let table: Value = serde_json::from_str(TABLE).unwrap();
    assert_eq!(wire::domain_hash("retained_precision_formation_v1", &definition).as_deref(), Some(wire::DEFINITION_SHA256));
    assert_eq!(table["product_formation_definitions"], json!([{"id":wire::DEFINITION_ID,"sha256":wire::DEFINITION_SHA256}]));
    assert_eq!(table["semantic_contract_id"], json!(wire::RETAINED_SEMANTIC_ID));
    assert_eq!(table["formulation_profile_id"], json!(wire::RETAINED_PROFILE_ID));
    assert_eq!(table["receipt_policy"], json!(wire::POLICY));
    assert_eq!(table["accuracy_classification"]["policy"], json!(wire::FACADE_POLICY));
}

/// Scope (G-j) and untranslated variants refuse typed instead of guessing.
#[test]
fn u1_refusals_are_typed() {
    let mode = PreviewSolverMode::SparseInteractive;
    // Another invocation's custody for the same candidate: the mode differs.
    let (candidate, _, _) = milestone_candidate(mode);
    let (_, other) = source_receipt::CapturedInvocation::parse(milestone_raw(), PreviewSolverMode::DenseScrutiny).unwrap();
    assert_eq!(wire::serialize_selected(&candidate, &other).err().map(|f| f.field_path), Some("invocation"));
    // A second requested case is wider F2a; since RV82-S2 such an invocation is
    // also not the one this candidate observed, so it is refused as foreign first
    // (the Scope branch remains for the candidate's own invocation).
    let mut raw = milestone_raw();
    let mut second = raw["model"]["load_cases"][0].clone();
    second["id"] = json!("case-2");
    raw["model"]["load_cases"].as_array_mut().unwrap().push(second);
    let (_, two) = source_receipt::CapturedInvocation::parse(raw, mode).unwrap();
    assert_eq!(wire::serialize_selected(&candidate, &two).err(), Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "invocation" }));
}

/// G-b and D39 on an actual load-row case (S11-G T1, RF-CANCEL-UDL-W1e8, captured
/// entry): the finding demotes a Passed report, so its sentence is disclosed in
/// the integrity diagnostic, and the legacy route declines without an attempt.
/// Capture leaves the ordinary bytes unchanged here too.
#[test]
fn u1_load_row_case_capture() {
    let cases: Value = serde_json::from_str(include_str!("../tests/fixtures/s11f/rf_cancel_cases.json")).unwrap();
    let raw = cases["cases"].as_array().unwrap().iter().find(|c| c["id"] == json!("RF-CANCEL-UDL-W1e8")).unwrap()["request"].clone();
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
        match &seed.legacy {
            Some(rp::LegacySeed::DeclinedWithoutAttempt { work, diagnostic_ref }) => {
                assert_eq!(*work, rp::LegacyWork { stage: "formation guard", helper_stage: AttemptStage::SourceClosure, charged: 0, rejected: 0, limit: 0 });
                assert_eq!(diagnostic_ref, &format!("diagnostic:source-recovery:{}", seed.case));
            }
            other => panic!("{mode:?}: D39 declined_without_attempt expected: {other:?}"),
        }
        println!("I61_U1_LOAD_ROW {} initial=report(sensitive) finding_ref={:?} d5={:?} legacy=declined_without_attempt", mode.as_str(), finding.diagnostic_ref, seed.d5_diagnostic_ref);
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

/// The private driver up to a refused candidate (selected native Run, then an
/// injected facade fault after the proof started).
fn refused_candidate(mode: PreviewSolverMode, fault: super::retained_receipt::TraceFault)
    -> (rp::PreparedCandidateRefusal, source_receipt::CapturedInvocation) {
    let (request, capture) = source_receipt::CapturedInvocation::parse(milestone_raw(), mode).unwrap();
    let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
    prepared.test_capture_mut().trace_fault = Some(fault);
    prepared.solve_native().unwrap();
    match prepared.project_candidate() { Err(r) => (r, capture), Ok(_) => panic!("refusal expected") }
}
fn write_receipt(name: &str, successor: &Value, mode: PreviewSolverMode) {
    if let Some(dir) = std::env::var("I61_U1_OUT").ok().map(std::path::PathBuf::from) {
        std::fs::write(dir.join(format!("{name}.json")), pretty(&json!({"id":name,"source":successor,"invocation":invocation(mode)}))).unwrap();
    }
}
fn body(successor: &Value) -> &Value {
    &successor["retained_precision"]["body"]
}

/// Grant 2, item 4: the C1-C3 unavailable representation of an actual
/// preparation refusal (injected trigger, as experiment 02: facts[0] diameter 0).
/// Under T3 it is not a publication; every accepted reader refuses it at G3.
#[test]
fn u1g2_preparation_refusal_representation() {
    let mode = PreviewSolverMode::SparseInteractive;
    let (request, capture) = source_receipt::CapturedInvocation::parse(milestone_raw(), mode).unwrap();
    let mut observer = rp::ProductCapture::prepared_probe();
    let ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
    observer.facts[0].diameter = 0.0;
    let failed = match observer.prepare_case(ordinary) { Err(f) => f, Ok(_) => panic!("helper refusal expected") };
    let successor = wire::serialize_unavailable(wire::Refused::Preparation(&failed), &capture).unwrap_or_else(|f| panic!("{f:?}"));
    let b = body(&successor);
    let case = &b["cases"][0];
    assert_eq!((case["status"].as_str(), &case["reason"]), (Some("unavailable"),
        &json!({"code":"source_unavailable","phase":"preparation","cause":{"kind":"prepared_product_failure","product_attempt_ref":0}})));
    assert_eq!((&case["run"], &case["source_ref"], &b["sources"], &b["work"]["execution_order"], &b["calls"]), (&Value::Null, &Value::Null, &json!([]), &json!([]), &json!([])));
    let attempt = &b["product_attempts"][0];
    assert_eq!((attempt["result"]["kind"].as_str(), attempt["result"]["error"]["kind"].as_str(), &attempt["run_ref"], &attempt["source_ref"]),
        (Some("unavailable"), Some("preparation"), &Value::Null, &Value::Null));
    // Unselected: the legacy disclosure is kept and referenced (C2:160), no method token.
    assert_eq!(b["ordinary_attempts"][0]["legacy_source"], json!({"disposition":"unavailable","diagnostic_ref":"diagnostic:source-recovery:case","work_ref":0}));
    assert!(successor["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"));
    assert!(successor["results"].as_array().unwrap().iter().all(|r| r.get("recovery_method").is_none()));
    let unavailable: Vec<&Value> = successor["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == wire::UNAVAILABLE_CODE).collect();
    assert_eq!(unavailable.len(), 1);
    assert_eq!((unavailable[0]["id"].as_str(), &case["diagnostic_ref"]), (Some("diagnostic:retained-precision:case:unavailable"), &unavailable[0]["id"]));
    write_receipt("u1g2_preparation_refusal_sparse_interactive", &successor, mode);
}

/// Grant 2, item 2 (D38): a prepared solve that fails before any kernel schedule
/// (injected: the prepared source is withdrawn) carries `run:null`, `run_ref:null`
/// and a `capture` error with the actual cause.
#[test]
fn u1g2_d38_failure_before_any_run() {
    for mode in MODES {
        let (request, capture) = source_receipt::CapturedInvocation::parse(milestone_raw(), mode).unwrap();
        let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
        prepared.test_capture_mut().source = None;
        assert!(prepared.solve_native().is_err());
        assert!(prepared.capture().native.is_none(), "precondition: no kernel schedule ran");
        let successor = wire::serialize_unavailable(wire::Refused::Native(&prepared), &capture).unwrap_or_else(|f| panic!("{f:?}"));
        let b = body(&successor);
        let (case, attempt) = (&b["cases"][0], &b["product_attempts"][0]);
        assert_eq!((&case["run"], &attempt["run_ref"], &case["source_ref"], &attempt["source_ref"]), (&Value::Null, &Value::Null, &Value::Null, &Value::Null));
        assert_eq!(attempt["result"], json!({"kind":"unavailable","error":{"kind":"capture","cause":{"kind":"association","detail":"prepared source"}}}));
        assert_eq!(attempt["stages"]["native"], json!("failed"), "the native stage was entered");
        assert_eq!((&case["reason"]["code"], &case["reason"]["phase"]), (&json!("source_unavailable"), &json!("preparation")));
        assert_eq!((&b["work"]["charged"], &b["work"]["execution_order"], &b["calls"]), (&json!(0), &json!([]), &json!([])));
        write_receipt(&format!("u1g2_d38_{}", mode.as_str()), &successor, mode);
    }
}

/// Grant 2, items 3 and 4: a candidate refused after its selected native Run (an
/// injected facade fault). The refused proof binds to its owner (U2, failure path);
/// the case is unavailable at the facade with the actual selected Run.
#[test]
fn u1g2_candidate_refusal_after_selected_run() {
    use super::retained_receipt::TraceFault as F;
    let mode = PreviewSolverMode::SparseInteractive;
    for (fault, error) in [(F::Maxima, "abandoned"), (F::ValuesCompletion, "values")] {
        let (refused, capture) = refused_candidate(mode, fault);
        let successor = wire::serialize_unavailable(wire::Refused::Candidate(&refused), &capture).unwrap_or_else(|f| panic!("{fault:?}: {f:?}"));
        let b = body(&successor);
        let (case, attempt) = (&b["cases"][0], &b["product_attempts"][0]);
        assert_eq!(attempt["result"]["error"]["kind"].as_str(), Some(error), "{fault:?}");
        assert_eq!((&case["reason"]["code"], &case["reason"]["phase"]), (&json!("facade_certificate"), &json!("facade")));
        assert_eq!(case["run"]["kernel_terminal"], json!({"kind":"selected","reason":null}));
        assert_eq!((&case["source_ref"], &attempt["source_ref"], &attempt["run_ref"]), (&json!(0), &json!(0), &json!(0)));
        assert_eq!(b["sources"].as_array().map(Vec::len), Some(1));
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
    let (mut first, capture) = refused_candidate(mode, F::ValuesCompletion);
    let (mut second, _) = refused_candidate(mode, F::ValuesCompletion);
    assert!(wire::serialize_unavailable(wire::Refused::Candidate(&first), &capture).is_ok(), "own refused proof serializes");
    first.test_swap_error(&mut second);
    let refused = wire::serialize_unavailable(wire::Refused::Candidate(&first), &capture);
    println!("I61_FOREIGN_OWNER failure_path_serializer_refused={}", refused.is_err());
    assert_eq!(refused.err(), Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "product_attempts[].proof.owner" }));
}

/// C1 §1 item 2 (grant 2): a failed verification keeps its translated failure
/// reason; a completed one has none. The milestone's records, with the
/// verification outcome replaced (a projection control, not a native run).
#[test]
fn u1g2_failed_verification_keeps_its_reason() {
    let (candidate, _, _) = milestone_candidate(PreviewSolverMode::SparseInteractive);
    let k::ExecutionOutcome::Selected(owner) = &candidate.capture().native.as_ref().unwrap().1.outcome else { panic!("selected") };
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
    let outcome = &candidate.capture().native.as_ref().unwrap().1.outcome;
    let k::ExecutionOutcome::Selected(owner) = outcome else { panic!("selected") };
    let records = owner.evidence().attempts.clone();
    assert_eq!(wire::test_kernel_terminal(outcome), (records.len(), json!({"kind":"selected","reason":null})));
    let unresolved = k::ExecutionOutcome::Unresolved { reason: k::UnresolvedReason::Ceiling, attempts: records.clone(), geometry: Vec::new() };
    assert_eq!(wire::test_kernel_terminal(&unresolved), (records.len(), json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}})));
    let refused = k::ExecutionOutcome::Refused { refusal: k::Refusal::Structure, attempts: Vec::new(), geometry: Vec::new() };
    assert_eq!(wire::test_kernel_terminal(&refused), (0, json!({"kind":"refused","reason":{"space":"refusal","tag":"structure"}})));
}

/// C2/C3: a source-constructor refusal needs `source_decline`, whose constructor
/// counts are not captured typed in grant 1-2, so it fails closed (typed).
#[test]
fn u1g2_source_constructor_refusal_fails_closed() {
    use super::retained_receipt::TraceFault as F;
    let mode = PreviewSolverMode::SparseInteractive;
    let (request, capture) = source_receipt::CapturedInvocation::parse(milestone_raw(), mode).unwrap();
    let mut observer = rp::ProductCapture::prepared_probe();
    let ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
    observer.trace_fault = Some(F::SourceConstruction);
    let failed = match observer.prepare_case(ordinary) { Err(f) => f, Ok(_) => panic!("source refusal expected") };
    assert!(matches!(failed.capture.error, Some(rp::CaptureError::Source(_))), "precondition: {:?}", failed.capture.error);
    assert_eq!(wire::serialize_unavailable(wire::Refused::Preparation(&failed), &capture).err(),
        Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Untranslated, field_path: "cases[].source_decline" }));
}

/// U2 on the failure path, certified proof of a commit refusal: the proof survives
/// an atomic-commit accounting refusal (as i51's control); a foreign one is refused.
#[test]
fn u2_failure_path_foreign_certificate_refused() {
    use rp::AdapterEvent as E;
    let mode = PreviewSolverMode::SparseInteractive;
    let commit_refusal = || {
        let (request, capture) = source_receipt::CapturedInvocation::parse(milestone_raw(), mode).unwrap();
        let prepared_case = || {
            let mut p = rp::PreparedCase::prepare_observed(request.clone(), mode, &capture).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
            p.solve_native().unwrap();
            p
        };
        let first = prepared_case();
        let before = first.capture().adapter.counts.get()[E::MapWrite as usize];
        let candidate = match first.project_candidate() { Ok(c) => c, Err(e) => panic!("{:?}", e.error) };
        let writes = candidate.capture().adapter.counts.get()[E::MapWrite as usize] - before;
        let prepared = prepared_case();
        let mut counts = prepared.capture().adapter.counts.get();
        counts[E::MapWrite as usize] = u64::MAX - (writes - 1);
        prepared.capture().adapter.counts.set(counts);
        let refusal = match prepared.project_candidate() { Err(r) => r, Ok(_) => panic!("commit refusal expected") };
        assert!(refusal.certificate().is_some() && refusal.proof_failure().is_none(), "precondition: certified proof survives");
        (refusal, capture)
    };
    let (mut first, capture) = commit_refusal();
    let (mut second, _) = commit_refusal();
    first.test_swap_certificate(&mut second);
    let refused = wire::serialize_unavailable(wire::Refused::Candidate(&first), &capture);
    assert_eq!(refused.err(), Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "product_attempts[].proof.owner" }));
}

/// RV82-S2: the supplied invocation binds structurally (its digest), not by mode
/// and case id alone: a same-mode invocation with another case label, project id
/// or load is refused before projection, on both serializer paths.
#[test]
fn u1g2_foreign_invocation_refused() {
    let mode = PreviewSolverMode::SparseInteractive;
    let (candidate, own, _) = milestone_candidate(mode);
    assert!(wire::serialize_selected(&candidate, &own).is_ok(), "own invocation serializes");
    let foreign = |edit: &dyn Fn(&mut Value)| {
        let mut raw = milestone_raw();
        edit(&mut raw);
        source_receipt::CapturedInvocation::parse(raw, mode).unwrap().1
    };
    let refused = Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "invocation" });
    for (label, edit) in [
        ("label", &(|r: &mut Value| r["model"]["load_cases"][0]["label"] = json!("another label")) as &dyn Fn(&mut Value)),
        ("project_id", &|r: &mut Value| r["model"]["project"]["id"] = json!("another-project")),
        ("load", &|r: &mut Value| r["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(0.0049)),
    ] {
        let other = foreign(edit);
        assert_eq!(other.mode(), mode, "{label}: same mode");
        assert_ne!(other.borrowed_digest(), own.borrowed_digest(), "{label}: a different invocation");
        assert_eq!(wire::serialize_selected(&candidate, &other).err(), refused, "{label}");
    }
    let (refusal, own_refusal_invocation) = refused_candidate(mode, super::retained_receipt::TraceFault::ValuesCompletion);
    assert!(wire::serialize_unavailable(wire::Refused::Candidate(&refusal), &own_refusal_invocation).is_ok());
    let other = foreign(&|r: &mut Value| r["model"]["load_cases"][0]["label"] = json!("another label"));
    assert_eq!(wire::serialize_unavailable(wire::Refused::Candidate(&refusal), &other).err(), refused);
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
    let (inv, case) = sparse.capture().native.as_ref().unwrap();
    let run = &inv.runs()[case.run];
    assert!(wire::test_invocation_arrays(inv, run).is_ok(), "own run");
    // An invocation whose meter never charged this run (same policy limit).
    let capacity = k::OriginCapacity::for_calls(&[1], &[]).unwrap();
    let fresh = k::RecordedInvocation::new(wire::INVOCATION_LIMIT, capacity).unwrap();
    assert_eq!(fresh.meter().checked_charged().exact(), Ok(0));
    assert_eq!(wire::test_invocation_arrays(&fresh, run).err(),
        Some(wire::ReceiptFailure { check: wire::ReceiptCheck::WorkCounterInconsistent, field_path: "work.charged" }), "R09");
}

/// RV82-N1 (R10, R14): T1 (a)'s guards on a doctored envelope: another legacy
/// disclosure still naming the selected case (G4), and an omitted id whose
/// diagnostic is not the legacy disclosure.
#[test]
fn u1g2_t1a_guard_negatives() {
    let (candidate, _, _) = milestone_candidate(PreviewSolverMode::SparseInteractive);
    let base = serde_json::to_value(candidate.envelope()).unwrap();
    let legacy_id = "diagnostic:source-recovery:case";
    let mut own = base.clone();
    assert!(wire::test_successor_envelope(&mut own, "case", Some(legacy_id)).is_ok(), "precondition: the actual envelope");
    // R10: a second legacy disclosure naming the selected case survives the omission.
    let mut doubled = base.clone();
    let mut copy = doubled["diagnostics"].as_array().unwrap().iter().find(|d| d["id"] == legacy_id).unwrap().clone();
    copy["id"] = json!("diagnostic:source-recovery:case:second");
    doubled["diagnostics"].as_array_mut().unwrap().push(copy);
    assert_eq!(wire::test_successor_envelope(&mut doubled, "case", Some(legacy_id)).err(),
        Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "diagnostics[legacy]" }), "R10");
    // R14: the captured id resolves to a diagnostic naming the case with another code.
    let mut recoded = base.clone();
    for d in recoded["diagnostics"].as_array_mut().unwrap() {
        if d["id"] == legacy_id { d["code"] = json!("NUMERICAL_INTEGRITY_SENSITIVE"); }
    }
    assert_eq!(wire::test_successor_envelope(&mut recoded, "case", Some(legacy_id)).err(),
        Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Association, field_path: "diagnostics[-legacy]" }), "R14");
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

/// RV82-N1 (R22): 2^53-1 itself is a safe JSON integer; only beyond it refuses.
#[test]
fn u1g2_safe_range_boundary() {
    let max_safe = (1usize << 53) - 1;
    let at = rp::LegacyWork { stage: "solve", helper_stage: AttemptStage::Solve, charged: max_safe, rejected: max_safe, limit: max_safe };
    let mut work = Vec::new();
    assert!(wire::test_legacy_source(Some(&rp::LegacySeed::Unavailable { work: at, diagnostic_ref: "d".into() }), 0, &mut work).is_ok(), "R22: 2^53-1 is in range");
    assert_eq!(work[0]["charged"], json!(max_safe as u64));
    let beyond = rp::LegacyWork { charged: max_safe + 1, ..at };
    assert_eq!(wire::test_legacy_source(Some(&rp::LegacySeed::Unavailable { work: beyond, diagnostic_ref: "d".into() }), 0, &mut Vec::new()).err().map(|f| f.check),
        Some(wire::ReceiptCheck::WorkCounterRange));
}

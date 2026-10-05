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

/// Protected byte control 1 (experiment 03 A, B, B'): capture never changes
/// the ordinary bytes, and the no-permit retained entry is the plain route.
#[test]
fn u1_ordinary_bytes_unchanged_under_capture() {
    for (mode, (name, len, digest)) in MODES.into_iter().zip(ORDINARY_SHA256) {
        assert_eq!(mode.as_str(), name);
        let plain = serde_json::to_vec(&run_linear_static_preview_value_with_mode(milestone_raw(), mode).unwrap()).unwrap();
        assert_eq!((plain.len(), sha(&plain).as_str()), (len, digest), "{name}: protected ordinary bytes");
        let direct = run_linear_static_preview_value_with_retained_direct(milestone_raw(), mode).unwrap();
        assert_eq!(serde_json::to_vec(direct.envelope()).unwrap(), plain, "{name}: A, no-permit direct entry");
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
/// in-process by the accepted Rust reader with eligibility off, with exact
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
        assert!(validation.invocation_bound && !validation.numerical_eligible, "{name}: eligibility off");
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
    // A second requested case is wider F2a (G-j): refused before any projection.
    let mut raw = milestone_raw();
    let mut second = raw["model"]["load_cases"][0].clone();
    second["id"] = json!("case-2");
    raw["model"]["load_cases"].as_array_mut().unwrap().push(second);
    let (_, two) = source_receipt::CapturedInvocation::parse(raw, mode).unwrap();
    assert_eq!(wire::serialize_selected(&candidate, &two).err(), Some(wire::ReceiptFailure { check: wire::ReceiptCheck::Scope, field_path: "cases" }));
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

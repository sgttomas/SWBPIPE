//! ROOT CP3 SF-1: a 0.4.0 invocation whose selected retained-source join
//! cannot finalize publishes every case on its ordinary route
//! (`load-reference-1`, no receipt), and selection first reserves the captured
//! replay's work. The two public-route scenarios are the independent CP3
//! reviewer's (budget cliff; composite with a sensitive pressure case). All
//! inputs are invented. Expected ordinary rows come from the uncaptured typed
//! route, which never attempts retained source.
//!
//! RF source-method design §5.5: the budget cliffs were tuned to the legacy
//! `Work::mul` precharge. Under the metered charging rule (HELP_HUMAN,
//! 2026-10-16) each scenario is re-derived from measured charges so that it
//! reaches the same path: public-limit scenarios are scaled (more declared
//! loads, more copies) within D1's limits, a private limit takes a new
//! private value, and N-3, which no longer reaches its cliff at the public
//! limit, is stated with a private limit beside a public-limit positive
//! control. The SF-1/SF-1R paths themselves are unchanged.
//!
//! R-A (DEL-04-08 Design `source-work-nondyadic-inputs.md` §4) lowers the source
//! route's evaluation work again (the committed witness's live join 310,217 ->
//! 307,339), so each cliff is re-derived by the same rule from measured charges.
//! Each scenario now sits on its cliff: its size or private limit is the first
//! at which its path is reached, and one unit below it (one load, moment or copy
//! fewer, or a limit one unit lower) a different path is taken, measured in both
//! modes. The limits themselves (8,000,000 and 64,000,000) are unchanged.
use super::*;

const CASE: &str = "case:join";

fn witness() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure_source/load_reference_source/eigen_motion.request.json"
    ))
    .unwrap()
}

/// The reviewer's probe P9: `extra` declared tip loads added to the committed
/// witness, cycling its three kinds (a 1e-6 N UZ force, a 1e-6 N UY force, a
/// copy of the witness's torque about RY). P9 used one to three; under the
/// metered rule with R-A a load adds about 3,300 to the live join (307,339
/// without any), so the scenario takes 1,111 to pass half of the 8M per-case
/// limit (measured, both modes: 1,110 loads 3,999,447, which passes the
/// screen; 1,111 4,002,051; 1,500 and 1,700 well past it).
fn with_extra_loads(mut request: Value, extra: usize) -> Value {
    let case = &mut request["model"]["load_cases"][0];
    for (i, dir) in ["UZ", "UY", "RY"].iter().cycle().take(extra).enumerate() {
        let mut load = case["primitive_loads"][0].clone();
        let id = format!("extra:{i}");
        load["id"] = json!(id);
        load["direction"] = json!(dir);
        if !dir.starts_with('R') {
            load["category"] = json!("concentrated_force");
            load["dimension"] = json!("force");
            load["magnitude"] = json!({"value": 1.0e-6, "unit": "N"});
        }
        case["primitive_loads"].as_array_mut().unwrap().push(load);
        case["analysis_state"]["load_sources"]
            .as_array_mut()
            .unwrap()
            .push(json!({"source_ref": id, "factor": 1.0}));
    }
    request
}

/// The reviewer's probe P3: a second, sensitive case carrying a 1e-5 Pa
/// pressure region, which retained source refuses.
fn with_sensitive_pressure_case(mut request: Value) -> Value {
    let mut case = request["model"]["load_cases"][0].clone();
    case["id"] = json!("case:pressure");
    case["primitive_loads"][0]["id"] = json!("torque:case:pressure");
    case["analysis_state"]["load_sources"][0]["source_ref"] = json!("torque:case:pressure");
    let p = json!("invented_load_reference_fallback_control_not_library_data");
    case["pressure_regions"] = json!([{"id": "region:fallback", "member_pipe_ids": ["member"],
        "pressure_basis": "internal_differential_zero_external_v1", "pressure": {"value": 1.0e-5, "unit": "Pa"},
        "terminals": [{"node_ref": "root", "closure_transfer": "transfers_to_wall", "provenance": p},
            {"node_ref": "tip", "closure_transfer": "transfers_to_wall", "provenance": p}], "provenance": p}]);
    request["model"]["load_cases"]
        .as_array_mut()
        .unwrap()
        .push(case);
    request
}

fn rows(envelope: &MechanicsEnvelope) -> Vec<(String, String, u64, String)> {
    envelope
        .results
        .iter()
        .map(|r| {
            (
                r.id.clone(),
                r.kind.clone(),
                r.value.to_bits(),
                r.unit.clone(),
            )
        })
        .collect()
}

fn diagnostics_for<'a>(
    envelope: &'a MechanicsEnvelope,
    code: &str,
    case: &str,
) -> Vec<&'a Diagnostic> {
    envelope
        .diagnostics
        .iter()
        .filter(|d| d.code == code && d.affected_refs.iter().any(|r| r == case))
        .collect()
}

/// Everything the fallback promises, for an envelope whose cases all have a
/// successful ordinary solve: the ordinary publication, bit for bit.
fn assert_ordinary_publication(
    envelope: &MechanicsEnvelope,
    request: &Value,
    mode: PreviewSolverMode,
    cases: &[&str],
) {
    assert_eq!(
        envelope.producer.semantic_contract_id,
        exact_admission::PRESSURE_SEMANTIC_CONTRACT_ID
    );
    assert_eq!(
        envelope.formulation_basis.profile_id,
        exact_admission::PRESSURE_PROFILE_ID
    );
    assert!(
        envelope.source_block_recovery.is_none(),
        "no receipt on the ordinary route"
    );
    assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED");
    let blocking: Vec<_> = envelope
        .diagnostics
        .iter()
        .filter(|d| d.severity == "blocking")
        .collect();
    assert!(blocking.is_empty(), "{blocking:?}");
    assert!(!envelope
        .diagnostics
        .iter()
        .any(|d| d.code == "SOURCE_BLOCK_RECOVERY_SELECTED"));
    // Numerical qualification, not the join, governs Current eligibility.
    assert_eq!(
        envelope.numerical_quality.status,
        NumericalQualityStatus::Sensitive
    );
    let typed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let ordinary = run_linear_static_preview_with_mode(typed, mode);
    assert_eq!(
        ordinary.producer.semantic_contract_id,
        exact_admission::PRESSURE_SEMANTIC_CONTRACT_ID
    );
    assert!(!ordinary.results.is_empty());
    assert_eq!(
        rows(envelope),
        rows(&ordinary),
        "fallback rows are the ordinary route's rows"
    );
    let records = envelope.contract_evidence.as_ref().unwrap()["load_reference_states"]
        .as_array()
        .unwrap()
        .clone();
    assert_eq!(records.len(), cases.len());
    for (record, case) in records.iter().zip(cases) {
        assert_eq!(record["load_case_id"], json!(case));
        assert_eq!(record["source_recovery"]["status"], "not_joined");
        let ordinary_method = if mode == PreviewSolverMode::DenseScrutiny {
            "ordinary_dense_structural_v1"
        } else {
            "ordinary_sparse_structural_v1"
        };
        assert_eq!(record["solve"]["recovery_method"], ordinary_method);
        let not_joined = diagnostics_for(envelope, case_state::SOURCE_RECOVERY_NOT_JOINED, case);
        assert_eq!(not_joined.len(), 1, "{case}");
        assert!(
            not_joined[0]
                .message
                .ends_with("retained_source_attempt=unavailable"),
            "{}",
            not_joined[0].message
        );
        assert_eq!(
            diagnostics_for(envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", case).len(),
            1,
            "{case}"
        );
    }
    for exact in envelope.contract_evidence.as_ref().unwrap()["exact_cases"]
        .as_array()
        .unwrap()
    {
        assert_ne!(exact["recovery_method"], "retained_source_blocks_exact_v1");
    }
}

#[test]
fn reviewer_scenario_a_budget_cliff_publishes_the_ordinary_route_instead_of_blocking() {
    // Re-derived for the metered rule: scaled from P9's 1..=3 extra loads to
    // 1,300/1,500/1,700, which put the live join past half of the public 8M
    // limit as P9's did under the legacy charges. Re-derived for R-A: the
    // cliff is 1,111 loads (live join 4,002,051 > 4,000,000); at 1,110 the
    // join (3,999,447) passes the screen and the case falls back later, at its
    // composite recipe reservation, without this decline (both modes).
    for extra in [1_111, 1_500, 1_700] {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            let request = with_extra_loads(witness(), extra);
            let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode)
                .unwrap_or_else(|e| panic!("extra={extra} {mode:?}: {e}"));
            assert_ordinary_publication(&envelope, &request, mode, &[CASE]);
            // The replay reservation declines the case before selection; no
            // finalization was attempted, so no fallback republication ran.
            let unavailable =
                &diagnostics_for(&envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", CASE)[0].message;
            assert!(
                unavailable.contains("captured replay reservation"),
                "{unavailable}"
            );
            assert!(unavailable.contains("Exact(Budget)"), "{unavailable}");
            assert!(unavailable.contains("limit: 8000000 }"), "the public per-case limit: {unavailable}");
            assert!(!unavailable.contains("could not finalize"), "{unavailable}");
        }
    }
}

#[test]
fn reviewer_scenario_b_composite_with_sensitive_pressure_case_publishes_the_ordinary_route() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let request = with_sensitive_pressure_case(witness());
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode)
            .unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_ordinary_publication(&envelope, &request, mode, &[CASE, "case:pressure"]);
        // The first case's successful attempt is declined with the recorded
        // invocation cause; the pressure case keeps its own refusal.
        let joined =
            &diagnostics_for(&envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", CASE)[0].message;
        assert!(joined.contains("invocation join withheld"), "{joined}");
        assert!(
            joined.contains("its selected join could not finalize: invocation receipt:"),
            "{joined}"
        );
        let pressure = &diagnostics_for(
            &envelope,
            "SOURCE_BLOCK_RECOVERY_UNAVAILABLE",
            "case:pressure",
        )[0]
        .message;
        assert!(!pressure.contains("invocation join withheld"), "{pressure}");
        assert!(
            pressure.contains("could not finalize"),
            "the withheld cause names the invocation: {pressure}"
        );
    }
}

#[test]
fn a_selected_case_whose_own_finalization_fails_after_the_reservation_falls_back() {
    // With a private case limit the committed witness's live join passes the
    // replay reservation (c <= L - c), but its complete finalization does not
    // fit. Private limit only: it is not public policy and never changes a
    // numerical criterion. (Under the legacy charges the limit was 7.0M: join
    // about 3.36M, finalization about 7.34M; under the metered rule 1.0M: join
    // 310,217, finalization 1,264,873 before U4b.) Re-derived for R-A: the join
    // charges 307,339 and the complete case 1,295,997, so the path is
    // reached for 614,678 <= L <= 1,295,996. The limit is the lower edge,
    // 2 x 307,339: at 614,677 the replay reservation refuses the case and no
    // finalization is attempted (both modes, measured).
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let request = witness();
        let (typed, capture) = CapturedInvocation::parse(request.clone(), mode).unwrap();
        let mut budget = SourceRecoveryBudget {
            per_case_limit: 614_678,
            ..Default::default()
        };
        let envelope = run_linear_static_preview_captured(typed, mode, Some(&capture), &mut budget);
        assert_ordinary_publication(&envelope, &request, mode, &[CASE]);
        let cause = budget
            .join_withheld
            .clone()
            .expect("fallback ran");
        assert!(cause.starts_with("case case:join: "), "{cause}");
        let message =
            &diagnostics_for(&envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", CASE)[0].message;
        assert!(
            message.contains("invocation join withheld") && message.contains(&cause),
            "{message}"
        );
        // The republication continues the same ledger (CP4 review N-2):
        // both attempts stay charged, nothing is refunded, and the whole
        // invocation stays within the one invocation limit.
        assert_eq!(budget.attempts, 2);
        assert!(budget.charged > budget.per_case_limit);
        assert!(budget.charged == budget.failed_charged);
        assert!(budget.charged <= budget.invocation_limit);
    }
}

#[test]
fn the_committed_witness_still_selects_and_finalizes_within_the_public_limit() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let envelope = run_linear_static_preview_value_with_mode(witness(), mode).unwrap();
        assert_eq!(
            envelope.producer.semantic_contract_id,
            exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID
        );
        assert_eq!(
            envelope.source_block_recovery.as_ref().unwrap()["body"]["status"],
            "qualified"
        );
        assert!(!envelope
            .diagnostics
            .iter()
            .any(|d| d.code == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"));
    }
}

/// The committed `physics_source/` request `name` (pre-0.4, exact route).
fn physics_source(name: &str) -> Value {
    let text = match name {
        "mixed" => include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/mixed.request.json"),
        "n05" => include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"),
        "n06" => include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n06.request.json"),
        other => panic!("{other}"),
    };
    serde_json::from_str(text).unwrap()
}

/// The committed `physics_source/mixed` request with case 1's
/// `modulus_basis_ref` removed: case 1 then needs recovery, which declines it
/// ("exact source recovery requires explicitly empty pressure regions") while
/// case 0 selects, so the invocation receipt fails (a composite requires every
/// case to qualify; design §1.2).
pub(crate) fn pre04_composite_failure_request() -> Value {
    let mut request = physics_source("mixed");
    request["model"]["load_cases"][1]
        .as_object_mut()
        .unwrap()
        .remove("modulus_basis_ref");
    request
}

/// The withheld cause's text, appended to a declined case's
/// `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic in a republication (0.4's text,
/// byte for byte, on both routes).
pub(crate) const WITHHELD: &str = "; the invocation publishes every case on its ordinary route because its selected join could not finalize: ";

/// Pre-0.4 failure unification (DEL-04-08 Design `pre04-failure-unification.md`
/// §2.1): what a pre-0.4 republication promises for an invocation whose cases
/// all have a successful ordinary solve. The ordinary `pressure-2` publication,
/// no receipt, `MECHANICS_SOLVED`, no blocking diagnostic and nothing selected,
/// with rows (id, kind, value bits, unit) equal to the uncaptured typed route's,
/// which never attempts retained source.
pub(crate) fn assert_pre04_republication(envelope: &MechanicsEnvelope, request: &Value, mode: PreviewSolverMode) {
    assert_eq!(envelope.producer.semantic_contract_id, exact_admission::PRESSURE_SEMANTIC_CONTRACT_ID, "{mode:?}");
    assert_eq!(envelope.formulation_basis.profile_id, exact_admission::PRESSURE_PROFILE_ID, "{mode:?}");
    assert!(envelope.source_block_recovery.is_none(), "{mode:?}: no receipt on the ordinary route");
    assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED", "{mode:?}");
    let blocking: Vec<_> = envelope.diagnostics.iter().filter(|d| d.severity == "blocking").collect();
    assert!(blocking.is_empty(), "{mode:?}: {blocking:?}");
    assert!(!envelope.diagnostics.iter().any(|d| d.code == "SOURCE_BLOCK_RECOVERY_SELECTED"), "{mode:?}");
    for exact in envelope.contract_evidence.as_ref().unwrap()["exact_cases"].as_array().unwrap() {
        assert_ne!(exact["recovery_method"], "retained_source_blocks_exact_v1", "{mode:?}");
    }
    let typed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let ordinary = run_linear_static_preview_with_mode(typed, mode);
    assert_eq!(ordinary.producer.semantic_contract_id, exact_admission::PRESSURE_SEMANTIC_CONTRACT_ID);
    assert!(!ordinary.results.is_empty());
    assert_eq!(rows(envelope), rows(&ordinary), "{mode:?}: republished rows are the ordinary route's rows");
}

/// A case's one `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` message in a republication.
pub(crate) fn unavailable(envelope: &MechanicsEnvelope, case: &str) -> String {
    let found = diagnostics_for(envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", case);
    assert_eq!(found.len(), 1, "{case}: {found:?}");
    found[0].message.clone()
}

/// A private-limit run of `request` on the shared captured route, with the
/// ledger it ended on.
fn captured(request: &Value, mode: PreviewSolverMode, per_case_limit: usize) -> (MechanicsEnvelope, SourceRecoveryBudget) {
    let (typed, capture) = CapturedInvocation::parse(request.clone(), mode).unwrap();
    let mut budget = SourceRecoveryBudget { per_case_limit, ..Default::default() };
    let envelope = run_linear_static_preview_captured(typed, mode, Some(&capture), &mut budget);
    (envelope, budget)
}

/// The republication's ledger: two runs' attempts, nothing refunded, nothing
/// selected, within the one invocation limit (CP4 review N-2).
fn assert_republication_ledger(budget: &SourceRecoveryBudget, attempts: usize) {
    assert_eq!(budget.attempts, attempts);
    assert_eq!(budget.charged, budget.failed_charged);
    assert!(budget.charged <= budget.invocation_limit);
}

#[test]
fn a_pre04_composite_finalization_failure_republishes_ordinarily() {
    // Restated by the pre-0.4 failure unification (design §7). This test was
    // `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback`,
    // which asserted `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` for this S4
    // failure as a characterization, "not an endorsement", of an outcome left
    // open. That outcome does not meet REQ-003 (c) or AX-005, and the owner
    // commissioned one failure behaviour for both routes (b7-plan, "After
    // B7b"; "16 start now."). The new expectation is the outcome REQ-003 (c)
    // already states for 0.4: the public entry republishes every case on its
    // ordinary route. No limit, criterion or reference moves to obtain it.
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let request = pre04_composite_failure_request();
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode)
            .unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_pre04_republication(&envelope, &request, mode);
        // Case 0 selected in the first run; in the republication it is withheld
        // with the invocation receipt's cause. Case 1's own decline carries it too.
        let cause = "invocation receipt: composite requires source selection and complete case qualification";
        let selected = unavailable(&envelope, "case");
        assert!(selected.contains("stage: \"invocation join withheld\""), "{selected}");
        assert!(selected.ends_with(&format!("{WITHHELD}{cause}")), "{selected}");
        let declined = unavailable(&envelope, "case:ordinary-pressure");
        assert!(declined.contains("exact source recovery requires explicitly empty pressure regions"), "{declined}");
        assert!(declined.ends_with(&format!("{WITHHELD}{cause}")), "{declined}");
    }
}

#[test]
fn a_pre04_case_finalization_failure_republishes_ordinarily() {
    // Design §2.1 (S3, per case): the committed `physics_source/n05` request at a
    // private case limit of 2c = 357,072 (live charge c = 178,536 in both
    // modes) passes the replay screen, and its finalization does not fit: the
    // composite recipe reservation refuses (measured, both modes; at 357,071 the
    // screen declines). Private limit only: it is not public policy.
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let request = physics_source("n05");
        let (envelope, budget) = captured(&request, mode, 357_072);
        assert_pre04_republication(&envelope, &request, mode);
        let cause = budget.join_withheld.clone().expect("republished");
        assert!(cause.starts_with("case case: composite recipe reservation: "), "{cause}");
        assert!(cause.contains("Exact(Budget)"), "{cause}");
        let message = unavailable(&envelope, "case");
        assert!(message.contains("stage: \"invocation join withheld\""), "{message}");
        assert!(message.ends_with(&format!("{WITHHELD}{cause}")), "{message}");
        assert_republication_ledger(&budget, 2);
        // One unit lower, the screen declines and nothing republishes.
        let (envelope, budget) = captured(&request, mode, 357_071);
        assert_pre04_republication(&envelope, &request, mode);
        assert!(budget.join_withheld.is_none());
        let message = unavailable(&envelope, "case");
        assert!(message.contains("captured replay reservation") && !message.contains("could not finalize"), "{message}");
        assert_eq!(budget.attempts, 1);
    }
}

#[test]
fn a_republished_case_that_needed_no_recovery_states_no_cause() {
    // Design §2.1: in a republication, a case that needs no recovery and has no
    // load-row finding carries no `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic.
    // The committed `physics_source/mixed` request: case 0 (live charge 178,556)
    // selects and, at a private limit of 500,000, fails its own finalization
    // (S3); case 1 (`case:ordinary-pressure`) passes its ordinary checks.
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let request = physics_source("mixed");
        let (envelope, budget) = captured(&request, mode, 500_000);
        assert_pre04_republication(&envelope, &request, mode);
        let cause = budget.join_withheld.clone().expect("republished");
        assert!(cause.starts_with("case case: "), "{cause}");
        assert!(unavailable(&envelope, "case").ends_with(&format!("{WITHHELD}{cause}")));
        let ordinary: Vec<_> = envelope
            .diagnostics
            .iter()
            .filter(|d| d.affected_refs.iter().any(|r| r == "case:ordinary-pressure"))
            .filter(|d| d.code.starts_with("SOURCE_") || d.message.contains("could not finalize"))
            .collect();
        assert!(ordinary.is_empty(), "{mode:?}: {ordinary:?}");
        assert_republication_ledger(&budget, 2);
    }
}

#[test]
fn a_support_norm_refusal_is_the_stated_cause() {
    // Design §4: a support-norm recipe refusal (S2) is the case's finalization
    // cause, and the row binding is not consulted, so the missing support rows
    // never read as "not unique". The committed `physics_source/n05` request has
    // a live charge of c = 178,536 in both modes. With the replay screen skipped
    // for this thread (test builds only), private case limits from 178,536 to
    // 182,631 refuse both supports' norm reservations, and the first (`anchor`)
    // is the cause; from 182,632 to 186,727 only `independent-spring` refuses.
    // Below 178,536 the live attempt is declined; from 186,728 finalization
    // fails at a later stage (measured, both modes).
    let request = physics_source("n05");
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        for (limit, support) in [
            (178_536, Some("anchor")),
            (182_631, Some("anchor")),
            (182_632, Some("independent-spring")),
            (186_727, Some("independent-spring")),
            (186_728, None),
        ] {
            let label = format!("{mode:?} {limit}");
            let (envelope, budget) =
                crate::replay_screen_hook::skipping(|| captured(&request, mode, limit));
            assert_pre04_republication(&envelope, &request, mode);
            let cause = budget.join_withheld.clone().unwrap_or_else(|| panic!("{label}: republished"));
            match support {
                Some(support) => {
                    let stated = format!("case case: support {support} norm: composite recipe reservation: RecoveryFailure {{ stage: \"receipt finalization\"");
                    assert!(cause.starts_with(&stated), "{label}: {cause}");
                    assert!(cause.contains("Exact(Budget)"), "{label}: {cause}");
                }
                None => assert!(!cause.contains(" norm: "), "{label}: {cause}"),
            }
            assert!(unavailable(&envelope, "case").ends_with(&format!("{WITHHELD}{cause}")), "{label}");
            for d in &envelope.diagnostics {
                assert!(!d.message.contains("row binding is not unique") && !d.message.contains("row binding is missing"), "{label}: {d:?}");
            }
            assert_republication_ledger(&budget, 2);
        }
        // Below the window the live attempt itself is declined; with the screen in
        // place, every limit of the window is below 2c and the screen declines.
        let (_, budget) = crate::replay_screen_hook::skipping(|| captured(&request, mode, 178_535));
        assert!(budget.join_withheld.is_none() && budget.attempts == 1, "{mode:?}");
        let (envelope, budget) = captured(&request, mode, 182_632);
        assert!(budget.join_withheld.is_none() && budget.attempts == 1, "{mode:?}");
        assert!(unavailable(&envelope, "case").contains("captured replay reservation"), "{mode:?}");
    }
}

#[test]
fn a_republished_case_whose_ordinary_attempt_failed_keeps_its_blocked_result() {
    // Design §2.1 (REQ-003 (d)): the committed `physics_source/n06` request's
    // ordinary attempt is rejected ("NumericallyUnresolved … stabilization
    // unresolved"), and at the public limit a selection replaces it. Here a
    // private case limit of 357,071 (live charge c = 175,079 in both modes, so
    // the screen passes) forces its finalization to fail (S3). The
    // republication has no selection to replace the rejected attempt, so the
    // case's blocked ordinary result stands, exactly as the typed route
    // publishes it, and the case's diagnostic states the cause.
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let request = physics_source("n06");
        let (envelope, budget) = captured(&request, mode, 357_071);
        let typed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
        let ordinary = run_linear_static_preview_with_mode(typed, mode);
        assert_eq!(envelope.producer.semantic_contract_id, exact_admission::PRESSURE_SEMANTIC_CONTRACT_ID, "{mode:?}");
        assert!(envelope.source_block_recovery.is_none(), "{mode:?}");
        assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE", "{mode:?}");
        assert_eq!(envelope.status.mechanics, ordinary.status.mechanics, "{mode:?}");
        assert!(envelope.results.is_empty() && ordinary.results.is_empty(), "{mode:?}");
        let blocking = |e: &MechanicsEnvelope| -> Vec<(String, String)> {
            e.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| (d.code.clone(), d.message.clone())).collect()
        };
        assert_eq!(blocking(&envelope), blocking(&ordinary), "{mode:?}: the blocked ordinary result stands");
        assert_eq!(blocking(&envelope)[0].0, "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED", "{mode:?}");
        let cause = budget.join_withheld.clone().expect("republished");
        assert!(cause.starts_with("case case: "), "{cause}");
        assert!(unavailable(&envelope, "case").ends_with(&format!("{WITHHELD}{cause}")), "{mode:?}");
        assert_republication_ledger(&budget, 2);
    }
}

/// A declared load added to case `case` of the request, with its ledger entry.
fn add_load(request: &mut Value, case: usize, id: &str, dir: &str, value: f64) {
    let case = &mut request["model"]["load_cases"][case];
    let mut load = case["primitive_loads"][0].clone();
    load["id"] = json!(id);
    load["direction"] = json!(dir);
    load["target"] = json!({"type": "node", "node": "tip"});
    if dir.starts_with('R') {
        load["category"] = json!("concentrated_moment");
        load["dimension"] = json!("moment");
        load["magnitude"] = json!({"value": value, "unit": "N*m"});
    } else {
        load["category"] = json!("concentrated_force");
        load["dimension"] = json!("force");
        load["magnitude"] = json!({"value": value, "unit": "N"});
    }
    case["primitive_loads"].as_array_mut().unwrap().push(load);
    case["analysis_state"]["load_sources"]
        .as_array_mut()
        .unwrap()
        .push(json!({"source_ref": id, "factor": 1.0}));
}

/// Copies case 0 under a new ID, renaming its loads and ledger entries.
fn copy_case(request: &mut Value, id: &str) {
    let mut case = request["model"]["load_cases"][0].clone();
    case["id"] = json!(id);
    for load in case["primitive_loads"].as_array_mut().unwrap() {
        let renamed = format!("{}:{id}", load["id"].as_str().unwrap());
        load["id"] = json!(renamed);
    }
    for source in case["analysis_state"]["load_sources"]
        .as_array_mut()
        .unwrap()
    {
        let renamed = format!("{}:{id}", source["source_ref"].as_str().unwrap());
        source["source_ref"] = json!(renamed);
    }
    request["model"]["load_cases"]
        .as_array_mut()
        .unwrap()
        .push(case);
}

#[test]
fn a_join_that_passes_the_replay_screen_but_cannot_finalize_falls_back_at_the_public_limit() {
    // CP4 review SF-1R: declared 1e-6 N*m tip RY moments give a live charge
    // that passes the c <= L - c screen at the public 8M limit; the stages
    // charged before replay then leave replay short. ROOT's ruling (b): the
    // screen is not a guarantee; the fallback is. Re-derived for the metered
    // rule: one moment (live charge about 3.99M under the legacy charges) now
    // finalizes; 280 moments give a live charge of 1,306,279, and the
    // finalization stages before replay, which grow with the request, leave
    // replay short (measured: 260 to 300 moments reach this path).
    // Re-derived for R-A: 250 to 300 moments reach it (both modes), and the
    // scenario takes the lower edge. At 249 (live charge 1,196,386) the join
    // is withheld earlier, at the composite current source binding; at 250
    // (1,199,406) it reaches the captured source replay short.
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let mut request = witness();
        for i in 0..250 {
            add_load(&mut request, 0, &format!("extra:ry:{i}"), "RY", 1.0e-6);
        }
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode)
            .unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_ordinary_publication(&envelope, &request, mode, &[CASE]);
        let message =
            &diagnostics_for(&envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", CASE)[0].message;
        assert!(message.contains("invocation join withheld"), "{message}");
        assert!(
            message.contains("could not finalize: case case:join: captured source replay"),
            "{message}"
        );
        assert!(
            !message.contains("captured replay reservation"),
            "{message}"
        );
    }
}

#[test]
fn a_republication_reports_a_case_s_own_replay_screen_refusal() {
    // CP4 review N-3: in a republication case:join is withheld, while
    // case:heavy (one extra tip UZ force) still reports its own screen
    // refusal. Re-derived for the metered rule with a private 660,000 case
    // limit, between twice case:join's live charge (2 x 310,217) and twice
    // case:heavy's (2 x 348,916): case:join passes its screen, case:heavy
    // fails its own. Under the legacy charges this ran at the public limit
    // (join about 3.36M, heavy just over 4M), case:join finalized and the
    // composite receipt failed on case:heavy ("invocation receipt:"). Now
    // case:heavy's cliff is out of reach at the public limit within D1's
    // limits (adding loads grows case:join's finalization reservation, which
    // binds the whole request, faster than case:heavy's live charge), and at
    // 660,000 case:join's own finalization does not fit, so the recorded cause
    // is case:join's. Private limit only. The public-limit positive control is
    // `the_republication_scenario_selects_both_cases_at_the_public_limit`.
    // Re-derived for R-A: the live charges are 307,339 (case:join) and
    // 342,667 (case:heavy), so the path is reached for 614,678 <= L <= 685,333.
    // The limit is the lower edge, 2 x 307,339: at 614,677 case:join is
    // refused at its own screen too and no fallback runs (both modes,
    // measured); from 685,334 case:heavy passes its screen.
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let request = heavy_request();
        let (typed, capture) = CapturedInvocation::parse(request.clone(), mode).unwrap();
        let mut budget = SourceRecoveryBudget {
            per_case_limit: 614_678,
            ..Default::default()
        };
        let envelope = run_linear_static_preview_captured(typed, mode, Some(&capture), &mut budget);
        assert_ordinary_publication(&envelope, &request, mode, &[CASE, "case:heavy"]);
        let cause = budget
            .join_withheld
            .clone()
            .expect("fallback ran");
        assert!(cause.starts_with("case case:join: "), "{cause}");
        let joined =
            &diagnostics_for(&envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", CASE)[0].message;
        assert!(joined.contains("invocation join withheld"), "{joined}");
        assert!(joined.contains(&format!("could not finalize: {cause}")), "{joined}");
        let heavy = &diagnostics_for(&envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", "case:heavy")
            [0]
        .message;
        assert!(heavy.contains("captured replay reservation"), "{heavy}");
        assert!(!heavy.contains("invocation join withheld"), "{heavy}");
        assert!(budget.charged <= budget.invocation_limit);
    }
}

/// N-3's request: the witness plus case:heavy, a copy with one extra tip UZ
/// force.
fn heavy_request() -> Value {
    let mut request = witness();
    copy_case(&mut request, "case:heavy");
    add_load(&mut request, 1, "extra:heavy", "UZ", 1.0e-6);
    request
}

/// N-3's positive control (RF design §5.5): at the public limit both cases of
/// the republication scenario now select and the composite receipt finalizes.
#[test]
fn the_republication_scenario_selects_both_cases_at_the_public_limit() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let envelope = run_linear_static_preview_value_with_mode(heavy_request(), mode).unwrap();
        assert_eq!(
            envelope.producer.semantic_contract_id,
            exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID
        );
        let receipt = envelope.source_block_recovery.as_ref().unwrap();
        assert_eq!(receipt["body"]["status"], "qualified");
        assert_eq!(receipt["body"]["cases"].as_array().unwrap().len(), 2);
        assert!(!envelope
            .diagnostics
            .iter()
            .any(|d| d.code == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"));
        for case in [CASE, "case:heavy"] {
            assert_eq!(
                diagnostics_for(&envelope, "SOURCE_BLOCK_RECOVERY_SELECTED", case).len(),
                1,
                "{case}"
            );
        }
    }
}

#[test]
fn a_fallback_after_invocation_limit_exhaustion_stays_within_the_invocation_limit() {
    // CP4 review N-2: selectable copies exhaust the 64M invocation limit
    // during the first run. The republication continues the same ledger, so
    // total executed work never exceeds that one limit. Re-derived for the
    // metered rule: N-2's ten copies (about 7.36M each under the legacy
    // charges) now fit; each copy charges about 1.64M plus its share of the
    // publication, so forty copies exhaust the limit (measured: the first run
    // fails at case:copy22's finalization with a budget refusal). Re-derived
    // for R-A: twenty copies now select and finalize within the limit; from
    // 21 copies the first run exhausts it, but up to 26 it fails at the
    // invocation receipt's composite publication reservation, not at a case.
    // From 27 copies the first run fails at a case's finalization with a
    // budget refusal (case:copy26's captured source replay), which is this
    // scenario, so it takes 27; at 26 the recorded cause names no case budget
    // refusal (measured, dense). Above 27 the failing stage varies with the
    // copy count (29 and 31 fail at the invocation publication reservation).
    const COPIES: usize = 27;
    let mut request = witness();
    for index in 1..COPIES {
        copy_case(&mut request, &format!("case:copy{index}"));
    }
    let cases: Vec<String> = std::iter::once(CASE.to_string())
        .chain((1..COPIES).map(|index| format!("case:copy{index}")))
        .collect();
    let case_refs: Vec<&str> = cases.iter().map(String::as_str).collect();
    let mode = PreviewSolverMode::DenseScrutiny;
    let (typed, capture) = CapturedInvocation::parse(request.clone(), mode).unwrap();
    let mut budget = SourceRecoveryBudget {
        per_case_limit: PHYSICS_SOURCE_WORK_LIMIT,
        ..Default::default()
    };
    let envelope = run_linear_static_preview_captured(typed, mode, Some(&capture), &mut budget);
    assert_ordinary_publication(&envelope, &request, mode, &case_refs);
    let cause = budget.join_withheld.clone().expect("fallback ran");
    assert!(cause.contains("Exact(Budget)"), "{cause}");
    // The first run attempts the cases up to the one whose finalization the
    // exhausted ledger refuses; the republication attempts every case (each
    // reports its own decline above). All are counted in the one continued
    // ledger, which ends within one per-case limit of the invocation limit.
    assert!(budget.attempts > COPIES && budget.attempts <= 2 * COPIES, "{}", budget.attempts);
    assert!(budget.charged <= budget.invocation_limit);
    assert!(budget.charged > budget.invocation_limit - budget.per_case_limit, "{}", budget.charged);
    assert_eq!(budget.invocation_limit, SOURCE_BLOCKS_INVOCATION_WORK_LIMIT);
}


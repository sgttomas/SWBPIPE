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
            .load_state_join_withheld
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

#[test]
fn the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback() {
    // Characterization only, not an endorsement: the inherited composite
    // outcome (physics-source-1 before B7b; its current pressure-source-1
    // twin since) is a separate open finding assigned to T3. This slice must
    // not change behaviour without the group (reviewer probe P12).
    let mut request: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure_source/physics_source/mixed.request.json"
    ))
    .unwrap();
    request["model"]["load_cases"][1]
        .as_object_mut()
        .unwrap()
        .remove("modulus_basis_ref");
    let outcome =
        run_linear_static_preview_value_with_mode(request, PreviewSolverMode::DenseScrutiny);
    assert_eq!(
        outcome.err().as_deref(),
        Some("SOURCE_BLOCKS_FINALIZATION_FAILED")
    );
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
            .load_state_join_withheld
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
    let cause = budget.load_state_join_withheld.clone().expect("fallback ran");
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

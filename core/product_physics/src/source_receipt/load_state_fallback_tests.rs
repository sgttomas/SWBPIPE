//! ROOT CP3 SF-1: a 0.4.0 invocation whose selected retained-source join
//! cannot finalize publishes every case on its ordinary route
//! (`load-reference-1`, no receipt), and selection first reserves the captured
//! replay's work. The two public-route scenarios are the independent CP3
//! reviewer's (budget cliff; composite with a sensitive pressure case). All
//! inputs are invented. Expected ordinary rows come from the uncaptured typed
//! route, which never attempts retained source.
use super::*;

const CASE: &str = "case:join";

fn witness() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/load_reference_source/eigen_motion.request.json"
    ))
    .unwrap()
}

/// The reviewer's probe P9: `extra` declared 1e-6 N tip forces (then a moment)
/// added to the committed witness. They raise the live join work past half of
/// the 8M per-case limit.
fn with_extra_loads(mut request: Value, extra: usize) -> Value {
    let case = &mut request["model"]["load_cases"][0];
    for (i, dir) in ["UZ", "UY", "RY"].iter().take(extra).enumerate() {
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
        case_state::LOAD_REFERENCE_SEMANTIC_CONTRACT_ID
    );
    assert_eq!(
        envelope.formulation_basis.profile_id,
        case_state::LOAD_STATE_PROFILE_ID
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
        case_state::LOAD_REFERENCE_SEMANTIC_CONTRACT_ID
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
    for extra in 1..=3 {
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
    // With a private 7.0M case limit the committed witness's live join
    // (about 3.36M) passes the replay reservation, but its complete
    // finalization (about 7.34M) does not fit. Private limit only: it is not
    // public policy and never changes a numerical criterion.
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let request = witness();
        let (typed, capture) = CapturedInvocation::parse(request.clone(), mode).unwrap();
        let mut budget = SourceRecoveryBudget {
            per_case_limit: 7_000_000,
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
        // The fallback is a separate run: its own ledger, never a refund.
        assert_eq!(budget.attempts, 1);
        assert!(budget.charged > 0 && budget.charged == budget.failed_charged);
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
            case_state::LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID
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
    // Characterization only, not an endorsement: the inherited physics-source-1
    // composite outcome is a separate open finding assigned to T3. This slice
    // must not change pre-0.4 behaviour (reviewer probe P12).
    let mut request: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/physics_source/mixed.request.json"
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

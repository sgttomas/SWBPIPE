//! DEL-10-05 benchmark/regression payload bindings for the DEC-065 CLI.
//!
//! This bounded module binds the `run-benchmark` and `run-regression` verbs to
//! the existing suite crates (`validation/benchmarks/mechanics` = DEL-09-01,
//! `validation/benchmarks/stress` = DEL-09-02,
//! `validation/benchmarks/nonlinear` = DEL-09-03). The suite crates remain the
//! single source of fixture identity, recorded comparison values, and encoded
//! comparison predicates: per-case comparison reuses each crate's own
//! `recorded_comparison_holds` accessor, its public fixture-specific
//! comparison accessor, its public `validate_*` predicates, or (nonlinear)
//! `NonlinearRegressionCase::matches_expected_outcome`.
//!
//! No new numeric tolerance constant and no release-threshold vocabulary is
//! introduced here. A requested case whose recorded comparison basis cannot be
//! reused through the suite crate's public value-addressable surface fails
//! closed with a structured blocking diagnostic instead of receiving an
//! invented tolerance. Per-case match/fail is regression evidence for current
//! solver behavior only; release thresholds, final tolerance policy, CI gate
//! policy, and professional reliance remain `TBD` (owner-gated; DEC-046
//! promotion untouched).

use std::panic::{catch_unwind, AssertUnwindSafe};

use serde::Serialize;

use crate::{Diagnostic, Reference};

use open_pipe_stress_mechanics_benchmarks as mechanics;
use open_pipe_stress_nonlinear_benchmarks as nonlinear;
use open_pipe_stress_stress_benchmarks as stress;

/// The suites' recorded claim posture, restated verbatim in every suite-run
/// report so per-case match/fail is never read as a release or acceptance
/// claim.
pub const REGRESSION_EVIDENCE_CLAIM_POSTURE: &str =
    "Per-case match/fail is regression evidence for current solver behavior \
     against the suite crate's recorded comparison values only; release \
     thresholds, final tolerance policy, CI gate policy, and professional \
     reliance remain TBD pending human approval (DEC-046 promotion is \
     owner-gated and untouched).";

const MECHANICS_COMPARISON_BASIS: &str =
    "recorded expected values and encoded comparison predicates reused from \
     validation/benchmarks/mechanics (DEL-09-01): recorded_comparison_holds \
     internal-assertion basis plus public validate_* predicates";
const STRESS_COMPARISON_BASIS: &str =
    "recorded expected values and encoded comparison predicate reused from \
     validation/benchmarks/stress (DEL-09-02): recorded_comparison_holds \
     internal-assertion basis over public recover_* observed values";
const NONLINEAR_COMPARISON_BASIS: &str =
    "recorded expected states, changed-support sets, residual records, and \
     diagnostics reused from validation/benchmarks/nonlinear (DEL-09-03) \
     through NonlinearRegressionCase::run and matches_expected_outcome \
     (exact-equality basis as encoded by the suite crate)";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CaseStatus {
    ExecutedAndMatched,
    ExecutedAndMismatched,
    Blocked,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValueComparison {
    pub name: String,
    pub unit: String,
    pub dimension: String,
    pub recorded: f64,
    pub observed: Option<f64>,
    pub delta: Option<f64>,
    pub within_recorded_basis: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SupportStateReport {
    pub support_id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegressionCaseDetail {
    pub recorded_states: Vec<SupportStateReport>,
    pub observed_states: Vec<SupportStateReport>,
    pub recorded_changed_supports: Vec<String>,
    pub observed_changed_supports: Vec<String>,
    pub recorded_converged: bool,
    pub observed_converged: Option<bool>,
    pub recorded_diagnostic_codes: Vec<String>,
    pub observed_diagnostic_codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CaseReport {
    pub fixture_id: String,
    pub family: String,
    pub status: CaseStatus,
    pub encoded_predicate: Option<&'static str>,
    pub encoded_predicate_result: Option<bool>,
    pub values: Vec<ValueComparison>,
    pub regression_detail: Option<RegressionCaseDetail>,
    pub block_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SuiteRunReport {
    pub suite: String,
    pub suite_deliverable: &'static str,
    pub claim_posture: &'static str,
    pub comparison_basis: &'static str,
    pub whole_suite_default_applied: bool,
    pub requested_case_count: usize,
    pub executed_and_matched: usize,
    pub executed_and_mismatched: usize,
    pub blocked: usize,
    pub cases: Vec<CaseReport>,
}

#[derive(Debug)]
pub struct SuiteRunOutcome {
    pub report: Option<SuiteRunReport>,
    pub diagnostics: Vec<Diagnostic>,
}

/// One evaluated (not yet reported) case: observed values keyed exactly by the
/// suite crate's recorded value names, plus the crate's encoded per-case
/// predicate where one is public.
struct EvaluatedCase {
    observed: Vec<(&'static str, f64)>,
    predicate: Option<(&'static str, bool)>,
}

enum CaseEvaluation {
    Evaluated(EvaluatedCase),
    SuiteEvaluated(Vec<(&'static str, f64)>),
    NotReusable(String),
    ExecutionFailed(String),
}

fn blocking(code: &str, ref_type: &str, ref_id: &str, message: String) -> Diagnostic {
    Diagnostic::runner_blocking(code, Reference::new(ref_type, ref_id), message)
}

fn requested_ids(cases: &[String], inventory_ids: &[String]) -> (Vec<String>, bool) {
    if cases.is_empty() {
        (inventory_ids.to_vec(), true)
    } else {
        (cases.to_vec(), false)
    }
}

fn not_reusable_reason(missing: &str) -> String {
    format!(
        "the suite crate does not expose a public value-addressable observed \
         counterpart for {missing}; its encoded verification for this case \
         lives in crate-internal wiring, so the recorded comparison basis \
         cannot be reused through the runner without re-encoding it. The case \
         fails closed rather than receive an invented tolerance or a new \
         comparison surface."
    )
}

fn compare_case(
    fixture_id: &str,
    family: String,
    expected: &[(String, String, String, f64)],
    evaluation: CaseEvaluation,
    comparison_holds: fn(f64, f64) -> bool,
) -> CaseReport {
    match evaluation {
        CaseEvaluation::NotReusable(reason) => CaseReport {
            fixture_id: fixture_id.to_string(),
            family,
            status: CaseStatus::Blocked,
            encoded_predicate: None,
            encoded_predicate_result: None,
            values: expected
                .iter()
                .map(|(name, unit, dimension, recorded)| ValueComparison {
                    name: name.clone(),
                    unit: unit.clone(),
                    dimension: dimension.clone(),
                    recorded: *recorded,
                    observed: None,
                    delta: None,
                    within_recorded_basis: None,
                })
                .collect(),
            regression_detail: None,
            block_reason: Some(reason),
        },
        CaseEvaluation::ExecutionFailed(reason) => CaseReport {
            fixture_id: fixture_id.to_string(),
            family,
            status: CaseStatus::Blocked,
            encoded_predicate: None,
            encoded_predicate_result: None,
            values: Vec::new(),
            regression_detail: None,
            block_reason: Some(format!("case execution failed: {reason}")),
        },
        CaseEvaluation::Evaluated(evaluated) => compare_evaluated_case(
            fixture_id,
            family,
            expected,
            evaluated.observed,
            evaluated.predicate,
            false,
            comparison_holds,
        ),
        CaseEvaluation::SuiteEvaluated(observed) => compare_evaluated_case(
            fixture_id,
            family,
            expected,
            observed,
            None,
            true,
            comparison_holds,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn compare_evaluated_case(
    fixture_id: &str,
    family: String,
    expected: &[(String, String, String, f64)],
    observed: Vec<(&'static str, f64)>,
    predicate: Option<(&'static str, bool)>,
    use_mechanics_fixture_comparison: bool,
    comparison_holds: fn(f64, f64) -> bool,
) -> CaseReport {
    if use_mechanics_fixture_comparison {
        let expected_names = expected
            .iter()
            .map(|(name, _, _, _)| name.as_str())
            .collect::<Vec<_>>();
        let mut seen = std::collections::BTreeSet::new();
        let duplicate_names = observed
            .iter()
            .filter_map(|(name, _)| (!seen.insert(*name)).then_some(*name))
            .collect::<Vec<_>>();
        let non_finite_names = observed
            .iter()
            .filter_map(|(name, value)| (!value.is_finite()).then_some(*name))
            .collect::<Vec<_>>();
        let missing_names = expected_names
            .iter()
            .copied()
            .filter(|name| !seen.contains(name))
            .collect::<Vec<_>>();
        let extra_names = seen
            .iter()
            .copied()
            .filter(|name| !expected_names.contains(name))
            .collect::<Vec<_>>();
        if !duplicate_names.is_empty()
            || !non_finite_names.is_empty()
            || !missing_names.is_empty()
            || !extra_names.is_empty()
        {
            return CaseReport {
                fixture_id: fixture_id.to_string(),
                family,
                status: CaseStatus::Blocked,
                encoded_predicate: predicate.map(|(name, _)| name),
                encoded_predicate_result: predicate.map(|(_, result)| result),
                values: expected
                    .iter()
                    .map(|(name, unit, dimension, recorded)| {
                        let observed_value = observed
                            .iter()
                            .find(|(observed_name, _)| observed_name == name)
                            .map(|(_, value)| *value)
                            .filter(|value| value.is_finite());
                        ValueComparison {
                            name: name.clone(),
                            unit: unit.clone(),
                            dimension: dimension.clone(),
                            recorded: *recorded,
                            observed: observed_value,
                            delta: observed_value.map(|value| value - *recorded),
                            within_recorded_basis: None,
                        }
                    })
                    .collect(),
                regression_detail: None,
                block_reason: Some(format!(
                    "suite-owned mechanics observations failed the runner seam: duplicate [{}], non-finite [{}], missing [{}], extra [{}]",
                    duplicate_names.join(", "),
                    non_finite_names.join(", "),
                    missing_names.join(", "),
                    extra_names.join(", ")
                )),
            };
        }
    }

    let mut values = Vec::new();
    let mut missing_names: Vec<&str> = Vec::new();
    let mut all_within = true;
    for (name, unit, dimension, recorded) in expected {
        let observed_value = observed
            .iter()
            .find(|(observed_name, _)| observed_name == name)
            .map(|(_, value)| *value);
        match observed_value {
            Some(observed_value) => {
                let within = if use_mechanics_fixture_comparison {
                    mechanics::fixture_recorded_comparison_holds(
                        fixture_id,
                        name,
                        observed_value,
                        *recorded,
                    )
                } else {
                    comparison_holds(observed_value, *recorded)
                };
                all_within &= within;
                values.push(ValueComparison {
                    name: name.clone(),
                    unit: unit.clone(),
                    dimension: dimension.clone(),
                    recorded: *recorded,
                    observed: Some(observed_value),
                    delta: Some(observed_value - *recorded),
                    within_recorded_basis: Some(within),
                });
            }
            None => {
                missing_names.push(name);
                values.push(ValueComparison {
                    name: name.clone(),
                    unit: unit.clone(),
                    dimension: dimension.clone(),
                    recorded: *recorded,
                    observed: None,
                    delta: None,
                    within_recorded_basis: None,
                });
            }
        }
    }

    if !missing_names.is_empty() {
        return CaseReport {
            fixture_id: fixture_id.to_string(),
            family,
            status: CaseStatus::Blocked,
            encoded_predicate: predicate.map(|(name, _)| name),
            encoded_predicate_result: predicate.map(|(_, result)| result),
            values,
            regression_detail: None,
            block_reason: Some(not_reusable_reason(&format!(
                "recorded value(s) {}",
                missing_names.join(", ")
            ))),
        };
    }

    let predicate_pass = predicate.map(|(_, ok)| ok).unwrap_or(true);
    let status = if all_within && predicate_pass {
        CaseStatus::ExecutedAndMatched
    } else {
        CaseStatus::ExecutedAndMismatched
    };
    CaseReport {
        fixture_id: fixture_id.to_string(),
        family,
        status,
        encoded_predicate: predicate.map(|(name, _)| name),
        encoded_predicate_result: predicate.map(|(_, result)| result),
        values,
        regression_detail: None,
        block_reason: None,
    }
}

fn finish_report(
    verb_token: &str,
    suite: &str,
    suite_deliverable: &'static str,
    comparison_basis: &'static str,
    whole_suite_default_applied: bool,
    cases: Vec<CaseReport>,
    diagnostics: &mut Vec<Diagnostic>,
) -> SuiteRunReport {
    let case_ref_type = if verb_token == "REGRESSION" {
        "regression_case"
    } else {
        "benchmark_case"
    };
    let mut matched = 0;
    let mut mismatched = 0;
    let mut blocked = 0;
    for case in &cases {
        match case.status {
            CaseStatus::ExecutedAndMatched => matched += 1,
            CaseStatus::ExecutedAndMismatched => {
                mismatched += 1;
                diagnostics.push(blocking(
                    &format!("HEADLESS_RUNNER_{verb_token}_CASE_MISMATCH"),
                    case_ref_type,
                    &case.fixture_id,
                    format!(
                        "case {} executed but did not match its recorded \
                         comparison values (regression evidence for current \
                         solver behavior; no release or acceptance meaning)",
                        case.fixture_id
                    ),
                ));
            }
            CaseStatus::Blocked => {
                blocked += 1;
                diagnostics.push(blocking(
                    &format!("HEADLESS_RUNNER_{verb_token}_CASE_COMPARISON_BASIS_NOT_REUSABLE"),
                    case_ref_type,
                    &case.fixture_id,
                    format!(
                        "case {} fails closed: {}",
                        case.fixture_id,
                        case.block_reason
                            .as_deref()
                            .unwrap_or("no reusable recorded comparison basis")
                    ),
                ));
            }
        }
    }
    SuiteRunReport {
        suite: suite.to_string(),
        suite_deliverable,
        claim_posture: REGRESSION_EVIDENCE_CLAIM_POSTURE,
        comparison_basis,
        whole_suite_default_applied,
        requested_case_count: cases.len(),
        executed_and_matched: matched,
        executed_and_mismatched: mismatched,
        blocked,
        cases,
    }
}

/// Execute the `run-benchmark` payload against a benchmark suite
/// (`mechanics` = DEL-09-01, `stress` = DEL-09-02).
pub fn run_benchmark_cases(suite: &str, cases: &[String]) -> SuiteRunOutcome {
    match suite {
        "mechanics" => run_wired_suite(
            "BENCHMARK",
            suite,
            "DEL-09-01",
            MECHANICS_COMPARISON_BASIS,
            cases,
            mechanics_inventory_expected,
            evaluate_mechanics_case,
            mechanics::recorded_comparison_holds,
        ),
        "stress" => run_wired_suite(
            "BENCHMARK",
            suite,
            "DEL-09-02",
            STRESS_COMPARISON_BASIS,
            cases,
            stress_inventory_expected,
            evaluate_stress_case,
            stress::recorded_comparison_holds,
        ),
        other => SuiteRunOutcome {
            report: None,
            diagnostics: vec![blocking(
                "HEADLESS_RUNNER_BENCHMARK_SUITE_UNSUPPORTED",
                "benchmark_suite",
                other,
                format!(
                    "run-benchmark supports the benchmark suites 'mechanics' \
                     (DEL-09-01) and 'stress' (DEL-09-02); '{other}' is not a \
                     supported benchmark suite (the nonlinear regression suite \
                     is bound to run-regression)"
                ),
            )],
        },
    }
}

/// Execute the `run-regression` payload against the regression suite
/// (`nonlinear` = DEL-09-03).
pub fn run_regression_cases(suite: &str, cases: &[String]) -> SuiteRunOutcome {
    match suite {
        "nonlinear" => run_nonlinear_suite(cases),
        other => SuiteRunOutcome {
            report: None,
            diagnostics: vec![blocking(
                "HEADLESS_RUNNER_REGRESSION_SUITE_UNSUPPORTED",
                "regression_suite",
                other,
                format!(
                    "run-regression supports the regression suite 'nonlinear' \
                     (DEL-09-03); '{other}' is not a supported regression \
                     suite (the mechanics and stress benchmark suites are \
                     bound to run-benchmark)"
                ),
            )],
        },
    }
}

type ExpectedInventory = Vec<(String, String, Vec<(String, String, String, f64)>)>;

#[allow(clippy::too_many_arguments)]
fn run_wired_suite(
    verb_token: &str,
    suite: &str,
    suite_deliverable: &'static str,
    comparison_basis: &'static str,
    cases: &[String],
    inventory_expected: fn() -> ExpectedInventory,
    evaluate_case: fn(&str) -> CaseEvaluation,
    comparison_holds: fn(f64, f64) -> bool,
) -> SuiteRunOutcome {
    let mut diagnostics = Vec::new();

    let inventory = match catch_unwind(AssertUnwindSafe(inventory_expected)) {
        Ok(inventory) => inventory,
        Err(_) => {
            diagnostics.push(blocking(
                &format!("HEADLESS_RUNNER_{verb_token}_CASE_EXECUTION_FAILED"),
                "benchmark_suite",
                suite,
                format!(
                    "suite '{suite}' fixture inventory construction panicked; \
                     no case executed"
                ),
            ));
            return SuiteRunOutcome {
                report: None,
                diagnostics,
            };
        }
    };
    let inventory_ids: Vec<String> = inventory.iter().map(|(id, _, _)| id.clone()).collect();
    let (requested, whole_suite_default_applied) = requested_ids(cases, &inventory_ids);

    let mut case_reports = Vec::new();
    for fixture_id in &requested {
        let Some((_, family, expected)) = inventory
            .iter()
            .find(|(inventory_id, _, _)| inventory_id == fixture_id)
        else {
            diagnostics.push(blocking(
                &format!("HEADLESS_RUNNER_{verb_token}_CASE_UNKNOWN"),
                "benchmark_case",
                fixture_id,
                format!(
                    "'{fixture_id}' is not a fixture_id in the '{suite}' \
                     suite's fixture_inventory()"
                ),
            ));
            case_reports.push(CaseReport {
                fixture_id: fixture_id.clone(),
                family: "UNKNOWN".to_string(),
                status: CaseStatus::Blocked,
                encoded_predicate: None,
                encoded_predicate_result: None,
                values: Vec::new(),
                regression_detail: None,
                block_reason: Some(format!(
                    "unknown fixture_id for suite '{suite}'; no case executed"
                )),
            });
            continue;
        };

        let evaluation = catch_unwind(AssertUnwindSafe(|| evaluate_case(fixture_id)))
            .unwrap_or_else(|_| {
                CaseEvaluation::ExecutionFailed("case evaluation panicked".to_string())
            });
        case_reports.push(compare_case(
            fixture_id,
            family.clone(),
            expected,
            evaluation,
            comparison_holds,
        ));
    }

    let report = finish_report(
        verb_token,
        suite,
        suite_deliverable,
        comparison_basis,
        whole_suite_default_applied,
        case_reports,
        &mut diagnostics,
    );
    SuiteRunOutcome {
        report: Some(report),
        diagnostics,
    }
}

// --- mechanics (DEL-09-01) wiring -----------------------------------------

fn mechanics_inventory_expected() -> ExpectedInventory {
    mechanics::fixture_inventory()
        .into_iter()
        .map(|fixture| {
            (
                fixture.fixture_id.to_string(),
                format!("{:?}", fixture.family),
                fixture
                    .expected_values
                    .iter()
                    .map(|value| {
                        (
                            value.name.to_string(),
                            value.unit.to_string(),
                            value.dimension.to_string(),
                            value.value,
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn evaluate_mechanics_case(fixture_id: &str) -> CaseEvaluation {
    match fixture_id {
        "MECH-PORTAL-SWAY-ORIGINAL" => match mechanics::solve_portal_frame_sway() {
            Ok(sway) => CaseEvaluation::Evaluated(EvaluatedCase {
                observed: vec![("top_right_sway_x", sway)],
                predicate: None,
            }),
            Err(error) => CaseEvaluation::ExecutionFailed(format!("{error:?}")),
        },
        "MECH-BRANCH-ASSEMBLY-THREE-MEMBER" => {
            match mechanics::solve_branch_assembly_benchmark() {
                Ok(result) => CaseEvaluation::Evaluated(EvaluatedCase {
                    observed: vec![
                        ("branch_axial_stiffness", result.branch_axial_stiffness),
                        ("header_lateral_stiffness", result.header_lateral_stiffness),
                        ("junction_uy_displacement", result.junction_uy_displacement),
                        (
                            "branch_tip_uy_displacement",
                            result.branch_tip_uy_displacement,
                        ),
                        ("branch_axial_extension", result.branch_axial_extension),
                        ("header_left_uy_reaction", result.header_left_uy_reaction),
                        ("header_right_uy_reaction", result.header_right_uy_reaction),
                        ("header_reaction_sum", result.header_reaction_sum),
                    ],
                    predicate: Some((
                        "open_pipe_stress_mechanics_benchmarks::validate_branch_assembly_benchmark",
                        mechanics::validate_branch_assembly_benchmark(),
                    )),
                }),
                Err(error) => CaseEvaluation::ExecutionFailed(format!("{error:?}")),
            }
        }
        "MECH-TP-PHYS-002-LINEAR-STATIC-INTEGRATION" => {
            match mechanics::solve_tp_phys_002_linear_static_integration() {
                Ok(result) => CaseEvaluation::Evaluated(EvaluatedCase {
                    observed: vec![
                        ("lumped_node_0_uy_force", result.lumped_node_0_uy_force),
                        ("lumped_node_1_uy_force", result.lumped_node_1_uy_force),
                        ("node_1_ux_displacement", result.node_1_ux_displacement),
                        ("node_1_uy_displacement", result.node_1_uy_displacement),
                        (
                            "node_1_uz_prescribed_displacement",
                            result.node_1_uz_displacement,
                        ),
                        (
                            "recovered_local_axial_force_j",
                            result.recovered_local_axial_force_j,
                        ),
                        ("recovered_local_shear_y_j", result.recovered_local_shear_y_j),
                    ],
                    predicate: Some((
                        "open_pipe_stress_mechanics_benchmarks::validate_tp_phys_002_linear_static_integration",
                        mechanics::validate_tp_phys_002_linear_static_integration(),
                    )),
                }),
                Err(error) => CaseEvaluation::ExecutionFailed(error),
            }
        }
        "MECH-TP-PHYS-004-LOAD-TO-RESULTANT" => {
            match mechanics::solve_tp_phys_004_load_to_resultant_integration() {
                Ok(result) => CaseEvaluation::Evaluated(EvaluatedCase {
                    observed: vec![
                        ("node_1_uy_displacement", result.node_1_uy_displacement),
                        ("node_1_rz_rotation", result.node_1_rz_rotation),
                        ("assembled_node_0_uy_force", result.assembled_node_0_uy_force),
                        (
                            "assembled_node_0_rz_moment",
                            result.assembled_node_0_rz_moment,
                        ),
                        ("midspan_shear_y", result.midspan_shear_y),
                        ("midspan_bending_z", result.midspan_bending_z),
                    ],
                    predicate: Some((
                        "open_pipe_stress_mechanics_benchmarks::validate_tp_phys_004_load_to_resultant_integration",
                        mechanics::validate_tp_phys_004_load_to_resultant_integration(),
                    )),
                }),
                Err(error) => CaseEvaluation::ExecutionFailed(error),
            }
        }
        "MECH-TP-PHYS-005-ORIENTED-LOAD-TO-RESULTANT" => {
            match mechanics::solve_tp_phys_005_oriented_load_to_resultant_integration() {
                Ok(result) => CaseEvaluation::Evaluated(EvaluatedCase {
                    observed: vec![
                        ("node_1_ux_displacement", result.node_1_ux_displacement),
                        ("node_1_rz_rotation", result.node_1_rz_rotation),
                        ("assembled_node_0_ux_force", result.assembled_node_0_ux_force),
                        (
                            "assembled_node_0_rz_moment",
                            result.assembled_node_0_rz_moment,
                        ),
                        ("midspan_shear_y", result.midspan_shear_y),
                        ("midspan_bending_z", result.midspan_bending_z),
                    ],
                    predicate: Some((
                        "open_pipe_stress_mechanics_benchmarks::validate_tp_phys_005_oriented_load_to_resultant_integration",
                        mechanics::validate_tp_phys_005_oriented_load_to_resultant_integration(),
                    )),
                }),
                Err(error) => CaseEvaluation::ExecutionFailed(error),
            }
        }
        "MECH-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-RESULTANT" => {
            match mechanics::solve_tp_phys_006_partial_span_load_to_resultant_integration() {
                Ok(result) => CaseEvaluation::Evaluated(EvaluatedCase {
                    observed: vec![
                        ("node_0_uy_equivalent_load", result.assembled_node_0_uy_force),
                        (
                            "node_0_rz_equivalent_moment",
                            result.assembled_node_0_rz_moment,
                        ),
                        ("node_1_uy_equivalent_load", result.assembled_node_1_uy_force),
                        (
                            "node_1_rz_equivalent_moment",
                            result.assembled_node_1_rz_moment,
                        ),
                        ("node_1_uy_displacement", result.node_1_uy_displacement),
                        ("node_1_rz_rotation", result.node_1_rz_rotation),
                        ("midspan_shear_y", result.midspan_shear_y),
                        ("midspan_bending_z", result.midspan_bending_z),
                    ],
                    predicate: Some((
                        "open_pipe_stress_mechanics_benchmarks::validate_tp_phys_006_partial_span_load_to_resultant_integration",
                        mechanics::validate_tp_phys_006_partial_span_load_to_resultant_integration(),
                    )),
                }),
                Err(error) => CaseEvaluation::ExecutionFailed(error),
            }
        }
        "MECH-TP-PHYS-007-STATION-SWEEP-RESULTANTS" => {
            match mechanics::solve_tp_phys_007_station_sweep_resultants_integration() {
                Ok(result) => {
                    let mut observed = Vec::new();
                    let mut names: Vec<String> = Vec::new();
                    for (index, station) in result.station_resultants.iter().enumerate() {
                        names.push(format!("requested_station_fraction_{index}"));
                        names.push(format!("station_{index}_shear_y"));
                        names.push(format!("station_{index}_bending_z"));
                        observed.push(station.station_fraction);
                        observed.push(station.shear_y);
                        observed.push(station.bending_z);
                    }
                    // Leak-free static names are not available for a dynamic
                    // station count; the fixture's station labels are stable,
                    // so the four-station wiring below mirrors the recorded
                    // inventory names exactly.
                    let mut wired: Vec<(&'static str, f64)> = Vec::new();
                    let static_names: [&'static str; 12] = [
                        "requested_station_fraction_0",
                        "station_0_shear_y",
                        "station_0_bending_z",
                        "requested_station_fraction_1",
                        "station_1_shear_y",
                        "station_1_bending_z",
                        "requested_station_fraction_2",
                        "station_2_shear_y",
                        "station_2_bending_z",
                        "requested_station_fraction_3",
                        "station_3_shear_y",
                        "station_3_bending_z",
                    ];
                    if names.len() == static_names.len() {
                        for (slot, value) in observed.into_iter().enumerate() {
                            wired.push((static_names[slot], value));
                        }
                        CaseEvaluation::Evaluated(EvaluatedCase {
                            observed: wired,
                            predicate: Some((
                                "open_pipe_stress_mechanics_benchmarks::validate_tp_phys_007_station_sweep_resultants_integration",
                                mechanics::validate_tp_phys_007_station_sweep_resultants_integration(),
                            )),
                        })
                    } else {
                        CaseEvaluation::ExecutionFailed(format!(
                            "station sweep returned {} stations; the recorded \
                             inventory names four stations",
                            result.station_resultants.len()
                        ))
                    }
                }
                Err(error) => CaseEvaluation::ExecutionFailed(error),
            }
        }
        "MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS" => {
            match mechanics::solve_tp_phys_008_thermal_pressure_axial_effects() {
                Ok(result) => CaseEvaluation::Evaluated(EvaluatedCase {
                    observed: vec![
                        ("thermal_axial_force", result.thermal_axial_force),
                        ("pressure_thrust_force", result.pressure_thrust_force),
                        ("total_axial_effect_force", result.total_axial_effect_force),
                        (
                            "equivalent_node_i_axial_load",
                            result.equivalent_node_i_axial_load,
                        ),
                        (
                            "equivalent_node_j_axial_load",
                            result.equivalent_node_j_axial_load,
                        ),
                        (
                            "recovered_local_i_axial_force",
                            result.recovered_local_i_axial_force,
                        ),
                        (
                            "recovered_local_j_axial_force",
                            result.recovered_local_j_axial_force,
                        ),
                        ("midspan_axial_force", result.midspan_axial_force),
                    ],
                    predicate: Some((
                        "open_pipe_stress_mechanics_benchmarks::validate_tp_phys_008_thermal_pressure_axial_effects",
                        mechanics::validate_tp_phys_008_thermal_pressure_axial_effects(),
                    )),
                }),
                Err(error) => CaseEvaluation::ExecutionFailed(error),
            }
        }
        "MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS" => {
            match mechanics::solve_tp_phys_009_combined_load_axial_effects() {
                Ok(result) => CaseEvaluation::Evaluated(EvaluatedCase {
                    observed: vec![
                        ("thermal_axial_force", result.thermal_axial_force),
                        ("pressure_thrust_force", result.pressure_thrust_force),
                        ("total_axial_effect_force", result.total_axial_effect_force),
                        ("node_0_ux_equivalent_load", result.assembled_node_0_ux_force),
                        ("node_0_uy_equivalent_load", result.assembled_node_0_uy_force),
                        (
                            "node_0_rz_equivalent_moment",
                            result.assembled_node_0_rz_moment,
                        ),
                        ("node_1_ux_equivalent_load", result.assembled_node_1_ux_force),
                        ("node_1_uy_equivalent_load", result.assembled_node_1_uy_force),
                        (
                            "node_1_rz_equivalent_moment",
                            result.assembled_node_1_rz_moment,
                        ),
                        ("node_1_ux_displacement", result.node_1_ux_displacement),
                        ("node_1_uy_displacement", result.node_1_uy_displacement),
                        ("node_1_rz_rotation", result.node_1_rz_rotation),
                        ("end_i_axial_force", result.end_i_axial_force),
                        ("end_i_shear_y", result.end_i_shear_y),
                        ("end_i_bending_z", result.end_i_bending_z),
                        ("midspan_axial_force", result.midspan_axial_force),
                        ("midspan_shear_y", result.midspan_shear_y),
                        ("midspan_bending_z", result.midspan_bending_z),
                    ],
                    predicate: Some((
                        "open_pipe_stress_mechanics_benchmarks::validate_tp_phys_009_combined_load_axial_effects",
                        mechanics::validate_tp_phys_009_combined_load_axial_effects(),
                    )),
                }),
                Err(error) => CaseEvaluation::ExecutionFailed(error),
            }
        }
        "MECH-TP-PHYS-014-CANONICAL-ANALYTICAL-PAYLOAD" => {
            match mechanics::solve_tp_phys_014_canonical_analytical_payload() {
                Ok(result) => CaseEvaluation::Evaluated(EvaluatedCase {
                    observed: vec![
                        ("distributed_load_count", result.distributed_load_count as f64),
                        ("point_force_count", result.point_force_count as f64),
                        ("node_0_uy_equivalent_load", result.assembled_node_0_uy_force),
                        (
                            "node_0_rz_equivalent_moment",
                            result.assembled_node_0_rz_moment,
                        ),
                        ("node_1_uy_equivalent_load", result.assembled_node_1_uy_force),
                        (
                            "node_1_rz_equivalent_moment",
                            result.assembled_node_1_rz_moment,
                        ),
                        ("node_1_uy_displacement", result.node_1_uy_displacement),
                        ("node_1_rz_rotation", result.node_1_rz_rotation),
                        ("midspan_shear_y", result.midspan_shear_y),
                        ("midspan_bending_z", result.midspan_bending_z),
                    ],
                    predicate: Some((
                        "open_pipe_stress_mechanics_benchmarks::validate_tp_phys_014_canonical_analytical_payload",
                        mechanics::validate_tp_phys_014_canonical_analytical_payload(),
                    )),
                }),
                Err(error) => CaseEvaluation::ExecutionFailed(error),
            }
        }
        "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE" => {
            let evidence = match mechanics::tp_phys_015a_canonical_solve_result_envelope() {
                Ok(evidence) => evidence,
                Err(error) => return CaseEvaluation::ExecutionFailed(error),
            };
            match mechanics::solve_tp_phys_014_canonical_analytical_payload() {
                Ok(result) => CaseEvaluation::Evaluated(EvaluatedCase {
                    observed: vec![
                        (
                            "quantity_result_count",
                            evidence.quantity_result_count as f64,
                        ),
                        (
                            "envelope_diagnostic_count",
                            evidence.envelope_diagnostic_count as f64,
                        ),
                        (
                            "export_validation_diagnostic_count",
                            evidence.export_validation_diagnostic_count as f64,
                        ),
                        (
                            "support_reaction_node_0_uy_force",
                            result.support_reaction_node_0_uy_force,
                        ),
                        (
                            "support_reaction_node_0_rz_moment",
                            result.support_reaction_node_0_rz_moment,
                        ),
                        ("midspan_shear_y", result.midspan_shear_y),
                        ("midspan_bending_z", result.midspan_bending_z),
                    ],
                    predicate: Some((
                        "open_pipe_stress_mechanics_benchmarks::validate_tp_phys_015a_canonical_solve_result_envelope",
                        mechanics::validate_tp_phys_015a_canonical_solve_result_envelope(),
                    )),
                }),
                Err(error) => CaseEvaluation::ExecutionFailed(error),
            }
        }
        _ => mechanics_suite_evaluation(fixture_id),
    }
}

fn mechanics_suite_evaluation(fixture_id: &str) -> CaseEvaluation {
    match mechanics::fixture_observations(fixture_id) {
        Ok(observations) => {
            let observed = observations
                .iter()
                .map(|observation| (observation.name, observation.value))
                .collect::<Vec<_>>();
            CaseEvaluation::SuiteEvaluated(observed)
        }
        Err(error) => CaseEvaluation::ExecutionFailed(format!(
            "suite-owned mechanics observation API returned {error:?}"
        )),
    }
}

// --- stress (DEL-09-02) wiring --------------------------------------------

fn stress_inventory_expected() -> ExpectedInventory {
    stress::fixture_inventory()
        .into_iter()
        .map(|fixture| {
            (
                fixture.fixture_id.to_string(),
                format!("{:?}", fixture.family),
                fixture
                    .expected_values
                    .iter()
                    .map(|value| {
                        (
                            value.name.to_string(),
                            value.unit.to_string(),
                            value.dimension.to_string(),
                            value.value,
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn optional_component(
    name: &'static str,
    component: Option<f64>,
) -> Result<(&'static str, f64), String> {
    component
        .map(|value| (name, value))
        .ok_or_else(|| format!("stress recovery produced no value for {name}"))
}

fn evaluate_stress_case(fixture_id: &str) -> CaseEvaluation {
    let collect = |pairs: Vec<Result<(&'static str, f64), String>>| -> CaseEvaluation {
        let mut observed = Vec::new();
        for pair in pairs {
            match pair {
                Ok(entry) => observed.push(entry),
                Err(reason) => return CaseEvaluation::ExecutionFailed(reason),
            }
        }
        CaseEvaluation::Evaluated(EvaluatedCase {
            observed,
            predicate: None,
        })
    };

    match fixture_id {
        "STRESS-AXIAL-NORMAL-ORIGINAL" => {
            let result = stress::recover_complete_fixture();
            collect(vec![optional_component(
                "axial_normal",
                result.components.axial_normal,
            )])
        }
        "STRESS-BENDING-NORMAL-ORIGINAL" => {
            let result = stress::recover_complete_fixture();
            collect(vec![
                optional_component("bending_normal_y", result.components.bending_normal_y),
                optional_component("bending_normal_z", result.components.bending_normal_z),
            ])
        }
        "STRESS-TORSIONAL-SHEAR-ORIGINAL" => {
            let result = stress::recover_complete_fixture();
            collect(vec![optional_component(
                "torsional_shear",
                result.components.torsional_shear,
            )])
        }
        "STRESS-PRESSURE-MEMBRANE-ORIGINAL" => {
            let result = stress::recover_complete_fixture();
            collect(vec![
                optional_component("pressure_hoop", result.components.pressure_hoop),
                optional_component(
                    "pressure_longitudinal",
                    result.components.pressure_longitudinal,
                ),
            ])
        }
        "STRESS-RANGE-MECHANICS-ORIGINAL" => {
            let result = stress::recover_range_fixture();
            collect(vec![
                optional_component("axial_normal_range", result.ranges.axial_normal_range),
                optional_component(
                    "bending_normal_y_range",
                    result.ranges.bending_normal_y_range,
                ),
                optional_component(
                    "bending_normal_z_range",
                    result.ranges.bending_normal_z_range,
                ),
                optional_component("torsional_shear_range", result.ranges.torsional_shear_range),
            ])
        }
        "STRESS-INTEGRATED-STRAIGHT-PIPE-ORIGINAL" => {
            let result = stress::recover_integrated_straight_pipe_stress_fixture();
            collect(vec![
                Ok(("end_j_axial_force", result.end_resultants.axial_force)),
                Ok((
                    "end_j_torsional_moment",
                    result.end_resultants.torsional_moment,
                )),
                optional_component("axial_normal", result.stress.components.axial_normal),
                optional_component("torsional_shear", result.stress.components.torsional_shear),
            ])
        }
        "STRESS-TP-PHYS-004-LOAD-TO-RESULTANT" => {
            let result = stress::recover_tp_phys_004_load_to_resultant_stress_fixture();
            collect(vec![
                Ok((
                    "midspan_bending_z",
                    result.station_resultants.bending_moment_z,
                )),
                optional_component(
                    "bending_normal_z",
                    result.station_stress.stress.components.bending_normal_z,
                ),
            ])
        }
        "STRESS-TP-PHYS-005-ORIENTED-LOAD-TO-STRESS" => {
            let result = stress::recover_tp_phys_005_oriented_load_to_stress_fixture();
            collect(vec![
                Ok((
                    "midspan_bending_z",
                    result.station_resultants.bending_moment_z,
                )),
                optional_component(
                    "bending_normal_z",
                    result.station_stress.stress.components.bending_normal_z,
                ),
                optional_component(
                    "axial_normal",
                    result.station_stress.stress.components.axial_normal,
                ),
            ])
        }
        "STRESS-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-STRESS" => {
            let result = stress::recover_tp_phys_006_partial_span_load_to_stress_fixture();
            collect(vec![
                Ok((
                    "midspan_bending_z",
                    result.station_resultants.bending_moment_z,
                )),
                optional_component(
                    "bending_normal_z",
                    result.station_stress.stress.components.bending_normal_z,
                ),
                optional_component(
                    "axial_normal",
                    result.station_stress.stress.components.axial_normal,
                ),
            ])
        }
        "STRESS-TP-PHYS-007-STATION-SWEEP-STRESS" => {
            let result = stress::recover_tp_phys_007_station_sweep_stress_fixture();
            if result.station_resultants.len() != 4 || result.station_stresses.len() != 4 {
                return CaseEvaluation::ExecutionFailed(format!(
                    "station sweep returned {} resultants and {} stresses; the \
                     recorded inventory names four stations",
                    result.station_resultants.len(),
                    result.station_stresses.len()
                ));
            }
            let static_names: [[&'static str; 4]; 4] = [
                [
                    "station_0_fraction",
                    "station_0_shear_y",
                    "station_0_bending_z",
                    "station_0_bending_normal_z",
                ],
                [
                    "station_1_fraction",
                    "station_1_shear_y",
                    "station_1_bending_z",
                    "station_1_bending_normal_z",
                ],
                [
                    "station_2_fraction",
                    "station_2_shear_y",
                    "station_2_bending_z",
                    "station_2_bending_normal_z",
                ],
                [
                    "station_3_fraction",
                    "station_3_shear_y",
                    "station_3_bending_z",
                    "station_3_bending_normal_z",
                ],
            ];
            let mut pairs = Vec::new();
            for index in 0..4 {
                let station = &result.station_resultants[index];
                let station_stress = &result.station_stresses[index];
                pairs.push(Ok((static_names[index][0], station.station_fraction)));
                pairs.push(Ok((static_names[index][1], station.shear_force_y)));
                pairs.push(Ok((static_names[index][2], station.bending_moment_z)));
                pairs.push(optional_component(
                    static_names[index][3],
                    station_stress.stress.components.bending_normal_z,
                ));
            }
            collect(pairs)
        }
        "STRESS-TP-PHYS-015-CANONICAL-RESULTANT-STRESS-RECOVERY" => {
            match stress::recover_tp_phys_015_canonical_resultant_stress_fixture() {
                Ok(result) => collect(vec![
                    Ok((
                        "canonical_midspan_shear_y",
                        result.station_resultants.shear_force_y,
                    )),
                    Ok((
                        "canonical_midspan_bending_z",
                        result.station_resultants.bending_moment_z,
                    )),
                    optional_component(
                        "station_midspan_axial_normal",
                        result.station_stress.stress.components.axial_normal,
                    ),
                    optional_component(
                        "station_midspan_bending_normal_z",
                        result.station_stress.stress.components.bending_normal_z,
                    ),
                    optional_component(
                        "station_midspan_bending_normal_y",
                        result.station_stress.stress.components.bending_normal_y,
                    ),
                    optional_component(
                        "station_midspan_torsional_shear",
                        result.station_stress.stress.components.torsional_shear,
                    ),
                ]),
                Err(error) => CaseEvaluation::ExecutionFailed(error),
            }
        }
        "STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS" => {
            let result = stress::recover_milltol_fixture();
            collect(vec![
                optional_component("axial_normal", result.components.axial_normal),
                optional_component("bending_normal_y", result.components.bending_normal_y),
                optional_component("bending_normal_z", result.components.bending_normal_z),
                optional_component("torsional_shear", result.components.torsional_shear),
                optional_component("pressure_hoop", result.components.pressure_hoop),
                optional_component(
                    "pressure_longitudinal",
                    result.components.pressure_longitudinal,
                ),
            ])
        }
        "STRESS-TP-PHYS-008-THERMAL-AXIAL-EFFECT-TO-STRESS" => CaseEvaluation::NotReusable(
            not_reusable_reason("recorded value 'thermal_axial_effect_force' (an explicit fixture input restatement)"),
        ),
        "STRESS-TP-PHYS-009-COMBINED-AXIAL-BENDING-TO-STRESS" => CaseEvaluation::NotReusable(
            not_reusable_reason("recorded value 'axial_effect_force' (an explicit fixture input restatement)"),
        ),
        "STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS" => CaseEvaluation::NotReusable(
            not_reusable_reason(
                "recorded values 'interpolated_elastic_modulus' and \
                 'interpolated_thermal_expansion_coefficient'",
            ),
        ),
        _ => CaseEvaluation::NotReusable(not_reusable_reason(
            "every recorded value of this case",
        )),
    }
}

// --- nonlinear (DEL-09-03) wiring -----------------------------------------

fn run_nonlinear_suite(cases: &[String]) -> SuiteRunOutcome {
    let mut diagnostics = Vec::new();

    let inventory = match catch_unwind(AssertUnwindSafe(nonlinear::fixture_inventory)) {
        Ok(inventory) => inventory,
        Err(_) => {
            diagnostics.push(blocking(
                "HEADLESS_RUNNER_REGRESSION_CASE_EXECUTION_FAILED",
                "regression_suite",
                "nonlinear",
                "suite 'nonlinear' fixture inventory construction panicked; no case executed"
                    .to_string(),
            ));
            return SuiteRunOutcome {
                report: None,
                diagnostics,
            };
        }
    };
    let inventory_ids: Vec<String> = inventory
        .iter()
        .map(|case| case.fixture_id.to_string())
        .collect();
    let (requested, whole_suite_default_applied) = requested_ids(cases, &inventory_ids);

    let mut case_reports = Vec::new();
    for fixture_id in &requested {
        let Some(case) = inventory
            .iter()
            .find(|case| case.fixture_id == fixture_id.as_str())
        else {
            diagnostics.push(blocking(
                "HEADLESS_RUNNER_REGRESSION_CASE_UNKNOWN",
                "regression_case",
                fixture_id,
                format!(
                    "'{fixture_id}' is not a fixture_id in the 'nonlinear' \
                     suite's fixture_inventory()"
                ),
            ));
            case_reports.push(CaseReport {
                fixture_id: fixture_id.clone(),
                family: "UNKNOWN".to_string(),
                status: CaseStatus::Blocked,
                encoded_predicate: None,
                encoded_predicate_result: None,
                values: Vec::new(),
                regression_detail: None,
                block_reason: Some(
                    "unknown fixture_id for suite 'nonlinear'; no case executed".to_string(),
                ),
            });
            continue;
        };

        case_reports.push(evaluate_nonlinear_case(case));
    }

    let report = finish_report(
        "REGRESSION",
        "nonlinear",
        "DEL-09-03",
        NONLINEAR_COMPARISON_BASIS,
        whole_suite_default_applied,
        case_reports,
        &mut diagnostics,
    );
    SuiteRunOutcome {
        report: Some(report),
        diagnostics,
    }
}

fn evaluate_nonlinear_case(case: &nonlinear::NonlinearRegressionCase) -> CaseReport {
    let recorded_states = case
        .expected_states
        .iter()
        .map(|expected| SupportStateReport {
            support_id: expected.support_id.to_string(),
            state: expected.state.as_str().to_string(),
        })
        .collect::<Vec<_>>();
    let recorded_changed_supports = case
        .expected_changed_supports
        .iter()
        .map(|support| support.to_string())
        .collect::<Vec<_>>();
    let recorded_diagnostic_codes = case
        .expected_diagnostic
        .as_ref()
        .map(|expected| vec![format!("{:?}", expected.code)])
        .unwrap_or_default();

    let residual_value = |observed: Option<f64>| ValueComparison {
        name: "residual_norm".to_string(),
        unit: "count".to_string(),
        dimension: "dimensionless".to_string(),
        recorded: case.expected_residual_norm,
        observed,
        delta: observed.map(|value| value - case.expected_residual_norm),
        // The suite crate's encoded predicate compares the residual by exact
        // equality (matches_expected_outcome); the per-value flag mirrors it.
        within_recorded_basis: observed.map(|value| value == case.expected_residual_norm),
    };

    match catch_unwind(AssertUnwindSafe(|| case.run())) {
        Ok(Ok(iteration)) => {
            let matched = case.matches_expected_outcome();
            CaseReport {
                fixture_id: case.fixture_id.to_string(),
                family: format!("{:?}", case.family),
                status: if matched {
                    CaseStatus::ExecutedAndMatched
                } else {
                    CaseStatus::ExecutedAndMismatched
                },
                encoded_predicate: Some(
                    "open_pipe_stress_nonlinear_benchmarks::NonlinearRegressionCase::matches_expected_outcome",
                ),
                encoded_predicate_result: Some(matched),
                values: vec![residual_value(Some(iteration.residual_norm))],
                regression_detail: Some(RegressionCaseDetail {
                    recorded_states,
                    observed_states: iteration
                        .states
                        .iter()
                        .map(|record| SupportStateReport {
                            support_id: record.support_id.clone(),
                            state: record.state.as_str().to_string(),
                        })
                        .collect(),
                    recorded_changed_supports,
                    observed_changed_supports: iteration.changed_supports.clone(),
                    recorded_converged: case.expected_converged,
                    observed_converged: Some(iteration.converged),
                    recorded_diagnostic_codes,
                    observed_diagnostic_codes: iteration
                        .diagnostics
                        .iter()
                        .map(|diagnostic| format!("{:?}", diagnostic.code))
                        .collect(),
                }),
                block_reason: None,
            }
        }
        Ok(Err(error)) => CaseReport {
            fixture_id: case.fixture_id.to_string(),
            family: format!("{:?}", case.family),
            status: CaseStatus::Blocked,
            encoded_predicate: Some(
                "open_pipe_stress_nonlinear_benchmarks::NonlinearRegressionCase::matches_expected_outcome",
            ),
            encoded_predicate_result: None,
            values: vec![residual_value(None)],
            regression_detail: Some(RegressionCaseDetail {
                recorded_states,
                observed_states: Vec::new(),
                recorded_changed_supports,
                observed_changed_supports: Vec::new(),
                recorded_converged: case.expected_converged,
                observed_converged: None,
                recorded_diagnostic_codes,
                observed_diagnostic_codes: Vec::new(),
            }),
            block_reason: Some(format!("case execution failed: {error:?}")),
        },
        Err(_) => CaseReport {
            fixture_id: case.fixture_id.to_string(),
            family: format!("{:?}", case.family),
            status: CaseStatus::Blocked,
            encoded_predicate: None,
            encoded_predicate_result: None,
            values: vec![residual_value(None)],
            regression_detail: None,
            block_reason: Some("case execution panicked".to_string()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FROZEN_MECHANICS_RUN: &str = include_str!(
        "../../../../validation/evidence/comparison_measurement/DEL0904_VD_20260811/\
         CURRENT_25_FIXTURE_RUNNER_OUTPUT.json"
    );

    fn expected(name: &str, recorded: f64) -> Vec<(String, String, String, f64)> {
        vec![(
            name.to_string(),
            "unit".to_string(),
            "dimension".to_string(),
            recorded,
        )]
    }

    fn suite_report(
        evaluation: CaseEvaluation,
        expected: &[(String, String, String, f64)],
    ) -> CaseReport {
        compare_case(
            "MECH-IMPOSED-DISPLACEMENT-SPRING",
            "ImposedDisplacement".to_string(),
            expected,
            evaluation,
            mechanics::recorded_comparison_holds,
        )
    }

    #[test]
    fn mechanics_whole_suite_is_25_cases_206_values_and_preserves_original_11_91() {
        let outcome = run_benchmark_cases("mechanics", &[]);
        assert!(outcome.diagnostics.is_empty(), "{:?}", outcome.diagnostics);
        let report = outcome.report.expect("mechanics report");
        assert_eq!(report.requested_case_count, 25);
        assert_eq!(report.executed_and_matched, 25);
        assert_eq!(report.executed_and_mismatched, 0);
        assert_eq!(report.blocked, 0);
        assert_eq!(
            report
                .cases
                .iter()
                .map(|case| case.values.len())
                .sum::<usize>(),
            206
        );
        assert!(report
            .cases
            .iter()
            .flat_map(|case| &case.values)
            .all(|value| {
                value.observed.is_some_and(f64::is_finite)
                    && value.within_recorded_basis == Some(true)
            }));

        let frozen: serde_json::Value =
            serde_json::from_str(FROZEN_MECHANICS_RUN).expect("frozen mechanics JSON");
        let frozen_cases = frozen["payload"]["suite_run"]["cases"]
            .as_array()
            .expect("frozen cases");
        let current = serde_json::to_value(&report).expect("serialize current report");
        let current_cases = current["cases"].as_array().expect("current cases");
        let frozen_original = frozen_cases
            .iter()
            .filter(|case| case["status"] == "executed_and_matched")
            .collect::<Vec<_>>();
        assert_eq!(frozen_original.len(), 11);
        assert_eq!(
            frozen_original
                .iter()
                .map(|case| case["values"].as_array().expect("values").len())
                .sum::<usize>(),
            91
        );
        for frozen_case in &frozen_original {
            let fixture_id = frozen_case["fixture_id"].as_str().expect("fixture id");
            let current_case = current_cases
                .iter()
                .find(|case| case["fixture_id"] == fixture_id)
                .expect("original case remains present");
            assert_eq!(
                current_case, *frozen_case,
                "original projection drift: {fixture_id}"
            );
        }

        let original_ids = frozen_original
            .iter()
            .map(|case| case["fixture_id"].as_str().expect("fixture id"))
            .collect::<std::collections::BTreeSet<_>>();
        let new_cases = report
            .cases
            .iter()
            .filter(|case| !original_ids.contains(case.fixture_id.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(new_cases.len(), 14);
        assert_eq!(
            new_cases
                .iter()
                .map(|case| case.values.len())
                .sum::<usize>(),
            115
        );
        for case in new_cases {
            let names = case
                .values
                .iter()
                .map(|value| value.name.as_str())
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(
                names.len(),
                case.values.len(),
                "duplicate name: {}",
                case.fixture_id
            );
        }
    }

    #[test]
    fn unknown_mechanics_fixture_id_fails_closed() {
        let outcome = run_benchmark_cases("mechanics", &["MECH-NOT-A-FIXTURE".to_string()]);
        let report = outcome.report.expect("mechanics report");
        assert_eq!(report.blocked, 1);
        assert_eq!(report.executed_and_matched, 0);
        assert!(outcome
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "HEADLESS_RUNNER_BENCHMARK_CASE_UNKNOWN" }));
    }

    #[test]
    fn incomplete_suite_observations_fail_closed_at_runner_seam() {
        let report = suite_report(
            CaseEvaluation::SuiteEvaluated(vec![]),
            &expected("spring_reaction_force", 6.0),
        );
        assert_eq!(report.status, CaseStatus::Blocked);
        assert!(report
            .block_reason
            .as_deref()
            .is_some_and(|reason| reason.contains("missing [spring_reaction_force]")));
    }

    #[test]
    fn duplicate_suite_observation_names_fail_closed_at_runner_seam() {
        let report = suite_report(
            CaseEvaluation::SuiteEvaluated(vec![
                ("spring_reaction_force", 6.0),
                ("spring_reaction_force", 6.0),
            ]),
            &expected("spring_reaction_force", 6.0),
        );
        assert_eq!(report.status, CaseStatus::Blocked);
        assert!(report
            .block_reason
            .as_deref()
            .is_some_and(|reason| reason.contains("duplicate [spring_reaction_force]")));
    }

    #[test]
    fn non_finite_suite_observation_fails_closed_at_runner_seam() {
        let report = suite_report(
            CaseEvaluation::SuiteEvaluated(vec![("spring_reaction_force", f64::NAN)]),
            &expected("spring_reaction_force", 6.0),
        );
        assert_eq!(report.status, CaseStatus::Blocked);
        assert!(report
            .block_reason
            .as_deref()
            .is_some_and(|reason| reason.contains("non-finite [spring_reaction_force]")));
    }

    #[test]
    fn suite_execution_error_fails_closed_at_runner_seam() {
        let report = suite_report(
            CaseEvaluation::ExecutionFailed("injected suite failure".to_string()),
            &expected("spring_reaction_force", 6.0),
        );
        assert_eq!(report.status, CaseStatus::Blocked);
        assert_eq!(
            report.block_reason.as_deref(),
            Some("case execution failed: injected suite failure")
        );
    }

    #[test]
    fn expected_observed_name_mismatch_fails_closed_at_runner_seam() {
        let report = suite_report(
            CaseEvaluation::SuiteEvaluated(vec![("unexpected", 6.0)]),
            &expected("spring_reaction_force", 6.0),
        );
        assert_eq!(report.status, CaseStatus::Blocked);
        let reason = report.block_reason.expect("block reason");
        assert!(reason.contains("missing [spring_reaction_force]"));
        assert!(reason.contains("extra [unexpected]"));
    }

    #[test]
    fn stress_and_nonlinear_whole_suites_remain_matched() {
        let stress = run_benchmark_cases("stress", &[]);
        let stress_report = stress.report.expect("stress report");
        assert_eq!(stress_report.executed_and_mismatched, 0);
        assert_eq!(stress_report.blocked, 3);
        assert_eq!(stress.diagnostics.len(), 3);
        let blocked_ids = stress_report
            .cases
            .iter()
            .filter(|case| case.status == CaseStatus::Blocked)
            .map(|case| case.fixture_id.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            blocked_ids,
            [
                "STRESS-TP-PHYS-008-THERMAL-AXIAL-EFFECT-TO-STRESS",
                "STRESS-TP-PHYS-009-COMBINED-AXIAL-BENDING-TO-STRESS",
                "STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS",
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(
            stress_report.executed_and_matched + stress_report.blocked,
            stress_report.requested_case_count
        );

        let nonlinear = run_regression_cases("nonlinear", &[]);
        assert!(
            nonlinear.diagnostics.is_empty(),
            "{:?}",
            nonlinear.diagnostics
        );
        let nonlinear_report = nonlinear.report.expect("nonlinear report");
        assert_eq!(nonlinear_report.requested_case_count, 5);
        assert_eq!(nonlinear_report.executed_and_matched, 5);
        assert_eq!(nonlinear_report.executed_and_mismatched, 0);
        assert_eq!(nonlinear_report.blocked, 0);
    }
}

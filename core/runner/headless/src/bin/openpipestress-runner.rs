//! Stable local headless runner CLI (DEC-065 / D-33 O-A).
//!
//! This binary is intentionally narrow: one foreground local process, JSON
//! request input from stdin or a caller-named file, structured JSON on stdout,
//! optional explicit file output, and no network/daemon/telemetry/hidden
//! filesystem mutation surface.

use std::io::{self, Read};
use std::process::ExitCode;

use open_pipe_stress_headless_runner::{
    benchmark_binding::{self, SuiteRunReport},
    redaction_binding::control_local_private,
    run_preview_in_memory_with_rule_check, validate_request, validate_result, Diagnostic,
    JobStateKind, Reference, RunnerOperation, RunnerRequest, RunnerResult, RunnerValidation,
};
use open_pipe_stress_product_physics::{LinearStaticPreviewRequest, MechanicsEnvelope};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const ARTIFACT: &str = "openpipestress.headless_runner_cli_output";
const SCHEMA_VERSION: &str = "1.0.0";
const POLICY_REF: &str = "DEC-065";

#[derive(Debug)]
struct CliArgs {
    verb: String,
    input_path: Option<String>,
    output_path: Option<String>,
    explicit_local_private_intent: bool,
}

#[derive(Debug, Deserialize)]
struct CliInput {
    request: RunnerRequest,
    #[serde(default)]
    solve: Option<SolveInput>,
    #[serde(default)]
    rule_check_aggregate: Option<String>,
    #[serde(default)]
    benchmark: Option<SuitePayload>,
    #[serde(default)]
    regression: Option<SuitePayload>,
}

#[derive(Debug, Deserialize)]
struct SolveInput {
    preview_model: Value,
}

/// Verb-named downstream payload beside `request`, following the settled
/// TP-RUNNER-015 `solve` wrapper precedent: a suite selector plus an optional
/// suite-local `fixture_id` case list (omitted or empty selects the named
/// suite's full `fixture_inventory()` case set).
#[derive(Debug, Deserialize)]
struct SuitePayload {
    suite: String,
    #[serde(default)]
    cases: Vec<String>,
}

#[derive(Debug, Serialize)]
struct CliPolicy {
    policy_ref: &'static str,
    binary: &'static str,
    stdout_default: bool,
    explicit_output_path_only: bool,
    local_foreground_process: bool,
    network_allowed: bool,
    telemetry_allowed: bool,
    daemon_allowed: bool,
    hidden_filesystem_mutation_allowed: bool,
    direct_sql_access_allowed: bool,
}

#[derive(Debug, Serialize)]
struct CliOutput {
    artifact: &'static str,
    schema_version: &'static str,
    deliverable_id: &'static str,
    package_id: &'static str,
    command: String,
    operation: Option<RunnerOperation>,
    policy: CliPolicy,
    request_validation: Option<RunnerValidation>,
    result_validation: Option<RunnerValidation>,
    runner_result: Option<RunnerResult>,
    mechanics_envelope: Option<MechanicsEnvelope>,
    suite_run: Option<SuiteRunReport>,
    diagnostics: Vec<Diagnostic>,
}

fn main() -> ExitCode {
    let args = match parse_args(std::env::args().skip(1)) {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };

    let input = match read_input(args.input_path.as_deref()) {
        Ok(input) => input,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };

    let (code, output) = execute_json(&args.verb, &input);
    let controlled = control_local_private(
        serde_json::to_value(output).expect("CLI output must serialize as JSON"),
        args.explicit_local_private_intent,
    );
    let rendered = serde_json::to_string_pretty(&controlled)
        .expect("controlled CLI output must serialize as JSON");
    println!("{rendered}");

    if !controlled.blocked {
        if let Some(output_path) = args.output_path.as_deref() {
            if let Err(error) = std::fs::write(output_path, format!("{rendered}\n")) {
                eprintln!("openpipestress-runner: cannot write {output_path}: {error}");
                return ExitCode::from(2);
            }
        }
    }

    ExitCode::from(if code == 2 {
        2
    } else if controlled.blocked {
        1
    } else {
        code
    })
}

fn parse_args<I>(args: I) -> Result<CliArgs, String>
where
    I: IntoIterator<Item = String>,
{
    let mut iter = args.into_iter();
    let Some(verb) = iter.next() else {
        return Err("openpipestress-runner: missing command".to_string());
    };
    if verb == "--help" || verb == "-h" {
        return Err(usage().to_string());
    }

    let mut input_path = None;
    let mut output_path = None;
    let mut explicit_local_private_intent = false;
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--input" => {
                let Some(value) = iter.next() else {
                    return Err("openpipestress-runner: --input requires a path or -".to_string());
                };
                input_path = Some(value);
            }
            "--output" => {
                let Some(value) = iter.next() else {
                    return Err("openpipestress-runner: --output requires a path".to_string());
                };
                output_path = Some(value);
            }
            "--explicit-local-private-intent" => {
                explicit_local_private_intent = true;
            }
            _ => return Err(format!("openpipestress-runner: unsupported argument {arg}")),
        }
    }

    Ok(CliArgs {
        verb,
        input_path,
        output_path,
        explicit_local_private_intent,
    })
}

fn usage() -> &'static str {
    "usage: openpipestress-runner <solve|validate-input|export-results|run-benchmark|run-regression> [--input <request.json>|-] [--output <result.json>] [--explicit-local-private-intent]"
}

fn read_input(input_path: Option<&str>) -> Result<String, String> {
    match input_path {
        Some("-") | None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .map_err(|error| format!("openpipestress-runner: cannot read stdin: {error}"))?;
            Ok(input)
        }
        Some(path) => std::fs::read_to_string(path)
            .map_err(|error| format!("openpipestress-runner: cannot read {path}: {error}")),
    }
}

fn execute_json(verb: &str, raw_input: &str) -> (u8, CliOutput) {
    let Some(operation) = operation_for_verb(verb) else {
        return (
            2,
            base_output(
                verb,
                None,
                None,
                None,
                None,
                vec![blocking(
                    "HEADLESS_RUNNER_CLI_UNSUPPORTED_COMMAND",
                    "cli_command",
                    verb,
                    "CLI command must be one of solve, validate-input, export-results, run-benchmark, or run-regression",
                )],
            ),
        );
    };

    let input: CliInput = match serde_json::from_str(raw_input) {
        Ok(input) => input,
        Err(error) => {
            return (
                2,
                base_output(
                    verb,
                    Some(operation),
                    None,
                    None,
                    None,
                    vec![blocking(
                        "HEADLESS_RUNNER_CLI_INPUT_JSON_INVALID",
                        "cli_input",
                        "stdin-or-file",
                        format!("input must be a JSON object with request metadata: {error}"),
                    )],
                ),
            );
        }
    };

    let request_validation = validate_request(&input.request);
    let mut diagnostics = Vec::new();
    if input.request.operation != operation {
        diagnostics.push(blocking(
            "HEADLESS_RUNNER_CLI_OPERATION_MISMATCH",
            "runner_request",
            &input.request.request_id,
            format!(
                "CLI verb {verb} must match request operation {}",
                operation_token(operation)
            ),
        ));
    }

    match operation {
        RunnerOperation::ValidateInput => {
            let has_blocking =
                request_validation.has_blocking_diagnostics() || has_blocking(&diagnostics);
            (
                if has_blocking { 1 } else { 0 },
                base_output(
                    verb,
                    Some(operation),
                    Some(request_validation),
                    None,
                    None,
                    diagnostics,
                ),
            )
        }
        RunnerOperation::Solve => execute_solve(verb, input, request_validation, diagnostics),
        RunnerOperation::RunBenchmark => execute_suite_verb(
            verb,
            RunnerOperation::RunBenchmark,
            input.benchmark,
            request_validation,
            diagnostics,
        ),
        RunnerOperation::RunRegression => execute_suite_verb(
            verb,
            RunnerOperation::RunRegression,
            input.regression,
            request_validation,
            diagnostics,
        ),
        RunnerOperation::ExportResults => {
            diagnostics.push(blocking(
                "HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD",
                "runner_operation",
                operation_token(operation),
                "operation verb is stable under DEC-065, but this tranche only implements the validated preview solve path; downstream payload binding must be supplied in a later bounded tranche",
            ));
            (
                1,
                base_output(
                    verb,
                    Some(operation),
                    Some(request_validation),
                    None,
                    None,
                    diagnostics,
                ),
            )
        }
        RunnerOperation::Tbd => unreachable!("TBD is not mapped from a stable CLI verb"),
    }
}

fn execute_suite_verb(
    verb: &str,
    operation: RunnerOperation,
    payload: Option<SuitePayload>,
    request_validation: RunnerValidation,
    mut diagnostics: Vec<Diagnostic>,
) -> (u8, CliOutput) {
    let (payload_field, missing_code) = match operation {
        RunnerOperation::RunBenchmark => ("benchmark", "HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING"),
        RunnerOperation::RunRegression => {
            ("regression", "HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING")
        }
        _ => unreachable!("execute_suite_verb only binds run-benchmark and run-regression"),
    };

    let Some(payload) = payload else {
        diagnostics.push(blocking(
            missing_code,
            "runner_operation",
            operation_token(operation),
            format!(
                "{verb} command requires {payload_field}.suite (with optional \
                 {payload_field}.cases fixture_id list) in the schema-first \
                 input JSON"
            ),
        ));
        return (
            1,
            base_output(
                verb,
                Some(operation),
                Some(request_validation),
                None,
                None,
                diagnostics,
            ),
        );
    };

    if request_validation.has_blocking_diagnostics() || has_blocking(&diagnostics) {
        return (
            1,
            base_output(
                verb,
                Some(operation),
                Some(request_validation),
                None,
                None,
                diagnostics,
            ),
        );
    }

    let outcome = match operation {
        RunnerOperation::RunBenchmark => {
            benchmark_binding::run_benchmark_cases(&payload.suite, &payload.cases)
        }
        RunnerOperation::RunRegression => {
            benchmark_binding::run_regression_cases(&payload.suite, &payload.cases)
        }
        _ => unreachable!("execute_suite_verb only binds run-benchmark and run-regression"),
    };
    diagnostics.extend(outcome.diagnostics);

    let exit_code = if has_blocking(&diagnostics) { 1 } else { 0 };
    let mut output = base_output(
        verb,
        Some(operation),
        Some(request_validation),
        None,
        None,
        diagnostics,
    );
    output.suite_run = outcome.report;
    (exit_code, output)
}

fn execute_solve(
    verb: &str,
    input: CliInput,
    request_validation: RunnerValidation,
    mut diagnostics: Vec<Diagnostic>,
) -> (u8, CliOutput) {
    let Some(solve) = input.solve else {
        diagnostics.push(blocking(
            "HEADLESS_RUNNER_SOLVE_PAYLOAD_MISSING",
            "runner_request",
            &input.request.request_id,
            "solve command requires solve.preview_model in the schema-first input JSON",
        ));
        return (
            1,
            base_output(
                verb,
                Some(RunnerOperation::Solve),
                Some(request_validation),
                None,
                None,
                diagnostics,
            ),
        );
    };

    if request_validation.has_blocking_diagnostics() || has_blocking(&diagnostics) {
        return (
            1,
            base_output(
                verb,
                Some(RunnerOperation::Solve),
                Some(request_validation),
                None,
                None,
                diagnostics,
            ),
        );
    }

    let preview_request: LinearStaticPreviewRequest =
        match serde_json::from_value(solve.preview_model) {
            Ok(preview_request) => preview_request,
            Err(error) => {
                diagnostics.push(blocking(
                    "HEADLESS_RUNNER_SOLVE_PAYLOAD_INVALID",
                    "runner_request",
                    &input.request.request_id,
                    format!("solve.preview_model must be a LinearStaticPreviewRequest: {error}"),
                ));
                return (
                    1,
                    base_output(
                        verb,
                        Some(RunnerOperation::Solve),
                        Some(request_validation),
                        None,
                        None,
                        diagnostics,
                    ),
                );
            }
        };

    let preview = run_preview_in_memory_with_rule_check(
        input.request,
        preview_request,
        input.rule_check_aggregate.as_deref(),
    );
    let result_validation = validate_result(&preview.runner_result);
    let clean = preview.runner_result.job.state == JobStateKind::Completed
        && preview.runner_result.diagnostics.is_empty()
        && !result_validation.has_blocking_diagnostics();
    let exit_code = if clean { 0 } else { 1 };

    (
        exit_code,
        base_output(
            verb,
            Some(RunnerOperation::Solve),
            Some(request_validation),
            Some(result_validation),
            Some(preview),
            diagnostics,
        ),
    )
}

fn base_output(
    verb: &str,
    operation: Option<RunnerOperation>,
    request_validation: Option<RunnerValidation>,
    result_validation: Option<RunnerValidation>,
    preview: Option<open_pipe_stress_headless_runner::PreviewRunnerOutput>,
    diagnostics: Vec<Diagnostic>,
) -> CliOutput {
    let (runner_result, mechanics_envelope) = match preview {
        Some(preview) => (Some(preview.runner_result), preview.mechanics_envelope),
        None => (None, None),
    };

    CliOutput {
        artifact: ARTIFACT,
        schema_version: SCHEMA_VERSION,
        deliverable_id: "DEL-10-05",
        package_id: "PKG-10",
        command: verb.to_string(),
        operation,
        policy: CliPolicy {
            policy_ref: POLICY_REF,
            binary: "openpipestress-runner",
            stdout_default: true,
            explicit_output_path_only: true,
            local_foreground_process: true,
            network_allowed: false,
            telemetry_allowed: false,
            daemon_allowed: false,
            hidden_filesystem_mutation_allowed: false,
            direct_sql_access_allowed: false,
        },
        request_validation,
        result_validation,
        runner_result,
        mechanics_envelope,
        suite_run: None,
        diagnostics,
    }
}

fn operation_for_verb(verb: &str) -> Option<RunnerOperation> {
    match verb {
        "solve" => Some(RunnerOperation::Solve),
        "validate-input" => Some(RunnerOperation::ValidateInput),
        "export-results" => Some(RunnerOperation::ExportResults),
        "run-benchmark" => Some(RunnerOperation::RunBenchmark),
        "run-regression" => Some(RunnerOperation::RunRegression),
        _ => None,
    }
}

fn operation_token(operation: RunnerOperation) -> &'static str {
    match operation {
        RunnerOperation::Solve => "solve",
        RunnerOperation::ValidateInput => "validate_input",
        RunnerOperation::ExportResults => "export_results",
        RunnerOperation::RunBenchmark => "run_benchmark",
        RunnerOperation::RunRegression => "run_regression",
        RunnerOperation::Tbd => "TBD",
    }
}

fn blocking(
    code: impl Into<String>,
    ref_type: impl Into<String>,
    ref_id: impl Into<String>,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic::runner_blocking(code, Reference::new(ref_type, ref_id), message)
}

fn has_blocking(diagnostics: &[Diagnostic]) -> bool {
    diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == open_pipe_stress_headless_runner::DiagnosticSeverity::Blocking
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_headless_runner::{
        PrivacyContext, ProfessionalBoundary, Provenance, RedistributionStatus, TbdDecisions,
    };
    use serde_json::json;

    fn request(operation: RunnerOperation) -> RunnerRequest {
        RunnerRequest {
            request_id: format!("request-{}", operation_token(operation).replace('_', "-")),
            operation,
            operation_ref: Reference::new("api_operation", operation_token(operation)),
            project_ref: Reference::new("project", "invented-project"),
            model_ref: Reference::new("model", "invented-model"),
            unit_system_ref: Reference::new("unit_system", "invented-si"),
            load_basis_refs: vec![Reference::new("load_case", "LC1")],
            input_manifest_ref: Reference::new("audit_manifest", "manifest-1"),
            requested_outputs: vec![
                "result_envelope".to_string(),
                "audit_manifest".to_string(),
                "diagnostics".to_string(),
            ],
            privacy: PrivacyContext::local_first_public_metadata(),
            provenance: Provenance {
                source_name: "invented final CLI fixture".to_string(),
                source_location: "core/runner/headless/src/bin/openpipestress-runner.rs"
                    .to_string(),
                source_license: "project invented".to_string(),
                contributor: "OpenPipeStress".to_string(),
                contributor_certification: "invented non-engineering example".to_string(),
                redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
                review_status: "accepted".to_string(),
            },
            professional_boundary: ProfessionalBoundary::project_default(),
            tbd_decisions: TbdDecisions::d33_local_cli_policy(),
        }
    }

    fn solve_input() -> String {
        let preview_model: Value = serde_json::from_str(include_str!(
            "../../../../../fixtures/product_preview/invented_preview_model.json"
        ))
        .expect("invented preview fixture must parse");
        json!({
            "request": request(RunnerOperation::Solve),
            "solve": {
                "preview_model": {
                    "model": preview_model,
                    "materials": []
                }
            }
        })
        .to_string()
    }

    #[test]
    fn stable_verbs_map_to_runner_operations() {
        assert_eq!(operation_for_verb("solve"), Some(RunnerOperation::Solve));
        assert_eq!(
            operation_for_verb("validate-input"),
            Some(RunnerOperation::ValidateInput)
        );
        assert_eq!(
            operation_for_verb("export-results"),
            Some(RunnerOperation::ExportResults)
        );
        assert_eq!(
            operation_for_verb("run-benchmark"),
            Some(RunnerOperation::RunBenchmark)
        );
        assert_eq!(
            operation_for_verb("run-regression"),
            Some(RunnerOperation::RunRegression)
        );
        assert_eq!(operation_for_verb("preview"), None);
    }

    #[test]
    fn validate_input_accepts_d33_policy_request() {
        let raw = json!({ "request": request(RunnerOperation::ValidateInput) }).to_string();
        let (code, output) = execute_json("validate-input", &raw);
        assert_eq!(code, 0);
        assert!(output
            .request_validation
            .expect("validation should be present")
            .diagnostics
            .is_empty());
        assert!(output.diagnostics.is_empty());
    }

    #[test]
    fn solve_runs_invented_preview_model_with_stdout_json_contract() {
        let (code, output) = execute_json("solve", &solve_input());
        assert_eq!(code, 0, "{output:#?}");
        assert_eq!(output.command, "solve");
        assert_eq!(output.operation, Some(RunnerOperation::Solve));
        assert_eq!(output.policy.policy_ref, "DEC-065");
        assert!(output.runner_result.is_some());
        assert!(output.mechanics_envelope.is_some());
        assert!(output.diagnostics.is_empty());
    }

    #[test]
    fn solve_blocks_missing_payload_without_running_solver() {
        let raw = json!({ "request": request(RunnerOperation::Solve) }).to_string();
        let (code, output) = execute_json("solve", &raw);
        assert_eq!(code, 1);
        assert!(output
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "HEADLESS_RUNNER_SOLVE_PAYLOAD_MISSING"));
        assert!(output.runner_result.is_none());
    }

    #[test]
    fn downstream_operation_verbs_are_stable_but_stubbed() {
        // Only export-results remains stubbed after the DEL-10-05
        // benchmark/regression payload-binding tranche.
        let raw = json!({ "request": request(RunnerOperation::ExportResults) }).to_string();
        let (code, output) = execute_json("export-results", &raw);
        assert_eq!(code, 1);
        assert!(output.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD"
        }));
    }

    #[test]
    fn run_benchmark_blocks_missing_payload_without_executing_cases() {
        let raw = json!({ "request": request(RunnerOperation::RunBenchmark) }).to_string();
        let (code, output) = execute_json("run-benchmark", &raw);
        assert_eq!(code, 1);
        assert!(output
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING"));
        assert!(!output.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD"
        }));
        assert!(output.suite_run.is_none());
    }

    #[test]
    fn run_regression_blocks_missing_payload_without_executing_cases() {
        let raw = json!({ "request": request(RunnerOperation::RunRegression) }).to_string();
        let (code, output) = execute_json("run-regression", &raw);
        assert_eq!(code, 1);
        assert!(output
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING"));
        assert!(output.suite_run.is_none());
    }

    #[test]
    fn run_benchmark_executes_named_mechanics_case_and_matches_recorded_values() {
        let raw = json!({
            "request": request(RunnerOperation::RunBenchmark),
            "benchmark": {
                "suite": "mechanics",
                "cases": ["MECH-TP-PHYS-004-LOAD-TO-RESULTANT"]
            }
        })
        .to_string();
        let (code, output) = execute_json("run-benchmark", &raw);
        assert_eq!(code, 0, "{output:#?}");
        let report = output.suite_run.expect("suite run report present");
        assert_eq!(report.suite, "mechanics");
        assert_eq!(report.requested_case_count, 1);
        assert_eq!(report.executed_and_matched, 1);
        assert!(!report.whole_suite_default_applied);
        let case = &report.cases[0];
        assert_eq!(case.fixture_id, "MECH-TP-PHYS-004-LOAD-TO-RESULTANT");
        assert_eq!(case.encoded_predicate_result, Some(true));
        assert!(case
            .values
            .iter()
            .all(|value| value.within_recorded_basis == Some(true) && value.observed.is_some()));
        assert!(output.diagnostics.is_empty());
    }

    #[test]
    fn run_benchmark_whole_suite_default_reports_every_case_without_silent_skips() {
        let raw = json!({
            "request": request(RunnerOperation::RunBenchmark),
            "benchmark": { "suite": "stress" }
        })
        .to_string();
        let (code, output) = execute_json("run-benchmark", &raw);
        // Fail-closed cases (no reusable public observed-value surface) make
        // the whole-suite default exit blocking, never silently partial.
        assert_eq!(code, 1);
        let report = output.suite_run.expect("suite run report present");
        assert!(report.whole_suite_default_applied);
        assert_eq!(report.requested_case_count, 15);
        assert_eq!(report.cases.len(), 15);
        assert_eq!(report.executed_and_matched, 12);
        assert_eq!(report.executed_and_mismatched, 0);
        assert_eq!(report.blocked, 3);
        assert_eq!(
            report.executed_and_matched + report.executed_and_mismatched + report.blocked,
            report.requested_case_count
        );
        assert!(output.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "HEADLESS_RUNNER_BENCHMARK_CASE_COMPARISON_BASIS_NOT_REUSABLE"
        }));
    }

    #[test]
    fn run_benchmark_multi_case_stress_run_matches_recorded_values() {
        let raw = json!({
            "request": request(RunnerOperation::RunBenchmark),
            "benchmark": {
                "suite": "stress",
                "cases": [
                    "STRESS-AXIAL-NORMAL-ORIGINAL",
                    "STRESS-RANGE-MECHANICS-ORIGINAL",
                    "STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS"
                ]
            }
        })
        .to_string();
        let (code, output) = execute_json("run-benchmark", &raw);
        assert_eq!(code, 0, "{output:#?}");
        let report = output.suite_run.expect("suite run report present");
        assert_eq!(report.requested_case_count, 3);
        assert_eq!(report.executed_and_matched, 3);
        assert!(output.diagnostics.is_empty());
    }

    #[test]
    fn run_benchmark_unknown_case_blocks_with_structured_diagnostic() {
        let raw = json!({
            "request": request(RunnerOperation::RunBenchmark),
            "benchmark": {
                "suite": "mechanics",
                "cases": ["MECH-DOES-NOT-EXIST"]
            }
        })
        .to_string();
        let (code, output) = execute_json("run-benchmark", &raw);
        assert_eq!(code, 1);
        assert!(output
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "HEADLESS_RUNNER_BENCHMARK_CASE_UNKNOWN"));
        let report = output.suite_run.expect("suite run report present");
        assert_eq!(report.blocked, 1);
        assert_eq!(report.cases[0].fixture_id, "MECH-DOES-NOT-EXIST");
    }

    #[test]
    fn run_benchmark_rejects_unsupported_suite_names() {
        let raw = json!({
            "request": request(RunnerOperation::RunBenchmark),
            "benchmark": { "suite": "nonlinear" }
        })
        .to_string();
        let (code, output) = execute_json("run-benchmark", &raw);
        assert_eq!(code, 1);
        assert!(output
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "HEADLESS_RUNNER_BENCHMARK_SUITE_UNSUPPORTED"));
        assert!(output.suite_run.is_none());
    }

    #[test]
    fn run_regression_executes_full_nonlinear_inventory_and_matches() {
        let raw = json!({
            "request": request(RunnerOperation::RunRegression),
            "regression": { "suite": "nonlinear" }
        })
        .to_string();
        let (code, output) = execute_json("run-regression", &raw);
        assert_eq!(code, 0, "{output:#?}");
        let report = output.suite_run.expect("suite run report present");
        assert_eq!(report.suite, "nonlinear");
        assert!(report.whole_suite_default_applied);
        assert_eq!(report.requested_case_count, 5);
        assert_eq!(report.executed_and_matched, 5);
        assert!(report.cases.iter().all(|case| {
            case.encoded_predicate_result == Some(true) && case.regression_detail.is_some()
        }));
        assert!(output.diagnostics.is_empty());
    }

    #[test]
    fn run_regression_named_case_reports_recorded_and_observed_outcome() {
        let raw = json!({
            "request": request(RunnerOperation::RunRegression),
            "regression": {
                "suite": "nonlinear",
                "cases": ["NL-NONCONVERGENCE-LIMIT-ORIGINAL"]
            }
        })
        .to_string();
        let (code, output) = execute_json("run-regression", &raw);
        assert_eq!(code, 0, "{output:#?}");
        let report = output.suite_run.expect("suite run report present");
        assert_eq!(report.executed_and_matched, 1);
        let detail = report.cases[0]
            .regression_detail
            .as_ref()
            .expect("regression detail present");
        assert_eq!(detail.observed_converged, Some(false));
        assert_eq!(detail.recorded_converged, false);
        assert!(!detail.observed_diagnostic_codes.is_empty());
    }

    #[test]
    fn run_regression_rejects_benchmark_suites() {
        let raw = json!({
            "request": request(RunnerOperation::RunRegression),
            "regression": { "suite": "mechanics" }
        })
        .to_string();
        let (code, output) = execute_json("run-regression", &raw);
        assert_eq!(code, 1);
        assert!(output
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "HEADLESS_RUNNER_REGRESSION_SUITE_UNSUPPORTED"));
    }
}

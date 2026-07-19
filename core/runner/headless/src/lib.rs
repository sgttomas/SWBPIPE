//! Headless runner contract support.
//!
//! This crate validates bounded in-memory request/result records for the
//! schema-first DEL-10-05 headless runner contract. It does not parse arbitrary
//! project files, invoke external processes, access the network, mutate the
//! filesystem, run GUI/report/adapter/local-FEA workflows, or emit professional
//! or code-compliance claims.

pub mod benchmark_binding;

use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_product_physics::{
    run_linear_static_preview, LinearStaticPreviewRequest, MechanicsEnvelope,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnalysisStatus {
    ModelIncomplete,
    MechanicsSolved,
    RuleInputsIncomplete,
    UserRuleChecked,
    UserRuleFailed,
    HumanReviewRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DiagnosticClass {
    SolveBlocking,
    RuleCheckBlocking,
    ProvenanceWarning,
    AssumptionWarning,
    NonlinearWarning,
    IpBoundaryWarning,
    UnitWarning,
    RunnerBlocking,
    ExportBlocking,
    PrivacyWarning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunnerOperation {
    Solve,
    ValidateInput,
    ExportResults,
    RunBenchmark,
    RunRegression,
    #[serde(rename = "TBD")]
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStateKind {
    Queued,
    Running,
    CancellationRequested,
    Completed,
    Failed,
    #[serde(rename = "TBD")]
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyClass {
    PublicMetadata,
    PrivateProjectData,
    PrivateRulePackData,
    Redacted,
    ProtectedSuspected,
    #[serde(rename = "TBD")]
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RedistributionStatus {
    PublicPermissive,
    PrivateOnly,
    Unknown,
    ProtectedSuspected,
    InventedNonEngineeringExample,
    #[serde(rename = "TBD")]
    Tbd,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Reference {
    pub ref_type: String,
    pub ref_id: String,
}

impl Reference {
    pub fn new(ref_type: impl Into<String>, ref_id: impl Into<String>) -> Self {
        Self {
            ref_type: ref_type.into(),
            ref_id: ref_id.into(),
        }
    }

    fn is_complete(&self) -> bool {
        !self.ref_type.trim().is_empty() && !self.ref_id.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Provenance {
    pub source_name: String,
    pub source_location: String,
    pub source_license: String,
    pub contributor: String,
    pub contributor_certification: String,
    pub redistribution_status: RedistributionStatus,
    pub review_status: String,
}

impl Provenance {
    fn is_complete(&self) -> bool {
        !self.source_name.trim().is_empty()
            && !self.source_location.trim().is_empty()
            && !self.source_license.trim().is_empty()
            && !self.contributor.trim().is_empty()
            && !self.contributor_certification.trim().is_empty()
            && !self.review_status.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChecksumRef {
    pub algorithm: String,
    pub canonicalization: String,
    pub payload_ref: Reference,
    pub value: String,
}

impl ChecksumRef {
    fn is_complete(&self) -> bool {
        matches!(self.algorithm.trim(), "sha256" | "sha512")
            && matches!(self.canonicalization.trim(), "rfc8785_jcs" | "NONE")
            && self.payload_ref.is_complete()
            && !self.value.trim().is_empty()
            && !self.value.trim().eq_ignore_ascii_case("TBD")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PrivacyContext {
    pub local_only: bool,
    pub telemetry_allowed: bool,
    pub private_payload_redacted: bool,
    pub classification: PrivacyClass,
}

impl PrivacyContext {
    pub fn local_first_public_metadata() -> Self {
        Self {
            local_only: true,
            telemetry_allowed: false,
            private_payload_redacted: true,
            classification: PrivacyClass::PublicMetadata,
        }
    }

    fn preserves_boundary(&self) -> bool {
        self.local_only
            && !self.telemetry_allowed
            && self.classification != PrivacyClass::ProtectedSuspected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProfessionalBoundary {
    pub human_review_required: bool,
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub software_makes_approval_claim: bool,
    pub software_makes_authentication_claim: bool,
}

impl ProfessionalBoundary {
    pub fn project_default() -> Self {
        Self {
            human_review_required: true,
            software_makes_compliance_claim: false,
            software_makes_certification_claim: false,
            software_makes_sealing_claim: false,
            software_makes_approval_claim: false,
            software_makes_authentication_claim: false,
        }
    }

    fn preserves_boundary(&self) -> bool {
        self.human_review_required
            && !self.software_makes_compliance_claim
            && !self.software_makes_certification_claim
            && !self.software_makes_sealing_claim
            && !self.software_makes_approval_claim
            && !self.software_makes_authentication_claim
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub class: DiagnosticClass,
    pub severity: DiagnosticSeverity,
    pub source: Reference,
    pub affected_object: Reference,
    pub message: String,
    pub remediation: String,
    pub provenance: Provenance,
}

impl Diagnostic {
    pub fn runner_blocking(
        code: impl Into<String>,
        affected_object: Reference,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            class: DiagnosticClass::RunnerBlocking,
            severity: DiagnosticSeverity::Blocking,
            source: Reference::new("headless_runner", "DEL-10-05"),
            affected_object,
            message: message.into(),
            remediation: "Provide explicit schema-first runner metadata before relying on this request or result.".to_string(),
            provenance: invented_provenance(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum DecisionState {
    #[serde(rename = "TBD")]
    Tbd,
    #[serde(rename = "SETTLED_DEC_065")]
    SettledDec065,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct TbdDecisions {
    pub final_cli_command_syntax: DecisionState,
    pub package_scripts: DecisionState,
    pub process_invocation: DecisionState,
    pub network_access: DecisionState,
    pub filesystem_mutation_policy: DecisionState,
    pub ci_provider: DecisionState,
    pub release_matrix: DecisionState,
    pub public_transport_protocol: DecisionState,
    pub external_adapter_formats: DecisionState,
    pub physical_project_container: DecisionState,
}

impl TbdDecisions {
    pub fn all_deferred() -> Self {
        Self {
            final_cli_command_syntax: DecisionState::Tbd,
            package_scripts: DecisionState::Tbd,
            process_invocation: DecisionState::Tbd,
            network_access: DecisionState::Tbd,
            filesystem_mutation_policy: DecisionState::Tbd,
            ci_provider: DecisionState::Tbd,
            release_matrix: DecisionState::Tbd,
            public_transport_protocol: DecisionState::Tbd,
            external_adapter_formats: DecisionState::Tbd,
            physical_project_container: DecisionState::Tbd,
        }
    }

    pub fn d33_local_cli_policy() -> Self {
        Self {
            final_cli_command_syntax: DecisionState::SettledDec065,
            package_scripts: DecisionState::SettledDec065,
            process_invocation: DecisionState::SettledDec065,
            network_access: DecisionState::SettledDec065,
            filesystem_mutation_policy: DecisionState::SettledDec065,
            ci_provider: DecisionState::Tbd,
            release_matrix: DecisionState::Tbd,
            public_transport_protocol: DecisionState::Tbd,
            external_adapter_formats: DecisionState::Tbd,
            physical_project_container: DecisionState::Tbd,
        }
    }

    fn preserves_d33_local_cli_policy(&self) -> bool {
        self.final_cli_command_syntax == DecisionState::SettledDec065
            && self.package_scripts == DecisionState::SettledDec065
            && self.process_invocation == DecisionState::SettledDec065
            && self.network_access == DecisionState::SettledDec065
            && self.filesystem_mutation_policy == DecisionState::SettledDec065
            && self.ci_provider == DecisionState::Tbd
            && self.release_matrix == DecisionState::Tbd
            && self.public_transport_protocol == DecisionState::Tbd
            && self.external_adapter_formats == DecisionState::Tbd
            && self.physical_project_container == DecisionState::Tbd
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunnerRequest {
    pub request_id: String,
    pub operation: RunnerOperation,
    pub operation_ref: Reference,
    pub project_ref: Reference,
    pub model_ref: Reference,
    pub unit_system_ref: Reference,
    pub load_basis_refs: Vec<Reference>,
    pub input_manifest_ref: Reference,
    pub requested_outputs: Vec<String>,
    pub privacy: PrivacyContext,
    pub provenance: Provenance,
    pub professional_boundary: ProfessionalBoundary,
    pub tbd_decisions: TbdDecisions,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct JobState {
    pub job_id: String,
    pub state: JobStateKind,
    pub current_step: u32,
    pub total_steps: u32,
    pub cancellation_supported: bool,
    pub cancellation_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResultEnvelopeRef {
    pub schema_ref: String,
    pub envelope_ref: Reference,
    pub compatibility: String,
}

impl ResultEnvelopeRef {
    pub fn result_export(envelope_ref: Reference) -> Self {
        Self {
            schema_ref: "schemas/results.schema.yaml".to_string(),
            envelope_ref,
            compatibility: "schema_first_json_result_envelope".to_string(),
        }
    }

    fn is_compatible(&self) -> bool {
        self.schema_ref == "schemas/results.schema.yaml"
            && self.compatibility == "schema_first_json_result_envelope"
            && self.envelope_ref.is_complete()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunnerResult {
    pub run_id: String,
    pub job: JobState,
    pub analysis_status: Vec<AnalysisStatus>,
    pub result_envelope_ref: ResultEnvelopeRef,
    pub result_refs: Vec<Reference>,
    pub audit_manifest_ref: Reference,
    pub checksums: Vec<ChecksumRef>,
    pub diagnostics: Vec<Diagnostic>,
    pub privacy: PrivacyContext,
    pub provenance: Provenance,
    pub professional_boundary: ProfessionalBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunnerValidation {
    pub diagnostics: Vec<Diagnostic>,
}

impl RunnerValidation {
    pub fn has_blocking_diagnostics(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Blocking)
    }
}

pub fn validate_request(request: &RunnerRequest) -> RunnerValidation {
    let mut diagnostics = Vec::new();

    if request.request_id.trim().is_empty() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_REQUEST_ID_MISSING",
            Reference::new("runner_request", "UNKNOWN"),
            "headless runner request must have a request identifier",
        ));
    }

    if request.operation == RunnerOperation::Tbd || !request.operation_ref.is_complete() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_OPERATION_MISSING",
            Reference::new("runner_request", &request.request_id),
            "headless runner request must identify a bounded operation",
        ));
    }

    if !request.project_ref.is_complete()
        || !request.model_ref.is_complete()
        || !request.unit_system_ref.is_complete()
        || !request.input_manifest_ref.is_complete()
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_REQUIRED_REFS_MISSING",
            Reference::new("runner_request", &request.request_id),
            "runner request must identify project, model, unit-system, and input-manifest references",
        ));
    }

    if request.load_basis_refs.is_empty()
        || request
            .load_basis_refs
            .iter()
            .any(|reference| !reference.is_complete())
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_LOAD_BASIS_MISSING",
            Reference::new("runner_request", &request.request_id),
            "runner request must identify load-case or combination basis references",
        ));
    }

    if request.requested_outputs.is_empty()
        || request
            .requested_outputs
            .iter()
            .any(|output| output.trim().is_empty())
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_OUTPUTS_MISSING",
            Reference::new("runner_request", &request.request_id),
            "runner request must identify requested structured outputs",
        ));
    }

    validate_shared_boundaries(
        "runner_request",
        &request.request_id,
        &request.privacy,
        &request.provenance,
        &request.professional_boundary,
        &mut diagnostics,
    );

    if !request.tbd_decisions.preserves_d33_local_cli_policy() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_D33_POLICY_MISMATCH",
            Reference::new("runner_request", &request.request_id),
            "runner request must use the DEC-065 / D-33 local CLI policy: final CLI, package-script posture, process invocation, network access, and filesystem mutation policy settled; CI, release matrix, public transport, external adapters, and project container still deferred",
        ));
    }

    RunnerValidation { diagnostics }
}

pub fn validate_result(result: &RunnerResult) -> RunnerValidation {
    let mut diagnostics = Vec::new();

    if result.run_id.trim().is_empty() || result.job.job_id.trim().is_empty() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RUN_ID_MISSING",
            Reference::new("runner_result", "UNKNOWN"),
            "headless runner result must identify the run and job",
        ));
    }

    if result.job.total_steps > 0 && result.job.current_step > result.job.total_steps {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_PROGRESS_INVALID",
            Reference::new("runner_result", &result.run_id),
            "job progress must not exceed total steps",
        ));
    }

    if !result
        .analysis_status
        .contains(&AnalysisStatus::HumanReviewRequired)
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_HUMAN_REVIEW_STATUS_MISSING",
            Reference::new("runner_result", &result.run_id),
            "runner results must preserve human-review-required status",
        ));
    }

    if !result.result_envelope_ref.is_compatible() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_EXPORT_REF_INVALID",
            Reference::new("runner_result", &result.run_id),
            "runner result must reference the schema-first JSON result export envelope",
        ));
    }

    if result.result_refs.is_empty()
        || result
            .result_refs
            .iter()
            .any(|reference| !reference.is_complete())
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_REFS_MISSING",
            Reference::new("runner_result", &result.run_id),
            "runner result must include deterministic references for computed result rows",
        ));
    }

    if !result.audit_manifest_ref.is_complete()
        || result.checksums.is_empty()
        || result
            .checksums
            .iter()
            .any(|checksum| !checksum.is_complete())
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_REPRODUCIBILITY_MISSING",
            Reference::new("runner_result", &result.run_id),
            "runner result must include audit-manifest and checksum references",
        ));
    }

    if result.result_envelope_ref.is_compatible()
        && !has_result_envelope_checksum(
            result,
            result.result_envelope_ref.envelope_ref.ref_id.as_str(),
        )
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_ENVELOPE_CHECKSUM_MISSING",
            Reference::new(
                "result_envelope",
                result.result_envelope_ref.envelope_ref.ref_id.as_str(),
            ),
            "runner result must include a checksum reference for the serialized result envelope",
        ));
    }

    validate_shared_boundaries(
        "runner_result",
        &result.run_id,
        &result.privacy,
        &result.provenance,
        &result.professional_boundary,
        &mut diagnostics,
    );

    RunnerValidation { diagnostics }
}

pub fn validate_result_with_optional_envelope_payload(
    result: &RunnerResult,
    result_envelope_payload: Option<&Value>,
) -> RunnerValidation {
    let mut diagnostics = validate_result(result).diagnostics;

    if let Some(payload) = result_envelope_payload {
        validate_result_envelope_payload(result, payload, &mut diagnostics);
    }

    RunnerValidation { diagnostics }
}

fn validate_result_envelope_payload(
    result: &RunnerResult,
    payload: &Value,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(root) = payload.as_object() else {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_ENVELOPE_PAYLOAD_INVALID",
            Reference::new("runner_result", &result.run_id),
            "result-envelope payload must be a schema-first JSON object",
        ));
        return;
    };

    if root.get("deliverable_id").and_then(Value::as_str) != Some("DEL-08-04") {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_ENVELOPE_PAYLOAD_INVALID",
            Reference::new("runner_result", &result.run_id),
            "result-envelope payload must identify the DEL-08-04 result-export schema wrapper",
        ));
    }

    let Some(envelope) = root.get("result_envelope").and_then(Value::as_object) else {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_ENVELOPE_PAYLOAD_INVALID",
            Reference::new("runner_result", &result.run_id),
            "result-envelope payload must include a result_envelope object",
        ));
        return;
    };

    let expected_envelope_id = result.result_envelope_ref.envelope_ref.ref_id.as_str();
    if envelope.get("envelope_id").and_then(Value::as_str) != Some(expected_envelope_id) {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_ENVELOPE_REF_MISMATCH",
            Reference::new("result_envelope", expected_envelope_id),
            "runner result-envelope reference must match the serialized envelope identifier",
        ));
    }

    if !envelope
        .get("analysis_status")
        .and_then(Value::as_array)
        .map(|statuses| {
            statuses
                .iter()
                .any(|status| status.as_str() == Some("HUMAN_REVIEW_REQUIRED"))
        })
        .unwrap_or(false)
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_ENVELOPE_PAYLOAD_INVALID",
            Reference::new("result_envelope", expected_envelope_id),
            "serialized result envelope must preserve HUMAN_REVIEW_REQUIRED status",
        ));
    }

    if !envelope
        .get("result_sets")
        .and_then(Value::as_array)
        .map(|sets| !sets.is_empty())
        .unwrap_or(false)
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_ENVELOPE_PAYLOAD_INVALID",
            Reference::new("result_envelope", expected_envelope_id),
            "serialized result envelope must include at least one result set",
        ));
    }

    if !envelope
        .get("reproducibility")
        .and_then(Value::as_object)
        .and_then(|reproducibility| reproducibility.get("deterministic_ordering"))
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_ENVELOPE_PAYLOAD_INVALID",
            Reference::new("result_envelope", expected_envelope_id),
            "serialized result envelope must preserve deterministic ordering evidence",
        ));
    }

    if !has_result_envelope_checksum(result, expected_envelope_id) {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_ENVELOPE_CHECKSUM_MISSING",
            Reference::new("result_envelope", expected_envelope_id),
            "runner result must include a checksum reference for the serialized result envelope",
        ));
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PreviewRunnerOutput {
    pub runner_result: RunnerResult,
    pub mechanics_envelope: Option<MechanicsEnvelope>,
}

/// Map an automatic rule-check status — the `core/rules/rule_check_runner`
/// worst-of `aggregate_status` vocabulary (`RULE_INPUTS_INCOMPLETE`,
/// `USER_RULE_CHECKED`, `USER_RULE_FAILED`) — to the runner analysis-status
/// vocabulary. `None` for any other string, so callers never silently coerce an
/// unrecognized status (CONTRACT no-silent-defaults). The wider model/mechanics
/// statuses and the external `HUMAN_APPROVED_FOR_PROJECT` record are not
/// producible from a rule-check aggregate and are intentionally absent here.
pub fn analysis_status_for_rule_check(rule_check_status: &str) -> Option<AnalysisStatus> {
    match rule_check_status {
        "RULE_INPUTS_INCOMPLETE" => Some(AnalysisStatus::RuleInputsIncomplete),
        "USER_RULE_CHECKED" => Some(AnalysisStatus::UserRuleChecked),
        "USER_RULE_FAILED" => Some(AnalysisStatus::UserRuleFailed),
        _ => None,
    }
}

pub fn run_preview_in_memory(
    request: RunnerRequest,
    preview_request: LinearStaticPreviewRequest,
) -> PreviewRunnerOutput {
    run_preview_in_memory_with_rule_check(request, preview_request, None)
}

/// As [`run_preview_in_memory`], but drives an automatic rule-check aggregate
/// status into the solve envelope and the runner result's `analysis_status`, so
/// a run that actually executed rule checks reports `USER_RULE_CHECKED` /
/// `USER_RULE_FAILED` instead of the solve-only `RULE_INPUTS_INCOMPLETE`
/// default. `rule_check_aggregate` is the `rule_check_runner` aggregate as its
/// vocabulary string (e.g. [`RuleCheckStatus::as_str`] output).
///
/// - `None`: no rule checks ran — the solve envelope keeps its conservative
///   `RULE_INPUTS_INCOMPLETE` and the analysis status is unchanged.
/// - a recognized status: written into the carried `MechanicsEnvelope`'s
///   `rule_check` (before its checksum binds) and reflected in `analysis_status`.
/// - any other non-`None` string: a blocking diagnostic
///   (`HEADLESS_RUNNER_RULE_CHECK_STATUS_INVALID`), never silently dropped; the
///   envelope is left at `RULE_INPUTS_INCOMPLETE` (conservative — no false pass).
pub fn run_preview_in_memory_with_rule_check(
    request: RunnerRequest,
    preview_request: LinearStaticPreviewRequest,
    rule_check_aggregate: Option<&str>,
) -> PreviewRunnerOutput {
    let request_validation = validate_request(&request);
    let run_id = format!("run:headless-preview:{}", request.request_id);

    if request_validation.has_blocking_diagnostics() {
        return PreviewRunnerOutput {
            runner_result: RunnerResult {
                run_id: run_id.clone(),
                job: JobState {
                    job_id: format!("job:{}", request.request_id),
                    state: JobStateKind::Failed,
                    current_step: 1,
                    total_steps: 1,
                    cancellation_supported: true,
                    cancellation_requested: false,
                },
                analysis_status: vec![
                    AnalysisStatus::ModelIncomplete,
                    AnalysisStatus::HumanReviewRequired,
                ],
                result_envelope_ref: ResultEnvelopeRef::result_export(Reference::new(
                    "result_envelope",
                    format!("result-envelope:{run_id}:not-produced"),
                )),
                result_refs: vec![Reference::new(
                    "runner_validation",
                    format!("runner-validation:{}", request.request_id),
                )],
                audit_manifest_ref: Reference::new(
                    "audit_manifest",
                    format!("audit-manifest:{run_id}:blocked"),
                ),
                checksums: vec![checksum_ref(
                    "runner_request",
                    &request.request_id,
                    &request,
                )],
                diagnostics: request_validation.diagnostics,
                privacy: request.privacy,
                provenance: request.provenance,
                professional_boundary: request.professional_boundary,
            },
            mechanics_envelope: None,
        };
    }

    let mut mechanics = run_linear_static_preview(preview_request);
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    // Drive the optional rule-check aggregate into the solve envelope before its
    // checksum binds, so the envelope and the runner analysis_status honestly
    // carry the rule-check outcome rather than the solve-only default. An
    // unrecognized aggregate is a blocking diagnostic, not a silent coercion.
    if let Some(aggregate) = rule_check_aggregate {
        if analysis_status_for_rule_check(aggregate).is_some() {
            mechanics.status.rule_check = aggregate.to_string();
        } else {
            diagnostics.push(Diagnostic::runner_blocking(
                "HEADLESS_RUNNER_RULE_CHECK_STATUS_INVALID",
                Reference::new("rule_check_aggregate", aggregate),
                "rule-check aggregate status must be one of RULE_INPUTS_INCOMPLETE, USER_RULE_CHECKED, or USER_RULE_FAILED",
            ));
        }
    }
    let result_refs = mechanics
        .results
        .iter()
        .map(|item| Reference::new("result", item.id.clone()))
        .collect::<Vec<_>>();
    let mut analysis_status = vec![AnalysisStatus::HumanReviewRequired];
    if mechanics.status.mechanics == "MECHANICS_SOLVED" {
        analysis_status.push(AnalysisStatus::MechanicsSolved);
    } else {
        analysis_status.push(AnalysisStatus::ModelIncomplete);
    }
    // Single source of truth: derive the rule-check analysis status from the
    // (possibly aggregate-driven) solve envelope.
    if let Some(status) = analysis_status_for_rule_check(&mechanics.status.rule_check) {
        analysis_status.push(status);
    }

    let runner_result = RunnerResult {
        run_id: run_id.clone(),
        job: JobState {
            job_id: format!("job:{}", request.request_id),
            state: JobStateKind::Completed,
            current_step: 3,
            total_steps: 3,
            cancellation_supported: true,
            cancellation_requested: false,
        },
        analysis_status,
        result_envelope_ref: ResultEnvelopeRef::result_export(Reference::new(
            "result_envelope",
            format!("result-envelope:{run_id}"),
        )),
        result_refs,
        audit_manifest_ref: Reference::new("audit_manifest", format!("audit-manifest:{run_id}")),
        checksums: vec![
            checksum_ref("runner_request", &request.request_id, &request),
            checksum_ref(
                "result_envelope",
                &format!("result-envelope:{run_id}"),
                &mechanics,
            ),
        ],
        diagnostics,
        privacy: request.privacy,
        provenance: request.provenance,
        professional_boundary: request.professional_boundary,
    };

    PreviewRunnerOutput {
        runner_result,
        mechanics_envelope: Some(mechanics),
    }
}

fn has_result_envelope_checksum(result: &RunnerResult, expected_envelope_id: &str) -> bool {
    result.checksums.iter().any(|checksum| {
        checksum.payload_ref.ref_type == "result_envelope"
            && checksum.payload_ref.ref_id == expected_envelope_id
            && checksum.is_complete()
    })
}

fn validate_shared_boundaries(
    ref_type: &str,
    ref_id: &str,
    privacy: &PrivacyContext,
    provenance: &Provenance,
    professional_boundary: &ProfessionalBoundary,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !privacy.preserves_boundary() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_PRIVACY_BOUNDARY_VIOLATION",
            Reference::new(ref_type, ref_id),
            "headless runner records must remain local-first, telemetry-off, and must not carry suspected protected payloads",
        ));
    }

    if !provenance.is_complete()
        || provenance.redistribution_status == RedistributionStatus::ProtectedSuspected
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_PROVENANCE_MISSING",
            Reference::new(ref_type, ref_id),
            "headless runner records must carry source, license, contributor, redistribution, and review provenance",
        ));
    }

    if !professional_boundary.preserves_boundary() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_PROFESSIONAL_BOUNDARY_VIOLATION",
            Reference::new(ref_type, ref_id),
            "runner output must not make compliance, certification, sealing, approval, or authentication claims",
        ));
    }
}

/// Checksum evidence over the shared RFC 8785 (JCS) canonical JSON text from
/// `core/serialization/canonical_json` (completion-plan hardening row H5), so
/// the headless runner hashes the same canonical bytes as every other
/// OpenPipeStress hash seam.
fn checksum_ref<T: Serialize>(ref_type: &str, ref_id: &str, value: &T) -> ChecksumRef {
    let payload = serde_json::to_value(value).expect("runner values must serialize");
    ChecksumRef {
        algorithm: "sha256".to_string(),
        canonicalization: "rfc8785_jcs".to_string(),
        payload_ref: Reference::new(ref_type, ref_id),
        value: sha256_hex(&canonical_json(&payload)),
    }
}

fn sha256_hex(payload: &str) -> String {
    let digest = Sha256::digest(payload.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn invented_provenance() -> Provenance {
    Provenance {
        source_name: "OpenPipeStress headless runner validator".to_string(),
        source_location: "core/runner/headless".to_string(),
        source_license: "project".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "diagnostic metadata only".to_string(),
        redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
        review_status: "accepted".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TP_PHYS_015_RESULT_ENVELOPE: &str = include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"
    );

    fn checksum(id: &str) -> ChecksumRef {
        ChecksumRef {
            algorithm: "sha256".to_string(),
            canonicalization: "rfc8785_jcs".to_string(),
            payload_ref: Reference::new("payload", id),
            value: format!("{id}-hash"),
        }
    }

    fn result_envelope_checksum(id: &str) -> ChecksumRef {
        ChecksumRef {
            algorithm: "sha256".to_string(),
            canonicalization: "rfc8785_jcs".to_string(),
            payload_ref: Reference::new("result_envelope", id),
            value: format!("{id}-hash"),
        }
    }

    fn provenance() -> Provenance {
        Provenance {
            source_name: "invented headless runner fixture".to_string(),
            source_location: "validation/benchmarks/invented".to_string(),
            source_license: "project invented".to_string(),
            contributor: "OpenPipeStress".to_string(),
            contributor_certification: "invented non-engineering example".to_string(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "accepted".to_string(),
        }
    }

    fn request() -> RunnerRequest {
        RunnerRequest {
            request_id: "request-1".to_string(),
            operation: RunnerOperation::Solve,
            operation_ref: Reference::new("api_operation", "ops.solve.job"),
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
            provenance: provenance(),
            professional_boundary: ProfessionalBoundary::project_default(),
            tbd_decisions: TbdDecisions::d33_local_cli_policy(),
        }
    }

    fn preview_request() -> LinearStaticPreviewRequest {
        LinearStaticPreviewRequest {
            model: serde_json::from_str(include_str!(
                "../../../../fixtures/product_preview/invented_preview_model.json"
            ))
            .expect("invented preview model fixture should parse"),
            materials: Vec::new(),
        }
    }

    fn result() -> RunnerResult {
        RunnerResult {
            run_id: "run-1".to_string(),
            job: JobState {
                job_id: "job-1".to_string(),
                state: JobStateKind::Completed,
                current_step: 3,
                total_steps: 3,
                cancellation_supported: true,
                cancellation_requested: false,
            },
            analysis_status: vec![
                AnalysisStatus::MechanicsSolved,
                AnalysisStatus::HumanReviewRequired,
            ],
            result_envelope_ref: ResultEnvelopeRef::result_export(Reference::new(
                "result_envelope",
                "result-envelope-1",
            )),
            result_refs: vec![Reference::new("result", "result:disp:N-100")],
            audit_manifest_ref: Reference::new("audit_manifest", "manifest-1"),
            checksums: vec![
                checksum("input-manifest"),
                result_envelope_checksum("result-envelope-1"),
            ],
            diagnostics: Vec::new(),
            privacy: PrivacyContext::local_first_public_metadata(),
            provenance: provenance(),
            professional_boundary: ProfessionalBoundary::project_default(),
        }
    }

    fn tp_phys_015_runner_result() -> RunnerResult {
        RunnerResult {
            run_id: "RUN-TP-PHYS-015A-CANONICAL-SOLVE-RESULT-ENVELOPE".to_string(),
            job: JobState {
                job_id: "job:tp-phys-015".to_string(),
                state: JobStateKind::Completed,
                current_step: 3,
                total_steps: 3,
                cancellation_supported: true,
                cancellation_requested: false,
            },
            analysis_status: vec![
                AnalysisStatus::MechanicsSolved,
                AnalysisStatus::HumanReviewRequired,
            ],
            result_envelope_ref: ResultEnvelopeRef::result_export(Reference::new(
                "result_envelope",
                "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE",
            )),
            result_refs: vec![
                Reference::new("result", "result:disp:node-N-2:uy"),
                Reference::new("result", "result:load-vector:node-N-1:uy"),
                Reference::new("result", "result:moment:element-E-1:midspan:bending-z"),
            ],
            audit_manifest_ref: Reference::new(
                "task_run_record",
                "TASK_RUN_2026-05-17_TP-RESULT-017",
            ),
            checksums: vec![
                checksum("runner-input"),
                ChecksumRef {
                    algorithm: "sha256".to_string(),
                    canonicalization: "rfc8785_jcs".to_string(),
                    payload_ref: Reference::new(
                        "result_envelope",
                        "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE",
                    ),
                    value: "tp-phys-015-result-envelope-hash".to_string(),
                },
            ],
            diagnostics: Vec::new(),
            privacy: PrivacyContext::local_first_public_metadata(),
            provenance: provenance(),
            professional_boundary: ProfessionalBoundary::project_default(),
        }
    }

    #[test]
    fn valid_request_has_no_blocking_diagnostics() {
        let validation = validate_request(&request());
        assert!(!validation.has_blocking_diagnostics(), "{validation:?}");
    }

    #[test]
    fn request_requires_load_basis_and_d33_policy() {
        let mut request = request();
        request.load_basis_refs.clear();
        request.tbd_decisions.public_transport_protocol = DecisionState::SettledDec065;
        let validation = validate_request(&request);
        let codes: Vec<_> = validation
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();
        assert!(codes.contains(&"HEADLESS_RUNNER_LOAD_BASIS_MISSING"));
        assert!(codes.contains(&"HEADLESS_RUNNER_D33_POLICY_MISMATCH"));
    }

    #[test]
    fn valid_result_has_no_blocking_diagnostics() {
        let validation = validate_result(&result());
        assert!(!validation.has_blocking_diagnostics(), "{validation:?}");
    }

    #[test]
    fn result_requires_human_review_and_result_export_compatibility() {
        let mut result = result();
        result.analysis_status = vec![AnalysisStatus::MechanicsSolved];
        result.result_envelope_ref.schema_ref = "ad_hoc_output.json".to_string();
        let validation = validate_result(&result);
        let codes: Vec<_> = validation
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();
        assert!(codes.contains(&"HEADLESS_RUNNER_HUMAN_REVIEW_STATUS_MISSING"));
        assert!(codes.contains(&"HEADLESS_RUNNER_RESULT_EXPORT_REF_INVALID"));
    }

    #[test]
    fn result_requires_schema_vocabulary_and_result_envelope_checksum() {
        let mut result = result();
        let envelope_checksum = result
            .checksums
            .iter_mut()
            .find(|checksum| checksum.payload_ref.ref_type == "result_envelope")
            .expect("fixture carries result-envelope checksum");
        envelope_checksum.algorithm = "blake3".to_string();
        envelope_checksum.canonicalization = "JCS-compatible-json".to_string();

        let validation = validate_result(&result);
        let codes: Vec<_> = validation
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();

        assert!(codes.contains(&"HEADLESS_RUNNER_REPRODUCIBILITY_MISSING"));
        assert!(codes.contains(&"HEADLESS_RUNNER_RESULT_ENVELOPE_CHECKSUM_MISSING"));
    }

    #[test]
    fn tp_phys_015_result_envelope_payload_validates_when_supplied() {
        let payload: Value = serde_json::from_str(TP_PHYS_015_RESULT_ENVELOPE)
            .expect("TP-PHYS-015 result envelope fixture parses");
        let result = tp_phys_015_runner_result();

        let validation = validate_result_with_optional_envelope_payload(&result, Some(&payload));

        assert!(!validation.has_blocking_diagnostics(), "{validation:?}");
        assert!(result
            .result_refs
            .iter()
            .any(|reference| reference.ref_id == "result:load-vector:node-N-1:uy"));
        assert!(result.checksums.iter().any(|checksum| {
            checksum.payload_ref.ref_type == "result_envelope"
                && checksum.payload_ref.ref_id == "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE"
        }));
    }

    #[test]
    fn full_payload_validation_blocks_envelope_ref_mismatch_and_missing_checksum() {
        let payload: Value = serde_json::from_str(TP_PHYS_015_RESULT_ENVELOPE)
            .expect("TP-PHYS-015 result envelope fixture parses");
        let mut result = tp_phys_015_runner_result();
        result.result_envelope_ref.envelope_ref.ref_id = "wrong-result-envelope".to_string();
        result
            .checksums
            .retain(|checksum| checksum.payload_ref.ref_type != "result_envelope");

        let validation = validate_result_with_optional_envelope_payload(&result, Some(&payload));
        let codes: Vec<_> = validation
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();

        assert!(codes.contains(&"HEADLESS_RUNNER_RESULT_ENVELOPE_REF_MISMATCH"));
        assert!(codes.contains(&"HEADLESS_RUNNER_RESULT_ENVELOPE_CHECKSUM_MISSING"));
    }

    #[test]
    fn reference_level_validation_still_accepts_without_full_payload() {
        let result = tp_phys_015_runner_result();

        let validation = validate_result_with_optional_envelope_payload(&result, None);

        assert!(!validation.has_blocking_diagnostics(), "{validation:?}");
    }

    #[test]
    fn privacy_and_professional_boundary_violations_are_blocking() {
        let mut result = result();
        result.privacy.telemetry_allowed = true;
        result.professional_boundary.software_makes_compliance_claim = true;
        let validation = validate_result(&result);
        let codes: Vec<_> = validation
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();
        assert!(codes.contains(&"HEADLESS_RUNNER_PRIVACY_BOUNDARY_VIOLATION"));
        assert!(codes.contains(&"HEADLESS_RUNNER_PROFESSIONAL_BOUNDARY_VIOLATION"));
    }

    #[test]
    fn preview_bridge_executes_product_physics_with_deterministic_refs() {
        let output = run_preview_in_memory(request(), preview_request());
        let mechanics = output
            .mechanics_envelope
            .as_ref()
            .expect("valid request should produce mechanics envelope");

        assert_eq!(mechanics.status.mechanics, "MECHANICS_SOLVED");
        assert!(output
            .runner_result
            .analysis_status
            .contains(&AnalysisStatus::MechanicsSolved));
        assert!(output
            .runner_result
            .result_refs
            .iter()
            .any(|reference| reference.ref_id == "result:stress:pipe-P-120:end-j:torsional-shear"));
        let envelope_ref_id = output
            .runner_result
            .result_envelope_ref
            .envelope_ref
            .ref_id
            .as_str();
        assert!(output.runner_result.checksums.iter().any(|checksum| {
            checksum.payload_ref.ref_type == "result_envelope"
                && checksum.payload_ref.ref_id == envelope_ref_id
                && checksum.value.len() == 64
        }));

        let validation = validate_result(&output.runner_result);
        assert!(!validation.has_blocking_diagnostics(), "{validation:?}");
    }

    #[test]
    fn preview_bridge_blocks_invalid_runner_metadata_before_solve() {
        let mut invalid = request();
        invalid.request_id.clear();
        let output = run_preview_in_memory(invalid, preview_request());

        assert!(output.mechanics_envelope.is_none());
        assert_eq!(output.runner_result.job.state, JobStateKind::Failed);
        assert!(output
            .runner_result
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "HEADLESS_RUNNER_REQUEST_ID_MISSING"));
    }

    #[test]
    fn analysis_status_for_rule_check_maps_the_runner_vocabulary() {
        assert_eq!(
            analysis_status_for_rule_check("RULE_INPUTS_INCOMPLETE"),
            Some(AnalysisStatus::RuleInputsIncomplete)
        );
        assert_eq!(
            analysis_status_for_rule_check("USER_RULE_CHECKED"),
            Some(AnalysisStatus::UserRuleChecked)
        );
        assert_eq!(
            analysis_status_for_rule_check("USER_RULE_FAILED"),
            Some(AnalysisStatus::UserRuleFailed)
        );
        // Not a rule-check status, and the empty string, map to nothing.
        assert_eq!(analysis_status_for_rule_check("MECHANICS_SOLVED"), None);
        assert_eq!(
            analysis_status_for_rule_check("HUMAN_REVIEW_REQUIRED"),
            None
        );
        assert_eq!(analysis_status_for_rule_check(""), None);
    }

    #[test]
    fn preview_bridge_drives_user_rule_failed_into_envelope_and_analysis_status() {
        let output = run_preview_in_memory_with_rule_check(
            request(),
            preview_request(),
            Some("USER_RULE_FAILED"),
        );
        let mechanics = output
            .mechanics_envelope
            .as_ref()
            .expect("valid request should produce mechanics envelope");
        // Driven into the solve envelope (before its checksum binds)...
        assert_eq!(mechanics.status.rule_check, "USER_RULE_FAILED");
        // ...and reflected in the runner analysis_status, replacing the
        // solve-only RULE_INPUTS_INCOMPLETE default.
        assert!(output
            .runner_result
            .analysis_status
            .contains(&AnalysisStatus::UserRuleFailed));
        assert!(!output
            .runner_result
            .analysis_status
            .contains(&AnalysisStatus::RuleInputsIncomplete));
        // The result still validates (human-review-required preserved; no
        // blocking diagnostics for a recognized aggregate).
        assert!(output.runner_result.diagnostics.is_empty());
        let validation = validate_result(&output.runner_result);
        assert!(!validation.has_blocking_diagnostics(), "{validation:?}");
    }

    #[test]
    fn preview_bridge_drives_user_rule_checked_into_analysis_status() {
        let output = run_preview_in_memory_with_rule_check(
            request(),
            preview_request(),
            Some("USER_RULE_CHECKED"),
        );
        let mechanics = output.mechanics_envelope.as_ref().expect("envelope");
        assert_eq!(mechanics.status.rule_check, "USER_RULE_CHECKED");
        assert!(output
            .runner_result
            .analysis_status
            .contains(&AnalysisStatus::UserRuleChecked));
        assert!(!output
            .runner_result
            .analysis_status
            .contains(&AnalysisStatus::RuleInputsIncomplete));
    }

    #[test]
    fn preview_bridge_incomplete_aggregate_equals_the_no_aggregate_default() {
        let driven = run_preview_in_memory_with_rule_check(
            request(),
            preview_request(),
            Some("RULE_INPUTS_INCOMPLETE"),
        );
        let default = run_preview_in_memory(request(), preview_request());
        // Explicitly passing the solve-only default is identical to passing None.
        assert_eq!(
            driven.runner_result.analysis_status,
            default.runner_result.analysis_status
        );
        assert!(driven
            .runner_result
            .analysis_status
            .contains(&AnalysisStatus::RuleInputsIncomplete));
        assert!(driven.runner_result.diagnostics.is_empty());
    }

    #[test]
    fn preview_bridge_blocks_unrecognized_rule_check_aggregate() {
        let output = run_preview_in_memory_with_rule_check(
            request(),
            preview_request(),
            Some("DEFINITELY_NOT_A_STATUS"),
        );
        let mechanics = output.mechanics_envelope.as_ref().expect("envelope");
        // Conservative: the envelope is never coerced by an unknown status; it
        // stays at the solve-only RULE_INPUTS_INCOMPLETE (no false pass).
        assert_eq!(mechanics.status.rule_check, "RULE_INPUTS_INCOMPLETE");
        assert!(output.runner_result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "HEADLESS_RUNNER_RULE_CHECK_STATUS_INVALID"
                && diagnostic.severity == DiagnosticSeverity::Blocking
        }));
        // The analysis_status still reflects the conservative default.
        assert!(output
            .runner_result
            .analysis_status
            .contains(&AnalysisStatus::RuleInputsIncomplete));
    }
}

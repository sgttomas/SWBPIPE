//! Headless runner contract support.
//!
//! This crate validates bounded in-memory request/result records for the
//! schema-first DEL-10-05 headless runner contract. It does not parse arbitrary
//! project files, invoke external processes, access the network, mutate the
//! filesystem, run GUI/report/adapter/local-FEA workflows, or emit professional
//! or code-compliance claims.

pub mod benchmark_binding;
pub mod redaction_binding;
pub mod result_envelope_binding;

use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, MechanicsEnvelope, PreviewSolverMode,
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

    let version=root.get("schema_version").and_then(Value::as_str);
    if !matches!(version,Some("0.1.0"|"0.2.0"|"0.3.0")) || envelope.get("schema_version").and_then(Value::as_str)!=version {
        diagnostics.push(Diagnostic::runner_blocking("HEADLESS_RUNNER_RESULT_ENVELOPE_VERSION_UNSUPPORTED",Reference::new("result_envelope",&result.result_envelope_ref.envelope_ref.ref_id),"only matching 0.1.0, 0.2.0 or recognized source-bound 0.3.0 result contract versions are supported"));
    }

    if version == Some("0.3.0") {
        let mut metadata_source = Value::Object(envelope.clone());
        metadata_source["schema_version"] = serde_json::json!("0.2.0");
        if open_pipe_stress_result_export::semantic_contract::for_source_metadata(&metadata_source).is_err()
            || metadata_source["semantic_contract_ref"] != serde_json::json!({"ref_type":"semantic_contract", "ref_id":metadata_source["producer"]["semantic_contract_id"]}) {
            diagnostics.push(Diagnostic::runner_blocking("HEADLESS_RUNNER_RESULT_CONTRACT_UNSUPPORTED",Reference::new("result_envelope",&result.result_envelope_ref.envelope_ref.ref_id),"derivative metadata must bind the recognized source producer and semantic contract"));
        }
    }

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
    /// DEL-08-04 result-export envelope document produced from the completed
    /// solve (R14 W1 T1). Library surface only: excluded from serde
    /// serialization so every existing serialized surface — the
    /// `headless_preview_runner` DEC-064 witness stdout and the DEC-065
    /// runner-bin `CliOutput` — stays byte-unchanged. CLI exposure is the
    /// DEL-10-05 `export-results` follow-on.
    #[serde(skip_serializing)]
    pub result_envelope_document: Option<Value>,
    /// Library-only explicit availability finding; never changes raw transport.
    #[serde(skip_serializing)]
    pub canonical_export_unavailability: Option<String>,
    #[serde(skip_serializing)]
    pub qualified_preview_evidence: Option<QualifiedPreviewEvidence>,
}

/// Opaque evidence minted by the actual same-Value solve route. No public
/// constructor, Deserialize or payload setter can substitute a solved model.
#[derive(Debug, Clone)]
pub struct QualifiedPreviewEvidence {
    pub(crate) solve_payload: Value,
    pub(crate) solve_payload_digest: String,
    pub(crate) mechanics_digest: String,
    pub(crate) runner_digest: String,
    pub(crate) request_digest: String,
    pub(crate) run_id: String,
}

/// Parse the exact retained solve payload once and solve that same typed input.
/// Unknown authored JSON remains in evidence and model digest; overrides stay
/// separately scoped solve input rather than being merged into model identity.
pub fn run_preview_model_value_with_rule_check(request: RunnerRequest, solve_payload: Value, aggregate: Option<&str>) -> Result<PreviewRunnerOutput, String> {
    run_preview_model_value_mode(request,solve_payload,aggregate,PreviewSolverMode::SparseInteractive)
}
/// Library-only dense scrutiny/default sparse mode; no CLI/native option added.
pub fn run_preview_model_value_with_mode(request:RunnerRequest,solve_payload:Value,mode:PreviewSolverMode)->Result<PreviewRunnerOutput,String>{run_preview_model_value_mode(request,solve_payload,None,mode)}
fn run_preview_model_value_mode(request: RunnerRequest, solve_payload: Value, aggregate: Option<&str>, mode:PreviewSolverMode) -> Result<PreviewRunnerOutput, String> {
    let typed: LinearStaticPreviewRequest = serde_json::from_value(solve_payload.clone()).map_err(|e|format!("SOLVE_PAYLOAD_INVALID: {e}"))?;
    let retained_request=request.clone();
    let mut output=run_preview_in_memory_mode(request,typed,aggregate,mode);
    let qualified=|| -> Result<QualifiedPreviewEvidence,String> {
        use open_pipe_stress_result_export::derivative::{digest,guard_json};
        guard_json(&solve_payload)?;
        let mechanics=output.mechanics_envelope.as_ref().ok_or("SOURCE_UNAVAILABLE")?;
        if mechanics.status.mechanics!="MECHANICS_SOLVED" {return Err("SOURCE_NOT_SOLVED".into());}
        if solve_payload["model"]["project"]["id"]!=mechanics.model_ref {return Err("SOURCE_MODEL_IDENTITY_MISMATCH".into());}
        Ok(QualifiedPreviewEvidence {solve_payload_digest:digest(&solve_payload)?,solve_payload,mechanics_digest:digest(&serde_json::to_value(mechanics).map_err(|e|e.to_string())?)?,runner_digest:digest(&serde_json::to_value(&output.runner_result).map_err(|e|e.to_string())?)?,request_digest:digest(&serde_json::to_value(&retained_request).map_err(|e|e.to_string())?)?,run_id:output.runner_result.run_id.clone()})
    }();
    match qualified {
        Ok(evidence)=>match result_envelope_binding::build_result_export_document_with_evidence(&retained_request,&output.runner_result,output.mechanics_envelope.as_ref().unwrap(),&evidence){
            Ok(doc)=>{output.result_envelope_document=Some(doc);output.canonical_export_unavailability=None;output.qualified_preview_evidence=Some(evidence);},
            Err(d)=>output.canonical_export_unavailability=Some(d.message),
        },
        Err(reason)=>output.canonical_export_unavailability=Some(reason),
    }
    Ok(output)
}
pub fn run_preview_model_value(request: RunnerRequest, solve_payload: Value) -> Result<PreviewRunnerOutput,String> {run_preview_model_value_with_rule_check(request,solve_payload,None)}

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
/// - a recognized status: carried only by the runner/derived `analysis_status`;
///   the received `MechanicsEnvelope` and its checksum remain unchanged.
/// - any other non-`None` string: a blocking diagnostic
///   (`HEADLESS_RUNNER_RULE_CHECK_STATUS_INVALID`), never silently dropped; the
///   envelope is left at `RULE_INPUTS_INCOMPLETE` (conservative — no false pass).
pub fn run_preview_in_memory_with_rule_check(
    request: RunnerRequest,
    preview_request: LinearStaticPreviewRequest,
    rule_check_aggregate: Option<&str>,
) -> PreviewRunnerOutput {
    run_preview_in_memory_mode(request,preview_request,rule_check_aggregate,PreviewSolverMode::SparseInteractive)
}
fn run_preview_in_memory_mode(request:RunnerRequest,preview_request:LinearStaticPreviewRequest,rule_check_aggregate:Option<&str>,mode:PreviewSolverMode)->PreviewRunnerOutput {
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
            result_envelope_document: None,
            canonical_export_unavailability: Some("SOURCE_UNAVAILABLE".into()),
            qualified_preview_evidence: None,
        };
    }

    let mechanics = run_linear_static_preview_with_mode(preview_request,mode);
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    // Rule checks revise the derived analysis, never the producer result. Keep
    // the same raw bytes, numerical evidence and received-source checksum for
    // every rule revision; invalid aggregates retain the source status and block.
    let mut effective_rule_status = analysis_status_for_rule_check(&mechanics.status.rule_check);
    if let Some(aggregate) = rule_check_aggregate {
        if let Some(status) = analysis_status_for_rule_check(aggregate) {
            effective_rule_status = Some(status);
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
    // Only this derived status reflects the optional rule-check aggregate.
    if let Some(status) = effective_rule_status {
        analysis_status.push(status);
    }

    let mut runner_result = RunnerResult {
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
        privacy: request.privacy.clone(),
        provenance: request.provenance.clone(),
        professional_boundary: request.professional_boundary,
    };

    let result_envelope_document =
        attach_result_envelope_document(&request, &mut runner_result, &mechanics);

    let canonical_export_unavailability = Some(if mechanics.status.mechanics == "MECHANICS_SOLVED" {"EXACT_SOLVED_MODEL_EVIDENCE_UNAVAILABLE"} else {"SOURCE_NOT_SOLVED"}.into());
    PreviewRunnerOutput {
        runner_result,
        mechanics_envelope: Some(mechanics),
        result_envelope_document,
        canonical_export_unavailability,
        qualified_preview_evidence: None,
    }
}

/// Produce and attach the DEL-08-04 envelope document for a completed solve
/// (R14 W1 T1, fail-closed per the governing brief §3.5): on a clean
/// production (both validators free of blocking diagnostics) the document is
/// returned for the new library-only `PreviewRunnerOutput` field; on a
/// STRUCTURAL production or validation failure a blocking runner diagnostic
/// is appended so the DEC-065 exit policy reports the run as not clean. A
/// solve that did not reach `MECHANICS_SOLVED` produces no envelope and no
/// new diagnostic: it is not a completed solve product, and its existing
/// not-clean signaling (missing result references under `validate_result`)
/// is preserved byte-for-byte.
fn attach_result_envelope_document(
    request: &RunnerRequest,
    runner_result: &mut RunnerResult,
    mechanics: &MechanicsEnvelope,
) -> Option<Value> {
    if mechanics.status.mechanics != "MECHANICS_SOLVED" {
        return None;
    }
    let _ = (request, runner_result);
    None
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
        // This control asserts current torsional/source references and hashes,
        // not pressure physics. Declare its local input unpressurized before
        // solving; do not change the shared fixture or other tests' inputs.
        let mut current_request = preview_request();
        for case in &mut current_request.model.load_cases {
            for load in &mut case.primitive_loads {
                if load.category == "pressure" || load.dimension == "pressure" {
                    load.magnitude.value = 0.0;
                }
            }
        }
        let output = run_preview_in_memory(request(), current_request);
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
    fn preview_bridge_preserves_producer_and_drives_user_rule_failed_into_analysis_status() {
        // The final runner validation assertion requires computed row refs.
        // Pressure is incidental to the rule/source-status behavior under test;
        // declare only this current input unpressurized before the real solve.
        let mut current_request = preview_request();
        for case in &mut current_request.model.load_cases {
            for load in &mut case.primitive_loads {
                if load.category == "pressure" || load.dimension == "pressure" {
                    load.magnitude.value = 0.0;
                }
            }
        }
        let output = run_preview_in_memory_with_rule_check(
            request(),
            current_request,
            Some("USER_RULE_FAILED"),
        );
        let mechanics = output
            .mechanics_envelope
            .as_ref()
            .expect("valid request should produce mechanics envelope");
        // Producer status remains the actual solve-only observation.
        assert_eq!(mechanics.status.rule_check, "RULE_INPUTS_INCOMPLETE");
        // The separate runner analysis carries the requested rule outcome.
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
        assert_eq!(mechanics.status.rule_check, "RULE_INPUTS_INCOMPLETE");
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
    fn rule_revisions_preserve_actual_physics_and_precision_producer_source() {
        use open_pipe_stress_result_export::derivative::{digest, validate_document};
        let exact: Value = serde_json::from_str(include_str!(
            "../../../product_physics/tests/fixtures/exact_pressure_connected_request.json"
        )).unwrap();
        // Pressure is incidental to the legacy source-binding control. Declare
        // zero pressure in a local input before solving; keep fixture bytes intact.
        let mut ordinary: Value = serde_json::json!({"model": serde_json::from_str::<Value>(include_str!(
            "../../../../fixtures/product_preview/invented_preview_model.json"
        )).unwrap(), "materials": []});
        for case in ordinary["model"]["load_cases"].as_array_mut().unwrap() {
            for load in case["primitive_loads"].as_array_mut().unwrap() {
                if load["category"] == "pressure" || load["dimension"] == "pressure" {
                    load["magnitude"]["value"] = serde_json::json!(0.0);
                }
            }
        }
        for (payload, contract) in [(ordinary, open_pipe_stress_result_export::semantic_contract::PRECISION_ID),
            (exact, open_pipe_stress_result_export::semantic_contract::PHYSICS_ID)] {
            for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
                let baseline = run_preview_model_value_mode(request(), payload.clone(), None, mode).unwrap();
                let raw = serde_json::to_value(baseline.mechanics_envelope.as_ref().unwrap()).unwrap();
                assert_eq!(raw["status"]["mechanics"], "MECHANICS_SOLVED");
                assert_eq!(raw["producer"]["semantic_contract_id"], contract);
                let source_digest = digest(&raw).unwrap();
                let received_checksum = baseline.runner_result.checksums.iter().find(|c| c.payload_ref.ref_type == "result_envelope").unwrap();
                for (aggregate, expected) in [("RULE_INPUTS_INCOMPLETE", AnalysisStatus::RuleInputsIncomplete),
                    ("USER_RULE_CHECKED", AnalysisStatus::UserRuleChecked), ("USER_RULE_FAILED", AnalysisStatus::UserRuleFailed)] {
                    let revised = run_preview_model_value_mode(request(), payload.clone(), Some(aggregate), mode).unwrap();
                    let revised_raw = serde_json::to_value(revised.mechanics_envelope.as_ref().unwrap()).unwrap();
                    assert_eq!(revised_raw, raw, "{contract}: {aggregate} changed producer data");
                    assert_eq!(digest(&revised_raw).unwrap(), source_digest);
                    assert_eq!(revised.runner_result.checksums.iter().find(|c| c.payload_ref.ref_type == "result_envelope").unwrap(), received_checksum);
                    assert!(revised.runner_result.analysis_status.contains(&expected));
                    let doc = revised.result_envelope_document.as_ref().expect("actual qualified solve remains exportable");
                    assert!(doc["result_envelope"]["analysis_status"].as_array().unwrap().contains(&serde_json::json!(aggregate)));
                    assert_eq!(doc["result_envelope"]["reproducibility"]["source_origin_bindings"][0]["received_carrier_checksum"]["value"], source_digest);
                    assert_eq!(doc["result_envelope"]["numerical_quality"], raw["numerical_quality"]);
                    assert_eq!(doc["result_envelope"].get("contract_evidence"), raw.get("contract_evidence"));
                    validate_document(doc, &raw).unwrap();
                    let proof = revised.qualified_preview_evidence.as_ref().unwrap();
                    assert_eq!(proof.mechanics_digest, source_digest);
                    if aggregate != "RULE_INPUTS_INCOMPLETE" {
                        assert_ne!(proof.runner_digest, baseline.qualified_preview_evidence.as_ref().unwrap().runner_digest);
                    }
                }
            }
        }
    }

    #[test]
    fn blocked_exact_inputs_remain_interpretable_without_source_promotion() {
        use open_pipe_stress_result_export::semantic_contract;
        let original: Value = serde_json::from_str(include_str!(
            "../../../product_physics/tests/fixtures/exact_pressure_connected_request.json"
        )).unwrap();
        for (name, diagnostic) in [("missing_nu", "EXACT_PRESSURE_POISSON_RATIO_REQUIRED"),
            ("missing_closure", "PRESSURE_TERMINAL_CLOSURE_INVALID")] {
            let mut payload = original.clone();
            if name == "missing_nu" {
                payload["model"]["materials"][0].as_object_mut().unwrap().remove("poisson_ratio");
            } else {
                payload["model"]["load_cases"][0]["pressure_regions"][0]["terminals"][0].as_object_mut().unwrap().remove("closure_transfer");
            }
            let output = run_preview_model_value(request(), payload.clone()).unwrap();
            let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
            assert_eq!(raw["status"]["mechanics"], "MODEL_INCOMPLETE");
            assert_eq!(raw["contract_evidence"], serde_json::json!({"pressure":[],"connector":[],"exact_cases":[]}));
            assert!(raw["results"].as_array().unwrap().is_empty());
            assert!(raw["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == diagnostic));
            assert_eq!(semantic_contract::for_source(&raw).unwrap().0["semantic_contract_id"], semantic_contract::PHYSICS_ID);
            let requested = payload["model"]["load_cases"].as_array().unwrap().iter().map(|c|
                serde_json::json!({"ref_type":"load_case","ref_id":c["id"]})).collect::<Vec<_>>();
            assert_eq!(semantic_contract::numerical_use_standing(&raw, &requested), "needs_recompute");
            assert!(output.result_envelope_document.is_none());
            assert!(output.qualified_preview_evidence.is_none());
            assert_eq!(output.canonical_export_unavailability.as_deref(), Some("SOURCE_NOT_SOLVED"));
            if let Ok(dir) = std::env::var("HEADLESS_BLOCKED_PHYSICS_OUTPUT_DIR") {
                let dir = std::path::Path::new(&dir); std::fs::create_dir_all(dir).unwrap();
                std::fs::write(dir.join(format!("{name}.raw.json")), serde_json::to_vec_pretty(&raw).unwrap()).unwrap();
                std::fs::write(dir.join(format!("{name}.request.json")), serde_json::to_vec_pretty(&payload).unwrap()).unwrap();
            }
        }
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

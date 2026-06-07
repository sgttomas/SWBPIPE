//! Headless runner contract support.
//!
//! This crate validates bounded in-memory request/result records for the
//! schema-first DEL-10-05 headless runner contract. It does not parse arbitrary
//! project files, invoke external processes, access the network, mutate the
//! filesystem, run GUI/report/adapter/local-FEA workflows, or emit professional
//! or code-compliance claims.

use open_pipe_stress_product_physics::{
    run_linear_static_preview, LinearStaticPreviewRequest, MechanicsEnvelope,
};
use serde::Serialize;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AnalysisStatus {
    ModelIncomplete,
    MechanicsSolved,
    RuleInputsIncomplete,
    UserRuleChecked,
    UserRuleFailed,
    HumanReviewRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RunnerOperation {
    Solve,
    ValidateInput,
    ExportResults,
    RunBenchmark,
    RunRegression,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum JobStateKind {
    Queued,
    Running,
    CancellationRequested,
    Completed,
    Failed,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PrivacyClass {
    PublicMetadata,
    PrivateProjectData,
    PrivateRulePackData,
    Redacted,
    ProtectedSuspected,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RedistributionStatus {
    PublicPermissive,
    PrivateOnly,
    Unknown,
    ProtectedSuspected,
    InventedNonEngineeringExample,
    Tbd,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChecksumRef {
    pub algorithm: String,
    pub canonicalization: String,
    pub payload_ref: Reference,
    pub value: String,
}

impl ChecksumRef {
    fn is_complete(&self) -> bool {
        matches!(self.algorithm.trim(), "sha256" | "sha512")
            && matches!(self.canonicalization.trim(), "JCS" | "NONE")
            && self.payload_ref.is_complete()
            && !self.value.trim().is_empty()
            && !self.value.trim().eq_ignore_ascii_case("TBD")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
    fn runner_blocking(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TbdDecisions {
    pub final_cli_command_syntax: bool,
    pub package_scripts: bool,
    pub process_invocation: bool,
    pub network_access: bool,
    pub filesystem_mutation_policy: bool,
    pub ci_provider: bool,
    pub release_matrix: bool,
    pub public_transport_protocol: bool,
    pub external_adapter_formats: bool,
    pub physical_project_container: bool,
}

impl TbdDecisions {
    pub fn all_deferred() -> Self {
        Self {
            final_cli_command_syntax: true,
            package_scripts: true,
            process_invocation: true,
            network_access: true,
            filesystem_mutation_policy: true,
            ci_provider: true,
            release_matrix: true,
            public_transport_protocol: true,
            external_adapter_formats: true,
            physical_project_container: true,
        }
    }

    fn preserves_deferred_surface(&self) -> bool {
        self.final_cli_command_syntax
            && self.package_scripts
            && self.process_invocation
            && self.network_access
            && self.filesystem_mutation_policy
            && self.ci_provider
            && self.release_matrix
            && self.public_transport_protocol
            && self.external_adapter_formats
            && self.physical_project_container
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct JobState {
    pub job_id: String,
    pub state: JobStateKind,
    pub current_step: u32,
    pub total_steps: u32,
    pub cancellation_supported: bool,
    pub cancellation_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

    if !request.tbd_decisions.preserves_deferred_surface() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_TBD_SURFACE_OVERSPECIFIED",
            Reference::new("runner_request", &request.request_id),
            "final CLI syntax, package scripts, process/network/filesystem policy, CI, release, transport, adapter, and container decisions must remain deferred in DEL-10-05",
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

pub fn run_preview_in_memory(
    request: RunnerRequest,
    preview_request: LinearStaticPreviewRequest,
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

    let mechanics = run_linear_static_preview(preview_request);
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
    if mechanics.status.rule_check == "RULE_INPUTS_INCOMPLETE" {
        analysis_status.push(AnalysisStatus::RuleInputsIncomplete);
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
        diagnostics: Vec::new(),
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

fn checksum_ref<T: Serialize>(ref_type: &str, ref_id: &str, value: &T) -> ChecksumRef {
    ChecksumRef {
        algorithm: "sha256".to_string(),
        canonicalization: "JCS".to_string(),
        payload_ref: Reference::new(ref_type, ref_id),
        value: sha256_hex(&canonical_json(value)),
    }
}

fn canonical_json<T: Serialize>(value: &T) -> String {
    let value = serde_json::to_value(value).expect("runner values must serialize");
    serde_json::to_string(&sort_json(value)).expect("runner values must encode as JSON")
}

fn sort_json(value: Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.into_iter().map(sort_json).collect()),
        Value::Object(object) => {
            let mut sorted = Map::new();
            let mut entries = object.into_iter().collect::<Vec<_>>();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            for (key, value) in entries {
                sorted.insert(key, sort_json(value));
            }
            Value::Object(sorted)
        }
        scalar => scalar,
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
            canonicalization: "JCS".to_string(),
            payload_ref: Reference::new("payload", id),
            value: format!("{id}-hash"),
        }
    }

    fn result_envelope_checksum(id: &str) -> ChecksumRef {
        ChecksumRef {
            algorithm: "sha256".to_string(),
            canonicalization: "JCS".to_string(),
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
            tbd_decisions: TbdDecisions::all_deferred(),
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
                    canonicalization: "JCS".to_string(),
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
    fn request_requires_load_basis_and_deferred_surface() {
        let mut request = request();
        request.load_basis_refs.clear();
        request.tbd_decisions.final_cli_command_syntax = false;
        let validation = validate_request(&request);
        let codes: Vec<_> = validation
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();
        assert!(codes.contains(&"HEADLESS_RUNNER_LOAD_BASIS_MISSING"));
        assert!(codes.contains(&"HEADLESS_RUNNER_TBD_SURFACE_OVERSPECIFIED"));
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
}

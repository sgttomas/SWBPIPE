//! File > Save Report Package… native admission (DEL-08-01 Design
//! `report-package-current-results.md` §2.6, W-18a slice (b1); FR-REP-001,
//! DEL-12-02 REXC-REQ-006).
//!
//! The webview sends the package request and the session's model as their
//! RFC 8785 (JCS) canonical texts, with the route evidence of
//! `DREP-PACKAGE-SAVE-009`. Evidence is never trusted for content. Before the
//! save dialog is shown, [`admit_report_package`]
//! 1. re-checks the screening and local-first evidence (local-private,
//!    explicit intent, lossless, not withheld, exact payload, zero blocking):
//!    defence in depth, since intent is the user's webview choice;
//! 2. applies the size caps, then binds both texts: each must be its own
//!    canonical form, the screening evidence must name the request text's
//!    SHA-256 (`REPORT-PACKAGE-SAVE-EVIDENCE-UNBOUND`), and the model text's
//!    SHA-256 must be the request's audit-manifest model hash
//!    (`REPORT-PACKAGE-SAVE-MODEL-UNBOUND`);
//! 3. screens the dispositions of both parsed values itself (the request: the
//!    report input, result document, source carrier, audit manifest and
//!    handoff records; and the model), with every `library_ref` resolved in
//!    the model project's local library store; a store read error refuses;
//! 4. binds the raw report context and the model text: `local_private`, the
//!    desktop export profile, the model's project id, name and units, and the
//!    application's own name and version;
//! 5. converts the request to its typed form and runs the shared producer
//!    (`report_package::wire`, §2.3), refusing a blocked package.
//!
//! Admission is a pure function of its arguments and the library lookup, so
//! its tests need no window. The command (`lib.rs` `save_report_package`)
//! prices the parse of both texts in its ingress lease before admission.
//! Nothing is sent anywhere (F-PIP-1). Standard claim fence applies (claims
//! taxonomy per DEC-081).

use crate::atomic_report_package_save::{self, AtomicSaveError};
use crate::command_ingress::{Args, ArgsDecoding, ValueDecoder, REPORT_PACKAGE_VEC_EXPANSION};
use crate::model_document_file::MAX_MODEL_DOCUMENT_BYTES;
use crate::model_document_save::{screen_dispositions_in, LibraryLookup, ModelDocumentSaveDiagnostic as Diagnostic};
use crate::report_package_bridge::{self, ReportPackageRequest};
use crate::solve_jobs::SolveJobError;
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use open_pipe_stress_report_package::bindings::{DESKTOP_PROFILE, REPORT_CONTEXT_NOT_LOCAL_PRIVATE};
use open_pipe_stress_report_package::ReportPackageContainerOutcome;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

/// The governed local-first route of this save.
pub const REPORT_PACKAGE_SAVE_ROUTE_ID: &str = "DREP-PACKAGE-SAVE-009";

pub const REDACTION_EVIDENCE_INVALID: &str = "REPORT-PACKAGE-REDACTION-EVIDENCE-INVALID";
pub const LOCAL_FIRST_EVIDENCE_INVALID: &str = "REPORT-PACKAGE-LOCAL-FIRST-EVIDENCE-INVALID";
pub const TOO_LARGE: &str = "REPORT-PACKAGE-SAVE-TOO-LARGE";
pub const NOT_CANONICAL: &str = "REPORT-PACKAGE-SAVE-NOT-CANONICAL";
pub const EVIDENCE_UNBOUND: &str = "REPORT-PACKAGE-SAVE-EVIDENCE-UNBOUND";
pub const MODEL_UNBOUND: &str = "REPORT-PACKAGE-SAVE-MODEL-UNBOUND";
pub const LIBRARY_STORE_UNAVAILABLE: &str = "REPORT-PACKAGE-SAVE-LIBRARY-STORE-UNAVAILABLE";
pub const PROFILE_NOT_DESKTOP: &str = "REPORT-PACKAGE-SAVE-PROFILE-NOT-DESKTOP";
pub const MODEL_MISMATCH: &str = "REPORT-PACKAGE-SAVE-MODEL-MISMATCH";
pub const SOFTWARE_MISMATCH: &str = "REPORT-PACKAGE-SAVE-SOFTWARE-MISMATCH";
pub const REQUEST_INVALID: &str = "REPORT-PACKAGE-SAVE-REQUEST-INVALID";
pub const PRODUCER_BLOCKED: &str = "REPORT-PACKAGE-PRODUCER-BLOCKED";
pub const PICKER_PATH_INVALID: &str = "REPORT-PACKAGE-PICKER-PATH-INVALID";
pub const WORKER_FAILED: &str = "REPORT-PACKAGE-SAVE-WORKER-FAILED";
/// The webview screener's code for a blocked disposition (`redactionExportControls.ts`).
pub use crate::model_document_save::PROTECTED_CONTENT_BLOCKED;

/// The largest request text admitted. The request carries the report input,
/// the result document, its source carrier and the handoff records, each a
/// whole-run document, so its cap is four model-document caps.
pub const MAX_REQUEST_TEXT_BYTES: u64 = 4 * MAX_MODEL_DOCUMENT_BYTES;
/// The model text's cap: the model-document open cap (W-6a).
pub const MAX_MODEL_TEXT_BYTES: u64 = MAX_MODEL_DOCUMENT_BYTES;

/// The webview's screening evidence: W-6a's `ModelDocumentSaveScreening` form
/// (the `ControlledExportLink` of a lossless local-private route) plus the
/// decision and finding counts the receipt reports.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ReportPackageSaveScreening {
    pub route_id: String,
    pub export_context: String,
    pub explicit_local_private_intent: bool,
    pub blocked: bool,
    pub materialization_withheld: bool,
    pub lossless_required: bool,
    pub exact_payload_match: bool,
    pub blocking_count: usize,
    pub decision_count: usize,
    pub finding_count: usize,
    /// `sha256:<hex>` of the request text the webview screened; it must be the
    /// SHA-256 of the text admitted.
    pub payload_sha256: String,
}

/// The metadata-only local-first route evidence (`enforceLocalFirstRoute`),
/// unchanged from the 1.0.0 command.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ReportPackageLocalFirstEvidence {
    pub route_id: String,
    pub export_context: String,
    pub storage_context: String,
    pub action: String,
    pub reason_code: String,
    pub blocked: bool,
    pub metadata_only: bool,
    pub explicit_local_private_intent: bool,
}

/// `{requestCanonicalText, modelCanonicalText, screening, localFirstEvidence?}`.
#[derive(Debug)]
pub struct ReportPackageSaveRequest {
    pub request_text: String,
    pub model_text: String,
    pub screening: ReportPackageSaveScreening,
    pub local_first: Option<ReportPackageLocalFirstEvidence>,
}

/// Decodes the command's raw-body arguments (strings and structs of scalars,
/// moved: [`ArgsDecoding::Moved`]).
pub(crate) fn save_report_package_args(args: &mut Args) -> Result<ReportPackageSaveRequest, SolveJobError> {
    Ok(ReportPackageSaveRequest {
        request_text: args.decode("requestCanonicalText")?,
        model_text: args.decode("modelCanonicalText")?,
        screening: args.decode("screening")?,
        local_first: args.decode("localFirstEvidence")?,
    })
}

/// The application identity the report context must name: the compiled Tauri
/// context's `productName` and `version` (`app.package_info()`), as the
/// desktop builder reads them from `tauri.conf.json`.
#[derive(Clone, Debug)]
pub struct ExpectedSoftware {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct ReportPackageMemberSummary {
    pub role: String,
    pub file_name: String,
    pub sha256_hex: String,
    pub byte_length: usize,
}

#[derive(Debug, Serialize)]
pub struct ReportPackageSaveReceipt {
    pub outcome: String,
    pub stage: String,
    pub code: String,
    pub container_file_name: String,
    pub byte_count: usize,
    pub package_identity_sha256_hex: String,
    pub container_sha256_hex: String,
    pub members: Vec<ReportPackageMemberSummary>,
    pub replaced_existing: bool,
    pub durability: String,
    pub redaction_route_id: String,
    pub redaction_decision_count: usize,
    pub redaction_finding_count: usize,
    pub redaction_blocking_count: usize,
    pub local_first_route_id: String,
    pub local_first_export_context: String,
    pub local_first_storage_context: String,
    pub local_first_action: String,
    pub local_first_reason_code: String,
    pub local_first_blocked: bool,
    pub local_first_metadata_only: bool,
    pub local_first_explicit_local_private_intent: bool,
    pub selected_basename: String,
    pub path_containment: String,
    pub limitation: String,
    /// Non-blocking findings of the native screen, e.g.
    /// `MODEL-LIBRARY-REF-UNRESOLVED`.
    pub warnings: Vec<Diagnostic>,
}

#[derive(Debug, Serialize)]
pub struct ReportPackageSaveCommandError {
    pub code: String,
    pub stage: String,
    pub message: String,
    pub os_error_kind: Option<String>,
    pub os_error_code: Option<i32>,
    pub cleanup_code: Option<String>,
    pub cleanup_message: Option<String>,
    pub selected_basename: String,
    pub path_containment: String,
    pub limitation: String,
    /// Every finding of a native admission refusal, with its path: a binding's
    /// JSON pointer under `/request` or `/model`, or a screen's member path
    /// rooted at `request` or `model`. Empty for producer and writer errors.
    pub diagnostics: Vec<Diagnostic>,
}

impl ReportPackageSaveCommandError {
    /// A `CODE: message` error of the shared producer or the bridge.
    pub fn bridge(message: String) -> Self {
        Self::with_stage(message.split(':').next().unwrap_or("REPORT-PACKAGE-BRIDGE-FAILED").to_string(), "assemble", message, Vec::new())
    }

    /// A native admission refusal: the first finding's code, every finding.
    pub fn refused(diagnostics: Vec<Diagnostic>) -> Self {
        let code = diagnostics.first().map(|item| item.code.clone()).unwrap_or_else(|| "REPORT-PACKAGE-SAVE-REFUSED".to_string());
        let message = diagnostics
            .iter()
            .map(|item| if item.path.is_empty() { format!("{}: {}", item.code, item.message) } else { format!("{} at {}: {}", item.code, item.path, item.message) })
            .collect::<Vec<_>>()
            .join(" | ");
        Self::with_stage(code, "admit", message, diagnostics)
    }

    fn with_stage(code: String, stage: &str, message: String, diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            code,
            stage: stage.to_string(),
            message,
            os_error_kind: None,
            os_error_code: None,
            cleanup_code: None,
            cleanup_message: None,
            selected_basename: String::new(),
            path_containment: atomic_report_package_save::PATH_CONTAINMENT.to_string(),
            limitation: atomic_report_package_save::TOCTOU_LIMITATION.to_string(),
            diagnostics,
        }
    }
}

impl From<AtomicSaveError> for ReportPackageSaveCommandError {
    fn from(value: AtomicSaveError) -> Self {
        Self {
            code: value.code,
            stage: value.stage,
            message: value.message,
            os_error_kind: value.os_error_kind,
            os_error_code: value.os_error_code,
            cleanup_code: value.cleanup_code,
            cleanup_message: value.cleanup_message,
            selected_basename: value.selected_basename,
            path_containment: value.path_containment,
            limitation: value.limitation,
            diagnostics: Vec::new(),
        }
    }
}

fn finding(code: &str, path: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic { code: code.to_string(), path: path.to_string(), message: message.into() }
}

fn refuse(code: &str, path: &str, message: impl Into<String>) -> ReportPackageSaveCommandError {
    ReportPackageSaveCommandError::refused(vec![finding(code, path, message)])
}

/// Re-checks the route evidence (§2.6 item 2): defence in depth, not caller
/// authentication.
pub fn validate_report_package_save_evidence(
    screening: &ReportPackageSaveScreening,
    local_first: Option<&ReportPackageLocalFirstEvidence>,
) -> Result<(), ReportPackageSaveCommandError> {
    let s = screening;
    let screened = s.route_id == REPORT_PACKAGE_SAVE_ROUTE_ID
        && s.export_context == "local_private"
        && s.explicit_local_private_intent
        && !s.blocked
        && !s.materialization_withheld
        && s.lossless_required
        && s.exact_payload_match
        && s.blocking_count == 0;
    if !screened {
        return Err(refuse(
            REDACTION_EVIDENCE_INVALID,
            "",
            format!("{REPORT_PACKAGE_SAVE_ROUTE_ID} local-private, lossless controlled route evidence with explicit intent is missing or blocked."),
        ));
    }
    let local_first_ok = local_first.is_some_and(|l| {
        l.route_id == REPORT_PACKAGE_SAVE_ROUTE_ID
            && l.export_context == "local_private"
            && l.storage_context == "local_private"
            && l.action == "include_metadata_only"
            && l.reason_code == "PRIVATE_LOCAL_METADATA_ALLOWED"
            && !l.blocked
            && l.metadata_only
            && l.explicit_local_private_intent
    });
    if !local_first_ok {
        return Err(refuse(LOCAL_FIRST_EVIDENCE_INVALID, "", "Metadata-only local-first route evidence is missing or blocked."));
    }
    Ok(())
}

/// The size caps of §2.6 item 3, before anything is parsed or priced.
pub fn check_text_sizes(request_text: &str, model_text: &str) -> Result<(), ReportPackageSaveCommandError> {
    for (what, text, cap) in [("request", request_text, MAX_REQUEST_TEXT_BYTES), ("model", model_text, MAX_MODEL_TEXT_BYTES)] {
        let bytes = text.len() as u64;
        if bytes > cap {
            return Err(refuse(TOO_LARGE, &format!("/{what}"), format!("The {what} text is {bytes} bytes; at most {cap} are admitted.")));
        }
    }
    Ok(())
}

/// The ingress lease growth for parsing both texts (§2.6 item 1, "a priced
/// parse of the text"): the request is parsed and then typed-decoded
/// (`(2 + REPORT_PACKAGE_VEC_EXPANSION)` M), the model only parsed (M), each
/// with its canonical re-serialization ([`crate::command_ingress::text_parse_bytes`]).
/// Applies the size caps first. A text that is not one JSON value is refused
/// here, before it is parsed.
pub fn parse_lease_bytes(request_text: &str, model_text: &str) -> Result<u64, ReportPackageSaveCommandError> {
    check_text_sizes(request_text, model_text)?;
    let priced = |what: &str, text: &str, decoding: ArgsDecoding| {
        crate::command_ingress::text_parse_bytes(text, decoding).map_err(|invalid| {
            refuse(NOT_CANONICAL, &format!("/{what}"), format!("The {what} text is not JSON ({} at byte {}).", invalid.reason, invalid.offset))
        })
    };
    let request = priced("request", request_text, ArgsDecoding::Rebuilt { vec_expansion: REPORT_PACKAGE_VEC_EXPANSION })?;
    let model = priced("model", model_text, ArgsDecoding::Moved)?;
    Ok(request.saturating_add(model))
}

/// Parses `text` and refuses unless it is its own RFC 8785 canonical form.
pub(crate) fn parse_canonical(what: &str, text: &str) -> Result<Value, ReportPackageSaveCommandError> {
    let value: Value = serde_json::from_str(text)
        .map_err(|error| refuse(NOT_CANONICAL, &format!("/{what}"), format!("The {what} text is not JSON ({error}).")))?;
    if canonical_json(&value) != text {
        return Err(refuse(NOT_CANONICAL, &format!("/{what}"), format!("The {what} text is not its own RFC 8785 (JCS) canonical form.")));
    }
    Ok(value)
}

/// The typed request, decoded from the moved `Value` by [`ValueDecoder`] (no
/// size hints, strings moved, bounded error text): the conversion
/// [`parse_lease_bytes`] prices.
pub(crate) fn typed_request(request: Value) -> Result<ReportPackageRequest, ReportPackageSaveCommandError> {
    ReportPackageRequest::deserialize(ValueDecoder(request))
        .map_err(|error| refuse(REQUEST_INVALID, "/request", format!("The request is not a version 2.0.0 report-package request: {error}.")))
}

/// A request admitted for the save dialog: the assembled package and the
/// screen's warnings.
#[derive(Debug)]
pub struct AdmittedReportPackage {
    pub package: ReportPackageContainerOutcome,
    pub screening: ReportPackageSaveScreening,
    pub local_first: ReportPackageLocalFirstEvidence,
    pub warnings: Vec<Diagnostic>,
}

fn text_at<'a>(value: &'a Value, pointer: &str) -> Option<&'a str> {
    value.pointer(pointer).and_then(Value::as_str)
}

/// The disposition screens of §2.6 item 4 over both parsed values, with the
/// model project's store: the blocking findings of either refuse, a store
/// read error refuses, unresolved references are warnings.
fn screen_content(request: &Value, model: &Value, project_id: &str, lookup: &mut LibraryLookup<'_>) -> Result<Vec<Diagnostic>, ReportPackageSaveCommandError> {
    let mut blocking = Vec::new();
    let mut warnings = Vec::new();
    for (root, value) in [("request", request), ("model", model)] {
        match screen_dispositions_in(value, root, project_id, LIBRARY_STORE_UNAVAILABLE, lookup) {
            Ok(found) => warnings.extend(found),
            Err(found) if found.iter().any(|item| item.code == LIBRARY_STORE_UNAVAILABLE) => {
                return Err(ReportPackageSaveCommandError::refused(found));
            }
            Err(found) => blocking.extend(found),
        }
    }
    if blocking.is_empty() { Ok(warnings) } else { Err(ReportPackageSaveCommandError::refused(blocking)) }
}

/// The raw context and model-text bindings of §2.6 item 5.
fn check_context(request: &Value, model: &Value, software: &ExpectedSoftware) -> Result<(), ReportPackageSaveCommandError> {
    if text_at(request, "/report/report_context/export_context") != Some("local_private") {
        return Err(refuse(
            REPORT_CONTEXT_NOT_LOCAL_PRIVATE,
            "/request/report/report_context/export_context",
            "The report package renders under local_private only.",
        ));
    }
    if text_at(request, "/export_profile_id") != Some(DESKTOP_PROFILE) {
        return Err(refuse(PROFILE_NOT_DESKTOP, "/request/export_profile_id", format!("The desktop saves only the {DESKTOP_PROFILE} profile.")));
    }
    let project_id = text_at(model, "/project/id");
    for pointer in ["/source_model_ref/ref_id", "/source_results/0/model_ref"] {
        if project_id.is_none() || text_at(request, pointer) != project_id {
            return Err(refuse(MODEL_MISMATCH, &format!("/request{pointer}"), "The request names another model than the model text's project.id."));
        }
    }
    let name = text_at(model, "/project/name").ok_or_else(|| refuse(MODEL_MISMATCH, "/model/project/name", "The model text has no project name."))?;
    if text_at(request, "/report/report_title") != Some(format!("{name} — Calculation Report").as_str()) {
        return Err(refuse(MODEL_MISMATCH, "/request/report/report_title", "The report title is not the model project's name with \" — Calculation Report\"."));
    }
    let units = model.pointer("/project/units").filter(|units| units.is_object());
    let reported = request.pointer("/report/calculation_report/model_input_summary/unit_display_summary/model_units");
    if units.is_none() || reported != units {
        return Err(refuse(
            MODEL_MISMATCH,
            "/request/report/calculation_report/model_input_summary/unit_display_summary/model_units",
            "The report's model units are not the model text's project.units.",
        ));
    }
    for (member, expected) in [("software_name", &software.name), ("software_version", &software.version)] {
        if text_at(request, &format!("/report/report_context/{member}")) != Some(expected.as_str()) {
            return Err(refuse(
                SOFTWARE_MISMATCH,
                &format!("/request/report/report_context/{member}"),
                format!("The report context's {member} is not this application's ({expected})."),
            ));
        }
    }
    Ok(())
}

/// The admission of §2.6 items 2-6, before any file is chosen.
pub fn admit_report_package(
    request: ReportPackageSaveRequest,
    software: &ExpectedSoftware,
    lookup: &mut LibraryLookup<'_>,
) -> Result<AdmittedReportPackage, ReportPackageSaveCommandError> {
    let ReportPackageSaveRequest { request_text, model_text, screening, local_first } = request;
    validate_report_package_save_evidence(&screening, local_first.as_ref())?;
    let local_first = local_first.expect("validated local-first evidence");
    check_text_sizes(&request_text, &model_text)?;

    // Canonical binding: the evidence names exactly this request text.
    let request_hex = sha256_hex(&request_text);
    if screening.payload_sha256 != format!("sha256:{request_hex}") {
        return Err(refuse(
            EVIDENCE_UNBOUND,
            "/request",
            format!("The screening evidence is for {}, not for this request text (sha256:{request_hex}).", screening.payload_sha256),
        ));
    }
    let request_value = parse_canonical("request", &request_text)?;
    drop(request_text);
    let model_value = parse_canonical("model", &model_text)?;
    let model_hex = sha256_hex(&model_text);
    drop(model_text);
    if text_at(&request_value, "/audit_manifest/model_hash/value") != Some(model_hex.as_str()) {
        return Err(refuse(
            MODEL_UNBOUND,
            "/request/audit_manifest/model_hash/value",
            format!("The request's audit model hash is not the model text's SHA-256 ({model_hex})."),
        ));
    }

    // Disposition screening of both values, then the context bindings.
    let project_id = text_at(&model_value, "/project/id").unwrap_or("").to_string();
    let warnings = screen_content(&request_value, &model_value, &project_id, lookup)?;
    check_context(&request_value, &model_value, software)?;
    drop(model_value);

    // Typed conversion and the shared producer (§2.3).
    let package = report_package_bridge::assemble_request(typed_request(request_value)?).map_err(ReportPackageSaveCommandError::bridge)?;
    if package.export_blocked {
        return Err(ReportPackageSaveCommandError::bridge(format!("{PRODUCER_BLOCKED}: {}", package.blocking_reasons.join(" | "))));
    }
    Ok(AdmittedReportPackage { package, screening, local_first, warnings })
}

impl AdmittedReportPackage {
    fn receipt(self, outcome: &str, stage: &str, code: String, written: Option<atomic_report_package_save::AtomicSaveOutcome>) -> ReportPackageSaveReceipt {
        let Self { package, screening, local_first, warnings } = self;
        let byte_count = if written.is_some() { package.container_bytes.len() } else { 0 };
        let (replaced_existing, durability, selected_basename, path_containment, limitation) = match written {
            Some(save) => (save.replaced_existing, save.durability, save.selected_basename, save.path_containment, save.limitation),
            None => (
                false,
                "not_applicable".to_string(),
                String::new(),
                atomic_report_package_save::PATH_CONTAINMENT.to_string(),
                atomic_report_package_save::TOCTOU_LIMITATION.to_string(),
            ),
        };
        ReportPackageSaveReceipt {
            outcome: outcome.to_string(),
            stage: stage.to_string(),
            code,
            container_file_name: package.container_file_name,
            byte_count,
            package_identity_sha256_hex: package.package_identity_sha256_hex,
            container_sha256_hex: package.container_sha256_hex,
            members: package
                .members
                .into_iter()
                .map(|member| ReportPackageMemberSummary {
                    role: member.role,
                    file_name: member.file_name,
                    sha256_hex: member.sha256_hex,
                    byte_length: member.byte_length,
                })
                .collect(),
            replaced_existing,
            durability,
            redaction_route_id: screening.route_id,
            redaction_decision_count: screening.decision_count,
            redaction_finding_count: screening.finding_count,
            redaction_blocking_count: screening.blocking_count,
            local_first_route_id: local_first.route_id,
            local_first_export_context: local_first.export_context,
            local_first_storage_context: local_first.storage_context,
            local_first_action: local_first.action,
            local_first_reason_code: local_first.reason_code,
            local_first_blocked: local_first.blocked,
            local_first_metadata_only: local_first.metadata_only,
            local_first_explicit_local_private_intent: local_first.explicit_local_private_intent,
            selected_basename,
            path_containment,
            limitation,
            warnings,
        }
    }
}

/// The save dialog and the atomic write of an admitted package; `choose` is the
/// dialog (a counting seam in tests): `None` is a cancel, `Some(Err)` an
/// unusable path.
pub fn save_admitted_report_package(
    admitted: AdmittedReportPackage,
    choose: impl FnOnce(&str) -> Option<Result<PathBuf, String>>,
) -> Result<ReportPackageSaveReceipt, ReportPackageSaveCommandError> {
    let Some(selected) = choose(&admitted.package.container_file_name) else {
        return Ok(admitted.receipt("cancelled", "picker", "REPORT-PACKAGE-SAVE-CANCELLED".to_string(), None));
    };
    let destination = selected.map_err(|error| ReportPackageSaveCommandError::bridge(format!("{PICKER_PATH_INVALID}: {error}")))?;
    let save = atomic_report_package_save::atomic_save_bytes(&destination, &admitted.package.container_bytes)?;
    let code = if save.durability == "durable" { "REPORT-PACKAGE-SAVED" } else { "REPORT-PACKAGE-SAVED-DURABILITY-UNCERTAIN" };
    Ok(admitted.receipt("saved", "complete", code.to_string(), Some(save)))
}

/// The shared invented report-package basis builder (DEL-08-01 Design §2.7),
/// by path inclusion, as the headless witness generator includes it.
#[cfg(test)]
#[path = "../../../../core/reporting/report_package/tests/common/package_basis.rs"]
mod package_basis;

/// Invented desktop-profile requests and their bound arguments, for this
/// module's tests and the command-path tests in `lib.rs`.
#[cfg(test)]
pub(crate) mod test_support {
    use super::package_basis::{build, Inputs, Profile};
    use super::*;
    use serde_json::json;

    const WITNESS: &str = include_str!("../../../../validation/witness/inputs/del1005_export_results_success_input.json");
    const MODEL: &str = include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json");
    const INPUT_MANIFEST: &str = include_str!("../../../../fixtures/reports/invented/del1005_input_manifest.json");

    pub(crate) fn software() -> ExpectedSoftware {
        ExpectedSoftware { name: "SWBPIPE".to_string(), version: "0.1.0".to_string() }
    }

    /// The model the committed witness was solved from.
    pub(crate) fn model() -> Value {
        serde_json::from_str(MODEL).unwrap()
    }

    /// The desktop-profile request derived by the shared builder from the
    /// committed witness and `model` (whose canonical SHA-256 becomes every
    /// model hash), with the desktop's native report context.
    pub(crate) fn desktop_request(model: &Value) -> Value {
        let witness: Value = serde_json::from_str(WITNESS).unwrap();
        let runner = &witness["export_results"];
        let manifest: Value = serde_json::from_str(INPUT_MANIFEST).unwrap();
        let solver = &runner["audit_manifest"]["solver_version"];
        let inputs = Inputs {
            model,
            source: &runner["source_results"][0],
            producer_document: &runner["result_documents"][0],
            solve_request: &Value::Null,
            input_manifest: &manifest,
            input_manifest_id: runner["audit_manifest"]["input_manifest_hash"]["payload_ref"].as_str().unwrap(),
            solver_name: solver["solver_name"].as_str().unwrap(),
            solver_version: solver["solver_version"].as_str().unwrap(),
            profile: Profile::Desktop,
            run_override: None,
        };
        let mut request = build(&inputs).unwrap().export_results;
        let name = model["project"]["name"].as_str().unwrap();
        request["report"]["report_title"] = json!(format!("{name} — Calculation Report"));
        request["report"]["report_context"]["software_name"] = json!(software().name);
        request["report"]["report_context"]["software_version"] = json!(software().version);
        request
    }

    pub(crate) fn hash_of(text: &str) -> String {
        format!("sha256:{}", sha256_hex(text))
    }

    pub(crate) fn screening(payload_sha256: String) -> ReportPackageSaveScreening {
        ReportPackageSaveScreening {
            route_id: REPORT_PACKAGE_SAVE_ROUTE_ID.to_string(),
            export_context: "local_private".to_string(),
            explicit_local_private_intent: true,
            blocked: false,
            materialization_withheld: false,
            lossless_required: true,
            exact_payload_match: true,
            blocking_count: 0,
            decision_count: 3,
            finding_count: 1,
            payload_sha256,
        }
    }

    pub(crate) fn local_first() -> ReportPackageLocalFirstEvidence {
        ReportPackageLocalFirstEvidence {
            route_id: REPORT_PACKAGE_SAVE_ROUTE_ID.to_string(),
            export_context: "local_private".to_string(),
            storage_context: "local_private".to_string(),
            action: "include_metadata_only".to_string(),
            reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED".to_string(),
            blocked: false,
            metadata_only: true,
            explicit_local_private_intent: true,
        }
    }

    /// The command's arguments for `request` and `model`, with clean evidence
    /// bound to the request's canonical text.
    pub(crate) fn save_request(request: &Value, model: &Value) -> ReportPackageSaveRequest {
        let request_text = canonical_json(request);
        let payload = hash_of(&request_text);
        ReportPackageSaveRequest { request_text, model_text: canonical_json(model), screening: screening(payload), local_first: Some(local_first()) }
    }

    /// The same arguments as the command's raw args object.
    pub(crate) fn args_body(request: &ReportPackageSaveRequest) -> Value {
        json!({
            "requestCanonicalText": request.request_text,
            "modelCanonicalText": request.model_text,
            "screening": serde_json::to_value(&request.screening).unwrap(),
            "localFirstEvidence": serde_json::to_value(&request.local_first).unwrap(),
        })
    }

    pub(crate) fn no_store(_: &str, _: &str, _: &str) -> Result<Option<Value>, String> {
        Ok(None)
    }

    /// A fresh, empty sandbox directory under the system temporary directory.
    pub(crate) fn sandbox(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "swbpipe-report-package-save-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir_all(&path).unwrap();
        path
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;
    use open_pipe_stress_report_package::{read_report_package_container, ROLE_RESULT_EXPORT};
    use serde_json::json;
    use std::fs;
    use std::sync::OnceLock;

    fn desktop() -> Value {
        static DESKTOP: OnceLock<Value> = OnceLock::new();
        DESKTOP.get_or_init(|| desktop_request(&model())).clone()
    }

    fn admit(request: ReportPackageSaveRequest) -> Result<AdmittedReportPackage, ReportPackageSaveCommandError> {
        admit_report_package(request, &software(), &mut no_store)
    }

    /// The refusal's code and the paths of its findings.
    #[track_caller]
    fn refusal(outcome: Result<AdmittedReportPackage, ReportPackageSaveCommandError>) -> (String, Vec<String>) {
        match outcome {
            Ok(_) => panic!("admitted"),
            Err(error) => (error.code.clone(), error.diagnostics.iter().map(|item| item.path.clone()).collect()),
        }
    }

    fn edited(edit: impl FnOnce(&mut Value)) -> ReportPackageSaveRequest {
        let mut request = desktop();
        edit(&mut request);
        save_request(&request, &model())
    }

    /// Admitted bytes written to a sandbox read back through the package
    /// reader, and the result member is the request's result document in JCS.
    #[test]
    fn an_admitted_package_is_saved_and_reads_back() {
        let request = desktop();
        let admitted = admit(save_request(&request, &model())).unwrap();
        assert!(admitted.warnings.is_empty(), "{:?}", admitted.warnings);
        let root = sandbox("read-back");
        let destination = root.join("invented.opsproj");
        let mut asked = Vec::new();
        let receipt = save_admitted_report_package(admitted, |name| {
            asked.push(name.to_string());
            Some(Ok(destination.clone()))
        })
        .unwrap();
        assert_eq!((receipt.outcome.as_str(), receipt.code.as_str()), ("saved", "REPORT-PACKAGE-SAVED"));
        assert_eq!(asked, vec![receipt.container_file_name.clone()]);
        assert_eq!((receipt.redaction_decision_count, receipt.redaction_finding_count, receipt.redaction_blocking_count), (3, 1, 0));
        let bytes = fs::read(&destination).unwrap();
        assert_eq!(bytes.len(), receipt.byte_count);
        assert_eq!(sha256_hex_bytes(&bytes), receipt.container_sha256_hex);
        let read = read_report_package_container(&bytes).unwrap();
        assert!(!read.export_blocked, "{:?}", read.blocking_reasons);
        assert_eq!(read.export_profile_id, DESKTOP_PROFILE);
        let member = read.members.iter().find(|member| member.role == ROLE_RESULT_EXPORT).unwrap();
        assert_eq!(member.bytes, canonical_json(&request["result_documents"][0]).into_bytes());
        fs::remove_dir_all(root).unwrap();
    }

    fn sha256_hex_bytes(bytes: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        format!("{:x}", Sha256::digest(bytes))
    }

    #[test]
    fn a_cancelled_dialog_writes_nothing() {
        let admitted = admit(save_request(&desktop(), &model())).unwrap();
        let receipt = save_admitted_report_package(admitted, |_| None).unwrap();
        assert_eq!((receipt.outcome.as_str(), receipt.code.as_str(), receipt.byte_count), ("cancelled", "REPORT-PACKAGE-SAVE-CANCELLED", 0));
    }

    #[test]
    fn route_evidence_is_rechecked() {
        let bound = save_request(&desktop(), &model());
        assert!(validate_report_package_save_evidence(&bound.screening, bound.local_first.as_ref()).is_ok());
        for edit in [
            (|s: &mut ReportPackageSaveScreening| s.route_id = "DREP-JSON-002".to_string()) as fn(&mut ReportPackageSaveScreening),
            |s| s.export_context = "public_report".to_string(),
            |s| s.explicit_local_private_intent = false,
            |s| s.blocked = true,
            |s| s.materialization_withheld = true,
            |s| s.lossless_required = false,
            |s| s.exact_payload_match = false,
            |s| s.blocking_count = 1,
        ] {
            let mut request = save_request(&desktop(), &model());
            edit(&mut request.screening);
            assert_eq!(refusal(admit(request)).0, REDACTION_EVIDENCE_INVALID);
        }
        for edit in [
            (|l: &mut ReportPackageLocalFirstEvidence| l.route_id = "DOTH-MODEL-DOC-007".to_string()) as fn(&mut ReportPackageLocalFirstEvidence),
            |l| l.export_context = "public_report".to_string(),
            |l| l.storage_context = "public_report".to_string(),
            |l| l.action = "block_storage".to_string(),
            |l| l.reason_code = "LOCAL_PRIVATE_INTENT_REQUIRED".to_string(),
            |l| l.blocked = true,
            |l| l.metadata_only = false,
            |l| l.explicit_local_private_intent = false,
        ] {
            let mut request = save_request(&desktop(), &model());
            edit(request.local_first.as_mut().unwrap());
            assert_eq!(refusal(admit(request)).0, LOCAL_FIRST_EVIDENCE_INVALID);
        }
        let mut request = save_request(&desktop(), &model());
        request.local_first = None;
        assert_eq!(refusal(admit(request)).0, LOCAL_FIRST_EVIDENCE_INVALID);
    }

    #[test]
    fn evidence_for_other_bytes_is_refused() {
        let mut request = save_request(&desktop(), &model());
        request.screening.payload_sha256 = hash_of("{}");
        assert_eq!(refusal(admit(request)), (EVIDENCE_UNBOUND.to_string(), vec!["/request".to_string()]));
        // Evidence for the model text, not the request text.
        let mut request = save_request(&desktop(), &model());
        request.screening.payload_sha256 = hash_of(&request.model_text);
        assert_eq!(refusal(admit(request)).0, EVIDENCE_UNBOUND);
    }

    #[test]
    fn non_canonical_texts_are_refused() {
        let pretty = serde_json::to_string_pretty(&desktop()).unwrap();
        let mut request = save_request(&desktop(), &model());
        request.screening.payload_sha256 = hash_of(&pretty);
        request.request_text = pretty;
        assert_eq!(refusal(admit(request)), (NOT_CANONICAL.to_string(), vec!["/request".to_string()]));
        let mut request = save_request(&desktop(), &model());
        request.request_text.push('\n');
        request.screening.payload_sha256 = hash_of(&request.request_text);
        assert_eq!(refusal(admit(request)).0, NOT_CANONICAL);
        let mut request = save_request(&desktop(), &model());
        request.request_text = "{not json".to_string();
        request.screening.payload_sha256 = hash_of(&request.request_text);
        assert_eq!(refusal(admit(request)).0, NOT_CANONICAL);
        // The model as authored (pretty), although its canonical form is bound.
        let mut request = save_request(&desktop(), &model());
        request.model_text = serde_json::to_string_pretty(&model()).unwrap();
        assert_eq!(refusal(admit(request)), (NOT_CANONICAL.to_string(), vec!["/model".to_string()]));
    }

    #[test]
    fn a_model_text_other_than_the_audited_model_is_refused() {
        let mut other = model();
        other["project"]["description"] = json!("Invented other model");
        let request = save_request(&desktop(), &other);
        assert_eq!(refusal(admit(request)), (MODEL_UNBOUND.to_string(), vec!["/request/audit_manifest/model_hash/value".to_string()]));
    }

    #[test]
    fn the_size_caps_apply_before_parsing() {
        let over = "x".repeat(MAX_MODEL_TEXT_BYTES as usize + 1);
        assert_eq!(check_text_sizes("{}", &over).unwrap_err().code, TOO_LARGE);
        assert_eq!(parse_lease_bytes("{}", &over).unwrap_err().code, TOO_LARGE);
        let at = "x".repeat(MAX_MODEL_TEXT_BYTES as usize);
        assert!(check_text_sizes("{}", &at).is_ok());
        // At the cap, the prescan refuses text that is not JSON before any parse.
        assert_eq!(parse_lease_bytes("{}", &at).unwrap_err().code, NOT_CANONICAL);
        drop((over, at));
        let over = "x".repeat(MAX_REQUEST_TEXT_BYTES as usize + 1);
        assert_eq!(check_text_sizes(&over, "{}").unwrap_err().diagnostics[0].path, "/request");
    }

    /// Review lesson (W-6a, W-6b): complete caller-authored clean evidence
    /// does not admit blocked content anywhere in the request.
    #[test]
    fn clean_evidence_cannot_admit_blocked_request_content_at_depth() {
        let quarantined = json!({"source_name": "invented", "review_status": "quarantined"});
        let cases: Vec<(&str, Box<dyn Fn(&mut Value)>)> = vec![
            ("report", Box::new(|r: &mut Value| r["report"]["report_sections"]["provenance_notes"][0]["nested"] = json!({"provenance_record": quarantined.clone()}))),
            ("document", Box::new(|r: &mut Value| r["result_documents"][0]["result_envelope"]["source_annotations"] = json!([{"provenance_record": quarantined.clone()}]))),
            ("carrier", Box::new(|r: &mut Value| r["source_results"][0]["results"][0]["metadata"] = json!({"component": "c", "provenance_record": quarantined.clone()}))),
            ("handoff", Box::new(|r: &mut Value| r["state_comparison_handoff_records"][0]["sections"]["handoff_sections"] = json!([{"provenance_record": quarantined.clone()}]))),
            ("rule pack", Box::new(|r: &mut Value| r["report"]["calculation_report"]["rule_pack_refs"] = json!([{"rule_pack_id": "rule-pack:invented", "redistribution_status": "protected_suspected"}]))),
        ];
        for (label, edit) in cases {
            let request = edited(|r| edit(r));
            assert!(validate_report_package_save_evidence(&request.screening, request.local_first.as_ref()).is_ok());
            let (code, paths) = refusal(admit(request));
            assert_eq!(code, PROTECTED_CONTENT_BLOCKED, "{label}");
            assert!(paths.iter().all(|path| path.starts_with("request.")), "{label}: {paths:?}");
        }
    }

    /// The model's own dispositions are screened, although the report input
    /// relabels its provenance (§2.6 item 4).
    #[test]
    fn model_only_content_is_screened() {
        let mut blocked = model();
        blocked["materials"][0]["provenance_record"] = json!({"source_name": "invented", "review_status": "quarantined"});
        let request = desktop_request(&blocked);
        let (code, paths) = refusal(admit(save_request(&request, &blocked)));
        assert_eq!((code.as_str(), paths), (PROTECTED_CONTENT_BLOCKED, vec!["model.materials[0].provenance_record".to_string()]));
    }

    fn stored(review_status: &str) -> Value {
        json!({"schema_version": "0.1.0", "material_library": {"provenance": {"review_status": "accepted"}},
            "material_records": [{"material_id": "invented-steel", "review_status": review_status}]})
    }

    /// A `library_ref` that occurs only in the request is resolved in the
    /// model project's store (a package request has no `/project/id`).
    #[test]
    fn a_request_only_library_reference_resolves_with_the_models_project() {
        let request = edited(|r| {
            r["report"]["report_sections"]["provenance_notes"][0]["library_ref"] =
                json!({"library_kind": "material", "library_id": " library:invented ", "record_id": "invented-steel"});
        });
        let project = model()["project"]["id"].as_str().unwrap().to_string();
        let run = |request: ReportPackageSaveRequest, answer: Result<Option<Value>, String>| {
            let mut calls = Vec::new();
            let mut lookup = |project: &str, kind: &str, id: &str| {
                calls.push((project.to_string(), kind.to_string(), id.to_string()));
                answer.clone()
            };
            let outcome = admit_report_package(request, &software(), &mut lookup);
            (outcome, calls)
        };
        let again = || save_request(&serde_json::from_str(&request.request_text).unwrap(), &model());
        let (outcome, calls) = run(again(), Ok(Some(stored("quarantined"))));
        assert_eq!(calls, vec![(project.clone(), "material".to_string(), "library:invented".to_string())]);
        assert_eq!(refusal(outcome).0, PROTECTED_CONTENT_BLOCKED);
        let (outcome, _) = run(again(), Err("invented store failure".to_string()));
        assert_eq!(refusal(outcome).0, LIBRARY_STORE_UNAVAILABLE);
        let (outcome, _) = run(again(), Ok(Some(stored("accepted"))));
        assert!(outcome.unwrap().warnings.is_empty());
        let (outcome, _) = run(again(), Ok(None));
        let warnings = outcome.unwrap().warnings;
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, crate::MODEL_LIBRARY_REF_UNRESOLVED);
        // A model without library references and a clean request never read the store.
        let (outcome, calls) = run(save_request(&desktop(), &model()), Err("unread".to_string()));
        assert!(outcome.is_ok() && calls.is_empty());
    }

    /// The raw context and the model-text bindings (§2.6 item 5).
    #[test]
    fn context_and_model_text_bindings_are_checked() {
        let (code, paths) = refusal(admit(edited(|r| r["report"]["report_context"]["export_context"] = json!("public_report"))));
        assert_eq!((code.as_str(), paths[0].as_str()), (REPORT_CONTEXT_NOT_LOCAL_PRIVATE, "/request/report/report_context/export_context"));
        let (code, _) = refusal(admit(edited(|r| r["export_profile_id"] = json!("runner_local_private_report_package_1"))));
        assert_eq!(code, PROFILE_NOT_DESKTOP);
        // A model text naming another project, with its hash audited (the
        // shared producer cannot derive a document from it, so only the native
        // binding speaks here).
        let mut other = model();
        other["project"]["id"] = json!("project:invented-other");
        let mut request = desktop();
        request["audit_manifest"]["model_hash"]["value"] = json!(sha256_hex(&canonical_json(&other)));
        let (code, paths) = refusal(admit(save_request(&request, &other)));
        assert_eq!((code.as_str(), paths[0].as_str()), (MODEL_MISMATCH, "/request/source_model_ref/ref_id"));
        let (code, paths) = refusal(admit(edited(|r| r["report"]["report_title"] = json!("Invented — Calculation Report"))));
        assert_eq!((code.as_str(), paths[0].as_str()), (MODEL_MISMATCH, "/request/report/report_title"));
        let (code, paths) =
            refusal(admit(edited(|r| r["report"]["calculation_report"]["model_input_summary"]["unit_display_summary"]["model_units"]["force"] = json!("kN"))));
        assert_eq!(code, MODEL_MISMATCH);
        assert!(paths[0].ends_with("/model_units"));
        for member in ["software_name", "software_version"] {
            let (code, paths) = refusal(admit(edited(|r| r["report"]["report_context"][member] = json!("Invented forged"))));
            assert_eq!((code.as_str(), paths[0].as_str()), (SOFTWARE_MISMATCH, format!("/request/report/report_context/{member}").as_str()));
        }
        // Units compare by keys and values in any property order.
        let mut reordered = desktop();
        let units = reordered["report"]["calculation_report"]["model_input_summary"]["unit_display_summary"]["model_units"].take();
        let mut entries: Vec<(String, Value)> = units.as_object().unwrap().clone().into_iter().collect();
        entries.reverse();
        reordered["report"]["calculation_report"]["model_input_summary"]["unit_display_summary"]["model_units"] =
            Value::Object(entries.into_iter().collect());
        assert!(admit(save_request(&reordered, &model())).is_ok());
    }

    /// After the native checks the shared producer still runs, and a typed
    /// decode failure is named.
    #[test]
    fn the_shared_producer_and_typed_conversion_follow_admission() {
        let error = admit(edited(|r| r["report"]["result_rows"][0]["quantity_display"] = json!("1 invented"))).unwrap_err();
        assert_eq!(error.code, "REPORT-PACKAGE-REPORT-ROWS-MISMATCH");
        let error = admit(edited(|r| r["source_basis_refs"] = json!("not a list"))).unwrap_err();
        assert_eq!(error.code, REQUEST_INVALID);
    }
}

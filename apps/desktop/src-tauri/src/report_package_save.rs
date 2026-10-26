//! File > Save Report Package… native admission (DEL-08-01 Design
//! `report-package-current-results.md` §2.6, W-18a slice (b1); FR-REP-001,
//! DEL-12-02 REXC-REQ-006).
//!
//! The webview sends the package request, the session's model and the run's
//! input manifest as their RFC 8785 (JCS) canonical texts, with the route
//! evidence of `DREP-PACKAGE-SAVE-009`. Evidence is never trusted for content.
//! Before the save dialog is shown, [`admit_report_package`]
//! 1. re-checks the screening and local-first evidence (local-private,
//!    explicit intent, lossless, not withheld, exact payload, zero blocking):
//!    defence in depth, since intent is the user's webview choice;
//! 2. applies the size caps, then binds the three texts: each must be its own
//!    canonical form, the screening evidence must name the request text's
//!    SHA-256 (`REPORT-PACKAGE-SAVE-EVIDENCE-UNBOUND`), the model text's
//!    SHA-256 must be the request's audit-manifest model hash
//!    (`REPORT-PACKAGE-SAVE-MODEL-UNBOUND`), and the input-manifest text's
//!    SHA-256 must be the request's audit input-manifest hash
//!    (`REPORT-PACKAGE-SAVE-INPUT-MANIFEST-UNBOUND`), with a `model_basis`
//!    naming the model text's `project.id` and embedding the model text's
//!    canonical form (the same code) and a complete `active_rule_packs`
//!    inventory (`REPORT-PACKAGE-SAVE-INPUT-MANIFEST-INVALID`);
//! 3. screens the dispositions of both parsed values itself (the request: the
//!    report input, result document, source carrier, audit manifest and
//!    handoff records; and the model) with both shared screens
//!    (`export_disposition_screen`): W-6b's structural screen, which judges
//!    every object at every depth by its own disposition members and refuses
//!    an array or object disposition value as malformed, and W-6a's
//!    store-resolved screen, with every `library_ref` resolved in the model
//!    project's local library store; a store read error refuses;
//! 4. binds the raw report context and the model text: `local_private`, the
//!    desktop export profile, the model's project id, name and units, and the
//!    application's own name and version; then the audit input-manifest
//!    hash's payload reference, which must be the session's reference for the
//!    input-manifest text (`input-manifest:<model>:<sha256>`,
//!    `REPORT-PACKAGE-SAVE-INPUT-MANIFEST-UNBOUND`; the shared producer binds
//!    it to the result document, the report and the handoff records, and, for
//!    the desktop profile, to the audit `manifest_id`); then it
//!    refuses a package whose rule packs (the report's and the audit
//!    manifest's references) are not the input manifest's
//!    `active_rule_packs`, compared as sets of `{rule_pack_id, version,
//!    checksum value}` (`REPORT-PACKAGE-RULE-PACKS-DISAGREE`, §2.5);
//! 5. converts the request to its typed form and runs the shared producer
//!    (`report_package::wire`, §2.3), refusing a blocked package.
//!
//! W-11 slice (b) (DEL-08-01 Design `exact-route-report-sections.md` §3.6): a
//! request whose result document has an exact identity, or whose report
//! carries `exact_route`, is bound to its source natively right after the
//! request text is parsed and before the other content checks: the named
//! job's custody, the resubmitted result and intake texts against the digests
//! the host recorded for that job, the native reader's eligible standing, the
//! admission's digests, then the request's source and the model text against
//! the delivered result and the solved model (`exact_source_binding.rs`).
//! Custody is checked again after the save dialog and before the write.
//! Before opening, such a request then refuses at the shared producer's
//! item 0 (`REPORT-PACKAGE-ROUTE-NOT-ADMITTED`).
//!
//! Admission is a pure function of its arguments, the library lookup and the
//! custody lookup, so its tests need no window. The command (`lib.rs` `save_report_package`)
//! prices the parse of the three texts in its ingress lease before admission.
//! The input manifest is a validation input only: it is not screened, because
//! no package member carries it, and it is never saved.
//! Nothing is sent anywhere (F-PIP-1). Standard claim fence applies (claims
//! taxonomy per DEC-081).

use crate::atomic_report_package_save::{self, AtomicSaveError};
use crate::command_ingress::{report_package_request_parse_bytes, text_parse_bytes, Args, ArgsDecoding, ValueDecoder};
use crate::model_document_file::MAX_MODEL_DOCUMENT_BYTES;
use crate::export_disposition_screen::{
    malformed_values, screen_dispositions_in, screen_value, DispositionDiagnostic as Diagnostic, LibraryLookup, PathStyle,
};
use crate::exact_source_binding::{self as exact_binding, ExactRefusal};
use crate::report_package_bridge::{self, ReportPackageRequest};
use crate::solve_jobs::{CustodyDigests, SolveJobError};
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use open_pipe_stress_report_package::bindings::{DESKTOP_PROFILE, REPORT_CONTEXT_NOT_LOCAL_PRIVATE};
use open_pipe_stress_report_package::ReportPackageContainerOutcome;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::PathBuf;

/// The governed local-first route of this save.
pub const REPORT_PACKAGE_SAVE_ROUTE_ID: &str = "DREP-PACKAGE-SAVE-009";

pub const REDACTION_EVIDENCE_INVALID: &str = "REPORT-PACKAGE-REDACTION-EVIDENCE-INVALID";
pub const LOCAL_FIRST_EVIDENCE_INVALID: &str = "REPORT-PACKAGE-LOCAL-FIRST-EVIDENCE-INVALID";
pub const TOO_LARGE: &str = "REPORT-PACKAGE-SAVE-TOO-LARGE";
pub const NOT_CANONICAL: &str = "REPORT-PACKAGE-SAVE-NOT-CANONICAL";
pub const EVIDENCE_UNBOUND: &str = "REPORT-PACKAGE-SAVE-EVIDENCE-UNBOUND";
pub const MODEL_UNBOUND: &str = "REPORT-PACKAGE-SAVE-MODEL-UNBOUND";
pub const INPUT_MANIFEST_UNBOUND: &str = "REPORT-PACKAGE-SAVE-INPUT-MANIFEST-UNBOUND";
pub const INPUT_MANIFEST_INVALID: &str = "REPORT-PACKAGE-SAVE-INPUT-MANIFEST-INVALID";
/// The package's rule packs are not the packs the solve recorded (§2.5; the
/// webview's early refusal has the same code, `reportPackageRequest.ts`).
pub const RULE_PACKS_DISAGREE: &str = "REPORT-PACKAGE-RULE-PACKS-DISAGREE";
pub const LIBRARY_STORE_UNAVAILABLE: &str = "REPORT-PACKAGE-SAVE-LIBRARY-STORE-UNAVAILABLE";
pub const PROFILE_NOT_DESKTOP: &str = "REPORT-PACKAGE-SAVE-PROFILE-NOT-DESKTOP";
pub const MODEL_MISMATCH: &str = "REPORT-PACKAGE-SAVE-MODEL-MISMATCH";
pub const SOFTWARE_MISMATCH: &str = "REPORT-PACKAGE-SAVE-SOFTWARE-MISMATCH";
pub const REQUEST_INVALID: &str = "REPORT-PACKAGE-SAVE-REQUEST-INVALID";
pub const PRODUCER_BLOCKED: &str = "REPORT-PACKAGE-PRODUCER-BLOCKED";
pub const PICKER_PATH_INVALID: &str = "REPORT-PACKAGE-PICKER-PATH-INVALID";
pub const WORKER_FAILED: &str = "REPORT-PACKAGE-SAVE-WORKER-FAILED";
pub const DISPOSITION_MALFORMED: &str = "REPORT-PACKAGE-SAVE-DISPOSITION-MALFORMED";
/// A request with a struct, at any depth, in positional (JSON array) form
/// (found in Codex review rounds 2 and 3 of #1648).
pub const REQUEST_POSITIONAL: &str = "REPORT-PACKAGE-SAVE-REQUEST-POSITIONAL";
/// The webview screener's code for a blocked disposition (`redactionExportControls.ts`).
pub use crate::export_disposition_screen::PROTECTED_CONTENT_BLOCKED;

/// The largest request text admitted. The request carries the report input,
/// the result document, its source carrier and the handoff records, each a
/// whole-run document, so its cap is four model-document caps.
pub const MAX_REQUEST_TEXT_BYTES: u64 = 4 * MAX_MODEL_DOCUMENT_BYTES;
/// The model text's cap: the model-document open cap (W-6a).
pub const MAX_MODEL_TEXT_BYTES: u64 = MAX_MODEL_DOCUMENT_BYTES;
/// The input-manifest text's cap. The session's input manifest embeds the
/// model payload and repeats its load cases and combinations
/// (`inputManifestService.ts`), so its cap is two model-document caps.
pub const MAX_INPUT_MANIFEST_TEXT_BYTES: u64 = 2 * MAX_MODEL_DOCUMENT_BYTES;

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

/// `{requestCanonicalText, modelCanonicalText, inputManifestCanonicalText,
/// screening, localFirstEvidence?}`.
#[derive(Debug)]
pub struct ReportPackageSaveRequest {
    pub request_text: String,
    pub model_text: String,
    /// The run's input manifest; a validation input only, never saved.
    pub input_manifest_text: String,
    pub screening: ReportPackageSaveScreening,
    pub local_first: Option<ReportPackageLocalFirstEvidence>,
    /// W-11 (§3.6): for an exact-route request only, the bytes its solve job
    /// delivered and the model text the job was sent.
    pub exact_result_text: Option<String>,
    pub exact_intake_text: Option<String>,
}

/// The custody lookup of the native source binding: the registry's
/// `custody_binding`.
pub type CustodyLookup<'a> = dyn Fn(&str) -> Option<CustodyDigests> + 'a;

/// A custody lookup that names no job: every exact-route request refuses at
/// its custody binding.
pub fn no_custody(_: &str) -> Option<CustodyDigests> {
    None
}

/// Decodes the command's raw-body arguments (strings and structs of scalars,
/// moved: [`ArgsDecoding::Moved`]).
pub(crate) fn save_report_package_args(args: &mut Args) -> Result<ReportPackageSaveRequest, SolveJobError> {
    Ok(ReportPackageSaveRequest {
        request_text: args.decode("requestCanonicalText")?,
        model_text: args.decode("modelCanonicalText")?,
        input_manifest_text: args.decode("inputManifestCanonicalText")?,
        screening: args.decode("screening")?,
        local_first: args.decode("localFirstEvidence")?,
        exact_result_text: args.decode("exactResultText")?,
        exact_intake_text: args.decode("exactIntakeText")?,
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
    /// Every finding of a native admission refusal, with its JSON pointer
    /// under `/request` or `/model` (or "" for the evidence). Empty for
    /// producer and writer errors.
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
pub fn check_text_sizes(request_text: &str, model_text: &str, input_manifest_text: &str) -> Result<(), ReportPackageSaveCommandError> {
    for (what, text, cap) in [
        ("request", request_text, MAX_REQUEST_TEXT_BYTES),
        ("model", model_text, MAX_MODEL_TEXT_BYTES),
        ("input_manifest", input_manifest_text, MAX_INPUT_MANIFEST_TEXT_BYTES),
    ] {
        let bytes = text.len() as u64;
        if bytes > cap {
            return Err(refuse(TOO_LARGE, &format!("/{what}"), format!("The {what} text is {bytes} bytes; at most {cap} are admitted.")));
        }
    }
    Ok(())
}

/// The size caps of an exact-route request's resubmitted texts (§3.6): the
/// result text's is the request text's, since the source is a member of the
/// request; the intake text's is the model text's.
pub fn check_exact_text_sizes(exact_result_text: Option<&str>, exact_intake_text: Option<&str>) -> Result<(), ReportPackageSaveCommandError> {
    for (what, text, cap) in [("exact_result", exact_result_text, MAX_REQUEST_TEXT_BYTES), ("exact_intake", exact_intake_text, MAX_MODEL_TEXT_BYTES)] {
        let bytes = text.map_or(0, str::len) as u64;
        if bytes > cap {
            return Err(refuse(TOO_LARGE, &format!("/{what}"), format!("The {what} text is {bytes} bytes; at most {cap} are admitted.")));
        }
    }
    Ok(())
}

/// [`parse_lease_bytes`] plus the priced parse of an exact-route request's two
/// resubmitted texts, each M plus its re-serialization
/// ([`crate::command_ingress::text_parse_bytes`]; §3.6, "Pricing"). The
/// native standing, the digests and the comparisons run under the held lease
/// as the command's working memory, within the existing W M provision.
pub fn parse_lease_bytes_with_exact(
    request_text: &str,
    model_text: &str,
    input_manifest_text: &str,
    exact_result_text: Option<&str>,
    exact_intake_text: Option<&str>,
) -> Result<u64, ReportPackageSaveCommandError> {
    let base = parse_lease_bytes(request_text, model_text, input_manifest_text)?;
    check_exact_text_sizes(exact_result_text, exact_intake_text)?;
    let priced = |what: &str, text: Option<&str>| match text {
        None => Ok(0),
        Some(text) => text_parse_bytes(text, ArgsDecoding::Moved).map_err(|invalid| {
            refuse(NOT_CANONICAL, &format!("/{what}"), format!("The {what} text is not JSON ({} at byte {}).", invalid.reason, invalid.offset))
        }),
    };
    Ok(base.saturating_add(priced("exact_result", exact_result_text)?).saturating_add(priced("exact_intake", exact_intake_text)?))
}

/// The ingress lease growth for parsing the three texts (§2.6 item 1, "a
/// priced parse of the text"): the request is parsed, then typed-decoded and
/// assembled, priced by its regions with the command's working memory
/// ([`crate::command_ingress::report_package_request_parse_bytes`]; DEL-04-07
/// memory-budget decision, "Report-package request pricing"); the model and
/// the input manifest are only parsed (M), each with its canonical
/// re-serialization ([`crate::command_ingress::text_parse_bytes`]). Applies
/// the size caps first. A text that is not one JSON value is refused here,
/// before it is parsed.
pub fn parse_lease_bytes(request_text: &str, model_text: &str, input_manifest_text: &str) -> Result<u64, ReportPackageSaveCommandError> {
    check_text_sizes(request_text, model_text, input_manifest_text)?;
    let priced = |what: &str, priced: Result<u64, crate::ingress::IngressInvalid>| {
        priced.map_err(|invalid| {
            refuse(NOT_CANONICAL, &format!("/{what}"), format!("The {what} text is not JSON ({} at byte {}).", invalid.reason, invalid.offset))
        })
    };
    let request = priced("request", report_package_request_parse_bytes(request_text))?;
    let model = priced("model", text_parse_bytes(model_text, ArgsDecoding::Moved))?;
    let input_manifest = priced("input_manifest", text_parse_bytes(input_manifest_text, ArgsDecoding::Moved))?;
    Ok(request.saturating_add(model).saturating_add(input_manifest))
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
///
/// Each member is moved out and decoded by [`ValueDecoder`] on its own, as the
/// whole decode decodes it, so a refusal names the member and the error kind
/// only, never the offending value or key (W-11 slice (c), Codex review round
/// 3 of #1639). An unknown top-level member is ignored, as there.
pub(crate) fn typed_request(request: Value) -> Result<ReportPackageRequest, ReportPackageSaveCommandError> {
    let invalid = |detail: String| {
        refuse(REQUEST_INVALID, "/request", format!("The request is not a version 2.0.0 report-package request: {detail}."))
    };
    // A positional request (a JSON array) is read by field order, as serde
    // reads it.
    let members = open_pipe_stress_report_package::wire::REQUEST_MEMBERS;
    let mut fields = match request {
        Value::Object(fields) => fields,
        Value::Array(items) => open_pipe_stress_report_renderer::positional_members(items, members, members.len())
            .map_err(|kind| invalid(format!("{kind} at the request")))?,
        _ => return Err(invalid("invalid type at the request".to_string())),
    };
    fn member<T: serde::de::DeserializeOwned>(fields: &mut serde_json::Map<String, Value>, key: &str, optional: bool) -> Result<T, String> {
        let decoded = match fields.remove(key) {
            None if !optional => return Err(format!("missing field at {key}")),
            None => T::deserialize(ValueDecoder(Value::Null)),
            Some(value) => T::deserialize(ValueDecoder(value)),
        };
        decoded.map_err(|error| format!("{} at {key}", open_pipe_stress_report_package::serde_error_kind(&error.to_string())))
    }
    // Every member, exhaustively (a new member is a compile error here).
    (|| -> Result<ReportPackageRequest, String> {
        Ok(ReportPackageRequest {
            package_id: member(&mut fields, "package_id", false)?,
            export_profile_id: member(&mut fields, "export_profile_id", false)?,
            source_model_ref: member(&mut fields, "source_model_ref", false)?,
            source_basis_refs: member(&mut fields, "source_basis_refs", false)?,
            report: member(&mut fields, "report", false)?,
            audit_manifest: member(&mut fields, "audit_manifest", false)?,
            result_documents: member(&mut fields, "result_documents", false)?,
            source_results: member(&mut fields, "source_results", false)?,
            state_comparison_handoff_records: member(&mut fields, "state_comparison_handoff_records", false)?,
            rule_check_aggregate: member(&mut fields, "rule_check_aggregate", true)?,
            solve_rule_check_status: member(&mut fields, "solve_rule_check_status", false)?,
        })
    })()
    .map_err(invalid)
}

/// A request admitted for the save dialog: the assembled package and the
/// screen's warnings.
#[derive(Debug)]
pub struct AdmittedReportPackage {
    pub package: ReportPackageContainerOutcome,
    pub screening: ReportPackageSaveScreening,
    pub local_first: ReportPackageLocalFirstEvidence,
    pub warnings: Vec<Diagnostic>,
    /// W-11 (§3.6): an exact-route request's job and the digests its binding
    /// checked, re-checked after the save dialog and before the write.
    pub exact_custody: Option<(String, CustodyDigests)>,
}

fn text_at<'a>(value: &'a Value, pointer: &str) -> Option<&'a str> {
    value.pointer(pointer).and_then(Value::as_str)
}

/// The disposition screens of §2.6 item 4 over both parsed values, each
/// finding named by its JSON pointer under `/request` or `/model`:
/// - the structural screen (W-6b's, [`screen_value`] and [`malformed_values`],
///   with no exemption: model documents carry rule-pack references only,
///   `schemas/model.schema.yaml:117`, and the report projects safe references,
///   `renderableReportInput.ts:416`): a blocking disposition member at any
///   depth is `PROTECTED_CONTENT_BLOCKED`, and an array or object disposition
///   value, `classification` included, is [`DISPOSITION_MALFORMED`] (it could
///   hide a blocking value);
/// - the store-resolved screen (W-6a's, [`screen_dispositions_in`]), with the
///   model project's store: a blocking record or stored library is
///   `PROTECTED_CONTENT_BLOCKED`, a store read error refuses, and an
///   unresolved reference is a warning.
fn screen_content(request: &Value, model: &Value, project_id: &str, lookup: &mut LibraryLookup<'_>) -> Result<Vec<Diagnostic>, ReportPackageSaveCommandError> {
    let roots = [("/request", request), ("/model", model)];
    let mut blocking = Vec::new();
    for (root, value) in roots {
        for found in screen_value(value) {
            // `<pointer> <status>`; the blocking statuses contain no space.
            let (pointer, status) = found.rsplit_once(' ').expect("pointer and status");
            blocking.push(finding(
                PROTECTED_CONTENT_BLOCKED,
                &format!("{root}{pointer}"),
                format!("The disposition `{status}` blocks the save; request human review of the record's disposition first."),
            ));
        }
        for found in malformed_values(value) {
            let (pointer, kind) = [" is an array", " is an object"]
                .iter()
                .find_map(|suffix| found.strip_suffix(suffix).map(|pointer| (pointer, &suffix[4..])))
                .expect("pointer and value kind");
            blocking.push(finding(
                DISPOSITION_MALFORMED,
                &format!("{root}{pointer}"),
                format!("The disposition member is {kind}, not text, so its disposition cannot be judged; nothing was saved."),
            ));
        }
    }
    let mut warnings = Vec::new();
    for (root, value) in roots {
        match screen_dispositions_in(value, root, PathStyle::Pointer, project_id, LIBRARY_STORE_UNAVAILABLE, lookup) {
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

/// The input manifest records a solve of the bound model: its `model_basis`
/// names the model text's `project.id` (`model_ref`) and embeds the model
/// (`model_payload`), as `inputManifestService.ts` builds it from the solved
/// model and `resultExportAdapter.ts` requires it canonically equal to the
/// session's (`CURRENT_MODEL_PAYLOAD_MISMATCH`). The payload is compared by
/// the SHA-256 of its RFC 8785 form with the model text's (`model_hex`, the
/// audit model hash). Its canonical form is part of the input-manifest text's,
/// so it is no larger than the re-serialization [`parse_lease_bytes`] prices,
/// which is freed by now, as is the input-manifest text.
pub(crate) fn check_manifest_model(input_manifest: &Value, model: &Value, model_hex: &str) -> Result<(), ReportPackageSaveCommandError> {
    let project_id = text_at(model, "/project/id");
    if project_id.is_none() || text_at(input_manifest, "/model_basis/model_ref") != project_id {
        return Err(refuse(
            INPUT_MANIFEST_UNBOUND,
            "/input_manifest/model_basis/model_ref",
            "The input manifest's model_basis.model_ref is not the model text's project.id: it records another model's solve.",
        ));
    }
    let embedded = input_manifest.pointer("/model_basis/model_payload").map(|payload| sha256_hex(&canonical_json(payload)));
    if embedded.as_deref() != Some(model_hex) {
        return Err(refuse(
            INPUT_MANIFEST_UNBOUND,
            "/input_manifest/model_basis/model_payload",
            format!("The input manifest's embedded model_basis.model_payload is not the model text (sha256 {model_hex}): it records the solve of another model."),
        ));
    }
    Ok(())
}

/// One rule pack's identity in the §2.5 comparison: `{rule_pack_id, version,
/// checksum value}`.
type RulePackIdentity = (String, String, String);

/// The solve's rule packs: the input manifest's `active_rule_packs`, as the
/// session records them (`InputManifestRulePack`: a non-empty id and version
/// and a bare lowercase 64-hex `checksum_sha256`). Set semantics: a repeated
/// entry counts once, as in the webview's check.
pub(crate) fn solve_rule_packs(input_manifest: &Value) -> Result<BTreeSet<RulePackIdentity>, ReportPackageSaveCommandError> {
    let invalid = || refuse(INPUT_MANIFEST_INVALID, "/input_manifest/active_rule_packs", "The input manifest records no complete active_rule_packs inventory (rule_pack_id, version and SHA-256 checksum_sha256 per pack).");
    let packs = input_manifest.get("active_rule_packs").and_then(Value::as_array).ok_or_else(invalid)?;
    packs
        .iter()
        .map(|pack| {
            let field = |key: &str| pack.get(key).and_then(Value::as_str).filter(|value| !value.trim().is_empty());
            let checksum = field("checksum_sha256").filter(|value| value.len() == 64 && value.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')));
            match (field("rule_pack_id"), field("version"), checksum) {
                (Some(id), Some(version), Some(checksum)) => Ok((id.to_string(), version.to_string(), checksum.to_string())),
                _ => Err(invalid()),
            }
        })
        .collect()
}

/// The package's rule packs as one of its records lists them (the array at
/// `pointer`, whose entries name their version `version_key`), keyed as the
/// webview keys them: a missing checksum value is "not recorded", which no
/// recorded SHA-256 equals. Set semantics.
fn package_rule_packs(request: &Value, pointer: &str, version_key: &str) -> BTreeSet<RulePackIdentity> {
    let text = |pack: &Value, pointer: &str, absent: &str| pack.pointer(pointer).and_then(Value::as_str).unwrap_or(absent).to_string();
    request
        .pointer(pointer)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
        .iter()
        .map(|pack| (text(pack, "/rule_pack_id", ""), text(pack, &format!("/{version_key}"), ""), text(pack, "/checksum/value", "not recorded")))
        .collect()
}

fn identity_text(set: &BTreeSet<RulePackIdentity>) -> String {
    if set.is_empty() {
        return "none".to_string();
    }
    set.iter().map(|(id, version, checksum)| format!("{id}@{version} (sha256 {checksum})")).collect::<Vec<_>>().join(", ")
}

/// §2.5's decided rule, re-checked natively: the package's rule packs (the
/// rule-check run's, as the report and the audit manifest record them; else
/// none) equal the solve's, the bound input manifest's `active_rule_packs`.
fn check_rule_packs_agree(request: &Value, solve: &BTreeSet<RulePackIdentity>) -> Result<(), ReportPackageSaveCommandError> {
    for (pointer, version_key) in [("/report/calculation_report/rule_pack_refs", "version"), ("/audit_manifest/rule_pack_refs", "rule_pack_version")] {
        let package = package_rule_packs(request, pointer, version_key);
        if package != *solve {
            return Err(refuse(
                RULE_PACKS_DISAGREE,
                &format!("/request{pointer}"),
                format!(
                    "The package's rule packs (the rule check's: {}) are not the packs the solve recorded ({}). Run the rule check with the model's pack, or change the model's rule-pack references and solve again.",
                    identity_text(&package),
                    identity_text(solve)
                ),
            ));
        }
    }
    Ok(())
}

fn exact_refused(refusal: ExactRefusal, path: &str) -> ReportPackageSaveCommandError {
    let message = refusal.message.strip_prefix(&format!("{}: ", refusal.code)).unwrap_or(&refusal.message).to_string();
    refuse(refusal.code, path, message)
}

/// Object form: no struct in the request, at any depth, is in positional
/// (JSON array) form. The shared decode reads a positional struct by field
/// order, while the save's exact detection, source binding, disposition
/// screen and context checks read members by key, so the save refuses one
/// before any of them, naming the fixed path only
/// (`wire::request_positional_struct_path`: a positional request, and each
/// decoded typed member and the report compared with its JSON). The ingress
/// pricing of a positional text is unchanged; only its admission refuses.
fn check_request_object_form(request: &Value) -> Result<(), ReportPackageSaveCommandError> {
    match open_pipe_stress_report_package::wire::request_positional_struct_path(request) {
        Some(path) => Err(refuse(REQUEST_POSITIONAL, "/request", format!("The request writes a struct in positional (array) form at {path}."))),
        None => Ok(()),
    }
}

/// Whether the parsed request is an exact-route request (§3.6): its result
/// document has an exact identity or its report carries `exact_route`.
fn exact_request(request: &Value) -> bool {
    let identity = text_at(request, "/result_documents/0/result_envelope/producer/semantic_contract_id");
    identity.is_some_and(open_pipe_stress_report_package::exact_sections::is_exact_identity)
        || request.pointer("/report/exact_route").is_some_and(|route| !route.is_null())
}

/// The native source binding of an exact-route save (§3.6): steps 1-5, then
/// the request's source and the model text against the delivered result and
/// the solved model. Returns the job and the digests it checked.
fn bind_exact_save_source(
    request: &Value,
    model_text: &str,
    exact_result_text: Option<&str>,
    exact_intake_text: Option<&str>,
    custody: &CustodyLookup<'_>,
) -> Result<(String, CustodyDigests), ReportPackageSaveCommandError> {
    const ADMISSION: &str = "/request/report/exact_route/admission";
    let admission = request.pointer("/report/exact_route/admission").cloned().unwrap_or(Value::Null);
    let digests = exact_binding::check_custody(&admission, custody).map_err(|refusal| exact_refused(refusal, &format!("{ADMISSION}/native_job_id")))?;
    exact_binding::check_delivered_texts(&admission, &digests, exact_result_text, exact_intake_text)
        .map_err(|refusal| exact_refused(refusal, ADMISSION))?;
    let bound = exact_binding::read_delivered_source(&admission, &digests, exact_result_text.unwrap_or(""), exact_intake_text.unwrap_or(""))
        .map_err(|refusal| exact_refused(refusal, ADMISSION))?;
    // The save's own equalities: the request's source is the delivered result,
    // and the (canonical) model text is the solved model.
    let source = request.pointer("/source_results/0").unwrap_or(&Value::Null);
    let source_sha256 = open_pipe_stress_result_export::derivative::digest(source).ok();
    let delivered_sha256 = open_pipe_stress_result_export::derivative::digest(&bound.source).ok();
    if source_sha256.is_none() || source_sha256 != delivered_sha256 {
        return Err(exact_refused(exact_binding::not_delivered("result"), "/request/source_results/0"));
    }
    if sha256_hex(model_text) != bound.model_sha256 {
        return Err(exact_refused(exact_binding::not_delivered("model"), "/model"));
    }
    let job = admission["native_job_id"].as_str().unwrap_or_default().to_string();
    Ok((job, digests))
}

/// The admission of §2.6 items 2-6, before any file is chosen, with no
/// custody: every exact-route request refuses at its custody binding (§3.6).
pub fn admit_report_package(
    request: ReportPackageSaveRequest,
    software: &ExpectedSoftware,
    lookup: &mut LibraryLookup<'_>,
) -> Result<AdmittedReportPackage, ReportPackageSaveCommandError> {
    admit_report_package_in(request, software, lookup, &no_custody)
}

/// The admission of §2.6 items 2-6 with the registry's custody lookup (the
/// command's), before any file is chosen.
pub fn admit_report_package_in(
    request: ReportPackageSaveRequest,
    software: &ExpectedSoftware,
    lookup: &mut LibraryLookup<'_>,
    custody: &CustodyLookup<'_>,
) -> Result<AdmittedReportPackage, ReportPackageSaveCommandError> {
    let ReportPackageSaveRequest { request_text, model_text, input_manifest_text, screening, local_first, exact_result_text, exact_intake_text } =
        request;
    validate_report_package_save_evidence(&screening, local_first.as_ref())?;
    let local_first = local_first.expect("validated local-first evidence");
    check_text_sizes(&request_text, &model_text, &input_manifest_text)?;
    check_exact_text_sizes(exact_result_text.as_deref(), exact_intake_text.as_deref())?;

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
    check_request_object_form(&request_value)?;
    // W-11 (§3.6): the native source binding of an exact-route request, before
    // the other content checks; a pressure-free request carries no texts.
    let exact_custody = if exact_request(&request_value) {
        Some(bind_exact_save_source(&request_value, &model_text, exact_result_text.as_deref(), exact_intake_text.as_deref(), custody)?)
    } else if exact_result_text.is_some() || exact_intake_text.is_some() {
        return Err(exact_refused(exact_binding::sections_mismatch("exact_route"), "/request/report/exact_route"));
    } else {
        None
    };
    drop(exact_result_text);
    drop(exact_intake_text);
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
    // The run's input manifest: its SHA-256 is the audit input-manifest hash.
    let input_manifest_value = parse_canonical("input_manifest", &input_manifest_text)?;
    let input_manifest_hex = sha256_hex(&input_manifest_text);
    drop(input_manifest_text);
    if text_at(&request_value, "/audit_manifest/input_manifest_hash/value") != Some(input_manifest_hex.as_str()) {
        return Err(refuse(
            INPUT_MANIFEST_UNBOUND,
            "/request/audit_manifest/input_manifest_hash/value",
            format!("The request's audit input-manifest hash is not the input-manifest text's SHA-256 ({input_manifest_hex})."),
        ));
    }
    check_manifest_model(&input_manifest_value, &model_value, &model_hex)?;
    let solve_packs = solve_rule_packs(&input_manifest_value)?;
    drop(input_manifest_value);

    // Disposition screening of both values, then the context bindings.
    let project_id = text_at(&model_value, "/project/id").unwrap_or("").to_string();
    let warnings = screen_content(&request_value, &model_value, &project_id, lookup)?;
    check_context(&request_value, &model_value, software)?;
    drop(model_value);
    // The audit hash names the input manifest by the session's reference for
    // that hash and the (now bound) model, which the shared producer binds to
    // the document, the report and the handoff records; then §2.5's rule.
    let manifest_ref = crate::analysis_runs::input_manifest_ref(&project_id, &input_manifest_hex);
    if request_value.pointer("/audit_manifest/input_manifest_hash/payload_ref") != manifest_ref.get("ref") {
        return Err(refuse(
            INPUT_MANIFEST_UNBOUND,
            "/request/audit_manifest/input_manifest_hash/payload_ref",
            format!("The request's audit input-manifest hash does not name the input-manifest text ({}).", manifest_ref["ref"].as_str().unwrap_or("")),
        ));
    }
    check_rule_packs_agree(&request_value, &solve_packs)?;
    drop(solve_packs);

    // Typed conversion and the shared producer (§2.3).
    let package = report_package_bridge::assemble_request(typed_request(request_value)?).map_err(ReportPackageSaveCommandError::bridge)?;
    if package.export_blocked {
        return Err(ReportPackageSaveCommandError::bridge(format!("{PRODUCER_BLOCKED}: {}", package.blocking_reasons.join(" | "))));
    }
    Ok(AdmittedReportPackage { package, screening, local_first, warnings, exact_custody })
}

impl AdmittedReportPackage {
    fn receipt(self, outcome: &str, stage: &str, code: String, written: Option<atomic_report_package_save::AtomicSaveOutcome>) -> ReportPackageSaveReceipt {
        let Self { package, screening, local_first, warnings, exact_custody: _ } = self;
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
/// unusable path. No custody: an exact-route package refuses before the write.
pub fn save_admitted_report_package(
    admitted: AdmittedReportPackage,
    choose: impl FnOnce(&str) -> Option<Result<PathBuf, String>>,
) -> Result<ReportPackageSaveReceipt, ReportPackageSaveCommandError> {
    save_admitted_report_package_in(admitted, &no_custody, choose)
}

/// [`save_admitted_report_package`] with the registry's custody lookup: an
/// exact-route package's custody is checked again after the dialog returns
/// and before the write, against the digests its binding checked (§3.6).
pub fn save_admitted_report_package_in(
    admitted: AdmittedReportPackage,
    custody: &CustodyLookup<'_>,
    choose: impl FnOnce(&str) -> Option<Result<PathBuf, String>>,
) -> Result<ReportPackageSaveReceipt, ReportPackageSaveCommandError> {
    let Some(selected) = choose(&admitted.package.container_file_name) else {
        return Ok(admitted.receipt("cancelled", "picker", "REPORT-PACKAGE-SAVE-CANCELLED".to_string(), None));
    };
    if let Some((job, digests)) = &admitted.exact_custody {
        if custody(job) != Some(*digests) {
            return Err(exact_refused(exact_binding::custody_released(), "/request/report/exact_route/admission/native_job_id"));
        }
    }
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

    /// The committed invented input manifest with the desktop session
    /// manifest's `model_basis` (`model_ref` and the embedded `model_payload`,
    /// for `model`) and `active_rule_packs` members (`inputManifestService.ts`).
    pub(crate) fn input_manifest(model: &Value, active_rule_packs: Value) -> Value {
        let mut manifest: Value = serde_json::from_str(INPUT_MANIFEST).unwrap();
        manifest["model_basis"] = json!({"model_ref": model["project"]["id"].clone(), "model_payload": model.clone()});
        manifest["active_rule_packs"] = active_rule_packs;
        manifest
    }

    /// One invented `active_rule_packs` entry (`InputManifestRulePack`).
    pub(crate) fn active_pack(id: &str, version: &str, checksum: &str) -> Value {
        json!({"rule_pack_id": id, "version": version, "checksum_sha256": checksum, "source_notice": "Invented non-code example only."})
    }

    /// The desktop-profile request derived by the shared builder from the
    /// committed witness and `model` (whose canonical SHA-256 becomes every
    /// model hash), with the desktop's native report context, for a solve
    /// that recorded no rule packs.
    pub(crate) fn desktop_request(model: &Value) -> Value {
        desktop_request_for(model, &input_manifest(model, json!([])))
    }

    /// As [`desktop_request`], for the run's input manifest `manifest`: its
    /// canonical SHA-256 is every input-manifest hash, and its reference is the
    /// session's (`input-manifest:<model>:<sha256>`).
    pub(crate) fn desktop_request_for(model: &Value, manifest: &Value) -> Value {
        let witness: Value = serde_json::from_str(WITNESS).unwrap();
        let runner = &witness["export_results"];
        let solver = &runner["audit_manifest"]["solver_version"];
        let manifest_ref = crate::analysis_runs::input_manifest_ref(model["project"]["id"].as_str().unwrap(), &sha256_hex(&canonical_json(manifest)));
        let inputs = Inputs {
            model,
            source: &runner["source_results"][0],
            producer_document: &runner["result_documents"][0],
            solve_request: &Value::Null,
            input_manifest: manifest,
            input_manifest_id: manifest_ref["ref"].as_str().unwrap(),
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

    /// `request` with a rule-check run of pack `id` (one failed and one
    /// incomplete check under the document's own `RULE_INPUTS_INCOMPLETE`
    /// token), its report reference carrying a declared `sha256`/`JCS`
    /// checksum and the audit reference its projection: consistent with each
    /// other, whatever the input manifest recorded.
    pub(crate) fn with_rule_pack(request: &Value, id: &str, version: &str, checksum: &str) -> Value {
        let mut request = request.clone();
        request["rule_check_aggregate"] = json!("RULE_INPUTS_INCOMPLETE");
        request["report"]["user_rule_checks"] = json!({
            "rule_pack_id": id,
            "grammar_version": "1.0.0",
            "aggregate_status": "RULE_INPUTS_INCOMPLETE",
            "checks": [
                {"check_id": "invented-check-a", "status": "USER_RULE_FAILED", "missing_inputs": []},
                {"check_id": "invented-check-b", "status": "RULE_INPUTS_INCOMPLETE", "missing_inputs": ["invented_input_x"]}
            ]
        });
        request["report"]["calculation_report"]["rule_pack_refs"] = json!([{
            "rule_pack_id": id,
            "version": version,
            "checksum": {"algorithm": "sha256", "canonicalization": "JCS", "payload_ref": {"ref_type": "rule_pack", "ref_id": id}, "value": checksum},
            "source_notice": "Invented non-code example only.",
            "redistribution_status": "invented_non_engineering_example",
            "completeness_status": "not_checked",
            "private_payload_redacted": true
        }]);
        request["audit_manifest"]["rule_pack_refs"] = json!([{
            "rule_pack_id": id,
            "rule_pack_version": version,
            "source_notice": "Invented non-code example only.",
            "redistribution_status": "invented_non_engineering_example",
            "checksum": {"algorithm": "sha256", "canonicalization": "rule_pack_payload_jcs", "payload_kind": "rule_pack_reference", "payload_ref": id, "value": checksum},
            "private_payload_redacted": true
        }]);
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
    /// bound to the request's canonical text, and the input manifest of
    /// [`desktop_request`] for `model` (no rule packs).
    pub(crate) fn save_request(request: &Value, model: &Value) -> ReportPackageSaveRequest {
        save_request_with(request, model, &input_manifest(model, json!([])))
    }

    /// As [`save_request`], with the input manifest `manifest`.
    pub(crate) fn save_request_with(request: &Value, model: &Value, manifest: &Value) -> ReportPackageSaveRequest {
        let request_text = canonical_json(request);
        let payload = hash_of(&request_text);
        ReportPackageSaveRequest {
            request_text,
            model_text: canonical_json(model),
            input_manifest_text: canonical_json(manifest),
            screening: screening(payload),
            local_first: Some(local_first()),
            exact_result_text: None,
            exact_intake_text: None,
        }
    }

    /// The same arguments as the command's raw args object.
    pub(crate) fn args_body(request: &ReportPackageSaveRequest) -> Value {
        json!({
            "requestCanonicalText": request.request_text,
            "modelCanonicalText": request.model_text,
            "inputManifestCanonicalText": request.input_manifest_text,
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
        let _save_lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
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
        assert_eq!(check_text_sizes("{}", &over, "{}").unwrap_err().code, TOO_LARGE);
        assert_eq!(parse_lease_bytes("{}", &over, "{}").unwrap_err().code, TOO_LARGE);
        let at = "x".repeat(MAX_MODEL_TEXT_BYTES as usize);
        assert!(check_text_sizes("{}", &at, "{}").is_ok());
        // At the cap, the prescan refuses text that is not JSON before any parse.
        assert_eq!(parse_lease_bytes("{}", &at, "{}").unwrap_err().code, NOT_CANONICAL);
        drop((over, at));
        let over = "x".repeat(MAX_REQUEST_TEXT_BYTES as usize + 1);
        assert_eq!(check_text_sizes(&over, "{}", "{}").unwrap_err().diagnostics[0].path, "/request");
        drop(over);
        // The input manifest's cap: two model-document caps.
        let over = "x".repeat(MAX_INPUT_MANIFEST_TEXT_BYTES as usize + 1);
        let error = parse_lease_bytes("{}", "{}", &over).unwrap_err();
        assert_eq!((error.code.as_str(), error.diagnostics[0].path.as_str()), (TOO_LARGE, "/input_manifest"));
        drop(over);
        let at = "x".repeat(MAX_INPUT_MANIFEST_TEXT_BYTES as usize);
        assert!(check_text_sizes("{}", "{}", &at).is_ok());
        assert_eq!(parse_lease_bytes("{}", "{}", &at).unwrap_err().code, NOT_CANONICAL);
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
            assert!(!paths.is_empty() && paths.iter().all(|path| path.starts_with("/request/")), "{label}: {paths:?}");
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
        assert_eq!(code, PROTECTED_CONTENT_BLOCKED);
        // The structural screen names the member, the store-resolved screen the record.
        assert_eq!(paths, vec!["/model/materials/0/provenance_record/review_status".to_string(), "/model/materials/0/provenance_record".to_string()]);
    }

    /// W-11 slice (c) (DEL-08-01 Design `exact-route-report-sections.md`
    /// §4.3, §6.2): the report's FR-REP-003 records, values and library
    /// references are screened in the request as every object is. A
    /// `quarantined`, `rejected` or `protected_suspected` record refuses the
    /// save; a `private_only` one is admitted.
    #[test]
    fn fr_rep_003_records_with_a_blocking_disposition_refuse_the_save() {
        let record = |member: &str, status: &str| {
            let mut record = json!({
                "source_name": "Invented material datasheet", "source_location": "invented-datasheet-shelf", "source_license": "invented-license",
                "contributor": "invented contributor", "contributor_certification": "invented certification",
                "redistribution_status": "private_only", "review_status": "pending", "privacy_classification": "private_project_data"
            });
            record[member] = json!(status);
            record
        };
        let with_records = |record: Value| {
            move |r: &mut Value| {
                let sections = &mut r["report"]["report_sections"];
                sections["provenance_notes"].as_array_mut().unwrap().push(record.clone());
                sections["user_supplied_values"].as_array_mut().unwrap().push(json!({
                    "value_id": "material-provenance:material:invented-steel", "value_category": "report_metadata",
                    "source": {"ref_type": "material", "ref_id": "material:invented-steel"}, "provenance": record.clone(),
                    "privacy_classification": "private_project_data", "required_for": ["reporting", "human_review"],
                    "review_status": record["review_status"].clone(), "missing_data_finding": false
                }));
            }
        };
        assert!(admit(edited(with_records(record("review_status", "pending")))).is_ok());
        for (member, status) in [("review_status", "quarantined"), ("review_status", "rejected"), ("redistribution_status", "protected_suspected")] {
            let (code, paths) = refusal(admit(edited(with_records(record(member, status)))));
            assert_eq!(code, PROTECTED_CONTENT_BLOCKED, "{member} {status}");
            assert!(paths.iter().any(|path| path.starts_with("/request/report/report_sections/provenance_notes/") && path.ends_with(member)), "{paths:?}");
            assert!(paths.iter().any(|path| path.starts_with("/request/report/report_sections/user_supplied_values/") && path.ends_with(&format!("provenance/{member}"))), "{paths:?}");
        }
        // A library reference's record.
        for (member, status) in [("review_status", "quarantined"), ("privacy_classification", "protected_suspected")] {
            let library = json!([{
                "reference_id": "library-ref:1", "library_kind": "material", "library_id": "invented-material-library",
                "library_version": "not recorded", "record_id": "not recorded", "entity_refs": ["material:invented-steel"], "provenance": record(member, status)
            }]);
            let (code, paths) = refusal(admit(edited(move |r: &mut Value| r["report"]["library_references"] = library.clone())));
            assert_eq!((code.as_str(), paths[0].as_str()), (PROTECTED_CONTENT_BLOCKED, format!("/request/report/library_references/0/provenance/{member}").as_str()));
        }
    }

    /// The structural screen: what W-6a's record screen does not judge
    /// (§2.6 item 4) is refused in either value, every object at every depth.
    #[test]
    fn the_structural_screen_judges_every_object_in_both_values() {
        let model_cases: Vec<(&str, fn(&mut Value), &str)> = vec![
            ("privacy class", |m| m["materials"][0]["provenance_record"] = json!({"privacy_classification": "protected_suspected"}),
                "/model/materials/0/provenance_record/privacy_classification"),
            ("direct member", |m| m["nodes"][0]["review_status"] = json!("rejected"), "/model/nodes/0/review_status"),
            ("nested member", |m| m["nodes"][0]["invented"] = json!({"deep": [{"redistribution_status": "protected_suspected"}]}),
                "/model/nodes/0/invented/deep/0/redistribution_status"),
            ("escaped key", |m| m["nodes"][0]["a/b~c"] = json!({"lifecycle_status": "quarantined"}), "/model/nodes/0/a~1b~0c/lifecycle_status"),
        ];
        for (label, edit, path) in model_cases {
            let mut blocked = model();
            edit(&mut blocked);
            let (code, paths) = refusal(admit(save_request(&desktop_request(&blocked), &blocked)));
            assert_eq!((code.as_str(), paths[0].as_str()), (PROTECTED_CONTENT_BLOCKED, path), "{label}");
        }
        let request_cases: Vec<(&str, fn(&mut Value), &str)> = vec![
            ("report provenance", |r| r["report"]["report_sections"]["provenance_notes"][0]["review_status"] = json!("quarantined"),
                "/request/report/report_sections/provenance_notes/0/review_status"),
            ("document", |r| r["result_documents"][0]["result_envelope"]["invented"] = json!({"privacy_classification": "protected_suspected"}),
                "/request/result_documents/0/result_envelope/invented/privacy_classification"),
            ("audit rule pack", |r| r["audit_manifest"]["rule_pack_refs"] = json!([{"redistribution_status": "rejected"}]),
                "/request/audit_manifest/rule_pack_refs/0/redistribution_status"),
        ];
        for (label, edit, path) in request_cases {
            let (code, paths) = refusal(admit(edited(edit)));
            assert_eq!((code.as_str(), paths[0].as_str()), (PROTECTED_CONTENT_BLOCKED, path), "{label}");
        }
        // Non-blocking dispositions at depth are admitted.
        let mut clear = model();
        clear["nodes"][0]["provenance_record"] = json!({"redistribution_status": "private_only", "review_status": "unknown", "privacy_classification": "private_project_data"});
        assert!(admit(save_request(&desktop_request(&clear), &clear)).is_ok());
    }

    /// A disposition member whose value is a list or an object could hide a
    /// blocking value: refused as malformed, in either value.
    #[test]
    fn non_string_disposition_values_are_refused() {
        let (code, paths) = refusal(admit(edited(|r| r["report"]["report_sections"]["provenance_notes"][0]["review_status"] = json!(["accepted", "quarantined"]))));
        assert_eq!((code.as_str(), paths[0].as_str()), (DISPOSITION_MALFORMED, "/request/report/report_sections/provenance_notes/0/review_status"));
        let mut malformed = model();
        malformed["nodes"][0]["redistribution_status"] = json!({"value": "private_only"});
        let (code, paths) = refusal(admit(save_request(&desktop_request(&malformed), &malformed)));
        assert_eq!((code.as_str(), paths[0].as_str()), (DISPOSITION_MALFORMED, "/model/nodes/0/redistribution_status"));
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
        // The input manifest is that model's, so the manifest's model binding
        // holds and the context binding is the one that refuses.
        let mut other = model();
        other["project"]["id"] = json!("project:invented-other");
        let manifest = input_manifest(&other, json!([]));
        let mut request = desktop_request_for(&model(), &manifest);
        request["audit_manifest"]["model_hash"]["value"] = json!(sha256_hex(&canonical_json(&other)));
        let (code, paths) = refusal(admit(save_request_with(&request, &other, &manifest)));
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
        // Codex review round 3 of #1639: the refusal names the member and the
        // kind, never the offending value, in the typed request and in the
        // report (an echoed value can be a private path).
        let typed = "/Users/invented-user/private-datasheets/steel-a.pdf";
        let error = admit(edited(|r| r["source_basis_refs"] = json!(typed))).unwrap_err();
        assert_eq!(error.code, REQUEST_INVALID);
        assert!(error.message.ends_with("The request is not a version 2.0.0 report-package request: invalid type at source_basis_refs."), "{}", error.message);
        let error = admit(edited(|r| r["report"]["report_sections"]["provenance_notes"][0]["review_status"] = json!(typed))).unwrap_err();
        let text = format!("{} {} {:?}", error.code, error.message, error.diagnostics);
        assert!(!text.contains(typed), "{text}");
    }

    const PACK_A: &str = "invented-pack-a";
    const PACK_B: &str = "invented-pack-b";

    fn hex(digit: char) -> String {
        digit.to_string().repeat(64)
    }

    /// The input manifest of a solve of [`model`] that recorded `packs`.
    fn manifest_with(packs: &[(&str, &str, String)]) -> Value {
        input_manifest(&model(), Value::Array(packs.iter().map(|(id, version, checksum)| active_pack(id, version, checksum)).collect()))
    }

    /// Codex W-18a (b2) round 1 (the bypass, before this binding admitted):
    /// a direct request whose input-manifest hash and reference name a solve
    /// that recorded pack A, with report and audit references consistent with
    /// each other for pack B, or for none, is refused with
    /// `REPORT-PACKAGE-RULE-PACKS-DISAGREE` before the save dialog, naming
    /// both sets; nothing is written.
    #[test]
    fn a_package_whose_rule_packs_are_not_the_bound_input_manifests_is_refused() {
        let manifest = manifest_with(&[(PACK_A, "1.0.0", hex('a'))]);
        let none = desktop_request_for(&model(), &manifest);
        let pack_b = with_rule_pack(&none, PACK_B, "1.0.0", &hex('b'));
        for (label, request, package) in [("none", &none, "none".to_string()), ("pack B", &pack_b, format!("{PACK_B}@1.0.0 (sha256 {})", hex('b')))] {
            let root = sandbox("rule-packs-disagree");
            let mut picks = 0usize;
            let outcome = admit(save_request_with(request, &model(), &manifest))
                .and_then(|admitted| save_admitted_report_package(admitted, |_| { picks += 1; Some(Ok(root.join("refused.opsproj"))) }));
            let error = match outcome { Err(error) => error, Ok(receipt) => panic!("{label}: admitted ({})", receipt.code) };
            assert_eq!((error.code.as_str(), error.stage.as_str()), (RULE_PACKS_DISAGREE, "admit"), "{label}: {}", error.message);
            assert_eq!(error.diagnostics[0].path, "/request/report/calculation_report/rule_pack_refs", "{label}");
            assert!(error.message.contains(&format!("the rule check's: {package})")), "{label}: {}", error.message);
            assert!(error.message.contains(&format!("{PACK_A}@1.0.0 (sha256 {})", hex('a'))), "{label}: {}", error.message);
            assert_eq!(picks, 0, "{label}: no save dialog is requested");
            assert_eq!(fs::read_dir(&root).unwrap().count(), 0, "{label}: nothing is written");
            fs::remove_dir_all(root).unwrap();
        }
        // The reverse: a rule check of pack A when the solve recorded none.
        let checked = with_rule_pack(&desktop(), PACK_A, "1.0.0", &hex('a'));
        assert_eq!(refusal(admit(save_request(&checked, &model()))).0, RULE_PACKS_DISAGREE);
        // The report and the audit references are each compared: an audit
        // reference naming the solve's pack does not stand in for the report's.
        let mut audit_only = with_rule_pack(&none, PACK_B, "1.0.0", &hex('b'));
        audit_only["audit_manifest"]["rule_pack_refs"] = with_rule_pack(&none, PACK_A, "1.0.0", &hex('a'))["audit_manifest"]["rule_pack_refs"].clone();
        assert_eq!(refusal(admit(save_request_with(&audit_only, &model(), &manifest))).0, RULE_PACKS_DISAGREE);
    }

    /// The agreeing cases still save: no packs on either side (every request
    /// above), and the rule check of the solve's own pack.
    #[test]
    fn a_rule_check_of_the_solves_pack_saves() {
        let manifest = manifest_with(&[(PACK_A, "1.0.0", hex('a'))]);
        let request = with_rule_pack(&desktop_request_for(&model(), &manifest), PACK_A, "1.0.0", &hex('a'));
        let admitted = admit(save_request_with(&request, &model(), &manifest)).unwrap_or_else(|error| panic!("{}", error.message));
        let _save_lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
        let root = sandbox("rule-packs-agree");
        let receipt = save_admitted_report_package(admitted, |_| Some(Ok(root.join("agreeing.opsproj")))).unwrap();
        assert_eq!(receipt.code, "REPORT-PACKAGE-SAVED");
        let read = read_report_package_container(&fs::read(root.join("agreeing.opsproj")).unwrap()).unwrap();
        assert!(!read.export_blocked, "{:?}", read.blocking_reasons);
        fs::remove_dir_all(root).unwrap();
        // A repeated solve entry counts once (set semantics, as the webview's check).
        let repeated = manifest_with(&[(PACK_A, "1.0.0", hex('a')), (PACK_A, "1.0.0", hex('a'))]);
        let request = with_rule_pack(&desktop_request_for(&model(), &repeated), PACK_A, "1.0.0", &hex('a'));
        assert!(admit(save_request_with(&request, &model(), &repeated)).is_ok());
        // A repeat does not stand in for another pack, and version and checksum are each compared.
        let two = manifest_with(&[(PACK_A, "1.0.0", hex('a')), (PACK_A, "1.0.0", hex('a')), (PACK_B, "1.0.0", hex('b'))]);
        let request = with_rule_pack(&desktop_request_for(&model(), &two), PACK_A, "1.0.0", &hex('a'));
        assert_eq!(refusal(admit(save_request_with(&request, &model(), &two))).0, RULE_PACKS_DISAGREE);
        for (version, checksum) in [("2.0.0", hex('a')), ("1.0.0", hex('c'))] {
            let request = with_rule_pack(&desktop_request_for(&model(), &manifest), PACK_A, version, &checksum);
            assert_eq!(refusal(admit(save_request_with(&request, &model(), &manifest))).0, RULE_PACKS_DISAGREE, "{version} {checksum}");
        }
    }

    /// A report reference without a declared checksum value is keyed "not
    /// recorded" (the webview's key), which no recorded SHA-256 equals.
    #[test]
    fn a_rule_pack_without_a_checksum_value_is_not_the_solves() {
        let manifest = manifest_with(&[(PACK_A, "1.0.0", hex('a'))]);
        let mut request = with_rule_pack(&desktop_request_for(&model(), &manifest), PACK_A, "1.0.0", &hex('a'));
        request["report"]["calculation_report"]["rule_pack_refs"][0]["checksum"].as_object_mut().unwrap().remove("value");
        let error = admit(save_request_with(&request, &model(), &manifest)).err().expect("refused");
        assert_eq!(error.code, RULE_PACKS_DISAGREE);
        assert!(error.message.contains(&format!("{PACK_A}@1.0.0 (sha256 not recorded)")), "{}", error.message);
    }

    /// The input-manifest text is bound: a text other than the one the
    /// request's audit hash records, or one the hash's reference does not
    /// name, is refused before any content check; so is a manifest without a
    /// complete `active_rule_packs` inventory.
    #[test]
    fn an_input_manifest_text_other_than_the_audited_one_is_refused() {
        let manifest = manifest_with(&[(PACK_A, "1.0.0", hex('a'))]);
        let request = desktop_request_for(&model(), &manifest);
        // Tampered: the solve's pack replaced by none, the audited hash kept.
        let tampered = manifest_with(&[]);
        let (code, paths) = refusal(admit(save_request_with(&request, &model(), &tampered)));
        assert_eq!((code.as_str(), paths[0].as_str()), (INPUT_MANIFEST_UNBOUND, "/request/audit_manifest/input_manifest_hash/value"));
        // The same manifest, not canonical.
        let mut pretty = save_request_with(&request, &model(), &manifest);
        pretty.input_manifest_text = serde_json::to_string_pretty(&manifest).unwrap();
        assert_eq!(refusal(admit(pretty)), (NOT_CANONICAL.to_string(), vec!["/input_manifest".to_string()]));
        // The audit hash value recomputed for the tampered text, but its
        // reference (and every record that names it) still the original's.
        let mut rehashed = desktop_request(&model());
        rehashed["audit_manifest"]["input_manifest_hash"]["payload_ref"] = request["audit_manifest"]["input_manifest_hash"]["payload_ref"].clone();
        let (code, paths) = refusal(admit(save_request_with(&rehashed, &model(), &tampered)));
        assert_eq!((code.as_str(), paths[0].as_str()), (INPUT_MANIFEST_UNBOUND, "/request/audit_manifest/input_manifest_hash/payload_ref"));
        // An incomplete inventory, bound by its own hash.
        for packs in [Value::Null, json!([{"rule_pack_id": PACK_A, "version": "1.0.0", "checksum_sha256": "sha256:a"}]), json!([{"rule_pack_id": PACK_A, "checksum_sha256": hex('a')}])] {
            let mut incomplete = input_manifest(&model(), packs.clone());
            if packs.is_null() {
                incomplete.as_object_mut().unwrap().remove("active_rule_packs");
            }
            let (code, paths) = refusal(admit(save_request_with(&desktop_request_for(&model(), &incomplete), &model(), &incomplete)));
            assert_eq!((code.as_str(), paths[0].as_str()), (INPUT_MANIFEST_INVALID, "/input_manifest/active_rule_packs"), "{packs}");
        }
    }

    /// The document with its derivative hash recomputed after an edit, so only
    /// the check the edit targets can refuse it.
    fn rehash(mut document: Value) -> Value {
        use open_pipe_stress_result_export::derivative::{checksum, reference};
        let envelope_id = document["result_envelope"]["envelope_id"].as_str().unwrap().to_string();
        document["result_envelope"]["reproducibility"].as_object_mut().unwrap().remove("derivative_hash");
        let hash = checksum(&document, "derivative_document_excludes_own_hash", reference("derivative_document", &envelope_id)).unwrap();
        document["result_envelope"]["reproducibility"]["derivative_hash"] = hash;
        document
    }

    /// Codex W-18a (b2) round 2, finding 1 (admitted before this binding):
    /// model B's input manifest, whose digest H the request's references name
    /// as `input-manifest:A:H`, sent with model A and A's result, is refused:
    /// the manifest's `model_basis` must be the bound model. So is a manifest
    /// naming model A whose embedded payload is another model, and a manifest
    /// that embeds no model.
    #[test]
    fn an_input_manifest_of_another_model_is_refused() {
        let project = model()["project"]["id"].as_str().unwrap().to_string();
        let mut other = model();
        other["project"]["id"] = json!("project:invented-other");
        other["project"]["name"] = json!("Invented other model");
        let manifest_b = input_manifest(&other, json!([]));
        let relabelled = desktop_request_for(&model(), &manifest_b);
        // The session's reference for model A and B's digest: `input-manifest:A:H`.
        let relabel = crate::analysis_runs::input_manifest_ref(&project, &sha256_hex(&canonical_json(&manifest_b)));
        assert_eq!(relabelled["audit_manifest"]["input_manifest_hash"]["payload_ref"], relabel["ref"]);
        let (code, paths) = refusal(admit(save_request_with(&relabelled, &model(), &manifest_b)));
        assert_eq!((code.as_str(), paths[0].as_str()), (INPUT_MANIFEST_UNBOUND, "/input_manifest/model_basis/model_ref"));
        // Model A's id, another model's payload (an edit of A after the solve).
        let mut edited = model();
        edited["project"]["description"] = json!("Invented edit after the solve");
        let manifest = input_manifest(&edited, json!([]));
        assert_eq!(manifest["model_basis"]["model_ref"], json!(project));
        let request = desktop_request_for(&model(), &manifest);
        let (code, paths) = refusal(admit(save_request_with(&request, &model(), &manifest)));
        assert_eq!((code.as_str(), paths[0].as_str()), (INPUT_MANIFEST_UNBOUND, "/input_manifest/model_basis/model_payload"));
        // No embedded model.
        let mut bare = input_manifest(&model(), json!([]));
        bare["model_basis"].as_object_mut().unwrap().remove("model_payload");
        let request = desktop_request_for(&model(), &bare);
        let (code, paths) = refusal(admit(save_request_with(&request, &model(), &bare)));
        assert_eq!((code.as_str(), paths[0].as_str()), (INPUT_MANIFEST_UNBOUND, "/input_manifest/model_basis/model_payload"));
    }

    /// Codex W-18a (b2) round 2, finding 2 (admitted before this binding): a
    /// request that keeps an old audit `manifest_id` and the result
    /// document's matching `audit_manifest_ref`, while its audited input hash,
    /// the hash's payload reference, the report's and the handoff records'
    /// references and the origin's qualification name another manifest of the
    /// same model (bound by its text), with the derivative hash recomputed, is
    /// refused: the desktop names its audit manifest by the input-manifest
    /// reference (§2.4), which the shared producer now requires.
    #[test]
    fn an_audit_manifest_id_other_than_the_input_manifest_reference_is_refused() {
        let first = input_manifest(&model(), json!([]));
        let mut second = first.clone();
        second["fixture_label"] = json!("Invented second session manifest of the same model; test fixture only");
        let old = desktop_request_for(&model(), &first);
        let mut moved = desktop_request_for(&model(), &second);
        let audit_ref = "/result_envelope/reproducibility/audit_manifest_ref";
        moved["audit_manifest"]["manifest_id"] = old["audit_manifest"]["manifest_id"].clone();
        *moved["result_documents"][0].pointer_mut(audit_ref).unwrap() = old["result_documents"][0].pointer(audit_ref).unwrap().clone();
        moved["result_documents"][0] = rehash(moved["result_documents"][0].take());
        assert_eq!(moved["audit_manifest"]["manifest_id"], old["audit_manifest"]["input_manifest_hash"]["payload_ref"]);
        assert_ne!(moved["audit_manifest"]["manifest_id"], moved["audit_manifest"]["input_manifest_hash"]["payload_ref"]);
        let error = admit(save_request_with(&moved, &model(), &second)).err().expect("refused");
        assert_eq!((error.code.as_str(), error.stage.as_str()), (open_pipe_stress_report_package::bindings::BINDING_MISMATCH, "assemble"), "{}", error.message);
        assert!(error.message.contains("desktop audit manifest id"), "{}", error.message);
        // The same edit with the old manifest's text is refused by the input-manifest binding.
        let (code, _) = refusal(admit(save_request_with(&moved, &model(), &first)));
        assert_eq!(code, INPUT_MANIFEST_UNBOUND);
    }

    // Review round 1 (the reviewer's regressions, adopted): an object-valued
    // `classification` outside a rule pack's own is malformed, in either value.
    #[test]
    fn arbitrary_request_classification_objects_are_malformed() {
        let result = admit(edited(|r| {
            r["report"]["report_sections"]["provenance_notes"][0]["classification"] =
                json!({"value": "protected_suspected"});
        }));
        let error = match result { Err(e) => e, Ok(_) => panic!("malformed request disposition admitted") };
        assert_eq!(error.code, DISPOSITION_MALFORMED);
        assert_eq!(error.diagnostics[0].path, "/request/report/report_sections/provenance_notes/0/classification");
    }

    #[test]
    fn arbitrary_model_classification_objects_are_malformed() {
        let mut m = model();
        m["nodes"][0]["classification"] = json!({"value": "protected_suspected"});
        let result = admit(save_request(&desktop_request(&m), &m));
        let error = match result { Err(e) => e, Ok(_) => panic!("malformed model disposition admitted") };
        assert_eq!(error.code, DISPOSITION_MALFORMED);
        assert_eq!(error.diagnostics[0].path, "/model/nodes/0/classification");
    }

    /// Round 2: the report package grants no `classification` exemption,
    /// whatever marker an object carries; a rule-pack-shaped classification
    /// object is malformed, clear or not.
    #[test]
    fn a_rule_pack_shaped_classification_object_is_malformed_in_a_report_package() {
        let pack = |classification: Value| move |r: &mut Value| {
            r["report"]["report_sections"]["provenance_notes"][0]["invented_pack"] =
                json!({"rule_pack_kind": "open_pipe_stress_rule_pack", "classification": classification});
        };
        let clear = json!({"privacy_class": "private", "redistribution_status": "private_only", "public_repository_policy": "quarantine_required",
            "protected_content_review_required": true, "private_values_allowed": true});
        let mut blocking = clear.clone();
        blocking["public_repository_policy"] = json!("quarantined");
        let mut nested = clear.clone();
        nested["public_repository_policy"] = json!({"value": "quarantined"});
        for classification in [clear, blocking, nested] {
            let (code, paths) = refusal(admit(edited(pack(classification))));
            assert_eq!((code.as_str(), paths[0].as_str()), (DISPOSITION_MALFORMED, "/request/report/report_sections/provenance_notes/0/invented_pack/classification"));
        }
    }
}

//! The derivation of a user-released report package request (W-18b slice
//! (r2b); DEL-08-01 Design `report-user-release.md` §2.6). Under scope R
//! native admission does not label or bound caller content: it derives the
//! whole request from the bound basis and requires canonical equality.
//!
//! The bound basis is the model `M`, the source carrier `S` (attested for `M`
//! with the solver mode `m`), the run record `R` (checked by native
//! admission, §2.5 step 8), the host's software identity `H` and the user's
//! record `U`. The input manifest `I` is derived from `M`, `S` and `m`; the
//! request's `I` must equal it. Every builder here is the Rust mirror of a
//! desktop builder, held equal by the committed vectors
//! `fixtures/reports/invented/user_release_derivation_vectors.json`:
//! - [`derive_input_manifest`]: `buildCurrentSessionInputManifest`
//!   (`inputManifestService.ts`), with the solve's fixed settings for `m`
//!   (`workspaceSession.ts`), no rule pack and no external asset;
//! - [`current_received_origin`] and [`current_result_document_base`]:
//!   `currentReceivedOrigin` and `currentResultDocumentBase`
//!   (`resultExportAdapter.ts`), then `derive_document`, unchanged;
//! - [`minimal_renderer_input`]: the desktop's user-released renderable input
//!   (`userReleasedRenderableReportInput`, `renderableReportInput.ts`), with
//!   the shared row projection ([`expected_report_result_rows`], whose value
//!   text is `quantity_display::display_quantity`), the shared unit display
//!   rule ([`QUANTITY_DISPLAY_POLICY`], [`source_displays_scaled_value`]) and
//!   the shared diagnostics projection ([`expected_report_diagnostics`]);
//! - [`derive`]: the request as `assembleUserReleasedReportPackageRequest`
//!   (`reportPackageRequest.ts`) writes it, self-checked with the producer's
//!   [`check_report_result_refs`] and [`check_unit_display_summary`].
//!
//! The record `U` is carried as given; its form and bound members are checked
//! elsewhere (§2.3, §2.5 step 4). Nothing here calls the producer, renders,
//! writes or admits anything: [`check_matches_derivation`] reports the first
//! JSON pointer at which a request differs, and the admitting entry that uses
//! it is (r2c)'s. The derivation records no user release and implies no
//! acceptance, issuance or lifecycle standing.
//! Standard claim fence applies (claims taxonomy per DEC-081).

use crate::bindings::{
    check_report_result_refs, check_unit_display_summary, expected_report_diagnostics, expected_report_result_rows,
};
use crate::admitted;
use crate::quantity_display::{source_displays_scaled_value, QUANTITY_DISPLAY_POLICY};
use crate::wire::{decode_report_package_request, LinkedSolverIdentity};
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_report_renderer::UserReleaseRecord;
use open_pipe_stress_result_export::derivative::{checksum, derive_document, digest, reference};
use open_pipe_stress_result_export::semantic_contract::{PRESSURE_ID, PRESSURE_SOURCE_ID, PREVIEW_PHYSICS_ID};
use serde_json::{json, Map, Value};
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// The user-released export profile (§2.1).
pub const USER_RELEASED_DESKTOP_PROFILE: &str = "desktop_user_released_report_package_1";
/// The user-released export context (DEL-12-02 runtime vocabulary, §3.1).
pub const USER_RELEASED_CONTEXT: &str = "user_released_report";
/// The package id prefix of a user-released package (§2.1).
pub const USER_RELEASED_PACKAGE_PREFIX: &str = "desktop-user-released-report-";
/// A request member is not its derivation (§2.5 step 9).
pub const CONTENT_NOT_DERIVED: &str = "REPORT-USER-RELEASE-CONTENT-NOT-DERIVED";
/// The source is an exact-route (`pressure-2`, `pressure-source-1`) result
/// (§0): a shareable package carries pressure-free results only.
pub const EXACT_ROUTE_NOT_ADMITTED: &str = "REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED";
/// The basis cannot be derived from (a member the derivation reads is absent
/// or not of the builders' form).
pub const BASIS_INVALID: &str = "REPORT-USER-RELEASE-BASIS-INVALID";

// The route-010 vocabulary (§2.5, §2.7, §2.8, §2.9), one definition for every
// layer that refuses with it: native admission (`src-tauri`), the producer
// and the headless runner.

/// The record is absent where route 010 needs it, or not of its form (§2.3,
/// §2.5 steps 3-4).
pub const RECORD_INVALID: &str = "REPORT-USER-RELEASE-RECORD-INVALID";
/// A bound record member differs from the value native admission computed or
/// bound (§2.5 step 4).
pub const UNBOUND: &str = "REPORT-USER-RELEASE-UNBOUND";
/// A user-release record on the local-private route 009 (§2.5 step 3).
pub const NOT_THIS_ROUTE: &str = "REPORT-USER-RELEASE-NOT-THIS-ROUTE";
/// The model carries a detectable library reference (§2.7).
pub const LIBRARY_REFERENCED: &str = "REPORT-USER-RELEASE-LIBRARY-REFERENCED";
/// The model or its input manifest carries a rule-pack reference (§2.7).
pub const RULE_PACK_REFERENCED: &str = "REPORT-USER-RELEASE-RULE-PACK-REFERENCED";
/// An object carries a non-project private class (§2.7).
pub const PRIVATE_CLASS: &str = "REPORT-USER-RELEASE-PRIVATE-CLASS";
/// This session's solver holds no attested solve of the model and result
/// (§2.4, §2.5 step 7).
pub const RESULT_NOT_ATTESTED: &str = "REPORT-USER-RELEASE-RESULT-NOT-ATTESTED";
/// The run record fails a Record Run binding check or one of route 010's
/// (§2.5 step 8).
pub const ANALYSIS_RECORD_UNBOUND: &str = "REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND";
/// The headless runner's refusal of every user-release marker (§2.8,
/// refusal 1).
pub const RUNNER_NOT_ADMITTED: &str = "REPORT-USER-RELEASE-RUNNER-NOT-ADMITTED";
/// The producer's general entries refuse every user-release marker (§2.8,
/// refusal 2).
pub const PACKAGE_NOT_ADMITTED: &str = "REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED";
/// Under the admitting entry, the request is not a user-released request, or
/// its record differs from the request's members (§2.8 items 8 and 10, the
/// handoff rule).
pub const PACKAGE_UNBOUND: &str = "REPORT-PACKAGE-USER-RELEASE-UNBOUND";

/// The three user-release markers (§2.8), each by its fixed path, in order.
pub const MARKER_PATHS: [&str; 3] = ["export_profile_id", "report.report_context.export_context", "report.report_context.user_release"];

/// The first user-release marker a request carries (§2.8): its
/// `export_profile_id` is the user-released profile; its
/// `report.report_context.export_context` is `user_released_report`; or its
/// `report.report_context` has a `user_release` member, whatever its value.
/// Returns the marker's fixed path from [`MARKER_PATHS`], never a value. The
/// report is read by key; a report in positional form is refused by the
/// producer's object-form check before any keyed check.
pub fn user_release_marker(export_profile_id: &str, report: &Value) -> Option<&'static str> {
    let context = report.get("report_context");
    if export_profile_id == USER_RELEASED_DESKTOP_PROFILE {
        Some(MARKER_PATHS[0])
    } else if context.and_then(|context| context.get("export_context")).and_then(Value::as_str) == Some(USER_RELEASED_CONTEXT) {
        Some(MARKER_PATHS[1])
    } else if context.and_then(Value::as_object).is_some_and(|context| context.contains_key("user_release")) {
        Some(MARKER_PATHS[2])
    } else {
        None
    }
}

/// The general entries' refusal of a marker (§2.8, refusal 2).
pub(crate) fn general_entry_refusal(marker: &str) -> String {
    format!(
        "{PACKAGE_NOT_ADMITTED}: this producer entry packages no user-released report; only the desktop's route DREP-PACKAGE-USER-RELEASE-010 builds one, through user_release::assemble_derived (marker: {marker})"
    )
}

/// The user-release marker of a typed container input: its profile, its
/// report context, or a record in that context.
pub(crate) fn container_marker(export_profile_id: &str, report: &open_pipe_stress_report_renderer::RenderableReportInput) -> Option<&'static str> {
    use open_pipe_stress_report_renderer::ReportExportContext;
    let context = report.report_context.as_ref();
    if export_profile_id == USER_RELEASED_DESKTOP_PROFILE {
        Some(MARKER_PATHS[0])
    } else if context.is_some_and(|context| context.export_context == ReportExportContext::UserReleasedReport) {
        Some(MARKER_PATHS[1])
    } else if context.is_some_and(|context| context.user_release.is_some()) {
        Some(MARKER_PATHS[2])
    } else {
        None
    }
}

/// Proof that the whole-request derivation held (§2.8). Its field is private
/// and it derives neither `Clone`, `Copy` nor `Default`: only
/// [`check_derived`] constructs one, and only [`assemble_derived`] calls that
/// (`user_release_reach_scan`).
pub(crate) struct DerivedUserRelease {
    _derived: (),
}

/// Which producer entry a request came through (§2.8). The general entries
/// pass [`ProducerEntry::General`], which refuses every user-release marker;
/// only [`assemble_derived`] passes [`ProducerEntry::UserRelease`]. It is not
/// an admitted set and changes no item-0 decision.
pub(crate) enum ProducerEntry {
    General,
    UserRelease(DerivedUserRelease),
}

impl ProducerEntry {
    pub(crate) fn is_user_release(&self) -> bool {
        matches!(self, ProducerEntry::UserRelease(_))
    }
}

/// `CURRENT_ORIGIN_LIMIT` (`resultExportAdapter.ts`).
pub const CURRENT_ORIGIN_LIMIT: &str = "Qualified Current received carrier; independent authentic original producer bytes unavailable; dimension absence is not producer attestation";
/// The solve's fixed nonlinear iteration policy (`workspaceSession.ts`).
pub const NONLINEAR_ITERATION_POLICY: &str = "DEC-046-CV-B-product-preview-active-set-count-v1";
/// The solver modes a desktop solve records.
pub const SOLVER_MODES: [&str; 2] = ["sparse_interactive", "dense_scrutiny"];
/// The desktop's software name when its configuration records none
/// (`renderableReportInput.ts` `buildReportContext`).
pub const DEFAULT_SOFTWARE_NAME: &str = "SWBPIPE desktop";
const RESULT_EXPORT_VERSION: &str = "0.3.0";
const DOCUMENT_DIAGNOSTIC_REMEDIATION: &str = "Review the carried solve diagnostic before relying on this exported result envelope.";
const SECTION_KINDS: [&str; 8] = [
    "model_input_summary",
    "load_cases",
    "results",
    "warnings_assumptions_provenance",
    "audit_manifest",
    "rule_pack_references",
    "limitations",
    "professional_boundary_notice",
];
const KNOWN_ANALYSIS_STATUSES: [&str; 6] =
    ["MODEL_INCOMPLETE", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED", "HUMAN_REVIEW_REQUIRED"];
const RULE_TOKENS: [&str; 3] = ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"];
const HUMAN_REVIEW_REQUIRED: &str = "HUMAN_REVIEW_REQUIRED";

/// The host's own software identity `H` (`app.package_info()`, the
/// configuration's `productName` and `version`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SoftwareIdentity {
    pub name: String,
    pub version: String,
}

/// The bound basis of §2.6. Each member is a validation input only; none is
/// written to the package except as the derivation writes it.
pub struct UserReleaseBasis<'a> {
    /// `M`, the model text's value.
    pub model: &'a Value,
    /// `S`, the source carrier (`/source_results/0`).
    pub source: &'a Value,
    /// `m`, the attested solver mode.
    pub solver_mode: &'a str,
    /// `R`, the solve's transient DEL-14-02 0.3.0 run record.
    pub run_record: &'a Value,
    /// `H`.
    pub software: &'a SoftwareIdentity,
    /// `U`, the record, as the request's `report.report_context.user_release`.
    pub record: &'a UserReleaseRecord,
}

/// `E(M, S, m, I, R, H, U)`: the derived input manifest, its identity, and
/// the derived request.
#[derive(Clone, Debug, PartialEq)]
pub struct UserReleaseDerivation {
    pub input_manifest: Value,
    /// SHA-256 of the input manifest's RFC 8785 bytes.
    pub input_manifest_sha256: String,
    /// `input-manifest:<safe model ref>:<sha256>`.
    pub input_manifest_ref: String,
    pub request: Value,
}

fn invalid(detail: impl std::fmt::Display) -> String {
    format!("{BASIS_INVALID}: {detail}")
}

fn text<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, String> {
    value.pointer(pointer).and_then(Value::as_str).ok_or_else(|| invalid(format_args!("{pointer} is not text")))
}

fn array<'a>(value: &'a Value, pointer: &str) -> Result<&'a Vec<Value>, String> {
    value.pointer(pointer).and_then(Value::as_array).ok_or_else(|| invalid(format_args!("{pointer} is not a list")))
}

/// The desktop's `text()`: a string's trimmed form, `None` when it is not a
/// string or is blank after JavaScript's `trim`.
fn trimmed(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_str).map(js_trim).filter(|text| !text.is_empty())
}

/// JavaScript's `String.prototype.trim`: its WhiteSpace (tab, vertical tab,
/// form feed, U+FEFF and the Space_Separator characters) and LineTerminator
/// (line feed, carriage return, U+2028, U+2029) code points. Unlike Rust's
/// `trim`, it keeps U+0085.
fn js_trim(text: &str) -> &str {
    text.trim_matches(|c: char| {
        matches!(
            c,
            '\t' | '\u{b}' | '\u{c}' | ' ' | '\u{a0}' | '\u{feff}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{202f}' | '\u{205f}' | '\u{3000}'
                | '\n' | '\r' | '\u{2028}' | '\u{2029}'
        )
    })
}

fn optional_text(value: Option<&Value>) -> Value {
    trimmed(value).map_or(Value::Null, |text| json!(text))
}

/// JavaScript's default `Array.prototype.sort` order: UTF-16 code units.
fn utf16_order(left: &str, right: &str) -> Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}

fn sorted_unique(items: impl Iterator<Item = String>) -> Vec<String> {
    let mut unique: Vec<String> = items.collect::<BTreeSet<_>>().into_iter().collect();
    unique.sort_by(|left, right| utf16_order(left, right));
    unique
}

/// `safeToken` / `safeId` (`inputManifestService.ts`, `reportPackageRequest.ts`):
/// each run of characters outside `[A-Za-z0-9._-]` becomes one `-`, leading
/// and trailing `-` are removed, and an empty result is `fallback`.
pub fn safe_token(value: &str, fallback: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut in_run = false;
    for c in value.chars() {
        if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') {
            out.push(c);
            in_run = false;
        } else if !in_run {
            out.push('-');
            in_run = true;
        }
    }
    let trimmed = out.trim_matches('-');
    if trimmed.is_empty() { fallback.to_string() } else { trimmed.to_string() }
}

/// The input manifest's reference, as `inputManifestRef` builds it.
pub fn input_manifest_ref(model_ref: &str, manifest_sha256: &str) -> String {
    format!("input-manifest:{}:{manifest_sha256}", safe_token(model_ref, "current-session"))
}

fn boundary() -> Value {
    json!({
        "human_review_required": true,
        "software_makes_compliance_claim": false,
        "software_makes_certification_claim": false,
        "software_makes_sealing_claim": false,
        "software_makes_approval_claim": false,
        "software_makes_authentication_claim": false
    })
}

/// The session provenance, the only provenance record of the minimal input
/// (`sessionProvenance`, `renderableReportInput.ts`).
pub fn session_provenance(model: &Value) -> Result<Value, String> {
    let name = text(model, "/project/name")?;
    Ok(json!({
        "source_name": format!("OpenPipeStress desktop session ({name})"),
        "source_location": "local desktop session",
        "source_license": "user_supplied_or_private",
        "contributor": "user_local_session",
        "contributor_certification": "not_asserted",
        "redistribution_status": "private_only",
        "review_status": "pending",
        "privacy_classification": "private_project_data"
    }))
}

/// The input manifest `buildCurrentSessionInputManifest` builds for a desktop
/// solve of `model` in `solver_mode` whose producer is `source`'s: the solve's
/// fixed settings for the mode (`workspaceSession.ts`), `active_rule_packs: []`
/// and `external_assets: []` (§2.6; every model with a rule-pack reference is
/// refused before the derivation, §2.7).
pub fn derive_input_manifest(model: &Value, source: &Value, solver_mode: &str) -> Result<Value, String> {
    if !SOLVER_MODES.contains(&solver_mode) {
        return Err(invalid(format_args!("the solver mode {solver_mode:?} is not a desktop solve's")));
    }
    let model_ref = text(model, "/project/id")?;
    if js_trim(model_ref).is_empty() {
        return Err(invalid("the model has no project id"));
    }
    let units = model.pointer("/project/units").filter(|units| units.is_object()).ok_or_else(|| invalid("/project/units is not an object"))?;
    let load_cases = array(model, "/load_cases")?;
    let combinations = match model.get("combinations") {
        None | Some(Value::Null) => json!([]),
        Some(Value::Array(items)) => Value::Array(items.clone()),
        Some(_) => return Err(invalid("/combinations is not a list")),
    };
    let (name, version) = (text(source, "/producer/component_name")?, text(source, "/producer/component_version")?);
    Ok(json!({
        "schema_version": "1.0.0",
        "document_kind": "openpipestress.current_session_input_manifest",
        "model_basis": { "model_ref": model_ref, "model_payload": model },
        "unit_basis": { "project_units": units },
        "solver_basis": {
            "solver_name": name,
            "solver_version": version,
            "solver_build_ref": format!("{name}@{version}"),
            "solver_mode": solver_mode,
            "settings": {
                "nonlinear_iteration_policy": NONLINEAR_ITERATION_POLICY,
                "sparse_evidence_lane": solver_mode == "sparse_interactive"
            }
        },
        "load_basis": { "load_cases": load_cases, "combinations": combinations },
        "active_rule_packs": [],
        "external_assets": [],
        "identity_policy": {
            "canonicalization": "rfc8785_jcs",
            "hash_algorithm": "sha256",
            "canonical_bytes_scope": "entire_input_manifest_object"
        },
        "replay_boundary": {
            "included_as_package_member": false,
            "portable_replay_claimed": false,
            "current_session_ref_hash_integrity_only": true
        }
    }))
}

/// The received-carrier origin `currentReceivedOrigin` binds
/// (`resultExportAdapter.ts`), with [`CURRENT_ORIGIN_LIMIT`].
pub fn current_received_origin(model: &Value, source: &Value, manifest_ref: &str) -> Result<Value, String> {
    let run = text(source, "/run_id")?;
    Ok(json!({
        "origin_id": "source-origin:current-received",
        "origin_class": "received_current_dimension_absent",
        "qualification_ref": reference("current_manifest", manifest_ref),
        "authentic_producer_available": false,
        "received_carrier_checksum": checksum(source, "received_current_dimension_absent_carrier", reference("received_current_carrier", run))?,
        "original_producer_checksum": null,
        "origin_limit": CURRENT_ORIGIN_LIMIT,
        "actual_model_ref": reference("model_payload", text(model, "/project/id")?),
        "mechanics_run_ref": reference("mechanics_run", run),
        "request_model_ref": null,
        "request_run_ref": null,
        "request_alias_disclosure": null
    }))
}

fn derivative_provenance() -> Value {
    json!({
        "source_name": "local qualified result derivative",
        "source_location": "apps/desktop/src/features/result-export/resultExportAdapter.ts",
        "source_license": "project-local",
        "contributor": "SWBPIPE",
        "contributor_certification": "local desktop session; human review required",
        "redistribution_status": "private_only",
        "review_status": "pending"
    })
}

/// Rust's `export_diagnostic_class`, as `diagnosticClass` mirrors it.
fn diagnostic_class(code: &str, severity: &str) -> &'static str {
    if code.contains("RULE") {
        return if severity == "blocking" { "RULE_CHECK_BLOCKING" } else { "ASSUMPTION_WARNING" };
    }
    if code.contains("UNIT") {
        return "UNIT_WARNING";
    }
    if code.contains("PROVENANCE") {
        return "PROVENANCE_WARNING";
    }
    if severity != "info" && severity != "warning" {
        return "SOLVE_BLOCKING";
    }
    if code.starts_with("NONLINEAR") {
        return "NONLINEAR_WARNING";
    }
    "ASSUMPTION_WARNING"
}

/// One source diagnostic in the document (`documentDiagnostic`).
fn document_diagnostic(diagnostic: &Value) -> Result<Value, String> {
    let code = diagnostic["code"].as_str().ok_or_else(|| invalid("a source diagnostic has no code"))?;
    let id = diagnostic["id"].as_str().filter(|id| !id.is_empty()).ok_or_else(|| format!("RESULT_EXPORT_DIAGNOSTIC_ID_MISSING: {code}"))?;
    let severity = diagnostic["severity"].as_str().ok_or_else(|| invalid(format_args!("source diagnostic {id} has no severity")))?;
    let source = match &diagnostic["source"] {
        Value::Null => "core/product_physics",
        Value::String(source) => source.as_str(),
        _ => return Err(invalid(format_args!("source diagnostic {id} has an unreadable source"))),
    };
    Ok(json!({
        "code": code,
        "class": diagnostic_class(code, severity),
        "severity": if severity == "info" || severity == "warning" { severity } else { "blocking" },
        "source": reference("solver_component", source),
        "affected_object": reference("preview_diagnostic", id),
        "message": diagnostic["message"],
        "remediation": DOCUMENT_DIAGNOSTIC_REMEDIATION,
        "provenance": derivative_provenance()
    }))
}

fn mapped_object_ref(item: &Value, what: &str) -> Result<Value, String> {
    match (item["object_type"].as_str(), item["ref"].as_str()) {
        (Some(kind), Some(id)) => Ok(reference(kind, id)),
        _ => Err(invalid(format_args!("{what} is not an object reference"))),
    }
}

/// The document base `currentResultDocumentBase` builds
/// (`resultExportAdapter.ts`) from the run record's `analysis_run` and the
/// input manifest's reference and solver basis. Its diagnostics are the
/// source's: a run record with any further diagnostic (a rule-pack reference
/// exclusion) is outside scope R (§2.7) and is refused here.
pub fn current_result_document_base(model: &Value, source: &Value, run_record: &Value, manifest_ref: &str, solver_basis: &Value) -> Result<Value, String> {
    let run = run_record.get("analysis_run").ok_or_else(|| invalid("the run record has no analysis_run"))?;
    let (model_id, source_run, run_id) = (text(model, "/project/id")?, text(source, "/run_id")?, text(run, "/run_id")?);
    let source_diagnostics = array(source, "/diagnostics")?;
    if array(run, "/diagnostics")?.len() != source_diagnostics.len() {
        return Err(invalid("the run record carries diagnostics the source does not (a rule-pack reference exclusion is outside scope R)"));
    }
    let diagnostics = source_diagnostics.iter().map(document_diagnostic).collect::<Result<Vec<_>, _>>()?;
    let load_basis_refs = array(run, "/load_basis_refs")?
        .iter()
        .map(|item| mapped_object_ref(item, "a load basis reference"))
        .collect::<Result<Vec<_>, _>>()?;
    let run_hashes = array(run, "/hashes")?
        .iter()
        .map(|hash| {
            Ok(json!({
                "algorithm": hash["algorithm"],
                "canonicalization": hash["canonicalization"],
                "payload_ref": mapped_object_ref(&hash["payload_ref"], "a run hash's payload reference")?,
                "value": hash["value"]
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(json!({
        "schema_version": RESULT_EXPORT_VERSION,
        "deliverable_id": "DEL-08-04",
        "package_id": "PKG-08",
        "scope_item": "SOW-046",
        "objectives": ["OBJ-007", "OBJ-009"],
        "export_format_status": {
            "baseline_format": "schema_first_json_result_envelope",
            "additional_formats": "TBD",
            "public_transport_protocol": "TBD",
            "local_fea_package_format": "TBD",
            "external_adapter_formats": "TBD"
        },
        "result_envelope": {
            "schema_version": RESULT_EXPORT_VERSION,
            "envelope_id": format!("result-envelope:{source_run}"),
            "model_ref": reference("model_payload", model_id),
            "run_ref": reference("analysis_run", run_id),
            "solver_version": {
                "solver_name": solver_basis["solver_name"],
                "solver_version": solver_basis["solver_version"],
                "solver_build_ref": solver_basis["solver_build_ref"]
            },
            "unit_system_ref": reference("unit_system", &format!("{model_id}:units")),
            "load_basis_refs": load_basis_refs,
            "result_sets": [{
                "set_id": format!("result-set:{source_run}:mechanics"),
                "set_type": "mechanics",
                "basis_ref": reference("analysis_run", run_id),
                "values": []
            }],
            "diagnostics": diagnostics,
            "provenance": derivative_provenance(),
            "reproducibility": {
                "model_hash": null,
                "run_hashes": run_hashes,
                "audit_manifest_ref": reference("audit_manifest", manifest_ref),
                "deterministic_ordering": true
            },
            "analysis_status": run["analysis_status"],
            "professional_boundary": run["professional_boundary"],
            "downstream_use": {
                "review": true,
                "regression_comparison": true,
                "report_consumption": true,
                "headless_automation": true,
                "governed_downstream_tooling": true,
                "additional_export_formats": "TBD"
            }
        }
    }))
}

/// The run record's checksum entry of `scope`, exactly one.
fn run_hash_value<'a>(run: &'a Value, scope: &str) -> Result<&'a str, String> {
    let mut found = array(run, "/hashes")?.iter().filter(|hash| hash["payload_scope"] == scope);
    match (found.next(), found.next()) {
        (Some(hash), None) => hash["value"].as_str().ok_or_else(|| invalid(format_args!("the run record's {scope} checksum has no value"))),
        _ => Err(invalid(format_args!("the run record has no single {scope} checksum"))),
    }
}

/// The report's sorted analysis statuses: the record's known statuses and
/// `HUMAN_REVIEW_REQUIRED`, each once.
fn report_analysis_status(run: &Value) -> Result<Vec<String>, String> {
    let recorded = array(run, "/analysis_status")?
        .iter()
        .map(|status| status.as_str().map(str::to_string).ok_or_else(|| invalid("an analysis status is not text")))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(sorted_unique(
        recorded.into_iter().chain([HUMAN_REVIEW_REQUIRED.to_string()]).filter(|status| KNOWN_ANALYSIS_STATUSES.contains(&status.as_str())),
    ))
}

fn report_checksum(payload_ref: &Value, value: Option<&str>) -> Value {
    json!({
        "algorithm": "sha256",
        "canonicalization": "rfc8785_jcs",
        "payload_ref": payload_ref,
        "value": value.filter(|value| !value.is_empty()).unwrap_or("TBD")
    })
}

fn report_envelope(payload_ref: Value, schema_id: &str, value: Option<&str>, provenance: &Value) -> Value {
    json!({
        "ref": payload_ref,
        "schema_ref": reference("schema", schema_id),
        "checksum": report_checksum(&payload_ref, value),
        "privacy_classification": "private_project_data",
        "provenance": provenance
    })
}

/// The report context `buildReportContext` writes, under
/// `user_released_report` with `generated_at` null, `H`'s software identity
/// and the record `U` (§2.6). The solver mode and settings are the bound input
/// manifest's.
fn report_context(source: &Value, run: &Value, input_manifest: &Value, software: &SoftwareIdentity, record: &UserReleaseRecord) -> Result<Value, String> {
    let solver_basis = input_manifest.get("solver_basis").ok_or_else(|| invalid("the input manifest has no solver basis"))?;
    let settings = solver_basis.get("settings").and_then(Value::as_object).ok_or_else(|| invalid("the input manifest's settings are not an object"))?;
    let mut keys: Vec<&String> = settings.keys().collect();
    keys.sort_by(|left, right| utf16_order(left, right));
    let solver_settings: Vec<Value> = keys.into_iter().map(|key| json!({"key": key, "value": canonical_json(&settings[key])})).collect();
    let coordinate_systems = sorted_unique(
        array(source, "/results")?
            .iter()
            .filter_map(|row| row.pointer("/metadata/coordinate_system").and_then(Value::as_str))
            .filter(|system| !js_trim(system).is_empty() && *system != "TBD")
            .map(str::to_string),
    );
    let name = js_trim(&software.name);
    Ok(json!({
        "export_context": USER_RELEASED_CONTEXT,
        "generated_at": null,
        "software_name": if name.is_empty() { DEFAULT_SOFTWARE_NAME } else { name },
        "software_version": optional_text(Some(&json!(software.version))),
        "build_release_identifier": null,
        "solver_name": optional_text(run.pointer("/solver_version/solver_name")),
        "solver_version": optional_text(run.pointer("/solver_version/solver_version")),
        "solver_build_ref": optional_text(run.pointer("/solver_version/build_ref/ref")),
        "analysis_run_name": optional_text(run.get("run_name")),
        "analysis_run_recorded_at": optional_text(run.get("created_at")),
        "solver_mode": optional_text(solver_basis.get("solver_mode")),
        "solver_settings_ref": optional_text(run.pointer("/settings_ref/ref")),
        "solver_settings": solver_settings,
        "model_coordinate_system": null,
        "result_coordinate_systems": coordinate_systems,
        "user_release": serde_json::to_value(record).map_err(|error| invalid(format_args!("the record does not serialize ({error})")))?
    }))
}

/// The minimal native renderer input (§2.6): the title, the model summary,
/// the load cases, the solve's result rows and diagnostics, the status
/// disclosures, the fixed technical-preview limitation, the run context and
/// the record. It carries no rule-pack reference, user-rule outcome,
/// component, spring-hanger, material, section or support provenance entry,
/// per-object hanger quantity, numerical-health row, library reference or
/// provenance diagnostic, and its persistence reference is `TBD`.
pub fn minimal_renderer_input(
    model: &Value,
    source: &Value,
    run_record: &Value,
    input_manifest: &Value,
    software: &SoftwareIdentity,
    record: &UserReleaseRecord,
) -> Result<Value, String> {
    let run = run_record.get("analysis_run").ok_or_else(|| invalid("the run record has no analysis_run"))?;
    let (model_id, name) = (text(model, "/project/id")?, text(model, "/project/name")?);
    let (source_run, source_model) = (text(source, "/run_id")?, text(source, "/model_ref")?);
    let provenance = session_provenance(model)?;
    let report_ref = format!("report:{source_run}");
    let statuses = report_analysis_status(run)?;
    let unresolved: Vec<&Value> = array(run, "/reproducibility/unresolved_tbd")?.iter().collect();

    let diagnostics = expected_report_diagnostics(source)?
        .into_iter()
        .map(|mut entry| {
            entry["provenance"] = provenance.clone();
            entry
        })
        .collect::<Vec<_>>();
    let mut disclosures: Vec<Value> = statuses
        .iter()
        .filter(|status| status.as_str() != HUMAN_REVIEW_REQUIRED)
        .map(|status| {
            json!({
                "status": status,
                "source": reference("analysis_run", source_run),
                "affected_object": reference("model", source_model),
                "explanation": format!("Automatic analysis status reported by the run record: {status}."),
                "human_review_required": true,
                "human_acceptance_ref": null
            })
        })
        .collect();
    disclosures.push(json!({
        "status": HUMAN_REVIEW_REQUIRED,
        "source": reference("report_renderer", &report_ref),
        "affected_object": reference("report", &report_ref),
        "explanation": "Human professional review is required before any reliance on this report.",
        "human_review_required": true,
        "human_acceptance_ref": null
    }));
    let report_sections = json!({
        "report_section_id": format!("sections:{source_run}"),
        "model_ref": reference("model", source_model),
        "run_ref": reference("analysis_run", source_run),
        "diagnostics": diagnostics,
        "analysis_status_disclosures": disclosures,
        "provenance_notes": [provenance],
        "user_supplied_values": [],
        "assumptions": [],
        "limitations": [{
            "limitation_id": format!("limitation:{source_run}:technical-preview"),
            "source": reference("report_renderer", &report_ref),
            "affected_scope": reference("model", source_model),
            "statement": "Output over invented or user-local data; not validated engineering output.",
            "effect": {
                "mechanics_solve_qualified": true,
                "user_rule_check_qualified": false,
                "report_completeness": "qualified",
                "human_review_required": true
            },
            "provenance": provenance
        }],
        "unresolved_tbds": unresolved.iter().enumerate().map(|(index, description)| json!({
            "tbd_id": format!("tbd:{source_run}:{}", index + 1),
            "affected_scope": reference("analysis_run", source_run),
            "description": description,
            "review_needed": true
        })).collect::<Vec<_>>(),
        "professional_boundary": boundary()
    });
    let sections_checksum = digest(&report_sections)?;

    let units = model.pointer("/project/units").filter(|units| units.is_object()).ok_or_else(|| invalid("/project/units is not an object"))?;
    let result_units = sorted_unique(
        array(source, "/results")?.iter().filter_map(|row| row["unit"].as_str()).filter(|unit| !unit.is_empty()).map(str::to_string),
    );
    let input_manifest_ref = run
        .pointer("/reproducibility/input_manifest_refs/0")
        .map(|item| mapped_object_ref(item, "the run record's input-manifest reference"))
        .transpose()?
        .unwrap_or_else(|| reference("audit_manifest", "TBD"));
    let load_case_summary = array(model, "/load_cases")?
        .iter()
        .map(|case| {
            let mut entry = Map::new();
            entry.insert("load_ref".into(), reference("load_case", case["id"].as_str().ok_or_else(|| invalid("a load case has no id"))?));
            for (from, to) in [("label", "label"), ("kind", "basis")] {
                if let Some(value) = case.get(from) {
                    entry.insert(to.into(), value.clone());
                }
            }
            entry.insert("source".into(), reference("model", model_id));
            entry.insert("provenance".into(), provenance.clone());
            Ok(Value::Object(entry))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let result_export_refs = array(run, "/result_refs")?
        .iter()
        .map(|item| {
            let payload_ref = mapped_object_ref(&item["result_ref"], "a run record result reference")?;
            Ok(report_envelope(payload_ref, "schemas/results.schema.yaml", item.pointer("/hash_refs/0/value").and_then(Value::as_str), &provenance))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let unresolved_runtime: Vec<Value> = unresolved
        .iter()
        .enumerate()
        .map(|(index, description)| {
            json!({
                "tbd_id": format!("tbd:{source_run}:{}", index + 1),
                "topic": "analysis run reproducibility",
                "affected_scope": reference("analysis_run", source_run),
                "description": description,
                "review_needed": true
            })
        })
        .collect();
    let calculation_report = json!({
        "report_id": report_ref,
        "model_input_summary": {
            "project_ref": reference("project", model_id),
            "model_ref": reference("model", source_model),
            "persistence_ref": reference("project_persistence", "TBD"),
            "unit_system_ref": reference("unit_system", "unit-system:dec-018-si-dual-display"),
            "unit_display_summary": {
                "storage_convention": "entered_units_preserved",
                "model_units": units,
                "result_units": result_units,
                "quantity_display_policy": QUANTITY_DISPLAY_POLICY,
                "conversion_performed": source_displays_scaled_value(source)
            },
            "model_hash": report_checksum(&reference("model", model_id), Some(&digest(model)?)),
            "input_manifest_ref": input_manifest_ref,
            "provenance": provenance
        },
        "load_case_summary": load_case_summary,
        "result_export_refs": result_export_refs,
        "audit_manifest_refs": [
            report_envelope(reference("analysis_run", source_run), "schemas/analysis_run.schema.yaml", Some(run_hash_value(run, "analysis_run_record")?), &provenance),
            report_envelope(reference("result_envelope", &format!("result-envelope:{source_run}")), "schemas/results.schema.yaml", Some(run_hash_value(run, "received_result")?), &provenance)
        ],
        "report_section_refs": [
            report_envelope(reference("report_sections", &format!("sections:{source_run}")), "schemas/report_sections.schema.yaml", Some(&sections_checksum), &provenance)
        ],
        "rule_pack_refs": [],
        "diagnostics": [],
        "template_slots": SECTION_KINDS.iter().enumerate().map(|(index, kind)| json!({
            "slot_id": format!("slot-{kind}"),
            "required": true,
            "section_kind": kind,
            "source_contract": reference("contract", "DEL-08-01"),
            "ordering_index": index + 1
        })).collect::<Vec<_>>(),
        "rendered_sections": SECTION_KINDS.iter().map(|kind| json!({
            "section_id": format!("section-{kind}"),
            "slot_id": format!("slot-{kind}"),
            "section_kind": kind,
            "title": kind.replace('_', " "),
            "source_refs": [reference("model", model_id)],
            "content_status": "rendered"
        })).collect::<Vec<_>>(),
        "analysis_status": statuses,
        "professional_boundary": boundary(),
        "provenance": provenance,
        "privacy_classification": "private_project_data",
        "unresolved_runtime_tbds": unresolved_runtime
    });
    Ok(json!({
        "report_title": format!("{name} — Calculation Report"),
        "calculation_report": calculation_report,
        "report_sections": report_sections,
        "result_rows": expected_report_result_rows(source)?,
        "report_context": report_context(source, run, input_manifest, software, record)?
    }))
}

/// The source's identity must be pressure-free (§0, §2.1).
fn check_pressure_free(source: &Value) -> Result<(), String> {
    match source.pointer("/producer/semantic_contract_id").and_then(Value::as_str) {
        Some(PREVIEW_PHYSICS_ID) => Ok(()),
        Some(id @ (PRESSURE_ID | PRESSURE_SOURCE_ID)) => Err(format!(
            "{EXACT_ROUTE_NOT_ADMITTED}: a shareable report package carries pressure-free (preview-physics-2) results only; this exact-route result ({id}) can be saved in a local private package."
        )),
        other => Err(invalid(format_args!("the source's identity {} is not preview-physics-2", other.unwrap_or("(none)")))),
    }
}

/// `E(M, S, m, I, R, H, U)` (§2.6): the derived input manifest and the whole
/// derived request. The run record's rule status must be the solve's own
/// (no rule-check aggregate, §2.5 step 8), and the run record must name the
/// derived input manifest. The derived report is checked with the producer's
/// own row-checksum and unit-summary checks, so a source whose rows are not
/// the run record's is refused here as by the producer.
pub fn derive(basis: &UserReleaseBasis<'_>) -> Result<UserReleaseDerivation, String> {
    let UserReleaseBasis { model, source, solver_mode, run_record, software, record } = *basis;
    check_pressure_free(source)?;
    let run = run_record.get("analysis_run").ok_or_else(|| invalid("the run record has no analysis_run"))?;
    let model_id = text(model, "/project/id")?;
    if text(source, "/model_ref")? != model_id {
        return Err(invalid("the source names another model than the model's project.id"));
    }
    let input_manifest = derive_input_manifest(model, source, solver_mode)?;
    let input_manifest_sha256 = digest(&input_manifest)?;
    let manifest_ref = input_manifest_ref(model_id, &input_manifest_sha256);
    // The run record names this manifest (`boundInputManifest`): one reference and one hash.
    let refs = array(run, "/reproducibility/input_manifest_refs")?;
    let hashes = array(run, "/reproducibility/input_manifest_hashes")?;
    if refs.len() != 1 || hashes.len() != 1 || refs[0]["ref"] != manifest_ref.as_str() || hashes[0]["value"] != input_manifest_sha256.as_str() {
        return Err(invalid("the run record does not name the derived input manifest"));
    }
    let rule_check = text(source, "/status/rule_check")?;
    let recorded_tokens: Vec<&str> =
        array(run, "/analysis_status")?.iter().filter_map(Value::as_str).filter(|status| RULE_TOKENS.contains(status)).collect();
    if recorded_tokens != [rule_check] {
        return Err(invalid(format_args!(
            "the run record's rule status ({}) is not the solve's ({rule_check})",
            if recorded_tokens.is_empty() { "none".to_string() } else { recorded_tokens.join(", ") }
        )));
    }

    let origin = current_received_origin(model, source, &manifest_ref)?;
    let base = current_result_document_base(model, source, run_record, &manifest_ref, &input_manifest["solver_basis"])?;
    let document = derive_document(base, model, source, origin, None)?;
    let report = minimal_renderer_input(model, source, run_record, &input_manifest, software, record)?;
    check_report_result_refs(&report, source)?;
    check_unit_display_summary(&report, source)?;

    let envelope = &document["result_envelope"];
    let model_hash = digest(model)?;
    if envelope.pointer("/reproducibility/model_hash/value").and_then(Value::as_str) != Some(model_hash.as_str()) {
        return Err(invalid("the derived document's model hash is not the model's"));
    }
    let mut source_basis_refs = vec![reference("analysis_run", text(run, "/run_id")?)];
    for item in array(&document, "/result_envelope/load_basis_refs")? {
        source_basis_refs.push(reference(text(item, "/ref_type")?, text(item, "/ref_id")?));
    }
    let request = json!({
        "package_id": format!("{USER_RELEASED_PACKAGE_PREFIX}{}", safe_token(text(source, "/run_id")?, "report-package")),
        "export_profile_id": USER_RELEASED_DESKTOP_PROFILE,
        "source_model_ref": reference("model", model_id),
        "source_basis_refs": source_basis_refs,
        "report": report,
        "audit_manifest": {
            "manifest_id": manifest_ref,
            "model_hash": {
                "algorithm": "sha256",
                "canonicalization": "rfc8785_jcs",
                "payload_kind": "model_json",
                "payload_ref": model_id,
                "value": model_hash
            },
            "input_manifest_hash": {
                "algorithm": "sha256",
                "canonicalization": "rfc8785_jcs",
                "payload_kind": "input_manifest_json",
                "payload_ref": manifest_ref,
                "value": input_manifest_sha256
            },
            "solver_version": {
                "solver_name": envelope["solver_version"]["solver_name"],
                "solver_version": envelope["solver_version"]["solver_version"],
                "solver_build_ref": envelope["solver_version"]["solver_build_ref"]
            },
            "unit_system_ref": envelope["unit_system_ref"]["ref_id"],
            "rule_pack_refs": [],
            "assets": [],
            "professional_boundary": boundary()
        },
        "result_documents": [document],
        "source_results": [source],
        "state_comparison_handoff_records": [],
        "rule_check_aggregate": null,
        "solve_rule_check_status": rule_check
    });
    Ok(UserReleaseDerivation { input_manifest, input_manifest_sha256, input_manifest_ref: manifest_ref, request })
}

fn pointer_token(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}

/// Two numbers are canonically equal when RFC 8785 writes them alike, that is
/// when their binary64 values are equal (`5` and `5.0`, `0` and `-0`).
fn numbers_equal(left: &serde_json::Number, right: &serde_json::Number) -> bool {
    match (left.as_f64(), right.as_f64()) {
        (Some(left), Some(right)) => left == right,
        _ => left == right,
    }
}

fn difference_at(expected: &Value, found: &Value, pointer: &mut String) -> bool {
    match (expected, found) {
        (Value::Object(expected), Value::Object(found)) => {
            let keys: BTreeSet<&String> = expected.keys().chain(found.keys()).collect();
            for key in keys {
                let length = pointer.len();
                pointer.push('/');
                pointer.push_str(&pointer_token(key));
                let differs = match (expected.get(key), found.get(key)) {
                    (Some(expected), Some(found)) => difference_at(expected, found, pointer),
                    _ => true,
                };
                if differs {
                    return true;
                }
                pointer.truncate(length);
            }
            false
        }
        (Value::Array(expected), Value::Array(found)) => {
            for index in 0..expected.len().max(found.len()) {
                let length = pointer.len();
                pointer.push('/');
                pointer.push_str(&index.to_string());
                let differs = match (expected.get(index), found.get(index)) {
                    (Some(expected), Some(found)) => difference_at(expected, found, pointer),
                    _ => true,
                };
                if differs {
                    return true;
                }
                pointer.truncate(length);
            }
            false
        }
        (Value::Number(expected), Value::Number(found)) => !numbers_equal(expected, found),
        _ => expected != found,
    }
}

/// The first JSON pointer (object members in code-point order, array
/// elements in order) at which `found` is not canonically `expected`: a
/// member present in only one of them, a value of another type, or another
/// value. `None` when the two are canonically equal. `""` names the root.
pub fn first_difference(expected: &Value, found: &Value) -> Option<String> {
    let mut pointer = String::new();
    difference_at(expected, found, &mut pointer).then_some(pointer)
}

/// The input manifest, then the whole request, must canonically equal their
/// derivations (§2.5 step 9). The refusal names the first differing JSON
/// pointer, never a value.
pub fn check_matches_derivation(derivation: &UserReleaseDerivation, input_manifest: &Value, request: &Value) -> Result<(), String> {
    if let Some(pointer) = first_difference(&derivation.input_manifest, input_manifest) {
        return Err(format!("{CONTENT_NOT_DERIVED}: the input manifest differs from its derivation at {}", shown_pointer(&pointer)));
    }
    if let Some(pointer) = first_difference(&derivation.request, request) {
        return Err(format!("{CONTENT_NOT_DERIVED}: the request differs from its derivation at {}", shown_pointer(&pointer)));
    }
    Ok(())
}

/// The derivation check (§2.5 step 9, §2.8): `E(M, S, m, I, R, H, U)`, then
/// the input manifest and the whole request must equal it canonically. Only
/// [`assemble_derived`] calls it, and only it constructs a
/// [`DerivedUserRelease`].
fn check_derived(basis: &UserReleaseBasis<'_>, input_manifest: &Value, request: &Value) -> Result<DerivedUserRelease, String> {
    let derivation = derive(basis)?;
    check_matches_derivation(&derivation, input_manifest, request)?;
    Ok(DerivedUserRelease { _derived: () })
}

/// The producer's one entry that admits the user-released profile (§2.8,
/// refusal 3). Its preflight runs first on each result document, reading no
/// member of the report: item 0 (the public
/// `document_form::check_route_admitted`), then
/// `exact_sections::check_user_released_profile` with the profile constant,
/// so an exact document is refused with
/// `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED` whatever the request
/// carries. Only then does it derive the request from `basis` and require the
/// input manifest `input_manifest` and the whole `request` to equal the
/// derivation (`REPORT-USER-RELEASE-CONTENT-NOT-DERIVED`, naming the first
/// differing pointer). It then runs the shared producer under
/// [`ProducerEntry::UserRelease`]. Native admission's route-010 branch is its
/// one caller, through `report_package_bridge::assemble_user_released_request`
/// (`user_release_reach_scan`). It records no user release; the user's record
/// is the request's.
pub fn assemble_derived(
    request: Value,
    linked: &LinkedSolverIdentity<'_>,
    basis: &UserReleaseBasis<'_>,
    input_manifest: &Value,
) -> Result<crate::ReportPackageContainerOutcome, String> {
    for document in request.get("result_documents").and_then(Value::as_array).into_iter().flatten() {
        crate::document_form::check_route_admitted(document)?;
        crate::exact_sections::check_user_released_profile(USER_RELEASED_DESKTOP_PROFILE, document)?;
    }
    let derived = check_derived(basis, input_manifest, &request)?;
    let request = decode_report_package_request(request).map_err(|detail| format!("{PACKAGE_UNBOUND}: {detail}"))?;
    crate::wire::assemble_wire_request_in(request, linked, &ProducerEntry::UserRelease(derived), admitted::PRODUCTION)
}

/// The `content_binding` the writer adds to the manifest's copy of the record
/// (§2.3): one entry per non-manifest member, in archive order, each the
/// SHA-256 of that member's exact bytes.
pub(crate) fn content_binding(members: &[crate::ContainerMember]) -> Value {
    json!({
        "algorithm": "sha256",
        "members": members.iter().map(|member| json!({
            "file_name": member.file_name,
            "role": member.role,
            "sha256": member.sha256_hex
        })).collect::<Vec<_>>()
    })
}

/// A pointer as a refusal shows it: the root as `/`, a long one cut to its
/// first 240 characters.
fn shown_pointer(pointer: &str) -> String {
    match pointer {
        "" => "/".to_string(),
        long if long.chars().count() > 240 => long.chars().take(240).chain(std::iter::once('\u{2026}')).collect(),
        short => short.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VECTORS: &str = include_str!("../../../../fixtures/reports/invented/user_release_derivation_vectors.json");
    const MODEL: &str = include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json");
    const SOURCE: &str = include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json");

    fn linked() -> LinkedSolverIdentity<'static> {
        LinkedSolverIdentity {
            solver_name: "open_pipe_stress_product_physics",
            solver_version: "0.2.0",
            solver_build_ref: "open_pipe_stress_product_physics@0.2.0",
        }
    }

    /// The first vector's derived request, typed, with its converted audit
    /// manifest.
    fn typed(edit: impl FnOnce(&mut Value)) -> (crate::wire::ReportPackageRequest, open_pipe_stress_audit_manifest::AuditManifest) {
        let vector = serde_json::from_str::<Value>(VECTORS).unwrap()["vectors"][0].clone();
        assert_eq!(vector["source_fixture"], "fixtures/results/preview_physics_connected_sparse.json");
        let (model, source): (Value, Value) = (serde_json::from_str(MODEL).unwrap(), serde_json::from_str(SOURCE).unwrap());
        let record: UserReleaseRecord = serde_json::from_value(vector["user_release"].clone()).unwrap();
        let software = SoftwareIdentity { name: "SWBPIPE".into(), version: "0.1.0".into() };
        let basis = UserReleaseBasis {
            model: &model,
            source: &source,
            solver_mode: vector["solver_mode"].as_str().unwrap(),
            run_record: &vector["run_record"],
            software: &software,
            record: &record,
        };
        let mut request = derive(&basis).unwrap().request;
        edit(&mut request);
        let request = decode_report_package_request(request).unwrap();
        let audit = crate::wire::audit_manifest(request.audit_manifest.clone(), &linked()).unwrap();
        (request, audit)
    }

    /// `bindings::check_request`, a general entry, refuses every marker
    /// (§2.8, refusal 2); under the user-release entry the request carries no
    /// handoff record set and the user-released profile, context and record
    /// (item 8, the handoff rule).
    #[test]
    fn check_request_refuses_markers_and_the_entry_binds_its_form() {
        let (request, audit) = typed(|_| {});
        let refusal = crate::bindings::check_request(&request, &audit, &linked()).unwrap_err();
        assert!(refusal.starts_with(&format!("{PACKAGE_NOT_ADMITTED}:")) && refusal.ends_with("(marker: export_profile_id)"), "{refusal}");
        let entry = ProducerEntry::UserRelease(DerivedUserRelease { _derived: () });
        let check = |request: &crate::wire::ReportPackageRequest, audit| crate::bindings::check_request_in(request, audit, &linked(), &entry, admitted::PRODUCTION);
        assert_eq!(check(&request, &audit), Ok(()));
        let (handoff, audit) = typed(|request| request["state_comparison_handoff_records"] = json!([{"deliverable_id": "DEL-08-06"}]));
        assert!(check(&handoff, &audit).unwrap_err().starts_with(&format!("{PACKAGE_UNBOUND}:")));
        let (local, audit) = typed(|request| request["export_profile_id"] = json!(crate::bindings::DESKTOP_PROFILE));
        assert!(check(&local, &audit).unwrap_err().starts_with(&format!("{PACKAGE_UNBOUND}:")));
        let (no_record, audit) = typed(|request| {
            request["report"]["report_context"].as_object_mut().unwrap().remove("user_release");
        });
        assert!(check(&no_record, &audit).unwrap_err().starts_with(&format!("{PACKAGE_UNBOUND}:")));
    }

    #[test]
    fn safe_token_matches_the_desktop_rule() {
        assert_eq!(safe_token("run:preview-linear-static-001", "report-package"), "run-preview-linear-static-001");
        assert_eq!(safe_token("::a  b::", "x"), "a-b");
        assert_eq!(safe_token("", "current-session"), "current-session");
        assert_eq!(safe_token("--", "current-session"), "current-session");
        assert_eq!(safe_token("é.ok_1-2", "x"), ".ok_1-2");
        assert_eq!(safe_token("a//b", "x"), "a-b");
    }

    #[test]
    fn first_difference_names_the_first_pointer_in_canonical_order() {
        let base = json!({"b": [1, {"c": "x"}], "a": 5.0, "a/b": {"~": true}});
        assert_eq!(first_difference(&base, &base), None);
        // 5 and 5.0 are one canonical number.
        assert_eq!(first_difference(&base, &json!({"b": [1, {"c": "x"}], "a": 5, "a/b": {"~": true}})), None);
        assert_eq!(first_difference(&base, &json!({"b": [1, {"c": "y"}], "a": 5, "a/b": {"~": true}})).as_deref(), Some("/b/1/c"));
        assert_eq!(first_difference(&base, &json!({"b": [1, {"c": "x"}, 2], "a": 5, "a/b": {"~": true}})).as_deref(), Some("/b/2"));
        assert_eq!(first_difference(&base, &json!({"b": [1, {"c": "x"}], "a": 5, "a/b": {"~": false}})).as_deref(), Some("/a~1b/~0"));
        assert_eq!(first_difference(&base, &json!({"b": [1, {"c": "x"}], "a": 5, "a/b": {"~": true}, "extra": null})).as_deref(), Some("/extra"));
        assert_eq!(first_difference(&base, &json!([])).as_deref(), Some(""));
        assert_eq!(first_difference(&json!(0), &json!(-0.0)), None);
    }

    #[test]
    fn the_document_diagnostic_class_follows_the_desktop_rules() {
        assert_eq!(diagnostic_class("USER_RULE_X", "blocking"), "RULE_CHECK_BLOCKING");
        assert_eq!(diagnostic_class("USER_RULE_X", "error"), "ASSUMPTION_WARNING");
        assert_eq!(diagnostic_class("UNIT_X", "error"), "UNIT_WARNING");
        assert_eq!(diagnostic_class("X_PROVENANCE", "info"), "PROVENANCE_WARNING");
        assert_eq!(diagnostic_class("NONLINEAR_X", "error"), "SOLVE_BLOCKING");
        assert_eq!(diagnostic_class("NONLINEAR_X", "warning"), "NONLINEAR_WARNING");
        assert_eq!(diagnostic_class("OTHER", "info"), "ASSUMPTION_WARNING");
    }

    #[test]
    fn js_trim_and_utf16_order_are_the_desktop_rules() {
        assert_eq!(js_trim("\u{feff} a \u{3000}"), "a");
        assert_eq!(js_trim("\u{85}a"), "\u{85}a");
        // U+1F600 sorts after U+FF61 in code points but before it in UTF-16
        // code units (its high surrogate is U+D83D).
        let mut items = vec!["\u{1F600}".to_string(), "\u{FF61}".to_string()];
        items.sort_by(|left, right| utf16_order(left, right));
        assert_eq!(items, vec!["\u{1F600}".to_string(), "\u{FF61}".to_string()]);
    }
}

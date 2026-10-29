//! Deterministic hash-bound HTML calculation-report renderer (DEC-021 / A7).
//!
//! This crate consumes a validated `CalculationReport` (report assembly
//! metadata), `ReportSections` (disclosure content), and caller-supplied
//! result rows, and emits a single self-contained, scriptless HTML document
//! with a deterministic byte layout. The SHA-256 of the HTML bytes is the
//! canonical hash-bound rendered-report evidence, recorded alongside the
//! existing envelope-hash evidence. Three protected-content lint gates apply:
//! the bundled template surface (tested), assembled section text pre-render,
//! and the final document text post-render — any blocking finding or blocking
//! validation diagnostic marks the outcome export-blocked.
//!
//! It does not fetch network resources, embed scripts or external
//! references, compose report content from project files, write files, or
//! emit professional approval, certification, sealing, authentication, or
//! code-compliance claims. PDF or paper output produced from this document is
//! a derived, non-hash-bound view (`derived_print_view`).
//!
//! The assembled section model (`assemble_report_sections`) is
//! emission-neutral and shared: the HTML emission here and the hash-bound
//! deterministic PDF emission (`pdf_emitter`, DEC-061) are two deterministic
//! emissions of this one assembled section model.

#![forbid(unsafe_code)]

// Code review r6 at #1682 (DEL-08-01 Design `report-user-release.md` §2.8,
// "Threat model"): this crate's `#[cfg(test)]` items compile only into its
// own unit-test binary. Cargo sets `CARGO_PRIMARY_PACKAGE` only for a package
// the command selected (as `cargo test` selects this one), never for a
// dependency, so a build that sets `--cfg test` for this crate as a
// dependency of a product fails to compile here.
#[cfg(test)]
const _: () = assert!(option_env!("CARGO_PRIMARY_PACKAGE").is_some(), "cfg(test) is set for this reporting crate as a dependency (DEL-08-01 Design section 2.8)");

use open_pipe_stress_protected_content_linter as linter;
use open_pipe_stress_report_generator as report;
use open_pipe_stress_report_sections as sections;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod exact_statements;
pub mod object_form;

/// Fixed render order: mirrors the required-section contract of
/// `report_generator` (`REQUIRED_SECTION_KINDS`); rendering never reorders
/// based on input ordering.
const SECTION_ORDER: [&str; 8] = [
    "model_input_summary",
    "load_cases",
    "results",
    "warnings_assumptions_provenance",
    "audit_manifest",
    "rule_pack_references",
    "limitations",
    "professional_boundary_notice",
];

/// Inline stylesheet for the bundled public template. Local-only: system
/// font stack, no remote fonts, no external URLs.
pub const PUBLIC_REPORT_TEMPLATE_CSS: &str = "\
body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,'Helvetica Neue',Arial,sans-serif;color:#1a1a1a;margin:2rem auto;max-width:60rem;line-height:1.45;}\
h1{font-size:1.4rem;border-bottom:2px solid #1a1a1a;padding-bottom:.3rem;}\
h2{font-size:1.1rem;margin-top:1.6rem;border-bottom:1px solid #888;padding-bottom:.2rem;}\
table{border-collapse:collapse;width:100%;margin:.6rem 0;}\
th,td{border:1px solid #bbb;padding:.3rem .5rem;text-align:left;font-size:.85rem;vertical-align:top;}\
th{background:#f0f0f0;}\
.status-banner{border:2px solid #1a1a1a;padding:.6rem;margin:.8rem 0;font-weight:600;}\
.blocked-banner{border:3px solid #8b0000;color:#8b0000;padding:.6rem;margin:.8rem 0;font-weight:700;}\
.derived-banner{border:3px dashed #8b5a00;color:#8b5a00;padding:.6rem;margin:.8rem 0;font-weight:700;}\
.boundary{border:2px solid #444;background:#fafafa;padding:.8rem;margin:1rem 0;}\
.signoff td{height:2.2rem;}\
.meta{font-size:.8rem;color:#444;}\
.local-private-banner{border:3px solid #4b2a7b;color:#4b2a7b;padding:.6rem;margin:.8rem 0;font-weight:700;}\
@media print{body{margin:.5in;}}";

/// One displayed result row. Display strings are caller-formatted; they are
/// part of the hashed bytes, so the caller's formatting is part of the
/// deterministic input contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultRow {
    pub row_id: String,
    pub label: String,
    pub case_ref: String,
    pub quantity_display: String,
    pub source_ref: String,
}

/// Export context the generating application rendered this document under.
/// The application's route control owns the context and the explicit
/// local-private intent; this value only records the outcome so the document
/// can disclose it. It grants no export authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportExportContext {
    PublicReport,
    LocalPrivate,
    /// A report of the user's own session for which the user recorded, for
    /// that one report, a user release of their own project data (W-18b,
    /// DEL-08-01 Design `report-user-release.md` §3.1). Only the report
    /// package's derivation-bound entry renders it, with a
    /// [`UserReleaseRecord`] (§2.8); every other caller refuses it.
    UserReleasedReport,
}

/// The user-release record's kind (§2.3).
pub const USER_RELEASE_RECORD_KIND: &str = "user_release_of_own_project_data";
/// The user-release record's version (§2.3). A later slice that widens what a
/// user-released package carries changes `scope.not_carried` and this.
pub const USER_RELEASE_RECORD_VERSION: &str = "1.0.0";
/// D-6 widened scope, retaining the legacy record form.
pub const USER_RELEASE_EXACT_RECORD_VERSION: &str = "1.1.0";
/// The statement the user ticks in the dialog, verbatim (§2.2, §2.3).
pub const USER_RELEASE_STATEMENT: &str =
    "I hold the right to share this project's data, and I release my project data in this report for sharing.";
/// The basis the statement rests on (§2.3).
pub const USER_RELEASE_STATEMENT_BASIS: &str =
    "docs/IP_AND_DATA_BOUNDARY.md \u{a7}6: a user intentionally exports their own data with documented redistribution rights";
/// The record's standing: a user's statement about rights in their data,
/// nothing more (§0, naming rule).
pub const USER_RELEASE_STANDING: &str = "user_statement_of_data_rights_only";
/// The record's scope kind (§2.3).
pub const USER_RELEASE_SCOPE_KIND: &str = "this_report_only";
/// The one data class a user release covers (§2.3).
pub const USER_RELEASED_DATA_CLASS: &str = "private_project_data";
/// What a user-released package does not carry under scope R (§2.1, §2.3).
pub const USER_RELEASE_NOT_CARRIED: [&str; 6] =
    ["rule_pack_references", "user_rule_outcomes", "run_comparisons", "handoff_records", "library_records", "audit_assets"];

/// The report a user release covers (§2.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserReleaseScope {
    pub scope_kind: String,
    /// `report:<analysis_run_ref>`, the report's `calculation_report.report_id`.
    pub report_id: String,
    /// The request's `package_id`, `desktop-user-released-report-<run>`.
    pub package_id: String,
    pub user_released_data_class: String,
    pub not_carried: Vec<String>,
}

/// The identities a user release is bound to (§2.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserReleaseBoundTo {
    pub model_ref: String,
    pub model_document_sha256: String,
    pub analysis_run_ref: String,
    pub input_manifest_ref: String,
    pub input_manifest_sha256: String,
    pub result_envelope_id: String,
    pub result_document_derivative_sha256: String,
}

/// A user's per-report user release of their own project data (W-18b,
/// DEL-08-01 Design `report-user-release.md` §2.3): the user's statement that
/// they hold the right to share this project's data, bound to the model and
/// result one report covers. It records a statement about rights in the
/// data; it is not an issuance act and records no acceptance. Every member is
/// required and no other is admitted; it never carries `content_binding`,
/// which only the package manifest's copy has.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserReleaseRecord {
    pub record_kind: String,
    pub record_version: String,
    pub statement: String,
    pub statement_basis: String,
    pub standing: String,
    pub scope: UserReleaseScope,
    pub bound_to: UserReleaseBoundTo,
    /// The application clock when the user confirmed, UTC,
    /// `YYYY-MM-DDTHH:MM:SSZ`.
    pub user_released_at: String,
}

/// `YYYY-MM-DDTHH:MM:SSZ` with a month, day, hour, minute and second in range.
fn is_user_released_at(text: &str) -> bool {
    let bytes = text.as_bytes();
    let digits = |range: std::ops::Range<usize>| bytes[range.clone()].iter().all(u8::is_ascii_digit).then(|| text[range].parse::<u32>().ok()).flatten();
    if bytes.len() != 20 || bytes[4] != b'-' || bytes[7] != b'-' || bytes[10] != b'T' || bytes[13] != b':' || bytes[16] != b':' || bytes[19] != b'Z' {
        return false;
    }
    match (digits(0..4), digits(5..7), digits(8..10), digits(11..13), digits(14..16), digits(17..19)) {
        (Some(_), Some(month), Some(day), Some(hour), Some(minute), Some(second)) => {
            (1..=12).contains(&month) && (1..=31).contains(&day) && hour < 24 && minute < 60 && second < 60
        }
        _ => false,
    }
}

impl UserReleaseRecord {
    /// The first member whose value is not of the defined form (§2.3): a
    /// constant that differs, a hex value that is not 64 lowercase hex
    /// characters, a `user_released_at` not of the form
    /// `YYYY-MM-DDTHH:MM:SSZ`, or an empty identity. `None` when every member
    /// is of its form. It names the member only, never its value.
    pub fn form_defect(&self) -> Option<&'static str> {
        let constants = [
            ("record_kind", self.record_kind.as_str(), USER_RELEASE_RECORD_KIND),
            ("statement", self.statement.as_str(), USER_RELEASE_STATEMENT),
            ("statement_basis", self.statement_basis.as_str(), USER_RELEASE_STATEMENT_BASIS),
            ("standing", self.standing.as_str(), USER_RELEASE_STANDING),
            ("scope.scope_kind", self.scope.scope_kind.as_str(), USER_RELEASE_SCOPE_KIND),
            ("scope.user_released_data_class", self.scope.user_released_data_class.as_str(), USER_RELEASED_DATA_CLASS),
        ];
        if let Some((member, _, _)) = constants.iter().find(|(_, found, expected)| found != expected) {
            return Some(member);
        }
        let exact = self.record_version == USER_RELEASE_EXACT_RECORD_VERSION;
        if !exact && self.record_version != USER_RELEASE_RECORD_VERSION {
            return Some("record_version");
        }
        let expected = USER_RELEASE_NOT_CARRIED.into_iter().chain(exact.then_some("ineligible_exact_route_results"));
        if self.scope.not_carried.iter().map(String::as_str).ne(expected) {
            return Some("scope.not_carried");
        }
        let bound = &self.bound_to;
        for (member, hex) in [
            ("bound_to.model_document_sha256", &bound.model_document_sha256),
            ("bound_to.input_manifest_sha256", &bound.input_manifest_sha256),
            ("bound_to.result_document_derivative_sha256", &bound.result_document_derivative_sha256),
        ] {
            if !is_lower_hex_64(hex) {
                return Some(member);
            }
        }
        for (member, identity) in [
            ("scope.report_id", &self.scope.report_id),
            ("scope.package_id", &self.scope.package_id),
            ("bound_to.model_ref", &bound.model_ref),
            ("bound_to.analysis_run_ref", &bound.analysis_run_ref),
            ("bound_to.input_manifest_ref", &bound.input_manifest_ref),
            ("bound_to.result_envelope_id", &bound.result_envelope_id),
        ] {
            if identity.trim().is_empty() {
                return Some(member);
            }
        }
        if !is_user_released_at(&self.user_released_at) {
            return Some("user_released_at");
        }
        None
    }
}

/// The user-release notice for `record` (§5.1), shared by the HTML, the PDF
/// and the package reader's notice check.
pub fn user_release_notice_for(record: &UserReleaseRecord) -> String {
    let bound = &record.bound_to;
    format!(
        "USER-RELEASED REPORT \u{2014} the user released their own project data in this report for sharing, stating that they hold the right to redistribute it (IP and data boundary \u{a7}6). The user release covers this report only: model SHA-256 {}, result document {} (SHA-256 {}), user-released {}. The report carries this model's summary, load cases and solve results, and no rule-pack reference, user-rule outcome or run comparison. The application found no library or rule-pack mark in this model. It cannot verify who holds the rights in the data, which rest on the user's statement. This statement concerns rights in the data only; the analysis status and the Professional Boundary notice below are unchanged.",
        bound.model_document_sha256, bound.result_envelope_id, bound.result_document_derivative_sha256, record.user_released_at
    )
}

/// The input's user-release record, when its export context is
/// `user_released_report` and it carries one.
pub fn user_released_record(input: &RenderableReportInput) -> Option<&UserReleaseRecord> {
    let context = input.report_context.as_ref()?;
    match context.export_context {
        ReportExportContext::UserReleasedReport => context.user_release.as_ref(),
        _ => None,
    }
}

/// Emission-neutral user-release notice shared by the HTML and PDF emissions
/// (§5.1): present only for `user_released_report` with a record, in place of
/// the local-private notice (the two contexts exclude each other).
pub fn user_release_notice_text(input: &RenderableReportInput) -> Option<String> {
    user_released_record(input).map(user_release_notice_for)
}

/// One recorded solver setting, display-formatted by the caller. The key and
/// value are part of the hashed bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportSetting {
    pub key: String,
    pub value: String,
}

/// Run and software context of the report (PRD §19.2): software, solver and
/// build identity, date/time, solver settings and coordinate system. Every
/// value is supplied by the caller from recorded session data; `None` (or an
/// empty string) renders as an explicit "not recorded" entry. The renderer
/// never reads a clock or the environment, so identical input renders
/// identical bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportContext {
    pub export_context: ReportExportContext,
    /// Caller-supplied date/time at which the application requested this render.
    pub generated_at: Option<String>,
    pub software_name: String,
    pub software_version: Option<String>,
    pub build_release_identifier: Option<String>,
    pub solver_name: Option<String>,
    pub solver_version: Option<String>,
    pub solver_build_ref: Option<String>,
    pub analysis_run_name: Option<String>,
    pub analysis_run_recorded_at: Option<String>,
    pub solver_mode: Option<String>,
    pub solver_settings_ref: Option<String>,
    pub solver_settings: Vec<ReportSetting>,
    pub model_coordinate_system: Option<String>,
    pub result_coordinate_systems: Vec<String>,
    /// The user's per-report user release (W-18b, §2.3): present only under
    /// `user_released_report`. Absent, it serializes to nothing, so every
    /// other input renders byte-identically.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_release: Option<UserReleaseRecord>,
}

/// Automatic user-rule check status vocabulary (the three statuses the rule
/// check runner may emit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRuleCheckStatus {
    RuleInputsIncomplete,
    UserRuleChecked,
    UserRuleFailed,
}

/// One user-rule check outcome: identity, status and the names of required
/// inputs that were not supplied. Formula text, limits and computed or
/// allowable values are deliberately not part of the report contract
/// (rule packs are user/private design-basis artifacts; only safe metadata
/// and required-input status are reported).
///
/// Since rule-pack format 0.5.0 a check has one outcome per load case or
/// combination its stress category maps (DEL-06-01 Design `rule-pack-0.5.0.md`
/// §2.7, §6), so a check id repeats once per basis and `basis_ref` names the
/// one this outcome was evaluated on. It is optional so that a 2.0.0 report
/// package written before this member existed still reads and renders (shown
/// as "load case not recorded"); the desktop always writes it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRuleCheckOutcome {
    pub check_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basis_ref: Option<UserRuleCheckBasisRef>,
    pub status: UserRuleCheckStatus,
    pub missing_inputs: Vec<String>,
}

/// The model load case or combination a user-rule outcome was evaluated on
/// (`ref_type` `load_case` or `combination`; `ref_id` the model's id).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRuleCheckBasisRef {
    pub ref_type: String,
    pub ref_id: String,
}

/// Shown once beside the rule-check headline (design §2.7): the counts count
/// outcomes, one per check and load case or combination.
pub const USER_RULE_CHECK_COUNT_SCOPE_NOTE: &str =
    "Counts are per check and load case or combination; one check can contribute more than once.";

/// The load case or combination cell of one outcome.
fn user_rule_check_basis_display(check: &UserRuleCheckOutcome) -> String {
    match &check.basis_ref {
        Some(basis) => format!("{} {}", basis.ref_type, basis.ref_id),
        None => "load case not recorded".to_string(),
    }
}

/// The user-rule check run recorded in the session for this report's result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRuleCheckRun {
    pub rule_pack_id: String,
    pub grammar_version: String,
    pub aggregate_status: UserRuleCheckStatus,
    pub checks: Vec<UserRuleCheckOutcome>,
}

/// The admission binding of an exact-route report (W-11, DEL-08-01 Design
/// `exact-route-report-sections.md` §3.1-§3.2), field for field: what Export
/// Result JSON admitted in the session. The renderer checks it against the
/// report's own records and composes the standing line from it; the input
/// carries no standing text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExactAdmissionBinding {
    pub semantic_contract_id: String,
    pub run_ref: String,
    pub result_envelope_id: String,
    pub received_result_sha256: String,
    pub model_sha256: String,
    pub input_manifest_ref: String,
    pub input_manifest_sha256: String,
    pub derivative_sha256: String,
    pub native_job_id: String,
    pub received_bytes_sha256: String,
}

/// One section basis's stress headline (§3.3): `basis` is `nominal_section`
/// or `reduced_section`; a withheld headline carries the withheld text and no
/// references.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportHeadline {
    pub basis: String,
    pub label: String,
    pub value_display: String,
    pub location_ref: Option<String>,
    pub source_ref: Option<String>,
}

/// The exact-route result block (§3.2): the result's producer identity, its
/// admission binding, the load/reference line where present and the
/// headlines, nominal then reduced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExactRouteSection {
    pub semantic_contract_id: String,
    pub admission: ExactAdmissionBinding,
    pub load_reference_states: Option<String>,
    pub headlines: Vec<ReportHeadline>,
}

/// One library record the model references (W-11 §4.2): safe metadata only,
/// no library payload or record snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryReference {
    pub reference_id: String,
    pub library_kind: String,
    pub library_id: String,
    pub library_version: String,
    pub record_id: String,
    pub entity_refs: Vec<String>,
    pub provenance: sections::Provenance,
}

/// Complete renderer input document. `report_context` and `user_rule_checks`
/// are absent only when the producer recorded no such context (rendered as
/// "not recorded") or no rule-check run exists. `exact_route` and
/// `library_references` (W-11) are absent on the pressure-free route and
/// without library references; absent, they emit nothing and serialize to
/// nothing, so such an input renders byte-identically.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderableReportInput {
    pub report_title: String,
    pub calculation_report: report::CalculationReport,
    pub report_sections: sections::ReportSections,
    pub result_rows: Vec<ResultRow>,
    #[serde(default)]
    pub report_context: Option<ReportContext>,
    #[serde(default)]
    pub user_rule_checks: Option<UserRuleCheckRun>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_route: Option<ExactRouteSection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub library_references: Option<Vec<LibraryReference>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OutcomeDiagnostic {
    pub code: String,
    pub severity: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OutcomeFinding {
    pub finding_id: String,
    pub code: String,
    pub severity: String,
    pub path: String,
    pub line: usize,
    pub column: usize,
    pub excerpt: String,
    pub message: String,
    pub blocking: bool,
}

/// Render result. `html` is produced for every render (a blocked document
/// renders with a visible blocked banner for diagnosis) except one refused for
/// its provenance records, whose content is withheld (`content_withheld`:
/// `html` empty, no lint findings, section messages withheld; W-11 slice (c),
/// §6.2). `export_blocked` is the gate the app must honor before saving,
/// printing, or exporting the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderOutcome {
    pub html: String,
    pub sha256_hex: String,
    pub export_blocked: bool,
    pub blocking_reasons: Vec<String>,
    pub report_validation_diagnostics: Vec<OutcomeDiagnostic>,
    pub section_validation_diagnostics: Vec<OutcomeDiagnostic>,
    pub pre_render_findings: Vec<OutcomeFinding>,
    pub post_render_findings: Vec<OutcomeFinding>,
    /// True only for a metadata-only refusal; absent from the serialized
    /// outcome otherwise, so every other outcome serializes as before.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub content_withheld: bool,
}

/// The section message a metadata-only refusal carries in place of each
/// section diagnostic's own (which may quote a record's source name).
pub const WITHHELD_SECTION_MESSAGE: &str = "Message withheld: this render was refused for its provenance records, so no report content is returned.";

pub fn escape_html(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(64);
    for byte in digest {
        hex.push_str(&format!("{byte:02x}"));
    }
    hex
}

/// The bundled public template skeleton (lint gate i surface): the document
/// frame with an empty body slot, exactly as `render_calculation_report`
/// emits around assembled content.
pub fn public_report_template() -> String {
    document_frame("", "")
}

fn document_frame(title: &str, body: &str) -> String {
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
<title>{title}</title>\n<style>{css}</style>\n</head>\n<body>\n{body}</body>\n</html>\n",
        title = title,
        css = PUBLIC_REPORT_TEMPLATE_CSS,
        body = body
    )
}

fn reference_text(reference: &report::Reference) -> String {
    format!("{}:{}", reference.ref_type, reference.ref_id)
}

fn section_reference_text(reference: &sections::Reference) -> String {
    format!("{}:{}", reference.ref_type, reference.ref_id)
}

fn checksum_text(checksum: &report::ChecksumRef) -> String {
    format!(
        "{} ({}, {}) = {}",
        reference_text(&checksum.payload_ref),
        checksum.algorithm,
        checksum.canonicalization,
        checksum.value
    )
}

fn model_units_text(units: &std::collections::BTreeMap<String, String>) -> String {
    if units.is_empty() {
        "none".to_string()
    } else {
        units
            .iter()
            .map(|(dimension, unit)| format!("{dimension}={unit}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn result_units_text(units: &[String]) -> String {
    if units.is_empty() {
        "none".to_string()
    } else {
        units.join(", ")
    }
}

const NOT_RECORDED: &str = "not recorded";

fn recorded(value: Option<&String>) -> String {
    match value.map(|text| text.trim()) {
        Some(text) if !text.is_empty() => text.to_string(),
        _ => NOT_RECORDED.to_string(),
    }
}

fn recorded_list(values: &[String]) -> String {
    let present: Vec<&str> = values
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .collect();
    if present.is_empty() {
        NOT_RECORDED.to_string()
    } else {
        present.join(", ")
    }
}

/// Registered display form of a user-rule check status token
/// (`docs/claims_registry.md` §2.1, DEC-102): the label is shown with its
/// authority domain and with the token reachable in place.
fn user_rule_check_status_display(status: &UserRuleCheckStatus) -> String {
    let (token, label) = match status {
        UserRuleCheckStatus::RuleInputsIncomplete => {
            ("RULE_INPUTS_INCOMPLETE", "Rule inputs incomplete")
        }
        UserRuleCheckStatus::UserRuleChecked => ("USER_RULE_CHECKED", "User-rule checked"),
        UserRuleCheckStatus::UserRuleFailed => ("USER_RULE_FAILED", "User-rule failed"),
    };
    format!("User-rule-check authority · {label} ({token})")
}

/// FR-RULE-003 (DEL-06-03 Design `pack-aggregate-incomplete.md`): while any
/// check is incomplete the pack headline is "INCOMPLETE — n check(s) failed;
/// m check(s) blocked", counted from the checks themselves, so a run recorded
/// under the earlier failed-first aggregate never headlines a fail either.
fn user_rule_check_aggregate_headline(run: &UserRuleCheckRun) -> String {
    let count = |status: UserRuleCheckStatus| run.checks.iter().filter(|check| check.status == status).count();
    let blocked = count(UserRuleCheckStatus::RuleInputsIncomplete);
    if blocked == 0 {
        return user_rule_check_status_display(&run.aggregate_status);
    }
    format!(
        "INCOMPLETE — {} check(s) failed; {} check(s) blocked · {}",
        count(UserRuleCheckStatus::UserRuleFailed),
        blocked,
        user_rule_check_status_display(&UserRuleCheckStatus::RuleInputsIncomplete)
    )
}

fn export_context_label(context: &ReportExportContext) -> &'static str {
    match context {
        ReportExportContext::PublicReport => "public_report",
        ReportExportContext::LocalPrivate => "local_private",
        ReportExportContext::UserReleasedReport => "user_released_report",
    }
}

fn run_context_rows(input: &RenderableReportInput) -> Vec<Vec<String>> {
    let context = input.report_context.as_ref();
    let field = |pick: fn(&ReportContext) -> Option<&String>| recorded(context.and_then(pick));
    let mut rows = vec![
        vec![
            "Report export context".to_string(),
            context
                .map(|context| export_context_label(&context.export_context).to_string())
                .unwrap_or_else(|| NOT_RECORDED.to_string()),
        ],
        vec![
            "Report generated at (application clock)".to_string(),
            field(|context| context.generated_at.as_ref()),
        ],
        vec![
            "Software".to_string(),
            field(|context| Some(&context.software_name)),
        ],
        vec![
            "Software version".to_string(),
            field(|context| context.software_version.as_ref()),
        ],
        vec![
            "Build/release identifier".to_string(),
            field(|context| context.build_release_identifier.as_ref()),
        ],
        vec![
            "Solver".to_string(),
            field(|context| context.solver_name.as_ref()),
        ],
        vec![
            "Solver version".to_string(),
            field(|context| context.solver_version.as_ref()),
        ],
        vec![
            "Solver build reference".to_string(),
            field(|context| context.solver_build_ref.as_ref()),
        ],
        vec![
            "Analysis run".to_string(),
            section_reference_text(&input.report_sections.run_ref),
        ],
        vec![
            "Analysis run name".to_string(),
            field(|context| context.analysis_run_name.as_ref()),
        ],
        vec![
            "Analysis run recorded at".to_string(),
            field(|context| context.analysis_run_recorded_at.as_ref()),
        ],
        vec![
            "Solver mode".to_string(),
            field(|context| context.solver_mode.as_ref()),
        ],
        vec![
            "Solver settings reference".to_string(),
            field(|context| context.solver_settings_ref.as_ref()),
        ],
    ];
    match context.map(|context| context.solver_settings.as_slice()) {
        Some(settings) if !settings.is_empty() => {
            for setting in settings {
                rows.push(vec![
                    format!("Solver setting: {}", setting.key),
                    setting.value.clone(),
                ]);
            }
        }
        _ => rows.push(vec![
            "Solver settings".to_string(),
            NOT_RECORDED.to_string(),
        ]),
    }
    // W-18b (§5.1): the user release's rows, only for `user_released_report`
    // with a record. `content_binding` is never rendered.
    if let Some(record) = user_released_record(input) {
        let bound = &record.bound_to;
        rows.push(vec!["User release statement".to_string(), record.statement.clone()]);
        rows.push(vec!["User release time (application clock)".to_string(), record.user_released_at.clone()]);
        rows.push(vec![
            "User release bound to".to_string(),
            format!(
                "model {} (SHA-256 {}); run {}; input manifest {} (SHA-256 {}); result document {} (SHA-256 {})",
                bound.model_ref,
                bound.model_document_sha256,
                bound.analysis_run_ref,
                bound.input_manifest_ref,
                bound.input_manifest_sha256,
                bound.result_envelope_id,
                bound.result_document_derivative_sha256
            ),
        ]);
    }
    rows
}

fn analysis_status_label(status: &sections::AnalysisStatus) -> &'static str {
    match status {
        sections::AnalysisStatus::ModelIncomplete => "model_incomplete",
        sections::AnalysisStatus::MechanicsSolved => "mechanics_solved",
        sections::AnalysisStatus::RuleInputsIncomplete => "rule_inputs_incomplete",
        sections::AnalysisStatus::UserRuleChecked => "user_rule_checked",
        sections::AnalysisStatus::UserRuleFailed => "user_rule_failed",
        sections::AnalysisStatus::HumanReviewRequired => "human_review_required",
    }
}

fn severity_label(severity: &sections::Severity) -> &'static str {
    match severity {
        sections::Severity::Info => "info",
        sections::Severity::Warning => "warning",
        sections::Severity::Blocking => "blocking",
    }
}

fn row(cells: &[String]) -> String {
    let mut html = String::from("<tr>");
    for cell in cells {
        html.push_str("<td>");
        html.push_str(&escape_html(cell));
        html.push_str("</td>");
    }
    html.push_str("</tr>\n");
    html
}

fn header_row(cells: &[String]) -> String {
    let mut html = String::from("<tr>");
    for cell in cells {
        html.push_str("<th>");
        html.push_str(&escape_html(cell));
        html.push_str("</th>");
    }
    html.push_str("</tr>\n");
    html
}

fn map_privacy(value: &report::PrivacyClassification) -> linter::PrivacyClassification {
    match value {
        report::PrivacyClassification::PublicMetadata => {
            linter::PrivacyClassification::PublicMetadata
        }
        report::PrivacyClassification::InventedPublicExample => {
            linter::PrivacyClassification::InventedPublicExample
        }
        report::PrivacyClassification::PrivateProjectData => {
            linter::PrivacyClassification::PrivateProjectData
        }
        report::PrivacyClassification::PrivateRulePackData => {
            linter::PrivacyClassification::PrivateRulePackData
        }
        report::PrivacyClassification::PrivateLibraryData => {
            linter::PrivacyClassification::PrivateLibraryData
        }
        report::PrivacyClassification::ProtectedSuspected => {
            linter::PrivacyClassification::ProtectedSuspected
        }
        report::PrivacyClassification::Redacted => linter::PrivacyClassification::Redacted,
    }
}

fn map_redistribution(value: &report::RedistributionStatus) -> linter::RedistributionStatus {
    match value {
        report::RedistributionStatus::PublicPermissive => {
            linter::RedistributionStatus::PublicPermissive
        }
        report::RedistributionStatus::PrivateOnly => linter::RedistributionStatus::PrivateOnly,
        report::RedistributionStatus::Unknown => linter::RedistributionStatus::Unknown,
        report::RedistributionStatus::ProtectedSuspected => {
            linter::RedistributionStatus::ProtectedSuspected
        }
        report::RedistributionStatus::InventedNonEngineeringExample => {
            linter::RedistributionStatus::InventedNonEngineeringExample
        }
        report::RedistributionStatus::Tbd => linter::RedistributionStatus::Tbd,
    }
}

fn map_review_status(value: &str) -> linter::ReviewStatus {
    // Unknown review-status strings stay Pending so the lint surface keeps
    // human review visible instead of silently accepting.
    match value.trim().to_ascii_lowercase().as_str() {
        "accepted" => linter::ReviewStatus::Accepted,
        "rejected" => linter::ReviewStatus::Rejected,
        "quarantined" => linter::ReviewStatus::Quarantined,
        "not_applicable" => linter::ReviewStatus::NotApplicable,
        _ => linter::ReviewStatus::Pending,
    }
}

/// Map report provenance onto the linter's provenance model. Public so the
/// deterministic PDF emitter (DEC-061) gates its text model with the same
/// provenance mapping the HTML emission uses.
pub fn lint_provenance(provenance: &report::Provenance) -> linter::Provenance {
    linter::Provenance {
        source_name: provenance.source_name.clone(),
        source_location: provenance.source_location.clone(),
        source_license: provenance.source_license.clone(),
        contributor: provenance.contributor.clone(),
        contributor_certification: provenance.contributor_certification.clone(),
        redistribution_status: map_redistribution(&provenance.redistribution_status),
        review_status: map_review_status(&provenance.review_status),
        privacy_classification: map_privacy(&provenance.privacy_classification),
    }
}

fn outcome_finding(finding: &linter::LintFinding) -> OutcomeFinding {
    OutcomeFinding {
        finding_id: finding.finding_id.clone(),
        code: format!("{:?}", finding.code),
        severity: format!("{:?}", finding.severity).to_ascii_lowercase(),
        path: finding.source_location.path.clone(),
        line: finding.source_location.line,
        column: finding.source_location.column,
        excerpt: finding.excerpt.clone(),
        message: finding.message.clone(),
        blocking: finding.is_blocking(),
    }
}

fn report_outcome_diagnostic(diagnostic: &report::Diagnostic) -> OutcomeDiagnostic {
    OutcomeDiagnostic {
        code: diagnostic.code.clone(),
        severity: format!("{:?}", diagnostic.severity).to_ascii_lowercase(),
        message: diagnostic.message.clone(),
    }
}

fn section_outcome_diagnostic(diagnostic: &sections::Diagnostic) -> OutcomeDiagnostic {
    OutcomeDiagnostic {
        code: diagnostic.code.clone(),
        severity: severity_label(&diagnostic.severity).to_string(),
        message: diagnostic.message.clone(),
    }
}

/// One content block of an assembled report section. This is the
/// emission-neutral pre-render model shared by the HTML emission in this
/// crate and the hash-bound deterministic PDF emission (`pdf_emitter`,
/// DEC-061). All text is raw (unescaped and unencoded); each emitter applies
/// its own escaping or byte encoding deterministically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SectionBlock {
    /// Sub-heading inside a section (HTML `<h3>`).
    Subheading { text: String },
    /// Paragraph; `meta` selects the small-print style (HTML `p.meta`).
    Paragraph { text: String, meta: bool },
    /// Table with one header row. `signoff` selects the human sign-off style
    /// with taller blank entry rows.
    Table {
        header: Vec<String>,
        rows: Vec<Vec<String>>,
        signoff: bool,
    },
    /// Professional-boundary banner box (HTML `div.boundary`).
    BoundaryBox { text: String },
}

/// One assembled report section: fixed key and title plus ordered content
/// blocks. Assembly is deterministic in the input; emitters never reorder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssembledSection {
    pub key: &'static str,
    pub title: &'static str,
    pub blocks: Vec<SectionBlock>,
}

fn subheading(text: &str) -> SectionBlock {
    SectionBlock::Subheading {
        text: text.to_string(),
    }
}

fn paragraph(text: &str) -> SectionBlock {
    SectionBlock::Paragraph {
        text: text.to_string(),
        meta: false,
    }
}

fn meta_paragraph(text: &str) -> SectionBlock {
    SectionBlock::Paragraph {
        text: text.to_string(),
        meta: true,
    }
}

fn table(header: &[&str], rows: Vec<Vec<String>>) -> SectionBlock {
    SectionBlock::Table {
        header: header.iter().map(|cell| cell.to_string()).collect(),
        rows,
        signoff: false,
    }
}

/// Assemble the eight required report sections in fixed order from the
/// validated input. This is the single assembled section model both
/// deterministic emissions (HTML here, PDF in `pdf_emitter`) consume.
pub fn assemble_report_sections(input: &RenderableReportInput) -> Vec<AssembledSection> {
    let calc = &input.calculation_report;
    let disclosure = &input.report_sections;
    let mut assembled = Vec::new();

    for key in SECTION_ORDER {
        let (title, blocks) = match key {
            "model_input_summary" => {
                let summary = &calc.model_input_summary;
                let mut rows = vec![
                    vec!["Project".to_string(), reference_text(&summary.project_ref)],
                    vec!["Model".to_string(), reference_text(&summary.model_ref)],
                    vec![
                        "Persistence envelope".to_string(),
                        reference_text(&summary.persistence_ref),
                    ],
                    vec![
                        "Unit system".to_string(),
                        reference_text(&summary.unit_system_ref),
                    ],
                ];
                if let Some(unit_display) = &summary.unit_display_summary {
                    rows.push(vec![
                        "Unit storage convention".to_string(),
                        unit_display.storage_convention.clone(),
                    ]);
                    rows.push(vec![
                        "Model units".to_string(),
                        model_units_text(&unit_display.model_units),
                    ]);
                    rows.push(vec![
                        "Result units".to_string(),
                        result_units_text(&unit_display.result_units),
                    ]);
                    rows.push(vec![
                        "Quantity display policy".to_string(),
                        unit_display.quantity_display_policy.clone(),
                    ]);
                    rows.push(vec![
                        "Report-time conversion".to_string(),
                        if unit_display.conversion_performed {
                            "true"
                        } else {
                            "false"
                        }
                        .to_string(),
                    ]);
                }
                let context = input.report_context.as_ref();
                rows.push(vec![
                    "Model coordinate system".to_string(),
                    recorded(context.and_then(|context| context.model_coordinate_system.as_ref())),
                ]);
                rows.push(vec![
                    "Result coordinate systems".to_string(),
                    context
                        .map(|context| recorded_list(&context.result_coordinate_systems))
                        .unwrap_or_else(|| NOT_RECORDED.to_string()),
                ]);
                rows.push(vec![
                    "Model hash".to_string(),
                    checksum_text(&summary.model_hash),
                ]);
                rows.push(vec![
                    "Input manifest".to_string(),
                    reference_text(&summary.input_manifest_ref),
                ]);
                (
                    "Model Input Summary",
                    vec![table(&["Item", "Reference"], rows)],
                )
            }
            "load_cases" => {
                let rows = calc
                    .load_case_summary
                    .iter()
                    .map(|case| {
                        vec![
                            reference_text(&case.load_ref),
                            case.label.clone(),
                            case.basis.clone(),
                            reference_text(&case.source),
                        ]
                    })
                    .collect();
                (
                    "Load Cases",
                    vec![table(&["Load case", "Label", "Basis", "Source"], rows)],
                )
            }
            "results" => {
                let mut blocks = Vec::new();
                if let Some(statement) = display_rule_statement(input) {
                    blocks.push(paragraph(statement));
                }
                if let Some(route) = &input.exact_route {
                    blocks.extend(exact_route_blocks(input, route));
                }
                if input.result_rows.is_empty() {
                    blocks.push(paragraph("No result rows supplied for this rendering."));
                } else {
                    let rows = input
                        .result_rows
                        .iter()
                        .map(|result_row| {
                            vec![
                                result_row.label.clone(),
                                result_row.case_ref.clone(),
                                result_row.quantity_display.clone(),
                                result_row.source_ref.clone(),
                            ]
                        })
                        .collect();
                    blocks.push(table(&["Result", "Case", "Value", "Source"], rows));
                }
                let export_rows = calc
                    .result_export_refs
                    .iter()
                    .map(|envelope| {
                        vec![
                            reference_text(&envelope.reference),
                            checksum_text(&envelope.checksum),
                        ]
                    })
                    .collect();
                blocks.push(table(
                    &["Referenced result export", "Checksum"],
                    export_rows,
                ));
                ("Results", blocks)
            }
            "warnings_assumptions_provenance" => {
                let unresolved = unresolved_provenance_warnings(input);
                // Diagnostic evidence (DEL-08-01 Design
                // `exact-route-report-sections.md` §3.7): a numerical-integrity
                // diagnostic or a long message is shown in the table by a short
                // cell that names its evidence label, and verbatim in the
                // section's last block.
                let mut evidence: Vec<(String, &sections::Diagnostic)> = Vec::new();
                let diagnostic_rows = sections::sorted_diagnostics(disclosure)
                    .into_iter()
                    .chain(unresolved.iter())
                    .map(|diagnostic| {
                        let message = if diagnostic_message_is_evidence(diagnostic) {
                            let label = format!("E{}", evidence.len() + 1);
                            let cell = diagnostic_evidence_cell(diagnostic, &label);
                            evidence.push((label, diagnostic));
                            cell
                        } else {
                            diagnostic.message.clone()
                        };
                        vec![
                            diagnostic.code.clone(),
                            severity_label(&diagnostic.severity).to_string(),
                            message,
                            diagnostic.remediation.clone(),
                        ]
                    })
                    .collect();
                let assumption_rows = disclosure
                    .assumptions
                    .iter()
                    .map(|assumption| {
                        vec![
                            assumption.assumption_id.clone(),
                            assumption.owner.clone(),
                            assumption.statement.clone(),
                            section_reference_text(&assumption.affected_scope),
                        ]
                    })
                    .collect();
                let value_rows = disclosure
                    .user_supplied_values
                    .iter()
                    .map(|value| {
                        let quantity = match &value.quantity {
                            Some(quantity) => format!(
                                "{} {} [{:?}]",
                                quantity.magnitude, quantity.unit, quantity.dimension
                            ),
                            None => "TBD".to_string(),
                        };
                        vec![
                            value.value_id.clone(),
                            value.value_category.clone(),
                            quantity,
                            section_reference_text(&value.source),
                            if value.missing_data_finding {
                                "yes"
                            } else {
                                "no"
                            }
                            .to_string(),
                        ]
                    })
                    .collect();
                let note_rows = disclosure
                    .provenance_notes
                    .iter()
                    .map(|note| {
                        vec![
                            note.source_name.clone(),
                            note.source_location.clone(),
                            note.source_license.clone(),
                            note.contributor.clone(),
                        ]
                    })
                    .collect();
                let mut blocks = vec![
                        subheading("Diagnostics"),
                        table(
                            &["Code", "Severity", "Message", "Remediation"],
                            diagnostic_rows,
                        ),
                        subheading("Assumptions"),
                        table(
                            &["Assumption", "Owner", "Statement", "Scope"],
                            assumption_rows,
                        ),
                        subheading("User-Supplied Values"),
                        table(
                            &[
                                "Value",
                                "Category",
                                "Quantity",
                                "Source",
                                "Missing-data finding",
                            ],
                            value_rows,
                        ),
                        subheading("Provenance Notes"),
                        table(&["Source", "Location", "License", "Contributor"], note_rows),
                    ];
                if let Some(references) = input.library_references.as_ref().filter(|list| !list.is_empty()) {
                    blocks.push(subheading("Library Records"));
                    blocks.push(table(
                        &["Library kind", "Library", "Version", "Record", "Referenced by"],
                        references
                            .iter()
                            .map(|reference| {
                                vec![
                                    reference.library_kind.clone(),
                                    reference.library_id.clone(),
                                    reference.library_version.clone(),
                                    reference.record_id.clone(),
                                    reference.entity_refs.join(", "),
                                ]
                            })
                            .collect(),
                    ));
                }
                if !evidence.is_empty() {
                    blocks.push(subheading(DIAGNOSTIC_EVIDENCE_SUBHEADING));
                    for (label, diagnostic) in &evidence {
                        blocks.push(meta_paragraph(&diagnostic_evidence_heading(diagnostic, label)));
                        blocks.push(paragraph(&diagnostic.message));
                    }
                }
                ("Warnings, Assumptions, And Provenance", blocks)
            }
            "audit_manifest" => {
                let envelope_rows = calc
                    .audit_manifest_refs
                    .iter()
                    .chain(calc.report_section_refs.iter())
                    .map(|envelope| {
                        vec![
                            reference_text(&envelope.reference),
                            checksum_text(&envelope.checksum),
                        ]
                    })
                    .collect();
                (
                    "Audit Manifest",
                    vec![
                        paragraph(
                            "The SHA-256 of this rendered calculation-report document is \
computed over its exact bytes and recorded alongside the referenced envelope hashes below. The \
HTML and PDF emissions of the same assembled report sections are each hash-recorded this way. \
The rendered-document hash is reported by the generating application next to this artifact; it \
cannot appear inside the bytes it binds.",
                        ),
                        table(&["Referenced envelope", "Checksum"], envelope_rows),
                        subheading("Software, Solver And Run"),
                        table(&["Item", "Value"], run_context_rows(input)),
                    ],
                )
            }
            "rule_pack_references" => {
                let rows = calc
                    .rule_pack_refs
                    .iter()
                    .map(|rule_pack| {
                        vec![
                            rule_pack.rule_pack_id.clone(),
                            recorded(rule_pack.name.as_ref()),
                            rule_pack.version.clone(),
                            checksum_text(&rule_pack.checksum),
                            rule_pack.source_notice.clone(),
                            rule_pack.completeness_status.clone(),
                        ]
                    })
                    .collect();
                let mut blocks = Vec::new();
                if calc.rule_pack_refs.is_empty() {
                    blocks.push(paragraph("No rule pack is referenced by this report."));
                } else {
                    blocks.push(table(
                        &[
                            "Rule pack",
                            "Name",
                            "Version",
                            "Checksum",
                            "Source notice",
                            "Completeness",
                        ],
                        rows,
                    ));
                }
                blocks.push(subheading("User-Rule Check Outcomes"));
                match &input.user_rule_checks {
                    None => blocks.push(paragraph(
                        "No user-rule check run is recorded for this report's result.",
                    )),
                    Some(run) => {
                        blocks.push(meta_paragraph(&format!(
                            "Rule pack: {} · Grammar version: {} · Aggregate status: {}",
                            run.rule_pack_id,
                            run.grammar_version,
                            user_rule_check_aggregate_headline(run)
                        )));
                        blocks.push(meta_paragraph(USER_RULE_CHECK_COUNT_SCOPE_NOTE));
                        let check_rows = run
                            .checks
                            .iter()
                            .map(|check| {
                                vec![
                                    check.check_id.clone(),
                                    user_rule_check_basis_display(check),
                                    user_rule_check_status_display(&check.status),
                                    if check.missing_inputs.is_empty() {
                                        "none".to_string()
                                    } else {
                                        check.missing_inputs.join(", ")
                                    },
                                ]
                            })
                            .collect();
                        blocks.push(table(
                            &["Check", "Load case or combination", "Status", "Missing inputs"],
                            check_rows,
                        ));
                    }
                }
                ("Rule Pack References", blocks)
            }
            "limitations" => {
                let limitation_rows = disclosure
                    .limitations
                    .iter()
                    .map(|limitation| {
                        vec![
                            limitation.limitation_id.clone(),
                            limitation.statement.clone(),
                            section_reference_text(&limitation.affected_scope),
                        ]
                    })
                    .collect();
                let tbd_rows = disclosure
                    .unresolved_tbds
                    .iter()
                    .map(|tbd| {
                        (
                            tbd.tbd_id.clone(),
                            tbd.description.clone(),
                            tbd.review_needed,
                        )
                    })
                    .chain(calc.unresolved_runtime_tbds.iter().map(|tbd| {
                        (
                            tbd.tbd_id.clone(),
                            tbd.description.clone(),
                            tbd.review_needed,
                        )
                    }))
                    .map(|(tbd_id, description, review_needed)| {
                        vec![
                            tbd_id,
                            description,
                            if review_needed { "yes" } else { "no" }.to_string(),
                        ]
                    })
                    .collect();
                (
                    "Limitations",
                    vec![
                        table(&["Limitation", "Statement", "Scope"], limitation_rows),
                        subheading("Unresolved TBDs"),
                        table(&["TBD", "Description", "Review needed"], tbd_rows),
                    ],
                )
            }
            "professional_boundary_notice" => {
                let signoff_rows = [
                    "Reviewer name",
                    "Qualification / license",
                    "Date",
                    "Signature",
                    "Notes",
                ]
                .iter()
                .map(|field| vec![field.to_string(), String::new()])
                .collect();
                (
                    "Professional Boundary And Human Review",
                    vec![
                        SectionBlock::BoundaryBox {
                            text: "SWBPIPE is decision-support software for piping \
design, flexibility, and stress-analysis workflows. It computes mechanical results from \
recorded user inputs and may evaluate user-supplied rule packs. It does not certify, seal, \
approve, authenticate, or determine code compliance for professional reliance. Code-specific \
and project-specific data are supplied by the user or user-controlled private sources. \
Competent human review and, where required, validation in accepted professional tools remain \
the responsibility of the user and project authority."
                                .to_string(),
                        },
                        subheading("Human Review Record"),
                        meta_paragraph(
                            "Completed only by a qualified human reviewer. The software \
records no approval and pre-fills no field.",
                        ),
                        SectionBlock::Table {
                            header: vec!["Field".to_string(), "Entry".to_string()],
                            rows: signoff_rows,
                            signoff: true,
                        },
                    ],
                )
            }
            _ => unreachable!("section order is a fixed constant"),
        };

        assembled.push(AssembledSection { key, title, blocks });
    }

    assembled
}

/// Deterministic HTML emission of one section's blocks. Byte layout is part
/// of the canonical hash-bound artifact contract.
pub fn section_body_html(blocks: &[SectionBlock]) -> String {
    let mut html = String::new();
    for block in blocks {
        match block {
            SectionBlock::Subheading { text } => {
                html.push_str("<h3>");
                html.push_str(&escape_html(text));
                html.push_str("</h3>\n");
            }
            SectionBlock::Paragraph { text, meta } => {
                html.push_str(if *meta { "<p class=\"meta\">" } else { "<p>" });
                html.push_str(&escape_html(text));
                html.push_str("</p>\n");
            }
            SectionBlock::Table {
                header,
                rows,
                signoff,
            } => {
                html.push_str(if *signoff {
                    "<table class=\"signoff\">\n"
                } else {
                    "<table>\n"
                });
                html.push_str(&header_row(header));
                for cells in rows {
                    html.push_str(&row(cells));
                }
                html.push_str("</table>\n");
            }
            SectionBlock::BoundaryBox { text } => {
                html.push_str("<div class=\"boundary\">\n<p>");
                html.push_str(&escape_html(text));
                html.push_str("</p>\n</div>\n");
            }
        }
    }
    html
}

/// Strip tags for lint purposes: the linter scans text, so feed it the
/// human-readable content of each section (markup removed, entities kept —
/// escaped entities cannot reconstruct prohibited phrases that were not in
/// the source text).
fn html_to_lint_text(html: &str) -> String {
    let mut text = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                text.push(' ');
            }
            _ if !in_tag => text.push(ch),
            _ => {}
        }
    }
    text
}

/// Emission-neutral document meta line (report/model/run identity) shared by
/// the HTML and PDF emissions.
pub fn report_meta_text(input: &RenderableReportInput) -> String {
    format!(
        "Report ID: {} · Model: {} · Run: {}",
        input.calculation_report.report_id,
        section_reference_text(&input.report_sections.model_ref),
        section_reference_text(&input.report_sections.run_ref),
    )
}

/// Emission-neutral analysis-status banner text shared by the HTML and PDF
/// emissions. FR-RULE-003: when the recorded user-rule checks include a
/// blocked check, a recorded rule-check pass or fail is shown as incomplete,
/// with the recorded status kept beside it (a run recorded under the earlier
/// failed-first aggregate carries `user_rule_failed`).
pub fn status_banner_text(input: &RenderableReportInput) -> String {
    let blocked = input.user_rule_checks.as_ref().is_some_and(|run| {
        run.checks
            .iter()
            .any(|check| check.status == UserRuleCheckStatus::RuleInputsIncomplete)
    });
    let labels: Vec<String> = input
        .report_sections
        .analysis_status_disclosures
        .iter()
        .map(|item| match item.status {
            sections::AnalysisStatus::UserRuleFailed | sections::AnalysisStatus::UserRuleChecked
                if blocked =>
            {
                format!(
                    "{} (recorded: {})",
                    analysis_status_label(&sections::AnalysisStatus::RuleInputsIncomplete),
                    analysis_status_label(&item.status)
                )
            }
            _ => analysis_status_label(&item.status).to_string(),
        })
        .collect();
    format!("Analysis status: {}", labels.join(", "))
}

/// Emission-neutral export-blocked banner text shared by the HTML and PDF
/// emissions.
pub fn export_blocked_banner_text(blocking_reasons: &[String]) -> String {
    format!("EXPORT BLOCKED — {}", blocking_reasons.join("; "))
}

/// Emission-neutral local-private notice shared by the HTML and PDF
/// emissions. Present only when the application recorded that this document
/// was rendered under its explicit local-private export selection; it records
/// that status in the document itself (warning/audit note) and is absent from
/// public-context renders.
pub fn local_private_notice_text(input: &RenderableReportInput) -> Option<&'static str> {
    match input.report_context.as_ref().map(|context| context.export_context) {
        Some(ReportExportContext::LocalPrivate) => Some(
            "LOCAL PRIVATE REPORT — rendered after explicit local-private export selection. \
This document contains private project values for the user's own local review. It is not for \
sharing or publication.",
        ),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Readability (DEL-08-01 Design `exact-route-report-sections.md` §3.7, §6.3):
// the display rule statement, the diagnostic evidence block and the run-name
// note. Every route; the HTML and the PDF carry them alike.
// ---------------------------------------------------------------------------

/// The report's display policy (DEL-08-01 Design
/// `report-package-current-results.md` §2.3 item 5). The desktop writes it
/// (`renderableReportInput.ts` `QUANTITY_DISPLAY_POLICY`) and the report
/// package's producer requires it (`report_package::bindings`).
pub const QUANTITY_DISPLAY_POLICY: &str = "display result-row values and stress headlines with explicit units; values stored in Pa are shown in MPa (1 MPa = 10^6 Pa, exact); displayed values are rounded to six significant digits; stored values and units are unchanged";

/// The display rule, stated once at the start of the Results section when
/// the report declares [`QUANTITY_DISPLAY_POLICY`].
pub const DISPLAY_RULE_STATEMENT: &str = "Displayed values: result values and stress headlines are rounded for display to six significant digits, half away from zero, and a value stored in Pa is displayed in MPa (1 MPa = 10^6 Pa, an exact decimal shift). Stored values and units are unchanged; the stored units are listed as Result units in the Model Input Summary.";

/// The display rule statement, when the report declares the display policy
/// that applies it. A report declaring another policy states none.
pub fn display_rule_statement(input: &RenderableReportInput) -> Option<&'static str> {
    input
        .calculation_report
        .model_input_summary
        .unit_display_summary
        .as_ref()
        .filter(|summary| summary.quantity_display_policy == QUANTITY_DISPLAY_POLICY)
        .map(|_| DISPLAY_RULE_STATEMENT)
}

/// A Diagnostics message longer than this many characters (Unicode scalar
/// values) is shown in full in the diagnostic evidence block, not in the
/// table. Every message of the producer's other diagnostics on the committed
/// witnesses is shorter (the longest, 720 characters).
pub const DIAGNOSTIC_MESSAGE_TABLE_LIMIT: usize = 1000;
/// The last block of the "Warnings, Assumptions, And Provenance" section.
pub const DIAGNOSTIC_EVIDENCE_SUBHEADING: &str = "Diagnostic Evidence (full text)";

/// Whether a diagnostic's message is shown in the evidence block: every
/// numerical-integrity diagnostic (its message is the producer's structural
/// evidence record), and every message over [`DIAGNOSTIC_MESSAGE_TABLE_LIMIT`].
pub fn diagnostic_message_is_evidence(diagnostic: &sections::Diagnostic) -> bool {
    diagnostic.code.starts_with(INTEGRITY_CODE_PREFIX) || diagnostic.message.chars().count() > DIAGNOSTIC_MESSAGE_TABLE_LIMIT
}

/// The code as a phrase: lower case, words separated by spaces, the first
/// letter capitalised (`NUMERICAL_INTEGRITY_CHECKS_PASSED` → "Numerical
/// integrity checks passed").
fn code_phrase(code: &str) -> String {
    let words = code.to_lowercase().replace('_', " ");
    let mut chars = words.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// What the diagnostic affects: the load case of the producer's
/// per-load-case numerical-integrity diagnostic, otherwise its affected object.
fn diagnostic_affected(diagnostic: &sections::Diagnostic) -> String {
    if diagnostic.code.starts_with(INTEGRITY_CODE_PREFIX) {
        format!("load case {}", diagnostic.affected_object.ref_id)
    } else {
        format!("affected: {}", diagnostic.affected_object.ref_id)
    }
}

/// The Diagnostics table's Message cell for a diagnostic shown in the
/// evidence block: "<code phrase> (<load case or affected object>); full
/// text: Diagnostic evidence E<k>".
pub fn diagnostic_evidence_cell(diagnostic: &sections::Diagnostic, label: &str) -> String {
    format!(
        "{} ({}); full text: Diagnostic evidence {label}",
        code_phrase(&diagnostic.code),
        diagnostic_affected(diagnostic)
    )
}

/// The evidence entry's heading line: "E<k>: <code> (<severity>), <load case
/// or affected object>". The message follows verbatim as its own paragraph.
pub fn diagnostic_evidence_heading(diagnostic: &sections::Diagnostic, label: &str) -> String {
    format!(
        "{label}: {} ({}), {}",
        diagnostic.code,
        severity_label(&diagnostic.severity),
        diagnostic_affected(diagnostic)
    )
}

/// The run name the product producer gives every desktop solve
/// (`core/product_physics/src/lib.rs`, `run_id`). Report, result and
/// limitation identifiers derived from it repeat across desktop runs.
pub const DESKTOP_FIXED_RUN_NAME: &str = "run:preview-linear-static-001";

/// The header's run-name note (§6.3 item 2): on a report of a run named
/// [`DESKTOP_FIXED_RUN_NAME`], it states that the name is the producer's fixed
/// name and identifies this run by its received result's SHA-256, the
/// checksum of the audit reference `result_envelope`
/// `result-envelope:<run>` (on an exact-route report, the admission
/// binding's `received_result_sha256`, which the renderer checks against
/// it). `None` for any other run name.
pub fn run_name_note_text(input: &RenderableReportInput) -> Option<String> {
    let run = &input.report_sections.run_ref.ref_id;
    if run != DESKTOP_FIXED_RUN_NAME {
        return None;
    }
    let envelope = format!("result-envelope:{run}");
    let received = input
        .calculation_report
        .audit_manifest_refs
        .iter()
        .find(|entry| entry.reference.ref_type == "result_envelope" && entry.reference.ref_id == envelope)
        .map(|entry| entry.checksum.value.as_str())
        .filter(|value| is_lower_hex_64(value));
    let lead = format!(
        "Run name: {run} is the producer's fixed run name for every desktop solve, not an identity of this run; the report, result and limitation identifiers derived from it repeat across runs."
    );
    Some(match received {
        Some(sha256) => format!("{lead} This run is identified by its received result sha256 {sha256}."),
        None => format!("{lead} This report records no received result sha256 that identifies the run."),
    })
}

// ---------------------------------------------------------------------------
// W-11: the exact-route result block, its admission binding and its limitation
// entries (DEL-08-01 Design `exact-route-report-sections.md` §3.2-§3.5).
// ---------------------------------------------------------------------------

/// The admission binding disagrees with the report's own records, or a field
/// is not of its form.
pub const EXACT_ADMISSION_BINDING_MISMATCH: &str = "REPORT-EXACT-ADMISSION-BINDING-MISMATCH";
/// An exact limitation entry whose statement is not a table statement, or
/// that appears without `exact_route`.
pub const EXACT_LIMITATION_UNREGISTERED: &str = "REPORT-EXACT-LIMITATION-UNREGISTERED";
/// The headlines are not one per section basis, nominal then reduced.
pub const EXACT_HEADLINE_UNREADABLE: &str = "REPORT-EXACT-HEADLINE-UNREADABLE";
/// The two section bases, in headline order.
pub const EXACT_SECTION_BASES: [&str; 2] = ["nominal_section", "reduced_section"];
pub const EXACT_ROUTE_SUBHEADING: &str = "Exact-Route Result";
pub const EXACT_HEADLINES_SUBHEADING: &str = "Stress Headlines by Section Basis";
pub const EXACT_HEADLINES_HEADER: [&str; 4] = ["Section basis", "Stress maximum", "Location", "Source"];
/// The session's sentence (`ResultsPanel.tsx`), after the headline table.
pub const EXACT_HEADLINES_PARAGRAPH: &str = "Each section basis has its own maximum; no governing basis is chosen.";
/// A withheld headline's absent location or source.
pub const NOT_AVAILABLE: &str = "not available";
/// The profile-reference limitation entry's id suffix (§2.4).
pub const EXACT_PROFILE_REFERENCE_SUFFIX: &str = "profile-reference";

fn is_lower_hex_64(text: &str) -> bool {
    text.len() == 64 && text.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// The binding fields that disagree with the report's own records or are not
/// of their form, in a fixed order (§3.2). Empty when the binding holds.
pub fn exact_admission_mismatches(input: &RenderableReportInput, route: &ExactRouteSection) -> Vec<&'static str> {
    let binding = &route.admission;
    let calc = &input.calculation_report;
    let summary = &calc.model_input_summary;
    let mut fields = Vec::new();
    if route.semantic_contract_id != binding.semantic_contract_id {
        fields.push("semantic_contract_id");
    }
    if binding.run_ref != input.report_sections.run_ref.ref_id {
        fields.push("run_ref");
    }
    // The received-result entry of the audit references, whose reference is
    // the binding's result envelope.
    let received = calc
        .audit_manifest_refs
        .iter()
        .find(|entry| entry.reference.ref_type == "result_envelope" && entry.reference.ref_id == binding.result_envelope_id);
    match received {
        None => fields.push("result_envelope_id"),
        Some(entry) if entry.checksum.value != binding.received_result_sha256 => fields.push("received_result_sha256"),
        Some(_) => {}
    }
    if binding.model_sha256 != summary.model_hash.value {
        fields.push("model_sha256");
    }
    if binding.input_manifest_ref != summary.input_manifest_ref.ref_id {
        fields.push("input_manifest_ref");
    }
    for (field, digest) in [
        ("received_result_sha256", &binding.received_result_sha256),
        ("model_sha256", &binding.model_sha256),
        ("input_manifest_sha256", &binding.input_manifest_sha256),
        ("derivative_sha256", &binding.derivative_sha256),
        ("received_bytes_sha256", &binding.received_bytes_sha256),
    ] {
        if !is_lower_hex_64(digest) && !fields.contains(&field) {
            fields.push(field);
        }
    }
    for (field, id) in [
        ("semantic_contract_id", &binding.semantic_contract_id),
        ("run_ref", &binding.run_ref),
        ("result_envelope_id", &binding.result_envelope_id),
        ("input_manifest_ref", &binding.input_manifest_ref),
        ("native_job_id", &binding.native_job_id),
    ] {
        if id.trim().is_empty() && !fields.contains(&field) {
            fields.push(field);
        }
    }
    fields
}

/// The standing line composed from a binding that holds (§3.2). "integrity
/// checked" is the session's label (`LoadReferenceStatesBlock.tsx`). When the
/// Diagnostics carry an integrity warning, the rendered line is preceded by
/// [`exact_integrity_warning_lead`].
pub fn exact_standing_line(binding: &ExactAdmissionBinding) -> String {
    format!(
        "integrity checked (the Rust reader's standing for the result Export Result JSON admitted in this session: \
received result sha256 {}, model sha256 {}, input manifest {} sha256 {})",
        binding.received_result_sha256, binding.model_sha256, binding.input_manifest_ref, binding.input_manifest_sha256
    )
}

/// The code family of the producer's per-load-case numerical-integrity
/// diagnostic, `diagnostic:numerical-integrity:<case>` (`core/product_physics`:
/// `append_integrity_report`, `append_integrity_failure`, `formation_guard::demote`).
pub const INTEGRITY_CODE_PREFIX: &str = "NUMERICAL_INTEGRITY_";
/// The one integrity code the producer publishes at `info`: the case's
/// integrity checks passed. Every other integrity code is `warning`
/// (`NUMERICAL_INTEGRITY_SENSITIVE`, `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`)
/// or `blocking` (the failures).
pub const INTEGRITY_CHECKS_PASSED: &str = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
/// Where the standing line points a reader for an integrity warning: the
/// Diagnostics table of the "Warnings, Assumptions, And Provenance" section.
pub const EXACT_INTEGRITY_WARNING_POINTER: &str = "see the Diagnostics table under Warnings, Assumptions, And Provenance";

/// The integrity warnings among the report's Diagnostics (§3.2): each entry
/// of `report_sections.diagnostics` whose code is of the integrity family and
/// which is not `NUMERICAL_INTEGRITY_CHECKS_PASSED` at `info`. Codes in the
/// Diagnostics table's order, each once.
pub fn exact_integrity_warning_codes(input: &RenderableReportInput) -> Vec<String> {
    let mut codes: Vec<String> = Vec::new();
    for diagnostic in sections::sorted_diagnostics(&input.report_sections) {
        let passed = diagnostic.code == INTEGRITY_CHECKS_PASSED && diagnostic.severity == sections::Severity::Info;
        if diagnostic.code.starts_with(INTEGRITY_CODE_PREFIX) && !passed && !codes.contains(&diagnostic.code) {
            codes.push(diagnostic.code.clone());
        }
    }
    codes
}

/// The standing line's lead when the Diagnostics carry integrity warnings
/// (§3.2): it names the codes and points to the Diagnostics table, so the line
/// never reads only "integrity checked" beside an integrity warning. `None`
/// without one.
pub fn exact_integrity_warning_lead(codes: &[String]) -> Option<String> {
    match codes.len() {
        0 => None,
        1 => Some(format!("integrity warning present in the Diagnostics: {} ({EXACT_INTEGRITY_WARNING_POINTER}); ", codes[0])),
        _ => Some(format!("integrity warnings present in the Diagnostics: {} ({EXACT_INTEGRITY_WARNING_POINTER}); ", codes.join(", "))),
    }
}

/// The exact-route block at the start of the Results section (§3.2). A
/// binding that does not hold states no standing; the report is blocked.
fn exact_route_blocks(input: &RenderableReportInput, route: &ExactRouteSection) -> Vec<SectionBlock> {
    let mismatches = exact_admission_mismatches(input, route);
    let standing = match mismatches.first() {
        None => {
            let lead = exact_integrity_warning_lead(&exact_integrity_warning_codes(input)).unwrap_or_default();
            format!("{lead}{}", exact_standing_line(&route.admission))
        }
        Some(field) => format!("not stated ({EXACT_ADMISSION_BINDING_MISMATCH}: {field})"),
    };
    let mut blocks = vec![
        subheading(EXACT_ROUTE_SUBHEADING),
        meta_paragraph(&format!("Semantic contract: {} · Numerical standing: {standing}", route.semantic_contract_id)),
    ];
    if let Some(line) = &route.load_reference_states {
        blocks.push(paragraph(line));
    }
    blocks.push(subheading(EXACT_HEADLINES_SUBHEADING));
    blocks.push(table(
        &EXACT_HEADLINES_HEADER,
        route
            .headlines
            .iter()
            .map(|headline| {
                vec![
                    headline.label.clone(),
                    headline.value_display.clone(),
                    headline.location_ref.clone().unwrap_or_else(|| NOT_AVAILABLE.to_string()),
                    headline.source_ref.clone().unwrap_or_else(|| NOT_AVAILABLE.to_string()),
                ]
            })
            .collect(),
    ));
    blocks.push(paragraph(EXACT_HEADLINES_PARAGRAPH));
    blocks
}

/// The renderer's exact-route refusals (§3.2, §3.5), as
/// [`render_calculation_report`] applies them with the embedded table
/// ([`exact_statements::exact_statement_table`]). Each returned reason blocks
/// export. `table` is a statement table, or its refusal when malformed, in
/// which case every exact limitation statement refuses.
pub fn exact_route_blocking_reasons(
    input: &RenderableReportInput,
    table: Result<&exact_statements::ExactStatementTable, &str>,
) -> Vec<String> {
    let mut reasons = Vec::new();
    if let Some(route) = &input.exact_route {
        for field in exact_admission_mismatches(input, route) {
            reasons.push(format!("exact route: {EXACT_ADMISSION_BINDING_MISMATCH}: {field}"));
        }
        let bases: Vec<&str> = route.headlines.iter().map(|headline| headline.basis.as_str()).collect();
        if bases != EXACT_SECTION_BASES {
            reasons.push(format!(
                "exact route: {EXACT_HEADLINE_UNREADABLE}: the headlines are not one per section basis, nominal then reduced"
            ));
        }
    }
    let prefix = format!("limitation:{}:exact:", input.report_sections.run_ref.ref_id);
    for limitation in &input.report_sections.limitations {
        let id = limitation.limitation_id.as_str();
        if !(id.starts_with("limitation:") && id.contains(":exact:")) {
            continue;
        }
        let refuse = |detail: &str| format!("exact limitations: {EXACT_LIMITATION_UNREGISTERED}: {id}: {detail}");
        if input.exact_route.is_none() {
            reasons.push(refuse("an exact-route limitation entry without an exact_route section"));
            continue;
        }
        match id.strip_prefix(prefix.as_str()) {
            Some(EXACT_PROFILE_REFERENCE_SUFFIX) => {}
            Some(n) if !n.is_empty() && n.bytes().all(|byte| byte.is_ascii_digit()) && !n.starts_with('0') => {
                match table {
                    Ok(table) if table.is_statement(&limitation.statement) => {}
                    Ok(_) => reasons.push(refuse("the statement is not a reviewed exact-route statement")),
                    Err(detail) => reasons.push(refuse(detail)),
                }
            }
            _ => reasons.push(refuse("the entry id is not an exact-route limitation id of this report's run")),
        }
    }
    reasons
}

/// The code of a report provenance record whose disposition blocks export
/// (W-11 slice (c), DEL-08-01 Design `exact-route-report-sections.md` §4.3,
/// §6.2).
pub const REPORT_PROVENANCE_DISPOSITION_BLOCKED: &str = "REPORT_PROVENANCE_DISPOSITION_BLOCKED";

/// The code of a report whose provenance records are private while its export
/// context is not `local_private` (W-11 slice (c), §6.2).
pub const REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE: &str = "REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE";

/// The context the report's provenance rules are judged in: local-private only
/// when the report records `local_private` (owner ruling on #1639, "C1 b, C2
/// a", 2026-10-26); a public report, an absent context or any other is not.
pub fn provenance_context(input: &RenderableReportInput) -> sections::ProvenanceContext {
    match input.report_context.as_ref().map(|context| context.export_context) {
        Some(ReportExportContext::LocalPrivate) => sections::ProvenanceContext::LocalPrivate,
        _ => sections::ProvenanceContext::NotLocalPrivate,
    }
}

/// The non-blocking warnings a local-private report shows for its unresolved
/// (`unknown` or `TBD`) provenance records, the report sections' and the
/// library references', after the report's own diagnostics in the warnings
/// section. Empty in every other context, where they block instead, and
/// without unresolved records, so such an input renders byte-identically.
pub fn unresolved_provenance_warnings(input: &RenderableReportInput) -> Vec<sections::Diagnostic> {
    if provenance_context(input) != sections::ProvenanceContext::LocalPrivate {
        return Vec::new();
    }
    let unresolved = |item: &sections::Diagnostic| item.code == "REPORT_SECTION_PROVENANCE_UNRESOLVED" && item.severity == sections::Severity::Warning;
    let mut warnings: Vec<sections::Diagnostic> = sections::validate_report_sections_in(&input.report_sections, sections::ProvenanceContext::LocalPrivate)
        .diagnostics
        .into_iter()
        .filter(unresolved)
        .collect();
    for reference in input.library_references.iter().flatten() {
        let affected = sections::Reference::new("library_reference", format!("{} (referenced by {})", reference.reference_id, reference.entity_refs.join(", ")));
        warnings.extend(
            sections::validate_provenance_record(&reference.provenance, affected, sections::ProvenanceContext::LocalPrivate)
                .into_iter()
                .filter(unresolved),
        );
    }
    warnings
}

/// The remediation every disposition refusal names.
const DISPOSITION_REMEDIATION: &str = "review that record's disposition in the model before reporting";

/// A record is public only when both its privacy class and its redistribution
/// status are public; every other class is private record content.
fn record_is_public(privacy: sections::PrivacyClassification, redistribution: sections::RedistributionStatus) -> bool {
    matches!(privacy, sections::PrivacyClassification::PublicMetadata | sections::PrivacyClassification::InventedPublicExample)
        && matches!(
            redistribution,
            sections::RedistributionStatus::PublicPermissive | sections::RedistributionStatus::InventedNonEngineeringExample
        )
}

/// The report's provenance refusals (W-11 slice (c), §4.3, §6.2), each of
/// which blocks export:
/// - `report provenance: REPORT_PROVENANCE_DISPOSITION_BLOCKED (<where> <member>
///   <value>; ...)`: a `review_status` of `quarantined` or `rejected` (the
///   library import's and the package save's blocking values,
///   `export_disposition_screen.rs` `SCREENED_MEMBERS`) on any report-section
///   provenance record, on a user-supplied value or an assumption itself, or on
///   a library reference's record. Every record is named by its collection
///   position and the field only (`user_supplied_values[3]`,
///   `assumptions[0]`, `library_references[2].provenance`): ids, labels and
///   source text are user-authored and never enter a reason (Codex review
///   round 3 of #1639); the panel resolves a position to its entity locally;
/// - `library references: <code> (library_references[i].provenance; ...)`: a library
///   reference's record judged by the section validation's own provenance
///   rules ([`sections::validate_provenance_record`]: incomplete,
///   `protected_suspected`, `unknown` or `TBD`), as every report-section
///   record already is;
/// - `report provenance: REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE (...)`:
///   any private record (a privacy class or redistribution status that is not
///   public) when the report's export context is absent or not
///   `local_private`, so a direct native call cannot render private record
///   content outside the local-private route.
///
/// The webview's route projection makes these records opaque leaves, so the
/// renderer judges them itself, for every emission (HTML, print view, PDF and
/// the package's members). A clean input allocates nothing here.
pub fn provenance_disposition_blocking_reasons(input: &RenderableReportInput) -> Vec<String> {
    fn review_blocks(status: sections::ReviewStatus) -> Option<&'static str> {
        match status {
            sections::ReviewStatus::Quarantined => Some("quarantined"),
            sections::ReviewStatus::Rejected => Some("rejected"),
            _ => None,
        }
    }
    fn push(reasons: &mut Vec<String>, place: String, member: &str, value: &str) {
        reasons.push(format!(
            "report provenance: {REPORT_PROVENANCE_DISPOSITION_BLOCKED} ({place} {member} {value}; {DISPOSITION_REMEDIATION})"
        ));
    }
    fn judge(reasons: &mut Vec<String>, collection: &str, suffix: &str, records: &mut dyn Iterator<Item = &sections::Provenance>) {
        for (index, record) in records.enumerate() {
            if let Some(value) = review_blocks(record.review_status) {
                push(reasons, format!("{collection}[{index}]{suffix}"), "review_status", value);
            }
        }
    }
    let mut reasons = Vec::new();
    let report_sections = &input.report_sections;
    judge(&mut reasons, "diagnostics", ".provenance", &mut report_sections.diagnostics.iter().map(|item| &item.provenance));
    judge(&mut reasons, "provenance_notes", "", &mut report_sections.provenance_notes.iter());
    judge(&mut reasons, "assumptions", ".provenance", &mut report_sections.assumptions.iter().map(|item| &item.provenance));
    judge(&mut reasons, "limitations", ".provenance", &mut report_sections.limitations.iter().map(|item| &item.provenance));
    for (index, value) in report_sections.user_supplied_values.iter().enumerate() {
        if let Some(status) = review_blocks(value.provenance.review_status) {
            push(&mut reasons, format!("user_supplied_values[{index}].provenance"), "review_status", status);
        }
        if let Some(status) = review_blocks(value.review_status) {
            push(&mut reasons, format!("user_supplied_values[{index}]"), "review_status", status);
        }
    }
    for (index, assumption) in report_sections.assumptions.iter().enumerate() {
        if let Some(status) = review_blocks(assumption.review_status) {
            push(&mut reasons, format!("assumptions[{index}]"), "review_status", status);
        }
    }
    for (index, reference) in input.library_references.iter().flatten().enumerate() {
        let record = &reference.provenance;
        if let Some(value) = review_blocks(record.review_status) {
            push(&mut reasons, format!("library_references[{index}].provenance"), "review_status", value);
        }
        let affected = sections::Reference::new("library_reference", format!("library_references[{index}]"));
        for diagnostic in sections::validate_provenance_record(record, affected, provenance_context(input)) {
            if diagnostic.severity == sections::Severity::Blocking {
                reasons.push(format!(
                    "library references: {} (library_references[{index}].provenance; {DISPOSITION_REMEDIATION})",
                    diagnostic.code
                ));
            }
        }
    }
    let local_private = matches!(
        input.report_context.as_ref().map(|context| context.export_context),
        Some(ReportExportContext::LocalPrivate)
    );
    // W-18b (DEL-08-01 Design `report-user-release.md` §2.6): under
    // `user_released_report` with a record, a record whose privacy class is
    // `private_project_data` is the user-released class and does not trip
    // this rule; any other non-public class still does. Every other context
    // keeps the rule exactly.
    let user_released = user_released_record(input).is_some();
    let released_class = |privacy: sections::PrivacyClassification| user_released && privacy == sections::PrivacyClassification::PrivateProjectData;
    if !local_private {
        let private = |record: &sections::Provenance| {
            !record_is_public(record.privacy_classification, record.redistribution_status) && !released_class(record.privacy_classification)
        };
        let first = report_sections
            .provenance_notes
            .iter()
            .position(private)
            .map(|index| format!("provenance_notes[{index}]"))
            .or_else(|| {
                report_sections.user_supplied_values.iter().position(|value| {
                    private(&value.provenance)
                        || !(matches!(
                            value.privacy_classification,
                            sections::PrivacyClassification::PublicMetadata | sections::PrivacyClassification::InventedPublicExample
                        ) || released_class(value.privacy_classification))
                }).map(|index| format!("user_supplied_values[{index}]"))
            })
            .or_else(|| report_sections.diagnostics.iter().position(|item| private(&item.provenance)).map(|index| format!("diagnostics[{index}].provenance")))
            .or_else(|| report_sections.assumptions.iter().position(|item| private(&item.provenance)).map(|index| format!("assumptions[{index}].provenance")))
            .or_else(|| report_sections.limitations.iter().position(|item| private(&item.provenance)).map(|index| format!("limitations[{index}].provenance")))
            .or_else(|| input.library_references.iter().flatten().position(|item| private(&item.provenance)).map(|index| format!("library_references[{index}].provenance")));
        if let Some(place) = first {
            let context = match input.report_context.as_ref().map(|context| context.export_context) {
                Some(ReportExportContext::PublicReport) => "public_report",
                Some(ReportExportContext::UserReleasedReport) => "user_released_report",
                _ => "not recorded",
            };
            reasons.push(format!(
                "report provenance: {REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE} (private record content at {place} under export context {context}; render it only under the local-private selection)"
            ));
        }
    }
    reasons
}

/// A `user_released_report` context without a user-release record, or a
/// record in any other context (W-18b, DEL-08-01 Design
/// `report-user-release.md` §4).
pub const REPORT_CONTEXT_USER_RELEASE_MISMATCH: &str = "REPORT-CONTEXT-USER-RELEASE-MISMATCH";

/// Renderer-level consistency checks of the report context and rule-check
/// blocks. Each returned code blocks export.
fn report_context_blocking_codes(input: &RenderableReportInput) -> Vec<&'static str> {
    let mut codes = Vec::new();
    if let Some(context) = &input.report_context {
        if context.software_name.trim().is_empty() {
            codes.push("REPORT-CONTEXT-SOFTWARE-NAME-MISSING");
        }
        // W-18b (§4): a user-released report carries its record, and no other
        // context carries one.
        let user_released = context.export_context == ReportExportContext::UserReleasedReport;
        if user_released != context.user_release.is_some() {
            codes.push(REPORT_CONTEXT_USER_RELEASE_MISMATCH);
        }
    }
    if let Some(run) = &input.user_rule_checks {
        if run.rule_pack_id.trim().is_empty()
            || !input
                .calculation_report
                .rule_pack_refs
                .iter()
                .any(|rule_pack| rule_pack.rule_pack_id == run.rule_pack_id)
        {
            codes.push("USER-RULE-CHECK-RULE-PACK-UNREFERENCED");
        }
        if run.checks.iter().any(|check| check.check_id.trim().is_empty()) {
            codes.push("USER-RULE-CHECK-ID-MISSING");
        }
        // A recorded basis names a load case or combination by a non-empty id.
        if run.checks.iter().any(|check| {
            check.basis_ref.as_ref().is_some_and(|basis| {
                basis.ref_id.trim().is_empty()
                    || !matches!(basis.ref_type.as_str(), "load_case" | "combination")
            })
        }) {
            codes.push("USER-RULE-CHECK-BASIS-REF-INVALID");
        }
    }
    codes
}

/// The code of a renderer input that does not decode.
pub const RENDER_INPUT_INVALID: &str = "RENDER-INPUT-INVALID";

/// Decode a renderer input, refusing an invalid one with its JSON path and
/// the error kind only (`RENDER-INPUT-INVALID: <kind> at <path>`), never the
/// offending value or key, which may be user-authored text (Codex review
/// round 3 of #1639).
///
/// Memory: each top-level member is moved out and decoded on its own, as the
/// whole decode moves it, so the input is never copied. Before that, each
/// element of the provenance-bearing collections (the report sections'
/// diagnostics, notes, values, assumptions and limitations, and the library
/// references) is decoded by reference one at a time, which holds at most one
/// element's copy, to place an error down to a disposition member.
pub fn decode_renderable_report_input(value: serde_json::Value) -> Result<RenderableReportInput, String> {
    decode_renderable_report_input_as(value, RENDER_INPUT_INVALID, "")
}

/// The kind of a serde decode error, from its message's fixed prefix; the
/// rest of the message, which can quote the offending value or key, is never
/// used. Shared by every value-free decode refusal of the report and package
/// paths.
pub fn serde_error_kind(message: &str) -> &'static str {
    [
        "unknown variant",
        "unknown field",
        "missing field",
        "invalid type",
        "invalid value",
        "invalid length",
        "duplicate field",
    ]
    .into_iter()
    .find(|kind| message.starts_with(kind))
    .unwrap_or("invalid input")
}

/// A struct written positionally (a JSON array, which serde decodes field by
/// field in declaration order) as its members by name, so a value-free decode
/// treats both forms alike: `names` in declaration order, of which the first
/// `required` have no default. More elements than names, or fewer than
/// `required`, is an `invalid length`, as the whole decode refuses it.
pub fn positional_members(items: Vec<serde_json::Value>, names: &[&str], required: usize) -> Result<serde_json::Map<String, serde_json::Value>, &'static str> {
    if items.len() > names.len() || items.len() < required {
        return Err("invalid length");
    }
    Ok(names.iter().map(|name| name.to_string()).zip(items).collect())
}

/// [`decode_renderable_report_input`] refusing with `code` and with every
/// path under `root` (`report` for the report-package request's `report`
/// member; empty for the renderer's own input).
pub fn decode_renderable_report_input_as(value: serde_json::Value, code: &str, root: &str) -> Result<RenderableReportInput, String> {
    use serde::de::DeserializeOwned;
    use serde::Deserialize;
    use serde_json::Value;
    fn kind(error: &serde_json::Error) -> &'static str {
        serde_error_kind(&error.to_string())
    }
    let refusal = |kind: &str, path: &str| match (root.is_empty(), path) {
        (true, path) => format!("{code}: {kind} at {path}"),
        (false, "the input") => format!("{code}: {kind} at {root}"),
        (false, path) => format!("{code}: {kind} at {root}.{path}"),
    };
    fn by_ref<'a, T: Deserialize<'a>>(value: &'a Value) -> Result<(), serde_json::Error> {
        T::deserialize(value).map(drop)
    }
    /// The failing disposition member of a provenance-bearing element.
    fn member(object: &Value, path: String) -> String {
        let Some(map) = object.as_object() else { return path };
        let checks: [(&str, fn(&Value) -> bool); 4] = [
            ("provenance", |v| by_ref::<sections::Provenance>(v).is_err()),
            ("redistribution_status", |v| by_ref::<sections::RedistributionStatus>(v).is_err()),
            ("review_status", |v| by_ref::<sections::ReviewStatus>(v).is_err()),
            ("privacy_classification", |v| by_ref::<sections::PrivacyClassification>(v).is_err()),
        ];
        for (key, check) in checks {
            if let Some(child) = map.get(key) {
                if check(child) {
                    let at = format!("{path}.{key}");
                    return if key == "provenance" { member(child, at) } else { at };
                }
            }
        }
        path
    }
    fn elements<T: for<'a> Deserialize<'a>>(list: Option<&Value>, path: &str) -> Result<(), (&'static str, String)> {
        let Some(items) = list.and_then(Value::as_array) else { return Ok(()) };
        for (index, item) in items.iter().enumerate() {
            if let Err(error) = by_ref::<T>(item) {
                return Err((kind(&error), member(item, format!("{path}[{index}]"))));
            }
        }
        Ok(())
    }
    let located = |result: Result<(), (&'static str, String)>| result.map_err(|(kind, path)| refusal(kind, &path));
    // `fields` is the input object; `root` stays the path root of every refusal.
    let mut fields = match value {
        Value::Object(fields) => fields,
        Value::Array(items) => positional_members(
            items,
            &["report_title", "calculation_report", "report_sections", "result_rows", "report_context", "user_rule_checks", "exact_route", "library_references"],
            4,
        )
        .map_err(|kind| refusal(kind, "the input"))?,
        _ => return Err(refusal("invalid type", "the input")),
    };
    if let Some(Value::Object(report_sections)) = fields.get("report_sections") {
        located(elements::<sections::Diagnostic>(report_sections.get("diagnostics"), "report_sections.diagnostics"))?;
        located(elements::<sections::Provenance>(report_sections.get("provenance_notes"), "report_sections.provenance_notes"))?;
        located(elements::<sections::UserSuppliedValue>(report_sections.get("user_supplied_values"), "report_sections.user_supplied_values"))?;
        located(elements::<sections::Assumption>(report_sections.get("assumptions"), "report_sections.assumptions"))?;
        located(elements::<sections::Limitation>(report_sections.get("limitations"), "report_sections.limitations"))?;
    }
    if let Some(list @ Value::Array(_)) = fields.get("library_references") {
        located(elements::<LibraryReference>(Some(list), "library_references"))?;
    }
    fn required<T: DeserializeOwned>(root: &mut serde_json::Map<String, Value>, key: &str) -> Result<T, (&'static str, String)> {
        let Some(value) = root.remove(key) else { return Err(("missing field", key.to_string())) };
        serde_json::from_value(value).map_err(|error| (kind(&error), key.to_string()))
    }
    fn optional<T: DeserializeOwned>(root: &mut serde_json::Map<String, Value>, key: &str) -> Result<Option<T>, (&'static str, String)> {
        match root.remove(key) {
            None => Ok(None),
            Some(value) => serde_json::from_value(value).map_err(|error| (kind(&error), key.to_string())),
        }
    }
    let at = |error: (&'static str, String)| refusal(error.0, &error.1);
    // Every member, exhaustively (a new member is a compile error here); an
    // unknown top-level member is ignored, as the whole decode ignores it.
    Ok(RenderableReportInput {
        report_title: required(&mut fields, "report_title").map_err(at)?,
        calculation_report: required(&mut fields, "calculation_report").map_err(at)?,
        report_sections: required(&mut fields, "report_sections").map_err(at)?,
        result_rows: required(&mut fields, "result_rows").map_err(at)?,
        report_context: optional(&mut fields, "report_context").map_err(at)?,
        user_rule_checks: optional(&mut fields, "user_rule_checks").map_err(at)?,
        exact_route: optional(&mut fields, "exact_route").map_err(at)?,
        library_references: optional(&mut fields, "library_references").map_err(at)?,
    })
}

/// Render the calculation report to a deterministic single-file HTML
/// document and evaluate every gate. The returned outcome carries the
/// canonical SHA-256 of the HTML bytes.
pub fn render_calculation_report(input: &RenderableReportInput) -> RenderOutcome {
    let report_validation = report::validate_report(&input.calculation_report);
    // Only a local-private report relaxes unresolved provenance to a warning.
    let section_validation = sections::validate_report_sections_in(&input.report_sections, provenance_context(input));

    // W-11 slice (c) (§6.2), Codex review round 2 of #1639: a render refused
    // for its provenance records (a private record outside local_private, a
    // blocking disposition, or suspected protected content) returns a
    // metadata-only outcome, before anything is assembled: codes, reasons and
    // affected ids, and no document body, lint excerpt or section message.
    let provenance_reasons = provenance_disposition_blocking_reasons(input);
    let protected_content = section_validation.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == sections::Severity::Blocking && diagnostic.code == "REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED"
    });
    if !provenance_reasons.is_empty() || protected_content {
        let mut blocking_reasons: Vec<String> = Vec::new();
        for diagnostic in &report_validation.diagnostics {
            if diagnostic.severity == report::DiagnosticSeverity::Blocking {
                blocking_reasons.push(format!("report validation: {}", diagnostic.code));
            }
        }
        for diagnostic in &section_validation.diagnostics {
            if diagnostic.severity == sections::Severity::Blocking {
                blocking_reasons.push(format!("section validation: {}", diagnostic.code));
            }
        }
        blocking_reasons.extend(provenance_reasons);
        return RenderOutcome {
            html: String::new(),
            sha256_hex: sha256_hex(&[]),
            export_blocked: true,
            blocking_reasons,
            report_validation_diagnostics: report_validation.diagnostics.iter().map(report_outcome_diagnostic).collect(),
            section_validation_diagnostics: section_validation
                .diagnostics
                .iter()
                .map(|diagnostic| OutcomeDiagnostic {
                    code: diagnostic.code.clone(),
                    severity: severity_label(&diagnostic.severity).to_string(),
                    message: WITHHELD_SECTION_MESSAGE.to_string(),
                })
                .collect(),
            pre_render_findings: Vec::new(),
            post_render_findings: Vec::new(),
            content_withheld: true,
        };
    }

    let assembled = assemble_report_sections(input);
    let section_bodies: Vec<String> = assembled
        .iter()
        .map(|section| section_body_html(&section.blocks))
        .collect();

    // Lint gate (ii): assembled section text, pre-render.
    let provenance = lint_provenance(&input.calculation_report.provenance);
    let pre_render_targets: Vec<linter::LintTarget> = assembled
        .iter()
        .zip(section_bodies.iter())
        .map(|(section, body_html)| linter::LintTarget {
            target_id: format!("report-section-{}", section.key),
            path: format!("report_renderer://section/{}", section.key),
            surface: linter::SurfaceKind::PublicReportExample,
            text: html_to_lint_text(body_html),
            provenance: provenance.clone(),
        })
        .collect();
    let mut pre_render_config =
        linter::LintConfiguration::public_surfaces_only("report-renderer-pre-render");
    pre_render_config.public_surface_roots = vec!["report_renderer://section/".to_string()];
    let pre_render_run = linter::lint_targets(
        "report-renderer-pre-render",
        pre_render_config,
        pre_render_targets,
    );

    let mut blocking_reasons: Vec<String> = Vec::new();
    for diagnostic in &report_validation.diagnostics {
        if diagnostic.severity == report::DiagnosticSeverity::Blocking {
            blocking_reasons.push(format!("report validation: {}", diagnostic.code));
        }
    }
    for diagnostic in &section_validation.diagnostics {
        if diagnostic.severity == sections::Severity::Blocking {
            blocking_reasons.push(format!("section validation: {}", diagnostic.code));
        }
    }
    for code in report_context_blocking_codes(input) {
        blocking_reasons.push(format!("report context: {code}"));
    }
    for reason in exact_route_blocking_reasons(input, exact_statements::exact_statement_table()) {
        blocking_reasons.push(reason);
    }
    for finding in &pre_render_run.findings {
        if finding.is_blocking() {
            blocking_reasons.push(format!("pre-render lint: {:?}", finding.code));
        }
    }

    // Assemble the document body in fixed order.
    let mut body = String::new();
    body.push_str(&format!("<h1>{}</h1>\n", escape_html(&input.report_title)));
    body.push_str(&format!(
        "<p class=\"meta\">{}</p>\n",
        escape_html(&report_meta_text(input)),
    ));
    if let Some(note) = run_name_note_text(input) {
        body.push_str(&format!("<p class=\"meta\">{}</p>\n", escape_html(&note)));
    }
    body.push_str(&format!(
        "<div class=\"status-banner\">{}</div>\n",
        escape_html(&status_banner_text(input)),
    ));
    if let Some(notice) = local_private_notice_text(input) {
        body.push_str(&format!(
            "<div class=\"local-private-banner\" data-export-context=\"local_private\">{}</div>\n",
            escape_html(notice),
        ));
    }
    if let Some(notice) = user_release_notice_text(input) {
        body.push_str(&format!(
            "<div class=\"user-release-banner\" data-export-context=\"user_released_report\">{}</div>\n",
            escape_html(&notice),
        ));
    }
    if !blocking_reasons.is_empty() {
        body.push_str(&format!(
            "<div class=\"blocked-banner\">{}</div>\n",
            escape_html(&export_blocked_banner_text(&blocking_reasons)),
        ));
    }
    for (section, body_html) in assembled.iter().zip(section_bodies.iter()) {
        body.push_str(&format!("<h2>{}</h2>\n", escape_html(section.title)));
        body.push_str(body_html);
    }
    body.push_str("<p class=\"meta\">");
    body.push_str(&escape_html(
        "Printed or PDF copies of this document are derived views and are not hash-bound \
evidence; only the exact bytes of this HTML file are bound by the recorded SHA-256.",
    ));
    body.push_str("</p>\n");

    let html = document_frame(&escape_html(&input.report_title), &body);

    // Lint gate (iii): final rendered document text, post-render.
    let mut post_render_config =
        linter::LintConfiguration::public_surfaces_only("report-renderer-post-render");
    post_render_config.public_surface_roots = vec!["report_renderer://document".to_string()];
    let post_render_run = linter::lint_targets(
        "report-renderer-post-render",
        post_render_config,
        vec![linter::LintTarget {
            target_id: "rendered-report-document".to_string(),
            path: "report_renderer://document".to_string(),
            surface: linter::SurfaceKind::PublicReportExample,
            text: html_to_lint_text(&html),
            provenance,
        }],
    );
    for finding in &post_render_run.findings {
        if finding.is_blocking() {
            blocking_reasons.push(format!("post-render lint: {:?}", finding.code));
        }
    }

    let sha256_hex = sha256_hex(html.as_bytes());
    RenderOutcome {
        export_blocked: !blocking_reasons.is_empty(),
        blocking_reasons,
        report_validation_diagnostics: report_validation
            .diagnostics
            .iter()
            .map(report_outcome_diagnostic)
            .collect(),
        section_validation_diagnostics: section_validation
            .diagnostics
            .iter()
            .map(section_outcome_diagnostic)
            .collect(),
        pre_render_findings: pre_render_run
            .findings
            .iter()
            .map(outcome_finding)
            .collect(),
        post_render_findings: post_render_run
            .findings
            .iter()
            .map(outcome_finding)
            .collect(),
        html,
        sha256_hex,
        content_withheld: false,
    }
}

/// Produce the derived print/PDF view: the canonical HTML with a banner
/// naming the canonical hash. The returned document is explicitly NOT
/// hash-bound evidence; the banner exists so paper/PDF copies carry the
/// canonical artifact's identity.
pub fn derived_print_view(canonical_html: &str, canonical_sha256_hex: &str) -> String {
    let banner = format!(
        "<div class=\"derived-banner\" data-derived-view=\"true\">DERIVED VIEW — not hash-bound \
evidence. Canonical rendered-report artifact SHA-256: {}</div>\n",
        escape_html(canonical_sha256_hex)
    );
    match canonical_html.find("<body>\n") {
        Some(index) => {
            let insert_at = index + "<body>\n".len();
            let mut derived = String::with_capacity(canonical_html.len() + banner.len());
            derived.push_str(&canonical_html[..insert_at]);
            derived.push_str(&banner);
            derived.push_str(&canonical_html[insert_at..]);
            derived
        }
        None => format!("{banner}{canonical_html}"),
    }
}

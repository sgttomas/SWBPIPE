//! Deterministic full-report-package container assembly (completion-plan row
//! E3; `DEC-028` multi-member archive form, `DEC-057` naming, `DEC-061`
//! rendered-report emissions).
//!
//! This crate assembles the PRD §22.6 "full report package" as the ruled
//! `DEC-028` multi-member archive: canonical-JSON members carry the domain
//! truth, a DEL-17-02-style package manifest lists every member with
//! JCS-basis per-member SHA-256 hashes (REQ-007/011/040-043), and evidence
//! binds to the canonical members and the manifest's per-member hashes — the
//! manifest hash serves as the package identity — not to raw archive
//! container bytes. The physical container is nevertheless byte-deterministic
//! (stored, uncompressed ZIP members in fixed order with fixed DOS
//! timestamps, no varying field anywhere), so identical inputs produce an
//! identical container whose SHA-256 is also recorded. Naming follows the
//! `DEC-057` rider: extension `.opsproj`, document kind "OpenPipeStress
//! Project Package".
//!
//! Members assembled (the `DEC-061` rendered-report pair plus the E3 row's
//! named members): the canonical rendered report HTML and hash-bound
//! deterministic PDF (both from `pdf_emitter::assemble_full_report_package`,
//! two emissions of one assembled section model), the audit-manifest member
//! (DEL-08-02), one DEL-08-04 0.3.0 result-export document member per
//! supplied document, and the state/comparison/handoff report-section envelope
//! members (DEL-08-06, caller-supplied canonical-JSON records).
//!
//! Gating: the rendered-report members carry every existing HTML/PDF gate
//! (validation, pre-render section lint, post-render document lint, PDF
//! text-model lint); the audit manifest is validated with its own crate's
//! validator and each result document with [`document_form`] (route
//! admission, the complete 0.3.0 form, header and self-hash, and the audit
//! equalities the reader applies); blocking findings mark the
//! package export-blocked. A blocked package still assembles (with its
//! reasons recorded in the manifest) so it can be diagnosed; callers must
//! honor `export_blocked` before saving or exporting the artifact.
//!
//! The [`reader`] module reads a container back and accepts only the current
//! package version, refusing every other version by name with no migration
//! (`DEC-028`; owner, 2026-10-10: "Close: one version").
//!
//! The container is an artifact format, not an issuance act. This crate does
//! not write files, fetch network resources, or emit lifecycle,
//! release-readiness, professional approval, certification, sealing,
//! authentication, or code-compliance claims; human professional review is
//! required before any reliance on package contents.

#![forbid(unsafe_code)]

use open_pipe_stress_audit_manifest as audit;
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_pdf_emitter as pdf;
use open_pipe_stress_report_renderer as renderer;
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub(crate) mod admitted;
pub mod bindings;
pub mod document_form;
pub mod exact_sections;
pub mod quantity_display;
pub mod reader;
pub mod schema_eval;
pub mod user_release;
pub mod wire;
/// The kind of a serde decode error without its text (value-free refusals;
/// W-11 slice (c), Codex review round 3 of #1639).
pub use open_pipe_stress_report_renderer::serde_error_kind;
#[cfg(test)]
mod admitted_tests;
#[cfg(test)]
mod forwarding_graph_scan;
#[cfg(test)]
mod exact_package_witness_tests;

use crate::admitted::AdmittedSet;
use crate::user_release::ProducerEntry;

pub use reader::{
    read_report_package_container, PackageReadRefusal, ReadReportPackage, SUPPORTED_PACKAGE_VERSION,
};

// ---------------------------------------------------------------------------
// Ruled naming and fixed member vocabulary.
// ---------------------------------------------------------------------------

/// `DEC-057` naming rider: the `DEC-028` container extension.
pub const CONTAINER_EXTENSION: &str = "opsproj";
/// `DEC-057` naming rider: the `DEC-028` container document kind.
pub const CONTAINER_DOCUMENT_KIND: &str = "OpenPipeStress Project Package";
/// Fixed manifest member file name (always the first archive member).
pub const PACKAGE_MANIFEST_FILE_NAME: &str = "package_manifest.json";
/// Schema version of the package manifest document.
pub const PACKAGE_MANIFEST_SCHEMA_VERSION: &str = "2.0.0";
/// Artifact identifier carried by the package manifest document.
pub const PACKAGE_MANIFEST_ARTIFACT: &str = "openpipestress.full_report_package_manifest";

pub const ROLE_PACKAGE_MANIFEST: &str = "package_manifest";
pub const ROLE_REPORT_HTML: &str = "canonical_rendered_report_html";
pub const ROLE_REPORT_PDF: &str = "canonical_rendered_report_pdf";
pub const ROLE_AUDIT_MANIFEST: &str = "audit_manifest";
pub const ROLE_RESULT_EXPORT: &str = "result_export";
pub const ROLE_STATE_COMPARISON_HANDOFF: &str = "state_comparison_handoff_sections";

pub(crate) const HASH_BASIS_JCS: &str = "jcs_canonical_json_bytes";
pub(crate) const HASH_BASIS_EXACT: &str = "exact_bytes";

// ---------------------------------------------------------------------------
// Input and outcome types.
// ---------------------------------------------------------------------------

/// Package-level reference (REQ-040 source model / source basis references).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PackageRef {
    pub ref_type: String,
    pub ref_id: String,
}

impl PackageRef {
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

/// Complete assembler input. Every identity is caller-supplied and
/// deterministic: the assembler generates no IDs and records no timestamps.
#[derive(Debug, Clone, PartialEq)]
pub struct ReportPackageContainerInput<'a> {
    /// Caller-supplied deterministic package identifier (REQ-040). Also the
    /// container file-name stem after sanitization.
    pub package_id: String,
    /// Caller-supplied export profile identifier (REQ-040).
    pub export_profile_id: String,
    /// Source model reference (REQ-040).
    pub source_model_ref: PackageRef,
    /// Source-basis references, e.g. analysis runs and load bases (REQ-040).
    pub source_basis_refs: Vec<PackageRef>,
    /// The renderable report input; HTML and PDF members are emitted from its
    /// single assembled section model (`DEC-061`).
    pub report: &'a renderer::RenderableReportInput,
    /// The DEL-08-02 audit manifest carried as a canonical-JSON member.
    pub audit_manifest: &'a audit::AuditManifest,
    /// DEL-08-04 0.3.0 result-export documents, one canonical-JSON member
    /// each (package version 2.0.0; DEL-08-01 Design
    /// `report-package-current-results.md` §2.2). Each is checked by
    /// [`document_form`] before it becomes a member.
    pub result_documents: &'a [Value],
    /// DEL-08-06 state/comparison/handoff report-section records
    /// (caller-assembled), one canonical-JSON member each.
    pub state_comparison_handoff_records: &'a [Value],
}

/// One hash-recorded container member, bytes included so callers can emit
/// either the archive form or the directory form of the `DEC-028` package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContainerMember {
    pub role: String,
    pub file_name: String,
    pub media_type: String,
    /// `jcs_canonical_json_bytes` for canonical-JSON members (their exact
    /// bytes are the JCS rendering, REQ-007) or `exact_bytes` for the
    /// rendered HTML/PDF artifacts.
    pub hash_basis: String,
    pub sha256_hex: String,
    pub byte_length: usize,
    /// Whether the member itself carries canonical identity (canonical-JSON
    /// members do; the rendered artifacts rely on the manifest, REQ-043).
    pub carries_canonical_identity: bool,
    pub bytes: Vec<u8>,
}

/// The assembled deterministic report-package container. Per `DEC-028`,
/// evidence binds to the members and the manifest's per-member hashes; the
/// manifest hash is the package identity. The container bytes and hash are
/// recorded as the deterministic physical transport form.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReportPackageContainerOutcome {
    /// `{sanitized package_id}.opsproj` (`DEC-057`).
    pub container_file_name: String,
    /// Always [`CONTAINER_DOCUMENT_KIND`].
    pub document_kind: String,
    /// Package identity per `DEC-028`: SHA-256 of the manifest member bytes.
    pub package_identity_sha256_hex: String,
    /// Deterministic archive bytes (stored ZIP, fixed order, no timestamps).
    pub container_bytes: Vec<u8>,
    /// SHA-256 of the container bytes (recorded; not the evidence binding).
    pub container_sha256_hex: String,
    /// All members in archive order; the manifest member is first.
    pub members: Vec<ContainerMember>,
    pub export_blocked: bool,
    pub blocking_reasons: Vec<String>,
    /// The underlying `DEC-061` rendered-report emission outcome.
    pub report: pdf::FullReportPackageOutcome,
}

// ---------------------------------------------------------------------------
// Public entry point.
// ---------------------------------------------------------------------------

/// Assemble the full report package container. Deterministic throughout:
/// identical inputs yield byte-identical members, manifest, and container.
pub fn assemble_report_package_container(
    input: &ReportPackageContainerInput,
) -> ReportPackageContainerOutcome {
    assemble_report_package_container_in(input, &ProducerEntry::General, admitted::PRODUCTION)
}

/// The outcome of a general entry's refusal of a user-release marker (W-18b,
/// DEL-08-01 Design `report-user-release.md` §2.8, refusal 2): nothing is
/// rendered, no member is materialized and the container is empty; the
/// outcome is export-blocked with the refusal as its one reason.
fn refused_outcome(input: &ReportPackageContainerInput, reason: String) -> ReportPackageContainerOutcome {
    let empty = sha256_hex(&[]);
    let blocking_reasons = vec![reason];
    let render = renderer::RenderOutcome {
        html: String::new(),
        sha256_hex: empty.clone(),
        export_blocked: true,
        blocking_reasons: blocking_reasons.clone(),
        report_validation_diagnostics: Vec::new(),
        section_validation_diagnostics: Vec::new(),
        pre_render_findings: Vec::new(),
        post_render_findings: Vec::new(),
        content_withheld: true,
    };
    let pdf = pdf::PdfEmissionOutcome {
        pdf_bytes: Vec::new(),
        sha256_hex: empty.clone(),
        page_count: 0,
        export_blocked: true,
        blocking_reasons: blocking_reasons.clone(),
        canonical_html_sha256_hex: empty.clone(),
        text_model_findings: Vec::new(),
    };
    ReportPackageContainerOutcome {
        container_file_name: format!("{}.{}", sanitize_component(&input.package_id), CONTAINER_EXTENSION),
        document_kind: CONTAINER_DOCUMENT_KIND.to_string(),
        package_identity_sha256_hex: empty.clone(),
        container_bytes: Vec::new(),
        container_sha256_hex: empty,
        members: Vec::new(),
        export_blocked: true,
        blocking_reasons: blocking_reasons.clone(),
        report: pdf::FullReportPackageOutcome { render, pdf, export_blocked: true, blocking_reasons, members: Vec::new() },
    }
}

/// [`assemble_report_package_container`] against `admitted` (W-11, DEL-08-01
/// Design `exact-route-report-sections.md` §5.5), forwarded to its item-0
/// check, under the producer `entry` (W-18b, DEL-08-01 Design
/// `report-user-release.md` §2.8). Under [`ProducerEntry::General`] a
/// user-release marker (the user-released profile, the
/// `user_released_report` context, or a record) is refused before anything
/// is rendered. Under [`ProducerEntry::UserRelease`] the report must carry its
/// record; the package then carries no state/comparison/handoff member, and
/// the manifest carries the record with its `content_binding` and the
/// user-released boundary notes (§5.3).
pub(crate) fn assemble_report_package_container_in(
    input: &ReportPackageContainerInput,
    entry: &ProducerEntry,
    admitted: AdmittedSet,
) -> ReportPackageContainerOutcome {
    if !entry.is_user_release() {
        if let Some(marker) = user_release::container_marker(&input.export_profile_id, input.report) {
            return refused_outcome(input, user_release::general_entry_refusal(marker));
        }
    }
    let mut blocking_reasons: Vec<String> = Vec::new();
    let record = input.report.report_context.as_ref().and_then(|context| context.user_release.as_ref());
    let record_value: Option<Value> = record.map(|record| serde_json::to_value(record).expect("the typed record serializes"));
    if entry.is_user_release() && record.is_none() {
        blocking_reasons.push(format!("{}: the admitting entry packages a report with its user-release record only", user_release::PACKAGE_UNBOUND));
    }

    if input.package_id.trim().is_empty() {
        blocking_reasons.push("package: package_id must not be empty".to_string());
    }
    if input.export_profile_id.trim().is_empty() {
        blocking_reasons.push("package: export_profile_id must not be empty".to_string());
    }
    if !input.source_model_ref.is_complete() {
        blocking_reasons.push("package: source_model_ref must be complete".to_string());
    }
    for basis in &input.source_basis_refs {
        if !basis.is_complete() {
            blocking_reasons.push("package: every source_basis_ref must be complete".to_string());
        }
    }

    // Rendered-report members: both emissions of one assembled section model,
    // carrying every existing HTML/PDF gate (DEC-061).
    let report = pdf::assemble_full_report_package(input.report);
    blocking_reasons.extend(report.blocking_reasons.iter().cloned());

    // Audit-manifest member (DEL-08-02), validated by its own crate.
    let manifest_validation = audit::validate_manifest(input.audit_manifest);
    for finding in &manifest_validation.findings {
        if finding.severity == audit::ManifestFindingSeverity::Blocking {
            blocking_reasons.push(format!(
                "audit manifest {}: {:?}: {}",
                finding.subject_id, finding.code, finding.message
            ));
        }
    }
    let audit_member_value = audit_manifest_document(input.audit_manifest);

    // Result-export members (DEL-08-04 0.3.0): the shared source-independent
    // checks (route admission, the complete 0.3.0 form, the transported header
    // and the self-hash) and the audit equalities the reader also applies.
    if input.result_documents.is_empty() {
        blocking_reasons.push("package: at least one result export member is required".to_string());
    }
    let audit_model_hash = input.audit_manifest.model_hash.as_ref().map(|hash| hash.value.as_str());
    let mut result_members: Vec<(String, &Value)> = Vec::new();
    for document in input.result_documents {
        let envelope_id = document
            .pointer("/result_envelope/envelope_id")
            .and_then(Value::as_str)
            .unwrap_or("");
        for check in [
            document_form::check_route_admitted_in(document, admitted),
            document_form::check_result_member_form(document),
            document_form::check_audit_equalities(document, &input.audit_manifest.manifest_id, audit_model_hash),
            document_form::check_document_subjects(document),
        ] {
            if let Err(reason) = check {
                blocking_reasons.push(format!("result export {envelope_id}: {reason}"));
            }
        }
        let file_name = format!("result_export_{}.json", sanitize_component(envelope_id));
        result_members.push((file_name, document));
    }

    // State/comparison/handoff report-section members (DEL-08-06). A
    // user-released package carries none (W-18b §2.1, §2.8).
    if record_value.is_some() {
        if !input.state_comparison_handoff_records.is_empty() {
            blocking_reasons.push(format!(
                "{}: a user-released package carries no state/comparison/handoff report-section member",
                user_release::PACKAGE_UNBOUND
            ));
        }
    } else if input.state_comparison_handoff_records.is_empty() {
        blocking_reasons.push(
            "package: at least one state/comparison/handoff report-section member is required"
                .to_string(),
        );
    }
    let mut section_members: Vec<(String, &Value)> = Vec::new();
    for record in input.state_comparison_handoff_records {
        let set_id = record
            .get("section_set_id")
            .and_then(Value::as_str)
            .unwrap_or("");
        if record.get("deliverable_id").and_then(Value::as_str) != Some("DEL-08-06") {
            blocking_reasons.push(format!(
                "state/comparison/handoff record {set_id}: must carry the DEL-08-06 envelope \
                 (deliverable_id)"
            ));
        }
        if set_id.trim().is_empty() {
            blocking_reasons.push(
                "state/comparison/handoff record: section_set_id must not be empty".to_string(),
            );
        }
        if let Some(diagnostics) = record.get("diagnostics").and_then(Value::as_array) {
            for diagnostic in diagnostics {
                if diagnostic.get("severity").and_then(Value::as_str) == Some("blocking") {
                    blocking_reasons.push(format!(
                        "state/comparison/handoff record {set_id}: blocking diagnostic {}",
                        diagnostic
                            .get("code")
                            .and_then(Value::as_str)
                            .unwrap_or("UNKNOWN")
                    ));
                }
            }
        }
        if json_nesting_depth(record) > MAX_MEMBER_JSON_NESTING {
            blocking_reasons.push(format!(
                "state/comparison/handoff record {set_id}: nesting deeper than \
                 {MAX_MEMBER_JSON_NESTING} levels is outside the package form"
            ));
        }
        let file_name = format!(
            "state_comparison_handoff_sections_{}.json",
            sanitize_component(set_id)
        );
        section_members.push((file_name, record));
    }

    // Materialize the non-manifest members in fixed archive order.
    let mut members: Vec<ContainerMember> = Vec::new();
    members.push(member_exact(
        ROLE_REPORT_HTML,
        "calculation_report.html",
        "text/html",
        report.render.html.as_bytes().to_vec(),
    ));
    members.push(member_exact(
        ROLE_REPORT_PDF,
        "calculation_report.pdf",
        "application/pdf",
        report.pdf.pdf_bytes.clone(),
    ));
    members.push(member_canonical_json(
        ROLE_AUDIT_MANIFEST,
        "audit_manifest.json",
        &audit_member_value,
    ));
    for (file_name, document) in &result_members {
        members.push(member_canonical_json(
            ROLE_RESULT_EXPORT,
            file_name,
            document,
        ));
    }
    for (file_name, record) in &section_members {
        members.push(member_canonical_json(
            ROLE_STATE_COMPARISON_HANDOFF,
            file_name,
            record,
        ));
    }

    // Duplicate member names would make the inventory ambiguous (REQ-011).
    for (index, member) in members.iter().enumerate() {
        if members
            .iter()
            .skip(index + 1)
            .any(|other| other.file_name == member.file_name)
        {
            blocking_reasons.push(format!(
                "package: duplicate member file name {}",
                member.file_name
            ));
        }
    }

    // The stored-ZIP archive form has 16-bit name lengths and member counts
    // and 32-bit sizes and offsets (no ZIP64). A package outside those limits
    // cannot be encoded, so it is blocked; the reader refuses its bytes.
    blocking_reasons.extend(archive_form_limit_reasons(&members));

    let mut export_blocked = !blocking_reasons.is_empty();

    // Package manifest member (DEL-17-02 REQ-007/011/040-043): member
    // inventory with per-member JCS-basis hashes, source references,
    // diagnostics, field status, and sidecar identification.
    let manifest_for = |export_blocked: bool, blocking_reasons: &[String]| {
        member_canonical_json(
            ROLE_PACKAGE_MANIFEST,
            PACKAGE_MANIFEST_FILE_NAME,
            &package_manifest_document(
                &ManifestFields {
                    package_id: &input.package_id,
                    export_profile_id: &input.export_profile_id,
                    source_model_ref: &input.source_model_ref,
                    source_basis_refs: &input.source_basis_refs,
                    export_blocked,
                    blocking_reasons,
                    user_release: record_value.as_ref(),
                },
                &members,
            ),
        )
    };
    let mut manifest_member = manifest_for(export_blocked, &blocking_reasons);
    // The exact archive length, manifest included, must fit the archive
    // form's 32-bit offsets. A blocked rebuild only grows the manifest, and a
    // blocked oversize package is refused by the reader in any case.
    if archive_length(&manifest_member, &members) > u64::from(u32::MAX) {
        blocking_reasons
            .push("package: the archive exceeds the archive form's 32-bit size limit".to_string());
        export_blocked = true;
        manifest_member = manifest_for(export_blocked, &blocking_reasons);
    }
    let package_identity_sha256_hex = manifest_member.sha256_hex.clone();
    members.insert(0, manifest_member);

    let container_bytes = build_deterministic_zip(&members);
    let container_sha256_hex = sha256_hex(&container_bytes);

    ReportPackageContainerOutcome {
        container_file_name: format!(
            "{}.{}",
            sanitize_component(&input.package_id),
            CONTAINER_EXTENSION
        ),
        document_kind: CONTAINER_DOCUMENT_KIND.to_string(),
        package_identity_sha256_hex,
        container_bytes,
        container_sha256_hex,
        members,
        export_blocked,
        blocking_reasons,
        report,
    }
}

// ---------------------------------------------------------------------------
// Member materialization.
// ---------------------------------------------------------------------------

fn archive_form_limit_reasons(members: &[ContainerMember]) -> Vec<String> {
    let mut reasons = Vec::new();
    for member in members {
        if member.file_name.len() > usize::from(u16::MAX) {
            reasons.push(format!(
                "package: member file name of {} bytes exceeds the archive form's 65535-byte limit",
                member.file_name.len()
            ));
        }
    }
    // One manifest member is added later; 0xFFFF is the ZIP64 sentinel.
    if members.len() + 1 >= usize::from(u16::MAX) {
        reasons.push(format!(
            "package: {} members exceed the archive form's member-count limit",
            members.len() + 1
        ));
    }
    reasons
}

/// Exact length of the stored-ZIP archive for the manifest plus `members`:
/// local entries, central entries and the end-of-central-directory record.
fn archive_length(manifest: &ContainerMember, members: &[ContainerMember]) -> u64 {
    std::iter::once(manifest)
        .chain(members)
        .map(|member| {
            let name = member.file_name.len() as u64;
            (30 + name + member.bytes.len() as u64) + (46 + name)
        })
        .sum::<u64>()
        + 22
}

/// Deepest array/object nesting a canonical-JSON member may have: the depth
/// the package reader can parse back (serde_json's recursion limit).
pub const MAX_MEMBER_JSON_NESTING: usize = 127;

/// Array/object nesting depth of a JSON value (a scalar is 0). Iterative, so
/// a deep caller-supplied record cannot exhaust the stack here.
fn json_nesting_depth(value: &Value) -> usize {
    let mut deepest = 0;
    let mut pending = vec![(value, 0usize)];
    while let Some((value, depth)) = pending.pop() {
        let children: Vec<&Value> = match value {
            Value::Array(items) => items.iter().collect(),
            Value::Object(map) => map.values().collect(),
            _ => continue,
        };
        deepest = deepest.max(depth + 1);
        pending.extend(children.into_iter().map(|child| (child, depth + 1)));
    }
    deepest
}

fn member_exact(role: &str, file_name: &str, media_type: &str, bytes: Vec<u8>) -> ContainerMember {
    ContainerMember {
        role: role.to_string(),
        file_name: file_name.to_string(),
        media_type: media_type.to_string(),
        hash_basis: HASH_BASIS_EXACT.to_string(),
        sha256_hex: sha256_hex(&bytes),
        byte_length: bytes.len(),
        carries_canonical_identity: false,
        bytes,
    }
}

/// A canonical-JSON member: the member bytes ARE the RFC 8785 (JCS) rendering
/// of the document, so the recorded hash is simultaneously the exact-bytes
/// hash and the JCS-basis hash (REQ-007).
fn member_canonical_json(role: &str, file_name: &str, document: &Value) -> ContainerMember {
    let bytes = canonical_json(document).into_bytes();
    ContainerMember {
        role: role.to_string(),
        file_name: file_name.to_string(),
        media_type: "application/json".to_string(),
        hash_basis: HASH_BASIS_JCS.to_string(),
        sha256_hex: sha256_hex(&bytes),
        byte_length: bytes.len(),
        carries_canonical_identity: true,
        bytes,
    }
}

/// Deterministic file-name component: ASCII alphanumerics, `-`, and `_` kept
/// (lowercased); every other character becomes `-`; empty input becomes
/// `member`.
pub(crate) fn sanitize_component(text: &str) -> String {
    let sanitized: String = text
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    if sanitized.is_empty() {
        "member".to_string()
    } else {
        sanitized
    }
}

// ---------------------------------------------------------------------------
// Audit-manifest member document (DEL-08-02 data as canonical JSON).
// ---------------------------------------------------------------------------

fn audit_manifest_document(manifest: &audit::AuditManifest) -> Value {
    json!({
        "artifact": "openpipestress.audit_manifest",
        "deliverable_id": "DEL-08-02",
        "package_id": "PKG-08",
        "manifest_id": manifest.manifest_id,
        "model_hash": optional_hash_record_json(&manifest.model_hash),
        "input_manifest_hash": optional_hash_record_json(&manifest.input_manifest_hash),
        "solver_version": {
            "solver_name": manifest.solver_version.solver_name,
            "solver_version": manifest.solver_version.solver_version,
            "solver_build_ref": manifest.solver_version.solver_build_ref
        },
        "unit_system_ref": manifest.unit_system_ref,
        "rule_pack_refs": manifest
            .rule_pack_refs
            .iter()
            .map(rule_pack_audit_ref_json)
            .collect::<Vec<_>>(),
        "assets": manifest.assets.iter().map(asset_entry_json).collect::<Vec<_>>(),
        "professional_boundary": {
            "software_makes_compliance_claim": manifest.professional_boundary.software_makes_compliance_claim,
            "software_makes_certification_claim": manifest.professional_boundary.software_makes_certification_claim,
            "software_makes_sealing_claim": manifest.professional_boundary.software_makes_sealing_claim,
            "software_makes_approval_claim": manifest.professional_boundary.software_makes_approval_claim,
            "human_review_required": manifest.professional_boundary.human_review_required
        }
    })
}

fn optional_hash_record_json(record: &Option<audit::HashRecord>) -> Value {
    match record {
        Some(record) => hash_record_json(record),
        None => Value::Null,
    }
}

fn hash_record_json(record: &audit::HashRecord) -> Value {
    json!({
        "algorithm": match record.algorithm {
            audit::HashAlgorithm::Sha256 => "sha256",
        },
        "canonicalization": match record.canonicalization {
            audit::Canonicalization::ProjectLocalDeterministicJson => {
                "project_local_deterministic_json"
            }
            audit::Canonicalization::None => "none",
            audit::Canonicalization::Rfc8785Jcs => "rfc8785_jcs",
            audit::Canonicalization::RulePackPayloadJcs => "rule_pack_payload_jcs",
        },
        "payload_kind": payload_kind_json(record.payload_kind),
        "payload_ref": record.payload_ref,
        "value": record.value
    })
}

fn payload_kind_json(kind: audit::PayloadKind) -> &'static str {
    match kind {
        audit::PayloadKind::ModelJson => "model_json",
        audit::PayloadKind::InputManifestJson => "input_manifest_json",
        audit::PayloadKind::RulePackReference => "rule_pack_reference",
        audit::PayloadKind::BinaryAsset => "binary_asset",
        audit::PayloadKind::ExternalArtifact => "external_artifact",
    }
}

fn privacy_class_json(class: audit::PrivacyClass) -> &'static str {
    match class {
        audit::PrivacyClass::PublicMetadata => "public_metadata",
        audit::PrivacyClass::PrivateProjectData => "private_project_data",
        audit::PrivacyClass::PrivateRulePackData => "private_rule_pack_data",
        audit::PrivacyClass::Redacted => "redacted",
        audit::PrivacyClass::Tbd => "TBD",
    }
}

fn redistribution_status_json(status: audit::RedistributionStatus) -> &'static str {
    match status {
        audit::RedistributionStatus::PublicPermissive => "public_permissive",
        audit::RedistributionStatus::PrivateOnly => "private_only",
        audit::RedistributionStatus::Unknown => "unknown",
        audit::RedistributionStatus::ProtectedSuspected => "protected_suspected",
        audit::RedistributionStatus::InventedNonEngineeringExample => {
            "invented_non_engineering_example"
        }
        audit::RedistributionStatus::Tbd => "TBD",
    }
}

fn rule_pack_audit_ref_json(rule_pack: &audit::RulePackAuditRef) -> Value {
    json!({
        "rule_pack_id": rule_pack.rule_pack_id,
        "rule_pack_version": rule_pack.rule_pack_version,
        "source_notice": rule_pack.source_notice,
        "redistribution_status": redistribution_status_json(rule_pack.redistribution_status),
        "checksum": optional_hash_record_json(&rule_pack.checksum),
        "private_payload_redacted": rule_pack.private_payload_redacted
    })
}

fn asset_entry_json(asset: &audit::AssetManifestEntry) -> Value {
    json!({
        "asset_id": asset.asset_id,
        "payload_kind": payload_kind_json(asset.payload_kind),
        "privacy_class": privacy_class_json(asset.privacy_class),
        "hash": optional_hash_record_json(&asset.hash),
        "provenance": asset.provenance
    })
}

// ---------------------------------------------------------------------------
// Package manifest document (DEL-17-02 REQ-007/011/040-043).
// ---------------------------------------------------------------------------

/// The variable fields of the package manifest; every other manifest field
/// is fixed by the package version. The reader rebuilds the manifest from
/// these fields and requires the stored manifest to equal the rebuild.
pub(crate) struct ManifestFields<'a> {
    pub(crate) package_id: &'a str,
    pub(crate) export_profile_id: &'a str,
    pub(crate) source_model_ref: &'a PackageRef,
    pub(crate) source_basis_refs: &'a [PackageRef],
    pub(crate) export_blocked: bool,
    pub(crate) blocking_reasons: &'a [String],
    /// The caller's user-release record (W-18b §2.3), without
    /// `content_binding`: the manifest writes it with the binding over
    /// `members`. `None` for every other package.
    pub(crate) user_release: Option<&'a Value>,
}

/// The package manifest's boundary notes: the three of every package, or,
/// with a user-release record, the user-released set (W-18b §5.3).
fn boundary_notes(user_released: bool) -> Value {
    if user_released {
        json!([
            "This container is an artifact format, not an issuance act; package contents are engineering decision-support information.",
            "Human professional review is required before any reliance on package contents.",
            "No timestamps or generated identifiers are recorded in this package except user_release.user_released_at, the time of the user release; identical inputs, including that time, produce identical bytes.",
            "The user released their own project data in this package for sharing (user_release). The record is the user's statement about rights in that data; it is not an issuance act."
        ])
    } else {
        json!([
            "This container is an artifact format, not an issuance act; package contents are \
             engineering decision-support information.",
            "Human professional review is required before any reliance on package contents.",
            "No timestamps or generated identifiers are recorded anywhere in this package; \
             identical inputs produce identical bytes."
        ])
    }
}

pub(crate) fn package_manifest_document(
    fields: &ManifestFields,
    members: &[ContainerMember],
) -> Value {
    let member_rows: Vec<Value> = members
        .iter()
        .map(|member| {
            json!({
                "role": member.role,
                "file_name": member.file_name,
                "media_type": member.media_type,
                "hash_basis": member.hash_basis,
                "sha256_hex": member.sha256_hex,
                "byte_length": member.byte_length,
                "carries_canonical_identity": member.carries_canonical_identity
            })
        })
        .collect();

    let mut document = json!({
        "artifact": PACKAGE_MANIFEST_ARTIFACT,
        "schema_version": PACKAGE_MANIFEST_SCHEMA_VERSION,
        "document_kind": CONTAINER_DOCUMENT_KIND,
        "container_extension": CONTAINER_EXTENSION,
        "decision_basis": ["DEC-028", "DEC-057", "DEC-061"],
        "package_id": fields.package_id,
        "export_profile_id": fields.export_profile_id,
        "source_model_ref": package_ref_json(fields.source_model_ref),
        "source_basis_refs": fields
            .source_basis_refs
            .iter()
            .map(package_ref_json)
            .collect::<Vec<_>>(),
        "package_identity": {
            "basis": "sha256_of_this_manifest_member_canonical_bytes",
            "note": "Evidence binds to the canonical members and this manifest's per-member \
                     hashes, not to raw archive container bytes (DEC-028)."
        },
        "manifest_self": {
            "role": ROLE_PACKAGE_MANIFEST,
            "file_name": PACKAGE_MANIFEST_FILE_NAME,
            "media_type": "application/json",
            "hash_basis": HASH_BASIS_JCS,
            "hash_note": "This manifest cannot record its own hash; its SHA-256 over these \
                          exact canonical bytes is the package identity."
        },
        "members": member_rows,
        "sidecars": {
            "id_map_members": [],
            "loss_report_members": [],
            "note": "No sidecar ID-map or loss-report members are required: canonical-JSON \
                     members carry canonical identity directly, and identity and hashes for \
                     the rendered HTML and PDF artifacts are carried by this manifest."
        },
        "target_field_status": {
            "member_inventory": "source_confirmed",
            "per_member_hashes": "source_confirmed",
            "source_model_ref": "source_confirmed",
            "source_basis_refs": "source_confirmed",
            "rendered_report_artifacts": "source_confirmed",
            "external_adapter_formats": "unsupported"
        },
        "diagnostics": {
            "export_blocked": fields.export_blocked,
            "blocking_reasons": fields.blocking_reasons
        },
        "boundary_notes": boundary_notes(fields.user_release.is_some()),
        "professional_boundary": {
            "human_review_required": true,
            "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false,
            "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false,
            "software_makes_authentication_claim": false
        }
    });
    // W-18b §2.3: the caller's record plus the binding of every other member,
    // written only when present.
    if let Some(record) = fields.user_release {
        let mut record = record.clone();
        if let Value::Object(object) = &mut record {
            object.insert("content_binding".to_string(), user_release::content_binding(members));
        }
        document["user_release"] = record;
    }
    document
}

fn package_ref_json(reference: &PackageRef) -> Value {
    json!({
        "ref_type": reference.ref_type,
        "ref_id": reference.ref_id
    })
}

// ---------------------------------------------------------------------------
// Deterministic archive assembly (stored ZIP, fixed order, fixed timestamps).
// ---------------------------------------------------------------------------

/// Fixed DOS date for every archive member: 1980-01-01, 00:00:00 (the ZIP
/// epoch — a constant, not a recorded time).
const ZIP_FIXED_DOS_TIME: u16 = 0;
const ZIP_FIXED_DOS_DATE: u16 = 0x0021;
const ZIP_VERSION: u16 = 20;

/// Build the deterministic archive: stored (uncompressed) members in the
/// given fixed order, fixed DOS timestamps, no extra fields, no comments,
/// ASCII member names. Identical members yield identical archive bytes.
pub(crate) fn build_deterministic_zip(members: &[ContainerMember]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut central: Vec<u8> = Vec::new();
    let mut entry_count: u16 = 0;

    for member in members {
        let name = member.file_name.as_bytes();
        let crc = crc32(&member.bytes);
        let size = member.bytes.len() as u32;
        let offset = out.len() as u32;

        // Local file header.
        push_u32(&mut out, 0x0403_4b50);
        push_u16(&mut out, ZIP_VERSION);
        push_u16(&mut out, 0); // general-purpose flags
        push_u16(&mut out, 0); // method: stored
        push_u16(&mut out, ZIP_FIXED_DOS_TIME);
        push_u16(&mut out, ZIP_FIXED_DOS_DATE);
        push_u32(&mut out, crc);
        push_u32(&mut out, size);
        push_u32(&mut out, size);
        push_u16(&mut out, name.len() as u16);
        push_u16(&mut out, 0); // extra length
        out.extend_from_slice(name);
        out.extend_from_slice(&member.bytes);

        // Central directory entry.
        push_u32(&mut central, 0x0201_4b50);
        push_u16(&mut central, ZIP_VERSION); // version made by
        push_u16(&mut central, ZIP_VERSION); // version needed
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u16(&mut central, ZIP_FIXED_DOS_TIME);
        push_u16(&mut central, ZIP_FIXED_DOS_DATE);
        push_u32(&mut central, crc);
        push_u32(&mut central, size);
        push_u32(&mut central, size);
        push_u16(&mut central, name.len() as u16);
        push_u16(&mut central, 0); // extra length
        push_u16(&mut central, 0); // comment length
        push_u16(&mut central, 0); // disk number
        push_u16(&mut central, 0); // internal attributes
        push_u32(&mut central, 0); // external attributes
        push_u32(&mut central, offset);
        central.extend_from_slice(name);

        // Wrapping: a package over the member-count limit is blocked, not a panic.
        entry_count = entry_count.wrapping_add(1);
    }

    let central_offset = out.len() as u32;
    let central_size = central.len() as u32;
    out.extend_from_slice(&central);

    // End of central directory.
    push_u32(&mut out, 0x0605_4b50);
    push_u16(&mut out, 0); // this disk
    push_u16(&mut out, 0); // central-directory disk
    push_u16(&mut out, entry_count);
    push_u16(&mut out, entry_count);
    push_u32(&mut out, central_size);
    push_u32(&mut out, central_offset);
    push_u16(&mut out, 0); // comment length
    out
}

fn push_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

/// CRC-32 (IEEE 802.3, reflected polynomial 0xEDB88320), computed bitwise:
/// deterministic and dependency-free.
pub(crate) fn crc32(bytes: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(64);
    for byte in digest {
        hex.push_str(&format!("{byte:02x}"));
    }
    hex
}

#[cfg(test)]
mod archive_length_tests {
    use super::*;

    #[test]
    fn archive_length_is_the_encoded_length() {
        let members: Vec<ContainerMember> =
            [("a.json", 0usize), ("bb.json", 7), ("ccc.html", 4096)]
                .iter()
                .map(|(name, size)| member_exact("r", name, "text/plain", vec![b'x'; *size]))
                .collect();
        let manifest = member_exact(
            "m",
            PACKAGE_MANIFEST_FILE_NAME,
            "application/json",
            vec![b'{'; 99],
        );
        let mut all = vec![manifest.clone()];
        all.extend(members.iter().cloned());
        assert_eq!(
            archive_length(&manifest, &members),
            build_deterministic_zip(&all).len() as u64
        );
    }
}

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

use open_pipe_stress_protected_content_linter as linter;
use open_pipe_stress_report_generator as report;
use open_pipe_stress_report_sections as sections;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

/// Complete renderer input document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderableReportInput {
    pub report_title: String,
    pub calculation_report: report::CalculationReport,
    pub report_sections: sections::ReportSections,
    pub result_rows: Vec<ResultRow>,
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

/// Render result. `html` is always produced (a blocked document renders with
/// a visible blocked banner for diagnosis); `export_blocked` is the gate the
/// app must honor before saving, printing, or exporting the artifact.
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
}

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
                let diagnostic_rows = sections::sorted_diagnostics(disclosure)
                    .iter()
                    .map(|diagnostic| {
                        vec![
                            diagnostic.code.clone(),
                            severity_label(&diagnostic.severity).to_string(),
                            diagnostic.message.clone(),
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
                (
                    "Warnings, Assumptions, And Provenance",
                    vec![
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
                    ],
                )
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
                            rule_pack.version.clone(),
                            checksum_text(&rule_pack.checksum),
                            rule_pack.source_notice.clone(),
                            rule_pack.completeness_status.clone(),
                        ]
                    })
                    .collect();
                (
                    "Rule Pack References",
                    vec![table(
                        &[
                            "Rule pack",
                            "Version",
                            "Checksum",
                            "Source notice",
                            "Completeness",
                        ],
                        rows,
                    )],
                )
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
                            text: "This document is a software-generated calculation record. \
It is not a professional engineering approval, certification, seal, authentication, or \
code-compliance determination. Human review by a qualified engineer is required before any \
reliance."
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
/// emissions.
pub fn status_banner_text(disclosure: &sections::ReportSections) -> String {
    let labels: Vec<&'static str> = disclosure
        .analysis_status_disclosures
        .iter()
        .map(|item| analysis_status_label(&item.status))
        .collect();
    format!("Analysis status: {}", labels.join(", "))
}

/// Emission-neutral export-blocked banner text shared by the HTML and PDF
/// emissions.
pub fn export_blocked_banner_text(blocking_reasons: &[String]) -> String {
    format!("EXPORT BLOCKED — {}", blocking_reasons.join("; "))
}

/// Render the calculation report to a deterministic single-file HTML
/// document and evaluate every gate. The returned outcome carries the
/// canonical SHA-256 of the HTML bytes.
pub fn render_calculation_report(input: &RenderableReportInput) -> RenderOutcome {
    let report_validation = report::validate_report(&input.calculation_report);
    let section_validation = sections::validate_report_sections(&input.report_sections);

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
    let pre_render_run = linter::lint_targets(
        "report-renderer-pre-render",
        linter::LintConfiguration::public_surfaces_only("report-renderer-pre-render"),
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
    body.push_str(&format!(
        "<div class=\"status-banner\">{}</div>\n",
        escape_html(&status_banner_text(&input.report_sections)),
    ));
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
    let post_render_run = linter::lint_targets(
        "report-renderer-post-render",
        linter::LintConfiguration::public_surfaces_only("report-renderer-post-render"),
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

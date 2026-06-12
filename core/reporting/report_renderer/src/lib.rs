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
        html.push_str(cell);
        html.push_str("</td>");
    }
    html.push_str("</tr>\n");
    html
}

fn header_row(cells: &[&str]) -> String {
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

fn lint_provenance(provenance: &report::Provenance) -> linter::Provenance {
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

struct AssembledSection {
    key: &'static str,
    title: &'static str,
    body_html: String,
    lint_text: String,
}

fn assemble_sections(input: &RenderableReportInput) -> Vec<AssembledSection> {
    let calc = &input.calculation_report;
    let disclosure = &input.report_sections;
    let mut assembled = Vec::new();

    for key in SECTION_ORDER {
        let (title, body_html) = match key {
            "model_input_summary" => {
                let summary = &calc.model_input_summary;
                let mut body = String::from("<table>\n");
                body.push_str(&header_row(&["Item", "Reference"]));
                body.push_str(&row(&[
                    escape_html("Project"),
                    escape_html(&reference_text(&summary.project_ref)),
                ]));
                body.push_str(&row(&[
                    escape_html("Model"),
                    escape_html(&reference_text(&summary.model_ref)),
                ]));
                body.push_str(&row(&[
                    escape_html("Persistence envelope"),
                    escape_html(&reference_text(&summary.persistence_ref)),
                ]));
                body.push_str(&row(&[
                    escape_html("Unit system"),
                    escape_html(&reference_text(&summary.unit_system_ref)),
                ]));
                if let Some(unit_display) = &summary.unit_display_summary {
                    body.push_str(&row(&[
                        escape_html("Unit storage convention"),
                        escape_html(&unit_display.storage_convention),
                    ]));
                    body.push_str(&row(&[
                        escape_html("Model units"),
                        escape_html(&model_units_text(&unit_display.model_units)),
                    ]));
                    body.push_str(&row(&[
                        escape_html("Result units"),
                        escape_html(&result_units_text(&unit_display.result_units)),
                    ]));
                    body.push_str(&row(&[
                        escape_html("Quantity display policy"),
                        escape_html(&unit_display.quantity_display_policy),
                    ]));
                    body.push_str(&row(&[
                        escape_html("Report-time conversion"),
                        escape_html(if unit_display.conversion_performed {
                            "true"
                        } else {
                            "false"
                        }),
                    ]));
                }
                body.push_str(&row(&[
                    escape_html("Model hash"),
                    escape_html(&checksum_text(&summary.model_hash)),
                ]));
                body.push_str(&row(&[
                    escape_html("Input manifest"),
                    escape_html(&reference_text(&summary.input_manifest_ref)),
                ]));
                body.push_str("</table>\n");
                ("Model Input Summary", body)
            }
            "load_cases" => {
                let mut body = String::from("<table>\n");
                body.push_str(&header_row(&["Load case", "Label", "Basis", "Source"]));
                for case in &calc.load_case_summary {
                    body.push_str(&row(&[
                        escape_html(&reference_text(&case.load_ref)),
                        escape_html(&case.label),
                        escape_html(&case.basis),
                        escape_html(&reference_text(&case.source)),
                    ]));
                }
                body.push_str("</table>\n");
                ("Load Cases", body)
            }
            "results" => {
                let mut body = String::new();
                if input.result_rows.is_empty() {
                    body.push_str("<p>No result rows supplied for this rendering.</p>\n");
                } else {
                    body.push_str("<table>\n");
                    body.push_str(&header_row(&["Result", "Case", "Value", "Source"]));
                    for result_row in &input.result_rows {
                        body.push_str(&row(&[
                            escape_html(&result_row.label),
                            escape_html(&result_row.case_ref),
                            escape_html(&result_row.quantity_display),
                            escape_html(&result_row.source_ref),
                        ]));
                    }
                    body.push_str("</table>\n");
                }
                body.push_str("<table>\n");
                body.push_str(&header_row(&["Referenced result export", "Checksum"]));
                for envelope in &calc.result_export_refs {
                    body.push_str(&row(&[
                        escape_html(&reference_text(&envelope.reference)),
                        escape_html(&checksum_text(&envelope.checksum)),
                    ]));
                }
                body.push_str("</table>\n");
                ("Results", body)
            }
            "warnings_assumptions_provenance" => {
                let mut body = String::from("<h3>Diagnostics</h3>\n<table>\n");
                body.push_str(&header_row(&["Code", "Severity", "Message", "Remediation"]));
                for diagnostic in sections::sorted_diagnostics(disclosure) {
                    body.push_str(&row(&[
                        escape_html(&diagnostic.code),
                        escape_html(severity_label(&diagnostic.severity)),
                        escape_html(&diagnostic.message),
                        escape_html(&diagnostic.remediation),
                    ]));
                }
                body.push_str("</table>\n<h3>Assumptions</h3>\n<table>\n");
                body.push_str(&header_row(&["Assumption", "Owner", "Statement", "Scope"]));
                for assumption in &disclosure.assumptions {
                    body.push_str(&row(&[
                        escape_html(&assumption.assumption_id),
                        escape_html(&assumption.owner),
                        escape_html(&assumption.statement),
                        escape_html(&section_reference_text(&assumption.affected_scope)),
                    ]));
                }
                body.push_str("</table>\n<h3>User-Supplied Values</h3>\n<table>\n");
                body.push_str(&header_row(&[
                    "Value",
                    "Category",
                    "Quantity",
                    "Source",
                    "Missing-data finding",
                ]));
                for value in &disclosure.user_supplied_values {
                    let quantity = match &value.quantity {
                        Some(quantity) => format!(
                            "{} {} [{:?}]",
                            quantity.magnitude, quantity.unit, quantity.dimension
                        ),
                        None => "TBD".to_string(),
                    };
                    body.push_str(&row(&[
                        escape_html(&value.value_id),
                        escape_html(&value.value_category),
                        escape_html(&quantity),
                        escape_html(&section_reference_text(&value.source)),
                        escape_html(if value.missing_data_finding { "yes" } else { "no" }),
                    ]));
                }
                body.push_str("</table>\n<h3>Provenance Notes</h3>\n<table>\n");
                body.push_str(&header_row(&["Source", "Location", "License", "Contributor"]));
                for note in &disclosure.provenance_notes {
                    body.push_str(&row(&[
                        escape_html(&note.source_name),
                        escape_html(&note.source_location),
                        escape_html(&note.source_license),
                        escape_html(&note.contributor),
                    ]));
                }
                body.push_str("</table>\n");
                ("Warnings, Assumptions, And Provenance", body)
            }
            "audit_manifest" => {
                let mut body = String::from(
                    "<p>The SHA-256 of this rendered HTML document is computed over its exact \
bytes and recorded alongside the referenced envelope hashes below. The rendered-document hash is \
reported by the generating application next to this artifact; it cannot appear inside the bytes \
it binds.</p>\n<table>\n",
                );
                body.push_str(&header_row(&["Referenced envelope", "Checksum"]));
                for envelope in calc
                    .audit_manifest_refs
                    .iter()
                    .chain(calc.report_section_refs.iter())
                {
                    body.push_str(&row(&[
                        escape_html(&reference_text(&envelope.reference)),
                        escape_html(&checksum_text(&envelope.checksum)),
                    ]));
                }
                body.push_str("</table>\n");
                ("Audit Manifest", body)
            }
            "rule_pack_references" => {
                let mut body = String::from("<table>\n");
                body.push_str(&header_row(&[
                    "Rule pack",
                    "Version",
                    "Checksum",
                    "Source notice",
                    "Completeness",
                ]));
                for rule_pack in &calc.rule_pack_refs {
                    body.push_str(&row(&[
                        escape_html(&rule_pack.rule_pack_id),
                        escape_html(&rule_pack.version),
                        escape_html(&checksum_text(&rule_pack.checksum)),
                        escape_html(&rule_pack.source_notice),
                        escape_html(&rule_pack.completeness_status),
                    ]));
                }
                body.push_str("</table>\n");
                ("Rule Pack References", body)
            }
            "limitations" => {
                let mut body = String::from("<table>\n");
                body.push_str(&header_row(&["Limitation", "Statement", "Scope"]));
                for limitation in &disclosure.limitations {
                    body.push_str(&row(&[
                        escape_html(&limitation.limitation_id),
                        escape_html(&limitation.statement),
                        escape_html(&section_reference_text(&limitation.affected_scope)),
                    ]));
                }
                body.push_str("</table>\n<h3>Unresolved TBDs</h3>\n<table>\n");
                body.push_str(&header_row(&["TBD", "Description", "Review needed"]));
                for tbd in &disclosure.unresolved_tbds {
                    body.push_str(&row(&[
                        escape_html(&tbd.tbd_id),
                        escape_html(&tbd.description),
                        escape_html(if tbd.review_needed { "yes" } else { "no" }),
                    ]));
                }
                for tbd in &calc.unresolved_runtime_tbds {
                    body.push_str(&row(&[
                        escape_html(&tbd.tbd_id),
                        escape_html(&tbd.description),
                        escape_html(if tbd.review_needed { "yes" } else { "no" }),
                    ]));
                }
                body.push_str("</table>\n");
                ("Limitations", body)
            }
            "professional_boundary_notice" => {
                let mut body = String::from("<div class=\"boundary\">\n<p>");
                body.push_str(&escape_html(
                    "This document is a software-generated calculation record. It is not a \
professional engineering approval, certification, seal, authentication, or code-compliance \
determination. Human review by a qualified engineer is required before any reliance.",
                ));
                body.push_str("</p>\n</div>\n<h3>Human Review Record</h3>\n<p class=\"meta\">");
                body.push_str(&escape_html(
                    "Completed only by a qualified human reviewer. The software records no \
approval and pre-fills no field.",
                ));
                body.push_str("</p>\n<table class=\"signoff\">\n");
                body.push_str(&header_row(&["Field", "Entry"]));
                for field in ["Reviewer name", "Qualification / license", "Date", "Signature", "Notes"] {
                    body.push_str(&row(&[escape_html(field), String::new()]));
                }
                body.push_str("</table>\n");
                ("Professional Boundary And Human Review", body)
            }
            _ => unreachable!("section order is a fixed constant"),
        };

        let lint_text = html_to_lint_text(&body_html);
        assembled.push(AssembledSection {
            key,
            title,
            body_html,
            lint_text,
        });
    }

    assembled
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

fn status_banner(disclosure: &sections::ReportSections) -> String {
    let mut banner = String::from("<div class=\"status-banner\">Analysis status: ");
    let labels: Vec<&'static str> = disclosure
        .analysis_status_disclosures
        .iter()
        .map(|item| analysis_status_label(&item.status))
        .collect();
    banner.push_str(&escape_html(&labels.join(", ")));
    banner.push_str("</div>\n");
    banner
}

/// Render the calculation report to a deterministic single-file HTML
/// document and evaluate every gate. The returned outcome carries the
/// canonical SHA-256 of the HTML bytes.
pub fn render_calculation_report(input: &RenderableReportInput) -> RenderOutcome {
    let report_validation = report::validate_report(&input.calculation_report);
    let section_validation = sections::validate_report_sections(&input.report_sections);

    let assembled = assemble_sections(input);

    // Lint gate (ii): assembled section text, pre-render.
    let provenance = lint_provenance(&input.calculation_report.provenance);
    let pre_render_targets: Vec<linter::LintTarget> = assembled
        .iter()
        .map(|section| linter::LintTarget {
            target_id: format!("report-section-{}", section.key),
            path: format!("report_renderer://section/{}", section.key),
            surface: linter::SurfaceKind::PublicReportExample,
            text: section.lint_text.clone(),
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
        "<p class=\"meta\">Report ID: {} · Model: {} · Run: {}</p>\n",
        escape_html(&input.calculation_report.report_id),
        escape_html(&section_reference_text(&input.report_sections.model_ref)),
        escape_html(&section_reference_text(&input.report_sections.run_ref)),
    ));
    body.push_str(&status_banner(&input.report_sections));
    if !blocking_reasons.is_empty() {
        body.push_str("<div class=\"blocked-banner\">EXPORT BLOCKED — ");
        body.push_str(&escape_html(&blocking_reasons.join("; ")));
        body.push_str("</div>\n");
    }
    for section in &assembled {
        body.push_str(&format!("<h2>{}</h2>\n", escape_html(section.title)));
        body.push_str(&section.body_html);
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
        pre_render_findings: pre_render_run.findings.iter().map(outcome_finding).collect(),
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

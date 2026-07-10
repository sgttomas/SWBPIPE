//! Deterministic PDF emitter gate and byte-golden determinism tests
//! (DEC-061), mirroring the renderer's determinism suite. All data invented.

use open_pipe_stress_pdf_emitter::{assemble_full_report_package, emit_calculation_report_pdf};
use open_pipe_stress_report_renderer::{
    assemble_report_sections, render_calculation_report, RenderableReportInput, ResultRow,
};
use open_pipe_stress_report_sections as sections;

const FIXTURE: &str =
    include_str!("../../../../fixtures/reports/invented/calculation_report_fixture.json");

fn invented_section_provenance() -> sections::Provenance {
    sections::Provenance {
        source_name: "Invented OpenPipeStress PDF emitter test".to_string(),
        source_location: "core/reporting/pdf_emitter/tests/emit.rs".to_string(),
        source_license: "project_fixture".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "Invented non-engineering data only.".to_string(),
        redistribution_status: sections::RedistributionStatus::InventedNonEngineeringExample,
        review_status: sections::ReviewStatus::Accepted,
        privacy_classification: sections::PrivacyClassification::InventedPublicExample,
    }
}

fn invented_report_sections() -> sections::ReportSections {
    sections::ReportSections {
        report_section_id: "invented-report-sections-001".to_string(),
        model_ref: sections::Reference::new("model", "invented-model"),
        run_ref: sections::Reference::new("analysis_run", "invented-run"),
        diagnostics: vec![],
        analysis_status_disclosures: vec![
            sections::AnalysisStatusDisclosure {
                status: sections::AnalysisStatus::MechanicsSolved,
                source: sections::Reference::new("solver", "invented-solver"),
                affected_object: sections::Reference::new("model", "invented-model"),
                explanation: "Invented linear-static preview mechanics solved.".to_string(),
                human_review_required: true,
                human_acceptance_ref: None,
            },
            sections::AnalysisStatusDisclosure {
                status: sections::AnalysisStatus::HumanReviewRequired,
                source: sections::Reference::new("pdf_emitter", "invented-emit"),
                affected_object: sections::Reference::new("report", "invented-report-001"),
                explanation: "Human professional review is required before any reliance."
                    .to_string(),
                human_review_required: true,
                human_acceptance_ref: None,
            },
        ],
        provenance_notes: vec![invented_section_provenance()],
        user_supplied_values: vec![],
        assumptions: vec![],
        limitations: vec![sections::Limitation {
            limitation_id: "invented-limitation-001".to_string(),
            source: sections::Reference::new("pdf_emitter", "invented-emit"),
            affected_scope: sections::Reference::new("model", "invented-model"),
            statement: "Invented preview data only; not engineering output.".to_string(),
            effect: sections::ReportEffect {
                mechanics_solve_qualified: true,
                user_rule_check_qualified: false,
                report_completeness: sections::ReportCompleteness::Qualified,
                human_review_required: true,
            },
            provenance: invented_section_provenance(),
        }],
        unresolved_tbds: vec![],
        professional_boundary: sections::ProfessionalBoundary::project_default(),
    }
}

fn fixture_input() -> RenderableReportInput {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("fixture parses");
    let calculation_report = serde_json::from_value(fixture["calculation_report"].clone())
        .expect("calculation_report deserializes via the serde feature");
    RenderableReportInput {
        report_title: "Invented Calculation Report (Technical Preview)".to_string(),
        calculation_report,
        report_sections: invented_report_sections(),
        result_rows: vec![ResultRow {
            row_id: "invented-row-001".to_string(),
            label: "Max bending stress (invented)".to_string(),
            case_ref: "load_case:invented-weight".to_string(),
            quantity_display: "12.5 MPa".to_string(),
            source_ref: "result_envelope:invented-result".to_string(),
        }],
    }
}

fn pdf_contains(pdf_bytes: &[u8], needle: &str) -> bool {
    pdf_bytes
        .windows(needle.len())
        .any(|window| window == needle.as_bytes())
}

#[test]
fn fixture_report_emits_deterministic_hash_bound_pdf() {
    let input = fixture_input();
    let first = emit_calculation_report_pdf(&input);
    let second = emit_calculation_report_pdf(&input);

    assert_eq!(
        first.pdf_bytes, second.pdf_bytes,
        "PDF emission must be byte-deterministic"
    );
    assert_eq!(first.sha256_hex, second.sha256_hex);
    assert_eq!(first.sha256_hex.len(), 64);
    assert!(first
        .sha256_hex
        .chars()
        .all(|ch| ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase()));
    assert!(first.page_count >= 1);
    assert!(
        !first.export_blocked,
        "fixture emission must pass all gates; blocking reasons: {:?}",
        first.blocking_reasons
    );

    assert!(first.pdf_bytes.starts_with(b"%PDF-1.4\n"));
    assert!(first.pdf_bytes.ends_with(b"%%EOF\n"));
    assert!(
        !pdf_contains(&first.pdf_bytes, "/CreationDate")
            && !pdf_contains(&first.pdf_bytes, "/ModDate"),
        "document metadata must carry no timestamps"
    );
    assert!(
        !pdf_contains(&first.pdf_bytes, "/ID"),
        "trailer must carry no document ID"
    );
    assert!(
        !pdf_contains(&first.pdf_bytes, "http://") && !pdf_contains(&first.pdf_bytes, "https://"),
        "document must have no external references"
    );
}

#[test]
fn pdf_hash_is_recorded_alongside_the_canonical_html_hash() {
    let input = fixture_input();
    let render = render_calculation_report(&input);
    let pdf = emit_calculation_report_pdf(&input);

    assert_eq!(pdf.canonical_html_sha256_hex, render.sha256_hex);
    assert_ne!(pdf.sha256_hex, render.sha256_hex);
}

#[test]
fn pdf_and_html_emit_from_one_assembled_section_model() {
    let input = fixture_input();
    let render = render_calculation_report(&input);
    let pdf = emit_calculation_report_pdf(&input);

    for section in assemble_report_sections(&input) {
        assert!(
            render.html.contains(section.title),
            "HTML emission must carry section title {:?}",
            section.title
        );
        assert!(
            pdf_contains(&pdf.pdf_bytes, section.title),
            "PDF emission must carry section title {:?}",
            section.title
        );
    }
    assert!(pdf_contains(&pdf.pdf_bytes, "Human Review Record"));
    assert!(pdf_contains(&pdf.pdf_bytes, "Analysis status:"));
    assert!(pdf_contains(&pdf.pdf_bytes, "Report ID:"));
}

#[test]
fn prohibited_professional_claim_blocks_pdf_export() {
    let mut input = fixture_input();
    input.report_sections.limitations[0].statement =
        "Certified by OpenPipeStress for construction use.".to_string();

    let outcome = emit_calculation_report_pdf(&input);

    assert!(outcome.export_blocked);
    assert!(outcome
        .blocking_reasons
        .iter()
        .any(|reason| reason.contains("ProhibitedProfessionalClaim")));
    assert!(outcome
        .text_model_findings
        .iter()
        .any(|finding| finding.blocking && finding.code == "ProhibitedProfessionalClaim"));
    assert!(
        pdf_contains(&outcome.pdf_bytes, "EXPORT BLOCKED"),
        "blocked document must carry a visible blocked banner"
    );
}

#[test]
fn incomplete_report_is_export_blocked() {
    let mut input = fixture_input();
    input.calculation_report.report_id = String::new();

    let outcome = emit_calculation_report_pdf(&input);

    assert!(outcome.export_blocked);
    assert!(outcome
        .blocking_reasons
        .iter()
        .any(|reason| reason.contains("REPORT-MISSING-ID")));
    assert!(pdf_contains(&outcome.pdf_bytes, "EXPORT BLOCKED"));
}

#[test]
fn full_report_package_binds_html_and_pdf_hashes() {
    let input = fixture_input();
    let first = assemble_full_report_package(&input);
    let second = assemble_full_report_package(&input);

    assert!(
        !first.export_blocked,
        "fixture package must pass all gates; blocking reasons: {:?}",
        first.blocking_reasons
    );
    assert_eq!(first.members.len(), 2);

    let html_member = &first.members[0];
    assert_eq!(html_member.role, "canonical_rendered_report_html");
    assert_eq!(html_member.media_type, "text/html");
    assert_eq!(html_member.sha256_hex, first.render.sha256_hex);
    assert_eq!(html_member.byte_length, first.render.html.len());

    let pdf_member = &first.members[1];
    assert_eq!(pdf_member.role, "canonical_rendered_report_pdf");
    assert_eq!(pdf_member.media_type, "application/pdf");
    assert_eq!(pdf_member.sha256_hex, first.pdf.sha256_hex);
    assert_eq!(pdf_member.byte_length, first.pdf.pdf_bytes.len());

    // Both emissions and their recorded hashes are deterministic.
    assert_eq!(first.render.html, second.render.html);
    assert_eq!(first.pdf.pdf_bytes, second.pdf.pdf_bytes);
    assert_eq!(first.members, second.members);

    // The package members carry the same hashes the standalone emissions
    // record.
    let render = render_calculation_report(&input);
    assert_eq!(first.render.sha256_hex, render.sha256_hex);
}

#[test]
fn result_row_content_reaches_the_pdf_bytes() {
    let input = fixture_input();
    let outcome = emit_calculation_report_pdf(&input);

    assert!(pdf_contains(&outcome.pdf_bytes, "Max bending stress"));
    assert!(pdf_contains(&outcome.pdf_bytes, "12.5 MPa"));
    // Cell text word-wraps deterministically, so assert a fragment that
    // stays on one wrapped line.
    assert!(pdf_contains(
        &outcome.pdf_bytes,
        "Invented preview data only;"
    ));
}

#[test]
fn non_winansi_characters_degrade_deterministically() {
    let mut input = fixture_input();
    input.result_rows[0].label = "Stress \u{03C3} max (invented)".to_string();

    let first = emit_calculation_report_pdf(&input);
    let second = emit_calculation_report_pdf(&input);

    assert_eq!(first.pdf_bytes, second.pdf_bytes);
    assert!(pdf_contains(&first.pdf_bytes, "Stress ? max"));
}

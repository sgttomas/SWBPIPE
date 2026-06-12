//! Renderer gate and determinism tests (DEC-021). All data invented.

use open_pipe_stress_protected_content_linter as linter;
use open_pipe_stress_report_renderer::{
    derived_print_view, escape_html, public_report_template, render_calculation_report,
    RenderableReportInput, ResultRow,
};
use open_pipe_stress_report_sections as sections;

const FIXTURE: &str =
    include_str!("../../../../fixtures/reports/invented/calculation_report_fixture.json");

fn invented_section_provenance() -> sections::Provenance {
    sections::Provenance {
        source_name: "Invented OpenPipeStress renderer test".to_string(),
        source_location: "core/reporting/report_renderer/tests/render.rs".to_string(),
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
                source: sections::Reference::new("report_renderer", "invented-render"),
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
            source: sections::Reference::new("report_renderer", "invented-render"),
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

#[test]
fn fixture_report_renders_deterministic_single_file_html() {
    let input = fixture_input();
    let first = render_calculation_report(&input);
    let second = render_calculation_report(&input);

    assert_eq!(first.html, second.html, "render must be deterministic");
    assert_eq!(first.sha256_hex, second.sha256_hex);
    assert_eq!(first.sha256_hex.len(), 64);

    assert!(first.html.starts_with("<!DOCTYPE html>"));
    assert!(
        !first.html.contains("<script"),
        "document must be scriptless"
    );
    assert!(
        !first.html.contains("http://") && !first.html.contains("https://"),
        "document must have no external references"
    );
    assert!(first.html.contains("Model Input Summary"));
    assert!(first.html.contains("Unit storage convention"));
    assert!(first.html.contains("entered_units_preserved"));
    assert!(first.html.contains("Model units"));
    assert!(first.html.contains("force=N, length=m, stress=MPa"));
    assert!(first.html.contains("Result units"));
    assert!(first.html.contains("Report-time conversion"));
    assert!(first.html.contains("Rule Pack References"));
    assert!(first.html.contains("Audit Manifest"));
    assert!(first.html.contains("human_review_required"));
    assert!(first.html.contains("Human Review Record"));
    assert!(first.html.contains("derived views"));
}

#[test]
fn fixture_report_is_not_export_blocked() {
    let outcome = render_calculation_report(&fixture_input());
    assert!(
        !outcome.export_blocked,
        "fixture render must pass all gates; blocking reasons: {:?}",
        outcome.blocking_reasons
    );
    assert!(outcome.blocking_reasons.is_empty());
}

#[test]
fn template_surface_passes_protected_content_lint() {
    // Lint gate (i): the bundled public template surface.
    let run = linter::lint_targets(
        "template-gate-test",
        linter::LintConfiguration::public_surfaces_only("template-gate-test"),
        vec![linter::LintTarget {
            target_id: "public-report-template".to_string(),
            path: "core/reporting/report_renderer/template".to_string(),
            surface: linter::SurfaceKind::PublicReportTemplate,
            text: public_report_template(),
            provenance: linter::invented_provenance(),
        }],
    );
    assert_eq!(
        run.summary.blocking_finding_count, 0,
        "template surface must carry no blocking lint findings: {:?}",
        run.findings
    );
}

#[test]
fn prohibited_professional_claim_blocks_export() {
    let mut input = fixture_input();
    input.report_sections.limitations[0].statement =
        "Certified by OpenPipeStress for construction use.".to_string();

    let outcome = render_calculation_report(&input);

    assert!(outcome.export_blocked);
    assert!(outcome
        .blocking_reasons
        .iter()
        .any(|reason| reason.contains("ProhibitedProfessionalClaim")));
    assert!(outcome
        .pre_render_findings
        .iter()
        .any(|finding| finding.blocking && finding.code == "ProhibitedProfessionalClaim"));
    assert!(outcome
        .post_render_findings
        .iter()
        .any(|finding| finding.blocking && finding.code == "ProhibitedProfessionalClaim"));
    assert!(outcome.html.contains("EXPORT BLOCKED"));
}

#[test]
fn incomplete_report_is_export_blocked_with_visible_banner() {
    let mut input = fixture_input();
    input.calculation_report.report_id = String::new();

    let outcome = render_calculation_report(&input);

    assert!(outcome.export_blocked);
    assert!(outcome
        .blocking_reasons
        .iter()
        .any(|reason| reason.contains("REPORT-MISSING-ID")));
    assert!(outcome.html.contains("EXPORT BLOCKED"));
}

#[test]
fn derived_print_view_names_canonical_hash_and_is_labeled() {
    let outcome = render_calculation_report(&fixture_input());
    let derived = derived_print_view(&outcome.html, &outcome.sha256_hex);

    assert!(derived.contains("DERIVED VIEW"));
    assert!(derived.contains(&outcome.sha256_hex));
    assert!(derived.contains("data-derived-view=\"true\""));
    assert_ne!(derived, outcome.html);
    // The canonical bytes are untouched: removing the banner line restores them.
    assert!(outcome.html.starts_with("<!DOCTYPE html>"));
}

#[test]
fn escaped_markup_cannot_inject_tags() {
    let mut input = fixture_input();
    input.result_rows[0].label = "<script>alert('x')</script>".to_string();

    let outcome = render_calculation_report(&input);

    assert!(!outcome.html.contains("<script"));
    assert!(outcome.html.contains("&lt;script&gt;"));
    assert_eq!(
        escape_html("a<b&\"c\"'d'"),
        "a&lt;b&amp;&quot;c&quot;&#39;d&#39;"
    );
}

#[test]
fn renderable_input_round_trips_through_json() {
    let input = fixture_input();
    let json = serde_json::to_value(&input).expect("input serializes");
    let back: RenderableReportInput = serde_json::from_value(json).expect("input deserializes");
    assert_eq!(back, input);
    assert_eq!(
        render_calculation_report(&back).sha256_hex,
        render_calculation_report(&input).sha256_hex
    );
}

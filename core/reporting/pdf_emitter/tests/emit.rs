//! Deterministic PDF emitter gate and byte-golden determinism tests
//! (DEC-061), mirroring the renderer's determinism suite. All data invented.

use open_pipe_stress_pdf_emitter::{assemble_full_report_package, emit_calculation_report_pdf};
use open_pipe_stress_report_renderer::{
    assemble_report_sections, render_calculation_report, RenderableReportInput, ReportContext,
    ReportExportContext, ResultRow, UserRuleCheckOutcome, UserRuleCheckRun, UserRuleCheckStatus,
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
        report_title: "Invented Calculation Report".to_string(),
        calculation_report,
        report_sections: invented_report_sections(),
        result_rows: vec![ResultRow {
            row_id: "invented-row-001".to_string(),
            label: "Max bending stress (invented)".to_string(),
            case_ref: "load_case:invented-weight".to_string(),
            quantity_display: "12.5 MPa".to_string(),
            source_ref: "result_envelope:invented-result".to_string(),
        }],
        report_context: None,
        user_rule_checks: None,
        exact_route: None,
        library_references: None,
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

#[test]
fn local_private_notice_reaches_the_pdf_only_for_a_local_private_context() {
    let mut input = fixture_input();
    let public = emit_calculation_report_pdf(&input);
    assert!(!pdf_contains(&public.pdf_bytes, "LOCAL PRIVATE REPORT"));

    input.report_context = Some(ReportContext {
        export_context: ReportExportContext::LocalPrivate,
        generated_at: Some("2026-10-22T12:00:00.000Z".to_string()),
        software_name: "SWBPIPE".to_string(),
        software_version: Some("0.1.0".to_string()),
        build_release_identifier: None,
        solver_name: None,
        solver_version: None,
        solver_build_ref: None,
        analysis_run_name: None,
        analysis_run_recorded_at: None,
        solver_mode: None,
        solver_settings_ref: None,
        solver_settings: vec![],
        model_coordinate_system: None,
        result_coordinate_systems: vec![],
    });
    let private = emit_calculation_report_pdf(&input);
    assert!(!private.export_blocked, "{:?}", private.blocking_reasons);
    assert!(pdf_contains(&private.pdf_bytes, "LOCAL PRIVATE REPORT"));
    assert_eq!(private.pdf_bytes, emit_calculation_report_pdf(&input).pdf_bytes);
}

#[test]
fn rule_pack_name_and_registered_status_forms_reach_the_pdf() {
    let mut input = fixture_input();
    input.calculation_report.rule_pack_refs[0].name = Some("Invented Pack".to_string());
    input.user_rule_checks = Some(UserRuleCheckRun {
        rule_pack_id: input.calculation_report.rule_pack_refs[0].rule_pack_id.clone(),
        grammar_version: "1.0.0".to_string(),
        aggregate_status: UserRuleCheckStatus::UserRuleChecked,
        checks: vec![UserRuleCheckOutcome {
            check_id: "chk-a".to_string(),
            basis_ref: None,
            status: UserRuleCheckStatus::UserRuleChecked,
            missing_inputs: vec![],
        }],
    });
    let outcome = emit_calculation_report_pdf(&input);
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    assert!(pdf_contains(&outcome.pdf_bytes, "Invented Pack"));
    // The aggregate meta line carries domain, display form and token.
    assert!(pdf_contains(&outcome.pdf_bytes, "User-rule checked"));
    // PDF literal strings escape parentheses.
    assert!(pdf_contains(&outcome.pdf_bytes, r"\(USER_RULE_CHECKED\)"));
    assert!(pdf_contains(&outcome.pdf_bytes, "User-rule-check authority"));
}

#[test]
fn a_recorded_rule_fail_with_a_blocked_check_is_bannered_incomplete() {
    // FR-RULE-003: the PDF status banner follows the recorded checks, keeping
    // the recorded status beside the correction.
    let mut input = fixture_input();
    input.report_sections.analysis_status_disclosures.push(sections::AnalysisStatusDisclosure {
        status: sections::AnalysisStatus::UserRuleFailed,
        source: sections::Reference::new("rule_check_runner", "invented-rule-run"),
        affected_object: sections::Reference::new("model", "invented-model"),
        explanation: "Invented recorded rule-check status.".to_string(),
        human_review_required: true,
        human_acceptance_ref: None,
    });
    input.user_rule_checks = Some(UserRuleCheckRun {
        rule_pack_id: "invented-rule-pack".to_string(),
        grammar_version: "1.0.0".to_string(),
        aggregate_status: UserRuleCheckStatus::UserRuleFailed,
        checks: vec![
            UserRuleCheckOutcome {
                check_id: "invented-check-failed".to_string(),
                basis_ref: None,
                status: UserRuleCheckStatus::UserRuleFailed,
                missing_inputs: vec![],
            },
            UserRuleCheckOutcome {
                check_id: "invented-check-blocked".to_string(),
                basis_ref: None,
                status: UserRuleCheckStatus::RuleInputsIncomplete,
                missing_inputs: vec!["invented_input_x".to_string()],
            },
        ],
    });
    let outcome = emit_calculation_report_pdf(&input);
    // The banner word-wraps, so assert fragments that stay on one line.
    assert!(pdf_contains(&outcome.pdf_bytes, "rule_inputs_incomplete"));
    assert!(pdf_contains(&outcome.pdf_bytes, "recorded:"));
    let mut without = input.clone();
    without.user_rule_checks = None;
    let plain = emit_calculation_report_pdf(&without);
    assert!(!pdf_contains(&plain.pdf_bytes, "recorded:"), "no correction without a blocked check");
}

// ---------------------------------------------------------------------------
// W-11 (DEL-08-01 Design `exact-route-report-sections.md` §3.2, §4.2 and §7,
// slice (a)): the exact-route block and Library Records are emitted from the
// one assembled section model, and inputs without them keep their bytes.
// ---------------------------------------------------------------------------

#[path = "../../report_renderer/tests/common/exact_input.rs"]
mod exact_input;

/// The base commit's PDF and canonical HTML hashes of the fixture input,
/// captured on main without W-11 (`aed4de9fbd`; equal to the `3a104e1e71`
/// capture).
const BASE_PDF_SHA256: &str = "38f385b297869011df0a0e80b923a7a878bdf2ef079786533141351c25dfe09d";
const BASE_HTML_SHA256: &str = "a0a80cd6af6b0231c32622e5c198603a674cc413808c5d9429bb07eaa3e8777f";

#[test]
fn existing_fixture_input_emits_byte_identical_pdf() {
    let outcome = emit_calculation_report_pdf(&fixture_input());
    assert_eq!(outcome.sha256_hex, BASE_PDF_SHA256);
    assert_eq!(outcome.canonical_html_sha256_hex, BASE_HTML_SHA256);
    let mut empty = fixture_input();
    empty.library_references = Some(vec![]);
    assert_eq!(emit_calculation_report_pdf(&empty).sha256_hex, BASE_PDF_SHA256);
}

#[test]
fn exact_block_and_library_records_emit_in_the_pdf() {
    let input = exact_input::with_exact_route(fixture_input());
    let render = render_calculation_report(&input);
    let pdf = emit_calculation_report_pdf(&input);
    assert!(!pdf.export_blocked, "{:?}", pdf.blocking_reasons);
    assert_eq!(pdf.canonical_html_sha256_hex, render.sha256_hex);
    assert!(pdf.text_model_findings.iter().all(|finding| !finding.blocking));
    for needle in [
        "Exact-Route Result",
        "Semantic contract: openpipestress.result_semantics/0.3.0/pressure-2",
        "Stress Headlines by Section Basis",
        "Section basis",
        "Stress maximum",
        "Nominal section",
        "Reduced section",
        "41354908.75465696 Pa",
        "not available",
        "Library Records",
        "Library kind",
        "Referenced by",
    ] {
        assert!(pdf_contains(&pdf.pdf_bytes, needle), "PDF emission must carry {needle:?}");
        assert!(render.html.contains(needle), "{needle}");
    }
    // The section model is shared: the exact block opens the Results section.
    let results = assemble_report_sections(&input).into_iter().find(|section| section.key == "results").unwrap();
    assert!(matches!(&results.blocks[0], open_pipe_stress_report_renderer::SectionBlock::Subheading { text } if text == "Exact-Route Result"));
    // A binding mismatch blocks the PDF as it blocks the HTML.
    let mut mismatched = input.clone();
    mismatched.exact_route.as_mut().unwrap().admission.run_ref = "invented-other-run".to_string();
    let blocked = emit_calculation_report_pdf(&mismatched);
    assert!(blocked.export_blocked);
    assert!(blocked.blocking_reasons.iter().any(|reason| reason == "exact route: REPORT-EXACT-ADMISSION-BINDING-MISMATCH: run_ref"));
    assert!(pdf_contains(&blocked.pdf_bytes, "EXPORT BLOCKED"));
}

// W-11 slice (c) (DEL-08-01 Design `exact-route-report-sections.md` §4.3,
// §6.2): a report provenance record the renderer refuses blocks the PDF too.
#[test]
fn a_quarantined_or_rejected_provenance_record_blocks_the_pdf() {
    for (status, value) in [(sections::ReviewStatus::Quarantined, "quarantined"), (sections::ReviewStatus::Rejected, "rejected")] {
        let mut input = fixture_input();
        let mut record = invented_section_provenance();
        record.source_name = "Invented blocked material record".to_string();
        record.redistribution_status = sections::RedistributionStatus::PrivateOnly;
        record.privacy_classification = sections::PrivacyClassification::PrivateProjectData;
        record.review_status = status;
        input.report_sections.provenance_notes.push(record);
        let index = input.report_sections.provenance_notes.len() - 1;
        let pdf = emit_calculation_report_pdf(&input);
        assert!(pdf.export_blocked, "{value}");
        let reason = format!("report provenance: REPORT_PROVENANCE_DISPOSITION_BLOCKED (provenance_notes[{index}] review_status {value}; review that record's disposition in the model before reporting)");
        assert!(pdf.blocking_reasons.contains(&reason), "{:?}", pdf.blocking_reasons);
        assert!(pdf_contains(&pdf.pdf_bytes, "EXPORT BLOCKED"));
    }
}

/// W-11 slice (c) (§6.2), builder to render to PDF: the desktop builder's
/// representative records (`renderedReport.test.tsx` writes them), with
/// blocking dispositions nested in structured notes and records carried into
/// their report records, block the PDF as they block the HTML; without the
/// blocking records the PDF emits, and outside local_private it refuses.
#[test]
fn the_desktop_builder_records_block_the_pdf_by_disposition_and_context() {
    const RECORDS: &str = include_str!("../../../../fixtures/reports/invented/report_sections_provenance_records.json");
    let fixture: serde_json::Value = serde_json::from_str(RECORDS).unwrap();
    let mut input = fixture_input();
    input.report_sections = serde_json::from_value(fixture["report_sections"].clone()).unwrap();
    let context: ReportContext = serde_json::from_value(serde_json::json!({
        "export_context": "local_private", "generated_at": null, "software_name": "Invented", "software_version": null,
        "build_release_identifier": null, "solver_name": null, "solver_version": null, "solver_build_ref": null,
        "analysis_run_name": null, "analysis_run_recorded_at": null, "solver_mode": null, "solver_settings_ref": null,
        "solver_settings": [], "model_coordinate_system": null, "result_coordinate_systems": []
    }))
    .unwrap();
    input.report_context = Some(context);
    let pdf = emit_calculation_report_pdf(&input);
    assert!(pdf.export_blocked);
    for needle in [
        "(provenance_notes[9] review_status rejected;",
        "(provenance_notes[10] review_status quarantined;",
        "(user_supplied_values[8] section-provenance:section:nested-rejected review_status rejected;",
    ] {
        assert!(pdf.blocking_reasons.iter().any(|reason| reason.contains(needle)), "{needle}: {:?}", pdf.blocking_reasons);
    }
    assert!(pdf.blocking_reasons.iter().any(|reason| reason == "section validation: REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED"));
    assert!(pdf_contains(&pdf.pdf_bytes, "EXPORT BLOCKED"));
    let blocks = |record: &sections::Provenance| {
        matches!(record.review_status, sections::ReviewStatus::Quarantined | sections::ReviewStatus::Rejected)
            || record.redistribution_status == sections::RedistributionStatus::ProtectedSuspected
    };
    let mut clean = input.clone();
    clean.report_sections.provenance_notes.retain(|note| !blocks(note));
    clean.report_sections.user_supplied_values.retain(|value| !blocks(&value.provenance));
    let emitted = emit_calculation_report_pdf(&clean);
    assert!(!emitted.export_blocked, "{:?}", emitted.blocking_reasons);
    assert!(pdf_contains(&emitted.pdf_bytes, "report_metadata"));
    let mut public = clean.clone();
    public.report_context.as_mut().unwrap().export_context = ReportExportContext::PublicReport;
    let refused = emit_calculation_report_pdf(&public);
    assert!(refused.export_blocked);
    assert!(refused.blocking_reasons.iter().any(|reason| reason.contains("REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE")));
}

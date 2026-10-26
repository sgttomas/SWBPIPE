//! Renderer gate and determinism tests (DEC-021). All data invented.

use open_pipe_stress_protected_content_linter as linter;
use open_pipe_stress_report_renderer::{
    derived_print_view, escape_html, public_report_template, render_calculation_report, status_banner_text,
    RenderableReportInput, ReportContext, ReportExportContext, ReportSetting, ResultRow,
    UserRuleCheckBasisRef, UserRuleCheckOutcome, UserRuleCheckRun, UserRuleCheckStatus,
    USER_RULE_CHECK_COUNT_SCOPE_NOTE,
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

fn invented_component_provenance() -> sections::Provenance {
    sections::Provenance {
        source_name: "Invented elbow marker component provenance".to_string(),
        source_location:
            "component.provenance=invented_example_user_entered_bend_values_no_code_table; geometry.bend_geometry_source_reference=invented_user_entered_preview_geometry"
                .to_string(),
        source_license: "project_fixture".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification:
            "Invented component metadata only; no protected standards tables, code-derived factors, or catalog defaults bundled."
                .to_string(),
        redistribution_status: sections::RedistributionStatus::InventedNonEngineeringExample,
        review_status: sections::ReviewStatus::Accepted,
        privacy_classification: sections::PrivacyClassification::InventedPublicExample,
    }
}

fn invented_report_sections() -> sections::ReportSections {
    let component_provenance = invented_component_provenance();
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
        provenance_notes: vec![invented_section_provenance(), component_provenance.clone()],
        user_supplied_values: vec![sections::UserSuppliedValue {
            value_id: "component-provenance:component:C-110".to_string(),
            value_category: "component_provenance:bend".to_string(),
            source: sections::Reference::new("component", "component:C-110"),
            quantity: None,
            provenance: component_provenance,
            privacy_classification: sections::PrivacyClassification::InventedPublicExample,
            required_for: vec![
                sections::RequiredFor::Reporting,
                sections::RequiredFor::HumanReview,
            ],
            review_status: sections::ReviewStatus::Accepted,
            missing_data_finding: false,
        }],
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

fn invented_report_context(export_context: ReportExportContext) -> ReportContext {
    ReportContext {
        export_context,
        generated_at: Some("2026-10-22T12:00:00.000Z".to_string()),
        software_name: "SWBPIPE".to_string(),
        software_version: Some("0.1.0".to_string()),
        build_release_identifier: None,
        solver_name: Some("invented_solver".to_string()),
        solver_version: Some("0.2.0".to_string()),
        solver_build_ref: Some("invented_solver@0.2.0".to_string()),
        analysis_run_name: Some("invented run record".to_string()),
        analysis_run_recorded_at: None,
        solver_mode: Some("sparse_interactive".to_string()),
        solver_settings_ref: Some("solver-settings:invented".to_string()),
        solver_settings: vec![ReportSetting {
            key: "invented_policy".to_string(),
            value: "\"invented-policy-v1\"".to_string(),
        }],
        model_coordinate_system: None,
        result_coordinate_systems: vec!["element_local".to_string(), "global".to_string()],
    }
}

fn invented_rule_check_run() -> UserRuleCheckRun {
    UserRuleCheckRun {
        rule_pack_id: "invented-rule-pack".to_string(),
        grammar_version: "1.0.0".to_string(),
        aggregate_status: UserRuleCheckStatus::RuleInputsIncomplete,
        checks: vec![
            UserRuleCheckOutcome {
                check_id: "invented-check-a".to_string(),
                basis_ref: None,
                status: UserRuleCheckStatus::UserRuleChecked,
                missing_inputs: vec![],
            },
            UserRuleCheckOutcome {
                check_id: "invented-check-b".to_string(),
                basis_ref: None,
                status: UserRuleCheckStatus::RuleInputsIncomplete,
                missing_inputs: vec!["invented_input_x".to_string(), "invented_input_y".to_string()],
            },
        ],
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
    assert!(first.html.contains("component-provenance:component:C-110"));
    assert!(first.html.contains("component_provenance:bend"));
    assert!(first.html.contains("component:component:C-110"));
    assert!(first
        .html
        .contains("Invented elbow marker component provenance"));
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
    let mut config = linter::LintConfiguration::public_surfaces_only("template-gate-test");
    config.public_surface_roots = vec!["core/reporting/report_renderer/template".to_string()];
    let run = linter::lint_targets(
        "template-gate-test",
        config,
        vec![linter::LintTarget {
            target_id: "public-report-template".to_string(),
            path: "core/reporting/report_renderer/template".to_string(),
            surface: linter::SurfaceKind::PublicReportTemplate,
            text: public_report_template(),
            provenance: linter::invented_provenance(),
        }],
    );
    assert_eq!(run.summary.scanned_target_count, 1);
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

#[test]
fn report_context_renders_prd_19_2_identity_settings_and_coordinate_rows() {
    let mut input = fixture_input();
    input.report_context = Some(invented_report_context(ReportExportContext::PublicReport));
    let outcome = render_calculation_report(&input);
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    let html = &outcome.html;
    for expected in [
        "Software, Solver And Run",
        "<td>Software</td><td>SWBPIPE</td>",
        "<td>Software version</td><td>0.1.0</td>",
        "<td>Build/release identifier</td><td>not recorded</td>",
        "<td>Solver version</td><td>0.2.0</td>",
        "<td>Solver build reference</td><td>invented_solver@0.2.0</td>",
        "<td>Report generated at (application clock)</td><td>2026-10-22T12:00:00.000Z</td>",
        "<td>Analysis run recorded at</td><td>not recorded</td>",
        "<td>Solver mode</td><td>sparse_interactive</td>",
        "<td>Solver setting: invented_policy</td><td>&quot;invented-policy-v1&quot;</td>",
        "<td>Model coordinate system</td><td>not recorded</td>",
        "<td>Result coordinate systems</td><td>element_local, global</td>",
        "<td>Report export context</td><td>public_report</td>",
        "No user-rule check run is recorded for this report&#39;s result.",
    ] {
        assert!(html.contains(expected), "missing {expected}");
    }
    assert!(!html.contains("LOCAL PRIVATE REPORT"));
    assert!(!html.contains("local-private-banner\""));
}

#[test]
fn absent_report_context_is_disclosed_as_not_recorded() {
    let outcome = render_calculation_report(&fixture_input());
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    assert!(outcome
        .html
        .contains("<td>Report export context</td><td>not recorded</td>"));
    assert!(outcome
        .html
        .contains("<td>Software version</td><td>not recorded</td>"));
    assert!(outcome
        .html
        .contains("<td>Solver settings</td><td>not recorded</td>"));
    assert!(!outcome.html.contains("LOCAL PRIVATE REPORT"));
}

#[test]
fn local_private_context_records_its_status_in_the_document() {
    let mut input = fixture_input();
    input.report_context = Some(invented_report_context(ReportExportContext::LocalPrivate));
    let private = render_calculation_report(&input);
    assert!(!private.export_blocked, "{:?}", private.blocking_reasons);
    assert!(private.html.contains(
        "<div class=\"local-private-banner\" data-export-context=\"local_private\">LOCAL PRIVATE REPORT"
    ));
    assert!(private.html.contains("not for sharing or publication"));
    assert!(private
        .html
        .contains("<td>Report export context</td><td>local_private</td>"));
    // The PRD section 19.3 notice is unchanged by the context.
    assert!(private.html.contains(
        "SWBPIPE is decision-support software for piping design, flexibility, and stress-analysis workflows."
    ));

    input.report_context = Some(invented_report_context(ReportExportContext::PublicReport));
    let public = render_calculation_report(&input);
    assert_ne!(public.sha256_hex, private.sha256_hex);
    assert!(!public.html.contains("LOCAL PRIVATE REPORT"));
}

#[test]
fn user_rule_check_headline_is_incomplete_while_any_check_is_blocked() {
    // FR-RULE-003: a failed and a blocked check headline INCOMPLETE with both
    // counts, even for a run recorded under the earlier failed-first aggregate.
    let mut input = fixture_input();
    input.report_context = Some(invented_report_context(ReportExportContext::LocalPrivate));
    let mut run = invented_rule_check_run();
    run.aggregate_status = UserRuleCheckStatus::UserRuleFailed;
    run.checks[0].status = UserRuleCheckStatus::UserRuleFailed;
    input.user_rule_checks = Some(run.clone());
    let html = render_calculation_report(&input).html;
    assert!(html.contains("Aggregate status: INCOMPLETE — 1 check(s) failed; 1 check(s) blocked · User-rule-check authority · Rule inputs incomplete (RULE_INPUTS_INCOMPLETE)"));
    assert!(!html.contains("Aggregate status: User-rule-check authority · User-rule failed"));
    // The failed check stays listed with its own status.
    assert!(html.contains("<td>invented-check-a</td><td>load case not recorded</td><td>User-rule-check authority · User-rule failed (USER_RULE_FAILED)</td>"));

    // A recorded failed rule-check analysis status is shown as incomplete in
    // the status banner, with the recorded status kept beside it.
    input.report_sections.analysis_status_disclosures.push(sections::AnalysisStatusDisclosure {
        status: sections::AnalysisStatus::UserRuleFailed,
        source: sections::Reference::new("rule_check_runner", "invented-rule-run"),
        affected_object: sections::Reference::new("model", "invented-model"),
        explanation: "Invented recorded rule-check status.".to_string(),
        human_review_required: true,
        human_acceptance_ref: None,
    });
    let html = render_calculation_report(&input).html;
    assert!(html.contains("rule_inputs_incomplete (recorded: user_rule_failed)"), "banner keeps a recorded fail headline");
    assert!(!html.contains("human_review_required, user_rule_failed"));
    assert_eq!(
        status_banner_text(&input),
        "Analysis status: mechanics_solved, human_review_required, rule_inputs_incomplete (recorded: user_rule_failed)"
    );

    // With every check evaluated, the recorded aggregate is shown as before.
    for check in &mut run.checks {
        check.status = UserRuleCheckStatus::UserRuleChecked;
        check.missing_inputs.clear();
    }
    run.aggregate_status = UserRuleCheckStatus::UserRuleChecked;
    input.user_rule_checks = Some(run);
    let html = render_calculation_report(&input).html;
    assert!(html.contains("Aggregate status: User-rule-check authority · User-rule checked (USER_RULE_CHECKED)"));
    assert!(html.contains("human_review_required, user_rule_failed</div>"), "a recorded status is shown unchanged when no check is blocked");
    assert!(!html.contains("INCOMPLETE —"));
}

#[test]
fn user_rule_check_outcomes_render_identity_status_and_missing_inputs_only() {
    let mut input = fixture_input();
    input.report_context = Some(invented_report_context(ReportExportContext::LocalPrivate));
    input.user_rule_checks = Some(invented_rule_check_run());
    let outcome = render_calculation_report(&input);
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    let html = &outcome.html;
    assert!(html.contains("User-Rule Check Outcomes"));
    assert!(html.contains(
        "Rule pack: invented-rule-pack · Grammar version: 1.0.0 · Aggregate status: INCOMPLETE — 0 check(s) failed; 1 check(s) blocked · User-rule-check authority · Rule inputs incomplete (RULE_INPUTS_INCOMPLETE)"
    ));
    assert!(html.contains(
        "<td>invented-check-a</td><td>load case not recorded</td><td>User-rule-check authority · User-rule checked (USER_RULE_CHECKED)</td><td>none</td>"
    ));
    assert!(html.contains(
        "<td>invented-check-b</td><td>load case not recorded</td><td>User-rule-check authority · Rule inputs incomplete (RULE_INPUTS_INCOMPLETE)</td><td>invented_input_x, invented_input_y</td>"
    ));
    // The fixture reference records no display name; it is disclosed as such.
    assert!(html.contains("<td>invented-rule-pack</td><td>not recorded</td><td>0.1.0</td>"));
    for raw in [
        "<td>USER_RULE_CHECKED</td>",
        "<td>RULE_INPUTS_INCOMPLETE</td>",
        "Aggregate status: RULE_INPUTS_INCOMPLETE",
    ] {
        assert!(!html.contains(raw), "raw status token shown without its display form: {raw}");
    }
    let again = render_calculation_report(&input);
    assert_eq!(again.sha256_hex, outcome.sha256_hex);
}

#[test]
fn rule_check_run_for_an_unreferenced_rule_pack_blocks_export() {
    let mut input = fixture_input();
    let mut run = invented_rule_check_run();
    run.rule_pack_id = "invented-other-pack".to_string();
    input.user_rule_checks = Some(run);
    let outcome = render_calculation_report(&input);
    assert!(outcome.export_blocked);
    assert!(outcome
        .blocking_reasons
        .iter()
        .any(|reason| reason == "report context: USER-RULE-CHECK-RULE-PACK-UNREFERENCED"));
}

#[test]
fn empty_rule_pack_refs_are_stated_explicitly() {
    let mut input = fixture_input();
    input.calculation_report.rule_pack_refs.clear();
    let outcome = render_calculation_report(&input);
    assert!(outcome
        .html
        .contains("No rule pack is referenced by this report."));
}

#[test]
fn report_context_round_trips_through_json() {
    let mut input = fixture_input();
    input.report_context = Some(invented_report_context(ReportExportContext::LocalPrivate));
    input.user_rule_checks = Some(invented_rule_check_run());
    let json = serde_json::to_value(&input).expect("input serializes");
    assert_eq!(json["report_context"]["export_context"], "local_private");
    assert_eq!(json["user_rule_checks"]["checks"][1]["status"], "RULE_INPUTS_INCOMPLETE");
    let back: RenderableReportInput = serde_json::from_value(json).expect("input deserializes");
    assert_eq!(back, input);
}

#[test]
fn demo_rule_pack_safe_metadata_and_local_private_context_pass_the_lint_gates() {
    let mut input = fixture_input();
    let rule_pack = &mut input.calculation_report.rule_pack_refs[0];
    rule_pack.rule_pack_id = "invented_demo_rule_pack".to_string();
    // The checksum names the pack it is attached to (checksum subjects).
    rule_pack.checksum.payload_ref.ref_id = "invented_demo_rule_pack".to_string();
    rule_pack.version = "0.2.0".to_string();
    rule_pack.source_notice = "Original invented demonstration data only. Not an engineering design basis, not a standards interpretation, and not suitable for professional reliance.".to_string();
    rule_pack.completeness_status = "incomplete".to_string();
    let mut context = invented_report_context(ReportExportContext::LocalPrivate);
    context.solver_settings = vec![
        ReportSetting {
            key: "nonlinear_iteration_policy".to_string(),
            value: "\"DEC-046-CV-B-product-preview-active-set-count-v1\"".to_string(),
        },
        ReportSetting {
            key: "sparse_evidence_lane".to_string(),
            value: "true".to_string(),
        },
    ];
    context.solver_build_ref = Some("open_pipe_stress_product_physics@0.2.0".to_string());
    input.report_context = Some(context);
    let mut run = invented_rule_check_run();
    run.rule_pack_id = "invented_demo_rule_pack".to_string();
    input.user_rule_checks = Some(run);

    let outcome = render_calculation_report(&input);
    assert!(
        !outcome.export_blocked,
        "blocking reasons: {:?}",
        outcome.blocking_reasons
    );
}

const DEMO_RULE_PACK: &str =
    include_str!("../../../../fixtures/product_preview/invented_demo_rule_pack.json");

/// The rule-pack reference the desktop builder derives from the unchanged demo
/// pack: safe metadata only, including its display name.
fn demo_rule_pack_ref() -> open_pipe_stress_report_generator::RulePackRef {
    let pack: serde_json::Value = serde_json::from_str(DEMO_RULE_PACK).expect("demo pack parses");
    let metadata = &pack["metadata"];
    let checksum = &pack["checksums"]["rule_pack_checksum"];
    serde_json::from_value(serde_json::json!({
        "rule_pack_id": metadata["rule_pack_id"],
        "name": metadata["name"],
        "version": metadata["rule_pack_version"],
        "checksum": {
            "algorithm": checksum["algorithm"],
            "canonicalization": checksum["canonicalization"],
            "payload_ref": {"ref_type": "rule_pack", "ref_id": metadata["rule_pack_id"]},
            "value": checksum["value"]
        },
        "source_notice": metadata["source_notice"],
        "redistribution_status": pack["classification"]["redistribution_status"],
        "completeness_status": "incomplete",
        "private_payload_redacted": true
    }))
    .expect("demo rule-pack reference deserializes")
}

#[test]
fn demo_rule_pack_name_version_and_checksum_render() {
    let mut input = fixture_input();
    input.calculation_report.rule_pack_refs = vec![demo_rule_pack_ref()];
    let outcome = render_calculation_report(&input);
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    assert!(outcome.html.contains(
        "<td>invented_demo_rule_pack</td><td>Invented Demonstration Rule Pack</td><td>0.3.0</td>"
    ));
    assert!(outcome
        .html
        .contains("c96d28d94a1d4eee99ae2f501c3d4d5748fb81bfa7feb034cc8d96d60651e68f"));
    let json = serde_json::to_value(&input.calculation_report.rule_pack_refs[0]).expect("serializes");
    assert_eq!(json["name"], "Invented Demonstration Rule Pack");
}

#[test]
fn blank_rule_pack_name_is_an_unsafe_reference() {
    let mut input = fixture_input();
    let mut reference = demo_rule_pack_ref();
    reference.name = Some("  ".to_string());
    input.calculation_report.rule_pack_refs = vec![reference];
    let outcome = render_calculation_report(&input);
    assert!(outcome
        .blocking_reasons
        .iter()
        .any(|reason| reason == "report validation: REPORT-RULE-PACK-REF-UNSAFE"));
}

// ---------------------------------------------------------------------------
// W-11 (DEL-08-01 Design `exact-route-report-sections.md` §3.2-§3.5, §4.2 and
// §7, slice (a)): the exact-route block, its admission binding and composed
// standing line, the exact limitation entries, Library Records, and the
// byte identity of every input without these fields.
// ---------------------------------------------------------------------------

#[path = "common/exact_input.rs"]
mod exact_input;

use exact_input::{with_exact_route, BYTES, DERIVATIVE, LOAD_REFERENCE_LINE, MANIFEST, MODEL, PRESSURE_2, RECEIVED, WITHHELD};
use open_pipe_stress_report_renderer::exact_statements::{exact_statement_table, producer_sha256};
use open_pipe_stress_report_renderer::{
    exact_route_blocking_reasons, exact_standing_line, EXACT_ADMISSION_BINDING_MISMATCH, EXACT_HEADLINES_HEADER,
    EXACT_HEADLINES_PARAGRAPH, EXACT_HEADLINES_SUBHEADING, EXACT_HEADLINE_UNREADABLE, EXACT_LIMITATION_UNREGISTERED,
    EXACT_ROUTE_SUBHEADING, NOT_AVAILABLE,
};

/// The base commit's canonical HTML hashes of the existing invented inputs,
/// captured on main without W-11 (`aed4de9fbd`): the fields' absence keeps
/// their bytes. The private input's hash carries W-17's rule-check section
/// (#1629); the base and public hashes equal their `3a104e1e71` captures.
const BASE_HTML_SHA256: &str = "a137f51d3b16723e221d19c0041de670d328a3e0f904b19bfe3e909e5059f5f2";
const PRIVATE_HTML_SHA256: &str = "e962c4aeeed5fe94ac2e70e7584e8d253e165cf85dfd10d8ab0aa24ed5ca51eb";
const PUBLIC_HTML_SHA256: &str = "43f9ac50b08818080dc8b2769d8b4db9cfb6b9ccb76a62ca2118c310917562c9";

const COVERAGE: &str = include_str!("../../../../fixtures/reports/exact_route_limitations.json");

fn exact_input() -> RenderableReportInput {
    let mut input = with_exact_route(fixture_input());
    input.report_context = Some(invented_report_context(ReportExportContext::LocalPrivate));
    input
}

fn has_reason(outcome: &open_pipe_stress_report_renderer::RenderOutcome, reason: &str) -> bool {
    outcome.blocking_reasons.iter().any(|item| item == reason)
}

#[test]
fn existing_invented_inputs_render_byte_identical() {
    let base = fixture_input();
    let mut private = fixture_input();
    private.report_context = Some(invented_report_context(ReportExportContext::LocalPrivate));
    private.user_rule_checks = Some(invented_rule_check_run());
    let mut public = fixture_input();
    public.report_context = Some(invented_report_context(ReportExportContext::PublicReport));
    for (input, expected) in [(&base, BASE_HTML_SHA256), (&private, PRIVATE_HTML_SHA256), (&public, PUBLIC_HTML_SHA256)] {
        assert_eq!(render_calculation_report(input).sha256_hex, expected);
        // Absent fields serialize to nothing; explicit nulls and an empty
        // library list render the same bytes.
        let json = serde_json::to_value(input).unwrap();
        assert!(json.get("exact_route").is_none() && json.get("library_references").is_none());
        let mut nulls = json.clone();
        nulls["exact_route"] = serde_json::Value::Null;
        nulls["library_references"] = serde_json::Value::Null;
        let back: RenderableReportInput = serde_json::from_value(nulls).unwrap();
        assert_eq!(&back, input);
        let mut empty = input.clone();
        empty.library_references = Some(vec![]);
        assert_eq!(render_calculation_report(&empty).sha256_hex, expected);
    }
}

#[test]
fn exact_block_and_library_records_render_with_the_composed_standing_line() {
    let input = exact_input();
    let outcome = render_calculation_report(&input);
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    let html = &outcome.html;
    let binding = &input.exact_route.as_ref().unwrap().admission;
    let standing = format!(
        "integrity checked (the Rust reader&#39;s standing for the result Export Result JSON admitted in this session: received result sha256 {RECEIVED}, model sha256 {MODEL}, input manifest {} sha256 {MANIFEST})",
        binding.input_manifest_ref
    );
    assert_eq!(
        exact_standing_line(binding),
        format!("integrity checked (the Rust reader's standing for the result Export Result JSON admitted in this session: received result sha256 {RECEIVED}, model sha256 {MODEL}, input manifest {} sha256 {MANIFEST})", binding.input_manifest_ref)
    );
    let block = format!(
        "<h3>{EXACT_ROUTE_SUBHEADING}</h3>\n<p class=\"meta\">Semantic contract: {PRESSURE_2} · Numerical standing: {standing}</p>\n<p>{LOAD_REFERENCE_LINE}</p>\n<h3>{EXACT_HEADLINES_SUBHEADING}</h3>\n<table>\n<tr><th>Section basis</th><th>Stress maximum</th><th>Location</th><th>Source</th></tr>\n<tr><td>Nominal section (nominal wall)</td><td>41354908.75465696 Pa</td><td>pipe:invented-span</td><td>result:invented-nominal-maximum</td></tr>\n<tr><td>Reduced section (wall minus mill tolerance and corrosion allowance)</td><td>{WITHHELD}</td><td>{NOT_AVAILABLE}</td><td>{NOT_AVAILABLE}</td></tr>\n</table>\n<p>{EXACT_HEADLINES_PARAGRAPH}</p>\n"
    );
    // At the start of the Results section, before the rows.
    assert!(html.contains(&format!("<h2>Results</h2>\n{block}<table>\n<tr><th>Result</th>")), "{html}");
    assert!(html.contains("<h3>Library Records</h3>\n<table>\n<tr><th>Library kind</th><th>Library</th><th>Version</th><th>Record</th><th>Referenced by</th></tr>\n<tr><td>material</td><td>invented-material-library</td><td>sha256:7777777777777777777777777777777777777777777777777777777777777777</td><td>not recorded</td><td>material:invented-a, material:invented-b</td></tr>\n</table>\n"));
    let provenance = html.find("<h3>Provenance Notes</h3>").unwrap();
    assert!(html.find("<h3>Library Records</h3>").unwrap() > provenance);
    // Every exact limitation statement is in the Limitations table.
    for (_, statement) in exact_statement_table().unwrap().entries() {
        assert!(html.contains(&escape_html(statement)));
    }
    // The PRD §19.3 notice keeps its place; no exact-route text sits beside it.
    let boundary = html.find("<div class=\"boundary\">").unwrap();
    assert!(html.find(EXACT_ROUTE_SUBHEADING).unwrap() < boundary);
    // An exact report has no blocking linter finding.
    assert!(outcome.pre_render_findings.iter().all(|finding| !finding.blocking));
    assert!(outcome.post_render_findings.iter().all(|finding| !finding.blocking));
}

#[test]
fn each_binding_field_that_disagrees_with_the_report_refuses() {
    type Edit = fn(&mut RenderableReportInput);
    let cases: [(&str, Edit); 11] = [
        ("semantic_contract_id", |i| i.exact_route.as_mut().unwrap().semantic_contract_id = "openpipestress.result_semantics/0.3.0/pressure-source-1".into()),
        ("run_ref", |i| i.exact_route.as_mut().unwrap().admission.run_ref = "invented-other-run".into()),
        ("result_envelope_id", |i| i.exact_route.as_mut().unwrap().admission.result_envelope_id = "result-envelope:invented-other-run".into()),
        ("received_result_sha256", |i| i.exact_route.as_mut().unwrap().admission.received_result_sha256 = BYTES.into()),
        ("model_sha256", |i| i.calculation_report.model_input_summary.model_hash.value = DERIVATIVE.into()),
        ("input_manifest_ref", |i| i.exact_route.as_mut().unwrap().admission.input_manifest_ref = "invented-other-manifest".into()),
        ("input_manifest_sha256", |i| i.exact_route.as_mut().unwrap().admission.input_manifest_sha256 = MANIFEST[..63].into()),
        ("derivative_sha256", |i| i.exact_route.as_mut().unwrap().admission.derivative_sha256 = DERIVATIVE.replace('4', "A")),
        ("native_job_id", |i| i.exact_route.as_mut().unwrap().admission.native_job_id = " ".into()),
        ("received_bytes_sha256", |i| i.exact_route.as_mut().unwrap().admission.received_bytes_sha256 = "sha256:5555".into()),
        ("model_sha256", |i| {
            i.calculation_report.model_input_summary.model_hash.value = "invented-hash".into();
            i.exact_route.as_mut().unwrap().admission.model_sha256 = "invented-hash".into();
        }),
    ];
    for (field, edit) in cases {
        let mut input = exact_input();
        edit(&mut input);
        let outcome = render_calculation_report(&input);
        assert!(outcome.export_blocked, "{field}");
        let reason = format!("exact route: {EXACT_ADMISSION_BINDING_MISMATCH}: {field}");
        assert!(has_reason(&outcome, &reason), "{field}: {:?}", outcome.blocking_reasons);
        assert!(outcome.html.contains(&format!("Numerical standing: not stated ({EXACT_ADMISSION_BINDING_MISMATCH}: ")), "{field}");
        assert!(!outcome.html.contains("integrity checked (the Rust reader"), "{field}");
    }
}

#[test]
fn exact_limitation_entries_outside_the_table_or_without_exact_route_refuse() {
    let coverage: serde_json::Value = serde_json::from_str(COVERAGE).unwrap();
    let source_paragraph = coverage["lists"][0]["limitations"][7].as_str().unwrap().to_string();
    let run = "invented-run";
    let first = format!("limitation:{run}:exact:1");
    // A statement outside the table: the producer's whole source paragraph,
    // and an invented text.
    for statement in [source_paragraph.as_str(), "An invented limitation statement."] {
        let mut input = exact_input();
        input.report_sections.limitations[1].statement = statement.to_string();
        assert_eq!(input.report_sections.limitations[1].limitation_id, first);
        let outcome = render_calculation_report(&input);
        assert!(outcome.export_blocked);
        assert!(has_reason(&outcome, &format!("exact limitations: {EXACT_LIMITATION_UNREGISTERED}: {first}: the statement is not a reviewed exact-route statement")), "{:?}", outcome.blocking_reasons);
    }
    // Entries an id does not place in this run's exact series.
    for id in [format!("limitation:{run}:exact:0"), format!("limitation:{run}:exact:x"), "limitation:invented-other-run:exact:1".to_string()] {
        let mut input = exact_input();
        input.report_sections.limitations[1].limitation_id = id.clone();
        let outcome = render_calculation_report(&input);
        assert!(has_reason(&outcome, &format!("exact limitations: {EXACT_LIMITATION_UNREGISTERED}: {id}: the entry id is not an exact-route limitation id of this report's run")), "{id}");
    }
    // Exact entries without exact_route, each refused.
    let mut input = exact_input();
    input.exact_route = None;
    let outcome = render_calculation_report(&input);
    let refused = outcome
        .blocking_reasons
        .iter()
        .filter(|reason| reason.contains(EXACT_LIMITATION_UNREGISTERED) && reason.ends_with("an exact-route limitation entry without an exact_route section"))
        .count();
    assert_eq!(refused, 20);
    // A malformed statement table refuses every exact statement.
    let input = exact_input();
    let reasons = exact_route_blocking_reasons(&input, Err("REPORT-EXACT-STATEMENT-TABLE-INVALID: invented"));
    assert_eq!(reasons.len(), 19);
    assert!(reasons.iter().all(|reason| reason.contains(EXACT_LIMITATION_UNREGISTERED) && reason.ends_with("REPORT-EXACT-STATEMENT-TABLE-INVALID: invented")));
    assert!(exact_route_blocking_reasons(&input, exact_statement_table()).is_empty());
    // The table keys producer strings by digest; the source paragraph has one.
    assert!(exact_statement_table().unwrap().statement_for(&producer_sha256(&source_paragraph)).is_some());
}

#[test]
fn headlines_out_of_basis_order_refuse() {
    for edit in [
        (|route: &mut open_pipe_stress_report_renderer::ExactRouteSection| route.headlines.reverse()) as fn(&mut _),
        |route| {
            route.headlines.pop();
        },
        |route| route.headlines[1].basis = "governing".to_string(),
    ] {
        let mut input = exact_input();
        edit(input.exact_route.as_mut().unwrap());
        let outcome = render_calculation_report(&input);
        assert!(outcome.export_blocked);
        assert!(outcome.blocking_reasons.iter().any(|reason| reason.starts_with(&format!("exact route: {EXACT_HEADLINE_UNREADABLE}"))));
    }
}

#[test]
fn an_unknown_field_in_the_new_structs_refuses() {
    let input = exact_input();
    let json = serde_json::to_value(&input).unwrap();
    let back: RenderableReportInput = serde_json::from_value(json.clone()).unwrap();
    assert_eq!(back, input);
    assert_eq!(render_calculation_report(&back).sha256_hex, render_calculation_report(&input).sha256_hex);
    for pointer in ["/exact_route", "/exact_route/admission", "/exact_route/headlines/0", "/library_references/0"] {
        let mut changed = json.clone();
        changed.pointer_mut(pointer).unwrap().as_object_mut().unwrap().insert("standing".to_string(), "invented".into());
        let error = serde_json::from_value::<RenderableReportInput>(changed).unwrap_err().to_string();
        assert!(error.contains("unknown field"), "{pointer}: {error}");
    }
}

#[test]
fn exact_route_texts_pass_the_claims_check() {
    // GEN-13's litany vocabulary (`tools/validation/validate_claims_language.py`).
    const LITANY: [&str; 11] = [
        "certif", "seal", "authenticat", "code-compliance", "code compliance", "professional", "approval", "approve",
        "endorse", "release-readiness", "production-readiness",
    ];
    let input = exact_input();
    let binding = &input.exact_route.as_ref().unwrap().admission;
    let mut texts = vec![
        exact_standing_line(binding),
        format!("not stated ({EXACT_ADMISSION_BINDING_MISMATCH}: model_sha256)"),
        EXACT_ROUTE_SUBHEADING.to_string(),
        EXACT_HEADLINES_SUBHEADING.to_string(),
        EXACT_HEADLINES_PARAGRAPH.to_string(),
        NOT_AVAILABLE.to_string(),
        "Library Records".to_string(),
    ];
    texts.extend(EXACT_HEADLINES_HEADER.iter().map(|text| text.to_string()));
    texts.extend(exact_route_blocking_reasons(&{
        let mut changed = exact_input();
        changed.exact_route = None;
        changed
    }, exact_statement_table()));
    for text in texts {
        let folded = text.to_lowercase();
        assert!(!folded.contains("acceptance"), "{text}");
        assert!(LITANY.iter().filter(|term| folded.contains(*term)).count() < 3, "{text}");
        assert!(text.chars().all(|ch| (' '..='~').contains(&ch) || ('\u{a0}'..='\u{ff}').contains(&ch)), "{text}");
    }
}

// --- Rule-pack format 0.5.0: one outcome per check and load case (DEL-06-01
// Design `rule-pack-0.5.0.md` §2.7, §6) -------------------------------------

fn outcome_on(check_id: &str, ref_type: &str, ref_id: &str, status: UserRuleCheckStatus) -> UserRuleCheckOutcome {
    UserRuleCheckOutcome {
        check_id: check_id.to_string(),
        basis_ref: Some(UserRuleCheckBasisRef { ref_type: ref_type.to_string(), ref_id: ref_id.to_string() }),
        status,
        missing_inputs: vec![],
    }
}

#[test]
fn one_check_on_two_load_cases_lists_both_named_with_the_count_scope_note() {
    let mut input = fixture_input();
    input.report_context = Some(invented_report_context(ReportExportContext::LocalPrivate));
    let mut run = invented_rule_check_run();
    run.aggregate_status = UserRuleCheckStatus::UserRuleFailed;
    run.checks = vec![
        outcome_on("invented-check-a", "load_case", "load:L-100", UserRuleCheckStatus::UserRuleChecked),
        outcome_on("invented-check-a", "combination", "combo:C-1", UserRuleCheckStatus::UserRuleFailed),
    ];
    input.user_rule_checks = Some(run);
    let outcome = render_calculation_report(&input);
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    let html = &outcome.html;
    assert!(html.contains("<th>Check</th><th>Load case or combination</th><th>Status</th><th>Missing inputs</th>"), "{html}");
    assert!(html.contains(
        "<td>invented-check-a</td><td>load_case load:L-100</td><td>User-rule-check authority · User-rule checked (USER_RULE_CHECKED)</td><td>none</td>"
    ));
    assert!(html.contains(
        "<td>invented-check-a</td><td>combination combo:C-1</td><td>User-rule-check authority · User-rule failed (USER_RULE_FAILED)</td><td>none</td>"
    ));
    // The headline is unchanged; the note says what its counts count, once.
    assert!(html.contains("Aggregate status: User-rule-check authority · User-rule failed (USER_RULE_FAILED)"));
    assert_eq!(html.matches(USER_RULE_CHECK_COUNT_SCOPE_NOTE).count(), 1);
    assert_eq!(
        USER_RULE_CHECK_COUNT_SCOPE_NOTE,
        "Counts are per check and load case or combination; one check can contribute more than once."
    );
}

#[test]
fn an_outcome_recorded_without_basis_ref_renders_load_case_not_recorded() {
    // A 2.0.0 renderer input written before the member existed still reads.
    let mut value: serde_json::Value = serde_json::to_value(fixture_input()).unwrap();
    value["report_context"] = serde_json::to_value(invented_report_context(ReportExportContext::LocalPrivate)).unwrap();
    value["user_rule_checks"] = serde_json::json!({
        "rule_pack_id": "invented-rule-pack",
        "grammar_version": "1.0.0",
        "aggregate_status": "USER_RULE_CHECKED",
        "checks": [{"check_id": "invented-check-a", "status": "USER_RULE_CHECKED", "missing_inputs": []}]
    });
    let input: RenderableReportInput = serde_json::from_value(value).expect("an input without basis_ref reads");
    assert_eq!(input.user_rule_checks.as_ref().unwrap().checks[0].basis_ref, None);
    let outcome = render_calculation_report(&input);
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    assert!(outcome.html.contains(
        "<td>invented-check-a</td><td>load case not recorded</td><td>User-rule-check authority · User-rule checked (USER_RULE_CHECKED)</td><td>none</td>"
    ));
    // An absent member is not written back.
    let written = serde_json::to_value(&input).unwrap();
    assert!(written["user_rule_checks"]["checks"][0].get("basis_ref").is_none());
}

#[test]
fn an_empty_or_unknown_recorded_basis_blocks_export() {
    for (ref_type, ref_id) in [("load_case", " "), ("load_case", ""), ("support", "load:L-100")] {
        let mut input = fixture_input();
        input.report_context = Some(invented_report_context(ReportExportContext::LocalPrivate));
        let mut run = invented_rule_check_run();
        run.checks = vec![outcome_on("invented-check-a", ref_type, ref_id, UserRuleCheckStatus::UserRuleChecked)];
        run.aggregate_status = UserRuleCheckStatus::UserRuleChecked;
        input.user_rule_checks = Some(run);
        let outcome = render_calculation_report(&input);
        assert!(outcome.export_blocked, "{ref_type} {ref_id:?}");
        assert!(outcome
            .blocking_reasons
            .iter()
            .any(|reason| reason == "report context: USER-RULE-CHECK-BASIS-REF-INVALID"), "{:?}", outcome.blocking_reasons);
    }
}

// ---------------------------------------------------------------------------
// W-11 slice (c) (DEL-08-01 Design `exact-route-report-sections.md` §4.3,
// §6.2): FR-REP-003 records by disposition and context. The renderer refuses
// a record the library import and the package save block (`quarantined`,
// `rejected`; `protected_suspected` through the section validation), an
// `unknown` redistribution status through the section validation, the same
// rules on a library reference's record, an assumption's own blocking review,
// and any private record outside the local-private context. A `private_only`
// record renders only under `local_private`.
// ---------------------------------------------------------------------------

use open_pipe_stress_report_renderer::{
    provenance_disposition_blocking_reasons, LibraryReference, REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE,
    REPORT_PROVENANCE_DISPOSITION_BLOCKED,
};

const REMEDIATION: &str = "review that record's disposition in the model before reporting";
const VALUE_ID: &str = "material-provenance:material:invented-steel";

fn private_record(review: sections::ReviewStatus, redistribution: sections::RedistributionStatus) -> sections::Provenance {
    sections::Provenance {
        source_name: "Invented material datasheet".to_string(),
        source_location: "invented-datasheet-shelf".to_string(),
        source_license: "invented-license".to_string(),
        contributor: "invented contributor".to_string(),
        contributor_certification: "invented certification".to_string(),
        redistribution_status: redistribution,
        review_status: review,
        privacy_classification: sections::PrivacyClassification::PrivateProjectData,
    }
}

/// The fixture input with one material record (note and value), under
/// `context` (`None`: no report context).
fn with_material_record(record: sections::Provenance, context: Option<ReportExportContext>) -> RenderableReportInput {
    let mut input = fixture_input();
    input.report_context = context.map(invented_report_context);
    let value_review = record.review_status;
    input.report_sections.provenance_notes.push(record.clone());
    input.report_sections.user_supplied_values.push(sections::UserSuppliedValue {
        value_id: VALUE_ID.to_string(),
        value_category: "report_metadata".to_string(),
        source: sections::Reference::new("material", "material:invented-steel"),
        quantity: None,
        provenance: record,
        privacy_classification: sections::PrivacyClassification::PrivateProjectData,
        required_for: vec![sections::RequiredFor::Reporting, sections::RequiredFor::HumanReview],
        review_status: value_review,
        missing_data_finding: false,
    });
    input
}

fn disposition_reasons(outcome: &open_pipe_stress_report_renderer::RenderOutcome) -> Vec<&str> {
    outcome
        .blocking_reasons
        .iter()
        .map(String::as_str)
        .filter(|reason| reason.starts_with("report provenance: ") || reason.starts_with("library references: "))
        .collect()
}

fn blocked(place: &str, status: &str) -> String {
    format!("report provenance: {REPORT_PROVENANCE_DISPOSITION_BLOCKED} ({place} review_status {status}; {REMEDIATION})")
}

#[test]
fn fr_rep_003_records_render_only_locally_and_refuse_blocking_dispositions_in_every_context() {
    use sections::RedistributionStatus as R;
    use sections::ReviewStatus as V;
    let local = Some(ReportExportContext::LocalPrivate);
    for review in [V::Pending, V::Accepted] {
        let outcome = render_calculation_report(&with_material_record(private_record(review, R::PrivateOnly), local));
        assert!(!outcome.export_blocked, "{review:?}: {:?}", outcome.blocking_reasons);
        assert!(outcome.html.contains(VALUE_ID));
        assert!(outcome.html.contains("Invented material datasheet"));
    }
    // Outside local_private (public_report, or no context at all) private
    // record content refuses, whatever its review.
    for (context, named) in [(Some(ReportExportContext::PublicReport), "public_report"), (None, "not recorded")] {
        for review in [V::Pending, V::Accepted] {
            let outcome = render_calculation_report(&with_material_record(private_record(review, R::PrivateOnly), context));
            assert!(outcome.export_blocked, "{named} {review:?}");
            assert_eq!(
                disposition_reasons(&outcome),
                [format!("report provenance: {REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE} (private record content at provenance_notes[2] under export context {named}; render it only under the local-private selection)")]
            );
        }
        // An invented public fixture still renders publicly.
        let mut public = fixture_input();
        public.report_context = context.map(invented_report_context);
        assert!(!render_calculation_report(&public).export_blocked, "{named}");
    }
    for context in [local, Some(ReportExportContext::PublicReport), None] {
        for (review, status) in [(V::Quarantined, "quarantined"), (V::Rejected, "rejected")] {
            let input = with_material_record(private_record(review, R::PrivateOnly), context);
            let outcome = render_calculation_report(&input);
            assert!(outcome.export_blocked, "{status}");
            let reasons = disposition_reasons(&outcome);
            assert_eq!(
                reasons[..3],
                [blocked("provenance_notes[2]", status), blocked(&format!("user_supplied_values[1] {VALUE_ID} provenance"), status), blocked(&format!("user_supplied_values[1] {VALUE_ID}"), status)],
                "{status}"
            );
            assert_eq!(reasons.len(), if context == local { 3 } else { 4 });
            // The value's own status blocks even where its record does not.
            let mut value_only = fixture_input();
            value_only.report_sections.user_supplied_values[0].review_status = review;
            assert_eq!(
                provenance_disposition_blocking_reasons(&value_only),
                [blocked("user_supplied_values[0] component-provenance:component:C-110", status)]
            );
        }
        // protected_suspected: the section validation refuses in every context.
        let outcome = render_calculation_report(&with_material_record(private_record(V::Pending, R::ProtectedSuspected), context));
        assert!(outcome.export_blocked);
        assert!(has_reason(&outcome, "section validation: REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED"), "{:?}", outcome.blocking_reasons);
        // unknown: blocks outside local_private (owner ruling "C1 b").
        if context != local {
            let outcome = render_calculation_report(&with_material_record(private_record(V::Pending, R::Unknown), context));
            assert!(outcome.export_blocked);
            assert!(has_reason(&outcome, "section validation: REPORT_SECTION_PROVENANCE_UNRESOLVED"), "{:?}", outcome.blocking_reasons);
        }
    }
}

/// Owner ruling on #1639 ("C1 b, C2 a", owner, 2026-10-26): the same record
/// with an unresolved (`unknown` or `TBD`) status renders in a local-private
/// report with a non-blocking warning in the warnings section naming the
/// record and the field, and blocks under `public_report` and without a
/// context. This fails without the context split.
#[test]
fn an_unresolved_record_warns_in_local_private_and_blocks_elsewhere() {
    use sections::PrivacyClassification as P;
    use sections::RedistributionStatus as R;
    use sections::ReviewStatus as V;
    let cases: [(fn(&mut sections::Provenance), &str); 4] = [
        (|p| p.redistribution_status = R::Unknown, "redistribution status unknown"),
        (|p| p.redistribution_status = R::Tbd, "redistribution status TBD"),
        (|p| p.review_status = V::Tbd, "review status TBD"),
        (|p| p.privacy_classification = P::Tbd, "privacy classification TBD"),
    ];
    for (edit, field) in cases {
        let mut record = private_record(V::Pending, R::PrivateOnly);
        edit(&mut record);
        let local = render_calculation_report(&with_material_record(record.clone(), Some(ReportExportContext::LocalPrivate)));
        assert!(!local.export_blocked, "{field}: {:?}", local.blocking_reasons);
        let row = format!(
            "<tr><td>REPORT_SECTION_PROVENANCE_UNRESOLVED</td><td>warning</td><td>user_supplied_value {VALUE_ID}: {field}; the source&#39;s status is not recorded, so this record&#39;s provenance is unresolved in this local-private report.</td>"
        );
        assert!(local.html.contains(&row), "{field}: {}", local.html);
        assert!(local.html.contains("<td>REPORT_SECTION_PROVENANCE_UNRESOLVED</td><td>warning</td><td>provenance Invented material datasheet: "), "{field}");
        assert!(local.section_validation_diagnostics.iter().all(|item| item.severity != "blocking"), "{field}");
        for context in [Some(ReportExportContext::PublicReport), None] {
            let refused = render_calculation_report(&with_material_record(record.clone(), context));
            assert!(refused.export_blocked, "{field} {context:?}");
            assert!(has_reason(&refused, "section validation: REPORT_SECTION_PROVENANCE_UNRESOLVED"), "{field} {context:?}: {:?}", refused.blocking_reasons);
        }
    }
    // A record without an unresolved status adds no warning row.
    let clean = render_calculation_report(&with_material_record(private_record(V::Pending, R::PrivateOnly), Some(ReportExportContext::LocalPrivate)));
    assert!(!clean.html.contains("REPORT_SECTION_PROVENANCE_UNRESOLVED"));
}

#[test]
fn an_assumption_with_a_blocking_review_refuses() {
    for (review, status) in [(sections::ReviewStatus::Quarantined, "quarantined"), (sections::ReviewStatus::Rejected, "rejected")] {
        let mut input = fixture_input();
        input.report_sections.assumptions.push(sections::Assumption {
            assumption_id: "assumption:invented-1".to_string(),
            owner: "invented owner".to_string(),
            source: sections::Reference::new("model", "invented-model"),
            affected_scope: sections::Reference::new("model", "invented-model"),
            statement: "Invented assumption.".to_string(),
            review_status: review,
            expiration_or_re_review_trigger: None,
            effect: sections::ReportEffect {
                mechanics_solve_qualified: true,
                user_rule_check_qualified: false,
                report_completeness: sections::ReportCompleteness::Qualified,
                human_review_required: true,
            },
            provenance: invented_section_provenance(),
        });
        let outcome = render_calculation_report(&input);
        assert!(outcome.export_blocked, "{status}");
        assert_eq!(disposition_reasons(&outcome), [blocked("assumptions[0] assumption:invented-1", status)]);
    }
}

#[test]
fn a_library_reference_record_is_judged_by_the_section_rules_and_the_disposition_rule() {
    let library = |provenance: sections::Provenance| LibraryReference {
        reference_id: "library-ref:1".to_string(),
        library_kind: "material".to_string(),
        library_id: "invented-material-library".to_string(),
        library_version: "not recorded".to_string(),
        record_id: "not recorded".to_string(),
        entity_refs: vec!["material:invented-steel".to_string()],
        provenance,
    };
    let mut clean = fixture_input();
    clean.report_context = Some(invented_report_context(ReportExportContext::LocalPrivate));
    clean.library_references = Some(vec![library(private_record(sections::ReviewStatus::Pending, sections::RedistributionStatus::PrivateOnly))]);
    assert!(provenance_disposition_blocking_reasons(&clean).is_empty());
    assert!(!render_calculation_report(&clean).export_blocked);
    let section_rule = |code: &str| format!("library references: {code} (library_references[0] referenced by material:invented-steel; {REMEDIATION})");
    let cases: [(fn(&mut sections::Provenance), String); 6] = [
        (|p| p.review_status = sections::ReviewStatus::Quarantined, blocked("library_references[0] (referenced by material:invented-steel) provenance", "quarantined")),
        (|p| p.review_status = sections::ReviewStatus::Rejected, blocked("library_references[0] (referenced by material:invented-steel) provenance", "rejected")),
        (|p| p.redistribution_status = sections::RedistributionStatus::ProtectedSuspected, section_rule("REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED")),
        (|p| p.privacy_classification = sections::PrivacyClassification::ProtectedSuspected, section_rule("REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED")),
        (|p| p.source_name = " ".to_string(), section_rule("REPORT_SECTION_PROVENANCE_INCOMPLETE")),
        (|p| p.review_status = sections::ReviewStatus::Rejected, blocked("library_references[0] (referenced by material:invented-steel) provenance", "rejected")),
    ];
    for (edit, reason) in cases {
        let mut input = clean.clone();
        edit(&mut input.library_references.as_mut().unwrap()[0].provenance);
        let outcome = render_calculation_report(&input);
        assert!(outcome.export_blocked, "{reason}");
        assert_eq!(disposition_reasons(&outcome), [reason]);
    }
    // An unknown library record warns under local_private (owner ruling "C1
    // b") and blocks elsewhere, as a report-section record does.
    let mut unknown = clean.clone();
    unknown.library_references.as_mut().unwrap()[0].provenance.redistribution_status = sections::RedistributionStatus::Unknown;
    let local = render_calculation_report(&unknown);
    assert!(!local.export_blocked, "{:?}", local.blocking_reasons);
    assert!(local.html.contains("<td>REPORT_SECTION_PROVENANCE_UNRESOLVED</td><td>warning</td><td>library_reference library-ref:1 (referenced by material:invented-steel): redistribution status unknown; "));
    unknown.report_context = Some(invented_report_context(ReportExportContext::PublicReport));
    assert!(disposition_reasons(&render_calculation_report(&unknown)).contains(&section_rule("REPORT_SECTION_PROVENANCE_UNRESOLVED").as_str()));
    unknown.report_context = None;
    assert!(disposition_reasons(&render_calculation_report(&unknown)).contains(&section_rule("REPORT_SECTION_PROVENANCE_UNRESOLVED").as_str()));
    // A private library record outside local_private refuses too.
    let mut public = clean.clone();
    public.report_context = Some(invented_report_context(ReportExportContext::PublicReport));
    assert_eq!(
        disposition_reasons(&render_calculation_report(&public)),
        [format!("report provenance: {REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE} (private record content at library_references[0].provenance under export context public_report; render it only under the local-private selection)")]
    );
}

#[test]
fn inputs_without_a_blocking_disposition_have_no_provenance_reason() {
    // The existing invented inputs (accepted, invented public records) and the
    // exact input (local private) carry none.
    assert!(provenance_disposition_blocking_reasons(&fixture_input()).is_empty());
    assert!(provenance_disposition_blocking_reasons(&exact_input()).is_empty());
}

/// The desktop builder's representative records (`renderedReport.test.tsx`
/// writes them; `tests/test_report_sections_provenance_records.py` validates
/// them against the schema).
const PROVENANCE_RECORDS: &str =
    include_str!("../../../../fixtures/reports/invented/report_sections_provenance_records.json");

#[test]
fn the_desktop_records_validate_and_refuse_only_their_blocking_dispositions() {
    let fixture: serde_json::Value = serde_json::from_str(PROVENANCE_RECORDS).unwrap();
    let records: sections::ReportSections = serde_json::from_value(fixture["report_sections"].clone()).expect("the desktop records deserialize");
    // A copied accepted review never makes a missing-data value accepted; the
    // only blocking section diagnostics are the nested protected_suspected
    // record's, carried from its provenance_record.
    let validation = sections::validate_report_sections(&records);
    assert!(
        validation.diagnostics.iter().all(|item| item.code != "REPORT_SECTION_MISSING_VALUE_MARKED_ACCEPTED"),
        "{:?}",
        validation.diagnostics
    );
    let blocking: Vec<(&str, &str)> = validation
        .diagnostics
        .iter()
        .filter(|item| item.severity == sections::Severity::Blocking)
        .map(|item| (item.code.as_str(), item.affected_object.ref_id.as_str()))
        .collect();
    assert_eq!(
        blocking,
        [
            ("REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED", "Invented suspected table"),
            ("REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED", "section-provenance:section:nested-suspected"),
        ]
    );
    let mut input = fixture_input();
    input.report_context = Some(invented_report_context(ReportExportContext::LocalPrivate));
    input.report_sections = records;
    let outcome = render_calculation_report(&input);
    assert!(outcome.export_blocked);
    let value = |index: usize, id: &str| format!("user_supplied_values[{index}] {id}");
    assert_eq!(
        disposition_reasons(&outcome),
        [
            blocked("provenance_notes[2]", "quarantined"),
            blocked("provenance_notes[7]", "rejected"),
            blocked("provenance_notes[9]", "rejected"),
            blocked("provenance_notes[10]", "quarantined"),
            blocked(&format!("{} provenance", value(1, "component-provenance:component:C-999")), "quarantined"),
            blocked(&value(1, "component-provenance:component:C-999"), "quarantined"),
            blocked(&format!("{} provenance", value(6, "section-provenance:section:rejected")), "rejected"),
            blocked(&value(6, "section-provenance:section:rejected"), "rejected"),
            blocked(&format!("{} provenance", value(8, "section-provenance:section:nested-rejected")), "rejected"),
            blocked(&value(8, "section-provenance:section:nested-rejected"), "rejected"),
            blocked(&format!("{} provenance", value(9, "section-provenance:section:note-quarantined")), "quarantined"),
            blocked(&value(9, "section-provenance:section:note-quarantined"), "quarantined"),
        ]
    );
    assert!(outcome.blocking_reasons.iter().any(|reason| reason == "section validation: REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED"));
    // Without the blocking records the same report renders.
    let blocks = |record: &sections::Provenance| {
        matches!(record.review_status, sections::ReviewStatus::Quarantined | sections::ReviewStatus::Rejected)
            || record.redistribution_status == sections::RedistributionStatus::ProtectedSuspected
    };
    let mut clean = input.clone();
    clean.report_sections.provenance_notes.retain(|note| !blocks(note));
    clean.report_sections.user_supplied_values.retain(|value| !blocks(&value.provenance));
    let rendered = render_calculation_report(&clean);
    assert!(!rendered.export_blocked, "{:?}", rendered.blocking_reasons);
    for text in ["material-provenance:material:accepted", "Invented accepted datasheet", "material provenance missing", "MATERIAL_PROVENANCE_MISSING"] {
        assert!(rendered.html.contains(text), "{text}");
    }
    // The same records under public_report refuse as private record content.
    let mut public = clean.clone();
    public.report_context = Some(invented_report_context(ReportExportContext::PublicReport));
    assert!(render_calculation_report(&public).blocking_reasons.iter().any(|reason| reason.contains(REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE)));
}

/// Owner ruling on #1639 ("C1 b, C2 a", owner, 2026-10-26): a user-typed path
/// in a record's `source_location` is shown as written in a local-private
/// report, and a direct native render under `public_report` or without a
/// context refuses the record, so the document reaches no sink.
#[test]
fn a_typed_path_stays_local() {
    let path = "/Users/invented-user/datasheets/steel-a.pdf";
    let mut record = private_record(sections::ReviewStatus::Pending, sections::RedistributionStatus::PrivateOnly);
    record.source_location = path.to_string();
    let local = render_calculation_report(&with_material_record(record.clone(), Some(ReportExportContext::LocalPrivate)));
    assert!(!local.export_blocked, "{:?}", local.blocking_reasons);
    assert!(local.html.contains(&format!("<td>{path}</td>")));
    for context in [Some(ReportExportContext::PublicReport), None] {
        let refused = render_calculation_report(&with_material_record(record.clone(), context));
        assert!(refused.export_blocked, "{context:?}");
        assert!(refused.blocking_reasons.iter().any(|reason| reason.contains(REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE)), "{context:?}");
    }
}

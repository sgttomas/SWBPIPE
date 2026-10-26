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

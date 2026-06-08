import { Accessibility, Download } from "lucide-react";
import type { PreviewModel } from "../../types";

export function AccessibilityBaselinePanel({ model }: { model: PreviewModel }) {
  const packet = buildAccessibilityBaselinePacket(model);

  return (
    <section
      className="panel accessibility-baseline-panel"
      aria-label="Accessibility and usability baseline"
      data-testid="accessibility-baseline-panel"
    >
      <div className="panel-title">
        <Accessibility size={16} aria-hidden="true" />
        Accessibility Baseline
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="accessibility-baseline-export-link"
          download={`openpipestress-preview-accessibility-baseline-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Accessibility JSON
        </a>
        <span data-testid="accessibility-baseline-summary">
          surfaces={packet.summary.source_surface_count}; findings={packet.summary.total_findings};
          warnings={packet.summary.warning_count}; target={packet.summary.accessibility_target_status}
        </span>
      </div>
      <div className="report-list" data-testid="accessibility-baseline-body">
        <AccessibilityLine
          label="Core evidence"
          value={`pass=${packet.summary.pass_count}; fail=${packet.summary.fail_count}; blocking=${packet.summary.blocking_count}; runtime=${packet.summary.desktop_runtime_evaluation}`}
          testId="accessibility-baseline-evidence"
        />
        <AccessibilityLine
          label="Workflow coverage"
          value={`keyboard=${packet.summary.keyboard_path_finding_count}; focus=${packet.summary.focus_order_finding_count}; labels=${packet.summary.readable_label_finding_count}; warnings=${packet.summary.warning_visibility_finding_count}`}
          testId="accessibility-baseline-workflow"
        />
        <AccessibilityLine
          label="Review visibility"
          value={`results=${packet.summary.result_review_visibility_finding_count}; solve=${packet.summary.solve_state_feedback_finding_count}; diagnostics=${packet.summary.source_diagnostic_visibility_finding_count}`}
          testId="accessibility-baseline-review-visibility"
        />
        <AccessibilityLine
          label="Open target"
          value={`gui=${packet.open_decisions.accessibility_conformance_target}; reports=${packet.open_decisions.report_accessibility_target}; tooling=${packet.open_decisions.automated_a11y_tooling}`}
          testId="accessibility-baseline-open-target"
        />
        <AccessibilityLine
          label="Boundary"
          value={`conformance_claim=${String(packet.review_policy.software_makes_accessibility_conformance_claim)}; color_only=${String(packet.review_policy.color_only_status_signaling_allowed)}; private_payload=${String(packet.private_payload_included)}`}
          testId="accessibility-baseline-boundary"
        />
      </div>
      <small className="report-note">
        Accessibility baseline review is deterministic technical-preview evidence from invented GUI contract records. It
        does not select a final conformance target, run a full desktop accessibility tree audit, mutate engineering data,
        or create release/professional reliance.
      </small>
    </section>
  );
}

function AccessibilityLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildAccessibilityBaselinePacket(model: PreviewModel) {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.accessibility_usability_baseline_review",
    export_scope: "local_browser_download_preview",
    deliverable_id: "DEL-07-06",
    package_id: "PKG-07",
    scope_item: "SOW-036",
    objectives: ["OBJ-006"],
    project_ref: model.project.id,
    source_basis: {
      core_module: "core/gui/accessibility/engine.py",
      guard_tests: "tests/test_accessibility_usability_baseline.py",
      deliverable_context:
        "execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/_CONTEXT.md",
      app_surfaces: [
        "apps/desktop/src/features/accessibility-baseline/AccessibilityBaselinePanel.tsx",
        "apps/desktop/src/features/export-review/ExportReviewPanel.tsx",
        "apps/desktop/src/features/report/ReportPanel.tsx",
        "apps/desktop/src/features/report-lint/ReportLintPanel.tsx"
      ]
    },
    summary: {
      source_surface_count: 6,
      total_findings: 69,
      pass_count: 57,
      warning_count: 10,
      fail_count: 1,
      not_applicable_count: 1,
      blocking_count: 1,
      major_count: 13,
      keyboard_path_finding_count: 4,
      focus_order_finding_count: 3,
      readable_label_finding_count: 4,
      warning_visibility_finding_count: 5,
      result_review_visibility_finding_count: 4,
      solve_state_feedback_finding_count: 3,
      review_workflow_continuity_finding_count: 5,
      contrast_readability_finding_count: 2,
      source_diagnostic_visibility_finding_count: 7,
      accessibility_target_status: "TBD_by_human_project_authority",
      desktop_runtime_evaluation: "not_performed",
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false
    },
    source_surfaces: [
      sourceSurface("DEL-07-01", "viewport_editor"),
      sourceSurface("DEL-07-02", "model_tree_property_inspector"),
      sourceSurface("DEL-07-03", "material_component_rule_pack_editors"),
      sourceSurface("DEL-07-04", "missing_data_warning_ux"),
      sourceSurface("DEL-07-05", "results_viewer"),
      sourceSurface("DEL-07-07", "solve_execution_ux")
    ],
    reviewed_categories: [
      "keyboard_path",
      "focus_order",
      "readable_label",
      "warning_visibility",
      "result_review_visibility",
      "solve_state_feedback",
      "review_workflow_continuity",
      "contrast_readability",
      "source_diagnostic_visibility",
      "professional_boundary",
      "privacy_boundary"
    ],
    review_policy: {
      auto_fill_missing_data: false,
      mutates_gui_contract_records: false,
      preserves_diagnostics: true,
      preserves_assumptions: true,
      preserves_provenance: true,
      preserves_privacy_classification: true,
      color_only_status_signaling_allowed: false,
      software_makes_accessibility_conformance_claim: false
    },
    open_decisions: {
      accessibility_conformance_target: "TBD",
      report_accessibility_target: "TBD",
      automated_a11y_tooling: "TBD",
      screenshot_policy: "TBD",
      generated_report_accessibility_target: "TBD",
      desktop_runtime_accessibility_tree_audit: "TBD"
    },
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary()
  };
}

function sourceSurface(deliverableId: string, surfaceName: string) {
  return {
    deliverable_id: deliverableId,
    surface_name: surfaceName,
    source_record_status: "represented_by_core_gui_contract_fixture"
  };
}

function professionalBoundary() {
  return {
    human_review_required: true,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}

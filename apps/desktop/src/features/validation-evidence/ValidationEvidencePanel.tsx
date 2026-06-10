import { ClipboardCheck, Download } from "lucide-react";
import type { PreviewModel } from "../../types";

const MANUAL_SECTIONS = [
  "product_scope_and_limitations",
  "solver_theory_summary",
  "unit_and_schema_verification",
  "element_verification",
  "load_and_stress_recovery_verification",
  "nonlinear_support_verification",
  "rule_pack_evaluator_verification",
  "gui_workflow_validation",
  "report_reproducibility_validation",
  "known_limitations_and_open_issues"
] as const;

const EVIDENCE_AREAS = [
  "mechanics_benchmarks",
  "stress_recovery_benchmarks",
  "nonlinear_support_regression",
  "gui_workflow_validation",
  "report_protected_content_lint",
  "release_quality_evidence",
  "validation_evidence_bundles",
  "professional_boundary"
] as const;

const RELEASE_PROFILES = ["skeleton", "python", "security", "cargo", "all"] as const;
const REQUIRED_RELEASE_PATHS = [
  "docs/BUILD_AND_RELEASE.md",
  "docs/RELEASE_NOTES_TEMPLATE.md",
  "docs/RELEASE_QUALITY_GATES.md",
  "docs/VALIDATION_STRATEGY.md",
  "docs/IP_AND_DATA_BOUNDARY.md",
  "docs/PROFESSIONAL_BOUNDARY.md",
  "execution/_DAG/_LATEST.md"
] as const;

const OPEN_VALIDATION_DECISIONS = [
  "benchmark_tolerance_policy",
  "public_benchmark_source_acceptance",
  "release_label_policy",
  "gui_validation_evidence_requirements",
  "validation_evidence_bundle_storage"
] as const;

const RELEASE_GATE_FAMILIES = ["solver", "rule_engine", "gui", "report_template", "mixed"] as const;

export function ValidationEvidencePanel({ model }: { model: PreviewModel }) {
  const packet = buildValidationEvidencePacket(model);
  return (
    <section
      className="panel validation-evidence-panel"
      aria-label="Validation evidence review"
      data-testid="validation-evidence-panel"
    >
      <div className="panel-title">
        <ClipboardCheck size={16} aria-hidden="true" />
        Validation Evidence
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="validation-evidence-export-link"
          download={`openpipestress-preview-validation-evidence-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Evidence JSON
        </a>
        <span data-testid="validation-evidence-summary">
          manual={packet.summary.manual_section_count}; evidence={packet.summary.evidence_area_count}; profiles=
          {packet.summary.release_profile_count}; open_tbd={packet.summary.open_validation_decision_count}
        </span>
      </div>
      <div className="report-list" data-testid="validation-evidence-body">
        <ValidationLine
          label="Manual map"
          value={`sections=${packet.summary.manual_section_count}; states=${packet.validation_manual.evidence_states.join(
            ", "
          )}`}
          testId="validation-evidence-manual"
        />
        <ValidationLine
          label="Evidence inventory"
          value={`areas=${packet.summary.evidence_area_count}; gui=${packet.evidence_inventory.gui_workflow_validation.status}; bundles=${packet.evidence_inventory.validation_evidence_bundles.status}`}
          testId="validation-evidence-inventory"
        />
        <ValidationLine
          label="Release checks"
          value={`profiles=${packet.release_readiness_tool.profiles.join(", ")}; skeleton_checks=${packet.release_readiness_tool.skeleton_check_count}; dry_run_default=${String(
            packet.release_readiness_tool.dry_run_default
          )}`}
          testId="validation-evidence-release-checks"
        />
        <ValidationLine
          label="Release gates"
          value={`families=${packet.release_quality_gates.gate_families.join(", ")}; thresholds=${packet.release_quality_gates.final_threshold_policy}; release_authorized=${String(
            packet.release_quality_gates.release_publication_authorized
          )}`}
          testId="validation-evidence-gates"
        />
        <ValidationLine
          label="Boundary"
          value={`release_claim=${String(packet.release_or_professional_claim)}; professional_claim=${String(
            packet.professional_boundary.software_makes_approval_claim
          )}; private_payload=${String(packet.private_payload_included)}`}
          testId="validation-evidence-boundary"
        />
      </div>
      <small className="report-note">
        Validation evidence is local technical-preview review context only. It organizes software-quality evidence and
        unresolved decisions; it does not authorize release, certify engineering work, or make code-compliance claims.
      </small>
    </section>
  );
}

function ValidationLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildValidationEvidencePacket(model: PreviewModel) {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.validation_release_evidence_review",
    export_scope: "local_browser_download_preview",
    deliverable_refs: ["DEL-09-04", "DEL-09-05", "DEL-10-04", "DEL-09-01", "DEL-09-02", "DEL-09-03", "DEL-08-05"],
    package_refs: ["PKG-09", "PKG-10", "PKG-08"],
    scope_items: ["SOW-026", "SOW-027", "SOW-032", "SOW-043"],
    objectives: ["OBJ-008", "OBJ-011"],
    project_ref: model.project.id,
    source_basis: {
      validation_strategy: "docs/VALIDATION_STRATEGY.md",
      validation_manual: "docs/validation_manual/index.md",
      release_quality_gates: "docs/RELEASE_QUALITY_GATES.md",
      build_and_release: "docs/BUILD_AND_RELEASE.md",
      release_readiness_tool: "tools/release/check_release_readiness.py",
      guard_test: "tests/test_release_readiness_script.py",
      app_surface: "apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx"
    },
    summary: {
      manual_section_count: MANUAL_SECTIONS.length,
      evidence_area_count: EVIDENCE_AREAS.length,
      release_profile_count: RELEASE_PROFILES.length,
      required_release_path_count: REQUIRED_RELEASE_PATHS.length,
      skeleton_check_count: 2,
      open_validation_decision_count: OPEN_VALIDATION_DECISIONS.length,
      release_gate_family_count: RELEASE_GATE_FAMILIES.length
    },
    validation_manual: {
      manual_status: "draft_evidence_index",
      manual_sections: MANUAL_SECTIONS,
      evidence_states: ["PLANNED", "DRAFT_EVIDENCE", "MAINTAINER_REVIEWED", "BLOCKED", "TBD"],
      separates_mechanics_validation_rule_and_reliance: true,
      professional_reliance_outside_software_authority: true,
      open_decisions: OPEN_VALIDATION_DECISIONS
    },
    evidence_inventory: {
      mechanics_benchmarks: evidenceArea("DRAFT_EVIDENCE", "validation/benchmarks/mechanics/"),
      stress_recovery_benchmarks: evidenceArea("DRAFT_EVIDENCE", "validation/benchmarks/stress/"),
      nonlinear_support_regression: evidenceArea("DRAFT_EVIDENCE", "validation/benchmarks/nonlinear/"),
      gui_workflow_validation: evidenceArea("PLANNED", "apps/desktop/SMOKE.md"),
      report_protected_content_lint: evidenceArea("draft_review_evidence", "core/reporting/protected_content_linter/"),
      release_quality_evidence: evidenceArea("TBD", "docs/RELEASE_QUALITY_GATES.md"),
      validation_evidence_bundles: evidenceArea("TBD", "future evidence package location/format"),
      professional_boundary: evidenceArea("draft_policy_surface", "docs/PROFESSIONAL_BOUNDARY.md")
    },
    release_readiness_tool: {
      command: "python3 tools/release/check_release_readiness.py --profile skeleton",
      execute_command: "python3 tools/release/check_release_readiness.py --profile skeleton --execute",
      profiles: RELEASE_PROFILES,
      required_paths: REQUIRED_RELEASE_PATHS,
      skeleton_check_count: 2,
      latest_dag_dependency_edges: "execution/_DAG/DAG-006/DependencyEdges.csv",
      browser_panel_runs_tool: false,
      dry_run_default: true,
      external_services_required: false
    },
    release_quality_gates: {
      gate_families: RELEASE_GATE_FAMILIES,
      final_threshold_policy: "TBD",
      ci_provider: "TBD",
      release_matrix: "TBD",
      signing_notarization: "TBD",
      release_publication_authorized: false,
      waiver_can_authorize_protected_or_private_exposure: false,
      software_quality_evidence_only: true
    },
    gui_validation_context: {
      smoke_record_surface: "apps/desktop/SMOKE.md",
      current_tranche_smoke_record: "TP-MAC-77",
      browser_console_errors_allowed: false,
      screenshot_matrix_policy: "TBD",
      accessibility_target_policy: "TBD",
      workflow_validation_status: "technical_preview_smoke_evidence_available_not_release_gate"
    },
    data_boundary: model.data_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary()
  };
}

function evidenceArea(status: string, source: string) {
  return {
    status,
    source,
    release_readiness_claim: false,
    professional_reliance_claim: false
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

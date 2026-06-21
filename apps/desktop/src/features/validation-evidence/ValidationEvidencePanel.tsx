import { ClipboardCheck, Download } from "lucide-react";
import type { ObjectRef, PreviewModel } from "../../types";

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
          manual={packet.summary.manual_section_count}; evidence=
          {packet.summary.evidence_area_count}; profiles=
          {packet.summary.release_profile_count}; open_tbd=
          {packet.summary.open_validation_decision_count}
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
          label="Unit policy"
          value={validationEvidenceUnitPolicySummary(packet.unit_policy_evidence)}
          testId="validation-evidence-unit-policy"
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
  const unitPolicyEvidence = buildValidationEvidenceUnitPolicy(model);

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
      release_gate_family_count: RELEASE_GATE_FAMILIES.length,
      unit_bearing_record_count: unitPolicyEvidence.unit_bearing_record_count
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
    unit_policy_evidence: unitPolicyEvidence,
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
      current_tranche_smoke_record: "TP-MAC-83",
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

type ValidationEvidenceUnitPolicyEvidence = {
  evidence_id: string;
  unit_system_ref: ObjectRef;
  source_model_ref: ObjectRef;
  storage_convention: "entered_units_preserved";
  validation_evidence_unit_policy: string;
  model_units: Record<string, string>;
  unit_bearing_record_count: number;
  unit_and_schema_manual_section: string;
  conversion_policy: string;
  conversion_performed: false;
  release_gate_threshold_policy: "TBD";
  decision_basis_refs: ObjectRef[];
  protected_content_included: false;
  private_payload_included: false;
};

function buildValidationEvidenceUnitPolicy(model: PreviewModel): ValidationEvidenceUnitPolicyEvidence {
  return {
    evidence_id: "unit-policy-evidence:validation-release-evidence-review",
    unit_system_ref: reference("UnitSystem", "unit-system:dec-018-si-dual-display"),
    source_model_ref: reference("Model", model.project.id),
    storage_convention: "entered_units_preserved",
    validation_evidence_unit_policy:
      "validation_evidence_records_project_unit_context_without_conversion_or_release_threshold_claim",
    model_units: sortedStringRecord(model.project.units),
    unit_bearing_record_count: countUnitBearingRecords(model),
    unit_and_schema_manual_section: "unit_and_schema_verification",
    conversion_policy: "validation_evidence_inventory_records_unit_context_without_conversion",
    conversion_performed: false,
    release_gate_threshold_policy: "TBD",
    decision_basis_refs: [
      reference("Decision", "DEC-018"),
      reference("Deliverable", "DEL-02-02"),
      reference("Deliverable", "DEL-09-05")
    ],
    protected_content_included: false,
    private_payload_included: false
  };
}

function validationEvidenceUnitPolicySummary(evidence: ValidationEvidenceUnitPolicyEvidence): string {
  return [
    `model=${formatUnitRecord(evidence.model_units)}`,
    `records=${evidence.unit_bearing_record_count}`,
    `manual=${evidence.unit_and_schema_manual_section}`,
    `conversion=${String(evidence.conversion_performed)}`
  ].join("; ");
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

function countUnitBearingRecords(model: PreviewModel): number {
  const pipeSectionQuantities = model.pipe_segments.flatMap((segment) => Object.values(segment.section));
  const materialQuantities = (model.materials ?? []).flatMap((material) => [
    material.elastic_modulus,
    material.shear_modulus,
    material.thermal_expansion_coefficient
  ]);
  const componentQuantities = model.components.flatMap(componentUnitQuantities);
  const loadQuantities = model.load_cases.flatMap((loadCase) =>
    (loadCase.primitive_loads ?? []).map((primitiveLoad) => primitiveLoad.magnitude)
  );
  return [...pipeSectionQuantities, ...materialQuantities, ...componentQuantities, ...loadQuantities].filter(hasUnit)
    .length;
}

function componentUnitQuantities(component: PreviewModel["components"][number]): unknown[] {
  return [
    component.geometry?.bend_radius,
    component.geometry?.bend_angle,
    component.geometry?.branch_run_size,
    component.geometry?.branch_header_size,
    component.geometry?.branch_connection_angle,
    component.geometry?.branch_reinforcement_area,
    component.geometry?.rigid_body_length,
    component.geometry?.end_a_size,
    component.geometry?.end_b_size,
    component.geometry?.weight,
    component.geometry?.center_of_gravity,
    component.modifiers?.sif_user_value,
    component.modifiers?.branch_header_sif_user_value,
    component.modifiers?.branch_branch_sif_user_value,
    component.modifiers?.flexibility_factor_user_value,
    component.modifiers?.stiffness_scaling_user_value,
    component.modifiers?.linear_stiffness_user_value,
    component.modifiers?.rotational_stiffness_user_value
  ];
}

function hasUnit(value: unknown): value is { unit: string } {
  return (
    typeof value === "object" &&
    value !== null &&
    "unit" in value &&
    typeof (value as { unit?: unknown }).unit === "string" &&
    (value as { unit: string }).unit.length > 0
  );
}

function sortedStringRecord(record: Record<string, string>): Record<string, string> {
  return Object.fromEntries(
    Object.entries(record)
      .filter(([, value]) => typeof value === "string" && value.length > 0)
      .sort(([left], [right]) => left.localeCompare(right))
  );
}

function formatUnitRecord(units: Record<string, string>): string {
  const entries = Object.entries(units).sort(([left], [right]) => left.localeCompare(right));
  if (entries.length === 0) return "none";
  return entries.map(([key, value]) => `${key}=${value}`).join(",");
}

function reference(objectType: string, ref: string): ObjectRef {
  return { object_type: objectType, ref };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value
    .replace(/[^a-z0-9]+/gi, "-")
    .replace(/^-+|-+$/g, "")
    .toLowerCase();
}

import { Ban, Download } from "lucide-react";
import type { Diagnostic, MechanicsResult, PreviewModel } from "../../types";

type GuiWarningClass =
  | "SOLVE_BLOCKING"
  | "RULE_CHECK_BLOCKING"
  | "PROVENANCE_WARNING"
  | "ASSUMPTION_WARNING"
  | "NONLINEAR_WARNING"
  | "IP_BOUNDARY_WARNING";

type MissingDataWarning = {
  warning_id: string;
  warning_class: GuiWarningClass;
  local_contract_class: string;
  severity: Diagnostic["severity"];
  analysis_status: string[];
  affected_refs: string[];
  source_refs: string[];
  message: string;
  remediation: string;
  blocks_mechanics_solve: boolean;
  blocks_rule_check: boolean;
  qualifies_mechanics_results: boolean;
  private_data_required: boolean;
  protected_content_required: boolean;
  source_status: string;
  assistive_text_available: boolean;
};

export function MissingDataBlockingPanel({
  model,
  result
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
}) {
  const packet = buildMissingDataBlockingPacket({ model, result });

  return (
    <section className="panel missing-data-panel" aria-label="Missing-data blocking review" data-testid="missing-data-panel">
      <div className="panel-title">
        <Ban size={16} aria-hidden="true" />
        Missing Data Blocking
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="missing-data-export-link"
          download={`openpipestress-preview-missing-data-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Warning JSON
        </a>
        <span data-testid="missing-data-summary">
          available; classes={packet.summary.warning_class_count}; active={packet.summary.active_warning_count};
          solve_blocked={String(packet.summary.mechanics_solve_blocked)}; rule_blocked=
          {String(packet.summary.rule_check_blocked)}; private_payload={String(packet.private_payload_included)}
        </span>
      </div>
      <div className="report-list" data-testid="missing-data-body">
        <MissingDataLine
          label="Class coverage"
          value={packet.class_inventory.map((item) => `${item.warning_class}:${item.status}`).join("; ")}
          testId="missing-data-class-coverage"
        />
        <MissingDataLine
          label="Status separation"
          value={`mechanics=${packet.status_inputs.mechanics}; rule_check=${packet.status_inputs.rule_check}; mechanics_reviewable=${String(packet.summary.mechanics_results_reviewable)}; mechanics_qualified=${String(packet.summary.mechanics_results_qualified_by_rule_inputs)}`}
          testId="missing-data-status-separation"
        />
        <MissingDataLine
          label="Blocking"
          value={`solve=${packet.summary.solve_blocking_count}; rule=${packet.summary.rule_check_blocking_count}; ids=${packet.blocking_summary.blocking_warning_ids.join(",") || "none"}`}
          testId="missing-data-blocking"
        />
        <MissingDataLine
          label="Assistive fields"
          value={`${packet.assistive_contract.required_text_fields.join(",")}; color_only=${String(packet.assistive_contract.color_only_signaling_allowed)}`}
          testId="missing-data-assistive"
        />
        <MissingDataLine
          label="Boundary"
          value={`silent_defaults=${String(packet.summary.silent_defaults_used)}; auto_fill=${String(packet.auto_fill_missing_data)}; protected=${String(packet.protected_content_included)}; compliance=${String(packet.professional_boundary.software_makes_compliance_claim)}`}
          testId="missing-data-boundary"
        />
      </div>
      <div className="operation-record-list" data-testid="missing-data-warnings">
        {packet.warnings.map((warning) => (
          <article
            className="operation-record"
            data-testid={`missing-data-warning-${safeFileToken(warning.warning_id)}`}
            key={warning.warning_id}
          >
            <strong>{warning.warning_class}</strong>
            <small>
              {warning.severity}; solve_blocking={String(warning.blocks_mechanics_solve)}; rule_blocking=
              {String(warning.blocks_rule_check)}
            </small>
            <p>
              {warning.message} {warning.remediation}
            </p>
          </article>
        ))}
      </div>
      <small className="report-note">
        Missing-data warnings distinguish mechanics solve readiness from user-rule checking. They do not fill missing
        engineering values and do not certify, approve, seal, authenticate, or declare code compliance.
      </small>
    </section>
  );
}

function MissingDataLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildMissingDataBlockingPacket({
  model,
  result
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
}) {
  const diagnostics = [...model.diagnostics, ...(result?.diagnostics ?? [])];
  const statusInputs = {
    mechanics: result?.status.mechanics ?? model.analysis_status.mechanics,
    rule_check: result?.status.rule_check ?? model.analysis_status.rule_check,
    professional_acceptance: result?.status.professional_acceptance ?? model.analysis_status.professional_acceptance
  };
  const warnings = buildWarnings({ model, result, diagnostics, statusInputs });
  const blockingWarnings = warnings.filter((warning) => warning.blocks_mechanics_solve || warning.blocks_rule_check);
  const classInventory = warningClassInventory(warnings);
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.missing_data_warning_blocking_review",
    deliverable_id: "DEL-07-04",
    package_id: "PKG-07",
    scope_item: "SOW-022",
    objectives: ["OBJ-006", "OBJ-011"],
    warning_set_id: `warning-set:${safeRefToken(model.project.id)}:desktop-preview`,
    project_ref: model.project.id,
    run_ref: result?.run_id ?? "not generated",
    status_inputs: statusInputs,
    class_inventory: classInventory,
    warnings,
    blocking_summary: {
      has_blocking_items: blockingWarnings.length > 0,
      blocking_warning_ids: blockingWarnings.map((warning) => warning.warning_id).sort(),
      solve_blocking_warning_ids: warnings.filter((warning) => warning.blocks_mechanics_solve).map((warning) => warning.warning_id).sort(),
      rule_check_blocking_warning_ids: warnings.filter((warning) => warning.blocks_rule_check).map((warning) => warning.warning_id).sort()
    },
    summary: {
      warning_class_count: classInventory.length,
      active_warning_count: warnings.length,
      blocking_warning_count: blockingWarnings.length,
      solve_blocking_count: warnings.filter((warning) => warning.blocks_mechanics_solve).length,
      rule_check_blocking_count: warnings.filter((warning) => warning.blocks_rule_check).length,
      provenance_warning_count: warnings.filter((warning) => warning.warning_class === "PROVENANCE_WARNING").length,
      assumption_warning_count: warnings.filter((warning) => warning.warning_class === "ASSUMPTION_WARNING").length,
      nonlinear_warning_count: warnings.filter((warning) => warning.warning_class === "NONLINEAR_WARNING").length,
      ip_boundary_warning_count: warnings.filter((warning) => warning.warning_class === "IP_BOUNDARY_WARNING").length,
      mechanics_solve_blocked: warnings.some((warning) => warning.blocks_mechanics_solve),
      rule_check_blocked: warnings.some((warning) => warning.blocks_rule_check),
      mechanics_results_reviewable: Boolean(result && !statusInputs.mechanics.toLowerCase().includes("incomplete")),
      mechanics_results_qualified_by_rule_inputs: Boolean(result && warnings.some((warning) => warning.blocks_rule_check)),
      silent_defaults_used: false,
      bundled_code_values_used: false,
      compliance_claim_made: false
    },
    analysis_boundary_contract: {
      contract_ref: "DEL-02-03",
      authority_model: "mechanics_rule_human_acceptance_separated",
      human_acceptance_record_policy: "external_hash_bound_human_record_only",
      gui_warning_class_status_map: guiWarningClassStatusMap(),
      local_warning_class_status_map: localWarningClassStatusMap()
    },
    assistive_contract: {
      required_text_fields: ["class", "severity", "affected_refs", "message", "remediation"],
      color_only_signaling_allowed: false,
      warning_list_text_available: true,
      export_json_available: true
    },
    source_of_truth_boundary: {
      surface: "DEL-07-04 missing_data_warning_blocking_review",
      gui_is_source_of_truth: false,
      mutation_route: "diagnostic_display_only_no_model_defaulting",
      direct_model_mutation_allowed: false,
      accepted_model_state_mutated: false
    },
    auto_fill_missing_data: false,
    data_boundary: model.data_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary(),
    provenance: previewProvenance()
  };
}

function buildWarnings({
  model,
  result,
  diagnostics,
  statusInputs
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  diagnostics: Diagnostic[];
  statusInputs: { mechanics: string; rule_check: string; professional_acceptance: string };
}): MissingDataWarning[] {
  const warnings: MissingDataWarning[] = [];
  const normalizedMechanics = statusInputs.mechanics.toLowerCase();
  if (normalizedMechanics.includes("incomplete") || diagnostics.some((item) => item.severity === "blocking")) {
    warnings.push(warning({
      warning_id: "solve-required-physical-inputs",
      warning_class: "SOLVE_BLOCKING",
      local_contract_class: "solve_required",
      severity: "blocking",
      analysis_status: ["MODEL_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"],
      affected_refs: [model.project.id],
      source_refs: [`analysis_status.mechanics:${statusInputs.mechanics}`],
      message: "Mechanics solve-required data is incomplete.",
      remediation: "Provide explicit geometry, support, load, material, section, and unit data before solving.",
      blocks_mechanics_solve: true,
      blocks_rule_check: true,
      qualifies_mechanics_results: false,
      private_data_required: false,
      source_status: "missing"
    }));
  }

  if (isRuleIncomplete(statusInputs.rule_check)) {
    warnings.push(warning({
      warning_id: "rule-check-required-inputs",
      warning_class: "RULE_CHECK_BLOCKING",
      local_contract_class: "code_check_required",
      severity: "warning",
      analysis_status: ["RULE_INPUTS_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"],
      affected_refs: [model.project.id],
      source_refs: [
        `analysis_status.rule_check:${statusInputs.rule_check}`,
        ...diagnostics.filter((item) => item.code.includes("RULE_INPUT")).map((item) => item.id ?? item.code)
      ],
      message: "User-rule required inputs are absent, so rule-check pass/fail and ratios stay blocked.",
      remediation: "Provide private rule-pack criteria, user-supplied values, provenance, and checksums before rule-check reliance.",
      blocks_mechanics_solve: false,
      blocks_rule_check: true,
      qualifies_mechanics_results: Boolean(result),
      private_data_required: true,
      source_status: "missing"
    }));
  }

  const componentRefs = model.components
    .filter((item) => item.provenance.toLowerCase().includes("no_flexibility_factor"))
    .map((item) => item.id);
  if (componentRefs.length > 0) {
    warnings.push(warning({
      warning_id: "component-modifier-provenance",
      warning_class: "PROVENANCE_WARNING",
      local_contract_class: "provenance_missing",
      severity: "warning",
      analysis_status: ["HUMAN_REVIEW_REQUIRED"],
      affected_refs: componentRefs,
      source_refs: componentRefs.map((ref) => `${ref}.provenance`),
      message: "Component SIF or flexibility-factor provenance is absent from the public preview.",
      remediation: "Keep component modifiers user-supplied or privately imported; do not infer protected table values.",
      blocks_mechanics_solve: false,
      blocks_rule_check: false,
      qualifies_mechanics_results: true,
      private_data_required: true,
      source_status: "missing"
    }));
  }

  const codeBasisRefs = [
    ...model.load_cases
      .filter((item) => item.provenance.toLowerCase().includes("no_code_combination"))
      .map((item) => item.id),
    ...(model.combinations ?? [])
      .filter((item) => item.provenance.toLowerCase().includes("no_code_default"))
      .map((item) => item.id)
  ];
  if (codeBasisRefs.length > 0) {
    warnings.push(warning({
      warning_id: "code-combination-criteria-absent",
      warning_class: "RULE_CHECK_BLOCKING",
      local_contract_class: "code_check_required",
      severity: "warning",
      analysis_status: ["RULE_INPUTS_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"],
      affected_refs: codeBasisRefs,
      source_refs: codeBasisRefs.map((ref) => `${ref}.provenance`),
      message: "Load cases and combinations are mechanics preview inputs, not user-code rule combinations.",
      remediation: "Supply private owner/code combination criteria before reporting user-rule checked status.",
      blocks_mechanics_solve: false,
      blocks_rule_check: true,
      qualifies_mechanics_results: Boolean(result),
      private_data_required: true,
      source_status: "missing"
    }));
  }

  const publicFixtureRefs = [
    ...(model.materials ?? [])
      .filter((item) => item.provenance.toLowerCase().includes("no_material_standard"))
      .map((item) => item.id),
    ...model.pipe_segments
      .filter((item) => item.provenance.toLowerCase().includes("no_standard_catalog"))
      .map((item) => item.id)
  ];
  if (publicFixtureRefs.length > 0) {
    warnings.push(warning({
      warning_id: "public-fixture-provenance-only",
      warning_class: "PROVENANCE_WARNING",
      local_contract_class: "provenance_missing",
      severity: "warning",
      analysis_status: ["HUMAN_REVIEW_REQUIRED"],
      affected_refs: publicFixtureRefs,
      source_refs: publicFixtureRefs.map((ref) => `${ref}.provenance`),
      message: "Material and pipe-section values are invented public fixture values.",
      remediation: "Replace them with user-controlled data and documented provenance before engineering reliance.",
      blocks_mechanics_solve: false,
      blocks_rule_check: false,
      qualifies_mechanics_results: true,
      private_data_required: true,
      source_status: "provided_invented_only"
    }));
  }

  if (statusInputs.professional_acceptance.toLowerCase().includes("not_provided")) {
    warnings.push(warning({
      warning_id: "professional-acceptance-not-provided",
      warning_class: "ASSUMPTION_WARNING",
      local_contract_class: "professional_boundary",
      severity: "warning",
      analysis_status: ["HUMAN_REVIEW_REQUIRED"],
      affected_refs: [result?.run_id ?? model.project.id],
      source_refs: [`analysis_status.professional_acceptance:${statusInputs.professional_acceptance}`],
      message: "No hash-bound human acceptance record is attached to this preview state.",
      remediation: "Treat outputs as review context until human authority records acceptance outside the software.",
      blocks_mechanics_solve: false,
      blocks_rule_check: false,
      qualifies_mechanics_results: Boolean(result),
      private_data_required: false,
      source_status: "not_provided"
    }));
  }

  return warnings.sort((left, right) => left.warning_id.localeCompare(right.warning_id));
}

function warningClassInventory(warnings: MissingDataWarning[]) {
  const classes: GuiWarningClass[] = [
    "SOLVE_BLOCKING",
    "RULE_CHECK_BLOCKING",
    "PROVENANCE_WARNING",
    "ASSUMPTION_WARNING",
    "NONLINEAR_WARNING",
    "IP_BOUNDARY_WARNING"
  ];
  return classes.map((warningClass) => {
    const count = warnings.filter((warning) => warning.warning_class === warningClass).length;
    return {
      warning_class: warningClass,
      status: count > 0 ? "active" : "available_no_active_preview_item",
      active_count: count,
      analysis_status: guiWarningClassStatusMap()[warningClass],
      required_text_fields: ["class", "severity", "affected_refs", "message", "remediation"],
      color_only_signaling_allowed: false
    };
  });
}

function warning(input: Omit<MissingDataWarning, "protected_content_required" | "assistive_text_available">): MissingDataWarning {
  return {
    ...input,
    protected_content_required: false,
    assistive_text_available: true
  };
}

function isRuleIncomplete(status: string): boolean {
  const normalized = status.toLowerCase();
  return normalized.includes("incomplete") || normalized.includes("missing") || normalized.includes("not_performed");
}

function guiWarningClassStatusMap(): Record<GuiWarningClass, string[]> {
  return {
    SOLVE_BLOCKING: ["MODEL_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"],
    RULE_CHECK_BLOCKING: ["RULE_INPUTS_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"],
    PROVENANCE_WARNING: ["HUMAN_REVIEW_REQUIRED"],
    ASSUMPTION_WARNING: ["HUMAN_REVIEW_REQUIRED"],
    NONLINEAR_WARNING: ["MECHANICS_SOLVED", "HUMAN_REVIEW_REQUIRED"],
    IP_BOUNDARY_WARNING: ["HUMAN_REVIEW_REQUIRED"]
  };
}

function localWarningClassStatusMap() {
  return {
    solve_required: ["MODEL_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"],
    code_check_required: ["RULE_INPUTS_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"],
    provenance_missing: ["HUMAN_REVIEW_REQUIRED"],
    assumption: ["HUMAN_REVIEW_REQUIRED"],
    incomplete_data: ["MODEL_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"],
    diagnostic: ["MECHANICS_SOLVED", "USER_RULE_CHECKED", "USER_RULE_FAILED", "HUMAN_REVIEW_REQUIRED"],
    professional_boundary: ["HUMAN_REVIEW_REQUIRED"]
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

function previewProvenance() {
  return {
    source_name: "OpenPipeStress desktop technical preview",
    source_location: "apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx",
    source_license: "project-invented metadata only",
    contributor: "OpenPipeStress app integration tranche",
    contributor_certification: "Invented warning metadata only; no protected standards, private rule pack, private project payload, or professional acceptance record.",
    redistribution_status: "invented_non_engineering_example",
    review_status: "pending",
    privacy_classification: "public_metadata"
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeRefToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9:_-]+/g, "-").replace(/^-+|-+$/g, "") || "ref";
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9_-]+/g, "-").replace(/^-+|-+$/g, "") || "preview";
}

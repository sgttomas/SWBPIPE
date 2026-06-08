import { Download, ShieldAlert } from "lucide-react";
import type { Diagnostic, MechanicsResult, PreviewModel } from "../../types";

type WarningClass =
  | "RULE_CHECK_BLOCKING"
  | "PROVENANCE_WARNING"
  | "ASSUMPTION_WARNING"
  | "IP_BOUNDARY_WARNING";

type CompletenessFinding = {
  finding_id: string;
  warning_class: WarningClass;
  severity: Diagnostic["severity"];
  affected_refs: string[];
  source_refs: string[];
  message: string;
  remediation: string;
  private_data_required: boolean;
  protected_content_required: boolean;
  mechanics_solve_blocking: boolean;
  rule_check_blocking: boolean;
};

export function RuleCheckPanel({ model, result }: { model: PreviewModel; result: MechanicsResult | null }) {
  const review = buildRuleCompletenessReview({ model, result });

  return (
    <section className="panel rule-check-panel" aria-label="Rule-check completeness" data-testid="rule-check-panel">
      <div className="panel-title">
        <ShieldAlert size={16} aria-hidden="true" />
        Rule-Check Completeness
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="rule-check-export-link"
          download={`openpipestress-preview-rule-completeness-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(review)}
        >
          <Download size={14} aria-hidden="true" />
          Local completeness JSON
        </a>
        <span data-testid="rule-check-summary">
          {review.summary.finding_count} review findings; rule_check_blocked=
          {String(review.summary.rule_check_blocked)}; mechanics_reviewable=
          {String(review.summary.mechanics_results_reviewable)}
        </span>
      </div>
      <div className="report-list" data-testid="rule-check-body">
        <ReviewLine label="Rule status" value={review.rule_check_status} testId="rule-check-status" />
        <ReviewLine label="Mechanics status" value={review.mechanics_status} testId="rule-check-mechanics-status" />
        <ReviewLine label="Boundary" value={ruleBoundary(review)} testId="rule-check-boundary" />
        <div className="operation-record-list" data-testid="rule-check-findings">
          {review.findings.map((finding) => (
            <article
              className="operation-record"
              data-testid={`rule-check-finding-${finding.finding_id}`}
              key={finding.finding_id}
            >
              <strong>{finding.warning_class}</strong>
              <small>
                {finding.severity}; rule_blocking={String(finding.rule_check_blocking)}; private_data=
                {String(finding.private_data_required)}
              </small>
              <p>
                {finding.message} {finding.remediation}
              </p>
            </article>
          ))}
        </div>
      </div>
      <small className="report-note">
        Completeness findings explain missing user/private rule-check data. Mechanics preview output remains review-only
        and is not a code-compliance, approval, certification, sealing, or professional acceptance result.
      </small>
    </section>
  );
}

function ReviewLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildRuleCompletenessReview({ model, result }: { model: PreviewModel; result: MechanicsResult | null }) {
  const ruleStatus = result?.status.rule_check ?? model.analysis_status.rule_check;
  const mechanicsStatus = result?.status.mechanics ?? model.analysis_status.mechanics;
  const professionalStatus = result?.status.professional_acceptance ?? model.analysis_status.professional_acceptance;
  const diagnostics = [...model.diagnostics, ...(result?.diagnostics ?? [])];
  const findings = buildFindings({ model, result, diagnostics, ruleStatus });
  const ruleBlockingCount = findings.filter((finding) => finding.rule_check_blocking).length;
  const provenanceWarningCount = findings.filter((finding) => finding.warning_class === "PROVENANCE_WARNING").length;
  const ipBoundaryWarningCount = findings.filter((finding) => finding.warning_class === "IP_BOUNDARY_WARNING").length;

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.rule_completeness_review",
    export_scope: "local_browser_download_preview",
    deliverable_refs: ["DEL-06-03", "DEL-07-04", "DEL-05-04", "DEL-08-03"],
    scope_items: ["SOW-004", "SOW-022"],
    objectives: ["OBJ-002", "OBJ-005", "OBJ-006", "OBJ-011"],
    project_ref: model.project.id,
    run_ref: result?.run_id ?? "not generated",
    model_ref: result?.model_ref ?? model.project.id,
    mechanics_status: mechanicsStatus,
    rule_check_status: ruleStatus,
    professional_acceptance_status: professionalStatus,
    summary: {
      finding_count: findings.length,
      rule_check_blocked: ruleBlockingCount > 0,
      rule_check_blocking_count: ruleBlockingCount,
      provenance_warning_count: provenanceWarningCount,
      ip_boundary_warning_count: ipBoundaryWarningCount,
      mechanics_results_reviewable: Boolean(result && !mechanicsStatus.toLowerCase().includes("incomplete")),
      silent_defaults_used: false,
      bundled_code_values_used: false,
      compliance_claim_made: false
    },
    findings,
    diagnostic_refs: diagnostics.map((item) => item.id ?? item.code),
    data_boundary: model.data_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary()
  };
}

function buildFindings({
  model,
  result,
  diagnostics,
  ruleStatus
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  diagnostics: Diagnostic[];
  ruleStatus: string;
}): CompletenessFinding[] {
  const findings: CompletenessFinding[] = [];
  const normalizedRuleStatus = ruleStatus.toLowerCase();
  if (
    normalizedRuleStatus.includes("incomplete") ||
    normalizedRuleStatus.includes("missing") ||
    normalizedRuleStatus.includes("not_performed")
  ) {
    findings.push({
      finding_id: "required-rule-inputs-missing",
      warning_class: "RULE_CHECK_BLOCKING",
      severity: "warning",
      affected_refs: [model.project.id],
      source_refs: [
        `analysis_status.rule_check:${ruleStatus}`,
        ...diagnostics.filter((item) => item.code.includes("RULE_INPUT")).map((item) => item.id ?? item.code)
      ],
      message:
        "User-supplied rule-pack inputs are absent, so rule-check status stays incomplete while mechanics output remains separate.",
      remediation:
        "Provide a private rule pack and project-specific criteria with provenance; the public preview does not bundle code defaults.",
      private_data_required: true,
      protected_content_required: false,
      mechanics_solve_blocking: false,
      rule_check_blocking: true
    });
  }

  const componentRefs = model.components
    .filter((item) => item.provenance.toLowerCase().includes("no_flexibility_factor"))
    .map((item) => item.id);
  if (componentRefs.length > 0) {
    findings.push({
      finding_id: "component-flexibility-factor-provenance",
      warning_class: "PROVENANCE_WARNING",
      severity: "warning",
      affected_refs: componentRefs,
      source_refs: componentRefs.map((ref) => `${ref}.provenance`),
      message: "Component flexibility or stress-intensification provenance is intentionally absent from the public preview.",
      remediation: "Keep component correction data user-supplied or privately imported; do not infer protected table values.",
      private_data_required: true,
      protected_content_required: false,
      mechanics_solve_blocking: false,
      rule_check_blocking: true
    });
  }

  const noCodeCombinationRefs = [
    ...model.load_cases
      .filter((item) => item.provenance.toLowerCase().includes("no_code_combination"))
      .map((item) => item.id),
    ...(model.combinations ?? [])
      .filter((item) => item.provenance.toLowerCase().includes("no_code_default"))
      .map((item) => item.id)
  ];
  if (noCodeCombinationRefs.length > 0) {
    findings.push({
      finding_id: "code-combination-criteria-absent",
      warning_class: "RULE_CHECK_BLOCKING",
      severity: "warning",
      affected_refs: noCodeCombinationRefs,
      source_refs: noCodeCombinationRefs.map((ref) => `${ref}.provenance`),
      message: "Load cases and combinations are mechanics-basis preview inputs, not code-specific rule combinations.",
      remediation: "Supply private owner/code combination criteria before reporting user-rule checked status.",
      private_data_required: true,
      protected_content_required: false,
      mechanics_solve_blocking: false,
      rule_check_blocking: true
    });
  }

  const provenanceRefs = [
    ...(model.materials ?? [])
      .filter((item) => item.provenance.toLowerCase().includes("no_material_standard"))
      .map((item) => item.id),
    ...model.pipe_segments
      .filter((item) => item.provenance.toLowerCase().includes("no_standard_catalog"))
      .map((item) => item.id)
  ];
  if (provenanceRefs.length > 0) {
    findings.push({
      finding_id: "public-fixture-provenance-only",
      warning_class: "PROVENANCE_WARNING",
      severity: "warning",
      affected_refs: provenanceRefs,
      source_refs: provenanceRefs.map((ref) => `${ref}.provenance`),
      message: "Material and pipe-section values are invented public fixture values, not verified project or catalog data.",
      remediation: "Replace them with user-controlled data and provenance before engineering reliance.",
      private_data_required: true,
      protected_content_required: false,
      mechanics_solve_blocking: false,
      rule_check_blocking: true
    });
  }

  if (result?.status.professional_acceptance.toLowerCase().includes("not_provided")) {
    findings.push({
      finding_id: "professional-acceptance-not-provided",
      warning_class: "ASSUMPTION_WARNING",
      severity: "warning",
      affected_refs: [result.run_id],
      source_refs: [`${result.run_id}.status.professional_acceptance`],
      message: "No human professional acceptance record is attached to this mechanics preview run.",
      remediation: "Treat all computed values as review context until a human authority records acceptance outside the software.",
      private_data_required: false,
      protected_content_required: false,
      mechanics_solve_blocking: false,
      rule_check_blocking: false
    });
  }

  return findings;
}

function ruleBoundary(review: ReturnType<typeof buildRuleCompletenessReview>): string {
  return [
    "private rule data absent",
    `silent defaults used=${String(review.summary.silent_defaults_used)}`,
    `bundled code values=${String(review.summary.bundled_code_values_used)}`,
    "no compliance claim"
  ].join("; ");
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
  return value.replace(/[^a-z0-9-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}

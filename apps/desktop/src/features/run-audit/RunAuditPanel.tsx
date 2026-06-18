import { Fingerprint } from "lucide-react";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";

const RESULT_UNIT_DISPLAY_ORDER = ["MPa", "N", "N*m", "mm", "rad"];

export function RunAuditPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const run = analysisRun?.analysis_run;
  const resultHashCount = run?.result_refs.reduce((count, item) => count + item.hash_refs.length, 0) ?? 0;
  const hashScopes = run?.hashes.map((item) => item.payload_scope).join(", ") ?? "not generated";
  return (
    <section className="panel run-audit-panel" aria-label="Run audit" data-testid="run-audit-panel">
      <div className="panel-title">
        <Fingerprint size={16} />
        Run Audit
      </div>
      {result && run ? (
        <div className="report-list">
          <AuditLine
            label="Model state"
            value={`${run.model_state_ref.object_type}; ${run.model_state_ref.ref}`}
            testId="run-audit-model-state"
          />
          <AuditLine
            label="Analysis run"
            value={`${analysisRun.deliverable_id}; ${run.run_kind}; ${run.run_id}`}
            testId="run-audit-analysis-run"
          />
          <AuditLine
            label="Run status"
            value={run.analysis_status.join(", ")}
            testId="run-audit-status"
          />
          <AuditLine
            label="Hash evidence"
            value={`${result.results.length} result rows; ${resultHashCount} result value hashes; scopes: ${hashScopes}`}
            testId="run-audit-hashes"
          />
          <AuditLine
            label="Unit audit"
            value={unitAuditSummary(model, result)}
            testId="run-audit-units"
          />
          <AuditLine
            label="Input manifest"
            value={run.reproducibility.input_manifest_refs.map((item) => `${item.object_type}:${item.ref}`).join(", ")}
            testId="run-audit-input-manifest"
          />
          <AuditLine
            label="Reproducibility TBDs"
            value={run.reproducibility.unresolved_tbd.join(", ")}
            testId="run-audit-reproducibility"
          />
          <AuditLine
            label="Immutability"
            value={immutabilitySummary(run.immutability_policy)}
            testId="run-audit-immutability"
          />
          <AuditLine
            label="Boundary"
            value={boundarySummary(run.professional_boundary)}
            testId="run-audit-boundary"
          />
        </div>
      ) : (
        <p className="muted" data-testid="run-audit-empty">
          Run mechanics preview to generate immutable model-state and analysis-run audit references for this local fixture.
        </p>
      )}
      <small className="report-note">
        Audit evidence for {model.project.id} is local technical-preview context; it is not a release or professional acceptance record.
      </small>
    </section>
  );
}

function AuditLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function immutabilitySummary(policy: AnalysisRunEnvelope["analysis_run"]["immutability_policy"]): string {
  const readOnly = policy.run_record_is_read_only ? "read-only run record" : "mutable run record requires review";
  const newRun = policy.new_run_required_for_change ? "changes require new run" : "change policy TBD";
  const hashBoundary = policy.hash_invalidates_external_acceptance
    ? "hash changes invalidate external acceptance"
    : "external acceptance hash policy TBD";
  return `${readOnly}; ${policy.mutation_policy}; ${newRun}; ${hashBoundary}`;
}

function boundarySummary(boundary: Record<string, boolean>): string {
  if (
    boundary.human_review_required &&
    !boundary.software_makes_compliance_claim &&
    !boundary.software_makes_certification_claim &&
    !boundary.software_makes_sealing_claim &&
    !boundary.software_makes_approval_claim &&
    !boundary.software_makes_authentication_claim
  ) {
    return "human review required; no compliance, certification, sealing, authentication, or approval claim";
  }
  return "professional boundary requires attention";
}

function unitAuditSummary(model: PreviewModel, result: MechanicsResult): string {
  return [
    `model=${formatUnitRecord(model.project.units)}`,
    `results=${uniqueResultUnits(result).join(",") || "none"}`,
    `rows=${result.results.length}`,
    "source=result_envelope",
    "conversion=false"
  ].join("; ");
}

function uniqueResultUnits(result: MechanicsResult): string[] {
  return Array.from(new Set(result.results.map((item) => item.unit).filter(Boolean))).sort(compareUnitSymbols);
}

function formatUnitRecord(units: Record<string, string>): string {
  const entries = Object.entries(units).sort(([left], [right]) => left.localeCompare(right));
  return entries.map(([dimension, unit]) => `${dimension}=${unit}`).join(",") || "none";
}

function compareUnitSymbols(left: string, right: string): number {
  const leftIndex = RESULT_UNIT_DISPLAY_ORDER.indexOf(left);
  const rightIndex = RESULT_UNIT_DISPLAY_ORDER.indexOf(right);
  if (leftIndex >= 0 && rightIndex >= 0) return leftIndex - rightIndex;
  if (leftIndex >= 0) return -1;
  if (rightIndex >= 0) return 1;
  return left.localeCompare(right, undefined, { numeric: true, sensitivity: "base" });
}

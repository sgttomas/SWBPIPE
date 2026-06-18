import { AlertTriangle, Download, Play, ShieldCheck, Square } from "lucide-react";
import type { AnalysisRunEnvelope, Diagnostic, MechanicsResult, PreviewModel, SolveJobAuditState } from "../../types";

export function SolvePanel({
  analysisRun,
  model,
  result,
  running,
  solveJob,
  onCancel,
  onRun
}: {
  analysisRun: AnalysisRunEnvelope | null;
  model: PreviewModel;
  result: MechanicsResult | null;
  running: boolean;
  solveJob: SolveJobAuditState;
  onCancel: () => void;
  onRun: () => void;
}) {
  const diagnostics = [...model.diagnostics, ...(result?.diagnostics ?? [])];
  const readinessItems = readinessSummary({ model, result, diagnostics });
  const packet = buildSolveJobPacket({ model, result, analysisRun, solveJob, running });
  return (
    <section className="panel solve-panel" aria-label="Solve execution" data-testid="solve-panel">
      <div className="panel-title">Execution</div>
      <div className="status-grid">
        <Status label="Mechanics" value={result?.status.mechanics ?? model.analysis_status.mechanics} />
        <Status label="Rule check" value={result?.status.rule_check ?? model.analysis_status.rule_check} />
        <Status label="Professional acceptance" value={result?.status.professional_acceptance ?? model.analysis_status.professional_acceptance} />
      </div>
      <section className="readiness-summary" aria-label="Solve readiness summary" data-testid="solve-readiness-summary">
        {readinessItems.map((item) => (
          <ReadinessRow key={item.id} item={item} />
        ))}
      </section>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="solve-job-export-link"
          download={`openpipestress-preview-solve-job-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Solve job JSON
        </a>
        <span data-testid="solve-job-summary">
          state={packet.summary.job_state}; events={packet.summary.event_count}; result_rows=
          {packet.summary.result_row_count}; cancellation_requested={String(packet.summary.cancellation_requested)}
        </span>
      </div>
      <div className="report-list solve-job-list" data-testid="solve-job-audit">
        <SolveLine label="Progress" value={progressSummary(packet)} testId="solve-job-progress" />
        <SolveLine label="Cancellation" value={cancellationSummary(packet)} testId="solve-job-cancellation" />
        <SolveLine label="Result binding" value={resultBinding(packet)} testId="solve-job-binding" />
        <SolveLine label="Unit policy" value={unitPolicySummary(packet)} testId="solve-job-unit-policy" />
        <SolveLine label="Boundary" value={boundarySummary(packet)} testId="solve-job-boundary" />
      </div>
      <button className="primary-action" data-testid="run-mechanics-preview" onClick={onRun} disabled={running} type="button">
        <Play size={16} />
        {running ? "Running preview" : "Run mechanics preview"}
      </button>
      <button
        className="secondary-action"
        data-testid="cancel-mechanics-preview"
        disabled={!running}
        title={running ? undefined : "Disabled: no mechanics preview is running, so there is nothing to cancel."}
        onClick={onCancel}
        type="button"
      >
        <Square size={15} aria-hidden="true" />
        Cancel preview
      </button>
    </section>
  );
}

function Status({ label, value }: { label: string; value: string }) {
  return (
    <div className="status-pill">
      <span>{label}</span>
      <strong data-testid={`status-${label.toLowerCase().replaceAll(" ", "-")}`}>{value.replaceAll("_", " ")}</strong>
    </div>
  );
}

type ReadinessItem = {
  id: "mechanics" | "rule" | "diagnostics" | "professional";
  label: string;
  value: string;
  tone: "ok" | "info" | "warning" | "blocking";
};

function readinessSummary({
  model,
  result,
  diagnostics
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  diagnostics: Diagnostic[];
}): ReadinessItem[] {
  const mechanicsStatus = result?.status.mechanics ?? model.analysis_status.mechanics;
  const ruleStatus = result?.status.rule_check ?? model.analysis_status.rule_check;
  const professionalStatus = result?.status.professional_acceptance ?? model.analysis_status.professional_acceptance;
  const warningCount = diagnostics.filter((item) => item.severity === "warning").length;
  const blockingCount = diagnostics.filter((item) => item.severity === "blocking" || item.severity === "error").length;

  return [
    {
      id: "mechanics",
      label: "Mechanics readiness",
      value: result
        ? `${result.results.length} computed result rows; ${formatStatus(mechanicsStatus)}`
        : `preview run not started; ${formatStatus(mechanicsStatus)}`,
      tone: result ? "ok" : "info"
    },
    {
      id: "rule",
      label: "Rule-check readiness",
      value: ruleReadiness(ruleStatus),
      tone: ruleStatus.toLowerCase().includes("incomplete") || ruleStatus.toLowerCase().includes("missing")
        ? "warning"
        : "info"
    },
    {
      id: "diagnostics",
      label: "Diagnostics",
      value: `${diagnostics.length} ${plural("diagnostic", diagnostics.length)}; ${warningCount} ${plural(
        "warning",
        warningCount
      )}; ${blockingCount} blocking/error`,
      tone: blockingCount > 0 ? "blocking" : warningCount > 0 ? "warning" : "ok"
    },
    {
      id: "professional",
      label: "Professional boundary",
      value: professionalReadiness(professionalStatus),
      tone: "warning"
    }
  ];
}

function ReadinessRow({ item }: { item: ReadinessItem }) {
  const Icon = item.tone === "ok" ? ShieldCheck : AlertTriangle;
  return (
    <div className={`readiness-row ${item.tone}`} data-testid={`readiness-${item.id}`}>
      <Icon size={14} aria-hidden="true" />
      <span>{item.label}</span>
      <strong>{item.value}</strong>
    </div>
  );
}

function ruleReadiness(status: string): string {
  const normalized = status.toLowerCase();
  if (normalized.includes("incomplete") || normalized.includes("missing") || normalized.includes("not_performed")) {
    return `rule inputs incomplete; mechanics results remain reviewable only; ${formatStatus(status)}`;
  }
  return formatStatus(status);
}

function professionalReadiness(status: string): string {
  const normalized = status.toLowerCase();
  if (normalized.includes("not_provided") || normalized.includes("not provided")) {
    return "human review required; no professional acceptance record";
  }
  return formatStatus(status);
}

function formatStatus(value: string): string {
  return value.replaceAll("_", " ").toLowerCase();
}

function plural(label: string, count: number): string {
  return count === 1 ? label : `${label}s`;
}

function SolveLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildSolveJobPacket({
  model,
  result,
  analysisRun,
  solveJob,
  running
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  solveJob: SolveJobAuditState;
  running: boolean;
}) {
  const run = analysisRun?.analysis_run;
  const resultRowCount = result?.results.length ?? 0;
  const resultHashCount = run?.result_refs.reduce((count, item) => count + item.hash_refs.length, 0) ?? 0;
  const unitPolicyEvidence = buildSolveJobUnitPolicyEvidence({ model, result, analysisRun });
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.solve_job_audit",
    export_scope: "local_browser_download_preview",
    deliverable_refs: ["DEL-07-07", "DEL-14-02", "DEL-04-06", "DEL-02-02"],
    scope_items: ["SOW-055", "SOW-072", "SOW-053"],
    objectives: ["OBJ-006", "OBJ-007", "OBJ-016"],
    project_ref: model.project.id,
    job_id: solveJob.job_id,
    summary: {
      job_state: solveJob.state,
      event_count: solveJob.events.length,
      result_row_count: resultRowCount,
      diagnostic_count: diagnosticsFor(model, result).length,
      cancellation_requested: solveJob.cancellation_requested,
      cancellation_status: solveJob.cancellation_status,
      running
    },
    progress_contract: {
      progress_basis: solveJob.progress_basis,
      percentages_synthesized: solveJob.percentages_synthesized,
      backend_percent_stream_available: solveJob.backend_percent_stream_available,
      latest_event_state: solveJob.events[solveJob.events.length - 1]?.state ?? "not_started"
    },
    cancellation: {
      request_control_visible: true,
      request_enabled: running,
      requested: solveJob.cancellation_requested,
      status: solveJob.cancellation_status,
      backend_job_seam: solveJob.backend_job_seam,
      backend_job_id: solveJob.backend_job_id,
      backend_cancellation_token: solveJob.backend_cancellation_token,
      cancellation_scope:
        solveJob.backend_job_seam === "tauri_backend_job"
          ? "cooperative_checkpoints_not_preemptive"
          : "ui_request_record_only_no_backend_job",
      mutates_solver_process_directly: false,
      cancellation_success_claimed: false
    },
    model_state_ref: run?.model_state_ref ?? null,
    analysis_run_ref: run ? { object_type: "AnalysisRun", ref: run.run_id } : null,
    run_kind: run?.run_kind ?? "not_started",
    analysis_status: run?.analysis_status ?? [],
    result_hash_count: resultHashCount,
    hash_scopes: run?.hashes.map((item) => item.payload_scope) ?? [],
    input_manifest_refs: run?.reproducibility.input_manifest_refs ?? [],
    unit_policy_evidence: unitPolicyEvidence,
    events: solveJob.events,
    diagnostics: diagnosticsFor(model, result),
    error_message: solveJob.error_message,
    data_boundary: model.data_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary()
  };
}

function buildSolveJobUnitPolicyEvidence({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  return {
    evidence_id: "unit-policy-evidence:solve-job-audit",
    unit_system_ref: reference("UnitSystem", "unit-system:dec-018-si-dual-display"),
    storage_convention: "entered_units_preserved",
    solve_job_unit_policy: "solve_job_audit_records_model_and_result_units_without_conversion",
    model_units: sortedStringRecord(model.project.units),
    result_units: uniqueSorted((result?.results ?? []).map((item) => item.unit)),
    result_row_count: result?.results.length ?? 0,
    analysis_run_ref: analysisRun?.analysis_run
      ? reference("AnalysisRun", analysisRun.analysis_run.run_id)
      : reference("AnalysisRun", "not generated"),
    conversion_policy: "solve_job_audit_preserves_source_units_no_conversion",
    conversion_performed: false,
    source: "apps/desktop/src/features/solve/SolvePanel.tsx",
    decision_basis_refs: [
      reference("Decision", "DEC-018"),
      reference("Deliverable", "DEL-02-02"),
      reference("Deliverable", "DEL-07-07"),
      reference("Deliverable", "DEL-14-02")
    ]
  };
}

function diagnosticsFor(model: PreviewModel, result: MechanicsResult | null): Diagnostic[] {
  return [...model.diagnostics, ...(result?.diagnostics ?? [])];
}

function progressSummary(packet: ReturnType<typeof buildSolveJobPacket>): string {
  return `${packet.progress_contract.latest_event_state}; ${packet.progress_contract.progress_basis}; percentages_synthesized=${String(
    packet.progress_contract.percentages_synthesized
  )}`;
}

function cancellationSummary(packet: ReturnType<typeof buildSolveJobPacket>): string {
  return [
    `control_visible=${String(packet.cancellation.request_control_visible)}`,
    `enabled=${String(packet.cancellation.request_enabled)}`,
    `requested=${String(packet.cancellation.requested)}`,
    `seam=${packet.cancellation.backend_job_seam}`,
    `token=${packet.cancellation.backend_cancellation_token}`,
    `success_claimed=${String(packet.cancellation.cancellation_success_claimed)}`
  ].join("; ");
}

function resultBinding(packet: ReturnType<typeof buildSolveJobPacket>): string {
  if (!packet.analysis_run_ref || !packet.model_state_ref) return "not generated; result rows=0; hashes=0";
  return `${packet.model_state_ref.ref}; ${packet.analysis_run_ref.ref}; result rows=${packet.summary.result_row_count}; hashes=${packet.result_hash_count}`;
}

function unitPolicySummary(packet: ReturnType<typeof buildSolveJobPacket>): string {
  const evidence = packet.unit_policy_evidence;
  const resultUnits = evidence.result_units.length > 0 ? evidence.result_units.join(",") : "none";
  return [
    `model=${formatUnitRecord(evidence.model_units)}`,
    `results=${resultUnits}`,
    `rows=${evidence.result_row_count}`,
    `conversion=${String(evidence.conversion_performed)}`
  ].join("; ");
}

function boundarySummary(packet: ReturnType<typeof buildSolveJobPacket>): string {
  return [
    `private payload=${String(packet.private_payload_included)}`,
    `protected content=${String(packet.protected_content_included)}`,
    `release/professional claim=${String(packet.release_or_professional_claim)}`,
    "human review required"
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

function reference(objectType: string, ref: string) {
  return { object_type: objectType, ref };
}

function sortedStringRecord(values: Record<string, string>): Record<string, string> {
  return Object.fromEntries(Object.entries(values).sort(([left], [right]) => left.localeCompare(right)));
}

function uniqueSorted(values: string[]): string[] {
  return Array.from(new Set(values.filter(Boolean))).sort();
}

function formatUnitRecord(units: Record<string, string>): string {
  return Object.entries(units)
    .map(([key, value]) => `${key}=${value}`)
    .join(",");
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}

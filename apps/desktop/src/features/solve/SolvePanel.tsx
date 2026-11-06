import { resultRowCount, resultUnits, runResultHashCount, type SessionAnalysisRun, type SessionResult } from "../../services/nativeResult";
import { AlertTriangle, Download, Play, ShieldCheck, Square } from "lucide-react";
import { LoadReferenceOutputGate } from "../results/LoadReferenceOutputGate";
import { LoadReferenceStatesBlock } from "./LoadReferenceStatesBlock";
import type { AnalysisRunEnvelope, Diagnostic, MechanicsResult, PreviewModel, SolveJobAuditState } from "../../types";
import type { PreviewSolverMode } from "../../services/previewService";
import { hasRecordedUnsolvedModelStatus, professionalStatusToken, ruleCheckStatusToken, solverDisplayWithToken, statusDisplayWithToken } from "../workspace/statusLabels";
import { describeResourceRefusal, describeSolveWorkerFailure, formatBytes, resourceRefusalCode } from "../../services/memoryPool";
import { describeSolveRefusalRetry, isResultTooLargeSolveJob, solveRefusalPoint } from "../workspace/solveJobAudit";
import { MemoryPoolPanel } from "./MemoryPoolPanel";
import type { NumericalHealthView } from "../results/numericalHealth";
import type { CurrentSessionInputManifest } from "../../services/inputManifestService";
import { RulePackRunReferences } from "../rule-packs/RulePackRunReferences";

export function SolvePanel({
  analysisRun,
  inputManifest = null,
  model,
  result,
  running,
  solveJob,
  solverMode,
  numericalHealth = null,
  onCancel,
  onRun,
  onSolverModeChange
}: {
  analysisRun: SessionAnalysisRun | null;
  /** The current solve's input manifest: its recorded rule-pack references and their diagnostics. */
  inputManifest?: CurrentSessionInputManifest | null;
  model: PreviewModel;
  result: SessionResult | null;
  /** DEL-07-05 numerical health: the diagnostics tile's header badge, for the held result. */
  numericalHealth?: NumericalHealthView | null;
  running: boolean;
  solveJob: SolveJobAuditState;
  solverMode: PreviewSolverMode;
  onCancel: () => void;
  onRun: () => void;
  onSolverModeChange: (mode: PreviewSolverMode) => void;
}) {
  // An absent model `diagnostics` array means no model diagnostics; the key is never added.
  const diagnostics = [...(model.diagnostics ?? []), ...(result?.diagnostics ?? [])];
  const readinessItems = readinessSummary({ model, result, diagnostics, solveJob });
  const packet = buildSolveJobPacket({ model, result, analysisRun, solveJob, running, solverMode });
  const refusalPoint = solveRefusalPoint(solveJob);
  const resultTooLarge = isResultTooLargeSolveJob(solveJob);
  return (
    <section className="panel solve-panel" aria-label="Solve execution" data-testid="solve-panel">
      <div className="panel-title">Execution</div>
      {/* The Mechanics / Rule check / Professional pills now live in the global
          status bar; the local status grid was removed to avoid duplication. */}
      <section className="readiness-summary" aria-label="Solve readiness summary" data-testid="solve-readiness-summary">
        {readinessItems.map((item) => (
          <ReadinessRow key={item.id} item={item} healthBadge={item.id === "diagnostics" && result ? numericalHealth : null} />
        ))}
      </section>
      <div className="report-actions">
        <LoadReferenceOutputGate result={result} testIdPrefix="solve-job">
        <ControlledExportLink
          className="report-export-link"
          data-testid="solve-job-export-link"
          download={`openpipestress-preview-solve-job-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Solve job JSON
        </ControlledExportLink>
        </LoadReferenceOutputGate>
        <span data-testid="solve-job-summary">
          state={packet.summary.job_state}; events={packet.summary.event_count}; result_rows=
          {packet.summary.result_row_count}; cancellation_requested={String(packet.summary.cancellation_requested)}
        </span>
      </div>
      {solveJob.error_message && !resultTooLarge ? <p role="status" data-testid="solve-job-error">{solveJob.error_message}</p> : null}
      {resultTooLarge ? (
        <p role="status" className="memory-pool-message warning" data-testid="solve-job-result-too-large">{solveJob.error_message}</p>
      ) : null}
      <RulePackRunReferences manifest={inputManifest} record={analysisRun} testId="solve-rule-pack-refs" />
      {solveJob.state === "failed" && describeSolveWorkerFailure(solveJob.error_message) ? (
        <p role="status" className="memory-pool-message error" data-testid="solve-job-worker-failure">
          {describeSolveWorkerFailure(solveJob.error_message)}
        </p>
      ) : null}
      {solveJob.resource.refusal ? (
        <p role="status" className="memory-pool-message warning" data-testid="solve-job-resource-refusal">
          Refused by the native memory pool: {describeResourceRefusal(solveJob.resource.refusal)} No result was published.
        </p>
      ) : null}
      {refusalPoint && solveJob.resource.refusal ? (
        <p className="memory-pool-message warning" data-testid="solve-job-refusal-retry">
          {describeSolveRefusalRetry(solveJob.resource.refusal, refusalPoint)} Refusal code: <code>{resourceRefusalCode(solveJob.resource.refusal)}</code>
        </p>
      ) : null}
      {solveJob.resource.queue_reason && (solveJob.state === "queued" || solveJob.state === "cancelling") ? (
        <p role="status" className="memory-pool-message info" data-testid="solve-job-queue-reason">
          Waiting in the queue: {describeResourceRefusal(solveJob.resource.queue_reason)} The queue retries by itself and starts
          the solve once the pool admits it; Cancel preview stops the wait.
        </p>
      ) : null}
      <div className="report-list solve-job-list" data-testid="solve-job-audit">
        <SolveLine label="Solver mode" value={solverModeSummary(packet)} testId="solve-job-solver-mode" />
        <SolveLine label="Progress" value={progressSummary(packet)} testId="solve-job-progress" />
        <SolveLine label="Cancellation" value={cancellationSummary(packet)} testId="solve-job-cancellation" />
        <SolveLine label="Memory" value={resourceSummary(packet)} testId="solve-job-resource" />
        <SolveLine label="Result binding" value={resultBinding(packet)} testId="solve-job-binding" />
        <SolveLine label="Unit policy" value={unitPolicySummary(packet)} testId="solve-job-unit-policy" />
        <SolveLine label="Boundary" value={boundarySummary(packet)} testId="solve-job-boundary" />
      </div>
      <LoadReferenceStatesBlock result={result} model={model} />
      <div className="solver-mode-control" role="group" aria-label="Solver mode" data-testid="solver-mode-control">
        <button
          aria-pressed={solverMode === "sparse_interactive"}
          className={solverMode === "sparse_interactive" ? "mode-button active" : "mode-button"}
          data-testid="solver-mode-sparse"
          disabled={running}
          onClick={() => onSolverModeChange("sparse_interactive")}
          type="button"
        >
          <Play size={14} aria-hidden="true" />
          Sparse interactive
        </button>
        <button
          aria-pressed={solverMode === "dense_scrutiny"}
          className={solverMode === "dense_scrutiny" ? "mode-button active" : "mode-button"}
          data-testid="solver-mode-dense"
          disabled={running}
          onClick={() => onSolverModeChange("dense_scrutiny")}
          type="button"
        >
          <ShieldCheck size={14} aria-hidden="true" />
          Dense scrutiny
        </button>
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
      <MemoryPoolPanel active={running} />
    </section>
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
  diagnostics,
  solveJob
}: {
  model: PreviewModel;
  result: SessionResult | null;
  diagnostics: Diagnostic[];
  solveJob: SolveJobAuditState;
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
        ? `${resultRowCount(result)} computed result rows; ${solverDisplayWithToken(mechanicsStatus)}`
        : solveJob.state === "failed" ? `Solve job state: ${solveJob.state}; ${solverDisplayWithToken(solveJob.state)}`
        : isResultTooLargeSolveJob(solveJob) ? "Solve job state: refused; the result is too large for the desktop to display, so no result was kept"
        : solveJob.state === "refused" ? "Solve job state: refused by the native memory pool; no result was published"
        : `preview run not started; ${hasRecordedUnsolvedModelStatus(mechanicsStatus) ? solverDisplayWithToken(mechanicsStatus) : statusDisplayWithToken(mechanicsStatus)}`,
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

function ReadinessRow({ item, healthBadge = null }: { item: ReadinessItem; healthBadge?: NumericalHealthView | null }) {
  const Icon = item.tone === "ok" ? ShieldCheck : AlertTriangle;
  return (
    <div className={`readiness-row ${item.tone}`} data-testid={`readiness-${item.id}`}>
      <Icon size={14} aria-hidden="true" />
      <span>{item.label}</span>
      <strong>
        {item.value}
        {healthBadge ? <> <span className={`numerical-health-badge numerical-health-${healthBadge.badge}`} data-badge={healthBadge.badge} data-testid="solve-numerical-health-badge">{healthBadge.headline}</span></> : null}
      </strong>
    </div>
  );
}

function ruleReadiness(status: string): string {
  const normalized = status.toLowerCase();
  const display = statusDisplayWithToken(ruleCheckStatusToken(status));
  if (normalized.includes("incomplete") || normalized.includes("missing") || normalized.includes("not_performed")) {
    return `${display}; mechanics results remain reviewable only`;
  }
  return display;
}

function professionalReadiness(status: string): string {
  return statusDisplayWithToken(professionalStatusToken(status));
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
  running,
  solverMode
}: {
  model: PreviewModel;
  result: SessionResult | null;
  analysisRun: SessionAnalysisRun | null;
  solveJob: SolveJobAuditState;
  running: boolean;
  solverMode: PreviewSolverMode;
}) {
  const run = analysisRun?.analysis_run;
  const rowCount = result ? resultRowCount(result) : 0;
  const resultHashCount = runResultHashCount(analysisRun);
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
      solver_mode: solverMode,
      event_count: solveJob.events.length,
      result_row_count: rowCount,
      diagnostic_count: diagnosticsFor(model, result).length,
      cancellation_requested: solveJob.cancellation_requested,
      cancellation_status: solveJob.cancellation_status,
      running
    },
    progress_contract: {
      solver_mode: solverMode,
      sparse_interactive_default: solverMode === "sparse_interactive",
      dense_scrutiny_explicit: solverMode === "dense_scrutiny",
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
      // The scope the native host reported for this job (a running job's worker
      // process is terminated on cancellation); browser fixture runs have no backend job.
      cancellation_scope:
        solveJob.backend_job_seam === "tauri_backend_job"
          ? solveJob.backend_cancellation_scope ?? "not_reported"
          : "ui_request_record_only_no_backend_job",
      mutates_solver_process_directly: false,
      cancellation_success_claimed: false
    },
    // DEL-04-07: the native memory grant and verdicts for this job. Budget policy
    // is recorded here, beside the run, never in the model document.
    resource: {
      source: solveJob.resource.source,
      invocation_budget_bytes: solveJob.resource.invocation_budget_bytes,
      pool_bytes: solveJob.resource.pool_bytes,
      route_pricing: solveJob.resource.route_pricing,
      ingress_reserved_bytes: solveJob.resource.ingress_reserved_bytes,
      // The ordinary solve runs in a worker process whose allocator is capped at
      // its grant; the grant grows on demand up to the per-invocation budget.
      ordinary_solve_capped_by: solveJob.resource.route_pricing === "ordinary_worker_capped" ? "worker_grant" : null,
      worker_grant_bytes: solveJob.resource.worker_grant_bytes,
      worker_peak_grant_bytes: solveJob.resource.worker_peak_grant_bytes,
      worker_peak_footprint_bytes: solveJob.resource.worker_peak_footprint_bytes,
      queue_reason: solveJob.resource.queue_reason,
      refusal: solveJob.resource.refusal,
      refusal_reason: solveJob.resource.refusal ? describeResourceRefusal(solveJob.resource.refusal) : null
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
  result: SessionResult | null;
  analysisRun: SessionAnalysisRun | null;
}) {
  return {
    evidence_id: "unit-policy-evidence:solve-job-audit",
    unit_system_ref: reference("UnitSystem", "unit-system:dec-018-si-dual-display"),
    storage_convention: "entered_units_preserved",
    solve_job_unit_policy: "solve_job_audit_records_model_and_result_units_without_conversion",
    model_units: sortedStringRecord(model.project.units),
    result_units: uniqueSorted((result ? resultUnits(result) : [])),
    result_row_count: (result ? resultRowCount(result) : 0),
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

function diagnosticsFor(model: PreviewModel, result: SessionResult | null): Diagnostic[] {
  return [...(model.diagnostics ?? []), ...(result?.diagnostics ?? [])];
}

function progressSummary(packet: ReturnType<typeof buildSolveJobPacket>): string {
  return `${packet.progress_contract.latest_event_state}; ${packet.progress_contract.progress_basis}; percentages_synthesized=${String(
    packet.progress_contract.percentages_synthesized
  )}`;
}

function solverModeSummary(packet: ReturnType<typeof buildSolveJobPacket>): string {
  return [
    `selected=${packet.summary.solver_mode}`,
    `sparse_default=${String(packet.progress_contract.sparse_interactive_default)}`,
    `dense_scrutiny=${String(packet.progress_contract.dense_scrutiny_explicit)}`
  ].join("; ");
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

function resourceSummary(packet: ReturnType<typeof buildSolveJobPacket>): string {
  const resource = packet.resource;
  if (resource.source === "not_reported" && !resource.refusal) return "not reported; no native job grant recorded";
  // The per-invocation budget bounds the request intake and caps the ordinary
  // solve's worker (its grant grows on demand up to the budget); held results
  // are bounded by the pool, headroom and count limits, not by the budget.
  const worker = resource.route_pricing === "ordinary_worker_capped"
    ? [`worker grant=${resource.worker_grant_bytes === null ? "none held" : formatBytes(resource.worker_grant_bytes)}`,
      `peak=${resource.worker_peak_grant_bytes === null ? "none" : formatBytes(resource.worker_peak_grant_bytes)}`,
      `peak footprint=${resource.worker_peak_footprint_bytes === null ? "not measured" : formatBytes(resource.worker_peak_footprint_bytes)}`]
    : [];
  return [
    `budget=${formatBytes(resource.invocation_budget_bytes)} (intake and solve worker cap)`,
    `pool=${formatBytes(resource.pool_bytes)}`,
    `route=${resource.route_pricing ?? "not granted"}`,
    ...worker,
    `refusal=${resource.refusal?.kind ?? "none"}`
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
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";

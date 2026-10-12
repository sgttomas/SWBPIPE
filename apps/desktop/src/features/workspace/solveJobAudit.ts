// The solve-job audit record (`SolveJobAuditState`): its starting value, the
// transitions published as a solve starts, is cancelled, completes or fails, the
// records that replace it when the model changes, and the poll that waits for a
// backend job. No React.

import { awaitPreviewMechanicsJob } from "../../services/previewService";
import type {
  BackendSolveJobCancellationReceipt,
  BackendSolveJobStatus,
  SolveJobStartReceipt
} from "../../services/previewService";
import { describeResourceRefusal, formatBytes, ResourceUnavailableError } from "../../services/memoryPool";
import type {
  AnalysisRunEnvelope,
  MechanicsResult,
  OperationOutcome,
  PoolStatus,
  PreviewModel,
  ResourceRefusal,
  SolveJobAuditState,
  SolveJobResourceRecord,
  SolveResourceGrant
} from "../../types";

const NO_BACKEND_JOB_TOKEN = "none_no_active_backend_job";

function notReportedResource(): SolveJobResourceRecord {
  return {
    source: "not_reported",
    invocation_budget_bytes: null,
    pool_bytes: null,
    route_pricing: null,
    ingress_reserved_bytes: null,
    queue_reason: null,
    refusal: null
  };
}

function grantedResource(grant: SolveResourceGrant | null): SolveJobResourceRecord {
  if (!grant) return notReportedResource();
  return { ...notReportedResource(), source: "native_job_receipt", ...grant };
}

/** The resource record of a refusal: the budget and pool come from the pool
 * status reported with it, when there is one; there is no route grant. */
function refusedResource(current: SolveJobResourceRecord, refusal: ResourceRefusal, pool: PoolStatus | null): SolveJobResourceRecord {
  if (current.source === "native_job_receipt") return { ...current, queue_reason: null, refusal };
  return {
    ...notReportedResource(),
    source: pool ? "native_refusal_pool_status" : "not_reported",
    invocation_budget_bytes: pool?.invocation_budget_bytes ?? null,
    pool_bytes: pool?.pool_bytes ?? null,
    refusal
  };
}

function grantSummary(grant: SolveResourceGrant | null): string {
  return grant
    ? `per-invocation budget ${formatBytes(grant.invocation_budget_bytes)} (bounds the model intake only); pool ${formatBytes(grant.pool_bytes)}; model intake reserved ${formatBytes(grant.ingress_reserved_bytes)}; route ordinary_unpriced (no byte bound is claimed for the solve itself)`
    : "the native host reported no resource grant";
}

export function initialSolveJob(): SolveJobAuditState {
  return {
    job_id: "job:preview-linear-static:not-started",
    state: "not_started",
    progress_basis: "preview_service_event_state_only_no_percent_stream",
    percentages_synthesized: false,
    backend_percent_stream_available: false,
    cancellation_requested: false,
    cancellation_status: "not_requested",
    backend_job_seam: "no_job_started",
    backend_job_id: null,
    backend_cancellation_token: NO_BACKEND_JOB_TOKEN,
    events: [
      {
        event_id: "solve-preview-not-started",
        state: "not_started",
        message: "Preview mechanics has not been requested in this session.",
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ],
    error_message: null,
    resource: notReportedResource()
  };
}

export function startSolveJob(model: PreviewModel | null, startReceipt: SolveJobStartReceipt): SolveJobAuditState {
  const modelRef = model?.project.id ?? "project:unknown";
  if (startReceipt.mode === "backend_job") {
    // A backend job starts queued; the native state is followed by `progressSolveJob`.
    return {
      job_id: startReceipt.job_id,
      state: "queued",
      progress_basis: "preview_service_event_state_only_no_percent_stream",
      percentages_synthesized: false,
      backend_percent_stream_available: false,
      cancellation_requested: false,
      cancellation_status: "not_requested",
      backend_job_seam: "tauri_backend_job",
      backend_job_id: startReceipt.job_id,
      backend_cancellation_token: startReceipt.backend_cancellation_token,
      events: [
        {
          event_id: "solve-preview-queued",
          state: "queued",
          message: `Backend solve job ${startReceipt.job_id} queued with a backend cancellation token through the application service boundary; ${grantSummary(startReceipt.resource)}; cancellation is cooperative at backend checkpoints (${startReceipt.cancellation_scope}); this service path does not stream percentage progress.`,
          result_available: false,
          diagnostic_count: model?.diagnostics.length ?? 0,
          result_row_count: 0,
          analysis_status: model ? [model.analysis_status.mechanics, model.analysis_status.rule_check] : []
        }
      ],
      error_message: null,
      resource: grantedResource(startReceipt.resource)
    };
  }
  return {
    job_id: `job:preview-linear-static:${safeJobToken(modelRef)}`,
    state: "running",
    progress_basis: "preview_service_event_state_only_no_percent_stream",
    percentages_synthesized: false,
    backend_percent_stream_available: false,
    cancellation_requested: false,
    cancellation_status: "not_requested",
    backend_job_seam: "browser_fixture_no_backend_job",
    backend_job_id: null,
    backend_cancellation_token: "unavailable_no_backend_job_browser_fixture_mode",
    events: [
      {
        event_id: "solve-preview-queued",
        state: "queued",
        message: "Preview mechanics command queued through the application service boundary.",
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      },
      {
        event_id: "solve-preview-running",
        state: "running",
        message: "Preview mechanics command is executing in browser fixture mode without a backend job; this service path does not stream percentage progress.",
        result_available: false,
        diagnostic_count: model?.diagnostics.length ?? 0,
        result_row_count: 0,
        analysis_status: model ? [model.analysis_status.mechanics, model.analysis_status.rule_check] : []
      }
    ],
    error_message: null,
    resource: notReportedResource()
  };
}

/** Follows a backend job's non-terminal status: a queue reason the native
 * ledger reports while the job waits, and the move from queued to running.
 * Terminal states are recorded by their own transitions. */
export function progressSolveJob(current: SolveJobAuditState, status: BackendSolveJobStatus): SolveJobAuditState {
  if (current.backend_job_id !== status.job_id || (status.state !== "queued" && status.state !== "running")) return current;
  if (current.state !== "queued" && current.state !== "running" && current.state !== "cancelling") return current;
  const reason = status.state === "queued" ? status.queue_reason : null;
  const resource = status.resource && current.resource.source !== "native_job_receipt"
    ? { ...grantedResource(status.resource), queue_reason: current.resource.queue_reason }
    : current.resource;
  if (status.state === "running" && current.state === "queued") {
    return {
      ...current,
      state: "running",
      resource: { ...resource, queue_reason: null },
      events: [
        ...current.events,
        {
          event_id: "solve-preview-running",
          state: "running",
          message: `Backend solve job ${status.job_id} is executing; cancellation is cooperative at backend checkpoints (${status.cancellation_scope}); this service path does not stream percentage progress.`,
          result_available: false,
          diagnostic_count: 0,
          result_row_count: 0,
          analysis_status: []
        }
      ]
    };
  }
  if (sameRefusal(reason, resource.queue_reason)) return resource === current.resource ? current : { ...current, resource };
  return {
    ...current,
    resource: { ...resource, queue_reason: reason },
    events: reason
      ? [
          ...current.events,
          {
            event_id: "solve-preview-queued-waiting",
            state: current.state,
            message: `Backend solve job ${status.job_id} is waiting in the queue: ${describeResourceRefusal(reason)}`,
            result_available: false,
            diagnostic_count: 0,
            result_row_count: 0,
            analysis_status: []
          }
        ]
      : current.events
  };
}

/** A job the native memory ledger refused: a terminal `refused` poll, or a
 * refused take of a completed job's result. The typed refusal is recorded as is. */
export function refusedSolveJob(current: SolveJobAuditState, refusal: ResourceRefusal, pool: PoolStatus | null = null): SolveJobAuditState {
  return {
    ...current,
    state: "refused",
    resource: refusedResource(current.resource, refusal, pool),
    events: [
      ...current.events,
      {
        event_id: "solve-preview-resource-refused",
        state: "refused",
        message: `Refused by the native memory pool (${refusal.kind}): ${describeResourceRefusal(refusal)} No result was published.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ],
    error_message: null
  };
}

/** A start the native memory ledger refused before any job existed. */
export function refusedBeforeBackendStartSolveJob(model: PreviewModel, error: ResourceUnavailableError): SolveJobAuditState {
  return refusedSolveJob(
    { ...initialSolveJob(), job_id: `job:preview-linear-static:refused-before-start:${safeJobToken(model.project.id)}`, events: [] },
    error.refusal,
    error.pool
  );
}

function sameRefusal(left: ResourceRefusal | null, right: ResourceRefusal | null): boolean {
  return JSON.stringify(left) === JSON.stringify(right);
}

export function modelChangedSolveJob(outcome: OperationOutcome): SolveJobAuditState {
  return {
    ...initialSolveJob(),
    job_id: "job:preview-linear-static:model-changed",
    events: [
      {
        event_id: "solve-preview-model-changed",
        state: "not_started",
        message: `Model changed by applied structured operation ${outcome.operation_id}; previous mechanics results were cleared because they no longer describe the edited model. Run a new solve.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

export function blankProjectCreatedSolveJob(model: PreviewModel): SolveJobAuditState {
  return {
    ...initialSolveJob(),
    job_id: `job:preview-linear-static:blank:${safeJobToken(model.project.id)}`,
    events: [
      {
        event_id: "solve-preview-blank-project-created",
        state: "not_started",
        message:
          "Blank local model document created as the authoring target; no mechanics results exist until explicit entities and loads are added and a solve is run.",
        result_available: false,
        diagnostic_count: model.diagnostics.length,
        result_row_count: 0,
        analysis_status: [model.analysis_status.mechanics, model.analysis_status.rule_check]
      }
    ]
  };
}

export function sessionHistoryChangedSolveJob(action: "undo" | "redo" | "batch", operationId: string): SolveJobAuditState {
  const verb = action === "undo" ? "Undid" : action === "redo" ? "Redid" : "Applied batch";
  return {
    ...initialSolveJob(),
    job_id: `job:preview-linear-static:session-${action}`,
    events: [
      {
        event_id: `solve-preview-session-${action}`,
        state: "not_started",
        message: `${verb} local session model operation ${operationId}; previous mechanics results were cleared because they no longer describe the current model. Run a new solve.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

/** Polls a backend job to its terminal state; `onStatus` sees each status read,
 * so a queue reason can be shown while the job waits. */
export async function awaitBackendSolveJob(
  jobId: string,
  onStatus?: (status: BackendSolveJobStatus) => void
): Promise<BackendSolveJobStatus> {
  return awaitPreviewMechanicsJob(jobId, onStatus);
}

export function cancelledSolveJob(current: SolveJobAuditState, status: BackendSolveJobStatus): SolveJobAuditState {
  return {
    ...current,
    state: "cancelled",
    cancellation_requested: true,
    cancellation_status: status.cancellation_status,
    resource: { ...current.resource, queue_reason: null },
    events: [
      ...current.events,
      {
        event_id: "solve-preview-cancelled",
        state: "cancelled",
        message: `Backend solve job ${status.job_id} stopped at a cooperative checkpoint (${status.cancellation_status}); no result was published and no cancellation-success guarantee is claimed.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ],
    error_message: status.error_message
  };
}

export function recordBackendCancellationReceipt(
  current: SolveJobAuditState,
  receipt: BackendSolveJobCancellationReceipt
): SolveJobAuditState {
  return {
    ...current,
    cancellation_status: receipt.cancellation_status,
    events: [
      ...current.events,
      {
        event_id: "solve-preview-cancel-receipt",
        state: current.state,
        message: `Backend cancellation receipt for ${receipt.job_id}: accepted=${String(receipt.accepted)}; status=${receipt.cancellation_status}; job_state=${receipt.job_state}; no cancellation success is claimed.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

export function recordBackendCancellationFailure(current: SolveJobAuditState, error: unknown): SolveJobAuditState {
  return {
    ...current,
    cancellation_status: "backend_cancellation_request_failed",
    events: [
      ...current.events,
      {
        event_id: "solve-preview-cancel-request-failed",
        state: current.state,
        message: `Backend cancellation request failed to reach the job registry: ${String(error)}. The solve job continues under its own state reporting.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

export function completeSolveJob(
  current: SolveJobAuditState,
  result: MechanicsResult,
  analysisRun: AnalysisRunEnvelope
): SolveJobAuditState {
  const cancellationStatus = current.cancellation_requested
    ? "request_recorded_run_completed_before_cancellation_took_effect"
    : "not_requested";
  return {
    ...current,
    state: "completed",
    cancellation_status: cancellationStatus,
    resource: { ...current.resource, queue_reason: null },
    events: [
      ...current.events,
      {
        event_id: "solve-preview-completed",
        state: "completed",
        message: `Preview mechanics completed with ${result.results.length} result rows bound to ${analysisRun.analysis_run.run_id}.`,
        result_available: true,
        diagnostic_count: result.diagnostics.length,
        result_row_count: result.results.length,
        analysis_status: analysisRun.analysis_run.analysis_status
      }
    ],
    error_message: null
  };
}

function restoredSolveJob(result: MechanicsResult, analysisRun: AnalysisRunEnvelope): SolveJobAuditState {
  return {
    job_id: `job:preview-linear-static:restored:${safeJobToken(analysisRun.analysis_run.run_id)}`,
    state: "completed",
    progress_basis: "restored_persisted_run_record_no_new_solve_executed",
    percentages_synthesized: false,
    backend_percent_stream_available: false,
    cancellation_requested: false,
    cancellation_status: "not_requested",
    backend_job_seam: "restored_persisted_run_no_new_solve",
    backend_job_id: null,
    backend_cancellation_token: NO_BACKEND_JOB_TOKEN,
    events: [
      {
        event_id: "solve-preview-restored",
        state: "completed",
        message: `Restored persisted preview mechanics run ${analysisRun.analysis_run.run_id} from the local project store; no new solve was executed in this session.`,
        result_available: true,
        diagnostic_count: result.diagnostics.length,
        result_row_count: result.results.length,
        analysis_status: analysisRun.analysis_run.analysis_status
      }
    ],
    error_message: null,
    resource: notReportedResource()
  };
}

export function failSolveJob(current: SolveJobAuditState, error: unknown): SolveJobAuditState {
  // A typed resource refusal is recorded as a refusal, never as a generic failure.
  if (error instanceof ResourceUnavailableError) return refusedSolveJob(current, error.refusal, error.pool);
  return {
    ...current,
    state: "failed",
    resource: { ...current.resource, queue_reason: null },
    events: [
      ...current.events,
      {
        event_id: "solve-preview-failed",
        state: "failed",
        message: `Preview mechanics failed: ${String(error)}`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: ["MODEL_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"]
      }
    ],
    error_message: String(error)
  };
}

export function requestSolveCancellation(current: SolveJobAuditState): SolveJobAuditState {
  if (current.state !== "running" && current.state !== "queued") return current;
  const backendJob = current.backend_job_seam === "tauri_backend_job";
  return {
    ...current,
    state: "cancelling",
    cancellation_requested: true,
    cancellation_status: backendJob
      ? "request_sent_to_backend_job_awaiting_receipt"
      : "request_recorded_no_backend_job_in_browser_fixture_mode",
    events: [
      ...current.events,
      {
        event_id: "solve-preview-cancel-requested",
        state: "cancelling",
        message: backendJob
          ? `Cancellation requested for backend solve job ${current.backend_job_id} using its backend cancellation token; cancellation is cooperative at backend checkpoints and success is not guaranteed.`
          : "Cancellation request recorded at the UI boundary; no backend job exists in browser fixture mode, so the in-flight fixture run cannot be interrupted.",
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

export function pendingBackendStartCancellationSolveJob(current: SolveJobAuditState): SolveJobAuditState {
  return {
    ...initialSolveJob(),
    job_id: "job:preview-linear-static:pending-start-cancel",
    state: "cancelling",
    cancellation_requested: true,
    cancellation_status: "request_recorded_awaiting_backend_job_start",
    events: [
      ...current.events,
      {
        event_id: "solve-preview-cancel-requested-before-backend-start",
        state: "cancelling",
        message:
          "Cancellation requested before backend job creation completed; the active solve generation will stop before start or dispatch exactly one cooperative cancellation when its backend receipt arrives.",
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

export function cancelledBeforeBackendStartSolveJob(model: PreviewModel): SolveJobAuditState {
  return {
    ...initialSolveJob(),
    job_id: `job:preview-linear-static:cancelled-before-start:${safeJobToken(model.project.id)}`,
    state: "cancelled",
    cancellation_requested: true,
    cancellation_status: "cancelled_before_backend_job_start",
    events: [
      {
        event_id: "solve-preview-cancelled-before-backend-start",
        state: "cancelled",
        message:
          "The active solve generation was cancelled before any backend job was started; no result was computed or published.",
        result_available: false,
        diagnostic_count: model.diagnostics.length,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

export function cancelledWithoutBackendSolveJob(current: SolveJobAuditState): SolveJobAuditState {
  return {
    ...current,
    state: "cancelled",
    cancellation_requested: true,
    cancellation_status: "cancelled_before_browser_fixture_result_publication",
    events: [
      ...current.events,
      {
        event_id: "solve-preview-browser-fixture-cancelled",
        state: "cancelled",
        message:
          "The browser fixture solve generation was cancelled before result publication; no backend cancellation-success claim is made.",
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ]
  };
}

function safeJobToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}

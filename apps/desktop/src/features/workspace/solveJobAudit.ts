// The solve-job audit record (`SolveJobAuditState`): its starting value, the
// transitions published as a solve starts, is cancelled, completes or fails, the
// records that replace it when the model changes, and the poll that waits for a
// backend job. No React.

import { awaitPreviewMechanicsJob, describeResultTooLarge, ResultTooLargeError, resultTooLargeCode } from "../../services/previewService";
import type {
  BackendSolveJobCancellationReceipt,
  BackendSolveJobStatus,
  ResultTooLarge,
  SolveJobStartReceipt
} from "../../services/previewService";
import { describeResourceRefusal, describeSolveWorkerFailure, formatBytes, resourceRefusalCode, ResourceUnavailableError } from "../../services/memoryPool";
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
    worker_grant_bytes: null,
    worker_peak_grant_bytes: null,
    worker_peak_footprint_bytes: null,
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

const largest = (recorded: number | null, reported: number | null) =>
  recorded === null ? reported : reported === null ? recorded : Math.max(recorded, reported);

/** A later report of the same job's grant: the worker's current grant follows
 * the native report; its peak grant and peak footprint never fall below values
 * already recorded. */
function withWorkerGrant(current: SolveJobResourceRecord, grant: SolveResourceGrant): SolveJobResourceRecord {
  const peak = largest(current.worker_peak_grant_bytes, grant.worker_peak_grant_bytes);
  const footprint = largest(current.worker_peak_footprint_bytes, grant.worker_peak_footprint_bytes);
  if (current.worker_grant_bytes === grant.worker_grant_bytes && current.worker_peak_grant_bytes === peak
    && current.worker_peak_footprint_bytes === footprint) return current;
  return { ...current, worker_grant_bytes: grant.worker_grant_bytes, worker_peak_grant_bytes: peak, worker_peak_footprint_bytes: footprint };
}

/** What cancelling a backend job does: a queued job is removed before a worker
 * starts; a running job's worker process is terminated, and the job reads
 * cancelled (no result published) only after that worker has exited. */
const BACKEND_CANCELLATION = "cancelling a queued job removes it before a worker starts; cancelling a running job terminates its worker process, and the job reads cancelled (no result published) only after the worker has exited";

function grantSummary(grant: SolveResourceGrant | null): string {
  return grant
    ? `per-invocation budget ${formatBytes(grant.invocation_budget_bytes)} (bounds the model intake and caps the ordinary solve worker, whose grant grows on demand up to it); pool ${formatBytes(grant.pool_bytes)}; model intake reserved ${formatBytes(grant.ingress_reserved_bytes)}; route ordinary_worker_capped; worker grant ${grant.worker_grant_bytes === null ? "not yet held" : formatBytes(grant.worker_grant_bytes)}`
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
    backend_cancellation_scope: null,
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
      backend_cancellation_scope: startReceipt.cancellation_scope || null,
      events: [
        {
          event_id: "solve-preview-queued",
          state: "queued",
          message: `Backend solve job ${startReceipt.job_id} queued with a backend cancellation token through the application service boundary; ${grantSummary(startReceipt.resource)}; ${BACKEND_CANCELLATION} (scope ${startReceipt.cancellation_scope}); this service path does not stream percentage progress.`,
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
    backend_cancellation_scope: null,
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

/** Follows a backend job's status: the worker grant it reports, a queue reason
 * the native ledger reports while the job waits, and the move from queued to
 * running. Terminal states are recorded by their own transitions. */
export function progressSolveJob(current: SolveJobAuditState, status: BackendSolveJobStatus): SolveJobAuditState {
  if (current.backend_job_id !== status.job_id) return current;
  if (current.state !== "queued" && current.state !== "running" && current.state !== "cancelling") return current;
  // Every status of this job, terminal ones included, carries the worker's
  // current and peak grant; the record follows it before any terminal transition.
  const resource = !status.resource ? current.resource
    : current.resource.source !== "native_job_receipt"
      ? { ...grantedResource(status.resource), queue_reason: current.resource.queue_reason }
      : withWorkerGrant(current.resource, status.resource);
  if (status.state !== "queued" && status.state !== "running") {
    return resource === current.resource ? current : { ...current, resource };
  }
  const reason = status.state === "queued" ? status.queue_reason : null;
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
          message: `Backend solve job ${status.job_id} is executing in its worker process; cancelling it terminates that worker process, and the job reads cancelled (no result published) only after the worker has exited (scope ${status.cancellation_scope}); this service path does not stream percentage progress.`,
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

/** Where in the native solve path a memory refusal stood (DEL-04-07
 * memory-budget decision, "Refusals are typed"). It is read from the record's
 * existing fields: the refused event's id, whether a backend job existed, and
 * the refusal's kind. */
export type SolveRefusalPoint =
  /** The start command: intake, queue or early retention admission; no job ran. */
  | "before_start"
  /** The worker while it ran: its growth, cap or footprint (`worker_*`). */
  | "worker"
  /** The finished result's lease, taken while the worker's grant is still held. */
  | "result_handoff"
  /** Retention of the finished result (`retention_full`). */
  | "publication"
  /** The take of a completed job's result into the app. */
  | "take"
  /** A native command of the session's own solve steps (an interactive admission). */
  | "session_step";

/** The refused event of a refusal at a native job's terminal state or at its start. */
const REFUSED_EVENT = "solve-preview-resource-refused";
/** The refused event of a refused take of a completed job's result. */
const REFUSED_AT_TAKE_EVENT = "solve-preview-resource-refused-at-take";
/** The refused event of a refused native command in the session's solve steps. */
const REFUSED_AT_SESSION_STEP_EVENT = "solve-preview-resource-refused-at-session-step";

/** Where the refusal recorded on a `refused` job stood; null for any other job. */
export function solveRefusalPoint(job: SolveJobAuditState): SolveRefusalPoint | null {
  const refusal = job.resource.refusal;
  if (job.state !== "refused" || !refusal) return null;
  const event = [...job.events].reverse().find((item) => item.state === "refused")?.event_id;
  return refusalPoint(event === REFUSED_AT_TAKE_EVENT ? "take" : event === REFUSED_AT_SESSION_STEP_EVENT ? "session_step" : "job", job.backend_job_id, refusal);
}

function refusalPoint(at: "job" | "take" | "session_step", backendJobId: string | null, refusal: ResourceRefusal): SolveRefusalPoint {
  if (at !== "job") return at;
  if (backendJobId === null) return "before_start";
  // A running job's own refusals are typed `worker_*`; at publication only the
  // retention bound refuses; any other refusal of a job is its result lease.
  if (refusal.kind.startsWith("worker_")) return "worker";
  if (refusal.kind === "retention_full") return "publication";
  return "result_handoff";
}

/** What happened at the refusal point and the retry path the design allows,
 * in plain language. It never promises that a retry will be admitted. */
export function describeSolveRefusalRetry(refusal: ResourceRefusal, point: SolveRefusalPoint): string {
  // The intake, a worker growth step, the result lease and the take each
  // re-sample a refusal that is only `headroom_insufficient` for up to 2 s.
  const headroom = refusal.kind === "worker_growth_refused" ? refusal.cause : refusal;
  const resampled = headroom.kind === "headroom_insufficient" ? " Available memory was re-sampled for up to 2 s before this refusal stood." : "";
  const where = {
    before_start: `Refused before the solve started; no job ran.${resampled}`,
    worker: `Refused while the solve ran: its worker process was stopped, with no fallback to another solver mode or route.${resampled}`,
    result_handoff: `Refused at the result hand-off after the solve finished; the finished result was dropped.${resampled}`,
    publication: "Refused at publication after the solve finished; the finished result was dropped.",
    take: `Refused when the finished result was handed to the app; the session does not retry a refused take, so it released the result.${resampled}`,
    session_step: "Refused at a native command of this solve's session steps; the session does not retry it."
  }[point];
  return `${where} ${solveRetryPath(refusal)}`;
}

/** The user's retry path for a refused solve: running it again, which is
 * admitted only if the pool, headroom and limits allow it then. */
function solveRetryPath(refusal: ResourceRefusal): string {
  switch (refusal.kind) {
    case "pressure_elevated":
      return "You can run the solve again once macOS memory pressure is back to normal.";
    case "queue_full":
      return "You can run the solve again once a queued solve has started.";
    case "retention_full":
      return "You can run the solve again once a waiting result has been collected or released.";
    case "custody_full":
      return "You can run the solve again once a held result has been released.";
    case "ingress_exceeds_budget":
    case "worker_cap_exceeded":
    case "worker_footprint_exceeded":
      return "This solve needed more than the per-invocation budget allows; a larger budget, up to the pool, can be set under Memory pool before running the solve again.";
    case "ingress_invalid":
      return "The request itself could not be received; running it again unchanged is refused the same way.";
    case "worker_growth_refused":
      return solveRetryPath(refusal.cause);
    default:
      return "You can run the solve again when more memory is available.";
  }
}

/** A job the native memory ledger refused: a terminal `refused` poll, a
 * refused start (`before_start`), a refused take of a completed job's result
 * (`take`), or a refused native command of the session's solve steps
 * (`session_step`). The typed refusal is recorded as is; the event names the
 * refusal point, the exact code and the retry path. */
export function refusedSolveJob(
  current: SolveJobAuditState,
  refusal: ResourceRefusal,
  pool: PoolStatus | null = null,
  at: "job" | "take" | "session_step" = "job"
): SolveJobAuditState {
  const point = refusalPoint(at, current.backend_job_id, refusal);
  return {
    ...current,
    state: "refused",
    resource: refusedResource(current.resource, refusal, pool),
    events: [
      ...current.events,
      {
        event_id: at === "take" ? REFUSED_AT_TAKE_EVENT : at === "session_step" ? REFUSED_AT_SESSION_STEP_EVENT : REFUSED_EVENT,
        state: "refused",
        message: `Refused by the native memory pool (${refusal.kind}): ${describeResourceRefusal(refusal)} No result was published. ${describeSolveRefusalRetry(refusal, point)} Refusal code: ${resourceRefusalCode(refusal)}.`,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ],
    error_message: null
  };
}

/** The refused event of a job whose result was too large to display. */
const RESULT_TOO_LARGE_EVENT = "solve-preview-result-too-large";

/** A job the native host refused because its result is larger than the
 * desktop can display. It is not a memory-pool refusal: the job's resource
 * record keeps no refusal, and the plain reading, with the code and exact
 * figures, is the job's message. No result exists in the app. */
export function resultTooLargeSolveJob(current: SolveJobAuditState, refusal: ResultTooLarge): SolveJobAuditState {
  const message = `${describeResultTooLarge(refusal)} Code: ${resultTooLargeCode(refusal)}.`;
  return {
    ...current,
    state: "refused",
    resource: { ...current.resource, queue_reason: null },
    events: [
      ...current.events,
      {
        event_id: RESULT_TOO_LARGE_EVENT,
        state: "refused",
        message,
        result_available: false,
        diagnostic_count: 0,
        result_row_count: 0,
        analysis_status: []
      }
    ],
    error_message: message
  };
}

/** Whether a job was refused because its result is too large to display. */
export function isResultTooLargeSolveJob(job: SolveJobAuditState): boolean {
  return job.state === "refused" && [...job.events].reverse().find((item) => item.state === "refused")?.event_id === RESULT_TOO_LARGE_EVENT;
}

/** A completed job whose result take the native memory ledger refused. */
export function refusedAtTakeSolveJob(current: SolveJobAuditState, error: ResourceUnavailableError): SolveJobAuditState {
  return refusedSolveJob(current, error.refusal, error.pool, "take");
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
        message: `Backend solve job ${status.job_id} reads cancelled (${status.cancellation_status}): a queued job was removed before a worker started, or a running job's worker process was terminated and has exited; no result was published, and no cancellation-success claim is made beyond that.`,
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
    backend_cancellation_scope: null,
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
  // The take and the start record their own refusals; one reaching here came
  // from a native command of the session's solve steps.
  if (error instanceof ResourceUnavailableError) return refusedSolveJob(current, error.refusal, error.pool, "session_step");
  if (error instanceof ResultTooLargeError) return resultTooLargeSolveJob(current, error.refusal);
  // A crashed or protocol-breaking solve worker is a failure, never a refusal.
  const worker = describeSolveWorkerFailure(String(error));
  return {
    ...current,
    state: "failed",
    resource: { ...current.resource, queue_reason: null },
    events: [
      ...current.events,
      {
        event_id: "solve-preview-failed",
        state: "failed",
        message: `Preview mechanics failed: ${String(error)}${worker ? ` (${worker})` : ""}`,
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
          ? `Cancellation requested for backend solve job ${current.backend_job_id} using its backend cancellation token; ${BACKEND_CANCELLATION}; no cancellation-success guarantee is claimed beyond that.`
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
          "Cancellation requested before backend job creation completed; the active solve generation will stop before start or dispatch exactly one cancellation when its backend receipt arrives.",
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

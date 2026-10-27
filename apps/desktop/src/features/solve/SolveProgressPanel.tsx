import { Square, X } from "lucide-react";
import { useEffect, useId, useRef, useState } from "react";
import type { SolveJobAuditState, SolveJobEvent } from "../../types";
import { describeResourceRefusal, describeSolveWorkerFailure, formatBytes, resourceRefusalCode } from "../../services/memoryPool";
import { describeSolveRefusalRetry, solveRefusalPoint } from "../workspace/solveJobAudit";

/** The words for each job state the solve-job record carries. */
const STATE_LABELS: Record<SolveJobEvent["state"], string> = {
  not_started: "Not started",
  queued: "Queued",
  running: "Running",
  cancelling: "Cancelling",
  cancelled: "Cancelled",
  completed: "Completed",
  failed: "Failed",
  refused: "Refused"
};

const ACTIVE_STATES: ReadonlySet<SolveJobEvent["state"]> = new Set(["queued", "running", "cancelling"]);

export function solveJobStateLabel(state: SolveJobEvent["state"]): string {
  return STATE_LABELS[state];
}

/**
 * The solve progress panel (DEL-07-07; Design `solve-progress-panel.md`). From
 * the moment a solve starts until the user closes it, the panel shows the job's
 * live record as the session's solve-job audit already holds it: the state, the
 * latest event's message (the stage), every event in order, the queue reason
 * and worker memory the native host reports, the cancellation status, and the
 * terminal completion, failure or refusal with its existing fields. It shows no
 * percentage, time estimate or elapsed time: the job reports states only
 * (`percentages_synthesized=false`) and its events carry no timestamps.
 */
export function SolveProgressPanel({
  solveJob,
  running,
  onCancel
}: {
  solveJob: SolveJobAuditState;
  running: boolean;
  onCancel: () => void;
}) {
  const [open, setOpen] = useState(false);
  const titleId = useId();
  const stateRef = useRef<HTMLParagraphElement>(null);
  // A run opens the panel; a record with no job (a model change, a new project) closes it.
  useEffect(() => {
    if (running) setOpen(true);
  }, [running]);
  useEffect(() => {
    if (solveJob.state === "not_started") setOpen(false);
  }, [solveJob.state]);

  if (!open || solveJob.state === "not_started") return null;

  const state = solveJob.state;
  const active = ACTIVE_STATES.has(state);
  const latest = solveJob.events[solveJob.events.length - 1] ?? null;
  const canCancel = running && (state === "queued" || state === "running");
  const refusal = solveJob.resource.refusal;
  const refusalPoint = solveRefusalPoint(solveJob);
  const queueReason = solveJob.resource.queue_reason;
  const workerFailure = state === "failed" ? describeSolveWorkerFailure(solveJob.error_message) : null;
  const completed = state === "completed" ? [...solveJob.events].reverse().find((event) => event.state === "completed") ?? null : null;
  const resource = solveJob.resource;

  function cancel() {
    if (!canCancel) return;
    onCancel();
    // The Cancel control stays where it is (aria-disabled) so keyboard focus is not lost.
    stateRef.current?.focus();
  }

  return (
    <section
      aria-labelledby={titleId}
      className={`solve-progress-panel ${state}`}
      data-state={state}
      data-testid="solve-progress-panel"
      role="region"
    >
      <header className="solve-progress-header">
        <h2 id={titleId}>Solve progress</h2>
        <code data-testid="solve-progress-job">{solveJob.job_id}</code>
      </header>
      <p
        aria-live="polite"
        className="solve-progress-state"
        data-testid="solve-progress-state"
        ref={stateRef}
        role="status"
        tabIndex={-1}
      >
        <strong>{solveJobStateLabel(state)}</strong>
        {latest ? <span data-testid="solve-progress-stage">{latest.message}</span> : null}
      </p>
      <p className="solve-progress-basis" data-testid="solve-progress-basis">
        {solveJob.percentages_synthesized || solveJob.backend_percent_stream_available
          ? `Progress basis: ${solveJob.progress_basis}.`
          : `The solve job reports its state and events, not a percentage, a time estimate or a start time, so none is shown (progress basis: ${solveJob.progress_basis}).`}
      </p>
      {queueReason && (state === "queued" || state === "cancelling") ? (
        <p className="memory-pool-message info" data-testid="solve-progress-queue-reason">
          Waiting in the queue: {describeResourceRefusal(queueReason)} Code: <code>{resourceRefusalCode(queueReason)}</code>
        </p>
      ) : null}
      {active && resource.route_pricing === "ordinary_worker_capped" ? (
        <p className="solve-progress-line" data-testid="solve-progress-worker">
          Worker memory: grant {resource.worker_grant_bytes === null ? "not yet held" : formatBytes(resource.worker_grant_bytes)}; peak grant{" "}
          {resource.worker_peak_grant_bytes === null ? "none" : formatBytes(resource.worker_peak_grant_bytes)}; peak footprint{" "}
          {resource.worker_peak_footprint_bytes === null ? "not measured" : formatBytes(resource.worker_peak_footprint_bytes)}; budget{" "}
          {formatBytes(resource.invocation_budget_bytes)}
        </p>
      ) : null}
      {solveJob.cancellation_requested ? (
        <p className="solve-progress-line" data-testid="solve-progress-cancellation">
          Cancellation requested: <code>{solveJob.cancellation_status}</code>
        </p>
      ) : null}
      {completed ? (
        <p className="memory-pool-message info" data-testid="solve-progress-completed">
          {completed.result_row_count} result {completed.result_row_count === 1 ? "row" : "rows"}; {completed.diagnostic_count}{" "}
          {completed.diagnostic_count === 1 ? "diagnostic" : "diagnostics"}.
        </p>
      ) : null}
      {state === "refused" && refusal ? (
        <div className="memory-pool-message warning" data-testid="solve-progress-refusal">
          <p>Refused by the native memory pool: {describeResourceRefusal(refusal)} No result was published.</p>
          {refusalPoint ? <p data-testid="solve-progress-retry">{describeSolveRefusalRetry(refusal, refusalPoint)}</p> : null}
          <p>
            Refusal code: <code data-testid="solve-progress-refusal-code">{resourceRefusalCode(refusal)}</code>
          </p>
        </div>
      ) : null}
      {state === "failed" ? (
        <p className="memory-pool-message error" data-testid="solve-progress-failure">
          {workerFailure ?? solveJob.error_message ?? "The solve failed."}
        </p>
      ) : null}
      <ol aria-label="Solve job events" className="solve-progress-events" data-testid="solve-progress-events">
        {solveJob.events.map((event, index) => (
          <li data-event-id={event.event_id} data-state={event.state} key={`${event.event_id}-${index}`}>
            <strong>{solveJobStateLabel(event.state)}</strong> <span>{event.message}</span>
          </li>
        ))}
      </ol>
      <div className="solve-progress-actions">
        {active ? (
          <button
            aria-disabled={!canCancel}
            className="secondary-action"
            data-testid="solve-progress-cancel"
            onClick={cancel}
            title={canCancel ? undefined : "Cancellation was requested; the panel waits for the job's final state."}
            type="button"
          >
            <Square size={14} aria-hidden="true" />
            {state === "cancelling" ? "Cancelling" : "Cancel solve"}
          </button>
        ) : (
          <button className="secondary-action" data-testid="solve-progress-close" onClick={() => setOpen(false)} type="button">
            <X size={14} aria-hidden="true" />
            Close
          </button>
        )}
      </div>
    </section>
  );
}

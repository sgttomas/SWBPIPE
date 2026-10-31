import { Square, X } from "lucide-react";
import { useId, useReducer, useRef } from "react";
import type { SolveJobAuditState, SolveJobEvent } from "../../types";
import { describeResourceRefusal, describeSolveWorkerFailure, formatBytes, resourceRefusalCode } from "../../services/memoryPool";
import { describeSolveRefusalRetry, isResultTooLargeSolveJob, solveRefusalPoint } from "../workspace/solveJobAudit";

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

/** What the panel shows: nothing, the current run before its job has reported,
 * or the current run's job record. `earlier` is the record the session held
 * when the run started; none of its events belong to this run. */
type RunView =
  | Readonly<{ kind: "hidden" }>
  | Readonly<{ kind: "starting"; earlier: readonly SolveJobEvent[] }>
  | Readonly<{ kind: "job"; job: SolveJobAuditState; earlier: readonly SolveJobEvent[] }>;

type RunTracker = {
  /** `running` at the previous render. */
  wasRunning: boolean;
  /** The record of the last render while no run was active. */
  idle: SolveJobAuditState;
  /** The record held when the current (or last) run started; null before any run. */
  startedFrom: SolveJobAuditState | null;
  /** The job id of the last record the current (or last) run produced. */
  runJobId: string | null;
  closed: boolean;
};

/**
 * Binds the panel to the current run (Codex review r1 of #1665): the session
 * turns `running` on before the run's first record replaces the previous one,
 * so the record held at the start, and its events, are never shown as this
 * run's. While the run is active the panel shows its records, or "Starting"
 * until the first one; afterwards it keeps the run's last job until the user
 * closes it or the session replaces it with another job's record.
 */
function runView(solveJob: SolveJobAuditState, running: boolean, tracker: RunTracker): RunView {
  if (running && !tracker.wasRunning) {
    tracker.startedFrom = tracker.idle;
    tracker.runJobId = null;
    tracker.closed = false;
  }
  if (tracker.startedFrom !== null && solveJob !== tracker.startedFrom && (running || tracker.wasRunning)) {
    tracker.runJobId = solveJob.job_id;
  }
  if (solveJob.state === "not_started") tracker.runJobId = null;
  if (!running) tracker.idle = solveJob;
  tracker.wasRunning = running;

  const startedFrom = tracker.startedFrom;
  if (startedFrom === null || tracker.closed) return { kind: "hidden" };
  const earlier = startedFrom.events;
  // A run that has not published a record yet shows Starting, whatever the
  // record it started from, including one that names no job (`not_started`,
  // a first run or a run after a model reset; Codex review r2 of #1665).
  if (running && solveJob === startedFrom) return { kind: "starting", earlier };
  if (solveJob.state === "not_started") return { kind: "hidden" };
  if (running) return { kind: "job", job: solveJob, earlier };
  return tracker.runJobId !== null && solveJob.job_id === tracker.runJobId ? { kind: "job", job: solveJob, earlier } : { kind: "hidden" };
}

/**
 * The solve progress panel (DEL-07-07; Design `solve-progress-panel.md`). From
 * the moment a solve starts until the user closes it, the panel shows that
 * run's job record as the session's solve-job audit holds it: the state, the
 * latest event's message (the stage), the run's events in order, the queue
 * reason and worker memory the native host reports, the cancellation status,
 * and the terminal completion, failure or refusal with its existing fields. It
 * shows no percentage, time estimate or elapsed time: the job reports states
 * only (`percentages_synthesized=false`) and its events carry no timestamps.
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
  const titleId = useId();
  const stateRef = useRef<HTMLParagraphElement>(null);
  const [, rerender] = useReducer((count: number) => count + 1, 0);
  const tracker = useRef<RunTracker>({ wasRunning: false, idle: solveJob, startedFrom: null, runJobId: null, closed: false });
  const view = runView(solveJob, running, tracker.current);

  if (view.kind === "hidden") return null;

  const job = view.kind === "job" ? view.job : null;
  const events = job ? job.events.filter((event) => !view.earlier.includes(event)) : [];
  const latest = events[events.length - 1] ?? null;
  const state = job?.state ?? null;
  const active = state === null || ACTIVE_STATES.has(state);
  // Before the job reports, Cancel stops the run before its start (the session's handler).
  const canCancel = running && (state === null || state === "queued" || state === "running");
  const refusal = job?.resource.refusal ?? null;
  const refusalPoint = job ? solveRefusalPoint(job) : null;
  const queueReason = job?.resource.queue_reason ?? null;
  const workerFailure = job && state === "failed" ? describeSolveWorkerFailure(job.error_message) : null;
  const completed = state === "completed" ? [...events].reverse().find((event) => event.state === "completed") ?? null : null;
  const resource = job?.resource ?? null;
  const basis = job ?? solveJob;

  function cancel() {
    if (!canCancel) return;
    onCancel();
    // The Cancel control stays where it is (aria-disabled) so keyboard focus is not lost.
    stateRef.current?.focus();
  }

  function close() {
    tracker.current.closed = true;
    rerender();
  }

  return (
    <section
      aria-labelledby={titleId}
      className={`solve-progress-panel ${state ?? "starting"}`}
      data-state={state ?? "starting"}
      data-testid="solve-progress-panel"
      role="region"
    >
      <header className="solve-progress-header">
        <h2 id={titleId}>Solve progress</h2>
        {job ? <code data-testid="solve-progress-job">{job.job_id}</code> : null}
      </header>
      <p
        aria-live="polite"
        className="solve-progress-state"
        data-testid="solve-progress-state"
        ref={stateRef}
        role="status"
        tabIndex={-1}
      >
        <strong>{state ? solveJobStateLabel(state) : "Starting"}</strong>
        {latest ? <span data-testid="solve-progress-stage">{latest.message}</span> : null}
        {state === null ? <span data-testid="solve-progress-stage">The solve was requested; its job has not reported a state yet.</span> : null}
      </p>
      <p className="solve-progress-basis" data-testid="solve-progress-basis">
        {basis.percentages_synthesized || basis.backend_percent_stream_available
          ? `Progress basis: ${basis.progress_basis}.`
          : `The solve job reports its state and events, not a percentage, a time estimate or a start time, so none is shown (progress basis: ${basis.progress_basis}).`}
      </p>
      {queueReason && (state === "queued" || state === "cancelling") ? (
        <p className="memory-pool-message info" data-testid="solve-progress-queue-reason">
          Waiting in the queue: {describeResourceRefusal(queueReason)} Code: <code>{resourceRefusalCode(queueReason)}</code>
        </p>
      ) : null}
      {active && resource?.route_pricing === "ordinary_worker_capped" ? (
        <p className="solve-progress-line" data-testid="solve-progress-worker">
          Worker memory: grant {resource.worker_grant_bytes === null ? "not yet held" : formatBytes(resource.worker_grant_bytes)}; peak grant{" "}
          {resource.worker_peak_grant_bytes === null ? "none" : formatBytes(resource.worker_peak_grant_bytes)}; peak footprint{" "}
          {resource.worker_peak_footprint_bytes === null ? "not measured" : formatBytes(resource.worker_peak_footprint_bytes)}; budget{" "}
          {formatBytes(resource.invocation_budget_bytes)}
        </p>
      ) : null}
      {job?.cancellation_requested ? (
        <p className="solve-progress-line" data-testid="solve-progress-cancellation">
          Cancellation requested: <code>{job.cancellation_status}</code>
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
      {job && isResultTooLargeSolveJob(job) ? (
        <p className="memory-pool-message warning" data-testid="solve-progress-result-too-large">{job.error_message}</p>
      ) : null}
      {state === "failed" ? (
        <p className="memory-pool-message error" data-testid="solve-progress-failure">
          {workerFailure ?? job?.error_message ?? "The solve failed."}
        </p>
      ) : null}
      <ol aria-label="Solve job events" className="solve-progress-events" data-testid="solve-progress-events">
        {events.map((event, index) => (
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
          <button className="secondary-action" data-testid="solve-progress-close" onClick={close} type="button">
            <X size={14} aria-hidden="true" />
            Close
          </button>
        )}
      </div>
    </section>
  );
}

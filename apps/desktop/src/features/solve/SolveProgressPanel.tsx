import { ArrowRight, Flag, Square, X } from "lucide-react";
import { useEffect, useReducer, useRef, type RefObject } from "react";
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

/** What is being solved, as the line names it: the model's name and its load cases and combinations. */
export type SolveSubject = Readonly<{ name: string; loadCases: number; combinations: number }>;

/** What the line shows: nothing, the current run before its job has reported,
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
 * Binds the line to the current run (Codex review r1 of #1665): the session
 * turns `running` on before the run's first record replaces the previous one,
 * so the record held at the start, and its events, are never shown as this
 * run's. While the run is active the line shows its records, or "Starting"
 * until the first one; afterwards it keeps the run's last job until the user
 * dismisses it or the session replaces it with another job's record (an edit
 * of the model replaces it with a record of no job).
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

function plural(count: number, one: string, many: string): string {
  return `${count} ${count === 1 ? one : many}`;
}

function subjectText(subject: SolveSubject | null | undefined): string {
  if (!subject) return "the model";
  const parts = [plural(subject.loadCases, "load case", "load cases")];
  if (subject.combinations > 0) parts.push(plural(subject.combinations, "combination", "combinations"));
  return `${subject.name} (${parts.join(", ")})`;
}

/**
 * The solve status line (DEL-07-07 REQ-014): one line in the band under the
 * toolbar, from the moment a solve starts until the user dismisses it or the
 * model changes. It says what is being solved and the state the job reports,
 * with Stop while the job can be stopped; when the job completes, the result's
 * rows and diagnostics and "Open results"; when it fails or is refused, the
 * reason the job gives, the way to retry, and "Show in Issues". Everything
 * else the job reports (its identifier, its stage and events in order, the
 * progress basis, the queue reason's exact code, the worker's memory, the
 * cancellation status, a refusal's exact code) is behind the line's one closed
 * "Details" disclosure, complete. It shows no percentage, time estimate or
 * elapsed time: the job reports states only (`percentages_synthesized=false`)
 * and its events carry no timestamps. It is part of the page's layout, never
 * laid over the model, the Key results panel or the Insert controls.
 */
export function SolveProgressPanel({
  solveJob,
  running,
  onCancel,
  onOpenResults,
  onShowIssue,
  subject = null
}: {
  solveJob: SolveJobAuditState;
  running: boolean;
  onCancel: () => void;
  /** Opens the Results stage. */
  onOpenResults?: () => void;
  /** Opens the Issues drawer at the solve's failure or refusal. */
  onShowIssue?: () => void;
  subject?: SolveSubject | null;
}) {
  const stateRef = useRef<HTMLParagraphElement>(null);
  const lineRef = useRef<HTMLElement>(null);
  const [, rerender] = useReducer((count: number) => count + 1, 0);
  const tracker = useRef<RunTracker>({ wasRunning: false, idle: solveJob, startedFrom: null, runJobId: null, closed: false });
  const view = runView(solveJob, running, tracker.current);
  const shown = view.kind !== "hidden";
  useLineHeight(lineRef, shown);

  if (view.kind === "hidden") return null;

  const job = view.kind === "job" ? view.job : null;
  const events = job ? job.events.filter((event) => !view.earlier.includes(event)) : [];
  const latest = events[events.length - 1] ?? null;
  const state = job?.state ?? null;
  const active = state === null || ACTIVE_STATES.has(state);
  // Before the job reports, Stop stops the run before its start (the session's handler).
  const canCancel = running && (state === null || state === "queued" || state === "running");
  const refusal = job?.resource.refusal ?? null;
  const refusalPoint = job ? solveRefusalPoint(job) : null;
  const queueReason = job?.resource.queue_reason ?? null;
  const workerFailure = job && state === "failed" ? describeSolveWorkerFailure(job.error_message) : null;
  const completed = state === "completed" ? [...events].reverse().find((event) => event.state === "completed") ?? null : null;
  const tooLarge = job !== null && isResultTooLargeSolveJob(job);
  const resource = job?.resource ?? null;
  const basis = job ?? solveJob;
  const solved = completed?.analysis_status.includes("MECHANICS_SOLVED") ?? false;
  const label = state === null ? "Starting" : state === "completed" && solved ? "Solved" : solveJobStateLabel(state);
  const tone = state === "failed" || state === "refused" ? "error" : state === "completed" ? "done" : state === "cancelled" ? "stopped" : "active";
  const issue = state === "failed" || state === "refused" || tooLarge;

  function cancel() {
    if (!canCancel) return;
    onCancel();
    // Stop stays where it is (aria-disabled) so keyboard focus is not lost.
    stateRef.current?.focus();
  }

  function close() {
    tracker.current.closed = true;
    rerender();
  }

  return (
    <section
      aria-label="Solve progress"
      className={`solve-status-line ${tone}`}
      data-state={state ?? "starting"}
      data-testid="solve-progress-panel"
      ref={lineRef}
      role="region"
    >
      <div className="solve-status-main">
        <p
          aria-live="polite"
          className="solve-status-text"
          data-testid="solve-progress-state"
          ref={stateRef}
          role="status"
          tabIndex={-1}
        >
          <strong>{label}</strong>{" "}
          {state === null ? <span>Solving {subjectText(subject)}; the solve was requested and its job has not reported a state yet.</span> : null}
          {state === "queued" ? <span>{subjectText(subject)} waits for a solve worker.</span> : null}
          {state === "running" ? <span>{subjectText(subject)}.</span> : null}
          {state === "cancelling" ? <span>Stop was requested; the line waits for the job&apos;s final state.</span> : null}
          {state === "cancelled" ? <span>The solve was stopped before it finished; no result was published.</span> : null}
          {completed ? (
            <span>
              {subjectText(subject)}: <span data-testid="solve-progress-completed">{completed.result_row_count} result {completed.result_row_count === 1 ? "row" : "rows"}.</span>
            </span>
          ) : null}
          {queueReason && (state === "queued" || state === "cancelling") ? (
            <span data-testid="solve-progress-queue-reason"> Waiting in the queue: {describeResourceRefusal(queueReason)}</span>
          ) : null}
          {state === "refused" && refusal ? (
            <span data-testid="solve-progress-refusal">Refused by the native memory pool: {describeResourceRefusal(refusal)} No result was published.</span>
          ) : null}
          {state === "refused" && refusal && refusalPoint ? <span data-testid="solve-progress-retry"> {describeSolveRefusalRetry(refusal, refusalPoint)}</span> : null}
          {tooLarge ? <span data-testid="solve-progress-result-too-large">{job?.error_message}</span> : null}
          {state === "failed" ? (
            <span data-testid="solve-progress-failure">{workerFailure ?? job?.error_message ?? "The solve failed."}</span>
          ) : null}
        </p>
        <div className="solve-status-actions">
          {active ? (
            <button
              aria-disabled={!canCancel}
              className="secondary-action"
              data-testid="solve-progress-cancel"
              onClick={cancel}
              title={canCancel ? "Stop the solve" : "Stop was requested; the line waits for the job's final state."}
              type="button"
            >
              <Square size={14} aria-hidden="true" />
              {state === "cancelling" ? "Cancelling" : "Stop"}
            </button>
          ) : null}
          {state === "completed" && onOpenResults ? (
            <button className="primary-action" data-testid="solve-progress-open-results" onClick={onOpenResults} type="button">
              Open results
              <ArrowRight size={14} aria-hidden="true" />
            </button>
          ) : null}
          {issue && onShowIssue ? (
            <button className="secondary-action" data-testid="solve-progress-show-issue" onClick={onShowIssue} type="button">
              <Flag size={14} aria-hidden="true" />
              Show in Issues
            </button>
          ) : null}
          {active ? null : (
            <button aria-label="Dismiss" className="solve-status-dismiss" data-testid="solve-progress-close" onClick={close} title="Dismiss" type="button">
              <X size={14} aria-hidden="true" />
            </button>
          )}
        </div>
      </div>
      <details className="solve-status-details" data-testid="solve-progress-details">
        <summary data-testid="solve-progress-details-toggle">Details</summary>
        <div className="solve-status-details-body">
          {job ? <p>Job: <code data-testid="solve-progress-job">{job.job_id}</code></p> : null}
          {latest ? <p>Stage: <span data-testid="solve-progress-stage">{latest.message}</span></p> : null}
          {state === null ? <p>Stage: <span data-testid="solve-progress-stage">The solve was requested; its job has not reported a state yet.</span></p> : null}
          {completed ? (
            <p data-testid="solve-progress-completed-counts">
              The job reported {completed.result_row_count} result {completed.result_row_count === 1 ? "row" : "rows"} and {completed.diagnostic_count}{" "}
              {completed.diagnostic_count === 1 ? "diagnostic" : "diagnostics"}; the Issues list gathers a diagnostic repeated on one object into one item.
            </p>
          ) : null}
          <p className="solve-progress-basis" data-testid="solve-progress-basis">
            {basis.percentages_synthesized || basis.backend_percent_stream_available
              ? `Progress basis: ${basis.progress_basis}.`
              : `The solve job reports its state and events, not a percentage, a time estimate or a start time, so none is shown (progress basis: ${basis.progress_basis}).`}
          </p>
          {queueReason && (state === "queued" || state === "cancelling") ? (
            <p data-testid="solve-progress-queue-code">Queue reason code: <code>{resourceRefusalCode(queueReason)}</code></p>
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
          {state === "refused" && refusal ? (
            <p>
              Refusal code: <code data-testid="solve-progress-refusal-code">{resourceRefusalCode(refusal)}</code>
            </p>
          ) : null}
          <ol aria-label="Solve job events" className="solve-progress-events" data-testid="solve-progress-events">
            {events.map((event, index) => (
              <li data-event-id={event.event_id} data-state={event.state} key={`${event.event_id}-${index}`}>
                <strong>{solveJobStateLabel(event.state)}</strong> <span>{event.message}</span>
              </li>
            ))}
          </ol>
        </div>
      </details>
    </section>
  );
}

/** Keeps `--solve-status-line-height` on the document at the line's height while it is
 * shown, so the Key results panel, laid over the workspace, opens below the line. */
function useLineHeight(ref: RefObject<HTMLElement | null>, shown: boolean) {
  useEffect(() => {
    const root = document.documentElement;
    const element = ref.current;
    if (!shown || !element) {
      root.style.removeProperty("--solve-status-line-height");
      return;
    }
    const set = () => root.style.setProperty("--solve-status-line-height", `${element.getBoundingClientRect().height}px`);
    set();
    const observer = typeof ResizeObserver === "function" ? new ResizeObserver(set) : null;
    observer?.observe(element);
    return () => {
      observer?.disconnect();
      root.style.removeProperty("--solve-status-line-height");
    };
  }, [ref, shown]);
}

/**
 * The last solve's failure or refusal, as the Issues drawer lists it: the reason
 * the job gives, the way to retry and the exact code, each as the status line
 * and the Analyze page give them.
 */
export function SolveIssueItem({ solveJob }: { solveJob: SolveJobAuditState }) {
  const refusal = solveJob.resource.refusal;
  const refusalPoint = solveRefusalPoint(solveJob);
  const tooLarge = isResultTooLargeSolveJob(solveJob);
  return (
    <section className="panel issues-solve" aria-label="Last solve" data-testid="issues-solve-item">
      <div className="panel-title">
        <Flag size={16} aria-hidden="true" />
        Last solve
        <span className="issues-section-count" aria-label="1 item">1</span>
      </div>
      <article className="issue-row error">
        <strong>{solveJob.state === "failed" ? "The solve failed" : "The solve was refused"}</strong>
        {solveJob.state === "failed" ? <p>{describeSolveWorkerFailure(solveJob.error_message) ?? solveJob.error_message ?? "The solve failed."}</p> : null}
        {tooLarge ? <p>{solveJob.error_message}</p> : null}
        {solveJob.state === "refused" && refusal ? (
          <>
            <p>Refused by the native memory pool: {describeResourceRefusal(refusal)} No result was published.</p>
            {refusalPoint ? <p>{describeSolveRefusalRetry(refusal, refusalPoint)}</p> : null}
            <p>
              Refusal code: <code>{resourceRefusalCode(refusal)}</code>
            </p>
          </>
        ) : null}
        <small>Job {solveJob.job_id}</small>
      </article>
    </section>
  );
}

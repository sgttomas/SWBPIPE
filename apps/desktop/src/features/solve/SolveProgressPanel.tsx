import { ArrowRight, ChevronRight, Flag, ListChecks, Square, X } from "lucide-react";
import { useEffect, useId, useReducer, useRef, useState, type RefObject } from "react";
import type { ResourceRefusal, SolveJobAuditState, SolveJobEvent } from "../../types";
import { describeResourceRefusalAfterColon, describeSolveWorkerFailure, formatBytes, poolRefusalReading, resourceRefusalCode, solveWorkerFailureCode } from "../../services/memoryPool";
import { registeredStatusLabel, statusDisplayWithToken } from "../workspace/statusLabels";
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
  /** Counts runs, so the line's Details closes at the start of each run. */
  serial: number;
  /** Whether a result was held at the last render while no run was active. */
  idleResultHeld: boolean;
  /** Whether a result was held when the current (or last) run started. */
  resultHeldAtStart: boolean;
  /** The last queue reason the current (or last) run's job reported, kept after it runs. */
  queueReason: ResourceRefusal | null;
  /** When the line saw the current (or last) run start, by the desktop's clock; null before any run. */
  startedAt: number | null;
  /** When that run's job reached its end (a final state, or the run ended); null while it runs. */
  endedAt: number | null;
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
function runView(solveJob: SolveJobAuditState, running: boolean, resultHeld: boolean, tracker: RunTracker): RunView {
  if (running && !tracker.wasRunning) {
    tracker.startedFrom = tracker.idle;
    tracker.resultHeldAtStart = tracker.idleResultHeld;
    tracker.queueReason = null;
    tracker.runJobId = null;
    tracker.closed = false;
    tracker.serial += 1;
    tracker.startedAt = Date.now();
    tracker.endedAt = null;
  }
  if (!running && tracker.wasRunning && tracker.startedAt !== null && tracker.endedAt === null) tracker.endedAt = Date.now();
  if (tracker.startedFrom !== null && solveJob !== tracker.startedFrom && (running || tracker.wasRunning)) {
    tracker.runJobId = solveJob.job_id;
  }
  if (solveJob.state === "not_started") tracker.runJobId = null;
  if ((running || tracker.wasRunning) && tracker.runJobId !== null && solveJob.resource.queue_reason) tracker.queueReason = solveJob.resource.queue_reason;
  if (!running) {
    tracker.idle = solveJob;
    tracker.idleResultHeld = resultHeld;
  }
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

/** The registered statuses a completed job reports, the mechanics (Solver) status first and the
 * others in the order reported, unshortened. */
function reportedStatuses(statuses: readonly string[]): string[] {
  const registered = statuses.filter((status) => registeredStatusLabel(status) !== null);
  const solver = registered.filter((status) => registeredStatusLabel(status)?.domain === "Solver");
  return [...solver, ...registered.filter((status) => !solver.includes(status))];
}

/** The closed line's plain words for a Solver status other than solved. A registered label
 * stands only in Details, with its authority domain and token. */
const NOT_SOLVED_WORDS: Readonly<Record<string, string>> = { MODEL_INCOMPLETE: "Not solved: the model is incomplete" };

/**
 * The completed line's outcome, in plain words, from the mechanics (Solver) status the job
 * reports: "Solved" for MECHANICS_SOLVED; "Not solved: the model is incomplete" for
 * MODEL_INCOMPLETE ("Not solved" for any other Solver status), with the way on to the findings;
 * the job's state word where it reports no Solver status, claiming nothing.
 */
export function completedWord(statuses: readonly string[]): { word: string; solved: boolean; unsolved: boolean } {
  const solver = statuses.map(registeredStatusLabel).find((row) => row?.domain === "Solver" && row.kind === "status") ?? null;
  if (solver === null) return { word: STATE_LABELS.completed, solved: false, unsolved: false };
  return solver.token === "MECHANICS_SOLVED"
    ? { word: "Solved", solved: true, unsolved: false }
    : { word: NOT_SOLVED_WORDS[solver.token] ?? "Not solved", solved: false, unsolved: true };
}

/** The states the job reported, in order, each once where it repeats. */
function reportedStates(events: readonly SolveJobEvent[]): string[] {
  const states: string[] = [];
  for (const event of events) {
    const label = solveJobStateLabel(event.state);
    if (states[states.length - 1] !== label) states.push(label);
  }
  return states;
}

/** A cancellation request, in plain words, by the state the job reached after it. */
function cancellationWords(state: SolveJobEvent["state"] | null): string {
  if (state === "cancelling") return "Stop was requested; the line waits for the job's final state.";
  if (state === "cancelled") return "Stopped at your request.";
  if (state === "completed") return "Stop was requested, but the job completed first.";
  return "Stop was requested.";
}

const plural = (count: number, one: string, many: string) => `${count} ${count === 1 ? one : many}`;

/** A measured interval in minutes and seconds ("0:42", "12:05"), with hours when needed ("1:02:05"). */
export function formatElapsed(milliseconds: number): string {
  const total = Math.max(0, Math.floor(milliseconds / 1000));
  const hours = Math.floor(total / 3600);
  const minutes = Math.floor((total % 3600) / 60);
  const seconds = String(total % 60).padStart(2, "0");
  return hours > 0 ? `${hours}:${String(minutes).padStart(2, "0")}:${seconds}` : `${minutes}:${seconds}`;
}

/**
 * The solve status line (DEL-07-07 REQ-014): one line in the band under the
 * toolbar, from the moment a solve starts until the user dismisses it or the
 * model changes. It says the state the job reports and nothing the job did not
 * report (no model facts: DEL-07-07 REQ-003), with Stop while the job can be
 * stopped (the one Stop control: the toolbar keeps Run, withheld while a solve
 * runs). When the job completes the line reads one sentence: "Solved." and the
 * result rows the job reported, with "Open results" unless the Results stage is
 * the one shown, and, only where something is left for the person to do and its
 * count can be read honestly, that thing as a control with its count ("Rule
 * inputs missing: N", opening the Issues drawer at those findings). A mechanics
 * status other than solved is the line's first word in plain words, then
 * "Nothing was solved." and no rows, with what to do as a control with its count
 * ("Blocking findings: N", opening the Issues drawer at the first of them), or
 * "Show in Issues" where no count can be read; never "Completed". Every
 * status the job reported stays in Details, with its token. When it fails or is
 * refused, the reason the job gives, the way to retry and the exact code, with
 * "Show in Issues".
 * Everything else the job reports is behind the line's one "Details" control,
 * complete, facts first; Details closes at the start of each run, its control
 * stays where it is at the end of the line, and its body opens beneath the line.
 * Nothing of an earlier job is shown before the new job's first report. While
 * the run is active the line shows the time since it started, measured by the
 * desktop and labelled as elapsed, counting once a second outside the live region;
 * the completed, stopped and failed lines say how long it took, and the figure
 * stands still. It shows no percentage, progress bar or estimate of the time
 * remaining or the time the job will take. While the job reports queued or running
 * a small moving mark beside the state word shows the display is alive (still under
 * reduced motion; the figure keeps counting, since it is information). It is part of the page's layout, never laid over the model,
 * the Key results panel or the Insert controls.
 */
export function SolveProgressPanel({
  solveJob,
  running,
  onCancel,
  onOpenResults,
  onShowIssue,
  resultsShown = false,
  resultHeld = false,
  ruleInputsMissing = null,
  onShowRuleInputs,
  blockingFindings = null,
  onShowBlocking
}: {
  solveJob: SolveJobAuditState;
  running: boolean;
  onCancel: () => void;
  /** Opens the Results stage. */
  onOpenResults?: () => void;
  /** Opens the Issues drawer at the solve's failure or refusal. */
  onShowIssue?: () => void;
  /** Whether the Results stage is the stage shown: "Open results" is then not offered. */
  resultsShown?: boolean;
  /** Whether the session holds a result; read while no run is active, so a stopped run can
   * say that the result held before it was cleared when it started. */
  resultHeld?: boolean;
  /** How many findings say a rule input is missing (the Missing data entries that block the rule
   * check); read only when the completed job reports RULE_INPUTS_INCOMPLETE. */
  ruleInputsMissing?: number | null;
  /** Opens the Issues drawer at those findings. */
  onShowRuleInputs?: () => void;
  /** How many of the Issues drawer's items holding a solve report are blocking or errors; read
   * when the completed job reports a mechanics status other than solved. */
  blockingFindings?: number | null;
  /** Opens the Issues drawer at the first of them. */
  onShowBlocking?: () => void;
}) {
  const stateRef = useRef<HTMLParagraphElement>(null);
  const lineRef = useRef<HTMLElement>(null);
  const detailsId = useId();
  const [, rerender] = useReducer((count: number) => count + 1, 0);
  const tracker = useRef<RunTracker>({ wasRunning: false, idle: solveJob, startedFrom: null, runJobId: null, closed: false, serial: 0, idleResultHeld: resultHeld, resultHeldAtStart: false, queueReason: null, startedAt: null, endedAt: null });
  const [openedRun, setOpenedRun] = useState<number | null>(null);
  const view = runView(solveJob, running, resultHeld, tracker.current);
  const shown = view.kind !== "hidden";
  // The job's end, by the desktop's clock, when its record first shows a final state.
  if (view.kind === "job" && !ACTIVE_STATES.has(view.job.state) && tracker.current.startedAt !== null && tracker.current.endedAt === null) tracker.current.endedAt = Date.now();
  const ticking = shown && tracker.current.startedAt !== null && tracker.current.endedAt === null;
  useLineHeight(lineRef, shown);
  // The elapsed figure counts once a second while the run is active, and stands still at its end.
  useEffect(() => {
    if (!ticking) return;
    const timer = setInterval(rerender, 1000);
    return () => clearInterval(timer);
  }, [ticking]);

  if (view.kind === "hidden") return null;

  const serial = tracker.current.serial;
  const detailsOpen = openedRun === serial;
  const job = view.kind === "job" ? view.job : null;
  const events = job ? job.events.filter((event) => !view.earlier.includes(event)) : [];
  const latest = events[events.length - 1] ?? null;
  const state = job?.state ?? null;
  const active = state === null || ACTIVE_STATES.has(state);
  const alive = state === "queued" || state === "running";
  // Before the job reports, Stop stops the run before its start (the session's handler).
  const canCancel = running && (state === null || state === "queued" || state === "running");
  const refusal = job?.resource.refusal ?? null;
  const refusalPoint = job ? solveRefusalPoint(job) : null;
  const queueReason = job?.resource.queue_reason ?? null;
  const workerFailure = job && state === "failed" ? describeSolveWorkerFailure(job.error_message) : null;
  const workerFailureCode = job && state === "failed" ? solveWorkerFailureCode(job.error_message) : null;
  const completed = state === "completed" ? [...events].reverse().find((event) => event.state === "completed") ?? null : null;
  const tooLarge = job !== null && isResultTooLargeSolveJob(job);
  const resource = job?.resource ?? null;
  const statuses = completed ? reportedStatuses(completed.analysis_status) : [];
  const outcome = completed ? completedWord(completed.analysis_status) : null;
  // Something left for the person to do, as a control with its count; only with an honest count.
  const ruleInputs = completed && completed.analysis_status.some((status) => registeredStatusLabel(status)?.token === "RULE_INPUTS_INCOMPLETE")
    && ruleInputsMissing !== null && ruleInputsMissing > 0 && onShowRuleInputs ? ruleInputsMissing : null;
  const label = state === null ? "Starting" : outcome ? outcome.word : state === "cancelled" ? "Stopped" : solveJobStateLabel(state);
  // The time since the run started, measured by the desktop: counting while active, still after.
  const { startedAt, endedAt } = tracker.current;
  const elapsed = startedAt === null ? null : formatElapsed((endedAt ?? Date.now()) - startedAt);
  const took = elapsed === null || endedAt === null ? null
    : outcome?.solved || (completed && !outcome?.unsolved) ? ` in ${elapsed}`
      : outcome?.unsolved ? ` (${elapsed} elapsed)`
        : completed || state === "cancelled" || state === "failed" ? ` after ${elapsed}` : null;
  const tone = state === "failed" || state === "refused" ? "error" : state === "completed" ? "done" : state === "cancelled" ? "stopped" : "active";
  // A mechanics status other than solved: what to do, as a control with its count where one can be read.
  const blocking = outcome?.unsolved && blockingFindings !== null && blockingFindings > 0 && onShowBlocking ? blockingFindings : null;
  const issue = state === "failed" || state === "refused" || tooLarge || (outcome?.unsolved === true && blocking === null);
  const poolRefusal = state === "refused" && refusal !== null;
  // A run clears the result held before it when its job starts (the session's solve).
  const earlierResultCleared = state === "cancelled" && tracker.current.resultHeldAtStart;

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
        <div className="solve-status-words">
        <p
          aria-live="polite"
          className="solve-status-text"
          data-testid="solve-progress-state"
          ref={stateRef}
          role="status"
          tabIndex={-1}
        >
          <strong>{label}</strong>
          {took ? <span data-testid="solve-progress-took">{took}</span> : null}
          {alive ? <span aria-hidden="true" className="solve-status-alive" data-testid="solve-progress-alive" /> : null}
          {poolRefusal ? null : state === "cancelled" || completed || (queueReason && (state === "queued" || state === "cancelling")) || tooLarge || state === "failed" ? "." : null}
          {state === "cancelled" ? <span> No result was published.</span> : null}
          {earlierResultCleared ? <span data-testid="solve-progress-earlier-cleared"> The earlier result was cleared when this run started.</span> : null}
          {completed && !outcome?.unsolved ? (
            <span>
              {" "}<span data-testid="solve-progress-completed">{plural(completed.result_row_count, "result row", "result rows")}.</span>
            </span>
          ) : null}
          {queueReason && (state === "queued" || state === "cancelling") ? (
            <span data-testid="solve-progress-queue-reason"> Waiting in the queue: {describeResourceRefusalAfterColon(queueReason)}</span>
          ) : null}
          {poolRefusal ? (
            <span data-testid="solve-progress-refusal">{poolRefusalReading(refusal)} No result was published.</span>
          ) : null}
          {poolRefusal && refusalPoint ? <span data-testid="solve-progress-retry"> {describeSolveRefusalRetry(refusal, refusalPoint)}</span> : null}
          {poolRefusal ? (
            <span> Code: <code data-testid="solve-progress-refusal-code">{resourceRefusalCode(refusal)}</code></span>
          ) : null}
          {tooLarge ? <span data-testid="solve-progress-result-too-large"> {job?.error_message}</span> : null}
          {state === "failed" ? (
            <span data-testid="solve-progress-failure"> {workerFailure ?? job?.error_message ?? "The solve failed."}</span>
          ) : null}
          {workerFailureCode ? <span> Code: <code data-testid="solve-progress-failure-code">{workerFailureCode}</code></span> : null}
        </p>
        {/* The counting figure stands outside the live region, so a screen reader is not told every
            second; the state word in the region still is. */}
        {active && elapsed !== null ? (
          <span className="solve-status-elapsed" data-testid="solve-progress-elapsed">
            · {elapsed} elapsed
          </span>
        ) : null}
        </div>
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
          {/* Nothing to open where nothing was solved. */}
          {state === "completed" && !outcome?.unsolved && onOpenResults && !resultsShown ? (
            <button className="primary-action" data-testid="solve-progress-open-results" onClick={onOpenResults} type="button">
              Open results
              <ArrowRight size={14} aria-hidden="true" />
            </button>
          ) : null}
          {blocking !== null ? (
            <button className="secondary-action" data-testid="solve-progress-blocking" onClick={onShowBlocking} type="button">
              <Flag size={14} aria-hidden="true" />
              Blocking findings: {blocking}
            </button>
          ) : null}
          {ruleInputs !== null ? (
            <button className="secondary-action" data-testid="solve-progress-rule-inputs" onClick={onShowRuleInputs} type="button">
              <ListChecks size={14} aria-hidden="true" />
              Rule inputs missing: {ruleInputs}
            </button>
          ) : null}
          {issue && onShowIssue ? (
            <button className="secondary-action" data-testid="solve-progress-show-issue" onClick={onShowIssue} type="button">
              <Flag size={14} aria-hidden="true" />
              Show in Issues
            </button>
          ) : null}
          <button
            aria-controls={detailsId}
            aria-expanded={detailsOpen}
            className="solve-status-details-toggle"
            data-testid="solve-progress-details-toggle"
            onClick={() => setOpenedRun(detailsOpen ? null : serial)}
            type="button"
          >
            <ChevronRight className="solve-status-details-chevron" size={14} aria-hidden="true" />
            Details
          </button>
          {active ? null : (
            <button aria-label="Dismiss" className="solve-status-dismiss" data-testid="solve-progress-close" onClick={close} title="Dismiss" type="button">
              <X size={14} aria-hidden="true" />
            </button>
          )}
        </div>
      </div>
      {/* Opens beneath the line at full width; closed at the start of each run, and it keeps
          the person's choice between the reports of one run. Facts first. */}
      <div className="solve-status-details" data-open={detailsOpen} data-testid="solve-progress-details" hidden={!detailsOpen} id={detailsId}>
        {state === null ? <p>The solve was requested; its job has not reported a state yet.</p> : null}
        {events.length > 0 ? <p data-testid="solve-progress-states">States reported: {reportedStates(events).join(", ")}</p> : null}
        {completed ? (
          <p data-testid="solve-progress-statuses">
            Statuses reported: {statuses.length > 0 ? statuses.map(statusDisplayWithToken).join("; ") : "none"}
          </p>
        ) : null}
        {completed ? (
          <p data-testid="solve-progress-completed-counts">
            The job reported {plural(completed.result_row_count, "result row", "result rows")} and{" "}
            {plural(completed.diagnostic_count, "diagnostic report", "diagnostic reports")}.
          </p>
        ) : null}
        {resource?.route_pricing === "ordinary_worker_capped" ? (
          <p data-testid="solve-progress-worker">
            Worker memory: {active ? `grant ${resource.worker_grant_bytes === null ? "not yet held" : formatBytes(resource.worker_grant_bytes)}; ` : ""}peak grant{" "}
            {resource.worker_peak_grant_bytes === null ? "none" : formatBytes(resource.worker_peak_grant_bytes)}; peak footprint{" "}
            {resource.worker_peak_footprint_bytes === null ? "not measured" : formatBytes(resource.worker_peak_footprint_bytes)}; budget{" "}
            {formatBytes(resource.invocation_budget_bytes)}
          </p>
        ) : null}
        {job?.cancellation_requested ? (
          <p data-testid="solve-progress-cancellation">
            {cancellationWords(state)} Code: <code>{job.cancellation_status}</code>
          </p>
        ) : null}
        {job && (queueReason ?? tracker.current.queueReason) ? (
          <p data-testid="solve-progress-queue-code">Queue reason code: <code>{resourceRefusalCode((queueReason ?? tracker.current.queueReason)!)}</code></p>
        ) : null}
        {job ? <p>Job: <code data-testid="solve-progress-job">{job.job_id}</code></p> : null}
        {events.length > 0 ? (
          <details className="solve-status-steps" data-testid="solve-progress-steps">
            <summary data-testid="solve-progress-steps-toggle">What the job reported at each step</summary>
            <ol aria-label="Solve job events" className="solve-progress-events" data-testid="solve-progress-events">
              {events.map((event, index) => (
                <li data-event-id={event.event_id} data-state={event.state} key={`${event.event_id}-${index}`}>
                  <strong>{solveJobStateLabel(event.state)}</strong>{" "}
                  {event === latest ? (
                    <span data-testid="solve-progress-stage">{event.message} <small className="muted">(the stage)</small></span>
                  ) : <span>{event.message}</span>}
                </li>
              ))}
            </ol>
          </details>
        ) : null}
      </div>
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
        {solveJob.state === "failed" && solveWorkerFailureCode(solveJob.error_message) ? (
          <p>Code: <code data-testid="issues-solve-failure-code">{solveWorkerFailureCode(solveJob.error_message)}</code></p>
        ) : null}
        {tooLarge ? <p>{solveJob.error_message}</p> : null}
        {solveJob.state === "refused" && refusal ? (
          <>
            <p>Refused{poolRefusalReading(refusal)} No result was published.</p>
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

import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { BackendSolveJobStatus, SolveJobStartReceipt } from "../../services/previewService";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel, SolveJobAuditState, SolveResourceGrant } from "../../types";
import {
  cancelledSolveJob,
  completeSolveJob,
  failSolveJob,
  initialSolveJob,
  modelChangedSolveJob,
  progressSolveJob,
  refusedSolveJob,
  requestSolveCancellation,
  resultTooLargeSolveJob,
  startSolveJob
} from "../workspace/solveJobAudit";
import { formatElapsed, SolveIssueItem, SolveProgressPanel } from "./SolveProgressPanel";
import { groupIssues, solveBlockingItems } from "../diagnostics/issueItems";
import missingDataStartModel from "../../../../../fixtures/embedded_agent/use_cases/missing-data/start_model.json";

// DEL-07-07 REQ-014: the solve status line shows the solve-job record the
// session already keeps, from invented native job reports. What the line keeps
// behind its closed Details is read only after Details is opened.
const GiB = 1024 ** 3;
const MiB = 1024 ** 2;
const model = {
  project: { id: "project:invented-progress-panel" },
  diagnostics: [],
  analysis_status: { mechanics: "MODEL_INCOMPLETE", rule_check: "RULE_INPUTS_INCOMPLETE" }
} as unknown as PreviewModel;
const grant = (worker: number | null, peak: number | null, footprint: number | null = null): SolveResourceGrant => ({
  invocation_budget_bytes: 12 * GiB, pool_bytes: 48 * GiB, route_pricing: "ordinary_worker_capped", ingress_reserved_bytes: 3 * MiB,
  worker_grant_bytes: worker, worker_peak_grant_bytes: peak, worker_peak_footprint_bytes: footprint
});
const receipt: SolveJobStartReceipt = {
  mode: "backend_job", job_id: "backend-solve-job-31", backend_cancellation_token: "token-31", state: "queued",
  cancellation_scope: "worker_process_terminated", resource: grant(null, null)
};
function status(overrides: Partial<BackendSolveJobStatus>): BackendSolveJobStatus {
  return {
    job_id: "backend-solve-job-31", state: "queued", queue_reason: null, cancellation_requested: false,
    cancellation_status: "not_requested", cancellation_scope: "worker_process_terminated", result_available: false,
    result_bytes: null, error_message: null, resource_refusal: null, resource: grant(null, null), ...overrides
  };
}

/** Whether the line's Details is open: its control says expanded and its body is shown. */
function detailsOpen(): boolean {
  const expanded = screen.getByTestId("solve-progress-details-toggle").getAttribute("aria-expanded") === "true";
  const hidden = (screen.getByTestId("solve-progress-details") as HTMLElement).hidden;
  if (expanded === hidden) throw new Error("the Details control and its body disagree");
  return expanded;
}

/** Opens the line's Details by a press on its control, as a person does, and fails loudly if
 * the press did not open it; nothing inside is read while it is closed. Already open (a
 * person's choice kept between the reports of one run), it is left as it is. */
function openDetails() {
  if (!detailsOpen()) fireEvent.click(screen.getByTestId("solve-progress-details-toggle"));
  if (!detailsOpen()) throw new Error("the line's Details did not open on a press of its control");
  return screen.getByTestId("solve-progress-details");
}

/** Opens Details, then its "What the job reported at each step" by a press on its summary. */
function openSteps() {
  openDetails();
  const steps = screen.getByTestId("solve-progress-steps") as HTMLDetailsElement;
  if (!steps.open) fireEvent.click(screen.getByTestId("solve-progress-steps-toggle"));
  if (!steps.open) throw new Error("the steps did not open on a press of their summary");
  return steps;
}

/** No percentage, progress figure or estimate of the time remaining or to come. */
const NO_ESTIMATE = /\d\s*%|\bETA\b|remaining|estimat|expected to take|will take/i;

function panel(job: SolveJobAuditState, running: boolean, onCancel = () => {}) {
  return <SolveProgressPanel solveJob={job} running={running} onCancel={onCancel} />;
}

/** Mounts the panel as the session does: idle on `before`, then a run starts
 * and its first record `job` replaces it. */
function renderRun(job: SolveJobAuditState, onCancel = () => {}, before: SolveJobAuditState = initialSolveJob()) {
  const view = render(panel(before, false, onCancel));
  view.rerender(panel(before, true, onCancel));
  view.rerender(panel(job, true, onCancel));
  return view;
}

describe("solve progress panel", () => {
  it("stays hidden until a run starts, so a restored or untouched record opens nothing", () => {
    const view = render(panel(initialSolveJob(), false));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
    // A completed record the session did not run (for example a restored run) opens nothing.
    view.rerender(panel({ ...initialSolveJob(), state: "completed" }, false));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
  });

  it("shows the live state, its latest event as the stage, every event in order, and no percentage or estimate", () => {
    const queued = startSolveJob(model, receipt);
    const view = renderRun(queued);
    const region = screen.getByRole("region", { name: "Solve progress" });
    expect(region).toHaveAttribute("data-state", "queued");
    expect(screen.getByTestId("solve-progress-state")).toHaveAttribute("role", "status");
    expect(screen.getByTestId("solve-progress-state")).toHaveAttribute("aria-live", "polite");
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Queued/);
    expect(screen.getByTestId("solve-progress-alive")).toHaveAttribute("aria-hidden", "true");
    openSteps();
    expect(screen.getByTestId("solve-progress-stage")).toHaveTextContent(queued.events[0].message);
    expect(screen.getByTestId("solve-progress-job")).toHaveTextContent("backend-solve-job-31");
    expect(screen.getByTestId("solve-progress-states")).toHaveTextContent(/^States reported: Queued$/);

    const waiting = progressSolveJob(queued, status({ queue_reason: { kind: "pool_exhausted", requested_bytes: 12 * GiB, reserved_bytes: 40 * GiB, pool_bytes: 48 * GiB } }));
    view.rerender(panel(waiting, true));
    expect(screen.getByTestId("solve-progress-queue-reason")).toHaveTextContent(
      "Waiting in the queue: The memory pool is fully reserved: this request needs 12 GiB, and 40 GiB of the 48 GiB pool is already reserved."
    );
    expect(screen.getByTestId("solve-progress-details")).not.toContainElement(screen.getByTestId("solve-progress-queue-reason"));
    openSteps();
    expect(screen.getByTestId("solve-progress-stage")).toHaveTextContent(waiting.events[waiting.events.length - 1].message);
    expect(screen.getByTestId("solve-progress-queue-code")).toHaveTextContent(
      "Queue reason code: pool_exhausted (requested_bytes=12884901888, reserved_bytes=42949672960, pool_bytes=51539607552)"
    );

    const running = progressSolveJob(progressSolveJob(waiting, status({ state: "running" })), status({ state: "running", resource: grant(64 * MiB, 96 * MiB, 70 * MiB) }));
    view.rerender(panel(running, true));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Running/);
    expect(screen.queryByTestId("solve-progress-queue-reason")).toBeNull();
    openSteps();
    expect(screen.getByTestId("solve-progress-stage")).toHaveTextContent(running.events[running.events.length - 1].message);
    expect(screen.getByTestId("solve-progress-states")).toHaveTextContent(/^States reported: Queued, Running$/);
    expect(screen.getByTestId("solve-progress-worker")).toHaveTextContent("Worker memory: grant 64 MiB; peak grant 96 MiB; peak footprint 70 MiB; budget 12 GiB");
    const events = within(screen.getByRole("list", { name: "Solve job events" })).getAllByRole("listitem");
    expect(events.map((item) => item.getAttribute("data-event-id"))).toEqual(["solve-preview-queued", "solve-preview-queued-waiting", "solve-preview-running"]);
    expect(events.map((item) => item.getAttribute("data-state"))).toEqual(["queued", "queued", "running"]);
    // Nothing reads as a fraction or an estimate; the only time is the measured elapsed figure.
    expect(screen.queryByTestId("solve-progress-basis")).toBeNull();
    expect(screen.getByTestId("solve-progress-panel").textContent).not.toMatch(NO_ESTIMATE);
    expect(screen.queryByRole("progressbar")).toBeNull();

    const result = { results: [{}], diagnostics: [] } as unknown as MechanicsResult;
    const run = { analysis_run: { run_id: "analysis-run:invented-sequence", analysis_status: [] } } as unknown as AnalysisRunEnvelope;
    const done = completeSolveJob(running, result, run);
    view.rerender(panel(done, false));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Completed/);
    expect(screen.queryByTestId("solve-progress-alive")).toBeNull();
    openSteps();
    expect(screen.getByTestId("solve-progress-stage")).toHaveTextContent(done.events[done.events.length - 1].message);
    expect(screen.getByTestId("solve-progress-states")).toHaveTextContent(/^States reported: Queued, Running, Completed$/);
    expect(screen.getByTestId("solve-progress-panel").textContent).not.toMatch(NO_ESTIMATE);
  });

  it("offers Cancel as a focusable button; once requested it stays in place, aria-disabled, and keyboard focus moves to the state", () => {
    const onCancel = vi.fn();
    const running = progressSolveJob(startSolveJob(model, receipt), status({ state: "running" }));
    const view = renderRun(running, onCancel);
    const cancel = screen.getByRole("button", { name: "Stop" });
    expect(cancel.tagName).toBe("BUTTON");
    expect(cancel).toHaveAttribute("type", "button");
    expect(cancel).not.toHaveAttribute("tabindex");
    expect(cancel).toHaveAttribute("aria-disabled", "false");
    cancel.focus();
    expect(document.activeElement).toBe(cancel);
    // A keyboard press of a focused native button delivers a click.
    fireEvent.click(cancel);
    expect(onCancel).toHaveBeenCalledTimes(1);
    expect(document.activeElement).toBe(screen.getByTestId("solve-progress-state"));

    const cancelling = requestSolveCancellation(running);
    view.rerender(panel(cancelling, true, onCancel));
    const pending = screen.getByTestId("solve-progress-cancel");
    expect(pending).toHaveTextContent("Cancelling");
    expect(pending).toHaveAttribute("aria-disabled", "true");
    fireEvent.click(pending);
    expect(onCancel).toHaveBeenCalledTimes(1);
    openDetails();
    expect(screen.getByTestId("solve-progress-cancellation")).toHaveTextContent("Stop was requested; the line waits for the job's final state. Code: request_sent_to_backend_job_awaiting_receipt");

    const cancelled = cancelledSolveJob(cancelling, status({ state: "cancelled", cancellation_requested: true, cancellation_status: "worker_terminated" }));
    view.rerender(panel(cancelled, false, onCancel));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Stopped after \d+:\d\d/);
    // Only what the job reports: a cancelled job may have finished and had its result discarded.
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Stopped after \d+:\d\d\. No result was published\.$/);
    expect(screen.queryByTestId("solve-progress-cancel")).toBeNull();
    openDetails();
    expect(screen.getByTestId("solve-progress-cancellation")).toHaveTextContent("Stopped at your request. Code: worker_terminated");
  });

  it("shows a memory-pool refusal with its code, exact bytes and retry path, and no result", () => {
    const started = startSolveJob(model, receipt);
    const view = renderRun(started);
    const refused = refusedSolveJob(started, { kind: "headroom_insufficient", required_bytes: 68165632, available_bytes: 52641792 });
    view.rerender(panel(refused, false));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Refused/);
    expect(screen.getByTestId("solve-progress-refusal")).toHaveTextContent(
      "by the native memory pool: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 65.01 MiB, 50.2 MiB available. No result was published."
    );
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Refused by the native memory pool: Not enough available memory/);
    expect(screen.getByTestId("solve-progress-retry")).toHaveTextContent(
      "Refused at the result hand-off after the solve finished; the finished result was dropped. Available memory was re-sampled for up to 2 s before this refusal stood. You can run the solve again when more memory is available."
    );
    // The reading, the way to retry and the exact code are on the line, in its polite live region.
    const state = screen.getByTestId("solve-progress-state");
    for (const id of ["solve-progress-refusal", "solve-progress-retry", "solve-progress-refusal-code"]) {
      expect(screen.getByTestId("solve-progress-details")).not.toContainElement(screen.getByTestId(id));
      expect(state).toContainElement(screen.getByTestId(id));
    }
    expect(screen.getByTestId("solve-progress-refusal-code")).toHaveTextContent("headroom_insufficient (required_bytes=68165632, available_bytes=52641792)");
    expect(screen.queryByTestId("solve-progress-completed")).toBeNull();
  });

  it("shows a worker failure by its existing reading", () => {
    const started = startSolveJob(model, receipt);
    const view = renderRun(started);
    view.rerender(panel(failSolveJob(started, new Error("SOLVE_WORKER_EXITED: invented exit")), false));
    expect(screen.getByTestId("solve-progress-failure")).toHaveTextContent(
      "The solve's worker process ended without returning a result. The solve failed; this is not a memory-pool refusal."
    );
    // The reading drops the code from the message, so the line states it beside the reading.
    expect(screen.getByTestId("solve-progress-state")).toContainElement(screen.getByTestId("solve-progress-failure-code"));
    expect(screen.getByTestId("solve-progress-failure-code")).toHaveTextContent(/^SOLVE_WORKER_EXITED$/);
  });

  it("shows completion with the rows and diagnostics the completed event carries; Close dismisses it until the next run", () => {
    const running = progressSolveJob(startSolveJob(model, receipt), status({ state: "running" }));
    const view = renderRun(running);
    const result = { results: [{}, {}, {}], diagnostics: [{}] } as unknown as MechanicsResult;
    const run = { analysis_run: { run_id: "analysis-run:invented-31", analysis_status: [] } } as unknown as AnalysisRunEnvelope;
    const completed = completeSolveJob(running, result, run);
    view.rerender(panel(completed, false));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Completed/);
    expect(screen.getByTestId("solve-progress-completed")).toHaveTextContent("3 result rows.");
    // The job's own count of findings is behind Details, not beside the Issues count: it is
    // read only once Details has been opened by a press on its summary.
    expect(detailsOpen()).toBe(false);
    fireEvent.click(screen.getByTestId("solve-progress-details-toggle"));
    expect(detailsOpen()).toBe(true);
    expect(screen.getByTestId("solve-progress-completed-counts")).toHaveTextContent(/^The job reported 3 result rows and 1 diagnostic report\.$/);
    fireEvent.click(screen.getByRole("button", { name: "Dismiss" }));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
    // The next run opens it again.
    view.rerender(panel(completed, true));
    view.rerender(panel(startSolveJob(model, receipt), true));
    expect(screen.getByTestId("solve-progress-panel")).toHaveAttribute("data-state", "queued");
  });

  it("closes when the record no longer describes a job (the model changed)", () => {
    const running = progressSolveJob(startSolveJob(model, receipt), status({ state: "running" }));
    const view = renderRun(running);
    view.rerender(panel(modelChangedSolveJob({ operation_id: "op:invented-edit" } as never), false));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
    // A later record the session did not run stays hidden.
    view.rerender(panel({ ...initialSolveJob(), state: "completed" }, false));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
  });

  it.each([
    ["failed", () => failSolveJob(startSolveJob(model, receipt), new Error("invented first failure"))],
    ["completed", () => completeSolveJob(progressSolveJob(startSolveJob(model, receipt), status({ state: "running" })),
      { results: [{}], diagnostics: [] } as unknown as MechanicsResult,
      { analysis_run: { run_id: "analysis-run:invented-first", analysis_status: [] } } as unknown as AnalysisRunEnvelope)]
  ])("a rerun after a %s job never shows the previous job: Starting with Cancel until the new job reports (Codex r1 of #1665)", (_kind, first) => {
    const onCancel = vi.fn();
    const previous = first();
    // The first run ends on `previous`, and the user closes the panel.
    const view = renderRun(previous, onCancel);
    view.rerender(panel(previous, false, onCancel));
    fireEvent.click(screen.getByRole("button", { name: "Dismiss" }));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
    // A new run starts; the session still holds the previous record until the start receipt.
    view.rerender(panel(previous, true, onCancel));
    const region = screen.getByRole("region", { name: "Solve progress" });
    expect(region).toHaveAttribute("data-state", "starting");
    expect(screen.queryByTestId("solve-progress-job")).toBeNull();
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent("Starting");
    expect(screen.queryByTestId("solve-progress-failure")).toBeNull();
    expect(screen.queryByTestId("solve-progress-completed")).toBeNull();
    // Before the new job's first report nothing of the earlier job is anywhere on the line or in Details.
    openDetails();
    expect(screen.queryByTestId("solve-progress-events")).toBeNull();
    expect(screen.queryByTestId("solve-progress-states")).toBeNull();
    expect(screen.queryByTestId("solve-progress-worker")).toBeNull();
    expect(screen.queryByTestId("solve-progress-cancellation")).toBeNull();
    for (const event of previous.events) expect(screen.getByTestId("solve-progress-panel").textContent).not.toContain(event.message);
    expect(screen.getByTestId("solve-progress-panel").textContent).not.toContain(previous.job_id);
    fireEvent.click(screen.getByRole("button", { name: "Stop" }));
    expect(onCancel).toHaveBeenCalledTimes(1);
    // A cancellation before the start receipt records a new state over the previous events; only the new event is this run's.
    const pending = { ...requestSolveCancellation(startSolveJob(model, receipt)), job_id: "job:preview-linear-static:pending-start-cancel",
      events: [...previous.events, { event_id: "solve-preview-cancel-requested-before-backend-start", state: "cancelling" as const, message: "invented pending cancel", result_available: false, diagnostic_count: 0, result_row_count: 0, analysis_status: [] }] };
    view.rerender(panel(pending, true, onCancel));
    openSteps();
    expect(within(screen.getByRole("list", { name: "Solve job events" })).getAllByRole("listitem").map((item) => item.getAttribute("data-event-id")))
      .toEqual(["solve-preview-cancel-requested-before-backend-start"]);
    // The new job's own record replaces it.
    view.rerender(panel({ ...startSolveJob(model, { ...receipt, job_id: "backend-solve-job-32" }) }, true, onCancel));
    openDetails();
    expect(screen.getByTestId("solve-progress-job")).toHaveTextContent("backend-solve-job-32");
    expect(screen.getByTestId("solve-progress-panel")).toHaveAttribute("data-state", "queued");
  });

  it("hides a later record of another job that no run produced (a restored run after the panel showed a run)", () => {
    const running = progressSolveJob(startSolveJob(model, receipt), status({ state: "running" }));
    const view = renderRun(running);
    const failed = failSolveJob(running, new Error("invented failure"));
    view.rerender(panel(failed, false));
    expect(screen.getByTestId("solve-progress-panel")).toHaveAttribute("data-state", "failed");
    view.rerender(panel({ ...initialSolveJob(), job_id: "job:preview-linear-static:restored:invented", state: "completed" }, false));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
  });

  it.each([
    ["a first run (no job yet)", () => initialSolveJob()],
    ["a run after a model reset", () => modelChangedSolveJob({ operation_id: "op:invented-reset" } as never)]
  ])("shows Starting with Cancel for %s while the start receipt is pending (Codex r2 of #1665)", (_case, before) => {
    const onCancel = vi.fn();
    const idle = before();
    const view = render(panel(idle, false, onCancel));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
    view.rerender(panel(idle, true, onCancel));
    expect(screen.getByRole("region", { name: "Solve progress" })).toHaveAttribute("data-state", "starting");
    const cancel = screen.getByRole("button", { name: "Stop" });
    expect(cancel).toHaveAttribute("aria-disabled", "false");
    fireEvent.click(cancel);
    expect(onCancel).toHaveBeenCalledTimes(1);
    openDetails();
    expect(screen.queryByTestId("solve-progress-events")).toBeNull();
    // A run that ends without publishing a record leaves nothing shown.
    view.rerender(panel(idle, false, onCancel));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
  });
});

describe("the status line's outcome and its actions", () => {
  const line = (job: SolveJobAuditState, running: boolean, actions: { onOpenResults?: () => void; onShowIssue?: () => void; resultsShown?: boolean; resultHeld?: boolean; onCancel?: () => void }) => (
    <SolveProgressPanel solveJob={job} running={running} onCancel={actions.onCancel ?? (() => {})} {...actions} />
  );
  const runningJob = () => progressSolveJob(startSolveJob(model, receipt), status({ state: "running" }));
  const completedWith = (statuses: string[], from = runningJob()) => completeSolveJob(from,
    { results: [{}, {}], diagnostics: [{}, {}] } as unknown as MechanicsResult,
    { analysis_run: { run_id: "analysis-run:invented-solved", analysis_status: statuses } } as unknown as AnalysisRunEnvelope);

  it("on completion reads one sentence, Solved and the rows; every status reported stays in Details with its token, the mechanics status first", () => {
    const onOpenResults = vi.fn();
    const view = render(line(initialSolveJob(), false, { onOpenResults }));
    view.rerender(line(initialSolveJob(), true, { onOpenResults }));
    view.rerender(line(runningJob(), true, { onOpenResults }));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Running$/);
    expect(screen.queryByTestId("solve-progress-open-results")).toBeNull();
    view.rerender(line(completedWith(["RULE_INPUTS_INCOMPLETE", "MECHANICS_SOLVED", "HUMAN_REVIEW_REQUIRED", "USER_RULE_CHECKED", "INTERNALLY_VERIFIED"]), false, { onOpenResults }));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Solved\. 2 result rows\.$/);
    // Human review required is on the line in no form, and no status's token or registered form is.
    expect(screen.getByTestId("solve-progress-panel").textContent).not.toMatch(/human review|[A-Z]{2,}_[A-Z]|Solver ·|Rule pack ·|Evidence ·/i);
    // No rule-inputs count was supplied: nothing to press for it.
    expect(screen.queryByTestId("solve-progress-rule-inputs")).toBeNull();
    expect(detailsOpen()).toBe(false);
    openDetails();
    expect(screen.getByTestId("solve-progress-statuses")).toHaveTextContent(
      /^Statuses reported: Solver · Mechanics solved \(MECHANICS_SOLVED\); Rule pack · Rule inputs incomplete \(RULE_INPUTS_INCOMPLETE\); Human · Human review required \(HUMAN_REVIEW_REQUIRED\); Rule pack · User-rule checked \(USER_RULE_CHECKED\); Evidence · Internally verified \(INTERNALLY_VERIFIED\)$/
    );
    fireEvent.click(screen.getByRole("button", { name: /Open results/ }));
    expect(onOpenResults).toHaveBeenCalledTimes(1);
    expect(screen.queryByTestId("solve-progress-show-issue")).toBeNull();
  });

  it("offers the missing rule inputs as one control with its count, which opens the Issues drawer at them; none without the status or the count", () => {
    const onShowRuleInputs = vi.fn();
    const render3 = (statuses: string[], ruleInputsMissing: number | null) => {
      const view = render(<SolveProgressPanel solveJob={initialSolveJob()} running={false} onCancel={() => {}} ruleInputsMissing={ruleInputsMissing} onShowRuleInputs={onShowRuleInputs} />);
      view.rerender(<SolveProgressPanel solveJob={initialSolveJob()} running onCancel={() => {}} ruleInputsMissing={ruleInputsMissing} onShowRuleInputs={onShowRuleInputs} />);
      view.rerender(<SolveProgressPanel solveJob={completedWith(statuses)} running={false} onCancel={() => {}} ruleInputsMissing={ruleInputsMissing} onShowRuleInputs={onShowRuleInputs} />);
      return view;
    };
    let view = render3(["MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"], 4);
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Solved\. 2 result rows\.$/);
    const control = screen.getByTestId("solve-progress-rule-inputs");
    expect(control).toHaveTextContent(/^Rule inputs missing: 4$/);
    fireEvent.click(control);
    expect(onShowRuleInputs).toHaveBeenCalledTimes(1);
    view.unmount();
    // The status alone, with no honest count, or a count without the status: not on the line.
    view = render3(["MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"], 0);
    expect(screen.queryByTestId("solve-progress-rule-inputs")).toBeNull();
    view.unmount();
    render3(["MECHANICS_SOLVED", "USER_RULE_CHECKED"], 4);
    expect(screen.queryByTestId("solve-progress-rule-inputs")).toBeNull();
  });

  it("a mechanics status other than solved is the line's first word, in the registry's plain words, with Show in Issues where no count is supplied", () => {
    const onShowIssue = vi.fn();
    const view = render(line(initialSolveJob(), false, { onShowIssue }));
    view.rerender(line(initialSolveJob(), true, { onShowIssue }));
    view.rerender(line(completedWith(["MODEL_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"]), false, { onShowIssue }));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Model incomplete after \d+:\d\d\. Nothing was solved\.$/);
    fireEvent.click(screen.getByTestId("solve-progress-show-issue"));
    expect(onShowIssue).toHaveBeenCalledTimes(1);
    // A completed job that reports no mechanics status claims nothing: the job's own state word.
    view.rerender(line(initialSolveJob(), true, { onShowIssue }));
    view.rerender(line(completedWith([]), false, { onShowIssue }));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Completed\. 2 result rows\.$/);
    expect(screen.queryByTestId("solve-progress-show-issue")).toBeNull();
  });

  it("does not offer Open results while the Results stage is the one shown, and offers it again elsewhere", () => {
    const view = render(line(initialSolveJob(), false, { onOpenResults: () => {} }));
    view.rerender(line(initialSolveJob(), true, { onOpenResults: () => {} }));
    const done = completedWith(["MECHANICS_SOLVED"]);
    view.rerender(line(done, false, { onOpenResults: () => {}, resultsShown: true }));
    expect(screen.queryByTestId("solve-progress-open-results")).toBeNull();
    view.rerender(line(done, false, { onOpenResults: () => {}, resultsShown: false }));
    expect(screen.getByTestId("solve-progress-open-results")).toBeInTheDocument();
  });

  it("on a failure says so with the job's reason and code and offers Show in Issues, not Open results", () => {
    const onShowIssue = vi.fn();
    const started = startSolveJob(model, receipt);
    const view = render(line(initialSolveJob(), false, { onShowIssue, onOpenResults: () => {} }));
    view.rerender(line(initialSolveJob(), true, { onShowIssue, onOpenResults: () => {} }));
    view.rerender(line(started, true, { onShowIssue, onOpenResults: () => {} }));
    view.rerender(line(failSolveJob(started, new Error("SOLVE_WORKER_PROTOCOL: invented reply")), false, { onShowIssue, onOpenResults: () => {} }));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Failed after \d+:\d\d\. The solve's worker process returned a reply the app could not read\..* Code: SOLVE_WORKER_PROTOCOL$/);
    expect(screen.queryByTestId("solve-progress-open-results")).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: /Show in Issues/ }));
    expect(onShowIssue).toHaveBeenCalledTimes(1);
  });

  it("closes Details at the start of each run, and keeps the person's choice between the reports of one run", () => {
    const view = render(line(initialSolveJob(), false, {}));
    view.rerender(line(initialSolveJob(), true, {}));
    const first = startSolveJob(model, receipt);
    view.rerender(line(first, true, {}));
    fireEvent.click(screen.getByTestId("solve-progress-details-toggle"));
    expect(detailsOpen()).toBe(true);
    // A later report of the same run: still open.
    const running = progressSolveJob(first, status({ state: "running" }));
    view.rerender(line(running, true, {}));
    expect(detailsOpen()).toBe(true);
    const done = completedWith([], running);
    view.rerender(line(done, false, {}));
    expect(detailsOpen()).toBe(true);
    // The next run starts closed.
    view.rerender(line(done, true, {}));
    expect(detailsOpen()).toBe(false);
    view.rerender(line(startSolveJob(model, { ...receipt, job_id: "backend-solve-job-33" }), true, {}));
    expect(detailsOpen()).toBe(false);
  });

  it("puts the words, the actions, Details and Dismiss in the order a person reads and tabs them, and Details keeps its place when it opens", () => {
    const view = render(line(initialSolveJob(), false, { onOpenResults: () => {} }));
    view.rerender(line(initialSolveJob(), true, { onOpenResults: () => {} }));
    view.rerender(line(completedWith(["MECHANICS_SOLVED"]), false, { onOpenResults: () => {} }));
    const order = () => [...screen.getByTestId("solve-progress-panel").querySelectorAll('[data-testid="solve-progress-state"], button')]
      .map((element) => element.getAttribute("data-testid"));
    const closed = ["solve-progress-state", "solve-progress-open-results", "solve-progress-details-toggle", "solve-progress-close"];
    expect(order()).toEqual(closed);
    openDetails();
    expect(order()).toEqual(closed);
    // The body opens after the line's controls, beneath them.
    const toggle = screen.getByTestId("solve-progress-details-toggle");
    expect(toggle.compareDocumentPosition(screen.getByTestId("solve-progress-details")) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(toggle).toHaveAttribute("aria-controls", screen.getByTestId("solve-progress-details").id);
  });

  it("Details leads with the facts, in order, and keeps every event's own message behind one closed disclosure", () => {
    const view = render(line(initialSolveJob(), false, {}));
    view.rerender(line(initialSolveJob(), true, {}));
    const queued = progressSolveJob(startSolveJob(model, receipt), status({ queue_reason: { kind: "queue_full", queued: 4, limit: 4 } }));
    const running = progressSolveJob(progressSolveJob(queued, status({ state: "running" })), status({ state: "running", resource: grant(64 * MiB, 96 * MiB, 70 * MiB) }));
    const done = completedWith(["MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"], running);
    for (const record of [queued, running]) view.rerender(line(record, true, {}));
    view.rerender(line(done, false, {}));
    const details = openDetails();
    const facts = [...details.children].filter((child) => child.tagName === "P").map((child) => child.getAttribute("data-testid") ?? child.textContent?.split(":")[0]);
    expect(facts).toEqual([
      "solve-progress-states", "solve-progress-statuses", "solve-progress-completed-counts", "solve-progress-worker", "solve-progress-queue-code", "Job"
    ]);
    const steps = screen.getByTestId("solve-progress-steps") as HTMLDetailsElement;
    expect(steps.open).toBe(false);
    expect(screen.getByTestId("solve-progress-steps-toggle")).toHaveTextContent("What the job reported at each step");
    openSteps();
    const items = within(screen.getByRole("list", { name: "Solve job events" })).getAllByRole("listitem");
    expect(items.map((item) => item.textContent)).toEqual(done.events.map((event) => expect.stringContaining(event.message)));
    expect(items[items.length - 1]).toContainElement(screen.getByTestId("solve-progress-stage"));
    expect(details.textContent).not.toMatch(/gathers a diagnostic repeated/);
  });

  it("shows the worker's peak footprint that first arrives with the job's final report, after completion", () => {
    const view = render(line(initialSolveJob(), false, {}));
    view.rerender(line(initialSolveJob(), true, {}));
    const running = progressSolveJob(progressSolveJob(startSolveJob(model, receipt), status({ state: "running" })), status({ state: "running", resource: grant(64 * MiB, 96 * MiB, null) }));
    view.rerender(line(running, true, {}));
    // The terminal status carries the measured footprint; the record follows it, then completes.
    const final = progressSolveJob(running, status({ state: "completed", result_available: true, resource: grant(null, 96 * MiB, 70 * MiB) }));
    view.rerender(line(completedWith(["MECHANICS_SOLVED"], final), false, {}));
    openDetails();
    expect(screen.getByTestId("solve-progress-worker")).toHaveTextContent(/^Worker memory: peak grant 96 MiB; peak footprint 70 MiB; budget 12 GiB$/);
  });

  it("shows a small moving mark beside the state word only while the job reports queued or running", () => {
    const view = render(line(initialSolveJob(), false, {}));
    view.rerender(line(initialSolveJob(), true, {}));
    expect(screen.queryByTestId("solve-progress-alive")).toBeNull();
    const queued = startSolveJob(model, receipt);
    view.rerender(line(queued, true, {}));
    expect(screen.getByTestId("solve-progress-alive")).toHaveAttribute("aria-hidden", "true");
    expect(screen.getByTestId("solve-progress-alive").textContent).toBe("");
    view.rerender(line(requestSolveCancellation(queued), true, {}));
    expect(screen.queryByTestId("solve-progress-alive")).toBeNull();
  });

  it("a stop requested by another route (the menu, its shortcut, the palette) reads the same on the line, without moving focus", () => {
    const onCancel = vi.fn();
    const view = render(line(initialSolveJob(), false, { onCancel }));
    view.rerender(line(initialSolveJob(), true, { onCancel }));
    const running = runningJob();
    view.rerender(line(running, true, { onCancel }));
    const elsewhere = document.createElement("button");
    document.body.append(elsewhere);
    elsewhere.focus();
    // The session's one cancel handler records the request; the line reads the record.
    view.rerender(line(requestSolveCancellation(running), true, { onCancel }));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Cancelling$/);
    expect(screen.getByTestId("solve-progress-cancel")).toHaveTextContent("Cancelling");
    expect(screen.getByTestId("solve-progress-cancel")).toHaveAttribute("aria-disabled", "true");
    expect(document.activeElement).toBe(elsewhere);
    fireEvent.click(screen.getByTestId("solve-progress-cancel"));
    expect(onCancel).not.toHaveBeenCalled();
    elsewhere.remove();
  });

  it("says a stopped run cleared the result held before it, and says nothing of the kind where none was held", () => {
    const stoppedAfter = (resultHeld: boolean) => {
      const view = render(line(initialSolveJob(), false, { resultHeld }));
      view.rerender(line(initialSolveJob(), true, { resultHeld: false }));
      const running = runningJob();
      view.rerender(line(running, true, { resultHeld: false }));
      view.rerender(line(cancelledSolveJob(requestSolveCancellation(running), status({ state: "cancelled", cancellation_requested: true, cancellation_status: "worker_terminated" })), false, { resultHeld: false }));
      const text = screen.getByTestId("solve-progress-state").textContent;
      view.unmount();
      return text;
    };
    expect(stoppedAfter(true)).toMatch(/^Stopped after \d+:\d\d\. No result was published\. The earlier result was cleared when this run started\.$/);
    expect(stoppedAfter(false)).toMatch(/^Stopped after \d+:\d\d\. No result was published\.$/);
  });
});

describe("a solve that ends with the model incomplete (the missing-data use case's model)", () => {
  // The committed use case: two pipes lacking a density and one seismic case. The job's completed
  // event carries MODEL_INCOMPLETE, no result rows and the solve's two findings (invented here as
  // the solve gives them: one per pipe).
  const useCaseModel = missingDataStartModel as unknown as PreviewModel;
  const findings = useCaseModel.pipe_segments.map((pipe) => ({
    id: `invented:${pipe.id}`, code: "EQUIVALENT_STATIC_INPUT_MISSING", severity: "blocking" as const,
    message: "invented: seismic generation requires material_density on every pipe segment", affected_refs: [pipe.id]
  }));

  it("reads that the model is incomplete and nothing was solved, offers the blocking findings with their count, never Completed or 0 result rows", () => {
    const onShowBlocking = vi.fn();
    const items = groupIssues({ model: useCaseModel, diagnostics: findings, diagnosticOrigins: ["solve", "solve"] });
    const count = solveBlockingItems(items).length;
    expect(count).toBe(2);
    const props = { onCancel: () => {}, blockingFindings: count, onShowBlocking, onShowIssue: () => {} };
    const view = render(<SolveProgressPanel solveJob={initialSolveJob()} running={false} {...props} />);
    view.rerender(<SolveProgressPanel solveJob={initialSolveJob()} running {...props} />);
    const running = progressSolveJob(startSolveJob(useCaseModel, receipt), status({ state: "running" }));
    view.rerender(<SolveProgressPanel solveJob={running} running {...props} />);
    const done = completeSolveJob(running, { results: [], diagnostics: findings } as unknown as MechanicsResult,
      { analysis_run: { run_id: "analysis-run:invented-missing-data", analysis_status: ["MODEL_INCOMPLETE"] } } as unknown as AnalysisRunEnvelope);
    view.rerender(<SolveProgressPanel solveJob={done} running={false} {...props} />);
    const line = screen.getByTestId("solve-progress-state");
    expect(line).toHaveTextContent(/^Model incomplete after \d+:\d\d\. Nothing was solved\.$/);
    expect(screen.getByTestId("solve-progress-panel").textContent).not.toMatch(/Completed|result rows?/);
    expect(screen.queryByTestId("solve-progress-show-issue")).toBeNull();
    expect(screen.queryByTestId("solve-progress-open-results")).toBeNull();
    const control = screen.getByTestId("solve-progress-blocking");
    expect(control).toHaveTextContent(/^Blocking findings: 2$/);
    fireEvent.click(control);
    expect(onShowBlocking).toHaveBeenCalledTimes(1);
    // The job's own event text is in Details, in its steps, and not on the line.
    const last = done.events[done.events.length - 1].message;
    expect(line.textContent).not.toContain(last);
    openSteps();
    expect(screen.getByTestId("solve-progress-stage")).toHaveTextContent(last);
    expect(screen.getByTestId("solve-progress-statuses")).toHaveTextContent(/^Statuses reported: Solver · Model incomplete \(MODEL_INCOMPLETE\)$/);
  });
});

describe("the line's elapsed time, measured by the desktop", () => {
  afterEach(() => { vi.useRealTimers(); });
  const runningJob = () => progressSolveJob(startSolveJob(model, receipt), status({ state: "running" }));
  const done = (from: SolveJobAuditState) => completeSolveJob(from,
    { results: [{}, {}], diagnostics: [] } as unknown as MechanicsResult,
    { analysis_run: { run_id: "analysis-run:invented-elapsed", analysis_status: ["MECHANICS_SOLVED"] } } as unknown as AnalysisRunEnvelope);

  it("counts once a second from the run's start outside the live region, says how long it took, and then stands still", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-01-01T00:00:00Z"));
    const view = render(panel(initialSolveJob(), false));
    view.rerender(panel(initialSolveJob(), true));
    const running = runningJob();
    view.rerender(panel(running, true));
    const figure = () => screen.getByTestId("solve-progress-elapsed").textContent;
    expect(figure()).toBe("· 0:00 elapsed");
    act(() => { vi.advanceTimersByTime(42_000); });
    expect(figure()).toBe("· 0:42 elapsed");
    // The figure is not in the live region; the state word is.
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Running$/);
    expect(screen.getByTestId("solve-progress-state")).not.toContainElement(screen.getByTestId("solve-progress-elapsed"));
    expect(screen.getByTestId("solve-progress-elapsed").closest("[aria-live]")).toBeNull();
    act(() => { vi.advanceTimersByTime(30_000); });
    view.rerender(panel(done(running), false));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Solved in 1:12\. 2 result rows\.$/);
    expect(screen.queryByTestId("solve-progress-elapsed")).toBeNull();
    act(() => { vi.advanceTimersByTime(60_000); });
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Solved in 1:12\. 2 result rows\.$/);
    expect(screen.getByTestId("solve-progress-panel").textContent).not.toMatch(NO_ESTIMATE);
  });

  it("a stopped run says how long it ran; hours appear when needed", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-01-01T00:00:00Z"));
    const view = render(panel(initialSolveJob(), false));
    view.rerender(panel(initialSolveJob(), true));
    const running = runningJob();
    view.rerender(panel(running, true));
    act(() => { vi.advanceTimersByTime(15_000); });
    view.rerender(panel(cancelledSolveJob(requestSolveCancellation(running), status({ state: "cancelled", cancellation_requested: true, cancellation_status: "worker_terminated" })), false));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Stopped after 0:15\. No result was published\.$/);
    expect(formatElapsed(3_725_000)).toBe("1:02:05");
    expect(formatElapsed(59_999)).toBe("0:59");
  });

  it("a recorded run the line did not see start shows no interval", () => {
    render(panel({ ...done(runningJob()) }, false));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
  });
});

describe("the Issues drawer's Last solve item", () => {
  const started = () => startSolveJob(model, receipt);

  it.each([
    ["SOLVE_WORKER_EXITED", "The solve's worker process ended without returning a result. The solve failed; this is not a memory-pool refusal."],
    ["SOLVE_WORKER_PROTOCOL", "The solve's worker process returned a reply the app could not read. The solve failed; this is not a memory-pool refusal."]
  ])("a worker failure %s: its reading and its exact code", (code, reading) => {
    render(<SolveIssueItem solveJob={failSolveJob(started(), new Error(`${code}: invented detail`))} />);
    const item = screen.getByTestId("issues-solve-item");
    expect(item).toHaveTextContent("The solve failed");
    expect(item).toHaveTextContent(reading);
    expect(screen.getByTestId("issues-solve-failure-code")).toHaveTextContent(new RegExp(`^${code}$`));
    expect(item).toHaveTextContent("Job backend-solve-job-31");
  });

  it("a memory-pool refusal: its reading, the way to retry and its exact code", () => {
    render(<SolveIssueItem solveJob={refusedSolveJob(started(), { kind: "headroom_insufficient", required_bytes: 68165632, available_bytes: 52641792 })} />);
    const item = screen.getByTestId("issues-solve-item");
    expect(item).toHaveTextContent("The solve was refused");
    expect(item).toHaveTextContent("Refused by the native memory pool: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 65.01 MiB, 50.2 MiB available. No result was published.");
    expect(item).toHaveTextContent("You can run the solve again when more memory is available.");
    expect(item).toHaveTextContent("Refusal code: headroom_insufficient (required_bytes=68165632, available_bytes=52641792)");
    expect(screen.queryByTestId("issues-solve-failure-code")).toBeNull();
  });

  it("a result too large to show: the job's own reading with its code, and no memory-pool wording", () => {
    const tooLarge = resultTooLargeSolveJob(started(), { code: "PREVIEW-SOLVE-RESULT-TOO-LARGE", result_bytes: 147_000_000, limit_bytes: 64 * MiB });
    render(<SolveIssueItem solveJob={tooLarge} />);
    const item = screen.getByTestId("issues-solve-item");
    expect(item).toHaveTextContent("The solve was refused");
    expect(item).toHaveTextContent(tooLarge.error_message!);
    expect(item).toHaveTextContent("Code: PREVIEW-SOLVE-RESULT-TOO-LARGE (result_bytes=147000000, limit_bytes=67108864)");
    expect(item).not.toHaveTextContent("native memory pool");
  });
});

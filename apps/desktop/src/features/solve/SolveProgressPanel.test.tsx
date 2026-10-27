import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
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
  startSolveJob
} from "../workspace/solveJobAudit";
import { SolveProgressPanel } from "./SolveProgressPanel";

// DEL-07-07 Design `solve-progress-panel.md`: the panel shows the solve-job
// record the session already keeps, from invented native job reports.
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

  it("shows the live state, its latest event as the stage, every event in order, and no percentage or time", () => {
    const queued = startSolveJob(model, receipt);
    const view = renderRun(queued);
    const region = screen.getByRole("region", { name: "Solve progress" });
    expect(region).toHaveAttribute("data-state", "queued");
    expect(screen.getByTestId("solve-progress-state")).toHaveAttribute("role", "status");
    expect(screen.getByTestId("solve-progress-state")).toHaveAttribute("aria-live", "polite");
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Queued/);
    expect(screen.getByTestId("solve-progress-stage")).toHaveTextContent(queued.events[0].message);
    expect(screen.getByTestId("solve-progress-job")).toHaveTextContent("backend-solve-job-31");

    const waiting = progressSolveJob(queued, status({ queue_reason: { kind: "pool_exhausted", requested_bytes: 12 * GiB, reserved_bytes: 40 * GiB, pool_bytes: 48 * GiB } }));
    view.rerender(panel(waiting, true));
    expect(screen.getByTestId("solve-progress-queue-reason")).toHaveTextContent(
      "Waiting in the queue: The memory pool is fully reserved: this request needs 12 GiB, and 40 GiB of the 48 GiB pool is already reserved. Code: pool_exhausted (requested_bytes=12884901888, reserved_bytes=42949672960, pool_bytes=51539607552)"
    );

    const running = progressSolveJob(progressSolveJob(waiting, status({ state: "running" })), status({ state: "running", resource: grant(64 * MiB, 96 * MiB, 70 * MiB) }));
    view.rerender(panel(running, true));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Running/);
    expect(screen.queryByTestId("solve-progress-queue-reason")).toBeNull();
    expect(screen.getByTestId("solve-progress-worker")).toHaveTextContent("Worker memory: grant 64 MiB; peak grant 96 MiB; peak footprint 70 MiB; budget 12 GiB");
    const events = within(screen.getByRole("list", { name: "Solve job events" })).getAllByRole("listitem");
    expect(events.map((item) => item.getAttribute("data-event-id"))).toEqual(["solve-preview-queued", "solve-preview-queued-waiting", "solve-preview-running"]);
    expect(events.map((item) => item.getAttribute("data-state"))).toEqual(["queued", "queued", "running"]);
    // The record reports states only: nothing reads as a fraction, an estimate or a clock.
    expect(screen.getByTestId("solve-progress-basis")).toHaveTextContent(
      "The solve job reports its state and events, not a percentage, a time estimate or a start time, so none is shown (progress basis: preview_service_event_state_only_no_percent_stream)."
    );
    expect(screen.getByTestId("solve-progress-panel").textContent).not.toMatch(/\d\s*%|\bETA\b|elapsed|remaining/i);
    expect(screen.queryByRole("progressbar")).toBeNull();
  });

  it("offers Cancel as a focusable button; once requested it stays in place, aria-disabled, and keyboard focus moves to the state", () => {
    const onCancel = vi.fn();
    const running = progressSolveJob(startSolveJob(model, receipt), status({ state: "running" }));
    const view = renderRun(running, onCancel);
    const cancel = screen.getByRole("button", { name: "Cancel solve" });
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
    expect(screen.getByTestId("solve-progress-cancellation")).toHaveTextContent("Cancellation requested: request_sent_to_backend_job_awaiting_receipt");

    const cancelled = cancelledSolveJob(cancelling, status({ state: "cancelled", cancellation_requested: true, cancellation_status: "worker_terminated" }));
    view.rerender(panel(cancelled, false, onCancel));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Cancelled/);
    expect(screen.queryByTestId("solve-progress-cancel")).toBeNull();
    expect(screen.getByTestId("solve-progress-cancellation")).toHaveTextContent("Cancellation requested: worker_terminated");
  });

  it("shows a memory-pool refusal with its code, exact bytes and retry path, and no result", () => {
    const started = startSolveJob(model, receipt);
    const view = renderRun(started);
    const refused = refusedSolveJob(started, { kind: "headroom_insufficient", required_bytes: 68165632, available_bytes: 52641792 });
    view.rerender(panel(refused, false));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Refused/);
    expect(screen.getByTestId("solve-progress-refusal")).toHaveTextContent(
      "Refused by the native memory pool: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 65.01 MiB, 50.2 MiB available. No result was published."
    );
    expect(screen.getByTestId("solve-progress-retry")).toHaveTextContent(
      "Refused at the result hand-off after the solve finished; the finished result was dropped. Available memory was re-sampled for up to 2 s before this refusal stood. You can run the solve again when more memory is available."
    );
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
  });

  it("shows completion with the rows and diagnostics the completed event carries; Close dismisses it until the next run", () => {
    const running = progressSolveJob(startSolveJob(model, receipt), status({ state: "running" }));
    const view = renderRun(running);
    const result = { results: [{}, {}, {}], diagnostics: [{}] } as unknown as MechanicsResult;
    const run = { analysis_run: { run_id: "analysis-run:invented-31", analysis_status: [] } } as unknown as AnalysisRunEnvelope;
    const completed = completeSolveJob(running, result, run);
    view.rerender(panel(completed, false));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Completed/);
    expect(screen.getByTestId("solve-progress-completed")).toHaveTextContent("3 result rows; 1 diagnostic.");
    fireEvent.click(screen.getByRole("button", { name: "Close" }));
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
    fireEvent.click(screen.getByRole("button", { name: "Close" }));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
    // A new run starts; the session still holds the previous record until the start receipt.
    view.rerender(panel(previous, true, onCancel));
    const region = screen.getByRole("region", { name: "Solve progress" });
    expect(region).toHaveAttribute("data-state", "starting");
    expect(screen.queryByTestId("solve-progress-job")).toBeNull();
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent("Starting");
    expect(screen.queryByTestId("solve-progress-failure")).toBeNull();
    expect(screen.queryByTestId("solve-progress-completed")).toBeNull();
    expect(within(screen.getByRole("list", { name: "Solve job events" })).queryAllByRole("listitem")).toHaveLength(0);
    fireEvent.click(screen.getByRole("button", { name: "Cancel solve" }));
    expect(onCancel).toHaveBeenCalledTimes(1);
    // A cancellation before the start receipt records a new state over the previous events; only the new event is this run's.
    const pending = { ...requestSolveCancellation(startSolveJob(model, receipt)), job_id: "job:preview-linear-static:pending-start-cancel",
      events: [...previous.events, { event_id: "solve-preview-cancel-requested-before-backend-start", state: "cancelling" as const, message: "invented pending cancel", result_available: false, diagnostic_count: 0, result_row_count: 0, analysis_status: [] }] };
    view.rerender(panel(pending, true, onCancel));
    expect(within(screen.getByRole("list", { name: "Solve job events" })).getAllByRole("listitem").map((item) => item.getAttribute("data-event-id")))
      .toEqual(["solve-preview-cancel-requested-before-backend-start"]);
    // The new job's own record replaces it.
    view.rerender(panel({ ...startSolveJob(model, { ...receipt, job_id: "backend-solve-job-32" }) }, true, onCancel));
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
});

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
    const view = render(panel(queued, true));
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
    const view = render(panel(running, true, onCancel));
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
    const view = render(panel(started, true));
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
    const view = render(panel(started, true));
    view.rerender(panel(failSolveJob(started, new Error("SOLVE_WORKER_EXITED: invented exit")), false));
    expect(screen.getByTestId("solve-progress-failure")).toHaveTextContent(
      "The solve's worker process ended without returning a result. The solve failed; this is not a memory-pool refusal."
    );
  });

  it("shows completion with the rows and diagnostics the completed event carries; Close dismisses it until the next run", () => {
    const running = progressSolveJob(startSolveJob(model, receipt), status({ state: "running" }));
    const view = render(panel(running, true));
    const result = { results: [{}, {}, {}], diagnostics: [{}] } as unknown as MechanicsResult;
    const run = { analysis_run: { run_id: "analysis-run:invented-31", analysis_status: [] } } as unknown as AnalysisRunEnvelope;
    const completed = completeSolveJob(running, result, run);
    view.rerender(panel(completed, false));
    expect(screen.getByTestId("solve-progress-state")).toHaveTextContent(/^Completed/);
    expect(screen.getByTestId("solve-progress-completed")).toHaveTextContent("3 result rows; 1 diagnostic.");
    fireEvent.click(screen.getByRole("button", { name: "Close" }));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
    // The next run opens it again.
    view.rerender(panel(startSolveJob(model, receipt), true));
    expect(screen.getByTestId("solve-progress-panel")).toHaveAttribute("data-state", "queued");
  });

  it("closes when the record no longer describes a job (the model changed)", () => {
    const running = progressSolveJob(startSolveJob(model, receipt), status({ state: "running" }));
    const view = render(panel(running, true));
    view.rerender(panel(modelChangedSolveJob({ operation_id: "op:invented-edit" } as never), false));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
    // A later record the session did not run stays hidden.
    view.rerender(panel({ ...initialSolveJob(), state: "completed" }, false));
    expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
  });
});

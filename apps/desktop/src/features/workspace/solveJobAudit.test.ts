import { describe, expect, it } from "vitest";
import { ResourceUnavailableError } from "../../services/memoryPool";
import type { BackendSolveJobStatus, SolveJobStartReceipt } from "../../services/previewService";
import type { PoolStatus, PreviewModel, SolveResourceGrant } from "../../types";
import {
  cancelledSolveJob,
  describeSolveRefusalRetry,
  failSolveJob,
  initialSolveJob,
  progressSolveJob,
  refusedAtTakeSolveJob,
  refusedBeforeBackendStartSolveJob,
  refusedSolveJob,
  requestSolveCancellation,
  solveRefusalPoint,
  startSolveJob
} from "./solveJobAudit";

// Pure audit transitions over invented native job reports (DEL-04-07).
const GiB = 1024 ** 3;
const model = {
  project: { id: "project:invented-memory-audit" },
  diagnostics: [],
  analysis_status: { mechanics: "MODEL_INCOMPLETE", rule_check: "RULE_INPUTS_INCOMPLETE" }
} as unknown as PreviewModel;
const baseGrant: SolveResourceGrant = { invocation_budget_bytes: 12 * GiB, pool_bytes: 48 * GiB, route_pricing: "ordinary_worker_capped", ingress_reserved_bytes: 3 * 1024 ** 2, worker_grant_bytes: null, worker_peak_grant_bytes: null, worker_peak_footprint_bytes: null };
const grant = (worker: number | null, peak: number | null, footprint: number | null = null): SolveResourceGrant =>
  ({ ...baseGrant, worker_grant_bytes: worker, worker_peak_grant_bytes: peak, worker_peak_footprint_bytes: footprint });
const receipt: SolveJobStartReceipt = {
  mode: "backend_job",
  job_id: "backend-solve-job-7",
  backend_cancellation_token: "token-7",
  state: "queued",
  cancellation_scope: "worker_process_terminated",
  resource: baseGrant
};
function status(overrides: Partial<BackendSolveJobStatus>): BackendSolveJobStatus {
  return {
    job_id: "backend-solve-job-7", state: "queued", queue_reason: null, cancellation_requested: false,
    cancellation_status: "not_requested", cancellation_scope: "worker_process_terminated",
    result_available: false, result_bytes: null, error_message: null, resource_refusal: null, resource: receipt.mode === "backend_job" ? receipt.resource : null,
    ...overrides
  };
}

describe("solve-job audit resource record", () => {
  it("starts with no reported resource", () => {
    expect(initialSolveJob().resource).toEqual({ source: "not_reported", invocation_budget_bytes: null, pool_bytes: null, route_pricing: null, ingress_reserved_bytes: null, worker_grant_bytes: null, worker_peak_grant_bytes: null, worker_peak_footprint_bytes: null, queue_reason: null, refusal: null });
  });

  it("records the native grant: per-invocation budget, pool and the worker-capped ordinary route", () => {
    const job = startSolveJob(model, receipt);
    expect(job.state).toBe("queued");
    expect(job.resource).toEqual({ source: "native_job_receipt", invocation_budget_bytes: 12 * GiB, pool_bytes: 48 * GiB, route_pricing: "ordinary_worker_capped", ingress_reserved_bytes: 3 * 1024 ** 2, worker_grant_bytes: null, worker_peak_grant_bytes: null, worker_peak_footprint_bytes: null, queue_reason: null, refusal: null });
    expect(job.events.at(-1)!.message).toContain("per-invocation budget 12 GiB (bounds the model intake and caps the ordinary solve worker, whose grant grows on demand up to it); pool 48 GiB; model intake reserved 3 MiB; route ordinary_worker_capped; worker grant not yet held");
    expect(job.events.at(-1)!.message).not.toMatch(/unpriced/);
    // A queued job can be cancelled.
    expect(requestSolveCancellation(job).state).toBe("cancelling");
    // The native cancellation scope is kept for the export.
    expect(job.backend_cancellation_scope).toBe("worker_process_terminated");
    expect(initialSolveJob().backend_cancellation_scope).toBeNull();
  });

  it("shows a queue reason while waiting, once per reason, then the move to running", () => {
    const started = startSolveJob(model, receipt);
    const waiting = progressSolveJob(started, status({ queue_reason: { kind: "pool_exhausted", requested_bytes: 12 * GiB, reserved_bytes: 40 * GiB, pool_bytes: 48 * GiB } }));
    expect(waiting.state).toBe("queued");
    expect(waiting.resource.queue_reason).toEqual({ kind: "pool_exhausted", requested_bytes: 12 * GiB, reserved_bytes: 40 * GiB, pool_bytes: 48 * GiB });
    expect(waiting.events.at(-1)).toMatchObject({ event_id: "solve-preview-queued-waiting", state: "queued" });
    expect(waiting.events.at(-1)!.message).toBe("Backend solve job backend-solve-job-7 is waiting in the queue: The memory pool is fully reserved: this request needs 12 GiB, and 40 GiB of the 48 GiB pool is already reserved.");
    const same = progressSolveJob(waiting, status({ queue_reason: { kind: "pool_exhausted", requested_bytes: 12 * GiB, reserved_bytes: 40 * GiB, pool_bytes: 48 * GiB } }));
    expect(same).toBe(waiting);
    const running = progressSolveJob(waiting, status({ state: "running" }));
    expect(running.state).toBe("running");
    expect(running.resource.queue_reason).toBeNull();
    expect(running.events.at(-1)!.event_id).toBe("solve-preview-running");
    // Another job's status does not touch this record.
    expect(progressSolveJob(running, status({ job_id: "other", state: "queued", queue_reason: { kind: "headroom_unknown" } }))).toBe(running);
  });

  it("shows the running-worker limit as a queue reason while the job waits", () => {
    const waiting = progressSolveJob(startSolveJob(model, receipt), status({ queue_reason: { kind: "worker_limit_reached", running: 4, limit: 4 } }));
    expect(waiting.state).toBe("queued");
    expect(waiting.resource.queue_reason).toEqual({ kind: "worker_limit_reached", running: 4, limit: 4 });
    expect(waiting.resource.refusal).toBeNull();
    expect(waiting.events.at(-1)!.message).toBe("Backend solve job backend-solve-job-7 is waiting in the queue: 4 solve workers are already running (limit 4); this solve waits until one finishes.");
  });

  it("follows the worker's current and peak grant as it grows, keeps the peak, and records the final report of a terminal status", () => {
    const running = progressSolveJob(startSolveJob(model, receipt), status({ state: "running", resource: grant(2 * GiB, 2 * GiB) }));
    expect(running.resource).toMatchObject({ worker_grant_bytes: 2 * GiB, worker_peak_grant_bytes: 2 * GiB });
    const grown = progressSolveJob(running, status({ state: "running", resource: grant(6 * GiB, 6 * GiB) }));
    expect(grown.resource).toMatchObject({ worker_grant_bytes: 6 * GiB, worker_peak_grant_bytes: 6 * GiB });
    expect(grown.events).toEqual(running.events);
    // An unchanged report leaves the record as it is.
    expect(progressSolveJob(grown, status({ state: "running", resource: grant(6 * GiB, 6 * GiB) }))).toBe(grown);
    // A report with a lower peak never lowers the recorded peak.
    expect(progressSolveJob(grown, status({ state: "running", resource: grant(3 * GiB, 3 * GiB) })).resource).toMatchObject({ worker_grant_bytes: 3 * GiB, worker_peak_grant_bytes: 6 * GiB });
    // The terminal status (the worker has exited and released its grant) is recorded before the terminal transition.
    const finished = progressSolveJob(grown, status({ state: "completed", result_available: true, resource: grant(null, 7 * GiB) }));
    expect(finished.state).toBe("running");
    expect(finished.resource).toMatchObject({ worker_grant_bytes: null, worker_peak_grant_bytes: 7 * GiB, route_pricing: "ordinary_worker_capped" });
    expect(finished.events).toEqual(grown.events);
  });

  it("records the worker's peak footprint, which can briefly exceed its peak grant, and never lowers it", () => {
    const running = progressSolveJob(startSolveJob(model, receipt), status({ state: "running", resource: grant(2 * GiB, 2 * GiB, 2.5 * GiB) }));
    expect(running.resource).toMatchObject({ worker_peak_grant_bytes: 2 * GiB, worker_peak_footprint_bytes: 2.5 * GiB });
    const later = progressSolveJob(running, status({ state: "running", resource: grant(3 * GiB, 3 * GiB, 1 * GiB) }));
    expect(later.resource).toMatchObject({ worker_peak_grant_bytes: 3 * GiB, worker_peak_footprint_bytes: 2.5 * GiB });
    const unmeasured = progressSolveJob(later, status({ state: "running", resource: grant(3 * GiB, 3 * GiB, null) }));
    expect(unmeasured).toBe(later);
  });

  it("states what cancelling a queued or running job does, without a cooperative-checkpoint or success claim", () => {
    const queued = startSolveJob(model, receipt);
    expect(queued.events.at(-1)!.message).toContain("cancelling a queued job removes it before a worker starts; cancelling a running job terminates its worker process, and the job reads cancelled (no result published) only after the worker has exited (scope worker_process_terminated)");
    const running = progressSolveJob(queued, status({ state: "running" }));
    expect(running.events.at(-1)!.message).toBe("Backend solve job backend-solve-job-7 is executing in its worker process; cancelling it terminates that worker process, and the job reads cancelled (no result published) only after the worker has exited (scope worker_process_terminated); this service path does not stream percentage progress.");
    const requested = requestSolveCancellation(running);
    expect(requested.events.at(-1)!.message).toBe("Cancellation requested for backend solve job backend-solve-job-7 using its backend cancellation token; cancelling a queued job removes it before a worker starts; cancelling a running job terminates its worker process, and the job reads cancelled (no result published) only after the worker has exited; no cancellation-success guarantee is claimed beyond that.");
    const cancelled = cancelledSolveJob(requested, status({ state: "cancelled", cancellation_requested: true, cancellation_status: "cancellation_requested_worker_terminating" }));
    expect(cancelled.state).toBe("cancelled");
    expect(cancelled.events.at(-1)!.message).toBe("Backend solve job backend-solve-job-7 reads cancelled (cancellation_requested_worker_terminating): a queued job was removed before a worker started, or a running job's worker process was terminated and has exited; no result was published, and no cancellation-success claim is made beyond that.");
    for (const job of [queued, running, requested, cancelled]) expect(job.events.map((event) => event.message).join(" ")).not.toMatch(/cooperative|checkpoint/);
  });

  it("records a terminal refusal as a typed refusal, not a generic error", () => {
    const refusal = { kind: "headroom_insufficient" as const, required_bytes: 12 * GiB, available_bytes: 5 * GiB };
    const refused = refusedSolveJob(startSolveJob(model, receipt), refusal);
    expect(refused.state).toBe("refused");
    expect(refused.error_message).toBeNull();
    expect(refused.resource).toMatchObject({ source: "native_job_receipt", invocation_budget_bytes: 12 * GiB, pool_bytes: 48 * GiB, route_pricing: "ordinary_worker_capped", refusal });
    expect(refused.events.at(-1)).toMatchObject({ event_id: "solve-preview-resource-refused", state: "refused", result_available: false });
    // A job's bare headroom refusal is its result lease at the hand-off: the point, the retry path and the exact code are named.
    expect(refused.events.at(-1)!.message).toBe("Refused by the native memory pool (headroom_insufficient): Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 12 GiB, 5 GiB available. No result was published. Refused at the result hand-off after the solve finished; the finished result was dropped. Available memory was re-sampled for up to 2 s before this refusal stood. You can run the solve again when more memory is available. Refusal code: headroom_insufficient (required_bytes=12884901888, available_bytes=5368709120).");
    expect(solveRefusalPoint(refused)).toBe("result_handoff");
    // A ResourceUnavailableError reaching the failure path is recorded the same way.
    const atStep = failSolveJob(startSolveJob(model, receipt), new ResourceUnavailableError(refusal, null));
    expect(atStep.state).toBe("refused");
    // There it is a native command of the session's solve steps, not the result hand-off.
    expect(atStep.events.at(-1)!.event_id).toBe("solve-preview-resource-refused-at-session-step");
    expect(solveRefusalPoint(atStep)).toBe("session_step");
    expect(failSolveJob(startSolveJob(model, receipt), new Error("ordinary failure")).state).toBe("failed");
  });

  const LARGER_BUDGET = "This solve needed more than the per-invocation budget allows; a larger budget, up to the pool, can be set under Memory pool before running the solve again.";
  const STOPPED = "Refused while the solve ran: its worker process was stopped, with no fallback to another solver mode or route.";
  it.each([
    [{ kind: "worker_cap_exceeded" as const, cap_bytes: 12 * GiB, requested_bytes: 13 * GiB }, "The solve needed more memory than the per-invocation budget of 12 GiB allows (requested 13 GiB).",
      `${STOPPED} ${LARGER_BUDGET} Refusal code: worker_cap_exceeded (cap_bytes=12884901888, requested_bytes=13958643712).`],
    [{ kind: "worker_growth_refused" as const, granted_bytes: 4 * GiB, requested_bytes: 8 * GiB, cause: { kind: "headroom_insufficient" as const, required_bytes: 4 * GiB, available_bytes: 1 * GiB } },
      "The solve needed more memory (8 GiB) than could be reserved while it ran: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 4 GiB, 1 GiB available.",
      `${STOPPED} Available memory was re-sampled for up to 2 s before this refusal stood. You can run the solve again when more memory is available. Refusal code: worker_growth_refused (granted_bytes=4294967296, requested_bytes=8589934592); cause headroom_insufficient (required_bytes=4294967296, available_bytes=1073741824).`],
    [{ kind: "worker_footprint_exceeded" as const, grant_bytes: 4 * GiB, footprint_bytes: 5 * GiB }, "The solve's measured memory footprint (5 GiB) does not fit the per-invocation budget, so the solve was ended (its reservation was 4 GiB).",
      `${STOPPED} ${LARGER_BUDGET} Refusal code: worker_footprint_exceeded (grant_bytes=4294967296, footprint_bytes=5368709120).`]
  ])("records a running solve's worker refusal %s as a typed refusal, keeping the worker grant", (refusal, sentence, retry) => {
    const running = progressSolveJob(startSolveJob(model, receipt), status({ state: "running", resource: grant(4 * GiB, 4 * GiB) }));
    const refused = refusedSolveJob(running, refusal);
    expect(refused.state).toBe("refused");
    expect(refused.error_message).toBeNull();
    expect(refused.resource).toMatchObject({ refusal, worker_grant_bytes: 4 * GiB, worker_peak_grant_bytes: 4 * GiB });
    expect(refused.events.at(-1)!.message).toBe(`Refused by the native memory pool (${refusal.kind}): ${sentence} No result was published. ${retry}`);
    expect(solveRefusalPoint(refused)).toBe("worker");
  });

  it("records a refused take of a completed job's result at the take, with its exact bytes and the session's retry path, and no rows", () => {
    const refusal = { kind: "headroom_insufficient" as const, required_bytes: 68165632, available_bytes: 52641792 };
    const completedNatively = progressSolveJob(startSolveJob(model, receipt), status({ state: "completed", result_available: true, result_bytes: 4096, resource: grant(null, 65 * 1024 ** 2) }));
    const refused = refusedAtTakeSolveJob(completedNatively, new ResourceUnavailableError(refusal, null));
    expect(refused.state).toBe("refused");
    expect(refused.error_message).toBeNull();
    expect(refused.resource).toMatchObject({ source: "native_job_receipt", worker_peak_grant_bytes: 65 * 1024 ** 2, refusal });
    expect(refused.events.at(-1)).toMatchObject({ event_id: "solve-preview-resource-refused-at-take", state: "refused", result_available: false, result_row_count: 0 });
    expect(refused.events.every((event) => event.state !== "completed" && event.result_row_count === 0)).toBe(true);
    expect(solveRefusalPoint(refused)).toBe("take");
    expect(refused.events.at(-1)!.message).toBe("Refused by the native memory pool (headroom_insufficient): Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 65.01 MiB, 50.2 MiB available. No result was published. Refused when the finished result was handed to the app; the session does not retry a refused take, so it released the result. Available memory was re-sampled for up to 2 s before this refusal stood. You can run the solve again when more memory is available. Refusal code: headroom_insufficient (required_bytes=68165632, available_bytes=52641792).");
    // A custody refusal at the take names its own retry path.
    const custody = refusedAtTakeSolveJob(completedNatively, new ResourceUnavailableError({ kind: "custody_full", delivered: 4, limit: 4 }, null));
    expect(describeSolveRefusalRetry(custody.resource.refusal!, solveRefusalPoint(custody)!)).toMatch(/You can run the solve again once a held result has been released\.$/);
  });

  it("names each refusal point from the record's existing fields", () => {
    const job = startSolveJob(model, receipt);
    const headroom = { kind: "headroom_insufficient" as const, required_bytes: 1, available_bytes: 0 };
    expect(solveRefusalPoint(refusedBeforeBackendStartSolveJob(model, new ResourceUnavailableError(headroom, null)))).toBe("before_start");
    expect(solveRefusalPoint(refusedSolveJob(job, headroom))).toBe("result_handoff");
    expect(solveRefusalPoint(refusedSolveJob(job, { kind: "pressure_elevated", level: "critical" }))).toBe("result_handoff");
    expect(solveRefusalPoint(refusedSolveJob(job, { kind: "retention_full", retained: 4, limit: 4 }))).toBe("publication");
    expect(solveRefusalPoint(refusedSolveJob(job, { kind: "worker_cap_exceeded", cap_bytes: 2, requested_bytes: 3 }))).toBe("worker");
    expect(solveRefusalPoint(refusedAtTakeSolveJob(job, new ResourceUnavailableError(headroom, null)))).toBe("take");
    expect(solveRefusalPoint(job)).toBeNull();
    expect(describeSolveRefusalRetry(headroom, "before_start")).toBe("Refused before the solve started; no job ran. Available memory was re-sampled for up to 2 s before this refusal stood. You can run the solve again when more memory is available.");
    expect(describeSolveRefusalRetry({ kind: "queue_full", queued: 4, limit: 4 }, "before_start")).toBe("Refused before the solve started; no job ran. You can run the solve again once a queued solve has started.");
    expect(describeSolveRefusalRetry({ kind: "retention_full", retained: 4, limit: 4 }, "publication")).toBe("Refused at publication after the solve finished; the finished result was dropped. You can run the solve again once a waiting result has been collected or released.");
  });

  it.each([
    ["SOLVE_WORKER_EXITED: worker exited with signal 9", "The solve's worker process ended without returning a result. The solve failed; this is not a memory-pool refusal."],
    ["SOLVE_WORKER_PROTOCOL: unexpected frame", "The solve's worker process returned a reply the app could not read. The solve failed; this is not a memory-pool refusal."]
  ])("records a solve worker failure (%s) as a failure, not a refusal", (message, sentence) => {
    const failed = failSolveJob(startSolveJob(model, receipt), new Error(message));
    expect(failed.state).toBe("failed");
    expect(failed.resource.refusal).toBeNull();
    expect(failed.error_message).toBe(`Error: ${message}`);
    expect(failed.events.at(-1)).toMatchObject({ event_id: "solve-preview-failed", state: "failed" });
    expect(failed.events.at(-1)!.message).toBe(`Preview mechanics failed: Error: ${message} (${sentence})`);
  });

  it("records a start refusal with the budget and pool the native pool status reported", () => {
    const pool = { invocation_budget_bytes: 8 * GiB, pool_bytes: 24 * GiB } as PoolStatus;
    const job = refusedBeforeBackendStartSolveJob(model, new ResourceUnavailableError({ kind: "pressure_elevated", level: "warning" }, pool));
    expect(job.state).toBe("refused");
    expect(job.backend_job_id).toBeNull();
    expect(job.resource).toEqual({ source: "native_refusal_pool_status", invocation_budget_bytes: 8 * GiB, pool_bytes: 24 * GiB, route_pricing: null, ingress_reserved_bytes: null, worker_grant_bytes: null, worker_peak_grant_bytes: null, worker_peak_footprint_bytes: null, queue_reason: null, refusal: { kind: "pressure_elevated", level: "warning" } });
    expect(job.events.map((event) => event.event_id)).toEqual(["solve-preview-resource-refused"]);
    expect(job.events[0].message).toContain("macOS reports memory pressure (warning)");
  });
});

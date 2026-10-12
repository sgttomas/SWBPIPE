import { describe, expect, it } from "vitest";
import { ResourceUnavailableError } from "../../services/memoryPool";
import type { BackendSolveJobStatus, SolveJobStartReceipt } from "../../services/previewService";
import type { PoolStatus, PreviewModel } from "../../types";
import {
  failSolveJob,
  initialSolveJob,
  progressSolveJob,
  refusedBeforeBackendStartSolveJob,
  refusedSolveJob,
  requestSolveCancellation,
  startSolveJob
} from "./solveJobAudit";

// Pure audit transitions over invented native job reports (DEL-04-07).
const GiB = 1024 ** 3;
const model = {
  project: { id: "project:invented-memory-audit" },
  diagnostics: [],
  analysis_status: { mechanics: "MODEL_INCOMPLETE", rule_check: "RULE_INPUTS_INCOMPLETE" }
} as unknown as PreviewModel;
const receipt: SolveJobStartReceipt = {
  mode: "backend_job",
  job_id: "backend-solve-job-7",
  backend_cancellation_token: "token-7",
  state: "queued",
  cancellation_scope: "cooperative_checkpoints_not_preemptive",
  resource: { invocation_budget_bytes: 12 * GiB, pool_bytes: 48 * GiB, route_pricing: "ordinary_unpriced", ingress_reserved_bytes: 3 * 1024 ** 2 }
};
function status(overrides: Partial<BackendSolveJobStatus>): BackendSolveJobStatus {
  return {
    job_id: "backend-solve-job-7", state: "queued", queue_reason: null, cancellation_requested: false,
    cancellation_status: "not_requested", cancellation_scope: "cooperative_checkpoints_not_preemptive",
    result_available: false, result_bytes: null, error_message: null, resource_refusal: null, resource: receipt.mode === "backend_job" ? receipt.resource : null,
    ...overrides
  };
}

describe("solve-job audit resource record", () => {
  it("starts with no reported resource", () => {
    expect(initialSolveJob().resource).toEqual({ source: "not_reported", invocation_budget_bytes: null, pool_bytes: null, route_pricing: null, ingress_reserved_bytes: null, queue_reason: null, refusal: null });
  });

  it("records the native grant: per-invocation budget, pool and the unpriced ordinary route", () => {
    const job = startSolveJob(model, receipt);
    expect(job.state).toBe("queued");
    expect(job.resource).toEqual({ source: "native_job_receipt", invocation_budget_bytes: 12 * GiB, pool_bytes: 48 * GiB, route_pricing: "ordinary_unpriced", ingress_reserved_bytes: 3 * 1024 ** 2, queue_reason: null, refusal: null });
    expect(job.events.at(-1)!.message).toContain("per-invocation budget 12 GiB (bounds model intake and result custody); pool 48 GiB; model intake reserved 3 MiB; route ordinary_unpriced (no byte bound is claimed for the solve itself)");
    // A queued job can be cancelled.
    expect(requestSolveCancellation(job).state).toBe("cancelling");
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
    expect(progressSolveJob(running, status({ job_id: "other", state: "queued", queue_reason: { kind: "solve_slot_busy" } }))).toBe(running);
  });

  it("records a terminal refusal as a typed refusal, not a generic error", () => {
    const refusal = { kind: "headroom_insufficient" as const, required_bytes: 12 * GiB, available_bytes: 5 * GiB };
    const refused = refusedSolveJob(startSolveJob(model, receipt), refusal);
    expect(refused.state).toBe("refused");
    expect(refused.error_message).toBeNull();
    expect(refused.resource).toMatchObject({ source: "native_job_receipt", invocation_budget_bytes: 12 * GiB, pool_bytes: 48 * GiB, route_pricing: "ordinary_unpriced", refusal });
    expect(refused.events.at(-1)).toMatchObject({ event_id: "solve-preview-resource-refused", state: "refused", result_available: false });
    expect(refused.events.at(-1)!.message).toBe("Refused by the native memory pool (headroom_insufficient): Not enough free memory on this Mac right now: needs 12 GiB, 5 GiB available. No result was published.");
    // A ResourceUnavailableError reaching the failure path is recorded the same way.
    expect(failSolveJob(startSolveJob(model, receipt), new ResourceUnavailableError(refusal, null)).state).toBe("refused");
    expect(failSolveJob(startSolveJob(model, receipt), new Error("ordinary failure")).state).toBe("failed");
  });

  it("records a start refusal with the budget and pool the native pool status reported", () => {
    const pool = { invocation_budget_bytes: 8 * GiB, pool_bytes: 24 * GiB } as PoolStatus;
    const job = refusedBeforeBackendStartSolveJob(model, new ResourceUnavailableError({ kind: "pressure_elevated", level: "warning" }, pool));
    expect(job.state).toBe("refused");
    expect(job.backend_job_id).toBeNull();
    expect(job.resource).toEqual({ source: "native_refusal_pool_status", invocation_budget_bytes: 8 * GiB, pool_bytes: 24 * GiB, route_pricing: null, ingress_reserved_bytes: null, queue_reason: null, refusal: { kind: "pressure_elevated", level: "warning" } });
    expect(job.events.map((event) => event.event_id)).toEqual(["solve-preview-resource-refused"]);
    expect(job.events[0].message).toContain("macOS reports memory pressure (warning)");
  });
});

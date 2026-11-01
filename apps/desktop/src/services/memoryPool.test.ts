import { describe, expect, it } from "vitest";
import {
  describeResourceRefusal,
  describeSolveWorkerFailure,
  parsePoolStatus,
  parseResourceRefusal,
  parseSolveResourceGrant,
  resourceRefusalCode
} from "./memoryPool";

// Strict readers and plain-language sentences for the worker-capped ordinary
// route (DEL-04-07). Invented native reports; no figure is a machine reading.
const GiB = 1024 ** 3;

describe("worker refusals", () => {
  it("reads the three worker refusal kinds with only their declared fields", () => {
    expect(parseResourceRefusal({ kind: "worker_cap_exceeded", cap_bytes: 12 * GiB, requested_bytes: 13 * GiB, extra: 1 }))
      .toEqual({ kind: "worker_cap_exceeded", cap_bytes: 12 * GiB, requested_bytes: 13 * GiB });
    expect(parseResourceRefusal({ kind: "worker_footprint_exceeded", grant_bytes: 4 * GiB, footprint_bytes: 5 * GiB }))
      .toEqual({ kind: "worker_footprint_exceeded", grant_bytes: 4 * GiB, footprint_bytes: 5 * GiB });
    expect(parseResourceRefusal({ kind: "worker_growth_refused", granted_bytes: 2 * GiB, requested_bytes: 3 * GiB, cause: { kind: "headroom_stale", age_ms: 2500, extra: true } }))
      .toEqual({ kind: "worker_growth_refused", granted_bytes: 2 * GiB, requested_bytes: 3 * GiB, cause: { kind: "headroom_stale", age_ms: 2500 } });
  });

  it("reads a growth refusal only when its nested cause is itself a readable refusal", () => {
    const growth = (cause: unknown) => ({ kind: "worker_growth_refused", granted_bytes: GiB, requested_bytes: 2 * GiB, cause });
    expect(parseResourceRefusal(growth(undefined))).toBeNull();
    expect(parseResourceRefusal(growth({ kind: "solve_slot_busy" }))).toBeNull();
    expect(parseResourceRefusal(growth({ kind: "pool_exhausted", requested_bytes: "1" }))).toBeNull();
    // A cause can itself be a worker refusal, up to a bounded depth.
    expect(parseResourceRefusal(growth(growth({ kind: "capacity_unknown" })))).toEqual(growth(growth({ kind: "capacity_unknown" })));
    let deep: unknown = { kind: "capacity_unknown" };
    for (let level = 0; level < 6; level += 1) deep = growth(deep);
    expect(parseResourceRefusal(deep)).toBeNull();
  });

  it("reads the running-worker limit as a queue reason with only its declared fields", () => {
    expect(parseResourceRefusal({ kind: "worker_limit_reached", running: 8, limit: 8, extra: 0 })).toEqual({ kind: "worker_limit_reached", running: 8, limit: 8 });
    expect(parseResourceRefusal({ kind: "worker_limit_reached", running: 8 })).toBeNull();
    expect(parseResourceRefusal({ kind: "worker_limit_reached", running: "8", limit: 8 })).toBeNull();
  });

  it("refuses malformed worker refusals and the retired solve_slot_busy kind", () => {
    expect(parseResourceRefusal({ kind: "solve_slot_busy" })).toBeNull();
    expect(parseResourceRefusal({ kind: "worker_cap_exceeded", cap_bytes: 12 * GiB })).toBeNull();
    expect(parseResourceRefusal({ kind: "worker_cap_exceeded", cap_bytes: -1, requested_bytes: 1 })).toBeNull();
    expect(parseResourceRefusal({ kind: "worker_footprint_exceeded", grant_bytes: 1.5, footprint_bytes: 2 })).toBeNull();
    expect(parseResourceRefusal({ kind: "worker_growth_refused", granted_bytes: 1, cause: { kind: "capacity_unknown" } })).toBeNull();
  });

  it("states each worker refusal in plain language, with the nested cause's own sentence", () => {
    expect(describeResourceRefusal({ kind: "worker_cap_exceeded", cap_bytes: 12 * GiB, requested_bytes: 12.5 * GiB }))
      .toBe("The solve needed more memory than the per-invocation budget of 12 GiB allows (requested 12.5 GiB). The memory the solve needs most likely comes from the volume of per-station result detail for this model and its load cases. A solve covers every load case and combination in the model, so to need less: remove the load cases and combinations this review does not need (Load Cases section); or solve the cases in smaller groups, each in its own copy of the model document (File > Save Model Document…), with the combinations in a copy holding only the cases they use; or split a large model into parts.");
    expect(describeResourceRefusal({
      kind: "worker_growth_refused", granted_bytes: 3 * GiB, requested_bytes: 5 * GiB,
      cause: { kind: "headroom_insufficient", required_bytes: 2 * GiB, available_bytes: 512 * 1024 ** 2 }
    })).toBe("The solve needed more memory (5 GiB) than could be reserved while it ran: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 2 GiB, 512 MiB available. The memory the solve needs most likely comes from the volume of per-station result detail for this model and its load cases. A solve covers every load case and combination in the model, so to need less: remove the load cases and combinations this review does not need (Load Cases section); or solve the cases in smaller groups, each in its own copy of the model document (File > Save Model Document…), with the combinations in a copy holding only the cases they use; or split a large model into parts.");
    expect(describeResourceRefusal({ kind: "worker_footprint_exceeded", grant_bytes: 3 * GiB, footprint_bytes: 12.25 * GiB }))
      .toBe("The solve's measured memory footprint (12.25 GiB) does not fit the per-invocation budget, so the solve was ended (its reservation was 3 GiB). The memory the solve needs most likely comes from the volume of per-station result detail for this model and its load cases. A solve covers every load case and combination in the model, so to need less: remove the load cases and combinations this review does not need (Load Cases section); or solve the cases in smaller groups, each in its own copy of the model document (File > Save Model Document…), with the combinations in a copy holding only the cases they use; or split a large model into parts.");
    expect(describeResourceRefusal({ kind: "budget_exceeds_pool", budget_bytes: 12 * GiB, pool_bytes: 8 * GiB }))
      .toBe("The per-invocation budget of 12 GiB is larger than the 8 GiB pool.");
    expect(describeResourceRefusal({ kind: "worker_limit_reached", running: 1, limit: 1 }))
      .toBe("1 solve worker is already running (limit 1); this solve waits until one finishes.");
    expect(describeResourceRefusal({ kind: "worker_limit_reached", running: 10, limit: 10 }))
      .toBe("10 solve workers are already running (limit 10); this solve waits until one finishes.");
  });
});

describe("solve worker failures", () => {
  it("reads crashed and protocol-breaking workers as failures, and nothing else", () => {
    expect(describeSolveWorkerFailure("SOLVE_WORKER_EXITED: worker ended with status 137")).toBe(
      "The solve's worker process ended without returning a result. The solve failed; this is not a memory-pool refusal."
    );
    expect(describeSolveWorkerFailure("Error: SOLVE_WORKER_PROTOCOL: truncated reply")).toBe(
      "The solve's worker process returned a reply the app could not read. The solve failed; this is not a memory-pool refusal."
    );
    expect(describeSolveWorkerFailure("RESOURCE_UNAVAILABLE: SOLVE_WORKER")).toBeNull();
    expect(describeSolveWorkerFailure("XSOLVE_WORKER_EXITED: not this code")).toBeNull();
    expect(describeSolveWorkerFailure(null)).toBeNull();
  });
});

describe("the worker-capped grant", () => {
  const grant = (overrides: Record<string, unknown> = {}) => ({
    invocation_budget_bytes: 12 * GiB, pool_bytes: 48 * GiB, route_pricing: "ordinary_worker_capped", ingress_reserved_bytes: 4096,
    worker_grant_bytes: 2 * GiB, worker_peak_grant_bytes: 3 * GiB, worker_peak_footprint_bytes: 2.5 * GiB, ...overrides
  });

  it("reads the grant with the worker's current and peak grant", () => {
    expect(parseSolveResourceGrant(grant({ extra: 1 }))).toEqual(grant());
    expect(parseSolveResourceGrant(grant({ worker_grant_bytes: null, worker_peak_grant_bytes: null })))
      .toMatchObject({ worker_grant_bytes: null, worker_peak_grant_bytes: null });
    // A worker that has exited holds no grant but keeps its peak.
    expect(parseSolveResourceGrant(grant({ worker_grant_bytes: null }))).toMatchObject({ worker_grant_bytes: null, worker_peak_grant_bytes: 3 * GiB });
    expect(parseSolveResourceGrant(grant({ worker_peak_grant_bytes: 12 * GiB }))).toMatchObject({ worker_peak_grant_bytes: 12 * GiB });
  });

  it("reads any whole peak footprint, including one above the peak grant or the budget", () => {
    expect(parseSolveResourceGrant(grant({ worker_peak_footprint_bytes: null }))).toMatchObject({ worker_peak_footprint_bytes: null });
    // Between growth exchanges the measured footprint can briefly exceed the peak grant.
    expect(parseSolveResourceGrant(grant({ worker_peak_footprint_bytes: 3.5 * GiB }))).toMatchObject({ worker_peak_grant_bytes: 3 * GiB, worker_peak_footprint_bytes: 3.5 * GiB });
    // A footprint above the budget ends the job refused; the reader still records the figure.
    expect(parseSolveResourceGrant(grant({ worker_peak_footprint_bytes: 13 * GiB }))).toMatchObject({ worker_peak_footprint_bytes: 13 * GiB });
  });

  it.each([
    ["the interim unpriced route", { route_pricing: "ordinary_unpriced" }],
    ["a missing current grant", { worker_grant_bytes: undefined }],
    ["a missing peak grant", { worker_peak_grant_bytes: undefined }],
    ["a missing peak footprint", { worker_peak_footprint_bytes: undefined }],
    ["a negative peak footprint", { worker_peak_footprint_bytes: -1 }],
    ["a fractional peak footprint", { worker_peak_footprint_bytes: 0.5 }],
    ["a current grant without a peak", { worker_peak_grant_bytes: null }],
    ["a current grant above its peak", { worker_grant_bytes: 4 * GiB }],
    ["a peak above the per-invocation budget", { worker_peak_grant_bytes: 13 * GiB }],
    ["a fractional grant", { worker_grant_bytes: 1.5 }]
  ])("refuses a grant with %s", (_label, overrides) => {
    const value: Record<string, unknown> = grant(overrides);
    for (const key of Object.keys(overrides)) if (overrides[key as keyof typeof overrides] === undefined) delete value[key];
    expect(parseSolveResourceGrant(value)).toBeNull();
  });
});

describe("the pool status with solve workers", () => {
  const status = (overrides: Record<string, unknown> = {}) => ({
    capacity_bytes: 16 * GiB, recommended_pool_bytes: 12 * GiB, pool_bytes: 12 * GiB, pool_above_recommended: false,
    invocation_budget_bytes: 12 * GiB, default_invocation_budget_bytes: 12 * GiB, max_invocation_budget_bytes: 12 * GiB, reserved_bytes: 5 * GiB,
    reserved_by_purpose: { ingress: GiB, worker: 4 * GiB, result: 0, custody: 0 }, running_workers: 2, queued_jobs: 0, running_jobs: 2,
    limits: { max_queued_jobs: 4, max_retained_results: 4, max_webview_custody_results: 4, max_terminal_records: 64, max_running_workers: 8 },
    retained_results: 0, webview_custody_results: 0, headroom: { available_bytes: 6 * GiB, pressure: "normal", sample_age_ms: 10 },
    ordinary_route_pricing: "worker_capped",
    last_refusal: { kind: "worker_growth_refused", granted_bytes: GiB, requested_bytes: 2 * GiB, cause: { kind: "pressure_unknown" } },
    ...overrides
  });

  it("reads running workers, worker reservations and a nested last refusal", () => {
    const read = parsePoolStatus(status());
    expect(read).toMatchObject({ running_workers: 2, limits: { max_running_workers: 8 }, reserved_by_purpose: { ingress: GiB, worker: 4 * GiB, result: 0, custody: 0 }, ordinary_route_pricing: "worker_capped" });
    expect(read!.last_refusal).toEqual({ kind: "worker_growth_refused", granted_bytes: GiB, requested_bytes: 2 * GiB, cause: { kind: "pressure_unknown" } });
    expect(read).not.toHaveProperty("unpriced_solve_running");
  });

  it.each([
    ["the interim unpriced pricing", { ordinary_route_pricing: "unpriced" }],
    ["no running worker count", { running_workers: undefined }],
    ["no worker reservation", { reserved_by_purpose: { ingress: 0, result: 0, custody: 0 } }],
    ["no running-worker limit", { limits: { max_queued_jobs: 4 } }],
    ["no limits", { limits: undefined }],
    ["an unreadable last refusal", { last_refusal: { kind: "solve_slot_busy" } }],
    ["no default budget", { default_invocation_budget_bytes: undefined }],
    ["no largest budget", { max_invocation_budget_bytes: undefined }],
    ["the retired qualified budget in place of the default and largest budgets", { default_invocation_budget_bytes: undefined, max_invocation_budget_bytes: undefined, qualified_invocation_budget_bytes: 12 * GiB }],
    ["a retired budget_unqualified last refusal", { last_refusal: { kind: "budget_unqualified", requested_bytes: 16 * GiB, qualified_bytes: 12 * GiB } }]
  ])("refuses a status with %s", (_label, overrides) => {
    const value: Record<string, unknown> = status(overrides);
    for (const key of Object.keys(overrides)) if (overrides[key as keyof typeof overrides] === undefined) delete value[key];
    expect(parsePoolStatus(value)).toBeNull();
  });
});

describe("budgets above the default", () => {
  it("reads the default and the largest per-invocation budget, and a selected budget above 12 GiB", () => {
    const read = parsePoolStatus({
      capacity_bytes: 64 * GiB, recommended_pool_bytes: 48 * GiB, pool_bytes: 48 * GiB, pool_above_recommended: false,
      invocation_budget_bytes: 40 * GiB, default_invocation_budget_bytes: 12 * GiB, max_invocation_budget_bytes: 48 * GiB, reserved_bytes: 0,
      reserved_by_purpose: { ingress: 0, worker: 0, result: 0, custody: 0 }, running_workers: 0, queued_jobs: 0, running_jobs: 0,
      limits: { max_running_workers: 8 }, retained_results: 0, webview_custody_results: 0,
      headroom: { available_bytes: 30 * GiB, pressure: "warning", sample_age_ms: 10 }, ordinary_route_pricing: "worker_capped", last_refusal: null
    });
    expect(read).toMatchObject({ invocation_budget_bytes: 40 * GiB, default_invocation_budget_bytes: 12 * GiB, max_invocation_budget_bytes: 48 * GiB });
    expect(read).not.toHaveProperty("qualified_invocation_budget_bytes");
  });

  it("no longer reads the retired budget_unqualified refusal", () => {
    expect(parseResourceRefusal({ kind: "budget_unqualified", requested_bytes: 16 * GiB, qualified_bytes: 12 * GiB })).toBeNull();
  });

  it("states capacity, headroom and intake refusals without naming a solve, since edits and saves meet them too", () => {
    expect(describeResourceRefusal({ kind: "capacity_unknown" })).toBe("This Mac's physical memory could not be read, so no new memory reservation is admitted.");
    expect(describeResourceRefusal({ kind: "headroom_unknown" })).toBe("Available memory on this Mac (free plus reclaimable file cache and purgeable memory) could not be read right now, so no new memory reservation is admitted.");
    expect(describeResourceRefusal({ kind: "headroom_stale", age_ms: 2500 })).toBe("The available-memory reading is out of date (2.5 s old), so no new memory reservation is admitted until it refreshes.");
    expect(describeResourceRefusal({ kind: "ingress_exceeds_budget", required_bytes: 13 * GiB, budget_bytes: 12 * GiB })).toBe("Receiving the request needs 13 GiB, more than the per-invocation budget of 12 GiB.");
    expect(describeResourceRefusal({ kind: "ingress_invalid", reason: "the body is not valid JSON" })).toBe("The request could not be received: the body is not valid JSON.");
  });
});

describe("the exact refusal code", () => {
  it("names the code and every field unrounded, a growth refusal's cause the same way", () => {
    expect(resourceRefusalCode({ kind: "headroom_insufficient", required_bytes: 68165632, available_bytes: 52641792 }))
      .toBe("headroom_insufficient (required_bytes=68165632, available_bytes=52641792)");
    expect(resourceRefusalCode({ kind: "worker_growth_refused", granted_bytes: 68157440, requested_bytes: 72598008,
      cause: { kind: "headroom_insufficient", required_bytes: 67108864, available_bytes: 0 } }))
      .toBe("worker_growth_refused (granted_bytes=68157440, requested_bytes=72598008); cause headroom_insufficient (required_bytes=67108864, available_bytes=0)");
    expect(resourceRefusalCode({ kind: "pressure_elevated", level: "warning" })).toBe("pressure_elevated (level=warning)");
    expect(resourceRefusalCode({ kind: "headroom_unknown" })).toBe("headroom_unknown");
  });
});

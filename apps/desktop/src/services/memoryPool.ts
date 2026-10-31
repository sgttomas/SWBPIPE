// The native memory pool as the desktop sees it (DEL-04-07 memory-budget
// decision): the typed resource refusal, the pool status and the per-job
// resource grant the native host reports, their plain-language reading, and
// the two pool commands. The native host owns the ledger and every admission
// verdict; nothing here computes, raises or bypasses a limit. Budget policy
// stays outside the model document.
import { invoke } from "@tauri-apps/api/core";
import type { MemoryPressureLevel, PoolStatus, ResourceRefusal, SolveResourceGrant } from "../types";

export const GIB = 1024 ** 3;
export const MIB = 1024 ** 2;

/** Bytes as GiB (from 1 GiB) or MiB, two decimals at most; "unknown" when not reported. */
export function formatBytes(bytes: number | null | undefined): string {
  if (typeof bytes !== "number" || !Number.isFinite(bytes)) return "unknown";
  const [value, unit] = Math.abs(bytes) >= GIB ? [bytes / GIB, "GiB"] : [bytes / MIB, "MiB"];
  const text = Number.isInteger(value) ? String(value) : value.toFixed(2).replace(/0+$/, "").replace(/\.$/, "");
  return `${text} ${unit}`;
}

function formatAge(ms: number): string {
  return ms < 1000 ? `${Math.round(ms)} ms` : `${(ms / 1000).toFixed(1).replace(/\.0$/, "")} s`;
}

const PRESSURE_LEVELS: readonly MemoryPressureLevel[] = ["normal", "warning", "critical", "unknown"];

/** What a person can reduce or split today so a solve publishes less result
 * detail: a solve covers every load case and combination in the model. */
export const SOLVE_RESULT_REDUCTION_OPTIONS = "remove the load cases and combinations this review does not need (Load Cases section); or solve the cases in smaller groups, each in its own copy of the model document (File > Save Model Document…), with the combinations in a copy holding only the cases they use; or split a large model into parts.";

/** Appended to a solve worker's memory refusals: the likely cause and what the
 * person can do now. The refusal's figures come first. */
export const SOLVE_WORKER_MEMORY_ADVICE = `The memory the solve needs most likely comes from the volume of per-station result detail for this model and its load cases. A solve covers every load case and combination in the model, so to need less: ${SOLVE_RESULT_REDUCTION_OPTIONS}`;

/** Plain-language reason for a native resource refusal. It states what the
 * native host reported; it never promises that a retry or another setting will succeed. */
export function describeResourceRefusal(refusal: ResourceRefusal): string {
  switch (refusal.kind) {
    case "capacity_unknown":
      return "This Mac's physical memory could not be read, so no new memory reservation is admitted.";
    case "headroom_unknown":
      return "Available memory on this Mac (free plus reclaimable file cache and purgeable memory) could not be read right now, so no new memory reservation is admitted.";
    case "headroom_stale":
      return `The available-memory reading is out of date (${formatAge(refusal.age_ms)} old), so no new memory reservation is admitted until it refreshes.`;
    case "headroom_insufficient":
      return `Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs ${formatBytes(refusal.required_bytes)}, ${formatBytes(refusal.available_bytes)} available.`;
    case "pressure_unknown":
      return "macOS did not report a memory-pressure state, so no new solve is admitted.";
    case "pressure_elevated":
      return `macOS reports memory pressure (${refusal.level}), so no new solve is admitted until it returns to normal.`;
    case "pool_exhausted":
      return `The memory pool is fully reserved: this request needs ${formatBytes(refusal.requested_bytes)}, and ${formatBytes(refusal.reserved_bytes)} of the ${formatBytes(refusal.pool_bytes)} pool is already reserved.`;
    case "budget_exceeds_pool":
      return `The per-invocation budget of ${formatBytes(refusal.budget_bytes)} is larger than the ${formatBytes(refusal.pool_bytes)} pool.`;
    case "pool_above_capacity":
      return `A pool of ${formatBytes(refusal.pool_bytes)} is larger than this Mac's physical memory (${formatBytes(refusal.capacity_bytes)}).`;
    case "pool_above_recommended_unacknowledged":
      return `A pool of ${formatBytes(refusal.pool_bytes)} is above the recommended ${formatBytes(refusal.recommended_bytes)} for this Mac; the request needs an explicit acknowledgement.`;
    case "ingress_exceeds_budget":
      return `Receiving the request needs ${formatBytes(refusal.required_bytes)}, more than the per-invocation budget of ${formatBytes(refusal.budget_bytes)}.`;
    case "ingress_invalid":
      return `The request could not be received: ${refusal.reason}.`;
    case "queue_full":
      return `The solve queue is full (${refusal.queued} of ${refusal.limit} waiting).`;
    case "retention_full":
      return `Too many completed results are waiting to be collected (${refusal.retained} of ${refusal.limit}).`;
    case "custody_full":
      return `The app already holds the maximum number of delivered results (${refusal.delivered} of ${refusal.limit}); a held result must be released first.`;
    case "worker_cap_exceeded":
      return `The solve needed more memory than the per-invocation budget of ${formatBytes(refusal.cap_bytes)} allows (requested ${formatBytes(refusal.requested_bytes)}). ${SOLVE_WORKER_MEMORY_ADVICE}`;
    case "worker_growth_refused":
      return `The solve needed more memory (${formatBytes(refusal.requested_bytes)}) than could be reserved while it ran: ${describeResourceRefusal(refusal.cause)} ${SOLVE_WORKER_MEMORY_ADVICE}`;
    case "worker_footprint_exceeded":
      return `The solve's measured memory footprint (${formatBytes(refusal.footprint_bytes)}) does not fit the per-invocation budget, so the solve was ended (its reservation was ${formatBytes(refusal.grant_bytes)}). ${SOLVE_WORKER_MEMORY_ADVICE}`;
    case "worker_limit_reached":
      return `${refusal.running} solve ${refusal.running === 1 ? "worker is" : "workers are"} already running (limit ${refusal.limit}); this solve waits until one finishes.`;
  }
}

/** The typed refusal exactly as the native host reported it: its code and each
 * of its fields (bytes unrounded), then a growth refusal's cause the same way.
 * For example `headroom_insufficient (required_bytes=68165632, available_bytes=52641792)`. */
export function resourceRefusalCode(refusal: ResourceRefusal): string {
  const fields = Object.entries(refusal).filter(([key]) => key !== "kind" && key !== "cause").map(([key, value]) => `${key}=${String(value)}`);
  const code = fields.length > 0 ? `${refusal.kind} (${fields.join(", ")})` : refusal.kind;
  return refusal.kind === "worker_growth_refused" ? `${code}; cause ${resourceRefusalCode(refusal.cause)}` : code;
}

/** Plain-language reading of a solve that failed because its worker process
 * crashed or broke the worker protocol (native `error_message` prefixed
 * `SOLVE_WORKER_EXITED:` or `SOLVE_WORKER_PROTOCOL:`); null for any other
 * failure. Such a job failed: it is not a memory-pool refusal. */
export function describeSolveWorkerFailure(message: string | null | undefined): string | null {
  const code = typeof message === "string" ? /\bSOLVE_WORKER_(EXITED|PROTOCOL):/.exec(message)?.[1] : undefined;
  if (code === "EXITED") return "The solve's worker process ended without returning a result. The solve failed; this is not a memory-pool refusal.";
  if (code === "PROTOCOL") return "The solve's worker process returned a reply the app could not read. The solve failed; this is not a memory-pool refusal.";
  return null;
}

type Fields = Record<string, unknown>;
const isCount = (value: unknown): value is number => typeof value === "number" && Number.isSafeInteger(value) && value >= 0;
const isRecord = (value: unknown): value is Fields => !!value && typeof value === "object" && !Array.isArray(value);
const countOrNull = (value: unknown): value is number | null => value === null || isCount(value);

/** How deeply a growth refusal's cause may nest; a deeper report is not readable. */
const MAX_REFUSAL_CAUSE_DEPTH = 4;

/** The typed refusal as reported, with only its declared fields; null for any other shape. */
export function parseResourceRefusal(value: unknown, depth = 0): ResourceRefusal | null {
  if (!isRecord(value) || typeof value.kind !== "string") return null;
  const counts = (...keys: string[]) => keys.every((key) => isCount(value[key]));
  const pick = <K extends string>(...keys: K[]) => Object.fromEntries(keys.map((key) => [key, value[key]])) as Record<K, number>;
  const kind = value.kind;
  switch (kind) {
    case "capacity_unknown":
    case "headroom_unknown":
    case "pressure_unknown":
      return { kind };
    case "headroom_stale":
      return counts("age_ms") ? { kind, ...pick("age_ms") } : null;
    case "headroom_insufficient":
      return counts("required_bytes", "available_bytes") ? { kind, ...pick("required_bytes", "available_bytes") } : null;
    case "pressure_elevated":
      // Elevated pressure is warning or critical; an unreported state is its own kind.
      return value.level === "warning" || value.level === "critical" ? { kind, level: value.level } : null;
    case "pool_exhausted":
      return counts("requested_bytes", "reserved_bytes", "pool_bytes") ? { kind, ...pick("requested_bytes", "reserved_bytes", "pool_bytes") } : null;
    case "budget_exceeds_pool":
      return counts("budget_bytes", "pool_bytes") ? { kind, ...pick("budget_bytes", "pool_bytes") } : null;
    case "pool_above_capacity":
      return counts("pool_bytes", "capacity_bytes") ? { kind, ...pick("pool_bytes", "capacity_bytes") } : null;
    case "pool_above_recommended_unacknowledged":
      return counts("pool_bytes", "recommended_bytes") ? { kind, ...pick("pool_bytes", "recommended_bytes") } : null;
    case "ingress_exceeds_budget":
      return counts("required_bytes", "budget_bytes") ? { kind, ...pick("required_bytes", "budget_bytes") } : null;
    case "ingress_invalid":
      return typeof value.reason === "string" ? { kind, reason: value.reason } : null;
    case "queue_full":
      return counts("queued", "limit") ? { kind, ...pick("queued", "limit") } : null;
    case "retention_full":
      return counts("retained", "limit") ? { kind, ...pick("retained", "limit") } : null;
    case "custody_full":
      return counts("delivered", "limit") ? { kind, ...pick("delivered", "limit") } : null;
    case "worker_cap_exceeded":
      return counts("cap_bytes", "requested_bytes") ? { kind, ...pick("cap_bytes", "requested_bytes") } : null;
    case "worker_growth_refused": {
      // The cause is the pool's own verdict on the growth: it must itself be readable.
      if (!counts("granted_bytes", "requested_bytes") || depth >= MAX_REFUSAL_CAUSE_DEPTH) return null;
      const cause = parseResourceRefusal(value.cause, depth + 1);
      return cause ? { kind, ...pick("granted_bytes", "requested_bytes"), cause } : null;
    }
    case "worker_footprint_exceeded":
      return counts("grant_bytes", "footprint_bytes") ? { kind, ...pick("grant_bytes", "footprint_bytes") } : null;
    case "worker_limit_reached":
      return counts("running", "limit") ? { kind, ...pick("running", "limit") } : null;
    default:
      return null;
  }
}

/** The per-job resource grant as reported; null for any other shape. The
 * worker's current and peak grant and its peak footprint are all present (null
 * before the worker holds a grant or is measured); a current grant needs a peak
 * at least as large, and the peak grant never exceeds the per-invocation budget
 * that caps the worker. The peak footprint is any whole byte count: it can
 * briefly exceed the peak grant between growth exchanges. */
export function parseSolveResourceGrant(value: unknown): SolveResourceGrant | null {
  if (!isRecord(value) || value.route_pricing !== "ordinary_worker_capped") return null;
  if (![value.invocation_budget_bytes, value.pool_bytes, value.ingress_reserved_bytes].every(isCount)) return null;
  if (!["worker_grant_bytes", "worker_peak_grant_bytes", "worker_peak_footprint_bytes"].every((key) => key in value)) return null;
  const grant = value.worker_grant_bytes, peak = value.worker_peak_grant_bytes, footprint = value.worker_peak_footprint_bytes;
  if (!countOrNull(grant) || !countOrNull(peak) || !countOrNull(footprint)) return null;
  if (grant !== null && (peak === null || grant > peak)) return null;
  if (peak !== null && peak > (value.invocation_budget_bytes as number)) return null;
  return {
    invocation_budget_bytes: value.invocation_budget_bytes as number,
    pool_bytes: value.pool_bytes as number,
    route_pricing: "ordinary_worker_capped",
    ingress_reserved_bytes: value.ingress_reserved_bytes as number,
    worker_grant_bytes: grant,
    worker_peak_grant_bytes: peak,
    worker_peak_footprint_bytes: footprint,
  };
}

/** The pool status as reported, with only its declared fields; null for any other shape. */
export function parsePoolStatus(value: unknown): PoolStatus | null {
  if (!isRecord(value) || !isRecord(value.reserved_by_purpose) || !isRecord(value.headroom) || !isRecord(value.limits)) return null;
  const purpose = value.reserved_by_purpose, headroom = value.headroom, limits = value.limits;
  const last = value.last_refusal === null ? null : parseResourceRefusal(value.last_refusal);
  if (
    !countOrNull(value.capacity_bytes) || !countOrNull(value.recommended_pool_bytes)
    || ![value.pool_bytes, value.invocation_budget_bytes, value.default_invocation_budget_bytes, value.max_invocation_budget_bytes, value.reserved_bytes,
      purpose.ingress, purpose.worker, purpose.result, purpose.custody, value.queued_jobs, value.running_jobs,
      value.running_workers, limits.max_running_workers, value.retained_results, value.webview_custody_results].every(isCount)
    || typeof value.pool_above_recommended !== "boolean"
    || !countOrNull(headroom.available_bytes) || !countOrNull(headroom.sample_age_ms)
    || !PRESSURE_LEVELS.includes(headroom.pressure as MemoryPressureLevel)
    || value.ordinary_route_pricing !== "worker_capped"
    || (value.last_refusal !== null && last === null)
  ) return null;
  return {
    capacity_bytes: value.capacity_bytes,
    recommended_pool_bytes: value.recommended_pool_bytes,
    pool_bytes: value.pool_bytes as number,
    pool_above_recommended: value.pool_above_recommended,
    invocation_budget_bytes: value.invocation_budget_bytes as number,
    default_invocation_budget_bytes: value.default_invocation_budget_bytes as number,
    max_invocation_budget_bytes: value.max_invocation_budget_bytes as number,
    reserved_bytes: value.reserved_bytes as number,
    reserved_by_purpose: { ingress: purpose.ingress as number, worker: purpose.worker as number, result: purpose.result as number, custody: purpose.custody as number },
    running_workers: value.running_workers as number,
    limits: { max_running_workers: limits.max_running_workers as number },
    queued_jobs: value.queued_jobs as number,
    running_jobs: value.running_jobs as number,
    retained_results: value.retained_results as number,
    webview_custody_results: value.webview_custody_results as number,
    headroom: { available_bytes: headroom.available_bytes, pressure: headroom.pressure as MemoryPressureLevel, sample_age_ms: headroom.sample_age_ms },
    ordinary_route_pricing: "worker_capped",
    last_refusal: last,
  };
}

/** A typed native resource refusal. The refusal is the native verdict; the
 * desktop shows it and records it, and never retries around it. */
export class ResourceUnavailableError extends Error {
  readonly code = "RESOURCE_UNAVAILABLE";
  constructor(readonly refusal: ResourceRefusal, readonly pool: PoolStatus | null) {
    super(`RESOURCE_UNAVAILABLE: ${describeResourceRefusal(refusal)}`);
    this.name = "ResourceUnavailableError";
  }
}

/** Reads a native command rejection: the typed resource refusal becomes a
 * `ResourceUnavailableError`, a typed invalid request an `INVALID_REQUEST` error;
 * any other rejection is returned unchanged. */
export function nativeInvokeError(error: unknown): unknown {
  if (!isRecord(error)) return error;
  if (error.code === "RESOURCE_UNAVAILABLE") {
    const refusal = parseResourceRefusal(error.refusal);
    if (!refusal) return new Error("RESOURCE_UNAVAILABLE: the native refusal could not be read.");
    return new ResourceUnavailableError(refusal, error.pool == null ? null : parsePoolStatus(error.pool));
  }
  if (error.code === "INVALID_REQUEST") {
    return new Error(`INVALID_REQUEST: ${typeof error.message === "string" ? error.message : "the native request was refused"}`);
  }
  return error;
}

export function isNativeMemoryPoolAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export const MEMORY_POOL_NATIVE_ONLY = "MEMORY_POOL_NATIVE_ONLY: the memory pool exists only in the native application; the browser preview reports no figures.";

export async function getMemoryPoolStatus(): Promise<PoolStatus> {
  if (!isNativeMemoryPoolAvailable()) throw new Error(MEMORY_POOL_NATIVE_ONLY);
  let reported: unknown;
  try { reported = await invoke<unknown>("get_memory_pool_status"); }
  catch (error) { throw nativeInvokeError(error); }
  const status = parsePoolStatus(reported);
  if (!status) throw new Error("MEMORY_POOL_STATUS_INVALID: the native pool status could not be read.");
  return status;
}

/** Requests a pool size and a per-invocation budget. The acknowledgement only
 * answers the above-recommended warning; the native host still refuses any
 * request that its capacity, headroom or pool checks do not admit. */
export async function configureMemoryPool(request: {
  poolBytes: number;
  invocationBudgetBytes: number;
  acknowledgeAboveRecommended: boolean;
}): Promise<PoolStatus> {
  if (!isNativeMemoryPoolAvailable()) throw new Error(MEMORY_POOL_NATIVE_ONLY);
  if (!isCount(request.poolBytes) || !isCount(request.invocationBudgetBytes) || request.poolBytes === 0 || request.invocationBudgetBytes === 0) {
    throw new Error("MEMORY_POOL_REQUEST_INVALID: the pool and the per-invocation budget are positive whole byte counts.");
  }
  let reported: unknown;
  try {
    reported = await invoke<unknown>("configure_memory_pool", {
      poolBytes: request.poolBytes,
      invocationBudgetBytes: request.invocationBudgetBytes,
      acknowledgeAboveRecommended: request.acknowledgeAboveRecommended,
    });
  } catch (error) { throw nativeInvokeError(error); }
  const status = parsePoolStatus(reported);
  if (!status) throw new Error("MEMORY_POOL_STATUS_INVALID: the native pool status could not be read.");
  return status;
}

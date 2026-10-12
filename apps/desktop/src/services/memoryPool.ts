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

/** Plain-language reason for a native resource refusal. It states what the
 * native host reported; it never promises that a retry or another setting will succeed. */
export function describeResourceRefusal(refusal: ResourceRefusal): string {
  switch (refusal.kind) {
    case "capacity_unknown":
      return "This Mac's physical memory could not be read, so no new solve is admitted.";
    case "headroom_unknown":
      return "Free memory on this Mac could not be read right now, so no new solve is admitted.";
    case "headroom_stale":
      return `The free-memory reading is out of date (${formatAge(refusal.age_ms)} old), so no new solve is admitted until it refreshes.`;
    case "headroom_insufficient":
      return `Not enough free memory on this Mac right now: needs ${formatBytes(refusal.required_bytes)}, ${formatBytes(refusal.available_bytes)} available.`;
    case "pressure_elevated":
      return `macOS reports memory pressure (${refusal.level}), so no new solve is admitted until it returns to normal.`;
    case "pool_exhausted":
      return `The memory pool is fully reserved: this request needs ${formatBytes(refusal.requested_bytes)}, and ${formatBytes(refusal.reserved_bytes)} of the ${formatBytes(refusal.pool_bytes)} pool is already reserved.`;
    case "budget_unqualified":
      return `A per-invocation budget of ${formatBytes(refusal.requested_bytes)} is above the qualified budget of ${formatBytes(refusal.qualified_bytes)}; only qualified budgets can be selected.`;
    case "budget_exceeds_pool":
      return `The per-invocation budget of ${formatBytes(refusal.budget_bytes)} is larger than the ${formatBytes(refusal.pool_bytes)} pool.`;
    case "pool_above_capacity":
      return `A pool of ${formatBytes(refusal.pool_bytes)} is larger than this Mac's physical memory (${formatBytes(refusal.capacity_bytes)}).`;
    case "pool_above_recommended_unacknowledged":
      return `A pool of ${formatBytes(refusal.pool_bytes)} is above the recommended ${formatBytes(refusal.recommended_bytes)} for this Mac; the request needs an explicit acknowledgement.`;
    case "ingress_exceeds_budget":
      return `Receiving the model document needs ${formatBytes(refusal.required_bytes)}, more than the per-invocation budget of ${formatBytes(refusal.budget_bytes)}.`;
    case "ingress_invalid":
      return `The model document could not be received: ${refusal.reason}.`;
    case "queue_full":
      return `The solve queue is full (${refusal.queued} of ${refusal.limit} waiting).`;
    case "retention_full":
      return `Too many completed results are waiting to be collected (${refusal.retained} of ${refusal.limit}).`;
    case "custody_full":
      return `The app already holds the maximum number of delivered results (${refusal.delivered} of ${refusal.limit}); a held result must be released first.`;
    case "solve_slot_busy":
      return "Another ordinary solve is running; ordinary solves are unpriced, so this one waits for it to finish.";
  }
}

type Fields = Record<string, unknown>;
const isCount = (value: unknown): value is number => typeof value === "number" && Number.isSafeInteger(value) && value >= 0;
const isRecord = (value: unknown): value is Fields => !!value && typeof value === "object" && !Array.isArray(value);
const countOrNull = (value: unknown): value is number | null => value === null || isCount(value);

/** The typed refusal as reported, with only its declared fields; null for any other shape. */
export function parseResourceRefusal(value: unknown): ResourceRefusal | null {
  if (!isRecord(value) || typeof value.kind !== "string") return null;
  const counts = (...keys: string[]) => keys.every((key) => isCount(value[key]));
  const pick = <K extends string>(...keys: K[]) => Object.fromEntries(keys.map((key) => [key, value[key]])) as Record<K, number>;
  const kind = value.kind;
  switch (kind) {
    case "capacity_unknown":
    case "headroom_unknown":
    case "solve_slot_busy":
      return { kind };
    case "headroom_stale":
      return counts("age_ms") ? { kind, ...pick("age_ms") } : null;
    case "headroom_insufficient":
      return counts("required_bytes", "available_bytes") ? { kind, ...pick("required_bytes", "available_bytes") } : null;
    case "pressure_elevated":
      return typeof value.level === "string" && value.level ? { kind, level: value.level } : null;
    case "pool_exhausted":
      return counts("requested_bytes", "reserved_bytes", "pool_bytes") ? { kind, ...pick("requested_bytes", "reserved_bytes", "pool_bytes") } : null;
    case "budget_unqualified":
      return counts("requested_bytes", "qualified_bytes") ? { kind, ...pick("requested_bytes", "qualified_bytes") } : null;
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
    default:
      return null;
  }
}

/** The per-job resource grant as reported; null for any other shape. */
export function parseSolveResourceGrant(value: unknown): SolveResourceGrant | null {
  if (!isRecord(value) || value.route_pricing !== "ordinary_unpriced") return null;
  if (![value.invocation_budget_bytes, value.pool_bytes, value.ingress_reserved_bytes].every(isCount)) return null;
  return {
    invocation_budget_bytes: value.invocation_budget_bytes as number,
    pool_bytes: value.pool_bytes as number,
    route_pricing: "ordinary_unpriced",
    ingress_reserved_bytes: value.ingress_reserved_bytes as number,
  };
}

/** The pool status as reported, with only its declared fields; null for any other shape. */
export function parsePoolStatus(value: unknown): PoolStatus | null {
  if (!isRecord(value) || !isRecord(value.reserved_by_purpose) || !isRecord(value.headroom)) return null;
  const purpose = value.reserved_by_purpose, headroom = value.headroom;
  const last = value.last_refusal === null ? null : parseResourceRefusal(value.last_refusal);
  if (
    !countOrNull(value.capacity_bytes) || !countOrNull(value.recommended_pool_bytes)
    || ![value.pool_bytes, value.invocation_budget_bytes, value.qualified_invocation_budget_bytes, value.reserved_bytes,
      purpose.ingress, purpose.result, purpose.custody, value.queued_jobs, value.running_jobs,
      value.retained_results, value.webview_custody_results].every(isCount)
    || typeof value.pool_above_recommended !== "boolean" || typeof value.unpriced_solve_running !== "boolean"
    || !countOrNull(headroom.available_bytes) || !countOrNull(headroom.sample_age_ms)
    || !PRESSURE_LEVELS.includes(headroom.pressure as MemoryPressureLevel)
    || value.ordinary_route_pricing !== "unpriced"
    || (value.last_refusal !== null && last === null)
  ) return null;
  return {
    capacity_bytes: value.capacity_bytes,
    recommended_pool_bytes: value.recommended_pool_bytes,
    pool_bytes: value.pool_bytes as number,
    pool_above_recommended: value.pool_above_recommended,
    invocation_budget_bytes: value.invocation_budget_bytes as number,
    qualified_invocation_budget_bytes: value.qualified_invocation_budget_bytes as number,
    reserved_bytes: value.reserved_bytes as number,
    reserved_by_purpose: { ingress: purpose.ingress as number, result: purpose.result as number, custody: purpose.custody as number },
    unpriced_solve_running: value.unpriced_solve_running,
    queued_jobs: value.queued_jobs as number,
    running_jobs: value.running_jobs as number,
    retained_results: value.retained_results as number,
    webview_custody_results: value.webview_custody_results as number,
    headroom: { available_bytes: headroom.available_bytes, pressure: headroom.pressure as MemoryPressureLevel, sample_age_ms: headroom.sample_age_ms },
    ordinary_route_pricing: "unpriced",
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
 * request that its capacity, headroom or qualification checks do not admit. */
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

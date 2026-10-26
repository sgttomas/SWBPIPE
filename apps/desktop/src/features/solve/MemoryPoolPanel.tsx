import { AlertTriangle, RefreshCw, SlidersHorizontal } from "lucide-react";
import { useCallback, useEffect, useRef, useState, type FormEvent } from "react";
import type { PoolStatus } from "../../types";
import {
  configureMemoryPool,
  describeResourceRefusal,
  formatBytes,
  getMemoryPoolStatus,
  GIB,
  isNativeMemoryPoolAvailable,
  ResourceUnavailableError
} from "../../services/memoryPool";

/** How often the pool is read while a solve is queued or running. */
export const MEMORY_POOL_ACTIVE_REFRESH_MS = 1000;

type Outcome = { tone: "applied" | "refused" | "error"; text: string };

/**
 * The native memory pool beside the solve controls (DEL-04-07 memory-budget
 * decision): the per-invocation budget, the aggregate pool, reservations
 * (including running solve workers' grants), queued work, available memory and the
 * last refusal, with controls for a pool size
 * and a per-invocation budget. The native host owns every verdict; this panel
 * shows the figures it reports and never fabricates one.
 */
export function MemoryPoolPanel({ active }: { active: boolean }) {
  const native = isNativeMemoryPoolAvailable();
  const [status, setStatus] = useState<PoolStatus | null>(null);
  const [readError, setReadError] = useState<string | null>(null);
  const [poolInput, setPoolInput] = useState("");
  const [budgetInput, setBudgetInput] = useState("");
  const [acknowledged, setAcknowledged] = useState(false);
  const [submitting, setSubmitting] = useState(false);
  const [outcome, setOutcome] = useState<Outcome | null>(null);
  const mounted = useRef(true);
  const readSequence = useRef(0);
  const inputsTouched = useRef(false);

  useEffect(() => {
    mounted.current = true;
    return () => { mounted.current = false; };
  }, []);

  const publish = useCallback((next: PoolStatus) => {
    setStatus(next);
    setReadError(null);
    if (!inputsTouched.current) {
      setPoolInput(gibInput(next.pool_bytes));
      setBudgetInput(gibInput(next.invocation_budget_bytes));
    }
  }, []);

  const refresh = useCallback(async () => {
    if (!isNativeMemoryPoolAvailable()) return;
    const ticket = ++readSequence.current;
    try {
      const next = await getMemoryPoolStatus();
      if (mounted.current && ticket === readSequence.current) publish(next);
    } catch (error) {
      if (mounted.current && ticket === readSequence.current) setReadError(errorText(error));
    }
  }, [publish]);

  useEffect(() => {
    if (!native) return;
    void refresh();
    if (!active) return;
    const timer = setInterval(() => void refresh(), MEMORY_POOL_ACTIVE_REFRESH_MS);
    return () => clearInterval(timer);
  }, [active, native, refresh]);

  if (!native) {
    return (
      <section className="memory-pool-panel" aria-label="Memory pool" data-testid="memory-pool-panel">
        <div className="panel-subtitle">Memory</div>
        <p role="status" data-testid="memory-pool-unavailable">
          Memory pool unavailable: the browser preview has no native memory pool, so no budget, pool or available-memory figures are shown.
        </p>
      </section>
    );
  }

  const poolBytes = bytesFromGib(poolInput);
  const budgetBytes = bytesFromGib(budgetInput);
  const recommended = status?.recommended_pool_bytes ?? null;
  const aboveRecommended = poolBytes !== null && recommended !== null && poolBytes > recommended;
  // A budget may not exceed the pool being requested (and never the machine's
  // capacity); the native side enforces the same rule.
  const capacity = status?.capacity_bytes ?? null;
  // An unedited pool field shows the current pool rounded; only an edit to it
  // (by at least 0.01 GiB) is a requested pool.
  const requestedPool = status && poolBytes !== null && Math.abs(poolBytes - status.pool_bytes) >= 0.01 * 1024 ** 3 ? poolBytes : null;
  const maxBudget = requestedPool !== null
    ? (capacity !== null ? Math.min(requestedPool, capacity) : requestedPool)
    : (status?.max_invocation_budget_bytes ?? null);
  const budgetAboveMax = budgetBytes !== null && maxBudget !== null && budgetBytes > maxBudget;
  const canApply = !!status && poolBytes !== null && budgetBytes !== null && !budgetAboveMax && !submitting && (!aboveRecommended || acknowledged);

  async function apply(event: FormEvent) {
    event.preventDefault();
    if (!canApply || poolBytes === null || budgetBytes === null) return;
    setSubmitting(true);
    setOutcome(null);
    const ticket = ++readSequence.current;
    try {
      const next = await configureMemoryPool({
        poolBytes,
        invocationBudgetBytes: budgetBytes,
        acknowledgeAboveRecommended: aboveRecommended && acknowledged
      });
      if (!mounted.current) return;
      if (ticket === readSequence.current) publish(next);
      setOutcome({
        tone: "applied",
        text: `Applied: pool ${formatBytes(next.pool_bytes)}; per-invocation budget ${formatBytes(next.invocation_budget_bytes)}.`
      });
    } catch (error) {
      if (!mounted.current) return;
      if (error instanceof ResourceUnavailableError) {
        if (error.pool && ticket === readSequence.current) publish(error.pool);
        setOutcome({ tone: "refused", text: `Refused by the native memory pool: ${describeResourceRefusal(error.refusal)}` });
      } else {
        setOutcome({ tone: "error", text: `The request did not reach a verdict: ${errorText(error)}` });
      }
    } finally {
      if (mounted.current) setSubmitting(false);
    }
  }

  return (
    <section className="memory-pool-panel" aria-label="Memory pool" data-testid="memory-pool-panel">
      <div className="panel-subtitle">Memory</div>
      {readError ? (
        <p role="status" className="memory-pool-message error" data-testid="memory-pool-read-error">
          Memory pool status could not be read: {readError}
        </p>
      ) : null}
      {status ? (
        <div className="report-list memory-pool-status" data-testid="memory-pool-status">
          <PoolLine
            label="Per-invocation budget"
            testId="memory-pool-budget"
            value={`${formatBytes(status.invocation_budget_bytes)} (default ${formatBytes(status.default_invocation_budget_bytes)}; any budget up to ${formatBytes(status.max_invocation_budget_bytes)}, the pool size, can be selected)`}
          />
          <PoolLine
            label="Budgets above the default"
            testId="memory-pool-budget-larger"
            value="each ordinary solve runs in a worker process held to the budget, so a larger budget admits larger ordinary solves; it does not reserve memory in advance: a queued solve waits for its starting grant until it is admitted or cancelled, and intake, worker growth and result transfers re-check insufficient available memory for up to 2 s before refusing"
          />
          <PoolLine
            label="What the budget bounds"
            testId="memory-pool-budget-scope"
            value="receiving the model document (request intake) and each ordinary solve's worker process, whose reservation grows on demand up to the budget; a queued solve waits for its starting grant until it is admitted or cancelled; a solve needing more than the budget is refused, and intake, worker growth and result transfers re-check insufficient available memory for up to 2 s before refusing; results held natively and by the app are bounded by the pool, available memory and result-count limits, not by the budget"
          />
          <PoolLine
            label="Edits, conversions and saves"
            testId="memory-pool-interactive"
            value="each request is admitted by the pool, available memory and the budget; unlike a solve, it is not refused because macOS reports memory pressure"
          />
          <PoolLine
            label="Memory pool"
            testId="memory-pool-size"
            value={`${formatBytes(status.pool_bytes)}; recommended ${formatBytes(status.recommended_pool_bytes)} for ${formatBytes(status.capacity_bytes)} physical memory`}
          />
          <PoolLine
            label="Reserved"
            testId="memory-pool-reserved"
            value={`${formatBytes(status.reserved_bytes)} (model intake ${formatBytes(status.reserved_by_purpose.ingress)}, solve workers ${formatBytes(status.reserved_by_purpose.worker)}, results ${formatBytes(status.reserved_by_purpose.result)}, results held by the app ${formatBytes(status.reserved_by_purpose.custody)})`}
          />
          <PoolLine
            label="Solve workers"
            testId="memory-pool-ordinary"
            value={`${status.running_workers} of at most ${status.limits.max_running_workers} running, holding ${formatBytes(status.reserved_by_purpose.worker)}; each is capped at the per-invocation budget`}
          />
          <PoolLine
            label="Work"
            testId="memory-pool-queue"
            value={`${status.queued_jobs} queued; ${status.running_jobs} running; ${status.retained_results} results awaiting collection; ${status.webview_custody_results} held by the app`}
          />
          <PoolLine
            label="Available memory (free and reclaimable cache)"
            testId="memory-pool-headroom"
            value={`${status.headroom.available_bytes === null ? "not reported" : `${formatBytes(status.headroom.available_bytes)} available`}; pressure ${status.headroom.pressure}${status.headroom.sample_age_ms === null ? "" : `; read ${status.headroom.sample_age_ms} ms ago`}`}
          />
          <PoolLine
            label="Last refusal"
            testId="memory-pool-last-refusal"
            value={status.last_refusal ? describeResourceRefusal(status.last_refusal) : "none"}
          />
          {status.pool_above_recommended ? (
            <p className="memory-pool-message warning" data-testid="memory-pool-above-recommended">
              <AlertTriangle size={14} aria-hidden="true" /> The pool is above the recommended size for this Mac.
            </p>
          ) : null}
        </div>
      ) : readError ? null : (
        <p role="status" data-testid="memory-pool-reading">Reading memory pool status…</p>
      )}
      <p className="memory-pool-note" data-testid="memory-pool-boundary">
        Figures are what this Mac reports when read; other applications can change available memory at any time. The budget caps
        what each solve worker may reserve and the pool caps what all work may reserve; these limits are not an assurance that a
        solve will run or finish. A 16 GiB Mac is the base design target; no statement is made for arbitrary competing workloads.
      </p>
      <form className="memory-pool-form" aria-label="Memory pool settings" onSubmit={(event) => void apply(event)}>
        <label className="memory-pool-field">
          <span>Pool size (GiB)</span>
          <input
            data-testid="memory-pool-input-pool"
            inputMode="decimal"
            min="0"
            step="0.25"
            type="number"
            value={poolInput}
            onChange={(event) => {
              inputsTouched.current = true;
              setPoolInput(event.target.value);
              setAcknowledged(false);
              setOutcome(null);
            }}
          />
        </label>
        <label className="memory-pool-field">
          <span>Per-invocation budget (GiB)</span>
          <input
            data-testid="memory-pool-input-budget"
            inputMode="decimal"
            max={maxBudget === null ? undefined : gibMax(maxBudget)}
            min="0"
            step="0.25"
            type="number"
            value={budgetInput}
            onChange={(event) => {
              inputsTouched.current = true;
              setBudgetInput(event.target.value);
              setOutcome(null);
            }}
          />
        </label>
        {budgetAboveMax ? (
          <p className="memory-pool-message warning" role="alert" data-testid="memory-pool-budget-above-max">
            <AlertTriangle size={14} aria-hidden="true" /> {formatBytes(budgetBytes)} is above the largest per-invocation budget,{" "}
            {formatBytes(maxBudget)} (the pool size). Choose a budget no larger than the pool.
          </p>
        ) : null}
        {aboveRecommended ? (
          <div className="memory-pool-message warning" role="alert" data-testid="memory-pool-recommended-warning">
            <p>
              <AlertTriangle size={14} aria-hidden="true" /> {formatBytes(poolBytes)} is above the recommended {formatBytes(recommended)} pool
              for this Mac. A larger pool leaves less memory for other applications.
            </p>
            <label className="memory-pool-ack">
              <input
                data-testid="memory-pool-acknowledge"
                type="checkbox"
                checked={acknowledged}
                onChange={(event) => setAcknowledged(event.target.checked)}
              />
              <span>
                I acknowledge the pool is above the recommended size. The native side still refuses it when available memory
                or capacity does not admit it.
              </span>
            </label>
          </div>
        ) : null}
        <div className="memory-pool-actions">
          <button className="secondary-action" data-testid="memory-pool-apply" disabled={!canApply} type="submit">
            <SlidersHorizontal size={14} aria-hidden="true" />
            {submitting ? "Requesting" : "Request pool settings"}
          </button>
          <button className="secondary-action" data-testid="memory-pool-refresh" onClick={() => void refresh()} type="button">
            <RefreshCw size={14} aria-hidden="true" />
            Refresh memory status
          </button>
        </div>
      </form>
      {outcome ? (
        <p role="status" className={`memory-pool-message ${outcome.tone === "applied" ? "info" : outcome.tone === "refused" ? "warning" : "error"}`} data-testid="memory-pool-outcome">
          {outcome.text}
        </p>
      ) : null}
    </section>
  );
}

function PoolLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function gibInput(bytes: number): string {
  const value = bytes / GIB;
  return Number.isInteger(value) ? String(value) : value.toFixed(2).replace(/0+$/, "").replace(/\.$/, "");
}

/** The largest whole hundredth of a GiB not above `bytes`, for the input's limit. */
function gibMax(bytes: number): string {
  return String(Math.floor((bytes / GIB) * 100) / 100);
}

function bytesFromGib(input: string): number | null {
  if (!input.trim()) return null;
  const value = Number(input);
  if (!Number.isFinite(value) || value <= 0) return null;
  const bytes = Math.floor(value * GIB);
  return Number.isSafeInteger(bytes) && bytes > 0 ? bytes : null;
}

function errorText(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { PoolStatus } from "../../types";
import { MemoryPoolPanel } from "./MemoryPoolPanel";

// Mocked native pool readings for the panel's display and request gating.
// Unit simulation only: no figure here is a reading of any machine.
const GiB = 1024 ** 3;
function poolStatus(overrides: Partial<PoolStatus> = {}): PoolStatus {
  return {
    capacity_bytes: 64 * GiB,
    recommended_pool_bytes: 48 * GiB,
    pool_bytes: 48 * GiB,
    pool_above_recommended: false,
    invocation_budget_bytes: 12 * GiB,
    default_invocation_budget_bytes: 12 * GiB,
    max_invocation_budget_bytes: 48 * GiB,
    reserved_bytes: 18.5 * GiB,
    reserved_by_purpose: { ingress: 0.5 * GiB, worker: 6 * GiB, result: 0, custody: 12 * GiB },
    running_workers: 2,
    limits: { max_running_workers: 8 },
    queued_jobs: 1,
    running_jobs: 2,
    retained_results: 0,
    webview_custody_results: 1,
    headroom: { available_bytes: 20 * GiB, pressure: "normal", sample_age_ms: 120 },
    ordinary_route_pricing: "worker_capped",
    last_refusal: { kind: "worker_cap_exceeded", cap_bytes: 12 * GiB, requested_bytes: 14 * GiB },
    ...overrides
  };
}

function nativeRuntime() {
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
}

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const line = (testId: string) => screen.getByTestId(testId).querySelector("strong")?.textContent;
const statusCalls = () => invokeMock.mock.calls.filter(([command]) => command === "get_memory_pool_status").length;

describe("the memory pool panel", () => {
  it("shows an unavailable state in the browser preview without fabricating figures", () => {
    render(<MemoryPoolPanel active={false} />);
    const panel = screen.getByTestId("memory-pool-panel");
    expect(screen.getByTestId("memory-pool-unavailable")).toHaveTextContent("Memory pool unavailable");
    expect(panel.textContent).not.toMatch(/GiB|MiB/);
    expect(within(panel).queryByRole("button")).toBeNull();
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("shows the budget, pool, reservations, running solve workers, queued work, available memory and last refusal", async () => {
    nativeRuntime();
    invokeMock.mockResolvedValue(poolStatus());
    render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    expect(invokeMock).toHaveBeenCalledWith("get_memory_pool_status");
    expect(line("memory-pool-budget")).toBe("12 GiB (default 12 GiB; any budget up to 48 GiB, the pool size, can be selected)");
    expect(line("memory-pool-budget-larger")).toBe("each ordinary solve runs in a worker process held to the budget, so a larger budget admits larger ordinary solves; it does not reserve memory in advance: a queued solve waits for its starting grant until it is admitted or cancelled, and intake, worker growth and result transfers re-check insufficient available memory for up to 2 s before refusing");
    // Interactive requests are admitted by pool, available memory and budget, not refused for macOS memory pressure.
    expect(line("memory-pool-interactive")).toBe("each request is admitted by the pool, available memory and the budget; unlike a solve, it is not refused because macOS reports memory pressure");
    expect(screen.getByTestId("memory-pool-panel").textContent).not.toMatch(/qualif/i);
    // The budget bounds the request intake and each ordinary solve's worker, not held results.
    expect(line("memory-pool-budget-scope")).toBe("receiving the model document (request intake) and each ordinary solve's worker process, whose reservation grows on demand up to the budget; a queued solve waits for its starting grant until it is admitted or cancelled; a solve needing more than the budget is refused, and intake, worker growth and result transfers re-check insufficient available memory for up to 2 s before refusing; results held natively and by the app are bounded by the pool, available memory and result-count limits, not by the budget");
    expect(screen.getByTestId("memory-pool-status").textContent).not.toMatch(/qualified up to|guarantee/i);
    expect(line("memory-pool-size")).toBe("48 GiB; recommended 48 GiB for 64 GiB physical memory");
    expect(line("memory-pool-reserved")).toBe("18.5 GiB (model intake 512 MiB, solve workers 6 GiB, results 0 MiB, results held by the app 12 GiB)");
    expect(line("memory-pool-ordinary")).toBe("2 of at most 8 running, holding 6 GiB; each is capped at the per-invocation budget");
    expect(line("memory-pool-queue")).toBe("1 queued; 2 running; 0 results awaiting collection; 1 held by the app");
    expect(line("memory-pool-headroom")).toBe("20 GiB available; pressure normal; read 120 ms ago");
    expect(line("memory-pool-last-refusal")).toBe("The solve needed more memory than the per-invocation budget of 12 GiB allows (requested 14 GiB).");
    // The interim unpriced, one-at-a-time wording is gone.
    expect(screen.getByTestId("memory-pool-panel").textContent).not.toMatch(/unpriced|one at a time/i);
    expect(screen.getByTestId("memory-pool-boundary").textContent).toMatch(/not an assurance/);
    expect(screen.getByTestId("memory-pool-boundary").textContent).toMatch(/16 GiB Mac is the base design target/);
    expect(screen.getByTestId("memory-pool-boundary").textContent).not.toMatch(/guarantee[sd]? (that|to)/i);
    // Inputs start at the reported pool and budget.
    expect(screen.getByLabelText("Pool size (GiB)")).toHaveValue(48);
    expect(screen.getByLabelText("Per-invocation budget (GiB)")).toHaveValue(12);
  });

  it("reports unknown capacity and headroom as unknown rather than inventing numbers", async () => {
    nativeRuntime();
    invokeMock.mockResolvedValue(poolStatus({ capacity_bytes: null, recommended_pool_bytes: null, headroom: { available_bytes: null, pressure: "unknown", sample_age_ms: null }, last_refusal: null }));
    render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    expect(line("memory-pool-size")).toBe("48 GiB; recommended unknown for unknown physical memory");
    expect(line("memory-pool-headroom")).toBe("not reported; pressure unknown");
    expect(line("memory-pool-last-refusal")).toBe("none");
  });

  it("shows no running solve worker and no worker reservation when none runs", async () => {
    nativeRuntime();
    invokeMock.mockResolvedValue(poolStatus({ running_workers: 0, running_jobs: 0, reserved_bytes: 12.5 * GiB, reserved_by_purpose: { ingress: 0.5 * GiB, worker: 0, result: 0, custody: 12 * GiB } }));
    render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    expect(line("memory-pool-ordinary")).toBe("0 of at most 8 running, holding 0 MiB; each is capped at the per-invocation budget");
    expect(line("memory-pool-reserved")).toBe("12.5 GiB (model intake 512 MiB, solve workers 0 MiB, results 0 MiB, results held by the app 12 GiB)");
  });

  it.each([
    [{ ordinary_route_pricing: "unpriced" }, "the interim unpriced route"],
    [{ running_workers: undefined, unpriced_solve_running: true }, "the interim running flag in place of a worker count"],
    [{ reserved_by_purpose: { ingress: 0, result: 0, custody: 0 } }, "reservations without the worker purpose"],
    [{ running_workers: -1 }, "a negative worker count"],
    [{ limits: { max_queued_jobs: 4 } }, "no running-worker limit"],
    [{ limits: undefined }, "no limits"]
  ] as Array<[Record<string, unknown>, string]>)("reads a pool status with %s as unreadable (%s)", async (overrides) => {
    nativeRuntime();
    invokeMock.mockResolvedValue({ ...poolStatus(), ...overrides });
    render(<MemoryPoolPanel active={false} />);
    expect(await screen.findByTestId("memory-pool-read-error")).toHaveTextContent("MEMORY_POOL_STATUS_INVALID");
  });

  it("requires an explicit acknowledgement above the recommended pool, and the acknowledgement does not hide a native headroom refusal", async () => {
    nativeRuntime();
    const refusal = { kind: "headroom_insufficient" as const, required_bytes: 56 * GiB, available_bytes: 20 * GiB };
    invokeMock.mockImplementation(async (command: string) => {
      if (command === "get_memory_pool_status") return poolStatus();
      if (command === "configure_memory_pool") throw { code: "RESOURCE_UNAVAILABLE", refusal, pool: poolStatus({ last_refusal: refusal }) };
      throw new Error(`unexpected ${command}`);
    });
    render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    const apply = screen.getByTestId("memory-pool-apply");
    fireEvent.change(screen.getByLabelText("Pool size (GiB)"), { target: { value: "56" } });
    expect(screen.getByTestId("memory-pool-recommended-warning")).toHaveTextContent("56 GiB is above the recommended 48 GiB pool for this Mac.");
    expect(apply).toBeDisabled();
    fireEvent.submit(apply.closest("form")!);
    expect(invokeMock.mock.calls.some(([command]) => command === "configure_memory_pool")).toBe(false);
    const ack = screen.getByRole("checkbox", { name: /I acknowledge the pool is above the recommended size/ });
    fireEvent.click(ack);
    expect(apply).toBeEnabled();
    fireEvent.click(apply);
    const outcome = await screen.findByTestId("memory-pool-outcome");
    expect(invokeMock).toHaveBeenCalledWith("configure_memory_pool", { poolBytes: 56 * GiB, invocationBudgetBytes: 12 * GiB, acknowledgeAboveRecommended: true });
    expect(outcome).toHaveTextContent("Refused by the native memory pool: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 56 GiB, 20 GiB available.");
    expect(outcome).not.toHaveTextContent("Applied");
    expect(line("memory-pool-last-refusal")).toBe("Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 56 GiB, 20 GiB available.");
    // Changing the request withdraws the acknowledgement.
    fireEvent.change(screen.getByLabelText("Pool size (GiB)"), { target: { value: "60" } });
    expect(screen.getByRole("checkbox", { name: /I acknowledge/ })).not.toBeChecked();
    expect(apply).toBeDisabled();
  });

  it("sends a pool at or below the recommended size without an acknowledgement, and shows the applied status", async () => {
    nativeRuntime();
    invokeMock.mockImplementation(async (command: string, args?: { poolBytes: number; invocationBudgetBytes: number }) => {
      if (command === "get_memory_pool_status") return poolStatus();
      if (command === "configure_memory_pool") return poolStatus({ pool_bytes: args!.poolBytes, max_invocation_budget_bytes: args!.poolBytes, invocation_budget_bytes: args!.invocationBudgetBytes });
      throw new Error(`unexpected ${command}`);
    });
    render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    fireEvent.change(screen.getByLabelText("Pool size (GiB)"), { target: { value: "24" } });
    fireEvent.change(screen.getByLabelText("Per-invocation budget (GiB)"), { target: { value: "8" } });
    expect(screen.queryByTestId("memory-pool-recommended-warning")).toBeNull();
    fireEvent.click(screen.getByTestId("memory-pool-apply"));
    expect(await screen.findByTestId("memory-pool-outcome")).toHaveTextContent("Applied: pool 24 GiB; per-invocation budget 8 GiB.");
    expect(invokeMock).toHaveBeenCalledWith("configure_memory_pool", { poolBytes: 24 * GiB, invocationBudgetBytes: 8 * GiB, acknowledgeAboveRecommended: false });
    expect(line("memory-pool-budget")).toBe("8 GiB (default 12 GiB; any budget up to 24 GiB, the pool size, can be selected)");
  });

  it("accepts a per-invocation budget above the 12 GiB default up to the pool, and holds a larger one back", async () => {
    nativeRuntime();
    invokeMock.mockImplementation(async (command: string, args?: { poolBytes: number; invocationBudgetBytes: number }) => {
      if (command === "get_memory_pool_status") return poolStatus();
      if (command === "configure_memory_pool") return poolStatus({ pool_bytes: args!.poolBytes, invocation_budget_bytes: args!.invocationBudgetBytes });
      throw new Error(`unexpected ${command}`);
    });
    render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    const budget = screen.getByLabelText("Per-invocation budget (GiB)");
    expect(budget).toHaveAttribute("max", "48");
    const apply = screen.getByTestId("memory-pool-apply");
    fireEvent.change(budget, { target: { value: "48.5" } });
    expect(screen.getByTestId("memory-pool-budget-above-max")).toHaveTextContent("48.5 GiB is above the largest per-invocation budget, 48 GiB (the pool size).");
    expect(apply).toBeDisabled();
    fireEvent.submit(apply.closest("form")!);
    expect(invokeMock.mock.calls.some(([command]) => command === "configure_memory_pool")).toBe(false);
    fireEvent.change(budget, { target: { value: "48" } });
    expect(screen.queryByTestId("memory-pool-budget-above-max")).toBeNull();
    fireEvent.change(budget, { target: { value: "20" } });
    expect(apply).toBeEnabled();
    fireEvent.click(apply);
    expect(await screen.findByTestId("memory-pool-outcome")).toHaveTextContent("Applied: pool 48 GiB; per-invocation budget 20 GiB.");
    expect(invokeMock).toHaveBeenCalledWith("configure_memory_pool", { poolBytes: 48 * GiB, invocationBudgetBytes: 20 * GiB, acknowledgeAboveRecommended: false });
    expect(line("memory-pool-budget")).toBe("20 GiB (default 12 GiB; any budget up to 48 GiB, the pool size, can be selected)");
  });

  it("checks the budget against the pool being requested, so pool and budget can be raised together", async () => {
    nativeRuntime();
    invokeMock.mockImplementation(async (command: string) => {
      if (command === "get_memory_pool_status") return poolStatus();
      throw new Error(`unexpected ${command}`);
    });
    render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    const budget = screen.getByLabelText("Per-invocation budget (GiB)");
    const apply = screen.getByTestId("memory-pool-apply");
    fireEvent.change(screen.getByLabelText("Pool size (GiB)"), { target: { value: "56" } });
    fireEvent.click(screen.getByRole("checkbox", { name: /I acknowledge/ }));
    fireEvent.change(budget, { target: { value: "52" } });
    expect(screen.queryByTestId("memory-pool-budget-above-max")).toBeNull();
    expect(budget).toHaveAttribute("max", "56");
    expect(apply).toBeEnabled();
    // Lowering the pool below the budget holds the budget back before any request.
    fireEvent.change(screen.getByLabelText("Pool size (GiB)"), { target: { value: "16" } });
    expect(screen.getByTestId("memory-pool-budget-above-max")).toBeInTheDocument();
    expect(apply).toBeDisabled();
  });

  it("limits the budget input to the largest budget rounded down", async () => {
    nativeRuntime();
    invokeMock.mockResolvedValue(poolStatus({ pool_bytes: 12 * GiB - 1, max_invocation_budget_bytes: 12 * GiB - 1 }));
    render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    expect(screen.getByLabelText("Per-invocation budget (GiB)")).toHaveAttribute("max", "11.99");
  });

  it.each([
    [{ kind: "pool_above_capacity", pool_bytes: 80 * GiB, capacity_bytes: 64 * GiB }, "12", "80", "A pool of 80 GiB is larger than this Mac's physical memory (64 GiB)."]
  ])("shows the native %s refusal in plain language", async (refusal, budget, pool, text) => {
    nativeRuntime();
    invokeMock.mockImplementation(async (command: string) => {
      if (command === "get_memory_pool_status") return poolStatus();
      if (command === "configure_memory_pool") throw { code: "RESOURCE_UNAVAILABLE", refusal, pool: null };
      throw new Error(`unexpected ${command}`);
    });
    render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    fireEvent.change(screen.getByLabelText("Per-invocation budget (GiB)"), { target: { value: budget } });
    fireEvent.change(screen.getByLabelText("Pool size (GiB)"), { target: { value: pool } });
    const ack = screen.queryByRole("checkbox", { name: /I acknowledge/ });
    if (ack) fireEvent.click(ack);
    fireEvent.click(screen.getByTestId("memory-pool-apply"));
    expect(await screen.findByTestId("memory-pool-outcome")).toHaveTextContent(`Refused by the native memory pool: ${text}`);
  });

  it.each([
    [{ kind: "pressure_unknown" }, "macOS did not report a memory-pressure state, so no new solve is admitted."],
    [{ kind: "pressure_elevated", level: "warning" }, "macOS reports memory pressure (warning), so no new solve is admitted until it returns to normal."],
    [{ kind: "pressure_elevated", level: "critical" }, "macOS reports memory pressure (critical), so no new solve is admitted until it returns to normal."],
    [{ kind: "worker_growth_refused", granted_bytes: 4 * GiB, requested_bytes: 6 * GiB, cause: { kind: "pool_exhausted", requested_bytes: 2 * GiB, reserved_bytes: 47 * GiB, pool_bytes: 48 * GiB } },
      "The solve needed more memory (6 GiB) than could be reserved while it ran: The memory pool is fully reserved: this request needs 2 GiB, and 47 GiB of the 48 GiB pool is already reserved."],
    [{ kind: "worker_footprint_exceeded", grant_bytes: 4 * GiB, footprint_bytes: 4.5 * GiB },
      "The solve's measured memory footprint (4.5 GiB) does not fit the per-invocation budget, so the solve was ended (its reservation was 4 GiB)."],
    [{ kind: "worker_limit_reached", running: 8, limit: 8 }, "8 solve workers are already running (limit 8); this solve waits until one finishes."]
  ])("shows the last %s refusal in plain language", async (refusal, text) => {
    nativeRuntime();
    invokeMock.mockResolvedValue(poolStatus({ last_refusal: refusal as PoolStatus["last_refusal"] }));
    render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    expect(line("memory-pool-last-refusal")).toBe(text);
  });

  it("refreshes while a solve is active and on demand, and shows an unreadable status as an error", async () => {
    nativeRuntime();
    invokeMock.mockResolvedValue(poolStatus());
    const view = render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    const idle = statusCalls();
    fireEvent.click(screen.getByTestId("memory-pool-refresh"));
    await waitFor(() => expect(statusCalls()).toBe(idle + 1));
    view.rerender(<MemoryPoolPanel active />);
    await waitFor(() => expect(statusCalls()).toBeGreaterThanOrEqual(idle + 3), { timeout: 3000 });
    invokeMock.mockResolvedValue({ pool_bytes: "a lot" });
    fireEvent.click(screen.getByTestId("memory-pool-refresh"));
    expect(await screen.findByTestId("memory-pool-read-error")).toHaveTextContent("MEMORY_POOL_STATUS_INVALID");
    view.rerender(<MemoryPoolPanel active={false} />);
  });
});

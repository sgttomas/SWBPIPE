import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
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
    qualified_invocation_budget_bytes: 12 * GiB,
    reserved_bytes: 12.5 * GiB,
    reserved_by_purpose: { ingress: 0.5 * GiB, result: 0, custody: 12 * GiB },
    unpriced_solve_running: true,
    queued_jobs: 1,
    running_jobs: 1,
    retained_results: 0,
    webview_custody_results: 1,
    headroom: { available_bytes: 20 * GiB, pressure: "normal", sample_age_ms: 120 },
    ordinary_route_pricing: "unpriced",
    last_refusal: { kind: "solve_slot_busy" },
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

  it("shows the budget, pool, reservations, unpriced ordinary solve, queued work, free memory and last refusal", async () => {
    nativeRuntime();
    invokeMock.mockResolvedValue(poolStatus());
    render(<MemoryPoolPanel active={false} />);
    await screen.findByTestId("memory-pool-status");
    expect(invokeMock).toHaveBeenCalledWith("get_memory_pool_status");
    expect(line("memory-pool-budget")).toBe("12 GiB (qualified up to 12 GiB)");
    expect(line("memory-pool-size")).toBe("48 GiB; recommended 48 GiB for 64 GiB physical memory");
    expect(line("memory-pool-reserved")).toBe("12.5 GiB (model intake 512 MiB, results 0 MiB, results held by the app 12 GiB)");
    expect(line("memory-pool-ordinary")).toBe("running; unpriced: no byte bound is claimed for the ordinary solve itself");
    expect(line("memory-pool-queue")).toBe("1 queued; 1 running; 0 results awaiting collection; 1 held by the app");
    expect(line("memory-pool-headroom")).toBe("20 GiB available; pressure normal; read 120 ms ago");
    expect(line("memory-pool-last-refusal")).toBe("Another ordinary solve is running; ordinary solves are unpriced, so this one waits for it to finish.");
    expect(screen.getByTestId("memory-pool-boundary").textContent).toMatch(/not an assurance/);
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
    expect(outcome).toHaveTextContent("Refused by the native memory pool: Not enough free memory on this Mac right now: needs 56 GiB, 20 GiB available.");
    expect(outcome).not.toHaveTextContent("Applied");
    expect(line("memory-pool-last-refusal")).toBe("Not enough free memory on this Mac right now: needs 56 GiB, 20 GiB available.");
    // Changing the request withdraws the acknowledgement.
    fireEvent.change(screen.getByLabelText("Pool size (GiB)"), { target: { value: "60" } });
    expect(screen.getByRole("checkbox", { name: /I acknowledge/ })).not.toBeChecked();
    expect(apply).toBeDisabled();
  });

  it("sends a pool at or below the recommended size without an acknowledgement, and shows the applied status", async () => {
    nativeRuntime();
    invokeMock.mockImplementation(async (command: string, args?: { poolBytes: number; invocationBudgetBytes: number }) => {
      if (command === "get_memory_pool_status") return poolStatus();
      if (command === "configure_memory_pool") return poolStatus({ pool_bytes: args!.poolBytes, invocation_budget_bytes: args!.invocationBudgetBytes });
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
    expect(line("memory-pool-budget")).toBe("8 GiB (qualified up to 12 GiB)");
  });

  it.each([
    [{ kind: "budget_unqualified", requested_bytes: 16 * GiB, qualified_bytes: 12 * GiB }, "16", "48", "A per-invocation budget of 16 GiB is above the qualified budget of 12 GiB; only qualified budgets can be selected."],
    [{ kind: "pool_above_capacity", pool_bytes: 80 * GiB, capacity_bytes: 64 * GiB }, "12", "80", "A pool of 80 GiB is larger than this Mac's physical memory (64 GiB)."],
    [{ kind: "budget_exceeds_pool", budget_bytes: 12 * GiB, pool_bytes: 8 * GiB }, "12", "8", "The per-invocation budget of 12 GiB is larger than the 8 GiB pool."]
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

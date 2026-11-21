// The Analyze page by removal: one status line in words and the solve controls lead; the
// memory settings and every record of the run sit unchanged inside two closed disclosures.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { loadPreviewModel } from "../../services/previewService";
import type { MechanicsResult, PoolStatus } from "../../types";
import { initialSolveJob, startSolveJob } from "../workspace/solveJobAudit";
import { SolvePanel } from "./SolvePanel";

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function solvedResult(): MechanicsResult {
  return JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_dense_scrutiny.json"), "utf8")) as MechanicsResult;
}

async function props() {
  const model = await loadPreviewModel();
  return { model, result: null, analysisRun: null, running: false, solveJob: initialSolveJob(),
    solverMode: "sparse_interactive" as const, onRun: vi.fn(), onCancel: vi.fn(), onSolverModeChange: vi.fn() };
}

const RECORD_IDS = [
  "solve-readiness-summary", "readiness-mechanics", "readiness-rule", "readiness-diagnostics", "readiness-professional",
  "solve-job-summary", "solve-job-audit", "solve-job-solver-mode", "solve-job-progress", "solve-job-cancellation",
  "solve-job-resource", "solve-job-binding", "solve-job-unit-policy", "solve-job-boundary"
];

it("leads after a solve with one status line in words, the solver choice and Run, and keeps both disclosures closed", async () => {
  const base = await props();
  const onOpenIssues = vi.fn();
  const result = solvedResult();
  render(<SolvePanel {...base} result={result} issueCount={30} onOpenIssues={onOpenIssues} />);

  const line = screen.getByTestId("solve-status-line");
  expect(screen.getByTestId("solve-status-text")).toHaveTextContent(/^Mechanics solved: \d+ result rows?\. Rule inputs incomplete\.$/);
  // No status code in capitals on the line.
  expect(line.textContent).not.toMatch(/[A-Z]{2,}_[A-Z_]+/);
  fireEvent.click(within(line).getByRole("button", { name: "30 issues" }));
  expect(onOpenIssues).toHaveBeenCalledTimes(1);

  const choice = screen.getByRole("group", { name: "Solver mode" });
  const sparse = within(choice).getByRole("button", { name: "Sparse solver" });
  const dense = within(choice).getByRole("button", { name: "Dense solver" });
  expect(sparse).toHaveAttribute("aria-pressed", "true");
  expect(sparse.title).toMatch(/default/);
  expect(dense.title).toMatch(/full stiffness matrix/);
  expect(screen.getByRole("button", { name: "Run" })).toBeEnabled();
  expect(screen.queryByTestId("cancel-mechanics-preview")).toBeNull();

  const memory = screen.getByTestId("solve-memory-settings");
  const record = screen.getByTestId("solve-run-record");
  expect(memory).not.toHaveAttribute("open");
  expect(record).not.toHaveAttribute("open");
  expect(within(memory).getByText("Memory settings").tagName).toBe("SUMMARY");
  expect(within(record).getByText("Run record").tagName).toBe("SUMMARY");
  // The line and the controls come first; the disclosures last, Run record at the foot.
  expect(line.compareDocumentPosition(choice) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  expect(choice.compareDocumentPosition(memory) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  expect(memory.compareDocumentPosition(record) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  for (const id of RECORD_IDS) {
    expect(record).toContainElement(screen.getByTestId(id));
    expect(screen.getByTestId(id)).not.toBeVisible();
  }
  expect(memory).toContainElement(screen.getByTestId("memory-pool-panel"));
  expect(screen.getByTestId("memory-pool-panel")).not.toBeVisible();
});

it("shows Cancel only while a solve runs, and Run reads Running", async () => {
  const base = await props();
  const running = startSolveJob(base.model, { mode: "backend_job", job_id: "backend-solve-job-1", backend_cancellation_token: "token-1", state: "running", cancellation_scope: "worker_process_terminated", resource: null });
  const view = render(<SolvePanel {...base} running solveJob={running} />);
  // A job just started reports queued; the line says so in words.
  expect(screen.getByTestId("solve-status-text")).toHaveTextContent("Waiting in the queue to solve.");
  expect(screen.getByTestId("run-mechanics-preview")).toHaveTextContent("Running");
  expect(screen.getByTestId("run-mechanics-preview")).toBeDisabled();
  const cancel = screen.getByRole("button", { name: "Cancel" });
  expect(cancel).toBeEnabled();
  fireEvent.click(cancel);
  expect(base.onCancel).toHaveBeenCalledTimes(1);
  // The run record says the control is visible exactly while it is.
  expect(screen.getByTestId("solve-job-cancellation")).toHaveTextContent("control_visible=true");

  view.rerender(<SolvePanel {...base} />);
  expect(screen.queryByTestId("cancel-mechanics-preview")).toBeNull();
  expect(screen.getByTestId("solve-status-text")).toHaveTextContent("Not solved yet.");
  expect(screen.getByTestId("solve-job-cancellation")).toHaveTextContent("control_visible=false");
});

it("reaches every record of the run, with its test id, once Run record is opened", async () => {
  render(<SolvePanel {...await props()} />);
  const record = screen.getByTestId("solve-run-record");
  fireEvent.click(within(record).getByText("Run record"));
  expect(record).toHaveAttribute("open");
  for (const id of RECORD_IDS) expect(screen.getByTestId(id)).toBeVisible();
  // The download, its private-values choice, its decisions line and its Details, unchanged.
  expect(screen.getByTestId("solve-job-export-link")).toBeVisible();
  expect(screen.getByTestId("solve-job-export-link-local-private-intent")).toBeVisible();
  expect(screen.getByTestId("solve-job-export-link-redaction-summary")).toHaveTextContent(/decisions=\d+; findings=\d+; blocked=true/);
  expect(within(screen.getByTestId("solve-job-export-link-redaction-details")).getByText("Details")).toBeVisible();
  fireEvent.click(screen.getByTestId("solve-job-export-link-local-private-intent"));
  expect(screen.getByTestId("solve-job-export-link")).toHaveAttribute("href", expect.stringMatching(/^data:application\/json/));
});

const GiB = 1024 ** 3;
function poolStatus(overrides: Partial<PoolStatus> = {}): PoolStatus {
  // Mocked native pool readings; no figure here is a reading of any machine.
  return {
    capacity_bytes: 64 * GiB, recommended_pool_bytes: 48 * GiB, pool_bytes: 48 * GiB, pool_above_recommended: false,
    invocation_budget_bytes: 12 * GiB, default_invocation_budget_bytes: 12 * GiB, max_invocation_budget_bytes: 48 * GiB,
    reserved_bytes: 0, reserved_by_purpose: { ingress: 0, worker: 0, result: 0, custody: 0 },
    running_workers: 0, limits: { max_running_workers: 8 }, queued_jobs: 0, running_jobs: 0, retained_results: 0, webview_custody_results: 0,
    headroom: { available_bytes: 20 * GiB, pressure: "normal", sample_age_ms: 120 }, ordinary_route_pricing: "worker_capped", last_refusal: null,
    ...overrides
  };
}

it("keeps the memory inputs working as before once Memory settings is opened, each with the line saying what it limits", async () => {
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string, args?: { poolBytes: number; invocationBudgetBytes: number }) => {
    if (command === "get_memory_pool_status") return poolStatus();
    if (command === "configure_memory_pool") return poolStatus({ pool_bytes: args!.poolBytes, max_invocation_budget_bytes: args!.poolBytes, invocation_budget_bytes: args!.invocationBudgetBytes });
    throw new Error(`unexpected ${command}`);
  });
  render(<SolvePanel {...await props()} />);
  const memory = screen.getByTestId("solve-memory-settings");
  await screen.findByTestId("memory-pool-status");
  expect(screen.getByLabelText("Memory pool (GiB)")).not.toBeVisible();
  fireEvent.click(within(memory).getByText("Memory settings"));
  expect(memory).toHaveAttribute("open");

  const pool = screen.getByLabelText("Memory pool (GiB)");
  const budget = screen.getByLabelText("Memory per solve (GiB)");
  expect(pool).toBeVisible();
  expect(pool).toHaveValue(48);
  expect(budget).toHaveValue(12);
  expect(pool).toHaveAccessibleDescription("The most memory all solves and the app's other work may reserve at once. Recommended for this Mac: 48 GiB.");
  expect(budget).toHaveAccessibleDescription("The per-invocation budget: the most memory one solve may use; a solve that needs more is refused. Default: 12 GiB.");
  fireEvent.change(pool, { target: { value: "24" } });
  fireEvent.change(budget, { target: { value: "8" } });
  fireEvent.click(screen.getByRole("button", { name: "Apply" }));
  expect(await screen.findByTestId("memory-pool-outcome")).toHaveTextContent("Applied: pool 24 GiB; per-invocation budget 8 GiB.");
  expect(invokeMock).toHaveBeenCalledWith("configure_memory_pool", { poolBytes: 24 * GiB, invocationBudgetBytes: 8 * GiB, acknowledgeAboveRecommended: false });
});

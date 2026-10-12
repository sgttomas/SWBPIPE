import { act, fireEvent, render, renderHook, waitFor, within } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import modelFixture from "../../../../../core/product_physics/tests/fixtures/preview_physics_invented_model.json";
import sourceFixture from "../../../../../fixtures/results/preview_physics_invented_sparse.json";
import { createSolveJobTransport, solveBytes } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { deliveredMechanicsResultJob, hasNativeMechanicsInvocation, loadDesignKnowledge } from "../../services/previewService";
import { SolvePanel } from "../solve/SolvePanel";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";

// The session's solve through the native job contract (DEL-04-07): queue
// reasons, typed refusals and delivered-result custody. Unit transport replay
// of recorded producer bytes through mocked IPC; NOT a native UI witness and
// no reading of any machine's memory.
const GiB = 1024 ** 3;
type Hook = (command: string, args: unknown, options: unknown, forward: () => Promise<unknown>) => Promise<unknown> | undefined;

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

async function session(hook: Hook = () => undefined) {
  const storage = await getLocalStorageCapability();
  const knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model: modelFixture, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return solveBytes(sourceFixture);
  }, { jobIdPrefix: "unit-memory-session" });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(modelFixture);
    const forward = () => transport.invoke(command, args, options);
    const hooked = hook(command, args, options, forward);
    if (hooked) return hooked;
    return forward();
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const rendered = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(rendered.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(rendered.result.current.model.modelHash).not.toBeNull());
  return { ...rendered, transport };
}

function renderSolvePanel(state: WorkspaceSession) {
  const { model } = state.model;
  const { result, analysisRun, running, solveJob, solverMode } = state.results;
  return render(<SolvePanel analysisRun={analysisRun} model={model!} result={result} running={running} solveJob={solveJob}
    solverMode={solverMode} onCancel={() => {}} onRun={() => {}} onSolverModeChange={() => {}} />);
}

const commands = () => invokeMock.mock.calls.map(([command]) => command as string);

it("shows the native queue reason while the job waits, then records the grant with the completed run", async () => {
  let release!: () => void;
  const released = new Promise<void>((resolve) => { release = resolve; });
  let polls = 0;
  const { result, transport } = await session((command, _args, _options, forward) => {
    if (command !== "poll_preview_mechanics_job") return undefined;
    polls += 1;
    if (polls === 1) return forward().then((answer) => ({ ...(answer as object), state: "queued", result_available: false, result_bytes: null, queue_reason: { kind: "headroom_insufficient", required_bytes: 12 * GiB, available_bytes: 7 * GiB } }));
    return released.then(forward);
  });
  let running!: Promise<void>;
  act(() => { running = result.current.results.handleRun(); });
  await waitFor(() => expect(result.current.results.solveJob.resource.queue_reason).not.toBeNull());
  const waiting = result.current.results.solveJob;
  expect(waiting.state).toBe("queued");
  expect(waiting.resource).toMatchObject({ source: "native_job_receipt", invocation_budget_bytes: 12 * GiB, pool_bytes: 12 * GiB, route_pricing: "ordinary_unpriced" });
  const view = renderSolvePanel(result.current);
  expect(within(view.container).getByTestId("solve-job-queue-reason")).toHaveTextContent("Waiting in the queue: Not enough free memory on this Mac right now: needs 12 GiB, 7 GiB available.");
  expect(within(view.container).getByTestId("solve-job-summary")).toHaveTextContent("state=queued");
  view.unmount();
  await act(async () => { release(); await running; });
  await waitFor(() => expect(result.current.results.currentSolvedResult).not.toBeNull());
  const done = result.current.results.solveJob;
  expect(done.state).toBe("completed");
  expect(done.resource).toMatchObject({ invocation_budget_bytes: 12 * GiB, pool_bytes: 12 * GiB, route_pricing: "ordinary_unpriced", queue_reason: null, refusal: null });
  expect(done.events.map((event) => event.event_id)).toEqual(["solve-preview-queued", "solve-preview-queued-waiting", "solve-preview-completed"]);
  // The poll never carries the result: it is taken once, and is still held by the session.
  expect(commands().filter((command) => command === "take_preview_mechanics_job_result")).toHaveLength(1);
  expect(transport.custody()).toHaveLength(1);
  // The solve-job JSON (the run's provenance export) carries the grant, outside the
  // model document; it is read through the browser download route here.
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  const panel = renderSolvePanel(result.current);
  expect(within(panel.container).getByTestId("solve-job-resource").querySelector("strong")).toHaveTextContent("budget=12 GiB; pool=12 GiB; route=ordinary_unpriced; refusal=none");
  fireEvent.click(within(panel.container).getByTestId("solve-job-export-link-local-private-intent"));
  const href = within(panel.container).getByTestId("solve-job-export-link").getAttribute("href")!;
  const packet = JSON.parse(decodeURIComponent(href.split(",", 2)[1]));
  expect(packet.resource).toMatchObject({ invocation_budget_bytes: 12 * GiB, pool_bytes: 12 * GiB, route_pricing: "ordinary_unpriced", ordinary_route_byte_bound_claimed: false, refusal: null });
  expect(JSON.stringify(result.current.model.model)).not.toMatch(/budget|pool_bytes/);
});

it("records a refused terminal job as a typed refusal with its budget, takes nothing and publishes no result", async () => {
  const refusal = { kind: "pool_exhausted", requested_bytes: 12 * GiB, reserved_bytes: 40 * GiB, pool_bytes: 48 * GiB };
  const { result } = await session((command, _args, _options, forward) => command === "poll_preview_mechanics_job"
    ? forward().then((answer) => ({ ...(answer as object), state: "refused", result_available: false, result_bytes: null, resource_refusal: refusal }))
    : undefined);
  await act(async () => { await result.current.results.handleRun(); });
  const job = result.current.results.solveJob;
  expect(job.state).toBe("refused");
  expect(job.error_message).toBeNull();
  expect(job.resource).toMatchObject({ invocation_budget_bytes: 12 * GiB, pool_bytes: 12 * GiB, route_pricing: "ordinary_unpriced", refusal });
  expect(job.events.at(-1)).toMatchObject({ event_id: "solve-preview-resource-refused", state: "refused" });
  expect(result.current.results.result).toBeNull();
  expect(result.current.results.running).toBe(false);
  expect(commands()).not.toContain("take_preview_mechanics_job_result");
  const view = renderSolvePanel(result.current);
  expect(within(view.container).getByTestId("solve-job-resource-refusal")).toHaveTextContent(
    "Refused by the native memory pool: The memory pool is fully reserved: this request needs 12 GiB, and 40 GiB of the 48 GiB pool is already reserved. No result was published."
  );
  expect(within(view.container).queryByTestId("solve-job-error")).toBeNull();
  expect(within(view.container).getByTestId("readiness-mechanics")).toHaveTextContent("refused by the native memory pool");
});

it("records a refused start with the pool status the native side reported", async () => {
  const pool = {
    capacity_bytes: 16 * GiB, recommended_pool_bytes: 12 * GiB, pool_bytes: 12 * GiB, pool_above_recommended: false,
    invocation_budget_bytes: 10 * GiB, qualified_invocation_budget_bytes: 12 * GiB, reserved_bytes: 0,
    reserved_by_purpose: { ingress: 0, result: 0, custody: 0 }, unpriced_solve_running: false, queued_jobs: 0, running_jobs: 0,
    retained_results: 0, webview_custody_results: 0, headroom: { available_bytes: null, pressure: "unknown", sample_age_ms: null },
    ordinary_route_pricing: "unpriced", last_refusal: { kind: "headroom_unknown" }
  };
  const { result } = await session((command) => command === "start_preview_mechanics_job"
    ? Promise.reject({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_unknown" }, pool })
    : undefined);
  await act(async () => { await result.current.results.handleRun(); });
  const job = result.current.results.solveJob;
  expect(job.state).toBe("refused");
  expect(job.resource).toEqual({ source: "native_refusal_pool_status", invocation_budget_bytes: 10 * GiB, pool_bytes: 12 * GiB, route_pricing: null, ingress_reserved_bytes: null, queue_reason: null, refusal: { kind: "headroom_unknown" } });
  expect(commands()).not.toContain("poll_preview_mechanics_job");
});

it("records a refused take (custody_full) as a typed refusal and releases the result the session will not take", async () => {
  const { result, transport } = await session((command) => command === "take_preview_mechanics_job_result"
    ? Promise.reject({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "custody_full", delivered: 4, limit: 4 }, pool: null })
    : undefined);
  await act(async () => { await result.current.results.handleRun(); });
  expect(result.current.results.solveJob.state).toBe("refused");
  expect(result.current.results.solveJob.resource.refusal).toEqual({ kind: "custody_full", delivered: 4, limit: 4 });
  expect(result.current.results.result).toBeNull();
  expect(transport.custody()).toEqual([]);
  await waitFor(() => expect(commands().filter((command) => command === "dispose_preview_mechanics_result")).toHaveLength(1));
  expect(transport.disposed()).toHaveLength(1);
});

it("releases a delivered result when a new solve replaces it, when reference inspection clears it, and when the session ends", async () => {
  const { result, transport, unmount } = await session();
  await act(async () => { await result.current.results.handleRun(); });
  await waitFor(() => expect(result.current.results.currentSolvedResult).not.toBeNull());
  const first = result.current.results.result!;
  const firstJob = deliveredMechanicsResultJob(first)!;
  expect(transport.custody()).toEqual([firstJob]);
  expect(hasNativeMechanicsInvocation(first, result.current.model.model, "sparse_interactive")).toBe(true);

  await act(async () => { await result.current.results.handleRun(); });
  await waitFor(() => expect(result.current.results.currentSolvedResult).not.toBeNull());
  const second = result.current.results.result!;
  expect(second).not.toBe(first);
  const secondJob = deliveredMechanicsResultJob(second)!;
  await waitFor(() => expect(transport.disposed()).toEqual([firstJob]));
  expect(transport.custody()).toEqual([secondJob]);

  await act(async () => { await result.current.results.handleInspectBundledReference(); });
  expect(result.current.results.result).toBeNull();
  await waitFor(() => expect(transport.custody()).toEqual([]));
  expect(transport.disposed()).toEqual([firstJob, secondJob]);

  await act(async () => { await result.current.results.handleRun(); });
  await waitFor(() => expect(result.current.results.result).not.toBeNull());
  const thirdJob = deliveredMechanicsResultJob(result.current.results.result)!;
  expect(transport.custody()).toEqual([thirdJob]);
  unmount();
  await waitFor(() => expect(transport.custody()).toEqual([]));
  expect(commands().filter((command) => command === "dispose_preview_mechanics_result")).toHaveLength(3);
});

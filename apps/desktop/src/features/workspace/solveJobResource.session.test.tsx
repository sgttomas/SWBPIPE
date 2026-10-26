import { act, fireEvent, render, renderHook, waitFor, within } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
// This file verifies the memory-pool session path (queue reasons, typed
// refusals, grant recording and native custody), not the preview-physics
// reader: that reader's verdict and the standing read from it are held
// passing here, as LoadReferenceStatesBlock.joinedGuard.test.tsx holds its
// standing, so the cases do not depend on which bundled reader build is
// present. The reader's own verdicts are covered by the reader suites.
vi.mock("../results/resultReader", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../results/resultReader")>();
  return {
    ...actual,
    readerValidate: (...args: Parameters<typeof actual.readerValidate>) => args[0] === "preview-physics" ? {} : actual.readerValidate(...args),
  };
});
vi.mock("../results/resultStanding", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../results/resultStanding")>();
  return { ...actual, numericalResultStanding: () => ({ contract: "preview_physics", status: "integrity_checked", eligible: true, findings: [] }) };
});
import modelFixture from "../../../../../core/product_physics/tests/fixtures/preview_physics_invented_model.json";
import sourceFixture from "../../../../../fixtures/results/preview_physics_invented_sparse.json";
import { createSolveJobTransport, REPLAY_WORKER_PEAK_FOOTPRINT_BYTES, REPLAY_WORKER_PEAK_GRANT_BYTES, solveBytes } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { deliveredMechanicsResultJob, hasNativeMechanicsInvocation, loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { verifyAnalysisRunRecord } from "../../services/analysisRunCompatibility";
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
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

let sessionSequence = 0;
type Answer = Parameters<typeof createSolveJobTransport>[0];
const replayFixture: Answer = (request) => {
  if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model: modelFixture, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
  return solveBytes(sourceFixture);
};
async function session(hook: Hook = () => undefined, { tauri = true, answer = replayFixture }: { tauri?: boolean; answer?: Answer } = {}) {
  const storage = await getLocalStorageCapability();
  const knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport(answer, { jobIdPrefix: `unit-memory-session:${++sessionSequence}` });
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
  if (tauri) (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
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

it("shows the native queue reason while the job waits, then records the grant in the completed run's solve-job record", async () => {
  let release!: () => void;
  const released = new Promise<void>((resolve) => { release = resolve; });
  let polls = 0;
  const { result, transport } = await session((command, _args, _options, forward) => {
    if (command !== "poll_preview_mechanics_job") return undefined;
    polls += 1;
    // A queued job's worker has not started, so it reports no worker grant yet.
    if (polls === 1) return forward().then((answer) => ({ ...(answer as object), state: "queued", result_available: false, result_bytes: null,
      resource: { ...(answer as { resource: object }).resource, worker_grant_bytes: null, worker_peak_grant_bytes: null, worker_peak_footprint_bytes: null },
      queue_reason: { kind: "headroom_insufficient", required_bytes: 12 * GiB, available_bytes: 7 * GiB } }));
    return released.then(forward);
  });
  let running!: Promise<void>;
  act(() => { running = result.current.results.handleRun(); });
  await waitFor(() => expect(result.current.results.solveJob.resource.queue_reason).not.toBeNull());
  const waiting = result.current.results.solveJob;
  expect(waiting.state).toBe("queued");
  expect(waiting.resource).toMatchObject({ source: "native_job_receipt", invocation_budget_bytes: 12 * GiB, pool_bytes: 12 * GiB, route_pricing: "ordinary_worker_capped", worker_grant_bytes: null, worker_peak_grant_bytes: null });
  const view = renderSolvePanel(result.current);
  expect(within(view.container).getByTestId("solve-job-queue-reason")).toHaveTextContent("Waiting in the queue: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 12 GiB, 7 GiB available.");
  // A queue wait is not a refusal of the job: the queue's own retry is named, and no refusal is shown.
  expect(within(view.container).getByTestId("solve-job-queue-reason")).toHaveTextContent("The queue retries by itself and starts the solve once the pool admits it; Cancel preview stops the wait.");
  expect(within(view.container).queryByTestId("solve-job-resource-refusal")).toBeNull();
  expect(within(view.container).queryByTestId("solve-job-refusal-retry")).toBeNull();
  expect(within(view.container).getByTestId("solve-job-summary")).toHaveTextContent("state=queued");
  view.unmount();
  await act(async () => { release(); await running; });
  await waitFor(() => expect(result.current.results.currentSolvedResult).not.toBeNull());
  const done = result.current.results.solveJob;
  expect(done.state).toBe("completed");
  // The completed job's final status reports the worker's peak grant; its worker has exited and holds none.
  expect(done.resource).toMatchObject({ invocation_budget_bytes: 12 * GiB, pool_bytes: 12 * GiB, route_pricing: "ordinary_worker_capped",
    worker_grant_bytes: null, worker_peak_grant_bytes: REPLAY_WORKER_PEAK_GRANT_BYTES, worker_peak_footprint_bytes: REPLAY_WORKER_PEAK_FOOTPRINT_BYTES, queue_reason: null, refusal: null });
  expect(done.events.map((event) => event.event_id)).toEqual(["solve-preview-queued", "solve-preview-queued-waiting", "solve-preview-completed"]);
  // The poll never carries the result: it is taken once, and is still held by the session.
  expect(commands().filter((command) => command === "take_preview_mechanics_job_result")).toHaveLength(1);
  expect(transport.custody()).toHaveLength(1);
  // The solve-job JSON (the run's provenance export) carries the grant, outside the
  // model document; it is read through the browser download route here.
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  const panel = renderSolvePanel(result.current);
  expect(within(panel.container).getByTestId("solve-job-resource").querySelector("strong")).toHaveTextContent("budget=12 GiB (intake and solve worker cap); pool=12 GiB; route=ordinary_worker_capped; worker grant=none held; peak=256 MiB; peak footprint=192 MiB; refusal=none");
  fireEvent.click(within(panel.container).getByTestId("solve-job-export-link-local-private-intent"));
  const href = within(panel.container).getByTestId("solve-job-export-link").getAttribute("href")!;
  const packet = JSON.parse(decodeURIComponent(href.split(",", 2)[1]));
  expect(done.resource.ingress_reserved_bytes).toEqual(expect.any(Number));
  expect(packet.resource).toMatchObject({ source: "native_job_receipt", invocation_budget_bytes: 12 * GiB, pool_bytes: 12 * GiB, route_pricing: "ordinary_worker_capped",
    ingress_reserved_bytes: done.resource.ingress_reserved_bytes, ordinary_solve_capped_by: "worker_grant", worker_grant_bytes: null,
    worker_peak_grant_bytes: REPLAY_WORKER_PEAK_GRANT_BYTES, worker_peak_footprint_bytes: REPLAY_WORKER_PEAK_FOOTPRINT_BYTES, refusal: null });
  expect(packet.resource).not.toHaveProperty("ordinary_route_byte_bound_claimed");
  // The export carries the cancellation scope the native side reported, not a fixed string.
  expect(packet.cancellation.cancellation_scope).toBe("unit_transport_replay_not_native_ui_qualification");
  // The export names the run it audits, so the grant is recoverable for that run.
  const record = result.current.results.analysisRun!;
  expect(packet.analysis_run_ref).toEqual({ object_type: "AnalysisRun", ref: record.analysis_run.run_id });
  expect(JSON.stringify(result.current.model.model)).not.toMatch(/budget|pool_bytes|ordinary_worker_capped|worker_grant/);
  // The persisted, checksummed analysis record carries no host-dependent member:
  // the budget, pool and pricing stay in the solve-job record only.
  expect(JSON.stringify(record)).not.toMatch(/invocation_budget_bytes|pool_bytes|ordinary_worker_capped|worker_grant|worker_peak|execution_resources/);
  expect(await verifyAnalysisRunRecord(record)).toBe("match");
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
  expect(job.resource).toMatchObject({ invocation_budget_bytes: 12 * GiB, pool_bytes: 12 * GiB, route_pricing: "ordinary_worker_capped", refusal });
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
  // A refused run makes no analysis record; its typed refusal and budget are recoverable from the solve-job export.
  expect(result.current.results.analysisRun).toBeNull();
  view.unmount();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  const panel = renderSolvePanel(result.current);
  fireEvent.click(within(panel.container).getByTestId("solve-job-export-link-local-private-intent"));
  const href = within(panel.container).getByTestId("solve-job-export-link").getAttribute("href")!;
  const packet = JSON.parse(decodeURIComponent(href.split(",", 2)[1]));
  expect(packet.resource).toMatchObject({ invocation_budget_bytes: 12 * GiB, pool_bytes: 12 * GiB, route_pricing: "ordinary_worker_capped", refusal,
    refusal_reason: "The memory pool is fully reserved: this request needs 12 GiB, and 40 GiB of the 48 GiB pool is already reserved." });
  expect(packet.analysis_run_ref).toBeNull();
});

it("records a solve refused while its worker grew: the typed refusal with its nested cause and the worker's peak grant, in the panel and the export", async () => {
  const refusal = { kind: "worker_growth_refused", granted_bytes: 3 * GiB, requested_bytes: 5 * GiB,
    cause: { kind: "pool_exhausted", requested_bytes: 2 * GiB, reserved_bytes: 11 * GiB, pool_bytes: 12 * GiB } };
  let polls = 0;
  const { result } = await session((command, _args, _options, forward) => {
    if (command !== "poll_preview_mechanics_job") return undefined;
    polls += 1;
    return forward().then((answer) => {
      const base = answer as { resource: Record<string, unknown> };
      // The worker runs with a 3 GiB grant, then its growth to 5 GiB is refused and the job ends.
      return polls === 1
        ? { ...base, state: "running", result_available: false, result_bytes: null, resource: { ...base.resource, worker_grant_bytes: 3 * GiB, worker_peak_grant_bytes: 3 * GiB, worker_peak_footprint_bytes: 2.75 * GiB } }
        : { ...base, state: "refused", result_available: false, result_bytes: null, resource_refusal: refusal, resource: { ...base.resource, worker_grant_bytes: null, worker_peak_grant_bytes: 3 * GiB, worker_peak_footprint_bytes: 3.25 * GiB } };
    });
  });
  await act(async () => { await result.current.results.handleRun(); });
  const job = result.current.results.solveJob;
  expect(job.state).toBe("refused");
  expect(job.error_message).toBeNull();
  // The measured footprint briefly exceeded the peak grant while the growth was refused.
  expect(job.resource).toMatchObject({ route_pricing: "ordinary_worker_capped", worker_grant_bytes: null, worker_peak_grant_bytes: 3 * GiB, worker_peak_footprint_bytes: 3.25 * GiB, refusal });
  expect(job.events.map((event) => event.event_id)).toEqual(["solve-preview-queued", "solve-preview-running", "solve-preview-resource-refused"]);
  expect(commands()).not.toContain("take_preview_mechanics_job_result");
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  const panel = renderSolvePanel(result.current);
  const sentence = "The solve needed more memory (5 GiB) than could be reserved while it ran: The memory pool is fully reserved: this request needs 2 GiB, and 11 GiB of the 12 GiB pool is already reserved.";
  expect(within(panel.container).getByTestId("solve-job-resource-refusal")).toHaveTextContent(`Refused by the native memory pool: ${sentence} No result was published.`);
  expect(within(panel.container).queryByTestId("solve-job-worker-failure")).toBeNull();
  expect(within(panel.container).getByTestId("solve-job-resource").querySelector("strong")).toHaveTextContent("worker grant=none held; peak=3 GiB; peak footprint=3.25 GiB; refusal=worker_growth_refused");
  fireEvent.click(within(panel.container).getByTestId("solve-job-export-link-local-private-intent"));
  const href = within(panel.container).getByTestId("solve-job-export-link").getAttribute("href")!;
  const packet = JSON.parse(decodeURIComponent(href.split(",", 2)[1]));
  expect(packet.resource).toMatchObject({ route_pricing: "ordinary_worker_capped", ordinary_solve_capped_by: "worker_grant", worker_grant_bytes: null,
    worker_peak_grant_bytes: 3 * GiB, worker_peak_footprint_bytes: 3.25 * GiB, refusal, refusal_reason: sentence });
  expect(packet.analysis_run_ref).toBeNull();
});

it.each([
  ["SOLVE_WORKER_EXITED: worker process ended (signal 9)", "The solve's worker process ended without returning a result. The solve failed; this is not a memory-pool refusal."],
  ["SOLVE_WORKER_PROTOCOL: reply frame truncated", "The solve's worker process returned a reply the app could not read. The solve failed; this is not a memory-pool refusal."]
])("shows a solve worker failure (%s) as a failure, not a refusal", async (message, sentence) => {
  const { result } = await session((command, _args, _options, forward) => command === "poll_preview_mechanics_job"
    ? forward().then((answer) => ({ ...(answer as object), state: "failed", result_available: false, result_bytes: null, error_message: message }))
    : undefined);
  await act(async () => { await result.current.results.handleRun(); });
  const job = result.current.results.solveJob;
  expect(job.state).toBe("failed");
  expect(job.resource.refusal).toBeNull();
  expect(job.error_message).toBe(`Error: ${message}`);
  expect(result.current.results.result).toBeNull();
  expect(commands()).not.toContain("take_preview_mechanics_job_result");
  const view = renderSolvePanel(result.current);
  expect(within(view.container).getByTestId("solve-job-error")).toHaveTextContent(message);
  expect(within(view.container).getByTestId("solve-job-worker-failure")).toHaveTextContent(sentence);
  expect(within(view.container).queryByTestId("solve-job-resource-refusal")).toBeNull();
  expect(within(view.container).getByTestId("readiness-mechanics")).not.toHaveTextContent("refused by the native memory pool");
});

// A hand-edited model document with a misspelled nested key (Agent 0, 2026-10-25: nested records
// closed). The value entry refuses it before anything is solved; this text is the native refusal
// for exactly this edit, pinned against the live value entry by product_physics'
// `tests/silent_drop_guard_runtime.rs` (`the_desktop_hand_edit_is_refused_with_this_text`).
const HAND_EDIT_REFUSAL = "UNKNOWN_MODEL_FIELD: request: unknown field `mill_tolerence` in the section of pipe segment `pipe:P-110` "
  + "(model.pipe_segments[1].section). The model document does not define these keys, so they would be ignored; nothing is solved. "
  + "Correct or remove each key.";

it("refuses a hand-edited nested unknown key at solve with a reason naming the record and path, and publishes nothing", async () => {
  const edited = structuredClone(modelFixture) as typeof modelFixture & { pipe_segments: Array<{ section: Record<string, unknown> }> };
  edited.pipe_segments[1].section.mill_tolerence = { value: 0.0005, unit: "m" };
  const sent: unknown[] = [];
  const { result } = await session((command, _args, _options, forward) => {
    if (command === "open_model_document_file") return Promise.resolve({ outcome: "opened", file_name: "hand-edited.json", document: structuredClone(edited) });
    // The native job ends as the value entry's refusal ends it: failed, with its reason, no result.
    return command === "poll_preview_mechanics_job"
      ? forward().then((answer) => ({ ...(answer as object), state: "failed", result_available: false, result_bytes: null, error_message: HAND_EDIT_REFUSAL }))
      : undefined;
  }, { answer: (request) => { sent.push(request.model); return solveBytes(sourceFixture); } });
  await act(async () => { await result.current.project.handleOpenModelDocument(); });
  // The desktop adopts the document unchanged: its own checks do not drop or refuse the key.
  expect(result.current.model.model).toEqual(edited);
  await act(async () => { await result.current.results.handleRun(); });
  // The misspelled key reaches the solve as authored.
  expect(sent).toEqual([edited]);
  const job = result.current.results.solveJob;
  expect(job.state).toBe("failed");
  expect(job.resource.refusal).toBeNull();
  expect(result.current.results.result).toBeNull();
  expect(commands()).not.toContain("take_preview_mechanics_job_result");
  const view = renderSolvePanel(result.current);
  const reason = within(view.container).getByTestId("solve-job-error");
  expect(reason).toHaveTextContent("unknown field `mill_tolerence` in the section of pipe segment `pipe:P-110` (model.pipe_segments[1].section)");
  expect(reason).toHaveTextContent("nothing is solved. Correct or remove each key.");
  expect(within(view.container).queryByTestId("solve-job-worker-failure")).toBeNull();
  expect(within(view.container).queryByTestId("solve-job-resource-refusal")).toBeNull();
});

it("records a refused start with the pool status the native side reported", async () => {
  const pool = {
    capacity_bytes: 16 * GiB, recommended_pool_bytes: 12 * GiB, pool_bytes: 12 * GiB, pool_above_recommended: false,
    invocation_budget_bytes: 10 * GiB, default_invocation_budget_bytes: 12 * GiB, max_invocation_budget_bytes: 12 * GiB, reserved_bytes: 0,
    reserved_by_purpose: { ingress: 0, worker: 0, result: 0, custody: 0 }, running_workers: 0, queued_jobs: 0, running_jobs: 0, limits: { max_running_workers: 8 },
    retained_results: 0, webview_custody_results: 0, headroom: { available_bytes: null, pressure: "unknown", sample_age_ms: null },
    ordinary_route_pricing: "worker_capped", last_refusal: { kind: "headroom_unknown" }
  };
  const { result } = await session((command) => command === "start_preview_mechanics_job"
    ? Promise.reject({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_unknown" }, pool })
    : undefined);
  await act(async () => { await result.current.results.handleRun(); });
  const job = result.current.results.solveJob;
  expect(job.state).toBe("refused");
  expect(job.resource).toEqual({ source: "native_refusal_pool_status", invocation_budget_bytes: 10 * GiB, pool_bytes: 12 * GiB, route_pricing: null, ingress_reserved_bytes: null,
    worker_grant_bytes: null, worker_peak_grant_bytes: null, worker_peak_footprint_bytes: null, queue_reason: null, refusal: { kind: "headroom_unknown" } });
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

const MiB = 1024 ** 2;
// The smoke's headroom figures (DEL-04-07 memory-budget decision): a 65 MiB grant plus an 8 KiB intake lease against 50.2 MiB free.
const SMOKE_HEADROOM = { kind: "headroom_insufficient", required_bytes: 68165632, available_bytes: 52641792 } as const;
const SMOKE_HEADROOM_CODE = "headroom_insufficient (required_bytes=68165632, available_bytes=52641792)";
const SMOKE_HEADROOM_SENTENCE = "Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 65.01 MiB, 50.2 MiB available.";

it("records a take refused for headroom as refused at the take, with its code and exact bytes, no rows and no run record, and releases the retained result", async () => {
  const { result, transport } = await session((command) => command === "take_preview_mechanics_job_result"
    ? Promise.reject({ code: "RESOURCE_UNAVAILABLE", refusal: SMOKE_HEADROOM, pool: null })
    : undefined);
  await act(async () => { await result.current.results.handleRun(); });
  const job = result.current.results.solveJob;
  expect(job.state).toBe("refused");
  expect(job.error_message).toBeNull();
  expect(job.resource.refusal).toEqual(SMOKE_HEADROOM);
  expect(job.events.map((event) => event.event_id)).toEqual(["solve-preview-queued", "solve-preview-resource-refused-at-take"]);
  expect(job.events.some((event) => event.state === "completed" || event.result_row_count > 0)).toBe(false);
  expect(result.current.results.result).toBeNull();
  expect(result.current.results.analysisRun).toBeNull();
  expect(result.current.results.currentSolvedResult).toBeNull();
  // The session does not retry a refused take: it was asked once, and the retained result is released unread.
  expect(commands().filter((command) => command === "take_preview_mechanics_job_result")).toHaveLength(1);
  await waitFor(() => expect(transport.disposed()).toHaveLength(1));
  expect(transport.custody()).toEqual([]);
  const view = renderSolvePanel(result.current);
  expect(within(view.container).getByTestId("solve-job-summary")).toHaveTextContent("state=refused; events=2; result_rows=0");
  expect(within(view.container).getByTestId("solve-job-resource-refusal")).toHaveTextContent(`Refused by the native memory pool: ${SMOKE_HEADROOM_SENTENCE} No result was published.`);
  expect(within(view.container).getByTestId("solve-job-refusal-retry")).toHaveTextContent(
    `Refused when the finished result was handed to the app; the session does not retry a refused take, so it released the result. Available memory was re-sampled for up to 2 s before this refusal stood. You can run the solve again when more memory is available. Refusal code: ${SMOKE_HEADROOM_CODE}`);
  expect(within(view.container).getByTestId("solve-job-resource").querySelector("strong")).toHaveTextContent("refusal=headroom_insufficient");
  expect(within(view.container).getByTestId("readiness-mechanics")).toHaveTextContent("refused by the native memory pool");
  expect(within(view.container).getByTestId("solve-job-binding")).toHaveTextContent("not generated; result rows=0; hashes=0");
});

it("records a solve refused at the result hand-off (its result lease) as refused there, with its code and exact bytes, and takes nothing", async () => {
  const { result } = await session((command, _args, _options, forward) => command === "poll_preview_mechanics_job"
    ? forward().then((answer) => ({ ...(answer as object), state: "refused", result_available: false, result_bytes: null, resource_refusal: SMOKE_HEADROOM }))
    : undefined);
  await act(async () => { await result.current.results.handleRun(); });
  const job = result.current.results.solveJob;
  expect(job.state).toBe("refused");
  expect(job.resource.refusal).toEqual(SMOKE_HEADROOM);
  expect(job.events.at(-1)).toMatchObject({ event_id: "solve-preview-resource-refused", state: "refused", result_row_count: 0 });
  expect(result.current.results.result).toBeNull();
  expect(result.current.results.analysisRun).toBeNull();
  expect(commands()).not.toContain("take_preview_mechanics_job_result");
  const view = renderSolvePanel(result.current);
  expect(within(view.container).getByTestId("solve-job-summary")).toHaveTextContent("state=refused; events=2; result_rows=0");
  expect(within(view.container).getByTestId("solve-job-resource-refusal")).toHaveTextContent(`Refused by the native memory pool: ${SMOKE_HEADROOM_SENTENCE} No result was published.`);
  expect(within(view.container).getByTestId("solve-job-refusal-retry")).toHaveTextContent(
    `Refused at the result hand-off after the solve finished; the finished result was dropped. Available memory was re-sampled for up to 2 s before this refusal stood. You can run the solve again when more memory is available. Refusal code: ${SMOKE_HEADROOM_CODE}`);
});

it("shows a worker growth refusal as before, now with its exact code and retry path (the smoke's figures)", async () => {
  const refusal = { kind: "worker_growth_refused", granted_bytes: 68157440, requested_bytes: 72598008,
    cause: { kind: "headroom_insufficient", required_bytes: 67108864, available_bytes: 0 } };
  const { result } = await session((command, _args, _options, forward) => command === "poll_preview_mechanics_job"
    ? forward().then((answer) => ({ ...(answer as object), state: "refused", result_available: false, result_bytes: null, resource_refusal: refusal,
      resource: { ...(answer as { resource: object }).resource, worker_grant_bytes: null, worker_peak_grant_bytes: 65 * MiB } }))
    : undefined);
  await act(async () => { await result.current.results.handleRun(); });
  expect(result.current.results.solveJob.state).toBe("refused");
  expect(result.current.results.solveJob.resource.refusal).toEqual(refusal);
  expect(commands()).not.toContain("take_preview_mechanics_job_result");
  const view = renderSolvePanel(result.current);
  expect(within(view.container).getByTestId("solve-job-summary")).toHaveTextContent("state=refused; events=2; result_rows=0");
  // The panel's refusal text is unchanged.
  expect(within(view.container).getByTestId("solve-job-resource-refusal").textContent).toBe(
    "Refused by the native memory pool: The solve needed more memory (69.23 MiB) than could be reserved while it ran: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 64 MiB, 0 MiB available. No result was published.");
  expect(within(view.container).getByTestId("solve-job-resource").querySelector("strong")).toHaveTextContent("refusal=worker_growth_refused");
  expect(within(view.container).getByTestId("solve-job-refusal-retry")).toHaveTextContent(
    "Refused while the solve ran: its worker process was stopped, with no fallback to another solver mode or route. Available memory was re-sampled for up to 2 s before this refusal stood. You can run the solve again when more memory is available. Refusal code: worker_growth_refused (granted_bytes=68157440, requested_bytes=72598008); cause headroom_insufficient (required_bytes=67108864, available_bytes=0)");
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

it("releases earlier native custody once when the session starts in the Tauri runtime, before its first solve", async () => {
  const { result, transport, unmount } = await session();
  await waitFor(() => expect(transport.disposeAllCount()).toBe(1));
  await act(async () => { await result.current.results.handleRun(); });
  await waitFor(() => expect(result.current.results.currentSolvedResult).not.toBeNull());
  const sequence = commands().filter((command) => command.includes("preview_mechanics"));
  expect(sequence[0]).toBe("dispose_all_preview_mechanics_results");
  expect(sequence.indexOf("start_preview_mechanics_job")).toBeGreaterThan(0);
  // A second session in the same webview does not release again: it may already hold results.
  unmount();
  renderHook(() => useWorkspaceSession());
  await act(async () => { await Promise.resolve(); });
  expect(commands().filter((command) => command === "dispose_all_preview_mechanics_results")).toHaveLength(1);
});

it("issues no session-start release in the browser fixture mode", async () => {
  const { result } = await session(undefined, { tauri: false });
  await act(async () => { await Promise.resolve(); });
  expect(result.current.model.model).not.toBeNull();
  expect(commands()).not.toContain("dispose_all_preview_mechanics_results");
});

it("abandons a started job whose poll fails: requests its cancellation, then releases anything it retains", async () => {
  const { result, transport } = await session((command) => command === "poll_preview_mechanics_job"
    ? Promise.reject(new Error("invented poll IPC failure"))
    : undefined);
  await act(async () => { await result.current.results.handleRun(); });
  expect(result.current.results.solveJob.state).toBe("failed");
  expect(result.current.results.result).toBeNull();
  await waitFor(() => expect(commands()).toContain("dispose_preview_mechanics_result"));
  const jobCommands = commands().filter((command) => command !== "dispose_all_preview_mechanics_results" && command.includes("preview_mechanics"));
  expect(jobCommands).toEqual(["start_preview_mechanics_job", "poll_preview_mechanics_job", "cancel_preview_mechanics_job", "dispose_preview_mechanics_result"]);
  const jobId = result.current.results.solveJob.job_id;
  const cancel = invokeMock.mock.calls.find(([command]) => command === "cancel_preview_mechanics_job")![1] as { jobId: string };
  expect(jobId).toContain(cancel.jobId);
  expect(transport.custody()).toEqual([]);
  expect(commands()).not.toContain("take_preview_mechanics_job_result");
});

it("releases a completed job's result when the take fails for a reason other than a typed refusal", async () => {
  const { result, transport } = await session((command) => command === "take_preview_mechanics_job_result"
    ? Promise.reject(new Error("invented take IPC failure"))
    : undefined);
  await act(async () => { await result.current.results.handleRun(); });
  expect(result.current.results.solveJob.state).toBe("failed");
  await waitFor(() => expect(transport.disposed()).toHaveLength(1));
  expect(commands()).not.toContain("cancel_preview_mechanics_job");
  expect(transport.custody()).toEqual([]);
});

it("keeps the run's solve-job grant, and adds none to the record, when a rule-check aggregate revises its analysis record", async () => {
  const { result } = await session();
  await act(async () => { await result.current.results.handleRun(); });
  await waitFor(() => expect(result.current.results.analysisRun).not.toBeNull());
  const first = result.current.results.analysisRun!;
  const grant = result.current.results.solveJob.resource;
  expect(grant).toMatchObject({ source: "native_job_receipt", invocation_budget_bytes: 12 * GiB, route_pricing: "ordinary_worker_capped", worker_peak_grant_bytes: REPLAY_WORKER_PEAK_GRANT_BYTES });
  const { projectSessionGeneration, uiModelRevision } = result.current.model;
  await act(async () => {
    await result.current.results.handleRuleCheckAggregate("USER_RULE_CHECKED", {
      projectSessionGeneration, modelRevision: uiModelRevision,
      result: result.current.results.currentSolvedResult, inputManifest: result.current.results.inputManifest
    });
  });
  await waitFor(() => expect(result.current.results.analysisRun).not.toBe(first));
  const revised = result.current.results.analysisRun!;
  expect(revised.analysis_run.analysis_status).toContain("USER_RULE_CHECKED");
  expect(result.current.results.solveJob.resource).toEqual(grant);
  expect(JSON.stringify(revised)).not.toMatch(/invocation_budget_bytes|pool_bytes|ordinary_worker_capped|worker_grant|worker_peak|execution_resources/);
  expect(await verifyAnalysisRunRecord(revised)).toBe("match");
});

it("abandons a job whose run is superseded while its start is in flight, so its result is never left retained", async () => {
  let answerStart!: () => void;
  const startGate = new Promise<void>((resolve) => { answerStart = resolve; });
  const { result, transport } = await session((command, _args, _options, forward) => command === "start_preview_mechanics_job"
    ? startGate.then(forward)
    : undefined);
  let running!: Promise<void>;
  act(() => { running = result.current.results.handleRun(); });
  await waitFor(() => expect(commands()).toContain("start_preview_mechanics_job"));
  // The run is superseded (reference inspection retires it) before the start answers.
  await act(async () => { await result.current.results.handleInspectBundledReference(); });
  await act(async () => { answerStart(); await running; });
  const { job_id: jobId } = await (invokeMock.mock.results[commands().indexOf("start_preview_mechanics_job")].value as Promise<{ job_id: string }>);
  const forJob = () => invokeMock.mock.calls.filter(([, args]) => (args as { jobId?: string } | undefined)?.jobId === jobId).map(([command]) => command);
  await waitFor(() => expect(forJob()).toContain("dispose_preview_mechanics_result"));
  expect(forJob()).toEqual(["cancel_preview_mechanics_job", "dispose_preview_mechanics_result"]);
  expect(commands()).not.toContain("take_preview_mechanics_job_result");
  expect(transport.custody()).toEqual([]);
  expect(result.current.results.result).toBeNull();
});

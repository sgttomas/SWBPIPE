import { act, render, renderHook, waitFor, within } from "@testing-library/react";
import { REPLAY_RESOURCE_GRANT, resultArrayBuffer, solveBytes, startRequestModel } from "../../test/nativeMechanicsReplay";
import { isDeepStrictEqual } from "node:util";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import modelFixture from "../../../../../core/product_physics/tests/fixtures/preview_physics_invented_model.json";
import sourceFixture from "../../../../../fixtures/results/preview_physics_invented_sparse.json";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge } from "../../services/previewService";
import { DesignWorkspacePanel } from "../design-workspace/DesignWorkspacePanel";
import { SolvePanel } from "../solve/SolvePanel";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";

// Unit transport replay of recorded producer bytes through mocked IPC: the
// preview-physics-1 sparse fixture is actual producer output (T0R S2a) for the
// invented demo model without its legacy pressure (preview_physics_invented_model.json).
// T0R: precision-1 is historical and never Current, so this Current witness moved.
// This is NOT a native UI qualification witness. Current standing comes only
// from the session's own solve path and qualification gate (handleRun).
const modelUnits = "model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC";
// A1: the derived model omits joint component:C-150 (refused on this route), so no
// N*m/rad row remains in this Current witness.
const recordedUnits = "MPa,N,N*m,N/m,Pa,boolean,count,m,mm,mode_code,rad,state_code";

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

async function recordedProducerSession() {
  // Capture the real browser bootstrap records before installing the replay.
  const storage = await getLocalStorageCapability();
  const knowledge = await loadDesignKnowledge();
  const jobId = "unit-transport-replay:invented-preview-physics-1-sparse";
  const scope = "unit_transport_replay_not_native_ui_qualification";
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(modelFixture);
    if (command === "start_preview_mechanics_job") {
      // Replay only the exact recorded input and mode; anything else fails.
      const sent = startRequestModel(args, options);
      if (!isDeepStrictEqual({ model: sent.model, mode: sent.mode }, { model: modelFixture, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
      return { job_id: jobId, backend_cancellation_token: `${jobId}:token`, state: "queued", cancellation_scope: scope, resource: { ...REPLAY_RESOURCE_GRANT, ingress_reserved_bytes: 4096 } };
    }
    if (command === "poll_preview_mechanics_job" && isDeepStrictEqual(args, { jobId })) {
      return { job_id: jobId, state: "completed", queue_reason: null, cancellation_requested: false, cancellation_status: "not_requested",
        cancellation_scope: scope, result_available: true, result_bytes: solveBytes(sourceFixture).length, error_message: null, resource_refusal: null, resource: { ...REPLAY_RESOURCE_GRANT, ingress_reserved_bytes: 4096 } };
    }
    if (command === "take_preview_mechanics_job_result" && isDeepStrictEqual(args, { jobId })) return resultArrayBuffer(solveBytes(sourceFixture));
    if (command === "dispose_preview_mechanics_result") return null;
    throw new Error(`REPLAY_COMMAND_UNSUPPORTED: ${command}`);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(hook.result.current.model.modelHash).not.toBeNull());
  return hook;
}

// The two panels with the props App.tsx passes them: SolvePanel receives the
// session result, DesignWorkspacePanel the qualified Current result.
function renderPanels(session: WorkspaceSession) {
  const { model, knowledge } = session.model;
  const { result, currentSolvedResult, analysisRun, comparison, proposal, selectedReviewTarget, running, solveJob, solverMode } = session.results;
  const solve = render(<SolvePanel analysisRun={analysisRun} model={model!} result={result} running={running} solveJob={solveJob}
    solverMode={solverMode} onCancel={() => {}} onRun={() => {}} onSolverModeChange={() => {}} />);
  const design = render(<DesignWorkspacePanel model={model!} knowledge={knowledge} result={currentSolvedResult} analysisRun={analysisRun}
    comparison={comparison} editorIntents={session.operations.editorIntents} proposal={proposal} selectedReviewTarget={selectedReviewTarget} />);
  const line = (view: { container: HTMLElement }, testId: string) =>
    within(view.container).getByTestId(testId).querySelector("strong")?.textContent;
  return { solve, design, line };
}

it("reads results=none in the solve and design unit lines without a Current result", async () => {
  const { result } = await recordedProducerSession();
  expect(result.current.results.result).toBeNull();
  expect(result.current.results.currentSolvedResult).toBeNull();
  const { solve, design, line } = renderPanels(result.current);
  expect(line(solve, "solve-job-unit-policy")).toBe(`${modelUnits}; results=none; rows=0; conversion=false`);
  expect(line(design, "design-workspace-units")).toBe(`${modelUnits}; results=none; comparison=none; conversion=false`);
});

it("reports a solved result's recorded units, including Pa, N*m and N/m, unconverted, and keeps the Sensitive demo out of Current", async () => {
  // K-D5 (DEL-04-01 kd5-member-relative-bound.md §5): the invented demo has nonlinear supports, so its
  // converged cases are published NUMERICAL_INTEGRITY_SENSITIVE. The solve panel reports the received rows'
  // units unconverted; the qualification gate admits no Current result, so the design line reads none.
  const { result } = await recordedProducerSession();
  await act(async () => { await result.current.results.handleRun(); });
  await waitFor(() => expect(result.current.results.result).not.toBeNull());
  const solved = result.current.results.result;
  expect(result.current.results.currentSolvedResult).toBeNull();
  expect(solved!.numerical_quality!.status).toBe("sensitive");
  expect(solved!.results).toHaveLength(625);
  expect([...new Set(solved!.results.map((row) => row.unit))].sort().join(",")).toBe(recordedUnits);
  expect(solved).toEqual(sourceFixture); // received producer bytes, unchanged

  const { solve, design, line } = renderPanels(result.current);
  expect(line(solve, "solve-job-unit-policy")).toBe(`${modelUnits}; results=${recordedUnits}; rows=625; conversion=false`);
  expect(within(solve.container).getByTestId("solve-job-summary")).toHaveTextContent("state=completed");
  expect(within(solve.container).getByTestId("solve-job-summary")).toHaveTextContent("result_rows=625");
  expect(line(design, "design-workspace-units")).toBe(`${modelUnits}; results=none; comparison=none; conversion=false`);
});

import { act, render, renderHook, waitFor, within } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import modelFixture from "../../../../../fixtures/product_preview/invented_preview_model.json";
import sourceFixture from "../../../../../fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json";
import { getLocalStorageCapability } from "../../services/projectService";
import { hasNativeMechanicsInvocation, loadDesignKnowledge } from "../../services/previewService";
import { DesignWorkspacePanel } from "../design-workspace/DesignWorkspacePanel";
import { SolvePanel } from "../solve/SolvePanel";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";

// Unit transport replay of recorded producer bytes through mocked IPC: the
// precision-1 sparse fixture is verbatim product stdout for the unchanged
// invented_preview_model.json (fixtures/product_preview/PRECISION_FIXTURES.md).
// This is NOT a native UI qualification witness. Current standing comes only
// from the session's own solve path and qualification gate (handleRun).
const modelUnits = "model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC";
const recordedUnits = "MPa,N,N*m,N*m/rad,N/m,boolean,count,m,mm,mode_code,rad,state_code";

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
  const jobId = "unit-transport-replay:invented-precision-1-sparse";
  const scope = "unit_transport_replay_not_native_ui_qualification";
  invokeMock.mockImplementation(async (command: string, args?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(modelFixture);
    if (command === "start_preview_mechanics_job_with_solver_mode") {
      // Replay only the exact recorded input and mode; anything else fails.
      if (!isDeepStrictEqual(args, { model: modelFixture, solverMode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
      return { job_id: jobId, backend_cancellation_token: `${jobId}:token`, state: "queued", cancellation_scope: scope };
    }
    if (command === "poll_preview_mechanics_job" && isDeepStrictEqual(args, { jobId })) {
      return { job_id: jobId, state: "completed", cancellation_requested: false, cancellation_status: "not_requested",
        cancellation_scope: scope, result: structuredClone(sourceFixture), error_message: null };
    }
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

it("reports a qualified Current result's recorded units, including N*m/rad and N/m, unconverted in the solve and design unit lines", async () => {
  const { result } = await recordedProducerSession();
  await act(async () => { await result.current.results.handleRun(); });
  await waitFor(() => expect(result.current.results.currentSolvedResult).not.toBeNull());
  const model = result.current.model.model, currentSolvedResult = result.current.results.currentSolvedResult;
  expect(result.current.results.result).toBe(currentSolvedResult);
  expect(hasNativeMechanicsInvocation(currentSolvedResult, model, "sparse_interactive")).toBe(true);
  expect(currentSolvedResult!.results).toHaveLength(830);
  expect([...new Set(currentSolvedResult!.results.map((row) => row.unit))].sort().join(",")).toBe(recordedUnits);
  expect(currentSolvedResult).toEqual(sourceFixture); // received producer bytes, unchanged

  const { solve, design, line } = renderPanels(result.current);
  expect(line(solve, "solve-job-unit-policy")).toBe(`${modelUnits}; results=${recordedUnits}; rows=830; conversion=false`);
  expect(within(solve.container).getByTestId("solve-job-summary")).toHaveTextContent("state=completed");
  expect(within(solve.container).getByTestId("solve-job-summary")).toHaveTextContent("result_rows=830");
  // comparison= lists only the units of rows matched between the default comparison bases (load:L-100 vs combination:C-OPER-ALT).
  expect(line(design, "design-workspace-units"))
    .toBe(`${modelUnits}; results=${recordedUnits}; comparison=MPa,N,N*m,mm,rad; conversion=false`);
  expect(within(design.container).getByTestId("design-workspace-current")).toHaveTextContent("result_rows=830");
  expect(currentSolvedResult).toEqual(sourceFixture);
});

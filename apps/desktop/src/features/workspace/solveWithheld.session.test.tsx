// The Solve command withheld in the desktop session (DEL-07-07 REQ-008, AC-005;
// DEL-07-04 AC-001, VER-001): an invented model lacking one solve-required value
// (pipe P-100's material reference, emptied from the bundled invented model). The
// session runs the actual wasm model checks (DEL-04-06 REQ-013); the expected
// finding is the one they return for the omitted value.
import { act, fireEvent, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import * as hashService from "../../services/hashService";
import type { PreviewModel } from "../../types";
import { buildBulkChangeBatch } from "../bulk-change/bulkChange";
import * as modelChecks from "../diagnostics/modelChecks";
import { SolvePanel } from "../solve/SolvePanel";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";

afterEach(() => { vi.restoreAllMocks(); });

const PIPE = "pipe:P-100";
const realRunModelChecks = modelChecks.runModelChecks;
/** Holds each of the next `count` model-check runs until its release, then runs the real checks. */
function holdChecks(count: number) {
  const releases: Array<() => void> = [];
  const spy = vi.spyOn(modelChecks, "runModelChecks");
  for (let index = 0; index < count; index += 1) {
    const held = new Promise<void>((resolve) => releases.push(resolve));
    spy.mockImplementationOnce(async (...args) => { await held; return realRunModelChecks(...args); });
  }
  return releases;
}

async function session() {
  const loaded = JSON.parse(JSON.stringify(await previewService.loadPreviewModel())) as PreviewModel;
  const material = loaded.pipe_segments.find((pipe) => pipe.id === PIPE)!.material;
  loaded.pipe_segments.find((pipe) => pipe.id === PIPE)!.material = "";
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(loaded);
  const start = vi.spyOn(previewService, "startPreviewMechanicsJob").mockRejectedValue(new Error("INVENTED-START-OBSERVED"));
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return { result: hook.result, start, material, unmount: hook.unmount };
}
type Result = Awaited<ReturnType<typeof session>>["result"];

/** The session cells a solve clears or replaces when it starts. */
function solveCells(state: WorkspaceSession) {
  const { result, analysisRun, inputManifest, solveJob, solveProof, ruleCheckAggregate, running } = state.results;
  return { result, analysisRun, inputManifest, solveJob, solveProof, ruleCheckAggregate, running };
}

function withheldCodes(result: Result) {
  const gate = result.current.results.solveGate;
  return gate.kind === "withheld" ? gate.diagnostics : null;
}

async function applyMaterial(result: Result, materialId: string) {
  const batch = await buildBulkChangeBatch(result.current.model.model!, [{ type: "pipe", id: PIPE }], { kind: "material", materialId }, hashService.canonicalJsonString, "solve-withheld-supply");
  await act(async () => { await result.current.operations.handleQueueOperationBatch(batch); });
  await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], false); });
  await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
}

it("withholds Solve on the missing value, names the finding and its entity, changes nothing, and follows the value through apply, undo and redo", async () => {
  const { result, start, material } = await session();
  const model = result.current.model.model!;
  const bytes = JSON.stringify(model);
  const cells = solveCells(result.current);
  expect(result.current.chrome.issuesDrawerOpen).toBe(false);

  // Solve: the checks run on the request (none is current), and the solve is withheld.
  await act(async () => { await result.current.results.handleRun(); });
  expect(start).not.toHaveBeenCalled();
  expect(solveCells(result.current)).toEqual(cells);
  expect(result.current.results.solveJob).toBe(cells.solveJob);
  expect(result.current.model.model).toBe(model);
  expect(JSON.stringify(result.current.model.model)).toBe(bytes);
  expect(result.current.chrome.issuesDrawerOpen).toBe(true);
  const standing = withheldCodes(result)!;
  const reading = result.current.results.solveGate;
  expect(reading.kind === "withheld" && reading.model).toBe(model);
  expect(standing.map((item) => [item.code, item.severity, item.affected_refs?.[0]])).toEqual([["MATERIAL_INPUT_MISSING", "blocking", PIPE]]);

  // The Solve panel names the finding; choosing it selects the pipe.
  const view = render(
    <SolvePanel model={result.current.model.model!} result={null} analysisRun={null} running={false} solveJob={result.current.results.solveJob}
      solverMode="sparse_interactive" onRun={() => {}} onCancel={() => {}} onSolverModeChange={() => {}}
      solveGate={result.current.results.solveGate} onSelectEntity={(entity) => { result.current.selection.handleSelectEntity(entity); }} />
  );
  expect(screen.getByRole("button", { name: "Run mechanics preview (withheld)" })).toHaveAccessibleDescription(/1 solve-blocking finding stands/);
  const notice = screen.getByRole("region", { name: "Solve is withheld" });
  act(() => { fireEvent.click(within(notice).getByRole("button", { name: /MATERIAL_INPUT_MISSING blocking/ })); });
  await waitFor(() => expect(result.current.selection.selection).toMatchObject({ type: "pipe", id: PIPE }));
  view.unmount();

  // Supplying the value by an applied operation clears the finding by itself.
  await applyMaterial(result, material);
  expect(result.current.model.model!.pipe_segments.find((pipe) => pipe.id === PIPE)!.material).toBe(material);
  await waitFor(() => expect(result.current.results.solveGate.kind).toBe("open"));

  // Undo brings the same finding back, by itself.
  act(() => { result.current.operations.handleUndoSessionModelEdit(); });
  await waitFor(() => expect(withheldCodes(result)).toEqual(standing));
  expect(JSON.stringify(result.current.model.model)).toBe(bytes);
  await act(async () => { await result.current.results.handleRun(); });
  expect(start).not.toHaveBeenCalled();

  // Redo supplies it again, and the solve starts.
  act(() => { result.current.operations.handleRedoSessionModelEdit(); });
  await waitFor(() => expect(result.current.results.solveGate.kind).toBe("open"));
  await act(async () => { await result.current.results.handleRun(); });
  expect(start).toHaveBeenCalledOnce();
});

/** Holds the next model-check run until released, then runs the real checks. */
function holdNextCheck() {
  const real = modelChecks.runModelChecks;
  let release: () => void = () => {};
  const gate = new Promise<void>((resolve) => { release = resolve; });
  vi.spyOn(modelChecks, "runModelChecks").mockImplementationOnce(async (...args) => { await gate; return real(...args); });
  return () => release();
}

it.each(["cancel", "model change"] as const)("a %s during the Solve request's check acts while the check is pending: nothing starts, nothing is cleared, Solve is free again; the late result changes nothing", async (interruption) => {
  const { result, start, material } = await session();
  const cells = solveCells(result.current);
  const release = holdNextCheck();
  let solving: Promise<void> = Promise.resolve();
  act(() => { solving = result.current.results.handleRun(); });
  expect(result.current.results.running).toBe(true);
  expect(result.current.chrome.modelCheckBusy).toBe(true);
  if (interruption === "cancel") act(() => { result.current.results.handleCancelRun(); });
  else await applyMaterial(result, material);
  // The check is still pending here.
  expect(start).not.toHaveBeenCalled();
  expect(result.current.results.running).toBe(false);
  expect(result.current.chrome.modelCheckBusy).toBe(false);
  expect(result.current.chrome.issuesDrawerOpen).toBe(false);
  // An applied operation records itself as the solve job; a cancel leaves the job as it was.
  const shownJob = interruption === "cancel" ? cells.solveJob : result.current.results.solveJob;
  expect(result.current.results.solveJob).toBe(shownJob);
  if (interruption === "cancel") expect(solveCells(result.current)).toEqual(cells);
  const runBefore = result.current.chrome.modelCheckRun;
  // The late result of the ended check changes nothing.
  await act(async () => { release(); await solving; });
  expect(start).not.toHaveBeenCalled();
  expect(result.current.results.running).toBe(false);
  expect(result.current.results.solveJob).toBe(shownJob);
  expect(result.current.chrome.modelCheckRun).toBe(runBefore);
  // Solve is free again: the next request runs to the solve's start (the value is supplied after the edit).
  if (interruption === "model change") {
    await act(async () => { await result.current.results.handleRun(); });
    expect(start).toHaveBeenCalledOnce();
  } else {
    await act(async () => { await result.current.results.handleRun(); });
    expect(result.current.results.solveGate.kind).toBe("withheld");
    expect(result.current.chrome.issuesDrawerOpen).toBe(true);
  }
});

it("an older Solve request's end never retires the re-check that follows an edit, and a newer check supersedes an older one", async () => {
  const { result, start, material } = await session();
  // Solve is withheld for the model as loaded, so a later edit is re-checked by itself.
  await act(async () => { await result.current.results.handleRun(); });
  expect(result.current.results.solveGate.kind).toBe("withheld");
  const [recheckAfterSupply, solveCheck] = holdChecks(2);
  await applyMaterial(result, material);
  expect(result.current.chrome.modelCheckBusy).toBe(true); // The re-check of the supplied value is pending.
  // A Solve request on the supplied value supersedes that re-check with its own.
  let solving: Promise<void> = Promise.resolve();
  act(() => { solving = result.current.results.handleRun(); });
  expect(result.current.results.running).toBe(true);
  // Undo ends the Solve request (its check is the newest, so it is retired) and re-checks the restored value.
  act(() => { result.current.operations.handleUndoSessionModelEdit(); });
  await waitFor(() => expect(result.current.results.solveGate.kind).toBe("withheld"));
  expect(result.current.results.solveGate.kind === "withheld" && result.current.results.solveGate.model).toBe(result.current.model.model);
  // The superseded re-check and the ended Solve request's check resolve late and change nothing.
  const run = result.current.chrome.modelCheckRun;
  await act(async () => { recheckAfterSupply(); solveCheck(); await solving; });
  expect(result.current.chrome.modelCheckRun).toBe(run);
  expect(start).not.toHaveBeenCalled();
  expect(result.current.results.running).toBe(false);
});

it("a check that throws is read as the checks unavailable: the solve proceeds and nothing stays held, so Solve can run again", async () => {
  const { result, start } = await session();
  vi.spyOn(modelChecks, "runModelChecks").mockRejectedValueOnce(new Error("invented library-reference failure"));
  await act(async () => { await result.current.results.handleRun(); });
  expect(start).toHaveBeenCalledTimes(1);
  expect(result.current.results.running).toBe(false);
  expect(result.current.chrome.modelCheckBusy).toBe(false);
  expect(result.current.results.solveGate).toEqual({ kind: "unknown", reason: "checks_unavailable" });
  expect(result.current.chrome.modelCheckRun?.solve).toEqual({ kind: "unavailable", reason: "The model checks did not complete: invented library-reference failure" });
  // The token was released: a second request runs.
  await act(async () => { await result.current.results.handleRun(); });
  expect(start).toHaveBeenCalledTimes(2);
});

it("an unmount during the Solve request's check starts nothing when the check later resolves", async () => {
  const { result, start, unmount } = await session();
  // An open reading for the check to resolve to: the model as loaded is checked with its value supplied.
  const model = result.current.model.model!;
  const supplied = structuredClone(model);
  supplied.pipe_segments.find((pipe) => pipe.id === PIPE)!.material = supplied.materials![0].id;
  const real = modelChecks.runModelChecks;
  let release: () => void = () => {};
  const held = new Promise<void>((resolve) => { release = resolve; });
  vi.spyOn(modelChecks, "runModelChecks").mockImplementationOnce(async () => { await held; return { ...(await real(supplied)), model }; });
  let solving: Promise<void> = Promise.resolve();
  act(() => { solving = result.current.results.handleRun(); });
  unmount();
  release();
  await solving;
  expect(start).not.toHaveBeenCalled();
});

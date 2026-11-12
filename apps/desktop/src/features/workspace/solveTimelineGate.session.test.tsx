// The solve timeline from the run request, through the Solve command's gate (DEL-07-07 REQ-013,
// AC-009; DEL-04-05, "The clock"): `solve_requested` is marked at the request, before the gate,
// and `checks_done` when the gate's model check ends, whether the check ran on this request or a
// current run was reused. A request that does not proceed ends its timeline: withheld as refused
// with its code, a cancel as cancelled, a model change as superseded. The session runs the actual
// wasm model checks on the bundled invented model, with one solve-required value emptied where the
// solve is to be withheld (as in solveWithheld.session.test.tsx).
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import * as hashService from "../../services/hashService";
import { readSolveTimeline, resetSolveTimelineForTests } from "../../services/solveTimeline";
import type { PreviewModel } from "../../types";
import { buildBulkChangeBatch } from "../bulk-change/bulkChange";
import * as modelChecks from "../diagnostics/modelChecks";
import * as solveJobAudit from "./solveJobAudit";
import { useWorkspaceSession } from "./workspaceSession";

afterEach(() => {
  resetSolveTimelineForTests();
  vi.restoreAllMocks();
});

const PIPE = "pipe:P-100";

/** A session on the invented model; `withheld` empties pipe P-100's material reference. The
 * native start is observed and answered by a backend receipt whose job then fails to be awaited,
 * so a solve that passes the gate marks `job_started` and ends there. */
async function session(withheld: boolean) {
  const loaded = JSON.parse(JSON.stringify(await previewService.loadPreviewModel())) as PreviewModel;
  const material = loaded.pipe_segments.find((pipe) => pipe.id === PIPE)!.material;
  if (withheld) loaded.pipe_segments.find((pipe) => pipe.id === PIPE)!.material = "";
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(loaded);
  const start = vi.spyOn(previewService, "startPreviewMechanicsJob").mockResolvedValue({
    mode: "backend_job", job_id: "invented-job", backend_cancellation_token: "invented-token"
  } as Awaited<ReturnType<typeof previewService.startPreviewMechanicsJob>>);
  vi.spyOn(solveJobAudit, "awaitBackendSolveJob").mockRejectedValue(new Error("INVENTED-AWAIT-OBSERVED"));
  vi.spyOn(previewService, "abandonPreviewMechanicsJob").mockResolvedValue(undefined as never);
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return { result: hook.result, start, material };
}
type Result = Awaited<ReturnType<typeof session>>["result"];

const stages = () => readSolveTimeline()?.marks.map((mark) => mark.stage) ?? [];
const checksDone = () => readSolveTimeline()?.marks.find((mark) => mark.stage === "checks_done")?.detail ?? null;

async function applyMaterial(result: Result, materialId: string) {
  const batch = await buildBulkChangeBatch(result.current.model.model!, [{ type: "pipe", id: PIPE }], { kind: "material", materialId }, hashService.canonicalJsonString, "solve-timeline-gate-supply");
  await act(async () => { await result.current.operations.handleQueueOperationBatch(batch); });
  await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], false); });
  await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
}

it("marks checks_done once, between solve_requested and job_started, for a check run on the request and for a reused one", async () => {
  const { result, start } = await session(false);
  const checks = vi.spyOn(modelChecks, "runModelChecks");

  // No run is current: the gate runs the checks on the request.
  await act(async () => { await result.current.results.handleRun(); });
  expect(checks).toHaveBeenCalledOnce();
  expect(start).toHaveBeenCalledOnce();
  expect(stages()).toEqual(["solve_requested", "checks_done", "job_started"]);
  expect(checksDone()).toEqual({ ran: true, blocking_findings: 0 });
  const first = readSolveTimeline()!;
  expect(first.marks.map((mark) => mark.since_start_ms)).toEqual([...first.marks.map((mark) => mark.since_start_ms)].sort((a, b) => a - b));

  // The run the request made is current for the model value: the gate reuses it.
  await act(async () => { await result.current.results.handleRun(); });
  expect(checks).toHaveBeenCalledOnce();
  expect(start).toHaveBeenCalledTimes(2);
  expect(readSolveTimeline()!.sequence).toBe(first.sequence + 1);
  expect(stages()).toEqual(["solve_requested", "checks_done", "job_started"]);
  expect(checksDone()).toEqual({ ran: false, blocking_findings: 0 });
});

it("ends a withheld request's timeline as refused with its code, after checks_done, and starts no job", async () => {
  const { result, start } = await session(true);
  expect(readSolveTimeline()).toBeNull();

  // Checked on the request.
  await act(async () => { await result.current.results.handleRun(); });
  expect(start).not.toHaveBeenCalled();
  expect(result.current.results.solveGate.kind).toBe("withheld");
  // The request began a timeline (the driver detects a begun solve by its sequence).
  const ran = readSolveTimeline()!;
  expect(ran.outcome).toBe("refused");
  expect(ran.marks.map((mark) => mark.stage)).toEqual(["solve_requested", "checks_done"]);
  expect(checksDone()).toEqual({ ran: true, blocking_findings: 1 });
  expect(ran.failure).toMatchObject({ stage: "checks_done", code: "SOLVE-WITHHELD-MODEL-CHECKS", error_name: null });

  // The standing run reused.
  await act(async () => { await result.current.results.handleRun(); });
  expect(start).not.toHaveBeenCalled();
  const reused = readSolveTimeline()!;
  expect(reused.sequence).toBe(ran.sequence + 1);
  expect(reused.outcome).toBe("refused");
  expect(reused.marks.map((mark) => mark.stage)).toEqual(["solve_requested", "checks_done"]);
  expect(checksDone()).toEqual({ ran: false, blocking_findings: 1 });
  expect(reused.failure).toMatchObject({ stage: "checks_done", code: "SOLVE-WITHHELD-MODEL-CHECKS" });
});

/** Holds the next model-check run until released, then runs the real checks. */
function holdNextCheck() {
  const real = modelChecks.runModelChecks;
  let release: () => void = () => {};
  const gate = new Promise<void>((resolve) => { release = resolve; });
  vi.spyOn(modelChecks, "runModelChecks").mockImplementationOnce(async (...args) => { await gate; return real(...args); });
  return () => release();
}

it.each([["cancel", "cancelled"], ["model change", "superseded"]] as const)("a %s during the request's check ends its timeline as %s, with no checks_done", async (interruption, outcome) => {
  const { result, start, material } = await session(true);
  const release = holdNextCheck();
  let solving: Promise<unknown> = Promise.resolve();
  act(() => { solving = result.current.results.handleRun(); });
  // The timeline has begun at the request, and is running while the check runs.
  expect(stages()).toEqual(["solve_requested"]);
  expect(readSolveTimeline()!.outcome).toBe("running");
  if (interruption === "cancel") act(() => { result.current.results.handleCancelRun(); });
  else await applyMaterial(result, material);
  await act(async () => { release(); await solving; });
  expect(start).not.toHaveBeenCalled();
  const timeline = readSolveTimeline()!;
  expect(timeline.outcome).toBe(outcome);
  expect(timeline.marks.map((mark) => mark.stage)).toEqual(["solve_requested"]);
  expect(timeline.failure).toBeNull();
});

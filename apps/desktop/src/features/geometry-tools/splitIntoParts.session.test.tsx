/** B5 (DEL-07-01 Design `split-and-slope-entry.md` §2): the equal-part split through the session
 * and the actual wasm engine. The batch validates, applies once, stores every intermediate node
 * bit-identical to A + (B − A) · (k / N) (Object.is per component), and one Undo restores the
 * model. Refusals match the engine's own. Invented inline model; project unit ft. */
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { applyOperationBatch } from "../../services/operationBatchService";
import { splitPartsModel } from "../../test-support/splitPartsModel";
import type { PreviewModel } from "../../types";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { buildGeometryBatch, emptyGeometryDraft } from "./geometryDraft";
import { buildSplitIntoPartsBatch, emptySplitPartsRequest, planSplitIntoParts, type SplitPartsRequest } from "./splitIntoParts";

async function session(model: PreviewModel) {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
afterEach(() => { vi.restoreAllMocks(); });

const request = (pipeId: string, parts: number): SplitPartsRequest => ({
  ...emptySplitPartsRequest(pipeId), parts: String(parts), nodeIdPattern: "node:S{k}", nodeLabelPattern: "S{k}",
  pipeIdPattern: "pipe:S{k}", pipeLabelPattern: "S-{k}", provenance: "invented split", firstNumber: "10", step: "10"
});

/** The stated formula, written out independently of the planner. */
function stated(a: { x: number; y: number; z: number }, b: { x: number; y: number; z: number }, k: number, n: number) {
  const t = k / n;
  const c = (p: number, q: number) => { const v = p + (q - p) * t; return v === 0 ? 0 : v; };
  return { x: c(a.x, b.x), y: c(a.y, b.y), z: c(a.z, b.z) };
}

async function applyThroughSession(model: PreviewModel, pipeId: string, parts: number) {
  const result = await session(model);
  const original = structuredClone(result.current.model.model!);
  const originalHash = (await computeModelHash(original))!.value;
  const batch = await buildSplitIntoPartsBatch(original, request(pipeId, parts), canonicalJsonString, `split-${pipeId}`);
  await act(async () => { await result.current.operations.handleQueueOperationBatch(batch); });
  const entry = result.current.operations.queuedBatches[0];
  await act(async () => { await result.current.operations.handleRunOperationBatch(entry, false); });
  const preview = result.current.operations.batchOutcomes[entry.key];
  expect(preview.operation_outcomes.flatMap((step) => step.diagnostics.filter((item) => item.severity === "blocking"))).toEqual([]);
  expect(preview.validation.batch_validation_status).toBe("passed");
  await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
  expect(result.current.operations.batchMessage).toBe("Batch applied once. Previous solve results were cleared; one undo checkpoint is available.");
  return { result, original, originalHash, batch };
}

describe("equal-part split through the session and the wasm engine", () => {
  it("stores every node bit-identical to the stated formula, in a non-identity unit, and one Undo restores the model", async () => {
    const model = splitPartsModel();
    const { result, originalHash, batch } = await applyThroughSession(model, "pipe:S", 7);
    // Corrections were needed in this case: the engine's own split rounds differently for some k.
    expect(batch.operations.some((op) => op.change.change_kind === "set_field")).toBe(true);
    const applied = result.current.model.model!;
    const a = model.nodes.find((n) => n.id === "node:A")!.position;
    const b = model.nodes.find((n) => n.id === "node:B")!.position;
    for (let k = 1; k < 7; k++) {
      const node = applied.nodes.find((n) => n.id === `node:S${k * 10}`)!;
      expect(node.label).toBe(`S${k * 10}`);
      const expected = stated(a, b, k, 7);
      for (const axis of ["x", "y", "z"] as const) expect(Object.is(node.position[axis], expected[axis]), `k=${k} ${axis}: ${node.position[axis]} vs ${expected[axis]}`).toBe(true);
    }
    // The chain A -S- S10 -S10- S20 … S60 -S60- B, tags and y_reference carried, line extended.
    const chain = ["pipe:S", "pipe:S10", "pipe:S20", "pipe:S30", "pipe:S40", "pipe:S50", "pipe:S60"].map((id) => applied.pipe_segments.find((p) => p.id === id)!);
    expect(chain.map((p) => [p.from, p.to])).toEqual([
      ["node:A", "node:S10"], ["node:S10", "node:S20"], ["node:S20", "node:S30"], ["node:S30", "node:S40"],
      ["node:S40", "node:S50"], ["node:S50", "node:S60"], ["node:S60", "node:B"]]);
    expect(chain.map((p) => p.label)).toEqual(["S", "S-10", "S-20", "S-30", "S-40", "S-50", "S-60"]);
    expect(chain.every((p) => JSON.stringify(p.tags) === JSON.stringify(["drain"]) && p.y_reference?.y === 1)).toBe(true);
    expect(applied.lines?.[0].pipe_refs).toEqual(["pipe:S", "pipe:S10", "pipe:S20", "pipe:S30", "pipe:S40", "pipe:S50", "pipe:S60"]);
    expect(result.current.operations.undoStack).toHaveLength(1);
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect((await computeModelHash(result.current.model.model!))!.value).toBe(originalHash);
  });

  it("a −0 endpoint coordinate stores +0 and stays bit-identical to the normalized formula", async () => {
    const model = splitPartsModel();
    const d = model.nodes.find((n) => n.id === "node:D")!;
    const e = model.nodes.find((n) => n.id === "node:E")!;
    d.position = { x: 20, y: -0, z: -0 };
    e.position = { x: 32.5, y: -0, z: -0 };
    const planned = await planSplitIntoParts(model, request("pipe:H", 4));
    expect(planned.ok).toBe(true);
    if (!planned.ok) return;
    for (const node of planned.nodes) { expect(Object.is(node.position.y, 0)).toBe(true); expect(Object.is(node.position.z, 0)).toBe(true); }
    const { result } = await applyThroughSession(model, "pipe:H", 4);
    for (const [k, x] of [[1, 23.125], [2, 26.25], [3, 29.375]] as const) {
      const node = result.current.model.model!.nodes.find((n) => n.id === `node:S${k * 10}`)!;
      expect(Object.is(node.position.x, x)).toBe(true);
      expect(Object.is(node.position.y, 0) && Object.is(node.position.z, 0)).toBe(true);
    }
  });

  it.each([
    ["pipe:V", "component:VALVE-C names the end node node:C", "components"],
    ["pipe:W", "wind exposure names pipe:W", "load_cases"]
  ])("%s: the planner refuses as the engine refuses the same single split (%s)", async (pipeId, _why, path) => {
    const model = splitPartsModel();
    const planned = await planSplitIntoParts(model, request(pipeId, 3));
    expect(planned.ok).toBe(false);
    if (planned.ok) return;
    expect(planned.reason).toContain(path);
    // The engine's own refusal of the existing single split at 1/3, through the same session.
    const result = await session(model);
    const draft = { ...emptyGeometryDraft(), action: "split" as const, pipeRefs: [pipeId], fraction: String(1 / 3),
      newNode: { id: "node:X", label: "X", provenance: "p" }, newPipe: { id: "pipe:X", label: "X", provenance: "p" } };
    const batch = await buildGeometryBatch(model, draft, "single-split");
    await act(async () => { await result.current.operations.handleQueueOperationBatch(batch); });
    const entry = result.current.operations.queuedBatches[0];
    await act(async () => { await result.current.operations.handleRunOperationBatch(entry, false); });
    const blocking = result.current.operations.batchOutcomes[entry.key].operation_outcomes.flatMap((step) => step.diagnostics.filter((item) => item.severity === "blocking"));
    const engine = blocking.find((item) => item.code === "OP-GEOMETRY-ATTACHMENT-UNSUPPORTED");
    expect(engine, JSON.stringify(blocking)).toBeTruthy();
    // The planner names the same references the engine names.
    const engineRefs = engine!.message.replace(/^.*attachments: /, "");
    expect(planned.reason).toContain(engineRefs);
  });

  it("a support at an end node does not block, as in the engine", async () => {
    const planned = await planSplitIntoParts(splitPartsModel(), request("pipe:H", 2));
    expect(planned.ok).toBe(true);
  });

  // r1: one separate existing split of the original pipe at k / N, through the engine.
  async function singleSplit(model: PreviewModel, pipeId: string, fraction: number) {
    const draft = { ...emptyGeometryDraft(), action: "split" as const, pipeRefs: [pipeId], fraction: String(fraction),
      newNode: { id: "node:ONE", label: "ONE", provenance: "p" }, newPipe: { id: "pipe:ONE", label: "ONE", provenance: "p" } };
    const batch = await buildGeometryBatch(model, draft, "one-split");
    return applyOperationBatch(model, batch, await computeModelHash(model));
  }

  it("node k equals what one existing split of the original pipe at k / N stores (engine, k = 1 … N−1)", async () => {
    const model = splitPartsModel();
    const { result } = await applyThroughSession(model, "pipe:S", 7);
    const applied = result.current.model.model!;
    for (let k = 1; k < 7; k++) {
      const single = await singleSplit(splitPartsModel(), "pipe:S", k / 7);
      expect(single.validation.application_status).toBe("applied_to_session_model");
      const one = single.applied_model!.nodes.find((n) => n.id === "node:ONE")!.position;
      const node = applied.nodes.find((n) => n.id === `node:S${k * 10}`)!.position;
      for (const axis of ["x", "y", "z"] as const) expect(Object.is(node[axis], one[axis]), `k=${k} ${axis}`).toBe(true);
    }
  });

  it("stores the formula's binary64 value, not the correctly rounded interpolation (A=6.97…, B=−2.55…, N=8, k=7)", async () => {
    const model = splitPartsModel();
    model.nodes.find((n) => n.id === "node:D")!.position = { x: 6.9705855667666725, y: -3.7, z: 9 };
    model.nodes.find((n) => n.id === "node:E")!.position = { x: -2.5521381070139633, y: -3.7, z: 9 };
    const { result } = await applyThroughSession(model, "pipe:H", 8);
    const x = result.current.model.model!.nodes.find((n) => n.id === "node:S70")!.position.x;
    expect(Object.is(x, -1.3617976477913842)).toBe(true);
    expect(x).not.toBe(-1.3617976477913838);
  });

  it("refuses a piece that rounds to zero length, as the engine refuses the same single split", async () => {
    const model = splitPartsModel();
    const ulp = 2 ** -36; // the spacing of binary64 near 1e5
    model.nodes.find((n) => n.id === "node:D")!.position = { x: 1e5, y: -3.7, z: 9 };
    model.nodes.find((n) => n.id === "node:E")!.position = { x: 1e5 + 2 * ulp, y: -3.7, z: 9 };
    // The source spans 2 ulp ≈ 8.9e-12 m, longer than the 1e-12 m tolerance; at 1/8 the node rounds onto D.
    expect(1e5 + 2 * ulp * 0.125).toBe(1e5);
    const planned = await planSplitIntoParts(model, request("pipe:H", 8));
    expect(planned.ok).toBe(false);
    if (!planned.ok) expect(planned.reason).toContain("axis tolerance");
    const single = await singleSplit(model, "pipe:H", 1 / 8);
    expect(single.applied_model).toBeNull();
    expect(JSON.stringify(single.operation_outcomes.flatMap((step) => step.diagnostics))).toContain("Degenerate or nonfinite axis/length");
  });

  // r1: the applier's record_shape includes the unit-aware corrosion and effective-wall rule.
  it.each([
    ["allowance equal to the wall", { corrosion_allowance: { value: 0.237, unit: "in" } }, false],
    ["mill tolerance in mm plus allowance in in exceed the wall", { mill_tolerance: { value: 3, unit: "mm" }, corrosion_allowance: { value: 0.12, unit: "in" } }, false],
    ["mill tolerance equal to the wall", { mill_tolerance: { value: 6.0198, unit: "mm" }, corrosion_allowance: { value: 0, unit: "in" } }, false],
    ["a negative allowance", { corrosion_allowance: { value: -0.01, unit: "in" } }, false],
    ["mill tolerance in mm plus allowance in in leave a wall", { mill_tolerance: { value: 3, unit: "mm" }, corrosion_allowance: { value: 0.1, unit: "in" } }, true]
  ])("corrosion rule, %s: planner and engine agree", async (_name, extra, accepted) => {
    const model = splitPartsModel();
    Object.assign(model.pipe_segments.find((p) => p.id === "pipe:H")!.section, extra);
    const planned = await planSplitIntoParts(model, request("pipe:H", 3));
    const single = await singleSplit(model, "pipe:H", 1 / 3);
    const engineMessages = single.operation_outcomes.flatMap((step) => step.diagnostics.map((item) => item.message));
    expect(planned.ok).toBe(accepted);
    expect(single.validation.application_status === "applied_to_session_model").toBe(accepted);
    if (!planned.ok) {
      expect(engineMessages.some((message) => message.includes(planned.reason.replace(/^Pipe pipe:H: /, "")))).toBe(true);
      await expect(buildSplitIntoPartsBatch(model, request("pipe:H", 3), canonicalJsonString, "b")).rejects.toThrow();
    }
  });
});

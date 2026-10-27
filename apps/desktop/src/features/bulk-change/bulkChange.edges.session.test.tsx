/** B2 (DEL-07-02 Design `bulk-change.md`, ScopeOfWork DEL-07-02:14): the edges the wording
 * promises, through the session and the actual wasm engine on invented inline models:
 * - a bend radius entered in a non-metre unit is written and stored exactly as entered, in the
 *   shape the inspector's Bend radius field writes;
 * - an absent radius has the `TBD` before the applier expects;
 * - a support that a load or another support names is not removed: the planner refuses a
 *   normal-reaction source or a primitive load target before queuing, mirroring the applier's
 *   OP-SUPPORT-DELETE-REFERENCED, which still blocks such a removal queued by another route;
 * - "Keep each support's restraints" writes the support's own restraints exactly. */
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { bulkChangeModel } from "../../test-support/bulkChangeModel";
import type { EntityRef, PreviewModel } from "../../types";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { buildDeleteSupportIntent } from "../model-tree/removalIntents";
import { buildBulkChangeBatch, planBulkChange, type BulkChangeRequest } from "./bulkChange";

async function session(model: PreviewModel) {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
afterEach(() => { vi.restoreAllMocks(); });

type Session = Awaited<ReturnType<typeof session>>;
async function queueValidate(result: Session, scope: EntityRef[], request: BulkChangeRequest) {
  const batch = await buildBulkChangeBatch(result.current.model.model!, scope, request, canonicalJsonString, `bulk-${request.kind}-edge`);
  await act(async () => { await result.current.operations.handleQueueOperationBatch(batch); });
  const entry = result.current.operations.queuedBatches[0];
  await act(async () => { await result.current.operations.handleRunOperationBatch(entry, false); });
  return { batch, outcome: result.current.operations.batchOutcomes[entry.key] };
}
async function apply(result: Session) {
  await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
}
const blockingCodes = (outcome: { diagnostics: Array<{ severity: string; code: string }>; operation_outcomes: Array<{ diagnostics: Array<{ severity: string; code: string }> }> }) =>
  [...outcome.diagnostics, ...outcome.operation_outcomes.flatMap((step) => step.diagnostics)].filter((item) => item.severity === "blocking").map((item) => item.code);
const bendB: EntityRef = { type: "component", id: "component:BEND-B" };

describe("bend radius in a non-metre unit is written and stored exactly as entered", () => {
  it.each([
    ["a millimetre project", "mm", "228.6", "mm", 228.6],
    ["a metre project, radius entered in inches", "m", "9", "in", 9]
  ])("%s", async (_name, projectLength, value, unit, stored) => {
    const model = bulkChangeModel();
    model.project.units.length = projectLength;
    if (projectLength === "mm") model.components[0].geometry = { bend_radius: { value: 152.4, unit: "mm" } };
    const result = await session(model);
    const { batch, outcome } = await queueValidate(result, [bendB], { kind: "bend-radius", value, unit });
    // The inspector's Bend radius field: after `{value, unit}` with the entered unit, dimension length.
    expect(batch.operations).toHaveLength(1);
    expect(batch.operations[0].change).toMatchObject({
      change_kind: "set_field", field_path: "geometry.bend_radius.value", before: projectLength === "mm" ? "152.4" : "0.15",
      after: JSON.stringify({ value: stored, unit }), unit, dimension: "length"
    });
    expect(blockingCodes(outcome)).toEqual([]);
    await apply(result);
    expect(result.current.model.model!.components[0].geometry!.bend_radius).toEqual({ value: stored, unit });
  });
});

describe("an absent bend radius", () => {
  it.each([
    ["no geometry", undefined],
    ["geometry without a radius", { bend_angle: { value: 90, unit: "deg" } }]
  ])("%s: before is TBD and the radius applies through the engine", async (_name, geometry) => {
    const model = bulkChangeModel();
    if (geometry === undefined) delete model.components[0].geometry;
    else model.components[0].geometry = geometry;
    const result = await session(model);
    const { batch, outcome } = await queueValidate(result, [bendB], { kind: "bend-radius", value: "0.229", unit: "m" });
    expect(batch.operations[0].change.before).toBe("TBD");
    expect(blockingCodes(outcome)).toEqual([]);
    await apply(result);
    expect(result.current.model.model!.components[0].geometry).toEqual({ ...(geometry ?? {}), bend_radius: { value: 0.229, unit: "m" } });
  });
});

describe("a support another entity names is not removed", () => {
  it("the planner refuses a normal-reaction source, so no batch is built or queued", async () => {
    const model = bulkChangeModel();
    model.supports.push({ id: "support:F", label: "Friction", node: "node:B", restraints: [], nonlinear: { behavior: "friction", dof: "UX", initial_state: "sticking",
      friction_coefficient: { value: 0.3, unit: "1" }, normal_reaction_source: { support_ref: "support:B", dof: "UY" } }, provenance: "invented" } as never);
    const result = await session(model);
    const scope: EntityRef[] = [{ type: "node", id: "node:B" }];
    const request: BulkChangeRequest = { kind: "support-remove", family: "guide" };
    expect(planBulkChange(result.current.model.model!, scope, request).refused).toEqual([
      { ref: { type: "support", id: "support:B" }, reason: "Support support:B is still referenced by support:F; delete or retarget them first." }]);
    await expect(buildBulkChangeBatch(result.current.model.model!, scope, request, canonicalJsonString, "bulk-refused")).rejects.toThrow("Nothing is queued while any entity is refused.");
    expect(result.current.operations.queuedBatches).toHaveLength(0);
  });

  it("the planner refuses a support a primitive load targets, naming the load, so nothing is queued", async () => {
    const model = bulkChangeModel();
    model.load_cases[0].primitive_loads = [{ id: "load:settle-C", category: "imposed_displacement", target: { type: "support", support: "support:C", dof: "UZ" },
      direction: "UZ", magnitude: { value: -0.006, unit: "m" }, dimension: "displacement", provenance: "invented" }] as never;
    const result = await session(model);
    const scope: EntityRef[] = [{ type: "node", id: "node:B" }, { type: "node", id: "node:C" }];
    const request: BulkChangeRequest = { kind: "support-remove", family: "guide" };
    const plan = planBulkChange(result.current.model.model!, scope, request);
    expect(plan.affected.map((entry) => entry.ref.id)).toEqual(["support:B"]);
    expect(plan.refused).toEqual([
      { ref: { type: "support", id: "support:C" }, reason: "Support support:C is still referenced by load:settle-C; delete or retarget them first." }]);
    await expect(buildBulkChangeBatch(result.current.model.model!, scope, request, canonicalJsonString, "bulk-refused")).rejects.toThrow("Nothing is queued while any entity is refused.");
    expect(result.current.operations.queuedBatches).toHaveLength(0);

    // Second guard: the same removal queued by another route is blocked by the applier with
    // the reference the planner names, and the whole batch leaves the model unchanged.
    const original = structuredClone(result.current.model.model!);
    const originalHash = (await computeModelHash(original))!.value;
    const supports = original.supports.filter((support) => support.id === "support:B" || support.id === "support:C");
    await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: "other-route", operations: supports.map((support) => buildDeleteSupportIntent(support, original)) }); });
    const entry = result.current.operations.queuedBatches[0];
    await act(async () => { await result.current.operations.handleRunOperationBatch(entry, false); });
    const outcome = result.current.operations.batchOutcomes[entry.key];
    expect(outcome.validation.batch_validation_status).toBe("blocked");
    expect(blockingCodes(outcome)).toContain("OP-SUPPORT-DELETE-REFERENCED");
    expect(JSON.stringify(outcome)).toContain("Support `support:C` is still referenced by primitive loads: load:settle-C.");
    await apply(result);
    expect(result.current.model.model).toEqual(original);
    expect((await computeModelHash(result.current.model.model!))!.value).toBe(originalHash);
    expect(result.current.operations.undoStack).toHaveLength(0);
  });
});

describe("keep restraints through the engine", () => {
  it("a guide replaced by an anchor keeps exactly its own restraints", async () => {
    const model = bulkChangeModel();
    model.supports[2].restraints = ["Uz", "UY"];
    const result = await session(model);
    const originals = model.supports.filter((support) => support.family === "guide").map((support) => structuredClone(support.restraints));
    const { batch, outcome } = await queueValidate(result, [{ type: "support", id: "support:B" }, { type: "support", id: "support:C" }],
      { kind: "support-kind", fromFamily: "guide", toFamily: "anchor", restraints: "keep" });
    expect(batch.operations.map((operation) => JSON.parse(operation.change.after).restraints)).toEqual(originals);
    expect(blockingCodes(outcome)).toEqual([]);
    await apply(result);
    const applied = result.current.model.model!.supports;
    expect(applied.filter((support) => support.id === "support:B" || support.id === "support:C").map((support) => [support.family, support.restraints]))
      .toEqual([["anchor", originals[0]], ["anchor", originals[1]]]);
  });
});

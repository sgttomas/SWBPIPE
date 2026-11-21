/** DEL-16-03, Commit: a person's own edit in a live desktop editor, applied where it is made. The session returns
 * one binder (`editorBinding`), outside `operations` and the context; the application root binds each editor's
 * own builder, with a declared input guard and a role, and gets back that editor's callback: form 1 `commit(input)`
 * for an immediate edit, form 2 `preview(input)` then `commit(handle)` for an action whose dialog shows the
 * engine's preview first. No entry takes an intent or a batch. No editor is bound in this slice, so these tests
 * bind real builders (the grid cell's, the bulk change's, the reconnect's) as the root will, and a pass-through
 * builder where the stamp's allowlist is under test. Every case runs through the session and the actual wasm
 * engine on an invented inline model. DEL-16-06: the binder is out of reach of the agent's host and offline intake. */
import { act, renderHook, waitFor } from "@testing-library/react";
import { useLayoutEffect } from "react";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import * as operationBatchService from "../../services/operationBatchService";
import * as hashService from "../../services/hashService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { requeueRetainedRecord, stampImportedBatch, type EditorBuilder, type EditorCommitResult, type EditorEdit, type EditorInputGuard } from "../../services/operationAuthorship";
import { stampAgentOperations } from "../../services/embeddedAgentSession";
import { parseOfflineBatch } from "../offline-proposal-intake/OfflineProposalIntakePanel";
import { buildReconnectBatch, emptyGeometryDraft } from "../geometry-tools/geometryDraft";
import { buildBulkChangeBatch, type BulkChangeRequest } from "../bulk-change/bulkChange";
import { makeRichIntent } from "../rich-authoring/formSupport";
import { buildGridOperationIntent, type GridColumn } from "./table/modelTableAdapter";
import { useWorkspaceSession } from "./workspaceSession";
import type { SessionEvent, SessionEventListener } from "./sessionEvents";
import { useRunSheetStore } from "../model-tree/runSheet/runSheetStore";
import { reconnectModel } from "../../test-support/reconnectModel";
import { bulkChangeModel } from "../../test-support/bulkChangeModel";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";

afterEach(() => { vi.restoreAllMocks(); });

async function session(load: () => PreviewModel = reconnectModel) {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(load());
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
type Session = Awaited<ReturnType<typeof session>>;
const hashOf = async (model: PreviewModel) => (await computeModelHash(model))!.value;
const plain = (value: unknown): value is Record<string, unknown> => !!value && typeof value === "object" && !Array.isArray(value);
const strings = (value: unknown, ...keys: string[]) => plain(value) && keys.every((key) => typeof value[key] === "string");

// ---- The builders, bound as the application root will bind them --------------------------------------------------

type GridInput = { nodeId: string; axis: "x" | "y" | "z"; value: string };
/** The grid cell's own builder (buildGridOperationIntent), on typed values: a node, an axis and the entered text. */
const gridCell: EditorBuilder<GridInput> = {
  name: "grid-cell",
  role: "gui_editor",
  input: { members: ["nodeId", "axis", "value"], check: (input) => strings(input, "nodeId", "axis", "value") && ["x", "y", "z"].includes(input.axis as string) ? null : "The grid cell takes a node, an axis and a value." },
  build: (model, input, id) => {
    const target = model.nodes.find((node) => node.id === input.nodeId);
    if (!target) throw new Error(`No node ${input.nodeId}.`);
    const column: GridColumn = { key: input.axis, label: input.axis.toUpperCase(), fieldPath: `position.${input.axis}`, objectType: "Node", changeKind: "set_field", dimension: "length",
      sourceNote: "entered coordinate", unit: () => model.project.units.length, value: () => String(target.position[input.axis]) };
    return { intent: buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence: id.sequence, value: input.value, interaction: "cell" }) };
  }
};
type BulkInput = { scope: EntityRef[]; request: BulkChangeRequest };
/** The bulk change's own builder (buildBulkChangeBatch), on typed values: the scope and the request. */
const bulkChange: EditorBuilder<BulkInput> = {
  name: "bulk-change",
  role: "gui_editor",
  input: { members: ["scope", "request"], check: (input) => Array.isArray(input.scope) && input.scope.every((ref) => strings(ref, "type", "id")) && strings(input.request, "kind") ? null : "The bulk change takes a scope and a request." },
  build: async (model, input, id) => ({ batch: await buildBulkChangeBatch(model, input.scope, input.request, canonicalJsonString, id.batchId) })
};
type ReconnectInput = { pipeId: string; targetNode: string };
/** The reconnect's own builder (buildReconnectBatch), on typed values; its dialog previews before it commits. */
const reconnect: EditorBuilder<ReconnectInput> = {
  name: "reconnect",
  role: "viewport_editor",
  input: { members: ["pipeId", "targetNode"], check: (input) => strings(input, "pipeId", "targetNode") ? null : "The reconnect takes a pipe and a target node." },
  build: async (model, input, id) => ({ batch: await buildReconnectBatch(model, { ...emptyGeometryDraft(), action: "reconnect", pipeRefs: [input.pipeId], reconnectEnd: "to", targetNode: input.targetNode, yReference: { x: "0", y: "0", z: "1" } }, id.batchId) })
};
type MaterialInput = { materialId: string };
/** A form that creates a material (makeRichIntent), for the focus after a create. */
const newMaterial: EditorBuilder<MaterialInput> = {
  name: "new-material",
  role: "gui_editor",
  input: { members: ["materialId"], check: (input) => strings(input, "materialId") ? null : "The form takes a material id." },
  build: (model, input) => {
    const intent = makeRichIntent({ object_type: "Material", ref: input.materialId }, "create_material", "materials", "not_present",
      { ...structuredClone(model.materials![0]), id: input.materialId, label: "New material" }, `Material ${input.materialId}`);
    return { intent: { ...intent, operation_kind: "create", change: { ...intent.change, unit: model.project.units.stress, dimension: "stress" } } };
  }
};
/** A builder that returns whatever the test hands it: only for the stamp's allowlist, which reads what a builder
 * returned. A real builder takes typed values only; this one stands for a builder that passed content through. */
function passThrough(): { builder: EditorBuilder<{ key: string }>; next: (edit: unknown) => void; built: () => number } {
  let pending: unknown = null, count = 0;
  return {
    builder: { name: "pass-through", role: "gui_editor", input: { members: ["key"], check: () => null }, build: () => { count += 1; return pending as EditorEdit; } },
    next: (edit) => { pending = edit; },
    built: () => count
  };
}

function bindCommit<I>(result: Session, builder: EditorBuilder<I>) { return result.current.editorBinding.bindCommit(builder); }
function bindDialog<I>(result: Session, builder: EditorBuilder<I>) { return result.current.editorBinding.bindDialog(builder); }
async function run<T>(work: () => Promise<T>): Promise<T> {
  let value: T | undefined;
  await act(async () => { value = await work(); });
  return value!;
}
function committed<T extends { status: string }>(outcome: T) {
  if (outcome.status !== "committed") throw new Error(`expected a commit, got: ${JSON.stringify(outcome)}`);
  return outcome as Extract<T, { status: "committed" }>;
}
function refused<T extends { status: string }>(outcome: T) {
  if (outcome.status !== "refused") throw new Error(`expected a refusal, got: ${JSON.stringify(outcome)}`);
  return outcome as Extract<T, { status: "refused" }>;
}

/** Everything a commit may change: the model, the history, the receipts and the review queue. */
function snapshot(result: Session) {
  const operations = result.current.operations;
  return structuredClone({
    model: result.current.model.model, undoStack: operations.undoStack, redoStack: operations.redoStack, batchReceipts: operations.batchReceipts,
    retainedReviewContext: operations.retainedReviewContext, appliedOperations: operations.appliedOperations, queuedBatches: operations.queuedBatches,
    editorIntents: operations.editorIntents, batchOutcomes: operations.batchOutcomes, operationOutcomes: operations.operationOutcomes
  });
}
/** The request state a refused call must leave alone (A10). */
const requestState = (result: Session) => ({ sequence: result.current.operations.operationRequest.current.sequence, busy: result.current.operations.operationBusy });

function events(result: Session) {
  const seen: SessionEvent[] = [];
  const stop = result.current.operations.sessionEvents.listen((event) => { seen.push(event); });
  return { seen, stop };
}

/** Holds the next call of an engine entry until released, then runs the real one. */
function hold(name: "validateOperationBatch" | "applyOperationBatch") {
  const real = operationBatchService[name];
  let release!: () => void;
  const gate = new Promise<void>((resolve) => { release = resolve; });
  let reached!: () => void;
  const arrived = new Promise<void>((resolve) => { reached = resolve; });
  vi.spyOn(operationBatchService, name).mockImplementationOnce(async (...args) => { reached(); await gate; return real(...args); });
  return { release, arrived };
}
/** Alters the next reply of an engine entry. */
function alter(name: "validateOperationBatch" | "applyOperationBatch", change: (outcome: operationBatchService.OperationBatchOutcome) => void) {
  const real = operationBatchService[name];
  vi.spyOn(operationBatchService, name).mockImplementationOnce(async (...args) => { const outcome = await real(...args); change(outcome); return outcome; });
}
const deepFrozen = (value: unknown): boolean => value === null || typeof value !== "object" ? true : Object.isFrozen(value) && Object.values(value).every(deepFrozen);

describe("form 1: the one-step commit of an immediate edit", () => {
  it("a field commits as one checkpoint stamped by the binding; Undo restores the canonical hash, Redo reapplies, and no person reads the session's id", async () => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    const originalHash = await hashOf(original);
    const commit = bindCommit(result, gridCell);
    const { seen } = events(result);
    const outcome = committed(await run(() => commit({ nodeId: "node:C", axis: "y", value: "2.5" })));

    const operations = result.current.operations;
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    const committedHash = await hashOf(result.current.model.model!);
    expect(deepFrozen(outcome)).toBe(true);
    expect(Object.keys(outcome).sort()).toEqual(["checkpointId", "historyName", "notice", "resultsCleared", "status"]);
    const operationId = operations.batchReceipts[0].batch.operations[0].operation_id;
    expect(outcome).toEqual({ status: "committed", checkpointId: operations.undoStack[0].checkpoint_id, historyName: operationId, resultsCleared: false, notice: null });
    // The checkpoint and its event carry the session's id; the submitted id is the named member a person reads.
    expect(operations.undoStack).toHaveLength(1);
    expect(operations.undoStack[0]).toMatchObject({ operation_id: expect.stringMatching(/^editor-commit-\d+$/), submitted_id: operationId, model: original });
    const checkpoint = seen.find((event) => event.type === "checkpoint")!;
    expect(checkpoint).toMatchObject({ operationId: operations.undoStack[0].operation_id, submittedId: operationId });
    // One decision record (the batch receipt with the engine's acceptance) and one outcome record, as for an applied queued batch.
    expect(operations.batchReceipts).toHaveLength(1);
    expect(operations.batchReceipts[0].outcome.acceptance).not.toBeNull();
    expect(operations.batchReceipts[0].batch.operations[0]).toMatchObject({ author_type: "user",
      source: { source_ref: `desktop-editor:gui_editor:${operationId}:1`, source_channel: "local_desktop_preview", source_role: "gui_editor" } });
    expect(operations.appliedOperations).toEqual([expect.objectContaining({ sequence: 1, operation_id: operationId, before: originalHash, after: committedHash, field_path: "batch" })]);
    expect(operations.retainedReviewContext).toEqual(operations.batchReceipts[0].batch.operations);
    expect(operations.queuedBatches).toHaveLength(0);
    expect(operations.editorIntents).toHaveLength(0);

    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(await hashOf(result.current.model.model!)).toBe(originalHash);
    expect(result.current.operations.operationMessage).toContain(`Undid ${operationId} `);
    expect(result.current.operations.operationMessage).not.toContain("editor-commit-");
    expect(result.current.results.solveJob.events.map((event) => JSON.stringify(event)).join()).not.toContain("editor-commit-");
    act(() => result.current.operations.handleRedoSessionModelEdit());
    expect(await hashOf(result.current.model.model!)).toBe(committedHash);
    expect(result.current.operations.operationMessage).toContain(`Redid ${operationId} `);
    expect(result.current.operations.undoStack[0].submitted_id).toBe(operationId);
  });

  it("a batch built by the bulk change's builder commits whole as one checkpoint; one Undo reverts every change in it", async () => {
    const result = await session(bulkChangeModel);
    const original = structuredClone(result.current.model.model!);
    const outcome = committed(await run(() => bindCommit(result, bulkChange)({ scope: [{ type: "pipe", id: "pipe:P2" }, { type: "pipe", id: "pipe:P3" }], request: { kind: "material", materialId: "material:alloy" } })));
    expect(result.current.model.model!.pipe_segments.map((pipe) => pipe.material)).toEqual(["material:steel", "material:alloy", "material:alloy"]);
    expect(outcome.historyName).toMatch(/^bulk-change-\d+$/);
    const operations = result.current.operations;
    expect(operations.undoStack).toHaveLength(1);
    expect(operations.batchReceipts[0].batch.operations.length).toBeGreaterThan(1);
    expect(operations.batchReceipts[0].batch.operations.every((operation) => operation.author_type === "user" && operation.source?.source_role === "gui_editor")).toBe(true);
    expect(operations.appliedOperations).toHaveLength(1);
    const committedHash = await hashOf(result.current.model.model!);
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(result.current.model.model).toEqual(original);
    act(() => result.current.operations.handleRedoSessionModelEdit());
    expect(await hashOf(result.current.model.model!)).toBe(committedHash);
  });

  it("a commit that creates focuses what it created, as the single change Apply does", async () => {
    const result = await session();
    committed(await run(() => bindCommit(result, newMaterial)({ materialId: "material:new" })));
    expect(result.current.model.model!.materials!.map((material) => material.id)).toContain("material:new");
    expect(result.current.selection.selection).toEqual({ type: "material", id: "material:new" });
  });

  it("the applied-batch path's checkpoint event carries no submitted id: the queue path's events are as they were", async () => {
    const result = await session();
    const { seen } = events(result);
    const model = result.current.model.model!;
    const intent = (await gridCell.build(model, { nodeId: "node:C", axis: "y", value: "2.5" }, { sequence: 1, batchId: "unused" }) as { intent: EditorOperationIntent }).intent;
    await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: "batch:queued", operations: [intent] }); });
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
    const checkpoint = seen.find((event) => event.type === "checkpoint")!;
    expect(checkpoint).toMatchObject({ type: "checkpoint", operationId: "batch:queued" });
    expect(Object.hasOwn(checkpoint, "submittedId")).toBe(false);
    expect(Object.hasOwn(result.current.operations.undoStack[0], "submitted_id")).toBe(false);
  });
});

describe("binding and input guards: no entry takes content built elsewhere", () => {
  it("the binder refuses at bind time, by name, a builder with no declared input guard or one naming an operation's members", async () => {
    const result = await session();
    const build = () => ({ intent: {} as EditorOperationIntent });
    expect(() => result.current.editorBinding.bindCommit({ name: "unguarded", role: "gui_editor", build } as unknown as EditorBuilder<object>)).toThrow("EDITOR-BIND-REFUSED: The builder unguarded has no declared input guard");
    expect(() => result.current.editorBinding.bindCommit({ name: "no-check", role: "gui_editor", input: { members: ["value"] }, build } as unknown as EditorBuilder<object>)).toThrow("has no declared input guard");
    expect(() => result.current.editorBinding.bindDialog({ name: "takes-operations", role: "gui_editor", input: { members: ["value", "operations"], check: () => null }, build })).toThrow("EDITOR-BIND-REFUSED: The builder takes-operations declares operations as input");
    expect(() => result.current.editorBinding.bindCommit({ name: "takes-target", role: "gui_editor", input: { members: ["target"], check: () => null }, build })).toThrow("EDITOR-BIND-REFUSED: The builder takes-target declares target as input");
    expect(() => result.current.editorBinding.bindCommit({ name: "no-role", role: "agent", input: { members: ["value"], check: () => null }, build } as unknown as EditorBuilder<object>)).toThrow("The builder no-role has no editor role.");
  });

  it("a callback runs the guard before the builder: content carrying an operation's or a stamp's members, or a member the guard does not name, is refused with nothing built, toggled or counted", async () => {
    const result = await session();
    const through = passThrough();
    const guarded = { ...through.builder, input: { members: ["key", "value"], check: () => null } as EditorInputGuard } as unknown as EditorBuilder<Record<string, unknown>>;
    const commit = bindCommit(result, guarded);
    const dialog = bindDialog(result, guarded);
    const cases: Array<[string, unknown, string]> = [
      ["an operations array", { key: "k", operations: [] }, "carries operations, a member of an operation, a batch or a stamp"],
      ["an operations array nested in a named member", { key: "k", value: { operations: [] } }, "carries operations, a member of an operation, a batch or a stamp"],
      ["an author", { key: "k", value: { author_type: "user" } }, "carries author_type, a member of an operation, a batch or a stamp"],
      ["a source at the top", { key: "k", source: {} }, "carries source, a member of an operation, a batch or a stamp"],
      ["a batch id", { key: "k", value: { batch_id: "b" } }, "carries batch_id"],
      ["a queue id", { key: "k", value: { queue_id: "q" } }, "carries queue_id"],
      ["a member the guard does not name", { key: "k", extra: 1 }, "carries extra, which the editor does not take"],
      ["a function", { key: "k", value: () => null }, "carries a function"],
      ["an intent itself", { intent: { operation_id: "op" } }, "carries intent, a member of an operation, a batch or a stamp"],
      ["a batch itself", { batch_id: "b", operations: [] }, "carries batch_id, a member of an operation, a batch or a stamp"],
      ["not an object", [{ key: "k" }], "is not the editor's typed values"]
    ];
    const before = snapshot(result), request = requestState(result);
    for (const [name, input, words] of cases) {
      expect(refused(await run(() => commit(input as never))).reason, name).toContain(words);
      expect(refused(await run(() => dialog.preview(input as never))).reason, name).toContain(words);
      expect(requestState(result), name).toEqual(request);
    }
    expect(through.built()).toBe(0);
    expect(snapshot(result)).toEqual(before);
  });
});

describe("the input is owned (P4)", () => {
  /** A builder whose guard names `key` and `value` and takes anything in them; it records the input it was given. */
  function recording() {
    const seen: { checked: unknown[]; built: unknown[] } = { checked: [], built: [] };
    const builder = { name: "recording", role: "gui_editor", input: { members: ["key", "value"], check: (input: unknown) => { seen.checked.push(input); return null; } },
      build: (_model: PreviewModel, input: unknown) => { seen.built.push(input); throw new Error("not built in this test"); } } as unknown as EditorBuilder<Record<string, unknown>>;
    return { builder, seen };
  }
  class Point { x = 1; }
  const accessor = { key: "k" } as Record<string, unknown>;
  Object.defineProperty(accessor, "value", { get: () => 1, enumerable: true });
  const hidden = { key: "k" } as Record<string, unknown>;
  Object.defineProperty(hidden, "value", { value: 1, enumerable: false });
  const symbolKey = { key: "k", [Symbol("tag")]: 1 };
  const cycle: Record<string, unknown> = { key: "k" };
  cycle.value = { back: cycle };
  const extraArray = Object.assign(["a"], { more: 1 });
  let deep: unknown = "end";
  for (let level = 0; level < 40; level++) deep = { next: deep };
  const unreadable = new Proxy({ key: "k" }, { ownKeys: () => { throw new Error("no keys"); } });
  const cases: Array<[string, unknown, string]> = [
    ["an accessor", accessor, "carries an accessor at value"],
    ["a non-enumerable member", hidden, "carries a non-enumerable member value"],
    ["a symbol key", symbolKey, "carries a symbol key"],
    ["a function", { key: "k", value: { run: () => null } }, "carries a function"],
    ["a Map", { key: "k", value: new Map([["a", 1]]) }, "is not plain data"],
    ["a Set", { key: "k", value: new Set([1]) }, "is not plain data"],
    ["a class instance", { key: "k", value: new Point() }, "is not plain data"],
    ["a Proxy over a class instance", { key: "k", value: new Proxy(new Point(), {}) }, "is not plain data"],
    ["a Proxy whose keys cannot be read", unreadable, "could not be read as typed values (no keys)"],
    ["a cycle", cycle, "contains a cycle"],
    ["more entries than the bound", { key: "k", value: new Array(70_000).fill(0) }, "is larger than 65536 entries"],
    ["a nesting deeper than the bound", { key: "k", value: deep }, "is nested deeper than 32 levels"],
    ["a number that is not finite", { key: "k", value: Number.NaN }, "carries a number that is not finite"],
    ["an array with another member", { key: "k", value: extraArray }, "carries an array with the member more"],
    ["an array with holes", { key: "k", value: [1, , 3] }, "carries an array with holes"], // eslint-disable-line no-sparse-arrays
    ["a target at the top", { key: "k", target: "node:C" }, "carries target, a member of an operation"],
    ["a change at the top", { key: "k", change: {} }, "carries change, a member of an operation"],
    ...["source_ref", "source_channel", "source_role", "source_model_hash", "change_kind", "field_path", "precondition", "agentContext", "importContext"].map((member) =>
      [`${member} nested`, { key: "k", value: [{ deeper: { [member]: "x" } }] }, `carries ${member}, a member of an operation`] as [string, unknown, string])
  ];

  it("each kind of input that is not data is refused by name, before the guard and the builder, with nothing toggled or counted", async () => {
    const result = await session();
    const { builder, seen } = recording();
    const commit = bindCommit(result, builder), dialog = bindDialog(result, builder);
    const before = snapshot(result), request = requestState(result);
    for (const [name, input, words] of cases) {
      expect(refused(await run(() => commit(input as never))).reason, name).toContain(words);
      expect(refused(await run(() => dialog.preview(input as never))).reason, name).toContain(words);
      expect(requestState(result), name).toEqual(request);
    }
    expect(seen).toEqual({ checked: [], built: [] });
    expect(snapshot(result)).toEqual(before);
  });

  it("a throw in the guard's check or in the builder is a refusal, never a rejected promise", async () => {
    const result = await session();
    const throwingCheck = { ...gridCell, name: "throwing-check", input: { members: gridCell.input.members, check: () => { throw new Error("check broke"); } } };
    const throwingBuild = { ...gridCell, name: "throwing-build", build: () => { throw new Error("build broke"); } };
    const before = snapshot(result);
    const input = { nodeId: "node:C", axis: "y" as const, value: "2.5" };
    expect(refused(await run(() => bindCommit(result, throwingCheck)(input))).reason).toContain("could not be read as typed values (check broke)");
    expect(refused(await run(() => bindDialog(result, throwingCheck).preview(input))).reason).toContain("check broke");
    expect(refused(await run(() => bindCommit(result, throwingBuild)(input))).reason).toContain("The editor could not build this change: build broke");
    expect(refused(await run(() => bindDialog(result, throwingBuild).preview(input))).reason).toContain("build broke");
    expect(snapshot(result)).toEqual(before);
  });

  it("the guard and the builder get one fresh frozen copy; the caller mutating its object during an async build leaves the built edit unchanged", async () => {
    const result = await session();
    let release!: () => void;
    const gate = new Promise<void>((resolve) => { release = resolve; });
    const received: { checked?: unknown; built?: unknown } = {};
    const slow: EditorBuilder<GridInput> = { ...gridCell, name: "slow-grid-cell",
      input: { members: gridCell.input.members, check: (input) => { received.checked = input; return gridCell.input.check(input); } },
      build: async (model, input, id) => { received.built = input; await gate; return gridCell.build(model, input, id); } };
    const mine: Record<string, unknown> = { nodeId: "node:C", axis: "y", value: "2.5" };
    let pending!: Promise<EditorCommitResult>;
    act(() => { pending = bindCommit(result, slow)(mine as GridInput); });
    await waitFor(() => expect(received.built).toBeDefined());
    mine.value = "9"; mine.axis = "x"; mine.nodeId = "node:A";
    release();
    committed(await run(() => pending));
    expect(received.checked).toBe(received.built);
    expect(received.built).not.toBe(mine);
    expect(Object.isFrozen(received.built)).toBe(true);
    expect(received.built).toEqual({ nodeId: "node:C", axis: "y", value: "2.5" });
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    expect(result.current.model.model!.nodes[0].position.x).not.toBe(9);
    expect(result.current.operations.batchReceipts[0].batch.operations[0].change).toMatchObject({ field_path: "position.y" });
  });

  it("a nested member called target is a typed value, and is accepted", async () => {
    const result = await session();
    type Placement = { placement: { target: string; value: string } };
    const placed: EditorBuilder<Placement> = { name: "placement", role: "gui_editor",
      input: { members: ["placement"], check: (input) => strings(input.placement, "target", "value") ? null : "The placement takes a target node and a value." },
      build: (model, input, id) => gridCell.build(model, { nodeId: input.placement.target, axis: "y", value: input.placement.value }, id) };
    committed(await run(() => bindCommit(result, placed)({ placement: { target: "node:C", value: "2.5" } })));
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
  });
});

describe("the stamp's allowlist, on what a builder returned", () => {
  it("each other origin is refused at the check it names, with the engine never asked and nothing changed", async () => {
    const result = await session();
    const model = structuredClone(result.current.model.model!);
    const editorBuilt = (await gridCell.build(model, { nodeId: "node:C", axis: "y", value: "2.5" }, { sequence: 1, batchId: "b" }) as { intent: EditorOperationIntent }).intent;
    const { author_type: _author, source: _source, ...unstamped } = editorBuilt;
    const agentOperation = { ...editorBuilt, operation_id: "op:agent", author_type: "agent", source: { source_ref: "turn", source_channel: "embedded_agent", source_role: "agent" } };
    const imported = stampImportedBatch(parseOfflineBatch(JSON.stringify({ batch_id: "imported-1", operations: [editorBuilt] })));
    const requeued = requeueRetainedRecord(structuredClone(editorBuilt), 0);
    if (!("batch" in requeued)) throw new Error(requeued.refused);
    const through = passThrough();
    const commit = bindCommit(result, through.builder);
    const origins: Array<[string, unknown, string]> = [
      ["the embedded agent's stamped proposal (it carries the queue id its stamp sets)", { batch: { batch_id: "agent-1", operations: stampAgentOperations([agentOperation], "handle", "request") } }, "carries a queue id"],
      ["an operation with an agent's author", { intent: agentOperation }, "carries another author than the person"],
      ["offline intake's stamped batch", { batch: imported.batch }, "carries another author than the person"],
      ["an import whose operation claims a person", { intent: { ...editorBuilt, source: { source_ref: "file", source_channel: "offline_intake", source_role: "imported" } } }, "carries a source that is not a desktop editor's own"],
      ["a requeued record, as the requeue batch", { batch: requeued.batch }, "is a stored record requeued after reopening"],
      ["a requeued record, as its operation alone", { intent: requeued.batch.operations[0] }, "is a stored record requeued after reopening"],
      ["a retained record with an agent's role", { intent: { ...editorBuilt, source: { source_ref: "retained:0", source_channel: "retained_requeue", source_role: "agent" } } }, "is a stored record requeued after reopening"],
      ["an operation with no author and no source", { intent: unstamped }, "names no author"],
      ["the desktop's channel with another role", { intent: { ...editorBuilt, source: { ...editorBuilt.source!, source_role: "agent" } } }, "carries a source that is not a desktop editor's own"],
      ["a source with a member no editor writes", { intent: { ...editorBuilt, source: { ...editorBuilt.source!, trusted: true } } }, "carries a source that is not a desktop editor's own"],
      ["an operation that was queued", { intent: { ...editorBuilt, queue_id: "editor-intent-1" } }, "carries a queue id"],
      ["a batch with a member no editor writes", { batch: { batch_id: "extra", operations: [editorBuilt], reviewed: true } }, "carries reviewed, which no desktop editor writes"],
      ["a batch with an import's context", { batch: imported.batch, importContext: imported.context }, "context or a precondition"],
      ["an empty batch", { batch: { batch_id: "empty", operations: [] } }, "holds no operation a desktop editor builds"]
    ];
    const validate = vi.spyOn(operationBatchService, "validateOperationBatch");
    const apply = vi.spyOn(operationBatchService, "applyOperationBatch");
    const before = snapshot(result);
    for (const [name, edit, words] of origins) {
      through.next(edit);
      expect(refused(await run(() => commit({ key: name }))).reason, name).toContain(words);
      expect(snapshot(result), name).toEqual(before);
    }
    expect(validate).not.toHaveBeenCalled();
    expect(apply).not.toHaveBeenCalled();
    // Accepted, and stamped by the binding: what an editor builds (the person named, the desktop editor's own source
    // or none). The binding vouches for it; a payload's claim is never read as a grant.
    through.next({ intent: { ...editorBuilt, source: { ...editorBuilt.source!, source_role: "viewport_editor" } } });
    committed(await run(() => commit({ key: "editor-built" })));
    const { source: _dropped, ...withoutSource } = editorBuilt;
    through.next({ intent: { ...withoutSource, operation_id: "op:no-source", change: { ...editorBuilt.change, before: "2.5", after: "2.75" } } });
    committed(await run(() => commit({ key: "no-source" })));
    expect(result.current.operations.batchReceipts.map((receipt) => receipt.batch.operations[0].source)).toEqual([
      { source_ref: `desktop-editor:gui_editor:${editorBuilt.operation_id}:1`, source_channel: "local_desktop_preview", source_role: "gui_editor" },
      { source_ref: "desktop-editor:gui_editor:op:no-source:1", source_channel: "local_desktop_preview", source_role: "gui_editor" }]);
    // A committed record presented again carries the binding's own stamp.
    through.next({ intent: result.current.operations.retainedReviewContext[0] });
    expect(refused(await run(() => commit({ key: "again" }))).reason).toContain("is a record already committed once");
  });

  describe("requeue", () => {
    const record = { operation_id: "op", operation_kind: "modify", target: { object_type: "Node", ref: "n" }, change: { change_kind: "set_field", field_path: "position.x", before: "0", after: "1" }, rationale: "r" };
    const requeued = (stored: object, index: number) => {
      const out = requeueRetainedRecord({ ...record, ...stored }, index);
      if (!("batch" in out)) throw new Error(out.refused);
      return out.batch.operations[0];
    };
    const RETAINED = (index: number) => ({ source_ref: `retained:${index}`, source_channel: "retained_requeue", source_role: "agent" });
    it("keeps an embedded agent's stored reference and channel", () => {
      expect(requeued({ author_type: "agent", source: { source_ref: "agent:turn:call", source_channel: "embedded_agent", source_role: "agent" } }, 4))
        .toMatchObject({ author_type: "agent", source: { source_ref: "agent:turn:call", source_channel: "embedded_agent", source_role: "agent" } });
    });
    it("keeps an offline import's stored reference and channel", () => {
      expect(requeued({ author_type: "agent", source: { source_ref: "offline-import:file:1", source_channel: "offline_intake", source_role: "imported" } }, 5))
        .toMatchObject({ author_type: "agent", source: { source_ref: "offline-import:file:1", source_channel: "offline_intake", source_role: "agent" } });
    });
    it("gives an agent record with an empty stored source the retained mark", () => {
      expect(requeued({ author_type: "agent", source: { source_ref: "", source_channel: "", source_role: "agent" } }, 6)).toMatchObject({ author_type: "agent", source: RETAINED(6) });
      expect(requeued({ author_type: "agent" }, 7)).toMatchObject({ author_type: "agent", source: RETAINED(7) });
    });
    it("gives an agent record on the desktop's own channel the retained mark", () => {
      expect(requeued({ author_type: "agent", source: { source_ref: "turn", source_channel: "local_desktop_preview", source_role: "agent" } }, 8)).toMatchObject({ author_type: "agent", source: RETAINED(8) });
    });
    it("gives an agent record carrying the commit binding's reference the retained mark", () => {
      expect(requeued({ author_type: "agent", source: { source_ref: "desktop-editor:gui_editor:op:1", source_channel: "embedded_agent", source_role: "agent" } }, 9)).toMatchObject({ author_type: "agent", source: RETAINED(9) });
    });
    it("gives a record of the person's the retained mark", () => {
      expect(requeued({ author_type: "user", source: { source_ref: "desktop-editor:gui_editor:op:1", source_channel: "local_desktop_preview", source_role: "gui_editor" } }, 3))
        .toMatchObject({ author_type: "user", source: { source_ref: "retained:3", source_channel: "retained_requeue", source_role: "retained_user" } });
    });
  });
});

describe("refusals change nothing", () => {
  it("the engine's refusal in validation returns its reason and frozen findings; the model, the history and the receipts stay as they were", async () => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    committed(await run(() => bindCommit(result, gridCell)({ nodeId: "node:B", axis: "y", value: "0.5" })));
    const before = snapshot(result);
    const apply = vi.spyOn(operationBatchService, "applyOperationBatch");
    // A builder with the `before` the field had: the engine refuses the stale basis.
    const stale: EditorBuilder<GridInput> = { ...gridCell, name: "stale-grid-cell", build: (_model, input, id) => gridCell.build(original, input, id) };
    const outcome = refused(await run(() => bindCommit(result, stale)({ nodeId: "node:B", axis: "y", value: "0.75" })));
    expect(outcome.findings.filter((item) => item.severity === "blocking").length).toBeGreaterThan(0);
    for (const finding of outcome.findings.filter((item) => item.severity === "blocking")) expect(outcome.reason).toContain(finding.message);
    expect(deepFrozen(outcome)).toBe(true);
    expect(snapshot(result)).toEqual(before);
    expect(apply).not.toHaveBeenCalled();
  });

  it("the engine refusing in apply mode, and each mismatch in the engine's replies, are refused by name with nothing changed", async () => {
    const result = await session();
    const commit = bindCommit(result, gridCell);
    const input = { nodeId: "node:C", axis: "y" as const, value: "2.5" };
    const cases: Array<[string, () => void, string]> = [
      ["validation of other content", () => alter("validateOperationBatch", (outcome) => { outcome.batch_hash = `sha256:${"0".repeat(64)}`; }), "is of other content than this change"],
      ["validation with no preview generated", () => alter("validateOperationBatch", (outcome) => { outcome.validation.diff_preview_status = "blocked_by_validation"; }), "generated no preview"],
      ["validation not discarded", () => alter("validateOperationBatch", (outcome) => { outcome.simulation_disposition = "committed_as_one_batch"; }), "did not end as a discarded validation"],
      ["validation in another mode", () => alter("validateOperationBatch", (outcome) => { outcome.mode = "apply"; }), "in another mode than validate-only"],
      ["validation of another batch", () => alter("validateOperationBatch", (outcome) => { outcome.batch_id = "other"; }), "names another batch"],
      ["validation on another model", () => alter("validateOperationBatch", (outcome) => { outcome.initial_model_hash = { ...outcome.initial_model_hash, value: `sha256:${"1".repeat(64)}` }; }), "made on another model"],
      ["the engine refusing in apply mode", () => alter("applyOperationBatch", (outcome) => { outcome.validation.application_status = "blocked"; outcome.applied_model = null;
        outcome.diagnostics = [...outcome.diagnostics, { id: "d", code: "INVENTED-APPLY-REFUSAL", severity: "blocking", message: "Invented apply refusal.", remediation: "", affected_refs: [], source: "test" }]; }), "Invented apply refusal."],
      ["an application of other content", () => alter("applyOperationBatch", (outcome) => { outcome.batch_hash = `sha256:${"2".repeat(64)}`; }), "applied other content than this change"],
      ["an applied model with another hash", () => alter("applyOperationBatch", (outcome) => { outcome.applied_model_backend_hash = `sha256:${"3".repeat(64)}`; }), "does not match the hash the engine reported"],
      ["an incomplete receipt", () => alter("applyOperationBatch", (outcome) => { outcome.acceptance = null; }), "did not return a complete application receipt"]
    ];
    const before = snapshot(result);
    for (const [name, arrange, words] of cases) {
      arrange();
      expect(refused(await run(() => commit(input))).reason, name).toContain(words);
      expect(snapshot(result), name).toEqual(before);
      vi.restoreAllMocks();
    }
    committed(await run(() => commit(input)));
  });

  it("while another request runs, a call is refused at once and the running one is not disturbed", async () => {
    const result = await session();
    const commit = bindCommit(result, gridCell);
    const held = hold("validateOperationBatch");
    let first: Promise<unknown> = Promise.resolve();
    act(() => { first = commit({ nodeId: "node:C", axis: "y", value: "2.5" }); });
    await held.arrived;
    const request = requestState(result);
    expect(refused(await run(() => commit({ nodeId: "node:B", axis: "y", value: "0.5" }))).reason).toContain("Another change is still being checked or applied");
    expect(requestState(result)).toEqual(request);
    held.release();
    expect(committed(await run(() => first as Promise<EditorCommitResult>))).toBeTruthy();
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    expect(result.current.model.model!.nodes[1].position.y).toBe(0);
  });
});

describe("what the person previewed is what is applied (P2)", () => {
  it("a nonempty reconnect whose preview rows are missing is refused before a handle is issued", async () => {
    const result = await session();
    const before = snapshot(result);
    alter("validateOperationBatch", (outcome) => { outcome.operation_outcomes = []; });
    const previewed = await run(() => bindDialog(result, reconnect).preview({ pipeId: "pipe:P", targetNode: "node:C" }));
    expect(refused(previewed).reason).toMatch(/holds 0 operation steps for the [1-9]\d* submitted/);
    expect(snapshot(result)).toEqual(before);
  });

  it("reordered rows are refused, in the preview and in the application", async () => {
    const result = await session(bulkChangeModel);
    const input = { scope: [{ type: "pipe" as const, id: "pipe:P2" }, { type: "pipe" as const, id: "pipe:P3" }], request: { kind: "material" as const, materialId: "material:alloy" } };
    const before = snapshot(result);
    alter("validateOperationBatch", (outcome) => { outcome.operation_outcomes.reverse(); });
    expect(refused(await run(() => bindCommit(result, bulkChange)(input))).reason).toContain("does not match submitted operation 1 at its place");
    alter("applyOperationBatch", (outcome) => { outcome.operation_outcomes.reverse(); });
    expect(refused(await run(() => bindCommit(result, bulkChange)(input))).reason).toContain("does not match submitted operation 1 at its place");
    expect(snapshot(result)).toEqual(before);
  });

  it("an apply reply that reports application with a blocking finding is refused", async () => {
    const result = await session();
    const before = snapshot(result);
    alter("applyOperationBatch", (outcome) => { outcome.diagnostics = [...outcome.diagnostics, { id: "d", code: "INVENTED-BLOCKING", severity: "blocking", message: "Invented blocking finding.", remediation: "", affected_refs: [], source: "test" }]; });
    const outcome = refused(await run(() => bindCommit(result, gridCell)({ nodeId: "node:C", axis: "y", value: "2.5" })));
    expect(outcome.reason).toContain("Invented blocking finding.");
    expect(outcome.findings.map((item) => item.code)).toContain("INVENTED-BLOCKING");
    expect(snapshot(result)).toEqual(before);
  });

  it("an apply reply with no applied-model hash is refused", async () => {
    const result = await session();
    const before = snapshot(result);
    alter("applyOperationBatch", (outcome) => { outcome.applied_model_backend_hash = null; });
    expect(refused(await run(() => bindCommit(result, gridCell)({ nodeId: "node:C", axis: "y", value: "2.5" }))).reason).toContain("reported no hash of the applied model");
    expect(snapshot(result)).toEqual(before);
  });

  it("an applied model whose hash cannot be computed is refused", async () => {
    const result = await session();
    const before = snapshot(result);
    const real = hashService.computeModelHash;
    vi.spyOn(hashService, "computeModelHash").mockImplementation(async (model) => model.nodes[2]?.position.y === 2.5 ? null : real(model));
    expect(refused(await run(() => bindCommit(result, gridCell)({ nodeId: "node:C", axis: "y", value: "2.5" }))).reason).toContain("hash could not be computed");
    expect(snapshot(result)).toEqual(before);
  });

  it("an apply reply with neither hash present is refused: an absent hash never equals another absent hash", async () => {
    const result = await session();
    const before = snapshot(result);
    const real = hashService.computeModelHash;
    vi.spyOn(hashService, "computeModelHash").mockImplementation(async (model) => model.nodes[2]?.position.y === 2.5 ? null : real(model));
    alter("applyOperationBatch", (outcome) => { delete (outcome as { applied_model_backend_hash?: unknown }).applied_model_backend_hash; });
    expect(refused(await run(() => bindCommit(result, gridCell)({ nodeId: "node:C", axis: "y", value: "2.5" }))).reason).toContain("reported no hash of the applied model");
    expect(snapshot(result)).toEqual(before);
  });
});

describe("supersession: a commit started on one model never lands on another", () => {
  async function savedSession() {
    const result = await session();
    await act(async () => { await result.current.project.handleSaveProject(); });
    await waitFor(() => expect(result.current.project.projectBusy).toBe(false));
    return { result, projectId: result.current.model.model!.project.id };
  }
  it.each([["the builder's await"], ["validation"], ["application"]])("a project opened during %s refuses the commit; nothing of it reaches the opened project", async (at) => {
    const { result, projectId } = await savedSession();
    let release!: () => void, arrived!: Promise<void>;
    let commit: (input: GridInput) => Promise<unknown>;
    if (at === "the builder's await") {
      const gate = new Promise<void>((resolve) => { release = resolve; });
      let reached!: () => void;
      arrived = new Promise<void>((resolve) => { reached = resolve; });
      commit = bindCommit(result, { ...gridCell, name: "slow-grid-cell", build: async (model, input, id) => { reached(); await gate; return gridCell.build(model, input, id); } });
    } else {
      ({ release, arrived } = hold(at === "validation" ? "validateOperationBatch" : "applyOperationBatch"));
      commit = bindCommit(result, gridCell);
    }
    let pending: Promise<unknown> = Promise.resolve();
    act(() => { pending = commit({ nodeId: "node:C", axis: "y", value: "2.5" }); });
    await arrived;
    const generation = result.current.model.projectSessionGeneration;
    await act(async () => { await result.current.project.handleOpenProject(projectId); });
    await waitFor(() => expect(result.current.project.projectBusy).toBe(false));
    expect(result.current.model.projectSessionGeneration).not.toBe(generation);
    const reopened = snapshot(result);
    release();
    expect(refused(await run(() => pending as Promise<EditorCommitResult>)).reason).toContain("changed while this edit was checked");
    expect(snapshot(result)).toEqual(reopened);
    expect(result.current.model.model!.nodes[2].position.y).toBe(2);
  });
});

describe("lifetime and currentness (P3)", () => {
  async function mounted(load: () => PreviewModel = reconnectModel) {
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(load());
    const hook = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
    return hook;
  }
  it.each([["the builder's await"], ["validation"], ["application"]])("an unmount during %s: nothing reports committed, and the engine applies nothing after an unmount before application", async (at) => {
    const hook = await mounted();
    let release!: () => void, arrived!: Promise<void>;
    let commit: (input: GridInput) => Promise<EditorCommitResult>;
    if (at === "the builder's await") {
      const gate = new Promise<void>((resolve) => { release = resolve; });
      let reached!: () => void;
      arrived = new Promise<void>((resolve) => { reached = resolve; });
      commit = hook.result.current.editorBinding.bindCommit({ ...gridCell, name: "slow-grid-cell", build: async (model, input, id) => { reached(); await gate; return gridCell.build(model, input, id); } });
    } else {
      ({ release, arrived } = hold(at === "validation" ? "validateOperationBatch" : "applyOperationBatch"));
      commit = hook.result.current.editorBinding.bindCommit(gridCell);
    }
    // Once the hold is in place, any further call of the engine's apply is a call after the unmount.
    const applied = vi.spyOn(operationBatchService, "applyOperationBatch");
    let pending: Promise<EditorCommitResult> = Promise.resolve(null as never);
    act(() => { pending = commit({ nodeId: "node:C", axis: "y", value: "2.5" }); });
    await arrived;
    const callsAtUnmount = applied.mock.calls.length;
    hook.unmount();
    release();
    const outcome = await pending;
    expect(outcome.status).toBe("refused");
    expect(applied.mock.calls.length).toBe(callsAtUnmount);
  });

  it("a callback kept after its workspace closed is refused at entry, with nothing built", async () => {
    const hook = await mounted();
    let built = 0;
    const commit = hook.result.current.editorBinding.bindCommit({ ...gridCell, name: "counted", build: (model, input, id) => { built += 1; return gridCell.build(model, input, id); } });
    const dialog = hook.result.current.editorBinding.bindDialog(reconnect);
    hook.unmount();
    expect(refused(await commit({ nodeId: "node:C", axis: "y", value: "2.5" })).reason).toContain("This workspace is closed");
    expect(refused(await dialog.preview({ pipeId: "pipe:P", targetNode: "node:C" })).reason).toContain("This workspace is closed");
    expect(refused(await dialog.commit({ handle: "editor-preview" })).reason).toContain("This workspace is closed");
    expect(built).toBe(0);
  });

  it("a project opened as the validation returns, before the engine is asked to apply: the engine applies nothing", async () => {
    const result = await session();
    await act(async () => { await result.current.project.handleSaveProject(); });
    await waitFor(() => expect(result.current.project.projectBusy).toBe(false));
    const projectId = result.current.model.model!.project.id;
    const apply = vi.spyOn(operationBatchService, "applyOperationBatch");
    const real = operationBatchService.validateOperationBatch;
    vi.spyOn(operationBatchService, "validateOperationBatch").mockImplementationOnce(async (...args) => {
      const outcome = await real(...args);
      // The reply is ready; the project is replaced before the session resumes with it.
      await result.current.project.handleOpenProject(projectId);
      return outcome;
    });
    const outcome = await run(() => bindCommit(result, gridCell)({ nodeId: "node:C", axis: "y", value: "2.5" }));
    await waitFor(() => expect(result.current.project.projectBusy).toBe(false));
    expect(refused(outcome).reason).toContain("changed while this edit was checked");
    expect(apply).not.toHaveBeenCalled();
    expect(result.current.model.model!.nodes[2].position.y).toBe(2);
  });
});

describe("live values at publication", () => {
  it("a result published during the commit's awaits is cleared and reported as cleared; a selection changed during them is the one Undo restores", async () => {
    const result = await session();
    const held = hold("validateOperationBatch");
    let pending: Promise<unknown> = Promise.resolve();
    act(() => { pending = bindCommit(result, gridCell)({ nodeId: "node:C", axis: "y", value: "2.5" }); });
    await held.arrived;
    await act(async () => { await result.current.results.handleInspectBundledReference(); });
    await waitFor(() => expect(result.current.results.historicalRun).not.toBeNull());
    act(() => { result.current.selection.handleSelectEntity({ type: "node", id: "node:D" }); });
    held.release();
    const outcome = committed(await run(() => pending as Promise<EditorCommitResult>));
    expect(outcome.resultsCleared).toBe(true);
    expect(result.current.results.historicalRun).toBeNull();
    expect(result.current.operations.undoStack[0].selection).toEqual({ type: "node", id: "node:D" });
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(result.current.selection.selection).toEqual({ type: "node", id: "node:D" });
  });

  it("a listener's failure after publication is a notice on a committed result, and the session's own clears complete", async () => {
    const result = await session();
    await act(async () => { await result.current.results.handleInspectBundledReference(); });
    await waitFor(() => expect(result.current.results.historicalRun).not.toBeNull());
    result.current.operations.sessionEvents.listen((event) => event.type === "checkpoint" ? () => { throw new Error("invented listener failure"); } : undefined);
    const outcome = committed(await run(() => bindCommit(result, gridCell)({ nodeId: "node:C", axis: "y", value: "2.5" })));
    expect(outcome.notice).toContain("invented listener failure");
    expect(outcome.resultsCleared).toBe(true);
    expect(result.current.results.historicalRun).toBeNull();
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    expect(result.current.operations.undoStack).toHaveLength(1);
  });
});

describe("a listener cannot split publication (P5)", () => {
  /** The session with a listener registered before the run sheet's, and the run sheet's own store, as the application holds it. */
  async function withSheet(first: SessionEventListener) {
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(reconnectModel());
    const hook = renderHook(() => {
      const current = useWorkspaceSession();
      useLayoutEffect(() => current.operations.sessionEvents.listen(first), [current.operations.sessionEvents]);
      const sheet = useRunSheetStore(current.operations.sessionEvents, null, current.model.model?.project.id ?? null);
      return { current, sheet };
    });
    await waitFor(() => expect(hook.result.current.current.model.model).not.toBeNull());
    return { result: { get current() { return hook.result.current.current; } } as Session, sheet: () => hook.result.current.sheet };
  }
  const throwsOnCheckpoint: SessionEventListener = (event) => event.type === "checkpoint" ? () => { throw new Error("invented first listener failure"); } : undefined;
  const ledgerIds = (sheet: () => ReturnType<typeof useRunSheetStore>) => sheet().current().ledger.records.map((record) => record.operationId);

  it("a checkpoint listener throwing before the run sheet's: the commit is committed with a notice, and the run sheet still takes the checkpoint", async () => {
    const { result, sheet } = await withSheet(throwsOnCheckpoint);
    const outcome = committed(await run(() => bindCommit(result, gridCell)({ nodeId: "node:C", axis: "y", value: "2.5" })));
    expect(outcome.notice).toContain("A view of the session did not take this change: invented first listener failure");
    expect(ledgerIds(sheet)).toEqual([result.current.operations.undoStack[0].operation_id]);
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
  });

  it("the queue's Apply with a throwing first listener behaves as before (same outcome, same message), and the later listener has run", async () => {
    const { result, sheet } = await withSheet(throwsOnCheckpoint);
    const model = result.current.model.model!;
    const intent = (await gridCell.build(model, { nodeId: "node:C", axis: "y", value: "2.5" }, { sequence: 1, batchId: "unused" }) as { intent: EditorOperationIntent }).intent;
    await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: "batch:queued", operations: [intent] }); });
    const message = result.current.operations.batchMessage;
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
    // As before the publish closure ran every listener: the model, the history and the queue are published, the
    // failure leaves the batch message as it was, and the first failure still reaches the Apply path.
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.operations.queuedBatches).toHaveLength(0);
    expect(result.current.operations.batchMessage).toBe(message);
    // And the run sheet, listening after the one that threw, took the checkpoint.
    expect(ledgerIds(sheet)).toEqual(["batch:queued"]);
  });

  it("a report-source subscriber throwing during the model write: the commit is committed, the rendered model is the new one, and the failure is reported once", async () => {
    const result = await session();
    const reported = vi.spyOn(console, "error").mockImplementation(() => undefined);
    let armed = true;
    result.current.results.reportSource.subscribe(() => { if (armed) { armed = false; throw new Error("invented subscriber failure"); } });
    const outcome = committed(await run(() => bindCommit(result, gridCell)({ nodeId: "node:C", axis: "y", value: "2.5" })));
    expect(armed).toBe(false);
    expect(outcome.notice).toBeNull();
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    expect(result.current.results.reportSourceSequence).toBe(result.current.results.reportSource.sequence);
    expect(reported.mock.calls.filter((call) => String(call[0]).startsWith("A report-source subscriber failed"))).toHaveLength(1);
  });

  it("the checkpoint event's payload is detached: a listener changing what it received changes nothing the session keeps", async () => {
    const result = await session();
    let received: Extract<SessionEvent, { type: "checkpoint" }> | null = null;
    result.current.operations.sessionEvents.listen((event) => { if (event.type === "checkpoint") received = event as typeof received; });
    committed(await run(() => bindCommit(result, gridCell)({ nodeId: "node:C", axis: "y", value: "2.5" })));
    const event = received!;
    const receipt = result.current.operations.batchReceipts[0].outcome;
    expect(event.appliedModel).not.toBe(receipt.applied_model);
    expect(event.appliedModel).not.toBe(result.current.model.model);
    expect(event.operationOutcomes).not.toBe(receipt.operation_outcomes);
    const kept = snapshot(result);
    (event.appliedModel as PreviewModel).nodes[2].position.y = 99;
    (event.operationOutcomes as unknown as Array<Record<string, unknown>>)[0].change_kind = "altered";
    expect(snapshot(result)).toEqual(kept);
  });
});

describe("form 2: the dialog's single-use preview handle", () => {
  it("a reconnect commits only with its own dialog's handle, on the current model, once", async () => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    const originalHash = await hashOf(original);
    const dialog = bindDialog(result, reconnect);
    const otherDialog = bindDialog(result, reconnect);
    // Form 1 never commits an operation that requires its dialog's preview.
    expect(refused(await run(() => bindCommit(result, reconnect)({ pipeId: "pipe:P", targetNode: "node:C" }))).reason).toContain("reconnect_pipe_run");
    const before = snapshot(result), request = requestState(result);
    expect(refused(await run(() => dialog.commit({ handle: "editor-preview" }))).reason).toContain("is not a preview this session showed");
    expect(refused(await run(() => dialog.commit({ pipeId: "pipe:P", targetNode: "node:C" } as never))).reason).toContain("is not a preview this session showed");
    expect(requestState(result)).toEqual(request);

    const previewed = await run(() => dialog.preview({ pipeId: "pipe:P", targetNode: "node:C" }));
    if (previewed.status !== "previewed") throw new Error(previewed.reason);
    expect(deepFrozen(previewed)).toBe(true);
    expect(previewed.extent.steps.map((step) => step.changeKind)).toContain("reconnect_pipe_run");
    expect(result.current.model.model).toEqual(original);
    expect(refused(await run(() => otherDialog.commit(previewed.handle))).reason).toContain("was shown by another editor");
    expect(snapshot(result)).toEqual(before);

    // The model changes: the handle is of an earlier model, and stays so after Undo back to the same content.
    committed(await run(() => bindCommit(result, gridCell)({ nodeId: "node:D", axis: "y", value: "0.5" })));
    expect(refused(await run(() => dialog.commit(previewed.handle))).reason).toContain("is of an earlier model");
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(await hashOf(result.current.model.model!)).toBe(originalHash);
    expect(refused(await run(() => dialog.commit(previewed.handle))).reason).toContain("is of an earlier model");
    expect(result.current.model.model).toEqual(original);

    // A fresh preview on the current model commits once; the handle is then spent, and Undo does not revive it.
    const current = await run(() => dialog.preview({ pipeId: "pipe:P", targetNode: "node:C" }));
    if (current.status !== "previewed") throw new Error(current.reason);
    committed(await run(() => dialog.commit(current.handle)));
    expect(result.current.model.model!.pipe_segments.find((pipe) => pipe.id === "pipe:P")!.to).toBe("node:C");
    expect(refused(await run(() => dialog.commit(current.handle))).reason).toContain("was already committed");
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(await hashOf(result.current.model.model!)).toBe(originalHash);
    expect(refused(await run(() => dialog.commit(current.handle))).reason).toContain("was already committed");
  });

  it("nothing returned is an object the session keeps: what a caller holds cannot change the model, the receipts or a later commit", async () => {
    const result = await session();
    const dialog = bindDialog(result, reconnect);
    const previewed = await run(() => dialog.preview({ pipeId: "pipe:P", targetNode: "node:C" }));
    if (previewed.status !== "previewed") throw new Error(previewed.reason);
    expect(() => { (previewed.extent.steps[0] as { changeKind: string }).changeKind = "set_field"; }).toThrow();
    expect(() => { (previewed.extent.steps[0].diff as unknown[]).push({}); }).toThrow();
    const outcome = committed(await run(() => dialog.commit(previewed.handle)));
    expect(() => { (outcome as { checkpointId: string }).checkpointId = "x"; }).toThrow();
    const hash = await hashOf(result.current.model.model!);
    const receipts = structuredClone(result.current.operations.batchReceipts);
    expect(JSON.stringify(previewed)).not.toContain("applied_model");
    expect(Object.values(outcome)).not.toContain(result.current.operations.batchReceipts[0].outcome);
    expect(await hashOf(result.current.model.model!)).toBe(hash);
    expect(result.current.operations.batchReceipts).toEqual(receipts);
  });
});

describe("save, reopen and requeue of a committed record", () => {
  it("is refused by the commit entry and reaches the model only by the review queue's Apply", async () => {
    const result = await session();
    committed(await run(() => bindCommit(result, gridCell)({ nodeId: "node:C", axis: "y", value: "2.5" })));
    // Undone, so the stored record's before value is the model's again; the record stays in the review context.
    act(() => result.current.operations.handleUndoSessionModelEdit());
    const projectId = result.current.model.model!.project.id;
    await act(async () => { await result.current.project.handleSaveProject(); });
    await waitFor(() => expect(result.current.project.projectBusy).toBe(false));
    await act(async () => { await result.current.project.handleOpenProject(projectId); });
    await waitFor(() => expect(result.current.project.projectBusy).toBe(false));
    const stored = result.current.operations.retainedReviewContext;
    expect(stored).toHaveLength(1);
    expect(stored[0].source?.source_ref).toMatch(/^desktop-editor:/);
    expect(result.current.operations.undoStack).toHaveLength(0);
    const reopened = structuredClone(result.current.model.model!);
    // Requeued: the record says what it is.
    let refusal: string | null = "not run";
    await act(async () => { refusal = await result.current.operations.handleRequeueRetainedRecord(structuredClone(stored[0]), 0); });
    expect(refusal).toBeNull();
    const entry = result.current.operations.queuedBatches[0];
    expect(entry.batch.operations[0]).toMatchObject({ author_type: "user", source: { source_ref: "retained:0", source_channel: "retained_requeue", source_role: "retained_user" } });
    // The commit entry refuses it, as the queued batch and as its operation alone.
    const through = passThrough();
    const commit = bindCommit(result, through.builder);
    through.next({ batch: entry.batch });
    expect(refused(await run(() => commit({ key: "requeued" }))).reason).toContain("is a stored record requeued after reopening");
    through.next({ intent: entry.batch.operations[0] });
    expect(refused(await run(() => commit({ key: "requeued-operation" }))).reason).toContain("is a stored record requeued after reopening");
    expect(result.current.model.model).toEqual(reopened);
    // The review queue applies it, only by Apply.
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], false); });
    expect(result.current.model.model).toEqual(reopened);
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    expect(result.current.operations.batchReceipts.at(-1)!.batch.operations[0].source?.source_ref).toBe("retained:0");
  });
});

describe("DEL-16-06: the binder and the command triggers are out of reach of the agent's host and offline intake", () => {
  const SOURCE = resolve(__dirname, "../..");
  const sources = (dir: string): string[] => readdirSync(dir).flatMap((name) => {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) return name === "test" || name === "test-support" ? [] : sources(path);
    return /\.(ts|tsx)$/.test(name) && !/\.test\.tsx?$/.test(name) ? [path] : [];
  });
  const files = sources(SOURCE).map((path) => ({ file: relative(SOURCE, path).split("\\").join("/"), text: readFileSync(path, "utf8") }));
  const REACH = /\b(editorBinding|bindCommit|bindDialog|stampEditorCommit|editorBuilderRefusal|editorInputRefusal|runMenuCommand|runMenuCommandRef|handleToolkitCommand|useSessionOperations|useWorkspaceSessionContext|useWorkspaceSession|handleApplyIntent|handleApplyNextQueuedIntent|handleRunOperationBatch|handleApplyDraftReview|applyOperationBatch|applyModelOperation)\b/;

  it("the agent's host is constructed with its five functions only", async () => {
    const result = await session();
    const owner = (result.current.agent as unknown as { owner: Record<string, unknown> }).owner;
    expect(Object.keys(owner).sort()).toEqual(["proposal", "register", "snapshot", "solve", "withdraw"]);
    const held = Object.values(owner);
    for (const entry of [result.current.editorBinding, result.current.editorBinding.bindCommit, result.current.editorBinding.bindDialog,
      result.current.operations.handleApplyIntent, result.current.operations.handleRunOperationBatch]) expect(held).not.toContain(entry);
    const agent = files.find((entry) => entry.file === "services/embeddedAgentSession.ts")!.text;
    const start = agent.indexOf("export interface WorkspaceExecution {");
    const declared = agent.slice(start, agent.indexOf("\n}", start));
    expect([...declared.matchAll(/^\s*(\w+)\(/gm)].map((match) => match[1]).sort()).toEqual(["proposal", "register", "snapshot", "solve", "withdraw"]);
  });

  it("no source of the agent, its proposals or offline intake names the binder, a command trigger, the session's context hooks or an apply", () => {
    const reached = files.filter(({ file }) => /^(features\/(embedded-agent|agent-proposals|offline-proposal-intake)\/|services\/embeddedAgent)/.test(file));
    expect(reached.length).toBeGreaterThan(3);
    expect(reached.filter(({ text }) => REACH.test(text)).map(({ file }) => file)).toEqual([]);
  });

  it("only the session, the stamp's module and the application root name the binder", () => {
    const named: Record<string, number> = {};
    for (const { file, text } of files) { const n = (text.match(/\beditorBinding\b|\bbindCommit\b|\bbindDialog\b/g) ?? []).length; if (n) named[file] = n; }
    expect(Object.keys(named).sort()).toEqual(["App.tsx", "features/workspace/workspaceSession.ts", "services/operationAuthorship.ts"]);
    const app = files.find((entry) => entry.file === "App.tsx")!.text;
    expect(app.match(/\beditorBinding\b/g)).toHaveLength(1);
    expect(app).toMatch(/const \{ editorBinding, \.\.\.session \} = useWorkspaceSession\(\);/);
  });
});

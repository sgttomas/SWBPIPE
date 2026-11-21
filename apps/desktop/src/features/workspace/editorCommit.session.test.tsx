/** DEL-16-03, Commit: the session's one entry for a person's own edit in a live desktop editor
 * (`commitEditorEdit`, with `previewEditorEdit` for an action's own dialog). This slice builds the
 * entry alone: no editor calls it yet, so the intents and batches here are built as the editors
 * build them (the grid's cell intent, the bulk change builder, the reconnect builder). Every case
 * runs through the session and the actual wasm engine on an invented inline model.
 * DEL-16-06: the entry is not among what the embedded agent's host or offline intake hold. */
import { act, renderHook, waitFor } from "@testing-library/react";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import * as operationBatchService from "../../services/operationBatchService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { requeueRetainedRecord, stampImportedBatch } from "../../services/operationAuthorship";
import { stampAgentOperations } from "../../services/embeddedAgentSession";
import { parseOfflineBatch } from "../offline-proposal-intake/OfflineProposalIntakePanel";
import { buildReconnectBatch, emptyGeometryDraft, type GeometryDraft } from "../geometry-tools/geometryDraft";
import { buildBulkChangeBatch } from "../bulk-change/bulkChange";
import { buildGridOperationIntent, type GridColumn } from "./table/modelTableAdapter";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";
import { reconnectModel } from "../../test-support/reconnectModel";
import { bulkChangeModel } from "../../test-support/bulkChangeModel";
import type { EditorOperationIntent, PreviewModel } from "../../types";

type Operations = WorkspaceSession["operations"];
type Edit = Parameters<Operations["commitEditorEdit"]>[1];

afterEach(() => { vi.restoreAllMocks(); });

async function session(load: () => PreviewModel = reconnectModel) {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(load());
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
type Session = Awaited<ReturnType<typeof session>>;

/** The grid's cell intent for node `index`'s y coordinate, as the grid builds it. */
function coordinateIntent(model: PreviewModel, index: number, value: string, sequence: number): EditorOperationIntent {
  const target = model.nodes[index];
  const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => model.project.units.length, value: () => String(target.position.y) };
  return buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence, value, interaction: "cell" });
}
function reconnectDraft(over: Partial<GeometryDraft> = {}): GeometryDraft {
  return { ...emptyGeometryDraft(), action: "reconnect", pipeRefs: ["pipe:P"], reconnectEnd: "to", targetNode: "node:C", yReference: { x: "0", y: "0", z: "1" }, ...over };
}
const hashOf = async (model: PreviewModel) => (await computeModelHash(model))!.value;

/** Everything a commit may change: the model, the history, the receipts and the review queue. */
function snapshot(result: Session) {
  const operations = result.current.operations;
  return structuredClone({
    model: result.current.model.model,
    undoStack: operations.undoStack,
    redoStack: operations.redoStack,
    batchReceipts: operations.batchReceipts,
    retainedReviewContext: operations.retainedReviewContext,
    appliedOperations: operations.appliedOperations,
    queuedBatches: operations.queuedBatches,
    editorIntents: operations.editorIntents,
    batchOutcomes: operations.batchOutcomes,
    operationOutcomes: operations.operationOutcomes
  });
}

async function commit(result: Session, route: unknown, edit: unknown, options?: Parameters<Operations["commitEditorEdit"]>[2]) {
  let outcome: Awaited<ReturnType<Operations["commitEditorEdit"]>> | undefined;
  await act(async () => { outcome = await result.current.operations.commitEditorEdit(route as never, edit as Edit, options); });
  return outcome!;
}
async function preview(result: Session, route: unknown, edit: unknown) {
  let outcome: Awaited<ReturnType<Operations["previewEditorEdit"]>> | undefined;
  await act(async () => { outcome = await result.current.operations.previewEditorEdit(route as never, edit as Edit); });
  return outcome!;
}
const gui = (result: Session) => result.current.operations.editorRoutes.gui;
const viewport = (result: Session) => result.current.operations.editorRoutes.viewport;
const committed = (outcome: Awaited<ReturnType<typeof commit>>) => {
  if (outcome.status !== "committed") throw new Error(`expected a commit, got a refusal: ${outcome.reason}`);
  return outcome;
};

describe("commit entry: one checkpoint, the receipts of an applied batch, Undo and Redo", () => {
  it("a single intent commits as one checkpoint, stamped by the route; Undo restores the canonical hash and Redo reapplies", async () => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    const originalHash = await hashOf(original);
    const intent = coordinateIntent(original, 2, "2.5", 1);
    // The payload's own source is not read: the route stamps its own role.
    const outcome = committed(await commit(result, gui(result), { intent: { ...intent, source: { ...intent.source!, source_role: "viewport_editor" } } }));

    const operations = result.current.operations;
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    const committedHash = await hashOf(result.current.model.model!);
    expect(committedHash).not.toBe(originalHash);
    expect(operations.undoStack).toHaveLength(1);
    expect(operations.undoStack[0]).toMatchObject({ checkpoint_id: outcome.checkpointId, operation_id: intent.operation_id, model: original });
    expect(operations.redoStack).toHaveLength(0);
    // One decision record (the batch receipt with the engine's acceptance) and one outcome record, as for an applied queued batch.
    expect(operations.batchReceipts).toHaveLength(1);
    expect(operations.batchReceipts[0].outcome.acceptance).not.toBeNull();
    expect(operations.batchReceipts[0].batch.operations).toEqual([{ ...intent, author_type: "user",
      source: { source_ref: `desktop-editor:gui_editor:${intent.operation_id}:1`, source_channel: "local_desktop_preview", source_role: "gui_editor" } }]);
    expect(operations.appliedOperations).toHaveLength(1);
    expect(operations.appliedOperations[0]).toMatchObject({ receipt_id: outcome.checkpointId.replace(/^undo-/, "applied-"), sequence: 1, operation_id: intent.operation_id,
      before: originalHash, after: committedHash, applied_model_hash: committedHash, field_path: "batch" });
    expect(operations.retainedReviewContext).toEqual(operations.batchReceipts[0].batch.operations);
    expect(outcome.outcome.mode).toBe("apply");
    expect(outcome.resultsCleared).toBe(false);
    // Nothing went through the review queue.
    expect(operations.queuedBatches).toHaveLength(0);
    expect(operations.editorIntents).toHaveLength(0);

    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(await hashOf(result.current.model.model!)).toBe(originalHash);
    expect(result.current.operations.undoStack).toHaveLength(0);
    act(() => result.current.operations.handleRedoSessionModelEdit());
    expect(await hashOf(result.current.model.model!)).toBe(committedHash);
    expect(result.current.operations.undoStack).toHaveLength(1);
  });

  it("a batch built as the bulk change builds it commits whole as one checkpoint; one Undo reverts every change in it", async () => {
    const result = await session(bulkChangeModel);
    const original = structuredClone(result.current.model.model!);
    const originalHash = await hashOf(original);
    const batch = await buildBulkChangeBatch(original, [{ type: "pipe", id: "pipe:P2" }, { type: "pipe", id: "pipe:P3" }], { kind: "material", materialId: "material:alloy" }, canonicalJsonString, "bulk-material-1");
    expect(batch.operations.length).toBeGreaterThan(1);
    const outcome = committed(await commit(result, viewport(result), { batch }));

    const operations = result.current.operations;
    expect(result.current.model.model!.pipe_segments.map((pipe) => pipe.material)).toEqual(["material:steel", "material:alloy", "material:alloy"]);
    expect(outcome.batchId).toBe("bulk-material-1");
    expect(operations.undoStack).toHaveLength(1);
    expect(operations.undoStack[0].operation_id).toBe("bulk-material-1");
    expect(operations.batchReceipts).toHaveLength(1);
    expect(operations.batchReceipts[0].batch.operations.map((operation) => [operation.author_type, operation.source?.source_role]))
      .toEqual(batch.operations.map(() => ["user", "viewport_editor"]));
    expect(operations.appliedOperations).toHaveLength(1);
    expect(operations.retainedReviewContext).toHaveLength(batch.operations.length);
    const committedHash = await hashOf(result.current.model.model!);

    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(await hashOf(result.current.model.model!)).toBe(originalHash);
    expect(result.current.model.model).toEqual(original);
    act(() => result.current.operations.handleRedoSessionModelEdit());
    expect(await hashOf(result.current.model.model!)).toBe(committedHash);
  });
});

describe("commit entry: refusals change nothing", () => {
  it("a batch of each other origin presented to the entry is refused there, with the engine never asked and nothing changed", async () => {
    const result = await session();
    const model = structuredClone(result.current.model.model!);
    const editorBuilt = coordinateIntent(model, 2, "2.5", 1);
    const { author_type: _author, source: _source, ...unstamped } = editorBuilt;
    const bare = { operation_id: "op:agent-1", operation_kind: editorBuilt.operation_kind, target: editorBuilt.target, change: editorBuilt.change, rationale: "agent" };
    const agentBatch = { batch_id: "agent-proposal-1", operations: stampAgentOperations([bare], "proposal-handle", "request-ref") };
    const imported = stampImportedBatch(parseOfflineBatch(JSON.stringify({ batch_id: "imported-1", operations: [{ ...editorBuilt, author_type: "user" }] })));
    const requeuedPerson = requeueRetainedRecord(structuredClone(editorBuilt), 0);
    const requeuedAgent = requeueRetainedRecord(structuredClone(agentBatch.operations[0]), 1);
    if (!("batch" in requeuedPerson) || !("batch" in requeuedAgent)) throw new Error("requeue refused the invented records");
    expect(requeuedPerson.batch.operations[0].author_type).toBe("user");
    const route = gui(result);
    const origins: Array<[string, unknown, unknown]> = [
      ["the embedded agent's batch", route, { batch: agentBatch }],
      ["the embedded agent's batch with its context", route, { batch: agentBatch, agentContext: { assumptions: [] } }],
      ["the embedded agent's single operation", route, { intent: agentBatch.operations[0] }],
      ["offline intake's batch", route, { batch: imported.batch }],
      ["an import with its context", route, { batch: imported.batch, importContext: imported.context }],
      ["an import whose operation claims a person", route, { intent: { ...editorBuilt, source: { source_ref: "file", source_channel: "offline_intake", source_role: "imported" } } }],
      ["a requeued retained record that claims a person", route, { batch: requeuedPerson.batch }],
      ["a requeued retained agent record", route, { batch: requeuedAgent.batch }],
      ["a retained record presented as a single operation", route, { intent: { ...editorBuilt, source: { source_ref: "retained:0", source_channel: "retained_requeue", source_role: "agent" } } }],
      ["one with no stamp, without an editor's route", undefined, { intent: unstamped }],
      ["one with no stamp, with a copy of an editor's route", { role: "gui_editor" }, { intent: unstamped }],
      ["one whose payload claims a person, without an editor's route", null, { intent: editorBuilt }],
      ["one whose payload claims a person, with a clone of an editor's route", structuredClone(route), { batch: { batch_id: "claims-person", operations: [editorBuilt] } }],
      ["a batch with a member no editor writes", route, { batch: { batch_id: "extra", operations: [editorBuilt], precondition: {} } }],
      ["an empty batch", route, { batch: { batch_id: "empty", operations: [] } }]
    ];
    const validate = vi.spyOn(operationBatchService, "validateOperationBatch");
    const apply = vi.spyOn(operationBatchService, "applyOperationBatch");
    const before = snapshot(result);
    for (const [name, presentedRoute, edit] of origins) {
      const outcome = await commit(result, presentedRoute, edit);
      expect(outcome.status, name).toBe("refused");
      if (outcome.status === "refused") expect(outcome.reason, name).toMatch(/nothing was applied/i);
      expect(snapshot(result), name).toEqual(before);
      const previewed = await preview(result, presentedRoute, edit);
      expect(previewed.status, name).toBe("refused");
    }
    expect(validate).not.toHaveBeenCalled();
    expect(apply).not.toHaveBeenCalled();
    // The same operation committed from an editor's route applies.
    committed(await commit(result, route, { intent: editorBuilt }));
    expect(apply).toHaveBeenCalledTimes(1);
  });

  it("a record already committed, presented again, is refused as a retained record", async () => {
    const result = await session();
    committed(await commit(result, gui(result), { intent: coordinateIntent(result.current.model.model!, 2, "2.5", 1) }));
    const retained = result.current.operations.retainedReviewContext[0];
    const before = snapshot(result);
    const outcome = await commit(result, gui(result), { intent: retained });
    expect(outcome).toMatchObject({ status: "refused", reason: expect.stringContaining("already committed once") });
    expect(snapshot(result)).toEqual(before);
  });

  it("the engine's refusal returns its reason, and the model, the history and the receipts stay as they were", async () => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    // A first commit, so there is history and receipts to keep.
    committed(await commit(result, gui(result), { intent: coordinateIntent(original, 1, "0.5", 1) }));
    const before = snapshot(result);
    const apply = vi.spyOn(operationBatchService, "applyOperationBatch");
    // The same field committed again with the `before` it had: the engine refuses the stale basis.
    const stale = coordinateIntent(original, 1, "0.75", 2);
    const outcome = await commit(result, gui(result), { intent: stale });
    if (outcome.status !== "refused") throw new Error("expected a refusal");
    expect(outcome.outcome).not.toBeNull();
    expect(outcome.outcome!.mode).toBe("validate_only");
    const findings = [...outcome.outcome!.diagnostics, ...outcome.outcome!.operation_outcomes.flatMap((step) => step.diagnostics)].filter((item) => item.severity === "blocking");
    expect(findings.length).toBeGreaterThan(0);
    for (const finding of findings) expect(outcome.reason).toContain(finding.message);
    expect(snapshot(result)).toEqual(before);
    expect(apply).not.toHaveBeenCalled();
  });
});

describe("commit entry: a held result", () => {
  it("a commit with a result held clears it and says so in its return; a commit with none held says none was cleared", async () => {
    const result = await session();
    await act(async () => { await result.current.results.handleInspectBundledReference(); });
    await waitFor(() => expect(result.current.results.historicalRun).not.toBeNull());
    const outcome = committed(await commit(result, gui(result), { intent: coordinateIntent(result.current.model.model!, 2, "2.5", 1) }));
    expect(outcome.resultsCleared).toBe(true);
    const results = result.current.results;
    expect([results.historicalRun, results.result, results.analysisRun, results.inputManifest, results.ruleCheckAggregate]).toEqual([null, null, null, null, null]);
    const next = committed(await commit(result, gui(result), { intent: coordinateIntent(result.current.model.model!, 1, "0.5", 2) }));
    expect(next.resultsCleared).toBe(false);
  });
});

describe("commit entry: the preview a bend or a reconnect requires", () => {
  it("a reconnect commits only with a preview of that exact batch on the current model", async () => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    const originalHash = await hashOf(original);
    const batch = await buildReconnectBatch(original, reconnectDraft(), "reconnect-1");
    const other = await buildReconnectBatch(original, reconnectDraft({ targetNode: "node:D" }), "reconnect-1");
    const before = snapshot(result);

    expect(await commit(result, gui(result), { batch })).toMatchObject({ status: "refused", reason: expect.stringContaining("reconnect_pipe_run") });
    // A forged handle, and a preview of another batch under the same identifier, do not unlock it.
    expect(await commit(result, gui(result), { batch }, { preview: { batchId: "reconnect-1" } })).toMatchObject({ status: "refused" });
    const otherPreview = await preview(result, gui(result), { batch: other });
    if (otherPreview.status !== "previewed") throw new Error(otherPreview.reason);
    expect(await commit(result, gui(result), { batch }, { preview: otherPreview.preview })).toMatchObject({ status: "refused", reason: expect.stringContaining("another change or an earlier model") });
    expect(snapshot(result)).toEqual(before);

    // A preview of this batch, then the model changes: the preview is of an earlier model.
    const early = await preview(result, gui(result), { batch });
    if (early.status !== "previewed") throw new Error(early.reason);
    expect(early.outcome.mode).toBe("validate_only");
    expect(result.current.model.model).toEqual(original);
    committed(await commit(result, gui(result), { intent: coordinateIntent(original, 3, "0.5", 1) }));
    const moved = snapshot(result);
    expect(await commit(result, gui(result), { batch }, { preview: early.preview })).toMatchObject({ status: "refused" });
    expect(snapshot(result)).toEqual(moved);

    // A preview of this batch on the current model unlocks it: one checkpoint, one Undo.
    const movedHash = await hashOf(result.current.model.model!);
    const current = await preview(result, gui(result), { batch });
    if (current.status !== "previewed") throw new Error(current.reason);
    committed(await commit(result, gui(result), { batch }, { preview: current.preview }));
    expect(result.current.model.model!.pipe_segments.find((pipe) => pipe.id === "pipe:P")!.to).toBe("node:C");
    expect(result.current.operations.undoStack).toHaveLength(2);
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(await hashOf(result.current.model.model!)).toBe(movedHash);
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(await hashOf(result.current.model.model!)).toBe(originalHash);
  });
});

describe("DEL-16-06: the commit entry and the command triggers are out of reach of the agent's host and offline intake", () => {
  const SOURCE = resolve(__dirname, "../..");
  const sources = (dir: string): string[] => readdirSync(dir).flatMap((name) => {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) return name === "test" || name === "test-support" ? [] : sources(path);
    return /\.(ts|tsx)$/.test(name) && !/\.test\.tsx?$/.test(name) ? [path] : [];
  });
  const files = sources(SOURCE).map((path) => ({ file: relative(SOURCE, path).split("\\").join("/"), text: readFileSync(path, "utf8") }));
  const ENTRY = /\b(commitEditorEdit|previewEditorEdit|editorRoutes|stampEditorCommit)\b/g;
  const TRIGGERS_AND_APPLY = /\b(commitEditorEdit|previewEditorEdit|editorRoutes|stampEditorCommit|runMenuCommand|runMenuCommandRef|handleApplyIntent|handleApplyNextQueuedIntent|handleRunOperationBatch|handleApplyDraftReview|applyOperationBatch|applyModelOperation)\b/;

  it("the agent's host is constructed with its five functions only", async () => {
    const result = await session();
    const owner = (result.current.agent as unknown as { owner: Record<string, unknown> }).owner;
    expect(Object.keys(owner).sort()).toEqual(["proposal", "register", "snapshot", "solve", "withdraw"]);
    const held = Object.values(owner);
    const operations = result.current.operations;
    for (const entry of [operations.commitEditorEdit, operations.previewEditorEdit, operations.editorRoutes.gui, operations.editorRoutes.viewport,
      operations.handleApplyIntent, operations.handleRunOperationBatch]) expect(held).not.toContain(entry);
    // The interface the host is written against names the same five members.
    const agent = files.find((entry) => entry.file === "services/embeddedAgentSession.ts")!.text;
    const declared = agent.slice(agent.indexOf("export interface WorkspaceExecution {"), agent.indexOf("\n}", agent.indexOf("export interface WorkspaceExecution {")));
    expect([...declared.matchAll(/^\s*(\w+)\(/gm)].map((match) => match[1]).sort()).toEqual(["proposal", "register", "snapshot", "solve", "withdraw"]);
  });

  it("no source of the agent, its proposals or offline intake names the commit entry, a command trigger or an apply", () => {
    const reached = files.filter(({ file }) => /^(features\/(embedded-agent|agent-proposals|offline-proposal-intake)\/|services\/embeddedAgent)/.test(file));
    expect(reached.length).toBeGreaterThan(3);
    expect(reached.filter(({ text }) => TRIGGERS_AND_APPLY.test(text)).map(({ file }) => file)).toEqual([]);
  });

  it("offline intake and the agent's panel are rendered with no commit entry, and the entry is handed to nothing yet", () => {
    const app = files.find((entry) => entry.file === "App.tsx")!.text;
    for (const tag of ["<OfflineProposalIntakePanel", "<EmbeddedAgentPanel"]) {
      const start = app.indexOf(tag);
      expect(start, tag).toBeGreaterThan(0);
      const element = app.slice(start, app.indexOf("/>", start));
      expect(element, tag).not.toMatch(TRIGGERS_AND_APPLY);
      expect(element, tag).not.toMatch(/onCommit|onPreviewEdit/);
    }
    // Only the session names the entry; the slice that wires the editors adds them here by name.
    const named: Record<string, number> = {};
    for (const { file, text } of files) { const n = (text.match(ENTRY) ?? []).length; if (n) named[file] = n; }
    expect(Object.keys(named).sort()).toEqual(["features/workspace/workspaceSession.ts", "services/operationAuthorship.ts"]);
  });
});

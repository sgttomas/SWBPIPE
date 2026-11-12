// DEL-04-11 REQ-008: the two routes that never register a write of a support's hanger selection, decided by the batch's
// structure (`writesSupportHangerSelection`), never by a label: requeue of a stored record and offline intake (the
// only producer that passes importContext). The host's own features register with or without a precondition, and an
// agent's proposal registers as before. The session is the desktop's own, loaded with the invented two-span sizing
// input's model (with the session's `diagnostics: []`); sized run identities and loads are invented, as in
// `hangerSelection.sized.test.ts`.
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import type { AgentContext } from "../../services/embeddedAgentSession";
import type { ImportContext } from "../../services/operationAuthorship";
import type { OperationBatch } from "../../services/operationBatchService";
import * as operationService from "../../services/operationService";
import * as previewService from "../../services/previewService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { draftIntent } from "../offline-proposal-intake/workflowSupport";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import {
  buildHangerSelectionBatch,
  SIZED_HANGER_SELECTION_FIELD_LABEL,
  SUPPORT_HANGER_WRITE_REFUSED_TEXT,
  type HangerDocument,
  type HangerSizingApplication
} from "./hangerSelection";

const model = () => ({ ...structuredClone(twoSpanInput.request.model), diagnostics: [] }) as unknown as PreviewModel;
const library = () => structuredClone(twoSpanInput.library) as unknown as HangerDocument;
const TARGET = { supportId: "support:B", restraints: ["UZ"] };
const SIZED: HangerSizingApplication = {
  run: { sizingRequestSha256: "1".repeat(64), solveRequestSha256: "2".repeat(64), libraryId: "invented.hangers.sizing", librarySha256: "3".repeat(64), hangerId: "VS-200-L", recordSha256: "4".repeat(64) },
  loads: { family: "variable_spring", coldLoadN: 5197.5, hotLoadN: 5000, designOperatingCase: "load:O1" }
};
const IMPORT: ImportContext = { kind: "offline_intake", fileBatchId: "file:intake", claimed: [] };
const AGENT: AgentContext = {
  requestRef: "request:test", threadId: "thread:test", turnId: "turn:test", callId: "call:test",
  basis: { workspaceHandle: "workspace:test", applicationGeneration: 0, modelRevision: 0, modelHash: null, stateId: null, runId: null }, assumptions: []
};

async function session() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model());
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
type Session = Awaited<ReturnType<typeof session>>;

async function register(result: Session, batch: OperationBatch, options: Parameters<Session["current"]["operations"]["registerOperationBatch"]>[1] = {}) {
  let key: string | null = null;
  await act(async () => { key = await result.current.operations.registerOperationBatch(structuredClone(batch), options); });
  return key as string | null;
}

async function requeue(result: Session, intent: EditorOperationIntent) {
  let refused: string | null = "not run";
  await act(async () => { refused = await result.current.operations.handleRequeueRetainedRecord(structuredClone(intent), 0); });
  return refused as string | null;
}

/** One Support operation, as a person's own stored record. */
function supportIntent(changeKind: "update_support" | "create_support", ref: string, before: string, after: string): EditorOperationIntent {
  return draftIntent({
    operation_kind: changeKind === "create_support" ? "create" : "modify",
    target: { object_type: "Support", ref },
    change: {
      change_id: `change:${ref}`, change_kind: changeKind, field_label: "Support", field_path: changeKind === "create_support" ? "supports" : "configuration",
      before, after, unit: "none", dimension: "dimensionless", source_note: "invented test record"
    },
    rationale: "Invented test record."
  });
}
const batchOf = (...operations: EditorOperationIntent[]): OperationBatch => ({ batch_id: "batch:gate", operations });

/** The record the gate reads, on both refused routes: requeue first, then offline intake. */
async function refusedOnBoth(result: Session, intent: EditorOperationIntent) {
  expect(await requeue(result, intent)).toBe(SUPPORT_HANGER_WRITE_REFUSED_TEXT);
  expect(result.current.operations.batchMessage).toBe(`Record could not be requeued: ${SUPPORT_HANGER_WRITE_REFUSED_TEXT}`);
  expect(await register(result, batchOf(intent), { importContext: IMPORT })).toBeNull();
  expect(result.current.operations.batchMessage).toBe(SUPPORT_HANGER_WRITE_REFUSED_TEXT);
  expect(result.current.operations.queuedBatches).toHaveLength(0);
}

async function admittedOnBoth(result: Session, intent: EditorOperationIntent) {
  expect(await requeue(result, intent)).toBeNull();
  expect(await register(result, batchOf(intent), { importContext: IMPORT })).not.toBeNull();
  expect(result.current.operations.queuedBatches).toHaveLength(2);
}

afterEach(() => { vi.restoreAllMocks(); });

describe("requeue and offline intake of a batch that writes a support's hanger selection", () => {
  it("refuse a sized selection as built, with its label altered, and with its label removed", async () => {
    const result = await session();
    const [built] = (await buildHangerSelectionBatch(result.current.model.model!, library(), "VS-200-L", TARGET, SIZED)).operations;
    expect(built.change.field_label).toBe(SIZED_HANGER_SELECTION_FIELD_LABEL);
    const altered = { ...structuredClone(built), change: { ...built.change, field_label: "Support" } };
    const { field_label: _removed, ...unlabelled } = built.change;
    const removed = { ...structuredClone(built), change: unlabelled } as unknown as EditorOperationIntent;
    for (const intent of [built, altered, removed]) await refusedOnBoth(result, intent);
  });

  it("refuse a hand-picked selection, which the hanger form itself registers", async () => {
    const result = await session();
    const handPicked = await buildHangerSelectionBatch(result.current.model.model!, library(), "VS-200-L", TARGET);
    await refusedOnBoth(result, handPicked.operations[0]);
    await act(async () => { await result.current.operations.handleQueueOperationBatch(structuredClone(handPicked)); });
    expect(result.current.operations.queuedBatches).toHaveLength(1);
  });

  it("admit a support configuration change with no hanger (guide to line_stop)", async () => {
    const result = await session();
    const before = JSON.stringify({ family: "guide", restraints: ["UY"], provenance: "invented_test_value" });
    const after = JSON.stringify({ family: "line_stop", restraints: ["UY"], provenance: "invented_test_value" });
    await admittedOnBoth(result, supportIntent("update_support", "support:C", before, after));
  });

  it("refuse an unparseable after, a created spring_hanger support with no hanger member, and the removal of a hanger", async () => {
    const result = await session();
    const guide = JSON.stringify({ family: "guide", restraints: ["UY"] });
    await refusedOnBoth(result, supportIntent("update_support", "support:C", guide, "{ not json"));
    await refusedOnBoth(result, supportIntent("create_support", "support:H", "not_present",
      JSON.stringify({ id: "support:H", label: "H", node: "node:B", family: "spring_hanger", restraints: ["UZ"], provenance: "invented_test_value" })));
    const hanger = JSON.stringify({ family: "variable_spring_hanger", restraints: ["UZ"], hanger: { hanger_type: "variable_spring_hanger", source_reference: "invented" } });
    await refusedOnBoth(result, supportIntent("update_support", "support:B", hanger, guide));
  });

  it("end in the registration refusal and never reach the lone intent path, which has no refusal of its own", async () => {
    // The lone intent path (handleApplyIntent) applies one queued editor intent; neither refused route reaches it, so
    // the refusal at registration is the only one these routes need.
    const result = await session();
    const apply = vi.spyOn(operationService, "applyModelOperation");
    const validate = vi.spyOn(operationService, "validateModelOperation");
    const [built] = (await buildHangerSelectionBatch(result.current.model.model!, library(), "VS-200-L", TARGET, SIZED)).operations;
    await refusedOnBoth(result, built);
    expect(apply).not.toHaveBeenCalled();
    expect(validate).not.toHaveBeenCalled();
    expect(result.current.operations.editorIntents).toHaveLength(0);
  });
});

describe("the routes that register a hanger write as before", () => {
  it("the sizing acts' batch, with or without a precondition", async () => {
    const result = await session();
    const sized = await buildHangerSelectionBatch(result.current.model.model!, library(), "VS-200-L", TARGET, SIZED);
    expect(await register(result, sized, { precondition: async () => null })).not.toBeNull();
    expect(await register(result, sized)).not.toBeNull();
  });

  it("an agent's proposal creating a variable spring hanger support", async () => {
    const result = await session();
    const create = supportIntent("create_support", "support:H", "not_present",
      JSON.stringify({ id: "support:H", label: "H", node: "node:B", family: "variable_spring_hanger", restraints: ["UZ"], provenance: "agent",
        hanger: { hanger_type: "variable_spring_hanger", source_reference: "agent-proposed" } }));
    expect(await register(result, batchOf({ ...create, author_type: "agent" }), { agentContext: AGENT })).not.toBeNull();
    expect(result.current.operations.queuedBatches[0].agentContext).toEqual(AGENT);
  });
});

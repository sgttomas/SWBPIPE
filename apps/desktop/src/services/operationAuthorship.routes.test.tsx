// The routes to the review queue and the one gate on operations not authored by the person
// (operationAuthorship.ts writesUserEnteredModifier). The intake panel is rendered: a file
// carrying a person-entered member shows the refusal and calls no queue. The source-reading
// test pins, by name, every queue entry and every place that stamps an author other than the
// user in the non-test desktop source, so a new route fails here until it is reviewed. It reads
// names, not behaviour: it cannot see a route that queues by another name, and it does not
// prove what a builder stamps; the reviewer is asked that question as well.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { loadPreviewModel } from "./previewService";
import { applyOperationBatch } from "./operationBatchService";
import { computeModelHash } from "./hashService";
import { requeueRetainedRecord, writesUserEnteredModifier } from "./operationAuthorship";
import { OfflineProposalIntakePanel } from "../features/offline-proposal-intake/OfflineProposalIntakePanel";
import { PropertyInspector } from "../features/model-tree/PropertyInspector";
import { DisplayUnitsProvider } from "../features/display-units";
import type { EditorOperationIntent } from "../types";

afterEach(cleanup);
const SOURCE = resolve(__dirname, "..");

function sources(dir = SOURCE): string[] {
  return readdirSync(dir).flatMap((name) => {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) return name === "test" || name === "test-support" ? [] : sources(path);
    return /\.(ts|tsx)$/.test(name) && !/\.test\.tsx?$/.test(name) ? [path] : [];
  });
}
function count(pattern: RegExp): Record<string, number> {
  const found: Record<string, number> = {};
  for (const file of sources()) {
    const n = (readFileSync(file, "utf8").match(pattern) ?? []).length;
    if (n) found[relative(SOURCE, file)] = n;
  }
  return found;
}

describe("routes to the review queue", () => {
  it("the intake panel shows the refusal and queues nothing", async () => {
    const model = await loadPreviewModel();
    const queue = vi.fn();
    render(<OfflineProposalIntakePanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={queue} />);
    const file = { batch_id: "file", operations: [{ operation_id: "op:m", operation_kind: "create", target: { object_type: "Material", ref: "material:m" },
      change: { change_id: "c", change_kind: "create_material", field_label: "m", field_path: "materials", before: "not_present", after: JSON.stringify({ id: "material:m", modifiers: { sif_user_value: { value: 9.75, unit: "none" } } }), unit: "Pa", dimension: "stress", source_note: "file" },
      rationale: "file" }] };
    fireEvent.change(screen.getByLabelText("Proposal batch JSON"), { target: { value: JSON.stringify(file) } });
    fireEvent.click(screen.getByRole("button", { name: "Queue imported batch for review" }));
    expect(screen.getByRole("alert").textContent).toContain("Operation 1 (op:m) of this file has a value that names modifiers");
    expect(screen.getByRole("alert").textContent).not.toContain("9.75");
    expect(queue).not.toHaveBeenCalled();
  });

  it("a person typing a text that reads as JSON into a label is not refused: the inspector's route calls none of the three gates", async () => {
    // Trigger: the person types into the inspector's value field and queues it. Route: the
    // inspector builds the intent with author_type "user" (PropertyInspector.tsx, the field's
    // literal text as after) and hands it to onQueueIntent, which the workspace binds to
    // handleQueueEditorIntent (workspaceSession.ts): it adds the intent to the person's queue and
    // calls no gate. The three gates are the agent's executor, offline intake and requeue only.
    const model = await loadPreviewModel();
    const node = model.nodes[0];
    const queue = vi.fn<(intent: EditorOperationIntent) => void>();
    render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector model={model} selection={{ type: "node", id: node.id }} onQueueIntent={queue} /></DisplayUnitsProvider>);
    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    fireEvent.click(screen.getByTestId("inspector-start-task"));
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value: '{"modifiers":{}}' } });
    fireEvent.click(screen.getByTestId("queue-editor-intent"));
    expect(queue).toHaveBeenCalledTimes(1);
    const intent = queue.mock.calls[0][0];
    expect(intent).toMatchObject({ author_type: "user", change: { change_kind: "set_field", field_path: "label", after: '{"modifiers":{}}' } });
    // The same text would be named by the gate, had the route called it.
    expect(writesUserEnteredModifier(intent)).toMatchObject({ member: "modifiers" });
    const session = readFileSync(join(SOURCE, "features/workspace/workspaceSession.ts"), "utf8");
    const handler = session.slice(session.indexOf("function handleQueueEditorIntent("), session.indexOf("async function handleReviewQueuedIntentsAsBatch("));
    expect(handler.length).toBeGreaterThan(0);
    expect(handler).not.toMatch(/writesUserEnteredModifier|personEnteredWriteIn|stampImportedBatch|requeueRetainedRecord/);
    // The engine writes it as the label's literal text when the person applies it.
    const outcome = await applyOperationBatch(model, { batch_id: "person-label", operations: [intent] }, await computeModelHash(model));
    expect(outcome.applied_model?.nodes.find(item => item.id === node.id)?.label).toBe('{"modifiers":{}}');
    // Stored and requeued after reopening, its author claim cannot be verified, so requeue refuses it.
    expect(requeueRetainedRecord(intent, 0)).toEqual({ refused: expect.stringMatching(/^This stored record has a value that names modifiers/) });
  }, 60_000);

  it("every queue entry and every non-user author stamp is on the reviewed list", () => {
    // The three places that stamp an author other than the user, each gated by writesUserEnteredModifier:
    // stampImportedBatch and requeueRetainedRecord (operationAuthorship.ts), stampAgentOperations
    // (embeddedAgentSession.ts; its two other matches are the tool results' author_type field).
    expect(count(/author_type\s*:\s*['"](agent|import_adapter|project_template)['"]/g)).toEqual({ "services/embeddedAgentSession.ts": 3, "services/operationAuthorship.ts": 2 });
    const authorship = readFileSync(join(SOURCE, "services/operationAuthorship.ts"), "utf8");
    const stamp = authorship.slice(authorship.indexOf("export function stampImportedBatch"));
    expect(stamp.indexOf("personEnteredWriteIn(batch.operations)")).toBeGreaterThan(0);
    expect(stamp.indexOf("personEnteredWriteIn(batch.operations)")).toBeLessThan(stamp.indexOf('author_type: "agent"'));
    const requeue = authorship.slice(authorship.indexOf("export function requeueRetainedRecord"));
    expect(requeue.indexOf("writesUserEnteredModifier(intent,")).toBeGreaterThan(0);
    expect(requeue.indexOf("writesUserEnteredModifier(intent,")).toBeLessThan(requeue.indexOf('author_type: "agent"'));
    const session = readFileSync(join(SOURCE, "services/embeddedAgentSession.ts"), "utf8");
    const propose = session.slice(session.indexOf("if(call.tool==='swbpipe_propose_operations'){"));
    expect(propose.indexOf("personEnteredWriteIn(")).toBeGreaterThan(0);
    expect(propose.indexOf("personEnteredWriteIn(")).toBeLessThan(propose.indexOf("stampAgentOperations("));
    // Every name through which a batch reaches the queue. Each other caller passes batches the
    // person's own builders stamp `user` (workflowSupport.ts draftIntent, formSupport.tsx and the
    // feature builders), so it needs no gate.
    expect(count(/\bregisterOperationBatch\b/g)).toEqual({ "App.tsx": 2, "features/workspace/workspaceSession.ts": 6 });
    expect(count(/\bhandleQueueOperationBatch\b/g)).toEqual({ "App.tsx": 9, "features/workspace/workspaceSession.ts": 3 });
    expect(count(/\bregisterBatch\b/g)).toEqual({ "App.tsx": 1, "features/model-tree/runSheet/RunSheet.tsx": 2 });
    // The queue's own state is set only inside the workspace session folder.
    expect(Object.keys(count(/\bsetQueuedBatches\b/g)).filter((file) => !file.startsWith("features/workspace/"))).toEqual([]);
    expect(count(/\bsetQueuedBatches\b/g)["features/workspace/workspaceSession.ts"]).toBe(11);
    expect(count(/\bonQueueBatch\b/g)).toEqual({
      "App.tsx": 8, "features/bulk-change/BulkChangePanel.tsx": 3, "features/geometry-tools/SplitIntoPartsPanel.tsx": 3,
      "features/geometry-tools/batchFormSupport.tsx": 2, "features/hanger-selection/HangerSelectionPanel.tsx": 1,
      "features/library/LibraryApplySection.tsx": 2, "features/library/LibraryManagerPanel.tsx": 5,
      "features/offline-proposal-intake/OfflineProposalIntakePanel.tsx": 1, "features/offline-proposal-intake/workflowSupport.ts": 1,
      "features/self-weight-authoring/SelfWeightPlanPanel.tsx": 1,
    });
  });
});

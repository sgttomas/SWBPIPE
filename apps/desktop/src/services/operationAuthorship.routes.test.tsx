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
import { OfflineProposalIntakePanel } from "../features/offline-proposal-intake/OfflineProposalIntakePanel";

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
    expect(screen.getByRole("alert").textContent).toContain("Operation 1 (op:m) of this file carries modifiers");
    expect(screen.getByRole("alert").textContent).not.toContain("9.75");
    expect(queue).not.toHaveBeenCalled();
  });

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
    expect(requeue.indexOf("writesUserEnteredModifier(intent)")).toBeGreaterThan(0);
    expect(requeue.indexOf("writesUserEnteredModifier(intent)")).toBeLessThan(requeue.indexOf('author_type: "agent"'));
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

// Model document 0.9.0 S3b: the condition-set operations through the workspace session's
// one operation route (queue, apply, Undo through the session checkpoint), bulk change's
// "Assign condition set" as one batch, and a saved and reopened model edited again. The
// session runs the actual wasm engine on S3a's committed corpus document.
import { act, cleanup, fireEvent, render, renderHook, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { applyModelOperation } from "../../services/operationService";
import { findLocalProject, openLocalProject, saveLocalProject } from "../../services/projectService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { buildBulkChangeBatch, planBulkChange } from "../bulk-change/bulkChange";
import { corpusBase, sessionCorpusBase } from "../../test-support/conditionSetCorpus";
import { ConditionSetsSection } from "./ConditionSetsSection";
import { ConditionUseControl } from "./ConditionUseControl";
import { PipeConditionSetField } from "./PipeConditionSetField";
import { conditionUseIntent, createConditionSetIntent } from "./conditionSetIntents";

async function session(model: PreviewModel) {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
afterEach(() => { cleanup(); vi.restoreAllMocks(); });

const SET = {
  id: "cs:line-a", label: "Line A", provenance: "invented S3b session input",
  installation_temperature: { value: 20, unit: "degC" },
  conditions: [{ key: "OPE1", temperature: { value: 150, unit: "degC" } }],
  added_weight: { magnitude: { value: -50, unit: "N/m" }, direction: "global_z" as const }
};

describe("condition sets through the session (queue, apply, Undo)", () => {
  it("creates a set, assigns it to every pipe as one bulk batch and sets a case's use; Undo restores each prior document exactly", async () => {
    const result = await session(sessionCorpusBase("s3a_01"));
    const hashes = [(await computeModelHash(result.current.model.model!))!.value];
    const ops = () => result.current.operations;

    // 1. Create through the single-intent route.
    act(() => ops().handleQueueEditorIntent(createConditionSetIntent(SET)));
    let applied = false;
    await act(async () => { applied = await ops().handleApplyIntent(ops().editorIntents[0]); });
    expect(applied).toBe(true);
    expect(result.current.model.model!.condition_sets).toEqual([SET]);
    hashes.push((await computeModelHash(result.current.model.model!))!.value);

    // 2. Bulk change: "Assign condition set" over the three pipes, one batch.
    const model = result.current.model.model!;
    const scope = model.pipe_segments.map((pipe) => ({ type: "pipe" as const, id: pipe.id }));
    const plan = planBulkChange(model, scope, { kind: "condition-set", setId: SET.id });
    expect(plan.affected.map((entry) => [entry.ref.id, entry.before, entry.after])).toEqual([["P1", "not stated", SET.id], ["P2", "not stated", SET.id], ["P3", "not stated", SET.id]]);
    const batch = await buildBulkChangeBatch(model, scope, { kind: "condition-set", setId: SET.id }, canonicalJsonString, "bulk-condition-set-1");
    expect(batch.operations.map((operation) => [operation.change.change_kind, operation.change.before, operation.change.after])).toEqual(
      ["P1", "P2", "P3"].map(() => ["assign_condition_set", "not_present", JSON.stringify({ condition_set_ref: SET.id })]));
    await act(async () => { await ops().handleQueueOperationBatch(batch); });
    await act(async () => { await ops().handleRunOperationBatch(ops().queuedBatches[0], true); });
    expect(result.current.model.model!.pipe_segments.map((pipe) => pipe.condition_set_ref)).toEqual([SET.id, SET.id, SET.id]);
    hashes.push((await computeModelHash(result.current.model.model!))!.value);

    // 3. The case's use, through the single-intent route.
    const sus = result.current.model.model!.load_cases.find((item) => item.id === "SUS")!;
    const use = await conditionUseIntent(canonicalJsonString, sus, { temperature: "OPE1", added_weight: true });
    act(() => ops().handleQueueEditorIntent(use));
    await act(async () => { applied = await ops().handleApplyIntent(ops().editorIntents[ops().editorIntents.length - 1]); });
    expect(applied).toBe(true);
    expect(result.current.model.model!.load_cases.find((item) => item.id === "SUS")?.condition_use).toEqual({ temperature: "OPE1", added_weight: true });
    expect(ops().undoStack).toHaveLength(3);

    // Undo each checkpoint: the session restores each prior document exactly.
    for (const expected of [hashes[2], hashes[1], hashes[0]]) {
      act(() => ops().handleUndoSessionModelEdit());
      expect((await computeModelHash(result.current.model.model!))!.value).toBe(expected);
    }
    expect(result.current.model.model!.condition_sets).toBeUndefined();
  });
});

describe("a saved and reopened model", () => {
  it("reopens with its sets, assignments and use shown, applies further edits, and saves and reopens them as applied", async () => {
    const edited = corpusBase("s3a_09");
    edited.project.id = "project:s3b-condition-sets-reopen";
    edited.load_cases.find((item) => item.id === "SUS")!.condition_use = { temperature: "OPE1" };
    const saved = await saveLocalProject(edited);
    expect(saved.model).toEqual(edited);
    const opened = (await openLocalProject("project:s3b-condition-sets-reopen"))!.model as PreviewModel;
    expect(await canonicalJsonString(opened)).toBe(await canonicalJsonString(edited));

    const list: EditorOperationIntent[] = [];
    render(<>
      <ConditionSetsSection model={opened} onQueueIntent={(intent) => list.push(intent)} />
      <PipeConditionSetField model={opened} selection={{ type: "pipe", id: "P2" }} onQueueIntent={(intent) => list.push(intent)} />
      <ConditionUseControl model={opened} loadCase={opened.load_cases.find((item) => item.id === "SUS")!} onQueueIntent={(intent) => list.push(intent)} />
    </>);
    expect(screen.getByTestId("condition-set-summary-cs:a")).toHaveTextContent("carried by P1, P2, P3");
    expect(screen.getByTestId("pipe-condition-set-current")).toHaveTextContent("Carries condition set cs:a.");
    expect(screen.getByTestId("load-condition-use-current")).toHaveTextContent("Stored use: temperature OPE1.");
    expect(screen.getByTestId("load-condition-use-temperature")).toHaveValue("OPE1");
    fireEvent.click(screen.getByTestId("load-condition-use-added-weight"));
    fireEvent.click(screen.getByTestId("load-condition-use-queue"));
    await waitFor(() => expect(list).toHaveLength(1));
    expect(list[0].change).toMatchObject({ change_kind: "set_load_case_condition_use", before: await canonicalJsonString({ temperature: "OPE1" }), after: JSON.stringify({ temperature: "OPE1", added_weight: true }) });

    // Apply the reopened edit, then edit one member of the reopened set and apply that too.
    const apply = async (model: PreviewModel, intent: EditorOperationIntent) => {
      const outcome = await applyModelOperation(model, intent, await computeModelHash(model));
      expect(outcome.diagnostics.filter((item) => item.severity === "blocking")).toEqual([]);
      expect(outcome.validation.application_status).toBe("applied_to_session_model");
      return outcome.applied_model as PreviewModel;
    };
    let current = await apply(opened, list[0]);
    cleanup();
    render(<ConditionSetsSection model={current} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.change(screen.getByTestId("condition-set-cs:a-label"), { target: { value: "Line A (reopened)" } });
    fireEvent.click(screen.getByTestId("condition-set-update-cs:a"));
    await waitFor(() => expect(list).toHaveLength(2));
    current = await apply(current, list[1]);

    // Save again (the replacement the person confirms, as the session's save does) and reopen
    // once more: the stored records are exactly the applied ones.
    const stored = await findLocalProject("project:s3b-condition-sets-reopen");
    await saveLocalProject(current, [], null, null, null, null, null, null, stored!.revision);
    const reopened = (await openLocalProject("project:s3b-condition-sets-reopen"))!.model as PreviewModel;
    expect(await canonicalJsonString(reopened)).toBe(await canonicalJsonString(current));
    expect(reopened.load_cases.find((item) => item.id === "SUS")?.condition_use).toEqual({ temperature: "OPE1", added_weight: true });
    expect(reopened.condition_sets).toEqual([{ ...edited.condition_sets![0], label: "Line A (reopened)" }]);
    expect(reopened.pipe_segments.map((pipe) => pipe.condition_set_ref)).toEqual(["cs:a", "cs:a", "cs:a"]);
  });
});

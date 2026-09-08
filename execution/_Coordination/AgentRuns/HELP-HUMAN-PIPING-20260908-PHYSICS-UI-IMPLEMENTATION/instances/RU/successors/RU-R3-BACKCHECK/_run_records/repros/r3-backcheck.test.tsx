import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import React from "react";
import { describe, expect, it, vi } from "vitest";

import { PipeViewport } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport";
import { applyResultMatchesSubmission, buildRouteSubmission, type DraftSubmission, type FrozenDraftReview, type RouteDraft } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft";
import { computeModelHash } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/services/hashService";
import { applyOperationBatch } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/services/operationBatchService";
import { validateModelOperation } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/services/operationService";
import { loadPreviewModel } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/services/previewService";
import type { PreviewModel } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/types";

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((onResolve) => { resolve = onResolve; });
  return { promise, resolve };
}

function batchDraft(model: PreviewModel): RouteDraft {
  return {
    startNodeId: model.nodes[0].id,
    endMode: "new",
    existingEndNodeId: "",
    newEnd: { id: "node:RU-R3", label: "RU R3 end", x: "3.2", y: "0", z: "0", coordinateUnit: "m", provenance: "explicit_r3_endpoint" },
    pipe: { id: "pipe:RU-R3", label: "RU R3 pipe", materialId: model.materials![0].id, outsideDiameter: "0.168", wallThickness: "0.007", lengthUnit: "m", yReferenceX: "0", yReferenceY: "0", yReferenceZ: "1", provenance: "explicit_r3_pipe" }
  };
}

function fillExistingRoute(model: PreviewModel) {
  const panel = screen.getByTestId("viewport-editor-intents");
  const change = (id: string, value: string) => fireEvent.change(within(panel).getByTestId(id), { target: { value } });
  change("viewport-create-pipe-id", "pipe:RU-R3-own");
  change("viewport-create-pipe-label", "RU R3 own route");
  change("viewport-create-pipe-from", model.nodes[0].id);
  change("viewport-create-pipe-to", model.nodes[1].id);
  change("viewport-create-pipe-material", model.materials![0].id);
  change("viewport-create-pipe-od", "0.168");
  change("viewport-create-pipe-wall", "0.007");
  change("viewport-create-pipe-yref-x", "0");
  change("viewport-create-pipe-yref-y", "0");
  change("viewport-create-pipe-yref-z", "1");
  change("viewport-create-pipe-provenance", "explicit_r3_own_provenance");
  return panel;
}

async function armedViewport(model: PreviewModel) {
  const apply = deferred<boolean>();
  let submission: DraftSubmission | null = null;
  const reviewId = "review:RU-R3-owned";
  const onAddDraft = async (candidate: DraftSubmission, generation: number): Promise<FrozenDraftReview | null> => {
    if (candidate.kind !== "single") throw new Error("expected single route");
    submission = structuredClone(candidate);
    const basisHash = await computeModelHash(model);
    if (!basisHash) throw new Error("expected hash");
    const outcome = await validateModelOperation(model, candidate.intent, basisHash);
    return { reviewId, generation, basisRevision: 1, basisEpoch: 1, basisHash, submission: candidate, outcome };
  };
  const props = {
    selection: { type: "node" as const, id: model.nodes[0].id },
    armedCreationTool: "pipe" as const,
    onArmCreationTool: vi.fn(),
    onSelect: vi.fn(),
    onInvalidateDraft: vi.fn(),
    onAddDraft,
    onApplyDraft: () => apply.promise
  };
  const view = render(<PipeViewport model={model} modelCommitToken={null} {...props} />);
  const panel = fillExistingRoute(model);
  fireEvent.click(within(panel).getByTestId("continue-pipe-after-queue"));
  fireEvent.click(within(panel).getByTestId("queue-explicit-pipe-intent"));
  await waitFor(() => expect(within(panel).getByTestId("apply-reviewed-draft")).toBeEnabled());
  fireEvent.click(within(panel).getByTestId("apply-reviewed-draft"));
  await waitFor(() => expect(within(panel).getByTestId("viewport-draft-flight-controls")).toBeDisabled());
  if (!submission || submission.kind !== "single") throw new Error("missing frozen route");
  const committed = structuredClone(model);
  committed.pipe_segments.push(JSON.parse(submission.intent.change.after));
  return { apply, committed, panel, props, reviewId, view };
}

describe("RU R3 affirmative residual backcheck", () => {
  it("rejects producer-derived batch outcomes with changed payload refs or malformed diagnostics", async () => {
    const model = await loadPreviewModel();
    const basisHash = await computeModelHash(model);
    const built = buildRouteSubmission(model, [], batchDraft(model), 1, "length=passed");
    if (!basisHash || !built.ok || built.submission.kind !== "batch") throw new Error("expected batch");
    const valid = await applyOperationBatch(model, built.submission.batch, basisHash);
    expect(valid.initial_model_hash.payload_ref).toBe("model:local_batch_input");
    expect(valid.submitted_initial_model_hash).toEqual(basisHash);
    expect(applyResultMatchesSubmission(built.submission, valid, basisHash)).toBe(true);
    const wrongEngine = structuredClone(valid);
    wrongEngine.initial_model_hash.payload_ref = "model:unrelated-engine-payload";
    const wrongEcho = structuredClone(valid);
    wrongEcho.submitted_initial_model_hash!.payload_ref = "project:unrelated-frozen-claim";
    const malformedInfo = structuredClone(valid);
    malformedInfo.diagnostics = [{ severity: "info" } as never];
    expect({
      wrongEngine: applyResultMatchesSubmission(built.submission, wrongEngine, basisHash),
      wrongEcho: applyResultMatchesSubmission(built.submission, wrongEcho, basisHash),
      malformedInfo: applyResultMatchesSubmission(built.submission, malformedInfo, basisHash)
    }).toEqual({ wrongEngine: false, wrongEcho: false, malformedInfo: false });
  });

  it("preserves continuation only when the model carries the accepted review token", async () => {
    const model = await loadPreviewModel();
    const { apply, committed, panel, props, reviewId, view } = await armedViewport(model);
    view.rerender(<PipeViewport model={committed} modelCommitToken={reviewId} {...props} />);
    expect(within(panel).getByTestId("continue-pipe-after-queue")).toBeChecked();
    expect(within(panel).getByTestId("viewport-create-pipe-from")).toHaveValue(model.nodes[1].id);
    await act(async () => { apply.resolve(true); await apply.promise; });
  });

  it("clears continuation for a coincident external replacement without the accepted token", async () => {
    const model = await loadPreviewModel();
    const { apply, committed, panel, props, view } = await armedViewport(model);
    committed.project.id = "project:external-coincident-r3";
    view.rerender(<PipeViewport model={committed} modelCommitToken={null} {...props} />);
    expect(within(panel).getByTestId("continue-pipe-after-queue")).not.toBeChecked();
    expect(within(panel).getByTestId("viewport-create-pipe-from")).toHaveValue("");
    await act(async () => { apply.resolve(false); await apply.promise; });
  });
});

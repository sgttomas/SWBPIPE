import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { PipeViewport } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport";
import { applyResultMatchesSubmission, buildRouteSubmission, type DraftSubmission, type RouteDraft, type FrozenDraftReview } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft";
import { computeModelHash } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/services/hashService";
import { applyOperationBatch } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/services/operationBatchService";
import { validateModelOperation } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/services/operationService";
import { loadPreviewModel } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/services/previewService";
import type { PreviewModel } from "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src/types";

afterEach(() => vi.restoreAllMocks());

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((onResolve) => { resolve = onResolve; });
  return { promise, resolve };
}

function routeDraft(model: PreviewModel): RouteDraft {
  return {
    startNodeId: model.nodes[0].id,
    endMode: "new",
    existingEndNodeId: "",
    newEnd: {
      id: "node:RU-successor-end",
      label: "RU successor end",
      x: "3.2",
      y: "0",
      z: "0",
      coordinateUnit: "m",
      provenance: "explicit_successor_endpoint_provenance"
    },
    pipe: {
      id: "pipe:RU-successor-route",
      label: "RU successor route",
      materialId: model.materials![0].id,
      outsideDiameter: "0.168",
      wallThickness: "0.007",
      lengthUnit: "m",
      yReferenceX: "0",
      yReferenceY: "0",
      yReferenceZ: "1",
      provenance: "explicit_successor_pipe_provenance"
    }
  };
}

function fillExistingRoute(model: PreviewModel) {
  const panel = screen.getByTestId("viewport-editor-intents");
  const change = (id: string, value: string) =>
    fireEvent.change(within(panel).getByTestId(id), { target: { value } });
  change("viewport-create-pipe-id", "pipe:RU-external-match");
  change("viewport-create-pipe-label", "RU external match");
  change("viewport-create-pipe-from", model.nodes[0].id);
  change("viewport-create-pipe-to", model.nodes[1].id);
  change("viewport-create-pipe-material", model.materials![0].id);
  change("viewport-create-pipe-od", "0.168");
  change("viewport-create-pipe-wall", "0.007");
  change("viewport-create-pipe-yref-x", "0");
  change("viewport-create-pipe-yref-y", "0");
  change("viewport-create-pipe-yref-z", "1");
  change("viewport-create-pipe-provenance", "explicit_external_match_provenance");
  return panel;
}

describe("RU successor affirmative adversarial backcheck", () => {
  it("rejects changed hash payload references according to their distinct producer contracts", async () => {
    const model = await loadPreviewModel();
    const basisHash = await computeModelHash(model);
    const built = buildRouteSubmission(model, [], routeDraft(model), 1, "length=passed");
    if (!basisHash || !built.ok || built.submission.kind !== "batch") throw new Error("expected batch");
    const valid = await applyOperationBatch(model, built.submission.batch, basisHash);
    expect(valid.initial_model_hash.payload_ref).toBe("model:local_batch_input");
    expect(valid.submitted_initial_model_hash).toEqual(basisHash);
    expect(applyResultMatchesSubmission(built.submission, valid, basisHash)).toBe(true);

    const wrongEngineRef = structuredClone(valid);
    wrongEngineRef.initial_model_hash.payload_ref = "model:unrelated-engine-payload";
    const wrongEchoedRef = structuredClone(valid);
    wrongEchoedRef.submitted_initial_model_hash!.payload_ref = "project:unrelated-frozen-claim";
    expect({
      engineGeneratedRefAccepted: applyResultMatchesSubmission(built.submission, wrongEngineRef, basisHash),
      echoedFrozenRefAccepted: applyResultMatchesSubmission(built.submission, wrongEchoedRef, basisHash)
    }).toEqual({ engineGeneratedRefAccepted: false, echoedFrozenRefAccepted: false });
  });

  it("rejects a malformed non-warning diagnostic instead of treating it as a clean receipt", async () => {
    const model = await loadPreviewModel();
    const basisHash = await computeModelHash(model);
    const built = buildRouteSubmission(model, [], routeDraft(model), 1, "length=passed");
    if (!basisHash || !built.ok || built.submission.kind !== "batch") throw new Error("expected batch");
    const malformed = await applyOperationBatch(model, built.submission.batch, basisHash);
    malformed.diagnostics = [{ severity: "info" } as never];
    expect(applyResultMatchesSubmission(built.submission, malformed, basisHash)).toBe(false);
  });

  it("clears continuation when an external replacement happens to contain the same route identity", async () => {
    const model = await loadPreviewModel();
    const apply = deferred<boolean>();
    let frozenSubmission: DraftSubmission | null = null;
    const onAddDraft = async (submission: DraftSubmission, generation: number): Promise<FrozenDraftReview | null> => {
      if (submission.kind !== "single") throw new Error("expected single route");
      frozenSubmission = structuredClone(submission);
      const basisHash = await computeModelHash(model);
      if (!basisHash) throw new Error("expected hash");
      const outcome = await validateModelOperation(model, submission.intent, basisHash);
      return { reviewId: "review:external-match", generation, basisRevision: 1, basisEpoch: 1, basisHash, submission, outcome };
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
    const view = render(<PipeViewport model={model} {...props} />);
    const panel = fillExistingRoute(model);
    fireEvent.click(within(panel).getByTestId("continue-pipe-after-queue"));
    fireEvent.click(within(panel).getByTestId("queue-explicit-pipe-intent"));
    await waitFor(() => expect(within(panel).getByTestId("apply-reviewed-draft")).toBeEnabled());
    fireEvent.click(within(panel).getByTestId("apply-reviewed-draft"));
    await waitFor(() => expect(within(panel).getByTestId("viewport-draft-flight-controls")).toBeDisabled());
    if (!frozenSubmission || frozenSubmission.kind !== "single") throw new Error("missing frozen route");

    const replacement = structuredClone(model);
    replacement.project.id = "project:external-replacement";
    replacement.pipe_segments.push(JSON.parse(frozenSubmission.intent.change.after));
    view.rerender(<PipeViewport model={replacement} {...props} />);

    expect(within(panel).getByTestId("continue-pipe-after-queue")).not.toBeChecked();
    expect(within(panel).getByTestId("viewport-create-pipe-from")).toHaveValue("");
    await act(async () => { apply.resolve(false); await apply.promise; });
  });
});

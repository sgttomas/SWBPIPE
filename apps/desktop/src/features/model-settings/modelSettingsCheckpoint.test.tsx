// Session checkpoint Undo/Redo for the node-proximity tolerance (DEL-02-01 Design
// §3.8). The applier's inverse of set-then-clear cannot restore an authored empty
// `model_settings: {}` (clear removes the empty record), so the desktop's checkpoint
// is what restores the exact prior document and model hash. A stale edit is refused
// and leaves the document and history unchanged. Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { act, cleanup, renderHook, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import * as previewService from "../../services/previewService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { clearNodeProximityToleranceIntent, setNodeProximityToleranceIntent } from "./modelSettingsIntents";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const demo = () =>
  JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

it("checkpoint Undo/Redo restores an authored empty model_settings and its hash; a stale edit changes nothing", async () => {
  const authoredEmpty = { ...demo(), model_settings: {} } as PreviewModel;
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(authoredEmpty);
  const { result } = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(result.current.model.model).not.toBeNull());
  const original = JSON.stringify(result.current.model.model);
  const originalHash = await computeModelHash(result.current.model.model!);

  const set = await setNodeProximityToleranceIntent(canonicalJsonString, result.current.model.model!, "2", "mm");
  const stale = await setNodeProximityToleranceIntent(canonicalJsonString, result.current.model.model!, "3", "mm");
  await act(async () => {
    expect(await result.current.operations.handleApplyIntent(set)).toBe(true);
  });
  const authored = JSON.stringify(result.current.model.model);
  const authoredHash = await computeModelHash(result.current.model.model!);

  await act(async () => {
    expect(await result.current.operations.handleApplyIntent(stale)).toBe(false);
  });
  expect(JSON.stringify(result.current.model.model)).toBe(authored);
  expect(result.current.operations.undoStack).toHaveLength(1);
  expect(
    Object.values(result.current.operations.operationOutcomes).some((outcome) =>
      outcome.diagnostics.some((diagnostic) => diagnostic.code === "OP-STALE-BEFORE-VALUE"),
    ),
  ).toBe(true);

  act(() => result.current.operations.handleUndoSessionModelEdit());
  expect(JSON.stringify(result.current.model.model)).toBe(original);
  expect(await computeModelHash(result.current.model.model!)).toEqual(originalHash);
  act(() => result.current.operations.handleRedoSessionModelEdit());
  expect(JSON.stringify(result.current.model.model)).toBe(authored);
  expect(await computeModelHash(result.current.model.model!)).toEqual(authoredHash);

  const clear = await clearNodeProximityToleranceIntent(canonicalJsonString, result.current.model.model!);
  await act(async () => {
    expect(await result.current.operations.handleApplyIntent(clear)).toBe(true);
  });
  expect(result.current.model.model).not.toHaveProperty("model_settings");
  act(() => result.current.operations.handleUndoSessionModelEdit());
  expect(JSON.stringify(result.current.model.model)).toBe(authored);
  act(() => result.current.operations.handleUndoSessionModelEdit());
  expect(JSON.stringify(result.current.model.model)).toBe(original);
  expect(await computeModelHash(result.current.model.model!)).toEqual(originalHash);
});

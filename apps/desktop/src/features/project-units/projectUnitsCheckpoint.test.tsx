// W-17 slice (b), DEL-02-02 Design `project-unit-selection.md` §2.5, §2.8, §4:
// a project-unit change goes through the session's commit like every
// operation, and Undo/Redo restore the checkpoint (the exact prior document and
// model hash). A stale change is refused and leaves the document and history
// unchanged. Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { act, cleanup, renderHook, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import * as previewService from "../../services/previewService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { setProjectUnitIntent } from "./projectUnitIntents";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const demo = () =>
  JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

it("checkpoint Undo/Redo restores the previous units map and hash; a stale change changes nothing", async () => {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(demo());
  const { result } = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(result.current.model.model).not.toBeNull());
  const original = JSON.stringify(result.current.model.model);
  const originalUnits = { ...result.current.model.model!.project.units };
  const originalHash = await computeModelHash(result.current.model.model!);

  const set = await setProjectUnitIntent(canonicalJsonString, result.current.model.model!, "force", "lbf");
  const stale = await setProjectUnitIntent(canonicalJsonString, result.current.model.model!, "force", "kN");
  await act(async () => {
    expect(await result.current.operations.handleApplyIntent(set)).toBe(true);
  });
  expect(result.current.model.model!.project.units).toEqual({ ...originalUnits, force: "lbf" });
  const authored = JSON.stringify(result.current.model.model);
  const authoredHash = await computeModelHash(result.current.model.model!);
  expect(authoredHash).not.toEqual(originalHash);

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
  expect(result.current.model.model!.project.units).toEqual(originalUnits);
  expect(await computeModelHash(result.current.model.model!)).toEqual(originalHash);
  act(() => result.current.operations.handleRedoSessionModelEdit());
  expect(JSON.stringify(result.current.model.model)).toBe(authored);
  expect(await computeModelHash(result.current.model.model!)).toEqual(authoredHash);

  // A missing key is set from not_present; undo removes it again.
  const moment = await setProjectUnitIntent(canonicalJsonString, result.current.model.model!, "moment", "N*m");
  expect(moment.change.before).toBe("not_present");
  await act(async () => {
    expect(await result.current.operations.handleApplyIntent(moment)).toBe(true);
  });
  expect(result.current.model.model!.project.units.moment).toBe("N*m");
  act(() => result.current.operations.handleUndoSessionModelEdit());
  expect(JSON.stringify(result.current.model.model)).toBe(authored);
  act(() => result.current.operations.handleUndoSessionModelEdit());
  expect(JSON.stringify(result.current.model.model)).toBe(original);
});

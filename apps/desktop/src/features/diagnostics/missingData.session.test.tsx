// DEL-07-06 AC-007, Missing data, before any solve (issue 530's repair): the workspace
// session opens the use case's start model and runs the model checks by itself, in the
// actual wasm engine; each missing density is a finding on its pipe with no solve.
// Entering the person's first value (`person_entered.json`) leaves exactly the other;
// entering both leaves none, and the Solve command's gate is open. No solve starts. The
// model and the values are the use case's invented ones.
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import startModel from "../../../../../fixtures/embedded_agent/use_cases/missing-data/start_model.json";
import personEntered from "../../../../../fixtures/embedded_agent/use_cases/missing-data/person_entered.json";
import * as previewService from "../../services/previewService";
import type { PreviewModel } from "../../types";
import { buildGridOperationIntent, type GridColumn } from "../workspace/table/modelTableAdapter";
import { useWorkspaceSession, type WorkspaceSession } from "../workspace/workspaceSession";
import { isSolveInputMissing, SOLVE_INPUT_MISSING_CODES } from "./solveInputMissingCodes";

afterEach(() => { vi.restoreAllMocks(); });

type Entry = (typeof personEntered.entries)[number];

/** The missing solve inputs the session's current run lists, as (pipe, field) pairs; null
 * while no completed run describes the model as it stands. */
function missing(session: WorkspaceSession): string[][] | null {
  const run = session.chrome.modelCheckRun;
  if (!run || run.model !== session.model.model || run.solve.kind !== "checked") return null;
  return run.findings
    .filter((finding) => finding.group === "solve" && isSolveInputMissing(finding.diagnostic))
    .map((finding) => (finding.diagnostic.affected_refs ?? []).slice(0, 2));
}

/** The person's entry, as the grid's quantity cell queues it for an absent optional value. */
function entryIntent(model: PreviewModel, entry: Entry, sequence: number) {
  expect(entry.field_path).toBe("section.material_density");
  const pipe = model.pipe_segments.find((item) => item.id === entry.entity_ref)!;
  const column: GridColumn = {
    key: "material_density", label: "Material density", fieldPath: "section.material_density.value", objectType: "Element",
    changeKind: "set_field", dimension: "density", sourceNote: "unit metadata required; entered unit captured explicitly",
    unit: () => entry.unit, value: () => "TBD", quantity: true
  };
  return buildGridOperationIntent({ column, model, row: { id: pipe.id, label: pipe.id, type: "pipe", raw: pipe, searchText: "" }, sequence, value: String(entry.value) });
}

it("reports every missing density before any solve, and follows each entered value until none is missing and Solve is open", async () => {
  expect(SOLVE_INPUT_MISSING_CODES.has("EQUIVALENT_STATIC_INPUT_MISSING")).toBe(true);
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(structuredClone(startModel) as unknown as PreviewModel);
  const start = vi.spyOn(previewService, "startPreviewMechanicsJob").mockRejectedValue(new Error("INVENTED-START-OBSERVED"));
  const { result } = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(result.current.model.model).not.toBeNull());

  // Opened: both densities are findings on their pipes, and Solve is withheld, with no solve.
  await waitFor(() => expect(missing(result.current)).toEqual([["pipe:md-inboard", "material_density"], ["pipe:md-outboard", "material_density"]]));
  expect(result.current.chrome.modelCheckRun!.findings.filter((finding) => finding.group === "solve" && finding.diagnostic.severity === "blocking").map((finding) => finding.diagnostic.code))
    .toEqual(["EQUIVALENT_STATIC_INPUT_MISSING", "EQUIVALENT_STATIC_INPUT_MISSING"]);
  expect(result.current.results.solveGate.kind).toBe("withheld");

  // The first value entered: exactly the other remains.
  const [first, second] = personEntered.entries;
  let applied = false;
  await act(async () => { applied = await result.current.operations.handleApplyIntent(entryIntent(result.current.model.model!, first, 1)); });
  expect(applied).toBe(true);
  expect(result.current.model.model!.pipe_segments.find((pipe) => pipe.id === first.entity_ref)!.section.material_density).toEqual({ value: first.value, unit: first.unit });
  await waitFor(() => expect(missing(result.current)).toEqual([["pipe:md-outboard", "material_density"]]));
  expect(result.current.results.solveGate.kind).toBe("withheld");

  // Both entered: none remains, and the Solve command's gate is open.
  await act(async () => { applied = await result.current.operations.handleApplyIntent(entryIntent(result.current.model.model!, second, 2)); });
  expect(applied).toBe(true);
  await waitFor(() => expect(missing(result.current)).toEqual([]));
  expect(result.current.results.solveGate.kind).toBe("open");
  expect(start).not.toHaveBeenCalled();
});

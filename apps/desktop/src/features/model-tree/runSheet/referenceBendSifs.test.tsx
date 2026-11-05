// Reference proof for the run sheet's bend SIF cells (DEL-09-01 `reference-system-run-sheet.md`
// §2, §3.4, §3.5; DEL-07-02 `run-sheet-entry.md` §1.2, §3.3, §9). The 26 reference rows are
// entered in table order with the run sheet's header and ids; the three bend rows (k, k source,
// SIFs, ids, geometry source) and the valve/flange rows (Component details) are typed through
// the real grid, the other rows through the same commit functions. Queue, preview, Apply, Undo,
// Redo, browser save and reopen; then the bend components, arcs, tangent nodes and the
// valve/flange components are compared with `fixtures/reference_system/reference_system.json`.
// Functional authoring-path evidence only: not a timing, native-usability or physics result.
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import referenceJson from "../../../../../../fixtures/reference_system/reference_system.json";
import { createLocalProject, openLocalProject } from "../../../services/projectService";
import * as previewService from "../../../services/previewService";
import type { PreviewModel } from "../../../types";
import { useModelledVisibility } from "../../../test-support/modelledVisibility";
import { useWorkspaceSession } from "../../workspace/workspaceSession";
import { RunSheet, RUN_SHEET_TIMING } from "./RunSheet";
import { BEND_LABELS } from "./runSheetBatch";
import { useRunSheetStore } from "./runSheetStore";
import { enterReferenceHeader, enterReferenceRows, REFERENCE_ROWS as ROWS, runSheetCell as cell, S5 } from "../../../test-support/referenceRunSheetEntry";

afterEach(() => { cleanup(); vi.restoreAllMocks(); RUN_SHEET_TIMING.engineCheckDelayMs = 250; });
useModelledVisibility();

const reference = referenceJson as unknown as PreviewModel;
/** The project's records the sheet does not create (spec §2: sections and the material are created first). */
function basisModel(): PreviewModel {
  const model = structuredClone(reference);
  for (const key of ["nodes", "pipe_segments", "components", "supports", "load_cases", "combinations", "reference_configurations", "diagnostics"] as const) (model as unknown as Record<string, unknown>)[key] = [];
  return model;
}
/** The fixture's records this route produces, by id. Labels the sheet fixes (§3.14) are the sheet's. */
function expectedRecords(): Array<[string, "nodes" | "pipe_segments" | "components", unknown]> {
  const out: Array<[string, "nodes" | "pipe_segments" | "components", unknown]> = [];
  const take = (collection: "nodes" | "pipe_segments" | "components", id: string, label?: string) => {
    const record = (reference[collection] as Array<{ id: string; label?: string }>).find((item) => item.id === id)!;
    expect(record, id).toBeDefined();
    out.push([id, collection, label === undefined ? record : { ...record, label }]);
  };
  for (const [component, arc, start, end] of [["RS-B1", "RS-P065", "RS-070", "RS-075"], ["RS-B2", "RS-P085", "RS-090", "RS-095"], ["RS-B3", "RS-P225", "RS-220", "RS-225"]]) {
    take("components", `component:${component}`, BEND_LABELS.component);
    take("pipe_segments", `pipe:${arc}`, BEND_LABELS.arc);
    take("nodes", `node:${start}`, BEND_LABELS.start);
    take("nodes", `node:${end}`, BEND_LABELS.end);
  }
  for (const id of ["RS-F1", "RS-V1", "RS-F2"]) take("components", `component:${id}`);
  return out;
}
function compareRecords(model: PreviewModel, where: string) {
  for (const [id, collection, record] of expectedRecords()) {
    const found = (model[collection] as Array<{ id: string }>).filter((item) => item.id === id);
    expect(found, `${where}: ${id}`).toEqual([record]);
  }
}

describe("reference bends with SIFs and the valve/flange rows, entered through the grid", () => {
  it("Queue, preview, Apply, Undo, Redo, browser save and reopen reproduce the fixture's bend, arc, tangent and valve/flange records", async () => {
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(basisModel());
    RUN_SHEET_TIMING.engineCheckDelayMs = 60_000;
    const current = {} as { session: ReturnType<typeof useWorkspaceSession>; store: ReturnType<typeof useRunSheetStore> };
    function Harness() {
      const session = useWorkspaceSession();
      const store = useRunSheetStore(session.operations.sessionEvents, session.model.activeModelIndex, session.model.model?.project.id ?? null);
      current.session = session; current.store = store;
      return session.model.model ? <RunSheet model={session.model.model} host={{ store, queuedBatches: session.operations.queuedBatches, editorIntents: session.operations.editorIntents, registerBatch: session.operations.registerOperationBatch }} /> : null;
    }
    render(<Harness />);
    await screen.findByTestId("run-sheet");
    const model = current.session.model.model!;
    // Header (run sheet §2) and the 26 rows (§3.4), shared with the reference-entry journey.
    enterReferenceHeader(current.store, model);
    const gridEntries = enterReferenceRows(() => current.store, model, reference);
    // B1 13, B2 12 (Bend R inherits) and B3 14 bend-row cells, six of them SIFs; each valve/flange
    // row's increment and Kind plus its 19 component fields.
    expect(gridEntries).toBe(13 + 12 + 14 + 3 * (2 + 19));

    const queue = screen.getByTestId("run-sheet-queue");
    await waitFor(() => expect(queue, screen.getByTestId("run-sheet-queue-reason").textContent ?? "").toBeEnabled(), { timeout: 10_000 });
    await act(async () => { fireEvent.click(queue); });
    await waitFor(() => expect(current.session.operations.queuedBatches, screen.getByTestId("run-sheet-live").textContent ?? "").toHaveLength(1));
    const operations = current.session.operations.queuedBatches[0].batch.operations;
    const realizations = operations.filter((op) => op.change.field_path === "bend_realization");
    expect(realizations.map((op) => [op.target.ref, JSON.parse(op.change.after)])).toEqual([
      ["component:RS-B1", { solver_consumption: "curved_bend_macro_element", flexibility_factor_user_value: { value: 4.2, unit: "none" }, source_reference: S5, sif_in_plane_user_value: { value: 2.1, unit: "1" }, sif_out_of_plane_user_value: { value: 1.8, unit: "1" } }],
      ["component:RS-B2", { solver_consumption: "curved_bend_macro_element", flexibility_factor_user_value: { value: 4.2, unit: "none" }, source_reference: S5, sif_in_plane_user_value: { value: 2.1, unit: "1" }, sif_out_of_plane_user_value: { value: 1.8, unit: "1" } }],
      ["component:RS-B3", { solver_consumption: "curved_bend_macro_element", flexibility_factor_user_value: { value: 3.6, unit: "none" }, source_reference: S5, sif_in_plane_user_value: { value: 2.3, unit: "1" }, sif_out_of_plane_user_value: { value: 1.9, unit: "1" } }],
    ]);
    for (const op of realizations) expect(operations[operations.indexOf(op) - 1].change.change_kind).toBe("insert_bend_at_corner");
    expect(current.session.model.model).toEqual(model);
    // Apply is refused until a clean preview of this exact batch; then all of it applies at once.
    await act(async () => { await current.session.operations.handleRunOperationBatch(current.session.operations.queuedBatches[0], true); });
    expect(current.session.model.model).toEqual(model);
    await act(async () => { await current.session.operations.handleRunOperationBatch(current.session.operations.queuedBatches[0], false); });
    await act(async () => { await current.session.operations.handleRunOperationBatch(current.session.operations.queuedBatches[0], true); });
    expect(current.session.operations.undoStack, current.session.operations.operationMessage ?? "").toHaveLength(1);
    const applied = current.session.model.model!;
    compareRecords(applied, "applied");
    expect(ROWS.map((_, index) => current.store.statuses.get(`r${index + 1}`)?.status)).toEqual(ROWS.map(() => "applied"));
    for (const [key, ip, op] of [["r8", "2.1", "1.8"], ["r10", "2.1", "1.8"], ["r25", "2.3", "1.9"]]) {
      expect(cell(key, "sifInPlane")).toHaveTextContent(ip);
      expect(cell(key, "sifOutOfPlane")).toHaveTextContent(op);
      expect(cell(key, "sifInPlane")).toHaveAccessibleDescription(/^stored component value/);
    }

    act(() => current.session.operations.handleUndoSessionModelEdit());
    expect(current.session.model.model).toEqual(model);
    expect(current.store.statuses.get("r25")?.status).toBe("draft");
    expect(cell("r25", "sifInPlane")).toHaveTextContent("2.3");
    act(() => current.session.operations.handleRedoSessionModelEdit());
    expect(current.session.model.model).toEqual(applied);
    expect(ROWS.map((_, index) => current.store.statuses.get(`r${index + 1}`)?.status)).toEqual(ROWS.map(() => "applied"));
    compareRecords(current.session.model.model!, "redone");

    await createLocalProject(current.session.model.model!);
    const reopened = await openLocalProject(model.project.id);
    compareRecords(reopened!.model, "reopened");
  }, 180_000);
});

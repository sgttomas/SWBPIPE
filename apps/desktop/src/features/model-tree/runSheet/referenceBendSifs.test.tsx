// Reference proof for the run sheet's bend SIF cells (DEL-09-01 `reference-system-run-sheet.md`
// §2, §3.4, §3.5; DEL-07-02 `run-sheet-entry.md` §1.2, §3.3, §9). The 26 reference rows are
// entered in table order with the run sheet's header and ids; the three bend rows (k, k source,
// SIFs, ids, geometry source) and the valve/flange rows (Component details) are typed through
// the real grid, the other rows through the same commit functions. Queue, preview, Apply, Undo,
// Redo, browser save and reopen; then the bend components, arcs, tangent nodes and the
// valve/flange components are compared with `fixtures/reference_system/reference_system.json`.
// Functional authoring-path evidence only: not a timing, native-usability or physics result.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import referenceJson from "../../../../../../fixtures/reference_system/reference_system.json";
import { createLocalProject, openLocalProject } from "../../../services/projectService";
import * as previewService from "../../../services/previewService";
import type { PreviewModel } from "../../../types";
import { useModelledVisibility } from "../../../test-support/modelledVisibility";
import { useWorkspaceSession } from "../../workspace/workspaceSession";
import { RunSheet, RUN_SHEET_TIMING } from "./RunSheet";
import { BEND_LABELS } from "./runSheetBatch";
import { blankRow, initialRunSheetState, type RunSheetState } from "./runSheetLedger";
import { useRunSheetStore } from "./runSheetStore";
import { COMPONENT_FIELDS, componentColumn, type ComponentRowInput } from "./runSheetComponents";
import { assignIds, recordShownPipeIds, setCell, sheetView, type SheetColumnKey } from "./runSheetView";

afterEach(() => { cleanup(); vi.restoreAllMocks(); RUN_SHEET_TIMING.engineCheckDelayMs = 250; });
useModelledVisibility();

const reference = referenceJson as unknown as PreviewModel;
const P = "invented reference-system fixture input (DEL-09-01 Work/reference-system.md); not standard, supplier, commercial or project data";
const S5 = "invented user value, DEL-09-01 Work/reference-system.md §5";
/** The project's records the sheet does not create (spec §2: sections and the material are created first). */
function basisModel(): PreviewModel {
  const model = structuredClone(reference);
  for (const key of ["nodes", "pipe_segments", "components", "supports", "load_cases", "combinations", "reference_configurations", "diagnostics"] as const) (model as unknown as Record<string, unknown>)[key] = [];
  return model;
}
type Cells = Partial<Record<SheetColumnKey, string>>;
const support = (code: string, restraints: string, id: string, label: string): Cells => ({ support: code, restraints, supportId: id, supportLabel: label });
/** Run sheet §3.4, in table order. `grid` rows are typed through the real cells. */
const ROWS: Array<{ cells: Cells; grid?: true; component?: string }> = [
  { cells: { dz: "1", kind: "P", ...support("A", "*", "support:RS-A", "Anchor A (nozzle A)") } },
  { cells: { dx: "0.4", section: "section:RS-S270", material: "material:RS-M1", orientation: "0 0 1", insulationThickness: "60", insulationDensity: "140", contentsDensity: "820", materialDensity: "7850", millTolerance: "1.0", corrosionAllowance: "1.5" } },
  { cells: { dx: "0.15", kind: "L" }, grid: true, component: "RS-F1" },
  { cells: { dx: "0.6", kind: "V" }, grid: true, component: "RS-V1" },
  { cells: { dx: "0.15", kind: "L" }, grid: true, component: "RS-F2" },
  { cells: { dx: "2.2", ...support("L", "UX", "support:RS-LS1", "Line stop LS1") } },
  { cells: { kind: "S", ...support("V", "UZ", "support:RS-V060", "Vertical support V060") } },
  { cells: { dx: "2.5", kind: "B", nodeId: "node:RS-B1-CORNER", bendRadius: "0.4", k: "4.2", kSource: S5, sifInPlane: "2.1", sifOutOfPlane: "1.8", bendStartId: "node:RS-070", bendEndId: "node:RS-075", bendArcId: "pipe:RS-P065", bendComponentId: "component:RS-B1", bendSource: S5 }, grid: true },
  { cells: { dz: "3", pipeId: "pipe:RS-P070", orientation: "1 0 0" } },
  { cells: { dz: "3", kind: "B", nodeId: "node:RS-B2-CORNER", k: "4.2", kSource: S5, sifInPlane: "2.1", sifOutOfPlane: "1.8", bendStartId: "node:RS-090", bendEndId: "node:RS-095", bendArcId: "pipe:RS-P085", bendComponentId: "component:RS-B2", bendSource: S5 }, grid: true },
  { cells: { dy: "3", pipeId: "pipe:RS-P090", orientation: "0 0 1" } },
  { cells: { dy: "4" } },
  { cells: { dy: "4", ...support("V", "UZ", "support:RS-FR1V", "RS-FR1V interim linear vertical support (one_way target pending R-4 S7)") } },
  { cells: { dy: "4" } },
  { cells: { dy: "3", ...support("V", "UZ", "support:RS-V140", "Vertical support V140") } },
  { cells: { dy: "2" } },
  { cells: { dy: "0.4", dz: "-0.025" } },
  { cells: { dy: "2.6", section: "section:RS-S220", millTolerance: "0.9", ...support("G", "UX UZ", "support:RS-G2", "Guide G2") } },
  { cells: { dy: "2" } },
  { cells: { dy: "0.6" } },
  { cells: { dy: "2.4", ...support("V", "UZ", "support:RS-V200", "Vertical support V200") } },
  { cells: { dy: "2", ...support("A", "*", "support:RS-B", "Anchor B (nozzle B)") } },
  { cells: { kind: "F", node: "node:RS-130" } },
  { cells: { dz: "-2.5", nodeId: "node:RS-215", pipeId: "pipe:RS-P210", section: "section:RS-S170", orientation: "1 0 0", insulationThickness: "50", millTolerance: "0.8", ...support("G", "UX UY", "support:RS-G3", "Branch guide G3") } },
  { cells: { dz: "-2.5", kind: "B", nodeId: "node:RS-B3-CORNER", pipeId: "pipe:RS-P220", bendRadius: "0.25", k: "3.6", kSource: S5, sifInPlane: "2.3", sifOutOfPlane: "1.9", bendStartId: "node:RS-220", bendEndId: "node:RS-225", bendArcId: "pipe:RS-P225", bendComponentId: "component:RS-B3", bendSource: S5 }, grid: true },
  { cells: { dx: "2", nodeId: "node:RS-230", pipeId: "pipe:RS-P230", orientation: "0 0 1", ...support("A", "*", "support:RS-C", "Anchor C (nozzle C)") } },
];
/** The valve and flanges' Component details (§3.4), from the reference table. */
function componentCells(id: string): ComponentRowInput {
  const record = reference.components.find((item) => item.id === `component:${id}`)!;
  const g = record.geometry!;
  return {
    id: record.id, label: record.label, rigidBodyLength: String(g.rigid_body_length!.value), endASize: "270", endASizeUnit: "mm", endBSize: "270", endBSizeUnit: "mm",
    weight: String(g.weight!.value), centerOfGravityX: String(g.center_of_gravity!.x), centerOfGravityY: String(g.center_of_gravity!.y), centerOfGravityZ: String(g.center_of_gravity!.z),
    connectionEndAReference: g.connection_end_a_reference!, connectionEndBReference: g.connection_end_b_reference!, stiffnessBehaviorReference: g.stiffness_behavior_reference!,
    rigidStiffnessScale: "10.0", rigidScaleSourceReference: S5, rigidSolverConsumption: "mechanics_geometry_only", geometrySourceReference: S5, provenance: P,
  };
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
    // Header (run sheet §2): Source ⟨P⟩, m, node:RS- / pipe:RS-P, 10/10, pad 3, from its start node; the six column units.
    act(() => current.store.update(() => {
      const state = initialRunSheetState(model.project.id, "reference", 1);
      return {
        ...state,
        header: { source: P, lengthUnit: "m", numbering: { ...state.header.numbering, nodePrefix: "node:RS-", pipePrefix: "pipe:RS-P", start: 10, step: 10, pad: 3, pipeNumber: "start" } },
        columnUnits: { insulationThickness: "mm", insulationDensity: "kg/m^3", contentsDensity: "kg/m^3", materialDensity: "kg/m^3", millTolerance: "mm", corrosionAllowance: "mm" },
      };
    }));
    const cell = (rowKey: string, column: string) => screen.getByTestId(`run-sheet-cell-${rowKey}-${column}`);
    const toggle = (name: string, shown: boolean) => {
      const button = screen.getByRole("button", { name });
      if ((button.getAttribute("aria-expanded") === "true") !== shown) fireEvent.click(button);
    };
    /** A person's entry: a code key in a code cell; otherwise a printable key opens the editor and Enter commits. */
    function type(rowKey: string, column: string, value: string) {
      const target = cell(rowKey, column);
      act(() => target.focus());
      if (column === "kind" || column === "support") { fireEvent.keyDown(target, { key: value }); expect(target).toHaveTextContent(value); return; }
      fireEvent.keyDown(target, { key: value[0] });
      const editor = within(target).getByRole("textbox");
      fireEvent.change(editor, { target: { value } });
      fireEvent.keyDown(editor, { key: "Enter" });
    }
    /** The commit path's own functions, cell by cell, in entry order (`RunSheet` commit and its shown-id recording). */
    function commit(rowKey: string, cells: Cells) {
      act(() => current.store.update((state) => Object.entries(cells).reduce((next: RunSheetState, [column, text]) => {
        let written = setCell(next, rowKey, column as SheetColumnKey, text!);
        if (text!.trim() || column === "kind") written = assignIds(written, rowKey, model, []);
        return recordShownPipeIds(written, sheetView({ state: written, statuses: current.store.statuses, model, conversions: new Map(), reservedIds: [], lookup: current.store.lookup }));
      }, state)));
    }
    let gridEntries = 0;
    for (const [index, row] of ROWS.entries()) {
      const key = `r${index + 1}`;
      if (!current.store.state.rows.some((item) => item.key === key)) act(() => current.store.update((state) => ({ ...state, rows: [...state.rows, blankRow(key)], nextRowKey: state.nextRowKey + 1 })));
      if (!row.grid) { commit(key, row.cells); continue; }
      const bend = row.cells.kind === "B";
      toggle("Ids and sources", bend);
      toggle("Component details", Boolean(row.component));
      for (const [column, value] of Object.entries(row.cells)) { type(key, column, value!); gridEntries += 1; }
      if (row.component) {
        const values = componentCells(row.component);
        for (const field of COMPONENT_FIELDS) {
          const value = values[field.key];
          if (!value) continue;
          type(key, componentColumn(field.key), value);
          gridEntries += 1;
        }
        // Body length, Weight and COG units are blank, shown as the project's (m, N).
        expect(cell(key, "component.lengthUnit")).toHaveAccessibleDescription(/derived: the project unit/);
      }
      if (bend) {
        expect(cell(key, "sifInPlane")).toHaveAccessibleName(`Row ${index + 1} In-plane SIF i_ip: ${row.cells.sifInPlane}`);
        expect(cell(key, "sifOutOfPlane")).toHaveAccessibleDescription(/^entered; source: this row's k source/);
      }
    }
    // B1 13, B2 12 (Bend R inherits) and B3 14 bend-row cells, six of them SIFs; each valve/flange
    // row's increment and Kind plus its 19 component fields.
    expect(gridEntries).toBe(13 + 12 + 14 + 3 * (2 + 19));
    toggle("Ids and sources", false); toggle("Component details", false);

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

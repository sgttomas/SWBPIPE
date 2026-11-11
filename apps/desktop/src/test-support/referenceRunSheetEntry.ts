// The reference system's run sheet (DEL-09-01),
// shared by the bend-SIF proof (`runSheet/referenceBendSifs.test.tsx`) and the reference-entry
// journey (`workspace/referenceEntryJourney.session.test.tsx`). The rows, the header and the
// valve/flange Component details are the run sheet's; `enterReferenceRows` types the bend and
// valve/flange rows through the real grid cells and the other rows through the sheet's own
// commit functions, in table order. Test support only: no product behaviour lives here.
import { act, fireEvent, screen, within } from "@testing-library/react";
import { expect } from "vitest";
import type { PreviewModel } from "../types";
import { blankRow, initialRunSheetState, type RunSheetState } from "../features/model-tree/runSheet/runSheetLedger";
import type { useRunSheetStore } from "../features/model-tree/runSheet/runSheetStore";
import { COMPONENT_FIELDS, componentColumn, type ComponentRowInput } from "../features/model-tree/runSheet/runSheetComponents";
import { assignIds, recordShownPipeIds, setCell, sheetView, type SheetColumnKey } from "../features/model-tree/runSheet/runSheetView";

/** ⟨P⟩ and ⟨S5⟩ of the run sheet (§1). */
export const P = "invented reference-system fixture input (DEL-09-01); not standard, supplier, commercial or project data";
export const S5 = "invented user value, DEL-09-01";

export type Cells = Partial<Record<SheetColumnKey, string>>;
const support = (code: string, restraints: string, id: string, label: string): Cells => ({ support: code, restraints, supportId: id, supportLabel: label });
/** Run sheet §3.4, in table order. `grid` rows are typed through the real cells. */
export const REFERENCE_ROWS: Array<{ cells: Cells; grid?: true; component?: string }> = [
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
export function referenceComponentCells(reference: PreviewModel, id: string): ComponentRowInput {
  const record = reference.components.find((item) => item.id === `component:${id}`)!;
  const g = record.geometry!;
  return {
    id: record.id, label: record.label, rigidBodyLength: String(g.rigid_body_length!.value), endASize: "270", endASizeUnit: "mm", endBSize: "270", endBSizeUnit: "mm",
    weight: String(g.weight!.value), centerOfGravityX: String(g.center_of_gravity!.x), centerOfGravityY: String(g.center_of_gravity!.y), centerOfGravityZ: String(g.center_of_gravity!.z),
    connectionEndAReference: g.connection_end_a_reference!, connectionEndBReference: g.connection_end_b_reference!, stiffnessBehaviorReference: g.stiffness_behavior_reference!,
    rigidStiffnessScale: "10.0", rigidScaleSourceReference: S5, rigidSolverConsumption: "mechanics_geometry_only", geometrySourceReference: S5, provenance: P,
  };
}

type Store = ReturnType<typeof useRunSheetStore>;

/** The run sheet's header (§2): Source ⟨P⟩, m, node:RS- / pipe:RS-P, 10/10, pad 3, from its start node; the six column units.
 * Draft setup: it sets the sheet's draft state directly, not through the header controls' handlers. */
export function enterReferenceHeader(store: Store, model: PreviewModel): void {
  act(() => store.update(() => {
    const state = initialRunSheetState(model.project.id, "reference", 1);
    return {
      ...state,
      header: { source: P, lengthUnit: "m", numbering: { ...state.header.numbering, nodePrefix: "node:RS-", pipePrefix: "pipe:RS-P", start: 10, step: 10, pad: 3, pipeNumber: "start" } },
      columnUnits: { insulationThickness: "mm", insulationDensity: "kg/m^3", contentsDensity: "kg/m^3", materialDensity: "kg/m^3", millTolerance: "mm", corrosionAllowance: "mm" },
    };
  }));
}

export const runSheetCell = (rowKey: string, column: string) => screen.getByTestId(`run-sheet-cell-${rowKey}-${column}`);

/**
 * Enters §3.4's 26 rows in table order on the mounted `RunSheet` whose store `store()` returns.
 * `grid` rows are typed through the real cells (a code key in a code cell; otherwise a printable
 * key opens the editor and Enter commits); the other rows go through the commit path's own
 * functions, cell by cell (`RunSheet` commit and its shown-id recording). `extra` adds cells to a
 * row (by its zero-based table index), entered by the same route as the row's own. Returns the
 * number of cells typed through the grid.
 */
export function enterReferenceRows(store: () => Store, model: PreviewModel, reference: PreviewModel, extra: (index: number) => Cells = () => ({})): number {
  const toggle = (name: string, shown: boolean) => {
    const button = screen.getByRole("button", { name });
    if ((button.getAttribute("aria-expanded") === "true") !== shown) fireEvent.click(button);
  };
  function type(rowKey: string, column: string, value: string) {
    const target = runSheetCell(rowKey, column);
    act(() => target.focus());
    if (column === "kind" || column === "support") { fireEvent.keyDown(target, { key: value }); expect(target).toHaveTextContent(value); return; }
    fireEvent.keyDown(target, { key: value[0] });
    const editor = within(target).getByRole("textbox");
    fireEvent.change(editor, { target: { value } });
    fireEvent.keyDown(editor, { key: "Enter" });
  }
  function commit(rowKey: string, cells: Cells) {
    act(() => store().update((state) => Object.entries(cells).reduce((next: RunSheetState, [column, text]) => {
      let written = setCell(next, rowKey, column as SheetColumnKey, text!);
      if (text!.trim() || column === "kind") written = assignIds(written, rowKey, model, []);
      return recordShownPipeIds(written, sheetView({ state: written, statuses: store().statuses, model, conversions: new Map(), reservedIds: [], lookup: store().lookup }));
    }, state)));
  }
  let gridEntries = 0;
  for (const [index, row] of REFERENCE_ROWS.entries()) {
    const key = `r${index + 1}`;
    if (!store().state.rows.some((item) => item.key === key)) act(() => store().update((state) => ({ ...state, rows: [...state.rows, blankRow(key)], nextRowKey: state.nextRowKey + 1 })));
    const cells = { ...row.cells, ...extra(index) };
    if (!row.grid) { commit(key, cells); continue; }
    const bend = row.cells.kind === "B";
    toggle("Ids and sources", bend);
    toggle("Component details", Boolean(row.component));
    for (const [column, value] of Object.entries(cells)) { type(key, column, value!); gridEntries += 1; }
    if (row.component) {
      const values = referenceComponentCells(reference, row.component);
      for (const field of COMPONENT_FIELDS) {
        const value = values[field.key];
        if (!value) continue;
        type(key, componentColumn(field.key), value);
        gridEntries += 1;
      }
      // Body length, Weight and COG units are blank, shown as the project's (m, N).
      expect(runSheetCell(key, "component.lengthUnit")).toHaveAccessibleDescription(/derived: the project unit/);
    }
    if (bend) {
      expect(runSheetCell(key, "sifInPlane")).toHaveAccessibleName(`Row ${index + 1} In-plane SIF i_ip: ${row.cells.sifInPlane}`);
      expect(runSheetCell(key, "sifOutOfPlane")).toHaveAccessibleDescription(/^entered; source: this row's k source/);
    }
  }
  toggle("Ids and sources", false); toggle("Component details", false);
  return gridEntries;
}

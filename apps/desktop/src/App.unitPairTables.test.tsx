// Issue 382 control (DEL-07-02 REQ-002 to REQ-004): the model tables always send an explicit
// value/unit pair in the stored unit, so the defect did not reach them; this shows they still
// work for every family member they offer. Through the whole App, the person edits a cell of the
// materials or sections table and its Apply runs the operation applier in the Wasm engine rebuilt
// from the candidate source (App.tsx `onApplyCellIntent`); the stored document is read back from
// the saved and reopened project. The tables offer material E, G and expansion coefficient and a
// shared section's diameter and wall; their quantity columns do not offer a unit change
// (`unitEditable: false`), and they do not offer a pipe's inline diameter or wall, a temperature
// point's G or a primitive magnitude. The model and values are invented.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { App } from "./App";
import * as previewService from "./services/previewService";
import { unitPairFamilyModel } from "./test-support/unitPairFamilyModel";
import { FAMILY_MEMBERS } from "./test-support/unitPairFamilyMembers";
import { captureAppSession, storedDocument } from "./test-support/unitPairSurfaceHarness";

// jsdom has no layout; the table's persistent editor needs a visible allocation (as MaterialTable.test.tsx).
beforeEach(() => { vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 1000, 400)); });
afterEach(() => { cleanup(); vi.restoreAllMocks(); });

const COLUMNS: Record<string, { grid: string; table: string; column: string }> = {
  "elastic_modulus.value": { grid: "materials", table: "material-engineering-table", column: "elastic" },
  "shear_modulus.value": { grid: "materials", table: "material-engineering-table", column: "shear" },
  "thermal_expansion_coefficient.value": { grid: "materials", table: "material-engineering-table", column: "thermal" },
  "properties.outside_diameter.value": { grid: "sections", table: "section-engineering-table", column: "outside" },
  "properties.wall_thickness.value": { grid: "sections", table: "section-engineering-table", column: "wall" }
};
const OFFERED = FAMILY_MEMBERS.filter((member) => member.fieldPath in COLUMNS);

describe.each(OFFERED)("model table: $name ($fieldPath)", (member) => {
  it("changing only the number keeps the stored unit, and the stored document holds it", async () => {
    const { grid, table, column } = COLUMNS[member.fieldPath];
    const session = captureAppSession();
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(unitPairFamilyModel());
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    fireEvent.click(await screen.findByTestId("layout-mode-grid"));
    fireEvent.click(screen.getByTestId(`entity-grid-type-${grid}`));
    const cell = `table-cell-${member.ref}-${column}`;
    await waitFor(() => expect(screen.getByTestId(cell).parentElement).toHaveAttribute("aria-readonly", "false"));
    fireEvent.doubleClick(screen.getByTestId(cell));
    const input = within(screen.getByTestId(table)).getByRole("textbox") as HTMLInputElement;
    // The cell names the stored unit; it offers no other.
    expect(input.getAttribute("aria-label") ?? input.labels?.[0]?.textContent ?? "").toContain(`[${member.stored.unit}]`);
    const applied = session.current.operations.appliedOperations.length;
    fireEvent.change(input, { target: { value: String(member.inStoredUnit) } });
    fireEvent.click(within(screen.getByTestId(table)).getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(session.current.operations.appliedOperations.length).toBe(applied + 1));
    await waitFor(() => expect(within(screen.getByTestId(table)).queryByRole("textbox")).toBeNull());
    // The applied receipt: the table sent the explicit pair in the stored unit.
    expect(session.current.operations.appliedOperations.find((receipt) => receipt.field_path === member.fieldPath)).toMatchObject({
      target_ref: member.ref, before: String(member.stored.value), after: JSON.stringify({ value: member.inStoredUnit, unit: member.stored.unit }) });
    expect(member.read(await storedDocument(session))).toEqual({ value: member.inStoredUnit, unit: member.stored.unit });
  });
});

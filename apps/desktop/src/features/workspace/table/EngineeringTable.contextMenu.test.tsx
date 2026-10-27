// B9: a table cell's context menu when the table changes under it. A row removed while its
// menu is open sends focus to the table's footer (as a removed edit row does), and a selection
// made elsewhere while the menu is open does not move the table's current cell, so closing the
// menu does not select the old row again. Invented rows only.
import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { useModelledVisibility } from "../../../test-support/modelledVisibility";
import { entityKey } from "../selectionState";
import { EngineeringTable } from "./EngineeringTable";
import { nodeCoordinateColumns } from "./modelTableAdapter";
import type { TableRow } from "./tableState";

useModelledVisibility();

const columns = nodeCoordinateColumns("m");
const rows: TableRow[] = [0, 1].map((i) => ({ key: entityKey({ type: "node", id: `n:${i}` }), label: `n:${i}`, cells: { x: { value: String(i), unit: "m" }, y: { value: "0", unit: "m" }, z: { value: "0", unit: "m" } } }));
const props = () => ({ label: "Invented table", rows, columns, generation: "g:1", density: "comfortable" as const, filter: "", selectedKey: rows[0].key,
  onSelect: vi.fn(), onApply: vi.fn(async () => ({ applied: true, messages: [] as string[] })) });
const cell = (i: number, column = "x") => screen.getByTestId(`table-cell-n:${i}-${column}`);

describe("the table cell menu when the table changes under it", () => {
  it("puts focus on the footer when the menu's row is removed, not on the page", () => {
    const p = props();
    const view = render(<EngineeringTable {...p} />);
    cell(1).focus();
    fireEvent.keyDown(cell(1), { key: "F10", shiftKey: true });
    view.rerender(<EngineeringTable {...p} rows={[rows[0]]} />);
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Escape" });
    expect(document.activeElement).not.toBe(document.body);
    expect(screen.getByRole("group", { name: "Invented table footer" })).toHaveFocus();
  });

  it("does not select the old row again when Escape returns focus after a selection made elsewhere", () => {
    const p = props();
    const view = render(<EngineeringTable {...p} />);
    cell(0).focus();
    p.onSelect.mockClear();
    fireEvent.keyDown(cell(0), { key: "F10", shiftKey: true });
    view.rerender(<EngineeringTable {...p} selectedKey={rows[1].key} />);
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Escape" });
    expect(cell(0)).toHaveFocus();
    expect(p.onSelect).not.toHaveBeenCalled();
  });
});

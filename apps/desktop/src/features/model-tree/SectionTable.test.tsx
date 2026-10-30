import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { ModelTree } from "./ModelTree";
import * as displayQuantityService from "../../services/displayQuantityService";
import { loadPreviewModel } from "../../services/previewService";
import type { EditorOperationIntent, PreviewModel } from "../../types";

async function fixture() {
  const model = await loadPreviewModel();
  model.sections = [
    { id: "s-a", name: "A", section_type: "pipe", properties: { outside_diameter: { value: 2, unit: "m" }, wall_thickness: { value: 10, unit: "mm" } }, provenance: "invented" },
    { id: "s-b", name: "B", section_type: "pipe", properties: { outside_diameter: { value: 100, unit: "mm" }, wall_thickness: { value: 5, unit: "mm" } }, provenance: "invented" }
  ];
  return model;
}
function open() { fireEvent.click(screen.getByTestId("layout-mode-grid")); fireEvent.click(screen.getByTestId("entity-grid-type-sections")); }
function table(review = false) { return screen.getByTestId(`section-engineering-table${review ? "-review" : ""}`); }
function editor(row: string, column: string, review = false) {
  fireEvent.doubleClick(screen.getByTestId(`${review ? "review" : "table"}-cell-${row}-${column}`));
  return within(table(review)).getByRole("textbox") as HTMLInputElement;
}
function order(review = false) { return within(table(review)).getAllByRole("rowheader").map((r) => r.textContent); }
function props(model: PreviewModel) { return { model, selection: { type: "section" as const, id: "s-a" }, onSelect: vi.fn(), onQueueIntent: vi.fn(), onApplyCellIntent: vi.fn(async (_intent: EditorOperationIntent) => ({ applied: true, messages: [] })) }; }

describe("Sections through common table", () => {
  beforeEach(() => { vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 1000, 400)); });
  afterEach(() => vi.restoreAllMocks());
  it("sorts physical OD and captures distinct actual OD/wall units, numeric no-op and canonical text before", async () => {
    const model = await fixture(); model.sections![0].name = " A "; const p = props(model); render(<ModelTree {...p} />); open();
    await waitFor(() => expect(screen.getByTestId("table-cell-s-a-outside").parentElement).toHaveAttribute("aria-readonly", "false"));
    fireEvent.click(within(table()).getByRole("button", { name: "Sort Outside dia." })); expect(order()).toEqual(["s-b", "s-a"]);
    let input = editor("s-a", "outside"); expect(input).toHaveAccessibleName("s-a Outside dia. [m]");
    fireEvent.change(input, { target: { value: "2.00" } }); fireEvent.click(within(table()).getByRole("button", { name: "Apply" })); expect(p.onApplyCellIntent).not.toHaveBeenCalled();
    input = editor("s-a", "wall"); expect(input).toHaveAccessibleName("s-a Wall [mm]");
    fireEvent.change(input, { target: { value: "11" } }); fireEvent.click(within(table()).getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(p.onApplyCellIntent).toHaveBeenCalledTimes(1));
    expect(p.onApplyCellIntent.mock.calls[0][0]).toMatchObject({ target: { object_type: "Section", ref: "s-a" }, change: { field_path: "properties.wall_thickness.value", before: "10", unit: "mm", after: JSON.stringify({ value: 11, unit: "mm" }) } });
    await waitFor(() => expect(within(table()).queryByRole("textbox")).toBeNull());
    fireEvent.change(editor("s-a", "name"), { target: { value: "A" } }); fireEvent.click(within(table()).getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(p.onApplyCellIntent).toHaveBeenCalledTimes(2)); expect(p.onApplyCellIntent.mock.calls[1][0].change).toMatchObject({ before: " A ", after: "A" });
  });
  it("keeps missing/TBD quantities and structured provenance read-only directly; review retains raw text and current units", async () => {
    const model = await fixture(); const section = model.sections![0] as any;
    section.properties.outside_diameter = { value: 2 }; section.properties.wall_thickness = { value: "TBD", unit: "mm" }; section.provenance = { source: "structured" };
    const p = props(model); const view = render(<ModelTree {...p} />); open(); await act(async () => {});
    for (const field of ["outside", "wall", "provenance"]) expect(screen.getByTestId(`table-cell-s-a-${field}`).parentElement).toHaveAttribute("aria-readonly", "true");
    fireEvent.click(screen.getByTestId("section-grid-review-disclosure"));
    fireEvent.change(editor("s-a", "outside", true), { target: { value: " " } }); fireEvent.click(within(table(true)).getByRole("button", { name: "Keep draft" }));
    fireEvent.change(editor("s-b", "name", true), { target: { value: "retained" } }); fireEvent.click(within(table(true)).getByRole("button", { name: "Keep draft" }));
    const changed = structuredClone(model); changed.sections![0].properties.outside_diameter = { value: 3, unit: "cm" }; view.rerender(<ModelTree {...p} model={changed} />);
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "s-a" } }); fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
    expect(p.onQueueIntent).toHaveBeenCalledTimes(1); expect(p.onQueueIntent.mock.calls[0][0].change).toMatchObject({ before: "3", unit: "cm", after: JSON.stringify({ value: "TBD", unit: "cm" }) });
    expect(screen.getByTestId("clear-entity-grid-drafts")).toBeEnabled(); fireEvent.click(screen.getByTestId("clear-entity-grid-drafts"));
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "" } }); expect(screen.getByTestId("review-cell-s-b-name")).toHaveTextContent("B");
  });
  it("discloses raw unsupported enum drafts when review is closed without enabling Queue", async () => {
    const model = await fixture(); render(<ModelTree {...props(model)} />); open(); await act(async () => {});
    fireEvent.click(screen.getByTestId("section-grid-review-disclosure")); fireEvent.doubleClick(screen.getByTestId("review-cell-s-a-type"));
    const input = within(table(true)).getByRole("combobox"); fireEvent.change(input, { target: { value: "TBD" } });
    fireEvent.click(within(table(true)).getByRole("button", { name: "Keep draft" })); expect(screen.getByTestId("queue-entity-grid-intents")).toBeDisabled();
    fireEvent.click(screen.getByTestId("section-grid-review-disclosure")); expect(screen.getByTestId("section-grid-review-disclosure")).toHaveTextContent("1 retained draft");
  });
  it.each(["value", "type", "unit", "generation"])("refuses stale %s without rebasing the captured Section edit", async (change) => {
    const model = await fixture(); const p = props(model); const view = render(<ModelTree {...p} projectSessionGeneration={0} />); open();
    await waitFor(() => expect(screen.getByTestId("table-cell-s-a-wall").parentElement).toHaveAttribute("aria-readonly", "false"));
    const input = editor("s-a", "wall"); fireEvent.change(input, { target: { value: "11" } }); const changed = structuredClone(model);
    if (change === "value") changed.sections![0].properties.wall_thickness!.value = 12;
    if (change === "type") (changed.sections![0].properties.wall_thickness as any).value = "10";
    if (change === "unit") changed.sections![0].properties.wall_thickness!.unit = "cm";
    view.rerender(<ModelTree {...p} model={changed} projectSessionGeneration={change === "generation" ? 1 : 0} />); await act(async () => {});
    if (change === "generation") expect(within(table()).queryByRole("textbox")).toBeNull();
    else { fireEvent.click(within(table()).getByRole("button", { name: "Apply" })); expect(input).toHaveValue("11"); expect(within(table()).getByRole("alert")).toBeInTheDocument(); }
    expect(p.onApplyCellIntent).not.toHaveBeenCalled();
  });
  it.each([undefined, "", false, ["mm"], { symbol: "mm" }])("does not invent a unit from malformed metadata %s", async (unit) => {
    const model = await fixture(); (model.sections![0].properties.wall_thickness as any).unit = unit; const p = props(model); render(<ModelTree {...p} />); open(); await act(async () => {});
    expect(screen.getByTestId("table-cell-s-a-wall").parentElement).toHaveAttribute("aria-readonly", "true");
    fireEvent.click(screen.getByTestId("section-grid-review-disclosure")); fireEvent.change(editor("s-a", "wall", true), { target: { value: "12" } }); fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
    expect(p.onQueueIntent.mock.calls[0][0].change).toMatchObject({ before: "10", unit: "", after: JSON.stringify({ value: 12, unit: "" }) });
    expect(p.onQueueIntent.mock.calls[0][0].validation.unit_validation).toBe("not_run");
  });
  it("keeps unchanged wall editing eligible while another OD conversion refreshes", async () => {
    const convert = displayQuantityService.convertDisplayQuantities; let hold = false; const pending: Array<() => void> = [];
    vi.spyOn(displayQuantityService, "convertDisplayQuantities").mockImplementation(async (items) => { const result = await convert(items); if (hold) await new Promise<void>((resolve) => pending.push(resolve)); return result; });
    const model = await fixture(); const p = props(model); const view = render(<ModelTree {...p} />); open();
    await waitFor(() => expect(screen.getByTestId("table-cell-s-a-wall").parentElement).toHaveAttribute("aria-readonly", "false"));
    const input = editor("s-a", "wall"); fireEvent.change(input, { target: { value: "11" } });
    hold = true; const changed = structuredClone(model); changed.sections![1].properties.outside_diameter!.value = 101; view.rerender(<ModelTree {...p} model={changed} />);
    await waitFor(() => expect(pending.length).toBeGreaterThan(0));
    try { fireEvent.keyDown(input, { key: "Tab" }); await waitFor(() => expect(p.onApplyCellIntent).toHaveBeenCalledTimes(1)); expect(p.onApplyCellIntent.mock.calls[0][0].change.unit).toBe("mm"); }
    finally { hold = false; await act(async () => pending.splice(0).forEach((resolve) => resolve())); }
  });
});

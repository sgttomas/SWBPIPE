import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { ModelTree } from "./ModelTree";
import * as displayQuantityService from "../../services/displayQuantityService";
import { loadPreviewModel } from "../../services/previewService";
import { DisplayUnitSelector, DisplayUnitsProvider } from "../display-units";
import type { EditorOperationIntent, PreviewModel } from "../../types";

async function fixture() {
  const model = await loadPreviewModel();
  const source = model.materials![0];
  model.materials = [
    { ...source, id: "m-a", label: "A", elastic_modulus: { value: 2, unit: "MPa" }, thermal_expansion_coefficient: { value: -0.00001, unit: "1/K" } },
    { ...source, id: "m-b", label: "B", elastic_modulus: { value: 1000, unit: "Pa" }, thermal_expansion_coefficient: { value: 0, unit: "1/degC" } }
  ];
  return model;
}
function open() { fireEvent.click(screen.getByTestId("layout-mode-grid")); fireEvent.click(screen.getByTestId("entity-grid-type-materials")); }
function table(review = false) { return screen.getByTestId(`material-engineering-table${review ? "-review" : ""}`); }
function editor(row: string, column: string, review = false) {
  fireEvent.doubleClick(screen.getByTestId(`${review ? "review" : "table"}-cell-${row}-${column}`));
  return within(table(review)).getByRole("textbox") as HTMLInputElement;
}
function order(review = false) { return within(table(review)).getAllByRole("rowheader").map((r) => r.textContent); }
function props(model: PreviewModel) { return { model, selection: { type: "material" as const, id: "m-a" }, onSelect: vi.fn(), onQueueIntent: vi.fn(), onApplyCellIntent: vi.fn(async (_intent: EditorOperationIntent) => ({ applied: true, messages: [] })) }; }

describe("Materials through shared direct/review table", () => {
  // jsdom has no layout. Supply a visible allocation for the persistent editor;
  // real clipping/alignment/resize is independently exercised in the browser.
  beforeEach(() => { vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 1000, 400)); });
  afterEach(() => { vi.restoreAllMocks(); });
  it("sorts equivalent quantities per column, captures exact before/unit, validates E/G and signed alpha, and skips numeric no-ops", async () => {
    const model = await fixture(); const p = props(model); render(<ModelTree {...p} />); open();
    await waitFor(() => expect(screen.getByTestId("table-cell-m-a-elastic").parentElement).toHaveAttribute("aria-readonly", "false"));
    expect(within(table()).getByRole("button", { name: "Sort Elastic" })).toHaveTextContent("per-row entered unit");
    fireEvent.click(within(table()).getByRole("button", { name: "Sort Elastic" })); expect(order()).toEqual(["m-b", "m-a"]);
    let input = editor("m-a", "elastic"); expect(input).toHaveAccessibleName("m-a Elastic [MPa]");
    fireEvent.change(input, { target: { value: "0" } }); fireEvent.click(within(table()).getByRole("button", { name: "Apply" }));
    expect(input).toHaveAttribute("aria-invalid", "true"); expect(p.onApplyCellIntent).not.toHaveBeenCalled();
    fireEvent.change(input, { target: { value: "2.00" } }); fireEvent.click(within(table()).getByRole("button", { name: "Apply" })); expect(p.onApplyCellIntent).not.toHaveBeenCalled();
    input = editor("m-a", "elastic"); fireEvent.change(input, { target: { value: "3.5" } }); fireEvent.click(within(table()).getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(p.onApplyCellIntent).toHaveBeenCalledTimes(1));
    expect(p.onApplyCellIntent.mock.calls[0][0]).toMatchObject({ target: { object_type: "Material", ref: "m-a" }, change: { field_path: "elastic_modulus.value", before: "2", after: JSON.stringify({ value: 3.5, unit: "MPa" }), unit: "MPa" } });
    await waitFor(() => expect(within(table()).queryByRole("textbox")).toBeNull());
    input = editor("m-a", "thermal"); fireEvent.change(input, { target: { value: "0" } }); fireEvent.click(within(table()).getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(p.onApplyCellIntent).toHaveBeenCalledTimes(2)); expect(p.onApplyCellIntent.mock.calls[1][0].change.after).toBe(JSON.stringify({ value: 0, unit: "1/K" }));
  });

  it("keeps an unchanged open Shear editor eligible while Elastic reconversion is pending", async () => {
    const realConvert = displayQuantityService.convertDisplayQuantities;
    let hold = false; const pending: Array<() => void> = [];
    vi.spyOn(displayQuantityService, "convertDisplayQuantities").mockImplementation(async (items) => {
      const result = await realConvert(items);
      if (hold) await new Promise<void>((resolve) => pending.push(resolve));
      return result;
    });
    const model = await fixture(); const p = props(model); const view = render(<ModelTree {...p} />); open();
    await waitFor(() => expect(screen.getByTestId("table-cell-m-a-shear").parentElement).toHaveAttribute("aria-readonly", "false"));
    fireEvent.click(within(table()).getByRole("button", { name: "Sort Elastic" })); expect(order()).toEqual(["m-b", "m-a"]);
    const input = editor("m-a", "shear"); fireEvent.change(input, { target: { value: "88" } });
    hold = true; const changed = structuredClone(model); changed.materials![0].elastic_modulus.value = 3;
    view.rerender(<ModelTree {...p} model={changed} />);
    await waitFor(() => expect(pending.length).toBeGreaterThan(0));
    expect(within(table()).getByRole("status")).toHaveTextContent("sort unavailable"); expect(order()).toEqual(["m-a", "m-b"]);
    expect(within(table()).getByRole("textbox")).toBe(input); expect(input).toHaveValue("88");
    try {
      fireEvent.click(within(table()).getByRole("button", { name: "Apply" }));
      await waitFor(() => expect(p.onApplyCellIntent).toHaveBeenCalledTimes(1));
      expect(p.onApplyCellIntent.mock.calls[0][0].change).toMatchObject({ field_path: "shear_modulus.value", before: String(model.materials![0].shear_modulus!.value), unit: model.materials![0].shear_modulus!.unit, after: JSON.stringify({ value: 88, unit: model.materials![0].shear_modulus!.unit }) });
    } finally { hold = false; await act(async () => pending.splice(0).forEach((resolve) => resolve())); }
  });

  it.each(["value", "type", "unit", "generation"])("keeps a changed %s editor basis ineligible during refresh and refuses stale Apply", async (change) => {
    const realConvert = displayQuantityService.convertDisplayQuantities;
    let hold = false; const pending: Array<() => void> = [];
    vi.spyOn(displayQuantityService, "convertDisplayQuantities").mockImplementation(async (items) => {
      const result = await realConvert(items); if (hold) await new Promise<void>((resolve) => pending.push(resolve)); return result;
    });
    const model = await fixture(); const p = props(model); const view = render(<ModelTree {...p} projectSessionGeneration={0} />); open();
    const shearCell = () => screen.getByTestId("table-cell-m-a-shear");
    await waitFor(() => expect(shearCell().parentElement).toHaveAttribute("aria-readonly", "false"));
    const input = editor("m-a", "shear"); fireEvent.change(input, { target: { value: "88" } });
    hold = true; const changed = structuredClone(model);
    if (change === "value") changed.materials![0].shear_modulus!.value = 4;
    if (change === "type") (changed.materials![0].shear_modulus as unknown as { value: unknown }).value = String(model.materials![0].shear_modulus!.value);
    if (change === "unit") changed.materials![0].shear_modulus!.unit = "MPa";
    view.rerender(<ModelTree {...p} model={changed} projectSessionGeneration={change === "generation" ? 1 : 0} />);
    await waitFor(() => expect(pending.length).toBeGreaterThan(0));
    try {
      if (change === "generation") { expect(within(table()).queryByRole("textbox")).toBeNull(); expect(shearCell().parentElement).toHaveAttribute("aria-readonly", "true"); }
      else {
        expect(within(table()).getByRole("textbox")).toBe(input); fireEvent.click(within(table()).getByRole("button", { name: "Apply" }));
        expect(within(table()).getByRole("alert")).toHaveTextContent("read-only"); expect(p.onApplyCellIntent).not.toHaveBeenCalled();
      }
    } finally { hold = false; await act(async () => pending.splice(0).forEach((resolve) => resolve())); }
    if (change !== "generation") {
      fireEvent.click(within(table()).getByRole("button", { name: "Apply" }));
      expect(p.onApplyCellIntent).not.toHaveBeenCalled();
      expect(within(table()).getByRole("alert")).toHaveTextContent(change === "type" ? "read-only" : "changed after editing began");
    }
  });

  it("keeps malformed bases unavailable directly and never invents units or metadata in review", async () => {
    const model = await fixture(); const material = model.materials![0] as unknown as Record<string, unknown>;
    material.elastic_modulus = { value: 2 }; material.shear_modulus = { value: 4, unit: "m" }; material.thermal_expansion_coefficient = { value: "TBD", unit: "1/K" }; material.provenance = { source: "structured" };
    const p = props(model); render(<ModelTree {...p} />); open();
    await act(async () => {});
    for (const column of ["elastic", "shear", "thermal", "provenance"]) { expect(screen.getByTestId(`table-cell-m-a-${column}`).parentElement).toHaveAttribute("aria-readonly", "true"); fireEvent.doubleClick(screen.getByTestId(`table-cell-m-a-${column}`)); expect(within(table()).queryByRole("textbox")).toBeNull(); }
    fireEvent.click(screen.getByTestId("material-grid-review-disclosure"));
    const input = editor("m-a", "elastic", true); fireEvent.change(input, { target: { value: " " } });
    fireEvent.click(within(table(true)).getByRole("button", { name: "Keep draft" })); expect(within(table(true)).getByRole("status")).toHaveTextContent("model unchanged");
    fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
    expect(p.onQueueIntent.mock.calls[0][0]).toMatchObject({ change: { before: "2", after: JSON.stringify({ value: "TBD", unit: "" }), unit: "" }, validation: { unit_validation: "not_run" } });
    expect(screen.getByTestId("review-cell-m-a-provenance").parentElement).toHaveAttribute("aria-readonly", "true");
  });

  it.each([undefined, "", "TBD", Infinity, NaN, "2"])("keeps nonnumeric source %s unavailable without quantity authoring", async (value) => {
    const model = await fixture(); (model.materials![0] as any).elastic_modulus = { value, unit: "MPa" };
    render(<ModelTree {...props(model)} />); open(); await act(async () => {});
    expect(screen.getByTestId("table-cell-m-a-elastic").parentElement).toHaveAttribute("aria-readonly", "true");
    fireEvent.doubleClick(screen.getByTestId("table-cell-m-a-elastic")); expect(within(table()).queryByRole("textbox")).toBeNull();
  });

  it.each([undefined, "", false, ["MPa"], { symbol: "MPa" }])("does not coerce malformed sibling unit %s into declared metadata", async (unit) => {
    const model = await fixture(); (model.materials![0] as any).elastic_modulus.unit = unit; const p = props(model);
    render(<ModelTree {...p} />); open(); await act(async () => {});
    expect(screen.getByTestId("table-cell-m-a-elastic").parentElement).toHaveAttribute("aria-readonly", "true");
    fireEvent.click(screen.getByTestId("material-grid-review-disclosure")); fireEvent.change(editor("m-a", "elastic", true), { target: { value: "3" } });
    fireEvent.click(screen.getByTestId("queue-entity-grid-intents")); expect(p.onQueueIntent.mock.calls[0][0].change.unit).toBe("");
    expect(p.onQueueIntent.mock.calls[0][0].validation.unit_validation).toBe("not_run");
  });

  it("retains raw review state across families/preferences and queues current basis only for visible submitted drafts", async () => {
    const model = await fixture(); const p = props(model); const view = render(<DisplayUnitsProvider><DisplayUnitSelector /><ModelTree {...p} projectSessionGeneration={1} /></DisplayUnitsProvider>); open();
    fireEvent.click(screen.getByTestId("material-grid-review-disclosure"));
    const input = editor("m-a", "elastic", true); fireEvent.change(input, { target: { value: "0x10" } });
    fireEvent.click(within(table(true)).getByRole("button", { name: "Keep draft" }));
    fireEvent.change(editor("m-b", "label", true), { target: { value: "retained" } }); fireEvent.click(within(table(true)).getByRole("button", { name: "Keep draft" }));
    fireEvent.change(screen.getByLabelText("Display units"), { target: { value: "US" } });
    fireEvent.click(screen.getByTestId("entity-grid-type-nodes")); fireEvent.click(screen.getByTestId("entity-grid-type-materials"));
    expect(screen.getByTestId("review-cell-m-a-elastic")).toHaveTextContent("0x10");
    const changed = structuredClone(model); changed.materials![0].elastic_modulus = { value: 7, unit: "MPa" };
    view.rerender(<DisplayUnitsProvider><DisplayUnitSelector /><ModelTree {...p} model={changed} projectSessionGeneration={1} /></DisplayUnitsProvider>);
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "m-a" } }); fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
    expect(p.onQueueIntent.mock.calls[0][0].change).toMatchObject({ before: "7", unit: "MPa", after: JSON.stringify({ value: 16, unit: "MPa" }) });
    expect(screen.getByTestId("review-cell-m-a-elastic")).toHaveTextContent("7");
    expect(screen.getByTestId("clear-entity-grid-drafts")).toBeEnabled(); fireEvent.click(screen.getByTestId("clear-entity-grid-drafts"));
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "" } }); expect(screen.getByTestId("review-cell-m-b-label")).toHaveTextContent("B");
  });

  it("keeps input identity/caret through staged sort invalidation and completion; invalid pinned rows suppress the whole sort", async () => {
    const model = await fixture(); const p = props(model); render(<ModelTree {...p} />); open(); fireEvent.click(screen.getByTestId("material-grid-review-disclosure"));
    await waitFor(() => expect(screen.getByTestId("table-cell-m-a-elastic").parentElement).toHaveAttribute("aria-readonly", "false"));
    fireEvent.click(within(table(true)).getByRole("button", { name: "Sort Elastic" })); await waitFor(() => expect(order(true)).toEqual(["m-b", "m-a"]));
    const input = editor("m-a", "elastic", true); fireEvent.change(input, { target: { value: "0.0001" } }); input.setSelectionRange(1, 1);
    expect(within(table(true)).getByRole("status")).toHaveTextContent("sort unavailable");
    await waitFor(() => expect(order(true)).toEqual(["m-a", "m-b"])); expect(within(table(true)).getByRole("textbox")).toBe(input); expect(input.selectionStart).toBe(1);
    fireEvent.change(input, { target: { value: "invalid" } }); fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "m-b" } });
    expect(within(table(true)).getByRole("status")).toHaveTextContent("sort unavailable"); expect(order(true)).toEqual(["m-b", "m-a"]);
    expect(within(table(true)).getByRole("button", { name: "Sort Elastic" }).parentElement).toHaveAttribute("aria-sort", "none");
  });
});

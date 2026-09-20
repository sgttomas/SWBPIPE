import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";
import { entityKey } from "../selectionState";
import { EngineeringTable, type TableApplyResult } from "./EngineeringTable";
import { nodeCoordinateColumns } from "./modelTableAdapter";
import type { CapturedCell, TableRow } from "./tableState";

const columns = nodeCoordinateColumns("m");
const rows = (count = 3): TableRow[] => Array.from({ length: count }, (_, i) => ({ key: entityKey({ type: "node", id: `n:${i}` }), label: `n:${i}`, cells: { x: { value: String(i), unit: "m" }, y: { value: "0", unit: "m" }, z: { value: "0", unit: "m" } } }));
function Harness({ count = 3, apply = async () => ({ applied: true, messages: [] }), generation = "p:1" }: { count?: number; apply?: (capture: CapturedCell, text: string) => Promise<TableApplyResult>; generation?: string }) {
  const [model, setModel] = useState(rows(count)); const [filter, setFilter] = useState("");
  return <><input aria-label="Filter" value={filter} onChange={(event) => setFilter(event.target.value)} /><button data-table-action="another-table">Outside</button><EngineeringTable label="Node coordinates" rows={model} columns={columns} generation={generation} density="comfortable" filter={filter} selectedKey={model[0].key} onSelect={() => {}} onApply={async (capture, text) => { const result = await apply(capture, text); if (result.applied) setModel((current) => current.map((row) => row.key === capture.rowKey ? { ...row, cells: { ...row.cells, [capture.columnKey]: { value: text, unit: capture.unit } } } : row)); return result; }} /></>;
}
function cell(row = 0, column = "x") { return screen.getByTestId(`table-cell-n:${row}-${column}`); }
function edit(row = 0, column = "x") { fireEvent.doubleClick(cell(row, column)); return screen.getByRole("textbox", { name: `n:${row} ${column.toUpperCase()} [m]` }); }

describe("EngineeringTable interaction ownership", () => {
  it("focuses on first click, edits on second, applies one captured cell and cancels without an operation", async () => {
    const apply = vi.fn(async () => ({ applied: true, messages: [] })); render(<Harness apply={apply} />);
    fireEvent.click(cell()); expect(screen.queryByRole("button", { name: "Apply" })).not.toBeInTheDocument();
    fireEvent.click(cell()); fireEvent.change(screen.getByRole("textbox", { name: "n:0 X [m]" }), { target: { value: "12.5" } });
    fireEvent.click(screen.getByRole("button", { name: "Apply" })); await waitFor(() => expect(cell()).toHaveTextContent("12.5"));
    expect(apply).toHaveBeenCalledTimes(1); expect(apply.mock.calls[0]).toEqual([expect.objectContaining({ rowKey: rows()[0].key, before: "0", unit: "m", generation: "p:1", columnKey: "x" }), "12.5"]);
    fireEvent.change(edit(), { target: { value: "99" } }); fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    expect(cell()).toHaveTextContent("12.5"); expect(apply).toHaveBeenCalledTimes(1);
  });
  it("keeps invalid text but reverts actual engine rejection with its diagnostic", async () => {
    const apply = vi.fn(async () => ({ applied: false, rejected: true, messages: ["ZERO_LENGTH: Element has no length"] })); render(<Harness apply={apply} />);
    const input = edit(); fireEvent.change(input, { target: { value: "not a length" } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    expect(input).toHaveValue("not a length"); expect(input).toHaveAttribute("aria-invalid", "true"); expect(apply).not.toHaveBeenCalled();
    fireEvent.change(input, { target: { value: "3" } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(input).toHaveValue("0")); expect(screen.getByRole("alert")).toHaveTextContent("Engine rejected: ZERO_LENGTH: Element has no length");
    fireEvent.click(screen.getByRole("button", { name: "Cancel" })); expect(cell()).toHaveTextContent("0");
  });
  it("uses file order for Enter under view sort, view order for Tab/arrows, and leaves caret/text undo keys alone", async () => {
    render(<Harness />); fireEvent.click(screen.getByRole("button", { name: "Sort X" })); fireEvent.click(screen.getByRole("button", { name: "Sort X" }));
    const input = edit(0); fireEvent.change(input, { target: { value: "4" } }); fireEvent.keyDown(input, { key: "Enter" });
    await waitFor(() => expect(cell(1)).toHaveFocus());
    fireEvent.keyDown(cell(1), { key: "Tab" }); expect(cell(1, "y")).toHaveFocus();
    fireEvent.keyDown(cell(1, "y"), { key: "7" }); const next = screen.getByRole("textbox", { name: "n:1 Y [m]" }); expect(next).toHaveValue("7");
    expect(fireEvent.keyDown(next, { key: "ArrowLeft" })).toBe(true); expect(fireEvent.keyDown(next, { key: "z", metaKey: true })).toBe(true);
    fireEvent.keyDown(next, { key: "Escape" }); expect(cell(1, "y")).toHaveTextContent("0"); expect(cell(1, "y")).toHaveFocus();
  });
  it("applies a valid edit on blur without stealing external focus", async () => {
    render(<Harness />); const input = edit(); fireEvent.change(input, { target: { value: "5" } });
    const outside = screen.getByRole("button", { name: "Outside" }); act(() => outside.focus());
    await waitFor(() => expect(cell()).toHaveTextContent("5")); expect(outside).toHaveFocus();
  });
  it("pins invalid drafts through virtual scrolling and filtering across the threshold", () => {
    render(<Harness count={160} />); const input = edit(); fireEvent.change(input, { target: { value: "bad" } });
    fireEvent.scroll(screen.getByTestId("engineering-table-rows"), { target: { scrollTop: 4000 } }); expect(input).toBeInTheDocument(); expect(input).toHaveValue("bad");
    fireEvent.change(screen.getByRole("textbox", { name: "Filter" }), { target: { value: "n:159" } });
    expect(screen.getByRole("textbox", { name: "n:0 X [m]" })).toHaveValue("bad"); expect(screen.getByText("Editing row retained outside the filter.")).toBeInTheDocument();
    fireEvent.change(screen.getByRole("textbox", { name: "Filter" }), { target: { value: "" } }); expect(screen.getByRole("textbox", { name: "n:0 X [m]" })).toHaveValue("bad");
  });
  it("ignores an old completion after a same-ID project replacement and a newer edit", async () => {
    let complete!: (result: TableApplyResult) => void;
    const apply = vi.fn(() => new Promise<TableApplyResult>((resolve) => { complete = resolve; }));
    const view = render(<Harness apply={apply} />); fireEvent.change(edit(), { target: { value: "8" } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    view.rerender(<Harness apply={apply} generation="p:2" />); fireEvent.change(edit(1), { target: { value: "new" } });
    await act(async () => complete({ applied: false, rejected: true, messages: ["Old rejection"] }));
    expect(screen.getByRole("textbox", { name: "n:1 X [m]" })).toHaveValue("new"); expect(screen.queryByText("Old rejection")).not.toBeInTheDocument();
  });
  it("deduplicates blur and Apply while preserving the original target on a next-cell click", async () => {
    let complete!: (result: TableApplyResult) => void;
    const apply = vi.fn((_capture: CapturedCell, _text: string) => new Promise<TableApplyResult>((resolve) => { complete = resolve; }));
    render(<Harness apply={apply} />);
    const input = edit(); fireEvent.change(input, { target: { value: "6" } });
    fireEvent.click(cell(1, "y"));
    fireEvent.blur(input); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    expect(apply).toHaveBeenCalledTimes(1); expect(apply.mock.calls[0][0]).toMatchObject({ rowKey: rows()[0].key, columnKey: "x", before: "0" });
    const outside = screen.getByRole("button", { name: "Outside" }); act(() => outside.focus());
    await act(async () => complete({ applied: true, messages: [] }));
    expect(cell()).toHaveTextContent("6"); expect(cell(1, "y")).toHaveTextContent("0"); expect(outside).toHaveFocus();
  });
  it("refuses a before-value change without silently rebasing a draft", async () => {
    const apply = vi.fn(async () => ({ applied: true, messages: [] }));
    const initial = rows();
    const props = { label: "Node coordinates", rows: initial, columns, generation: "p:1", density: "comfortable" as const, filter: "", selectedKey: initial[0].key, onSelect: vi.fn(), onApply: apply };
    const view = render(<EngineeringTable {...props} />); fireEvent.change(edit(), { target: { value: "9" } });
    const changed = rows(); changed[0] = { ...changed[0], cells: { ...changed[0].cells, x: { value: "2", unit: "m" } } };
    view.rerender(<EngineeringTable {...props} rows={changed} />); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    expect(apply).not.toHaveBeenCalled(); expect(screen.getByRole("alert")).toHaveTextContent("changed after editing began"); expect(screen.getByRole("textbox")).toHaveValue("9");
    fireEvent.click(screen.getByRole("button", { name: "Cancel" })); expect(cell()).toHaveTextContent("2");
  });

  it("lets Tab leave either table boundary and keeps pending edits read-only", async () => {
    let complete!: (result: TableApplyResult) => void;
    const apply = vi.fn((_capture: CapturedCell, _text: string) => new Promise<TableApplyResult>((resolve) => { complete = resolve; }));
    render(<Harness apply={apply} />);
    expect(fireEvent.keyDown(cell(), { key: "Tab", shiftKey: true })).toBe(true);
    expect(fireEvent.keyDown(cell(2, "z"), { key: "Tab" })).toBe(true);
    const input = edit(); fireEvent.change(input, { target: { value: "4" } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    fireEvent.change(input, { target: { value: "99" } }); fireEvent.blur(input);
    expect(apply).toHaveBeenCalledTimes(1); expect(input).toHaveValue("4");
    await act(async () => complete({ applied: true, messages: [] })); expect(cell()).toHaveTextContent("4");
  });

  it("marks a removed target as a retained draft and Cancel focuses a live row", () => {
    const initial = rows(); const apply = vi.fn(async () => ({ applied: true, messages: [] }));
    const props = { label: "Node coordinates", rows: initial, columns, generation: "p:1", density: "comfortable" as const, filter: "", selectedKey: initial[0].key, onSelect: vi.fn(), onApply: apply };
    const view = render(<EngineeringTable {...props} />); fireEvent.change(edit(), { target: { value: "8" } });
    view.rerender(<EngineeringTable {...props} rows={initial.slice(1)} />);
    expect(screen.getByRole("alert")).toHaveTextContent("edited row was removed"); expect(screen.getByRole("button", { name: "n:0 (removed edit)" })).toBeDisabled();
    fireEvent.click(screen.getByRole("button", { name: "Apply" })); expect(apply).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Cancel" })); expect(screen.queryByText("n:0 (removed edit)")).not.toBeInTheDocument(); expect(cell(1)).toHaveFocus();
  });

  it("publishes keyboard focus entry without turning the first pointer click into editing", () => {
    const initial = rows(); const select = vi.fn();
    render(<EngineeringTable label="Coordinates" rows={initial} columns={columns} generation="p:1" density="comfortable" filter="" selectedKey={initial[2].key} onSelect={select} onApply={vi.fn()} />);
    act(() => cell().focus()); expect(select).toHaveBeenLastCalledWith(initial[0].key);
    fireEvent.pointerDown(cell(1)); act(() => cell(1).focus()); fireEvent.click(cell(1));
    expect(screen.queryByRole("textbox")).not.toBeInTheDocument(); expect(select).toHaveBeenLastCalledWith(initial[1].key);
  });

  it("keeps invalid A when B receives a click then Enter or a printable character", () => {
    render(<Harness />); const input = edit(); fireEvent.change(input, { target: { value: "invalid A" } });
    fireEvent.click(cell(1)); fireEvent.keyDown(cell(1), { key: "Enter" });
    expect(screen.getByRole("textbox", { name: "n:0 X [m]" })).toHaveValue("invalid A");
    fireEvent.keyDown(cell(1), { key: "8" }); expect(screen.getByRole("textbox", { name: "n:0 X [m]" })).toHaveValue("invalid A");
    expect(screen.queryByRole("textbox", { name: "n:1 X [m]" })).not.toBeInTheDocument();
  });
  it("does not publish deferred navigation selection after external focus and selection win", async () => {
    let complete!: (result: TableApplyResult) => void; const apply = vi.fn(() => new Promise<TableApplyResult>((resolve) => { complete = resolve; }));
    const initial = rows(); const select = vi.fn();
    const props = { label: "Coordinates", rows: initial, columns, generation: "p:1", density: "comfortable" as const, filter: "", selectedKey: initial[0].key, onSelect: select, onApply: apply };
    const view = render(<><button>External C</button><EngineeringTable {...props} /></>);
    const input = edit(); fireEvent.change(input, { target: { value: "4" } }); fireEvent.keyDown(input, { key: "Enter" });
    view.rerender(<><button>External C</button><EngineeringTable {...props} selectedKey={initial[2].key} /></>);
    const outside = screen.getByRole("button", { name: "External C" }); act(() => outside.focus()); select.mockClear();
    await act(async () => complete({ applied: true, messages: [] }));
    expect(outside).toHaveFocus(); expect(select).not.toHaveBeenCalled();
  });

  it("clears completed-operation feedback when a new project generation replaces the model", async () => {
    const apply = vi.fn(async () => ({ applied: true, messages: ["OLD_PROJECT_WARNING: retained source diagnostic"] }));
    const view = render(<Harness apply={apply} generation="p:1" />);
    fireEvent.change(edit(), { target: { value: "4" } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(screen.getByRole("status")).toHaveTextContent("OLD_PROJECT_WARNING"));
    expect(screen.queryByRole("button", { name: "Cancel" })).not.toBeInTheDocument();
    view.rerender(<Harness apply={apply} generation="p:2" />);
    expect(screen.queryByText(/OLD_PROJECT_WARNING/)).not.toBeInTheDocument();
    expect(screen.queryByText(/previous cell edit was cancelled/)).not.toBeInTheDocument();
  });

});

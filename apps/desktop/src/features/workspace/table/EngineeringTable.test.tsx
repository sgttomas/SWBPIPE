import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";
import { entityKey } from "../selectionState";
import { EngineeringTable, useTableBodyHeight, type TableApplyResult } from "./EngineeringTable";
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

  it.each(["no-op", "accepted"])("exits the editing forward boundary after %s Apply to a persistent visible footer", async (mode) => {
    const apply = vi.fn(async () => ({ applied: true, messages: [] })); render(<Harness apply={apply} />);
    const input = edit(2, "z"); if (mode === "accepted") fireEvent.change(input, { target: { value: "3" } });
    fireEvent.keyDown(input, { key: "Tab" });
    await waitFor(() => expect(document.querySelector(".engineering-table-footer")).toHaveFocus());
    expect(screen.queryByRole("textbox", { name: "n:2 Z [m]" })).not.toBeInTheDocument();
    expect(cell(2, "z")).not.toHaveFocus(); expect(apply).toHaveBeenCalledTimes(mode === "accepted" ? 1 : 0);
  });
  it("exits a no-op backward editing boundary to the row header", async () => {
    render(<Harness />); fireEvent.keyDown(edit(), { key: "Tab", shiftKey: true });
    await waitFor(() => expect(screen.getByRole("button", { name: "n:0" })).toHaveFocus());
  });
  it("retains invalid and engine-rejected boundary edits instead of exiting", async () => {
    const apply = vi.fn(async () => ({ applied: false, rejected: true, messages: ["BOUNDARY_REJECTION: source diagnostic"] })); render(<Harness apply={apply} />);
    const input = edit(2, "z"); fireEvent.change(input, { target: { value: "invalid" } }); fireEvent.keyDown(input, { key: "Tab" });
    expect(input).toHaveFocus(); expect(input).toHaveValue("invalid"); expect(apply).not.toHaveBeenCalled();
    fireEvent.change(input, { target: { value: "3" } }); fireEvent.keyDown(input, { key: "Tab" });
    await waitFor(() => expect(input).toHaveValue("0")); expect(input).toHaveFocus(); expect(screen.getByRole("alert")).toHaveTextContent("BOUNDARY_REJECTION");
  });
  it("does not steal external focus when a pending boundary Apply completes", async () => {
    let complete!: (result: TableApplyResult) => void; const apply = vi.fn(() => new Promise<TableApplyResult>((resolve) => { complete = resolve; }));
    render(<Harness apply={apply} />); const input = edit(2, "z"); fireEvent.change(input, { target: { value: "3" } }); fireEvent.keyDown(input, { key: "Tab" });
    const outside = screen.getByRole("button", { name: "Outside" }); act(() => outside.focus());
    await act(async () => complete({ applied: true, messages: [] })); expect(outside).toHaveFocus(); expect(cell(2, "z")).toHaveTextContent("3");
  });

  it.each([false, true])("preserves newer selection without DOM focus movement, including ABA=%s", async (aba) => {
    let complete!: (result: TableApplyResult) => void; const apply = vi.fn(() => new Promise<TableApplyResult>((resolve) => { complete = resolve; }));
    const initial = rows(); const select = vi.fn();
    const props = { label: "Coordinates", rows: initial, columns, generation: "p:1", density: "comfortable" as const, filter: "", selectedKey: initial[0].key, onSelect: select, onApply: apply };
    const view = render(<EngineeringTable {...props} />);
    const input = edit(); fireEvent.change(input, { target: { value: "4" } }); fireEvent.keyDown(input, { key: "Enter" });
    view.rerender(<EngineeringTable {...props} selectedKey={initial[2].key} />);
    if (aba) view.rerender(<EngineeringTable {...props} selectedKey={initial[0].key} />);
    expect(input).toHaveFocus(); select.mockClear();
    await act(async () => complete({ applied: true, messages: [] })); expect(select).not.toHaveBeenCalled(); expect(cell(1)).not.toHaveFocus();
  });
  it.each(["filter", "deletion"])("restores a live roving entry after %s without stealing external focus", (change) => {
    const initial = rows(); const select = vi.fn();
    const props = { label: "Coordinates", rows: initial, columns, generation: "p:1", density: "comfortable" as const, filter: "", selectedKey: initial[0].key, onSelect: select, onApply: vi.fn() };
    const view = render(<><input aria-label="Outside filter" /><EngineeringTable {...props} /></>);
    act(() => cell().focus()); const outside = screen.getByRole("textbox", { name: "Outside filter" }); act(() => outside.focus()); select.mockClear();
    view.rerender(<><input aria-label="Outside filter" /><EngineeringTable {...props} rows={change === "deletion" ? initial.slice(1) : initial} filter={change === "filter" ? "n:1" : ""} /></>);
    expect(cell(1)).toHaveAttribute("tabindex", "0"); expect(outside).toHaveFocus(); expect(select).not.toHaveBeenCalled();
    act(() => cell(1).focus()); expect(select).toHaveBeenLastCalledWith(initial[1].key);
  });

  it("owns valid pointer Cancel before a browser default blur with null relatedTarget", async () => {
    const apply = vi.fn(async () => ({ applied: true, messages: [] })); render(<Harness apply={apply} />);
    const input = edit(); fireEvent.change(input, { target: { value: "6.2" } });
    const cancel = screen.getByRole("button", { name: "Cancel" });
    // Native evidence has no DOM telemetry. Exercise the inferred default-action ordering:
    // pointer-down may blur the input to body before the eventual button click.
    if (fireEvent.pointerDown(cancel)) fireEvent.blur(input, { relatedTarget: null });
    expect(input).toHaveValue("6.2");
    fireEvent.pointerUp(cancel); fireEvent.click(cancel);
    await waitFor(() => expect(cell()).toHaveTextContent("0")); expect(apply).not.toHaveBeenCalled();
  });
  it("does not activate Apply on pointer-down and applies exactly once on the eventual click", async () => {
    const apply = vi.fn(async () => ({ applied: true, messages: [] })); render(<Harness apply={apply} />);
    const input = edit(); fireEvent.change(input, { target: { value: "4.6" } }); const button = screen.getByRole("button", { name: "Apply" });
    if (fireEvent.pointerDown(button)) fireEvent.blur(input, { relatedTarget: null });
    expect(apply).not.toHaveBeenCalled(); expect(input).toHaveValue("4.6");
    fireEvent.pointerUp(button); fireEvent.click(button); fireEvent.click(button);
    await waitFor(() => expect(cell()).toHaveTextContent("4.6")); expect(apply).toHaveBeenCalledTimes(1);
  });
  it.each(["Apply", "Cancel"].flatMap((name) => ["drag-away", "pointercancel"].map((abandon) => ({ name, abandon }))))("abandons $name via $abandon without leaving a guard against ordinary blur Apply", async ({ name, abandon }) => {
    const apply = vi.fn(async () => ({ applied: true, messages: [] })); render(<Harness apply={apply} />);
    const input = edit(); fireEvent.change(input, { target: { value: "5.1" } }); const button = screen.getByRole("button", { name });
    if (fireEvent.pointerDown(button)) fireEvent.blur(input, { relatedTarget: null });
    fireEvent.pointerLeave(button);
    if (abandon === "pointercancel") fireEvent.pointerCancel(button);
    else fireEvent.pointerUp(screen.getByRole("button", { name: "Outside" }));
    expect(apply).not.toHaveBeenCalled(); expect(input).toHaveValue("5.1");
    const outside = screen.getByRole("button", { name: "Outside" }); act(() => outside.focus());
    await waitFor(() => expect(cell()).toHaveTextContent("5.1")); expect(apply).toHaveBeenCalledTimes(1); expect(outside).toHaveFocus();
  });

  it.each(["Apply", "Cancel"])("preserves keyboard footer %s activation", async (name) => {
    const apply = vi.fn(async () => ({ applied: true, messages: [] })); render(<Harness apply={apply} />);
    const input = edit(); fireEvent.change(input, { target: { value: "2.5" } }); const button = screen.getByRole("button", { name });
    act(() => button.focus()); expect(apply).not.toHaveBeenCalled();
    // Keyboard-produced click: no pointer-down path and no pointer click count.
    fireEvent.click(button, { detail: 0 });
    await waitFor(() => expect(cell()).toHaveTextContent(name === "Apply" ? "2.5" : "0"));
    expect(apply).toHaveBeenCalledTimes(name === "Apply" ? 1 : 0);
  });

  it("measures only a finite visible body slot and retains its last positive height while hidden", () => {
    const callbacks: Array<() => void> = [];
    const original = globalThis.ResizeObserver;
    globalThis.ResizeObserver = class { constructor(callback: () => void) { callbacks.push(callback); } observe() {} disconnect() {} unobserve() {} } as unknown as typeof ResizeObserver;
    function Slot({ active }: { active: boolean }) { const slot = useTableBodyHeight(true, active); return <div hidden={!active}><div ref={slot.ref} data-testid="allocated-slot" /><output>{slot.height}</output></div>; }
    try {
      const view = render(<Slot active />); const slot = screen.getByTestId("allocated-slot"); let available = 120;
      Object.defineProperty(slot, "clientHeight", { get: () => available });
      Object.defineProperty(slot, "clientWidth", { get: () => 240 });
      act(() => callbacks.at(-1)!()); expect(screen.getByText("120")).toBeInTheDocument();
      view.rerender(<Slot active={false} />); available = 0; act(() => callbacks.at(-1)!()); expect(screen.getByText("120")).toBeInTheDocument();
      available = 74; view.rerender(<Slot active />); expect(screen.getByText("74")).toBeInTheDocument();
      available = 52; act(() => callbacks.at(-1)!()); expect(screen.getByText("52")).toBeInTheDocument();
    } finally { globalThis.ResizeObserver = original; }
  });

  it("rebinds allocation observation when a retained review swaps small and virtual body elements", () => {
    const callbacks: Array<() => void> = []; const original = globalThis.ResizeObserver;
    globalThis.ResizeObserver = class { constructor(callback: () => void) { callbacks.push(callback); } observe() {} disconnect() {} unobserve() {} } as unknown as typeof ResizeObserver;
    const heights = vi.spyOn(HTMLElement.prototype, "clientHeight", "get").mockImplementation(function (this: HTMLElement) { return Number(this.dataset.height ?? 0); });
    const widths = vi.spyOn(HTMLElement.prototype, "clientWidth", "get").mockReturnValue(240);
    function Slot({ show, height }: { show: boolean; height: number }) { const body = useTableBodyHeight(true, true); return <><output>{body.height}</output>{show ? <div ref={body.ref} data-height={height} /> : null}</>; }
    try {
      const view = render(<Slot show={false} height={123} />); view.rerender(<Slot show height={123} />); expect(screen.getByText("123")).toBeInTheDocument();
      const old = callbacks.at(-1)!; view.rerender(<Slot show={false} height={123} />); view.rerender(<Slot show height={87} />); expect(screen.getByText("87")).toBeInTheDocument();
      act(() => old()); expect(screen.getByText("87")).toBeInTheDocument();
      view.rerender(<Slot show height={0} />); act(() => callbacks.at(-1)!()); expect(screen.getByText("0")).toBeInTheDocument();
    } finally { heights.mockRestore(); widths.mockRestore(); globalThis.ResizeObserver = original; }
  });

  it("records positive size changes under an inert page and reveals with the current height", () => {
    const callbacks: Array<() => void> = []; const original = globalThis.ResizeObserver;
    globalThis.ResizeObserver = class { constructor(callback: () => void) { callbacks.push(callback); } observe() {} disconnect() {} unobserve() {} } as unknown as typeof ResizeObserver;
    const heights = vi.spyOn(HTMLElement.prototype, "clientHeight", "get").mockImplementation(function (this: HTMLElement) { return Number(this.dataset.height ?? 0); });
    const widths = vi.spyOn(HTMLElement.prototype, "clientWidth", "get").mockReturnValue(240);
    function Slot({ inactive, height }: { inactive: boolean; height: number }) { const body = useTableBodyHeight(true, true); return <div inert={inactive}><output>{body.height}</output><div ref={body.ref} data-height={height} /></div>; }
    try {
      const view = render(<Slot inactive={false} height={120} />); expect(screen.getByText("120")).toBeInTheDocument();
      view.rerender(<Slot inactive height={73} />); act(() => callbacks.at(-1)!()); expect(screen.getByText("73")).toBeInTheDocument();
      view.rerender(<Slot inactive={false} height={73} />); expect(screen.getByText("73")).toBeInTheDocument();
    } finally { heights.mockRestore(); widths.mockRestore(); globalThis.ResizeObserver = original; }
  });

  it("reports a visible zero-height allocation instead of hiding a stale viewport behind overflow", () => {
    const heights = vi.spyOn(HTMLElement.prototype, "clientHeight", "get").mockReturnValue(0);
    const widths = vi.spyOn(HTMLElement.prototype, "clientWidth", "get").mockReturnValue(240);
    try {
      const initial = rows(); const apply = vi.fn();
      render(<EngineeringTable bounded label="Coordinates" rows={initial} columns={columns} generation="p:1" density="comfortable" filter="" selectedKey={initial[0].key} onSelect={vi.fn()} onApply={apply} />);
      expect(screen.getByRole("alert")).toHaveTextContent("No space is available for table rows");
      expect(screen.getByTestId("engineering-table-rows")).toHaveStyle({ height: "0px" }); expect(apply).not.toHaveBeenCalled();
    } finally { heights.mockRestore(); widths.mockRestore(); }
  });

});

it("recovers focus after a later canonical label publication filters the owned row out without selecting a fallback", async () => {
  const select = vi.fn(); const apply = vi.fn(async () => ({ applied: true, messages: [] }));
  const row = { key: rows()[0].key, label: "n:0", searchText: "original", cells: { label: { value: "original", unit: "none" } } };
  const props = { label: "Nodes", rows: [row], columns: [{ key: "label", label: "Label", unit: "", kind: "text" as const }], generation: "p", density: "comfortable" as const, filter: "original", selectedKey: row.key, onSelect: select, onApply: apply };
  const view = render(<><button>Outside</button><EngineeringTable {...props} /></>);
  fireEvent.doubleClick(cell(0, "label")); const input = screen.getByRole("textbox", { name: "n:0 Label" });
  fireEvent.change(input, { target: { value: "renamed" } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
  await waitFor(() => expect(cell(0, "label")).toHaveFocus()); select.mockClear();
  view.rerender(<><button>Outside</button><EngineeringTable {...props} rows={[{ ...row, searchText: "renamed", cells: { label: { value: "renamed", unit: "none" } } }]} /></>);
  expect(screen.getByRole("group", { name: "Nodes footer" })).toHaveFocus(); expect(select).not.toHaveBeenCalled();
});

it("keeps text rejection local and ignores a late text completion after same-ID generation replacement", async () => {
  let complete!: (result: TableApplyResult) => void;
  const apply = vi.fn(() => new Promise<TableApplyResult>((resolve) => { complete = resolve; }));
  const row = { key: rows()[0].key, label: "n:0", cells: { label: { value: "Original", unit: "none" } } };
  const props = { label: "Nodes", rows: [row], columns: [{ key: "label", label: "Label", unit: "", kind: "text" as const }], generation: "p:1", density: "comfortable" as const, filter: "", selectedKey: row.key, onSelect: vi.fn(), onApply: apply };
  const view = render(<EngineeringTable {...props} />);
  fireEvent.doubleClick(cell(0, "label")); fireEvent.change(screen.getByRole("textbox"), { target: { value: "Rejected" } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
  await act(async () => complete({ applied: false, rejected: true, messages: ["TEXT_REJECTED"] }));
  expect(screen.getByRole("textbox")).toHaveValue("Original"); expect(screen.getByRole("alert")).toHaveTextContent("Engine rejected: TEXT_REJECTED");
  fireEvent.change(screen.getByRole("textbox"), { target: { value: "Pending" } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
  view.rerender(<EngineeringTable {...props} generation="p:2" />);
  fireEvent.doubleClick(cell(0, "label")); fireEvent.change(screen.getByRole("textbox"), { target: { value: "New draft" } });
  await act(async () => complete({ applied: true, messages: ["Old text accepted"] }));
  expect(screen.getByRole("textbox")).toHaveValue("New draft"); expect(screen.queryByText("Old text accepted")).not.toBeInTheDocument();
});

it.each(["direct", "review"] as const)("initializes %s numeric/text selection once per edit without resetting later caret placement", (policy) => {
  for (const field of ["x", "label"] as const) {
    const row = { key: rows()[0].key, label: "n:0", cells: { [field]: { value: "1234", unit: field === "x" ? "m" : "none" } } };
    const shared = { label: "Initial selection", rows: [row], columns: [{ key: field, label: field === "x" ? "X" : "Label", unit: field === "x" ? "m" : "", kind: field === "x" ? "quantity" as const : "text" as const }], generation: "p", density: "comfortable" as const, filter: "", selectedKey: row.key, onSelect: vi.fn() };
    const view = render(policy === "direct"
      ? <EngineeringTable {...shared} onApply={vi.fn()} />
      : <EngineeringTable {...shared} policy="review" resetEditsKey={0} onDraftChange={vi.fn()} onKeepDraft={() => ({ retained: true, messages: [] })} />);
    const target = () => screen.getByTestId(`${policy === "review" ? "review" : "table"}-cell-n:0-${field}`);
    fireEvent.keyDown(target(), { key: "7" });
    let input = screen.getByRole("textbox") as HTMLInputElement;
    expect(input.value).toBe("7"); expect([input.selectionStart, input.selectionEnd]).toEqual([1, 1]);
    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    fireEvent.keyDown(target(), { key: "Enter" }); input = screen.getByRole("textbox") as HTMLInputElement;
    expect([input.selectionStart, input.selectionEnd]).toEqual([0, 4]);
    input.setSelectionRange(2, 2);
    act(() => screen.getByRole("button", { name: policy === "review" ? "Keep draft" : "Apply" }).focus());
    act(() => input.focus()); expect([input.selectionStart, input.selectionEnd]).toEqual([2, 2]);
    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    fireEvent.doubleClick(target()); input = screen.getByRole("textbox") as HTMLInputElement;
    expect([input.selectionStart, input.selectionEnd]).toEqual([0, 4]);
    view.unmount();
  }
});

it("does not advertise cached projected sort keys from a removed pinned editor row", () => {
  const source = rows(2).map((row, index) => ({ ...row, cells: { x: { value: String(index), unit: "Pa", sortValue: index } } }));
  const projected = [{ key: "x", label: "X", unit: "per row", kind: "quantity" as const, projectedSort: true }];
  const props = { label: "Projection", rows: source, columns: projected, generation: "p", density: "comfortable" as const, filter: "", selectedKey: source[0].key, onSelect: vi.fn(), onApply: vi.fn() };
  const view = render(<EngineeringTable {...props} />); fireEvent.click(screen.getByRole("button", { name: "Sort X" }));
  fireEvent.doubleClick(screen.getByTestId("table-cell-n:0-x"));
  view.rerender(<EngineeringTable {...props} rows={source.slice(1)} />);
  expect(screen.getByRole("status")).toHaveTextContent("sort unavailable"); expect(screen.getByRole("button", { name: "Sort X" }).parentElement).toHaveAttribute("aria-sort", "none");
  expect(screen.getByRole("textbox")).toHaveValue("0");
});

describe("opt-in typed enum on the common input", () => {
  const enumColumns = [{ key: "type", label: "Type", kind: "text" as const, unit: "", options: ["pipe"], validate: (value: string) => value.trim() === "pipe" ? undefined : "Choose or enter pipe." }];
  const enumRows = (source = "unsupported"): TableRow[] => [{ key: entityKey({ type: "section", id: "s" }), label: "s", cells: { type: { value: source, enumSource: source, unit: "" } } }];
  const base = { label: "Sections", rows: enumRows(), columns: enumColumns, generation: "p:1", density: "comfortable" as const, filter: "", selectedKey: enumRows()[0].key, onSelect: vi.fn() };
  function begin() { fireEvent.doubleClick(screen.getByTestId("table-cell-s-type")); const input = screen.getByRole("combobox"); fireEvent.click(input); return input; }
  it.each(["Enter", "Tab"])("completes a nonempty prefix on %s exactly once", async (key) => {
    const apply = vi.fn(async (_captured: CapturedCell, _text: string) => ({ applied: true, messages: [] })); render(<EngineeringTable {...base} onApply={apply} />);
    const input = begin(); fireEvent.change(input, { target: { value: "pi" } }); fireEvent.keyDown(input, { key });
    await waitFor(() => expect(apply).toHaveBeenCalledTimes(1)); expect(apply.mock.calls[0]).toEqual([expect.objectContaining({ before: "unsupported" }), "pipe"]);
  });
  it("preserves unsupported source through navigation and first Escape, then cancels on second Escape", () => {
    const apply = vi.fn(); render(<EngineeringTable {...base} onApply={apply} />); const input = begin();
    fireEvent.keyDown(input, { key: "ArrowDown" }); expect(input).toHaveValue("unsupported");
    fireEvent.keyDown(input, { key: "Escape" }); expect(screen.queryByRole("listbox")).toBeNull(); expect(input).toHaveValue("unsupported");
    fireEvent.keyDown(input, { key: "Escape" }); expect(screen.queryByRole("combobox")).toBeNull(); expect(apply).not.toHaveBeenCalled();
  });
  it("does not silently complete a prefix-shaped unsupported source on Enter", () => {
    const apply = vi.fn(); render(<EngineeringTable {...base} rows={enumRows("p")} onApply={apply} />); const input = begin();
    fireEvent.keyDown(input, { key: "Enter" }); expect(input).toHaveValue("p"); expect(apply).not.toHaveBeenCalled();
    expect(screen.getByRole("alert")).toHaveTextContent("Choose or enter pipe");
  });
  it("pointer choice only changes buffer; pointer cancellation/drag-away never chooses", async () => {
    const apply = vi.fn(async (_captured: CapturedCell, _text: string) => ({ applied: true, messages: [] })); render(<EngineeringTable {...base} onApply={apply} />); const input = begin();
    let option = screen.getByRole("option", { name: "pipe" }); fireEvent.pointerDown(option); expect(input).toHaveValue("unsupported");
    fireEvent.pointerCancel(option); fireEvent.click(option, { detail: 1 }); expect(input).toHaveValue("unsupported");
    fireEvent.pointerDown(option); fireEvent.pointerLeave(option); fireEvent.click(option, { detail: 1 }); expect(input).toHaveValue("unsupported");
    fireEvent.pointerDown(option); fireEvent.click(option, { detail: 1 }); expect(input).toHaveValue("pipe"); expect(input).toHaveFocus(); expect(apply).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Apply" })); await waitFor(() => expect(apply).toHaveBeenCalledTimes(1));
  });
  it("does not complete empty input or a prefix on blur; direct invalid text stays while raw review text is kept", async () => {
    const apply = vi.fn(); const view = render(<EngineeringTable {...base} onApply={apply} />); let input = begin();
    fireEvent.change(input, { target: { value: "" } }); fireEvent.keyDown(input, { key: "Enter" }); expect(input).toHaveValue(""); expect(screen.getByRole("alert")).toHaveTextContent("Choose or enter pipe");
    fireEvent.change(input, { target: { value: "pi" } }); fireEvent.blur(input); expect(input).toHaveValue("pi"); expect(apply).not.toHaveBeenCalled(); view.unmount();
    const keep = vi.fn(() => ({ retained: true, messages: ["Draft retained; model unchanged."] }));
    render(<EngineeringTable {...base} policy="review" resetEditsKey={0} onDraftChange={vi.fn()} onKeepDraft={keep} />);
    fireEvent.doubleClick(screen.getByTestId("review-cell-s-type")); input = screen.getByRole("combobox"); fireEvent.change(input, { target: { value: "TBD" } }); fireEvent.click(screen.getByRole("button", { name: "Keep draft" }));
    expect(keep).toHaveBeenCalledWith(expect.anything(), "TBD");
  });
  it("closes the body popup on hidden/inert/detached ancestry and external focus without stealing focus", async () => {
    const apply = vi.fn(); const view = render(<div data-testid="enum-owner"><button>External enum control</button><EngineeringTable {...base} onApply={apply} /></div>);
    const input = begin(); const owner = screen.getByTestId("enum-owner");
    act(() => { owner.hidden = true; }); await waitFor(() => expect(screen.queryByRole("listbox")).toBeNull()); expect(input).toHaveValue("unsupported");
    act(() => { owner.hidden = false; }); fireEvent.click(input); expect(screen.getByRole("listbox")).toBeInTheDocument();
    act(() => { owner.setAttribute("inert", ""); }); await waitFor(() => expect(screen.queryByRole("listbox")).toBeNull());
    act(() => { owner.removeAttribute("inert"); }); fireEvent.click(input);
    const parent = owner.parentElement!; act(() => owner.remove()); await waitFor(() => expect(screen.queryByRole("listbox")).toBeNull()); act(() => parent.appendChild(owner));
    act(() => input.focus()); fireEvent.click(input); const external = screen.getByRole("button", { name: "External enum control" }); act(() => external.focus());
    expect(screen.queryByRole("listbox")).toBeNull(); expect(external).toHaveFocus(); expect(input).toHaveValue("unsupported"); expect(apply).not.toHaveBeenCalled(); view.unmount();
  });
  it("closes options on source/catalog/active change without replacing input or changing draft", () => {
    const apply = vi.fn(); const view = render(<EngineeringTable {...base} onApply={apply} />); const input = begin();
    fireEvent.change(input, { target: { value: "pi" } }); view.rerender(<EngineeringTable {...base} rows={enumRows("changed")} onApply={apply} />);
    expect(screen.queryByRole("listbox")).toBeNull(); expect(screen.getByRole("combobox")).toBe(input); expect(input).toHaveValue("pi");
    fireEvent.click(input); view.rerender(<EngineeringTable {...base} rows={enumRows("changed")} columns={[{ ...enumColumns[0], options: [] }]} onApply={apply} />); expect(screen.queryByRole("listbox")).toBeNull();
    view.rerender(<EngineeringTable {...base} onApply={apply} />); fireEvent.click(input); view.rerender(<EngineeringTable {...base} active={false} onApply={apply} />); expect(screen.queryByRole("listbox")).toBeNull(); expect(apply).not.toHaveBeenCalled();
  });
});

it("keeps complete compact feedback in Info and exempts only this table's chrome from blur Apply", async () => {
  const longError = "The requested field could not be applied because its captured source and engineering basis no longer match. Review the complete retained evidence before retrying; all draft text remains available.";
  const apply = vi.fn(async () => ({ applied: false, messages: [longError] }));
  render(<EngineeringTable label="Compact feedback" compact bounded rows={rows()} columns={columns} generation="p:1" density="comfortable" filter="" selectedKey={rows()[0].key} onSelect={() => {}} onApply={apply} />);
  const input = edit(); fireEvent.change(input, { target: { value: "2" } });
  const table = screen.getByTestId("engineering-table"); const owned = document.createElement("button"), foreign = document.createElement("button");
  // No replacement selector or API: jsdom's absent native feature must not be
  // queried during ordinary blur cleanup. Browser cases verify actual opening.
  const info = table.querySelector<HTMLElement>(".engineering-table-info")!;
  expect(info.hidePopover).toBeUndefined();
  const stateQuery = vi.spyOn(info, "matches");
  owned.dataset.tableChromeOwner = table.dataset.tableOwner; foreign.dataset.tableChromeOwner = "another-table";
  document.body.append(owned, foreign);
  try {
    fireEvent.blur(input, { relatedTarget: owned }); expect(apply).not.toHaveBeenCalled(); expect(input).toHaveValue("2");
    fireEvent.blur(input, { relatedTarget: foreign }); await waitFor(() => expect(apply).toHaveBeenCalledTimes(1));
    expect(stateQuery).not.toHaveBeenCalled();
    expect(screen.getByRole("alert")).toHaveTextContent(longError);
    expect(table.querySelector(".engineering-table-info")).toHaveTextContent(longError);
    expect(document.getElementById(input.getAttribute("aria-describedby")!)).toHaveTextContent(longError);
  } finally { stateQuery.mockRestore(); owned.remove(); foreign.remove(); }
});


it("uses native closed/open Info state when the dismissal API is present", () => {
  const apply = vi.fn();
  render(<EngineeringTable label="Supported Info" compact bounded rows={rows()} columns={columns} generation="p:1" density="comfortable" filter="" selectedKey={rows()[0].key} onSelect={() => {}} onApply={apply} />);
  const table = screen.getByTestId("engineering-table"), info = table.querySelector<HTMLElement>(".engineering-table-info")!;
  let open = false;
  const hide = vi.fn(() => { open = false; });
  Object.defineProperty(info, "hidePopover", { configurable: true, value: hide });
  const originalMatches = info.matches.bind(info);
  const stateQuery = vi.spyOn(info, "matches").mockImplementation((selector) => selector === ":popover-open" ? open : originalMatches(selector));
  try {
    expect(fireEvent.keyDown(table, { key: "Escape" })).toBe(true);
    expect(stateQuery).toHaveBeenCalledWith(":popover-open"); expect(hide).not.toHaveBeenCalled();
    open = true; act(() => info.focus());
    expect(fireEvent.keyDown(info, { key: "Escape" })).toBe(false);
    expect(hide).toHaveBeenCalledTimes(1); expect(screen.getByRole("button", { name: "Supported Info Info" })).toHaveFocus();
    expect(apply).not.toHaveBeenCalled();
  } finally { stateQuery.mockRestore(); Reflect.deleteProperty(info, "hidePopover"); }
});

it("does not swallow a genuine supported-popover state-query failure", () => {
  render(<EngineeringTable label="Native state error" compact bounded rows={rows()} columns={columns} generation="p:1" density="comfortable" filter="" selectedKey={rows()[0].key} onSelect={() => {}} onApply={vi.fn()} />);
  const table = screen.getByTestId("engineering-table"), info = table.querySelector<HTMLElement>(".engineering-table-info")!;
  const failure = new Error("intentional native popover state-query failure"), hide = vi.fn();
  Object.defineProperty(info, "hidePopover", { configurable: true, value: hide });
  const stateQuery = vi.spyOn(info, "matches").mockImplementation(() => { throw failure; });
  let observed: unknown;
  // Capture only this deliberately injected exception after it escapes the
  // product handler; unrelated errors retain the runner's normal handling.
  const captureExpected = (event: ErrorEvent) => { if (event.error === failure) { observed = event.error; event.preventDefault(); event.stopImmediatePropagation(); } };
  window.addEventListener("error", captureExpected, true);
  try {
    fireEvent.keyDown(table, { key: "Escape" });
    expect(observed).toBe(failure); expect(hide).not.toHaveBeenCalled();
  } finally { window.removeEventListener("error", captureExpected, true); stateQuery.mockRestore(); Reflect.deleteProperty(info, "hidePopover"); }
});

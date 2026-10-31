// Run sheet S2 (DEL-07-02 Design `run-sheet-entry.md` §4, §5, §7 S2): the sheet's
// keyboard flow, codes, focus recovery, accessible names and live region, the
// unit lock, and draft-row deletion, on the invented S1 fixture model.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { useMemo, useState } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import fixtureJson from "../../../../../../fixtures/model_operations/run_sheet_authoring.json";
import * as operationBatchService from "../../../services/operationBatchService";
import type { OperationBatch } from "../../../services/operationBatchService";
import type { PreviewModel } from "../../../types";
import { modelIndexFor } from "../../workspace/modelIndex";
import { createSessionEventChannel, type SessionEventChannel } from "../../workspace/sessionEvents";
import { useModelledVisibility } from "../../../test-support/modelledVisibility";
import { RUN_SHEET_TIMING, RunSheet } from "./RunSheet";
import { useRunSheetStore, type RunSheetStore } from "./runSheetStore";

type FixtureRow = Record<string, string>;
type FixtureCase = { id: string; sheet: { source: string; lengthUnit?: string; columnUnits?: Record<string, string>; rows: FixtureRow[] }; expected_batch: OperationBatch };
const fixture = fixtureJson as unknown as { basis_model: PreviewModel; cases: FixtureCase[] };
const basis = () => structuredClone(fixture.basis_model);
const harness: { channel: SessionEventChannel | null; store: RunSheetStore | null } = { channel: null, store: null };

function Harness({ model, register }: { model: PreviewModel; register: (batch: OperationBatch) => Promise<string | null> }) {
  const [channel] = useState(() => createSessionEventChannel());
  const index = useMemo(() => modelIndexFor(model, 0, 0), [model]);
  const store = useRunSheetStore(channel, index, model.project.id);
  harness.channel = channel;
  harness.store = store;
  return <RunSheet host={{ store, queuedBatches: [], editorIntents: [], registerBatch: register }} model={model} />;
}
let register: ReturnType<typeof vi.fn>;
function renderSheet(model = basis()) {
  register = vi.fn(async (batch: OperationBatch) => `operation-batch-for-${batch.batch_id}`);
  return render(<Harness model={model} register={register as never} />);
}
const cell = (row: string, column: string) => screen.getByTestId(`run-sheet-cell-${row}-${column}`);
const live = () => screen.getByTestId("run-sheet-live");
/** Type text into a cell: the first key opens the editor, the text is entered, then `end` commits. */
function type(row: string, column: string, text: string, end: Record<string, unknown> = { key: "Enter" }) {
  const target = cell(row, column);
  act(() => target.focus());
  fireEvent.keyDown(target, { key: text[0] });
  const input = within(target).getByRole("textbox");
  fireEvent.change(input, { target: { value: text } });
  fireEvent.keyDown(input, end);
}
/** Set a code cell with one key. */
function code(row: string, column: "kind" | "support", key: string) {
  const target = cell(row, column);
  act(() => target.focus());
  fireEvent.keyDown(target, { key });
}
/** A sheet from F node:E with one run, typed through the cells. */
function typeStart() {
  fireEvent.change(screen.getByLabelText("Run sheet source"), { target: { value: "invented sheet source" } });
  code("r1", "kind", "f");
  type("r1", "node", "node:E");
}

useModelledVisibility();
beforeEach(() => { RUN_SHEET_TIMING.engineCheckDelayMs = 60_000; });
afterEach(() => { cleanup(); vi.restoreAllMocks(); RUN_SHEET_TIMING.engineCheckDelayMs = 250; });

describe("run sheet keyboard flow (§5.1)", () => {
  it("Enter follows the fast path dX → dY → dZ → Kind → Support → the next row's dX, and appends a row on the last row's Support", () => {
    renderSheet();
    type("r1", "dx", "3");
    expect(cell("r1", "dy")).toHaveFocus();
    fireEvent.keyDown(cell("r1", "dy"), { key: "Enter" });
    expect(cell("r1", "dz")).toHaveFocus();
    type("r1", "dz", "-1");
    expect(cell("r1", "kind")).toHaveFocus();
    fireEvent.keyDown(cell("r1", "kind"), { key: "Enter" });
    expect(cell("r1", "support")).toHaveFocus();
    expect(screen.queryByTestId("run-sheet-row-r2")).toBeNull();
    fireEvent.keyDown(cell("r1", "support"), { key: "Enter" });
    expect(screen.getByTestId("run-sheet-row-r2")).toBeInTheDocument();
    expect(cell("r2", "dx")).toHaveFocus();
    expect(live()).toHaveTextContent("Row 2 added.");
    // Enter on Support of a row that is not the last moves to the next row's dX and appends nothing.
    act(() => cell("r1", "support").focus());
    fireEvent.keyDown(cell("r1", "support"), { key: "Enter" });
    expect(cell("r2", "dx")).toHaveFocus();
    expect(screen.queryByTestId("run-sheet-row-r3")).toBeNull();
  });

  it("Enter in Material, Restraints and k commits and moves down the column, and stays on the last row", () => {
    renderSheet();
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    for (const [column, text] of [["material", "material:steel"], ["restraints", "UX UY"], ["k", "1.5"]] as const) {
      type("r1", column, text);
      expect(cell("r2", column)).toHaveFocus();
      expect(cell("r1", column)).toHaveTextContent(text);
      type("r2", column, text);
      expect(cell("r2", column)).toHaveFocus();
      expect(screen.queryByTestId("run-sheet-row-r3")).toBeNull();
    }
  });

  it("Tab and Shift+Tab move through editable cells in column order, wrapping rows, and Tab past the last cell reaches the footer", () => {
    renderSheet();
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    act(() => cell("r1", "dx").focus());
    fireEvent.keyDown(cell("r1", "dx"), { key: "Tab" });
    expect(cell("r1", "dy")).toHaveFocus();
    fireEvent.keyDown(cell("r1", "dy"), { key: "Tab", shiftKey: true });
    expect(cell("r1", "dx")).toHaveFocus();
    // The last editable cell of row 1 wraps to row 2's dX; the read-only readouts are skipped.
    act(() => cell("r1", "pipeId").focus());
    fireEvent.keyDown(cell("r1", "pipeId"), { key: "Tab" });
    expect(cell("r2", "dx")).toHaveFocus();
    // An editor's Tab commits and moves the same way.
    type("r2", "label", "L2", { key: "Tab" });
    expect(cell("r2", "nodeId")).toHaveFocus();
    expect(cell("r2", "label")).toHaveTextContent("L2");
    act(() => cell("r2", "pipeId").focus());
    fireEvent.keyDown(cell("r2", "pipeId"), { key: "Tab" });
    expect(screen.getByTestId("run-sheet-footer").contains(document.activeElement)).toBe(true);
    // Arrow keys reach the read-only readouts.
    act(() => cell("r1", "pipeId").focus());
    fireEvent.keyDown(cell("r1", "pipeId"), { key: "ArrowRight" });
    expect(cell("r1", "position")).toHaveFocus();
    expect(cell("r1", "position")).toHaveAttribute("aria-readonly", "true");
  });

  it("Enter during IME composition, and Enter or code keys with Ctrl, Meta or Alt, do nothing", () => {
    renderSheet();
    act(() => cell("r1", "dx").focus());
    fireEvent.keyDown(cell("r1", "dx"), { key: "3" });
    const input = within(cell("r1", "dx")).getByRole("textbox");
    fireEvent.change(input, { target: { value: "3" } });
    fireEvent.keyDown(input, { key: "Enter", isComposing: true });
    expect(within(cell("r1", "dx")).getByRole("textbox")).toHaveFocus();
    fireEvent.keyDown(input, { key: "Enter", altKey: true });
    expect(within(cell("r1", "dx")).getByRole("textbox")).toBeInTheDocument();
    fireEvent.keyDown(input, { key: "Escape" });
    expect(cell("r1", "dx")).toHaveFocus();
    expect(cell("r1", "dx")).toHaveTextContent("0");
    fireEvent.keyDown(cell("r1", "dx"), { key: "Enter", isComposing: true });
    expect(cell("r1", "dx")).toHaveFocus();
    for (const modifier of [{ ctrlKey: true }, { metaKey: true }, { altKey: true }]) {
      fireEvent.keyDown(cell("r1", "kind"), { key: "b", ...modifier });
      expect(cell("r1", "kind")).not.toHaveTextContent("B");
    }
  });

  it("a code key sets the code at once; a key that is not a code changes nothing and says which codes there are", () => {
    renderSheet();
    code("r1", "kind", "p");
    expect(cell("r1", "kind")).toHaveTextContent("P");
    expect(within(cell("r1", "kind")).queryByRole("textbox")).toBeNull();
    code("r1", "kind", "x");
    expect(cell("r1", "kind")).toHaveTextContent("P");
    expect(live()).toHaveTextContent("x is not a kind code. Codes: R run, B bend run, S support only, F start from node, P start at point; blank is a run.");
    code("r1", "support", "a");
    expect(cell("r1", "support")).toHaveTextContent("A");
    code("r1", "support", "q");
    expect(cell("r1", "support")).toHaveTextContent("A");
    expect(live()).toHaveTextContent("q is not a support code. Codes: A anchor, G guide, L line stop, V vertical support; blank is no support.");
    fireEvent.keyDown(cell("r1", "support"), { key: "Delete" });
    expect(cell("r1", "support")).toHaveTextContent("");
  });

  it("H and I in the sheet are the sheet's; ⌘ accelerators reach the shell; the sheet's two chords stop at the sheet", () => {
    renderSheet();
    const shell = vi.fn();
    window.addEventListener("keydown", shell);
    try {
      // In a code cell H and I are not codes: consumed with the code list, nothing else acts.
      for (const key of ["h", "i"]) {
        code("r1", "kind", key);
        expect(live()).toHaveTextContent(`${key} is not a kind code.`);
      }
      // In a text cell they start editing.
      act(() => cell("r1", "material").focus());
      fireEvent.keyDown(cell("r1", "material"), { key: "H" });
      expect(within(cell("r1", "material")).getByRole("textbox")).toHaveValue("H");
      fireEvent.keyDown(within(cell("r1", "material")).getByRole("textbox"), { key: "Escape" });
      expect(shell).not.toHaveBeenCalled();
      // The shell's ⌘ keys pass through, unprevented.
      for (const key of ["z", "1", "i", "k"]) fireEvent.keyDown(cell("r1", "dx"), { key, metaKey: true });
      fireEvent.keyDown(cell("r1", "dx"), { key: "z", metaKey: true, shiftKey: true });
      expect(shell.mock.calls.map(([event]) => [event.key, event.defaultPrevented])).toEqual([["z", false], ["1", false], ["i", false], ["k", false], ["z", false]]);
      shell.mockClear();
      fireEvent.keyDown(cell("r1", "dx"), { key: "d", metaKey: true });
      fireEvent.keyDown(cell("r1", "dx"), { key: "Enter", ctrlKey: true });
      expect(shell).not.toHaveBeenCalled();
    } finally {
      window.removeEventListener("keydown", shell);
    }
  });

  it("Escape cancels an edit and is consumed; Fill down copies the effective value into the selected following rows as entered values", () => {
    renderSheet();
    const shell = vi.fn();
    window.addEventListener("keydown", shell);
    type("r1", "material", "material:steel");
    act(() => cell("r1", "material").focus());
    fireEvent.keyDown(cell("r1", "material"), { key: "m" });
    fireEvent.keyDown(within(cell("r1", "material")).getByRole("textbox"), { key: "Escape" });
    expect(shell).not.toHaveBeenCalled();
    window.removeEventListener("keydown", shell);
    expect(cell("r1", "material")).toHaveTextContent("material:steel");
    expect(cell("r1", "material")).toHaveFocus();
    for (let i = 0; i < 3; i += 1) fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    // k never inherits; Fill down copies it as an entered value (O2).
    type("r1", "k", "1.5");
    act(() => cell("r1", "k").focus());
    fireEvent.keyDown(cell("r1", "k"), { key: "ArrowDown", shiftKey: true });
    fireEvent.keyDown(cell("r1", "k"), { key: "ArrowDown", shiftKey: true });
    fireEvent.keyDown(cell("r1", "k"), { key: "d", ctrlKey: true });
    expect(cell("r2", "k")).toHaveTextContent("1.5");
    expect(cell("r3", "k")).toHaveTextContent("1.5");
    expect(cell("r4", "k")).not.toHaveTextContent("1.5");
    expect(cell("r3", "k")).toHaveAccessibleDescription(/^entered/);
    expect(live()).toHaveTextContent("Filled k into 2 rows.");
    // Material inherits; filled, it becomes an entered value.
    expect(cell("r4", "material")).toHaveAccessibleDescription(/^inherited from row 1/);
    act(() => cell("r3", "material").focus());
    fireEvent.keyDown(cell("r3", "material"), { key: "d", metaKey: true });
    expect(cell("r4", "material")).toHaveAccessibleDescription(/^entered/);
  });
});

describe("run sheet Queue reads the committed sheet (Codex r1 P1)", () => {
  it("Ctrl+Enter inside an editor commits the new dX and queues a batch built from it; the ledger records the same value", async () => {
    renderSheet();
    typeStart();
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    type("r2", "dx", "2", { key: "Tab" });
    type("r2", "od", "0.1143", { key: "Tab" });
    type("r2", "wall", "0.00602", { key: "Tab" });
    type("r2", "material", "material:steel", { key: "Tab" });
    type("r2", "orientation", "0 0 1", { key: "Tab" });
    await waitFor(() => expect(screen.getByTestId("run-sheet-queue")).toBeEnabled());
    // Change dX from 2 to 5 and press Ctrl+Enter inside the editor.
    type("r2", "dx", "5", { key: "Enter", ctrlKey: true });
    await waitFor(() => expect(register).toHaveBeenCalledOnce());
    const batch = register.mock.calls[0][0] as OperationBatch;
    const created = batch.operations.find((operation) => operation.change.change_kind === "create_node")!;
    // node:E is at (0, 6, 0): the end node is at x = 5.
    expect(JSON.parse(created.change.after).position).toEqual({ x: 5, y: 6, z: 0 });
    const submitted = harness.store!.current().submitted[batch.batch_id];
    expect(submitted.values.r2.dx).toBe("5");
    expect(submitted.batch).toEqual(batch);
  });
});

describe("run sheet focus recovery (§5.1)", () => {
  it("after Queue focus stays on the same cell, now read-only", async () => {
    renderSheet();
    typeStart();
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    type("r2", "dx", "3");
    type("r2", "od", "0.1143", { key: "Tab" });
    type("r2", "wall", "0.00602", { key: "Tab" });
    type("r2", "material", "material:steel", { key: "Tab" });
    type("r2", "orientation", "0 0 1", { key: "Tab" });
    act(() => cell("r2", "material").focus());
    await waitFor(() => expect(screen.getByTestId("run-sheet-queue")).toBeEnabled());
    fireEvent.keyDown(cell("r2", "material"), { key: "Enter", ctrlKey: true });
    await waitFor(() => expect(register).toHaveBeenCalledOnce());
    await waitFor(() => expect(screen.getByTestId("run-sheet-row-r2")).toHaveAttribute("data-status", "queued"));
    expect(cell("r2", "material")).toHaveFocus();
    expect(cell("r2", "material")).toHaveAttribute("aria-readonly", "true");
    fireEvent.keyDown(cell("r2", "material"), { key: "x" });
    expect(live()).toHaveTextContent("Queued in batch:run-sheet-sheet-1-1; read-only until it is applied or cleared from Pending changes.");
    // Tab from a read-only cell moves to the next editable cell; with every row queued, to the footer.
    fireEvent.keyDown(cell("r2", "material"), { key: "Tab" });
    expect(screen.getByTestId("run-sheet-footer").contains(document.activeElement)).toBe(true);
  });

  it("after Delete row focus goes to the same column of the next shown row, else the previous, else the footer; after Insert row to the new row's dX", () => {
    renderSheet();
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    const openMenu = (row: string, column: string) => {
      act(() => cell(row, column).focus());
      fireEvent.keyDown(cell(row, column), { key: "F10", shiftKey: true });
      return screen.getByRole("menu");
    };
    const run = (menu: HTMLElement, label: string) => fireEvent.click(within(menu).getByText(label).closest("[role='menuitem']")!);
    run(openMenu("r2", "material"), "Delete row");
    expect(cell("r3", "material")).toHaveFocus();
    run(openMenu("r3", "material"), "Delete row");
    expect(cell("r1", "material")).toHaveFocus();
    run(openMenu("r1", "section"), "Insert row below");
    const added = screen.getAllByTestId(/^run-sheet-row-r\d+$/).at(-1)!.dataset.testid!.replace("run-sheet-row-", "");
    expect(cell(added, "dx")).toHaveFocus();
    run(openMenu(added, "dx"), "Insert row above");
    const rows = screen.getAllByTestId(/^run-sheet-row-r\d+$/).map((row) => row.dataset.testid!.replace("run-sheet-row-", ""));
    expect(rows).toHaveLength(3);
    expect(cell(rows[1], "dx")).toHaveFocus();
    for (const row of rows.slice(1)) run(openMenu(row, "k"), "Delete row");
    run(openMenu("r1", "k"), "Delete row");
    expect(screen.getByTestId("run-sheet-footer").contains(document.activeElement)).toBe(true);
  });

  it("after Undo or Redo focus stays on the same row's cell while it is shown; a row menu's unavailable items stay focusable and give their reason", async () => {
    renderSheet();
    typeStart();
    act(() => cell("r1", "node").focus());
    // A session Undo of some other surface's checkpoint folds nothing of this row; focus stays.
    act(() => { harness.channel!.stage({ type: "checkpoint", operationId: "op:other" })(); });
    act(() => { harness.channel!.stage({ type: "undone", operationId: "op:other" })(); });
    expect(cell("r1", "node")).toHaveFocus();
    act(() => { harness.channel!.stage({ type: "redone", operationId: "op:other" })(); });
    expect(cell("r1", "node")).toHaveFocus();
    fireEvent.keyDown(cell("r1", "node"), { key: "ContextMenu" });
    const menu = screen.getByRole("menu");
    const fit = within(menu).getByText("Fit row in viewport").closest("[role='menuitem']")!;
    expect(fit).toHaveAttribute("aria-disabled", "true");
    expect(menu).toHaveTextContent("A row that is not applied has no geometry in the model yet.");
    fireEvent.keyDown(menu, { key: "Escape" });
    await waitFor(() => expect(cell("r1", "node")).toHaveFocus());
  });
});

describe("run sheet names, descriptions and announcements (§2.5, §4.1, §5.3)", () => {
  it("names each cell 'Row n column: value unit', describes its origin and refusal, and heads each row with kind, status and refusal", () => {
    renderSheet();
    typeStart();
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    type("r2", "dx", "3");
    expect(screen.getByRole("table", { name: "Run sheet" })).toBeInTheDocument();
    expect(cell("r2", "dx")).toHaveAccessibleName("Row 2 dX: 3 m");
    expect(cell("r2", "dx")).toHaveAccessibleDescription(/^entered/);
    expect(cell("r2", "dy")).toHaveAccessibleName("Row 2 dY: 0 m");
    expect(cell("r2", "dy")).toHaveAccessibleDescription(/^blank: zero on this axis/);
    expect(live()).toHaveTextContent("Row 2 refused: enter a material id of this model.");
    expect(cell("r2", "material")).toHaveAccessibleDescription(/refused: Enter a material id of this model\./);
    expect(screen.getByTestId("run-sheet-row-header-r2")).toHaveAccessibleName(/^Row 2, run, draft, refused: /);
    // Inherited from stored pipes meeting node:E (none meet it): blank; enter material, then the next row inherits it.
    type("r2", "material", "material:steel");
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    type("r3", "dy", "2");
    expect(cell("r3", "material")).toHaveAccessibleName("Row 3 Material: material:steel");
    expect(cell("r3", "material")).toHaveAccessibleDescription(/^inherited from row 2/);
    expect(within(cell("r3", "material")).getByText("↑", { exact: false })).toBeInTheDocument();
    // Ids are derived italics until overwritten.
    expect(cell("r2", "nodeId")).toHaveAccessibleDescription(/^derived: the sheet's numbering/);
    expect(cell("r1", "kind")).toHaveAccessibleDescription(/Codes: R run, B bend run/);
    // An edit that changes later rows' effective values marks them "changed by row n".
    type("r2", "material", "material:alloy", { key: "Tab" });
    expect(cell("r3", "material")).toHaveAccessibleDescription(/changed by row 2/);
    act(() => cell("r3", "dy").focus());
    expect(cell("r3", "material")).not.toHaveAccessibleDescription(/changed by row 2/);
    // Queue lists the refused rows.
    expect(screen.getByTestId("run-sheet-queue")).toBeDisabled();
    expect(screen.getByTestId("run-sheet-queue")).toHaveAttribute("title", "Refused rows: 2, 3. Correct them to queue.");
  });

  it("an F row naming a stored node inherits from the stored pipes that agree, shown as from that pipe", () => {
    renderSheet();
    typeStart();
    code("r1", "kind", "f");
    type("r1", "node", "node:A");
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    type("r2", "dz", "2");
    expect(cell("r2", "material")).toHaveAccessibleDescription(/^inherited from pipe pipe:AB/);
    expect(within(cell("r2", "material")).getByText("⤷", { exact: false })).toBeInTheDocument();
  });
});

describe("run sheet unit lock and deletion (§4.2, §3.4)", () => {
  it("locks the length unit while any draft row holds increments, with the reason, and unlocks when they are cleared", () => {
    renderSheet();
    const unit = screen.getByLabelText("Length unit") as HTMLInputElement | HTMLSelectElement;
    expect(unit).toBeEnabled();
    type("r1", "dx", "2");
    expect(screen.getByLabelText("Length unit")).toBeDisabled();
    expect(screen.getByLabelText("Length unit")).toHaveAttribute("title", "Draft rows hold increments in m. Queue or clear them to change the unit.");
    expect(screen.getByTestId("run-sheet-unit-lock")).toHaveTextContent("Draft rows hold increments in m.");
    act(() => cell("r1", "dx").focus());
    fireEvent.keyDown(cell("r1", "dx"), { key: "Backspace" });
    expect(screen.getByLabelText("Length unit")).toBeEnabled();
  });

  it("deleting a draft P, F or S row recomputes the rows after it", () => {
    renderSheet();
    fireEvent.change(screen.getByLabelText("Run sheet source"), { target: { value: "invented" } });
    // P at (1, 2, 0), a run of dX 3, an S row with an anchor, a run of dY 1.
    code("r1", "kind", "p");
    type("r1", "dx", "1"); type("r1", "dy", "2");
    for (let i = 0; i < 3; i += 1) fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    type("r2", "dx", "3");
    code("r3", "kind", "s"); code("r3", "support", "a"); type("r3", "restraints", "*");
    type("r4", "dy", "1");
    expect(cell("r2", "position")).toHaveTextContent("4, 2, 0");
    expect(cell("r4", "position")).toHaveTextContent("4, 3, 0");
    const remove = (row: string) => {
      act(() => cell(row, "dx").focus());
      fireEvent.keyDown(cell(row, "dx"), { key: "F10", shiftKey: true });
      fireEvent.click(within(screen.getByRole("menu")).getByText("Delete row").closest("[role='menuitem']")!);
    };
    // Deleting the S row: row 4 continues from row 2's end, unchanged.
    remove("r3");
    expect(screen.getByTestId("run-sheet-row-header-r4")).toHaveAccessibleName(/^Row 3, run, draft/);
    expect(cell("r4", "position")).toHaveTextContent("4, 3, 0");
    // Deleting the P row: the run has no current node.
    remove("r1");
    expect(screen.getByTestId("run-sheet-row-header-r2")).toHaveAccessibleName(/^Row 1, run, draft, refused: start the sheet with F/);
    expect(cell("r2", "position")).toHaveTextContent("");
    // An F row naming a node drafted by a deleted row is refused until it is changed.
    code("r2", "kind", "p");
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    const f = screen.getAllByTestId(/^run-sheet-row-r\d+$/).at(-1)!.dataset.testid!.replace("run-sheet-row-", "");
    const drafted = cell("r2", "nodeId").textContent!;
    code(f, "kind", "f");
    type(f, "node", drafted);
    expect(screen.getByTestId(`run-sheet-row-header-${f}`)).toHaveAccessibleName(/start from node, draft$/);
    remove("r2");
    expect(screen.getByTestId(`run-sheet-row-header-${f}`)).toHaveAccessibleName(/start from node, draft, refused: start the sheet with F/);
    expect(cell(f, "node")).toHaveAccessibleDescription(new RegExp(`Node ${drafted} is not in the model or drafted earlier in this sheet`));
  });
});

describe("run sheet S1 fixture batches queued from the UI (§7 S2)", () => {
  const LABELS: Record<string, string> = { insulationDensity: "Ins. dens.", contentsDensity: "Contents dens.", insulationThickness: "Ins. thk" };
  it.each(fixture.cases.map((item) => [item.id, item] as const))("%s: the batch typed and queued in the sheet equals the hand-built batch", async (_id, item) => {
    const validate = vi.spyOn(operationBatchService, "validateOperationBatch");
    renderSheet();
    fireEvent.change(screen.getByLabelText("Run sheet source"), { target: { value: item.sheet.source } });
    if (item.sheet.lengthUnit) fireEvent.change(screen.getByLabelText("Length unit"), { target: { value: item.sheet.lengthUnit } });
    for (const [column, unit] of Object.entries(item.sheet.columnUnits ?? {})) fireEvent.change(screen.getByLabelText(`${LABELS[column]} unit`), { target: { value: unit } });
    item.sheet.rows.forEach((row, index) => {
      const key = `r${index + 1}`;
      if (index > 0) fireEvent.click(screen.getByTestId("run-sheet-add-row"));
      for (const [column, text] of Object.entries(row)) {
        if (column === "key" || text === "") continue;
        if (column === "kind" || column === "support") code(key, column, text);
        else type(key, column, text, { key: "Tab" });
      }
    });
    if (!Object.keys(item.sheet.columnUnits ?? {}).some((column) => ["materialDensity", "millTolerance", "corrosionAllowance"].includes(column))) {
      await waitFor(() => expect(screen.getByTestId("run-sheet-queue")).toBeEnabled());
    }
    fireEvent.click(screen.getByTestId("run-sheet-queue"));
    await waitFor(() => expect(validate).toHaveBeenCalled());
    await waitFor(() => expect(live()).toHaveTextContent(/Queued rows|Queue refused by the engine/));
    const built = validate.mock.calls.at(-1)![1];
    const expected = { ...item.expected_batch, batch_id: "batch:run-sheet-sheet-1-1" };
    expect(built).toEqual(expected);
    if (item.id === "refused-last-step") expect(register).not.toHaveBeenCalled();
    else expect(register.mock.calls[0][0]).toEqual(expected);
  });
});

describe("run sheet ids and sources, padding and pipe numbering (G1–G3; §1.2, §3.2)", () => {
  it("the Ids and sources group is shown and hidden like Section extras, with labelled cells whose origin is derived until entered", () => {
    renderSheet();
    const toggle = screen.getByRole("button", { name: "Ids and sources" });
    expect(toggle).toHaveAttribute("aria-expanded", "false");
    expect(screen.queryByTestId("run-sheet-cell-r1-supportId")).toBeNull();
    fireEvent.click(toggle);
    expect(toggle).toHaveAttribute("aria-expanded", "true");
    expect(screen.getByRole("button", { name: "Section extras" })).toHaveAttribute("aria-expanded", "false");
    typeStart();
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    type("r2", "dx", "3");
    type("r2", "material", "material:steel");
    code("r2", "support", "a");
    type("r2", "restraints", "*");
    expect(cell("r2", "supportId")).toHaveAccessibleName("Row 2 Support id: support:N20");
    expect(cell("r2", "supportId")).toHaveAccessibleDescription(/^derived: support: and the support node's id token/);
    expect(cell("r2", "supportLabel")).toHaveAccessibleName("Row 2 Support label: N20");
    type("r2", "supportId", "support:RS-A");
    type("r2", "supportLabel", "Anchor A (invented)");
    expect(cell("r2", "supportId")).toHaveAccessibleName("Row 2 Support id: support:RS-A");
    expect(cell("r2", "supportId")).toHaveAccessibleDescription(/^entered/);
    expect(cell("r2", "supportLabel")).toHaveAccessibleName("Row 2 Support label: Anchor A (invented)");
    // Clearing an entered id returns it to its derived value.
    act(() => cell("r2", "supportId").focus());
    fireEvent.keyDown(cell("r2", "supportId"), { key: "Backspace" });
    expect(cell("r2", "supportId")).toHaveAccessibleName("Row 2 Support id: support:N20");
    // Bend cells on a row that is not a bend are refused, and the refusal names the cell.
    expect(cell("r2", "bendStartId")).toHaveAccessibleName("Row 2 Bend start id:");
    type("r2", "bendStartId", "node:X");
    expect(cell("r2", "bendStartId")).toHaveAccessibleDescription(/refused: Bend start id applies only to B rows; clear it\./);
    type("r2", "bendSource", "s", { key: "Tab" });
    expect(cell("r2", "bendSource")).toHaveAccessibleDescription(/refused: Bend geometry source applies only to B rows; clear it\./);
    // Tab moves from Pipe id into the group, in column order.
    act(() => cell("r2", "pipeId").focus());
    fireEvent.keyDown(cell("r2", "pipeId"), { key: "Tab" });
    expect(cell("r2", "supportId")).toHaveFocus();
    fireEvent.click(toggle);
    expect(screen.queryByTestId("run-sheet-cell-r2-supportId")).toBeNull();
  });

  it("a bend row shows its derived ids and the sheet's Source until they are entered", () => {
    renderSheet();
    fireEvent.click(screen.getByRole("button", { name: "Ids and sources" }));
    typeStart();
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    code("r2", "kind", "b");
    type("r2", "dx", "3");
    expect(cell("r2", "bendStartId")).toHaveAccessibleName("Row 2 Bend start id: node:N20-T1");
    expect(cell("r2", "bendStartId")).toHaveAccessibleDescription(/^derived: the corner node's id token/);
    expect(cell("r2", "bendSource")).toHaveAccessibleName("Row 2 Bend geometry source: invented sheet source");
    expect(cell("r2", "bendSource")).toHaveAccessibleDescription(/^derived: the sheet's Source/);
    type("r2", "bendArcId", "pipe:B1-ARC");
    type("r2", "bendSource", "invented bend geometry source");
    expect(cell("r2", "bendArcId")).toHaveAccessibleName("Row 2 Bend arc id: pipe:B1-ARC");
    expect(cell("r2", "bendArcId")).toHaveAccessibleDescription(/^entered/);
    expect(cell("r2", "bendSource")).toHaveAccessibleName("Row 2 Bend geometry source: invented bend geometry source");
    // An entered id that the model holds is refused in its own cell.
    type("r2", "bendEndId", "node:A");
    expect(cell("r2", "bendEndId")).toHaveAccessibleDescription(/refused: Bend end id node:A now exists in the model\./);
  });

  it("pads numbers and numbers a pipe by its start node; both fields are labelled and fixed once a row has a number", () => {
    renderSheet();
    const pad = screen.getByLabelText("Pad numbers to N digits");
    const from = screen.getByLabelText("Pipe number from");
    expect(pad).toHaveValue("");
    expect(from).toHaveValue("end");
    fireEvent.change(screen.getByLabelText("Node id prefix"), { target: { value: "node:RS-" } });
    fireEvent.change(screen.getByLabelText("Pipe id prefix"), { target: { value: "pipe:RS-P" } });
    fireEvent.change(pad, { target: { value: "3" } });
    fireEvent.change(from, { target: { value: "start" } });
    fireEvent.change(screen.getByLabelText("Run sheet source"), { target: { value: "invented" } });
    expect(pad).toBeEnabled();
    code("r1", "kind", "p");
    expect(screen.getByLabelText("Pad numbers to N digits")).toBeDisabled();
    expect(screen.getByLabelText("Pipe number from")).toBeDisabled();
    expect(screen.getByLabelText("Node id prefix")).toBeDisabled();
    type("r1", "dx", "30");
    fireEvent.click(screen.getByTestId("run-sheet-add-row"));
    type("r2", "dx", "0.4");
    expect(cell("r1", "nodeId")).toHaveAccessibleName("Row 1 Node id: node:RS-010");
    expect(cell("r2", "nodeId")).toHaveAccessibleName("Row 2 Node id: node:RS-020");
    expect(cell("r2", "pipeId")).toHaveAccessibleName("Row 2 Pipe id: pipe:RS-P010");
    expect(cell("r2", "pipeId")).toHaveAccessibleDescription(/^derived: the sheet's numbering, by its start node node:RS-010 \(row 1\)/);
  });

  it("an invalid padding disables Queue with the reason", () => {
    renderSheet();
    fireEvent.change(screen.getByLabelText("Pad numbers to N digits"), { target: { value: "0" } });
    expect(screen.getByTestId("run-sheet-queue-reason")).toHaveTextContent("Pad numbers to a whole number of digits from 1 to 16, or leave it blank.");
  });
});

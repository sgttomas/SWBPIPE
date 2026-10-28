import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import fixtureJson from "../../../fixtures/model_operations/run_sheet_authoring.json";
import * as previewService from "./services/previewService";
import type { PreviewModel } from "./types";
import { RUN_SHEET_TIMING } from "./features/model-tree/runSheet/RunSheet";
import { useModelledVisibility } from "./test-support/modelledVisibility";

// Run sheet S2 through the shell (DEL-07-02 Design `run-sheet-entry.md` §3.5, §3.6,
// §7 S2): rows typed in Grid mode's Run sheet tab, queued to Pending changes,
// previewed and applied there; Undo and Redo, another surface's checkpoints,
// outside restoration, a stale queue basis and Requeue, Clear pending changes,
// and the engine's late-step refusal. Invented S1 fixture model (project unit m).

useModelledVisibility();
const fixture = fixtureJson as unknown as { basis_model: PreviewModel };
const initialInnerWidth = window.innerWidth;
beforeEach(() => {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(structuredClone(fixture.basis_model));
  RUN_SHEET_TIMING.engineCheckDelayMs = 60_000;
});
afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  RUN_SHEET_TIMING.engineCheckDelayMs = 250;
  Object.defineProperty(window, "innerWidth", { configurable: true, value: initialInnerWidth });
  window.localStorage.clear();
});

async function renderSheet() {
  Object.defineProperty(window, "innerWidth", { configurable: true, value: 1440 });
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  fireEvent.click(screen.getByTestId("layout-mode-grid"));
  fireEvent.click(await screen.findByTestId("entity-grid-type-run-sheet"));
  await screen.findByTestId("run-sheet");
  fireEvent.change(screen.getByLabelText("Run sheet source"), { target: { value: "invented run-sheet source" } });
}
const cell = (row: string, column: string) => screen.getByTestId(`run-sheet-cell-${row}-${column}`);
const status = (row: string) => screen.getByTestId(`run-sheet-row-${row}`).getAttribute("data-status");
const statuses = (...rows: string[]) => rows.map(status);
const sheet = () => screen.getByTestId("run-sheet");
const cursor = () => Number(sheet().getAttribute("data-ledger-cursor"));
function type(row: string, column: string, text: string) {
  const target = cell(row, column);
  act(() => target.focus());
  fireEvent.keyDown(target, { key: text[0] });
  const input = within(target).getByRole("textbox");
  fireEvent.change(input, { target: { value: text } });
  fireEvent.keyDown(input, { key: "Tab" });
}
function code(row: string, column: "kind" | "support", key: string) {
  act(() => cell(row, column).focus());
  fireEvent.keyDown(cell(row, column), { key });
}
function addRow(): string {
  fireEvent.click(screen.getByTestId("run-sheet-add-row"));
  return screen.getAllByTestId(/^run-sheet-row-r\d+$/).at(-1)!.dataset.testid!.replace("run-sheet-row-", "");
}
/** F node:E, a run of dX 3 with its section, material and orientation, and a run of dY 2 with an anchor. */
function typeChain(start = "node:E") {
  code("r1", "kind", "f");
  type("r1", "node", start);
  const r2 = addRow();
  type(r2, "dx", "3");
  type(r2, "od", "0.1143");
  type(r2, "wall", "0.00602");
  type(r2, "material", "material:steel");
  type(r2, "orientation", "0 0 1");
  const r3 = addRow();
  type(r3, "dy", "2");
  code(r3, "support", "a");
  type(r3, "restraints", "*");
  return [r2, r3] as const;
}
async function queueRows() {
  await waitFor(() => expect(screen.getByTestId("run-sheet-queue")).toBeEnabled());
  fireEvent.click(screen.getByTestId("run-sheet-queue"));
  await waitFor(() => expect(screen.getByTestId("run-sheet-live")).toHaveTextContent(/Queued rows/));
}
const toReview = () => fireEvent.click(screen.getByTestId("workspace-review"));
const toSheet = () => fireEvent.click(screen.getByTestId("stage-tab-model-tree"));
const batchArticle = (batchId: string) => screen.getByText(batchId, { selector: "h4" }).closest("article")!;
async function previewAndApply(batchId: string) {
  toReview();
  const article = batchArticle(batchId);
  fireEvent.click(within(article).getByRole("button", { name: "Validate batch" }));
  await waitFor(() => expect(within(batchArticle(batchId)).getAllByText(/Temporary step/).length).toBeGreaterThan(0));
  fireEvent.click(within(batchArticle(batchId)).getByRole("button", { name: "Apply entire batch" }));
  await waitFor(() => expect(screen.getByTestId("batch-review-message")).toHaveTextContent("Batch applied once."));
  toSheet();
}
const undo = () => fireEvent.click(screen.getByTestId("workspace-undo"));
const redo = () => fireEvent.click(screen.getByTestId("workspace-redo"));
/** Another surface's checkpoint: node:A's y applied directly in the Nodes grid. */
async function editNodeA(value: string) {
  fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
  const target = () => screen.getByTestId("table-cell-node:A-y");
  fireEvent.click(target()); fireEvent.click(target());
  fireEvent.change(screen.getByRole("textbox", { name: "node:A Y [m]" }), { target: { value } });
  fireEvent.click(within(screen.getByTestId("engineering-table")).getByRole("button", { name: "Apply" }));
  await waitFor(() => expect(target()).toHaveTextContent(value));
  fireEvent.click(screen.getByTestId("entity-grid-type-run-sheet"));
  await screen.findByTestId("run-sheet");
}
const treeRow = (type: string, id: string) => screen.queryByTestId(`tree-row-${type}-${encodeURIComponent(id)}`);

describe("run sheet through the shell", () => {
  it("typed rows queue as one batch, show their diff in Pending changes, apply once; Undo returns them to draft and Redo to applied", async () => {
    await renderSheet();
    const [r2, r3] = typeChain();
    await queueRows();
    expect(statuses("r1", r2, r3)).toEqual(["queued", "queued", "queued"]);
    toReview();
    expect(screen.getByTestId("batch-review-summary")).toHaveTextContent("1 queued batches; 0 batches applied this session.");
    const article = batchArticle("batch:run-sheet-sheet-1-1");
    expect(article).toHaveTextContent("5 proposed changes.");
    toSheet();
    await previewAndApply("batch:run-sheet-sheet-1-1");
    expect(statuses("r1", r2, r3)).toEqual(["applied", "applied", "applied"]);
    expect(cursor()).toBe(1);
    expect(cell(r3, "nodeId")).toHaveTextContent("node:N30");
    fireEvent.click(screen.getByTestId("layout-mode-tree"));
    expect(treeRow("node", "node:N30")).toBeInTheDocument();
    expect(treeRow("support", "support:N30")).toBeInTheDocument();
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    undo();
    await waitFor(() => expect(statuses("r1", r2, r3)).toEqual(["draft", "draft", "draft"]));
    expect(cursor()).toBe(0);
    // Undo put the queued values back as the draft rows' values.
    expect(cell(r2, "dx")).toHaveTextContent("3");
    expect(cell(r2, "material")).toHaveTextContent("material:steel");
    redo();
    await waitFor(() => expect(statuses("r1", r2, r3)).toEqual(["applied", "applied", "applied"]));
    expect(cursor()).toBe(1);
    expect(screen.getByTestId(`run-sheet-row-header-${r2}`)).not.toHaveAccessibleName(/edited, not applied/);
  });

  it("Redo after Undo of a create reads applied with no edit marker; a value typed while draft survives Redo as an edit only if it differs", async () => {
    await renderSheet();
    const [r2, r3] = typeChain();
    await queueRows();
    await previewAndApply("batch:run-sheet-sheet-1-1");
    undo();
    await waitFor(() => expect(status(r2)).toBe("draft"));
    // While draft: retype r2's material with the same value, and r3's label with a new one.
    type(r2, "material", "material:steel");
    type(r3, "label", "Invented label");
    redo();
    await waitFor(() => expect(status(r2)).toBe("applied"));
    expect(screen.getByTestId(`run-sheet-row-header-${r2}`)).toHaveAccessibleName(/^Row 2, run, applied$/);
    expect(screen.getByTestId(`run-sheet-row-header-${r3}`)).toHaveAccessibleName(/^Row 3, run, applied, edited, not applied$/);
    expect(cell(r3, "label")).toHaveAccessibleDescription(/edited, not applied/);
    // Discard edit returns the row to its applied values.
    act(() => cell(r3, "label").focus());
    fireEvent.keyDown(cell(r3, "label"), { key: "F10", shiftKey: true });
    fireEvent.click(within(screen.getByRole("menu")).getByText("Discard edit").closest("[role='menuitem']")!);
    expect(screen.getByTestId(`run-sheet-row-header-${r3}`)).toHaveAccessibleName(/^Row 3, run, applied$/);
  });

  it("multiple Undo and Redo: two sheet batches with another surface's checkpoint between them; the cursor and every row follow", async () => {
    await renderSheet();
    const [r2, r3] = typeChain();
    await queueRows();
    await previewAndApply("batch:run-sheet-sheet-1-1");
    await editNodeA("0.25");
    const r4 = addRow();
    type(r4, "dx", "1.5");
    await queueRows();
    await previewAndApply("batch:run-sheet-sheet-1-2");
    const all = () => statuses("r1", r2, r3, r4);
    expect([cursor(), ...all()]).toEqual([3, "applied", "applied", "applied", "applied"]);
    undo();
    await waitFor(() => expect([cursor(), ...all()]).toEqual([2, "applied", "applied", "applied", "draft"]));
    undo();
    // The node edit is undone: a foreign record; the sheet's rows are unchanged.
    await waitFor(() => expect([cursor(), ...all()]).toEqual([1, "applied", "applied", "applied", "draft"]));
    undo();
    await waitFor(() => expect([cursor(), ...all()]).toEqual([0, "draft", "draft", "draft", "draft"]));
    redo();
    await waitFor(() => expect([cursor(), ...all()]).toEqual([1, "applied", "applied", "applied", "draft"]));
    redo();
    await waitFor(() => expect([cursor(), ...all()]).toEqual([2, "applied", "applied", "applied", "draft"]));
    redo();
    await waitFor(() => expect([cursor(), ...all()]).toEqual([3, "applied", "applied", "applied", "applied"]));
  });

  it("restoration: a sheet-owned support removed on another surface detaches its row, and Undo of that removal returns it to applied; Adopt and Remove from sheet work", async () => {
    await renderSheet();
    const [r2, r3] = typeChain();
    await queueRows();
    await previewAndApply("batch:run-sheet-sheet-1-1");
    fireEvent.click(screen.getByTestId("layout-mode-tree"));
    fireEvent.click(treeRow("support", "support:N30")!);
    fireEvent.contextMenu(treeRow("support", "support:N30")!);
    fireEvent.click(within(await screen.findByRole("menu")).getByTestId("tree-context-menu-item-queue-removal"));
    fireEvent.click(await within(screen.getByTestId("workspace-section-operations")).findByTestId("apply-intent-editor-intent-1"));
    await waitFor(() => expect(treeRow("support", "support:N30")).toBeNull());
    toSheet();
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    expect(status(r3)).toBe("detached");
    expect(screen.getByTestId(`run-sheet-detached-${r3}`)).toHaveTextContent("Row 3: Changed outside the sheet: support:N30");
    expect(status(r2)).toBe("applied");
    // Adopt needs every owned entity: the support is gone.
    act(() => cell(r3, "dx").focus());
    fireEvent.keyDown(cell(r3, "dx"), { key: "F10", shiftKey: true });
    expect(within(screen.getByRole("menu")).getByText("Adopt current values").closest("[role='menuitem']")).toHaveAttribute("aria-disabled", "true");
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Escape" });
    undo();
    await waitFor(() => expect(status(r3)).toBe("applied"));
    // A sheet-owned node edited on another surface: Adopt current values, with no operation.
    fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
    const y = () => screen.getByTestId("table-cell-node:N30-y");
    fireEvent.click(y()); fireEvent.click(y());
    fireEvent.change(screen.getByRole("textbox", { name: "node:N30 Y [m]" }), { target: { value: "8.5" } });
    fireEvent.click(within(screen.getByTestId("engineering-table")).getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(y()).toHaveTextContent("8.5"));
    fireEvent.click(screen.getByTestId("entity-grid-type-run-sheet"));
    await waitFor(() => expect(status(r3)).toBe("detached"));
    const records = sheet().getAttribute("data-ledger-records");
    act(() => cell(r3, "dx").focus());
    fireEvent.keyDown(cell(r3, "dx"), { key: "F10", shiftKey: true });
    fireEvent.click(within(screen.getByRole("menu")).getByText("Adopt current values").closest("[role='menuitem']")!);
    expect(status(r3)).toBe("applied");
    expect(sheet().getAttribute("data-ledger-records")).toBe(records);
    // Remove from sheet: the row leaves the sheet, the model keeps its entities.
    act(() => cell(r2, "dx").focus());
    fireEvent.keyDown(cell(r2, "dx"), { key: "F10", shiftKey: true });
    expect(within(screen.getByRole("menu")).getByText("Delete row").closest("[role='menuitem']")).toHaveAttribute("aria-disabled", "true");
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Escape" });
    fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
    const y20 = () => screen.getByTestId("table-cell-node:N20-y");
    fireEvent.click(y20()); fireEvent.click(y20());
    fireEvent.change(screen.getByRole("textbox", { name: "node:N20 Y [m]" }), { target: { value: "6.5" } });
    fireEvent.click(within(screen.getByTestId("engineering-table")).getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(y20()).toHaveTextContent("6.5"));
    fireEvent.click(screen.getByTestId("entity-grid-type-run-sheet"));
    await waitFor(() => expect(status(r2)).toBe("detached"));
    act(() => cell(r2, "dx").focus());
    fireEvent.keyDown(cell(r2, "dx"), { key: "F10", shiftKey: true });
    fireEvent.click(within(screen.getByRole("menu")).getByText("Remove from sheet").closest("[role='menuitem']")!);
    expect(screen.queryByTestId(`run-sheet-row-${r2}`)).toBeNull();
    fireEvent.click(screen.getByTestId("layout-mode-tree"));
    expect(treeRow("pipe", "pipe:P20")).toBeInTheDocument();
  });

  it("an applied row reads stored geometry: after its end node moves outside and Adopt, it shows the stored end and a new row continues from it (Codex r1 P1)", async () => {
    await renderSheet();
    code("r1", "kind", "f");
    type("r1", "node", "node:E");
    const r2 = addRow();
    type(r2, "dx", "2"); type(r2, "od", "0.1143"); type(r2, "wall", "0.00602"); type(r2, "material", "material:steel"); type(r2, "orientation", "0 0 1");
    await queueRows();
    await previewAndApply("batch:run-sheet-sheet-1-1");
    expect(cell(r2, "position")).toHaveTextContent("2, 6, 0");
    // The last node moves on another surface from x = 2 to x = 12.
    fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
    const x = () => screen.getByTestId("table-cell-node:N20-x");
    fireEvent.click(x()); fireEvent.click(x());
    fireEvent.change(screen.getByRole("textbox", { name: "node:N20 X [m]" }), { target: { value: "12" } });
    fireEvent.click(within(screen.getByTestId("engineering-table")).getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(x()).toHaveTextContent("12"));
    fireEvent.click(screen.getByTestId("entity-grid-type-run-sheet"));
    await waitFor(() => expect(status(r2)).toBe("detached"));
    act(() => cell(r2, "dx").focus());
    fireEvent.keyDown(cell(r2, "dx"), { key: "F10", shiftKey: true });
    fireEvent.click(within(screen.getByRole("menu")).getByText("Adopt current values").closest("[role='menuitem']")!);
    expect(status(r2)).toBe("applied");
    expect(cell(r2, "position")).toHaveTextContent("12, 6, 0");
    expect(cell(r2, "dx")).toHaveAccessibleName("Row 2 dX: 12 m");
    expect(cell(r2, "dx")).toHaveAccessibleDescription(/^derived: stored end − stored predecessor/);
    const r3 = addRow();
    type(r3, "dx", "1");
    expect(cell(r3, "position")).toHaveTextContent("13, 6, 0");
    expect(screen.getByTestId(`run-sheet-row-header-${r3}`)).toHaveAccessibleName(/^Row 3, run, draft$/);
    await queueRows();
    await previewAndApply("batch:run-sheet-sheet-1-2");
    fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
    expect(screen.getByTestId("table-cell-node:N30-x")).toHaveTextContent("13");
  });

  it("a model edit on another surface after Queue retires the registration; the rows return to draft marked Requeue needed; Requeue queues a new batch that applies", async () => {
    await renderSheet();
    const [r2, r3] = typeChain();
    await queueRows();
    await editNodeA("0.5");
    toReview();
    const retired = batchArticle("batch:run-sheet-sheet-1-1");
    expect(within(retired).getByRole("note")).toHaveTextContent("Prepared on an earlier model and withdrawn. Prepare it again.");
    expect(within(retired).getAllByRole("button").map((button) => button.textContent)).toEqual(["Dismiss"]);
    expect(screen.getByTestId("batch-review-summary")).toHaveTextContent("0 queued batches; 0 batches applied this session. 1 withdrawn on a model change.");
    fireEvent.click(within(retired).getByRole("button", { name: "Dismiss" }));
    expect(screen.queryByText("batch:run-sheet-sheet-1-1", { selector: "h4" })).toBeNull();
    toSheet();
    expect(statuses("r1", r2, r3)).toEqual(["draft", "draft", "draft"]);
    expect(screen.getByTestId(`run-sheet-row-header-${r2}`)).toHaveAccessibleName(/^Row 2, run, draft, requeue needed$/);
    expect(screen.getByTestId("run-sheet-queue")).toHaveTextContent("Requeue rows");
    await queueRows();
    await previewAndApply("batch:run-sheet-sheet-1-2");
    expect(statuses("r1", r2, r3)).toEqual(["applied", "applied", "applied"]);
    expect(screen.getByTestId(`run-sheet-row-header-${r2}`)).toHaveAccessibleName(/^Row 2, run, applied$/);
  });

  it("Clear pending changes withdraws the batch; the rows return to draft with their values", async () => {
    await renderSheet();
    const [r2, r3] = typeChain();
    await queueRows();
    toReview();
    fireEvent.click(screen.getByTestId("clear-pending-batches"));
    toSheet();
    expect(statuses("r1", r2, r3)).toEqual(["draft", "draft", "draft"]);
    expect(screen.getByTestId(`run-sheet-row-header-${r2}`)).toHaveAccessibleName(/^Row 2, run, draft$/);
    expect(cell(r2, "dx")).toHaveTextContent("3");
    expect(cell(r3, "support")).toHaveTextContent("A");
    expect(screen.getByTestId("run-sheet-queue")).toHaveTextContent("Queue rows");
  });

  it("late-step rollback: a bend radius too large at the last corner is refused by the engine; the rows return to draft with the refusal on that row and the model is unchanged", async () => {
    await renderSheet();
    code("r1", "kind", "f");
    type("r1", "node", "node:E");
    const r2 = addRow();
    code(r2, "kind", "b");
    type(r2, "dx", "3"); type(r2, "od", "0.1143"); type(r2, "wall", "0.00602"); type(r2, "material", "material:steel"); type(r2, "orientation", "0 0 1");
    type(r2, "bendRadius", "0.5"); type(r2, "k", "1.5"); type(r2, "kSource", "invented k");
    const r3 = addRow();
    code(r3, "kind", "b");
    type(r3, "dy", "3"); type(r3, "bendRadius", "5"); type(r3, "k", "1.5"); type(r3, "kSource", "invented k");
    const r4 = addRow();
    type(r4, "dz", "3"); type(r4, "orientation", "1 0 0");
    await waitFor(() => expect(screen.getByTestId("run-sheet-queue")).toBeEnabled());
    fireEvent.click(screen.getByTestId("run-sheet-queue"));
    await waitFor(() => expect(screen.getByTestId("run-sheet-live")).toHaveTextContent("Queue refused by the engine: row 3. The model was not changed."));
    expect(statuses("r1", r2, r3, r4)).toEqual(["draft", "draft", "draft", "draft"]);
    expect(screen.getByTestId(`run-sheet-row-header-${r3}`)).toHaveAccessibleName(/^Row 3, bend run, draft, refused: OP-/);
    expect(screen.getByTestId(`run-sheet-row-header-${r2}`)).toHaveAccessibleName(/^Row 2, bend run, draft$/);
    toReview();
    expect(screen.getByTestId("batch-review-summary")).toHaveTextContent("0 queued batches; 0 batches applied this session.");
    toSheet();
    fireEvent.click(screen.getByTestId("layout-mode-tree"));
    expect(treeRow("node", "node:N20")).toBeNull();
  });
});

describe("run sheet bend rows read stored tangent points (Codex r2 P2)", () => {
  /** F node:A (0, 0, 0); a bend run of dX 10 (corner (10, 0, 0), R 1); then `next`. Tangents: (9, 0, 0) and (10, 1, 0). */
  async function appliedBend(next: Array<[string, string]> = [["dy", "10"]], second?: Array<[string, string]>) {
    await renderSheet();
    code("r1", "kind", "f");
    type("r1", "node", "node:A");
    const r2 = addRow();
    code(r2, "kind", "b");
    for (const [column, text] of [["dx", "10"], ["od", "0.1143"], ["wall", "0.00602"], ["material", "material:steel"], ["orientation", "0 0 1"], ["bendRadius", "1"], ["k", "1.5"], ["kSource", "invented k"]]) type(r2, column, text);
    const r3 = addRow();
    if (second) code(r3, "kind", "b");
    for (const [column, text] of next) type(r3, column, text);
    let r4: string | null = null;
    if (second) {
      r4 = addRow();
      for (const [column, text] of second) type(r4, column, text);
    }
    await queueRows();
    await previewAndApply("batch:run-sheet-sheet-1-1");
    return { r2, r3, r4 };
  }
  const moveNode = async (id: string, axis: "x" | "y" | "z", value: string) => {
    fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
    const target = () => screen.getByTestId(`table-cell-${id}-${axis}`);
    fireEvent.click(target()); fireEvent.click(target());
    fireEvent.change(screen.getByRole("textbox", { name: `${id} ${axis.toUpperCase()} [m]` }), { target: { value } });
    fireEvent.click(within(screen.getByTestId("engineering-table")).getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(target()).toHaveTextContent(value));
    fireEvent.click(screen.getByTestId("entity-grid-type-run-sheet"));
    await screen.findByTestId("run-sheet");
  };
  const menuItem = (row: string, label: string) => {
    act(() => cell(row, "dx").focus());
    fireEvent.keyDown(cell(row, "dx"), { key: "F10", shiftKey: true });
    return within(screen.getByRole("menu")).getByText(label).closest("[role='menuitem']") as HTMLElement;
  };

  it("the row after an applied bend shows stored end minus the stored outgoing tangent point; a draft row after it emits what it displays", async () => {
    const { r2, r3 } = await appliedBend();
    expect(statuses(r2, r3)).toEqual(["applied", "applied"]);
    // The bend row's own pipe ends at the incoming tangent point (9, 0, 0).
    expect(cell(r2, "position")).toHaveTextContent("9, 0, 0");
    expect(cell(r2, "dx")).toHaveAccessibleName("Row 2 dX: 9 m");
    // The next row's pipe starts at the outgoing tangent point (10, 1, 0): dY = 10 − 1 = 9.
    expect(cell(r3, "dy")).toHaveAccessibleName("Row 3 dY: 9 m");
    expect(cell(r3, "position")).toHaveTextContent("10, 10, 0");
    const r4 = addRow();
    type(r4, "dx", "1");
    expect(cell(r4, "position")).toHaveTextContent("11, 10, 0");
    await queueRows();
    toReview();
    const batch = JSON.parse(within(batchArticle("batch:run-sheet-sheet-1-2")).getByText(/"batch_id"/).textContent!);
    const created = batch.operations.find((operation: { change: { change_kind: string } }) => operation.change.change_kind === "create_node");
    expect(JSON.parse(created.change.after).position).toEqual({ x: 11, y: 10, z: 0 });
  });

  it("an F branch naming a stored tangent node resolves; Start branch here on an applied bend names its outgoing tangent node, and on a draft bend is unavailable", async () => {
    const { r2 } = await appliedBend();
    const f = addRow();
    code(f, "kind", "f");
    type(f, "node", "node:N20-T1");
    expect(screen.getByTestId(`run-sheet-row-header-${f}`)).toHaveAccessibleName(/^Row 4, start from node, draft$/);
    const run = addRow();
    type(run, "dz", "2");
    type(run, "orientation", "1 0 0");
    expect(cell(run, "position")).toHaveTextContent("9, 0, 2");
    expect(screen.getByTestId(`run-sheet-row-header-${run}`)).toHaveAccessibleName(/^Row 5, run, draft$/);
    fireEvent.click(menuItem(r2, "Start branch here"));
    const branch = screen.getAllByTestId(/^run-sheet-row-r\d+$/).at(-1)!.dataset.testid!.replace("run-sheet-row-", "");
    expect(cell(branch, "node")).toHaveTextContent("node:N20-T2");
    expect(screen.getByTestId(`run-sheet-row-header-${branch}`)).toHaveAccessibleName(/start from node, draft$/);
    // A draft bend's corner is replaced on apply: no branch may start there.
    const draftBend = addRow();
    code(draftBend, "kind", "f");
    type(draftBend, "node", "node:E");
    const b = addRow();
    code(b, "kind", "b");
    type(b, "dx", "3");
    const item = menuItem(b, "Start branch here");
    expect(item).toHaveAttribute("aria-disabled", "true");
    expect(screen.getByRole("menu")).toHaveTextContent("A bend's corner is replaced by its tangent points when it is applied; start a branch from the applied bend.");
  });

  it("two consecutive bends: each row reads its stored tangent points", async () => {
    const { r2, r3, r4 } = await appliedBend([["dy", "10"], ["bendRadius", "1"], ["k", "1.5"], ["kSource", "invented k"]], [["dz", "10"], ["orientation", "1 0 0"]]);
    expect(statuses(r2, r3, r4!)).toEqual(["applied", "applied", "applied"]);
    expect(cell(r2, "dx")).toHaveAccessibleName("Row 2 dX: 9 m");
    // Row 3 runs from the first outgoing tangent (10, 1, 0) to the second incoming tangent (10, 9, 0).
    expect(cell(r3, "dy")).toHaveAccessibleName("Row 3 dY: 8 m");
    expect(cell(r3, "position")).toHaveTextContent("10, 9, 0");
    // Row 4 runs from the second outgoing tangent (10, 10, 1) to (10, 10, 10).
    expect(cell(r4!, "dz")).toHaveAccessibleName("Row 4 dZ: 9 m");
  });

  it("a detached bend row: its tangent point moved outside shows the stored position, and the next row's increment follows it", async () => {
    const { r2, r3 } = await appliedBend();
    await moveNode("node:N20-T2", "y", "1.5");
    expect(status(r2)).toBe("detached");
    expect(screen.getByTestId(`run-sheet-detached-${r2}`)).toHaveTextContent("Changed outside the sheet: node:N20-T2");
    expect(status(r3)).toBe("applied");
    expect(cell(r3, "dy")).toHaveAccessibleName("Row 3 dY: 8.5 m");
    fireEvent.click(menuItem(r2, "Adopt current values"));
    expect(status(r2)).toBe("applied");
    expect(cell(r3, "dy")).toHaveAccessibleName("Row 3 dY: 8.5 m");
  });
});

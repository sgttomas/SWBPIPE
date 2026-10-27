// B9: context menus on tree rows and on the grids' rows and cells. Each test opens the menu
// from the keyboard or the pointer, moves in it, and checks that an item runs the existing
// command (the viewport's view command, the inspector's removal intent, the table's own edit
// and sort), that disabled items say why, and that focus returns to where the menu was opened.
// Invented fixtures only.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import type { ViewportViewCommand } from "../viewport/viewportSelection";
import { entityKey, singletonSelection, type OrderedSelectionState } from "../workspace/selectionState";
import { ModelTree } from "./ModelTree";
import { PropertyInspector } from "./PropertyInspector";
import { useModelledVisibility } from "../../test-support/modelledVisibility";

useModelledVisibility();

afterEach(cleanup);

/** Invented model: every entity and value is synthetic test input. */
function inventedModel(): PreviewModel {
  const model = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));
  model.project.units = { length: "mm", force: "N", pressure: "MPa", temperature: "C", angle: "rad" };
  model.nodes = [
    { id: "node:A", label: "Node A", position: { x: 0, y: 0, z: 0 }, provenance: "invented node" },
    { id: "node:B", label: "Node B", position: { x: 1000, y: 0, z: 0 }, provenance: "invented node" }
  ];
  model.pipe_segments = [
    { id: "pipe:AB", label: "Pipe AB", from: "node:A", to: "node:B", material: "material:X", provenance: "invented pipe",
      section: { outside_diameter: { value: 100, unit: "mm" }, wall_thickness: { value: 5, unit: "mm" } } }
  ];
  model.supports = [{ id: "support:A", label: "Support A", node: "node:A", restraints: ["UX", "UY"], provenance: "invented support" }];
  model.materials = [{ id: "material:X", label: "Invented material", elastic_modulus: { value: 200000, unit: "MPa" }, provenance: "invented material" }];
  return model;
}

const node = (id: string): EntityRef => ({ type: "node", id });

function selectionOf(refs: readonly EntityRef[], focus: EntityRef = refs[0]): OrderedSelectionState {
  const keys = refs.map(entityKey);
  return { orderedKeys: keys, primaryKey: keys[0], rangeAnchorKey: keys[0], focusKey: entityKey(focus), preparationEpoch: 0 };
}

function mountTree({ selection = node("node:A"), selectionState = selectionOf([selection]), model = inventedModel(), withViewport = true } = {}) {
  const viewCommand = vi.fn<(command: ViewportViewCommand) => void>();
  const onSelect = vi.fn();
  const onQueueIntent = vi.fn<(intent: EditorOperationIntent) => void>();
  const viewCommandRef = { current: withViewport ? viewCommand : null };
  const view = render(
    <ModelTree model={model} selection={selection} selectionState={selectionState} onSelect={onSelect} onQueueIntent={onQueueIntent}
      viewCommandRef={viewCommandRef} showAllAvailable={false} />
  );
  return { view, viewCommand, onSelect, onQueueIntent, model };
}

const treeList = () => screen.getByTestId("model-tree-virtual");
const treeRow = (ref: EntityRef) => screen.getByTestId(`tree-row-${encodeURIComponent(ref.type)}-${encodeURIComponent(ref.id)}`);
const menuItem = (menu: HTMLElement, id: string) => within(menu).getByTestId(`${menu.dataset.testid}-item-${id}`);

describe("the tree row context menu", () => {
  it("opens with Shift+F10 on the focused row, moves with the arrows, and routes Hide to the viewport's command", () => {
    const { viewCommand, onSelect } = mountTree();
    treeList().focus();
    fireEvent.keyDown(treeList(), { key: "F10", shiftKey: true });
    const menu = screen.getByRole("menu", { name: "Commands for node node:A" });
    expect(menu.dataset.testid).toBe("tree-context-menu");
    expect(menuItem(menu, "hide-selection")).toHaveFocus();
    // H and I run Hide and Isolate in the viewport only, so the tree's menu shows no shortcut.
    expect(menuItem(menu, "hide-selection")).not.toHaveAttribute("aria-keyshortcuts");
    fireEvent.keyDown(menu, { key: "ArrowDown" });
    expect(menuItem(menu, "isolate-selection")).toHaveFocus();
    fireEvent.keyDown(menu, { key: "ArrowUp" });
    fireEvent.keyDown(menu, { key: "Enter" });
    expect(viewCommand).toHaveBeenCalledExactlyOnceWith({ type: "hide-selection" });
    expect(screen.queryByRole("menu")).toBeNull();
    expect(treeList()).toHaveFocus();
    // Opening the menu from the keyboard never changes the selection.
    expect(onSelect).not.toHaveBeenCalled();
  });

  it("opens with the ContextMenu key and closes on Escape with focus back on the tree", () => {
    const { viewCommand } = mountTree();
    treeList().focus();
    fireEvent.keyDown(treeList(), { key: "ContextMenu" });
    const menu = screen.getByRole("menu");
    fireEvent.keyDown(menu, { key: "Escape" });
    expect(screen.queryByRole("menu")).toBeNull();
    expect(treeList()).toHaveFocus();
    expect(viewCommand).not.toHaveBeenCalled();
  });

  it("offers Isolate, Fit selection and Show all through the same command, each disabled with its reason where it does not apply", () => {
    const { viewCommand } = mountTree();
    treeList().focus();
    fireEvent.keyDown(treeList(), { key: "F10", shiftKey: true });
    let menu = screen.getByRole("menu");
    fireEvent.click(menuItem(menu, "isolate-selection"));
    expect(viewCommand).toHaveBeenLastCalledWith({ type: "isolate-selection" });
    fireEvent.keyDown(treeList(), { key: "F10", shiftKey: true });
    menu = screen.getByRole("menu");
    fireEvent.click(menuItem(menu, "fit-selection"));
    expect(viewCommand).toHaveBeenLastCalledWith({ type: "fit-selection" });
    fireEvent.keyDown(treeList(), { key: "F10", shiftKey: true });
    menu = screen.getByRole("menu");
    const showAll = menuItem(menu, "show-all");
    expect(showAll).toHaveAttribute("aria-disabled", "true");
    expect(showAll).toHaveAccessibleDescription("All eligible entities are already shown.");
    fireEvent.click(showAll);
    expect(viewCommand).toHaveBeenCalledTimes(2);
  });

  it("from the keyboard on a row that is not selected, leaves the selection and says the commands act on it", () => {
    const selection = node("node:A");
    const { viewCommand, onSelect } = mountTree({ selection, selectionState: selectionOf([selection], node("node:B")) });
    treeList().focus();
    fireEvent.keyDown(treeList(), { key: "F10", shiftKey: true });
    const menu = screen.getByRole("menu", { name: "Commands for node node:B" });
    for (const id of ["hide-selection", "isolate-selection", "fit-selection", "queue-removal"]) {
      expect(menuItem(menu, id)).toHaveAttribute("aria-disabled", "true");
      expect(menuItem(menu, id)).toHaveAccessibleDescription("This acts on the selection, which does not include node:B. Select it first.");
    }
    fireEvent.keyDown(menu, { key: "Enter" });
    expect(viewCommand).not.toHaveBeenCalled();
    expect(onSelect).not.toHaveBeenCalled();
  });

  it("a right-click on a row that is not selected selects it first; on a selected row it keeps the selection", () => {
    const a = node("node:A"); const b = node("node:B");
    const first = mountTree({ selection: a });
    fireEvent.contextMenu(treeRow(b), { clientX: 40, clientY: 50 });
    expect(first.onSelect).toHaveBeenCalledOnce();
    expect(first.onSelect.mock.calls[0][0]).toMatchObject(b);
    expect(first.onSelect.mock.calls[0][1]).toEqual({});
    expect(screen.getByRole("menu", { name: "Commands for node node:B" })).toBeInTheDocument();
    cleanup();

    const multiple = mountTree({ selection: a, selectionState: selectionOf([a, b]) });
    fireEvent.contextMenu(treeRow(b), { clientX: 40, clientY: 50 });
    expect(multiple.onSelect).not.toHaveBeenCalled();
    const menu = screen.getByRole("menu", { name: "Commands for node node:B" });
    expect(menuItem(menu, "hide-selection")).not.toHaveAttribute("aria-disabled");
    expect(menuItem(menu, "queue-removal")).toHaveAccessibleDescription("Removal queues one entity at a time; select only node:B.");
    // The right-click leaves focus on the tree list, which takes it back on close.
    fireEvent.keyDown(menu, { key: "Escape" });
    expect(treeList()).toHaveFocus();
  });

  it.each([
    [node("node:A"), "queue-delete-node-intent"],
    [{ type: "pipe", id: "pipe:AB" } as EntityRef, "queue-delete-pipe-intent"],
    [{ type: "support", id: "support:A" } as EntityRef, "queue-delete-support-intent"]
  ])("queues the inspector's own removal intent for %o", (target, inspectorButton) => {
    const model = inventedModel();
    const inspectorQueue = vi.fn<(intent: EditorOperationIntent) => void>();
    const inspector = render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector model={model} selection={target} onQueueIntent={inspectorQueue} /></DisplayUnitsProvider>);
    fireEvent.click(screen.getByTestId(inspectorButton));
    const expected = inspectorQueue.mock.calls[0][0];
    inspector.unmount();

    const before = structuredClone(model);
    const { onQueueIntent } = mountTree({ selection: target, model });
    fireEvent.contextMenu(treeRow(target));
    const menu = screen.getByRole("menu");
    fireEvent.click(menuItem(menu, "queue-removal"));
    expect(onQueueIntent).toHaveBeenCalledExactlyOnceWith(expected);
    expect(model).toEqual(before);
  });

  it("states why a command is unavailable without a viewport or for an entity type without one", () => {
    const material: EntityRef = { type: "material", id: "material:X" };
    mountTree({ selection: material, withViewport: false });
    fireEvent.contextMenu(treeRow(material));
    const menu = screen.getByRole("menu");
    expect(menuItem(menu, "hide-selection")).toHaveAccessibleDescription("The viewport is unavailable.");
    expect(menuItem(menu, "show-all")).toHaveAccessibleDescription("The viewport is unavailable.");
  });

  it("opens no menu on a group row", () => {
    const selection = node("node:A");
    mountTree({ selection, selectionState: { ...singletonSelection(selection), focusKey: null } });
    treeList().focus();
    // Move to the first group row: the project row comes first.
    fireEvent.keyDown(treeList(), { key: "Home" });
    fireEvent.keyDown(treeList(), { key: "ArrowDown" });
    expect(treeList().getAttribute("aria-activedescendant")).toMatch(/^tree-group-/u);
    fireEvent.keyDown(treeList(), { key: "F10", shiftKey: true });
    expect(screen.queryByRole("menu")).toBeNull();
  });
});

describe("the bulk grid row context menu", () => {
  function openGrid(family: string) {
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    fireEvent.click(screen.getByTestId(`entity-grid-type-${family}`));
  }

  it("opens on a row from the pointer, selecting it, and from Shift+F10 on its ID, keeping the platform menu in its fields", () => {
    const pipe: EntityRef = { type: "pipe", id: "pipe:AB" };
    const { onSelect, viewCommand } = mountTree({ selection: node("node:A") });
    openGrid("pipes");
    const header = screen.getByTestId("entity-grid-row-pipe-pipe:AB");
    const row = header.closest("tr")!;
    const field = within(row).getAllByRole("textbox")[0];
    // A field's own right-click is the platform's text menu.
    expect(fireEvent.contextMenu(field)).toBe(true);
    expect(screen.queryByRole("menu")).toBeNull();

    expect(fireEvent.contextMenu(row.querySelector("td")!)).toBe(false);
    expect(onSelect).toHaveBeenCalledExactlyOnceWith(pipe);
    let menu = screen.getByRole("menu", { name: "Commands for pipe pipe:AB" });
    expect(menu.dataset.testid).toBe("grid-context-menu");
    fireEvent.keyDown(menu, { key: "Escape" });
    expect(header).toHaveFocus();

    onSelect.mockClear();
    fireEvent.keyDown(header, { key: "F10", shiftKey: true });
    menu = screen.getByRole("menu", { name: "Commands for pipe pipe:AB" });
    expect(onSelect).not.toHaveBeenCalled();
    // pipe:AB is not selected here (node:A is): the commands say so.
    expect(menuItem(menu, "hide-selection")).toHaveAccessibleDescription("This acts on the selection, which does not include pipe:AB. Select it first.");
    // Tab leaves the menu for the tab stop after the ID button, as Tab from it would.
    fireEvent.keyDown(menu, { key: "Tab" });
    expect(screen.getByTestId("entity-grid-input-pipe:AB-label")).toHaveFocus();
    expect(viewCommand).not.toHaveBeenCalled();
  });
});

describe("the bulk grid when its last row goes", () => {
  it("puts focus on the grid's table region, not the page, when the menu's only row is removed (BULK_LAST_ROW_REMOVED)", () => {
    const model = inventedModel();
    const selection: EntityRef = { type: "pipe", id: "pipe:AB" };
    const { view } = mountTree({ selection, model });
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    fireEvent.click(screen.getByTestId("entity-grid-type-pipes"));
    const header = screen.getByTestId("entity-grid-row-pipe-pipe:AB");
    header.focus();
    fireEvent.keyDown(header, { key: "F10", shiftKey: true });
    expect(screen.getByRole("menu")).toBeInTheDocument();
    view.rerender(<ModelTree model={{ ...model, pipe_segments: [] }} selection={selection} selectionState={selectionOf([selection])} onSelect={vi.fn()} />);
    expect(header.isConnected).toBe(false);
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Escape" });
    expect(document.activeElement).not.toBe(document.body);
    expect(screen.getByRole("region", { name: "Editable model entity table" })).toHaveFocus();
    expect(screen.getByTestId("entity-grid-empty")).toBeInTheDocument();
  });
});

describe("the engineering table cell context menu", () => {
  const nodeCell = (id: string, column: string) => screen.getByTestId(`table-cell-${id}-${column}`);

  function openNodeFields(selection = node("node:A"), model = inventedModel()) {
    const mounted = mountTree({ selection, model });
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
    return mounted;
  }

  it("puts the cell's own commands first: Edit cell (Enter) opens the editor as Enter does", () => {
    openNodeFields();
    const cell = nodeCell("node:A", "x");
    cell.focus();
    fireEvent.keyDown(cell, { key: "F10", shiftKey: true });
    const menu = screen.getByRole("menu", { name: "Commands for node node:A" });
    expect(menu.dataset.testid).toBe("table-context-menu");
    expect(within(menu).getAllByRole("menuitem").map((item) => item.dataset.menuItemId)).toEqual([
      "edit-cell", "sort-ascending", "sort-descending", "clear-sort",
      "hide-selection", "isolate-selection", "fit-selection", "show-all", "queue-removal"
    ]);
    const edit = menuItem(menu, "edit-cell");
    expect(edit).toHaveFocus();
    expect(edit).toHaveAttribute("aria-keyshortcuts", "Enter");
    fireEvent.keyDown(menu, { key: "Enter" });
    expect(screen.queryByRole("menu")).toBeNull();
    const editor = screen.getByRole("textbox", { name: /node:A X/u });
    expect(editor).toHaveFocus();
    // Escape cancels the edit as it does for an edit begun with Enter.
    fireEvent.keyDown(editor, { key: "Escape" });
    expect(screen.queryByRole("textbox", { name: /node:A X/u })).toBeNull();
  });

  it("sorts through the column's own sort state, and Clear sort is disabled with its reason until then", () => {
    openNodeFields();
    const grid = screen.getByRole("grid", { name: "Node fields" });
    const xHeader = within(grid).getAllByRole("columnheader").find((header) => header.textContent?.startsWith("X"))!;
    expect(xHeader).toHaveAttribute("aria-sort", "none");
    const cell = nodeCell("node:A", "x");
    fireEvent.contextMenu(cell);
    let menu = screen.getByRole("menu");
    expect(menuItem(menu, "clear-sort")).toHaveAccessibleDescription("The table is not sorted.");
    fireEvent.click(menuItem(menu, "sort-descending"));
    expect(xHeader).toHaveAttribute("aria-sort", "descending");
    expect(cell).toHaveFocus();
    fireEvent.contextMenu(cell);
    menu = screen.getByRole("menu");
    expect(menuItem(menu, "sort-descending")).toHaveAccessibleDescription("Already sorted by X, descending.");
    fireEvent.click(menuItem(menu, "clear-sort"));
    expect(xHeader).toHaveAttribute("aria-sort", "none");
  });

  it("disables Edit cell on a read-only cell with the cell's reason", () => {
    const model = inventedModel();
    // Structured provenance is shown, never edited as text: that cell is read-only.
    (model.nodes[1] as { provenance: unknown }).provenance = { source: "invented survey" };
    openNodeFields(node("node:A"), model);
    const grid = screen.getByRole("grid", { name: "Node fields" });
    const readOnly = within(grid).getAllByRole("gridcell").find((cell) => cell.getAttribute("aria-readonly") === "true");
    expect(readOnly).toBeDefined();
    const button = readOnly!.querySelector<HTMLElement>("[data-table-cell]")!;
    button.focus();
    fireEvent.keyDown(button, { key: "ContextMenu" });
    const edit = menuItem(screen.getByRole("menu"), "edit-cell");
    expect(edit).toHaveAttribute("aria-disabled", "true");
    expect(edit).toHaveAccessibleDescription(readOnly!.getAttribute("title") || "This cell is read-only.");
  });

  it("a right-click on another row's cell selects that row first", () => {
    const { onSelect } = openNodeFields(node("node:A"));
    fireEvent.contextMenu(nodeCell("node:B", "x"));
    expect(onSelect).toHaveBeenCalledExactlyOnceWith(node("node:B"));
    expect(screen.getByRole("menu", { name: "Commands for node node:B" })).toBeInTheDocument();
  });
});

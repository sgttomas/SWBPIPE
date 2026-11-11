/** B2 (DEL-07-02 Design `bulk-change.md`): Change selected… in the B9 entity menus of the tree,
 * the bulk grid, the engineering tables and the viewport. It opens Bulk change on the selection,
 * follows the B9 selection rule, and is absent where no Bulk change is wired. Invented fixtures. */
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { bulkChangeModel } from "../../test-support/bulkChangeModel";
import { useModelledVisibility } from "../../test-support/modelledVisibility";
import type { EntityRef } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { ModelTree } from "../model-tree/ModelTree";
import { PipeViewport } from "../viewport/PipeViewport";
import { entityKey, singletonSelection, type OrderedSelectionState } from "../workspace/selectionState";

useModelledVisibility();
afterEach(cleanup);

const pipe = (id: string): EntityRef => ({ type: "pipe", id });
function selectionOf(refs: readonly EntityRef[]): OrderedSelectionState {
  const keys = refs.map(entityKey);
  return { orderedKeys: keys, primaryKey: keys[0], rangeAnchorKey: keys[0], focusKey: keys[0], preparationEpoch: 0 };
}
const item = (menu: HTMLElement, id: string) => within(menu).getByTestId(`${menu.dataset.testid}-item-${id}`);

function mountTree(selected: readonly EntityRef[], onChangeSelected?: () => void) {
  render(<ModelTree model={bulkChangeModel()} selection={selected[0]} selectionState={selectionOf(selected)} onSelect={vi.fn()} onQueueIntent={vi.fn()}
    viewCommandRef={{ current: vi.fn() }} onChangeSelected={onChangeSelected} />);
}

describe("Change selected… in the entity menus", () => {
  it("in the tree, from the keyboard, runs on a multiple selection and keeps it", () => {
    const onChangeSelected = vi.fn();
    mountTree([pipe("pipe:P1"), pipe("pipe:P2")], onChangeSelected);
    const tree = screen.getByTestId("model-tree-virtual");
    tree.focus();
    fireEvent.keyDown(tree, { key: "F10", shiftKey: true });
    const menu = screen.getByRole("menu", { name: "Commands for pipe pipe:P1" });
    expect(within(menu).getAllByRole("menuitem").map((entry) => entry.dataset.menuItemId)).toEqual([
      "hide-selection", "isolate-selection", "fit-selection", "show-all", "change-selected", "queue-removal"
    ]);
    const change = item(menu, "change-selected");
    expect(change).toHaveTextContent("Change selected…");
    fireEvent.keyDown(menu, { key: "End" });
    fireEvent.keyDown(menu, { key: "ArrowUp" });
    expect(change).toHaveFocus();
    fireEvent.keyDown(menu, { key: "Enter" });
    expect(onChangeSelected).toHaveBeenCalledTimes(1);
    expect(screen.queryByRole("menu")).toBeNull();
  });

  it("in the bulk grid, is disabled with the B9 reason when the row is not selected", () => {
    const onChangeSelected = vi.fn();
    mountTree([pipe("pipe:P1")], onChangeSelected);
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    fireEvent.click(screen.getByTestId("entity-grid-type-pipes"));
    const header = screen.getByTestId("entity-grid-row-pipe-pipe:P2");
    fireEvent.keyDown(header, { key: "F10", shiftKey: true });
    const menu = screen.getByRole("menu", { name: "Commands for pipe pipe:P2" });
    const change = item(menu, "change-selected");
    expect(change).toHaveAttribute("aria-disabled", "true");
    expect(change).toHaveAccessibleDescription("This acts on the selection, which does not include pipe:P2. Select it first.");
    fireEvent.click(change);
    expect(onChangeSelected).not.toHaveBeenCalled();
    fireEvent.keyDown(menu, { key: "Escape" });
    fireEvent.keyDown(screen.getByTestId("entity-grid-row-pipe-pipe:P1"), { key: "F10", shiftKey: true });
    fireEvent.click(item(screen.getByRole("menu"), "change-selected"));
    expect(onChangeSelected).toHaveBeenCalledTimes(1);
  });

  it("in an engineering table cell", () => {
    const onChangeSelected = vi.fn();
    mountTree([{ type: "node", id: "node:A" }], onChangeSelected);
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
    fireEvent.contextMenu(screen.getByTestId("table-cell-node:A-x"));
    fireEvent.click(item(screen.getByRole("menu"), "change-selected"));
    expect(onChangeSelected).toHaveBeenCalledTimes(1);
  });

  it("is absent where no Bulk change is wired", () => {
    mountTree([pipe("pipe:P1")]);
    const tree = screen.getByTestId("model-tree-virtual");
    tree.focus();
    fireEvent.keyDown(tree, { key: "F10", shiftKey: true });
    expect(within(screen.getByRole("menu")).queryByTestId("tree-context-menu-item-change-selected")).toBeNull();
  });

  it("in the viewport, on a label", () => {
    const onChangeSelected = vi.fn();
    const selection = pipe("pipe:P1");
    render(
      <DisplayUnitsProvider>
        <PipeViewport model={bulkChangeModel()} result={null} selection={selection} selectionState={{ ...singletonSelection(selection), focusKey: entityKey(selection) }}
          onSelect={vi.fn((ref: EntityRef) => singletonSelection(ref))} onQueueIntent={vi.fn()} onChangeSelected={onChangeSelected} />
      </DisplayUnitsProvider>
    );
    fireEvent.contextMenu(within(screen.getByTestId("viewport-selection-layer")).getByTestId("viewport-select-pipe:P1"));
    fireEvent.click(screen.getByTestId("viewport-context-menu-item-change-selected"));
    expect(onChangeSelected).toHaveBeenCalledTimes(1);
  });
});

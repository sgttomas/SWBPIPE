import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import { useModelledVisibility } from "./test-support/modelledVisibility";

useModelledVisibility();

// B2 through the product (DEL-07-02 Design `bulk-change.md`): Change selected… in the tree's
// menu opens Bulk change on the selection with focus; the Bulk change tab is the button route;
// a queued bulk change reaches Batch review as one batch, applies once and one Undo reverts it.
// Uses the bundled preview model.

const initialInnerWidth = window.innerWidth;
afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  Object.defineProperty(window, "innerWidth", { configurable: true, value: initialInnerWidth });
  window.localStorage.clear();
});

async function renderShell() {
  Object.defineProperty(window, "innerWidth", { configurable: true, value: 1440 });
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
}

function pipeRows(): HTMLElement[] {
  const tree = screen.getByTestId("model-tree-virtual");
  return within(tree).getAllByRole("treeitem").filter((row) => row.dataset.testid?.startsWith("tree-row-pipe-"));
}

describe("Bulk change in the shell", () => {
  it("opens from the tree's Change selected… on a range, queues one batch, applies it once and Undo reverts it", async () => {
    await renderShell();
    const [first, second] = pipeRows();
    fireEvent.click(first);
    fireEvent.click(second, { shiftKey: true });
    fireEvent.contextMenu(pipeRows()[1]);
    const menu = await screen.findByRole("menu");
    fireEvent.click(within(menu).getByTestId("tree-context-menu-item-change-selected"));
    const panel = await screen.findByTestId("bulk-change");
    await waitFor(() => expect(panel).toHaveFocus());
    expect(screen.getByTestId("operation-tab-bulk")).toHaveAttribute("aria-pressed", "true");
    expect(within(panel).getByTestId("bulk-change-scope")).toHaveTextContent("2 selected entities");

    fireEvent.change(within(panel).getByTestId("bulk-change-kind"), { target: { value: "tag" } });
    fireEvent.change(within(panel).getByTestId("bulk-change-tag"), { target: { value: "b2-what-if" } });
    expect(within(panel).getByTestId("bulk-change-count")).toHaveTextContent("2 entities affected; 0 unchanged; 0 not matched; 0 refused.");
    fireEvent.click(within(panel).getByTestId("bulk-change-queue"));

    const review = await screen.findByTestId("batch-review-summary");
    await waitFor(() => expect(review).toHaveTextContent("1 queued batches; 0 batches applied this session."));
    const batch = screen.getByText("bulk-tag-1").closest("article")!;
    expect(batch).toHaveTextContent("2 proposed changes.");
    fireEvent.click(within(batch).getByRole("button", { name: "Apply entire batch" }));
    await waitFor(() => expect(screen.getByTestId("batch-review-summary")).toHaveTextContent("0 queued batches; 1 batches applied this session."));
    expect(screen.getByTestId("batch-review-message")).toHaveTextContent("Batch applied once.");

    fireEvent.click(screen.getByTestId("operation-tab-bulk"));
    fireEvent.change(screen.getByTestId("bulk-change-kind"), { target: { value: "tag" } });
    fireEvent.change(screen.getByTestId("bulk-change-tag"), { target: { value: "b2-what-if" } });
    // Applied: both pipes now carry the tag.
    expect(screen.getByTestId("bulk-change-count")).toHaveTextContent("0 entities affected; 2 unchanged; 0 not matched; 0 refused.");
    fireEvent.click(screen.getByTestId("operation-tab-review"));
    fireEvent.click(screen.getByTestId("undo-session-model-edit"));
    fireEvent.click(screen.getByTestId("operation-tab-bulk"));
    // One Undo reverts the whole batch. The checkpoint restores the primary selection, the
    // second pipe, which no longer carries the tag.
    await waitFor(() => expect(screen.getByTestId("bulk-change-scope")).toHaveTextContent("1 selected entity"));
    expect(screen.getByTestId("bulk-change-count")).toHaveTextContent("1 entity affected; 0 unchanged; 0 not matched; 0 refused.");
    expect(screen.getByTestId("undo-session-model-edit")).toBeDisabled();
  });

  // Keyboard alone: Shift+F10 or the ContextMenu key opens the menu, End then ArrowUp reaches
  // Change selected… (Queue removal stays last), and Enter opens the panel on the selection.
  function activateChangeSelected(invoker: HTMLElement, key: { key: string; shiftKey?: boolean }) {
    invoker.focus();
    fireEvent.keyDown(invoker, key);
    const menu = screen.getByRole("menu");
    fireEvent.keyDown(menu, { key: "End" });
    fireEvent.keyDown(menu, { key: "ArrowUp" });
    expect(within(menu).getByText("Change selected…").closest("[role='menuitem']")).toHaveFocus();
    fireEvent.keyDown(menu, { key: "Enter" });
  }
  async function expectPanel(scopeText: string) {
    const panel = await screen.findByTestId("bulk-change");
    await waitFor(() => expect(panel).toHaveFocus());
    expect(within(panel).getByTestId("bulk-change-scope")).toHaveTextContent(scopeText);
  }
  function nodeRows(): HTMLElement[] {
    return within(screen.getByTestId("model-tree-virtual")).getAllByRole("treeitem").filter((row) => row.dataset.testid?.startsWith("tree-row-node-"));
  }
  async function twoNodesInTheTable() {
    await renderShell();
    const [first, second] = nodeRows();
    const ids = [first, second].map((row) => decodeURIComponent(row.dataset.testid!.replace("tree-row-node-", "")));
    fireEvent.click(first);
    fireEvent.click(second, { shiftKey: true });
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
    return ids;
  }

  it("from an engineering-table cell with Shift+F10 and Enter, on the cell's row", async () => {
    const [first] = await twoNodesInTheTable();
    const cell = screen.getAllByRole("gridcell").map((gridcell) => gridcell.querySelector<HTMLElement>("[data-table-cell]")).find((button) => button?.dataset.rowKey?.includes(first))!;
    // Existing table behaviour: keyboard focus on a cell makes its row the selection, so the
    // panel acts on that one node.
    activateChangeSelected(cell, { key: "F10", shiftKey: true });
    await expectPanel("1 selected entity");
    fireEvent.change(screen.getByTestId("bulk-change-kind"), { target: { value: "tag" } });
    fireEvent.change(screen.getByTestId("bulk-change-tag"), { target: { value: "b2-key" } });
    expect(within(screen.getByTestId("bulk-change-affected")).getAllByRole("row").slice(1).map((row) => row.firstElementChild?.textContent)).toEqual([first]);
  });

  it("from an engineering-table row header with the ContextMenu key and Enter", async () => {
    const [first] = await twoNodesInTheTable();
    const header = within(screen.getAllByRole("rowheader").find((rowheader) => rowheader.textContent?.includes(first))!).getByRole("button");
    activateChangeSelected(header, { key: "ContextMenu" });
    await expectPanel("2 selected entities");
  });

  it("from a viewport label with Shift+F10 and Enter", async () => {
    await renderShell();
    const [first, second] = pipeRows();
    fireEvent.click(first);
    fireEvent.click(second, { shiftKey: true });
    const id = decodeURIComponent(second.dataset.testid!.replace("tree-row-pipe-", ""));
    activateChangeSelected(within(screen.getByTestId("viewport-selection-layer")).getByTestId(`viewport-select-${id}`), { key: "F10", shiftKey: true });
    await expectPanel("2 selected entities");
  });

  it("from the viewport canvas with the ContextMenu key and Enter, for the selection", async () => {
    await renderShell();
    const [first, second] = pipeRows();
    fireEvent.click(first);
    fireEvent.click(second, { shiftKey: true });
    activateChangeSelected(screen.getByTestId("viewport-canvas"), { key: "ContextMenu" });
    await expectPanel("2 selected entities");
  });
});

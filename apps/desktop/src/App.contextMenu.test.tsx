import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import { useModelledVisibility } from "./test-support/modelledVisibility";

useModelledVisibility();

// B9 through the product: the tree's context menu reaches the mounted viewport's own view
// commands through the shell (App passes the viewport's dispatcher to the tree), and its
// removal reaches the shell's pending-changes queue. Uses the bundled preview model.

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

function firstPipeRow(): HTMLElement {
  const tree = screen.getByTestId("model-tree-virtual");
  return within(tree).getAllByRole("treeitem").find((row) => row.dataset.testid?.startsWith("tree-row-pipe-"))!;
}

describe("the tree context menu in the shell", () => {
  it("hides through the viewport's Hide and shows again through its Show all", async () => {
    await renderShell();
    const row = firstPipeRow();
    fireEvent.contextMenu(row);
    let menu = await screen.findByRole("menu");
    fireEvent.click(within(menu).getByTestId("tree-context-menu-item-hide-selection"));
    await waitFor(() => expect(firstPipeRow()).toHaveTextContent("Hidden"));
    expect(screen.getByTestId("viewport-view-command-status")).toHaveTextContent("1 selected item hidden.");

    fireEvent.contextMenu(firstPipeRow());
    menu = await screen.findByRole("menu");
    const showAll = within(menu).getByTestId("tree-context-menu-item-show-all");
    expect(showAll).not.toHaveAttribute("aria-disabled");
    fireEvent.click(showAll);
    await waitFor(() => expect(firstPipeRow()).not.toHaveTextContent("Hidden"));
  });

  it("queues the selected pipe's removal for review in Pending changes", async () => {
    await renderShell();
    const row = firstPipeRow();
    fireEvent.contextMenu(row);
    const menu = await screen.findByRole("menu");
    fireEvent.click(within(menu).getByTestId("tree-context-menu-item-queue-removal"));
    // The same row the inspector's Delete pipe panel queues, for the same review and Apply.
    const queued = await within(screen.getByTestId("workspace-section-operations")).findByTestId("operation-apply-row-editor-intent-1");
    expect(queued).toHaveTextContent("not_present");
    expect(within(screen.getByTestId("workspace-section-operations")).getByTestId("operation-apply-summary")).toHaveTextContent("1 queued; 0 applied");
  });

  it("from the keyboard, queuing a removal hides the tree and moves focus to the Pending changes page that replaced it", async () => {
    await renderShell();
    const tree = screen.getByTestId("model-tree-virtual");
    fireEvent.contextMenu(firstPipeRow());
    const menu = await screen.findByRole("menu");
    fireEvent.keyDown(menu, { key: "End" });
    fireEvent.keyDown(menu, { key: "Enter" });
    await within(screen.getByTestId("workspace-section-operations")).findByTestId("operation-apply-row-editor-intent-1");
    expect(tree.closest(".inactive")).not.toBeNull();
    expect(document.activeElement).not.toBe(document.body);
    expect(document.activeElement?.closest(".inactive")).toBeNull();
    expect(screen.getByTestId("workspace-section-operations")).toHaveFocus();
  });
});

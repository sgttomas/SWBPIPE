import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import * as previewService from "./services/previewService";
import { splitPartsModel } from "./test-support/splitPartsModel";
import { useModelledVisibility } from "./test-support/modelledVisibility";

useModelledVisibility();

// B5 through the product (DEL-07-01 Design `split-and-slope-entry.md`): Split into N… in the
// tree's and the viewport's menus opens Split into equal parts on the pipe with focus; the queued
// split reaches Batch review as one batch, applies once and one Undo reverts it. The viewport's
// new route end takes the computed end point exactly. Invented model (project unit ft).

const initialInnerWidth = window.innerWidth;
beforeEach(() => { vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(splitPartsModel()); });
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
const treeRow = (type: string, id: string) => screen.queryByTestId(`tree-row-${type}-${encodeURIComponent(id)}`);

async function fillAndQueue(parts: string) {
  const panel = await screen.findByTestId("split-into-parts");
  await waitFor(() => expect(within(panel).getByLabelText("Number of equal parts")).toHaveFocus());
  expect(screen.getByTestId("operation-tab-geometry")).toHaveAttribute("aria-pressed", "true");
  expect(within(panel).getByTestId("split-parts-pipe").querySelector(".virtual-target-picker-current")).toHaveTextContent("pipe:H");
  const enter = (label: string, value: string) => fireEvent.change(within(panel).getByLabelText(label), { target: { value } });
  enter("Number of equal parts", parts);
  enter("New node ID pattern", "node:H{k}");
  enter("New node label pattern", "H{k}");
  enter("New pipe ID pattern", "pipe:H{k}");
  enter("New pipe label pattern", "H{k}");
  enter("Provenance", "invented split");
  await waitFor(() => expect(within(panel).getByTestId("split-parts-queue")).toBeEnabled());
  fireEvent.click(within(panel).getByTestId("split-parts-queue"));
}

describe("Split into N… in the shell", () => {
  it("from the tree's menu: one batch, applied once, one Undo reverts it", async () => {
    await renderShell();
    fireEvent.click(treeRow("pipe", "pipe:H")!);
    fireEvent.contextMenu(treeRow("pipe", "pipe:H")!);
    const menu = await screen.findByRole("menu");
    fireEvent.click(within(menu).getByTestId("tree-context-menu-item-split-into-parts"));
    await fillAndQueue("4");
    const review = await screen.findByTestId("batch-review-summary");
    await waitFor(() => expect(review).toHaveTextContent("1 queued batches; 0 batches applied this session."));
    const batch = screen.getByText("split-parts-1").closest("article")!;
    fireEvent.click(within(batch).getByRole("button", { name: "Apply entire batch" }));
    await waitFor(() => expect(screen.getByTestId("batch-review-summary")).toHaveTextContent("0 queued batches; 1 batches applied this session."));
    for (const id of ["node:H1", "node:H2", "node:H3"]) expect(treeRow("node", id)).toBeInTheDocument();
    fireEvent.click(screen.getByTestId("undo-session-model-edit"));
    await waitFor(() => expect(treeRow("node", "node:H1")).not.toBeInTheDocument());
    expect(treeRow("node", "node:H3")).not.toBeInTheDocument();
    expect(screen.getByTestId("undo-session-model-edit")).toBeDisabled();
  });

  function activate(invoker: HTMLElement, key: { key: string; shiftKey?: boolean }) {
    invoker.focus();
    fireEvent.keyDown(invoker, key);
    const menu = screen.getByRole("menu");
    const item = within(menu).getByText("Split into N…").closest("[role='menuitem']") as HTMLElement;
    for (let i = 0; i < 12 && document.activeElement !== item; i++) fireEvent.keyDown(menu, { key: "ArrowDown" });
    expect(item).toHaveFocus();
    fireEvent.keyDown(menu, { key: "Enter" });
  }

  it("from a tree row with Shift+F10 and Enter", async () => {
    await renderShell();
    fireEvent.click(treeRow("pipe", "pipe:H")!);
    activate(treeRow("pipe", "pipe:H")!, { key: "F10", shiftKey: true });
    await fillAndQueue("2");
    expect(await screen.findByTestId("split-parts-message")).toHaveTextContent("1 operations queued as one batch for review.");
  });

  it("from a viewport label with the ContextMenu key and Enter", async () => {
    await renderShell();
    fireEvent.click(treeRow("pipe", "pipe:H")!);
    activate(within(screen.getByTestId("viewport-selection-layer")).getByTestId("viewport-select-pipe:H"), { key: "ContextMenu" });
    await fillAndQueue("2");
  });

  it("is disabled, with the B9 reason, on a pipe that is not selected", async () => {
    await renderShell();
    fireEvent.click(treeRow("node", "node:A")!);
    const label = within(screen.getByTestId("viewport-selection-layer")).getByTestId("viewport-select-pipe:H");
    label.focus();
    fireEvent.keyDown(label, { key: "ContextMenu" });
    const item = within(screen.getByRole("menu")).getByText("Split into N…").closest("[role='menuitem']")!;
    expect(item).toHaveAttribute("aria-disabled", "true");
    expect(screen.getByRole("menu")).toHaveTextContent("This acts on the selection, which does not include pipe:H. Select it first.");
  });
});

describe("the viewport's new route end from a length and direction", () => {
  it("fills the typed coordinates with the computed end point in the project unit, exactly as displayed", async () => {
    await renderShell();
    fireEvent.click(screen.getByTestId("command-pipe"));
    const panel = screen.getByTestId("viewport-editor-intents");
    const picker = within(panel).getByTestId("viewport-create-pipe-from");
    fireEvent.change(within(picker).getByRole("combobox"), { target: { value: "node:E" } });
    fireEvent.click(within(picker).getByRole("option", { name: /node:E$/ }));
    fireEvent.click(within(panel).getByLabelText("New node"));
    const R = "Route end";
    fireEvent.change(within(panel).getByLabelText(`${R}: Entry method`), { target: { value: "angle" } });
    for (const [label, value] of [["Length along the element", "2.71"], ["Length unit", "m"], ["Horizontal direction X", "0"], ["Horizontal direction Y", "1"],
      ["Horizontal direction Z", "0"], ["Up direction", "+z"], ["Angle from the horizontal (positive rises)", "7.5"], ["Angle unit", "deg"]]) {
      fireEvent.change(within(panel).getByLabelText(`${R}: ${label}`), { target: { value } });
    }
    const use = within(panel).getByRole("button", { name: "Use these coordinates for the route end" });
    await waitFor(() => expect(use).toBeEnabled());
    const readout = within(panel).getByLabelText(`${R}: Computed end point`).textContent!;
    expect(readout).toMatch(/^Computed end point \(ft\): X 32\.5, Y /);
    const [, x, y, z] = /X (\S+), Y (\S+), Z (\S+)$/.exec(readout)!;
    fireEvent.click(use);
    expect(within(panel).getByTestId("viewport-route-end-x")).toHaveValue(x);
    expect(within(panel).getByTestId("viewport-route-end-y")).toHaveValue(y);
    expect(within(panel).getByTestId("viewport-route-end-z")).toHaveValue(z);
    // The coordinate unit becomes the project unit, so create_node stores the numbers unchanged.
    expect(within(panel).getByTestId("viewport-route-end-unit")).toHaveTextContent("ft");
  });
});

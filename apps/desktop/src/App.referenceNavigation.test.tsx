import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { App } from "./App";
import { setSurfaceVisibilityProbeForTests } from "./features/workspace/surfaceReveal";
import { changeFormControl } from "./test-support/workspaceTestControls";

// jsdom does no layout, so each case models the browser's visibility as selectionReveal.test.tsx does:
// attached, not under [hidden] or [inert], and not in a pane the shell hides (`.inactive`).
beforeEach(() => setSurfaceVisibilityProbeForTests((element) => element.isConnected && !element.closest("[hidden], [inert], .inactive")));
afterEach(() => {
  setSurfaceVisibilityProbeForTests(null);
  vi.restoreAllMocks();
});

const rowId = (type: string, id: string) => `tree-row-${encodeURIComponent(type)}-${encodeURIComponent(id)}`;
const tree = () => screen.getByLabelText("Model tree");
const inspector = () => screen.getByTestId("property-inspector");

/** Selects pipe:P-110 in the bundled invented demo model, with the Nodes group collapsed and a task draft pinned to the pipe. */
async function pipeWithDraftAndCollapsedNodes() {
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  const nodes = within(tree()).getByTestId("tree-group-Nodes");
  if (nodes.getAttribute("aria-expanded") === "true") fireEvent.click(nodes);
  expect(within(tree()).getByTestId("tree-group-Nodes")).toHaveAttribute("aria-expanded", "false");
  expect(within(tree()).queryByTestId(rowId("node", "node:N-110"))).toBeNull();
  const pipe = await within(tree()).findByTestId(rowId("pipe", "pipe:P-110"));
  fireEvent.click(pipe);
  fireEvent.click(within(inspector()).getByRole("tab", { name: "Task" }));
  fireEvent.click(within(inspector()).getByTestId("inspector-start-task"));
  changeFormControl(within(inspector()).getByTestId("editor-intent-field"), { target: { value: "label" } });
  fireEvent.change(within(inspector()).getByTestId("editor-intent-value"), { target: { value: "Draft riser label" } });
  fireEvent.click(within(inspector()).getByRole("tab", { name: "Properties" }));
}

function expectDraftStillPinnedToPipe() {
  fireEvent.click(within(inspector()).getByRole("tab", { name: "Task" }));
  expect(within(inspector()).getByTestId("inspector-frozen-task-target")).toHaveTextContent("Draft target: pipe: pipe:P-110");
  expect(within(inspector()).getByTestId("editor-intent-value")).toHaveValue("Draft riser label");
}

describe("inspector references through the App selection route", () => {
  it("selects a node reference, reveals its row in the collapsed group and keeps the pinned draft", async () => {
    await pipeWithDraftAndCollapsedNodes();
    const overview = within(inspector()).getByTestId("inspector-overview");
    fireEvent.click(within(overview).getByRole("button", { name: "Select node Low point elbow (node:N-110)" }));

    expect(within(inspector()).getByRole("heading", { level: 2 })).toHaveTextContent("Low point elbow — node:N-110");
    // The pressed reference has gone; focus stays in the inspector, on its Properties panel, not the tree.
    expect(document.activeElement).toBe(within(inspector()).getByRole("tabpanel"));
    await waitFor(() => expect(within(tree()).getByTestId("tree-group-Nodes")).toHaveAttribute("aria-expanded", "true"));
    expect(await within(tree()).findByTestId(rowId("node", "node:N-110"))).toHaveAttribute("aria-selected", "true");
    expect(within(tree()).getByTestId(rowId("pipe", "pipe:P-110"))).toHaveAttribute("aria-selected", "false");
    expectDraftStillPinnedToPipe();
  });

  it("selects a material reference and leaves an active tree filter as the user set it", async () => {
    await pipeWithDraftAndCollapsedNodes();
    fireEvent.change(within(tree()).getByTestId("model-tree-filter-input"), { target: { value: "P-110" } });
    const summary = within(tree()).getByTestId("model-tree-filter-summary").textContent;
    const overview = within(inspector()).getByTestId("inspector-overview");
    fireEvent.click(within(overview).getByRole("button", { name: /^Select material .* \(material:invented-carbon-steel\)$/u }));

    expect(within(inspector()).getByRole("heading", { level: 2 })).toHaveTextContent(/ — material:invented-carbon-steel$/u);
    expect(within(tree()).getByTestId("model-tree-filter-input")).toHaveValue("P-110");
    expect(within(tree()).getByTestId("model-tree-filter-summary").textContent).toBe(summary);
    expect(within(tree()).queryByTestId(rowId("material", "material:invented-carbon-steel"))).toBeNull();
    expectDraftStillPinnedToPipe();
  });
});

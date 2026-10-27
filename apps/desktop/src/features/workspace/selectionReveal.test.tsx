// B8: a selection made on one surface (the viewport, the tree, the grid, a result row) is
// shown on the others. The shared selection already highlights every surface; these tests
// cover the reveal: the tree opens and scrolls to the selected row, the grid scrolls to it,
// and the results table can follow the selected entity. Invented fixtures only.
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { EntityRef, MechanicsResult, PreviewModel } from "../../types";
import { ModelTree } from "../model-tree/ModelTree";
import { ResultsPanel } from "../results/ResultsPanel";
import { currentCarrierHeader } from "../../test-support/currentCarrierHeader";
import { entityKey } from "./selectionState";

function nodesModel(count: number, pipes = 1): PreviewModel {
  const nodes = Array.from({ length: count }, (_, index) => ({
    id: `n/${index}`, label: `Node ${index}`, position: { x: index, y: 0, z: 0 }, provenance: "invented"
  }));
  const pipeSegments = Array.from({ length: pipes }, (_, index) => ({
    id: `p/${index}`, label: `Pipe ${index}`, from: `n/${index}`, to: `n/${index + 1}`, section: {}, material: "", provenance: "invented"
  }));
  return {
    project: { id: "p", name: "Invented", description: "", units: { length: "m", angle: "rad" } },
    schema_version: "0.2.0",
    analysis_status: { mechanics: "NOT_RUN", rule_check: "NOT_RUN", professional_acceptance: "NOT_REVIEWED" },
    materials: [], sections: [], nodes, pipe_segments: pipeSegments,
    supports: [], components: [], load_cases: [], combinations: [], diagnostics: []
  } as unknown as PreviewModel;
}

function tree(model: PreviewModel, selection: EntityRef, onFocusChange = vi.fn()) {
  return <ModelTree model={model} onSelect={vi.fn()} onFocusChange={onFocusChange} selection={selection} />;
}

afterEach(() => {
  vi.restoreAllMocks();
});

describe("the tree reveals a selection made elsewhere", () => {
  it("scrolls a far row into view and makes it the tree's current row", () => {
    const model = nodesModel(160);
    const onFocusChange = vi.fn();
    const view = render(tree(model, { type: "node", id: "n/0" }, onFocusChange));
    const list = screen.getByTestId("model-tree-virtual");
    expect(list.scrollTop).toBe(0);
    expect(within(list).queryByTestId("tree-row-node-n%2F150")).toBeNull();

    view.rerender(tree(model, { type: "node", id: "n/150" }, onFocusChange));

    expect(list.scrollTop).toBeGreaterThan(0);
    expect(within(list).getByTestId("tree-row-node-n%2F150")).toHaveClass("focused");
    expect(within(list).getByTestId("tree-row-node-n%2F150")).toHaveAttribute("aria-selected", "true");
    expect(onFocusChange).toHaveBeenLastCalledWith(entityKey({ type: "node", id: "n/150" }));
    // Reveal never moves DOM focus.
    expect(document.activeElement).toBe(document.body);
  });

  it("opens a collapsed group to show the selected row", () => {
    const model = nodesModel(4);
    const view = render(tree(model, { type: "node", id: "n/0" }));
    const nodesGroup = screen.getAllByRole("treeitem").find((row) => row.textContent?.includes("Nodes") && row.hasAttribute("aria-expanded"))!;
    fireEvent.click(nodesGroup);
    expect(nodesGroup).toHaveAttribute("aria-expanded", "false");
    expect(screen.queryByTestId("tree-row-node-n%2F2")).toBeNull();
    act(() => (document.activeElement as HTMLElement | null)?.blur());

    view.rerender(tree(model, { type: "node", id: "n/2" }));

    expect(screen.getByTestId("tree-row-node-n%2F2")).toHaveClass("focused");
    expect(screen.getAllByRole("treeitem").find((row) => row.textContent?.includes("Nodes") && row.hasAttribute("aria-expanded")))
      .toHaveAttribute("aria-expanded", "true");
  });

  it("leaves the tree's position alone while focus is in the tree", () => {
    const model = nodesModel(160);
    const view = render(tree(model, { type: "node", id: "n/0" }));
    const list = screen.getByTestId("model-tree-virtual");
    act(() => list.focus());

    view.rerender(tree(model, { type: "node", id: "n/150" }));

    expect(list.scrollTop).toBe(0);
    expect(within(list).queryByTestId("tree-row-node-n%2F150")).toBeNull();
  });
});

describe("the grid reveals a selection made elsewhere", () => {
  it("scrolls the node table to the selected row without moving focus", () => {
    const model = nodesModel(160);
    const view = render(tree(model, { type: "node", id: "n/0" }));
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    act(() => (document.activeElement as HTMLElement | null)?.blur());
    const rows = screen.getAllByTestId("engineering-table-rows")[0];
    expect(rows.scrollTop).toBe(0);

    view.rerender(tree(model, { type: "node", id: "n/150" }));

    expect(rows.scrollTop).toBeGreaterThan(0);
    const selected = within(rows).getAllByRole("row").filter((row) => row.getAttribute("aria-selected") === "true");
    expect(selected).toHaveLength(1);
    expect(selected[0].textContent).toContain("n/150");
    expect(document.activeElement).toBe(document.body);
  });

  it("scrolls a bulk family's selected row into view", () => {
    const model = nodesModel(6, 5);
    const scrolled: Element[] = [];
    Object.defineProperty(Element.prototype, "scrollIntoView", {
      configurable: true, writable: true, value: function (this: Element) { scrolled.push(this); }
    });
    try {
      const view = render(tree(model, { type: "pipe", id: "p/0" }));
      fireEvent.click(screen.getByTestId("layout-mode-grid"));
      act(() => (document.activeElement as HTMLElement | null)?.blur());
      scrolled.length = 0;

      view.rerender(tree(model, { type: "pipe", id: "p/4" }));

      const row = screen.getByTestId("entity-grid-row-pipe-p-4").closest("tr")!;
      expect(row).toHaveClass("active");
      expect(scrolled).toContain(row);
    } finally {
      delete (Element.prototype as { scrollIntoView?: unknown }).scrollIntoView;
    }
  });
});

describe("the virtual bulk grid (review r1, #1667)", () => {
  function pipeGrid(selection: string) {
    return tree(nodesModel(161, 160), { type: "pipe", id: selection });
  }
  function openGrid() {
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    act(() => (document.activeElement as HTMLElement | null)?.blur());
    return screen.getByTestId("entity-grid-virtual-rows");
  }

  it("scrolls to an outside selection once", () => {
    const view = render(pipeGrid("p/0"));
    const list = openGrid();
    expect(list.scrollTop).toBe(0);
    view.rerender(pipeGrid("p/150"));
    expect(list.scrollTop).toBeGreaterThan(0);
    expect(screen.getByTestId("entity-grid-row-pipe-p-150")).toHaveAttribute("aria-pressed", "true");
  });

  // Review r2 (#1667): the reveal waits for the list that holds the selected row.
  it("reveals a selection made before the grid is first opened", () => {
    const view = render(pipeGrid("p/0"));
    view.rerender(pipeGrid("p/150"));
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    expect(screen.getByTestId("entity-grid-virtual-rows").scrollTop).toBeGreaterThan(0);
    expect(screen.getByTestId("entity-grid-row-pipe-p-150")).toBeInTheDocument();
  });

  it("reveals a selection that switches the grid from nodes to pipes", () => {
    const model = nodesModel(161, 160);
    const view = render(tree(model, { type: "node", id: "n/0" }));
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    act(() => (document.activeElement as HTMLElement | null)?.blur());
    view.rerender(tree(model, { type: "pipe", id: "p/150" }));
    expect(screen.getByTestId("entity-grid-virtual-rows").scrollTop).toBeGreaterThan(0);
    expect(screen.getByTestId("entity-grid-row-pipe-p-150")).toBeInTheDocument();
  });

  it("reveals the last of rapid selections and keeps no index after a manual scroll", () => {
    const model = nodesModel(161, 160);
    const view = render(tree(model, { type: "pipe", id: "p/0" }));
    const list = openGrid();
    view.rerender(tree(model, { type: "pipe", id: "p/150" }));
    view.rerender(tree(model, { type: "pipe", id: "p/20" }));
    view.rerender(tree(model, { type: "pipe", id: "p/140" }));
    expect(screen.getByTestId("entity-grid-row-pipe-p-140")).toBeInTheDocument();
    list.scrollTop = 0;
    fireEvent.scroll(list);
    view.rerender(tree({ ...model, pipe_segments: [...model.pipe_segments].reverse() }, { type: "pipe", id: "p/140" }));
    expect(list.scrollTop).toBe(0);
  });

  it("keeps its position while an edit has focus in the grid", () => {
    const view = render(pipeGrid("p/0"));
    const list = openGrid();
    const input = list.querySelector("input") as HTMLInputElement;
    expect(input).not.toBeNull();
    act(() => input.focus());
    view.rerender(pipeGrid("p/150"));
    expect(list.scrollTop).toBe(0);
    expect(document.activeElement).toBe(input);
  });
});

// Review r3 (#1667): a reveal asked for while its surface is hidden is done when the surface is
// shown. jsdom does no layout and stores any scrollTop; a browser does not scroll an element under
// [hidden] (display: none). These tests restore that browser behaviour for the duration.
describe("a reveal asked for while the surface is hidden", () => {
  const native = Object.getOwnPropertyDescriptor(Element.prototype, "scrollTop")!;
  beforeEach(() => {
    Object.defineProperty(Element.prototype, "scrollTop", {
      configurable: true,
      get(this: Element) { return native.get!.call(this); },
      set(this: Element, value: number) { if (!this.closest("[hidden]")) native.set!.call(this, value); }
    });
  });
  afterEach(() => {
    Object.defineProperty(Element.prototype, "scrollTop", native);
  });
  const model = nodesModel(161, 160);
  const show = (layout: "grid" | "tree") => {
    fireEvent.click(screen.getByTestId(`layout-mode-${layout}`));
    act(() => (document.activeElement as HTMLElement | null)?.blur());
  };

  it("reveals in the pipe grid a selection made while the Tree layout was shown", () => {
    const view = render(tree(model, { type: "pipe", id: "p/0" }));
    show("grid");
    show("tree");
    view.rerender(tree(model, { type: "pipe", id: "p/150" }));
    show("grid");
    expect(screen.getByTestId("entity-grid-virtual-rows").scrollTop).toBeGreaterThan(0);
    expect(screen.getByTestId("entity-grid-row-pipe-p-150")).toBeInTheDocument();
  });

  it("reveals a selection a filter hid, once the filter is cleared with the grid closed and the grid reopens", () => {
    const view = render(tree(model, { type: "pipe", id: "p/0" }));
    show("grid");
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "p/0" } });
    act(() => (document.activeElement as HTMLElement | null)?.blur());
    view.rerender(tree(model, { type: "pipe", id: "p/150" }));
    show("tree");
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "" } });
    act(() => (document.activeElement as HTMLElement | null)?.blur());
    show("grid");
    expect(screen.getByTestId("entity-grid-virtual-rows").scrollTop).toBeGreaterThan(0);
    expect(screen.getByTestId("entity-grid-row-pipe-p-150")).toBeInTheDocument();
  });

  it("reveals in the node table a selection made while the Tree layout was shown", () => {
    const view = render(tree(model, { type: "node", id: "n/0" }));
    show("grid");
    show("tree");
    view.rerender(tree(model, { type: "node", id: "n/150" }));
    show("grid");
    const rows = screen.getAllByTestId("engineering-table-rows")[0];
    expect(rows.scrollTop).toBeGreaterThan(0);
    expect(within(rows).getAllByRole("row").some((row) => row.getAttribute("aria-selected") === "true" && row.textContent?.includes("n/150"))).toBe(true);
  });

  it("reveals in the tree a selection made while the Grid layout was shown", () => {
    const view = render(tree(model, { type: "node", id: "n/0" }));
    show("grid");
    view.rerender(tree(model, { type: "node", id: "n/150" }));
    show("tree");
    const list = screen.getByTestId("model-tree-virtual");
    expect(list.scrollTop).toBeGreaterThan(0);
    expect(within(list).getByTestId("tree-row-node-n%2F150")).toHaveClass("focused");
  });
});

describe("the results table follows the selected entity", () => {
  const result = {
    ...currentCarrierHeader(), run_id: "invented", model_ref: "invented",
    status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" },
    summary: {}, diagnostics: [],
    results: [
      { id: "r:a", kind: "global_nodal_rotation_x", value: 0.1, unit: "rad", entity_ref: "node:A" },
      { id: "r:b", kind: "global_nodal_rotation_x", value: 0.2, unit: "rad", entity_ref: "node:B" },
      { id: "r:b2", kind: "global_nodal_rotation_y", value: 0.3, unit: "rad", entity_ref: "node:B" }
    ]
  } as unknown as MechanicsResult;
  const panel = (selectedEntity: EntityRef | null) => (
    <ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} selectedEntity={selectedEntity} />
  );

  it("shows only the selected entity's rows while the toggle is on, and follows a new selection", () => {
    const view = render(panel({ type: "node", id: "node:A" }));
    const toggle = screen.getByTestId("result-selection-only");
    expect(toggle).toHaveAttribute("aria-pressed", "false");
    expect(screen.getByTestId("result-row-r:b")).toBeInTheDocument();

    // Keyboard: the toggle is a native button, reachable by Tab and pressed with Enter or Space.
    toggle.focus();
    expect(document.activeElement).toBe(toggle);
    fireEvent.click(toggle);
    expect(toggle).toHaveAttribute("aria-pressed", "true");
    expect(toggle.textContent).toContain("node:A");
    expect(screen.getByTestId("result-row-r:a")).toBeInTheDocument();
    expect(screen.queryByTestId("result-row-r:b")).toBeNull();

    view.rerender(panel({ type: "node", id: "node:B" }));
    expect(screen.queryByTestId("result-row-r:a")).toBeNull();
    expect(screen.getByTestId("result-row-r:b")).toBeInTheDocument();
    expect(screen.getByTestId("result-row-r:b2")).toBeInTheDocument();
    expect(screen.getByTestId("result-filter-summary").textContent).toContain("2 of 3");

    fireEvent.click(toggle);
    expect(screen.getByTestId("result-row-r:a")).toBeInTheDocument();
  });

  it("shows no rows, and says so, when the selection is cleared while the toggle is on (review r1, #1667)", () => {
    const view = render(panel({ type: "node", id: "node:A" }));
    fireEvent.click(screen.getByTestId("result-selection-only"));
    view.rerender(panel({ type: "project", id: "p" }));
    const toggle = screen.getByTestId("result-selection-only");
    expect(toggle).toHaveAttribute("aria-pressed", "true");
    expect(toggle).toBeEnabled();
    expect(screen.queryByTestId("result-row-r:a")).toBeNull();
    expect(screen.queryByTestId("result-row-r:b")).toBeNull();
    expect(screen.getByTestId("result-selection-only-empty")).toHaveTextContent("No model entity is selected, so no rows are shown.");
    expect(screen.getByTestId("result-filter-summary").textContent).toContain("0 of 3");
    view.rerender(panel(null));
    expect(screen.queryByTestId("result-row-r:a")).toBeNull();
    fireEvent.click(toggle);
    expect(screen.queryByTestId("result-selection-only-empty")).toBeNull();
    expect(screen.getByTestId("result-row-r:a")).toBeInTheDocument();
  });

  it("says the other filters still apply when the selection restriction is turned off (review r2, #1667)", () => {
    const view = render(panel({ type: "node", id: "node:A" }));
    fireEvent.change(screen.getByTestId("result-filter-input"), { target: { value: "absent" } });
    fireEvent.click(screen.getByTestId("result-selection-only"));
    view.rerender(panel({ type: "project", id: "p" }));
    const message = screen.getByTestId("result-selection-only-empty");
    expect(message).not.toHaveTextContent("show every row");
    expect(message).toHaveTextContent("remove the selection restriction; the other table filters still apply.");
    fireEvent.click(screen.getByTestId("result-selection-only"));
    expect(screen.queryByTestId("result-row-r:a")).toBeNull();
    expect(screen.getByTestId("result-filter-summary")).toHaveTextContent("0 of 3");
  });

  it("is unavailable without a selected model entity", () => {
    render(panel({ type: "project", id: "p" }));
    expect(screen.getByTestId("result-selection-only")).toBeDisabled();
  });
});

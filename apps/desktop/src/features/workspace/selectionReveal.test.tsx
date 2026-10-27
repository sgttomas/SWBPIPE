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
import { canSurfaceRevealNow, layoutVisibility, setSurfaceVisibilityProbeForTests } from "./surfaceReveal";

function nodesModel(count: number, pipes = 1, libraries = 0): PreviewModel {
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
    materials: Array.from({ length: libraries }, (_, index) => ({ id: `m/${index}`, label: `Material ${index}`, provenance: "invented" })),
    sections: Array.from({ length: libraries }, (_, index) => ({ id: `s/${index}`, name: `Section ${index}`, section_type: "pipe", properties: {}, provenance: "invented" })),
    nodes, pipe_segments: pipeSegments,
    supports: [], components: [], load_cases: [], combinations: [], diagnostics: []
  } as unknown as PreviewModel;
}

function tree(model: PreviewModel, selection: EntityRef, onFocusChange = vi.fn(), boundedGrid = false) {
  return <ModelTree boundedGrid={boundedGrid} model={model} onSelect={vi.fn()} onFocusChange={onFocusChange} selection={selection} />;
}

// jsdom does no layout: it has no `checkVisibility` and reports no client rectangles, so the
// browser's visibility probe would say no surface can reveal. Each case models the browser's
// visibility instead: attached, not under [hidden] or [inert], and not in a pane the shell's
// stylesheet hides (`.shell-tree-host.inactive` is display: none). The browser itself is
// covered by the review's Chromium probe (#1667 r3 and r4).
const modelledVisibility = (element: Element) => element.isConnected && !element.closest("[hidden], [inert], .inactive");
beforeEach(() => setSurfaceVisibilityProbeForTests(modelledVisibility));
afterEach(() => {
  setSurfaceVisibilityProbeForTests(null);
  vi.restoreAllMocks();
});

// A browser does not scroll an element that is not laid out; jsdom stores any scrollTop.
function withBrowserScrolling() {
  const native = Object.getOwnPropertyDescriptor(Element.prototype, "scrollTop")!;
  beforeEach(() => {
    Object.defineProperty(Element.prototype, "scrollTop", {
      configurable: true,
      get(this: Element) { return native.get!.call(this); },
      set(this: Element, value: number) { if (!this.closest("[hidden], .inactive")) native.set!.call(this, value); }
    });
  });
  afterEach(() => {
    Object.defineProperty(Element.prototype, "scrollTop", native);
  });
}

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
  withBrowserScrolling();
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

// Review r4 (#1667).
describe("whether a surface can reveal now", () => {
  it("asks the browser: attached, not hidden or inert, and laid out", () => {
    const host = document.createElement("div");
    expect(layoutVisibility(host)).toBe(false); // not attached
    document.body.append(host);
    try {
      const check = vi.fn(() => true);
      Object.assign(host, { checkVisibility: check });
      expect(layoutVisibility(host)).toBe(true);
      check.mockReturnValue(false);
      expect(layoutVisibility(host)).toBe(false); // display: none on the element or an ancestor
      check.mockReturnValue(true);
      host.setAttribute("inert", "");
      expect(layoutVisibility(host)).toBe(false);
      host.removeAttribute("inert");
      host.hidden = true;
      expect(layoutVisibility(host)).toBe(false);
      host.hidden = false;
      // Without checkVisibility, a laid-out element has client rectangles.
      Object.assign(host, { checkVisibility: undefined });
      vi.spyOn(host, "getClientRects").mockReturnValue([{}] as unknown as DOMRectList);
      expect(layoutVisibility(host)).toBe(true);
      vi.spyOn(host, "getClientRects").mockReturnValue([] as unknown as DOMRectList);
      expect(layoutVisibility(host)).toBe(false);
    } finally {
      host.remove();
    }
  });

  it("uses the injected probe, and none without a host", () => {
    const host = document.createElement("div");
    setSurfaceVisibilityProbeForTests(() => true);
    expect(canSurfaceRevealNow(host)).toBe(true);
    expect(canSurfaceRevealNow(null)).toBe(false);
    setSurfaceVisibilityProbeForTests(() => false);
    expect(canSurfaceRevealNow(host)).toBe(false);
  });
});

describe("a table opened for the first time shows the selection (review r4, #1667)", () => {
  withBrowserScrolling();
  const model = nodesModel(161, 160, 161);
  const cases = [
    { family: "node", type: "node" as const, id: "n/150", rows: "engineering-table-rows" },
    { family: "material", type: "material" as const, id: "m/150", rows: "material-engineering-table-rows" },
    { family: "section", type: "section" as const, id: "s/150", rows: "section-engineering-table-rows" }
  ];
  for (const bounded of [false, true]) {
    for (const item of cases) {
      it(`${item.family} table${bounded ? ", bounded" : ""}`, () => {
        const view = render(tree(model, { type: item.type, id: `${item.id.slice(0, 2)}0` }, vi.fn(), bounded));
        view.rerender(tree(model, { type: item.type, id: item.id }, vi.fn(), bounded));
        fireEvent.click(screen.getByTestId("layout-mode-grid"));
        const rows = screen.getAllByTestId(item.rows)[0];
        expect(rows.scrollTop).toBeGreaterThan(0);
        expect(within(rows).getAllByRole("row").some((row) => row.getAttribute("aria-selected") === "true" && row.textContent?.includes(item.id))).toBe(true);
      });
    }
  }
});

describe("a pane the stylesheet hides, or an inert or hidden pane (review r4, #1667)", () => {
  withBrowserScrolling();
  const model = nodesModel(161, 160);
  function pane(selection: EntityRef, wrapper: { inactive?: boolean; inert?: boolean; hidden?: boolean }, bounded = false) {
    return (
      <div className={wrapper.inactive ? "shell-tree-host inactive" : "shell-tree-host"} inert={wrapper.inert} hidden={wrapper.hidden}>
        {tree(model, selection, vi.fn(), bounded)}
      </div>
    );
  }
  const open = (layout: "grid" | "tree") => {
    fireEvent.click(screen.getByTestId(`layout-mode-${layout}`));
    act(() => (document.activeElement as HTMLElement | null)?.blur());
  };

  for (const bounded of [false, true]) {
    it(`pipe grid${bounded ? ", bounded" : ""}: reveals when the CSS-hidden pane is shown again`, () => {
      const view = render(pane({ type: "pipe", id: "p/0" }, {}, bounded));
      open("grid");
      view.rerender(pane({ type: "pipe", id: "p/0" }, { inactive: true }, bounded));
      // jsdom gives a bounded list no height, so opening at p/0 may already scroll one row.
      const before = screen.getByTestId("entity-grid-virtual-rows").scrollTop;
      view.rerender(pane({ type: "pipe", id: "p/150" }, { inactive: true }, bounded));
      expect(screen.getByTestId("entity-grid-virtual-rows").scrollTop).toBe(before);
      view.rerender(pane({ type: "pipe", id: "p/150" }, {}, bounded));
      expect(screen.getByTestId("entity-grid-virtual-rows").scrollTop).toBeGreaterThan(100 * 30);
      expect(screen.getByTestId("entity-grid-row-pipe-p-150")).toBeInTheDocument();
    });

    it(`node table${bounded ? ", bounded" : ""}: reveals when the CSS-hidden pane is shown again`, () => {
      const view = render(pane({ type: "node", id: "n/0" }, {}, bounded));
      open("grid");
      view.rerender(pane({ type: "node", id: "n/0" }, { inactive: true }, bounded));
      view.rerender(pane({ type: "node", id: "n/150" }, { inactive: true }, bounded));
      const rows = screen.getAllByTestId("engineering-table-rows")[0];
      const before = rows.scrollTop;
      expect(before).toBeLessThan(100);
      view.rerender(pane({ type: "node", id: "n/150" }, {}, bounded));
      expect(rows.scrollTop).toBeGreaterThan(100 * 30);
      expect(within(rows).getAllByRole("row").some((row) => row.getAttribute("aria-selected") === "true" && row.textContent?.includes("n/150"))).toBe(true);
    });
  }

  for (const state of [{ inactive: true }, { inert: true }, { hidden: true }] as const) {
    const name = Object.keys(state)[0];
    it(`tree: waits while the pane is ${name}, and reveals when it is shown`, () => {
      const view = render(pane({ type: "node", id: "n/0" }, {}));
      view.rerender(pane({ type: "node", id: "n/150" }, state));
      const list = screen.getByTestId("model-tree-virtual");
      expect(list.scrollTop).toBe(0);
      expect(within(list).queryByTestId("tree-row-node-n%2F150")).toBeNull();
      view.rerender(pane({ type: "node", id: "n/150" }, {}));
      expect(list.scrollTop).toBeGreaterThan(0);
      expect(within(list).getByTestId("tree-row-node-n%2F150")).toHaveClass("focused");
    });
  }
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

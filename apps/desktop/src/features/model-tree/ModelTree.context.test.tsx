import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import { ModelTree } from "./ModelTree";

/** Invented model: every entity and value is synthetic test input. */
function inventedModel(): PreviewModel {
  return {
    project: { id: "p", name: "Project", description: "", units: { length: "m" } },
    schema_version: "0.2.0",
    analysis_status: { mechanics: "NOT_RUN", rule_check: "NOT_RUN", professional_acceptance: "NOT_REVIEWED" },
    materials: [], sections: [],
    nodes: [
      { id: "node:A", label: "Node A", position: { x: 0, y: 0, z: 0 }, provenance: "test" },
      { id: "node:B", label: "Node B", position: { x: 1, y: 0, z: 0 }, provenance: "test" },
      { id: "node:C", label: "Node C", position: { x: 2, y: 0, z: 0 }, provenance: "test" }
    ],
    pipe_segments: [{ id: "pipe:AB", label: "Pipe AB", from: "node:A", to: "node:B", section: {}, material: "", provenance: "test" }],
    supports: [
      { id: "support:H", label: "Hanger at B", node: "node:B", family: "variable_spring_hanger", restraints: [], provenance: "test" },
      { id: "support:U", label: "Unknown family", node: "node:A", family: "invented_family", restraints: [], provenance: "test" }
    ],
    components: [{ id: "component:E", label: "Elbow", kind: "bend", node: "node:B", provenance: "test" }],
    load_cases: [], combinations: [], diagnostics: []
  } as unknown as PreviewModel;
}

const row = (type: string, id: string) => screen.getByTestId(`tree-row-${type}-${encodeURIComponent(id)}`);

describe("ModelTree row context and group counts", () => {
  it("states what each entity connects or where it sits, after its exact ID", () => {
    render(<ModelTree model={inventedModel()} onSelect={vi.fn()} selection={{ type: "project", id: "p" }} />);
    expect(row("pipe", "pipe:AB")).toHaveTextContent("pipe:AB · A → B");
    expect(row("node", "node:A")).toHaveTextContent("node:A · 1 pipe");
    expect(row("node", "node:C")).toHaveTextContent("node:C · no pipes");
    // A known support family shows its product label; an unknown one is shown as stored.
    expect(row("support", "support:H")).toHaveTextContent("support:H · at B · Variable spring hanger");
    expect(row("support", "support:U")).toHaveTextContent("support:U · at A · invented family");
    expect(row("component", "component:E")).toHaveTextContent("component:E · at B · bend");
    // The context also reaches the row's title, so it is not only visible on screen.
    expect(row("pipe", "pipe:AB")).toHaveAttribute("title", "Pipe AB (pipe:AB · A → B)");
  });

  it("marks the keyboard's current row, group or entity, apart from the selection", () => {
    render(<ModelTree model={inventedModel()} onSelect={vi.fn()} selection={{ type: "project", id: "p" }} />);
    const tree = screen.getByRole("tree", { name: "Model" });
    const current = () => document.getElementById(tree.getAttribute("aria-activedescendant") ?? "")!;
    fireEvent.keyDown(tree, { key: "Home" });
    fireEvent.keyDown(tree, { key: "ArrowDown" });
    expect(current()).toHaveClass("tree-group-row", "focused");
    fireEvent.keyDown(tree, { key: "ArrowDown" });
    expect(current()).not.toHaveClass("tree-group-row");
    expect(current()).toHaveClass("focused");
    expect(tree.querySelectorAll(".tree-row.focused")).toHaveLength(1);
  });

  it("counts each group's entities, and while filtering its matches of that total", () => {
    render(<ModelTree model={inventedModel()} onSelect={vi.fn()} selection={{ type: "project", id: "p" }} />);
    expect(screen.getByTestId("tree-group-count-Nodes")).toHaveTextContent(/^3$/u);
    expect(screen.getByTestId("tree-group-count-Supports")).toHaveTextContent(/^2$/u);
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "node:B" } });
    expect(screen.getByTestId("tree-group-count-Nodes")).toHaveTextContent("1 of 3");
    // A group whose every entity matches shows its plain count.
    expect(screen.getByTestId("tree-group-count-Pipes")).toHaveTextContent(/^1$/u);
    expect(screen.getByTestId("tree-group-count-Supports")).toHaveTextContent("1 of 2");
    expect(screen.getByTestId("model-tree-filter-summary")).toHaveTextContent(/^\d+ of 8 model entities match the filter$/u);
  });
});

/** The invented model with enough nodes that the tree's list holds only the rows near the view. */
function longInventedModel(): PreviewModel {
  const model = inventedModel();
  const extra = Array.from({ length: 120 }, (_, index) => ({
    id: `node:X${index}`, label: `Node X${index}`, position: { x: index, y: 1, z: 0 }, provenance: "test"
  }));
  return { ...model, nodes: [...model.nodes, ...extra] } as PreviewModel;
}

describe("ModelTree count summary and long-list note", () => {
  it("counts the model with no filter, and the entities that match a filter", () => {
    render(<ModelTree model={inventedModel()} onSelect={vi.fn()} selection={{ type: "project", id: "p" }} />);
    const summary = screen.getByTestId("model-tree-filter-summary");
    expect(summary).toHaveTextContent(/^8 model entities$/u);
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "Unknown family" } });
    expect(summary).toHaveTextContent(/^1 of 8 model entities match the filter$/u);
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "no-match-token" } });
    expect(summary).toHaveTextContent(/^0 of 8 model entities match the filter$/u);
    // Only spaces filter nothing, so the plain count returns.
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "   " } });
    expect(summary).toHaveTextContent(/^8 model entities$/u);
  });

  it("says nothing about the view when the list holds every row", () => {
    render(<ModelTree model={inventedModel()} onSelect={vi.fn()} selection={{ type: "project", id: "p" }} />);
    expect(screen.queryByTestId("model-tree-window-note")).toBeNull();
    expect(screen.getByTestId("model-tree-filter-input")).toHaveAccessibleDescription("8 model entities");
  });

  it("says a long list holds only the rows near the view and where the whole model is saved", () => {
    render(<ModelTree model={longInventedModel()} onSelect={vi.fn()} selection={{ type: "project", id: "p" }} />);
    const tree = screen.getByRole("tree", { name: "Model" });
    // The list does hold fewer rows than the tree has.
    expect(tree.querySelectorAll('[role="treeitem"]').length).toBeLessThan(128);
    const note = screen.getByTestId("model-tree-window-note");
    expect(note).toHaveTextContent(
      "The list holds only the rows near the view, so text selected and copied from it covers those rows only; the whole model is saved with File > Save Model Document…"
    );
    // Read with the count, from the filter, without taking focus.
    expect(screen.getByTestId("model-tree-filter-input")).toHaveAccessibleDescription(
      `128 model entities ${note.textContent}`
    );
    expect(note).not.toHaveAttribute("tabindex");
    expect(note.contains(document.activeElement)).toBe(false);
    // Scrolling does not take it away.
    tree.scrollTop = 2000;
    fireEvent.scroll(tree);
    expect(screen.getByTestId("model-tree-window-note")).toBe(note);
    // A filter short enough for the list to hold every row removes it.
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "node:X1" } });
    expect(screen.getByTestId("model-tree-filter-summary")).toHaveTextContent(/^\d+ of 128 model entities match the filter$/u);
    expect(screen.queryByTestId("model-tree-window-note")).toBeNull();
  });
});

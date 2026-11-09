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

  it("counts each group's entities, and while filtering its matches of that total", () => {
    render(<ModelTree model={inventedModel()} onSelect={vi.fn()} selection={{ type: "project", id: "p" }} />);
    expect(screen.getByTestId("tree-group-count-Nodes")).toHaveTextContent(/^3$/u);
    expect(screen.getByTestId("tree-group-count-Supports")).toHaveTextContent(/^2$/u);
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "node:B" } });
    expect(screen.getByTestId("tree-group-count-Nodes")).toHaveTextContent("1 of 3");
    // A group whose every entity matches shows its plain count.
    expect(screen.getByTestId("tree-group-count-Pipes")).toHaveTextContent(/^1$/u);
    expect(screen.getByTestId("tree-group-count-Supports")).toHaveTextContent("1 of 2");
    expect(screen.getByTestId("model-tree-filter-summary")).toHaveTextContent("of 8 model entities visible");
  });
});

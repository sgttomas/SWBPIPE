import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import { singletonSelection } from "../workspace/selectionState";
import { ModelTree } from "./ModelTree";

function largeModel(): PreviewModel {
  const nodes = Array.from({ length: 160 }, (_, index) => ({
    id: `n/${index}`,
    label: `Node ${index}`,
    position: { x: index, y: 0, z: 0 },
    provenance: "test"
  }));
  return {
    project: { id: "p", name: "Project", description: "", units: { length: "m" } },
    schema_version: "0.2.0",
    analysis_status: { mechanics: "NOT_RUN", rule_check: "NOT_RUN", professional_acceptance: "NOT_REVIEWED" },
    materials: [], sections: [], nodes,
    pipe_segments: [{ id: "n/0", label: "Pipe collision", from: "n/0", to: "n/1", section: {}, material: "", provenance: "test" }],
    supports: [], components: [], load_cases: [], combinations: [], diagnostics: []
  } as unknown as PreviewModel;
}

describe("ModelTree virtual typed selection", () => {
  it("uses collision-safe IDs, publishes range metadata, and scrolls off-window keyboard focus", () => {
    const model = largeModel();
    const onSelect = vi.fn();
    const onFocusChange = vi.fn();
    render(<ModelTree
      model={model}
      selection={{ type: "node", id: "n/0" }}
      selectionState={singletonSelection({ type: "node", id: "n/0" })}
      onFocusChange={onFocusChange}
      onSelect={onSelect}
    />);

    expect(screen.getByRole("tree", { name: "Model" })).toHaveAttribute("aria-multiselectable", "true");
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "n/0" } });
    expect(screen.getByTestId("tree-row-node-n%2F0")).toBeInTheDocument();
    expect(screen.getByTestId("tree-row-pipe-n%2F0")).toBeInTheDocument();
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "" } });
    fireEvent.click(screen.getByTestId("tree-row-node-n%2F1"), { shiftKey: true });
    expect(onSelect).toHaveBeenLastCalledWith(
      expect.objectContaining({ type: "node", id: "n/1" }),
      expect.objectContaining({ additive: true, range: true }),
      expect.any(Array)
    );

    const tree = screen.getByRole("tree", { name: "Model" });
    for (let index = 0; index < 145; index += 1) fireEvent.keyDown(tree, { key: "ArrowDown" });
    expect(tree.scrollTop).toBeGreaterThan(0);
    expect(onFocusChange).toHaveBeenCalled();
    expect(tree.querySelectorAll("[data-virtual-index]").length).toBeLessThan(60);
  });
});

import { act, createEvent, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import * as hashing from "../../services/hashService";
import { DisplayUnitsProvider } from "../display-units";
import { entityKey, setSelectionFocus, singletonSelection } from "../workspace/selectionState";
import { ModelTree } from "./ModelTree";
import { PropertyInspector } from "./PropertyInspector";

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
  it("publishes the exact current filter input timing pair", async () => {
    const onFilterPublication = vi.fn();
    render(<ModelTree
      model={largeModel()}
      onFilterPublication={onFilterPublication}
      onSelect={vi.fn()}
      selection={{ type: "node", id: "n/0" }}
    />);
    const input = screen.getByTestId("model-tree-filter-input");
    const event = createEvent.change(input, { target: { value: "Node 42" } });
    Object.defineProperty(event, "timeStamp", { value: 987.5 });
    fireEvent(input, event);
    await waitFor(() => expect(onFilterPublication).toHaveBeenLastCalledWith(expect.objectContaining({
      actionSequence: 1,
      inputAt: expect.any(Number),
      inputEventTimeStamp: 987.5,
      query: "Node 42"
    })));
  });

  it("retains collision-safe grid drafts across type/filter changes and resets them on a same-ID new session", () => {
    const model = largeModel();
    const onQueueIntent = vi.fn();
    const view = render(<ModelTree
      model={model}
      onQueueIntent={onQueueIntent}
      onSelect={vi.fn()}
      projectSessionGeneration={0}
      selection={{ type: "node", id: "n/0" }}
    />);
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    openNodeGridReview();
    fireEvent.change(reviewEditor("n/0-x"), { target: { value: "10" } });
    fireEvent.change(reviewEditor("n/1-x"), { target: { value: "11" } });
    fireEvent.click(screen.getByTestId("entity-grid-type-pipes"));
    fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
    expect(reviewEditor("n/0-x")).toHaveValue("10");
    expect(reviewEditor("n/1-x")).toHaveValue("11");
    fireEvent.click(screen.getByTestId("layout-mode-tree"));
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    openNodeGridReview();
    expect(reviewEditor("n/0-x")).toHaveValue("10");
    expect(reviewEditor("n/1-x")).toHaveValue("11");

    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "n/0" } });
    fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
    expect(onQueueIntent).toHaveBeenCalledTimes(1);
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "" } });
    expect(reviewEditor("n/0-x")).toHaveValue("0");
    expect(reviewEditor("n/1-x")).toHaveValue("11");

    view.rerender(<ModelTree
      model={structuredClone(model)}
      onQueueIntent={onQueueIntent}
      onSelect={vi.fn()}
      projectSessionGeneration={1}
      selection={{ type: "node", id: "n/0" }}
    />);
    expect(reviewEditor("n/1-x")).toHaveValue("1");
  });

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

  it("restores stored typed focus and publishes a visible fallback when filtering removes it", () => {
    const model = largeModel();
    const onFocusChange = vi.fn();
    const selected = { type: "node", id: "n/0" } as const;
    const selectionState = setSelectionFocus(
      singletonSelection(selected),
      entityKey({ type: "node", id: "n/150" })
    );
    render(<ModelTree
      model={model}
      selection={selected}
      selectionState={selectionState}
      onFocusChange={onFocusChange}
      onSelect={vi.fn()}
    />);

    const tree = screen.getByRole("tree", { name: "Model" });
    expect(tree).toHaveAttribute("aria-activedescendant", "tree-row-node-n%2F150");
    expect(tree.scrollTop).toBeGreaterThan(0);

    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "Node 2" } });
    expect(tree).toHaveAttribute("aria-activedescendant", "tree-row-node-n%2F2");
    expect(onFocusChange).toHaveBeenLastCalledWith(entityKey({ type: "node", id: "n/2" }));
  });

  it("implements Right expand/first-child and Left parent/collapse keyboard behavior", () => {
    const model = largeModel();
    const onSelect = vi.fn();
    render(<ModelTree
      model={model}
      selection={{ type: "project", id: model.project.id }}
      onFocusChange={vi.fn()}
      onSelect={onSelect}
    />);
    const tree = screen.getByRole("tree", { name: "Model" });
    const nodesGroup = screen.getByTestId("tree-group-Nodes");
    fireEvent.click(nodesGroup);
    expect(nodesGroup).toHaveAttribute("aria-expanded", "false");

    // Home focuses Project; Down focuses the first group, then continue until
    // the collapsed Nodes group becomes the active descendant.
    fireEvent.keyDown(tree, { key: "Home" });
    while (tree.getAttribute("aria-activedescendant") !== "tree-group-Nodes") {
      fireEvent.keyDown(tree, { key: "ArrowDown" });
    }
    fireEvent.keyDown(tree, { key: "ArrowRight" });
    expect(screen.getByTestId("tree-group-Nodes")).toHaveAttribute("aria-expanded", "true");
    expect(tree).toHaveAttribute("aria-activedescendant", "tree-group-Nodes");

    fireEvent.keyDown(tree, { key: "ArrowRight" });
    expect(tree).toHaveAttribute("aria-activedescendant", "tree-row-node-n%2F0");
    fireEvent.keyDown(tree, { key: "ArrowLeft" });
    expect(tree).toHaveAttribute("aria-activedescendant", "tree-group-Nodes");
    expect(screen.getByTestId("tree-group-Nodes")).toHaveAttribute("aria-expanded", "true");
    expect(onSelect).not.toHaveBeenCalled();
    fireEvent.keyDown(tree, { key: "ArrowLeft" });
    expect(screen.getByTestId("tree-group-Nodes")).toHaveAttribute("aria-expanded", "false");
    expect(onSelect).not.toHaveBeenCalled();
  });

  it("lets filtered groups collapse independently and restores pre-search expansion without selecting", () => {
    const model = largeModel();
    const onSelect = vi.fn();
    const selected = { type: "node", id: "n/0" } as const;
    render(<ModelTree model={model} selection={selected} onSelect={onSelect} />);
    const tree = screen.getByRole("tree", { name: "Model" });
    const input = screen.getByTestId("model-tree-filter-input");
    const group = () => screen.getByTestId("tree-group-Nodes");
    const child = () => screen.queryByTestId("tree-row-node-n%2F0");
    fireEvent.click(group());
    expect(group()).toHaveAttribute("aria-expanded", "false");
    fireEvent.change(input, { target: { value: "Node 0" } });
    expect(group()).toHaveAttribute("aria-expanded", "true");
    expect(child()).toHaveAttribute("aria-selected", "true");
    fireEvent.keyDown(tree, { key: "ArrowRight" });
    expect(tree).toHaveAttribute("aria-activedescendant", "tree-row-node-n%2F0");
    fireEvent.keyDown(tree, { key: "ArrowLeft" });
    expect(tree).toHaveAttribute("aria-activedescendant", "tree-group-Nodes");
    expect(group()).toHaveAttribute("aria-expanded", "true");
    fireEvent.keyDown(tree, { key: "ArrowLeft" });
    expect(group()).toHaveAttribute("aria-expanded", "false");
    expect(child()).not.toBeInTheDocument();
    fireEvent.change(input, { target: { value: "  NODE 0  " } });
    expect(group()).toHaveAttribute("aria-expanded", "false");
    for (const key of ["ArrowRight", "Enter", " ", "Enter", " "]) {
      const expanded = group().getAttribute("aria-expanded") === "true";
      fireEvent.keyDown(tree, { key });
      expect(group()).toHaveAttribute("aria-expanded", String(!expanded));
      expect(Boolean(child())).toBe(!expanded);
    }
    fireEvent.click(group());
    expect(group()).toHaveAttribute("aria-expanded", "false");
    fireEvent.change(input, { target: { value: "Node 1" } });
    expect(group()).toHaveAttribute("aria-expanded", "true");
    fireEvent.change(input, { target: { value: "" } });
    expect(group()).toHaveAttribute("aria-expanded", "false");
    fireEvent.change(input, { target: { value: "Node 0" } });
    expect(child()).toHaveAttribute("aria-selected", "true");
    expect(onSelect).not.toHaveBeenCalled();
  });

  it.each([
    ["node", "Node collision", "Position"],
    ["pipe", "Pipe collision", "From"],
    ["support", "Support collision", "Restraints"],
    ["component", "Component collision", "Kind"]
  ] as const)("renders typed %s inspector identity for a four-way raw-ID collision", (type, heading, property) => {
    const id = "same/id";
    const model = largeModel();
    model.nodes = [
      { id, label: "Node collision", position: { x: 1, y: 2, z: 3 }, provenance: "test" },
      { id: "end", label: "End", position: { x: 2, y: 2, z: 3 }, provenance: "test" }
    ];
    model.pipe_segments = [{ id, label: "Pipe collision", from: id, to: "end", section: {}, material: "m", provenance: "test" }];
    model.supports = [{ id, label: "Support collision", node: id, restraints: ["UX"], provenance: "test" }];
    model.components = [{ id, label: "Component collision", node: id, kind: "valve", provenance: "test" }];
    const view = render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onQueueIntent={vi.fn()}
      selection={{ type, id }}
    /></DisplayUnitsProvider>);

    expect(screen.getByRole("heading", { name: `${heading} — ${type}: ${id}` })).toBeVisible();
    expect(view.getByTestId("property-inspector").textContent).toContain(property);
    view.unmount();
  });

  it("retains a frozen property task across a later multi-selection and disables its mutation routes", () => {
    const model = largeModel();
    const first = { type: "node", id: "n/0" } as const;
    const view = render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onApplyIntent={vi.fn()}
      onQueueIntent={vi.fn()}
      onValidateIntent={vi.fn()}
      selection={first}
      selectionState={singletonSelection(first)}
    /></DisplayUnitsProvider>);
    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    expect(screen.getByTestId("inspector-task-empty")).toHaveTextContent("Current selection: node: n/0");
    expect(screen.queryByTestId("task-action-footer")).not.toBeInTheDocument();
    fireEvent.click(screen.getByTestId("inspector-start-task"));
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value: "42" } });

    const second = { type: "node", id: "n/1" } as const;
    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onApplyIntent={vi.fn()}
      onQueueIntent={vi.fn()}
      onValidateIntent={vi.fn()}
      selection={second}
      selectionState={{
        ...singletonSelection(second),
        orderedKeys: [entityKey(first), entityKey(second)]
      }}
    /></DisplayUnitsProvider>);

    expect(screen.getByTestId("inspector-frozen-task-target")).toHaveTextContent("node: n/0");
    expect(screen.getByTestId("editor-intent-value")).toHaveValue("42");
    expect(screen.getByTestId("queue-editor-intent")).toBeDisabled();
    expect(screen.getByTestId("validate-editor-intent-inline")).toBeDisabled();
    expect(screen.getByTestId("apply-editor-intent-inline")).toBeDisabled();
    expect(screen.getByText(/existing frozen draft is retained/)).toBeInTheDocument();
  });

  it("keeps a rich Properties draft mounted and target-frozen during aggregate inspection", () => {
    const model = largeModel();
    model.supports = [{ id: "support/A", label: "Support A", node: "n/0", restraints: ["UX"], provenance: "source" }];
    const support = { type: "support", id: "support/A" } as const;
    const node = { type: "node", id: "n/1" } as const;
    const view = render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onQueueIntent={vi.fn()}
      selection={support}
      selectionState={singletonSelection(support)}
    /></DisplayUnitsProvider>);
    fireEvent.change(screen.getByLabelText("Support provenance"), { target: { value: "entered rich draft" } });

    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onQueueIntent={vi.fn()}
      selection={node}
      selectionState={{ ...singletonSelection(node), orderedKeys: [entityKey(support), entityKey(node)] }}
    /></DisplayUnitsProvider>);
    expect(screen.getByTestId("aggregate-property-inspector")).toHaveTextContent("Existing Properties drafts remain attached");
    expect(screen.getByLabelText("Support provenance")).toHaveValue("entered rich draft");
    expect(screen.getByLabelText("Support provenance")).toBeDisabled();

    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onQueueIntent={vi.fn()}
      selection={support}
      selectionState={singletonSelection(support)}
    /></DisplayUnitsProvider>);
    expect(screen.getByLabelText("Support provenance")).toHaveValue("entered rich draft");

    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onQueueIntent={vi.fn()}
      projectSessionGeneration={1}
      selection={support}
      selectionState={singletonSelection(support)}
    /></DisplayUnitsProvider>);
    expect(screen.getByLabelText("Support provenance")).toHaveValue("source");
  });

  it("rejects a pending rich support queue across same-primary membership ABA and permits a fresh retry", async () => {
    const model = largeModel();
    model.supports = [{ id: "support/A", label: "Support A", node: "n/0", restraints: ["UX"], provenance: "source" }];
    const support = { type: "support", id: "support/A" } as const;
    const node = { type: "node", id: "n/1" } as const;
    const singleton = singletonSelection(support);
    const aggregate = {
      ...singleton,
      orderedKeys: [entityKey(support), entityKey(node)],
      preparationEpoch: singleton.preparationEpoch + 1
    };
    const restored = singletonSelection(support, aggregate);
    const queued = vi.fn();
    let epoch = 20;
    const getPreparationEpoch = () => epoch;
    let finishCanonicalization!: (value: string) => void;
    const canonical = vi.spyOn(hashing, "canonicalJsonString")
      .mockImplementationOnce(() => new Promise<string>((resolve) => { finishCanonicalization = resolve; }))
      .mockImplementation(async (value) => JSON.stringify(value));
    const view = render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      getPreparationEpoch={getPreparationEpoch}
      model={model}
      onQueueIntent={queued}
      selection={support}
      selectionState={singleton}
    /></DisplayUnitsProvider>);
    fireEvent.change(screen.getByLabelText("Support provenance"), { target: { value: "entered rich draft" } });
    fireEvent.click(screen.getByRole("button", { name: "Queue support configuration" }));
    await waitFor(() => expect(canonical).toHaveBeenCalledTimes(1));

    epoch += 1;
    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      getPreparationEpoch={getPreparationEpoch}
      model={model}
      onQueueIntent={queued}
      selection={support}
      selectionState={aggregate}
    /></DisplayUnitsProvider>);
    epoch += 1;
    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      getPreparationEpoch={getPreparationEpoch}
      model={model}
      onQueueIntent={queued}
      selection={support}
      selectionState={restored}
    /></DisplayUnitsProvider>);
    await act(async () => { finishCanonicalization(JSON.stringify({ provenance: "source", restraints: ["UX"] })); });

    expect(queued).not.toHaveBeenCalled();
    expect(screen.getByRole("alert")).toHaveTextContent("selection, model or pending changes changed");
    expect(screen.getByLabelText("Support provenance")).toHaveValue("entered rich draft");

    fireEvent.click(screen.getByRole("button", { name: "Queue support configuration" }));
    await waitFor(() => expect(queued).toHaveBeenCalledTimes(1));
    expect(queued).toHaveBeenLastCalledWith(expect.objectContaining({
      target: { object_type: "Support", ref: "support/A" },
      change: expect.objectContaining({
        after: expect.stringContaining("entered rich draft"),
        change_kind: "update_support",
        field_path: "configuration"
      })
    }));
    expect(screen.getByRole("status")).toHaveTextContent("queued for validation and review");
    canonical.mockRestore();
  });

  it("closes a frozen Task only after its apply route confirms acceptance", async () => {
    const target = { type: "node", id: "n/0" } as const;
    const apply = vi.fn()
      .mockResolvedValueOnce(false)
      .mockRejectedValueOnce(new Error("invented rejection"))
      .mockResolvedValueOnce(true);
    render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={largeModel()}
      onApplyIntent={apply}
      onQueueIntent={vi.fn()}
      selection={target}
      selectionState={singletonSelection(target)}
    /></DisplayUnitsProvider>);
    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    fireEvent.click(screen.getByTestId("inspector-start-task"));
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value: "42" } });

    fireEvent.click(screen.getByTestId("apply-editor-intent-inline"));
    await waitFor(() => expect(apply).toHaveBeenCalledTimes(1));
    expect(screen.getByTestId("inspector-frozen-task-target")).toHaveTextContent("node: n/0");
    expect(screen.getByTestId("editor-intent-value")).toHaveValue("42");

    fireEvent.click(screen.getByTestId("apply-editor-intent-inline"));
    await waitFor(() => expect(apply).toHaveBeenCalledTimes(2));
    expect(screen.getByTestId("inspector-frozen-task-target")).toBeInTheDocument();

    fireEvent.click(screen.getByTestId("apply-editor-intent-inline"));
    await waitFor(() => expect(screen.getByTestId("inspector-task-empty")).toBeInTheDocument());
    expect(apply).toHaveBeenCalledTimes(3);
  });

  it("focuses the requested inspector field after applying the frozen target and view, including repeat routes", () => {
    const model = largeModel();
    const target = { type: "node", id: "n/1" } as const;
    const request = { sequence: 1, target, view: "task" as const, focusTestId: "editor-intent-field" };
    const view = render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onQueueIntent={vi.fn()}
      selection={{ type: "node", id: "n/0" }}
      taskRequest={request}
    /></DisplayUnitsProvider>);
    expect(screen.getByTestId("inspector-frozen-task-target")).toHaveTextContent("node: n/1");
    expect(screen.getByTestId("editor-intent-field")).toHaveFocus();

    fireEvent.click(screen.getByRole("tab", { name: "Properties" }));
    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onQueueIntent={vi.fn()}
      selection={{ type: "node", id: "n/0" }}
      taskRequest={{ ...request, sequence: 2 }}
    /></DisplayUnitsProvider>);
    expect(screen.getByTestId("editor-intent-field")).toHaveFocus();
  });

  it("targets rich Properties commands independently while retaining the frozen Task", async () => {
    const model = largeModel();
    const queue = vi.fn();
    const first = { type: "node", id: "n/0" } as const;
    const second = { type: "node", id: "n/1" } as const;
    const view = render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onQueueIntent={queue}
      selection={first}
      selectionState={singletonSelection(first)}
    /></DisplayUnitsProvider>);

    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    fireEvent.click(screen.getByTestId("inspector-start-task"));
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value: "42" } });

    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onQueueIntent={queue}
      selection={first}
      selectionState={singletonSelection(first)}
      taskRequest={{
        sequence: 1,
        target: second,
        view: "properties",
        focusTestId: "create-section-id"
      }}
    /></DisplayUnitsProvider>);

    await waitFor(() => expect(screen.getByTestId("create-section-id")).toHaveFocus());
    expect(screen.getByRole("heading", { name: "Node 1 — node: n/1" })).toBeVisible();

    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    expect(screen.getByTestId("inspector-frozen-task-target")).toHaveTextContent("node: n/0");
    expect(screen.getByTestId("editor-intent-value")).toHaveValue("42");
    expect(within(screen.getByTestId("delete-node-intent-panel")).getByTestId("editor-operation-preview"))
      .toHaveTextContent("Node; n/0");

    fireEvent.click(screen.getByRole("tab", { name: "Properties" }));
    fireEvent.click(screen.getByTestId("queue-delete-node-intent"));
    expect(queue).toHaveBeenLastCalledWith(expect.objectContaining({
      target: expect.objectContaining({ object_type: "Node", ref: "n/1" }),
      change: expect.objectContaining({ change_kind: "delete_node" })
    }));

    fireEvent.click(screen.getByTestId("queue-create-support-intent"));
    expect(screen.getByLabelText("New support node current value")).toHaveTextContent("Node 1 (n/1)");

    fireEvent.change(screen.getByTestId("create-component-radius"), { target: { value: "1" } });
    fireEvent.change(screen.getByTestId("create-component-angle"), { target: { value: "90" } });
    fireEvent.change(screen.getByTestId("create-component-plane"), { target: { value: "XY" } });
    fireEvent.click(screen.getByTestId("queue-create-component-intent"));
    expect(screen.getByLabelText("New component node current value")).toHaveTextContent("Node 1 (n/1)");
    expect(queue.mock.calls.map(([intent]) => intent.change.change_kind)).toEqual(expect.arrayContaining([
      "delete_node",
      "create_support",
      "insert_component_symbol"
    ]));

    const third = { type: "node", id: "n/2" } as const;
    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector
      model={model}
      onQueueIntent={queue}
      selection={third}
      selectionState={singletonSelection(third)}
    /></DisplayUnitsProvider>);
    expect(screen.getByRole("heading", { name: "Node 2 — node: n/2" })).toBeVisible();
    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    expect(screen.getByTestId("inspector-frozen-task-target")).toHaveTextContent("node: n/0");
    expect(screen.getByTestId("editor-intent-value")).toHaveValue("42");
  });
});


describe("legacy indexed tree search vocabulary", () => {
  it("retains provenance, references and restraints without changing selection", () => {
    const model = largeModel();
    Object.assign(model.project, { description: "plant-description-token" });
    model.nodes = model.nodes.slice(0, 2);
    model.nodes[0].provenance = "node-source-token";
    model.pipe_segments[0].material = "alloy-reference-token";
    model.pipe_segments[0].provenance = "pipe-source-token";
    Object.assign(model, {
      materials: [{ id: "m", label: "Alloy", provenance: "material-source-token" }],
      sections: [{ id: "s", label: "Section", section_type: "section-type-token", provenance: { source_reference: "catalog-source-token" } }],
      supports: [{ id: "support-a", label: "Anchor", node: "n/0", family: "anchor", restraints: ["UX"], provenance: "support-source-token", hanger: { hanger_type: "hanger-type-token", source_reference: "hanger-source-token", mechanics_consumption: "hanger-consumption-token" } }],
      components: [{ id: "c", label: "Fitting", node: "n/0", kind: "bend", provenance: "component-source-token", geometry: { bend_radius: { value: 123.456, unit: "mm" }, bend_pipe_ref: "bend-reference-token", branch_geometry_source_reference: "branch-reference-token", rigid_component_source_reference: "rigid-reference-token", manufacturer_reference: "manufacturer-token", center_of_gravity: { x: 12, y: 34, z: 56, unit: "mm" } }, modifiers: { torsional_stiffness_user_value: { value: 789.123, unit: "N.m/rad" }, source_reference: "modifier-source-token" }, mechanics_interface: { solver_consumption: "solver-consumption-token" }, completeness: [{ diagnostic_code: "completeness-token", status: "pending-token" }] }],
      load_cases: [{ id: "l", name: "Operating", provenance: "load-source-token" }],
      combinations: [{ id: "combo", name: "Envelope", provenance: "combination-source-token" }]
    });
    const onSelect = vi.fn();
    const before = JSON.stringify(model);
    render(<ModelTree model={model} selection={{ type: "project", id: "p" }} onSelect={onSelect} />);
    const witnesses: [string, string, string[]][] = [
      ["project", "p", ["plant-description-token", "model"]],
      ["material", "m", ["material-source-token"]],
      ["section", "s", ["section-type-token", "catalog-source-token", "pipe section"]],
      ["node", "n/0", ["node-source-token"]],
      ["pipe", "n/0", ["alloy-reference-token", "pipe-source-token", "pipe segment"]],
      ["support", "support-a", ["UX", "support-source-token", "hanger-type-token", "hanger-source-token", "hanger-consumption-token"]],
      ["component", "c", ["component-source-token", "123.456 mm", "bend-reference-token", "branch-reference-token", "rigid-reference-token", "manufacturer-token", "12 34 56 mm", "789.123 N.m/rad", "modifier-source-token", "solver-consumption-token", "completeness-token", "pending-token"]],
      ["load", "l", ["load-source-token", "load case"]],
      ["combination", "combo", ["combination-source-token"]]
    ];
    for (const [type, id, terms] of witnesses) for (const term of terms) {
      fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: term } });
      expect(screen.getByTestId(`tree-row-${type}-${encodeURIComponent(id)}`), term).toBeInTheDocument();
    }
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "UX" } });
    expect(screen.queryByTestId("tree-row-node-n%2F0")).not.toBeInTheDocument();
    expect(screen.queryByTestId("tree-row-pipe-n%2F0")).not.toBeInTheDocument();
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "unrelated-missing-token" } });
    expect(screen.queryByTestId("tree-row-support-support-a")).not.toBeInTheDocument();
    expect(onSelect).not.toHaveBeenCalled();
    expect(JSON.stringify(model)).toBe(before);
  });
});

// Existing review journeys explicitly enter the retained multi-change workflow.
function openNodeGridReview() {
  const summary = screen.getByTestId("node-grid-review-disclosure");
  if (summary.getAttribute("aria-expanded") !== "true") fireEvent.click(summary);
}

// The shared review table opens one cell editor at a time; retain any prior raw draft first.
function reviewEditor(cell: string): HTMLInputElement {
  const table = screen.getByTestId("engineering-table-review");
  const existing = table.querySelector<HTMLInputElement>("input");
  const split = cell.lastIndexOf("-"); const row = cell.slice(0, split); const column = cell.slice(split + 1);
  if (existing?.getAttribute("aria-label")?.startsWith(`${row} ${column.length === 1 ? column.toUpperCase() : column === "label" ? "Label" : "Provenance"}`)) return existing;
  const keep = table.querySelector<HTMLButtonElement>('[data-table-action="apply"]');
  if (keep) fireEvent.click(keep);
  fireEvent.doubleClick(screen.getByTestId(`review-cell-${cell}`));
  return table.querySelector<HTMLInputElement>("input")!;
}

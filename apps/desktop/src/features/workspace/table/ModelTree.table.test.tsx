import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ModelTree } from "../../model-tree/ModelTree";
import { loadPreviewModel } from "../../../services/previewService";

describe("node table and retained review transitions", () => {
  it("preserves an invalid editor through actual family and Tree transitions with inactive surfaces inert", async () => {
    const model = await loadPreviewModel(); const apply = vi.fn(async (_intent: import("../../../types").EditorOperationIntent) => ({ applied: true, messages: [] }));
    render(<ModelTree model={model} selection={{ type: "node", id: model.nodes[0].id }} onSelect={vi.fn()} onApplyCellIntent={apply} />);
    fireEvent.click(screen.getByTestId("layout-mode-grid")); fireEvent.doubleClick(screen.getByTestId(`table-cell-${model.nodes[0].id}-x`));
    const editor = screen.getByRole("textbox", { name: `${model.nodes[0].id} X [m]` }); fireEvent.change(editor, { target: { value: "invalid" } });
    fireEvent.blur(editor); fireEvent.click(screen.getByTestId("entity-grid-type-pipes"));
    expect(screen.getByTestId("engineering-table").parentElement).toHaveAttribute("inert"); expect(screen.getByTestId("engineering-table").parentElement).toHaveAttribute("hidden");
    fireEvent.click(screen.getByTestId("entity-grid-type-nodes")); expect(screen.getByRole("textbox", { name: `${model.nodes[0].id} X [m]` })).toHaveValue("invalid");
    fireEvent.click(screen.getByTestId("layout-mode-tree")); expect(screen.getByTestId("entity-grid").parentElement).toHaveAttribute("inert");
    fireEvent.click(screen.getByTestId("layout-mode-grid")); expect(screen.getByRole("textbox", { name: `${model.nodes[0].id} X [m]` })).toHaveValue("invalid");
    fireEvent.click(screen.getByRole("button", { name: "Cancel" })); expect(screen.getByTestId(`table-cell-${model.nodes[0].id}-x`)).toHaveFocus(); expect(apply).not.toHaveBeenCalled();
  });
  it("reports retained bulk drafts while collapsed without queueing them, across family and Tree round trips", async () => {
    const model = await loadPreviewModel(); const queue = vi.fn();
    render(<ModelTree model={model} selection={{ type: "node", id: model.nodes[0].id }} onSelect={vi.fn()} onQueueIntent={queue} />);
    fireEvent.click(screen.getByTestId("layout-mode-grid")); const summary = screen.getByTestId("node-grid-review-disclosure");
    fireEvent.click(summary); await waitFor(() => expect(summary).toHaveAttribute("aria-expanded", "true"));
    fireEvent.change(reviewEditor(`${model.nodes[0].id}-x`), { target: { value: "2.500" } });
    fireEvent.click(summary); await waitFor(() => expect(summary).toHaveTextContent("1 retained draft"));
    fireEvent.click(screen.getByTestId("entity-grid-type-pipes")); fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
    fireEvent.click(screen.getByTestId("layout-mode-tree")); fireEvent.click(screen.getByTestId("layout-mode-grid"));
    expect(summary).toHaveTextContent("1 retained draft"); expect(queue).not.toHaveBeenCalled();
    fireEvent.click(summary); expect(reviewEditor(`${model.nodes[0].id}-x`)).toHaveValue("2.500");
    fireEvent.click(screen.getByTestId("queue-entity-grid-intents")); expect(queue).toHaveBeenCalledTimes(1);
    expect(queue.mock.calls[0][0].change.after).toBe('{"value":2.5,"unit":"m"}');
  });
  it("does not fabricate a coordinate unit when the source model omits it", async () => {
    const model = structuredClone(await loadPreviewModel()); delete (model.project.units as Partial<typeof model.project.units>).length;
    const apply = vi.fn(async (_intent: import("../../../types").EditorOperationIntent) => ({ applied: true, messages: [] }));
    render(<ModelTree model={model} selection={{ type: "node", id: model.nodes[0].id }} onSelect={vi.fn()} onApplyCellIntent={apply} />);
    fireEvent.click(screen.getByTestId("layout-mode-grid")); expect(screen.getByRole("button", { name: "Sort X" })).toHaveTextContent("unit missing");
    fireEvent.doubleClick(screen.getByTestId(`table-cell-${model.nodes[0].id}-x`));
    fireEvent.change(screen.getByRole("textbox", { name: `${model.nodes[0].id} X []` }), { target: { value: "2" } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    expect(screen.getByRole("alert")).toHaveTextContent("no declared length unit"); expect(apply).not.toHaveBeenCalled();
  });

  it("uses one visible review work area while retaining direct invalid text and bulk drafts", async () => {
    const model = await loadPreviewModel(); const apply = vi.fn(async (_intent: import("../../../types").EditorOperationIntent) => ({ applied: true, messages: [] })); const queue = vi.fn();
    render(<ModelTree boundedGrid model={model} selection={{ type: "node", id: model.nodes[0].id }} onSelect={vi.fn()} onApplyCellIntent={apply} onQueueIntent={queue} />);
    fireEvent.click(screen.getByTestId("layout-mode-grid")); fireEvent.doubleClick(screen.getByTestId(`table-cell-${model.nodes[0].id}-x`));
    const editor = screen.getByRole("textbox", { name: `${model.nodes[0].id} X [m]` }); fireEvent.change(editor, { target: { value: "bad" } }); fireEvent.blur(editor);
    const summary = screen.getByTestId("node-grid-review-disclosure"); fireEvent.click(summary);
    await waitFor(() => expect(screen.getByTestId("engineering-table").parentElement).toHaveAttribute("hidden"));
    expect(screen.getByTestId("engineering-table").parentElement).toHaveAttribute("inert"); expect(screen.getByTestId("retained-direct-draft")).toHaveTextContent("Direct node edit retained");
    fireEvent.change(reviewEditor(`${model.nodes[0].id}-y`), { target: { value: "0.5" } });
    expect(summary).toHaveTextContent("Return to node fields"); fireEvent.click(summary);
    await waitFor(() => expect(editor).toBeVisible()); expect(editor).toHaveValue("bad"); expect(summary).toHaveTextContent("1 retained draft");
    expect(apply).not.toHaveBeenCalled(); expect(queue).not.toHaveBeenCalled(); fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
  });

});

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

describe("Node scalar direct and review semantics", () => {
  it("uses dimensionless trimmed text, rejects blank, accepts explicit TBD and skips only exact normalized no-ops", async () => {
    const model = structuredClone(await loadPreviewModel()); const first = model.nodes[0]; first.label = "Original";
    const apply = vi.fn(async (_intent: import("../../../types").EditorOperationIntent) => ({ applied: true, messages: [] }));
    const view = render(<ModelTree model={model} selection={{ type: "node", id: first.id }} onSelect={vi.fn()} onApplyCellIntent={apply} />);
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    const edit = () => { fireEvent.doubleClick(screen.getByTestId(`table-cell-${first.id}-label`)); return screen.getByRole("textbox", { name: `${first.id} Label` }); };
    expect(screen.getByRole("button", { name: "Sort Label" })).not.toHaveTextContent("[");
    fireEvent.change(edit(), { target: { value: " Original " } }); fireEvent.click(screen.getByRole("button", { name: "Apply" })); expect(apply).not.toHaveBeenCalled();
    fireEvent.change(edit(), { target: { value: "  " } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    expect(screen.getByRole("alert")).toHaveTextContent("explicitly enter TBD"); expect(apply).not.toHaveBeenCalled();
    fireEvent.change(screen.getByRole("textbox", { name: `${first.id} Label` }), { target: { value: " TBD " } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(apply).toHaveBeenCalledTimes(1));
    expect(apply.mock.calls[0][0]).toMatchObject({ target: { object_type: "Node", ref: first.id }, change: { field_path: "label", before: "Original", after: "TBD", unit: "none", dimension: "dimensionless" } });
    await waitFor(() => expect(screen.queryByRole("button", { name: "Apply" })).not.toBeInTheDocument());
    const unusual = structuredClone(model); unusual.nodes[0].label = " Original ";
    view.rerender(<ModelTree model={unusual} selection={{ type: "node", id: first.id }} onSelect={vi.fn()} onApplyCellIntent={apply} />);
    edit(); fireEvent.click(screen.getByRole("button", { name: "Apply" })); await waitFor(() => expect(apply).toHaveBeenCalledTimes(2));
    expect(apply.mock.calls[1][0].change).toMatchObject({ before: " Original ", after: "Original" });
  });

  it("keeps structured Node provenance read-only and refuses a shape change during an active string edit", async () => {
    const model = structuredClone(await loadPreviewModel()); const first = model.nodes[0];
    const apply = vi.fn(async (_intent: import("../../../types").EditorOperationIntent) => ({ applied: true, messages: [] }));
    const props = { model, selection: { type: "node" as const, id: first.id }, onSelect: vi.fn(), onApplyCellIntent: apply };
    const view = render(<ModelTree {...props} />); fireEvent.click(screen.getByTestId("layout-mode-grid"));
    fireEvent.doubleClick(screen.getByTestId(`table-cell-${first.id}-provenance`));
    fireEvent.change(screen.getByRole("textbox", { name: `${first.id} Provenance` }), { target: { value: "replacement" } });
    const changed = structuredClone(model); const provenance = { source_ref: "keep-object", detail: ["a", "b"] };
    Object.assign(changed.nodes[0], { provenance }); view.rerender(<ModelTree {...props} model={changed} />);
    fireEvent.click(screen.getByRole("button", { name: "Apply" })); expect(screen.getByRole("alert")).toHaveTextContent("now read-only"); expect(apply).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    const cell = screen.getByTestId(`table-cell-${first.id}-provenance`); expect(cell).toHaveTextContent(JSON.stringify(provenance));
    fireEvent.doubleClick(cell); expect(screen.queryByRole("textbox", { name: `${first.id} Provenance` })).not.toBeInTheDocument();
    fireEvent.click(screen.getByTestId("node-grid-review-disclosure"));
    fireEvent.doubleClick(screen.getByTestId(`review-cell-${first.id}-provenance`)); expect(screen.queryByRole("button", { name: "Keep draft" })).not.toBeInTheDocument();
    expect(changed.nodes[0].provenance).toEqual(provenance);
    const numeric = structuredClone(model); Object.assign(numeric.nodes[0], { provenance: 42 });
    view.rerender(<ModelTree {...props} model={numeric} />);
    fireEvent.doubleClick(screen.getByTestId(`review-cell-${first.id}-provenance`));
    expect(screen.queryByRole("button", { name: "Keep draft" })).not.toBeInTheDocument();
    expect(screen.getByTestId(`review-cell-${first.id}-provenance`)).toHaveTextContent("42");
  });

  it("stages raw blanks and invalid quantities, keeps character-start drafts, and queues current before/unit exactly once", async () => {
    const model = structuredClone(await loadPreviewModel()); const first = model.nodes[0]; const queue = vi.fn(); const apply = vi.fn();
    const props = { model, selection: { type: "node" as const, id: first.id }, onSelect: vi.fn(), onQueueIntent: queue, onApplyCellIntent: apply };
    const view = render(<ModelTree {...props} />); fireEvent.click(screen.getByTestId("layout-mode-grid")); fireEvent.click(screen.getByTestId("node-grid-review-disclosure"));
    expect(screen.getByTestId("entity-grid-boundary")).toHaveTextContent("Blank or whitespace text becomes TBD when queued");
    fireEvent.keyDown(screen.getByTestId(`review-cell-${first.id}-label`), { key: "Q" });
    fireEvent.click(screen.getByRole("button", { name: "Keep draft" })); expect(screen.getByRole("status")).toHaveTextContent("Draft retained; model unchanged.");
    expect(screen.getByTestId(`review-cell-${first.id}-label`)).toHaveTextContent("Q"); expect(apply).not.toHaveBeenCalled(); expect(queue).not.toHaveBeenCalled();
    fireEvent.change(reviewEditor(`${first.id}-label`), { target: { value: "   " } });
    fireEvent.change(reviewEditor(`${first.id}-x`), { target: { value: "invalid quantity" } });
    const updated = structuredClone(model); updated.nodes[0].position.x = 8; updated.project.units.length = "mm";
    view.rerender(<ModelTree {...props} model={updated} />);
    // No helper blur: Queue consumes the active raw editor and cancels its local state.
    fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
    expect(queue).toHaveBeenCalledTimes(2); expect(queue.mock.calls[0][0].change.after).toBe("TBD");
    expect(queue.mock.calls[1][0].change).toMatchObject({ before: "8", unit: "mm", after: '{"value":"invalid quantity","unit":"mm"}' });
    expect(screen.queryByRole("button", { name: "Keep draft" })).not.toBeInTheDocument(); expect(screen.getByTestId("entity-grid-change-count")).toHaveTextContent("0 changed cells"); expect(apply).not.toHaveBeenCalled();
  });

  it("filtered Queue preserves hidden drafts and explicit Clear removes all drafts without active-editor resurrection", async () => {
    const model = await loadPreviewModel(); const queue = vi.fn();
    render(<ModelTree model={model} selection={{ type: "node", id: model.nodes[0].id }} onSelect={vi.fn()} onQueueIntent={queue} />);
    fireEvent.click(screen.getByTestId("layout-mode-grid")); fireEvent.click(screen.getByTestId("node-grid-review-disclosure"));
    fireEvent.change(reviewEditor(`${model.nodes[0].id}-label`), { target: { value: "first" } });
    fireEvent.change(reviewEditor(`${model.nodes[1].id}-provenance`), { target: { value: "hidden" } });
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: model.nodes[0].id } });
    fireEvent.click(screen.getByTestId("queue-entity-grid-intents")); expect(queue).toHaveBeenCalledTimes(1);
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "" } });
    expect(reviewEditor(`${model.nodes[1].id}-provenance`)).toHaveValue("hidden");
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: model.nodes[0].id } });
    fireEvent.change(reviewEditor(`${model.nodes[0].id}-label`), { target: { value: "clear active" } });
    fireEvent.click(screen.getByTestId("clear-entity-grid-drafts"));
    fireEvent.change(screen.getByTestId("model-tree-filter-input"), { target: { value: "" } });
    expect(screen.getByTestId("entity-grid-change-count")).toHaveTextContent("0 changed cells");
    expect(screen.getByTestId(`review-cell-${model.nodes[1].id}-provenance`)).toHaveTextContent(model.nodes[1].provenance); expect(queue).toHaveBeenCalledTimes(1);
  });
});

it("retires a source-unit-stale review editor without overwriting lifted text and queues on the current unit", async () => {
  const model = structuredClone(await loadPreviewModel()); const first = model.nodes[0]; const queue = vi.fn();
  const props = { model, selection: { type: "node" as const, id: first.id }, onSelect: vi.fn(), onQueueIntent: queue };
  const view = render(<ModelTree {...props} />); fireEvent.click(screen.getByTestId("layout-mode-grid")); fireEvent.click(screen.getByTestId("node-grid-review-disclosure"));
  const editor = reviewEditor(`${first.id}-x`); fireEvent.change(editor, { target: { value: "2.500" } });
  const current = structuredClone(model); current.project.units.length = "mm"; current.nodes[0].position.x = 8;
  view.rerender(<ModelTree {...props} model={current} />);
  fireEvent.change(editor, { target: { value: "obsolete late text" } }); expect(editor).toHaveValue("2.500");
  fireEvent.click(screen.getByRole("button", { name: "Keep draft" })); expect(screen.getByRole("alert")).toHaveTextContent("entered unit changed");
  fireEvent.click(screen.getByRole("button", { name: "Cancel" })); expect(screen.getByRole("status")).toHaveTextContent("without rewriting retained drafts");
  expect(screen.getByTestId(`review-cell-${first.id}-x`)).toHaveTextContent("2.500");
  fireEvent.click(screen.getByTestId("queue-entity-grid-intents")); expect(queue).toHaveBeenCalledTimes(1);
  expect(queue.mock.calls[0][0].change).toMatchObject({ before: "8", unit: "mm", after: '{"value":2.5,"unit":"mm"}' });
});

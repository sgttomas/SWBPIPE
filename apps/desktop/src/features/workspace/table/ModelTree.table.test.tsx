import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ModelTree } from "../../model-tree/ModelTree";
import { loadPreviewModel } from "../../../services/previewService";

describe("node table and retained review transitions", () => {
  it("preserves an invalid editor through actual family and Tree transitions with inactive surfaces inert", async () => {
    const model = await loadPreviewModel(); const apply = vi.fn(async () => ({ applied: true, messages: [] }));
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
    fireEvent.change(screen.getByTestId(`entity-grid-input-${model.nodes[0].id}-x`), { target: { value: "2.500" } });
    fireEvent.click(summary); await waitFor(() => expect(summary).toHaveTextContent("1 retained draft"));
    fireEvent.click(screen.getByTestId("entity-grid-type-pipes")); fireEvent.click(screen.getByTestId("entity-grid-type-nodes"));
    fireEvent.click(screen.getByTestId("layout-mode-tree")); fireEvent.click(screen.getByTestId("layout-mode-grid"));
    expect(summary).toHaveTextContent("1 retained draft"); expect(queue).not.toHaveBeenCalled();
    fireEvent.click(summary); expect(screen.getByTestId(`entity-grid-input-${model.nodes[0].id}-x`)).toHaveValue("2.500");
    fireEvent.click(screen.getByTestId("queue-entity-grid-intents")); expect(queue).toHaveBeenCalledTimes(1);
    expect(queue.mock.calls[0][0].change.after).toBe('{"value":2.5,"unit":"m"}');
  });
  it("does not fabricate a coordinate unit when the source model omits it", async () => {
    const model = structuredClone(await loadPreviewModel()); delete (model.project.units as Partial<typeof model.project.units>).length;
    const apply = vi.fn(async () => ({ applied: true, messages: [] }));
    render(<ModelTree model={model} selection={{ type: "node", id: model.nodes[0].id }} onSelect={vi.fn()} onApplyCellIntent={apply} />);
    fireEvent.click(screen.getByTestId("layout-mode-grid")); expect(screen.getByRole("button", { name: "Sort X" })).toHaveTextContent("unit missing");
    fireEvent.doubleClick(screen.getByTestId(`table-cell-${model.nodes[0].id}-x`));
    fireEvent.change(screen.getByRole("textbox", { name: `${model.nodes[0].id} X []` }), { target: { value: "2" } }); fireEvent.click(screen.getByRole("button", { name: "Apply" }));
    expect(screen.getByRole("alert")).toHaveTextContent("no declared length unit"); expect(apply).not.toHaveBeenCalled();
  });

  it("uses one visible review work area while retaining direct invalid text and bulk drafts", async () => {
    const model = await loadPreviewModel(); const apply = vi.fn(async () => ({ applied: true, messages: [] })); const queue = vi.fn();
    render(<ModelTree boundedGrid model={model} selection={{ type: "node", id: model.nodes[0].id }} onSelect={vi.fn()} onApplyCellIntent={apply} onQueueIntent={queue} />);
    fireEvent.click(screen.getByTestId("layout-mode-grid")); fireEvent.doubleClick(screen.getByTestId(`table-cell-${model.nodes[0].id}-x`));
    const editor = screen.getByRole("textbox", { name: `${model.nodes[0].id} X [m]` }); fireEvent.change(editor, { target: { value: "bad" } }); fireEvent.blur(editor);
    const summary = screen.getByTestId("node-grid-review-disclosure"); fireEvent.click(summary);
    await waitFor(() => expect(screen.getByTestId("engineering-table").parentElement).toHaveAttribute("hidden"));
    expect(screen.getByTestId("engineering-table").parentElement).toHaveAttribute("inert"); expect(screen.getByTestId("retained-direct-draft")).toHaveTextContent("Direct coordinate edit retained");
    fireEvent.change(screen.getByTestId(`entity-grid-input-${model.nodes[0].id}-y`), { target: { value: "0.5" } });
    expect(summary).toHaveTextContent("Return to node coordinates"); fireEvent.click(summary);
    await waitFor(() => expect(editor).toBeVisible()); expect(editor).toHaveValue("bad"); expect(summary).toHaveTextContent("1 retained draft");
    expect(apply).not.toHaveBeenCalled(); expect(queue).not.toHaveBeenCalled(); fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
  });

});

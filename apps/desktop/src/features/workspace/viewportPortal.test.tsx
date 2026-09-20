import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import { PipeViewport } from "../viewport/PipeViewport";

const emptySelection = Object.freeze({
  orderedKeys: [],
  primaryKey: null,
  rangeAnchorKey: null,
  focusKey: null,
  preparationEpoch: 0
});

describe("viewport authoring panel portal", () => {
  it("keeps the panel in the viewport without a target and moves the same working controls into a target when supplied", async () => {
    const model = await loadPreviewModel();
    const props = {
      armedCreationTool: "node" as const,
      model,
      onSelect: vi.fn(() => emptySelection),
      selection: { type: "node" as const, id: model.nodes[0].id }
    };
    const view = render(<PipeViewport {...props} />);
    const shell = view.container.querySelector<HTMLElement>(".viewport-shell")!;
    expect(within(shell).getByTestId("viewport-editor-intents")).toBeInTheDocument();

    const target = document.createElement("div");
    target.id = "shell-routing-panel-test-target";
    document.body.appendChild(target);
    view.rerender(<PipeViewport {...props} authoringPanelContainer={target} />);
    const portalled = within(target).getByTestId("viewport-editor-intents");
    expect(within(shell).queryByTestId("viewport-editor-intents")).toBeNull();
    fireEvent.change(within(portalled).getByTestId("viewport-create-node-id"), { target: { value: "node:portal" } });
    expect(within(portalled).getByTestId("viewport-create-node-id")).toHaveValue("node:portal");

    view.rerender(<PipeViewport {...props} authoringPanelContainer={null} />);
    expect(within(shell).getByTestId("viewport-editor-intents")).toBeInTheDocument();
    target.remove();
  });
});

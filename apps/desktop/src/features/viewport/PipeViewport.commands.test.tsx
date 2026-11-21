import { act, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider } from "../display-units";
import { fitRowCommand } from "../model-tree/runSheet/RunSheet";
import { loadPreviewModel } from "../../services/previewService";
import type { EntityRef, PreviewModel } from "../../types";
import { PipeViewport } from "./PipeViewport";
import type { ViewportViewCommand } from "./viewportSelection";

// The view's commands as other surfaces send them. jsdom has no WebGL, so a fit that would move the
// camera reports the renderer unavailable; a fit with nothing to frame says so and moves nothing.

function mount(model: PreviewModel, selection: EntityRef) {
  const viewCommandRef: { current: ((command: ViewportViewCommand) => void) | null } = { current: null };
  render(<DisplayUnitsProvider initialPreference="SI"><PipeViewport model={model} selection={selection} onSelect={vi.fn()} viewCommandRef={viewCommandRef} /></DisplayUnitsProvider>);
  const send = (command: ViewportViewCommand) => act(() => viewCommandRef.current?.(command));
  const status = () => screen.getByTestId("viewport-view-command-status");
  return { send, status };
}

describe("Fit row in viewport", () => {
  it("carries the row's own target: a Fit Selection of that entity, or of none", () => {
    expect(fitRowCommand({ type: "pipe", id: "pipe:B" })).toEqual({ type: "fit-selection", targets: [{ type: "pipe", id: "pipe:B" }] });
    expect(fitRowCommand(null)).toEqual({ type: "fit-selection", targets: [] });
  });

  it("with A selected, frames the row's B; a row with nothing drawn moves nothing and claims no fit", async () => {
    const model = await loadPreviewModel();
    const [a, b] = model.pipe_segments;
    const { send, status } = mount(model, { type: "pipe", id: a.id });
    send(fitRowCommand({ type: "pipe", id: b.id }));
    expect(status()).toHaveTextContent("Fit Selection requested; the 3D renderer is unavailable.");
    send(fitRowCommand(null));
    expect(status()).toHaveTextContent("Fit Selection: nothing it names is drawn; the view is unchanged.");
  });
});

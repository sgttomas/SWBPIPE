import { act, render, screen, waitFor } from "@testing-library/react";
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

describe("Measure, armed, refusing a pick", () => {
  const distance = () => screen.queryByTestId("viewport-measurement-readout")?.querySelector('[data-measurement-label="Distance"]') ?? null;
  const prompt = () => screen.getByTestId("viewport-measure-prompt");

  it("says why in its prompt until the next accepted pick, and clears the finished measurement it does not answer", async () => {
    const model = await loadPreviewModel();
    const pipe = model.pipe_segments[0];
    const support = model.supports[0];
    const { send } = mount(model, { type: "project", id: model.project.id });
    send({ type: "set-measurement", active: true });
    expect(prompt()).toHaveTextContent("Measure: pick a node or a pipe · press Measure again to end");
    send({ type: "apply-measurement-target", ref: { type: "pipe", id: pipe.id } });
    await waitFor(() => expect(distance()).not.toBeNull());
    send({ type: "apply-measurement-target", ref: { type: "support", id: support.id } });
    expect(prompt()).toHaveTextContent("Measure: a support cannot be measured · pick a node or a pipe · press Measure again to end");
    expect(distance()).toBeNull();
    // The next accepted pick ends the refusal.
    send({ type: "apply-measurement-target", ref: { type: "node", id: pipe.from } });
    expect(prompt()).toHaveTextContent("Measure: pick a second node · press Measure again to end");
    // A refusal while a first node waits keeps that node.
    send({ type: "apply-measurement-target", ref: { type: "support", id: support.id } });
    expect(prompt()).toHaveTextContent("Measure: a support cannot be measured · pick a second node · press Measure again to end");
    send({ type: "apply-measurement-target", ref: { type: "node", id: pipe.to } });
    await waitFor(() => expect(distance()).not.toBeNull());
  });
});

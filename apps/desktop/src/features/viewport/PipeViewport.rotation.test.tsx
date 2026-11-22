import { render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider } from "../display-units";
import { loadBundledMechanicsReference, loadPreviewModel } from "../../services/previewService";
import { PipeViewport } from "./PipeViewport";
import { buildRotationOverlay } from "./deformationOverlay";

// The viewport's deformation disclosure carries the rotation glyph readout
// (DEL-07-05:1). All data are invented or the bundled invented reference.

describe("PipeViewport rotation readout", () => {
  it("offers no Deformation control, and so no rotation readout, without a result", async () => {
    // The overlay's own "not started" state is deformationOverlay.test.ts's; with no result the
    // view shows no Deformation control at all, so it states no deformed shape or rotation.
    const model = await loadPreviewModel();
    render(<DisplayUnitsProvider><PipeViewport model={model} selection={{ type: "project", id: model.project.id }} onSelect={vi.fn()} /></DisplayUnitsProvider>);
    expect(screen.queryByTestId("viewport-deformation-status")).toBeNull();
    expect(screen.queryByTestId("viewport-rotation-summary")).toBeNull();
    expect(screen.queryByTestId("viewport-deformation-summary")).toBeNull();
  });

  it("shows the emitted rotation maximum and converts only its display unit", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const overlay = buildRotationOverlay(model, source);
    expect(overlay.state).toBe("available");
    render(<DisplayUnitsProvider initialPreference="US"><PipeViewport model={model} result={source} selection={{ type: "project", id: model.project.id }} onSelect={vi.fn()} /></DisplayUnitsProvider>);
    const summary = screen.getByTestId("viewport-rotation-summary");
    expect(summary).toHaveTextContent(`rotation: available; nodes=${overlay.nodeCount}; drawn=${overlay.drawnCount}; max |component|=`);
    await waitFor(() => expect(summary.querySelector('[data-display-status="converted"]')).not.toBeNull());
    expect(summary).toHaveTextContent("deg");
    expect(screen.getByTestId("viewport-rotation-boundary")).toHaveTextContent(overlay.boundary);
  });
});

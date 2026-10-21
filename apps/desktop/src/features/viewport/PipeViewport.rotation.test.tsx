import { render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider } from "../display-units";
import { loadBundledMechanicsReference, loadPreviewModel } from "../../services/previewService";
import { PipeViewport } from "./PipeViewport";
import { buildRotationOverlay } from "./deformationOverlay";

// The viewport's deformation disclosure carries the rotation glyph readout
// (DEL-07-05:1). All data are invented or the bundled invented reference.

describe("PipeViewport rotation readout", () => {
  it("reports a not-started rotation overlay without a result", async () => {
    const model = await loadPreviewModel();
    render(<DisplayUnitsProvider><PipeViewport model={model} selection={{ type: "project", id: model.project.id }} onSelect={vi.fn()} /></DisplayUnitsProvider>);
    expect(screen.getByTestId("viewport-rotation-summary")).toHaveTextContent("rotation: not started; result rows=0");
    expect(screen.getByTestId("viewport-rotation-boundary")).toHaveTextContent("scale=not_generated; professional_claim=false");
    // The translational readout is unchanged.
    expect(screen.getByTestId("viewport-deformation-summary")).toHaveTextContent("not started; result rows=0");
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

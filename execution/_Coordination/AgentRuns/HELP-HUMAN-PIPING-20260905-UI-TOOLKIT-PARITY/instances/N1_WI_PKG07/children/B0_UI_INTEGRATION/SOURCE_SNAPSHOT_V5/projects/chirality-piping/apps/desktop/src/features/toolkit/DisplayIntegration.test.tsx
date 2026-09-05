import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider, DisplayUnitSelector } from "../display-units";
import { PropertyInspector } from "../model-tree/PropertyInspector";
import { ModelTree } from "../model-tree/ModelTree";
import { PipeViewport, buildDeformationOverlay } from "../viewport/PipeViewport";
import { selectedProperties, selectedPropertyRows } from "../model-workspace/modelView";
import { loadPreviewModel } from "../../services/previewService";
import { computeModelHash } from "../../services/hashService";

describe("typed model display integration", () => {
  it("keeps typed node coordinate source units and the legacy property text aligned", async () => {
    const model = await loadPreviewModel();
    model.project.units.length = "mm";
    model.nodes[0].position.x = 1250;
    const selection = { type: "node" as const, id: model.nodes[0].id };
    const row = selectedPropertyRows(model, selection).find(([label]) => label === "Position")![1];
    expect(row.quantities?.[0]).toEqual({ value: 1250, unit: "mm", dimension_id: "length", label: "x" });
    expect(selectedProperties(model, selection).find(([label]) => label === "Position")?.[1]).toBe(row.text);
  });

  it("converts actual inspector and grid readouts through Rust while preserving original model, hash and typed drafts", async () => {
    const model = await loadPreviewModel();
    model.nodes[0].position.x = 1;
    const exact = JSON.stringify(model);
    const hash = await computeModelHash(model);
    const selection = { type: "node" as const, id: model.nodes[0].id };
    render(<DisplayUnitsProvider><DisplayUnitSelector /><PropertyInspector model={model} selection={selection} onQueueIntent={vi.fn()} /><ModelTree model={model} selection={selection} onQueueIntent={vi.fn()} onSelect={vi.fn()} /></DisplayUnitsProvider>);
    fireEvent.change(screen.getByTestId("editor-intent-field"), { target: { value: "position.x" } });
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    const draft = screen.getByTestId(`entity-grid-input-${model.nodes[0].id}-x`);
    fireEvent.change(draft, { target: { value: "2.3450" } });
    fireEvent.change(screen.getByLabelText("Display units"), { target: { value: "US" } });
    const position = screen.getByText("Position").parentElement!;
    await waitFor(() => expect(position.querySelector('[data-display-status="converted"]')).not.toBeNull());
    expect(position).toHaveTextContent("in");
    expect(draft).toHaveValue("2.3450");
    expect(screen.getByTestId("editor-intent-value")).toHaveValue("1");
    expect(JSON.stringify(model)).toBe(exact);
    expect((await computeModelHash(model))?.value).toBe(hash?.value);
    fireEvent.change(screen.getByLabelText("Display units"), { target: { value: "entered" } });
    expect(position).toHaveTextContent("x=1 m");
    expect(draft).toHaveValue("2.3450");
  });

  it("retains deformation geometry while converting only the viewport scale readout", async () => {
    const model = await loadPreviewModel();
    const before = buildDeformationOverlay(model, null);
    render(<DisplayUnitsProvider initialPreference="US"><PipeViewport model={model} selection={{ type: "project", id: model.project.id }} onSelect={vi.fn()} /></DisplayUnitsProvider>);
    const scale = screen.getByTestId("viewport-scale-bar");
    await waitFor(() => expect(scale.querySelector('[data-display-status="converted"]')).not.toBeNull());
    expect(scale).toHaveTextContent("in");
    expect(buildDeformationOverlay(model, null)).toEqual(before);
  });
});

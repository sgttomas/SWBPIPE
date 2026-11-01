// G6: enter the reference system's F1 in either creation form without later
// end-size corrections. Exercise the emitted operation and browser save/reopen.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import referenceJson from "../../../../../fixtures/reference_system/reference_system.json";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { applyModelOperation } from "../../services/operationService";
import { createLocalProject, openLocalProject } from "../../services/projectService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { changeFormControl, expectFormControlValue } from "../../test-support/workspaceTestControls";
import { DisplayUnitsProvider } from "../display-units";
import { PropertyInspector } from "../model-tree/PropertyInspector";
import { PipeViewport } from "../viewport/PipeViewport";
import { emptySelection } from "../workspace/selectionState";

vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: async () => ({ route: "tauri_unit_catalog", catalog: unitCatalogFixture() })
}));
afterEach(cleanup);

describe("rigid-component quantity units (G6)", () => {
  it.each(["inspector", "viewport"] as const)("creates and reopens reference F1 from the %s form with body/COG in m and end sizes in mm", async (surface) => {
    const model = structuredClone(referenceJson) as unknown as PreviewModel;
    model.diagnostics = [];
    const target = model.components.find((component) => component.id === "component:RS-F1")!;
    model.components = model.components.filter((component) => component !== target);
    const before = structuredClone(model);
    const queue = vi.fn<(intent: EditorOperationIntent) => void>();
    const selection = { type: "node" as const, id: "node:RS-030" };
    render(<DisplayUnitsProvider initialPreference="SI">{surface === "inspector"
      ? <PropertyInspector model={model} selection={selection} onQueueIntent={queue} />
      : <PipeViewport model={model} selection={selection} onSelect={() => emptySelection()} onQueueIntent={queue} armedCreationTool="component" />
    }</DisplayUnitsProvider>);
    const prefix = surface === "inspector" ? "create" : "viewport-create";
    const control = (field: string) => screen.getByTestId(`${prefix}-component-${field}`);
    const enter = (field: string, value: string) => changeFormControl(control(field), { target: { value } });
    enter("kind", "flange");
    await waitFor(() => {
      if (control("end-a-size-unit").getAttribute("aria-expanded") !== "true") fireEvent.click(control("end-a-size-unit"));
      expect(screen.getByRole("option", { name: /mm/ })).toBeInTheDocument();
    });
    fireEvent.keyDown(control("end-a-size-unit"), { key: "Escape" });
    for (const unit of ["length-unit", "end-a-size-unit", "end-b-size-unit", "cog-unit"]) {
      expectFormControlValue(control(unit), "m");
      expect(control(unit)).toHaveAccessibleName(/unit/);
    }
    enter("id", target.id);
    enter("label", target.label);
    const picker = within(control("pipe"));
    fireEvent.change(picker.getByRole("combobox"), { target: { value: "pipe:RS-P020" } });
    fireEvent.click(picker.getByRole("option", { name: /pipe:RS-P020$/ }));
    enter("body-length", "0.15");
    enter("end-a-size", "270");
    enter("end-a-size-unit", "mm");
    enter("end-b-size", "270");
    enter("end-b-size-unit", "mm");
    enter("weight", "350");
    enter("cog-x", "-0.075");
    enter("cog-y", "0");
    enter("cog-z", "0");
    enter("cog-unit", "mm");
    expectFormControlValue(control("length-unit"), "m");
    expectFormControlValue(control("end-a-size-unit"), "mm");
    expect(control("cog-x")).toHaveValue("-0.075");
    enter("cog-unit", "m");
    enter("end-a-ref", "node:RS-020 (body end on RS-P020)");
    enter("end-b-ref", "node:RS-030 (body end on RS-P020)");
    enter("stiffness-ref", target.geometry!.stiffness_behavior_reference!);
    enter("source", target.geometry!.rigid_component_source_reference!);
    enter("provenance", target.provenance);
    enter("stiffness-scale", "10");
    enter("scale-source", target.modifiers!.source_reference!);
    enter("solver-consumption", "mechanics_geometry_only");
    const button = screen.getByTestId(surface === "inspector" ? "queue-create-component-intent" : "queue-explicit-component-intent");
    expect(button).toBeEnabled();
    fireEvent.click(button);
    expect(queue).toHaveBeenCalledOnce();
    const intent = queue.mock.calls[0][0];
    expect(JSON.parse(intent.change.after)).toEqual(target);
    expect(intent.validation.unit_validation).toContain("end_a_size=dec018_catalog_dimension_match");
    expect(model).toEqual(before);

    const applied = await applyModelOperation(model, intent, null);
    expect(applied.validation.application_status, JSON.stringify(applied.diagnostics)).toBe("applied_to_session_model");
    expect(applied.applied_model!.components.find((component) => component.id === target.id)).toEqual(target);
    await createLocalProject(applied.applied_model!);
    const reopened = await openLocalProject(model.project.id);
    expect(reopened!.model.components.find((component) => component.id === target.id)).toEqual(target);
  });
});

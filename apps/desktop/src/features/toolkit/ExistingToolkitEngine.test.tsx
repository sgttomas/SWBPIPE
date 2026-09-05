import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { SectionAssignment } from "./SectionAssignment";
import { GuardedRemoval } from "./GuardedRemoval";
import { MaterialTemperatureForm } from "../material-temperature/MaterialTemperatureForm";
import { SupportConfigurationForm } from "../support-configuration/SupportConfigurationForm";
import { WindExposureForm } from "../wind-exposure/WindExposureForm";
import { PropertyInspector } from "../model-tree/PropertyInspector";
import { makeRichIntent } from "../rich-authoring/formSupport";
import { loadPreviewModel } from "../../services/previewService";
import { applyModelOperation } from "../../services/operationService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
const enter = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
async function applied(model: PreviewModel, intent: EditorOperationIntent) {
  const result = await applyModelOperation(model, intent, null);
  expect(result.validation.application_status, JSON.stringify(result.diagnostics)).toBe("applied_to_session_model");
  return result.applied_model!;
}

describe("existing toolkit shared engine", () => {
  it("assigns a shared section, propagates edits to all bound pipes, then detaches without changing quantities", async () => {
    let model = await loadPreviewModel();
    model.sections = [{ id: "section:shared-test", name: "Invented shared section", section_type: "pipe", properties: { outside_diameter: { value: 170, unit: "mm" }, wall_thickness: { value: 7, unit: "mm" } }, provenance: "invented test" }];
    for (const id of model.pipe_segments.slice(0, 2).map((pipe) => pipe.id)) {
      const queue = vi.fn();
      render(<SectionAssignment model={model} selection={{ type: "pipe", id }} onQueueIntent={queue} />);
      enter("Shared section", "section:shared-test");
      const button = screen.getByRole("button", { name: "Queue section assignment" });
      await waitFor(() => expect(button).toBeEnabled()); fireEvent.click(button);
      expect(queue.mock.calls[0][0].change.after).toBe('{"section_ref":"section:shared-test"}');
      model = await applied(model, queue.mock.calls[0][0]); cleanup();
    }
    const edit = makeRichIntent({ object_type: "Section", ref: "section:shared-test" }, "set_field", "properties.outside_diameter.value", "170", { value: 190, unit: "mm" }, "Shared diameter");
    edit.change.unit = "mm"; edit.change.dimension = "length";
    model = await applied(model, edit);
    for (const pipe of model.pipe_segments.slice(0, 2)) expect(pipe.section.outside_diameter).toEqual({ value: 190, unit: "mm" });
    const bound = model.pipe_segments[0]; const exact = structuredClone(bound.section);
    const blocked = makeRichIntent({ object_type: "Element", ref: bound.id }, "set_field", "section.outside_diameter.value", "190", { value: 180, unit: "mm" }, "Independent diameter");
    blocked.change.unit = "mm"; blocked.change.dimension = "length";
    expect((await applyModelOperation(model, blocked, null)).validation.application_status).not.toBe("applied_to_session_model");
    const queue = vi.fn(); render(<SectionAssignment model={model} selection={{ type: "pipe", id: bound.id }} onQueueIntent={queue} />);
    const detach = screen.getByRole("button", { name: "Queue detachment to local dimensions" });
    await waitFor(() => expect(detach).toBeEnabled()); fireEvent.click(detach);
    expect(queue.mock.calls[0][0].change.after).toBe('{"source_section_ref":"section:shared-test"}');
    const detached = await applied(model, queue.mock.calls[0][0]);
    expect(detached.pipe_segments[0].section_ref).toBeUndefined(); expect(detached.pipe_segments[0].section).toEqual(exact);
  });
  it("blocks a referenced material removal through the actual engine without changing the model", async () => {
    const model = await loadPreviewModel(); const original = JSON.stringify(model); const queue = vi.fn();
    render(<GuardedRemoval model={model} selection={{ type: "material", id: model.materials![0].id }} onQueueIntent={queue} />);
    const button = screen.getByRole("button", { name: "Queue removal for review" });
    await waitFor(() => expect(button).toBeEnabled()); fireEvent.click(button);
    const result = await applyModelOperation(model, queue.mock.calls[0][0], null);
    expect(result.validation.application_status).not.toBe("applied_to_session_model");
    expect(result.diagnostics.some((diagnostic) => /REFERENCED/.test(diagnostic.code))).toBe(true);
    expect(JSON.stringify(model)).toBe(original);
  });
  it("applies the reviewed support, material temperature and wind forms through the shared engine", async () => {
    let model = await loadPreviewModel(); let queue = vi.fn();
    render(<SupportConfigurationForm model={model} selection={{ type: "support", id: model.supports[0].id }} onQueueIntent={queue} />);
    enter("Support family", "line_stop");
    for (const dof of ["UX", "UY", "UZ", "RX", "RY", "RZ"]) {
      const checkbox = screen.getByLabelText(`Restrain ${dof}`) as HTMLInputElement;
      if (checkbox.checked !== (dof === "UX")) fireEvent.click(checkbox);
    } fireEvent.click(screen.getByRole("button", { name: "Queue support configuration" }));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce()); model = await applied(model, queue.mock.calls[0][0]); expect(model.supports[0].family).toBe("line_stop"); expect(model.supports[0].restraints).toEqual(["UX"]); cleanup();
    queue = vi.fn(); render(<MaterialTemperatureForm model={model} selection={{ type: "material", id: model.materials![0].id }} onQueueIntent={queue} />);
    fireEvent.click(screen.getByRole("button", { name: "Add temperature point" })); enter("Point 1 ID", "temperature:invented");
    for (const [label, value, unit] of [["Temperature", "20", "degC"], ["Elastic modulus", "200000", "MPa"], ["Shear modulus", "80000", "MPa"], ["Thermal expansion coefficient", "0.00001", "1/degC"]]) {
      enter(`Point 1 ${label} value`, value); enter(`Point 1 ${label} unit`, unit);
    }
    enter("Point 1 provenance", "invented unit test"); fireEvent.click(screen.getByRole("button", { name: "Queue temperature points" }));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce()); model = await applied(model, queue.mock.calls[0][0]); expect(model.materials![0].temperature_points![0].temperature).toEqual({ value: 20, unit: "degC" }); cleanup();
    model.load_cases[0].equivalent_static = { wind: {} }; queue = vi.fn();
    render(<WindExposureForm model={model} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={queue} />);
    fireEvent.click(screen.getByRole("button", { name: "Add exposure span" })); enter("Span 1 pipe", model.pipe_segments[0].id);
    enter("Span 1 start fraction value", "0.1"); enter("Span 1 start fraction unit", "1"); enter("Span 1 end fraction value", "0.5"); enter("Span 1 end fraction unit", "1");
    fireEvent.click(screen.getByRole("button", { name: "Queue wind exposure" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    model = await applied(model, queue.mock.calls[0][0]); expect(model.load_cases[0].equivalent_static!.wind!.exposed_spans![0].end_fraction).toEqual({ value: 0.5, unit: "1" });
  });
  it("queues explicit mass quantities including zero, retaining missing insulation as a warning", async () => {
    let model = await loadPreviewModel(); const queue = vi.fn();
    const view = render(<PropertyInspector model={model} selection={{ type: "pipe", id: model.pipe_segments[0].id }} onQueueIntent={queue} />);
    for (const [key, value, unit, dimension] of [["material_density", "7800", "kg/m^3", "density"], ["contents_density", "0", "kg/m^3", "density"], ["insulation_thickness", "0", "mm", "length"]]) {
      fireEvent.change(screen.getByTestId("editor-intent-field"), { target: { value: `section.${key}.value` } });
      expect(screen.getByTestId("editor-intent-unit")).toHaveValue("");
      fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value } }); fireEvent.change(screen.getByTestId("editor-intent-unit"), { target: { value: unit } });
      fireEvent.click(screen.getByTestId("queue-editor-intent")); const intent = queue.mock.calls.at(-1)![0];
      expect(intent.change).toMatchObject({ before: "TBD", after: JSON.stringify({ value: Number(value), unit }), dimension, unit });
      const result = await applyModelOperation(model, intent, null);
      expect(result.validation.application_status, JSON.stringify(result.diagnostics)).toBe("applied_to_session_model");
      if (key === "insulation_thickness") expect(result.diagnostics.some((diagnostic) => diagnostic.code === "OP-MASS-NOT-SOLVE-READY")).toBe(true);
      model = result.applied_model!; view.rerender(<PropertyInspector model={model} selection={{ type: "pipe", id: model.pipe_segments[0].id }} onQueueIntent={queue} />);
    }
    expect(screen.getByTestId("mass-input-completeness")).toHaveTextContent("Insulation mass is incomplete");
    expect(model.pipe_segments[0].section.insulation_density).toBeUndefined();
  });
});

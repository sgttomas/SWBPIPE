import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { MaterialConstitutiveForm, PressureAuthoringPanel } from "./PressureAuthoringPanel";
import { MaterialTemperatureForm } from "../material-temperature/MaterialTemperatureForm";
import type { PreviewModel, EditorOperationIntent } from "../../types";
import fixture from "../../../../../fixtures/model_operations/exact_pressure_authoring_model.json";
vi.mock("../../services/hashService", () => ({ canonicalJsonString: vi.fn(async (value: unknown) => JSON.stringify(value)) }));
afterEach(cleanup);
const model = fixture as PreviewModel;
const change = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
it("requires explicit profile choice and queues only the profile projection", async () => {
  const legacy = structuredClone(model); legacy.schema_version = "0.2.0"; delete legacy.pressure_contract;
  const queue = vi.fn(); render(<PressureAuthoringPanel model={legacy} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={queue} />);
  expect((screen.getByRole("button", { name: "Queue pressure profile" }) as HTMLButtonElement).disabled).toBe(true);
  change("Pressure profile", "exact_straight_pressure_v2"); fireEvent.click(screen.getByRole("button", { name: "Queue pressure profile" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const intent = queue.mock.calls[0][0] as EditorOperationIntent;
  expect(intent.target).toEqual({ object_type: "Model", ref: model.project.id });
  expect(JSON.parse(intent.change.after)).toEqual({ schema_version: "0.3.0", pressure_contract: model.pressure_contract });
  expect(legacy.load_cases).toEqual(model.load_cases);
});
it("retains regions and ordered terminal declarations through an edit", async () => {
  const queue = vi.fn(); render(<PressureAuthoringPanel model={model} selection={{ type: "load", id: model.load_cases[0].id }} loadCaseId={model.load_cases[0].id} onQueueIntent={queue} />);
  change("Region 1 terminal 2 closure transfer", "separately_supported_or_compensated");
  fireEvent.click(screen.getByRole("button", { name: "Queue pressure regions" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const intent = queue.mock.calls[0][0] as EditorOperationIntent; const after = JSON.parse(intent.change.after);
  expect(after[0].pressure).toEqual({ value: 2000, unit: "kPa" });
  expect(after[0].terminals.map((t: { node_ref: string }) => t.node_ref)).toEqual(model.load_cases[0].pressure_regions![0].terminals.map(t => t.node_ref));
  expect(after[0].terminals[1].closure_transfer).toBe("separately_supported_or_compensated");
  expect(after[0].provenance).toBe(model.load_cases[0].pressure_regions![0].provenance);
});
it("queues explicit empty regions for an undeclared case without changing its loads", async () => {
  const input = structuredClone(model); delete input.load_cases[1].pressure_regions;
  const queue = vi.fn(); render(<PressureAuthoringPanel model={input} selection={{ type: "load", id: input.load_cases[1].id }} loadCaseId={input.load_cases[1].id} onQueueIntent={queue} />);
  fireEvent.click(screen.getByRole("button", { name: "Queue explicit empty pressure regions" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  expect(queue.mock.calls[0][0].change.before).toBe("not_present"); expect(queue.mock.calls[0][0].change.after).toBe("[]");
  expect(input.load_cases[1].primitive_loads).toEqual(model.load_cases[1].primitive_loads);
});
it("allows explicit negative nu and does not synthesize G", async () => {
  const queue = vi.fn(); render(<MaterialConstitutiveForm model={model} selection={{ type: "material", id: model.materials![0].id }} onQueueIntent={queue} />);
  change("Poisson ratio nu value", "-0.2"); fireEvent.click(screen.getByRole("button", { name: "Queue constitutive properties" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const after = JSON.parse(queue.mock.calls[0][0].change.after); expect(after.poisson_ratio).toEqual({ value: -0.2, unit: "1" }); expect(after.shear_modulus).toBeUndefined();
});
it("preserves explicitly authored temperature-point nu", async () => {
  const input = structuredClone(model); input.materials![0].temperature_points = [{ id: "point:a", elastic_modulus: { value: 200, unit: "GPa" }, poisson_ratio: { value: -0.2, unit: "1" }, provenance: "synthetic" }];
  const queue = vi.fn(); render(<MaterialTemperatureForm model={input} selection={{ type: "material", id: input.materials![0].id }} onQueueIntent={queue} />);
  fireEvent.click(screen.getByRole("button", { name: "Queue temperature points" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  expect(JSON.parse(queue.mock.calls[0][0].change.after)[0].poisson_ratio).toEqual({ value: -0.2, unit: "1" });
});

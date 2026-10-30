import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { PressureAuthoringPanel, pressureModeText } from "./PressureAuthoringPanel";
import { pressureProfileTarget } from "./pressureContract";
import { selectedPropertyRows } from "../model-workspace/modelView";
import { evaluateModelDocumentLocal } from "../../services/projectService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { applyModelOperation, validateModelOperation } from "../../services/operationService";
import { makeRichIntent } from "../rich-authoring/formSupport";
import type { PreviewModel, EditorOperationIntent } from "../../types";
import fixture from "../../../../../fixtures/model_operations/exact_pressure_authoring_model.json";

afterEach(cleanup);
function model(): PreviewModel {
  const model = structuredClone(fixture) as PreviewModel;
  model.schema_version = "0.8.0";
  delete model.pressure_contract;
  for (const pipe of model.pipe_segments) {
    pipe.section.corrosion_allowance = { value: 0, unit: "m" };
  }
  return model;
}

it("B7b preserves authored null presence in the profile display and before projection", async () => {
  const current = model();
  current.pressure_contract = null as unknown as PreviewModel["pressure_contract"];
  expect(pressureModeText(current)).toContain("invalid authored pressure contract");
  const queue = vi.fn();
  render(<PressureAuthoringPanel model={current} selection={{ type: "project", id: current.project.id }} onQueueIntent={queue} />);
  fireEvent.change(screen.getByLabelText("Pressure profile"), { target: { value: "exact_pressure_v3" } });
  fireEvent.click(screen.getByRole("button", { name: "Queue pressure profile" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const intent = queue.mock.calls[0][0] as EditorOperationIntent;
  expect(JSON.parse(intent.change.before)).toEqual({ schema_version: "0.8.0", pressure_contract: null });
  expect(JSON.parse(intent.change.after)).toEqual(pressureProfileTarget(current));
  expect(current.pressure_contract).toBeNull();
});

it("B7b never supplies an absent user allowance during pressure activation", () => {
  const current = model();
  delete current.pipe_segments[0].section.corrosion_allowance;
  const before = structuredClone(current);
  const queue = vi.fn();
  render(<PressureAuthoringPanel model={current} selection={{ type: "project", id: current.project.id }} onQueueIntent={queue} />);
  fireEvent.change(screen.getByLabelText("Pressure profile"), { target: { value: "exact_pressure_v3" } });
  expect(screen.getByRole("button", { name: "Queue pressure profile" })).toBeDisabled();
  expect(screen.getByText(/Enter an explicit corrosion allowance/)).toBeInTheDocument();
  expect(queue).not.toHaveBeenCalled();
  expect(current).toEqual(before);
  const rows = selectedPropertyRows(current, { type: "pipe", id: current.pipe_segments[0].id });
  expect(rows.some(([label]) => label === "Corrosion allowance")).toBe(false);
});

it("B7b saves incomplete current document bytes by version and refuses all retired versions", () => {
  const current = model();
  current.pressure_contract = null as unknown as PreviewModel["pressure_contract"];
  delete current.pipe_segments[0].section.corrosion_allowance;
  const before = structuredClone(current);
  expect(evaluateModelDocumentLocal(current).status).toBe("current");
  expect(current).toEqual(before);
  for (const version of ["0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0"]) {
    const retired = structuredClone(current);
    retired.schema_version = version;
    expect(evaluateModelDocumentLocal(retired).status).toBe("retired_schema");
  }
});

it("B7b WASM common Review/Apply refuses an exhausted wall on an unbound scalar edit", async () => {
  const current = model();
  const pipe = current.pipe_segments[0];
  pipe.section.wall_thickness = { value: 0.01, unit: "m" };
  pipe.section.mill_tolerance = { value: 0.002, unit: "m" };
  const hash = await computeModelHash(current);
  for (const [value, accepted] of [[0.007, true], [0.009, false]] as const) {
    const intent = makeRichIntent({ object_type: "Element", ref: pipe.id },
      "set_field", "section.corrosion_allowance.value", "0", value, "Corrosion allowance");
    intent.change.unit = "m";
    intent.change.dimension = "length";
    const reviewed = await validateModelOperation(current, intent, hash);
    const applied = await applyModelOperation(current, intent, hash);
    expect(reviewed.applied_model).toBeNull();
    expect(Boolean(applied.applied_model)).toBe(accepted);
    if (!accepted) expect(applied.diagnostics.map(d => d.code)).toContain("OP-SECTION-BINDING-INVALID");
  }
});

it("B7b WASM applies explicit omitted-region then profile edits without manufacturing CA", async () => {
  let current = model();
  const originalSections = structuredClone(current.pipe_segments);
  const loadCase = current.load_cases[0];
  loadCase.primitive_loads = [];
  delete loadCase.pressure_regions;
  const regions = makeRichIntent({ object_type: "Load", ref: loadCase.id }, "update_load",
    "pressure_regions", "not_present", [], "Pressure regions");
  const outcome = await applyModelOperation(current, regions, await computeModelHash(current));
  expect(outcome.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
  current = outcome.applied_model as PreviewModel;
  const profile = makeRichIntent({ object_type: "Model", ref: current.project.id }, "set_field",
    "pressure_profile", await canonicalJsonString({ schema_version: "0.8.0" }),
    pressureProfileTarget(current), "Pressure profile");
  const activated = await applyModelOperation(current, profile, await computeModelHash(current));
  expect(activated.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
  expect((activated.applied_model as PreviewModel).pipe_segments).toEqual(originalSections);
  expect((activated.applied_model as PreviewModel).pressure_contract).toEqual(pressureProfileTarget(current).pressure_contract);
});

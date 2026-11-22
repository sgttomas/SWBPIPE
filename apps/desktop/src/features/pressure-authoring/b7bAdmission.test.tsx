import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { PressureAuthoringPanel, pressureModeText, shearChangeAndActivationBatch } from "./PressureAuthoringPanel";
import { applyOperationBatch, validateOperationBatch } from "../../services/operationBatchService";
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
  model.schema_version = "0.9.0";
  delete model.pressure_contract;
  // The pressure elongation setting needs the contract; a pressure-free document carries neither.
  if (model.model_settings) {
    delete model.model_settings.pressure_elongation;
    if (Object.keys(model.model_settings).length === 0) delete model.model_settings;
  }
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
  expect(JSON.parse(intent.change.before)).toEqual({ schema_version: "0.9.0", pressure_contract: null });
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
    "pressure_profile", await canonicalJsonString({ schema_version: "0.9.0" }),
    pressureProfileTarget(current), "Pressure profile");
  const activated = await applyModelOperation(current, profile, await computeModelHash(current));
  expect(activated.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
  expect((activated.applied_model as PreviewModel).pipe_segments).toEqual(originalSections);
  expect((activated.applied_model as PreviewModel).pressure_contract).toEqual(pressureProfileTarget(current).pressure_contract);
});

it("activation carries a pressure-free shear selection into the contract and out of the model settings", async () => {
  for (const stated of ["euler_bernoulli", "timoshenko_annulus_energy_v1"] as const) {
    const current = model();
    current.model_settings = { shear_model: stated };
    // The queued target carries the stated selection, not the authoring default.
    expect(pressureProfileTarget(current).pressure_contract?.shear_model).toBe(stated);
    const profile = makeRichIntent({ object_type: "Model", ref: current.project.id }, "set_field",
      "pressure_profile", await canonicalJsonString({ schema_version: "0.9.0" }),
      pressureProfileTarget(current), "Pressure profile");
    const hash = await computeModelHash(current);
    const reviewed = await validateModelOperation(current, profile, hash);
    expect(reviewed.diff_preview.map(row => [row.field_path, row.after])).toEqual([
      ["pressure_profile", profile.change.after], ["model_settings.shear_model", "not_present"]]);
    const activated = await applyModelOperation(current, profile, hash);
    expect(activated.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
    const applied = activated.applied_model as PreviewModel;
    expect(applied.pressure_contract?.shear_model).toBe(stated);
    expect(applied.model_settings).toBeUndefined();
    // The other model is refused rather than overwriting the stated choice.
    const other = stated === "euler_bernoulli" ? "timoshenko_annulus_energy_v1" : "euler_bernoulli";
    const conflicting = makeRichIntent({ object_type: "Model", ref: current.project.id }, "set_field",
      "pressure_profile", await canonicalJsonString({ schema_version: "0.9.0" }),
      pressureProfileTarget(current, other), "Pressure profile");
    const refused = await applyModelOperation(current, conflicting, hash);
    expect(refused.applied_model).toBeFalsy();
    expect(refused.diagnostics.map(d => d.code)).toContain("OP-PRESSURE-PROFILE-SHEAR-MODEL-CONFLICT");
  }
});

it("a pick that differs from the stated model queues the setting's change, then the activation that carries it", async () => {
  for (const [stated, pick] of [["euler_bernoulli", "timoshenko_annulus_energy_v1"], ["timoshenko_annulus_energy_v1", "euler_bernoulli"]] as const) {
    const current = model();
    current.model_settings = { shear_model: stated };
    const batch = await shearChangeAndActivationBatch(current, pick);
    expect(batch.operations.map(op => [op.target.object_type, op.change.change_kind, op.change.field_path])).toEqual([
      ["Project", "set_model_setting", "model_settings.shear_model"], ["Model", "set_field", "pressure_profile"]]);
    const hash = await computeModelHash(current);
    const reviewed = await validateOperationBatch(current, batch, hash);
    expect(reviewed.validation.batch_validation_status).toBe("passed");
    // Three review rows: the setting changed, then the activation and the setting's move into the contract.
    const rows = reviewed.operation_outcomes.flatMap(step => step.diff_preview.map(row => [row.field_path, row.before, row.after]));
    expect(rows).toEqual([
      ["model_settings.shear_model", JSON.stringify(stated), JSON.stringify(pick)],
      ["pressure_profile", batch.operations[1].change.before, batch.operations[1].change.after],
      ["model_settings.shear_model", JSON.stringify(pick), "not_present"]]);
    const applied = await applyOperationBatch(current, batch, hash);
    expect(applied.validation.application_status).toBe("applied_to_session_model");
    expect(applied.applied_model?.pressure_contract?.shear_model).toBe(pick);
    expect(applied.applied_model?.model_settings).toBeUndefined();
    // The activation alone, naming the pick, is still refused: the stated model's change is not with it.
    const bare = makeRichIntent({ object_type: "Model", ref: current.project.id }, "set_field", "pressure_profile",
      await canonicalJsonString({ schema_version: "0.9.0" }), pressureProfileTarget(current, pick), "Pressure profile");
    expect((await applyModelOperation(current, bare, hash)).diagnostics.map(d => d.code)).toContain("OP-PRESSURE-PROFILE-SHEAR-MODEL-CONFLICT");
  }
});

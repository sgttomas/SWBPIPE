// The Model settings "Straight-member shear deformation" control (DEL-05-03 Design
// pf-timoshenko.md §1, §5 "Desktop"): Euler–Bernoulli (the default) or Timoshenko
// (energy-matched annular factor). Choosing either writes `model_settings.shear_model` as an
// explicit user set (#1649's option A), Reset to default clears it, a document whose user
// never chose keeps it absent, a line states the arc and Poisson's ratio rules, and on a
// document with a pressure contract the control is disabled and points to the pressure
// panel. Each choice is checked by the wasm operation engine. Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { applyModelOperation, validateModelOperation } from "../../services/operationService";
import { computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { ModelSettingsSection, SHEAR_MODEL_EXACT_NOTE } from "./ModelSettingsSection";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const demo = () =>
  JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
const byId = (testId: string) => screen.getByTestId(testId);

async function apply(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const hash = await computeModelHash(model);
  expect((await validateModelOperation(model, intent, hash)).diagnostics).toEqual([]);
  const outcome = await applyModelOperation(model, intent, hash);
  expect(outcome.validation.application_status).toBe("applied_to_session_model");
  return outcome.applied_model as PreviewModel;
}

async function queued(list: EditorOperationIntent[], count: number): Promise<EditorOperationIntent> {
  await waitFor(() => expect(list).toHaveLength(count));
  return list[count - 1];
}

beforeAll(async () => {
  await loadWasmEngine();
});
afterEach(cleanup);

describe("model settings: straight-member shear deformation", () => {
  it("shows the default, states the rules, and offers both choices with nothing written", () => {
    const model = demo();
    render(<ModelSettingsSection model={model} onQueueIntent={() => undefined} />);
    expect(byId("model-settings-shear-model")).toHaveTextContent("Straight-member shear deformation");
    expect(byId("model-settings-shear-model-effective")).toHaveTextContent("Effective: Euler–Bernoulli (default)");
    const note = byId("model-settings-shear-model-note").textContent ?? "";
    expect(note).toContain("Realized arcs stay Euler–Bernoulli");
    expect(note).toContain("Poisson's ratio E/(2G) − 1");
    expect(byId("model-settings-shear-model-euler_bernoulli")).not.toBeChecked();
    expect(byId("model-settings-shear-model-euler_bernoulli")).not.toBeDisabled();
    expect(byId("model-settings-shear-model-timoshenko_annulus_energy_v1")).not.toBeDisabled();
    expect(screen.getByLabelText("Timoshenko (energy-matched annular factor)")).toBeInTheDocument();
    expect(screen.getByLabelText("Euler–Bernoulli (the default)")).toBeInTheDocument();
    expect(byId("model-settings-shear-model-reset")).toHaveAttribute("title", "Euler–Bernoulli already applies by default.");
    expect(screen.queryByTestId("model-settings-shear-model-exact")).toBeNull();
    expect(model.model_settings).toBeUndefined();
  });

  it("an explicit Euler–Bernoulli choice writes the member and Reset to default clears it", async () => {
    const original = demo();
    let model = demo();
    const list: EditorOperationIntent[] = [];
    const view = render(<ModelSettingsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.click(byId("model-settings-shear-model-euler_bernoulli"));
    const set = await queued(list, 1);
    expect(set.target).toEqual({ object_type: "Project", ref: original.project.id });
    expect(set.change).toMatchObject({
      change_kind: "set_model_setting", field_path: "model_settings.shear_model",
      before: "not_present", after: "euler_bernoulli", unit: "none", dimension: "dimensionless"
    });
    expect(byId("model-settings-shear-model-notice")).toHaveTextContent("queued for Review/Apply");
    model = await apply(model, set);
    expect(model.model_settings).toEqual({ shear_model: "euler_bernoulli" });

    view.rerender(<ModelSettingsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    expect(byId("model-settings-shear-model-effective")).toHaveTextContent("Effective: Euler–Bernoulli");
    expect(byId("model-settings-shear-model-euler_bernoulli")).toBeChecked();
    expect(byId("model-settings-shear-model-euler_bernoulli")).toHaveAttribute("title", "This choice is already set.");
    fireEvent.click(byId("model-settings-shear-model-reset"));
    const reset = await queued(list, 2);
    expect(reset.change).toMatchObject({ change_kind: "clear_model_setting", field_path: "model_settings.shear_model", before: "\"euler_bernoulli\"", after: "not_present" });
    model = await apply(model, reset);
    expect(model).not.toHaveProperty("model_settings");
    expect(await computeModelHash(model)).toEqual(await computeModelHash(original));
  });

  it("the Timoshenko choice sends timoshenko_annulus_energy_v1 to the operation engine", async () => {
    const model = demo();
    const list: EditorOperationIntent[] = [];
    render(<ModelSettingsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.click(byId("model-settings-shear-model-timoshenko_annulus_energy_v1"));
    // The engine judges the choice. Until the product half removes 0.8.0's interim refusal
    // (pf-timoshenko.md §1) it refuses this exact value by name and nothing is queued; once it
    // is removed, the explicit set is queued. Either outcome shows the value the control sent.
    await waitFor(() => expect(list.length > 0 || screen.queryByTestId("model-settings-shear-model-error") !== null).toBe(true));
    if (list.length > 0) {
      expect(list[0].change).toMatchObject({ change_kind: "set_model_setting", field_path: "model_settings.shear_model", before: "not_present", after: "timoshenko_annulus_energy_v1" });
    } else {
      expect(byId("model-settings-shear-model-diagnostics")).toHaveTextContent("MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED");
      expect(byId("model-settings-shear-model-error")).toHaveTextContent("was refused by the operation engine; nothing was queued");
    }
    // A Timoshenko document shows its choice.
    const selected = { ...demo(), model_settings: { shear_model: "timoshenko_annulus_energy_v1" } } as PreviewModel;
    cleanup();
    render(<ModelSettingsSection model={selected} onQueueIntent={() => undefined} />);
    expect(byId("model-settings-shear-model-effective")).toHaveTextContent("Effective: Timoshenko (energy-matched annular factor)");
    expect(byId("model-settings-shear-model-timoshenko_annulus_energy_v1")).toBeChecked();
    expect(byId("model-settings-shear-model-reset")).not.toBeDisabled();
  });

  it("is disabled on a document with a pressure contract and points to the pressure panel", () => {
    const exact = { ...demo(), pressure_contract: { version: "3.0.0", mode: "exact_pressure_v3", shear_model: "timoshenko_annulus_energy_v1" } } as PreviewModel;
    const list: EditorOperationIntent[] = [];
    render(<ModelSettingsSection model={exact} onQueueIntent={(intent) => list.push(intent)} />);
    expect(byId("model-settings-shear-model-exact")).toHaveTextContent(SHEAR_MODEL_EXACT_NOTE);
    expect(SHEAR_MODEL_EXACT_NOTE).toContain("Pressure panel");
    for (const id of ["model-settings-shear-model-euler_bernoulli", "model-settings-shear-model-timoshenko_annulus_energy_v1", "model-settings-shear-model-reset"]) {
      expect(byId(id)).toBeDisabled();
      expect(byId(id)).toHaveAttribute("title", SHEAR_MODEL_EXACT_NOTE);
    }
    fireEvent.click(byId("model-settings-shear-model-timoshenko_annulus_energy_v1"));
    expect(list).toEqual([]);
  });

  it("refuses a second pending change for the setting, independently of the tolerance", async () => {
    const model = demo();
    const list: EditorOperationIntent[] = [];
    const view = render(<ModelSettingsSection model={model} queuedIntents={list} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.click(byId("model-settings-shear-model-euler_bernoulli"));
    await queued(list, 1);
    view.rerender(<ModelSettingsSection model={model} queuedIntents={[...list]} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.click(byId("model-settings-shear-model-timoshenko_annulus_energy_v1"));
    expect(await screen.findByTestId("model-settings-shear-model-error")).toHaveTextContent(
      "Review or remove the pending shear deformation change before queuing another."
    );
    expect(screen.queryByTestId("model-settings-node-proximity-error")).toBeNull();
    // The tolerance still queues beside it.
    fireEvent.change(byId("model-settings-node-proximity-value"), { target: { value: "3" } });
    fireEvent.click(byId("model-settings-node-proximity-queue"));
    await queued(list, 2);
    expect(list[1].change.field_path).toBe("model_settings.node_proximity_tolerance");
  });
});

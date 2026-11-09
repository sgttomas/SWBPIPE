// The Model settings "Pressure elongation (Bourdon)" control (DEL-04-07 Design
// pressure-elongation.md §3.1, §3.3): one switch, "Include pressure elongation", shown only on
// a document with a pressure contract; switched on, a checklist "Operating load cases"
// preselected with the cases whose kind is "operating" and editable; queueing "on" needs at
// least one ticked case; switching off queues clear_model_setting. A contextual note while
// the member is absent; a "Not included" line naming pressurised cases the setting leaves
// out. Each change is checked by the wasm operation engine. Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { applyModelOperation, validateModelOperation } from "../../services/operationService";
import { computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { ModelSettingsSection, PRESSURE_ELONGATION_NONE_TICKED, PRESSURE_ELONGATION_OFF_NOTE } from "./ModelSettingsSection";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const demo = () =>
  JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
const byId = (testId: string) => screen.getByTestId(testId);
const SWITCH = "model-settings-pressure-elongation-switch";
const caseBox = (id: string) => byId(`model-settings-pressure-elongation-case-${id}`);

/** The demo with a pressure contract; L-100 is an operating case, L-200 a sustained one. */
function pressureDocument(): PreviewModel {
  const model = demo();
  model.pressure_contract = { version: "3.0.0", mode: "exact_pressure_v3", shear_model: "euler_bernoulli" } as PreviewModel["pressure_contract"];
  model.load_cases[0].kind = " Operating ";
  model.load_cases[1].kind = "sustained";
  return model;
}

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

describe("model settings: pressure elongation", () => {
  it("is not offered on a document without a pressure contract", () => {
    render(<ModelSettingsSection model={demo()} onQueueIntent={() => undefined} />);
    expect(screen.queryByTestId("model-settings-pressure-elongation")).toBeNull();
    expect(screen.queryByRole("switch", { name: "Include pressure elongation" })).toBeNull();
    // The other sections keep their test ids.
    expect(byId("model-settings-node-proximity")).toBeInTheDocument();
    expect(byId("model-settings-shear-model")).toBeInTheDocument();
  });

  it("is off by default with the contextual note, and preselects the operating cases when switched on", () => {
    const model = pressureDocument();
    render(<ModelSettingsSection model={model} onQueueIntent={() => undefined} />);
    expect(byId("model-settings-pressure-elongation")).toHaveTextContent("Pressure elongation (Bourdon)");
    const toggle = screen.getByRole("switch", { name: "Include pressure elongation" });
    expect(toggle).not.toBeChecked();
    expect(byId("model-settings-pressure-elongation-off-note").textContent).toBe(
      "Off (default). Pressure elongation is not included in any load case. Documents saved before this setting existed included it in every pressurised case; turn it on and name the operating cases to include it there."
    );
    expect(PRESSURE_ELONGATION_OFF_NOTE).toBe(byId("model-settings-pressure-elongation-off-note").textContent);
    expect(screen.queryByTestId("model-settings-pressure-elongation-cases")).toBeNull();

    fireEvent.click(toggle);
    expect(toggle).toBeChecked();
    expect(byId("model-settings-pressure-elongation-cases")).toHaveTextContent("Operating load cases");
    expect(caseBox("load:L-100")).toBeChecked();
    expect(caseBox("load:L-200")).not.toBeChecked();
    expect(byId("model-settings-pressure-elongation-queue")).not.toBeDisabled();
    // Editable: untick the preselected case, and queueing "on" is refused with the reason.
    fireEvent.click(caseBox("load:L-100"));
    expect(caseBox("load:L-100")).not.toBeChecked();
    expect(byId("model-settings-pressure-elongation-queue")).toBeDisabled();
    expect(byId("model-settings-pressure-elongation-queue")).toHaveAttribute("title", PRESSURE_ELONGATION_NONE_TICKED);
    expect(byId("model-settings-pressure-elongation-none-ticked")).toHaveTextContent(PRESSURE_ELONGATION_NONE_TICKED);
    fireEvent.click(caseBox("load:L-200"));
    expect(byId("model-settings-pressure-elongation-queue")).not.toBeDisabled();
    // Switching an unqueued draft off queues nothing and restores the preselection.
    fireEvent.click(toggle);
    expect(screen.queryByTestId("model-settings-pressure-elongation-cases")).toBeNull();
    fireEvent.click(toggle);
    expect(caseBox("load:L-100")).toBeChecked();
    expect(caseBox("load:L-200")).not.toBeChecked();
  });

  it("nothing is preselected when no case's kind is operating", () => {
    const model = pressureDocument();
    model.load_cases[0].kind = "primitive_user_load";
    render(<ModelSettingsSection model={model} onQueueIntent={() => undefined} />);
    fireEvent.click(screen.getByRole("switch", { name: "Include pressure elongation" }));
    expect(caseBox("load:L-100")).not.toBeChecked();
    expect(caseBox("load:L-200")).not.toBeChecked();
    expect(byId("model-settings-pressure-elongation-queue")).toBeDisabled();
  });

  it("queues set and clear with their exact payloads, and the applied record round-trips", async () => {
    const original = pressureDocument();
    let model = pressureDocument();
    const list: EditorOperationIntent[] = [];
    const view = render(<ModelSettingsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.click(screen.getByRole("switch", { name: "Include pressure elongation" }));
    fireEvent.click(byId("model-settings-pressure-elongation-queue"));
    const set = await queued(list, 1);
    expect(set.target).toEqual({ object_type: "Project", ref: original.project.id });
    expect(set.change).toMatchObject({
      change_kind: "set_model_setting",
      field_path: "model_settings.pressure_elongation",
      before: "not_present",
      after: "{\"operating_load_cases\":[\"load:L-100\"]}",
      unit: "none",
      dimension: "dimensionless"
    });
    expect(byId("model-settings-pressure-elongation-notice")).toHaveTextContent("queued for Review/Apply");
    model = await apply(model, set);
    expect(model.model_settings).toEqual({ pressure_elongation: { operating_load_cases: ["load:L-100"] } });

    view.rerender(<ModelSettingsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    expect(screen.getByRole("switch", { name: "Include pressure elongation" })).toBeChecked();
    expect(screen.queryByTestId("model-settings-pressure-elongation-off-note")).toBeNull();
    expect(caseBox("load:L-100")).toBeChecked();
    expect(byId("model-settings-pressure-elongation-queue")).toHaveAttribute("title", "These operating load cases are already set.");

    fireEvent.click(screen.getByRole("switch", { name: "Include pressure elongation" }));
    const clear = await queued(list, 2);
    expect(clear.change).toMatchObject({
      change_kind: "clear_model_setting",
      field_path: "model_settings.pressure_elongation",
      before: "{\"operating_load_cases\":[\"load:L-100\"]}",
      after: "not_present",
      unit: "none",
      dimension: "dimensionless"
    });
    model = await apply(model, clear);
    expect(model.model_settings).toBeUndefined();
    expect(model).toEqual(original);
  });

  it("names every pressurised case the setting does not include", () => {
    const model = pressureDocument();
    const region = { id: "region:r" } as unknown as NonNullable<PreviewModel["load_cases"][number]["pressure_regions"]>[number];
    model.load_cases[0].pressure_regions = [region];
    model.load_cases[1].pressure_regions = [region];
    model.load_cases.push({ id: "load:L-300", label: "", kind: "operating", status: "preview_only", provenance: "invented", pressure_regions: [region] });
    // Off: no line (the contextual note speaks instead).
    const view = render(<ModelSettingsSection model={model} onQueueIntent={() => undefined} />);
    expect(screen.queryByTestId("model-settings-pressure-elongation-not-included")).toBeNull();
    const on = { ...model, model_settings: { pressure_elongation: { operating_load_cases: ["load:L-100"] } } };
    view.rerender(<ModelSettingsSection model={on} onQueueIntent={() => undefined} />);
    expect(byId("model-settings-pressure-elongation-not-included").textContent).toBe(
      "Not included: Invented alternate gravity and occasional preview (load:L-200), load:L-300"
    );
  });

  it("a refusal by the engine stays here by name and nothing is queued", async () => {
    const model = pressureDocument();
    // The setting names a case another session removed: the engine refuses the stale document.
    const list: EditorOperationIntent[] = [];
    render(<ModelSettingsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.click(screen.getByRole("switch", { name: "Include pressure elongation" }));
    fireEvent.click(caseBox("load:L-200"));
    model.load_cases.splice(1, 1);
    fireEvent.click(byId("model-settings-pressure-elongation-queue"));
    await waitFor(() => expect(byId("model-settings-pressure-elongation-error")).toBeInTheDocument());
    expect(screen.getByTestId("model-settings-pressure-elongation-diagnostic-MODEL-SETTING-PRESSURE-ELONGATION-CASE-UNKNOWN")).toHaveTextContent(
      "names `load:L-200`, which is not a load case of this document."
    );
    expect(list).toHaveLength(0);
  });
});

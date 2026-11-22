// The Model settings "Shear deformation in straight members" control (DEL-05-03 Design
// pf-timoshenko.md §1, §5 "Desktop"): "Included (Timoshenko)" or "Not included
// (Euler–Bernoulli)". Choosing either writes `model_settings.shear_model` as an explicit
// user set (#1649's option A); there is no reset. A document that states no choice reads as
// Euler–Bernoulli and says so, nothing is written for it, a line states the factor, arc and
// Poisson's ratio rules. On a document with a pressure contract only one line speaks: the
// contract's theory and where it is set, with no choice, Effective line or note. Each choice
// is checked by the wasm operation engine. Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { applyModelOperation, validateModelOperation } from "../../services/operationService";
import { computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { ModelSettingsSection, SHEAR_MODEL_EXACT_NOTE, shearModelExactLine } from "./ModelSettingsSection";

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

describe("model settings: shear deformation in straight members", () => {
  it("reads a document that states no choice as Euler–Bernoulli, states the rules, and offers both choices with nothing written", () => {
    const model = demo();
    render(<ModelSettingsSection model={model} onQueueIntent={() => undefined} />);
    expect(byId("model-settings-shear-model")).toHaveTextContent("Shear deformation in straight members");
    expect(byId("model-settings-shear-model-effective")).toHaveTextContent(
      "Effective: Not included (Euler–Bernoulli). This document does not state a choice.");
    expect(byId("model-settings-shear-model")).not.toHaveTextContent("default");
    expect(byId("model-settings-shear-model-note").textContent).toBe(
      "Applies to every straight member, tee leg, valve, flange or other rigid component, and geometry-only bend; realized arcs stay Euler–Bernoulli. " +
      "The shear factor is the energy-matched one for an annular section, and Poisson's ratio is E/(2G) − 1 from each case's moduli, not an entered value.");
    expect(byId("model-settings-shear-model-euler_bernoulli")).not.toBeChecked();
    expect(byId("model-settings-shear-model-euler_bernoulli")).not.toBeDisabled();
    expect(byId("model-settings-shear-model-timoshenko_annulus_energy_v1")).not.toBeDisabled();
    const offered = screen.getAllByRole("radio").filter((radio) => radio.getAttribute("name") === "model-settings-shear-model");
    expect(offered.map((radio) => radio.parentElement?.textContent)).toEqual(["Included (Timoshenko)", "Not included (Euler–Bernoulli)"]);
    expect(screen.queryByTestId("model-settings-shear-model-reset")).toBeNull();
    expect(screen.queryByTestId("model-settings-shear-model-exact")).toBeNull();
    expect(model.model_settings).toBeUndefined();
  });

  it("an explicit Euler–Bernoulli choice writes the member, and is changed only by the other choice", async () => {
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
    expect(byId("model-settings-shear-model-effective").textContent).toBe("Effective: Not included (Euler–Bernoulli)");
    expect(byId("model-settings-shear-model-euler_bernoulli")).toBeChecked();
    expect(byId("model-settings-shear-model-euler_bernoulli")).toHaveAttribute("title", "This choice is already set.");
    expect(screen.queryByTestId("model-settings-shear-model-reset")).toBeNull();
    fireEvent.click(byId("model-settings-shear-model-timoshenko_annulus_energy_v1"));
    const changed = await queued(list, 2);
    expect(changed.change).toMatchObject({ change_kind: "set_model_setting", field_path: "model_settings.shear_model", before: "\"euler_bernoulli\"", after: "timoshenko_annulus_energy_v1" });
    model = await apply(model, changed);
    expect(model.model_settings).toEqual({ shear_model: "timoshenko_annulus_energy_v1" });
    expect(original).not.toHaveProperty("model_settings");
  });

  it("the Timoshenko choice writes timoshenko_annulus_energy_v1 as an explicit set", async () => {
    let model = demo();
    const list: EditorOperationIntent[] = [];
    render(<ModelSettingsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.click(byId("model-settings-shear-model-timoshenko_annulus_energy_v1"));
    const set = await queued(list, 1);
    expect(set.change).toMatchObject({ change_kind: "set_model_setting", field_path: "model_settings.shear_model", before: "not_present", after: "timoshenko_annulus_energy_v1" });
    model = await apply(model, set);
    expect(model.model_settings).toEqual({ shear_model: "timoshenko_annulus_energy_v1" });
    // A Timoshenko document shows its choice.
    const selected = { ...demo(), model_settings: { shear_model: "timoshenko_annulus_energy_v1" } } as PreviewModel;
    cleanup();
    render(<ModelSettingsSection model={selected} onQueueIntent={() => undefined} />);
    expect(byId("model-settings-shear-model-effective").textContent).toBe("Effective: Included (Timoshenko)");
    expect(byId("model-settings-shear-model-timoshenko_annulus_energy_v1")).toBeChecked();
  });

  it.each([
    ["timoshenko_annulus_energy_v1", "Included (Timoshenko)"],
    ["euler_bernoulli", "Not included (Euler–Bernoulli)"]
  ] as const)("on a document with a pressure contract stating %s, shows only the contract's theory", (stated, label) => {
    // Such a document holds no pressure-free setting (activation removes it); a stray one is never read here.
    const stray = stated === "euler_bernoulli" ? "timoshenko_annulus_energy_v1" : "euler_bernoulli";
    for (const settings of [undefined, { shear_model: stray }]) {
      const exact = { ...demo(), pressure_contract: { version: "3.0.0", mode: "exact_pressure_v3", shear_model: stated }, ...(settings ? { model_settings: settings } : {}) } as PreviewModel;
      const list: EditorOperationIntent[] = [];
      render(<ModelSettingsSection model={exact} onQueueIntent={(intent) => list.push(intent)} />);
      const line = byId("model-settings-shear-model-exact");
      expect(line.textContent).toBe(shearModelExactLine(stated));
      expect(line.textContent).toContain(`: ${label}.`);
      expect(line.textContent).toContain(SHEAR_MODEL_EXACT_NOTE);
      for (const id of ["model-settings-shear-model-effective", "model-settings-shear-model-note", "model-settings-shear-model-euler_bernoulli", "model-settings-shear-model-timoshenko_annulus_energy_v1"]) {
        expect(screen.queryByTestId(id)).toBeNull();
      }
      expect(screen.queryAllByRole("radio").filter((radio) => radio.getAttribute("name") === "model-settings-shear-model")).toEqual([]);
      expect(list).toEqual([]);
      cleanup();
    }
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

  it("shows neither option as chosen on a document that states no choice, so either can be stated", () => {
    // A radio that already looked chosen could not be chosen: Euler–Bernoulli, in effect here, could never be
    // stated. The Effective line says which is in effect and that the document states no choice. (Timoshenko is
    // offered first; the one in effect is never taken from the order.)
    render(<ModelSettingsSection model={demo()} onQueueIntent={() => undefined} />);
    expect(byId("model-settings-shear-model-timoshenko_annulus_energy_v1")).not.toBeChecked();
    expect(byId("model-settings-shear-model-euler_bernoulli")).not.toBeChecked();
    expect(byId("model-settings-shear-model-euler_bernoulli")).not.toBeDisabled();
    expect(byId("model-settings-shear-model-effective").textContent).toBe("Effective: Not included (Euler–Bernoulli). This document does not state a choice.");
  });
});

// The Loads page's Wind and Seismic sections through the real wasm operation engine, on the
// committed demonstration model (`fixtures/product_preview/invented_demo_model.json`): each
// section queues the inspector's `update_load` operations, checked by the engine, and the
// queued changes apply in order.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { applyModelOperation } from "../../services/operationService";
import { computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { SeismicSection, WindSection } from "./EquivalentStaticSections";
import { seismicSummary, windSummary } from "./equivalentStaticIntents";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const demo = () => JSON.parse(readFileSync(path.join(root, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
const caseOf = (model: PreviewModel) => model.load_cases.find((loadCase) => loadCase.id === "load:L-100")!;
const type = (testId: string, value: string) => fireEvent.change(screen.getByTestId(testId), { target: { value } });

async function applyAll(model: PreviewModel, intents: EditorOperationIntent[]): Promise<PreviewModel> {
  for (const intent of intents) {
    const outcome = await applyModelOperation(model, intent, await computeModelHash(model));
    expect(outcome.diagnostics.filter((item) => item.severity === "blocking")).toEqual([]);
    model = outcome.applied_model as PreviewModel;
  }
  return model;
}

beforeAll(async () => {
  await loadWasmEngine();
});
afterEach(cleanup);

describe("the Wind section", () => {
  it("enters pressure, shape factor, direction and exposed pipes as the inspector's operations", async () => {
    let model = demo();
    expect(windSummary(caseOf(model).equivalent_static)).toBe("none");
    const list: EditorOperationIntent[] = [];
    const view = render(<WindSection model={model} onQueueIntent={(intent) => list.push(intent)} loadCase={caseOf(model)} unitCatalogRoute={null} />);
    expect(screen.getByTestId("load-case-wind-pressure-unit")).toHaveValue("Pa");
    expect(screen.getByTestId("load-case-wind-queue")).toBeDisabled();
    type("load-case-wind-pressure-value", "1.2");
    type("load-case-wind-pressure-unit", "kPa");
    type("load-case-wind-shape-factor-value", "0.7");
    type("load-case-wind-direction", "global_x");
    fireEvent.click(screen.getByRole("option", { name: /P-100/ }));
    fireEvent.click(screen.getByTestId("load-case-wind-queue"));
    await waitFor(() => expect(list).toHaveLength(4));
    expect(list.map((intent) => [intent.change.change_kind, intent.change.field_path, intent.change.before, intent.change.after, intent.change.unit, intent.change.dimension])).toEqual([
      ["update_load", "equivalent_static.wind.pressure.value", "TBD", JSON.stringify({ value: 1.2, unit: "kPa" }), "kPa", "pressure"],
      ["update_load", "equivalent_static.wind.shape_factor.value", "TBD", "0.7", "none", "dimensionless"],
      ["update_load", "equivalent_static.wind.direction", "TBD", "global_x", "none", "dimensionless"],
      ["update_load", "equivalent_static.wind.exposed_pipe_refs", "TBD", "pipe:P-100", "none", "dimensionless"]
    ]);
    model = await applyAll(model, list);
    expect(caseOf(model).equivalent_static?.wind).toMatchObject({ pressure: { value: 1.2, unit: "kPa" }, shape_factor: { value: 0.7, unit: "none" }, direction: "global_x", exposed_pipe_refs: ["pipe:P-100"] });
    expect(windSummary(caseOf(model).equivalent_static)).toBe("1.2 kPa, +X, 1 pipe");
    view.unmount();

    // An edit of a stored value names the stored value as its before.
    list.length = 0;
    render(<WindSection model={model} onQueueIntent={(intent) => list.push(intent)} loadCase={caseOf(model)} unitCatalogRoute={null} />);
    type("load-case-wind-pressure-value", "1.5");
    fireEvent.click(screen.getByTestId("load-case-wind-queue"));
    await waitFor(() => expect(list).toHaveLength(1));
    expect(list[0].change).toMatchObject({ before: "1.2", after: JSON.stringify({ value: 1.5, unit: "kPa" }) });
  });
});

describe("the Seismic section", () => {
  it("enters gravity and the three factors, and shows the engine's refusal by code", async () => {
    let model = demo();
    expect(seismicSummary(caseOf(model).equivalent_static)).toBe("none");
    const list: EditorOperationIntent[] = [];
    const view = render(<SeismicSection model={model} onQueueIntent={(intent) => list.push(intent)} loadCase={caseOf(model)} unitCatalogRoute={null} />);
    expect(screen.getByTestId("load-case-seismic-x-unit")).toHaveTextContent("g");
    type("load-case-seismic-gravity-value", "-9.81");
    type("load-case-seismic-gravity-unit", "m/s^2");
    fireEvent.click(screen.getByTestId("load-case-seismic-queue"));
    expect(await screen.findByTestId("load-case-seismic-finding-OP-VALUE-NOT-POSITIVE")).toHaveAttribute("data-severity", "blocking");
    expect(list).toEqual([]);

    type("load-case-seismic-gravity-value", "9.81");
    type("load-case-seismic-x-value", "0.3");
    type("load-case-seismic-y-value", "0.3");
    type("load-case-seismic-z-value", "0.2");
    fireEvent.click(screen.getByTestId("load-case-seismic-queue"));
    await waitFor(() => expect(list).toHaveLength(4));
    expect(list.map((intent) => [intent.change.field_path, intent.change.after, intent.change.unit, intent.change.dimension])).toEqual([
      ["equivalent_static.seismic.gravity_acceleration.value", JSON.stringify({ value: 9.81, unit: "m/s^2" }), "m/s^2", "acceleration"],
      ["equivalent_static.seismic.g_factor_x.value", "0.3", "none", "dimensionless"],
      ["equivalent_static.seismic.g_factor_y.value", "0.3", "none", "dimensionless"],
      ["equivalent_static.seismic.g_factor_z.value", "0.2", "none", "dimensionless"]
    ]);
    model = await applyAll(model, list);
    expect(seismicSummary(caseOf(model).equivalent_static)).toBe("0.3 g X, 0.3 g Y, 0.2 g Z");
    view.unmount();
    render(<SeismicSection model={model} onQueueIntent={() => undefined} loadCase={caseOf(model)} unitCatalogRoute={null} />);
    expect(screen.getByTestId("load-case-seismic-gravity-value")).toHaveValue("9.81");
    expect(screen.getByTestId("load-case-seismic-queue")).toBeDisabled();
  });
});

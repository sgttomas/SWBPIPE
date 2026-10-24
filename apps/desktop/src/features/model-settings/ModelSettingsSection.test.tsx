// The Project page's node-proximity tolerance control (DEL-02-01 Design §3.8;
// DEL-04-06 Design model-topology-checks.md). Owner ruling, 2026-10-23: "I
// choose C, but my value is "5mm" and give the user the means to change it."
// Each queued edit is checked and applied here through the wasm applier, the
// browser-mode engine (the native solve that reads the setting is covered in
// src-tauri and the applier's through-solve test). Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { applyModelOperation, validateModelOperation } from "../../services/operationService";
import { computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { DisplayUnitsProvider } from "../display-units";
import { PropertyInspector } from "../model-tree/PropertyInspector";
import { ModelSettingsSection, toleranceUnitChoices } from "./ModelSettingsSection";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const demo = () =>
  JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;

async function apply(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const hash = await computeModelHash(model);
  const preview = await validateModelOperation(model, intent, hash);
  expect(preview.diagnostics).toEqual([]);
  expect(preview.diff_preview).toHaveLength(1);
  const outcome = await applyModelOperation(model, intent, hash);
  expect(outcome.diagnostics).toEqual([]);
  expect(outcome.validation.application_status).toBe("applied_to_session_model");
  return outcome.applied_model as PreviewModel;
}

async function queued(list: EditorOperationIntent[], count: number): Promise<EditorOperationIntent> {
  await waitFor(() => expect(list).toHaveLength(count));
  return list[count - 1];
}

const byId = (testId: string) => screen.getByTestId(testId);

beforeAll(async () => {
  await loadWasmEngine();
});
afterEach(cleanup);

describe("model settings: node-proximity tolerance", () => {
  it("shows the default, sets a value with its unit, and resets to the default", async () => {
    const original = demo();
    expect(original.model_settings).toBeUndefined();
    let model = demo();
    const list: EditorOperationIntent[] = [];
    const view = render(<ModelSettingsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    expect(byId("model-settings-node-proximity-effective")).toHaveTextContent("Effective: 5 mm (default)");
    expect(byId("model-settings-node-proximity-value")).toHaveValue("");
    expect(byId("model-settings-node-proximity-unit")).toHaveValue("m");
    expect(byId("model-settings-node-proximity-reset")).toBeDisabled();
    expect(byId("model-settings-node-proximity-reset")).toHaveAttribute("title", "The 5 mm default already applies.");
    expect(byId("model-settings-node-proximity-queue")).toBeDisabled();

    fireEvent.change(byId("model-settings-node-proximity-value"), { target: { value: "2" } });
    fireEvent.change(byId("model-settings-node-proximity-unit"), { target: { value: "mm" } });
    fireEvent.click(byId("model-settings-node-proximity-queue"));
    const set = await queued(list, 1);
    expect(set.target).toEqual({ object_type: "Project", ref: original.project.id });
    expect(set.operation_kind).toBe("modify");
    expect(set.change).toMatchObject({
      change_kind: "set_model_setting",
      field_path: "model_settings.node_proximity_tolerance",
      before: "not_present",
      after: "2",
      unit: "mm",
      dimension: "length"
    });
    expect(byId("model-settings-node-proximity-notice")).toHaveTextContent("queued for Review/Apply");
    model = await apply(model, set);
    expect(model.model_settings).toEqual({ node_proximity_tolerance: { value: 2, unit: "mm" } });

    view.rerender(<ModelSettingsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    expect(byId("model-settings-node-proximity-effective")).toHaveTextContent("Effective: 2 mm");
    expect(byId("model-settings-node-proximity-value")).toHaveValue("2");
    expect(byId("model-settings-node-proximity-unit")).toHaveValue("mm");
    expect(byId("model-settings-node-proximity-queue")).toHaveAttribute("title", "The tolerance is unchanged.");
    fireEvent.click(byId("model-settings-node-proximity-reset"));
    const reset = await queued(list, 2);
    expect(reset.change).toMatchObject({
      change_kind: "clear_model_setting",
      before: JSON.stringify({ unit: "mm", value: 2 }),
      after: "not_present",
      unit: "none",
      dimension: "dimensionless"
    });
    model = await apply(model, reset);
    expect(model).not.toHaveProperty("model_settings");
    // The applier returns canonical key order; the content and model hash are the original's.
    expect(model).toEqual(original);
    expect(await computeModelHash(model)).toEqual(await computeModelHash(original));
    view.rerender(<ModelSettingsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    expect(byId("model-settings-node-proximity-effective")).toHaveTextContent("Effective: 5 mm (default)");
    expect(byId("model-settings-node-proximity-value")).toHaveValue("");
  });

  it("shows a refusal by name and queues nothing", async () => {
    const model = demo();
    const list: EditorOperationIntent[] = [];
    render(<ModelSettingsSection model={model} onQueueIntent={(intent) => list.push(intent)} />);
    for (const [entered, code, message] of [
      ["0", "MODEL-SETTING-TOLERANCE-NOT-POSITIVE", "model_settings.node_proximity_tolerance.value is 0; the tolerance must be greater than zero."],
      ["-3", "MODEL-SETTING-TOLERANCE-NOT-POSITIVE", "model_settings.node_proximity_tolerance.value is -3; the tolerance must be greater than zero."],
      ["inf", "MODEL-SETTING-TOLERANCE-NOT-FINITE", "model_settings.node_proximity_tolerance.value is inf; a finite length greater than zero is required."],
      ["five", "OP-VALUE-NOT-NUMERIC", "Replacement value `five` for `model_settings.node_proximity_tolerance` is not a number."]
    ]) {
      fireEvent.change(byId("model-settings-node-proximity-value"), { target: { value: entered } });
      fireEvent.click(byId("model-settings-node-proximity-queue"));
      const item = await screen.findByTestId(`model-settings-node-proximity-diagnostic-${code}`);
      expect(item).toHaveTextContent(`${code}: ${message}`);
      expect(byId("model-settings-node-proximity-error")).toHaveTextContent("was refused by the operation engine; nothing was queued");
      await waitFor(() => expect(byId("model-settings-node-proximity-queue")).not.toBeDisabled());
    }
    expect(list).toEqual([]);
  });

  it("offers the catalog's length units, and the project's length unit first without the catalog", () => {
    const model = demo();
    expect(toleranceUnitChoices(model, null)).toEqual(["m", "mm", "in"]);
    expect(toleranceUnitChoices(model, catalogRoute(["m", "mm", "in", "ft"]))).toEqual(["m", "mm", "in", "ft"]);
    // A catalog unit of another dimension is never offered.
    expect(toleranceUnitChoices(model, catalogRoute(["m", "mm", "N"]))).toEqual(["m", "mm"]);
    model.model_settings = { node_proximity_tolerance: { value: 0.25, unit: "in" } };
    expect(toleranceUnitChoices(model, null)).toEqual(["in", "m", "mm"]);
    render(<ModelSettingsSection model={model} onQueueIntent={() => undefined} />);
    expect(byId("model-settings-node-proximity-effective")).toHaveTextContent("Effective: 0.25 in");
    expect(byId("model-settings-node-proximity-unit")).toHaveValue("in");
  });

  it("refuses a second pending change for the setting", async () => {
    const model = demo();
    const list: EditorOperationIntent[] = [];
    const view = render(<ModelSettingsSection model={model} queuedIntents={list} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.change(byId("model-settings-node-proximity-value"), { target: { value: "3" } });
    fireEvent.click(byId("model-settings-node-proximity-queue"));
    await queued(list, 1);
    view.rerender(<ModelSettingsSection model={model} queuedIntents={[...list]} onQueueIntent={(intent) => list.push(intent)} />);
    fireEvent.change(byId("model-settings-node-proximity-value"), { target: { value: "4" } });
    fireEvent.click(byId("model-settings-node-proximity-queue"));
    expect(await screen.findByTestId("model-settings-node-proximity-error")).toHaveTextContent(
      "Review or remove the pending node-proximity tolerance change before queuing another."
    );
    expect(list).toHaveLength(1);
  });

  it("is on the Project page of the property inspector", () => {
    const model = demo();
    const view = render(
      <DisplayUnitsProvider initialPreference="SI">
        <PropertyInspector model={model} selection={{ type: "project", id: model.project.id }} onQueueIntent={() => undefined} />
      </DisplayUnitsProvider>
    );
    expect(within(byId("model-settings-details")).getByText("Model settings")).toBeInTheDocument();
    expect(byId("model-settings-details")).toHaveAttribute("open");
    expect(byId("model-settings-node-proximity-effective")).toHaveTextContent("5 mm (default)");
    view.rerender(
      <DisplayUnitsProvider initialPreference="SI">
        <PropertyInspector model={model} selection={{ type: "node", id: model.nodes[0].id }} onQueueIntent={() => undefined} />
      </DisplayUnitsProvider>
    );
    expect(screen.queryByTestId("model-settings-details")).toBeNull();
  });
});

/** A loaded catalog route listing `symbols` (length entries for m, mm, in, ft; force for N). */
function catalogRoute(symbols: string[]): UnitCatalogRoute {
  const dimension = (symbol: string) => (symbol === "N" ? "force" : "length");
  return {
    route: "tauri_unit_catalog",
    catalog: {
      entries: symbols.map((symbol, index) => ({
        unit_id: `unit:${symbol}`,
        symbol,
        dimension_id: dimension(symbol),
        canonical: index === 0,
        review_status: "accepted"
      }))
    }
  } as unknown as UnitCatalogRoute;
}

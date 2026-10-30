// W-17 slice (c) (DEL-02-02 Design `project-unit-selection.md` §7.2, §7.3, §7.6):
// the load case's solve temperature takes project.units.temperature, the seismic
// gravity takes the composition <length>/s^2, and the component modifiers take
// the §2.4 compositions. A missing key (absent, empty or whitespace-only) or a
// composition the catalog lacks gives no unit and is refused by name; a unit the
// user picks in the inspector lifts it; the grid refuses and points to the
// inspector. Stored units are kept. Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { changeFormControl, expectFormControlValue } from "../../test-support/workspaceTestControls";
import { DisplayUnitsProvider } from "../display-units";
import { ModelTree } from "./ModelTree";
import { PropertyInspector } from "./PropertyInspector";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null, delayed: null as Promise<UnitCatalogRoute | null> | null }));
vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: () => route.delayed ?? Promise.resolve(route.current)
}));
const CATALOG: UnitCatalogRoute = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
const PREVIEW: UnitCatalogRoute = { route: "unavailable_browser_preview", diagnostic: "UNIT-CATALOG-DESKTOP-ONLY: fixture" };

afterEach(() => { cleanup(); route.current = null; route.delayed = null; });

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
/** The bundled invented demo, with its valve's stored stiffness units removed so the defaults show. */
function model(units: Record<string, string | undefined> = {}): PreviewModel {
  const document = JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
  const merged: Record<string, string> = { ...document.project.units };
  for (const [key, value] of Object.entries(units)) {
    if (value === undefined) delete merged[key];
    else merged[key] = value;
  }
  document.project.units = merged as PreviewModel["project"]["units"];
  const modifiers = document.components.find((component) => component.id === "component:C-130")!.modifiers as Record<string, unknown>;
  delete modifiers.linear_stiffness_user_value;
  delete modifiers.rotational_stiffness_user_value;
  return document;
}

const byId = (testId: string) => screen.getByTestId(testId);
const NOTICE = "editor-intent-missing-project-unit";
const LOAD: EntityRef = { type: "load", id: "load:L-100" };
const VALVE: EntityRef = { type: "component", id: "component:C-130" };
const TEMPERATURE = "modulus_basis_temperature.value";
const GRAVITY = "equivalent_static.seismic.gravity_acceleration.value";

function mount(document: PreviewModel, selection: EntityRef) {
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  const view = render(
    <DisplayUnitsProvider initialPreference="SI">
      <PropertyInspector model={document} selection={selection} onQueueIntent={queue} onValidateIntent={vi.fn()} onApplyIntent={vi.fn(async () => true)} />
    </DisplayUnitsProvider>
  );
  return { queue, view };
}

async function editField(document: PreviewModel, selection: EntityRef, fieldPath: string, catalog: UnitCatalogRoute = CATALOG) {
  route.current = catalog;
  const mounted = mount(document, selection);
  await waitFor(() => expect(byId("property-unit-catalog-status")).not.toHaveTextContent("loading"));
  fireEvent.click(screen.getByRole("tab", { name: "Task" }));
  fireEvent.click(byId("inspector-start-task"));
  changeFormControl(byId("editor-intent-field"), { target: { value: fieldPath } });
  return mounted;
}

function queueValue(value: string) {
  fireEvent.change(byId("editor-intent-value"), { target: { value } });
  fireEvent.click(byId("queue-editor-intent"));
}

describe("solve temperature takes project.units.temperature (§7.2)", () => {
  it("defaults to the key (degC in the demo), not K, and queues it", async () => {
    const { queue } = await editField(model(), LOAD, TEMPERATURE);
    expect(screen.queryByTestId(NOTICE)).toBeNull();
    expectFormControlValue(byId("editor-intent-unit"), "degC");
    queueValue("400");
    expect(JSON.parse(queue.mock.calls[0][0].change.after)).toEqual({ value: 400, unit: "degC" });
  });

  it("reads a legacy C as degC", async () => {
    await editField(model({ temperature: "C" }), LOAD, TEMPERATURE);
    expectFormControlValue(byId("editor-intent-unit"), "degC");
  });

  it.each([["absent", undefined], ["empty", ""], ["whitespace-only", "  "]])("key %s: no unit, the key named, nothing queued until a unit is picked", async (_label, value) => {
    const { queue } = await editField(model({ temperature: value }), LOAD, TEMPERATURE);
    expectFormControlValue(byId("editor-intent-unit"), "");
    expect(byId(NOTICE)).toHaveTextContent("Missing project unit: temperature. This field has no default unit, and nothing is assumed.");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "400" } });
    expect(byId("queue-editor-intent")).toBeDisabled();
    changeFormControl(byId("editor-intent-unit"), { target: { value: "K" } });
    expect(screen.queryByTestId(NOTICE)).toBeNull();
    fireEvent.click(byId("queue-editor-intent"));
    expect(JSON.parse(queue.mock.calls[0][0].change.after)).toEqual({ value: 400, unit: "K" });
  });

  it("a stored quantity keeps its unit; a key change moves only a new entry's starting unit", async () => {
    // A stored K quantity reads K, whatever the key says.
    const stored = model();
    stored.load_cases[0].modulus_basis_temperature = { value: 400, unit: "K" };
    await editField(stored, LOAD, TEMPERATURE);
    expectFormControlValue(byId("editor-intent-unit"), "K");
    cleanup();

    // An operation captured under degC keeps degC after the key changes to degF;
    // a new entry then starts at degF.
    const before = model();
    const { queue, view } = await editField(before, LOAD, TEMPERATURE);
    queueValue("400");
    const captured = queue.mock.calls[0][0];
    const after = model({ temperature: "degF" });
    view.rerender(
      <DisplayUnitsProvider initialPreference="SI">
        <PropertyInspector model={after} selection={LOAD} onQueueIntent={queue} onValidateIntent={vi.fn()} onApplyIntent={vi.fn(async () => true)} />
      </DisplayUnitsProvider>
    );
    expect(JSON.parse(captured.change.after)).toEqual({ value: 400, unit: "degC" });
    expect(captured.change.unit).toBe("degC");
    changeFormControl(byId("editor-intent-field"), { target: { value: GRAVITY } });
    changeFormControl(byId("editor-intent-field"), { target: { value: TEMPERATURE } });
    expectFormControlValue(byId("editor-intent-unit"), "degF");
  });
});

describe("seismic gravity takes <length>/s^2 (§7.3)", () => {
  it("with length m, defaults to m/s^2 and queues it", async () => {
    const { queue } = await editField(model(), LOAD, GRAVITY);
    expectFormControlValue(byId("editor-intent-unit"), "m/s^2");
    queueValue("9.80665");
    expect(JSON.parse(queue.mock.calls[0][0].change.after)).toEqual({ value: 9.80665, unit: "m/s^2" });
  });

  it.each(["mm", "in", "ft"])("with length %s, starts unselected with the notice naming the composed symbol until a unit is picked", async (length) => {
    const { queue } = await editField(model({ length }), LOAD, GRAVITY);
    expectFormControlValue(byId("editor-intent-unit"), "");
    expect(byId(NOTICE)).toHaveTextContent(`${length}/s^2 is not a catalog unit; choose one.`);
    fireEvent.change(byId("editor-intent-value"), { target: { value: "9.8" } });
    expect(byId("queue-editor-intent")).toBeDisabled();
    changeFormControl(byId("editor-intent-unit"), { target: { value: "m/s^2" } });
    expect(screen.queryByTestId(NOTICE)).toBeNull();
    fireEvent.click(byId("queue-editor-intent"));
    expect(queue.mock.calls[0][0].change.unit).toBe("m/s^2");
  });

  it("with length absent, names length; empty or whitespace-only, says the key is empty", async () => {
    await editField(model({ length: undefined }), LOAD, GRAVITY);
    expect(byId(NOTICE)).toHaveAttribute("data-key", "length");
    expect(byId(NOTICE)).toHaveTextContent("Missing project unit: length. This field has no default unit, and nothing is assumed.");
    cleanup();
    for (const blank of ["", "  "]) {
      await editField(model({ length: blank }), LOAD, GRAVITY);
      expectFormControlValue(byId("editor-intent-unit"), "");
      expect(byId(NOTICE)).toHaveTextContent("project.units.length is empty; choose one.");
      cleanup();
    }
  });

  it("without the catalog (browser preview), the composition is kept unchecked", async () => {
    await editField(model({ length: "in" }), LOAD, GRAVITY, PREVIEW);
    expect(byId("editor-intent-value").closest("label")).toHaveTextContent("(in/s^2)");
    expect(screen.queryByTestId(NOTICE)).toBeNull();
  });
});

describe("component modifiers take the §2.4 compositions (§7.6)", () => {
  const LINEAR = "modifiers.linear_stiffness_user_value.value";
  const ROTATIONAL = "modifiers.rotational_stiffness_user_value.value";

  it("the demo (no moment key) keeps N/m and N*m/rad", async () => {
    await editField(model(), VALVE, LINEAR);
    expectFormControlValue(byId("editor-intent-unit"), "N/m");
    changeFormControl(byId("editor-intent-field"), { target: { value: ROTATIONAL } });
    expectFormControlValue(byId("editor-intent-unit"), "N*m/rad");
  });

  it("the US preset gives lbf/in; rotational stiffness starts unselected with the notice", async () => {
    const us = { length: "in", force: "lbf", moment: "lbf*in", pressure: "psi", stress: "psi", temperature: "degF", angle: "deg" };
    await editField(model(us), VALVE, LINEAR);
    expectFormControlValue(byId("editor-intent-unit"), "lbf/in");
    changeFormControl(byId("editor-intent-field"), { target: { value: ROTATIONAL } });
    expectFormControlValue(byId("editor-intent-unit"), "");
    expect(byId(NOTICE)).toHaveTextContent("lbf*in/deg is not a catalog unit; choose one.");
  });

  it("names a missing part: force for linear stiffness, angle for rotational stiffness", async () => {
    await editField(model({ force: undefined, moment: "N*m" }), VALVE, LINEAR);
    expect(byId(NOTICE)).toHaveAttribute("data-key", "force");
    cleanup();
    await editField(model({ angle: undefined }), VALVE, ROTATIONAL);
    expect(byId(NOTICE)).toHaveAttribute("data-key", "angle");
  });

  it("a stored unit is kept", async () => {
    const stored = model({ length: "in", force: "lbf" });
    (stored.components.find((component) => component.id === "component:C-130")!.modifiers as Record<string, unknown>).linear_stiffness_user_value = { value: 1.5e7, unit: "N/m" };
    await editField(stored, VALVE, LINEAR);
    expectFormControlValue(byId("editor-intent-unit"), "N/m");
  });
});

describe("the grid refuses an empty cell by name and points to the inspector", () => {
  async function grid(document: PreviewModel, entityType: "load_cases" | "components", inputId: string, catalog: UnitCatalogRoute = CATALOG) {
    route.current = catalog;
    const queue = vi.fn<(intent: EditorOperationIntent) => void>();
    render(<ModelTree model={document} onQueueIntent={queue} onSelect={vi.fn()} projectSessionGeneration={0} selection={{ type: "load", id: "load:L-100" }} />);
    fireEvent.click(byId("layout-mode-grid"));
    fireEvent.click(byId(`entity-grid-type-${entityType}`));
    // The grid loads the catalog when it mounts.
    await act(async () => {});
    fireEvent.change(byId(inputId), { target: { value: "2" } });
    fireEvent.click(byId("queue-entity-grid-intents"));
    return queue;
  }

  it("solve temperature queues in the key's unit, and with the key missing is refused", async () => {
    const queued = await grid(model(), "load_cases", "entity-grid-input-load:L-100-modulus-basis-temperature");
    expect(queued.mock.calls[0][0].change.unit).toBe("degC");
    cleanup();
    const refused = await grid(model({ temperature: "  " }), "load_cases", "entity-grid-input-load:L-100-modulus-basis-temperature");
    expect(refused).not.toHaveBeenCalled();
    expect(screen.getByText("Not queued: Solve temperature of load:L-100 has no unit. Missing project unit: temperature. This cell has no default unit, and nothing is assumed. Choose its unit in the property inspector, or set temperature in Project units on the Project page.")).toBeInTheDocument();
  });

  it("seismic gravity: m/s^2 for m; refused for in/s^2 and for a missing length", async () => {
    const queued = await grid(model(), "load_cases", "entity-grid-input-load:L-100-seismic-gravity");
    expect(queued.mock.calls[0][0].change.unit).toBe("m/s^2");
    cleanup();
    const inches = await grid(model({ length: "in" }), "load_cases", "entity-grid-input-load:L-100-seismic-gravity");
    expect(inches).not.toHaveBeenCalled();
    expect(screen.getByText("Not queued: Seismic gravity of load:L-100 has no unit. in/s^2 is not a catalog unit; choose one. Choose its unit in the property inspector.")).toBeInTheDocument();
    cleanup();
    const missing = await grid(model({ length: undefined }), "load_cases", "entity-grid-input-load:L-100-seismic-gravity");
    expect(missing).not.toHaveBeenCalled();
    expect(screen.getByText(/^Not queued: Seismic gravity of load:L-100 has no unit\. Missing project unit: length\. This cell has no default unit/)).toBeInTheDocument();
  });

  it("component linear stiffness: N/m in the demo; refused with force missing", async () => {
    const queued = await grid(model(), "components", "entity-grid-input-component:C-130-linear-stiffness");
    expect(queued.mock.calls[0][0].change.unit).toBe("N/m");
    cleanup();
    const refused = await grid(model({ force: undefined }), "components", "entity-grid-input-component:C-130-linear-stiffness");
    expect(refused).not.toHaveBeenCalled();
    expect(screen.getByText(/^Not queued: Linear stiffness of component:C-130 has no unit\. Missing project unit: force\./)).toBeInTheDocument();
  });
});

// Codex review round 1 (P2): a catalog that arrives after the field was opened replaces
// an inherited default it does not admit; a unit the user chose is kept.
describe("a catalog that arrives late (§7.3, §7.6)", () => {
  async function openWhileLoading(document: PreviewModel, selection: EntityRef, fieldPath: string) {
    let arrive!: (value: UnitCatalogRoute) => void;
    route.delayed = new Promise<UnitCatalogRoute>((resolve) => { arrive = resolve; });
    const mounted = mount(document, selection);
    expect(byId("property-unit-catalog-status")).toHaveTextContent("loading");
    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    fireEvent.click(byId("inspector-start-task"));
    changeFormControl(byId("editor-intent-field"), { target: { value: fieldPath } });
    return { ...mounted, arrive: async () => { await act(async () => { arrive(CATALOG); }); } };
  }

  it("inch project, seismic gravity: in/s^2 while loading, then unselected with the notice and nothing queued", async () => {
    const { arrive, queue } = await openWhileLoading(model({ length: "in" }), LOAD, GRAVITY);
    expectFormControlValue(byId("editor-intent-unit"), "in/s^2");
    await arrive();
    expectFormControlValue(byId("editor-intent-unit"), "");
    expect(byId(NOTICE)).toHaveTextContent("in/s^2 is not a catalog unit; choose one.");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "386" } });
    expect(byId("queue-editor-intent")).toBeDisabled();
    fireEvent.click(byId("queue-editor-intent"));
    expect(queue).not.toHaveBeenCalled();
  });

  it("US preset, rotational stiffness: lbf*in/deg while loading, then unselected with the notice", async () => {
    const us = { length: "in", force: "lbf", moment: "lbf*in", pressure: "psi", stress: "psi", temperature: "degF", angle: "deg" };
    const { arrive } = await openWhileLoading(model(us), VALVE, "modifiers.rotational_stiffness_user_value.value");
    expectFormControlValue(byId("editor-intent-unit"), "lbf*in/deg");
    await arrive();
    expectFormControlValue(byId("editor-intent-unit"), "");
    expect(byId(NOTICE)).toHaveTextContent("lbf*in/deg is not a catalog unit; choose one.");
    expect(byId("queue-editor-intent")).toBeDisabled();
  });

  it("a stored unit is kept when the catalog arrives", async () => {
    const stored = model({ length: "in" });
    stored.load_cases[0].equivalent_static = { seismic: { gravity_acceleration: { value: 9.8, unit: "m/s^2" } } } as PreviewModel["load_cases"][number]["equivalent_static"];
    const { arrive } = await openWhileLoading(stored, LOAD, GRAVITY);
    await arrive();
    expectFormControlValue(byId("editor-intent-unit"), "m/s^2");
    expect(screen.queryByTestId(NOTICE)).toBeNull();
  });

  it("a unit the user chose is kept when the inherited default changes; an inherited one follows", async () => {
    // While the catalog loads, the selector offers only the inherited default, so a different
    // choice is made with the catalog loaded and the inherited default is then changed by a
    // project-unit change (the same recompute a late catalog triggers).
    const { queue, view } = await editField(model(), LOAD, TEMPERATURE);
    changeFormControl(byId("editor-intent-unit"), { target: { value: "K" } });
    const rerender = (document: PreviewModel) => view.rerender(
      <DisplayUnitsProvider initialPreference="SI">
        <PropertyInspector model={document} selection={LOAD} onQueueIntent={queue} onValidateIntent={vi.fn()} onApplyIntent={vi.fn(async () => true)} />
      </DisplayUnitsProvider>
    );
    rerender(model({ temperature: "degF" }));
    expectFormControlValue(byId("editor-intent-unit"), "K");
    queueValue("400");
    expect(JSON.parse(queue.mock.calls[0][0].change.after)).toEqual({ value: 400, unit: "K" });
    // Re-selecting the field drops the choice; the inherited default then follows the key.
    changeFormControl(byId("editor-intent-field"), { target: { value: GRAVITY } });
    changeFormControl(byId("editor-intent-field"), { target: { value: TEMPERATURE } });
    expectFormControlValue(byId("editor-intent-unit"), "degF");
    rerender(model({ temperature: "degR" }));
    expectFormControlValue(byId("editor-intent-unit"), "degR");
  });
});

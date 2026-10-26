// W-17 (DEL-02-02 Design `project-unit-selection.md` §3): the entity editors on
// a document missing project unit keys. Selecting a node, a pipe, a component or
// a load does not throw; a field whose default unit is a missing key has no
// default (nothing is assumed), names the key and is refused until a unit is
// stated; a unit the user picks in the field's own selector is kept; and nothing
// is queued meanwhile. With every key present the editors behave as before.
// Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import type { ProjectUnitKey } from "../../services/projectService";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { changeFormControl, expectFormControlValue } from "../../test-support/workspaceTestControls";
import { DisplayUnitsProvider } from "../display-units";
import { ModelTree } from "./ModelTree";
import { PropertyInspector } from "./PropertyInspector";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: async () => route.current
}));
const CATALOG: UnitCatalogRoute = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };

afterEach(() => { cleanup(); route.current = null; });

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
/** The bundled invented demo with all seven keys, and with the stored quantities
 * removed whose unit would otherwise hide the project default: pipe P-100's wall,
 * bend C-110's angle, valve C-130's weight and centre of gravity. Load L-100 has no wind. */
function model(missing: ProjectUnitKey[] = []): PreviewModel {
  const document = JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
  document.project.units = { ...document.project.units, moment: "N*m" };
  for (const key of missing) delete (document.project.units as Record<string, string>)[key];
  delete (document.pipe_segments.find((pipe) => pipe.id === "pipe:P-100")!.section as Record<string, unknown>).wall_thickness;
  delete document.components.find((component) => component.id === "component:C-110")!.geometry!.bend_angle;
  const valve = document.components.find((component) => component.id === "component:C-130")!.geometry!;
  delete valve.weight;
  delete valve.center_of_gravity;
  return document;
}

const byId = (testId: string) => screen.getByTestId(testId);
const NOTICE = "editor-intent-missing-project-unit";

function mount(document: PreviewModel, selection: EntityRef) {
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  const validate = vi.fn<(intent: EditorOperationIntent) => void>();
  render(
    <DisplayUnitsProvider initialPreference="SI">
      <PropertyInspector model={document} selection={selection} onQueueIntent={queue} onValidateIntent={validate} onApplyIntent={vi.fn(async () => true)} />
    </DisplayUnitsProvider>
  );
  return { queue, validate };
}

/** Selects the entity, starts a property task and picks the field; returns the queue. */
async function editField(document: PreviewModel, selection: EntityRef, fieldPath: string) {
  const mounted = mount(document, selection);
  await waitFor(() => expect(byId("property-unit-catalog-status")).toHaveTextContent("DEC-018"));
  fireEvent.click(screen.getByRole("tab", { name: "Task" }));
  fireEvent.click(byId("inspector-start-task"));
  changeFormControl(byId("editor-intent-field"), { target: { value: fieldPath } });
  return mounted;
}

const INVENTED = ["m", "rad", "N", "Pa"];

type Case = { key: ProjectUnitKey; selection: EntityRef; fieldPath: string; unit: string; mode: "positions" | "required" | "choose"; pick?: string };
const CASES: Case[] = [
  { key: "length", selection: { type: "node", id: "node:N-110" }, fieldPath: "position.x", unit: "m", mode: "positions" },
  { key: "length", selection: { type: "pipe", id: "pipe:P-100" }, fieldPath: "section.mill_tolerance.value", unit: "m", mode: "choose", pick: "mm" },
  { key: "length", selection: { type: "component", id: "component:C-130" }, fieldPath: "geometry.center_of_gravity", unit: "m", mode: "required" },
  { key: "angle", selection: { type: "component", id: "component:C-110" }, fieldPath: "geometry.bend_angle.value", unit: "rad", mode: "choose", pick: "deg" },
  { key: "force", selection: { type: "component", id: "component:C-130" }, fieldPath: "geometry.weight.value", unit: "N", mode: "choose", pick: "kN" },
  { key: "pressure", selection: { type: "load", id: "load:L-100" }, fieldPath: "equivalent_static.wind.pressure.value", unit: "Pa", mode: "choose", pick: "kPa" }
];

describe("entity editors with every project unit key", () => {
  it.each(CASES)("$selection.type $fieldPath defaults to the $key key's unit and queues as before", async ({ selection, fieldPath, unit, mode }) => {
    route.current = CATALOG;
    const { queue } = await editField(model(), selection, fieldPath);
    expect(screen.queryByTestId(NOTICE)).toBeNull();
    if (mode !== "required") expectFormControlValue(byId("editor-intent-unit"), unit);
    fireEvent.change(byId("editor-intent-value"), { target: { value: mode === "required" ? "x=1, y=0, z=0 m" : "2" } });
    expect(byId("queue-editor-intent")).toBeEnabled();
    fireEvent.click(byId("queue-editor-intent"));
    expect(queue).toHaveBeenCalledTimes(1);
    expect(queue.mock.calls[0][0].change.unit).toBe(unit);
  });

  it("the component creation form defaults to the keys' units, with no notice", async () => {
    route.current = CATALOG;
    mount(model(), { type: "node", id: "node:N-110" });
    await waitFor(() => expect(byId("property-unit-catalog-status")).toHaveTextContent("DEC-018"));
    expectFormControlValue(byId("create-component-radius-unit"), "m");
    expectFormControlValue(byId("create-component-angle-unit"), "rad");
    expect(screen.queryByTestId(/^create-component-missing-project-unit-/)).toBeNull();
  });
});

describe("entity editors on a document missing a project unit key", () => {
  it.each(CASES)("missing $key: $selection.type $fieldPath has no default, names the key and queues nothing ($mode)", async ({ key, selection, fieldPath, mode, pick }) => {
    route.current = CATALOG;
    const document = model([key]);
    const before = JSON.stringify(document);
    const { queue, validate } = await editField(document, selection, fieldPath);
    const notice = byId(NOTICE);
    expect(notice).toHaveAttribute("data-key", key);
    expect(notice).toHaveTextContent(`Missing project unit: ${key}.`);
    if (mode === "positions") {
      expect(notice).toHaveAttribute("role", "alert");
      expect(notice).toHaveTextContent("Re-author the model with a length unit; no fix is offered.");
    } else {
      expect(notice).toHaveTextContent("nothing is assumed");
      expect(notice).toHaveTextContent("Project units on the Project page");
    }
    if (mode !== "required") {
      // No unit is invented: the field's unit starts unselected.
      expectFormControlValue(byId("editor-intent-unit"), "");
      expect(INVENTED).not.toContain(byId("editor-intent-unit").getAttribute("data-value"));
    }
    fireEvent.change(byId("editor-intent-value"), { target: { value: mode === "required" ? "x=1, y=0, z=0 m" : "2" } });
    for (const action of ["queue-editor-intent", "validate-editor-intent-inline", "apply-editor-intent-inline"]) expect(byId(action)).toBeDisabled();
    fireEvent.click(byId("queue-editor-intent"));
    expect(queue).not.toHaveBeenCalled();
    expect(validate).not.toHaveBeenCalled();
    if (mode === "positions") {
      // A position is stored in project.units.length: an explicit unit does not lift the lock.
      changeFormControl(byId("editor-intent-unit"), { target: { value: "mm" } });
      expect(byId(NOTICE)).toBeInTheDocument();
      expect(byId("queue-editor-intent")).toBeDisabled();
    }
    if (pick) {
      // A unit the user picks in the field's own selector is kept, not a fallback.
      changeFormControl(byId("editor-intent-unit"), { target: { value: pick } });
      expect(screen.queryByTestId(NOTICE)).toBeNull();
      expect(byId("queue-editor-intent")).toBeEnabled();
      fireEvent.click(byId("queue-editor-intent"));
      expect(queue).toHaveBeenCalledTimes(1);
      expect(queue.mock.calls[0][0].change.unit).toBe(pick);
    }
    expect(queue.mock.calls.every(([intent]) => !INVENTED.includes(intent.change.unit))).toBe(true);
    expect(JSON.stringify(document)).toBe(before);
  });

  it("missing angle and force: the component creation form starts those units unselected and names each key", async () => {
    route.current = CATALOG;
    const document = model(["angle", "force"]);
    const queue = mount(document, { type: "node", id: "node:N-110" }).queue;
    await waitFor(() => expect(byId("property-unit-catalog-status")).toHaveTextContent("DEC-018"));
    expectFormControlValue(byId("create-component-radius-unit"), "m");
    expectFormControlValue(byId("create-component-angle-unit"), "TBD");
    expect(byId("create-component-missing-project-unit-angle")).toHaveTextContent("Missing project unit: angle.");
    changeFormControl(byId("create-component-kind"), { target: { value: "valve" } });
    expectFormControlValue(byId("create-component-force-unit"), "TBD");
    expect(byId("create-component-missing-project-unit-force")).toHaveTextContent("Missing project unit: force.");
    expect(byId("queue-create-component-intent")).toBeDisabled();
    expect(queue).not.toHaveBeenCalled();
  });

  // A document without the map itself is refused at every entry (MODEL-FILE-SHAPE); an empty map is permitted.
  it("selecting a node, pipe, component and load with every key missing renders without throwing", () => {
    const document = model(["length", "force", "moment", "pressure", "stress", "temperature", "angle"]);
    for (const selection of [
      { type: "node", id: "node:N-110" }, { type: "pipe", id: "pipe:P-100" }, { type: "component", id: "component:C-110" },
      { type: "component", id: "component:C-130" }, { type: "load", id: "load:L-100" }
    ] as EntityRef[]) {
      mount(document, selection);
      expect(byId("property-inspector")).toBeInTheDocument();
      // The node's Properties name the missing unit rather than showing one.
      if (selection.type === "node") expect(screen.getByText("3.2, 0, 0 (Missing project unit: length)")).toBeInTheDocument();
      cleanup();
    }
  });
});

describe("grid queue on a document missing a project unit key", () => {
  function grid(document: PreviewModel) {
    const queue = vi.fn<(intent: EditorOperationIntent) => void>();
    render(<ModelTree model={document} onQueueIntent={queue} onSelect={vi.fn()} projectSessionGeneration={0} selection={{ type: "component", id: "component:C-110" }} />);
    fireEvent.click(byId("layout-mode-grid"));
    fireEvent.click(byId("entity-grid-type-components"));
    fireEvent.change(byId("entity-grid-input-component:C-110-bend-angle"), { target: { value: "1.2" } });
    fireEvent.click(byId("queue-entity-grid-intents"));
    return queue;
  }

  it("with angle present, a bend angle cell queues in the angle key's unit", () => {
    const queue = grid(model());
    expect(queue).toHaveBeenCalledTimes(1);
    expect(queue.mock.calls[0][0].change.unit).toBe("rad");
  });

  it("with angle missing, the cell is refused by name and nothing is queued", () => {
    const queue = grid(model(["angle"]));
    expect(queue).not.toHaveBeenCalled();
    expect(screen.getByText(/^Not queued: Bend angle of component:C-110 has no unit\. Missing project unit: angle\./)).toBeInTheDocument();
  });
});

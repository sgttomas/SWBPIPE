// The inspector's property task under DEL-02-02 REQ-022 and AC-011 (second independent
// reading, finding 1): an absent member's quantity or vector takes its unit from the
// project, as a node position always does. A value typed beside it is cleared when that
// unit moves with the project units, and the task says which unit changed. A stored unit
// and a unit the person chose never clear. Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { changeFormControl, chooseCompactOption } from "../../test-support/workspaceTestControls";
import { DisplayUnitsProvider } from "../display-units";
import { PropertyInspector } from "./PropertyInspector";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: () => Promise.resolve(route.current)
}));

afterEach(() => { cleanup(); route.current = null; });

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
/** The invented demo, with a bend, a reducer and a legacy joint whose geometry is absent,
 * and the valve's stiffness modifiers removed so their units come from the project. */
function demo(units: Record<string, string> = {}, edit: (model: PreviewModel) => void = () => {}): PreviewModel {
  const model = JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
  model.project.units = { ...model.project.units, ...units } as PreviewModel["project"]["units"];
  model.components.push(
    { id: "component:T-BEND", label: "Bend without geometry", kind: "bend", node: "node:N-110", provenance: "invented" },
    { id: "component:T-REDUCER", label: "Reducer without offset", kind: "reducer", node: "node:N-110", provenance: "invented" },
    { id: "component:T-JOINT", label: "Legacy joint", kind: "expansion_joint", node: "node:N-110", provenance: "invented" }
  );
  // A pipe without a stored wall thickness: its mill tolerance and corrosion allowance then
  // take the project length (with a stored wall thickness they take its unit).
  const pipe = structuredClone(model.pipe_segments[0]);
  model.pipe_segments.push({ ...pipe, id: "pipe:T-PIPE", label: "Pipe without wall", section: { outside_diameter: pipe.section.outside_diameter } });
  const modifiers = model.components.find((component) => component.id === "component:C-130")!.modifiers as Record<string, unknown>;
  delete modifiers.linear_stiffness_user_value;
  delete modifiers.rotational_stiffness_user_value;
  edit(model);
  return model;
}

const byId = (testId: string) => screen.getByTestId(testId);
const notice = () => screen.queryByTestId("editor-intent-unit-cleared");

async function startTask(selection: EntityRef, fieldPath: string, units: Record<string, string> = {}) {
  route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  const tree = (model: PreviewModel, session = 1) => (
    <DisplayUnitsProvider initialPreference="SI">
      <PropertyInspector model={model} selection={selection} onQueueIntent={queue} projectSessionGeneration={session} />
    </DisplayUnitsProvider>
  );
  const view = render(tree(demo(units)));
  await waitFor(() => expect(byId("property-unit-catalog-status")).not.toHaveTextContent("loading"));
  fireEvent.click(screen.getByRole("tab", { name: "Task" }));
  fireEvent.click(byId("inspector-start-task"));
  changeFormControl(byId("editor-intent-field"), { target: { value: fieldPath } });
  const starting = (byId("editor-intent-value") as HTMLInputElement).value;
  return {
    starting,
    rerender: (next: Record<string, string>, session = 1, edit?: (model: PreviewModel) => void) => view.rerender(tree(demo(next, edit), session))
  };
}

const escape = (text: string) => text.replace(/[.*+?^${}()|[\]\\/]/g, "\\$&");
const cleared = (from: string, to: string) => new RegExp(`unit changed from ${escape(from)} to ${escape(to)} with the project units; the values beside it were cleared\\.`);

describe("the property task clears a value typed beside a unit taken from the project", () => {
  it("the reviewer's case: an absent solve temperature, 400 typed, the project temperature unit changed", async () => {
    const { starting, rerender } = await startTask({ type: "load", id: "load:L-100" }, "modulus_basis_temperature.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "400" } });
    rerender({ temperature: "K" });
    expect(byId("editor-intent-value")).toHaveValue(starting);
    expect(notice()).toHaveTextContent(cleared("degC", "K"));
  });

  it.each([
    ["node coordinates", { type: "node", id: "node:N-110" }, "position.x", { length: "mm" }, "m", "mm"],
    ["pipe mill tolerance", { type: "pipe", id: "pipe:T-PIPE" }, "section.mill_tolerance.value", { length: "mm" }, "m", "mm"],
    ["pipe corrosion allowance", { type: "pipe", id: "pipe:T-PIPE" }, "section.corrosion_allowance.value", { length: "mm" }, "m", "mm"],
    ["a component's missing radius", { type: "component", id: "component:T-BEND" }, "geometry.bend_radius.value", { length: "mm" }, "m", "mm"],
    ["a component's missing angle", { type: "component", id: "component:T-BEND" }, "geometry.bend_angle.value", { angle: "deg" }, "rad", "deg"],
    ["a reducer's eccentric offset (a vector)", { type: "component", id: "component:T-REDUCER" }, "geometry.eccentric_offset", { length: "mm" }, "m", "mm"],
    ["a component's linear stiffness", { type: "component", id: "component:C-130" }, "modifiers.linear_stiffness_user_value.value", { length: "mm" }, "N/m", "N/mm"],
    ["a legacy joint's movement limit", { type: "component", id: "component:T-JOINT" }, "geometry.movement_limit.value", { length: "mm" }, "m", "mm"],
    ["a load case's seismic gravity", { type: "load", id: "load:L-100" }, "equivalent_static.seismic.gravity_acceleration.value", { length: "mm" }, "m/s^2", null],
    ["a load case's wind pressure", { type: "load", id: "load:L-100" }, "equivalent_static.wind.pressure.value", { pressure: "MPa" }, "Pa", "MPa"]
  ] as const)("%s", async (_family, selection, fieldPath, units, from, to) => {
    const { starting, rerender } = await startTask(selection as EntityRef, fieldPath);
    const typed = fieldPath === "geometry.eccentric_offset" ? "0, -0.05, 0" : "4";
    fireEvent.change(byId("editor-intent-value"), { target: { value: typed } });
    rerender(units);
    expect(byId("editor-intent-value")).toHaveValue(starting);
    // A composed unit the catalog lacks gives no unit ("none"), which is a move too.
    expect(notice()).toHaveTextContent(to === null ? new RegExp(`unit changed from ${escape(from)} to `) : cleared(from, to));
  });

  it("a stored unit never clears", async () => {
    // The valve's body length is stored with its own unit. (Likewise a mill tolerance on a
    // pipe whose wall thickness is stored takes that stored unit.)
    const { rerender } = await startTask({ type: "component", id: "component:C-130" }, "geometry.rigid_body_length.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "0.4" } });
    rerender({ length: "mm" });
    expect(byId("editor-intent-value")).toHaveValue("0.4");
    expect(notice()).toBeNull();
  });

  it("a mill tolerance on a pipe with a stored wall thickness takes that stored unit: it moves only with the record and never clears", async () => {
    const { rerender } = await startTask({ type: "pipe", id: "pipe:P-100" }, "section.mill_tolerance.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "0.001" } });
    rerender({ length: "mm" });
    expect(byId("editor-intent-value")).toHaveValue("0.001");
    // An applied edit stores the wall thickness in mm: the mill tolerance's unit follows the
    // stored member, not the project units, so the typed value stays.
    rerender({ length: "mm" }, 1, (model) => {
      const wall = model.pipe_segments[0].section.wall_thickness!;
      model.pipe_segments[0].section.wall_thickness = { value: Number(wall.value) * 1000, unit: "mm" };
    });
    expect(byId("editor-intent-value")).toHaveValue("0.001");
    expect(notice()).toBeNull();
  });

  it("a unit the person chose never clears, and a change that leaves the unit clears nothing", async () => {
    const { rerender } = await startTask({ type: "load", id: "load:L-100" }, "modulus_basis_temperature.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "400" } });
    rerender({ pressure: "MPa" });
    expect(byId("editor-intent-value")).toHaveValue("400");
    expect(notice()).toBeNull();
    // The person's own choice of unit after typing is not a move with the project units.
    chooseCompactOption(byId("editor-intent-unit"), "K");
    expect(byId("editor-intent-value")).toHaveValue("400");
    expect(notice()).toBeNull();
    rerender({ temperature: "degF" });
    expect(byId("editor-intent-value")).toHaveValue("400");
    expect(notice()).toBeNull();
  });

  it("a later move that clears nothing names where the unit now is", async () => {
    const { rerender } = await startTask({ type: "load", id: "load:L-100" }, "modulus_basis_temperature.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "400" } });
    rerender({ temperature: "K" });
    expect(notice()).toHaveTextContent(cleared("degC", "K"));
    rerender({ temperature: "degF" });
    expect(notice()).toHaveTextContent(cleared("degC", "degF"));
  });
});

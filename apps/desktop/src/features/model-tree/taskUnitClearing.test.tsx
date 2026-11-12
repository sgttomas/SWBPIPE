// The inspector's property task under DEL-02-02 REQ-022 and AC-011: when the unit shown
// beside a typed value moves by anything other than the person's own choice of that unit
// in the task's unit control, the value is cleared and the task says which unit changed.
// The cause does not matter: the project units, another stored value replaced, the
// field's own stored value replaced from outside, an absent member becoming stored.
// Invented data only.
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
const cleared = (from: string, to: string) => new RegExp(`unit changed from ${escape(from)} to ${escape(to)}; the values beside it were cleared\\.`);

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

  it("a stored unit that does not move clears nothing", async () => {
    // The valve's body length is stored with its own unit; a project length change leaves it.
    const { rerender } = await startTask({ type: "component", id: "component:C-130" }, "geometry.rigid_body_length.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "0.4" } });
    rerender({ length: "mm" });
    expect(byId("editor-intent-value")).toHaveValue("0.4");
    expect(notice()).toBeNull();
  });

  // The third reading's case. The earlier version of this test expected the typed 0.001 to
  // stay while its unit moved from m to mm: it had recorded the defect as behaviour.
  it.each([
    ["section.mill_tolerance.value", "0.001"],
    ["section.corrosion_allowance.value", "0.002"]
  ])("%s beside a wall thickness re-stored in another unit is cleared", async (fieldPath, typed) => {
    const { rerender } = await startTask({ type: "pipe", id: "pipe:P-100" }, fieldPath);
    fireEvent.change(byId("editor-intent-value"), { target: { value: typed } });
    rerender({ length: "mm" });
    expect(byId("editor-intent-value")).toHaveValue(typed);
    expect(notice()).toBeNull();
    // Another edit stores the wall thickness in mm: the unit shown moves without the person's choice.
    rerender({ length: "mm" }, 1, (model) => {
      const wall = model.pipe_segments[0].section.wall_thickness!;
      model.pipe_segments[0].section.wall_thickness = { value: Number(wall.value) * 1000, unit: "mm" };
    });
    expect(byId("editor-intent-value")).not.toHaveValue(typed);
    expect(notice()).toHaveTextContent(cleared("m", "mm"));
  });

  it("the field's own stored value replaced from outside in another unit clears the typed value", async () => {
    const { rerender } = await startTask({ type: "component", id: "component:C-130" }, "geometry.rigid_body_length.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "0.4" } });
    rerender({}, 1, (model) => {
      const valve = model.components.find((component) => component.id === "component:C-130")!;
      const length = valve.geometry!.rigid_body_length!;
      valve.geometry!.rigid_body_length = { value: Number(length.value) * 1000, unit: "mm" };
    });
    expect(byId("editor-intent-value")).not.toHaveValue("0.4");
    expect(notice()).toHaveTextContent(cleared("m", "mm"));
  });

  it("an absent member becoming stored in another unit clears the typed value", async () => {
    const { rerender } = await startTask({ type: "load", id: "load:L-100" }, "modulus_basis_temperature.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "400" } });
    rerender({}, 1, (model) => { model.load_cases[0].modulus_basis_temperature = { value: 300, unit: "K" }; });
    expect(byId("editor-intent-value")).toHaveValue("300");
    expect(notice()).toHaveTextContent(cleared("degC", "K"));
  });

  it("baseline: another field is another entry, so switching field raises no notice; a real move on the new field still clears", async () => {
    const { rerender } = await startTask({ type: "load", id: "load:L-100" }, "modulus_basis_temperature.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "400" } });
    changeFormControl(byId("editor-intent-field"), { target: { value: "equivalent_static.wind.pressure.value" } });
    expect(notice()).toBeNull();
    fireEvent.change(byId("editor-intent-value"), { target: { value: "5" } });
    rerender({ pressure: "MPa" });
    expect(notice()).toHaveTextContent(cleared("Pa", "MPa"));
  });

  it("baseline: another project session raises no notice", async () => {
    const { rerender } = await startTask({ type: "load", id: "load:L-100" }, "modulus_basis_temperature.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "400" } });
    rerender({ temperature: "K" }, 2);
    expect(notice()).toBeNull();
  });

  it("a unit the person chose never clears, and a change that leaves the unit clears nothing", async () => {
    const { rerender } = await startTask({ type: "load", id: "load:L-100" }, "modulus_basis_temperature.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "400" } });
    rerender({ pressure: "MPa" });
    expect(byId("editor-intent-value")).toHaveValue("400");
    expect(notice()).toBeNull();
    // The person's own choice of the unit after typing is the one exemption.
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

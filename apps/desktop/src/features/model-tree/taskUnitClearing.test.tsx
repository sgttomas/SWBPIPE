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

async function startTask(selection: EntityRef, fieldPath: string, units: Record<string, string> = {}, edit?: (model: PreviewModel) => void) {
  route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  const tree = (model: PreviewModel, session = 1) => (
    <DisplayUnitsProvider initialPreference="SI">
      <PropertyInspector model={model} selection={selection} onQueueIntent={queue} projectSessionGeneration={session} />
    </DisplayUnitsProvider>
  );
  const view = render(tree(demo(units, edit)));
  await waitFor(() => expect(byId("property-unit-catalog-status")).not.toHaveTextContent("loading"));
  fireEvent.click(screen.getByRole("tab", { name: "Task" }));
  fireEvent.click(byId("inspector-start-task"));
  changeFormControl(byId("editor-intent-field"), { target: { value: fieldPath } });
  const starting = (byId("editor-intent-value") as HTMLInputElement).value;
  return {
    starting,
    queue,
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

  it("baseline: the task's entry replaced by Cancel raises no notice; a real move after starting again still clears", async () => {
    const { starting, rerender } = await startTask({ type: "load", id: "load:L-100" }, "modulus_basis_temperature.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "400" } });
    chooseCompactOption(byId("editor-intent-unit"), "K");
    fireEvent.click(byId("cancel-editor-intent"));
    fireEvent.click(byId("inspector-start-task"));
    expect(notice()).toBeNull();
    changeFormControl(byId("editor-intent-field"), { target: { value: "modulus_basis_temperature.value" } });
    expect(byId("editor-intent-value")).toHaveValue(starting);
    expect(notice()).toBeNull();
    fireEvent.change(byId("editor-intent-value"), { target: { value: "410" } });
    rerender({ temperature: "K" });
    expect(notice()).toHaveTextContent(cleared("degC", "K"));
  });

  // The fourth reading. Each sequence is driven as a person would; "outside" is a change of
  // the model made elsewhere while the task is open.
  const steel = { type: "material", id: "material:invented-carbon-steel" } as const;
  const storeModulus = (value: number, unit: string) => (model: PreviewModel) => {
    model.materials![0].elastic_modulus = { value, unit } as never;
  };

  it("finding 1: the stored number retyped, then an outside edit re-stores it in another unit", async () => {
    const { starting, rerender } = await startTask(steel, "elastic_modulus.value");
    expect(starting).toBe("200000000000");
    // Select all and type: the input passes through "2" before the same digits stand again.
    fireEvent.change(byId("editor-intent-value"), { target: { value: "2" } });
    fireEvent.change(byId("editor-intent-value"), { target: { value: "200000000000" } });
    expect(byId("editor-intent-value")).toHaveValue("200000000000");
    rerender({}, 1, storeModulus(200000, "MPa"));
    expect(byId("editor-intent-value")).toHaveValue("200000");
    expect(notice()).toHaveTextContent(cleared("Pa", "MPa"));
  });

  it("finding 1: an outside edit first stores the typed number, then re-stores it in another unit", async () => {
    const { rerender } = await startTask(steel, "elastic_modulus.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "150000000000" } });
    rerender({}, 1, storeModulus(150000000000, "Pa"));
    rerender({}, 1, storeModulus(150000, "MPa"));
    expect(byId("editor-intent-value")).toHaveValue("150000");
    expect(notice()).toHaveTextContent(cleared("Pa", "MPa"));
  });

  it("finding 1, untouched: nothing typed, an outside edit re-stores the value in another unit", async () => {
    const { rerender } = await startTask(steel, "elastic_modulus.value");
    expect(screen.getByText("New elastic modulus (Pa)")).toBeInTheDocument();
    rerender({}, 1, storeModulus(200000, "MPa"));
    expect(screen.getByText("New elastic modulus (MPa)")).toBeInTheDocument();
    // The untouched entry follows the stored entry: no digits stand beside a unit they were
    // not stored in, nothing is queueable, and nothing was cleared.
    expect(byId("editor-intent-value")).toHaveValue("200000");
    expect(byId("queue-editor-intent")).toBeDisabled();
    expect(notice()).toBeNull();
  });

  // The task's draft is keyed to the stored entry it was made against (DEL-02-02 REQ-022).
  const spare = { type: "material", id: "material:T-SPARE" } as const;
  const withSpare = (value: number, unit: string, label = "Spare") => (model: PreviewModel) => {
    model.materials!.push({ ...structuredClone(model.materials![0]), id: "material:T-SPARE", label, elastic_modulus: { value, unit } } as never);
  };
  const add = () => byId("queue-editor-intent") as HTMLButtonElement;
  const stale = () => screen.queryByTestId("editor-intent-stale");

  it("finding 2: the target deleted, then re-created in another unit: nothing typed for the old record stands beside the new unit", async () => {
    const { rerender } = await startTask(spare, "elastic_modulus.value", {}, withSpare(210000, "MPa"));
    fireEvent.change(byId("editor-intent-value"), { target: { value: "150000" } });
    rerender({});
    // While it is absent the person is told, and nothing can be added.
    expect(byId("editor-intent-target-absent")).toHaveTextContent("The task's target, material: material:T-SPARE, is not in the model, so nothing can be added.");
    expect(screen.queryByTestId("queue-editor-intent")).toBeNull();
    rerender({}, 1, withSpare(210000000000, "Pa"));
    expect(screen.queryByDisplayValue("150000")).toBeNull();
    expect(byId("editor-intent-value")).toHaveValue("210000000000");
    expect(screen.getByText("New elastic modulus (Pa)")).toBeInTheDocument();
    expect(notice()).toHaveTextContent(cleared("MPa", "Pa"));
    expect(add()).toBeDisabled();
  });

  it("stale: an edited entry whose stored value changes under the same unit is shown against the changed value and cannot be added", async () => {
    const { rerender, queue } = await startTask(steel, "elastic_modulus.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "210000000000" } });
    rerender({}, 1, storeModulus(195000000000, "Pa"));
    expect(byId("editor-intent-value")).toHaveValue("210000000000");
    expect(stale()).toHaveTextContent("The stored elastic modulus is now 195000000000 Pa; this entry was made against 200000000000 Pa.");
    expect(add()).toBeDisabled();
    expect(add().getAttribute("aria-describedby")).toBe("editor-intent-stale-reason");
    expect(document.getElementById("editor-intent-stale-reason")).toHaveTextContent("Not added: the stored value changed after this entry was made.");
    expect(byId("apply-editor-intent-inline")).toBeDisabled();
    // A keystroke elsewhere does not end it.
    fireEvent.change(byId("editor-intent-rationale"), { target: { value: "checked against the mill certificate" } });
    expect(stale()).not.toBeNull();
    expect(add()).toBeDisabled();
    // The deliberate act beside the stale entry does.
    fireEvent.click(byId("editor-intent-stale-confirm"));
    expect(stale()).toBeNull();
    expect(add()).toBeEnabled();
    fireEvent.click(add());
    expect(queue).toHaveBeenCalledTimes(1);
    expect(queue.mock.calls[0][0].change.after).toBe(JSON.stringify({ value: 210000000000, unit: "Pa" }));
    expect(queue.mock.calls[0][0].change.before).toBe("195000000000");
  });

  it("choosing a unit is an edit: the digits then shown are the person's beside it, and a later stored change makes them stale", async () => {
    const { rerender } = await startTask(steel, "elastic_modulus.value");
    chooseCompactOption(byId("editor-intent-unit"), "MPa");
    expect(byId("editor-intent-value")).toHaveValue("200000000000");
    rerender({}, 1, storeModulus(195000000000, "Pa"));
    // Not 195000000000 beside MPa: stored digits never follow into a unit the person chose.
    expect(byId("editor-intent-value")).toHaveValue("200000000000");
    expect(stale()).toHaveTextContent("The stored elastic modulus is now 195000000000 Pa; this entry was made against 200000000000 Pa.");
    expect(add()).toBeDisabled();
  });

  it("stale: editing the value again ends it", async () => {
    const { rerender } = await startTask(steel, "elastic_modulus.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "210000000000" } });
    rerender({}, 1, storeModulus(195000000000, "Pa"));
    expect(stale()).not.toBeNull();
    fireEvent.change(byId("editor-intent-value"), { target: { value: "211000000000" } });
    expect(stale()).toBeNull();
    expect(add()).toBeEnabled();
  });

  it("identifier reuse in one update, no absent render, the field differing under the same unit: the typed draft is stale", async () => {
    const { rerender } = await startTask(spare, "elastic_modulus.value", {}, withSpare(210000, "MPa"));
    fireEvent.change(byId("editor-intent-value"), { target: { value: "150000" } });
    rerender({}, 1, withSpare(190000, "MPa", "Spare re-created"));
    expect(byId("editor-intent-value")).toHaveValue("150000");
    expect(stale()).toHaveTextContent("The stored elastic modulus is now 190000 MPa; this entry was made against 210000 MPa.");
    expect(add()).toBeDisabled();
  });

  it("identifier reuse in one update, the unit differing: the typed draft is cleared and the new stored entry shown", async () => {
    const { rerender } = await startTask(spare, "elastic_modulus.value", {}, withSpare(210000, "MPa"));
    fireEvent.change(byId("editor-intent-value"), { target: { value: "150000" } });
    rerender({}, 1, withSpare(210000000000, "Pa", "Spare re-created"));
    expect(byId("editor-intent-value")).toHaveValue("210000000000");
    expect(notice()).toHaveTextContent(cleared("MPa", "Pa"));
    expect(stale()).toBeNull();
    expect(add()).toBeDisabled();
  });

  it("identifier reuse in one update, nothing typed: the entry follows the new stored value and unit", async () => {
    const { rerender } = await startTask(spare, "elastic_modulus.value", {}, withSpare(210000, "MPa"));
    rerender({}, 1, withSpare(210000000000, "Pa", "Spare re-created"));
    expect(byId("editor-intent-value")).toHaveValue("210000000000");
    expect(screen.getByText("New elastic modulus (Pa)")).toBeInTheDocument();
    expect(notice()).toBeNull();
    expect(add()).toBeDisabled();
  });

  it("the stated limit: identifier reuse in one update, identical in the field, is indistinguishable from no change", async () => {
    const { rerender, queue } = await startTask(spare, "elastic_modulus.value", {}, withSpare(210000, "MPa"));
    fireEvent.change(byId("editor-intent-value"), { target: { value: "150000" } });
    rerender({}, 1, withSpare(210000, "MPa", "Spare re-created"));
    // The person's proposal means what it meant: 150000 MPa against a stored 210000 MPa.
    expect(byId("editor-intent-value")).toHaveValue("150000");
    expect(stale()).toBeNull();
    expect(notice()).toBeNull();
    fireEvent.click(add());
    expect(queue.mock.calls[0][0].change.after).toBe(JSON.stringify({ value: 150000, unit: "MPa" }));
  });

  it("a new project session arriving in the same update as an outside edit ends the task with nothing carried", async () => {
    const { rerender } = await startTask(steel, "elastic_modulus.value");
    fireEvent.change(byId("editor-intent-value"), { target: { value: "210000000000" } });
    rerender({}, 2, storeModulus(200000, "MPa"));
    expect(byId("inspector-task-empty")).toBeInTheDocument();
    expect(screen.queryByTestId("editor-intent-value")).toBeNull();
    fireEvent.click(byId("inspector-start-task"));
    changeFormControl(byId("editor-intent-field"), { target: { value: "elastic_modulus.value" } });
    expect(byId("editor-intent-value")).toHaveValue("200000");
    expect(notice()).toBeNull();
    expect(stale()).toBeNull();
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

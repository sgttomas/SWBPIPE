// The inspector's property task: an entry the person has not edited is the field's own
// stored entry, so it follows the stored value and its unit together when the model
// changes under the open task (an applied change, Undo, Redo). Editing is an event:
// retyping the stored digits is still an edit, so is choosing a unit, and an edited entry is
// not replaced when the stored value changes. What becomes of an edited entry when its unit
// changes is not decided here. Nor is whether an edited entry may still be added once the
// stored value it was made against has changed: no test here adds or applies an edited entry
// AFTER such a change, so that none records as intended what the fuller rule decides.
// Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { changeFormControl } from "../../test-support/workspaceTestControls";
import { DisplayUnitsProvider } from "../display-units";
import { PropertyInspector } from "./PropertyInspector";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: () => Promise.resolve(route.current)
}));

afterEach(() => { cleanup(); route.current = null; });

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const steel: EntityRef = { type: "material", id: "material:invented-carbon-steel" };

/** The invented demo, its steel's elastic modulus stored as given. */
function demo(modulus?: { value: number; unit: string }): PreviewModel {
  const model = JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
  if (modulus) model.materials![0].elastic_modulus = modulus as never;
  return model;
}

const value = () => screen.getByTestId("editor-intent-value") as HTMLInputElement;
const add = () => screen.getByTestId("queue-editor-intent") as HTMLButtonElement;

/** A task open on the steel's elastic modulus, stored as 200000000000 Pa; `restore` gives the inspector a changed model. */
async function modulusTask() {
  route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  const tree = (model: PreviewModel) => (
    <DisplayUnitsProvider initialPreference="SI">
      <PropertyInspector model={model} selection={steel} onQueueIntent={queue} projectSessionGeneration={1} />
    </DisplayUnitsProvider>
  );
  const view = render(tree(demo()));
  await waitFor(() => expect(screen.getByTestId("property-unit-catalog-status")).not.toHaveTextContent("loading"));
  fireEvent.click(screen.getByRole("tab", { name: "Task" }));
  fireEvent.click(screen.getByTestId("inspector-start-task"));
  changeFormControl(screen.getByTestId("editor-intent-field"), { target: { value: "elastic_modulus.value" } });
  expect(value()).toHaveValue("200000000000");
  expect(screen.getByText("New elastic modulus (Pa)")).toBeInTheDocument();
  return { queue, restore: (modulus: { value: number; unit: string }) => view.rerender(tree(demo(modulus))) };
}

describe("the property task's entry while the model changes under it", () => {
  it("an entry the person has not edited follows the stored value and unit together", async () => {
    const { queue, restore } = await modulusTask();
    restore({ value: 200000, unit: "MPa" });
    expect(screen.getByText("New elastic modulus (MPa)")).toBeInTheDocument();
    // Not 200000000000 beside MPa: digits nobody typed never stand beside a unit they were not stored in.
    expect(value()).toHaveValue("200000");
    expect(add()).toBeDisabled();
    expect(queue).not.toHaveBeenCalled();
  });

  it("an entry the person has not edited follows a stored value that changes under the same unit", async () => {
    const { restore } = await modulusTask();
    restore({ value: 195000000000, unit: "Pa" });
    expect(value()).toHaveValue("195000000000");
    expect(add()).toBeDisabled();
  });

  it("editing is an event: the stored digits retyped are the person's entry and do not follow", async () => {
    const { restore } = await modulusTask();
    // Select all and type: the input passes through "2" before the same digits stand again.
    fireEvent.change(value(), { target: { value: "2" } });
    fireEvent.change(value(), { target: { value: "200000000000" } });
    restore({ value: 195000000000, unit: "Pa" });
    expect(value()).toHaveValue("200000000000");
  });

  it("an edited entry queues what the person typed, and is left as typed when the stored value then changes", async () => {
    const { queue, restore } = await modulusTask();
    fireEvent.change(value(), { target: { value: "210000000000" } });
    fireEvent.click(add());
    expect(queue).toHaveBeenCalledTimes(1);
    expect(queue.mock.calls[0][0].change.after).toBe(JSON.stringify({ value: 210000000000, unit: "Pa" }));
    expect(queue.mock.calls[0][0].change.before).toBe("200000000000");
    restore({ value: 195000000000, unit: "Pa" });
    expect(value()).toHaveValue("210000000000");
  });

  it("choosing a unit is an edit: the digits shown beside it then are the person's entry and do not follow", async () => {
    const { queue, restore } = await modulusTask();
    changeFormControl(screen.getByTestId("editor-intent-unit"), { target: { value: "psi" } });
    expect(screen.getByText("New elastic modulus (psi)")).toBeInTheDocument();
    expect(value()).toHaveValue("200000000000");
    fireEvent.click(add());
    expect(queue.mock.calls[0][0].change.after).toBe(JSON.stringify({ value: 200000000000, unit: "psi" }));
    restore({ value: 195000000000, unit: "Pa" });
    // Not 195000000000 beside psi: digits the person never saw beside the unit they chose.
    expect(screen.getByText("New elastic modulus (psi)")).toBeInTheDocument();
    expect(value()).toHaveValue("200000000000");
  });

  it("choosing the unit already shown changes nothing and is not an edit: the entry still follows", async () => {
    const { restore } = await modulusTask();
    changeFormControl(screen.getByTestId("editor-intent-unit"), { target: { value: "Pa" } });
    restore({ value: 195000000000, unit: "Pa" });
    // Not 200000000000 kept and offered: a click that changed nothing does not make the old digits the person's proposal.
    expect(value()).toHaveValue("195000000000");
    expect(add()).toBeDisabled();
  });

  it("Cancel and a new task start from the stored entry again", async () => {
    const { restore } = await modulusTask();
    fireEvent.change(value(), { target: { value: "210000000000" } });
    fireEvent.click(screen.getByTestId("cancel-editor-intent"));
    fireEvent.click(screen.getByTestId("inspector-start-task"));
    changeFormControl(screen.getByTestId("editor-intent-field"), { target: { value: "elastic_modulus.value" } });
    expect(value()).toHaveValue("200000000000");
    restore({ value: 200000, unit: "MPa" });
    expect(value()).toHaveValue("200000");
  });
});

describe("an unedited quantity offers nothing, whatever its stored unit normalizes to", () => {
  it("a stored unit with spaces around it: nothing to Add until the person types or chooses a unit", async () => {
    route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
    const queue = vi.fn<(intent: EditorOperationIntent) => void>();
    render(
      <DisplayUnitsProvider initialPreference="SI">
        <PropertyInspector model={demo({ value: 200000000000, unit: " Pa " })} selection={steel} onQueueIntent={queue} onApplyIntent={async () => true} projectSessionGeneration={1} />
      </DisplayUnitsProvider>
    );
    await waitFor(() => expect(screen.getByTestId("property-unit-catalog-status")).not.toHaveTextContent("loading"));
    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    fireEvent.click(screen.getByTestId("inspector-start-task"));
    changeFormControl(screen.getByTestId("editor-intent-field"), { target: { value: "elastic_modulus.value" } });
    expect(value()).toHaveValue("200000000000");
    // Not the unchanged number with the unit trimmed, offered with nothing typed or chosen.
    expect(add()).toBeDisabled();
    expect(screen.getByTestId("validate-editor-intent-inline")).toBeDisabled();
    expect(screen.getByTestId("apply-editor-intent-inline")).toBeDisabled();
    fireEvent.change(value(), { target: { value: "210000000000" } });
    expect(add()).toBeEnabled();
  });
});

describe("an unedited text entry offers nothing, whatever its stored text normalizes to", () => {
  const node: EntityRef = { type: "node", id: "node:N-110" };
  async function labelTask(stored: string) {
    route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
    const model = demo();
    model.nodes.find((entry) => entry.id === node.id)!.label = stored;
    const queue = vi.fn<(intent: EditorOperationIntent) => void>();
    render(
      <DisplayUnitsProvider initialPreference="SI">
        <PropertyInspector model={model} selection={node} onQueueIntent={queue} onApplyIntent={async () => true} projectSessionGeneration={1} />
      </DisplayUnitsProvider>
    );
    await waitFor(() => expect(screen.getByTestId("property-unit-catalog-status")).not.toHaveTextContent("loading"));
    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    fireEvent.click(screen.getByTestId("inspector-start-task"));
    changeFormControl(screen.getByTestId("editor-intent-field"), { target: { value: "label" } });
    return queue;
  }

  it.each([["an empty label", ""], ["a label with spaces around it", " N "]])("%s: nothing to Add or Apply until the person types", async (_name, stored) => {
    const queue = await labelTask(stored);
    expect(value()).toHaveValue(stored);
    // Not "TBD", nor the trimmed text, offered with nothing typed.
    expect(add()).toBeDisabled();
    expect(screen.getByTestId("apply-editor-intent-inline")).toBeDisabled();
    fireEvent.change(value(), { target: { value: "Low point" } });
    expect(add()).toBeEnabled();
    fireEvent.click(add());
    expect(queue.mock.calls[0][0].change.after).toBe("Low point");
  });
});

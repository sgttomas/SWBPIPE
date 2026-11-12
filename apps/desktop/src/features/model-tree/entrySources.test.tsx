// PRD §5.5; DEL-07-03 AC-001; DEL-07-02 AX-001. A creation form's source starts empty:
// the product supplies none, and the form does not queue without one (the engine
// refuses an empty provenance too). From the second record on the form shows the
// user's own last source, and on New section the user's own last length unit, in
// the field, editable before queueing. Nothing else the form queues changes.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { chooseCompactOption, expectFormControlValue } from "../../test-support/workspaceTestControls";
import { DisplayUnitsProvider } from "../display-units";
import { PropertyInspector } from "./PropertyInspector";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: () => Promise.resolve(route.current)
}));

afterEach(() => { cleanup(); route.current = null; });

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const demo = (): PreviewModel =>
  JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;

const byId = (testId: string) => screen.getByTestId(testId);
const type = (testId: string, value: string) => fireEvent.change(byId(testId), { target: { value } });
const after = (intent: EditorOperationIntent) => JSON.parse(intent.change.after) as Record<string, unknown>;

async function mount(selection: EntityRef) {
  route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  const tree = (model: PreviewModel, projectSessionGeneration: number) => (
    <DisplayUnitsProvider initialPreference="SI">
      <PropertyInspector model={model} selection={selection} onQueueIntent={queue} projectSessionGeneration={projectSessionGeneration} />
    </DisplayUnitsProvider>
  );
  const view = render(tree(demo(), 1));
  await waitFor(() => expect(byId("property-unit-catalog-status")).not.toHaveTextContent("loading"));
  const intents = () => queue.mock.calls.map(([intent]) => intent);
  /** The same project session with a changed model, or a new session (another project). */
  const rerender = (model: PreviewModel, projectSessionGeneration = 1) => view.rerender(tree(model, projectSessionGeneration));
  return { intents, rerender };
}

function withLength(length: string): PreviewModel {
  const model = demo();
  model.project.units = { ...model.project.units, length } as PreviewModel["project"]["units"];
  return model;
}

describe("New section: the source and the length unit are the user's own", () => {
  it("starts with an empty source and refuses an untouched record", async () => {
    await mount({ type: "project", id: "project:invented-demo" });
    expect(byId("create-section-provenance")).toHaveValue("");
    type("create-section-od", "270");
    type("create-section-wall", "9");
    expect(byId("queue-create-section-intent")).toBeDisabled();
    type("create-section-provenance", "   ");
    expect(byId("queue-create-section-intent")).toBeDisabled();
  });

  it("keeps the user's last unit and source for the next section; an edit changes only that record", async () => {
    const { intents } = await mount({ type: "project", id: "project:invented-demo" });
    // The project unit is m; the user chooses mm and types the source.
    expectFormControlValue(byId("create-section-length-unit"), "m");
    chooseCompactOption(byId("create-section-length-unit"), "mm");
    type("create-section-od", "270");
    type("create-section-wall", "9");
    type("create-section-provenance", "source A");
    fireEvent.click(byId("queue-create-section-intent"));
    const first = intents()[0];
    expect(first.change).toMatchObject({ change_kind: "create_section", unit: "mm" });
    expect(after(first)).toEqual({
      id: first.target.ref, name: expect.any(String), section_type: "pipe",
      properties: { outside_diameter: { value: 270, unit: "mm" }, wall_thickness: { value: 9, unit: "mm" } }, provenance: "source A"
    });

    // The second record shows the first's unit and source; quantities start empty.
    expectFormControlValue(byId("create-section-length-unit"), "mm");
    expect(byId("create-section-provenance")).toHaveValue("source A");
    expect(byId("create-section-od")).toHaveValue("");
    type("create-section-od", "220");
    type("create-section-wall", "8");
    type("create-section-provenance", "source B");
    fireEvent.click(byId("queue-create-section-intent"));
    const second = intents()[1];
    expect(after(second)).toMatchObject({ properties: { outside_diameter: { value: 220, unit: "mm" } }, provenance: "source B" });
    // The first record queued is unchanged by the edit.
    expect(after(intents()[0]).provenance).toBe("source A");
    expect(Object.keys(after(second)).sort()).toEqual(Object.keys(after(first)).sort());

    // The third shows the user's latest source.
    expect(byId("create-section-provenance")).toHaveValue("source B");
  });
});

describe("New material and New support: the source is the user's own", () => {
  it("material: empty first, refused untouched, kept for the next record", async () => {
    const { intents } = await mount({ type: "project", id: "project:invented-demo" });
    expect(byId("create-material-provenance")).toHaveValue("");
    type("create-material-elastic", "200000");
    type("create-material-shear", "80000");
    expect(byId("queue-create-material-intent")).toBeDisabled();
    type("create-material-provenance", "material source");
    fireEvent.click(byId("queue-create-material-intent"));
    expect(after(intents()[0]).provenance).toBe("material source");
    expect(byId("create-material-provenance")).toHaveValue("material source");
    expect(byId("create-material-elastic")).toHaveValue("");
  });

  it("support: empty first, refused untouched, kept for the next record", async () => {
    const { intents } = await mount({ type: "node", id: "node:N-110" });
    expect(byId("create-support-provenance")).toHaveValue("");
    expect(byId("queue-create-support-intent")).toBeDisabled();
    type("create-support-provenance", "support source");
    fireEvent.click(byId("queue-create-support-intent"));
    expect(after(intents()[0]).provenance).toBe("support source");
    expect(byId("create-support-provenance")).toHaveValue("support source");
  });

  it("component: the geometry source and the provenance start empty", async () => {
    await mount({ type: "node", id: "node:N-110" });
    expect(byId("create-component-source")).toHaveValue("");
    expect(byId("create-component-provenance")).toHaveValue("");
  });
});

describe("New section: a unit the user did not change follows the project units (DEL-02-02 REQ-022)", () => {
  it("an untouched unit follows a mid-session change of the project length unit, before and after a queue", async () => {
    const { intents, rerender } = await mount({ type: "project", id: "project:invented-demo" });
    expectFormControlValue(byId("create-section-length-unit"), "m");
    rerender(withLength("mm"));
    expectFormControlValue(byId("create-section-length-unit"), "mm");
    type("create-section-od", "270");
    type("create-section-wall", "9");
    type("create-section-provenance", "source A");
    fireEvent.click(byId("queue-create-section-intent"));
    expect(intents()[0].change.unit).toBe("mm");
    rerender(withLength("in"));
    expectFormControlValue(byId("create-section-length-unit"), "in");
  });

  it("a unit the user changed is kept for the next section, also across a project-unit change", async () => {
    const { intents, rerender } = await mount({ type: "project", id: "project:invented-demo" });
    chooseCompactOption(byId("create-section-length-unit"), "mm");
    type("create-section-od", "270");
    type("create-section-wall", "9");
    type("create-section-provenance", "source A");
    fireEvent.click(byId("queue-create-section-intent"));
    expect(intents()[0].change.unit).toBe("mm");
    rerender(withLength("in"));
    expectFormControlValue(byId("create-section-length-unit"), "mm");
  });
});

describe("the user's text never crosses projects", () => {
  it("a new project session starts every inspector creation form empty and the section unit in the new project's unit", async () => {
    const { rerender } = await mount({ type: "node", id: "node:N-110" });
    chooseCompactOption(byId("create-section-length-unit"), "mm");
    type("create-section-od", "270");
    type("create-section-wall", "9");
    type("create-section-provenance", "section source");
    fireEvent.click(byId("queue-create-section-intent"));
    type("create-material-provenance", "material source");
    type("create-support-provenance", "support source");
    type("create-component-source", "geometry source");
    type("create-component-provenance", "component source");
    expect(byId("create-section-provenance")).toHaveValue("section source");

    rerender(withLength("m"), 2);
    for (const field of ["create-section-provenance", "create-material-provenance", "create-support-provenance", "create-component-source", "create-component-provenance"]) {
      expect(byId(field)).toHaveValue("");
    }
    expectFormControlValue(byId("create-section-length-unit"), "m");
  });
});

describe("material and component units the user did not change follow the project units (DEL-02-02 REQ-022)", () => {
  function withUnits(units: Record<string, string>): PreviewModel {
    const model = demo();
    model.project.units = { ...model.project.units, ...units } as PreviewModel["project"]["units"];
    return model;
  }

  it("material: an untouched modulus unit follows a mid-session change of the project pressure unit; a changed one stays", async () => {
    // The modulus unit's project key is `pressure` (PropertyInspector stressUnit()).
    const { rerender } = await mount({ type: "project", id: "project:invented-demo" });
    expectFormControlValue(byId("create-material-stress-unit"), "Pa");
    rerender(withUnits({ pressure: "MPa" }));
    expectFormControlValue(byId("create-material-stress-unit"), "MPa");
    chooseCompactOption(byId("create-material-stress-unit"), "Pa");
    rerender(withUnits({ pressure: "MPa" }));
    expectFormControlValue(byId("create-material-stress-unit"), "Pa");
  });

  it("component: an untouched unit follows a mid-session change; a changed one stays", async () => {
    const { rerender } = await mount({ type: "node", id: "node:N-110" });
    expectFormControlValue(byId("create-component-radius-unit"), "m");
    rerender(withUnits({ length: "mm" }));
    expectFormControlValue(byId("create-component-radius-unit"), "mm");
    chooseCompactOption(byId("create-component-radius-unit"), "m");
    rerender(withUnits({ length: "in" }));
    expectFormControlValue(byId("create-component-radius-unit"), "m");
  });
});

// A typed number when the project unit moves (decided by the SWBPIPE Agent 0; DEL-02-02
// AC-011): every number typed beside a unit the user did not change is cleared when that
// unit moves, nothing is converted, and the form names the unit, from and to; the sources
// and every other entry are kept. A unit the user changed does not move; a project-unit
// change that leaves a form's unit as it was clears nothing.
const cleared = (label: string, from: string, to: string) =>
  `The ${label} unit changed from ${from} to ${to} with the project units; the values beside it were cleared.`;

function withUnits(units: Record<string, string>): PreviewModel {
  const model = demo();
  model.project.units = { ...model.project.units, ...units } as PreviewModel["project"]["units"];
  return model;
}

describe("A typed number when the project unit moves (inspector)", () => {
  it("section: the diameter and wall are cleared and the form says so; the name and source are kept", async () => {
    const { rerender } = await mount({ type: "project", id: "project:invented-demo" });
    type("create-section-name", "Typed section");
    type("create-section-od", "100");
    type("create-section-wall", "5");
    type("create-section-provenance", "section source");
    rerender(withUnits({ length: "mm" }));
    expectFormControlValue(byId("create-section-length-unit"), "mm");
    expect(byId("create-section-od")).toHaveValue("");
    expect(byId("create-section-wall")).toHaveValue("");
    expect(byId("create-section-name")).toHaveValue("Typed section");
    expect(byId("create-section-provenance")).toHaveValue("section source");
    expect(byId("create-section-unit-cleared")).toHaveTextContent(cleared("length", "m", "mm"));
  });

  it("section: a unit the user changed does not move, and a change that leaves the length unit clears nothing", async () => {
    const { rerender } = await mount({ type: "project", id: "project:invented-demo" });
    type("create-section-od", "100");
    rerender(withUnits({ pressure: "MPa" }));
    expect(byId("create-section-od")).toHaveValue("100");
    expect(screen.queryByTestId("create-section-unit-cleared")).toBeNull();
    chooseCompactOption(byId("create-section-length-unit"), "mm");
    type("create-section-od", "270");
    rerender(withUnits({ length: "in" }));
    expectFormControlValue(byId("create-section-length-unit"), "mm");
    expect(byId("create-section-od")).toHaveValue("270");
    expect(screen.queryByTestId("create-section-unit-cleared")).toBeNull();
  });

  it("section: the user's own change of unit clears nothing; it is not a move with the project units", async () => {
    await mount({ type: "project", id: "project:invented-demo" });
    type("create-section-od", "100");
    type("create-section-wall", "5");
    chooseCompactOption(byId("create-section-length-unit"), "mm");
    expect(byId("create-section-od")).toHaveValue("100");
    expect(byId("create-section-wall")).toHaveValue("5");
    expect(screen.queryByTestId("create-section-unit-cleared")).toBeNull();
  });

  it("material: each unit that moves clears only the values beside it", async () => {
    const { rerender } = await mount({ type: "project", id: "project:invented-demo" });
    expectFormControlValue(byId("create-material-thermal-unit"), "1/degC");
    type("create-material-label", "Typed material");
    type("create-material-elastic", "200000");
    type("create-material-shear", "80000");
    type("create-material-thermal", "0.000012");
    type("create-material-provenance", "material source");
    rerender(withUnits({ pressure: "MPa" }));
    expect(byId("create-material-elastic")).toHaveValue("");
    expect(byId("create-material-shear")).toHaveValue("");
    expect(byId("create-material-thermal")).toHaveValue("0.000012");
    expect(byId("create-material-unit-cleared")).toHaveTextContent(cleared("modulus", "Pa", "MPa"));
    rerender(withUnits({ pressure: "MPa", temperature: "K" }));
    expect(byId("create-material-thermal")).toHaveValue("");
    expect(byId("create-material-unit-cleared")).toHaveTextContent(cleared("thermal expansion", "1/degC", "1/K"));
    expect(byId("create-material-label")).toHaveValue("Typed material");
    expect(byId("create-material-provenance")).toHaveValue("material source");
  });

  it("support: the stiffness is cleared and the form says so; the source is kept", async () => {
    const { rerender } = await mount({ type: "node", id: "node:N-110" });
    type("create-support-stiffness", "1500");
    type("create-support-provenance", "support source");
    rerender(withUnits({ linear_stiffness: "kN/m" }));
    expectFormControlValue(byId("create-support-stiffness-unit"), "kN/m");
    expect(byId("create-support-stiffness")).toHaveValue("");
    expect(byId("create-support-provenance")).toHaveValue("support source");
    expect(byId("create-support-unit-cleared")).toHaveTextContent(cleared("stiffness", "N/m", "kN/m"));
  });

  it("component: the values beside the moved unit are cleared; the angle and the sources are kept", async () => {
    const { rerender } = await mount({ type: "node", id: "node:N-110" });
    type("create-component-radius", "0.45");
    type("create-component-angle", "1.5708");
    type("create-component-source", "geometry source");
    type("create-component-provenance", "component source");
    rerender(withUnits({ length: "mm" }));
    expectFormControlValue(byId("create-component-radius-unit"), "mm");
    expect(byId("create-component-radius")).toHaveValue("");
    expect(byId("create-component-angle")).toHaveValue("1.5708");
    expect(byId("create-component-source")).toHaveValue("geometry source");
    expect(byId("create-component-provenance")).toHaveValue("component source");
    expect(byId("create-component-unit-cleared")).toHaveTextContent(cleared("length", "m", "mm"));
  });
});

// DEL-02-02 REQ-022 / AC-011: a unit the user chose is where the next record starts.
describe("the next record starts in the unit the user chose (inspector)", () => {
  it("material: the chosen modulus and expansion units carry to the next material", async () => {
    const { intents } = await mount({ type: "project", id: "project:invented-demo" });
    chooseCompactOption(byId("create-material-stress-unit"), "MPa");
    chooseCompactOption(byId("create-material-thermal-unit"), "1/K");
    type("create-material-elastic", "200000");
    type("create-material-shear", "80000");
    type("create-material-provenance", "material source");
    fireEvent.click(byId("queue-create-material-intent"));
    expect(intents()[0].change.unit).toBe("MPa");
    expectFormControlValue(byId("create-material-stress-unit"), "MPa");
    expectFormControlValue(byId("create-material-thermal-unit"), "1/K");
    expect(byId("create-material-elastic")).toHaveValue("");
  });

  it("support: the chosen stiffness unit carries to the next support", async () => {
    // The project declares kN/m; the user chooses the catalog's N/m.
    const { intents, rerender } = await mount({ type: "node", id: "node:N-110" });
    rerender(withUnits({ linear_stiffness: "kN/m" }));
    chooseCompactOption(byId("create-support-stiffness-unit"), "N/m");
    type("create-support-stiffness", "1500");
    type("create-support-provenance", "support source");
    fireEvent.click(byId("queue-create-support-intent"));
    expect(after(intents()[0]).properties).toEqual({ linear_stiffness: { value: 1500, unit: "N/m" } });
    expectFormControlValue(byId("create-support-stiffness-unit"), "N/m");
  });

  it("component: the chosen radius unit carries to the next component", async () => {
    const { intents } = await mount({ type: "node", id: "node:N-110" });
    chooseCompactOption(byId("create-component-radius-unit"), "mm");
    type("create-component-radius", "450");
    type("create-component-angle", "1.5708");
    type("create-component-source", "geometry source");
    type("create-component-provenance", "component source");
    await waitFor(() => expect(byId("queue-create-component-intent")).toBeEnabled());
    fireEvent.click(byId("queue-create-component-intent"));
    expect(after(intents()[0]).geometry).toMatchObject({ bend_radius: { value: 450, unit: "mm" } });
    expectFormControlValue(byId("create-component-radius-unit"), "mm");
    expect(byId("create-component-radius")).toHaveValue("");
  });
});

// DEL-07-03 AC-001: a record without a source is refused by name, in place.
describe("a missing source is named beside Queue (inspector)", () => {
  const reason = (record: string, named: string) => `Not queued: enter the ${record}'s ${named}. A record without its source is refused.`;
  it.each([
    ["section", "create-section", "queue-create-section-intent", "provenance"],
    ["material", "create-material", "queue-create-material-intent", "provenance"],
    ["support", "create-support", "queue-create-support-intent", "provenance"]
  ])("%s", async (record, prefix, button, named) => {
    await mount(record === "support" ? { type: "node", id: "node:N-110" } : { type: "project", id: "project:invented-demo" });
    expect(byId(`${prefix}-source-reason`)).toHaveTextContent(reason(record, named));
    expect(byId(button)).toHaveAttribute("aria-describedby", `${prefix}-source-reason`);
    type(`${prefix}-provenance`, "typed source");
    expect(screen.queryByTestId(`${prefix}-source-reason`)).toBeNull();
    expect(byId(button)).not.toHaveAttribute("aria-describedby");
  });

  it("component: names the geometry source and the provenance, then each that remains", async () => {
    await mount({ type: "node", id: "node:N-110" });
    expect(byId("create-component-source-reason")).toHaveTextContent(reason("component", "geometry source and provenance"));
    expect(byId("queue-create-component-intent")).toHaveAttribute("aria-describedby", "create-component-source-reason");
    type("create-component-provenance", "component source");
    expect(byId("create-component-source-reason")).toHaveTextContent(reason("component", "geometry source"));
    type("create-component-source", "geometry source");
    expect(screen.queryByTestId("create-component-source-reason")).toBeNull();
  });
});

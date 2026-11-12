// The user's text never crosses projects (PRD §5.5; DEL-07-03 AC-001): the viewport's
// New component form keeps the user's own sources within a project session and starts
// empty in the next one.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { chooseCompactOption, expectFormControlValue } from "../../test-support/workspaceTestControls";
import { DisplayUnitsProvider } from "../display-units";
import { modelIndexFor } from "../workspace/modelIndex";
import { PipeViewport } from "./PipeViewport";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: async () => route.current
}));

afterEach(() => { cleanup(); route.current = null; });

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const demo = (): PreviewModel =>
  JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
const byId = (testId: string) => screen.getByTestId(testId);
const type = (testId: string, value: string) => fireEvent.change(byId(testId), { target: { value } });

describe("viewport New component sources", () => {
  it("start empty, stay within a project session and start empty in the next", () => {
    const first = demo();
    const tree = (model: PreviewModel, sessionGeneration: number, revision: number) => (
      <DisplayUnitsProvider initialPreference="SI">
        <PipeViewport model={model} modelIndex={modelIndexFor(model, sessionGeneration, revision)} selection={{ type: "node", id: "node:N-110" }} onSelect={vi.fn()} onQueueIntent={vi.fn()} />
      </DisplayUnitsProvider>
    );
    const view = render(tree(first, 1, 0));
    expect(byId("viewport-create-component-source")).toHaveValue("");
    expect(byId("viewport-create-component-provenance")).toHaveValue("");
    type("viewport-create-component-source", "geometry source");
    type("viewport-create-component-provenance", "component source");

    // An edit within the same project session keeps the user's text.
    const edited = structuredClone(first);
    view.rerender(tree(edited, 1, 1));
    expect(byId("viewport-create-component-provenance")).toHaveValue("component source");

    // Another project: nothing of the user's text carries.
    const next = demo();
    view.rerender(tree(next, 2, 0));
    expect(byId("viewport-create-component-source")).toHaveValue("");
    expect(byId("viewport-create-component-provenance")).toHaveValue("");
  });
});

describe("viewport node and route drafts", () => {
  const tree = (model: PreviewModel, sessionGeneration: number, revision: number) => (
    <DisplayUnitsProvider initialPreference="SI">
      <PipeViewport model={model} modelIndex={modelIndexFor(model, sessionGeneration, revision)} selection={{ type: "node", id: "node:N-110" }} onSelect={vi.fn()} onQueueIntent={vi.fn()} onArmCreationTool={vi.fn()} />
    </DisplayUnitsProvider>
  );
  const routeEndNew = () => document.querySelector<HTMLInputElement>('input[name="route-end-mode"][value="new"]')!;

  it("the node draft is kept within a project session, also across a model change, and starts empty in the next", () => {
    const first = demo();
    const view = render(tree(first, 1, 0));
    type("viewport-create-node-id", "node:typed");
    type("viewport-create-node-label", "Typed node");
    type("viewport-create-node-x", "1.5");
    type("viewport-create-node-provenance", "node source");

    // An apply within the same project session changes the model; the node draft stays.
    view.rerender(tree(structuredClone(first), 1, 1));
    expect(byId("viewport-create-node-provenance")).toHaveValue("node source");
    expect(byId("viewport-create-node-x")).toHaveValue("1.5");

    view.rerender(tree(demo(), 2, 0));
    for (const field of ["viewport-create-node-id", "viewport-create-node-label", "viewport-create-node-x", "viewport-create-node-provenance"]) {
      expect(byId(field)).toHaveValue("");
    }
  });

  it("the route drafts are kept within a project session and start empty in the next", () => {
    const first = demo();
    const view = render(tree(first, 1, 0));
    type("viewport-create-pipe-id", "pipe:typed");
    type("viewport-create-pipe-provenance", "pipe source");
    fireEvent.click(routeEndNew());
    type("viewport-route-end-provenance", "route end source");

    // The same project session, the model unchanged: the user's text stays.
    view.rerender(tree(first, 1, 0));
    expect(byId("viewport-create-pipe-provenance")).toHaveValue("pipe source");
    expect(byId("viewport-route-end-provenance")).toHaveValue("route end source");

    view.rerender(tree(demo(), 2, 0));
    expect(byId("viewport-create-pipe-id")).toHaveValue("");
    expect(byId("viewport-create-pipe-provenance")).toHaveValue("");
    expect(routeEndNew().checked).toBe(false);
    expect((screen.queryByTestId("viewport-route-end-provenance") as HTMLInputElement | null)?.value ?? "").toBe("");
  });
});

// DEL-02-02 REQ-022: a unit the user did not change follows the project units; one the user changed is kept.
describe("viewport New node unit", () => {
  const withLength = (length: string): PreviewModel => {
    const model = demo();
    model.project.units = { ...model.project.units, length } as PreviewModel["project"]["units"];
    return model;
  };
  const tree = (model: PreviewModel, queue = vi.fn()) => (
    <DisplayUnitsProvider initialPreference="SI">
      <PipeViewport model={model} modelIndex={modelIndexFor(model, 1, 0)} selection={{ type: "node", id: "node:N-110" }} onSelect={vi.fn()} onQueueIntent={queue} onArmCreationTool={vi.fn()} />
    </DisplayUnitsProvider>
  );
  const fillNode = (id: string) => {
    type("viewport-create-node-id", id);
    type("viewport-create-node-label", "Typed node");
    type("viewport-create-node-x", "1");
    type("viewport-create-node-y", "0");
    type("viewport-create-node-z", "0");
    type("viewport-create-node-provenance", "node source");
  };

  it("an untouched unit follows a mid-session change of the project length unit, before and after a queue", async () => {
    route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
    const queue = vi.fn();
    const view = render(tree(withLength("m"), queue));
    expectFormControlValue(byId("viewport-create-node-unit"), "m");
    view.rerender(tree(withLength("mm"), queue));
    expectFormControlValue(byId("viewport-create-node-unit"), "mm");
    fillNode("node:typed-1");
    await waitFor(() => expect(byId("queue-explicit-node-intent")).toBeEnabled());
    fireEvent.click(byId("queue-explicit-node-intent"));
    expect(queue).toHaveBeenCalledOnce();
    expect(queue.mock.calls[0][0].change.unit).toBe("mm");
    view.rerender(tree(withLength("in"), queue));
    expectFormControlValue(byId("viewport-create-node-unit"), "in");
  });

  it("a unit the user changed is kept for the next node, also across a project-unit change", async () => {
    route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
    const queue = vi.fn();
    const view = render(tree(withLength("m"), queue));
    await waitFor(() => expect(byId("viewport-create-node-unit-basis")).not.toHaveTextContent(/unavailable|pending/i));
    chooseCompactOption(byId("viewport-create-node-unit"), "mm");
    fillNode("node:typed-1");
    await waitFor(() => expect(byId("queue-explicit-node-intent")).toBeEnabled());
    fireEvent.click(byId("queue-explicit-node-intent"));
    expect(queue.mock.calls[0][0].change.unit).toBe("mm");
    expectFormControlValue(byId("viewport-create-node-unit"), "mm");
    expect(byId("viewport-create-node-x")).toHaveValue("");
    view.rerender(tree(withLength("in"), queue));
    expectFormControlValue(byId("viewport-create-node-unit"), "mm");
  });
});

describe("viewport New component units", () => {
  it("an untouched unit follows a mid-session change of the project length unit; a changed one stays", async () => {
    route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
    const withLength = (length: string): PreviewModel => {
      const model = demo();
      model.project.units = { ...model.project.units, length } as PreviewModel["project"]["units"];
      return model;
    };
    const tree = (model: PreviewModel) => (
      <DisplayUnitsProvider initialPreference="SI">
        <PipeViewport model={model} modelIndex={modelIndexFor(model, 1, 0)} selection={{ type: "node", id: "node:N-110" }} onSelect={vi.fn()} onQueueIntent={vi.fn()} onArmCreationTool={vi.fn()} />
      </DisplayUnitsProvider>
    );
    const view = render(tree(withLength("m")));
    await waitFor(() => expect(byId("viewport-create-node-unit-basis")).not.toHaveTextContent(/unavailable|pending/i));
    expectFormControlValue(byId("viewport-create-component-radius-unit"), "m");
    view.rerender(tree(withLength("mm")));
    expectFormControlValue(byId("viewport-create-component-radius-unit"), "mm");
    chooseCompactOption(byId("viewport-create-component-radius-unit"), "m");
    view.rerender(tree(withLength("in")));
    expectFormControlValue(byId("viewport-create-component-radius-unit"), "m");
  });
});

// A typed number when the project unit moves (decided by the SWBPIPE Agent 0; DEL-02-02
// AC-011): the numbers beside a unit the user did not change are cleared when it moves,
// and the form says so; everything else typed is kept.
describe("A typed number when the project unit moves (viewport)", () => {
  const withUnits = (units: Record<string, string>): PreviewModel => {
    const model = demo();
    model.project.units = { ...model.project.units, ...units } as PreviewModel["project"]["units"];
    return model;
  };
  const tree = (model: PreviewModel) => (
    <DisplayUnitsProvider initialPreference="SI">
      <PipeViewport model={model} modelIndex={modelIndexFor(model, 1, 0)} selection={{ type: "node", id: "node:N-110" }} onSelect={vi.fn()} onQueueIntent={vi.fn()} onArmCreationTool={vi.fn()} />
    </DisplayUnitsProvider>
  );
  const cleared = (label: string, from: string, to: string) =>
    `The ${label} unit changed from ${from} to ${to} with the project units; the values beside it were cleared.`;

  it("node: X, Y and Z are cleared and the form says so; the id, label and source are kept", () => {
    const view = render(tree(withUnits({ length: "m" })));
    type("viewport-create-node-id", "node:typed");
    type("viewport-create-node-label", "Typed node");
    type("viewport-create-node-x", "1.5");
    type("viewport-create-node-y", "2");
    type("viewport-create-node-z", "-3");
    type("viewport-create-node-provenance", "node source");
    view.rerender(tree(withUnits({ length: "mm" })));
    expectFormControlValue(byId("viewport-create-node-unit"), "mm");
    for (const axis of ["x", "y", "z"]) expect(byId(`viewport-create-node-${axis}`)).toHaveValue("");
    expect(byId("viewport-create-node-id")).toHaveValue("node:typed");
    expect(byId("viewport-create-node-label")).toHaveValue("Typed node");
    expect(byId("viewport-create-node-provenance")).toHaveValue("node source");
    expect(byId("viewport-create-node-unit-cleared")).toHaveTextContent(cleared("coordinate", "m", "mm"));
  });

  it("node: a change that leaves the length unit clears nothing; a unit the user changed does not move", async () => {
    route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
    const view = render(tree(withUnits({ length: "m" })));
    type("viewport-create-node-x", "1.5");
    view.rerender(tree(withUnits({ length: "m", pressure: "MPa" })));
    expect(byId("viewport-create-node-x")).toHaveValue("1.5");
    expect(screen.queryByTestId("viewport-create-node-unit-cleared")).toBeNull();
    await waitFor(() => expect(byId("viewport-create-node-unit-basis")).not.toHaveTextContent(/unavailable|pending/i));
    chooseCompactOption(byId("viewport-create-node-unit"), "mm");
    type("viewport-create-node-z", "4");
    view.rerender(tree(withUnits({ length: "in" })));
    expectFormControlValue(byId("viewport-create-node-unit"), "mm");
    expect(byId("viewport-create-node-z")).toHaveValue("4");
    expect(screen.queryByTestId("viewport-create-node-unit-cleared")).toBeNull();
  });

  it("component: the radius is cleared and the form says so; the angle and the sources are kept", () => {
    const view = render(tree(withUnits({ length: "m" })));
    type("viewport-create-component-radius", "0.45");
    type("viewport-create-component-angle", "1.5708");
    type("viewport-create-component-source", "geometry source");
    type("viewport-create-component-provenance", "component source");
    view.rerender(tree(withUnits({ length: "mm" })));
    expectFormControlValue(byId("viewport-create-component-radius-unit"), "mm");
    expect(byId("viewport-create-component-radius")).toHaveValue("");
    expect(byId("viewport-create-component-angle")).toHaveValue("1.5708");
    expect(byId("viewport-create-component-source")).toHaveValue("geometry source");
    expect(byId("viewport-create-component-provenance")).toHaveValue("component source");
    expect(byId("viewport-create-component-unit-cleared")).toHaveTextContent(cleared("length", "m", "mm"));
  });
});

describe("viewport New component: the chosen unit and the named source", () => {
  const tree = (model: PreviewModel, queue = vi.fn()) => (
    <DisplayUnitsProvider initialPreference="SI">
      <PipeViewport model={model} modelIndex={modelIndexFor(model, 1, 0)} selection={{ type: "node", id: "node:N-110" }} onSelect={vi.fn()} onQueueIntent={queue} onArmCreationTool={vi.fn()} />
    </DisplayUnitsProvider>
  );

  it("the chosen radius unit carries to the next component", async () => {
    route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
    const queue = vi.fn();
    render(tree(demo(), queue));
    await waitFor(() => expect(byId("viewport-create-node-unit-basis")).not.toHaveTextContent(/unavailable|pending/i));
    chooseCompactOption(byId("viewport-create-component-radius-unit"), "mm");
    type("viewport-create-component-radius", "450");
    type("viewport-create-component-angle", "1.5708");
    type("viewport-create-component-source", "geometry source");
    type("viewport-create-component-provenance", "component source");
    await waitFor(() => expect(byId("queue-explicit-component-intent")).toBeEnabled());
    fireEvent.click(byId("queue-explicit-component-intent"));
    expect(JSON.parse(queue.mock.calls[0][0].change.after).geometry).toMatchObject({ bend_radius: { value: 450, unit: "mm" } });
    expectFormControlValue(byId("viewport-create-component-radius-unit"), "mm");
    expect(byId("viewport-create-component-radius")).toHaveValue("");
  });

  it("a missing source is named beside Queue", () => {
    render(tree(demo()));
    const reason = (named: string) => `Not queued: enter the component's ${named}. A record without its source is refused.`;
    expect(byId("viewport-create-component-source-reason")).toHaveTextContent(reason("geometry source and provenance"));
    expect(byId("queue-explicit-component-intent")).toHaveAttribute("aria-describedby", "viewport-create-component-source-reason");
    type("viewport-create-component-source", "geometry source");
    expect(byId("viewport-create-component-source-reason")).toHaveTextContent(reason("provenance"));
    type("viewport-create-component-provenance", "component source");
    expect(screen.queryByTestId("viewport-create-component-source-reason")).toBeNull();
  });
});

// The independent review, finding 1: typed input in the route's direction-entry aid and in a
// picker's search must not survive a project change, and the aid starts afresh on Cancel.
describe("viewport direction entry and picker search", () => {
  const tree = (model: PreviewModel, sessionGeneration: number) => (
    <DisplayUnitsProvider initialPreference="SI">
      <PipeViewport model={model} modelIndex={modelIndexFor(model, sessionGeneration, 0)} armedCreationTool="pipe" selection={{ type: "node", id: "node:N-110" }} onSelect={vi.fn()} onQueueIntent={vi.fn()} onArmCreationTool={vi.fn()} />
    </DisplayUnitsProvider>
  );
  const routeEndNew = () => fireEvent.click(document.querySelector<HTMLInputElement>('input[name="route-end-mode"][value="new"]')!);
  const length = () => screen.getByLabelText("Route end: Length along the element") as HTMLInputElement;
  const search = (testId: string) => within(byId(testId)).getByRole("combobox") as HTMLInputElement;

  it("a new project session starts the direction entry and the picker search empty", () => {
    const view = render(tree(demo(), 1));
    routeEndNew();
    fireEvent.change(length(), { target: { value: "2.5" } });
    fireEvent.change(search("viewport-create-pipe-material"), { target: { value: "steel" } });
    view.rerender(tree(demo(), 1));
    expect(length().value).toBe("2.5");
    expect(search("viewport-create-pipe-material").value).toBe("steel");
    view.rerender(tree(demo(), 2));
    routeEndNew();
    expect(length().value).toBe("");
    expect(search("viewport-create-pipe-material").value).toBe("");
  });

  it("Cancel route starts the direction entry afresh", () => {
    render(tree(demo(), 1));
    routeEndNew();
    fireEvent.change(length(), { target: { value: "2.5" } });
    fireEvent.click(byId("cancel-pipe-draft"));
    routeEndNew();
    expect(length().value).toBe("");
  });
});

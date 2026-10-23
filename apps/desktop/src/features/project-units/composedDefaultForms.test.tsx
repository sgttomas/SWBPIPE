import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { SI_PROJECT_UNITS, US_CUSTOMARY_PROJECT_UNITS, buildBlankLocalModelDocument, type ProjectUnitMap } from "../../services/projectService";
import { computeModelHash } from "../../services/hashService";
import { validateModelOperation } from "../../services/operationService";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { changeFormControl, expectFormControlValue } from "../../test-support/workspaceTestControls";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { LoadCaseManagerPanel } from "../load-cases/LoadCaseManagerPanel";
import { PropertyInspector } from "../model-tree/PropertyInspector";

// W-17 slice (a) §2.4 and §4: the forms' composed defaults against the catalog,
// and a rotational imposed displacement on a new blank document. Unit validation
// and operation validation run in the real core engine (wasm).
const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null, pending: null as Promise<UnitCatalogRoute> | null }));
vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: async () => route.pending ? await route.pending : route.current
}));
const CATALOG: UnitCatalogRoute = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
const PREVIEW: UnitCatalogRoute = { route: "unavailable_browser_preview", diagnostic: "UNIT-CATALOG-DESKTOP-ONLY: fixture" };

afterEach(() => { cleanup(); route.current = null; route.pending = null; });

/** A new blank document with invented entities: every value is synthetic test input. */
function blankWith(units: ProjectUnitMap): PreviewModel {
  const model = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"), units);
  model.nodes = [
    { id: "node:A", label: "Node A", position: { x: 0, y: 0, z: 0 }, provenance: "invented node" },
    { id: "node:B", label: "Node B", position: { x: 1, y: 0, z: 0 }, provenance: "invented node" }
  ];
  model.pipe_segments = [{ id: "pipe:AB", label: "Pipe AB", from: "node:A", to: "node:B", section: {}, material: "material:X", provenance: "invented pipe" }];
  model.supports = [{ id: "support:A", label: "Support A", node: "node:A", restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"], provenance: "invented support" }];
  model.load_cases = [{ id: "load:L1", label: "Invented case", kind: "primitive_user_load", status: "draft", provenance: "invented case", primitive_loads: [] }];
  return model;
}

const byId = (testId: string) => screen.getByTestId(testId);
const choose = (testId: string, value: string) => fireEvent.change(byId(testId), { target: { value } });

function mountLoads(model: PreviewModel) {
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  render(<LoadCaseManagerPanel model={model} onQueueIntent={queue} onSelect={vi.fn()} selection={{ type: "project", id: model.project.id }} />);
  return queue;
}

function mountInspector(model: PreviewModel) {
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector model={model} selection={{ type: "node", id: "node:B" }} onQueueIntent={queue} /></DisplayUnitsProvider>);
  return queue;
}

describe("load form composed defaults", () => {
  it("in with N: the distributed force starts unselected with the notice, and queues once a catalog unit is chosen", async () => {
    route.current = CATALOG;
    const queue = mountLoads(blankWith({ ...SI_PROJECT_UNITS, length: "in" }));
    await waitFor(() => expect(byId("load-manager-create-primitive-unit").querySelectorAll("option").length).toBeGreaterThan(1));
    choose("load-manager-create-primitive-category", "distributed_force");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("");
    expect(byId("load-manager-create-primitive-unit-notice")).toHaveTextContent("N/in is not a catalog unit; choose one");
    expect(byId("load-manager-create-primitive-preview")).toHaveTextContent("N/in is not a catalog unit; choose one");
    expect(byId("queue-create-primitive-intent")).toBeDisabled();
    const offered = [...byId("load-manager-create-primitive-unit").querySelectorAll("option")].map((option) => option.value);
    expect(offered[0]).toBe("");
    expect(offered).not.toContain("N/in");
    choose("load-manager-create-primitive-unit", "lbf/in");
    expect(screen.queryByTestId("load-manager-create-primitive-unit-notice")).not.toBeInTheDocument();
    await waitFor(() => expect(byId("queue-create-primitive-intent")).toBeEnabled());
    fireEvent.click(byId("queue-create-primitive-intent"));
    expect(queue.mock.calls.at(-1)![0].change).toMatchObject({ unit: "lbf/in", dimension: "force_per_length" });
  });

  it("mm with N: the distributed default is N/mm and the moment default is the moment key, not N*mm", async () => {
    route.current = CATALOG;
    mountLoads(blankWith({ ...SI_PROJECT_UNITS, length: "mm" }));
    await waitFor(() => expect(byId("load-manager-create-primitive-unit").querySelectorAll("option").length).toBeGreaterThan(1));
    choose("load-manager-create-primitive-category", "distributed_force");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("N/mm");
    choose("load-manager-create-primitive-category", "concentrated_moment");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("N*m");
    expect(screen.queryByTestId("load-manager-create-primitive-unit-notice")).not.toBeInTheDocument();
  });

  it("a moment drafted in the moment key is accepted by the core applier", async () => {
    route.current = PREVIEW;
    const model = blankWith({ ...SI_PROJECT_UNITS, length: "mm" });
    const queue = mountLoads(model);
    choose("load-manager-create-primitive-category", "concentrated_moment");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("N*m");
    await waitFor(() => expect(byId("queue-create-primitive-intent")).toBeEnabled());
    fireEvent.click(byId("queue-create-primitive-intent"));
    const intent = queue.mock.calls.at(-1)![0];
    expect(intent.change).toMatchObject({ unit: "N*m", dimension: "moment" });
    const outcome = await validateModelOperation(model, intent, await computeModelHash(model));
    expect(outcome.diagnostics.filter((diagnostic) => diagnostic.severity === "blocking")).toEqual([]);
  });

  it("without a moment key, a catalog composition is the default", async () => {
    route.current = CATALOG;
    const model = blankWith(SI_PROJECT_UNITS);
    delete model.project.units.moment;
    mountLoads(model);
    await waitFor(() => expect(byId("load-manager-create-primitive-unit").querySelectorAll("option").length).toBeGreaterThan(1));
    choose("load-manager-create-primitive-category", "concentrated_moment");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("N*m");
    expect(screen.queryByTestId("load-manager-create-primitive-unit-notice")).not.toBeInTheDocument();
  });

  it("without a moment key, a non-catalog composition gives no default and the notice", async () => {
    route.current = CATALOG;
    const model = blankWith({ ...SI_PROJECT_UNITS, length: "mm" });
    delete model.project.units.moment;
    const queue = mountLoads(model);
    await waitFor(() => expect(byId("load-manager-create-primitive-unit").querySelectorAll("option").length).toBeGreaterThan(1));
    choose("load-manager-create-primitive-category", "concentrated_moment");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("");
    expect(byId("load-manager-create-primitive-unit-notice")).toHaveTextContent("N*mm is not a catalog unit; choose one");
    expect(byId("queue-create-primitive-intent")).toBeDisabled();
    choose("load-manager-create-primitive-unit", "lbf*ft");
    await waitFor(() => expect(byId("queue-create-primitive-intent")).toBeEnabled());
    fireEvent.click(byId("queue-create-primitive-intent"));
    expect(queue.mock.calls.at(-1)![0].change).toMatchObject({ unit: "lbf*ft", dimension: "moment" });
  });

  it("both presets: no composed default is cleared", async () => {
    route.current = CATALOG;
    for (const [units, distributed] of [[SI_PROJECT_UNITS, "N/m"], [US_CUSTOMARY_PROJECT_UNITS, "lbf/in"]] as const) {
      mountLoads(blankWith(units));
      await waitFor(() => expect(byId("load-manager-create-primitive-unit").querySelectorAll("option").length).toBeGreaterThan(1));
      choose("load-manager-create-primitive-category", "distributed_force");
      expect(byId("load-manager-create-primitive-unit")).toHaveValue(distributed);
      choose("load-manager-create-primitive-category", "concentrated_moment");
      expect(byId("load-manager-create-primitive-unit")).toHaveValue(units.moment);
      cleanup();
    }
  });
});

describe("present but invalid keys never compose (Codex P2 on 93acfa719d)", () => {
  for (const [label, moment] of [["empty", ""], ["whitespace", "  "], ["non-string", 7]] as const) {
    it(`a ${label} moment key: the moment starts unselected with the notice and nothing can be queued`, async () => {
      route.current = PREVIEW;
      const model = blankWith(SI_PROJECT_UNITS);
      (model.project.units as Record<string, unknown>).moment = moment;
      const queue = mountLoads(model);
      choose("load-manager-create-primitive-category", "concentrated_moment");
      fireEvent.change(byId("load-manager-create-primitive-magnitude"), { target: { value: "80" } });
      expect(byId("load-manager-create-primitive-unit")).toHaveValue("");
      expect([...byId("load-manager-create-primitive-unit").querySelectorAll("option")].map((option) => option.value)).toEqual([""]);
      expect(byId("load-manager-create-primitive-unit-notice")).toHaveTextContent(
        typeof moment === "string" ? "project.units.moment is empty; choose one" : "project.units.moment is not a unit symbol; choose one"
      );
      // Give any pending unit validation time to settle; the queue stays closed.
      await new Promise((resolve) => setTimeout(resolve, 50));
      expect(byId("queue-create-primitive-intent")).toBeDisabled();
      fireEvent.click(byId("queue-create-primitive-intent"));
      expect(queue).not.toHaveBeenCalled();
    });
  }

  it("a blank length key: distributed force and support stiffness get no default; nothing is queued with N/m", async () => {
    route.current = CATALOG;
    const model = blankWith(SI_PROJECT_UNITS);
    model.project.units.length = " ";
    const queue = mountLoads(model);
    await waitFor(() => expect(byId("load-manager-create-primitive-unit").querySelectorAll("option").length).toBeGreaterThan(1));
    choose("load-manager-create-primitive-category", "distributed_force");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("");
    expect(byId("load-manager-create-primitive-unit-notice")).toHaveTextContent("project.units.length is empty; choose one");
    expect(byId("queue-create-primitive-intent")).toBeDisabled();
    expect(queue).not.toHaveBeenCalled();
    cleanup();
    const inspectorQueue = mountInspector(model);
    await waitFor(() => expect(byId("create-support-stiffness-unit-notice")).toHaveTextContent("project.units.length is empty; choose one"));
    expectFormControlValue(byId("create-support-stiffness-unit"), "");
    fireEvent.change(byId("create-support-stiffness"), { target: { value: "1500" } });
    expect(byId("queue-create-support-intent")).toBeDisabled();
    expect(inspectorQueue).not.toHaveBeenCalled();
  });

  it("a blank linear_stiffness or temperature key: the inspector gives no default and the notice", async () => {
    route.current = PREVIEW;
    const model = blankWith(SI_PROJECT_UNITS);
    model.project.units.linear_stiffness = "";
    model.project.units.temperature = " ";
    mountInspector(model);
    expect(byId("create-support-stiffness-unit-notice")).toHaveTextContent("project.units.linear_stiffness is empty; choose one");
    expectFormControlValue(byId("create-support-stiffness-unit"), "");
    expect(byId("create-material-thermal-unit-notice")).toHaveTextContent("project.units.temperature is empty; choose one");
    expectFormControlValue(byId("create-material-thermal-unit"), "");
  });
});

describe("a replaced project never keeps an inherited default unit (Codex P2 on 9018ebf587)", () => {
  function renderLoads(model: PreviewModel) {
    const queue = vi.fn<(intent: EditorOperationIntent) => void>();
    const view = render(<LoadCaseManagerPanel model={model} onQueueIntent={queue} onSelect={vi.fn()} selection={{ type: "project", id: model.project.id }} />);
    const replace = (next: PreviewModel) =>
      view.rerender(<LoadCaseManagerPanel model={next} onQueueIntent={queue} onSelect={vi.fn()} selection={{ type: "project", id: next.project.id }} />);
    return { queue, replace };
  }

  function withoutMoment(units: ProjectUnitMap): PreviewModel {
    const model = blankWith(units);
    delete model.project.units.moment;
    return model;
  }

  async function expectNothingQueueable(queue: ReturnType<typeof vi.fn>) {
    await new Promise((resolve) => setTimeout(resolve, 50));
    expect(byId("queue-create-primitive-intent")).toBeDisabled();
    fireEvent.click(byId("queue-create-primitive-intent"));
    expect(queue).not.toHaveBeenCalled();
  }

  for (const [label, moment] of [["empty", ""], ["whitespace", "  "]] as const) {
    it(`replacing a composed N*m draft with a project whose moment key is ${label} clears the unit`, async () => {
      route.current = PREVIEW;
      const { queue, replace } = renderLoads(withoutMoment(SI_PROJECT_UNITS));
      choose("load-manager-create-primitive-category", "concentrated_moment");
      fireEvent.change(byId("load-manager-create-primitive-magnitude"), { target: { value: "80" } });
      expect(byId("load-manager-create-primitive-unit")).toHaveValue("N*m");
      await waitFor(() => expect(byId("queue-create-primitive-intent")).toBeEnabled());
      const next = blankWith(SI_PROJECT_UNITS);
      next.project.id = "project:replacement";
      next.project.units.moment = moment;
      replace(next);
      expect(byId("load-manager-create-primitive-unit")).toHaveValue("");
      expect(byId("load-manager-create-primitive-unit-notice")).toHaveTextContent("project.units.moment is empty; choose one");
      await expectNothingQueueable(queue);
    });
  }

  it("the catalog resolving after the replacement clears a default the new project's catalog lacks", async () => {
    let resolveCatalog!: (catalog: UnitCatalogRoute) => void;
    route.pending = new Promise((resolve) => { resolveCatalog = resolve; });
    const { queue, replace } = renderLoads(withoutMoment(SI_PROJECT_UNITS));
    choose("load-manager-create-primitive-category", "concentrated_moment");
    fireEvent.change(byId("load-manager-create-primitive-magnitude"), { target: { value: "80" } });
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("N*m");
    // The replacement composes N*mm, unchecked while the catalog is still loading.
    replace(withoutMoment({ ...SI_PROJECT_UNITS, length: "mm" }));
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("N*mm");
    await act(async () => resolveCatalog(CATALOG));
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("");
    expect(byId("load-manager-create-primitive-unit-notice")).toHaveTextContent("N*mm is not a catalog unit; choose one");
    await expectNothingQueueable(queue);
  });

  it("the catalog resolving after a replacement with an empty moment key keeps the unit cleared", async () => {
    let resolveCatalog!: (catalog: UnitCatalogRoute) => void;
    route.pending = new Promise((resolve) => { resolveCatalog = resolve; });
    const { queue, replace } = renderLoads(withoutMoment(SI_PROJECT_UNITS));
    choose("load-manager-create-primitive-category", "concentrated_moment");
    fireEvent.change(byId("load-manager-create-primitive-magnitude"), { target: { value: "80" } });
    const next = blankWith(SI_PROJECT_UNITS);
    next.project.units.moment = "";
    replace(next);
    await act(async () => resolveCatalog(CATALOG));
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("");
    expect(byId("load-manager-create-primitive-unit-notice")).toHaveTextContent("project.units.moment is empty; choose one");
    await expectNothingQueueable(queue);
  });

  it("keeps an explicit unit choice across the replacement", async () => {
    route.current = CATALOG;
    const { queue, replace } = renderLoads(withoutMoment(SI_PROJECT_UNITS));
    await waitFor(() => expect(byId("load-manager-create-primitive-unit").querySelectorAll("option").length).toBeGreaterThan(1));
    choose("load-manager-create-primitive-category", "concentrated_moment");
    choose("load-manager-create-primitive-unit", "lbf*ft");
    const next = blankWith(SI_PROJECT_UNITS);
    next.project.units.moment = "";
    replace(next);
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("lbf*ft");
    await waitFor(() => expect(byId("queue-create-primitive-intent")).toBeEnabled());
    fireEvent.click(byId("queue-create-primitive-intent"));
    expect(queue.mock.calls.at(-1)![0].change).toMatchObject({ unit: "lbf*ft", dimension: "moment" });
  });

  it("inspector: replacing the project clears an inherited stiffness and expansion default", async () => {
    route.current = PREVIEW;
    const queue = vi.fn<(intent: EditorOperationIntent) => void>();
    const first = blankWith(SI_PROJECT_UNITS);
    const tree = (model: PreviewModel) => <DisplayUnitsProvider initialPreference="SI"><PropertyInspector model={model} selection={{ type: "node", id: "node:B" }} onQueueIntent={queue} /></DisplayUnitsProvider>;
    const view = render(tree(first));
    expectFormControlValue(byId("create-support-stiffness-unit"), "N/m");
    expectFormControlValue(byId("create-material-thermal-unit"), "1/degC");
    fireEvent.change(byId("create-support-stiffness"), { target: { value: "1500" } });
    expect(byId("queue-create-support-intent")).toBeEnabled();
    const next = blankWith(SI_PROJECT_UNITS);
    next.project.units.length = " ";
    next.project.units.temperature = "";
    view.rerender(tree(next));
    expectFormControlValue(byId("create-support-stiffness-unit"), "");
    expect(byId("create-support-stiffness-unit-notice")).toHaveTextContent("project.units.length is empty; choose one");
    expectFormControlValue(byId("create-material-thermal-unit"), "");
    expect(byId("create-material-thermal-unit-notice")).toHaveTextContent("project.units.temperature is empty; choose one");
    expect(byId("queue-create-support-intent")).toBeDisabled();
    fireEvent.click(byId("queue-create-support-intent"));
    expect(queue).not.toHaveBeenCalled();
  });
});

describe("inspector composed defaults", () => {
  it("in with N and a legacy C temperature: stiffness and expansion start unselected with the notice, nothing substituted", async () => {
    route.current = CATALOG;
    const model = blankWith({ ...SI_PROJECT_UNITS, length: "in" });
    model.project.units.temperature = "C";
    const queue = mountInspector(model);
    await waitFor(() => expect(byId("create-support-stiffness-unit-notice")).toHaveTextContent("N/in is not a catalog unit; choose one"));
    expectFormControlValue(byId("create-support-stiffness-unit"), "");
    expect(byId("create-material-thermal-unit-notice")).toHaveTextContent("1/C is not a catalog unit; choose one");
    expectFormControlValue(byId("create-material-thermal-unit"), "");
    fireEvent.change(byId("create-support-stiffness"), { target: { value: "1500" } });
    expect(byId("queue-create-support-intent")).toBeDisabled();
    changeFormControl(byId("create-support-stiffness-unit"), { target: { value: "lbf/in" } });
    expect(screen.queryByTestId("create-support-stiffness-unit-notice")).not.toBeInTheDocument();
    expect(byId("queue-create-support-intent")).toBeEnabled();
    fireEvent.click(byId("queue-create-support-intent"));
    expect(JSON.parse(queue.mock.calls.at(-1)![0].change.after).properties).toEqual({ linear_stiffness: { value: 1500, unit: "lbf/in" } });
  });

  it("both presets: the composed defaults are kept", async () => {
    route.current = CATALOG;
    for (const [units, stiffness, expansion] of [[SI_PROJECT_UNITS, "N/m", "1/degC"], [US_CUSTOMARY_PROJECT_UNITS, "lbf/in", "1/degF"]] as const) {
      mountInspector(blankWith(units));
      await waitFor(() => expect(byId("property-unit-catalog-status")).toHaveTextContent("DEC-018"));
      expectFormControlValue(byId("create-support-stiffness-unit"), stiffness);
      expectFormControlValue(byId("create-material-thermal-unit"), expansion);
      expect(screen.queryByTestId("create-support-stiffness-unit-notice")).not.toBeInTheDocument();
      cleanup();
    }
  });
});

describe("angle on a new blank document", () => {
  it("a rotational imposed displacement is drafted in rad and accepted by the core applier", async () => {
    route.current = PREVIEW;
    const model = blankWith(SI_PROJECT_UNITS);
    const queue = mountLoads(model);
    choose("load-manager-create-primitive-category", "imposed_displacement");
    choose("load-manager-create-primitive-direction", "RX");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("rad");
    await waitFor(() => expect(byId("queue-create-primitive-intent")).toBeEnabled());
    fireEvent.click(byId("queue-create-primitive-intent"));
    const intent = queue.mock.calls.at(-1)![0];
    expect(intent.change).toMatchObject({ unit: "rad", dimension: "rotation" });
    const outcome = await validateModelOperation(model, intent, await computeModelHash(model));
    expect(outcome.diagnostics.filter((diagnostic) => diagnostic.severity === "blocking")).toEqual([]);
    // The same operation on the document without the angle key is refused by the applier.
    const sixKeys = structuredClone(model);
    delete sixKeys.project.units.angle;
    const refused = await validateModelOperation(sixKeys, intent, await computeModelHash(sixKeys));
    expect(refused.diagnostics.map((diagnostic) => diagnostic.code)).toContain("OP-UNIT-METADATA-MISSING");
  });
});

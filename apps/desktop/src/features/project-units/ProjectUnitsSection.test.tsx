// W-17 slice (b), DEL-02-02 Design `project-unit-selection.md` §3 "A Project
// units section" and §4 "Vitest, section": the seven keys, the length lock, the
// missing-key notices, a refusal shown with nothing queued and a second change
// for one key refused. Each queued change is checked and applied through the
// wasm applier, the browser-mode engine. Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { applyModelOperation, validateModelOperation } from "../../services/operationService";
import { computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { PROJECT_UNIT_KEYS, SI_PROJECT_UNITS } from "../../services/projectService";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { DisplayUnitsProvider } from "../display-units";
import { PropertyInspector } from "../model-tree/PropertyInspector";
import { LENGTH_LOCKED_REASON, ProjectUnitsSection, projectUnitSectionChoices } from "./ProjectUnitsSection";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
/** The bundled demo: five nodes, and no `moment` key. */
const demo = () =>
  JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
/** The demo with all seven keys. */
const seven = () => {
  const model = demo();
  model.project.units = { ...model.project.units, moment: "N*m" };
  return model;
};
/** No nodes, so the length unit may change. */
const empty = (model: PreviewModel): PreviewModel => ({ ...model, nodes: [], pipe_segments: [], supports: [], components: [] });

const CATALOG: UnitCatalogRoute = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };

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
const options = (testId: string) => [...byId(testId).querySelectorAll("option")].map((option) => option.value);

beforeAll(async () => {
  await loadWasmEngine();
});
afterEach(cleanup);

describe("project units section", () => {
  it("shows the seven keys with the catalog's symbols, and locks length while nodes exist", () => {
    const model = seven();
    render(<ProjectUnitsSection model={model} onQueueIntent={() => undefined} unitCatalogRoute={CATALOG} />);
    for (const key of PROJECT_UNIT_KEYS) {
      expect(byId(`project-units-section-${key}`)).toHaveValue(model.project.units[key]);
      expect(screen.queryByTestId(`project-units-section-${key}-notice`)).toBeNull();
    }
    expect(options("project-units-section-force")).toEqual(["N", "kN", "lbf"]);
    expect(options("project-units-section-stress")).toEqual(["Pa", "kPa", "MPa", "GPa", "bar", "psi", "ksi"]);
    expect(options("project-units-section-temperature")).toEqual(["K", "degC", "degF", "degR"]);
    expect(byId("project-units-section-length")).toBeDisabled();
    expect(byId("project-units-section-length")).toHaveAttribute("title", LENGTH_LOCKED_REASON);
    expect(byId("project-units-section-length-locked")).toHaveTextContent("The length unit is fixed once nodes exist");
    expect(byId("project-units-section-force")).toBeEnabled();
  });

  it("queues a key change checked by the engine; it changes that key and nothing else", async () => {
    const original = seven();
    const list: EditorOperationIntent[] = [];
    render(<ProjectUnitsSection model={original} onQueueIntent={(intent) => list.push(intent)} unitCatalogRoute={CATALOG} />);
    fireEvent.change(byId("project-units-section-force"), { target: { value: "lbf" } });
    const set = await queued(list, 1);
    expect(set.operation_kind).toBe("modify");
    expect(set.target).toEqual({ object_type: "Project", ref: original.project.id });
    expect(set.change).toMatchObject({
      change_kind: "set_project_unit",
      field_path: "project.units.force",
      before: "N",
      after: "lbf",
      unit: "lbf",
      dimension: "force"
    });
    expect(byId("project-units-section-notice")).toHaveTextContent("queued for Review/Apply");
    const next = await apply(original, set);
    expect(next).toEqual({ ...original, project: { ...original.project, units: { ...original.project.units, force: "lbf" } } });
  });

  it("allows a length change while the project has no nodes", async () => {
    const model = empty(seven());
    const list: EditorOperationIntent[] = [];
    render(<ProjectUnitsSection model={model} onQueueIntent={(intent) => list.push(intent)} unitCatalogRoute={CATALOG} />);
    expect(byId("project-units-section-length")).toBeEnabled();
    expect(screen.queryByTestId("project-units-section-length-locked")).toBeNull();
    fireEvent.change(byId("project-units-section-length"), { target: { value: "mm" } });
    const set = await queued(list, 1);
    expect(set.change).toMatchObject({ field_path: "project.units.length", before: "m", after: "mm" });
    expect((await apply(model, set)).project.units.length).toBe("mm");
  });

  it("offers to set a missing key, never a silent default", async () => {
    const model = demo();
    expect(model.project.units).not.toHaveProperty("moment");
    const list: EditorOperationIntent[] = [];
    render(<ProjectUnitsSection model={model} onQueueIntent={(intent) => list.push(intent)} unitCatalogRoute={CATALOG} />);
    const notice = byId("project-units-section-moment-notice");
    expect(notice).toHaveAttribute("data-notice", "missing-offer");
    expect(notice).toHaveTextContent("Missing project unit: moment. Choose a moment unit to set it");
    expect(byId("project-units-section-moment")).toHaveValue("");
    expect(byId("project-units-section-moment")).toBeEnabled();
    fireEvent.change(byId("project-units-section-moment"), { target: { value: "N*m" } });
    const set = await queued(list, 1);
    expect(set.change).toMatchObject({ field_path: "project.units.moment", before: "not_present", after: "N*m", dimension: "moment" });
    expect((await apply(model, set)).project.units).toEqual({ ...model.project.units, moment: "N*m" });
  });

  it("gives missing length with nodes a re-author diagnostic and no fix; without nodes, an offer", () => {
    const withNodes = seven();
    delete (withNodes.project.units as Record<string, string>).length;
    const view = render(<ProjectUnitsSection model={withNodes} onQueueIntent={() => undefined} unitCatalogRoute={CATALOG} />);
    const notice = byId("project-units-section-length-notice");
    expect(notice).toHaveAttribute("data-notice", "missing-hard-lock");
    expect(notice).toHaveAttribute("role", "alert");
    expect(notice).toHaveTextContent("Missing project unit: length. The model has nodes whose positions have no stated unit");
    expect(notice).toHaveTextContent("no fix is offered");
    expect(byId("project-units-section-length")).toBeDisabled();
    view.rerender(<ProjectUnitsSection model={empty(withNodes)} onQueueIntent={() => undefined} unitCatalogRoute={CATALOG} />);
    expect(byId("project-units-section-length-notice")).toHaveAttribute("data-notice", "missing-offer");
    expect(byId("project-units-section-length")).toBeEnabled();
  });

  it("shows a legacy C as stored, with a note that a new choice is a catalog symbol", async () => {
    const model = seven();
    model.project.units.temperature = "C";
    const list: EditorOperationIntent[] = [];
    render(<ProjectUnitsSection model={model} onQueueIntent={(intent) => list.push(intent)} unitCatalogRoute={CATALOG} />);
    expect(byId("project-units-section-temperature")).toHaveValue("C");
    expect(options("project-units-section-temperature")).toEqual(["C", "K", "degC", "degF", "degR"]);
    expect(byId("project-units-section-temperature").querySelector('option[value="C"]')).toBeDisabled();
    expect(byId("project-units-section-temperature-notice")).toHaveTextContent("Stored as C, a legacy spelling. A new choice must be a catalog symbol (degC).");
    fireEvent.change(byId("project-units-section-temperature"), { target: { value: "degC" } });
    const set = await queued(list, 1);
    expect(set.change).toMatchObject({ before: "C", after: "degC" });
    expect((await apply(model, set)).project.units.temperature).toBe("degC");
  });

  it("shows a refusal by code and message, and queues nothing", async () => {
    // A catalog listing a force symbol the core catalog lacks: the engine refuses it by name.
    const route: UnitCatalogRoute = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
    if (route.route === "tauri_unit_catalog") {
      route.catalog.entries.push({ ...route.catalog.entries.find((entry) => entry.symbol === "lbf")!, unit_id: "unit:invented", symbol: "kgf_invented" });
    }
    const model = seven();
    const list: EditorOperationIntent[] = [];
    render(<ProjectUnitsSection model={model} onQueueIntent={(intent) => list.push(intent)} unitCatalogRoute={route} />);
    fireEvent.change(byId("project-units-section-force"), { target: { value: "kgf_invented" } });
    const item = await screen.findByTestId("project-units-section-diagnostic-OP-PROJECT-UNIT-NOT-CANONICAL");
    expect(item).toHaveTextContent("OP-PROJECT-UNIT-NOT-CANONICAL: `kgf_invented` is not a catalog unit symbol.");
    expect(byId("project-units-section-error")).toHaveTextContent("was refused by the operation engine; nothing was queued");
    // A retired document is refused by the engine too, by name (model document 0.8.0, r6),
    // and a newer one by the project-unit check.
    for (const [version, code] of [["0.6.0", "MODEL_SCHEMA_VERSION_RETIRED"], ["0.9.0", "OP-PROJECT-UNIT-DOCUMENT-INVALID"]] as const) {
      cleanup();
      const old = { ...seven(), schema_version: version };
      render(<ProjectUnitsSection model={old} onQueueIntent={(intent) => list.push(intent)} unitCatalogRoute={CATALOG} />);
      fireEvent.change(byId("project-units-section-force"), { target: { value: "kN" } });
      expect(await screen.findByTestId(`project-units-section-diagnostic-${code}`)).toBeInTheDocument();
      expect(list).toEqual([]);
    }
  });

  it("refuses a second pending change for one key, and allows one for another", async () => {
    const model = seven();
    const list: EditorOperationIntent[] = [];
    const view = render(<ProjectUnitsSection model={model} queuedIntents={list} onQueueIntent={(intent) => list.push(intent)} unitCatalogRoute={CATALOG} />);
    fireEvent.change(byId("project-units-section-force"), { target: { value: "lbf" } });
    await queued(list, 1);
    view.rerender(<ProjectUnitsSection model={model} queuedIntents={[...list]} onQueueIntent={(intent) => list.push(intent)} unitCatalogRoute={CATALOG} />);
    await waitFor(() => expect(byId("project-units-section-force")).toBeEnabled());
    fireEvent.change(byId("project-units-section-force"), { target: { value: "kN" } });
    expect(await screen.findByTestId("project-units-section-error")).toHaveTextContent(
      "Review or remove the pending project force unit change before queuing another."
    );
    expect(list).toHaveLength(1);
    fireEvent.change(byId("project-units-section-pressure"), { target: { value: "psi" } });
    expect((await queued(list, 2)).change.field_path).toBe("project.units.pressure");
  });

  it("offers the stored and preset symbols without the catalog", () => {
    const model = seven();
    expect(projectUnitSectionChoices(model, "stress", null)).toEqual(["MPa", "Pa", "psi"]);
    expect(projectUnitSectionChoices(model, "angle", null)).toEqual(["rad", "deg"]);
    model.project.units.temperature = "C";
    expect(projectUnitSectionChoices(model, "temperature", null)).toEqual(["degC", "degF"]);
    expect(projectUnitSectionChoices(empty(model), "length", CATALOG)).toEqual(["m", "mm", "in", "ft"]);
    expect(Object.keys(SI_PROJECT_UNITS)).toEqual([...PROJECT_UNIT_KEYS]);
  });

  it("is on the Project page of the property inspector, beside Model settings", () => {
    const model = seven();
    const view = render(
      <DisplayUnitsProvider initialPreference="SI">
        <PropertyInspector model={model} selection={{ type: "project", id: model.project.id }} onQueueIntent={() => undefined} />
      </DisplayUnitsProvider>
    );
    expect(within(byId("project-units-details")).getByText("Project units")).toBeInTheDocument();
    expect(byId("project-units-details")).toHaveAttribute("open");
    expect(byId("model-settings-details").nextElementSibling).toBe(byId("project-units-details"));
    expect(byId("project-units-section-force")).toHaveValue("N");
    view.rerender(
      <DisplayUnitsProvider initialPreference="SI">
        <PropertyInspector model={model} selection={{ type: "node", id: model.nodes[0].id }} onQueueIntent={() => undefined} />
      </DisplayUnitsProvider>
    );
    expect(screen.queryByTestId("project-units-details")).toBeNull();
  });
});

/** The Project page as opened: the property inspector with the project selected. */
function projectPage(model: PreviewModel, list: EditorOperationIntent[]) {
  return render(
    <DisplayUnitsProvider initialPreference="SI">
      <PropertyInspector model={model} selection={{ type: "project", id: model.project.id }} queuedIntents={list} onQueueIntent={(intent) => list.push(intent)} />
    </DisplayUnitsProvider>
  );
}

// Codex r1: a document opened without `project.units`, or without some keys,
// must open the Project page and show §3's notices; reading it writes nothing.
describe("Project page on a document missing project units", () => {
  it("opens without the map: every key missing, length with nodes locked, another key offered", async () => {
    const model = seven();
    delete (model.project as { units?: unknown }).units;
    const before = JSON.stringify(model);
    const list: EditorOperationIntent[] = [];
    projectPage(model, list);
    expect(byId("model-settings-details")).toBeInTheDocument();
    for (const key of PROJECT_UNIT_KEYS) {
      expect(byId(`project-units-section-${key}-notice`)).toHaveTextContent(`Missing project unit: ${key}.`);
    }
    const length = byId("project-units-section-length-notice");
    expect(length).toHaveAttribute("data-notice", "missing-hard-lock");
    expect(length).toHaveTextContent("no fix is offered");
    expect(byId("project-units-section-length")).toBeDisabled();
    expect(byId("project-units-section-force-notice")).toHaveAttribute("data-notice", "missing-offer");
    expect(byId("project-units-section-force")).toHaveValue("");
    // No unit is invented for the tolerance entry from the absent map, and the document is untouched.
    expect(options("model-settings-node-proximity-unit")).not.toContain("");
    expect(JSON.stringify(model)).toBe(before);
    fireEvent.change(byId("project-units-section-force"), { target: { value: "N" } });
    const set = await queued(list, 1);
    expect(set.change).toMatchObject({ change_kind: "set_project_unit", field_path: "project.units.force", before: "not_present", after: "N" });
    expect(JSON.stringify(model)).toBe(before);
    // Applied, the change adds the map holding the one key.
    expect((await apply(model, set)).project.units).toEqual({ force: "N" });
  });

  it("with some keys missing and nodes: length is a hard lock, another key queues with before not_present", async () => {
    const model = seven();
    const units = model.project.units as Record<string, string>;
    delete units.length;
    delete units.pressure;
    const before = JSON.stringify(model);
    const list: EditorOperationIntent[] = [];
    projectPage(model, list);
    expect(byId("project-units-section-length-notice")).toHaveAttribute("data-notice", "missing-hard-lock");
    expect(byId("project-units-section-length")).toBeDisabled();
    expect(byId("project-units-section-pressure-notice")).toHaveAttribute("data-notice", "missing-offer");
    expect(screen.queryByTestId("project-units-section-force-notice")).toBeNull();
    expect(byId("project-units-section-force")).toHaveValue("N");
    fireEvent.change(byId("project-units-section-pressure"), { target: { value: "psi" } });
    const set = await queued(list, 1);
    expect(set.change).toMatchObject({ field_path: "project.units.pressure", before: "not_present", after: "psi", dimension: "pressure" });
    expect(JSON.stringify(model)).toBe(before);
    expect((await apply(model, set)).project.units).toEqual({ ...units, pressure: "psi" });
  });

  it("with length missing and no nodes: length is offered and queues with before not_present", async () => {
    const model = empty(seven());
    model.project = { ...model.project, units: { ...model.project.units } };
    delete (model.project.units as Record<string, string>).length;
    const list: EditorOperationIntent[] = [];
    projectPage(model, list);
    expect(byId("project-units-section-length-notice")).toHaveAttribute("data-notice", "missing-offer");
    expect(byId("project-units-section-length")).toBeEnabled();
    fireEvent.change(byId("project-units-section-length"), { target: { value: "in" } });
    const set = await queued(list, 1);
    expect(set.change).toMatchObject({ field_path: "project.units.length", before: "not_present", after: "in", dimension: "length" });
    expect((await apply(model, set)).project.units.length).toBe("in");
  });
});

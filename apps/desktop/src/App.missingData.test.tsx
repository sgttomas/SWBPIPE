// Issue 380, the Missing data use case's correction flow (DEL-07-06 AC-007) through the whole App:
// DEL-07-07 REQ-008 / VER-003 (findings, selection and the Solve command's state), DEL-07-02
// REQ-004 / VER-001 (the value entered by the person in the inspector, with its unit) and
// DEL-04-06 REQ-013 (the checks before any solve are the solve's own, run here by the shared Wasm
// engine rebuilt from the candidate source). The start is the scenario's authored model; the
// expected findings are the scenario's missing values and the entries are the person's authored
// values, never product output. No native solve runs in this test: the request that would start
// one is observed and refused, so a started job is visible and none is completed here.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { App } from "./App";
import startModel from "../../../fixtures/embedded_agent/use_cases/missing-data/start_model.json";
import scenario from "../../../fixtures/embedded_agent/use_cases/missing-data/scenario.json";
import personEntered from "../../../fixtures/embedded_agent/use_cases/missing-data/person_entered.json";
import * as previewService from "./services/previewService";
import type { UnitCatalogRoute } from "./services/unitCatalogService";
import type { PreviewModel, PreviewSolverMode } from "./types";
import { unitCatalogFixture } from "./test-support/unitCatalogFixture";
import { changeFormControl } from "./test-support/workspaceTestControls";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("./services/unitCatalogService", async (original) => ({
  ...await original<typeof import("./services/unitCatalogService")>(),
  loadUnitCatalog: () => Promise.resolve(route.current)
}));

afterEach(() => { cleanup(); route.current = null; vi.restoreAllMocks(); });

const MISSING = scenario.expectations.missing_values.map((item) => ({ entity: item.entity_ref, field: `${item.field_path}.value` }));
const ENTRY = new Map(personEntered.entries.map((entry) => [entry.entity_ref, entry]));
const CODE = "EQUIVALENT_STATIC_INPUT_MISSING";
const inspector = () => screen.getByTestId("property-inspector");

function section(id: string) {
  let found = screen.queryByTestId(`workspace-section-${id}`);
  if (!found || found.classList.contains("inactive")) {
    if (id === "operations") fireEvent.click(screen.getByTestId("workspace-review"));
    else {
      fireEvent.click(screen.getByTestId("menu-view"));
      fireEvent.click(screen.getByTestId(`menu-item-view.section.${id}`));
    }
    found = screen.getByTestId(`workspace-section-${id}`);
  }
  return within(found);
}
function review() {
  const operations = section("operations");
  const tab = operations.getByTestId("operation-tab-review");
  if (tab.getAttribute("aria-pressed") !== "true") fireEvent.click(tab);
  return section("operations");
}
const solve = () => section("solve");
const withheldNotice = () => solve().queryByRole("region", { name: "Solve is withheld" });
const findingEntities = (notice: HTMLElement) =>
  within(notice).getAllByTestId(`model-check-${CODE}`).map((button) => button.getAttribute("data-entity"));

/** The scenario's model with the person's entries: what the final solve must receive. */
function expectedFinal(): PreviewModel {
  const model = structuredClone(startModel) as unknown as PreviewModel;
  for (const entry of personEntered.entries) {
    const pipe = model.pipe_segments.find((item) => item.id === entry.entity_ref)!;
    (pipe.section as Record<string, unknown>)[entry.field_path.split(".")[1]] = { value: entry.value, unit: entry.unit };
  }
  return model;
}

async function open() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(structuredClone(startModel) as unknown as PreviewModel);
  const start = vi.spyOn(previewService, "startPreviewMechanicsJob").mockRejectedValue(new Error("INVENTED-START-OBSERVED"));
  route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  return start;
}

/** From the finding on `entity` to its pipe, then the person's value and unit through the inspector and Review's Apply. */
async function enterFromFinding(entity: string) {
  const notice = withheldNotice()!;
  fireEvent.click(within(notice).getAllByTestId(`model-check-${CODE}`).find((button) => button.getAttribute("data-entity") === `pipe:${entity}`)!);
  await waitFor(() => expect(screen.getByTestId("status-selection")).toHaveTextContent(`pipe: ${entity}`));
  const taskTab = within(inspector()).getByRole("tab", { name: /^Task$/ });
  if (taskTab.getAttribute("aria-selected") !== "true") fireEvent.click(taskTab);
  const cancel = within(inspector()).queryByTestId("cancel-editor-intent");
  if (cancel) fireEvent.click(cancel);
  fireEvent.click(within(inspector()).getByTestId("inspector-start-task"));
  const field = MISSING.find((item) => item.entity === entity)!.field;
  changeFormControl(within(inspector()).getByTestId("editor-intent-field"), { target: { value: field } });
  const value = within(inspector()).getByTestId("editor-intent-value") as HTMLInputElement;
  expect(value).toHaveValue("TBD"); // nothing supplied it
  const entry = ENTRY.get(entity)!;
  fireEvent.change(value, { target: { value: String(entry.value) } });
  fireEvent.change(within(inspector()).getByTestId("editor-intent-unit"), { target: { value: entry.unit } });
  fireEvent.click(within(inspector()).getByTestId("queue-editor-intent"));
  const applies = review().getAllByTestId(/^apply-intent-editor-intent-/);
  fireEvent.click(applies[applies.length - 1]);
  await waitFor(() => expect(review().getByTestId("operation-apply-message")).toHaveTextContent("Applied op:editor-intent-pipe"));
}

it.each(["sparse_interactive", "dense_scrutiny"] as PreviewSolverMode[])(
  "withholds Solve before any job on both missing values, reaches each pipe from its finding, takes the person's values, clears, follows undo and redo, and starts the solve on exactly the entered model (%s)",
  async (mode) => {
    const start = await open();
    if (mode === "dense_scrutiny") fireEvent.click(solve().getByTestId("solver-mode-dense"));

    // Solve: the checks run on the request, find both values, and no job starts.
    fireEvent.click(solve().getByTestId("run-mechanics-preview"));
    const notice = await solve().findByRole("region", { name: "Solve is withheld" });
    expect(within(notice).getByTestId("solve-withheld-summary")).toHaveTextContent("2 solve-blocking findings stand for this model.");
    expect(findingEntities(notice).sort()).toEqual(MISSING.map((item) => `pipe:${item.entity}`).sort());
    expect(solve().getByTestId("run-mechanics-preview")).toHaveAccessibleName("Run mechanics preview (withheld)");
    expect(start).not.toHaveBeenCalled();
    // The Issues drawer lists the same findings.
    const panel = await screen.findByTestId("model-checks-panel");
    expect(within(panel).getAllByTestId(`model-check-${CODE}`)).toHaveLength(2);

    // From each finding to its pipe, and the person's value.
    for (const { entity } of MISSING) await enterFromFinding(entity);

    // The findings clear by themselves once both values stand.
    await waitFor(() => expect(withheldNotice()).toBeNull());
    expect(solve().getByTestId("run-mechanics-preview")).toHaveAccessibleName("Run mechanics preview");
    expect(start).not.toHaveBeenCalled();

    // Undo brings back the finding of the value it removes; redo clears it again.
    fireEvent.click(review().getByTestId("undo-session-model-edit"));
    await waitFor(() => expect(withheldNotice()).not.toBeNull());
    expect(findingEntities(withheldNotice()!)).toEqual([`pipe:${MISSING[MISSING.length - 1].entity}`]);
    fireEvent.click(review().getByTestId("redo-session-model-edit"));
    await waitFor(() => expect(withheldNotice()).toBeNull());

    // Solve now starts, once, on exactly the start model with the person's entries.
    fireEvent.click(solve().getByTestId("run-mechanics-preview"));
    await waitFor(() => expect(start).toHaveBeenCalledOnce());
    const [requested, requestedMode] = start.mock.calls[0];
    expect(JSON.parse(JSON.stringify(requested))).toEqual(JSON.parse(JSON.stringify(expectedFinal())));
    expect(requestedMode).toBe(mode);
  }
);

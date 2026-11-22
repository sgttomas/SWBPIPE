// Issue 380 (DEL-07-02 REQ-002 to REQ-004, VER-001): through the whole App, the person enters an
// absent material shear modulus G in the property inspector, with its unit, and Review's Apply
// runs the operation applier in the shared Wasm engine (rebuilt from the candidate source). The
// value and unit are read back from the inspector. Without a chosen unit the engine refuses and
// nothing is supplied. The start is the Missing data scenario's invented model with its G removed;
// the entered G is invented. The unit catalog is supplied as the native application supplies it.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { App } from "./App";
import startModel from "../../../fixtures/embedded_agent/use_cases/missing-data/start_model.json";
import * as previewService from "./services/previewService";
import type { UnitCatalogRoute } from "./services/unitCatalogService";
import type { PreviewModel } from "./types";
import { unitCatalogFixture } from "./test-support/unitCatalogFixture";
import { changeFormControl } from "./test-support/workspaceTestControls";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("./services/unitCatalogService", async (original) => ({
  ...await original<typeof import("./services/unitCatalogService")>(),
  loadUnitCatalog: () => Promise.resolve(route.current)
}));

afterEach(() => { cleanup(); route.current = null; vi.restoreAllMocks(); });

const MATERIAL = "material:md-steel-like";
const inspector = () => screen.getByTestId("property-inspector");
const value = () => within(inspector()).getByTestId("editor-intent-value") as HTMLInputElement;
const add = () => within(inspector()).getByTestId("queue-editor-intent") as HTMLButtonElement;
const shownUnit = () => (value().closest("label")?.textContent ?? "");

function review() {
  const section = screen.queryByTestId("workspace-section-operations");
  if (!section || section.classList.contains("inactive")) fireEvent.click(screen.getByTestId("workspace-review"));
  const tab = screen.getByTestId("operation-tab-review");
  if (tab.getAttribute("aria-pressed") !== "true") fireEvent.click(tab);
  return within(screen.getByTestId("workspace-section-operations"));
}

function startShearModulusTask() {
  fireEvent.click(within(screen.getByLabelText("Model tree")).getByTestId(`tree-row-material-${encodeURIComponent(MATERIAL)}`));
  const taskTab = within(inspector()).getByRole("tab", { name: /^Task$/ });
  if (taskTab.getAttribute("aria-selected") !== "true") fireEvent.click(taskTab);
  const cancel = within(inspector()).queryByTestId("cancel-editor-intent");
  if (cancel) fireEvent.click(cancel);
  fireEvent.click(within(inspector()).getByTestId("inspector-start-task"));
  changeFormControl(within(inspector()).getByTestId("editor-intent-field"), { target: { value: "shear_modulus.value" } });
}

async function openWithoutG() {
  const model = structuredClone(startModel) as unknown as PreviewModel & { materials: Array<Record<string, unknown>> };
  delete model.materials[0].shear_modulus;
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
  route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await within(screen.getByLabelText("Model tree")).findByTestId(`tree-row-material-${encodeURIComponent(MATERIAL)}`);
  startShearModulusTask();
  await waitFor(() => expect(within(inspector()).getByTestId("property-unit-catalog-status")).not.toHaveTextContent("loading"));
}

it("creates an absent G from the person's value and unit through the inspector and Apply, and reads it back", async () => {
  await openWithoutG();
  // Absent: the field says so and offers nothing to add.
  expect(value()).toHaveValue("TBD");
  expect(add()).toBeDisabled();
  fireEvent.change(value(), { target: { value: "79.3" } });
  changeFormControl(within(inspector()).getByTestId("editor-intent-unit"), { target: { value: "GPa" } });
  expect(shownUnit()).toContain("(GPa)");
  fireEvent.click(add());
  fireEvent.click(review().getByTestId("apply-intent-editor-intent-1"));
  await waitFor(() => expect(review().getByTestId("operation-apply-message")).toHaveTextContent("Applied op:editor-intent-material"));

  // Read back: a new task on G shows the stored value and unit, and nothing to add.
  startShearModulusTask();
  expect(value()).toHaveValue("79.3");
  expect(shownUnit()).toContain("(GPa)");
  expect(add()).toBeDisabled();

  // Undo removes it again, and redo restores it.
  fireEvent.click(screen.getByTestId("workspace-undo"));
  await waitFor(() => expect(review().getByTestId("session-history-chip")).toHaveTextContent("0 undo / 1 redo"));
  startShearModulusTask();
  expect(value()).toHaveValue("TBD");
  fireEvent.click(screen.getByTestId("workspace-redo"));
  await waitFor(() => expect(review().getByTestId("session-history-chip")).toHaveTextContent("1 undo / 0 redo"));
  startShearModulusTask();
  expect(value()).toHaveValue("79.3");
  expect(shownUnit()).toContain("(GPa)");
});

it("refuses an absent G entered with no unit chosen: the engine blocks it at Apply and supplies nothing", async () => {
  await openWithoutG();
  fireEvent.change(value(), { target: { value: "79.3" } });
  // No unit was chosen: the field still shows the absence marker as its unit.
  expect(shownUnit()).toContain("(TBD)");
  expect(add()).not.toBeDisabled();
  fireEvent.click(add());
  fireEvent.click(review().getByTestId("apply-intent-editor-intent-1"));
  await waitFor(() => expect(review().getByTestId("operation-apply-message")).toHaveTextContent(
    "Operation op:editor-intent-material-material:md-steel-like-shear_modulus.value was not applied (blocked)"));
  expect(document.body.textContent).toContain("OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
  // G is still absent.
  startShearModulusTask();
  expect(value()).toHaveValue("TBD");
});

// Through the whole App: Undo restores the stored value AND its unit under an open property
// task, and an entry the person has not edited follows them. Before this was so, the task
// kept the digits of the undone state beside the restored unit and offered them for Add as
// the person's own. The unit catalog is supplied so that the task's unit control offers a
// second stress unit, as the native application does. Invented data only.
import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { App } from "./App";
import type { UnitCatalogRoute } from "./services/unitCatalogService";
import { unitCatalogFixture } from "./test-support/unitCatalogFixture";
import { changeFormControl } from "./test-support/workspaceTestControls";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("./services/unitCatalogService", async (original) => ({
  ...await original<typeof import("./services/unitCatalogService")>(),
  loadUnitCatalog: () => Promise.resolve(route.current)
}));

afterEach(() => { route.current = null; vi.restoreAllMocks(); });

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

function startModulusTask() {
  fireEvent.click(within(screen.getByLabelText("Model tree")).getByTestId("tree-row-material-material%3Ainvented-carbon-steel"));
  const taskTab = within(inspector()).getByRole("tab", { name: /^Task$/ });
  if (taskTab.getAttribute("aria-selected") !== "true") fireEvent.click(taskTab);
  const cancel = within(inspector()).queryByTestId("cancel-editor-intent");
  if (cancel) fireEvent.click(cancel);
  fireEvent.click(within(inspector()).getByTestId("inspector-start-task"));
  changeFormControl(within(inspector()).getByTestId("editor-intent-field"), { target: { value: "elastic_modulus.value" } });
}

it("after Undo restores another unit, an untouched task entry shows the restored value and unit and offers nothing to Add", async () => {
  route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await within(screen.getByLabelText("Model tree")).findByTestId("tree-row-material-material%3Ainvented-carbon-steel");

  // The person re-stores the modulus as 200000 MPa and applies it.
  startModulusTask();
  await waitFor(() => expect(within(inspector()).getByTestId("property-unit-catalog-status")).not.toHaveTextContent("loading"));
  expect(value()).toHaveValue("200000000000");
  expect(shownUnit()).toContain("(Pa)");
  fireEvent.change(value(), { target: { value: "200000" } });
  changeFormControl(within(inspector()).getByTestId("editor-intent-unit"), { target: { value: "MPa" } });
  expect(shownUnit()).toContain("(MPa)");
  fireEvent.click(add());
  fireEvent.click(review().getByTestId("apply-intent-editor-intent-1"));
  await waitFor(() => expect(review().getByTestId("operation-apply-message")).toHaveTextContent("Applied op:editor-intent-material"));

  // A new task on the same field, nothing typed: the stored entry, and nothing to Add.
  startModulusTask();
  expect(value()).toHaveValue("200000");
  expect(shownUnit()).toContain("(MPa)");
  expect(add()).toBeDisabled();

  // Undo restores 200000000000 Pa under the open task.
  fireEvent.click(review().getByTestId("undo-session-model-edit"));
  await waitFor(() => expect(review().getByTestId("session-history-chip")).toHaveTextContent("0 undo / 1 redo"));
  expect(shownUnit()).toContain("(Pa)");
  // Not 200000 beside Pa, a millionth of what is stored, offered as the person's own.
  expect(value()).toHaveValue("200000000000");
  expect(add()).toBeDisabled();

  // Redo re-stores 200000 MPa, and the untouched entry follows again.
  fireEvent.click(review().getByTestId("redo-session-model-edit"));
  await waitFor(() => expect(review().getByTestId("session-history-chip")).toHaveTextContent("1 undo / 0 redo"));
  expect(shownUnit()).toContain("(MPa)");
  expect(value()).toHaveValue("200000");
  expect(add()).toBeDisabled();
});

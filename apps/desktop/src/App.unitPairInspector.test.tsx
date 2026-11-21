// Issue 382 control (DEL-07-02 REQ-002 to REQ-004): the property inspector always sends an
// explicit value/unit pair, so the defect did not reach it; this shows it still works for every
// family member it offers. Through the whole App, the person edits the field in the inspector
// and Review's Apply runs the operation applier in the Wasm engine rebuilt from the candidate
// source; the stored document is read back from the saved and reopened project (the App's own
// workspace session is captured by a call-through wrapper). The inspector offers material E, G
// and expansion coefficient, a pipe's inline diameter and wall, and a load case's first primitive
// magnitude; it does not offer a shared section's properties or a temperature point's G. The
// model and values are invented; the unit catalog is supplied as the native application does.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { App } from "./App";
import * as previewService from "./services/previewService";
import type { UnitCatalogRoute } from "./services/unitCatalogService";
import { unitCatalogFixture } from "./test-support/unitCatalogFixture";
import { changeFormControl } from "./test-support/workspaceTestControls";
import { FAMILY, unitPairFamilyModel } from "./test-support/unitPairFamilyModel";
import { FAMILY_MEMBERS, type FamilyMember } from "./test-support/unitPairFamilyMembers";
import { captureAppSession, storedDocument, type SessionRef } from "./test-support/unitPairSurfaceHarness";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("./services/unitCatalogService", async (original) => ({
  ...await original<typeof import("./services/unitCatalogService")>(),
  loadUnitCatalog: () => Promise.resolve(route.current)
}));

afterEach(() => { cleanup(); route.current = null; vi.restoreAllMocks(); });

const inspector = () => screen.getByTestId("property-inspector");
const value = () => within(inspector()).getByTestId("editor-intent-value") as HTMLInputElement;

function review() {
  const section = screen.queryByTestId("workspace-section-operations");
  if (!section || section.classList.contains("inactive")) fireEvent.click(screen.getByTestId("workspace-review"));
  const tab = screen.getByTestId("operation-tab-review");
  if (tab.getAttribute("aria-pressed") !== "true") fireEvent.click(tab);
  return within(screen.getByTestId("workspace-section-operations"));
}

const ROWS: Record<string, { type: string; id: string }> = {
  Material: { type: "material", id: FAMILY.material },
  Element: { type: "pipe", id: FAMILY.inlinePipe },
  Load: { type: "load", id: FAMILY.loadCase }
};
/** The members the inspector offers: all but the shared section and the temperature point,
 * and of the load case only its first primitive. */
const OFFERED = FAMILY_MEMBERS.filter((member) => member.objectType !== "Section" && !member.fieldPath.startsWith("temperature_points.")
  && member.fieldPath !== "primitive_loads.1.magnitude.value");

async function openTask(member: FamilyMember): Promise<SessionRef> {
  const session = captureAppSession();
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(unitPairFamilyModel());
  route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  const row = ROWS[member.objectType];
  const tree = within(screen.getByLabelText("Model tree"));
  fireEvent.click(await tree.findByTestId(`tree-row-${row.type}-${encodeURIComponent(row.id)}`));
  const taskTab = within(inspector()).getByRole("tab", { name: /^Task$/ });
  if (taskTab.getAttribute("aria-selected") !== "true") fireEvent.click(taskTab);
  fireEvent.click(within(inspector()).getByTestId("inspector-start-task"));
  changeFormControl(within(inspector()).getByTestId("editor-intent-field"), { target: { value: member.fieldPath } });
  await waitFor(() => expect(within(inspector()).getByTestId("property-unit-catalog-status")).not.toHaveTextContent("loading"));
  return session;
}

async function queueAndApply(session: SessionRef) {
  const add = within(inspector()).getByTestId("queue-editor-intent") as HTMLButtonElement;
  expect(add).not.toBeDisabled();
  fireEvent.click(add);
  const intent = session.current.operations.editorIntents.at(-1)!;
  fireEvent.click(review().getByTestId("apply-intent-editor-intent-1"));
  await waitFor(() => expect(review().getByTestId("operation-apply-message")).toHaveTextContent(`Applied ${intent.operation_id}`));
  return intent;
}

describe.each(OFFERED)("property inspector: $name ($fieldPath)", (member) => {
  it("changing only the number keeps the stored unit, and the stored document holds it", async () => {
    const session = await openTask(member);
    fireEvent.change(value(), { target: { value: String(member.inStoredUnit) } });
    const intent = await queueAndApply(session);
    expect(intent.change).toMatchObject({ field_path: member.fieldPath, unit: member.stored.unit, dimension: member.dimension,
      after: JSON.stringify({ value: member.inStoredUnit, unit: member.stored.unit }) });
    expect(member.read(await storedDocument(session))).toEqual({ value: member.inStoredUnit, unit: member.stored.unit });
  });

  it("changing the unit and the number writes both, and the stored document holds the pair", async () => {
    const session = await openTask(member);
    changeFormControl(within(inspector()).getByTestId("editor-intent-unit"), { target: { value: member.otherUnit.unit } });
    fireEvent.change(value(), { target: { value: String(member.otherUnit.value) } });
    const intent = await queueAndApply(session);
    expect(intent.change).toMatchObject({ field_path: member.fieldPath, unit: member.otherUnit.unit, after: JSON.stringify(member.otherUnit) });
    expect(member.read(await storedDocument(session))).toEqual(member.otherUnit);
  });
});

// Issue 382 (DEL-07-02 REQ-003; DEL-16-02 REQ-010): the load case manager's primitive magnitude
// edit sends an explicit value/unit pair with the primitive's stored dimension member. The engine
// now judges a magnitude against the dimension its category, target and direction give, so this
// shows that a magnitude edit in the stored unit and in another unit of the same dimension both
// apply, for a distributed force on a pipe and a concentrated force at a node. Through the whole
// App, Review's Apply runs the operation applier in the Wasm engine rebuilt from the candidate
// source; the stored document is read back from the saved and reopened project. The model and
// values are invented; the unit catalog is supplied as the native application supplies it.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { App } from "./App";
import * as previewService from "./services/previewService";
import type { UnitCatalogRoute } from "./services/unitCatalogService";
import { unitCatalogFixture } from "./test-support/unitCatalogFixture";
import { changeFormControl } from "./test-support/workspaceTestControls";
import { unitPairFamilyModel } from "./test-support/unitPairFamilyModel";
import { FAMILY_MEMBERS, type FamilyMember } from "./test-support/unitPairFamilyMembers";
import { captureAppSession, storedDocument } from "./test-support/unitPairSurfaceHarness";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("./services/unitCatalogService", async (original) => ({
  ...await original<typeof import("./services/unitCatalogService")>(),
  loadUnitCatalog: () => Promise.resolve(route.current)
}));

afterEach(() => { cleanup(); route.current = null; vi.restoreAllMocks(); });

/** The native menu's view command, as App.test.tsx dispatches it. */
const nativeMenuCommand = (command: string) => window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));

function openSection(id: "loads" | "operations") {
  let section = screen.queryByTestId(`workspace-section-${id}`);
  if (!section || section.classList.contains("inactive")) {
    if (id === "operations" && screen.queryByTestId("workspace-review")) fireEvent.click(screen.getByTestId("workspace-review"));
    else if (screen.queryByTestId("menu-view")) {
      fireEvent.click(screen.getByTestId("menu-view"));
      fireEvent.click(screen.getByTestId(`menu-item-view.section.${id}`));
    } else act(() => nativeMenuCommand(`view.section.${id}`));
    section = screen.getByTestId(`workspace-section-${id}`);
  }
  expect(section).not.toHaveClass("inactive");
  if (id === "operations") {
    const tab = screen.getByTestId("operation-tab-review");
    if (tab.getAttribute("aria-pressed") !== "true") fireEvent.click(tab);
  }
  return within(section);
}

const PRIMITIVES: Record<string, string> = { "primitive_loads.0.magnitude.value": "load:uf-line", "primitive_loads.1.magnitude.value": "load:uf-tip" };
const MAGNITUDES = FAMILY_MEMBERS.filter((member) => member.fieldPath in PRIMITIVES);

async function editMagnitude(member: FamilyMember, entered: { value: number; unit: string }) {
  const session = captureAppSession();
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(unitPairFamilyModel());
  route.current = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
  render(<App />);
  await screen.findByTestId("workspace-toolbar");
  const manager = within(openSection("loads").getByTestId("load-case-manager"));
  fireEvent.click(manager.getByTestId(`load-manager-primitive-${PRIMITIVES[member.fieldPath]}`));
  expect(manager.getByTestId("load-manager-selected-primitive")).toHaveTextContent(member.fieldPath);
  await waitFor(() => expect(manager.getByTestId("load-manager-magnitude-unit")).toHaveValue(member.stored.unit));
  // The unit list is the field's dimension's; it holds the other unit entered here.
  await waitFor(() => expect([...(manager.getByTestId("load-manager-magnitude-unit") as HTMLSelectElement).options].map((option) => option.value)).toContain(entered.unit));
  if (entered.unit !== member.stored.unit) changeFormControl(manager.getByTestId("load-manager-magnitude-unit"), { target: { value: entered.unit } });
  fireEvent.change(manager.getByTestId("load-manager-magnitude-value"), { target: { value: String(entered.value) } });
  fireEvent.click(manager.getByTestId("queue-load-magnitude-intent"));
  const intent = session.current.operations.editorIntents.at(-1)!;
  expect(intent.change).toMatchObject({ field_path: member.fieldPath, unit: entered.unit, dimension: member.dimension, after: JSON.stringify(entered) });
  const review = openSection("operations");
  fireEvent.click(review.getAllByTestId(/^apply-intent-/)[0]);
  await waitFor(() => expect(review.getByTestId("operation-apply-message")).toHaveTextContent(`Applied ${intent.operation_id}`));
  expect(member.read(await storedDocument(session))).toEqual(entered);
}

describe.each(MAGNITUDES)("load case manager: $name ($fieldPath)", (member) => {
  it("applies a magnitude edit in the stored unit, and the stored document holds it", async () => {
    await editMagnitude(member, { value: member.inStoredUnit, unit: member.stored.unit });
  });

  it("applies a magnitude edit in another unit of the same dimension, and the stored document holds the pair", async () => {
    await editMagnitude(member, member.otherUnit);
  });
});

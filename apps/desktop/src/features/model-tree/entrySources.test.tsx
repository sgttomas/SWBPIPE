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
  render(
    <DisplayUnitsProvider initialPreference="SI">
      <PropertyInspector model={demo()} selection={selection} onQueueIntent={queue} />
    </DisplayUnitsProvider>
  );
  await waitFor(() => expect(byId("property-unit-catalog-status")).not.toHaveTextContent("loading"));
  const intents = () => queue.mock.calls.map(([intent]) => intent);
  return { intents };
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

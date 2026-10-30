// W-17 (DEL-02-02 Design `project-unit-selection.md` §3) in the viewport's
// creation forms: a unit whose project key is absent starts unselected, the
// form names the key with the property inspector's notice
// (`features/project-units/missingProjectUnit.ts`) and nothing is queued. With
// every key present no notice is shown. Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import type { ProjectUnitKey } from "../../services/projectService";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { changeFormControl, expectFormControlValue } from "../../test-support/workspaceTestControls";
import { DisplayUnitsProvider } from "../display-units";
import { PipeViewport } from "./PipeViewport";

const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: async () => route.current
}));
const CATALOG: UnitCatalogRoute = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };

afterEach(() => { cleanup(); route.current = null; });

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
function model(missing: ProjectUnitKey[] = []): PreviewModel {
  const document = JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
  document.project.units = { ...document.project.units, moment: "N*m" };
  for (const key of missing) delete (document.project.units as Record<string, string>)[key];
  return document;
}

const byId = (testId: string) => screen.getByTestId(testId);

async function mount(document: PreviewModel) {
  const queue = vi.fn();
  render(
    <DisplayUnitsProvider initialPreference="SI">
      <PipeViewport model={document} selection={{ type: "node", id: "node:N-110" }} onSelect={vi.fn()} onQueueIntent={queue} />
    </DisplayUnitsProvider>
  );
  route.current = CATALOG;
  await waitFor(() => expect(byId("viewport-create-node-unit-basis")).not.toHaveTextContent(/unavailable|pending/i));
  return queue;
}

describe("viewport creation forms and missing project units", () => {
  it("with every key: no notice, and the component units default to the keys", async () => {
    route.current = CATALOG;
    await mount(model());
    expectFormControlValue(byId("viewport-create-component-angle-unit"), "rad");
    expect(screen.queryByTestId(/missing-project-unit/)).toBeNull();
  });

  it("missing angle and force: component creation names each key, starts the unit unselected and cannot queue", async () => {
    route.current = CATALOG;
    const queue = await mount(model(["angle", "force"]));
    expectFormControlValue(byId("viewport-create-component-angle-unit"), "TBD");
    expect(byId("viewport-create-component-missing-project-unit-angle")).toHaveTextContent(
      "Missing project unit: angle. This field has no default unit, and nothing is assumed. Choose its unit here, or set angle in Project units on the Project page."
    );
    expect(byId("queue-explicit-component-intent")).toBeDisabled();
    changeFormControl(byId("viewport-create-component-kind"), { target: { value: "valve" } });
    expectFormControlValue(byId("viewport-create-component-force-unit"), "TBD");
    expect(byId("viewport-create-component-missing-project-unit-force")).toHaveTextContent("Missing project unit: force.");
    expect(screen.queryByTestId("viewport-create-component-missing-project-unit-angle")).toBeNull();
    // A unit picked in the field's own selector lifts the notice.
    changeFormControl(byId("viewport-create-component-force-unit"), { target: { value: "kN" } });
    expect(screen.queryByTestId("viewport-create-component-missing-project-unit-force")).toBeNull();
    expect(queue).not.toHaveBeenCalled();
  });

  it("missing length with nodes: the node form shows the hard lock and the pipe form names the key", async () => {
    route.current = CATALOG;
    const queue = await mount(model(["length"]));
    expect(byId("viewport-create-node-missing-project-unit-length")).toHaveAttribute("role", "alert");
    expect(byId("viewport-create-node-missing-project-unit-length")).toHaveTextContent("Re-author the model with a length unit");
    expect(byId("viewport-create-pipe-missing-project-unit-length")).toHaveTextContent("Missing project unit: length.");
    expect(byId("viewport-create-component-missing-project-unit-length")).toHaveTextContent("Missing project unit: length.");
    expect(queue).not.toHaveBeenCalled();
  });
});

import { act, cleanup, fireEvent, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { SI_PROJECT_UNITS, US_CUSTOMARY_PROJECT_UNITS, type ProjectUnitMap } from "../../services/projectService";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { ProjectUnitsDialog, type BlankProjectUnitsRequest } from "./ProjectUnitsDialog";

// W-17 slice (a), DEL-02-02 Design `project-unit-selection.md` §3 "Units at creation".
const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: async () => route.current
}));
const CATALOG: UnitCatalogRoute = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
const PREVIEW: UnitCatalogRoute = { route: "unavailable_browser_preview", diagnostic: "UNIT-CATALOG-DESKTOP-ONLY: fixture" };

afterEach(() => { cleanup(); route.current = null; });

function open(catalog: UnitCatalogRoute, kind: BlankProjectUnitsRequest["kind"] = "local") {
  route.current = catalog;
  const create = vi.fn<(units: ProjectUnitMap) => void>();
  const cancel = vi.fn();
  render(<ProjectUnitsDialog request={{ kind, opener: null, create, cancel }} />);
  return { create, cancel };
}

const byId = (testId: string) => screen.getByTestId(testId);
const choose = (testId: string, value: string) => fireEvent.change(byId(testId), { target: { value } });
const optionValues = (testId: string) => [...byId(testId).querySelectorAll("option")].map((option) => option.value);
const catalogLoaded = () => waitFor(() => expect(byId("project-units-length")).toBeEnabled());

describe("units at creation", () => {
  it("opens with SI selected, shows the seven keys and the storage text, and creates SI", async () => {
    const { create } = open(CATALOG);
    await catalogLoaded();
    expect(byId("project-units-dialog")).toHaveAttribute("role", "dialog");
    expect(byId("project-units-preset")).toHaveValue("si");
    expect(byId("project-units-preset")).toHaveFocus();
    for (const [key, symbol] of Object.entries(SI_PROJECT_UNITS)) expect(byId(`project-units-${key}`)).toHaveValue(symbol);
    expect(byId("project-units-detail")).toHaveTextContent(
      "Values are stored in the units you enter, and the solver converts them at its boundary. The length unit is fixed once the project has nodes."
    );
    expect(screen.queryByTestId("project-units-composed")).not.toBeInTheDocument();
    fireEvent.click(byId("project-units-create"));
    expect(create).toHaveBeenCalledWith({ ...SI_PROJECT_UNITS });
  });

  it("applies the US customary preset", async () => {
    const { create } = open(CATALOG, "load_state");
    await catalogLoaded();
    expect(byId("project-units-dialog")).toHaveAttribute("data-kind", "load_state");
    choose("project-units-preset", "us_customary");
    for (const [key, symbol] of Object.entries(US_CUSTOMARY_PROJECT_UNITS)) expect(byId(`project-units-${key}`)).toHaveValue(symbol);
    fireEvent.click(byId("project-units-create"));
    expect(create).toHaveBeenCalledWith({ ...US_CUSTOMARY_PROJECT_UNITS });
  });

  it("offers a Custom choice per key from the catalog, canonical symbols only, and lists the composed defaults", async () => {
    const { create } = open(CATALOG);
    await catalogLoaded();
    expect(optionValues("project-units-temperature")).toEqual(["K", "degC", "degF", "degR"]);
    expect(optionValues("project-units-temperature")).not.toContain("C");
    expect(optionValues("project-units-moment")).toEqual(["N*m", "kN*m", "lbf*in", "lbf*ft"]);
    expect(optionValues("project-units-stress")).toContain("MPa");
    choose("project-units-length", "mm");
    expect(byId("project-units-preset")).toHaveValue("custom");
    const composed = byId("project-units-composed");
    expect(within(composed).getByTestId("project-units-composed-distributed_force")).toHaveTextContent("Distributed force: N/mm");
    expect(within(composed).getByTestId("project-units-composed-distributed_force")).toHaveAttribute("data-in-catalog", "true");
    // A mixed SI/US choice composes N/in, which the catalog lacks: a notice, not a refusal.
    choose("project-units-length", "in");
    for (const kind of ["distributed_force", "linear_stiffness"]) {
      expect(byId(`project-units-composed-${kind}`)).toHaveAttribute("data-in-catalog", "false");
      expect(byId(`project-units-composed-${kind}`)).toHaveTextContent("N/in (not a catalog unit; that field will start unselected)");
    }
    expect(byId("project-units-composed-thermal_expansion")).toHaveAttribute("data-in-catalog", "true");
    expect(byId("project-units-create")).toBeEnabled();
    fireEvent.click(byId("project-units-create"));
    expect(create).toHaveBeenCalledWith({ ...SI_PROJECT_UNITS, length: "in" });
  });

  it("offers presets only where the catalog is not exposed", async () => {
    const { create } = open(PREVIEW);
    await waitFor(() => expect(byId("project-units-catalog-status")).toHaveTextContent("Custom units need the desktop unit catalog; presets are available. UNIT-CATALOG-DESKTOP-ONLY: fixture"));
    for (const key of Object.keys(SI_PROJECT_UNITS)) expect(byId(`project-units-${key}`)).toBeDisabled();
    expect(byId("project-units-preset").querySelector('option[value="custom"]')).toBeDisabled();
    choose("project-units-preset", "us_customary");
    fireEvent.click(byId("project-units-create"));
    expect(create).toHaveBeenCalledWith({ ...US_CUSTOMARY_PROJECT_UNITS });
  });

  it("cancels on Cancel and on Escape, creating nothing", async () => {
    const { create, cancel } = open(CATALOG);
    await catalogLoaded();
    fireEvent.keyDown(window, { key: "Escape" });
    expect(cancel).toHaveBeenCalledTimes(1);
    fireEvent.click(byId("project-units-cancel"));
    expect(cancel).toHaveBeenCalledTimes(2);
    expect(create).not.toHaveBeenCalled();
  });
});

describe("the session's blank routes ask for units first", () => {
  it("New blank and file.new-blank open the dialog; Create builds the blank with the chosen map", async () => {
    route.current = PREVIEW;
    const hook = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
    act(() => hook.result.current.chrome.runMenuCommand("file.new-blank"));
    const request = hook.result.current.project.blankProjectUnitsRequest;
    expect(request?.kind).toBe("local");
    // Nothing is created until Create; menu commands are held while the dialog is open.
    expect(hook.result.current.model.model?.project.name).not.toBe("Blank Local Model");
    act(() => hook.result.current.chrome.runMenuCommand("file.new-local"));
    expect(hook.result.current.project.blankProjectUnitsRequest).toBe(request);
    act(() => request!.create({ ...US_CUSTOMARY_PROJECT_UNITS }));
    expect(hook.result.current.project.blankProjectUnitsRequest).toBeNull();
    await waitFor(() => expect(hook.result.current.model.model?.project.name).toBe("Blank Local Model"));
    expect(hook.result.current.model.model?.project.units).toEqual(US_CUSTOMARY_PROJECT_UNITS);

    act(() => hook.result.current.project.requestBlankProject("load_state"));
    expect(hook.result.current.project.blankProjectUnitsRequest?.kind).toBe("load_state");
    act(() => hook.result.current.project.blankProjectUnitsRequest!.cancel());
    expect(hook.result.current.project.blankProjectUnitsRequest).toBeNull();
    expect(hook.result.current.model.model?.project.name).toBe("Blank Local Model");
    act(() => hook.result.current.project.requestBlankProject("load_state"));
    act(() => hook.result.current.project.blankProjectUnitsRequest!.create({ ...SI_PROJECT_UNITS, length: "mm" }));
    await waitFor(() => expect(hook.result.current.model.model?.project.name).toBe("Blank Local Load/reference Model 0.7.0"));
    expect(hook.result.current.model.model?.project.units).toEqual({ ...SI_PROJECT_UNITS, length: "mm" });
  });
});

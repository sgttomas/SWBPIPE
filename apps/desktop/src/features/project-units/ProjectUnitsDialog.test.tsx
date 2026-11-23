import { act, cleanup, fireEvent, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { SI_PROJECT_UNITS, US_CUSTOMARY_PROJECT_UNITS, type ProjectUnitMap } from "../../services/projectService";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { ProjectUnitsDialog, type BlankProjectKind, type BlankProjectUnitsRequest } from "./ProjectUnitsDialog";

// W-17 slice (a), DEL-02-02 Design `project-unit-selection.md` §3 "Units at creation".
const route = vi.hoisted(() => ({ current: null as UnitCatalogRoute | null }));
vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: async () => route.current
}));
const CATALOG: UnitCatalogRoute = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
const PREVIEW: UnitCatalogRoute = { route: "unavailable_browser_preview", diagnostic: "UNIT-CATALOG-DESKTOP-ONLY: fixture" };

afterEach(() => { cleanup(); route.current = null; });

function open(catalog: UnitCatalogRoute, kind: BlankProjectUnitsRequest["kind"] = undefined) {
  route.current = catalog;
  const create = vi.fn<(units: ProjectUnitMap, kind: BlankProjectKind) => void>();
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
    expect(create).toHaveBeenCalledWith({ ...SI_PROJECT_UNITS }, "local");
  });

  it("applies the US customary preset", async () => {
    const { create } = open(CATALOG, "load_state");
    await catalogLoaded();
    expect(byId("project-units-dialog")).toHaveAttribute("data-kind", "load_state");
    choose("project-units-preset", "us_customary");
    for (const [key, symbol] of Object.entries(US_CUSTOMARY_PROJECT_UNITS)) expect(byId(`project-units-${key}`)).toHaveValue(symbol);
    fireEvent.click(byId("project-units-create"));
    expect(create).toHaveBeenCalledWith({ ...US_CUSTOMARY_PROJECT_UNITS }, "load_state");
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
    expect(create).toHaveBeenCalledWith({ ...SI_PROJECT_UNITS, length: "in" }, "local");
  });

  it("offers presets only where the catalog is not exposed", async () => {
    const { create } = open(PREVIEW);
    await waitFor(() => expect(byId("project-units-catalog-status")).toHaveTextContent("Custom units need the desktop unit catalog; presets are available. UNIT-CATALOG-DESKTOP-ONLY: fixture"));
    for (const key of Object.keys(SI_PROJECT_UNITS)) expect(byId(`project-units-${key}`)).toBeDisabled();
    expect(byId("project-units-preset").querySelector('option[value="custom"]')).toBeDisabled();
    choose("project-units-preset", "us_customary");
    fireEvent.click(byId("project-units-create"));
    expect(create).toHaveBeenCalledWith({ ...US_CUSTOMARY_PROJECT_UNITS }, "local");
  });

  it("offers the starting model, the ordinary blank selected; the other is chosen by its plain words", async () => {
    const { create } = open(CATALOG);
    await catalogLoaded();
    const dialog = byId("project-units-dialog");
    expect(within(dialog).getByRole("heading")).toHaveTextContent(/^New blank project$/);
    const group = byId("project-units-kind");
    expect(within(group).getByText("Starting model", { selector: "legend" })).toBeInTheDocument();
    expect(within(group).getByRole("radio", { name: "Ordinary blank model" })).toBeChecked();
    expect(byId("project-units-kind-local")).toBeChecked();
    expect(dialog).toHaveAttribute("data-kind", "local");
    fireEvent.click(within(group).getByRole("radio", { name: "Set up for internal pressure (pressure regions on each load case)" }));
    expect(byId("project-units-kind-load-state")).toBeChecked();
    expect(dialog).toHaveAttribute("data-kind", "load_state");
    fireEvent.click(byId("project-units-create"));
    expect(create).toHaveBeenCalledWith({ ...SI_PROJECT_UNITS }, "load_state");
  });

  it("names no version, contract or load/reference state on its face; one help line names the pressure route", async () => {
    open(CATALOG);
    await catalogLoaded();
    const face = byId("project-units-dialog").textContent ?? "";
    for (const word of ["0.9.0", "3.0.0", "load/reference", "Load/reference", "exact pressure contract"]) expect(face).not.toContain(word);
    expect(byId("project-units-kind-help")).toHaveTextContent(/^Choose this when load cases carry internal pressure; it uses the exact pressure route the guide describes\.$/);
    expect(face.match(/exact pressure/g)).toHaveLength(1);
  });

  it("keeps Tab inside the dialog, through the checked starting model", async () => {
    open(CATALOG);
    await catalogLoaded();
    const order: string[] = [];
    for (let step = 0; step < 12; step += 1) {
      fireEvent.keyDown(window, { key: "Tab" });
      order.push((document.activeElement as HTMLElement).getAttribute("data-testid") ?? "");
      if (order.at(-1) === "project-units-kind-local" && step > 0) break;
    }
    // From the preset: the seven unit selectors, Cancel, Create, then round to the starting model.
    expect(order).toContain("project-units-cancel");
    expect(order).toContain("project-units-create");
    expect(order.at(-1)).toBe("project-units-kind-local");
    expect(order).not.toContain("project-units-kind-load-state");
    for (const id of order) expect(byId("project-units-dialog").contains(byId(id))).toBe(true);
    // The chosen option is the stop once chosen; Shift+Tab goes back from it to Create.
    fireEvent.click(byId("project-units-kind-load-state"));
    byId("project-units-kind-load-state").focus();
    fireEvent.keyDown(window, { key: "Tab", shiftKey: true });
    expect(byId("project-units-create")).toHaveFocus();
    fireEvent.keyDown(window, { key: "Tab" });
    expect(byId("project-units-kind-load-state")).toHaveFocus();
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
  it("New and file.new-blank open the one dialog, ordinary blank selected; Create builds the chosen kind with the chosen map", async () => {
    route.current = PREVIEW;
    const hook = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
    act(() => hook.result.current.chrome.runMenuCommand("file.new-blank"));
    const request = hook.result.current.project.blankProjectUnitsRequest;
    expect(request?.kind ?? "local").toBe("local");
    // Nothing is created until Create; menu commands are held while the dialog is open.
    expect(hook.result.current.model.model?.project.name).not.toBe("Blank Local Model");
    act(() => hook.result.current.chrome.runMenuCommand("file.new-local"));
    expect(hook.result.current.project.blankProjectUnitsRequest).toBe(request);
    act(() => request!.create({ ...US_CUSTOMARY_PROJECT_UNITS }, "local"));
    expect(hook.result.current.project.blankProjectUnitsRequest).toBeNull();
    await waitFor(() => expect(hook.result.current.model.model?.project.name).toBe("Blank Local Model"));
    expect(hook.result.current.model.model?.project.units).toEqual(US_CUSTOMARY_PROJECT_UNITS);

    // New (requestBlankProject()) opens the same dialog; Cancel creates nothing.
    act(() => hook.result.current.project.requestBlankProject());
    expect(hook.result.current.project.blankProjectUnitsRequest?.kind ?? "local").toBe("local");
    act(() => hook.result.current.project.blankProjectUnitsRequest!.cancel());
    expect(hook.result.current.project.blankProjectUnitsRequest).toBeNull();
    expect(hook.result.current.model.model?.project.name).toBe("Blank Local Model");
    // The starting model chosen in the dialog is the kind built.
    act(() => hook.result.current.project.requestBlankProject());
    act(() => hook.result.current.project.blankProjectUnitsRequest!.create({ ...SI_PROJECT_UNITS, length: "mm" }, "load_state"));
    await waitFor(() => expect(hook.result.current.model.model?.project.name).toBe("Blank Local Load/reference Model 0.9.0"));
    expect(hook.result.current.model.model?.project.units).toEqual({ ...SI_PROJECT_UNITS, length: "mm" });
  });

  it("File › New Blank Project passes the chosen starting model through the same dialog", async () => {
    route.current = PREVIEW;
    const hook = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
    act(() => hook.result.current.chrome.runMenuCommand("file.new-blank"));
    render(<ProjectUnitsDialog request={hook.result.current.project.blankProjectUnitsRequest} />);
    expect(byId("project-units-kind-local")).toBeChecked();
    fireEvent.click(byId("project-units-kind-load-state"));
    fireEvent.click(byId("project-units-create"));
    expect(hook.result.current.project.blankProjectUnitsRequest).toBeNull();
    await waitFor(() => expect(hook.result.current.model.model?.pressure_contract).toBeDefined());
    expect(hook.result.current.model.model?.reference_configurations).toEqual([]);
  });
});

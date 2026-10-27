// A8, the displacement extremes panel (DEL-07-05 Design `displacement-extremes.md` §5 to §7, §9):
// the display rule, the case chooser, the filter, the sort, paging, node selection and the
// render-cost contract. Invented data only (`resultOverlayFixtures`).
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import type { MechanicsResult, PreviewModel } from "../../types";
import { supportLoadWithheldStateText } from "./supportLoadSummary";
import {
  DISPLACEMENT_EXTREMES_BOUNDARY_TEXT,
  DISPLACEMENT_EXTREMES_NO_CASE,
  DISPLACEMENT_EXTREMES_PAGE,
  DisplacementExtremesPanel
} from "./DisplacementExtremesPanel";
import { DISPLACEMENT_ENVELOPE_REASON, DISPLACEMENT_NO_RESULT, FILTER_HANGERS, familyFilterId } from "./displacementExtremes";

afterEach(cleanup);

type Row = MechanicsResult["results"][number];
const L1 = { ref_type: "load_case", ref_id: "load:L-1" } as const;
const L2 = { ref_type: "load_case", ref_id: "load:L-2" } as const;
const PASSING = healthStanding({ eligible: true });

function model(): PreviewModel {
  const base = overlayModel();
  return {
    ...base,
    supports: [
      { id: "support:S-A", label: "Anchor A", node: "node:A", family: "anchor", restraints: [] },
      { id: "support:H-C", label: "Hanger C", node: "node:C", family: "spring_hanger", restraints: [] }
    ]
  } as unknown as PreviewModel;
}

const META = { component: "invented", coordinate_system: "global", location: "node", basis: "invented", sign_convention: "invented" };
const move = (id: string, axis: "x" | "y" | "z", node: string, value: number, unit = "mm", basis: Row["basis_ref"] = L1, prefix = "displacement") =>
  row(id, `global_nodal_${prefix}_${axis}`, node, value, unit, { basis_ref: basis, metadata: META });

function rows(): Row[] {
  return [
    move("ux:A:1", "x", "node:A", 1234.5678),
    move("ux:A:2", "x", "node:A", -2345.6789, "mm", L2),
    move("uy:B:1", "y", "node:B", 0.000123456789),
    move("uy:C:2", "y", "node:C", -7, "mm", L2),
    move("rz:C:1", "z", "node:C", 0.0123456789, "rad", L1, "rotation")
  ];
}

describe("displacement extremes panel: display rule", () => {
  it("shows values under the report's display rule in cells, titles and headers, never a stored value at full precision", () => {
    const view = render(<DisplacementExtremesPanel model={model()} result={overlayResult(rows())} standing={PASSING} />);
    expect(screen.getByTestId("displacement-extremes-column-ux").textContent).toBe("ux (mm)");
    expect(screen.getByTestId("displacement-extremes-column-rz").textContent).toBe("rz (rad)");
    const max = screen.getByTestId("displacement-extremes-cell-node:A-ux-max");
    expect(max.textContent).toBe("1234.57Invented one (load:L-1)");
    expect(max.querySelector("span[title]")?.getAttribute("title")).toBe("1234.57 mm, Invented one (load:L-1) (ux:A:1)");
    expect(screen.getByTestId("displacement-extremes-cell-node:A-ux-min").textContent).toBe("-2345.68Invented two (load:L-2)");
    expect(screen.getByTestId("displacement-extremes-cell-node:B-uy-max").textContent).toBe("0.000123457Invented one (load:L-1)");
    expect(screen.getByTestId("displacement-extremes-cell-node:C-rz-max").textContent).toBe("0.0123457Invented one (load:L-1)");
    expect(screen.getByTestId("displacement-extremes-cell-node:D-ux-max").textContent).toBe("—none");
    const markup = view.container.innerHTML;
    for (const stored of ["1234.5678", "2345.6789", "0.000123456789", "0.0123456789"]) expect(markup).not.toContain(stored);
    expect(screen.getByTestId("displacement-extremes-exclusions").textContent).toBe("Maximum and minimum over 2 of 2 cases.");
    expect(screen.getByText(DISPLACEMENT_EXTREMES_BOUNDARY_TEXT)).toBeTruthy();
  });

  it("puts the unit in each cell when a column's units differ, and states a node's mixed units", () => {
    render(<DisplacementExtremesPanel model={model()} result={overlayResult([...rows(), move("ux:B:m", "x", "node:B", 0.5, "m"), move("ux:C:mm", "x", "node:C", 3), move("ux:A:m", "x", "node:A", 2, "m", L2)].filter((item) => item.id !== "ux:A:2"))} standing={PASSING} />);
    expect(screen.getByTestId("displacement-extremes-column-ux").textContent).toBe("ux");
    expect(screen.getByTestId("displacement-extremes-cell-node:B-ux-max").textContent).toBe("0.5 mInvented one (load:L-1)");
    expect(screen.getByTestId("displacement-extremes-cell-node:A-ux-max").textContent).toBe("Units differ (m, mm); no extreme is formed");
  });
});

describe("displacement extremes panel: cases, standing and filter", () => {
  it("chooses cases, disables withheld and envelope cases with their reasons, and counts them", () => {
    const envelope = move("env", "x", "node:A", 99, "mm", { ref_type: "combination", ref_id: "combination:ENV" });
    envelope.metadata = { ...META, basis: "explicit_user_range_envelope" };
    const standing = healthStanding({ eligible: true, states: { "load:L-1": "checks_passed", "load:L-2": "sensitive" } });
    render(<DisplacementExtremesPanel model={model()} result={overlayResult([...rows(), envelope])} standing={standing} />);
    const l2 = screen.getByTestId("displacement-extremes-case-load_case:load:L-2") as HTMLInputElement;
    expect(l2.disabled).toBe(true);
    expect(l2.checked).toBe(false);
    expect(screen.getByTestId("displacement-extremes-case-reason-load_case:load:L-2").textContent).toBe(` ${supportLoadWithheldStateText("sensitive")} (2 rows)`);
    expect(l2.getAttribute("aria-describedby")).toBe("displacement-extremes-case-reason-load_case:load:L-2");
    // The envelope combination has no combination state in this standing, so it is withheld first.
    expect((screen.getByTestId("displacement-extremes-case-combination:combination:ENV") as HTMLInputElement).disabled).toBe(true);
    expect(screen.getByTestId("displacement-extremes-exclusions").textContent).toBe("Maximum and minimum over 1 of 3 cases; not included: 2 withheld by numerical standing (3 rows).");
    expect(screen.getByTestId("displacement-extremes-cell-node:A-ux-min").textContent).toBe("1234.57Invented one (load:L-1)");
    const l1 = screen.getByTestId("displacement-extremes-case-load_case:load:L-1") as HTMLInputElement;
    fireEvent.click(l1);
    expect(screen.getByTestId("displacement-extremes-no-case").textContent).toBe(DISPLACEMENT_EXTREMES_NO_CASE);
    expect(screen.queryByTestId("displacement-extremes-table")).toBeNull();
    fireEvent.click(screen.getByTestId("displacement-extremes-cases-all"));
    expect(screen.getByTestId("displacement-extremes-table")).toBeTruthy();
  });

  it("labels an admitted range envelope and keeps it out", () => {
    const envelope = move("env", "x", "node:A", 99, "mm", { ref_type: "combination", ref_id: "combination:ENV" });
    envelope.metadata = { ...META, basis: "explicit_user_range_envelope" };
    render(<DisplacementExtremesPanel model={model()} result={overlayResult([...rows(), envelope])} standing={healthStanding({ eligible: true, route: "pressure_source" })} />);
    expect(screen.getByTestId("displacement-extremes-case-reason-combination:combination:ENV").textContent).toBe(` ${DISPLACEMENT_ENVELOPE_REASON}`);
    expect(screen.getByTestId("displacement-extremes-envelope-note")).toBeTruthy();
    expect(screen.getByTestId("displacement-extremes-exclusions").textContent).toBe("Maximum and minimum over 2 of 3 cases; not included: 1 range envelope.");
    expect(screen.getByTestId("displacement-extremes-cell-node:A-ux-max").textContent).toBe("1234.57Invented one (load:L-1)");
  });

  it("withholds everything for an ineligible result and says no case is admitted", () => {
    render(<DisplacementExtremesPanel model={model()} result={overlayResult(rows())} standing={healthStanding({ eligible: false })} />);
    expect(screen.getByTestId("displacement-extremes-no-case").textContent).toBe(DISPLACEMENT_EXTREMES_NO_CASE);
    expect(screen.getByTestId("displacement-extremes-standing")).toBeTruthy();
  });

  it("filters to the nodes of a support kind and names the supports", () => {
    render(<DisplacementExtremesPanel model={model()} result={overlayResult(rows())} standing={PASSING} />);
    const filter = screen.getByTestId("displacement-extremes-filter") as HTMLSelectElement;
    expect([...filter.options].map((option) => option.textContent)).toEqual(["All nodes (4)", "Support nodes (2)", "Hangers (1)", "Anchor (1)", "Spring hanger (1)"]);
    fireEvent.change(filter, { target: { value: FILTER_HANGERS } });
    const table = screen.getByTestId("displacement-extremes-table");
    expect(table.getAttribute("aria-label")).toBe("Displacement and rotation extremes: Hangers");
    expect(within(table).getAllByRole("row").filter((item) => item.getAttribute("data-testid")?.startsWith("displacement-extremes-row-")).map((item) => item.getAttribute("data-testid"))).toEqual(["displacement-extremes-row-node:C"]);
    expect(within(table).getByText("Hanger C: Spring hanger")).toBeTruthy();
    fireEvent.change(filter, { target: { value: familyFilterId("anchor") } });
    expect(screen.getByTestId("displacement-extremes-row-node:A")).toBeTruthy();
    expect(screen.queryByTestId("displacement-extremes-row-node:C")).toBeNull();
  });

  it("sorts by a component's largest magnitude with aria-sort", () => {
    render(<DisplacementExtremesPanel model={model()} result={overlayResult(rows())} standing={PASSING} />);
    fireEvent.change(screen.getByTestId("displacement-extremes-sort"), { target: { value: "uy" } });
    expect(screen.getByTestId("displacement-extremes-column-uy").getAttribute("aria-sort")).toBe("descending");
    expect(screen.getByTestId("displacement-extremes-column-ux").getAttribute("aria-sort")).toBeNull();
    const order = [...screen.getByTestId("displacement-extremes-table").querySelectorAll("tbody tr")].map((item) => item.getAttribute("data-testid"));
    expect(order).toEqual(["displacement-extremes-row-node:C", "displacement-extremes-row-node:B", "displacement-extremes-row-node:A", "displacement-extremes-row-node:D"]);
  });

  it("states the unavailable state under a custom test-id prefix", () => {
    render(<DisplacementExtremesPanel model={model()} result={null} standing={PASSING} testIdPrefix="a6-disp" />);
    expect(screen.getByTestId("a6-disp-unavailable").textContent).toBe(DISPLACEMENT_NO_RESULT);
    expect(screen.getByTestId("a6-disp-panel")).toBeTruthy();
  });
});

describe("displacement extremes panel: paging and selection", () => {
  const count = DISPLACEMENT_EXTREMES_PAGE * 2 + 5;
  function large(): { model: PreviewModel; result: MechanicsResult } {
    const nodes = Array.from({ length: count }, (_, index) => ({ id: `node:N${index}`, label: "", position: { x: index, y: 0, z: 0 } }));
    return {
      model: { ...overlayModel(), nodes, supports: [] } as unknown as PreviewModel,
      result: overlayResult(nodes.map((node, index) => move(`ux:${index}`, "x", node.id, index)))
    };
  }
  const shownRows = () => screen.getByTestId("displacement-extremes-table").querySelectorAll("tbody tr").length;

  it("renders one page of rows at most and steps through the pages", () => {
    const { model: big, result } = large();
    render(<DisplacementExtremesPanel model={big} result={result} standing={PASSING} />);
    expect(shownRows()).toBe(DISPLACEMENT_EXTREMES_PAGE);
    expect(screen.getByTestId("displacement-extremes-page").textContent).toBe(`Nodes 1–${DISPLACEMENT_EXTREMES_PAGE} of ${count}`);
    expect((screen.getByTestId("displacement-extremes-page-previous") as HTMLButtonElement).disabled).toBe(true);
    fireEvent.click(screen.getByTestId("displacement-extremes-page-next"));
    fireEvent.click(screen.getByTestId("displacement-extremes-page-next"));
    expect(shownRows()).toBe(5);
    expect(screen.getByTestId("displacement-extremes-page").textContent).toBe(`Nodes ${DISPLACEMENT_EXTREMES_PAGE * 2 + 1}–${count} of ${count}`);
    expect((screen.getByTestId("displacement-extremes-page-next") as HTMLButtonElement).disabled).toBe(true);
    // A new sort lists from the first page again.
    fireEvent.change(screen.getByTestId("displacement-extremes-sort"), { target: { value: "ux" } });
    expect(screen.getByTestId("displacement-extremes-page").textContent).toBe(`Nodes 1–${DISPLACEMENT_EXTREMES_PAGE} of ${count}`);
    expect(screen.getByTestId("displacement-extremes-table").querySelector("tbody tr")?.getAttribute("data-testid")).toBe(`displacement-extremes-row-node:N${count - 1}`);
  });

  it("selects a node by click or keyboard, marks the selected node and shows its page", () => {
    const { model: big, result } = large();
    const onSelectNode = vi.fn();
    const view = render(<DisplacementExtremesPanel model={big} result={result} standing={PASSING} onSelectNode={onSelectNode} />);
    const button = screen.getByRole("button", { name: "Select node node:N3" });
    fireEvent.click(button);
    expect(onSelectNode).toHaveBeenLastCalledWith("node:N3");
    button.focus();
    expect(document.activeElement).toBe(button);
    expect(button.tagName).toBe("BUTTON");
    view.rerender(<DisplacementExtremesPanel model={big} result={result} standing={PASSING} onSelectNode={onSelectNode} selectedNodeId={`node:N${count - 2}`} />);
    expect(screen.getByTestId("displacement-extremes-page").textContent).toBe(`Nodes ${DISPLACEMENT_EXTREMES_PAGE * 2 + 1}–${count} of ${count}`);
    const selected = screen.getByTestId(`displacement-extremes-row-node:N${count - 2}`);
    expect(selected.getAttribute("aria-current")).toBe("true");
    expect(selected.className).toBe("selected-result-row");
    // The user may page away from the selection; it is applied again only when it changes.
    fireEvent.click(screen.getByTestId("displacement-extremes-page-previous"));
    view.rerender(<DisplacementExtremesPanel model={big} result={result} standing={PASSING} onSelectNode={onSelectNode} selectedNodeId={`node:N${count - 2}`} />);
    expect(screen.getByTestId("displacement-extremes-page").textContent).toBe(`Nodes ${DISPLACEMENT_EXTREMES_PAGE + 1}–${DISPLACEMENT_EXTREMES_PAGE * 2} of ${count}`);
  });

  it("shows node names as text when no selection handler is given", () => {
    render(<DisplacementExtremesPanel model={model()} result={overlayResult(rows())} standing={PASSING} />);
    expect(screen.queryByRole("button", { name: /^Select node/ })).toBeNull();
  });
});

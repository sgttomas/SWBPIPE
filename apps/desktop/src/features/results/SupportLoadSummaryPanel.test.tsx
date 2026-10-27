// A2, the per-support load summary panel (DEL-07-05 Design `support-load-summary.md` §5, §6, §8):
// the table, the display rule, withheld rows, stepping by button, key and focus, and the
// unavailable state. Invented data only (`resultOverlayFixtures`).
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { healthStanding, overlayModel, overlayResult, reaction, row } from "../../test-support/resultOverlayFixtures";
import type { MechanicsResult } from "../../types";
import { SUPPORT_LOAD_ALLOWABLES_TEXT, SupportLoadSummaryPanel } from "./SupportLoadSummaryPanel";
import { SUPPORT_LOAD_NO_RESULT, SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE, supportLoadWithheldStateText } from "./supportLoadSummary";

afterEach(cleanup);

type Row = MechanicsResult["results"][number];
const L2 = { ref_type: "load_case", ref_id: "load:L-2" } as const;

function rows(): Row[] {
  const forSupport = (support: string, scale: number, basis?: Row["basis_ref"]) => (["Fx", "Fy", "Fz", "Mx", "My", "Mz"] as const).map((component, index) =>
    reaction(support, component, (index + 1) * scale, index < 3 ? "N" : "N*m", { id: `r:${support}:${component}:${basis?.ref_id ?? "L-1"}`, ...(basis ? { basis_ref: basis } : {}) }));
  return [
    ...forSupport("support:S-A", 1234.5678),
    ...forSupport("support:S-A", -0.5, L2),
    row("uA", "global_nodal_displacement_x", "node:A", 1.23456789, "mm"),
    ...forSupport("support:S-C", 10)
  ];
}

const PASSING = healthStanding({ eligible: true });

function renderPanel(props: Partial<Parameters<typeof SupportLoadSummaryPanel>[0]> = {}) {
  return render(<SupportLoadSummaryPanel model={overlayModel()} result={overlayResult(rows())} standing={PASSING} {...props} />);
}

describe("SupportLoadSummaryPanel", () => {
  it("tabulates one support's cases with the report display rule and units in the headers", () => {
    renderPanel();
    const table = screen.getByRole("table", { name: "Loads and movements at support support:S-A" });
    expect(within(table).getByRole("columnheader", { name: "Fx (N)" })).toBeTruthy();
    expect(within(table).getByRole("columnheader", { name: "Mz (N*m)" })).toBeTruthy();
    expect(within(table).getByRole("columnheader", { name: "|F|" })).toBeTruthy();
    expect(within(table).getByRole("rowheader", { name: "Invented one (load:L-1)" })).toBeTruthy();
    // 1234.5678 to six significant digits; 6 × 1234.5678 = 7407.4068.
    expect(screen.getByTestId("support-load-cell-load_case:load:L-1-Fx").textContent).toBe("1234.57");
    expect(screen.getByTestId("support-load-cell-load_case:load:L-1-Mz").textContent).toBe("7407.41");
    expect(screen.getByTestId("support-load-cell-load_case:load:L-1-ux").textContent).toBe("1.23457");
    expect(screen.getByTestId("support-load-cell-load_case:load:L-2-Fx").textContent).toBe("-0.5");
    expect(screen.getByTestId("support-load-cell-load_case:load:L-2-ux").textContent).toBe("—not emitted");
    expect(screen.getByTestId("support-load-extreme-max-Fx").textContent).toBe("1234.57Invented one (load:L-1)");
    expect(screen.getByTestId("support-load-extreme-min-Fx").textContent).toBe("-0.5Invented two (load:L-2)");
    expect(screen.getByTestId("support-load-extremes-note").textContent).toBe("Maximum and minimum over 2 of 2 rows.");
    expect(screen.getByTestId("support-load-context").textContent).toContain("Frame: global. Sign convention: support-on-pipe");
    expect(screen.getByTestId("support-load-allowables").textContent).toBe(SUPPORT_LOAD_ALLOWABLES_TEXT);
  });

  it("formats every value, its title and the extremes' titles by the display rule; no stored value shows raw", () => {
    // Invented: Fx rounds (1234.5678 N), Fy is stated in Pa (shown in MPa), Fz mixes units so
    // its cells carry their own units and it has no extreme.
    const stated: Row[] = [
      reaction("support:S-A", "Fx", 1234.5678, "N", { id: "r:Fx:1" }),
      reaction("support:S-A", "Fx", -0.5, "N", { id: "r:Fx:2", basis_ref: L2 }),
      reaction("support:S-A", "Fy", 123456789.5, "Pa", { id: "r:Fy:1" }),
      reaction("support:S-A", "Fy", 2500000, "Pa", { id: "r:Fy:2", basis_ref: L2 }),
      reaction("support:S-A", "Fz", 98765.4321, "N", { id: "r:Fz:1" }),
      reaction("support:S-A", "Fz", 0.123456789, "Pa", { id: "r:Fz:2", basis_ref: L2 }),
      row("uA", "global_nodal_displacement_x", "node:A", 1.23456789, "mm")
    ];
    const view = render(<SupportLoadSummaryPanel model={overlayModel()} result={overlayResult(stated)} standing={PASSING} />);
    const title = (testId: string) => screen.getByTestId(testId).querySelector("span[title]")?.getAttribute("title");
    const cell = (key: string, column: string) => `support-load-cell-load_case:load:${key}-${column}`;
    expect(screen.getByRole("columnheader", { name: "Fy (MPa)" })).toBeTruthy();
    expect(screen.getByRole("columnheader", { name: "Fz" })).toBeTruthy();
    expect(screen.getByTestId(cell("L-1", "Fx")).textContent).toBe("1234.57");
    expect(title(cell("L-1", "Fx"))).toBe("1234.57 N (r:Fx:1)");
    expect(screen.getByTestId(cell("L-1", "Fy")).textContent).toBe("123.457");
    expect(title(cell("L-1", "Fy"))).toBe("123.457 MPa (r:Fy:1)");
    expect(screen.getByTestId(cell("L-2", "Fy")).textContent).toBe("2.5");
    expect(title(cell("L-2", "Fy"))).toBe("2.5 MPa (r:Fy:2)");
    expect(screen.getByTestId(cell("L-1", "Fz")).textContent).toBe("98765.4 N");
    expect(title(cell("L-1", "Fz"))).toBe("98765.4 N (r:Fz:1)");
    expect(screen.getByTestId(cell("L-2", "Fz")).textContent).toBe("1.23457e-7 MPa");
    expect(title(cell("L-2", "Fz"))).toBe("1.23457e-7 MPa (r:Fz:2)");
    expect(title(cell("L-1", "ux"))).toBe("1.23457 mm (uA)");
    expect(title("support-load-extreme-max-Fx")).toBe("1234.57 N (r:Fx:1)");
    expect(title("support-load-extreme-min-Fx")).toBe("-0.5 N (r:Fx:2)");
    expect(screen.getByTestId("support-load-extreme-max-Fy").textContent).toBe("123.457Invented one (load:L-1)");
    expect(title("support-load-extreme-max-Fy")).toBe("123.457 MPa (r:Fy:1)");
    expect(title("support-load-extreme-min-Fy")).toBe("2.5 MPa (r:Fy:2)");
    expect(title("support-load-extreme-max-ux")).toBe("1.23457 mm (uA)");
    expect(screen.getByTestId("support-load-extreme-max-Fz").textContent).toBe("Units differ (N, Pa); no extreme is formed");
    // Nowhere in the panel's markup, text or attributes, does a stored value appear at full precision.
    const markup = view.container.innerHTML;
    for (const raw of ["1234.5678", "123456789", "2500000", "98765.4321", "1.23456789"]) expect(markup).not.toContain(raw);
  });

  it("shows a withheld case's reason and none of its values", () => {
    renderPanel({ standing: healthStanding({ eligible: true, states: { "load:L-1": "checks_passed", "load:L-2": "sensitive" } }) });
    expect(screen.getByTestId("support-load-withheld-load_case:load:L-2").textContent).toBe(supportLoadWithheldStateText("sensitive"));
    expect(screen.queryByTestId("support-load-cell-load_case:load:L-2-Fx")).toBeNull();
    expect(screen.getByTestId("support-load-extreme-min-Fx").textContent).toBe("1234.57Invented one (load:L-1)");
    expect(screen.getByTestId("support-load-extremes-note").textContent).toBe("Maximum and minimum over 1 of 2 rows; not included: 1 withheld by numerical standing.");
  });

  it("withholds everything at ineligible standing and repeats the health header", () => {
    renderPanel({ standing: healthStanding({ eligible: false }) });
    expect(screen.getByTestId("support-load-withheld-load_case:load:L-1").textContent).toBe(SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE);
    expect(screen.getByTestId("support-load-standing").textContent).toContain("Numerical health");
    expect(screen.getByTestId("support-load-extreme-max-Fx").textContent).toBe("—none");
  });

  it("steps through the supports by button and by Alt+Arrow, and states the position", () => {
    renderPanel();
    expect(screen.getByTestId("support-load-position").textContent).toBe("Support 1 of 3");
    expect((screen.getByRole("button", { name: "Previous support" }) as HTMLButtonElement).disabled).toBe(true);
    fireEvent.click(screen.getByRole("button", { name: "Next support" }));
    expect(screen.getByTestId("support-load-position").textContent).toBe("Support 2 of 3");
    expect(screen.getByTestId("support-load-cell-load_case:load:L-1-Fx").textContent).toBe("10");
    fireEvent.keyDown(screen.getByTestId("support-load-summary"), { key: "ArrowRight", altKey: true });
    expect(screen.getByTestId("support-load-position").textContent).toBe("Support 3 of 3");
    expect(screen.getByTestId("support-load-absence-load_case:load:L-1").textContent).toBe("No reaction rows");
    expect((screen.getByRole("button", { name: "Next support" }) as HTMLButtonElement).disabled).toBe(true);
    fireEvent.keyDown(screen.getByTestId("support-load-summary"), { key: "ArrowRight", altKey: true });
    expect(screen.getByTestId("support-load-position").textContent).toBe("Support 3 of 3");
    fireEvent.keyDown(screen.getByTestId("support-load-summary"), { key: "ArrowLeft" });
    expect(screen.getByTestId("support-load-position").textContent).toBe("Support 3 of 3");
    fireEvent.keyDown(screen.getByTestId("support-load-summary"), { key: "ArrowLeft", altKey: true });
    expect(screen.getByTestId("support-load-position").textContent).toBe("Support 2 of 3");
    fireEvent.change(screen.getByRole("combobox", { name: "Support" }), { target: { value: "support:S-A" } });
    expect(screen.getByTestId("support-load-position").textContent).toBe("Support 1 of 3");
  });

  it("follows a focused support, such as the session's selection", () => {
    const view = renderPanel({ focusSupportId: "support:S-C" });
    expect(screen.getByTestId("support-load-position").textContent).toBe("Support 2 of 3");
    view.rerender(<SupportLoadSummaryPanel model={overlayModel()} result={overlayResult(rows())} standing={PASSING} focusSupportId="support:S-D" />);
    expect(screen.getByTestId("support-load-position").textContent).toBe("Support 3 of 3");
    view.rerender(<SupportLoadSummaryPanel model={overlayModel()} result={overlayResult(rows())} standing={PASSING} focusSupportId="node:A" />);
    expect(screen.getByTestId("support-load-position").textContent).toBe("Support 3 of 3");
  });

  it("states why nothing is tabulated, under its own test-id prefix", () => {
    renderPanel({ result: null, testIdPrefix: "key-results-support-load" });
    expect(screen.getByTestId("key-results-support-load-unavailable").textContent).toBe(SUPPORT_LOAD_NO_RESULT);
    expect(screen.queryByRole("table")).toBeNull();
  });
});

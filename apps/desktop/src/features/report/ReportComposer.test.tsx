import { useState } from "react";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import type { MechanicsResult } from "../../types";
import { ReportComposer } from "./ReportComposer";
import { optionalReportComposition, DEFAULT_REPORT_COMPOSITION_SECTION, type ReportComposition } from "./reportComposition";

afterEach(cleanup);

const result = { results: [
  { id: "a", kind: "global_nodal_displacement_x", value: 1, unit: "m", entity_ref: "node:1", basis_ref: { ref_type: "load_case", ref_id: "a" } },
  { id: "b", kind: "pipe_lame_hoop_stress_v2", value: 2, unit: "Pa", entity_ref: "pipe:1", basis_ref: { ref_type: "load_case", ref_id: "b" } }
] } as MechanicsResult;

function Composer() {
  const [composition, setComposition] = useState<ReportComposition | null>(null);
  return <><ReportComposer composition={composition} result={result} onChange={setComposition} />
    <output data-testid="composition">{JSON.stringify(composition)}</output></>;
}
function current() { return JSON.parse(screen.getByTestId("composition").textContent!) as ReportComposition | null; }

describe("session report composer", () => {
  it("leaves the default absent and can explicitly omit every optional subsection", () => {
    expect(optionalReportComposition([DEFAULT_REPORT_COMPOSITION_SECTION])).toBeNull();
    render(<Composer />);
    expect(current()).toBeNull();
    fireEvent.click(screen.getByRole("checkbox", { name: "All result rows" }));
    expect(current()?.sections).toEqual([]);
    expect(screen.getByText("Result rows shown: 0 of 2.")).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Reset report sections" }));
    expect(current()).toBeNull();
  });

  it("excludes overlap and supports keyboard-accessible ordering and view controls", () => {
    render(<Composer />);
    fireEvent.click(screen.getByRole("checkbox", { name: "Displacements" }));
    fireEvent.click(screen.getByRole("checkbox", { name: "Stresses" }));
    expect(current()?.sections.map((section) => section.section_id)).toEqual(["result_displacements", "result_stresses"]);
    fireEvent.click(screen.getByRole("button", { name: "Move Stresses up" }));
    expect(current()?.sections.map((section) => section.section_id)).toEqual(["result_stresses", "result_displacements"]);
    fireEvent.change(screen.getByLabelText("View for Stresses"), { target: { value: "largest" } });
    fireEvent.change(screen.getByLabelText("Rows per display unit"), { target: { value: "7" } });
    expect(current()?.sections[0].view).toEqual({ kind: "largest", limit: 7 });
    fireEvent.click(screen.getByRole("checkbox", { name: "All result rows" }));
    expect(current()).toBeNull();
  });

  it("retains unknown cases visibly after re-solve so they cannot become an empty filter", () => {
    const composition: ReportComposition = { composition_version: "1.0.0", sections: [
      { section_id: "result_stresses", cases: ["load_case:retired"], view: { kind: "all" } }
    ] };
    render(<ReportComposer composition={composition} result={result} onChange={() => {}} />);
    const retired = screen.getByRole("option", { name: "load_case:retired (not in current result)" }) as HTMLOptionElement;
    expect(retired.selected).toBe(true);
  });
});

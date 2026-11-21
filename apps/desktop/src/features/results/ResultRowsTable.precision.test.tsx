// The result rows print every number by the report's display rule (six significant digits, half
// away from zero): the Value column in the displayed unit and the Entered column in the stored
// unit. The full value is each cell's hover text; the row itself is not changed.
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { CASE_NOT_STATED, ResultRowsTable } from "./ResultRowsTable";
import type { MechanicsResult } from "../../types";

afterEach(cleanup);

describe("the result rows' precision", () => {
  it("prints the entered value to six significant digits and keeps the full value as its hover text", () => {
    const row = { id: "r:uz", kind: "global_nodal_displacement_z", entity_ref: "node:N-100", value: -6.633480541462957, unit: "mm", basis_ref: { ref_type: "load_case", ref_id: "load:L-100" }, metadata: {} };
    const items = [row] as unknown as MechanicsResult["results"];
    const before = JSON.stringify(items);
    render(
      <ResultRowsTable csvKind="t" items={items} selectedResultId={null} onSelectResult={() => {}} entityName={(ref) => ref} caseName={(basis) => basis?.ref_id ?? CASE_NOT_STATED}
        recoveredCases={new Set()} dimension={() => "length"} frameText={() => ""} notes={() => null} sort={null} onSort={() => {}} sortDisabledReason={null} />
    );
    const entered = screen.getByTestId("result-row-dual-r:uz");
    expect(entered.textContent).toBe("Entered: -6.63348 mm");
    expect(entered.getAttribute("title")).toBe("-6.633480541462957 mm");
    expect(screen.getByTestId("result-row-r:uz").textContent).not.toContain("6.633480541462957");
    expect(JSON.stringify(items)).toBe(before);
  });
});

import { cleanup, render, screen, within } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import type { MechanicsResult } from "../../types";
import { hasNativeMechanicsInvocation } from "../../services/previewService";
import { PhysicsSourceEvidencePanel, endpointLocationDescription } from "./PhysicsSourceEvidencePanel";

afterEach(cleanup);
const fixtureSources = import.meta.glob("../../../../../fixtures/product_preview/physics_source/*.raw.json", { query: "?raw", import: "default", eager: true }) as Record<string,string>;
const fixture = (name: string) => JSON.parse(fixtureSources[`../../../../../fixtures/product_preview/physics_source/${name}.raw.json`]) as MechanicsResult;
it.each(["n05", "n06", "mixed", "fields"])('shows %s recorded methods and bounded maximum without promoting imported data', name => {
  for (const mode of ["sparse_interactive", "dense_scrutiny"]) {
    const result = fixture(`${name}-${mode}`), before = JSON.stringify(result);
    const view = render(<PhysicsSourceEvidencePanel result={result} />);
    const panel = screen.getByRole("region", { name: "Recorded recovery evidence" });
    expect(panel).toHaveTextContent(`Ordinary numerical attempt: ${result.numerical_quality!.status}`);
    expect(panel).toHaveTextContent("exact_straight_pressure_v2");
    expect(panel).toHaveTextContent("These recorded labels do not grant Current use");
    const physical = result.contract_evidence as any, receipt = result.source_block_recovery as any;
    expect(within(panel).getByTestId("physics-source-case-coverage")).toHaveTextContent(`Recorded cases: ${receipt.body.cases.length}; physical cases: ${physical.exact_cases.length}`);
    for (const c of receipt.body.cases) {
      const row = within(panel).getByTestId(`physics-source-case-${c.basis_ref.ref_id}`);
      expect(row).toHaveTextContent(c.selected_method);
      expect(row).toHaveTextContent(`Producer outcome: ${c.outcome}`);
    }
    for (const c of physical.exact_cases) for (const maximum of c.pipe_stress_extrema) {
      const row = within(panel).getByTestId(`physics-source-maximum-${maximum.result_id}`);
      expect(row).toHaveTextContent(`[${maximum.value_lower_pa}, ${maximum.value_upper_pa}] Pa`);
      if (maximum.basis === "retained_source_endpoint_normal_max_v1") {
        expect(row).toHaveTextContent(endpointLocationDescription(maximum.locations));
        expect(row).toHaveTextContent(`relative error bound: ${maximum.relative_error_bound}`);
      }
    }
    expect(hasNativeMechanicsInvocation(result, null)).toBe(false);
    expect(JSON.stringify(result)).toBe(before);
    view.unmount();
  }
});
it('describes constant, tied and ambiguous endpoint claims without inventing uniqueness', () => {
  // Presentation-only states, not simulated producer receipts or physical tests.
  expect(endpointLocationDescription({ kind: "whole_span_constant" })).toContain("every station");
  expect(endpointLocationDescription({ kind: "strict_endpoint", endpoint: "j" })).toContain("does not identify a unique circumferential fibre");
  expect(endpointLocationDescription({ kind: "endpoint_candidates", exact_tie_proven: true, interior_equal_possible: false })).toContain("Both endpoints tie");
  expect(endpointLocationDescription({ kind: "endpoint_candidates", exact_tie_proven: false, interior_equal_possible: true })).toContain("equality is not proved");
  expect(endpointLocationDescription(null)).toContain("no unique governing station");
});

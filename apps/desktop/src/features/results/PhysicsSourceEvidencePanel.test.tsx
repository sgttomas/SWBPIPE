import { cleanup, render, screen, within } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import type { MechanicsResult } from "../../types";
import { hasNativeMechanicsInvocation } from "../../services/previewService";
import { PhysicsSourceEvidencePanel, endpointLocationDescription, maximumIntervalReading, qualifiedIntervalDescription } from "./PhysicsSourceEvidencePanel";

afterEach(cleanup);
// B7b: the composite is pressure-source-1 (3.0.0/exact_pressure_v3), the v3 twin of the
// retired physics-source-1; its committed producer raws.
const fixtureSources = import.meta.glob("../../../../../fixtures/product_preview/pressure_source/physics_source/*.raw.json", { query: "?raw", import: "default", eager: true }) as Record<string,string>;
const fixture = (name: string) => JSON.parse(fixtureSources[`../../../../../fixtures/product_preview/pressure_source/physics_source/${name}.raw.json`]) as MechanicsResult;
it.each(["n05", "n06", "mixed", "fields"])('shows %s recorded methods and bounded maximum without promoting imported data', name => {
  let ordinaryMaxima = 0;
  for (const mode of ["sparse_interactive", "dense_scrutiny"]) {
    const result = fixture(`${name}-${mode}`), before = JSON.stringify(result);
    const view = render(<PhysicsSourceEvidencePanel result={result} />);
    const panel = screen.getByRole("region", { name: "Recorded recovery evidence" });
    expect(panel).toHaveTextContent(`Ordinary numerical attempt: ${result.numerical_quality!.status}`);
    expect(panel).toHaveTextContent("exact_pressure_v3");
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
        expect(maximum.enclosure_scope).toBe("retained_action_projection_and_ordered_binary64_recipe;represented_source_section_operands");
        expect(row).toHaveTextContent(`Bound: [${maximum.value_lower_pa}, ${maximum.value_upper_pa}] Pa.`);
        expect(row).toHaveTextContent(endpointLocationDescription(maximum.locations));
        expect(row).toHaveTextContent(`relative error bound: ${maximum.relative_error_bound}`);
        // The retained endpoint method keeps its own explanation of what its bound covers.
        expect(row).toHaveTextContent("The bound covers retained-action projection and ordered arithmetic with the represented section operands.");
        expect(row).not.toHaveTextContent("polynomial coefficient enclosures");
        expect(within(row).queryByTestId(`physics-source-maximum-interval-${maximum.result_id}`)).toBeNull();
      } else {
        ordinaryMaxima += 1;
        // The interval and the record's qualification of it are one sentence.
        expect(maximum.enclosure_scope).toBe("supplied_binary64_polynomial_coefficient_enclosures; solution error is separate");
        expect(within(row).getByTestId(`physics-source-maximum-interval-${maximum.result_id}`).textContent)
          .toBe(`Bound over the supplied binary64 polynomial coefficient enclosures: [${maximum.value_lower_pa}, ${maximum.value_upper_pa}] Pa. Solution error is separate.`);
        expect(row).not.toHaveTextContent("physical enclosure");
        expect(row).not.toHaveTextContent("retained-action projection");
      }
    }
    expect(hasNativeMechanicsInvocation(result, null)).toBe(false);
    expect(JSON.stringify(result)).toBe(before);
    view.unmount();
  }
  // The mixed fixture is the one that holds ordinary stress maxima.
  expect(ordinaryMaxima).toBe(name === "mixed" ? 4 : 0);
});
const POLYNOMIAL_SCOPE = "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate";
const RETAINED_BASIS = "retained_source_endpoint_normal_max_v1";
const RETAINED_SCOPE = "retained_action_projection_and_ordered_binary64_recipe;represented_source_section_operands";
const uninterpreted = (maximum: any) => `Recorded interval: [${maximum.value_lower_pa}, ${maximum.value_upper_pa}] Pa. This panel does not interpret the scope this record gives for the interval, so it is not shown as a bound.`;
it('shows no interval as a bound unless the record states the scope its explanation is for', () => {
  const result = fixture("mixed-sparse_interactive"), maxima = (result.contract_evidence as any).exact_cases.flatMap((c: any) => c.pipe_stress_extrema);
  const ordinary = maxima.filter((m: any) => m.basis === undefined), retained = maxima.filter((m: any) => m.basis === RETAINED_BASIS);
  expect(ordinary).toHaveLength(2);
  expect(retained).toHaveLength(2);
  // Edited records: presentation states, not producer records. An ordinary record given the
  // retained basis with its polynomial scope intact, and one with no scope; a retained record
  // given the polynomial scope, and one with no scope.
  ordinary[0].basis = RETAINED_BASIS;
  delete ordinary[1].enclosure_scope;
  retained[0].enclosure_scope = POLYNOMIAL_SCOPE;
  delete retained[1].enclosure_scope;
  render(<PhysicsSourceEvidencePanel result={result} />);
  for (const maximum of [...ordinary, ...retained]) {
    const row = screen.getByTestId(`physics-source-maximum-${maximum.result_id}`);
    expect(within(row).getByTestId(`physics-source-maximum-interval-${maximum.result_id}`).textContent).toBe(uninterpreted(maximum));
    expect(row.textContent).not.toMatch(/Bound/);
    // Neither explanation is attached to an interval whose scope does not match it.
    expect(row).not.toHaveTextContent("Solution error is separate");
    expect(row).not.toHaveTextContent("The bound covers retained-action projection");
  }
});
it('reads an interval explanation only from an exactly stated scope that agrees with the basis', () => {
  expect(maximumIntervalReading({ enclosure_scope: POLYNOMIAL_SCOPE })).toBe("polynomial_enclosure");
  // The qualification stays attached under any basis but the retained one.
  expect(maximumIntervalReading({ basis: "another_basis", enclosure_scope: POLYNOMIAL_SCOPE })).toBe("polynomial_enclosure");
  expect(maximumIntervalReading({ basis: RETAINED_BASIS, enclosure_scope: RETAINED_SCOPE })).toBe("retained_endpoint");
  expect(maximumIntervalReading({ basis: RETAINED_BASIS, enclosure_scope: POLYNOMIAL_SCOPE })).toBe("uninterpreted");
  expect(maximumIntervalReading({ enclosure_scope: RETAINED_SCOPE })).toBe("uninterpreted");
  for (const scope of [undefined, null, "", POLYNOMIAL_SCOPE.toUpperCase(), `${POLYNOMIAL_SCOPE} `, "supplied_binary64_polynomial_coefficient_enclosures", 7, {}])
    for (const basis of [undefined, RETAINED_BASIS]) expect(maximumIntervalReading({ basis, enclosure_scope: scope })).toBe("uninterpreted");
  expect(qualifiedIntervalDescription({ value_lower_pa: 1.5, value_upper_pa: 2.5, enclosure_scope: POLYNOMIAL_SCOPE }))
    .toBe("Bound over the supplied binary64 polynomial coefficient enclosures: [1.5, 2.5] Pa. Solution error is separate.");
  expect(qualifiedIntervalDescription({ value_lower_pa: 1.5, value_upper_pa: 2.5 })).toBe(uninterpreted({ value_lower_pa: 1.5, value_upper_pa: 2.5 }));
  // An unreadable endpoint is shown as unavailable, with the qualification still attached.
  expect(qualifiedIntervalDescription({ value_lower_pa: "1", enclosure_scope: POLYNOMIAL_SCOPE }))
    .toBe("Bound over the supplied binary64 polynomial coefficient enclosures: [unavailable, unavailable] Pa. Solution error is separate.");
});
it('describes constant, tied and ambiguous endpoint claims without inventing uniqueness', () => {
  // Presentation-only states, not simulated producer receipts or physical tests.
  expect(endpointLocationDescription({ kind: "whole_span_constant" })).toContain("every station");
  expect(endpointLocationDescription({ kind: "strict_endpoint", endpoint: "j" })).toContain("does not identify a unique circumferential fibre");
  expect(endpointLocationDescription({ kind: "endpoint_candidates", exact_tie_proven: true, interior_equal_possible: false })).toContain("Both endpoints tie");
  expect(endpointLocationDescription({ kind: "endpoint_candidates", exact_tie_proven: false, interior_equal_possible: true })).toContain("equality is not proved");
  expect(endpointLocationDescription(null)).toContain("no unique governing station");
});

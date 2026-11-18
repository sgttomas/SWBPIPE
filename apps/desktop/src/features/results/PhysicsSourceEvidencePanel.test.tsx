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
        expect(maximum.coefficient_basis).toBe("retained_section_functionals_binary64");
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
        expect(maximum.coefficient_basis).toBe("j_side_section_equilibrium_binary64_enclosures");
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
const POLYNOMIAL_COEFFICIENTS = "j_side_section_equilibrium_binary64_enclosures";
const POLYNOMIAL_SCOPE = "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate";
const RETAINED_BASIS = "retained_source_endpoint_normal_max_v1";
const RETAINED_COEFFICIENTS = "retained_section_functionals_binary64";
const RETAINED_SCOPE = "retained_action_projection_and_ordered_binary64_recipe;represented_source_section_operands";
const uninterpreted = (maximum: any) => `Recorded interval: [${maximum.value_lower_pa}, ${maximum.value_upper_pa}] Pa. This panel cannot determine the interval's scope from this record, so it is not shown as a bound.`;
// Edited records: presentation states, not producer records. Each takes one committed record of
// the named kind and leaves its basis, coefficient basis and scope not all of one kind.
const edits: Array<[string, "ordinary" | "retained", (maximum: any) => void]> = [
  ["an ordinary record given the retained basis", "ordinary", m => { m.basis = RETAINED_BASIS; }],
  ["an ordinary record given the retained basis and scope, its coefficient basis kept", "ordinary", m => { m.basis = RETAINED_BASIS; m.enclosure_scope = RETAINED_SCOPE; }],
  ["an ordinary record given the retained coefficient basis", "ordinary", m => { m.coefficient_basis = RETAINED_COEFFICIENTS; }],
  ["an ordinary record given another basis", "ordinary", m => { m.basis = "another_basis"; }],
  ["an ordinary record with no coefficient basis", "ordinary", m => { delete m.coefficient_basis; }],
  ["an ordinary record with no scope", "ordinary", m => { delete m.enclosure_scope; }],
  ["a retained record given the polynomial scope", "retained", m => { m.enclosure_scope = POLYNOMIAL_SCOPE; }],
  ["a retained record given the polynomial scope, its basis removed", "retained", m => { delete m.basis; m.enclosure_scope = POLYNOMIAL_SCOPE; }],
  ["a retained record given the polynomial coefficient basis", "retained", m => { m.coefficient_basis = POLYNOMIAL_COEFFICIENTS; }],
  ["a retained record with no basis", "retained", m => { delete m.basis; }],
  ["a retained record with no coefficient basis", "retained", m => { delete m.coefficient_basis; }],
  ["a retained record with no scope", "retained", m => { delete m.enclosure_scope; }],
];
it.each(edits)('shows no interval as a bound for %s', (_name, kind, edit) => {
  const result = fixture("mixed-sparse_interactive"), maxima = (result.contract_evidence as any).exact_cases.flatMap((c: any) => c.pipe_stress_extrema);
  const maximum = maxima.find((m: any) => kind === "retained" ? m.basis === RETAINED_BASIS : m.basis === undefined);
  // The committed record is of its kind before the edit, and the panel reads it so.
  expect(maximumIntervalReading(maximum)).toBe(kind === "retained" ? "retained_endpoint" : "polynomial_enclosure");
  edit(maximum);
  render(<PhysicsSourceEvidencePanel result={result} />);
  const row = screen.getByTestId(`physics-source-maximum-${maximum.result_id}`);
  expect(within(row).getByTestId(`physics-source-maximum-interval-${maximum.result_id}`).textContent).toBe(uninterpreted(maximum));
  expect(row.textContent).not.toMatch(/Bound/);
  // Neither explanation is attached to an interval the record's fields do not agree on.
  expect(row).not.toHaveTextContent("Solution error is separate");
  expect(row).not.toHaveTextContent("The bound covers retained-action projection");
});
it('reads an interval explanation only from a basis, coefficient basis and scope that all belong to one kind', () => {
  const polynomial = { coefficient_basis: POLYNOMIAL_COEFFICIENTS, enclosure_scope: POLYNOMIAL_SCOPE };
  const retained = { basis: RETAINED_BASIS, coefficient_basis: RETAINED_COEFFICIENTS, enclosure_scope: RETAINED_SCOPE };
  expect(maximumIntervalReading(polynomial)).toBe("polynomial_enclosure");
  expect(maximumIntervalReading(retained)).toBe("retained_endpoint");
  // Every other combination of the three fields' known values is uninterpreted.
  for (const basis of [undefined, RETAINED_BASIS, "another_basis"]) for (const coefficient_basis of [undefined, POLYNOMIAL_COEFFICIENTS, RETAINED_COEFFICIENTS]) for (const enclosure_scope of [undefined, POLYNOMIAL_SCOPE, RETAINED_SCOPE]) {
    const record = { basis, coefficient_basis, enclosure_scope };
    const expected = basis === undefined && coefficient_basis === POLYNOMIAL_COEFFICIENTS && enclosure_scope === POLYNOMIAL_SCOPE ? "polynomial_enclosure"
      : basis === RETAINED_BASIS && coefficient_basis === RETAINED_COEFFICIENTS && enclosure_scope === RETAINED_SCOPE ? "retained_endpoint" : "uninterpreted";
    expect(maximumIntervalReading(record)).toBe(expected);
  }
  // The scope is read exactly.
  for (const scope of [null, "", POLYNOMIAL_SCOPE.toUpperCase(), `${POLYNOMIAL_SCOPE} `, "supplied_binary64_polynomial_coefficient_enclosures", 7, {}]) {
    expect(maximumIntervalReading({ ...polynomial, enclosure_scope: scope })).toBe("uninterpreted");
    expect(maximumIntervalReading({ ...retained, enclosure_scope: scope })).toBe("uninterpreted");
  }
  // A basis that is present but not a string value the schema knows is not an absent basis.
  expect(maximumIntervalReading({ ...polynomial, basis: null })).toBe("uninterpreted");
  expect(qualifiedIntervalDescription({ value_lower_pa: 1.5, value_upper_pa: 2.5, ...polynomial }))
    .toBe("Bound over the supplied binary64 polynomial coefficient enclosures: [1.5, 2.5] Pa. Solution error is separate.");
  expect(qualifiedIntervalDescription({ value_lower_pa: 1.5, value_upper_pa: 2.5 })).toBe(uninterpreted({ value_lower_pa: 1.5, value_upper_pa: 2.5 }));
  // An unreadable endpoint is shown as unavailable, with the qualification still attached.
  expect(qualifiedIntervalDescription({ value_lower_pa: "1", ...polynomial }))
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

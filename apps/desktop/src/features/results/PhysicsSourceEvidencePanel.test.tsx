import { cleanup, render, screen, within } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import type { MechanicsResult } from "../../types";
import { hasNativeMechanicsInvocation } from "../../services/previewService";
import { PhysicsSourceEvidencePanel, POLYNOMIAL_ENCLOSURE_FIXED, RETAINED_ENDPOINT_FIXED, endpointLocationDescription, maximumIntervalReading, qualifiedIntervalDescription } from "./PhysicsSourceEvidencePanel";
import recoverySchemaSource from "../../../../../schemas/physics_source_recovery.schema.json?raw";

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
        expect(maximum).toMatchObject(FIXED.retained);
        expect(row).toHaveTextContent(`Bound: [${maximum.value_lower_pa}, ${maximum.value_upper_pa}] Pa.`);
        expect(row).toHaveTextContent(endpointLocationDescription(maximum.locations));
        expect(row).toHaveTextContent(`relative error bound: ${maximum.relative_error_bound}`);
        // The retained endpoint method keeps its own explanation of what its bound covers.
        expect(row).toHaveTextContent("The bound covers retained-action projection and ordered arithmetic with the represented section operands. It does not certify material or primitive geometry accuracy.");
        expect(row).not.toHaveTextContent("polynomial coefficient enclosures");
        expect(within(row).queryByTestId(`physics-source-maximum-interval-${maximum.result_id}`)).toBeNull();
      } else {
        ordinaryMaxima += 1;
        // The interval and the record's qualification of it are one sentence.
        expect(maximum).toMatchObject(FIXED.ordinary);
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
  // The mixed fixture is the one that holds ordinary stress maxima; the check above is not vacuous.
  expect(ordinaryMaxima > 0).toBe(name === "mixed");
});
const uninterpreted = (maximum: any) => `Recorded interval: [${typeof maximum.value_lower_pa === "number" && Number.isFinite(maximum.value_lower_pa) ? maximum.value_lower_pa : "unavailable"}, ${typeof maximum.value_upper_pa === "number" && Number.isFinite(maximum.value_upper_pa) ? maximum.value_upper_pa : "unavailable"}] Pa. The record's fields are not consistent with a method this panel explains, so the interval is not shown as a bound.`;
// The schema holds the list of fixed members; it is read here, not copied, and the edited records
// below are built from it and not from the panel's own maps.
const recoverySchema = JSON.parse(recoverySchemaSource);
const resolveSchema = (node: any): any => { let at = node; while (at?.$ref) at = at.$ref.slice(2).split("/").reduce((n: any, key: string) => n[key], recoverySchema); return at; };
const maximumListUses = (() => {
  const uses: any[] = [];
  const visit = (node: any) => {
    if (Array.isArray(node)) return node.forEach(visit);
    if (!node || typeof node !== "object") return;
    for (const [key, value] of Object.entries(node)) { if (key === "pipe_stress_extrema" && value && typeof value === "object" && !Array.isArray(value)) uses.push(value); visit(value); }
  };
  visit(recoverySchema);
  return uses;
})();
const fixedBy = (variant: any): Record<string, unknown> => Object.fromEntries(Object.entries(variant.properties).filter(([, property]: [string, any]) => property && "const" in property).map(([key, property]: [string, any]) => [key, property.const]));
const variantsOf = (use: any) => {
  const variants = resolveSchema(use).items.oneOf.map(resolveSchema);
  return { variants, retained: variants.filter((variant: any) => "basis" in variant.properties), ordinary: variants.filter((variant: any) => !("basis" in variant.properties)) };
};
const first = variantsOf(maximumListUses[0]);
const FIXED = { ordinary: fixedBy(first.ordinary[0]), retained: fixedBy(first.retained[0]) };
type Kind = keyof typeof FIXED;
const otherKind = (kind: Kind): Kind => kind === "ordinary" ? "retained" : "ordinary";

it('checks every member the schema fixes for each kind of stress-maximum record', () => {
  // The panel's two maps must be exactly the schema's fixed members. A member the schema fixes and
  // the panel does not check, or the reverse, fails here.
  expect(maximumListUses.length).toBeGreaterThan(0);
  for (const use of maximumListUses) {
    const { variants, retained, ordinary } = variantsOf(use);
    expect(variants).toHaveLength(2);
    expect(retained).toHaveLength(1);
    expect(ordinary).toHaveLength(1);
    expect(RETAINED_ENDPOINT_FIXED).toEqual(fixedBy(retained[0]));
    expect(POLYNOMIAL_ENCLOSURE_FIXED).toEqual(fixedBy(ordinary[0]));
    // Neither variant admits a member it does not name, so a record of one kind cannot carry the
    // other kind's fixed members.
    for (const variant of variants) expect(variant.additionalProperties).toBe(false);
  }
});

// Edited records: presentation states, not producer records. Each takes one committed record of
// the named kind and breaks one condition the panel's sentences rely on.
const edits: Array<[string, Kind, (maximum: any) => void]> = [];
for (const kind of ["ordinary", "retained"] as Kind[]) {
  for (const [member, value] of Object.entries(FIXED[kind])) {
    edits.push([`${kind}: ${member} removed`, kind, m => { delete m[member]; }]);
    edits.push([`${kind}: ${member} changed`, kind, m => { m[member] = typeof value === "number" ? value * 2 : `${String(value)}x`; }]);
    edits.push([`${kind}: ${member} null`, kind, m => { m[member] = null; }]);
  }
  for (const [member, value] of Object.entries(FIXED[otherKind(kind)])) {
    if (!(member in FIXED[kind])) edits.push([`${kind}: given the other kind's ${member}`, kind, m => { m[member] = value; }]);
  }
  // Every fixed member of the other kind at once, this kind's own left where the other has none:
  // the hybrids the independent reviews named (an ordinary record relabelled as retained keeps its
  // approximation; a retained record relabelled as ordinary keeps its basis and relative limit).
  edits.push([`${kind}: every member the other kind fixes`, kind, m => { Object.assign(m, FIXED[otherKind(kind)]); }]);
  edits.push([`${kind}: reversed endpoints`, kind, m => { [m.value_lower_pa, m.value_upper_pa] = [m.value_upper_pa + 1, m.value_lower_pa]; }]);
  edits.push([`${kind}: a negative lower endpoint`, kind, m => { m.value_lower_pa = -1; }]);
  edits.push([`${kind}: an endpoint that is not a number`, kind, m => { m.value_upper_pa = String(m.value_upper_pa); }]);
  edits.push([`${kind}: no lower endpoint`, kind, m => { delete m.value_lower_pa; }]);
}
edits.push(["retained: published value above the interval", "retained", m => { m.value_pa = m.value_upper_pa + 1; }]);
edits.push(["retained: published value below the interval", "retained", m => { m.value_pa = m.value_lower_pa - 1; }]);
edits.push(["retained: no published value", "retained", m => { delete m.value_pa; }]);
edits.push(["retained: a negative absolute error bound", "retained", m => { m.absolute_error_bound_pa = -1; }]);
edits.push(["retained: no absolute error bound", "retained", m => { delete m.absolute_error_bound_pa; }]);
edits.push(["retained: a relative error bound above the limit", "retained", m => { m.relative_error_bound = 2e-9; }]);
edits.push(["retained: a negative relative error bound", "retained", m => { m.relative_error_bound = -1e-12; }]);
edits.push(["retained: a relative error bound that is not a number", "retained", m => { m.relative_error_bound = "0"; }]);
const committedMaximum = (result: MechanicsResult, kind: Kind) => (result.contract_evidence as any).exact_cases.flatMap((c: any) => c.pipe_stress_extrema)
  .find((m: any) => kind === "retained" ? m.basis !== undefined : m.basis === undefined);
it.each(edits)('shows no interval as a bound for a record edited as %s', (_name, kind, edit) => {
  const result = fixture("mixed-sparse_interactive"), maximum = committedMaximum(result, kind);
  // The committed record is of its kind before the edit, and the panel reads it so.
  expect(maximumIntervalReading(maximum)).toBe(kind === "retained" ? "retained_endpoint" : "polynomial_enclosure");
  edit(maximum);
  expect(maximumIntervalReading(maximum)).toBe("uninterpreted");
  render(<PhysicsSourceEvidencePanel result={result} />);
  const row = screen.getByTestId(`physics-source-maximum-${maximum.result_id}`);
  expect(within(row).getByTestId(`physics-source-maximum-interval-${maximum.result_id}`).textContent).toBe(uninterpreted(maximum));
  expect(row.textContent).not.toMatch(/Bound/);
  expect(row).toHaveTextContent("Recorded method:");
  // Neither explanation, and none of the retained method's figures, is attached to such a record.
  expect(row).not.toHaveTextContent("Solution error is separate");
  expect(row).not.toHaveTextContent("The bound covers retained-action projection");
  expect(row).not.toHaveTextContent("Published value");
  expect(row).not.toHaveTextContent("station witness");
});
it('reads an interval explanation only from a record that meets every condition of one kind', () => {
  const interval = { value_lower_pa: 1.5, value_upper_pa: 2.5 };
  const polynomial = { ...interval, ...FIXED.ordinary };
  const retained = { ...interval, value_pa: 2, absolute_error_bound_pa: 0.5, relative_error_bound: 1e-10, ...FIXED.retained };
  expect(maximumIntervalReading(polynomial)).toBe("polynomial_enclosure");
  expect(maximumIntervalReading(retained)).toBe("retained_endpoint");
  // The limits of each condition are inside it: equal endpoints, a zero lower endpoint, a published
  // value at either endpoint, zero error bounds and a relative error bound at the limit.
  expect(maximumIntervalReading({ ...polynomial, value_lower_pa: 0, value_upper_pa: 0 })).toBe("polynomial_enclosure");
  for (const value_pa of [1.5, 2.5]) expect(maximumIntervalReading({ ...retained, value_pa, absolute_error_bound_pa: 0, relative_error_bound: 0 })).toBe("retained_endpoint");
  expect(maximumIntervalReading({ ...retained, relative_error_bound: 1e-9 })).toBe("retained_endpoint");
  // A complete record of one kind that also carries a member only the other kind fixes.
  for (const [member, value] of Object.entries(FIXED.ordinary)) if (!(member in FIXED.retained)) expect(maximumIntervalReading({ ...retained, [member]: value })).toBe("uninterpreted");
  for (const [member, value] of Object.entries(FIXED.retained)) if (!(member in FIXED.ordinary)) expect(maximumIntervalReading({ ...polynomial, [member]: value })).toBe("uninterpreted");
  // The scope is read exactly.
  for (const scope of [undefined, null, "", String(FIXED.ordinary.enclosure_scope).toUpperCase(), `${String(FIXED.ordinary.enclosure_scope)} `, "supplied_binary64_polynomial_coefficient_enclosures", 7, {}]) {
    expect(maximumIntervalReading({ ...polynomial, enclosure_scope: scope })).toBe("uninterpreted");
    expect(maximumIntervalReading({ ...retained, enclosure_scope: scope })).toBe("uninterpreted");
  }
  for (const endpoint of [Number.NaN, Number.POSITIVE_INFINITY, "2.5", null]) expect(maximumIntervalReading({ ...polynomial, value_upper_pa: endpoint })).toBe("uninterpreted");
  expect(qualifiedIntervalDescription(polynomial)).toBe("Bound over the supplied binary64 polynomial coefficient enclosures: [1.5, 2.5] Pa. Solution error is separate.");
  expect(qualifiedIntervalDescription(interval)).toBe(uninterpreted(interval));
  // An unreadable endpoint is shown as unavailable and the interval is not called a bound.
  expect(qualifiedIntervalDescription({ ...polynomial, value_lower_pa: "1" })).toBe(uninterpreted({ value_lower_pa: "1", value_upper_pa: 2.5 }));
});
it('describes constant, tied and ambiguous endpoint claims without inventing uniqueness', () => {
  // Presentation-only states, not simulated producer receipts or physical tests.
  expect(endpointLocationDescription({ kind: "whole_span_constant" })).toContain("every station");
  expect(endpointLocationDescription({ kind: "strict_endpoint", endpoint: "j" })).toContain("does not identify a unique circumferential fibre");
  expect(endpointLocationDescription({ kind: "endpoint_candidates", exact_tie_proven: true, interior_equal_possible: false })).toContain("Both endpoints tie");
  expect(endpointLocationDescription({ kind: "endpoint_candidates", exact_tie_proven: false, interior_equal_possible: true })).toContain("equality is not proved");
  expect(endpointLocationDescription(null)).toContain("no unique governing station");
  // An endpoint is the text i or j, not anything that prints as one.
  expect(endpointLocationDescription({ kind: "strict_endpoint", endpoint: ["i"] })).toContain("no unique governing station");
});

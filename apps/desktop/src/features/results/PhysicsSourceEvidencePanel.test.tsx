import { cleanup, render, screen, within } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import type { MechanicsResult } from "../../types";
import { hasNativeMechanicsInvocation } from "../../services/previewService";
import { PhysicsSourceEvidencePanel, RECORDED_FIELDS_NOTICE, recordedFieldLines, recordedIntervalDescription } from "./PhysicsSourceEvidencePanel";

afterEach(cleanup);
// B7b: the composite is pressure-source-1 (3.0.0/exact_pressure_v3), the v3 twin of the
// retired physics-source-1; its committed producer raws.
const fixtureSources = import.meta.glob("../../../../../fixtures/product_preview/pressure_source/physics_source/*.raw.json", { query: "?raw", import: "default", eager: true }) as Record<string,string>;
const fixture = (name: string) => JSON.parse(fixtureSources[`../../../../../fixtures/product_preview/pressure_source/physics_source/${name}.raw.json`]) as MechanicsResult;
const POLYNOMIAL_SCOPE = "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate";
const RETAINED_SCOPE = "retained_action_projection_and_ordered_binary64_recipe;represented_source_section_operands";
// Sentences the panel once said of its own about a record. It now says none of them: each relied
// on a condition between the record's fields that the panel does not check.
const PANEL_CLAIMS = [/Bound/, /physical enclosure/, /The bound covers/, /does not certify/, /Strict endpoint/, /interior stations/, /Whole span constant/, /every station/,
  /Both endpoints/, /Interior equality/, /no unique governing station/, /station witness/, /Published value/];
const maximaOf = (result: MechanicsResult) => (result.contract_evidence as any).exact_cases.flatMap((c: any) => c.pipe_stress_extrema);

it.each(["n05", "n06", "mixed", "fields"])('shows %s recorded methods and each stress maximum as its recorded fields, without promoting imported data', name => {
  let ordinaryMaxima = 0, retainedMaxima = 0;
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
    for (const c of physical.exact_cases) {
      // Said once before a case's records, and only where there are records to speak of.
      const notice = within(panel).queryByTestId(`physics-source-recorded-fields-notice-${c.load_case_id}`);
      if (c.pipe_stress_extrema.length > 0) expect(notice!.textContent).toBe(RECORDED_FIELDS_NOTICE); else expect(notice).toBeNull();
      for (const maximum of c.pipe_stress_extrema) {
        const row = within(panel).getByTestId(`physics-source-maximum-${maximum.result_id}`);
        const retained = maximum.basis !== undefined;
        if (retained) retainedMaxima += 1; else ordinaryMaxima += 1;
        // The interval and the scope the record states for it are one sentence in one element: the
        // interval is never shown without that scope. For an ordinary record the scope is the
        // qualification itself, "solution error is separate" included.
        expect(maximum.enclosure_scope).toBe(retained ? RETAINED_SCOPE : POLYNOMIAL_SCOPE);
        expect(within(row).getByTestId(`physics-source-maximum-interval-${maximum.result_id}`).textContent)
          .toBe(`Recorded interval: [${maximum.value_lower_pa}, ${maximum.value_upper_pa}] Pa. Recorded scope: ${maximum.enclosure_scope}.`);
        expect(row).toHaveTextContent(`Recorded pipe: ${maximum.pipe_id}. Recorded result: ${maximum.result_id}.`);
        // Every other field is shown as recorded, under its own name.
        expect(within(row).getByTestId(`physics-source-maximum-fields-${maximum.result_id}`).textContent).toBe(recordedFieldLines(maximum).join(" "));
        expect(row).toHaveTextContent(`Recorded coefficient basis: ${maximum.coefficient_basis}.`);
        expect(row).toHaveTextContent(`Recorded station fraction: ${maximum.station_fraction}.`);
        if (retained) {
          expect(row).toHaveTextContent(`Recorded basis: ${maximum.basis}.`);
          expect(row).toHaveTextContent(`Recorded value: ${maximum.value_pa} Pa.`);
          expect(row).toHaveTextContent(`Recorded relative error bound: ${maximum.relative_error_bound}.`);
          expect(row).toHaveTextContent(`Recorded location: kind ${maximum.locations.kind}`);
        } else {
          expect(row).toHaveTextContent(`Recorded approximation: ${maximum.approximation}.`);
          expect(row).not.toHaveTextContent("Recorded value:");
        }
        for (const claim of PANEL_CLAIMS) expect(row.textContent).not.toMatch(claim);
      }
    }
    expect(hasNativeMechanicsInvocation(result, null)).toBe(false);
    expect(JSON.stringify(result)).toBe(before);
    view.unmount();
  }
  // The checks above ran on producer records of both kinds: the mixed fixture holds ordinary maxima.
  if (name === "mixed") expect(ordinaryMaxima).toBeGreaterThan(0);
  expect(retainedMaxima).toBeGreaterThan(0);
});

// Edited records: presentation states, not producer records. Each is a record whose own fields
// disagree, taken from the independent readings of this panel. The panel shows each as recorded
// and says nothing of its own about any of them.
const edits: Array<[string, "ordinary" | "retained", (maximum: any) => void, string[]]> = [
  ["an ordinary record given the retained basis and scope", "ordinary", m => { m.basis = "retained_source_endpoint_normal_max_v1"; m.enclosure_scope = RETAINED_SCOPE; }, ["Recorded basis: retained_source_endpoint_normal_max_v1.", "Recorded approximation: piecewise_quadratic_straight_section_statics."]],
  ["an ordinary record with reversed endpoints", "ordinary", m => { [m.value_lower_pa, m.value_upper_pa] = [10, 5]; }, []],
  ["an ordinary record with no scope", "ordinary", m => { delete m.enclosure_scope; }, []],
  ["an ordinary record naming another pipe", "ordinary", m => { m.pipe_id = "ghost"; }, ["Recorded pipe: ghost."]],
  ["an ordinary record with a station fraction of 2", "ordinary", m => { m.station_fraction = 2; }, ["Recorded station fraction: 2."]],
  ["a retained record given the polynomial scope, its basis removed", "retained", m => { delete m.basis; m.enclosure_scope = POLYNOMIAL_SCOPE; }, ["Recorded coefficient basis: retained_section_functionals_binary64."]],
  ["a retained record whose location names the other endpoint", "retained", m => { m.locations = { kind: "strict_endpoint", endpoint: "j" }; }, ["Recorded location: kind strict_endpoint, endpoint j."]],
  ["a retained record whose location is a claimed tie", "retained", m => { m.locations = { kind: "endpoint_candidates", exact_tie_proven: true, interior_equal_possible: true }; }, ["Recorded location: kind endpoint_candidates, exact_tie_proven true, interior_equal_possible true."]],
  ["a retained record whose location is not a record", "retained", m => { m.locations = ["i"]; }, ["Recorded location: unavailable."]],
  ["a retained record with a zero absolute error bound", "retained", m => { m.absolute_error_bound_pa = 0; }, ["Recorded absolute error bound: 0 Pa."]],
  ["a retained record whose value is outside its interval", "retained", m => { m.value_pa = m.value_upper_pa + 1; }, []],
  ["a retained record whose value is not a number", "retained", m => { m.value_pa = "12"; }, ["Recorded value: unavailable."]],
];
it.each(edits)('shows %s as recorded and asserts nothing about it', (_name, kind, edit, expected) => {
  const result = fixture("mixed-sparse_interactive");
  const maximum = maximaOf(result).find((m: any) => kind === "retained" ? m.basis !== undefined : m.basis === undefined);
  edit(maximum);
  render(<PhysicsSourceEvidencePanel result={result} />);
  const row = screen.getByTestId(`physics-source-maximum-${maximum.result_id}`);
  expect(within(row).getByTestId(`physics-source-maximum-interval-${maximum.result_id}`).textContent).toBe(recordedIntervalDescription(maximum));
  expect(within(row).getByTestId(`physics-source-maximum-fields-${maximum.result_id}`).textContent).toBe(recordedFieldLines(maximum).join(" "));
  for (const sentence of expected) expect(row).toHaveTextContent(sentence);
  for (const claim of PANEL_CLAIMS) expect(row.textContent).not.toMatch(claim);
});

it('gives each recorded field under its own name and infers nothing from any of them', () => {
  expect(recordedIntervalDescription({ value_lower_pa: 1.5, value_upper_pa: 2.5, enclosure_scope: POLYNOMIAL_SCOPE }))
    .toBe("Recorded interval: [1.5, 2.5] Pa. Recorded scope: supplied_binary64_polynomial_coefficient_enclosures; solution error is separate.");
  // An endpoint that is not a finite number, and a scope that is not text, are shown as unavailable.
  expect(recordedIntervalDescription({ value_lower_pa: "1", value_upper_pa: Number.NaN, enclosure_scope: 7 })).toBe("Recorded interval: [unavailable, unavailable] Pa. Recorded scope: unavailable.");
  expect(recordedIntervalDescription({})).toBe("Recorded interval: [unavailable, unavailable] Pa. Recorded scope: unavailable.");
  // Only the fields a record has are listed, in a fixed order, each as recorded.
  expect(recordedFieldLines({})).toEqual([]);
  expect(recordedFieldLines({ station_fraction: 0.5, coefficient_basis: "a_basis", value_pa: 3, relative_limit: 1e-9, absolute_error_bound_pa: null }))
    .toEqual(["Recorded coefficient basis: a_basis.", "Recorded value: 3 Pa.", "Recorded absolute error bound: unavailable.", "Recorded relative limit: 1e-9.", "Recorded station fraction: 0.5."]);
  // A location record is given member by member; one that is not a record, or holds none of the
  // known members, is unavailable. No station is named or excluded on its strength.
  expect(recordedFieldLines({ locations: { kind: "whole_span_constant" } })).toEqual(["Recorded location: kind whole_span_constant."]);
  expect(recordedFieldLines({ locations: { kind: "strict_endpoint", endpoint: ["i"] } })).toEqual(["Recorded location: kind strict_endpoint, endpoint unavailable."]);
  // A value of another kind than its field's is unavailable: text is not shown as a number, a
  // number is not shown as text, and a word is not shown as a recorded flag.
  expect(recordedFieldLines({ basis: 7, value_pa: "12", station_fraction: true, locations: { kind: 3, exact_tie_proven: "true" } }))
    .toEqual(["Recorded basis: unavailable.", "Recorded value: unavailable.", "Recorded station fraction: unavailable.", "Recorded location: kind unavailable, exact_tie_proven unavailable."]);
  expect(recordedFieldLines({ locations: {} })).toEqual(["Recorded location: unavailable."]);
  expect(recordedFieldLines({ locations: null })).toEqual(["Recorded location: unavailable."]);
  // The notice says who decides agreement and does not say that a check ran.
  expect(RECORDED_FIELDS_NOTICE).toBe("These are the record's fields as recorded. Whether they agree with each other and with the result is for the result's reader; this panel does not check it.");
});

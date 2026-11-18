import { cleanup, render, screen, within } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import type { MechanicsResult } from "../../types";
import { hasNativeMechanicsInvocation } from "../../services/previewService";
import { PhysicsSourceEvidencePanel } from "./PhysicsSourceEvidencePanel";

afterEach(cleanup);
// B7b: the composite is pressure-source-1 (3.0.0/exact_pressure_v3), the v3 twin of the
// retired physics-source-1; its committed producer raws.
const fixtureSources = import.meta.glob("../../../../../fixtures/product_preview/pressure_source/physics_source/*.raw.json", { query: "?raw", import: "default", eager: true }) as Record<string,string>;
const fixture = (name: string) => JSON.parse(fixtureSources[`../../../../../fixtures/product_preview/pressure_source/physics_source/${name}.raw.json`]) as MechanicsResult;
const POLYNOMIAL_SCOPE = "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate";
const RETAINED_SCOPE = "retained_action_projection_and_ordered_binary64_recipe;represented_source_section_operands";
const NOTICE = "These entries show selected fields from the stress-maximum record. This view does not run the whole-result reader to check whether those fields agree with each other or with the linked result.";
const MARKER = "Unavailable to display";
// Sentences the panel once said of its own about a record. It now says none of them, anywhere in a
// case: each relied on a condition between the record's fields that the panel does not check.
const PANEL_CLAIMS = [/Bound/, /physical enclosure/, /The bound covers/, /does not certify/, /Strict endpoint/, /interior stations/, /Whole span constant/, /every station/,
  /Both endpoints/, /Interior equality/, /no unique governing station/, /station witness/, /Published value/, /unique circumferential fibre/, /Recorded interval/];
const maximaOf = (result: MechanicsResult) => (result.contract_evidence as any).exact_cases.flatMap((c: any) => c.pipe_stress_extrema);

/** What a definition list renders, read from its own elements: one [label, value] for each label
 * element and the value element that follows it. Nothing here reads joined text. */
const pairs = (list: HTMLElement): string[][] => {
  expect(list.tagName).toBe("DL");
  return Array.from(list.querySelectorAll("dt")).map(label => {
    const value = label.nextElementSibling!;
    expect(value.tagName).toBe("DD");
    return [label.textContent!, value.textContent!];
  });
};
const listsOf = (row: HTMLElement, id: string) => ({
  identity: pairs(within(row).getByTestId(`physics-source-maximum-identity-${id}`)),
  interval: pairs(within(row).getByTestId(`physics-source-maximum-interval-${id}`)),
  fields: pairs(within(row).getByTestId(`physics-source-maximum-fields-${id}`)),
});
// The expected text of a recorded value, written here and not taken from the panel: a text is
// quoted as JSON text; a number is the parsed number as it prints.
const quoted = (v: string) => JSON.stringify(v), bare = (v: number | boolean) => String(v);
/** The location entries a producer record has: its kind, and its endpoint where it records one. */
const locationOf = (m: any): string[][] => [["Recorded location kind", quoted(m.locations.kind)], ...(m.locations.endpoint === undefined ? [] : [["Recorded location endpoint", quoted(m.locations.endpoint)]])];

it.each(["n05", "n06", "mixed", "fields"])('shows each %s stress maximum as labelled recorded fields, without promoting imported data', name => {
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
      // No removed sentence returns anywhere in the case, inside a row or beside one.
      for (const claim of PANEL_CLAIMS) expect(row.textContent).not.toMatch(claim);
    }
    for (const c of physical.exact_cases) {
      // Said once before a case's entries, and only where there are entries to speak of.
      const notice = within(panel).queryByTestId(`physics-source-recorded-fields-notice-${c.load_case_id}`);
      if (c.pipe_stress_extrema.length > 0) expect(notice!.textContent).toBe(NOTICE); else expect(notice).toBeNull();
      for (const maximum of c.pipe_stress_extrema) {
        const row = within(panel).getByTestId(`physics-source-maximum-${maximum.result_id}`);
        const retained = maximum.basis !== undefined;
        if (retained) retainedMaxima += 1; else ordinaryMaxima += 1;
        expect(within(row).getByText("Elastic normal-stress maximum evidence entry").tagName).toBe("STRONG");
        const shown = listsOf(row, maximum.result_id);
        expect(shown.identity).toEqual([["Recorded pipe", quoted(maximum.pipe_id)], ["Recorded result", quoted(maximum.result_id)]]);
        // The two endpoints and the scope the record states for them are one element: the
        // endpoints are never shown without that scope. For an ordinary record the scope is the
        // qualification itself, "solution error is separate" included.
        expect(maximum.enclosure_scope).toBe(retained ? RETAINED_SCOPE : POLYNOMIAL_SCOPE);
        expect(shown.interval).toEqual([
          ["Recorded lower endpoint (Pa)", bare(maximum.value_lower_pa)],
          ["Recorded upper endpoint (Pa)", bare(maximum.value_upper_pa)],
          ["Recorded scope", quoted(maximum.enclosure_scope)],
        ]);
        // Every other field the row showed on main, each under its own label.
        expect(shown.fields).toEqual(retained ? [
          ["Recorded basis", quoted(maximum.basis)],
          ["Recorded coefficient basis", quoted(maximum.coefficient_basis)],
          ["Recorded value (Pa)", bare(maximum.value_pa)],
          ["Recorded absolute error bound (Pa)", bare(maximum.absolute_error_bound_pa)],
          ["Recorded relative error bound", bare(maximum.relative_error_bound)],
          ["Recorded relative limit", bare(maximum.relative_limit)],
          ["Recorded station fraction", bare(maximum.station_fraction)],
          ...locationOf(maximum),
        ] : [
          ["Recorded coefficient basis", quoted(maximum.coefficient_basis)],
          ["Recorded approximation", quoted(maximum.approximation)],
          ["Recorded station fraction", bare(maximum.station_fraction)],
        ]);
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

// Edited records: presentation states, not producer records, taken from the independent readings
// of this panel. Each is rendered by itself and read from the rendered label and value elements.
// `interval` and `fields` give the whole expected list where the edit changes it.
type Shown = { interval?: (m: any) => string[][]; fields?: (m: any) => string[][] };
const retainedFields = (m: any, location: string[][]): string[][] => [
  ["Recorded basis", quoted(m.basis)], ["Recorded coefficient basis", quoted(m.coefficient_basis)], ["Recorded value (Pa)", bare(m.value_pa)],
  ["Recorded absolute error bound (Pa)", bare(m.absolute_error_bound_pa)], ["Recorded relative error bound", bare(m.relative_error_bound)],
  ["Recorded relative limit", bare(m.relative_limit)], ["Recorded station fraction", bare(m.station_fraction)], ...location,
];
const edits: Array<[string, "ordinary" | "retained", (maximum: any) => void, Shown]> = [
  // The fifth reading's pair. One text value that spells out a second member is one labelled
  // value, and no endpoint field is shown; two members are two labelled values.
  ["a location whose kind is the single text \"strict_endpoint, endpoint i\"", "retained", m => { m.locations = { kind: "strict_endpoint, endpoint i" }; },
    { fields: m => retainedFields(m, [["Recorded location kind", "\"strict_endpoint, endpoint i\""]]) }],
  ["a location with the two members kind and endpoint", "retained", m => { m.locations = { kind: "strict_endpoint", endpoint: "i" }; },
    { fields: m => retainedFields(m, [["Recorded location kind", "\"strict_endpoint\""], ["Recorded location endpoint", "\"i\""]]) }],
  // A recorded text "unavailable" is quoted; the panel's own marker is not.
  ["a recorded text \"Unavailable to display\" beside a value that cannot be displayed", "retained", m => { m.basis = MARKER; m.coefficient_basis = 7; m.locations = { kind: "unavailable" }; },
    { fields: m => [["Recorded basis", "\"Unavailable to display\""], ["Recorded coefficient basis", MARKER], ...retainedFields(m, [["Recorded location kind", "\"unavailable\""]]).slice(2)] }],
  ["a lower endpoint given as text", "ordinary", m => { m.value_lower_pa = "5"; },
    { interval: m => [["Recorded lower endpoint (Pa)", MARKER], ["Recorded upper endpoint (Pa)", bare(m.value_upper_pa)], ["Recorded scope", quoted(POLYNOMIAL_SCOPE)]] }],
  ["reversed endpoints", "ordinary", m => { [m.value_lower_pa, m.value_upper_pa] = [10, 5]; },
    { interval: () => [["Recorded lower endpoint (Pa)", "10"], ["Recorded upper endpoint (Pa)", "5"], ["Recorded scope", quoted(POLYNOMIAL_SCOPE)]] }],
  ["no scope and no endpoints", "ordinary", m => { delete m.enclosure_scope; delete m.value_lower_pa; delete m.value_upper_pa; },
    { interval: () => [["Recorded lower endpoint (Pa)", MARKER], ["Recorded upper endpoint (Pa)", MARKER], ["Recorded scope", MARKER]] }],
  ["an ordinary record given the retained basis and scope", "ordinary", m => { m.basis = "retained_source_endpoint_normal_max_v1"; m.enclosure_scope = RETAINED_SCOPE; },
    { interval: m => [["Recorded lower endpoint (Pa)", bare(m.value_lower_pa)], ["Recorded upper endpoint (Pa)", bare(m.value_upper_pa)], ["Recorded scope", quoted(RETAINED_SCOPE)]],
      fields: m => [["Recorded basis", "\"retained_source_endpoint_normal_max_v1\""], ["Recorded coefficient basis", quoted(m.coefficient_basis)], ["Recorded approximation", quoted(m.approximation)], ["Recorded station fraction", bare(m.station_fraction)]] }],
  ["a station fraction of 2", "ordinary", m => { m.station_fraction = 2; },
    { fields: m => [["Recorded coefficient basis", quoted(m.coefficient_basis)], ["Recorded approximation", quoted(m.approximation)], ["Recorded station fraction", "2"]] }],
  ["a location naming the other endpoint", "retained", m => { m.locations = { kind: "strict_endpoint", endpoint: "j" }; },
    { fields: m => retainedFields(m, [["Recorded location kind", "\"strict_endpoint\""], ["Recorded location endpoint", "\"j\""]]) }],
  ["a location that claims a tie", "retained", m => { m.locations = { kind: "endpoint_candidates", exact_tie_proven: true, interior_equal_possible: "true" }; },
    { fields: m => retainedFields(m, [["Recorded location kind", "\"endpoint_candidates\""], ["Recorded location exact_tie_proven", "true"], ["Recorded location interior_equal_possible", MARKER]]) }],
  ["a location that is not a record", "retained", m => { m.locations = ["i"]; }, { fields: m => retainedFields(m, [["Recorded location", MARKER]]) }],
  ["a location with none of the members shown", "retained", m => { m.locations = { station: 3 }; }, { fields: m => retainedFields(m, [["Recorded location", MARKER]]) }],
  ["a value outside its endpoints, a zero error bound and a zero parsed from 0.0", "retained", m => { m.value_pa = m.value_upper_pa + 1; m.absolute_error_bound_pa = JSON.parse("0.0"); m.relative_limit = JSON.parse("-0.0"); },
    { fields: m => [["Recorded basis", quoted(m.basis)], ["Recorded coefficient basis", quoted(m.coefficient_basis)], ["Recorded value (Pa)", bare(m.value_upper_pa + 1)], ["Recorded absolute error bound (Pa)", "0"],
      ["Recorded relative error bound", bare(m.relative_error_bound)], ["Recorded relative limit", "0"], ["Recorded station fraction", bare(m.station_fraction)], ...locationOf(m)] }],
  ["a value given as text and an empty basis", "retained", m => { m.value_pa = "12"; m.basis = ""; },
    { fields: m => [["Recorded basis", "\"\""], ["Recorded coefficient basis", quoted(m.coefficient_basis)], ["Recorded value (Pa)", MARKER], ...retainedFields(m, locationOf(m)).slice(3)] }],
];
it.each(edits)('shows %s as labelled values and asserts nothing about it', (_name, kind, edit, expected) => {
  const result = fixture("mixed-sparse_interactive");
  const maximum = maximaOf(result).find((m: any) => kind === "retained" ? m.basis !== undefined : m.basis === undefined);
  const pipe = maximum.pipe_id, id = maximum.result_id;
  edit(maximum);
  render(<PhysicsSourceEvidencePanel result={result} />);
  const row = screen.getByTestId(`physics-source-maximum-${id}`), shown = listsOf(row, id);
  expect(shown.identity).toEqual([["Recorded pipe", quoted(pipe)], ["Recorded result", quoted(id)]]);
  if (expected.interval) expect(shown.interval).toEqual(expected.interval(maximum));
  if (expected.fields) expect(shown.fields).toEqual(expected.fields(maximum));
  // The row is its heading and its three lists and nothing else, and the case says none of the
  // removed sentences.
  expect(Array.from(row.children).map(child => child.tagName)).toEqual(["STRONG", "DL", "DL", "DL"]);
  const article = row.closest("article")!;
  for (const claim of PANEL_CLAIMS) expect(article.textContent).not.toMatch(claim);
});

it('shows a record without a pipe or a result identifier with the marker, not as a recorded text', () => {
  const result = fixture("mixed-sparse_interactive");
  const maximum = maximaOf(result)[0], id = maximum.result_id;
  maximum.pipe_id = 9;
  render(<PhysicsSourceEvidencePanel result={result} />);
  expect(listsOf(screen.getByTestId(`physics-source-maximum-${id}`), id).identity).toEqual([["Recorded pipe", MARKER], ["Recorded result", quoted(id)]]);
});

import { cleanup, render, screen, within } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import type { MechanicsResult } from "../../types";
import { hasNativeMechanicsInvocation } from "../../services/previewService";
import { documentStatusState } from "./integrityStatus";
import { NUMERICAL_HEALTH_STATE_TEXT } from "./numericalHealthText";
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

const nodeNames = (parent: Element) => Array.from(parent.childNodes).map(node => node.nodeType === Node.ELEMENT_NODE ? (node as Element).tagName : "#text");
/** What a definition list renders, read from its own elements: one [label, value] for each pair.
 * The list holds pair wrappers and nothing else, each wrapper exactly one label element followed by
 * one value element, each of those text only: a value with no label, or loose text, fails here.
 * Nothing here reads joined text. */
const pairs = (list: HTMLElement): string[][] => {
  expect(list.tagName).toBe("DL");
  expect(nodeNames(list).every(name => name === "DIV")).toBe(true);
  return Array.from(list.children).map(wrapper => {
    expect(nodeNames(wrapper)).toEqual(["DT", "DD"]);
    const [label, value] = Array.from(wrapper.children);
    for (const element of [label, value]) expect(nodeNames(element).every(name => name === "#text")).toBe(true);
    return [label.textContent!, value.textContent!];
  });
};
/** Where the panel puts a maximum, as its test identifiers have it: the position of its case among
 * the receipt's cases, then its own position in that case. No recorded identifier is part of it. */
const positionOf = (result: MechanicsResult, maximum: unknown): string => {
  const receiptCases = (result.source_block_recovery as any).body.cases as any[];
  for (const c of (result.contract_evidence as any).exact_cases) {
    const entry = c.pipe_stress_extrema.indexOf(maximum);
    if (entry >= 0) return `${receiptCases.findIndex(r => r.basis_ref.ref_id === c.load_case_id)}-${entry}`;
  }
  throw new Error("the maximum is not in the result");
};
const listsOf = (row: HTMLElement, at: string) => ({
  identity: pairs(within(row).getByTestId(`physics-source-maximum-identity-${at}`)),
  interval: pairs(within(row).getByTestId(`physics-source-maximum-interval-${at}`)),
  fields: pairs(within(row).getByTestId(`physics-source-maximum-fields-${at}`)),
});
// The expected text of a recorded value, written here and not taken from the panel. A text made of
// printable ASCII with no two adjacent spaces (every text in the fixtures and in the edits below) is
// the word "text" and the text as a JSON string; a number is the parsed number as it prints. Texts
// outside that set are the subject of the spelling tests at the end.
const quoted = (v: string) => {
  expect(v).toMatch(/^[\x20-\x7e]*$/);
  expect(v).not.toMatch(/ {2}/);
  return `text ${JSON.stringify(v)}`;
};
const bare = (v: number | boolean) => String(v);
/** The location entries a producer record has: its kind, and its endpoint where it records one. */
const locationOf = (m: any): string[][] => [["Recorded location kind", quoted(m.locations.kind)], ...(m.locations.endpoint === undefined ? [] : [["Recorded location endpoint", quoted(m.locations.endpoint)]])];

it.each(["n05", "n06", "mixed", "fields"])('shows each %s stress maximum as labelled recorded fields, without promoting imported data', name => {
  let ordinaryMaxima = 0, retainedMaxima = 0;
  for (const mode of ["sparse_interactive", "dense_scrutiny"]) {
    const result = fixture(`${name}-${mode}`), before = JSON.stringify(result);
    const view = render(<PhysicsSourceEvidencePanel result={result} />);
    const panel = screen.getByRole("region", { name: "Recorded recovery evidence" });
    expect(panel).toHaveTextContent(`Ordinary numerical attempt: ${NUMERICAL_HEALTH_STATE_TEXT[documentStatusState(result.numerical_quality!.status)]}`);
    expect(panel.textContent).not.toContain("checks_passed");
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
      // Said once before a case's entries, and only where there are entries to speak of: the notice
      // is a child of its case's article, immediately before the first entry, and every entry of
      // the case follows it.
      const caseIndex = receipt.body.cases.findIndex((r: any) => r.basis_ref.ref_id === c.load_case_id);
      const article = within(panel).getByTestId(`physics-source-case-${c.load_case_id}`), children = Array.from(article.children);
      const notice = within(panel).queryByTestId(`physics-source-recorded-fields-notice-${caseIndex}`);
      const entries = children.filter(child => child.getAttribute("data-testid")?.startsWith("physics-source-maximum-"));
      expect(entries).toHaveLength(c.pipe_stress_extrema.length);
      if (c.pipe_stress_extrema.length > 0) {
        expect(notice!.textContent).toBe(NOTICE);
        expect(children.indexOf(notice!)).toBe(children.indexOf(entries[0]) - 1);
        expect(children.slice(children.indexOf(notice!) + 1)).toEqual(entries);
      } else expect(notice).toBeNull();
      for (const maximum of c.pipe_stress_extrema) {
        const at = positionOf(result, maximum), row = within(panel).getByTestId(`physics-source-maximum-${at}`);
        const retained = maximum.basis !== undefined;
        if (retained) retainedMaxima += 1; else ordinaryMaxima += 1;
        expect(within(row).getByText("Elastic normal-stress maximum evidence entry").tagName).toBe("STRONG");
        const shown = listsOf(row, at);
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
const retainedNumbers = (m: any, location: string[][]): string[][] => [
  ["Recorded value (Pa)", bare(m.value_pa)],
  ["Recorded absolute error bound (Pa)", bare(m.absolute_error_bound_pa)], ["Recorded relative error bound", bare(m.relative_error_bound)],
  ["Recorded relative limit", bare(m.relative_limit)], ["Recorded station fraction", bare(m.station_fraction)], ...location,
];
const retainedFields = (m: any, location: string[][]): string[][] => [
  ["Recorded basis", quoted(m.basis)], ["Recorded coefficient basis", quoted(m.coefficient_basis)], ...retainedNumbers(m, location),
];
const edits: Array<[string, "ordinary" | "retained", (maximum: any) => void, Shown]> = [
  // The fifth reading's pair. One text value that spells out a second member is one labelled
  // value, and no endpoint field is shown; two members are two labelled values.
  ["a location whose kind is the single text \"strict_endpoint, endpoint i\"", "retained", m => { m.locations = { kind: "strict_endpoint, endpoint i" }; },
    { fields: m => retainedFields(m, [["Recorded location kind", 'text "strict_endpoint, endpoint i"']]) }],
  ["a location with the two members kind and endpoint", "retained", m => { m.locations = { kind: "strict_endpoint", endpoint: "i" }; },
    { fields: m => retainedFields(m, [["Recorded location kind", 'text "strict_endpoint"'], ["Recorded location endpoint", 'text "i"']]) }],
  // A recorded text is the word "text" and its quoted spelling; the panel's own marker is neither.
  ["a recorded text \"Unavailable to display\" beside a value that cannot be displayed", "retained", m => { m.basis = MARKER; m.coefficient_basis = 7; m.locations = { kind: "unavailable" }; },
    { fields: m => [["Recorded basis", 'text "Unavailable to display"'], ["Recorded coefficient basis", MARKER], ...retainedNumbers(m, [["Recorded location kind", 'text "unavailable"']])] }],
  ["a lower endpoint given as text", "ordinary", m => { m.value_lower_pa = "5"; },
    { interval: m => [["Recorded lower endpoint (Pa)", MARKER], ["Recorded upper endpoint (Pa)", bare(m.value_upper_pa)], ["Recorded scope", quoted(POLYNOMIAL_SCOPE)]] }],
  ["reversed endpoints", "ordinary", m => { [m.value_lower_pa, m.value_upper_pa] = [10, 5]; },
    { interval: () => [["Recorded lower endpoint (Pa)", "10"], ["Recorded upper endpoint (Pa)", "5"], ["Recorded scope", quoted(POLYNOMIAL_SCOPE)]] }],
  ["no scope and no endpoints", "ordinary", m => { delete m.enclosure_scope; delete m.value_lower_pa; delete m.value_upper_pa; },
    { interval: () => [["Recorded lower endpoint (Pa)", MARKER], ["Recorded upper endpoint (Pa)", MARKER], ["Recorded scope", MARKER]] }],
  ["an ordinary record given the retained basis and scope", "ordinary", m => { m.basis = "retained_source_endpoint_normal_max_v1"; m.enclosure_scope = RETAINED_SCOPE; },
    { interval: m => [["Recorded lower endpoint (Pa)", bare(m.value_lower_pa)], ["Recorded upper endpoint (Pa)", bare(m.value_upper_pa)], ["Recorded scope", quoted(RETAINED_SCOPE)]],
      fields: m => [["Recorded basis", 'text "retained_source_endpoint_normal_max_v1"'], ["Recorded coefficient basis", quoted(m.coefficient_basis)], ["Recorded approximation", quoted(m.approximation)], ["Recorded station fraction", bare(m.station_fraction)]] }],
  ["a station fraction of 2", "ordinary", m => { m.station_fraction = 2; },
    { fields: m => [["Recorded coefficient basis", quoted(m.coefficient_basis)], ["Recorded approximation", quoted(m.approximation)], ["Recorded station fraction", "2"]] }],
  ["a location naming the other endpoint", "retained", m => { m.locations = { kind: "strict_endpoint", endpoint: "j" }; },
    { fields: m => retainedFields(m, [["Recorded location kind", 'text "strict_endpoint"'], ["Recorded location endpoint", 'text "j"']]) }],
  ["a location that claims a tie", "retained", m => { m.locations = { kind: "endpoint_candidates", exact_tie_proven: true, interior_equal_possible: "true" }; },
    { fields: m => retainedFields(m, [["Recorded location kind", 'text "endpoint_candidates"'], ["Recorded location exact_tie_proven", "true"], ["Recorded location interior_equal_possible", MARKER]]) }],
  ["a location that is not a record", "retained", m => { m.locations = ["i"]; }, { fields: m => retainedFields(m, [["Recorded location", MARKER]]) }],
  ["a location with none of the members shown", "retained", m => { m.locations = { station: 3 }; }, { fields: m => retainedFields(m, [["Recorded location", MARKER]]) }],
  ["a value outside its endpoints, a zero error bound and a zero parsed from 0.0", "retained", m => { m.value_pa = m.value_upper_pa + 1; m.absolute_error_bound_pa = JSON.parse("0.0"); m.relative_limit = JSON.parse("-0.0"); },
    { fields: m => [["Recorded basis", quoted(m.basis)], ["Recorded coefficient basis", quoted(m.coefficient_basis)], ["Recorded value (Pa)", bare(m.value_upper_pa + 1)], ["Recorded absolute error bound (Pa)", "0"],
      ["Recorded relative error bound", bare(m.relative_error_bound)], ["Recorded relative limit", "0"], ["Recorded station fraction", bare(m.station_fraction)], ...locationOf(m)] }],
  ["a value given as text and an empty basis", "retained", m => { m.value_pa = "12"; m.basis = ""; },
    { fields: m => [["Recorded basis", 'text ""'], ["Recorded coefficient basis", quoted(m.coefficient_basis)], ["Recorded value (Pa)", MARKER], ...retainedFields(m, locationOf(m)).slice(3)] }],
];
it.each(edits)('shows %s as labelled values and asserts nothing about it', (_name, kind, edit, expected) => {
  const result = fixture("mixed-sparse_interactive");
  const maximum = maximaOf(result).find((m: any) => kind === "retained" ? m.basis !== undefined : m.basis === undefined);
  const pipe = maximum.pipe_id, id = maximum.result_id, at = positionOf(result, maximum);
  edit(maximum);
  render(<PhysicsSourceEvidencePanel result={result} />);
  const row = screen.getByTestId(`physics-source-maximum-${at}`), shown = listsOf(row, at);
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
  const maximum = maximaOf(result)[0], id = maximum.result_id, at = positionOf(result, maximum);
  maximum.pipe_id = 9;
  render(<PhysicsSourceEvidencePanel result={result} />);
  expect(listsOf(screen.getByTestId(`physics-source-maximum-${at}`), at).identity).toEqual([["Recorded pipe", MARKER], ["Recorded result", quoted(id)]]);
});

it('gives two records the same test identifiers when they differ only in their recorded identifiers', () => {
  // Identifiers go by position: a recorded result identifier that repeats, or that spells another
  // row's list, selects nothing.
  const result = fixture("mixed-sparse_interactive");
  const [first, second] = maximaOf(result).filter((m: any) => positionOf(result, m).startsWith(positionOf(result, maximaOf(result)[0]).split("-")[0] + "-"));
  const at = [positionOf(result, first), positionOf(result, second)];
  second.result_id = first.result_id;
  first.pipe_id = "identity-x";
  render(<PhysicsSourceEvidencePanel result={result} />);
  expect(at[0]).not.toBe(at[1]);
  expect(listsOf(screen.getByTestId(`physics-source-maximum-${at[0]}`), at[0]).identity).toEqual([["Recorded pipe", quoted("identity-x")], ["Recorded result", quoted(first.result_id)]]);
  expect(listsOf(screen.getByTestId(`physics-source-maximum-${at[1]}`), at[1]).identity).toEqual([["Recorded pipe", quoted(second.pipe_id)], ["Recorded result", quoted(first.result_id)]]);
});

// The spelling of a recorded text. Each text is put in a retained record's basis, rendered, and read
// from the value element. What is asserted is the element's own text: the word "text", then a
// spelling made only of characters from U+0020 to U+007E with no two adjacent spaces, which parses
// as a JSON string to the recorded text. So nothing in a recorded text can change how the
// characters around it are displayed, and two different recorded texts never render the same.
const SPELLINGS: Array<[string, string]> = [
  ["direction controls", "\u202Etniopdne_tcirts\u202C"],
  ["zero-width characters", "strict\u200B_end\u200Dpoint\uFEFF"],
  ["repeated, leading and trailing spaces", "  a   b  "],
  ["a lone surrogate", "a\uD800b"],
  ["a look-alike letter", "strict_endp\u043Eint"],
  ["a line separator and a line break", "a\u2028b\nc"],
  ["a literal backslash-u escape", "\\u202e and \\u0020"],
  ["the marker's own words", MARKER],
  ["the empty text", ""],
  ["a no-break space, a tab and a delete character", "a\u00A0b\tc\u007Fd"],
  ["a character beyond the basic plane", "a\u{1F600}b"],
  ["quotation marks and a backslash", 'a "b" \\ c'],
  ["the word text", "text"],
];
const basisValue = (recorded: unknown): string => {
  const result = fixture("mixed-sparse_interactive");
  const maximum = maximaOf(result).find((m: any) => m.basis !== undefined), at = positionOf(result, maximum);
  maximum.basis = recorded;
  const view = render(<PhysicsSourceEvidencePanel result={result} />);
  const [label, value] = pairs(screen.getByTestId(`physics-source-maximum-fields-${at}`))[0];
  view.unmount();
  expect(label).toBe("Recorded basis");
  return value;
};
it.each(SPELLINGS)('spells a recorded text holding %s in printable ASCII that parses back to it', (_name, recorded) => {
  const value = basisValue(recorded);
  expect(value).toMatch(/^text "[\x20-\x7e]*"$/);
  expect(value).not.toMatch(/ {2}/);
  expect(JSON.parse(value.slice("text ".length))).toBe(recorded);
});

it('shows direction controls as visible escapes, so the text cannot display as another text', () => {
  // The sixth reading's case: with the controls live, the reversed text displays as strict_endpoint.
  expect(basisValue("\u202Etniopdne_tcirts\u202C")).toBe('text "\\u202etniopdne_tcirts\\u202c"');
  expect(basisValue("strict_endpoint")).toBe('text "strict_endpoint"');
  expect(basisValue("a  b")).toBe('text "a \\u0020b"');
});

it('puts the word "text" first in a recorded text equal to the marker, and no such word in the marker', () => {
  // In reading order the first word settles the kind, with or without the quotation marks: a value
  // that begins with "text" is a recorded text; the marker, a number and a flag never begin so.
  expect(basisValue(MARKER)).toBe('text "Unavailable to display"');
  for (const notText of [7, null, ["Unavailable to display"], { text: "x" }, true]) expect(basisValue(notText)).toBe(MARKER);
  expect(MARKER.startsWith("text")).toBe(false);
  for (const name of ["n05", "mixed"]) {
    const view = render(<PhysicsSourceEvidencePanel result={fixture(`${name}-sparse_interactive`)} />);
    const values = Array.from(document.querySelectorAll("dl")).flatMap(list => pairs(list as HTMLElement));
    expect(values.length).toBeGreaterThan(0);
    // Every value is a recorded text, a number as it prints, a flag, or the marker.
    for (const [, value] of values) expect(value).toMatch(/^(text "[\x20-\x7e]*"|-?[0-9][0-9.e+-]*|true|false|Unavailable to display)$/);
    view.unmount();
  }
});

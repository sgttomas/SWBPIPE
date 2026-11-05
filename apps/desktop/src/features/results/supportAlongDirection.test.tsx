// R-6/R-7 S-K1 (DEL-04-03 Work `skew-restraints.md` §7.1; R-1's desktop change, with authoring's
// should-fix and Agent 0's corrections 1-3): how the webview treats
// `support_reaction_along_direction_v1`, on the committed product fixtures written by
// product_physics' `the_reader_fixture_is_the_product_output` (one case and 2·case:
// `skew_along_direction_preview.json`) and `the_combinations_reader_fixture_is_the_product_output`
// (two cases; signed, absolute-sum, SRSS, range-envelope and subtraction combinations:
// `skew_along_direction_combinations_preview.json`). Invented models; no mocks of the reader.
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import single from "../../../../../fixtures/results/skew_along_direction_preview.json";
import combinations from "../../../../../fixtures/results/skew_along_direction_combinations_preview.json";
import type { MechanicsResult, PreviewModel } from "../../types";
import { declaredSourceResultDimension } from "../../services/previewService";
import { DisplayUnitsProvider } from "../display-units";
import { preambleValue, readCsv, numericField, textField } from "../../test/csvRead";
import { buildReactionOverlay, REACTION_COMPONENT_KIND, type ReactionOverlay } from "../viewport/reactionOverlay";
import { resultOverlayCases } from "../viewport/resultColourOverlay";
import { numericalHealthView, parseNumericalHealth, type ResultOverlayStanding } from "./numericalHealth";
import { knownSemanticNotices, N_ALONG_DIRECTION, PREVIEW_ALONG_DIRECTION_KIND, resultRowLabel } from "./knownSemanticLimitations";
import { readerValidate } from "./resultReader";
import { ResultsPanel } from "./ResultsPanel";
import { SUPPORT_LOAD_COLUMNS, SUPPORT_ALONG_DIRECTION_KIND, SUPPORT_ALONG_MULTIPLE, buildSupportLoadSummaries } from "./supportLoadSummary";
import { supportLoadCsv } from "./supportLoadSummaryCsv";
import { SUPPORT_ALONG_NONE_TEXT, SupportLoadSummaryPanel } from "./SupportLoadSummaryPanel";

const saved: Array<{ text: string; fileName: string }> = [];
vi.mock("../result-export/csvSave", () => ({
  saveCsvTable: vi.fn(async (input: { text: string; fileName: string }) => {
    saved.push(input);
    return { mode: "browser", file_name: input.fileName, byte_count: input.text.length };
  })
}));
afterEach(() => {
  cleanup();
  saved.length = 0;
});

type Fixture = { envelope: unknown; request: { model: unknown } };
const source = (fixture: Fixture = single) => structuredClone(fixture.envelope) as MechanicsResult;
const model = (fixture: Fixture = single) => structuredClone(fixture.request.model) as PreviewModel;
const KIND = "support_reaction_along_direction_v1";
const withoutAlong = (result: MechanicsResult): MechanicsResult => ({ ...result, results: result.results.filter(row => row.kind !== KIND) });

function standing(states: Record<string, "checks_passed" | "sensitive">, combinationStates: Record<string, "checks_passed" | "sensitive">): ResultOverlayStanding {
  const parsed = parseNumericalHealth({
    route: "generic", verdict_only: false,
    cases: Object.entries(states).map(([caseId, state]) => ({ case_id: caseId, state, code: null, integrity_diagnostic_id: null, rcond: null, digits: null, conditioning_sensitive: false, causes: [] })),
    combinations: Object.entries(combinationStates).map(([id, state]) => ({ combination_id: id, state, cases: Object.keys(states), withheld: false }))
  });
  return { eligible: true, health: numericalHealthView(parsed, { eligible: true, verdictHold: false }) };
}
const PASSING = standing({ "case:skew": "checks_passed" }, { C2: "checks_passed" });
const COMBINED = ["C_SIGNED", "C_ABS", "C_SRSS", "C_ENV", "C_DIFF"];
const PASSING_TWO = standing({ "case:skew": "checks_passed", "case:b": "checks_passed" }, Object.fromEntries(COMBINED.map(id => [id, "checks_passed" as const])));
const rowOf = (result: MechanicsResult, entity: string, refId: string, kind = KIND, component?: string) =>
  result.results.find(row => row.kind === kind && row.entity_ref === entity && row.basis_ref?.ref_id === refId && (component === undefined || row.metadata?.component === component))!;

/** Every observable part of the overlay: its state and counts, and each Map entry in full. */
function overlayView(overlay: ReactionOverlay) {
  return {
    ...overlay,
    forces: [...overlay.forces].map(([id, vector]) => [id, { ...vector }]),
    moments: [...overlay.moments].map(([id, vector]) => [id, { ...vector }])
  };
}

describe("support_reaction_along_direction_v1 in the webview", () => {
  it("keeps the exact kind string, and the webview reader admits both product documents", () => {
    expect(SUPPORT_ALONG_DIRECTION_KIND).toBe(KIND);
    expect(PREVIEW_ALONG_DIRECTION_KIND).toBe(KIND);
    expect(source().results.filter(row => row.kind === KIND)).toHaveLength(4);
    expect(source(combinations).results.filter(row => row.kind === KIND)).toHaveLength(14);
    expect(() => readerValidate("preview-physics", source())).not.toThrow();
    expect(() => readerValidate("preview-physics", source(combinations))).not.toThrow();
  });

  it("is a known kind: one signed force, with its own row label and notice", () => {
    for (const row of source(combinations).results.filter(item => item.kind === KIND)) {
      expect(declaredSourceResultDimension(row)).toBe("force");
      expect(resultRowLabel(row, source(combinations))).toBe(N_ALONG_DIRECTION);
    }
    const notices = knownSemanticNotices(source());
    expect(notices.filter(notice => notice.id === "along-direction-label")).toEqual([{ id: "along-direction-label", text: `Along-direction reaction: ${N_ALONG_DIRECTION}.` }]);
    expect(knownSemanticNotices(withoutAlong(source())).some(notice => notice.id === "along-direction-label")).toBe(false);
  });

  // Agent 0's correction 1: the reaction overlay, compared in full with and without the scalar.
  it("leaves every reaction arrow unchanged: the overlay is the same with and without the along rows", () => {
    for (const [fixture, states] of [[single, PASSING], [combinations, PASSING_TWO]] as const) {
      const result = source(fixture);
      const cases = resultOverlayCases(model(fixture), result);
      expect(cases.length).toBeGreaterThan(1);
      for (const overlayCase of cases) {
        const withScalar = buildReactionOverlay({ model: model(fixture), result, standing: states, overlayCase });
        const plain = buildReactionOverlay({ model: model(fixture), result: withoutAlong(result), standing: states, overlayCase });
        expect(overlayView(withScalar)).toEqual(overlayView(plain));
        if (overlayCase.basis.ref_type === "combination" && fixture === combinations && overlayCase.basis.ref_id !== "C_SIGNED" && overlayCase.basis.ref_id !== "C_DIFF") continue;
        // The arrows are the emitted global Fx/Fy/Fz, drawn for each skewed record.
        expect(withScalar.state).toBe("available");
        for (const id of ["stop_B", "spring_B"]) {
          const vector = withScalar.forces.get(id)!;
          const [fx, fy, fz] = ["Fx", "Fy", "Fz"].map(c => rowOf(result, id, overlayCase.basis.ref_id, REACTION_COMPONENT_KIND, c).value);
          expect(vector.components).toEqual({ x: fx, y: fy, z: fz });
          expect(vector.length).toBeGreaterThan(0);
          expect(vector.displayLength).toBeGreaterThan(0);
          expect(vector.direction.x).toBeCloseTo(fx / vector.length, 15);
          expect(vector.emittedMagnitude).toBe(rowOf(result, id, overlayCase.basis.ref_id, "support_reaction_force_magnitude_v2", "force_magnitude").value);
        }
      }
    }
    // case:b's along value is negative (the reaction acts against +d): its arrows are still the components'.
    expect(rowOf(source(combinations), "stop_B", "case:b").value).toBeLessThan(0);
    // A withheld case: the same held overlay with or without the scalar.
    const held = standing({ "case:skew": "checks_passed", "case:b": "sensitive" }, Object.fromEntries(COMBINED.map(id => [id, "checks_passed" as const])));
    const caseB = resultOverlayCases(model(combinations), source(combinations)).find(item => item.basis.ref_id === "case:b")!;
    const withScalar = buildReactionOverlay({ model: model(combinations), result: source(combinations), standing: held, overlayCase: caseB });
    expect(withScalar.state).toBe("held");
    expect(withScalar.heldRowCount).toBeGreaterThan(0);
    expect(overlayView(withScalar)).toEqual(overlayView(buildReactionOverlay({ model: model(combinations), result: withoutAlong(source(combinations)), standing: held, overlayCase: caseB })));
    expect(REACTION_COMPONENT_KIND).toBe("support_reaction_component_v2");
  });

  it("the support summary lists it per case beside the global columns, never as a column or extreme", () => {
    const summaries = buildSupportLoadSummaries({ model: model(), result: source(), standing: PASSING });
    expect(SUPPORT_LOAD_COLUMNS.some(column => column.kind === KIND)).toBe(false);
    for (const supportId of ["stop_B", "spring_B"]) {
      const summary = summaries.supports.find(item => item.supportId === supportId)!;
      const stored = model().supports.find(item => item.id === supportId) as unknown as { direction: unknown };
      expect(summary.direction).toEqual(stored.direction);
      for (const row of summary.rows) {
        const emitted = rowOf(source(), supportId, row.case.basis.ref_id);
        expect(row.along).toEqual({ value: emitted.value, unit: "N", resultId: emitted.id, signConvention: emitted.metadata!.sign_convention, derivation: null });
        expect(row.alongDetail).toBeNull();
        // The global components stay the emitted ones (the along row is no column).
        expect(row.reactionAbsence).toBeNull();
        expect(Object.keys(row.values)).toEqual(expect.arrayContaining(["Fx", "Fy", "Fz"]));
        expect(Object.values(row.values).some(value => value?.resultId === emitted.id)).toBe(false);
      }
      expect(summary.extremes.every(extreme => extreme.max?.resultId !== rowOf(source(), supportId, "case:skew").id)).toBe(true);
    }
    const axis = summaries.supports.find(item => item.supportId === "anchor_A")!;
    expect(axis.direction).toBeNull();
    expect(axis.rows.every(row => row.along === null)).toBe(true);
  });

  // Agent 0's correction 2: each row's own derivation and sign meaning, on the real-reader fixture.
  it("states each along value with its own row's derivation and sign convention: signed, unsigned and selected", () => {
    const result = source(combinations);
    const summary = buildSupportLoadSummaries({ model: model(combinations), result, standing: PASSING_TWO }).supports.find(item => item.supportId === "stop_B")!;
    const expected: Record<string, string | null> = {
      "case:skew": null, "case:b": null, C_SIGNED: null, C_DIFF: null,
      C_ABS: "absolute sum, unsigned", C_SRSS: "SRSS, unsigned", C_ENV: "range envelope"
    };
    expect(summary.rows.map(row => row.case.basis.ref_id).sort()).toEqual(Object.keys(expected).sort());
    for (const row of summary.rows) {
      const emitted = rowOf(result, "stop_B", row.case.basis.ref_id);
      expect(row.along, row.case.key).toEqual({ value: emitted.value, unit: "N", resultId: emitted.id, signConvention: emitted.metadata!.sign_convention, derivation: expected[row.case.basis.ref_id] });
    }
    const signConvention = (id: string) => summary.rows.find(row => row.case.basis.ref_id === id)!.along!.signConvention!;
    expect(signConvention("case:skew")).toBe("support_on_pipe_positive_along_authored_direction");
    expect(signConvention("C_ABS")).toMatch(/^unsigned value: the absolute sum/);
    expect(signConvention("C_SRSS")).toMatch(/^unsigned value: the square root of the sum of squares/);
    expect(signConvention("C_ENV")).toMatch(/^value is the max mode-selected source result/);
    expect(signConvention("C_DIFF")).toMatch(/result-state subtraction/);
    // No combination row is described with the load case's physical sign convention.
    for (const id of COMBINED) expect(signConvention(id)).not.toBe("support_on_pipe_positive_along_authored_direction");
    // The panel shows each value with its derivation and its own convention.
    render(<SupportLoadSummaryPanel model={model(combinations)} result={result} standing={PASSING_TWO} focusSupportId="stop_B" />);
    expect(screen.getByTestId("support-load-along-note")).toHaveTextContent(`(${KIND}; d = (3, 4, 0), as stored): the emitted value per case, each with its own row's derivation and sign convention.`);
    expect(screen.getByTestId("support-load-along-combination:C_ABS")).toHaveTextContent("absolute sum, unsigned");
    expect(screen.getByTestId("support-load-along-sign-combination:C_ABS")).toHaveTextContent(`(sign convention: ${signConvention("C_ABS")})`);
    expect(screen.getByTestId("support-load-along-combination:C_ENV")).toHaveTextContent("range envelope");
    expect(screen.getByTestId("support-load-along-sign-load_case:case:skew")).toHaveTextContent("(sign convention: support_on_pipe_positive_along_authored_direction)");
  });

  it("two along rows for one support and case list none, with the stated reason, never the first", () => {
    const twice = (cases: string[]) => {
      const result = source();
      for (const refType of cases) {
        const emitted = result.results.find(row => row.kind === KIND && row.entity_ref === "stop_B" && row.basis_ref?.ref_type === refType)!;
        result.results.push({ ...structuredClone(emitted), id: `${emitted.id}:again`, value: emitted.value * 2 });
      }
      return buildSupportLoadSummaries({ model: model(), result, standing: PASSING }).supports.find(item => item.supportId === "stop_B")!;
    };
    const one = twice(["load_case"]);
    const doubled = one.rows.find(row => row.case.basis.ref_type === "load_case")!;
    expect(doubled.along).toBeNull();
    expect(doubled.alongDetail).toBe(SUPPORT_ALONG_MULTIPLE);
    expect(doubled.alongDetail).toBe("more than one along-direction row was emitted for this case; none is listed");
    // The global columns are unaffected, and the combination's single row is still listed.
    expect(Object.keys(doubled.values)).toEqual(expect.arrayContaining(["Fx", "Fy", "Fz"]));
    const listed = one.rows.find(row => row.case.basis.ref_type === "combination")!;
    expect(listed.along?.resultId).toBe(rowOf(source(), "stop_B", "C2").id);
    expect(listed.alongDetail).toBeNull();
    const both = twice(["load_case", "combination"]);
    expect(both.rows.every(row => row.along === null && row.alongDetail === SUPPORT_ALONG_MULTIPLE)).toBe(true);
    // The panel states the reason in place of the value.
    const result = source();
    const emitted = rowOf(result, "stop_B", "case:skew");
    result.results.push({ ...structuredClone(emitted), id: `${emitted.id}:again` });
    render(<SupportLoadSummaryPanel model={model()} result={result} standing={PASSING} focusSupportId="stop_B" />);
    expect(screen.getByTestId("support-load-along-load_case:case:skew")).toHaveTextContent(`case:skew: ${SUPPORT_ALONG_MULTIPLE}`);
  });

  it("a withheld case lists no along value", () => {
    const withheld: ResultOverlayStanding = { ...PASSING, eligible: false };
    const summary = buildSupportLoadSummaries({ model: model(), result: source(), standing: withheld }).supports.find(item => item.supportId === "stop_B")!;
    expect(summary.rows.every(row => row.standing === "withheld" && row.along === null && row.alongDetail === null)).toBe(true);
    render(<SupportLoadSummaryPanel model={model()} result={source()} standing={withheld} focusSupportId="stop_B" />);
    expect(screen.getByTestId("support-load-along-none")).toHaveTextContent(SUPPORT_ALONG_NONE_TEXT);
  });

  it("the panel states the along reaction for a skewed support, and nothing for an axis-aligned one", () => {
    render(<SupportLoadSummaryPanel model={model()} result={source()} standing={PASSING} focusSupportId="stop_B" />);
    expect(screen.getByTestId("support-load-along-note")).toHaveTextContent("not a column, not in the maximum and minimum, and not included in this global-component CSV");
    expect(screen.getByTestId("support-load-along-load_case:case:skew")).toHaveTextContent("case:skew: 1276.86 N");
    cleanup();
    render(<SupportLoadSummaryPanel model={model()} result={source()} standing={PASSING} focusSupportId="anchor_A" />);
    expect(screen.queryByTestId("support-load-along")).toBeNull();
  });

  // Agent 0's correction 3: out of the global-component CSV, in the ordinary results-row CSV.
  it("stays out of the support-load CSV and is exported by the ordinary results-row CSV with its force dimension", async () => {
    const summary = buildSupportLoadSummaries({ model: model(), result: source(), standing: PASSING }).supports.find(item => item.supportId === "stop_B")!;
    const along = new Set(source().results.filter(row => row.kind === KIND).map(row => row.id));
    const table = supportLoadCsv({ summary, provenance: { modelHash: "m", resultHash: "r", analysisRoutes: ["preview-physics"] }, softwareVersion: "test", unitBasis: "entered" } as never);
    expect(JSON.stringify(table.table)).not.toContain(KIND);
    for (const id of along) expect(JSON.stringify(table.table)).not.toContain(id);

    render(<DisplayUnitsProvider initialPreference="SI"><ResultsPanel result={source()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} /></DisplayUnitsProvider>);
    const buttons = screen.getAllByRole("button", { name: /^Export CSV: .* results$/ });
    await waitFor(() => expect(document.querySelectorAll('[data-display-status="pending"]').length).toBe(0));
    for (const button of buttons) fireEvent.click(button);
    await waitFor(() => expect(saved.length).toBe(buttons.length));
    const exported = new Map<string, { csv: ReturnType<typeof readCsv>; line: string[] }>();
    for (const item of saved) {
      const csv = readCsv(item.text);
      for (const line of csv.rows) {
        const id = textField(csv, line, "Result id");
        if (along.has(id)) exported.set(id, { csv, line });
      }
    }
    expect([...exported.keys()].sort()).toEqual([...along].sort());
    for (const [id, { csv, line }] of exported) {
      const row = source().results.find(item => item.id === id)!;
      expect(textField(csv, line, "Row label")).toBe(N_ALONG_DIRECTION);
      expect(textField(csv, line, "Entity")).toBe(row.entity_ref);
      expect(textField(csv, line, "Location")).toBe("node");
      expect(textField(csv, line, "Frame")).toBe("support_direction");
      const entered = numericField(csv, line, "Entered value");
      expect(Object.is(Number(entered.field), row.value)).toBe(true);
      expect(entered.unit).toBe("N");
      expect(preambleValue(csv, "unit_basis")).toContain("force N");
    }
  });
});

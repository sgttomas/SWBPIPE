// "Within this run" (W-12 S3c; DEL-14-01 Design §4.3): the options, the default
// (today's fixed pair when offered), the choice held with its run, the stale-id
// guard, the empty states, the packaged comparison and the panel's selects. The
// comparison itself is `buildPreviewComparison`, unchanged. Invented data only.
import rangeEnvelopeResult from "../../../../../fixtures/results/preview_physics_range_envelope_sparse.json";
import rangeEnvelopeModel from "../../../../../core/product_physics/tests/fixtures/preview_physics_range_envelope_model.json";
import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import {
  buildPreviewComparison,
  DEFAULT_COMPARISON_LEFT_BASIS_REF,
  DEFAULT_COMPARISON_RIGHT_BASIS_REF
} from "../../services/previewService";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { ComparisonPanel, type WithinRunControls } from "./ComparisonPanel";
import {
  defaultWithinRunPair,
  heldPairForRun,
  NO_COMBINATION_ROWS,
  NO_LOAD_CASE_OR_COMBINATION_ROWS,
  NO_LOAD_CASE_ROWS,
  packagedComparison,
  resolveWithinRun,
  WITHIN_RUN_BUSY_PROJECT,
  WITHIN_RUN_BUSY_REPORT,
  WITHIN_RUN_BUSY_SOLVE,
  withinRunBasisOptions,
  withinRunBusyReason,
  type WithinRunView
} from "./withinRunComparison";

const RANGE = "combination:U4B-RANGE-MAX";
/** The producer's range-envelope combination rows include kinds the unchanged builder
 * declares no source dimension for (it refuses them: the known gap routed to T6), so
 * the comparable fixture keeps the combination rows of declared kinds only. */
const UNDECLARED_COMBINATION_KINDS = /^(support_reaction_|combination_modulus_basis_record$)/;
const producerResult = () => structuredClone(rangeEnvelopeResult) as unknown as MechanicsResult;
const result = () => {
  const source = producerResult();
  source.results = source.results.filter((row) => !(row.basis_ref?.ref_type === "combination" && UNDECLARED_COMBINATION_KINDS.test(row.kind)));
  return source;
};
const model = () => structuredClone(rangeEnvelopeModel) as unknown as PreviewModel;
const analysisRun = (runId: string) =>
  ({ analysis_run: { run_id: runId, model_state_ref: { object_type: "ModelState", ref: "model-state:invented" } } }) as unknown as AnalysisRunEnvelope;
/** The range-envelope result with its combination rows moved to the fixed combination id. */
function withFixedCombination(): MechanicsResult {
  const source = result();
  for (const row of source.results) if (row.basis_ref?.ref_id === RANGE) row.basis_ref = { ...row.basis_ref, ref_id: DEFAULT_COMPARISON_RIGHT_BASIS_REF };
  return source;
}
function counts(source: MechanicsResult, refType: string) {
  const map = new Map<string, number>();
  for (const row of source.results) if (row.basis_ref?.ref_type === refType) map.set(row.basis_ref.ref_id, (map.get(row.basis_ref.ref_id) ?? 0) + 1);
  return map;
}

describe("the options", () => {
  it("are the load cases and combinations with result rows, with their counts, in the solved model's order", () => {
    const source = result();
    const options = withinRunBasisOptions(source, model());
    const caseCounts = counts(source, "load_case"), combinationCounts = counts(source, "combination");
    expect(options.loadCases.map((option) => option.ref_id)).toEqual(["load:L-100", "load:L-200"]);
    expect(options.loadCases.map((option) => option.row_count)).toEqual([caseCounts.get("load:L-100"), caseCounts.get("load:L-200")]);
    expect(options.loadCases[0].label).toBe("Invented operating gravity and pressure preview");
    // C-OPER-ALT is in the model but has no rows (withheld), so it is not offered.
    expect(model().combinations!.map((combination) => combination.id)).toContain(DEFAULT_COMPARISON_RIGHT_BASIS_REF);
    expect(options.combinations.map((option) => option.ref_id)).toEqual([RANGE]);
    expect(options.combinations[0].row_count).toBe(combinationCounts.get(RANGE));
  });

  it("follow the model's order, then list bases the model does not name in code-point order; other basis kinds are not offered", () => {
    const source = result();
    const base = source.results[0];
    source.results.push(
      { ...base, id: "invented:z", basis_ref: { ref_type: "load_case", ref_id: "load:Z" } },
      { ...base, id: "invented:a", basis_ref: { ref_type: "load_case", ref_id: "load:A" } },
      { ...base, id: "invented:other", basis_ref: { ref_type: "reference_configuration", ref_id: "load:other" } },
      { ...base, id: "invented:empty", basis_ref: { ref_type: "load_case", ref_id: "" } }
    );
    const reversed = model();
    reversed.load_cases = [...reversed.load_cases].reverse();
    expect(withinRunBasisOptions(source, reversed).loadCases.map((option) => option.ref_id)).toEqual(["load:L-200", "load:L-100", "load:A", "load:Z"]);
    // Without the solved model, every basis is in code-point order, unlabelled.
    const unlisted = withinRunBasisOptions(source, null);
    expect(unlisted.loadCases.map((option) => option.ref_id)).toEqual(["load:A", "load:L-100", "load:L-200", "load:Z"]);
    expect(unlisted.loadCases.every((option) => option.label === null)).toBe(true);
  });
});

describe("the default", () => {
  it("reproduces today's comparison when the fixed load case and combination are both offered", () => {
    const source = withFixedCombination(), run = analysisRun("run:invented");
    const options = withinRunBasisOptions(source, model());
    expect(defaultWithinRunPair(options)).toEqual({ left: DEFAULT_COMPARISON_LEFT_BASIS_REF, right: DEFAULT_COMPARISON_RIGHT_BASIS_REF });
    const today = buildPreviewComparison({ result: source, analysisRun: run });
    const view = resolveWithinRun({ result: source, analysisRun: run, model: model(), held: null, basis: "1:1" });
    expect(view.status === "ready" && view.source).toBe("default");
    expect(view.status === "ready" && view.comparison).toStrictEqual(today);
    expect(today.summary.comparable_result_pairs).toBeGreaterThan(0);
    // The session's fixed-pair comparison is reused as it is.
    const reused = resolveWithinRun({ result: source, analysisRun: run, model: model(), held: null, basis: "1:1", defaultComparison: today });
    expect(reused.status === "ready" && reused.comparison).toBe(today);
    // Without a choice, the packages carry exactly the fixed-pair comparison.
    expect(packagedComparison(view, today)).toBe(today);
  });

  it("is the first offered case and combination when the fixed pair is not offered", () => {
    const source = result(), run = analysisRun("run:invented");
    const view = resolveWithinRun({ result: source, analysisRun: run, model: model(), held: null, basis: "1:1" });
    expect(view.status).toBe("ready");
    if (view.status !== "ready") return;
    expect(view.pair).toEqual({ left: "load:L-100", right: RANGE });
    expect(view.source).toBe("default");
    // The comparison is the unchanged builder's for that pair.
    const expected = buildPreviewComparison({ result: source, analysisRun: run, leftBasisRef: "load:L-100", rightBasisRef: RANGE });
    expect(view.comparison).toStrictEqual(expected);
    expect(expected.summary.comparable_result_pairs).toBeGreaterThan(0);
    // The packages keep the fixed-pair comparison until the user chooses.
    const today = buildPreviewComparison({ result: source, analysisRun: run });
    expect(packagedComparison(view, today)).toBe(today);
  });

  it("is nothing, with the reason, when no load case or no combination has result rows", () => {
    const run = analysisRun("run:invented");
    const noCombination = result();
    noCombination.results = noCombination.results.filter((row) => row.basis_ref?.ref_type !== "combination");
    const noCase = result();
    noCase.results = noCase.results.filter((row) => row.basis_ref?.ref_type !== "load_case");
    const neither = result();
    neither.results = neither.results.filter((row) => !row.basis_ref);
    const reasons = [noCombination, noCase, neither].map((source) => {
      const view = resolveWithinRun({ result: source, analysisRun: run, model: model(), held: null, basis: "1:1" });
      return view.status === "unavailable" ? view.reason : view.status;
    });
    expect(reasons).toEqual([NO_COMBINATION_ROWS, NO_LOAD_CASE_ROWS, NO_LOAD_CASE_OR_COMBINATION_ROWS]);
    expect(resolveWithinRun({ result: null, analysisRun: run, model: null, held: null, basis: "1:1" })).toEqual({ status: "no_result" });
    expect(resolveWithinRun({ result: result(), analysisRun: null, model: null, held: null, basis: "1:1" })).toEqual({ status: "no_result" });
  });
});

describe("the choice", () => {
  it("applies to the run it was made on, and the packages then carry it", () => {
    const source = result(), run = analysisRun("run:invented");
    const held = { basis: "4:2", left: "load:L-200", right: RANGE };
    const view = resolveWithinRun({ result: source, analysisRun: run, model: model(), held, basis: "4:2" });
    expect(view.status === "ready" && [view.source, view.pair]).toEqual(["selected", { left: "load:L-200", right: RANGE }]);
    const today = buildPreviewComparison({ result: source, analysisRun: run });
    expect(packagedComparison(view, today)).toBe(view.status === "ready" ? view.comparison : undefined);
    if (view.status === "ready" && view.comparison) {
      expect(view.comparison.left.basis_ref.ref).toBe("load:L-200");
      expect(view.comparison.right.basis_ref.ref).toBe(RANGE);
      expect(view.comparison.summary.tolerance_status).toBe("not_tolerance_checked");
      expect(view.comparison.summary.tolerance_profile_ref).toBe("TBD");
      expect(view.comparison.unit_policy_evidence.conversion_performed).toBe(false);
    }
  });

  it("reads as the default on any other run, and a held id the run does not offer is never compared", () => {
    const options = withinRunBasisOptions(result(), model());
    const held = { basis: "4:2", left: "load:L-200", right: RANGE };
    expect(heldPairForRun(held, "4:2", options)).toEqual({ left: "load:L-200", right: RANGE });
    // A new solve, an edit or a replacement gives a new basis key.
    expect(heldPairForRun(held, "5:3", options)).toBeNull();
    // The same key with an id this run does not offer: never compared.
    expect(heldPairForRun({ ...held, left: "load:L-999" }, "4:2", options)).toBeNull();
    expect(heldPairForRun({ ...held, right: DEFAULT_COMPARISON_RIGHT_BASIS_REF }, "4:2", options)).toBeNull();
    // A load case id offered as the combination (or the reverse) is not offered.
    expect(heldPairForRun({ ...held, right: "load:L-100" }, "4:2", options)).toBeNull();
    const stale = resolveWithinRun({ result: result(), analysisRun: analysisRun("run:invented"), model: model(), held: { ...held, left: "load:L-999" }, basis: "4:2" });
    expect(stale.status === "ready" && [stale.source, stale.pair.left]).toEqual(["default", "load:L-100"]);
  });

  it("states the builder's refusal instead of comparing, and the packages then carry no comparison", () => {
    // The producer's own range-envelope rows: the unchanged builder refuses their
    // undeclared kinds (the known gap routed to T6).
    const source = producerResult(), run = analysisRun("run:invented");
    const held = { basis: "1:1", left: "load:L-100", right: RANGE };
    const view = resolveWithinRun({ result: source, analysisRun: run, model: model(), held, basis: "1:1" });
    expect(view.status === "ready" && view.comparison).toBeNull();
    expect(view.status === "ready" && view.refusal).toContain("ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED");
    expect(packagedComparison(view, null)).toBeNull();
  });
});

it("states why the choice is disabled while the session is busy", () => {
  expect(withinRunBusyReason({ projectBusy: false, running: false, reportPackageBusy: false })).toBeNull();
  expect(withinRunBusyReason({ projectBusy: true, running: true, reportPackageBusy: true })).toBe(WITHIN_RUN_BUSY_PROJECT);
  expect(withinRunBusyReason({ projectBusy: false, running: true, reportPackageBusy: true })).toBe(WITHIN_RUN_BUSY_SOLVE);
  expect(withinRunBusyReason({ projectBusy: false, running: false, reportPackageBusy: true })).toBe(WITHIN_RUN_BUSY_REPORT);
});

describe("the panel", () => {
  const panel = (view: WithinRunView, overrides: Partial<WithinRunControls> = {}) => {
    const onSelect = vi.fn();
    const controls: WithinRunControls = { view, disabledReason: null, onSelect, packagedComparisonId: view.status === "ready" ? view.comparison?.comparison_id ?? null : null, ...overrides };
    const rendered = render(<ComparisonPanel comparison={view.status === "ready" ? view.comparison : null} result={null} onSelectResult={() => {}} withinRun={controls} />);
    return { onSelect, rendered };
  };

  it("offers labelled selects of the run's bases, and a change chooses both ids", () => {
    const source = withFixedCombination(), run = analysisRun("run:invented");
    const view = resolveWithinRun({ result: source, analysisRun: run, model: model(), held: null, basis: "1:1" });
    const { onSelect } = panel(view);
    const left = screen.getByLabelText("Load case") as HTMLSelectElement;
    const right = screen.getByLabelText("Combination") as HTMLSelectElement;
    expect(left).toBe(screen.getByTestId("comparison-left-select"));
    expect(right).toBe(screen.getByTestId("comparison-right-select"));
    expect(left.value).toBe(DEFAULT_COMPARISON_LEFT_BASIS_REF);
    expect(right.value).toBe(DEFAULT_COMPARISON_RIGHT_BASIS_REF);
    expect(left.disabled || right.disabled).toBe(false);
    expect([...left.options].map((option) => option.value)).toEqual(["load:L-100", "load:L-200"]);
    expect(left.options[0].textContent).toMatch(/^Invented operating gravity and pressure preview \(load:L-100\); \d+ result rows$/);
    expect(screen.getByRole("group", { name: "Within this run" })).toBeTruthy();
    expect(screen.getByTestId("comparison-summary")).toHaveTextContent(`${DEFAULT_COMPARISON_LEFT_BASIS_REF};`);
    // The shown comparison is the packaged one: no note.
    expect(screen.queryByTestId("comparison-package-basis")).toBeNull();
    fireEvent.change(left, { target: { value: "load:L-200" } });
    expect(onSelect).toHaveBeenLastCalledWith({ left: "load:L-200", right: DEFAULT_COMPARISON_RIGHT_BASIS_REF });
    // No wording of standing.
    expect(screen.getByTestId("comparison-panel").textContent).not.toMatch(/certif|accept|approv|compliant|pass\b|fail\b/i);
  });

  it("is disabled with the stated reason while busy", () => {
    const view = resolveWithinRun({ result: result(), analysisRun: analysisRun("run:invented"), model: model(), held: null, basis: "1:1" });
    panel(view, { disabledReason: WITHIN_RUN_BUSY_SOLVE });
    for (const testId of ["comparison-left-select", "comparison-right-select"]) {
      const select = screen.getByTestId(testId) as HTMLSelectElement;
      expect(select.disabled).toBe(true);
      expect(select.title).toBe(WITHIN_RUN_BUSY_SOLVE);
      expect(select.getAttribute("aria-describedby")).toBe("comparison-within-run-disabled-reason");
    }
    expect(screen.getByTestId("comparison-within-run-disabled-reason")).toHaveTextContent(WITHIN_RUN_BUSY_SOLVE);
  });

  it("says plainly that there is nothing to compare, instead of an empty comparison", () => {
    const source = result();
    source.results = source.results.filter((row) => row.basis_ref?.ref_type !== "combination");
    const run = analysisRun("run:invented");
    const view = resolveWithinRun({ result: source, analysisRun: run, model: model(), held: null, basis: "1:1" });
    const today = buildPreviewComparison({ result: source, analysisRun: run });
    panel(view, { packagedComparisonId: today.comparison_id });
    expect(screen.getByTestId("comparison-unavailable")).toHaveTextContent(NO_COMBINATION_ROWS);
    expect(screen.queryByTestId("comparison-summary")).toBeNull();
    expect(screen.queryByTestId("comparison-left-select")).toBeNull();
    // The packages keep the fixed-pair comparison, and the panel says so.
    expect(screen.getByTestId("comparison-package-basis")).toHaveTextContent(today.comparison_id);
  });

  it("names the builder's refusal", () => {
    const view: WithinRunView = { status: "ready", options: withinRunBasisOptions(result(), model()), pair: { left: "load:L-100", right: RANGE }, source: "selected", comparison: null, refusal: "ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED: invented" };
    panel(view);
    expect(screen.getByTestId("comparison-refused")).toHaveTextContent("Not compared: ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED: invented");
    expect(screen.queryByTestId("comparison-summary")).toBeNull();
  });

  it("without the session's controls, shows the given comparison as before", () => {
    const source = withFixedCombination(), run = analysisRun("run:invented");
    const today = buildPreviewComparison({ result: source, analysisRun: run });
    const rendered = render(<ComparisonPanel comparison={today} result={null} onSelectResult={() => {}} />);
    expect(within(rendered.container).queryByTestId("comparison-within-run")).toBeNull();
    expect(within(rendered.container).getByTestId("comparison-summary")).toBeTruthy();
    rendered.unmount();
    render(<ComparisonPanel comparison={null} result={null} onSelectResult={() => {}} withinRun={{ view: { status: "no_result" }, disabledReason: null, onSelect: () => {}, packagedComparisonId: null }} />);
    expect(screen.getByTestId("comparison-empty")).toBeTruthy();
    expect(screen.queryByTestId("comparison-within-run")).toBeNull();
  });
});

// "Within this run" (W-12 S3c): the options, the default
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
  DEFAULT_COMPARISON_RIGHT_BASIS_REF,
  escapeComparisonBasis,
  previewComparisonBases,
  previewComparisonId
} from "../../services/previewService";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { ComparisonPanel, type WithinRunControls } from "./ComparisonPanel";
import {
  codePointCompare,
  buildWithinRunComparison,
  defaultWithinRunPair,
  droppedBases,
  droppedNotice,
  fixedPairComparison,
  basisName,
  COMPARISON_BASIS_WITHOUT_ROWS,
  packageNotice,
  packageNoticeParts,
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
  type HeldWithinRunSelection,
  type WithinRunView
} from "./withinRunComparison";
import { ComparisonPackageNotice } from "./ComparisonPackageNotice";

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

it("orders unlisted bases by code point, not UTF-16 code unit", () => {
  const supplementary = "load:\u{10000}", privateUse = "load:\u{E000}";
  // UTF-16 order would put the surrogate pair (0xD800) first.
  expect(supplementary < privateUse).toBe(true);
  expect(codePointCompare(privateUse, supplementary)).toBe(-1);
  expect(codePointCompare(supplementary, privateUse)).toBe(1);
  expect(codePointCompare("load:a", "load:a")).toBe(0);
  expect(codePointCompare("load:a", "load:ab")).toBe(-1);
  const source = result();
  const base = source.results[0];
  source.results.push(
    { ...base, id: "invented:supplementary", basis_ref: { ref_type: "load_case", ref_id: supplementary } },
    { ...base, id: "invented:private-use", basis_ref: { ref_type: "load_case", ref_id: privateUse } }
  );
  expect(withinRunBasisOptions(source, model()).loadCases.map((option) => option.ref_id)).toEqual(["load:L-100", "load:L-200", privateUse, supplementary]);
});

describe("the left basis governs the pairing (Codex r1 on #1619)", () => {
  it("compares a chosen L-200 against L-200's own rows, never L-100's", () => {
    const source = result(), run = analysisRun("run:invented");
    const comparison = buildPreviewComparison({ result: source, analysisRun: run, leftBasisRef: "load:L-200", rightBasisRef: RANGE });
    const byId = new Map(source.results.map((row) => [row.id, row]));
    expect(comparison.left.basis_ref.ref).toBe("load:L-200");
    expect(comparison.result_deltas.length).toBeGreaterThan(0);
    for (const delta of comparison.result_deltas) {
      const left = byId.get(delta.left_result_id)!, right = byId.get(delta.right_result_id)!;
      expect(left.basis_ref?.ref_id).toBe("load:L-200");
      expect(right.source_result_refs).toContain(left.id);
      expect(delta.raw_delta).toBe(right.value - left.value);
    }
    // The case Codex reproduced: the old lookup took the L-100 source row and gave the L-100 delta
    // (305.419308 N*m). W-15 (DEC-045) forms valve C-130's member pipe:P-130 with its stiffness scale,
    // so these values moved with the capture of this model (were -304.6290472562317, 236.26369667435384,
    // 118.13184833717692 and L-100's -609.2580945124633).
    const endJ = comparison.result_deltas.find((delta) => delta.right_result_id === "result:combination:combination-U4B-RANGE-MAX:moment:pipe-P-100:bending-z:end-j")!;
    expect(endJ.left_result_id).toBe("result:loadcase:load-L-200:moment:pipe-P-100:bending-z:end-j");
    expect([endJ.left_value, endJ.right_value, endJ.raw_delta, endJ.unit]).toEqual([-305.41930776611434, -305.41930776611434, 0, "N*m"]);
    const bendingZ = comparison.result_deltas.find((delta) => delta.right_result_id === "result:combination:combination-U4B-RANGE-MAX:moment:pipe-P-100:bending-z")!;
    expect(bendingZ.left_result_id).toBe("result:loadcase:load-L-200:moment:pipe-P-100:bending-z");
    expect(bendingZ.raw_delta).toBe(226.7444902621995 - 113.37224513109975);
    // The L-100 side of the same row pairs with L-100's own row.
    const fromL100 = buildPreviewComparison({ result: source, analysisRun: run, leftBasisRef: "load:L-100", rightBasisRef: RANGE });
    const endJ100 = fromL100.result_deltas.find((delta) => delta.right_result_id === endJ.right_result_id)!;
    expect(endJ100.left_result_id).toBe("result:moment:pipe-P-100:bending-z:end-j");
    expect(endJ100.raw_delta).toBe(-305.41930776611434 - -610.8386155322287);
  });

  it("with no L-100 rows, the fixed pair gives 0 pairs and every C-OPER-ALT row target-only, never paired with another case", () => {
    const source = result();
    source.results = source.results.filter((row) => row.basis_ref?.ref_id !== "load:L-100");
    for (const row of source.results) if (row.basis_ref?.ref_id === RANGE) row.basis_ref = { ...row.basis_ref, ref_id: DEFAULT_COMPARISON_RIGHT_BASIS_REF };
    const rightRows = source.results.filter((row) => row.basis_ref?.ref_id === DEFAULT_COMPARISON_RIGHT_BASIS_REF);
    expect(rightRows.some((row) => row.source_result_refs?.some((ref) => ref.includes("load-L-200")))).toBe(true);
    const comparison = buildPreviewComparison({ result: source, analysisRun: analysisRun("run:invented") });
    expect(comparison.left.basis_ref.ref).toBe(DEFAULT_COMPARISON_LEFT_BASIS_REF);
    expect(comparison.left.result_count).toBe(0);
    expect(comparison.summary.comparable_result_pairs).toBe(0);
    expect(comparison.result_deltas).toEqual([]);
    expect(comparison.summary.unmatched_right_results).toBe(rightRows.length);
    expect(comparison.unit_policy_evidence.matched_result_units).toEqual([]);
  });
});

/** The session's resolution for one Current run (options from its solved model). */
function resolve(source: MechanicsResult | null, run: AnalysisRunEnvelope | null, held: HeldWithinRunSelection | null = null, withFixedPair = false) {
  const fixedPair = withFixedPair ? fixedPairComparison(source, run) : null;
  const resolved = resolveWithinRun({ result: source, analysisRun: run, options: source && run ? withinRunBasisOptions(source, model()) : null, held, fixedPair });
  return { ...resolved, fixedPair, packaged: packagedComparison(resolved, fixedPair ?? fixedPairComparison(source, run)) };
}

describe("the default", () => {
  it("reproduces today's comparison when the fixed load case and combination are both offered", () => {
    const source = withFixedCombination(), run = analysisRun("run:invented");
    const options = withinRunBasisOptions(source, model());
    expect(defaultWithinRunPair(options)).toEqual({ left: DEFAULT_COMPARISON_LEFT_BASIS_REF, right: DEFAULT_COMPARISON_RIGHT_BASIS_REF });
    const today = buildPreviewComparison({ result: source, analysisRun: run });
    const { view, build, fixedPair, packaged } = resolve(source, run, null, true);
    expect(view.status === "ready" && view.source).toBe("default");
    expect(view.status === "ready" && view.comparison).toStrictEqual(today);
    expect(today.summary.comparable_result_pairs).toBeGreaterThan(0);
    // The session's fixed-pair build is reused as it is, and the packages carry it.
    expect(build).toBe(fixedPair);
    expect(packaged).toBe(fixedPair);
    expect(JSON.stringify(packaged!.comparison)).toBe(JSON.stringify(today));
  });

  it("is the first offered case and combination when the fixed pair is not offered", () => {
    const source = result(), run = analysisRun("run:invented");
    const { view, packaged } = resolve(source, run);
    expect(view.status).toBe("ready");
    if (view.status !== "ready") return;
    expect(view.pair).toEqual({ left: "load:L-100", right: RANGE });
    expect(view.source).toBe("default");
    const expected = buildPreviewComparison({ result: source, analysisRun: run, leftBasisRef: "load:L-100", rightBasisRef: RANGE });
    expect(view.comparison).toStrictEqual(expected);
    expect(expected.summary.comparable_result_pairs).toBeGreaterThan(0);
    // Without a choice the packages carry the fixed pair's: C-OPER-ALT has no rows, so its
    // unavailable record, not an empty comparison.
    expect(packaged!.comparison).toBeNull();
    expect(packaged!.unavailable).toMatchObject({ left_basis_ref: DEFAULT_COMPARISON_LEFT_BASIS_REF, right_basis_ref: DEFAULT_COMPARISON_RIGHT_BASIS_REF, reason_code: COMPARISON_BASIS_WITHOUT_ROWS });
    // C-OPER-ALT is withheld by the solve, so it has no rows: the notice says no comparison is carried.
    expect(packageNotice(packaged)).toBe("The packages carry no comparison of load case L-100 with combination C-OPER-ALT: combination C-OPER-ALT has no results in this run (withheld by the solve: the model has nonlinear supports, so superposing case states is not a solved state).");
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
      const { view } = resolve(source, run);
      return view.status === "unavailable" ? view.reason : view.status;
    });
    expect(reasons).toEqual([NO_COMBINATION_ROWS, NO_LOAD_CASE_ROWS, NO_LOAD_CASE_OR_COMBINATION_ROWS]);
    expect(resolve(null, run).view).toEqual({ status: "no_result" });
    expect(resolve(result(), null).view).toEqual({ status: "no_result" });
  });
});

describe("the choice", () => {
  it("applies while the run offers both bases, and the packages then carry it", () => {
    const source = result(), run = analysisRun("run:invented");
    const { view, build, packaged } = resolve(source, run, { left: "load:L-200", right: RANGE });
    expect(view.status === "ready" && [view.source, view.pair]).toEqual(["selected", { left: "load:L-200", right: RANGE }]);
    expect(packaged).toBe(build);
    const comparison = packaged!.comparison!;
    expect([comparison.left.basis_ref.ref, comparison.right.basis_ref.ref]).toEqual(["load:L-200", RANGE]);
    expect(comparison.summary.tolerance_status).toBe("not_tolerance_checked");
    expect(comparison.summary.tolerance_profile_ref).toBe("TBD");
    expect(comparison.unit_policy_evidence.conversion_performed).toBe(false);
    expect(packageNotice(packaged)).toBe("The packages include the comparison of load case L-200 with combination U4B-RANGE-MAX.");
  });

  it("names the bases a run does not offer; such an id is never compared", () => {
    const options = withinRunBasisOptions(result(), model());
    expect(droppedBases({ left: "load:L-200", right: RANGE }, options)).toEqual([]);
    expect(droppedBases({ left: "load:L-999", right: RANGE }, options)).toEqual(["load:L-999"]);
    expect(droppedBases({ left: "load:L-999", right: DEFAULT_COMPARISON_RIGHT_BASIS_REF }, options)).toEqual(["load:L-999", DEFAULT_COMPARISON_RIGHT_BASIS_REF]);
    // A load case offered as the combination (or the reverse) is not offered.
    expect(droppedBases({ left: RANGE, right: "load:L-100" }, options)).toEqual([RANGE, "load:L-100"]);
    expect(droppedNotice(["load:L-999"])).toBe("The choice was dropped because a Current run had no result rows for load:L-999. The default pair applies until you choose again.");
    expect(droppedNotice(["load:A", "combination:B"])).toContain("no result rows for load:A and combination:B.");
    // Even if handed a stale choice, the resolution compares the default, not the stale id.
    const stale = resolve(result(), analysisRun("run:invented"), { left: "load:L-999", right: RANGE });
    expect(stale.view.status === "ready" && [stale.view.source, stale.view.pair.left]).toEqual(["default", "load:L-100"]);
  });

  it("puts a structured unavailable record in the packages when the builder refuses the choice", () => {
    // The producer's own range-envelope rows: the unchanged builder refuses their
    // undeclared kinds (the known gap routed to T6).
    const source = producerResult(), run = analysisRun("run:invented");
    const { view, packaged } = resolve(source, run, { left: "load:L-200", right: RANGE });
    expect(view.status === "ready" && view.comparison).toBeNull();
    expect(view.status === "ready" && view.refusal).toContain("ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED");
    expect(packaged!.comparison).toBeNull();
    expect(packaged!.unavailable).toEqual({
      document_kind: "openpipestress.technical_preview.comparison_unavailable",
      comparison_id: previewComparisonId(source.run_id, "load:L-200", RANGE),
      comparison_kind: "single_run_load_basis_review",
      status: "unavailable",
      left_basis_ref: "load:L-200",
      right_basis_ref: RANGE,
      analysis_run_ref: { object_type: "AnalysisRun", ref: "run:invented" },
      reason_code: "ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED",
      reason: expect.stringContaining("has no source dimension declaration"),
      release_or_professional_claim: false
    });
    expect(packageNotice(packaged)).toMatch(/^Packages omit the comparison of load case L-200 vs combination U4B-RANGE-MAX: ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED/);
    // The fixed pair: unavailable here, where the solve withheld C-OPER-ALT (no rows), not
    // refused by the builder; with rows for both bases it is built and nothing is unavailable.
    expect(buildWithinRunComparison(source, run, { left: "load:L-100", right: DEFAULT_COMPARISON_RIGHT_BASIS_REF }).unavailable).toMatchObject({ reason_code: COMPARISON_BASIS_WITHOUT_ROWS });
    expect(buildWithinRunComparison(withFixedCombination(), run, { left: "load:L-100", right: DEFAULT_COMPARISON_RIGHT_BASIS_REF }).unavailable).toBeNull();
  });
});

describe("the comparison id", () => {
  const FIXED_ID_SUFFIX = ":load-L-100-to-combination-C-OPER-ALT";
  it("keeps the fixed pair's pre-S3c id, byte for byte", () => {
    const today = buildPreviewComparison({ result: withFixedCombination(), analysisRun: analysisRun("run:invented") });
    expect(today.comparison_id).toBe(`comparison:run:preview-linear-static-001${FIXED_ID_SUFFIX}`);
    expect(previewComparisonBases(today.comparison_id)).toEqual({ run: "run:preview-linear-static-001", left: DEFAULT_COMPARISON_LEFT_BASIS_REF, right: DEFAULT_COMPARISON_RIGHT_BASIS_REF });
    expect(previewComparisonId("run:with:colons", DEFAULT_COMPARISON_LEFT_BASIS_REF, DEFAULT_COMPARISON_RIGHT_BASIS_REF)).toBe(`comparison:run:with:colons${FIXED_ID_SUFFIX}`);
  });

  it("is never read as the fixed pair for another pair, and never equals the fixed pair's id (Codex r2 on #1619)", () => {
    // A legal combination id that is the legacy suffix's text.
    const pairId = previewComparisonId("run:invented", "load:L-200", "load-L-100-to-combination-C-OPER-ALT");
    expect(previewComparisonBases(pairId)).toEqual({ run: "run:invented", left: "load:L-200", right: "load-L-100-to-combination-C-OPER-ALT" });
    // The fixed pair with a run that spells the earlier pair form's middle.
    const fixedId = previewComparisonId("run:invented:load~3a.L-200:to", DEFAULT_COMPARISON_LEFT_BASIS_REF, DEFAULT_COMPARISON_RIGHT_BASIS_REF);
    expect(pairId).not.toBe(fixedId);
    expect(previewComparisonBases(fixedId)).toEqual({ run: "run:invented:load~3a.L-200:to", left: DEFAULT_COMPARISON_LEFT_BASIS_REF, right: DEFAULT_COMPARISON_RIGHT_BASIS_REF });
    expect(pairId).toBe("comparison-pair:run~3a.invented:load~3a.L-200:load-L-100-to-combination-C-OPER-ALT");
    // Non-canonical or foreign ids read as nothing.
    for (const id of ["comparison-pair:a:b", "comparison-pair:a:b:c:d", "comparison-pair:~41.:b:c", "comparison-pair:~110000.:b:c", "comparison-pair:a:b:c~", "other:a:b:c"]) {
      expect(previewComparisonBases(id)).toBeNull();
    }
  });

  it("round-trips random runs and bases, so distinct triples give distinct ids", () => {
    // A deterministic generator over the characters that matter: the escape and
    // separator characters, the legacy suffix's own text, surrogates and astral code points.
    let seed = 0x5eed;
    const next = () => { seed = (seed * 1103515245 + 12345) & 0x7fffffff; return seed; };
    const atoms = [":", "~", ".", "-", "_", "a", "Z", "0", "to", "load", "L-100", "-to-", "combination", "C-OPER-ALT", "\u{10000}", "\u{E000}", "\uD800", "\uDC00", "é", "%", " ", "comparison", "comparison-pair"];
    const randomText = () => Array.from({ length: next() % 7 }, () => atoms[next() % atoms.length]).join("");
    const seen = new Map<string, string>();
    for (let index = 0; index < 2000; index += 1) {
      const run = randomText(), left = index % 50 === 0 ? DEFAULT_COMPARISON_LEFT_BASIS_REF : randomText(), right = index % 50 === 0 ? DEFAULT_COMPARISON_RIGHT_BASIS_REF : randomText();
      const id = previewComparisonId(run, left, right);
      expect(previewComparisonBases(id)).toEqual({ run, left, right });
      const triple = JSON.stringify([run, left, right]);
      if (seen.has(id)) expect(seen.get(id)).toBe(triple);
      seen.set(id, triple);
    }
    expect(seen.size).toBeGreaterThan(1500);
    expect(escapeComparisonBasis("a:b")).not.toContain(":");
  });
});

it("states why the choice is disabled while the session is busy", () => {
  expect(withinRunBusyReason({ projectBusy: false, running: false, reportPackageBusy: false })).toBeNull();
  expect(withinRunBusyReason({ projectBusy: true, running: true, reportPackageBusy: true })).toBe(WITHIN_RUN_BUSY_PROJECT);
  expect(withinRunBusyReason({ projectBusy: false, running: true, reportPackageBusy: true })).toBe(WITHIN_RUN_BUSY_SOLVE);
  expect(withinRunBusyReason({ projectBusy: false, running: false, reportPackageBusy: true })).toBe(WITHIN_RUN_BUSY_REPORT);
});

describe("the panel", () => {
  const panel = (resolved: ReturnType<typeof resolve>, overrides: Partial<WithinRunControls> = {}) => {
    const onSelect = vi.fn();
    const view = resolved.view;
    const controls: WithinRunControls = { view, disabledReason: null, onSelect, packaged: resolved.packaged, dropped: null, ...overrides };
    render(<ComparisonPanel comparison={view.status === "ready" ? view.comparison : null} result={null} onSelectResult={() => {}} withinRun={controls} />);
    return { onSelect };
  };

  it("offers labelled selects of the run's bases, and a change chooses both ids", () => {
    const { onSelect } = panel(resolve(withFixedCombination(), analysisRun("run:invented"), null, true));
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
    expect(screen.getByTestId("comparison-package-basis")).toHaveTextContent("The packages include the comparison of load case L-100 with combination C-OPER-ALT.");
    fireEvent.change(left, { target: { value: "load:L-200" } });
    expect(onSelect).toHaveBeenLastCalledWith({ left: "load:L-200", right: DEFAULT_COMPARISON_RIGHT_BASIS_REF });
    // No wording of standing.
    expect(screen.getByTestId("comparison-panel").textContent).not.toMatch(/certif|accept|approv|compliant|pass\b|fail\b/i);
  });

  it("is disabled with the stated reason while busy", () => {
    panel(resolve(result(), analysisRun("run:invented")), { disabledReason: WITHIN_RUN_BUSY_SOLVE });
    for (const testId of ["comparison-left-select", "comparison-right-select"]) {
      const select = screen.getByTestId(testId) as HTMLSelectElement;
      expect(select.disabled).toBe(true);
      expect(select.title).toBe(WITHIN_RUN_BUSY_SOLVE);
      expect(select.getAttribute("aria-describedby")).toBe("comparison-within-run-disabled-reason");
    }
    expect(screen.getByTestId("comparison-within-run-disabled-reason")).toHaveTextContent(WITHIN_RUN_BUSY_SOLVE);
  });

  it("says plainly that there is nothing to compare, and still names what the packages carry", () => {
    const source = result();
    source.results = source.results.filter((row) => row.basis_ref?.ref_type !== "combination");
    panel(resolve(source, analysisRun("run:invented")));
    expect(screen.getByTestId("comparison-unavailable")).toHaveTextContent(NO_COMBINATION_ROWS);
    expect(screen.queryByTestId("comparison-summary")).toBeNull();
    expect(screen.queryByTestId("comparison-left-select")).toBeNull();
    expect(screen.getByTestId("comparison-package-basis")).toHaveTextContent("The packages carry no comparison of load case L-100 with combination C-OPER-ALT: combination C-OPER-ALT has no results in this run (withheld by the solve: the model has nonlinear supports, so superposing case states is not a solved state).");
  });

  it("names the builder's refusal and says the packages omit the comparison, though no comparison id is shown (Codex finding 3)", () => {
    panel(resolve(producerResult(), analysisRun("run:invented"), { left: "load:L-200", right: RANGE }));
    expect(screen.getByTestId("comparison-refused")).toHaveTextContent("Not compared: ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED");
    expect(screen.queryByTestId("comparison-summary")).toBeNull();
    expect(screen.getByTestId("comparison-package-basis")).toHaveTextContent("Packages omit the comparison of load case L-200 vs combination U4B-RANGE-MAX: ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED");
  });

  it("names a dropped choice", () => {
    panel(resolve(result(), analysisRun("run:invented")), { dropped: ["load:L-999"] });
    expect(screen.getByTestId("comparison-package-basis-dropped")).toHaveTextContent(droppedNotice(["load:L-999"]));
  });

  it("the package notice beside the package actions reads the same", () => {
    const resolved = resolve(result(), analysisRun("run:invented"), { left: "load:L-200", right: RANGE });
    render(<ComparisonPackageNotice packaged={resolved.packaged} testId="report-comparison-package-notice" />);
    expect(screen.getByTestId("report-comparison-package-notice")).toHaveTextContent("The packages include the comparison of load case L-200 with combination U4B-RANGE-MAX.");
    const { container } = render(<ComparisonPackageNotice packaged={null} testId="none" />);
    expect(container.textContent).toBe("");
  });

  it("without the session's controls, shows the given comparison as before", () => {
    const today = buildPreviewComparison({ result: withFixedCombination(), analysisRun: analysisRun("run:invented") });
    const rendered = render(<ComparisonPanel comparison={today} result={null} onSelectResult={() => {}} />);
    expect(within(rendered.container).queryByTestId("comparison-within-run")).toBeNull();
    expect(within(rendered.container).getByTestId("comparison-summary")).toBeTruthy();
    rendered.unmount();
    render(<ComparisonPanel comparison={null} result={null} onSelectResult={() => {}} withinRun={{ view: { status: "no_result" }, disabledReason: null, onSelect: () => {}, packaged: null, dropped: null }} />);
    expect(screen.getByTestId("comparison-empty")).toBeTruthy();
    expect(screen.queryByTestId("comparison-within-run")).toBeNull();
  });
});

describe("the package notice", () => {
  const run = analysisRun("run:invented");
  it("names a basis by its kind and its id without the stored prefix", () => {
    expect(basisName("left", "load:L-100")).toBe("load case L-100");
    expect(basisName("right", "combination:C-OPER-ALT")).toBe("combination C-OPER-ALT");
    // An id without the prefix, or only the prefix, is shown as it is.
    expect(basisName("left", "case:six")).toBe("load case case:six");
    expect(basisName("right", "combination:")).toBe("combination combination:");
  });

  it("compared: both bases have rows; the members and the sentence", () => {
    const parts = packageNoticeParts(buildWithinRunComparison(result(), run, { left: "load:L-200", right: RANGE }))!;
    expect(parts).toEqual({
      left: "load:L-200", right: RANGE, leftName: "load case L-200", rightName: "combination U4B-RANGE-MAX",
      state: "compared", withoutRows: [], reason: null,
      text: "The packages include the comparison of load case L-200 with combination U4B-RANGE-MAX."
    });
  });

  it("none carried: a side without rows, with the solve's reason only where the solve withheld it", () => {
    // C-OPER-ALT is withheld by the solve (its gate and diagnostic).
    const withheld = packageNoticeParts(buildWithinRunComparison(result(), run, { left: "load:L-100", right: DEFAULT_COMPARISON_RIGHT_BASIS_REF }))!;
    expect([withheld.state, withheld.withoutRows.map((basis) => [basis.side, basis.ref_id, basis.withheld])]).toEqual(["none_carried", [["right", DEFAULT_COMPARISON_RIGHT_BASIS_REF, true]]]);
    expect(withheld.reason).toBe("withheld by the solve: the model has nonlinear supports, so superposing case states is not a solved state");
    // A gate with no diagnostic message: withheld, with no text of its own.
    const silent = result();
    silent.diagnostics = silent.diagnostics.filter((item) => item.code !== "NONLINEAR_COMBINATION_REQUIRES_SOLVE");
    expect(packageNoticeParts(buildWithinRunComparison(silent, run, { left: "load:L-100", right: DEFAULT_COMPARISON_RIGHT_BASIS_REF }))!.text)
      .toBe("The packages carry no comparison of load case L-100 with combination C-OPER-ALT: combination C-OPER-ALT has no results in this run (withheld by the solve).");
    // A basis the run has no rows for and the solve did not withhold: no reason.
    const missing = packageNoticeParts(buildWithinRunComparison(result(), run, { left: "load:L-999", right: RANGE }))!;
    expect([missing.state, missing.reason, missing.text]).toEqual([
      "none_carried", null,
      "The packages carry no comparison of load case L-999 with combination U4B-RANGE-MAX: load case L-999 has no results in this run."
    ]);
    // Both sides: both named; the reason names the side it belongs to.
    const both = packageNoticeParts(buildWithinRunComparison(result(), run, { left: "load:L-999", right: DEFAULT_COMPARISON_RIGHT_BASIS_REF }))!;
    expect(both.withoutRows.map((basis) => basis.side)).toEqual(["left", "right"]);
    expect(both.text).toBe("The packages carry no comparison of load case L-999 with combination C-OPER-ALT: load case L-999 and combination C-OPER-ALT have no results in this run (combination C-OPER-ALT withheld by the solve: the model has nonlinear supports, so superposing case states is not a solved state).");
    const neither = packageNoticeParts(buildWithinRunComparison(result(), run, { left: "load:L-999", right: "combination:C-999" }))!;
    expect([neither.reason, neither.text]).toEqual([null, "The packages carry no comparison of load case L-999 with combination C-999: load case L-999 and combination C-999 have no results in this run."]);
  });

  it("omitted: the builder refused the pair; today's sentence with the bases named", () => {
    const parts = packageNoticeParts(buildWithinRunComparison(producerResult(), run, { left: "load:L-200", right: RANGE }))!;
    expect([parts.state, parts.withoutRows]).toEqual(["omitted", []]);
    expect(parts.reason).toMatch(/^ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED/);
    expect(parts.text).toBe(`Packages omit the comparison of load case L-200 vs combination U4B-RANGE-MAX: ${parts.reason}`);
  });

  it("is null without a Current run", () => {
    expect(packageNoticeParts(null)).toBeNull();
    expect(packageNotice(null)).toBeNull();
  });
});

describe("the fixed pair without a choice (Agent 0's absent-pair rule)", () => {
  const run = analysisRun("run:invented");
  const WITHHELD = "Combination withheld: the model has nonlinear supports, so superposing case states is not a solved state.";
  /** The range-envelope result with no row, gate or diagnostic of either fixed basis. */
  function withoutFixedBases(): MechanicsResult {
    const source = result();
    source.results = source.results.filter((row) => row.basis_ref?.ref_id !== DEFAULT_COMPARISON_LEFT_BASIS_REF && row.basis_ref?.ref_id !== DEFAULT_COMPARISON_RIGHT_BASIS_REF);
    const evidence = source.contract_evidence as { combination_gates?: Array<{ combination_id: string }> };
    evidence.combination_gates = (evidence.combination_gates ?? []).filter((gate) => gate.combination_id !== DEFAULT_COMPARISON_RIGHT_BASIS_REF);
    source.diagnostics = source.diagnostics.filter((item) => !item.affected_refs?.includes(DEFAULT_COMPARISON_RIGHT_BASIS_REF));
    return source;
  }
  function modelWithout(): PreviewModel {
    const solved = model();
    solved.load_cases = solved.load_cases.filter((entry) => entry.id !== DEFAULT_COMPARISON_LEFT_BASIS_REF);
    solved.combinations = solved.combinations!.filter((entry) => entry.id !== DEFAULT_COMPARISON_RIGHT_BASIS_REF);
    return solved;
  }

  it("is nothing, with no notice, when neither fixed basis is in the model, has rows or was withheld", () => {
    expect(fixedPairComparison(withoutFixedBases(), run, modelWithout())).toBeNull();
    expect(fixedPairComparison(withoutFixedBases(), run, null)).toBeNull();
    expect(packageNotice(null)).toBeNull();
    // An explicit pair is unchanged: it still carries the record when a basis has no rows.
    expect(buildWithinRunComparison(withoutFixedBases(), run, { left: DEFAULT_COMPARISON_LEFT_BASIS_REF, right: DEFAULT_COMPARISON_RIGHT_BASIS_REF }).unavailable)
      .toMatchObject({ reason_code: COMPARISON_BASIS_WITHOUT_ROWS });
  });

  it("is the unavailable record when a fixed basis is only in the model (no rows, no withheld gate)", () => {
    const solved = modelWithout();
    solved.load_cases = [...solved.load_cases, { ...model().load_cases.find((entry) => entry.id === DEFAULT_COMPARISON_LEFT_BASIS_REF)! }];
    const build = fixedPairComparison(withoutFixedBases(), run, solved)!;
    expect(build.comparison).toBeNull();
    expect(build.unavailable).toMatchObject({
      left_basis_ref: DEFAULT_COMPARISON_LEFT_BASIS_REF,
      right_basis_ref: DEFAULT_COMPARISON_RIGHT_BASIS_REF,
      reason_code: COMPARISON_BASIS_WITHOUT_ROWS,
      reason: `${COMPARISON_BASIS_WITHOUT_ROWS}: load:L-100 and combination:C-OPER-ALT have no result rows in this run, so the pair is not compared.`
    });
    // The combination only in the model: the same.
    const withCombination = modelWithout();
    withCombination.combinations = [...withCombination.combinations!, { ...model().combinations!.find((entry) => entry.id === DEFAULT_COMPARISON_RIGHT_BASIS_REF)! }];
    expect(fixedPairComparison(withoutFixedBases(), run, withCombination)!.unavailable).toMatchObject({ reason_code: COMPARISON_BASIS_WITHOUT_ROWS });
  });

  it("is the unavailable record with the solve's withholding text when the solve withheld a fixed basis", () => {
    // The range-envelope run: L-100 has rows and the solve withheld C-OPER-ALT.
    const build = fixedPairComparison(result(), run, model())!;
    expect(build.comparison).toBeNull();
    expect(build.unavailable).toMatchObject({
      reason_code: COMPARISON_BASIS_WITHOUT_ROWS,
      reason: `${COMPARISON_BASIS_WITHOUT_ROWS}: combination:C-OPER-ALT has no result rows in this run, so the pair is not compared. combination:C-OPER-ALT: ${WITHHELD}`
    });
    // Withheld only (no row of either basis, not in the model): still the record.
    const withheldOnly = withoutFixedBases();
    (withheldOnly.contract_evidence as { combination_gates: unknown[] }).combination_gates.push({ combination_id: DEFAULT_COMPARISON_RIGHT_BASIS_REF, reason: "NONLINEAR_COMBINATION_REQUIRES_SOLVE", withheld: true });
    expect(fixedPairComparison(withheldOnly, run, modelWithout())!.unavailable).toMatchObject({ reason_code: COMPARISON_BASIS_WITHOUT_ROWS });
  });
});

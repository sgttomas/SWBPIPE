// "Within this run" (W-12 S3c; DEL-14-01 Design `desktop-states-runs-comparison.md`
// §4.3): the user chooses the load case (left) and the combination (right) that
// `buildPreviewComparison` compares, instead of its fixed ids. The comparison
// itself is unchanged: pairing through `source_result_refs` with equal explicit
// units, nothing converted, `not_tolerance_checked`, tolerance profile `TBD`.
//
// - **Options** are the bases that have result rows in the current run: each
//   distinct `basis_ref.ref_id` with `ref_type` `load_case` (left) or
//   `combination` (right), in the run's solved-model order, then any the model
//   does not list in code-point order.
// - **Default.** The fixed pair when both are offered (today's view), else the
//   first offered case and combination; with no case or no combination, nothing,
//   with the reason.
// - **Held with its run.** A choice is held with the basis key of the run it was
//   made on (the session's result and analysis-run sequences). Any other run reads
//   the default in the very render that first shows it, and a held id that is not
//   offered by the current run is never compared (the stale-id guard).
import {
  buildPreviewComparison,
  DEFAULT_COMPARISON_LEFT_BASIS_REF,
  DEFAULT_COMPARISON_RIGHT_BASIS_REF
} from "../../services/previewService";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewComparison, PreviewModel } from "../../types";

export type WithinRunBasisOption = { ref_id: string; label: string | null; row_count: number };
export type WithinRunOptions = { loadCases: WithinRunBasisOption[]; combinations: WithinRunBasisOption[] };
export type WithinRunPair = { left: string; right: string };
/** The user's choice, held with the basis key of the run it was made on. */
export type HeldWithinRunSelection = { basis: string; left: string; right: string };

export type WithinRunView =
  | { status: "no_result" }
  | { status: "unavailable"; options: WithinRunOptions; reason: string }
  | {
      status: "ready";
      options: WithinRunOptions;
      pair: WithinRunPair;
      source: "default" | "selected";
      /** The comparison of `pair`, or null when the builder refused it (`refusal`). */
      comparison: PreviewComparison | null;
      refusal: string | null;
    };

export const NO_LOAD_CASE_OR_COMBINATION_ROWS =
  "No load case and no combination in this run has result rows, so there is nothing to compare within this run.";
export const NO_LOAD_CASE_ROWS = "No load case in this run has result rows, so there is nothing to compare within this run.";
export const NO_COMBINATION_ROWS = "No combination in this run has result rows, so there is nothing to compare within this run.";

function codePointCompare(left: string, right: string): number {
  return left < right ? -1 : left > right ? 1 : 0;
}

function orderedOptions(counts: Map<string, number>, listed: Array<{ id: string; label?: string }> | undefined): WithinRunBasisOption[] {
  const options: WithinRunBasisOption[] = [];
  const seen = new Set<string>();
  for (const entry of listed ?? []) {
    const count = counts.get(entry.id);
    if (count === undefined || seen.has(entry.id)) continue;
    seen.add(entry.id);
    options.push({ ref_id: entry.id, label: typeof entry.label === "string" && entry.label ? entry.label : null, row_count: count });
  }
  for (const id of [...counts.keys()].filter((id) => !seen.has(id)).sort(codePointCompare)) {
    options.push({ ref_id: id, label: null, row_count: counts.get(id)! });
  }
  return options;
}

/** The load cases and combinations that have result rows in `result`, ordered by
 * the run's solved model (`model`, its input manifest's snapshot). */
export function withinRunBasisOptions(result: MechanicsResult, model: PreviewModel | null | undefined): WithinRunOptions {
  const loadCaseCounts = new Map<string, number>();
  const combinationCounts = new Map<string, number>();
  for (const row of result.results) {
    const basis = row.basis_ref;
    if (!basis || typeof basis.ref_id !== "string" || basis.ref_id === "") continue;
    const counts = basis.ref_type === "load_case" ? loadCaseCounts : basis.ref_type === "combination" ? combinationCounts : null;
    if (counts) counts.set(basis.ref_id, (counts.get(basis.ref_id) ?? 0) + 1);
  }
  return {
    loadCases: orderedOptions(loadCaseCounts, Array.isArray(model?.load_cases) ? model.load_cases : undefined),
    combinations: orderedOptions(combinationCounts, Array.isArray(model?.combinations) ? model.combinations : undefined)
  };
}

/** Today's fixed pair when both are offered, else the first offered of each; null
 * with no case or no combination. */
export function defaultWithinRunPair(options: WithinRunOptions): WithinRunPair | null {
  const [firstCase] = options.loadCases;
  const [firstCombination] = options.combinations;
  if (!firstCase || !firstCombination) return null;
  const offersDefault =
    options.loadCases.some((option) => option.ref_id === DEFAULT_COMPARISON_LEFT_BASIS_REF) &&
    options.combinations.some((option) => option.ref_id === DEFAULT_COMPARISON_RIGHT_BASIS_REF);
  return offersDefault
    ? { left: DEFAULT_COMPARISON_LEFT_BASIS_REF, right: DEFAULT_COMPARISON_RIGHT_BASIS_REF }
    : { left: firstCase.ref_id, right: firstCombination.ref_id };
}

function unavailableReason(options: WithinRunOptions): string {
  if (options.loadCases.length === 0 && options.combinations.length === 0) return NO_LOAD_CASE_OR_COMBINATION_ROWS;
  return options.loadCases.length === 0 ? NO_LOAD_CASE_ROWS : NO_COMBINATION_ROWS;
}

/** The held choice when it was made on this run (`basis`) and both of its ids are
 * offered by this run; otherwise null, and the default applies. */
export function heldPairForRun(held: HeldWithinRunSelection | null, basis: string, options: WithinRunOptions): WithinRunPair | null {
  if (!held || held.basis !== basis) return null;
  const offered =
    options.loadCases.some((option) => option.ref_id === held.left) &&
    options.combinations.some((option) => option.ref_id === held.right);
  return offered ? { left: held.left, right: held.right } : null;
}

/** "Within this run" for the session's current result: its options, the pair it
 * compares and that pair's comparison. `defaultComparison`, when given, is the
 * session's comparison of the fixed pair, reused when the pair is the fixed one. */
export function resolveWithinRun({
  result,
  analysisRun,
  model,
  held,
  basis,
  defaultComparison = null
}: {
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  model: PreviewModel | null | undefined;
  held: HeldWithinRunSelection | null;
  basis: string;
  defaultComparison?: PreviewComparison | null;
}): WithinRunView {
  if (!result || !analysisRun) return { status: "no_result" };
  const options = withinRunBasisOptions(result, model);
  const selected = heldPairForRun(held, basis, options);
  const pair = selected ?? defaultWithinRunPair(options);
  if (!pair) return { status: "unavailable", options, reason: unavailableReason(options) };
  const source = selected ? "selected" : "default";
  if (defaultComparison && pair.left === DEFAULT_COMPARISON_LEFT_BASIS_REF && pair.right === DEFAULT_COMPARISON_RIGHT_BASIS_REF) {
    return { status: "ready", options, pair, source, comparison: defaultComparison, refusal: null };
  }
  try {
    const comparison = buildPreviewComparison({ result, analysisRun, leftBasisRef: pair.left, rightBasisRef: pair.right });
    return { status: "ready", options, pair, source, comparison, refusal: null };
  } catch (error) {
    return { status: "ready", options, pair, source, comparison: null, refusal: error instanceof Error ? error.message : String(error) };
  }
}

/** The comparison the report, handoff and export packages carry: the user's
 * choice for this run when one is held, else the fixed-pair comparison exactly as
 * before S3c (so a document built without a choice is unchanged). */
export function packagedComparison(view: WithinRunView, defaultComparison: PreviewComparison | null): PreviewComparison | null {
  return view.status === "ready" && view.source === "selected" ? view.comparison : defaultComparison;
}

/** The stated reasons the choice is disabled (and refused) while the session is busy. */
export const WITHIN_RUN_BUSY_PROJECT = "Another project operation or prompt is in progress.";
export const WITHIN_RUN_BUSY_SOLVE = "A solve is in progress; its run will replace this one.";
export const WITHIN_RUN_BUSY_REPORT = "A report package is being prepared from this comparison.";

export function withinRunBusyReason({ projectBusy, running, reportPackageBusy }: { projectBusy: boolean; running: boolean; reportPackageBusy: boolean }): string | null {
  if (projectBusy) return WITHIN_RUN_BUSY_PROJECT;
  if (running) return WITHIN_RUN_BUSY_SOLVE;
  if (reportPackageBusy) return WITHIN_RUN_BUSY_REPORT;
  return null;
}

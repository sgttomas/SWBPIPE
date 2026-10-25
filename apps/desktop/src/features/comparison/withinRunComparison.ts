// "Within this run" (W-12 S3c; DEL-14-01 Design `desktop-states-runs-comparison.md`
// §4.3): the user chooses the load case (left) and the combination (right) that
// `buildPreviewComparison` compares, instead of its fixed ids. The comparison
// itself is unchanged: pairing through `source_result_refs` of the left basis with
// equal explicit units, nothing converted, `not_tolerance_checked`, tolerance
// profile `TBD`.
//
// - **Options** are the bases that have result rows in the current run: each
//   distinct `basis_ref.ref_id` with `ref_type` `load_case` (left) or
//   `combination` (right), in the run's solved-model order, then any the model
//   does not list in code-point order.
// - **Default.** The fixed pair when both are offered (today's view), else the
//   first offered case and combination; with no case or no combination, nothing,
//   with the reason.
// - **The choice is revalidated, never silently carried or reset.** It stays with
//   the session across solves, edits and replacements and applies to every Current
//   run that offers both of its bases. When a Current run does not offer one, the
//   session drops the choice, reverts to the default and names the dropped bases
//   (`droppedBases`); an id the run does not offer is never compared.
// - **Packages follow the choice** (Agent 0, option A): the report, handoff and
//   export packages carry the chosen pair's comparison, or a structured unavailable
//   record when the builder refuses it; without a choice, the fixed-pair comparison
//   exactly as before S3c.
import {
  buildPreviewComparison,
  DEFAULT_COMPARISON_LEFT_BASIS_REF,
  DEFAULT_COMPARISON_RIGHT_BASIS_REF,
  previewComparisonId
} from "../../services/previewService";
import type { AnalysisRunEnvelope, MechanicsResult, ObjectRef, PreviewComparison, PreviewModel } from "../../types";

export type WithinRunBasisOption = { ref_id: string; label: string | null; row_count: number };
export type WithinRunOptions = { loadCases: WithinRunBasisOption[]; combinations: WithinRunBasisOption[] };
export type WithinRunPair = { left: string; right: string };
/** The user's choice of a load case and a combination. */
export type HeldWithinRunSelection = WithinRunPair;

/** What a package carries in place of a comparison the builder refused: the chosen
 * bases, the run and the builder's reason, by name. */
export type PreviewComparisonUnavailable = {
  document_kind: "openpipestress.technical_preview.comparison_unavailable";
  comparison_id: string;
  comparison_kind: "single_run_load_basis_review";
  status: "unavailable";
  left_basis_ref: string;
  right_basis_ref: string;
  analysis_run_ref: ObjectRef;
  reason_code: string;
  reason: string;
  release_or_professional_claim: false;
};

/** One pair's outcome: its comparison, or the record of its refusal. */
export type WithinRunComparisonBuild = {
  pair: WithinRunPair;
  comparison: PreviewComparison | null;
  unavailable: PreviewComparisonUnavailable | null;
};

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

/** Unicode scalar-value (code-point) order. JavaScript's `<` compares UTF-16 code
 * units, which puts a supplementary character (U+10000 and above, a surrogate pair
 * from 0xD800) before U+E000..U+FFFF. */
export function codePointCompare(left: string, right: string): number {
  const a = left[Symbol.iterator](), b = right[Symbol.iterator]();
  for (;;) {
    const x = a.next(), y = b.next();
    if (x.done || y.done) return x.done && y.done ? 0 : x.done ? -1 : 1;
    const difference = x.value.codePointAt(0)! - y.value.codePointAt(0)!;
    if (difference !== 0) return difference < 0 ? -1 : 1;
  }
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

/** The bases of `held` that `options` does not offer (left as a load case, right as
 * a combination), in that order; empty when the choice applies. */
export function droppedBases(held: HeldWithinRunSelection, options: WithinRunOptions): string[] {
  const dropped: string[] = [];
  if (!options.loadCases.some((option) => option.ref_id === held.left)) dropped.push(held.left);
  if (!options.combinations.some((option) => option.ref_id === held.right)) dropped.push(held.right);
  return dropped;
}

/** The comparison of one pair, or, when the unchanged builder refuses it, the record
 * of that refusal with the pair and the reason. */
export function buildWithinRunComparison(result: MechanicsResult, analysisRun: AnalysisRunEnvelope, pair: WithinRunPair): WithinRunComparisonBuild {
  try {
    return { pair, comparison: buildPreviewComparison({ result, analysisRun, leftBasisRef: pair.left, rightBasisRef: pair.right }), unavailable: null };
  } catch (error) {
    const reason = error instanceof Error ? error.message : String(error);
    const code = /^([A-Z][A-Z0-9_-]*):/.exec(reason)?.[1] ?? "COMPARISON-BUILD-REFUSED";
    return {
      pair,
      comparison: null,
      unavailable: {
        document_kind: "openpipestress.technical_preview.comparison_unavailable",
        comparison_id: previewComparisonId(result.run_id, pair.left, pair.right),
        comparison_kind: "single_run_load_basis_review",
        status: "unavailable",
        left_basis_ref: pair.left,
        right_basis_ref: pair.right,
        analysis_run_ref: { object_type: "AnalysisRun", ref: analysisRun.analysis_run.run_id },
        reason_code: code,
        reason,
        release_or_professional_claim: false
      }
    };
  }
}

const FIXED_PAIR: WithinRunPair = { left: DEFAULT_COMPARISON_LEFT_BASIS_REF, right: DEFAULT_COMPARISON_RIGHT_BASIS_REF };
const samePair = (left: WithinRunPair, right: WithinRunPair) => left.left === right.left && left.right === right.right;

/** The fixed pair's build, as the session holds it for the packages without a choice. */
export function fixedPairComparison(result: MechanicsResult | null, analysisRun: AnalysisRunEnvelope | null): WithinRunComparisonBuild | null {
  return result && analysisRun ? buildWithinRunComparison(result, analysisRun, FIXED_PAIR) : null;
}

/** "Within this run" for the session's current result: its options, the pair it
 * compares and that pair's outcome. `held` applies only when the run offers both
 * of its bases (the session drops it otherwise). `fixedPair` is the session's
 * fixed-pair build, reused when the pair is the fixed one. */
export function resolveWithinRun({
  result,
  analysisRun,
  options,
  held,
  fixedPair = null
}: {
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  options: WithinRunOptions | null;
  held: HeldWithinRunSelection | null;
  fixedPair?: WithinRunComparisonBuild | null;
}): { view: WithinRunView; build: WithinRunComparisonBuild | null } {
  if (!result || !analysisRun || !options) return { view: { status: "no_result" }, build: null };
  const selected = held && droppedBases(held, options).length === 0 ? held : null;
  const pair = selected ?? defaultWithinRunPair(options);
  if (!pair) return { view: { status: "unavailable", options, reason: unavailableReason(options) }, build: null };
  const build = fixedPair && samePair(pair, FIXED_PAIR) ? fixedPair : buildWithinRunComparison(result, analysisRun, pair);
  return {
    view: { status: "ready", options, pair, source: selected ? "selected" : "default", comparison: build.comparison, refusal: build.unavailable?.reason ?? null },
    build
  };
}

/** What the report, handoff and export packages carry: the choice's outcome when a
 * choice applies, else the fixed-pair build exactly as before S3c. */
export function packagedComparison(
  resolved: { view: WithinRunView; build: WithinRunComparisonBuild | null },
  fixedPair: WithinRunComparisonBuild | null
): WithinRunComparisonBuild | null {
  return resolved.view.status === "ready" && resolved.view.source === "selected" ? resolved.build : fixedPair;
}

/** The package notice: which comparison the packages carry, or that they omit it
 * and why. Null when there is no Current run. */
export function packageNotice(packaged: WithinRunComparisonBuild | null): string | null {
  if (!packaged) return null;
  const pair = `${packaged.pair.left} vs ${packaged.pair.right}`;
  if (packaged.comparison) return `Packages include: ${pair}.`;
  return `Packages omit the comparison of ${pair}: ${packaged.unavailable?.reason ?? "it could not be built"}`;
}

/** The notice of a choice dropped because a Current run did not offer its bases. It
 * stays until the next choice, so it speaks of that run, not the one shown. */
export function droppedNotice(dropped: string[]): string {
  return `The choice was dropped because a Current run had no result rows for ${dropped.join(" and ")}. The default pair applies until you choose again.`;
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

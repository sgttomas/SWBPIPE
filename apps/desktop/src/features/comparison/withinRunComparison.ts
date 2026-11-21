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
//   record when the builder refuses it; without a choice, the fixed pair's.
// - **A basis without rows is not compared** (Agent 0): when a basis of the pair has
//   no result rows in the run, the packages carry the unavailable record (pair, run,
//   reason, with the solve's own withholding text where the solve withheld it), not an
//   empty comparison. A pair whose two bases both have rows is built as before.
// - **An absent fixed pair is omitted** (Agent 0): without a choice, when neither
//   fixed basis is in the run's solved model, has result rows or was withheld by the
//   solve, the packages carry no fixed-pair record and there is no package notice.
import {
  buildPreviewComparison,
  type ComparedRun,
  DEFAULT_COMPARISON_LEFT_BASIS_REF,
  DEFAULT_COMPARISON_RIGHT_BASIS_REF,
  previewComparisonId
} from "../../services/previewService";
import type { AnalysisRunEnvelope, MechanicsResult, ObjectRef, PreviewComparison, PreviewModel } from "../../types";
import { previewCombinationGates } from "../results/knownSemanticLimitations";

export type WithinRunBasisOption = { ref_id: string; label: string | null; row_count: number };
export type WithinRunOptions = { loadCases: WithinRunBasisOption[]; combinations: WithinRunBasisOption[] };
export type WithinRunPair = { left: string; right: string };
/** The user's choice of a load case and a combination. */
export type HeldWithinRunSelection = WithinRunPair;

/** What a package carries in place of a comparison that is not made: the pair's
 * bases, the run and the reason by name, when a basis has no result rows in the run
 * (`COMPARISON_BASIS_WITHOUT_ROWS`) or the builder refused the pair. */
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

/** A basis of a pair with no result rows in the run, and the solve's own reason
 * when the solve withheld it (its gate's diagnostic message), else null. */
export type WithinRunBasisWithoutRows = { side: "left" | "right"; ref_id: string; withheld_reason: string | null; withheld: boolean };

/** One pair's outcome: its comparison, or the record of its refusal, and the bases
 * of the pair that have no result rows in the run (left, then right). */
export type WithinRunComparisonBuild = {
  pair: WithinRunPair;
  comparison: PreviewComparison | null;
  unavailable: PreviewComparisonUnavailable | null;
  basesWithoutRows: WithinRunBasisWithoutRows[];
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

/** Row kinds that record how a basis was solved, not a result of it (the producers'
 * modulus-basis presence records, `source_receipt/rows.rs`): a withheld combination
 * may still have them, so they never count as its result rows. */
export const BOOKKEEPING_ROW_KINDS: ReadonlySet<string> = new Set(["modulus_basis_record", "combination_modulus_basis_record"]);

/** A row that is a result of its basis (not a bookkeeping record). */
function isResultRow(row: MechanicsResult["results"][number]): boolean {
  return !BOOKKEEPING_ROW_KINDS.has(row.kind);
}

/** The combinations the solve withheld, with each gate's reason code, from either
 * evidence form: the preview route's `contract_evidence.combination_gates` and the
 * exact route's `contract_evidence.exact_combinations[].gate`. */
export function withheldCombinationGates(result: MechanicsResult): { combination_id: string; reason: string | null }[] {
  const gates = previewCombinationGates(result)
    .filter((gate) => gate.withheld)
    .map((gate) => ({ combination_id: gate.combination_id, reason: typeof gate.reason === "string" ? gate.reason : null }));
  const exact = (result.contract_evidence as { exact_combinations?: unknown } | undefined)?.exact_combinations;
  if (Array.isArray(exact)) {
    for (const record of exact) {
      const entry = record as { combination_id?: unknown; gate?: { withheld?: unknown; reason?: unknown } } | null;
      if (entry && typeof entry.combination_id === "string" && entry.gate?.withheld === true) {
        gates.push({ combination_id: entry.combination_id, reason: typeof entry.gate.reason === "string" ? entry.gate.reason : null });
      }
    }
  }
  return gates;
}

/** The load cases and combinations that have result rows in `result` (bookkeeping
 * rows do not count, and a combination the solve withheld is not offered), ordered by
 * the run's solved model (`model`, its input manifest's snapshot). */
export function withinRunBasisOptions(result: MechanicsResult, model: PreviewModel | null | undefined): WithinRunOptions {
  const loadCaseCounts = new Map<string, number>();
  const combinationCounts = new Map<string, number>();
  const withheld = new Set(withheldCombinationGates(result).map((gate) => gate.combination_id));
  for (const row of result.results) {
    const basis = row.basis_ref;
    if (!basis || typeof basis.ref_id !== "string" || basis.ref_id === "" || !isResultRow(row)) continue;
    const counts = basis.ref_type === "load_case" ? loadCaseCounts : basis.ref_type === "combination" && !withheld.has(basis.ref_id) ? combinationCounts : null;
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

/** The bases of `pair` with no result rows in `result`, each with the solve's own
 * withholding text where its combination gate withheld it: the message of the gate's
 * diagnostic (its code is the gate's reason and it names the combination first). */
export function basesWithoutRows(result: MechanicsResult, pair: WithinRunPair): WithinRunBasisWithoutRows[] {
  // Strictly by kind and id: a load case and a combination may share an id. The left
  // basis is a load case, the right a combination; only the right can be withheld.
  const hasRows = (refType: "load_case" | "combination", id: string) =>
    result.results.some((row) => row.basis_ref?.ref_type === refType && row.basis_ref.ref_id === id && isResultRow(row));
  const gate = withheldCombinationGates(result).find((item) => item.combination_id === pair.right);
  const withoutRows: WithinRunBasisWithoutRows[] = [];
  if (!hasRows("load_case", pair.left)) withoutRows.push({ side: "left", ref_id: pair.left, withheld: false, withheld_reason: null });
  if (gate || !hasRows("combination", pair.right)) {
    const message = gate
      ? result.diagnostics?.find((item) => item.code === gate.reason && item.affected_refs?.[0] === pair.right)?.message
      : undefined;
    withoutRows.push({ side: "right", ref_id: pair.right, withheld: Boolean(gate), withheld_reason: typeof message === "string" && message !== "" ? message : null });
  }
  return withoutRows;
}

/** The reason code of a pair with a basis that has no result rows in the run. */
export const COMPARISON_BASIS_WITHOUT_ROWS = "COMPARISON-BASIS-WITHOUT-RESULT-ROWS";

/** The record's reason for bases without rows: the code, the bases by their stored
 * ids, and for each basis the solve withheld, the solve's own message. */
function withoutRowsReason(withoutRows: WithinRunBasisWithoutRows[]): string {
  const ids = withoutRows.map((basis) => basis.ref_id);
  // One id for both sides (a direct caller's pair) is named once, as both.
  const subject = ids.length === 2 && ids[0] === ids[1]
    ? `${ids[0]} has no result rows in this run as the load case or as the combination, so the pair is not compared.`
    : `${ids.join(" and ")} ${ids.length === 1 ? "has" : "have"} no result rows in this run, so the pair is not compared.`;
  const withheld = withoutRows.filter((basis) => basis.withheld).map((basis) =>
    basis.withheld_reason ? `${basis.ref_id}: ${basis.withheld_reason}` : `${basis.ref_id}: withheld by the solve.`);
  return [`${COMPARISON_BASIS_WITHOUT_ROWS}: ${subject}`, ...withheld].join(" ");
}

function unavailableRecord(result: MechanicsResult, analysisRun: ComparedRun, pair: WithinRunPair, code: string, reason: string): PreviewComparisonUnavailable {
  return {
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
  };
}

/** The comparison of one pair; when a basis has no result rows in the run, the
 * unavailable record with that reason (the builder is not asked); when the unchanged
 * builder refuses the pair, the record of that refusal with the pair and the reason. */
export function buildWithinRunComparison(result: MechanicsResult, analysisRun: ComparedRun, pair: WithinRunPair): WithinRunComparisonBuild {
  const withoutRows = basesWithoutRows(result, pair);
  if (withoutRows.length > 0) {
    return { pair, comparison: null, unavailable: unavailableRecord(result, analysisRun, pair, COMPARISON_BASIS_WITHOUT_ROWS, withoutRowsReason(withoutRows)), basesWithoutRows: withoutRows };
  }
  try {
    return { pair, comparison: buildPreviewComparison({ result, analysisRun, leftBasisRef: pair.left, rightBasisRef: pair.right }), unavailable: null, basesWithoutRows: withoutRows };
  } catch (error) {
    const reason = error instanceof Error ? error.message : String(error);
    const code = /^([A-Z][A-Z0-9_-]*):/.exec(reason)?.[1] ?? "COMPARISON-BUILD-REFUSED";
    return { pair, comparison: null, unavailable: unavailableRecord(result, analysisRun, pair, code, reason), basesWithoutRows: withoutRows };
  }
}

const FIXED_PAIR: WithinRunPair = { left: DEFAULT_COMPARISON_LEFT_BASIS_REF, right: DEFAULT_COMPARISON_RIGHT_BASIS_REF };
const samePair = (left: WithinRunPair, right: WithinRunPair) => left.left === right.left && left.right === right.right;

/** Whether either fixed basis belongs to this run: it has result rows, the solve
 * withheld it (its combination gate), or the solved model lists it (the load case as
 * a load case, the combination as a combination). */
function fixedPairPresent(result: MechanicsResult, model: PreviewModel | null | undefined): boolean {
  // Strictly by kind and id (a load case and a combination may share an id); any row of
  // the basis, a bookkeeping record included, shows that it belongs to the run.
  if (result.results.some((row) =>
    (row.basis_ref?.ref_type === "load_case" && row.basis_ref.ref_id === FIXED_PAIR.left) ||
    (row.basis_ref?.ref_type === "combination" && row.basis_ref.ref_id === FIXED_PAIR.right))) return true;
  if (withheldCombinationGates(result).some((gate) => gate.combination_id === FIXED_PAIR.right)) return true;
  const loadCases = Array.isArray(model?.load_cases) ? model.load_cases : [];
  const combinations = Array.isArray(model?.combinations) ? model.combinations : [];
  return loadCases.some((entry) => entry?.id === FIXED_PAIR.left) || combinations.some((entry) => entry?.id === FIXED_PAIR.right);
}

/** The fixed pair's build, as the session holds it for the packages without a choice;
 * null, so the packages carry no record and there is no notice, when neither fixed
 * basis belongs to the run (`fixedPairPresent`; Agent 0's absent-pair rule). `model`
 * is the run's solved model. */
export function fixedPairComparison(result: MechanicsResult | null, analysisRun: ComparedRun | null, model?: PreviewModel | null): WithinRunComparisonBuild | null {
  if (!result || !analysisRun || !fixedPairPresent(result, model)) return null;
  return buildWithinRunComparison(result, analysisRun, FIXED_PAIR);
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
  analysisRun: ComparedRun | null;
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
 * choice applies, else the fixed pair's build (its unavailable record when a basis
 * has no result rows; nothing when neither fixed basis belongs to the run). */
export function packagedComparison(
  resolved: { view: WithinRunView; build: WithinRunComparisonBuild | null },
  fixedPair: WithinRunComparisonBuild | null
): WithinRunComparisonBuild | null {
  return resolved.view.status === "ready" && resolved.view.source === "selected" ? resolved.build : fixedPair;
}

/** What the packages carry for one run and a held choice, by the session's own rules
 * (`resultsSessionState.ts`): the options of `result` under its solved `model`, the
 * held choice dropped when the run does not offer both of its bases, the fixed pair
 * (or nothing, by the absent-pair rule) without an applying choice. Also returns
 * whether the held choice was dropped for this run. Pure: it reads only its inputs. */
export function packagedComparisonForRun({
  result,
  analysisRun,
  model,
  held
}: {
  result: MechanicsResult;
  analysisRun: ComparedRun;
  model: PreviewModel | null | undefined;
  held: HeldWithinRunSelection | null;
}): { packaged: WithinRunComparisonBuild | null; heldDropped: boolean } {
  const options = withinRunBasisOptions(result, model);
  const heldDropped = held !== null && droppedBases(held, options).length > 0;
  const fixedPair = fixedPairComparison(result, analysisRun, model);
  const resolved = resolveWithinRun({ result, analysisRun, options, held: heldDropped ? null : held, fixedPair });
  return { packaged: packagedComparison(resolved, fixedPair), heldDropped };
}

/** A basis as a person reads it, as the Loads page names its kinds: the kind in
 * words and the id without its stored prefix ("load case L-100", "combination
 * C-OPER-ALT"). An id without the prefix is shown as it is. */
export function basisName(side: "left" | "right", refId: string): string {
  const [kind, prefix] = side === "left" ? ["load case", "load:"] : ["combination", "combination:"];
  return `${kind} ${refId.startsWith(prefix) && refId.length > prefix.length ? refId.slice(prefix.length) : refId}`;
}

/** What the local-private report package carries for the within-run pair (the
 * shareable package carries no comparison, DEL-08-01 REQ-019):
 * - `compared`: both bases have result rows and the package includes their comparison;
 * - `none_carried`: a basis has no result rows in this run (`withoutRows`, with the
 *   solve's own withholding text where there is one), so no comparison is carried;
 * - `omitted`: the builder refused the pair (`reason` is its reason). */
export type PackageNoticeState = "compared" | "none_carried" | "omitted";
export type PackageNoticeParts = {
  /** The stored ids. */
  left: string;
  right: string;
  /** The names a person reads (`basisName`). */
  leftName: string;
  rightName: string;
  state: PackageNoticeState;
  /** `none_carried`: the bases without rows; empty otherwise. */
  withoutRows: WithinRunBasisWithoutRows[];
  /** `none_carried`: the reason in words, or null when the solve gives none;
   * `omitted`: the builder's reason; `compared`: null. */
  reason: string | null;
  text: string;
};

/** "withheld by the solve: <its reason>", from the gate's message without its
 * leading "Combination withheld:" and its final full stop; "withheld by the solve"
 * when the gate has no message; null when the solve did not withhold the basis. */
function withheldPhrase(basis: WithinRunBasisWithoutRows): string | null {
  if (!basis.withheld) return null;
  if (!basis.withheld_reason) return "withheld by the solve";
  const reason = basis.withheld_reason.replace(/^Combination withheld:\s*/, "").replace(/\.$/, "").trim();
  return reason ? `withheld by the solve: ${reason}` : "withheld by the solve";
}

/** The package notice's members and sentence. Null when nothing is packaged: no
 * Current run, or no fixed pair in the run without a choice (the absent-pair rule). */
export function packageNoticeParts(packaged: WithinRunComparisonBuild | null): PackageNoticeParts | null {
  if (!packaged) return null;
  const { left, right } = packaged.pair;
  const leftName = basisName("left", left), rightName = basisName("right", right);
  const base = { left, right, leftName, rightName };
  const withoutRows = packaged.basesWithoutRows ?? [];
  if (withoutRows.length > 0) {
    const names = withoutRows.map((basis) => basisName(basis.side, basis.ref_id));
    const phrases = withoutRows.map((basis) => [basis, withheldPhrase(basis)] as const).filter(([, phrase]) => phrase !== null);
    const reason = phrases.length === 0
      ? null
      : withoutRows.length === 1
        ? phrases[0][1]!
        : phrases.map(([basis, phrase]) => `${basisName(basis.side, basis.ref_id)} ${phrase}`).join("; ");
    const subject = `${names.join(" and ")} ${names.length === 1 ? "has" : "have"} no results in this run`;
    return {
      ...base,
      state: "none_carried",
      withoutRows,
      reason,
      text: `The local-private report package carries no comparison of ${leftName} with ${rightName}: ${subject}${reason ? ` (${reason})` : ""}.`
    };
  }
  if (packaged.comparison) {
    return { ...base, state: "compared", withoutRows: [], reason: null, text: `The local-private report package includes the comparison of ${leftName} with ${rightName}.` };
  }
  const reason = packaged.unavailable?.reason ?? "it could not be built";
  return { ...base, state: "omitted", withoutRows: [], reason, text: `The local-private report package omits the comparison of ${leftName} vs ${rightName}: ${reason}` };
}

/** The package notice: which comparison the local-private report package carries, or
 * that it carries none and why. Null when nothing is packaged (`packageNoticeParts`). */
export function packageNotice(packaged: WithinRunComparisonBuild | null): string | null {
  return packageNoticeParts(packaged)?.text ?? null;
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

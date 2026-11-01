/** Key results after a solve (survey item A6; DEL-07-05 Design `key-results-panel.md`).
 *
 * What the key-results panel offers for one result, read from the derivations the views it opens
 * already use: the stress and ratio table's rule categories (A3) and the viewport's ratio
 * quantities and cases (A4, A5), the support load summary's supports (A2) and the result's own
 * support-state rows. Nothing here forms a value, a limit, a ratio or a status: a category and its
 * ratios exist only where the user's rule-check run of this result states them, and an incomplete
 * or absent pack is reported as incomplete or absent. Each action names existing view state (a
 * viewport quantity id and case key, a support id, a results-table filter) and nothing else. */

import type { SessionResult } from "../../services/nativeResult";
import type { MechanicsResult, PreviewModel } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import { deriveRuleCheckBindingPlan } from "../../services/ruleCheckService";
import { statusDisplay } from "../workspace/statusLabels";
import {
  RULE_RATIO_QUANTITY_PREFIX,
  basisKey,
  resultOverlayCases,
  ruleOutcomePlace,
  type ResultOverlayCase
} from "../viewport/resultColourOverlay";
import type { ResultOverlayStanding } from "./numericalHealth";
import { buildStressRatioTable, type StressRatioGroup } from "./stressRatioTable";
import { SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE, buildSupportLoadSummaries, caseWithheldReason } from "./supportLoadSummary";
import { overlayMayShowRow } from "./numericalHealth";
import { rowCaseKey } from "./resultsNavigator";

type ResultRow = MechanicsResult["results"][number];

/* -------------------------------------------------------------------------- *
 * Actions and keys (§2, §5)
 * -------------------------------------------------------------------------- */

export type KeyResultsActionId =
  | "ratio"
  | "deflection"
  | "support"
  | "contact"
  | "results-table"
  | "stress-ratio-table"
  | "displacement-extremes"
  | "edit";

/** One letter per action, active while focus is in the panel and not in a field or a select. */
export const KEY_RESULTS_KEYS: Readonly<Record<KeyResultsActionId, string>> = Object.freeze({
  ratio: "R",
  deflection: "D",
  support: "S",
  contact: "C",
  "results-table": "T",
  "stress-ratio-table": "A",
  "displacement-extremes": "X",
  edit: "E"
});

const ACTION_BY_KEY = new Map(Object.entries(KEY_RESULTS_KEYS).map(([action, key]) => [key, action as KeyResultsActionId] as const));

/** The action a key press names: its letter, either case, with no Alt, Ctrl or Meta. */
export function keyResultsKeyAction(event: Readonly<{ key: string; altKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }>): KeyResultsActionId | null {
  if (event.altKey || event.ctrlKey || event.metaKey || event.key.length !== 1) return null;
  return ACTION_BY_KEY.get(event.key.toUpperCase()) ?? null;
}

/* -------------------------------------------------------------------------- *
 * The panel's contents (§3, §4)
 * -------------------------------------------------------------------------- */

/** The support-state rows of the nonlinear active-set solve (DEL-04-04, DEL-04-07): one per
 * nonlinear support and case, value 0 inactive, 1 active, 2 sticking, 3 sliding, as each row's
 * `metadata.sign_convention` states. The results table shows them; the panel only counts them. */
export const SUPPORT_STATE_ROW_KIND = "nonlinear_support_active_set_state_code";

export const KEY_RESULTS_NO_RESULT = "No result in this session yet: run the solve, and its key results are offered here.";
export const KEY_RESULTS_HISTORICAL = "A reopened run is shown in Results. Key results apply to this session's own solve, so no view is offered for it.";
export const KEY_RESULTS_NOT_SOLVED = "Mechanics not solved, so no view is drawn. The results table lists the run's diagnostics.";
export const KEY_RATIO_ABSENT =
  "No rule pack: the model references none and no rule-check run of this result is held. Stress categories and ratios come only from your rule pack, so none is shown.";
export function keyRatioNotRunText(packCount: number): string {
  return `The model references ${packCount} rule pack${packCount === 1 ? "" : "s"}, but no rule-check run of this result is held, so no stress category or ratio is shown.`;
}
export const KEY_RATIO_NOT_THIS_RESULT = "The rule-check run does not belong to this result, so its categories and ratios are not shown.";
export function keyRatioNoOutcomesText(rulePackId: string): string {
  return `The rule-check run of rule pack ${rulePackId} has no outcomes, so no ratio is shown. The rule-check run states why.`;
}
export function keyRatioUsedText(rulePackId: string, outcomeCount: number): string {
  return `Rule pack ${rulePackId}: ${outcomeCount} outcome${outcomeCount === 1 ? "" : "s"} of this result.`;
}
export function keyRatioIncompleteText(incomplete: number, total: number): string {
  return `Incomplete: ${incomplete} of ${total} outcome${total === 1 ? "" : "s"} have incomplete inputs. They form no ratio, colour nothing and are never shown as passing.`;
}
export const KEY_RATIO_NOT_PLACED =
  "None of this category's outcomes is located on a single pipe, so the contour cannot place it. The stress and ratio table lists them.";
export const KEY_RATIO_ENVELOPE_ONLY =
  "This category's outcomes on a single pipe are all on range envelopes, which the contour does not draw. The stress and ratio table lists them.";
export const KEY_RATIO_NONE = "No stress category to show.";
export const KEY_DEFLECTION_NO_CASES = "No load case or combination with result rows.";
export const KEY_SUPPORTS_NONE = "No support is declared as an anchor or equipment connection.";
export const KEY_CONTACT_NONE = "This result states no support contact or lift-off state rows; it has no nonlinear support state to show.";

export type KeyRatioCategory = Readonly<{
  /** The viewport's ratio quantity id (`rule-ratio:` and the table group's id). */
  quantityId: string;
  /** The pack's name and id, as the stress and ratio table labels the category. */
  label: string;
  outcomeCount: number;
  incompleteCount: number;
  withheldCount: number;
  /** The first case, in the viewport's case order, with an outcome the contour can place. */
  caseKey: string | null;
  caseLabel: string | null;
  /** Why the contour has nothing to place for the category, or null. */
  unavailableReason: string | null;
}>;

export type KeyRatioState = "absent" | "not_run" | "not_this_result" | "no_outcomes" | "used";

export type KeyRatioSection = Readonly<{
  state: KeyRatioState;
  note: string;
  /** The run's own aggregate status, shown with its registered label. */
  aggregate: Readonly<{ token: string; text: string }> | null;
  /** Set when some outcome of the run has incomplete inputs. */
  incompleteNote: string | null;
  categories: readonly KeyRatioCategory[];
  /** Categories the pack declares that have no outcome in this run. */
  declaredWithoutOutcomes: readonly string[];
}>;

export type KeySupport = Readonly<{ supportId: string; label: string; text: string }>;

export type KeyResults = Readonly<{
  state: "available" | "no_result" | "historical" | "not_solved";
  /** Why no view is offered, when not available. */
  reason: string | null;
  /** The numerical standing as the results panel words it, or null when nothing qualifies it. */
  standingText: string | null;
  ratio: KeyRatioSection;
  /** The viewport's cases (`resultOverlayCases`): load cases, then combinations. */
  cases: readonly ResultOverlayCase[];
  supports: Readonly<{ reason: string | null; items: readonly KeySupport[] }>;
  contact: Readonly<{ rowCount: number; caseCount: number }>;
}>;

const EMPTY_RATIO: KeyRatioSection = Object.freeze({
  state: "absent", note: KEY_RATIO_ABSENT, aggregate: null, incompleteNote: null, categories: Object.freeze([]), declaredWithoutOutcomes: Object.freeze([])
});

export function standingText(standing: ResultOverlayStanding | null | undefined): string | null {
  if (!standing) return null;
  if (standing.health) return [standing.health.headline, standing.health.outputSentence].filter(Boolean).join(". ");
  return standing.eligible ? null : SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE;
}

function ratioCategory(group: StressRatioGroup, cases: readonly ResultOverlayCase[], pipeIds: ReadonlySet<string>): KeyRatioCategory {
  const rows = group.rows;
  const placed = rows.filter((row) => ruleOutcomePlace(row, pipeIds) !== null);
  const drawable = new Set(placed.flatMap((row) => (row.case && !row.case.envelope ? [basisKey(row.case.basis)] : [])));
  const first = cases.find((item) => drawable.has(item.key)) ?? null;
  return Object.freeze({
    quantityId: `${RULE_RATIO_QUANTITY_PREFIX}${group.id}`,
    label: group.label,
    outcomeCount: rows.length,
    incompleteCount: rows.filter((row) => row.outcome?.status === "RULE_INPUTS_INCOMPLETE").length,
    withheldCount: rows.filter((row) => row.standing === "withheld").length,
    caseKey: first?.key ?? null,
    caseLabel: first?.label ?? null,
    unavailableReason: first ? null : placed.length === 0 ? KEY_RATIO_NOT_PLACED : KEY_RATIO_ENVELOPE_ONLY
  });
}

function ratioSection(input: {
  model: PreviewModel;
  result: MechanicsResult;
  standing: ResultOverlayStanding | null | undefined;
  ruleCheckRun: SessionRuleCheckRun | null;
  cases: readonly ResultOverlayCase[];
}): KeyRatioSection {
  const { model, result, standing, ruleCheckRun, cases } = input;
  const table = buildStressRatioTable({ model, result, standing, ruleCheckRun });
  const packCount = model.rule_pack_refs?.length ?? 0;
  const run = table.run;
  const base = { aggregate: null, incompleteNote: null, categories: Object.freeze([]), declaredWithoutOutcomes: Object.freeze([]) };
  if (run.state === "none") {
    return Object.freeze(packCount === 0 ? EMPTY_RATIO : { ...base, state: "not_run" as const, note: keyRatioNotRunText(packCount) });
  }
  const token = ruleCheckRun!.result.aggregate_status;
  const aggregate = Object.freeze({ token, text: statusDisplay(token) });
  if (run.state === "not_this_result") return Object.freeze({ ...base, state: "not_this_result" as const, note: KEY_RATIO_NOT_THIS_RESULT });
  if (run.state === "no_outcomes") return Object.freeze({ ...base, aggregate, state: "no_outcomes" as const, note: keyRatioNoOutcomesText(run.rulePackId ?? "") });
  const pipeIds = new Set(model.pipe_segments.map((pipe) => pipe.id));
  const groups = table.groups.filter((group) => group.source === "rule_category");
  const categories = groups.map((group) => ratioCategory(group, cases, pipeIds));
  const incomplete = ruleCheckRun!.result.checks.filter((outcome) => outcome.status === "RULE_INPUTS_INCOMPLETE").length;
  const present = new Set(groups.map((group) => group.ref));
  const declaredWithoutOutcomes = deriveRuleCheckBindingPlan(ruleCheckRun!.rulePackDocument).stressCategories
    .filter((category) => !present.has(category.category_id))
    .map((category) => (category.name ? `${category.name} (${category.category_id})` : category.category_id));
  return Object.freeze({
    state: "used" as const,
    note: keyRatioUsedText(run.rulePackId ?? "", run.outcomeCount),
    aggregate,
    incompleteNote: incomplete > 0 || token === "RULE_INPUTS_INCOMPLETE" ? keyRatioIncompleteText(incomplete, run.outcomeCount) : null,
    categories: Object.freeze(categories),
    declaredWithoutOutcomes: Object.freeze(declaredWithoutOutcomes)
  });
}

function keySupports(model: PreviewModel, result: MechanicsResult, standing: ResultOverlayStanding | null | undefined): KeyResults["supports"] {
  const summaries = buildSupportLoadSummaries({ model, result, standing });
  if (summaries.state !== "available") return Object.freeze({ reason: summaries.reason, items: Object.freeze([]) });
  const families = new Map(model.supports.map((support) => [support.id, support.family ?? null] as const));
  const items = summaries.supports.flatMap((summary): KeySupport[] => {
    const parts: string[] = [];
    if (families.get(summary.supportId) === "anchor") parts.push("anchor");
    if (summary.equipment || summary.equipmentInterfaces.length > 0) parts.push("equipment connection");
    return parts.length > 0 ? [Object.freeze({ supportId: summary.supportId, label: summary.label, text: `${summary.label} (${parts.join(", ")})` })] : [];
  });
  return Object.freeze({ reason: items.length > 0 ? null : KEY_SUPPORTS_NONE, items: Object.freeze(items) });
}

/**
 * The support-state rows the numerical standing withholds, each with the reason the A2 and A3
 * panels give for its case (`caseWithheldReason`): the overlays' rule (`overlayMayShowRow`), so a
 * row of a case or combination with no admitted health state is marked, never shown as plain.
 */
export function withheldSupportStateRows(result: MechanicsResult, standing: ResultOverlayStanding | null | undefined): Readonly<Record<string, string>> {
  const marks: Record<string, string> = {};
  for (const row of result.results) {
    if (row.kind === SUPPORT_STATE_ROW_KIND && !overlayMayShowRow(standing, row)) marks[row.id] = caseWithheldReason(standing, row.basis_ref ?? null);
  }
  return Object.freeze(marks);
}

function supportStateCounts(rows: readonly ResultRow[]): KeyResults["contact"] {
  const states = rows.filter((row) => row.kind === SUPPORT_STATE_ROW_KIND);
  return Object.freeze({ rowCount: states.length, caseCount: new Set(states.map(rowCaseKey)).size });
}

/**
 * The key results of `result`, with the inputs the results panels already take: App's
 * `viewportResultStanding` and the session's rule-check run, passed only while `result` is the
 * current solved result it was run on. `historical` is a reopened run in the Results section.
 */
export function buildKeyResults(input: {
  model: PreviewModel;
  result: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined;
  ruleCheckRun?: SessionRuleCheckRun | null;
  historical?: boolean;
}): KeyResults {
  const { model, result, standing, ruleCheckRun = null, historical = false } = input;
  const none = (state: KeyResults["state"], reason: string): KeyResults => Object.freeze({
    state, reason, standingText: null, ratio: EMPTY_RATIO, cases: Object.freeze([]),
    supports: Object.freeze({ reason, items: Object.freeze([]) }), contact: Object.freeze({ rowCount: 0, caseCount: 0 })
  });
  if (historical) return none("historical", KEY_RESULTS_HISTORICAL);
  if (!result) return none("no_result", KEY_RESULTS_NO_RESULT);
  if (result.status.mechanics !== "MECHANICS_SOLVED") return { ...none("not_solved", KEY_RESULTS_NOT_SOLVED), standingText: standingText(standing) };
  const cases = Object.freeze(resultOverlayCases(model, result));
  return Object.freeze({
    state: "available",
    reason: null,
    standingText: standingText(standing),
    ratio: ratioSection({ model, result, standing, ruleCheckRun, cases }),
    cases,
    supports: keySupports(model, result, standing),
    contact: supportStateCounts(result.results)
  });
}

/* -------------------------------------------------------------------------- *
 * Opening after a solve (§3)
 * -------------------------------------------------------------------------- */

/** What the auto-open rule remembers across results: the last result it saw, and whether it has
 * already opened the panel by itself in this app session. */
export type KeyResultsAutoOpenMemory = Readonly<{ lastResult: SessionResult | null; openedOnce: boolean }>;
export const KEY_RESULTS_AUTO_OPEN_START: KeyResultsAutoOpenMemory = Object.freeze({ lastResult: null, openedOnce: false });

/**
 * The panel opens by itself for a newly published result of the session's own solve with
 * mechanics solved: the first one of the app session, and every one while the user's "Open after
 * every solve" is set. A reopened run, a cleared result, a refusal or a failed solve opens nothing.
 */
export function keyResultsAutoOpen(
  memory: KeyResultsAutoOpenMemory,
  input: Readonly<{ result: SessionResult | null; historical: boolean; everySolve: boolean }>
): Readonly<{ open: boolean; memory: KeyResultsAutoOpenMemory }> {
  if (input.result === memory.lastResult) return { open: false, memory };
  const fresh = input.result !== null && !input.historical && input.result.status.mechanics === "MECHANICS_SOLVED";
  const open = fresh && (!memory.openedOnce || input.everySolve);
  return { open, memory: Object.freeze({ lastResult: input.result, openedOnce: memory.openedOnce || open }) };
}

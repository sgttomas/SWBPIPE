/** The governing values of a result (DEL-07-05 OUT-001: "find the governing values"): the largest
 * displacement, the largest support reaction in each direction, the largest stress of each stress
 * table group and the largest ratio of each rule category, for all cases or one case. Each is
 * selected from what the derived summaries already give, by the same builders the tables use
 * (`buildSupportLoadSummaries`, `buildDisplacementExtremes`, `buildStressRatioTable`, or S's tables
 * for a natively held result): this module selects a row and forms no value, unit, ratio or
 * standing. A withheld or range-envelope row is never selected, as the summaries leave them out of
 * their maxima; values in different units are not compared. */
import { boundSummary, isNativeResult, nativeRowKey, type SessionResult } from "../../services/nativeResult";
import { displacementExtremesFromSummary, stressRatioTableFromSummary } from "../../services/resultSummary";
import type { MechanicsResult, PreviewModel } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import { ALL_CASES } from "./resultsNavigator";
import type { ResultOverlayStanding } from "./numericalHealth";
import { buildSupportLoadSummaries, type SupportLoadColumnId, type SupportLoadSummaries } from "./supportLoadSummary";
import { buildDisplacementExtremes, displacementExtremesView, FILTER_ALL, type DisplacementComponentId } from "./displacementExtremes";
import { buildStressRatioTable, stressRatioLocationText, stressRatioView, type StressRatioRow, type StressRatioTable } from "./stressRatioTable";

/** A selected value: what it is, where, in which case, and how to reach its row. */
export type GoverningValue = Readonly<{
  value: number;
  unit: string;
  /** The display-units dimension of the value; "ratio" for a ratio, which has no unit. */
  dimension: string;
  /** The entity's readable name, as its table names it. */
  entityLabel: string;
  caseKey: string | null;
  caseLabel: string;
  /** The component or quantity, as its table's column names it. */
  component: string;
}>;

export type GoverningDisplacement = GoverningValue & Readonly<{ nodeId: string }>;
export type GoverningReaction = GoverningValue & Readonly<{ supportId: string; column: SupportLoadColumnId }>;
export type GoverningStress = GoverningValue & Readonly<{ groupId: string; groupLabel: string; resultId: string | null; entityRef: string | null }>;

/** A quantity this result does not have, in a few words, in its place. */
export type Absent = Readonly<{ absent: string }>;

export type GoverningValues = Readonly<{
  displacement: GoverningDisplacement | Absent;
  reactions: readonly (GoverningReaction | (Absent & { column: SupportLoadColumnId }))[];
  stresses: readonly GoverningStress[] | Absent;
  ratios: readonly GoverningStress[] | Absent;
}>;

/** The reaction directions summarised: the force components, then the moment components. */
export const GOVERNING_REACTION_COLUMNS: readonly SupportLoadColumnId[] = Object.freeze(["Fx", "Fy", "Fz", "Mx", "My", "Mz"]);
const TRANSLATIONS: readonly DisplacementComponentId[] = ["ux", "uy", "uz"];

export const GOVERNING_NO_RESULT = "No result";
export const GOVERNING_NONE_ADMITTED = "None admitted";
export const GOVERNING_UNITS_DIFFER = "Units differ";

/** The support summaries the support table shows for this result. */
export function supportSummariesOf(model: PreviewModel, result: SessionResult | null, standing: ResultOverlayStanding | null | undefined): SupportLoadSummaries {
  if (result && isNativeResult(result)) {
    const summary = boundSummary(result, model);
    return summary ? summary.header.tables.support_load_summaries as SupportLoadSummaries : buildSupportLoadSummaries({ model, result: null, standing });
  }
  return buildSupportLoadSummaries({ model, result: result as MechanicsResult | null, standing });
}

/** The stress and ratio table the stress table shows for this result. */
export function stressTableOf(model: PreviewModel, result: SessionResult | null, fullResult: MechanicsResult | null, standing: ResultOverlayStanding | null | undefined, ruleCheckRun: SessionRuleCheckRun | null): StressRatioTable {
  if (!result || !isNativeResult(result)) return buildStressRatioTable({ model, result: result as MechanicsResult | null, standing, ruleCheckRun });
  if (ruleCheckRun) return buildStressRatioTable({ model, result: fullResult, standing, ruleCheckRun });
  const summary = boundSummary(result, model);
  return summary ? stressRatioTableFromSummary(summary, nativeRowKey) : buildStressRatioTable({ model, result: null, standing, ruleCheckRun: null });
}

/** Of candidates in one unit, the one of largest magnitude (the first on a tie); Absent when there is
 * none or their units differ. */
function largest<T extends { value: number; unit: string }>(candidates: readonly T[]): T | Absent {
  if (candidates.length === 0) return { absent: GOVERNING_NONE_ADMITTED };
  if (new Set(candidates.map((item) => item.unit)).size > 1) return { absent: GOVERNING_UNITS_DIFFER };
  return candidates.reduce((best, item) => (Math.abs(item.value) > Math.abs(best.value) ? item : best));
}

export function isAbsent(value: unknown): value is Absent {
  return typeof value === "object" && value !== null && "absent" in value;
}

export function governingValues(input: {
  model: PreviewModel;
  result: SessionResult | null;
  fullResult?: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined;
  ruleCheckRun?: SessionRuleCheckRun | null;
  caseKey: string;
}): GoverningValues {
  const { model, result, standing, caseKey } = input;
  const chosen = caseKey === ALL_CASES ? null : caseKey;
  if (!result) {
    const none = { absent: GOVERNING_NO_RESULT };
    return { displacement: none, reactions: GOVERNING_REACTION_COLUMNS.map((column) => ({ ...none, column })), stresses: none, ratios: none };
  }

  // Displacement: the node extremes of the displacement table over the chosen cases.
  const data = result && isNativeResult(result)
    ? (boundSummary(result, model) ? displacementExtremesFromSummary(boundSummary(result, model)!) : buildDisplacementExtremes({ model, result: null, standing }))
    : buildDisplacementExtremes({ model, result: result as MechanicsResult | null, standing });
  let displacement: GoverningDisplacement | Absent = { absent: data.reason ?? GOVERNING_NONE_ADMITTED };
  if (data.state === "available") {
    const caseKeys = chosen ? (data.cases.some((item) => item.key === chosen && item.choosable) ? [chosen] : []) : null;
    const view = displacementExtremesView(data, { caseKeys, filter: FILTER_ALL, sort: "model" });
    const candidates: GoverningDisplacement[] = [];
    for (const node of view.rows) {
      for (const component of TRANSLATIONS) {
        for (const extreme of [node.extremes[component].max, node.extremes[component].min]) {
          if (extreme) candidates.push({ value: extreme.value, unit: extreme.unit, dimension: "displacement", entityLabel: node.label, caseKey: extreme.caseKey, caseLabel: extreme.caseLabel, component, nodeId: node.nodeId });
        }
      }
    }
    displacement = largest(candidates);
  }

  // Reactions: each direction's largest over the supports, from the support table's rows.
  const summaries = supportSummariesOf(model, result, standing);
  const reactions = GOVERNING_REACTION_COLUMNS.map((column) => {
    if (summaries.state !== "available") return { absent: summaries.reason ?? GOVERNING_NONE_ADMITTED, column };
    const candidates: GoverningReaction[] = [];
    for (const summary of summaries.supports) {
      if (chosen) {
        for (const row of summary.rows) {
          const value = row.values[column];
          if (row.case.key === chosen && row.standing === "admitted" && row.inExtremes && value) candidates.push({ value: value.value, unit: value.unit, dimension: column.startsWith("F") ? "force" : "moment", entityLabel: summary.label, caseKey: row.case.key, caseLabel: row.case.label, component: column, supportId: summary.supportId, column });
        }
      } else {
        const extreme = summary.extremes.find((item) => item.column === column);
        for (const value of [extreme?.max, extreme?.min]) {
          if (value) candidates.push({ value: value.value, unit: value.unit, dimension: column.startsWith("F") ? "force" : "moment", entityLabel: summary.label, caseKey: value.caseKey, caseLabel: value.caseLabel, component: column, supportId: summary.supportId, column });
        }
      }
    }
    const found = largest(candidates);
    return isAbsent(found) ? { ...found, column } : found;
  });

  // Stress and ratio: the top row of each group of the stress table, as the table ranks it.
  const table = stressTableOf(model, result, input.fullResult ?? null, standing, input.ruleCheckRun ?? null);
  const top = (source: "emitted_stress" | "rule_category", mode: "stress" | "ratio"): GoverningStress[] => table.groups
    .filter((group) => group.source === source && group.modes.includes(mode))
    .flatMap((group) => {
      const rows = chosen ? group.rows.filter((row) => row.case?.key === chosen) : group.rows;
      const view = stressRatioView({ ...group, rows }, { mode, threshold: "" });
      const first = view.sorted ? view.ranked[0]?.row : undefined;
      return first ? [stressItem(group.id, group.label, first, mode)] : [];
    });
  const stresses = top("emitted_stress", "stress");
  const ratios = top("rule_category", "ratio");
  return {
    displacement,
    reactions,
    stresses: table.state !== "available" ? { absent: table.reason ?? GOVERNING_NONE_ADMITTED } : stresses.length > 0 ? stresses : { absent: GOVERNING_NONE_ADMITTED },
    ratios: ratios.length > 0 ? ratios : { absent: ratioAbsence(table, model) }
  };
}

function stressItem(groupId: string, groupLabel: string, row: StressRatioRow, mode: "stress" | "ratio"): GoverningStress {
  const location = row.locations[0];
  const base = { groupId, groupLabel, entityLabel: location ? stressRatioLocationText(location) : row.target.entityRef ?? "", caseKey: row.case?.key ?? null, caseLabel: row.caseLabel, resultId: row.target.resultId, entityRef: row.target.entityRef };
  if (mode === "ratio") return { ...base, value: row.outcome!.ratio!, unit: "", dimension: "ratio", component: "ratio" };
  const value = row.stress ?? row.outcome!.computed!;
  return { ...base, value: value.value, unit: value.unit, dimension: "stress", component: location?.component ?? "stress" };
}

/** Why there is no ratio, in a few words. */
export function ratioAbsence(table: StressRatioTable, model: PreviewModel): string {
  if (table.run.state === "none") return (model.rule_pack_refs?.length ?? 0) === 0 ? "No rule pack" : "Rule check not run";
  if (table.run.state === "not_this_result") return "Rule check of another run";
  if (table.run.state === "no_outcomes") return "No rule-check outcomes";
  return "No complete ratio";
}

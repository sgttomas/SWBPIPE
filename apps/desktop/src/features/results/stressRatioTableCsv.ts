// RT-R6 CSV adapter of the stress and ratio table (DEL-08-04-R14; Design `csv-export.md` §2).
// One file per group. It reads the view the panel renders (`stressRatioView` under the current
// mode and threshold, every page: `stressRatioPage` with no row limit), in the view's order, and
// writes the user-rule outcome columns with the table's own wording. Rule packs appear in the
// preamble by identifier, version and checksum only (R8). It recomputes nothing.
import type { CsvExport } from "../result-export/CsvExportControl";
import { CSV_NOT_RECORDED, csvExportFileName, type CsvProvenance, type CsvRulePackRef } from "../result-export/csvTable";
import { joinColumnGroups, quantityColumns, textColumn, type CsvQuantityEntry } from "../result-export/csvValues";
import {
  ratioDirection,
  stressRatioLocationText,
  stressRatioPage,
  type RatioDirection,
  type StressRatioGroup,
  type StressRatioRow,
  type StressRatioValue,
  type StressRatioView
} from "./stressRatioTable";

export const STRESS_RATIO_CSV_KIND = "stress-ratio";

/** The table's status wording (`StressRatioTablePanel`). */
export const STRESS_RATIO_STATUS_TEXT: Readonly<Record<string, string>> = Object.freeze({
  RULE_INPUTS_INCOMPLETE: "Incomplete (RULE_INPUTS_INCOMPLETE)",
  USER_RULE_FAILED: "Failed by your rule (USER_RULE_FAILED)",
  USER_RULE_CHECKED: "Checked by your rule (USER_RULE_CHECKED)"
});
/** Each ratio row's direction (§3.1), in the Direction column. */
export const RATIO_DIRECTION_TEXT: Readonly<Record<RatioDirection, string>> = Object.freeze({
  upper: "Upper bound: computed ÷ limit",
  lower: "Lower bound: limit ÷ computed"
});
export const STRESS_RATIO_WITHHELD = "Withheld";
export const STRESS_RATIO_NOT_RANKED = "not ranked";
export const STRESS_RATIO_NONE = "none";
export const STRESS_RATIO_NO_RATIO = "No ratio is formed";
export const STRESS_RATIO_NO_LOCATION = "No location stated";

type Line = Readonly<{
  position: string;
  part: "withheld-case" | "pinned" | "ranked" | "unranked";
  row: StressRatioRow | null;
  caseKey: string;
  caseLabel: string;
  /** For a withheld case or a pinned row: the reason in place of every value. */
  reason: string | null;
}>;

function lines(group: StressRatioGroup, view: StressRatioView): Line[] {
  const page = stressRatioPage(view, Number.POSITIVE_INFINITY);
  return [
    ...group.withheldCases.map((item): Line => ({
      position: STRESS_RATIO_NOT_RANKED, part: "withheld-case", row: null, caseKey: item.case.key, caseLabel: item.case.label,
      reason: `${item.rowCount} row${item.rowCount === 1 ? "" : "s"} withheld. ${item.reason}`
    })),
    ...view.pinned.map((row): Line => {
      const outcome = row.outcome!;
      const incomplete = outcome.status === "RULE_INPUTS_INCOMPLETE";
      const reason = incomplete ? `${outcome.incompleteReason ?? ""}${row.standing === "withheld" ? ` ${row.withheldReason ?? ""}` : ""}`.trim() : row.withheldReason ?? STRESS_RATIO_WITHHELD;
      return { position: STRESS_RATIO_NOT_RANKED, part: "pinned", row, caseKey: row.case?.key ?? "", caseLabel: row.caseLabel, reason: reason || STRESS_RATIO_WITHHELD };
    }),
    ...page.ranked.map((item): Line => ({ position: String(item.rank), part: "ranked", row: item.row, caseKey: item.row.case?.key ?? "", caseLabel: item.row.caseLabel, reason: null })),
    ...view.unranked.map((row): Line => ({ position: STRESS_RATIO_NOT_RANKED, part: "unranked", row, caseKey: row.case?.key ?? "", caseLabel: row.caseLabel, reason: null }))
  ];
}

const entry = (value: StressRatioValue | null | undefined, absent: string): CsvQuantityEntry => (value ? { value: value.value, unit: value.unit } : { absent });

export function stressRatioCsv(input: {
  group: StressRatioGroup;
  view: StressRatioView;
  provenance: CsvProvenance;
  /** The rule packs of the session's rule-check run, when the group shows its outcomes. */
  rulePacks: readonly CsvRulePackRef[];
  softwareVersion: string;
  unitBasis: string;
  threshold: string;
}): CsvExport {
  const { group, view } = input;
  const rule = group.source === "rule_category";
  const listed = lines(group, view);
  const groups = [
    textColumn(view.sorted ? "Rank" : "Position", listed.map((line) => line.position)),
    textColumn("Part", listed.map((line) => line.part)),
    textColumn("Row key", listed.map((line) => line.row?.key ?? "")),
    textColumn("Case key", listed.map((line) => line.caseKey)),
    textColumn("Case", listed.map((line) => line.caseLabel)),
    textColumn("Location", listed.map((line) => (line.row ? (line.row.locations.length > 0 ? line.row.locations.map(stressRatioLocationText).join("; ") : STRESS_RATIO_NO_LOCATION) : ""))),
    textColumn("Location result ids", listed.map((line) => line.row?.locations.flatMap((item) => (item.resultId ? [item.resultId] : [])).join("; ") ?? "")),
    textColumn("Standing", listed.map((line) => (line.part === "withheld-case" ? "withheld" : line.row!.standing)))
  ];
  if (rule) {
    groups.push(
      textColumn("Check", listed.map((line) => line.row?.outcome?.checkId ?? "")),
      textColumn("Outcome identity", listed.map((line) => line.row?.outcome?.identity ?? "")),
      textColumn("Status", listed.map((line) => {
        const outcome = line.row?.outcome;
        if (!outcome) return "";
        if (line.part === "pinned") return outcome.status === "RULE_INPUTS_INCOMPLETE" ? STRESS_RATIO_STATUS_TEXT.RULE_INPUTS_INCOMPLETE : STRESS_RATIO_WITHHELD;
        return STRESS_RATIO_STATUS_TEXT[outcome.status] ?? outcome.status;
      })),
      textColumn("Reason", listed.map((line) => line.reason ?? "")),
      quantityColumns("Ratio", listed.map((line) => {
        const outcome = line.row?.outcome;
        if (line.reason !== null || !outcome) return { absent: line.reason ?? STRESS_RATIO_NONE };
        return outcome.ratio !== null && ratioDirection(outcome.relation) ? { value: outcome.ratio, unit: "" } : { absent: outcome.ratioNote ?? STRESS_RATIO_NO_RATIO };
      })),
      textColumn("Direction", listed.map((line) => {
        const outcome = line.row?.outcome;
        const direction = outcome && outcome.ratio !== null && line.reason === null ? ratioDirection(outcome.relation) : null;
        return direction ? RATIO_DIRECTION_TEXT[direction] : STRESS_RATIO_NONE;
      })),
      textColumn("Relation", listed.map((line) => line.row?.outcome?.relation ?? "")),
      quantityColumns("Computed", listed.map((line) => (line.reason !== null ? { absent: line.reason } : entry(line.row?.outcome?.computed, STRESS_RATIO_NONE)))),
      quantityColumns("Limit", listed.map((line) => (line.reason !== null ? { absent: line.reason } : entry(line.row?.outcome?.limit, STRESS_RATIO_NONE))))
    );
  } else {
    groups.push(
      quantityColumns("Stress", listed.map((line) => (line.reason !== null ? { absent: line.reason } : entry(line.row?.stress, STRESS_RATIO_NONE)))),
      textColumn("Stress result id", listed.map((line) => line.row?.stress?.resultId ?? ""))
    );
  }
  const { columns, rows } = joinColumnGroups(listed.length, groups);
  const threshold = input.threshold.trim() === "" ? "none" : input.threshold.trim();
  return {
    table: {
      preamble: {
        table: STRESS_RATIO_CSV_KIND, softwareVersion: input.softwareVersion, ...input.provenance,
        caseOrFilter: `category: ${group.label} (${group.ref}); shown by: ${view.mode}; threshold: ${threshold}`,
        unitBasis: input.unitBasis,
        ...(rule && input.rulePacks.length > 0 ? { rulePacks: input.rulePacks } : {})
      },
      columns, rows
    },
    fileName: csvExportFileName(STRESS_RATIO_CSV_KIND, group.id)
  };
}

/** A rule pack reference (`rulePackReference`, the report's) as identifier, version and
 * checksum; a value the pack does not record is `not_recorded`. */
export function csvRulePackRef(reference: { rule_pack_id: string; version: string; checksum: { algorithm: string; value: string } }): CsvRulePackRef {
  const token = (value: string | null | undefined) => (typeof value === "string" && value.trim() !== "" && !/\s/.test(value) && value !== "not recorded" ? value : CSV_NOT_RECORDED);
  const checksum = reference.checksum.value.includes(":") ? reference.checksum.value : `${reference.checksum.algorithm}:${reference.checksum.value}`;
  return { id: token(reference.rule_pack_id), version: token(reference.version), checksum: token(checksum) };
}

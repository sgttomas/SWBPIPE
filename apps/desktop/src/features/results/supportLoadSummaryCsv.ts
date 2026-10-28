// RT-R6 CSV adapter of the support load summary (DEL-08-04-R14; Design `csv-export.md` §2). It
// reads the summaries the panel renders (`buildSupportLoadSummaries`): the shown support, with
// its maximum and minimum rows (`support-loads`), or every support with one row per support and
// case (`support-loads-all`). It recomputes nothing and reads nothing from the page.
import type { CsvExport } from "../result-export/CsvExportControl";
import { csvExportFileName, type CsvProvenance } from "../result-export/csvTable";
import { joinColumnGroups, quantityColumns, textColumn, type CsvQuantityEntry } from "../result-export/csvValues";
import {
  SUPPORT_LOAD_COLUMNS,
  type SupportLoadRow,
  type SupportLoadSummaries,
  type SupportLoadSummary,
  type SupportLoadValue
} from "./supportLoadSummary";

export const SUPPORT_LOADS_CSV_KIND = "support-loads";
export const SUPPORT_LOADS_ALL_CSV_KIND = "support-loads-all";

/** The panel's text for a row's absent reactions (`SupportLoadSummaryPanel`). */
export const SUPPORT_LOAD_ABSENCE_TEXT: Readonly<Record<NonNullable<SupportLoadRow["reactionAbsence"]>, string>> = Object.freeze({
  attribution_withheld: "Reaction attribution withheld by the solver",
  no_rows: "No reaction rows",
  frame_not_global: "Reaction rows in another frame; not tabulated"
});
/** The panel's text for a cell with no emitted value, and an extreme with none. */
export const SUPPORT_LOAD_NOT_EMITTED = "not emitted";
export const SUPPORT_LOAD_NO_EXTREME = "none";

export function supportLoadAbsenceText(row: SupportLoadRow): string | null {
  return row.reactionAbsence ? `${SUPPORT_LOAD_ABSENCE_TEXT[row.reactionAbsence]}${row.reactionAbsenceDetail ? ` (${row.reactionAbsenceDetail})` : ""}` : null;
}

type Line = Readonly<{
  summary: SupportLoadSummary;
  row: "case" | "maximum" | "minimum";
  caseKey: string;
  caseLabel: string;
  envelope: string;
  standing: string;
  status: string;
  values: ReadonlyArray<Readonly<{ value: SupportLoadValue | null; caseKey: string; caseLabel: string; absent: string }>>;
}>;

function caseLine(summary: SupportLoadSummary, row: SupportLoadRow): Line {
  const absence = supportLoadAbsenceText(row);
  return {
    summary, row: "case", caseKey: row.case.key, caseLabel: row.case.label, envelope: row.case.envelope ? "range envelope" : "",
    standing: row.standing, status: row.standing === "withheld" ? row.withheldReason ?? "" : absence ?? "",
    values: SUPPORT_LOAD_COLUMNS.map((item) => {
      const value = row.values[item.id] ?? null;
      const absent = row.standing === "withheld" ? row.withheldReason ?? "withheld"
        : item.of === "support" && absence ? absence : SUPPORT_LOAD_NOT_EMITTED;
      return { value: row.standing === "withheld" || (item.of === "support" && absence) ? null : value, caseKey: row.case.key, caseLabel: row.case.label, absent };
    })
  };
}

function extremeLines(summary: SupportLoadSummary): Line[] {
  const extremes = new Map(summary.extremes.map((item) => [item.column, item] as const));
  return (["max", "min"] as const).map((which) => ({
    summary, row: which === "max" ? "maximum" : "minimum", caseKey: "", caseLabel: which === "max" ? "Maximum" : "Minimum",
    envelope: "", standing: "", status: "",
    values: SUPPORT_LOAD_COLUMNS.map((item) => {
      const extreme = extremes.get(item.id);
      const value = extreme?.[which] ?? null;
      return { value, caseKey: value?.caseKey ?? "", caseLabel: value?.caseLabel ?? "", absent: extreme?.note ?? SUPPORT_LOAD_NO_EXTREME };
    })
  }));
}

function table(lines: readonly Line[]) {
  const groups = [
    textColumn("Support id", lines.map((line) => line.summary.supportId)),
    textColumn("Support", lines.map((line) => line.summary.label)),
    textColumn("Node id", lines.map((line) => line.summary.nodeId)),
    textColumn("Row", lines.map((line) => line.row)),
    textColumn("Case key", lines.map((line) => line.caseKey)),
    textColumn("Case", lines.map((line) => line.caseLabel)),
    textColumn("Envelope", lines.map((line) => line.envelope)),
    textColumn("Standing", lines.map((line) => line.standing)),
    textColumn("Status", lines.map((line) => line.status)),
    textColumn("Sign convention", lines.map((line) => line.summary.signConvention ?? "no reaction rows listed"))
  ];
  SUPPORT_LOAD_COLUMNS.forEach((item, index) => {
    const entries: CsvQuantityEntry[] = lines.map((line) => {
      const value = line.values[index];
      return value.value ? { value: value.value.value, unit: value.value.unit } : { absent: value.absent };
    });
    groups.push(quantityColumns(item.label, entries));
    groups.push(textColumn(`${item.label} result id`, lines.map((line) => line.values[index].value?.resultId ?? "")));
    groups.push(textColumn(`${item.label} case key`, lines.map((line) => (line.values[index].value ? line.values[index].caseKey : ""))));
  });
  return joinColumnGroups(lines.length, groups);
}

type Context = Readonly<{ provenance: CsvProvenance; softwareVersion: string; unitBasis: string }>;

/** The shown support's table: its case rows, then its maximum and minimum rows. */
export function supportLoadCsv(input: Context & { summary: SupportLoadSummary }): CsvExport {
  const { summary } = input;
  const { columns, rows } = table([...summary.rows.map((row) => caseLine(summary, row)), ...extremeLines(summary)]);
  return {
    table: {
      preamble: {
        table: SUPPORT_LOADS_CSV_KIND, softwareVersion: input.softwareVersion, ...input.provenance,
        caseOrFilter: `support ${summary.supportId}; every case row`, unitBasis: input.unitBasis
      },
      columns, rows
    },
    fileName: csvExportFileName(SUPPORT_LOADS_CSV_KIND, summary.supportId)
  };
}

/** Every support in one file: one row per support and case, in support and case order. */
export function supportLoadsAllCsv(input: Context & { summaries: SupportLoadSummaries }): CsvExport {
  const lines = input.summaries.supports.flatMap((summary) => summary.rows.map((row) => caseLine(summary, row)));
  const { columns, rows } = table(lines);
  return {
    table: {
      preamble: {
        table: SUPPORT_LOADS_ALL_CSV_KIND, softwareVersion: input.softwareVersion, ...input.provenance,
        caseOrFilter: "every support; every case row", unitBasis: input.unitBasis
      },
      columns, rows
    },
    fileName: csvExportFileName(SUPPORT_LOADS_ALL_CSV_KIND)
  };
}

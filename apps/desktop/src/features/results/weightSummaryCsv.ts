// RT-R6 CSV adapters of the weight summary and centre-of-gravity tables (DEL-08-04-R14; Design
// `csv-export.md` §2). They read the summary the panel renders (`computeWeightSummary`'s
// result). Model data: there is no result, so the preamble states `result_hash` and
// `analysis_route` as `not_applicable`. They recompute nothing and read nothing from the page.
import type { CsvExport } from "../result-export/CsvExportControl";
import { csvExportFileName, type CsvProvenance } from "../result-export/csvTable";
import { joinColumnGroups, quantityColumns, textColumn, type CsvQuantityEntry } from "../result-export/csvValues";
import type { CategoryTotal, WeightSummary } from "./weightSummary";

export const WEIGHT_CSV_KIND = "weight";
export const WEIGHT_COG_CSV_KIND = "weight-cog";
/** The panel's row headers and its text for a category with no total. */
export const WEIGHT_PIPES_LABEL = "Pipes (metal with any stated contents and insulation)";
export const WEIGHT_COMPONENTS_LABEL = "Components (weight in place of their member's)";
export const WEIGHT_NO_TOTAL = "none";

type Available = Extract<WeightSummary, { state: "available" }>;
type Context = Readonly<{ summary: Available; provenance: CsvProvenance; softwareVersion: string; unitBasis: string }>;

function caseOrFilter(summary: Available): string {
  return `scope: ${summary.scope.label} (${summary.scope.id}); gravity: ${summary.gravity.value} ${summary.gravity.unit} along ${summary.gravity.axis}${summary.partial ? "; partial" : ""}`;
}

type Line = Readonly<{ id: string; label: string; counted: number; weight: CsvQuantityEntry }>;

function categoryLines(id: string, label: string, category: CategoryTotal): Line[] {
  if (category.totals.length === 0) return [{ id, label, counted: category.count, weight: { absent: WEIGHT_NO_TOTAL } }];
  return category.totals.map((item) => ({ id, label, counted: category.count, weight: { value: item.weight, unit: item.unit } }));
}

/** The weight table: one row per category and stated unit, then the total. */
export function weightSummaryCsv(input: Context & { totalLabel: string }): CsvExport {
  const { summary } = input;
  const lines: Line[] = [
    ...categoryLines("pipes", WEIGHT_PIPES_LABEL, summary.pipes),
    ...categoryLines("components", WEIGHT_COMPONENTS_LABEL, summary.components),
    {
      id: "total", label: input.totalLabel, counted: summary.pipes.count + summary.components.count,
      weight: summary.total ? { value: summary.total.weight, unit: summary.total.unit } : { absent: summary.note ?? WEIGHT_NO_TOTAL }
    }
  ];
  const { columns, rows } = joinColumnGroups(lines.length, [
    textColumn("Category id", lines.map((line) => line.id)),
    textColumn("Category", lines.map((line) => line.label)),
    quantityColumns("Counted", lines.map((line) => ({ value: line.counted, unit: "" }))),
    quantityColumns("Weight", lines.map((line) => line.weight))
  ]);
  return {
    table: {
      preamble: { table: WEIGHT_CSV_KIND, softwareVersion: input.softwareVersion, ...input.provenance, caseOrFilter: caseOrFilter(summary), unitBasis: input.unitBasis },
      columns, rows
    },
    fileName: csvExportFileName(WEIGHT_CSV_KIND, summary.scope.id)
  };
}

/** The centre-of-gravity table: the global position in the model's length unit. */
export function weightCogCsv(input: Context & { cogLabel: string }): CsvExport {
  const { summary } = input;
  const cog = summary.centreOfGravity;
  const axis = (key: "x" | "y" | "z"): CsvQuantityEntry => (cog ? { value: cog[key], unit: summary.lengthUnit } : { absent: summary.note ?? WEIGHT_NO_TOTAL });
  const { columns, rows } = joinColumnGroups(1, [
    textColumn("Centre of gravity", [input.cogLabel]),
    textColumn("Frame", ["Global"]),
    quantityColumns("X", [axis("x")]),
    quantityColumns("Y", [axis("y")]),
    quantityColumns("Z", [axis("z")])
  ]);
  return {
    table: {
      preamble: { table: WEIGHT_COG_CSV_KIND, softwareVersion: input.softwareVersion, ...input.provenance, caseOrFilter: caseOrFilter(summary), unitBasis: input.unitBasis },
      columns, rows
    },
    fileName: csvExportFileName(WEIGHT_COG_CSV_KIND, summary.scope.id)
  };
}

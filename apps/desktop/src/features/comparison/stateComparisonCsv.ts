// RT-R6 CSV adapter of the saved-state comparison table (DEL-08-04-R14; Design `csv-export.md`
// §2). It reads the comparison the view renders (`StateComparison` rows under the view's
// filters, every page): one line per field change, or one line for an entity without one.
// Values are written as stored, unconverted, as the view shows them. Two-sided: side `a` is the
// view's Left state and `b` its Right state; neither has a result, so their result hashes and
// analysis routes are `not_applicable`.
import type { ModelStateSummary } from "../../services/modelStateService";
import type { CsvExport } from "../result-export/CsvExportControl";
import { csvRecordedModelHash } from "../result-export/csvProvenance";
import { CSV_NOT_APPLICABLE, csvExportFileName } from "../result-export/csvTable";
import { CSV_DIMENSIONLESS_UNIT, joinColumnGroups, quantityColumns, storedValueCell, textColumn, type CsvQuantityEntry } from "../result-export/csvValues";
import {
  FAMILY_LABELS,
  isQuantity,
  pathText,
  type FieldChange,
  type RowClassification,
  type StateComparisonRow
} from "./stateComparison";

export const STATE_COMPARISON_CSV_KIND = "state-comparison";

/** The view's wording (`StateComparisonView`). */
export const CLASSIFICATION_LABELS: Record<RowClassification, string> = {
  added: "Added",
  removed: "Removed",
  changed: "Changed",
  unchanged: "Unchanged"
};
export const CHANGE_LABELS: Record<FieldChange["kind"], string> = {
  added: "added",
  removed: "removed",
  changed: "changed",
  unit_changed: "unit changed (not converted)"
};
export const STATE_VALUE_ABSENT = "(absent)";
export const STATE_VALUE_NOT_ONE_NUMBER = "not one number; see the stored text";

/** A stored value as the view shows it, uncut: a quantity as its number(s) and its own unit,
 * anything else as JSON. The view's `storedValueText` is this, cut to its cell length. */
export function storedValueFullText(value: unknown): string {
  if (value === undefined) return STATE_VALUE_ABSENT;
  if (isQuantity(value)) {
    const numbers = Object.entries(value).filter(([key]) => key !== "unit");
    const shownNumbers = numbers.length === 1 && numbers[0][0] === "value"
      ? String(numbers[0][1])
      : `{${numbers.map(([key, number]) => `${key}: ${String(number)}`).join(", ")}}`;
    return `${shownNumbers} ${value.unit}`;
  }
  return JSON.stringify(value);
}

/** One side's stored value as a number in its own unit, when it is one number. */
function sideEntry(value: unknown): CsvQuantityEntry {
  if (value === undefined) return { absent: STATE_VALUE_ABSENT };
  if (typeof value === "number" && Number.isFinite(value)) return { value, unit: "" };
  if (isQuantity(value)) {
    const numbers = Object.entries(value).filter(([key]) => key !== "unit");
    if (numbers.length === 1 && numbers[0][0] === "value") return { value: numbers[0][1] as number, unit: value.unit };
  }
  return { absent: STATE_VALUE_NOT_ONE_NUMBER };
}

const storedUnit = (unit: string) => (unit === "" ? CSV_DIMENSIONLESS_UNIT : unit);

type Line = Readonly<{ row: StateComparisonRow; change: FieldChange | null }>;

export function stateComparisonCsv(input: {
  left: ModelStateSummary;
  right: ModelStateSummary;
  /** The rows the view's filters match, in the view's order. */
  rows: readonly StateComparisonRow[];
  filters: Readonly<{ classification: string; family: string; idFilter: string }>;
  softwareVersion: string;
  unitBasis: string;
}): CsvExport {
  const lines: Line[] = input.rows.flatMap((row): Line[] => (row.changes.length > 0 ? row.changes.map((change) => ({ row, change })) : [{ row, change: null }]));
  const side = (name: "left" | "right") => [
    quantityColumns(name === "left" ? "Left value" : "Right value", lines.map((line) => (line.change ? sideEntry(line.change[name]) : { absent: STATE_VALUE_ABSENT })), (value) => storedValueCell(value), storedUnit),
    textColumn(name === "left" ? "Left stored" : "Right stored", lines.map((line) => (line.change ? storedValueFullText(line.change[name]) : "")))
  ];
  const { columns, rows } = joinColumnGroups(lines.length, [
    textColumn("Family id", lines.map((line) => line.row.family)),
    textColumn("Family", lines.map((line) => FAMILY_LABELS[line.row.family])),
    textColumn("Id", lines.map((line) => line.row.id)),
    textColumn("Key", lines.map((line) => line.row.key.join("/"))),
    textColumn("Classification", lines.map((line) => CLASSIFICATION_LABELS[line.row.classification])),
    textColumn("Field", lines.map((line) => (line.change ? pathText(line.change.path) : ""))),
    textColumn("Change", lines.map((line) => (line.change ? CHANGE_LABELS[line.change.kind] : ""))),
    ...side("left"),
    ...side("right")
  ]);
  const { left, right, filters } = input;
  return {
    table: {
      preamble: {
        table: STATE_COMPARISON_CSV_KIND,
        softwareVersion: input.softwareVersion,
        sides: {
          a: { modelHash: csvRecordedModelHash(left.model_hash), resultHash: CSV_NOT_APPLICABLE, analysisRoutes: [CSV_NOT_APPLICABLE] },
          b: { modelHash: csvRecordedModelHash(right.model_hash), resultHash: CSV_NOT_APPLICABLE, analysisRoutes: [CSV_NOT_APPLICABLE] }
        },
        caseOrFilter: `a = left state ${left.state_id}; b = right state ${right.state_id}; classification: ${filters.classification}; family: ${filters.family}; id contains: ${filters.idFilter.trim() || "any"}`,
        unitBasis: input.unitBasis
      },
      columns, rows
    },
    fileName: csvExportFileName(STATE_COMPARISON_CSV_KIND, `${left.state_id} ${right.state_id}`)
  };
}

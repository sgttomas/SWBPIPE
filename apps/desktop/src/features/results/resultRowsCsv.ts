// RT-R6 CSV adapter of the results panel's row tables, one per group, and its endpoint pair
// (DEL-08-04-R14; Design `csv-export.md` §2). The panel passes each row as it renders it, with
// the readout its value cell shows (`useDisplayQuantity`, through `useDisplayQuantityResolver`:
// the same preference, converter and request). HELP_HUMAN engineering-judgment resolution,
// 2026-10-28: the CSV writes that converter's binary64 value, the value the panel shows, as its
// shortest round-trip decimal. Under "entered" display units the value is the stored value,
// exact; under SI or US it is the converter's binary64 result, and the unit basis names each
// conversion's stored unit, display unit and factor so a reader can reverse it.
import type { DisplayQuantity, DisplaySourceQuantity, DisplayUnitPreference } from "../display-units";
import type { DisplayQuantityRequest, DisplayQuantityResult } from "../../services/displayQuantityService";
import type { CsvExport } from "../result-export/CsvExportControl";
import { csvExportFileName, csvNumber, csvStatus, csvText, slugToken, type CsvCell, type CsvProvenance } from "../result-export/csvTable";
import { CSV_DIMENSIONLESS_UNIT, CSV_NOT_FINITE, joinColumnGroups, quantityColumns, storedValueCell, textColumn } from "../result-export/csvValues";

/** One row as the panel renders it. */
export type ResultRowsCsvRow = Readonly<{
  id: string;
  category: string;
  label: string;
  health: string;
  withheld: string;
  componentBody: string;
  entity: string;
  location: string;
  frame: string;
  quantity: DisplaySourceQuantity;
  /** The readout the value cell shows (the session's display conversion, settled). */
  display: DisplayQuantity;
}>;

const shownUnit = (unit: string) => (unit === "" ? CSV_DIMENSIONLESS_UNIT : unit);

/** The value cell: the readout's binary64 value, as its shortest round-trip decimal. An
 * unconverted readout is the stored value. A value that is not a finite number is a status. */
export function displayValueCell(display: DisplayQuantity): CsvCell {
  return typeof display.value === "number" && Number.isFinite(display.value) ? csvNumber(display.value) : csvStatus(CSV_NOT_FINITE);
}

function statusText(display: DisplayQuantity): string {
  return display.notice ? `${display.status}: ${display.notice}` : display.status;
}

function valueColumns(rows: readonly { display: DisplayQuantity; quantity: DisplaySourceQuantity }[]) {
  const display = rows.map((row) => row.display);
  const units = new Set(display.map((item) => shownUnit(item.unit)));
  const value = units.size === 1
    ? { columns: [{ kind: "number" as const, label: "Value", unit: [...units][0] }], cells: display.map((item) => [displayValueCell(item)]) }
    : {
        columns: [{ kind: "number" as const, label: "Value", unitColumn: "Value unit" }, { kind: "unit" as const, label: "Value unit" }],
        cells: display.map((item) => [displayValueCell(item), csvText(shownUnit(item.unit))])
      };
  return [
    value,
    textColumn("Display", display.map(statusText)),
    quantityColumns("Entered value", rows.map((row) => (typeof row.quantity.value === "number" ? { value: row.quantity.value, unit: row.quantity.unit } : { absent: CSV_NOT_FINITE })), (stored) => storedValueCell(stored), shownUnit)
  ];
}

type Context = Readonly<{ provenance: CsvProvenance; softwareVersion: string; unitBasis: string; caseOrFilter: string; caseQualifier?: string }>;

/** The group's kind: `results-<group>` (DEL-07-01 `export-file-names.md`). */
export function resultGroupCsvKind(title: string): string {
  return `results-${slugToken(title)}`;
}

/** One group's table: every row the filters admit, in the panel's order (all pages). */
export function resultGroupCsv(input: Context & { title: string; rows: readonly ResultRowsCsvRow[] }): CsvExport {
  const { rows } = input;
  const { columns, rows: cells } = joinColumnGroups(rows.length, [
    textColumn("Result id", rows.map((row) => row.id)),
    textColumn("Category", rows.map((row) => row.category)),
    textColumn("Row label", rows.map((row) => row.label)),
    textColumn("Numerical health", rows.map((row) => row.health)),
    textColumn("Withheld", rows.map((row) => row.withheld)),
    textColumn("Component body", rows.map((row) => row.componentBody)),
    textColumn("Entity", rows.map((row) => row.entity)),
    textColumn("Location", rows.map((row) => row.location)),
    textColumn("Frame", rows.map((row) => row.frame)),
    ...valueColumns(rows)
  ]);
  const kind = resultGroupCsvKind(input.title);
  return {
    table: {
      preamble: { table: kind, softwareVersion: input.softwareVersion, ...input.provenance, caseOrFilter: input.caseOrFilter, unitBasis: input.unitBasis },
      columns, rows: cells
    },
    fileName: csvExportFileName(kind, input.caseQualifier)
  };
}

export const ENDPOINT_PAIR_CSV_KIND = "results-endpoint-pair";

/** The endpoint pair: each end's location, result row and value; a row the result does not hold
 * states the panel's own label for it. */
export function endpointPairCsv(input: Context & {
  rows: readonly Readonly<{ location: string; resultId: string; fallback: string; quantity: DisplaySourceQuantity | null; display: DisplayQuantity | null }>[];
}): CsvExport {
  const held = input.rows.map((row) => ({
    display: row.display ?? { value: Number.NaN, unit: "", status: "unavailable" as const },
    quantity: row.quantity ?? { value: Number.NaN, unit: "", dimension_id: "unknown" }
  }));
  const [value, display, entered] = valueColumns(held);
  const absentRows = input.rows.map((row) => row.display === null);
  const replaceAbsent = (group: { columns: unknown[]; cells: CsvCell[][] }, reason: (index: number) => string) => ({
    columns: group.columns as never,
    cells: group.cells.map((cells, index) => (absentRows[index] ? cells.map(() => csvStatus(reason(index))) : cells))
  });
  const { columns, rows } = joinColumnGroups(input.rows.length, [
    textColumn("Location", input.rows.map((row) => row.location)),
    textColumn("Result id", input.rows.map((row) => row.resultId)),
    replaceAbsent(value, (index) => input.rows[index].fallback),
    { columns: display.columns, cells: display.cells.map((cells, index) => (absentRows[index] ? [csvText("no result row")] : cells)) },
    replaceAbsent(entered, (index) => input.rows[index].fallback)
  ]);
  return {
    table: {
      preamble: { table: ENDPOINT_PAIR_CSV_KIND, softwareVersion: input.softwareVersion, ...input.provenance, caseOrFilter: input.caseOrFilter, unitBasis: input.unitBasis },
      columns, rows
    },
    fileName: csvExportFileName(ENDPOINT_PAIR_CSV_KIND)
  };
}

/** The unit basis of these tables, in one deterministic text:
 * - `display units entered; values as stored, exact`; or
 * - `display units <SI|US>; converted in binary64 by the session's display converter (core/units:
 *   stored unit to its canonical unit, then to the display unit); each formula below is
 *   approximate, from the converter's outputs for 0 and 1 stored unit;
 *   <dimension> <stored>-><display> approximately v*<factor>[+<offset>]; ...;
 *   as entered: <dimension> <unit>, ...`
 * The converter's unit definitions (`core/units` transforms) are not exposed to the webview,
 * so the formula is stated as approximate: applied in binary64 it is close to the CSV value but
 * need not reproduce it bit for bit. No converted value is called exact. Conversions are listed
 * by dimension, stored unit and display unit, each once. */
export async function displayConversionBasis(input: {
  preference: DisplayUnitPreference;
  rows: readonly { quantity: DisplaySourceQuantity; display: DisplayQuantity }[];
  convertRaw: (request: Omit<DisplayQuantityRequest, "id">) => Promise<DisplayQuantityResult | null>;
}): Promise<string> {
  if (input.preference === "entered") return "display units entered; values as stored, exact";
  const converted = new Map<string, { dimension: string; from: string; to: string }>();
  const entered = new Set<string>();
  for (const row of input.rows) {
    if (row.display.status === "converted") {
      const item = { dimension: row.quantity.dimension_id, from: row.quantity.unit, to: row.display.unit };
      converted.set(JSON.stringify([item.dimension, item.from, item.to]), item);
    } else entered.add(`${row.quantity.dimension_id} ${shownUnit(row.quantity.unit)}`);
  }
  const parts: string[] = [];
  for (const key of [...converted.keys()].sort()) {
    const item = converted.get(key)!;
    const formula = await approximateFormula(item, input.convertRaw);
    parts.push(`${item.dimension} ${item.from}->${item.to} ${formula ? `approximately ${formulaText(formula)}` : "formula not_recorded"}`);
  }
  const head = `display units ${input.preference}; converted in binary64 by the session's display converter (core/units: stored unit to its canonical unit, then to the display unit); each formula below is approximate, from the converter's outputs for 0 and 1 stored unit`;
  const tail = entered.size > 0 ? [`as entered: ${[...entered].sort().join(", ")}`] : [];
  return [head, ...parts, ...tail].join("; ");
}

/** An approximate formula `v*factor+offset`: the converter's output for 0 is the offset, and
 * its output for 1 less that is the factor. */
export type ApproximateFormula = Readonly<{ factor: number; offset: number }>;

export async function approximateFormula(
  item: Readonly<{ dimension: string; from: string; to: string }>,
  convertRaw: (request: Omit<DisplayQuantityRequest, "id">) => Promise<DisplayQuantityResult | null>
): Promise<ApproximateFormula | null> {
  const at = async (value: number) => {
    const result = await convertRaw({ value, from_unit: item.from, to_unit: item.to, dimension_id: item.dimension });
    return result && result.status === "converted" && Number.isFinite(result.value) ? result.value : null;
  };
  const zero = await at(0);
  const one = await at(1);
  return zero === null || one === null ? null : { factor: one - zero, offset: zero === 0 ? 0 : zero };
}

export function formulaText(formula: ApproximateFormula): string {
  return `v*${String(formula.factor)}${formula.offset === 0 ? "" : `${formula.offset < 0 ? "" : "+"}${String(formula.offset)}`}`;
}

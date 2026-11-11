// RT-R6 (DEL-08-04-R14; Design `csv-export.md` §3): the cells a table adapter writes for its
// numeric columns. A value is the display rule's exact step (`exactDisplayQuantity`), the same
// conversion the panel's `displayQuantityParts` rounds for the screen, so CSV and screen never
// disagree beyond display rounding. It is written as the shortest binary64 decimal when that
// reads back to the exact decimal, and otherwise as the exact decimal text itself.
import {
  canonicalDecimal,
  exactDecimalText,
  exactDisplayQuantity,
  type ExactDisplayQuantity
} from "../report/reportQuantityDisplay";
import {
  csvDecimal,
  csvNumber,
  csvStatus,
  csvText,
  type CsvCell,
  type CsvColumn
} from "./csvTable";

/** The unit label of a dimensionless value (the catalog symbol, `DISPLAY_TARGETS`). */
export const CSV_DIMENSIONLESS_UNIT = "1";

/** The display unit the panel shows for a stored unit; "" (dimensionless) is labelled "1". */
export function csvDisplayUnit(unit: string): string {
  const shown = exactDisplayQuantity(0, unit).unit;
  return shown === "" ? CSV_DIMENSIONLESS_UNIT : shown;
}

/** The numeric cell of an exact decimal: a binary64 when one reads back exactly, else the
 * exact decimal text. */
export function exactDecimalCell(exact: ExactDisplayQuantity): CsvCell {
  if (exact.digits === "") return csvNumber(exact.sign === "-" ? -0 : 0);
  const text = exactDecimalText(exact);
  const parsed = Number(text);
  if (Number.isFinite(parsed)) {
    const back = canonicalDecimal(String(parsed), 0);
    if (back.sign === exact.sign && back.digits === exact.digits && back.point === exact.point) return csvNumber(parsed);
  }
  return csvDecimal(text);
}

/** A panel's text in place of a value the display rule cannot show (not a finite number). */
export const CSV_NOT_FINITE = "outside the representable range";

/** The numeric cell of a stored value in its display unit (the panel's conversion, exact). A
 * non-finite value is never written as a number: it is a status cell. */
export function exactValueCell(value: number, unit: string): CsvCell {
  if (!Number.isFinite(value)) return csvStatus(CSV_NOT_FINITE);
  return exactDecimalCell(exactDisplayQuantity(value, unit));
}

/** A stored value written as stored (a view that converts nothing, such as the comparisons). */
export function storedValueCell(value: number): CsvCell {
  if (!Number.isFinite(value)) return csvStatus(CSV_NOT_FINITE);
  return exactDecimalCell({ ...canonicalDecimal(Object.is(value, -0) ? "-0" : String(value), 0), unit: "" });
}

/** One numeric column's entry for a row: a value with its stored unit, or the panel's own
 * reason text for an absent value. */
export type CsvQuantityEntry = Readonly<{ value: number; unit: string }> | Readonly<{ absent: string }>;

const present = (entry: CsvQuantityEntry): entry is Readonly<{ value: number; unit: string }> => "value" in entry;

/** The columns and per-row cells of one numeric quantity. When every present value shares one
 * display unit, the header states it (`UX [mm]`); otherwise a per-row unit column follows
 * (`UX`, `UX unit`). `convert` is the panel's conversion: the display rule's exact step by
 * default, or `storedValueCell` for a view that shows stored values. */
export function quantityColumns(
  label: string,
  entries: readonly CsvQuantityEntry[],
  convert: (value: number, unit: string) => CsvCell = exactValueCell,
  unitOf: (unit: string) => string = csvDisplayUnit
): { columns: CsvColumn[]; cells: CsvCell[][] } {
  return quantityColumnsIn(label, entries, quantityHeaderUnit(new Set(entries.filter(present).map((entry) => unitOf(entry.unit)))), convert, unitOf);
}

/** The header unit of a numeric column whose present values have these display units: the one
 * unit when there is exactly one, else null (a per-row unit column follows). */
export function quantityHeaderUnit(units: ReadonlySet<string>): string | null {
  return units.size === 1 ? [...units][0] : null;
}

/** `quantityColumns` with its header unit already decided (`quantityHeaderUnit` over every row of
 * the table), so a table written in parts states the same columns and cells for each part. */
export function quantityColumnsIn(
  label: string,
  entries: readonly CsvQuantityEntry[],
  headerUnit: string | null,
  convert: (value: number, unit: string) => CsvCell = exactValueCell,
  unitOf: (unit: string) => string = csvDisplayUnit
): { columns: CsvColumn[]; cells: CsvCell[][] } {
  const cell = (entry: CsvQuantityEntry): CsvCell => (present(entry) ? convert(entry.value, entry.unit) : csvStatus(entry.absent));
  if (headerUnit !== null) {
    return { columns: [{ kind: "number", label, unit: headerUnit }], cells: entries.map((entry) => [cell(entry)]) };
  }
  const unitLabel = `${label} unit`;
  return {
    columns: [{ kind: "number", label, unitColumn: unitLabel }, { kind: "unit", label: unitLabel }],
    // An absent value's unit cell carries the same stated reason (a unit cell is never blank).
    cells: entries.map((entry) => [cell(entry), present(entry) ? csvText(unitOf(entry.unit)) : csvStatus(entry.absent)])
  };
}

/** Column groups side by side: the columns in order, and each row's cells joined. */
export function joinColumnGroups(
  rowCount: number,
  groups: readonly { columns: readonly CsvColumn[]; cells: readonly (readonly CsvCell[])[] }[]
): { columns: CsvColumn[]; rows: CsvCell[][] } {
  const columns = groups.flatMap((group) => group.columns);
  const rows = Array.from({ length: rowCount }, (_, index) => groups.flatMap((group) => group.cells[index] ?? []));
  return { columns, rows };
}

/** A text column group from one text per row. */
export function textColumn(label: string, texts: readonly string[]): { columns: CsvColumn[]; cells: CsvCell[][] } {
  return { columns: [{ kind: "text", label }], cells: texts.map((text) => [csvText(text)]) };
}

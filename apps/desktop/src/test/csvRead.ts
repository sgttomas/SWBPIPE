// RT-R6 test support: an RFC 4180 reader for the CSV the serializer writes, and the exact
// round-trip checks of a numeric field (DEL-08-04 AC-002).
import { expect } from "vitest";
import { canonicalDecimal, exactDecimalText, exactDisplayQuantity, roundedDisplayText, displayQuantityParts } from "../features/report/reportQuantityDisplay";
import { CSV_BOM } from "../features/result-export/csvTable";

export type ReadCsv = {
  preamble: Array<[string, string]>;
  header: string[];
  rows: string[][];
};

export function parseCsvRows(text: string): string[][] {
  expect(text.startsWith(CSV_BOM)).toBe(true);
  expect(text.endsWith("\r\n")).toBe(true);
  const body = text.slice(1);
  const rows: string[][] = [];
  let row: string[] = [];
  let field = "";
  let quoted = false;
  for (let i = 0; i < body.length; i += 1) {
    const c = body[i];
    if (quoted) {
      if (c === '"' && body[i + 1] === '"') { field += '"'; i += 1; }
      else if (c === '"') quoted = false;
      else field += c;
    } else if (c === '"' && field === "") quoted = true;
    else if (c === ",") { row.push(field); field = ""; }
    else if (c === "\r" && body[i + 1] === "\n") { row.push(field); rows.push(row); row = []; field = ""; i += 1; }
    else field += c;
  }
  return rows;
}

/** The preamble rows, the header and the table rows. */
export function readCsv(text: string): ReadCsv {
  const rows = parseCsvRows(text);
  const blank = rows.findIndex((row) => row.length === 1 && row[0] === "");
  expect(blank).toBeGreaterThan(0);
  return { preamble: rows.slice(0, blank).map((row) => [row[0], row[1]] as [string, string]), header: rows[blank + 1], rows: rows.slice(blank + 2) };
}

export function preambleValue(csv: ReadCsv, key: string): string | undefined {
  return csv.preamble.find(([name]) => name === key)?.[1];
}

/** A numeric column's field and unit in one row: `label [unit]`, or `label` with `label unit`. */
export function numericField(csv: ReadCsv, row: string[], label: string): { field: string; unit: string } {
  const fixed = csv.header.findIndex((name) => name.startsWith(`${label} [`) && name.endsWith("]") && name.slice(label.length + 2, -1).indexOf("[") < 0);
  if (fixed >= 0) return { field: row[fixed], unit: csv.header[fixed].slice(label.length + 2, -1) };
  const at = csv.header.indexOf(label);
  const unitAt = csv.header.indexOf(`${label} unit`);
  expect(at, `column ${label}`).toBeGreaterThanOrEqual(0);
  expect(unitAt, `unit column of ${label}`).toBeGreaterThanOrEqual(0);
  return { field: row[at], unit: row[unitAt] };
}

export function textField(csv: ReadCsv, row: string[], label: string): string {
  const at = csv.header.indexOf(label);
  expect(at, `column ${label}`).toBeGreaterThanOrEqual(0);
  return row[at];
}

/** The field is exactly the display rule's exact step of the stored value, in the panel's
 * display unit: as a decimal, digit for digit, and `Number(field) === Number(exact)`. The
 * screen's rounded text is the rounding step of the same decimal. */
export function expectExactDisplayField(got: { field: string; unit: string }, value: number, unit: string): void {
  const exact = exactDisplayQuantity(value, unit);
  const shownUnit = exact.unit === "" ? "1" : exact.unit;
  expect(got.unit, `${value} ${unit}`).toBe(shownUnit);
  expect(got.field, `${value} ${unit}`).not.toBe("");
  const parsed = canonicalDecimal(got.field, 0);
  expect({ ...parsed, sign: parsed.digits === "" && got.field.startsWith("-") ? "-" : parsed.sign }, `${value} ${unit}`)
    .toEqual({ sign: exact.sign, digits: exact.digits, point: exact.digits === "" ? 0 : exact.point });
  expect(Object.is(Number(got.field), Number(exactDecimalText(exact))), `${got.field} vs ${exactDecimalText(exact)}`).toBe(true);
  expect(roundedDisplayText(parsed)).toBe(displayQuantityParts(value, unit).number);
}

/** The field is the stored value itself (a view that converts nothing). */
export function expectStoredField(got: { field: string; unit: string }, value: number, unit: string): void {
  expect(got.unit).toBe(unit === "" ? "1" : unit);
  expect(Object.is(Number(got.field), value), `${got.field} vs ${value}`).toBe(true);
}

/** No numeric field is empty, and none is a token or text that would read as a number by accident. */
export function expectNoEmptyNumericField(csv: ReadCsv): void {
  csv.header.forEach((name, index) => {
    if (/\[[^\]]+\]$/.test(name)) for (const row of csv.rows) expect(row[index], name).not.toBe("");
  });
}

// RT-R6 CSV serializer (DEL-08-04-R14; Design `csv-export.md` §3). One pure function turns
// `{preamble, columns, rows}` into the file's text; every results table goes through it.
// It converts no units and rounds nothing: an adapter passes the value already returned by
// the table's own conversion function for the column's displayed unit.

/** The `notice` preamble value: the registry's BS-VALID canonical text, verbatim
 * (`docs/claims_registry.md` §1). GEN-13 pins this declaration as an anchor
 * (`tools/validation/validate_claims_language.py`), so a drift here fails the check. */
export const CSV_PREAMBLE_NOTICE = "Candidate designs are validated in the user's accepted professional tools (external-prover correlation, PRD §22.5). Internal benchmarks and rule checks are development verification and screening evidence.";

/** The fixed preamble keys, in file order (column A). Machine keys only. */
export const CSV_PREAMBLE_KEYS = [
  "notice",
  "export_context",
  "released_report",
  "table",
  "software_version",
  "model_hash",
  "result_hash",
  "analysis_route",
  "case_or_filter",
  "unit_basis",
  "rule_pack"
] as const;

/** A two-sided table's keys in place of `model_hash`, `result_hash` and `analysis_route`, in
 * file order. `a` and `b` follow the view's own sides (Design `csv-export.md` §3). */
export const CSV_PAIR_PROVENANCE_KEYS = [
  "model_hash_a",
  "model_hash_b",
  "result_hash_a",
  "result_hash_b",
  "analysis_route_a",
  "analysis_route_b"
] as const;

/** A result exists but carries no recorded value for the key. */
export const CSV_NOT_RECORDED = "not_recorded";
/** There is no result (a model-only or state table), so the key has no value. */
export const CSV_NOT_APPLICABLE = "not_applicable";
export const CSV_PROVENANCE_TOKENS: readonly string[] = [CSV_NOT_RECORDED, CSV_NOT_APPLICABLE];
const HASH = /^sha256:[0-9a-f]{64}$/;

/** Whether a preamble hash value is a hash: `sha256:` and 64 lowercase hex digits. The tokens
 * are not hex and not `sha256:`-shaped, so they never pass. */
export function isCsvHashValue(value: unknown): value is string {
  return typeof value === "string" && HASH.test(value);
}

/** UTF-8 byte-order mark, written first. */
export const CSV_BOM = "﻿";
/** Every row, including the last, ends with CRLF. */
export const CSV_ROW_END = "\r\n";

/** A text cell: labels, identifiers, units, rule outcome wording. Always formula-guarded. */
export type CsvTextCell = { readonly kind: "text"; readonly text: string };
/** A finite number in the column's displayed unit, unrounded. Never formula-guarded. */
export type CsvNumberCell = { readonly kind: "number"; readonly value: number };
/** An absent value: the table's stated reason ("withheld", "No result", ...). Never blank. */
export type CsvStatusCell = { readonly kind: "status"; readonly reason: string };
/** An exact decimal that no binary64 reads back exactly, written as its own decimal text in a
 * numeric column (the display rule's exact step, `exactDisplayQuantity`). Never guarded. */
export type CsvDecimalCell = { readonly kind: "decimal"; readonly text: string };
export type CsvCell = CsvTextCell | CsvNumberCell | CsvStatusCell | CsvDecimalCell;

export const csvText = (text: string): CsvTextCell => ({ kind: "text", text });
export const csvNumber = (value: number): CsvNumberCell => ({ kind: "number", value });
export const csvStatus = (reason: string): CsvStatusCell => ({ kind: "status", reason });
export const csvDecimal = (text: string): CsvDecimalCell => ({ kind: "decimal", text });
const DECIMAL_TEXT = /^-?\d+(?:\.\d+)?(?:e[+-]\d+)?$/;

/** A text column (identifiers, labels, status, rule outcomes). */
export type CsvTextColumn = { readonly kind: "text"; readonly label: string };
/** A per-row unit column, for a numeric column whose rows mix kinds. */
export type CsvUnitColumn = { readonly kind: "unit"; readonly label: string };
/** A numeric column. Its unit is either fixed and written in the header (`UX [mm]`), or
 * carried per row by the named unit column of the same table. Exactly one is given. */
export type CsvNumberColumn =
  | { readonly kind: "number"; readonly label: string; readonly unit: string; readonly unitColumn?: undefined }
  | { readonly kind: "number"; readonly label: string; readonly unitColumn: string; readonly unit?: undefined };
export type CsvColumn = CsvTextColumn | CsvUnitColumn | CsvNumberColumn;

/** A rule pack, by identifier, version and checksum only (R8). */
export type CsvRulePackRef = { readonly id: string; readonly version: string; readonly checksum: string };

/** One side's provenance. A hash is `sha256:<hex>`, or `not_recorded` (a result with no
 * recorded hash) or `not_applicable` (no result). The analysis route is the result's
 * `producer.semantic_contract_id`, with the same tokens. */
export type CsvProvenance = {
  readonly modelHash: string;
  readonly resultHash: string;
  /** One or more analysis route identifiers, written in one row, space-separated, in order;
   * or one token alone. */
  readonly analysisRoutes: readonly string[];
};

/** The provenance preamble. `export_context` and `released_report` are fixed by the
 * serializer (`local_private`, `false`), and `notice` is `CSV_PREAMBLE_NOTICE`. Every value
 * is required and non-empty: missing metadata is stated by a token, never silently filled
 * (R11). A one-sided table states `model_hash`, `result_hash` and `analysis_route`; a
 * two-sided table (`sides`) states the `_a` and `_b` keys of `CSV_PAIR_PROVENANCE_KEYS`. */
export type CsvPreamble = {
  /** The DEL-07-01 `export-file-names.md` CSV kind slug, e.g. `stress-ratio`. */
  readonly table: string;
  readonly softwareVersion: string;
  readonly caseOrFilter: string;
  readonly unitBasis: string;
  /** Only for tables with rule outcomes: one `rule_pack` row each, `<id> <version> <checksum>`. */
  readonly rulePacks?: readonly CsvRulePackRef[];
} & (
  | (CsvProvenance & { readonly sides?: undefined })
  | { readonly sides: { readonly a: CsvProvenance; readonly b: CsvProvenance } }
);

export type CsvTable = {
  readonly preamble: CsvPreamble;
  readonly columns: readonly CsvColumn[];
  /** Rows in the table's own order; the serializer never reorders them. */
  readonly rows: readonly (readonly CsvCell[])[];
};

export class CsvSerializationError extends Error {
  constructor(readonly code: string, message: string) {
    super(`${code}: ${message}`);
    this.name = "CsvSerializationError";
  }
}

const TOKEN = "[a-z0-9]+";
/** The CSV kinds of DEL-07-01 `export-file-names.md`. `results-<group>` takes one or more
 * group tokens. */
const CSV_KIND_SOURCE =
  `results(?:-${TOKEN})+|displacement-extremes|stress-ratio|support-loads-all|support-loads|weight-cog|weight|run-comparison|state-comparison`;
const CSV_KIND = new RegExp(`^(?:${CSV_KIND_SOURCE})$`);
/** `swbpipe-<csv kind>[-<qualifier>].csv` under the token rule. Kinds whose longer form adds
 * tokens (`support-loads-all`, `weight-cog`) are matched by the shorter kind plus tokens. */
export const CSV_EXPORT_FILE_NAME =
  /^swbpipe-(?:results(?:-[a-z0-9]+)+|displacement-extremes|stress-ratio|support-loads|weight|run-comparison|state-comparison)(?:-[a-z0-9]+)*\.csv$/;

export function isCsvKind(kind: string): boolean {
  return CSV_KIND.test(kind);
}

export function isCsvExportFileName(name: string): boolean {
  return CSV_EXPORT_FILE_NAME.test(name);
}

/** Slug a label to `export-file-names.md` tokens: lowercase ASCII letters and digits joined
 * by single hyphens. Returns "" when nothing remains; the caller then falls back to the
 * stable key the label stands for. */
export function slugToken(label: string): string {
  return label.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "");
}

/** The suggested name `swbpipe-<kind>[-<qualifier>].csv`. The qualifier is slugged; a
 * qualifier that slugs to nothing is refused, so the caller passes the stable key instead. */
export function csvExportFileName(kind: string, qualifier?: string): string {
  if (!isCsvKind(kind)) throw new CsvSerializationError("CSV_KIND_INVALID", "not a CSV kind of export-file-names.md");
  let name = `swbpipe-${kind}`;
  if (qualifier !== undefined) {
    const slug = slugToken(qualifier);
    if (!slug) throw new CsvSerializationError("CSV_QUALIFIER_EMPTY", "the qualifier slugs to nothing; pass its stable key");
    name += `-${slug}`;
  }
  return `${name}.csv`;
}

// The ASCII starters, LF, and their full-width forms (U+FF1D, U+FF0B, U+FF0D, U+FF20), which some
// spreadsheet locales also read as a formula start.
const FORMULA_LEADING = new Set(["=", "+", "-", "@", "\t", "\r", "\n", "\uFF1D", "\uFF0B", "\uFF0D", "\uFF20"]);
const LONE_SURROGATE = /[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?<![\uD800-\uDBFF])[\uDC00-\uDFFF]/;

/** Formula guard: a text that a spreadsheet would read as a formula gets a leading `'`. */
export function neutralizeCsvText(text: string): string {
  return text.length > 0 && FORMULA_LEADING.has(text[0]) ? `'${text}` : text;
}

/** RFC 4180: a field with a comma, a double quote, CR or LF is quoted, quotes doubled. */
function quoteField(field: string): string {
  return /[",\r\n]/.test(field) ? `"${field.replace(/"/g, '""')}"` : field;
}

function textField(text: unknown, where: string): string {
  if (typeof text !== "string") throw new CsvSerializationError("CSV_TEXT_INVALID", `${where} is not text`);
  if (LONE_SURROGATE.test(text)) throw new CsvSerializationError("CSV_TEXT_NOT_UTF8", `${where} is not encodable as UTF-8`);
  return quoteField(neutralizeCsvText(text));
}

/** The shortest decimal that parses back to exactly `value`.
 *
 * ECMAScript `Number.prototype.toString` (via `String()`) already yields the shortest digit
 * string that round-trips (ECMA-262 Number::toString), with no locale. Its exponent forms
 * (`1e+21`, `1e-7`, `-1.5e-7`) are kept: they are exact and spreadsheets read them as
 * numbers. Negative zero is the one case `String()` loses (`String(-0) === "0"`), so it is
 * written `-0`, which `Number("-0")` reads back as `-0` under `Object.is`. Non-finite values
 * are refused: they must arrive as status cells. */
export function formatCsvNumber(value: number): string {
  if (typeof value !== "number" || !Number.isFinite(value)) {
    throw new CsvSerializationError("CSV_NUMBER_NOT_FINITE", "NaN and ±Infinity are written as status cells, never as numbers");
  }
  return Object.is(value, -0) ? "-0" : String(value);
}

function required(value: unknown, key: string): string {
  if (typeof value !== "string" || value.trim() === "") {
    throw new CsvSerializationError("CSV_PREAMBLE_VALUE_MISSING", `preamble ${key} needs a stated value`);
  }
  return value;
}

function spaceJoined(parts: readonly string[], key: string): string {
  return parts.map((part) => {
    const value = required(part, key);
    if (/\s/.test(value)) throw new CsvSerializationError("CSV_PREAMBLE_VALUE_INVALID", `preamble ${key} parts carry no whitespace`);
    return value;
  }).join(" ");
}

function hashValue(value: unknown, key: string): string {
  const text = required(value, key);
  if (!isCsvHashValue(text) && !CSV_PROVENANCE_TOKENS.includes(text)) {
    throw new CsvSerializationError("CSV_PREAMBLE_VALUE_INVALID", `preamble ${key} is a sha256: hash or a token`);
  }
  return text;
}

function routeValue(routes: unknown, key: string): string {
  if (!Array.isArray(routes) || routes.length === 0) {
    throw new CsvSerializationError("CSV_PREAMBLE_VALUE_MISSING", `preamble ${key} needs one or more routes`);
  }
  if (routes.length > 1 && routes.some((route) => CSV_PROVENANCE_TOKENS.includes(route))) {
    throw new CsvSerializationError("CSV_PREAMBLE_VALUE_INVALID", `preamble ${key} states a token alone`);
  }
  return spaceJoined(routes, key);
}

function provenanceRows(preamble: CsvPreamble): [string, string][] {
  if (preamble.sides === undefined) {
    return [
      ["model_hash", hashValue(preamble.modelHash, "model_hash")],
      ["result_hash", hashValue(preamble.resultHash, "result_hash")],
      ["analysis_route", routeValue(preamble.analysisRoutes, "analysis_route")]
    ];
  }
  const { a, b } = preamble.sides;
  if (!a || !b) throw new CsvSerializationError("CSV_PREAMBLE_VALUE_MISSING", "a two-sided preamble states both sides");
  return [
    ["model_hash_a", hashValue(a.modelHash, "model_hash_a")],
    ["model_hash_b", hashValue(b.modelHash, "model_hash_b")],
    ["result_hash_a", hashValue(a.resultHash, "result_hash_a")],
    ["result_hash_b", hashValue(b.resultHash, "result_hash_b")],
    ["analysis_route_a", routeValue(a.analysisRoutes, "analysis_route_a")],
    ["analysis_route_b", routeValue(b.analysisRoutes, "analysis_route_b")]
  ];
}

function preambleRows(preamble: CsvPreamble): [string, string][] {
  if (!isCsvKind(required(preamble.table, "table"))) {
    throw new CsvSerializationError("CSV_KIND_INVALID", "preamble table is not a CSV kind of export-file-names.md");
  }
  const rows: [string, string][] = [
    ["notice", CSV_PREAMBLE_NOTICE],
    ["export_context", "local_private"],
    ["released_report", "false"],
    ["table", preamble.table],
    ["software_version", required(preamble.softwareVersion, "software_version")],
    ...provenanceRows(preamble),
    ["case_or_filter", required(preamble.caseOrFilter, "case_or_filter")],
    ["unit_basis", required(preamble.unitBasis, "unit_basis")]
  ];
  for (const pack of preamble.rulePacks ?? []) {
    rows.push(["rule_pack", spaceJoined([pack.id, pack.version, pack.checksum], "rule_pack")]);
  }
  return rows;
}

function headerLabel(column: CsvColumn, labels: ReadonlyMap<string, CsvColumn>): string {
  const label = required(column.label, "column label");
  if (column.kind !== "number") return label;
  const fixed = column.unit !== undefined;
  const perRow = column.unitColumn !== undefined;
  if (fixed === perRow) throw new CsvSerializationError("CSV_COLUMN_UNIT_INVALID", `${label} states exactly one of unit or unitColumn`);
  if (fixed) return `${label} [${required(column.unit, `${label} unit`)}]`;
  if (labels.get(column.unitColumn!)?.kind !== "unit") {
    throw new CsvSerializationError("CSV_COLUMN_UNIT_INVALID", `${label} names no unit column of this table`);
  }
  return label;
}

function cellField(cell: CsvCell, column: CsvColumn, where: string): string {
  switch (cell?.kind) {
    case "number":
      if (column.kind !== "number") throw new CsvSerializationError("CSV_CELL_KIND_INVALID", `${where}: a number belongs in a numeric column`);
      return formatCsvNumber(cell.value);
    case "decimal":
      if (column.kind !== "number") throw new CsvSerializationError("CSV_CELL_KIND_INVALID", `${where}: a number belongs in a numeric column`);
      if (typeof cell.text !== "string" || !DECIMAL_TEXT.test(cell.text)) throw new CsvSerializationError("CSV_DECIMAL_INVALID", `${where} is not decimal text`);
      return cell.text;
    case "status":
      // An absent value carries its stated reason, so no field of a numeric column is empty.
      return textField(required(cell.reason, `${where} status reason`), where);
    case "text":
      if (column.kind === "number") throw new CsvSerializationError("CSV_CELL_KIND_INVALID", `${where}: a numeric column takes numbers or status cells`);
      // A per-row unit is stated, or the row carries a status saying why it is missing (R3, R11).
      if (column.kind === "unit" && (typeof cell.text !== "string" || cell.text.trim() === "")) {
        throw new CsvSerializationError("CSV_UNIT_EMPTY", `${where}: a unit cell states its unit, or is a status cell`);
      }
      return textField(cell.text, where);
    default:
      throw new CsvSerializationError("CSV_CELL_KIND_INVALID", `${where} is not a CSV cell`);
  }
}

/** Serialize one table: BOM, preamble rows, one empty row, the header row, then the rows in
 * the given order. Deterministic: the same input gives the same text, with no timestamp and
 * no locale-dependent formatting. Throws `CsvSerializationError` rather than writing an
 * empty number, a non-finite number or a malformed preamble. */
export function serializeCsvTable(table: CsvTable): string {
  const columns = table.columns;
  if (!Array.isArray(columns) || columns.length === 0) throw new CsvSerializationError("CSV_COLUMNS_MISSING", "a table has columns");
  const labels = new Map<string, CsvColumn>();
  for (const column of columns) {
    if (labels.has(column.label)) throw new CsvSerializationError("CSV_COLUMN_DUPLICATE", "column labels are unique");
    labels.set(column.label, column);
  }
  const lines: string[] = preambleRows(table.preamble).map(([key, value]) => `${key},${textField(value, key)}`);
  lines.push("");
  lines.push(columns.map((column) => textField(headerLabel(column, labels), "header")).join(","));
  table.rows.forEach((row, rowIndex) => {
    if (!Array.isArray(row) || row.length !== columns.length) {
      throw new CsvSerializationError("CSV_ROW_WIDTH", `row ${rowIndex} has ${columns.length} cells`);
    }
    // Every column index, holes included (`map` would skip a hole and leave an empty field).
    const fields: string[] = [];
    for (let index = 0; index < columns.length; index += 1) {
      fields.push(cellField(row[index], columns[index], `row ${rowIndex} column ${index}`));
    }
    lines.push(fields.join(","));
  });
  return CSV_BOM + lines.map((line) => line + CSV_ROW_END).join("");
}

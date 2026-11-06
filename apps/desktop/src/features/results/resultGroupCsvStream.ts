// C1 native custody (DEL-04-05 `c1-native-custody-plan.md` §5; DEL-08-04 Design `csv-export.md`):
// a results group's CSV written in parts, for a result held natively. Its text is exactly
// `serializeCsvTable(resultGroupCsv({...rows}).table)` over every row the panel's filters admit
// in the group, with no row ceiling, while at most one page of rows (and the text of that page)
// is held here at a time.
//
// The group's columns and unit basis depend on every row (one value column with a header unit, or
// a per-row unit column; the conversions listed in `unit_basis`), so the rows are read twice by
// bounded pages: the first pass keeps only the distinct units and one representative row per
// conversion (`conversionBasisKey`), which decide the head; the second writes each page's rows
// under that layout (`resultGroupCsvTable`, the same cell writers `resultGroupCsv` uses). A page
// whose units are not the first pass's (the rows changed between passes) fails rather than write
// a header that does not describe them.
import type { DisplayUnitPreference } from "../display-units";
import type { DisplayQuantityRequest, DisplayQuantityResult } from "../../services/displayQuantityService";
import { serializeCsvHead, serializeCsvRows, type CsvProvenance } from "../result-export/csvTable";
import {
  addResultRowsCsvUnits,
  conversionBasisKey,
  displayConversionBasis,
  emptyResultRowsCsvUnits,
  resultGroupCsvFileName,
  resultGroupCsvPreamble,
  resultGroupCsvTable,
  resultRowsCsvLayout,
  type ResultRowsCsvRow
} from "./resultRowsCsv";

export const RESULT_GROUP_CSV_ROWS_CHANGED = "RESULT-GROUP-CSV-ROWS-CHANGED";

/** The group's rows as the panel renders them, by bounded pages, in the panel's order; called once
 * per pass, each call reading the pages afresh. */
export type ResultGroupCsvPages = () => AsyncIterable<readonly ResultRowsCsvRow[]>;

export type ResultGroupCsvStreamInput = Readonly<{
  title: string;
  provenance: CsvProvenance;
  softwareVersion: string;
  caseOrFilter: string;
  caseQualifier?: string;
  preference: DisplayUnitPreference;
  convertRaw: (request: Omit<DisplayQuantityRequest, "id">) => Promise<DisplayQuantityResult | null>;
  pages: ResultGroupCsvPages;
}>;

export type ResultGroupCsvStream = Readonly<{ fileName: string; chunks: AsyncIterable<string> }>;

function changed(detail: string): Error {
  return new Error(`${RESULT_GROUP_CSV_ROWS_CHANGED}: ${detail}; the CSV was not completed.`);
}

/** The group's file name and its text in parts: the head, then one part per page of rows. */
export function resultGroupCsvStream(input: ResultGroupCsvStreamInput): ResultGroupCsvStream {
  const fileName = resultGroupCsvFileName(input.title, input.caseQualifier);
  async function* chunks(): AsyncGenerator<string> {
    const units = emptyResultRowsCsvUnits();
    const basis = new Map<string, ResultRowsCsvRow>();
    let total = 0;
    for await (const page of input.pages()) {
      addResultRowsCsvUnits(units, page);
      for (const row of page) if (!basis.has(conversionBasisKey(row))) basis.set(conversionBasisKey(row), row);
      total += page.length;
    }
    const layout = resultRowsCsvLayout(units);
    const unitBasis = await displayConversionBasis({ preference: input.preference, rows: [...basis.values()], convertRaw: input.convertRaw });
    const { columns } = resultGroupCsvTable([], layout);
    yield serializeCsvHead(resultGroupCsvPreamble({ ...input, unitBasis }), columns);
    let written = 0;
    for await (const page of input.pages()) {
      const pageUnits = emptyResultRowsCsvUnits();
      addResultRowsCsvUnits(pageUnits, page);
      if ([...pageUnits.display].some((unit) => !units.display.has(unit)) || [...pageUnits.entered].some((unit) => !units.entered.has(unit)) || page.some((row) => !basis.has(conversionBasisKey(row)))) {
        throw changed("a page's units are not those of the first pass");
      }
      const { rows } = resultGroupCsvTable(page, layout);
      if (rows.length > 0) yield serializeCsvRows(columns, rows, written);
      written += page.length;
      if (written > total) throw changed(`more than the ${total} rows of the first pass`);
    }
    if (written !== total) throw changed(`${written} rows, not the ${total} of the first pass`);
  }
  return { fileName, chunks: chunks() };
}

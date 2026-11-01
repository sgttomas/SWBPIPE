// C1 slice 4 (DEL-04-05 `c1-native-custody-plan.md` §5; DEL-08-04 `csv-export.md`): a results
// group's CSV written in parts is byte for byte the CSV of the unchanged formatters over all its
// rows (`serializeCsvTable(resultGroupCsv(...).table)`), for every page size, under entered and
// converted display units, with mixed units, ties, quoting, formula guards, negative zero and
// non-finite values; it holds at most one page of rows and one part of text at a time, and a group
// larger than the retired 200,000-row interim streams completely without being stored anywhere.
import { describe, expect, it } from "vitest";
import type { DisplayQuantity, DisplaySourceQuantity } from "../display-units";
import type { DisplayQuantityRequest, DisplayQuantityResult } from "../../services/displayQuantityService";
import { CSV_NOT_RECORDED, serializeCsvTable } from "../result-export/csvTable";
import { displayConversionBasis, resultGroupCsv, type ResultRowsCsvRow } from "./resultRowsCsv";
import { RESULT_GROUP_CSV_ROWS_CHANGED, resultGroupCsvStream, type ResultGroupCsvStreamInput } from "./resultGroupCsvStream";
import { createHash } from "node:crypto";

const PROVENANCE = { modelHash: CSV_NOT_RECORDED, resultHash: CSV_NOT_RECORDED, analysisRoutes: ["openpipestress.result_semantics/0.3.0/preview-physics-2"] };

/** A deterministic converter (the same function on both sides of each comparison). */
const FACTORS: Record<string, { to: string; factor: number }> = { mm: { to: "in", factor: 1 / 25.4 }, N: { to: "lbf", factor: 0.224808943 }, Pa: { to: "psi", factor: 0.000145037738 } };
async function convertRaw(request: Omit<DisplayQuantityRequest, "id">): Promise<DisplayQuantityResult | null> {
  const factor = FACTORS[request.from_unit];
  return factor && factor.to === request.to_unit ? { status: "converted", value: request.value * factor.factor, unit: request.to_unit } as unknown as DisplayQuantityResult : null;
}
function display(quantity: DisplaySourceQuantity, converted: boolean): DisplayQuantity {
  const factor = FACTORS[quantity.unit];
  if (converted && factor && typeof quantity.value === "number" && Number.isFinite(quantity.value)) {
    return { value: quantity.value * factor.factor, unit: factor.to, status: "converted" } as DisplayQuantity;
  }
  return { value: quantity.value, unit: quantity.unit, status: "entered", ...(typeof quantity.value === "number" && Number.isFinite(quantity.value) ? {} : { notice: "not a finite number" }) } as DisplayQuantity;
}

/** Row `i` of an invented group: a few entities and units, quoted and formula-like texts, ties,
 * negative zero and a non-finite value at fixed positions. */
function row(i: number, converted: boolean, mixed: boolean): ResultRowsCsvRow {
  const units = mixed ? ["mm", "N", "Pa", ""] : ["mm"];
  const unit = units[i % units.length];
  const value = i % 97 === 5 ? -0 : i % 211 === 7 ? Number.NaN : i % 13 === 0 ? 1.5 : (i * 0.1) - 3;
  const quantity: DisplaySourceQuantity = { value, unit, dimension_id: unit === "mm" ? "length" : unit === "N" ? "force" : unit === "Pa" ? "stress" : "dimensionless" };
  return {
    id: i % 50 === 3 ? `r,"${i}"` : `result:${i}`,
    category: "physical_quantity",
    label: i % 17 === 0 ? `=SUM(A${i})` : i % 19 === 0 ? "-lead" : "",
    health: i % 5 === 0 ? "checks passed" : "",
    withheld: i % 23 === 0 ? "withheld: a stated reason" : "",
    componentBody: "",
    entity: `node:${i % 7}`,
    location: i % 2 ? "end_i" : "end_j",
    frame: "Global",
    quantity,
    display: display(quantity, converted)
  };
}

async function oracle(rows: readonly ResultRowsCsvRow[], converted: boolean, caseQualifier?: string): Promise<{ text: string; fileName: string }> {
  const unitBasis = await displayConversionBasis({ preference: converted ? "si" : "entered", rows, convertRaw });
  const built = resultGroupCsv({ title: "Displacement", rows, provenance: PROVENANCE, softwareVersion: "0.1.0", unitBasis, caseOrFilter: "group: Displacement; c", caseQualifier });
  return { text: serializeCsvTable(built.table), fileName: built.fileName };
}

function input(pages: ResultGroupCsvStreamInput["pages"], converted: boolean, caseQualifier?: string): ResultGroupCsvStreamInput {
  return { title: "Displacement", provenance: PROVENANCE, softwareVersion: "0.1.0", caseOrFilter: "group: Displacement; c", caseQualifier, preference: converted ? "si" : "entered", convertRaw, pages };
}

function paged(rows: readonly ResultRowsCsvRow[], size: number): ResultGroupCsvStreamInput["pages"] {
  return async function* () {
    for (let at = 0; at < rows.length; at += size) yield rows.slice(at, at + size);
  };
}

async function concatenated(stream: AsyncIterable<string>): Promise<string> {
  let text = "";
  for await (const part of stream) text += part;
  return text;
}

describe("a results group's CSV in parts", () => {
  for (const converted of [false, true]) {
    for (const mixed of [false, true]) {
      it(`is the formatters' CSV byte for byte (${converted ? "converted" : "entered"} units, ${mixed ? "mixed" : "one"} unit) at every page size`, async () => {
        const rows = Array.from({ length: 433 }, (_, i) => row(i, converted, mixed));
        const expected = await oracle(rows, converted, "load_case:W");
        for (const size of [1, 7, 50, 432, 433, 1000]) {
          const streamed = resultGroupCsvStream(input(paged(rows, size), converted, "load_case:W"));
          expect(streamed.fileName).toBe(expected.fileName);
          expect(await concatenated(streamed.chunks), `page size ${size}`).toBe(expected.text);
        }
      });
    }
  }

  it("is the formatters' CSV for one row, all non-finite values, and no row", async () => {
    for (const rows of [[row(1, false, false)], [7, 218, 429].map((i) => row(i, true, true)), []]) {
      expect(await concatenated(resultGroupCsvStream(input(paged(rows, 3), true)).chunks)).toBe((await oracle(rows, true)).text);
    }
  });

  it("fails rather than write a header that does not describe a page whose units changed", async () => {
    let pass = 0;
    const pages = async function* () {
      pass += 1;
      yield [row(2, false, false)];
      // The second pass reads a unit the first never saw.
      yield [pass === 1 ? row(4, false, false) : { ...row(4, false, false), display: { ...row(4, false, false).display, unit: "in" } }];
    };
    await expect(concatenated(resultGroupCsvStream(input(pages, false)).chunks)).rejects.toThrow(RESULT_GROUP_CSV_ROWS_CHANGED);
    let rowsPass = 0;
    const fewer = async function* () {
      rowsPass += 1;
      yield [row(2, false, false)];
      if (rowsPass === 1) yield [row(4, false, false)];
    };
    await expect(concatenated(resultGroupCsvStream(input(fewer, false)).chunks)).rejects.toThrow(RESULT_GROUP_CSV_ROWS_CHANGED);
  });

  it("streams 250,001 rows (above the retired 200,000-row interim) holding one page and one part at a time", async () => {
    const total = 250_001;
    const size = 1000;
    let live = 0, maxLive = 0, produced = 0;
    // Pages built on demand and dropped once written; `live` counts the pages produced and not yet
    // consumed, so a writer that kept rows would show more than one outstanding page.
    const pages = async function* () {
      for (let at = 0; at < total; at += size) {
        const page = Array.from({ length: Math.min(size, total - at) }, (_, k) => row(at + k, false, false));
        live += 1;
        maxLive = Math.max(maxLive, live);
        produced += page.length;
        yield page;
        live -= 1;
      }
    };
    const hash = createHash("sha256");
    let parts = 0, maxPart = 0, bytes = 0, lines = 0, headLines = 0, last = "";
    for await (const part of resultGroupCsvStream(input(pages, false)).chunks) {
      parts += 1;
      maxPart = Math.max(maxPart, part.length);
      bytes += part.length;
      const count = part.split("\r\n").length - 1;
      if (parts === 1) headLines = count;
      lines += count;
      last = part;
      hash.update(part);
    }
    expect(produced).toBe(2 * total);
    expect(maxLive).toBe(1);
    expect(parts).toBe(1 + Math.ceil(total / size));
    // Every row was written, the last one last; a part is one page of rows, far below the whole text.
    expect(lines - headLines).toBe(total);
    const final = row(total - 1, false, false);
    expect(last).toBe(`${final.id},${final.category},${final.label},${final.health},${final.withheld},,${final.entity},${final.location},${final.frame},${String(final.quantity.value)},entered,${String(final.quantity.value)}\r\n`);
    expect(maxPart).toBeLessThan(bytes / 200);
    expect(hash.digest("hex")).toMatch(/^[0-9a-f]{64}$/);
  }, 120_000);
});

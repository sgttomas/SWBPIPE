import { describe, expect, it } from "vitest";
import vectors from "../../../../../fixtures/reports/invented/report_composition_vectors.json";
import { composeReportRows, type ReportComposition } from "./reportComposition";
import type { ReportResultRow } from "./reportResultRows";

const rows = vectors.rows as ReportResultRow[];

describe("report composition selection parity with Rust", () => {
  for (const vector of vectors.selection_vectors) {
    it(vector.name, () => {
      const selected = composeReportRows(rows, vector.composition as ReportComposition);
      expect(selected.sections).toEqual(vector.expected_sections);
      expect(selected.shown).toEqual(vector.shown);
      expect(selected.bound_rows).toEqual(vectors.bound_rows);
      expect(selected.disclosure_rows).toEqual(["numerical-health:A"]);
      const shown = selected.sections.flatMap((section) => section.row_ids);
      expect(new Set(shown).size).toEqual(shown.length);
    });
  }
  for (const vector of vectors.refusal_vectors) {
    it(`refuses ${vector.name}`, () => {
      let refusal = "";
      try { composeReportRows(rows, vector.composition as ReportComposition); }
      catch (error) { refusal = String(error); }
      expect(refusal.match(/REPORT-COMPOSITION-[A-Z-]+/g)).toEqual(vector.codes);
    });
  }
  it("does not mutate or filter the complete bound input", () => {
    const before = JSON.stringify(rows);
    composeReportRows(rows, vectors.selection_vectors[0].composition as ReportComposition);
    expect(JSON.stringify(rows)).toEqual(before);
  });
});

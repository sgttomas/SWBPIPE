import { describe, expect, it } from "vitest";
import vectors from "../../../../../fixtures/reports/invented/report_result_rows_parity.json";
import type { MechanicsResult } from "../../types";
import { REPORT_ROW_SECTION_BASIS_UNREADABLE, reportResultRows } from "./reportResultRows";

// The report-row contract (DEL-08-01 Design `report-package-current-results.md`
// §2.3 item 6). The Rust producer's helper is checked against the same vectors by
// `core/reporting/report_package/tests/report_rows_parity.rs`.
describe("report-row contract parity", () => {
  it("matches the shared vectors", () => {
    const source = vectors.source_results as unknown as Pick<MechanicsResult, "results">;
    expect(reportResultRows(source)).toEqual(vectors.expected_rows);
  });

  // W-11 (DEL-08-01 Design `exact-route-report-sections.md` §3.4): the section-basis suffix.
  it("adds the section-basis suffix to exact-route rows and none to the others", () => {
    const source = vectors.exact_route_source_results as unknown as Pick<MechanicsResult, "results">;
    expect(reportResultRows(source)).toEqual(vectors.exact_route_expected_rows);
    expect(JSON.stringify(vectors.source_results)).not.toContain("section_basis=");
  });

  it("refuses another section_basis value or more than one token", () => {
    expect(vectors.section_basis_refused_sources).toHaveLength(2);
    for (const refused of vectors.section_basis_refused_sources) {
      const source = refused as unknown as Pick<MechanicsResult, "results">;
      expect(() => reportResultRows(source)).toThrow(new Error(`${REPORT_ROW_SECTION_BASIS_UNREADABLE}: ${source.results[0].id}`));
    }
  });
});

import { describe, expect, it } from "vitest";
import vectors from "../../../../../fixtures/reports/invented/report_result_rows_parity.json";
import type { MechanicsResult } from "../../types";
import { reportResultRows } from "./reportResultRows";

// The report-row contract (DEL-08-01 Design `report-package-current-results.md`
// §2.3 item 6). The Rust producer's helper is checked against the same vectors by
// `core/reporting/report_package/tests/report_rows_parity.rs`.
describe("report-row contract parity", () => {
  it("matches the shared vectors", () => {
    const source = vectors.source_results as unknown as Pick<MechanicsResult, "results">;
    expect(reportResultRows(source)).toEqual(vectors.expected_rows);
  });
});

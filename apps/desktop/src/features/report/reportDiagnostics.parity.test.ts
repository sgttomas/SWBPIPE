import { describe, expect, it } from "vitest";
import vectors from "../../../../../fixtures/reports/invented/report_diagnostics_parity.json";
import type { MechanicsResult } from "../../types";
import { diagnosticsForSections } from "./renderableReportInput";

// The report's projection of a result's diagnostics (DEL-08-01 Design
// `exact-route-report-sections.md` §3.6, §6.3 item 4). The Rust check's projection is checked
// against the same vectors by `core/reporting/report_package/tests/report_diagnostics_parity.rs`.
// The provenance is the desktop session's and is not part of the projection.
const PROVENANCE = {
  source_name: "invented session",
  source_location: "local desktop session",
  source_license: "user_supplied_or_private",
  contributor: "user_local_session",
  contributor_certification: "not_asserted",
  redistribution_status: "private_only",
  review_status: "pending",
  privacy_classification: "private_project_data"
};

describe("report diagnostics projection parity", () => {
  it("matches the shared vectors", () => {
    expect(vectors.vectors.length).toBeGreaterThanOrEqual(20);
    for (const vector of vectors.vectors) {
      expect(vector.reason.length).toBeGreaterThan(0);
      const result = { diagnostics: vector.diagnostics } as unknown as Pick<MechanicsResult, "diagnostics">;
      const entries = diagnosticsForSections(result, PROVENANCE);
      for (const entry of entries) expect(entry.provenance, vector.reason).toBe(PROVENANCE);
      const projected = entries.map(({ provenance: _provenance, ...rest }) => rest);
      // Compared as JSON, as the report is sent: a key whose value is undefined is absent.
      expect(JSON.parse(JSON.stringify(projected)), vector.reason).toEqual(vector.expected);
    }
  });
});

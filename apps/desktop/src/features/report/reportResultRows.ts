import type { MechanicsResult } from "../../types";

/** One rendered result row (the renderer's `ResultRow`). */
export type ReportResultRow = {
  row_id: string;
  label: string;
  case_ref: string;
  quantity_display: string;
  source_ref: string;
};

/** The report-row contract (DEL-08-01 Design `report-package-current-results.md`
 * §2.3 item 6): one row per source row, in order, with this five-field
 * projection. The report package's Rust producer derives the same rows
 * (`report_package::bindings::expected_report_result_rows`) and requires them
 * equal; both are checked against the shared vectors
 * `fixtures/reports/invented/report_result_rows_parity.json`. A change here
 * changes the contract, the Rust helper and the vectors together. */
export function reportResultRows(result: Pick<MechanicsResult, "results">): ReportResultRow[] {
  return result.results.map((item) => ({
    row_id: `row:${item.id}`,
    label: item.metadata ? `${item.kind} (${item.metadata.component})` : item.kind,
    case_ref: item.basis_ref ? `${item.basis_ref.ref_type}:${item.basis_ref.ref_id}` : "TBD",
    quantity_display: `${item.value} ${item.unit}`,
    source_ref: `result:${item.id}`
  }));
}

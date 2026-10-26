import type { MechanicsResult } from "../../types";

export const REPORT_ROW_SECTION_BASIS_UNREADABLE = "REPORT-ROW-SECTION-BASIS-UNREADABLE";

/** The section-basis suffix (§3.4): `metadata.basis` split on ";", each token trimmed.
 * Exactly one `section_basis=nominal_section` token adds " [nominal section]", exactly one
 * `section_basis=reduced_section` adds " [reduced section]", and no `section_basis=` token
 * adds nothing. Any other value, or more than one such token, refuses. The Rust helper
 * (`report_package::bindings::expected_report_result_rows`) applies the same rule. */
function sectionBasisSuffix(id: string, metadata: { basis?: unknown }): string {
  if (typeof metadata.basis !== "string") return "";
  const tokens = metadata.basis
    .split(";")
    .map((token) => token.trim())
    .filter((token) => token.startsWith("section_basis="))
    .map((token) => token.slice("section_basis=".length));
  if (tokens.length === 0) return "";
  if (tokens.length === 1 && tokens[0] === "nominal_section") return " [nominal section]";
  if (tokens.length === 1 && tokens[0] === "reduced_section") return " [reduced section]";
  throw new Error(`${REPORT_ROW_SECTION_BASIS_UNREADABLE}: ${id}`);
}

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
 * changes the contract, the Rust helper and the vectors together.
 *
 * W-11 (DEL-08-01 Design `exact-route-report-sections.md` §3.4): a row whose
 * `metadata.basis` names its section basis adds it to the label (`sectionBasisSuffix`);
 * no preview-physics-2 row carries the token, so its rows are unchanged. */
export function reportResultRows(result: Pick<MechanicsResult, "results">): ReportResultRow[] {
  return result.results.map((item) => ({
    row_id: `row:${item.id}`,
    label: item.metadata ? `${item.kind} (${item.metadata.component})${sectionBasisSuffix(item.id, item.metadata)}` : item.kind,
    case_ref: item.basis_ref ? `${item.basis_ref.ref_type}:${item.basis_ref.ref_id}` : "TBD",
    quantity_display: `${item.value} ${item.unit}`,
    source_ref: `result:${item.id}`
  }));
}

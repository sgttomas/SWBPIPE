/** RT-A3 §4 (DEL-05-02 Work `combination-certificate.md` §2, the residual text boundary): the one
 * classification of a result diagnostic's message, used by the report's projection
 * (`renderableReportInput.ts` `diagnosticsForSections`, mirrored by Rust
 * `report_package::bindings::expected_report_diagnostics`) and by the Diagnostics panel's list
 * and explanation (`presentedResultDiagnostics`). It reads typed identities and a reader's
 * verdict only, never a message's wording, and derives no standing or certificate fact. */
import type { Diagnostic, MechanicsResult } from "../../types";
import { readerDispatch, readerValidate, type ReaderName } from "./resultReader";

/** The label before a diagnostic message passed through as supplied text. */
export const SUPPLEMENTAL_DIAGNOSTIC_LABEL = "Supplied diagnostic text, unverified (not a combination certificate statement): ";

function recordObject(value: unknown): Record<string, unknown> | null {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? (value as Record<string, unknown>) : null;
}

/** The combination identities a result states (the reader's closed-roster population without the
 * model, which is always within its population with the model): its gates, EX records, certificate
 * and operand records, and the owners of its combination-typed rows and numerical cases, less any
 * identity the result gives another entity (a load case, a row's entity). Rust
 * `result_export::document_combination_identities` is the same population. */
export function documentCombinationIdentities(
  result: Partial<Pick<MechanicsResult, "results" | "contract_evidence" | "numerical_quality">>
): Set<string> {
  const text = (value: unknown): string | null => (typeof value === "string" ? value : null);
  const list = (value: unknown): unknown[] => (Array.isArray(value) ? value : []);
  const field = (value: unknown, key: string): unknown => recordObject(value)?.[key];
  const typed = (basis: unknown): string | null => (field(basis, "ref_type") === "combination" ? text(field(basis, "ref_id")) : null);
  const evidence = result.contract_evidence;
  const rows = list(result.results);
  const cases = list(field(result.numerical_quality, "cases"));
  const ids = new Set<string>();
  const add = (into: Set<string>, value: string | null) => { if (value !== null) into.add(value); };
  for (const member of ["combination_gates", "exact_combinations", "combination_operands"]) {
    for (const record of list(field(evidence, member))) add(ids, text(field(record, "combination_id")));
  }
  for (const record of list(field(field(evidence, "combination_certificate"), "combinations"))) add(ids, text(field(record, "combination_id")));
  for (const row of rows) add(ids, typed(field(row, "basis_ref")));
  for (const item of cases) add(ids, typed(field(item, "basis_ref")));
  const other = new Set<string>();
  for (const item of cases) if (typed(field(item, "basis_ref")) === null) add(other, text(field(field(item, "basis_ref"), "ref_id")));
  for (const member of ["preview_cases", "exact_cases"]) {
    for (const record of list(field(evidence, member))) add(other, text(field(record, "load_case_id")));
  }
  for (const row of rows) add(other, text(field(row, "entity_ref")));
  return new Set([...ids].filter((id) => id !== "" && !other.has(id)));
}

/** A diagnostic's presented message. On a result with combinations, a diagnostic is copied only
 * when it names one of them and `rosterChecked` holds (the closed roster's reader admitted it in
 * its canonical form); every other message is quoted after the supplemental label, as text
 * supplied with the result and verified by nothing, its source text kept whole. A result without
 * combinations is unchanged. */
export function suppliedDiagnosticMessage(
  diagnostic: { message: string; affected_refs?: string[] | null },
  combinations: ReadonlySet<string>,
  rosterChecked: boolean
): string {
  if (combinations.size === 0) return diagnostic.message;
  const refs = Array.isArray(diagnostic.affected_refs) ? diagnostic.affected_refs : [];
  if (rosterChecked && refs.some((ref) => typeof ref === "string" && combinations.has(ref))) return diagnostic.message;
  return `${SUPPLEMENTAL_DIAGNOSTIC_LABEL}"${diagnostic.message}"`;
}

/** The ordinary routes' readers (`resultExportAdapter.ts` alike); a retained route has none here. */
const ROUTE_READERS: Readonly<Record<string, ReaderName>> = { preview_physics: "preview-physics", pressure: "pressure", pressure_source: "pressure-source" };

/** Whether the current Rust reader admits the result itself, whole, on an ordinary route: the header
 * dispatch's route and that route's full reader (`readerValidate`, without the model, whose
 * population is the result's). A refusal, an unread result and a retained successor (whose G7 base
 * read applies no roster text closure; its receipt binds notice codes, cardinalities and refs, not
 * prose) are not. */
function rosterCheckedByReader(result: MechanicsResult): boolean {
  if (result.retained_precision !== undefined) return false;
  const reader = ROUTE_READERS[readerDispatch(result).route];
  if (reader === undefined) return false;
  try { readerValidate(reader, result); return true; } catch { return false; }
}

/** The result's diagnostics as the Diagnostics panel lists and explains them, each message
 * classified by `suppliedDiagnosticMessage`, with the roster checked only where
 * `rosterCheckedByReader` holds: on any other result with combinations every message is quoted as
 * supplied. A result without combinations is not read and is returned unchanged. */
export function presentedResultDiagnostics(result: MechanicsResult | null): Diagnostic[] {
  const diagnostics = result?.diagnostics ?? [];
  if (!result) return diagnostics;
  const combinations = documentCombinationIdentities(result);
  if (combinations.size === 0) return diagnostics;
  const rosterChecked = rosterCheckedByReader(result);
  return diagnostics.map((diagnostic) => {
    const message = suppliedDiagnosticMessage(diagnostic, combinations, rosterChecked);
    return message === diagnostic.message ? diagnostic : { ...diagnostic, message };
  });
}

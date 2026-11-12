/** RT-A3 §4 (DEL-05-02 Work `combination-certificate.md` §2, the residual text boundary): the one
 * classification of a result diagnostic's message, used by the report's projection
 * (`renderableReportInput.ts` `diagnosticsForSections`, mirrored by Rust
 * `report_package::bindings::expected_report_diagnostics`) and by the Diagnostics panel's list
 * and explanation (`presentedResultDiagnostics`). It reads typed identities and a reader's
 * verdict only (for a retained successor, the registered retained reader's acceptance of its exact
 * bytes; DEL-04-08 §8.53), never a message's wording, and derives no standing or certificate fact. */
import type { Diagnostic, MechanicsResult } from "../../types";
import { isNativeResult, nativeDiagnosticsPresentation, type NativeResult, type SessionResult } from "../../services/nativeResult";
import { readerDispatch, readerValidate, type ReaderName } from "./resultReader";
import { retainedPrecisionRegistration } from "./retainedPrecisionStanding";

/** The label before a diagnostic message passed through as supplied text. */
export const SUPPLEMENTAL_DIAGNOSTIC_LABEL = "Supplied diagnostic text, unverified (not a combination certificate statement): ";

/** RT-A5: the five typed held
 * gate codes (Rust `result_export::HELD_STATUS_GATE_CODES`). Such a notice's refusal text is bound to its
 * held-status record by the reader, not verified as true, and supplies no row bound, eligibility,
 * certificate notice or standing, so it is always quoted whole as supplied text, whatever the roster or
 * a certificate attests. Keyed on the typed code only, never on a message's wording. */
export const HELD_STATUS_GATE_CODES: ReadonlySet<string> = new Set([
  "HELD_STATUS_SOURCE_UNAVAILABLE",
  "HELD_STATUS_SOURCE_UNQUALIFIED",
  "HELD_STATUS_BINDING",
  "HELD_STATUS_SLIDING_GROUND_DIFFERS",
  "HELD_STATUS_SOLVE_FAILED"
]);
/** The held-status codes every presenting path quotes whole as supplied text (Rust
 * `result_export::HELD_STATUS_QUOTED_CODES`): the five held gates and `HELD_STATUS_RELEASED`. A distinct
 * set from `HELD_STATUS_GATE_CODES`: quoting is presentation, the gates are the reader's roster. */
export const HELD_STATUS_QUOTED_CODES: ReadonlySet<string> = new Set([...HELD_STATUS_GATE_CODES, "HELD_STATUS_RELEASED"]);
/** Whether a diagnostic's typed code is one of the quoted held-status codes. */
export function heldStatusQuotedNotice(diagnostic: { code?: unknown }): boolean {
  return typeof diagnostic.code === "string" && HELD_STATUS_QUOTED_CODES.has(diagnostic.code);
}

/** DEL-04-07: the warning on a case admitted under the cancellation-screen exception (Rust
 * `result_export::CAP_REMAINDER_WARNING_CODE`). The pressure readers bind its presence, its whole
 * fixed message and its references to the exception record, so where the reader admitted the result its
 * message is a product diagnostic, copied as it stands, never quoted as supplied text. */
export const CAP_REMAINDER_WARNING_CODE = "PRESSURE_REGION_CAP_REMAINDER_RETAINED";

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

/** A diagnostic's presented message. A held-status notice (`heldStatusQuotedNotice`) is always quoted
 * after the supplemental label. Otherwise, on a result with combinations, a diagnostic is copied only
 * when it names one of them and `rosterChecked` holds (the closed roster's reader admitted it in
 * its canonical form), or when it is the exception's warning (`CAP_REMAINDER_WARNING_CODE`) and
 * `rosterChecked` holds; every other message is quoted after the supplemental label, as text
 * supplied with the result and verified by nothing, its source text kept whole. A result without
 * combinations is otherwise unchanged. */
export function suppliedDiagnosticMessage(
  diagnostic: { code?: unknown; message: string; affected_refs?: string[] | null },
  combinations: ReadonlySet<string>,
  rosterChecked: boolean
): string {
  if (heldStatusQuotedNotice(diagnostic)) return `${SUPPLEMENTAL_DIAGNOSTIC_LABEL}"${diagnostic.message}"`;
  if (combinations.size === 0) return diagnostic.message;
  if (rosterChecked && diagnostic.code === CAP_REMAINDER_WARNING_CODE) return diagnostic.message;
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
 * prose) are not. A retained successor's certificate notices are attested separately
 * (`certificateAttestedByRetainedReader`). */
export function rosterCheckedByReader(result: MechanicsResult): boolean {
  if (result.retained_precision !== undefined) return false;
  const reader = ROUTE_READERS[readerDispatch(result).route];
  if (reader === undefined) return false;
  try { readerValidate(reader, result); return true; } catch { return false; }
}

/** The certificate's own notices: the typed id namespace and the reserved codes (never their wording). */
const CERTIFICATE_ID_PREFIX = "diagnostic:combination-certificate:";
const CERTIFICATE_CODES: ReadonlySet<string> = new Set(["COMBINATION_ROWS_WITHIN_STATED_BOUND", "COMBINATION_ROWS_NOT_ASSESSED"]);
function certificateNotice(diagnostic: { id?: unknown; code?: unknown }): boolean {
  return typeof diagnostic.id === "string" && diagnostic.id.startsWith(CERTIFICATE_ID_PREFIX)
    && typeof diagnostic.code === "string" && CERTIFICATE_CODES.has(diagnostic.code);
}

/** RT-A3 §4 D3 (DEL-04-08 `retained-pricing-rules.md` §8.53; DEL-05-02 Work `combination-certificate.md`
 * §2): whether a retained successor's certificate notices are reader-attested. The successor carries
 * the ordinary run's member, and the retained reader's G7 base read validates it whole on the
 * projection, whose diagnostics are the successor's own: every notice in the certificate's reserved
 * vocabulary must then be exactly one that the validated member re-derives (id, code, message, source
 * and refs). So where the registered retained reader accepted these exact bytes
 * (`retainedPrecisionRegistration`) and the member is present, a notice with the certificate's typed
 * id and code is the canonical one. Nothing else on a retained successor is attested: its other
 * messages stay supplied text. */
export function certificateAttestedByRetainedReader(result: MechanicsResult): boolean {
  if (result.retained_precision === undefined) return false;
  if (recordObject(result.contract_evidence)?.["combination_certificate"] === undefined) return false;
  return Boolean(retainedPrecisionRegistration(result)?.validation);
}

/** The three facts the classification reads, each read only when it is needed (in this order). */
type PresentationFacts = Readonly<{ combinations: () => ReadonlySet<string>; rosterChecked: () => boolean; certificateChecked: () => boolean }>;

/** A whole result's facts: the helpers above, on the value itself. */
function wholeValueFacts(result: MechanicsResult): PresentationFacts {
  return {
    combinations: () => documentCombinationIdentities(result),
    rosterChecked: () => rosterCheckedByReader(result),
    certificateChecked: () => certificateAttestedByRetainedReader(result)
  };
}

/** A population no diagnostic names (ids are never empty): with nothing attested, every message is
 * quoted as supplied. A native handle whose facts are unavailable reads it (fail closed). */
const UNVERIFIED_POPULATION: ReadonlySet<string> = new Set([""]);

/** C1: a native handle's
 * facts, Rust's on its held bytes and delivered in S (`nativeDiagnosticsPresentation`), each the
 * whole-value helper's on the same bytes. Read from the handle's custody only: never its guarded
 * members (`results`, `retained_precision`) and never a reader on the webview handle. Unavailable
 * facts (missing, malformed, another binding, ended custody) attest nothing. */
function nativeFacts(result: NativeResult): PresentationFacts {
  const facts = nativeDiagnosticsPresentation(result);
  return {
    combinations: () => facts?.combinations ?? UNVERIFIED_POPULATION,
    rosterChecked: () => facts?.rosterChecked === true,
    certificateChecked: () => facts?.certificateAttested === true
  };
}

/** The result's diagnostics as the Diagnostics panel lists and explains them, each message
 * classified by `suppliedDiagnosticMessage`, with the roster checked only where
 * `rosterCheckedByReader` holds: on any other result with combinations every message is quoted as
 * supplied, except a retained successor's certificate notices where the retained reader attested
 * the member (`certificateAttestedByRetainedReader`); a held-status notice is quoted always. A result
 * without combinations and without a held-status notice is not read and is returned unchanged. A
 * native handle is classified the same way by the same code, with the facts Rust read of its held
 * bytes (`nativeFacts`); its held-status notices are quoted exactly as the whole value's. */
export function presentedResultDiagnostics(result: SessionResult | null): Diagnostic[] {
  const diagnostics = result?.diagnostics ?? [];
  if (!result) return diagnostics;
  const facts = isNativeResult(result) ? nativeFacts(result) : wholeValueFacts(result);
  const combinations = facts.combinations();
  if (combinations.size === 0 && !diagnostics.some(heldStatusQuotedNotice)) return diagnostics;
  const rosterChecked = facts.rosterChecked();
  const certificateChecked = facts.certificateChecked();
  return diagnostics.map((diagnostic) => {
    if (certificateChecked && certificateNotice(diagnostic) && !heldStatusQuotedNotice(diagnostic)) return diagnostic;
    const message = suppliedDiagnosticMessage(diagnostic, combinations, rosterChecked);
    return message === diagnostic.message ? diagnostic : { ...diagnostic, message };
  });
}

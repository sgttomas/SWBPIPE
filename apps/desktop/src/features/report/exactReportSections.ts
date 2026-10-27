/** W-11 slice (b) (DEL-08-01 Design `exact-route-report-sections.md` §2.4, §3.2-§3.3):
 * the source-derived exact-route report sections, as pure projections of the received
 * result. The report package's Rust producer and the desktop's native render re-derive
 * the same sections (`report_package::exact_sections::expected_exact_route_core` and
 * `expected_exact_limitations`); both sides are checked against the shared vectors
 * `fixtures/reports/invented/exact_report_sections_parity.json`. A change here changes
 * the Rust projection and the vectors together.
 *
 * Nothing here admits a result to an output: the builder reaches these projections only
 * with a recorded Export Result JSON admission (`exactReportAdmission.ts`), and only the
 * rendered report and the report package, which the owner opened on 2026-10-26 (§5.1),
 * admit the exact routes; the report JSON packet keeps refusing them. Limitation statements come only from the closed statement
 * table (`exactReportLimitations.ts`). Standard claim fence applies (claims taxonomy per
 * DEC-081). */
import type { MechanicsResult } from "../../types";
import { guardResultJson, resultDigest } from "../result-export/resultExportAdapter";
import { PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID } from "../results/numericalResultQuality";
import { loadReferenceDisclosure } from "../results/loadReferenceDisclosure";
import { exactReportStatements } from "./exactReportLimitations";

/** The exact-route identities whose reports carry these sections. */
export const EXACT_REPORT_CONTRACT_IDS: readonly string[] = Object.freeze([PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID]);

export const REPORT_EXACT_HEADLINE_UNREADABLE = "REPORT-EXACT-HEADLINE-UNREADABLE";
/** The native host's section code (§3.6), used here for a source that is not exact. */
export const REPORT_EXACT_SECTIONS_MISMATCH = "REPORT-EXACT-SECTIONS-MISMATCH";

/** The section-basis headlines (§3.3), nominal then reduced, with the session's labels.
 * One constant for the session's results panel and the report. */
export const SECTION_BASIS_HEADLINES = Object.freeze([
  Object.freeze({ key: "max_open_formula_stress_nominal_section", basis: "nominal_section", label: "Nominal section (nominal wall)" }),
  Object.freeze({ key: "max_open_formula_stress_reduced_section", basis: "reduced_section", label: "Reduced section (wall minus mill tolerance and corrosion allowance)" })
] as const);
/** A null headline key (incomplete coverage) is withheld, never a value. */
export const WITHHELD_SECTION_BASIS_HEADLINE =
  "withheld: a stress maximum on this basis is not available for every member in every load case";

/** One section basis's headline (the renderer's `ReportHeadline`). */
export type ReportHeadline = {
  basis: string;
  label: string;
  value_display: string;
  location_ref: string | null;
  source_ref: string | null;
};

/** The source-derived part of the exact-route section (`ExactRouteCore`). */
export type ExactRouteCore = {
  semantic_contract_id: string;
  run_ref: string;
  result_envelope_id: string;
  received_result_sha256: string;
  load_reference_states: string | null;
  headlines: ReportHeadline[];
};

/** One exact limitation entry without the `effect` and `provenance` it shares with
 * the report's technical-preview entry (`ExpectedLimitation`). */
export type ExactLimitationEntry = {
  limitation_id: string;
  source: { ref_type: "semantic_contract"; ref_id: string };
  affected_scope: { ref_type: "analysis_run"; ref_id: string };
  statement: string;
};

/** Whether `result` names an exact-route identity (its producer identity). */
export function isExactReportSource(result: MechanicsResult | null | undefined): boolean {
  const id = result?.producer?.semantic_contract_id;
  return typeof id === "string" && EXACT_REPORT_CONTRACT_IDS.includes(id);
}

function exactIdentity(result: MechanicsResult): string {
  if (!isExactReportSource(result)) throw new Error(`${REPORT_EXACT_SECTIONS_MISMATCH}: the source is not an exact-route result`);
  return result.producer!.semantic_contract_id as string;
}

function headline(summary: Record<string, unknown>, entry: (typeof SECTION_BASIS_HEADLINES)[number]): ReportHeadline {
  const unreadable = () => new Error(`${REPORT_EXACT_HEADLINE_UNREADABLE}: the ${entry.basis} headline is missing or not readable.`);
  if (!Object.hasOwn(summary, entry.key)) throw unreadable();
  const item = summary[entry.key] as Record<string, unknown> | null;
  if (item === null) {
    return { basis: entry.basis, label: entry.label, value_display: WITHHELD_SECTION_BASIS_HEADLINE, location_ref: null, source_ref: null };
  }
  if (
    typeof item !== "object" || Array.isArray(item) ||
    typeof item.value !== "number" || !Number.isFinite(item.value) ||
    typeof item.unit !== "string" || typeof item.location_ref !== "string" || typeof item.result_ref !== "string"
  ) {
    throw unreadable();
  }
  return { basis: entry.basis, label: entry.label, value_display: `${item.value} ${item.unit}`, location_ref: item.location_ref, source_ref: item.result_ref };
}

/** The source-derived part (§3.5): the identity, the run, the result envelope, the
 * received result's digest, the load/reference line (the `rendered-report` surface's
 * disclosure line) and the headlines, nominal then reduced. */
export async function exactRouteCore(result: MechanicsResult): Promise<ExactRouteCore> {
  guardResultJson(result);
  const identity = exactIdentity(result);
  const run = typeof result.run_id === "string" && result.run_id ? result.run_id : null;
  if (run === null) throw new Error(`${REPORT_EXACT_SECTIONS_MISMATCH}: the source names no run`);
  const summary = result.summary as unknown;
  if (typeof summary !== "object" || summary === null || Array.isArray(summary)) {
    throw new Error(`${REPORT_EXACT_HEADLINE_UNREADABLE}: the nominal_section headline is missing or not readable.`);
  }
  const headlines = SECTION_BASIS_HEADLINES.map((entry) => headline(summary as Record<string, unknown>, entry));
  return {
    semantic_contract_id: identity,
    run_ref: run,
    result_envelope_id: `result-envelope:${run}`,
    received_result_sha256: await resultDigest(result),
    load_reference_states: loadReferenceDisclosure(result, "rendered-report"),
    headlines
  };
}

/** The exact limitation entries for `run` (§2.4): one per published
 * `formulation_basis.limitations` string, in order, with the closed table's statement
 * (`REPORT-EXACT-LIMITATION-UNREGISTERED` for a string the table lacks), then the
 * profile reference, which binds the semantic table `{id, sha256}` by reference. */
export async function exactLimitationEntries(
  result: MechanicsResult,
  run: string,
  table: { id: string; sha256: string }
): Promise<ExactLimitationEntry[]> {
  const identity = exactIdentity(result);
  const limitations = (result.formulation_basis as { limitations?: unknown } | undefined)?.limitations;
  if (!Array.isArray(limitations) || limitations.length === 0) {
    throw new Error(`${REPORT_EXACT_SECTIONS_MISMATCH}: the source publishes no limitations`);
  }
  const statements = await exactReportStatements(identity, limitations as string[]);
  const entry = (limitation_id: string, statement: string): ExactLimitationEntry => ({
    limitation_id,
    source: { ref_type: "semantic_contract", ref_id: identity },
    affected_scope: { ref_type: "analysis_run", ref_id: run },
    statement
  });
  return [
    ...statements.map((statement, index) => entry(`limitation:${run}:exact:${index + 1}`, statement)),
    entry(
      `limitation:${run}:exact:profile-reference`,
      `The profile limitations of the semantic table ${table.id} (sha256 ${table.sha256}) are bound by reference and are not restated in this report.`
    )
  ];
}

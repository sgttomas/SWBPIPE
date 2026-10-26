/** Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §5.4; owner ruling 11B,
 * 2026-10-21): the one disclosure line each opened output panel shows for a result that
 * carries resolved load/reference states. It reads only the published records'
 * `reference_configuration_id`, `load_case_id` and `history.kind`, in published order,
 * without duplicates; it computes and converts nothing and changes no standing, row or
 * byte. A panel shows it only where it admits the result (Rust's eligible standing).
 * Under `docs/claims_registry.md` §4 each line is a deliverable-specific scope statement
 * in ordinary prose, not a boundary statement. UI text, except one line: the
 * `rendered-report` surface's line is written into the rendered report's exact-route
 * result block (W-11, DEL-08-01 Design `exact-route-report-sections.md` §3.2), which
 * every report surface still refuses for an exact result until the owner opens it. No
 * other line is written into a document, package, record or run result. */
import { sourceContract } from "./resultReader";
import type { MechanicsResult } from "../../types";

export type LoadReferenceDisclosureSurface = "result-export" | "stress-neutral" | "rule-check" | "rendered-report";

const TAILS: Readonly<Record<LoadReferenceDisclosureSurface, string>> = Object.freeze({
  "result-export": "The export carries them in contract_evidence.load_reference_states, with the producer's limitations.",
  "stress-neutral": "The package carries them in contract_evidence.load_reference_states, with the producer's limitations.",
  "rule-check": "Rules bind the published rows of these load cases, which are solved with these states' resolved contributions.",
  "rendered-report": "Their published limitations are listed under Limitations.",
});

/** `{configurations}`, `{cases}` and `{histories}` are the published identifiers, comma-separated. */
export const N_LOAD_REFERENCE_INCLUDED = "This result includes resolved load/reference states (reference configuration {configurations}; load cases {cases}; history {histories}).";
export const N_LOAD_REFERENCE_EXPORT = `${N_LOAD_REFERENCE_INCLUDED} ${TAILS["result-export"]}`;
export const N_LOAD_REFERENCE_PACKAGE = `${N_LOAD_REFERENCE_INCLUDED} ${TAILS["stress-neutral"]}`;
export const N_LOAD_REFERENCE_RULE_CHECK = `${N_LOAD_REFERENCE_INCLUDED} ${TAILS["rule-check"]}`;
/** The rendered report's line (W-11 §3.2); the report package's Rust producer composes the same text. */
export const N_LOAD_REFERENCE_RENDERED_REPORT = `${N_LOAD_REFERENCE_INCLUDED} ${TAILS["rendered-report"]}`;

const TEMPLATES: Readonly<Record<LoadReferenceDisclosureSurface, string>> = Object.freeze({
  "result-export": N_LOAD_REFERENCE_EXPORT,
  "stress-neutral": N_LOAD_REFERENCE_PACKAGE,
  "rule-check": N_LOAD_REFERENCE_RULE_CHECK,
  "rendered-report": N_LOAD_REFERENCE_RENDERED_REPORT,
});

function distinct(values: unknown[]): string[] {
  const out: string[] = [];
  for (const value of values) if (typeof value === "string" && value.length > 0 && !out.includes(value)) out.push(value);
  return out;
}

/** The surface's disclosure line for an exact-route result that publishes
 * `contract_evidence.load_reference_states`, or null (no such group, another route, or
 * a group without the three identifiers). */
export function loadReferenceDisclosure(source: MechanicsResult | null | undefined, surface: LoadReferenceDisclosureSurface): string | null {
  if (!source) return null;
  const records = (source.contract_evidence as { load_reference_states?: unknown } | null | undefined)?.load_reference_states;
  if (!Array.isArray(records) || records.length === 0) return null;
  let route: string;
  try { route = sourceContract(source); } catch { return null; }
  if (route !== "pressure" && route !== "pressure_source") return null;
  const typed = records as { reference_configuration_id?: unknown; load_case_id?: unknown; history?: { kind?: unknown } | null }[];
  const configurations = distinct(typed.map(r => r?.reference_configuration_id));
  const cases = distinct(typed.map(r => r?.load_case_id));
  const histories = distinct(typed.map(r => r?.history?.kind));
  if (!configurations.length || !cases.length || !histories.length) return null;
  // One pass with a function replacer: identifiers are shown verbatim, whatever they contain.
  const values: Record<string, string> = { configurations: configurations.join(", "), cases: cases.join(", "), histories: histories.join(", ") };
  return TEMPLATES[surface].replace(/\{(configurations|cases|histories)\}/g, (_match, key: string) => values[key]);
}

/** The panel line, with the test id `{testIdPrefix}-load-reference-disclosure` (the
 * surface id by default; the rule-check run panel uses `rule-check-run`). */
export function LoadReferenceDisclosure({ text, surface, testIdPrefix }: { text: string | null | undefined; surface: LoadReferenceDisclosureSurface; testIdPrefix?: string }) {
  if (!text) return null;
  return <p className="muted" role="note" data-testid={`${testIdPrefix ?? surface}-load-reference-disclosure`}>{text}</p>;
}

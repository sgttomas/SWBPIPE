/** A stored run's binding to the semantic table that gives its rows their meaning (DEL-05-02,
 * DEL-14-02): the one decision a reopen makes before it reads any row, and what a person is told
 * of a withheld run. The native twin is `src-tauri/src/analysis_run_v03_validate.rs`. */

import { FORMER_TABLE_TEXT, isKnownFormerDigest, semanticContractMismatch, UNRECOGNISED_TABLE_TEXT, ANALYSIS_SEMANTIC_CONTRACT_MISMATCH } from "./semanticContractRefusal";
export { semanticContractMismatch, ANALYSIS_SEMANTIC_CONTRACT_MISMATCH };

/** A saved result with no record of the revision of its table. */
export const ANALYSIS_SEMANTIC_CONTRACT_UNRECORDED = "ANALYSIS_SEMANTIC_CONTRACT_UNRECORDED";
const UNRECORDED_TABLE_TEXT = "this saved result names its result rules but does not record their revision, so its rows cannot be interpreted reliably; the saved result is preserved; run the analysis again to view current results";
const UNREADABLE_RESULT_TEXT = "the stored result cannot be read as a result, so it has no rows to show; the stored run is unchanged; run the analysis again on its saved model state to create a new result";
const INTEGRITY_TEXT = "the stored run does not agree with the hashes recorded with it, or they cannot be checked, so its rows cannot be interpreted reliably; the stored run is unchanged; run the analysis again on its saved model state to create a new result";

/** Why a reopened run's rows are withheld: the record names a former digest; it names a digest (or
 * binding) this version does not read; nothing records a binding; the stored run fails the hashes
 * recorded with it; or there is no stored result that reads as one (nothing to bind). */
export type WithheldReason = "former_digest" | "unrecognised_digest" | "unrecorded" | "integrity" | "unreadable_result";
/** The one decision of a reopen: its rows are read under the installed table or a registry variant
 * of the statement's own table, or withheld for one reason. The native reopen states the same JSON. */
export type TableBinding = Readonly<{ read: "installed" | "recorded" }> | Readonly<{ withheld: WithheldReason }>;

export function bindingRead(binding: TableBinding): boolean {
  return "read" in binding;
}

/** The hash findings that withhold a reopened run's rows (DEL-14-02 requirement 015: the record,
 * the received result, the input manifest and the state binding against their recorded hashes).
 * The project's own model and envelope hashes are not the run's, and a saved run has no stored
 * input manifest, so neither withholds anything. */
export const INTEGRITY_FINDINGS: ReadonlySet<string> = new Set([
  "HISTORICAL_ANALYSIS_HASH_MISMATCH", "HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE",
  "HISTORICAL_RESULT_HASH_MISMATCH", "HISTORICAL_RESULT_HASH_MISSING",
  "HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE",
  "RECORDED_RUN_INPUT_MANIFEST_HASH_MISMATCH", "RECORDED_RUN_MODEL_STATE_HASH_MISMATCH", "RECORDED_RUN_RECORD_HASH_MISMATCH",
]);

/** What the record claims, read before any row: no binding (`unrecorded`), a registry variant of
 * the statement's own table, the installed table's binding, or another binding. */
export type BindingClaim = Readonly<{ claim: "unrecorded" }> | Readonly<{ claim: "recorded" | "installed" }> | Readonly<{ claim: "other"; recorded: unknown }>;

/** The claim of a stored record over its statement. `statementId` is the statement header's
 * `producer.semantic_contract_id`; `installed` is the installed table's binding the header
 * dispatch names for it (null when the header names no table this version reads); `variant` is the
 * registry variant the record's binding names exactly (null for none). */
export function bindingClaim(record: unknown, statementId: unknown, installed: Readonly<{ id: string; sha256: string }> | null, variant: Readonly<{ id: string; sha256: string }> | null): BindingClaim {
  const envelope = record && typeof record === "object" && !Array.isArray(record) ? record as Record<string, unknown> : null;
  const run = envelope?.schema_version === "0.3.0" && envelope.analysis_run && typeof envelope.analysis_run === "object" && !Array.isArray(envelope.analysis_run)
    ? envelope.analysis_run as Record<string, unknown> : null;
  const reproducibility = run?.reproducibility && typeof run.reproducibility === "object" && !Array.isArray(run.reproducibility) ? run.reproducibility as Record<string, unknown> : null;
  const recorded = reproducibility?.semantic_contract;
  const named = recorded && typeof recorded === "object" && !Array.isArray(recorded) ? recorded as Record<string, unknown> : null;
  if (!named || Object.keys(named).length !== 2 || typeof named.id !== "string" || typeof named.sha256 !== "string") return { claim: "unrecorded" };
  if (variant && variant.id === statementId) return { claim: "recorded" };
  if (installed && named.id === statementId && named.id === installed.id && named.sha256 === installed.sha256) return { claim: "installed" };
  return { claim: "other", recorded };
}

/** The decision: a claim to a table this version reads holds only when no integrity finding was found. */
export function decideBinding(claim: BindingClaim, checkFindings: readonly string[]): TableBinding {
  if (claim.claim === "unrecorded") return { withheld: "unrecorded" };
  if (checkFindings.some((finding) => INTEGRITY_FINDINGS.has(finding))) return { withheld: "integrity" };
  if (claim.claim === "other") return { withheld: isKnownFormerDigest(claim.recorded) ? "former_digest" : "unrecognised_digest" };
  return { read: claim.claim };
}

/** The code a withheld run is refused under (none for integrity or an unreadable result: their
 * findings are the refusal). */
export function withheldCode(reason: WithheldReason): string | null {
  switch (reason) {
    case "former_digest": case "unrecognised_digest": return ANALYSIS_SEMANTIC_CONTRACT_MISMATCH;
    case "unrecorded": return ANALYSIS_SEMANTIC_CONTRACT_UNRECORDED;
    case "integrity": case "unreadable_result": return null;
  }
}

/** What a person is told of a withheld run. */
export function withheldMessage(reason: WithheldReason): string {
  switch (reason) {
    case "former_digest": return FORMER_TABLE_TEXT;
    case "unrecognised_digest": return UNRECOGNISED_TABLE_TEXT;
    case "unrecorded": return UNRECORDED_TABLE_TEXT;
    case "integrity": return INTEGRITY_TEXT;
    case "unreadable_result": return UNREADABLE_RESULT_TEXT;
  }
}

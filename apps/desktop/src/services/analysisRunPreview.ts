/** C1 native custody, slice 5's builder (DEL-04-05 `c1-native-custody-plan.md`): the webview's
 * side of the native analysis-run preview (`src-tauri/src/analysis_run_v03_build.rs`). Nothing
 * calls it yet: the session switches from `buildAnalysisRunPreview` in slice 4, and Record Run and
 * Save Project take the job handle in slice 5. Until then `buildAnalysisRunV03` stays the oracle
 * (`analysisRunV03Parity.test.ts`).
 *
 * - `read_preview_mechanics_job_analysis_run_preview {jobId, inputManifest: {manifest_ref,
 *   manifest_sha256, canonical_bytes}, ruleCheckAggregate, rulePackExclusions}`: the current (0.3.0)
 *   record `buildAnalysisRunPreview` would build for the held result, built natively, returned as
 *   its header only. The session's manifest evidence goes without its parsed copy (its canonical
 *   bytes fix it); the aggregate is the GUI rule-check aggregate or `null`.
 * - The header carries the run's identity, both checksums, the statuses, the record's stored length
 *   and every record member except the row-scale ones (`result_refs`, the producer's
 *   `diagnostics`, and the copied `contract_evidence`, `source_block_recovery` and
 *   `retained_precision`), which it counts (`result_ref_count`, `producer_diagnostic_count`) or
 *   names (`carried_members`); the rule-pack reference diagnostics the record appends are carried
 *   whole. The full record stays native, for the persistence that reads its exact bytes.
 * - The response is the header's JSON as raw bytes (an ArrayBuffer through Tauri);
 *   `decodeAnalysisRunPreview` refuses any other shape with `ANALYSIS-RUN-PREVIEW-MALFORMED`. A
 *   refusal is the TypeScript builder's own message (or `ANALYSIS-RUN-INPUT-SHAPE-INVALID` where
 *   the TypeScript throws a `TypeError`). */
import type { AnalysisRunEnvelope } from "../types";
import type { CurrentSessionInputManifestEvidence } from "./inputManifestService";
import type { ExcludedRulePackRef } from "./rulePackRefDiagnostics";

export const ANALYSIS_RUN_PREVIEW_COMMAND = "read_preview_mechanics_job_analysis_run_preview";
export const ANALYSIS_RUN_PREVIEW_MALFORMED = "ANALYSIS-RUN-PREVIEW-MALFORMED";
export const ANALYSIS_RUN_PREVIEW_KIND = "swbpipe.desktop.analysis_run_preview";
export const ANALYSIS_RUN_PREVIEW_VERSION = 1;
/** The record members the header leaves out. */
export const ANALYSIS_RUN_ROW_SCALE_MEMBERS = Object.freeze(["diagnostics", "result_refs", "contract_evidence", "source_block_recovery", "retained_precision"] as const);

type Run = AnalysisRunEnvelope["analysis_run"];
/** The record without its row-scale members. */
export type AnalysisRunRecordHeader = Omit<AnalysisRunEnvelope, "analysis_run"> & {
  analysis_run: Omit<Run, (typeof ANALYSIS_RUN_ROW_SCALE_MEMBERS)[number]>;
};
export type AnalysisRunPreview = Readonly<{
  document_kind: typeof ANALYSIS_RUN_PREVIEW_KIND;
  version: typeof ANALYSIS_RUN_PREVIEW_VERSION;
  job_id: string;
  route: string;
  /** The record's exact stored length in bytes. */
  record_bytes: number;
  /** The record's rule-check status (the aggregate, or the solve's). */
  rule_check_status: string;
  result_ref_count: number;
  producer_diagnostic_count: number;
  /** Which source members the record copies whole. */
  carried_members: readonly string[];
  record: AnalysisRunRecordHeader;
  rule_pack_ref_diagnostics: NonNullable<Run["diagnostics"]>;
}>;

export class AnalysisRunPreviewMalformed extends Error {
  constructor(reason: string) {
    super(`${ANALYSIS_RUN_PREVIEW_MALFORMED}: ${reason}`);
    this.name = "AnalysisRunPreviewMalformed";
  }
}
function fail(reason: string): never {
  throw new AnalysisRunPreviewMalformed(reason);
}
const count = (value: unknown, what: string): number => (Number.isSafeInteger(value) && (value as number) >= 0 ? value as number : fail(`${what} is not a count`));
const text = (value: unknown, what: string): string => (typeof value === "string" ? value : fail(`${what} is not text`));
const object = (value: unknown, what: string): Record<string, unknown> => (value && typeof value === "object" && !Array.isArray(value) ? value as Record<string, unknown> : fail(`${what} is not an object`));
const list = (value: unknown, what: string): unknown[] => (Array.isArray(value) ? value : fail(`${what} is not a list`));

function freeze<T>(value: T): T {
  if (value && typeof value === "object") {
    for (const item of Object.values(value)) freeze(item);
    Object.freeze(value);
  }
  return value;
}

/** Reads the preview's response, checking its shape. */
export function decodeAnalysisRunPreview(input: ArrayBuffer | Uint8Array): AnalysisRunPreview {
  let header: unknown;
  try {
    header = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(input));
  } catch {
    fail("the response is not UTF-8 JSON");
  }
  const h = object(header, "the header");
  if (h.document_kind !== ANALYSIS_RUN_PREVIEW_KIND || h.version !== ANALYSIS_RUN_PREVIEW_VERSION) fail("not an analysis-run preview of this version");
  const record = object(h.record, "record");
  const run = object(record.analysis_run, "record.analysis_run");
  for (const member of ANALYSIS_RUN_ROW_SCALE_MEMBERS) if (Object.hasOwn(run, member)) fail(`the header carries record.analysis_run.${member}`);
  text(run.run_id, "record.analysis_run.run_id");
  for (const scope of ["analysis_run_record", "received_result"]) {
    const claims = list(run.hashes, "record.analysis_run.hashes").filter((hash) => object(hash, "a checksum").payload_scope === scope);
    if (claims.length !== 1 || !/^[0-9a-f]{64}$/.test(text((claims[0] as Record<string, unknown>).value, `the ${scope} checksum`))) fail(`no single ${scope} checksum`);
  }
  for (const status of list(run.analysis_status, "record.analysis_run.analysis_status")) text(status, "a status");
  return freeze({
    document_kind: ANALYSIS_RUN_PREVIEW_KIND,
    version: ANALYSIS_RUN_PREVIEW_VERSION,
    job_id: text(h.job_id, "job_id"),
    route: text(h.route, "route"),
    record_bytes: count(h.record_bytes, "record_bytes"),
    rule_check_status: text(h.rule_check_status, "rule_check_status"),
    result_ref_count: count(h.result_ref_count, "result_ref_count"),
    producer_diagnostic_count: count(h.producer_diagnostic_count, "producer_diagnostic_count"),
    carried_members: list(h.carried_members, "carried_members").map((member) => text(member, "a carried member")),
    record: record as AnalysisRunRecordHeader,
    rule_pack_ref_diagnostics: list(h.rule_pack_ref_diagnostics, "rule_pack_ref_diagnostics") as AnalysisRunPreview["rule_pack_ref_diagnostics"]
  });
}

/** The record's checksum of `scope` from a header. */
export function previewChecksum(preview: AnalysisRunPreview, scope: "analysis_run_record" | "received_result"): string {
  return preview.record.analysis_run.hashes.find((hash) => hash.payload_scope === scope)!.value;
}

/* -------------------------------------------------------------------------- *
 * Adapter (slice 4 binds `invoke` to the raw-body ingress)
 * -------------------------------------------------------------------------- */

/** Sends the preview's arguments and returns its response. */
export type AnalysisRunPreviewInvoke = (command: typeof ANALYSIS_RUN_PREVIEW_COMMAND, args: Record<string, unknown>) => Promise<ArrayBuffer | Uint8Array | string>;

/** The native preview of the held result's record, with `buildAnalysisRunPreview`'s inputs. */
export async function readAnalysisRunPreview(
  invoke: AnalysisRunPreviewInvoke,
  jobId: string,
  {
    inputManifest,
    ruleCheckAggregate = null,
    rulePackExclusions = []
  }: {
    inputManifest: Pick<CurrentSessionInputManifestEvidence, "manifest_ref" | "manifest_sha256" | "canonical_bytes">;
    ruleCheckAggregate?: string | null;
    rulePackExclusions?: readonly ExcludedRulePackRef[];
  }
): Promise<AnalysisRunPreview> {
  const response = await invoke(ANALYSIS_RUN_PREVIEW_COMMAND, {
    jobId,
    inputManifest: { manifest_ref: inputManifest.manifest_ref, manifest_sha256: inputManifest.manifest_sha256, canonical_bytes: inputManifest.canonical_bytes },
    ruleCheckAggregate,
    rulePackExclusions
  });
  if (typeof response === "string") fail("a text response where bytes were expected");
  return decodeAnalysisRunPreview(response);
}

import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import path from "node:path";
import os from "node:os";

export type CollectionMode = "qualification" | "characterization";
export const CHARACTERIZATION_PRODUCT_REVISION = "8468a33c86adb622b25e98f98b0eaf28c7e9fa0e";
export function collectionMode(value: string | undefined, phase = "candidate", diagnostic?: string, focused = false): CollectionMode {
  if (value === undefined) return "qualification";
  if (!["qualification", "characterization"].includes(value) || phase !== "candidate" || diagnostic !== undefined || focused)
    throw new Error("unknown or conflicting collection mode");
  return value as CollectionMode;
}
export function validateFixedSelection(counts: readonly number[], runs: readonly number[]) {
  if (JSON.stringify(counts) !== "[1000,10000]" || JSON.stringify(runs) !== "[1,2,3,4,5]")
    throw new Error("collection requires ordered five-by-two plan without subsets, duplicates or replacements");
}
export function runCollectionDisposition(mode: CollectionMode, scored: { status: string; validityFailures: readonly string[]; targetFailures: readonly string[] }, errors: readonly unknown[], segmentCount: number) {
  const complete = segmentCount === 243;
  const valid = !errors.length && !scored.validityFailures.length && ["PASS_METRIC_ACCEPTANCE", "FAIL_TARGETS"].includes(scored.status);
  const abort = !valid || !complete || (mode === "qualification" && scored.status !== "PASS_METRIC_ACCEPTANCE");
  return { mode, evidenceValidity: valid ? "VALID" : "INVALID", collectionCompleteness: complete ? "COMPLETE" : "INCOMPLETE",
    targetOutcome: scored.status, attemptDisposition: abort ? "ABORT_REMAINING" : "CONTINUE", cohortContribution: valid && complete ? 1 : 0 };
}
// Used by the actual serial driver. Errors are never reclassified as target misses.
export async function executeCollectionRun<T extends { collection: { attemptDisposition: string } }>(work: () => Promise<T>) {
  const result = await work();
  if (result.collection.attemptDisposition === "ABORT_REMAINING") throw new Error("collection aborted; original scored outcome retained");
  return result;
}
export function attemptDisposition(mode: CollectionMode, plan: readonly { runId: string }[], records: readonly any[], cohort: any) {
  const runs = plan.map(expected => {
    const matches = records.filter(r => r.expected?.runId === expected.runId);
    const r = matches.length === 1 ? matches[0] : undefined;
    return { runId: expected.runId, disposition: r?.collection ?? { evidenceValidity: "INVALID", collectionCompleteness: "INCOMPLETE",
      attemptDisposition: r?.started ? "ABORT_REMAINING" : "UNATTEMPTED", targetOutcome: "UNAVAILABLE", cohortContribution: 0 }, error: r?.error ?? null };
  });
  const complete = plan.length === 10 && records.length === 10 && runs.every(r => r.disposition.evidenceValidity === "VALID" && r.disposition.collectionCompleteness === "COMPLETE") && !cohort.validityFailures.length;
  return { mode, evidenceValidity: complete ? "VALID" : "INVALID_OR_UNAVAILABLE", collectionCompleteness: complete ? "COMPLETE_TEN_RUNS" : "INCOMPLETE_ATTEMPT",
    targetOutcome: cohort.status, attemptDisposition: complete && (mode === "characterization" || cohort.status === "PASS_COHORT_METRICS") ? "COMPLETED" : "ABORTED", runs };
}
export function validateReferenceProfile(record: any, cohortId: string, host: { model: string; memoryBytes: number }) {
  if (record?.schema !== "ui-foundation.reference-profile/v1" || record.cohortId !== cohortId || record.productRevision !== CHARACTERIZATION_PRODUCT_REVISION ||
      record.hostModel !== "Apple M5 Max" || host.model !== record.hostModel || record.memoryBytes !== 128 * 1024 ** 3 || host.memoryBytes !== record.memoryBytes ||
      record.refreshHz !== 60 || record.externallyVerified !== true || typeof record.verificationEvidence !== "string" || !record.verificationEvidence ||
      !Number.isFinite(Date.parse(record.verifiedAt)) || JSON.stringify(record.viewport) !== "[1440,920]" || record.browserDpr !== 2 || record.effectiveDprCap !== 2)
    throw new Error("required externally verified reference profile unavailable or mismatched");
  return record;
}
export async function bindReferenceProfile(env: NodeJS.ProcessEnv = process.env) {
  const file = env.UI_FOUNDATION_REFERENCE_PROFILE, sha256 = env.UI_FOUNDATION_REFERENCE_PROFILE_SHA256;
  if (!file || !path.isAbsolute(file) || !/^[a-f0-9]{64}$/.test(sha256 ?? "")) throw new Error("reference profile path/hash required");
  const bytes = await readFile(file);
  if (createHash("sha256").update(bytes).digest("hex") !== sha256) throw new Error("reference profile bytes drift");
  const record = validateReferenceProfile(JSON.parse(bytes.toString()), env.UI_FOUNDATION_COHORT_ID ?? "", { model: os.cpus()[0]?.model, memoryBytes: os.totalmem() });
  return { file, sha256, record, observation: "external refresh verification plus host readback; boundary checks only" };
}

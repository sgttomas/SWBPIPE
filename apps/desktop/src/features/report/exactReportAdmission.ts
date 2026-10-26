/** W-11 slice (b) (DEL-08-01 Design `exact-route-report-sections.md` §3.1): the Export
 * Result JSON admission of an exact-route report's source, bound to the admitted bytes.
 *
 * Equal identities and runs do not show that Export Result JSON admitted these result
 * bytes, so the report binds the admission itself. `admitExactReportSource` runs the
 * Current builder (`buildCurrentResultExport`: eligible standing with the live native
 * invocation, the bound input manifest and run record, re-checked at its end) and returns
 * `{document, binding}`; every binding field is read from the document and recomputed
 * from the inputs, and any difference refuses (`REPORT-EXACT-ADMISSION-BINDING-MISMATCH`).
 * The admission is recorded in a module-private `WeakMap` with its four input objects
 * and their checked JSON text, so a copy, a JSON round trip, a hand-built object or an
 * admission made elsewhere is not recorded (`REPORT-EXACT-ADMISSION-REQUIRED`), and a
 * recorded admission whose objects, text, invocation, standing or custody no longer hold
 * is stale (`REPORT-EXACT-ADMISSION-STALE`).
 *
 * This is an equality check, not an output gate: every report surface keeps refusing
 * an exact result by its output-policy cell, and the native render and the package's
 * Rust boundary refuse every exact identity, until the owner opens them (ruling 10A).
 * Tests reach it below the gate. Standard claim fence applies (claims taxonomy per
 * DEC-081). */
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { buildCurrentResultExport, resultDigest, type JsonObject } from "../result-export/resultExportAdapter";
import { numericalResultStanding } from "../results/resultStanding";
import { canonicalSha256Hex, checkedJsonText } from "../../services/hashService";
import type { CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import {
  deliveredIntakeText,
  deliveredMechanicsResultJob,
  hasNativeMechanicsInvocation,
  receivedResultBytes
} from "../../services/previewService";
import { isExactReportSource } from "./exactReportSections";

export const REPORT_EXACT_ADMISSION_BINDING_MISMATCH = "REPORT-EXACT-ADMISSION-BINDING-MISMATCH";
export const REPORT_EXACT_ADMISSION_REQUIRED = "REPORT-EXACT-ADMISSION-REQUIRED";
export const REPORT_EXACT_ADMISSION_STALE = "REPORT-EXACT-ADMISSION-STALE";
export const N_REPORT_EXACT_ADMISSION_REQUIRED = "an exact-route report is built only from a result that Export Result JSON admits.";
export const N_REPORT_EXACT_ADMISSION_STALE = "the result Export Result JSON admitted is no longer this session's live result.";

/** The admission binding, field for field (the renderer's `ExactAdmissionBinding`). */
export type ExactAdmissionBinding = {
  semantic_contract_id: string;
  run_ref: string;
  result_envelope_id: string;
  received_result_sha256: string;
  model_sha256: string;
  input_manifest_ref: string;
  input_manifest_sha256: string;
  derivative_sha256: string;
  native_job_id: string;
  received_bytes_sha256: string;
};

export type ExactReportAdmission = Readonly<{ document: JsonObject; binding: Readonly<ExactAdmissionBinding> }>;

export type ExactReportAdmissionInputs = {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  inputManifest: CurrentSessionInputManifestEvidence;
};

/** The texts an exact render or save resubmits to the native host (§3.6). */
export type ExactSourceSubmission = { exactResultText: string; exactIntakeText: string };

type Recorded = ExactReportAdmissionInputs & { text: string; solverMode: string; bytes: string };
const recorded = new WeakMap<object, Recorded>();

function mismatch(field: string): Error {
  return new Error(`${REPORT_EXACT_ADMISSION_BINDING_MISMATCH}: ${field}`);
}

function stale(): Error {
  return new Error(`${REPORT_EXACT_ADMISSION_STALE}: ${N_REPORT_EXACT_ADMISSION_STALE}`);
}

function textAt(value: unknown, path: readonly string[]): string | null {
  let item: unknown = value;
  for (const key of path) {
    if (typeof item !== "object" || item === null || Array.isArray(item)) return null;
    item = (item as Record<string, unknown>)[key];
  }
  return typeof item === "string" ? item : null;
}

const LOWER_HEX_64 = /^[0-9a-f]{64}$/;

async function utf8Sha256(text: string): Promise<string> {
  const digest = await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(text));
  return Array.from(new Uint8Array(digest), (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** Admits an exact-route source for a report (§3.1). Refuses a source that is not an
 * exact-route result, anything `buildCurrentResultExport` refuses, and any binding field
 * whose document value and recomputed value differ. */
export async function admitExactReportSource(inputs: ExactReportAdmissionInputs): Promise<ExactReportAdmission> {
  const { model, result, analysisRun, inputManifest } = inputs;
  if (!isExactReportSource(result)) throw mismatch("semantic_contract_id");
  const document = await buildCurrentResultExport({ model, result, analysisRun, inputManifest });
  const envelope = document.result_envelope as JsonObject | undefined;
  const nativeJobId = deliveredMechanicsResultJob(result);
  const bytes = receivedResultBytes(result);
  const run = analysisRun.analysis_run;
  const runManifestHash = run.reproducibility.input_manifest_hashes.length === 1 ? run.reproducibility.input_manifest_hashes[0].value : null;
  const recomputed: Record<keyof ExactAdmissionBinding, string | null> = {
    semantic_contract_id: textAt(result, ["producer", "semantic_contract_id"]),
    run_ref: result.run_id,
    result_envelope_id: `result-envelope:${result.run_id}`,
    received_result_sha256: await resultDigest(result),
    model_sha256: await canonicalSha256Hex(model),
    input_manifest_ref: inputManifest.manifest_ref.ref,
    input_manifest_sha256: inputManifest.manifest_sha256,
    derivative_sha256: textAt(envelope, ["reproducibility", "derivative_hash", "value"]),
    native_job_id: nativeJobId,
    received_bytes_sha256: bytes === null ? null : await utf8Sha256(bytes)
  };
  const origins = (envelope?.reproducibility as JsonObject | undefined)?.source_origin_bindings;
  const fromDocument: Partial<Record<keyof ExactAdmissionBinding, string | null>> = {
    semantic_contract_id: textAt(envelope, ["producer", "semantic_contract_id"]),
    run_ref: textAt(envelope, ["run_ref", "ref_id"]),
    result_envelope_id: textAt(envelope, ["envelope_id"]),
    received_result_sha256: Array.isArray(origins) && origins.length > 0 ? textAt(origins[0], ["received_carrier_checksum", "value"]) : null,
    model_sha256: textAt(envelope, ["reproducibility", "model_hash", "value"]),
    input_manifest_ref: textAt(envelope, ["reproducibility", "audit_manifest_ref", "ref_id"]),
    // The run record's input-manifest hash, which the builder bound to the manifest.
    input_manifest_sha256: runManifestHash
  };
  const binding = {} as ExactAdmissionBinding;
  for (const field of Object.keys(recomputed) as (keyof ExactAdmissionBinding)[]) {
    const value = recomputed[field];
    if (typeof value !== "string" || value.trim() === "") throw mismatch(field);
    if (Object.hasOwn(fromDocument, field) && fromDocument[field] !== value) throw mismatch(field);
    if (field.endsWith("_sha256") && !LOWER_HEX_64.test(value)) throw mismatch(field);
    binding[field] = value;
  }
  // The admission is for the live custody of these bytes, still held after the awaits.
  if (deliveredMechanicsResultJob(result) !== binding.native_job_id || receivedResultBytes(result) !== bytes) throw stale();
  const admission: ExactReportAdmission = Object.freeze({ document, binding: Object.freeze(binding) });
  recorded.set(admission, {
    model,
    result,
    analysisRun,
    inputManifest,
    text: checkedJsonText({ model, result, analysisRun, inputManifest }),
    solverMode: inputManifest.manifest.solver_basis.solver_mode,
    bytes: bytes as string
  });
  return admission;
}

/** Re-checks a recorded admission against the live session, synchronously (§3.1): the
 * builder at its start and after its last await, and its callers immediately before
 * IPC. An unrecorded admission refuses `REPORT-EXACT-ADMISSION-REQUIRED`; a recorded one
 * offered for other objects, or whose text, native invocation, standing, custody or
 * received bytes no longer hold, refuses `REPORT-EXACT-ADMISSION-STALE`. */
export function checkExactReportAdmission(admission: unknown, inputs: ExactReportAdmissionInputs): void {
  const record = typeof admission === "object" && admission !== null ? recorded.get(admission) : undefined;
  if (!record) throw new Error(`${REPORT_EXACT_ADMISSION_REQUIRED}: ${N_REPORT_EXACT_ADMISSION_REQUIRED}`);
  const { model, result, analysisRun, inputManifest } = inputs;
  if (record.model !== model || record.result !== result || record.analysisRun !== analysisRun || record.inputManifest !== inputManifest) throw stale();
  let text: string;
  try { text = checkedJsonText({ model, result, analysisRun, inputManifest }); } catch { throw stale(); }
  if (text !== record.text) throw stale();
  if (!hasNativeMechanicsInvocation(result, model, record.solverMode)) throw stale();
  let eligible = false;
  try { eligible = numericalResultStanding(result, model).eligible; } catch { eligible = false; }
  if (!eligible) throw stale();
  if (deliveredMechanicsResultJob(result) !== (admission as ExactReportAdmission).binding.native_job_id) throw stale();
  if (receivedResultBytes(result) !== record.bytes) throw stale();
}

/** The live check, then the texts the native host re-reads (§3.6): the bytes the job
 * delivered and the model text it was sent. Synchronous, for the pre-IPC re-check. */
export function exactSourceSubmission(admission: unknown, inputs: ExactReportAdmissionInputs): ExactSourceSubmission {
  checkExactReportAdmission(admission, inputs);
  const exactResultText = receivedResultBytes(inputs.result);
  const exactIntakeText = deliveredIntakeText(inputs.result);
  if (exactResultText === null || exactIntakeText === null) throw stale();
  return { exactResultText, exactIntakeText };
}

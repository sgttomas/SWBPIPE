/** C1 native custody, slice 5's persistence (DEL-04-05 `c1-native-custody-plan.md` slice 5): Record
 * Run and Save/Create Project of a native-custody solve, from the job handle and small arguments.
 *
 * - **What is sent.** The job id, the record's inputs as the native preview read them (the session's
 *   manifest evidence without its parsed copy, the rule-check aggregate, the rule-pack exclusions),
 *   the record the session previewed (its run id and both checksums) and the model text the job was
 *   sent (`deliveredIntakeText`: `checkedJsonText` of the captured model, exactly what
 *   `startPreviewMechanicsJob` sent; null once the result is no longer held or its registration
 *   does not hold), with Record Run's base, state and labels or the project's small fields. No
 *   result, no record, no envelope hash and no C2 full copy: the native host checks the model text
 *   against the digest it recorded when it admitted the job and against the manifest's model
 *   (`NATIVE-PERSISTENCE-CUSTODY`), rebuilds the record from the held bytes, refuses a record that
 *   is not the previewed one (`NATIVE-PERSISTENCE-RECORD-STALE`), and stores today's forms.
 * - **What comes back.** Record Run's outcome, as `record_analysis_run`'s. Save Project's small
 *   reply (`NativeProjectSave`): the stored project's summary, proposal, review target, model hash,
 *   the envelope hash computed natively over what was stored, the stored model's hash and what
 *   happened to the result (saved, or not saved because it did not fit the store:
 *   `PROJECT-SAVE-RESULT-TOO-LARGE`, the model-only save). Both are JSON bodies (Tauri delivers a
 *   parsed value); a reply carrying a model, result or record, or of another shape, is refused.
 * - **A reopened saved run** (C1 slice 6's save continuity, `src-tauri/src/stored_save.rs`). A project
 *   whose saved run was reopened into native custody is saved or created from the restored handle:
 *   the request names the handle, its stored source (as the reopen returned it) and the envelope
 *   basis the session holds, with the project's small fields; no result, record, model hash or
 *   envelope hash. The native host writes the stored run as the legacy session wrote it back (its
 *   stored values as `JSON.stringify` of their parse, read by serde_json), the stored project's model
 *   hash evidence, and the envelope hash over what was stored (the basis's stored evidence when the
 *   payload is unchanged). The reply adds `restored`: the handle (still held), its stored source and
 *   the findings of the project as stored. */
import { invokeWithIngress } from "./nativeIngress";
import { previewChecksum } from "./analysisRunPreview";
import { nativeCustody, nativeRunPreview, type NativeAnalysisRun, type NativeResult } from "./nativeResult";
import { deliveredIntakeText } from "./previewService";
import type { RestoredFindings, RestoredRun } from "./nativeReopen";
import type { CurrentSessionInputManifestEvidence } from "./inputManifestService";
import type { ExcludedRulePackRef } from "./rulePackRefDiagnostics";
import type { RuleCheckStatus } from "./ruleCheckService";
import type {
  AgentProposal,
  EditorOperationIntent,
  LocalProjectSummary,
  ModelDocumentVersionStatus,
  ModelHashEvidence,
  PreviewModel,
  ProjectEnvelopeHashEvidence,
  SelectedReviewTarget
} from "../types";

export const NATIVE_RECORD_RUN_COMMAND = "record_preview_mechanics_job_analysis_run";
export const NATIVE_SAVE_PROJECT_COMMAND = "save_preview_mechanics_job_local_project";
export const NATIVE_SAVE_RESTORED_PROJECT_COMMAND = "save_restored_result_local_project";
export const NATIVE_PROJECT_SAVE_MALFORMED = "NATIVE-PROJECT-SAVE-MALFORMED";
export const PROJECT_SAVE_RESULT_TOO_LARGE = "PROJECT-SAVE-RESULT-TOO-LARGE";
export const PROJECT_SAVE_TOO_LARGE = "PROJECT-SAVE-TOO-LARGE";
export const RUN_RECORD_RESULT_TOO_LARGE = "RUN-RECORD-RESULT-TOO-LARGE";

/** The inputs a native record was built with (the session keeps them with the record). */
export type NativeRunInputs = {
  inputManifest: CurrentSessionInputManifestEvidence;
  ruleCheckAggregate: RuleCheckStatus | null;
  rulePackExclusions: readonly ExcludedRulePackRef[];
};

/** The members every native persistence request carries: the job and the previewed record. */
export type NativePersistenceRequest = Readonly<{
  jobId: string;
  inputManifest: Readonly<Pick<CurrentSessionInputManifestEvidence, "manifest_ref" | "manifest_sha256" | "canonical_bytes">>;
  ruleCheckAggregate: RuleCheckStatus | null;
  rulePackExclusions: readonly ExcludedRulePackRef[];
  expectedRecord: Readonly<{ run_id: string; record_sha256: string; received_sha256: string }>;
  /** The model text the job was sent (`deliveredIntakeText`), or null when it is not held. */
  intakeText: string | null;
}>;

/** The request members for a native result and its native record, as the preview read them. */
export function nativePersistenceRequest(result: NativeResult, run: NativeAnalysisRun, inputs: NativeRunInputs): NativePersistenceRequest {
  const preview = nativeRunPreview(run);
  return Object.freeze({
    jobId: nativeCustody(result).jobId,
    inputManifest: Object.freeze({ manifest_ref: inputs.inputManifest.manifest_ref, manifest_sha256: inputs.inputManifest.manifest_sha256, canonical_bytes: inputs.inputManifest.canonical_bytes }),
    ruleCheckAggregate: inputs.ruleCheckAggregate,
    rulePackExclusions: inputs.rulePackExclusions,
    expectedRecord: Object.freeze({
      run_id: preview.record.analysis_run.run_id,
      record_sha256: previewChecksum(preview, "analysis_run_record"),
      received_sha256: previewChecksum(preview, "received_result")
    }),
    intakeText: deliveredIntakeText(result)
  });
}

/** Record Run's own members (as `record_analysis_run`'s request, without its record, result and manifest). */
export type NativeRecordRunMembers = {
  expected_base: string;
  state_id: string | null;
  new_state: { labels: unknown; parent_state_id: string | null } | null;
  run_labels: unknown;
};

/** Records the native run; the reply is the store's outcome (`RecordRunOutcome`). */
export async function recordNativeAnalysisRun<T>(request: NativePersistenceRequest, members: NativeRecordRunMembers): Promise<T> {
  return invokeWithIngress<T>(NATIVE_RECORD_RUN_COMMAND, { ...request, request: members });
}

/** The project's small fields a native save sends (as `create_local_project`'s and `save_local_project`'s). */
export type NativeProjectFields = {
  mode: "create" | "save";
  model: PreviewModel;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  selectedReviewTarget: SelectedReviewTarget | null;
  modelHash: ModelHashEvidence | null;
  confirmedRevision: string | null;
  /** A save of the session's own stored project: the stored envelope hash it last opened or saved. */
  expectedBase: string | null;
};

/** What happened to the result in a native save. */
export type NativeSavedResult =
  | { outcome: "saved"; run_id: string; result_bytes: number; record_bytes: number }
  | { outcome: "not_saved"; code: typeof PROJECT_SAVE_RESULT_TOO_LARGE; message: string; result_bytes: number; record_bytes: number; row_bytes: number; limit_bytes: number; refused: { what: string; bytes: number } };

/** What a reopened saved run's re-save says of the run (`RestoredSave`). */
export type NativeRestoredSave = Readonly<{
  /** The restored handle, still held: the save neither disposes nor requalifies it. */
  handle: string;
  origin: "restored_saved_run";
  source: RestoredRun["source"];
  /** The findings of the project as stored (its native reopen's); null after a model-only save. */
  findings: RestoredFindings | null;
  /** `sha256:<hex>`: the envelope payload hash of what was stored. */
  envelope_payload_hash: string;
  envelope_basis_kept: boolean;
}>;

/** The native save's reply: no model, result or record. */
export type NativeProjectSave = Readonly<{
  summary: LocalProjectSummary;
  proposal: AgentProposal | null;
  selected_review_target: SelectedReviewTarget | null;
  model_hash: ModelHashEvidence | null;
  /** Null only when a reopened project's unchanged re-save keeps a stored project that had none. */
  project_envelope_hash: ProjectEnvelopeHashEvidence | null;
  model_document_version: ModelDocumentVersionStatus | null;
  /** `sha256:<hex>` of the stored model's canonical text (`computeModelHash`'s value). */
  stored_model_hash: string;
  result: NativeSavedResult;
  /** A reopened saved run's re-save only. */
  restored?: NativeRestoredSave;
}>;

const isObject = (value: unknown): value is Record<string, unknown> => !!value && typeof value === "object" && !Array.isArray(value);
const count = (value: unknown) => typeof value === "number" && Number.isSafeInteger(value) && value >= 0;

/** The reply as the session reads it, or `NATIVE-PROJECT-SAVE-MALFORMED`. */
export function decodeNativeProjectSave(value: unknown): NativeProjectSave {
  return decodeSave(value, null);
}

/** A reopened saved run's re-save reply (`restored` carries the restored run's members). */
export function decodeRestoredProjectSave(value: unknown, restored: RestoredRun): NativeProjectSave {
  return decodeSave(value, restored);
}

function decodeSave(value: unknown, restored: RestoredRun | null): NativeProjectSave {
  const fail = (detail: string): never => { throw new Error(`${NATIVE_PROJECT_SAVE_MALFORMED}: ${detail}`); };
  if (!isObject(value)) return fail("the reply is not an object");
  for (const member of ["model", "mechanics_result", "analysis_run", "editor_intents"]) {
    if (member in value) fail(`the reply carries ${member}`);
  }
  const summary = value.summary;
  if (!isObject(summary) || typeof summary.project_id !== "string" || typeof summary.message !== "string") fail("the summary is not a project summary");
  const envelope = value.project_envelope_hash;
  if (!(isObject(envelope) && typeof envelope.value === "string") && !(restored && envelope === null)) fail("the envelope hash is not one");
  if (typeof value.stored_model_hash !== "string" || !/^sha256:[0-9a-f]{64}$/.test(value.stored_model_hash)) fail("the stored model hash is not one");
  const result = value.result;
  if (!isObject(result)) return fail("the result outcome is missing");
  if (result.outcome === "saved") {
    if (typeof result.run_id !== "string" || !count(result.result_bytes) || !count(result.record_bytes)) fail("the saved result's facts are malformed");
  } else if (result.outcome === "not_saved") {
    if (result.code !== PROJECT_SAVE_RESULT_TOO_LARGE || typeof result.message !== "string" || !["result_bytes", "record_bytes", "row_bytes", "limit_bytes"].every((key) => count(result[key]))) {
      fail("the unsaved result's facts are malformed");
    }
  } else {
    fail("the result outcome is neither saved nor not_saved");
  }
  if (!restored) {
    if ("restored" in value) fail("a fresh save's reply carries a restored run");
  } else {
    const run = (isObject(value.restored) ? value.restored : fail("the reply has no restored run")) as Record<string, unknown>;
    if (run.handle !== restored.handle || run.origin !== "restored_saved_run") fail("the restored run is not the handle's");
    const source = run.source;
    if (!isObject(source) || (["kind", "project_id", "recording_id", "result_sha256", "record_sha256"] as const).some((key) => source[key] !== restored.source[key])) fail("the restored run's stored source is not the handle's");
    if (typeof run.envelope_payload_hash !== "string" || typeof run.envelope_basis_kept !== "boolean") fail("the restored run's envelope facts are malformed");
    const found = run.findings;
    const findingsRead = isObject(found) && Array.isArray(found.findings) && found.findings.every((item: unknown) => typeof item === "string") && typeof found.run_id === "string";
    if (result.outcome === "saved" ? !findingsRead : found !== null) fail("the restored run's findings are malformed");
  }
  return Object.freeze(value) as unknown as NativeProjectSave;
}

/** Saves or creates the project with the native result (module docs). */
export async function saveNativeLocalProject(request: NativePersistenceRequest, fields: NativeProjectFields): Promise<NativeProjectSave> {
  const reply = await invokeWithIngress<unknown>(NATIVE_SAVE_PROJECT_COMMAND, {
    ...request,
    mode: fields.mode,
    projectId: fields.mode === "save" ? fields.model.project.id : null,
    projectName: fields.mode === "save" ? fields.model.project.name : null,
    model: fields.model,
    editorIntents: fields.editorIntents,
    proposal: fields.proposal,
    selectedReviewTarget: fields.selectedReviewTarget,
    modelHash: fields.modelHash,
    confirmedRevision: fields.confirmedRevision,
    expectedBase: fields.expectedBase
  });
  return decodeNativeProjectSave(reply);
}

/** The envelope basis the session holds for a reopened saved run: the payload hash and the stored
 * envelope evidence of the project as it last opened or re-saved it (the host accepts only a basis
 * it established for the handle). */
export type RestoredEnvelopeBasis = Readonly<{ payload_sha256: string | null; evidence: ProjectEnvelopeHashEvidence | null }>;

/** Saves or creates the project whose saved run is `restored`, from its handle (module docs). The
 * model hash written is the stored project's, held natively; `fields.modelHash` is not sent. */
export async function saveRestoredLocalProject(restored: RestoredRun, basis: RestoredEnvelopeBasis, fields: NativeProjectFields): Promise<NativeProjectSave> {
  const reply = await invokeWithIngress<unknown>(NATIVE_SAVE_RESTORED_PROJECT_COMMAND, {
    jobId: restored.handle,
    expectedSource: restored.source,
    envelopeBasis: basis,
    mode: fields.mode,
    projectId: fields.mode === "save" ? fields.model.project.id : null,
    projectName: fields.mode === "save" ? fields.model.project.name : null,
    model: fields.model,
    editorIntents: fields.editorIntents,
    proposal: fields.proposal,
    selectedReviewTarget: fields.selectedReviewTarget,
    confirmedRevision: fields.confirmedRevision,
    expectedBase: fields.expectedBase
  });
  return decodeRestoredProjectSave(reply, restored);
}

/** The stored model's hash evidence, as `computeModelHash` writes it for the request's model. */
export function storedModelHashEvidence(reply: NativeProjectSave, model: PreviewModel): ModelHashEvidence {
  return { algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_scope: "model_payload", payload_ref: model.project.id, value: reply.stored_model_hash, hash_status: "computed_local_preview" };
}

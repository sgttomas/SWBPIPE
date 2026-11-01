/** C1 native custody, slice 5's persistence (DEL-04-05 `c1-native-custody-plan.md` slice 5): Record
 * Run and Save/Create Project of a native-custody solve, from the job handle and small arguments.
 *
 * - **What is sent.** The job id, the record's inputs as the native preview read them (the session's
 *   manifest evidence without its parsed copy, the rule-check aggregate, the rule-pack exclusions)
 *   and the record the session previewed (its run id and both checksums), with Record Run's base,
 *   state and labels or the project's small fields. No result, no record, no envelope hash and no
 *   C2 full copy: the native host rebuilds the record from the held bytes, refuses a record that is
 *   not the previewed one (`NATIVE-PERSISTENCE-RECORD-STALE`), and stores today's forms.
 * - **What comes back.** Record Run's outcome, as `record_analysis_run`'s. Save Project's small
 *   reply (`NativeProjectSave`): the stored project's summary, proposal, review target, model hash,
 *   the envelope hash computed natively over what was stored, the stored model's hash and what
 *   happened to the result (saved, or not saved because it did not fit the store:
 *   `PROJECT-SAVE-RESULT-TOO-LARGE`, the model-only save). Both are JSON bodies (Tauri delivers a
 *   parsed value); a reply carrying a model, result or record, or of another shape, is refused. */
import { invokeWithIngress } from "./nativeIngress";
import { previewChecksum } from "./analysisRunPreview";
import { nativeCustody, nativeRunPreview, type NativeAnalysisRun, type NativeResult } from "./nativeResult";
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
    })
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

/** The native save's reply: no model, result or record. */
export type NativeProjectSave = Readonly<{
  summary: LocalProjectSummary;
  proposal: AgentProposal | null;
  selected_review_target: SelectedReviewTarget | null;
  model_hash: ModelHashEvidence | null;
  project_envelope_hash: ProjectEnvelopeHashEvidence;
  model_document_version: ModelDocumentVersionStatus | null;
  /** `sha256:<hex>` of the stored model's canonical text (`computeModelHash`'s value). */
  stored_model_hash: string;
  result: NativeSavedResult;
}>;

const isObject = (value: unknown): value is Record<string, unknown> => !!value && typeof value === "object" && !Array.isArray(value);
const count = (value: unknown) => typeof value === "number" && Number.isSafeInteger(value) && value >= 0;

/** The reply as the session reads it, or `NATIVE-PROJECT-SAVE-MALFORMED`. */
export function decodeNativeProjectSave(value: unknown): NativeProjectSave {
  const fail = (detail: string): never => { throw new Error(`${NATIVE_PROJECT_SAVE_MALFORMED}: ${detail}`); };
  if (!isObject(value)) return fail("the reply is not an object");
  for (const member of ["model", "mechanics_result", "analysis_run", "editor_intents"]) {
    if (member in value) fail(`the reply carries ${member}`);
  }
  const summary = value.summary;
  if (!isObject(summary) || typeof summary.project_id !== "string" || typeof summary.message !== "string") fail("the summary is not a project summary");
  const envelope = value.project_envelope_hash;
  if (!isObject(envelope) || typeof envelope.value !== "string") fail("the envelope hash is not one");
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

/** The stored model's hash evidence, as `computeModelHash` writes it for the request's model. */
export function storedModelHashEvidence(reply: NativeProjectSave, model: PreviewModel): ModelHashEvidence {
  return { algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_scope: "model_payload", payload_ref: model.project.id, value: reply.stored_model_hash, hash_status: "computed_local_preview" };
}

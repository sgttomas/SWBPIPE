/** Unit fake of the native persistence commands (C1 slice 5), NOT the native host: for the session
 * tests whose fake stores speak today's submitted commands. For each native Record Run or Save/
 * Create Project request it
 * - checks what the session sent: the job handle and small inputs only (no result, record, result
 *   text or envelope hash), for a job still held natively, with the model text the job's start
 *   sent (byte-equal to it, the native host's digest check) whose parse is the manifest's model
 *   (`sameModelData`; the native host compares JavaScript's numbers, refusing as it refuses);
 * - rebuilds the request today's submitted path sends from the test's copy of the held bytes with
 *   the unchanged TypeScript paths: the record (`buildAnalysisRunPreview` with the request's own
 *   inputs, refused as the native host refuses it when it is not the previewed record), the
 *   result (`JSON.parse` of the bytes; `jsonTextKeepingNegativeZero` for Record Run) and the
 *   session's envelope hash. The native writer stores exactly these forms
 *   (`native_persistence_tests.rs` against `persistence_forms.json`);
 * - hands it to the test's own fake (`submit`), and answers in the native reply's shape.
 * The actual commands' replies, from the native store, are `native_persistence_replies.json`
 * (`nativePersistenceReplay.ts`). */
import { buildAnalysisRunPreview } from "../services/previewService";
import { computeModelHash, computeProjectEnvelopeHash } from "../services/hashService";
import { jsonTextKeepingNegativeZero } from "../services/jsonText";
import { sameModelData } from "../features/results/resultStanding";
import { NATIVE_RECORD_RUN_COMMAND, NATIVE_SAVE_PROJECT_COMMAND, type NativeProjectSave } from "../services/nativePersistence";
import type { AnalysisRunEnvelope, LocalProjectEnvelope, MechanicsResult } from "../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

/** The members a native request may carry; anything else (a result, a record, a full copy) fails. */
const SAVE_MEMBERS = ["jobId", "inputManifest", "ruleCheckAggregate", "rulePackExclusions", "expectedRecord", "intakeText", "mode", "projectId", "projectName", "model", "editorIntents", "proposal", "selectedReviewTarget", "modelHash", "confirmedRevision", "expectedBase"];
const RECORD_MEMBERS = ["jobId", "inputManifest", "ruleCheckAggregate", "rulePackExclusions", "expectedRecord", "intakeText", "request"];

export type SubmittedSave = { mode: "create" | "save"; request: Json };
export type NativePersistenceFakeOptions = {
  /** The held bytes of a native-custody job while it is held, else null. */
  heldBytes: (jobId: string) => string | null;
  /** The model text a native-custody job's start sent, while it is held, else null. */
  heldIntake: (jobId: string) => string | null;
  /** The test's fake of `save_local_project` (`mode: "save"`, its `request`) or `create_local_project` (its flat args). */
  save: (submitted: SubmittedSave) => Promise<LocalProjectEnvelope> | LocalProjectEnvelope;
  /** The test's fake of `record_analysis_run` (its `request`). */
  record?: (request: Json) => Promise<unknown> | unknown;
};

export const isNativePersistenceCommand = (command: string) => command === NATIVE_SAVE_PROJECT_COMMAND || command === NATIVE_RECORD_RUN_COMMAND;

export function createNativePersistenceFake(options: NativePersistenceFakeOptions) {
  const sent: Array<{ command: string; args: Json }> = [];
  async function rebuilt(args: Json, allowed: readonly string[]) {
    for (const key of Object.keys(args)) if (!allowed.includes(key)) throw new Error(`NATIVE_PERSISTENCE_FAKE: the request carries ${key}`);
    const bytes = options.heldBytes(args.jobId);
    if (bytes === null) throw `NATIVE-PERSISTENCE-CUSTODY: job \`${args.jobId}\`'s result is no longer held; nothing was written.`;
    const result = JSON.parse(bytes) as MechanicsResult;
    const evidence = { ...args.inputManifest, manifest: JSON.parse(args.inputManifest.canonical_bytes) };
    const intake = options.heldIntake(args.jobId);
    if (typeof args.intakeText !== "string") throw `NATIVE-PERSISTENCE-CUSTODY: the model text job \`${args.jobId}\` was sent is not in the request; nothing was written.`;
    if (args.intakeText !== intake) throw `NATIVE-PERSISTENCE-CUSTODY: the model text in the request is not the body job \`${args.jobId}\` admitted; nothing was written.`;
    if (!sameModelData(JSON.parse(args.intakeText), evidence.manifest.model_basis?.model_payload)) throw `NATIVE-PERSISTENCE-CUSTODY: the input manifest's model is not the model job \`${args.jobId}\` solved; nothing was written.`;
    const record = await buildAnalysisRunPreview(result, { inputManifest: evidence, ruleCheckAggregate: args.ruleCheckAggregate, rulePackExclusions: args.rulePackExclusions }) as AnalysisRunEnvelope;
    const claim = (scope: string) => record.analysis_run.hashes.find((hash) => hash.payload_scope === scope)!.value;
    const expected = args.expectedRecord;
    if (record.analysis_run.run_id !== expected.run_id || claim("analysis_run_record") !== expected.record_sha256 || claim("received_result") !== expected.received_sha256) {
      throw "NATIVE-PERSISTENCE-RECORD-STALE: the record built now is not the record the session previewed; nothing was written.";
    }
    return { bytes, result, record };
  }
  async function invoke(command: string, args: Json): Promise<unknown> {
    sent.push({ command, args: structuredClone(args) });
    if (command === NATIVE_RECORD_RUN_COMMAND) {
      const { result, record } = await rebuilt(args, RECORD_MEMBERS);
      if (!options.record) throw new Error("NATIVE_PERSISTENCE_FAKE: no record store");
      const request = args.request;
      return options.record({
        project_id: result.model_ref,
        expected_base: request.expected_base,
        run_record: record,
        result_json: jsonTextKeepingNegativeZero(result),
        input_manifest_canonical: args.inputManifest.canonical_bytes,
        state_id: request.state_id,
        new_state: request.new_state,
        run_labels: request.run_labels
      });
    }
    if (command !== NATIVE_SAVE_PROJECT_COMMAND) throw new Error(`NATIVE_PERSISTENCE_FAKE: ${command}`);
    const { result, record } = await rebuilt(args, SAVE_MEMBERS);
    const fields = { model: args.model, editor_intents: args.editorIntents, proposal: args.proposal, selected_review_target: args.selectedReviewTarget, model_hash: args.modelHash };
    const envelopeHash = await computeProjectEnvelopeHash({ ...fields, mechanics_result: result, analysis_run: record });
    const stored = await options.save(args.mode === "save"
      ? { mode: "save", request: { project_id: args.projectId, project_name: args.projectName, ...fields, mechanics_result: result, analysis_run: record, project_envelope_hash: envelopeHash, confirmed_revision: args.confirmedRevision, expected_base: args.expectedBase } }
      : { mode: "create", request: { model: args.model, editorIntents: args.editorIntents, proposal: args.proposal, selectedReviewTarget: args.selectedReviewTarget, mechanicsResult: result, analysisRun: record, modelHash: args.modelHash, projectEnvelopeHash: envelopeHash, confirmedRevision: args.confirmedRevision } });
    const reply: NativeProjectSave = {
      summary: stored.summary,
      proposal: stored.proposal ?? null,
      selected_review_target: stored.selected_review_target ?? null,
      model_hash: stored.model_hash ?? null,
      project_envelope_hash: stored.project_envelope_hash ?? envelopeHash!,
      model_document_version: stored.model_document_version ?? null,
      stored_model_hash: (await computeModelHash(stored.model))!.value,
      result: { outcome: "saved", run_id: result.run_id, result_bytes: JSON.stringify(result).length, record_bytes: JSON.stringify(record).length }
    };
    return reply;
  }
  return { invoke, sent, handles: isNativePersistenceCommand };
}

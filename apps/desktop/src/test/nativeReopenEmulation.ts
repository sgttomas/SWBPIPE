/** C1 slice 6, unit emulation only (NOT a native reopen and NOT proof of one). Tests written before
 * slice 6 mock the whole-value `open_local_project` / `read_analysis_run`; the session now opens through
 * `open_local_project_in_native_custody` / `read_analysis_run_in_native_custody`. This bridge answers
 * the native commands from those mocks so the tests keep exercising the session's production path:
 *
 * - the findings are the unchanged TypeScript contexts' over the mock's full rows (the oracle the
 *   native port is checked against: `historicalReopenParity.test.ts`, `stored_custody_tests.rs`);
 * - the restored handle's S, pages and semantics are answered by the actual Rust commands' recorded
 *   replies for the handle's bytes (`nativeReplayStore.ts`; an unrecorded request fails the test);
 * - its bounded whole copies return the mock's own values (at or under 64 MiB), as the C2 copy does;
 * - a save of the reopened project from the handle (`save_restored_result_local_project`) is answered
 *   by the legacy `saveLocalProject` / `createLocalProject` the session called before (the test's legacy
 *   mock on the desktop, the browser store without the desktop runtime), sent what the legacy
 *   session sent (the handle's values, the stored model hash evidence, `computeProjectEnvelopeHash`'s
 *   evidence or the basis's when its payload is unchanged), its reply shaped as the native one with the
 *   TypeScript context's findings of the stored project. The actual re-save's proof is
 *   `stored_save_tests.rs` (byte parity with the submitted path) and `nativeReopen.session.test.tsx`.
 *
 * The reopen's actual connecting proof is `nativeReopen.session.test.tsx` on the commands' committed
 * replies (`native_reopen_replies.json`, written by `stored_custody_tests.rs`). */
// The historical contexts are imported when first needed: this module is loaded by the test invoke
// wrapper inside the mocked `@tauri-apps/api/core` factory, which they import.
import { jsonTextKeepingNegativeZero } from "../services/jsonText";
import { nativeReplayReply } from "./nativeReplayStore";

/** `nativeReopen.ts`'s `RESTORED_HANDLE_PREFIX`. */
const RESTORED_HANDLE_PREFIX = "restored-stored-result-";
const contexts = () => import("../features/results/HistoricalRunContext");

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type AnyInvoke = (command: string, args?: unknown, options?: unknown) => unknown;

const OPEN = "open_local_project_in_native_custody";
const READ = "read_analysis_run_in_native_custody";
const SAVE_RESTORED = "save_restored_result_local_project";
const RESULT_COMMANDS = new Set([
  "read_preview_mechanics_job_verdict", "read_preview_mechanics_job_summary", "query_preview_mechanics_job_rows",
  "lookup_preview_mechanics_job_rows", "read_preview_mechanics_job_row_semantics", "query_preview_mechanics_job_source_recovery",
  "query_preview_mechanics_job_navigator"
]);
const ROW_SCALE = ["result_refs", "diagnostics", "contract_evidence", "source_block_recovery", "retained_precision"];

let sequence = 0;
/** Emulated handles: the mock's result text and record text, the stored source, and (a saved
 * project's) its stored model hash evidence. */
type Held = { result: string; record: string; disposed: boolean; source: Json; modelHash: Json };
const held = new Map<string, Held>();

const bytes = (text: string) => {
  const encoded = new TextEncoder().encode(text);
  return encoded.buffer.slice(encoded.byteOffset, encoded.byteOffset + encoded.byteLength);
};

function header(record: unknown): Json {
  if (!record || typeof record !== "object" || Array.isArray(record)) return { record, rule_pack_ref_diagnostics: [], result_ref_count: null, diagnostic_count: null, carried_members: [] };
  const copy = structuredClone(record) as Json;
  const run = copy.analysis_run && typeof copy.analysis_run === "object" && !Array.isArray(copy.analysis_run) ? copy.analysis_run : null;
  const diagnostics = Array.isArray(run?.diagnostics) ? run.diagnostics.filter((item: Json) => item?.source_annotation?.source === "desktop/solve-rule-pack-resolution") : [];
  const counts = { result_ref_count: Array.isArray(run?.result_refs) ? run.result_refs.length : null, diagnostic_count: Array.isArray(run?.diagnostics) ? run.diagnostics.length : null };
  const carried = ["contract_evidence", "source_block_recovery", "retained_precision"].filter((member) => run && member in run);
  if (run) for (const member of ROW_SCALE) delete run[member];
  return { record: copy, rule_pack_ref_diagnostics: diagnostics, ...counts, carried_members: carried };
}

function restored(origin: string, kind: string, projectId: string, recordingId: string | null, result: unknown, record: unknown, context: Json, modelHash: Json = null) {
  const handle = `${RESTORED_HANDLE_PREFIX}emulated-${++sequence}`;
  const resultText = jsonTextKeepingNegativeZero(result ?? null), recordText = JSON.stringify(record ?? null);
  const source = { kind, project_id: projectId, recording_id: recordingId, result_sha256: "unit-emulation", record_sha256: "unit-emulation" };
  held.set(handle, { result: resultText, record: recordText, disposed: false, source, modelHash });
  return {
    handle, origin,
    source,
    result_bytes: new TextEncoder().encode(resultText).length, record_bytes: new TextEncoder().encode(recordText).length,
    result_present: result != null, record_present: record != null,
    findings: { findings: context.findings, run_id: context.runId, renderable: context.mechanicsResult !== null, mechanics: context.mechanicsResult?.status?.mechanics ?? null, envelope_payload_hash: context.envelopePayloadHash, model_hash: context.modelHash },
    record_header: header(record), record_header_bytes: null
  };
}

/** The emulated reply to `command`, or `undefined` when the bridge does not answer it. */
export function emulateNativeReopen(mock: AnyInvoke, command: string, args: Json): Promise<unknown> | undefined {
  if (command === OPEN) {
    return (async () => {
      const envelope = await mock("open_local_project", { projectId: args?.projectId ?? null }) as Json;
      if (!envelope) return envelope ?? null;
      const { mechanics_result: result, analysis_run: record, stored_result_withheld: withheld, ...members } = structuredClone(envelope);
      if (withheld) throw new Error("NATIVE_REOPEN_EMULATION: a withheld legacy envelope has no native counterpart; answer the native command directly");
      if (result == null && record == null) return { ...members, stored_run: { state: "none" } };
      const context = await (await contexts()).buildHistoricalRunContext(envelope);
      return { ...members, stored_run: { state: "restored", ...restored("restored_saved_run", "saved_project_run", envelope.model?.project?.id ?? "", null, result, record, context, envelope.model_hash ?? null) } };
    })();
  }
  if (command === READ) {
    return (async () => {
      const read = await mock("read_analysis_run", args) as Json;
      const context = await (await contexts()).buildRecordedRunContext(read);
      let result: unknown = null;
      try { result = JSON.parse(read.result_json); } catch { result = read.result_json; }
      return { run: read.run, state: read.state, input_manifest_canonical: read.input_manifest_canonical, restored: restored("restored_recorded_run", "recorded_run", read.run.project_id, read.run.recording_id, result, read.record, context) };
    })();
  }
  const jobId = typeof args?.jobId === "string" ? args.jobId : null;
  const entry = jobId ? held.get(jobId) : undefined;
  if (!entry || !jobId) return undefined;
  if (command === "dispose_preview_mechanics_result") {
    const released = !entry.disposed;
    entry.disposed = true;
    return Promise.resolve({ job_id: jobId, released, released_bytes: 0, delivered: false });
  }
  if (entry.disposed) return Promise.reject(command === SAVE_RESTORED ? `NATIVE-PERSISTENCE-CUSTODY: \`${jobId}\`'s stored run is no longer held; nothing was written.` : `PREVIEW-SOLVE-RESULT-UNAVAILABLE: ${jobId} is disposed`);
  if (command === SAVE_RESTORED) return emulatedRestoredSave(mock, jobId, entry, args);
  if (command === "read_preview_mechanics_job_rows_all") return Promise.resolve(bytes(entry.result));
  if (command === "read_restored_result_record_all") return Promise.resolve(bytes(entry.record));
  if (command === "release_preview_mechanics_job_full_copy") return Promise.resolve(true);
  if (RESULT_COMMANDS.has(command)) {
    try { return Promise.resolve(nativeReplayReply(command, args, jobId, entry.result)); } catch (error) { return Promise.reject(error); }
  }
  return undefined;
}

/** The legacy session's save of the reopened project, through the test's legacy mock (module docs). */
async function emulatedRestoredSave(mock: AnyInvoke, jobId: string, entry: Held, args: Json): Promise<unknown> {
  if (entry.source.kind !== "saved_project_run") throw `NATIVE-PERSISTENCE-CUSTODY: \`${jobId}\` holds a restored recorded run; nothing was written.`;
  const { computeModelHash, computeProjectEnvelopeHash } = await import("../services/hashService");
  const result = JSON.parse(entry.result), record = JSON.parse(entry.record);
  const fields = { model: args.model, editor_intents: args.editorIntents, proposal: args.proposal ?? null, selected_review_target: args.selectedReviewTarget ?? null };
  const computed = await computeProjectEnvelopeHash({ ...fields, mechanics_result: result, analysis_run: record, model_hash: entry.modelHash });
  const kept = computed?.value === args.envelopeBasis?.payload_sha256;
  const envelopeHash = kept ? args.envelopeBasis.evidence : computed;
  // The legacy session's own call (`projectService`): the test's legacy mock on the desktop, the browser
  // store in a test that leaves the desktop runtime before saving.
  const service = await import("../services/projectService");
  void mock;
  const legacy = await (args.mode === "create"
    ? service.createLocalProject(args.model, args.editorIntents, fields.proposal, fields.selected_review_target, result, record, entry.modelHash, envelopeHash, args.confirmedRevision ?? null)
    : service.saveLocalProject(args.model, args.editorIntents, fields.proposal, fields.selected_review_target, result, record, entry.modelHash, envelopeHash, args.confirmedRevision ?? null, args.expectedBase ?? null)) as Json;
  const stored = { ...legacy, mechanics_result: legacy?.mechanics_result ?? result, analysis_run: legacy?.analysis_run ?? record, model: legacy?.model ?? args.model };
  const context = (await (await contexts()).buildHistoricalRunContext(stored))!;
  // The stored project's model hash evidence, as the store reports it after the save (the native host
  // writes the one it holds, so its reply and its held value never differ).
  entry.modelHash = stored.model_hash ?? entry.modelHash;
  const length = (value: unknown) => new TextEncoder().encode(JSON.stringify(value ?? null)).length;
  return {
    summary: legacy?.summary ?? { project_id: args.model.project.id, message: "Saved current project model snapshot without external file copies." },
    proposal: stored.proposal ?? null, selected_review_target: stored.selected_review_target ?? null,
    model_hash: stored.model_hash ?? entry.modelHash, project_envelope_hash: stored.project_envelope_hash ?? envelopeHash,
    model_document_version: stored.model_document_version ?? null,
    stored_model_hash: (await computeModelHash(stored.model))!.value,
    result: { outcome: "saved", run_id: context.runId, result_bytes: length(stored.mechanics_result), record_bytes: length(stored.analysis_run) },
    restored: {
      handle: jobId, origin: "restored_saved_run", source: entry.source,
      findings: { findings: context.findings, run_id: context.runId, renderable: context.mechanicsResult !== null, mechanics: null, envelope_payload_hash: context.envelopePayloadHash, model_hash: null },
      envelope_payload_hash: computed?.value ?? "", envelope_basis_kept: kept
    }
  };
}

/** Test support: forget the emulated handles. */
export function resetNativeReopenEmulation(): void {
  held.clear();
}

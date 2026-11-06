/** C1 slice 6: saved and recorded runs reopen into native custody, in the actual workspace session, on
 * the actual Rust commands' replies (`test-support/native-reopen/native_reopen_replies.json`, written by
 * `src-tauri/src/stored_custody_tests.rs`): the session's own native Save Project and Record Run
 * requests (`native_persistence_replies.json`) on a real store, a session reset, then the native open,
 * every request the historical panel sends on the restored handle (S, pages, semantics), and the
 * recorded run's native read, then Save Project from the restored handle (C1 slice 6's save
 * continuity: the actual `save_restored_result_local_project` on the same store). Each request must
 * equal a recorded request exactly (its handle written `$HANDLE`); anything else fails. With NATIVE_REOPEN_RECORD=1 the panel's requests are written to
 * `native_reopen_requests.json` for the Rust test to answer. All data are invented or committed
 * fixtures. */
import { act, cleanup, render, screen, waitFor, within } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock, { nativeReopen: "actual" }) }));
import { nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { isNativeResult, nativeCustody } from "../../services/nativeResult";
import { NATIVE_OPEN_PROJECT_COMMAND, NATIVE_READ_RUN_COMMAND } from "../../services/nativeReopen";
import { NATIVE_SAVE_RESTORED_PROJECT_COMMAND } from "../../services/nativePersistence";
import { RUN_RECORD_MANIFEST_UNAVAILABLE } from "../../services/analysisRunService";
import { HistoricalRunPanel } from "../results/HistoricalRunContext";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const REPLIES = resolve(__dirname, "../../test-support/native-reopen/native_reopen_replies.json");
const REQUESTS = resolve(__dirname, "../../test-support/native-reopen/native_reopen_requests.json");
const RECORD = process.env.NATIVE_REOPEN_RECORD === "1";
/** The handle the replies' `$HANDLE` reads as here (a restored handle's prefix). */
const HANDLE = "restored-stored-result-replay-1";
const READ_HANDLE = "restored-stored-result-replay-2";
/** A disposed handle never resurrects, so each test that opens holds its own. */
const SAVE_HANDLE = "restored-stored-result-replay-3";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  delete (window as Json).__TAURI_INTERNALS__;
});

function substitute(value: unknown, from: string, to: string): unknown {
  if (value === from) return to;
  if (Array.isArray(value)) return value.map((item) => substitute(item, from, to));
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, substitute(item, from, to)]));
  return value;
}
function delivered(reply: Json): unknown {
  if (!reply.ok) throw reply.error;
  if (reply.body === "json") return structuredClone(reply.value);
  const bytes = Buffer.from(reply.base64, "base64");
  return bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
}

/** `override` answers a request on the handle before the recorded replies (a test's own variant). */
async function session(handle = HANDLE, override?: (command: string, args: Json) => unknown) {
  const document = JSON.parse(readFileSync(REPLIES, "utf8")) as Json;
  const { model } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const recorded: Json[] = [];
  const disposed: string[] = [];
  const openReply = substitute(document.open.reply.value, "$HANDLE", handle) as Json;
  const readReply = substitute(document.read.reply.value, "$HANDLE", READ_HANDLE) as Json;
  invokeMock.mockImplementation(async (command: string, args: Json) => {
    switch (command) {
      case "get_local_storage_capability": return storage;
      case "load_design_knowledge": return knowledge;
      case "sync_native_shell_state": return null;
      case "check_model_library_refs": return [];
      case "load_preview_model": return structuredClone(model);
      case "dispose_all_preview_mechanics_results": return { released_results: 0, released_bytes: 0, abandoned_jobs: 0 };
      case "list_model_states": return [];
      case "list_analysis_runs": return [readReply.run];
      case "dispose_preview_mechanics_result": disposed.push(args.jobId); return { job_id: args.jobId, released: true, released_bytes: 1, delivered: false };
      case NATIVE_OPEN_PROJECT_COMMAND:
        expect(args).toEqual(document.open.args);
        return structuredClone(openReply);
      case NATIVE_READ_RUN_COMMAND:
        expect(args).toEqual(document.read.args);
        return structuredClone(readReply);
    }
    if (args?.jobId === handle) {
      const request = substitute(args, handle, "$HANDLE");
      const replaced = override?.(command, request);
      if (replaced !== undefined) return replaced;
      const found = (document.requests as Json[]).find((entry) => entry.command === command && isDeepStrictEqual(entry.args, request));
      if (found) return found.reply.body === "json" ? substitute(delivered(found.reply), "$HANDLE", handle) : delivered(found.reply);
      if (RECORD) {
        // Kept at once, merged with the requests recorded before (each run reaches the next one).
        let kept: Json[] = [];
        try { kept = JSON.parse(readFileSync(REQUESTS, "utf8")).requests; } catch { kept = []; }
        if (!kept.some((entry) => entry.command === command && isDeepStrictEqual(entry.args, request))) kept.push({ command, args: request });
        writeFileSync(REQUESTS, `${JSON.stringify({ document_kind: "swbpipe.desktop.native_reopen_requests", note: "Written by nativeReopen.session.test.tsx (NATIVE_REOPEN_RECORD=1): the requests the session and historical panel send on the restored handle; stored_custody_tests.rs answers each with its actual command.", requests: kept }, null, 1)}\n`);
        recorded.push({ command, args: request });
      }
      throw new Error(`NATIVE_REOPEN_REPLAY: ${command} ${JSON.stringify(request)} has no recorded reply (NATIVE_REOPEN_RECORD=1, then NATIVE_REOPEN_REPLIES_WRITE=1 in stored_custody_tests)`);
    }
    throw new Error(`NATIVE_REOPEN_REPLAY: unexpected ${command}`);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  let current!: WorkspaceSession;
  function Harness() {
    current = useWorkspaceSession();
    const history = current.results.historicalRun;
    return history ? <HistoricalRunPanel key={history.runId} context={history} /> : null;
  }
  const view = render(<Harness />);
  await waitFor(() => expect(current.model.model).not.toBeNull());
  return { state: () => current, document, openReply, readReply, disposed, recorded, view };
}

const called = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command);
const WHOLE_COPIES = ["open_local_project", "read_preview_mechanics_job_rows_all", "read_restored_result_record_all", "take_preview_mechanics_job_result"];

describe("saved and recorded runs reopen into native custody (C1 slice 6)", () => {
  it("opens the stored project with its saved run held natively and inspects it through S and pages", async () => {
    const { state, openReply, disposed, recorded, view } = await session();
    const projectId = openReply.model.project.id as string;
    await act(async () => { await state().project.handleOpenProject(projectId); });
    await waitFor(() => expect(state().results.historicalRun?.designation, String(state().project.projectMessage)).toBe("historical_saved_run"));
    const history = state().results.historicalRun!;
    // A restored handle with its stored origin: never the current result, never a solve.
    expect(history.restored?.handle).toBe(HANDLE);
    expect(history.restored?.origin).toBe("restored_saved_run");
    expect(isNativeResult(history.mechanicsResult)).toBe(true);
    expect(nativeCustody(history.mechanicsResult as never).origin).toBe("restored_saved_run");
    expect(history.findings).toEqual(openReply.stored_run.findings.findings);
    expect(history.runId).toBe(openReply.stored_run.findings.run_id);
    expect(history.rawMechanicsResult).toBeNull();
    expect(history.rawAnalysisRun).toBeNull();
    expect(state().results.result).toBeNull();
    expect(state().results.analysisRun).toBeNull();
    expect(state().results.inputManifest).toBeNull();
    expect(state().project.history.recordRunUnavailableReason).toBe(RUN_RECORD_MANIFEST_UNAVAILABLE);
    // The stored envelope hash agrees with the native recomputation over the stored values.
    await waitFor(() => expect(state().project.projectEnvelopeHashIntegrity?.integrity_status).toBe("verified_match"));
    // The historical panel reads S and a native page of the restored bytes.
    const panel = await screen.findByTestId("historical-run-context");
    expect(panel).toHaveTextContent("HISTORICAL_INPUT_MANIFEST_MISSING");
    expect(panel).toHaveTextContent(`mechanics=${history.mechanicsResult!.status.mechanics}`);
    const results = within(panel).getByTestId("results-panel");
    if (RECORD) {
      await new Promise((settle) => setTimeout(settle, 1500));
      return;
    }
    // The first native page shows the stored result's first published rows.
    const { source } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
    await waitFor(() => expect(results).toHaveTextContent(source.results[0].id));
    expect(called("query_preview_mechanics_job_rows").length).toBeGreaterThan(0);
    expect(called("read_preview_mechanics_job_row_semantics").length).toBeGreaterThan(0);
    // Nothing whole crossed: no legacy open, no full copy, no take.
    for (const command of ["open_local_project", "read_preview_mechanics_job_rows_all", "read_restored_result_record_all", "take_preview_mechanics_job_result"]) expect(called(command), command).toHaveLength(0);
    // The handle is released once, when the session ends.
    expect(disposed).toEqual([]);
    view.unmount();
    await waitFor(() => expect(disposed).toEqual([HANDLE]));
  });

  it("saves the reopened project from its restored handle: nothing whole crosses and the run stays historical", async () => {
    const { state, openReply, disposed, view, document } = await session(SAVE_HANDLE);
    await act(async () => { await state().project.handleOpenProject(openReply.model.project.id); });
    await waitFor(() => expect(state().results.historicalRun?.restored?.handle, String(state().project.projectMessage)).toBe(SAVE_HANDLE));
    await waitFor(() => expect(state().project.projectEnvelopeHashIntegrity?.integrity_status).toBe("verified_match"));
    const before = state().results.historicalRun!;
    await act(async () => { await state().project.handleSaveProject(); });
    if (RECORD) return;
    await waitFor(() => expect(state().project.projectOperation, String(state().project.projectMessage)).toBe("save"));
    // The request: the handle, its stored source, the envelope basis and the small fields; no result,
    // record, model hash or envelope hash.
    const sent = called(NATIVE_SAVE_RESTORED_PROJECT_COMMAND);
    expect(sent).toHaveLength(1);
    const args = sent[0][1] as Json;
    expect(args.jobId).toBe(SAVE_HANDLE);
    expect(args.expectedSource).toEqual(openReply.stored_run.source);
    expect(args.envelopeBasis).toEqual({ payload_sha256: openReply.stored_run.findings.envelope_payload_hash, evidence: openReply.project_envelope_hash });
    expect(args.expectedBase).toBe(openReply.project_envelope_hash.value);
    for (const member of ["modelHash", "projectEnvelopeHash", "mechanicsResult", "analysisRun", "intakeText", "inputManifest"]) expect(args, member).not.toHaveProperty(member);
    // The actual command's reply: saved with the stored run, the stored evidence kept (unchanged project).
    const reply = (document.requests as Json[]).find((entry) => entry.command === NATIVE_SAVE_RESTORED_PROJECT_COMMAND)!.reply.value;
    expect(reply.result.outcome).toBe("saved");
    expect(reply.restored.envelope_basis_kept).toBe(true);
    expect(state().project.projectMessage).toBe(reply.summary.message);
    // The history keeps the same handle and stored source (historical provenance), with the findings of the
    // project as stored; nothing becomes the current result, and the save disposes nothing.
    const after = state().results.historicalRun!;
    expect(after.designation).toBe("historical_saved_run");
    expect(after.restored).toBe(before.restored);
    expect(after.mechanicsResult).toBe(before.mechanicsResult);
    expect(after.findings).toEqual(reply.restored.findings.findings);
    expect(after.envelopeHash).toEqual(reply.project_envelope_hash);
    expect(after.envelopePayloadHash).toBe(reply.restored.envelope_payload_hash);
    expect(state().results.result).toBeNull();
    expect(state().project.modelEdited).toBe(false);
    await waitFor(() => expect(state().project.projectEnvelopeHashIntegrity?.integrity_status).toBe("verified_match"));
    for (const command of [...WHOLE_COPIES, "save_local_project", "create_local_project", "save_preview_mechanics_job_local_project"]) expect(called(command), command).toHaveLength(0);
    expect(disposed).toEqual([]);
    view.unmount();
    await waitFor(() => expect(disposed).toEqual([SAVE_HANDLE]));
  });

  it("keeps the history and the handle when the re-save is refused or saves the model only", async () => {
    const SAVE = NATIVE_SAVE_RESTORED_PROJECT_COMMAND;
    const document = JSON.parse(readFileSync(REPLIES, "utf8")) as Json;
    const actual = (document.requests as Json[]).find((entry) => entry.command === SAVE).reply.value;
    // Variants of the actual reply: a binding refusal, and a model-only save (the run not stored).
    const modelOnly = { ...structuredClone(actual), result: { outcome: "not_saved", code: "PROJECT-SAVE-RESULT-TOO-LARGE", message: "PROJECT-SAVE-RESULT-TOO-LARGE (result_bytes=1, record_bytes=1, row_bytes=3, limit_bytes=2): the result was not saved.", result_bytes: 1, record_bytes: 1, row_bytes: 3, limit_bytes: 2, refused: { what: "project row", bytes: 3 } } };
    modelOnly.restored = { ...modelOnly.restored, findings: null, envelope_basis_kept: false };
    modelOnly.summary = { ...modelOnly.summary, message: `Saved current project model snapshot without external file copies. ${modelOnly.result.message}` };
    for (const [variant, handle, answer] of [
      ["refused", "restored-stored-result-replay-4", () => Promise.reject("NATIVE-PERSISTENCE-CUSTODY: the envelope basis the session names is not one established; nothing was written.")],
      ["model only", "restored-stored-result-replay-5", () => Promise.resolve(substitute(modelOnly, "$HANDLE", "restored-stored-result-replay-5"))]
    ] as const) {
      const { state, openReply, disposed, view } = await session(handle, (command) => command === SAVE ? answer() : undefined);
      await act(async () => { await state().project.handleOpenProject(openReply.model.project.id); });
      await waitFor(() => expect(state().results.historicalRun?.restored?.handle).toBe(handle));
      const before = state().results.historicalRun!;
      await act(async () => { await state().project.handleSaveProject(); });
      await waitFor(() => expect(state().project.projectOperation, variant).toBe(variant === "refused" ? "save_failed" : "save"));
      expect(state().project.projectMessage).toContain(variant === "refused" ? "NATIVE-PERSISTENCE-CUSTODY" : "PROJECT-SAVE-RESULT-TOO-LARGE");
      // The run stays shown and held as it was; nothing becomes current; nothing whole crosses.
      expect(state().results.historicalRun, variant).toBe(before);
      expect(state().results.result).toBeNull();
      expect(disposed, variant).toEqual([]);
      for (const command of WHOLE_COPIES) expect(called(command), command).toHaveLength(0);
      view.unmount();
      await waitFor(() => expect(disposed).toEqual([handle]));
      cleanup();
      invokeMock.mockReset();
      resetSessionResultCustodyForTests();
    }
  });

  it("checks a recorded run natively and releases its handle at once", async () => {
    const { state, openReply, readReply, disposed } = await session();
    await act(async () => { await state().project.handleOpenProject(openReply.model.project.id); });
    await waitFor(() => expect(state().project.history.runs?.length).toBe(1));
    let checked = false;
    await act(async () => { checked = await state().project.history.checkRun(readReply.run.recording_id); });
    expect(checked).toBe(true);
    const check = state().project.history.runCheck!;
    expect(check.context?.findings).toEqual(readReply.restored.findings.findings);
    expect(check.context?.designation).toBe("recorded_run");
    expect(check.context?.mechanicsResult).toBeNull();
    await waitFor(() => expect(disposed).toContain(READ_HANDLE));
    expect(called("read_analysis_run")).toHaveLength(0);
  });
});

/** C1 slice 6: saved and recorded runs reopen into native custody, in the actual workspace session, on
 * the actual Rust commands' replies (`test-support/native-reopen/native_reopen_replies.json`, written by
 * `src-tauri/src/stored_custody_tests.rs`): the session's own native Save Project and Record Run
 * requests (`native_persistence_replies.json`) on a real store, a session reset, then the native open,
 * every request the historical panel sends on the restored handle (S, pages, semantics), and the
 * recorded run's native read. Each request must equal a recorded request exactly (its handle written
 * `$HANDLE`); anything else fails. With NATIVE_REOPEN_RECORD=1 the panel's requests are written to
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

async function session() {
  const document = JSON.parse(readFileSync(REPLIES, "utf8")) as Json;
  const { model } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const recorded: Json[] = [];
  const disposed: string[] = [];
  const openReply = substitute(document.open.reply.value, "$HANDLE", HANDLE) as Json;
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
    if (args?.jobId === HANDLE) {
      const request = substitute(args, HANDLE, "$HANDLE");
      const found = (document.requests as Json[]).find((entry) => entry.command === command && isDeepStrictEqual(entry.args, request));
      if (found) return delivered(found.reply);
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
    expect(state().project.projectEnvelopeHashIntegrity?.status ?? "verified").not.toBe("mismatch");
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

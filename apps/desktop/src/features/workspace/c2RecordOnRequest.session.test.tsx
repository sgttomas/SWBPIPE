/** The session's C2 loader (`loadC2Full`) gives a native result's run record only to a consumer that
 * asks for it. A consumer that does not ask (the sample proposal, the user-released package, the
 * gates given no record) is neither refused for the record's stored length nor made to wait for
 * the record's build; one that asks gets today's record, checked against the native preview's
 * checksums. The result's own 64 MiB limit refuses every consumer. Unit transport replay of
 * recorded producer bytes through mocked IPC; NOT a native UI qualification witness. */
import { act, renderHook, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import modelFixture from "../../../../../core/product_physics/tests/fixtures/preview_physics_invented_model.json";
import sourceFixture from "../../../../../fixtures/results/preview_physics_invented_sparse.json";
import { createSolveJobTransport, solveBytes } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import * as previewService from "../../services/previewService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import * as nativeResult from "../../services/nativeResult";
import { FullResultRefused, isNativeAnalysisRun, isNativeResult, RESULT_DETAIL_WEBVIEW_LIMIT_BYTES } from "../../services/nativeResult";
import type { C2FullResult } from "../results/C2ResultGate";
import { useWorkspaceSession } from "./workspaceSession";

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

let sequence = 0;
async function solvedSession(options: { fullCopyLimitBytes?: number } = {}) {
  const storage = await getLocalStorageCapability();
  const knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((sent) => {
    if (!isDeepStrictEqual({ model: sent.model, mode: sent.mode }, { model: modelFixture, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return solveBytes(sourceFixture);
  }, { jobIdPrefix: `unit-c2-record-on-request:${++sequence}`, ...options });
  invokeMock.mockImplementation(async (command: string, args?: unknown, invokeOptions?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(modelFixture);
    if (transport.handles(command)) return transport.invoke(command, args, invokeOptions);
    throw new Error(`REPLAY_COMMAND_UNSUPPORTED: ${command}`);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(hook.result.current.model.modelHash).not.toBeNull());
  await act(async () => { await hook.result.current.results.handleRun(); });
  await waitFor(() => expect(hook.result.current.results.solveJob.state).toBe("completed"));
  expect(isNativeResult(hook.result.current.results.result)).toBe(true);
  expect(isNativeAnalysisRun(hook.result.current.results.analysisRun)).toBe(true);
  const copies = () => transport.commands().filter((name) => name === "read_preview_mechanics_job_rows_all").length;
  const load = async (request?: { record: boolean }): Promise<C2FullResult> => {
    let full: C2FullResult | null = null;
    await act(async () => { full = await hook.result.current.results.loadC2Full(hook.result.current.results.result!, request); });
    return full!;
  };
  return { hook, load, copies };
}

/** The native preview of every record, with its stored length above the webview's limit. */
function recordAboveLimit() {
  const preview = nativeResult.nativeRunPreview;
  return vi.spyOn(nativeResult, "nativeRunPreview").mockImplementation((run) => ({ ...preview(run), record_bytes: RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1 }));
}
const failure = (promise: Promise<unknown>) => promise.then(() => null, (error: unknown) => error);

describe("a consumer that does not ask for the run record (the change)", () => {
  it("gets the full copy and no record, unrefused by the record's length and without building it", async () => {
    const { hook, load, copies } = await solvedSession();
    const builds = vi.spyOn(previewService, "buildAnalysisRunPreview");
    const previews = recordAboveLimit();
    const full = await load({ record: false });
    expect(full.result).toEqual(sourceFixture);
    expect(full.analysisRun).toBeNull();
    expect(builds).not.toHaveBeenCalled();
    // The record's length was not read for it.
    expect(previews).not.toHaveBeenCalled();
    expect(copies()).toBe(1);
    // The sample proposal reads the copy alone, so the record's length does not refuse it.
    await act(async () => { await hook.result.current.results.handleProposal(); });
    expect(hook.result.current.results.proposal).not.toBeNull();
    expect(builds).not.toHaveBeenCalled();
    expect(copies()).toBe(1);
  }, 60_000);
});

describe("a consumer that asks for the run record (kept)", () => {
  it("is refused by name when the record's stored length is above the limit", async () => {
    const { load } = await solvedSession();
    const builds = vi.spyOn(previewService, "buildAnalysisRunPreview");
    recordAboveLimit();
    for (const request of [undefined, { record: true }]) {
      const error = await failure(load(request));
      expect(error).toBeInstanceOf(FullResultRefused);
      expect(error).toMatchObject({ code: "RESULT-DETAIL-WEBVIEW-LIMIT", payloadKind: "record", resultBytes: RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1, limitBytes: RESULT_DETAIL_WEBVIEW_LIMIT_BYTES });
    }
    expect(builds).not.toHaveBeenCalled();
  }, 60_000);

  it("gets the record built on the copy with the native preview's checksums, and a mismatch fails by name", async () => {
    const { hook, load } = await solvedSession();
    const preview = nativeResult.nativeRunPreview(hook.result.current.results.analysisRun as nativeResult.NativeAnalysisRun).record;
    const claim = (hashes: readonly { payload_scope: string; value: string }[], scope: string) => hashes.find((hash) => hash.payload_scope === scope)?.value;
    const build = previewService.buildAnalysisRunPreview;
    const mismatched = vi.spyOn(previewService, "buildAnalysisRunPreview").mockImplementation(async (...args) => {
      const built = await build(...args);
      return { ...built, analysis_run: { ...built.analysis_run, hashes: built.analysis_run.hashes.map((hash) => hash.payload_scope === "received_result" ? { ...hash, value: "0".repeat(64) } : hash) } };
    });
    expect(String(await failure(load({ record: true })))).toContain("C2-RECORD-MISMATCH");
    mismatched.mockRestore();
    // A failed build is not kept: the next request builds afresh and gets the native record.
    const full = await load({ record: true });
    expect(full.analysisRun).not.toBeNull();
    for (const scope of ["analysis_run_record", "received_result"]) {
      expect(claim(full.analysisRun!.analysis_run.hashes, scope)).toBeTruthy();
      expect(claim(full.analysisRun!.analysis_run.hashes, scope)).toBe(claim(preview.analysis_run.hashes, scope));
    }
  }, 60_000);
});

describe("the result's own limit (kept)", () => {
  it("refuses every consumer, with or without the record", async () => {
    const length = new TextEncoder().encode(solveBytes(sourceFixture)).byteLength;
    const { load } = await solvedSession({ fullCopyLimitBytes: length - 1 });
    for (const request of [{ record: false }, { record: true }]) {
      const error = await failure(load(request));
      expect(error).toBeInstanceOf(FullResultRefused);
      expect(error).toMatchObject({ code: "RESULT-DETAIL-WEBVIEW-LIMIT", payloadKind: "result", resultBytes: length, limitBytes: length - 1 });
    }
  }, 60_000);
});

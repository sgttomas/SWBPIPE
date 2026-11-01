// C1 slice 4 (DEL-04-05 `c1-native-custody-plan.md`): the connecting path of a fresh solve held
// natively, through the actual raw-body ingress, the replay transport and the actual Rust commands'
// recorded replies (`test/nativeReplayStore.ts`, `src-tauri/src/native_replay_tests.rs`): start with
// native custody, poll, the native verdict (delivered as a parsed JSON value), the registration's
// reader validations, the standing, S (an ArrayBuffer) and the handle. Nothing takes the result,
// and the handle refuses every full-row access. Unit transport replay; NOT native UI qualification.
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { awaitPreviewMechanicsJob, disposeDeliveredMechanicsResult, readNativeFreshResult, resetSessionResultCustodyForTests, startPreviewMechanicsJob } from "./previewService";
import { decodeNativeVerdict, FullResultRefused, isNativeResult, loadFullResult, nativeCustody, NativeResultRowsGuarded, resetNativeResultsForTests } from "./nativeResult";
import { createNativeMechanicsReplay, createSolveJobTransport, nativeMechanicsReplayPair, REPLAY_COMMANDS } from "../test/nativeMechanicsReplay";
import { readerDispatch, rememberReceivedText } from "../features/results/resultReader";
import type { MechanicsResult } from "../types";

type Replay = ReturnType<typeof createNativeMechanicsReplay>;
function install(replay: { invoke: Replay["invoke"] }) {
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
    if (command === "dispose_all_preview_mechanics_results") return Promise.resolve({ released_results: 0, released_bytes: 0, abandoned_jobs: 0 });
    if (REPLAY_COMMANDS.includes(command)) return replay.invoke(command, args, options);
    return Promise.reject(new Error(`UNEXPECTED_COMMAND: ${command}`));
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
}
beforeEach(() => { resetSessionResultCustodyForTests(); resetNativeResultsForTests(); });
afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("C1 slice 4: a fresh solve's result is read natively and never taken", () => {
  for (const profile of ["preview", "pressure"] as const) {
    it(`${profile}: verdict, registration, standing and S through the actual ingress and the actual commands' replies`, async () => {
      const replay = createNativeMechanicsReplay({ profile });
      install(replay);
      const pair = nativeMechanicsReplayPair("sparse_interactive", { profile });
      const started = await startPreviewMechanicsJob(pair.model, "sparse_interactive", { custody: "native" });
      if (started.mode !== "backend_job") throw new Error("not a backend job");
      const receipt = started;
      const terminal = await awaitPreviewMechanicsJob(receipt.job_id);
      expect(terminal.state).toBe("completed");
      const reading = await readNativeFreshResult(receipt.job_id, terminal.result_bytes ?? 0, { standingModel: pair.model, summaryModel: pair.model });
      const native = reading.result;
      expect(isNativeResult(native)).toBe(true);
      expect(reading.unregistered).toBeNull();
      // The oracle: the full rows read by the existing (wasm) readers, in the test only.
      const full = JSON.parse(pair.bytes) as MechanicsResult;
      rememberReceivedText(full, pair.bytes);
      const legacy = readerDispatch(full, pair.model).standing;
      expect(reading.standing.eligible).toBe(legacy.eligible);
      expect(reading.standing.findings).toEqual([...legacy.findings]);
      expect(nativeCustody(native).verdict.route).toBe(readerDispatch(full).route);
      expect(nativeCustody(native).summary.header.row_count).toBe(full.results.length);
      expect(native.run_id).toBe(full.run_id);
      expect(native.model_ref).toBe(full.model_ref);
      expect(native.diagnostics).toEqual(full.diagnostics);
      // The handle holds no rows and refuses every full-row access, by name.
      expect(() => (native as unknown as MechanicsResult).results).toThrow(NativeResultRowsGuarded);
      expect(() => JSON.stringify(native)).toThrow("NATIVE-RESULT-ROWS-GUARDED");
      expect(() => structuredClone(native)).toThrow();
      expect(() => readerDispatch(native, undefined, undefined).route).not.toThrow();
      // Nothing took the result; the job stays held natively until disposal.
      const sent = invokeMock.mock.calls.map((call) => call[0]);
      expect(sent).not.toContain("take_preview_mechanics_job_result");
      expect(sent).not.toContain("read_preview_mechanics_job_rows_all");
      expect(sent).toEqual(expect.arrayContaining(["read_preview_mechanics_job_verdict", "read_preview_mechanics_job_summary"]));
      expect(replay.nativeHeld()).toEqual([receipt.job_id]);
      await disposeDeliveredMechanicsResult(native);
      expect(replay.nativeHeld()).toEqual([]);
    });
  }

  it("reads the verdict as Tauri delivers a JSON body (a parsed value), and refuses text or bytes", () => {
    const value = { ok: true, route: "preview_physics", binding: null, error: null, standing: { route: "preview_physics", token: "needs_recompute", status: "needs_recompute", eligible: false, findings: ["X"] } };
    expect(decodeNativeVerdict(value).route).toBe("preview_physics");
    expect(() => decodeNativeVerdict(JSON.stringify(value))).toThrow("NATIVE-VERDICT-MALFORMED: the delivery is text");
    expect(() => decodeNativeVerdict(new TextEncoder().encode(JSON.stringify(value)).buffer)).toThrow("NATIVE-VERDICT-MALFORMED");
  });

  it("C2's full copy: refused by name one byte above the limit, delivered whole at it", async () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    const length = new TextEncoder().encode(pair.bytes).byteLength;
    for (const [limit, refused] of [[length - 1, true], [length, false]] as const) {
      resetNativeResultsForTests();
      const transport = createSolveJobTransport(() => pair.bytes, { fullCopyLimitBytes: limit });
      install({ invoke: transport.invoke });
      const started = await startPreviewMechanicsJob(pair.model, "sparse_interactive", { custody: "native" });
      if (started.mode !== "backend_job") throw new Error("not a backend job");
      const receipt = started;
      await awaitPreviewMechanicsJob(receipt.job_id);
      const native = (await readNativeFreshResult(receipt.job_id, length, { standingModel: pair.model, summaryModel: pair.model })).result;
      const load = loadFullResult(native, async (text) => JSON.parse(text) as MechanicsResult);
      if (refused) {
        const error = await load.catch((caught) => caught);
        expect(error).toBeInstanceOf(FullResultRefused);
        expect(error).toMatchObject({ code: "RESULT-DETAIL-WEBVIEW-LIMIT", resultBytes: length, limitBytes: length - 1 });
      } else {
        expect((await load).run_id).toBe(native.run_id);
      }
      await disposeDeliveredMechanicsResult(native);
    }
  });
});

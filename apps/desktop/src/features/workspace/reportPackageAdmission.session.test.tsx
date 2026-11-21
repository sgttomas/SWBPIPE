/** DEL-08-01 REQ-024, "Package and user release": what the report package's four commands do,
 * through the workspace session, for a result the rendered report does not take, and the
 * user-release dialog where the report frame's statements cannot be obtained.
 *
 * The four commands are Save Report Package… and Save Shareable Report Package…, each from the
 * panel's button (the session's handler the button calls) and from the File menu. For a
 * pressure-free result that is not numerically eligible, an unsolved result and an unreadable
 * result, the session gives no package source (`renderableSolvedResult`, from which App enables
 * the package controls), and each command leaves its refusal, word for word, with no dialog and
 * no save. For an exact-route result whose frame statements the native host refuses, the
 * shareable command's dialog shows the refusal in place of the statement and saves nothing.
 *
 * Unit transport replay of committed producer bytes through mocked IPC (the native replies are
 * the recorded corpus's); NOT a native UI qualification witness. Every value is invented. */
import { act, cleanup, render, renderHook, screen, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { REPORT_FRAME_STATEMENTS_UNAVAILABLE } from "../report/reportFrameStatements";
import { UserReleaseDialog, USER_RELEASE_FRAME_STATEMENTS_REFUSAL } from "../report/UserReleaseDialog";
import { useWorkspaceSession } from "./workspaceSession";
import type { PreviewModel } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Mode = "sparse_interactive" | "dense_scrutiny";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const WITNESS = "fixtures/results/invented/agent_solve_witness";
const NOT_ELIGIBLE = "Not saved: report-package save needs the current, numerically eligible mechanics result.";
const SOLVE_FIRST = "Not saved: report-package save needs the current mechanics result and analysis-run record. Solve first.";

let sequence = 0;
/** The session over the replayed solve of `model` returning `bytes`, solved in `mode`, until the
 * job reaches `state`. `verdict`, when given, answers the verdict command in place of the replay. */
async function solved(model: PreviewModel, bytes: string, state: "completed" | "failed", verdict?: (args: Json) => unknown) {
  const mode: Mode = "sparse_interactive";
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-report-package-admission:${++sequence}` });
  invokeMock.mockImplementation(async (command: string, args?: Json, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(model);
    if (command === "save_report_package") throw new Error("INVENTED-SAVE-NOT-EXPECTED");
    if (verdict && command === "read_preview_mechanics_job_verdict") {
      const answer = verdict(args);
      if (answer !== undefined) return answer;
    }
    return transport.invoke(command, args, options);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  const rendered = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(rendered.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(rendered.result.current.model.modelHash).not.toBeNull());
  await act(async () => { await rendered.result.current.results.handleRun(); });
  await waitFor(() => expect(rendered.result.current.results.solveJob.state, rendered.result.current.results.solveJob.error_message ?? undefined).toBe(state));
  return rendered;
}

const saveCalls = () => invokeMock.mock.calls.filter(([command]) => command === "save_report_package");

/** The three states, each from committed producer bytes. */
const STATES: [string, () => { model: PreviewModel; bytes: string; state: "completed" | "failed" }, string][] = [
  ["a pressure-free result that is not numerically eligible", () => {
    // The pressure-free pair, its one case marked sensitive by the solver's own quality fields.
    const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    const source = JSON.parse(bytes);
    source.numerical_quality.status = "sensitive";
    source.numerical_quality.cases[0].solve_quality = "sensitive";
    return { model, bytes: JSON.stringify(source), state: "completed" };
  }, NOT_ELIGIBLE],
  ["an unsolved result", () => {
    // The producer's blocked document (MODEL_INCOMPLETE), its exact bytes.
    const request = JSON.parse(text(`${WITNESS}/blocked_retained_evaluation.request.json`));
    return { model: request.model as PreviewModel, bytes: text(`${WITNESS}/blocked_retained_evaluation-sparse_interactive.raw.json`), state: "completed" };
  }, NOT_ELIGIBLE],
  ["an unreadable result", () => {
    // The exact pair with forged numerical evidence: the reader refuses it, and the session holds no result.
    const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
    const source = JSON.parse(bytes);
    source.contract_evidence.exact_cases[0].pipe_sections[0].nominal_section.As_m2 *= 1.01;
    return { model, bytes: JSON.stringify(source), state: "failed" };
  }, SOLVE_FIRST]
];

describe("the report package's four commands for a result the rendered report does not take (DEL-08-01 REQ-024)", () => {
  it.each(STATES)("%s: no package source, and each command refuses word for word, with no dialog and no save", async (_name, make, message) => {
    const { model, bytes, state } = make();
    const { result: hook } = await solved(model, bytes, state);
    const results = () => hook.current.results;
    if (state === "completed") {
      expect(results().result).not.toBeNull();
      expect(results().currentSolvedResult).toBeNull();
    } else {
      expect(results().result).toBeNull();
    }
    // App enables the panel's package buttons and the menu's package items from this source only.
    expect(results().renderableSolvedResult).toBeNull();
    const commands: [string, () => Promise<void> | void][] = [
      ["Save Report Package… (button)", () => results().handleSaveReportPackage()],
      ["Save Report Package… (File menu)", () => hook.current.chrome.runMenuCommand("file.save-report-package")],
      ["Save Shareable Report Package… (button)", () => results().requestSaveShareableReportPackage()],
      ["Save Shareable Report Package… (File menu)", () => hook.current.chrome.runMenuCommand("file.save-shareable-report-package")]
    ];
    for (const [command, run] of commands) {
      await act(async () => { await run(); });
      await waitFor(() => expect(results().reportPackageRoute, command).toEqual({ route: "not_requested", diagnostic: message }));
      expect(results().userReleaseRequest, command).toBeNull();
      expect(results().reportPackageBusy, command).toBe(false);
    }
    expect(saveCalls()).toHaveLength(0);
  }, 60000);
});

describe("the user-release dialog where the frame's statements cannot be obtained (DEL-08-01 REQ-024)", () => {
  it("refuses in place of the statement, offers no statement and saves nothing", async () => {
    const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
    const refusal = `${REPORT_FRAME_STATEMENTS_UNAVAILABLE}: invented refusal of the native host`;
    const { result: hook } = await solved(model, bytes, "completed", (args) => {
      if (args?.operation === "report_frame_statements") throw { code: "INVALID_REQUEST", message: refusal };
      return undefined;
    });
    expect(hook.current.results.renderableSolvedResult).not.toBeNull();
    await act(async () => { await hook.current.results.requestSaveShareableReportPackage(); });
    const request = hook.current.results.userReleaseRequest!;
    expect(request).not.toBeNull();
    expect(request.refusal?.kind).toBe("frame_statements");
    expect(request.refusal?.reason).toContain(refusal);
    expect(request.frameStatements).toEqual([]);
    render(<UserReleaseDialog request={request} />);
    const dialog = screen.getByTestId("user-release-dialog");
    expect(dialog).toHaveTextContent(USER_RELEASE_FRAME_STATEMENTS_REFUSAL);
    expect(screen.queryByTestId("user-release-statement")).toBeNull();
    expect(screen.queryByTestId("user-release-frame-statements")).toBeNull();
    expect(screen.getByTestId("user-release-confirm")).toBeDisabled();
    act(() => screen.getByTestId("user-release-confirm").click());
    act(() => request.cancel());
    expect(hook.current.results.userReleaseRequest).toBeNull();
    expect(saveCalls()).toHaveLength(0);
  }, 60000);
});

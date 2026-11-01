/** RT-U4: a solve the native host refused because its result is larger than the
 * desktop can display (`PREVIEW-SOLVE-RESULT-TOO-LARGE`, `MAX_DISPLAYABLE_RESULT_BYTES`
 * in `solve_jobs.rs`). Through the actual workspace session, with the native job
 * transport replay of committed producer bytes and a fake of the native store's
 * commands (unit transport replay; NOT a native UI witness). The native refusal
 * itself is covered by the Rust tests in `solve_jobs.rs`. All data are invented. */
import { act, cleanup, render, screen, waitFor, within } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock, { nativeReopen: "actual" }) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { RUN_RECORD_NO_RUN } from "../../services/analysisRunService";
import { SolvePanel } from "../solve/SolvePanel";
import { SolveProgressPanel } from "../solve/SolveProgressPanel";
import { initialSolveJob } from "./solveJobAudit";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";
import type { LocalProjectEnvelope } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

const TOO_LARGE = { code: "PREVIEW-SOLVE-RESULT-TOO-LARGE", result_bytes: 3_950_000_000, limit_bytes: 67_108_864 };
const MESSAGE = "The solve finished, but its result is 3.68 GiB, above the desktop's display limit of 64 MiB. The per-station result detail of this model and its load cases is too large for the desktop to display. No result is shown or kept. A solve covers every load case and combination in the model, so to come under the limit: remove the load cases and combinations this review does not need (Load Cases section); or solve the cases in smaller groups, each in its own copy of the model document (File > Save Model Document…), with the combinations in a copy holding only the cases they use; or split a large model into parts. Code: PREVIEW-SOLVE-RESULT-TOO-LARGE (result_bytes=3950000000, limit_bytes=67108864).";

let sequence = 0;
/** The session over the replayed solve of the committed pressure pair; `refuse()` makes later jobs end refused for their size. */
async function session() {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-result-too-large:${++sequence}` });
  const saved: Json[] = [];
  let refusing = false;
  /** What `open_local_project` returns, and a refusal `save_local_project` answers with, when set. */
  const native: { opened: Json | null; openedNative: Json | null; saveRefusal: string | null } = { opened: null, openedNative: null, saveRefusal: null };
  invokeMock.mockImplementation(async (command: string, args: Json, options?: unknown) => {
    switch (command) {
      case "get_local_storage_capability": return storage;
      case "load_design_knowledge": return knowledge;
      case "sync_native_shell_state": return null;
      case "check_model_library_refs": return [];
      case "load_preview_model": if (args === undefined) return structuredClone(model); break;
      case "find_local_project": return null;
      case "list_model_states": return [];
      case "list_analysis_runs": return [];
      case "record_analysis_run": throw new Error("record_analysis_run must not be called");
      case "open_local_project": return structuredClone(native.opened);
      case "open_local_project_in_native_custody": return structuredClone(native.openedNative);
      case "save_local_project": {
        const request = args.request as Json;
        saved.push(request);
        if (native.saveRefusal) throw native.saveRefusal;
        return {
          summary: { project_id: request.model.project.id, project_name: request.model.project.name, message: "Saved current project model snapshot without external file copies." },
          model: request.model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: request.mechanics_result ?? null,
          analysis_run: request.analysis_run ?? null, model_hash: request.model_hash, project_envelope_hash: { value: `sha256:saved#${saved.length}` }, model_document_version: null
        } as unknown as LocalProjectEnvelope;
      }
      case "poll_preview_mechanics_job": {
        const answer = await transport.invoke(command, args, options);
        if (!refusing) return answer;
        // As the native host reports a job refused for its result's size: no result, no lease, no memory-pool refusal.
        return { ...answer, state: "refused", result_available: false, result_bytes: null, resource_refusal: null, result_too_large: TOO_LARGE,
          error_message: `PREVIEW-SOLVE-RESULT-TOO-LARGE: ${answer.job_id}'s result is 3950000000 bytes, above the 67108864 bytes the desktop can display; it was discarded without being read` };
      }
    }
    return transport.invoke(command, args, options);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  let current!: WorkspaceSession;
  function Harness() {
    current = useWorkspaceSession();
    return null;
  }
  render(<Harness />);
  await waitFor(() => expect(current.model.model).not.toBeNull());
  await waitFor(() => expect(current.model.modelHash).not.toBeNull());
  return { state: () => current, saved, transport, native, refuse: () => { refusing = true; } };
}

const calls = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command);

describe("a solve refused because its result is too large to display (RT-U4)", () => {
  it("shows the plain reason with the size, the limit, the cause and the advice; no result enters the session; Record Run has no run; Save Project saves no result", async () => {
    const { state, saved, transport, refuse } = await session();
    // A solved result held before the refused run is replaced by nothing.
    await act(async () => { await state().results.handleRun(); });
    await waitFor(() => expect(state().results.solveJob.state).toBe("completed"));
    expect(state().results.result).not.toBeNull();
    const [firstJob] = transport.nativeHeld();
    refuse();
    await act(async () => { await state().results.handleRun(); });
    const job = state().results.solveJob;
    expect(job.state).toBe("refused");
    expect(job.events.at(-1)).toMatchObject({ event_id: "solve-preview-result-too-large", state: "refused", message: MESSAGE, result_available: false, result_row_count: 0 });
    expect(job.error_message).toBe(MESSAGE);
    // Not a memory-pool refusal.
    expect(job.resource.refusal).toBeNull();
    // Nothing was taken for the refused job (nor, under C1 native custody, for the earlier one,
    // held natively), and no result or run record is held.
    expect(calls("take_preview_mechanics_job_result")).toHaveLength(0);
    expect(state().results.result).toBeNull();
    expect(state().results.currentSolvedResult).toBeNull();
    expect(state().results.analysisRun).toBeNull();
    expect(state().results.inputManifest).toBeNull();
    expect(state().results.running).toBe(false);
    // The earlier result (held natively) was released when the new run replaced it. (The refused
    // job's reply is emulated here: a native-custody job has no displayable limit, so the host
    // refuses only a webview-custody result for its size.)
    await waitFor(() => expect(transport.disposed()).toEqual([firstJob]));

    const { model } = state().model;
    const { result, analysisRun, running, solverMode } = state().results;
    const view = render(<SolvePanel analysisRun={analysisRun} model={model!} result={result} running={running} solveJob={job}
      solverMode={solverMode} onCancel={() => {}} onRun={() => {}} onSolverModeChange={() => {}} />);
    const shown = within(view.container).getByTestId("solve-job-result-too-large");
    expect(shown).toHaveTextContent(MESSAGE);
    expect(shown.textContent).not.toMatch(/UTF-8/);
    expect(within(view.container).queryByTestId("solve-job-resource-refusal")).toBeNull();
    expect(within(view.container).queryByTestId("solve-job-error")).toBeNull();
    expect(within(view.container).getByTestId("readiness-mechanics")).toHaveTextContent(
      "Solve job state: refused; the result is too large for the desktop to display, so no result was kept");
    expect(within(view.container).getByTestId("readiness-mechanics")).not.toHaveTextContent("memory pool");
    view.unmount();

    // Record Run: there is no run to record, and nothing is recorded.
    expect(state().project.history.recordRunUnavailableReason).toBe(RUN_RECORD_NO_RUN);
    let recorded = true;
    await act(async () => { recorded = await state().project.history.recordRun({ labels: { run_name: "R", tags: [], notes: [], assumptions: [] }, snapshotName: "After solve" }); });
    expect(recorded).toBe(false);
    expect(calls("record_analysis_run")).toHaveLength(0);

    // Save Project saves the model without a mechanics result or run record.
    await act(async () => { await state().project.handleSaveProject(); });
    expect(saved).toHaveLength(1);
    expect(saved[0].model.project.id).toBe(model!.project.id);
    expect(saved[0].mechanics_result ?? null).toBeNull();
    expect(saved[0].analysis_run ?? null).toBeNull();
  });

  it("opens a project whose saved run the native host could not hold: the model opens, the person is told, nothing is hashed or kept, and a save in place is refused", async () => {
    // C1 slice 6: a saved run is reopened into native custody, never withheld for its length alone; when
    // the memory pool does not admit holding it, the open refuses the run by name (the actual command's
    // reply on a real store, `native_reopen_replies.json`) and the model opens for editing.
    const { state, saved, native } = await session();
    const document = JSON.parse(readFileSync(resolve(__dirname, "../../test-support/native-reopen/native_reopen_replies.json"), "utf8"));
    const reply = document.open_refused.reply.value;
    native.openedNative = reply;
    await act(async () => { await state().project.handleOpenProject(reply.model.project.id); });
    expect(state().model.model?.project.id).toBe(reply.model.project.id);
    expect(state().results.result).toBeNull();
    expect(state().results.analysisRun).toBeNull();
    expect(state().results.historicalRun).toBeNull();
    expect(reply.stored_run.message).toMatch(/^STORED-RESULT-RESTORE-REFUSED: project `[^`]+`'s stored run \(result \d+ bytes/);
    expect(state().project.projectMessage).toBe(`${reply.summary.message} ${reply.stored_run.message}`);
    // The refused run is not hashed as if the project held no run.
    await waitFor(() => expect(state().project.projectEnvelopeHashIntegrity?.integrity_status).toBe("hash_recompute_unavailable"));
    // A save in place would replace the stored run with none: refused by the session, nothing sent.
    await act(async () => { await state().project.handleSaveProject(); });
    expect(saved).toHaveLength(0);
    expect(state().project.projectMessage).toContain("STORED-RESULT-RESTORE-REFUSED: Not saved.");
    expect(state().project.projectMessage).toContain("The stored run is kept unchanged.");
  });

  it("shows the reason in the solve progress panel as the run ends", async () => {
    const { state, refuse } = await session();
    refuse();
    const view = render(<SolveProgressPanel solveJob={initialSolveJob()} running={false} onCancel={() => {}} />);
    view.rerender(<SolveProgressPanel solveJob={initialSolveJob()} running onCancel={() => {}} />);
    await act(async () => { await state().results.handleRun(); });
    view.rerender(<SolveProgressPanel solveJob={state().results.solveJob} running={false} onCancel={() => {}} />);
    expect(screen.getByTestId("solve-progress-panel")).toHaveAttribute("data-state", "refused");
    expect(screen.getByTestId("solve-progress-result-too-large")).toHaveTextContent(MESSAGE);
    expect(screen.queryByTestId("solve-progress-refusal")).toBeNull();
    expect(screen.queryByTestId("solve-progress-failure")).toBeNull();
  });
});

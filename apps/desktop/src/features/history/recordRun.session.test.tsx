/** W-12 S2 (DEL-14-01 Design `desktop-states-runs-comparison.md` §3): Record Run in the
 * actual workspace session, against a fake of the native store's commands. The solve is
 * the native job transport replay of committed producer bytes (unit transport replay;
 * NOT a native UI qualification witness). The store's own checks (the binding, the
 * recording time, the revision hash, immutability, the one transaction) are the Rust
 * tests in `analysis_runs_tests.rs` and `native_persistence_tests.rs`. C1 slice 5: the native
 * result is saved and recorded from its job handle; the main case runs on the actual commands'
 * replies (`native_persistence_replies.json`), request for request, and the others through the
 * unit fake (`nativePersistenceFake.ts`). All data are invented. */
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { useState } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { createNativePersistenceFake, isNativePersistenceCommand } from "../../test/nativePersistenceFake";
import { nativePersistenceReplay } from "../../test/nativePersistenceReplay";
import { NATIVE_RECORD_RUN_COMMAND, NATIVE_SAVE_PROJECT_COMMAND } from "../../services/nativePersistence";
import { isNativeResult } from "../../services/nativeResult";
import { getLocalStorageCapability, type StoredProjectEntry } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import * as previewService from "../../services/previewService";
import { computeModelHash } from "../../services/hashService";
import {
  recordableRun,
  RUN_RECORD_MANIFEST_UNAVAILABLE,
  RUN_RECORD_NO_RUN,
  type AnalysisRunSummary,
  type RunLabels
} from "../../services/analysisRunService";
import type { ModelStateSummary } from "../../services/modelStateService";
import { savedStatesSentence } from "../project-storage/storedProjectConflict";
import { ResultsPanel } from "../results/ResultsPanel";
import { useWorkspaceSession, type WorkspaceSession } from "../workspace/workspaceSession";
import type { LocalProjectEnvelope, PreviewModel } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

const LABELS: RunLabels = {
  run_name: "Run – rev A", tags: ["tie-in"], notes: ["Invented note."],
  assumptions: [{ assumption_id: "assumption:R-1", kind: "assumption", status: "unresolved", statement: "Vendor stiffness pending.", affected_refs: [] }]
};

type Store = {
  /** Stored project rows: id -> envelope hash. */
  rows: Map<string, string>;
  states: ModelStateSummary[];
  runs: AnalysisRunSummary[];
  records: Json[];
  /** `find_local_project`'s answer for an id not in `rows`. */
  foreign: StoredProjectEntry | null;
  /** When set, the next project save answers once this settles. */
  gate: Promise<void> | null;
  /** When set, the next native save reads the held result at once and answers once this settles. */
  lateReply: Promise<void> | null;
  savedEnvelopes: LocalProjectEnvelope[];
};

let sequence = 0;
/** The session over the replayed solve of the committed pressure-2 pair, and a fake store; with
 * `replay`, the native persistence commands get that store's actual replies, in order. */
async function session(options: { stored?: boolean; foreign?: StoredProjectEntry | null; replay?: string } = {}) {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-record-run:${++sequence}` });
  const store: Store = { rows: new Map(), states: [], runs: [], records: [], foreign: options.foreign ?? null, gate: null, lateReply: null, savedEnvelopes: [] };
  let rowSequence = 0;
  const saveProject = (request: Json): LocalProjectEnvelope => {
    const id = request.model.project.id as string;
    const base = `sha256:${id}#${++rowSequence}`;
    store.rows.set(id, base);
    const envelope = {
      summary: { project_id: id, project_name: request.model.project.name, message: "Saved current project model snapshot without external file copies." },
      model: request.model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: request.mechanics_result ?? null,
      analysis_run: request.analysis_run ?? null, model_hash: request.model_hash, project_envelope_hash: { value: base }, model_document_version: null
    } as unknown as LocalProjectEnvelope;
    store.savedEnvelopes.push(envelope);
    return envelope;
  };
  const recordRun = async (request: Json) => {
    store.records.push(request);
    if (request.expected_base !== store.rows.get(request.project_id)) throw `PROJECT-BASE-MOVED: project \`${request.project_id}\` changed after this session opened or saved it.`;
    const created = `2026-10-24T10:00:0${store.records.length}.000Z`;
    let state = store.states.find((candidate) => candidate.state_id === request.state_id);
    if (!state) {
      const hash = (await computeModelHash(JSON.parse(request.input_manifest_canonical).model_basis.model_payload))!.value;
      state = { ...request.new_state.labels, project_id: request.project_id, state_id: `state:${String(store.records.length).padStart(32, "0")}`,
        state_kind: "post_solve_snapshot", created_at: created, parent_state_id: request.new_state.parent_state_id, model_hash: hash,
        record_hash: "r", labels_updated_at: created, child_state_ids: [], recorded_run_count: 0, analysis_status: ["HUMAN_REVIEW_REQUIRED"] } as ModelStateSummary;
      store.states.push(state);
    }
    const run: AnalysisRunSummary = { ...request.run_labels, project_id: request.project_id, recording_id: `runrec:${String(store.records.length).padStart(32, "0")}`,
      state_id: state.state_id, producer_run_id: request.run_record.analysis_run.run_id, created_at: created, run_record_hash: "h", result_hash: "r",
      labels_updated_at: created, analysis_status: request.run_record.analysis_run.analysis_status };
    store.runs.push(run);
    return { run, state, state_outcome: request.state_id ? "existing" : "created", message: `Recorded run "${run.run_name}" (${run.recording_id}). Renaming, tagging or adding notes does not change the recorded run.` };
  };
  const refuseForeign = (request: Json) => {
    const id = request.model.project.id as string;
    if (store.foreign?.project_id === id && request.confirmed_revision !== store.foreign.revision && !store.rows.has(id)) {
      throw `PROJECT-ID-EXISTS: project \`${id}\` ("${store.foreign.project_name}", last saved at unix 2) is already stored; replacing it needs the user's confirmation.`;
    }
  };
  const fake = createNativePersistenceFake({
    heldBytes: transport.heldBytes,
    save: ({ request }) => { refuseForeign(request); return saveProject(request); },
    record: recordRun
  });
  const replay = options.replay ? nativePersistenceReplay(options.replay) : null;
  invokeMock.mockImplementation(async (command: string, args: Json, options?: unknown) => {
    switch (command) {
      case "get_local_storage_capability": return storage;
      case "load_design_knowledge": return knowledge;
      case "sync_native_shell_state": return null;
      case "check_model_library_refs": return [];
      case "load_preview_model": if (args === undefined) return structuredClone(model); break;
      case "find_local_project": {
        const id = args.projectId as string;
        if (store.rows.has(id)) return { project_id: id, project_name: "Stored", storage_mode: "local_sqlite", created_at_unix: 1, updated_at_unix: 2, revision: `rev:${store.rows.get(id)}`, saved_state_count: store.states.length, recorded_run_count: store.runs.length };
        return store.foreign && store.foreign.project_id === id ? store.foreign : null;
      }
      case "save_local_project": {
        const request = args.request as Json;
        if (store.gate) { const gate = store.gate; store.gate = null; await gate; }
        refuseForeign(request);
        return saveProject(request);
      }
      case NATIVE_SAVE_PROJECT_COMMAND:
      case NATIVE_RECORD_RUN_COMMAND: {
        if (command === NATIVE_SAVE_PROJECT_COMMAND && store.gate) { const gate = store.gate; store.gate = null; await gate; }
        if (command === NATIVE_SAVE_PROJECT_COMMAND && store.lateReply) {
          // The native command took its share of the held result at once; its reply is late.
          const gate = store.lateReply; store.lateReply = null;
          const reply = fake.invoke(command, args);
          await gate;
          return reply;
        }
        if (!replay) return fake.invoke(command, args);
        // The actual command's reply; the fake store follows it so the session's lists do too.
        const reply = replay.invoke(command, args) as Json;
        if (command === NATIVE_SAVE_PROJECT_COMMAND) store.rows.set(reply.summary.project_id, reply.project_envelope_hash.value);
        else {
          if (!store.states.some((state) => state.state_id === reply.state.state_id)) store.states.push(reply.state);
          store.runs.push(reply.run);
        }
        return reply;
      }
      case "open_local_project": return structuredClone(store.savedEnvelopes.at(-1)!);
      case "list_model_states": return store.states.filter((state) => state.project_id === args.projectId);
      case "list_analysis_runs": return store.runs.filter((run) => run.project_id === args.projectId);
      case "record_analysis_run": return recordRun(args.request as Json);
    }
    return transport.invoke(command, args, options);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  let current!: WorkspaceSession;
  function Harness() {
    current = useWorkspaceSession();
    const [selected, setSelected] = useState<string | null>(null);
    return <ResultsPanel result={current.results.result} knowledge={null} analysisRun={current.results.analysisRun} selectedResultId={selected} onSelectResult={setSelected} />;
  }
  render(<Harness />);
  await waitFor(() => expect(current.model.model).not.toBeNull());
  await waitFor(() => expect(current.model.modelHash).not.toBeNull());
  if (options.stored) await act(async () => { await current.project.handleSaveProject(); });
  return { state: () => current, store, model: model as PreviewModel, fake, replay, transport };
}

async function solve(state: () => WorkspaceSession) {
  await act(async () => { await state().results.handleRun(); });
  await waitFor(() => expect(state().results.solveJob.state).toBe("completed"));
  expect(state().results.analysisRun).not.toBeNull();
  expect(state().results.inputManifest).not.toBeNull();
}

const history = (state: () => WorkspaceSession) => state().project.history;
const calls = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command).map(([, args]) => args as Json);
/** Project saves of either route (a native result is saved from its job handle). */
const saves = () => invokeMock.mock.calls.filter(([name]) => name === "save_local_project" || name === NATIVE_SAVE_PROJECT_COMMAND);

describe("Record Run in the session (W-12 S2)", () => {
  it("names why there is nothing to record: no solve, or a run restored from the project row without its manifest", () => {
    expect(recordableRun({ result: null, analysisRun: null, inputManifest: null, restored: false }).reason).toBe(RUN_RECORD_NO_RUN);
    expect(recordableRun({ result: null, analysisRun: null, inputManifest: null, restored: true }).reason).toBe(RUN_RECORD_MANIFEST_UNAVAILABLE);
    expect(RUN_RECORD_MANIFEST_UNAVAILABLE).toMatch(/^RUN-RECORD-MANIFEST-UNAVAILABLE: re-solve to record this run/);
  });

  it("is unavailable for a run restored from the project row, until a re-solve", async () => {
    const { state, store } = await session({ stored: true });
    expect(history(state).recordRunUnavailableReason).toBe(RUN_RECORD_NO_RUN);
    await solve(state);
    await act(async () => { await state().project.handleSaveProject(); });
    await act(async () => { await state().project.handleOpenProject(state().model.model!.project.id); });
    await waitFor(() => expect(state().results.historicalRun?.designation).toBe("historical_saved_run"));
    expect(state().results.inputManifest).toBeNull();
    expect(history(state).recordRunUnavailableReason).toBe(RUN_RECORD_MANIFEST_UNAVAILABLE);
    let recorded = true;
    await act(async () => { recorded = await history(state).recordRun({ labels: LABELS, snapshotName: "After solve" }); });
    expect(recorded).toBe(false);
    expect(history(state).historyMessage).toBe(`Not recorded: ${RUN_RECORD_MANIFEST_UNAVAILABLE}`);
    expect(store.records).toHaveLength(0);
    await solve(state);
    expect(history(state).recordRunUnavailableReason).toBeNull();
  });

  it("saves the local project first, writes a named post-solve snapshot with the run, and leaves the current-run slot unchanged", async () => {
    // The actual native commands' replies, request for request: the session sends the job handle,
    // the previewed record's identity and small inputs, never a result, record or full copy.
    const { state, store, model, replay } = await session({ replay: "store" });
    await solve(state);
    const slot = { result: state().results.result, analysisRun: state().results.analysisRun, inputManifest: state().results.inputManifest };
    expect(history(state).recordRunState).toBeNull();
    invokeMock.mockClear();
    const typescriptRecords = vi.spyOn(previewService, "buildAnalysisRunPreview");
    let recorded = false;
    await act(async () => { recorded = await history(state).recordRun({ labels: LABELS, snapshotName: "After solve" }); });
    expect(recorded, history(state).historyMessage ?? undefined).toBe(true);
    const order = invokeMock.mock.calls.map(([name]) => name).filter((name) => isNativePersistenceCommand(name) || name === "save_local_project" || name === "record_analysis_run");
    expect(order).toEqual([NATIVE_SAVE_PROJECT_COMMAND, NATIVE_RECORD_RUN_COMMAND]);
    expect(replay!.remaining()).toBe(1);
    const [saveRequest] = calls(NATIVE_SAVE_PROJECT_COMMAND), [request] = calls(NATIVE_RECORD_RUN_COMMAND);
    expect(saveRequest.model.project.id).toBe(model.project.id);
    expect(request.request.expected_base).toBe(store.rows.get(model.project.id));
    expect(request.request.state_id).toBeNull();
    expect(request.request.new_state).toEqual({ labels: { state_name: "After solve", tags: [], notes: [], assumptions: [] }, parent_state_id: null });
    expect(request.request.run_labels).toEqual(LABELS);
    expect(request.expectedRecord.record_sha256).toBe(slot.analysisRun!.analysis_run.hashes.find((hash) => hash.payload_scope === "analysis_run_record")!.value);
    expect(request.inputManifest.canonical_bytes).toBe(slot.inputManifest!.canonical_bytes);
    for (const sent of [saveRequest, request]) {
      for (const absent of ["mechanicsResult", "analysisRun", "projectEnvelopeHash", "run_record", "result_json"]) expect(Object.hasOwn(sent, absent) || Object.hasOwn(sent.request ?? {}, absent), absent).toBe(false);
    }
    // No full copy, no TypeScript record, no submitted command.
    expect(invokeMock.mock.calls.some(([name]) => name === "read_preview_mechanics_job_rows_all" || name === "save_local_project" || name === "record_analysis_run")).toBe(false);
    expect(typescriptRecords).not.toHaveBeenCalled();
    // The slot, and with it the project envelope, are untouched by the recording.
    expect(state().results.result).toBe(slot.result);
    expect(state().results.analysisRun).toBe(slot.analysisRun);
    expect(state().results.inputManifest).toBe(slot.inputManifest);
    expect(saves()).toHaveLength(1);
    expect(history(state).historyOperation).toBe("record_run_with_snapshot");
    expect(history(state).historyMessage).toBe(replay!.scenarios[1].reply.value.message);
    await waitFor(() => expect(history(state).runs?.map((run) => run.run_name)).toEqual(["Run – rev A"]));
    // The next recording of that solve is tied to the listed snapshot by its model hash.
    await waitFor(() => expect(history(state).recordRunState?.state_id).toBe(store.states[0].state_id));
    await act(async () => { await history(state).recordRun({ labels: { ...LABELS, run_name: "Again" }, snapshotName: null }); });
    expect(history(state).historyOperation).toBe("record_run");
    expect(replay!.remaining()).toBe(0);
    const again = calls(NATIVE_RECORD_RUN_COMMAND)[1];
    expect(again.request.state_id).toBe(store.states[0].state_id);
    expect(again.request.new_state).toBeNull();
    expect(saves()).toHaveLength(1);
    // The replace prompt counts both kinds of history.
    expect(savedStatesSentence({ project_id: model.project.id, project_name: "p", storage_mode: "local_sqlite", created_at_unix: 1, updated_at_unix: 2, revision: "r", saved_state_count: 1, recorded_run_count: 2 }))
      .toBe(` Replacing it keeps the 1 saved model state and 2 recorded runs attached to project id ${model.project.id}.`);
  });

  it("asks for a post-solve snapshot name when no saved state holds the solved model", async () => {
    const { state, store } = await session({ stored: true });
    await solve(state);
    let recorded = true;
    await act(async () => { recorded = await history(state).recordRun({ labels: LABELS, snapshotName: "  " }); });
    expect(recorded).toBe(false);
    expect(history(state).historyMessage).toBe("Not recorded: no saved state holds the solved model; name a post-solve snapshot to record the run against.");
    await act(async () => { recorded = await history(state).recordRun({ labels: { ...LABELS, run_name: " " }, snapshotName: "After solve" }); });
    expect(history(state).historyMessage).toBe("Not recorded: a recorded run needs a name.");
    expect(store.records).toHaveLength(0);
  });

  it("refuses by name while another project operation owns the session", async () => {
    const { state, store } = await session({ stored: true });
    await solve(state);
    let release!: () => void;
    store.gate = new Promise<void>((resolve) => { release = resolve; });
    let saving!: Promise<void>;
    act(() => { saving = state().project.handleSaveProject(); });
    await waitFor(() => expect(saves()).toHaveLength(2));
    let recorded = true;
    await act(async () => { recorded = await history(state).recordRun({ labels: LABELS, snapshotName: "After solve" }); });
    expect(recorded).toBe(false);
    expect(history(state).historyMessage).toBe("Not recorded: another project operation or prompt is in progress; nothing was replaced.");
    expect(history(state).historyOperation).toBe("record_run_busy");
    expect(store.records).toHaveLength(0);
    release();
    await act(async () => { await saving; });
  });

  it("follows the project save that landed: after Save as a new project the solve is of the old project, so nothing is recorded", async () => {
    const { model } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
    const foreign: StoredProjectEntry = { project_id: model.project.id, project_name: "Another stored project", storage_mode: "local_sqlite", created_at_unix: 1, updated_at_unix: 2, revision: "rev:foreign" };
    const { state, store } = await session({ foreign });
    await solve(state);
    let recording!: Promise<boolean>;
    act(() => { recording = history(state).recordRun({ labels: LABELS, snapshotName: "After solve" }); });
    await waitFor(() => expect(state().project.replaceProjectPrompt).not.toBeNull());
    act(() => state().project.replaceProjectPrompt!.choose("save-as-new"));
    let recorded = true;
    await act(async () => { recorded = await recording; });
    expect(recorded).toBe(false);
    const savedId = [...store.rows.keys()][0];
    expect(savedId).toMatch(new RegExp(`^${model.project.id}:copy-`));
    expect(history(state).historyOperation).toBe("record_run_saved_as_new");
    expect(history(state).historyMessage).toBe(`Not recorded: the local project was saved as a new project (${savedId}), but the run is a solve of project ${model.project.id}. Solve the new project to record a run of it.`);
    expect(store.records).toHaveLength(0);
  });

  it("keeps the user's result page across Record Run and a re-solve: S2 gives runs no new identity the results view keys on", async () => {
    const { state } = await session({ stored: true });
    await solve(state);
    fireEvent.click(await screen.findByTestId("next-result-page"));
    await waitFor(() => expect(screen.getByTestId("result-page-summary")).toHaveTextContent(/page 2 of/));
    const runId = state().results.result!.run_id;
    await act(async () => { await history(state).recordRun({ labels: LABELS, snapshotName: "After solve" }); });
    expect(history(state).historyOperation).toBe("record_run_with_snapshot");
    expect(screen.getByTestId("result-page-summary")).toHaveTextContent(/page 2 of/);
    // A re-solve delivers a new result object with the producer's same run id.
    const before = state().results.result;
    await solve(state);
    // Identity only: a deep comparison would read the native handles' guarded members.
    expect(state().results.result !== before).toBe(true);
    expect(state().results.result!.run_id).toBe(runId);
    await waitFor(() => expect(screen.getByTestId("result-page-summary")).toHaveTextContent(/page 2 of/));
  });
});

describe("Save Project and Record Run of a native result (C1 slice 5)", () => {
  it("saves the model without a result that does not fit the store, keeps the solve, and refuses Record Run above the limit by name (the actual commands' replies)", async () => {
    const { state, replay } = await session({ replay: "small" });
    await solve(state);
    const slot = { result: state().results.result, analysisRun: state().results.analysisRun };
    invokeMock.mockClear();
    await act(async () => { await state().project.handleSaveProject(); });
    expect(state().project.projectOperation).toBe("save");
    const message = state().project.projectMessage!;
    expect(message).toContain("PROJECT-SAVE-RESULT-TOO-LARGE");
    expect(message).toContain("the result was not saved");
    expect(message).toBe(replay!.scenarios[0].reply.value.summary.message);
    // The in-memory native solve is kept: the result was not saved, not dropped.
    expect(state().results.result).toBe(slot.result);
    expect(state().results.analysisRun).toBe(slot.analysisRun);
    expect(isNativeResult(state().results.result)).toBe(true);
    expect(state().project.projectEnvelopeHash?.value).toBe(replay!.scenarios[0].reply.value.project_envelope_hash.value);
    let recorded = true;
    await act(async () => { recorded = await history(state).recordRun({ labels: LABELS, snapshotName: "After solve" }); });
    expect(recorded).toBe(false);
    expect(history(state).historyMessage).toBe(`Not recorded: ${replay!.scenarios[1].reply.error}`);
    expect(history(state).historyMessage).toMatch(/^Not recorded: RUN-RECORD-RESULT-TOO-LARGE \(result_bytes=\d+, record_bytes=\d+, row_bytes=\d+, limit_bytes=\d+\)/);
    expect(replay!.remaining()).toBe(0);
    expect(invokeMock.mock.calls.some(([name]) => name === "read_preview_mechanics_job_rows_all" || name === "save_local_project" || name === "record_analysis_run")).toBe(false);
  });

  it("a save whose reply lands after a re-solve records the run it was sent with and keeps the new solve", async () => {
    const { state, store } = await session();
    await solve(state);
    const first = state().results.result!;
    let release!: () => void;
    store.lateReply = new Promise<void>((resolve) => { release = resolve; });
    let saving!: Promise<void>;
    act(() => { saving = state().project.handleSaveProject(); });
    await waitFor(() => expect(calls(NATIVE_SAVE_PROJECT_COMMAND)).toHaveLength(1));
    const sentJob = calls(NATIVE_SAVE_PROJECT_COMMAND)[0].jobId;
    await solve(state);
    const second = state().results.result!;
    expect(second !== first).toBe(true);
    release();
    await act(async () => { await saving; });
    // The store holds the run the save was sent with; the session keeps the new Current solve.
    expect(store.savedEnvelopes.at(-1)!.mechanics_result!.run_id).toBe(first.run_id);
    expect(state().results.result).toBe(second);
    expect(isNativeResult(state().results.result)).toBe(true);
    expect(calls(NATIVE_SAVE_PROJECT_COMMAND).map((args) => args.jobId)).toEqual([sentJob]);
    expect(store.savedEnvelopes).toHaveLength(1);
  });

  it("a save for a job no longer held is refused by name and writes nothing", async () => {
    const { state, store, transport } = await session();
    await solve(state);
    const [jobId] = transport.nativeHeld();
    // The native host releases the held result (as a reset or disposal elsewhere would).
    await transport.invoke("dispose_preview_mechanics_result", { jobId });
    await act(async () => { await state().project.handleSaveProject(); });
    expect(state().project.projectOperation).toBe("save_failed");
    expect(state().project.projectMessage).toMatch(/^Save failed: NATIVE-PERSISTENCE-CUSTODY: /);
    expect(store.savedEnvelopes).toHaveLength(0);
  });
});

/** W-12 S2 (DEL-14-01 Design `desktop-states-runs-comparison.md` §3): Record Run in the
 * actual workspace session, against a fake of the native store's commands. The solve is
 * the native job transport replay of committed producer bytes (unit transport replay;
 * NOT a native UI qualification witness). The store's own checks (the binding, the
 * recording time, the revision hash, immutability, the one transaction) are the Rust
 * tests in `analysis_runs_tests.rs`. All data are invented. */
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { useState } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability, type StoredProjectEntry } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
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
import type { C2FullResult } from "../results/C2ResultGate";
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
  savedEnvelopes: LocalProjectEnvelope[];
};

let sequence = 0;
/** The session over the replayed solve of the committed pressure-2 pair, and a fake store. */
async function session(options: { stored?: boolean; foreign?: StoredProjectEntry | null } = {}) {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-record-run:${++sequence}` });
  const store: Store = { rows: new Map(), states: [], runs: [], records: [], foreign: options.foreign ?? null, gate: null, savedEnvelopes: [] };
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
        const id = request.model.project.id as string;
        if (store.foreign?.project_id === id && request.confirmed_revision !== store.foreign.revision && !store.rows.has(id)) {
          throw `PROJECT-ID-EXISTS: project \`${id}\` ("${store.foreign.project_name}", last saved at unix 2) is already stored; replacing it needs the user's confirmation.`;
        }
        return saveProject(request);
      }
      case "open_local_project": return structuredClone(store.savedEnvelopes.at(-1)!);
      case "list_model_states": return store.states.filter((state) => state.project_id === args.projectId);
      case "list_analysis_runs": return store.runs.filter((run) => run.project_id === args.projectId);
      case "record_analysis_run": {
        const request = args.request as Json;
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
      }
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
  return { state: () => current, store, model: model as PreviewModel };
}

async function solve(state: () => WorkspaceSession) {
  await act(async () => { await state().results.handleRun(); });
  await waitFor(() => expect(state().results.solveJob.state).toBe("completed"));
  expect(state().results.analysisRun).not.toBeNull();
  expect(state().results.inputManifest).not.toBeNull();
}

const history = (state: () => WorkspaceSession) => state().project.history;
const calls = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command).map(([, args]) => args as Json);

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
    const { state, store, model } = await session();
    await solve(state);
    const slot = { result: state().results.result, analysisRun: state().results.analysisRun, inputManifest: state().results.inputManifest };
    expect(history(state).recordRunState).toBeNull();
    let recorded = false;
    await act(async () => { recorded = await history(state).recordRun({ labels: LABELS, snapshotName: "After solve" }); });
    expect(recorded).toBe(true);
    const order = invokeMock.mock.calls.map(([name]) => name).filter((name) => name === "save_local_project" || name === "record_analysis_run");
    expect(order).toEqual(["save_local_project", "record_analysis_run"]);
    const request = store.records[0];
    expect(request.project_id).toBe(model.project.id);
    expect(request.expected_base).toBe(store.rows.get(model.project.id));
    expect(request.state_id).toBeNull();
    expect(request.new_state).toEqual({ labels: { state_name: "After solve", tags: [], notes: [], assumptions: [] }, parent_state_id: null });
    expect(request.run_labels).toEqual(LABELS);
    // C1 interim: Record Run writes a native result through its C2 full copy and full record (the
    // bytes today's store path writes); the record is the session's (its checksums are the header's).
    let full: C2FullResult | null = null;
    await act(async () => { full = await state().results.loadC2Full(slot.result!); });
    expect(request.run_record).toEqual(full!.analysisRun);
    expect(request.run_record.analysis_run.hashes).toStrictEqual(slot.analysisRun!.analysis_run.hashes);
    expect(request.input_manifest_canonical).toBe(slot.inputManifest!.canonical_bytes);
    expect(JSON.parse(request.result_json)).toEqual(full!.result);
    // The slot, and with it the project envelope, are untouched by the recording.
    expect(state().results.result).toBe(slot.result);
    expect(state().results.analysisRun).toBe(slot.analysisRun);
    expect(state().results.inputManifest).toBe(slot.inputManifest);
    expect(calls("save_local_project")).toHaveLength(1);
    expect(history(state).historyOperation).toBe("record_run_with_snapshot");
    await waitFor(() => expect(history(state).runs?.map((run) => run.run_name)).toEqual(["Run – rev A"]));
    // The next recording of that solve is tied to the listed snapshot by its model hash.
    await waitFor(() => expect(history(state).recordRunState?.state_id).toBe(store.states[0].state_id));
    await act(async () => { await history(state).recordRun({ labels: { ...LABELS, run_name: "Again" }, snapshotName: null }); });
    expect(store.records[1].state_id).toBe(store.states[0].state_id);
    expect(store.records[1].new_state).toBeNull();
    expect(calls("save_local_project")).toHaveLength(1);
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
    await waitFor(() => expect(calls("save_local_project")).toHaveLength(2));
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

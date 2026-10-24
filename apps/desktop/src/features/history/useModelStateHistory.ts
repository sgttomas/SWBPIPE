// The session's model-state history (W-12 S1 and S2; DEL-14-01 Design
// `desktop-states-runs-comparison.md` §2, §3): Save Model State…, Open State,
// Record Run, the History panel's lists of states and recorded runs, label
// edits, delete and a recorded run's check. Called only by the session
// (`useWorkspaceSession`), which hands it the cells and handlers it needs; the
// History panel reads the result through `useSessionHistory()`.
import { useEffect, useRef, useState } from "react";
import type { MutableRefObject } from "react";
import {
  deleteModelState,
  editModelStateLabels,
  listModelStates,
  MODEL_STATE_STORE_UNAVAILABLE,
  modelStateDescription,
  readModelState,
  saveModelState,
  stateIsOpenable,
  type ModelStateKind,
  type ModelStateRead,
  type ModelStateSummary,
  type StateLabels
} from "../../services/modelStateService";
import {
  deleteAnalysisRun,
  editAnalysisRunLabels,
  LABELS_DO_NOT_CHANGE_RUN,
  listAnalysisRuns,
  readAnalysisRun,
  recordableRun,
  recordAnalysisRun,
  runDescription,
  type AnalysisRunSummary,
  type CurrentRunSlot,
  type RunLabels
} from "../../services/analysisRunService";
import { buildRecordedRunContext, type HistoricalRunContext } from "../results/HistoricalRunContext";
import { computeModelHash } from "../../services/hashService";
import type { CurrentSessionInputManifest } from "../../services/inputManifestService";
import { nativeFailureText } from "../../services/nativeIngress";
import { evaluateModelDocumentLocal } from "../../services/projectService";
import type { LocalProjectSummary, ModelHashEvidence, PreviewModel, ProjectEnvelopeHashEvidence } from "../../types";
import { buildModelIndex } from "../workspace/modelIndex";
import { unsavedChangesRefusal, type UnsavedChangesOutcome, type UnsavedChangesRequest } from "../project-storage/unsavedChangesGuard";

export type ModelStateHistoryDeps = {
  /** The session's model as rendered, and the one it holds now. */
  model: PreviewModel | null;
  currentModel: MutableRefObject<PreviewModel | null>;
  projectSessionGeneration: number;
  projectSessionGenerationRef: MutableRefObject<number>;
  /** The stored project this session is, at the envelope hash it last opened or wrote. */
  storedBaseRef: MutableRefObject<{ projectId: string; value: string | null } | null>;
  /** The session's unsaved-changes guard (`unsavedChangesGuard.ts`). */
  confirmReplacingEdits: (request: UnsavedChangesRequest) => Promise<UnsavedChangesOutcome>;
  acquireProjectOperation: () => symbol | null;
  releaseProjectOperation: (owner: symbol) => void;
  /** Save Local Project's flow, with its replace prompt: the save that landed,
   * or null. "Save as a new project" lands under a new project id. */
  saveLocalProject: () => Promise<LandedProjectSave | null>;
  /** Makes an opened state's snapshot the working model (a new project session). */
  /** Another project operation or prompt owns the session: Open State and Save
   * Model State start no prompt and no operation then. */
  projectSessionBusy: () => boolean;
  /** The session's stored-project binding now (base and persistence metadata). */
  captureProjectBinding: () => ProjectBinding;
  /** Makes an opened state's snapshot the working model, bound to the state's
   * project (`binding`: captured before the unsaved-changes guard ran). Returns
   * the library-reference warnings of the adopted model, for the message. */
  adoptStateSnapshot: (opened: ModelStateRead, snapshotHash: ModelHashEvidence, binding: ProjectBinding) => Promise<string>;
  /** The session's current-run slot as rendered (W-12 S2): Record Run records it and never changes it. */
  runSlot: CurrentRunSlot;
};

/** Why a state with blocking document findings is not opened. */
export function notOpenableReason(state: ModelStateSummary): string {
  return `the saved state ${modelStateDescription(state)} is MODEL_INCOMPLETE: its model has blocking document findings, so it is kept as a comparison basis but cannot be opened as the working model (as such a project cannot be opened).`;
}

/** The session's stored-project binding: the stored base it last saw for a
 * project, and the persistence metadata that goes with it. */
export type ProjectBinding = {
  base: { projectId: string; value: string | null } | null;
  summary: LocalProjectSummary | null;
  envelopeHash: ProjectEnvelopeHashEvidence | null;
};

/** A Save Local Project that landed: the stored project, its new base and the
 * document it stored (after "Save as a new project", the renamed working model). */
export type LandedProjectSave = { projectId: string; base: string | null; model: PreviewModel };

export type SaveModelStateForm = { labels: StateLabels; kind: ModelStateKind };

/** Record Run's form: the run's labels and, when no saved state holds the solved
 * model, the name of the post-solve snapshot written with it. */
export type RecordRunForm = { labels: RunLabels; snapshotName: string | null };

/** A recorded run read and checked against its stored state (`buildRecordedRunContext`). */
export type RecordedRunCheck = {
  recordingId: string; context: HistoricalRunContext | null; failure: string | null;
  /** The stored input manifest, for the rule-pack references it records (null when unreadable). */
  manifest?: CurrentSessionInputManifest | null;
};

function parsedManifest(text: string): CurrentSessionInputManifest | null {
  try {
    const value: unknown = JSON.parse(text);
    return value && typeof value === "object" && !Array.isArray(value) ? value as CurrentSessionInputManifest : null;
  } catch {
    return null;
  }
}

function storeAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export function useModelStateHistory(deps: ModelStateHistoryDeps) {
  const [states, setStates] = useState<ModelStateSummary[] | null>(null);
  const [runs, setRuns] = useState<AnalysisRunSummary[] | null>(null);
  const [runCheck, setRunCheck] = useState<RecordedRunCheck | null>(null);
  // The model hash of the slot's solved model (its input manifest's payload), for
  // naming the state Record Run ties the run to.
  const [solvedModelHash, setSolvedModelHash] = useState<{ manifest: unknown; hash: string | null } | null>(null);
  const [statesProjectId, setStatesProjectId] = useState<string | null>(null);
  const [historyMessage, setHistoryMessage] = useState("No model states listed yet.");
  const [historyOperation, setHistoryOperation] = useState("not_started");
  const [historyBusy, setHistoryBusy] = useState(false);
  // The saved state the working model was opened from or last saved as, for
  // this project session: the parent of the next state saved.
  const [basis, setBasis] = useState<{ generation: number; projectId: string; stateId: string } | null>(null);
  const listRequest = useRef(0);
  const depsRef = useRef(deps);
  depsRef.current = deps;

  const projectId = deps.model?.project.id ?? null;
  const basisStateId = basis && basis.generation === deps.projectSessionGeneration && basis.projectId === projectId ? basis.stateId : null;

  async function refreshStates(forProjectId: string | null = depsRef.current.currentModel.current?.project.id ?? null) {
    const request = ++listRequest.current;
    if (!forProjectId) return;
    if (!storeAvailable()) {
      setStates(null);
      setRuns(null);
      setStatesProjectId(forProjectId);
      setHistoryMessage(MODEL_STATE_STORE_UNAVAILABLE);
      setHistoryOperation("store_unavailable");
      return;
    }
    try {
      const [listed, listedRuns] = await Promise.all([listModelStates(forProjectId), listAnalysisRuns(forProjectId)]);
      if (request !== listRequest.current) return;
      setStates(listed);
      setRuns(listedRuns);
      setStatesProjectId(forProjectId);
    } catch (error) {
      if (request !== listRequest.current) return;
      setStates(null);
      setRuns(null);
      setStatesProjectId(forProjectId);
      setHistoryMessage(`List failed: ${nativeFailureText(error)}`);
      setHistoryOperation("list_failed");
    }
  }

  useEffect(() => {
    void refreshStates(projectId);
    setRunCheck(null);
  }, [projectId, deps.projectSessionGeneration]);

  const slotManifest = deps.runSlot.inputManifest;
  useEffect(() => {
    if (!slotManifest) { setSolvedModelHash(null); return; }
    let live = true;
    void computeModelHash(slotManifest.manifest.model_basis.model_payload)
      .then((hash) => { if (live) setSolvedModelHash({ manifest: slotManifest, hash: hash?.value ?? null }); })
      .catch(() => { if (live) setSolvedModelHash({ manifest: slotManifest, hash: null }); });
    return () => { live = false; };
  }, [slotManifest]);

  const recordable = recordableRun(deps.runSlot);
  const solvedHash = recordable.run && solvedModelHash?.manifest === recordable.run.inputManifest ? solvedModelHash.hash : null;
  /** The listed state of the slot's solved model, if any: the state Record Run ties the run to. */
  const recordRunState = solvedHash ? states?.find((state) => state.model_hash === solvedHash) ?? null : null;

  function begin(): symbol | null {
    const owner = depsRef.current.acquireProjectOperation();
    if (!owner) {
      setHistoryMessage("Another project operation is in progress; try again when it finishes.");
      setHistoryOperation("busy");
      return null;
    }
    setHistoryBusy(true);
    return owner;
  }

  function end(owner: symbol) {
    depsRef.current.releaseProjectOperation(owner);
    setHistoryBusy(false);
  }

  /** Save Model State…: the working model as a new immutable state of its
   * project. A state needs the project's stored row: when the session is not
   * that stored project, Save Local Project's flow runs first. */
  function refusedWhileBusy(refused: string, operation: string): boolean {
    if (!depsRef.current.projectSessionBusy()) return false;
    setHistoryMessage(unsavedChangesRefusal("busy", refused));
    setHistoryOperation(operation);
    return true;
  }

  async function saveState(form: SaveModelStateForm): Promise<boolean> {
    let requestModel = depsRef.current.currentModel.current;
    if (!requestModel) return false;
    if (refusedWhileBusy("Not saved", "save_state_busy")) return false;
    let requestProjectId = requestModel.project.id;
    let parentStateId = basisStateId;
    if (!form.labels.state_name.trim()) {
      setHistoryMessage("Not saved: a model state needs a name.");
      setHistoryOperation("save_state_refused");
      return false;
    }
    const isStoredProject = () => {
      const base = depsRef.current.storedBaseRef.current;
      return base?.projectId === requestProjectId && Boolean(base.value);
    };
    if (!isStoredProject()) {
      setHistoryMessage("A model state needs its project's stored copy: saving the local project first.");
      const landed = await depsRef.current.saveLocalProject();
      if (!landed) {
        setHistoryMessage("Not saved: the local project was not saved, so the model state was not saved.");
        setHistoryOperation("save_state_needs_project");
        return false;
      }
      if (depsRef.current.currentModel.current !== landed.model) {
        setHistoryMessage("Not saved: the working model changed while the local project was saved, so the model state was not saved.");
        setHistoryOperation("save_state_superseded");
        return false;
      }
      // The continuation follows the save that landed: after "Save as a new
      // project" that is the renamed working model under its new project id,
      // and a parent state of the old project no longer applies.
      if (landed.projectId !== requestProjectId) parentStateId = null;
      requestModel = landed.model;
      requestProjectId = landed.projectId;
      if (!isStoredProject()) {
        setHistoryMessage("Not saved: the local project was not saved, so the model state was not saved.");
        setHistoryOperation("save_state_needs_project");
        return false;
      }
    }
    const owner = begin();
    if (!owner) return false;
    const generation = depsRef.current.projectSessionGenerationRef.current;
    const sameSession = () => generation === depsRef.current.projectSessionGenerationRef.current;
    try {
      const outcome = await saveModelState({
        expectedBase: depsRef.current.storedBaseRef.current!.value!,
        model: requestModel,
        kind: form.kind,
        parentStateId,
        labels: form.labels
      });
      if (!sameSession()) return true;
      if (depsRef.current.currentModel.current === requestModel) {
        setBasis({ generation, projectId: requestProjectId, stateId: outcome.state.state_id });
      }
      setHistoryMessage(outcome.message);
      setHistoryOperation(outcome.outcome === "saved" ? "save_state" : "save_state_already_saved");
      await refreshStates(requestProjectId);
      return true;
    } catch (error) {
      if (!sameSession()) return false;
      setHistoryMessage(`Not saved: ${nativeFailureText(error)}`);
      setHistoryOperation("save_state_failed");
      return false;
    } finally {
      end(owner);
    }
  }

  /** Open State: the state's snapshot becomes the working model, behind the
   * unsaved-changes guard. Its version goes through the one-current-version
   * check and its hash is recomputed before it is adopted. */
  async function openState(stateId: string): Promise<boolean> {
    const state = states?.find((candidate) => candidate.state_id === stateId);
    const requestProjectId = depsRef.current.currentModel.current?.project.id;
    if (!state || !requestProjectId) return false;
    if (!stateIsOpenable(state)) {
      setHistoryMessage(`Not opened: ${notOpenableReason(state)}`);
      setHistoryOperation("open_state_refused");
      return false;
    }
    // Before the guard: no prompt may open while another operation owns the session.
    if (refusedWhileBusy("Not opened", "open_state_busy")) return false;
    // Captured before the guard, whose "Save first" may move the session to a
    // copy's project id ("Save as a new project").
    const binding = depsRef.current.captureProjectBinding();
    const guarded = await depsRef.current.confirmReplacingEdits({
      action: `Opening the saved state ${modelStateDescription(state)}`, verb: "open it"
    });
    if (guarded !== "proceed") {
      setHistoryMessage(unsavedChangesRefusal(guarded, "Not opened"));
      setHistoryOperation("open_state_cancelled");
      return false;
    }
    const owner = begin();
    if (!owner) return false;
    const generation = depsRef.current.projectSessionGenerationRef.current;
    const heldModel = depsRef.current.currentModel.current;
    const unchanged = () => generation === depsRef.current.projectSessionGenerationRef.current && depsRef.current.currentModel.current === heldModel;
    try {
      const opened = await readModelState(requestProjectId, stateId);
      const version = evaluateModelDocumentLocal(opened.model);
      if (version.status !== "current") {
        throw new Error(`Model document refused for editing: status=${version.status}; ${version.detail}`);
      }
      const hash = await computeModelHash(opened.model);
      if (!hash || hash.value !== opened.state.model_hash || opened.model.project.id !== requestProjectId) {
        throw new Error("MODEL-STATE-INTEGRITY: the opened snapshot does not match the saved state's model hash or project.");
      }
      buildModelIndex(opened.model, 0, 0);
      if (!unchanged()) {
        setHistoryMessage("Not opened: the session changed while the state was read; the working model was kept.");
        setHistoryOperation("open_state_superseded");
        return false;
      }
      const libraryWarnings = depsRef.current.adoptStateSnapshot(opened, hash, binding);
      const adoptedGeneration = depsRef.current.projectSessionGenerationRef.current;
      const adoptedModel = depsRef.current.currentModel.current;
      setBasis({ generation: adoptedGeneration, projectId: requestProjectId, stateId });
      setHistoryMessage(`Opened model state ${modelStateDescription(opened.state)} as the working model. Edits make a new working model; the saved state does not change.`);
      setHistoryOperation("open_state");
      // As a project open does: the snapshot's library references are checked
      // against the store, and the answer lands only on the session it was asked for.
      const warnings = await libraryWarnings;
      if (warnings && adoptedGeneration === depsRef.current.projectSessionGenerationRef.current && depsRef.current.currentModel.current === adoptedModel) {
        setHistoryMessage((message) => `${message}${warnings}`);
      }
      return true;
    } catch (error) {
      setHistoryMessage(`Not opened: ${nativeFailureText(error)}`);
      setHistoryOperation("open_state_failed");
      return false;
    } finally {
      end(owner);
    }
  }

  async function editLabels(stateId: string, labels: StateLabels): Promise<boolean> {
    const state = states?.find((candidate) => candidate.state_id === stateId);
    if (!state) return false;
    const owner = begin();
    if (!owner) return false;
    try {
      const edited = await editModelStateLabels(state.project_id, stateId, labels, state.labels_updated_at);
      setHistoryMessage(`Labels of model state ${modelStateDescription(edited)} saved. Renaming, tagging or adding notes does not change the saved state.`);
      setHistoryOperation("edit_state_labels");
      await refreshStates(state.project_id);
      return true;
    } catch (error) {
      setHistoryMessage(`Labels not saved: ${nativeFailureText(error)}`);
      setHistoryOperation("edit_state_labels_failed");
      return false;
    } finally {
      end(owner);
    }
  }

  async function deleteState(stateId: string): Promise<boolean> {
    const state = states?.find((candidate) => candidate.state_id === stateId);
    if (!state) return false;
    const owner = begin();
    if (!owner) return false;
    try {
      const receipt = await deleteModelState(state.project_id, stateId);
      if (basis?.stateId === stateId) setBasis(null);
      setHistoryMessage(receipt.message);
      setHistoryOperation("delete_state");
      await refreshStates(state.project_id);
      return true;
    } catch (error) {
      setHistoryMessage(`Not deleted: ${nativeFailureText(error)}`);
      setHistoryOperation("delete_state_failed");
      return false;
    } finally {
      end(owner);
    }
  }

  /** Record Run (W-12 S2): the slot's solve as a recorded revision tied to the
   * project's state of the solved model, or to a new post-solve snapshot the user
   * names. The run needs the project's stored row: when the session is not that
   * stored project, Save Local Project's flow runs first, and the recording follows
   * the save that landed. The current-run slot is never changed. */
  async function recordRun(form: RecordRunForm): Promise<boolean> {
    const slot = recordableRun(depsRef.current.runSlot);
    if (!slot.run) {
      setHistoryMessage(`Not recorded: ${slot.reason}`);
      setHistoryOperation("record_run_unavailable");
      return false;
    }
    const run = slot.run;
    if (refusedWhileBusy("Not recorded", "record_run_busy")) return false;
    if (!form.labels.run_name.trim()) {
      setHistoryMessage("Not recorded: a recorded run needs a name.");
      setHistoryOperation("record_run_refused");
      return false;
    }
    const runProjectId = run.result.model_ref;
    const isStoredProject = () => {
      const base = depsRef.current.storedBaseRef.current;
      return base?.projectId === runProjectId && Boolean(base.value);
    };
    if (!isStoredProject()) {
      setHistoryMessage("A recorded run needs its project's stored copy: saving the local project first.");
      const landed = await depsRef.current.saveLocalProject();
      if (!landed) {
        setHistoryMessage("Not recorded: the local project was not saved, so the run was not recorded.");
        setHistoryOperation("record_run_needs_project");
        return false;
      }
      // The recording follows the save that landed. After "Save as a new project"
      // the stored project is the renamed model's, not the solved model's.
      if (landed.projectId !== runProjectId) {
        setHistoryMessage(`Not recorded: the local project was saved as a new project (${landed.projectId}), but the run is a solve of project ${runProjectId}. Solve the new project to record a run of it.`);
        setHistoryOperation("record_run_saved_as_new");
        return false;
      }
      if (!isStoredProject()) {
        setHistoryMessage("Not recorded: the local project was not saved, so the run was not recorded.");
        setHistoryOperation("record_run_needs_project");
        return false;
      }
    }
    const owner = begin();
    if (!owner) return false;
    const generation = depsRef.current.projectSessionGenerationRef.current;
    const sameSession = () => generation === depsRef.current.projectSessionGenerationRef.current;
    try {
      const solved = await computeModelHash(run.inputManifest.manifest.model_basis.model_payload);
      const listed = statesProjectId === runProjectId ? states : null;
      const state = solved ? listed?.find((candidate) => candidate.model_hash === solved.value) ?? null : null;
      const snapshotName = form.snapshotName?.trim() ?? "";
      if (!state && !snapshotName) {
        setHistoryMessage("Not recorded: no saved state holds the solved model; name a post-solve snapshot to record the run against.");
        setHistoryOperation("record_run_needs_snapshot");
        return false;
      }
      const outcome = await recordAnalysisRun({
        expectedBase: depsRef.current.storedBaseRef.current!.value!,
        run,
        stateId: state?.state_id ?? null,
        newState: state ? null : {
          labels: { state_name: snapshotName, tags: [], notes: [], assumptions: [] },
          parentStateId: basisStateId
        },
        labels: form.labels
      });
      if (!sameSession()) return true;
      setHistoryMessage(outcome.message);
      setHistoryOperation(outcome.state_outcome === "created" ? "record_run_with_snapshot" : "record_run");
      await refreshStates(runProjectId);
      return true;
    } catch (error) {
      if (!sameSession()) return false;
      setHistoryMessage(`Not recorded: ${nativeFailureText(error)}`);
      setHistoryOperation("record_run_failed");
      return false;
    } finally {
      end(owner);
    }
  }

  async function editRunLabels(recordingId: string, labels: RunLabels): Promise<boolean> {
    const run = runs?.find((candidate) => candidate.recording_id === recordingId);
    if (!run) return false;
    const owner = begin();
    if (!owner) return false;
    try {
      const edited = await editAnalysisRunLabels(run.project_id, recordingId, labels, run.labels_updated_at);
      setHistoryMessage(`Labels of recorded run ${runDescription(edited)} saved. ${LABELS_DO_NOT_CHANGE_RUN}`);
      setHistoryOperation("edit_run_labels");
      await refreshStates(run.project_id);
      return true;
    } catch (error) {
      setHistoryMessage(`Labels not saved: ${nativeFailureText(error)}`);
      setHistoryOperation("edit_run_labels_failed");
      return false;
    } finally {
      end(owner);
    }
  }

  async function deleteRun(recordingId: string): Promise<boolean> {
    const run = runs?.find((candidate) => candidate.recording_id === recordingId);
    if (!run) return false;
    const owner = begin();
    if (!owner) return false;
    try {
      const receipt = await deleteAnalysisRun(run.project_id, recordingId);
      if (runCheck?.recordingId === recordingId) setRunCheck(null);
      setHistoryMessage(receipt.message);
      setHistoryOperation("delete_run");
      await refreshStates(run.project_id);
      return true;
    } catch (error) {
      setHistoryMessage(`Not deleted: ${nativeFailureText(error)}`);
      setHistoryOperation("delete_run_failed");
      return false;
    } finally {
      end(owner);
    }
  }

  /** Reads a recorded run (the store verifies its bytes) and checks it against its
   * stored state in this session. Nothing about the session changes. */
  async function checkRun(recordingId: string): Promise<boolean> {
    const run = runs?.find((candidate) => candidate.recording_id === recordingId);
    if (!run) return false;
    const owner = begin();
    if (!owner) return false;
    const generation = depsRef.current.projectSessionGenerationRef.current;
    try {
      const read = await readAnalysisRun(run.project_id, recordingId);
      const context = await buildRecordedRunContext(read);
      if (generation !== depsRef.current.projectSessionGenerationRef.current) return false;
      setRunCheck({ recordingId, context, failure: null, manifest: parsedManifest(read.input_manifest_canonical) });
      setHistoryOperation("check_run");
      return true;
    } catch (error) {
      if (generation !== depsRef.current.projectSessionGenerationRef.current) return false;
      setRunCheck({ recordingId, context: null, failure: nativeFailureText(error) });
      setHistoryOperation("check_run_failed");
      return false;
    } finally {
      end(owner);
    }
  }

  return {
    runs: statesProjectId === projectId ? runs : null,
    runCheck,
    recordRunUnavailableReason: recordable.reason,
    /** The recordable run's input manifest (its rule-pack references), shown in the Record Run form. */
    recordRunManifest: recordable.run?.inputManifest.manifest ?? null,
    recordRunState,
    recordRun,
    editRunLabels,
    deleteRun,
    checkRun,
    states: statesProjectId === projectId ? states : null,
    basisStateId,
    historyMessage,
    historyOperation,
    historyBusy,
    refreshStates,
    saveState,
    openState,
    editLabels,
    deleteState
  };
}

export type ModelStateHistory = ReturnType<typeof useModelStateHistory>;

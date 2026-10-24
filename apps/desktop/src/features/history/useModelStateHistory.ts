// The session's model-state history (W-12 S1; DEL-14-01 Design
// `desktop-states-runs-comparison.md` §2): Save Model State…, Open State, the
// History panel's list, label edits and delete. Called only by the session
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
import { computeModelHash } from "../../services/hashService";
import { nativeFailureText } from "../../services/nativeIngress";
import { evaluateModelDocumentLocal } from "../../services/projectService";
import type { ModelHashEvidence, PreviewModel } from "../../types";
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
  adoptStateSnapshot: (opened: ModelStateRead, snapshotHash: ModelHashEvidence) => void;
};

/** Why a state with blocking document findings is not opened. */
export function notOpenableReason(state: ModelStateSummary): string {
  return `the saved state ${modelStateDescription(state)} is MODEL_INCOMPLETE: its model has blocking document findings, so it is kept as a comparison basis but cannot be opened as the working model (as such a project cannot be opened).`;
}

/** A Save Local Project that landed: the stored project, its new base and the
 * document it stored (after "Save as a new project", the renamed working model). */
export type LandedProjectSave = { projectId: string; base: string | null; model: PreviewModel };

export type SaveModelStateForm = { labels: StateLabels; kind: ModelStateKind };

function storeAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export function useModelStateHistory(deps: ModelStateHistoryDeps) {
  const [states, setStates] = useState<ModelStateSummary[] | null>(null);
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
      setStatesProjectId(forProjectId);
      setHistoryMessage(MODEL_STATE_STORE_UNAVAILABLE);
      setHistoryOperation("store_unavailable");
      return;
    }
    try {
      const listed = await listModelStates(forProjectId);
      if (request !== listRequest.current) return;
      setStates(listed);
      setStatesProjectId(forProjectId);
    } catch (error) {
      if (request !== listRequest.current) return;
      setStates(null);
      setStatesProjectId(forProjectId);
      setHistoryMessage(`List failed: ${nativeFailureText(error)}`);
      setHistoryOperation("list_failed");
    }
  }

  useEffect(() => {
    void refreshStates(projectId);
  }, [projectId, deps.projectSessionGeneration]);

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
  async function saveState(form: SaveModelStateForm): Promise<boolean> {
    let requestModel = depsRef.current.currentModel.current;
    if (!requestModel) return false;
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
      depsRef.current.adoptStateSnapshot(opened, hash);
      setBasis({ generation: depsRef.current.projectSessionGenerationRef.current, projectId: requestProjectId, stateId });
      setHistoryMessage(`Opened model state ${modelStateDescription(opened.state)} as the working model. Edits make a new working model; the saved state does not change.`);
      setHistoryOperation("open_state");
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

  return {
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

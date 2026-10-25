// The session's model-state history (W-12 S1 and S2; DEL-14-01 Design
// `desktop-states-runs-comparison.md` §2, §3): Save Model State…, Open State,
// Record Run, the History panel's lists of states and recorded runs, label
// edits, delete, a recorded run's check, Compare… of two saved states (S3a) and
// Compare… of two runs (S3b: two recorded runs, or a recorded run and the current run).
// Called only by the session
// (`useWorkspaceSession`), which hands it the cells and handlers it needs; the
// History panel reads the result through `useSessionHistory()`.
import { useEffect, useRef, useState } from "react";
import type { MutableRefObject } from "react";
import {
  admitModelStateComparison,
  deleteModelState,
  deleteUnreadableModelState,
  editModelStateLabels,
  listModelStates,
  MODEL_STATE_STORE_UNAVAILABLE,
  modelStateDescription,
  readModelState,
  readModelStateForComparison,
  saveModelState,
  stateIsOpenable,
  type ModelStateKind,
  type ModelStateRead,
  type ModelStateSummary,
  type StateLabels
} from "../../services/modelStateService";
import {
  admitAnalysisRunComparison,
  deleteAnalysisRun,
  deleteUnreadableAnalysisRun,
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
import { COMPARISON_STATE_NOT_EXACT, compareModelDocuments, StateComparisonRefusal, type StateComparison } from "../comparison/stateComparison";
import { computeModelHash } from "../../services/hashService";
import { RULE_PACKS_NOT_RESOLVED, type UnreadableRow } from "../../services/historyListEntries";
import { compareRunInputs, RunComparisonRefusal, type RunComparison } from "../comparison/runComparison";
import { CURRENT_RUN_ID, currentRunBytes, currentRunSide, currentRunUnavailableReason, recordedRunSide, type RunComparisonSide } from "../comparison/runComparisonSides";
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

/** Why a received snapshot that does not hash to its state's stored model hash is
 * not used (Open State and Compare…): the webview decodes JSON numbers as
 * binary64, so a stored value such as an integer beyond 2^53 arrives as another
 * number. */
export function notExactReason(state: Pick<ModelStateSummary, "state_name" | "state_id" | "model_hash">): string {
  return `the saved state ${modelStateDescription(state)} holds values the desktop cannot represent exactly (its snapshot as received does not hash to the stored ${state.model_hash.slice(0, 19)}…)`;
}

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

/** A comparison of two saved states of the session's project (W-12 S3a): the
 * earlier-saved state is the left side. View-only; nothing is exported or kept. */
export type StateComparisonResult = {
  projectId: string;
  left: ModelStateSummary;
  right: ModelStateSummary;
  comparison: StateComparison;
};

/** A comparison of two runs of the session's project (W-12 S3b): the earlier-recorded
 * run is the left side, and the current run, when it is one, the right. View-only;
 * nothing is exported or kept. */
export type RunComparisonResult = {
  projectId: string;
  left: RunComparisonSide;
  right: RunComparisonSide;
  comparison: RunComparison;
  /** The current-run slot a comparison with a current side was made from (its result,
   * record and manifest); it is shown only while the session's slot is that one. */
  currentRun?: CurrentRunIdentity | null;
};

/** The identity of a current-run slot: its result, record and manifest objects. */
export type CurrentRunIdentity = Pick<CurrentRunSlot, "result" | "analysisRun" | "inputManifest">;

/** The refusal of a comparison whose current run the session retired or replaced
 * while the comparison was prepared. */
export const COMPARISON_CURRENT_RUN_CHANGED = "COMPARISON-CURRENT-RUN-CHANGED";

/** Whether `slot` is still the current run `identity` names (an edit, a replacement
 * solve, a cleared slot or a project replacement makes it another). */
export function sameCurrentRun(identity: CurrentRunIdentity, slot: CurrentRunSlot): boolean {
  return identity.result === slot.result && identity.analysisRun === slot.analysisRun && identity.inputManifest === slot.inputManifest;
}

/** The stated reason when nothing selected can be compared. */
export const SELECT_TWO_TO_COMPARE = "Select two saved states, or two runs, to compare.";

/** Why Compare… of these states is not offered, or null when it is. */
export function stateCompareUnavailableReason(selected: readonly string[], states: readonly ModelStateSummary[] | null): string | null {
  if (!states) return "No saved states are listed.";
  const listed = selected.filter((id) => states.some((state) => state.state_id === id));
  if (listed.length !== 2) return SELECT_TWO_TO_COMPARE;
  return null;
}

/** Why Compare… of these runs is not offered, or null when it is: two listed recorded
 * runs, or one and the current run, none refused by the store when it was read. */
export function runCompareUnavailableReason(
  selected: readonly string[],
  runs: readonly AnalysisRunSummary[] | null,
  currentRunReason: string | null,
  refusedRuns: Readonly<Record<string, string>>
): string | null {
  const listed = selected.filter((id) => (id === CURRENT_RUN_ID && currentRunReason === null) || runs?.some((run) => run.recording_id === id));
  if (listed.length !== 2) return SELECT_TWO_TO_COMPARE;
  const refused = listed.find((id) => Object.hasOwn(refusedRuns, id));
  return refused ? `The recorded run ${refused} cannot be compared: ${refusedRuns[refused]}` : null;
}

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
  // The rows the store listed as unreadable (W-12): ids and refusals only.
  const [unreadable, setUnreadable] = useState<{ states: UnreadableRow[]; runs: UnreadableRow[] } | null>(null);
  const [runCheck, setRunCheck] = useState<RecordedRunCheck | null>(null);
  const [stateComparison, setStateComparison] = useState<StateComparisonResult | null>(null);
  const [runComparison, setRunComparison] = useState<RunComparisonResult | null>(null);
  // Recorded runs a read refused by name as recorded before rule-pack resolution (S2-era),
  // with the refusal: the list cannot tell them (it reads no manifest), so they are shown
  // as not comparable once a read has said so. Runs are immutable, so it stays true.
  const [refusedRuns, setRefusedRuns] = useState<{ projectId: string | null; runs: Record<string, string> }>({ projectId: null, runs: {} });
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
      setUnreadable(null);
      setStatesProjectId(forProjectId);
      setHistoryMessage(MODEL_STATE_STORE_UNAVAILABLE);
      setHistoryOperation("store_unavailable");
      return;
    }
    try {
      const [listed, listedRuns] = await Promise.all([listModelStates(forProjectId), listAnalysisRuns(forProjectId)]);
      if (request !== listRequest.current) return;
      setStates(listed.states);
      setRuns(listedRuns.runs);
      setUnreadable({ states: listed.unreadable, runs: listedRuns.unreadable });
      setStatesProjectId(forProjectId);
    } catch (error) {
      if (request !== listRequest.current) return;
      setStates(null);
      setRuns(null);
      setUnreadable(null);
      setStatesProjectId(forProjectId);
      setHistoryMessage(`List failed: ${nativeFailureText(error)}`);
      setHistoryOperation("list_failed");
    }
  }

  useEffect(() => {
    void refreshStates(projectId);
    setRunCheck(null);
    setStateComparison(null);
    setRunComparison(null);
  }, [projectId, deps.projectSessionGeneration]);

  /** Keeps a read's by-name refusal of a run recorded before rule-pack resolution. */
  function noteRefusedRun(forProjectId: string, recordingId: string, failure: string) {
    if (!failure.startsWith(RULE_PACKS_NOT_RESOLVED)) return;
    setRefusedRuns((current) => ({ projectId: forProjectId, runs: { ...(current.projectId === forProjectId ? current.runs : {}), [recordingId]: failure } }));
  }

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
      if (!hash || hash.value !== opened.state.model_hash) {
        throw new Error(`MODEL-STATE-INTEGRITY: ${notExactReason(opened.state)}, so it is not opened.`);
      }
      if (opened.model.project.id !== requestProjectId) {
        throw new Error("MODEL-STATE-INTEGRITY: the opened snapshot is not of the saved state's project, so it is not opened.");
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

  /** Deletes a row the store listed as unreadable (W-12), by its handle: a recorded run
   * always (the delete trusts none of it), a saved state only when no stored row names it
   * (the store refuses otherwise, with the counts). Nothing of the row is read. */
  async function deleteUnreadable(kind: "state" | "run", handle: string): Promise<boolean> {
    const projectId = statesProjectId;
    const listed = kind === "state" ? unreadable?.states : unreadable?.runs;
    if (!projectId || !listed?.some((row) => row.handle === handle)) return false;
    const owner = begin();
    if (!owner) return false;
    try {
      // By its handle: a shown id is cut and may name another row.
      const receipt = kind === "state" ? await deleteUnreadableModelState(projectId, handle) : await deleteUnreadableAnalysisRun(projectId, handle);
      setHistoryMessage(receipt.message);
      setHistoryOperation(kind === "state" ? "delete_state" : "delete_run");
      await refreshStates(projectId);
      return true;
    } catch (error) {
      setHistoryMessage(`Not deleted: ${nativeFailureText(error)}`);
      setHistoryOperation(kind === "state" ? "delete_state_failed" : "delete_run_failed");
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
      const failure = nativeFailureText(error);
      noteRefusedRun(run.project_id, recordingId, failure);
      setRunCheck({ recordingId, context: null, failure });
      setHistoryOperation("check_run_failed");
      return false;
    } finally {
      end(owner);
    }
  }

  /** Compare… (W-12 S3a): two saved states of the session's project, read through
   * the metered store reads after the store admits their combined snapshot size,
   * then compared by stable id (`stateComparison.ts`). It goes through the
   * one-owner check and is refused while a prompt or operation holds the session.
   * Nothing about the session or the states changes. */
  async function compareStates(firstStateId: string, secondStateId: string): Promise<boolean> {
    const first = states?.find((candidate) => candidate.state_id === firstStateId);
    const second = states?.find((candidate) => candidate.state_id === secondStateId);
    if (!first || !second || first.state_id === second.state_id || first.project_id !== second.project_id) {
      setHistoryMessage("Not compared: select two different saved states of this project.");
      setHistoryOperation("compare_states_refused");
      return false;
    }
    if (refusedWhileBusy("Not compared", "compare_states_busy")) return false;
    // The earlier-saved state is the left side.
    const [left, right] = first.created_at <= second.created_at ? [first, second] : [second, first];
    const owner = begin();
    if (!owner) return false;
    const generation = depsRef.current.projectSessionGenerationRef.current;
    const sameSession = () => generation === depsRef.current.projectSessionGenerationRef.current;
    try {
      // The store refuses by name, before either snapshot is read, when the two
      // together exceed the per-invocation budget.
      await admitModelStateComparison(left.project_id, left.state_id, right.state_id);
      // The comparison read: integrity-checked as Open's, and a MODEL_INCOMPLETE
      // state (a comparison basis) is read too.
      const leftRead = await readModelStateForComparison(left.project_id, left.state_id);
      const rightRead = await readModelStateForComparison(right.project_id, right.state_id);
      if (leftRead.model.project.id !== left.project_id || rightRead.model.project.id !== right.project_id) {
        throw new Error("MODEL-STATE-INTEGRITY: a read snapshot is not of the saved state's project; nothing was compared.");
      }
      // Fail closed at the native-to-webview boundary: the webview's JSON numbers
      // are binary64, so a stored value such as 9007199254740993 arrives as another
      // number. Each received snapshot must hash, in the webview's canonical form,
      // to the state's stored model hash, or the comparison is refused.
      for (const read of [leftRead, rightRead]) {
        const received = await computeModelHash(read.model);
        if (!received || received.value !== read.state.model_hash) {
          throw new StateComparisonRefusal(COMPARISON_STATE_NOT_EXACT, `${notExactReason(read.state)}, so it is not compared.`);
        }
      }
      const comparison = compareModelDocuments(leftRead.model, rightRead.model);
      if (!sameSession()) return false;
      setStateComparison({ projectId: left.project_id, left: leftRead.state, right: rightRead.state, comparison });
      setRunComparison(null);
      const { added, removed, changed, unchanged } = comparison.summary;
      setHistoryMessage(`Compared ${modelStateDescription(left)} with ${modelStateDescription(right)}: ${added} added, ${removed} removed, ${changed} changed, ${unchanged} unchanged. Neither state changes.`);
      setHistoryOperation("compare_states");
      return true;
    } catch (error) {
      if (!sameSession()) return false;
      setHistoryMessage(`Not compared: ${error instanceof StateComparisonRefusal ? error.message : nativeFailureText(error)}`);
      setHistoryOperation("compare_states_failed");
      return false;
    } finally {
      end(owner);
    }
  }

  const currentRunReason = currentRunUnavailableReason(deps.runSlot, projectId);

  // A shown comparison with a current side holds only while that run is the session's
  // current run: when the slot changes (an edit, another solve, a cleared result, a
  // project replacement) it is closed and the status says why. Recorded-only
  // comparisons are unaffected.
  const staleCurrentComparison = runComparison?.currentRun ? !sameCurrentRun(runComparison.currentRun, deps.runSlot) : false;
  useEffect(() => {
    if (!staleCurrentComparison) return;
    setRunComparison(null);
    setHistoryMessage("Run comparison closed: the current run it compared changed (an edit, another solve or a cleared result); compare again to see the current run.");
    setHistoryOperation("compare_runs_current_changed");
  }, [staleCurrentComparison]);

  /** Compare… of two runs (W-12 S3b): two recorded runs of the session's project, or
   * one and the session's current run. The store first admits the runs' combined
   * stored size, verifying each as its list does; each recorded run is then read
   * through the metered, integrity-checked read (`read_analysis_run`, Check record's
   * read; a run recorded before rule-pack resolution is refused by name) and hashed
   * again here (`recordedRunSide`); the rows are compared under the standing gate
   * (`runComparison.ts`). It goes through the one-owner check and is refused while a
   * prompt or operation holds the session. Nothing about the session or the runs
   * changes. */
  async function compareRuns(firstId: string, secondId: string): Promise<boolean> {
    const requestProjectId = projectId;
    const listed = (id: string) => runs?.find((candidate) => candidate.recording_id === id) ?? null;
    const valid = (id: string) => (id === CURRENT_RUN_ID ? currentRunReason === null : listed(id) !== null);
    if (!requestProjectId || statesProjectId !== requestProjectId || firstId === secondId || !valid(firstId) || !valid(secondId)) {
      setHistoryMessage(`Not compared: select two different runs of this project${currentRunReason && [firstId, secondId].includes(CURRENT_RUN_ID) ? ` (${currentRunReason})` : ""}.`);
      setHistoryOperation("compare_runs_refused");
      return false;
    }
    if (refusedWhileBusy("Not compared", "compare_runs_busy")) return false;
    // The earlier-recorded run is the left side; the current run is the right.
    const order = (id: string) => (id === CURRENT_RUN_ID ? "\uffff" : listed(id)!.created_at);
    const [leftId, rightId] = order(firstId) <= order(secondId) ? [firstId, secondId] : [secondId, firstId];
    const recordingIds = [leftId, rightId].filter((id) => id !== CURRENT_RUN_ID);
    const owner = begin();
    if (!owner) return false;
    const generation = depsRef.current.projectSessionGenerationRef.current;
    const sameSession = () => generation === depsRef.current.projectSessionGenerationRef.current;
    const slot = depsRef.current.runSlot;
    const captured: CurrentRunIdentity = { result: slot.result, analysisRun: slot.analysisRun, inputManifest: slot.inputManifest };
    // Fail closed: the current run the comparison was asked for must still be the
    // session's current run when its side is built and when the result is published.
    const requireCurrentRun = () => {
      if (!sameCurrentRun(captured, depsRef.current.runSlot)) {
        throw new RunComparisonRefusal(COMPARISON_CURRENT_RUN_CHANGED, "the current run changed (an edit, another solve or a cleared result) while the comparison was prepared, so nothing was compared; compare again.");
      }
    };
    try {
      const withCurrent = recordingIds.length === 1;
      // The store refuses by name, before either run is read, when the runs' stored
      // results and manifests (with the current run's) exceed the per-invocation budget.
      await admitAnalysisRunComparison(requestProjectId, recordingIds, withCurrent ? currentRunBytes(slot) : null);
      const side = async (id: string): Promise<RunComparisonSide> => {
        if (id === CURRENT_RUN_ID) { requireCurrentRun(); return currentRunSide(slot); }
        try {
          return await recordedRunSide(await readAnalysisRun(requestProjectId, id));
        } catch (error) {
          if (!(error instanceof RunComparisonRefusal)) noteRefusedRun(requestProjectId, id, nativeFailureText(error));
          throw error;
        }
      };
      const left = await side(leftId);
      const right = await side(rightId);
      const comparison = compareRunInputs(left.input, right.input);
      if (!sameSession()) return false;
      if (withCurrent) requireCurrentRun();
      setRunComparison({ projectId: requestProjectId, left, right, comparison, currentRun: withCurrent ? captured : null });
      setStateComparison(null);
      const { compared, not_compared, unmatched_left, unmatched_right } = comparison.summary;
      setHistoryMessage(`Compared ${left.description} with ${right.description}: ${compared} row pair${compared === 1 ? "" : "s"} compared, ${not_compared} not compared, ${unmatched_left} only in the left run, ${unmatched_right} only in the right run. Neither run changes.`);
      setHistoryOperation("compare_runs");
      return true;
    } catch (error) {
      if (!sameSession()) return false;
      setHistoryMessage(`Not compared: ${error instanceof RunComparisonRefusal ? error.message : nativeFailureText(error)}`);
      setHistoryOperation("compare_runs_failed");
      return false;
    } finally {
      end(owner);
    }
  }

  return {
    /** The last comparison of two runs (view-only), or null. */
    runComparison: runComparison && runComparison.projectId === projectId && !staleCurrentComparison ? runComparison : null,
    compareRuns,
    closeRunComparison: () => setRunComparison(null),
    /** Why the session's current run cannot be compared, or null when it can. */
    currentRunUnavailableReason: currentRunReason,
    /** Recorded runs a read refused by name as recorded before rule-pack resolution, with the refusal. */
    refusedRuns: refusedRuns.projectId === projectId ? refusedRuns.runs : {},
    /** The last comparison of two saved states (view-only), or null. */
    stateComparison: stateComparison && stateComparison.projectId === projectId ? stateComparison : null,
    compareStates,
    closeStateComparison: () => setStateComparison(null),
    runs: statesProjectId === projectId ? runs : null,
    /** The rows the store could not read, by id and refusal only. */
    unreadable: statesProjectId === projectId ? unreadable : null,
    deleteUnreadable,
    runCheck,
    recordRunUnavailableReason: recordable.reason,
    /** The recordable run's input manifest (its rule-pack references), shown in the Record Run form. */
    recordRunManifest: recordable.run?.inputManifest.manifest ?? null,
    /** The recordable run's transient record (its rule-pack reference diagnostics). */
    recordRunRecord: recordable.run?.analysisRun ?? null,
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

// Named immutable model states (W-12 S1; DEL-14-01 Design
// `desktop-states-runs-comparison.md` §2; FR-CMP-001 and FR-MOD-006's state
// level). The native store (`apps/desktop/src-tauri/src/model_states.rs`) is the
// only store of states: it re-canonicalizes and hashes every snapshot, computes
// the record hash and refuses what does not match. The browser preview keeps no
// states (SQLite-only history, design §7), so every call there is refused by name.
import { invokeWithIngress } from "./nativeIngress";
import { partitionListed, type UnreadableRow } from "./historyListEntries";
import { canonicalJsonString, computeModelHash } from "./hashService";
import type { ModelDocumentVersionStatus, PreviewModel } from "../types";

export const MODEL_STATE_KINDS = [
  "design_snapshot",
  "pre_solve_snapshot",
  "post_solve_snapshot",
  "comparison_basis",
  "handoff_basis"
] as const;
export type ModelStateKind = (typeof MODEL_STATE_KINDS)[number];

/** A state-level assumption or open issue, in the model's `unresolved_assumptions` vocabulary. */
export type StateAssumption = {
  assumption_id: string;
  kind: "assumption" | "open_issue";
  statement: string;
  status: "unresolved" | "resolved" | "rejected";
  affected_refs: string[];
  resolution_note?: string;
};

/** A state's editable labels: kept outside the record hash, so editing them does not change the saved state. */
export type StateLabels = {
  state_name: string;
  tags: string[];
  notes: string[];
  assumptions: StateAssumption[];
};

/** One state as the store lists it (columns only; no snapshot). */
export type ModelStateSummary = StateLabels & {
  project_id: string;
  state_id: string;
  state_kind: ModelStateKind;
  created_at: string;
  parent_state_id: string | null;
  /** The snapshot's model hash, `sha256:<hex>`. */
  model_hash: string;
  record_hash: string;
  labels_updated_at: string;
  /** Saved states whose parent this is, each verified (W-12); they block its deletion. */
  child_state_ids: string[];
  /** How many recorded-run rows name this state, counted from their columns (W-12); they
   * block its deletion. The runs themselves come from the run list, each verified. */
  recorded_run_count?: number;
  /** How many saved-state rows name this state as their parent but cannot be read (W-12);
   * counted, never listed. They block its deletion too. */
  unreadable_references?: number;
  /** The record's analysis status. `MODEL_INCOMPLETE`: blocking document
   * findings; the state is kept (a comparison basis) but cannot be opened. */
  analysis_status: string[];
};

/** A state whose snapshot has blocking document findings is not opened as the working model. */
export function stateIsOpenable(state: Pick<ModelStateSummary, "analysis_status">): boolean {
  return !state.analysis_status.includes("MODEL_INCOMPLETE");
}

export type ModelStateSaveOutcome = {
  outcome: "saved" | "already_saved";
  state: ModelStateSummary;
  message: string;
};

export type ModelStateRead = {
  state: ModelStateSummary;
  /** The emitted `model_state.schema.json` `ModelStateRecord` (labels joined in). */
  record: Record<string, unknown>;
  model: PreviewModel;
  model_document_version: ModelDocumentVersionStatus;
};

export type ModelStateDeleteReceipt = { project_id: string; state_id: string; deleted: boolean; message: string };

export const MODEL_STATE_STORE_UNAVAILABLE =
  "MODEL-STATE-STORE-UNAVAILABLE: saved model states and recorded runs live only in the desktop app's local SQLite store; the browser preview keeps none.";

export const LABELS_DO_NOT_CHANGE_STATE = "Renaming, tagging or adding notes does not change the saved state.";

function hasTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function requireNativeStore(): void {
  if (!hasTauriRuntime()) throw new Error(MODEL_STATE_STORE_UNAVAILABLE);
}

export type SaveModelStateInput = {
  /** The stored envelope hash value the session last opened or saved (its base). */
  expectedBase: string;
  model: PreviewModel;
  kind: ModelStateKind;
  parentStateId: string | null;
  labels: StateLabels;
};

/** Saves the model as a state of its project. The snapshot is the model's
 * canonical JCS text and the claim is its model hash, both from the engine's
 * hashing seam; the native store recomputes both and refuses a mismatch. */
export async function saveModelState(input: SaveModelStateInput): Promise<ModelStateSaveOutcome> {
  requireNativeStore();
  const [modelCanonical, modelHash] = await Promise.all([canonicalJsonString(input.model), computeModelHash(input.model)]);
  if (!modelHash) throw new Error("MODEL-STATE-HASH-UNAVAILABLE: the model hash could not be computed; nothing was saved.");
  return invokeWithIngress<ModelStateSaveOutcome>("save_model_state", {
    request: {
      project_id: input.model.project.id,
      expected_base: input.expectedBase,
      model_canonical: modelCanonical,
      model_hash: modelHash.value,
      state_kind: input.kind,
      parent_state_id: input.parentStateId,
      labels: input.labels
    }
  });
}

/** The project's saved states: the readable summaries and, separately, each row the
 * store cannot read, by its id and refusal only (`historyListEntries.ts`). */
export async function listModelStates(projectId: string): Promise<{ states: ModelStateSummary[]; unreadable: UnreadableRow[] }> {
  requireNativeStore();
  const { readable, unreadable } = partitionListed(await invokeWithIngress<Array<ModelStateSummary | { unreadable: UnreadableRow }>>("list_model_states", { projectId }));
  return { states: readable, unreadable };
}

export async function readModelState(projectId: string, stateId: string): Promise<ModelStateRead> {
  requireNativeStore();
  return invokeWithIngress<ModelStateRead>("read_model_state", { projectId, stateId });
}

export async function editModelStateLabels(
  projectId: string,
  stateId: string,
  labels: StateLabels,
  expectedUpdatedAt: string | null
): Promise<ModelStateSummary> {
  requireNativeStore();
  return invokeWithIngress<ModelStateSummary>("edit_model_state_labels", { projectId, stateId, labels, expectedUpdatedAt });
}

export async function deleteModelState(projectId: string, stateId: string): Promise<ModelStateDeleteReceipt> {
  requireNativeStore();
  return invokeWithIngress<ModelStateDeleteReceipt>("delete_model_state", { projectId, stateId });
}

/** Deletes the state row an unreadable list entry names, by its handle (never its shown id). */
export async function deleteUnreadableModelState(projectId: string, unreadableHandle: string): Promise<ModelStateDeleteReceipt> {
  requireNativeStore();
  return invokeWithIngress<ModelStateDeleteReceipt>("delete_model_state", { projectId, unreadableHandle });
}

export function modelStateDescription(state: Pick<ModelStateSummary, "state_name" | "state_id">): string {
  return `"${state.state_name}" (${state.state_id})`;
}

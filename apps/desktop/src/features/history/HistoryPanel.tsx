import { useState } from "react";
import { History } from "lucide-react";
import {
  LABELS_DO_NOT_CHANGE_STATE,
  MODEL_STATE_KINDS,
  modelStateDescription,
  stateIsOpenable,
  type ModelStateKind,
  type ModelStateSummary,
  type StateAssumption,
  type StateLabels
} from "../../services/modelStateService";
import { useSessionHistory, useSessionModel, useSessionProject } from "../workspace/WorkspaceSessionContext";
import { notOpenableReason } from "./useModelStateHistory";

// The History panel (W-12 S1; DEL-14-01 Design `desktop-states-runs-comparison.md`
// §2, §4.3): the project's saved model states with their labels, Save Model
// State…, Open State, label editing and delete. Recorded runs and Compare… are
// S2/S3. A state is a snapshot for reopening and comparison; nothing here
// approves, accepts or certifies a model (F-PIP-2, F-PIP-3).

const KIND_LABELS: Record<ModelStateKind, string> = {
  design_snapshot: "Design snapshot",
  pre_solve_snapshot: "Pre-solve snapshot",
  post_solve_snapshot: "Post-solve snapshot",
  comparison_basis: "Comparison basis",
  handoff_basis: "Handoff basis"
};

type AssumptionDraft = { assumption_id: string; kind: StateAssumption["kind"]; status: StateAssumption["status"]; statement: string; resolution_note: string; refs: string };
type LabelsDraft = { name: string; tags: string; notes: string; assumptions: AssumptionDraft[] };

let draftSequence = 0;

function draftFrom(labels: StateLabels | null): LabelsDraft {
  return {
    name: labels?.state_name ?? "",
    tags: labels?.tags.join(", ") ?? "",
    notes: labels?.notes.join("\n") ?? "",
    assumptions: (labels?.assumptions ?? []).map((assumption) => ({
      assumption_id: assumption.assumption_id,
      kind: assumption.kind,
      status: assumption.status,
      statement: assumption.statement,
      resolution_note: assumption.resolution_note ?? "",
      refs: assumption.affected_refs.join(", ")
    }))
  };
}

/** The labels a draft names; the native store checks them again. */
export function labelsFromDraft(draft: LabelsDraft): StateLabels {
  const list = (text: string, separator: RegExp) => text.split(separator).map((item) => item.trim()).filter(Boolean);
  return {
    state_name: draft.name.trim(),
    tags: [...new Set(list(draft.tags, /,/))],
    notes: list(draft.notes, /\n/),
    assumptions: draft.assumptions.filter((row) => row.statement.trim()).map((row) => ({
      assumption_id: row.assumption_id,
      kind: row.kind,
      status: row.status,
      statement: row.statement.trim(),
      affected_refs: list(row.refs, /,/),
      ...(row.resolution_note.trim() ? { resolution_note: row.resolution_note.trim() } : {})
    }))
  };
}

function LabelsFields({ draft, onChange, testId }: { draft: LabelsDraft; onChange: (draft: LabelsDraft) => void; testId: string }) {
  const setRow = (index: number, patch: Partial<AssumptionDraft>) =>
    onChange({ ...draft, assumptions: draft.assumptions.map((row, at) => (at === index ? { ...row, ...patch } : row)) });
  return (
    <div className="model-history-fields">
      <label className="memory-pool-field">
        <span>Name (required)</span>
        <input data-testid={`${testId}-name`} value={draft.name} required onChange={(event) => onChange({ ...draft, name: event.target.value })} />
      </label>
      <label className="memory-pool-field">
        <span>Tags (comma separated)</span>
        <input data-testid={`${testId}-tags`} value={draft.tags} onChange={(event) => onChange({ ...draft, tags: event.target.value })} />
      </label>
      <label className="memory-pool-field">
        <span>Notes (one per line)</span>
        <textarea data-testid={`${testId}-notes`} rows={2} value={draft.notes} onChange={(event) => onChange({ ...draft, notes: event.target.value })} />
      </label>
      <fieldset className="model-history-assumptions">
        <legend>State-level assumptions and open issues</legend>
        {draft.assumptions.map((row, index) => (
          <div className="model-history-assumption" key={row.assumption_id} data-testid={`${testId}-assumption-${index}`}>
            <select aria-label="Kind" value={row.kind} onChange={(event) => setRow(index, { kind: event.target.value as AssumptionDraft["kind"] })}>
              <option value="assumption">Assumption</option>
              <option value="open_issue">Open issue</option>
            </select>
            <select aria-label="Status" value={row.status} onChange={(event) => setRow(index, { status: event.target.value as AssumptionDraft["status"] })}>
              <option value="unresolved">Unresolved</option>
              <option value="resolved">Resolved</option>
              <option value="rejected">Rejected</option>
            </select>
            <input aria-label="Statement" placeholder="Statement" value={row.statement} onChange={(event) => setRow(index, { statement: event.target.value })} />
            <input aria-label="Affected entity ids" placeholder="Affected entity ids (comma separated)" value={row.refs} onChange={(event) => setRow(index, { refs: event.target.value })} />
            {row.status !== "unresolved" ? (
              <input aria-label="Resolution note" placeholder="Resolution note" value={row.resolution_note} onChange={(event) => setRow(index, { resolution_note: event.target.value })} />
            ) : null}
            <button type="button" onClick={() => onChange({ ...draft, assumptions: draft.assumptions.filter((_, at) => at !== index) })}>Remove</button>
          </div>
        ))}
        <button
          type="button"
          data-testid={`${testId}-add-assumption`}
          onClick={() => onChange({
            ...draft,
            assumptions: [...draft.assumptions, {
              assumption_id: `assumption:state-${Date.now().toString(36)}-${++draftSequence}`,
              kind: "assumption", status: "unresolved", statement: "", resolution_note: "", refs: ""
            }]
          })}
        >
          Add assumption or open issue
        </button>
      </fieldset>
    </div>
  );
}

function SaveStateForm({ onDone }: { onDone: () => void }) {
  const { saveState, historyBusy } = useSessionHistory();
  const [draft, setDraft] = useState<LabelsDraft>(() => draftFrom(null));
  const [kind, setKind] = useState<ModelStateKind>("design_snapshot");
  const labels = labelsFromDraft(draft);
  return (
    <form
      className="model-history-form"
      data-testid="model-state-save-form"
      aria-label="Save Model State"
      onSubmit={(event) => {
        event.preventDefault();
        void saveState({ labels, kind }).then((saved) => { if (saved) onDone(); });
      }}
    >
      <LabelsFields draft={draft} onChange={setDraft} testId="model-state-save" />
      <label className="memory-pool-field">
        <span>Kind</span>
        <select data-testid="model-state-save-kind" value={kind} onChange={(event) => setKind(event.target.value as ModelStateKind)}>
          {MODEL_STATE_KINDS.map((option) => <option key={option} value={option}>{KIND_LABELS[option]}</option>)}
        </select>
      </label>
      <div className="report-actions">
        <button type="submit" data-testid="model-state-save-submit" disabled={historyBusy || !labels.state_name}>Save state</button>
        <button type="button" onClick={onDone}>Cancel</button>
      </div>
    </form>
  );
}

function StateRow({ state, states }: { state: ModelStateSummary; states: ModelStateSummary[] }) {
  const { openState, editLabels, deleteState, historyBusy, basisStateId } = useSessionHistory();
  const [editing, setEditing] = useState<LabelsDraft | null>(null);
  const [confirmingDelete, setConfirmingDelete] = useState(false);
  const named = (id: string) => {
    const found = states.find((candidate) => candidate.state_id === id);
    return found ? modelStateDescription(found) : id;
  };
  const testId = `model-state-${state.state_id}`;
  return (
    <li className="model-history-state" data-testid={testId}>
      <div className="model-history-state-head">
        <strong data-testid={`${testId}-name`}>{state.state_name}</strong>
        <span className="muted"> · {KIND_LABELS[state.state_kind] ?? state.state_kind} · saved {state.created_at}</span>
        {basisStateId === state.state_id ? <span className="muted" data-testid={`${testId}-basis`}> · working model's basis</span> : null}
      </div>
      <p className="muted model-history-meta">
        {state.state_id} · model {state.model_hash.slice(0, 19)}…
        {state.parent_state_id ? <> · from {named(state.parent_state_id)}</> : null}
      </p>
      {state.tags.length ? <p className="model-history-meta" data-testid={`${testId}-tags`}>Tags: {state.tags.join(", ")}</p> : null}
      {state.notes.map((note, index) => <p className="model-history-meta" key={index}>{note}</p>)}
      {state.assumptions.length ? (
        <ul className="model-history-meta">
          {state.assumptions.map((assumption) => (
            <li key={assumption.assumption_id}>{assumption.kind === "open_issue" ? "Open issue" : "Assumption"} ({assumption.status}): {assumption.statement}</li>
          ))}
        </ul>
      ) : null}
      {editing ? (
        <form
          className="model-history-form"
          data-testid={`${testId}-labels-form`}
          aria-label={`Edit labels of ${state.state_name}`}
          onSubmit={(event) => {
            event.preventDefault();
            void editLabels(state.state_id, labelsFromDraft(editing)).then((saved) => { if (saved) setEditing(null); });
          }}
        >
          <p className="muted" data-testid="model-state-labels-note">{LABELS_DO_NOT_CHANGE_STATE}</p>
          <LabelsFields draft={editing} onChange={setEditing} testId={`${testId}-labels`} />
          <div className="report-actions">
            <button type="submit" data-testid={`${testId}-labels-save`} disabled={historyBusy || !editing.name.trim()}>Save labels</button>
            <button type="button" onClick={() => setEditing(null)}>Cancel</button>
          </div>
        </form>
      ) : null}
      {!stateIsOpenable(state) ? (
        <p className="muted" data-testid={`${testId}-not-openable`}>Cannot be opened: {notOpenableReason(state)}</p>
      ) : null}
      {state.child_state_ids.length ? (
        <p className="muted" data-testid={`${testId}-delete-blocked`}>
          Delete is blocked while saved state{state.child_state_ids.length === 1 ? "" : "s"} {state.child_state_ids.map(named).join(", ")} name{state.child_state_ids.length === 1 ? "s" : ""} it as parent.
        </p>
      ) : null}
      {confirmingDelete ? (
        <div className="report-actions" data-testid={`${testId}-delete-confirm`}>
          <span>Delete this saved state? This cannot be undone.</span>
          <button type="button" data-testid={`${testId}-delete-cancel`} onClick={() => setConfirmingDelete(false)}>Cancel</button>
          <button type="button" data-testid={`${testId}-delete-yes`} disabled={historyBusy} onClick={() => { setConfirmingDelete(false); void deleteState(state.state_id); }}>Delete state</button>
        </div>
      ) : (
        <div className="report-actions">
          <button type="button" data-testid={`${testId}-open`} disabled={historyBusy || !stateIsOpenable(state)} onClick={() => void openState(state.state_id)}>Open state</button>
          <button type="button" data-testid={`${testId}-edit`} disabled={historyBusy || editing !== null} onClick={() => setEditing(draftFrom(state))}>Edit labels</button>
          <button type="button" data-testid={`${testId}-delete`} disabled={historyBusy || state.child_state_ids.length > 0} onClick={() => setConfirmingDelete(true)}>Delete…</button>
        </div>
      )}
    </li>
  );
}

export function HistoryPanel() {
  const { states, historyMessage, historyBusy, refreshStates } = useSessionHistory();
  const { model } = useSessionModel();
  const { projectBusy } = useSessionProject();
  const [saving, setSaving] = useState(false);
  if (!model) return null;
  return (
    <section className="panel model-history-panel" aria-label="Model history" data-testid="model-history-panel">
      <div className="panel-title">
        <History size={16} aria-hidden="true" />
        Model History
      </div>
      <p className="muted">
        Saved model states of project {model.project.id}: immutable snapshots of the model, kept in the local store for
        reopening and comparison. Human review is still required for any use of them.
      </p>
      <div className="report-actions">
        <button type="button" data-testid="model-state-save-open" disabled={saving || historyBusy || projectBusy} onClick={() => setSaving(true)}>
          Save Model State…
        </button>
        <button type="button" data-testid="model-history-refresh" disabled={historyBusy} onClick={() => void refreshStates()}>
          Refresh
        </button>
      </div>
      {saving ? <SaveStateForm onDone={() => setSaving(false)} /> : null}
      <p className="muted" role="status" data-testid="model-history-message">{historyMessage}</p>
      {states ? (
        states.length ? (
          <ul className="model-history-list" data-testid="model-history-list">
            {states.map((state) => <StateRow key={`${state.state_id}:${state.labels_updated_at}`} state={state} states={states} />)}
          </ul>
        ) : (
          <p className="muted" data-testid="model-history-empty">No saved model states for this project yet.</p>
        )
      ) : null}
    </section>
  );
}

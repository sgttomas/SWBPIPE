import { restoredRecordDiagnostics } from "../../services/nativeReopen";
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
import { LABELS_DO_NOT_CHANGE_RUN, runDescription, type AnalysisRunSummary, type RunLabels } from "../../services/analysisRunService";
import { recordedRunBindingFindings } from "../results/HistoricalRunContext";
import { RulePackRunReferences } from "../rule-packs/RulePackRunReferences";
import { RULE_PACKS_NOT_RESOLVED, unreadableGuidance, type UnreadableRow } from "../../services/historyListEntries";
import { useSessionHistory, useSessionModel, useSessionProject } from "../workspace/WorkspaceSessionContext";
import { notOpenableReason, runCompareUnavailableReason, stateCompareUnavailableReason } from "./useModelStateHistory";
import { StateComparisonView } from "../comparison/StateComparisonView";
import { RunComparisonView } from "../comparison/RunComparisonView";
import { CURRENT_RUN_DESCRIPTION, CURRENT_RUN_ID } from "../comparison/runComparisonSides";

// The History panel (W-12 S1 and S2; DEL-14-01 Design
// `desktop-states-runs-comparison.md` §2, §3, §4.3): the project's saved model
// states with their labels, Save Model State…, Open State, label editing and
// delete; and under each state its recorded analysis runs, with Record Run…,
// their labels, a check against the stored state and delete; and Compare… of two
// selected saved states (S3a, design §4.1, §4.3) or of two selected runs (S3b, §4.2:
// two recorded runs, or one and the session's current run), whose result is shown
// below the lists (view-only). A state is a snapshot for reopening
// and comparison and a recorded run is a review and reproducibility record;
// nothing here approves, accepts or
// certifies a model or a run (F-PIP-2, F-PIP-3).

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

/** The stated reason of a control disabled while a project operation owns the session. */
const BUSY_REASON = "Another project operation or prompt is in progress.";

/** Why an unreadable saved state is not offered for comparison. */
const UNREADABLE_NOT_COMPARED = "This stored row cannot be read, so it cannot be compared.";
/** Why a third saved state cannot be selected for comparison. */
const TWO_SELECTED = "Two saved states are selected; clear one to choose another.";
/** Why a third run cannot be selected for comparison. */
const TWO_RUNS_SELECTED = "Two runs are selected; clear one to choose another.";

/** A run's compare selection: its checkbox, disabled with the stated reason. */
type RunSelection = { selected: boolean; selectionFull: boolean; onToggle: (id: string) => void };

function RunCompareSelect({ id, testId, selection, refusal }: { id: string; testId: string; selection: RunSelection; refusal: string | null }) {
  const disabledReason = refusal ? `Not compared: ${refusal}` : !selection.selected && selection.selectionFull ? TWO_RUNS_SELECTED : null;
  return (
    <label className="model-history-compare-select">
      <input
        type="checkbox"
        data-testid={`${testId}-compare-select`}
        checked={selection.selected && !refusal}
        disabled={disabledReason !== null}
        title={disabledReason ?? undefined}
        aria-describedby={refusal ? `${testId}-not-compared` : undefined}
        onChange={() => selection.onToggle(id)}
      />
      <span> Select to compare</span>
      {refusal ? <span id={`${testId}-not-compared`} className="muted"> Not compared: {refusal}</span> : null}
    </label>
  );
}

/** A disabled control's stated reason (the dead-control audit's accessible reason). */
function reasonFor(busy: boolean, other: string | null = null): string | undefined {
  if (busy) return BUSY_REASON;
  return other ?? undefined;
}

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

function runLabelsFromDraft(draft: LabelsDraft): RunLabels {
  const { state_name, ...rest } = labelsFromDraft(draft);
  return { run_name: state_name, ...rest };
}

function runDraftFrom(labels: RunLabels | null): LabelsDraft {
  return draftFrom(labels ? { ...labels, state_name: labels.run_name } : null);
}

const RUN_ASSUMPTIONS_LEGEND = "Run-level assumptions and open issues";
/** analysis_run 0.3 has no assumption slot (design §3). */
const RUN_ASSUMPTIONS_NOTE = "Run-level assumptions are kept and shown with the run; they are not part of its analysis record.";

function LabelsFields({ draft, onChange, testId, legend = "State-level assumptions and open issues" }: { draft: LabelsDraft; onChange: (draft: LabelsDraft) => void; testId: string; legend?: string }) {
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
        <legend>{legend}</legend>
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
  const { projectBusy } = useSessionProject();
  const busy = historyBusy || projectBusy;
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
        <button type="submit" data-testid="model-state-save-submit" disabled={busy || !labels.state_name} title={reasonFor(busy, labels.state_name ? null : "A model state needs a name.")}>Save state</button>
        <button type="button" onClick={onDone}>Cancel</button>
      </div>
    </form>
  );
}

function RunRow({ run, selection }: { run: AnalysisRunSummary; selection: RunSelection }) {
  const { editRunLabels, deleteRun, checkRun, runCheck, historyBusy, refusedRuns } = useSessionHistory();
  const { projectBusy } = useSessionProject();
  const busy = historyBusy || projectBusy;
  const [editing, setEditing] = useState<LabelsDraft | null>(null);
  const [confirmingDelete, setConfirmingDelete] = useState(false);
  const testId = `recorded-run-${run.recording_id}`;
  const check = runCheck?.recordingId === run.recording_id ? runCheck : null;
  const bindingFindings = check?.context ? recordedRunBindingFindings(check.context) : [];
  return (
    <li className="model-history-run" data-testid={testId}>
      <div className="model-history-state-head">
        <strong data-testid={`${testId}-name`}>{run.run_name}</strong>
        <span className="muted"> · recorded run · recorded {run.created_at}</span>
        <RunCompareSelect id={run.recording_id} testId={testId} selection={selection} refusal={refusedRuns?.[run.recording_id] ?? null} />
      </div>
      <p className="muted model-history-meta">
        {run.recording_id} · producer run {run.producer_run_id} · {run.analysis_status.join(", ")}
      </p>
      {run.tags.length ? <p className="model-history-meta" data-testid={`${testId}-tags`}>Tags: {run.tags.join(", ")}</p> : null}
      {run.notes.map((note, index) => <p className="model-history-meta" key={index}>{note}</p>)}
      {run.assumptions.length ? (
        <ul className="model-history-meta" data-testid={`${testId}-assumptions`}>
          {run.assumptions.map((assumption) => (
            <li key={assumption.assumption_id}>{assumption.kind === "open_issue" ? "Open issue" : "Assumption"} ({assumption.status}): {assumption.statement}</li>
          ))}
        </ul>
      ) : null}
      {check ? (
        <div className="model-history-meta" data-testid={`${testId}-check`} role="status">
          {check.failure ? (
            <>
              <p>Not read: {check.failure}</p>
              {check.failure.startsWith(RULE_PACKS_NOT_RESOLVED) ? <p data-testid={`${testId}-re-record`}>{unreadableGuidance(check.failure)}</p> : null}
            </>
          ) : (
            <>
              <p>{bindingFindings.length
                ? "The recorded run does not match its stored state and record:"
                : "The record, its stored result and input manifest, and its state's model hash agree."} This is a record check only; it establishes no current result.</p>
              {check.context?.findings.length ? <ul>{check.context.findings.map((finding) => <li key={finding}>{finding}</li>)}</ul> : null}
              <RulePackRunReferences manifest={check.manifest} record={check.context?.restored ? restoredRecordDiagnostics(check.context.restored) : check.context?.analysisRun} testId={`${testId}-rule-pack-refs`} />
            </>
          )}
        </div>
      ) : null}
      {editing ? (
        <form
          className="model-history-form"
          data-testid={`${testId}-labels-form`}
          aria-label={`Edit labels of ${run.run_name}`}
          onSubmit={(event) => {
            event.preventDefault();
            void editRunLabels(run.recording_id, runLabelsFromDraft(editing)).then((saved) => { if (saved) setEditing(null); });
          }}
        >
          <p className="muted" data-testid="recorded-run-labels-note">{LABELS_DO_NOT_CHANGE_RUN} {RUN_ASSUMPTIONS_NOTE}</p>
          <LabelsFields draft={editing} onChange={setEditing} testId={`${testId}-labels`} legend={RUN_ASSUMPTIONS_LEGEND} />
          <div className="report-actions">
            <button type="submit" data-testid={`${testId}-labels-save`} disabled={busy || !editing.name.trim()} title={reasonFor(busy, editing.name.trim() ? null : "A recorded run needs a name.")}>Save labels</button>
            <button type="button" onClick={() => setEditing(null)}>Cancel</button>
          </div>
        </form>
      ) : null}
      {confirmingDelete ? (
        <div className="report-actions" data-testid={`${testId}-delete-confirm`}>
          <span>Delete this recorded run and its labels? This cannot be undone.</span>
          <button type="button" data-testid={`${testId}-delete-cancel`} onClick={() => setConfirmingDelete(false)}>Cancel</button>
          <button type="button" data-testid={`${testId}-delete-yes`} disabled={busy} title={reasonFor(busy)} onClick={() => { setConfirmingDelete(false); void deleteRun(run.recording_id); }}>Delete run</button>
        </div>
      ) : (
        <div className="report-actions">
          <button type="button" data-testid={`${testId}-check-run`} disabled={busy} title={reasonFor(busy)} onClick={() => void checkRun(run.recording_id)}>Check record</button>
          <button type="button" data-testid={`${testId}-edit`} disabled={busy || editing !== null} title={reasonFor(busy, editing ? "The labels are being edited below." : null)} onClick={() => setEditing(runDraftFrom(run))}>Edit labels</button>
          <button type="button" data-testid={`${testId}-delete`} disabled={busy} title={reasonFor(busy)} onClick={() => setConfirmingDelete(true)}>Delete…</button>
        </div>
      )}
    </li>
  );
}

/** A row the store could not read (W-12): its id, its named refusal and what to do.
 * Nothing that reads it is offered; delete is (the store refuses a state something
 * depends on, naming its blockers). */
function UnreadableRowItem({ kind, row }: { kind: "state" | "run"; row: UnreadableRow }) {
  const { deleteUnreadable, historyBusy } = useSessionHistory();
  const { projectBusy } = useSessionProject();
  const busy = historyBusy || projectBusy;
  const [confirming, setConfirming] = useState(false);
  const testId = `unreadable-${kind}-${row.handle}`;
  return (
    <li className="model-history-unreadable-row" data-testid={testId}>
      <strong>Unreadable {kind === "state" ? "saved state" : "recorded run"}</strong> <code>{row.id}</code>
      <label className="model-history-compare-select">
        <input type="checkbox" data-testid={`${testId}-compare-select`} disabled checked={false} title={UNREADABLE_NOT_COMPARED} aria-describedby={`${testId}-not-compared`} onChange={() => {}} />
        <span id={`${testId}-not-compared`} className="muted"> Not compared: {UNREADABLE_NOT_COMPARED}</span>
      </label>
      <p className="model-history-meta" role="note" data-testid={`${testId}-refusal`}>{row.refusal}</p>
      <p className="model-history-meta" data-testid={`${testId}-guidance`}>{unreadableGuidance(row.refusal)}</p>
      {row.references ? (
        <p className="model-history-meta" data-testid={`${testId}-references`}>
          {row.references} stored row{row.references === 1 ? " names" : "s name"} it; delete {row.references === 1 ? "that row" : "those rows"} first.
        </p>
      ) : null}
      {confirming ? (
        <div className="report-actions" role="group" aria-label={`Confirm delete of ${row.id}`}>
          <span>Delete this unreadable row? This cannot be undone.</span>
          <button type="button" data-testid={`${testId}-delete-yes`} disabled={busy} title={reasonFor(busy)} onClick={() => { setConfirming(false); void deleteUnreadable(kind, row.handle); }}>Delete</button>
          <button type="button" onClick={() => setConfirming(false)}>Cancel</button>
        </div>
      ) : (
        <div className="report-actions">
          <button type="button" data-testid={`${testId}-delete`} disabled={busy || row.references > 0} title={reasonFor(busy, row.references > 0 ? "Stored rows name this state; delete them first." : null)} onClick={() => setConfirming(true)}>Delete…</button>
        </div>
      )}
    </li>
  );
}

function RecordRunForm({ onDone }: { onDone: () => void }) {
  const { recordRun, recordRunState, recordRunUnavailableReason, recordRunManifest, recordRunRecord, historyBusy } = useSessionHistory();
  const { projectBusy } = useSessionProject();
  const busy = historyBusy || projectBusy;
  const [draft, setDraft] = useState<LabelsDraft>(() => runDraftFrom(null));
  const [snapshotName, setSnapshotName] = useState("");
  const labels = runLabelsFromDraft(draft);
  const needsSnapshot = !recordRunState;
  const missing = recordRunUnavailableReason
    ?? (!labels.run_name ? "A recorded run needs a name." : needsSnapshot && !snapshotName.trim() ? "Name the post-solve snapshot the run is recorded against." : null);
  return (
    <form
      className="model-history-form"
      data-testid="record-run-form"
      aria-label="Record Run"
      onSubmit={(event) => {
        event.preventDefault();
        void recordRun({ labels, snapshotName: needsSnapshot ? snapshotName : null }).then((recorded) => { if (recorded) onDone(); });
      }}
    >
      <p className="muted" data-testid="record-run-state">
        {recordRunState
          ? <>The run is recorded against the saved state {modelStateDescription(recordRunState)}, which holds the solved model.</>
          : <>No saved state holds the solved model. The run is recorded against a new post-solve snapshot of the solved model, written with it.</>}
      </p>
      {needsSnapshot ? (
        <label className="memory-pool-field">
          <span>Post-solve snapshot name (required)</span>
          <input data-testid="record-run-snapshot-name" value={snapshotName} required onChange={(event) => setSnapshotName(event.target.value)} />
        </label>
      ) : null}
      <RulePackRunReferences manifest={recordRunManifest} record={recordRunRecord} testId="record-run-rule-pack-refs" />
      <LabelsFields draft={draft} onChange={setDraft} testId="record-run" legend={RUN_ASSUMPTIONS_LEGEND} />
      <p className="muted">{LABELS_DO_NOT_CHANGE_RUN} {RUN_ASSUMPTIONS_NOTE}</p>
      <div className="report-actions">
        <button type="submit" data-testid="record-run-submit" disabled={busy || missing !== null} title={reasonFor(busy, missing)}>Record run</button>
        <button type="button" onClick={onDone}>Cancel</button>
      </div>
    </form>
  );
}

function StateRow({ state, states, runs, selected, selectionFull, onToggleSelect, runSelection }: {
  state: ModelStateSummary;
  states: ModelStateSummary[];
  runs: AnalysisRunSummary[];
  selected: boolean;
  /** Two other states are already selected for comparison. */
  selectionFull: boolean;
  onToggleSelect: (stateId: string) => void;
  runSelection: (recordingId: string) => RunSelection;
}) {
  const { openState, editLabels, deleteState, historyBusy, basisStateId } = useSessionHistory();
  const { projectBusy } = useSessionProject();
  const busy = historyBusy || projectBusy;
  const [editing, setEditing] = useState<LabelsDraft | null>(null);
  const [confirmingDelete, setConfirmingDelete] = useState(false);
  const named = (id: string) => {
    const found = states.find((candidate) => candidate.state_id === id);
    return found ? modelStateDescription(found) : id;
  };
  const testId = `model-state-${state.state_id}`;
  // Rows naming this state that the lists could not present as readable runs under it
  // (unreadable children, run rows not listed as readable): counted, never listed by id.
  const unreadableNaming = (state.unreadable_references ?? 0) + Math.max(0, (state.recorded_run_count ?? runs.length) - runs.length);
  return (
    <li className="model-history-state" data-testid={testId}>
      <div className="model-history-state-head">
        <strong data-testid={`${testId}-name`}>{state.state_name}</strong>
        <span className="muted"> · {KIND_LABELS[state.state_kind] ?? state.state_kind} · saved {state.created_at}</span>
        {basisStateId === state.state_id ? <span className="muted" data-testid={`${testId}-basis`}> · working model's basis</span> : null}
        <label className="model-history-compare-select">
          <input
            type="checkbox"
            data-testid={`${testId}-compare-select`}
            checked={selected}
            disabled={!selected && selectionFull}
            title={!selected && selectionFull ? TWO_SELECTED : undefined}
            onChange={() => onToggleSelect(state.state_id)}
          />
          <span> Select to compare</span>
        </label>
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
            <button type="submit" data-testid={`${testId}-labels-save`} disabled={busy || !editing.name.trim()} title={reasonFor(busy, editing.name.trim() ? null : "A model state needs a name.")}>Save labels</button>
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
      {runs.length ? (
        <p className="muted" data-testid={`${testId}-delete-blocked-runs`}>
          Delete is blocked while recorded run{runs.length === 1 ? "" : "s"} {runs.map(runDescription).join(", ")} {runs.length === 1 ? "is" : "are"} tied to it.
        </p>
      ) : null}
      {unreadableNaming ? (
        <p className="muted" data-testid={`${testId}-delete-blocked-unreadable`}>
          Delete is blocked while {unreadableNaming} stored row{unreadableNaming === 1 ? " that cannot be read names" : "s that cannot be read name"} it; they are listed below.
        </p>
      ) : null}
      {confirmingDelete ? (
        <div className="report-actions" data-testid={`${testId}-delete-confirm`}>
          <span>Delete this saved state? This cannot be undone.</span>
          <button type="button" data-testid={`${testId}-delete-cancel`} onClick={() => setConfirmingDelete(false)}>Cancel</button>
          <button type="button" data-testid={`${testId}-delete-yes`} disabled={busy} title={reasonFor(busy)} onClick={() => { setConfirmingDelete(false); void deleteState(state.state_id); }}>Delete state</button>
        </div>
      ) : (
        <div className="report-actions">
          <button type="button" data-testid={`${testId}-open`} disabled={busy || !stateIsOpenable(state)} title={reasonFor(busy, stateIsOpenable(state) ? null : notOpenableReason(state))} onClick={() => void openState(state.state_id)}>Open state</button>
          <button type="button" data-testid={`${testId}-edit`} disabled={busy || editing !== null} title={reasonFor(busy, editing ? "The labels are being edited below." : null)} onClick={() => setEditing(draftFrom(state))}>Edit labels</button>
          <button type="button" data-testid={`${testId}-delete`} disabled={busy || state.child_state_ids.length > 0 || runs.length > 0 || unreadableNaming > 0} title={reasonFor(busy, state.child_state_ids.length ? "Saved states name this state as their parent." : runs.length ? "Recorded runs are tied to this state." : unreadableNaming ? "Stored rows that cannot be read name this state." : null)} onClick={() => setConfirmingDelete(true)}>Delete…</button>
        </div>
      )}
      {runs.length ? (
        <ul className="model-history-list model-history-runs" data-testid={`${testId}-runs`} aria-label={`Recorded runs of ${state.state_name}`}>
          {runs.map((run) => <RunRow key={`${run.recording_id}:${run.labels_updated_at}`} run={run} selection={runSelection(run.recording_id)} />)}
        </ul>
      ) : null}
    </li>
  );
}

export function HistoryPanel() {
  const {
    states, runs, unreadable, historyMessage, historyBusy, refreshStates, recordRunUnavailableReason, compareStates, stateComparison, closeStateComparison,
    compareRuns, runComparison, closeRunComparison, currentRunUnavailableReason, refusedRuns, refusedStateExport, exportRefusedState
  } = useSessionHistory();
  const { model } = useSessionModel();
  const { projectBusy } = useSessionProject();
  const [saving, setSaving] = useState(false);
  const [recording, setRecording] = useState(false);
  // The saved states selected for Compare… (at most two), by id.
  const [selected, setSelected] = useState<string[]>([]);
  const selectedListed = states ? selected.filter((id) => states.some((state) => state.state_id === id)) : [];
  // The runs selected for Compare… (at most two): recorded runs by id, the current run as
  // CURRENT_RUN_ID. Selecting a run clears the selected states, and selecting a state the runs.
  const [selectedRuns, setSelectedRuns] = useState<string[]>([]);
  const runListed = (id: string) => (id === CURRENT_RUN_ID ? !currentRunUnavailableReason : (runs ?? []).some((run) => run.recording_id === id));
  const selectedRunsListed = selectedRuns.filter(runListed);
  const comparingRuns = selectedRunsListed.length > 0;
  const compareUnavailable = comparingRuns
    ? runCompareUnavailableReason(selectedRunsListed, runs, currentRunUnavailableReason ?? null, refusedRuns ?? {})
    : stateCompareUnavailableReason(selectedListed, states);
  const toggleSelect = (stateId: string) => {
    setSelectedRuns([]);
    setSelected((current) => current.includes(stateId) ? current.filter((id) => id !== stateId) : [...current.filter((id) => states?.some((state) => state.state_id === id)), stateId].slice(-2));
  };
  const toggleRun = (id: string) => {
    setSelected([]);
    setSelectedRuns((current) => current.includes(id) ? current.filter((other) => other !== id) : [...current.filter(runListed), id].slice(-2));
  };
  const runSelection = (id: string): RunSelection => ({ selected: selectedRunsListed.includes(id), selectionFull: selectedRunsListed.length >= 2, onToggle: toggleRun });
  // A readable run whose state is not a readable listed state has no row under a state.
  const orphanRuns = states ? (runs ?? []).filter((run) => !states.some((state) => state.state_id === run.state_id)) : [];
  const runsOf = (stateId: string) => (runs ?? []).filter((run) => run.state_id === stateId).sort((left, right) => left.created_at < right.created_at ? -1 : left.created_at > right.created_at ? 1 : 0);
  if (!model) return null;
  return (
    <section className="panel model-history-panel" aria-label="Model history" data-testid="model-history-panel">
      <div className="panel-title">
        <History size={16} aria-hidden="true" />
        Model History
      </div>
      <p className="muted">
        Saved model states of project {model.project.id}: immutable snapshots of the model, kept in the local store for
        reopening and comparison, each with the analysis runs recorded against it. A recorded run ties a solve's result,
        solver, settings, units and load basis to the exact saved model it solved. Human review is still required for any use of them.
      </p>
      <div className="report-actions">
        <button type="button" data-testid="model-state-save-open" disabled={saving || historyBusy || projectBusy} title={reasonFor(historyBusy || projectBusy, saving ? "The Save Model State form is open below." : null)} onClick={() => setSaving(true)}>
          Save Model State…
        </button>
        <button type="button" data-testid="record-run-open" disabled={recording || historyBusy || projectBusy || recordRunUnavailableReason !== null} title={reasonFor(historyBusy || projectBusy, recording ? "The Record Run form is open below." : recordRunUnavailableReason)} onClick={() => setRecording(true)}>
          Record Run…
        </button>
        <button
          type="button"
          data-testid="model-state-compare"
          disabled={historyBusy || projectBusy || compareUnavailable !== null}
          title={reasonFor(historyBusy || projectBusy, compareUnavailable)}
          onClick={() => {
            if (comparingRuns) { const [first, second] = selectedRunsListed; void compareRuns(first, second); return; }
            const [first, second] = selectedListed; void compareStates(first, second);
          }}
        >
          Compare…
        </button>
        <button type="button" data-testid="model-history-refresh" disabled={historyBusy} title={reasonFor(historyBusy)} onClick={() => void refreshStates()}>
          Refresh
        </button>
      </div>
      {recordRunUnavailableReason && !historyBusy && !projectBusy ? <p className="muted" data-testid="record-run-unavailable">Record Run: {recordRunUnavailableReason}</p> : null}
      {saving ? <SaveStateForm onDone={() => setSaving(false)} /> : null}
      {recording ? <RecordRunForm onDone={() => setRecording(false)} /> : null}
      <p className="muted" role="status" data-testid="model-history-message">{historyMessage}</p>
      {refusedStateExport ? (
        <div className="report-actions" data-testid="model-history-refused-state-export">
          <button type="button" data-testid="model-history-export-stored-state" disabled={historyBusy || projectBusy} title={reasonFor(historyBusy || projectBusy, null)} onClick={() => exportRefusedState()}>
            Save the stored document to a file…
          </button>
          <span className="muted">Saves {refusedStateExport.description} exactly as stored, without opening it; add project.units to the file, then open it with Open Model Document….</span>
        </div>
      ) : null}
      <div className="model-history-current-run" data-testid="current-run-entry">
        <strong>Current run</strong> <span className="muted">· {CURRENT_RUN_DESCRIPTION}</span>
        <RunCompareSelect
          id={CURRENT_RUN_ID}
          testId="current-run"
          selection={runSelection(CURRENT_RUN_ID)}
          refusal={currentRunUnavailableReason ?? null}
        />
      </div>
      {states ? (
        states.length ? (
          <ul className="model-history-list" data-testid="model-history-list">
            {states.map((state) => (
              <StateRow
                key={`${state.state_id}:${state.labels_updated_at}`}
                state={state}
                states={states}
                runs={runsOf(state.state_id)}
                selected={selectedListed.includes(state.state_id)}
                selectionFull={selectedListed.length >= 2}
                onToggleSelect={toggleSelect}
                runSelection={runSelection}
              />
            ))}
          </ul>
        ) : unreadable?.states.length ? null : (
          <p className="muted" data-testid="model-history-empty">No saved model states for this project yet.</p>
        )
      ) : null}
      {orphanRuns.length ? (
        <section className="model-history-orphan-runs" aria-label="Runs whose state cannot be read" data-testid="model-history-orphan-runs">
          <strong>Runs whose state cannot be read</strong>
          <p className="muted">These recorded runs are readable, but the saved state they are tied to is not, so they are listed here; each can be checked or deleted. A state is deleted only after every row naming it is.</p>
          <ul className="model-history-list model-history-runs">
            {orphanRuns.map((run) => <RunRow key={`${run.recording_id}:${run.labels_updated_at}`} run={run} selection={runSelection(run.recording_id)} />)}
          </ul>
        </section>
      ) : null}
      {stateComparison ? <StateComparisonView key={`${stateComparison.left.state_id}:${stateComparison.right.state_id}`} result={stateComparison} onClose={closeStateComparison} /> : null}
      {runComparison ? <RunComparisonView key={`${runComparison.left.id}:${runComparison.right.id}`} result={runComparison} onClose={closeRunComparison} /> : null}
      {unreadable && (unreadable.states.length || unreadable.runs.length) ? (
        <section className="model-history-unreadable" aria-label="Unreadable history rows" data-testid="model-history-unreadable">
          <strong>Rows the store cannot read</strong>
          <p className="muted">These stored rows do not verify, so nothing of them is shown, opened, checked or compared; only their ids and the reasons are listed.</p>
          <ul>
            {unreadable.states.map((row) => <UnreadableRowItem key={row.handle} kind="state" row={row} />)}
            {unreadable.runs.map((row) => <UnreadableRowItem key={row.handle} kind="run" row={row} />)}
          </ul>
        </section>
      ) : null}
    </section>
  );
}

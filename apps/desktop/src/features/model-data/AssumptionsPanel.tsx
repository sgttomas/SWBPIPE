// Assumptions and open issues (W-13 §3.3, FR-MOD-006): add, edit, resolve and
// reject the model's unresolved assumptions and open issues, and filter them
// by status and subject. A record names the current selection as its subject,
// or the project when nothing is selected. Resolving or rejecting requires a
// resolution note. Every edit is a structured operation queued for review.
import { useMemo, useState } from "react";
import type { EntityRef, ModelAssumption } from "../../types";
import { RECORD_FAMILIES, disabledBy, nextModelDataId, recordIntent, useReseed } from "./modelDataIntents";
import { useModelDataQueue, type ModelDataQueueProps } from "./useModelDataQueue";

const STATUSES = ["unresolved", "resolved", "rejected"] as const;
const PROJECT_LEVEL = "__project__";

export function subjectOf(selection: EntityRef | null | undefined, projectId: string): string | null {
  if (!selection || selection.type === "project" || selection.type === "diagnostic" || selection.id === projectId) return null;
  return selection.id;
}

export function filterAssumptions(records: ModelAssumption[], status: string, subject: string): ModelAssumption[] {
  return records.filter((record) =>
    (status === "all" || record.status === status) &&
    (subject === "all" ||
      (subject === PROJECT_LEVEL ? (record.affected_refs ?? []).length === 0 : (record.affected_refs ?? []).includes(subject))));
}

export function AssumptionsPanel(props: ModelDataQueueProps & { selection?: EntityRef | null }) {
  const { model, selection } = props;
  const records = model.unresolved_assumptions ?? [];
  const [statusFilter, setStatusFilter] = useState("all");
  const [subjectFilter, setSubjectFilter] = useState("all");
  const [kind, setKind] = useState<ModelAssumption["kind"]>("assumption");
  const [statement, setStatement] = useState("");
  const { busy, error, notice, queue } = useModelDataQueue(props);
  const subject = subjectOf(selection, model.project.id);
  const subjects = useMemo(
    () => Array.from(new Set(records.flatMap((record) => record.affected_refs ?? []))).sort(),
    [records]
  );
  const shown = filterAssumptions(records, statusFilter, subjectFilter);

  function add() {
    const record: ModelAssumption = {
      assumption_id: nextModelDataId(model, "assumption:A", props.queuedIntents),
      kind,
      statement: statement.trim(),
      status: "unresolved"
    };
    if (subject) record.affected_refs = [subject];
    void queue((canonicalize) => recordIntent(canonicalize, RECORD_FAMILIES.assumption, "create", record)).then((queued) => {
      if (queued) setStatement("");
    });
  }

  return (
    <section className="panel model-data-panel" aria-label="Assumptions and open issues" data-testid="assumptions-panel">
      <div className="panel-title">Assumptions and open issues</div>
      <div className="model-data-filters">
        <label>Status
          <select aria-label="Filter by status" data-testid="assumptions-filter-status" value={statusFilter} onChange={(event) => setStatusFilter(event.target.value)}>
            <option value="all">All</option>
            {STATUSES.map((status) => <option key={status} value={status}>{status}</option>)}
          </select>
        </label>
        <label>Subject
          <select aria-label="Filter by subject" data-testid="assumptions-filter-subject" value={subjectFilter} onChange={(event) => setSubjectFilter(event.target.value)}>
            <option value="all">All</option>
            <option value={PROJECT_LEVEL}>Project level</option>
            {subjects.map((id) => <option key={id} value={id}>{id}</option>)}
          </select>
        </label>
      </div>
      {shown.length === 0 ? <p data-testid="assumptions-empty">No matching records.</p> : (
        <ul className="model-data-list">
          {shown.map((record) => <AssumptionRow key={record.assumption_id} record={record} {...props} />)}
        </ul>
      )}
      <fieldset className="model-data-fieldset">
        <legend>Add</legend>
        <label>Kind
          <select aria-label="Kind" data-testid="assumption-create-kind" value={kind} onChange={(event) => setKind(event.target.value as ModelAssumption["kind"])}>
            <option value="assumption">Assumption</option>
            <option value="open_issue">Open issue</option>
          </select>
        </label>
        <label>Statement
          <textarea aria-label="Statement" data-testid="assumption-create-statement" rows={2} value={statement} onChange={(event) => setStatement(event.target.value)} />
        </label>
        <p className="model-data-note" data-testid="assumption-create-subject">Subject: {subject ?? `project ${model.project.id}`}</p>
        <button type="button" data-testid="assumption-create" {...disabledBy(busy, [!statement.trim(), "Enter a statement."])} onClick={add}>Add record</button>
      </fieldset>
      {error ? <p role="alert">{error}</p> : null}
      {notice ? <p role="status">{notice}</p> : null}
    </section>
  );
}

function AssumptionRow(props: ModelDataQueueProps & { record: ModelAssumption }) {
  const { record } = props;
  const [statement, setStatement] = useState(record.statement);
  const [note, setNote] = useState(record.resolution_note ?? "");
  useReseed(record, () => {
    setStatement(record.statement);
    setNote(record.resolution_note ?? "");
  });
  const { busy, error, queue } = useModelDataQueue(props);
  const family = RECORD_FAMILIES.assumption;
  const update = (next: ModelAssumption) => void queue((canonicalize) => recordIntent(canonicalize, family, "update", next, record));
  const settle = (status: "resolved" | "rejected") => update({ ...record, status, resolution_note: note.trim() });
  const reopen = () => {
    const next: ModelAssumption = { ...record, status: "unresolved" };
    delete next.resolution_note;
    update(next);
  };
  const id = record.assumption_id;
  return (
    <li data-testid={`assumption-row-${id}`}>
      <div>
        <strong>{record.kind === "open_issue" ? "Open issue" : "Assumption"}</strong> <code>{id}</code>{" "}
        <span data-testid={`assumption-status-${id}`}>{record.status}</span>{" "}
        <span>{(record.affected_refs ?? []).length ? `subject ${record.affected_refs!.join(", ")}` : "project level"}</span>
      </div>
      <label>Statement
        <textarea aria-label={`Statement ${id}`} data-testid={`assumption-statement-${id}`} rows={2} value={statement} onChange={(event) => setStatement(event.target.value)} />
      </label>
      <button type="button" data-testid={`assumption-edit-${id}`} {...disabledBy(busy, [!statement.trim(), "Enter a statement."], [statement.trim() === record.statement, "The statement is unchanged."])}
        onClick={() => update({ ...record, statement: statement.trim() })}>
        Save statement
      </button>
      <label>Resolution note
        <input aria-label={`Resolution note ${id}`} data-testid={`assumption-note-${id}`} value={note} onChange={(event) => setNote(event.target.value)} />
      </label>
      <div className="model-data-actions">
        <button type="button" data-testid={`assumption-resolve-${id}`} {...disabledBy(busy, [!note.trim(), "Enter a resolution note first."])} onClick={() => settle("resolved")}>Resolve</button>
        <button type="button" data-testid={`assumption-reject-${id}`} {...disabledBy(busy, [!note.trim(), "Enter a resolution note first."])} onClick={() => settle("rejected")}>Reject</button>
        {record.status !== "unresolved" ? (
          <button type="button" data-testid={`assumption-reopen-${id}`} {...disabledBy(busy)} onClick={reopen}>Reopen</button>
        ) : null}
        <button type="button" data-testid={`assumption-remove-${id}`} {...disabledBy(busy)}
          onClick={() => void queue((canonicalize) => recordIntent(canonicalize, family, "delete", record, record))}>
          Remove
        </button>
      </div>
      {error ? <p role="alert">{error}</p> : null}
    </li>
  );
}

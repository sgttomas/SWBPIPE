// Model tree: the Lines section (W-13 §3.2, §6). A line groups existing pipes
// and is not read by the solve. The user creates a line from the selected
// pipes, renames it, or removes it; each edit is a structured operation queued
// for validation, diff review and apply.
import { useState } from "react";
import type { EntityRef, ModelLine } from "../../types";
import { entityRefFromKey, type OrderedSelectionState } from "../workspace/selectionState";
import { RECORD_FAMILIES, disabledBy, lineOfPipe, nextModelDataId, recordIntent, selectedPipeIds, useReseed } from "./modelDataIntents";
import { useModelDataQueue, type ModelDataQueueProps } from "./useModelDataQueue";

export function LinesSection(props: ModelDataQueueProps & { selectionState?: OrderedSelectionState; selection?: EntityRef }) {
  const { model, selectionState, selection } = props;
  const lines = model.lines ?? [];
  const refs: EntityRef[] = selectionState
    ? selectionState.orderedKeys.flatMap((key) => {
        const ref = entityRefFromKey(key);
        return ref ? [ref] : [];
      })
    : selection ? [selection] : [];
  const pipes = selectedPipeIds(model, refs);
  const lined = pipes.filter((pipe) => lineOfPipe(model, pipe));
  const [label, setLabel] = useState("");
  const { busy, error, notice, queue } = useModelDataQueue(props);
  const family = RECORD_FAMILIES.line;

  function create() {
    const id = nextModelDataId(model, "line:L", props.queuedIntents);
    const record: ModelLine = { id, pipe_refs: pipes, provenance: "user_entered" };
    if (label.trim()) record.label = label.trim();
    void queue((canonicalize) => recordIntent(canonicalize, family, "create", record)).then((queued) => {
      if (queued) setLabel("");
    });
  }

  return (
    <section className="model-tree-lines" aria-label="Lines" data-testid="lines-section">
      <h3>Lines</h3>
      <p className="model-data-note">A line groups pipes; the solve does not read it.</p>
      {lines.length === 0 ? <p data-testid="lines-empty">No lines in this model.</p> : (
        <ul className="model-data-list">
          {lines.map((line) => <LineRow key={line.id} line={line} {...props} />)}
        </ul>
      )}
      <div className="model-data-create">
        <label>New line label
          <input aria-label="New line label" data-testid="line-create-label" value={label} onChange={(event) => setLabel(event.target.value)} />
        </label>
        <button type="button" data-testid="line-create" {...disabledBy(busy, [pipes.length === 0, "Select one or more pipes to group."], [lined.length > 0, "A selected pipe already belongs to a line."])} onClick={create}>
          Create line from selected pipes ({pipes.length})
        </button>
        {lined.length > 0 ? (
          <p role="note" data-testid="line-create-conflict">
            {lined.map((pipe) => `${pipe} belongs to ${lineOfPipe(model, pipe)?.id}`).join("; ")}; a pipe belongs to at most one line.
          </p>
        ) : null}
      </div>
      {error ? <p role="alert">{error}</p> : null}
      {notice ? <p role="status">{notice}</p> : null}
    </section>
  );
}

function LineRow(props: ModelDataQueueProps & { line: ModelLine }) {
  const { line } = props;
  const [name, setName] = useState(line.label ?? "");
  useReseed(line, () => setName(line.label ?? ""));
  const { busy, error, queue } = useModelDataQueue(props);
  const family = RECORD_FAMILIES.line;
  const renamed = (): ModelLine => {
    const next: ModelLine = { ...line };
    if (name.trim()) next.label = name.trim();
    else delete next.label;
    return next;
  };
  return (
    <li data-testid={`line-row-${line.id}`}>
      <strong>{line.label ?? line.id}</strong> <code>{line.id}</code>{" "}
      <span data-testid={`line-pipes-${line.id}`}>{line.pipe_refs.length ? line.pipe_refs.join(", ") : "no pipes"}</span>
      <div className="model-data-actions">
        <input aria-label={`Rename ${line.id}`} data-testid={`line-rename-input-${line.id}`} value={name} onChange={(event) => setName(event.target.value)} />
        <button type="button" data-testid={`line-rename-${line.id}`} {...disabledBy(busy, [(name.trim() || undefined) === line.label, "Enter a different label to rename the line."])}
          onClick={() => void queue((canonicalize) => recordIntent(canonicalize, family, "update", renamed(), line))}>
          Rename
        </button>
        <button type="button" data-testid={`line-remove-${line.id}`} {...disabledBy(busy)}
          onClick={() => void queue((canonicalize) => recordIntent(canonicalize, family, "delete", line, line))}>
          Remove line
        </button>
      </div>
      {error ? <p role="alert">{error}</p> : null}
    </li>
  );
}

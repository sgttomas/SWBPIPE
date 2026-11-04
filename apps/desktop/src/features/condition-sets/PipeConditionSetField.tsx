// Model document 0.9.0 S3b (§4.2): the pipe inspector's "Condition set" field. It assigns
// one of the model's sets, or clears it, through `assign_condition_set`; the engine checks
// the change before it is queued for Review/Apply. No set is suggested or prefilled.
import { useState } from "react";
import type { EntityRef } from "../../types";
import { disabledBy, useReseed } from "../model-data/modelDataIntents";
import { NOT_STATED, assignConditionSetIntent } from "./conditionSetIntents";
import { EngineFeedback, useEngineCheckedQueue, type EngineCheckedQueueProps } from "./engineCheckedQueue";

export function PipeConditionSetField(props: EngineCheckedQueueProps & { selection: EntityRef }) {
  const { model, selection } = props;
  const pipe = selection.type === "pipe" ? model.pipe_segments.find((item) => item.id === selection.id) : undefined;
  const current = pipe?.condition_set_ref ?? "";
  const [chosen, setChosen] = useState(current);
  useReseed([selection.id, current], () => setChosen(current));
  const queue = useEngineCheckedQueue(props);
  if (!pipe) return null;
  const sets = model.condition_sets ?? [];
  const unresolved = current && !sets.some((set) => set.id === current);
  return (
    <section className="editor-intent" aria-label="Pipe condition set" data-testid="pipe-condition-set">
      <h3>Condition set</h3>
      <p data-testid="pipe-condition-set-current">
        {current ? `Carries condition set ${current}.` : `Condition set: ${NOT_STATED}.`}
        {unresolved ? " That set is not in this model (OPERATING_CONDITION_SET_UNRESOLVED)." : ""}
      </p>
      <label>Condition set
        <select aria-label="Condition set" data-testid="pipe-condition-set-select" value={chosen} onChange={(event) => setChosen(event.target.value)}>
          <option value="">{NOT_STATED}</option>
          {unresolved ? <option value={current} disabled>{current} (not in this model)</option> : null}
          {sets.map((set) => <option key={set.id} value={set.id}>{set.label ? `${set.label} (${set.id})` : set.id}</option>)}
        </select>
      </label>
      {sets.length === 0 ? <p className="muted">Create a condition set in the model tree first.</p> : null}
      <button type="button" data-testid="pipe-condition-set-assign"
        {...disabledBy(queue.busy, [!chosen, "Choose a condition set."], [chosen === current, "This pipe already carries that set."])}
        onClick={() => void queue.submit((canonicalize) => assignConditionSetIntent(canonicalize, pipe, chosen))}>
        Check and queue assignment
      </button>
      <button type="button" data-testid="pipe-condition-set-clear" {...disabledBy(queue.busy, [!current, "This pipe carries no condition set."])}
        onClick={() => void queue.submit((canonicalize) => assignConditionSetIntent(canonicalize, pipe, null))}>
        Check and queue clear
      </button>
      <EngineFeedback error={queue.error} notice={queue.notice} diagnostics={queue.diagnostics} testId="pipe-condition-set" />
    </section>
  );
}

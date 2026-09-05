import { useState } from "react";
import type { OperationBatch } from "../../services/operationBatchService";
import { computeModelHash } from "../../services/hashService";
import { generateSelfWeightPlan, type SelfWeightOperationPlan } from "../../services/selfWeightPlanService";
import { draftIntent, usePreparation, type WorkflowProps } from "../offline-proposal-intake/workflowSupport";

export function selfWeightPlanBatch(plan: SelfWeightOperationPlan, sourceHash: string): OperationBatch {
  if (plan.source_model_hash !== sourceHash || plan.scope_label !== "selected_pipe_mass_only" || !plan.changes.length)
    throw new Error("The self-weight plan does not match the current model or supported scope. Generate it again.");
  return {
    batch_id: `batch:self-weight:${crypto.randomUUID()}`,
    operations: plan.changes.map(change => {
      if (change.object_type !== "Load" || change.operation_kind !== "create" ||
          (change.change_kind !== "create_load_case" && change.change_kind !== "create_primitive_load"))
        throw new Error("The generator returned an unsupported operation. No batch was queued.");
      return draftIntent({
        operation_kind: change.operation_kind,
        target: { object_type: change.object_type, ref: change.target_ref },
        change: {
          change_id: `change:${crypto.randomUUID()}`, change_kind: change.change_kind,
          field_label: change.field_label, field_path: change.field_path,
          before: change.before, after: change.after, unit: change.unit,
          dimension: change.dimension, source_note: change.source_note
        },
        rationale: "User-requested self-weight plan for selected pipe mass only."
      });
    })
  };
}
export function SelfWeightPlanPanel(props: WorkflowProps) {
  const [input, setInput] = useState({ caseId: "", label: "", value: "", unit: "", axis: "", provenance: "", pipes: [] as string[] });
  const [review, setReview] = useState<{
    plan: SelfWeightOperationPlan; model: WorkflowProps["model"]; inputs: typeof input;
    selectionId: string; selectionType: string; requestEpoch?: number; revision: number;
  } | null>(null);
  const state = usePreparation(props, input);
  const disabledReason = props.busy ? "Wait for the current operation to finish before changing this draft."
    : state.pending ? "Preparing the self-weight plan or its review batch. Wait for it to finish." : undefined;
  const current = review && review.revision === state.revision && review.model === props.model && review.inputs === input &&
    review.selectionId === props.selection.id && review.selectionType === props.selection.type &&
    review.requestEpoch === props.requestEpoch && !props.busy;
  async function generate() {
    setReview(null);
    await state.run(async () => {
      if (![input.caseId, input.label, input.value, input.unit, input.axis, input.provenance].every(v => v.trim()) || !input.pipes.length)
        throw new Error("Enter the case ID, label, gravity value, unit, direction, provenance and at least one pipe.");
      if (!Number.isFinite(Number(input.value))) throw new Error("Gravity requires a finite explicit number.");
      const hash = await computeModelHash(props.model);
      if (!hash) throw new Error("Model hash is unavailable.");
      const result = await generateSelfWeightPlan(props.model, {
        case_id: input.caseId, label: input.label, pipe_refs: input.pipes,
        gravity: { value: Number(input.value), unit: input.unit, axis: input.axis },
        provenance: input.provenance, source_model_hash: hash.value
      });
      if (result.status !== "ready" || !result.plan)
        throw new Error(result.diagnostics.map(d => `${d.code}: ${d.message} ${d.remediation}`).join("\n") || "The generator did not return a plan.");
      if (result.source_model_hash !== hash.value || result.plan.source_model_hash !== hash.value)
        throw new Error("The generated plan has a different source model hash.");
      return result.plan;
    }, plan => setReview({ plan, model: props.model, inputs: input,
      selectionId: props.selection.id, selectionType: props.selection.type, requestEpoch: props.requestEpoch, revision: state.revision }));
  }
  async function queue() {
    if (!current || !review) return;
    await state.run(async () => {
      const hash = await computeModelHash(props.model);
      if (!hash) throw new Error("Model hash is unavailable.");
      return selfWeightPlanBatch(review.plan, hash.value);
    }, batch => { props.onQueueBatch(batch); setReview(null); });
  }
  return <section id="self-weight-plan" tabIndex={-1} aria-label="Self-weight planning">
    <h3>Plan self-weight loads</h3>
    {disabledReason && <p id="self-weight-plan-disabled-reason" role="status">{disabledReason}</p>}
    <p>Uses selected pipe mass only. Components, supports and equipment are excluded. Missing mass inputs are reported; none are assumed.</p>
    <fieldset disabled={props.busy || state.pending} aria-describedby={disabledReason ? "self-weight-plan-disabled-reason" : undefined} title={disabledReason}><legend>Load case and gravity</legend>
      {([['caseId', 'Self-weight case ID'], ['label', 'Self-weight case label'], ['value', 'Gravity value'], ['unit', 'Gravity unit'], ['provenance', 'Self-weight provenance']] as const).map(([key, label]) =>
        <label key={key}>{label}<input value={input[key]} onChange={event => setInput({ ...input, [key]: event.target.value })} /></label>)}
      <label>Gravity direction<select value={input.axis} onChange={event => setInput({ ...input, axis: event.target.value })}>
        <option value="">Choose an axis</option>{["global_x", "global_y", "global_z"].map(axis => <option key={axis} value={axis}>{axis.replace("global_", "Global ").toUpperCase()}</option>)}
      </select></label>
      <p>Enter a signed acceleration and a compatible acceleration unit, such as m/s^2.</p>
      <fieldset><legend>Pipes to include</legend>{props.model.pipe_segments.map(pipe =>
        <label key={pipe.id}><input type="checkbox" checked={input.pipes.includes(pipe.id)} onChange={event => setInput({ ...input,
          pipes: event.target.checked ? [...input.pipes, pipe.id] : input.pipes.filter(id => id !== pipe.id) })} />{pipe.id}</label>)}</fieldset>
      <button type="button" onClick={() => void generate()}>Generate self-weight plan</button>
    </fieldset>
    {state.error && <p role="alert">{state.error}</p>}
    {current && review && <div>
      <p>{review.plan.changes.length} proposed changes for {review.plan.source_evidence.length} selected pipes.</p>
      <p>Review the source evidence, then queue the complete plan as one batch.</p>
      <pre aria-label="Self-weight source evidence">{JSON.stringify(review.plan.source_evidence, null, 2)}</pre>
      <details><summary>Proposed changes and model basis</summary><p>{review.plan.source_model_hash}</p>
        <pre>{JSON.stringify(review.plan.changes, null, 2)}</pre></details>
      <button type="button" disabled={state.pending} aria-describedby={disabledReason ? "self-weight-plan-disabled-reason" : undefined} title={disabledReason} onClick={() => void queue()}>Queue complete self-weight plan</button>
    </div>}
    {review && !current && <p role="status">This plan is stale. Generate it again against the current model and inputs.</p>}
  </section>;
}

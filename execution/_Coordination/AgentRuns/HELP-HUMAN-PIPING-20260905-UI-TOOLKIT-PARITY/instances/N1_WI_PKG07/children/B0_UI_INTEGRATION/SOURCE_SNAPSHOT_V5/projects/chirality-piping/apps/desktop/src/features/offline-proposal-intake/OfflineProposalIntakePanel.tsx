import { useState } from "react";
import type { OperationBatch } from "../../services/operationBatchService";
import { usePreparation, type WorkflowProps } from "./workflowSupport";

// Only envelope parsing happens here. The shared engine performs strict operation validation.
// Returning the original parsed object preserves all asserted author/source/rationale/flags.
export function parseOfflineBatch(text: string): OperationBatch {
  const value: unknown = JSON.parse(text);
  if (!value || typeof value !== "object" || Array.isArray(value))
    throw new Error("Provide a batch object with a batch ID and operations.");
  const batch = value as Record<string, unknown>;
  if (typeof batch.batch_id !== "string" || !batch.batch_id.trim() ||
      !Array.isArray(batch.operations) || batch.operations.length === 0)
    throw new Error("A batch needs a nonempty batch_id and at least one operation.");
  return value as OperationBatch;
}
export function OfflineProposalIntakePanel(props: WorkflowProps & { capabilityReference?: unknown }) {
  const [text, setText] = useState("");
  const state = usePreparation(props, text);
  const disabledReason = props.busy ? "Wait for the current operation to finish before editing or queueing a proposal."
    : state.pending ? "Reading the proposal file. Wait for it to finish."
    : !text.trim() ? "Paste a proposal batch or choose a proposal file before queueing it for review." : undefined;
  return <section id="offline-proposal-intake" tabIndex={-1} aria-label="Offline proposal intake">
    <h3>Review an imported proposal</h3>
    <p>Paste or open an operation batch. All changes are reviewed together before you choose to apply them.</p>
    <p>Author attribution is supplied by the file and is not verified identity. This workflow has no connected agent provider.</p>
    {disabledReason && <p id="offline-proposal-disabled-reason" role="status">{disabledReason}</p>}
    <label>Proposal file<input type="file" accept=".json,application/json" aria-describedby={disabledReason ? "offline-proposal-disabled-reason" : undefined} title={disabledReason} disabled={props.busy || state.pending}
      onChange={event => {
        const file = event.target.files?.[0];
        if (file) void state.run(() => file.text(), setText);
        event.target.value = "";
      }} /></label>
    <label>Proposal batch JSON<textarea value={text} aria-describedby={disabledReason ? "offline-proposal-disabled-reason" : undefined} title={disabledReason} disabled={props.busy || state.pending}
      onChange={event => setText(event.target.value)} rows={10} /></label>
    <button type="button" aria-describedby={disabledReason ? "offline-proposal-disabled-reason" : undefined} title={disabledReason} disabled={props.busy || state.pending || !text.trim()} onClick={() => {
      try { props.onQueueBatch(parseOfflineBatch(text)); state.setError(""); }
      catch (cause) { state.setError(cause instanceof Error ? cause.message : String(cause)); }
    }}>Queue imported batch for review</button>
    {state.error && <p role="alert">{state.error}</p>}
    <details><summary>Offline authoring reference</summary>
      <p>Keep author, source, rationale and review flags intact. The application checks the complete batch through the shared operation engine.</p>
      {props.capabilityReference === undefined ? <p>The capability reference is not available in this view.</p> : <>
        <a download="piping-operation-capabilities.json" href={`data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(props.capabilityReference, null, 2))}`}>Download capability and schema reference</a>
        <pre>{JSON.stringify(props.capabilityReference, null, 2)}</pre>
      </>}
    </details>
  </section>;
}

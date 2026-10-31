import { useState } from "react";
import type { OperationBatch } from "../../services/operationBatchService";
import { usePreparation, type WorkflowProps } from "./workflowSupport";
import { stampImportedBatch } from "../../services/operationAuthorship";

// Only envelope parsing happens here. The shared engine performs strict operation validation.
// The parsed object keeps every asserted field; queueing then stamps each operation
// agent-authored with a host source (`stampImportedBatch`), keeping the file's claimed
// author and source only as display context.
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
export function OfflineProposalIntakePanel(props: WorkflowProps & {
  capabilityReference?: unknown;
  /** On-demand executable examples (W-10), loaded when the reference is opened. */
  loadOperationExamples?: () => Promise<Record<string, unknown>>;
}) {
  const [text, setText] = useState("");
  const [examples, setExamples] = useState<{ model: unknown; value: Record<string, unknown> } | null>(null);
  const [examplesState, setExamplesState] = useState<"idle" | "loading" | "failed">("idle");
  const loadExamples = () => {
    if (!props.loadOperationExamples || examplesState === "loading" || examples?.model === props.model) return;
    const model = props.model;
    setExamplesState("loading");
    props.loadOperationExamples().then(
      (value) => { setExamples({ model, value }); setExamplesState("idle"); },
      () => setExamplesState("failed")
    );
  };
  const currentExamples = examples?.model === props.model ? examples.value : undefined;
  const reference = props.capabilityReference === undefined || currentExamples === undefined
    ? props.capabilityReference
    : { ...(props.capabilityReference as Record<string, unknown>), operation_examples: currentExamples };
  const state = usePreparation(props, text);
  const disabledReason = props.busy ? "Wait for the current operation to finish before editing or queueing a proposal."
    : state.pending ? "Reading the proposal file. Wait for it to finish."
    : !text.trim() ? "Paste a proposal batch or choose a proposal file before queueing it for review." : undefined;
  return <section id="offline-proposal-intake" tabIndex={-1} aria-label="Offline proposal intake">
    <h3>Review an imported proposal</h3>
    <p>Paste or open an operation batch. All changes are reviewed together before you choose to apply them.</p>
    <p>Imported operations are recorded as agent-proposed. The author and source the file claims are shown in review but are not verified and do not change how the operations are checked. Values only you enter (a bend flexibility factor k, SIFs and other component modifiers) are refused from an import; enter them in the component inspector. This workflow has no connected agent provider.</p>
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
      try { const imported = stampImportedBatch(parseOfflineBatch(text)); props.onQueueBatch(imported.batch, { importContext: imported.context }); state.setError(""); }
      catch (cause) { state.setError(cause instanceof Error ? cause.message : String(cause)); }
    }}>Queue imported batch for review</button>
    {state.error && <p role="alert">{state.error}</p>}
    <details onToggle={(event) => { if ((event.currentTarget as HTMLDetailsElement).open) loadExamples(); }}><summary>Offline authoring reference</summary>
      <p>Keep rationale and review flags intact. The application records every imported operation as agent-proposed and checks the complete batch through the shared operation engine.</p>
      {props.capabilityReference === undefined ? <p>The capability reference is not available in this view.</p> : <>
        {props.loadOperationExamples ? <p data-testid="operation-examples-status">{examplesState === "loading"
          ? "Checking an executable reconnect example against this model with the shared engine."
          : examplesState === "failed" ? "The executable examples could not be checked; the reference below omits them."
          : currentExamples ? "Executable examples were checked against this model by the shared engine." : "Open this reference to check executable examples against the current model."}</p> : null}
        <a download="piping-operation-capabilities.json" href={`data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(reference, null, 2))}`}>Download capability and schema reference</a>
        <pre data-testid="capability-reference-json">{JSON.stringify(reference, null, 2)}</pre>
      </>}
    </details>
  </section>;
}

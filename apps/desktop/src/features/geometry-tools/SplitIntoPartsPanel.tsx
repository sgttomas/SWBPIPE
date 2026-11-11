// B5 (DEL-07-01 Design `split-and-slope-entry.md` §2): Split into equal parts. It previews the
// plan for one pipe and queues it as one batch through the session's existing batch queue, where
// it is validated, applied whole and reverted by one Undo.
import { useEffect, useMemo, useRef, useState } from "react";
import type { OperationBatch } from "../../services/operationBatchService";
import { canonicalJsonString } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import { VirtualTargetPicker } from "../workspace/VirtualTargetPicker";
import { exactCoordinateText } from "./directionArithmetic";
import {
  MAX_PARTS,
  NUMBER_TOKEN,
  buildSplitIntoPartsBatch,
  emptySplitPartsRequest,
  engineConvertLengths,
  planSplitIntoParts,
  type ConvertLengths,
  type SplitPartsPlan,
  type SplitPartsRequest
} from "./splitIntoParts";

/** "Split into N…" from a menu: the pipe, and a serial so that each request is new. */
export type SplitPartsOpenRequest = { pipeId: string; serial: number };

type Props = {
  model: PreviewModel;
  onQueueBatch: (batch: OperationBatch) => void | Promise<void>;
  busy?: boolean;
  selectedPipeRefs?: readonly string[];
  openRequest?: SplitPartsOpenRequest | null;
  canonicalize?: (value: unknown) => Promise<string>;
  convert?: ConvertLengths;
};

const TEXT_FIELDS: Array<[keyof SplitPartsRequest, string]> = [
  ["parts", "Number of equal parts"],
  ["nodeIdPattern", "New node ID pattern"],
  ["nodeLabelPattern", "New node label pattern"],
  ["pipeIdPattern", "New pipe ID pattern"],
  ["pipeLabelPattern", "New pipe label pattern"],
  ["firstNumber", "First number"],
  ["step", "Number step"],
  ["provenance", "Provenance"]
];

export function SplitIntoPartsPanel({ model, onQueueBatch, busy = false, selectedPipeRefs = [], openRequest = null, canonicalize = canonicalJsonString, convert = engineConvertLengths }: Props) {
  const panel = useRef<HTMLElement>(null);
  const partsInput = useRef<HTMLInputElement>(null);
  const [request, setRequest] = useState<SplitPartsRequest>(() => emptySplitPartsRequest(openRequest?.pipeId ?? ""));
  const [planned, setPlanned] = useState<{ plan: SplitPartsPlan; request: SplitPartsRequest; model: PreviewModel } | null>(null);
  const [queuing, setQueuing] = useState(false);
  const [message, setMessage] = useState<{ tone: "status" | "alert"; text: string } | null>(null);
  const sequence = useRef(0);
  const latest = useRef({ model, request });
  latest.current = { model, request };

  useEffect(() => {
    if (!openRequest) return;
    setRequest((previous) => ({ ...previous, pipeId: openRequest.pipeId }));
    setMessage(null);
    (partsInput.current ?? panel.current)?.focus();
  }, [openRequest?.serial]);

  useEffect(() => {
    let live = true;
    planSplitIntoParts(model, request, convert).then((plan) => { if (live) setPlanned({ plan, request, model }); });
    return () => { live = false; };
  }, [model, request, convert]);

  const plan = planned && planned.request === request && planned.model === model ? planned.plan : null;
  const blocked = busy || queuing ? "Another operation is running." : !plan ? "Checking the split…" : plan.ok ? null : plan.reason;
  const pipeOptions = useMemo(() => model.pipe_segments.map((pipe) => ({ value: pipe.id, label: pipe.label || pipe.id, keywords: [pipe.from, pipe.to] })), [model.pipe_segments]);
  const set = (key: keyof SplitPartsRequest, value: string) => { setRequest((previous) => ({ ...previous, [key]: value })); setMessage(null); };

  async function queue() {
    if (blocked) return;
    const captured = { model, request };
    setQueuing(true);
    setMessage(null);
    try {
      const batch = await buildSplitIntoPartsBatch(model, request, canonicalize, `split-parts-${++sequence.current}`, convert);
      if (latest.current.model !== captured.model || latest.current.request !== captured.request) {
        setMessage({ tone: "alert", text: "The model or the split changed. Review the preview and queue again." });
        return;
      }
      await onQueueBatch(batch);
      setMessage({ tone: "status", text: `${batch.operations.length} operations queued as one batch for review.` });
    } catch (error) {
      setMessage({ tone: "alert", text: `Nothing was queued: ${error instanceof Error ? error.message : String(error)}` });
    } finally {
      setQueuing(false);
    }
  }

  const unit = model.project.units.length ?? "";
  return (
    <section className="panel" id="split-into-parts" ref={panel} tabIndex={-1} aria-label="Split into equal parts" data-testid="split-into-parts">
      <h3>Split into equal parts</h3>
      <p>Splits one pipe into N equal parts with N − 1 new nodes, using the existing split. The original pipe keeps its ID on the first part; each new pipe carries the source's data. Node k is placed at A + (B − A) · (k / N) in the project length unit ({unit || "not set"}), with no rounding to display precision. {NUMBER_TOKEN} in a pattern is replaced by first + (k − 1) · step.</p>
      <VirtualTargetPicker label="Pipe to split" options={pipeOptions} value={request.pipeId} onChange={(value) => set("pipeId", value)} testId="split-parts-pipe" />
      <button type="button" disabled={selectedPipeRefs.length !== 1}
        title={selectedPipeRefs.length === 1 ? "Split the selected pipe." : "Select exactly one pipe first."}
        onClick={() => set("pipeId", selectedPipeRefs[0])}>Use selected pipe</button>
      {TEXT_FIELDS.map(([key, label]) => (
        <label key={key}>{label}
          <input aria-label={label} ref={key === "parts" ? partsInput : undefined} inputMode={key === "parts" || key === "firstNumber" || key === "step" ? "numeric" : undefined}
            value={request[key]} onChange={(event) => set(key, event.target.value)} />
        </label>
      ))}
      <p>At most {MAX_PARTS} parts in one batch.</p>
      {plan?.ok ? (
        <>
          <table data-testid="split-parts-nodes">
            <caption>New nodes ({unit})</caption>
            <thead><tr><th scope="col">k</th><th scope="col">ID</th><th scope="col">Label</th><th scope="col">X</th><th scope="col">Y</th><th scope="col">Z</th></tr></thead>
            <tbody>{plan.nodes.map((node) => (
              <tr key={node.id}><td>{node.k}</td><td>{node.id}</td><td>{node.label}</td>
                <td>{exactCoordinateText(node.position.x)}</td><td>{exactCoordinateText(node.position.y)}</td><td>{exactCoordinateText(node.position.z)}</td></tr>
            ))}</tbody>
          </table>
          <table data-testid="split-parts-pieces">
            <caption>Pipes after the split</caption>
            <thead><tr><th scope="col">ID</th><th scope="col">From</th><th scope="col">To</th></tr></thead>
            <tbody>{plan.pieces.map((piece) => <tr key={piece.id}><td>{piece.id}</td><td>{piece.from}</td><td>{piece.to}</td></tr>)}</tbody>
          </table>
          <p data-testid="split-parts-summary">{plan.parts - 1} splits{plan.corrections ? ` and ${plan.corrections} exact placement${plan.corrections === 1 ? "" : "s"}, where a split rounds a coordinate differently from the stated position` : ""}.</p>
        </>
      ) : null}
      <button type="button" data-testid="split-parts-queue" disabled={Boolean(blocked)} title={blocked ?? undefined}
        aria-describedby={blocked ? "split-parts-blocked" : undefined} onClick={() => void queue()}>
        Queue split as one batch
      </button>
      {blocked ? <p id="split-parts-blocked" data-testid="split-parts-blocked">{blocked}</p> : null}
      {message ? <p role={message.tone} data-testid="split-parts-message">{message.text}</p> : null}
      <p>Validate and apply the batch in Batch review. It is applied whole or not at all, and one Undo reverts it.</p>
    </section>
  );
}

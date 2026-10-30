import { useEffect, useRef, useState } from "react";
import type { EntityRef, PreviewModel } from "../../types";
import type { OperationBatch } from "../../services/operationBatchService";

export type BatchFormProps = {
  model: PreviewModel;
  selection: EntityRef;
  onQueueBatch: (batch: OperationBatch) => void;
  busy?: boolean;
  requestEpoch?: number;
};
export function text(value: string, label: string): string {
  if (!value.trim()) throw new Error(`${label} is required.`);
  return value.trim();
}
export function number(value: string, label: string): number {
  if (!value.trim() || !Number.isFinite(Number(value))) throw new Error(`${label} requires a finite number.`);
  return Number(value);
}
export function issue(check: () => unknown): string {
  try { check(); return ""; } catch (error) { return error instanceof Error ? error.message : String(error); }
}
let nextBatch = 0;
export function batchId(kind: string): string {
  return `batch:${kind}:${Date.now()}:${++nextBatch}`;
}
/** No model publication. Every model/selection/busy/draft transition invalidates async preparation, including ABA. */
export function useBatchQueue(props: BatchFormProps, draft: unknown) {
  const [preparing, setPreparing] = useState(false);
  const [feedback, setFeedback] = useState("");
  const latest = useRef({ props, draft, epoch: 0 });
  const previous = latest.current;
  const changed = previous.props.model !== props.model || previous.props.selection.id !== props.selection.id ||
    previous.props.selection.type !== props.selection.type || previous.props.busy !== props.busy || previous.props.requestEpoch !== props.requestEpoch || previous.draft !== draft;
  latest.current = { props, draft, epoch: previous.epoch + (changed ? 1 : 0) };
  const live = useRef(false);
  const pending = useRef(false);
  useEffect(() => {
    live.current = true;
    return () => { live.current = false; latest.current.epoch++; };
  }, []);
  function cancel(message = "") {
    latest.current.epoch++;
    setFeedback(message);
  }
  async function queue(build: () => Promise<OperationBatch> | OperationBatch) {
    if (props.busy || pending.current) return;
    const epoch = latest.current.epoch;
    pending.current = true;
    setPreparing(true);
    setFeedback("");
    try {
      const batch = await build();
      if (!live.current) return;
      if (epoch !== latest.current.epoch || latest.current.props.busy) {
        setFeedback("Preparation cancelled because the model, selection or draft changed. Review and queue again.");
        return;
      }
      latest.current.props.onQueueBatch(batch);
      setFeedback("Batch queued for validation and review. No model changes applied.");
    } catch (error) {
      if (live.current && epoch === latest.current.epoch) setFeedback(error instanceof Error ? error.message : String(error));
    } finally {
      pending.current = false;
      if (live.current) setPreparing(false);
    }
  }
  return { queue, cancel, feedback, preparing: preparing || Boolean(props.busy) };
}

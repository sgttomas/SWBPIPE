// Queues a model-data intent after the asynchronous canonicalization of its
// before-value, and only while the model and pending changes it was prepared
// against are still current (the same discipline as rich authoring's queue).
import { useEffect, useRef, useState } from "react";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { canonicalJsonString } from "../../services/hashService";
import type { Canonicalize } from "./modelDataIntents";

export type ModelDataQueueProps = {
  model: PreviewModel;
  queuedIntents?: EditorOperationIntent[];
  onQueueIntent: (intent: EditorOperationIntent) => void;
  operationBusy?: boolean;
};

export function useModelDataQueue(props: ModelDataQueueProps, canonicalize: Canonicalize = canonicalJsonString) {
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const latest = useRef(props);
  latest.current = props;
  const live = useRef(false);
  useEffect(() => {
    live.current = true;
    return () => {
      live.current = false;
    };
  }, []);

  async function queue(build: (canonicalize: Canonicalize) => Promise<EditorOperationIntent>): Promise<boolean> {
    if (busy || props.operationBusy) return false;
    const captured = props;
    setBusy(true);
    setError("");
    setNotice("");
    try {
      const intent = await build(canonicalize);
      const current = latest.current;
      if (!live.current) return false;
      if (current.model !== captured.model || current.queuedIntents !== captured.queuedIntents || current.operationBusy) {
        throw new Error("The model or pending changes changed. Review this draft and queue it again.");
      }
      if (current.queuedIntents?.some((queued) =>
        queued.target.object_type === intent.target.object_type && queued.target.ref === intent.target.ref &&
        queued.change.field_path === intent.change.field_path)) {
        throw new Error("Review or remove the pending change for this record before queuing another.");
      }
      current.onQueueIntent(intent);
      setNotice(`${intent.change.field_label} queued for validation and review.`);
      return true;
    } catch (caught) {
      if (live.current) setError(caught instanceof Error ? caught.message : String(caught));
      return false;
    } finally {
      if (live.current) setBusy(false);
    }
  }

  return { busy: busy || Boolean(props.operationBusy), error, notice, setError, queue };
}

// The workspace session's history and queue events (DEL-07-02 Design
// `run-sheet-entry.md` §3.6, run sheet S2): one transient callback channel.
// A listener stages its transition from an event; staging is fallible and
// writes nothing, so an apply path stages in its Prepare phase. The returned
// publish only places prepared values (a ref and one state setter) and runs in
// the path's Publish block. Events outside a model publication are emitted:
// staged and published at once, a listener's failure contained.
import type { OperationBatchOutcome } from "../../services/operationBatchService";
import type { PreviewModel } from "../../types";

export type SessionEvent =
  | Readonly<{ type: "queued"; batchId: string; key: string; basisRevision: number; basisHash: string }>
  | Readonly<{ type: "validated"; batchId: string; key: string; outcome: OperationBatchOutcome }>
  | Readonly<{ type: "refused"; batchId: string; key: string; outcome: OperationBatchOutcome }>
  | Readonly<{ type: "checkpoint"; operationId: string; appliedModel?: PreviewModel; operationOutcomes?: OperationBatchOutcome["operation_outcomes"] }>
  | Readonly<{ type: "undone"; operationId: string }>
  | Readonly<{ type: "redone"; operationId: string }>
  | Readonly<{ type: "history-reset"; projectId: string | null }>
  | Readonly<{ type: "invalidated"; batchId: string; key: string; reason: string }>
  | Readonly<{ type: "withdrawn"; batchId: string; key: string }>
  | Readonly<{ type: "abandoned"; batchId: string; key: string; reason: string }>;

/** Stage a transition for the event; return its publish, or nothing when the event changes nothing. */
export type SessionEventListener = (event: SessionEvent) => (() => void) | void;

export type SessionEventChannel = Readonly<{
  listen(listener: SessionEventListener): () => void;
  /** Prepare: stage every listener's transition. A throw propagates and changes nothing. */
  stage(event: SessionEvent): () => void;
  /** Stage and publish at once; a listener's failure is contained and reported. */
  emit(event: SessionEvent): void;
}>;

export function createSessionEventChannel(onListenerFailure: (error: unknown, event: SessionEvent) => void = () => {}): SessionEventChannel {
  const listeners = new Set<SessionEventListener>();
  const stage = (event: SessionEvent) => {
    const publishes: Array<() => void> = [];
    for (const listener of listeners) {
      const publish = listener(event);
      if (publish) publishes.push(publish);
    }
    return () => { for (const publish of publishes) publish(); };
  };
  return {
    listen(listener) {
      listeners.add(listener);
      return () => { listeners.delete(listener); };
    },
    stage,
    emit(event) {
      try {
        stage(event)();
      } catch (error) {
        onListenerFailure(error, event);
      }
    },
  };
}

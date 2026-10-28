// Run sheet S2 (DEL-07-02 Design `run-sheet-entry.md` §3.6): the sheet's
// transient state, held beside the workspace session so it follows every
// session checkpoint whichever surface is shown. It listens on the session's
// event channel: a staged transition is the pure `applySessionEvent` of the
// event and the current state, and its publish places that value (a ref and
// one setter). Statuses are derived on every model revision from the
// session's `activeModelIndex`; the sheet never builds an index of its own.
import { useLayoutEffect, useMemo, useRef, useState } from "react";
import type { ModelIndex } from "../../workspace/modelIndex";
import type { SessionEventChannel } from "../../workspace/sessionEvents";
import { entityKey } from "../../workspace/selectionState";
import { applySessionEvent, initialRunSheetState, reconcileRunSheet, type OwnedRef, type RecordLookup, type RowReconciliation, type RunSheetState } from "./runSheetLedger";

/** One map lookup in the index's `entities` per expected id. */
export function indexLookup(index: ModelIndex | null | undefined): RecordLookup {
  return (ref: OwnedRef) => index?.entities.get(entityKey({ type: ref.type, id: ref.id }))?.record;
}

export type RunSheetStore = Readonly<{
  state: RunSheetState;
  /** The latest published state, for handlers that run between renders. */
  current: () => RunSheetState;
  statuses: ReadonlyMap<string, RowReconciliation>;
  lookup: RecordLookup;
  /** A sheet edit: a pure update of the latest state, published at once. */
  update: (change: (state: RunSheetState) => RunSheetState) => void;
}>;

export function useRunSheetStore(events: SessionEventChannel, modelIndex: ModelIndex | null | undefined, projectId: string | null): RunSheetStore {
  const [state, setState] = useState<RunSheetState>(() => initialRunSheetState(projectId));
  const latest = useRef(state);
  const publish = (next: RunSheetState) => {
    latest.current = next;
    setState(next);
  };
  useLayoutEffect(() => events.listen((event) => {
    const next = applySessionEvent(latest.current, event);
    return next === latest.current ? undefined : () => publish(next);
  }), [events]);
  // The first model of the session (no history reset precedes it) names the
  // sheet's project; any later change of project arrives as a history reset.
  useLayoutEffect(() => {
    if (projectId !== null && latest.current.projectId === null) publish({ ...latest.current, projectId });
  }, [projectId]);
  const lookup = useMemo(() => indexLookup(modelIndex), [modelIndex]);
  const statuses = useMemo(() => reconcileRunSheet(state, lookup), [state, lookup]);
  return {
    state,
    current: () => latest.current,
    statuses,
    lookup,
    update: (change) => {
      const next = change(latest.current);
      if (next !== latest.current) publish(next);
    },
  };
}

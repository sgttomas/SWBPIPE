import { useCallback, useMemo, useRef, useState } from "react";
import type { EntityRef } from "../../types";
import { emptySelection, entityRefFromKey } from "./selectionState";
import type { EntityKey, OrderedSelectionState } from "./selectionState";

/**
 * The selection cells of the workspace session: the primary selection, the
 * ordered selection and its ref, the hidden and isolate-hidden entity sets, the
 * tree's filter publication with its callback, and the selected pipe refs. It
 * declares no effect.
 *
 * Called only by the session, which is `AppSession` in `App.tsx`. The selection
 * changes only through the session's selection handlers; of the setters here,
 * only the two visibility sets (`setHiddenEntityKeys`,
 * `setIsolateHiddenEntityKeys`) are handed to components.
 */
export function useSelectionSessionState() {
  const [selection, setPrimarySelection] = useState<EntityRef | null>(null);
  const [orderedSelection, setOrderedSelection] = useState<OrderedSelectionState>(() => emptySelection());
  const orderedSelectionRef = useRef<OrderedSelectionState>(orderedSelection);
  const [hiddenEntityKeys, setHiddenEntityKeys] = useState<ReadonlySet<EntityKey>>(() => new Set());
  const [isolateHiddenEntityKeys, setIsolateHiddenEntityKeys] = useState<ReadonlySet<EntityKey>>(() => new Set());
  const [treePublication, setTreePublication] = useState<{
    actionSequence: number;
    publicationSequence: number;
    query: string;
    visibleCount: number;
    inputAt: number | null;
    inputEventTimeStamp: number | null;
    publishedAt: number;
  } | null>(null);
  const handleTreePublication = useCallback((publication: NonNullable<typeof treePublication>) => {
    setTreePublication(publication);
  }, []);
  const selectedPipeRefs = useMemo(
    () => orderedSelection.orderedKeys.flatMap((key) => {
      const ref = entityRefFromKey(key);
      return ref?.type === "pipe" ? [ref.id] : [];
    }),
    [orderedSelection.orderedKeys]
  );
  return {
    selection, setPrimarySelection,
    orderedSelection, setOrderedSelection,
    orderedSelectionRef,
    hiddenEntityKeys, setHiddenEntityKeys,
    isolateHiddenEntityKeys, setIsolateHiddenEntityKeys,
    treePublication, setTreePublication,
    handleTreePublication,
    selectedPipeRefs
  };
}

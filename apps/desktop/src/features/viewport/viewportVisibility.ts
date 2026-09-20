import type { ModelIndex } from "../workspace/modelIndex";
import type { EntityKey } from "../workspace/selectionState";
import { effectiveHiddenEntityKeys } from "./viewportSelection";

export type ViewportVisibilityProjection = Readonly<{
  hiddenKeys: ReadonlySet<EntityKey>;
  dimmedKeys: ReadonlySet<EntityKey>;
  hiddenCount: number;
  isolationActive: boolean;
}>;

/** Derives presentation only; isolation membership is literal and Hide wins. */
export function deriveViewportVisibility(
  index: ModelIndex,
  explicitHiddenKeys: ReadonlySet<EntityKey>,
  isolationSelectionKeys: ReadonlySet<EntityKey> | null
): ViewportVisibilityProjection {
  const hiddenKeys = effectiveHiddenEntityKeys(index, explicitHiddenKeys);
  const dimmedKeys = new Set<EntityKey>();
  let hiddenCount = 0;
  for (const key of index.visibilityEligibleKeys) {
    if (hiddenKeys.has(key)) hiddenCount += 1;
    else if (isolationSelectionKeys !== null && !isolationSelectionKeys.has(key)) dimmedKeys.add(key);
  }
  return Object.freeze({ hiddenKeys, dimmedKeys, hiddenCount, isolationActive: isolationSelectionKeys !== null });
}

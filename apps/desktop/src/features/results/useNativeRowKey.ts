/** C1: the internal row key a native result's table rows carry for a published result id (the
 * session's selection), found by bounded native lookup; the id itself for a full result. */
import { useEffect, useState } from "react";
import { isNativeResult, lookupNativeRows, nativeRowKey, type SessionResult } from "../../services/nativeResult";

export function useNativeRowKey(result: SessionResult | null | undefined, resultId: string | null): string | null {
  const native = !!result && isNativeResult(result);
  const [found, setFound] = useState<{ of: SessionResult; id: string; key: string | null } | null>(null);
  useEffect(() => {
    if (!native || !result || !resultId || !isNativeResult(result)) return;
    let live = true;
    lookupNativeRows(result, { ids: [resultId], match: "first" }).then(
      (lookup) => { if (live) setFound({ of: result, id: resultId, key: lookup.ids[0] ? nativeRowKey(lookup.ids[0].ordinal) : null }); },
      () => { if (live) setFound({ of: result, id: resultId, key: null }); }
    );
    return () => { live = false; };
  }, [native, result, resultId]);
  if (!native) return resultId;
  return found && found.of === result && found.id === resultId ? found.key : null;
}

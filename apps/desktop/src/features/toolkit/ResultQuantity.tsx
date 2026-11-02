import { useEffect, useState } from "react";
import { QuantityReadout } from "../display-units";
import { isNativeResult, lookupNativeRows, type SessionResult } from "../../services/nativeResult";
import type { MechanicsResult } from "../../types";

type Row = MechanicsResult["results"][number];

/** Resolve typed result rows without parsing serialized interpretation evidence. A native result's
 * row comes by one bounded native lookup by id (`results.find`). */
export function ResultQuantity({ result, id, fallback }: { result: SessionResult | null; id: string; fallback: string }) {
  const [found, setFound] = useState<{ of: SessionResult; id: string; row: Row | null } | null>(null);
  useEffect(() => {
    if (!result || !isNativeResult(result)) return;
    let live = true;
    lookupNativeRows(result, { ids: [id], match: "first" }).then(
      (lookup) => { if (live) setFound({ of: result, id, row: lookup.ids[0]?.row ?? null }); },
      () => { if (live) setFound({ of: result, id, row: null }); }
    );
    return () => { live = false; };
  }, [result, id]);
  const native = !!result && isNativeResult(result);
  if (native && !(found && found.of === result && found.id === id)) return <span>{fallback} <small>(reading the source quantity…)</small></span>;
  const row = native ? found!.row : (result as MechanicsResult | null)?.results.find((candidate) => candidate.id === id);
  if (!row) return <span>{fallback} <small>(source quantity unavailable)</small></span>;
  return <QuantityReadout quantity={{
    value: row.value,
    unit: row.unit,
    dimension_id: row.dimension ?? "unknown"
  }} />;
}

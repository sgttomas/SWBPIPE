import { QuantityReadout } from "../display-units";
import type { MechanicsResult } from "../../types";

/** Resolve typed result rows without parsing serialized interpretation evidence. */
export function ResultQuantity({ result, id, fallback }: { result: MechanicsResult | null; id: string; fallback: string }) {
  const row = result?.results.find((candidate) => candidate.id === id);
  if (!row) return <span>{fallback} <small>(source quantity unavailable)</small></span>;
  return <QuantityReadout quantity={{
    value: row.value,
    unit: row.unit,
    dimension_id: row.dimension ?? "unknown"
  }} />;
}

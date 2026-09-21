import { useEffect, useMemo, useState } from "react";
import { DISPLAY_TARGETS } from "../../display-units/targets";
import { convertDisplayQuantities, type DisplayQuantityRequest } from "../../../services/displayQuantityService";
import type { TableRow } from "./tableState";

export type QuantityColumn = Readonly<{ key: string; dimension: string }>;
export const quantityCellKey = (row: string, column: string) => JSON.stringify([row, column]);

/** A presentation-only projection. A result is usable only for its complete
 * current basis, including raw review text and typed row/family ownership. */
export function useQuantitySortProjection(rows: readonly TableRow[], columns: readonly QuantityColumn[], generation: string, policy: string) {
  const basis = JSON.stringify([generation, policy, rows.map((row) => [row.key, columns.map((column) => {
    const cell = row.cells[column.key];
    return [column.key, column.dimension, cell.value, cell.unit, cell.sortBasis, cell.convertible];
  })])]);
  const request = useMemo(() => {
    const parsed = JSON.parse(basis) as [string, string, [string, [string, string, string, string, string | null, boolean | null][]][]];
    const items: DisplayQuantityRequest[] = [];
    for (const [row, cells] of parsed[2]) for (const [key, dimension, raw, unit, , convertible] of cells) {
      const target = DISPLAY_TARGETS[dimension]?.SI;
      // Deliberately matches existing review Number parsing, including invalid
      // blank staging remaining unavailable rather than becoming zero.
      if (convertible === false || !raw.trim() || !Number.isFinite(Number(raw)) || !unit.trim() || !target) continue;
      items.push({ id: quantityCellKey(row, key), value: Number(raw), from_unit: unit, to_unit: target, dimension_id: dimension });
    }
    return items;
  }, [basis]);
  const [result, setResult] = useState<{ basis: string; values: ReadonlyMap<string, number> } | null>(null);
  useEffect(() => {
    let current = true;
    if (!request.length) { setResult({ basis, values: new Map() }); return; }
    void convertDisplayQuantities(request).then((response) => {
      if (!current) return;
      const values = new Map<string, number>();
      // Reject malformed envelopes/duplicates/unknown identities as a whole.
      if (Array.isArray(response) && response.length === request.length && new Set(response.map((item) => item?.id)).size === request.length && response.every((item) => request.some((source) => source.id === item?.id))) {
        for (const source of request) {
          const item = response.find((candidate) => candidate.id === source.id);
          if (item?.status === "converted" && item.unit === source.to_unit && Number.isFinite(item.value)) values.set(source.id, item.value);
        }
      }
      setResult({ basis, values });
    }).catch(() => { if (current) setResult({ basis, values: new Map() }); });
    return () => { current = false; };
  }, [basis, request]);
  // Compare during render, not in an effect: a new draft must immediately
  // invalidate keys from the previous source before another sort can occur.
  return result?.basis === basis ? result.values : new Map<string, number>();
}

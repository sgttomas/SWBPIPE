import { useEffect, useMemo, useState } from "react";
import { DISPLAY_TARGETS } from "../../display-units/targets";
import { convertDisplayQuantities, type DisplayQuantityRequest } from "../../../services/displayQuantityService";
import type { TableRow } from "./tableState";

export type QuantityColumn = Readonly<{ key: string; dimension: string }>;
export const quantityCellKey = (row: string, column: string) => JSON.stringify([row, column]);
type CellBasis = [string, string, string, string, string | null, boolean | null, boolean | null, string | null];
const EMPTY_VALUES: ReadonlyMap<string, number> = new Map();

/** Sort keys require the complete current request. Direct eligibility retains
 * only successful proofs for exact unchanged cells while that request refreshes. */
export function useQuantitySortProjection(rows: readonly TableRow[], columns: readonly QuantityColumn[], generation: string, policy: string) {
  const basis = JSON.stringify([generation, policy, rows.map((row) => [row.key, columns.map((column) => {
    const cell = row.cells[column.key];
    return [column.key, column.dimension, cell.value, cell.unit, cell.sortBasis, cell.convertible, cell.readonly, cell.unavailable];
  })])]);
  const plan = useMemo(() => {
    const parsed = JSON.parse(basis) as [string, string, [string, CellBasis[]][]];
    const request: DisplayQuantityRequest[] = [];
    const eligibleBases = new Map<string, string>();
    for (const [row, cells] of parsed[2]) for (const cell of cells) {
      const [key, dimension, raw, unit, , convertible, readonly, unavailable] = cell;
      const target = DISPLAY_TARGETS[dimension]?.SI;
      // Matches existing review Number parsing; blank staging is not zero.
      if (convertible === false || !raw.trim() || !Number.isFinite(Number(raw)) || !unit.trim() || !target) continue;
      const id = quantityCellKey(row, key);
      request.push({ id, value: Number(raw), from_unit: unit, to_unit: target, dimension_id: dimension });
      if (!readonly && !unavailable) eligibleBases.set(id, JSON.stringify([row, ...cell, target]));
    }
    return { request, eligibleBases, scope: JSON.stringify(parsed.slice(0, 2)) };
  }, [basis]);
  const [result, setResult] = useState<{ plan: typeof plan; values: ReadonlyMap<string, number> } | null>(null);
  const [proofs, setProofs] = useState<{ scope: string; cells: ReadonlyMap<string, string> } | null>(null);
  useEffect(() => {
    let current = true;
    setResult(null); // Do not retain old-project sort data through a pending refresh.
    // Commit-time pruning bounds memory to current cells. Render below also
    // checks exact bases immediately; no speculative render mutates a cache.
    setProofs((previous) => ({ scope: plan.scope, cells: new Map([...plan.eligibleBases].filter(([id, cellBasis]) => previous?.scope === plan.scope && previous.cells.get(id) === cellBasis)) }));
    const publish = (values: ReadonlyMap<string, number>) => {
      if (!current) return;
      setResult({ plan, values });
      // An owned settled negative/malformed result revokes proof. Pending work
      // alone does not revoke exact unchanged previously converted cells.
      setProofs({ scope: plan.scope, cells: new Map([...plan.eligibleBases].filter(([id]) => values.has(id))) });
    };
    if (!plan.request.length) { publish(EMPTY_VALUES); return; }
    void convertDisplayQuantities(plan.request).then((response) => {
      if (!current) return;
      const values = new Map<string, number>();
      // Reject malformed envelopes/duplicates/unknown identities as a whole.
      if (Array.isArray(response) && response.length === plan.request.length && new Set(response.map((item) => item?.id)).size === plan.request.length && response.every((item) => plan.request.some((source) => source.id === item?.id))) {
        for (const source of plan.request) {
          const item = response.find((candidate) => candidate.id === source.id);
          if (item?.status === "converted" && item.unit === source.to_unit && Number.isFinite(item.value)) values.set(source.id, item.value);
        }
      }
      publish(values);
    }).catch(() => publish(EMPTY_VALUES));
    return () => { current = false; };
  }, [plan]);
  return useMemo(() => ({
    // Request identity also prevents reusing an old sort when a changed basis
    // returns to earlier bytes before its new conversion has completed.
    sortValues: result?.plan === plan ? result.values : EMPTY_VALUES,
    eligibleCells: new Set([...plan.eligibleBases].filter(([id, cellBasis]) => proofs?.scope === plan.scope && proofs.cells.get(id) === cellBasis).map(([id]) => id)) as ReadonlySet<string>
  }), [plan, result, proofs]);
}

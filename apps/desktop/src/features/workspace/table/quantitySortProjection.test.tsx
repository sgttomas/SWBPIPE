import { act, renderHook, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { entityKey } from "../selectionState";
import { useQuantitySortProjection, quantityCellKey } from "./quantitySortProjection";
import { convertDisplayQuantities, type DisplayQuantityRequest, type DisplayQuantityResult } from "../../../services/displayQuantityService";
import type { TableRow } from "./tableState";
vi.mock("../../../services/displayQuantityService", () => ({ convertDisplayQuantities: vi.fn() }));
const columns = [{ key: "elastic", dimension: "stress" }, { key: "thermal", dimension: "thermal_expansion_coefficient" }];
const key = entityKey({ type: "material", id: "m" });
const rows = (raw = "2", unit = "GPa"): TableRow[] => [{ key, label: "m", cells: { elastic: { value: raw, unit }, thermal: { value: "TBD", unit: "" } } }];

describe("quantity projection ownership", () => {
  it("invalidates immediately for raw draft, unit, generation and policy changes and discards stale completions", async () => {
    const pending: Array<{ request: DisplayQuantityRequest[]; resolve: (r: DisplayQuantityResult[]) => void }> = [];
    vi.mocked(convertDisplayQuantities).mockImplementation((request) => new Promise((resolve) => pending.push({ request, resolve })));
    const view = renderHook(({ raw, unit, generation, policy }) => useQuantitySortProjection(rows(raw, unit), columns, generation, policy), { initialProps: { raw: "2", unit: "GPa", generation: "p1", policy: "materials:review" } });
    expect(pending[0].request).toEqual([{ id: quantityCellKey(key, "elastic"), value: 2, from_unit: "GPa", to_unit: "Pa", dimension_id: "stress" }]);
    const resolve = async (index: number, value: number) => act(async () => pending[index].resolve([{ id: quantityCellKey(key, "elastic"), status: "converted", value, unit: "Pa" }]));
    await resolve(0, 2e9); expect(view.result.current.sortValues.get(quantityCellKey(key, "elastic"))).toBe(2e9);
    view.rerender({ raw: "2.0", unit: "GPa", generation: "p1", policy: "materials:review" }); expect(view.result.current.sortValues.size).toBe(0);
    view.rerender({ raw: "3", unit: "MPa", generation: "p2", policy: "materials:direct" });
    await resolve(1, 2e9); expect(view.result.current.sortValues.size).toBe(0);
    await resolve(2, 3e6); expect(view.result.current.sortValues.get(quantityCellKey(key, "elastic"))).toBe(3e6);
    expect(view.result.current.sortValues.has(quantityCellKey(key, "thermal"))).toBe(false);
    view.rerender({ raw: " ", unit: "MPa", generation: "p2", policy: "materials:direct" }); expect(view.result.current.sortValues.size).toBe(0);
    expect(pending).toHaveLength(3);
  });
  it.each(["wrong-unit", "nonfinite", "duplicate", "missing", "unknown"])("rejects %s projection responses", async (kind) => {
    vi.mocked(convertDisplayQuantities).mockImplementation(async (request) => {
      const converted: DisplayQuantityResult = { id: request[0].id, status: "converted", value: kind === "nonfinite" ? Infinity : 2, unit: kind === "wrong-unit" ? "psi" : "Pa" };
      return kind === "missing" ? [] : kind === "duplicate" ? [converted, converted] : kind === "unknown" ? [{ ...converted, id: "unknown" }] : [converted];
    });
    const view = renderHook(() => useQuantitySortProjection(rows(), columns, "p", "materials:review"));
    await waitFor(() => expect(convertDisplayQuantities).toHaveBeenCalled()); await act(async () => {});
    expect(view.result.current.sortValues.size).toBe(0);
  });
});

function directRows(value: unknown = 2, unit = "MPa", availability: { readonly?: boolean; unavailable?: string } = {}): TableRow[] {
  const cell = (v: unknown) => ({ value: String(v), unit, sortBasis: JSON.stringify([typeof v, Object.is(v, -0) ? "-0" : String(v), unit, null]), convertible: typeof v === "number" && Number.isFinite(v), ...availability });
  return [{ key, label: "m", cells: { elastic: cell(value), shear: cell(3) } }];
}
const directColumns = [{ key: "elastic", dimension: "stress" }, { key: "shear", dimension: "stress" }];
function deferredConversions() {
  const pending: Array<{ request: DisplayQuantityRequest[]; resolve: (r: DisplayQuantityResult[]) => void; reject: (reason: Error) => void }> = [];
  vi.mocked(convertDisplayQuantities).mockImplementation((request) => new Promise((resolve, reject) => pending.push({ request, resolve, reject })));
  const finish = async (index: number, response?: DisplayQuantityResult[]) => act(async () => pending[index].resolve(response ?? pending[index].request.map((item) => ({ id: item.id, status: "converted" as const, value: item.value, unit: item.to_unit }))));
  return { pending, finish };
}

describe("exact current-cell conversion eligibility", () => {
  it("retains unchanged proof during refresh without exposing any old sort keys or stale response proof", async () => {
    const { pending, finish } = deferredConversions();
    const view = renderHook(({ rows }) => useQuantitySortProjection(rows, directColumns, "p1", "materials:direct"), { initialProps: { rows: directRows() } });
    await finish(0); expect(view.result.current.eligibleCells.size).toBe(2); expect(view.result.current.sortValues.size).toBe(2);
    view.rerender({ rows: directRows(4) });
    expect(view.result.current.eligibleCells.has(quantityCellKey(key, "elastic"))).toBe(false);
    expect(view.result.current.eligibleCells.has(quantityCellKey(key, "shear"))).toBe(true); expect(view.result.current.sortValues.size).toBe(0);
    view.rerender({ rows: directRows(5) }); await finish(1);
    expect(view.result.current.eligibleCells.has(quantityCellKey(key, "elastic"))).toBe(false);
    expect(view.result.current.eligibleCells.has(quantityCellKey(key, "shear"))).toBe(true); expect(view.result.current.sortValues.size).toBe(0);
    await finish(2); expect(view.result.current.eligibleCells.size).toBe(2); expect(view.result.current.sortValues.size).toBe(2);
    view.rerender({ rows: directRows(6) }); expect(view.result.current.eligibleCells.has(quantityCellKey(key, "shear"))).toBe(true);
    await finish(3, pending[3].request.map((item) => item.id === quantityCellKey(key, "shear") ? { id: item.id, status: "unavailable", message: "Current converter refuses this quantity" } : { id: item.id, status: "converted", value: item.value, unit: item.to_unit }));
    expect(view.result.current.eligibleCells.has(quantityCellKey(key, "shear"))).toBe(false);
    expect(view.result.current.eligibleCells.has(quantityCellKey(key, "elastic"))).toBe(true);
  });

  it.each(["value", "type", "unit", "dimension", "row-type", "row-id", "field", "generation", "family", "policy", "missing-unit", "readonly", "unavailable", "removed"])("revokes %s changes immediately and does not revive a pruned old basis", async (change) => {
    const { pending, finish } = deferredConversions();
    const initial = { rows: directRows(), columns: directColumns, generation: "p1", policy: "materials:direct" };
    const view = renderHook(({ rows, columns, generation, policy }) => useQuantitySortProjection(rows, columns, generation, policy), { initialProps: initial });
    await finish(0); expect(view.result.current.eligibleCells.has(quantityCellKey(key, "elastic"))).toBe(true);
    const changed = { ...initial };
    if (change === "value") changed.rows = directRows(4);
    if (change === "type") changed.rows = directRows("2");
    if (change === "unit") changed.rows = directRows(2, "psi");
    if (change === "dimension") changed.columns = [{ key: "elastic", dimension: "pressure" }, directColumns[1]];
    if (change === "row-type" || change === "row-id") changed.rows = [{ ...initial.rows[0], key: entityKey({ type: change === "row-type" ? "node" : "material", id: change === "row-id" ? "other" : "m" }) }];
    if (change === "field") { changed.rows = [{ ...initial.rows[0], cells: { other: initial.rows[0].cells.elastic, shear: initial.rows[0].cells.shear } }]; changed.columns = [{ key: "other", dimension: "stress" }, directColumns[1]]; }
    if (change === "generation") changed.generation = "p2";
    if (change === "family") changed.policy = "sections:direct";
    if (change === "policy") changed.policy = "materials:review";
    if (change === "missing-unit") changed.rows = directRows(2, "");
    if (change === "readonly") changed.rows = directRows(2, "MPa", { readonly: true });
    if (change === "unavailable") changed.rows = directRows(2, "MPa", { unavailable: "Malformed current field" });
    if (change === "removed") changed.rows = [];
    view.rerender(changed);
    expect(view.result.current.eligibleCells.has(quantityCellKey(changed.rows[0]?.key ?? key, change === "field" ? "other" : "elastic"))).toBe(false);
    expect(view.result.current.sortValues.size).toBe(0);
    view.rerender(initial);
    expect(view.result.current.eligibleCells.has(quantityCellKey(key, "elastic"))).toBe(false); expect(view.result.current.sortValues.size).toBe(0);
    await finish(pending.length - 1);
    expect(view.result.current.eligibleCells.has(quantityCellKey(key, "elastic"))).toBe(true);
  });

  it.each(["malformed", "rejected"])("revokes old proof after an owned settled %s response, not merely pending work", async (kind) => {
    const { pending, finish } = deferredConversions();
    const view = renderHook(({ rows }) => useQuantitySortProjection(rows, directColumns, "p1", "materials:direct"), { initialProps: { rows: directRows() } });
    await finish(0); view.rerender({ rows: directRows(4) });
    expect(view.result.current.eligibleCells.has(quantityCellKey(key, "shear"))).toBe(true);
    if (kind === "malformed") await finish(1, []);
    else await act(async () => pending[1].reject(new Error("conversion unavailable")));
    expect(view.result.current.eligibleCells.size).toBe(0); expect(view.result.current.sortValues.size).toBe(0);
  });
});

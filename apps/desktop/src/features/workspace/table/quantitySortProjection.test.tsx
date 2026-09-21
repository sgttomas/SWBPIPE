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
    await resolve(0, 2e9); expect(view.result.current.get(quantityCellKey(key, "elastic"))).toBe(2e9);
    view.rerender({ raw: "2.0", unit: "GPa", generation: "p1", policy: "materials:review" }); expect(view.result.current.size).toBe(0);
    view.rerender({ raw: "3", unit: "MPa", generation: "p2", policy: "materials:direct" });
    await resolve(1, 2e9); expect(view.result.current.size).toBe(0);
    await resolve(2, 3e6); expect(view.result.current.get(quantityCellKey(key, "elastic"))).toBe(3e6);
    expect(view.result.current.has(quantityCellKey(key, "thermal"))).toBe(false);
    view.rerender({ raw: " ", unit: "MPa", generation: "p2", policy: "materials:direct" }); expect(view.result.current.size).toBe(0);
    expect(pending).toHaveLength(3);
  });
  it.each(["wrong-unit", "nonfinite", "duplicate", "missing", "unknown"])("rejects %s projection responses", async (kind) => {
    vi.mocked(convertDisplayQuantities).mockImplementation(async (request) => {
      const converted: DisplayQuantityResult = { id: request[0].id, status: "converted", value: kind === "nonfinite" ? Infinity : 2, unit: kind === "wrong-unit" ? "psi" : "Pa" };
      return kind === "missing" ? [] : kind === "duplicate" ? [converted, converted] : kind === "unknown" ? [{ ...converted, id: "unknown" }] : [converted];
    });
    const view = renderHook(() => useQuantitySortProjection(rows(), columns, "p", "materials:review"));
    await waitFor(() => expect(convertDisplayQuantities).toHaveBeenCalled()); await act(async () => {});
    expect(view.result.current.size).toBe(0);
  });
});

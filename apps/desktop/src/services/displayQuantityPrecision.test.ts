import { describe, expect, it } from "vitest";
import vectors from "../../../../fixtures/results/precision_transport_v0_3.json";
import { convertDisplayQuantities } from "./displayQuantityService";

// The real WASM engine is loaded by the maintained test setup. No converter mock.
describe("same-unit precision through the actual WASM display bridge", () => {
  it("preserves every admitted vector and normalized temperature alias", async () => {
    const values = [...vectors.vectors.map(v => Number(v.decimal)), 1e-16, -1e-16, 0.2677777778417276];
    for (const [unit, target, dimension] of [["degC", "degC", "temperature"], ["C", "degC", "temperature"], ["MPa", "MPa", "stress"]]) {
      const request = values.map((value, i) => ({ id: String(i), value, from_unit: unit, to_unit: target, dimension_id: dimension }));
      const before = JSON.stringify(request);
      const results = await convertDisplayQuantities(request);
      expect(results).toHaveLength(values.length);
      results.forEach((result, i) => {
        expect(result.status).toBe("converted");
        if (result.status === "converted") { expect(Object.is(result.value, values[i])).toBe(true); expect(result.unit).toBe(target); }
      });
      expect(JSON.stringify(request)).toBe(before);
    }
  });
  it("rejects unknown or dimension-incompatible same-symbol requests", async () => {
    const pressureReference = { id: "reference", value: 1, from_unit: "bar", to_unit: "bar", dimension_id: "pressure", reference: "gauge" };
    const results = await convertDisplayQuantities([
      { id: "unknown", value: 1, from_unit: "invalid", to_unit: "invalid", dimension_id: "temperature" },
      { id: "wrong", value: 1, from_unit: "C", to_unit: "C", dimension_id: "force" },
      { id: "nonfinite", value: Number.POSITIVE_INFINITY, from_unit: "MPa", to_unit: "MPa", dimension_id: "stress" },
      pressureReference
    ]);
    expect(results.every(result => result.status === "unavailable")).toBe(true);
  });
});

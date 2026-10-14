import { describe, expect, it } from "vitest";
import { expandRetainedPrecisionDefinition, readRetainedPrecisionCorpus } from "./retainedPrecisionCorpus";

describe("compact retained corpus storage", () => {
  it("preserves repeated values with independent containers and data keys", () => {
    const definition = { format: "retained-precision-corpus-dag/v1", layout: "compact",
      nodes: ["__proto__", -0, [1, 0, 1], [0, 2, 2]], root: 3 };
    const value = expandRetainedPrecisionDefinition(definition);
    expect(Object.prototype.hasOwnProperty.call(value[0], "__proto__")).toBe(true);
    expect(Object.is(value[0].__proto__, -0)).toBe(true);
    value[0].__proto__ = 7;
    expect(Object.is(value[1].__proto__, -0)).toBe(true);
  });

  it("refuses obsolete and cyclic or malformed definitions", () => {
    expect(() => expandRetainedPrecisionDefinition({ cases: [] })).toThrow();
    expect(() => expandRetainedPrecisionDefinition({ format: "retained-precision-corpus-dag/v1",
      layout: "compact", nodes: [[0, 0]], root: 0 })).toThrow();
    expect(() => expandRetainedPrecisionDefinition({ format: "retained-precision-corpus-dag/v1",
      layout: "compact", nodes: ["same", 1, [1, 0, 1, 0, 1]], root: 2 })).toThrow();
  });

  it("rejects negative zero as a structural tag or reference", () => {
    const definition = { format: "retained-precision-corpus-dag/v1", layout: "compact",
      nodes: [17, [0, 0]], root: 1 };
    expect(() => expandRetainedPrecisionDefinition({ ...definition, root: -0 })).toThrow();
    expect(() => expandRetainedPrecisionDefinition({ ...definition, nodes: [17, [0, -0]] })).toThrow();
    expect(() => expandRetainedPrecisionDefinition({ ...definition, nodes: [17, [-0, 0]] })).toThrow();
  });

  it("retains the full shared case sets and isolates separate consumers", () => {
    const first = readRetainedPrecisionCorpus();
    expect([first.cases.length, first.mutations.length, first.must_pass.length]).toEqual([45, 593, 91]);
    const id = first.cases[0].id;
    first.cases[0].id = "changed-by-this-test";
    expect(readRetainedPrecisionCorpus().cases[0].id).toBe(id);
  });
});

// The combination editor's operand rules on an invented model.
import { describe, expect, it } from "vitest";
import type { PreviewModel } from "../../types";
import { combinationOperandRefusal, moveTerm, resolveReference, termPayload, termsDraftProblem } from "./combinationAuthoring";

function model(): PreviewModel {
  return {
    load_cases: [{ id: "A" }, { id: "B" }, { id: "S" }],
    supports: [],
    combinations: [
      // S is both a load case and a combination: a bare "S" names the load case.
      { id: "S", label: "shadowed", basis: "mechanics", terms: [{ load_case: "A", factor: 1 }], provenance: "t" },
      { id: "D", label: "d", basis: "result_state_subtraction", minuend_id: "E", subtrahend_id: "A", provenance: "t" },
      { id: "E", label: "e", basis: "range_envelope", operand_ids: ["F", "S"], mode: "max", provenance: "t" },
      { id: "F", label: "f", basis: "srss", terms: [{ load_case: "B", factor: 1 }], provenance: "t" }
    ]
  } as unknown as PreviewModel;
}

describe("combination operand rules", () => {
  it("resolves a bare reference to the load case first, then the combination", () => {
    expect(resolveReference(model(), "S")).toEqual({ id: "S", kind: "load_case", shadowsCombination: true });
    expect(resolveReference(model(), "E")).toEqual({ id: "E", kind: "combination", shadowsCombination: false });
    expect(resolveReference(model(), "Z").kind).toBe("unknown");
  });

  it("follows bare references that resolve to combinations, and not those that resolve to load cases, when finding a cycle", () => {
    // D names E (minuend), E names F (operand); its "S" resolves to the load case.
    expect(combinationOperandRefusal(model(), "F", "D")).toContain("D → E → F → D");
    expect(combinationOperandRefusal(model(), "S", "E")).toBeNull();
    expect(combinationOperandRefusal(model(), "F", "F")).toContain("self-reference");
  });

  it("checks a term list and writes each term in its exact shape", () => {
    expect(termsDraftProblem(model(), [], "N")).toBe("add at least one term");
    expect(termsDraftProblem(model(), [{ kind: "load_case", ref: "S", factor: "1" }, { kind: "combination", ref: "S", factor: "2" }], "N")).toBeNull();
    expect(termsDraftProblem(model(), [{ kind: "combination", ref: "A", factor: "1" }], "N")).toBe("term 1: A is not a combination of this model");
    expect(termsDraftProblem(model(), [{ kind: "load_case", ref: "A", factor: "1e" }], "N")).toBe("term 1: enter a finite factor");
    expect(JSON.stringify(termPayload({ kind: "load_case", ref: "A", factor: "-0.5" }))).toBe('{"load_case":"A","factor":-0.5}');
    expect(JSON.stringify(termPayload({ kind: "combination", ref: "F", factor: "2" }))).toBe('{"combination":"F","factor":2}');
  });

  it("moves a term within the list and leaves out-of-range moves unchanged", () => {
    expect(moveTerm(["a", "b", "c"], 2, -1)).toEqual(["a", "c", "b"]);
    expect(moveTerm(["a", "b", "c"], 0, -1)).toEqual(["a", "b", "c"]);
    expect(moveTerm(["a", "b", "c"], 2, 1)).toEqual(["a", "b", "c"]);
  });
});

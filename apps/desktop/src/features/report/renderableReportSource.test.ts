/** DEL-08-01 REQ-017 and REQ-024: the rendered report's source (`renderableReportSource`).
 * It is Current with the numerical-eligibility requirement separated out, and it
 * requires solved mechanics and a readable standing in its place (Agent 0 and the
 * INTERLOCUTOR on PR 328's text round). Every other Current requirement removes it
 * exactly as it removes Current. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { STANDING_FINDINGS, renderableReportSource } from "./renderableReportSource";

const root = resolve(__dirname, "../../../../../");
const result = { id: "invented-result" };
const base = {
  current: null,
  result,
  otherCurrentRequirementsHold: true,
  boundRoute: true,
  mechanicsSolved: true,
  standing: { findings: ["NUMERICAL_INTEGRITY_NOT_QUALIFIED", "NUMERICAL_CASE_EVIDENCE_INCOMPLETE"] }
};

describe("the rendered report's source", () => {
  it("holds the closed standing-finding vocabulary equal to the generated catalogue", () => {
    const catalogue = JSON.parse(readFileSync(resolve(root, "fixtures/reports/numerical_vocabulary.json"), "utf8")) as { tags: { vocabulary: string; tag: string }[] };
    expect([...STANDING_FINDINGS].sort()).toEqual(catalogue.tags.filter((t) => t.vocabulary === "standing_finding").map((t) => t.tag).sort());
  });

  it("is Current whenever the result is eligible, whatever the other inputs say", () => {
    const current = { id: "invented-current" };
    for (const other of [base, { ...base, boundRoute: false }, { ...base, mechanicsSolved: false }, { ...base, standing: null }]) {
      expect(renderableReportSource({ ...other, current })).toBe(current);
    }
  });

  it("admits a solved, readable result of a bound route (exact or pressure-free Timoshenko) that is not eligible", () => {
    expect(renderableReportSource(base)).toBe(result);
    expect(renderableReportSource({ ...base, standing: { findings: [] } })).toBe(result);
  });

  it("removes it for each other Current requirement, the route, unsolved mechanics, an unreadable standing and no standing", () => {
    for (const [name, inputs] of [
      ["another Current requirement fails", { ...base, otherCurrentRequirementsHold: false }],
      ["not a bound route's result", { ...base, boundRoute: false }],
      // An unsolved result with a standing object present (the reader's own unsolved finding beside it).
      ["unsolved mechanics", { ...base, mechanicsSolved: false, standing: { findings: ["MECHANICS_NOT_SOLVED"] } }],
      ["unsolved, standing findings only", { ...base, mechanicsSolved: false }],
      // An unreadable result with a standing object present: a route refusal's finding.
      ["an unreadable standing", { ...base, standing: { findings: ["NUMERICAL_INTEGRITY_NOT_QUALIFIED", "SOURCE_PHYSICS_NONLINEAR_STANDING"] } }],
      ["an ambiguous id", { ...base, standing: { findings: ["NUMERICAL_EVIDENCE_ID_AMBIGUOUS"] } }],
      ["no standing", { ...base, standing: null }],
      ["no result", { ...base, result: null }]
    ] as const) {
      expect(renderableReportSource(inputs as typeof base), name).toBeNull();
    }
  });
});

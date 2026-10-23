// DEL-07-05 numerical health §3 "Replacing 'Needs recompute'": the chip and the rail
// show the health header (rail: short form) only with the session's verdict-only hold;
// every other case keeps "Needs recompute". The hold itself is `numericalHealthHold`
// (numericalHealth.test.ts covers its generic, pressure-source and retained cases).
import { describe, expect, it } from "vitest";
import { railStageState, runPresenceFromCells, statusChipInputsFromCells, statusChips, statusChipText } from "./shellLayout";

const solved = { status: { mechanics: "MECHANICS_SOLVED", professional_acceptance: "NOT_PROVIDED" } };
const hold = {
  headline: "Numerical health: 1 sensitive, 1 checks passed",
  shortHeadline: "Numerical health: sensitive",
  outputSentence: "No results of this run can be exported or used by rule checks while 1 load case(s) are not checks passed."
};
const cells = (extra: Record<string, unknown>) => ({
  model: { analysis_status: { mechanics: "MECHANICS_SOLVED" } },
  result: solved,
  historicalRun: null,
  solveJob: { state: "completed" as const },
  ruleCheckAggregate: null,
  ...extra
});

describe("shell chip and rail under a numerical-health hold", () => {
  it("shows the header on the chip and the short form on the rail and Review reason", () => {
    const run = runPresenceFromCells({ result: solved, historicalRun: null, solveJob: { state: "completed" }, hasQualifiedCurrentResult: false, numericalHealthHold: hold });
    expect(run.needsRecompute).toBe(true);
    expect(railStageState("results", run).caption).toBe("Numerical health: sensitive");
    expect(railStageState("review", run)).toMatchObject({ enabled: false, reason: "Numerical health: sensitive" });
    const chips = statusChips(statusChipInputsFromCells(cells({ hasQualifiedCurrentResult: false, numericalHealthHold: hold })), "results");
    expect(chips).toHaveLength(1);
    expect(chips[0]).toMatchObject({ token: "needs_recompute", label: "Numerical health: 1 sensitive, 1 checks passed", source: "Derived numerical qualification",
      numericalHealth: { headline: hold.headline, outputSentence: hold.outputSentence } });
    expect(statusChipText(chips[0])).toBe("Solver · Numerical health: 1 sensitive, 1 checks passed");
  });

  it("keeps Needs recompute without the hold (missing evidence, another route, or any other unmet requirement)", () => {
    for (const absent of [undefined, null]) {
      const run = runPresenceFromCells({ result: solved, historicalRun: null, solveJob: { state: "completed" }, hasQualifiedCurrentResult: false, numericalHealthHold: absent });
      expect(railStageState("results", run).caption).toBe("Needs recompute");
      expect(railStageState("review", run).reason).toBe("Needs recompute");
      const [chip] = statusChips(statusChipInputsFromCells(cells({ hasQualifiedCurrentResult: false, numericalHealthHold: absent })), "results");
      expect(chip.label).toBe("Needs recompute");
      expect(chip.numericalHealth).toBeUndefined();
    }
  });

  it("changes nothing for a Current result or a reopened run", () => {
    const current = runPresenceFromCells({ result: solved, historicalRun: null, solveJob: { state: "completed" }, hasQualifiedCurrentResult: true, numericalHealthHold: hold });
    expect(current.needsRecompute).toBeUndefined();
    expect(current.numericalHealthHold).toBeUndefined();
    expect(railStageState("results", current).caption).toBeNull();
    expect(railStageState("review", current).enabled).toBe(true);
    const historical = runPresenceFromCells({ result: solved, historicalRun: {}, solveJob: { state: "completed" }, hasQualifiedCurrentResult: false, numericalHealthHold: hold });
    expect(railStageState("results", historical).caption).toBe("Historical");
    expect(statusChips(statusChipInputsFromCells(cells({ historicalRun: {}, hasQualifiedCurrentResult: false, numericalHealthHold: hold })), "results")).toEqual([]);
  });
});

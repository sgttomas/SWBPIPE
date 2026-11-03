// RT-U4: the solve timeline's exact code table is what its sources produce, entry for entry, and
// its lower-case codes and local reasons are the ones the recording sites use.
import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { extractSolveTimelineCodes, renderTable, TABLE_FILE } from "../../scripts/solveTimelineCodes.mjs";
import { BACKEND_JOB_STATES, PREVIEW_SOLVER_MODES } from "./previewService";
import { LOCAL_REASON_CODES, READER_VERDICT_CODES } from "./solveTimelineCodes";
import { LOWER_CODES } from "./solveTimeline";

describe("the solve timeline's code table", () => {
  it("is generated from its sources: regenerate with `node apps/desktop/scripts/solveTimelineCodes.mjs --write`", () => {
    expect(readFileSync(TABLE_FILE, "utf8")).toBe(renderTable(extractSolveTimelineCodes()));
  });

  it("lists the U4b readers' route-prefixed codes on both routes (u4b_evidence.rs `route.fail`/`route.require`)", () => {
    for (const code of ["SOURCE_PHYSICS_SHEAR_OPERAND_AMBIGUOUS", "SOURCE_PREVIEW_PHYSICS_SHEAR_OPERAND_AMBIGUOUS"]) {
      expect(READER_VERDICT_CODES).toContain(code);
    }
    // Not the row kinds, tokens or producer diagnostics u4b_evidence.rs reads.
    for (const code of ["TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN", "SOURCE_PHYSICS_TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN",
      "SOURCE_PREVIEW_PHYSICS_MECHANICS_SOLVED", "SOURCE_PREVIEW_PHYSICS_ARC_TOROIDAL_HOOP_DOMAIN"]) {
      expect(READER_VERDICT_CODES).not.toContain(code);
    }
  });

  it("lists exactly the solver modes and job states as its lower-case codes", () => {
    expect([...LOWER_CODES].sort()).toEqual([...PREVIEW_SOLVER_MODES, ...BACKEND_JOB_STATES].sort());
  });

  it("lists every reason the registration step records", () => {
    const source = readFileSync(path.join(path.dirname(TABLE_FILE), "previewService.ts"), "utf8");
    const start = source.indexOf("async function validateCapturedSource");
    const body = source.slice(start, source.indexOf("\n}\n", start));
    const recorded = [...body.matchAll(/(?:return|\?\?)\s+"([A-Z_]+)"/g)].map((match) => match[1]);
    expect(recorded.length).toBeGreaterThanOrEqual(4);
    expect(recorded.filter((code) => !LOCAL_REASON_CODES.includes(code))).toEqual([]);
  });
});

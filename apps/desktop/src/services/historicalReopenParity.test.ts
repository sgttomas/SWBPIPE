/** C1 slice 6: the native reopen's oracle. `historical_reopen_cases.json` equals its generator
 * (`test-support/historicalReopenCases.ts`): every case's findings are the unchanged TypeScript
 * contexts' (`buildHistoricalRunContext`, `buildRecordedRunContext`) over the full rows. The
 * native leg (`stored_custody_tests.rs`) must give the same findings, in order, from the stored
 * texts. HISTORICAL_REOPEN_CASES_WRITE=1 rewrites the file. */
import { describe, expect, it } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { HISTORICAL_REOPEN_CASES, historicalReopenCases, reopenCasesText } from "../test-support/historicalReopenCases";

const ROOT = resolve(__dirname, "../../../../");
const WRITE = process.env.HISTORICAL_REOPEN_CASES_WRITE === "1";

describe("the native reopen's oracle", () => {
  it("the committed cases are the TypeScript contexts' findings", async () => {
    const file = await historicalReopenCases();
    const text = reopenCasesText(file);
    const path = resolve(ROOT, HISTORICAL_REOPEN_CASES);
    if (WRITE) writeFileSync(path, text);
    const committed = readFileSync(path, "utf8");
    // Small enough to commit: clean runs refer to the builder corpus.
    expect(text.length).toBeLessThan(8 * 1024 * 1024);
    const lines = (t: string) => t.split("\n");
    expect(lines(committed).length).toBe(lines(text).length);
    lines(text).forEach((line, index) => expect(lines(committed)[index] === line, `line ${index + 1}: ${line.slice(0, 120)}`).toBe(true));
    const saved = file.cases.filter(c => c.kind === "saved");
    const recorded = file.cases.filter(c => c.kind === "recorded");
    expect(saved.length).toBeGreaterThanOrEqual(80);
    expect(recorded.length).toBeGreaterThanOrEqual(10);
    // The corpus reaches the validator's refusals, the hash findings and clean runs alike.
    const findings = new Set(file.cases.flatMap(c => "findings" in c.expected ? c.expected.findings : []));
    for (const expected of ["HISTORICAL_INPUT_MANIFEST_MISSING", "HISTORICAL_RESULT_HASH_MISMATCH", "HISTORICAL_ANALYSIS_HASH_MISMATCH", "ANALYSIS_RUN_SCHEMA_VERSION_RETIRED",
      "HISTORICAL_ANALYSIS_SCHEMA_VERSION_UNSUPPORTED", "HISTORICAL_MECHANICS_EVIDENCE_MALFORMED", "HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE", "HISTORICAL_MODEL_HASH_MISMATCH",
      "HISTORICAL_ENVELOPE_HASH_MISMATCH", "ANALYSIS_ROW_SOURCE_BINDING_MISMATCH", "ANALYSIS_ROW_INTERPRETATION_MISMATCH", "ANALYSIS_SOURCE_STATUS_MISMATCH",
      "RECORDED_RUN_MODEL_STATE_HASH_MISMATCH", "RECORDED_RUN_RECORD_HASH_MISMATCH", "HISTORICAL_ANALYSIS_EVIDENCE_INVALID"]) {
      expect(findings.has(expected), expected).toBe(true);
    }
  }, 900_000);
});

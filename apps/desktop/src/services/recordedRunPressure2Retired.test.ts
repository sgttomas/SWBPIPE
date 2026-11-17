// RT-L2 (DEL-04-07): the ordinary identity moved to pressure-3,
// and the pressure-2 recorded-table bases (model documents 0.8.0 and 0.9.0, RT-A3 reducers 2a and
// 2b, increments 3 and 4, RT-A4 and the combination certificate) are not carried into it
// (`analysisRunCompatibility.ts` RECORDED_RUN_TABLE_BASES; Python's twin is `compatibility.py`).
// Each pressure-2 run recorded under them, main's records verbatim (fixtures/persistence/
// recorded_model_*, outside `repin`'s reach), follows the retired-identity rule: its record still
// verifies against its own checksum, and both its source and its record are refused by name with the
// message that the result predates the pressure elongation setting and must be re-solved.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { recordedRunSemanticBinding, validateAnalysisRunV03, verifyAnalysisRunRecord } from "./analysisRunCompatibility";
import { readerDispatch } from "../features/results/resultReader";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Json = any;
const RECORDED: ReadonlyArray<readonly [string, string]> = [
  ["recorded_model_0_7_0", "3e2aac3c"], ["recorded_model_0_8_0", "5451fbf3"],
  ["recorded_model_0_9_0_pre_2a", "d46e1963"], ["recorded_model_0_9_0_pre_2b", "032e861f"],
  ["recorded_model_0_9_0_pre_ce", "a2db16a5"], ["recorded_model_0_9_0_pre_rt_a4", "be29fabc"],
  ["recorded_model_0_9_0_pre_nl", "10455bf3"], ["recorded_model_0_9_0_pre_cert", "2c20583d"]
];
const PRESSURE_2 = "openpipestress.result_semantics/0.3.0/pressure-2";
const PRESSURE_3 = "openpipestress.result_semantics/0.3.0/pressure-3";

describe.each(RECORDED)("a pressure-2 run recorded under an earlier table (%s)", (dir, prefix) => {
  const read = (name: string): Json => JSON.parse(readFileSync(resolve(__dirname, `../../../../fixtures/persistence/${dir}`, name), "utf8"));
  const record = () => read("pressure_plain_connected_sparse.analysis_run.json");
  const source = () => read("pressure_connected_mechanics_sparse.json");

  it("is main's record verbatim: its checksum matches and it binds a pressure-2 table", async () => {
    const run = record();
    expect(await verifyAnalysisRunRecord(run)).toBe("match");
    expect(run.analysis_run.reproducibility.semantic_contract.id).toBe(PRESSURE_2);
    expect(run.analysis_run.reproducibility.semantic_contract.sha256.startsWith(prefix)).toBe(true);
    expect(source().producer.semantic_contract_id).toBe(PRESSURE_2);
  });

  it("is no recorded basis of the current table", () => {
    const current = { id: PRESSURE_3, sha256: "f".repeat(64) };
    expect(recordedRunSemanticBinding(record().analysis_run.reproducibility.semantic_contract, current)).toEqual(current);
  });

  it("is refused by name: its source at Rust's header dispatch, its record with the same code and message", async () => {
    const dispatch = readerDispatch(source());
    expect(dispatch.ok).toBe(false);
    expect(dispatch.error?.code).toBe("SOURCE_PRESSURE_2_RETIRED");
    // The message tells the user the result predates the pressure-elongation setting and must be re-solved.
    expect(dispatch.error?.detail).toBe(
      "SOURCE_PRESSURE_2_RETIRED: pressure-2 results predate the pressure elongation setting and are no longer read; re-solve the model");
    const run = record();
    await expect(validateAnalysisRunV03(run, source(), run.analysis_run.load_basis_refs)).rejects.toMatchObject({ code: "SOURCE_PRESSURE_2_RETIRED", message: dispatch.error?.detail });
    await expect(validateAnalysisRunV03(run, source(), run.analysis_run.load_basis_refs, undefined, { recordedRead: true })).rejects.toMatchObject({ code: "SOURCE_PRESSURE_2_RETIRED", message: dispatch.error?.detail });
  });
});

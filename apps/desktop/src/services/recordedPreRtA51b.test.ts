// Operating-case slice 1b (DEL-05-02 Work `operating-case.md` "As built (1b)", recorded runs): 1b
// restated the preview-physics combination policy (v2 -> v3), so the preview-physics-2 digest moved
// (e7b93f41 -> f41b9d3e) while its rows and limitations did not. Main's records at bbf384dec
// (fixtures/persistence/recorded_pre_rt_a5_1b_pf, verbatim) bind e7b93f41: they read on the
// recorded read under that registry variant, the current record validator (which binds the current
// table only) refuses the historical binding, and reading changes no byte.
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import type { AnalysisRunEnvelope, MechanicsResult } from "../types";
import { validateAnalysisRunV03, verifyAnalysisRunRecord } from "./analysisRunCompatibility";
import { recordedReaderDispatch, recordedReaderValidate } from "../features/results/resultReader";
import { numericalResultStanding, recordedResultStanding } from "../features/results/resultStanding";
import { validateRetainedPrecisionRecorded } from "../features/results/retainedPrecisionReader";
import { recordedPreRtA4Binding, recordedPreRtA4Table } from "./recordedPreRtA4";

const root = resolve(__dirname, "../../../../");
const fixture = (path: string) => readFileSync(resolve(root, "fixtures/persistence/recorded_pre_rt_a5_1b_pf", path), "utf8");
const PREVIEW = "openpipestress.result_semantics/0.3.0/preview-physics-2";
const MAIN_SHA = "e7b93f416f700f8efdb31bf1ba26dedde52963d768d6ead5a73dbf1c645a360c";
const RECORDED = { recordedRead: true } as const;

describe("a run main recorded before operating-case slice 1b", () => {
  it.each(["transient_run_input.json", "transient_run_input_rule_packs.json"])("%s reads on the recorded read under main's table only", async name => {
    const record = (JSON.parse(fixture(`preview_physics_2/${name}`)) as { run_record: AnalysisRunEnvelope }).run_record;
    const source = JSON.parse(fixture("preview_physics_2/preview_physics_connected_sparse.json")) as MechanicsResult;
    const before = JSON.stringify([record, source]);
    const binding = { id: PREVIEW, sha256: MAIN_SHA };
    expect(record.analysis_run.reproducibility.semantic_contract).toEqual(binding);
    expect(recordedPreRtA4Binding(binding)).toEqual(binding);
    const current = createHash("sha256").update(readFileSync(resolve(root, "fixtures/results/semantic_contract_v0_3_preview_physics_2.json"))).digest("hex");
    expect(current).not.toBe(MAIN_SHA);
    expect((await recordedPreRtA4Table(binding)).semantic_contract_id).toBe(PREVIEW);
    expect(await verifyAnalysisRunRecord(record)).toBe("match");
    await validateAnalysisRunV03(record, source, record.analysis_run.load_basis_refs, undefined, RECORDED);
    await expect(validateAnalysisRunV03(record, source, record.analysis_run.load_basis_refs)).rejects.toThrow();
    expect(recordedReaderValidate(source, binding)).toEqual({});
    expect(recordedReaderDispatch(source, binding).binding).toEqual(binding);
    // Rows and limitations are the current table's: the recorded standing is the one the statement reads with now.
    expect(recordedResultStanding(source, binding).findings).toEqual(numericalResultStanding(source).findings);
    expect(JSON.stringify([record, source])).toBe(before);
  });

  it("main's successor statement reads under main's retained table", async () => {
    const successor = JSON.parse(fixture("preview_physics_retained_1/retained_precision_milestone_successor_sparse_interactive.json")) as { source: MechanicsResult };
    const binding = { id: "openpipestress.result_semantics/0.3.0/preview-physics-retained-1", sha256: "26dce9c21261b1ca894af05a3b6f4a94db88904894cb47f7296f892e72ecfb7e" };
    expect((await validateRetainedPrecisionRecorded(successor.source, binding)).publication_sha256).toBeTruthy();
  });
});

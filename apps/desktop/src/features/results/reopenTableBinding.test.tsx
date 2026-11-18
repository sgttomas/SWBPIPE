// The reopen's one table-binding decision (DEL-05-02, DEL-14-02): a run that main's own record
// builder recorded under a revision of the pressure-3 table this version does not hold
// (`fixtures/persistence/recorded_table_revised`, README there) reopens with its rows withheld on
// the saved-run path and on the recorded-run path; the same statement with a record of the
// installed table keeps its rows. Native twins: `stored_custody_tests.rs`
// (`a_run_recorded_under_a_revised_table_reopens_withheld_and_resaves_as_stored`) and the parity
// cases `saved-revised-table:*` and `revised-table:*`.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { render, screen, cleanup } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { buildHistoricalRunContext, buildRecordedRunContext, HistoricalRunPanel, WithheldRunNotice } from "./HistoricalRunContext";
import { recordedRunSide } from "../comparison/runComparisonSides";
import { COMPARISON_RUN_RECORD_MISMATCH, RunComparisonRefusal } from "../comparison/runComparison";
import { runDescription } from "../../services/analysisRunService";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { REVISED_TABLE_FIXTURE, revisedTableWitness } from "../../test-support/historicalReopenCases";
import { UNRECOGNISED_TABLE_REFUSAL_TEXT } from "../../test-support/tableBindingRefusals";
import type { LocalProjectEnvelope, MechanicsResult, PreviewModel } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const ROOT = resolve(__dirname, "../../../../../");
const fixture = (name: string): Json => JSON.parse(readFileSync(resolve(ROOT, REVISED_TABLE_FIXTURE, name), "utf8"));
const INSTALLED_PRESSURE_3 = { id: "openpipestress.result_semantics/0.3.0/pressure-3", sha256: "757e5b18e97b5f5c5dcca235a682abfb25768d6af6c70bc81cb10813998200d9" };
const WITHHELD_MESSAGE = "The results of this run are not shown: this run names a revision of these result rules that this version does not recognise; the stored run is unchanged; run the analysis again on its saved model state to create a new result.";

afterEach(cleanup);

describe("a run recorded under a revision of its table this version does not hold", () => {
  it("is main's own record of main's statement, binding pressure-3 under another digest", () => {
    const record = fixture("pressure_plain_connected_sparse.analysis_run.json");
    const binding = record.analysis_run.reproducibility.semantic_contract;
    expect(binding.id).toBe(INSTALLED_PRESSURE_3.id);
    expect(binding.sha256).not.toBe(INSTALLED_PRESSURE_3.sha256);
    expect(record.analysis_run.result_refs.every((ref: Json) => ref.semantic_contract.sha256 === binding.sha256)).toBe(true);
  });

  it("W1/W2/W6/W7, saved-run path: identity and stored values kept, rows, exports and detail withheld", async () => {
    const { opened } = await revisedTableWitness();
    const before = JSON.stringify(opened);
    const context = (await buildHistoricalRunContext(opened))!;
    expect(context.binding).toStrictEqual({ withheld: "unrecognised_digest" });
    expect(context.findings).toContain(UNRECOGNISED_TABLE_REFUSAL_TEXT);
    // Its integrity holds: no hash finding; the refusal is the binding's.
    expect(context.findings.filter((finding) => finding.includes("HASH"))).toStrictEqual([]);
    expect(context.mechanicsResult).toBeNull();
    expect(context.recordedTableRead).toBeNull();
    // W7: the values a re-save writes are the opened ones, untouched.
    expect(context.rawMechanicsResult).toBe(opened.mechanics_result);
    expect(context.rawAnalysisRun).toBe(opened.analysis_run);
    expect(context.runId).toBe((opened.mechanics_result as MechanicsResult).run_id);
    render(<HistoricalRunPanel context={context} />);
    expect(screen.getByTestId("historical-run-withheld-message").textContent).toBe(WITHHELD_MESSAGE);
    expect(screen.getByTestId("historical-run-withheld-code").textContent).toBe("ANALYSIS_SEMANTIC_CONTRACT_MISMATCH");
    expect(screen.getByText(new RegExp(`Saved evidence for ${context.runId}`))).toBeInTheDocument();
    // W6: no row table, no row detail, no interpreted CSV export.
    expect(screen.queryAllByRole("table")).toHaveLength(0);
    expect(document.querySelectorAll("[data-csv-export]")).toHaveLength(0);
    expect(screen.queryAllByTestId(/export-csv/)).toHaveLength(0);
    expect(JSON.stringify(opened)).toBe(before);
  });

  it("W2/W6, recorded-run path: Check record says so, and the run is not compared", async () => {
    const { read } = await revisedTableWitness();
    const context = await buildRecordedRunContext(structuredClone(read));
    expect(context.binding).toStrictEqual({ withheld: "unrecognised_digest" });
    expect(context.findings).toContain(UNRECOGNISED_TABLE_REFUSAL_TEXT);
    expect(context.mechanicsResult).toBeNull();
    render(<WithheldRunNotice binding={context.binding} testId="check-withheld" />);
    expect(screen.getByTestId("check-withheld-message").textContent).toBe(WITHHELD_MESSAGE);
    const refusal = await recordedRunSide(structuredClone(read)).then(() => null, (error: RunComparisonRefusal) => error);
    expect(refusal).toBeInstanceOf(RunComparisonRefusal);
    expect(refusal!.code).toBe(COMPARISON_RUN_RECORD_MISMATCH);
    expect(refusal!.message).toBe(`${COMPARISON_RUN_RECORD_MISMATCH}: the results of the recorded run ${runDescription(read.run)} are not shown, so it is not compared: this run names a revision of these result rules that this version does not recognise; the stored run is unchanged; run the analysis again on its saved model state to create a new result.`);
  });

  it("W3/W4: the same statement with main's record of the installed table keeps its rows; without a record they are withheld", async () => {
    const source = fixture("pressure_connected_mechanics_sparse.json") as MechanicsResult;
    const model = fixture("exact_pressure_connected_request.json").model as PreviewModel;
    // Main's committed record of this statement under the installed table: it differs from the
    // witness record only in its manifest reference and its table digest.
    const analysis_run = JSON.parse(readFileSync(resolve(ROOT, "fixtures/results/pressure_plain_connected_sparse.analysis_run.json"), "utf8"));
    expect(analysis_run.analysis_run.reproducibility.semantic_contract).toStrictEqual(INSTALLED_PRESSURE_3);
    const model_hash = await computeModelHash(model);
    const envelope = async (record: Json) => {
      const payload = { model, mechanics_result: source, analysis_run: record, model_hash, editor_intents: [], proposal: null, selected_review_target: null };
      return { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload) } as unknown as LocalProjectEnvelope;
    };
    const bound = (await buildHistoricalRunContext(await envelope(analysis_run)))!;
    expect(bound.binding).toStrictEqual({ read: "installed" });
    expect(bound.mechanicsResult).toBe(source);
    render(<HistoricalRunPanel context={bound} />);
    expect(screen.queryByTestId("historical-run-withheld")).toBeNull();
    expect(screen.queryAllByRole("table").length).toBeGreaterThan(0);
    cleanup();
    const unbound = (await buildHistoricalRunContext(await envelope(null)))!;
    expect(unbound.binding).toStrictEqual({ withheld: "unrecorded" });
    expect(unbound.mechanicsResult).toBeNull();
    expect(unbound.rawMechanicsResult).toBe(source);
    render(<HistoricalRunPanel context={unbound} />);
    expect(screen.getByTestId("historical-run-withheld-code").textContent).toBe("ANALYSIS_SEMANTIC_CONTRACT_UNRECORDED");
    expect(screen.queryAllByRole("table")).toHaveLength(0);
    expect(document.querySelectorAll("[data-csv-export]")).toHaveLength(0);
  });
});

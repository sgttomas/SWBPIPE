// W-12 S2 (DEL-14-01 Design §3): `HistoricalRunContext` accepts the real state ref of a
// recorded run, verified against the stored state it was read with, and keeps flagging
// every other mismatch. The recorded run is the native store's committed witness of a
// real producer result (fixtures/persistence/recorded_run_witness.json).
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import type { AnalysisRunRead } from "../../services/analysisRunService";
import type { ModelStateSummary } from "../../services/modelStateService";
import type { AnalysisRunEnvelope, LocalProjectEnvelope } from "../../types";
import { buildHistoricalRunContext, buildRecordedRunContext, recordedRunBindingFindings } from "./HistoricalRunContext";
import { analysisRecordProjection } from "../../services/analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";

const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const input = JSON.parse(text("fixtures/persistence/transient_run_input.json")) as { run_record: AnalysisRunEnvelope; input_manifest_canonical: string };
const witness = JSON.parse(text("fixtures/persistence/recorded_run_witness.json")) as {
  state_id: string; state_model_hash: string; created_at: string; recording_id: string; run_record_hash: string; recorded_record: AnalysisRunEnvelope;
};
const resultText = text("fixtures/results/preview_physics_connected_sparse.json");
const projectId = JSON.parse(input.input_manifest_canonical).model_basis.model_ref as string;

function read(): AnalysisRunRead {
  const state = {
    project_id: projectId, state_id: witness.state_id, state_kind: "post_solve_snapshot", created_at: witness.created_at, parent_state_id: null,
    model_hash: witness.state_model_hash, record_hash: "r", labels_updated_at: witness.created_at, child_state_ids: [], recorded_run_ids: [witness.recording_id],
    analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "After solve", tags: [], notes: [], assumptions: []
  } as ModelStateSummary;
  return {
    run: {
      project_id: projectId, recording_id: witness.recording_id, state_id: witness.state_id, producer_run_id: witness.recorded_record.analysis_run.run_id,
      created_at: witness.created_at, run_record_hash: witness.run_record_hash, result_hash: "r", labels_updated_at: witness.created_at,
      analysis_status: witness.recorded_record.analysis_run.analysis_status, run_name: "Witness run", tags: [], notes: [], assumptions: []
    },
    state,
    record: structuredClone(witness.recorded_record),
    result_json: resultText,
    input_manifest_canonical: input.input_manifest_canonical
  };
}

describe("a recorded run's context", () => {
  it("accepts the real state ref of a recorded run bound to its stored state", async () => {
    const context = await buildRecordedRunContext(read());
    expect(context.designation).toBe("recorded_run");
    expect(context.findings).not.toContain("HISTORICAL_INPUT_MANIFEST_MISSING");
    expect(recordedRunBindingFindings(context)).toEqual([]);
    expect(context.modelHash?.value).toBe(witness.state_model_hash);
    expect(context.runId).toBe(witness.recorded_record.analysis_run.run_id);
  });

  it("flags a state ref that is not the stored state's, and every other mismatch", async () => {
    const otherState = read();
    otherState.state = { ...otherState.state, state_id: "state:other" };
    expect(recordedRunBindingFindings(await buildRecordedRunContext(otherState))).toEqual(expect.arrayContaining(["HISTORICAL_MODEL_STATE_REF_MISMATCH", "ANALYSIS_SOURCE_MODEL_STATE_MISMATCH"]));

    const otherModel = read();
    otherModel.state = { ...otherModel.state, model_hash: `sha256:${"0".repeat(64)}` };
    expect(recordedRunBindingFindings(await buildRecordedRunContext(otherModel))).toEqual(expect.arrayContaining(["RECORDED_RUN_MODEL_STATE_HASH_MISMATCH", "ANALYSIS_RECORDED_MODEL_STATE_HASH_MISMATCH"]));

    const otherManifest = read();
    const manifest = JSON.parse(otherManifest.input_manifest_canonical);
    manifest.solver_basis.solver_mode = "dense_scrutiny";
    otherManifest.input_manifest_canonical = JSON.stringify(manifest);
    expect(recordedRunBindingFindings(await buildRecordedRunContext(otherManifest))).toEqual(expect.arrayContaining(["RECORDED_RUN_INPUT_MANIFEST_HASH_MISMATCH", "ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH"]));

    const otherResult = read();
    const result = JSON.parse(otherResult.result_json);
    result.results[0].value += 1;
    otherResult.result_json = JSON.stringify(result);
    expect(recordedRunBindingFindings(await buildRecordedRunContext(otherResult))).toContain("HISTORICAL_RESULT_HASH_MISMATCH");

    const otherHash = read();
    otherHash.run = { ...otherHash.run, run_record_hash: "0".repeat(64) };
    expect(recordedRunBindingFindings(await buildRecordedRunContext(otherHash))).toContain("RECORDED_RUN_RECORD_HASH_MISMATCH");

    const transient = read();
    transient.record = structuredClone(input.run_record);
    expect(recordedRunBindingFindings(await buildRecordedRunContext(transient))).toContain("HISTORICAL_MODEL_STATE_REF_MISMATCH");
  });

  it("flags a coherently resealed record that contradicts its stored manifest (Codex r1's mutation)", async () => {
    const forged = read();
    const run = forged.record.analysis_run;
    run.settings_ref = { object_type: "SolverSettings", ref: "solver-settings:other" };
    run.solver_version!.build_ref = { object_type: "ExternalReference", ref: "other-solver@9" };
    run.load_basis_refs.push({ object_type: "LoadCase", ref: "LC-ABSENT" });
    const claim = run.hashes.find((hash) => hash.payload_scope === "analysis_run_record")!;
    claim.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(forged.record));
    forged.run = { ...forged.run, run_record_hash: claim.value };
    const findings = recordedRunBindingFindings(await buildRecordedRunContext(forged));
    expect(findings).not.toEqual([]);
    expect(findings).toContain("ANALYSIS_RECORDED_SOLVER_MISMATCH");
  });

  it("a saved project row still flags any ref other than the synthetic one", async () => {
    const model = JSON.parse(input.input_manifest_canonical).model_basis.model_payload;
    const opened = { model, mechanics_result: JSON.parse(resultText), analysis_run: structuredClone(witness.recorded_record), model_hash: null, project_envelope_hash: null } as unknown as LocalProjectEnvelope;
    const context = await buildHistoricalRunContext(opened);
    expect(context?.findings).toContain("HISTORICAL_MODEL_STATE_REF_MISMATCH");
    expect(context?.findings).toContain("HISTORICAL_INPUT_MANIFEST_MISSING");
  });
});

// Model document 0.9.0 (DEL-02-01 Design model-document-0.9.0.md §3.2; owner ruling
// "0.8.0 a2", applied to every retired version): a run recorded before the 0.9.0 ripple
// keeps its existing read and compare behaviour. The record, its source and its model are
// main's at 5bb2f9ff0, verbatim (fixtures/persistence/recorded_model_0_8_0): an ordinary
// pressure-2 run of model 0.8.0 that binds the 0.8.0-era pressure-2 table (5451fbf3…).
// The digest is matched by its prefix here, and the fixture is outside `repin`'s reach
// (tools/serialization/semantic_contract_tables.py EXCLUDED), so a later table step cannot
// rewrite the recorded basis.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { manifestFixedMembers, recordedAnalysisRunRevision, validateAnalysisRunV03, verifyAnalysisRunRecord } from "./analysisRunCompatibility";
import { buildCurrentSessionInputManifest } from "./inputManifestService";
import { computeModelHash } from "./hashService";
import { buildRecordedRunContext, recordedRunBindingFindings } from "../features/results/HistoricalRunContext";
import { recordedRunSide } from "../features/comparison/runComparisonSides";

const DIR = resolve(__dirname, "../../../../fixtures/persistence/recorded_model_0_8_0");
const read = (name: string) => readFileSync(resolve(DIR, name), "utf8");
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Json = any;
const record = (): Json => JSON.parse(read("pressure_plain_connected_sparse.analysis_run.json"));
const source = (): Json => JSON.parse(read("pressure_connected_mechanics_sparse.json"));
const RECORDED_TABLE_PREFIX = "5451fbf3";

async function recordedRead(edit: (record: Json) => void = () => undefined) {
  const run = record(), result = source();
  const model = JSON.parse(read("exact_pressure_connected_request.json")).model;
  expect(model.schema_version).toBe("0.8.0");
  for (const c of model.load_cases) { c.kind ||= "sustained"; c.status ||= "ready_for_preview_diagnostics"; c.provenance ||= "invented review input"; }
  const manifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: result.producer.component_name, solver_version: result.producer.component_version, solver_build_ref: run.analysis_run.solver_version.build_ref.ref, solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
  const stateId = `state:${"c".repeat(32)}`, createdAt = "2026-10-25T10:00:00.000Z";
  for (const { member, expected } of await manifestFixedMembers(manifest.manifest)) {
    if (member === "input_manifest_refs" || member === "input_manifest_hashes") run.analysis_run.reproducibility[member] = expected; else run.analysis_run[member] = expected;
  }
  edit(run);
  const recorded = await recordedAnalysisRunRevision(run, { stateId, createdAt, inputManifest: manifest.manifest });
  const hash = (recorded.analysis_run.hashes as Json[]).find((h: Json) => h.payload_scope === "analysis_run_record").value;
  const state: Json = { project_id: model.project.id, state_id: stateId, state_kind: "post_solve_snapshot", created_at: createdAt, parent_state_id: null, model_hash: (await computeModelHash(model))!.value, record_hash: "r", labels_updated_at: createdAt, child_state_ids: [], recorded_run_count: 1, analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "Recorded at 0.8.0", tags: [], notes: [], assumptions: [] };
  return { state, record: recorded, result_json: read("pressure_connected_mechanics_sparse.json"), input_manifest_canonical: manifest.canonical_bytes,
    run: { project_id: model.project.id, recording_id: `runrec:${hash.slice(0, 32)}`, state_id: stateId, producer_run_id: result.run_id, created_at: createdAt, run_record_hash: hash, result_hash: "r", labels_updated_at: createdAt, analysis_status: recorded.analysis_run.analysis_status, run_name: "Recorded at 0.8.0", tags: [], notes: [], assumptions: [] } } as Json;
}

describe("a pressure-2 run recorded before model document 0.9.0 (a2)", () => {
  it("is main's record verbatim: its checksum matches and it binds the 0.8.0-era table", async () => {
    const run = record();
    expect(await verifyAnalysisRunRecord(run)).toBe("match");
    expect(run.analysis_run.reproducibility.semantic_contract.id).toBe("openpipestress.result_semantics/0.3.0/pressure-2");
    expect(run.analysis_run.reproducibility.semantic_contract.sha256.startsWith(RECORDED_TABLE_PREFIX)).toBe(true);
  });
  it("reads as before", async () => {
    const run = record();
    await expect(validateAnalysisRunV03(run, source(), run.analysis_run.load_basis_refs)).resolves.toBeUndefined();
  });
  it("displays with no finding and is offered for comparison as a recorded revision", async () => {
    const recorded = await recordedRead();
    expect(await verifyAnalysisRunRecord(recorded.record)).toBe("match");
    expect(recordedRunBindingFindings(await buildRecordedRunContext(recorded))).toEqual([]);
    await expect(recordedRunSide(recorded)).resolves.toMatchObject({ kind: "recorded" });
  });
  it("is not a warrant for any other table digest: a changed recorded binding is still refused", async () => {
    const run = record();
    run.analysis_run.reproducibility.semantic_contract.sha256 = "0".repeat(64);
    await expect(validateAnalysisRunV03(run, source(), run.analysis_run.load_basis_refs)).rejects.toThrow("ANALYSIS_SEMANTIC_CONTRACT_MISMATCH");
    const mixed = record();
    mixed.analysis_run.result_refs[0].semantic_contract.sha256 = "1".repeat(64);
    await expect(validateAnalysisRunV03(mixed, source(), mixed.analysis_run.load_basis_refs)).rejects.toThrow("ANALYSIS_ROW_SOURCE_BINDING_MISMATCH");
  });
});

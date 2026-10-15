/** The desktop AnalysisRun record of a load/reference-state result (T1 WP2), on the
 * current exact identities (B7b: the v3 twins of the retired load-reference-1 and
 * load-reference-source-1). For both, the desktop record mirrors Python
 * `build_analysis_run_v0_3` (DEL-14-02). Proof: the desktop record of each committed raw
 * equals the committed Python carrier by canonical hash after normalizing only the three
 * route-independent fields in which the two builders already differ for every identity
 * (provenance labels, determinism notes, and therefore the record self-hash), and the
 * desktop validator reads Python's carrier.
 * - Joined, pressure-source-1 (fixtures/results/pressure_source_load_state_*.analysis_run.json):
 *   a source record carrying contract_evidence and source_block_recovery.
 * - Ordinary, pressure-1 (fixtures/results/pressure_load_state_*.analysis_run.json): a record
 *   carrying neither, its result refs binding pressure-1's current table. */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../types";
import { buildAnalysisRunV03, modelLoadBasisRefs, validateAnalysisRunV03, verifyAnalysisRunRecord } from "./analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "./hashService";
import { receivedFixture } from "../test-support/receivedText";

const root = resolve(__dirname, "../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const LR = ["connected-sparse", "connected-dense", "pressure-sparse", "pressure-dense"];
const LRS = ["n05", "n06", "fields", "mixed", "eigen_motion"].flatMap(name => ["sparse", "dense"].map(mode => `${name}-${mode}`));
const MODE: Record<string, string> = { sparse: "sparse_interactive", dense: "dense_scrutiny" };
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const FAMILY = {
  ordinary: { folder: "pressure/load_reference", carrier: "pressure_load_state" },
  joined: { folder: "pressure_source/load_reference_source", carrier: "pressure_source_load_state" },
} as const;

function normalize(record: Json): Json {
  const copy = structuredClone(record);
  const run = copy.analysis_run;
  delete run.provenance;
  delete run.reproducibility.determinism_notes;
  run.hashes = run.hashes.filter((h: Json) => h.payload_scope !== "analysis_run_record");
  for (const ref of run.result_refs) delete ref.provenance;
  return copy;
}

function inputs(kind: keyof typeof FAMILY, name: string) {
  const [base, mode] = [name.slice(0, name.lastIndexOf("-")), name.slice(name.lastIndexOf("-") + 1)];
  const { folder, carrier } = FAMILY[kind];
  const raw = receivedFixture<MechanicsResult>(`fixtures/product_preview/${folder}/${base}-${MODE[mode]}.raw.json`);
  const model = json(`fixtures/product_preview/${folder}/${base}.request.json`).model as PreviewModel;
  const python = json(`fixtures/results/${carrier}_${name.replace("-", "_")}.analysis_run.json`) as AnalysisRunEnvelope;
  const ref = python.analysis_run.reproducibility.input_manifest_refs[0];
  const manifest = { manifest_ref: ref, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: raw.model_ref }, solver_basis: { solver_name: raw.producer!.component_name, solver_version: raw.producer!.component_version, solver_build_ref: "unavailable:solver-build-not-supplied" } } };
  return { raw, model, python, manifest };
}
async function mirror(name: string, kind: keyof typeof FAMILY = "joined") {
  const { raw, model, python, manifest } = inputs(kind, name);
  const desktop = await buildAnalysisRunV03(raw, manifest, undefined, modelLoadBasisRefs(model));
  return { raw, model, python, desktop };
}
const PRESSURE_1 = { id: "openpipestress.result_semantics/0.3.0/pressure-1", sha256: "a4e3bf2cfb7e0b3fa84a564cde03545cefafda903c34f4dd6a526fa0be982375" };

describe("joined (pressure-source-1) AnalysisRun mirror", () => {
  it.each(LRS)("%s equals the Python carrier by canonical hash (route-independent fields normalized)", async name => {
    const { raw, model, python, desktop } = await mirror(name);
    expect(await canonicalSha256HexCheckedV1(normalize(desktop))).toBe(await canonicalSha256HexCheckedV1(normalize(python)));
    const run = desktop.analysis_run;
    expect(run.contract_evidence).toEqual(raw.contract_evidence);
    expect(run.source_block_recovery).toEqual(raw.source_block_recovery);
    expect(await verifyAnalysisRunRecord(desktop)).toBe("match");
    await expect(validateAnalysisRunV03(desktop, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
    await expect(validateAnalysisRunV03(python, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
  });
});

describe("ordinary (pressure-1) AnalysisRun mirror", () => {
  it.each(LR)("%s equals the Python carrier by canonical hash (route-independent fields normalized); each validator reads the other's record", async name => {
    const { raw, model, python, desktop } = await mirror(name, "ordinary");
    expect(python.analysis_run.run_id).toBe(raw.run_id);
    expect(await canonicalSha256HexCheckedV1(normalize(desktop))).toBe(await canonicalSha256HexCheckedV1(normalize(python)));
    const run = desktop.analysis_run;
    // Neither namespace is embedded: pressure-1 is not a source record (Python's SOURCE_RECORD_CONTRACT_IDS).
    expect(Object.hasOwn(run, "contract_evidence")).toBe(false);
    expect(Object.hasOwn(run, "source_block_recovery")).toBe(false);
    expect(Object.hasOwn(run, "retained_precision")).toBe(false);
    // The record and every result ref bind pressure-1's current table.
    expect(run.reproducibility.semantic_contract).toStrictEqual(PRESSURE_1);
    expect(run.result_refs).toHaveLength(raw.results.length);
    for (const ref of run.result_refs) expect({ id: ref.semantic_contract?.id, sha256: ref.semantic_contract?.sha256 }).toStrictEqual(PRESSURE_1);
    expect(await verifyAnalysisRunRecord(desktop)).toBe("match");
    await expect(validateAnalysisRunV03(desktop, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
    await expect(validateAnalysisRunV03(python, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
  });
});

describe("records bound to other evidence are refused", () => {
  it.each([
    ["ordinary carrying contract_evidence", (r: Json, raw: Json) => { r.analysis_run.contract_evidence = structuredClone(raw.contract_evidence); }, "ANALYSIS_PHYSICS_SOURCE_DOWNGRADE_FORBIDDEN"],
    ["ordinary carrying a source receipt", (r: Json) => { r.analysis_run.source_block_recovery = {}; }, "SOURCE_RECOVERY_METADATA_CONTRADICTION"],
    ["ordinary bound to pressure-source-1's table", (r: Json) => { r.analysis_run.reproducibility.semantic_contract = { id: "openpipestress.result_semantics/0.3.0/pressure-source-1", sha256: "dec4f4e7af6f60cf5615facc1b1ff11c8d5df5b3907fc28d922d661672383435" }; }, "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"],
    ["ordinary with another row binding", (r: Json) => { r.analysis_run.result_refs[0].semantic_contract.sha256 = "0".repeat(64); }, "ANALYSIS_ROW_SOURCE_BINDING_MISMATCH"],
  ])("%s", async (_name, edit, code) => {
    const { raw, model, python } = await mirror("pressure-sparse", "ordinary");
    const forged = structuredClone(python); edit(forged, raw);
    await expect(validateAnalysisRunV03(forged, raw, modelLoadBasisRefs(model))).rejects.toThrow(code);
  });
  it("a tampered raw ordinary result is refused by the reader before a record is built", async () => {
    const { raw, model, manifest } = inputs("ordinary", "connected-sparse");
    const tampered = structuredClone(raw) as Json; tampered.contract_evidence.load_reference_states[0].members[0].fit_strain = 0.5;
    await expect(buildAnalysisRunV03(tampered, manifest, undefined, modelLoadBasisRefs(model))).rejects.toThrow("SOURCE_LOAD_REFERENCE_STRETCH_BINDING");
  });
  it.each([
    ["joined without its retained evidence", (r: Json) => { delete r.analysis_run.contract_evidence; }, "ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH"],
    ["joined without load_reference_states", (r: Json) => { delete r.analysis_run.contract_evidence.load_reference_states; }, "ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH"],
    ["joined without its receipt", (r: Json) => { delete r.analysis_run.source_block_recovery; }, "ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH"],
    ["joined with another table hash", (r: Json) => { r.analysis_run.reproducibility.semantic_contract.sha256 = "0".repeat(64); }, "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"],
  ])("%s", async (_name, edit, code) => {
    const { raw, model, desktop } = await mirror("mixed-sparse");
    const forged = structuredClone(desktop); edit(forged);
    await expect(validateAnalysisRunV03(forged, raw, modelLoadBasisRefs(model))).rejects.toThrow(code);
  });
  it("a tampered raw joined result is refused by the reader before a record is built", async () => {
    const { raw, model, manifest } = inputs("joined", "mixed-sparse");
    const tampered = structuredClone(raw) as Json; tampered.contract_evidence.load_reference_states[0].members[0].fit_strain = 0.5;
    await expect(buildAnalysisRunV03(tampered, manifest, undefined, modelLoadBasisRefs(model))).rejects.toThrow("SOURCE_LOAD_REFERENCE_STRETCH_BINDING");
  });
});

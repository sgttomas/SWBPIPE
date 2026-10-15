/** The desktop AnalysisRun record of a load/reference-state result (T1 WP2), on the
 * current exact identities (B7b: the v3 twins of the retired load-reference-1 and
 * load-reference-source-1).
 * - Joined, pressure-source-1: the desktop record mirrors Python
 *   `build_analysis_run_v0_3`. Proof: the desktop record of each committed raw equals the
 *   committed Python carrier (fixtures/results/pressure_source_load_state_*.analysis_run.json)
 *   by canonical hash after normalizing only the three route-independent fields in which
 *   the two builders already differ for every identity (provenance labels, determinism
 *   notes, and therefore the record self-hash).
 * - Ordinary, pressure-1: the desktop builds and reads no record
 *   (ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH), while Python's product path now writes one
 *   (fixtures/results/pressure_load_state_*.analysis_run.json). */
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
async function mirror(name: string) {
  const { raw, model, python, manifest } = inputs("joined", name);
  const desktop = await buildAnalysisRunV03(raw, manifest, undefined, modelLoadBasisRefs(model));
  return { raw, model, python, desktop };
}

describe("joined (pressure-source-1) AnalysisRun mirror", () => {
  it.each(LRS)("%s equals the Python carrier by canonical hash (route-independent fields normalized)", async name => {
    const { raw, model, python, desktop } = await mirror(name);
    expect(await canonicalSha256HexCheckedV1(normalize(desktop))).toBe(await canonicalSha256HexCheckedV1(normalize(python)));
    const run = desktop.analysis_run;
    expect(run.contract_evidence).toEqual(raw.contract_evidence);
    expect(run.source_block_recovery).toEqual(raw.source_block_recovery);
    expect(await verifyAnalysisRunRecord(desktop)).toBe("match");
    await expect(validateAnalysisRunV03(desktop, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
  });
});

describe("ordinary (pressure-1): the desktop builds and reads no record", () => {
  it.each(LR)("%s is refused by name; Python's carrier exists and the desktop does not read it", async name => {
    const { raw, model, python, manifest } = inputs("ordinary", name);
    expect(python.analysis_run.run_id).toBe(raw.run_id);
    await expect(buildAnalysisRunV03(raw, manifest, undefined, modelLoadBasisRefs(model))).rejects.toThrow("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH");
    await expect(validateAnalysisRunV03(python, raw, modelLoadBasisRefs(model))).rejects.toThrow("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH");
  });
});

describe("records bound to other evidence are refused", () => {
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

/** The desktop AnalysisRun record of a load/reference-state result mirrors
 * Python `build_analysis_run_v0_3` (T1 WP2): load-reference-1 is recorded like
 * physics-1, load-reference-source-1 like physics-source-1. Proof: the desktop
 * record of each committed raw equals the committed Python carrier
 * (fixtures/results/load_reference*.analysis_run.json) by canonical hash after
 * normalizing only the three route-independent fields in which the two
 * builders already differ for every identity (provenance labels, determinism
 * notes, and therefore the record self-hash). */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../types";
import { analysisRecordProjection, buildAnalysisRunV03, modelLoadBasisRefs, validateAnalysisRunV03, verifyAnalysisRunRecord } from "./analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "./hashService";
import { receivedFixture } from "../test-support/receivedText";

const root = resolve(__dirname, "../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const LR = ["connected-sparse", "connected-dense", "pressure-sparse", "pressure-dense"];
const LRS = ["n05", "n06", "fields", "mixed", "eigen_motion"].flatMap(name => ["sparse", "dense"].map(mode => `${name}-${mode}`));
const MODE: Record<string, string> = { sparse: "sparse_interactive", dense: "dense_scrutiny" };
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

function normalize(record: Json): Json {
  const copy = structuredClone(record);
  const run = copy.analysis_run;
  delete run.provenance;
  delete run.reproducibility.determinism_notes;
  run.hashes = run.hashes.filter((h: Json) => h.payload_scope !== "analysis_run_record");
  for (const ref of run.result_refs) delete ref.provenance;
  return copy;
}

async function mirror(kind: "load_reference" | "load_reference_source", name: string, executionResources?: Json) {
  const [base, mode] = [name.slice(0, name.lastIndexOf("-")), name.slice(name.lastIndexOf("-") + 1)];
  const folder = kind === "load_reference" ? "load_reference" : "load_reference_source";
  const raw = receivedFixture<MechanicsResult>(`fixtures/product_preview/${folder}/${base}-${MODE[mode]}.raw.json`);
  const model = json(`fixtures/product_preview/${folder}/${base}.request.json`).model as PreviewModel;
  const carrierName = `${kind}_${name.replace("-", "_")}`;
  const python = json(`fixtures/results/${carrierName}.analysis_run.json`) as AnalysisRunEnvelope;
  const ref = python.analysis_run.reproducibility.input_manifest_refs[0];
  const manifest = { manifest_ref: ref, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: raw.model_ref }, solver_basis: { solver_name: raw.producer!.component_name, solver_version: raw.producer!.component_version, solver_build_ref: "unavailable:solver-build-not-supplied" } } };
  const desktop = await buildAnalysisRunV03(raw, manifest, undefined, modelLoadBasisRefs(model), executionResources);
  return { raw, model, python, desktop, manifest };
}

describe.each([["load_reference", LR], ["load_reference_source", LRS]] as const)("%s AnalysisRun mirror", (kind, names) => {
  it.each(names)("%s equals the Python carrier by canonical hash (route-independent fields normalized)", async name => {
    const { raw, model, python, desktop } = await mirror(kind, name);
    expect(await canonicalSha256HexCheckedV1(normalize(desktop))).toBe(await canonicalSha256HexCheckedV1(normalize(python)));
    const run = desktop.analysis_run;
    if (kind === "load_reference_source") {
      expect(run.contract_evidence).toEqual(raw.contract_evidence);
      expect(run.source_block_recovery).toEqual(raw.source_block_recovery);
    } else {
      expect(Object.hasOwn(run, "contract_evidence")).toBe(false);
      expect(Object.hasOwn(run, "source_block_recovery")).toBe(false);
    }
    expect(await verifyAnalysisRunRecord(desktop)).toBe("match");
    await expect(validateAnalysisRunV03(desktop, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
  });
});

describe("records bound to other evidence are refused", () => {
  it.each([
    ["joined without its retained evidence", (r: Json) => { delete r.analysis_run.contract_evidence; }, "ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH"],
    ["joined without load_reference_states", (r: Json) => { delete r.analysis_run.contract_evidence.load_reference_states; }, "ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH"],
    ["joined without its receipt", (r: Json) => { delete r.analysis_run.source_block_recovery; }, "ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH"],
    ["joined with another table hash", (r: Json) => { r.analysis_run.reproducibility.semantic_contract.sha256 = "0".repeat(64); }, "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"],
  ])("%s", async (_name, edit, code) => {
    const { raw, model, desktop } = await mirror("load_reference_source", "mixed-sparse");
    const forged = structuredClone(desktop); edit(forged);
    await expect(validateAnalysisRunV03(forged, raw, modelLoadBasisRefs(model))).rejects.toThrow(code);
  });
  it("an ordinary record may not carry retained evidence", async () => {
    const { raw, model, desktop } = await mirror("load_reference", "pressure-sparse");
    const forged = structuredClone(desktop) as Json; forged.analysis_run.contract_evidence = structuredClone(raw.contract_evidence);
    await expect(validateAnalysisRunV03(forged, raw, modelLoadBasisRefs(model))).rejects.toThrow("ANALYSIS_PHYSICS_SOURCE_DOWNGRADE_FORBIDDEN");
  });
  it("a tampered raw result is refused by the reader before a record is built", async () => {
    const { raw, model } = await mirror("load_reference", "connected-sparse");
    const tampered = structuredClone(raw) as Json; tampered.contract_evidence.load_reference_states[0].members[0].fit_strain = 0.5;
    const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:x" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: raw.model_ref }, solver_basis: { solver_name: raw.producer!.component_name, solver_version: raw.producer!.component_version, solver_build_ref: "x" } } };
    await expect(buildAnalysisRunV03(tampered, manifest, undefined, modelLoadBasisRefs(model))).rejects.toThrow("SOURCE_LOAD_REFERENCE_");
  });
});

// DEL-04-07: the desktop records the native memory grant a completed run
// executed under, in the one current 0.3 format (Python twin:
// tests/test_analysis_run_compatibility.py). The headless producer runs under
// no memory pool, so its committed carriers carry none.
describe("execution resources in the analysis record", () => {
  const GiB = 1024 ** 3;
  const GRANT = { invocation_budget_bytes: 12 * GiB, pool_bytes: 48 * GiB, route_pricing: "ordinary_unpriced", ingress_reserved_bytes: 3 * 1024 ** 2 };
  const reseal = async (record: Json) => {
    const seal = record.analysis_run.hashes.find((h: Json) => h.payload_scope === "analysis_run_record");
    seal.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(record));
    return record;
  };

  it("adds exactly the grant to the record, under the record checksum, and nothing else", async () => {
    const { raw, model, python, desktop } = await mirror("load_reference_source", "n05-sparse", GRANT);
    expect(desktop.analysis_run.execution_resources).toEqual(GRANT);
    expect(Object.hasOwn(python.analysis_run, "execution_resources")).toBe(false);
    const expected = normalize(python); expected.analysis_run.execution_resources = GRANT;
    expect(await canonicalSha256HexCheckedV1(normalize(desktop))).toBe(await canonicalSha256HexCheckedV1(expected));
    expect(await verifyAnalysisRunRecord(desktop)).toBe("match");
    await expect(validateAnalysisRunV03(desktop, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
    const tampered = structuredClone(desktop) as Json; tampered.analysis_run.execution_resources.pool_bytes = 64 * GiB;
    expect(await verifyAnalysisRunRecord(tampered)).toBe("mismatch");
    await expect(validateAnalysisRunV03(tampered, raw, modelLoadBasisRefs(model))).rejects.toThrow("ANALYSIS_RECORD_CHECKSUM_MISMATCH");
  });

  it("a record that predates the member still verifies and validates", async () => {
    const { raw, model, desktop } = await mirror("load_reference_source", "n05-sparse");
    expect(Object.hasOwn(desktop.analysis_run, "execution_resources")).toBe(false);
    expect(await verifyAnalysisRunRecord(desktop)).toBe("match");
    await expect(validateAnalysisRunV03(desktop, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
  });

  it.each([
    ["another pricing", { route_pricing: "priced" }],
    ["a zero budget", { invocation_budget_bytes: 0 }],
    ["a boolean pool", { pool_bytes: true }],
    ["a negative intake", { ingress_reserved_bytes: -1 }],
    ["a fractional intake", { ingress_reserved_bytes: 1.5 }],
    ["an extra field", { queue_reason: null }],
  ])("refuses %s, in the builder and in a resealed record", async (_name, change) => {
    const { raw, model, manifest } = await mirror("load_reference_source", "n05-sparse");
    await expect(buildAnalysisRunV03(raw, manifest, undefined, modelLoadBasisRefs(model), { ...GRANT, ...change } as Json)).rejects.toThrow("ANALYSIS_EXECUTION_RESOURCES_INVALID");
    const { desktop } = await mirror("load_reference_source", "n05-sparse", GRANT);
    const forged = structuredClone(desktop) as Json; forged.analysis_run.execution_resources = { ...GRANT, ...change };
    await reseal(forged);
    expect(await verifyAnalysisRunRecord(forged)).toBe("match");
    await expect(validateAnalysisRunV03(forged, raw, modelLoadBasisRefs(model))).rejects.toThrow("ANALYSIS_EXECUTION_RESOURCES_INVALID");
  });
});

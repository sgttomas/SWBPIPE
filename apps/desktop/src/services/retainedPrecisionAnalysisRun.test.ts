/** U6d: the AnalysisRun carries the F2a successor's complete receipt (C1:162;
 * D2 4.9.6; plan 1d and 4). The record copies the receipt whole, never
 * recomputing or repairing it, and validation requires exact equality with the
 * source's (ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH); a receipt on any
 * other identity's record is refused (ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN).
 * The codes are Python's (U6b). Inputs are PP's pinned milestone successor bytes
 * (D-U6-5) delivered through mocked IPC: a unit transport replay, NOT a native
 * witness (plan F-1; D-U6-3). */
import { afterEach, describe, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../types";
import {
  ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN, ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH, analysisRecordProjection, buildAnalysisRunV02, buildAnalysisRunV03,
  modelLoadBasisRefs, validateAnalysisRunV03, verifyAnalysisRunRecord,
} from "./analysisRunCompatibility";
import { bindSourceResultDimensions, buildAnalysisRunPreview, runPreviewMechanics, type PreviewSolverMode } from "./previewService";
import { buildCurrentSessionInputManifest } from "./inputManifestService";
import { canonicalSha256HexCheckedV1, checkedJsonText } from "./hashService";
import { validateRetainedPrecision } from "../features/results/retainedPrecision";
import { PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256 } from "../features/results/numericalResultQuality";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../");
const caseFile = JSON.parse(readFileSync(resolve(root, "fixtures/results/retained_precision_carrier_cases.json"), "utf8"));
const MODES = ["sparse_interactive", "dense_scrutiny"] as const;
function milestone(mode: PreviewSolverMode) {
  const entry = caseFile.fixtures[`milestone_${mode}`];
  const bytes = readFileSync(resolve(root, entry.path));
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(entry.sha256);
  const doc = JSON.parse(bytes.toString("utf8"));
  return { source: doc.source as MechanicsResult, invocation: doc.invocation as Json, model: doc.invocation.request.model as PreviewModel };
}
async function delivered(mode: PreviewSolverMode, source?: MechanicsResult) {
  const pinned = milestone(mode);
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string) => {
    expect(command).toBe("run_preview_mechanics_with_solver_mode");
    return structuredClone(source ?? pinned.source);
  });
  const received = await runPreviewMechanics(pinned.model, mode);
  // The pinned request model has no desktop load-case status, which the input
  // manifest requires; the manifest model adds an invented one. Delivery and
  // registration use the exact pinned invocation.
  const manifestModel = { ...pinned.model, load_cases: pinned.model.load_cases.map(c => ({ ...c, status: "invented_test_status" })) } as PreviewModel;
  const inputManifest = await buildCurrentSessionInputManifest({ model: manifestModel, solver: { solver_name: received.producer!.component_name, solver_version: received.producer!.component_version, solver_build_ref: "unit-transport-replay-not-native-witness", solver_mode: mode, settings: {} }, active_rule_packs: [], external_assets: [] });
  return { ...pinned, received, inputManifest };
}
const manifestFor = (source: MechanicsResult) => ({ manifest_ref: { object_type: "InputManifest", ref: "manifest:invented-retained" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: source.model_ref }, solver_basis: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:invented" } } });
/** The reader's own projection to the base identity (C1 G7). */
function baseProjection(source: MechanicsResult): MechanicsResult {
  const p = structuredClone(source) as Json;
  delete p.retained_precision;
  p.producer.semantic_contract_id = PREVIEW_PHYSICS_CONTRACT_ID;
  p.formulation_basis.profile_id = "product_preview_mechanics_v1";
  for (const row of p.results) delete row.recovery_method;
  return p;
}

afterEach(() => { invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });

describe.each(MODES)("%s: the AnalysisRun copies the receipt and validates its equality", (mode) => {
  it("builds through the product path with the complete receipt, byte-equal, and validates", async () => {
    const { received, inputManifest, invocation, model } = await delivered(mode);
    const before = checkedJsonText(received);
    const record = await buildAnalysisRunPreview(received, { inputManifest });
    expect(checkedJsonText(received)).toBe(before);
    const run = record.analysis_run;
    expect(checkedJsonText(run.retained_precision)).toBe(checkedJsonText(received.retained_precision));
    expect(run.retained_precision).not.toBe(received.retained_precision);
    expect(Object.hasOwn(run, "source_block_recovery")).toBe(false);
    expect(Object.hasOwn(run, "contract_evidence")).toBe(false);
    expect(record.schema_version).toBe("0.3.0");
    expect(run.reproducibility.semantic_contract).toStrictEqual({ id: PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, sha256: PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256 });
    // The received_result hash binds the whole raw document, receipt included.
    expect(run.hashes.find(h => h.payload_scope === "received_result")!.value).toBe(await canonicalSha256HexCheckedV1(received));
    expect(await verifyAnalysisRunRecord(record)).toBe("match");
    await validateAnalysisRunV03(record, received, modelLoadBasisRefs(model));
    // Reader revalidation from the record plus the source, with the invocation.
    const { retained_precision: _omit, ...publication } = received;
    const back = { ...structuredClone(publication), retained_precision: structuredClone(run.retained_precision) };
    const original = await validateRetainedPrecision(received, invocation);
    expect(await validateRetainedPrecision(back, invocation)).toStrictEqual(original);
    // U7: the revalidated record is eligible with its invocation, as the original is.
    expect([original.numerical_eligible, original.standing]).toStrictEqual([true, "eligible"]);
  });

  it("row interpretation equals the base preview-physics-1 projection's except the contract binding", async () => {
    const { received, model } = await delivered(mode);
    const record = await buildAnalysisRunV03(received, manifestFor(received), undefined, modelLoadBasisRefs(model));
    const base = await buildAnalysisRunV03(baseProjection(received), manifestFor(received), undefined, modelLoadBasisRefs(model));
    expect(record.analysis_run.result_refs).toHaveLength(received.results.length);
    record.analysis_run.result_refs.forEach((row, i) => {
      const other = base.analysis_run.result_refs[i];
      expect(row.semantic_contract).toStrictEqual({ id: PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, sha256: PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256, signature_id: other.semantic_contract!.signature_id });
      expect({ ...row, semantic_contract: null, hash_refs: null }).toStrictEqual({ ...other, semantic_contract: null, hash_refs: null });
    });
  });

  it.each([
    ["dropped", (run: Json) => { delete run.retained_precision; }],
    ["altered", (run: Json) => { run.retained_precision.receipt_sha256 = "0".repeat(64); }],
    ["altered in its body", (run: Json) => { run.retained_precision.body.cases[0].status = "not_required"; }],
    ["replaced by null", (run: Json) => { run.retained_precision = null; }],
  ] as const)("a %s copy fails with ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH", async (_label, edit) => {
    const { received, model } = await delivered(mode);
    const record = await buildAnalysisRunV03(received, manifestFor(received), undefined, modelLoadBasisRefs(model));
    const changed = structuredClone(record); edit(changed.analysis_run);
    await expect(validateAnalysisRunV03(changed, received, modelLoadBasisRefs(model))).rejects.toThrow(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);
    // The same copy with its keys reordered is the same receipt.
    const reordered = structuredClone(record);
    const receipt = reordered.analysis_run.retained_precision as Json;
    reordered.analysis_run.retained_precision = { receipt_sha256: receipt.receipt_sha256, body: receipt.body };
    await validateAnalysisRunV03(reordered, received, modelLoadBasisRefs(model));
  });

  it("an edited source is refused by the reader at build and at validation, with its own code", async () => {
    const { received, model, source } = await delivered(mode);
    const record = await buildAnalysisRunV03(received, manifestFor(received), undefined, modelLoadBasisRefs(model));
    const edited = structuredClone(source); edited.results[0].value = 12345;
    await expect(buildAnalysisRunV03(edited, manifestFor(edited), undefined, modelLoadBasisRefs(model))).rejects.toThrow("RETAINED_PRECISION_RECEIPT_MISMATCH");
    await expect(validateAnalysisRunV03(record, edited, modelLoadBasisRefs(model))).rejects.toThrow("RETAINED_PRECISION_RECEIPT_MISMATCH");
    // A source the checked profile refuses is recorded only after the reader: its own code first.
    const surrogate = structuredClone(source) as Json; surrogate.results[0].metadata.basis = "\ud800";
    await expect(buildAnalysisRunV03(surrogate, manifestFor(surrogate), undefined, modelLoadBasisRefs(model))).rejects.toThrow("RETAINED_PRECISION_RECEIPT_MISMATCH");
    // The receipt is never repaired: a recomputed-looking copy is still refused.
    const quality = structuredClone(source) as Json; quality.numerical_quality.status = "checks_passed";
    await expect(buildAnalysisRunV03(quality, manifestFor(quality), undefined, modelLoadBasisRefs(model))).rejects.toThrow("RETAINED_PRECISION_RECEIPT_MISMATCH");
  });

  it("a relabelled successor is refused by every builder; the legacy builder refuses the successor", async () => {
    const { received, model } = await delivered(mode);
    const relabelled = structuredClone(received) as Json;
    relabelled.producer.semantic_contract_id = PREVIEW_PHYSICS_CONTRACT_ID;
    relabelled.formulation_basis.profile_id = "product_preview_mechanics_v1";
    await expect(buildAnalysisRunV03(relabelled, manifestFor(relabelled), undefined, modelLoadBasisRefs(model))).rejects.toThrow("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
    const record = await buildAnalysisRunV03(received, manifestFor(received), undefined, modelLoadBasisRefs(model));
    await expect(validateAnalysisRunV03(record, relabelled, modelLoadBasisRefs(model))).rejects.toThrow("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH");
    await expect(buildAnalysisRunV02(received, manifestFor(received))).rejects.toThrow("HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED");
  });

  it("rule-check status composition changes only the record's status, never the raw document", async () => {
    const { received, inputManifest } = await delivered(mode);
    const before = checkedJsonText(received);
    const plain = await buildAnalysisRunPreview(received, { inputManifest });
    const checked = await buildAnalysisRunPreview(received, { inputManifest, ruleCheckAggregate: "USER_RULE_CHECKED" });
    expect(checked.analysis_run.analysis_status).toContain("USER_RULE_CHECKED");
    expect(plain.analysis_run.analysis_status).toContain("RULE_INPUTS_INCOMPLETE");
    const received_result = (r: AnalysisRunEnvelope) => r.analysis_run.hashes.find(h => h.payload_scope === "received_result");
    expect(received_result(checked)).toStrictEqual(received_result(plain));
    expect(checkedJsonText(checked.analysis_run.retained_precision)).toBe(checkedJsonText(plain.analysis_run.retained_precision));
    expect(checkedJsonText(received)).toBe(before);
    // Legacy dimension binding is a no-op for the successor: the same object back.
    expect(bindSourceResultDimensions(received)).toBe(received);
  });
});

describe("the historical v0.2 builder never drops a receipt (RV91 SF-1; Python's F-U6b-3 twin)", () => {
  const legacy = () => JSON.parse(readFileSync(resolve(root, "fixtures/product_preview/invented_mechanics_result.json"), "utf8")) as MechanicsResult;
  const legacyManifest = (source: MechanicsResult) => ({ manifest_ref: { object_type: "InputManifest", ref: "manifest:invented-legacy" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: source.model_ref }, solver_basis: { solver_name: "synthetic", solver_version: "1", solver_build_ref: "synthetic@1" } } });
  // Both legacy record shapes the builder accepts (RV91 N-1 on round 02): 0.1.0 and 0.2.0.
  const shaped = (version: string) => { const source = bindSourceResultDimensions(legacy()) as Json; source.schema_version = version; return source; };
  it.each(["0.1.0", "0.2.0"])("a %s legacy-shaped source without a receipt or token still builds (control)", async (version) => {
    const source = shaped(version);
    expect(source.results.length).toBeGreaterThan(2);
    const record = await buildAnalysisRunV02(source, legacyManifest(source));
    expect(record.schema_version).toBe("0.2.0");
    expect(Object.hasOwn(record.analysis_run, "retained_precision")).toBe(false);
  });
  it.each(["0.1.0", "0.2.0"].flatMap(version => [
    [version, "a retained_precision object", (s: Json) => { s.retained_precision = structuredClone(milestone("sparse_interactive").source.retained_precision); }],
    [version, "an empty retained_precision object", (s: Json) => { s.retained_precision = {}; }],
    [version, "a null retained_precision member", (s: Json) => { s.retained_precision = null; }],
    [version, "a W1 token on the first row", (s: Json) => { s.results[0].recovery_method = "contribution_preserving_multiprecision_v1"; }],
    [version, "a W1 token on the last row only", (s: Json) => { s.results[s.results.length - 1].recovery_method = "contribution_preserving_multiprecision_v1"; }],
  ] as [string, string, (s: Json) => void][]))("a %s legacy-shaped source carrying %s is refused ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN", async (version, _label, edit) => {
    const source = shaped(version); edit(source);
    await expect(buildAnalysisRunV02(source, legacyManifest(source))).rejects.toThrow(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);
  });
  it("a source with precision metadata keeps its existing refusal", async () => {
    const { source } = milestone("dense_scrutiny");
    await expect(buildAnalysisRunV02(source, legacyManifest(source))).rejects.toThrow("HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED");
  });
});

describe("no other identity's record carries a receipt", () => {
  it("a preview-physics-1 record given the successor's receipt is ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN", async () => {
    const preview = JSON.parse(readFileSync(resolve(root, "fixtures/results/preview_physics_connected_sparse.json"), "utf8")) as MechanicsResult;
    const record = await buildAnalysisRunV03(preview, manifestFor(preview));
    expect(Object.hasOwn(record.analysis_run, "retained_precision")).toBe(false);
    await validateAnalysisRunV03(record, preview);
    const injected = structuredClone(record);
    injected.analysis_run.retained_precision = structuredClone(milestone("sparse_interactive").source.retained_precision);
    injected.analysis_run.hashes = [...injected.analysis_run.hashes.filter(h => h.payload_scope !== "analysis_run_record"), { ...record.analysis_run.hashes.find(h => h.payload_scope === "analysis_run_record")!, value: await canonicalSha256HexCheckedV1(analysisRecordProjection(injected)) }];
    await expect(validateAnalysisRunV03(injected, preview)).rejects.toThrow(ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
    const nulled = structuredClone(record); (nulled.analysis_run as Json).retained_precision = null;
    await expect(validateAnalysisRunV03(nulled, preview)).rejects.toThrow(ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
  });
});

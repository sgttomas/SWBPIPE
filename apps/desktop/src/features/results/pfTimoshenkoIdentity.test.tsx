/** DEL-05-03 Design `pf-timoshenko.md` §4: the desktop admits the pressure-free Timoshenko
 * identity (`preview-physics-timoshenko-1`) beside preview-physics-2, keyed by the envelope's
 * `semantic_contract_id`: the Rust reader's binding, the fresh set (the Current result), the
 * AnalysisRun record tables, the result export, the report package's admitted set and its
 * limitation 0 entry, and the rendered report's `pf_timoshenko` section and texts. A crossed
 * model (one that does not select Timoshenko) is not eligible.
 *
 * The Timoshenko source is a TRANSFORMED STAND-IN until the product publishes one: the committed
 * preview-physics-2 producer bytes (`fixtures/results/preview_physics_connected_sparse.json`) with
 * the identity, the profile and limitation 0 swapped, delivered through the unit solve-job
 * transport for the replay model with `model_settings.shear_model` set. Unit transport replay
 * through mocked IPC; NOT a native UI qualification witness. Invented ids. */
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { validateAnalysisRunV03 } from "../../services/analysisRunCompatibility";
import { buildCurrentResultExport } from "../result-export/resultExportAdapter";
import { readerDispatch, sourceContract } from "./resultReader";
import { numericalResultStanding } from "./resultStanding";
import { FRESH_SEMANTIC_CONTRACT_IDS, isFreshSemanticResult } from "./knownSemanticLimitations";
import { PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_IDS, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "./numericalResultQuality";
import { PREVIEW_PHYSICS_CONTRACT_SHA256, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256 } from "../../test-support/contractBindings";
import { buildReportPackageRequest, reportPackageExactSubmission, REPORT_PACKAGE_ADMITTED_CONTRACT_IDS } from "../report/reportPackageRequest";
import { assembleRenderedReport } from "../report/renderedReportAssembly";
import { PF_TIMOSHENKO_SECTION, pfTimoshenkoLimitationEntry, REPORT_PF_TIMOSHENKO_SECTIONS_MISMATCH } from "../report/pfTimoshenkoReport";
import timoshenkoTable from "../../../../../fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const SOLVER = { solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "open_pipe_stress_product_physics@0.2.0", solver_mode: "sparse_interactive", settings: {} };

afterEach(() => {
  invokeMock.mockReset();
  vi.restoreAllMocks();
  resetSessionResultCustodyForTests();
  delete (window as Json).__TAURI_INTERNALS__;
});

function timoshenkoBytes(bytes: string): string {
  const source = JSON.parse(bytes) as Json;
  source.producer.semantic_contract_id = PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID;
  source.formulation_basis = { profile_id: timoshenkoTable.formulation_profile_id, limitations: timoshenkoTable.supported_profile_limitations };
  return JSON.stringify(source);
}

/** A Current session over `bytes` for the replay model with `shearModel` (absent when null). */
async function session(shearModel: string | null, timoshenko: boolean) {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const model = structuredClone(pair.model) as PreviewModel;
  if (shearModel !== null) model.model_settings = { ...(model.model_settings ?? {}), shear_model: shearModel };
  const bytes = timoshenko ? timoshenkoBytes(pair.bytes) : pair.bytes;
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(createSolveJobTransport(() => bytes, { jobIdPrefix: "invented-pf-timoshenko-job" }).invoke);
  const result = await runPreviewMechanics(model);
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: SOLVER, active_rule_packs: [], external_assets: [] });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, inputManifest, analysisRun };
}

describe("the pressure-free Timoshenko identity on the desktop", () => {
  it("is a fresh identity of the preview route, beside preview-physics-2", () => {
    expect(PREVIEW_PHYSICS_CONTRACT_IDS).toEqual([PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID]);
    expect(FRESH_SEMANTIC_CONTRACT_IDS).toContain(PREVIEW_PHYSICS_CONTRACT_ID);
    expect(FRESH_SEMANTIC_CONTRACT_IDS).toContain(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    expect(REPORT_PACKAGE_ADMITTED_CONTRACT_IDS).toContain(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    expect(timoshenkoTable.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    expect(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256).toBe("94538daa46b4795550a8c9e6b4ec05d8eb40d5be34b78c1a630613dc7ae2732f");
    expect(PREVIEW_PHYSICS_CONTRACT_SHA256).toBe("5a6630d9157ad1f4232c558d0cd57414677add8d1c95cc0a5d19a2579fb3a05b");
  });

  it("the Rust reader binds each identity's own table, and mixed arrays read unsupported", () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    const eb = JSON.parse(pair.bytes) as MechanicsResult;
    const ti = JSON.parse(timoshenkoBytes(pair.bytes)) as MechanicsResult;
    expect(readerDispatch(eb).binding).toEqual({ id: PREVIEW_PHYSICS_CONTRACT_ID, sha256: PREVIEW_PHYSICS_CONTRACT_SHA256 });
    expect(readerDispatch(ti).binding).toEqual({ id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, sha256: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256 });
    expect(sourceContract(ti)).toBe("preview_physics");
    const mixed = structuredClone(ti) as Json;
    mixed.formulation_basis.limitations[0] = eb.formulation_basis!.limitations[0];
    expect(numericalResultStanding(mixed as MechanicsResult, structuredClone(pair.model) as PreviewModel).eligible).toBe(false);
    expect(() => pfTimoshenkoLimitationEntry(mixed as MechanicsResult, "run")).toThrow(REPORT_PF_TIMOSHENKO_SECTIONS_MISMATCH);
  });

  it("becomes the Current result, records its AnalysisRun, exports and refuses a crossed model", async () => {
    const { model, result, inputManifest, analysisRun } = await session("timoshenko_annulus_energy_v1", true);
    expect(result.producer?.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    expect(isFreshSemanticResult(result)).toBe(true);
    expect(numericalResultStanding(result, model).eligible).toBe(true);
    const binding = { id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, sha256: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256 };
    expect(analysisRun.analysis_run.reproducibility.semantic_contract).toEqual(binding);
    expect(new Set(analysisRun.analysis_run.result_refs.map((ref: Json) => ref.semantic_contract.id))).toEqual(new Set([binding.id]));
    await validateAnalysisRunV03(analysisRun, result);
    const document = await buildCurrentResultExport({ model, result, analysisRun, inputManifest });
    expect((document.result_envelope as Json).producer.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    expect((document.result_envelope as Json).semantic_contract_ref).toEqual({ ref_type: "semantic_contract", ref_id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID });
    // The same bytes from a model without the selection: a crossed pair, not eligible.
    const crossed = structuredClone(model);
    delete (crossed as Json).model_settings;
    const standing = numericalResultStanding(result, crossed);
    expect(standing.eligible).toBe(false);
    expect(standing.findings.join(" ")).toContain("SOURCE_PREVIEW_PHYSICS_SHEAR_MODEL_IDENTITY");
  });

  it("the report package carries the identity's limitation 0; Euler–Bernoulli reports do not", async () => {
    const ti = await session("timoshenko_annulus_energy_v1", true);
    const request = await buildReportPackageRequest({ ...ti, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null });
    const run = ti.result.run_id;
    const limitations = (request.report as Json).report_sections.limitations as Json[];
    expect(limitations.map((item) => item.limitation_id)).toEqual([`limitation:${run}:technical-preview`, `limitation:${run}:pf-timoshenko:1`]);
    expect(limitations[1]).toMatchObject({
      source: { ref_type: "semantic_contract", ref_id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID },
      affected_scope: { ref_type: "analysis_run", ref_id: run },
      statement: timoshenkoTable.supported_profile_limitations[0],
      effect: limitations[0].effect,
      provenance: limitations[0].provenance
    });
    // The package binds the entry against its source; no section and no texts are sent.
    expect(PF_TIMOSHENKO_SECTION in (request.report as Json)).toBe(false);
    expect(reportPackageExactSubmission(request)).toBeNull();
    const eb = await session(null, false);
    const plain = await buildReportPackageRequest({ ...eb, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null });
    expect(((plain.report as Json).report_sections.limitations as Json[]).map((item) => item.limitation_id)).toEqual([`limitation:${eb.result.run_id}:technical-preview`]);
  });

  it("the rendered report carries the pf_timoshenko section and its texts; Euler–Bernoulli carries neither", async () => {
    const ti = await session("timoshenko_annulus_energy_v1", true);
    const assembly = await assembleRenderedReport({ ...ti, projectSummary: null, ruleCheckRun: null, numericalHealth: null, context: "local_private", intent: true, generatedAt: null });
    const input = assembly.input as Json;
    expect(input.exact_route).toBeUndefined();
    expect(input[PF_TIMOSHENKO_SECTION]).toEqual({ semantic_contract_id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, admission: { ...assembly.admission!.binding } });
    expect(assembly.admission!.binding.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    expect(input.report_sections.limitations[1].limitation_id).toBe(`limitation:${ti.result.run_id}:pf-timoshenko:1`);
    const texts = assembly.exactSubmission!();
    expect(JSON.parse(texts.exactResultText).producer.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    const eb = await session(null, false);
    const plain = await assembleRenderedReport({ ...eb, projectSummary: null, ruleCheckRun: null, numericalHealth: null, context: "local_private", intent: true, generatedAt: null });
    expect(PF_TIMOSHENKO_SECTION in (plain.input as Json)).toBe(false);
    expect(plain.exactSubmission).toBeNull();
    expect(plain.admission).toBeNull();
  });
});

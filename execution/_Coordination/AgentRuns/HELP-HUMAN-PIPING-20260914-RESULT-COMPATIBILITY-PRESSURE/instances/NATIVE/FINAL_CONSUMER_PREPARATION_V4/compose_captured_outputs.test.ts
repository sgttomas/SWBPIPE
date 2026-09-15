import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { buildAnalysisRunV02, verifyAnalysisRunRecord } from "@final-product/services/analysisRunCompatibility";
import { buildCurrentSessionInputManifest } from "@final-product/services/inputManifestService";
import { buildCurrentResultExport, validateResultDocument } from "@final-product/features/result-export/resultExportAdapter";
import { buildStressNeutralExportPacket, validateStressNeutralExportPacket } from "@final-product/features/stress-neutral/StressNeutralExportPanel";

type Signature = { signature_id: string; kind: string; unit: string; component: string | null };
type CaseSpec = {
  request_path: string;
  output_path: string;
  request_sha256: string;
  output_sha256: string;
  expected_signatures: Signature[];
  solver: { solver_name: string; solver_version: string; solver_build_ref: string; solver_mode: string; settings: Record<string, unknown> };
};
type CaseMap = {
  schema_version: "1.0.0";
  final_candidate: { source_commit: string; source_tree_sha256: string; runner_sha256: string; wasm_sha256: string };
  cases: Record<string, CaseSpec>;
};

const REQUIRED_CASES = ["modulus", "combination_modulus", "curved", "friction", "hanger_constant_effort", "rotational_nonlinear", "zero_pressure_longitudinal"];
const REQUIRED_SUPPLEMENTAL_SIGNATURES = ["supported-source-022", "supported-source-029", "supported-source-050", "supported-source-051", "supported-source-052", "supported-source-053", "supported-source-054", "supported-source-056", "supported-source-057", "supported-source-058", "supported-source-059"];
const sha256 = (bytes: Buffer | string) => createHash("sha256").update(bytes).digest("hex");
const load = (filename: string) => JSON.parse(readFileSync(filename, "utf8"));
const canonicalClone = (value: unknown) => JSON.parse(JSON.stringify(value));
const assertReleased = (value: unknown, label: string) => {
  if (typeof value !== "string" || !value || value === "UNRELEASED" || value.includes("<")) throw new Error(`FINAL_BINDING_UNRELEASED: ${label}`);
};

describe("captured headless producer to desktop consumers", () => {
  it("composes all seven real outputs and preserves the source carrier", async () => {
    const mapPath = process.env.FINAL_CONSUMER_CASE_MAP;
    const outputRoot = process.env.FINAL_CONSUMER_OUTPUT_DIR;
    if (!mapPath || !path.isAbsolute(mapPath) || !outputRoot || !path.isAbsolute(outputRoot)) throw new Error("FINAL_CONSUMER_CASE_MAP and FINAL_CONSUMER_OUTPUT_DIR must be absolute");
    const map = load(mapPath) as CaseMap;
    expect(Object.keys(map.cases).sort()).toEqual([...REQUIRED_CASES].sort());
    for (const [key, value] of Object.entries(map.final_candidate)) assertReleased(value, `final_candidate.${key}`);
    const workingRoot = process.env.FINAL_WORKING_ROOT!;
    const semanticContract = load(path.join(workingRoot, "fixtures/results/semantic_contract_v0_2.json"));
    expect(semanticContract.rows).toHaveLength(60);
    const signatureKey = (row: any) => JSON.stringify([row.kind, row.unit, row.metadata?.component ?? row.component ?? null]);
    const contractByKey = new Map(semanticContract.rows.map((row: any) => [signatureKey(row), row.signature_id]));
    const observedSignatureIds = new Set<string>();
    const receipts: unknown[] = [];
    for (const caseName of REQUIRED_CASES) {
      const spec = map.cases[caseName];
      for (const field of ["request_path", "output_path", "request_sha256", "output_sha256"] as const) assertReleased(spec[field], `${caseName}.${field}`);
      for (const field of ["solver_name", "solver_version", "solver_build_ref", "solver_mode"] as const) assertReleased(spec.solver[field], `${caseName}.solver.${field}`);
      const requestBytes = readFileSync(spec.request_path);
      const outputBytes = readFileSync(spec.output_path);
      expect(sha256(requestBytes), `${caseName} request bytes`).toBe(spec.request_sha256);
      expect(sha256(outputBytes), `${caseName} output bytes`).toBe(spec.output_sha256);
      const requestEnvelope = JSON.parse(requestBytes.toString("utf8"));
      const runnerEnvelope = JSON.parse(outputBytes.toString("utf8"));
      const model = requestEnvelope?.solve?.preview_model?.model;
      const result = runnerEnvelope?.mechanics_envelope;
      if (!model || !result) throw new Error(`CAPTURE_SHAPE_INVALID: ${caseName}`);
      expect(result.status?.mechanics, `${caseName} mechanics status`).toBe("MECHANICS_SOLVED");
      expect(result.model_ref, `${caseName} model identity`).toBe(model.project?.id);
      const sourceBefore = JSON.stringify(result);
      for (const row of result.results) {
        const signatureId = contractByKey.get(signatureKey(row));
        if (typeof signatureId === "string") observedSignatureIds.add(signatureId);
      }
      for (const expected of spec.expected_signatures) {
        const frozen = semanticContract.rows.find((row: any) => row.signature_id === expected.signature_id);
        expect(frozen && [frozen.kind, frozen.unit, frozen.component]).toEqual([expected.kind, expected.unit, expected.component]);
        expect(result.results.some((row: any) => row.kind === expected.kind && row.unit === expected.unit && (row.metadata?.component ?? null) === expected.component), `${caseName}:${expected.signature_id}`).toBe(true);
      }
      const inputManifest = await buildCurrentSessionInputManifest({ model, solver: spec.solver, active_rule_packs: [], external_assets: [] });
      const analysisRun = await buildAnalysisRunV02(result, inputManifest);
      expect(await verifyAnalysisRunRecord(analysisRun), `${caseName} analysis record`).toBe("match");
      const resultExport = await buildCurrentResultExport({ model, result, analysisRun, inputManifest });
      await validateResultDocument(resultExport, result);
      const stressPacket = await buildStressNeutralExportPacket({ model, result, analysisRun });
      await validateStressNeutralExportPacket(stressPacket);
      expect(JSON.stringify(result), `${caseName} source carrier mutation`).toBe(sourceBefore);
      expect(analysisRun.analysis_run.result_refs).toHaveLength(result.results.length);
      expect(resultExport.result_envelope.row_accounting).toHaveLength(result.results.length);
      expect(stressPacket.result_rows).toHaveLength(result.results.length);
      const caseDir = path.join(outputRoot, caseName);
      mkdirSync(caseDir, { recursive: true });
      const artifacts: Record<string, unknown> = {
        "input_manifest.json": inputManifest,
        "analysis_run_v0_2.json": analysisRun,
        "result_export_v0_2.json": resultExport,
        "stress_neutral_v0_2.json": stressPacket,
      };
      const artifactHashes: Record<string, string> = {};
      for (const [filename, value] of Object.entries(artifacts)) {
        const bytes = `${JSON.stringify(value, null, 2)}\n`;
        writeFileSync(path.join(caseDir, filename), bytes);
        artifactHashes[filename] = sha256(bytes);
      }
      const receipt = {
        case: caseName,
        claim_class: "actual_product_frontend_constructor_composition_of_captured_headless_output",
        gui_or_browser_download_claimed: false,
        python_member_materialization_claimed: false,
        candidate: map.final_candidate,
        captured_request: { path: spec.request_path, sha256: spec.request_sha256 },
        captured_output: { path: spec.output_path, sha256: spec.output_sha256 },
        source_run_id: result.run_id,
        source_model_ref: result.model_ref,
        source_row_count: result.results.length,
        expected_signature_ids: spec.expected_signatures.map((item) => item.signature_id),
        artifact_sha256: artifactHashes,
      };
      writeFileSync(path.join(caseDir, "composition_receipt.json"), `${JSON.stringify(receipt, null, 2)}\n`);
      receipts.push(canonicalClone(receipt));
    }
    for (const signatureId of REQUIRED_SUPPLEMENTAL_SIGNATURES) expect(observedSignatureIds.has(signatureId), `actual supplemental signature ${signatureId}`).toBe(true);
    expect(observedSignatureIds.has("supported-source-059"), "runtime 059 reachability is mandatory").toBe(true);
    writeFileSync(path.join(outputRoot, "composition_index.json"), `${JSON.stringify({ cases: receipts, semantic_contract_signature_count: semanticContract.rows.length, observed_signature_ids: [...observedSignatureIds].sort(), required_supplemental_signature_ids: REQUIRED_SUPPLEMENTAL_SIGNATURES, all_required_supplemental_signatures_observed: true, full_sixty_runtime_coverage_requires_shipped_sparse_dense_evidence_too: true }, null, 2)}\n`);
  });
});

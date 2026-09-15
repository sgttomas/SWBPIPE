import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { expect, test } from "vitest";
import { buildAnalysisRunV02 } from "@final-product/services/analysisRunCompatibility";
import { buildCurrentSessionInputManifest } from "@final-product/services/inputManifestService";
import { buildCurrentResultExport } from "@final-product/features/result-export/resultExportAdapter";

const sha256 = (value: Buffer) => createHash("sha256").update(value).digest("hex");

test("authentic raw modulus producer reaches desktop result constructor", async () => {
  const map = JSON.parse(readFileSync(process.env.FINAL_CONSUMER_CASE_MAP!, "utf8"));
  const spec = map.cases.modulus;
  const requestBytes = readFileSync(spec.request_path);
  const outputBytes = readFileSync(spec.output_path);
  expect(sha256(requestBytes)).toBe(spec.request_sha256);
  expect(sha256(outputBytes)).toBe(spec.output_sha256);
  const request = JSON.parse(requestBytes.toString("utf8"));
  const delivered = JSON.parse(outputBytes.toString("utf8"));
  const result = (delivered.payload ?? delivered).mechanics_envelope;
  expect(result.status.mechanics).toBe("MECHANICS_SOLVED");
  expect(result.results.some((row: any) => row.kind === "modulus_basis_record")).toBe(true);
  expect(result.results.every((row: any) => !Object.hasOwn(row, "dimension"))).toBe(true);
  const model = request.solve.preview_model.model;
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: spec.solver, active_rule_packs: [], external_assets: [] });
  const analysisRun = await buildAnalysisRunV02(result, inputManifest);
  await buildCurrentResultExport({ model, result, analysisRun, inputManifest });
});

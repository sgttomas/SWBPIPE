/** RV92: is the TS AnalysisRun receipt-copy check exact? Review harness only. */
import { describe, expect, it } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { buildAnalysisRunV03, validateAnalysisRunV03, analysisRecordProjection, modelLoadBasisRefs } from "./services/analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "./services/hashService";
import { validateRetainedPrecision } from "./features/results/retainedPrecision";
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const out = process.env.RV92_EXACT;
describe.skipIf(!out)("RV92 exactness (TypeScript)", () => {
  it("probes", async () => {
    const res: Json = {};
    for (const mode of ["sparse_interactive", "dense_scrutiny"]) {
      const doc = JSON.parse(readFileSync(resolve(__dirname, `../../../fixtures/results/retained_precision_milestone_successor_${mode}.json`), "utf8"));
      const src = doc.source, inv = doc.invocation, model = inv.request.model;
      const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:rv92" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: src.model_ref }, solver_basis: { solver_name: src.producer.component_name, solver_version: src.producer.component_version, solver_build_ref: "rv92" } } };
      const record: Json = await buildAnalysisRunV03(src, manifest as Json, undefined, modelLoadBasisRefs(model));
      for (const [name, path, value] of [["int0_to_false", ["body", "builds", 0, "id"], false], ["int1_to_true", ["body", "builds", 1, "id"], true], ["int_to_float", ["body", "builds", 0, "id"], 0.0]] as const) {
        const m = structuredClone(record); let t = m.analysis_run.retained_precision;
        for (const k of path.slice(0, -1)) t = t[k];
        t[path[path.length - 1]] = value;
        const o: Json = { path };
        try { await validateAnalysisRunV03(m, src, modelLoadBasisRefs(model)); o.unsealed = "ok"; } catch (e) { o.unsealed = String((e as Error).message).split(":")[0]; }
        const h = m.analysis_run.hashes.find((x: Json) => x.payload_scope === "analysis_run_record"); h.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(m));
        try { await validateAnalysisRunV03(m, src, modelLoadBasisRefs(model)); o.resealed = "ok"; } catch (e) { o.resealed = String((e as Error).message).split(":")[0]; }
        const back = structuredClone(src); back.retained_precision = m.analysis_run.retained_precision;
        try { await validateRetainedPrecision(back, inv); o.copy_revalidates = true; } catch (e) { o.copy_revalidates = `${(e as Json).gate}:${(e as Json).code}`; }
        res[`${mode}:${name}`] = o;
      }
    }
    writeFileSync(out!, JSON.stringify(res, null, 1));
    expect(true).toBe(true);
  }, 600_000);
});

/** RV92 (U6f): the receipt's survival, TypeScript. Step a: register the
 * milestone with its invocation (the product's registration route), build and
 * validate the AnalysisRun, write it, then save and reopen the project (JSON
 * text, as src-tauri persists it). Step b: put every carrier output's receipt
 * back on the raw source and revalidate. Review harness only. */
import { describe, expect, it } from "vitest";
import { appendFileSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { numericalResultStanding } from "./features/results/numericalResultQuality";
import { validateRetainedPrecision } from "./features/results/retainedPrecision";
import { registerRetainedPrecision, retainedPrecisionStanding, retainedPrecisionRegistration } from "./features/results/retainedPrecisionStanding";
import { buildHistoricalRunContext } from "./features/results/HistoricalRunContext";
import { buildAnalysisRunV03, validateAnalysisRunV03, verifyAnalysisRunRecord, modelLoadBasisRefs } from "./services/analysisRunCompatibility";
import { computeModelHash, computeProjectEnvelopeHash } from "./services/hashService";
import type { LocalProjectEnvelope, MechanicsResult, PreviewModel } from "./types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const dir = process.env.RV92_SURVIVAL, step = process.env.RV92_STEP;
const root = resolve(__dirname, "../../../");
const log = (o: Json) => appendFileSync(`${dir}/ts_step_${step}.jsonl`, JSON.stringify(o) + "\n");

describe.skipIf(!dir || !step)("RV92 survival (TypeScript)", () => {
  it("carries the receipt", async () => {
    for (const mode of ["sparse_interactive", "dense_scrutiny"]) {
      const doc = JSON.parse(readFileSync(resolve(root, `fixtures/results/retained_precision_milestone_successor_${mode}.json`), "utf8"));
      const src = doc.source as MechanicsResult, inv = doc.invocation, model = inv.request.model as PreviewModel;
      const original = JSON.stringify((src as Json).retained_precision);
      if (step === "a") {
        await registerRetainedPrecision(src, inv);
        const standing = numericalResultStanding(src, model);
        const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:rv92" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: src.model_ref }, solver_basis: { solver_name: src.producer!.component_name, solver_version: src.producer!.component_version, solver_build_ref: "rv92" } } };
        const record = await buildAnalysisRunV03(src, manifest as Json, undefined, modelLoadBasisRefs(model));
        await validateAnalysisRunV03(record, src, modelLoadBasisRefs(model));
        writeFileSync(`${dir}/ts_ar_${mode}.json`, JSON.stringify(record));
        // Save (JSON text) and reopen: revalidated, never registered.
        const envelope: Json = JSON.parse(JSON.stringify({ model, mechanics_result: src, analysis_run: record, model_hash: await computeModelHash(model), editor_intents: [], proposal: null, selected_review_target: null }));
        envelope.project_envelope_hash = await computeProjectEnvelopeHash(envelope);
        const context = (await buildHistoricalRunContext(envelope as LocalProjectEnvelope))!;
        const reopened = context.mechanicsResult!;
        log({ mode, registered_standing: retainedPrecisionStanding(src, model), standing_status: standing.status, standing_eligible: standing.eligible,
          ar_verify: await verifyAnalysisRunRecord(record), receipt_byte_equal: JSON.stringify((record.analysis_run as Json).retained_precision) === original,
          reopen_findings: context.findings, reopen_receipt_byte_equal: JSON.stringify((reopened as Json).retained_precision) === original,
          reopen_registration: retainedPrecisionRegistration(reopened), reopen_standing: retainedPrecisionStanding(reopened, model),
          reopen_eligible: numericalResultStanding(reopened, model).eligible });
        continue;
      }
      for (const [carrier, path, pointer] of [["rust_derivative", `${dir}/rust_derivative_${mode}.json`, "result_envelope"], ["python_analysis_run", `${dir}/py_ar_${mode}.json`, "analysis_run"], ["ts_analysis_run", `${dir}/ts_ar_${mode}.json`, "analysis_run"]] as const) {
        let d: Json;
        try { d = JSON.parse(readFileSync(path, "utf8")); } catch { log({ mode, carrier, missing: true }); continue; }
        const receipt = d[pointer]?.retained_precision;
        const back = structuredClone(src) as Json; back.retained_precision = receipt;
        let error: string | null = null, revalidated = false;
        try { const v = await validateRetainedPrecision(back, inv); const b0 = await validateRetainedPrecision(src, inv); revalidated = JSON.stringify(v) === JSON.stringify(b0) && v.invocation_bound && v.numerical_eligible === b0.numerical_eligible; } catch (e) { error = `${(e as Json).gate}:${(e as Json).code}`; }
        let regStanding: Json = null;
        try { await registerRetainedPrecision(back, inv); regStanding = retainedPrecisionStanding(back, model); } catch (e) { regStanding = { refused: (e as Json).code }; }
        log({ mode, carrier, receipt_byte_equal: JSON.stringify(receipt) === original, revalidated, error, registered_standing: regStanding });
        if (carrier !== "rust_derivative") {
          try { await validateAnalysisRunV03(d, back, modelLoadBasisRefs(model)); log({ mode, carrier: `${carrier}_ts_validate`, result: "ok" }); } catch (e) { log({ mode, carrier: `${carrier}_ts_validate`, result: String((e as Error).message).split(":")[0] }); }
        }
      }
    }
    expect(true).toBe(true);
  }, 600_000);
});

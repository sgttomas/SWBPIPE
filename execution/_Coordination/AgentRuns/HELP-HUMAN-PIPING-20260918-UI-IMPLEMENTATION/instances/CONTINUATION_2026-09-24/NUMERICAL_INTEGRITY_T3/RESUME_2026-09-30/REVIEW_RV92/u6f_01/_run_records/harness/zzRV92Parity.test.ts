/** RV92 (U6f): the TypeScript column of the parity table. Review harness only,
 * copied into the review lane after the suite runs; not part of the candidate.
 * Reads $RV92_PROBES/index.json; writes one JSON line per probe to $RV92_OUT.
 * Registration follows the product route (previewService.validateCapturedSource):
 * only a probe with an invocation registers, through registerRetainedPrecision. */
import { describe, expect, it } from "vitest";
import { appendFileSync, readFileSync, writeFileSync } from "node:fs";
import { sourceContract, numericalResultStanding, sourceSemanticBinding, hasCurrentSourceContract } from "./features/results/numericalResultQuality";
import { validateRetainedPrecision, validateRetainedPrecisionTransport } from "./features/results/retainedPrecision";
import { registerRetainedPrecision, retainedPrecisionStanding, classificationSummary, retainedPrecisionStandingText } from "./features/results/retainedPrecisionStanding";
import { ruleBindingRefusal, isFreshSemanticResult, standingReason } from "./features/results/knownSemanticLimitations";
import { loadReferenceOutputRefusal } from "./features/results/loadReferenceOutputAvailability";
import { buildAnalysisRunV02, buildAnalysisRunV03, validateAnalysisRunV03, verifyAnalysisRunRecord } from "./services/analysisRunCompatibility";
import type { MechanicsResult, PreviewModel } from "./types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const dir = process.env.RV92_PROBES, out = process.env.RV92_OUT;
const code = (e: unknown) => { const x = e as Json; return x?.code ?? String(x?.message ?? x).split(":")[0]; };
async function r(fn: () => unknown, fmt: (v: Json) => string = () => ""): Promise<string> {
  try { return "ok:" + fmt(await fn()); } catch (e) { return "err:" + code(e); }
}

describe.skipIf(!dir || !out)("RV92 parity (TypeScript)", () => {
  it("writes the TS column", async () => {
    writeFileSync(out!, "");
    const index: Json[] = JSON.parse(readFileSync(`${dir}/index.json`, "utf8"));
    for (const entry of index) {
      const p: Json = JSON.parse(readFileSync(`${dir}/${entry.file}`, "utf8"));
      const src = p.source as MechanicsResult;
      const model = { load_cases: (p.load_cases as string[]).map(id => ({ id })) } as unknown as PreviewModel;
      const o: Json = { id: p.id };
      let route = "throw";
      try { route = sourceContract(src); } catch (e) { route = "throw:" + code(e); }
      o.route = route;
      // The header route (TS's transport dispatch) and the unused transport validator.
      o.transport = route === "unsupported" ? "err:" + numericalResultStanding(src, model).findings[0] : route.startsWith("throw") ? "err:" + route : "ok:" + (() => { try { return sourceSemanticBinding(src).id; } catch { return "route:" + route; } })();
      if (route === "retained_preview_physics") o.transport_validator = await r(() => validateRetainedPrecisionTransport(src));
      // Raw dispatch: the header route, then (successor) the accepted reader without an invocation.
      o.raw = route === "unsupported" ? "err:" + numericalResultStanding(src, model).findings[0]
        : route === "retained_preview_physics" ? await r(() => validateRetainedPrecision(src), () => sourceSemanticBinding(src).id)
        : route.startsWith("throw") ? "err:" + route : "ok:" + (() => { try { return sourceSemanticBinding(src).id; } catch { return "route:" + route; } })();
      // Registration as the product does it: only with a captured invocation.
      if (route === "retained_preview_physics" && p.invocation !== null) o.register = await r(() => registerRetainedPrecision(src, p.invocation), (v: Json) => `bound=${v.invocation_bound},eligible=${v.numerical_eligible}`);
      const ns = (() => { try { return numericalResultStanding(src, model); } catch (e) { return { status: "throw:" + code(e), eligible: false, findings: [] as string[] }; } })();
      o.standing_status = ns.status; o.standing_eligible = ns.eligible; o.standing_findings = ns.findings;
      o.standing = route === "unsupported" ? "unsupported" : route === "retained_preview_physics" ? retainedPrecisionStanding(src, model).standing : `generic:${ns.status}`;
      o.fresh = isFreshSemanticResult(src);
      o.standing_reason = (() => { try { return "ok:" + String(standingReason(src)); } catch (e) { return "err:" + code(e); } })();
      const rows: Json[] = Array.isArray(src.results) ? src.results : [];
      const t0 = Date.now();
      o.binding = rows.map(row => { try { return ruleBindingRefusal(src, row); } catch (e) { return "throw:" + code(e); } });
      o.binding_ms = Date.now() - t0;
      o.headline_binding = ["max_displacement", "max_open_formula_stress"].map(k => {
        const ref = (src as Json).summary?.[k]?.result_ref; const row = rows.find(x => x?.id === ref);
        return row ? ruleBindingRefusal(src, row) : "absent";
      });
      o.summary = route === "retained_preview_physics" ? classificationSummary(src, model) : [];
      o.output_refusal = (() => { try { return loadReferenceOutputRefusal(src)?.split(":")[0] ?? null; } catch (e) { return "throw:" + code(e); } })();
      if (route === "retained_preview_physics") o.standing_text = retainedPrecisionStandingText(src).split(";")[0];
      // AnalysisRun 0.3: build, validate, and the receipt mutations.
      const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:rv92" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: src.model_ref }, solver_basis: { solver_name: src.producer?.component_name ?? "x", solver_version: src.producer?.component_version ?? "x", solver_build_ref: "rv92" } } };
      let record: Json = null;
      o.ar_build = await r(async () => { record = await buildAnalysisRunV03(structuredClone(src), manifest as Json); return record; }, (v: Json) => v.schema_version);
      o.ar_mutations = {};
      if (record) {
        o.ar_validate = await r(() => validateAnalysisRunV03(record, src));
        o.ar_verify = await verifyAnalysisRunRecord(record);
        const run = record.analysis_run;
        o.ar_receipt_equal = Object.hasOwn(run, "retained_precision") ? JSON.stringify(run.retained_precision) === JSON.stringify((src as Json).retained_precision) : null;
        if (Object.hasOwn(run, "retained_precision")) {
          const muts: [string, (x: Json) => void][] = [
            ["dropped", x => { delete x.analysis_run.retained_precision; }],
            ["receipt_sha_zero", x => { x.analysis_run.retained_precision.receipt_sha256 = "0".repeat(64); }],
            ["body_charged_plus1", x => { x.analysis_run.retained_precision.body.work.charged += 1; }],
            ["null", x => { x.analysis_run.retained_precision = null; }],
          ];
          for (const [name, fn] of muts) { const m = structuredClone(record); fn(m); o.ar_mutations[name] = await r(() => validateAnalysisRunV03(m, src)); }
          const back = structuredClone(src) as Json; back.retained_precision = structuredClone(run.retained_precision);
          o.ar_reopen = await r(() => validateRetainedPrecision(back, p.invocation ?? undefined), (v: Json) => `bound=${v.invocation_bound},eligible=${v.numerical_eligible}`);
        } else {
          const m = structuredClone(record); m.analysis_run.retained_precision = { receipt_sha256: "0".repeat(64) };
          o.ar_mutations.seeded_other = await r(() => validateAnalysisRunV03(m, src));
        }
      }
      o.ar_v02 = await r(() => buildAnalysisRunV02(structuredClone(src), manifest as Json), (v: Json) => v.schema_version);
      o.current = hasCurrentSourceContract(src);
      appendFileSync(out!, JSON.stringify(o) + "\n");
    }
    expect(true).toBe(true);
  }, 3_600_000);
});
